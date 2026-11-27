;;; fe-test-scheduler.el --- Memory-aware LPT scheduler for fe-test -*- lexical-binding: t; -*-

;;; Commentary:
;; Dispatches test cases sorted in Longest Processing Time (LPT) order using
;; asynchronous `make-process` workers backed by `xtask test-exec supervise`,
;; gating launches via live process-tree RSS/footprint sampling (`mem-tree`).

;;; Code:

(require 'cl-lib)
(require 'fe-test-dsl)
(require 'fe-test-profile)
(require 'fe-test-cache)

(defconst fe-test-admission-ratio 0.78
  "Maximum fraction of memory budget used before pausing new launches.")

(defconst fe-test-cold-start-grace-ms 4000
  "Grace period in milliseconds reserving predicted memory for new workers.")

(defconst fe-test-poll-interval-sec 0.05
  "Event-loop poll interval in seconds while active workers run.")

(defvar fe-test-mem-sample-interval-ms 150
  "Minimum interval in milliseconds between live `mem-tree' polls.")

(cl-defstruct (fe-test--worker
               (:constructor fe-test--make-worker))
  (case nil)
  (proc nil)
  (buffer nil)
  (started-ms 0 :type integer)
  (est-mem 0 :type integer)
  (sampled nil :type boolean)
  (observed-mem 0 :type integer)
  (cache-key nil)
  (os-pid nil)
  (proc-status nil)
  (proc-exit nil)
  (cancelled nil :type boolean)
  (done nil :type boolean)
  (result nil))

(defun fe-test--build-supervise-argv (tc)
  "Build `xtask test-exec supervise ...' argument vector for TC."
  (let ((args (list "test-exec" "supervise"
                    "--timeout-ms" (number-to-string (fe-test-case-timeout-ms tc))
                    "--poll-ms" "40")))
    (when (> (fe-test-case-memory-limit-bytes tc) 0)
      (setq args (append args (list "--memory-limit-bytes"
                                    (number-to-string (fe-test-case-memory-limit-bytes tc))))))
    (when (fe-test-case-cwd tc)
      (setq args (append args (list "--cwd" (fe-test-case-cwd tc)))))
    (dolist (pair (fe-test-case-env tc))
      (if (cdr pair)
          (setq args (append args (list "--env" (format "%s=%s" (car pair) (cdr pair)))))
        (setq args (append args (list "--unset-env" (car pair))))))
    (when (fe-test-case-pty tc)
      (setq args (append args (list "--pty"))))
    (append args (list "--") (fe-test-case-command tc))))

(defun fe-test--evaluate-supervise-output (tc raw-stdout elapsed-fallback-ms proc-status proc-exit)
  "Parse `femacs-test-exec-v1' JSON from RAW-STDOUT and validate receipt for TC."
  (condition-case err
      (let* ((supervisor-ok (and (eq proc-status 'exit) (eql proc-exit 0)))
             (trimmed (string-trim (or raw-stdout "")))
             (lines (split-string trimmed "\n" t))
             (last-line (car (last lines)))
             (exec (and supervisor-ok
                        last-line
                        (json-parse-string last-line
                                           :object-type 'alist
                                           :array-type 'list
                                           :null-object nil
                                           :false-object nil)))
             (schema (and exec (alist-get 'schema exec)))
             (exit-code (and exec (alist-get 'exit_code exec)))
             (timed-out (and exec (alist-get 'timed_out exec)))
             (mem-exceeded (and exec (alist-get 'memory_exceeded exec)))
             (children-reaped (and exec (eq (alist-get 'children_reaped exec) t)))
             (elapsed (or (and exec (alist-get 'elapsed_ms exec)) elapsed-fallback-ms))
             (rss (or (and exec (alist-get 'peak_rss_bytes exec)) 0))
             (fp (or (and exec (alist-get 'peak_footprint_bytes exec)) 0))
             (check-fn (fe-test-case-check-fn tc))
             (verdict (cond
                       ((not supervisor-ok)
                        (list nil (format "supervisor failed (status=%s exit=%s)" proc-status proc-exit)))
                       ((null exec) (list nil "missing femacs-test-exec-v1 output"))
                       ((not (equal schema "femacs-test-exec-v1"))
                        (list nil (format "unexpected supervisor schema: %S" schema)))
                       (timed-out (list nil (format "timed out after %dms" elapsed)))
                       (mem-exceeded (list nil (format "memory limit exceeded (%dB)" (max rss fp))))
                       ((not children-reaped)
                        (list nil "child process tree or output pipes not cleanly reaped (children_reaped=false)"))
                       (check-fn (funcall check-fn exec))
                       ((eql exit-code 0) (list t "ok"))
                       (t (list nil (format "exit_code=%s" exit-code)))))
             (ok (car verdict))
             (detail (or (cadr verdict) (if ok "ok" "failed"))))
        (list (cons 'id (fe-test-case-id tc))
              (cons 'suite (fe-test-case-suite tc))
              (cons 'title (fe-test-case-title tc))
              (cons 'status (if ok "passed" "failed"))
              (cons 'cached nil)
              (cons 'elapsedMs elapsed)
              (cons 'exitCode exit-code)
              (cons 'timedOut timed-out)
              (cons 'memoryExceeded mem-exceeded)
              (cons 'childrenReaped children-reaped)
              (cons 'supervisorOk supervisor-ok)
              (cons 'peakRssBytes rss)
              (cons 'peakFootprintBytes fp)
              (cons 'detail detail)
              (cons 'stdout (and exec (alist-get 'stdout exec)))
              (cons 'stderr (and exec (alist-get 'stderr exec)))))
    (error
     (list (cons 'id (fe-test-case-id tc))
           (cons 'suite (fe-test-case-suite tc))
           (cons 'title (fe-test-case-title tc))
           (cons 'status "failed")
           (cons 'cached nil)
           (cons 'elapsedMs elapsed-fallback-ms)
           (cons 'exitCode nil)
           (cons 'timedOut nil)
           (cons 'memoryExceeded nil)
           (cons 'childrenReaped nil)
           (cons 'supervisorOk nil)
           (cons 'peakRssBytes 0)
           (cons 'peakFootprintBytes 0)
           (cons 'detail (format "malformed supervisor receipt: %s (%s)"
                                 (error-message-string err)
                                 (string-trim (or raw-stdout ""))))
           (cons 'stdout "")
           (cons 'stderr raw-stdout)))))

(defun fe-test--sort-lpt (cases profile-table)
  "Return CASES sorted descending by predicted elapsed time, then ID."
  (sort (copy-sequence cases)
        (lambda (a b)
          (let ((ta (fe-test-profile-predict-ms profile-table a))
                (tb (fe-test-profile-predict-ms profile-table b)))
            (if (/= ta tb)
                (> ta tb)
              (string< (fe-test-case-id a) (fe-test-case-id b)))))))

(defun fe-test--compute-projected-bytes (workers now-ms)
  "Compute conservative memory projection across WORKERS at NOW-MS.
Retains prior sampled observations and full estimated reservations for
workers that have not yet been sampled or remain inside cold-start grace."
  (let ((total 0))
    (dolist (w workers)
      (let* ((age (max 0 (- now-ms (fe-test--worker-started-ms w))))
             (est (fe-test--worker-est-mem w))
             (obs (fe-test--worker-observed-mem w)))
        (setq total
              (+ total
                 (cond
                  ((not (fe-test--worker-sampled w)) est)
                  ((< age fe-test-cold-start-grace-ms) (max obs est))
                  (t obs))))))
    total))

(defun fe-test--refresh-active-memory! (xtask-bin workers)
  "Sample live process-tree memory via `xtask test-exec mem-tree' into WORKERS.
Requires a valid `femacs-test-mem-v1' schema and numeric `accounted_bytes' row
for each live worker; preserves prior samples or reservations when a snapshot
fails or omits a worker's PID row."
  (let ((pids nil))
    (dolist (w workers)
      (when (and (fe-test--worker-os-pid w)
                 (process-live-p (fe-test--worker-proc w)))
        (push (number-to-string (fe-test--worker-os-pid w)) pids)))
    (when pids
      (let ((mem (condition-case nil
                     (fe-test-call-json xtask-bin (cons "test-exec" (cons "mem-tree" (nreverse pids))))
                   (error nil))))
        (when (and mem (equal (alist-get 'schema mem) "femacs-test-mem-v1"))
          (let ((live-total (alist-get 'total_accounted_bytes mem))
                (per-pid (alist-get 'per_pid mem)))
            (when (listp per-pid)
              (dolist (w workers)
                (when (and (fe-test--worker-os-pid w)
                           (process-live-p (fe-test--worker-proc w)))
                  (let* ((pid-sym (intern (number-to-string (fe-test--worker-os-pid w))))
                         (row (alist-get pid-sym per-pid))
                         (acc (and row (alist-get 'accounted_bytes row))))
                    (when (and row (numberp acc))
                      (setf (fe-test--worker-sampled w) t)
                      (setf (fe-test--worker-observed-mem w) acc))))))
            (and (numberp live-total) live-total)))))))

(defun fe-test--pick-admissible (pending active heavy-workers profile-table
                                          live-bytes ceiling max-workers)
  "Pick next admissible case index from PENDING given active state, or nil."
  (let ((active-count (length active))
        (active-heavy (cl-count-if (lambda (w) (fe-test-case-heavy (fe-test--worker-case w)))
                                   active)))
    (when (< active-count max-workers)
      (cl-loop for tc in pending
               for idx from 0
               for is-heavy = (fe-test-case-heavy tc)
               for est = (fe-test-profile-predict-mem-bytes profile-table tc)
               when (and (or (not is-heavy) (< active-heavy heavy-workers))
                         (or (= active-count 0)
                             (<= (+ live-bytes est) ceiling)))
               return idx))))

(defun fe-test-run-schedule (cases options profile-table)
  "Execute CASES under OPTIONS using PROFILE-TABLE; return results plist."
  (let* ((xtask-bin (plist-get options :xtask-bin))
         (state-dir (plist-get options :state-dir))
         (use-cache (plist-get options :cache))
         (keep-going (plist-get options :keep-going))
         (max-workers (max 1 (or (plist-get options :workers) 4)))
         (heavy-workers (max 1 (or (plist-get options :heavy-workers) 2)))
         (budget-bytes (* (max 256 (or (plist-get options :budget-mib) 2048)) 1024 1024))
         (ceiling (truncate (* fe-test-admission-ratio (float budget-bytes))))
         (digest-memo (make-hash-table :test #'equal))
         (pending nil)
         (results nil)
         (active nil)
         (aborting nil)
         (admission-pauses 0)
         (peak-agg 0)
         (last-mem-check-ms 0)
         (paused-last-tick nil))

    (make-directory state-dir t)
    (make-directory (expand-file-name "tmp" state-dir) t)

    (when use-cache
      (fe-test-cache-warm-digests xtask-bin cases digest-memo))

    ;; Partition cached hits from runnable queue (in LPT order for uncached cases).
    (let ((to-run nil))
      (dolist (tc cases)
        (let* ((key (and use-cache (fe-test-case-cacheable tc)
                         (fe-test-cache-key tc xtask-bin digest-memo)))
               (hit (and key (fe-test-cache-lookup state-dir key))))
          (if hit
              (push (list (cons 'id (fe-test-case-id tc))
                          (cons 'suite (fe-test-case-suite tc))
                          (cons 'title (fe-test-case-title tc))
                          (cons 'status "passed")
                          (cons 'cached t)
                          (cons 'elapsedMs 0)
                          (cons 'exitCode 0)
                          (cons 'timedOut nil)
                          (cons 'memoryExceeded nil)
                          (cons 'childrenReaped t)
                          (cons 'supervisorOk t)
                          (cons 'peakRssBytes (or (alist-get 'peakRssBytes hit) 0))
                          (cons 'peakFootprintBytes (or (alist-get 'peakFootprintBytes hit) 0))
                          (cons 'detail (format "cached (orig %dms)"
                                                (or (alist-get 'originalElapsedMs hit) 0))))
                    results)
            (push (cons tc key) to-run))))
      (let* ((sorted-tcs (fe-test--sort-lpt (mapcar #'car to-run) profile-table))
             (key-map (make-hash-table :test #'equal)))
        (dolist (pair to-run)
          (puthash (fe-test-case-id (car pair)) (cdr pair) key-map))
        (setq pending (mapcar (lambda (tc) (cons tc (gethash (fe-test-case-id tc) key-map)))
                              sorted-tcs))))

    (cl-labels
        ((launch-item (item)
           (let* ((tc (car item))
                  (ckey (cdr item))
                  (buf (generate-new-buffer (format " *fe-test-%s*" (fe-test-case-id tc))))
                  (argv (fe-test--build-supervise-argv tc))
                  (est (fe-test-profile-predict-mem-bytes profile-table tc))
                  (worker (fe-test--make-worker
                           :case tc
                           :buffer buf
                           :started-ms (fe-test-now-ms)
                           :est-mem est
                           :cache-key ckey))
                  (proc (let ((process-connection-type nil)
                              (default-directory (file-name-as-directory (fe-test-repo-root))))
                          (make-process
                           :name (format "fe-test:%s" (fe-test-case-id tc))
                           :buffer buf
                           :command (cons xtask-bin argv)
                           :connection-type 'pipe
                           :coding 'utf-8-unix
                           :noquery t
                           :sentinel (lambda (p _event)
                                       (unless (process-live-p p)
                                         (setf (fe-test--worker-proc-status worker) (process-status p))
                                         (setf (fe-test--worker-proc-exit worker) (process-exit-status p))
                                         (setf (fe-test--worker-done worker) t)))))))
             (setf (fe-test--worker-proc worker) proc)
             (setf (fe-test--worker-os-pid worker) (process-id proc))
             (push worker active)))

         (collect-finished ()
           (let ((still-active nil))
             (dolist (w active)
               (let ((proc (fe-test--worker-proc w)))
                 (when (and (not (fe-test--worker-done w))
                            (not (process-live-p proc)))
                   (setf (fe-test--worker-proc-status w) (process-status proc))
                   (setf (fe-test--worker-proc-exit w) (process-exit-status proc))
                   (setf (fe-test--worker-done w) t))
                 (when (and (fe-test--worker-done w) (null (fe-test--worker-proc-status w)))
                   (setf (fe-test--worker-proc-status w) (process-status proc))
                   (setf (fe-test--worker-proc-exit w) (process-exit-status proc)))
                 (if (fe-test--worker-done w)
                     (let* ((tc (fe-test--worker-case w))
                            (buf (fe-test--worker-buffer w))
                            (raw (if (buffer-live-p buf)
                                     (with-current-buffer buf (buffer-string))
                                   ""))
                            (fallback-ms (max 0 (- (fe-test-now-ms)
                                                   (fe-test--worker-started-ms w))))
                            (res (unwind-protect
                                     (if (fe-test--worker-cancelled w)
                                         (list (cons 'id (fe-test-case-id tc))
                                               (cons 'suite (fe-test-case-suite tc))
                                               (cons 'title (fe-test-case-title tc))
                                               (cons 'status "cancelled")
                                               (cons 'cached nil)
                                               (cons 'elapsedMs fallback-ms)
                                               (cons 'exitCode nil)
                                               (cons 'timedOut nil)
                                               (cons 'memoryExceeded nil)
                                               (cons 'childrenReaped nil)
                                               (cons 'supervisorOk nil)
                                               (cons 'peakRssBytes 0)
                                               (cons 'peakFootprintBytes 0)
                                               (cons 'detail "cancelled due to fail-fast abort")
                                               (cons 'stdout "")
                                               (cons 'stderr ""))
                                       (fe-test--evaluate-supervise-output
                                        tc raw fallback-ms
                                        (fe-test--worker-proc-status w)
                                        (fe-test--worker-proc-exit w)))
                                   (when (buffer-live-p buf)
                                     (kill-buffer buf)))))
                       (push res results)
                       (let ((single-peak (max (or (alist-get 'peakRssBytes res) 0)
                                               (or (alist-get 'peakFootprintBytes res) 0))))
                         (when (> single-peak peak-agg)
                           (setq peak-agg single-peak)))
                       (when (and use-cache
                                  (fe-test--worker-cache-key w)
                                  (equal (alist-get 'status res) "passed")
                                  (eq (alist-get 'supervisorOk res) t)
                                  (eq (alist-get 'childrenReaped res) t))
                         (fe-test-cache-store state-dir (fe-test--worker-cache-key w) res))
                       (when (and (equal (alist-get 'status res) "failed")
                                  (not keep-going))
                         (setq aborting t)))
                   (push w still-active))))
             (setq active (nreverse still-active)))))

      (while (or active (and pending (not aborting)))
        (collect-finished)

        (when (and aborting active)
          (dolist (w active)
            (when (process-live-p (fe-test--worker-proc w))
              (setf (fe-test--worker-cancelled w) t)
              (ignore-errors (signal-process (fe-test--worker-proc w) 'TERM))))
          (let ((term-deadline (+ (float-time) 0.8)))
            (while (and (cl-some (lambda (w) (process-live-p (fe-test--worker-proc w))) active)
                        (< (float-time) term-deadline))
              (accept-process-output nil 0.05)))
          (dolist (w active)
            (when (process-live-p (fe-test--worker-proc w))
              (ignore-errors (kill-process (fe-test--worker-proc w)))))
          (accept-process-output nil 0.05)
          (collect-finished))

        (let ((now-ms (fe-test-now-ms)))
          (when (and active (not aborting)
                     (or (= last-mem-check-ms 0)
                         (>= (- now-ms last-mem-check-ms) fe-test-mem-sample-interval-ms)))
            (setq last-mem-check-ms now-ms)
            (let ((live-total (fe-test--refresh-active-memory! xtask-bin active)))
              (when (and (numberp live-total) (> live-total peak-agg))
                (setq peak-agg live-total)))))

        (when (and pending (not aborting) (< (length active) max-workers))
          (let ((launched-any nil)
                (keep-launching t))
            (while (and keep-launching pending (< (length active) max-workers) (not aborting))
              (let* ((now-ms (fe-test-now-ms))
                     (live-for-pick (if (null active)
                                        0
                                      (fe-test--compute-projected-bytes active now-ms)))
                     (idx (fe-test--pick-admissible
                           (mapcar #'car pending)
                           active heavy-workers profile-table
                           live-for-pick ceiling max-workers)))
                (if idx
                    (let ((chosen (nth idx pending)))
                      (setq pending (append (cl-subseq pending 0 idx)
                                            (cl-subseq pending (1+ idx))))
                      (launch-item chosen)
                      (setq launched-any t)
                      (setq paused-last-tick nil))
                  (unless (or launched-any paused-last-tick (null pending) (null active))
                    (setq admission-pauses (1+ admission-pauses))
                    (setq paused-last-tick t))
                  (setq keep-launching nil))))))

        (when (or active (and pending (not aborting)))
          (accept-process-output nil fe-test-poll-interval-sec)))

      (dolist (item pending)
        (let ((tc (car item)))
          (push (list (cons 'id (fe-test-case-id tc))
                      (cons 'suite (fe-test-case-suite tc))
                      (cons 'title (fe-test-case-title tc))
                      (cons 'status "unstarted")
                      (cons 'cached nil)
                      (cons 'elapsedMs 0)
                      (cons 'detail "unstarted due to fail-fast abort"))
                results))))

    (list :results (nreverse results)
          :peak-aggregate-bytes peak-agg
          :admission-pauses admission-pauses)))

(provide 'fe-test-scheduler)
;;; fe-test-scheduler.el ends here
