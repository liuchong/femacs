;;; runtime-library-contracts.el --- owned behavior contracts -*- lexical-binding: t; -*-

(require 'json)
(require 'cl-lib)
(require 'dframe)
(defvar fe-library-contract-frame nil)
(defvar fe-library-contract-cache nil)
(defvar fe-library-contract-buffer nil)
(defvar fe-library-contract-create-hook nil)

(defun fe-library-contracts--dframe (hidden)
  "Exercise real frame creation, callback context, and caller continuation."
  (let* ((origin (selected-frame))
         (origin-buffer (current-buffer))
         (buffer (generate-new-buffer (if hidden " *fe-library-contract*"
                                        "*fe-library-contract*")))
         (fe-library-contract-frame nil)
         (fe-library-contract-cache nil)
         (fe-library-contract-buffer buffer)
         (calls 0) callback-buffer callback-frame continuation failure
         (started (current-time))
         (fe-library-contract-create-hook
          (list (lambda ()
                  (setq calls (1+ calls)
                        callback-buffer (eq (current-buffer) buffer)
                        callback-frame (eq (selected-frame) fe-library-contract-frame))
                  (unless (and callback-buffer callback-frame)
                    (error "Creation callback has incorrect application context"))))))
    (unwind-protect
        (condition-case err
            (progn
              ;; Leave the real platform focus hooks and make-frame semantics intact.
              (dframe-frame-mode 1 'fe-library-contract-frame 'fe-library-contract-cache
                                 'fe-library-contract-buffer "Library contract"
                                 #'ignore '((minibuffer . nil))
                                 nil nil 'fe-library-contract-create-hook)
              (setq continuation (and (eq (current-buffer) buffer)
                                      (eq (selected-frame) fe-library-contract-frame)
                                      (eq (window-buffer) buffer))))
          (error (setq failure (let ((print-length 16) (print-level 4))
                                (format "%S" err)))))
      (when (frame-live-p origin) (select-frame origin))
      (when (frame-live-p fe-library-contract-frame)
        (delete-frame fe-library-contract-frame t))
      (when (buffer-live-p buffer) (kill-buffer buffer))
      (when (buffer-live-p origin-buffer) (set-buffer origin-buffer)))
    (list (cons "hidden_buffer" (if hidden t :json-false))
          (cons "callback_calls" calls)
          (cons "callback_buffer_correct" (if callback-buffer t :json-false))
          (cons "callback_frame_correct" (if callback-frame t :json-false))
          (cons "continuation_correct" (if continuation t :json-false))
          (cons "passed" (if (and (= calls 1) callback-buffer callback-frame
                                  continuation (null failure)) t :json-false))
          (cons "error" failure)
          (cons "seconds" (float-time (time-subtract (current-time) started))))))

(defun runtime-library-contracts-run ()
  "Collect actual contract observations; a failed reference stays failed."
  (let* ((rows (list (fe-library-contracts--dframe t)
                     (fe-library-contracts--dframe nil)))
         (correct (cl-every (lambda (row) (eq (cdr (assoc "passed" row)) t)) rows))
         (coding-system-for-write 'utf-8-unix))
    (with-temp-buffer
      (insert (json-encode
               (list (cons "schema" "femacs-runtime-library-contract-v1")
                     (cons "contract" "dframe-create-buffer-v1")
                     (cons "collection_complete" t)
                     (cons "correct_behavior" (if correct t :json-false))
                     (cons "noninteractive" (if noninteractive t :json-false))
                     (cons "window_system" window-system)
                     (cons "library_origin" (symbol-file 'dframe-frame-mode 'defun))
                     (cons "after_make_frame_functions"
                           (let ((print-length 16) (print-level 4))
                             (format "%S" after-make-frame-functions)))
                     (cons "rows" (vconcat rows)))) "\n")
      (write-region (point-min) (point-max)
                    (or (getenv "FE_FRAME_BUFFER_OBSERVATION")
                        (error "Owned contract output is required"))
                    nil 'silent nil 'excl))
    (kill-emacs 0)))

(provide 'runtime-library-contracts)
