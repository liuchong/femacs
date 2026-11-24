;;; items-core.el --- Shared Lisp domain logic for offline build and HTTP app  -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure item catalog, HTML rendering, and JSON payload encoding shared between
;; the standalone offline site generator (`build-site.el`) and the live HTTP
;; handler (`http-app.el`).

;;; Code:

(defconst femacs-web-sample-records
  '(((id . "item-1") (title . "Standalone Lisp Host") (status . "ready"))
    ((id . "item-2") (title . "Relocatable Runtime") (status . "ready"))
    ((id . "item-3") (title . "Socketpair HTTP Bridge") (status . "active")))
  "Canonical catalog items rendered identically offline and over HTTP.")

(defun femacs-web-format-items (&optional records)
  "Normalize RECORDS into deterministic alists with id, title, and label fields."
  (let ((source (or records femacs-web-sample-records)))
    (mapcar
     (lambda (rec)
       (let ((id (alist-get 'id rec))
             (title (alist-get 'title rec))
             (status (alist-get 'status rec)))
         (list (cons 'id id)
               (cons 'title title)
               (cons 'status status)
               (cons 'label (format "[%s] %s (%s)" id title status)))))
     source)))

(defun femacs-web-escape-html (text)
  "Escape HTML special characters in TEXT."
  (let ((out text))
    (setq out (replace-regexp-in-string "&" "&amp;" out t t))
    (setq out (replace-regexp-in-string "<" "&lt;" out t t))
    (setq out (replace-regexp-in-string ">" "&gt;" out t t))
    (setq out (replace-regexp-in-string "\"" "&quot;" out t t))
    (replace-regexp-in-string "'" "&#39;" out t t)))

(defun femacs-web-render-html (&optional records)
  "Render normalized RECORDS as a self-contained HTML5 document."
  (let* ((items (femacs-web-format-items records))
         (lis (mapconcat
               (lambda (item)
                 (format "      <li data-id=\"%s\">%s</li>"
                         (femacs-web-escape-html (alist-get 'id item))
                         (femacs-web-escape-html (alist-get 'label item))))
               items
               "\n")))
    (concat
     "<!doctype html>\n"
     "<html lang=\"en\">\n"
     "<head>\n"
     "  <meta charset=\"utf-8\">\n"
     "  <title>FEmacs Standalone Lisp Web Example</title>\n"
     "</head>\n"
     "<body>\n"
     "  <main>\n"
     "    <h1>FEmacs Standalone Lisp Items</h1>\n"
     "    <ul id=\"items\">\n"
     lis "\n"
     "    </ul>\n"
     "  </main>\n"
     "</body>\n"
     "</html>\n")))

(defun femacs-web-items-payload (&optional records)
  "Encode normalized RECORDS into a deterministic JSON string."
  (let ((items (femacs-web-format-items records)))
    (json-serialize
     (list (cons 'ok t)
           (cons 'count (length items))
           (cons 'items (vconcat items))))))

(provide 'femacs-web-core)
;;; items-core.el ends here
