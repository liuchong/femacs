;;; stable-source-context-path-tests.el --- protect source lookup -*- lexical-binding: t; -*-

(require 'ert)

(ert-deftest ssc01-context-before-physical-product ()
  (should
   (equal (stable-suite--source-context-load-path
           '("product/lisp" "site") "product/lisp" "context/lisp")
          '("context/lisp/" "context/lisp/" "site" "product/lisp"))))

(ert-deftest ssc02-subdirectory-and-original-spelling ()
  (should
   (equal (stable-suite--source-context-load-path
           '("product/lisp/" "product/lisp/emacs-lisp")
           "product/lisp/" "context/lisp/")
          '("context/lisp/" "context/lisp/" "context/lisp/emacs-lisp/"
            "product/lisp/" "product/lisp/emacs-lisp"))))

(ert-deftest ssc03-prefix-boundary ()
  (should
   (equal (stable-suite--source-context-load-path
           '("product/lisp-extra" "other/product/lisp")
           "product/lisp" "context/lisp")
          '("context/lisp/" "product/lisp-extra" "other/product/lisp"))))

(ert-deftest ssc04-unrelated-and-nil-entries ()
  (should
   (equal (stable-suite--source-context-load-path
           '("site" nil "product/lisp" "extra") "product/lisp" "context/lisp")
          '("context/lisp/" "site" nil "context/lisp/" "extra" "product/lisp"))))

(ert-deftest ssc05-input-list-is-preserved ()
  (let* ((paths (list "product/lisp" nil "site"))
         (before (copy-tree paths)))
    (stable-suite--source-context-load-path paths "product/lisp" "context/lisp")
    (should (equal paths before))))

(ert-deftest ssc06-empty-load-path ()
  (should (equal (stable-suite--source-context-load-path nil "product/lisp" "context/lisp")
                 '("context/lisp/"))))

;;; stable-source-context-path-tests.el ends here
