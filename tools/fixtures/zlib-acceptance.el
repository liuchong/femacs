;;; zlib-acceptance.el --- native zlib decompression acceptance -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

;; These constants are the exact bytes produced with Python 3's gzip/zlib
;; modules, without an external gzip process:
;;
;;   small = b"alpha\nbeta\n"
;;   large = b"0123456789abcdef" * 2500
;;   gzip.compress(small, mtime=0)
;;   zlib.compress(small)
;;   gzip.compress(large, mtime=0)
;;   gzip.compress(large, mtime=0)[:64]
;;   corrupt = bytearray(gzip.compress(small, mtime=0))
;;   corrupt[-1] ^= 0xff
;;
;; The constants are bounded, self-contained, and decoded only through the
;; native zlib primitives under test.

(require 'base64)
(require 'ert)

(defconst zlib-acceptance--small "alpha\nbeta\n")
(defconst zlib-acceptance--large
  (mapconcat #'identity (make-list 2500 "0123456789abcdef") ""))
(defconst zlib-acceptance--large-length 40000)
(defconst zlib-acceptance--large-hash
  "49b5147f78225eabab7fb57a6e7d2b40eb5f78495ec830db98b81ab452885769")
(defconst zlib-acceptance--partial-length 11369)
(defconst zlib-acceptance--partial-hash
  "6ee3f2331d21980b9f65fa460021023d381a92ff9defe15cd4da68eb1b630e74")

(defconst zlib-acceptance--gzip-small
  (base64-decode-string "H4sIAAAAAAAC/0vMKchI5EpKLUnkAgBuUDBuCwAAAA=="))
(defconst zlib-acceptance--zlib-small
  (base64-decode-string "eJxLzCnISORKSi1J5AIAGCcDtw=="))
(defconst zlib-acceptance--gzip-large
  (base64-decode-string
   "H4sIAAAAAAAC/+3HyQHAEAAAsJWUusZB2X+EDiL5JTwxvbnU1sdc3z7B3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3d3f3i/zQudYZAnAAA"))
(defconst zlib-acceptance--gzip-truncated
  (base64-decode-string
   "H4sIAAAAAAAC/+3HyQHAEAAAsJWUusZB2X+EDiL5JTwxvbnU1sdc3z7B3d3d3d3d3d3d3d3d3d3d3d3d3d3d3Q=="))
(defconst zlib-acceptance--gzip-corrupt
  (base64-decode-string "H4sIAAAAAAAC/0vMKchI5EpKLUnkAgBuUDBuCwAA/w=="))

(defmacro zlib-acceptance--with-region (prefix compressed suffix allow-partial &rest body)
  "Run BODY in a unibyte buffer around COMPRESSED.
PREFIX and SUFFIX surround the region; pass ALLOW-PARTIAL to the native API.
Bind `zlib-start',
`zlib-end', `zlib-original', and `zlib-result' for BODY."
  (declare (indent 4))
  `(with-temp-buffer
     (set-buffer-multibyte nil)
     (insert ,prefix)
     (let ((zlib-start (point)))
       (insert ,compressed)
       (let ((zlib-end (point)))
         (insert ,suffix)
         (let ((zlib-original (buffer-string)))
           (goto-char zlib-start)
           (let ((zlib-result (zlib-decompress-region zlib-start zlib-end ,allow-partial)))
             ,@body))))))

(ert-deftest zlib-acceptance-native-api ()
  "The product must expose both native zlib primitives."
  (dolist (symbol '(zlib-available-p zlib-decompress-region))
    (should (and (fboundp symbol)
                 (subrp (symbol-function symbol)))))
  (when (fboundp 'zlib-available-p)
    (should (zlib-available-p))))

(ert-deftest zlib-acceptance-gzip-small-region ()
  "Gzip decoding preserves prefix, suffix, point, and the success value."
  (zlib-acceptance--with-region "P" zlib-acceptance--gzip-small "S" nil
    (should (eq zlib-result t))
    (should (= (point) zlib-start))
    (should (equal (buffer-string)
                   (concat "P" zlib-acceptance--small "S")))))

(ert-deftest zlib-acceptance-zlib-small ()
  "Zlib-format decoding is auto-detected."
  (zlib-acceptance--with-region "" zlib-acceptance--zlib-small "" nil
    (should (eq zlib-result t))
    (should (= (point) 1))
    (should (equal (buffer-string) zlib-acceptance--small))))

(ert-deftest zlib-acceptance-gzip-multi-output ()
  "A single region produces more than one 16-KiB output step."
  (zlib-acceptance--with-region "" zlib-acceptance--gzip-large "" nil
    (should (eq zlib-result t))
    (should (= (buffer-size) zlib-acceptance--large-length))
    (should (= (point) 1))
    (should (string= (secure-hash 'sha256 (buffer-string))
                     zlib-acceptance--large-hash))))

(ert-deftest zlib-acceptance-truncated-default ()
  "A truncated stream returns nil and leaves the original region intact."
  (zlib-acceptance--with-region "P" zlib-acceptance--gzip-truncated "S" nil
    (should (null zlib-result))
    (should (= (point) zlib-start))
    (should (equal (buffer-string) zlib-original))))

(ert-deftest zlib-acceptance-corrupt-default ()
  "A corrupted stream returns nil and leaves the original region intact."
  (zlib-acceptance--with-region "P" zlib-acceptance--gzip-corrupt "S" nil
    (should (null zlib-result))
    (should (= (point) zlib-start))
    (should (equal (buffer-string) zlib-original))))

(ert-deftest zlib-acceptance-allow-partial ()
  "ALLOW-PARTIAL keeps decoded bytes and reports remaining input bytes."
  (zlib-acceptance--with-region "P" zlib-acceptance--gzip-truncated "S" t
    (should (integerp zlib-result))
    (should (= zlib-result 0))
    (should (= (buffer-size)
               (+ 2 zlib-acceptance--partial-length)))
    (should (= (point) zlib-start))
    (should (string= (secure-hash
                      'sha256
                      (buffer-substring-no-properties
                       zlib-start
                       (+ zlib-start zlib-acceptance--partial-length)))
                     zlib-acceptance--partial-hash))))

(ert-deftest zlib-acceptance-multibyte-error ()
  "The native primitive rejects a multibyte buffer without changing it."
  (with-temp-buffer
    (insert "not compressed")
    (let ((before (buffer-string))
          (point-before (point))
          (err (condition-case error
                   (progn
                     (zlib-decompress-region (point-min) (point-max))
                     nil)
                 (error error))))
      (should err)
      (should (string= (error-message-string err)
                       "This function can be called only in unibyte buffers"))
      (should (equal (buffer-string) before))
      (should (= (point) point-before)))))

(provide 'zlib-acceptance)

;;; zlib-acceptance.el ends here
