;;; Metadata only: do not redefine original functions, guards or tests.
(require 'json)
(add-hook
 'after-load-functions
 (lambda (file)
   (let ((expected (getenv "FEMACS_ORIGINAL_TEST_FILE")))
     (when (and expected (stringp file)
                (equal (file-truename file) (file-truename expected)))
       (let* ((base (file-name-base file))
              (helper (cond ((equal base "eval-tests") 'eval-tests-33014-func)
                            ((equal base "data-tests") 'data-tests-popcnt)))
              (definition (and helper (fboundp helper) (symbol-function helper)))
              (coding-system-for-write 'utf-8-unix))
         (with-temp-file (getenv "FEMACS_ORIGINAL_MODE_RECORD")
           (insert
            (json-encode
             (list (cons "loaded_file" file)
                   (cons "logical_file" (getenv "STABLE_SUITE_FILE_ID"))
                   (cons "helper" (and helper (symbol-name helper)))
                   (cons "helper_defined" (if definition t :json-false))
                   (cons "helper_bytecode" (if (byte-code-function-p definition) t :json-false))))
            "\n")))))))
