;;; module-user-pointer-protection.el --- module pointer contracts -*- lexical-binding: t; -*-

(require 'ert)
(require 'cl-lib)
(require 'cl-generic)
(require 'mod-test
         (expand-file-name "../test/src/emacs-module-resources/mod-test"
                           invocation-directory))

(cl-defgeneric femacs-module-user-pointer--kind (object))
(cl-defmethod femacs-module-user-pointer--kind ((_ t)) 'ordinary)
(cl-defmethod femacs-module-user-pointer--kind ((_ user-ptr)) 'user-ptr)

(ert-deftest module-user-pointer/native-object-and-generic-dispatch ()
  "Classify a real C-module pointer and dispatch through its built-in class."
  (let ((pointer (mod-test-userptr-make 42)))
    (should (eq (user-ptrp pointer) t))
    (should (eq (type-of pointer) 'user-ptr))
    (should (= (mod-test-userptr-get pointer) 42))
    (should (eq (femacs-module-user-pointer--kind pointer) 'user-ptr))))

(ert-deftest module-user-pointer/other-objects-and-arity ()
  "Reject ordinary objects without dereferencing them as module pointers."
  (dolist (object (list nil t 0 1.5 'ordinary "text" [1 2] '(a . b)
                        #'identity))
    (should-not (user-ptrp object))
    (should (eq (femacs-module-user-pointer--kind object) 'ordinary)))
  (should-error (user-ptrp) :type 'wrong-number-of-arguments)
  (should-error (user-ptrp nil nil) :type 'wrong-number-of-arguments))

(provide 'module-user-pointer-protection)
;;; module-user-pointer-protection.el ends here
