;;; std/base64.el --- Pure-Rust Base64 standard & URL-safe codecs -*- lexical-binding: t; -*-

;;; Code:

(defun std-base64-encode (data &optional url-safe)
  "Encode DATA (string or unibyte bytes) to Base64 (URL-safe without padding if URL-SAFE)."
  (internal-host-call "%std-base64-encode" data url-safe))

(defun std-base64-decode (encoded &optional url-safe)
  "Decode Base64 string ENCODED into a raw unibyte byte buffer."
  (internal-host-call "%std-base64-decode" encoded url-safe))

(defun std-base64-decode-string (encoded &optional url-safe)
  "Decode Base64 string ENCODED into a UTF-8 string."
  (internal-host-call "%std-base64-decode-string" encoded url-safe))

(provide 'std/base64)
;;; std/base64.el ends here