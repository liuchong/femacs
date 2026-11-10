;;; runtime-call-frames-functional.el --- Bytecode indirect-call backtrace frame checks -*- lexical-binding: t; -*-

(require 'cl-lib)
(require 'bytecomp)

(defconst rfc--defined-symbols
  '(rfc--require
    rfc--make-wrapper-2
    rfc--make-apply-wrapper-3
    rfc--frame-first-arg
    rfc--assert-call-frame
    rfc--assert-no-residual-frames
    rfc--run-suite
    rfc--defined-symbols))

(defun rfc--require (condition fmt &rest args)
  "Signal an uncaught error when CONDITION is nil."
  (unless condition
    (error "%s" (apply #'format fmt args))))

(defun rfc--make-wrapper-2 (op-symbol)
  "Build a 2-arg bytecode closure (f arg) that invokes OP-SYMBOL via Bcall2.
Opcodes from src/bytecode.rs:
  192 = Bconstant 0 (push OP-SYMBOL)
    2 = Bstack_ref2 (push f from frame_base[0])
    2 = Bstack_ref2 (push arg from frame_base[1])
   34 = Bcall2      (call OP-SYMBOL with 2 args)
  135 = Breturn     (return top of stack)
Arity descriptor 514 = (2 << 8) | 2."
  (make-byte-code 514 (unibyte-string 192 2 2 34 135) (vector op-symbol) 8))

(defun rfc--make-apply-wrapper-3 ()
  "Build a 3-arg bytecode closure (f prefix spread) that invokes `apply' via Bcall3.
Opcodes from src/bytecode.rs:
  192 = Bconstant 0 (push `apply')
    3 = Bstack_ref3 (push f from frame_base[0])
    3 = Bstack_ref3 (push prefix from frame_base[1])
    3 = Bstack_ref3 (push spread from frame_base[2])
   35 = Bcall3      (call `apply' with 3 args)
  135 = Breturn     (return top of stack)
Arity descriptor 771 = (3 << 8) | 3."
  (make-byte-code 771 (unibyte-string 192 3 3 3 35 135) (vector 'apply) 12))

(defun rfc--frame-first-arg (frame)
  "Return the first call argument from a backtrace FRAME.
In `backtrace-frames' (via `mapbacktrace'), each evaluated frame has the
shape (t FUNC ARGS FLAGS), so (nth 2 FRAME) is the ARGS list and its car
is the first argument.  In singular `backtrace-frame', ARGS is spliced so
(nth 2 FRAME) is directly the first argument."
  (let ((third (nth 2 frame)))
    (if (consp third)
        (car third)
      third)))

(defun rfc--assert-call-frame (frames target expected-op expected-wrapper case-id)
  "Locate TARGET in FRAMES and assert its immediate outer frame is EXPECTED-OP."
  (rfc--require (consp frames) "%s: backtrace-frames did not return a non-empty list" case-id)
  (let ((tail frames)
        (found-tail nil))
    (while (and tail (not found-tail))
      (let ((frame (car tail)))
        (when (and (consp frame)
                   (eq (car frame) t)
                   (eq (cadr frame) target))
          (setq found-tail tail)))
      (setq tail (cdr tail)))
    (rfc--require found-tail "%s: target frame missing from backtrace-frames" case-id)
    (let ((outer (cadr found-tail))
          (caller (nth 2 found-tail)))
      (rfc--require (consp outer)
                    "%s: missing outer frame after target frame" case-id)
      (rfc--require (eq (car outer) t)
                    "%s: outer frame is not marked evaluated: %S" case-id outer)
      (rfc--require (eq (cadr outer) expected-op)
                    "%s: expected outer frame %S, got %S"
                    case-id expected-op (cadr outer))
      (rfc--require (eq (rfc--frame-first-arg outer) target)
                    "%s: outer %S frame first argument is not eq to target: %S"
                    case-id expected-op outer)
      (rfc--require (consp caller)
                    "%s: missing wrapper caller frame outside %S" case-id expected-op)
      (rfc--require (eq (car caller) t)
                    "%s: wrapper caller frame is not marked evaluated: %S"
                    case-id caller)
      (rfc--require (eq (cadr caller) expected-wrapper)
                    "%s: expected caller frame eq to wrapper, got %S"
                    case-id (cadr caller)))))

(defun rfc--assert-no-residual-frames (frames forbidden-callables case-id)
  "Verify that none of FORBIDDEN-CALLABLES remain active in FRAMES."
  (dolist (frame frames)
    (when (consp frame)
      (let ((fn (cadr frame)))
        (dolist (forbidden forbidden-callables)
          (rfc--require (not (eq fn forbidden))
                        "%s: residual frame for %S remained on backtrace after unwind"
                        case-id forbidden))))))

(defun rfc--run-suite ()
  "Execute all P458 indirect-call backtrace frame checks and print pass marker."
  (let* ((byte-compile-warnings nil)
         (funcall-wrapper (rfc--make-wrapper-2 'funcall))
         (apply2-wrapper (rfc--make-wrapper-2 'apply))
         (apply3-wrapper (rfc--make-apply-wrapper-3))
         (target (byte-compile
                  '(lambda (&rest values)
                     (garbage-collect)
                     (list values (backtrace-frames)))))
         (error-target (byte-compile
                        '(lambda (tag payload)
                           (garbage-collect)
                           (signal 'wrong-type-argument (list tag payload)))))
         (throw-target (byte-compile
                        '(lambda (tag payload)
                           (garbage-collect)
                           (throw tag (list payload (backtrace-frames))))))
         (all-callables (list funcall-wrapper apply2-wrapper apply3-wrapper
                              target error-target throw-target))
         (passed nil))
    (dolist (fn all-callables)
      (rfc--require (byte-code-function-p fn)
                    "rfc00-bytecode-check: expected byte-code-function-p, got %S" fn))

    ;; 1. funcall single value through bytecode Bcall2 wrapper.
    (let* ((arg (list 'funcall-item 42))
           (res (funcall funcall-wrapper target arg))
           (values (car res))
           (frames (cadr res)))
      (rfc--require (equal values (list arg))
                    "rfc01-funcall-single-arg: unexpected values %S" values)
      (rfc--assert-call-frame frames target 'funcall funcall-wrapper
                              'rfc01-funcall-single-arg)
      (push 'rfc01-funcall-single-arg passed))

    ;; 2. 2-arg apply with nil spread list.
    (let* ((res (funcall apply2-wrapper target nil))
           (values (car res))
           (frames (cadr res)))
      (rfc--require (null values)
                    "rfc02-apply-nil-spread: expected nil values, got %S" values)
      (rfc--assert-call-frame frames target 'apply apply2-wrapper
                              'rfc02-apply-nil-spread)
      (push 'rfc02-apply-nil-spread passed))

    ;; 3. 2-arg apply with single-element spread list (Fapply mutates spread slot in place).
    (let* ((res (funcall apply2-wrapper target (list (cons 'single 7))))
           (values (car res))
           (frames (cadr res)))
      (rfc--require (equal values '((single . 7)))
                    "rfc03-apply-single-spread: unexpected values %S" values)
      (rfc--assert-call-frame frames target 'apply apply2-wrapper
                              'rfc03-apply-single-spread)
      (push 'rfc03-apply-single-spread passed))

    ;; 4. 2-arg apply with multi-element spread list.
    (let* ((res (funcall apply2-wrapper target
                         (list 'first (cons 'second 2) "third")))
           (values (car res))
           (frames (cadr res)))
      (rfc--require (equal values '(first (second . 2) "third"))
                    "rfc04-apply-multi-spread: unexpected values %S" values)
      (rfc--assert-call-frame frames target 'apply apply2-wrapper
                              'rfc04-apply-multi-spread)
      (push 'rfc04-apply-multi-spread passed))

    ;; 5. 3-arg apply with fixed prefix and multi-element spread list.
    (let* ((res (funcall apply3-wrapper target
                         (cons 'fixed 0)
                         (list 'tail-1 (cons 'tail-2 9))))
           (values (car res))
           (frames (cadr res)))
      (rfc--require (equal values '((fixed . 0) tail-1 (tail-2 . 9)))
                    "rfc05-apply3-prefix-and-spread: unexpected values %S" values)
      (rfc--assert-call-frame frames target 'apply apply3-wrapper
                              'rfc05-apply3-prefix-and-spread)
      (push 'rfc05-apply3-prefix-and-spread passed))

    ;; 6. Compiled error signal unwinds cleanly and subsequent call recovers.
    (let ((captured-err :did-not-signal))
      (setq captured-err
            (condition-case err
                (progn
                  (funcall apply3-wrapper error-target
                           'integerp (list (cons 'bad-value 19)))
                  :returned-normally)
              (wrong-type-argument err)))
      (rfc--require (equal captured-err
                           '(wrong-type-argument integerp (bad-value . 19)))
                    "rfc06-compiled-error-unwind-and-recovery: unexpected error %S"
                    captured-err)
      (rfc--assert-no-residual-frames (backtrace-frames) all-callables
                                      'rfc06-compiled-error-unwind-and-recovery)
      (let* ((rec (funcall apply2-wrapper target (list 'recovered-after-error)))
             (rec-values (car rec))
             (rec-frames (cadr rec)))
        (rfc--require (equal rec-values '(recovered-after-error))
                      "rfc06-compiled-error-unwind-and-recovery: recovery values %S"
                      rec-values)
        (rfc--assert-call-frame rec-frames target 'apply apply2-wrapper
                                'rfc06-compiled-error-unwind-and-recovery))
      (push 'rfc06-compiled-error-unwind-and-recovery passed))

    ;; 7. Compiled non-local throw captures apply frame, unwinds cleanly, and recovers.
    (let ((thrown (catch 'rfc-catch-tag
                    (funcall apply3-wrapper throw-target
                             'rfc-catch-tag (list (cons 'thrown-payload 23)))
                    :did-not-throw)))
      (rfc--require (and (consp thrown)
                         (equal (car thrown) '(thrown-payload . 23)))
                    "rfc07-compiled-throw-unwind-and-recovery: unexpected thrown value %S"
                    thrown)
      (rfc--assert-call-frame (cadr thrown) throw-target 'apply apply3-wrapper
                              'rfc07-compiled-throw-unwind-and-recovery)
      (rfc--assert-no-residual-frames (backtrace-frames) all-callables
                                      'rfc07-compiled-throw-unwind-and-recovery)
      (let* ((rec (funcall funcall-wrapper target 'recovered-after-throw))
             (rec-values (car rec))
             (rec-frames (cadr rec)))
        (rfc--require (equal rec-values '(recovered-after-throw))
                      "rfc07-compiled-throw-unwind-and-recovery: recovery values %S"
                      rec-values)
        (rfc--assert-call-frame rec-frames target 'funcall funcall-wrapper
                                'rfc07-compiled-throw-unwind-and-recovery))
      (push 'rfc07-compiled-throw-unwind-and-recovery passed))

    (let ((result (list 'P458-CALL-FRAMES (nreverse passed))))
      (prin1 result)
      (terpri)
      result)))

(unwind-protect
    (rfc--run-suite)
  (let ((syms rfc--defined-symbols))
    (dolist (sym syms)
      (when (fboundp sym)
        (fmakunbound sym))
      (when (boundp sym)
        (makunbound sym)))))

;;; runtime-call-frames-functional.el ends here
