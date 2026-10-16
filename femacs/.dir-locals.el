;;; Directory Local Variables         -*- no-byte-compile: t; -*-
;;; For more information see (info "(Emacs) Directory Variables")

;; The settings the reference's own checkout carries at its root, kept here for the trees the
;; port owns.  They are what makes the port's byte compiler produce the *same* files as the
;; reference's build: the docstring filling (`emacs-lisp-docstring-fill-column`, `fill-column`,
;; `sentence-end-double-space`) and the Lisp indentation (`indent-tabs-mode`).  Measured
;; 2026-10-16 (status 4.403): without them `lisp/emacs-lisp/bytecomp.elc` differed from the
;; reference's copy in four places, all inside one docstring, and the reference program compiling
;; the *same source* through the same recipe produced the reference's exact bytes - so the
;; difference was this file's absence, not the port's compiler.  Only the settings the products
;; depend on are carried; the reference's own C-mode and bug-reference entries are not the port's.
((nil . ((tab-width . 8)
         (sentence-end-double-space . t)
         (fill-column . 72)
         (emacs-lisp-docstring-fill-column . 72)))
 (emacs-lisp-mode . ((indent-tabs-mode . nil)))
 (lisp-data-mode . ((indent-tabs-mode . nil))))
