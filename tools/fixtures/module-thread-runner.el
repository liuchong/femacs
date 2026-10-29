;;; module-thread-runner.el --- owned stable module/thread runner -*- lexical-binding: t; -*-

(require 'json)
(require 'ert)
(require 'cl-lib)

(defun femacs-module-thread--read (file)
  (let ((json-object-type 'alist)
        (json-key-type 'symbol)
        (json-array-type 'list)
        (json-false nil))
    (json-read-file file)))

(defun femacs-module-thread--write (file value)
  (let ((coding-system-for-write 'utf-8-unix))
    (with-temp-file file
      (insert (json-encode value))
      (insert "\n"))))

(defun femacs-module-thread--metadata ()
  `((version . ,emacs-version)
    (system_type . ,(symbol-name system-type))
    (configuration . ,system-configuration)
    (configuration_features . ,system-configuration-features)
    (load_path . ,(vconcat load-path))
    (data_directory . ,data-directory)
    (exec_directory . ,exec-directory)
    (features . ,(vconcat (sort (mapcar #'symbol-name features) #'string<)))
    (module_file_suffix . ,(or module-file-suffix ""))
    (load_suffixes . ,(vconcat load-suffixes))
    (native_available . ,(if (and (fboundp 'native-comp-available-p)
                                  (native-comp-available-p))
                             t
                           :json-false))))

(let* ((request-file (getenv "FEMACS_MODULE_THREAD_REQUEST"))
       (result-file (getenv "FEMACS_MODULE_THREAD_RESULT"))
       (request (femacs-module-thread--read request-file))
       (operation (alist-get 'operation request))
       (file (alist-get 'file request))
       (invocation-directory-value (alist-get 'invocation_directory request))
       (invocation-name-value (alist-get 'invocation_name request))
       (test-id (alist-get 'id request))
       (reply `((schema . "femacs-module-thread-process-v1")
                (operation . ,operation)
                (file . ,file)))
       (started (float-time))
       (exit-code 0))

  (when invocation-directory-value
    (setq invocation-directory invocation-directory-value))
  (when invocation-name-value
    (setq invocation-name invocation-name-value))

  (condition-case-unless-debug failure
      (progn
        (load file nil t t)
        (setq reply (append reply
                            (list (cons 'metadata (femacs-module-thread--metadata)))))
        (pcase operation
          ("discover"
           (let* ((tests (ert-select-tests t t))
                  (ids (sort (delete-dups
                              (mapcar (lambda (test)
                                        (symbol-name (ert-test-name test)))
                                      tests))
                             #'string<)))
             (setq reply (append reply (list (cons 'ids (vconcat ids)))))))
          ("ert"
           (let* ((test (ert-get-test (intern test-id)))
                  (begin (float-time))
                  (result (ert-run-test test))
                  (seconds (- (float-time) begin))
                  (status (cond ((ert-test-passed-p result) "passed")
                                ((ert-test-skipped-p result) "skipped")
                                (t "failed")))
                  (duration (ert-test-result-duration result)))
             (setq reply
                   (append reply
                           `((id . ,test-id)
                             (status . ,status)
                             (seconds . ,(if (numberp duration) duration seconds))
                             (wall_seconds . ,seconds)
                             (result_type . ,(symbol-name (type-of result)))
                             (diagnostic . ,(prin1-to-string result))
                             (metadata . ,(femacs-module-thread--metadata))))))
          (_
           (error "unknown module/thread operation: %S" operation)))
        (setq reply (append reply '((complete . t)))))
    (error
     (setq exit-code 1)
     (setq reply
           (append reply
                   `((complete . :json-false)
                     (status . "load-or-process-error")
                     (error_symbol . ,(symbol-name (car failure)))
                     (diagnostic . ,(prin1-to-string failure)))))))

  (setq reply (append reply `((wall_seconds . ,(- (float-time) started)))))
  (femacs-module-thread--write result-file reply)
  (kill-emacs exit-code))

;;; module-thread-runner.el ends here
