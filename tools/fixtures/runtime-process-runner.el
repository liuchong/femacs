;;; runtime-process-runner.el --- Isolated runtime process protocol -*- lexical-binding: t; -*-
;; The Python owner supplies only operation/fixture/selector/ID/path inputs.
;; Results come from the running product. No expected test answers are inputs.
(require 'json)
(require 'ert)
(require 'cl-lib)

(defun femacs-runtime-process--read (file)
  (let ((json-object-type 'alist) (json-key-type 'symbol)
        (json-array-type 'list) (json-false nil))
    (json-read-file file)))

(defun femacs-runtime-process--write (file result)
  (let ((coding-system-for-write 'utf-8-unix))
    (with-temp-file file (insert (json-encode result)) (insert "\n"))))

(defun femacs-runtime-process--diagnostic (value)
  ;; Bound the printer itself: successful assertion histories and large error
  ;; strings must never first become an unlimited intermediate JSON string.
  (let ((print-length 16) (print-level 8) (print-circle t)
        (characters nil) (count 0) (truncated nil))
    (catch 'femacs-runtime-diagnostic-limit
      (prin1 value
             (lambda (character)
               (when (>= count 4096)
                 (setq truncated t)
                 (throw 'femacs-runtime-diagnostic-limit nil))
               (push character characters)
               (setq count (1+ count)))))
    (concat (apply #'string (nreverse characters))
            (if truncated " ...[truncated]" ""))))

(defun femacs-runtime-process--metadata ()
  `((version . ,emacs-version) (system_type . ,(symbol-name system-type))
    (configuration . ,system-configuration)
    (configuration_features . ,system-configuration-features)
    (load_path . ,(vconcat load-path))
    (data_directory . ,data-directory) (exec_directory . ,exec-directory)
    (features . ,(vconcat (sort (mapcar #'symbol-name features) #'string<)))
    (native_available . ,(if (and (fboundp 'native-comp-available-p)
                                 (native-comp-available-p)) t :json-false))))

(defun femacs-runtime-process--observations ()
  ;; Detach after a case, outside its measured ERT body. These snapshots are
  ;; diagnostics; their own allocations are not attributed to workload timing.
  `((language . ,(when (boundp 'femacs-runtime-language--observations)
                   (vconcat (mapcar (lambda (row) (vector (car row) (cdr row)))
                                    femacs-runtime-language--observations))))
    (collector . ,(when (fboundp 'femacs--gc-collector-status)
                    (femacs--gc-collector-status)))
    (observer . ,(when (fboundp 'femacs--gc-snapshot)
                   (femacs--gc-snapshot)))))

(let* ((request-file (getenv "FEMACS_RUNTIME_REQUEST"))
       (result-file (getenv "FEMACS_RUNTIME_RESULT"))
       (request (femacs-runtime-process--read request-file))
       (operation (alist-get 'operation request))
       (reply `((schema . "femacs-runtime-process-v1") (operation . ,operation)))
       (started (float-time))
       (exit-code 1))
  (condition-case-unless-debug failure
      (progn
        (dolist (fixture (alist-get 'fixtures request)) (load fixture nil t t))
        (setq reply (append reply (list (cons 'metadata (femacs-runtime-process--metadata)))))
        (cond
         ((equal operation "discover")
          (let ((tests (ert-select-tests (alist-get 'selector request) t)))
            (setq reply (append reply
                                (list (cons 'ids (vconcat (sort (mapcar (lambda (test)
                                                                        (symbol-name (ert-test-name test)))
                                                                      tests) #'string<))))))
            (setq exit-code 0)))
         ((equal operation "ert")
          (let* ((id (alist-get 'id request))
                 (test (ert-get-test (intern id)))
                 (begin (float-time))
                 (result (ert-run-test test))
                 (seconds (- (float-time) begin))
                 (status (cond ((ert-test-passed-p result) "passed")
                               ((ert-test-skipped-p result) "skipped")
                               (t "failed"))))
            (setq reply (append reply
                                `((id . ,id) (status . ,status) (seconds . ,seconds)
                                  (result_type . ,(symbol-name (type-of result)))
                                  (diagnostic . ,(if (equal status "passed") ""
                                                  (femacs-runtime-process--diagnostic
                                                   (if (ert-test-result-with-condition-p result)
                                                       (ert-test-result-with-condition-condition result)
                                                     result))))
                                  (observations . ,(femacs-runtime-process--observations)))))
            (when (equal status "passed") (setq exit-code 0))))
         ((equal operation "abi")
          (require 'comp)
          (unless (and (boundp 'comp-abi-hash) (stringp comp-abi-hash))
            (error "Product did not supply an actual native ABI string"))
          (setq reply (append reply `((abi_literal . ,(prin1-to-string comp-abi-hash))
                                     (backend_version . ,(vconcat (comp-libgccjit-version))))))
          (setq exit-code 0))
         ((equal operation "native-unit")
          (let* ((path (alist-get 'path request))
                 (outcome (condition-case err (progn (native-elisp-load path) :loaded)
                            (error err))))
            (setq reply (append reply
                                (if (eq outcome :loaded) '((loaded . t))
                                  `((loaded . :json-false)
                                    (error_symbol . ,(symbol-name (car outcome)))
                                    (diagnostic . ,(femacs-runtime-process--diagnostic outcome))))))
            ;; Rejection is an observation, assessed against the selected
            ;; format-plan row by the external owner; it is not swallowed.
            (setq exit-code 0)))
         ((equal operation "paths-unavailable")
          (let ((rows nil))
            (dolist (path (alist-get 'paths request))
              (let ((row
                     (condition-case denied
                         (progn
                           (with-temp-buffer
                             (insert-file-contents-literally path nil 0 1))
                           `((path . ,path) (readable . t)))
                       (file-error `((path . ,path) (readable . :json-false)
                                     (diagnostic . ,(femacs-runtime-process--diagnostic denied)))))))
                (push row rows)))
            (setq reply (append reply (list (cons 'paths (vconcat (nreverse rows))))))
            (setq exit-code 0)))
         (t (error "Unknown runtime process operation: %S" operation)))
        (setq reply (append reply '((complete . t)))))
    (error
     (setq reply (append reply `((complete . :json-false)
                                (status . "load-or-process-error")
                                (error_symbol . ,(symbol-name (car failure)))
                                (diagnostic . ,(femacs-runtime-process--diagnostic failure)))))))
  (setq reply (append reply `((wall_seconds . ,(- (float-time) started)))))
  (femacs-runtime-process--write result-file reply)
  (kill-emacs exit-code))

;;; runtime-process-runner.el ends here
