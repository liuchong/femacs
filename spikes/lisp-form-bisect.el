;;; lisp-form-bisect.el --- narrow a byte-compile failure to one top-level form  -*- lexical-binding: t -*-
;;
;; The L1 worklist (tools/l1-worklist.sh) reports failures per *file*.  This narrows one of them to
;; the top-level form that starts it: compile the file's first N forms and report each verdict, so
;; the first FAIL names the form that was added last.
;;
;;   <emacs> -batch --no-site-file --no-site-lisp -l spikes/lisp-form-bisect.el \
;;       prefix-scan <file.el> <scratch.el>
;;   <emacs> -batch --no-site-file --no-site-lisp -l spikes/lisp-form-bisect.el \
;;       write-prefix <file.el> <out.el> <N>
;;
;; Readings (docs/status.md 4.375): for lisp/emacs-lisp/checkdoc.el, prefixes 1..84 compile and form
;; 85 -- checkdoc-this-string-valid-engine, line 1472 -- is the first failure; every longer prefix
;; fails too.  The reference compiles the same 85-form prefix successfully (write-prefix, then
;; -f batch-byte-compile), while the port returns nil with no error text at all, which is why the
;; file-level worklist showed no message for this file.

(defun bis-forms (target)
  "The top-level forms of TARGET, as source strings."
  (with-temp-buffer
    (insert-file-contents target)
    (goto-char (point-min))
    (let (fs)
      (condition-case nil
          (while (not (eobp))
            (let ((start (point)))
              (read (current-buffer))
              (push (buffer-substring-no-properties start (point)) fs)))
        (error nil))
      (nreverse fs))))

(defun bis-header (target)
  "The first line of TARGET: the file's cookie and commentary line."
  (with-temp-buffer
    (insert-file-contents target)
    (buffer-substring-no-properties (point-min) (line-end-position 1))))

(defun bis-write (out header forms n)
  "Write OUT as HEADER followed by the first N of FORMS."
  (with-temp-file out
    (insert header)
    (let ((i 0))
      (dolist (f forms)
        (setq i (1+ i))
        (when (<= i n)
          (insert "\n" f))))))

(defun bis-short (s)
  "First 68 characters of S on one line."
  (let ((one (replace-regexp-in-string "\n" " " s)))
    (substring one 0 (min 68 (length one)))))

(defun bis-prefix-scan (target scratch)
  "Compile the first N forms of TARGET for every N and report each verdict."
  (let ((header (bis-header target))
        (forms (bis-forms target))
        (acc nil)
        (n 0))
    (princ (format "target=%s forms=%d\n" (file-name-nondirectory target) (length forms)))
    (dolist (f forms)
      (setq n (1+ n))
      (setq acc (append acc (list f)))
      (bis-write scratch header acc n)
      (let ((ok (condition-case e (byte-compile-file scratch) (error nil))))
        (princ (format "%s n=%d %s\n" (if ok "OK" "FAIL") n (bis-short f)))))))

(defun bis-main ()
  "Dispatch on the mode named on the command line."
  (let ((mode (car command-line-args-left))
        (rest (cdr command-line-args-left)))
    (cond
     ((equal mode "prefix-scan")
      (bis-prefix-scan (nth 0 rest) (nth 1 rest)))
     ((equal mode "write-prefix")
      (bis-write (nth 1 rest) (bis-header (nth 0 rest)) (bis-forms (nth 0 rest))
                 (string-to-number (nth 2 rest)))
      (princ (format "wrote %s\n" (nth 1 rest))))
     (t
      (princ "usage: lisp-form-bisect.el prefix-scan|write-prefix ...\n")))))

(bis-main)
