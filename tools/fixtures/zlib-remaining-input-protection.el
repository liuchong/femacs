;;; zlib-remaining-input-protection.el --- remaining input regression -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'ert)
(require 'base64)

(ert-deftest fe-zlib-remaining-input-after-corruption ()
  "Report unconsumed bytes after an error, rather than always returning zero."
  (dolist (symbol '(zlib-available-p zlib-decompress-region))
    (should (and (fboundp symbol) (subrp (symbol-function symbol)))))
  (should (zlib-available-p))
  ;; gzip.compress(b"alpha\nbeta\n", mtime=0), with the final length byte
  ;; XORed with 0xff, followed by five additional compressed-region bytes.
  ;; The failed stream consumes 31 bytes and produces 11 before its error.
  (let ((compressed
         (concat (base64-decode-string
                  "H4sIAAAAAAAC/0vMKchI5EpKLUnkAgBuUDBuCwAA/w==")
                 "TRAIL")))
    (with-temp-buffer
      (set-buffer-multibyte nil)
      (insert "P")
      (let ((start (point)))
        (insert compressed)
        (let ((end (point)))
          (insert "S")
          (goto-char start)
          (let ((remaining (zlib-decompress-region start end t)))
            (should (integerp remaining))
            (should (= remaining 5))
            ;; ALLOW-PARTIAL replaces the whole requested region; the
            ;; unconsumed byte count is returned, not left in the buffer.
            (should (equal (buffer-string) "Palpha\nbeta\nS"))
            (should (= (point) start))))))))

(provide 'zlib-remaining-input-protection)
;;; zlib-remaining-input-protection.el ends here
