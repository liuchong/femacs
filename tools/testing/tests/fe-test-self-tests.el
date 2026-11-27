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

(provide 'fe-test-self-tests)
;;; fe-test-self-tests.el ends here
