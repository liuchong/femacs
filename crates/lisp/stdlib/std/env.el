;;; std/env.el --- Process environment & .env file loader -*- lexical-binding: t; -*-

;;; Code:

(defun std-env-get (key &optional default)
  "Return environment variable KEY, or DEFAULT (nil if omitted) when unset."
  (internal-host-call "%std-env-get" key default))

(defun std-env-all ()
  "Return a hash-table of all current environment variables."
  (internal-host-call "%std-env-all"))

(defun std-env-parse-dotenv (text)
  "Parse `.env' syntax TEXT into a hash-table of key/value strings."
  (internal-host-call "%std-env-parse-dotenv" text))

(defun std-env-load-dotenv (path &optional override)
  "Parse `.env' file at PATH into a hash-table and apply values via `setenv'.
Existing environment variables are preserved unless OVERRIDE is non-nil."
  (let* ((text (internal-host-call "%std-fs-read-string" path))
         (parsed (std-env-parse-dotenv text)))
    (maphash (lambda (k v)
               (when (or override (null (getenv k)))
                 (setenv k v)))
             parsed)
    parsed))

(provide 'std/env)
;;; std/env.el ends here