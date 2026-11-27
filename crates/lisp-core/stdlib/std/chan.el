;;; std/chan.el --- Thread-safe typed channels across Lisp isolates -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure-Rust MPMC channels (`std-chan-make`, `std-chan-send`, `std-chan-recv`, etc.)
;; safe to pass by integer handle across parallel `NoGil` `std/task` workers.

;;; Code:

(defun std-chan-make (&optional capacity)
  "Create a new channel. CAPACITY 0 or nil creates an unbounded channel."
  (internal-host-call "%std-chan-make" (or capacity 0)))

(defun std-chan-send (chan value)
  "Send VALUE on CHAN, blocking if CHAN is bounded and full."
  (internal-host-call "%std-chan-send" chan value))

(defun std-chan-try-send (chan value)
  "Attempt non-blocking send of VALUE on CHAN; return t on success or nil if full."
  (internal-host-call "%std-chan-try-send" chan value))

(defun std-chan-recv (chan)
  "Receive a value from CHAN as (ok . VALUE), or (closed . nil) if closed and empty."
  (let ((pair (internal-host-call "%std-chan-recv" chan)))
    (cons (if (nth 0 pair) 'ok 'closed) (nth 1 pair))))

(defun std-chan-try-recv (chan)
  "Non-blocking receive on CHAN; returns (STATUS . VALUE) where STATUS is `ok', `empty', or `closed'."
  (let ((pair (internal-host-call "%std-chan-try-recv" chan)))
    (cons (nth 0 pair) (nth 1 pair))))

(defun std-chan-recv-timeout (chan timeout-ms)
  "Receive from CHAN up to TIMEOUT-MS ms; returns (STATUS . VALUE) with STATUS `ok', `timeout', or `closed'."
  (let ((pair (internal-host-call "%std-chan-recv-timeout" chan timeout-ms)))
    (cons (nth 0 pair) (nth 1 pair))))

(defun std-chan-close (chan)
  "Close CHAN cleanly so waiting receivers drain and exit."
  (internal-host-call "%std-chan-close" chan))

(defun std-chan-closed-p (chan)
  "Return non-nil if CHAN has been closed."
  (internal-host-call "%std-chan-closed-p" chan))

(defun std-chan-len (chan)
  "Return the number of currently queued items in CHAN."
  (internal-host-call "%std-chan-len" chan))

(provide 'std/chan)
;;; std/chan.el ends here
