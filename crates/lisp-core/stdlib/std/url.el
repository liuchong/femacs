;;; std/url.el --- Pure-Rust URL parsing, joining, and query codecs -*- lexical-binding: t; -*-

;;; Code:

(defun std-url-parse (url)
  "Parse URL string into a hash-table (`scheme', `host', `port', `path', `query', `fragment', `href')."
  (internal-host-call "%std-url-parse" url))

(defun std-url-join (base rel)
  "Resolve REL URL against BASE URL."
  (internal-host-call "%std-url-join" base rel))

(defun std-url-encode (text)
  "Percent-encode TEXT as a query component."
  (internal-host-call "%std-url-encode" text))

(defun std-url-decode (text)
  "Decode URL form/percent-encoded TEXT."
  (internal-host-call "%std-url-decode" text))

(defun std-url-build-query (params)
  "Encode PARAMS (hash-table or alist) into a query string."
  (internal-host-call "%std-url-build-query" params))

(provide 'std/url)
;;; std/url.el ends here