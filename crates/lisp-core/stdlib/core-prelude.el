;;; core-prelude.el --- General-purpose prelude for lisp-core -*- lexical-binding: t; -*-

;;; Commentary:
;; Runtime helpers for script execution (with shebang + lexical-binding),
;; byte-compilation, and ERT test execution without requiring editor UI files.

;;; Code:

(defconst lisp-core-version "0.0.1"
  "Version of the `lisp-core' general-purpose runtime.")

(defvar command-line-args-left nil
  "Trailing CLI arguments passed to `--script', `run', `-f', or `-e'.")

(defun command-line--load-script (file)
  "Load FILE as a lexical-binding Lisp script, stripping any `#!' shebang line."
  (load-with-code-conversion
   file file nil t
   (lambda (buffer file)
     (with-current-buffer buffer
       (setq-local lexical-binding t)
       (goto-char (point-min))
       (when (looking-at "#!")
         (delete-region (point) (progn (forward-line 1) (point)))
         (insert ";; -*- lexical-binding: t -*-\n"))
       (eval-buffer buffer nil file nil t)))))

(defun lisp-core--compile-files (strict &rest files)
  "Byte-compile FILES, signaling an error if compilation fails or warns under STRICT."
  (require 'bytecomp)
  (let ((byte-compile-error-on-warn strict)
        (failures nil))
    (dolist (file files)
      (unless (eq (byte-compile-file file) t)
        (push file failures)))
    (when failures
      (error "Byte-compilation failed for: %s"
             (mapconcat #'identity (nreverse failures) ", ")))
    t))

(defun lisp-core--run-ert-files (&rest files)
  "Load test FILES (if provided) and execute all registered ERT tests in batch."
  (require 'ert)
  (dolist (file files)
    (load (expand-file-name file) nil t))
  (ert-run-tests-batch-and-exit t))

;; Convenient autoloads for core general-purpose modules.
(dolist (spec '((cl-lib . "emacs-lisp/cl-lib")
                (ert . "emacs-lisp/ert")
                (ert-x . "emacs-lisp/ert-x")
                (bytecomp . "emacs-lisp/bytecomp")
                (rx . "emacs-lisp/rx")
                (pp . "emacs-lisp/pp")
                (json . "json")
                (xml . "xml")
                (dom . "dom")
                (eieio . "emacs-lisp/eieio")
                (ring . "emacs-lisp/ring")
                (avl-tree . "emacs-lisp/avl-tree")
                (radix-tree . "emacs-lisp/radix-tree")
                (generator . "emacs-lisp/generator")
                (thunk . "emacs-lisp/thunk")
                (benchmark . "emacs-lisp/benchmark")
                (std/chan . "std/chan")
                (std/sync . "std/sync")
                (std/task . "std/task")
                (std/http . "std/http")
                (std/kv . "std/kv")
                (std/db . "std/db")
                (std/json . "std/json")
                (std/toml . "std/toml")
                (std/csv . "std/csv")
                (std/yaml . "std/yaml")
                (std/url . "std/url")
                (std/base64 . "std/base64")
                (std/archive . "std/archive")
                (std/crypto . "std/crypto")
                (std/time . "std/time")
                (std/fs . "std/fs")
                (std/path . "std/path")
                (std/regex . "std/regex")
                (std/proc . "std/proc")
                (std/env . "std/env")
                (std/cli . "std/cli")
                (std/log . "std/log")))
  (let ((feat (car spec))
        (lib (cdr spec)))
    (unless (or (featurep feat) (assoc feat load-history))
      (put feat 'lisp-core-library lib))))

(provide 'core-prelude)
;;; core-prelude.el ends here
