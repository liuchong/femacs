;;; runtime-native.el --- Runtime native-profile B cases -*- lexical-binding: t; -*-

;; Profile-only ERT cases for the planned real libgccjit/ELN path.  They use
;; the actual public comp.el entry points and do not replace production native
;; compilation with a toy library.  Loading this file only defines helpers and
;; ERT tests; it never compiles or loads source.
;;
;; Primary-owned format-negative units are supplied through
;; FEMACS_RUNTIME_NEGATIVE_MANIFEST, a JSON file under
;; FEMACS_RUNTIME_TEST_ROOT with:
;;   {"schema":"femacs-runtime-native-negative-v1",
;;    "units":[{"path":"relative-under-test-root.eln",
;;              "sha256":"lowercase-hex",
;;              "kind":"null|missing|count|abi|blob",
;;              "expect_error":"native-lisp-file-inconsistent"}]}
;; Dump resurrection and live-code lifetime remain primary build/dump cases.

(require 'ert)
(require 'cl-lib)
(require 'json)
(require 'comp)

(defvar femacs-rtn--compile-debug 0)
(defvar femacs-rtn--roots nil)
(defvar femacs-rtn08-cleanup-count 0)

(defun femacs-rtn--root ()
  (let ((root (getenv "FEMACS_RUNTIME_TEST_ROOT")))
    (unless (and root (> (length root) 0))
      (error "FEMACS_RUNTIME_TEST_ROOT is not set"))
    (file-name-as-directory (expand-file-name root))))

(defun femacs-rtn--path (name)
  (let* ((root (femacs-rtn--root))
         (path (expand-file-name name root)))
    (unless (string-prefix-p root path)
      (error "Runtime-native path is outside FEMACS_RUNTIME_TEST_ROOT: %s" path))
    (make-directory (file-name-directory path) t)
    path))

(defun femacs-rtn--workdir (name)
  (let ((dir (femacs-rtn--path (concat "work-" name "/"))))
    (make-directory dir t)
    dir))

(defmacro femacs-rtn--with-compile-env (work &rest body)
  "Run BODY with all native compiler writes confined to the test root."
  `(let* ((default-directory (file-name-as-directory ,work))
          (temporary-file-directory
           (file-name-as-directory (femacs-rtn--path "tmp/")))
          (native-comp-eln-load-path
           (list (file-name-as-directory (femacs-rtn--path "eln/"))))
          (comp-running-batch-compilation t)
          (native-comp-debug femacs-rtn--compile-debug))
     (make-directory temporary-file-directory t)
     (make-directory (car native-comp-eln-load-path) t)
     ,@body))

(defun femacs-rtn--compile-form (form)
  (let ((work (femacs-rtn--workdir "form")))
    (femacs-rtn--with-compile-env work
      (let ((result (native-compile form)))
        (unless (native-comp-function-p result)
          (ert-fail (format "native-compile did not return a native function: %S" result)))
        result))))

(defun femacs-rtn--compile-file (name source)
  (let* ((work (femacs-rtn--workdir name))
         (source-file (expand-file-name (concat name ".el") work))
         (output-file (expand-file-name (concat name ".eln") work)))
    (let ((coding-system-for-write 'utf-8-unix))
      (with-temp-file source-file
        (insert ";;; -*- lexical-binding: t; -*-\n")
        (insert source)))
    (femacs-rtn--with-compile-env work
      (let ((result (native-compile source-file output-file)))
        (unless (and (stringp result) (file-exists-p result))
          (ert-fail (format "native-compile did not produce an ELN file: %S" result)))
        result))))

(defun femacs-rtn--sha256 (file)
  (with-temp-buffer
    (set-buffer-multibyte nil)
    (insert-file-contents-literally file)
    (secure-hash 'sha256 (current-buffer))))

(defun femacs-rtn--status ()
  (when (fboundp 'femacs--gc-collector-status)
    (let ((gc-cons-threshold most-positive-fixnum))
      (femacs--gc-collector-status))))

(defun femacs-rtn--total (status index)
  (and status (aref (aref status 1) index)))

(defun femacs-rtn--churn (iterations)
  (let ((gc-cons-threshold 30000)
        (gc-cons-percentage 0.0)
        (junk nil))
    (dotimes (i iterations)
      (setq junk (cons i junk)))
    (setq junk nil)))

(defun femacs-rtn--until-minor (before)
  (let ((rounds 8)
        (found nil))
    (dotimes (_ rounds)
      (unless found
        (femacs-rtn--churn 4000)
        (let ((status (femacs-rtn--status)))
          (when (and status
                     (> (femacs-rtn--total status 4) before))
            (setq found status)))))
    (unless found
      (ert-fail "native collector profile produced no completed minor"))
    found))

(ert-deftest rtn01-public-init-release-arity-and-reentry ()
  (unwind-protect
      (progn
        (should (eq (comp--init-ctxt) t))
        (let ((err (condition-case err
                       (comp--init-ctxt)
                     (error err))))
          (should (eq (car err) 'native-ice)))
        (should (eq (comp--release-ctxt) t))
        (should (eq (comp--release-ctxt) t)))
    (ignore-errors (comp--release-ctxt))))

(ert-deftest rtn02-real-compile-file-and-native-identity ()
  (let* ((feature 'femacs-rtn02-feature)
         (symbol 'femacs-rtn02-add1)
         (eln (femacs-rtn--compile-file
               "rtn02"
               "(defun femacs-rtn02-add1 (x) (+ x 1))
(provide 'femacs-rtn02-feature)
")))
    (should (stringp eln))
    (should (file-exists-p eln))
    (native-elisp-load eln)
    (should (featurep feature))
    (should (native-comp-function-p (symbol-function symbol)))
    (should (= (funcall (symbol-function symbol) 4) 5))))

(ert-deftest rtn03-advice-init-release-repeat ()
  (let* ((work (femacs-rtn--workdir "advice"))
         (init-before 0)
         (init-after 0)
         (release-before 0)
         (release-after 0)
         (bad-args nil)
         (init-advice
          (lambda (orig &rest args)
            (cl-incf init-before)
            (unless (null args) (setq bad-args t))
            (let ((result (apply orig args)))
              (when (eq result t) (cl-incf init-after))
              result)))
         (release-advice
          (lambda (orig &rest args)
            (cl-incf release-before)
            (unless (null args) (setq bad-args t))
            (let ((result (apply orig args)))
              (when (eq result t) (cl-incf release-after))
              result))))
    (advice-add 'comp--init-ctxt :around init-advice)
    (advice-add 'comp--release-ctxt :around release-advice)
    (unwind-protect
        (femacs-rtn--with-compile-env work
          (dotimes (_ 2)
            (let ((f (native-compile '(lambda (x) (+ x 1)))))
              (should (native-comp-function-p f))
              (should (= (funcall f 7) 8)))))
      (advice-remove 'comp--init-ctxt init-advice)
      (advice-remove 'comp--release-ctxt release-advice))
    (should (= init-before 2))
    (should (= init-after 2))
    (should (= release-before 2))
    (should (= release-after 2))
    (should-not bad-args)))

(ert-deftest rtn04-repeat-error-and-post-error-success ()
  (let ((work (femacs-rtn--workdir "error")))
    (femacs-rtn--with-compile-env work
      (let ((first (native-compile '(lambda (x) (+ x 2)))))
        (should (native-comp-function-p first))
        (should (= (funcall first 3) 5)))
      (let ((err (condition-case err
                     (native-compile '(lambda () femacs-rtn04-missing-variable))
                   (error err))))
        (should err))
      (let ((after (native-compile '(lambda (x) (* x 3)))))
        (should (native-comp-function-p after))
        (should (= (funcall after 4) 12))))))

(ert-deftest rtn05-verbose-debug-repeat-release ()
  (let* ((work (femacs-rtn--workdir "debug"))
         (femacs-rtn--compile-debug 2))
    (femacs-rtn--with-compile-env work
      (dotimes (_ 2)
        (let ((f (native-compile '(lambda (x) (+ x 3)))))
          (should (native-comp-function-p f))
          (should (= (funcall f 4) 7)))))))

(defun femacs-rtn06-advice-f (x y)
  (declare (speed 2))
  (+ x y))

(defun femacs-rtn06-redefine-f (x y)
  (declare (speed 2))
  (- x y))

(ert-deftest rtn06-redefinition-advice-trampoline ()
  (let ((saved-advice (symbol-function 'femacs-rtn06-advice-f))
        (saved-redefine (symbol-function 'femacs-rtn06-redefine-f))
        (feature 'femacs-rtn06-feature)
        (eln nil))
    (unwind-protect
        (progn
          (setq eln
                (femacs-rtn--compile-file
                 "rtn06"
                 "(defun femacs-rtn06-advice-f (x y)
  (declare (speed 2))
  (+ x y))
(defun femacs-rtn06-redefine-f (x y)
  (declare (speed 2))
  (- x y))
(provide 'femacs-rtn06-feature)
"))
          (native-elisp-load eln)
          (should (featurep feature))
          (should (native-comp-function-p
                   (symbol-function 'femacs-rtn06-advice-f)))
          (should (native-comp-function-p
                   (symbol-function 'femacs-rtn06-redefine-f)))
          (let* ((work (femacs-rtn--workdir "redef"))
                 (seen nil)
                 (advice (lambda (&rest args) (setq seen args))))
            (femacs-rtn--with-compile-env work
              (advice-add '+ :before advice)
              (unwind-protect
                  (progn
                    (should (= (femacs-rtn06-advice-f 3 4) 7))
                    (should (equal seen '(3 4))))
                (advice-remove '+ advice))))
          (let ((seen nil))
            (cl-letf (((symbol-function '-)
                       (lambda (&rest args)
                         (setq seen args)
                         'xxx)))
              (should (eq (femacs-rtn06-redefine-f 10 2) 'xxx))
              (should (equal seen '(10 2))))))
      (fset 'femacs-rtn06-advice-f saved-advice)
      (fset 'femacs-rtn06-redefine-f saved-redefine))))

(defun femacs-rtn--assert-native-mutation (cell)
  (should (equal (car cell) '(:young)))
  (should (equal (cdr cell) '(:young))))

(ert-deftest rtn07-native-car-cdr-mutation-and-collector ()
  (let ((f (femacs-rtn--compile-form
            '(lambda (cell value)
               (setcar cell value)
               (setcdr cell value)
               cell))))
    (let ((status (femacs-rtn--status)))
      (cond
       ((null status)
        (let ((cell (cons 0 0)))
          (should (eq (funcall f cell '(:young)) cell))
          (femacs-rtn--assert-native-mutation cell)))
       ((= (femacs-rtn--total status 0) 0)
        (let ((cell (cons 0 0)))
          (should (eq (funcall f cell '(:young)) cell))
          (femacs-rtn--assert-native-mutation cell)
          (garbage-collect)
          (femacs-rtn--assert-native-mutation cell)))
       (t
        (let ((cell (cons :old :old)))
          (let ((femacs-rtn--roots (list cell)))
            (garbage-collect)
            (femacs-rtn--until-minor (femacs-rtn--total status 4))
            (funcall f cell (list :young))
            (femacs-rtn--until-minor
             (femacs-rtn--total (femacs-rtn--status) 4))
            (femacs-rtn--assert-native-mutation cell)
            (garbage-collect)
            (femacs-rtn--assert-native-mutation cell))))))))

(defun femacs-rtn08-cleanup ()
  (cl-incf femacs-rtn08-cleanup-count))

(ert-deftest rtn08-native-error-unwind-and-gc ()
  (setq femacs-rtn08-cleanup-count 0)
  (let ((f (femacs-rtn--compile-form
            '(lambda (flag)
               (unwind-protect
                   (if flag
                       (error "rtn08-error")
                     42)
                 (femacs-rtn08-cleanup))))))
    (let ((err (condition-case err (funcall f t) (error err))))
      (should (eq (car err) 'error))
      (should (equal (cadr err) "rtn08-error")))
    (should (= femacs-rtn08-cleanup-count 1))
    (should (= (funcall f nil) 42))
    (should (= femacs-rtn08-cleanup-count 2))
    (garbage-collect)
    (should (= (funcall f nil) 42))
    (should (= femacs-rtn08-cleanup-count 3))))

(ert-deftest rtn09-duplicate-native-load ()
  (let* ((feature 'femacs-rtn09-feature)
         (symbol 'femacs-rtn09-value)
         (eln (femacs-rtn--compile-file
               "rtn09"
               "(defun femacs-rtn09-value (x) (list :rtn09 x))
(provide 'femacs-rtn09-feature)
")))
    (native-elisp-load eln)
    (should (featurep feature))
    (should (native-comp-function-p (symbol-function symbol)))
    (native-elisp-load eln)
    (should (featurep feature))
    (should (native-comp-function-p (symbol-function symbol)))
    (should (equal (funcall (symbol-function symbol) 9) '(:rtn09 9)))))

(ert-deftest rtn11-debug-file-repeat-release ()
  (let* ((root (femacs-rtn--root))
         (pwd (getenv "PWD"))
         (process-cwd (and pwd (file-name-as-directory (expand-file-name pwd))))
         (work (femacs-rtn--workdir "debug-file"))
         (femacs-rtn--compile-debug 3))
    (unless (and process-cwd (string-prefix-p root process-cwd))
      (ert-fail
       "debug3 libgccjit.log requires process cwd under FEMACS_RUNTIME_TEST_ROOT"))
    (femacs-rtn--with-compile-env work
      (dotimes (_ 2)
        (let ((f (native-compile '(lambda (x) (+ x 5)))))
          (should (native-comp-function-p f))
          (should (= (funcall f 5) 10)))))
    (should (file-exists-p (expand-file-name "libgccjit.log" process-cwd)))))

(ert-deftest rtn10-format-negative-manifest ()
  (let* ((root (femacs-rtn--root))
         (name (getenv "FEMACS_RUNTIME_NEGATIVE_MANIFEST")))
    (unless (and name (> (length name) 0))
      (ert-fail "FEMACS_RUNTIME_NEGATIVE_MANIFEST is not set by the primary driver"))
    (let* ((manifest (expand-file-name name root))
           (json-object-type 'alist)
           (json-key-type 'string)
           (json-array-type 'list)
           (document nil)
           (schema nil)
           (units nil))
      (unless (string-prefix-p root manifest)
        (ert-fail (format "Negative manifest is outside FEMACS_RUNTIME_TEST_ROOT: %s"
                          manifest)))
      (should (file-exists-p manifest))
      (setq document (json-read-file manifest))
      (setq schema (alist-get "schema" document nil nil #'equal))
      (setq units (alist-get "units" document nil nil #'equal))
      (should (equal schema "femacs-runtime-native-negative-v1"))
      ;; The same fixture must load with no fault selected. Otherwise rejection
      ;; at an unrelated early boundary could make every negative look valid.
      (let* ((control (alist-get "control" document nil nil #'equal))
             (path (alist-get "path" control nil nil #'equal))
             (sha256 (alist-get "sha256" control nil nil #'equal))
             (full (and (stringp path) (expand-file-name path root))))
        (should (equal (alist-get "kind" control nil nil #'equal) "valid-format"))
        (should (stringp sha256))
        (should (and full (string-prefix-p root full) (file-exists-p full)))
        (should (equal sha256 (femacs-rtn--sha256 full)))
        (should (eq (progn (native-elisp-load full) :control-loaded) :control-loaded)))
      (should (listp units))
      (should (> (length units) 0))
      (dolist (unit units)
        (let* ((path (alist-get "path" unit nil nil #'equal))
               (sha256 (alist-get "sha256" unit nil nil #'equal))
               (kind (alist-get "kind" unit nil nil #'equal))
               (expected (alist-get "expect_error" unit nil nil #'equal))
               (full (and (stringp path) (expand-file-name path root))))
          (should (stringp path))
          (should (stringp sha256))
          (should (stringp kind))
          (should (stringp expected))
          (unless (and full (string-prefix-p root full))
            (ert-fail (format "Negative unit is outside FEMACS_RUNTIME_TEST_ROOT: %S"
                              path)))
          (should (file-exists-p full))
          (should (equal sha256 (femacs-rtn--sha256 full)))
          (let ((err (condition-case err
                         (native-elisp-load full)
                       (error err))))
            (should err)
            (should (eq (car err) (intern expected)))))))))

(ert-deftest rtn12-native-error-hierarchy-and-handler-dispatch ()
  (dolist (entry '((native-compiler-error native-compiler-error error)
                   (native-ice native-ice native-compiler-error error)
                   (native-lisp-load-failed native-lisp-load-failed error)
                   (native-lisp-wrong-reloc native-lisp-wrong-reloc native-lisp-load-failed error)
                   (wrong-register-subr-call wrong-register-subr-call native-lisp-load-failed error)
                   (native-lisp-file-inconsistent native-lisp-file-inconsistent native-lisp-load-failed error)
                   (comp-sanitizer-error comp-sanitizer-error error)))
    (should (equal (get (car entry) 'error-conditions) (cdr entry))))
  (should (eq (condition-case nil
                  (signal 'native-lisp-file-inconsistent '("runtime fixture"))
                (native-lisp-load-failed :caught-parent))
              :caught-parent)))

(ert-deftest rtn13-batch-defalias-does-not-queue-deferred-compilation ()
  (should noninteractive)
  (let* ((symbol 'femacs-rtn13-batch-function)
         (native-comp-jit-compilation t)
         (comp-deferred-pending-h (make-hash-table :test #'eq))
         (load-true-file-name (femacs-rtn--path "rtn13.elc")))
    (unwind-protect
        (progn
          (defalias symbol (byte-compile '(lambda (x) (+ x 13))))
          (should (= (funcall symbol 2) 15))
          (should (= (hash-table-count comp-deferred-pending-h) 0)))
      (fmakunbound symbol))))

(ert-deftest rtn14-startup-native-abi-and-bytecode-fallback ()
  (should (stringp comp-abi-hash))
  (should (string-match-p "\\`[[:xdigit:]]+\\'" comp-abi-hash))
  (should (stringp comp-native-version-dir))
  (should (string-suffix-p (concat "-" comp-abi-hash) comp-native-version-dir))
  (let* ((work (femacs-rtn--workdir "abi-fallback"))
         (source (expand-file-name "rtn14-fallback.el" work))
         (compiled (concat source "c"))
         (native-comp-jit-compilation nil)
         (native-comp-eln-load-path (list (expand-file-name "empty-native-cache/" work)))
         (symbol 'femacs-rtn14-fallback-value))
    (with-temp-file source (insert "(defun femacs-rtn14-fallback-value () 314)\n"))
    (unwind-protect
        (progn
          (byte-compile-file source)
          (should (file-exists-p compiled))
          (should (load compiled nil t t))
          (should (= (funcall symbol) 314)))
      (when (fboundp symbol) (fmakunbound symbol)))))

;;; runtime-native.el ends here
