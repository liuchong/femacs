;;; std/fs.el --- Pure-Rust filesystem, walkdir, and glob utilities -*- lexical-binding: t; -*-

;;; Code:

(defun std-fs-read-bytes (path)
  "Read raw bytes from PATH as a unibyte string."
  (internal-host-call "%std-fs-read-bytes" path))

(defun std-fs-read-string (path)
  "Read UTF-8 text from PATH."
  (internal-host-call "%std-fs-read-string" path))

(defun std-fs-write-bytes (path data &optional create-parents)
  "Write bytes/string DATA to PATH, creating parent directories if CREATE-PARENTS is non-nil."
  (internal-host-call "%std-fs-write-bytes" path data create-parents))

(defun std-fs-write-string (path text &optional create-parents)
  "Write UTF-8 TEXT to PATH, creating parent directories if CREATE-PARENTS is non-nil."
  (internal-host-call "%std-fs-write-bytes" path text create-parents))

(defun std-fs-append (path data)
  "Append DATA to PATH (creating file and parent directories as needed)."
  (internal-host-call "%std-fs-append" path data))

(defun std-fs-mkdir-p (path)
  "Recursively create directory PATH."
  (internal-host-call "%std-fs-mkdir-p" path))

(defun std-fs-rm-rf (path)
  "Remove file or directory tree PATH recursively if it exists."
  (internal-host-call "%std-fs-rm-rf" path))

(defun std-fs-copy (src dst)
  "Copy file SRC to DST, creating parent directories as needed."
  (internal-host-call "%std-fs-copy" src dst))

(defun std-fs-rename (src dst)
  "Rename/move SRC to DST."
  (internal-host-call "%std-fs-rename" src dst))

(defun std-fs-stat (path)
  "Return metadata hash-table for PATH, or nil if it does not exist."
  (internal-host-call "%std-fs-stat" path))

(defun std-fs-exists-p (path)
  "Return non-nil if PATH exists on disk."
  (not (null (std-fs-stat path))))

(defun std-fs-walk (dir &optional max-depth)
  "Recursively walk DIR up to MAX-DEPTH and return sorted paths."
  (internal-host-call "%std-fs-walk" dir max-depth))

(defun std-fs-glob (dir pattern)
  "Find paths inside DIR matching glob PATTERN (e.g. \"**/*.el\")."
  (internal-host-call "%std-fs-glob" dir pattern))

(defun std-fs-glob-match (pattern text)
  "Return non-nil if glob PATTERN matches TEXT."
  (internal-host-call "%std-fs-glob-match" pattern text))

(defun std-fs-tempdir (&optional prefix)
  "Create and return a unique temporary directory path with optional PREFIX."
  (internal-host-call "%std-fs-tempdir" prefix))

(provide 'std/fs)
;;; std/fs.el ends here