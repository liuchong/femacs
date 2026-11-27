;;; std/task.el --- Parallel OS threads & bounded worker pools across NoGil isolates -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure-Rust parallel task spawning and `std-task-parallel-map' executing inside
;; isolated per-thread `NoGil` VMs.

;;; Code:

(defun std-task-mode ()
  "Return current VM concurrency mode symbol (`no-gil' or `gil')."
  (internal-host-call "%std-task-mode"))

(defun std-task-cpu-count ()
  "Return logical CPU count available for parallel task execution."
  (internal-host-call "%std-task-cpu-count"))

(defun std-task--serialize-worker-fn (fn target-sym)
  "Serialize FN (symbol, lambda, or compiled closure) as setup form defining TARGET-SYM."
  (let ((print-length nil)
        (print-level nil)
        (print-circle nil))
    (cond
     ((symbolp fn)
      (unless (fboundp fn)
        (error "std/task: function `%s' is void" fn))
      (let ((def (symbol-function fn)))
        (if (subrp def)
            (format "(progn (require 'std/chan) (require 'std/sync) (defalias '%S #'%S))"
                    target-sym fn)
          (let ((printed (prin1-to-string def)))
            (format "(progn (require 'std/chan) (require 'std/sync) (defalias '%S '%s) (defalias '%S #'%S))"
                    fn printed target-sym fn)))))
     ((or (functionp fn) (consp fn))
      (format "(progn (require 'std/chan) (require 'std/sync) (defalias '%S '%s))"
              target-sym (prin1-to-string fn)))
     (t
      (error "std/task: expected function, symbol, or code string; got %S" fn)))))

(defun std-task--build-eval-code (fn-or-code args)
  "Build self-contained Lisp source evaluating FN-OR-CODE with ARGS in a worker isolate."
  (if (stringp fn-or-code)
      (if (null args)
          (format "(progn (require 'std/chan) (require 'std/sync) %s)" fn-or-code)
        (let ((print-length nil)
              (print-level nil))
          (format "(progn (require 'std/chan) (require 'std/sync) (let ((std-task-args '%s)) %s))"
                  (prin1-to-string args)
                  fn-or-code)))
    (let ((setup (std-task--serialize-worker-fn fn-or-code 'std-task--entry))
          (print-length nil)
          (print-level nil))
      (format "(progn %s (apply #'std-task--entry '%s))"
              setup
              (prin1-to-string args)))))

(defun std-task-spawn (fn-or-code &rest args)
  "Spawn a parallel task running FN-OR-CODE (code string, symbol, or lambda) with ARGS."
  (internal-host-call "%std-task-spawn"
                      (std-task--build-eval-code fn-or-code args)))

(defun std-task-done-p (task)
  "Return non-nil if TASK has completed."
  (internal-host-call "%std-task-done-p" task))

(defun std-task-await (task)
  "Wait for TASK to finish and return its value (or signal its error)."
  (internal-host-call "%std-task-await" task))

(defun std-task-await-timeout (task timeout-ms)
  "Wait up to TIMEOUT-MS ms for TASK; return (ok . VAL) or (timeout . nil)."
  (let ((pair (internal-host-call "%std-task-await-timeout" task timeout-ms)))
    (cons (nth 0 pair) (nth 1 pair))))

(defun std-task-parallel-map (fn items &optional workers)
  "Apply FN in parallel across ITEMS using up to WORKERS isolates, preserving order."
  (if (null items)
      nil
    (let* ((item-list (if (vectorp items) (append items nil) items))
           (setup (std-task--serialize-worker-fn fn 'std-task--map-fn)))
      (internal-host-call "%std-task-parallel-map"
                          setup
                          "std-task--map-fn"
                          item-list
                          (or workers 0)))))

(defun std-task-pool-create (&optional size)
  "Create a bounded worker pool of SIZE pre-warmed `NoGil' worker isolates."
  (internal-host-call "%std-task-pool-create" (or size (std-task-cpu-count))))

(defun std-task-pool-submit (pool fn-or-code &rest args)
  "Submit FN-OR-CODE with ARGS to POOL and return a task handle awaitable via `std-task-await'."
  (internal-host-call "%std-task-pool-submit"
                      pool
                      (std-task--build-eval-code fn-or-code args)))

(defun std-task-pool-shutdown (pool)
  "Gracefully shut down worker POOL and join its threads."
  (internal-host-call "%std-task-pool-shutdown" pool))

(provide 'std/task)
;;; std/task.el ends here
