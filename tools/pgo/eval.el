;;; pgo/eval.el --- fixed evaluation corpus for the O2/PGO comparison -*- lexical-binding: t -*-
;; This file is measurement input, not a ported Lisp file.  Its data, sizes and
;; operation sequence are disjoint from tools/pgo/train.el.  The six BENCH cases
;; keep tools/bench.sh's metric: min of five inside the program.

(setq gc-cons-threshold most-positive-fixnum)

(defun pgo-eval-bc-loop (n) (let ((i 0)) (while (< i n) (setq i (1+ i)))))
(defun pgo-eval-bc-sum (n) (let ((i 0) (s 0)) (while (< i n) (setq s (+ s i)) (setq i (1+ i))) s))
(defun pgo-eval-bc-car (n) (let ((x '(1 2 3))) (dotimes (_ n) (car x))))
(mapc #'byte-compile '(pgo-eval-bc-loop pgo-eval-bc-sum pgo-eval-bc-car))

(defmacro pgo-eval-measure (kind label &rest body)
  `(let ((best nil))
     (dotimes (_ 5)
       (let ((t0 (float-time)))
         ,@body
         (let ((dt (- (float-time) t0)))
           (setq best (if best (min best dt) dt)))))
     (message "%s %-12s %8.4f ms" ,kind (quote ,label) (* 1000 best))))

(defun pgo-eval-text ()
  (mapconcat (lambda (i) (format "row %d: delta epsilon zeta\n" i))
             (number-sequence 1 9000) ""))

;; --- six microbenchmarks, same names and metric as tools/bench.sh -----------------
(let ((text (mapconcat (lambda (i) (format "line %d: alpha beta gamma\n" i))
                       (number-sequence 1 8000) "")))
  (pgo-eval-measure "BENCH" bc-loop (pgo-eval-bc-loop 1000000))
  (pgo-eval-measure "BENCH" bc-sum (pgo-eval-bc-sum 1000000))
  (pgo-eval-measure "BENCH" bc-car (pgo-eval-bc-car 1000000))
  (pgo-eval-measure "BENCH" regexp-repl (replace-regexp-in-string "alpha \\(beta\\)" "X-\\1" text))
  (pgo-eval-measure "BENCH" buffer-scan
    (with-temp-buffer
      (insert text) (goto-char (point-min))
      (while (re-search-forward "^line [0-9]+:" nil t))))
  (pgo-eval-measure "BENCH" sort-100k (sort (number-sequence 1 100000) #'<)))

;; --- real workloads ---------------------------------------------------------------
(pgo-eval-measure "TASK" load
  (progn
    (load "emacs-lisp/byte-opt" nil t)
    (load "emacs-lisp/bytecomp" nil t)
    (load "emacs-lisp/macroexp" nil t)
    (load "emacs-lisp/cconv" nil t)
    (load "emacs-lisp/cl-extra" nil t)))

(pgo-eval-measure "TASK" edit
  (with-temp-buffer
    (insert (pgo-eval-text))
    (goto-char (point-min))
    (while (search-forward "epsilon" nil t) (replace-match "EPSILON" t t))
    (goto-char (point-min))
    (while (forward-word 1))
    (upcase-region (point-min) (min (point-max) 50000))
    (delete-region (point-min) (min (point-max) 10000))))

(pgo-eval-measure "TASK" regex
  (let ((text (mapconcat (lambda (i) (format "omega %d: tau\n" i))
                         (number-sequence 1 6000) "")))
    (replace-regexp-in-string "omega \\([0-9]+\\)" "O-\\1" text)))

(pgo-eval-measure "TASK" gc
  (let ((gc-cons-threshold 100000)
        (acc nil)
        (i 0))
    (while (< i 300000)
      (push (cons i (format "g%d" i)) acc)
      (setq i (1+ i)))
    (garbage-collect)
    (length acc)))

(message "PGO-EVAL-DONE")
