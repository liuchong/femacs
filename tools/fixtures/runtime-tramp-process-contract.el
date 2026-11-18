;;; runtime-tramp-process-contract.el --- Owned remote-process observations -*- lexical-binding: t; -*-

;; Authored signal controls are separate from the two unchanged original tests.
;; A diagnostic pass never rewrites or supersedes a formal original result.
(require 'cl-lib)
(require 'json)
(require 'ert)
(require 'tramp)
(require 'ert-x)

(defvar runtime-tramp--trace nil)
(defvar runtime-tramp--rows nil)
(defvar runtime-tramp--mock-ps-bin nil)

(defun runtime-tramp--mock-ps (args)
  (if (and runtime-tramp--mock-ps-bin
           (equal (car args) "ps")
           (equal (file-remote-p default-directory 'method) "mock"))
      (cons runtime-tramp--mock-ps-bin (cdr args))
    args))

(defun runtime-tramp--text (value &optional limit)
  (let ((print-length 12) (print-level 5) (print-circle t))
    (truncate-string-to-width (format "%S" value) (or limit 2048) nil nil "...[truncated]")))

(defun runtime-tramp--process-file (original program &rest arguments)
  "Observe ORIGINAL without changing its arguments, output or errors."
  (let ((started (float-time)) result)
    (unwind-protect
        (setq result (apply original program arguments))
      (when (< (length runtime-tramp--trace) 128)
        (let* ((destination (nth 1 arguments))
               (buffer (cond ((bufferp destination) destination)
                             ((eq destination t) (current-buffer)))))
          (push `((program . ,(runtime-tramp--text program 256))
                  (arguments . ,(runtime-tramp--text arguments 1024))
                  (result . ,(runtime-tramp--text result 256))
                  (seconds . ,(- (float-time) started))
                  (output_tail . ,(if (buffer-live-p buffer)
                                      (with-current-buffer buffer
                                        (buffer-substring-no-properties
                                         (max (point-min) (- (point-max) 2048)) (point-max)))
                                    "")))
                runtime-tramp--trace))))))

(defun runtime-tramp--original (id)
  (let* ((started (float-time))
         (runtime-tramp--trace nil)
         result)
    (unwind-protect
        (progn
          (advice-add 'process-file :around #'runtime-tramp--process-file)
          (ert-run-tests id
                         (lambda (event &rest arguments)
                           (when (eq event 'test-ended)
                             (setq result (nth 2 arguments))))))
      (advice-remove 'process-file #'runtime-tramp--process-file))
    (unless result (error "original ERT result was not delivered"))
    `((id . ,(symbol-name id))
      (status . ,(cond ((ert-test-passed-p result) "passed")
                      ((ert-test-skipped-p result) "skipped") (t "failed")))
      (seconds . ,(- (float-time) started))
      (condition . ,(if (ert-test-result-with-condition-p result)
                       (runtime-tramp--text (ert-test-result-with-condition-condition result) 4096)
                     "nil"))
      (process_file_trace . ,(vconcat (nreverse runtime-tramp--trace))))))

(defun runtime-tramp--wait (process predicate deadline)
  (while (and (not (funcall predicate)) (< (float-time) deadline))
    (accept-process-output process 0.02 nil t))
  (funcall predicate))

(defun runtime-tramp--signal (kind signal)
  (let* ((started (float-time))
         (default-directory (file-truename ert-remote-temporary-file-directory))
         (buffer (generate-new-buffer " *runtime-tramp-signal*"))
         (row `((argument_kind . ,(symbol-name kind)) (signal . ,(format "%s" signal))))
         process)
    (unwind-protect
        (condition-case failure
            (with-current-buffer buffer
              ;; This command cannot finish naturally. The marker and exit17
              ;; demonstrate that the installed handler received the signal.
              (setq process (start-file-process-shell-command
                             (format "runtime-tramp-%s-%s" kind signal) buffer
                             (concat "trap 'printf \"CONTRACT-SIGNAL\\n\"; exit 17' 2; "
                                     "printf 'CONTRACT-READY\\n'; "
                                     "while :; do sleep 0.05; done")))
              (unless (and (numberp (process-get process 'remote-pid))
                           (runtime-tramp--wait process
                            (lambda () (string-match-p "CONTRACT-READY\n" (buffer-string)))
                            (+ (float-time) 4)))
                (error "remote signal handler did not become ready"))
              (let* ((remote-pid (process-get process 'remote-pid))
                     (argument (pcase kind
                                 ('process process) ('name (process-name process))
                                 ('pid remote-pid) ('pid-string (number-to-string remote-pid))))
                     (sent (float-time))
                     (code (if (memq kind '(pid pid-string))
                               (signal-process argument signal default-directory)
                             (signal-process argument signal)))
                     (drains 0))
                (while (and (< drains 64) (accept-process-output process 0 nil t))
                  (setq drains (1+ drains)))
                (setq row (append row `((ready . t) (signal_return . ,code)
                                        (nonblocking_drains . ,drains)
                                        (live_after_nonblocking_drain . ,(if (process-live-p process) t :json-false)))))
                (unless (and (equal code 0)
                             (runtime-tramp--wait process (lambda () (not (process-live-p process)))
                                                 (+ sent 4)))
                  (error "remote process did not exit after delivered signal"))
                (accept-process-output process 0 nil t)
                (unless (and (= (process-exit-status process) 17)
                             (string-match-p "CONTRACT-SIGNAL\n" (buffer-string)))
                  (error "remote signal handler marker or exit status differs"))
                (setq row (append row `((passed . t) (exit_status . ,(process-exit-status process))
                                        (signal_to_exit_seconds . ,(- (float-time) sent)))))))
          (error (setq row (append row `((passed . :json-false)
                                         (condition . ,(runtime-tramp--text failure)))))))
      (when (processp process)
        (when (process-live-p process) (ignore-errors (kill-process process)))
        (ignore-errors (delete-process process)))
      (setq row (append row `((output . ,(with-current-buffer buffer
                                           (buffer-substring-no-properties
                                            (max (point-min) (- (point-max) 2048)) (point-max))))
                              (seconds . ,(- (float-time) started)))))
      (kill-buffer buffer))
    row))

(defun runtime-tramp-process-contract-run ()
  (let ((destination (getenv "FEMACS_TRAMP_CONTRACT_RESULT"))
        (original-file (getenv "FEMACS_TRAMP_ORIGINAL_TEST"))
        (started (float-time)) original watch remote-context error-value)
    (unwind-protect
        (condition-case failure
            (progn
              (let* ((script (getenv "FEMACS_STABLE_MOCK_SHELL_ENV"))
                     (mock-bin (getenv "FEMACS_STABLE_MOCK_BIN"))
                     (ps-bin (and mock-bin (expand-file-name "ps" mock-bin))))
                (unless (and script (file-regular-p script) (equal script (getenv "ENV")))
                  (error "owned mock shell environment missing"))
                (when (and ps-bin (file-regular-p ps-bin) (file-executable-p ps-bin)
                           (zerop (logand (or (file-modes ps-bin) #o6000) #o6000)))
                  (setq runtime-tramp--mock-ps-bin ps-bin)
                  (with-eval-after-load 'tramp-sh
                    (advice-add 'tramp-sh-handle-process-file :filter-args #'runtime-tramp--mock-ps)))
                (setq tramp-remote-process-environment
                      (cons (concat "ENV=" (shell-quote-argument script))
                            (cl-remove-if (lambda (item) (string-prefix-p "ENV=" item))
                                          tramp-remote-process-environment))))
              (unless (and (equal (file-remote-p ert-remote-temporary-file-directory 'method) "mock")
                           (getenv "TMPDIR")
                           (equal (directory-file-name (file-local-name ert-remote-temporary-file-directory))
                                  (directory-file-name (getenv "TMPDIR"))))
                (error "remote controls require the owned mock temporary directory"))
              (setq remote-context `((method . "mock")
                                     (local_directory . ,(directory-file-name (getenv "TMPDIR")))))
              (load original-file nil t t)
              (setq original (mapcar #'runtime-tramp--original
                                     '(tramp-test28-process-file tramp-test31-signal-process)))
              (dolist (kind '(process name pid pid-string))
                (dolist (signal '(2 INT))
                  (push (runtime-tramp--signal kind signal) runtime-tramp--rows)))
              (load (getenv "FEMACS_REMOTE_WATCH_FIXTURE") nil t t)
              (setq watch (runtime-remote-watch-contract-run)))
          (error (setq error-value (runtime-tramp--text failure 4096))))
      (ignore-errors (advice-remove 'tramp-sh-handle-process-file #'runtime-tramp--mock-ps))
      (ignore-errors (tramp-cleanup-all-connections)))
    (let ((coding-system-for-write 'utf-8-unix))
      (with-temp-file destination
        (insert (json-encode
                 `((schema . "femacs-tramp-process-contract-v1")
                   (contract . "ready-remote-signal-delivery-v1")
                   (collection_complete . ,(if (and (not error-value) (= (length runtime-tramp--rows) 8)
                                                    (= (length original) 2)
                                                    (eq (alist-get 'collection_complete watch) t)) t :json-false))
                   (signal_contract_passed . ,(if (and (= (length runtime-tramp--rows) 8)
                                                      (cl-every (lambda (row) (eq (alist-get 'passed row) t))
                                                                runtime-tramp--rows)) t :json-false))
                   (original_cases . ,(vconcat original))
                   (watch_contract . ,watch)
                   (owned_remote_context . ,remote-context)
                   (signal_rows . ,(vconcat (nreverse runtime-tramp--rows)))
                   (error . ,error-value)
                   (library_files . ,(vector (symbol-file 'tramp-signal-process 'defun)
                                              (symbol-file 'tramp-sh-handle-process-file 'defun)))
                   (seconds . ,(- (float-time) started)))))
        (insert "\n")))
    (kill-emacs (if error-value 1 0))))

;;; runtime-tramp-process-contract.el ends here
