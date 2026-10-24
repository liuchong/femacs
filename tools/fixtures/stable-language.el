;;; stable-language.el --- Bounded stable-language comparisons -*- lexical-binding: t; -*-

(require 'ert)
(require 'cl-lib)
(require 'json)
(defvar femacs-stable-observation-file nil)
(defvar femacs-stable--observations nil)
(defvar femacs-sl-dynamic :unset)
(defvar femacs-sl-order nil)

(defun femacs-stable--record (id value)
  (let ((print-circle t) (print-length nil) (print-level nil))
    (setf (alist-get id femacs-stable--observations nil nil #'equal)
          (prin1-to-string value)))
  (when femacs-stable-observation-file
    (let ((table (make-hash-table :test 'equal)))
      (dolist (row femacs-stable--observations)
        (puthash (car row) (cdr row) table))
      (let ((coding-system-for-write 'utf-8-unix))
        (with-temp-file femacs-stable-observation-file
          (insert (json-serialize
                   `(:schema "femacs-stable-language-v1" :observations ,table))))))))

(ert-deftest sl01-lexical-closure-and-redefinition ()
  (let ((saved (and (fboundp 'femacs-sl-target) (symbol-function 'femacs-sl-target))))
    (unwind-protect
        (let* ((bias 5)
               (closure (lambda (x) (femacs-sl-target (+ bias x))))
               first second)
          (fset 'femacs-sl-target (lambda (x) (+ x 1)))
          (setq first (funcall closure 2))
          (fset 'femacs-sl-target (lambda (x) (* x 2)))
          (setq second (funcall closure 2))
          (should (equal (list first second) '(8 14)))
          (femacs-stable--record "sl01-lexical-closure-and-redefinition" (list first second)))
      (if saved (fset 'femacs-sl-target saved) (fmakunbound 'femacs-sl-target)))))

(ert-deftest sl02-dynamic-binding-and-unwind ()
  (let ((femacs-sl-dynamic :outer) (events nil))
    (let ((result
           (catch 'femacs-sl-exit
             (let ((femacs-sl-dynamic :inner))
               (unwind-protect
                   (progn (push femacs-sl-dynamic events)
                          (throw 'femacs-sl-exit :caught))
                 (push :cleanup events))))))
      (should (equal (list result (reverse events) femacs-sl-dynamic)
                     '(:caught (:inner :cleanup) :outer)))
      (femacs-stable--record "sl02-dynamic-binding-and-unwind"
                             (list result (reverse events) femacs-sl-dynamic)))))

(ert-deftest sl03-multibyte-markers-and-narrowing ()
  (with-temp-buffer
    (insert "甲a🙂\nb")
    (let ((mark (copy-marker 3 t)))
      (unwind-protect
          (progn
            (goto-char 3) (insert "é")
            (let ((size (buffer-size))
                  (bytes (string-bytes (buffer-string))))
              (narrow-to-region 2 5)
              (let ((value (list (marker-position mark) (buffer-string) size bytes)))
                (should (equal value '(4 "aé🙂" 6 12)))
                (femacs-stable--record "sl03-multibyte-markers-and-narrowing" value))))
        (set-marker mark nil)))))

(ert-deftest sl04-bytecode-evaluation-order-and-error ()
  (let* ((fn (lambda ()
               (unwind-protect
                   (list (progn (push :a femacs-sl-order) 1)
                         (progn (push :b femacs-sl-order)
                                (signal 'wrong-type-argument '(integerp "x")))
                         (progn (push :unreached femacs-sl-order) 3))
                 (push :cleanup femacs-sl-order))))
         (compiled (byte-compile fn))
         observations)
    (dolist (callable (list fn compiled))
      (let ((femacs-sl-order nil))
        (let ((error-value (condition-case err (funcall callable) (error err))))
          (should (equal error-value '(wrong-type-argument integerp "x")))
          (should (equal (reverse femacs-sl-order) '(:a :b :cleanup)))
          (push (list error-value (reverse femacs-sl-order)) observations))))
    (should (equal (car observations) (cadr observations)))
    (femacs-stable--record "sl04-bytecode-evaluation-order-and-error" observations)))

(ert-deftest sl05-weak-table-closure-and-finalizer-roots ()
  ;; This checks live roots/no premature finalization. It does not assert when
  ;; an unreachable conservative root disappears or promise a finalizer order.
  (let* ((key (list "live"))
         (table (make-hash-table :test 'eq :weakness 'key))
         (finalized nil)
         (guard (make-finalizer (lambda () (setq finalized t))))
         (closure (let ((captured key)) (lambda () captured))))
    (puthash key '(1 2 3) table)
    (dotimes (_ 3) (garbage-collect))
    (should guard)
    (should-not finalized)
    (should (eq (funcall closure) key))
    (should (equal (gethash key table) '(1 2 3)))
    (femacs-stable--record "sl05-weak-table-closure-and-finalizer-roots"
                           (list (funcall closure) (gethash key table) finalized (type-of guard)))))

(ert-deftest sl06-reader-printer-and-circular-data ()
  (let* ((value (list :utf8 "hé🙂" [1 nil t] '(a . b)))
         (print-circle t)
         (read-circle t)
         (encoded (prin1-to-string value))
         (decoded (read encoded))
         (ring (list :cycle)))
    (should (equal value decoded))
    (setcdr ring ring)
    (let ((restored (read (prin1-to-string ring))))
      (should (eq restored (cdr restored)))
      (should (eq (car restored) :cycle)))
    (femacs-stable--record "sl06-reader-printer-and-circular-data" (list encoded (car ring)))))

;;; stable-language.el ends here
