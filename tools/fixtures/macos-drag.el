;;; macos-drag.el --- Configurable native drag acceptance -*- lexical-binding: t; -*-
(require 'json)
(setq inhibit-startup-screen t initial-scratch-message "Drag the owned acceptance text file into this window.\n")
(defvar femacs-drag-report (getenv "FEMACS_DRAG_REPORT"))
(defvar femacs-drag-events nil)
(setq ns-drag-motion-function
      (lambda (frame x y)
        (push `((frame-live . ,(if (frame-live-p frame) t :json-false)) (x . ,x) (y . ,y)) femacs-drag-events)
        (with-temp-file femacs-drag-report (insert (json-encode (reverse femacs-drag-events))))))
