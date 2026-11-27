;;; suite-static.el --- Parallel cached static & gate suite -*- lexical-binding: t; -*-

;;; Commentary:
;; Migrates high-frequency static boundary, own-voice, bootstrap, modernization,
;; ledger-tick, and strict Elisp byte-compilation gates into `fe-test'.

;;; Code:

(require 'fe-test-dsl)

(let ((py-env '(("PYTHONDONTWRITEBYTECODE" . "1"))))
  (fe-test-register-case
   :id "static/own-voice"
   :suite "static"
   :title "No bare reference product name in owned prose"
   :command '("python3" "tools/check-own-voice.py")
   :env py-env
   :timeout-ms 30000
   :cache-inputs '("tools/check-own-voice.py" "tools/testing" "xtask/src" "src" "lib" "lib-src" "platform")
   :tags '("static" "fast"))

  (fe-test-register-case
   :id "static/unsafe-boundary"
   :suite "static"
   :title "Unsafe boundary manifest & forbid metrics gate"
   :command '("python3" "tools/check-unsafe-boundary.py"
              "--manifest" "tools/unsafe-boundary-manifest.json"
              "--require-zero-static-mut"
              "--require-zero-active-stitching"
              "--require-100pct-safety-docs"
              "--require-no-file-allows"
              "--min-forbid-files" "150")
   :env py-env
   :timeout-ms 45000
   :cache-inputs '("tools/check-unsafe-boundary.py"
                   "tools/unsafe-boundary-manifest.json"
                   "Cargo.toml"
                   "crates"
                   "xtask/src"
                   "src"
                   "lib"
                   "lib-src")
   :tags '("static" "fast"))

  (fe-test-register-case
   :id "static/repository-bootstrap"
   :suite "static"
   :title "Repository layout, ignore rules & build-support descriptions"
   :command '("python3" "tools/test-repository-bootstrap.py")
   :env py-env
   :timeout-ms 30000
   :cache-inputs '("tools/test-repository-bootstrap.py"
                   "tools/check-cli-options.py"
                   ".gitignore"
                   "Cargo.toml"
                   "build-support")
   :tags '("static" "fast"))

  (fe-test-register-case
   :id "static/tooling-modernization"
   :suite "static"
   :title "Tooling inventory SHA-256 & G4 contract suite"
   :command '("python3" "tools/test-tooling-modernization.py")
   :env py-env
   :timeout-ms 45000
   :cache-inputs '("tools/test-tooling-modernization.py"
                   "maintenance/tooling-inventory.json"
                   "tools")
   :tags '("static" "fast"))

  (fe-test-register-case
   :id "static/ledger-ticks"
   :suite "static"
   :title "Porting ledger symbol ticks consistency"
   :command '("python3" "tools/check-ledger-ticks.py")
   :env py-env
   :timeout-ms 30000
   :cache-inputs '("tools/check-ledger-ticks.py" "docs/ports" "docs/port-map.md" "lib" "src")
   :tags '("static" "fast")))

(let ((lisp-host (plist-get (fe-test-targets) :lisp-host)))
  (when lisp-host
    (fe-test-register-case
     :id "static/fe-test-byte-compile"
     :suite "static"
     :title "Strict zero-warning byte-compile of tools/testing/**/*.el"
     :command (list lisp-host "--batch" "-Q"
                    "-L" "tools/testing"
                    "-L" "tools/testing/suites"
                    "--eval"
                    (concat
                     "(progn (require (quote bytecomp))"
                     " (let* ((byte-compile-error-on-warn t)"
                     "        (out (make-temp-file \"fe-bc-\" t))"
                     "        (files (append"
                     "                (directory-files \"tools/testing\" t \"\\\\.el\\\\'\")"
                     "                (directory-files \"tools/testing/suites\" t \"\\\\.el\\\\'\")"
                     "                (when (file-directory-p \"tools/testing/tests\")"
                     "                  (directory-files \"tools/testing/tests\" t \"\\\\.el\\\\'\")))))"
                     "   (unwind-protect"
                     "       (dolist (f files)"
                     "         (let* ((dest (expand-file-name (concat (file-name-nondirectory f) \"c\") out))"
                     "                (byte-compile-dest-file-function (lambda (_) dest)))"
                     "           (unless (eq (byte-compile-file f) t)"
                     "             (error \"Byte-compile failed or warned for %s\" f))))"
                     "     (delete-directory out t))))"))
     :env (list (cons "TMPDIR" (fe-test-path "build/test-framework/tmp")))
     :timeout-ms 45000
     :cache-inputs '("tools/testing")
     :tags '("static" "fast"))))

;;; suite-static.el ends here
