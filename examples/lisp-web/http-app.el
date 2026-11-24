;;; http-app.el --- Lisp HTTP handler evaluated behind http-host  -*- lexical-binding: t; -*-

;;; Commentary:
;; Implements `femacs-http-handle' for `femacs-lisp --rpc http`.
;; Serves HTML/JSON endpoints via shared `femacs-web-core` functions.

;;; Code:

(let ((here (file-name-directory (or load-file-name buffer-file-name default-directory))))
  (add-to-list 'load-path here)
  (require 'subr-x)
  (require 'femacs-web-core (expand-file-name "items-core" here)))

(defun femacs-web--json-response (status obj &optional extra-headers)
  "Build HTTP response alist for STATUS and serializable Lisp OBJ."
  (let ((headers (append '(("content-type" . "application/json; charset=utf-8"))
                         extra-headers)))
    (list (cons 'status status)
          (cons 'headers headers)
          (cons 'body (json-serialize obj)))))

(defun femacs-http-handle (req)
  "Dispatch HTTP REQ alist delivered over JSON-RPC 2.0 `http.handle`."
  (let* ((method (upcase (or (alist-get 'method req) "GET")))
         (path (or (alist-get 'path req) "/"))
         (query (or (alist-get 'query req) ""))
         (body (or (alist-get 'body req) "")))
    (cond
     ((and (string= method "GET") (string= path "/"))
      (list (cons 'status 200)
            (cons 'headers '(("content-type" . "text/html; charset=utf-8")))
            (cons 'body (femacs-web-render-html))))
     ((and (string= method "GET") (string= path "/api/items"))
      (list (cons 'status 200)
            (cons 'headers '(("content-type" . "application/json; charset=utf-8")))
            (cons 'body (femacs-web-items-payload))))
     ((and (string= method "POST") (string= path "/api/echo"))
      (femacs-web--json-response
       200
       (list (cons 'method method)
             (cons 'path path)
             (cons 'query query)
             (cons 'bytes (string-bytes body))
             (cons 'body body))))
     ((and (string= method "GET") (string= path "/api/boom-with-logs"))
      (princ "femacs-web log line 1\nfemacs-web log line 2\n")
      (message "femacs-web diagnostic stderr log before intentional error")
      (error "Intentional boom in /api/boom-with-logs"))
     ((and (string= method "GET") (string= path "/api/check-cloexec"))
      (with-temp-buffer
        (let* ((script (concat
                        "found=\"\"; n=3; "
                        "while [ \"$n\" -le 64 ]; do "
                        "  if [ -e \"/dev/fd/$n\" ]; then "
                        "    if [ -z \"$found\" ]; then found=\"$n\"; else found=\"$found,$n\"; fi; "
                        "  fi; "
                        "  n=$((n + 1)); "
                        "done; printf '%s' \"$found\""))
               (rc (call-process "/bin/sh" nil (current-buffer) nil "-c" script))
               (open-fds (string-trim (buffer-string))))
          (femacs-web--json-response
           200
           (list (cons 'exitCode rc)
                 (cons 'leakedFds open-fds)
                 (cons 'clean (if (string= open-fds "") t :false)))))))
     (t
      (femacs-web--json-response
       404
       (list (cons 'error "not found")
             (cons 'method method)
             (cons 'path path)))))))

(provide 'femacs-http-app)
;;; http-app.el ends here
