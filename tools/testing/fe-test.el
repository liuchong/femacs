;;; fe-test.el --- Main CLI and suite orchestrator for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Native Lisp test framework entrypoint run by `dist/lisp/bin/femacs-lisp'.
;; Parses CLI flags, loads registered test suites, invokes memory-aware LPT
;; execution, updates persistent EWMA/failure/report JSON files, and exits cleanly.

;;; Code:

(require 'cl-lib)

(let* ((this-dir (file-name-directory
                  (or load-file-name
                      (and (boundp 'byte-compile-current-file) byte-compile-current-file)
                      (locate-library "fe-test")
                      (expand-file-name "tools/testing/fe-test.el" default-directory)))))
  (add-to-list 'load-path this-dir)
  (add-to-list 'load-path (expand-file-name "suites" this-dir))
  (add-to-list 'load-path (expand-file-name "tests" this-dir)))

(require 'fe-test-dsl)
(require 'fe-test-profile)
(require 'fe-test-cache)
(require 'fe-test-scheduler)
(require 'fe-test-reporter)

(defvar fe-test-library-only nil
  "When non-nil, loading `fe-test' defines functions without executing the CLI.")

(defun fe-test--load-suites (include-self)
  "Clear registry and load suites (including self-tests when INCLUDE-SELF)."
  (fe-test-clear-registry)
  (load (fe-test-path "tools/testing/suites/suite-static.el") nil t)
  (load (fe-test-path "tools/testing/suites/suite-smoke.el") nil t)
  (load (fe-test-path "tools/testing/suites/suite-host.el") nil t)
  (load (fe-test-path "tools/testing/suites/suite-ert-core.el") nil t)
  (when include-self
    (load (fe-test-path "tools/testing/tests/fe-test-self-tests.el") nil t)))

(defun fe-test-filter-by-changed (cases changed-files)
  "Select CASES affected by CHANGED-FILES."
  (if (null changed-files)
      cases
    (let ((need-all (cl-some (lambda (f)
                               (or (string-prefix-p "Cargo." f)
                                   (string-prefix-p "crates/" f)
                                   (string-prefix-p "src/" f)
                                   (string-prefix-p "lib/" f)
                                   (string-prefix-p "xtask/" f)
                                   (string-prefix-p "tools/testing/" f)))
                             changed-files)))
      (if need-all
          cases
        (cl-remove-if-not
         (lambda (tc)
           (or (string= (fe-test-case-suite tc) "static")
               (cl-some (lambda (inp)
                          (cl-some (lambda (chg)
                                     (or (string= inp chg)
                                         (string-prefix-p (file-name-as-directory inp) chg)
                                         (string-prefix-p (file-name-as-directory chg) inp)))
                                   changed-files))
                        (fe-test-case-cache-inputs tc))))
         cases)))))

(defun fe-test-select-shard (cases shard-index shard-total profile-table)
  "Select 1-based LPT-balanced shard SHARD-INDEX out of SHARD-TOTAL."
  (when (or (< shard-total 1) (< shard-index 1) (> shard-index shard-total))
    (error "Invalid shard %d/%d" shard-index shard-total))
  (if (= shard-total 1)
      cases
    (let* ((sorted (fe-test--sort-lpt cases profile-table))
           (buckets (make-vector shard-total nil))
           (loads (make-vector shard-total 0)))
      (dolist (tc sorted)
        (let ((best 0)
              (best-load (aref loads 0)))
          (dotimes (i shard-total)
            (when (< (aref loads i) best-load)
              (setq best i
                    best-load (aref loads i))))
          (push tc (aref buckets best))
          (aset loads best (+ (aref loads best)
                              (fe-test-profile-predict-ms profile-table tc)))))
      (nreverse (aref buckets (1- shard-index))))))

