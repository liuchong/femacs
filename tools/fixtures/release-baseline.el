;;; release-baseline.el --- Bounded Stage IV release compatibility ERT suite -*- lexical-binding: t; -*-

;; Independent partner-owned ERT fixture for Stage IV release baseline verification
;; (IV-D4 and IV-A2 in docs/modernization.md).
;;
;; Covers 10 bounded, deterministic functional groups across:
;;   1. User-style configuration (`early-init.el`, `init.el`, `custom-file` round-trip, keymaps)
;;   2. Shipped core libraries (`cl-lib`, `seq`, `subr-x`, `rx`)
;;   3. Shipped buffer/workflow modes (`outline-mode`, `tabulated-list-mode`, `dired`)
;;   4. Shipped developer tooling (`compile`, `diff-mode`, rectangles & paren balance)
;;   5. Unicode/multibyte coding systems, EOL conversion (`utf-8-unix`/`utf-8-dos`), file I/O
;;   6. Buffer search, regex subgroup replacement, `case-fold-search`, narrowing/widening
;;   7. Undo/redo chains across `undo-boundary` and `atomic-change-group` rollback on error
;;   8. Markers (insertion types), overlays, text properties, and point/text/number registers
;;   9. Error hierarchies (`define-error`/`condition-case`), `unwind-protect`, `catch`/`throw`
;;  10. Active GC root preservation (lexical closures, buffers, markers, weak/strong hashes)
;;
;; Loaded in batch mode via `tools/check-ert.py` and `tools/test-release-baseline.py`.

