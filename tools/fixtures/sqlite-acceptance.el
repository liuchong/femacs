;;; sqlite-acceptance.el --- Real system database workflows -*- lexical-binding: t; -*-
(require 'ert)

(defmacro fe-sqlite--with-db (symbol &rest body)
  (declare (indent 1))
  `(let ((,symbol (sqlite-open)))
     (unwind-protect (progn ,@body)
       (sqlite-close ,symbol))))

(ert-deftest fe-sqlite-capability ()
  (should (sqlite-available-p))
  (should (string-match-p "\\`[0-9]+\\.[0-9]+\\.[0-9]+" (sqlite-version)))
  (dolist (function '(sqlite-open sqlite-close sqlite-execute sqlite-select
                     sqlite-execute-batch sqlite-transaction sqlite-commit
                     sqlite-rollback sqlite-pragma sqlite-next sqlite-columns
                     sqlite-more-p sqlite-finalize sqlite-version))
    (should (subrp (symbol-function function))))
  (fe-sqlite--with-db db
    (should (sqlitep db))
    (should (eq (type-of db) 'sqlite))
    (should-not (sqlitep 'unrelated))))

(ert-deftest fe-sqlite-typed-rows ()
  (fe-sqlite--with-db db
    (sqlite-execute db "create table values_test (txt text, raw blob, big integer, real_value real, absent, yes, no)")
    (let ((blob (unibyte-string 0 128 255))
          (big 9223372036854775806))
      (put-text-property 0 (length blob) 'coding-system 'binary blob)
      (should (= 1 (sqlite-execute db "insert into values_test values (?, ?, ?, ?, ?, ?, ?)"
                                 (vector "你好&世界" blob big -1.25 nil t 'false))))
      (should (equal (sqlite-select db "select * from values_test")
                     (list (list "你好&世界" blob big -1.25 nil 1 0)))))))

(ert-deftest fe-sqlite-list-and-vector-parameters ()
  (fe-sqlite--with-db db
    (sqlite-execute db "create table parameter_test (n integer, label text)")
    (sqlite-execute db "insert into parameter_test values (?, ?)" '(1 "一"))
    (sqlite-execute db "insert into parameter_test values (?, ?)" [2 "二"])
    (should (equal (sqlite-select db "select label from parameter_test where n >= ? order by n" '(1))
                   '(("一") ("二"))))))

(ert-deftest fe-sqlite-rollback-and-commit ()
  (fe-sqlite--with-db db
    (sqlite-execute db "create table transaction_test (n integer)")
    (sqlite-transaction db)
    (sqlite-execute db "insert into transaction_test values (1)")
    (sqlite-rollback db)
    (should-not (sqlite-select db "select * from transaction_test"))
    (sqlite-transaction db)
    (sqlite-execute db "insert into transaction_test values (2)")
    (sqlite-commit db)
    (should (equal (sqlite-select db "select * from transaction_test") '((2))))))

(ert-deftest fe-sqlite-statement-lifetime ()
  (fe-sqlite--with-db db
    (let ((statement (sqlite-select db "select 1 as n union all select 2" nil 'set)))
      (unwind-protect
          (progn
            (should (sqlitep statement))
            (should (equal (sqlite-columns statement) '("n")))
            (should (sqlite-more-p statement))
            (should (equal (sqlite-next statement) '(1)))
            (should (equal (sqlite-next statement) '(2)))
            (should-not (sqlite-next statement))
            (should-not (sqlite-more-p statement))
            (sqlite-finalize statement)
            (should-error (sqlite-next statement) :type 'sqlite-error)
            (setq statement nil))
        (when statement (sqlite-finalize statement))))))

(ert-deftest fe-sqlite-file-reopen-and-readonly ()
  (let* ((root (or (getenv "FEMACS_SQLITE_TEST_DIR") (error "Owned output directory required")))
         (path (expand-file-name "unicode-database.sqlite" root))
         (db (sqlite-open path)))
    (unwind-protect
        (progn
          (sqlite-execute db "create table saved (value text)")
          (sqlite-execute db "insert into saved values (?)" '("可持久保存")))
      (sqlite-close db))
    (should (> (file-attribute-size (file-attributes path)) 0))
    (setq db (sqlite-open path t))
    (unwind-protect
        (progn
          (should (equal (sqlite-select db "select value from saved") '(("可持久保存"))))
          (should-error (sqlite-execute db "insert into saved values ('denied')")
                        :type 'sqlite-error))
      (sqlite-close db))))

(ert-deftest fe-sqlite-error-recovery ()
  (fe-sqlite--with-db db
    (should-error (sqlite-execute db "select * from no_such_table") :type 'sqlite-error)
    (should-error (sqlite-execute db "select ?" 'invalid-values) :type 'sqlite-error)
    (should (equal (sqlite-select db "select ?" ["still usable"]) '(("still usable"))))))

(ert-deftest fe-sqlite-closed-database ()
  (let ((db (sqlite-open)))
    (sqlite-close db)
    (should (sqlite-close db))
    (should (sqlitep db))
    (should-error (sqlite-select db "select 1") :type 'sqlite-error)))

(ert-deftest fe-sqlite-retention-across-gc ()
  (fe-sqlite--with-db db
    (sqlite-execute db "create table retained (n integer, value text)")
    (let ((gc-cons-threshold 32768))
      (dotimes (n 40)
        (sqlite-execute db "insert into retained values (?, ?)" (list n "保留"))
        (garbage-collect)))
    (let ((statement (sqlite-select db "select n, value from retained order by n" nil 'set)))
      (unwind-protect
          (progn
            (dotimes (n 40)
              (garbage-collect)
              (should (equal (sqlite-next statement) (list n "保留"))))
            (should-not (sqlite-next statement)))
        (sqlite-finalize statement)))))

(ert-deftest fe-sqlite-batch-and-pragma ()
  (fe-sqlite--with-db db
    (sqlite-execute-batch db "create table batched (n integer); insert into batched values (1); insert into batched values (2);")
    (sqlite-pragma db "foreign_keys = ON")
    (should (equal (sqlite-select db "pragma foreign_keys") '((1))))
    (should (equal (sqlite-select db "select n from batched order by n" nil 'full)
                   '(("n") (1) (2))))))
