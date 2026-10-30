;;; runtime-gc-fallback-protection.el --- Nested Full fallback protection -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)
(require 'cl-lib)

(defvar femacs-runtime-gc-fallback-protection--runs 0)
(defvar femacs-runtime-gc-fallback-protection--module-error nil)
(defvar femacs-runtime-gc-fallback-protection--callback-error nil)
(defvar femacs-runtime-gc-fallback-protection--nested-status nil)

(defun femacs-runtime-gc-fallback-protection--install (missing)
  "Install an unreachable finalizer that latches fallback before a nested GC."
  (make-finalizer
   (lambda ()
     (cl-incf femacs-runtime-gc-fallback-protection--runs)
     (condition-case error
         (progn
           (setq femacs-runtime-gc-fallback-protection--module-error
                 (should-error (module-load missing) :type 'module-open-failed))
           ;; These objects are created after the original collection's sweep.
           (setq femacs-runtime-gc--roots
                 (vector (cons (copy-sequence "finalizer-child") 73)
                         (+ 2.0 0.125)
                         (list (copy-sequence "nested-leaf") (vector 1 2 3))))
           (garbage-collect)
           (setq femacs-runtime-gc-fallback-protection--nested-status
                 (femacs-runtime-gc--status)))
       (error
        ;; Allocator finalizer error handling must not hide a fixture failure.
        (setq femacs-runtime-gc-fallback-protection--callback-error error)))))
  nil)

(ert-deftest rtg-fallback-protection-finalizer-latch-and-nested-full ()
  "Preserve finalizer-created objects and permanent fallback across nested GC."
  (should (fboundp 'module-load))
  (let* ((out (getenv "FEMACS_RUNTIME_OUT"))
         (missing (and out (expand-file-name "missing-finalizer-module.so" out)))
         (initial (femacs-runtime-gc--status)))
    (should missing)
    (should-not (file-exists-p missing))
    (should (= (femacs-runtime-gc--total initial femacs-runtime-gc--t-mode) 1))
    (should (= (femacs-runtime-gc--total initial femacs-runtime-gc--t-unknown-producer) 0))
    (setq femacs-runtime-gc-fallback-protection--runs 0
          femacs-runtime-gc-fallback-protection--module-error nil
          femacs-runtime-gc-fallback-protection--callback-error nil
          femacs-runtime-gc-fallback-protection--nested-status nil
          femacs-runtime-gc--roots nil)
    (unwind-protect
        (progn
          (femacs-runtime-gc-fallback-protection--install missing)
          (dotimes (_ 4)
            (when (= femacs-runtime-gc-fallback-protection--runs 0)
              (garbage-collect)))
          (should (= femacs-runtime-gc-fallback-protection--runs 1))
          (should-not femacs-runtime-gc-fallback-protection--callback-error)
          (should (eq (car femacs-runtime-gc-fallback-protection--module-error)
                      'module-open-failed))
          (should (equal (nth 1 femacs-runtime-gc-fallback-protection--module-error)
                         missing))
          (let ((status femacs-runtime-gc-fallback-protection--nested-status))
            (should status)
            ;; Pending-finalizer priority can precede the explicit-Full reason.
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-kind) 0))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-unknown-producer) 1))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-valid) 0))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-registry-len) 0)))
          (garbage-collect)
          (should (= femacs-runtime-gc-fallback-protection--runs 1))
          (should (equal femacs-runtime-gc--roots
                         '[("finalizer-child" . 73) 2.125 ("nested-leaf" [1 2 3])]))
          (let ((status (femacs-runtime-gc--status)))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-unknown-producer) 1))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-last-reason) 2))
            (should (= (femacs-runtime-gc--total status femacs-runtime-gc--t-registry-len) 0))))
      (setq femacs-runtime-gc--roots nil))))

(provide 'runtime-gc-fallback-protection)
;;; runtime-gc-fallback-protection.el ends here
