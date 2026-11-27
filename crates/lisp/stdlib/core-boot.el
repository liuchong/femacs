;;; core-boot.el --- General-purpose Lisp runtime bootstrap -*- lexical-binding: t; -*-

;;; Commentary:
;; Clean, editor-free bootstrap for the standalone general-purpose Lisp runtime.
;; Loads core language primitives, UTF-8/Unicode coding systems without requiring
;; external legacy charset map tables, CLOS/generic dispatch, sequence/map
;; utilities, file I/O helpers, byte-compiler support, and script/test runners.

;;; Code:

(setq inhibit-load-charset-map t)
(setq purify-flag nil)
(setq max-lisp-eval-depth (max max-lisp-eval-depth 4200))

(defvar lisp-core--quiet-boot
  (and (boundp 'inhibit-message)
       inhibit-message
       (let ((v (getenv-internal "LISP_VERBOSE")))
         (or (null v) (string= v "") (string= v "0"))))
  "Non-nil to suppress bootstrap load progress messages.")

;; Ensure `stdlib/', `stdlib/emacs-lisp', and `stdlib/international' sit at the
;; front of `load-path' when booting from cold stdlib sources (prior to `subr').
(let ((root (or (and (boundp 'lisp-core--boot-root) lisp-core--boot-root)
                (car load-path)))
      (subdirs '("international" "emacs-lisp")))
  (if (and (stringp root) (file-directory-p root))
      (progn
        (while subdirs
          (let ((sub-dir (expand-file-name (car subdirs) root)))
            (if (and (file-directory-p sub-dir)
                     (null (member sub-dir load-path)))
                (setq load-path (cons sub-dir load-path))))
          (setq subdirs (cdr subdirs)))
        (if (null (member root load-path))
            (setq load-path (cons root load-path))))))

;; Preload core Lisp language layers in strict dependency order.
(load "emacs-lisp/debug-early" nil lisp-core--quiet-boot)
(load "emacs-lisp/byte-run" nil lisp-core--quiet-boot)
(load "emacs-lisp/backquote" nil lisp-core--quiet-boot)
(load "subr" nil lisp-core--quiet-boot)
(load "keymap" nil lisp-core--quiet-boot)
(load "version" nil lisp-core--quiet-boot)
(load "widget" nil lisp-core--quiet-boot)
(load "custom" nil lisp-core--quiet-boot)

(unless (fboundp 'custom-declare-face)
  (defun custom-declare-face (face _spec _doc &rest _args)
    "No-op face declaration stub for headless general-purpose runtime."
    face))

(unless (fboundp 'font-lock-add-keywords)
  (defun font-lock-add-keywords (&rest _args)
    "No-op syntax-highlighting hook for headless general-purpose runtime."
    nil))

(unless (boundp 'minor-mode-alist)
  (defvar minor-mode-alist nil))
(unless (boundp 'minor-mode-map-alist)
  (defvar minor-mode-map-alist nil))
(unless (boundp 'tramp-mode)
  (defvar tramp-mode nil))
(unless (boundp 'special-mode-map)
  (defvar special-mode-map (make-sparse-keymap)))
(unless (boundp 'prog-mode-map)
  (defvar prog-mode-map (make-sparse-keymap)))
(unless (boundp 'prog-mode-syntax-table)
  (defvar prog-mode-syntax-table (make-syntax-table)))
(unless (fboundp 'prog-mode)
  (defun prog-mode ()
    "Minimal headless parent mode stub."
    (setq major-mode 'prog-mode)))
(unless (boundp 'font-lock-mode)
  (defvar font-lock-mode nil))
(unless (fboundp 'font-lock-mode)
  (defun font-lock-mode (&rest _args) nil))
(unless (fboundp 'font-lock-ensure)
  (defun font-lock-ensure (&rest _args) nil))
(unless (fboundp 'setopt--set-local)
  (defun setopt--set-local (variable value)
    "Minimal headless buffer-local setter for file-local variables."
    (set (make-local-variable variable) value)))
(unless (fboundp 'count-lines)
  (defun count-lines (start end &optional ignore-invisible-lines)
    "Return number of lines between START and END."
    (declare (ftype (function ((or integer marker) (or integer marker) &optional t)
                             integer))
             (side-effect-free t))
    (save-excursion
      (save-restriction
        (narrow-to-region start end)
        (cond ((and (not ignore-invisible-lines)
                    (eq selective-display t))
               (goto-char (point-min))
               (save-match-data
                 (let ((done 0))
                   (while (re-search-forward "\n\\|\r[^\n]" nil t 40)
                     (setq done (+ 40 done)))
                   (while (re-search-forward "\n\\|\r[^\n]" nil t 1)
                     (setq done (+ 1 done)))
                   (goto-char (point-max))
                   (if (and (/= start end)
                            (not (bolp)))
                       (1+ done)
                     done))))
              (ignore-invisible-lines
               (goto-char (point-min))
               (save-match-data
                 (- (buffer-size)
                    (forward-line (buffer-size))
                    (let ((invisible-count 0)
                          prop)
                      (goto-char (point-min))
                      (while (re-search-forward "\n\\|\r[^\n]" nil t)
                        (setq prop (get-char-property (1- (point)) 'invisible))
                        (if (if (eq buffer-invisibility-spec t)
                                prop
                              (or (memq prop buffer-invisibility-spec)
                                  (assq prop buffer-invisibility-spec)))
                            (setq invisible-count (1+ invisible-count))))
                      invisible-count))))
              (t
               (goto-char (point-max))
               (if (bolp)
                   (1- (line-number-at-pos))
                 (line-number-at-pos))))))))

(unless (fboundp 'messages-buffer)
  (defun messages-buffer ()
    "Return the \"*Messages*\" buffer, creating it if needed."
    (or (get-buffer "*Messages*")
        (get-buffer-create "*Messages*"))))
(unless (fboundp 'buffer-disable-undo)
  (defun buffer-disable-undo (&optional buffer)
    "Disable undo recording in BUFFER (or current buffer)."
    (with-current-buffer (if buffer (get-buffer buffer) (current-buffer))
      (setq buffer-undo-list t))))
(unless (fboundp 'filter-buffer-substring)
  (defun filter-buffer-substring (beg end &optional delete)
    "Return substring between BEG and END, deleting it if DELETE is non-nil."
    (if delete
        (delete-and-extract-region beg end)
      (buffer-substring beg end))))
(unless (fboundp 'newline)
  (defun newline (&optional arg _interactive)
    "Insert ARG (default 1) newlines."
    (insert (make-string (or arg 1) ?\n))))
(unless (fboundp 'deactivate-mark)
  (defun deactivate-mark (&optional _force) nil))
(unless (fboundp 'undo-auto-amalgamate)
  (defun undo-auto-amalgamate () nil))
(unless (fboundp 'string-empty-p)
  (defsubst string-empty-p (string)
    "Check whether STRING is empty."
    (declare (pure t) (side-effect-free t))
    (string= string "")))
(unless (fboundp 'tty-set-up-initial-frame-faces)
  (defun tty-set-up-initial-frame-faces ()
    "No-op initial TTY frame face setup for headless runtime."
    nil))
(unless (boundp 'emacs-lisp-mode-syntax-table)
  (defvar emacs-lisp-mode-syntax-table (make-syntax-table)))
(unless (fboundp 'emacs-lisp-mode)
  (defun emacs-lisp-mode ()
    "Minimal headless major mode stub for Lisp source inspection."
    (setq major-mode 'emacs-lisp-mode)
    (set-syntax-table emacs-lisp-mode-syntax-table)))
(unless (fboundp 'easy-menu-define)
  (defmacro easy-menu-define (&rest _args) nil))
(unless (fboundp 'tool-bar-local-item)
  (defun tool-bar-local-item (&rest _args) nil))
(unless (fboundp 'tool-bar-local-item-from-menu)
  (defun tool-bar-local-item-from-menu (&rest _args) nil))
(unless (fboundp 'define-abbrev-table)
  (defun define-abbrev-table (tablename definitions &optional docstring &rest props)
    "Minimal headless abbrev-table stub."
    (unless (boundp tablename)
      (set tablename (if (fboundp 'obarray-make)
                         (obarray-make 17)
                       (make-vector 17 0))))
    (put tablename 'abbrev-table-definitions (cons (cons definitions props) docstring))
    tablename))
(unless (featurep 'compile)
  (defmacro define-compilation-mode (mode name doc &rest body)
    "Minimal headless stub for `define-compilation-mode'."
    `(defun ,mode ()
       ,doc
       (setq major-mode ',mode)
       (setq mode-name ,name)
       ,@body))
  (defun compilation-forget-errors () nil)
  (provide 'compile))

(unless (fboundp 'bound-and-true-p)
  (defmacro bound-and-true-p (var)
    "Return the value of symbol VAR if it is bound, else nil."
    `(and (boundp ',var) ,var)))

(load "international/mule" nil lisp-core--quiet-boot)

;; Load full `international/mule-conf' when external `.map' files exist; otherwise
;; define all map-free core Unicode/ASCII/Latin-1/UTF-8/UTF-16/binary coding systems.
(if (and (boundp 'charset-map-path)
         charset-map-path
         (locate-file-internal "8859-2" charset-map-path '(".map" "")))
    (progn
      (load "international/mule-conf" nil lisp-core--quiet-boot)
      (load "env" nil lisp-core--quiet-boot)
      (load "format" nil lisp-core--quiet-boot)
      (load "obarray" nil lisp-core--quiet-boot)
      (load "case-table" nil lisp-core--quiet-boot)
      (load "international/characters" nil lisp-core--quiet-boot))
  (progn
    (define-charset-alias 'ucs 'unicode)
    (define-charset 'latin-iso8859-1
      "Right-Hand Part of ISO/IEC 8859/1 (Latin-1): ISO-IR-100"
      :short-name "RHP of Latin-1"
      :long-name "RHP of ISO/IEC 8859/1 (Latin-1): ISO-IR-100"
      :iso-final-char ?A
      :emacs-mule-id 129
      :code-space [32 127]
      :code-offset 160)
    (define-charset 'control-1
      "8-bit control code (0x80..0x9F)"
      :short-name "8-bit control code"
      :code-space [128 159]
      :code-offset 128)
    (define-charset 'eight-bit-control
      "Raw bytes in the range 0x80..0x9F"
      :short-name "Raw bytes 0x80..0x9F"
      :supplementary-p t
      :code-space [128 159]
      :code-offset #x3FFF80)
    (define-charset 'eight-bit-graphic
      "Raw bytes in the range 0xA0..0xFF"
      :short-name "Raw bytes 0xA0..0xFF"
      :supplementary-p t
      :code-space [160 255]
      :code-offset #x3FFFA0)
    (define-charset 'unicode-bmp
      "Unicode Basic Multilingual Plane (U+0000..U+FFFF)"
      :short-name "Unicode BMP"
      :code-space [0 255 0 255]
      :code-offset 0)
    (define-charset 'unicode-smp
      "Unicode Supplementary Multilingual Plane (U+10000..U+1FFFF)"
      :short-name "Unicode SMP"
      :code-space [0 255 0 255]
      :code-offset #x10000)
    (define-charset 'unicode-sip
      "Unicode Supplementary Ideographic Plane (U+20000..U+2FFFF)"
      :short-name "Unicode SIP"
      :code-space [0 255 0 255]
      :code-offset #x20000)
    (define-charset 'unicode-ssp
      "Unicode Supplementary Special-purpose Plane (U+E0000..U+EFFFF)"
      :short-name "Unicode SSP"
      :code-space [0 255 0 255]
      :code-offset #xE0000)

    (setq standard-translation-table-for-decode nil)
    (setq standard-translation-table-for-encode nil)

    (define-coding-system-alias 'binary 'no-conversion)
    (define-coding-system-alias 'unix 'undecided-unix)
    (define-coding-system-alias 'dos 'undecided-dos)
    (define-coding-system-alias 'mac 'undecided-mac)

    (define-coding-system 'prefer-utf-8
      "Like `undecided' but prefer UTF-8 when appropriate."
      :coding-type 'undecided
      :mnemonic ?-
      :charset-list '(emacs)
      :prefer-utf-8 t
      :inhibit-null-byte-detection 0
      :inhibit-iso-escape-detection 0)
    (define-coding-system 'raw-text
      "Raw text preserving 8-bit bytes."
      :coding-type 'raw-text
      :for-unibyte t
      :mnemonic ?t)
    (define-coding-system 'no-conversion-multibyte
      "Raw bytes in multibyte form."
      :coding-type 'raw-text
      :eol-type 'unix
      :mnemonic ?=)
    (define-coding-system 'iso-latin-1
      "ISO-8859-1 Latin-1 coding system."
      :coding-type 'charset
      :mnemonic ?1
      :charset-list '(iso-8859-1)
      :mime-charset 'iso-8859-1)
    (define-coding-system-alias 'iso-8859-1 'iso-latin-1)
    (define-coding-system-alias 'latin-1 'iso-latin-1)

    (define-coding-system 'us-ascii
      "US-ASCII coding system."
      :coding-type 'charset
      :mnemonic ?-
      :charset-list '(ascii)
      :default-char ??
      :mime-charset 'us-ascii)
    (define-coding-system-alias 'iso-safe 'us-ascii)
    (define-coding-system-alias 'ascii 'us-ascii)

    (define-coding-system 'utf-8
      "UTF-8 (no signature BOM)."
      :coding-type 'utf-8
      :mnemonic ?U
      :charset-list '(unicode)
      :mime-charset 'utf-8)
    (define-coding-system 'utf-8-with-signature
      "UTF-8 (with signature BOM)."
      :coding-type 'utf-8
      :mnemonic ?U
      :charset-list '(unicode)
      :bom t)
    (define-coding-system 'utf-8-auto
      "UTF-8 (auto-detect signature BOM)."
      :coding-type 'utf-8
      :mnemonic ?U
      :charset-list '(unicode)
      :bom '(utf-8-with-signature . utf-8))
    (define-coding-system-alias 'mule-utf-8 'utf-8)
    (define-coding-system-alias 'cp65001 'utf-8)

    (define-coding-system 'utf-8-emacs
      "Internal UTF-8 coding supporting all extended characters."
      :coding-type 'utf-8
      :mnemonic ?U
      :charset-list '(emacs))
    (define-coding-system-alias 'emacs-internal 'utf-8-emacs-unix)

    (define-coding-system 'utf-16le
      "UTF-16LE (little endian, no BOM)."
      :coding-type 'utf-16
      :mnemonic ?U
      :charset-list '(unicode)
      :endian 'little
      :mime-text-unsuitable t
      :mime-charset 'utf-16le)
    (define-coding-system 'utf-16be
      "UTF-16BE (big endian, no BOM)."
      :coding-type 'utf-16
      :mnemonic ?U
      :charset-list '(unicode)
      :endian 'big
      :mime-text-unsuitable t
      :mime-charset 'utf-16be)
    (define-coding-system 'utf-16le-with-signature
      "UTF-16 (little endian, with BOM)."
      :coding-type 'utf-16
      :mnemonic ?U
      :charset-list '(unicode)
      :bom t
      :endian 'little
      :mime-text-unsuitable t
      :mime-charset 'utf-16)
    (define-coding-system 'utf-16be-with-signature
      "UTF-16 (big endian, with BOM)."
      :coding-type 'utf-16
      :mnemonic ?U
      :charset-list '(unicode)
      :bom t
      :endian 'big
      :mime-text-unsuitable t
      :mime-charset 'utf-16)
    (define-coding-system 'utf-16
      "UTF-16 (auto-detect endianness, big endian with BOM on encode)."
      :coding-type 'utf-16
      :mnemonic ?U
      :charset-list '(unicode)
      :bom '(utf-16le-with-signature . utf-16be-with-signature)
      :endian 'big
      :mime-text-unsuitable t
      :mime-charset 'utf-16)
    (define-coding-system-alias 'utf-16-le 'utf-16le-with-signature)
    (define-coding-system-alias 'utf-16-be 'utf-16be-with-signature)

    (set-safe-terminal-coding-system-internal 'us-ascii)
    (setq file-coding-system-alist
          '(("\\.elc\\'" . utf-8-emacs)
            ("\\.el\\'" . prefer-utf-8)
            ("\\.utf\\(-8\\)?\\'" . utf-8)
            ("\\(\\`\\|/\\)loaddefs.el\\'" . (raw-text . raw-text-unix))
            ("\\.tar\\'" . (no-conversion . no-conversion))
            ("" . (undecided . nil))))
    (set-coding-system-priority 'utf-8 'iso-latin-1)

    (load "env" nil lisp-core--quiet-boot)
    (load "format" nil lisp-core--quiet-boot)
    (load "obarray" nil lisp-core--quiet-boot)
    (load "case-table" nil lisp-core--quiet-boot)
    (load "core-autoloads" nil lisp-core--quiet-boot)
    (load "emacs-lisp/derived" nil lisp-core--quiet-boot)

    (setq load-source-file-function #'load-with-code-conversion)
    (load "files" nil lisp-core--quiet-boot)

    ;; Bootstrap eager macro-expansion with `pcase' before `international/characters'.
    (load "emacs-lisp/macroexp" nil lisp-core--quiet-boot)
    (unless (compiled-function-p (symbol-function 'macroexpand-all))
      (let ((macroexp--pending-eager-loads '(skip)))
        (load "emacs-lisp/pcase" nil lisp-core--quiet-boot)
        (load "emacs-lisp/inline" nil lisp-core--quiet-boot))
      (let ((max-lisp-eval-depth (* 2 max-lisp-eval-depth)))
        (load "emacs-lisp/macroexp" nil lisp-core--quiet-boot)))
    (require 'gv)
    (load "emacs-lisp/easy-mmode" nil lisp-core--quiet-boot)
    (load "button" nil lisp-core--quiet-boot)

    (defvar current-language-environment "UTF-8")
    (unless (fboundp 'get-language-info)
      (defun get-language-info (_lang-env _key) nil))
    (let ((orig-put (symbol-function 'put-charset-property))
          (orig-map (symbol-function 'map-charset-chars))
          (orig-dec (symbol-function 'decode-char))
          (orig-enc (symbol-function 'encode-char))
          (orig-pair (symbol-function 'set-case-syntax-pair)))
      (unwind-protect
          (progn
            (fset 'put-charset-property
                  (lambda (charset propname value)
                    (when (charsetp charset)
                      (funcall orig-put charset propname value))))
            (fset 'map-charset-chars
                  (lambda (function charset &optional arg from-code to-code)
                    (when (charsetp charset)
                      (funcall orig-map function charset arg from-code to-code))))
            (fset 'decode-char
                  (lambda (charset code-point)
                    (when (charsetp charset)
                      (funcall orig-dec charset code-point))))
            (fset 'encode-char
                  (lambda (char charset)
                    (when (and char (charsetp charset))
                      (funcall orig-enc char charset))))
            (fset 'set-case-syntax-pair
                  (lambda (uc lc table)
                    (when (and (integerp uc) (integerp lc))
                      (funcall orig-pair uc lc table))))
            (load "international/characters" nil lisp-core--quiet-boot))
        (fset 'put-charset-property orig-put)
        (fset 'map-charset-chars orig-map)
        (fset 'decode-char orig-dec)
        (fset 'encode-char orig-enc)
        (fset 'set-case-syntax-pair orig-pair)))))

(unless (featurep 'core-autoloads)
  (load "core-autoloads" nil lisp-core--quiet-boot))
(unless (featurep 'derived)
  (load "emacs-lisp/derived" nil lisp-core--quiet-boot))
(unless (featurep 'files)
  (setq load-source-file-function #'load-with-code-conversion)
  (load "files" nil lisp-core--quiet-boot))

(unless (featurep 'macroexp)
  (load "emacs-lisp/macroexp" nil lisp-core--quiet-boot)
  (unless (compiled-function-p (symbol-function 'macroexpand-all))
    (let ((macroexp--pending-eager-loads '(skip)))
      (load "emacs-lisp/pcase" nil lisp-core--quiet-boot)
      (load "emacs-lisp/inline" nil lisp-core--quiet-boot))
    (let ((max-lisp-eval-depth (* 2 max-lisp-eval-depth)))
      (load "emacs-lisp/macroexp" nil lisp-core--quiet-boot))))

(require 'gv)
(unless (featurep 'easy-mmode)
  (load "emacs-lisp/easy-mmode" nil lisp-core--quiet-boot))
(unless (featurep 'button)
  (load "button" nil lisp-core--quiet-boot))
(load "emacs-lisp/cl-preloaded" nil lisp-core--quiet-boot)
(load "emacs-lisp/oclosure" nil lisp-core--quiet-boot)
(load "emacs-lisp/cl-generic" nil lisp-core--quiet-boot)
(load "emacs-lisp/seq" nil lisp-core--quiet-boot)
(load "emacs-lisp/map" nil lisp-core--quiet-boot)
(load "emacs-lisp/subr-x" nil lisp-core--quiet-boot)
(load "emacs-lisp/nadvice" nil lisp-core--quiet-boot)
(load "emacs-lisp/timer" nil lisp-core--quiet-boot)
(load "emacs-lisp/float-sup" nil lisp-core--quiet-boot)
(require 'rx)
(rx-define lisp-mode-symbol (+ (| (syntax word)
                                  (syntax symbol)
                                  (: "\\" nonl))))
(load "emacs-lisp/shorthands" nil lisp-core--quiet-boot)
(load "emacs-lisp/warnings" nil lisp-core--quiet-boot)
(load "emacs-lisp/cconv" nil lisp-core--quiet-boot)

(unless (fboundp 'function-documentation)
  (cl-defgeneric function-documentation (function)
    "Extract the raw docstring info from FUNCTION."
    (let ((docstring-p (lambda (doc)
                         (or (stringp doc)
                             (fixnump doc) (fixnump (cdr-safe doc))))))
      (pcase function
        ((pred closurep)
         (when (> (length function) 4)
           (let ((doc (aref function 4)))
             (when (funcall docstring-p doc) doc))))
        ((or (pred stringp) (pred vectorp)) "Keyboard macro.")
        (`(keymap . ,_)
         "Prefix command (definition is a keymap associating keystrokes with commands).")
        ((or `(lambda ,_args . ,body) `(autoload ,_file . ,body))
         (let ((doc (car body)))
           (when (funcall docstring-p doc)
             doc)))
        ((pred symbolp)
         (let ((f (indirect-function function)))
           (if f (function-documentation f)
             (signal 'void-function (list function)))))
        (`(macro . ,f) (function-documentation f))
        (_
         (let ((doc (internal-subr-documentation function)))
           (if (eq t doc)
               (signal 'invalid-function (list function))
             doc))))))
  (cl-defmethod function-documentation ((function accessor))
    (oclosure--accessor-docstring function))
  (cl-defmethod function-documentation ((f cconv--interactive-helper))
    (function-documentation (cconv--interactive-helper--fun f))))

(when (compiled-function-p (symbol-function 'cconv-fv))
  (setq internal-make-interpreted-closure-function
        #'cconv-make-interpreted-closure))

;; Default clean UTF-8 coding behavior for general-purpose scripting & I/O.
(set-coding-system-priority 'utf-8 'iso-latin-1)
(setq-default buffer-file-coding-system 'utf-8)
(setq default-file-name-coding-system 'utf-8)
(setq file-name-coding-system 'utf-8)
(set-terminal-coding-system 'utf-8)

;; Load general-purpose prelude (script runner, strict compile, ERT driver).
(load "core-prelude" nil lisp-core--quiet-boot)

;; Clear temporary load tracking before optional portable dump creation.
(setq current-load-list nil)
(setq custom-current-group-alist nil)

;;; core-boot.el ends here
