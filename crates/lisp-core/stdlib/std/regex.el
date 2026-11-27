;;; std/regex.el --- Linear-time pure-Rust regular expressions -*- lexical-binding: t; -*-

;;; Code:

(defun std-regex-match-p (pattern text)
  "Return non-nil if regex PATTERN matches anywhere in TEXT."
  (internal-host-call "%std-regex-match-p" pattern text))

(defun std-regex-find (pattern text)
  "Return first match of regex PATTERN in TEXT, or nil."
  (internal-host-call "%std-regex-find" pattern text))

(defun std-regex-find-all (pattern text)
  "Return list of all non-overlapping matches of regex PATTERN in TEXT."
  (internal-host-call "%std-regex-find-all" pattern text))

(defun std-regex-captures (pattern text)
  "Return list of capture groups (index 0 = full match, nil for unmatched optional groups), or nil."
  (internal-host-call "%std-regex-captures" pattern text))

(defun std-regex-replace (pattern text replacement &optional all)
  "Replace first (or all when ALL is non-nil) match of PATTERN in TEXT with REPLACEMENT."
  (internal-host-call "%std-regex-replace" pattern text replacement all))

(defun std-regex-split (pattern text)
  "Split TEXT by regex PATTERN."
  (internal-host-call "%std-regex-split" pattern text))

(provide 'std/regex)
;;; std/regex.el ends here