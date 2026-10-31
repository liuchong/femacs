;;; stable-suite-runner.el --- machine bridge for the stable corpus -*- lexical-binding: t; -*-

;; Loaded into an already-built product by tools/stable-suite.py.  It writes
;; machine-readable discovery/run records to STABLE_SUITE_RESULT and leaves all
;; human output on the product's stdout/stderr.  It does not choose tests,
;; filter platform/features, or claim compatibility.

(defvar find-function-C-source-directory)
(let ((context (getenv "STABLE_SUITE_SOURCE_CONTEXT"))
      (product-lisp (getenv "STABLE_SUITE_PRODUCT_LISP")))
  (when context
    (unless (and product-lisp (file-directory-p context)
                 (file-directory-p (expand-file-name "lisp" context)))
      (error "missing bound source-test context"))
    (let ((original (file-name-as-directory product-lisp))
          (overlay (file-name-as-directory (expand-file-name "lisp" context))))
      (setq load-path
            (cons overlay
                  (mapcar (lambda (path)
                            (if (and (stringp path)
                                     (string-prefix-p original
                                       (file-name-as-directory path)))
                                (concat overlay
                                  (substring (file-name-as-directory path)
                                             (length original)))
                              path))
                          load-path))))
    (setq source-directory (file-name-as-directory context)
          find-function-C-source-directory (expand-file-name "src/" context))))

(require 'ert)
(require 'json)
(require 'cl-lib)

;; Only the owned Darwin mock fixtures request this shell initialization.
;; Tramp clears ENV when re-execing its remote shell, so the process ENV alone
;; is insufficient.  Use its explicit remote environment with shell quoting.
(defvar stable-suite--mock-shell-environment nil)
(let ((script (getenv "FEMACS_STABLE_MOCK_SHELL_ENV")))
  (when script
    (unless (and (eq system-type 'darwin)
                 (member (getenv "STABLE_SUITE_FILE_ID")
                         '("test/lisp/autorevert-tests.el"
                           "test/lisp/filenotify-tests.el"))
                 (equal (getenv "ENV") script)
                 (file-regular-p script)
                 (not (file-symlink-p script))
                 (file-in-directory-p script
                                      (getenv "FEMACS_RUNTIME_CASE_ROOT")))
      (error "invalid owned mock shell initialization"))
    (with-eval-after-load 'tramp
      (let ((content (with-temp-buffer
                       (insert-file-contents-literally script)
                       (buffer-string))))
        (unless (equal content "case $- in\n  *m*) set +m ;;\nesac\n")
          (error "mock shell initialization content differs"))
        (setq tramp-remote-process-environment
              (cons (concat "ENV=" (shell-quote-argument script))
                    (cl-remove-if (lambda (value)
                                    (string-prefix-p "ENV=" value))
                                  tramp-remote-process-environment)))
        (setq stable-suite--mock-shell-environment
              (list (cons "content_sha256" (secure-hash 'sha256 content))
                    (cons "file" script)
                    (cons "process_env" (getenv "ENV"))
                    (cons "remote_env"
                          (car tramp-remote-process-environment))))))))

(defun stable-suite--env (name)
  (or (getenv name)
      (error "missing environment variable %s" name)))

(defun stable-suite--environment ()
  (append
   (list
   (cons "emacs_version" (emacs-version))
   (cons "system_configuration" system-configuration)
   (cons "configuration_features" system-configuration-features)
   (cons "collector_mode" (getenv "FEMACS_GC_COLLECTOR"))
   (cons "observer_mode" (getenv "FEMACS_GC_OBSERVATION"))
   (cons "mock_shell_environment" stable-suite--mock-shell-environment)
   (cons "system_type" (symbol-name system-type))
   (cons "window_system" (and (boundp 'window-system)
                              window-system
                              (format "%s" window-system)))
   (cons "features"
         (sort (mapcar #'symbol-name features) #'string<)))
   (when (equal (getenv "STABLE_SUITE_DISPLAY") "terminal")
     (list (cons "noninteractive" (if noninteractive t :json-false))
           (cons "terminal_name" (unless noninteractive (terminal-name)))
           (cons "native_comp_async_jobs"
                 (and (boundp 'native-comp-async-jobs-number)
                      native-comp-async-jobs-number))
           (cons "native_comp_thread_limit" (getenv "OMP_THREAD_LIMIT"))
           (cons "available_processors" (num-processors))))))

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

(defun stable-suite--batch ()
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
          (load file nil nil t)
          (setq ids (sort (cl-set-difference
                           (stable-suite--registered-ids) before :test #'equal)
                          #'string<))
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
                       (stable-suite--event
                        (list (cons "event" "test-started")
                              (cons "file" file-id) (cons "test_id" id)))
                     (let ((result (nth 2 args)))
                       (unless result (error "ERT ended without a result"))
                       (setq ended (1+ ended))
                       (stable-suite--event
                        (append (list (cons "event" "test-ended")
                                      (cons "file" file-id))
                                (stable-suite--result-record id test result)))))))))))
      (error (setq status "run-error" error-text (error-message-string error))))
    (unless (= ended (length ids)) (setq status "incomplete"))
    (stable-suite--event
     (list (cons "event" "finished") (cons "file" file-id)
           (cons "status" status) (cons "error" error-text)
           (cons "requested_count" (length ids)) (cons "ended_count" ended)))))

(defun stable-suite-main ()
  (let ((mode (stable-suite--env "STABLE_SUITE_MODE")))
    (cond
     ((equal mode "discover") (stable-suite--discover))
     ((equal mode "run") (stable-suite--run))
     ((equal mode "batch") (stable-suite--batch))
     (t (error "unknown STABLE_SUITE_MODE: %s" mode)))))

(provide 'stable-suite-runner)
