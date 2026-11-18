;;; g7-native-write-barrier.el --- native cons stores on a live old cons -*- lexical-binding: t; -*-

;; Correctness load for the G7 barrier argument: a native-compiled function
;; creates fresh conses and stores them into a cons that already survived a
;; full collection, then a minor must keep those children readable through the
;; old cell.  This is behavioural evidence only: it does not count barrier
;; calls, because the public interface exposes neither the emitted call nor a
;; barrier counter (disassemble returns no text for a native function and the
;; ELN is not materialised in an inspectable path).
;;
;; Runs only on a collector-enabled product with FEMACS_GC_COLLECTOR=young; the
;; cadence helpers come from gc-young-cadence.el so the minor is a real
;; Automatic cycle, not an invented one.

(require 'ert)
(load (expand-file-name "gc-young-cadence.el"
                        (file-name-directory (or load-file-name buffer-file-name))))

(defvar fe-native-barrier--cell nil
  "Holds the promoted cons so it stays a root across the minor.")

(defun fe-native-barrier--mode ()
  (femacs-gc-young-cadence--total (femacs-gc-young-cadence--status)
                                  femacs-gc-young-cadence--t-mode))

(defun fe-native-barrier--minors ()
  (femacs-gc-young-cadence--total (femacs-gc-young-cadence--status)
                                  femacs-gc-young-cadence--t-minors))

(defun fe-native-barrier--reason0 ()
  (femacs-gc-young-cadence--reason (femacs-gc-young-cadence--status) 0))

(defun fe-native-barrier--churn-until-minor (budget)
  "Allocate BUDGET conses in chunks until a minor completes; return its status."
  (let ((minors (fe-native-barrier--minors))
        (remaining budget)
        (saw nil)
        status)
    (while (and (> remaining 0) (not saw))
      (let ((chunk (min remaining 64)))
        (femacs-gc-young-cadence--churn-young chunk)
        (setq remaining (- remaining chunk)
              status (femacs-gc-young-cadence--status))
        (when (and (> (femacs-gc-young-cadence--total
                       status femacs-gc-young-cadence--t-minors)
                      minors)
                   (= (femacs-gc-young-cadence--total
                       status femacs-gc-young-cadence--t-kind)
                      1))
          (setq saw t))))
    (unless saw (ert-fail "no young cycle completed within the allocation budget"))
    status))

(ert-deftest fe-native-barrier-setcar-setcdr-on-live-old-cons ()
  "Native setcar/setcdr into a promoted cons survive an Automatic minor."
  (skip-unless (native-comp-available-p))
  (should (fboundp 'femacs--gc-collector-status))
  (should (= (fe-native-barrier--mode) 1)) ; young must be enabled
  (let* ((value 'write-barrier-payload)
         (cell (cons 'seed nil))
         (reason0-before (fe-native-barrier--reason0))
         (minors-before (fe-native-barrier--minors)))
    (setq fe-native-barrier--cell cell)
    ;; Promote: survive a full collection so the cell is old.
    (garbage-collect)
    (should (eq (car cell) 'seed))
    (should (null (cdr cell)))
    (message "native-barrier promoted mode=%d minors=%d" (fe-native-barrier--mode)
             minors-before)
    ;; Native writer: it allocates the children and stores them through
    ;; setcar and setcdr.  Neither child is referenced anywhere else.
    (let ((writer (native-compile
                   `(lambda (cell value)
                      (setcar cell (list 'native value))
                      (setcdr cell (list 'tail value))
                      cell))))
      (should (native-comp-function-p writer))
      (should (eq (funcall writer cell value) cell)))
    ;; Only the old cell now references the freshly written children.
    (should (consp (car cell)))
    (should (consp (cdr cell)))
    (let ((status (fe-native-barrier--churn-until-minor 8000)))
      (message "native-barrier after minor minors=%d reason0=%d last-kind=%d"
               (femacs-gc-young-cadence--total status femacs-gc-young-cadence--t-minors)
               (femacs-gc-young-cadence--reason status 0)
               (femacs-gc-young-cadence--total status femacs-gc-young-cadence--t-kind))
      (should (> (femacs-gc-young-cadence--total
                  status femacs-gc-young-cadence--t-minors)
                 minors-before))
      (should (> (femacs-gc-young-cadence--reason status 0) reason0-before)))
    ;; Behaviour after the minor: both children are still the written values.
    (should (consp (car cell)))
    (should (eq (car (car cell)) 'native))
    (should (eq (car (cdr (car cell))) value))
    (should (consp (cdr cell)))
    (should (eq (car (cdr cell)) 'tail))
    (should (eq (car (cdr (cdr cell))) value))
    (setq fe-native-barrier--cell nil)))
