;;; stable-terminal-isolation-protection.el --- owned helper contracts -*- lexical-binding: t; -*-

(require 'ert)
(require 'cl-lib)
(defvar native-comp-async-jobs-number)
(defvar native-comp-jit-compilation)

(defmacro stp--with-scope (&rest body)
  (declare (indent 0))
  `(let* ((process-environment (copy-sequence process-environment))
          (base (or (getenv "FE_STABLE_TERMINAL_TEST_ROOT")
                    (error "owned protection root is required")))
          (root (make-temp-file (expand-file-name "stp-" base) t))
          (program (expand-file-name "program" root))
          (dump (expand-file-name "dump" root))
          (original (expand-file-name "original.el" root))
          (buffer (generate-new-buffer " stp-owned")))
     (unwind-protect
         (progn
           (dolist (path (list program dump original))
             (with-temp-file path (insert "owned synthetic input\n")))
           (setenv "STABLE_SUITE_DIAGNOSTIC" nil)
           (setenv "STABLE_SUITE_MACHINE_ERROR_DELIVERY" "erc-original-terminal-errors-v1")
           (setenv "STABLE_SUITE_FILE_ID" "test/lisp/erc/erc-scenarios-scrolltobottom.el")
           (setenv "STABLE_SUITE_FILE" original)
           (setenv "STABLE_SUITE_PRODUCT_PROGRAM" program)
           (setenv "STABLE_SUITE_PRODUCT_DUMP" (concat "--dump-file=" dump))
           (setenv "ERC_TESTS_SUBPROCESS" "1")
           ,@body)
       (when (buffer-live-p buffer) (kill-buffer buffer))
       (delete-directory root t))))

(ert-deftest stp-original-helper-arguments-and-budget ()
  (stp--with-scope
    (let ((noninteractive t) seen worker-limit
          (switches (list "-Q" "-nw" "-l" original "-eval" "original-command"))
          (previous-limit (getenv "OMP_THREAD_LIMIT")))
      (should
       (eq buffer
           (apply #'stable-suite--erc-make-term
                  (lambda (&rest args)
                    (setq seen args worker-limit (getenv "OMP_THREAD_LIMIT")) buffer)
                  "original-test" program nil switches)))
      (should (equal seen
                     (append (list "original-test" program nil
                                   (concat "--dump-file=" dump)
                                   "-l" stable-suite--runner-file
                                   "-f" "stable-suite--erc-child-setup") switches)))
      (should (equal worker-limit "1"))
      (should (equal previous-limit (getenv "OMP_THREAD_LIMIT"))))))

(ert-deftest stp-reject-other-image-or-missing-dump ()
  (stp--with-scope
    (let ((noninteractive t) (called 0))
      (dolist (bad-program (list original program))
        (when (equal bad-program program) (setenv "STABLE_SUITE_PRODUCT_DUMP" nil))
        (should-error
         (stable-suite--erc-make-term
          (lambda (&rest _) (cl-incf called) buffer)
          "original-test" bad-program nil "-Q" "-nw" "-l" original)))
      (should (= called 0)))))

(ert-deftest stp-unrelated-term-call-unchanged ()
  (stp--with-scope
    (setenv "ERC_TESTS_SUBPROCESS" nil)
    (let (seen)
      (should (eq buffer (stable-suite--erc-make-term
                          (lambda (&rest args) (setq seen args) buffer)
                          "unrelated" "unrelated-program" nil "untouched")))
      (should (equal seen '("unrelated" "unrelated-program" nil "untouched"))))))

(ert-deftest stp-child-scope-and-jit-preserved ()
  (stp--with-scope
    (let ((noninteractive nil) (window-system nil)
          (native-comp-async-jobs-number 8)
          (native-comp-jit-compilation t) (debugger #'ignore))
      (stable-suite--erc-child-setup)
      (should (= native-comp-async-jobs-number 1))
      (should native-comp-jit-compilation)
      (setenv "ERC_TESTS_SUBPROCESS" nil)
      (should-error (stable-suite--erc-child-setup)))))

(ert-deftest stp-ert-error-and-startup-failure-remain-errors ()
  (stp--with-scope
    (should-error
     (stable-suite--erc-child-debugger #'ignore 'error '(error "original-ert-error"))
     :type 'error)
    (should
     (= 2 (catch 'stp-owned-exit
            (cl-letf (((symbol-function 'ert-running-test) (lambda () nil))
                      ((symbol-function 'kill-emacs)
                       (lambda (code &rest _) (throw 'stp-owned-exit code))))
              (stable-suite--erc-child-debugger #'ignore 'error '(error "startup"))))))))

(ert-deftest stp-child-respects-existing-observation-quota ()
  (stp--with-scope
    (let ((path (expand-file-name "observations.jsonl" root))
          (stable-suite--diagnostic-bytes 0))
      (with-temp-file path (insert (make-string (* 8 1024 1024) ?x)))
      (setenv "STABLE_SUITE_DIAGNOSTIC" path)
      (stable-suite--diagnostic-snapshot "owned-quota-check")
      (should (= (file-attribute-size (file-attributes path)) (* 8 1024 1024))))))

(provide 'stable-terminal-isolation-protection)
