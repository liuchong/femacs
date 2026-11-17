;;; gc-young-cadence.el --- Automatic young cadence fixture -*- lexical-binding: t; -*-

;; Functional fixture for the young-cons cadence design in
;; docs/gc-young-cadence.md.  Runs only on a collector-enabled product that
;; exposes femacs--gc-collector-status.  It never calls garbage-collect to
;; invent a minor, never widens retention caps, and never treats a reason-5
;; Full as Young success.

(require 'ert)
(require 'cl-lib)

(defconst femacs-gc-young-cadence--t-mode 0)
(defconst femacs-gc-young-cadence--t-kind 2)
(defconst femacs-gc-young-cadence--t-majors 3)
(defconst femacs-gc-young-cadence--t-minors 4)
(defconst femacs-gc-young-cadence--t-last-reason 18)
(defconst femacs-gc-young-cadence--t-block-capacity 23)

(defun femacs-gc-young-cadence--status ()
  "Return the collector status wire or fail if unavailable."
  (unless (fboundp 'femacs--gc-collector-status)
    (ert-fail "femacs--gc-collector-status is unavailable in this product"))
  (let ((gc-cons-threshold most-positive-fixnum))
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

(defun femacs-gc-young-cadence--churn (n)
  "Allocate N transient conses under ordinary Automatic safepoints."
  (let ((gc-cons-threshold most-positive-fixnum)
        (gc-cons-percentage 0.1)
        (junk nil))
    (dotimes (i n)
      (setq junk (cons i junk)))
    (setq junk nil)))

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
  "Young mode must complete at least one Automatic minor; major control stays full."
  (let* ((before (femacs-gc-young-cadence--status))
         (mode (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-mode))
         (minors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-minors))
         (majors (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-majors))
         (reason0 (femacs-gc-young-cadence--reason before 0))
         (reason3 (femacs-gc-young-cadence--reason before 3))
         (block (femacs-gc-young-cadence--total before femacs-gc-young-cadence--t-block-capacity))
         ;; Several cadence windows under the retention room, plus headroom for
         ;; non-cons allocation and safepoint lag.  Not a widened cap.
         (budget (max 8000 (* 12 (max block 1))))
         (remaining budget)
         (saw-young nil)
         status)
    (should (> block 0))
    (while (and (> remaining 0) (not saw-young))
      (let ((chunk (min remaining 64)))
        (femacs-gc-young-cadence--churn chunk)
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
      (should (> (femacs-gc-young-cadence--reason status 3) reason3)))
     (t (ert-fail (format "unexpected collector mode %S" mode))))))
