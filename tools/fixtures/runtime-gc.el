;;; runtime-gc.el --- Runtime collector-profile B cases -*- lexical-binding: t; -*-

;; Profile-only ERT cases for the planned generation-aware collector.  They
;; call the private collector status API and are not loaded/run on the stable
;; baseline.  The status wire is [1 totals reason-counts]; totals has 24
;; numeric slots and reason-counts has 13, as bound in
;; docs/runtime-modernization.md.
;;
;; These cases do not allocate their own mock status vector, do not call
;; garbage-collect to pretend it produced a minor, and do not replace weak
;; semantics with strong roots.  Raw-alias/component-only, free-list/bitmap
;; boundary and dump-held paths remain primary component/build cases.  A
;; bounded actual allocation below exercises the allocation-cap Full path.

(require 'ert)
(require 'cl-lib)

(defconst femacs-runtime-gc--totals-count 24)
(defconst femacs-runtime-gc--reasons-count 13)

(defconst femacs-runtime-gc--t-mode 0)
(defconst femacs-runtime-gc--t-valid 1)
(defconst femacs-runtime-gc--t-kind 2)
(defconst femacs-runtime-gc--t-majors 3)
(defconst femacs-runtime-gc--t-minors 4)
(defconst femacs-runtime-gc--t-consecutive 5)
(defconst femacs-runtime-gc--t-last-major-cons 6)
(defconst femacs-runtime-gc--t-young-survivors 7)
(defconst femacs-runtime-gc--t-young-frees 8)
(defconst femacs-runtime-gc--t-dirty 9)
(defconst femacs-runtime-gc--t-raw 10)
(defconst femacs-runtime-gc--t-registry-len 11)
(defconst femacs-runtime-gc--t-registry-overflows 12)
(defconst femacs-runtime-gc--t-storage-invalidations 13)
(defconst femacs-runtime-gc--t-unknown-producer 14)
(defconst femacs-runtime-gc--t-allocated-cons 15)
(defconst femacs-runtime-gc--t-allocated-bytes 16)
(defconst femacs-runtime-gc--t-full-fallbacks 17)
(defconst femacs-runtime-gc--t-last-reason 18)
(defconst femacs-runtime-gc--t-startup-fulls 19)
(defconst femacs-runtime-gc--t-explicit-fulls 20)
(defconst femacs-runtime-gc--t-cap-fulls 21)
(defconst femacs-runtime-gc--t-cycle-sequence 22)
(defconst femacs-runtime-gc--t-block-capacity 23)

(defconst femacs-runtime-gc--reason-pending-doomed 12)

(defvar femacs-runtime-gc--roots nil)
(defvar femacs-runtime-gc--buffer-local nil)
(defvar femacs-runtime-gc--finalizer-runs 0)
(defvar femacs-runtime-gc--finalizer-cell nil)
(defvar femacs-runtime-gc--error-runs 0)
(defvar femacs-runtime-gc--recursive-runs 0)
(defvar femacs-runtime-gc--pending-a 0)
(defvar femacs-runtime-gc--pending-b 0)

(defun femacs-runtime-gc--status ()
  "Return and validate the collector status wire."
  (unless (fboundp 'femacs--gc-collector-status)
    (ert-fail "femacs--gc-collector-status is unavailable in this profile"))
  (let ((gc-cons-threshold most-positive-fixnum))
    (let ((status (femacs--gc-collector-status)))
      (unless (vectorp status)
        (ert-fail (format "collector status is not a vector: %S" status)))
      (unless (= (length status) 3)
        (ert-fail (format "collector status length is not 3: %S" status)))
      (unless (= (aref status 0) 1)
        (ert-fail (format "collector status schema is not 1: %S" status)))
      (let ((totals (aref status 1))
            (reasons (aref status 2)))
        (unless (and (vectorp totals)
                     (= (length totals) femacs-runtime-gc--totals-count))
          (ert-fail (format "collector totals are not a %d-slot vector: %S"
                            femacs-runtime-gc--totals-count totals)))
        (unless (and (vectorp reasons)
                     (= (length reasons) femacs-runtime-gc--reasons-count))
          (ert-fail (format "collector reasons are not a %d-slot vector: %S"
                            femacs-runtime-gc--reasons-count reasons))))
      status)))

(defun femacs-runtime-gc--total (status index)
  (let ((value (aref (aref status 1) index)))
    (unless (integerp value)
      (ert-fail (format "collector total %d is not an integer: %S" index value)))
    (unless (>= value 0)
      (ert-fail (format "collector total %d is negative: %S" index value)))
    value))

(defun femacs-runtime-gc--reason (status index)
  (let ((value (aref (aref status 2) index)))
    (unless (and (integerp value) (>= value 0))
      (ert-fail (format "collector reason count %d is invalid: %S" index value)))
    value))

(defun femacs-runtime-gc--churn (iterations)
  "Allocate bounded transient conses so automatic collection can run."
  (let ((gc-cons-threshold 30000)
        (gc-cons-percentage 0.0)
        (junk nil))
    (dotimes (i iterations)
      (setq junk (cons i junk)))
    (setq junk nil)))

(defun femacs-runtime-gc--until-minor (before &optional rounds)
  "Capture a completed Young after BEFORE within the original cons budget."
  (let ((remaining (* (or rounds 8) 4000))
        (found nil)
        status)
    ;; A large chunk can contain Young followed by a required cap Full.  Never
    ;; accept that Full as the Young window required by survival/finalizer tests.
    ;; Small chunks improve observation; they do not assume GC cannot recurse.
    (while (and (> remaining 0) (not found))
      (let ((chunk (min remaining 128)))
        (femacs-runtime-gc--churn chunk)
        (setq remaining (- remaining chunk)
              status (femacs-runtime-gc--status))
        (when (and (> (femacs-runtime-gc--total
                       status femacs-runtime-gc--t-minors) before)
                   (= (femacs-runtime-gc--total
                       status femacs-runtime-gc--t-kind) 1))
          (setq found status))))
    (unless found
      (ert-fail (format "No completed Young snapshot within %d conses; status=%S"
                        (* (or rounds 8) 4000) status)))
    found))

(defun femacs-runtime-gc--setcar-fresh (cell)
  (let ((young (list :young-car)))
    (setcar cell young)
    nil))

(defun femacs-runtime-gc--setcdr-fresh (cell)
  (let ((young (list :young-cdr)))
    (setcdr cell young)
    nil))

(defun femacs-runtime-gc--store-symbol (symbol)
  (set symbol (list :symbol))
  nil)

(defun femacs-runtime-gc--store-vector (vector)
  (aset vector 0 (list :vector))
  nil)

(defun femacs-runtime-gc--store-hash (table)
  (puthash :key (list :hash) table)
  nil)

(defun femacs-runtime-gc--store-buffer (buffer)
  (with-current-buffer buffer
    (setq femacs-runtime-gc--buffer-local (list :buffer)))
  nil)

(defun femacs-runtime-gc--weak-put (table key)
  (puthash key (vector (list :weak-young)) table)
  nil)

(defun femacs-runtime-gc--check-weak (mode key table)
  (let ((value (gethash key table)))
    (if value
        (progn
          (should (vectorp value))
          (should (consp (aref value 0)))
          (should (eq (car (aref value 0)) :weak-young)))
      (should (memq mode '(value key-and-value))))))

(defun femacs-runtime-gc--install-finalizer-callback ()
  (let ((cell (list :alive)))
    (make-finalizer
     (lambda ()
       (cl-incf femacs-runtime-gc--finalizer-runs)
       (setq femacs-runtime-gc--finalizer-cell (car cell))))
    nil))

(defun femacs-runtime-gc--install-error-finalizer ()
  (make-finalizer
   (lambda ()
     (cl-incf femacs-runtime-gc--error-runs)
     (error "rtg-finalizer-error")))
  nil)

(defun femacs-runtime-gc--install-recursive-finalizer ()
  (make-finalizer
   (lambda ()
     (cl-incf femacs-runtime-gc--recursive-runs)
     (garbage-collect)))
  nil)

(defun femacs-runtime-gc--install-pending-pair ()
  ;; A is created first so the doomed queue runs A before B.  A's nested
  ;; explicit full sees the still-doomed B and must select Full reason 12.
  (make-finalizer
   (lambda ()
     (cl-incf femacs-runtime-gc--pending-a)
     (garbage-collect)))
  (make-finalizer
   (lambda ()
     (cl-incf femacs-runtime-gc--pending-b)))
  nil)

(ert-deftest rtg01-status-wire-shape ()
  (let* ((status (femacs-runtime-gc--status))
         (totals (aref status 1))
         (reasons (aref status 2)))
    (should (= (length status) 3))
    (should (= (aref status 0) 1))
    (should (= (length totals) 24))
    (should (= (length reasons) 13))
    (dotimes (i 24)
      (should (integerp (aref totals i)))
      (should (>= (aref totals i) 0)))
    (dotimes (i 13)
      (should (integerp (aref reasons i)))
      (should (>= (aref reasons i) 0)))
    (should (memq (aref totals femacs-runtime-gc--t-mode) '(0 1)))
    (should (memq (aref totals femacs-runtime-gc--t-valid) '(0 1)))
    (should (memq (aref totals femacs-runtime-gc--t-kind) '(0 1 2)))
    (should (memq (aref totals femacs-runtime-gc--t-unknown-producer) '(0 1)))
    (should (<= 0 (aref totals femacs-runtime-gc--t-last-reason) 12))
    (should (> (aref totals femacs-runtime-gc--t-block-capacity) 0))))

(ert-deftest rtg02-explicit-full-is-full ()
  (let* ((before (femacs-runtime-gc--status))
         (majors (femacs-runtime-gc--total before femacs-runtime-gc--t-majors))
         (minors (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
         (explicit (femacs-runtime-gc--total
                    before femacs-runtime-gc--t-explicit-fulls)))
    (garbage-collect)
    (let ((after (femacs-runtime-gc--status)))
      (should (= (femacs-runtime-gc--total after femacs-runtime-gc--t-minors)
                 minors))
      (should (> (femacs-runtime-gc--total after femacs-runtime-gc--t-majors)
                 majors))
      (should (= (femacs-runtime-gc--total after femacs-runtime-gc--t-kind)
                 0))
      (should (> (femacs-runtime-gc--total
                  after femacs-runtime-gc--t-explicit-fulls)
                 explicit))
      (should (= (femacs-runtime-gc--total after femacs-runtime-gc--t-last-reason)
                 2)))))

(ert-deftest rtg03-automatic-churn-young-or-major-control ()
  (let* ((before (femacs-runtime-gc--status))
         (mode (femacs-runtime-gc--total before femacs-runtime-gc--t-mode))
         (majors (femacs-runtime-gc--total before femacs-runtime-gc--t-majors))
         (minors (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
         (reason3 (femacs-runtime-gc--reason before 3)))
    (if (= mode 0)
        (progn
          (femacs-runtime-gc--churn 12000)
          (let ((after (femacs-runtime-gc--status)))
            (should (= (femacs-runtime-gc--total
                        after femacs-runtime-gc--t-minors)
                       minors))
            (should (> (femacs-runtime-gc--total
                        after femacs-runtime-gc--t-majors)
                       majors))
            (should (= (femacs-runtime-gc--total
                        after femacs-runtime-gc--t-kind)
                       0))
            (should (>= (femacs-runtime-gc--reason after 3) reason3))))
      (let ((after (femacs-runtime-gc--until-minor minors)))
        (should (> (femacs-runtime-gc--total
                    after femacs-runtime-gc--t-minors)
                   minors))
        (should (= (femacs-runtime-gc--total after femacs-runtime-gc--t-kind)
                   1))
        (should (> (femacs-runtime-gc--total
                    after femacs-runtime-gc--t-cycle-sequence)
                   (femacs-runtime-gc--total
                    before femacs-runtime-gc--t-cycle-sequence)))))))

(ert-deftest rtg04-old-cons-car-cdr-to-young ()
  (let* ((before (femacs-runtime-gc--status))
         (mode (femacs-runtime-gc--total before femacs-runtime-gc--t-mode))
         (old-car (cons :old :old))
         (old-cdr (cons :old :old)))
    (let ((femacs-runtime-gc--roots (list old-car old-cdr)))
      (if (= mode 0)
          (progn
            (let ((gc-cons-threshold most-positive-fixnum))
              (femacs-runtime-gc--setcar-fresh old-car)
              (femacs-runtime-gc--setcdr-fresh old-cdr))
            (garbage-collect)
            (should (= (femacs-runtime-gc--total
                        (femacs-runtime-gc--status)
                        femacs-runtime-gc--t-minors)
                       (femacs-runtime-gc--total
                        before femacs-runtime-gc--t-minors))))
        (let* ((first (femacs-runtime-gc--until-minor
                       (femacs-runtime-gc--total
                        before femacs-runtime-gc--t-minors)))
               (first-minors (femacs-runtime-gc--total
                              first femacs-runtime-gc--t-minors)))
          (let ((gc-cons-threshold most-positive-fixnum))
            (femacs-runtime-gc--setcar-fresh old-car)
            (femacs-runtime-gc--setcdr-fresh old-cdr))
          (femacs-runtime-gc--until-minor first-minors)
          (should (equal (car old-car) '(:young-car)))
          (should (equal (cdr old-cdr) '(:young-cdr)))
          (garbage-collect)))
      (should (equal (car old-car) '(:young-car)))
      (should (equal (cdr old-cdr) '(:young-cdr)))
      (setq femacs-runtime-gc--roots nil))))

(ert-deftest rtg05-old-noncons-owners-to-young ()
  (let ((symbol (make-symbol "femacs-rtg-owner-symbol"))
        (vector (vector nil))
        (table (make-hash-table :test 'eq))
        (buffer (generate-new-buffer " *femacs-rtg-owner*")))
    (unwind-protect
        (progn
          (with-current-buffer buffer
            (make-local-variable 'femacs-runtime-gc--buffer-local))
          (let ((gc-cons-threshold most-positive-fixnum))
            (garbage-collect)
            (femacs-runtime-gc--store-symbol symbol)
            (femacs-runtime-gc--store-vector vector)
            (femacs-runtime-gc--store-hash table)
            (femacs-runtime-gc--store-buffer buffer))
          (let* ((before (femacs-runtime-gc--status))
                 (mode (femacs-runtime-gc--total
                        before femacs-runtime-gc--t-mode)))
            (if (= mode 1)
                (femacs-runtime-gc--until-minor
                 (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
              (femacs-runtime-gc--churn 8000)))
          (should (equal (symbol-value symbol) '(:symbol)))
          (should (equal (aref vector 0) '(:vector)))
          (should (equal (gethash :key table) '(:hash)))
          (should (equal (with-current-buffer buffer
                           femacs-runtime-gc--buffer-local)
                         '(:buffer)))
          (garbage-collect)
          (should (equal (symbol-value symbol) '(:symbol)))
          (should (equal (aref vector 0) '(:vector)))
          (should (equal (gethash :key table) '(:hash)))
          (should (equal (with-current-buffer buffer
                           femacs-runtime-gc--buffer-local)
                         '(:buffer))))
      (kill-buffer buffer))))

(ert-deftest rtg06-weak-new-noncons-cons-children ()
  (let* ((modes '(key value key-or-value key-and-value))
         (keys (mapcar (lambda (mode)
                         (intern (format "femacs-rtg-weak-%s" mode)))
                       modes))
         (tables (mapcar (lambda (mode)
                           (make-hash-table :test 'eq :weakness mode))
                         modes)))
    (let ((gc-cons-threshold most-positive-fixnum))
      (garbage-collect)
      (cl-loop for table in tables
               for key in keys
               do (femacs-runtime-gc--weak-put table key)))
    (let* ((before (femacs-runtime-gc--status))
           (mode (femacs-runtime-gc--total before femacs-runtime-gc--t-mode)))
      (if (= mode 1)
          (femacs-runtime-gc--until-minor
           (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
        (femacs-runtime-gc--churn 8000)))
    (cl-loop for mode in modes
             for key in keys
             for table in tables
             do (femacs-runtime-gc--check-weak mode key table))
    (garbage-collect)
    (cl-loop for mode in modes
             for key in keys
             for table in tables
             do (femacs-runtime-gc--check-weak mode key table))))

(ert-deftest rtg07-new-finalizer-callback-roots ()
  (setq femacs-runtime-gc--finalizer-runs 0
        femacs-runtime-gc--finalizer-cell nil)
  (let* ((before (femacs-runtime-gc--status))
         (mode (femacs-runtime-gc--total before femacs-runtime-gc--t-mode)))
    (let ((gc-cons-threshold most-positive-fixnum))
      (garbage-collect)
      (femacs-runtime-gc--install-finalizer-callback))
    (if (= mode 1)
        (progn
          (femacs-runtime-gc--until-minor
           (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
          (should (= femacs-runtime-gc--finalizer-runs 0)))
      (femacs-runtime-gc--churn 8000))
    (dotimes (_ 8) (garbage-collect))
    (should (= femacs-runtime-gc--finalizer-runs 1))
    (when (> femacs-runtime-gc--finalizer-runs 0)
      (should (eq femacs-runtime-gc--finalizer-cell :alive)))))

(ert-deftest rtg08-finalizer-error-and-recursive ()
  (setq femacs-runtime-gc--error-runs 0
        femacs-runtime-gc--recursive-runs 0)
  (femacs-runtime-gc--install-error-finalizer)
  (femacs-runtime-gc--install-recursive-finalizer)
  (dotimes (_ 8)
    (garbage-collect)
    (make-list 1000 nil))
  (should (= femacs-runtime-gc--error-runs 1))
  (should (= femacs-runtime-gc--recursive-runs 1)))

(ert-deftest rtg09-pending-doomed-forces-full ()
  (setq femacs-runtime-gc--pending-a 0
        femacs-runtime-gc--pending-b 0)
  (let ((before (femacs-runtime-gc--reason
                 (femacs-runtime-gc--status)
                 femacs-runtime-gc--reason-pending-doomed)))
    (femacs-runtime-gc--install-pending-pair)
    (dotimes (_ 16)
      (garbage-collect)
      (make-list 1000 nil))
    (let ((after (femacs-runtime-gc--reason
                  (femacs-runtime-gc--status)
                  femacs-runtime-gc--reason-pending-doomed)))
      (should (> after before)))
    (should (= femacs-runtime-gc--pending-a 1))
    (should (= femacs-runtime-gc--pending-b 1))))

(ert-deftest rtg10-cap-selected-full ()
  (let* ((initial (femacs-runtime-gc--status))
         (mode (femacs-runtime-gc--total initial femacs-runtime-gc--t-mode)))
    (if (= mode 0)
        (progn
          (garbage-collect)
          (let ((after (femacs-runtime-gc--status)))
            (should (= (femacs-runtime-gc--total
                        after femacs-runtime-gc--t-minors)
                       (femacs-runtime-gc--total
                        initial femacs-runtime-gc--t-minors)))
            (should (> (femacs-runtime-gc--total
                        after femacs-runtime-gc--t-majors)
                       (femacs-runtime-gc--total
                        initial femacs-runtime-gc--t-majors)))))
      (garbage-collect)
      (let* ((before (femacs-runtime-gc--status))
             (live (femacs-runtime-gc--total
                    before femacs-runtime-gc--t-last-major-cons))
             (cap (+ live 1998))
             (allocation (+ cap 2048))
             (bytes (* 16 allocation))
             (reason5 (femacs-runtime-gc--reason before 5))
             (junk nil))
        (let ((gc-cons-threshold bytes)
              (gc-cons-percentage 0.0))
          (dotimes (_ allocation)
            (setq junk (cons nil junk))))
        (setq junk nil)
        (let ((after (femacs-runtime-gc--status)))
          (should (> (femacs-runtime-gc--reason after 5) reason5))
          (should (> (femacs-runtime-gc--total
                      after femacs-runtime-gc--t-majors)
                     (femacs-runtime-gc--total
                      before femacs-runtime-gc--t-majors))))))))

;;; runtime-gc.el ends here