(require 'ert)
(require 'cl-lib)
(require 'seq)
(require 'subr-x)
(require 'rx)
(require 'json)
(require 'cus-edit)
(require 'outline)
(require 'tabulated-list)
(require 'dired)
(require 'compile)
(require 'diff-mode)
(require 'elec-pair)
(require 'register)
(require 'rect)

(defvar femacs-release-baseline-observation-file nil
  "Optional JSON file path where deterministic per-test observations are written.")

(defvar femacs-rb--observations nil
  "Alist of (TEST-NAME . OBSERVATION-HASH) accumulated during the batch run.")

(defvar femacs-rb--diagnostics nil
  "Alist of (TEST-NAME . DIAGNOSTIC-HASH) for non-deterministic metrics like GC counts.")

(defun femacs-rb--plist-to-hash (plist)
  "Convert a flat PLIST with keyword/string keys into an `equal' hash table."
  (let ((ht (make-hash-table :test 'equal)))
    (while plist
      (let ((k (car plist))
            (v (cadr plist)))
        (puthash (if (symbolp k) (substring (symbol-name k) 1) (format "%s" k))
                 v ht)
        (setq plist (cddr plist))))
    ht))

(defun femacs-rb--flush-observations ()
  "Persist accumulated observations and diagnostics as UTF-8 JSON when configured."
  (when (and (stringp femacs-release-baseline-observation-file)
             (> (length femacs-release-baseline-observation-file) 0))
    (let* ((obs-hash (make-hash-table :test 'equal))
           (diag-hash (make-hash-table :test 'equal))
           (root (make-hash-table :test 'equal)))
      (dolist (entry (reverse femacs-rb--observations))
        (puthash (car entry) (cdr entry) obs-hash))
      (dolist (entry (reverse femacs-rb--diagnostics))
        (puthash (car entry) (cdr entry) diag-hash))
      (puthash "schema" "femacs-release-baseline-observations-v1" root)
      (puthash "test_count" (hash-table-count obs-hash) root)
      (puthash "observations" obs-hash root)
      (puthash "diagnostics" diag-hash root)
      (let ((coding-system-for-write 'utf-8-unix))
        (with-temp-file femacs-release-baseline-observation-file
          (insert (json-serialize root)))))))

(defun femacs-rb--record (test-name deterministic-plist &optional diagnostic-plist)
  "Record DETERMINISTIC-PLIST (and optional DIAGNOSTIC-PLIST) for TEST-NAME and flush."
  (let ((key (if (symbolp test-name) (symbol-name test-name) test-name)))
    (setq femacs-rb--observations
          (cons (cons key (femacs-rb--plist-to-hash deterministic-plist))
                (assoc-delete-all key femacs-rb--observations)))
    (when diagnostic-plist
      (setq femacs-rb--diagnostics
            (cons (cons key (femacs-rb--plist-to-hash diagnostic-plist))
                  (assoc-delete-all key femacs-rb--diagnostics))))
    (femacs-rb--flush-observations)))

;; Custom variables and structs used by Group 1 and Group 2
(defgroup femacs-rb-test nil
  "Temporary group for Stage IV release baseline tests."
  :group 'emacs)

(defcustom femacs-rb01-custom-limit 128
  "Test customizable integer option."
  :type 'integer
  :group 'femacs-rb-test)

(defcustom femacs-rb01-custom-labels '("default")
  "Test customizable string list option."
  :type '(repeat string)
  :group 'femacs-rb-test)

(defvar femacs-rb01-load-trace nil
  "Trace list populated by simulated early-init and init files.")

(cl-defstruct (femacs-rb02-record (:constructor femacs-rb02-make-record))
  id label weight tags)

;; Error hierarchy used by Group 9
(define-error 'femacs-rb09-base-error "Femacs baseline error")
(define-error 'femacs-rb09-child-error "Femacs child baseline error" 'femacs-rb09-base-error)

;;; Group 1: User-style configuration (early-init, init, custom-file, keymaps)

(ert-deftest rb01-user-config-init-and-custom-roundtrip ()
  "Verify user-style early-init/init loading, custom-file persistence, and keymap lookup."
  (let ((tmpdir (make-temp-file "femacs-rb01-" t))
        (orig-limit femacs-rb01-custom-limit)
        (orig-labels femacs-rb01-custom-labels)
        (orig-custom-file custom-file)
        (inhibit-message t))
    (unwind-protect
        (let ((early-init (expand-file-name "early-init.el" tmpdir))
              (user-init (expand-file-name "init.el" tmpdir))
              (cust-file (expand-file-name "custom.el" tmpdir))
              (keymap (make-sparse-keymap)))
          (setq femacs-rb01-load-trace nil)
          (let ((coding-system-for-write 'utf-8-unix))
            (with-temp-file early-init
              (insert ";; -*- lexical-binding: t; -*-\n"
                      "(push 'early-init femacs-rb01-load-trace)\n"))
            (with-temp-file user-init
              (insert ";; -*- lexical-binding: t; -*-\n"
                      "(push 'user-init femacs-rb01-load-trace)\n"
                      "(setq custom-file " (format "%S" cust-file) ")\n"))
            (with-temp-file cust-file
              (insert ";; -*- lexical-binding: t; -*-\n")))
          (let ((user-emacs-directory (file-name-as-directory tmpdir)))
            (load early-init nil t)
            (load user-init nil t))
          (should (equal (reverse femacs-rb01-load-trace) '(early-init user-init)))
          (should (equal custom-file cust-file))
          ;; Customize variables, save to custom-file, perturb, and reload
          (custom-set-variables
           '(femacs-rb01-custom-limit 4096)
           '(femacs-rb01-custom-labels '("alpha" "中文-tag" "omega")))
          (should (= femacs-rb01-custom-limit 4096))
          (should (equal femacs-rb01-custom-labels '("alpha" "中文-tag" "omega")))
          (let ((user-init-file user-init)) (custom-save-all))
          (should (file-exists-p cust-file))
          (setq femacs-rb01-custom-limit 1
                femacs-rb01-custom-labels nil)
          (load cust-file nil t)
          (should (= femacs-rb01-custom-limit 4096))
          (should (equal femacs-rb01-custom-labels '("alpha" "中文-tag" "omega")))
          ;; Verify keymap binding and parent inheritance
          (let ((parent-map (make-sparse-keymap)))
            (define-key parent-map (kbd "C-c a") 'femacs-rb01-parent-cmd)
            (set-keymap-parent keymap parent-map)
            (define-key keymap (kbd "C-c b") 'femacs-rb01-child-cmd)
            (should (eq (lookup-key keymap (kbd "C-c a")) 'femacs-rb01-parent-cmd))
            (should (eq (lookup-key keymap (kbd "C-c b")) 'femacs-rb01-child-cmd))
            (femacs-rb--record
             'rb01-user-config-init-and-custom-roundtrip
             (list :trace (vconcat (mapcar #'symbol-name (reverse femacs-rb01-load-trace)))
                   :limit femacs-rb01-custom-limit
                   :labels (vconcat femacs-rb01-custom-labels)
                   :key_a (symbol-name (lookup-key keymap (kbd "C-c a")))
                   :key_b (symbol-name (lookup-key keymap (kbd "C-c b")))))))
      (setq femacs-rb01-custom-limit orig-limit
            femacs-rb01-custom-labels orig-labels
            custom-file orig-custom-file)
      (when (file-directory-p tmpdir)
        (delete-directory tmpdir t)))))

;;; Group 2: Shipped core libraries (cl-lib, seq, subr-x, rx)

(ert-deftest rb02-shipped-libraries-cl-seq-subr-x-and-rx ()
  "Verify cl-lib structs/loops, seq combinators, subr-x threading, and rx regex compilation."
  (let* ((records (list (femacs-rb02-make-record :id 1 :label " core " :weight 15 :tags '(a b))
                        (femacs-rb02-make-record :id 2 :label "skip" :weight 4 :tags '(b))
                        (femacs-rb02-make-record :id 3 :label " vm_opt " :weight 25 :tags '(a c))
                        (femacs-rb02-make-record :id 4 :label "gc-root" :weight 18 :tags '(c))))
         (filtered (seq-filter (lambda (r) (>= (femacs-rb02-record-weight r) 15)) records))
         (labels (seq-map (lambda (r) (string-trim (femacs-rb02-record-label r))) filtered))
         (total-weight (cl-loop for r in filtered sum (femacs-rb02-record-weight r)))
         (joined (string-join (thread-last labels (seq-map #'upcase)) "|"))
         (pattern (rx bol "pkg-" (group (+ digit)) ":" (+ space)
                      (group (+ (any alnum "_" "-"))) eol))
         (sample "pkg-308: native_cocoa-v4"))
    (should (= (length filtered) 3))
    (should (equal labels '("core" "vm_opt" "gc-root")))
    (should (= total-weight 58))
    (should (equal joined "CORE|VM_OPT|GC-ROOT"))
    (should (string-match pattern sample))
    (let ((matched-id (match-string 1 sample))
          (matched-name (match-string 2 sample))
          (destructured (cl-destructuring-bind (a b &rest rest) '(10 20 30 40)
                          (list (+ a b) (length rest)))))
      (should (equal matched-id "308"))
      (should (equal matched-name "native_cocoa-v4"))
      (should (equal destructured '(30 2)))
      (femacs-rb--record
       'rb02-shipped-libraries-cl-seq-subr-x-and-rx
       (list :filtered_ids (vconcat (mapcar #'femacs-rb02-record-id filtered))
             :joined joined
             :total_weight total-weight
             :rx_id matched-id
             :rx_name matched-name
             :destructured (vconcat destructured))))))

;;; Group 3: Shipped buffer/workflow modes (outline-mode, tabulated-list-mode, dired)

(ert-deftest rb03-shipped-modes-outline-tabulated-list-and-dired ()
  "Verify outline heading navigation/promotion, tabulated-list sorting, and dired directory listing."
  (let ((outline-headings nil)
        (promoted-line nil)
        (tab-ids-asc nil)
        (tab-ids-desc nil)
        (dired-names nil)
        (tmpdir (make-temp-file "femacs-rb03-" t))
        (inhibit-message t))
    (unwind-protect
        (progn
          ;; 1. outline-mode navigation and heading promotion
          (with-temp-buffer
            (outline-mode)
            (insert "* Section Alpha\nAlpha body\n** Subsection A1\nLeaf A1\n* Section Beta\nBeta body\n")
            (goto-char (point-min))
            (should (outline-on-heading-p))
            (push (buffer-substring-no-properties (line-beginning-position) (line-end-position))
                  outline-headings)
            (outline-next-visible-heading 1)
            (push (buffer-substring-no-properties (line-beginning-position) (line-end-position))
                  outline-headings)
            (outline-promote)
            (setq promoted-line
                  (buffer-substring-no-properties (line-beginning-position) (line-end-position)))
            (outline-next-visible-heading 1)
            (push (buffer-substring-no-properties (line-beginning-position) (line-end-position))
                  outline-headings)
            (setq outline-headings (nreverse outline-headings)))
          (should (equal outline-headings '("* Section Alpha" "** Subsection A1" "* Section Beta")))
          (should (equal promoted-line "* Subsection A1"))

          ;; 2. tabulated-list-mode rendering and sorting
          (with-temp-buffer
            (tabulated-list-mode)
            (setq tabulated-list-format [("Component" 14 t) ("State" 8 nil)])
            (setq tabulated-list-entries
                  (list (list "id-vm" ["vm-dispatch" "ready"])
                        (list "id-alloc" ["allocator" "ready"])
                        (list "id-ns" ["nsterm" "active"])))
            (setq tabulated-list-sort-key '("Component" . nil))
            (tabulated-list-init-header)
            (tabulated-list-print t)
            (goto-char (point-min))
            (while (not (eobp))
              (when (tabulated-list-get-id)
                (push (tabulated-list-get-id) tab-ids-asc))
              (forward-line 1))
            (setq tab-ids-asc (nreverse tab-ids-asc))
            (setq tabulated-list-sort-key '("Component" . t))
            (tabulated-list-print t)
            (goto-char (point-min))
            (while (not (eobp))
              (when (tabulated-list-get-id)
                (push (tabulated-list-get-id) tab-ids-desc))
              (forward-line 1))
            (setq tab-ids-desc (nreverse tab-ids-desc)))
          (should (equal tab-ids-asc '("id-alloc" "id-ns" "id-vm")))
          (should (equal tab-ids-desc '("id-vm" "id-ns" "id-alloc")))

          ;; 3. dired directory listing and filename extraction
          (let ((file-a (expand-file-name "01-alpha.txt" tmpdir))
                (file-b (expand-file-name "02-beta.el" tmpdir))
                (dired-use-ls-dired nil))
            (with-temp-file file-a (insert "alpha\n"))
            (with-temp-file file-b (insert ";; beta\n"))
            (let ((dbuf (dired-noselect tmpdir "-al")))
              (unwind-protect
                  (with-current-buffer dbuf
                    (should (derived-mode-p 'dired-mode))
                    (goto-char (point-min))
                    (should (dired-goto-file file-a))
                    (push (file-name-nondirectory (dired-get-filename)) dired-names)
                    (should (dired-goto-file file-b))
                    (push (file-name-nondirectory (dired-get-filename)) dired-names)
                    (setq dired-names (nreverse dired-names)))
                (when (buffer-live-p dbuf)
                  (kill-buffer dbuf)))))
          (should (equal dired-names '("01-alpha.txt" "02-beta.el")))
          (femacs-rb--record
           'rb03-shipped-modes-outline-tabulated-list-and-dired
           (list :outline_headings (vconcat outline-headings)
                 :promoted promoted-line
                 :tab_asc (vconcat tab-ids-asc)
                 :tab_desc (vconcat tab-ids-desc)
                 :dired_names (vconcat dired-names))))
      (when (file-directory-p tmpdir)
        (delete-directory tmpdir t)))))

;;; Group 4: Shipped developer tooling (compile, diff-mode, rectangles & paren check)

(ert-deftest rb04-shipped-tooling-compile-diff-and-electric-pair ()
  "Verify compilation error locus parsing, diff-mode hunk navigation, and rectangle/paren ops."
  (let ((tmpdir (make-temp-file "femacs-rb04-" t))
        (inhibit-message t))
    (unwind-protect
        (let ((src-a (expand-file-name "mod_a.c" tmpdir))
              (src-b (expand-file-name "mod_b.c" tmpdir))
              (comp-loci nil)
              (diff-hunks 0)
              (extracted-rect nil)
              (rect-after nil)
              (elec-result nil))
          (with-temp-file src-a
            (insert "int a1 = 0;\nint a2 = 1;\nint a3 = bad;\nint a4 = 3;\n"))
          (with-temp-file src-b
            (insert "int b1 = 0;\nint b2 = 1;\nint b3 = 2;\nint b4 = warn;\n"))

          ;; 1. compilation-mode error locus parsing
          (with-temp-buffer
            (setq default-directory (file-name-as-directory tmpdir))
            (insert "-*- mode: compilation; -*-\n"
                    "mod_a.c:3:10: error: undeclared identifier\n"
                    "mod_b.c:4:10: warning: unused variable\n")
            (compilation-mode)
            (goto-char (point-min))
            (compilation-next-error 1)
            (let* ((msg1 (get-text-property (point) 'compilation-message))
                   (loc1 (compilation--message->loc msg1)))
              (push (list (caar (compilation--loc->file-struct loc1))
                          (compilation--loc->line loc1)
                          (compilation--loc->col loc1))
                    comp-loci))
            (compilation-next-error 1)
            (let* ((msg2 (get-text-property (point) 'compilation-message))
                   (loc2 (compilation--message->loc msg2)))
              (push (list (caar (compilation--loc->file-struct loc2))
                          (compilation--loc->line loc2)
                          (compilation--loc->col loc2))
                    comp-loci))
            (setq comp-loci (nreverse comp-loci)))
          (should (equal comp-loci '(("mod_a.c" 3 10) ("mod_b.c" 4 10))))

          ;; 2. diff-mode hunk navigation
          (with-temp-buffer
            (insert "--- a/mod_a.c\n+++ b/mod_a.c\n"
                    "@@ -1,3 +1,3 @@\n int a1 = 0;\n-int a2 = 1;\n+int a2 = 2;\n"
                    "@@ -10,3 +10,3 @@\n int x = 0;\n-int y = 1;\n+int y = 2;\n")
            (diff-mode)
            (goto-char (point-min))
            (diff-hunk-next)
            (should (looking-at-p "@@ -1,3 \\+1,3 @@"))
            (setq diff-hunks (1+ diff-hunks))
            (diff-hunk-next)
            (should (looking-at-p "@@ -10,3 \\+10,3 @@"))
            (setq diff-hunks (1+ diff-hunks)))
          (should (= diff-hunks 2))

          ;; 3. Rectangle extraction/insertion and electric-pair / check-parens
          (with-temp-buffer
            (insert "aa11xx\nbb22yy\ncc33zz\n")
            (let ((start (save-excursion (goto-char (point-min)) (forward-char 2) (point)))
                  (end (save-excursion (goto-char (point-min)) (forward-line 2) (forward-char 4) (point))))
              (setq extracted-rect (delete-extract-rectangle start end))
              (goto-char (point-min))
              (insert-rectangle extracted-rect)
              (setq rect-after (buffer-string))))
          (should (equal extracted-rect '("11" "22" "33")))
        (should (equal rect-after "11aaxx\n22bbyy\n33cczz\n"))

          (with-temp-buffer
            (emacs-lisp-mode)
            (electric-pair-local-mode 1)
            (let ((last-command-event ?\())
              (self-insert-command 1))
            (insert "defun foo ")
            (let ((last-command-event ?\[))
              (self-insert-command 1))
            (insert "x")
            (setq elec-result (buffer-string))
            (check-parens)
            (goto-char (point-max))
            (insert "(")
            (should-error (scan-sexps (1- (point-max)) 1) :type 'scan-error)
            (should-error (check-parens) :type 'user-error))
          (should (equal elec-result "(defun foo [x])"))

          (femacs-rb--record
           'rb04-shipped-tooling-compile-diff-and-electric-pair
           (list :comp_loci (vconcat (mapcar #'vconcat comp-loci))
                 :diff_hunks diff-hunks
                 :extracted_rect (vconcat extracted-rect)
                 :elec_result elec-result)))
      (when (file-directory-p tmpdir)
        (delete-directory tmpdir t)))))

;;; Group 5: Unicode/multibyte coding systems, EOL conversion, and file round-trip

(ert-deftest rb05-unicode-coding-eol-and-file-roundtrip ()
  "Verify UTF-8 multibyte text, combining marks, emoji, EOL conversion, and disk persistence."
  (let ((tmpdir (make-temp-file "femacs-rb05-" t))
        (sample "ASCII | Café | 中文测试 | 日本語 | 한글 | e\u0301 | 🚀🦀\nLine-2: λ≈∑\n")
        (inhibit-message t))
    (unwind-protect
        (let ((unix-file (expand-file-name "sample-unix.txt" tmpdir))
              (dos-file (expand-file-name "sample-dos.txt" tmpdir))
              (char-len (length sample))
              (byte-len (string-bytes sample))
              (unix-raw-bytes nil)
              (dos-raw-bytes nil)
              (reloaded-unix nil)
              (reloaded-dos nil))
          (should (multibyte-string-p sample))
          (should (> byte-len char-len))

          (let ((coding-system-for-write 'utf-8-unix))
            (write-region sample nil unix-file nil 'silent))
          (let ((coding-system-for-write 'utf-8-dos))
            (write-region sample nil dos-file nil 'silent))

          ;; Verify raw byte difference (2 LF lines -> DOS file is exactly 2 bytes longer)
          (with-temp-buffer
            (set-buffer-multibyte nil)
            (insert-file-contents-literally unix-file)
            (setq unix-raw-bytes (buffer-size))
            (should (not (search-forward "\r\n" nil t))))
          (with-temp-buffer
            (set-buffer-multibyte nil)
            (insert-file-contents-literally dos-file)
            (setq dos-raw-bytes (buffer-size))
            (goto-char (point-min))
            (should (search-forward "\r\n" nil t)))
          (should (= (- dos-raw-bytes unix-raw-bytes) 2))

          ;; Reload both with UTF-8 decoding and verify identical logical character content
          (with-temp-buffer
            (let ((coding-system-for-read 'utf-8-unix))
              (insert-file-contents unix-file))
            (setq reloaded-unix (buffer-string)))
          (with-temp-buffer
            (let ((coding-system-for-read 'utf-8-dos))
              (insert-file-contents dos-file))
            (setq reloaded-dos (buffer-string)))
          (should (equal reloaded-unix sample))
          (should (equal reloaded-dos sample))

          (femacs-rb--record
           'rb05-unicode-coding-eol-and-file-roundtrip
           (list :char_len char-len
                 :byte_len byte-len
                 :unix_raw_bytes unix-raw-bytes
                 :dos_raw_bytes dos-raw-bytes
                 :equal_roundtrip (and (equal reloaded-unix sample)
                                       (equal reloaded-dos sample)
                                       t))))
      (when (file-directory-p tmpdir)
        (delete-directory tmpdir t)))))

;;; Group 6: Search, regex subgroup replacement, case-fold, and narrowing/widening

(ert-deftest rb06-search-regexp-replace-narrowing-and-case-fold ()
  "Verify search/replace, subgroup backreferences, case-fold-search, and narrow-to-region isolation."
  (with-temp-buffer
    (insert "HDR: key=alpha_01 val=10\n"
            "ROW: key=Beta_02 val=20\n"
            "ROW: key=gamma_03 val=30\n"
            "FTR: key=delta_04 val=40\n")
    ;; 1. Case-sensitive vs case-insensitive search
    (goto-char (point-min))
    (let ((case-fold-search nil))
      (should-not (search-forward "beta_02" nil t))
      (should (search-forward "Beta_02" nil t)))
    (goto-char (point-min))
    (let ((case-fold-search t))
      (should (search-forward "beta_02" nil t)))

    ;; 2. Narrow to middle two ROW lines and perform regex subgroup replacement
    (goto-char (point-min))
    (forward-line 1)
    (let ((region-start (point))
          (region-end (save-excursion (forward-line 2) (point)))
          (replaced-count 0)
          (narrowed-text nil)
          (widened-text nil))
      (save-restriction
        (narrow-to-region region-start region-end)
        (goto-char (point-min))
        (while (re-search-forward "key=\\([A-Za-z]+\\)_\\([0-9]+\\) val=\\([0-9]+\\)" nil t)
          (replace-match "id=\\2(\\1) score=\\3" t nil)
          (setq replaced-count (1+ replaced-count)))
        (setq narrowed-text (buffer-string)))
      (setq widened-text (buffer-string))
      (should (= replaced-count 2))
      (should (equal narrowed-text
                     "ROW: id=02(Beta) score=20\nROW: id=03(gamma) score=30\n"))
      (should (equal widened-text
                     (concat "HDR: key=alpha_01 val=10\n"
                             "ROW: id=02(Beta) score=20\n"
                             "ROW: id=03(gamma) score=30\n"
                             "FTR: key=delta_04 val=40\n")))
      (femacs-rb--record
       'rb06-search-regexp-replace-narrowing-and-case-fold
       (list :replaced_count replaced-count
             :narrowed_text narrowed-text
             :widened_text widened-text)))))

;;; Group 7: Undo/redo boundaries and atomic-change-group rollback

(ert-deftest rb07-undo-redo-boundaries-and-atomic-change-group ()
  "Verify step-by-step undo across undo-boundary and atomic-change-group rollback on error."
  (with-temp-buffer
    (buffer-enable-undo)
    (insert "step-0")
    (undo-boundary)
    (insert " -> step-1")
    (undo-boundary)
    (goto-char (point-min))
    (delete-char 6)
    (insert "STEP-0")
    (let ((state-after-edits (buffer-string))
          (undo-tail nil)
          (state-undo-1 nil)
          (state-undo-2 nil)
          (state-after-commit nil)
          (state-after-rollback nil)
          (rollback-caught nil))
      (should (equal state-after-edits "STEP-0 -> step-1"))
      ;; Undo last boundary (restores "step-0 -> step-1")
      (setq undo-tail (primitive-undo 1 buffer-undo-list))
      (setq state-undo-1 (buffer-string))
      (should (equal state-undo-1 "step-0 -> step-1"))
      ;; Undo previous boundary (restores "step-0")
      (primitive-undo 1 undo-tail)
      (setq state-undo-2 (buffer-string))
      (should (equal state-undo-2 "step-0"))

      ;; Successful atomic-change-group commits normally
      (atomic-change-group
        (goto-char (point-max))
        (insert " [committed]"))
      (setq state-after-commit (buffer-string))
      (should (equal state-after-commit "step-0 [committed]"))

      ;; Aborted atomic-change-group rolls back all intermediate mutations
      (setq rollback-caught
            (condition-case nil
                (atomic-change-group
                  (goto-char (point-min))
                  (insert "CORRUPTED-PREFIX ")
                  (goto-char (point-max))
                  (delete-char -5)
                  (error "Simulated mid-transaction failure"))
              (error 'rolled-back)))
      (setq state-after-rollback (buffer-string))
      (should (eq rollback-caught 'rolled-back))
      (should (equal state-after-rollback "step-0 [committed]"))
      (femacs-rb--record
       'rb07-undo-redo-boundaries-and-atomic-change-group
       (list :after_edits state-after-edits
             :undo_1 state-undo-1
             :undo_2 state-undo-2
             :after_commit state-after-commit
             :after_rollback state-after-rollback
             :rollback_status (symbol-name rollback-caught))))))

;;; Group 8: Markers, overlays, text properties, and registers

(ert-deftest rb08-markers-overlays-text-properties-and-registers ()
  "Verify marker insertion types, overlay shifts, text properties, and register operations."
  (with-temp-buffer
    (insert "alpha beta gamma")
    (let ((m-stay (copy-marker 7 nil))
          (m-advance (copy-marker 7 t))
          (ov (make-overlay 7 11)))
      (overlay-put ov 'category 'femacs-rb08-ov)
      (put-text-property 1 6 'face 'bold)
      (put-text-property 12 17 'femacs-tag 'tail-token)
      ;; Insert at position 7 (right at the start of "beta")
      (goto-char 7)
      (insert "NEW-")
      (let ((stay-pos (marker-position m-stay))
            (adv-pos (marker-position m-advance))
            (ov-start (overlay-start ov))
            (ov-end (overlay-end ov))
            (prop-at-1 (get-text-property 1 'face))
            (prop-change (next-single-property-change 1 'face))
            (tail-pos (next-single-property-change 1 'femacs-tag))
            (reg-text nil))
        (should (= stay-pos 7))
        (should (= adv-pos 11))
        (should (= ov-start 7))
        (should (= ov-end 15))
        (should (eq prop-at-1 'bold))
        (should (= prop-change 6))
        (should (= tail-pos 16))
        ;; Register copy/insert and numeric increment
        (copy-to-register ?r 7 15)
        (number-to-register 40 ?n)
        (increment-register 2 ?n)
        (goto-char (point-max))
        (insert " | ")
        (insert-register ?r t)
        (setq reg-text (buffer-substring-no-properties (point-min) (point-max)))
        (should (= (get-register ?n) 42))
        (should (equal reg-text "alpha NEW-beta gamma | NEW-beta"))
        (delete-overlay ov)
        (set-marker m-stay nil)
        (set-marker m-advance nil)
        (femacs-rb--record
         'rb08-markers-overlays-text-properties-and-registers
         (list :stay_pos stay-pos
               :adv_pos adv-pos
               :ov_range (vector ov-start ov-end)
               :prop_change prop-change
               :tail_pos tail-pos
               :reg_num (get-register ?n)
               :final_text reg-text))))))

;;; Group 9: Errors, condition-case hierarchy, unwind-protect, and catch/throw

(ert-deftest rb09-errors-condition-case-unwind-protect-and-catch-throw ()
  "Verify custom error inheritance, LIFO unwind-protect cleanup, and state restoration across nonlocal exits."
  (let ((events nil)
        (caught-err nil)
        (caught-tag nil))
    (with-temp-buffer
      (insert "line-1\nline-2\nline-3\n")
      (goto-char (point-min))
      (forward-line 1)
      (let ((saved-pt (point))
            (saved-buf (current-buffer)))
        ;; 1. Child error caught by base-error handler after inner unwind-protect
        (setq caught-err
              (condition-case err
                  (unwind-protect
                      (progn
                        (push 'try-body events)
                        (signal 'femacs-rb09-child-error '("payload" 99)))
                    (push 'cleanup-1 events))
                (femacs-rb09-base-error
                 (push 'handler-base events)
                 (list (car err) (cdr err)))))
        ;; 2. Non-local catch/throw across save-excursion, save-restriction, and temp buffer
        (setq caught-tag
              (catch 'femacs-rb09-escape
                (save-excursion
                  (save-restriction
                    (narrow-to-region (line-beginning-position) (line-end-position))
                    (goto-char (point-max))
                    (unwind-protect
                        (with-temp-buffer
                          (insert "ephemeral")
                          (push 'inside-temp events)
                          (throw 'femacs-rb09-escape 'escaped-cleanly))
                      (push 'cleanup-2 events))))))
        ;; Outer buffer, point, and widening must all be restored
        (should (eq (current-buffer) saved-buf))
        (should (= (point) saved-pt))
        (should (= (point-min) 1))
        (should (= (point-max) 22))
        (should (equal caught-err '(femacs-rb09-child-error ("payload" 99))))
        (should (eq caught-tag 'escaped-cleanly))
        (should (equal (reverse events)
                       '(try-body cleanup-1 handler-base inside-temp cleanup-2)))
        (femacs-rb--record
         'rb09-errors-condition-case-unwind-protect-and-catch-throw
         (list :events (vconcat (mapcar #'symbol-name (reverse events)))
               :err_sym (symbol-name (car caught-err))
               :err_data (vconcat (cadr caught-err))
               :caught_tag (symbol-name caught-tag)
               :restored_point (point)
               :restored_max (point-max)))))))

;;; Group 10: Active GC root preservation (closures, buffers, markers, weak/strong hashes)

(ert-deftest rb10-gc-live-roots-weak-hash-and-closures ()
  "Verify live closures, buffer-locals, markers, and strong/weak hash tables across active GC.
Conservative C-stack scanning may keep unreachable stack words alive within a frame, so this
test strongly asserts that all live roots (strong hash entries, live weak-table values,
lexical closures, buffer-local variables, and live markers) survive intact without corruption,
while recording ephemeral weak-entry reclamation as a diagnostic without skipping."
  (let* ((gc-cons-threshold 100000)
         (gc-cons-percentage 0.1)
         (gc-before gcs-done)
         (strong-ht (make-hash-table :test 'equal))
         (weak-ht (make-hash-table :test 'eq :weakness 'value))
         (retained-sym (make-symbol "retained-value-sym"))
         (closures nil))
    (puthash "persist-key" (list "中文-root" [10 20 30]) strong-ht)
    (puthash 'live-key retained-sym weak-ht)
    (puthash 'ephemeral-key (cons 'temp (make-string 64 ?x)) weak-ht)
    (dotimes (i 12)
      (let ((captured (list i (format "closure-%02d" i) (vector i (* i 10)))))
        (push (lambda (delta)
                (list (+ (car captured) delta)
                      (cadr captured)
                      (aref (nth 2 captured) 1)))
              closures)))
    (setq closures (nreverse closures))
    (with-temp-buffer
      (setq-local femacs-rb10-local-root '("buffer-local-alive" 777))
      (insert "marker-anchor-text")
      (let ((live-marker (copy-marker 8 t))
            (churn-sum 0))
        ;; Allocate transient objects under low gc-cons-threshold and force GC
        (dotimes (i 15000)
          (let ((transient (list i (make-vector 4 i) (format "churn-%d" i))))
            (setq churn-sum (+ churn-sum (aref (cadr transient) 0)))))
        (garbage-collect)
        (let ((gc-delta (- gcs-done gc-before))
              (closure-sample (funcall (nth 5 closures) 100))
              (heap-rows (garbage-collect-heapsize))
              (mem-counts (memory-use-counts))
              (ephemeral-reclaimed (null (gethash 'ephemeral-key weak-ht))))
          (should (> gc-delta 0))
          (should (= churn-sum 112492500))
          (should (equal (gethash "persist-key" strong-ht) '("中文-root" [10 20 30])))
          (should (eq (gethash 'live-key weak-ht) retained-sym))
          (should (equal closure-sample '(105 "closure-05" 50)))
          (should (equal femacs-rb10-local-root '("buffer-local-alive" 777)))
          (should (= (marker-position live-marker) 8))
          (should (>= (length heap-rows) 9))
          (should (= (length mem-counts) 7))
          (set-marker live-marker nil)
          (femacs-rb--record
           'rb10-gc-live-roots-weak-hash-and-closures
           (list :churn_sum churn-sum
                 :strong_val (vconcat (list (car (gethash "persist-key" strong-ht))
                                            (cadr (gethash "persist-key" strong-ht))))
                 :weak_live_preserved (eq (gethash 'live-key weak-ht) retained-sym)
                 :closure_sample (vconcat closure-sample)
                 :local_root (vconcat femacs-rb10-local-root)
                 :heap_categories (length heap-rows)
                 :mem_count_len (length mem-counts))
           (list :gc_delta gc-delta
                 :weak_ephemeral_reclaimed (if ephemeral-reclaimed t :false))))))))

(provide 'release-baseline)
;;; release-baseline.el ends here
