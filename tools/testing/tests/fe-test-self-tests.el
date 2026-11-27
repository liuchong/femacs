;;; fe-test-self-tests.el --- ERT unit & integration self-tests for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Verifies LPT ordering, EWMA profile persistence, disjoint sharding,
;; content-addressed cache hits and invalidation, `--rerun-failed`, fail-fast
;; cancellation, memory-admission pausing, and zero-warning strict byte-compile.

;;; Code:

(require 'ert)
(require 'cl-lib)
(require 'bytecomp)

(eval-and-compile
  (defvar fe-test-library-only)
  (let ((fe-test-library-only t)
        (this-dir (file-name-directory
                   (or load-file-name
                       (and (boundp 'byte-compile-current-file) byte-compile-current-file)
                       (expand-file-name "tools/testing/tests/fe-test-self-tests.el"
                                         default-directory)))))
    (let ((testing-dir (directory-file-name
                        (file-name-directory (directory-file-name this-dir)))))
      (add-to-list 'load-path testing-dir)
      (require 'fe-test))))

(defmacro fe-test-self--with-tmp-dir (var &rest body)
  "Bind VAR to a fresh temp directory under `build/test-framework/tmp' for BODY."
  (declare (indent 1))
  `(let* ((root (fe-test-path "build/test-framework/tmp"))
          (_ (make-directory root t))
          (,var (let ((temporary-file-directory root))
                  (make-temp-file "fe-self-" t))))
     (unwind-protect
         (progn ,@body)
       (ignore-errors (delete-directory ,var t)))))

(ert-deftest fe-test-self-ewma-and-profile-roundtrip ()
  (should (= (fe-test-ewma nil 250) 250))
  (should (= (fe-test-ewma 100 200) 170))
  (fe-test-self--with-tmp-dir tmp
    (let ((tbl (fe-test-profile-load tmp)))
      (should (= (hash-table-count tbl) 0))
      (fe-test-profile-update
       tmp tbl
       (list (list (cons 'id "self/a") (cons 'status "passed")
                   (cons 'elapsedMs 100) (cons 'peakRssBytes 1024)
                   (cons 'peakFootprintBytes 2048))
             (list (cons 'id "self/cached") (cons 'status "passed") (cons 'cached t)
                   (cons 'elapsedMs 9999))))
      (let ((reloaded (fe-test-profile-load tmp)))
        (should (= (alist-get 'elapsedMs (gethash "self/a" reloaded)) 100))
        (should (null (gethash "self/cached" reloaded)))))))

(ert-deftest fe-test-self-lpt-order-and-sharding ()
  (let ((tbl (make-hash-table :test #'equal))
        (cases (list (fe-test-make-case :id "c/short" :suite "s" :command '("/bin/echo" "1"))
                     (fe-test-make-case :id "c/long" :suite "s" :command '("/bin/echo" "2"))
                     (fe-test-make-case :id "c/mid" :suite "s" :command '("/bin/echo" "3"))
                     (fe-test-make-case :id "c/tiny" :suite "s" :command '("/bin/echo" "4")))))
    (puthash "c/short" '((elapsedMs . 50)) tbl)
    (puthash "c/long" '((elapsedMs . 900)) tbl)
    (puthash "c/mid" '((elapsedMs . 300)) tbl)
    (puthash "c/tiny" '((elapsedMs . 10)) tbl)
    (let ((sorted (mapcar #'fe-test-case-id (fe-test--sort-lpt cases tbl))))
      (should (equal sorted '("c/long" "c/mid" "c/short" "c/tiny"))))
    (let* ((s1 (mapcar #'fe-test-case-id (fe-test-select-shard cases 1 2 tbl)))
           (s2 (mapcar #'fe-test-case-id (fe-test-select-shard cases 2 2 tbl)))
           (all (sort (append (copy-sequence s1) (copy-sequence s2)) #'string<)))
      (should (equal all '("c/long" "c/mid" "c/short" "c/tiny")))
      (should (null (cl-intersection s1 s2 :test #'string=))))))

(ert-deftest fe-test-self-cache-hit-and-invalidation ()
  (fe-test-self--with-tmp-dir tmp
    (let* ((state-dir (expand-file-name "state" tmp))
           (tracked (expand-file-name "tracked-input.txt" tmp))
           (counter-file (expand-file-name "runs.txt" tmp))
           (lisp-bin (plist-get (fe-test-targets) :lisp-host)))
      (with-temp-file tracked (insert "v1\n"))
      (let* ((tc (fe-test-make-case
                  :id "self/cache-case"
                  :suite "self"
                  :title "cache sensitivity"
                  :command (list lisp-bin "--batch" "-Q" "--eval"
                                 (format "(with-temp-file %S (insert \"ran\"))" counter-file))
                  :cacheable t
                  :cache-inputs (list tracked)))
             (opts (list :suite "self" :cache t :workers 1 :heavy-workers 1
                         :budget-mib 1024 :state-dir state-dir
                         :report-out (expand-file-name "r1.json" state-dir)))
             (run1 (cdr (fe-test-execute-cases (list tc) opts))))
        (should (= (alist-get 'passed run1) 1))
        (should (= (alist-get 'cached run1) 0))
        (should (file-regular-p counter-file))
        (delete-file counter-file)
        (let ((run2 (cdr (fe-test-execute-cases (list tc) opts))))
          (should (= (alist-get 'passed run2) 1))
          (should (= (alist-get 'cached run2) 1))
          (should (not (file-exists-p counter-file))))
        (with-temp-file tracked (insert "v2-mutated\n"))
        (let ((run3 (cdr (fe-test-execute-cases (list tc) opts))))
          (should (= (alist-get 'passed run3) 1))
          (should (= (alist-get 'cached run3) 0))
          (should (file-regular-p counter-file)))))))

(ert-deftest fe-test-self-rerun-failed-selects-only-failures ()
  (fe-test-self--with-tmp-dir tmp
    (let* ((state-dir (expand-file-name "state" tmp))
           (flag-file (expand-file-name "should-pass" tmp))
           (c-pass (fe-test-make-case
                    :id "self/always-ok" :suite "self" :cacheable nil
                    :command '("/bin/sh" "-c" "exit 0")))
           (c-flip (fe-test-make-case
                    :id "self/flaky" :suite "self" :cacheable nil
                    :command (list "/bin/sh" "-c" (format "test -f %s" flag-file))))
           (cases (list c-pass c-flip))
           (opts (list :suite "self" :cache nil :keep-going t :workers 2
                       :heavy-workers 1 :budget-mib 1024 :state-dir state-dir
                       :report-out (expand-file-name "r-fail.json" state-dir)))
           (res1 (fe-test-execute-cases cases opts)))
      (should (= (car res1) 1))
      (should (= (alist-get 'failed (cdr res1)) 1))
      (should (= (alist-get 'passed (cdr res1)) 1))
      (with-temp-file flag-file (insert "ok\n"))
      (let* ((rerun-opts (plist-put (copy-sequence opts) :rerun-failed t))
             (selected (fe-test-select-cases cases "self" rerun-opts
                                             (fe-test-profile-load state-dir) state-dir)))
        (should (equal (mapcar #'fe-test-case-id selected) '("self/flaky")))
        (let ((res2 (fe-test-execute-cases selected opts)))
          (should (= (car res2) 0))
          (should (= (alist-get 'passed (cdr res2)) 1)))))))

(ert-deftest fe-test-self-fail-fast-and-admission-pauses ()
  (fe-test-self--with-tmp-dir tmp
    ;; 1. Fail-fast aborts lingering sleeps cleanly.
    (let* ((s1 (expand-file-name "ff" tmp))
           (c-fail (fe-test-make-case :id "self/ff-1" :suite "self" :cacheable nil
                                      :command '("/bin/sh" "-c" "exit 1")))
           (c-slow1 (fe-test-make-case :id "self/ff-2" :suite "self" :cacheable nil
                                       :command '("/bin/sh" "-c" "sleep 5")))
           (c-slow2 (fe-test-make-case :id "self/ff-3" :suite "self" :cacheable nil
                                       :command '("/bin/sh" "-c" "sleep 5")))
           (res (fe-test-execute-cases
                 (list c-fail c-slow1 c-slow2)
                 (list :suite "self" :cache nil :keep-going nil :workers 3
                       :heavy-workers 3 :budget-mib 1024 :state-dir s1
                       :report-out (expand-file-name "r-ff.json" s1)))))
      (should (= (car res) 1))
      (should (= (+ (alist-get 'failed (cdr res))
                    (alist-get 'cancelled (cdr res))
                    (alist-get 'unstarted (cdr res)))
                 3)))
    ;; 2. Memory admission pauses cleanly under tight budget without deadlock.
    (let* ((s2 (expand-file-name "mem" tmp))
           (cases (list (fe-test-make-case :id "self/m1" :suite "self" :heavy t
                                           :cacheable nil :command '("/bin/sh" "-c" "sleep 0.25"))
                        (fe-test-make-case :id "self/m2" :suite "self" :heavy t
                                           :cacheable nil :command '("/bin/sh" "-c" "sleep 0.25"))))
           (res (fe-test-execute-cases
                 cases
                 (list :suite "self" :cache nil :keep-going t :workers 2
                       :heavy-workers 2 :budget-mib 256 :state-dir s2
                       :report-out (expand-file-name "r-mem.json" s2)))))
      (should (= (car res) 0))
      (should (= (alist-get 'passed (cdr res)) 2))
      (should (>= (alist-get 'admissionPauses (cdr res)) 1)))))

(ert-deftest fe-test-self-strict-byte-compile ()
  (fe-test-self--with-tmp-dir out
    (let* ((byte-compile-error-on-warn t)
           (files (append
                   (directory-files (fe-test-path "tools/testing") t "\\.el\\'")
                   (directory-files (fe-test-path "tools/testing/suites") t "\\.el\\'")
                   (directory-files (fe-test-path "tools/testing/tests") t "\\.el\\'"))))
      (should (>= (length files) 11))
      (dolist (f files)
        (let* ((dest (expand-file-name (concat (file-name-nondirectory f) "c") out))
               (byte-compile-dest-file-function (lambda (_) dest)))
          (should (eq (byte-compile-file f) t)))))))

(defun fe-test-self--write-exec-script (path body)
  "Write executable shell script BODY to PATH."
  (let ((coding-system-for-write 'utf-8-unix))
    (with-temp-file path
      (insert "#!/bin/sh\n" body "\n")))
  (set-file-modes path #o755)
  path)

(ert-deftest fe-test-self-rejects-unreaped-and-supervisor-errors ()
  (let ((dummy (fe-test-make-case :id "self/eval-unit" :suite "self" :command '("/bin/true"))))
    (let ((unreaped (fe-test--evaluate-supervise-output
                     dummy
                     "{\"schema\":\"femacs-test-exec-v1\",\"exit_code\":0,\"timed_out\":false,\"memory_exceeded\":false,\"elapsed_ms\":12,\"peak_rss_bytes\":100,\"peak_footprint_bytes\":100,\"output_drained\":true,\"children_reaped\":false,\"stdout\":\"\",\"stderr\":\"\"}"
                     12 'exit 0)))
      (should (equal (alist-get 'status unreaped) "failed"))
      (should (null (alist-get 'childrenReaped unreaped))))
    (let ((bad-sup (fe-test--evaluate-supervise-output
                    dummy
                    "{\"schema\":\"femacs-test-exec-v1\",\"exit_code\":0,\"timed_out\":false,\"memory_exceeded\":false,\"elapsed_ms\":12,\"peak_rss_bytes\":100,\"peak_footprint_bytes\":100,\"output_drained\":true,\"children_reaped\":true,\"stdout\":\"\",\"stderr\":\"\"}"
                    12 'exit 2)))
      (should (equal (alist-get 'status bad-sup) "failed"))
      (should (null (alist-get 'supervisorOk bad-sup)))))
  (fe-test-self--with-tmp-dir tmp
    (let* ((real-xtask (fe-test-find-xtask))
           (mock-xtask (expand-file-name "mock-xtask.sh" tmp))
           (state-dir (expand-file-name "state" tmp))
           (inp (expand-file-name "inp.txt" tmp)))
      (with-temp-file inp (insert "stable\n"))
      (fe-test-self--write-exec-script
       mock-xtask
       (format
        "sub=\"$2\"
if [ \"$sub\" = \"supervise\" ]; then
  case \"$*\" in
    *MOCK_UNREAPED*)
      printf '{\"schema\":\"femacs-test-exec-v1\",\"exit_code\":0,\"signal\":null,\"timed_out\":false,\"memory_exceeded\":false,\"elapsed_ms\":5,\"peak_rss_bytes\":1024,\"peak_footprint_bytes\":1024,\"peak_accounted_bytes\":1024,\"max_tree_pids\":1,\"output_drained\":false,\"children_reaped\":false,\"stdout\":\"\",\"stderr\":\"\",\"stdout_truncated\":false,\"stderr_truncated\":false}\\n'
      exit 0 ;;
    *MOCK_SUPERVISOR_FAIL*)
      printf '{\"schema\":\"femacs-test-exec-v1\",\"exit_code\":0,\"signal\":null,\"timed_out\":false,\"memory_exceeded\":false,\"elapsed_ms\":5,\"peak_rss_bytes\":1024,\"peak_footprint_bytes\":1024,\"peak_accounted_bytes\":1024,\"max_tree_pids\":1,\"output_drained\":true,\"children_reaped\":true,\"stdout\":\"\",\"stderr\":\"\",\"stdout_truncated\":false,\"stderr_truncated\":false}\\n'
      exit 2 ;;
  esac
fi
exec %S \"$@\""
        real-xtask))
      (let* ((c1 (fe-test-make-case :id "self/unreaped" :suite "self" :cacheable t
                                    :cache-inputs (list inp)
                                    :command (list "/bin/echo" "MOCK_UNREAPED")))
             (c2 (fe-test-make-case :id "self/sup-fail" :suite "self" :cacheable t
                                    :cache-inputs (list inp)
                                    :command (list "/bin/echo" "MOCK_SUPERVISOR_FAIL")))
             (opts (list :suite "self" :cache t :keep-going t :workers 2
                         :heavy-workers 2 :budget-mib 1024 :state-dir state-dir
                         :xtask-bin mock-xtask
                         :report-out (expand-file-name "rep.json" state-dir)))
             (run1 (fe-test-execute-cases (list c1 c2) opts))
             (cache-dir (fe-test-cache-dir state-dir))
             (cached-files (when (file-directory-p cache-dir)
                             (directory-files cache-dir nil "\\.json\\'"))))
        (should (= (car run1) 1))
        (should (= (alist-get 'passed (cdr run1)) 0))
        (should (= (alist-get 'failed (cdr run1)) 2))
        (should (null cached-files))
        (let ((run2 (fe-test-execute-cases (list c1 c2) opts)))
          (should (= (alist-get 'cached (cdr run2)) 0))
          (should (= (alist-get 'failed (cdr run2)) 2)))))))

(ert-deftest fe-test-self-admission-gap-and-full-concurrency-sampling ()
  ;; Pure projection checks: unsampled reservations + sampled retention.
  (let* ((mb (* 1024 1024))
         (w-unsampled (fe-test--make-worker :started-ms 1000 :est-mem (* 200 mb) :sampled nil :observed-mem 0))
         (w-grace (fe-test--make-worker :started-ms 1000 :est-mem (* 200 mb) :sampled t :observed-mem (* 80 mb)))
         (w-mature (fe-test--make-worker :started-ms 1000 :est-mem (* 200 mb) :sampled t :observed-mem (* 60 mb))))
    (should (= (fe-test--compute-projected-bytes (list w-unsampled) 1100) (* 200 mb)))
    (should (= (fe-test--compute-projected-bytes (list w-unsampled w-grace) 1100) (* 400 mb)))
    (should (= (fe-test--compute-projected-bytes (list w-mature) 6000) (* 60 mb))))
  (fe-test-self--with-tmp-dir tmp
    (let* ((real-xtask (fe-test-find-xtask))
           (mock-xtask (expand-file-name "mem-wrapper.sh" tmp))
           (mem-log (expand-file-name "mem-tree.log" tmp))
           (stamp-a (expand-file-name "stamp-a.txt" tmp))
           (stamp-b (expand-file-name "stamp-b.txt" tmp)))
      (fe-test-self--write-exec-script
       mock-xtask
       (format
        "if [ \"$2\" = \"mem-tree\" ]; then
  printf 'call\\n' >> %S
  shift 2
  entries=\"\"
  sep=\"\"
  for p in \"$@\"; do
    entries=\"${entries}${sep}\\\"${p}\\\":{\\\"rss_bytes\\\":5242880,\\\"footprint_bytes\\\":5242880,\\\"accounted_bytes\\\":5242880,\\\"tree_pids\\\":[${p}]}\"
    sep=\",\"
  done
  printf '{\"schema\":\"femacs-test-mem-v1\",\"total_rss_bytes\":5242880,\"total_footprint_bytes\":5242880,\"total_accounted_bytes\":5242880,\"per_pid\":{%%s}}\\n' \"$entries\"
  exit 0
fi
exec %S \"$@\""
        mem-log real-xtask))
      ;; 1. Sampling gap protection: 500ms interval cannot allow 2nd heavy worker inside gap.
      (let* ((fe-test-mem-sample-interval-ms 500)
             (s1 (expand-file-name "s-gap" tmp))
             (cmd-body (lambda (out)
                         (format "perl -MTime::HiRes=time -e '$|=1; printf \"%%d\\n\", int(time()*1000); select(undef,undef,undef,0.35); printf \"%%d\\n\", int(time()*1000);' > %s" out)))
             (ca (fe-test-make-case :id "self/gap-a" :suite "self" :heavy t :cacheable nil
                                    :command (list "/bin/sh" "-c" (funcall cmd-body stamp-a))))
             (cb (fe-test-make-case :id "self/gap-b" :suite "self" :heavy t :cacheable nil
                                    :command (list "/bin/sh" "-c" (funcall cmd-body stamp-b))))
             (res (fe-test-execute-cases
                   (list ca cb)
                   (list :suite "self" :cache nil :keep-going t :workers 2
                         :heavy-workers 2 :budget-mib 256 :state-dir s1
                         :xtask-bin mock-xtask
                         :report-out (expand-file-name "r-gap.json" s1)))))
        (should (= (car res) 0))
        (should (>= (alist-get 'admissionPauses (cdr res)) 1))
        (cl-labels ((read-pair (path)
                      (with-temp-buffer
                        (insert-file-contents path)
                        (mapcar #'string-to-number (split-string (buffer-string) "\n" t)))))
          (let* ((pa (read-pair stamp-a))
                 (pb (read-pair stamp-b))
                 (a0 (nth 0 pa)) (a1 (nth 1 pa))
                 (b0 (nth 0 pb)) (b1 (nth 1 pb)))
            (should (or (>= b0 a1) (>= a0 b1))))))
      ;; 2. Full concurrency + empty pending still polls mem-tree periodically.
      (when (file-exists-p mem-log) (delete-file mem-log))
      (let* ((fe-test-mem-sample-interval-ms 120)
             (s2 (expand-file-name "s-full" tmp))
             (c-solo (fe-test-make-case :id "self/full-conc" :suite "self" :cacheable nil
                                        :command '("/bin/sh" "-c" "sleep 0.55")))
             (res (fe-test-execute-cases
                   (list c-solo)
                   (list :suite "self" :cache nil :keep-going t :workers 1
                         :heavy-workers 1 :budget-mib 512 :state-dir s2
                         :xtask-bin mock-xtask
                         :report-out (expand-file-name "r-full.json" s2))))
             (calls (with-temp-buffer
                      (insert-file-contents mem-log)
                      (length (split-string (buffer-string) "\n" t)))))
        (should (= (car res) 0))
        (should (>= calls 3))
        (should (>= (alist-get 'peakAggregateRssBytes (cdr res)) 5242880))))))

(ert-deftest fe-test-self-runtime-closure-invalidation ()
  (should (null (fe-test-product-runtime-inputs "/usr/bin/true")))
  (fe-test-self--with-tmp-dir tmp
    (let* ((app-root (expand-file-name "Mock.app/Contents" tmp))
           (bin-path (expand-file-name "MacOS/Femacs" app-root))
           (pdmp-path (expand-file-name "Resources/libexec/Femacs.pdmp" app-root))
           (lisp-file (expand-file-name "Resources/lisp/core.el" app-root))
           (etc-file (expand-file-name "Resources/etc/DOC" app-root))
           (state-dir (expand-file-name "state" tmp)))
      (make-directory (file-name-directory bin-path) t)
      (make-directory (file-name-directory pdmp-path) t)
      (make-directory (file-name-directory lisp-file) t)
      (make-directory (file-name-directory etc-file) t)
      (fe-test-self--write-exec-script bin-path "printf 'mock-app-ok\\n'")
      (with-temp-file pdmp-path (insert "pdmp-v1\n"))
      (with-temp-file lisp-file (insert "(provide 'core)\n"))
      (with-temp-file etc-file (insert "doc-v1\n"))
      (let* ((inputs (fe-test-product-runtime-inputs bin-path))
             (tc (fe-test-make-case :id "self/runtime-closure" :suite "self"
                                    :command (list bin-path)
                                    :cacheable (not (null inputs))
                                    :cache-inputs inputs))
             (opts (list :suite "self" :cache t :workers 1 :heavy-workers 1
                         :budget-mib 1024 :state-dir state-dir
                         :report-out (expand-file-name "r-clos.json" state-dir))))
        (should (member pdmp-path inputs))
        (should (member (expand-file-name "Resources/lisp" app-root) inputs))
        (should (member (expand-file-name "Resources/etc" app-root) inputs))
        ;; Run 1 (miss) -> Run 2 (hit)
        (should (= (alist-get 'cached (cdr (fe-test-execute-cases (list tc) opts))) 0))
        (should (= (alist-get 'cached (cdr (fe-test-execute-cases (list tc) opts))) 1))
        ;; Mutate only .pdmp -> cache must miss
        (with-temp-file pdmp-path (insert "pdmp-v2\n"))
        (should (= (alist-get 'cached (cdr (fe-test-execute-cases (list tc) opts))) 0))
        ;; Mutate only Resources/lisp -> cache must miss
        (with-temp-file lisp-file (insert "(provide 'core) ;; v2\n"))
        (should (= (alist-get 'cached (cdr (fe-test-execute-cases (list tc) opts))) 0))
        ;; Mutate only Resources/etc -> cache must miss
        (with-temp-file etc-file (insert "doc-v2\n"))
        (should (= (alist-get 'cached (cdr (fe-test-execute-cases (list tc) opts))) 0))))))

(provide 'fe-test-self-tests)
;;; fe-test-self-tests.el ends here
