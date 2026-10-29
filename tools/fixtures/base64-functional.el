;;; base64-functional.el --- public byte/string/region workflows -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)

(ert-deftest fe-base64-functional/binary-roundtrip ()
  "Exercise both alphabets through actual string and buffer primitives."
  (let ((payloads (list "" "any text"
                        (apply #'unibyte-string (number-sequence 0 255)))))
    (dolist (url '(nil t))
      (dolist (payload payloads)
        (let ((encoded (if url (base64url-encode-string payload)
                         (base64-encode-string payload t))))
          (should (equal (base64-decode-string encoded url) payload))
          (should-not (multibyte-string-p
                       (base64-decode-string encoded url)))
          (with-temp-buffer
            (set-buffer-multibyte nil)
            (insert "<" payload ">")
            (let ((end (1- (point-max))))
              (if url (base64url-encode-region 2 end)
                (base64-encode-region 2 end t)))
            (should (equal (buffer-string) (concat "<" encoded ">")))
            (should (= (base64-decode-region 2 (1- (point-max)) url)
                       (length payload)))
            (should (equal (buffer-string) (concat "<" payload ">")))))))))

(ert-deftest fe-base64-functional/invalid-input-is-atomic ()
  "An invalid encoded byte signals and leaves the buffer and markers intact."
  (dolist (url '(nil t))
    (dolist (input (list (concat (unibyte-string 255) "YW55IHRleHQ=")
                        (concat "YW55" (unibyte-string 255) "IHRleHQ=")
                        (concat "YW55IHRleHQ=" (unibyte-string 255))))
      (should-error (base64-decode-string input url) :type 'error)
      (dolist (multibyte '(nil t))
        (with-temp-buffer
          (set-buffer-multibyte multibyte)
          (insert "<" input ">")
          (goto-char 3)
          (let ((before (buffer-string))
                (old-point (point))
                (marker (copy-marker (1- (point-max)) t)))
            (unwind-protect
                (progn
                  (should-error
                   (base64-decode-region 2 (1- (point-max)) url)
                   :type 'error)
                  (should (equal before (buffer-string)))
                  (should (= old-point (point)))
                  (should (= (marker-position marker) (1- (point-max)))))
              (set-marker marker nil))))))))

(ert-deftest fe-base64-functional/explicit-ignore-invalid ()
  "Only the explicit permissive argument ignores a bad encoded byte."
  (dolist (url '(nil t))
    (let ((input (concat "YW55" (unibyte-string 255) "IHRleHQ=")))
      (should (equal (base64-decode-string input url t) "any text"))
      (with-temp-buffer
        (set-buffer-multibyte nil)
        (insert input)
        (should (= (base64-decode-region (point-min) (point-max) url t) 8))
        (should (equal (buffer-string) "any text"))))))

(ert-deftest fe-base64-functional/alphabet-padding-and-whitespace ()
  "Keep the public alphabet, padding and whitespace behavior."
  (let ((payload (unibyte-string 251 255)))
    (should (equal (base64-encode-string payload t) "+/8="))
    (should (equal (base64url-encode-string payload) "-_8="))
    (should (equal (base64url-encode-string payload t) "-_8"))
    (should (equal (base64-decode-string "-_8" t) payload))
    (should-error (base64-decode-string "-_8=" nil) :type 'error)
    (should-error (base64-decode-string "+/8=" t) :type 'error)
    (should (equal (base64-decode-string " \tYW55\nIHRleHQ=\r\n")
                   "any text"))))

(provide 'base64-functional)
;;; base64-functional.el ends here
