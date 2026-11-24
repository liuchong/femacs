;;; build-site.el --- Offline static HTML + JSON generator  -*- lexical-binding: t; -*-

;;; Commentary:
;; Builds static `index.html` and `items.json` outputs using `femacs-web-core`.
;; Usage: femacs-lisp --script examples/lisp-web/build-site.el -- --out DIR

;;; Code:

(let ((here (file-name-directory (or load-file-name buffer-file-name default-directory))))
  (add-to-list 'load-path here)
  (require 'femacs-web-core (expand-file-name "items-core" here)))

(defun femacs-web--parse-out-dir (args)
  "Extract `--out DIR' or first positional path from ARGS."
  (let ((out nil)
        (rest args))
    (while rest
      (let ((cur (car rest)))
        (cond
         ((string= cur "--out")
          (setq rest (cdr rest))
          (unless rest
            (error "--out requires a directory argument"))
          (setq out (car rest)))
         ((string-prefix-p "--out=" cur)
          (setq out (substring cur (length "--out="))))
         ((null out)
          (setq out cur))
         (t
          (error "Unexpected argument: %s" cur))))
      (setq rest (cdr rest)))
    (or out "site-out")))

(let* ((out-dir (expand-file-name (femacs-web--parse-out-dir command-line-args-left)))
       (html-path (expand-file-name "index.html" out-dir))
       (json-path (expand-file-name "items.json" out-dir))
       (html-body (femacs-web-render-html))
       (json-body (femacs-web-items-payload)))
  (make-directory out-dir t)
  (with-temp-file html-path
    (insert html-body))
  (with-temp-file json-path
    (insert json-body "\n"))
  (princ (format "Generated %s and %s\n" html-path json-path)))

;;; build-site.el ends here
