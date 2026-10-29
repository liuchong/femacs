;;; runtime-gc-owner-batch-protection.el --- Remembered-owner batch marking protections -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

;; Post-integration ERT protections for P436 (`trace_remembered_owners`
;; batching via `mark_objects` over the frozen registry prefix).
;; Reuses `runtime-gc.el` helpers and status wire schema across both
;; `major` (mode 0) and `young` (mode 1) collector profiles without
;; private cycle-forcing hooks or unbounded recursion.

(require 'ert)
(require 'cl-lib)

(dolist (fn '(femacs-runtime-gc--status
              femacs-runtime-gc--total
              femacs-runtime-gc--churn
              femacs-runtime-gc--until-minor))
  (unless (fboundp fn)
    (error "runtime-gc-owner-batch-protection requires helper `%s' from runtime-gc.el" fn)))

(dolist (sym '(femacs-runtime-gc--t-mode
               femacs-runtime-gc--t-valid
               femacs-runtime-gc--t-kind
               femacs-runtime-gc--t-majors
               femacs-runtime-gc--t-minors
               femacs-runtime-gc--t-registry-len
               femacs-runtime-gc--t-registry-overflows
               femacs-runtime-gc--roots
               femacs-runtime-gc--buffer-local))
  (unless (boundp sym)
    (error "runtime-gc-owner-batch-protection requires symbol `%s' from runtime-gc.el" sym)))

(defconst femacs-runtime-gc-owner-batch--chain-depth 9000)
(defconst femacs-runtime-gc-owner-batch--owner-count 32)

(defun femacs-runtime-gc-owner-batch--install-deep-chain
    (owner-pre owner-chain owner-mid owner-post depth)
  "Attach young payloads and an iteratively built DEPTH-level branching vector chain.
Each chain node is [NEXT-NODE BRANCH-LEAF STEP] so left-to-right array marking
visits slot 0 (NEXT-NODE) while retaining the current node's continuation entry
on the mark stack alongside the outer remembered-owner batch entry."
  (aset owner-pre 0 (vector :pre-young (list :pre-cons 11 22)))
  (aset owner-mid 0 (vector :mid-young (list :mid-cons 33 44)))
  (aset owner-post 0 (vector :post-young (list :post-cons 55 66)))
  (let ((cursor (vector :chain-terminal depth))
        (step depth))
    (while (> step 0)
      (setq step (1- step))
      (let ((branch-leaf (vector :branch-leaf step (- depth step))))
        (setq cursor (vector cursor branch-leaf step))))
    (aset owner-chain 0 cursor))
  nil)

