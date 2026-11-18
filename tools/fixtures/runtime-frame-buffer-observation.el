;;; runtime-frame-buffer-observation.el --- bounded frame observations -*- lexical-binding: t; -*-

(require 'json)
(defvar expose-hidden-buffer)

(defun runtime-frame-buffer-observation ()
  "Observe original frame/buffer transitions without changing their outcome."
  (let ((origin (selected-frame))
        (started (current-time))
        rows complete failure)
    (condition-case err
        (progn
          (dolist (expose '(nil t))
            (let ((buffer (generate-new-buffer " frame-buffer-observation"))
                  frame before after after-display initial-window-buffer)
              (unwind-protect
                  (with-current-buffer buffer
                    (let ((expose-hidden-buffer expose))
                      (setq before (buffer-name)
                            frame (make-frame '((name . "owned-frame-observation")
                                                (width . 80) (height . 25)))
                            after (buffer-name)
                            initial-window-buffer
                            (buffer-name (window-buffer (frame-selected-window frame))))
                      ;; Match dframe's buffer/display sequence; do not alter
                      ;; any original hook, frame function or error handler.
                      (save-excursion
                        (select-frame frame)
                        (set-window-buffer nil buffer)
                        (set-window-dedicated-p (selected-window) t))
                      (setq after-display (buffer-name))
                      (push (list
                             (cons "expose_hidden_buffer" (if expose t :json-false))
                             (cons "before" before)
                             (cons "after_make_frame" after)
                             (cons "initial_new_window_buffer" initial-window-buffer)
                             (cons "after_display_sequence" after-display)
                             (cons "new_window_buffer"
                                   (buffer-name (window-buffer (frame-selected-window frame))))
                             (cons "new_frame_window_system" (window-system frame))
                             (cons "new_frame_selected" (if (eq frame (selected-frame)) t :json-false)))
                            rows)))
                (when (frame-live-p origin) (select-frame origin))
                (when (and frame (frame-live-p frame)) (delete-frame frame t))
                (when (buffer-live-p buffer) (kill-buffer buffer)))))
          (setq complete t))
      (error (setq failure (error-message-string err))))
    (let ((coding-system-for-write 'utf-8-unix)
          (json-encoding-pretty-print nil))
      (with-temp-buffer
        (insert (json-encode
                 (list (cons "schema" "femacs-runtime-frame-buffer-observation-v1")
                       (cons "collection_complete" (if complete t :json-false))
                       (cons "noninteractive" (if noninteractive t :json-false))
                       (cons "window_system" window-system)
                       (cons "initial_window_system" initial-window-system)
                       (cons "after_make_frame_functions"
                             (let ((print-length 16) (print-level 4))
                               (format "%S" after-make-frame-functions)))
                       (cons "before_make_frame_hook"
                             (let ((print-length 16) (print-level 4))
                               (format "%S" before-make-frame-hook)))
                       (cons "make_frame_origin" (symbol-file 'make-frame 'defun))
                       (cons "rows" (vconcat (nreverse rows)))
                       (cons "error" failure)
                       (cons "seconds" (float-time (time-subtract (current-time) started))))) "\n")
        (write-region (point-min) (point-max)
                      (or (getenv "FE_FRAME_BUFFER_OBSERVATION")
                          (error "owned observation output is required"))
                      nil 'silent nil 'excl)))
    (kill-emacs (if complete 0 2))))

(provide 'runtime-frame-buffer-observation)