(defun fe-test-select-cases (cases suite-name opts profile-table state-dir)
  "Filter CASES by SUITE-NAME and CLI OPTS."
  (let* ((suite-filtered
          (pcase suite-name
            ("all" cases)
            ("fast" (cl-remove-if-not
                     (lambda (tc) (member (fe-test-case-suite tc) '("static" "smoke" "host")))
                     cases))
            (_ (cl-remove-if-not
                (lambda (tc) (string= (fe-test-case-suite tc) suite-name))
                cases))))
         (pattern (plist-get opts :filter))
         (regex-filtered
          (if pattern
              (cl-remove-if-not
               (lambda (tc)
                 (or (string-match-p pattern (fe-test-case-id tc))
                     (string-match-p pattern (fe-test-case-title tc))))
               suite-filtered)
            suite-filtered))
         (changed-filtered
          (if (plist-get opts :changed)
              (fe-test-filter-by-changed regex-filtered (fe-test-git-changed-files))
            regex-filtered))
         (rerun-filtered
          (if (plist-get opts :rerun-failed)
              (let ((failed-ids (fe-test-failures-load state-dir)))
                (if (null failed-ids)
                    nil
                  (cl-remove-if-not
                   (lambda (tc) (member (fe-test-case-id tc) failed-ids))
                   changed-filtered)))
            changed-filtered))
         (shard (plist-get opts :shard)))
    (if shard
        (fe-test-select-shard rerun-filtered (car shard) (cdr shard) profile-table)
      rerun-filtered)))

(defun fe-test-default-workers ()
  "Return sensible default worker concurrency bounded cleanly."
  (min 4 (max 2 (if (fboundp 'num-processors) (num-processors) 4))))

(defun fe-test-parse-cli (args)
  "Parse ARGS list into an options plist."
  (let ((suite "fast")
        (filter-re nil)
        (rerun-failed nil)
        (keep-going nil)
        (changed nil)
        (shard nil)
        (cache t)
        (workers (fe-test-default-workers))
        (heavy-workers 2)
        (budget-mib 2048)
        (xtask-bin nil)
        (exec-bin nil)
        (verify-host nil)
        (list-only nil)
        (report-out nil)
        (state-dir nil)
        (rest args))
    (while rest
      (let ((cur (pop rest)))
        (pcase cur
          ("--suite" (setq suite (or (pop rest) (error "--suite requires a name"))))
          ("--filter" (setq filter-re (or (pop rest) (error "--filter requires a regexp"))))
          ("--rerun-failed" (setq rerun-failed t))
          ("--keep-going" (setq keep-going t))
          ("--changed" (setq changed t))
          ("--no-cache" (setq cache nil))
          ("--verify-host" (setq verify-host t))
          ("--list" (setq list-only t))
          ("--shard"
           (let ((raw (or (pop rest) (error "--shard requires INDEX/TOTAL"))))
             (unless (string-match "\\`\\([0-9]+\\)/\\([0-9]+\\)\\'" raw)
               (error "Invalid --shard format '%s'; expected INDEX/TOTAL" raw))
             (setq shard (cons (string-to-number (match-string 1 raw))
                               (string-to-number (match-string 2 raw))))))
          ("--workers"
           (setq workers (string-to-number (or (pop rest) (error "--workers requires N")))))
          ("--heavy-workers"
           (setq heavy-workers (string-to-number
                                (or (pop rest) (error "--heavy-workers requires N")))))
          ("--budget-mib"
           (setq budget-mib (string-to-number (or (pop rest) (error "--budget-mib requires N")))))
          ("--xtask-bin" (setq xtask-bin (pop rest)))
          ("--exec-bin" (setq exec-bin (pop rest)))
          ("--report-out" (setq report-out (pop rest)))
          ("--state-dir" (setq state-dir (pop rest)))
          ("--help"
           (princ "Usage: lisp-core --batch -Q -L tools/testing -l fe-test.el -- [OPTIONS]\n")
           (princ "  --suite <fast|static|smoke|host|ert|self|all>\n")
           (princ "  --filter <REGEXP>  --rerun-failed  --keep-going  --changed\n")
           (princ "  --shard <I/N>      --no-cache      --verify-host --list\n")
           (princ "  --workers <N>      --heavy-workers <N>  --budget-mib <N>\n")
           (princ "  --exec-bin <PATH>  --xtask-bin <PATH> --state-dir <DIR> --report-out <PATH>\n")
           (kill-emacs 0))
          (_ (error "Unknown fe-test option: %s" cur)))))
    (list :suite suite
          :filter filter-re
          :rerun-failed rerun-failed
          :keep-going keep-going
          :changed changed
          :shard shard
          :cache cache
          :workers workers
          :heavy-workers heavy-workers
          :budget-mib budget-mib
          :xtask-bin xtask-bin
          :exec-bin exec-bin
          :verify-host verify-host
          :list-only list-only
          :report-out report-out
          :state-dir state-dir)))

(defun fe-test-execute-cases (cases opts)
  "Execute explicit list of CASES with OPTS and return `(exit-code . report)'."
  (let* ((state-dir (fe-test-state-dir (plist-get opts :state-dir)))
         (report-path (or (plist-get opts :report-out)
                          (expand-file-name "latest-report.json" state-dir)))
         (xtask-bin (fe-test-find-exec-runner (or (plist-get opts :exec-bin)
                                                  (plist-get opts :xtask-bin))))
         (suite-name (or (plist-get opts :suite) "custom"))
         (profile-table (fe-test-profile-load state-dir)))
    (when (plist-get opts :verify-host)
      (fe-test-verify-stable-host xtask-bin))
    (let* ((start-ms (fe-test-now-ms))
           (run-out (fe-test-run-schedule
                     cases
                     (list :xtask-bin xtask-bin
                           :state-dir state-dir
                           :cache (plist-get opts :cache)
                           :keep-going (plist-get opts :keep-going)
                           :workers (plist-get opts :workers)
                           :heavy-workers (plist-get opts :heavy-workers)
                           :budget-mib (plist-get opts :budget-mib))
                     profile-table))
           (wall-ms (max 1 (- (fe-test-now-ms) start-ms)))
           (results (plist-get run-out :results)))
      (fe-test-profile-update state-dir profile-table results)
      (let* ((report (fe-test-write-report report-path suite-name wall-ms run-out results))
             (failed-list (let ((ft (alist-get 'failedTests report)))
                            (if (vectorp ft) (append ft nil) ft)))
             (failed-count (alist-get 'failed report))
             (unstarted-count (alist-get 'unstarted report))
             (cancelled-count (alist-get 'cancelled report))
             (ok (and (= failed-count 0)
                      (= unstarted-count 0)
                      (= cancelled-count 0))))
        (fe-test-failures-save state-dir suite-name failed-list)
        (fe-test-print-summary report results report-path)
        (cons (if ok 0 1) report)))))

(defun fe-test-main (args)
  "Run CLI entrypoint for ARGS and return integer process exit code."
  (let* ((opts (fe-test-parse-cli args))
         (suite-name (plist-get opts :suite))
         (state-dir (fe-test-state-dir (plist-get opts :state-dir))))
    (if (string= suite-name "self")
        (let ((default-directory (file-name-as-directory (fe-test-repo-root)))
              (self-file (fe-test-path "tools/testing/tests/fe-test-self-tests.el")))
          (require 'ert)
          (load self-file nil t)
          (ert-run-tests-batch-and-exit (plist-get opts :filter)))
      (let ((profile-table (fe-test-profile-load state-dir)))
        (fe-test--load-suites nil)
        (let ((selected (fe-test-select-cases
                         (fe-test-registered-cases)
                         suite-name opts profile-table state-dir)))
          (cond
           ((plist-get opts :list-only)
            (dolist (tc selected)
              (princ (format "%s\t%s\t%s\n"
                             (fe-test-case-id tc)
                             (fe-test-case-suite tc)
                             (fe-test-case-title tc))))
            0)
           ((and (plist-get opts :rerun-failed) (null selected))
            (princ "fe-test: no recorded failures to rerun\n")
            0)
           ((null selected)
            (error "fe-test: no test cases matched selection (suite=%s)" suite-name))
           (t
            (car (fe-test-execute-cases selected opts)))))))))

(when (and noninteractive (not fe-test-library-only))
  (let ((cli-args (copy-sequence command-line-args-left)))
    (setq command-line-args-left nil)
    (when (equal (car cli-args) "--")
      (pop cli-args))
    (kill-emacs (fe-test-main cli-args))))

(provide 'fe-test)
;;; fe-test.el ends here
