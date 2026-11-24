;;; femacs-eliscript-worker.el --- Opt-in Eliscript worker integration  -*- lexical-binding: t; -*-

;;; Commentary:
;; Strictly opt-in editor integration for Eliscript worker processes using the
;; frozen JS runtime, Lisp client closure, and installed Bun/Node runners.
;; Loading this library never edits user Lisp files or spawns any background
;; runtime until explicitly started by the caller.

;;; Code:

(require 'cl-lib)
(require 'json)
(require 'subr-x)

(defgroup femacs-eliscript nil
  "Opt-in Eliscript asynchronous worker integration."
  :group 'tools)

(defcustom femacs-eliscript-package-dir nil
  "Directory containing a frozen prebuilt Eliscript package."
  :type '(choice (const :tag "Unset" nil) directory)
  :group 'femacs-eliscript)

(defcustom femacs-eliscript-js-runner "bun"
  "JavaScript executable used for Eliscript workers."
  :type 'string
  :group 'femacs-eliscript)

(defcustom femacs-eliscript-startup-timeout 10.0
  "Seconds to wait for worker ready handshake."
  :type 'number
  :group 'femacs-eliscript)

(defcustom femacs-eliscript-analysis-buffer-name "*Eliscript Analysis*"
  "Default buffer name displaying asynchronous analysis results."
  :type 'string
  :group 'femacs-eliscript)

(define-error 'femacs-eliscript-error
  "FEmacs Eliscript worker error")
(define-error 'femacs-eliscript-missing-runtime-error
  "Required JavaScript runtime is not available"
  'femacs-eliscript-error)
(define-error 'femacs-eliscript-missing-resource-error
  "Required Eliscript worker resource is missing"
  'femacs-eliscript-error)
(define-error 'femacs-eliscript-capability-error
  "Eliscript worker does not support required capability"
  'femacs-eliscript-error)
(define-error 'femacs-eliscript-side-effect-unknown-error
  "Side-effectful Eliscript request completion is unknown"
  'femacs-eliscript-error)

(defvar eliscript-worker--project-directory)
(defvar eliscript-worker-startup-timeout)

(defconst femacs-eliscript--client-libraries
  '("eliscript-value-codec"
    "eliscript-value-stream"
    "eliscript-worker"
    "eliscript-service")
  "Required Eliscript Lisp client files in dependency order.")

(defconst femacs-eliscript--ambiguous-side-effect-codes
  '("timeout" "worker-exit" "worker-restart" "worker-stop"
    "client-protocol-error" "cancelled")
  "Error codes indicating unknown external effect state for side-effect calls.")

(defvar femacs-eliscript--loaded-client-dir nil
  "Directory from which the frozen Eliscript client closure was loaded.")

(defvar femacs-eliscript--worker-launch-config
  (make-hash-table :test #'eq :weakness 'key)
  "Per-worker launch configuration plist keyed by worker struct.")

(defvar femacs-eliscript--pending-launch-config nil
  "Dynamic launch configuration bound during `femacs-eliscript-start'.")

(cl-defstruct (femacs-eliscript-install
               (:constructor femacs-eliscript-install--create))
  package-dir
  client-dir
  worker-script
  runner-program
  runner-executable
  runner-version
  working-directory
  module-path
  project-manifest)

(cl-defstruct (femacs-eliscript-session
               (:constructor femacs-eliscript-session--create))
  install
  service
  worker
  module-path
  project-manifest
  module-digest
  output-buffer
  value-codec
  value-chunks
  operations
  closed)

(cl-defstruct (femacs-eliscript-request
               (:constructor femacs-eliscript-request--create))
  session
  service-request
  operation-name
  request-id
  worker-generation
  module-digest
  buffer
  buffer-tick
  side-effect
  completion-status
  done
  result
  error
  timing
  progress-events)

(defun femacs-eliscript--signal (error-symbol code message &optional details)
  "Signal ERROR-SYMBOL with structured CODE, MESSAGE, and optional DETAILS."
  (signal error-symbol
          (list message
                (append (list (cons 'code code)
                              (cons 'message message))
                        details))))

(defun femacs-eliscript--resolve-runner (runner)
  "Resolve RUNNER executable path or fail cleanly with typed error."
  (let ((candidate (or runner
                       femacs-eliscript-js-runner
                       (getenv "FEMACS_ELISCRIPT_JS_RUNNER")
                       "bun")))
    (unless (and (stringp candidate) (not (string-empty-p candidate)))
      (femacs-eliscript--signal
       'femacs-eliscript-missing-runtime-error
       "missing-js-runtime"
       "JavaScript runner must be a non-empty program name or path"
       `((runner . ,candidate))))
    (let ((resolved
           (if (or (file-name-absolute-p candidate)
                   (string-match-p "/" candidate))
               (let ((expanded (expand-file-name candidate)))
                 (when (and (file-regular-p expanded)
                            (file-executable-p expanded))
                   expanded))
             (executable-find candidate))))
      (unless resolved
        (femacs-eliscript--signal
         'femacs-eliscript-missing-runtime-error
         "missing-js-runtime"
         (format "JavaScript runtime not found or not executable: %s" candidate)
         `((runner . ,candidate))))
      resolved)))

(defun femacs-eliscript--runner-version (executable)
  "Return `--version` output for EXECUTABLE or \"unknown\"."
  (condition-case nil
      (with-temp-buffer
        (if (eql 0 (process-file executable nil t nil "--version"))
            (let ((text (string-trim (buffer-string))))
              (if (string-empty-p text) "unknown" text))
          "unknown"))
    (error "unknown")))

(defun femacs-eliscript--find-worker-script (package-dir)
  "Return worker entrypoint inside PACKAGE-DIR without triggering bootstrap."
  (let ((found
         (cl-find-if
          #'file-regular-p
          (list (expand-file-name "runtime/worker.mjs" package-dir)
                (expand-file-name "dist/runtime/worker.mjs" package-dir)))))
    (unless found
      (femacs-eliscript--signal
       'femacs-eliscript-missing-resource-error
       "missing-worker-resource"
       (format "Eliscript worker runtime script missing under %s" package-dir)
       `((package-dir . ,package-dir)
         (expected . ("runtime/worker.mjs" "dist/runtime/worker.mjs")))))
    found))

(defun femacs-eliscript--find-client-dir (package-dir)
  "Return directory holding all required worker client files in PACKAGE-DIR."
  (let* ((candidates
          (list (expand-file-name "tools/worker" package-dir)
                (expand-file-name "lisp/worker" package-dir)
                (expand-file-name "lisp" package-dir)))
         (matched
          (cl-find-if
           (lambda (dir)
             (and (file-directory-p dir)
                  (cl-every
                   (lambda (lib)
                     (file-regular-p
                      (expand-file-name (concat lib ".el") dir)))
                   femacs-eliscript--client-libraries)))
           candidates)))
    (unless matched
      (femacs-eliscript--signal
       'femacs-eliscript-missing-resource-error
       "missing-worker-resource"
       (format "Eliscript Lisp worker client files missing under %s" package-dir)
       `((package-dir . ,package-dir)
         (searched . ,candidates)
         (required . ,femacs-eliscript--client-libraries))))
    matched))

(defun femacs-eliscript--find-default-module (package-dir)
  "Return default compiled analysis module under PACKAGE-DIR if present."
  (cl-find-if
   #'file-regular-p
   (list (expand-file-name "analysis/analysis.mjs" package-dir)
         (expand-file-name "examples/emacs-analysis/analysis.mjs" package-dir)
         (expand-file-name "dist/examples/emacs-analysis/analysis.mjs" package-dir))))

(defun femacs-eliscript--find-default-manifest (package-dir module-path)
  "Return project manifest adjacent to MODULE-PATH or under PACKAGE-DIR."
  (let ((candidates
         (delq nil
               (list
                (and module-path
                     (expand-file-name "eliscript-project.json"
                                       (file-name-directory module-path)))
                (expand-file-name "analysis/eliscript-project.json" package-dir)))))
    (cl-find-if #'file-regular-p candidates)))

(cl-defun femacs-eliscript-resolve-install
    (&key package-dir runner working-directory module-path project-manifest)
  "Resolve and validate runner, worker script, client files, and module inputs.

Fails cleanly with `femacs-eliscript-missing-runtime-error' or
`femacs-eliscript-missing-resource-error' without falling back to source
bootstrap."
  (let* ((runner-name (or runner
                          femacs-eliscript-js-runner
                          (getenv "FEMACS_ELISCRIPT_JS_RUNNER")
                          "bun"))
         (runner-exec (femacs-eliscript--resolve-runner runner-name))
         (runner-ver (femacs-eliscript--runner-version runner-exec))
         (raw-pkg (or package-dir
                      femacs-eliscript-package-dir
                      (getenv "FEMACS_ELISCRIPT_PACKAGE_DIR"))))
    (unless (and (stringp raw-pkg) (not (string-empty-p raw-pkg)))
      (femacs-eliscript--signal
       'femacs-eliscript-missing-resource-error
       "missing-worker-resource"
       "Eliscript package directory is not configured"
       `((package-dir . ,raw-pkg))))
    (let ((pkg-dir (expand-file-name raw-pkg)))
      (unless (file-directory-p pkg-dir)
        (femacs-eliscript--signal
         'femacs-eliscript-missing-resource-error
         "missing-worker-resource"
         (format "Eliscript package directory does not exist: %s" pkg-dir)
         `((package-dir . ,pkg-dir))))
      (let* ((worker-script (femacs-eliscript--find-worker-script pkg-dir))
             (client-dir (femacs-eliscript--find-client-dir pkg-dir))
             (workdir (file-name-as-directory
                       (expand-file-name (or working-directory pkg-dir))))
             (resolved-module
              (if module-path
                  (let ((expanded (expand-file-name module-path)))
                    (unless (file-regular-p expanded)
                      (femacs-eliscript--signal
                       'femacs-eliscript-missing-resource-error
                       "missing-worker-resource"
                       (format "Eliscript module does not exist: %s" expanded)
                       `((path . ,expanded))))
                    expanded)
                (femacs-eliscript--find-default-module pkg-dir)))
             (resolved-manifest
              (cond
               (project-manifest
                (let ((expanded (expand-file-name project-manifest)))
                  (unless (file-regular-p expanded)
                    (femacs-eliscript--signal
                     'femacs-eliscript-missing-resource-error
                     "missing-worker-resource"
                     (format "Eliscript project manifest does not exist: %s"
                             expanded)
                     `((path . ,expanded))))
                  expanded))
               (t (femacs-eliscript--find-default-manifest
                   pkg-dir resolved-module)))))
        (unless (file-directory-p workdir)
          (femacs-eliscript--signal
           'femacs-eliscript-missing-resource-error
           "missing-worker-resource"
           (format "Working directory does not exist: %s" workdir)
           `((working-directory . ,workdir))))
        (femacs-eliscript-install--create
         :package-dir pkg-dir
         :client-dir client-dir
         :worker-script worker-script
         :runner-program runner-name
         :runner-executable runner-exec
         :runner-version runner-ver
         :working-directory workdir
         :module-path resolved-module
         :project-manifest resolved-manifest)))))

(defun femacs-eliscript--launch-with-session-config (orig-fn worker)
  "Bind per-session working directory and startup timeout around WORKER launch."
  (let ((cfg (or (gethash worker femacs-eliscript--worker-launch-config)
                 femacs-eliscript--pending-launch-config)))
    (if (not cfg)
        (funcall orig-fn worker)
      (puthash worker cfg femacs-eliscript--worker-launch-config)
      (let ((eliscript-worker--project-directory
             (plist-get cfg :working-directory))
            (eliscript-worker-startup-timeout
             (or (plist-get cfg :startup-timeout)
                 eliscript-worker-startup-timeout)))
        (funcall orig-fn worker)))))

(defun femacs-eliscript--load-client (install)
  "Load the frozen Eliscript client closure named by INSTALL without mutating globals."
  (let ((client-dir (femacs-eliscript-install-client-dir install)))
    (unless (and (equal femacs-eliscript--loaded-client-dir client-dir)
                 (featurep 'eliscript-service))
      (add-to-list 'load-path client-dir)
      (dolist (lib femacs-eliscript--client-libraries)
        (load (expand-file-name (concat lib ".el") client-dir) nil t t))
      (setq femacs-eliscript--loaded-client-dir client-dir))
    (unless (advice-member-p #'femacs-eliscript--launch-with-session-config
                             'eliscript-worker--launch)
      (advice-add 'eliscript-worker--launch :around
                  #'femacs-eliscript--launch-with-session-config))))

(defun femacs-eliscript--file-sha256 (path)
  "Return hex SHA-256 digest of file PATH."
  (with-temp-buffer
    (set-buffer-multibyte nil)
    (insert-file-contents-literally path)
    (secure-hash 'sha256 (current-buffer))))

(defun femacs-eliscript--tokenize-text (text)
  "Tokenize TEXT cleanly in Lisp matching the analysis reference rules."
  (let ((case-fold-search nil)
        tokens)
    (with-temp-buffer
      (insert (downcase text))
      (goto-char (point-min))
      (while (re-search-forward "[[:alnum:]_]+" nil t)
        (push (match-string-no-properties 0) tokens)))
    (vconcat (nreverse tokens))))

(defun femacs-eliscript-documents-from-buffer (&optional buffer id)
  "Build analysis document vector from BUFFER using ID or buffer name."
  (let ((target (or buffer (current-buffer))))
    (with-current-buffer target
      (vector
       `((id . ,(or id (buffer-name target)))
         (text . ,(buffer-substring-no-properties (point-min) (point-max))))))))

(defun femacs-eliscript--statistics-reference (documents)
  "Compute reference document statistics vector for DOCUMENTS."
  (let ((results (make-vector (length documents) nil)))
    (dotimes (i (length documents))
      (let* ((doc (aref documents i))
             (terms (femacs-eliscript--tokenize-text
                     (or (alist-get 'text doc) "")))
             (seen (make-hash-table :test #'equal)))
        (dotimes (j (length terms))
          (puthash (aref terms j) t seen))
        (aset results i
              `((id . ,(alist-get 'id doc))
                (terms . ,(length terms))
                (uniqueTerms . ,(hash-table-count seen))))))
    results))

(defun femacs-eliscript--search-reference (documents query)
  "Compute reference search score vector for DOCUMENTS and QUERY."
  (let ((qterms (femacs-eliscript--tokenize-text query))
        hits)
    (dotimes (i (length documents))
      (let* ((doc (aref documents i))
             (terms (femacs-eliscript--tokenize-text
                     (or (alist-get 'text doc) "")))
             (counts (make-hash-table :test #'equal))
             (matches 0))
        (dotimes (j (length terms))
          (let ((term (aref terms j)))
            (puthash term (1+ (gethash term counts 0)) counts)))
        (dotimes (j (length qterms))
          (setq matches (+ matches (gethash (aref qterms j) counts 0))))
        (when (> matches 0)
          (push `((id . ,(alist-get 'id doc))
                  (matches . ,matches)
                  (terms . ,(length terms)))
                hits))))
    (vconcat (nreverse hits))))

(defun femacs-eliscript--frequencies-reference (documents)
  "Compute reference sorted term-frequency alist for DOCUMENTS."
  (let ((counts (make-hash-table :test #'equal))
        entries)
    (dotimes (i (length documents))
      (let ((terms (femacs-eliscript--tokenize-text
                    (or (alist-get 'text (aref documents i)) ""))))
        (dotimes (j (length terms))
          (let ((term (aref terms j)))
            (puthash term (1+ (gethash term counts 0)) counts)))))
    (maphash (lambda (term count) (push (cons (intern term) count) entries))
             counts)
    (sort entries (lambda (a b)
                    (string-lessp (symbol-name (car a))
                                  (symbol-name (car b)))))))

(defun femacs-eliscript--frequency-equal (left right)
  "Order-independent equality check for term frequency alists LEFT and RIGHT."
  (and (= (length left) (length right))
       (cl-every (lambda (entry)
                   (equal (cdr entry) (alist-get (car entry) right)))
                 left)))

(defun femacs-eliscript--documents-size (arguments)
  "Compute total character length across DOCUMENTS in ARGUMENTS."
  (let ((docs (car arguments))
        (total 0))
    (dotimes (i (length docs))
      (setq total (+ total (length (or (alist-get 'text (aref docs i)) "")))))
    total))

(defun femacs-eliscript-analysis-operations (&optional threshold)
  "Return default buffer-analysis service operations with THRESHOLD."
  (let ((limit (or threshold 0)))
    (list
     (eliscript-service-operation
      'document-statistics #'femacs-eliscript--statistics-reference
      :export-name "document_statistics"
      :workload-size #'femacs-eliscript--documents-size
      :threshold limit)
     (eliscript-service-operation
      'search-documents #'femacs-eliscript--search-reference
      :export-name "search_documents"
      :workload-size #'femacs-eliscript--documents-size
      :threshold limit)
     (eliscript-service-operation
      'term-frequencies #'femacs-eliscript--frequencies-reference
      :export-name "term_frequencies"
      :workload-size #'femacs-eliscript--documents-size
      :threshold limit
      :equal #'femacs-eliscript--frequency-equal))))

(cl-defun femacs-eliscript-start
    (&key package-dir runner working-directory module-path project-manifest
          module-digest operations verify value-codec value-chunks output-buffer)
  "Start an opt-in Eliscript worker session and return `femacs-eliscript-session'."
  (let* ((install
          (femacs-eliscript-resolve-install
           :package-dir package-dir
           :runner runner
           :working-directory working-directory
           :module-path module-path
           :project-manifest project-manifest))
         (mod (femacs-eliscript-install-module-path install))
         (manifest (femacs-eliscript-install-project-manifest install)))
    (unless mod
      (femacs-eliscript--signal
       'femacs-eliscript-missing-resource-error
       "missing-worker-resource"
       "No Eliscript module path provided or discovered in package directory"
       `((package-dir . ,(femacs-eliscript-install-package-dir install)))))
    (femacs-eliscript--load-client install)
    (let* ((manifest-digest
            (and manifest (eliscript-worker--project-manifest-digest manifest)))
           (_ (when (and manifest-digest module-digest
                         (not (equal manifest-digest module-digest)))
                (femacs-eliscript--signal
                 'femacs-eliscript-error
                 "module-digest-conflict"
                 "Explicit module-digest conflicts with project-manifest digest"
                 `((module-digest . ,module-digest)
                   (manifest-digest . ,manifest-digest)
                   (project-manifest . ,manifest)
                   (module-path . ,mod)))))
           (resolved-digest
            (or manifest-digest
                module-digest
                (femacs-eliscript--file-sha256 mod)))
           (ops (or operations (femacs-eliscript-analysis-operations 0)))
           (declared
            (if manifest
                (eliscript-service-module-declare
                 mod
                 :project-manifest manifest
                 :module-digest resolved-digest)
              (eliscript-service-module-declare
               mod :module-digest resolved-digest)))
           (cmd (list (femacs-eliscript-install-runner-executable install)
                      (femacs-eliscript-install-worker-script install)))
           (femacs-eliscript--pending-launch-config
            (list :working-directory
                  (femacs-eliscript-install-working-directory install)
                  :startup-timeout femacs-eliscript-startup-timeout))
           (service
            (eliscript-service-start
             declared ops
             :command cmd
             :verify verify
             :value-codec value-codec
             :value-chunks value-chunks))
           (worker (eliscript-service-worker service)))
      (femacs-eliscript-session--create
       :install install
       :service service
       :worker worker
       :module-path mod
       :project-manifest manifest
       :module-digest resolved-digest
       :output-buffer (or output-buffer femacs-eliscript-analysis-buffer-name)
       :value-codec value-codec
       :value-chunks value-chunks
       :operations ops
       :closed nil))))

(defun femacs-eliscript-stop (session &optional force)
  "Stop SESSION and terminate its worker process."
  (when (and (femacs-eliscript-session-p session)
             (not (femacs-eliscript-session-closed session)))
    (setf (femacs-eliscript-session-closed session) t)
    (unwind-protect
        (eliscript-service-stop (femacs-eliscript-session-service session) force)
      (remhash (femacs-eliscript-session-worker session)
               femacs-eliscript--worker-launch-config))))

(defun femacs-eliscript-restart (session)
  "Restart SESSION's underlying worker generation."
  (unless (and (femacs-eliscript-session-p session)
               (not (femacs-eliscript-session-closed session)))
    (femacs-eliscript--signal
     'femacs-eliscript-error "session-closed" "Eliscript session is closed"))
  (eliscript-service-restart (femacs-eliscript-session-service session)))

(defun femacs-eliscript-status (session)
  "Return diagnostic alist for SESSION."
  (let* ((install (femacs-eliscript-session-install session))
         (service (femacs-eliscript-session-service session))
         (worker (femacs-eliscript-session-worker session))
         (cfg (gethash worker femacs-eliscript--worker-launch-config))
         (proc (eliscript-worker-process worker))
         (declared-digest
          (or (eliscript-service-module-digest
               (eliscript-service-module service))
              (femacs-eliscript-session-module-digest session))))
    `((runner-program . ,(femacs-eliscript-install-runner-program install))
      (runner-executable . ,(femacs-eliscript-install-runner-executable install))
      (runner-version . ,(femacs-eliscript-install-runner-version install))
      (package-dir . ,(femacs-eliscript-install-package-dir install))
      (client-dir . ,(femacs-eliscript-install-client-dir install))
      (worker-script . ,(femacs-eliscript-install-worker-script install))
      (working-directory . ,(or (plist-get cfg :working-directory)
                                (femacs-eliscript-install-working-directory install)))
      (pid . ,(and proc (process-id proc)))
      (live . ,(if (eliscript-worker-live-p worker) t nil))
      (state . ,(eliscript-worker-state worker))
      (generation . ,(eliscript-worker-generation worker))
      (restart-count . ,(eliscript-worker-restart-count worker))
      (capabilities . ,(copy-sequence (eliscript-worker-capabilities worker)))
      (module-path . ,(femacs-eliscript-session-module-path session))
      (project-manifest . ,(femacs-eliscript-session-project-manifest session))
      (module-digest . ,declared-digest)
      (last-exit . ,(eliscript-worker-last-exit worker))
      (stderr . ,(eliscript-worker-stderr worker)))))

(defun femacs-eliscript-require-capabilities
    (session required &optional capability-probes)
  "Verify SESSION exposes REQUIRED capabilities and passes optional CAPABILITY-PROBES.

Signals `femacs-eliscript-capability-error' with `code = \"unsupported-capability\"'
if any required capability is absent or fails its live probe."
  (let* ((status (femacs-eliscript-status session))
         (advertised (alist-get 'capabilities status))
         (missing
          (cl-remove-if (lambda (cap) (member cap advertised)) required)))
    (when missing
      (femacs-eliscript--signal
       'femacs-eliscript-capability-error
       "unsupported-capability"
       (format "Worker is missing required capability: %s"
               (string-join missing ", "))
       `((missing . ,missing)
         (advertised . ,advertised)
         (runner-program . ,(alist-get 'runner-program status)))))
    (dolist (probe capability-probes)
      (let* ((cap (car probe))
             (fn (cdr probe)))
        (when (and (member cap required) fn)
          (condition-case err
              (unless (funcall fn session)
                (femacs-eliscript--signal
                 'femacs-eliscript-capability-error
                 "unsupported-capability"
                 (format "Worker capability %s failed live verification" cap)
                 `((capability . ,cap)
                   (advertised . ,advertised)
                   (runner-program . ,(alist-get 'runner-program status)))))
            (femacs-eliscript-capability-error
             (signal (car err) (cdr err)))
            (error
             (femacs-eliscript--signal
              'femacs-eliscript-capability-error
              "unsupported-capability"
              (format "Worker capability %s failed live verification: %s"
                      cap (error-message-string err))
              `((capability . ,cap)
                (cause . ,(error-message-string err))
                (advertised . ,advertised)
                (runner-program . ,(alist-get 'runner-program status)))))))))
    t))

(defun femacs-eliscript--module-resolution-missing-p (err-obj)
  "Return non-nil when ERR-OBJ represents an explicit module-resolution failure."
  (let ((code (and (listp err-obj) (alist-get 'code err-obj)))
        (msg (and (listp err-obj) (alist-get 'message err-obj))))
    (and (stringp code)
         (or (equal code "ERR_MODULE_NOT_FOUND")
             (and (member code '("ResolveMessage" "MODULE_NOT_FOUND"))
                  (stringp msg)
                  (string-match-p
                   "Cannot find \\(?:package\\|module\\)" msg))))))

(defun femacs-eliscript-probe-runtime-resolution
    (session probe-module &optional probe-manifest export-name timeout-ms validator)
  "Probe whether SESSION resolves package imports from PROBE-MODULE at runtime.

PROBE-MODULE should be a compiled Eliscript module placed outside
`node_modules' ancestors when verifying `runtime-resolution'.  Optional
VALIDATOR checks the resolved return value.  Returns an alist recording
`outcome' (`supported', `unsupported', or `error'), `advertised',
`supported', `honest', `result', and `error' without terminating SESSION."
  (unless (and (femacs-eliscript-session-p session)
               (not (femacs-eliscript-session-closed session)))
    (femacs-eliscript--signal
     'femacs-eliscript-error "session-closed" "Eliscript session is closed"))
  (let* ((status (femacs-eliscript-status session))
         (worker (femacs-eliscript-session-worker session))
         (advertised (and (member "runtime-resolution"
                                  (alist-get 'capabilities status))
                          t))
         (exp-mod (expand-file-name probe-module))
         (exp-manifest (and probe-manifest (expand-file-name probe-manifest)))
         (exp-digest (if exp-manifest
                         (eliscript-worker--project-manifest-digest exp-manifest)
                       (femacs-eliscript--file-sha256 exp-mod)))
         (done nil)
         (probe-val nil)
         (probe-err nil)
         (req-id nil))
    (unwind-protect
        (progn
          (setq req-id
                (eliscript-worker-call
                 worker
                 exp-mod
                 (or export-name "ping_resolution")
                 nil
                 (lambda (val err)
                   (setq probe-val val probe-err err done t))
                 :module-digest exp-digest
                 :project-manifest exp-manifest
                 :value-codec t
                 :value-chunks t
                 :timeout-ms (or timeout-ms 10000)))
          (let ((deadline (+ (float-time) (/ (or timeout-ms 10000) 1000.0) 1.0)))
            (while (and (not done) (< (float-time) deadline))
              (let ((proc (eliscript-worker-process worker)))
                (if (and proc (process-live-p proc))
                    (accept-process-output proc 0.05)
                  (sleep-for 0.02))))))
      (when (and req-id (not done))
        (ignore-errors (eliscript-worker-cancel worker req-id))))
    (let* ((outcome nil)
           (final-err nil))
      (cond
       ((not done)
        (setq outcome 'error
              final-err '((code . "probe-timeout")
                          (message . "Runtime-resolution probe timed out"))))
       (probe-err
        (setq final-err probe-err
              outcome (if (femacs-eliscript--module-resolution-missing-p probe-err)
                          'unsupported
                        'error)))
       (t
        (let ((valid t))
          (when validator
            (condition-case verr
                (setq valid (funcall validator probe-val))
              (error
               (setq valid nil
                     final-err `((code . "probe-validator-error")
                                 (message . ,(error-message-string verr)))))))
          (if valid
              (setq outcome 'supported)
            (setq outcome 'error
                  final-err (or final-err
                                '((code . "probe-invalid-result")
                                  (message . "Runtime-resolution probe returned unexpected value"))))))))
      (let* ((supported (eq outcome 'supported))
             (honest
              (cond
               ((eq outcome 'supported) (if advertised t nil))
               ((eq outcome 'unsupported) (if advertised nil t))
               (t nil))))
        `((capability . "runtime-resolution")
          (advertised . ,advertised)
          (supported . ,supported)
          (outcome . ,outcome)
          (honest . ,honest)
          (runner-program . ,(alist-get 'runner-program status))
          (runner-version . ,(alist-get 'runner-version status))
          (module-path . ,exp-mod)
          (result . ,probe-val)
          (error . ,final-err))))))

(defun femacs-eliscript-runtime-resolution-probe
    (probe-module &optional probe-manifest export-name validator)
  "Return a capability probe verifying `runtime-resolution' on PROBE-MODULE."
  (lambda (session)
    (let ((report
           (femacs-eliscript-probe-runtime-resolution
            session probe-module probe-manifest export-name nil validator)))
      (unless (eq (alist-get 'outcome report) 'supported)
        (let ((err-obj (alist-get 'error report))
              (status (femacs-eliscript-status session)))
          (femacs-eliscript--signal
           'femacs-eliscript-capability-error
           "unsupported-capability"
           (format "Worker capability runtime-resolution failed live probe: %s"
                   (or (alist-get 'message err-obj) "resolution failed"))
           `((capability . "runtime-resolution")
             (outcome . ,(alist-get 'outcome report))
             (advertised . ,(alist-get 'advertised report))
             (honest . ,(alist-get 'honest report))
             (runner-program . ,(alist-get 'runner-program status))
             (runner-version . ,(alist-get 'runner-version status))
             (probe-error . ,err-obj)))))
      t)))

(defun femacs-eliscript--normalize-error (request err-obj)
  "Normalize service ERR-OBJ for REQUEST, enforcing side-effect unknown semantics."
  (when err-obj
    (let ((code (alist-get 'code err-obj)))
      (if (and (femacs-eliscript-request-side-effect request)
               (member code femacs-eliscript--ambiguous-side-effect-codes))
          `((code . "side-effect-completion-unknown")
            (unknown-completion . t)
            (replay-permitted . nil)
            (cause-code . ,code)
            (cause-message . ,(or (alist-get 'message err-obj)
                                  "worker completion unknown"))
            (request-id . ,(femacs-eliscript-request-request-id request))
            (worker-generation . ,(femacs-eliscript-request-worker-generation request))
            (message . ,(format "Side-effect operation completion unknown (%s); no automatic replay"
                                code)))
        err-obj))))

(cl-defun femacs-eliscript-call
    (session operation-name arguments callback
             &key path buffer apply progress metrics timeout-ms side-effect)
  "Invoke OPERATION-NAME asynchronously on SESSION with full request tracking."
  (unless (and (femacs-eliscript-session-p session)
               (not (femacs-eliscript-session-closed session)))
    (femacs-eliscript--signal
     'femacs-eliscript-error "session-closed" "Eliscript session is closed"))
  (let* ((service (femacs-eliscript-session-service session))
         (worker (femacs-eliscript-session-worker session))
         (pre-gen (eliscript-worker-generation worker)))
    (when (and side-effect (eq (eliscript-service-verify service) 'always))
      (femacs-eliscript--signal
       'femacs-eliscript-error
       "side-effect-verify-disallowed"
       "Side-effectful operations must never use dual-run verification"))
    (let* ((req
            (femacs-eliscript-request--create
             :session session
             :operation-name operation-name
             :worker-generation pre-gen
             :module-digest (femacs-eliscript-session-module-digest session)
             :buffer buffer
             :buffer-tick (and buffer
                               (with-current-buffer buffer
                                 (buffer-chars-modified-tick)))
             :side-effect (and side-effect t)
             :completion-status 'pending
             :done nil
             :progress-events nil))
           (svc-req
            (eliscript-service-call
             service operation-name arguments
             (lambda (val err-obj)
               (let* ((norm-err (femacs-eliscript--normalize-error req err-obj))
                      (status
                       (cond
                        ((null norm-err) 'succeeded)
                        ((alist-get 'unknown-completion norm-err) 'unknown)
                        ((equal (alist-get 'code norm-err) "cancelled")
                         'cancelled)
                        (t 'failed))))
                 (setf (femacs-eliscript-request-done req) t
                       (femacs-eliscript-request-result req) (and (null norm-err) val)
                       (femacs-eliscript-request-error req) norm-err
                       (femacs-eliscript-request-completion-status req) status)
                 (when callback
                   (funcall callback (and (null norm-err) val) norm-err req))))
             :path path
             :buffer buffer
             :apply apply
             :progress (lambda (pval)
                         (push pval (femacs-eliscript-request-progress-events req))
                         (when progress (funcall progress pval)))
             :metrics (lambda (timing)
                        (setf (femacs-eliscript-request-timing req) timing)
                        (when metrics (funcall metrics timing)))
             :timeout-ms timeout-ms)))
      (setf (femacs-eliscript-request-service-request req) svc-req
            (femacs-eliscript-request-request-id req)
            (eliscript-service-request-worker-id svc-req)
            (femacs-eliscript-request-worker-generation req)
            (eliscript-worker-generation worker))
      req)))

(defun femacs-eliscript-cancel (request)
  "Cancel pending REQUEST and return resulting completion status.

For side-effectful requests already dispatched to the worker, completion
remains marked unknown because cancellation cannot roll back external effects."
  (when (and (femacs-eliscript-request-p request)
             (not (femacs-eliscript-request-done request)))
    (let ((svc-req (femacs-eliscript-request-service-request request)))
      (eliscript-service-cancel svc-req)))
  (and (femacs-eliscript-request-p request)
       (femacs-eliscript-request-completion-status request)))

(defun femacs-eliscript-await (request &optional timeout-seconds)
  "Pump worker output until REQUEST finishes or TIMEOUT-SECONDS expires."
  (let* ((session (femacs-eliscript-request-session request))
         (worker (femacs-eliscript-session-worker session))
         (deadline (and timeout-seconds (+ (float-time) timeout-seconds))))
    (while (and (not (femacs-eliscript-request-done request))
                (or (null deadline) (< (float-time) deadline)))
      (let ((proc (eliscript-worker-process worker)))
        (if (and proc (process-live-p proc))
            (accept-process-output proc 0.05)
          (sleep-for 0.02))))
    request))

(cl-defun femacs-eliscript-call-side-effect
    (session operation-name arguments callback &key progress metrics timeout-ms)
  "Dispatch side-effectful OPERATION-NAME strictly once with unknown-on-failure semantics."
  (femacs-eliscript-call
   session operation-name arguments callback
   :path 'accelerated
   :progress progress
   :metrics metrics
   :timeout-ms timeout-ms
   :side-effect t))

(defun femacs-eliscript--render-analysis-buffer
    (session req target-buffer stats &optional query search-hits)
  "Populate SESSION output buffer with structured STATS and metadata."
  (let* ((status (femacs-eliscript-status session))
         (out-name (femacs-eliscript-session-output-buffer session))
         (out-buf (get-buffer-create out-name))
         (entry (and (vectorp stats) (> (length stats) 0) (aref stats 0))))
    (with-current-buffer out-buf
      (let ((inhibit-read-only t))
        (erase-buffer)
        (insert (format "Eliscript Analysis Report\n"))
        (insert (format "Buffer: %s (tick %S)\n"
                        (buffer-name target-buffer)
                        (femacs-eliscript-request-buffer-tick req)))
        (insert (format "Runner: %s %s (pid %S, generation %S, request %s)\n"
                        (alist-get 'runner-program status)
                        (alist-get 'runner-version status)
                        (alist-get 'pid status)
                        (femacs-eliscript-request-worker-generation req)
                        (femacs-eliscript-request-request-id req)))
        (insert (format "Module: %s (sha256 %s)\n"
                        (alist-get 'module-path status)
                        (femacs-eliscript-request-module-digest req)))
        (when entry
          (insert (format "Terms: %S, Unique Terms: %S\n"
                          (alist-get 'terms entry)
                          (alist-get 'uniqueTerms entry))))
        (when query
          (insert (format "Query %S hits: %S\n" query search-hits)))))
    out-buf))

(cl-defun femacs-eliscript-analyze-buffer
    (session &key buffer callback annotate query output-buffer timeout-ms progress metrics)
  "Asynchronously analyze BUFFER via Eliscript worker and update visible UI buffer.

Validates buffer modification tick prior to any edit and wraps visible buffer
annotations inside `atomic-change-group' so a single `undo' reverts them."
  (let* ((target (or buffer (current-buffer)))
         (docs (femacs-eliscript-documents-from-buffer target))
         (op-name (if query 'search-documents 'document-statistics))
         (args (if query (list docs query) (list docs)))
         (req-holder (vector nil)))
    (when output-buffer
      (setf (femacs-eliscript-session-output-buffer session) output-buffer))
    (setf
     (aref req-holder 0)
     (femacs-eliscript-call
      session op-name args
      (lambda (value err-obj req)
        (unless err-obj
          (femacs-eliscript--render-analysis-buffer
           session req target
           (unless query value)
           query
           (when query value)))
        (when callback
          (funcall callback value err-obj req)))
      :path 'accelerated
      :buffer target
      :apply (when annotate
               (lambda (value)
                 (let ((entry (and (vectorp value)
                                   (> (length value) 0)
                                   (aref value 0))))
                   (save-excursion
                     (goto-char (point-min))
                     (if query
                         (insert (format ";; [eliscript] query=%S matches=%S terms=%S\n"
                                         query
                                         (alist-get 'matches entry)
                                         (alist-get 'terms entry)))
                       (insert (format ";; [eliscript] terms=%S unique=%S\n"
                                       (alist-get 'terms entry)
                                       (alist-get 'uniqueTerms entry))))))))
      :progress progress
      :metrics metrics
      :timeout-ms timeout-ms))
    (aref req-holder 0)))

(provide 'femacs-eliscript-worker)
;;; femacs-eliscript-worker.el ends here
