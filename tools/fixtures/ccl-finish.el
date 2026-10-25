;;; ccl-finish.el --- CCL exit and observation serialization cases -*- lexical-binding: t; -*-

(require 'ert)
(require 'ccl)
(require 'cl-lib)
(require 'json)

(defconst femacs-g4-fixture-root
  (expand-file-name "../.." (file-name-directory (or load-file-name buffer-file-name))))

(define-ccl-program femacs-g4-copy
  '(1 ((loop (read r0) (write r0) (repeat)))))
(define-ccl-program femacs-g4-double
  '(2 ((loop (read r0) (write r0) (write r0) (repeat)))))

(ert-deftest femacs-ccl-finish-ascii ()
  (should (equal (ccl-execute-on-string 'femacs-g4-copy (make-vector 9 0) "abc") "abc")))

(ert-deftest femacs-ccl-finish-multibyte ()
  (should (equal (ccl-execute-on-string 'femacs-g4-copy (make-vector 9 0) "中文λ🙂") "中文λ🙂")))

(ert-deftest femacs-ccl-finish-empty ()
  (should (equal (ccl-execute-on-string 'femacs-g4-copy (make-vector 9 0) "") "")))

(ert-deftest femacs-ccl-finish-source-suspend-resume ()
  (let ((status (make-vector 9 0)))
    (should (equal (ccl-execute-on-string 'femacs-g4-copy status "ab" t) "ab"))
    (should (equal (ccl-execute-on-string 'femacs-g4-copy status "cd") "cd"))))

(ert-deftest femacs-ccl-finish-destination-suspend ()
  (should (equal (ccl-execute-on-string 'femacs-g4-double (make-vector 9 0)
                                       (make-string 1500 ?x))
                 (make-string 3000 ?x))))

(ert-deftest femacs-ccl-invalid-command-still-errors ()
  (should-error (ccl-execute-on-string [1 2 -1] (make-vector 9 0) "x")))

;; Explicitly isolated serializer component. Android/JNI is mocked here; real
;; device clipboard, editing, rendering and rotation remain separate acceptance.
(defvar femacs-android-report-file)
(defvar femacs-android-run-token)
(defvar femacs-android-pattern-colors)
(defvar femacs-android-observation-timer)
(defvar femacs-android-acceptance-buffer)
(defvar femacs-android-clipboard-token)
(defvar femacs-android-battery-observation)

(ert-deftest femacs-android-fixture-serializes-real-json-null ()
  (let* ((parent (expand-file-name "build/acceptance" femacs-g4-fixture-root))
         (_ (make-directory parent t))
         (directory (make-temp-file (expand-file-name "ccl-json-" parent) t))
         (femacs-android-report-file (expand-file-name "observation.json" directory))
         (femacs-android-run-token "serializer-component")
         (femacs-android-pattern-colors nil)
         (femacs-android-observation-timer nil)
         (femacs-android-acceptance-buffer nil)
         (femacs-android-clipboard-token nil)
         (femacs-android-battery-observation nil)
         (window-system 'android)
         (clipboard nil))
    (unwind-protect
        (cl-letf (((symbol-function 'android-set-clipboard)
                   (lambda (value) (setq clipboard value)))
                  ((symbol-function 'android-get-clipboard) (lambda () clipboard))
                  ((symbol-function 'android-query-battery)
                   (lambda () '(100 10000 900000 900000 2 -1 1 250))))
          (load (expand-file-name "tools/fixtures/android-acceptance.el" femacs-g4-fixture-root)
                nil t t)
          (femacs-android-write-observation)
          (let* ((json-object-type 'alist)
                 (json-null :observed-null)
                 (value (json-read-file femacs-android-report-file)))
            (should (eq (alist-get 'error value) :observed-null))
            (should (equal (alist-get 'run_token value) femacs-android-run-token))
            (should (equal (alist-get 'clipboard_actual value) clipboard))))
      (when (timerp femacs-android-observation-timer)
        (cancel-timer femacs-android-observation-timer))
      (when (buffer-live-p femacs-android-acceptance-buffer)
        (kill-buffer femacs-android-acceptance-buffer))
      ;; Only the fresh per-case owned directory is retired here.
      (delete-directory directory t))))

;;; ccl-finish.el ends here
