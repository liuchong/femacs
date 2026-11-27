;;; std/cli.el --- Declarative CLI flag & positional argument parser -*- lexical-binding: t; -*-

;;; Code:

(require 'cl-lib)

(defun std-cli--find-spec (specs name-or-flag long-p)
  "Locate option spec in SPECS matching NAME-OR-FLAG."
  (cl-find-if
   (lambda (spec)
     (if long-p
         (equal (plist-get spec :long) name-or-flag)
       (equal (plist-get spec :short) name-or-flag)))
   specs))

(defun std-cli--set-result (table key val)
  "Store KEY -> VAL into result hash-table TABLE using string key name."
  (puthash (if (symbolp key) (symbol-name key) key) val table))

(defun std-cli-parse (specs &optional argv)
  "Parse ARGV (default `command-line-args-left') according to SPECS.
Each entry in SPECS is a plist with `:name' (symbol or string) plus optional
`:short' (e.g. \"-p\"), `:long' (e.g. \"--port\"), `:flag' (t for boolean),
`:default', `:required', `:parse' (`int', `float', or function), and `:help'.
Returns a hash-table containing parsed option keys plus \"_\" for positional args."
  (let ((args (if (null argv) command-line-args-left argv))
        (res (make-hash-table :test 'equal))
        (positional nil)
        (only-pos nil))
    (dolist (spec specs)
      (let ((k (plist-get spec :name))
            (def (plist-get spec :default)))
        (when k
          (std-cli--set-result res k (if (plist-get spec :flag) (not (null def)) def)))))
    (while args
      (let ((tok (pop args)))
        (cond
         (only-pos
          (push tok positional))
         ((string= tok "--")
          (setq only-pos t))
         ((and (> (length tok) 2) (string-prefix-p "--" tok))
          (let* ((eq-pos (string-match "=" tok))
                 (flag (if eq-pos (substring tok 0 eq-pos) tok))
                 (inline-val (when eq-pos (substring tok (1+ eq-pos))))
                 (spec (std-cli--find-spec specs flag t)))
            (unless spec
              (error "Unknown option %s" flag))
            (let ((k (plist-get spec :name)))
              (if (plist-get spec :flag)
                  (std-cli--set-result res k t)
                (let ((raw (or inline-val
                               (if args (pop args)
                                 (error "Option %s requires a value" flag)))))
                  (std-cli--set-result res k (std-cli--coerce spec raw)))))))
         ((and (> (length tok) 1) (string-prefix-p "-" tok))
          (let ((spec (std-cli--find-spec specs tok nil)))
            (unless spec
              (error "Unknown option %s" tok))
            (let ((k (plist-get spec :name)))
              (if (plist-get spec :flag)
                  (std-cli--set-result res k t)
                (let ((raw (if args (pop args)
                             (error "Option %s requires a value" tok))))
                  (std-cli--set-result res k (std-cli--coerce spec raw)))))))
         (t
          (push tok positional)))))
    (dolist (spec specs)
      (when (plist-get spec :required)
        (let* ((k (plist-get spec :name))
               (kname (if (symbolp k) (symbol-name k) k)))
          (when (null (gethash kname res))
            (error "Missing required option %s"
                   (or (plist-get spec :long) (plist-get spec :short) kname))))))
    (puthash "_" (nreverse positional) res)
    res))

(defun std-cli--coerce (spec raw)
  "Convert RAW CLI string according to `:parse' property in SPEC."
  (let ((parser (plist-get spec :parse)))
    (cond
     ((eq parser 'int) (string-to-number raw))
     ((eq parser 'float) (float (string-to-number raw)))
     ((functionp parser) (funcall parser raw))
     (t raw))))

(defun std-cli-format-help (program-name specs)
  "Render human-readable CLI help text for PROGRAM-NAME and SPECS."
  (let ((lines (list (format "Usage: %s [OPTIONS] [ARGS...]" program-name) "" "Options:")))
    (dolist (spec specs)
      (let* ((short (or (plist-get spec :short) "  "))
             (long (or (plist-get spec :long) ""))
             (help (or (plist-get spec :help) ""))
             (def (plist-get spec :default))
             (def-str (if def (format " (default: %s)" def) "")))
        (push (format "  %-3s %-18s %s%s" short long help def-str) lines)))
    (mapconcat #'identity (nreverse lines) "\n")))

(provide 'std/cli)
;;; std/cli.el ends here