;;; std/time.el --- Pure-Rust timestamps, timezones, and monotonic clocks -*- lexical-binding: t; -*-

;;; Code:

(defun std-time-now-utc ()
  "Return current UTC time formatted as RFC-3339."
  (internal-host-call "%std-time-now-utc"))

(defun std-time-now-unix-ms ()
  "Return current Unix epoch timestamp in milliseconds."
  (internal-host-call "%std-time-now-unix-ms"))

(defun std-time-monotonic-ms ()
  "Return monotonic elapsed milliseconds since runtime initialization."
  (internal-host-call "%std-time-monotonic-ms"))

(defun std-time-sleep-ms (ms)
  "Sleep current thread for MS milliseconds."
  (internal-host-call "%std-time-sleep-ms" ms))

(defun std-time-parse-rfc3339 (str)
  "Parse RFC-3339 timestamp STR into a hash-table of components."
  (internal-host-call "%std-time-parse-rfc3339" str))

(defun std-time-format (unix-secs format &optional timezone)
  "Format UNIX-SECS with strftime FORMAT in TIMEZONE (default UTC)."
  (internal-host-call "%std-time-format" unix-secs format timezone))

(defun std-time-add-seconds (timestamp seconds)
  "Add integer SECONDS to RFC-3339 TIMESTAMP and return RFC-3339 string."
  (internal-host-call "%std-time-add-seconds" timestamp seconds))

(defun std-time-diff-seconds (a b)
  "Return floating-point seconds difference (A - B) between two RFC-3339 timestamps."
  (internal-host-call "%std-time-diff-seconds" a b))

(provide 'std/time)
;;; std/time.el ends here