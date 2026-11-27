;;; std/json.el --- Fast pure-Rust JSON codec -*- lexical-binding: t; -*-

;;; Code:

(defun std-json-parse (str)
  "Parse JSON string STR into Lisp values (hash-tables, lists, scalars)."
  (internal-host-call "%std-json-parse" str))

(defun std-json-stringify (value &optional pretty)
  "Serialize Lisp VALUE to a JSON string. When PRETTY is non-nil, pretty-print."
  (internal-host-call "%std-json-stringify" value pretty))

(provide 'std/json)
;;; std/json.el ends here