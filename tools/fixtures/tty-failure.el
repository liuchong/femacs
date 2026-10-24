;;; tty-failure.el --- Recoverable make-terminal-frame failure fixture -*- lexical-binding: t; -*-

;;; Commentary:
;; Exercises SD-A2 recoverable `make-terminal-frame` failure paths in a live
;; editor process without adding any test-only public Lisp primitive:
;; 1. Repeated `make-terminal-frame` attempts on owned nonexistent, regular-file,
;;    and directory paths fail with the exact expected `init_tty` diagnostics
;;    ("Could not open file: ..." / "Not a tty device: ...").
;; 2. Optional `FEMACS_TTY_FAIL_PTY` exercises the post-open named-terminal
;;    `delete_tty` failure path with an undefined `tty-type`.
;; 3. Verifies `terminal-list`, `terminal-name` values, and `frame-list` remain
;;    unchanged and live across all failures, forces garbage collection, creates
;;    and saves a UTF-8 buffer to disk, and writes a JSON receipt.
;; Note: process-level `terminal-list` invariance does not by itself measure C
;; heap deallocation (`xfree` of `tty_display_info`/`Wcm`), which is verified
;; separately by source-level ownership review of `src/term.rs`.

;;; Code:

(require 'json)

(defvar femacs-tty-failure-schema "femacs-tty-failure-v1")

(defun femacs-tty-failure--terminal-snapshot ()
  "Return a deterministic snapshot of live terminals and frames."
  (let ((terms (terminal-list)))
    (list :terminal_count (length terms)
          :terminal_names (mapcar #'terminal-name terms)
          :all_live (null (memq nil (mapcar #'terminal-live-p terms)))
          :frame_count (length (frame-list)))))

(defun femacs-tty-failure--expect-error (tty-path tty-type expected-substring)
  "Call `make-terminal-frame' on TTY-PATH and TTY-TYPE and require EXPECTED-SUBSTRING."
  (let (( outcome nil))
    (condition-case err
        (let ((frame (make-terminal-frame
                      (list (cons 'tty tty-path)
                            (cons 'tty-type tty-type)))))
          (when (frame-live-p frame)
            (delete-frame frame t))
          (error "make-terminal-frame unexpectedly succeeded for %s" tty-path))
      (error
       (let ((msg (error-message-string err)))
         (unless (and (eq (car err) 'error)
                      (stringp msg)
                      (string-match-p (regexp-quote expected-substring) msg)
                      (string-match-p (regexp-quote tty-path) msg))
           (error "Unexpected make-terminal-frame diagnostic for %s: %S (expected %S)"
                  tty-path msg expected-substring))
         (setq outcome (list (cons 'tty tty-path)
                             (cons 'tty_type tty-type)
                             (cons 'error_symbol (symbol-name (car err)))
                             (cons 'message msg))))))
    outcome))

(defun femacs-tty-failure--expect-termtype-error (tty-path tty-type)
  "Call `make-terminal-frame' on valid TTY-PATH with undefined TTY-TYPE."
  (let ((outcome nil)
        (expected (format "Terminal type %s is not defined" tty-type)))
    (condition-case err
        (let ((frame (make-terminal-frame
                      (list (cons 'tty tty-path)
                            (cons 'tty-type tty-type)))))
          (when (frame-live-p frame)
            (delete-frame frame t))
          (error "make-terminal-frame unexpectedly succeeded for tty-type %s" tty-type))
      (error
       (let ((msg (error-message-string err)))
         (unless (and (eq (car err) 'error)
                      (stringp msg)
                      (string-match-p (regexp-quote expected) msg))
           (error "Unexpected tty-type diagnostic: %S (expected %S)" msg expected))
         (setq outcome (list (cons 'tty tty-path)
                             (cons 'tty_type tty-type)
                             (cons 'error_symbol (symbol-name (car err)))
                             (cons 'message msg))))))
    outcome))

(defun femacs-tty-failure-run ()
  "Execute repeated recoverable TTY frame creation failures and write report."
  (let* ((work-dir (or (getenv "FEMACS_TTY_FAIL_DIR")
                       (error "FEMACS_TTY_FAIL_DIR is required")))
         (report-file (or (getenv "FEMACS_TTY_FAIL_REPORT")
                          (expand-file-name "tty-failure-report.json" work-dir)))
         (pty-device (getenv "FEMACS_TTY_FAIL_PTY"))
         (missing-path (expand-file-name "owned-missing-tty-device" work-dir))
         (regular-path (expand-file-name "owned-regular-not-a-tty.txt" work-dir))
         (directory-path (expand-file-name "owned-directory-not-a-tty" work-dir))
         (saved-path (expand-file-name "owned-survivor-buffer.txt" work-dir))
         (expected-text "femacs-tty-failure-survivor-你好\nsecond-line-ok\n")
         (before (femacs-tty-failure--terminal-snapshot))
         (records nil))
    (make-directory work-dir t)
    (when (file-exists-p missing-path)
      (delete-file missing-path))
    (with-temp-file regular-path
      (insert "not a tty device\n"))
    (make-directory directory-path t)

    ;; Repeat early open/isatty failure branches across 3 rounds (9 calls).
    (dotimes (round 3)
      (push (append (list (cons 'round (1+ round))
                          (cons 'kind "missing"))
                    (femacs-tty-failure--expect-error
                     missing-path "vt100" "Could not open file: "))
            records)
      (push (append (list (cons 'round (1+ round))
                          (cons 'kind "regular"))
                    (femacs-tty-failure--expect-error
                     regular-path "vt100" "Not a tty device: "))
            records)
      (push (append (list (cons 'round (1+ round))
                          (cons 'kind "directory"))
                    (femacs-tty-failure--expect-error
                     directory-path "vt100" "Could not open file: "))
            records))

    ;; If the harness supplies an owned PTY device path, also exercise the
    ;; post-open named-terminal failure branch (`delete_tty` on undefined TERM).
    (when (and pty-device (> (length pty-device) 0))
      (push (append (list (cons 'round 1)
                          (cons 'kind "undefined-term-type"))
                    (femacs-tty-failure--expect-termtype-error
                     pty-device "femacs-undefined-term-sd-a2"))
            records))

    ;; Verify `get-device-terminal` on an unopened device signals without crash.
    (let ((lookup-err nil))
      (condition-case err
          (get-device-terminal missing-path)
        (error
         (setq lookup-err (error-message-string err))))
      (unless (and (stringp lookup-err)
                   (string-match-p (regexp-quote missing-path) lookup-err))
        (error "Expected get-device-terminal error for %s, got %S"
               missing-path lookup-err)))

    (garbage-collect)

    (let ((after (femacs-tty-failure--terminal-snapshot)))
      (unless (and (equal (plist-get before :terminal_count)
                          (plist-get after :terminal_count))
                   (equal (plist-get before :terminal_names)
                          (plist-get after :terminal_names))
                   (eq (plist-get after :all_live) t)
                   (equal (plist-get before :frame_count)
                          (plist-get after :frame_count)))
        (error "Terminal/frame state changed across failed make-terminal-frame calls: before=%S after=%S"
               before after))

      ;; Create, edit, and save a UTF-8 buffer after the failed TTY attempts and GC.
      (let ((buf (generate-new-buffer " *femacs-tty-failure-edit*")))
        (unwind-protect
            (with-current-buffer buf
              (set-buffer-file-coding-system 'utf-8-unix)
              (insert expected-text)
              (goto-char (point-min))
              (end-of-line)
              (let ((write-region-inhibit-fsync t))
                (write-region (point-min) (point-max) saved-path nil 'silent)))
          (when (buffer-live-p buf)
            (kill-buffer buf))))

      (let ((disk-text (with-temp-buffer
                         (let ((coding-system-for-read 'utf-8-unix))
                           (insert-file-contents saved-path)
                           (buffer-string)))))
        (unless (equal disk-text expected-text)
          (error "Saved buffer mismatch after TTY failures: %S != %S"
                 disk-text expected-text)))

      (let ((payload
             (list (cons 'schema femacs-tty-failure-schema)
                   (cons 'passed t)
                   (cons 'failure_count (length records))
                   (cons 'terminal_count_before (plist-get before :terminal_count))
                   (cons 'terminal_count_after (plist-get after :terminal_count))
                   (cons 'terminal_names_before (vconcat (plist-get before :terminal_names)))
                   (cons 'terminal_names_after (vconcat (plist-get after :terminal_names)))
                   (cons 'frame_count_before (plist-get before :frame_count))
                   (cons 'frame_count_after (plist-get after :frame_count))
                   (cons 'all_terminals_live t)
                   (cons 'gc_completed t)
                   (cons 'saved_file saved-path)
                   (cons 'saved_bytes (file-attribute-size (file-attributes saved-path)))
                   (cons 'records (vconcat (nreverse records)))
                   (cons 'ownership_boundary
                         "Process-level terminal-list invariance and post-GC survival do not directly measure C heap xfree of tty_display_info/Wcm; exact deallocation is verified by source review of src/term.rs."))))
        (with-temp-file report-file
          (set-buffer-file-coding-system 'utf-8-unix)
          (insert (json-serialize payload
                                  :false-object :json-false
                                  :null-object nil)
                  "\n"))))))

(when noninteractive
  (femacs-tty-failure-run))

(provide 'tty-failure)
;;; tty-failure.el ends here
