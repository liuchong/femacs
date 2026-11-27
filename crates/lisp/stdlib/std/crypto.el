;;; std/crypto.el --- Pure-Rust CSPRNG, SHA, HMAC, UUID, and hex helpers -*- lexical-binding: t; -*-

;;; Code:

(defun std-crypto-random-bytes (len)
  "Return LEN cryptographically random bytes as a unibyte string."
  (internal-host-call "%std-crypto-random-bytes" len))

(defun std-crypto-sha256 (data)
  "Return lowercase hex SHA-256 digest of DATA."
  (internal-host-call "%std-crypto-sha256" data))

(defun std-crypto-sha256-bytes (data)
  "Return raw 32-byte SHA-256 digest of DATA as a unibyte string."
  (internal-host-call "%std-crypto-sha256-bytes" data))

(defun std-crypto-sha512 (data)
  "Return lowercase hex SHA-512 digest of DATA."
  (internal-host-call "%std-crypto-sha512" data))

(defun std-crypto-hmac-sha256 (key data)
  "Return lowercase hex HMAC-SHA256 of DATA using KEY."
  (internal-host-call "%std-crypto-hmac-sha256" key data))

(defun std-crypto-uuid-v4 ()
  "Generate a random UUID v4 string."
  (internal-host-call "%std-crypto-uuid-v4"))

(defun std-crypto-uuid-v7 ()
  "Generate a time-ordered UUID v7 string."
  (internal-host-call "%std-crypto-uuid-v7"))

(defun std-crypto-hex-encode (data)
  "Encode bytes/string DATA as lowercase hexadecimal."
  (internal-host-call "%std-crypto-hex-encode" data))

(defun std-crypto-hex-decode (hex)
  "Decode hexadecimal string HEX into raw unibyte bytes."
  (internal-host-call "%std-crypto-hex-decode" hex))

(defun std-crypto-constant-time-eq (a b)
  "Compare two byte sequences or strings A and B in constant time."
  (internal-host-call "%std-crypto-constant-time-eq" a b))

(provide 'std/crypto)
;;; std/crypto.el ends here