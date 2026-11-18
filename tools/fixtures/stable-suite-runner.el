;;; stable-suite-runner.el --- machine bridge for the stable corpus -*- lexical-binding: t; -*-

;; Loaded into an already-built product by tools/stable-suite.py.  It writes
;; machine-readable discovery/run records to STABLE_SUITE_RESULT and leaves all
;; human output on the product's stdout/stderr.  It does not choose tests,
;; filter platform/features, or claim compatibility.

(defconst stable-suite--runner-file load-file-name)
(defvar find-function-C-source-directory)
(defun stable-suite--source-context-load-path (paths product-lisp context-lisp)
  "Keep context libraries first and retain physical product PATHS for lookup."
  (let ((original (file-name-as-directory product-lisp))
        (overlay (file-name-as-directory context-lisp)))
    (cons overlay
          (append
           (mapcar (lambda (path)
                     (if (and (stringp path)
                              (string-prefix-p original
                                (file-name-as-directory path)))
                         (concat overlay
                           (substring (file-name-as-directory path)
                                      (length original)))
                       path))
                   paths)
           ;; Dump load-history keeps these physical names.  Help uses their
           ;; lexical load-path prefixes when shortening source filenames.
           (delq nil
                 (mapcar (lambda (path)
                           (and (stringp path)
                                (string-prefix-p original
                                  (file-name-as-directory path))
                                path))
                         paths))))))

(let* ((context (getenv "STABLE_SUITE_SOURCE_CONTEXT"))
       (product-lisp (getenv "STABLE_SUITE_PRODUCT_LISP"))
       (archive-profile (getenv "STABLE_SUITE_ARCHIVE_SOURCE_PROFILE"))
       (shared-context (getenv "STABLE_SUITE_ARCHIVE_SHARED_CONTEXT")))
  (when (or archive-profile shared-context)
    (let ((case-root (getenv "FEMACS_RUNTIME_CASE_ROOT"))
          (logical (getenv "STABLE_SUITE_FILE_ID"))
          (physical (getenv "STABLE_SUITE_FILE")))
      (unless (and (equal archive-profile "owned-original-archive-source-context-v1")
                   shared-context context case-root physical
                   (member logical '("test/lisp/arc-mode-tests.el"
                                     "test/lisp/tar-mode-tests.el"))
                   (not (file-symlink-p context))
                   (equal (file-truename context)
                          (expand-file-name "out" (file-truename case-root)))
                   (equal (file-truename physical)
                          (expand-file-name logical (file-truename context)))
                   (not (equal (file-truename shared-context)
                               (file-truename context))))
        (error "invalid owned archive source-test context"))))
  (when context
    (let ((library-context (or shared-context context)))
      (unless (and product-lisp (file-directory-p context)
                   (file-directory-p (expand-file-name "test" context))
                   (file-directory-p (expand-file-name "lisp" library-context)))
        (error "missing bound source-test context"))
      (setq load-path
            (stable-suite--source-context-load-path
             load-path product-lisp (expand-file-name "lisp" library-context)))
      (setq source-directory (file-name-as-directory context)
            find-function-C-source-directory
            (expand-file-name "src/" library-context)))))
(let ((invocation-dir (getenv "STABLE_SUITE_INVOCATION_DIRECTORY"))
      (invocation-prog (getenv "STABLE_SUITE_INVOCATION_NAME")))
  (when (or invocation-dir invocation-prog)
    (unless (and invocation-dir invocation-prog
                 (equal (getenv "STABLE_SUITE_FILE_ID")
                        "test/src/emacs-module-tests.el")
                 (file-directory-p invocation-dir)
                 (file-executable-p
                  (expand-file-name invocation-prog invocation-dir)))
      (error "invalid owned module invocation context"))
    (setq invocation-directory (file-name-as-directory invocation-dir)
          invocation-name invocation-prog)))

