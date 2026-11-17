;;; gc-young-cadence.el --- Automatic young cadence fixture -*- lexical-binding: t; -*-

;; Functional fixture for the young-cons cadence design in
;; docs/gc-young-cadence.md.  Runs only on a collector-enabled product that
;; exposes femacs--gc-collector-status.  It never calls garbage-collect to
;; invent a minor, never widens retention caps, and never treats a reason-5
;; Full as Young success.
;;
;; Young reachability is exercised through a byte-compiled call/allocation
;; ring so Bcall..Bcall5's do_call housekeeping entry is required, not only
;; an interpreted while loop.  Major-control uses a finite Automatic budget.

(require 'ert)
(require 'cl-lib)
(require 'bytecomp)

(defconst femacs-gc-young-cadence--t-mode 0)
(defconst femacs-gc-young-cadence--t-kind 2)
(defconst femacs-gc-young-cadence--t-majors 3)
(defconst femacs-gc-young-cadence--t-minors 4)
(defconst femacs-gc-young-cadence--t-last-reason 18)
(defconst femacs-gc-young-cadence--t-block-capacity 23)

;; Finite Automatic budgets: large enough for cadence windows / ordinary
;; major pressure, small enough that most-positive-fixnum cannot hide a
;; missing trigger.  Percentage 0.0 keeps the absolute threshold decisive.
(defconst femacs-gc-young-cadence--major-threshold 8000)
(defconst femacs-gc-young-cadence--young-threshold 100000)

(defun femacs-gc-young-cadence--status ()
  "Return the collector status wire or fail if unavailable."
  (unless (fboundp 'femacs--gc-collector-status)
    (ert-fail "femacs--gc-collector-status is unavailable in this product"))
  ;; Bound the status read itself; do not use most-positive-fixnum as the
  ;; Automatic driver for the major-control case below.
  (let ((gc-cons-threshold femacs-gc-young-cadence--young-threshold)
        (gc-cons-percentage 0.0))
    (let ((status (femacs--gc-collector-status)))
      (unless (and (vectorp status) (= (length status) 3) (= (aref status 0) 1))
        (ert-fail (format "collector status wire mismatch: %S" status)))
      (let ((totals (aref status 1))
            (reasons (aref status 2)))
        (unless (and (vectorp totals) (= (length totals) 24))
          (ert-fail (format "collector totals mismatch: %S" totals)))
        (unless (and (vectorp reasons) (= (length reasons) 13))
          (ert-fail (format "collector reasons mismatch: %S" reasons))))
      status)))

(defun femacs-gc-young-cadence--total (status index)
  (let ((value (aref (aref status 1) index)))
    (unless (and (integerp value) (>= value 0))
      (ert-fail (format "collector total %d invalid: %S" index value)))
    value))

(defun femacs-gc-young-cadence--reason (status index)
  (let ((value (aref (aref status 2) index)))
    (unless (and (integerp value) (>= value 0))
      (ert-fail (format "collector reason %d invalid: %S" index value)))
    value))

(defun femacs-gc-young-cadence--inc (x)
  "Small callee for byte-compiled do_call rings."
  (1+ x))

(defun femacs-gc-young-cadence--byte-worker (n)
  "Allocate N conses while calling a byte-compiled helper each step."
  (let ((junk nil)
        (i 0))
    (while (< i n)
      (setq junk (cons (cons i nil) junk)
            i (femacs-gc-young-cadence--inc i)))
    (setq junk nil)
    i))

(defun femacs-gc-young-cadence--ensure-byte-ring ()
  "Byte-compile the worker and its callee used by the Automatic young case."
  (let ((byte-compile-delete-errors nil)
        (byte-compile-error-on-warn nil))
    (unless (byte-code-function-p (symbol-function 'femacs-gc-young-cadence--inc))
      (byte-compile 'femacs-gc-young-cadence--inc))
    (unless (byte-code-function-p (symbol-function 'femacs-gc-young-cadence--byte-worker))
      (byte-compile 'femacs-gc-young-cadence--byte-worker))
    (unless (and (byte-code-function-p (symbol-function 'femacs-gc-young-cadence--inc))
                 (byte-code-function-p (symbol-function 'femacs-gc-young-cadence--byte-worker)))
      (ert-fail "byte-compiled cadence call ring is unavailable"))))

(defun femacs-gc-young-cadence--churn-major (n)
  "Finite-threshold Automatic allocation for the major-layout control."
  (let ((gc-cons-threshold femacs-gc-young-cadence--major-threshold)
        (gc-cons-percentage 0.0)
        (junk nil))
    (dotimes (i n)
      (setq junk (cons i junk)))
    (setq junk nil)))

(defun femacs-gc-young-cadence--churn-young (n)
  "Byte-compiled call/allocation ring for the young Automatic case."
  (femacs-gc-young-cadence--ensure-byte-ring)
  (let ((gc-cons-threshold femacs-gc-young-cadence--young-threshold)
        (gc-cons-percentage 0.0))
    (femacs-gc-young-cadence--byte-worker n)))

(ert-deftest gyc01-explicit-full-unchanged ()
  "Explicit garbage-collect remains Full reason 2 under young mode."
  (let* ((before (femacs-gc-young-cadence--status))
         (mode (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-mode))
         (minors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-minors))
         (majors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-majors)))
    (should (memq mode '(0 1)))
    (garbage-collect)
    (let ((after (femacs-gc-young-cadence--status)))
      (should (= (femacs-gc-young-cadence--total after femacs-gc-young-cadence--t-minors)
                 minors))
      (should (> (femacs-gc-young-cadence--total after femacs-gc-young-cadence--t-majors)
                 majors))
      (should (= (femacs-gc-young-cadence--total after femacs-gc-young-cadence--t-kind) 0))
      (should (= (femacs-gc-young-cadence--total after femacs-gc-young-cadence--t-last-reason)
                 2)))))

(ert-deftest gyc02-automatic-young-or-major-control ()
  "Young mode completes a minor via byte-call ring; major control stays full."
  (let* ((before (femacs-gc-young-cadence--status))
         (mode (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-mode))
         (minors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-minors))
         (majors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-majors))
         (reason0 (femacs-gc-young-cadence--reason before 0))
         (reason3 (femacs-gc-young-cadence--reason before 3))
         (block (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-block-capacity))
         (budget (if (= mode 1)
                     (max 8000 (* 12 (max block 1)))
                   (max 16000 (* 4 femacs-gc-young-cadence--major-threshold))))
         (remaining budget)
         (saw-young nil)
         status)
    (should (> block 0))
    (while (and (> remaining 0) (not saw-young))
      (let ((chunk (min remaining (if (= mode 1) 64 256))))
        (if (= mode 1)
            (femacs-gc-young-cadence--churn-young chunk)
          (femacs-gc-young-cadence--churn-major chunk))
        (setq remaining (- remaining chunk)
              status (femacs-gc-young-cadence--status))
        (when (and (> (femacs-gc-young-cadence--total
                       status femacs-gc-young-cadence--t-minors)
                      minors)
                   (= (femacs-gc-young-cadence--total
                       status femacs-gc-young-cadence--t-kind)
                      1))
          (setq saw-young t))))
    (setq status (or status (femacs-gc-young-cadence--status)))
    (cond
     ((= mode 1)
      (should (byte-code-function-p
               (symbol-function 'femacs-gc-young-cadence--byte-worker)))
      (should (byte-code-function-p
               (symbol-function 'femacs-gc-young-cadence--inc)))
      (should saw-young)
      (should (> (femacs-gc-young-cadence--total
                  status femacs-gc-young-cadence--t-minors)
                 minors))
      (should (> (femacs-gc-young-cadence--reason status 0) reason0)))
     ((= mode 0)
      (should (not saw-young))
      (should (= (femacs-gc-young-cadence--total
                  status femacs-gc-young-cadence--t-minors)
                 minors))
      (should (> (femacs-gc-young-cadence--total
                  status femacs-gc-young-cadence--t-majors)
                 majors))
      (should (> (femacs-gc-young-cadence--reason status 3) reason3))
      (should (= (femacs-gc-young-cadence--total
                  status femacs-gc-young-cadence--t-kind)
                 0)))
     (t (ert-fail (format "unexpected collector mode %S" mode))))))
