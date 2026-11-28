;;; check-crate-boundaries.el --- Verify crate isolation between editor targets and standalone crates -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure FEmacs `elisp' replacement for the retired Python gate.
;; Checks:
;; 1. Standalone language profiles (`lisp-vm', `lisp-std', `lisp', `elisp')
;;    never transitively pull C build crates (`cc', `cmake', `pkg-config',
;;    `bindgen', `ring', `aws-lc-*', `openssl-sys', `rusqlite', `libsqlite3-sys',
;;    or any `*-sys' crate except `windows-sys').
;; 2. Editor/tooling crates (`femacs-host', `femacs-tools', `xtask', `femacs')
;;    never transitively pull standalone language batteries (`elisp', `lisp-std',
;;    `rustyline', `ureq', `rustls', `rustls-rustcrypto', `webpki-roots', `redb').

;;; Code:

(require 'cl-lib)
(require 'json)

(defconst fe-cb--forbidden-c-build-crates
  '("cc" "cmake" "pkg-config" "bindgen" "ring" "aws-lc-rs" "aws-lc-sys"
    "rusqlite" "libsqlite3-sys" "openssl-sys" "curl-sys" "libgit2-sys"
    "zstd-sys" "zlib-sys" "bzip2-sys" "lzma-sys" "libz-sys")
  "C/toolchain dependency crate names forbidden in standalone language crates.")

(defconst fe-cb--allowed-ffi-sys-crates
  '("windows-sys")
  "Allowed pure FFI binding `*-sys' crate names.")

(defconst fe-cb--forbidden-editor-batteries
  '("elisp" "lisp-std" "rustyline" "ureq" "rustls"
    "rustls-rustcrypto" "webpki-roots" "redb")
  "Standalone language battery crates forbidden in editor/tooling dependencies.")

(defconst fe-cb--standalone-profiles
  '(("lisp-vm")
    ("lisp-std")
    ("lisp")
    ("lisp" "--no-default-features" "--features" "vm")
    ("lisp" "--no-default-features" "--features" "vm,pkg")
    ("lisp" "--no-default-features")
    ("elisp")
    ("elisp" "--no-default-features"))
  "Standalone package + Cargo flag profiles checked for zero C dependencies.")

(defconst fe-cb--editor-packages
  '("femacs-host" "femacs-tools" "xtask" "femacs")
  "Editor/tooling packages checked for standalone battery isolation.")

