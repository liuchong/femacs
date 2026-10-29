;;; treesit-integration.el --- Real parser/GC/dump integration -*- lexical-binding: t; -*-

(require 'ert)
(require 'treesit)

(ert-deftest treesit-integration-roots-and-dead-buffer ()
  "Live nodes and native queries retain their owners across collection."
  (should (treesit-language-available-p 'json))
  (let (node query)
    (with-temp-buffer
      (insert "[1,2]")
      (let ((parser (treesit-parser-create 'json)))
        (setq node (treesit-parser-root-node parser)
              query (treesit-query-compile 'json "(number) @n" t))
        (dotimes (_ 4)
          (garbage-collect)
          (should (equal (treesit-node-type node) "document"))
          (should (= 2 (length (treesit-query-capture node query)))))
        (should (treesit-query-eagerly-compiled-p query))))
    (garbage-collect)
    (should (treesit-node-p node))
    (should (treesit-compiled-query-p query))
    (should-error (treesit-node-type node) :type 'treesit-node-buffer-killed)))

(ert-deftest treesit-integration-deleted-parser-node-lifetime ()
  "Deleting a parser's list membership preserves a retained node's tree."
  (should (treesit-language-available-p 'json))
  (with-temp-buffer
    (insert "[7]")
    (let* ((parser (treesit-parser-create 'json))
           (node (treesit-parser-root-node parser)))
      (treesit-parser-delete parser)
      (garbage-collect)
      (should-not (treesit-parser-list))
      ;; The stable node guard checks timestamp/buffer, not list membership.
      (should (equal (treesit-node-type node) "document"))
      (should-not (treesit-node-check node 'live))
      (should-error (treesit-parser-root-node parser) :type 'treesit-parser-deleted))))

(ert-deftest treesit-integration-lazy-query-native-lifecycle ()
  "Compile and collect actual native cursors without changing query source."
  (should (treesit-language-available-p 'json))
  (dotimes (_ 12)
    (with-temp-buffer
      (insert "[3,4]")
      (let* ((query (treesit-query-compile 'json "(number) @n"))
             (parser (treesit-parser-create 'json))
             (node (treesit-parser-root-node parser)))
        (should-not (treesit-query-eagerly-compiled-p query))
        (garbage-collect)
        (should (= 2 (length (treesit-query-capture node query))))
        (should (treesit-query-eagerly-compiled-p query))
        (should (equal (treesit-query-source query) "(number) @n"))))
    (garbage-collect)))

(defvar femacs-treesit-dump-query nil)

(defun femacs-treesit-dump-path ()
  (let ((root (getenv "FEMACS_RUNTIME_TEST_ROOT")))
    (unless (and root (file-directory-p root))
      (error "Existing owned dump directory required"))
    (expand-file-name "treesit-image.pdmp" root)))

(defun femacs-treesit-dump-prepare ()
  "Create an image containing an actual eagerly compiled query."
  (when (file-exists-p (femacs-treesit-dump-path))
    (error "Fresh dump output required"))
  (setq femacs-treesit-dump-query (treesit-query-compile 'json "(number) @n" t))
  (unless (treesit-query-eagerly-compiled-p femacs-treesit-dump-query)
    (error "Query did not eagerly compile before dumping"))
  (garbage-collect)
  (dump-emacs-portable (femacs-treesit-dump-path)))

(defun femacs-treesit-dump-assert ()
  "A new process must lazily rebuild the query's nulled native handles."
  (unless (and (treesit-compiled-query-p femacs-treesit-dump-query)
               (not (treesit-query-eagerly-compiled-p femacs-treesit-dump-query))
               (eq (treesit-query-language femacs-treesit-dump-query) 'json)
               (equal (treesit-query-source femacs-treesit-dump-query) "(number) @n"))
    (error "Restored query retained native pointers or lost its Lisp roots"))
  (with-temp-buffer
    (insert "[9]")
    (let* ((parser (treesit-parser-create 'json))
           (node (treesit-parser-root-node parser)))
      (unless (= 1 (length (treesit-query-capture node femacs-treesit-dump-query)))
        (error "Restored query failed its real parser capture"))))
  (unless (treesit-query-eagerly-compiled-p femacs-treesit-dump-query)
    (error "Restored query did not rebuild native handles"))
  (require 'json)
  (with-temp-file (expand-file-name "treesit-dump-verified.json" (getenv "FEMACS_RUNTIME_TEST_ROOT"))
    (insert (json-encode '((restored . t) (recompiled . t) (captures . 1))))))

(provide 'treesit-integration)
