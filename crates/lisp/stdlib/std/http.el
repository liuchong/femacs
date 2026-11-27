;;; std/http.el --- Pure-Rust HTTP/HTTPS client & embedded HTTP server -*- lexical-binding: t; -*-

;;; Commentary:
;; Pure-Rust HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto`) and embedded
;; HTTP/1.1 localhost/TCP server.

;;; Code:

(require 'std/json)

(defun std-http-request (method url &optional headers body timeout-ms)
  "Perform HTTP/HTTPS request METHOD to URL with optional HEADERS, BODY, and TIMEOUT-MS.
Returns a hash-table with keys \"status\", \"ok\", \"headers\", \"body\",
\"body-bytes\", and \"url\"."
  (internal-host-call "%std-http-request" method url headers body timeout-ms))

(defun std-http-get (url &optional headers timeout-ms)
  "Send an HTTP GET request to URL."
  (std-http-request "GET" url headers nil timeout-ms))

(defun std-http-head (url &optional headers timeout-ms)
  "Send an HTTP HEAD request to URL."
  (std-http-request "HEAD" url headers nil timeout-ms))

(defun std-http-delete (url &optional headers timeout-ms)
  "Send an HTTP DELETE request to URL."
  (std-http-request "DELETE" url headers nil timeout-ms))

(defun std-http-post (url body &optional headers timeout-ms)
  "Send an HTTP POST request to URL with BODY."
  (std-http-request "POST" url headers body timeout-ms))

(defun std-http-put (url body &optional headers timeout-ms)
  "Send an HTTP PUT request to URL with BODY."
  (std-http-request "PUT" url headers body timeout-ms))

(defun std-http-patch (url body &optional headers timeout-ms)
  "Send an HTTP PATCH request to URL with BODY."
  (std-http-request "PATCH" url headers body timeout-ms))

(defun std-http-get-json (url &optional headers timeout-ms)
  "Fetch URL via GET and decode its response body as JSON."
  (let* ((resp (std-http-get url headers timeout-ms))
         (status (gethash "status" resp)))
    (unless (gethash "ok" resp)
      (error "HTTP GET %s returned status %s" url status))
    (std-json-parse (gethash "body" resp))))

(defun std-http-post-json (url payload &optional headers timeout-ms)
  "Encode PAYLOAD as JSON, POST to URL, and return parsed response hash-table."
  (let ((hdrs (copy-alist (or headers nil))))
    (unless (assoc-string "content-type" hdrs t)
      (push '("content-type" . "application/json") hdrs))
    (std-http-post url (std-json-stringify payload) hdrs timeout-ms)))

(defun std-http-server-start (&optional bind-addr)
  "Start an embedded pure-Rust HTTP/1.1 server on BIND-ADDR (default \"127.0.0.1:0\").
Returns a hash-table with keys \"id\", \"addr\", and \"port\"."
  (internal-host-call "%std-http-server-start" (or bind-addr "127.0.0.1:0")))

(defun std-http-server-accept (server-id &optional timeout-ms)
  "Wait up to TIMEOUT-MS ms (default 5000) for next request on SERVER-ID, or return nil."
  (internal-host-call "%std-http-server-accept" server-id (or timeout-ms 5000)))

(defun std-http-server-respond (server-id req-id status &optional headers body)
  "Send HTTP response STATUS/HEADERS/BODY for REQ-ID queued on SERVER-ID."
  (internal-host-call "%std-http-server-respond" server-id req-id status headers body))

(defun std-http-server-stop (server-id)
  "Stop embedded HTTP server SERVER-ID and join its listener thread."
  (internal-host-call "%std-http-server-stop" server-id))

(defun std-http-serve (bind-addr handler &optional max-requests)
  "Start an embedded HTTP server on BIND-ADDR and dispatch each request to HANDLER.
HANDLER receives the request hash-table and returns either:
- (STATUS HEADERS BODY) list
- (STATUS . BODY) cons
- a hash-table with \"status\", optional \"headers\", \"body\"
- or a string body (status 200).
If MAX-REQUESTS is non-nil, stop automatically after handling that many requests."
  (let* ((srv (std-http-server-start bind-addr))
         (srv-id (gethash "id" srv))
         (handled 0))
    (unwind-protect
        (while (or (null max-requests) (< handled max-requests))
          (let ((req (std-http-server-accept srv-id 5000)))
            (when req
              (let ((req-id (gethash "id" req))
                    (res (condition-case err
                             (funcall handler req)
                           (error (list 500 nil (error-message-string err))))))
                (cond
                 ((hash-table-p res)
                  (std-http-server-respond srv-id req-id
                                           (or (gethash "status" res) 200)
                                           (gethash "headers" res)
                                           (or (gethash "body" res) "")))
                 ((and (consp res) (integerp (car res)) (consp (cdr res)))
                  (std-http-server-respond srv-id req-id
                                           (nth 0 res) (nth 1 res) (nth 2 res)))
                 ((and (consp res) (integerp (car res)))
                  (std-http-server-respond srv-id req-id
                                           (car res) nil (cdr res)))
                 (t
                  (std-http-server-respond srv-id req-id 200 nil (format "%s" res))))
                (setq handled (1+ handled))))))
      (std-http-server-stop srv-id))
    handled))

(provide 'std/http)
;;; std/http.el ends here