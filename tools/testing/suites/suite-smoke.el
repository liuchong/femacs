;;; suite-smoke.el --- Fast native build/bundle smoke checks -*- lexical-binding: t; -*-

;;; Commentary:
;; Native Elisp & shell/codesign smoke suite replacing ad-hoc post-cleanup
;; Python snippets with parallel execution and exact manifest checks.

;;; Code:

(require 'fe-test-dsl)

(let* ((targets (fe-test-targets))
       (term-emacs (plist-get targets :term-emacs))
       (term-client (plist-get targets :term-client))
       (cocoa-app (plist-get targets :cocoa-app))
       (cocoa-staged (plist-get targets :cocoa-staged))
       (cocoa-bin (plist-get targets :cocoa-bin))
       (cocoa-client (plist-get targets :cocoa-client))
       (lisp-host (plist-get targets :lisp-host))
       (batch-expr
        (concat
         "(prin1 (list (+ 20 22)"
         " (treesit-available-p)"
         " (and (require (quote treesit)) (featurep (quote treesit)))"
         " (featurep (quote x-windows))"
         " (string= (secure-hash (quote sha256) \"abc\")"
         "          \"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\")))")))

  (when (and cocoa-app cocoa-staged)
    (fe-test-register-case
     :id "smoke/app-parity"
     :suite "smoke"
     :title "Staged Femacs.app matches installed /Applications/Fe.app"
     :command (list "diff" "-rq" "dist/macos/Femacs.app" cocoa-app)
     :timeout-ms 30000
     :cacheable nil
     :tags '("smoke" "fast")))

  (when cocoa-app
    (fe-test-register-case
     :id "smoke/app-codesign"
     :suite "smoke"
     :title "Strict deep codesign verification on /Applications/Fe.app"
     :command (list "codesign" "--verify" "--deep" "--strict" cocoa-app)
     :timeout-ms 30000
     :cacheable nil
     :tags '("smoke" "fast")))

  (when term-emacs
    (let ((term-inputs (fe-test-product-runtime-inputs term-emacs)))
      (fe-test-register-case
       :id "smoke/terminal-batch"
       :suite "smoke"
       :title "Terminal binary batch Lisp, treesit & SHA-256 smoke"
       :command (list term-emacs "--batch" "-Q" "--eval" batch-expr)
       :timeout-ms 20000
       :cacheable (not (null term-inputs))
       :cache-inputs term-inputs
       :check-fn (lambda (exec)
                   (let ((out (string-trim (or (alist-get 'stdout exec) ""))))
                     (if (and (eql (alist-get 'exit_code exec) 0)
                              (string= out "(42 t t nil t)"))
                         (list t out)
                       (list nil (format "unexpected output=%S exit=%s"
                                         out (alist-get 'exit_code exec))))))
       :tags '("smoke" "fast"))))

  (when cocoa-bin
    (let ((cocoa-inputs (fe-test-product-runtime-inputs cocoa-bin)))
      (fe-test-register-case
       :id "smoke/cocoa-batch"
       :suite "smoke"
       :title "Cocoa binary batch Lisp, treesit & SHA-256 smoke"
       :command (list cocoa-bin "--batch" "-Q" "--eval" batch-expr)
       :timeout-ms 20000
       :cacheable (not (null cocoa-inputs))
       :cache-inputs cocoa-inputs
       :check-fn (lambda (exec)
                   (let ((out (string-trim (or (alist-get 'stdout exec) ""))))
                     (if (and (eql (alist-get 'exit_code exec) 0)
                              (string= out "(42 t t nil t)"))
                         (list t out)
                       (list nil (format "unexpected output=%S exit=%s"
                                         out (alist-get 'exit_code exec))))))
       :tags '("smoke" "fast"))))

  (when lisp-host
    (let ((lisp-inputs (fe-test-product-runtime-inputs lisp-host)))
      (fe-test-register-case
       :id "smoke/lisp-host-eval"
       :suite "smoke"
       :title "Standalone femacs-lisp eval smoke"
       :command (list lisp-host "--batch" "-Q" "--eval" "(princ (+ 20 22))")
       :timeout-ms 15000
       :cacheable (not (null lisp-inputs))
       :cache-inputs lisp-inputs
       :check-fn (lambda (exec)
                   (let ((out (string-trim (or (alist-get 'stdout exec) ""))))
                     (if (and (eql (alist-get 'exit_code exec) 0)
                              (string= out "42"))
                         (list t out)
                       (list nil (format "unexpected output=%S exit=%s"
                                         out (alist-get 'exit_code exec))))))
       :tags '("smoke" "fast")))

    (fe-test-register-case
     :id "smoke/lisp-manifest"
     :suite "smoke"
     :title "dist/lisp/manifest.json binary SHA-256 verification"
     :command (list lisp-host "--batch" "-Q" "-L" "tools/testing" "--eval"
                    "(progn (require (quote fe-test-dsl)) (fe-test-verify-stable-host (fe-test-find-xtask)) (princ \"MANIFEST_OK\"))")
     :timeout-ms 20000
     :cacheable nil
     :tags '("smoke" "fast")))

  (let ((client (or term-client cocoa-client)))
    (when client
      (fe-test-register-case
       :id "smoke/emacsclient-version"
       :suite "smoke"
       :title "Built emacsclient reports version 32.0.50"
       :command (list client "--version")
       :timeout-ms 10000
       :cache-inputs (list client)
       :check-fn (lambda (exec)
                   (let ((out (string-trim (or (alist-get 'stdout exec) ""))))
                     (if (and (eql (alist-get 'exit_code exec) 0)
                              (string-match-p "32\\.0\\.50" out))
                         (list t out)
                       (list nil (format "unexpected version=%S" out)))))
       :tags '("smoke" "fast"))))

  (when lisp-host
    (fe-test-register-case
     :id "smoke/no-pycache"
     :suite "smoke"
     :title "Ensure tools/__pycache__ is absent"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    "(when (file-exists-p \"tools/__pycache__\") (error \"tools/__pycache__ exists\"))")
     :timeout-ms 10000
     :cacheable nil
     :tags '("smoke" "fast"))))

;;; suite-smoke.el ends here
