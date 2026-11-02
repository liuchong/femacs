;;; runtime-language.el --- Shared runtime-language B cases -*- lexical-binding: t; -*-

;; Additional public-language cases for the runtime B matrix.  These cases
;; deliberately use only APIs available in the stable 31.1 baseline and the
;; candidate.  Private observer/collector/native APIs do not belong here.
;;
;; Loading this file only defines variables, helpers and ERT tests.  It does
;; not byte-compile, load or compile any source.  Tests that write optional
;; observations restrict writes to FEMACS_RUNTIME_TEST_ROOT.

(require 'ert)
(require 'cl-lib)
(require 'json)

(defvar femacs-runtime-language-observation-file nil)
(defvar femacs-runtime-language--observations nil)
(defvar femacs-runtime-language--finalizer-runs 0)
(defvar femacs-runtime-language--finalizer-errors 0)
(defvar femacs-runtime-language--finalizer-recursive 0)
(defvar femacs-runtime-language--finalizer-captured 0)
(defvar femacs-runtime-language--dynamic :unset)
(defvar femacs-runtime-language--order nil)

(defun femacs-runtime-language--root ()
  "Return the caller-owned runtime test root."
  (let ((root (getenv "FEMACS_RUNTIME_TEST_ROOT")))
    (unless (and root (> (length root) 0))
      (error "FEMACS_RUNTIME_TEST_ROOT is not set"))
    (file-name-as-directory (expand-file-name root))))

(defun femacs-runtime-language--record (id value)
  "Record VALUE for ID and optionally write the deterministic observation file."
  (setf (alist-get id femacs-runtime-language--observations nil nil #'equal)
        (prin1-to-string value))
  (when femacs-runtime-language-observation-file
    (let* ((root (femacs-runtime-language--root))
           (file (expand-file-name femacs-runtime-language-observation-file root)))
      (unless (string-prefix-p root file)
        (error "Observation file is outside FEMACS_RUNTIME_TEST_ROOT: %s" file))
      (make-directory (file-name-directory file) t)
      (let ((table (make-hash-table :test 'equal)))
        (dolist (row femacs-runtime-language--observations)
          (puthash (car row) (cdr row) table))
        (let ((coding-system-for-write 'utf-8-unix))
          (with-temp-file file
            (insert (json-serialize
                     (list :schema "femacs-runtime-language-v1"
                           :observations table)))))))))

(defun femacs-runtime-language--arity (a &optional b &rest rest)
  (list a b rest))

(ert-deftest rtl01-lexical-closure-and-redefinition ()
  (let ((saved (and (fboundp 'femacs-runtime-language-target)
                    (symbol-function 'femacs-runtime-language-target)))
        first second)
    (unwind-protect
        (let* ((bias 5)
               (closure (lambda (x)
                          (femacs-runtime-language-target (+ bias x)))))
          (fset 'femacs-runtime-language-target (lambda (x) (+ x 1)))
          (setq first (funcall closure 2))
          (fset 'femacs-runtime-language-target (lambda (x) (* x 2)))
          (setq second (funcall closure 2))
          (should (equal (list first second) '(8 14)))
          (femacs-runtime-language--record
           "rtl01-lexical-closure-and-redefinition" (list first second)))
      (if saved
          (fset 'femacs-runtime-language-target saved)
        (fmakunbound 'femacs-runtime-language-target)))))

(ert-deftest rtl02-dynamic-binding-and-unwind ()
  (let ((femacs-runtime-language--dynamic :outer)
        (events nil)
        result)
    (setq result
          (catch 'femacs-runtime-language-exit
            (let ((femacs-runtime-language--dynamic :inner))
              (unwind-protect
                  (progn
                    (push femacs-runtime-language--dynamic events)
                    (throw 'femacs-runtime-language-exit :caught))
                (push :cleanup events)))))
    (let ((final-events (nreverse events)))
      (should (equal (list result final-events femacs-runtime-language--dynamic)
                     '(:caught (:inner :cleanup) :outer)))
      (femacs-runtime-language--record
       "rtl02-dynamic-binding-and-unwind"
       (list result final-events femacs-runtime-language--dynamic)))))

(ert-deftest rtl03-optional-rest-arity ()
  (should (equal (femacs-runtime-language--arity 1) '(1 nil nil)))
  (should (equal (femacs-runtime-language--arity 1 2) '(1 2 nil)))
  (should (equal (femacs-runtime-language--arity 1 nil 2) '(1 nil (2))))
  (should (equal (femacs-runtime-language--arity 1 2 3 4) '(1 2 (3 4))))
  (should-error (femacs-runtime-language--arity)
                :type 'wrong-number-of-arguments)
  (let ((err (condition-case err
                 (funcall (lambda (a b) (list a b)) 1)
               (error err))))
    (should (eq (car err) 'wrong-number-of-arguments)))
  (femacs-runtime-language--record
   "rtl03-optional-rest-arity"
   (list (femacs-runtime-language--arity 1)
         (femacs-runtime-language--arity 1 2 3 4))))

(ert-deftest rtl04-left-to-right-effects-and-error ()
  (let* ((fn (lambda ()
               (list (progn (push :a femacs-runtime-language--order) 1)
                     (progn (push :b femacs-runtime-language--order)
                            (signal 'wrong-type-argument '(integerp "x")))
                     (progn (push :unreached femacs-runtime-language--order) 3))))
         (compiled (byte-compile fn))
         observations)
    (dolist (callable (list fn compiled))
      (let ((femacs-runtime-language--order nil)
            (events nil)
            (error-value nil))
        (setq error-value
              (condition-case err (funcall callable) (error err)))
        (setq events (nreverse femacs-runtime-language--order))
        (should (equal error-value '(wrong-type-argument integerp "x")))
        (should (equal events '(:a :b)))
        (push (list error-value events) observations)))
    (should (equal (car observations) (cadr observations)))
    (femacs-runtime-language--record
     "rtl04-left-to-right-effects-and-error" observations)))

(ert-deftest rtl05-catch-throw-unwind-and-quit ()
  (let ((events nil)
        result)
    (setq result
          (catch 'femacs-runtime-language-catch
            (unwind-protect
                (progn
                  (push :throw-body events)
                  (throw 'femacs-runtime-language-catch :thrown))
              (push :throw-cleanup events))))
    (should (eq result :thrown))
    (let ((inhibit-quit nil)
          (quit-result
           (condition-case err
               (unwind-protect
                   (signal 'quit nil)
                 (push :quit-cleanup events))
             (quit (push :quit-caught events) :caught))))
      (should (eq quit-result :caught)))
    (let ((final-events (nreverse events)))
      (should (equal final-events
                     '(:throw-body :throw-cleanup :quit-cleanup :quit-caught)))
      (femacs-runtime-language--record
       "rtl05-catch-throw-unwind-and-quit" final-events))))

(ert-deftest rtl06-unicode-text-and-markers ()
  (with-temp-buffer
    (insert "甲a🙂\nb")
    (let ((mark (copy-marker 3 t)))
      (unwind-protect
          (progn
            (goto-char 3)
            (insert "é")
            (let ((size (buffer-size))
                  (bytes (string-bytes (buffer-string))))
              (narrow-to-region 2 5)
              (let ((value (list (marker-position mark)
                                 (buffer-string)
                                 size
                                 bytes)))
                (should (equal value '(4 "aé🙂" 6 12)))
                (femacs-runtime-language--record
                 "rtl06-unicode-text-and-markers" value))))
        (set-marker mark nil)))))

(ert-deftest rtl07-reader-printer-roundtrip ()
  ;; High unsigned words must stay positive even when their signed machine
  ;; representation would fit a negative fixnum during reader construction.
  (dolist (pair '(("#xdfffffffffffffff" . "16140901064495857663")
                  ("#xe000000000000000" . "16140901064495857664")
                  ("#xffffffffffffffff" . "18446744073709551615")))
    (let ((number (read (car pair))))
      (should (bignump number))
      (should (equal (number-to-string number) (cdr pair)))
      (should (= (read (prin1-to-string number)) number))))
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
      (should (eq (car restored) :cycle))
      (femacs-runtime-language--record
       "rtl07-reader-printer-roundtrip" (list encoded (car ring))))))

(defun femacs-runtime-language--weak-live (mode)
  (let* ((key (list :key))
         (value (list :value))
         (roots (list key value))
         (table (make-hash-table :test 'eq :weakness mode)))
    (puthash key value table)
    (dotimes (_ 3) (garbage-collect))
    (should (memq key roots))
    (should (eq (gethash key table) value))
    (list (gethash key table) (hash-table-count table))))

(defun femacs-runtime-language--weak-probe (mode)
  (let ((table (make-hash-table :test 'eq :weakness mode)))
    (puthash (list :key) (list :value) table)
    table))

(ert-deftest rtl08-weak-modes-after-explicit-full ()
  (let ((live nil)
        (dead nil))
    (dolist (mode '(key value key-or-value key-and-value))
      (push (cons mode (femacs-runtime-language--weak-live mode)) live)
      (let ((table (femacs-runtime-language--weak-probe mode)))
        (dotimes (_ 8) (garbage-collect))
        (let ((count (hash-table-count table)))
          (should (memq count '(0 1)))
          (push (cons mode count) dead))))
    (femacs-runtime-language--record
     "rtl08-weak-modes-after-explicit-full"
     (list (nreverse live) (nreverse dead)))))

(defun femacs-runtime-language--install-finalizer (kind)
  (let ((cell (list :pending)))
    (pcase kind
      ('normal
       (make-finalizer
        (lambda ()
          (setcar cell :ran)
          (cl-incf femacs-runtime-language--finalizer-runs)))
       cell)
      ('error
       (make-finalizer
        (lambda ()
          (setcar cell :error)
          (cl-incf femacs-runtime-language--finalizer-errors)
          (error "rtl-finalizer-error")))
       cell)
      ('recursive
       (make-finalizer
        (lambda ()
          (setcar cell :recursive)
          (cl-incf femacs-runtime-language--finalizer-recursive)
          (garbage-collect)))
       cell))))

(defun femacs-runtime-language--install-captured-finalizer ()
  (let ((cell (list :target)))
    (make-finalizer
     (lambda ()
       (when (equal cell '(:target))
         (cl-incf femacs-runtime-language--finalizer-captured))))
    nil))

(ert-deftest rtl09-finalizer-once-error-and-finalizer-only ()
  (setq femacs-runtime-language--finalizer-runs 0
        femacs-runtime-language--finalizer-errors 0
        femacs-runtime-language--finalizer-recursive 0
        femacs-runtime-language--finalizer-captured 0)
  (let ((cells (list (femacs-runtime-language--install-finalizer 'normal)
                     (femacs-runtime-language--install-finalizer 'error)
                     (femacs-runtime-language--install-finalizer 'recursive))))
    (femacs-runtime-language--install-captured-finalizer)
    (dotimes (_ 8) (garbage-collect))
    (should (<= femacs-runtime-language--finalizer-runs 1))
    (should (<= femacs-runtime-language--finalizer-errors 1))
    (should (<= femacs-runtime-language--finalizer-recursive 1))
    (should (<= femacs-runtime-language--finalizer-captured 1))
    (dolist (cell cells)
      (should (memq (car cell) '(:pending :ran :error :recursive))))
    (femacs-runtime-language--record
     "rtl09-finalizer-once-error-and-finalizer-only"
     (list femacs-runtime-language--finalizer-runs
           femacs-runtime-language--finalizer-errors
           femacs-runtime-language--finalizer-recursive
           femacs-runtime-language--finalizer-captured
           (mapcar #'car cells)))))

;;; runtime-language.el ends here
