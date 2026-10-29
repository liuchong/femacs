;;; process-thread-wait-gc-protection.el --- suspended waiter GC -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)

(ert-deftest fe-process-thread-wait/gc-during-suspended-wait-and-filter ()
  "Collect while another thread waits, then collect in its output filter."
  (skip-unless (fboundp 'make-thread))
  (let* ((mutex (make-mutex "fe-wait-gc"))
         (condition (make-condition-variable mutex "fe-wait-gc-ready"))
         (ready nil)
         (chunks nil)
         (collections 0)
         process worker timer)
    (unwind-protect
        (progn
          (setq process
                (make-process
                 :name "fe-wait-gc" :command '("cat")
                 :connection-type 'pipe :coding 'no-conversion :noquery t
                 :filter (lambda (_process text)
                           (garbage-collect)
                           (setq collections (1+ collections))
                           (push text chunks))))
          (set-process-thread process nil)
          (setq worker
                (make-thread
                 (lambda ()
                   (with-mutex mutex
                     (setq ready t)
                     (condition-notify condition t))
                   (while (accept-process-output process)))
                 "fe-wait-gc-worker"))
          (with-mutex mutex
            (while (not ready)
              (condition-wait condition)))
          (setq timer
                (run-at-time
                 0 nil
                 (lambda ()
                   (dotimes (_ 3)
                     (garbage-collect)
                     (setq collections (1+ collections)))
                   (process-send-string process "after-gc\n")
                   (process-send-eof process))))
          (let ((deadline (+ (float-time) 2)))
            (while (and (thread-live-p worker) (< (float-time) deadline))
              (accept-process-output nil 0.05)))
          (should-not (thread-live-p worker))
          (thread-join worker)
          (setq worker nil)
          (should (>= collections 4))
          (should (equal (apply #'concat (nreverse chunks)) "after-gc\n"))
          (should (eq (process-status process) 'exit)))
      (when (timerp timer) (cancel-timer timer))
      (when (process-live-p process) (delete-process process))
      (when worker (thread-join worker)))))

(provide 'process-thread-wait-gc-protection)
;;; process-thread-wait-gc-protection.el ends here
