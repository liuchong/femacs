;;; std/path.el --- Pure-Rust cross-platform path manipulation -*- lexical-binding: t; -*-

;;; Code:

(defun std-path-join (&rest parts)
  "Join path PARTS (strings or a single list of strings) using forward slashes."
  (let ((flat (if (and (= (length parts) 1) (listp (car parts)))
                  (car parts)
                parts)))
    (internal-host-call "%std-path-join" flat)))

(defun std-path-normalize (path)
  "Lexically normalize `.' and `..' components in PATH."
  (internal-host-call "%std-path-normalize" path))

(defun std-path-dirname (path)
  "Return parent directory of PATH, or nil."
  (internal-host-call "%std-path-dirname" path))

(defun std-path-basename (path)
  "Return final file name component of PATH, or nil."
  (internal-host-call "%std-path-basename" path))

(defun std-path-stem (path)
  "Return file name stem without final extension of PATH, or nil."
  (internal-host-call "%std-path-stem" path))

(defun std-path-ext (path)
  "Return file extension without leading dot of PATH, or nil."
  (internal-host-call "%std-path-ext" path))

(defun std-path-abs-p (path)
  "Return non-nil if PATH is absolute."
  (internal-host-call "%std-path-is-abs" path))

(provide 'std/path)
;;; std/path.el ends here