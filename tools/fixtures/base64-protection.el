;;; base64-protection.el --- byte-domain decoder protection -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)
(require 'cl-lib)

(ert-deftest fe-base64-protection/entire-single-byte-domain ()
  "Each possible first byte is handled by the selected public alphabet."
  (dolist (url '(nil t))
    (let ((alphabet (if url
                        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
                      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")))
      (dotimes (byte 256)
        (let* ((input (concat (unibyte-string byte) "AAA"))
               (index (cl-position byte alphabet)))
          (cond
           (index
            (should (equal (base64-decode-string input url)
                           (unibyte-string (* index 4) 0 0))))
           ((and url (memq byte '(9 10 12 13 32)))
            ;; Whitespace is ignored and URL input permits missing padding.
            (should (equal (base64-decode-string input url)
                           (unibyte-string 0 0))))
           (t (should-error (base64-decode-string input url) :type 'error))))))))

(ert-deftest fe-base64-protection/final-byte-in-each-quartet-slot ()
  "The final byte value is rejected in every quartet position without mutation."
  (dolist (url '(nil t))
    (dotimes (slot 4)
      (let ((input (copy-sequence "AAAA")))
        (aset input slot 255)
        (should-error (base64-decode-string input url) :type 'error)
        (with-temp-buffer
          (set-buffer-multibyte nil)
          (insert input)
          (let ((before (buffer-string)))
            (should-error
             (base64-decode-region (point-min) (point-max) url)
             :type 'error)
            (should (equal before (buffer-string)))))))))

(provide 'base64-protection)
;;; base64-protection.el ends here
