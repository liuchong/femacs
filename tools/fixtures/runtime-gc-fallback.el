;;; runtime-gc-fallback.el --- Permanent Full fallback integration fixture -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

;; Integration fixture for P437: after a persistent collector fallback latch
;; (`UNKNOWN_PRODUCER` via `module-load' -> `dynlib_open'), subsequent explicit
;; and automatic collections stay on the Full path, preserve multi-generation
;; cons/vector/string/float/symbol/hash-table reachability and public error
;; semantics, and write deterministic snapshots/results to `gc-fallback.json'
;; inside `FEMACS_RUNTIME_OUT' without host paths.

(require 'ert)
(require 'cl-lib)
(require 'json)

(unless (fboundp 'femacs-runtime-gc--status)
  (let ((sibling (expand-file-name
                  "runtime-gc.el"
                  (file-name-directory (or load-file-name buffer-file-name "")))))
    (when (file-readable-p sibling)
      (load sibling nil t t))))

(dolist (fn '(femacs-runtime-gc--status
              femacs-runtime-gc--total
              femacs-runtime-gc--reason
              femacs-runtime-gc--churn))
  (unless (fboundp fn)
    (error "runtime-gc-fallback requires helper `%s' from runtime-gc.el" fn)))

(dolist (sym '(femacs-runtime-gc--t-mode
               femacs-runtime-gc--t-valid
               femacs-runtime-gc--t-kind
               femacs-runtime-gc--t-majors
               femacs-runtime-gc--t-minors
               femacs-runtime-gc--t-consecutive
               femacs-runtime-gc--t-registry-len
               femacs-runtime-gc--t-registry-overflows
               femacs-runtime-gc--t-unknown-producer
               femacs-runtime-gc--t-full-fallbacks
               femacs-runtime-gc--t-last-reason
               femacs-runtime-gc--t-explicit-fulls
               femacs-runtime-gc--t-cycle-sequence
               femacs-runtime-gc--roots))
  (unless (boundp sym)
    (error "runtime-gc-fallback requires symbol `%s' from runtime-gc.el" sym)))

(defvar femacs-runtime-language--observations nil)
(defvar femacs-runtime-gc-fallback--last-observation nil)

(defconst femacs-runtime-gc-fallback--schema "femacs-runtime-gc-fallback-v1")
(defconst femacs-runtime-gc-fallback--case-id
  "rtg-fallback-01-unknown-producer-permanent-full-and-registry")
(defconst femacs-runtime-gc-fallback--missing-basename
  "nonexistent-producer-module.so")
(defconst femacs-runtime-gc-fallback--output-name
  "gc-fallback.json")

(defun femacs-runtime-gc-fallback--out-dir ()
  "Return the caller-owned `FEMACS_RUNTIME_OUT' directory."
  (let ((out (getenv "FEMACS_RUNTIME_OUT"))
        (root (getenv "FEMACS_RUNTIME_TEST_ROOT")))
    (cond
     ((and out (> (length out) 0))
      (let ((dir (file-name-as-directory (expand-file-name out))))
        (make-directory dir t)
        dir))
     ((and root (> (length root) 0))
      (let ((dir (file-name-as-directory
                  (expand-file-name "out" (expand-file-name root)))))
        (make-directory dir t)
        dir))
     (t
      (error "FEMACS_RUNTIME_OUT is not set")))))

(defun femacs-runtime-gc-fallback--write-report (payload)
  "Write PAYLOAD to `gc-fallback.json' in `FEMACS_RUNTIME_OUT' and record observation."
  (setq femacs-runtime-gc-fallback--last-observation payload)
  (setf (alist-get femacs-runtime-gc-fallback--case-id
                   femacs-runtime-language--observations
                   nil nil #'equal)
        (prin1-to-string payload))
  (let* ((dir (femacs-runtime-gc-fallback--out-dir))
         (file (expand-file-name femacs-runtime-gc-fallback--output-name dir))
         (coding-system-for-write 'utf-8-unix))
    (with-temp-file file
      (insert (json-encode payload))
      (insert "\n")))
  payload)

(defun femacs-runtime-gc-fallback--owned-temp-dir ()
  "Create an owned temporary directory under `FEMACS_RUNTIME_TEST_ROOT' when set."
  (let* ((env-root (getenv "FEMACS_RUNTIME_TEST_ROOT"))
         (parent (if (and env-root (> (length env-root) 0))
                     (expand-file-name env-root)
                   temporary-file-directory)))
    (make-directory parent t)
    (make-temp-file (expand-file-name "femacs-rtg-fallback-" parent) t)))

(defun femacs-runtime-gc-fallback--until-auto-full (before-majors &optional rounds)
  "Churn bounded transient conses (with `gc-cons-percentage' 0.0) until an automatic Full GC completes."
  (let ((remaining (* (or rounds 4) 6000))
        (found nil)
        status)
    ;; `consing_threshold' clamps `gc-cons-threshold' to at least
    ;; `GC_DEFAULT_THRESHOLD / 10' (80,000 bytes = 5,000 conses) when
    ;; `gc-cons-percentage' is 0.0.  A 6,000-cons chunk (96,000 bytes)
    ;; deterministically crosses that threshold within a single call.
    (while (and (> remaining 0) (not found))
      (let ((chunk (min remaining 6000)))
        (femacs-runtime-gc--churn chunk)
        (setq remaining (- remaining chunk)
              status (femacs-runtime-gc--status))
        (when (> (femacs-runtime-gc--total
                  status femacs-runtime-gc--t-majors)
                 before-majors)
          (setq found status))))
    (unless found
      (ert-fail (format "No completed automatic Full snapshot within %d conses; status=%S"
                        (* (or rounds 4) 6000) status)))
    found))

(defun femacs-runtime-gc-fallback--build-gen0 ()
  "Construct initial multi-type live graph before permanent fallback."
  (let ((old-cons (cons (cons :gen0-car (copy-sequence "gen0-car-str"))
                        (list :gen0-cdr (+ 1.0 0.25))))
        (old-vec (vector (copy-sequence "gen0-vec-str")
                         (+ 2.0 0.5)
                         (list :gen0-vec-cons 10 (copy-sequence "gen0-vec-leaf"))
                         nil))
        (old-sym (make-symbol "femacs-rtg-fallback-owner-sym"))
        (old-table (make-hash-table :test 'equal)))
    (set old-sym (vector (copy-sequence "gen0-sym-str")
                         (+ 3.0 0.75)
                         nil))
    (setplist old-sym (list :gen0-prop (copy-sequence "gen0-prop-str")
                            :gen0-float (+ 5.0 0.125)))
    (puthash "gen0-key"
             (list :gen0-table-val
                   (+ 4.0 0.125)
                   (copy-sequence "gen0-table-str")
                   (vector :gen0-table-vec (copy-sequence "gen0-table-leaf")))
             old-table)
    (list :cons old-cons :vec old-vec :sym old-sym :table old-table)))

(defun femacs-runtime-gc-fallback--attach-gen1 (graph)
  "Attach Gen 1 cons/vector/string/float children to Gen 0 owners in GRAPH."
  (let ((old-cons (plist-get graph :cons))
        (old-vec (plist-get graph :vec))
        (old-sym (plist-get graph :sym))
        (old-table (plist-get graph :table)))
    (setcdr old-cons
            (list :gen1-cdr
                  (copy-sequence "gen1-cons-str")
                  (+ 10.0 0.5)
                  (vector :gen1-nested-vec
                          (copy-sequence "gen1-nested-str")
                          (+ 11.0 0.25)
                          nil)))
    (aset old-vec 3
          (list :gen1-vec-tail
                (copy-sequence "gen1-vec-tail-str")
                (+ 12.0 0.75)
                (vector :gen1-vec-child (copy-sequence "gen1-vec-child-str") nil)))
    (aset (symbol-value old-sym) 2
          (cons (copy-sequence "gen1-sym-tail") (+ 13.0 0.125)))
    (puthash "gen1-key"
             (vector :gen1-table-entry
                     (copy-sequence "gen1-table-str")
                     (+ 14.0 0.625)
                     (list :gen1-table-cons 42 (copy-sequence "gen1-table-leaf"))
                     nil)
             old-table)
    nil))

(defun femacs-runtime-gc-fallback--attach-gen2 (graph)
  "Attach Gen 2 cons/vector/string/float children to Gen 1 nodes in GRAPH."
  (let* ((old-cons (plist-get graph :cons))
         (gen1-nested-vec (nth 3 (cdr old-cons)))
         (old-vec (plist-get graph :vec))
         (gen1-vec-child (nth 3 (aref old-vec 3)))
         (old-table (plist-get graph :table))
         (gen1-table-vec (gethash "gen1-key" old-table)))
    (aset gen1-nested-vec 3
          (list :gen2-cons
                (copy-sequence "gen2-cons-str")
                (+ 20.0 0.5)
                (vector :gen2-vec (copy-sequence "gen2-vec-str") (+ 21.0 0.25) nil)))
    (aset gen1-vec-child 2
          (cons (copy-sequence "gen2-child-str") (+ 22.0 0.75)))
    (aset gen1-table-vec 4
          (list :gen2-table-tail (copy-sequence "gen2-table-str") (+ 23.0 0.125)))
    nil))

(defun femacs-runtime-gc-fallback--attach-gen3 (graph)
  "Attach Gen 3 cons/vector/string/float children to Gen 2 nodes in GRAPH."
  (let* ((old-cons (plist-get graph :cons))
         (gen1-nested-vec (nth 3 (cdr old-cons)))
         (gen2-vec (nth 3 (aref gen1-nested-vec 3)))
         (old-sym (plist-get graph :sym)))
    (aset gen2-vec 3
          (list :gen3-cons
                (copy-sequence "gen3-cons-str")
                (+ 30.0 0.5)
                (vector :gen3-leaf-vec (copy-sequence "gen3-leaf-str") (+ 31.0 0.875))))
    (put old-sym
         :gen3-prop
         (vector :gen3-sym-prop (copy-sequence "gen3-prop-str") (+ 32.0 0.375)))
    nil))

(defun femacs-runtime-gc-fallback--verify-graph (graph stage)
  "Verify multi-generation cons/vector/string/float reachability in GRAPH at STAGE."
  (let* ((old-cons (plist-get graph :cons))
         (old-vec (plist-get graph :vec))
         (old-sym (plist-get graph :sym))
         (old-table (plist-get graph :table))
         (sym-val (symbol-value old-sym))
         (sym-plist (symbol-plist old-sym))
         (g0-entry (gethash "gen0-key" old-table)))
    (should (equal (car old-cons) '(:gen0-car . "gen0-car-str")))
    (should (equal (aref old-vec 0) "gen0-vec-str"))
    (should (= (aref old-vec 1) 2.5))
    (should (equal (aref old-vec 2) '(:gen0-vec-cons 10 "gen0-vec-leaf")))
    (should (equal (aref sym-val 0) "gen0-sym-str"))
    (should (= (aref sym-val 1) 3.75))
    (should (equal (plist-get sym-plist :gen0-prop) "gen0-prop-str"))
    (should (= (plist-get sym-plist :gen0-float) 5.125))
    (should (equal g0-entry
                   '(:gen0-table-val 4.125 "gen0-table-str"
                                     [:gen0-table-vec "gen0-table-leaf"])))
    (when (>= stage 1)
      (let ((cdr1 (cdr old-cons))
            (vec-tail1 (aref old-vec 3))
            (g1-entry (gethash "gen1-key" old-table)))
        (should (eq (nth 0 cdr1) :gen1-cdr))
        (should (equal (nth 1 cdr1) "gen1-cons-str"))
        (should (= (nth 2 cdr1) 10.5))
        (should (eq (aref (nth 3 cdr1) 0) :gen1-nested-vec))
        (should (equal (aref (nth 3 cdr1) 1) "gen1-nested-str"))
        (should (= (aref (nth 3 cdr1) 2) 11.25))
        (should (eq (nth 0 vec-tail1) :gen1-vec-tail))
        (should (equal (nth 1 vec-tail1) "gen1-vec-tail-str"))
        (should (= (nth 2 vec-tail1) 12.75))
        (should (equal (aref sym-val 2) '("gen1-sym-tail" . 13.125)))
        (should (eq (aref g1-entry 0) :gen1-table-entry))
        (should (equal (aref g1-entry 1) "gen1-table-str"))
        (should (= (aref g1-entry 2) 14.625))
        (should (equal (aref g1-entry 3)
                       '(:gen1-table-cons 42 "gen1-table-leaf")))))
    (when (>= stage 2)
      (let* ((gen1-nested-vec (nth 3 (cdr old-cons)))
             (gen2-list (aref gen1-nested-vec 3))
             (gen2-vec (nth 3 gen2-list))
             (gen1-vec-child (nth 3 (aref old-vec 3)))
             (g1-entry (gethash "gen1-key" old-table)))
        (should (eq (nth 0 gen2-list) :gen2-cons))
        (should (equal (nth 1 gen2-list) "gen2-cons-str"))
        (should (= (nth 2 gen2-list) 20.5))
        (should (eq (aref gen2-vec 0) :gen2-vec))
        (should (equal (aref gen2-vec 1) "gen2-vec-str"))
        (should (= (aref gen2-vec 2) 21.25))
        (should (equal (aref gen1-vec-child 2) '("gen2-child-str" . 22.75)))
        (should (equal (aref g1-entry 4)
                       '(:gen2-table-tail "gen2-table-str" 23.125)))))
    (when (>= stage 3)
      (let* ((gen1-nested-vec (nth 3 (cdr old-cons)))
             (gen2-vec (nth 3 (aref gen1-nested-vec 3)))
             (gen3-list (aref gen2-vec 3)))
        (should (equal gen3-list
                       '(:gen3-cons "gen3-cons-str" 30.5
                                    [:gen3-leaf-vec "gen3-leaf-str" 31.875])))
        (should (equal (plist-get sym-plist :gen3-prop)
                       '[:gen3-sym-prop "gen3-prop-str" 32.375]))))
    `((stage . ,stage)
      (gen0_car . ,(symbol-name (car (car old-cons))))
      (gen0_car_str . ,(cdr (car old-cons)))
      (gen0_vec_str . ,(aref old-vec 0))
      (gen0_vec_float . ,(aref old-vec 1))
      (gen0_vec_leaf . ,(nth 2 (aref old-vec 2)))
      (gen0_sym_str . ,(aref sym-val 0))
      (gen0_sym_float . ,(aref sym-val 1))
      (gen0_prop_str . ,(plist-get sym-plist :gen0-prop))
      (gen0_prop_float . ,(plist-get sym-plist :gen0-float))
      (gen0_table_float . ,(nth 1 g0-entry))
      (gen0_table_str . ,(nth 2 g0-entry))
      (gen0_table_leaf . ,(aref (nth 3 g0-entry) 1))
      (gen1_cons_str . ,(when (>= stage 1) (nth 1 (cdr old-cons))))
      (gen1_cons_float . ,(when (>= stage 1) (nth 2 (cdr old-cons))))
      (gen1_nested_str . ,(when (>= stage 1) (aref (nth 3 (cdr old-cons)) 1)))
      (gen1_nested_float . ,(when (>= stage 1) (aref (nth 3 (cdr old-cons)) 2)))
      (gen1_vec_tail_str . ,(when (>= stage 1) (nth 1 (aref old-vec 3))))
      (gen1_vec_tail_float . ,(when (>= stage 1) (nth 2 (aref old-vec 3))))
      (gen1_sym_tail_str . ,(when (>= stage 1) (car (aref sym-val 2))))
      (gen1_sym_tail_float . ,(when (>= stage 1) (cdr (aref sym-val 2))))
      (gen1_table_str . ,(when (>= stage 1) (aref (gethash "gen1-key" old-table) 1)))
      (gen1_table_float . ,(when (>= stage 1) (aref (gethash "gen1-key" old-table) 2)))
      (gen1_table_leaf . ,(when (>= stage 1)
                            (nth 2 (aref (gethash "gen1-key" old-table) 3))))
      (gen2_cons_str . ,(when (>= stage 2)
                          (nth 1 (aref (nth 3 (cdr old-cons)) 3))))
      (gen2_cons_float . ,(when (>= stage 2)
                            (nth 2 (aref (nth 3 (cdr old-cons)) 3))))
      (gen2_vec_str . ,(when (>= stage 2)
                         (aref (nth 3 (aref (nth 3 (cdr old-cons)) 3)) 1)))
      (gen2_vec_float . ,(when (>= stage 2)
                           (aref (nth 3 (aref (nth 3 (cdr old-cons)) 3)) 2)))
      (gen2_child_str . ,(when (>= stage 2)
                           (car (aref (nth 3 (aref old-vec 3)) 2))))
      (gen2_child_float . ,(when (>= stage 2)
                             (cdr (aref (nth 3 (aref old-vec 3)) 2))))
      (gen2_table_str . ,(when (>= stage 2)
                           (nth 1 (aref (gethash "gen1-key" old-table) 4))))
      (gen2_table_float . ,(when (>= stage 2)
                             (nth 2 (aref (gethash "gen1-key" old-table) 4))))
      (gen3_cons_str . ,(when (>= stage 3)
                          (let* ((g1v (nth 3 (cdr old-cons)))
                                 (g2v (nth 3 (aref g1v 3))))
                            (nth 1 (aref g2v 3)))))
      (gen3_cons_float . ,(when (>= stage 3)
                            (let* ((g1v (nth 3 (cdr old-cons)))
                                   (g2v (nth 3 (aref g1v 3))))
                              (nth 2 (aref g2v 3)))))
      (gen3_leaf_str . ,(when (>= stage 3)
                          (let* ((g1v (nth 3 (cdr old-cons)))
                                 (g2v (nth 3 (aref g1v 3)))
                                 (g3v (nth 3 (aref g2v 3))))
                            (aref g3v 1))))
      (gen3_leaf_float . ,(when (>= stage 3)
                            (let* ((g1v (nth 3 (cdr old-cons)))
                                   (g2v (nth 3 (aref g1v 3)))
                                   (g3v (nth 3 (aref g2v 3))))
                              (aref g3v 2))))
      (gen3_prop_str . ,(when (>= stage 3)
                          (aref (plist-get sym-plist :gen3-prop) 1)))
      (gen3_prop_float . ,(when (>= stage 3)
                            (aref (plist-get sym-plist :gen3-prop) 2)))
      (hash_table_count . ,(hash-table-count old-table)))))

(ert-deftest rtg-fallback-01-unknown-producer-permanent-full-and-registry ()
  "Verify failed `module-load' latches permanent Full fallback without breaking multi-generation reachability."
  (should (fboundp 'module-load))
  (let ((temp-dir (femacs-runtime-gc-fallback--owned-temp-dir))
        (graph nil))
    (unwind-protect
        (let ((missing-module
               (expand-file-name femacs-runtime-gc-fallback--missing-basename temp-dir)))
          (should-not (file-exists-p missing-module))
          (let ((gc-cons-threshold most-positive-fixnum))
            (setq graph (femacs-runtime-gc-fallback--build-gen0)
                  femacs-runtime-gc--roots graph))
          ;; Step 1: Healthy explicit Full GC before `module-load'.
          (let* ((initial (femacs-runtime-gc--status))
                 (mode (femacs-runtime-gc--total initial femacs-runtime-gc--t-mode))
                 (initial-reg (femacs-runtime-gc--total
                               initial femacs-runtime-gc--t-registry-len))
                 (initial-majors (femacs-runtime-gc--total initial femacs-runtime-gc--t-majors))
                 (initial-minors (femacs-runtime-gc--total initial femacs-runtime-gc--t-minors))
                 (initial-explicit (femacs-runtime-gc--total
                                    initial femacs-runtime-gc--t-explicit-fulls)))
            (should (= (femacs-runtime-gc--total
                        initial femacs-runtime-gc--t-unknown-producer)
                       0))
            (garbage-collect)
            (femacs-runtime-gc-fallback--verify-graph graph 0)
            (let* ((pre-latch (femacs-runtime-gc--status))
                   (pre-latch-reg (femacs-runtime-gc--total
                                   pre-latch femacs-runtime-gc--t-registry-len))
                   (pre-latch-majors (femacs-runtime-gc--total
                                      pre-latch femacs-runtime-gc--t-majors))
                   (pre-latch-minors (femacs-runtime-gc--total
                                      pre-latch femacs-runtime-gc--t-minors))
                   (pre-latch-fallbacks (femacs-runtime-gc--total
                                         pre-latch femacs-runtime-gc--t-full-fallbacks))
                   (pre-latch-explicit (femacs-runtime-gc--total
                                        pre-latch femacs-runtime-gc--t-explicit-fulls))
                   (pre-latch-reason2 (femacs-runtime-gc--reason pre-latch 2))
                   (pre-latch-reason3 (femacs-runtime-gc--reason pre-latch 3))
                   (pre-latch-reason8 (femacs-runtime-gc--reason pre-latch 8))
                   module-err
                   latched
                   latched-reg
                   explicit-after
                   explicit-reg
                   auto1
                   auto1-reg
                   auto2
                   auto2-reg
                   public-graph)
              (should (> pre-latch-majors initial-majors))
              (should (= pre-latch-minors initial-minors))
              (should (> pre-latch-explicit initial-explicit))
              (should (= (femacs-runtime-gc--total pre-latch femacs-runtime-gc--t-kind) 0))
              (should (= (femacs-runtime-gc--total pre-latch femacs-runtime-gc--t-last-reason) 2))
              (should (= (femacs-runtime-gc--total
                          pre-latch femacs-runtime-gc--t-unknown-producer)
                         0))
              (should (= (femacs-runtime-gc--total
                          pre-latch femacs-runtime-gc--t-registry-overflows)
                         0))
              (if (= mode 1)
                  (progn
                    (should (= (femacs-runtime-gc--total pre-latch femacs-runtime-gc--t-valid) 1))
                    (should (> pre-latch-reg 0)))
                (should (= (femacs-runtime-gc--total pre-latch femacs-runtime-gc--t-valid) 0))
                (should (= pre-latch-reg 0)))

              ;; Step 2: Attach Gen 1 objects and trigger failed `module-load' without intermediate GC.
              (let ((gc-cons-threshold most-positive-fixnum))
                (femacs-runtime-gc-fallback--attach-gen1 graph)
                (setq module-err
                      (should-error (module-load missing-module)
                                    :type 'module-open-failed))
                (setq latched (femacs-runtime-gc--status)))
              (should (eq (car module-err) 'module-open-failed))
              (should (equal (nth 1 module-err) missing-module))
              (should (stringp (nth 2 module-err)))
              (should (> (length (nth 2 module-err)) 0))
              (setq latched-reg
                    (femacs-runtime-gc--total latched femacs-runtime-gc--t-registry-len))
              (should (= (femacs-runtime-gc--total
                          latched femacs-runtime-gc--t-unknown-producer)
                         1))
              (should (= (femacs-runtime-gc--total latched femacs-runtime-gc--t-valid) 0))
              (should (= (femacs-runtime-gc--total latched femacs-runtime-gc--t-majors)
                         pre-latch-majors))
              (should (= (femacs-runtime-gc--total latched femacs-runtime-gc--t-minors)
                         pre-latch-minors))
              ;; Latch time itself does not clear the previous cycle's registry.
              (should (= latched-reg pre-latch-reg))

              ;; Step 3: Post-latch explicit Full GC (`reason' 2 precedes `mode' 3 and `unknown' 8).
              (garbage-collect)
              (setq explicit-after (femacs-runtime-gc--status)
                    explicit-reg (femacs-runtime-gc--total
                                  explicit-after femacs-runtime-gc--t-registry-len))
              (femacs-runtime-gc-fallback--verify-graph graph 1)
              (should (= (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-unknown-producer)
                         1))
              (should (= (femacs-runtime-gc--total explicit-after femacs-runtime-gc--t-valid) 0))
              (should (= (femacs-runtime-gc--total explicit-after femacs-runtime-gc--t-kind) 0))
              (should (= (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-last-reason)
                         2))
              (should (> (femacs-runtime-gc--reason explicit-after 2) pre-latch-reason2))
              (should (> (femacs-runtime-gc--total explicit-after femacs-runtime-gc--t-majors)
                         pre-latch-majors))
              (should (= (femacs-runtime-gc--total explicit-after femacs-runtime-gc--t-minors)
                         pre-latch-minors))
              (should (> (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-explicit-fulls)
                         pre-latch-explicit))
              (should (> (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-full-fallbacks)
                         pre-latch-fallbacks))
              (should (= (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-consecutive)
                         0))
              (when (= mode 0)
                (should (= explicit-reg 0)))

              ;; Step 4: First post-latch automatic collection must be Full (reason 8 in young mode, 3 in major).
              (let ((gc-cons-threshold most-positive-fixnum))
                (femacs-runtime-gc-fallback--attach-gen2 graph))
              (setq auto1
                    (femacs-runtime-gc-fallback--until-auto-full
                     (femacs-runtime-gc--total explicit-after femacs-runtime-gc--t-majors))
                    auto1-reg
                    (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-registry-len))
              (femacs-runtime-gc-fallback--verify-graph graph 2)
              (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-unknown-producer) 1))
              (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-valid) 0))
              (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-kind) 0))
              (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-minors)
                         pre-latch-minors))
              (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-consecutive) 0))
              (should (> (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-full-fallbacks)
                         (femacs-runtime-gc--total
                          explicit-after femacs-runtime-gc--t-full-fallbacks)))
              (if (= mode 1)
                  (progn
                    (should (= (femacs-runtime-gc--total
                                auto1 femacs-runtime-gc--t-last-reason)
                               8))
                    (should (> (femacs-runtime-gc--reason auto1 8) pre-latch-reason8)))
                (should (= (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-last-reason) 3))
                (should (> (femacs-runtime-gc--reason auto1 3) pre-latch-reason3))
                (should (= auto1-reg 0)))

              ;; Step 5: Second post-latch automatic collection confirms permanence across multiple cycles.
              (let ((gc-cons-threshold most-positive-fixnum))
                (femacs-runtime-gc-fallback--attach-gen3 graph))
              (setq auto2
                    (femacs-runtime-gc-fallback--until-auto-full
                     (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-majors))
                    auto2-reg
                    (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-registry-len)
                    public-graph
                    (femacs-runtime-gc-fallback--verify-graph graph 3))
              (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-unknown-producer) 1))
              (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-valid) 0))
              (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-kind) 0))
              (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-minors)
                         pre-latch-minors))
              (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-consecutive) 0))
              (should (> (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-full-fallbacks)
                         (femacs-runtime-gc--total auto1 femacs-runtime-gc--t-full-fallbacks)))
              (if (= mode 1)
                  (progn
                    (should (= (femacs-runtime-gc--total
                                auto2 femacs-runtime-gc--t-last-reason)
                               8))
                    (should (> (femacs-runtime-gc--reason auto2 8)
                               (femacs-runtime-gc--reason auto1 8))))
                (should (= (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-last-reason) 3))
                (should (> (femacs-runtime-gc--reason auto2 3)
                           (femacs-runtime-gc--reason auto1 3)))
                (should (= auto2-reg 0)))

              (femacs-runtime-gc-fallback--write-report
               `((schema . ,femacs-runtime-gc-fallback--schema)
                 (id . ,femacs-runtime-gc-fallback--case-id)
                 (mode . ,mode)
                 (module_error
                  . ((symbol . ,(symbol-name (car module-err)))
                     (basename . ,femacs-runtime-gc-fallback--missing-basename)
                     (path_matched . t)
                     (message_nonempty . t)))
                 (snapshots
                  . ((initial . ,initial)
                     (pre_latch_full . ,pre-latch)
                     (latched_immediate . ,latched)
                     (post_explicit_full . ,explicit-after)
                     (post_auto_full_1 . ,auto1)
                     (post_auto_full_2 . ,auto2)))
                 (registry_len
                  . ((initial . ,initial-reg)
                     (pre_latch_full . ,pre-latch-reg)
                     (latched_immediate . ,latched-reg)
                     (post_explicit_full . ,explicit-reg)
                     (post_auto_full_1 . ,auto1-reg)
                     (post_auto_full_2 . ,auto2-reg)))
                 (deltas_after_latch
                  . ((minors . ,(- (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-minors)
                                   pre-latch-minors))
                     (majors . ,(- (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-majors)
                                   pre-latch-majors))
                     (explicit_fulls
                      . ,(- (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-explicit-fulls)
                            pre-latch-explicit))
                     (full_fallbacks
                      . ,(- (femacs-runtime-gc--total auto2 femacs-runtime-gc--t-full-fallbacks)
                            pre-latch-fallbacks))
                     (reason_2_explicit
                      . ,(- (femacs-runtime-gc--reason auto2 2) pre-latch-reason2))
                     (reason_3_major_mode
                      . ,(- (femacs-runtime-gc--reason auto2 3) pre-latch-reason3))
                     (reason_8_unknown_producer
                      . ,(- (femacs-runtime-gc--reason auto2 8) pre-latch-reason8))))
                 (public_graph . ,public-graph))))))
      (setq femacs-runtime-gc--roots nil)
      (when (and temp-dir (file-directory-p temp-dir))
        (delete-directory temp-dir t)))))

(provide 'runtime-gc-fallback)
;;; runtime-gc-fallback.el ends here
