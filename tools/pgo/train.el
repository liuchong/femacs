;;; pgo/train.el --- fixed profile-training corpus -*- lexical-binding: t -*-
;; Run only against the profile-generate build, with LLVM_PROFILE_FILE pointing at
;; <phase>/profiles/train/.  Data, sizes and operation sequence differ from
;; tools/pgo/eval.el on purpose; the code paths overlap, the inputs do not.

(setq gc-cons-threshold most-positive-fixnum)

(defun pgo-train-loop (n) (let ((i 0)) (while (< i n) (setq i (1+ i)))))
(defun pgo-train-sum (n) (let ((i 0) (s 0)) (while (< i n) (setq s (+ s i)) (setq i (1+ i))) s))
(defun pgo-train-car (n) (let ((x '(4 5 6))) (dotimes (_ n) (car x))))
(mapc #'byte-compile '(pgo-train-loop pgo-train-sum pgo-train-car))

(defun pgo-train-lines (n word)
  (mapconcat (lambda (i) (format "entry %d: %s\n" i word))
             (number-sequence 1 n) ""))

;; bytecode VM through the same three shapes
(pgo-train-loop 700000)
(pgo-train-sum 700000)
(pgo-train-car 700000)

;; buffer insertion, scanning, replacement and motion
(let ((text (pgo-train-lines 12000 "charlie")))
  (with-temp-buffer
    (insert text)
    (goto-char (point-min))
    (while (search-forward "entry" nil t) (replace-match "item" t t))
    (goto-char (point-min))
    (while (forward-word 1))
    (upcase-region (point-min) (min (point-max) 30000))))

;; regexp engine on a different corpus than eval.el
(let ((text (mapconcat (lambda (i) (format "bravo %d: sigma\n" i))
                       (number-sequence 1 4500) "")))
  (replace-regexp-in-string "bravo \\([0-9]+\\)" "B-\\1" text))

;; loading/compilation paths
(load "emacs-lisp/byte-opt" nil t)
(load "emacs-lisp/bytecomp" nil t)
(load "emacs-lisp/macroexp" nil t)
(load "emacs-lisp/cconv" nil t)

;; GC pressure with different allocation sizes from eval.el
(let ((gc-cons-threshold 120000)
      (acc nil)
      (i 0))
  (while (< i 180000)
    (push (vector i (number-to-string i) (make-string 8 (+ ?a (% i 26)))) acc)
    (setq i (1+ i)))
  (garbage-collect)
  (length acc))

(sort (number-sequence 1 60000) #'<)
(message "PGO-TRAIN-DONE")
