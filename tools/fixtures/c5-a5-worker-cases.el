;;; c5-a5-worker-cases.el --- Detached C5/A5 editor worker acceptance cases  -*- lexical-binding: t; -*-

;;; Commentary:
;; Executed inside isolated Terminal PTY and Cocoa GUI FEmacs sessions from
;; a detached workspace outside both source repositories.  Expects env vars:
;;   FEMACS_A5_WORKER_EL   - path to copied femacs-eliscript-worker.el
;;   FEMACS_A5_PKG_DIR     - path to frozen Eliscript package
;;   FEMACS_A5_RUNNER      - "bun" or "node"
;;   FEMACS_A5_EXPECT_GUI  - "ns" or "tty"
;;   FEMACS_A5_WORK_DIR    - writable scratch dir for side-effect files
;;   FEMACS_A5_REPORT      - output JSON report file

;;; Code:

(require 'cl-lib)
(require 'json)
(require 'subr-x)

(setq inhibit-startup-screen t
      initial-scratch-message nil)

(defun femacs-a5--wait-until (pred &optional timeout-seconds process)
  "Pump events until PRED returns non-nil or TIMEOUT-SECONDS elapses."
  (let ((deadline (+ (float-time) (or timeout-seconds 10.0))))
    (while (and (not (funcall pred)) (< (float-time) deadline))
      (if (and process (process-live-p process))
          (accept-process-output process 0.03)
        (accept-process-output nil 0.03))
      (sleep-for 0.01))
    (funcall pred)))

(defun femacs-a5--wait-live-process (proc)
  "Wait for live process PROC or pump briefly."
  (when (and proc (process-live-p proc))
    (accept-process-output proc 0.03)))

(defun femacs-a5--obj-get (obj key)
  "Extract string KEY from persistent object OBJ or alist."
  (cond
   ((eliscript-value-object-p obj)
    (cdr (seq-find (lambda (e) (equal (car e) key))
                   (eliscript-value-object-entries obj))))
   ((listp obj)
    (alist-get (intern key) obj nil nil #'eq))
   (t nil)))

(defun femacs-a5--canonical-dir (path)
  "Return canonical directory string without trailing slash for PATH."
  (directory-file-name (file-truename (expand-file-name path))))

(defun femacs-a5--manifest-digest (manifest-path)
  "Read digest hex string from project MANIFEST-PATH."
  (with-temp-buffer
    (insert-file-contents manifest-path)
    (alist-get 'digest
               (json-parse-buffer
                :object-type 'alist
                :array-type 'array
                :null-object nil
                :false-object :false))))

(cl-defun femacs-a5--call-await
    (session operation-name arguments
             &key path buffer apply progress metrics timeout-ms side-effect
             await-seconds)
  "Invoke OPERATION-NAME asynchronously, await it, and cancel on non-local exit."
  (let ((req nil))
    (unwind-protect
        (progn
          (setq req
                (if side-effect
                    (femacs-eliscript-call-side-effect
                     session operation-name arguments nil
                     :progress progress :metrics metrics :timeout-ms timeout-ms)
                  (femacs-eliscript-call
                   session operation-name arguments nil
                   :path path :buffer buffer :apply apply
                   :progress progress :metrics metrics :timeout-ms timeout-ms)))
          (femacs-eliscript-await req (or await-seconds 30.0))
          (unless (femacs-eliscript-request-done req)
            (femacs-eliscript-cancel req)
            (femacs-eliscript--signal
             (if side-effect
                 'femacs-eliscript-side-effect-unknown-error
               'femacs-eliscript-error)
             (if side-effect "side-effect-completion-unknown" "await-timeout")
             "Timed out waiting for Eliscript request completion"
             `((request-id . ,(femacs-eliscript-request-request-id req)))))
          (when-let* ((err-obj (femacs-eliscript-request-error req)))
            (let* ((code (alist-get 'code err-obj))
                   (msg (or (alist-get 'message err-obj)
                            "Eliscript request failed"))
                   (sym (if (equal code "side-effect-completion-unknown")
                            'femacs-eliscript-side-effect-unknown-error
                          'femacs-eliscript-error)))
              (signal sym (list msg err-obj))))
          (femacs-eliscript-request-result req))
      (when (and req (not (femacs-eliscript-request-done req)))
        (femacs-eliscript-cancel req)))))

(defun femacs-a5--run-cases ()
  "Execute all C5/A5 checks and write JSON report before exiting."
  (let* ((worker-el (getenv "FEMACS_A5_WORKER_EL"))
         (pkg-dir (getenv "FEMACS_A5_PKG_DIR"))
         (module-root
          (let ((raw (getenv "FEMACS_A5_MODULE_ROOT")))
            (if (and raw (not (string-empty-p raw))) raw pkg-dir)))
         (reloc-mod
          (let ((raw (getenv "FEMACS_A5_RELOCATED_PROBE_MOD")))
            (and raw (not (string-empty-p raw)) raw)))
         (reloc-manifest
          (let ((raw (getenv "FEMACS_A5_RELOCATED_PROBE_MANIFEST")))
            (and raw (not (string-empty-p raw)) raw)))
         (runner (or (getenv "FEMACS_A5_RUNNER") "bun"))
         (expect-gui (or (getenv "FEMACS_A5_EXPECT_GUI") "tty"))
         (work-dir (getenv "FEMACS_A5_WORK_DIR"))
         (report-path (getenv "FEMACS_A5_REPORT"))
         (results nil)
         (overall-ok t))
    (condition-case top-err
        (progn
          ;; 1. Verify opt-in cleanliness before loading integration client
          (let ((pre-processes
                 (cl-remove-if-not
                  (lambda (p)
                    (string-match-p "bun\\|node\\|eliscript" (process-name p)))
                  (process-list))))
            (unless (and (not (featurep 'femacs-eliscript-worker))
                         (not (featurep 'eliscript-worker))
                         (null pre-processes))
              (error "Eliscript or JS runtime active before opt-in load: %S"
                     pre-processes))
            (push `((id . "opt_in_clean") (passed . t)
                    (framep . ,(symbol-name (framep (selected-frame)))))
                  results))

          ;; Verify expected interactive frame type
          (let ((ftype (framep (selected-frame))))
            (if (equal expect-gui "ns")
                (unless (and (eq ftype 'ns) (not noninteractive) (display-graphic-p))
                  (error "Expected Cocoa ns frame, got framep=%S noninteractive=%S"
                         ftype noninteractive))
              (unless (and (eq ftype t) (not noninteractive) (not (display-graphic-p)))
                (error "Expected TTY frame, got framep=%S noninteractive=%S"
                       ftype noninteractive))))

          (load worker-el nil t t)
          (require 'femacs-eliscript-worker)

          (when (or (fboundp 'femacs-eliscript-call-sync)
                    (fboundp 'femacs-eliscript-call-side-effect-sync))
            (error "Removed synchronous wrappers are unexpectedly bound"))

          ;; 2. Negative failure cases before starting live worker
          (let ((err-missing-rt
                 (condition-case e
                     (progn
                       (femacs-eliscript-resolve-install
                        :package-dir pkg-dir
                        :runner (expand-file-name "no-such-js-runner" work-dir))
                       nil)
                   (femacs-eliscript-missing-runtime-error e))))
            (unless (and err-missing-rt
                         (equal (alist-get 'code (nth 2 err-missing-rt))
                                "missing-js-runtime"))
              (error "Expected missing-js-runtime error, got %S" err-missing-rt)))

          (let* ((empty-pkg (expand-file-name "empty-pkg" work-dir)))
            (make-directory empty-pkg t)
            (let ((err-missing-res
                   (condition-case e
                       (progn
                         (femacs-eliscript-resolve-install
                          :package-dir empty-pkg
                          :runner runner)
                         nil)
                     (femacs-eliscript-missing-resource-error e))))
              (unless (and err-missing-res
                           (equal (alist-get 'code (nth 2 err-missing-res))
                                  "missing-worker-resource"))
                (error "Expected missing-worker-resource error, got %S"
                       err-missing-res))))
          (push `((id . "negative_missing_runtime_and_resource") (passed . t))
                results)

          ;; 3. Analysis session: dual-run verify, buffer edit + single undo, search, stale-buffer
          (let* ((analysis-mod (expand-file-name "analysis/analysis.mjs" module-root))
                 (analysis-manifest
                  (expand-file-name "analysis/eliscript-project.json" module-root))
                 (session
                  (femacs-eliscript-start
                   :package-dir pkg-dir
                   :runner runner
                   :working-directory module-root
                   :module-path analysis-mod
                   :project-manifest analysis-manifest
                   :verify 'always
                   :value-codec nil
                   :value-chunks nil)))
            (unwind-protect
                (let ((status (femacs-eliscript-status session))
                      (buf (get-buffer-create "*C5 Target*")))
                  (femacs-eliscript-require-capabilities
                   session '("request" "cancel" "timeout" "shutdown"
                             "module-digest" "project-manifest"
                             "value-codec" "value-chunks"))
                  (let ((err-cap
                         (condition-case e
                             (progn
                               (femacs-eliscript-require-capabilities
                                session '("nonexistent-capability-z"))
                               nil)
                           (femacs-eliscript-capability-error e))))
                    (unless (and err-cap
                                 (equal (alist-get 'code (nth 2 err-cap))
                                        "unsupported-capability"))
                      (error "Expected unsupported-capability error, got %S" err-cap)))

                  ;; 3a. Real buffer analysis -> *Eliscript Analysis* + atomic edit + single undo
                  (with-current-buffer buf
                    (erase-buffer)
                    (buffer-enable-undo)
                    (insert "alpha beta alpha gamma\nbeta delta\n")
                    (undo-boundary)
                    (switch-to-buffer buf)
                    (let* ((req (femacs-eliscript-analyze-buffer
                                 session :buffer buf :annotate t)))
                      (femacs-a5--wait-until
                       (lambda () (femacs-eliscript-request-done req))
                       20.0 (eliscript-worker-process
                             (femacs-eliscript-session-worker session)))
                      (unless (eq (femacs-eliscript-request-completion-status req)
                                  'succeeded)
                        (error "Buffer analysis failed: %S"
                               (femacs-eliscript-request-error req)))
                      (let ((expected-annotated
                             ";; [eliscript] terms=6 unique=4\nalpha beta alpha gamma\nbeta delta\n")
                            (report-text
                             (with-current-buffer "*Eliscript Analysis*"
                               (buffer-string))))
                        (unless (equal (buffer-string) expected-annotated)
                          (error "Unexpected annotated buffer: %S" (buffer-string)))
                        (unless (and (string-match-p "Terms: 6, Unique Terms: 4" report-text)
                                     (string-match-p (regexp-quote runner) report-text))
                          (error "Unexpected report buffer: %S" report-text))
                        (undo-boundary)
                        (undo)
                        (unless (equal (buffer-string)
                                       "alpha beta alpha gamma\nbeta delta\n")
                          (error "Single undo did not restore buffer: %S"
                                 (buffer-string))))

                      ;; 3b. Search query
                      (let* ((req-search
                              (femacs-eliscript-analyze-buffer
                               session :buffer buf :query "alpha")))
                        (femacs-a5--wait-until
                         (lambda () (femacs-eliscript-request-done req-search))
                         20.0 (eliscript-worker-process
                               (femacs-eliscript-session-worker session)))
                        (unless (and (eq (femacs-eliscript-request-completion-status
                                          req-search)
                                         'succeeded)
                                     (= (alist-get 'matches
                                                   (aref (femacs-eliscript-request-result
                                                          req-search)
                                                         0))
                                        2))
                          (error "Buffer search failed: %S"
                                 (femacs-eliscript-request-result req-search))))

                      ;; 3c. Stale buffer rejection: modify buffer right after dispatch
                      (let* ((req-stale
                              (femacs-eliscript-analyze-buffer
                               session :buffer buf :annotate t)))
                        (goto-char (point-max))
                        (insert "user edit wins\n")
                        (femacs-a5--wait-until
                         (lambda () (femacs-eliscript-request-done req-stale))
                         20.0 (eliscript-worker-process
                               (femacs-eliscript-session-worker session)))
                        (unless (and (eq (femacs-eliscript-request-completion-status
                                          req-stale)
                                         'failed)
                                     (equal (alist-get 'code
                                                       (femacs-eliscript-request-error
                                                        req-stale))
                                            "stale-buffer")
                                     (string-suffix-p "user edit wins\n" (buffer-string))
                                     (not (string-prefix-p ";; [eliscript]" (buffer-string))))
                          (error "Stale buffer check failed: req=%S text=%S"
                                 (femacs-eliscript-request-error req-stale)
                                 (buffer-string))))

                      (push `((id . "buffer_analysis_search_stale_and_cap")
                              (passed . t)
                              (runner . ,(alist-get 'runner-program status))
                              (runner_version . ,(alist-get 'runner-version status))
                              (pid . ,(alist-get 'pid status))
                              (capabilities . ,(vconcat (alist-get 'capabilities status)))
                              (module_digest . ,(alist-get 'module-digest status)))
                            results)))
              (femacs-eliscript-stop session t)))

          ;; 4. Probe session: chunking, source maps, cancel, stderr, crash/recovery, side effects
          (let* ((probe-mod (expand-file-name "probes/c5-probes.mjs" module-root))
                 (probe-manifest
                  (expand-file-name "probes/eliscript-project.json" module-root))
                 (ops
                  (list
                   (eliscript-service-operation
                    'ping-resolution (lambda () nil) :export-name "ping_resolution")
                   (eliscript-service-operation
                    'chunked-echo (lambda (x) x) :export-name "chunked_echo")
                   (eliscript-service-operation
                    'slow-cancellable (lambda (_s _d) nil) :export-name "slow_cancellable")
                   (eliscript-service-operation
                    'trigger-mapped-error (lambda (r) r) :export-name "trigger_mapped_error")
                   (eliscript-service-operation
                    'hang-forever (lambda () nil) :export-name "hang_forever")
                   (eliscript-service-operation
                    'stderr-log-echo (lambda (m) m) :export-name "stderr_log_echo")
                   (eliscript-service-operation
                    'side-effect-append (lambda (_p _tok _ms) nil)
                    :export-name "side_effect_append")
                   (eliscript-service-operation
                    'report-cwd (lambda () nil) :export-name "report_cwd")))
                 (session
                  (femacs-eliscript-start
                   :package-dir pkg-dir
                   :runner runner
                   :working-directory module-root
                   :module-path probe-mod
                   :project-manifest probe-manifest
                   :verify nil
                   :value-codec t
                   :value-chunks t
                   :operations ops)))
            (unwind-protect
                (progn
                  ;; 4a. Runtime module resolution probe
                  (let ((ping (femacs-a5--call-await session 'ping-resolution nil)))
                    (unless (and (eliscript-value-object-p ping)
                                 (eq (femacs-a5--obj-get ping "resolved") t))
                      (error "Runtime resolution probe unexpected: %S" ping)))

                  ;; 4b. Persistent value codec + chunked streaming (> max-text-part-units)
                  (let* ((big-str (make-string 18000 ?x))
                         (payload
                          (eliscript-value-map
                           (list (cons (eliscript-value-keyword "body" "c5") big-str)
                                 (cons (eliscript-value-keyword "count" "c5") 42))))
                         (progress-seen nil)
                         (echoed
                          (femacs-a5--call-await
                           session 'chunked-echo (list payload)
                           :progress (lambda (p) (push p progress-seen)))))
                    (unless (and (eliscript-value-map-p echoed)
                                 (>= (length progress-seen) 1)
                                 (equal
                                  (cdr (seq-find
                                        (lambda (kv)
                                          (and (eliscript-value-keyword-p (car kv))
                                               (equal (eliscript-value-keyword-name (car kv))
                                                      "body")))
                                        (eliscript-value-map-entries echoed)))
                                  big-str))
                      (error "Chunked persistent value mismatch"))

                    ;; Oversized stream limit rejection under tightened max-total-string-units
                    (let* ((eliscript-worker-value-stream-limits
                            (append '((max-total-string-units . 1024))
                                    eliscript-worker-value-stream-limits))
                           (limit-err
                            (condition-case e
                                (progn
                                  (femacs-a5--call-await
                                   session 'chunked-echo (list (make-string 2048 ?y)))
                                  nil)
                              (error e))))
                      (unless limit-err
                        (error "Expected value stream limit error")))
                    (push `((id . "chunked_values_and_limits")
                            (passed . t)
                            (progress_count . ,(length progress-seen)))
                          results))

                  ;; 4c. Cooperative cancellation after receiving progress
                  (let* ((cancel-req
                          (femacs-eliscript-call
                           session 'slow-cancellable '(20 80) nil))
                         (proc (eliscript-worker-process
                                (femacs-eliscript-session-worker session))))
                    (unless (femacs-a5--wait-until
                             (lambda ()
                               (>= (length (femacs-eliscript-request-progress-events
                                            cancel-req))
                                   1))
                             10.0 proc)
                      (error "Slow request never emitted initial progress"))
                    (femacs-eliscript-cancel cancel-req)
                    (femacs-a5--wait-until
                     (lambda () (femacs-eliscript-request-done cancel-req))
                     5.0 proc)
                    (unless (and (eq (femacs-eliscript-request-completion-status cancel-req)
                                     'cancelled)
                                 (eliscript-worker-live-p
                                  (femacs-eliscript-session-worker session)))
                      (error "Cancellation failed or killed worker: %S"
                             (femacs-eliscript-request-completion-status cancel-req)))
                    (push `((id . "cooperative_cancel")
                            (passed . t)
                            (progress_before_cancel
                             . ,(length (femacs-eliscript-request-progress-events
                                         cancel-req))))
                          results))

                  ;; 4d. Mapped .eli error location via source maps
                  (let ((mapped-err
                         (condition-case e
                             (progn
                               (femacs-a5--call-await
                                session 'trigger-mapped-error '("boom"))
                               nil)
                           (femacs-eliscript-error e))))
                    (unless mapped-err
                      (error "Expected mapped worker error"))
                    (let* ((err-obj (nth 2 mapped-err))
                           (loc (eliscript-worker-error-location err-obj))
                           (formatted (eliscript-worker-format-error err-obj)))
                      (unless (and loc
                                   (string-suffix-p "c5-probes.eli"
                                                    (or (alist-get 'file loc) ""))
                                   (integerp (alist-get 'line loc))
                                   (> (alist-get 'line loc) 0)
                                   (string-match-p "c5-mapped-error: boom" formatted))
                        (error "Expected .eli source-map error, got loc=%S fmt=%S"
                               loc formatted))
                      (push `((id . "mapped_source_error")
                              (passed . t)
                              (location . ,loc)
                              (formatted . ,formatted))
                            results)))

                  ;; 4e. Stderr log capture
                  (let* ((echo-obj
                          (femacs-a5--call-await
                           session 'stderr-log-echo '("c5-log-marker")))
                         (worker (femacs-eliscript-session-worker session)))
                    (accept-process-output (eliscript-worker-process worker) 0.05)
                    (let ((stderr-text (eliscript-worker-stderr worker)))
                      (unless (and (eliscript-value-object-p echo-obj)
                                   (string-match-p "worker-stderr-log:c5-log-marker"
                                                   stderr-text))
                        (error "Expected stderr marker in worker stderr, got %S"
                               stderr-text))
                      (push `((id . "stderr_log_isolation") (passed . t)) results)))

                  ;; 4f. Crash (SIGKILL) + recovery on next generation
                  (let* ((worker (femacs-eliscript-session-worker session))
                         (gen-before (eliscript-worker-generation worker))
                         (hang-req (femacs-eliscript-call session 'hang-forever nil nil))
                         (proc (eliscript-worker-process worker)))
                    (signal-process proc 'SIGKILL)
                    (femacs-a5--wait-until
                     (lambda () (femacs-eliscript-request-done hang-req))
                     5.0 nil)
                    (unless (and (eq (femacs-eliscript-request-completion-status hang-req)
                                     'failed)
                                 (equal (alist-get 'code
                                                   (femacs-eliscript-request-error
                                                    hang-req))
                                        "worker-exit"))
                      (error "Crash fail unexpected: %S"
                             (femacs-eliscript-request-error hang-req)))
                    (let ((recovered
                           (femacs-a5--call-await
                            session 'stderr-log-echo '("post-crash"))))
                      (unless (and (eliscript-value-object-p recovered)
                                   (= (eliscript-worker-generation worker)
                                      (1+ gen-before)))
                        (error "Post-crash generation recovery failed: gen=%S"
                               (eliscript-worker-generation worker)))
                      (push `((id . "worker_crash_and_recovery")
                              (passed . t)
                              (gen_before . ,gen-before)
                              (gen_after . ,(eliscript-worker-generation worker)))
                            results)))

                  ;; 4g. Side-effectful request: verify disallowed, timeout unknown, kill unknown, user-cancel unknown, zero replay
                  (let* ((se-timeout-file (expand-file-name "se-timeout.txt" work-dir))
                         (se-kill-file (expand-file-name "se-kill.txt" work-dir))
                         (se-cancel-file (expand-file-name "se-cancel.txt" work-dir)))
                    (dolist (f (list se-timeout-file se-kill-file se-cancel-file))
                      (when (file-exists-p f) (delete-file f)))

                    ;; Reject verify=always for side-effect session
                    (let* ((se-verify-sess
                            (femacs-eliscript-start
                             :package-dir pkg-dir
                             :runner runner
                             :working-directory pkg-dir
                             :module-path probe-mod
                             :project-manifest probe-manifest
                             :verify 'always
                             :value-codec t
                             :value-chunks t
                             :operations ops)))
                      (unwind-protect
                          (let ((v-err
                                 (condition-case e
                                     (progn
                                       (femacs-eliscript-call-side-effect
                                        se-verify-sess 'side-effect-append
                                        (list se-timeout-file "bad" 10) nil)
                                       nil)
                                   (femacs-eliscript-error e))))
                            (unless (and v-err
                                         (equal (alist-get 'code (nth 2 v-err))
                                                "side-effect-verify-disallowed"))
                              (error "Expected side-effect-verify-disallowed, got %S"
                                     v-err)))
                        (femacs-eliscript-stop se-verify-sess t)))

                    ;; Side effect timeout -> completion unknown, strictly 1 execution
                    (let ((t-err
                           (condition-case e
                               (progn
                                 (femacs-a5--call-await
                                  session 'side-effect-append
                                  (list se-timeout-file "timeout-token" 2500)
                                  :side-effect t :timeout-ms 250 :await-seconds 10.0)
                                 nil)
                             (femacs-eliscript-side-effect-unknown-error e))))
                      (unless (and t-err
                                   (equal (alist-get 'code (nth 2 t-err))
                                          "side-effect-completion-unknown")
                                   (equal (alist-get 'cause-code (nth 2 t-err)) "timeout")
                                   (null (alist-get 'replay-permitted (nth 2 t-err))))
                        (error "Expected side-effect timeout unknown, got %S" t-err))
                      ;; Subsequent call triggers new generation and succeeds without replaying
                      (femacs-a5--call-await session 'stderr-log-echo '("after-se-timeout"))
                      (sleep-for 0.15)
                      (let ((lines
                             (with-temp-buffer
                               (insert-file-contents se-timeout-file)
                               (split-string (buffer-string) "\n" t))))
                        (unless (equal lines '("timeout-token"))
                          (error "Expected strictly 1 side-effect line on timeout, got %S"
                                 lines))))

                    ;; Side effect worker SIGKILL -> completion unknown, strictly 1 execution
                    (let* ((se-req
                            (femacs-eliscript-call-side-effect
                             session 'side-effect-append
                             (list se-kill-file "kill-token" 2500) nil))
                           (worker (femacs-eliscript-session-worker session))
                           (proc (eliscript-worker-process worker)))
                      (unless (femacs-a5--wait-until
                               (lambda ()
                                 (and (file-exists-p se-kill-file)
                                      (> (nth 7 (file-attributes se-kill-file)) 0)))
                               5.0 proc)
                        (error "Side-effect file never written before SIGKILL"))
                      (signal-process proc 'SIGKILL)
                      (femacs-a5--wait-until
                       (lambda () (femacs-eliscript-request-done se-req))
                       5.0 nil)
                      (unless (and (eq (femacs-eliscript-request-completion-status se-req)
                                       'unknown)
                                   (equal (alist-get 'code
                                                     (femacs-eliscript-request-error se-req))
                                          "side-effect-completion-unknown")
                                   (equal (alist-get 'cause-code
                                                     (femacs-eliscript-request-error se-req))
                                          "worker-exit"))
                        (error "Expected unknown on side-effect SIGKILL, got %S"
                               (femacs-eliscript-request-error se-req)))
                      (femacs-a5--call-await session 'stderr-log-echo '("after-se-kill"))
                      (let ((lines
                             (with-temp-buffer
                               (insert-file-contents se-kill-file)
                               (split-string (buffer-string) "\n" t))))
                        (unless (equal lines '("kill-token"))
                          (error "Expected strictly 1 side-effect line after kill, got %S"
                                 lines))))

                    ;; Side effect user cancel after dispatch -> unknown status, single callback, zero replay
                    (let* ((cancel-cb-count 0)
                           (se-cancel-req
                            (femacs-eliscript-call-side-effect
                             session 'side-effect-append
                             (list se-cancel-file "cancel-token" 1500)
                             (lambda (_val _err _r)
                               (setq cancel-cb-count (1+ cancel-cb-count)))))
                           (proc (eliscript-worker-process
                                  (femacs-eliscript-session-worker session))))
                      (unless (femacs-a5--wait-until
                               (lambda ()
                                 (and (file-exists-p se-cancel-file)
                                      (> (nth 7 (file-attributes se-cancel-file)) 0)))
                               5.0 proc)
                        (error "Side-effect cancel file never written before cancel"))
                      (let ((cancel-ret (femacs-eliscript-cancel se-cancel-req))
                            (err-obj (femacs-eliscript-request-error se-cancel-req)))
                        (unless (and (eq cancel-ret 'unknown)
                                     (eq (femacs-eliscript-request-completion-status se-cancel-req)
                                         'unknown)
                                     (= cancel-cb-count 1)
                                     (equal (alist-get 'code err-obj)
                                            "side-effect-completion-unknown")
                                     (equal (alist-get 'cause-code err-obj)
                                            "cancelled")
                                     (null (alist-get 'replay-permitted err-obj)))
                          (error "Expected side-effect user-cancel unknown, got ret=%S cb=%S err=%S"
                                 cancel-ret cancel-cb-count err-obj)))
                      (femacs-a5--call-await session 'stderr-log-echo '("after-se-cancel"))
                      (sleep-for 0.2)
                      (let ((lines
                             (with-temp-buffer
                               (insert-file-contents se-cancel-file)
                               (split-string (buffer-string) "\n" t))))
                        (unless (and (= cancel-cb-count 1)
                                     (equal lines '("cancel-token")))
                          (error "Expected strictly 1 callback and 1 line after side-effect cancel, cb=%S lines=%S"
                                 cancel-cb-count lines))))

                    (push `((id . "side_effect_single_run_and_unknown_completion")
                            (passed . t))
                          results)))
              (femacs-eliscript-stop session t))

            ;; 5. Project-manifest and explicit module-digest consistency
            (let* ((expected-digest (femacs-a5--manifest-digest probe-manifest))
                   (conflict-err
                    (condition-case e
                        (progn
                          (femacs-eliscript-start
                           :package-dir pkg-dir
                           :runner runner
                           :working-directory pkg-dir
                           :module-path probe-mod
                           :project-manifest probe-manifest
                           :module-digest (make-string 64 ?0)
                           :verify nil
                           :value-codec t
                           :value-chunks t
                           :operations ops)
                          nil)
                      (femacs-eliscript-error e))))
              (unless (and conflict-err
                           (equal (alist-get 'code (nth 2 conflict-err))
                                  "module-digest-conflict"))
                (error "Expected module-digest-conflict error, got %S" conflict-err))
              (let ((digest-sess
                     (femacs-eliscript-start
                      :package-dir pkg-dir
                      :runner runner
                      :working-directory pkg-dir
                      :module-path probe-mod
                      :project-manifest probe-manifest
                      :module-digest expected-digest
                      :verify nil
                      :value-codec t
                      :value-chunks t
                      :operations ops)))
                (unwind-protect
                    (let* ((status (femacs-eliscript-status digest-sess))
                           (svc-digest
                            (eliscript-service-module-digest
                             (eliscript-service-module
                              (femacs-eliscript-session-service digest-sess))))
                           (cwd-res (femacs-a5--call-await digest-sess 'report-cwd nil)))
                      (unless (and (equal expected-digest svc-digest)
                                   (equal expected-digest (alist-get 'module-digest status))
                                   (equal expected-digest
                                          (femacs-eliscript-session-module-digest digest-sess))
                                   (stringp (femacs-a5--obj-get cwd-res "cwd")))
                        (error "Consistent digest session mismatch: expected=%s svc=%s status=%s"
                               expected-digest svc-digest (alist-get 'module-digest status)))
                      (push `((id . "module_digest_consistency")
                              (passed . t)
                              (digest . ,expected-digest))
                            results))
                  (femacs-eliscript-stop digest-sess t))))

            ;; 6. Per-session working directory isolation across concurrent sessions and restarts
            (let ((dir-a (expand-file-name "cwd-a" work-dir))
                  (dir-b (expand-file-name "cwd-b" work-dir))
                  (sess-a nil)
                  (sess-b nil))
              (make-directory dir-a t)
              (make-directory dir-b t)
              (unwind-protect
                  (let ((canon-a (femacs-a5--canonical-dir dir-a))
                        (canon-b (femacs-a5--canonical-dir dir-b)))
                    (setq sess-a
                          (femacs-eliscript-start
                           :package-dir pkg-dir
                           :runner runner
                           :working-directory dir-a
                           :module-path probe-mod
                           :project-manifest probe-manifest
                           :verify nil
                           :value-codec t
                           :value-chunks t
                           :operations ops))
                    (let ((cwd-a1
                           (femacs-a5--canonical-dir
                            (femacs-a5--obj-get
                             (femacs-a5--call-await sess-a 'report-cwd nil) "cwd")))
                          (status-a1 (femacs-eliscript-status sess-a)))
                      (unless (and (equal cwd-a1 canon-a)
                                   (equal (femacs-a5--canonical-dir
                                           (alist-get 'working-directory status-a1))
                                          canon-a))
                        (error "Session A initial cwd mismatch: %s vs %s" cwd-a1 canon-a)))

                    (setq sess-b
                          (femacs-eliscript-start
                           :package-dir pkg-dir
                           :runner runner
                           :working-directory dir-b
                           :module-path probe-mod
                           :project-manifest probe-manifest
                           :verify nil
                           :value-codec t
                           :value-chunks t
                           :operations ops))
                    (let ((cwd-b1
                           (femacs-a5--canonical-dir
                            (femacs-a5--obj-get
                             (femacs-a5--call-await sess-b 'report-cwd nil) "cwd")))
                          (status-b1 (femacs-eliscript-status sess-b)))
                      (unless (and (equal cwd-b1 canon-b)
                                   (equal (femacs-a5--canonical-dir
                                           (alist-get 'working-directory status-b1))
                                          canon-b))
                        (error "Session B initial cwd mismatch: %s vs %s" cwd-b1 canon-b)))

                    ;; Explicit restart of Session A must remain in dir-a (not B's dir)
                    (femacs-eliscript-restart sess-a)
                    (let ((cwd-a2
                           (femacs-a5--canonical-dir
                            (femacs-a5--obj-get
                             (femacs-a5--call-await sess-a 'report-cwd nil) "cwd")))
                          (status-a2 (femacs-eliscript-status sess-a)))
                      (unless (and (equal cwd-a2 canon-a)
                                   (equal (femacs-a5--canonical-dir
                                           (alist-get 'working-directory status-a2))
                                          canon-a))
                        (error "Session A explicit restart cwd mismatch: %s vs %s"
                               cwd-a2 canon-a)))

                    ;; SIGKILL + automatic relaunch of Session A must still stay in dir-a
                    (let* ((worker-a (femacs-eliscript-session-worker sess-a))
                           (gen-before (eliscript-worker-generation worker-a))
                           (proc-a (eliscript-worker-process worker-a)))
                      (signal-process proc-a 'SIGKILL)
                      (femacs-a5--wait-until
                       (lambda () (not (eliscript-worker-live-p worker-a)))
                       5.0 nil)
                      (let ((cwd-a3
                             (femacs-a5--canonical-dir
                              (femacs-a5--obj-get
                               (femacs-a5--call-await sess-a 'report-cwd nil) "cwd")))
                            (status-a3 (femacs-eliscript-status sess-a))
                            (cwd-b2
                             (femacs-a5--canonical-dir
                              (femacs-a5--obj-get
                               (femacs-a5--call-await sess-b 'report-cwd nil) "cwd"))))
                        (unless (and (equal cwd-a3 canon-a)
                                     (equal (femacs-a5--canonical-dir
                                             (alist-get 'working-directory status-a3))
                                            canon-a)
                                     (= (alist-get 'generation status-a3) (1+ gen-before))
                                     (equal cwd-b2 canon-b))
                          (error "Auto-relaunch cwd isolation failed: a=%s status=%s b=%s"
                                 cwd-a3 (alist-get 'working-directory status-a3) cwd-b2)))
                      (push `((id . "session_cwd_isolation")
                              (passed . t)
                              (dir_a_suffix . "cwd-a")
                              (dir_b_suffix . "cwd-b")
                              (gen_after_auto_relaunch
                               . ,(eliscript-worker-generation worker-a)))
                            results)))
                (when sess-b (femacs-eliscript-stop sess-b t))
                (when sess-a (femacs-eliscript-stop sess-a t))))

            ;; 7. Optional C6 relocated ancestor-less runtime-resolution probe
            (when (and reloc-mod reloc-manifest)
              (let ((reloc-sess
                     (femacs-eliscript-start
                      :package-dir pkg-dir
                      :runner runner
                      :working-directory module-root
                      :module-path probe-mod
                      :project-manifest probe-manifest
                      :verify nil
                      :value-codec t
                      :value-chunks t
                      :operations ops)))
                (unwind-protect
                    (let* ((validator
                            (lambda (val)
                              (and (eliscript-value-object-p val)
                                   (eq (femacs-a5--obj-get val "resolved") t)
                                   (eq (femacs-a5--obj-get val "progressFn") t))))
                           (probe
                            (femacs-eliscript-probe-runtime-resolution
                             reloc-sess reloc-mod reloc-manifest nil nil validator))
                           (outcome (alist-get 'outcome probe))
                           (advertised (alist-get 'advertised probe))
                           (supported (alist-get 'supported probe))
                           (honest (alist-get 'honest probe))
                           (err-obj (alist-get 'error probe)))
                      (cond
                       ((eq outcome 'error)
                        (error "Relocated runtime-resolution probe errored unexpectedly: %S"
                               probe))
                       ((eq outcome 'supported)
                        (unless (eq (femacs-eliscript-require-capabilities
                                     reloc-sess '("runtime-resolution")
                                     (list (cons "runtime-resolution"
                                                 (femacs-eliscript-runtime-resolution-probe
                                                  reloc-mod reloc-manifest nil validator))))
                                    t)
                          (error "Expected live runtime-resolution gate to pass when supported")))
                       ((eq outcome 'unsupported)
                        (unless (femacs-eliscript--module-resolution-missing-p err-obj)
                          (error "Expected explicit module-resolution missing error, got %S"
                                 probe))
                        (let ((gate-err
                               (condition-case e
                                   (progn
                                     (femacs-eliscript-require-capabilities
                                      reloc-sess '("runtime-resolution")
                                      (list (cons "runtime-resolution"
                                                  (femacs-eliscript-runtime-resolution-probe
                                                   reloc-mod reloc-manifest nil validator))))
                                     nil)
                                 (femacs-eliscript-capability-error e))))
                          (unless (and gate-err
                                       (equal (alist-get 'code (nth 2 gate-err))
                                              "unsupported-capability"))
                            (error "Expected unsupported-capability on live gate, got %S"
                                   gate-err)))
                        (unless (eliscript-worker-live-p
                                 (femacs-eliscript-session-worker reloc-sess))
                          (error "Relocated resolution probe killed worker session"))
                        (let ((after
                               (femacs-a5--call-await
                                reloc-sess 'stderr-log-echo '("after-reloc-probe"))))
                          (unless (eliscript-value-object-p after)
                            (error "Follow-up call after failed relocated probe failed"))))
                       (t
                        (error "Unexpected probe outcome: %S" outcome)))
                      (push `((id . "relocated_runtime_resolution_probe")
                              (passed . t)
                              (outcome . ,(symbol-name outcome))
                              (advertised . ,(if advertised t :false))
                              (supported . ,(if supported t :false))
                              (honest . ,(if honest t :false))
                              (error_code . ,(alist-get 'code err-obj))
                              (error_message . ,(alist-get 'message err-obj)))
                            results))
                  (femacs-eliscript-stop reloc-sess t)))))))
      (error
       (setq overall-ok nil)
       (push `((id . "fatal_error")
               (passed . nil)
               (error . ,(error-message-string top-err)))
             results)))

    (when report-path
      (with-temp-file report-path
        (insert
         (decode-coding-string
          (json-serialize
           `((passed . ,(if overall-ok t :false))
             (frame_type . ,(format "%s" (framep (selected-frame))))
             (window_system . ,(format "%s" (window-system)))
             (noninteractive . ,(if noninteractive t :false))
             (cases . ,(vconcat (nreverse results))))
           :null-object nil :false-object :false)
          'utf-8-unix)
         "\n")))
    (kill-emacs (if overall-ok 0 1))))

(run-at-time 60 nil
             (lambda ()
               (message "C5/A5 watchdog timed out")
               (kill-emacs 99)))

(add-hook 'window-setup-hook
          (lambda ()
            (run-at-time 0.05 nil #'femacs-a5--run-cases)))

(provide 'c5-a5-worker-cases)
;;; c5-a5-worker-cases.el ends here
