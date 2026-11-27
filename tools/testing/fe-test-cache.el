;;; fe-test-cache.el --- Content-addressed pass result cache for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Computes deterministic SHA-256 keys from command lines, environment overrides,
;; executable digests, and declared input digests via `xtask test-exec digest`.

;;; Code:

(require 'cl-lib)
(require 'fe-test-dsl)

(defconst fe-test-cache-max-entries 500
  "Maximum cached test records retained in `result-cache/'.")

(defun fe-test-cache-dir (state-dir)
  "Return the cache directory path under STATE-DIR."
  (expand-file-name "result-cache" state-dir))

(defun fe-test--digest-path (xtask-bin path memo)
  "Return cached SHA-256 hex digest for PATH using MEMO and XTASK-BIN."
  (let ((abs (expand-file-name path (fe-test-repo-root))))
    (or (gethash abs memo)
        (if (not (file-exists-p abs))
            (puthash abs "missing" memo)
          (let* ((res (fe-test-call-json xtask-bin (list "test-exec" "digest" abs)))
                 (hex (alist-get 'digest res)))
            (puthash abs hex memo))))))

(defun fe-test-cache-warm-digests (xtask-bin cases memo)
  "Batch-compute digests for all unique targets and inputs in CASES into MEMO."
  (let ((needed nil)
        (seen (make-hash-table :test #'equal)))
    (dolist (p (cons (fe-test-path "tools/testing")
                     (cl-loop for tc in cases
                              when (fe-test-case-cacheable tc)
                              append (cons (car (fe-test-case-command tc))
                                           (fe-test-case-cache-inputs tc)))))
      (when (and (stringp p) (> (length p) 0))
        (let ((abs (if (file-name-absolute-p p) p (fe-test-path p))))
          (when (and (file-exists-p abs)
                     (not (gethash abs memo))
                     (not (gethash abs seen))
                     ;; Do not hash generic system shells as directory trees; only files or repo dirs.
                     (or (file-regular-p abs)
                         (string-prefix-p (file-name-as-directory (fe-test-repo-root))
                                          (expand-file-name abs))))
            (puthash abs t seen)
            (push abs needed)))))
    (when needed
      (let* ((batch (nreverse needed))
             (res (fe-test-call-json xtask-bin (append (list "test-exec" "digest") batch)))
             (entries (alist-get 'entries res)))
        (dolist (entry entries)
          (let ((path (alist-get 'path entry))
                (sha (alist-get 'sha256 entry)))
            (when (and path sha)
              (puthash path sha memo))))))))

(defun fe-test-cache-key (tc xtask-bin memo)
  "Compute content-addressed cache key for TC using MEMO digests."
  (let* ((cmd (fe-test-case-command tc))
         (prog (car cmd))
         (prog-abs (if (file-name-absolute-p prog)
                       prog
                     (or (executable-find prog) (fe-test-path prog))))
         (prog-sha (if (and prog-abs (file-regular-p prog-abs))
                       (fe-test--digest-path xtask-bin prog-abs memo)
                     "system-exe"))
         (framework-sha (fe-test--digest-path xtask-bin (fe-test-path "tools/testing") memo))
         (inputs (mapcar (lambda (rel)
                           (list rel (fe-test--digest-path xtask-bin rel memo)))
                         (sort (copy-sequence (fe-test-case-cache-inputs tc)) #'string<)))
         (env-sorted (sort (copy-sequence (fe-test-case-env tc))
                           (lambda (a b) (string< (car a) (car b)))))
         (payload (list (cons 'schema "femacs-test-cache-v1")
                        (cons 'id (fe-test-case-id tc))
                        (cons 'command (vconcat cmd))
                        (cons 'cwd (or (fe-test-case-cwd tc) ""))
                        (cons 'pty (if (fe-test-case-pty tc) t :json-false))
                        (cons 'env (vconcat (mapcar (lambda (kv)
                                                      (vector (car kv) (or (cdr kv) "")))
                                                    env-sorted)))
                        (cons 'progSha prog-sha)
                        (cons 'frameworkSha framework-sha)
                        (cons 'inputs (vconcat (mapcar (lambda (pair)
                                                         (vector (car pair) (cadr pair)))
                                                       inputs))))))
    (secure-hash 'sha256 (json-encode payload))))

(defun fe-test-cache-lookup (state-dir key)
  "Return cached passed result alist for KEY under STATE-DIR, or nil."
  (let* ((file (expand-file-name (concat key ".json") (fe-test-cache-dir state-dir)))
         (data (fe-test-json-read-file file)))
    (when (and data
               (equal (alist-get 'schema data) "femacs-test-cache-v1")
               (equal (alist-get 'key data) key)
               (equal (alist-get 'status data) "passed"))
      data)))

(defun fe-test-cache-store (state-dir key result)
  "Store passed RESULT under KEY in STATE-DIR and enforce LRU cap."
  (let* ((dir (fe-test-cache-dir state-dir))
         (file (expand-file-name (concat key ".json") dir))
         (record (list (cons 'schema "femacs-test-cache-v1")
                       (cons 'key key)
                       (cons 'id (alist-get 'id result))
                       (cons 'suite (alist-get 'suite result))
                       (cons 'status "passed")
                       (cons 'storedAt (fe-test-now-iso8601))
                       (cons 'originalElapsedMs (alist-get 'elapsedMs result))
                       (cons 'peakRssBytes (alist-get 'peakRssBytes result))
                       (cons 'peakFootprintBytes (alist-get 'peakFootprintBytes result))
                       (cons 'detail (or (alist-get 'detail result) "")))))
    (fe-test-json-write-file file record)
    (let ((entries (directory-files dir t "\\.json\\'")))
      (when (> (length entries) fe-test-cache-max-entries)
        (let ((sorted (sort entries
                            (lambda (a b)
                              (time-less-p
                               (file-attribute-modification-time (file-attributes a))
                               (file-attribute-modification-time (file-attributes b))))))
              (excess (- (length entries) fe-test-cache-max-entries)))
          (dotimes (_ excess)
            (when sorted
              (ignore-errors (delete-file (pop sorted))))))))))

(provide 'fe-test-cache)
;;; fe-test-cache.el ends here
