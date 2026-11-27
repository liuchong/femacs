;;; fe-test-dsl.el --- Shared data model and target helpers for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Core case definition, registry, repository/binary resolution, JSON I/O, and
;; git changed-file helpers for the native Lisp test framework (`fe-test`).

;;; Code:

(require 'cl-lib)
(require 'json)

(cl-defstruct (fe-test-case
               (:constructor fe-test-make-case))
  (id nil :type string)
  (suite nil :type string)
  (title "" :type string)
  (command nil :type list)
  (cwd nil)
  (env nil :type list)
  (timeout-ms 60000 :type integer)
  (memory-limit-bytes 0 :type integer)
  (heavy nil :type boolean)
  (pty nil :type boolean)
  (cacheable t :type boolean)
  (cache-inputs nil :type list)
  (tags nil :type list)
  (check-fn nil))

(defvar fe-test--registry nil
  "Registered `fe-test-case' entries in declaration order.")

(defvar fe-test-repo-root-override nil
  "Optional override for repository root discovery in tests.")

(defvar fe-test-xtask-override nil
  "Optional override for the supervisor/digest runner executable path.")

(defvar fe-test-exec-override nil
  "Optional override for the `elisp'/`xtask' exec runner executable path.")

(defvar fe-test-state-dir-override nil
  "Optional override for persistent state storage directory.")

(defconst fe-test-hermetic-editor-env
  '(("EMACSLOADPATH" . nil)
    ("EMACSDATA" . nil)
    ("EMACSDOC" . nil)
    ("EMACSPATH" . nil))
  "Environment overrides scrubbing external resource search-path variables.")

(defun fe-test-clear-registry ()
  "Reset all registered test cases."
  (setq fe-test--registry nil))

(defun fe-test-registered-cases ()
  "Return a copy of all registered `fe-test-case' objects."
  (copy-sequence fe-test--registry))

(defun fe-test-register-case (&rest plist)
  "Create and register a `fe-test-case' from PLIST."
  (let* ((tc (apply #'fe-test-make-case plist))
         (id (fe-test-case-id tc))
         (suite (fe-test-case-suite tc)))
    (unless (and (stringp id) (> (length id) 0))
      (error "fe-test-register-case requires non-empty :id"))
    (unless (and (stringp suite) (> (length suite) 0))
      (error "fe-test-register-case requires non-empty :suite for %s" id))
    (unless (and (listp (fe-test-case-command tc)) (fe-test-case-command tc))
      (error "fe-test-register-case requires non-empty :command for %s" id))
    (when (cl-find id fe-test--registry :key #'fe-test-case-id :test #'string=)
      (error "Duplicate fe-test case id: %s" id))
    (setq fe-test--registry (nconc fe-test--registry (list tc)))
    tc))

(defun fe-test-repo-root ()
  "Return the absolute repository root directory (without trailing slash)."
  (or fe-test-repo-root-override
      (let* ((this-file (or load-file-name
                            (and (boundp 'byte-compile-current-file)
                                 byte-compile-current-file)
                            (locate-library "fe-test-dsl")))
             (probe (if this-file
                        (file-name-directory
                         (directory-file-name
                          (file-name-directory
                           (directory-file-name (file-name-directory this-file)))))
                      default-directory)))
        (directory-file-name
         (expand-file-name
          (if (file-exists-p (expand-file-name "Cargo.toml" probe))
              probe
            default-directory))))))

(defun fe-test-path (rel)
  "Resolve repository-relative path REL into an absolute path."
  (expand-file-name rel (fe-test-repo-root)))

(defun fe-test-state-dir (&optional override)
  "Return persistent state directory for profiles, failures, cache, and reports."
  (directory-file-name
   (expand-file-name
    (or override fe-test-state-dir-override (fe-test-path "build/test-framework")))))

(defun fe-test--xtask-binary-p (program)
  "Return non-nil when PROGRAM invokes legacy `xtask test-exec ...' subcommands."
  (and (stringp program)
       (string-match-p "\\`xtask\\(\\.exe\\)?\\'"
                       (file-name-nondirectory program))))

(defun fe-test--exec-subcommand (runner-bin)
  "Return top-level exec subcommand (`\"test-exec\"' or `\"exec\"') for RUNNER-BIN."
  (if (fe-test--xtask-binary-p runner-bin)
      "test-exec"
    "exec"))

(defun fe-test--exec-argv (runner-bin &rest args)
  "Build RUNNER-BIN subcommand argv prefixing `test-exec' or `exec' before ARGS."
  (cons (fe-test--exec-subcommand runner-bin) args))

(defun fe-test-find-exec-runner (&optional override)
  "Locate the built `elisp' or `xtask' exec runner."
  (let* ((self-exe (when (and (boundp 'invocation-directory)
                              (boundp 'invocation-name)
                              (stringp invocation-directory)
                              (stringp invocation-name)
                              (string-match-p "\\`elisp\\(\\.exe\\)?\\'"
                                              (file-name-nondirectory invocation-name)))
                     (expand-file-name invocation-name invocation-directory)))
         (cand (or override
                   fe-test-exec-override
                   fe-test-xtask-override
                   (cl-find-if
                    (lambda (p) (and p (file-executable-p p)))
                    (list self-exe
                          (fe-test-path "build/cargo/release/elisp")
                          (fe-test-path "build/cargo/debug/elisp")
                          (fe-test-path "build/cargo/release/xtask")
                          (fe-test-path "build/cargo/debug/xtask")
                          (executable-find "elisp"))))))
    (unless (and cand (file-executable-p cand))
      (error "No exec runner found; build `elisp' with `cargo build -p elisp --target-dir build/cargo'"))
    (expand-file-name cand)))

(defun fe-test-find-xtask (&optional override)
  "Locate the exec runner binary (`elisp' or `xtask')."
  (fe-test-find-exec-runner override))

(defun fe-test-targets ()
  "Discover built target binaries in the workspace and host environment."
  (let* ((term-emacs (fe-test-path "build/terminal/port/src/emacs"))
         (term-client (fe-test-path "build/terminal/port/lib-src/emacsclient"))
         (cocoa-app "/Applications/Fe.app")
         (cocoa-staged (fe-test-path "dist/macos/Femacs.app"))
         (cocoa-bin "/Applications/Fe.app/Contents/MacOS/Femacs")
         (cocoa-client "/Applications/Fe.app/Contents/MacOS/bin/emacsclient")
         (staged-bin (fe-test-path "dist/macos/Femacs.app/Contents/MacOS/Femacs"))
         (lisp-host (fe-test-path "dist/lisp/bin/femacs-lisp"))
         (lisp-pdmp (fe-test-path "dist/lisp/bin/femacs-lisp.pdmp"))
         (http-host (fe-test-path "dist/lisp/bin/http-host"))
         (env-editor (getenv "FEMACS_TEST_EDITOR"))
         (editor (cond
                  ((and env-editor (file-executable-p env-editor)) env-editor)
                  ((file-executable-p term-emacs) term-emacs)
                  ((file-executable-p cocoa-bin) cocoa-bin)
                  ((file-executable-p staged-bin) staged-bin)
                  (t nil))))
    (list :term-emacs (and (file-executable-p term-emacs) term-emacs)
          :term-client (and (file-executable-p term-client) term-client)
          :cocoa-app (and (file-directory-p cocoa-app) cocoa-app)
          :cocoa-staged (and (file-directory-p cocoa-staged) cocoa-staged)
          :cocoa-bin (and (file-executable-p cocoa-bin) cocoa-bin)
          :cocoa-client (and (file-executable-p cocoa-client) cocoa-client)
          :lisp-host (and (file-executable-p lisp-host) lisp-host)
          :lisp-pdmp (and (file-regular-p lisp-pdmp) lisp-pdmp)
          :http-host (and (file-executable-p http-host) http-host)
          :editor editor)))

(defun fe-test-product-runtime-inputs (binary)
  "Return the full runtime input closure for product BINARY, or nil if unknown.
Includes the executable, its dumped `.pdmp' image, `lisp/', `etc/', and bundled
runtime shared-library directories when verified present on disk."
  (when (and (stringp binary) (file-executable-p binary))
    (let* ((abs (expand-file-name binary (fe-test-repo-root)))
           (dir (directory-file-name (file-name-directory abs))))
      (cond
       ;; macOS .app bundle: .../Contents/MacOS/<exe>
       ((string= (file-name-nondirectory dir) "MacOS")
        (let* ((contents (directory-file-name (file-name-directory dir)))
               (res (expand-file-name "Resources" contents))
               (pdmp (expand-file-name "libexec/Femacs.pdmp" res))
               (lisp-dir (expand-file-name "lisp" res))
               (etc-dir (expand-file-name "etc" res))
               (fw-dir (expand-file-name "Frameworks" contents)))
          (when (and (file-regular-p pdmp)
                     (file-directory-p lisp-dir)
                     (file-directory-p etc-dir))
            (cl-remove-if-not
             #'file-exists-p
             (list abs pdmp lisp-dir etc-dir fw-dir)))))
       ;; Standalone dist/lisp host: .../bin/femacs-lisp
       ((and (string= (file-name-nondirectory abs) "femacs-lisp")
             (string= (file-name-nondirectory dir) "bin"))
        (let* ((root (directory-file-name (file-name-directory dir)))
               (pdmp (expand-file-name "femacs-lisp.pdmp" dir))
               (lisp-dir (expand-file-name "lisp" root))
               (etc-dir (expand-file-name "etc" root)))
          (when (and (file-regular-p pdmp)
                     (file-directory-p lisp-dir)
                     (file-directory-p etc-dir))
            (cl-remove-if-not
             #'file-exists-p
             (list abs pdmp lisp-dir etc-dir
                   (expand-file-name "runtime-image" root)
                   (expand-file-name "runtime-lcms" root)
                   (expand-file-name "runtime-tls" root)
                   (expand-file-name "runtime-treesit" root))))))
       ;; Repo terminal build: .../port/src/emacs
       ((and (string= (file-name-nondirectory abs) "emacs")
             (string= (file-name-nondirectory dir) "src"))
        (let* ((pdmp (expand-file-name "emacs.pdmp" dir))
               (repo-lisp (fe-test-path "lisp"))
               (repo-etc (fe-test-path "etc")))
          (when (and (file-regular-p pdmp)
                     (file-directory-p repo-lisp)
                     (file-directory-p repo-etc))
            (list abs pdmp repo-lisp repo-etc))))
       (t nil)))))

(defun fe-test-json-read-file (path)
  "Read JSON object from PATH as an alist, or nil if PATH does not exist."
  (when (file-regular-p path)
    (with-temp-buffer
      (insert-file-contents path)
      (json-parse-string (buffer-string)
                         :object-type 'alist
                         :array-type 'list
                         :null-object nil
                         :false-object nil))))

(defun fe-test-json-write-file (path data)
  "Atomically write Lisp object DATA encoded as JSON to PATH."
  (let* ((abs (expand-file-name path))
         (dir (file-name-directory abs))
         (tmp (format "%s.tmp.%d" abs (emacs-pid))))
    (make-directory dir t)
    (let ((coding-system-for-write 'utf-8-unix))
      (with-temp-file tmp
        (insert (json-encode data) "\n")))
    (rename-file tmp abs t)))

(defun fe-test-call-json (program args &optional cwd)
  "Run PROGRAM with ARGS synchronously and parse JSON stdout into an alist."
  (with-temp-buffer
    (let* ((default-directory (file-name-as-directory (or cwd (fe-test-repo-root))))
           (rc (apply #'call-process program nil (current-buffer) nil args))
           (out (buffer-string)))
      (unless (eql rc 0)
        (error "Command failed (exit %s): %s %s\n%s"
               rc program (mapconcat #'identity args " ") out))
      (json-parse-string out
                         :object-type 'alist
                         :array-type 'list
                         :null-object nil
                         :false-object nil))))

(defun fe-test-verify-stable-host (xtask-bin)
  "Verify cached `dist/lisp' binaries against `dist/lisp/manifest.json'."
  (let* ((manifest-path (fe-test-path "dist/lisp/manifest.json"))
         (manifest (fe-test-json-read-file manifest-path)))
    (unless manifest
      (error "Missing dist/lisp/manifest.json at %s" manifest-path))
    (let* ((binaries (alist-get 'binaries manifest))
           (lisp-bin (fe-test-path "dist/lisp/bin/femacs-lisp"))
           (lisp-pdmp (fe-test-path "dist/lisp/bin/femacs-lisp.pdmp"))
           (http-bin (fe-test-path "dist/lisp/bin/http-host"))
           (digest-res (fe-test-call-json
                        xtask-bin
                        (fe-test--exec-argv xtask-bin "digest" lisp-bin lisp-pdmp http-bin)))
           (entries (alist-get 'entries digest-res)))
      (dolist (pair `((femacs-lisp . ,lisp-bin)
                      (femacs-lisp.pdmp . ,lisp-pdmp)
                      (http-host . ,http-bin)))
        (let* ((sym (car pair))
               (full (cdr pair))
               (want (alist-get 'sha256 (alist-get sym binaries)))
               (got-row (cl-find full entries
                                 :key (lambda (row) (alist-get 'path row))
                                 :test #'string=))
               (got (and got-row (alist-get 'sha256 got-row))))
          (unless (and want got (string= want got))
            (error "Host verification failed for %s: expected %s got %s"
                   sym want got)))))))

(defun fe-test-git-changed-files ()
  "Return deduplicated repo-relative files modified in working tree or vs HEAD."
  (let ((root (fe-test-repo-root))
        (seen (make-hash-table :test #'equal))
        (out nil))
    (with-temp-buffer
      (let ((default-directory (file-name-as-directory root)))
        (when (eql 0 (call-process "git" nil (current-buffer) nil
                                   "status" "--porcelain" "--untracked-files=normal"))
          (goto-char (point-min))
          (while (not (eobp))
            (let ((line (buffer-substring-no-properties
                         (line-beginning-position) (line-end-position))))
              (when (>= (length line) 4)
                (let* ((raw (substring line 3))
                       (parts (split-string raw " -> " t)))
                  (dolist (p parts)
                    (unless (gethash p seen)
                      (puthash p t seen)
                      (push p out))))))
            (forward-line 1)))))
    (nreverse out)))

(defun fe-test-now-iso8601 ()
  "Return current UTC timestamp formatted as ISO-8601."
  (format-time-string "%Y-%m-%dT%H:%M:%SZ" nil t))

(defun fe-test-now-ms ()
  "Return monotonic/wall time in milliseconds as an integer."
  (truncate (* (float-time) 1000.0)))

(provide 'fe-test-dsl)
;;; fe-test-dsl.el ends here
