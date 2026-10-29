;;; macos-background-gc.el --- Native background/GC regression -*- lexical-binding: t; -*-

;; Exercise the actual native setter with live default faces.  Batch mode does
;; not create the required native view and cannot stand in for this regression.
(require 'json)
(setq inhibit-startup-screen t initial-scratch-message nil)

(defun femacs-background-gc-run ()
  (let ((report (getenv "FEMACS_BACKGROUND_GC_REPORT"))
        (first (selected-frame)) second cases failure)
    (condition-case error-data
        (progn
          (unless (eq window-system 'ns)
            (error "A real Cocoa frame is required"))
          (setq second (make-frame '((name . "Fe background GC B")
                                    (width . 60) (height . 25))))
          (dolist (entry `(("A" . ,first) ("B" . ,second)))
            (let ((frame (cdr entry)))
              (select-frame frame)
              (dolist (color '("#feffff" "white" "#123456" "black"))
                (let ((started (float-time)))
                  (set-frame-parameter frame 'background-color color)
                  ;; Collect before redisplay can replace the corrupted slot.
                  (garbage-collect)
                  (unless (equal (color-values color frame)
                                 (color-values (frame-parameter frame 'background-color) frame))
                    (error "Background color did not survive collection"))
                  (with-current-buffer (get-buffer-create
                                        (format " *background-gc-%s*" (car entry)))
                    (erase-buffer)
                    (insert "Native background, Unicode 编辑, and collection.")
                    (set-window-buffer (frame-selected-window frame) (current-buffer)))
                  (redisplay t)
                  (garbage-collect)
                  (push `((id . ,(concat (car entry) "/" color))
                          (passed . t) (seconds . ,(- (float-time) started))) cases))))))
      (error (setq failure (format "%S" error-data))))
    (when (and second (frame-live-p second)) (delete-frame second t))
    (with-temp-file report
      (insert (json-encode
               `((passed . ,(if (and (not failure) (= (length cases) 8)) t :json-false))
                 (cases . ,(vconcat (nreverse cases)))
                 (error . ,failure) (gc_count . ,gcs-done)))))
    (kill-emacs (if failure 1 0))))

(run-at-time 0.1 nil #'femacs-background-gc-run)