(defun fe-cb--repo-root ()
  "Resolve repository root directory cleanly."
  (let* ((this-file (or load-file-name
                        (and (boundp 'byte-compile-current-file)
                             byte-compile-current-file)
                        (expand-file-name "tools/check-crate-boundaries.el"
                                          default-directory)))
         (tools-dir (directory-file-name (file-name-directory this-file)))
         (parent (directory-file-name (file-name-directory tools-dir))))
    (if (file-exists-p (expand-file-name "Cargo.toml" parent))
        parent
      (directory-file-name (expand-file-name default-directory)))))

(defun fe-cb--forbidden-c-dep-p (pkg-name)
  "Return non-nil when PKG-NAME is a forbidden C/toolchain dependency."
  (or (member pkg-name fe-cb--forbidden-c-build-crates)
      (and (string-suffix-p "-sys" pkg-name)
           (not (member pkg-name fe-cb--allowed-ffi-sys-crates)))))

(defun fe-cb--run-cargo (repo args)
  "Run `cargo' in REPO with ARGS and return stdout string or signal an error."
  (with-temp-buffer
    (let* ((default-directory (file-name-as-directory repo))
           (rc (apply #'call-process "cargo" nil (current-buffer) nil args))
           (out (buffer-string)))
      (unless (eql rc 0)
        (error "`cargo %s` failed (rc=%s):\n%s"
               (mapconcat #'identity args " ")
               rc
               (string-trim out)))
      out)))

(defun fe-cb--workspace-package-names (repo)
  "Return list of workspace package names in REPO."
  (let* ((raw (fe-cb--run-cargo repo '("metadata" "--no-deps" "--format-version" "1")))
         (parsed (json-parse-string raw :object-type 'alist :array-type 'list))
         (pkgs (alist-get 'packages parsed)))
    (mapcar (lambda (pkg) (alist-get 'name pkg)) pkgs)))

(defun fe-cb--resolve-tree (repo pkg extra-args)
  "Return deduplicated list of crate names resolved for PKG with EXTRA-ARGS."
  (let* ((args (append (list "tree" "-p" pkg "-e" "normal,build" "--prefix" "none")
                       extra-args))
         (out (fe-cb--run-cargo repo args))
         (seen (make-hash-table :test #'equal))
         (names nil))
    (dolist (line (split-string out "\n" t))
      (let* ((trimmed (string-trim-left line))
             (parts (split-string trimmed " " t))
             (tok (car parts)))
        (when (and tok (> (length tok) 0) (not (gethash tok seen)))
          (puthash tok t seen)
          (push tok names))))
    (nreverse names)))

(defun fe-cb--evaluate-rules (existing-names standalone-fn editor-fn)
  "Evaluate standalone & editor isolation rules using resolver callbacks.
Returns a list of error strings (nil when all checks pass)."
  (let ((failures nil))
    (dolist (profile fe-cb--standalone-profiles)
      (let ((pkg (car profile))
            (extra (cdr profile)))
        (if (not (member pkg existing-names))
            (push (format "missing required standalone package: %s" pkg) failures)
          (let* ((deps (funcall standalone-fn pkg extra))
                 (bad-c (sort (cl-remove-if-not #'fe-cb--forbidden-c-dep-p deps)
                              #'string<)))
            (when bad-c
              (let ((label (string-trim (mapconcat #'identity profile " "))))
                (push (format "%s pulled C/build dependency(s): %s"
                              label
                              (mapconcat #'identity bad-c ", "))
                      failures)))))))
    (dolist (pkg fe-cb--editor-packages)
      (when (member pkg existing-names)
        (let* ((deps (funcall editor-fn pkg))
               (bad-bat (sort (cl-remove-if-not
                               (lambda (d) (member d fe-cb--forbidden-editor-batteries))
                               deps)
                              #'string<)))
          (when bad-bat
            (push (format "editor/tooling package `%s' pulled standalone language deps: %s"
                          pkg
                          (mapconcat #'identity bad-bat ", "))
                  failures)))))
    (nreverse failures)))

(defun fe-cb--self-test ()
  "Run isolated unit checks for crate-boundary classification and rule evaluation."
  (unless (and (fe-cb--forbidden-c-dep-p "cc")
               (fe-cb--forbidden-c-dep-p "openssl-sys")
               (fe-cb--forbidden-c-dep-p "custom-ffi-sys")
               (not (fe-cb--forbidden-c-dep-p "windows-sys"))
               (not (fe-cb--forbidden-c-dep-p "serde")))
    (error "self-test failed: fe-cb--forbidden-c-dep-p"))
  (let* ((all-pkgs '("lisp-vm" "lisp-std" "lisp" "elisp"
                     "femacs-host" "femacs-tools" "xtask" "femacs"))
         (clean-errors
          (fe-cb--evaluate-rules
           all-pkgs
           (lambda (_pkg _extra) '("core" "windows-sys" "serde" "redb"))
           (lambda (_pkg) '("core" "lisp-vm" "serde"))))
         (c-leak-errors
          (fe-cb--evaluate-rules
           all-pkgs
           (lambda (pkg _extra)
             (if (string= pkg "lisp-std") '("cc" "ring") '("serde")))
           (lambda (_pkg) '("serde"))))
         (battery-leak-errors
          (fe-cb--evaluate-rules
           all-pkgs
           (lambda (_pkg _extra) '("serde"))
           (lambda (pkg)
             (if (string= pkg "femacs-host") '("lisp-std" "ureq") '("serde"))))))
    (when clean-errors
      (error "self-test failed on clean graph: %S" clean-errors))
    (unless (= (length c-leak-errors) 1)
      (error "self-test failed to flag C leak cleanly: %S" c-leak-errors))
    (unless (and (= (length battery-leak-errors) 1)
                 (string-match-p "femacs-host" (car battery-leak-errors)))
      (error "self-test failed to flag battery leak cleanly: %S" battery-leak-errors)))
  (princ "check-crate-boundaries --self-test: ok (3/3)\n"))

(defun fe-cb-main (&optional argv)
  "Run crate-boundary verification or `--self-test' based on ARGV."
  (let ((args (or argv command-line-args-left)))
    (if (member "--self-test" args)
        (fe-cb--self-test)
      (let* ((repo (fe-cb--repo-root))
             (existing (fe-cb--workspace-package-names repo))
             (failures
              (fe-cb--evaluate-rules
               existing
               (lambda (pkg extra) (fe-cb--resolve-tree repo pkg extra))
               (lambda (pkg) (fe-cb--resolve-tree repo pkg nil)))))
        (if failures
            (progn
              (dolist (item failures)
                (message "ERROR: %s" item))
              (kill-emacs 1))
          (princ (format "check-crate-boundaries: ok (checked standalone=%d, editor isolation verified)\n"
                         (length fe-cb--standalone-profiles))))))))

(fe-cb-main)

;;; check-crate-boundaries.el ends here
