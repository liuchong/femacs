;;; std/proc.el --- Bounded subprocess execution -*- lexical-binding: t; -*-

;;; Code:

(defun std-proc-run (program &optional args options)
  "Run PROGRAM with ARGS and optional OPTIONS hash-table/alist.
Supported options: \"cwd\", \"env\", \"stdin\", \"timeout-ms\".
Returns a hash-table with \"code\", \"ok\", \"stdout\", \"stderr\", and \"timed-out\"."
  (internal-host-call "%std-proc-run" program args options))

(defun std-proc-output (program &rest args)
  "Run PROGRAM with ARGS, signaling an error on non-zero exit and returning stdout."
  (let ((res (std-proc-run program args)))
    (unless (gethash "ok" res)
      (error "Command %s failed (code %s): %s"
             program
             (gethash "code" res)
             (gethash "stderr" res)))
    (gethash "stdout" res)))

(provide 'std/proc)
;;; std/proc.el ends here