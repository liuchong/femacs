;;; fe-test-reporter.el --- Report writer & console summary for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Emits `latest-report.json` (`femacs-test-report-v1`) and prints a concise
;; human-readable summary including parallel speedup, peak memory, slowest cases,
;; and failure diagnostics.

;;; Code:

(require 'cl-lib)
(require 'fe-test-dsl)

(defun fe-test--report-case-json (res)
  "Normalize a result alist RES for JSON report emission."
  (list (cons 'id (alist-get 'id res))
        (cons 'suite (alist-get 'suite res))
        (cons 'title (or (alist-get 'title res) ""))
        (cons 'status (alist-get 'status res))
        (cons 'cached (if (alist-get 'cached res) t :json-false))
        (cons 'elapsedMs (or (alist-get 'elapsedMs res) 0))
        (cons 'exitCode (alist-get 'exitCode res))
        (cons 'timedOut (if (alist-get 'timedOut res) t :json-false))
        (cons 'memoryExceeded (if (alist-get 'memoryExceeded res) t :json-false))
        (cons 'childrenReaped (if (eq (alist-get 'childrenReaped res) t) t :json-false))
        (cons 'peakRssBytes (or (alist-get 'peakRssBytes res) 0))
        (cons 'peakFootprintBytes (or (alist-get 'peakFootprintBytes res) 0))
        (cons 'detail (or (alist-get 'detail res) ""))))

(defun fe-test-write-report (report-path suite-name wall-ms summary results)
  "Write structured report to REPORT-PATH and return the report alist."
  (let* ((passed (cl-count-if (lambda (r) (equal (alist-get 'status r) "passed")) results))
         (failed-rows (cl-remove-if-not (lambda (r) (equal (alist-get 'status r) "failed")) results))
         (failed (length failed-rows))
         (cached (cl-count-if (lambda (r) (alist-get 'cached r)) results))
         (unstarted (cl-count-if (lambda (r) (equal (alist-get 'status r) "unstarted")) results))
         (cancelled (cl-count-if (lambda (r) (equal (alist-get 'status r) "cancelled")) results))
         (case-sum-ms (cl-loop for r in results sum (or (alist-get 'elapsedMs r) 0)))
         (concurrency (if (> wall-ms 0)
                          (/ (float (round (* (/ (float case-sum-ms) wall-ms) 100.0))) 100.0)
                        1.0))
         (peak-agg (or (plist-get summary :peak-aggregate-bytes) 0))
         (peak-mib (/ (float (round (* (/ (float peak-agg) (* 1024.0 1024.0)) 100.0))) 100.0))
         (pauses (or (plist-get summary :admission-pauses) 0))
         (slowest-raw (cl-subseq
                       (sort (copy-sequence results)
                             (lambda (a b)
                               (> (or (alist-get 'elapsedMs a) 0)
                                  (or (alist-get 'elapsedMs b) 0))))
                       0 (min 5 (length results))))
         (failed-ids (mapcar (lambda (r) (alist-get 'id r)) failed-rows))
         (report (list (cons 'format "femacs-test-report-v1")
                       (cons 'suite suite-name)
                       (cons 'generatedAt (fe-test-now-iso8601))
                       (cons 'total (length results))
                       (cons 'passed passed)
                       (cons 'failed failed)
                       (cons 'cached cached)
                       (cons 'unstarted unstarted)
                       (cons 'cancelled cancelled)
                       (cons 'wallMs wall-ms)
                       (cons 'cumulativeCaseMs case-sum-ms)
                       (cons 'observedConcurrency concurrency)
                       (cons 'peakAggregateRssBytes peak-agg)
                       (cons 'peakAggregateRssMiB peak-mib)
                       (cons 'admissionPauses pauses)
                       (cons 'slowest
                             (vconcat (mapcar (lambda (r)
                                                (list (cons 'id (alist-get 'id r))
                                                      (cons 'elapsedMs (or (alist-get 'elapsedMs r) 0))
                                                      (cons 'cached (if (alist-get 'cached r) t :json-false))))
                                              slowest-raw)))
                       (cons 'failedTests (vconcat failed-ids))
                       (cons 'cases (vconcat (mapcar #'fe-test--report-case-json results))))))
    (fe-test-json-write-file report-path report)
    report))

(defun fe-test--tail-text (text max-chars)
  "Return last MAX-CHARS of TEXT with single-line whitespace normalization."
  (if (or (null text) (string-empty-p text))
      ""
    (let* ((len (length text))
           (slice (if (> len max-chars)
                      (concat "..." (substring text (- len max-chars)))
                    text)))
      (string-trim slice))))

(defun fe-test-print-summary (report results report-path)
  "Print concise console summary of REPORT and failure details from RESULTS."
  (let ((suite (alist-get 'suite report))
        (total (alist-get 'total report))
        (passed (alist-get 'passed report))
        (failed (alist-get 'failed report))
        (cached (alist-get 'cached report))
        (unstarted (alist-get 'unstarted report))
        (cancelled (alist-get 'cancelled report))
        (wall-ms (alist-get 'wallMs report))
        (case-sum-ms (alist-get 'cumulativeCaseMs report))
        (concurrency (alist-get 'observedConcurrency report))
        (peak-mib (alist-get 'peakAggregateRssMiB report))
        (pauses (alist-get 'admissionPauses report))
        (slowest (alist-get 'slowest report)))
    (princ (format "\n== fe-test summary (%s) ==\n" suite))
    (princ (format "total=%d passed=%d failed=%d cached=%d unstarted=%d cancelled=%d\n"
                   total passed failed cached unstarted cancelled))
    (princ (format "wall=%.3fs caseSum=%.3fs concurrency=%.2fx peakMem=%.2f MiB admissionPauses=%d\n"
                   (/ wall-ms 1000.0) (/ case-sum-ms 1000.0) concurrency peak-mib pauses))
    (when slowest
      (princ "slowest:\n")
      (dolist (row (if (vectorp slowest) (append slowest nil) slowest))
        (princ (format "  - %s: %dms%s\n"
                       (alist-get 'id row)
                       (alist-get 'elapsedMs row)
                       (if (eq (alist-get 'cached row) t) " (cached)" "")))))
    (dolist (r results)
      (when (equal (alist-get 'status r) "failed")
        (princ (format "\nFAIL %s (%s)\n"
                       (alist-get 'id r)
                       (or (alist-get 'detail r) "error")))
        (let ((out (fe-test--tail-text (alist-get 'stdout r) 500))
              (err (fe-test--tail-text (alist-get 'stderr r) 500)))
          (unless (string-empty-p out)
            (princ (format "  stdout: %s\n" out)))
          (unless (string-empty-p err)
            (princ (format "  stderr: %s\n" err))))))
    (princ (format "report: %s\n" report-path))))

(provide 'fe-test-reporter)
;;; fe-test-reporter.el ends here
