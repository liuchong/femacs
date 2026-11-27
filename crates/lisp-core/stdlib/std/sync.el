;;; std/sync.el --- Cross-isolate atomics, mutexes, and shared maps -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure-Rust synchronization primitives (`std/sync`) shareable across parallel `NoGil` isolates.

;;; Code:

(defun std-atomic-make (&optional initial)
  "Create an atomic 64-bit integer initialized to INITIAL (default 0)."
  (internal-host-call "%std-atomic-make" (or initial 0)))

(defun std-atomic-load (atomic)
  "Atomically load the current integer value of ATOMIC."
  (internal-host-call "%std-atomic-load" atomic))

(defun std-atomic-store (atomic value)
  "Atomically store VALUE into ATOMIC and return VALUE."
  (internal-host-call "%std-atomic-store" atomic value))

(defun std-atomic-add (atomic delta)
  "Atomically add DELTA to ATOMIC and return the new sum."
  (internal-host-call "%std-atomic-add" atomic delta))

(defun std-atomic-cas (atomic expected desired)
  "Compare-and-swap ATOMIC from EXPECTED to DESIRED, returning non-nil on success."
  (internal-host-call "%std-atomic-cas" atomic expected desired))

(defun std-mutex-make (&optional initial)
  "Create a mutex optionally protecting INITIAL value."
  (internal-host-call "%std-mutex-make" initial))

(defun std-mutex-lock (mutex)
  "Acquire MUTEX and return its stored value."
  (internal-host-call "%std-mutex-lock" mutex))

(defun std-mutex-unlock (mutex &optional new-value)
  "Release MUTEX, optionally updating its protected value when NEW-VALUE is passed."
  (if (null new-value)
      (internal-host-call "%std-mutex-unlock" mutex)
    (internal-host-call "%std-mutex-unlock" mutex new-value)))

(defun std-mutex-get (mutex)
  "Read MUTEX stored value under lock."
  (internal-host-call "%std-mutex-get" mutex))

(defun std-mutex-set (mutex value)
  "Swap MUTEX stored value to VALUE under lock and return the prior value."
  (internal-host-call "%std-mutex-set" mutex value))

(defmacro std-with-mutex (mutex &rest body)
  "Execute BODY while holding MUTEX, guaranteeing unlock on unwind."
  (declare (indent 1) (debug t))
  (let ((m-var (make-symbol "mutex")))
    `(let ((,m-var ,mutex))
       (std-mutex-lock ,m-var)
       (unwind-protect
           (progn ,@body)
         (internal-host-call "%std-mutex-unlock" ,m-var)))))

(defun std-shared-make ()
  "Create a concurrent shared string-keyed map accessible across isolates."
  (internal-host-call "%std-shared-make"))

(defun std-shared-get (map key &optional default)
  "Lookup KEY in shared MAP, returning DEFAULT if missing."
  (internal-host-call "%std-shared-get" map key default))

(defun std-shared-set (map key value)
  "Insert KEY -> VALUE into shared MAP and return VALUE."
  (internal-host-call "%std-shared-set" map key value))

(defun std-shared-del (map key)
  "Delete KEY from shared MAP, returning non-nil if present."
  (internal-host-call "%std-shared-del" map key))

(defun std-shared-keys (map)
  "Return sorted list of keys currently present in shared MAP."
  (internal-host-call "%std-shared-keys" map))

(provide 'std/sync)
;;; std/sync.el ends here
