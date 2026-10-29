;;; process-thread-wait-protection.el --- multi-thread process wait regression -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)

(ert-deftest fe-process-thread-wait/unlocked-eof-round-isolation ()
  "Wait on unlocked pipe subprocesses across worker threads without EBADF or hang."
  (skip-unless (fboundp 'make-thread))
  (dotimes (round 12)
    (let (processes threads sentinel-events)
      (dotimes (i 10)
        (let* ((tag (format "fe-wait-%d-%d" round i))
               (process
                (make-process
                 :name tag
                 :command '("cat")
                 :connection-type 'pipe
                 :noquery t
                 :sentinel (lambda (proc event)
                             (push (cons (process-name proc) event)
                                   sentinel-events)))))
          (set-process-thread process nil)
          (should (null (process-thread process)))
          (push process processes)
          (push (make-thread
                 (lambda ()
                   (while (accept-process-output process)))
                 tag)
                threads)))
      (thread-yield)
      (mapc #'process-send-eof processes)
      (dolist (thread threads)
        (thread-join thread))
      (dolist (process processes)
        (should (eq (process-status process) 'exit))
        (should (= (process-exit-status process) 0)))
      (should (= (length sentinel-events) 10)))))

(ert-deftest fe-process-thread-wait/payload-and-staggered-eof ()
  "Preserve per-process output and clean EOF termination across concurrent threads."
  (skip-unless (fboundp 'make-thread))
  (let (processes threads outputs)
    (dotimes (i 10)
      (let* ((idx i)
             (tag (format "fe-payload-%d" idx))
             (chunks nil)
             (process
              (make-process
               :name tag
               :command '("cat")
               :connection-type 'pipe
               :noquery t
               :filter (lambda (_proc string)
                         (push string chunks)))))
        (set-process-thread process nil)
        (push process processes)
        (push (make-thread
               (lambda ()
                 (while (accept-process-output process))
                 (push (cons idx (apply #'concat (nreverse chunks))) outputs))
               tag)
              threads)))
    (setq processes (nreverse processes))
    (dotimes (i 10)
      (let ((proc (nth i processes)))
        (process-send-string proc (format "line-%d\n" i))
        (process-send-eof proc)))
    (dolist (thread threads)
      (thread-join thread))
    (should (= (length outputs) 10))
    (dotimes (i 10)
      (should (equal (cdr (assq i outputs)) (format "line-%d\n" i))))))

(ert-deftest fe-process-thread-wait/idle-thread-preserves-just-this-one-nil ()
  "Keep reading other unlocked processes when JUST-THIS-ONE is nil with an extra thread."
  (skip-unless (fboundp 'make-thread))
  (let* ((mtx (make-mutex "fe-idle-mtx"))
         (cv (make-condition-variable mtx "fe-idle-cv"))
         (ready nil)
         (release nil)
         (p0-chunks nil)
         (p1-chunks nil)
         (p1-events nil)
         (p2-chunks nil)
         (idle-thread
          (make-thread
           (lambda ()
             (with-mutex mtx
               (setq ready t)
               (condition-notify cv t)
               (while (not release)
                 (condition-wait cv))))
           "fe-idle-extra"))
         p0 p1 p2)
    (unwind-protect
        (progn
          (with-mutex mtx
            (while (not ready)
              (condition-wait cv)))
          (setq p0 (make-process
                    :name "fe-p0"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc s) (push s p0-chunks)))
                p1 (make-process
                    :name "fe-p1"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc s) (push s p1-chunks))
                    :sentinel (lambda (_proc ev) (push ev p1-events)))
                p2 (make-process
                    :name "fe-p2"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc s) (push s p2-chunks))))
          (set-process-thread p0 nil)
          (set-process-thread p1 nil)
          (set-process-thread p2 nil)
          (process-send-string p1 "other-ready\n")
          (process-send-eof p1)
          (should (null (accept-process-output p0 0.25 nil nil)))
          (should (eq (process-status p0) 'run))
          (should (null p0-chunks))
          (should (equal (apply #'concat (nreverse p1-chunks)) "other-ready\n"))
          (should (eq (process-status p1) 'exit))
          (should p1-events)
          (process-send-string p2 "other-ready\n")
          (should (null (accept-process-output p0 0.25 nil t)))
          (should (null p2-chunks))
          (should (null (accept-process-output p0 0.25 nil nil)))
          (should (equal (apply #'concat (nreverse p2-chunks)) "other-ready\n"))
          (process-send-string p0 "from-p0\n")
          (process-send-eof p0)
          (process-send-eof p2)
          (while (accept-process-output p0 0.2 nil nil))
          (while (accept-process-output p2 0.2 nil nil))
          (should (equal (apply #'concat (nreverse p0-chunks)) "from-p0\n"))
          (should (eq (process-status p0) 'exit)))
      (dolist (proc (list p0 p1 p2))
        (when (process-live-p proc)
          (delete-process proc)))
      (with-mutex mtx
        (setq release t)
        (condition-notify cv t))
      (thread-join idle-thread))))

(ert-deftest fe-process-thread-wait/deep-nested-unwind-and-outer-ownership ()
  "Preserve outer waiter ownership across deep (>128) nested waits and nonlocal exits."
  (skip-unless (fboundp 'make-thread))
  (let ((max-lisp-eval-depth (max max-lisp-eval-depth 4000))
        (p0-chunks nil)
        (p1-chunks nil)
        (inner-aborted nil)
        (filter-aborted nil)
        (deep-reached 0)
        (deep-unwound nil)
        (outer-got-output nil)
        (pending-timers nil)
        p0 p1 p2 worker-thread)
    (unwind-protect
        (progn
          (setq p0 (make-process
                    :name "fe-nest-p0"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc s) (push s p0-chunks)))
                p1 (make-process
                    :name "fe-nest-p1"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc s) (push s p1-chunks)))
                p2 (make-process
                    :name "fe-nest-p2"
                    :command '("cat")
                    :connection-type 'pipe
                    :coding 'no-conversion
                    :noquery t
                    :filter (lambda (_proc _s)
                              (throw 'fe-filter-abort 'filter-aborted))))
          (set-process-thread p0 nil)
          (set-process-thread p1 nil)
          (set-process-thread p2 nil)
          (setq worker-thread
                (make-thread
                 (lambda ()
                   (while (eq (process-status p1) 'run)
                     (accept-process-output p1 0.05 nil nil)))
                 "fe-nest-worker"))
          (letrec ((nest-wait
                    (lambda (depth)
                      (setq deep-reached (max deep-reached depth))
                      (if (< depth 140)
                          (let ((tm (run-at-time
                                     0 nil
                                     (lambda ()
                                       (funcall nest-wait (1+ depth))))))
                            (push tm pending-timers)
                            (accept-process-output p0 0.2 nil nil))
                        (accept-process-output nil 0.001)
                        (throw 'fe-deep-unwind (cons 'unwound depth))))))
            (let ((outer-tm
                   (run-at-time
                    0 nil
                    (lambda ()
                      (accept-process-output nil 0.001)
                      (setq inner-aborted
                            (catch 'fe-inner-abort
                              (let ((abort-tm
                                     (run-at-time
                                      0 nil
                                      (lambda ()
                                        (throw 'fe-inner-abort 'aborted)))))
                                (push abort-tm pending-timers)
                                (accept-process-output p0 0.2 nil nil))))
                      (setq filter-aborted
                            (catch 'fe-filter-abort
                              (process-send-string p2 "trigger\n")
                              (accept-process-output p2 0.2 nil t)))
                      (setq deep-unwound
                            (catch 'fe-deep-unwind
                              (funcall nest-wait 2)))
                      (process-send-string p0 "for-outer-p0\n")
                      (process-send-string p1 "for-worker-p1\n")
                      (process-send-eof p1)))))
              (push outer-tm pending-timers)
              (setq outer-got-output (accept-process-output p0 0.5 nil nil))))
          (process-send-eof p0)
          (process-send-eof p2)
          (while (accept-process-output p0 0.2 nil nil))
          (thread-join worker-thread)
          (setq worker-thread nil)
          (should (eq inner-aborted 'aborted))
          (should (eq filter-aborted 'filter-aborted))
          (should (= deep-reached 140))
          (should (equal deep-unwound '(unwound . 140)))
          (should outer-got-output)
          (should (equal (apply #'concat (nreverse p0-chunks)) "for-outer-p0\n"))
          (should (equal (apply #'concat (nreverse p1-chunks)) "for-worker-p1\n")))
      (dolist (tm pending-timers)
        (when (timerp tm)
          (cancel-timer tm)))
      (dolist (proc (list p0 p1 p2))
        (when (process-live-p proc)
          (delete-process proc)))
      (when worker-thread
        (thread-join worker-thread)))))

(provide 'process-thread-wait-protection)
;;; process-thread-wait-protection.el ends here