(require 'ert)
(require 'json)
(require 'cl-lib)

;; Only the owned Darwin mock fixtures request this shell initialization.
;; Tramp clears ENV when re-execing its remote shell, so the process ENV alone
;; is insufficient.  Use its explicit remote environment with shell quoting.
(defvar stable-suite--mock-shell-environment nil)
(defvar stable-suite--mock-ps-bin nil)
(defun stable-suite--mock-process-file (args)
  (if (and stable-suite--mock-ps-bin
           (equal (car args) "ps")
           (equal (file-remote-p default-directory 'method) "mock"))
      (cons stable-suite--mock-ps-bin (cdr args))
    args))
(let ((script (getenv "FEMACS_STABLE_MOCK_SHELL_ENV")))
  (when script
    (unless (and (eq system-type 'darwin)
                 (member (getenv "STABLE_SUITE_FILE_ID")
                         '("test/lisp/autorevert-tests.el"
                           "test/lisp/filenotify-tests.el"
                           "test/lisp/dnd-tests.el"
                           "test/lisp/ffap-tests.el"
                           "test/lisp/shadowfile-tests.el"
                           "test/lisp/vc/emerge-tests.el"
                           "test/lisp/progmodes/eglot-tests.el"
                           "test/lisp/net/tramp-tests.el"
                           "test/lisp/eshell/em-glob-tests.el"
                           "test/lisp/eshell/esh-ext-tests.el"
                           "test/lisp/eshell/esh-proc-tests.el"
                           "test/lisp/eshell/esh-util-tests.el"
                           "test/lisp/eshell/esh-var-tests.el"))
                 (equal (getenv "ENV") script)
                 (file-regular-p script)
                 (not (file-symlink-p script))
                 (file-in-directory-p script
                                      (getenv "FEMACS_RUNTIME_CASE_ROOT")))
      (error "invalid owned mock shell initialization"))
    (let* ((mock-bin (getenv "FEMACS_STABLE_MOCK_BIN"))
           (ps-bin (and mock-bin (expand-file-name "ps" mock-bin))))
      (unless (and mock-bin
                   (file-directory-p mock-bin)
                   (not (file-symlink-p mock-bin))
                   (file-in-directory-p mock-bin
                                        (getenv "FEMACS_RUNTIME_CASE_ROOT"))
                   (file-regular-p ps-bin)
                   (not (file-symlink-p ps-bin))
                   (file-executable-p ps-bin)
                   (zerop (logand (or (file-modes ps-bin) #o6000) #o6000)))
        (error "invalid owned non-setuid mock ps helper"))
      (setq stable-suite--mock-ps-bin ps-bin)
      (advice-add 'process-file :filter-args #'stable-suite--mock-process-file)
      (with-eval-after-load 'tramp
        (let ((content (with-temp-buffer
                         (insert-file-contents-literally script)
                         (buffer-string))))
          (unless (equal content
                         "case $- in\n  *i*)\n    set +m\n    set +o vi +o emacs\n    if [ -t 0 ]; then\n      stty -icanon icrnl -inlcr -igncr -echo min 1 time 0 || exit 70\n    fi\n    trap 'trap - DEBUG; set +m' DEBUG\n    ;;\nesac\n")
            (error "mock shell initialization content differs"))
          (setq tramp-remote-process-environment
                (cons (concat "ENV=" (shell-quote-argument script))
                      (cl-remove-if (lambda (value)
                                      (string-prefix-p "ENV=" value))
                                    tramp-remote-process-environment)))
          (setq stable-suite--mock-shell-environment
                (list (cons "profile" "owned-darwin-mock-noncanonical-icrnl")
                      (cons "content_sha256" (secure-hash 'sha256 content))
                      (cons "file" script)
                      (cons "process_env" (getenv "ENV"))
                      (cons "remote_env"
                            (car tramp-remote-process-environment)))))))))

(defun stable-suite--env (name)
  (or (getenv name)
      (error "missing environment variable %s" name)))

(defvar stable-suite--compiler-output-context nil)

(defun stable-suite--prepare-compiler-output ()
  "Give the original compiler bootstrap an owned last output directory."
  (when (equal (getenv "STABLE_SUITE_FILE_ID") "test/src/comp-tests.el")
    (let* ((enabled (and (fboundp 'native-comp-available-p)
                         (native-comp-available-p)))
           (before (and (boundp 'native-comp-eln-load-path)
                        (copy-sequence native-comp-eln-load-path)))
           target)
      (when enabled
        (unless (and (listp before) (cl-every #'stringp before))
          (error "invalid original native compiler search path"))
        (let ((root (file-name-as-directory
                     (stable-suite--env "FEMACS_RUNTIME_CASE_ROOT"))))
          (setq target (file-name-as-directory
                        (expand-file-name "native-cache/compiler-output" root)))
          (make-directory target t)
          (unless (and (file-in-directory-p target root)
                       (not (member target before)))
            (error "compiler output is not a fresh owned target")))
        (setq native-comp-eln-load-path (append before (list target))))
      (setq stable-suite--compiler-output-context
            (list (cons "profile" "owned-original-compiler-output-v1")
                  (cons "scope" "test/src/comp-tests.el-only")
                  (cons "enabled" (if enabled t :json-false))
                  (cons "before" (vconcat before))
                  (cons "after" (vconcat (if enabled native-comp-eln-load-path before)))
                  (cons "target" target))))))

(defun stable-suite--compiler-output-snapshot ()
  (when stable-suite--compiler-output-context
    (append stable-suite--compiler-output-context
            (list (cons "actual_paths"
                        (vconcat (and (boundp 'native-comp-eln-load-path)
                                      native-comp-eln-load-path)))))))

(defun stable-suite--environment ()
  (append
   (list
   (cons "emacs_version" (emacs-version))
   (cons "product_version" emacs-version)
   (cons "noninteractive" (if noninteractive t :json-false))
   (cons "display_mode" (or (getenv "STABLE_SUITE_DISPLAY") "batch"))
   (cons "display_graphic" (if (display-graphic-p) t :json-false))
   (cons "display_images" (if (display-images-p) t :json-false))
   (cons "native_comp_available"
         (if (and (fboundp 'native-comp-available-p) (native-comp-available-p)) t :json-false))
   (cons "term" (getenv "TERM"))
   (cons "test_environment"
         (let ((values (make-hash-table :test #'equal)))
           (dolist (name '("ERC_TESTS_GRAPHICAL" "ERC_TESTS_GRAPHICAL_ALL"))
             (when-let* ((value (getenv name)))
               (puthash name value values)))
           values))
   (cons "system_configuration" system-configuration)
   (cons "configuration_features" system-configuration-features)
   (cons "collector_mode" (getenv "FEMACS_GC_COLLECTOR"))
   (cons "observer_mode" (getenv "FEMACS_GC_OBSERVATION"))
   (cons "mock_shell_environment" stable-suite--mock-shell-environment)
   (cons "compiler_output_context" (stable-suite--compiler-output-snapshot))
   (cons "system_type" (symbol-name system-type))
   (cons "window_system" (and (boundp 'window-system)
                              window-system
                              (format "%s" window-system)))
   (cons "features"
         (sort (mapcar #'symbol-name features) #'string<)))
   (when (member (getenv "STABLE_SUITE_DISPLAY") '("terminal" "cocoa"))
     (list (cons "terminal_name" (unless noninteractive (terminal-name)))
           (cons "native_comp_async_jobs"
                 (and (boundp 'native-comp-async-jobs-number)
                      native-comp-async-jobs-number))
           (cons "native_comp_thread_limit" (getenv "OMP_THREAD_LIMIT"))
           (cons "available_processors" (num-processors))))))

(defun stable-suite--excluded-ids ()
  "Return the driver-declared excluded ID list, or nil.
The value is a comma-separated environment list.  Only the driver sets it,
and only for a reduced second invocation of a file whose earlier run could
not continue past a blocking case."
  (let ((raw (getenv "STABLE_SUITE_EXCLUDE_IDS")))
    (and raw
         (not (equal raw ""))
         (split-string raw "," t))))

(defun stable-suite--registered-ids ()
  (let (ids)
    (dolist (test (ert-select-tests t t))
      (let ((name (ert-test-name test)))
        (when (and name (symbolp name))
          (push (symbol-name name) ids))))
    (sort (delete-dups ids) #'string<)))

(defun stable-suite--test-tags (id)
  (condition-case nil
      (mapcar #'symbol-name
              (ert-test-tags (ert-get-test (intern id))))
    (error nil)))

(defun stable-suite--write (payload)
  (let ((path (stable-suite--env "STABLE_SUITE_RESULT"))
        (coding-system-for-write 'utf-8-unix)
        (json-encoding-pretty-print nil))
    (with-temp-file path
      (insert (json-encode payload) "\n"))))

(defun stable-suite--discover ()
  (let* ((file (getenv "STABLE_SUITE_FILE"))
         (file-id (or (getenv "STABLE_SUITE_FILE_ID") file))
         (before (stable-suite--registered-ids))
         (status "loaded")
         (error-text nil))
    (condition-case error
        (if file
            (load file nil nil t)
          (error "STABLE_SUITE_FILE is not set"))
      (error
       (setq status "load-error"
             error-text (error-message-string error))))
    (let* ((after (stable-suite--registered-ids))
           (new (sort (cl-set-difference after before :test #'equal)
                      #'string<))
           (tests (mapcar
                   (lambda (id)
                     (list (cons "id" id)
                           (cons "tags" (vconcat (stable-suite--test-tags id)))))
                   new)))
      (stable-suite--write
       (append
        (stable-suite--environment)
        (list
         (cons "mode" "discover")
         (cons "file" file-id)
         (cons "status" status)
         (cons "before_ids" (vconcat before))
         (cons "after_ids" (vconcat after))
         (cons "new_ids" (vconcat new))
         (cons "tests" (vconcat tests))
         (cons "error" error-text)))))))

(defun stable-suite--result-record (id test result)
  (let* ((skipped (ert-test-result-type-p result :skipped))
         (expected (ert-test-result-expected-p test result))
         (status (cond (skipped "skipped")
                       (expected "passed")
                       (t "failed")))
         (condition
          (and (ert-test-result-with-condition-p result)
               (prin1-to-string
                (ert-test-result-with-condition-condition result))))
         (messages (ert-test-result-messages result))
         (duration (ert-test-result-duration result))
         (diagnostic
          (format "result=%s expected=%s condition=%s messages=%S"
                  (type-of result) expected condition messages)))
    (list
     (cons "test_id" id)
     (cons "status" status)
     (cons "duration" (if (numberp duration) duration nil))
     (cons "result_type" (symbol-name (type-of result)))
     (cons "expected" (if expected t :json-false))
     (cons "skipped" (if skipped t :json-false))
     (cons "condition" condition)
     (cons "messages" messages)
     (cons "diagnostic" diagnostic))))

(defun stable-suite--run-one (id record-cell)
  (let ((selector (intern id)))
    (let ((selected (ert-select-tests selector t)))
      (unless (and (= (length selected) 1)
                   (equal (symbol-name
                           (ert-test-name (car selected)))
                          id))
        (error "exact ID did not select exactly one test"))
      (ert-run-tests
       selector
       (lambda (event &rest args)
         (when (eq event 'test-ended)
           (let ((test (nth 1 args))
                 (result (nth 2 args)))
             (when (and test result
                        (equal (symbol-name (ert-test-name test)) id))
               (setcar record-cell
                       (stable-suite--result-record id test result))))))))))

(defun stable-suite--run ()
  (let* ((file (getenv "STABLE_SUITE_FILE"))
         (file-id (or (getenv "STABLE_SUITE_FILE_ID") file))
         (id (stable-suite--env "STABLE_SUITE_TEST"))
         (status "load-error")
         (error-text nil)
         (record-cell (list nil))
         record)
    (condition-case error
        (if file
            (load file nil nil t)
          (error "STABLE_SUITE_FILE is not set"))
      (error
       (setq error-text (error-message-string error))))
    (when (null error-text)
      (condition-case error
          (progn
            (stable-suite--run-one id record-cell)
            (setq record (car record-cell))
            (setq status (if record
                             (cdr (assoc "status" record))
                           "incomplete")))
        (error
         (setq status "run-error"
               error-text (error-message-string error)))))
    (stable-suite--write
     (append
      (stable-suite--environment)
      (list
       (cons "mode" "run")
       (cons "file" file-id)
       (cons "test_id" id)
       (cons "status" status)
       (cons "error" error-text)
       (cons "record" record))))))

(defun stable-suite--event (payload)
  "Append one complete progress record outside the disposable case state."
  (let ((coding-system-for-write 'utf-8-unix)
        (json-encoding-pretty-print nil))
    (with-temp-buffer
      ;; json-encode uses delete-char, which calls undo-auto-amalgamate even
      ;; in this undo-disabled buffer. Do not leak its command flag into
      ;; the next original test or let it amalgamate another buffer's undo.
      (let ((undo-auto--this-command-amalgamating nil)
            (undo-auto--last-boundary-cause nil))
        (insert (json-encode payload) "\n")
        (write-region (point-min) (point-max)
                      (stable-suite--env "STABLE_SUITE_PROGRESS") t 'silent)))))

(defvar stable-suite--diagnostic-timer nil)
(defvar stable-suite--diagnostic-id nil)
(defvar stable-suite--diagnostic-bytes 0)
(defvar stable-suite--diagnostic-child nil)
(defvar stable-suite--diagnostic-child-result nil)

(defun stable-suite--diagnostic-snapshot (phase)
  "Record bounded observations without changing JIT, tests, or debugger policy."
  (let ((path (getenv "STABLE_SUITE_DIAGNOSTIC")))
    (when path
      (condition-case nil
          (let ((coding-system-for-write 'utf-8-unix)
                (json-encoding-pretty-print nil)
                (buffers nil)
                (functions nil))
            (dolist (symbol '(native-compile comp--final erc-speedbar--ensure
                             erc-speedbar--emulate-sidebar tramp-process-actions))
              (when (fboundp symbol)
                (let ((definition (symbol-function symbol)))
                  (push (list (cons "symbol" (symbol-name symbol))
                              (cons "origin" (symbol-file symbol 'defun))
                              (cons "byte_code" (if (byte-code-function-p definition) t :json-false))
                              (cons "native" (if (and (fboundp 'native-comp-function-p)
                                                       (native-comp-function-p definition))
                                                  t :json-false))) functions))))
            (dolist (buffer (buffer-list))
              (when (and (< (length buffers) 4)
                         (buffer-live-p buffer)
                         (with-current-buffer buffer
                           (or (derived-mode-p 'term-mode)
                               (equal (buffer-name) "*Backtrace*"))))
                (with-current-buffer buffer
                  (push (list (cons "name" (buffer-name))
                              (cons "tail" (buffer-substring-no-properties
                                            (max (point-min) (- (point-max) 4096))
                                            (point-max)))) buffers))))
            (with-temp-buffer
              (insert
               (json-encode
                (list (cons "schema" "femacs-stable-runtime-observation-v1")
                      (cons "file" (getenv "STABLE_SUITE_FILE_ID"))
                      (cons "test_id" stable-suite--diagnostic-id)
                      (cons "phase" phase)
                      (cons "pid" (emacs-pid))
                      (cons "nested_terminal_child" stable-suite--diagnostic-child)
                      (cons "child_test_result" stable-suite--diagnostic-child-result)
                      (cons "native_comp_jit" (if (bound-and-true-p native-comp-jit-compilation) t :json-false))
                      (cons "native_comp_async_jobs"
                            (and (boundp 'native-comp-async-jobs-number) native-comp-async-jobs-number))
                      (cons "native_comp_eln_load_path"
                            (vconcat (and (boundp 'native-comp-eln-load-path)
                                          (cl-subseq native-comp-eln-load-path 0
                                            (min 16 (length native-comp-eln-load-path))))))
                      (cons "functions" (vconcat functions))
                      (cons "buffers" (vconcat buffers)))) "\n")
              (when (and (<= (string-bytes (buffer-string)) 65536)
                         (<= (+ (max stable-suite--diagnostic-bytes
                                     (or (file-attribute-size (file-attributes path)) 0))
                                (string-bytes (buffer-string))) (* 8 1024 1024)))
                (setq stable-suite--diagnostic-bytes
                      (+ stable-suite--diagnostic-bytes (string-bytes (buffer-string))))
                (write-region (point-min) (point-max) path t 'silent))))
        ;; Observation failure never supplies a test outcome or extends a deadline.
        (error nil)))))

(defun stable-suite--diagnostic-cancel ()
  (when (timerp stable-suite--diagnostic-timer)
    (cancel-timer stable-suite--diagnostic-timer))
  (setq stable-suite--diagnostic-timer nil))

(defun stable-suite--diagnostic-start (id)
  (stable-suite--diagnostic-cancel)
  (setq stable-suite--diagnostic-id id)
  (when (getenv "STABLE_SUITE_DIAGNOSTIC")
    (stable-suite--diagnostic-snapshot "case-started")
    (let* ((default (string-to-number (or (getenv "STABLE_SUITE_TIMEOUT") "300")))
           (budget (if (equal id (getenv "STABLE_SUITE_DIAGNOSTIC_SPECIAL_CASE"))
                       (string-to-number (getenv "STABLE_SUITE_DIAGNOSTIC_SPECIAL_SECONDS"))
                     default)))
      (setq stable-suite--diagnostic-timer
            (run-at-time (max 0.1 (- budget 5)) nil
                         #'stable-suite--diagnostic-snapshot "before-case-deadline")))))

(defun stable-suite--machine-debugger (original &rest args)
  "Deliver an original error to ERT instead of waiting for human input.
Some terminal scenario helpers bind debug-on-error and call debugger
directly.  Do not suppress or replace their condition."
  (if (and (eq (car args) 'error) (consp (cadr args))
           (symbolp (car (cadr args))))
      (let ((debug-on-error nil) (inhibit-debugger t))
        (signal (car (cadr args)) (cdr (cadr args))))
    (apply original args)))

(defconst stable-suite--erc-terminal-files
  '("test/lisp/erc/erc-scenarios-status-sidebar.el"
    "test/lisp/erc/erc-scenarios-keep-place-indicator.el"
    "test/lisp/erc/erc-scenarios-keep-place-indicator-trunc.el"
    "test/lisp/erc/erc-scenarios-scrolltobottom.el"
    "test/lisp/erc/erc-scenarios-scrolltobottom-relaxed.el"))

(defun stable-suite--erc-error-scope-p ()
  (and (equal (getenv "STABLE_SUITE_MACHINE_ERROR_DELIVERY")
              "erc-original-terminal-errors-v1")
       (member (getenv "STABLE_SUITE_FILE_ID") stable-suite--erc-terminal-files)))

(defun stable-suite--erc-child-debugger (original &rest args)
  "Preserve ERT errors and make startup failure observable to its parent."
  (if (and (eq (car args) 'error) (consp (cadr args))
           (symbolp (car (cadr args))))
      (if (ignore-errors (ert-running-test))
          (let ((debug-on-error nil) (inhibit-debugger t))
            (signal (car (cadr args)) (cdr (cadr args))))
        (message "Original terminal child startup error: %S" (cadr args))
        (kill-emacs 2))
    (apply original args)))

(defun stable-suite--erc-observe-test (original test)
  "Record the original terminal child's actual ERT result without replacing it."
  (let ((result (funcall original test)))
    (condition-case nil
        (let* ((id (symbol-name (ert-test-name test)))
               (record (stable-suite--result-record id test result))
               (stable-suite--diagnostic-id id)
               (stable-suite--diagnostic-child-result
                (list (cons "program" (getenv "STABLE_SUITE_PRODUCT_PROGRAM"))
                      (cons "dump" (getenv "STABLE_SUITE_PRODUCT_DUMP"))
                      (cons "test_id" id)
                      (cons "status" (cdr (assoc "status" record)))
                      (cons "condition" (cdr (assoc "condition" record))))))
          (stable-suite--diagnostic-snapshot "original-terminal-child-test-ended"))
      (error nil))
    result))

(defun stable-suite--erc-child-setup ()
  "Set machine error delivery in the original helper's fresh terminal child."
  (unless (and (stable-suite--erc-error-scope-p)
               (equal (getenv "ERC_TESTS_SUBPROCESS") "1")
               (not noninteractive) (null window-system))
    (error "invalid original terminal child error-delivery scope"))
  ;; Match the already-declared owned interactive-editor worker budget.  The
  ;; original helper reconstructs argv rather than inheriting parent evals.
  (setq native-comp-async-jobs-number 1
        debugger (apply-partially #'stable-suite--erc-child-debugger debugger))
  (unless (advice-member-p #'stable-suite--erc-observe-test 'ert-run-test)
    (advice-add 'ert-run-test :around #'stable-suite--erc-observe-test))
  (stable-suite--diagnostic-snapshot "original-terminal-child-ready"))

(defun stable-suite--erc-make-term (original name program startfile &rest switches)
  "Keep the original terminal helper and bind its selected product inputs."
  (if (not (equal (getenv "ERC_TESTS_SUBPROCESS") "1"))
      (apply original name program startfile switches)
    (let ((selected (getenv "STABLE_SUITE_PRODUCT_PROGRAM"))
          (dump (getenv "STABLE_SUITE_PRODUCT_DUMP")))
      (unless (and noninteractive (stable-suite--erc-error-scope-p)
                   (null startfile) selected dump
                   (file-equal-p selected program)
                   (string-prefix-p "--dump-file=" dump)
                   (file-regular-p (substring dump (length "--dump-file=")))
                   (member "-nw" switches)
                   (member (getenv "STABLE_SUITE_FILE") switches)
                   (not (cl-some (lambda (arg) (string-prefix-p "--dump-file=" arg))
                                 switches))
                   stable-suite--runner-file
                   (file-regular-p stable-suite--runner-file))
        (error "original terminal helper product or arguments differ"))
      (let* ((process-environment (cons "OMP_THREAD_LIMIT=1" process-environment))
             (bound (append (list dump "-l" stable-suite--runner-file
                                 "-f" "stable-suite--erc-child-setup") switches))
             (buffer (apply original name program startfile bound))
             (process (get-buffer-process buffer)))
        (setq stable-suite--diagnostic-child
              (list (cons "name" name) (cons "program" program)
                    (cons "argv" (vconcat bound))
                    (cons "pid" (and process (process-id process)))))
        (stable-suite--diagnostic-snapshot "original-terminal-child-started")
        buffer))))

(defun stable-suite--batch-ert ()
  "Use original per-file ERT execution, selecting every discovered ID."
  (let* ((file (stable-suite--env "STABLE_SUITE_FILE"))
         (file-id (stable-suite--env "STABLE_SUITE_FILE_ID"))
         (before (stable-suite--registered-ids))
         ids
         (ended 0)
         (status "complete")
         error-text)
    (condition-case error
        (progn
          (stable-suite--prepare-compiler-output)
          (stable-suite--diagnostic-snapshot "before-load")
          (load file nil nil t)
          (setq ids (sort (cl-set-difference
                           (stable-suite--registered-ids) before :test #'equal)
                          #'string<))
          ;; A case that wedges the process inside a blocking system call
          ;; cannot be resumed by any later work in the same process, so the
          ;; affected file needs a second, explicitly reduced invocation.  The
          ;; exclusion is declared by the driver, echoed here, and never
          ;; invented by the runner: the original case set stays discoverable
          ;; and the reduced run reports exactly what it asked for.
          (let ((excluded (stable-suite--excluded-ids)))
            (when excluded
              (setq excluded (cl-remove-if-not
                              (lambda (id) (member id ids)) excluded))
              (setq ids (cl-remove-if (lambda (id) (member id excluded)) ids))
              (stable-suite--event
               (list (cons "event" "excluded")
                     (cons "file" file-id)
                     (cons "excluded" (vconcat excluded))))))
          (stable-suite--event
           (append (stable-suite--environment)
                   (list (cons "event" "loaded") (cons "file" file-id)
                         (cons "selector" "all-discovered-ids")
                         (cons "ids" (vconcat ids))
                         (cons "tests"
                               (vconcat (mapcar
                                (lambda (id) (list (cons "id" id)
                                                   (cons "tags" (vconcat
                                                    (stable-suite--test-tags id)))))
                                ids))))))
          (when ids
            (ert-run-tests
             (cons 'member (mapcar #'intern ids))
             (lambda (event &rest args)
               (when (memq event '(test-started test-ended))
                 (let* ((test (nth 1 args))
                        (id (and test (symbol-name (ert-test-name test)))))
                   (unless (member id ids)
                     (error "ERT emitted an undiscovered test identity: %S" id))
                   (if (eq event 'test-started)
                       (progn
                         (stable-suite--diagnostic-start id)
                         (stable-suite--event
                          (list (cons "event" "test-started")
                                (cons "file" file-id) (cons "test_id" id))))
                     (let ((result (nth 2 args)))
                       (stable-suite--diagnostic-cancel)
                       (unless result (error "ERT ended without a result"))
                       (setq ended (1+ ended))
                       (stable-suite--event
                        (append (list (cons "event" "test-ended")
                                      (cons "file" file-id))
                                (stable-suite--result-record id test result)))))))))))
      (error (setq status "run-error" error-text (error-message-string error))))
    (stable-suite--diagnostic-cancel)
    (unless (= ended (length ids)) (setq status "incomplete"))
    (stable-suite--event
     (list (cons "event" "finished") (cons "file" file-id)
           (cons "compiler_output_context" (stable-suite--compiler-output-snapshot))
           (cons "status" status) (cons "error" error-text)
           (cons "requested_count" (length ids)) (cons "ended_count" ended)))))

(defun stable-suite--batch ()
  "Keep original test isolation and deliver terminal errors to the machine."
  (let* ((enabled (getenv "STABLE_SUITE_MACHINE_ERROR_DELIVERY"))
         (debugger (if enabled
                       (apply-partially #'stable-suite--machine-debugger debugger)
                     debugger)))
    (when enabled
      (unless (and (stable-suite--erc-error-scope-p)
                   (member (getenv "STABLE_SUITE_DISPLAY") '(nil "terminal" "cocoa")))
        (error "invalid original terminal scenario error-delivery scope")))
    (unwind-protect
        (progn
          (when (and enabled noninteractive)
            (advice-add 'make-term :around #'stable-suite--erc-make-term))
          (stable-suite--batch-ert))
      (when (and enabled noninteractive)
        (advice-remove 'make-term #'stable-suite--erc-make-term)))))

(defun stable-suite-main ()
  (let ((mode (stable-suite--env "STABLE_SUITE_MODE")))
    (cond
     ((equal mode "discover") (stable-suite--discover))
     ((equal mode "run") (stable-suite--run))
     ((equal mode "batch") (stable-suite--batch))
     (t (error "unknown STABLE_SUITE_MODE: %s" mode)))))

(provide 'stable-suite-runner)
