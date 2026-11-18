;;; runtime-remote-watch-contract.el --- Owned remote-watch lifecycle -*- lexical-binding: t; -*-

;; Loaded only by the maintained remote-process fixture. Original test skips
;; remain skips. The authored controls never ask the test's monitor identity.
(require 'filenotify)
(require 'backtrace)

(defvar runtime-remote-watch--trace nil)

(defun runtime-remote-watch--monitor (original &rest arguments)
  (condition-case failure
      (apply original arguments)
    (ert-test-skipped
     (condition-case observation-error
       (when (< (length runtime-remote-watch--trace) 8)
       (let* ((descriptor file-notify--test-desc)
              (frames nil) (index 0))
         (while (< index 24)
           (let* ((frame (backtrace-frame index)) (function (nth 1 frame)))
             (when (symbolp function) (push (symbol-name function) frames)))
           (setq index (1+ index)))
         (push `((condition . ,(runtime-tramp--text failure))
                 (descriptor_is_process . ,(if (processp descriptor) t :json-false))
                 (descriptor_live . ,(if (and (processp descriptor) (process-live-p descriptor)) t :json-false))
                 (descriptor_status . ,(if (processp descriptor) (symbol-name (process-status descriptor)) "not-process"))
                 (cached_monitor . ,(runtime-tramp--text (alist-get descriptor file-notify--test-monitors) 256))
                 (stack_functions . ,(vconcat (nreverse frames)))) runtime-remote-watch--trace)))
       (error (push `((observation_error . ,(runtime-tramp--text observation-error)))
                    runtime-remote-watch--trace)))
     (signal (car failure) (cdr failure)))))

(defun runtime-remote-watch--original (id)
  (let ((runtime-remote-watch--trace nil) result)
    (unwind-protect
        (progn
          (advice-add 'file-notify--test-monitor :around #'runtime-remote-watch--monitor)
          (setq result (runtime-tramp--original id)))
      (advice-remove 'file-notify--test-monitor #'runtime-remote-watch--monitor))
    (append result `((monitor_trace . ,(vconcat (nreverse runtime-remote-watch--trace)))))))

(defun runtime-remote-watch--one (operation)
  (let* ((started (float-time))
         (local (make-temp-file "remote-watch-" t))
         (remote (concat (file-remote-p ert-remote-temporary-file-directory) local))
         (target (if (eq operation 'emulated-unmount) (concat remote "/watched-file") remote))
         (events nil) descriptor stopped-file
         (row `((id . ,(symbol-name operation)))))
    (unwind-protect
        (condition-case failure
            (progn
              (when (eq operation 'emulated-unmount)
                (with-temp-file (expand-file-name "watched-file" local)))
              (setq descriptor
                    (file-notify-add-watch target '(change)
                     (lambda (event)
                       (when (< (length events) 32)
                         (push (symbol-name (nth 1 event)) events)
                         (when (eq (nth 1 event) 'stopped) (setq stopped-file (nth 2 event)))))))
              (unless (file-notify-valid-p descriptor) (error "new remote watch is not valid"))
              (setq row (append row '((valid_before . t))))
              (pcase operation
                ('remove-watch (file-notify-rm-watch descriptor))
                ('delete-watched-directory (delete-directory remote))
                ('emulated-unmount
                 ;; This is the original test's emulated backend stimulus,
                 ;; not a real host filesystem unmount.
                 (file-notify-callback (list descriptor '(unmounted) (file-local-name remote)))))
              (let ((deadline (+ (float-time) 20)))
                (while (and (< (float-time) deadline)
                            (or (file-notify-valid-p descriptor)
                                (and (eq operation 'emulated-unmount) (not (member "stopped" events)))))
                  ;; Special file-notify events need actual event dispatch.
                  ;; No private monitor query runs before delivering them.
                  (read-event nil nil 0.02)))
              (when (file-notify-valid-p descriptor) (error "remote watch stayed valid"))
              (when (and (eq operation 'emulated-unmount) (not (member "stopped" events)))
                (error "emulated unmount did not deliver stopped"))
              (setq row (append row '((passed . t) (valid_after . :json-false)))))
          (error (setq row (append row `((passed . :json-false)
                                         (condition . ,(runtime-tramp--text failure)))))))
      (when descriptor (ignore-errors (file-notify-rm-watch descriptor)))
      (when (file-directory-p local) (delete-directory local t)))
    (append row `((events . ,(vconcat (nreverse events)))
                  (watched_local_file . ,(file-local-name target))
                  (stopped_local_file . ,(when stopped-file (file-local-name stopped-file)))
                  (stopped_file_matches_watched . ,(if (and stopped-file
                                                          (equal (file-local-name target) (file-local-name stopped-file)))
                                                     t :json-false))
                  (seconds . ,(- (float-time) started))))))

(defun runtime-remote-watch-contract-run ()
  (load (getenv "FEMACS_REMOTE_WATCH_ORIGINAL_TEST") nil t t)
  (let* ((original (mapcar #'runtime-remote-watch--original
                          '(file-notify-test06-dir-validity-remote file-notify-test12-unmount-remote)))
         (rows (mapcar #'runtime-remote-watch--one
                       '(remove-watch delete-watched-directory emulated-unmount))))
    `((schema . "femacs-remote-watch-contract-v1")
      (contract . "remote-watch-lifecycle-v1")
      (collection_complete . t)
      (lifecycle_complete . ,(if (cl-every (lambda (row) (eq (alist-get 'passed row) t)) rows) t :json-false))
      (correct_behavior . ,(if (and (cl-every (lambda (row) (eq (alist-get 'passed row) t)) rows)
                                   (eq (alist-get 'stopped_file_matches_watched (car (last rows))) t)) t :json-false))
      (library_origin . ,(symbol-file 'file-notify-callback 'defun))
      (original_cases . ,(vconcat original)) (rows . ,(vconcat rows)))))

;;; runtime-remote-watch-contract.el ends here
