;;; c-ctype-low-byte-probe.el --- the printing of low-byte-control codepoints  -*- lexical-binding: t -*-
;;
;; P-191: `lib/include/c-ctype.h`'s predicates compare an *int* character code in C, and the
;; port's mirror once compared `c as u8`.  The observable consequence is here: with
;; `escape-control-characters` on, a string whose characters have a control byte as their low
;; byte must print verbatim, because none of the codes is a control character.
;;
;; Run it under both programs and compare the single line:
;;     emacs -Q --batch -l spikes/c-ctype-low-byte-probe.el
;; The reading is `:printed`, which must contain the characters themselves (not `\15`/`\31`),
;; and `:roundtrip`, which must be t.

(let* ((cps '(#x0100 #x0107 #x010a #x011b #x017f #x2019 #x590d #x4e00
              #x1f600 #x1f60a #xffff #x10ffff))
       (string (apply #'string cps))
       (overrides '(t (escape-newlines . t) (escape-control-characters . t))))
  (prin1 (list :len (length string)
               :printed (with-temp-buffer
                          (prin1 string (current-buffer) overrides)
                          (buffer-string))
               :roundtrip (equal (read (with-temp-buffer
                                         (prin1 string (current-buffer) overrides)
                                         (buffer-string)))
                                 string))
         t)
  (terpri))
