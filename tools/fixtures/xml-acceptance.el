;;; xml-acceptance.el --- Real libxml2 functional boundaries -*- lexical-binding: t; -*-
(require 'ert)

(defun fe-xml--parse (text &optional html discard)
  (with-temp-buffer
    (insert text)
    (funcall (if html #'libxml-parse-html-region #'libxml-parse-xml-region)
             nil nil nil discard)))

(ert-deftest fe-xml-capability ()
  (should (libxml-available-p))
  (dolist (function '(libxml-parse-html-region libxml-parse-xml-region))
    (should (subrp (symbol-function function)))
    (should (equal (subr-arity (symbol-function function)) '(0 . 4)))))

(ert-deftest fe-xml-attributes-entities-unicode ()
  (should (equal (fe-xml--parse "<root a='x' b='y'>前&amp;<child/>后</root>")
                 '(root ((a . "x") (b . "y")) "前&" (child nil) "后"))))

(ert-deftest fe-xml-cdata ()
  (should (equal (fe-xml--parse "<x><![CDATA[a<b]]></x>") '(x nil "a<b"))))

(ert-deftest fe-xml-top-level-comments ()
  (should (equal (fe-xml--parse "<!--before--><root/><!--after-->")
                 '(top nil (comment nil "before") (root nil) (comment nil "after")))))

(ert-deftest fe-xml-discard-top-level-only ()
  (should (equal (fe-xml--parse "<!--before--><root><!--inside-->x</root><!--after-->" nil t)
                 '(root nil (comment nil "inside") "x"))))

(ert-deftest fe-xml-html-recovery ()
  (should (equal (fe-xml--parse "<html><body><p>one<p>two</body></html>" t)
                 '(html nil (body nil (p nil "one") (p nil "two"))))))

(ert-deftest fe-xml-malformed-and-empty ()
  (dolist (text '("" "<a><b></a>")) (should-not (fe-xml--parse text))))

(ert-deftest fe-xml-reversed-markers-and-gap ()
  (with-temp-buffer
    (insert "ignored|")
    (let ((start (point)))
      (insert "<x>你好</x>")
      (let ((end (point)))
        (insert "|ignored")
        (goto-char (+ start 3))
        (insert "!") (delete-char -1)
        (let ((before (buffer-string)))
          (should (equal (libxml-parse-xml-region (copy-marker end) (copy-marker start))
                         '(x nil "你好")))
          (should (equal before (buffer-string))))))))

(ert-deftest fe-xml-narrowed-default-region ()
  (with-temp-buffer
    (insert "invalid-prefix")
    (let ((start (point)))
      (insert "<x>中</x>")
      (let ((end (point)))
        (insert "invalid-suffix")
        (narrow-to-region start end)
        (should (equal (libxml-parse-xml-region) '(x nil "中")))))))

(ert-deftest fe-xml-base-url-type-error ()
  (with-temp-buffer
    (insert "<x/>")
    (should-error (libxml-parse-xml-region nil nil 17) :type 'wrong-type-argument)
    (should (equal (buffer-string) "<x/>"))))

(ert-deftest fe-xml-region-range-error ()
  (with-temp-buffer
    (insert "<x/>")
    (should-error (libxml-parse-xml-region (1+ (point-max)) nil) :type 'args-out-of-range)))

(ert-deftest fe-xml-retained-dom-and-gc ()
  (let ((gc-cons-threshold 32768) (saved nil)
        (expected '(root ((key . "值")) (leaf nil "你好") (comment nil "kept"))))
    (dotimes (_ 80)
      (push (fe-xml--parse "<root key='值'><leaf>你好</leaf><!--kept--></root>") saved)
      (garbage-collect))
    (should (= (length saved) 80))
    (dolist (dom saved) (should (equal dom expected)))))

(ert-deftest fe-xml-dom-file-roundtrip ()
  (let* ((root (or (getenv "FEMACS_XML_TEST_DIR") (error "Owned output directory required")))
         (path (expand-file-name "xml-dom.el" root))
         (dom (fe-xml--parse "<root a='值'>你好</root>")))
    (let ((coding-system-for-write 'utf-8-unix))
      (with-temp-file path (prin1 dom (current-buffer))))
    (with-temp-buffer
      (insert-file-contents path)
      (should (equal (read (current-buffer)) dom)))))
