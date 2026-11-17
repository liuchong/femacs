;;; image-loader-protection.el --- PNG/JPEG loader protection -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

;; Protective checks for the port's own PNG and JPEG readers (PORTING.md
;; P-467).  They are not a replacement for the original image test files: they
;; cover what those files cannot reach in a batch session - a real decode on a
;; window-system frame, the memory (`:data') and file arms of both loaders, and
;; the `setjmp'/`longjmp' recovery paths with damaged and truncated input.
;;
;; Run it with a *graphical* product, because `image-size' needs a window-system
;; frame:
;;
;;   EMACSLOADPATH=<repo>/lisp EMACSDATA=<repo>/etc IMAGE_LOADER_ROOT=<repo> \
;;   IMAGE_LOADER_RESULT=/tmp/image-loader-result.txt \
;;   <product> -Q -l tools/fixtures/image-loader-protection.el
;;
;; The checks are driven directly rather than through ERT: ERT's runner waits
;; for the command loop in a GUI session (measured here: `ert-run-tests' in
;; `window-setup-hook' never returns), while the batch lane runs ERT normally.
;; The fixture is loaded as a file, so the checks start from
;; `window-setup-hook' - a top-level form still runs before the toolkit frame
;; exists.  Every damaged input lives in a fresh temporary file and is removed
;; by the probe; the repository and the user's directories are never written.

(require 'cl-lib)
(require 'subr-x)

(defmacro image-loader-protection--should (form)
  "Signal a check failure when FORM is nil."
  `(unless ,form (error "check failed: %S" ',form)))

(defun image-loader-protection--root ()
  "Return the repository root given to this probe."
  (or (getenv "IMAGE_LOADER_ROOT")
      (error "image-loader-protection needs IMAGE_LOADER_ROOT")))

(defun image-loader-protection--path (relative)
  (expand-file-name relative (image-loader-protection--root)))

(defun image-loader-protection--slurp (file)
  "Return FILE's bytes as a unibyte string."
  (with-temp-buffer
    (set-buffer-multibyte nil)
    (insert-file-contents-literally file)
    (buffer-string)))

(defun image-loader-protection--unibyte (&rest parts)
  "Concatenate PARTS into a unibyte string.
`concat' promotes its result to multibyte when any part is, which would
re-encode the PNG signature and stop it from being a signature at all."
  (with-temp-buffer
    (set-buffer-multibyte nil)
    (dolist (part parts) (insert part))
    (buffer-string)))

(defun image-loader-protection--write (bytes)
  "Write BYTES to a fresh temporary file and return its name."
  (let ((file (make-temp-file "image-loader-protection-" nil ".bin")))
    (with-temp-file file
      (set-buffer-multibyte nil)
      (insert bytes))
    file))

(defconst image-loader-protection--png-size '(100 . 200))
(defconst image-loader-protection--jpeg-size '(30 . 30))

(defun image-loader-protection--size (spec)
  (image-size (create-image spec) t))

(defun image-loader-protection--memory-size (type bytes)
  (image-size (create-image bytes type t) t))

(defun image-loader-protection--clean-decode-p (thunk valid-size)
  "Return non-nil when THUNK decodes VALID-SIZE and logs no loader error.
This is what distinguishes a real decode from the cache's placeholder image:
the placeholder can have the same dimensions (the JPEG fixtures are 30x30),
but a failed load leaves an image error in the log."
  (let ((before (length (image-loader-protection--log-text))))
    (let ((size (funcall thunk)))
      (and (equal size valid-size)
           (not (string-match-p "\\(PNG error\\|Error reading JPEG\\|Error decoding\\|Error parsing\\)"
                                (substring (image-loader-protection--log-text) before)))))))

(defun image-loader-protection--rejected-p (spec valid-size)
  "Return non-nil when SPEC does not decode into VALID-SIZE.
A rejected image surfaces either as a signalled error or as the cache's
placeholder size, so the check is that the reader never reports the real
dimensions for damaged input - and never crashes."
  (condition-case nil
      (not (equal (image-size (create-image spec) t) valid-size))
    (error t)))

(defun image-loader-protection--rejected-memory-p (type bytes valid-size)
  (condition-case nil
      (not (equal (image-loader-protection--memory-size type bytes) valid-size))
    (error t)))

(defun image-loader-protection--log-text ()
  (with-current-buffer (messages-buffer) (buffer-string)))

(defun image-loader-protection--error-logged-p (thunk pattern)
  "Call THUNK; return non-nil when the log gained a line matching PATTERN."
  (let ((before (length (image-loader-protection--log-text))))
    (condition-case nil (funcall thunk) (error nil))
    (let ((text (image-loader-protection--log-text)))
      (and (> (length text) before)
           (string-match-p pattern (substring text before))))))

(defun image-loader-protection--damaged (kind)
  "Run the damaged-input checks for KIND, one of `png' or `jpeg'."
  (let* ((png (image-loader-protection--slurp
               (image-loader-protection--path "test/data/image/blank-100x200.png")))
         (jpeg (image-loader-protection--slurp
                (image-loader-protection--path "test/data/image/black.jpg")))
         (png-cut (substring png 0 64))
         (png-bad (image-loader-protection--unibyte
                   (substring png 0 8) (make-string 48 #xff)))
         (jpeg-cut (substring jpeg 0 96))
         (jpeg-bad (image-loader-protection--unibyte
                    (substring jpeg 0 2) (make-string 64 0)
                    (substring jpeg 66)))
         files)
    (unwind-protect
        (pcase kind
          ('png
           ;; The valid PNG is 100x200, so the cache's placeholder size is
           ;; distinguishable; the first call for each distinct message also
           ;; proves the loader reported the error (Emacs collapses a repeated
           ;; identical message to "[N times]", so each is decoded once).
           (image-loader-protection--should
            (image-loader-protection--error-logged-p
             (lambda () (image-loader-protection--memory-size 'png png-cut))
             "PNG error"))
           (image-loader-protection--should
            (image-loader-protection--error-logged-p
             (lambda () (image-loader-protection--memory-size 'png png-bad))
             "PNG error"))
           (dolist (bytes (list png-cut png-bad))
             (let ((file (image-loader-protection--write bytes)))
               (push file files)
               (image-loader-protection--should
                (image-loader-protection--rejected-p
                 file image-loader-protection--png-size)))))
          ('jpeg
           ;; The only JPEG fixtures are 30x30, which is also the cache's
           ;; placeholder size, so rejection is read from the error log.
           (image-loader-protection--should
            (image-loader-protection--error-logged-p
             (lambda () (image-loader-protection--memory-size 'jpeg jpeg-bad))
             "Error reading JPEG image"))
           (let ((file (image-loader-protection--write jpeg-bad)))
             (push file files)
             (image-loader-protection--should
              (image-loader-protection--error-logged-p
               (lambda () (image-loader-protection--size file))
               "Error reading JPEG image")))
           ;; A truncated JPEG may only warn (C's own `JWRN_JPEG_EOF' path);
           ;; either outcome is C's, and neither may crash the editor.
           (let ((size (condition-case nil
                           (image-loader-protection--memory-size 'jpeg jpeg-cut)
                         (error nil))))
             (image-loader-protection--should
              (or (null size) (consp size))))))
      (dolist (file files)
        (ignore-errors (delete-file file))))
    t))

(defconst image-loader-protection--checks
  `((file-png . ,(lambda ()
                   (equal (image-loader-protection--size
                           (image-loader-protection--path
                            "test/data/image/blank-100x200.png"))
                          image-loader-protection--png-size)))
    (memory-png . ,(lambda ()
                     (let ((data (image-loader-protection--slurp
                                  (image-loader-protection--path
                                   "test/data/image/blank-100x200.png"))))
                       (equal (image-loader-protection--memory-size 'png data)
                              image-loader-protection--png-size))))
    (file-jpeg . ,(lambda ()
                    (equal (image-loader-protection--size
                            (image-loader-protection--path "test/data/image/black.jpg"))
                           image-loader-protection--jpeg-size)))
    (memory-jpeg . ,(lambda ()
                      (let ((data (image-loader-protection--slurp
                                   (image-loader-protection--path
                                    "test/data/image/black.jpg"))))
                        (equal (image-loader-protection--memory-size 'jpeg data)
                               image-loader-protection--jpeg-size))))
    (damaged-png . ,(lambda () (image-loader-protection--damaged 'png)))
    (damaged-jpeg . ,(lambda () (image-loader-protection--damaged 'jpeg)))
    (readers-recover . ,(lambda ()
                          ;; Clear the cache so the decoders actually run again
                          ;; instead of returning an already loaded image.
                          (clear-image-cache t)
                          (let ((png (image-loader-protection--slurp
                                      (image-loader-protection--path
                                       "test/data/image/blank-100x200.png")))
                                (jpeg (image-loader-protection--slurp
                                       (image-loader-protection--path
                                        "test/data/image/black.jpg"))))
                            (and (equal (image-loader-protection--memory-size 'png png)
                                        image-loader-protection--png-size)
                                 (equal (image-loader-protection--memory-size 'jpeg jpeg)
                                        image-loader-protection--jpeg-size)))))))

(defun image-loader-protection--report ()
  "Run the checks, write a summary, and exit with their status."
  (unless (display-graphic-p)
    (error "image-loader-protection needs a window-system frame"))
  ;; The log buffer must exist before a loader reports its error, or
  ;; `message_dolog' drops the line on the floor.
  (messages-buffer)
  (clear-image-cache t)
  (let* ((result (getenv "IMAGE_LOADER_RESULT"))
         (lines nil)
         (failures 0)
         (passes 0))
    (dolist (check image-loader-protection--checks)
      (let* ((name (car check))
             (outcome (condition-case err
                          (if (funcall (cdr check)) 'passed 'failed)
                        (error (format "error %S" err)))))
        (if (eq outcome 'passed)
            (setq passes (1+ passes))
          (setq failures (1+ failures)))
        (push (format "%s: %s" name outcome) lines)))
    (let ((summary (format "image-loader-protection: passed %d, failed %d\n%s\n"
                           passes failures (string-join (nreverse lines) "\n"))))
      (when result
        (with-temp-file result (insert summary)))
      (message "%s" summary)
      (kill-emacs (if (zerop failures) 0 1)))))

(add-hook 'window-setup-hook #'image-loader-protection--report)

;;; image-loader-protection.el ends here
