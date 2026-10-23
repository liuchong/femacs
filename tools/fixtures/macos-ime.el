;;; macos-ime.el --- Real system IME acceptance -*- lexical-binding: t; -*-
(setq inhibit-startup-screen t initial-scratch-message nil)
(defvar femacs-ime-buffer (get-buffer-create "*femacs-ime-acceptance*"))
(defvar femacs-ime-report (getenv "FEMACS_IME_REPORT"))
(switch-to-buffer femacs-ime-buffer)
(erase-buffer)
(set-buffer-modified-p nil)
(defun femacs-ime-snapshot ()
  (let ((record
         (with-current-buffer femacs-ime-buffer
           `((text . ,(buffer-substring-no-properties (point-min) (point-max)))
             (working . ,(if (boundp 'ns-working-text) ns-working-text ""))
             (marked . ,(if (and (boundp 'ns-working-overlay) ns-working-overlay) t :json-false))
             (minibuffer . ,(if (active-minibuffer-window) t :json-false))
             (native . ,(symbol-name window-system))))))
    (with-temp-file femacs-ime-report (insert (json-encode record)))))
(require 'json)
(run-with-timer 0.1 0.1 #'femacs-ime-snapshot)
