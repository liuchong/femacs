;;; std/log.el --- Structured text & JSON logging -*- lexical-binding: t; -*-

;;; Code:

(require 'std/json)
(require 'std/time)

(defvar std-log-level 'info
  "Minimum log severity: one of `debug', `info', `warn', or `error'.")

(defvar std-log-format 'text
  "Output format symbol: `text' or `json'.")

(defun std-log--level-rank (lvl)
  "Map severity symbol LVL to numeric order."
  (pcase lvl
    ('debug 0)
    ('info 1)
    ('warn 2)
    ('error 3)
    (_ 1)))

(defun std-log-format-line (level message &optional fields)
  "Format a structured log record for LEVEL, MESSAGE, and optional FIELDS alist/hash-table."
  (let ((ts (std-time-now-utc))
        (lvl-str (upcase (symbol-name level))))
    (if (eq std-log-format 'json)
        (let ((obj (make-hash-table :test 'equal)))
          (puthash "ts" ts obj)
          (puthash "level" lvl-str obj)
          (puthash "msg" message obj)
          (cond
           ((hash-table-p fields)
            (maphash (lambda (k v) (puthash (format "%s" k) v obj)) fields))
           ((listp fields)
            (dolist (pair fields)
              (when (consp pair)
                (puthash (format "%s" (car pair)) (cdr pair) obj)))))
          (std-json-stringify obj))
      (let ((extra
             (cond
              ((hash-table-p fields)
               (let ((acc nil))
                 (maphash (lambda (k v) (push (format "%s=%S" k v) acc)) fields)
                 (mapconcat #'identity (nreverse acc) " ")))
              ((listp fields)
               (mapconcat (lambda (p) (format "%s=%S" (car p) (cdr p))) fields " "))
              (t ""))))
        (if (string-empty-p extra)
            (format "%s [%s] %s" ts lvl-str message)
          (format "%s [%s] %s %s" ts lvl-str message extra))))))

(defun std-log--emit (level message &optional fields)
  "Emit structured log line to stderr if LEVEL meets `std-log-level'."
  (when (>= (std-log--level-rank level) (std-log--level-rank std-log-level))
    (let ((line (std-log-format-line level message fields)))
      (princ (concat line "\n") #'external-debugging-output)
      line)))

(defun std-log-debug (message &optional fields)
  "Log debug MESSAGE with optional structured FIELDS."
  (std-log--emit 'debug message fields))

(defun std-log-info (message &optional fields)
  "Log info MESSAGE with optional structured FIELDS."
  (std-log--emit 'info message fields))

(defun std-log-warn (message &optional fields)
  "Log warning MESSAGE with optional structured FIELDS."
  (std-log--emit 'warn message fields))

(defun std-log-error (message &optional fields)
  "Log error MESSAGE with optional structured FIELDS."
  (std-log--emit 'error message fields))

(provide 'std/log)
;;; std/log.el ends here