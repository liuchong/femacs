;;; android-acceptance.el --- Owned Android editing observations -*- lexical-binding: t; -*-

;; The device harness prepends these two values to its private transferred copy.
;; This file itself contains no device path or fixed run identity.
(defvar femacs-android-report-file nil)
(defvar femacs-android-run-token nil)
(defvar femacs-android-pattern-colors nil)
(defvar femacs-android-observation-timer nil)
(defvar femacs-android-acceptance-buffer nil)
(defvar femacs-android-clipboard-token nil)
(defvar femacs-android-battery-observation nil)

(unless (and (stringp femacs-android-report-file)
             (stringp femacs-android-run-token)
             (not (equal femacs-android-run-token "")))
  (error "Android acceptance requires a private report and run token"))
(unless (eq window-system 'android)
  (error "Android GUI acceptance requires the actual Android window system"))

(require 'json)
(setq femacs-android-clipboard-token
      (concat "Femacs clipboard 中文 " femacs-android-run-token))
;; These are the real native entrypoints declared by the owned Android Lisp
;; backend. The harness invokes this fixture only in its fresh owned AVD.
(android-set-clipboard femacs-android-clipboard-token)
(setq femacs-android-battery-observation (android-query-battery))
(setq femacs-android-acceptance-buffer
      (get-buffer-create "*Femacs Android Acceptance*"))
(switch-to-buffer femacs-android-acceptance-buffer)
(fundamental-mode)
(erase-buffer)
(setq-local truncate-lines nil)
(setq-local mode-line-format nil header-line-format nil tab-line-format nil)
(set-window-margins (selected-window) 0 0)
(set-window-fringes (selected-window) 0 0)
(set-buffer-modified-p nil)

(defun femacs-android-display-pattern ()
  "Observe five current glyph cells with per-run display-only backgrounds."
  (when (equal (buffer-substring-no-properties (point-min) (point-max)) "HELLO")
    (unless (= (length femacs-android-pattern-colors) 5)
      (error "Five private run colors are required"))
    (with-silent-modifications
      (let ((position 1))
        (dolist (color femacs-android-pattern-colors)
          (put-text-property position (1+ position) 'face
                             `(:background ,color :foreground "black"))
          (setq position (1+ position)))))
    (redisplay t)
    (let ((position 1) (cells nil)
          (edges (window-inside-pixel-edges))
          (top (nth 1 (window-pixel-edges))))
      ;; posn X is relative to the text area; Y is relative to the window top.
      ;; This fixture disables margins/fringes/header/tab/mode lines above.
      (dolist (color femacs-android-pattern-colors)
        (let* ((here (posn-x-y (posn-at-point position)))
               (next (posn-x-y (posn-at-point (1+ position)))))
          (unless (and here next (= (cdr here) (cdr next))
                       (< (car here) (car next)))
            (error "Current glyph cells unavailable"))
          (push `((color . ,color)
                  (rect . ,(vector (+ (car edges) (car here))
                                   (+ top (cdr here))
                                   (+ (car edges) (car next))
                                   (+ top (cdr here) (frame-char-height))))) cells))
        (setq position (1+ position)))
      (vconcat (nreverse cells)))))

(defun femacs-android-write-observation ()
  "Record current editor state without moving the editing point or selection."
  (let ((observation nil)
        (target femacs-android-acceptance-buffer))
    (condition-case err
        (progn
          (unless (buffer-live-p target)
            (error "Acceptance editing buffer was killed"))
          (setq observation
                (with-current-buffer target
                  (let ((pattern (femacs-android-display-pattern)))
                  `((schema . "femacs-android-observation-v1")
                    (run_token . ,femacs-android-run-token)
                    (text . ,(buffer-substring-no-properties (point-min) (point-max)))
                    (modified . ,(if (buffer-modified-p) t :json-false))
                    (point . ,(point))
                    (buffer . ,(buffer-name))
                    (selected_buffer . ,(buffer-name (window-buffer (selected-window))))
                    (window_system . ,(symbol-name window-system))
                    (pid . ,(emacs-pid))
                    (frame_width . ,(frame-pixel-width))
                    (frame_height . ,(frame-pixel-height))
                    (char_height . ,(frame-char-height))
                    (window_edges . ,(vconcat (window-pixel-edges)))
                    (display_pattern . ,(or pattern []))
                    (clipboard_expected . ,femacs-android-clipboard-token)
                    (clipboard_actual . ,(or (android-get-clipboard) :json-null))
                    (battery . ,(if femacs-android-battery-observation
                                    (vconcat femacs-android-battery-observation)
                                  :json-null))
                    (default_directory . ,default-directory)
                    (error . :json-null))))))
      (error
       (setq observation
             `((schema . "femacs-android-observation-v1")
               (run_token . ,femacs-android-run-token)
               (pid . ,(emacs-pid))
               (error . ,(error-message-string err))))))
    ;; Rename within the owned app directory, so the reader never sees half JSON.
    (let ((candidate (concat femacs-android-report-file ".next")))
      (with-temp-file candidate
        (let ((coding-system-for-write 'utf-8-unix)
              (json-null :json-null))
          (insert (json-encode observation) "\n")))
      (rename-file candidate femacs-android-report-file t))))

(when (timerp femacs-android-observation-timer)
  (cancel-timer femacs-android-observation-timer))
(setq femacs-android-observation-timer
      (run-at-time 0 0.2 #'femacs-android-write-observation))
(redisplay t)

;;; android-acceptance.el ends here
