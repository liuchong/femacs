;;; tls-acceptance.el --- GnuTLS integration acceptance -*- lexical-binding: t; -*-
(require 'cl-lib)
(require 'ert)
(require 'gnutls)
(require 'hex-util)

(let ((original (getenv "FEMACS_TLS_ORIGINAL")))
  (unless (and original (file-readable-p original))
    (error "Missing readable FEMACS_TLS_ORIGINAL"))
  (load original nil t t))

(defun fe-tls--env (name)
  (let ((value (getenv name)))
    (unless (and value (not (string-empty-p value)))
      (error "Missing %s" name))
    value))

(defun fe-tls--port ()
  (let ((port (string-to-number (fe-tls--env "FEMACS_TLS_PORT"))))
    (unless (> port 0)
      (error "Invalid FEMACS_TLS_PORT"))
    port))

(defun fe-tls--record (name value)
  (let* ((root (or (getenv "FEMACS_TLS_TEST_DIR")
                   (error "Missing FEMACS_TLS_TEST_DIR")))
         (path (expand-file-name (concat name ".el") root))
         (coding-system-for-write 'utf-8-unix))
    (with-temp-file path
      (prin1 value (current-buffer))
      (insert "\n"))))

(defun fe-tls--make-process (name nowait)
  (let ((buffer (generate-new-buffer name)))
    (condition-case failure
        (cons (make-network-process
               :name name :buffer buffer :host "127.0.0.1"
               :service (fe-tls--port) :family 'ipv4 :coding 'binary
               :noquery t :nowait nowait :sentinel #'ignore)
              buffer)
      (error
       (kill-buffer buffer)
       (signal (car failure) (cdr failure))))))

(defun fe-tls--sync-connect (hostname &optional trustfile)
  (let* ((pair (fe-tls--make-process "fe-tls-sync" nil))
         (process (car pair))
         (buffer (cdr pair)))
    (condition-case failure
        (progn
          (gnutls-negotiate
           :process process :type 'gnutls-x509pki :hostname hostname
           :trustfiles (list (or trustfile (fe-tls--env "FEMACS_TLS_CA")))
           :verify-error t :priority-string "NORMAL")
          pair)
      (error
       (ignore-errors (gnutls-deinit process))
       (ignore-errors (delete-process process))
       (kill-buffer buffer)
       (signal (car failure) (cdr failure))))))

(defun fe-tls--async-connect (hostname)
  (let* ((events (list nil))
         (buffer (generate-new-buffer "fe-tls-async"))
         (params (gnutls-boot-parameters
                  :type 'gnutls-x509pki :hostname hostname
                  :trustfiles (list (fe-tls--env "FEMACS_TLS_CA"))
                  :verify-error t :priority-string "NORMAL")))
    (condition-case failure
        (list (make-network-process
               :name "fe-tls-async" :buffer buffer :host "127.0.0.1"
               :service (fe-tls--port) :family 'ipv4 :coding 'binary
               :noquery t :nowait t
               :tls-parameters (cons 'gnutls-x509pki params)
               :sentinel (lambda (_process message)
                           (setcar events (cons message (car events)))))
              buffer events)
      (error
       (kill-buffer buffer)
       (signal (car failure) (cdr failure))))))

(defun fe-tls--cleanup (process buffer)
  (when (processp process)
    (ignore-errors (gnutls-deinit process))
    (when (process-live-p process)
      (ignore-errors (delete-process process))))
  (when (buffer-live-p buffer)
    (kill-buffer buffer)))

(defun fe-tls--read-pong (process)
  (let ((buffer (process-buffer process))
        (deadline (+ (float-time) 10)))
    (while (and (< (float-time) deadline)
                (not (with-current-buffer buffer
                       (string-match-p "pong\n" (buffer-string)))))
      (accept-process-output process 0.1))
    (with-current-buffer buffer
      (buffer-string))))

(defun fe-tls--close (process)
  ;; The nonblocking primitive may have sent close_notify while still waiting
  ;; for the peer's reply.  The Python peer independently requires both alerts.
  ;; Do not pump process input and retry against a potentially retired session.
  (let ((result (gnutls-bye process nil)))
    (should (memq result '(t gnutls-e-again gnutls-e-interrupted)))
    (fe-tls--record "close-result" result))
  (should (eq (gnutls-deinit process) t)))

(defun fe-tls--peer-summary (status)
  (let ((cert (plist-get status :certificate)))
    (list :protocol (plist-get status :protocol)
          :cipher (plist-get status :cipher)
          :mac (plist-get status :mac)
          :key-exchange (plist-get status :key-exchange)
          :warnings (plist-get status :warnings)
          :subject (plist-get cert :subject)
          :issuer (plist-get cert :issuer)
          :valid-from (plist-get cert :valid-from)
          :valid-to (plist-get cert :valid-to))))

(ert-deftest fe-tls-foundation ()
  (let ((capabilities (gnutls-available-p)))
    (should (consp capabilities))
    (dolist (capability '(gnutls3 macs digests ciphers AEAD-ciphers))
      (should (memq capability capabilities)))
    (should (integerp libgnutls-version))
    (should (> libgnutls-version 0)))
  (dolist (spec '((gnutls-available-p 0 0)
                  (gnutls-boot 3 3)
                  (gnutls-errorp 1 1)
                  (gnutls-error-fatalp 1 1)
                  (gnutls-error-string 1 1)
                  (gnutls-deinit 1 1)
                  (gnutls-bye 2 2)
                  (gnutls-peer-status 1 1)
                  (gnutls-peer-status-warning-describe 1 1)
                  (gnutls-format-certificate 1 1)
                  (gnutls-ciphers 0 0)
                  (gnutls-macs 0 0)
                  (gnutls-digests 0 0)
                  (gnutls-hash-mac 3 3)
                  (gnutls-hash-digest 2 2)
                  (gnutls-symmetric-encrypt 4 5)
                  (gnutls-symmetric-decrypt 4 5)
                  (gnutls-asynchronous-parameters 2 2)
                  (gnutls-get-initstage 1 1)))
    (let ((function (symbol-function (nth 0 spec))))
      (should (subrp function))
      (should (equal (subr-arity function)
                     (cons (nth 1 spec) (nth 2 spec)))))))

(ert-deftest fe-tls-error-contract ()
  (should-not (gnutls-errorp t))
  (should-not (gnutls-errorp 'gnutls-e-again))
  (should (gnutls-errorp nil))
  (should (gnutls-errorp -1))
  (should-not (gnutls-error-fatalp t))
  (should-not (gnutls-error-fatalp 'gnutls-e-again))
  (should (equal (gnutls-error-string t) "Not an error"))
  (should (equal (gnutls-error-string "not-an-error")
                 "Not an error symbol or code"))
  (should (equal (gnutls-error-string 'fe-tls-no-such-error)
                 "Symbol has no numeric gnutls-code property"))
  (should-error (gnutls-error-fatalp "not-an-error") :type 'error)
  (should-error (gnutls-error-fatalp 'fe-tls-no-such-error) :type 'error)
  (should-error (gnutls-format-certificate 42) :type 'wrong-type-argument)
  (should-error (gnutls-format-certificate "not a certificate") :type 'error)
  (should-error (gnutls-hash-digest 'FE-TLS-NO-SUCH-DIGEST "x") :type 'error)
  (should-error (gnutls-hash-mac 'FE-TLS-NO-SUCH-MAC "key" "x") :type 'error)
  (should-error (gnutls-symmetric-encrypt
                 'FE-TLS-NO-SUCH-CIPHER
                 (make-string 16 0) (make-string 16 0) (make-string 16 0))
                :type 'error)
  (should-error (gnutls-symmetric-encrypt
                 'AES-128-CBC
                 (make-string 15 0) (make-string 16 0) (make-string 16 0))
                :type 'error)
  (should-error (gnutls-symmetric-encrypt
                 'AES-128-CBC
                 (make-string 16 0) (make-string 15 0) (make-string 16 0))
                :type 'error)
  (should-error (gnutls-symmetric-encrypt
                 'AES-128-CBC
                 (make-string 16 0) (make-string 16 0) (make-string 15 0))
                :type 'error)
  (should-error (gnutls-hash-digest 'SHA256 42) :type 'wrong-type-argument)
  (should-error (gnutls-hash-mac 'SHA256 42 "x") :type 'wrong-type-argument)
  (should (equal (encode-hex-string (gnutls-hash-digest 'SHA256 "abc"))
                 "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")))

(ert-deftest fe-tls-known-vectors ()
  (should (equal (encode-hex-string (gnutls-hash-digest 'SHA256 "abc"))
                 "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"))
  (let ((key (make-string 20 11)))
    (should (equal (encode-hex-string (gnutls-hash-mac 'SHA256 key "Hi There"))
                   "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7")))
  (let* ((cipher (or (assq 'AES-128-CBC (gnutls-ciphers))
                     (ert-fail "AES-128-CBC is unavailable")))
         (cplist (cdr cipher))
         (key (decode-hex-string "2b7e151628aed2a6abf7158809cf4f3c"))
         (iv (decode-hex-string "000102030405060708090a0b0c0d0e0f"))
         (input (decode-hex-string "6bc1bee22e409f96e93d7e117393172a"))
         (mutable (copy-sequence key))
         (output (gnutls-symmetric-encrypt cplist mutable iv input))
         (data (nth 0 output))
         (actual-iv (nth 1 output)))
    (should (equal (encode-hex-string data)
                   "7649abac8119b246cee98e9b12e9197d"))
    (should (equal actual-iv iv))
    (should (equal mutable (make-string 16 0)))
    (should (equal (nth 0 (gnutls-symmetric-decrypt
                           cplist (copy-sequence key) actual-iv data))
                   input)))
  (let* ((cipher (or (assq 'AES-128-GCM (gnutls-ciphers))
                     (ert-fail "AES-128-GCM is unavailable")))
         (cplist (cdr cipher))
         (key (make-string 16 0))
         (iv (make-string 12 0))
         (input (make-string 16 0))
         (output (gnutls-symmetric-encrypt cplist (copy-sequence key) iv input))
         (data (nth 0 output))
         (actual-iv (nth 1 output)))
    (should (equal (encode-hex-string data)
                   "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf"))
    (should (equal actual-iv iv))
    (should (equal (nth 0 (gnutls-symmetric-decrypt
                           cplist (copy-sequence key) actual-iv data))
                   input))
    (let ((tampered (copy-sequence data)))
      (aset tampered (1- (length tampered))
            (logxor (aref tampered (1- (length tampered))) 1))
      (should-error (gnutls-symmetric-decrypt
                     cplist (copy-sequence key) actual-iv tampered)
                    :type 'error))
    (should (equal (gnutls-hash-digest 'SHA256 "still usable")
                   (secure-hash 'sha256 "still usable" nil nil t)))))

(ert-deftest fe-tls-buffer-region ()
  (let* ((text "xabc")
         (expected (secure-hash 'sha256 "abc" nil nil t)))
    (should (equal (gnutls-hash-digest 'SHA256 (list text 1 4)) expected))
    (with-temp-buffer
      (insert text)
      (should (equal (gnutls-hash-digest 'SHA256 (current-buffer))
                     (secure-hash 'sha256 (current-buffer) nil nil t)))
      (should (equal (gnutls-hash-digest
                      'SHA256 (list (current-buffer) 2 5))
                     expected)))))

(ert-deftest fe-tls-local-lifecycle ()
  (let* ((pair (fe-tls--sync-connect "localhost"))
         (process (car pair))
         (buffer (cdr pair)))
    (unwind-protect
        (progn
          (should (eq (process-status process) 'open))
          (dotimes (_ 20) (garbage-collect))
          (should (eq (process-status process) 'open))
          (let* ((status (gnutls-peer-status process))
                 (cert (plist-get status :certificate))
                 (text (gnutls-format-certificate (plist-get cert :pem))))
            (should (listp status))
            (should-not (plist-get status :warnings))
            (should (equal (plist-get cert :subject) "CN=localhost"))
            (should (stringp (plist-get status :protocol)))
            (should (stringp (plist-get status :cipher)))
            (should (stringp (plist-get status :mac)))
            (should (stringp text))
            (should (string-match-p "Subject:" text))
            (fe-tls--record "lifecycle" (fe-tls--peer-summary status)))
          (process-send-string process "ping\n")
          (should (equal (fe-tls--read-pong process) "pong\n"))
          (fe-tls--close process)
          (should (process-live-p process)))
      (fe-tls--cleanup process buffer))))

(ert-deftest fe-tls-local-hostname-rejection ()
  (let* ((pair (fe-tls--make-process "fe-tls-hostname" nil))
         (process (car pair))
         (buffer (cdr pair)))
    (unwind-protect
        (let* ((failure
                (should-error
                 (gnutls-negotiate
                  :process process :type 'gnutls-x509pki
                  :hostname "wrong.example"
                  :trustfiles (list (fe-tls--env "FEMACS_TLS_CA"))
                  :verify-error t :priority-string "NORMAL")
                 :type 'error))
               (message (error-message-string failure)))
          (should (string-match-p "does not match" message))
          (fe-tls--record "hostname-rejection" message))
      (fe-tls--cleanup process buffer))))

(ert-deftest fe-tls-local-wrong-ca ()
  (let* ((pair (fe-tls--make-process "fe-tls-wrong-ca" nil))
         (process (car pair))
         (buffer (cdr pair)))
    (unwind-protect
        (let* ((failure
                (should-error
                 (gnutls-negotiate
                  :process process :type 'gnutls-x509pki
                  :hostname "localhost"
                  :trustfiles (list (fe-tls--env "FEMACS_TLS_WRONG_CA"))
                  :verify-error t :priority-string "NORMAL")
                 :type 'error))
               (message (error-message-string failure)))
          (should (string-match-p "Certificate validation failed" message))
          (fe-tls--record "wrong-ca-rejection" message))
      (fe-tls--cleanup process buffer))))

(ert-deftest fe-tls-local-async ()
  (let* ((state (fe-tls--async-connect "localhost"))
         (process (nth 0 state))
         (buffer (nth 1 state))
         (events (nth 2 state)))
    (unwind-protect
        (progn
          (dotimes (_ 20) (garbage-collect))
          (let ((deadline (+ (float-time) 10)))
            (while (and (eq (process-status process) 'connect)
                        (< (float-time) deadline))
              (accept-process-output process 0.1)))
          (should (eq (process-status process) 'open))
          (accept-process-output process 0.1)
          (should (cl-some (lambda (message)
                             (string-match-p "open" message))
                           (car events)))
          (fe-tls--record "async-events" (car events))
          (let* ((status (gnutls-peer-status process))
                 (cert (plist-get status :certificate)))
            (should (listp status))
            (should-not (plist-get status :warnings))
            (should (equal (plist-get cert :subject) "CN=localhost"))
            (fe-tls--record "async" (fe-tls--peer-summary status)))
          (process-send-string process "ping\n")
          (should (equal (fe-tls--read-pong process) "pong\n"))
          (fe-tls--close process))
      (fe-tls--cleanup process buffer))))

(provide 'tls-acceptance)
;;; tls-acceptance.el ends here
