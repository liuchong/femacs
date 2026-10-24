;;; bytecode-runtime.el --- Stage III VM shared-tail semantic & observation suite -*- lexical-binding: t; -*-

;; Independent partner-owned ERT fixture for Stage III VM dispatch modernization
;; (T-VM1 through T-VM6 in docs/runtime-modernization.md).
;;
;; Loaded in batch mode via `tools/check-ert.py` / `tools/test-bytecode-runtime.py`
;; with `--eval (setq femacs-bytecode-runtime-observation-file "build/runtime/obs.json")`.

(require 'ert)
(require 'cl-lib)
(require 'bytecomp)
(require 'byte-opt)
(require 'disass)

(defvar femacs-bytecode-runtime-observation-file nil
  "Target JSON file path where deterministic per-test observations are written.")

(defvar femacs-bytecode-runtime--observations nil
  "Alist of (TEST-NAME . OBSERVATION-PLIST) accumulated during the batch run.")

(defvar femacs-bytecode-runtime--diagnostics nil
  "Alist of (TEST-NAME . DIAGNOSTIC-PLIST) for non-deterministic metrics like GC counts.")

(defun femacs-bc--flush-observations ()
  "Persist accumulated observations and diagnostics as UTF-8 JSON when configured."
  (when (and (stringp femacs-bytecode-runtime-observation-file)
             (> (length femacs-bytecode-runtime-observation-file) 0))
    (let* ((obs-hash (make-hash-table :test 'equal))
           (diag-hash (make-hash-table :test 'equal))
           (root (make-hash-table :test 'equal)))
      (dolist (entry (reverse femacs-bytecode-runtime--observations))
        (puthash (car entry) (cdr entry) obs-hash))
      (dolist (entry (reverse femacs-bytecode-runtime--diagnostics))
        (puthash (car entry) (cdr entry) diag-hash))
      (puthash "schema" "femacs-bytecode-runtime-observations-v1" root)
      (puthash "test_count" (hash-table-count obs-hash) root)
      (puthash "observations" obs-hash root)
      (puthash "diagnostics" diag-hash root)
      (let ((coding-system-for-write 'utf-8-unix))
        (with-temp-file femacs-bytecode-runtime-observation-file
          (insert (json-serialize root)))))))

(defun femacs-bc--plist-to-hash (plist)
  "Convert a flat PLIST with string/symbol keys into an `equal' hash table for `json-serialize'."
  (let ((ht (make-hash-table :test 'equal)))
    (while plist
      (let ((k (car plist))
            (v (cadr plist)))
        (puthash (if (symbolp k) (substring (symbol-name k) 1) (format "%s" k))
                 v ht)
        (setq plist (cddr plist))))
    ht))

(defun femacs-bc--record (test-name deterministic-plist &optional diagnostic-plist)
  "Record DETERMINISTIC-PLIST (and optional DIAGNOSTIC-PLIST) for TEST-NAME and flush."
  (let ((key (if (symbolp test-name) (symbol-name test-name) test-name)))
    (setq femacs-bytecode-runtime--observations
          (cons (cons key (femacs-bc--plist-to-hash deterministic-plist))
                (assoc-delete-all key femacs-bytecode-runtime--observations)))
    (when diagnostic-plist
      (setq femacs-bytecode-runtime--diagnostics
            (cons (cons key (femacs-bc--plist-to-hash diagnostic-plist))
                  (assoc-delete-all key femacs-bytecode-runtime--diagnostics))))
    (femacs-bc--flush-observations)))

(defun femacs-bc--decode-instructions (fn)
  "Decode bytecode closure FN's unibyte instruction stream into a list of (OPCODE . OPERAND)."
  (should (byte-code-function-p fn))
  (let* ((bytestr (aref fn 1))
         (len (length bytestr))
         (pc 0)
         (insns nil))
    (should (stringp bytestr))
    (should (not (multibyte-string-p bytestr)))
    (while (< pc len)
      (let ((op (aref bytestr pc)))
        (cond
         ;; 3-byte instructions: 2-byte little-endian operand
         ((memq op '(15 23 31 39 47 49 50 129 130 131 132 133 134 179))
          (should (< (+ pc 2) len))
          (let ((arg (logior (aref bytestr (1+ pc))
                             (ash (aref bytestr (+ pc 2)) 8))))
            (push (cons op arg) insns)
            (setq pc (+ pc 3))))
         ;; 2-byte instructions: 1-byte operand
         ((memq op '(6 14 22 30 38 46 175 176 177 178 182))
          (should (< (1+ pc) len))
          (let ((arg (aref bytestr (1+ pc))))
            (push (cons op arg) insns)
            (setq pc (+ pc 2))))
         ;; 1-byte instructions: immediate or no operand
         (t
          (push (cons op nil) insns)
          (setq pc (1+ pc))))))
    (nreverse insns)))

(defun femacs-bc--opcode-set (fn)
  "Return sorted unique list of numeric opcodes present in compiled closure FN."
  (let ((ops (mapcar #'car (femacs-bc--decode-instructions fn))))
    (sort (delete-dups ops) #'<)))

(defun femacs-bc--has-opcode-range-p (ops lo hi)
  "Return t if any opcode in OPS lies in inclusive range [LO, HI]."
  (cl-some (lambda (op) (and (>= op lo) (<= op hi))) ops))

(defun femacs-bc--disassemble-string (fn)
  "Return the disassembly string of compiled function FN."
  (with-temp-buffer
    (disassemble fn (current-buffer))
    (buffer-string)))

(defun femacs-bc--compile-form (lambda-form)
  "Return (INTERP-FN . COMP-FN) for LAMBDA-FORM with lexical-binding enabled."
  (let* ((interp-fn (eval lambda-form t))
         (byte-compile-warnings nil)
         (comp-fn (byte-compile lambda-form)))
    (should (functionp interp-fn))
    (should (not (byte-code-function-p interp-fn)))
    (should (byte-code-function-p comp-fn))
    (cons interp-fn comp-fn)))

;;; ----------------------------------------------------------------------------
;;; T-VM1: varref/varset/varbind/unbind operand widths (short 0..5, 1-byte 6..255,
;;;        2-byte >=256) and constant indices >255 with disassembly/opcode proof.
;;; ----------------------------------------------------------------------------

(ert-deftest tvm1-varref-varset-varbind-short-and-1byte ()
  "Verify short (0..5) and 1-byte (6..255) varref, varset, varbind, and unbind opcodes."
  (let* ((vars (cl-loop for i from 0 below 14
                        collect (intern (format "femacs-tvm1-short-var-%d" i)))))
    (dolist (v vars)
      (eval `(defvar ,v -1) t)
      (set v -1))
    (let* ((let-bindings (cl-loop for v in vars
                                  for i from 0
                                  collect (list v (+ 100 i))))
           (mutations (cl-loop for v in vars
                               for i from 0
                               collect `(setq ,v (+ ,v ,(* (1+ i) 3)))))
           (reads (cons 'list vars))
           (form `(lambda (delta)
                    (let ,let-bindings
                      ,@mutations
                      (setq ,(car vars) (+ ,(car vars) delta))
                      (setq ,(nth 10 vars) (+ ,(nth 10 vars) delta))
                      ,reads)))
           (pair (femacs-bc--compile-form form))
           (interp-fn (car pair))
           (comp-fn (cdr pair))
           (ops (femacs-bc--opcode-set comp-fn))
           (disasm (femacs-bc--disassemble-string comp-fn))
           (interp-res (funcall interp-fn 7))
           (comp-res (funcall comp-fn 7))
           (post-bind-values (mapcar #'symbol-value vars)))
      ;; Interpreted and byte-compiled results must match and restore outer bindings
      (should (equal interp-res comp-res))
      (should (equal post-bind-values (make-list 14 -1)))
      ;; Prove short (0..5) and 1-byte (6..255) opcodes were actually emitted
      (should (femacs-bc--has-opcode-range-p ops 8 13))   ; Bvarref..Bvarref5
      (should (memq 14 ops))                              ; Bvarref6 (1-byte)
      (should (femacs-bc--has-opcode-range-p ops 16 21))  ; Bvarset..Bvarset5
      (should (memq 22 ops))                              ; Bvarset6 (1-byte)
      (should (femacs-bc--has-opcode-range-p ops 24 29))  ; Bvarbind..Bvarbind5
      (should (memq 30 ops))                              ; Bvarbind6 (1-byte)
      (should (memq 46 ops))                              ; Bunbind6 (1-byte, 14 vars)
      (should (string-match-p "varref" disasm))
      (should (string-match-p "varset" disasm))
      (should (string-match-p "varbind" disasm))
      (should (string-match-p "unbind" disasm))
      (femacs-bc--record
       'tvm1-varref-varset-varbind-short-and-1byte
       (list :result (vconcat comp-res)
             :restored_all_minus_one (equal post-bind-values (make-list 14 -1))
             :has_varref_short (if (femacs-bc--has-opcode-range-p ops 8 13) t :false)
             :has_varref6 (if (memq 14 ops) t :false)
             :has_varset_short (if (femacs-bc--has-opcode-range-p ops 16 21) t :false)
             :has_varset6 (if (memq 22 ops) t :false)
             :has_varbind_short (if (femacs-bc--has-opcode-range-p ops 24 29) t :false)
             :has_varbind6 (if (memq 30 ops) t :false)
             :has_unbind6 (if (memq 46 ops) t :false))))))

(ert-deftest tvm1-varref7-varset7-varbind7-unbind7-and-constant2-wide ()
  "Verify 2-byte (>=256) varref7, varset7, varbind7, unbind7, and Bconstant2 (>255 constants)."
  (let* ((nvars 264)
         (vars (cl-loop for i from 0 below nvars
                        collect (intern (format "femacs-tvm1-wide-var-%03d" i))))
         (consts (cl-loop for i from 0 below 264
                          collect (format "wide-const-payload-%03d" i))))
    (dolist (v vars)
      (eval `(defvar ,v -99) t)
      (set v -99))
    (let* ((let-bindings (cl-loop for v in vars
                                  for i from 0
                                  collect (list v i)))
           (set-forms (cl-loop for v in vars
                               for c in consts
                               for i from 0
                               collect `(setq ,v (+ ,v (funcall keep-fn ,c) ,(if (= (mod i 2) 0) 1 2)))))
           (sum-form (cons '+ vars))
           (form `(lambda (keep-fn)
                    (let ,let-bindings
                      ,@set-forms
                      ,sum-form)))
           (pair (femacs-bc--compile-form form))
           (interp-fn (car pair))
           (comp-fn (cdr pair))
           (insns (femacs-bc--decode-instructions comp-fn))
           (ops (femacs-bc--opcode-set comp-fn))
           (const-vec (aref comp-fn 2))
           (disasm (femacs-bc--disassemble-string comp-fn))
           (interp-res (funcall interp-fn #'length))
           (comp-res (funcall comp-fn #'length))
           (restored (cl-every (lambda (v) (eql (symbol-value v) -99)) vars))
           (max-varref7-arg (cl-loop for (op . arg) in insns
                                     when (= op 15) maximize arg))
           (max-varset7-arg (cl-loop for (op . arg) in insns
                                     when (= op 23) maximize arg))
           (max-varbind7-arg (cl-loop for (op . arg) in insns
                                      when (= op 31) maximize arg))
           (max-unbind7-arg (cl-loop for (op . arg) in insns
                                     when (= op 47) maximize arg))
           (max-const2-arg (cl-loop for (op . arg) in insns
                                    when (= op 129) maximize arg)))
      (should (equal interp-res comp-res))
      (should restored)
      (should (> (length const-vec) 500))
      ;; Prove 2-byte opcodes (15=Bvarref7, 23=Bvarset7, 31=Bvarbind7, 47=Bunbind7, 129=Bconstant2)
      ;; were emitted with 16-bit operands >= 256.
      (should (memq 15 ops))
      (should (memq 23 ops))
      (should (memq 31 ops))
      (should (memq 47 ops))
      (should (memq 129 ops))
      (should (>= max-varref7-arg 256))
      (should (>= max-varset7-arg 256))
      (should (>= max-varbind7-arg 256))
      (should (= max-unbind7-arg nvars))
      (should (>= max-const2-arg 256))
      (should (string-match-p "varbind" disasm))
      (should (string-match-p "unbind" disasm))
      (femacs-bc--record
       'tvm1-varref7-varset7-varbind7-unbind7-and-constant2-wide
       (list :sum comp-res
             :restored (if restored t :false)
             :const_vec_len (length const-vec)
             :has_varref7 (if (memq 15 ops) t :false)
             :has_varset7 (if (memq 23 ops) t :false)
             :has_varbind7 (if (memq 31 ops) t :false)
             :has_unbind7 (if (memq 47 ops) t :false)
             :has_constant2 (if (memq 129 ops) t :false)
             :max_varref7_arg max-varref7-arg
             :max_varset7_arg max-varset7-arg
             :max_varbind7_arg max-varbind7-arg
             :max_unbind7_arg max-unbind7-arg
             :max_const2_arg max-const2-arg)))))

;;; ----------------------------------------------------------------------------
;;; T-VM2: subr/interpreted/lexbound compiled calls, optional/rest/wide arity
;;;        (Bcall..Bcall5, Bcall6, Bcall7), nested returns, wrong-number-of-arguments,
;;;        and evaluation depth / backtrace restoration.
;;; ----------------------------------------------------------------------------

(ert-deftest tvm2-calls-subr-interpreted-lexbound-and-wide-arity ()
  "Verify DOCALL across subrs, interpreted closures, lexbound bytecode closures, and Bcall/Bcall6/Bcall7."
  (let* ((order-log nil)
         (record-arg (lambda (tag val)
                       (push (cons tag val) order-log)
                       val))
         ;; Interpreted helper (not byte-compiled)
         (interp-callee (eval '(lambda (a &optional b &rest more)
                                 (list 'interp a (or b :default-b) more))
                              t))
         ;; Lexbound byte-compiled helper with optional & rest arguments
         ;; Exercises DOCALL -> CLOSUREP + FIXNUMP(template) -> continue 'setup_frame
         ;; and Breturn with saved_top != NULL.
         (lex-leaf (byte-compile
                    '(lambda (x &optional y &rest z)
                       (list 'lex-leaf x (or y 10) z))))
         (lex-mid (byte-compile
                   `(lambda (fn a b c)
                      (let ((r1 (funcall fn a))
                            (r2 (funcall fn a b))
                            (r3 (funcall fn a b c (+ a b c))))
                        (list r1 r2 r3)))))
         ;; Wide call form: 8-arg call (Bcall6 = 38) and 260-arg call (Bcall7 = 39)
         (wide-args-260 (cl-loop for i from 1 to 260 collect i))
         (wide-caller-form
          `(lambda (f-var)
             (list
              ;; Bcall (0 args) through Bcall5 (5 args) via funcall
              (funcall f-var)
              (funcall f-var 10)
              (funcall f-var 10 20)
              (funcall f-var 10 20 30)
              (funcall f-var 10 20 30 40)
              ;; Bcall6 (8 args: f-var + 8 arguments -> operand 8)
              (funcall f-var 1 2 3 4 5 6 7 8)
              ;; Bcall7 (260 args: f-var + 260 arguments -> operand 260)
              (funcall f-var ,@wide-args-260))))
         (wide-pair (femacs-bc--compile-form wide-caller-form))
         (wide-interp (car wide-pair))
         (wide-comp (cdr wide-pair))
         (wide-insns (femacs-bc--decode-instructions wide-comp))
         (wide-ops (femacs-bc--opcode-set wide-comp))
         (vararg-collector (byte-compile '(lambda (&rest xs) (cons (length xs) (apply #'+ xs)))))
         ;; Order-of-evaluation caller
         (order-caller (byte-compile
                        '(lambda (rec callee)
                           (funcall callee
                                    (funcall rec 'first 1)
                                    (funcall rec 'second 2)
                                    (funcall rec 'third 3)
                                    (funcall rec 'fourth 4))))))
    (should (not (byte-code-function-p interp-callee)))
    (should (byte-code-function-p lex-leaf))
    (should (fixnump (aref lex-leaf 0)))
    (let* ((nested-res (funcall lex-mid lex-leaf 3 4 5))
           (interp-call-res (funcall lex-mid interp-callee 3 4 5))
           (wide-interp-res (funcall wide-interp vararg-collector))
           (wide-comp-res (funcall wide-comp vararg-collector))
           (order-res (progn
                        (setq order-log nil)
                        (funcall order-caller record-arg lex-leaf)))
           (recorded-order (mapcar #'car (nreverse order-log)))
           (call6-operands (cl-loop for (op . arg) in wide-insns
                                    when (= op 38) collect arg))
           (call7-operands (cl-loop for (op . arg) in wide-insns
                                    when (= op 39) collect arg)))
      (should (equal wide-interp-res wide-comp-res))
      (should (equal recorded-order '(first second third fourth)))
      ;; Prove Bcall..Bcall5 (32..37), Bcall6 (38), and Bcall7 (39, arg=260) were emitted
      (should (femacs-bc--has-opcode-range-p wide-ops 32 37))
      (should (memq 38 wide-ops))
      (should (memq 39 wide-ops))
      (should (equal call6-operands '(8)))
      (should (equal call7-operands '(260)))
      (femacs-bc--record
       'tvm2-calls-subr-interpreted-lexbound-and-wide-arity
       (list :nested_lex (format "%S" nested-res)
             :nested_interp (format "%S" interp-call-res)
             :wide_results (format "%S" wide-comp-res)
             :order_res (format "%S" order-res)
             :eval_order (vconcat (mapcar #'symbol-name recorded-order))
             :call6_operands (vconcat call6-operands)
             :call7_operands (vconcat call7-operands))))))

(ert-deftest tvm2-wrong-arity-and-eval-depth-restoration ()
  "Verify wrong-number-of-arguments and max-lisp-eval-depth recovery across bytecode frames."
  (let* ((strict-fn (byte-compile '(lambda (a b &optional c) (list a b c))))
         (recurse-self
          (byte-compile
           '(lambda (self n)
              (if (<= n 0)
                  'bottom
                (1+ (eval (list 'progn
                                '(identity (identity (identity nil)))
                                (list 'funcall (list 'quote self) (list 'quote self) (1- n)))
                          t))))))
         (recurse-docall
          (byte-compile
           '(lambda (self n)
              (if (<= n 0)
                  'bottom
                (1+ (funcall self self (1- n)))))))
         (caller (byte-compile
                  '(lambda (f)
                     (list
                      (condition-case err
                          (funcall f 1)
                        (wrong-number-of-arguments (list 'too-few (car err) (length (cdr err)))))
                      (condition-case err
                          (funcall f 1 2 3 4)
                        (wrong-number-of-arguments (list 'too-many (car err) (length (cdr err)))))
                      (funcall f 10 20 30)))))
         (arity-res (funcall caller strict-fn))
         (depth-err (let ((max-lisp-eval-depth 120))
                      (condition-case err
                          (funcall recurse-self recurse-self 400)
                        (error (list 'caught-depth-error (car err))))))
         (docall-depth-err (let ((max-lisp-eval-depth 120))
                             (condition-case err
                                 (funcall recurse-docall recurse-docall 400)
                               (error (list 'caught-docall-depth-error (car err) (cadr err))))))
         ;; After deep recursion overflow, normal bytecode execution must succeed cleanly
         (post-recovery (funcall strict-fn 'ok-a 'ok-b 'ok-c)))
    (should (equal (nth 0 arity-res) '(too-few wrong-number-of-arguments 2)))
    (should (equal (nth 1 arity-res) '(too-many wrong-number-of-arguments 2)))
    (should (equal (nth 2 arity-res) '(10 20 30)))
    (should (equal depth-err '(caught-depth-error excessive-lisp-nesting)))
    (should (equal docall-depth-err
                   '(caught-docall-depth-error error "Lisp nesting exceeds `max-lisp-eval-depth'")))
    (should (equal post-recovery '(ok-a ok-b ok-c)))
    (femacs-bc--record
     'tvm2-wrong-arity-and-eval-depth-restoration
     (list :arity_res (format "%S" arity-res)
           :depth_err (format "%S" depth-err)
           :docall_depth_err (format "%S" docall-depth-err)
           :post_recovery (format "%S" post-recovery)))))

;;; ----------------------------------------------------------------------------
;;; T-VM3: conditional/unconditional forward/backward branches, nil/non-nil
;;;        else-pop stack behavior, and Bswitch (small linear <=5 and hash >5).
;;; ----------------------------------------------------------------------------

(ert-deftest tvm3-branches-forward-backward-and-else-pop ()
  "Verify Bgoto, Bgotoifnil, Bgotoifnonnil, Bgotoifnilelsepop, Bgotoifnonnilelsepop, and backward loops."
  (let* ((and-form '(lambda (step-fn a b c)
                      (and (funcall step-fn 'a a)
                           (funcall step-fn 'b b)
                           (funcall step-fn 'c c))))
         (or-form '(lambda (step-fn a b c)
                     (or (funcall step-fn 'a a)
                         (funcall step-fn 'b b)
                         (funcall step-fn 'c c))))
         (loop-form '(lambda (step-fn limit)
                       (let ((i 0)
                             (acc nil))
                         (while (< i limit)
                           ;; For-effect `or' emits Bgotoifnonnil (132)
                           (or (= (mod i 2) 0)
                               (funcall step-fn 'odd i))
                           (if (= i 2)
                               (push (cons 'two (funcall step-fn 'two i)) acc)
                             (push (cons 'other i) acc))
                           (setq i (1+ i)))
                         (nreverse acc))))
         (and-pair (femacs-bc--compile-form and-form))
         (or-pair (femacs-bc--compile-form or-form))
         (loop-pair (femacs-bc--compile-form loop-form))
         (and-ops (femacs-bc--opcode-set (cdr and-pair)))
         (or-ops (femacs-bc--opcode-set (cdr or-pair)))
         (loop-ops (femacs-bc--opcode-set (cdr loop-pair)))
         (run-suite
          (lambda (and-fn or-fn loop-fn)
            (let* ((trace nil)
                   (step (lambda (tag val)
                           (push (cons tag val) trace)
                           val))
                   (results
                    (list
                     (funcall and-fn step 'x1 'x2 'x3)
                     (funcall and-fn step 'x1 nil 'x3)
                     (funcall or-fn step nil nil 'z3)
                     (funcall or-fn step nil 'y2 'z3)
                     (funcall loop-fn step 5))))
              (cons results (nreverse trace)))))
         (out-interp (funcall run-suite (car and-pair) (car or-pair) (car loop-pair)))
         (out-comp (funcall run-suite (cdr and-pair) (cdr or-pair) (cdr loop-pair)))
         (res-interp (car out-interp))
         (res-comp (car out-comp))
         (trace-interp (cdr out-interp))
         (trace-comp (cdr out-comp)))
    (should (equal res-interp res-comp))
    (should (equal trace-interp trace-comp))
    ;; Prove all 5 branch opcodes (130..134) are emitted across the compiled forms:
    ;; 133 = Bgotoifnilelsepop in `and-form', 134 = Bgotoifnonnilelsepop in `or-form',
    ;; 130 = Bgoto, 131 = Bgotoifnil, 132 = Bgotoifnonnil in `loop-form'.
    (should (memq 133 and-ops))
    (should (memq 134 or-ops))
    (should (memq 130 loop-ops))
    (should (memq 131 loop-ops))
    (should (memq 132 loop-ops))
    (femacs-bc--record
     'tvm3-branches-forward-backward-and-else-pop
     (list :results (format "%S" res-comp)
           :side_effects (format "%S" trace-comp)
           :and_has_133 (if (memq 133 and-ops) t :false)
           :or_has_134 (if (memq 134 or-ops) t :false)
           :loop_has_130_131_132 (if (and (memq 130 loop-ops)
                                          (memq 131 loop-ops)
                                          (memq 132 loop-ops))
                                     t :false)))))

(ert-deftest tvm3-switch-linear-and-hash-jump-tables ()
  "Verify Bswitch (opcode 183) on both linear search (count <= 5) and hash lookup (count > 5)."
  (let* ((byte-compile-cond-use-jump-table t)
         ;; 4 cases -> hash-table-count = 4 (<= 5) with :test 'eq -> linear search loop in Bswitch
         ;; (src/bytecode.rs:1817-1832, hitting `GOTO!(OP_BRANCH); break;').
         (linear-form
          '(lambda (x side-effect-fn)
             (let ((tag (funcall side-effect-fn x)))
               (cond
                ((eq tag 'alpha) '(linear-hit 1))
                ((eq tag 'beta)  '(linear-hit 2))
                ((eq tag 'gamma) '(linear-hit 3))
                ((eq tag 'delta) '(linear-hit 4))
                (t (list 'linear-miss tag))))))
         ;; 8 cases -> hash-table-count = 8 (> 5) -> hash_find path in Bswitch
         ;; (src/bytecode.rs:1833-1842, hitting `GOTO!(OP_BRANCH)').
         (hash-form
          '(lambda (x side-effect-fn)
             (let ((tag (funcall side-effect-fn x)))
               (cond
                ((eq tag 'k1) '(hash-hit 1))
                ((eq tag 'k2) '(hash-hit 2))
                ((eq tag 'k3) '(hash-hit 3))
                ((eq tag 'k4) '(hash-hit 4))
                ((eq tag 'k5) '(hash-hit 5))
                ((eq tag 'k6) '(hash-hit 6))
                ((eq tag 'k7) '(hash-hit 7))
                ((eq tag 'k8) '(hash-hit 8))
                (t (list 'hash-miss tag))))))
         (linear-pair (femacs-bc--compile-form linear-form))
         (hash-pair (femacs-bc--compile-form hash-form))
         (linear-comp (cdr linear-pair))
         (hash-comp (cdr hash-pair))
         (linear-ops (femacs-bc--opcode-set linear-comp))
         (hash-ops (femacs-bc--opcode-set hash-comp))
         (linear-ht (cl-find-if #'hash-table-p (append (aref linear-comp 2) nil)))
         (hash-ht (cl-find-if #'hash-table-p (append (aref hash-comp 2) nil)))
         (linear-inputs '(alpha beta gamma delta epsilon nil))
         (hash-inputs '(k1 k4 k8 k99 nil))
         (eval-both
          (lambda (lin-fn hsh-fn)
            (let* ((log nil)
                   (se (lambda (v)
                         (push v log)
                         v))
                   (results
                    (list
                     (mapcar (lambda (k) (funcall lin-fn k se)) linear-inputs)
                     (mapcar (lambda (k) (funcall hsh-fn k se)) hash-inputs))))
              (cons results (nreverse log)))))
         (out-interp (funcall eval-both (car linear-pair) (car hash-pair)))
         (out-comp (funcall eval-both linear-comp hash-comp))
         (res-interp (car out-interp))
         (res-comp (car out-comp))
         (se-interp (cdr out-interp))
         (se-comp (cdr out-comp)))
    (should (equal res-interp res-comp))
    (should (equal se-interp se-comp))
    ;; Prove Bswitch (183) and exact jump-table sizes (4 <= 5 and 8 > 5) were emitted
    (should (memq 183 linear-ops))
    (should (memq 183 hash-ops))
    (should (hash-table-p linear-ht))
    (should (= (hash-table-count linear-ht) 4))
    (should (eq (hash-table-test linear-ht) 'eq))
    (should (hash-table-p hash-ht))
    (should (= (hash-table-count hash-ht) 8))
    (femacs-bc--record
     'tvm3-switch-linear-and-hash-jump-tables
     (list :results (format "%S" res-comp)
           :side_effects (format "%S" se-comp)
           :linear_has_switch (if (memq 183 linear-ops) t :false)
           :hash_has_switch (if (memq 183 hash-ops) t :false)
           :linear_table_count (hash-table-count linear-ht)
           :hash_table_count (hash-table-count hash-ht)))))

;;; ----------------------------------------------------------------------------
;;; T-VM4: dynamic binding/unbinding, trapped symbol writes/watchers, aliases,
;;;        unbound symbols, and exceptional unwind with restored state.
;;; ----------------------------------------------------------------------------

(defvar femacs-tvm4-Watched 'initial-watched)
(defvar femacs-tvm4-Target 'initial-target)
(defvaralias 'femacs-tvm4-Alias 'femacs-tvm4-Target)
(defvar femacs-tvm4-Unbound nil)

(ert-deftest tvm4-dynamic-binding-watchers-aliases-and-unbound ()
  "Verify VARREF, VARSET, VARBIND, and DOUNBIND with watchers, varaliases, unbound vars, and unwind."
  (setq femacs-tvm4-Watched 'initial-watched)
  (setq femacs-tvm4-Target 'initial-target)
  (makunbound 'femacs-tvm4-Unbound)
  (let* ((watcher-events nil)
         (watcher-fn (lambda (sym newval op where)
                       (push (list sym newval op (bufferp where)) watcher-events)))
         (worker-form
          '(lambda (trigger-error-p)
             (let ((femacs-tvm4-Watched 'bound-outer)
                   (femacs-tvm4-Alias 'alias-bound))
               (setq femacs-tvm4-Watched 'mutated-outer)
               (setq femacs-tvm4-Alias 'alias-mutated)
               (let ((inner-res
                      (condition-case err
                          (let ((femacs-tvm4-Watched 'bound-inner))
                            (setq femacs-tvm4-Watched 'mutated-inner)
                            (when trigger-error-p
                              ;; Reading unbound special variable via VARREF signals void-variable
                              (setq femacs-tvm4-Watched femacs-tvm4-Unbound))
                            (list 'ok femacs-tvm4-Watched femacs-tvm4-Target))
                        (void-variable
                         (list 'caught-void (cadr err) femacs-tvm4-Watched femacs-tvm4-Target)))))
                 (list inner-res femacs-tvm4-Watched femacs-tvm4-Target femacs-tvm4-Alias)))))
         (pair (femacs-bc--compile-form worker-form))
         (interp-fn (car pair))
         (comp-fn (cdr pair)))
    (unwind-protect
        (progn
          (add-variable-watcher 'femacs-tvm4-Watched watcher-fn)
          (setq watcher-events nil)
          (let ((interp-ok (funcall interp-fn nil))
                (interp-err (funcall interp-fn t))
                (interp-events (nreverse watcher-events)))
            (setq watcher-events nil)
            (let ((comp-ok (funcall comp-fn nil))
                  (comp-err (funcall comp-fn t))
                  (comp-events (nreverse watcher-events)))
              (should (equal interp-ok comp-ok))
              (should (equal interp-err comp-err))
              (should (equal interp-events comp-events))
              (should (eq femacs-tvm4-Watched 'initial-watched))
              (should (eq femacs-tvm4-Target 'initial-target))
              (should (not (boundp 'femacs-tvm4-Unbound)))
              (femacs-bc--record
               'tvm4-dynamic-binding-watchers-aliases-and-unbound
               (list :comp_ok (format "%S" comp-ok)
                     :comp_err (format "%S" comp-err)
                     :watcher_events (format "%S" comp-events)
                     :restored_watched (symbol-name femacs-tvm4-Watched)
                     :restored_target (symbol-name femacs-tvm4-Target)
                     :unbound_still_unbound (if (not (boundp 'femacs-tvm4-Unbound)) t :false))))))
      (remove-variable-watcher 'femacs-tvm4-Watched watcher-fn))))

;;; ----------------------------------------------------------------------------
;;; T-VM5: nested catch/throw, condition-case, unwind-protect, normal/popped
;;;        handlers (Bpushcatch, Bpushconditioncase, Bpophandler), nonlocal returns
;;;        across frames, and GC while bytecode constants/locals/stack remain live.
;;; ----------------------------------------------------------------------------

(ert-deftest tvm5-handlers-nonlocal-exit-and-gc-live-roots ()
  "Verify PUSHHANDLER (sys_setjmp reload + OP_BRANCH), Bpophandler, unwind-protect, and GC root survival."
  (let* ((inner-thrower
          (byte-compile
           '(lambda (mode tag payload)
              (let ((local-guard (list 'inner-guard payload)))
                (unwind-protect
                    (cond
                     ((eq mode 'throw)
                      (throw tag (list 'thrown payload local-guard)))
                     ((eq mode 'signal)
                      (signal 'arith-error (list payload local-guard)))
                     (t
                      (list 'normal payload local-guard)))
                  (garbage-collect))))))
         (outer-form
          '(lambda (callee mode)
             (let (( cleanup-log nil)
                   (live-local (list (concat "live-" "prefix") (vector 10 20 30))))
               (let ((outcome
                      (catch 'outer-tag
                        (condition-case err
                            (unwind-protect
                                (let ((normal-catch
                                       ;; Normal handler push + pop (Bpophandler = 48)
                                       (catch 'unused-tag
                                         (funcall callee 'normal 'unused-tag 'step-1))))
                                  (push (cons 'after-normal (car normal-catch)) cleanup-log)
                                  ;; Second handler push that may take sys_setjmp != 0 path
                                  (let ((res (funcall callee mode 'outer-tag 'step-2)))
                                    (push 'normal-completed cleanup-log)
                                    (list 'completed res live-local)))
                              (push 'inner-unwind cleanup-log)
                              (garbage-collect))
                          (arith-error
                           (push 'caught-arith cleanup-log)
                           (garbage-collect)
                           (list 'recovered-signal (cdr err) live-local))))))
                 (list outcome (nreverse cleanup-log) live-local)))))
         (pair (femacs-bc--compile-form outer-form))
         (interp-fn (car pair))
         (comp-fn (cdr pair))
         (ops (femacs-bc--opcode-set comp-fn))
         (gc-before (1+ gcs-done))
         (interp-all (mapcar (lambda (m) (funcall interp-fn inner-thrower m))
                             '(normal throw signal)))
         (comp-all (mapcar (lambda (m) (funcall comp-fn inner-thrower m))
                           '(normal throw signal)))
         (gc-delta (- (1+ gcs-done) gc-before)))
    (should (equal interp-all comp-all))
    ;; Prove Bpushconditioncase (49), Bpushcatch (50), Bpophandler (48), and Bunwind_protect (142)
    (should (memq 48 ops))
    (should (memq 49 ops))
    (should (memq 50 ops))
    (should (memq 142 ops))
    (femacs-bc--record
     'tvm5-handlers-nonlocal-exit-and-gc-live-roots
     (list :outcomes (format "%S" comp-all)
           :has_pophandler (if (memq 48 ops) t :false)
           :has_pushconditioncase (if (memq 49 ops) t :false)
           :has_pushcatch (if (memq 50 ops) t :false)
           :has_unwind_protect (if (memq 142 ops) t :false))
     (list :gc_delta gc-delta))))

;;; ----------------------------------------------------------------------------
;;; T-VM6: backward-loop quit behavior (quitcounter wrapping on entry vs after
;;;        sys_setjmp handler unwind), deep nested calls/stack growth, and forced GC.
;;; ----------------------------------------------------------------------------

(ert-deftest tvm6-backward-branch-quit-and-post-setjmp-quitcounter ()
  "Verify OP_BRANCH backward-jump quitcounter wrapping from fresh entry and after sys_setjmp."
  (let* (;; Source-form interpreted vs compiled check when quit-flag is nil
         (loop-form
          '(lambda (limit)
             (let ((i 0)
                   (acc 0))
               (while (< i limit)
                 (setq acc (+ acc i))
                 (setq i (1+ i)))
               (list i acc))))
         (pair (femacs-bc--compile-form loop-form))
         (clean-interp (funcall (car pair) 300))
         (clean-comp (funcall (cdr pair) 300))
         ;; Compiled function testing exact backward-branch quitcounter behavior:
         ;; 1) From fresh entry (`quitcounter = 1'), setting `quit-flag = t' inside the loop
         ;;    triggers `maybe_quit()' when `quitcounter' wraps to 0 on the 255th backward branch.
         ;; 2) Immediately after a `sys_setjmp != 0' catch/condition-case recovery in the same frame,
         ;;    `PUSHHANDLER' sets `quitcounter = 255' (`-1 as c_uchar'), so setting `quit-flag = t'
         ;;    triggers `maybe_quit()' on the VERY FIRST (1st) backward branch!
         (quit-probe-fn
          (byte-compile
           '(lambda (recover-first-p)
              (let ((iters 0)
                    (inhibit-quit nil))
                (when recover-first-p
                  (condition-case nil
                      (signal 'arith-error '( priming-setjmp ))
                    (arith-error nil)))
                (condition-case err
                    (progn
                      (setq quit-flag t)
                      (while (< iters 1000)
                        (setq iters (1+ iters)))
                      (list 'no-quit iters))
                  (quit
                   (setq quit-flag nil)
                   (list 'caught-quit (car err) iters)))))))
         (fresh-quit (funcall quit-probe-fn nil))
         (post-setjmp-quit (funcall quit-probe-fn t)))
    (should (equal clean-interp clean-comp))
    (should (equal fresh-quit '(caught-quit quit 255)))
    (should (equal post-setjmp-quit '(caught-quit quit 1)))
    (femacs-bc--record
     'tvm6-backward-branch-quit-and-post-setjmp-quitcounter
     (list :clean_loop (vconcat clean-comp)
           :fresh_quit (format "%S" fresh-quit)
           :post_setjmp_quit (format "%S" post-setjmp-quit)))))

(ert-deftest tvm6-deep-calls-stack-growth-and-forced-gc-stress ()
  "Verify deep lexbound bytecode recursion and forced GC under low gc-cons-threshold."
  (let* ((gc-before gcs-done)
         (tree-builder
          (byte-compile
           '(lambda (self depth tag)
              (if (<= depth 0)
                  (list 'leaf tag (make-string 12 ?x))
                (let* ((left (funcall self self (1- depth) (* tag 2)))
                       (right (funcall self self (1- depth) (1+ (* tag 2)))))
                  (list 'node depth tag (length left) (car right)))))))
         (interp-builder
          (eval '(lambda (self depth tag)
                   (if (<= depth 0)
                       (list 'leaf tag (make-string 12 ?x))
                     (let* ((left (funcall self self (1- depth) (* tag 2)))
                            (right (funcall self self (1- depth) (1+ (* tag 2)))))
                       (list 'node depth tag (length left) (car right)))))
                t))
         (interp-res (funcall interp-builder interp-builder 7 1))
         (comp-res (let ((gc-cons-threshold 4096))
                     (funcall tree-builder tree-builder 7 1)))
         (gc-delta (- gcs-done gc-before)))
    (should (equal interp-res comp-res))
    (femacs-bc--record
     'tvm6-deep-calls-stack-growth-and-forced-gc-stress
     (list :tree_result (format "%S" comp-res))
     (list :gc_delta gc-delta))))

(provide 'bytecode-runtime)
;;; bytecode-runtime.el ends here
