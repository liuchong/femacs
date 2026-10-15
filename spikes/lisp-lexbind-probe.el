;;; lisp-lexbind-probe.el --- report the lexical-binding state and the compiler's object shape
;;
;; Run it with both programs and compare (docs/status.md 4.374):
;;
;;   EMACSLOADPATH=<repo>/emacs/lisp build/emacs-ref/src/temacs -Q --batch -l spikes/lisp-lexbind-probe.el
;;   build/port/src/temacs -Q --batch -l spikes/lisp-lexbind-probe.el
;;
;; What it reports, and why:
;;
;; * the four spellings of `lexical-binding`'s state.  In the reference the buffer-local value and
;;   the toplevel value are both nil while the default is t; in the port the first two read t.
;; * the shape of a byte-compiled lambda.  The reference's `(byte-compile '(lambda (x) x))` has a
;;   *list* arglist and a one-element constant vector (`len=4 arglist=(x) consts=[x] depth=1`);
;;   the port's has an integer arglist and an *empty* constant vector
;;   (`len=5 arglist=257 consts=[] depth=2`).  The two conventions are the dynamic and the lexical
;;   one, and the port switches to the reference's shape when `lexical-binding` is forced nil.
;; * `make-closure` on a prototype.  The reference accepts
;;   `(make-closure (byte-compile '(lambda (x) x)) 42)`; the port errors with
;;   `("Closure vars do not fit in constvec")` because its prototype's constant vector is empty.
(princ (format "state: default=%S toplevel=%S buffer-local=%S local-p=%S\n"
               (default-value 'lexical-binding)
               (default-toplevel-value 'lexical-binding)
               (buffer-local-value 'lexical-binding (current-buffer))
               (local-variable-p 'lexical-binding)))
(let ((f (byte-compile '(lambda (x) x))))
  (princ (format "shape: len=%S arglist=%S consts=%S depth=%S\n"
                 (length f) (aref f 0) (aref f 2) (aref f 3))))
(let ((f (byte-compile '(lambda (x) x))))
  (princ (format "closure: %S\n"
                 (condition-case e (funcall (make-closure f 42)) (error (format "ERR:%S" e))))))
(let ((saved lexical-binding))
  (setq lexical-binding nil)
  (let ((f (byte-compile '(lambda (x) x))))
    (princ (format "shape-with-nil: arglist=%S consts=%S\n" (aref f 0) (aref f 2))))
  (setq lexical-binding saved))
