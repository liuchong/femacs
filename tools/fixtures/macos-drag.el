;;; macos-drag.el --- Configurable native drag acceptance -*- lexical-binding: t; -*-
(require 'json)
(setq inhibit-startup-screen t initial-scratch-message "Drag the owned acceptance text file into this window.\n")
(defvar femacs-drag-report (getenv "FEMACS_DRAG_REPORT"))
(defvar femacs-drag-events nil)
(defvar femacs-drag-ready (getenv "FEMACS_DRAG_READY"))
(defvar femacs-drag-file (getenv "FEMACS_DRAG_FILE"))
(modify-frame-parameters nil '((name . "Femacs Drag Target") (left . 700) (top . 80)
                               (width . 65) (height . 28)))
(setq ns-drag-motion-function
      (lambda (frame x y)
        (push `((frame-live . ,(if (frame-live-p frame) t :json-false))
                (name . ,(frame-parameter frame 'name)) (x . ,x) (y . ,y)) femacs-drag-events)
        (with-temp-file femacs-drag-report (insert (json-encode (reverse femacs-drag-events))))))
(defun femacs-drag-snapshot ()
  (when femacs-drag-ready
    (with-temp-file femacs-drag-ready
      (insert
       (json-encode
        `((pid . ,(emacs-pid)) (native . ,(symbol-name window-system))
          (frame . ,(frame-parameter nil 'name))
          (load_path . ,(vconcat load-path))
          (data_directory . ,data-directory) (exec_directory . ,exec-directory)
          (doc_directory . ,doc-directory)
          (opened . ,(if (and femacs-drag-file (find-buffer-visiting femacs-drag-file))
                         (with-current-buffer (find-buffer-visiting femacs-drag-file)
                           (buffer-substring-no-properties (point-min) (point-max)))
                       :json-null))))))))
(run-with-timer 0.1 0.1 #'femacs-drag-snapshot)
