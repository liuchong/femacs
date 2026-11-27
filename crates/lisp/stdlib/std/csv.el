;;; std/csv.el --- Pure-Rust CSV codec -*- lexical-binding: t; -*-

;;; Code:

(defun std-csv-parse (str &optional has-headers)
  "Parse CSV string STR into a list of rows.
When HAS-HEADERS is non-nil, returns a list of hash-tables keyed by column headers."
  (internal-host-call "%std-csv-parse" str has-headers))

(defun std-csv-stringify (rows &optional headers)
  "Serialize ROWS (lists or hash-tables) with optional HEADERS list to a CSV string."
  (internal-host-call "%std-csv-stringify" rows headers))

(provide 'std/csv)
;;; std/csv.el ends here