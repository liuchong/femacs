;;; fe-test-profile.el --- Persistent EWMA timings & failure ledger -*- lexical-binding: t; -*-

;;; Commentary:
;; Reads and updates `profile.json` (`femacs-test-profile-v1`) using an
;; exponential weighted moving average (`0.7 * latest + 0.3 * prev`), plus
;; `last-failures.json` (`femacs-test-failures-v1`) for `--rerun-failed`.

;;; Code:

(require 'cl-lib)
(require 'fe-test-dsl)

(defun fe-test-profile-path (state-dir)
  "Return the persistent profile JSON path under STATE-DIR."
  (expand-file-name "profile.json" state-dir))

(defun fe-test-failures-path (state-dir)
  "Return the persistent failure-ledger JSON path under STATE-DIR."
  (expand-file-name "last-failures.json" state-dir))

(defun fe-test-ewma (prev latest)
  "Blend integer PREV and LATEST measurements using 0.7 / 0.3 weights."
  (if (and (numberp prev) (> prev 0))
      (max 1 (round (+ (* 0.7 (float latest)) (* 0.3 (float prev)))))
    (max 0 (round (float latest)))))

(defun fe-test-profile-load (state-dir)
  "Load existing profile map (`id -> alist`) from STATE-DIR."
  (let ((table (make-hash-table :test #'equal))
        (data (fe-test-json-read-file (fe-test-profile-path state-dir))))
    (when (and data (equal (alist-get 'format data) "femacs-test-profile-v1"))
      (let ((files (alist-get 'files data)))
        (dolist (entry files)
          (let ((key (if (symbolp (car entry)) (symbol-name (car entry)) (car entry))))
            (puthash key (cdr entry) table)))))
    table))

(defun fe-test-profile-predict-ms (profile-table tc)
  "Return predicted duration in ms for TC from PROFILE-TABLE."
  (let* ((rec (gethash (fe-test-case-id tc) profile-table))
         (ms (and rec (alist-get 'elapsedMs rec))))
    (cond
     ((and (numberp ms) (> ms 0)) ms)
     ((fe-test-case-heavy tc) 5000)
     (t 1000))))

(defun fe-test-profile-predict-mem-bytes (profile-table tc)
  "Return predicted peak footprint/RSS bytes for TC from PROFILE-TABLE."
  (let* ((rec (gethash (fe-test-case-id tc) profile-table))
         (fp (and rec (alist-get 'peakFootprintBytes rec)))
         (rss (and rec (alist-get 'peakRssBytes rec)))
         (obs (max (or fp 0) (or rss 0))))
    (cond
     ((> obs (* 16 1024 1024)) obs)
     ((fe-test-case-heavy tc) (* 256 1024 1024))
     (t (* 96 1024 1024)))))

(defun fe-test-profile-update (state-dir profile-table results)
  "Update PROFILE-TABLE with non-cached RESULTS and persist atomically."
  (let ((now (fe-test-now-iso8601)))
    (dolist (res results)
      (let* ((id (alist-get 'id res))
             (cached (alist-get 'cached res))
             (status (alist-get 'status res)))
        (unless (or cached (equal status "unstarted") (equal status "cancelled"))
          (let* ((prev (gethash id profile-table))
                 (prev-runs (or (and prev (alist-get 'runs prev)) 0))
                 (prev-ms (and prev (alist-get 'elapsedMs prev)))
                 (prev-rss (and prev (alist-get 'peakRssBytes prev)))
                 (prev-fp (and prev (alist-get 'peakFootprintBytes prev)))
                 (cur-ms (or (alist-get 'elapsedMs res) 0))
                 (cur-rss (or (alist-get 'peakRssBytes res) 0))
                 (cur-fp (or (alist-get 'peakFootprintBytes res) 0)))
            (puthash id
                     (list (cons 'runs (1+ prev-runs))
                           (cons 'status status)
                           (cons 'updatedAt now)
                           (cons 'elapsedMs (fe-test-ewma prev-ms cur-ms))
                           (cons 'peakRssBytes (fe-test-ewma prev-rss cur-rss))
                           (cons 'peakFootprintBytes (fe-test-ewma prev-fp cur-fp)))
                     profile-table))))))
  (let ((keys nil)
        (entries nil))
    (maphash (lambda (k _v) (push k keys)) profile-table)
    (setq keys (sort keys #'string<))
    (dolist (k keys)
      (push (cons k (gethash k profile-table)) entries))
    (fe-test-json-write-file
     (fe-test-profile-path state-dir)
     (list (cons 'format "femacs-test-profile-v1")
           (cons 'updatedAt (fe-test-now-iso8601))
           (cons 'files (if entries
                            (nreverse entries)
                          (make-hash-table :test #'equal)))))))

(defun fe-test-failures-load (state-dir)
  "Return list of previously failed test IDs from STATE-DIR."
  (let ((data (fe-test-json-read-file (fe-test-failures-path state-dir))))
    (if (and data (equal (alist-get 'format data) "femacs-test-failures-v1"))
        (let ((items (alist-get 'failedTests data)))
          (if (listp items) items nil))
      nil)))

(defun fe-test-failures-save (state-dir suite-name failed-ids)
  "Persist FAILED-IDS for SUITE-NAME to STATE-DIR."
  (fe-test-json-write-file
   (fe-test-failures-path state-dir)
   (list (cons 'format "femacs-test-failures-v1")
         (cons 'suite suite-name)
         (cons 'updatedAt (fe-test-now-iso8601))
         (cons 'failedTests (vconcat failed-ids)))))

(provide 'fe-test-profile)
;;; fe-test-profile.el ends here
