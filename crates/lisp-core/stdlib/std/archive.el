;;; std/archive.el --- Pure-Rust gzip/zlib, tar, and zip archive tools -*- lexical-binding: t; -*-

;;; Code:

(defun std-gzip-compress (data)
  "Compress DATA (string or unibyte bytes) into gzip bytes."
  (internal-host-call "%std-gzip-compress" data))

(defun std-gzip-decompress (data)
  "Decompress gzip DATA bytes into raw unibyte bytes."
  (internal-host-call "%std-gzip-decompress" data))

(defun std-zlib-compress (data)
  "Compress DATA (string or unibyte bytes) into zlib bytes."
  (internal-host-call "%std-zlib-compress" data))

(defun std-zlib-decompress (data)
  "Decompress zlib DATA bytes into raw unibyte bytes."
  (internal-host-call "%std-zlib-decompress" data))

(defun std-tar-pack (src-dir out-path &optional gzip)
  "Pack SRC-DIR into tar archive OUT-PATH (gzipped when GZIP is non-nil)."
  (internal-host-call "%std-tar-pack" src-dir out-path gzip))

(defun std-tar-unpack (archive-path dest-dir &optional gzip)
  "Safely unpack tar ARCHIVE-PATH into DEST-DIR (gzipped when GZIP is non-nil).
Rejects path-traversal entries and symlinks."
  (internal-host-call "%std-tar-unpack" archive-path dest-dir gzip))

(defun std-tar-list (archive-path &optional gzip)
  "Return list of entry paths inside tar ARCHIVE-PATH."
  (internal-host-call "%std-tar-list" archive-path gzip))

(defun std-zip-pack (src-dir out-path)
  "Pack SRC-DIR into zip archive OUT-PATH using pure-Rust Deflate."
  (internal-host-call "%std-zip-pack" src-dir out-path))

(defun std-zip-unpack (archive-path dest-dir)
  "Safely unpack zip ARCHIVE-PATH into DEST-DIR, rejecting path-traversal entries."
  (internal-host-call "%std-zip-unpack" archive-path dest-dir))

(defun std-zip-list (archive-path)
  "Return list of entry paths inside zip ARCHIVE-PATH."
  (internal-host-call "%std-zip-list" archive-path))

(provide 'std/archive)
;;; std/archive.el ends here