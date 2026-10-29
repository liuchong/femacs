;;; runtime-native-lifecycle.el --- Native lifetime cases -*- lexical-binding: t; -*-
;; Definitions only; load runtime-native.el first in the final driver.
(require 'ert)
(require 'cl-lib)

(defvar femacs-runtime-native-held nil)
(defvar femacs-runtime-native-weak nil)
(defvar femacs-runtime-recursive-active nil)
(defvar femacs-runtime-recursive-count 0)

(ert-deftest rtn12-native-unit-public-accessors ()
  (should (fboundp 'femacs-rtn--compile-form))
  (dolist (function '(subr-native-comp-unit native-comp-unit-file native-comp-unit-set-file))
    (should (fboundp function)))
  (let* ((function (femacs-rtn--compile-form '(lambda (x) (+ x 1))))
         (unit (subr-native-comp-unit function))
         (file (native-comp-unit-file unit))
         (replacement (femacs-rtn--path "unit-accessor.eln"))
         (gc-cons-threshold most-positive-fixnum))
    (should unit)
    (should (stringp file))
    (unwind-protect
        (progn
          (should (eq unit (native-comp-unit-set-file unit replacement)))
          (should (equal (native-comp-unit-file unit) replacement)))
      (native-comp-unit-set-file unit file))
    (should (= (funcall function 41) 42))
    (should-error (native-comp-unit-file nil) :type 'wrong-type-argument)
    (should-error (native-comp-unit-set-file nil file) :type 'wrong-type-argument)))

(ert-deftest rtn13-recursive-load-preserves-live-relocations ()
  (should (fboundp 'femacs-rtn--compile-file))
  (let* ((old (getenv "FEMACS_RUNTIME_RECURSIVE_UNIT"))
         (femacs-runtime-recursive-count 0)
         (femacs-runtime-recursive-active nil)
         (unit
          (femacs-rtn--compile-file
           "recursive-unit"
           "(defvar femacs-runtime-recursive-active nil)
(defvar femacs-runtime-recursive-count 0)
(setq femacs-runtime-recursive-count (1+ femacs-runtime-recursive-count))
(if femacs-runtime-recursive-active
    (garbage-collect)
  (let ((femacs-runtime-recursive-active t))
    (native-elisp-load (getenv \"FEMACS_RUNTIME_RECURSIVE_UNIT\"))))
(defun femacs-runtime-recursive-value (x) (list :recursive x \"中文\"))
")))
    (unwind-protect
        (progn
          (setenv "FEMACS_RUNTIME_RECURSIVE_UNIT" unit)
          (native-elisp-load unit)
          (should (= femacs-runtime-recursive-count 2))
          (should (native-comp-function-p (symbol-function 'femacs-runtime-recursive-value)))
          (garbage-collect)
          (should (equal (femacs-runtime-recursive-value 7) '(:recursive 7 "中文"))))
      (setenv "FEMACS_RUNTIME_RECURSIVE_UNIT" old))))

(defun femacs-runtime-native-prepare-held ()
  "Return only the unit file; keep function rooted separately, unit weakly."
  (let ((file
         (femacs-rtn--compile-file
          "held-code"
          "(defun femacs-runtime-held-value (x) (list :held (+ x 1) \"中文\"))\n")))
    (native-elisp-load file)
    (setq femacs-runtime-native-held (symbol-function 'femacs-runtime-held-value)
          femacs-runtime-native-weak (make-hash-table :weakness 'value))
    (puthash :unit (subr-native-comp-unit femacs-runtime-native-held) femacs-runtime-native-weak)
    (fmakunbound 'femacs-runtime-held-value)
    file))

(defun femacs-runtime-native-collect-released-unit ()
  "Observe bounded actual collection; absence of observation is a failure."
  (let ((round 0))
    (while (and (< round 20) (gethash :unit femacs-runtime-native-weak))
      ;; Allocate fresh frames/data between full collections. No claim that
      ;; this eliminates every possible conservative stack reference.
      (let ((scratch (make-list 4000 round))) (length scratch))
      (garbage-collect)
      (setq round (1+ round)))
    (not (gethash :unit femacs-runtime-native-weak))))

(ert-deftest rtn14-live-native-function-keeps-unit-through-gc ()
  (should (fboundp 'subr-native-comp-unit))
  (let* ((file (femacs-runtime-native-prepare-held))
         (saved (concat file ".retained")))
    (unwind-protect
        (progn
          ;; Preserve bytes while the original filename is unavailable.
          (rename-file file saved)
          (dotimes (_ 4)
            (garbage-collect)
            (should (gethash :unit femacs-runtime-native-weak))
            (should (native-comp-function-p femacs-runtime-native-held))
            (should (equal (funcall femacs-runtime-native-held 41) '(:held 42 "中文")))))
      (when (and (file-exists-p saved) (not (file-exists-p file)))
        (rename-file saved file))
      (setq femacs-runtime-native-held nil))))

(ert-deftest rtn15-unreachable-native-unit-is-not-strong-cache-rooted ()
  (unwind-protect
      (progn
        (femacs-runtime-native-prepare-held)
        (setq femacs-runtime-native-held nil)
        (should (femacs-runtime-native-collect-released-unit)))
    (setq femacs-runtime-native-held nil
          femacs-runtime-native-weak nil)))

;;; runtime-native-lifecycle.el ends here
