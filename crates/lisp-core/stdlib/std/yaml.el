;;; std/yaml.el --- Pure-Rust YAML 1.2 codec -*- lexical-binding: t; -*-

;;; Code:

(defun std-yaml-parse (str)
  "Parse YAML string STR into Lisp values."
  (internal-host-call "%std-yaml-parse" str))

(defun std-yaml-stringify (value)
  "Serialize Lisp VALUE into a YAML string."
  (internal-host-call "%std-yaml-stringify" value))

(provide 'std/yaml)
;;; std/yaml.el ends here