;;; std/kv.el --- Embedded pure-Rust ACID key-value store (redb) -*- lexical-binding: t; -*-

;;; Code:

(defun std-kv-open (path)
  "Open or create an ACID KV database at PATH (or \":memory:\"). Returns handle."
  (internal-host-call "%std-kv-open" path))

(defun std-kv-get (kv key)
  "Look up UTF-8 string value for KEY in KV, or nil if absent."
  (internal-host-call "%std-kv-get" kv key))

(defun std-kv-get-bytes (kv key)
  "Look up raw unibyte byte buffer for KEY in KV, or nil if absent."
  (internal-host-call "%std-kv-get-bytes" kv key))

(defun std-kv-set (kv key value)
  "Persist VALUE (string or unibyte bytes) under KEY in KV."
  (internal-host-call "%std-kv-set" kv key value))

(defun std-kv-del (kv key)
  "Delete KEY from KV; return non-nil if KEY existed."
  (internal-host-call "%std-kv-del" kv key))

(defun std-kv-list (kv &optional prefix)
  "Return sorted list of keys in KV matching optional PREFIX."
  (internal-host-call "%std-kv-list" kv prefix))

(defun std-kv-close (kv)
  "Close KV handle."
  (internal-host-call "%std-kv-close" kv))

(defmacro std-kv-with (binding &rest body)
  "Bind VAR to (std-kv-open PATH) in BINDING, run BODY, and always close."
  (declare (indent 1))
  (let ((var (car binding))
        (path (cadr binding)))
    `(let ((,var (std-kv-open ,path)))
       (unwind-protect
           (progn ,@body)
         (std-kv-close ,var)))))

(provide 'std/kv)
;;; std/kv.el ends here