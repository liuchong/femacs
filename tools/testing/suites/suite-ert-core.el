;;; suite-ert-core.el --- Parallel LPT-scheduled core ERT suites -*- lexical-binding: t; -*-

;;; Commentary:
;; Dispatches 20 high-frequency core ERT files (`test/src/*-tests.el` +
;; `test/lisp/sort-tests.el`) as independent parallel workers under `fe-test'.

;;; Code:

(require 'fe-test-dsl)

(let* ((targets (fe-test-targets))
       (editor (plist-get targets :editor))
       (ert-files '("test/src/buffer-tests.el"
                    "test/src/callint-tests.el"
                    "test/src/casefiddle-tests.el"
                    "test/src/character-tests.el"
                    "test/src/chartab-tests.el"
                    "test/src/cmds-tests.el"
                    "test/src/data-tests.el"
                    "test/src/doc-tests.el"
                    "test/src/editfns-tests.el"
                    "test/src/eval-tests.el"
                    "test/src/floatfns-tests.el"
                    "test/src/fns-tests.el"
                    "test/src/json-tests.el"
                    "test/src/keymap-tests.el"
                    "test/src/marker-tests.el"
                    "test/src/search-tests.el"
                    "test/src/syntax-tests.el"
                    "test/src/textprop-tests.el"
                    "test/src/undo-tests.el"
                    "test/lisp/sort-tests.el")))
  (when editor
    (let ((runtime-inputs (fe-test-product-runtime-inputs editor)))
      (dolist (rel ert-files)
        (when (file-regular-p (fe-test-path rel))
          (let* ((base (file-name-sans-extension (file-name-nondirectory rel)))
                 (id (concat "ert/" base)))
            (fe-test-register-case
             :id id
             :suite "ert"
             :title (format "Core ERT %s" rel)
             :command (list editor "--batch" "-Q"
                            "-l" "ert"
                            "-l" rel
                            "--eval" "(ert-run-tests-batch-and-exit '(not (tag :expensive-test)))")
             :env '(("LC_ALL" . "C"))
             :timeout-ms 120000
             :heavy (member base '("buffer-tests" "editfns-tests" "fns-tests" "eval-tests" "sort-tests"))
             :cacheable (not (null runtime-inputs))
             :cache-inputs (and runtime-inputs (append runtime-inputs (list rel)))
             :tags '("ert" "core"))))))))

;;; suite-ert-core.el ends here