(defun femacs-runtime-gc-owner-batch--verify-deep-chain
    (owner-pre owner-chain owner-mid owner-post depth)
  "Iteratively verify sibling owners and every level of the DEPTH-level chain."
  (should (equal (aref owner-pre 1) :owner-pre))
  (should (equal (aref owner-chain 1) :owner-chain))
  (should (equal (aref owner-mid 1) :owner-mid))
  (should (equal (aref owner-post 1) :owner-post))
  (should (equal (aref owner-pre 0) (vector :pre-young '(:pre-cons 11 22))))
  (should (equal (aref owner-mid 0) (vector :mid-young '(:mid-cons 33 44))))
  (should (equal (aref owner-post 0) (vector :post-young '(:post-cons 55 66))))
  (let ((cursor (aref owner-chain 0))
        (step 0))
    (while (< step depth)
      (should (vectorp cursor))
      (should (= (length cursor) 3))
      (should (= (aref cursor 2) step))
      (let ((branch-leaf (aref cursor 1)))
        (should (vectorp branch-leaf))
        (should (= (length branch-leaf) 3))
        (should (eq (aref branch-leaf 0) :branch-leaf))
        (should (= (aref branch-leaf 1) step))
        (should (= (aref branch-leaf 2) (- depth step))))
      (setq cursor (aref cursor 0)
            step (1+ step)))
    (should (vectorp cursor))
    (should (= (length cursor) 2))
    (should (eq (aref cursor 0) :chain-terminal))
    (should (= (aref cursor 1) depth))))

(defun femacs-runtime-gc-owner-batch--install-wave1
    (old-vectors old-table old-symbol old-buffer)
  "Attach first-wave young non-cons owners and cons subgraphs to old owners."
  (let ((count (length old-vectors)))
    (dotimes (i count)
      (let ((owner (aref old-vectors i))
            (wave1-vec (vector :wave1-vec i nil))
            (wave1-cons (list :wave1-cons i (- count i))))
        (aset owner 0 (cons wave1-vec wave1-cons)))))
  (puthash :wave1-key
           (vector :wave1-hash-owner (list :wave1-hash-cons 77) nil)
           old-table)
  (set old-symbol (vector :wave1-sym-owner (list :wave1-sym-cons 88) nil))
  (with-current-buffer old-buffer
    (setq femacs-runtime-gc--buffer-local
          (vector :wave1-buf-owner (list :wave1-buf-cons 99) nil)))
  nil)

(defun femacs-runtime-gc-owner-batch--install-wave2
    (old-vectors old-table old-symbol old-buffer)
  "Attach second-wave young subgraphs solely to first-wave suffix non-cons owners."
  (let ((count (length old-vectors)))
    (dotimes (i count)
      (let* ((pair (aref (aref old-vectors i) 0))
             (wave1-vec (car pair)))
        (aset wave1-vec 2 (vector :wave2-vec i (list :wave2-cons i (+ 1000 i)))))))
  (let ((hash-vec (gethash :wave1-key old-table))
        (sym-vec (symbol-value old-symbol))
        (buf-vec (with-current-buffer old-buffer
                   femacs-runtime-gc--buffer-local)))
    (aset hash-vec 2 (vector :wave2-hash (list :wave2-hash-cons 177)))
    (aset sym-vec 2 (vector :wave2-sym (list :wave2-sym-cons 188)))
    (aset buf-vec 2 (vector :wave2-buf (list :wave2-buf-cons 199))))
  nil)

(defun femacs-runtime-gc-owner-batch--verify-waves
    (old-vectors old-table old-symbol old-buffer wave2-installed)
  "Verify first-wave and optional second-wave subgraphs across all owners."
  (let ((count (length old-vectors)))
    (dotimes (i count)
      (let* ((owner (aref old-vectors i))
             (pair (aref owner 0))
             (wave1-vec (car pair))
             (wave1-cons (cdr pair)))
        (should (equal (aref owner 1) (cons :old-owner i)))
        (should (consp pair))
        (should (vectorp wave1-vec))
        (should (= (length wave1-vec) 3))
        (should (eq (aref wave1-vec 0) :wave1-vec))
        (should (= (aref wave1-vec 1) i))
        (should (equal wave1-cons (list :wave1-cons i (- count i))))
        (if wave2-installed
            (should (equal (aref wave1-vec 2)
                           (vector :wave2-vec i (list :wave2-cons i (+ 1000 i)))))
          (should (null (aref wave1-vec 2)))))))
  (let ((hash-vec (gethash :wave1-key old-table))
        (sym-vec (symbol-value old-symbol))
        (buf-vec (with-current-buffer old-buffer
                   femacs-runtime-gc--buffer-local)))
    (should (vectorp hash-vec))
    (should (eq (aref hash-vec 0) :wave1-hash-owner))
    (should (equal (aref hash-vec 1) '(:wave1-hash-cons 77)))
    (should (vectorp sym-vec))
    (should (eq (aref sym-vec 0) :wave1-sym-owner))
    (should (equal (aref sym-vec 1) '(:wave1-sym-cons 88)))
    (should (vectorp buf-vec))
    (should (eq (aref buf-vec 0) :wave1-buf-owner))
    (should (equal (aref buf-vec 1) '(:wave1-buf-cons 99)))
    (if wave2-installed
        (progn
          (should (equal (aref hash-vec 2)
                         (vector :wave2-hash '(:wave2-hash-cons 177))))
          (should (equal (aref sym-vec 2)
                         (vector :wave2-sym '(:wave2-sym-cons 188))))
          (should (equal (aref buf-vec 2)
                         (vector :wave2-buf '(:wave2-buf-cons 199)))))
      (should (null (aref hash-vec 2)))
      (should (null (aref sym-vec 2)))
      (should (null (aref buf-vec 2))))))

(ert-deftest rtg-owner-batch-01-deep-branching-vector-chain ()
  "Preserve a 9000-level branching vector chain and sibling old owners across Young and Full GC."
  (let ((depth femacs-runtime-gc-owner-batch--chain-depth)
        (owner-pre (vector nil :owner-pre))
        (owner-chain (vector nil :owner-chain))
        (owner-mid (vector nil :owner-mid))
        (owner-post (vector nil :owner-post)))
    (unwind-protect
        (progn
          (setq femacs-runtime-gc--roots
                (vector owner-pre owner-chain owner-mid owner-post))
          (let ((gc-cons-threshold most-positive-fixnum))
            (garbage-collect)
            (femacs-runtime-gc-owner-batch--install-deep-chain
             owner-pre owner-chain owner-mid owner-post depth))
          (let* ((before (femacs-runtime-gc--status))
                 (mode (femacs-runtime-gc--total before femacs-runtime-gc--t-mode))
                 (minors (femacs-runtime-gc--total before femacs-runtime-gc--t-minors))
                 (majors (femacs-runtime-gc--total before femacs-runtime-gc--t-majors))
                 (reg-before (femacs-runtime-gc--total
                              before femacs-runtime-gc--t-registry-len)))
            (if (= mode 1)
                (let ((after (femacs-runtime-gc--until-minor minors)))
                  (should (> (femacs-runtime-gc--total
                              after femacs-runtime-gc--t-minors)
                             minors))
                  (should (= (femacs-runtime-gc--total
                              after femacs-runtime-gc--t-kind)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after femacs-runtime-gc--t-valid)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after femacs-runtime-gc--t-registry-overflows)
                             0))
                  (should (>= (femacs-runtime-gc--total
                               after femacs-runtime-gc--t-registry-len)
                              (+ reg-before (* 2 depth)))))
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
                           0)))))
          (femacs-runtime-gc-owner-batch--verify-deep-chain
           owner-pre owner-chain owner-mid owner-post depth)
          (let ((majors-before-full
                 (femacs-runtime-gc--total
                  (femacs-runtime-gc--status) femacs-runtime-gc--t-majors)))
            (garbage-collect)
            (let ((after-full (femacs-runtime-gc--status)))
              (should (> (femacs-runtime-gc--total
                          after-full femacs-runtime-gc--t-majors)
                         majors-before-full))
              (should (= (femacs-runtime-gc--total
                          after-full femacs-runtime-gc--t-kind)
                         0))))
          (femacs-runtime-gc-owner-batch--verify-deep-chain
           owner-pre owner-chain owner-mid owner-post depth))
      (setq femacs-runtime-gc--roots nil))))

(ert-deftest rtg-owner-batch-02-suffix-owners-across-young-and-full ()
  "Preserve multi-owner subgraphs when Young appends new non-cons owners to the registry suffix."
  (let* ((count femacs-runtime-gc-owner-batch--owner-count)
         (old-vectors (make-vector count nil))
         (old-table (make-hash-table :test 'eq))
         (old-symbol (make-symbol "femacs-rtg-batch-owner-symbol"))
         (old-buffer (generate-new-buffer " *femacs-rtg-batch-owner*")))
    (unwind-protect
        (progn
          (dotimes (i count)
            (aset old-vectors i (vector nil (cons :old-owner i))))
          (with-current-buffer old-buffer
            (make-local-variable 'femacs-runtime-gc--buffer-local)
            (setq femacs-runtime-gc--buffer-local nil))
          (set old-symbol nil)
          (setq femacs-runtime-gc--roots
                (list old-vectors old-table old-symbol))
          (let ((gc-cons-threshold most-positive-fixnum))
            (garbage-collect)
            (femacs-runtime-gc-owner-batch--install-wave1
             old-vectors old-table old-symbol old-buffer))
          (let* ((before1 (femacs-runtime-gc--status))
                 (mode (femacs-runtime-gc--total before1 femacs-runtime-gc--t-mode))
                 (minors1 (femacs-runtime-gc--total before1 femacs-runtime-gc--t-minors))
                 (majors1 (femacs-runtime-gc--total before1 femacs-runtime-gc--t-majors))
                 (reg1 (femacs-runtime-gc--total
                        before1 femacs-runtime-gc--t-registry-len)))
            (if (= mode 1)
                (let ((after1 (femacs-runtime-gc--until-minor minors1)))
                  (should (> (femacs-runtime-gc--total
                              after1 femacs-runtime-gc--t-minors)
                             minors1))
                  (should (= (femacs-runtime-gc--total
                              after1 femacs-runtime-gc--t-kind)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after1 femacs-runtime-gc--t-valid)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after1 femacs-runtime-gc--t-registry-overflows)
                             0))
                  (should (>= (femacs-runtime-gc--total
                               after1 femacs-runtime-gc--t-registry-len)
                              (+ reg1 count 3))))
              (femacs-runtime-gc--churn 8000)
              (let ((after1 (femacs-runtime-gc--status)))
                (should (= (femacs-runtime-gc--total
                            after1 femacs-runtime-gc--t-minors)
                           minors1))
                (should (> (femacs-runtime-gc--total
                            after1 femacs-runtime-gc--t-majors)
                           majors1)))))
          (femacs-runtime-gc-owner-batch--verify-waves
           old-vectors old-table old-symbol old-buffer nil)
          (let ((gc-cons-threshold most-positive-fixnum))
            (femacs-runtime-gc-owner-batch--install-wave2
             old-vectors old-table old-symbol old-buffer))
          (let* ((before2 (femacs-runtime-gc--status))
                 (mode (femacs-runtime-gc--total before2 femacs-runtime-gc--t-mode))
                 (minors2 (femacs-runtime-gc--total before2 femacs-runtime-gc--t-minors))
                 (majors2 (femacs-runtime-gc--total before2 femacs-runtime-gc--t-majors))
                 (reg2 (femacs-runtime-gc--total
                        before2 femacs-runtime-gc--t-registry-len)))
            (if (= mode 1)
                (let ((after2 (femacs-runtime-gc--until-minor minors2)))
                  (should (> (femacs-runtime-gc--total
                              after2 femacs-runtime-gc--t-minors)
                             minors2))
                  (should (= (femacs-runtime-gc--total
                              after2 femacs-runtime-gc--t-kind)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after2 femacs-runtime-gc--t-valid)
                             1))
                  (should (= (femacs-runtime-gc--total
                              after2 femacs-runtime-gc--t-registry-overflows)
                             0))
                  (should (>= (femacs-runtime-gc--total
                               after2 femacs-runtime-gc--t-registry-len)
                              (+ reg2 count 3))))
              (femacs-runtime-gc--churn 8000)
              (let ((after2 (femacs-runtime-gc--status)))
                (should (= (femacs-runtime-gc--total
                            after2 femacs-runtime-gc--t-minors)
                           minors2))
                (should (> (femacs-runtime-gc--total
                            after2 femacs-runtime-gc--t-majors)
                           majors2)))))
          (femacs-runtime-gc-owner-batch--verify-waves
           old-vectors old-table old-symbol old-buffer t)
          (garbage-collect)
          (femacs-runtime-gc-owner-batch--verify-waves
           old-vectors old-table old-symbol old-buffer t))
      (setq femacs-runtime-gc--roots nil)
      (when (buffer-live-p old-buffer)
        (kill-buffer old-buffer)))))

(provide 'runtime-gc-owner-batch-protection)
;;; runtime-gc-owner-batch-protection.el ends here
