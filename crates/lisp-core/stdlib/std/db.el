;;; std/db.el --- Pure-Rust embedded transactional SQL engine on redb -*- lexical-binding: t; -*-

;;; Code:

(defun std-db-open (path)
  "Open or create a pure-Rust SQL database at PATH (or \":memory:\")."
  (internal-host-call "%std-db-open" path))

(defun std-db-exec (db sql &optional params)
  "Execute DML/DDL SQL statement with optional PARAMS list on DB."
  (internal-host-call "%std-db-exec" db sql params))

(defun std-db-query (db sql &optional params)
  "Execute SQL SELECT query with optional PARAMS list on DB.
Returns a list of hash-tables mapping column names to values."
  (internal-host-call "%std-db-query" db sql params))

(defun std-db-tables (db)
  "Return sorted list of table names in DB."
  (internal-host-call "%std-db-tables" db))

(defun std-db-begin (db)
  "Begin an isolated transaction on DB."
  (internal-host-call "%std-db-begin" db))

(defun std-db-commit (db)
  "Commit the active transaction on DB atomically."
  (internal-host-call "%std-db-commit" db))

(defun std-db-rollback (db)
  "Roll back the active transaction on DB."
  (internal-host-call "%std-db-rollback" db))

(defun std-db-close (db)
  "Close DB handle."
  (internal-host-call "%std-db-close" db))

(defmacro std-db-with (binding &rest body)
  "Bind VAR to (std-db-open PATH) in BINDING, evaluate BODY, and close."
  (declare (indent 1))
  (let ((var (car binding))
        (path (cadr binding)))
    `(let ((,var (std-db-open ,path)))
       (unwind-protect
           (progn ,@body)
         (std-db-close ,var)))))

(defmacro std-db-with-tx (db &rest body)
  "Execute BODY inside a transaction on DB, committing on success or rolling back on error."
  (declare (indent 1))
  (let ((ok-sym (gensym "tx-ok")))
    `(let ((,ok-sym nil))
       (std-db-begin ,db)
       (unwind-protect
           (prog1 (progn ,@body)
             (std-db-commit ,db)
             (setq ,ok-sym t))
         (unless ,ok-sym
           (ignore-errors (std-db-rollback ,db)))))))

(provide 'std/db)
;;; std/db.el ends here