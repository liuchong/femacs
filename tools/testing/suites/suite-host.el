;;; suite-host.el --- Pure Lisp host & http-host integration tests -*- lexical-binding: t; -*-

;;; Commentary:
;; Tests standalone `dist/lisp/bin/femacs-lisp` CLI flags, script argument
;; forwarding, exit codes, `unwind-protect` on `kill-emacs`, multiline `--repl`,
;; and `dist/lisp/bin/http-host` JSON-RPC HTTP dispatch without Python.

;;; Code:

(require 'fe-test-dsl)

(let* ((targets (fe-test-targets))
       (lisp-host (plist-get targets :lisp-host))
       (http-host (plist-get targets :http-host))
       (inputs (and lisp-host (fe-test-product-runtime-inputs lisp-host)))
       (cacheable (not (null inputs))))
  (when lisp-host
    (fe-test-register-case
     :id "host/cli-version-help"
     :suite "host"
     :title "femacs-lisp --version, --help, and bad-flag exit code 2"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    (concat
                     "(let ((exe " (prin1-to-string lisp-host) "))"
                     "  (with-temp-buffer"
                     "    (let ((rc1 (call-process exe nil t nil \"--version\")))"
                     "      (unless (and (eql rc1 0) (string-match-p \"femacs-lisp\" (buffer-string)))"
                     "        (error \"version mismatch: %s\" (buffer-string))))"
                     "    (erase-buffer)"
                     "    (let ((rc2 (call-process exe nil t nil \"--help\")))"
                     "      (unless (and (eql rc2 0) (string-match-p \"--script\" (buffer-string)))"
                     "        (error \"help mismatch: %s\" (buffer-string))))"
                     "    (erase-buffer)"
                     "    (let ((rc3 (call-process exe nil t nil \"--definitely-unknown-flag\")))"
                     "      (unless (eql rc3 2)"
                     "        (error \"expected exit 2, got %s\" rc3)))))"))
     :timeout-ms 20000
     :cacheable cacheable
     :cache-inputs inputs
     :tags '("host" "fast"))

    (fe-test-register-case
     :id "host/eval-and-script-args"
     :suite "host"
     :title "femacs-lisp non-batch --eval printing and --script argument forwarding"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    (concat
                     "(let* ((exe " (prin1-to-string lisp-host) ")"
                     "       (tmp (make-temp-file \"fe-host-script-\" nil \".el\")))"
                     "  (unwind-protect"
                     "      (progn"
                     "        (with-temp-file tmp"
                     "          (insert \"(princ (json-serialize (vconcat command-line-args-left)))\\n\"))"
                     "        (with-temp-buffer"
                     "          (let ((rc (call-process exe nil t nil \"-Q\" \"--eval\" \"(+ 20 22)\")))"
                     "            (unless (and (eql rc 0) (string= (string-trim (buffer-string)) \"42\"))"
                     "              (error \"eval output mismatch: %s\" (buffer-string))))"
                     "          (erase-buffer)"
                     "          (let ((rc (call-process exe nil t nil \"--batch\" \"-Q\" \"--script\" tmp \"--\" \"alpha\" \"beta\")))"
                     "            (unless (and (eql rc 0) (string= (string-trim (buffer-string)) \"[\\\"alpha\\\",\\\"beta\\\"]\"))"
                     "              (error \"script args mismatch: %s\" (buffer-string))))))"
                     "    (ignore-errors (delete-file tmp))))"))
     :env (list (cons "TMPDIR" (fe-test-path "build/test-framework/tmp")))
     :timeout-ms 20000
     :cacheable cacheable
     :cache-inputs inputs
     :tags '("host" "fast"))

    (fe-test-register-case
     :id "host/exit-codes-and-unwind"
     :suite "host"
     :title "femacs-lisp kill-emacs exit codes (0/7), error (1), and C4 unwind contract"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    (concat
                     "(let ((exe " (prin1-to-string lisp-host) "))"
                     "  (unless (eql 0 (call-process exe nil nil nil \"--batch\" \"-Q\" \"--eval\" \"(kill-emacs 0)\"))"
                     "    (error \"expected exit 0\"))"
                     "  (unless (eql 7 (call-process exe nil nil nil \"--batch\" \"-Q\" \"--eval\" \"(kill-emacs 7)\"))"
                     "    (error \"expected exit 7\"))"
                     "  (unless (eql 1 (call-process exe nil nil nil \"--batch\" \"-Q\" \"--eval\" \"(error \\\"boom\\\")\"))"
                     "    (error \"expected exit 1\"))"
                     "  (with-temp-buffer"
                     "    (let ((rc (call-process exe nil t nil \"--batch\" \"-Q\" \"--eval\""
                     "                          \"(unwind-protect (kill-emacs 7) (princ \\\"BAD-CLEANUP\\\") (kill-emacs 2))\")))"
                     "      (unless (and (eql rc 7) (not (string-match-p \"BAD-CLEANUP\" (buffer-string))))"
                     "        (error \"kill-emacs bypass contract failed rc=%s out=%S\" rc (buffer-string)))))"
                     "  (let ((cleaned nil))"
                     "    (condition-case nil"
                     "        (unwind-protect (error \"inner\") (setq cleaned t))"
                     "      (error nil))"
                     "    (unless cleaned (error \"normal error unwind-protect did not run\"))))"))
     :timeout-ms 25000
     :cacheable cacheable
     :cache-inputs inputs
     :tags '("host" "fast"))

    (fe-test-register-case
     :id "host/repl-multiline"
     :suite "host"
     :title "femacs-lisp --repl evaluates multiline form and exits on EOF"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    (concat
                     "(let* ((exe " (prin1-to-string lisp-host) ")"
                     "       (buf (generate-new-buffer \" *repl-out*\"))"
                     "       (process-connection-type nil)"
                     "       (proc (make-process :name \"repl-test\""
                     "                           :buffer buf"
                     "                           :command (list exe \"-Q\" \"--repl\")"
                     "                           :connection-type (quote pipe)"
                     "                           :noquery t)))"
                     "  (unwind-protect"
                     "      (progn"
                     "        (process-send-string proc \"(+\\n 20\\n 22)\\n\")"
                     "        (process-send-eof proc)"
                     "        (let ((deadline (+ (float-time) 10.0)))"
                     "          (while (and (process-live-p proc) (< (float-time) deadline))"
                     "            (accept-process-output proc 0.05)))"
                     "        (when (process-live-p proc)"
                     "          (kill-process proc)"
                     "          (error \"repl timed out\"))"
                     "        (unless (eql (process-exit-status proc) 0)"
                     "          (error \"repl exit=%s\" (process-exit-status proc)))"
                     "        (with-current-buffer buf"
                     "          (unless (string-match-p \"42\" (buffer-string))"
                     "            (error \"repl missing 42: %S\" (buffer-string)))))"
                     "    (when (process-live-p proc) (ignore-errors (kill-process proc)))"
                     "    (kill-buffer buf)))"))
     :timeout-ms 20000
     :cacheable cacheable
     :cache-inputs inputs
     :tags '("host" "fast")))

  (when (and lisp-host http-host)
    (fe-test-register-case
     :id "host/http-host-roundtrip"
     :suite "host"
     :title "http-host binds localhost, serves Lisp routes, returns 404, and shuts down"
     :command (list lisp-host "--batch" "-Q" "--eval"
                    (concat
                     "(let* ((http-exe " (prin1-to-string http-host) ")"
                     "       (lisp-exe " (prin1-to-string lisp-host) ")"
                     "       (handler \"examples/lisp-web/http-app.el\")"
                     "       (buf (generate-new-buffer \" *http-host*\"))"
                     "       (process-connection-type nil)"
                     "       (proc (make-process :name \"http-host-test\""
                     "                           :buffer buf"
                     "                           :command (list http-exe \"--listen\" \"127.0.0.1:0\""
                     "                                          \"--lisp-bin\" lisp-exe"
                     "                                          \"--handler\" handler)"
                     "                           :connection-type (quote pipe)"
                     "                           :noquery t))"
                     "       (addr nil))"
                     "  (unwind-protect"
                     "      (progn"
                     "        (let ((deadline (+ (float-time) 15.0)))"
                     "          (while (and (not addr) (process-live-p proc) (< (float-time) deadline))"
                     "            (accept-process-output proc 0.05)"
                     "            (with-current-buffer buf"
                     "              (when (string-match \"LISTENING_ADDR=\\\\([^\\r\\n]+\\\\)\" (buffer-string))"
                     "                (setq addr (match-string 1 (buffer-string)))))))"
                     "        (unless addr"
                     "          (error \"http-host did not bind; output=%S\" (with-current-buffer buf (buffer-string))))"
                     "        (with-temp-buffer"
                     "          (let ((rc (call-process \"/usr/bin/curl\" nil t nil \"-sS\" \"-w\" \"\\nHTTP_CODE=%{http_code}\" (format \"http://%s/api/items\" addr))))"
                     "            (unless (and (eql rc 0) (string-match-p \"HTTP_CODE=200\" (buffer-string)))"
                     "              (error \"GET /api/items failed: %s\" (buffer-string))))"
                     "          (erase-buffer)"
                     "          (let ((rc (call-process \"/usr/bin/curl\" nil t nil \"-sS\" \"-X\" \"POST\" \"-d\" \"hello\" \"-w\" \"\\nHTTP_CODE=%{http_code}\" (format \"http://%s/api/echo?x=1\" addr))))"
                     "            (unless (and (eql rc 0)"
                     "                         (string-match-p \"HTTP_CODE=200\" (buffer-string))"
                     "                         (string-match-p \"\\\"body\\\":\\\"hello\\\"\" (buffer-string)))"
                     "              (error \"POST /api/echo failed: %s\" (buffer-string))))"
                     "          (erase-buffer)"
                     "          (let ((rc (call-process \"/usr/bin/curl\" nil t nil \"-sS\" \"-w\" \"\\nHTTP_CODE=%{http_code}\" (format \"http://%s/missing-route\" addr))))"
                     "            (unless (and (eql rc 0) (string-match-p \"HTTP_CODE=404\" (buffer-string)))"
                     "              (error \"GET /missing-route expected 404: %s\" (buffer-string))))))"
                     "    (when (process-live-p proc) (ignore-errors (signal-process proc (quote TERM))))"
                     "    (let ((wait-end (+ (float-time) 3.0)))"
                     "      (while (and (process-live-p proc) (< (float-time) wait-end))"
                     "        (accept-process-output nil 0.05)))"
                     "    (when (process-live-p proc) (ignore-errors (kill-process proc)))"
                     "    (kill-buffer buf)))"))
     :timeout-ms 25000
     :cacheable cacheable
     :cache-inputs (and inputs (append (list http-host "examples/lisp-web/http-app.el" "examples/lisp-web/items-core.el") inputs))
     :tags '("host" "fast"))))

;;; suite-host.el ends here
