;;; file-notify-remote-diagnostic.el --- Error-only Base64 and Tramp state capture for remote09 -*- lexical-binding: t; -*-

;; Loaded by tools/remote-notification-diagnostic.py (or in place of
;; tools/fixtures/stable-suite-runner.el for remote09 diagnostic runs).
;;
;; Design constraints:
;; - Successful calls preserve decoder results and do not alter
;;   `tramp-verbose', filters, timers, or region contents. Wrapping still
;;   changes stack/timing; a passing diagnostic is not a race-freedom proof.
;; - On `(error "Invalid base64 data")' (or an error/quit signal from
;;   `base64-decode-region' / `base64-decode-string'), captures the exact
;;   failing input bytes (up to 65,536 bytes), the first non-Base64 byte
;;   offset and its surrounding byte window, the active Tramp connection and
;;   monitor process states/buffers, and a bounded backtrace before
;;   re-signaling the original condition unchanged.
;; Retention limits are not a constant-memory guarantee: hashing and diagnostic
;; construction can copy full input strings. The isolated runner has bounded
;; inputs and must remain under the external process-tree resource controller.

(unless (featurep 'stable-suite-runner)
  (let* ((this-file (or load-file-name buffer-file-name))
         (fixture-dir (and this-file (file-name-directory this-file)))
         (runner (and fixture-dir
                      (expand-file-name "stable-suite-runner.el" fixture-dir))))
    (unless (and runner (file-regular-p runner))
      (error "missing sibling stable-suite-runner.el"))
    (load runner nil t t)))

(require 'backtrace)
(require 'cl-lib)
(require 'ert)
(require 'json)

(defconst femacs-remote09-diag--max-payload-bytes 65536
  "Maximum raw byte count retained for a failing Base64 payload.")

(defconst femacs-remote09-diag--max-buffer-bytes 16384
  "Maximum raw byte count retained per live process buffer on failure.")

(defconst femacs-remote09-diag--max-frames 48
  "Maximum backtrace frame count retained on failure.")

(defvar femacs-remote09-diag--captures nil
  "List of error-only Base64 failure records captured in this process.")

(defvar femacs-remote09-diag--installed nil
  "Non-nil once diagnostic wrappers are installed in this process.")

(defvar femacs-remote09-diag--orig-decode-region
  (symbol-function 'base64-decode-region)
  "Original `base64-decode-region' function before diagnostic wrapping.")

(defvar femacs-remote09-diag--orig-decode-string
  (symbol-function 'base64-decode-string)
  "Original `base64-decode-string' function before diagnostic wrapping.")

(defvar femacs-remote09-diag--orig-result-record
  (symbol-function 'stable-suite--result-record)
  "Original `stable-suite--result-record' function before diagnostic wrapping.")

(defun femacs-remote09-diag--base64-byte-p (byte &optional base64url)
  "Return non-nil if BYTE is valid Base64 alphabet, padding, or whitespace."
  (or (and (>= byte ?A) (<= byte ?Z))
      (and (>= byte ?a) (<= byte ?z))
      (and (>= byte ?0) (<= byte ?9))
      (if base64url
          (memq byte '(?- ?_ ?= ?\s ?\t ?\n ?\r ?\f ?\v))
        (memq byte '(?+ ?/ ?= ?\s ?\t ?\n ?\r ?\f ?\v)))))

(defun femacs-remote09-diag--hex-encode (unibyte-str)
  "Encode UNIBYTE-STR as lowercase hexadecimal."
  (mapconcat (lambda (b) (format "%02x" b)) unibyte-str ""))

(defun femacs-remote09-diag--first-invalid-byte (unibyte-str &optional base64url)
  "Find the first byte in UNIBYTE-STR that is not valid Base64 or whitespace."
  (let ((len (length unibyte-str))
        (idx 0)
        (found nil))
    (while (and (< idx len) (not found))
      (let ((b (aref unibyte-str idx)))
        (unless (femacs-remote09-diag--base64-byte-p b base64url)
          (let* ((win-start (max 0 (- idx 32)))
                 (win-end (min len (+ idx 64)))
                 (window (substring unibyte-str win-start win-end)))
            (setq found
                  (list (cons "offset" idx)
                        (cons "byte" b)
                        (cons "window_start" win-start)
                        (cons "window_end" win-end)
                        (cons "window_hex"
                              (femacs-remote09-diag--hex-encode window))
                        (cons "window_latin1"
                              (decode-coding-string window 'iso-latin-1))))))
        (setq idx (1+ idx))))
    found))

(defun femacs-remote09-diag--bytes-summary (raw limit &optional base64url)
  "Build a bounded deterministic byte summary for string RAW up to LIMIT."
  (let* ((raw (or raw ""))
         (exact (not (multibyte-string-p raw)))
         ;; The Tramp decoder's temporary input buffer is unibyte. Never
         ;; transcode its failing bytes before recording the evidence.
         (unibyte (if exact raw (encode-coding-string raw 'utf-8-emacs t)))
         (total (length unibyte))
         (clipped (substring unibyte 0 (min total limit))))
    (list (cons "exact_input_bytes" (if exact t :json-false))
          (cons "representation" (if exact "unibyte-input" "utf8-emacs-text"))
          (cons "alphabet_checked"
                (if base64url "base64url" "standard-base64"))
          (cons "byte_length" total)
          (cons "retained_bytes" (length clipped))
          (cons "truncated" (if (> total limit) t :json-false))
          (cons "sha256" (secure-hash 'sha256 unibyte))
          (cons "raw_hex" (femacs-remote09-diag--hex-encode clipped))
          (cons "raw_latin1" (decode-coding-string clipped 'iso-latin-1))
          (cons "first_invalid_byte"
                (femacs-remote09-diag--first-invalid-byte
                 unibyte base64url)))))

(defun femacs-remote09-diag--safe-prin1 (value)
  "Format VALUE with bounded depth and length."
  (let ((print-length 24)
        (print-level 6)
        (print-circle t))
    (let ((printed (prin1-to-string value)))
      (if (> (length printed) 4096)
          (concat (substring printed 0 4096) " [truncated]")
        printed))))

(defun femacs-remote09-diag--backtrace ()
  "Capture a bounded list of backtrace frame strings."
  (let ((frames nil)
        (count 0))
    (ignore-errors
      (dolist (frame (backtrace-get-frames))
        (when (< count femacs-remote09-diag--max-frames)
          (push (femacs-remote09-diag--safe-prin1 frame) frames)
          (setq count (1+ count)))))
    (vconcat (nreverse frames))))

(defun femacs-remote09-diag--process-snapshot ()
  "Capture bounded state of all live and exited processes and their buffers."
  (let ((rows nil))
    (ignore-errors
      (dolist (proc (process-list))
        (let* ((buf (process-buffer proc))
               (buf-live (and buf (buffer-live-p buf)))
               (buf-raw (and buf-live
                             (with-current-buffer buf
                               (buffer-substring-no-properties
                                (point-min) (point-max)))))
               (plist (process-plist proc))
               (tramp-vec (plist-get plist 'tramp-vector))
               (rest-str (plist-get plist 'tramp-rest-string))
               (events (plist-get plist 'tramp-events))
               (locked (plist-get plist 'locked))
               (remote-echo (plist-get plist 'remote-echo))
               (check-echo (plist-get plist 'check-remote-echo))
               (file-monitor
                (and (fboundp 'tramp-get-connection-property)
                     (ignore-errors
                       (tramp-get-connection-property proc "file-monitor")))))
          (push
           (list
            (cons "name" (process-name proc))
            (cons "status" (symbol-name (process-status proc)))
            (cons "exit_status" (process-exit-status proc))
            (cons "command" (vconcat (mapcar (lambda (argument) (format "%s" argument))
                                             (or (process-command proc) nil))))
            (cons "buffer_name" (and buf (buffer-name buf)))
            (cons "buffer_live" (if buf-live t :json-false))
            (cons "tramp_vector"
                  (and tramp-vec (femacs-remote09-diag--safe-prin1 tramp-vec)))
            (cons "file_monitor"
                  (and file-monitor (femacs-remote09-diag--safe-prin1 file-monitor)))
            (cons "locked" (if locked t :json-false))
            (cons "remote_echo" (if remote-echo t :json-false))
            (cons "check_remote_echo" (if check-echo t :json-false))
            (cons "tramp_events"
                  (and events (femacs-remote09-diag--safe-prin1 events)))
            (cons "tramp_rest_string"
                  (and (stringp rest-str)
                       (femacs-remote09-diag--bytes-summary
                        rest-str femacs-remote09-diag--max-buffer-bytes)))
            (cons "buffer_summary"
                  (and buf-live
                       (femacs-remote09-diag--bytes-summary
                        buf-raw femacs-remote09-diag--max-buffer-bytes))))
           rows))))
    (vconcat (nreverse rows))))

(defun femacs-remote09-diag--write-optional-output (record)
  "Write RECORD to `FEMACS_REMOTE09_DIAG_OUT' when configured."
  (let ((out (or (getenv "FEMACS_REMOTE09_DIAG_OUT")
                 (and (getenv "FEMACS_RUNTIME_OUT")
                      (expand-file-name "remote09-diagnostic.json"
                                        (getenv "FEMACS_RUNTIME_OUT"))))))
    (when (and (stringp out) (> (length out) 0))
      (ignore-errors
        (let ((coding-system-for-write 'utf-8-unix)
              (json-encoding-pretty-print t))
          (with-temp-file out
            (insert
             (json-encode
              (list (cons "schema" "femacs-remote09-diagnostic-v1")
                    (cons "capture_count"
                          (length femacs-remote09-diag--captures))
                    (cons "latest" record)
                    (cons "captures"
                          (vconcat (reverse femacs-remote09-diag--captures)))))
             "\n")))))))

(defun femacs-remote09-diag--record-region-error
    (beg end base64url ignore-invalid err)
  "Record a failing `base64-decode-region' call without altering state or signal."
  (ignore-errors
    (let ((caller-buf (current-buffer))
          (caller-point (point))
          (caller-pmin (point-min))
          (caller-pmax (point-max))
          (caller-modified (buffer-modified-p))
          (caller-multibyte enable-multibyte-characters)
          (caller-size (buffer-size)))
      (save-current-buffer
        (save-excursion
          (save-restriction
            (save-match-data
              (let* ((pos-beg (cond ((markerp beg) (marker-position beg))
                                    ((integerp beg) beg)
                                    (t nil)))
                     (pos-end (cond ((markerp end) (marker-position end))
                                    ((integerp end) end)
                                    (t nil)))
                     (start (if (integerp pos-beg)
                                (max caller-pmin (min pos-beg caller-pmax))
                              caller-pmin))
                     (stop (if (integerp pos-end)
                               (max caller-pmin (min pos-end caller-pmax))
                             caller-pmax))
                     (lo (min start stop))
                     (hi (max start stop))
                     (raw (buffer-substring-no-properties lo hi))
                     (record
                      (list
                       (cons "operation" "base64-decode-region")
                       (cons "error_symbol" (symbol-name (car-safe err)))
                       (cons "error_data"
                             (femacs-remote09-diag--safe-prin1 (cdr-safe err)))
                       (cons "error_message" (error-message-string err))
                       (cons "base64url" (if base64url t :json-false))
                       (cons "ignore_invalid" (if ignore-invalid t :json-false))
                       (cons "buffer_name" (buffer-name caller-buf))
                       (cons "multibyte" (if caller-multibyte t :json-false))
                       (cons "buffer_modified"
                             (if caller-modified t :json-false))
                       (cons "buffer_size" caller-size)
                       (cons "point" caller-point)
                       (cons "point_min" caller-pmin)
                       (cons "point_max" caller-pmax)
                       (cons "region_start" lo)
                       (cons "region_end" hi)
                       (cons "char_length" (length raw))
                       (cons "payload"
                             (femacs-remote09-diag--bytes-summary
                              raw femacs-remote09-diag--max-payload-bytes
                              base64url))
                       (cons "processes" (femacs-remote09-diag--process-snapshot))
                       (cons "backtrace" (femacs-remote09-diag--backtrace)))))
                (push record femacs-remote09-diag--captures)
                (femacs-remote09-diag--write-optional-output record)))))))))

(defun femacs-remote09-diag--record-string-error (string base64url noerror err)
  "Record a failing `base64-decode-string' call without altering state or signal."
  (ignore-errors
    (save-current-buffer
      (save-excursion
        (save-restriction
          (save-match-data
            (let* ((raw (if (stringp string) string ""))
                   (record
                    (list
                     (cons "operation" "base64-decode-string")
                     (cons "error_symbol" (symbol-name (car-safe err)))
                     (cons "error_data"
                           (femacs-remote09-diag--safe-prin1 (cdr-safe err)))
                     (cons "error_message" (error-message-string err))
                     (cons "base64url" (if base64url t :json-false))
                     (cons "noerror" (if noerror t :json-false))
                     (cons "multibyte"
                           (if (and (stringp string) (multibyte-string-p string))
                               t :json-false))
                     (cons "char_length" (length raw))
                     (cons "payload"
                           (femacs-remote09-diag--bytes-summary
                            raw femacs-remote09-diag--max-payload-bytes
                            base64url))
                     (cons "processes" (femacs-remote09-diag--process-snapshot))
                     (cons "backtrace" (femacs-remote09-diag--backtrace)))))
              (push record femacs-remote09-diag--captures)
              (femacs-remote09-diag--write-optional-output record))))))))

(unless femacs-remote09-diag--installed
  (setq femacs-remote09-diag--installed t)
  (let ((orig-region femacs-remote09-diag--orig-decode-region)
        (orig-string femacs-remote09-diag--orig-decode-string)
        (orig-record femacs-remote09-diag--orig-result-record))
    (fset 'base64-decode-region
          (lambda (beg end &optional base64url ignore-invalid)
            (interactive "r")
            (condition-case err
                (funcall orig-region beg end base64url ignore-invalid)
              ((error quit)
               (femacs-remote09-diag--record-region-error
                beg end base64url ignore-invalid err)
               (signal (car err) (cdr err))))))
    (fset 'base64-decode-string
          (lambda (string &optional base64url noerror)
            (condition-case err
                (funcall orig-string string base64url noerror)
              ((error quit)
               (femacs-remote09-diag--record-string-error
                string base64url noerror err)
               (signal (car err) (cdr err))))))
    (fset 'stable-suite--result-record
          (lambda (id test result)
            (append
             (funcall orig-record id test result)
             (list
              (cons "base64_error_capture_count"
                    (length femacs-remote09-diag--captures))
              (cons "base64_error_captures"
                    (vconcat (reverse femacs-remote09-diag--captures)))))))))

(when (and (equal (getenv "FEMACS_REMOTE09_DIAG_SELF_TEST") "1")
           (not (ert-test-boundp
                 'femacs-remote09-diag-self-test-region-byte255-failure)))
  (ert-deftest femacs-remote09-diag-self-test-region-byte255-failure ()
    "Verify failing unibyte region with byte 255 preserves error, point, and buffer."
    (setq femacs-remote09-diag--captures nil)
    (let* ((dummy-buf (get-buffer-create " *femacs-remote09-diag-dummy-buf*"))
           (dummy-proc nil)
           (test-buf (generate-new-buffer " *tramp/mock-diag-region-test*"))
           (prefix (unibyte-string ?H ?E ?A ?D ?E ?R ?:))
           (payload (unibyte-string ?T ?W ?F ?u 255 ?Y ?W ?J ?j))
           (suffix (unibyte-string ?: ?F ?O ?O ?T ?E ?R))
           (full-bytes (concat prefix payload suffix)))
      (unwind-protect
          (progn
            (with-current-buffer dummy-buf
              (set-buffer-multibyte nil)
              (erase-buffer)
              (insert (unibyte-string ?O ?K ?\n 255)))
            (setq dummy-proc
                  (make-process
                   :name "femacs-remote09-diag-dummy"
                   :buffer dummy-buf
                   :command '("sh" "-c" "cat # 100%")
                   :connection-type 'pipe
                   :noquery t))
            (process-put dummy-proc 'tramp-vector
                         '["mock" nil "localhost" "owned-diag"])
            (process-put dummy-proc 'tramp-rest-string
                         (unibyte-string ?R ?E ?S ?T 255))
            (process-put dummy-proc 'locked t)
            (process-put dummy-proc 'remote-echo t)
            (with-current-buffer test-buf
              (set-buffer-multibyte nil)
              (insert full-bytes)
              (narrow-to-region 8 17)
              (goto-char 11)
              (set-buffer-modified-p nil)
              (string-match "HEADER" "HEADER")
              (let ((saved-match (match-data))
                    (caught nil))
                (condition-case err
                    (base64-decode-region (point-min) (point-max))
                  (error (setq caught err)))
                (unless (equal caught '(error "Invalid base64 data"))
                  (error "unexpected condition: %S" caught))
                (unless (eq (current-buffer) test-buf)
                  (error "current-buffer changed"))
                (unless (= (point) 11)
                  (error "point changed: %S" (point)))
                (unless (and (= (point-min) 8) (= (point-max) 17))
                  (error "narrowing changed: %S..%S" (point-min) (point-max)))
                (unless (not enable-multibyte-characters)
                  (error "multibyte mode changed"))
                (unless (not (buffer-modified-p))
                  (error "buffer-modified-p changed"))
                (unless (equal (buffer-substring-no-properties
                                (point-min) (point-max))
                               payload)
                  (error "region bytes changed"))
                (unless (equal (save-restriction
                                 (widen)
                                 (buffer-substring-no-properties
                                  (point-min) (point-max)))
                               full-bytes)
                  (error "full buffer bytes changed"))
                (unless (equal (match-data) saved-match)
                  (error "match-data changed"))
                (when (and dummy-proc (process-live-p dummy-proc))
                  (delete-process dummy-proc))
                (signal (car caught) (cdr caught)))))
        (when (and dummy-proc (process-live-p dummy-proc))
          (ignore-errors (delete-process dummy-proc)))
        (when (buffer-live-p dummy-buf)
          (ignore-errors (kill-buffer dummy-buf)))
        (when (buffer-live-p test-buf)
          (ignore-errors (kill-buffer test-buf))))))

  (ert-deftest femacs-remote09-diag-self-test-string-and-multibyte-failure ()
    "Verify unibyte base64url and multibyte string failures with byte 255."
    (setq femacs-remote09-diag--captures nil)
    (let ((unibyte-bad (unibyte-string ?_ ?- ?8 ?= 255))
          (multibyte-bad (string-to-multibyte
                          (unibyte-string ?T ?W ?F ?u 255)))
          (caught1 nil)
          (caught2 nil))
      (condition-case err
          (base64-decode-string unibyte-bad t)
        (error (setq caught1 err)))
      (unless (equal caught1 '(error "Invalid base64 data"))
        (error "unexpected unibyte string condition: %S" caught1))
      (condition-case err
          (base64-decode-string multibyte-bad)
        (error (setq caught2 err)))
      (unless (equal caught2 '(error "Invalid base64 data"))
        (error "unexpected multibyte string condition: %S" caught2))
      (signal (car caught2) (cdr caught2))))

  (ert-deftest femacs-remote09-diag-self-test-success-path ()
    "Verify valid Base64 and ignore-invalid calls succeed without captures."
    (setq femacs-remote09-diag--captures nil)
    (with-temp-buffer
      (set-buffer-multibyte nil)
      (insert "PREFIX:TWFu:SUFFIX")
      (narrow-to-region 8 12)
      (goto-char 8)
      (should (= (base64-decode-region (point-min) (point-max)) 3))
      (should (equal (buffer-string) "Man"))
      (widen)
      (should (equal (buffer-string) "PREFIX:Man:SUFFIX")))
    (with-temp-buffer
      (set-buffer-multibyte nil)
      (insert (unibyte-string ?_ ?- 255 ?8 ?=))
      (should (= (base64-decode-region (point-min) (point-max) t t) 2))
      (should (equal (buffer-string) (unibyte-string 255 239))))
    (should (equal (base64-decode-string "TWFu") "Man"))
    (should (equal (base64-decode-string
                    (unibyte-string ?T ?W ?F 255 ?u) nil t)
                   "Man"))
    (should (null femacs-remote09-diag--captures))))

(provide 'file-notify-remote-diagnostic)
;;; file-notify-remote-diagnostic.el ends here
