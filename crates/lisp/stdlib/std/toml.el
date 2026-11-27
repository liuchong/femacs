;;; std/toml.el --- Pure-Rust TOML codec -*- lexical-binding: t; -*-

;;; Code:

(defun std-toml-parse (str)
  "Parse TOML string STR into a hash-table / nested Lisp structure."
  (internal-host-call "%std-toml-parse" str))

(defun std-toml-stringify (value)
  "Serialize hash-table / map VALUE into a pretty TOML string."
  (internal-host-call "%std-toml-stringify" value))

(provide 'std/toml)
;;; std/toml.el ends here