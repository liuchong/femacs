;;; gio-stream-protection.el --- GIO startup stream protection -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

(require 'cl-lib)
(require 'ert)
(require 'json)
(require 'tramp-sh)

(defconst fe-gio--startup
  "Supported arguments for GIO_USE_FILE_MONITOR environment variable:\n kqueue - 20\n")

(cl-defmacro fe-gio--with-process ((proc events) &body body)
  "Run BODY with a real pipe process and a captured editor-event sink."
  (declare (indent 1))
  `(let* ((,proc (make-pipe-process :name "fe-gio-protection"
                                   :buffer nil :noquery t))
          (,events nil)
          (tramp-verbose 0))
     (unwind-protect
         (cl-letf (((symbol-function 'insert-special-event)
                    (lambda (event) (push event ,events))))
           (process-put ,proc 'tramp-events '(changed attribute-changed moved))
           ,@body)
       (tramp-flush-connection-properties ,proc)
       (when (process-live-p ,proc) (delete-process ,proc)))))

(ert-deftest fe-gio-captured-header-split ()
  (fe-gio--with-process (proc events)
    (tramp-sh-gio-monitor-process-filter
     proc "Supported arguments for GIO_USE_FILE_MONITOR environment variable:\n")
    (should-not (tramp-connection-property-p proc "file-monitor"))
    (tramp-sh-gio-monitor-process-filter proc " kqueue - 20\n")
    (should (eq (tramp-get-connection-property proc "file-monitor")
                'GKqueueFileMonitor))
    (should-not (process-get proc 'tramp-rest-string))
    (should-not events)))

(ert-deftest fe-gio-every-two-chunk-boundary ()
  (dotimes (offset (1+ (length fe-gio--startup)))
    (fe-gio--with-process (proc events)
      (tramp-sh-gio-monitor-process-filter proc (substring fe-gio--startup 0 offset))
      (tramp-sh-gio-monitor-process-filter proc (substring fe-gio--startup offset))
      (should (eq (tramp-get-connection-property proc "file-monitor")
                  'GKqueueFileMonitor))
      (should-not (process-get proc 'tramp-rest-string))
      (should-not events))))

(ert-deftest fe-gio-bytewise-startup ()
  (fe-gio--with-process (proc events)
    (dotimes (offset (length fe-gio--startup))
      (tramp-sh-gio-monitor-process-filter
       proc (substring fe-gio--startup offset (1+ offset))))
    (should (eq (tramp-get-connection-property proc "file-monitor")
                'GKqueueFileMonitor))
    (should-not (process-get proc 'tramp-rest-string))
    (should-not events)))

(ert-deftest fe-gio-complete-unknown-without-event ()
  (fe-gio--with-process (proc events)
    (tramp-sh-gio-monitor-process-filter proc "Unknown monitor startup\n")
    (should (tramp-connection-property-p proc "file-monitor"))
    (should-not (tramp-get-connection-property proc "file-monitor"))
    (should-not (process-get proc 'tramp-rest-string))
    (should-not events)))

(ert-deftest fe-gio-original-first-backend-selection ()
  ;; The original recognizer does not search later backend lines.  Preserve
  ;; that rule for both complete and split responses, rather than silently
  ;; inventing a different backend-selection policy in a stream fix.
  (let ((first (concat "Supported arguments for "
                       "GIO_USE_FILE_MONITOR environment variable:\n inotify - 30\n")))
    (dolist (chunks (list (list (concat first " kqueue - 20\n"))
                         (list first " kqueue - 20\n")))
      (fe-gio--with-process (proc events)
        (dolist (chunk chunks) (tramp-sh-gio-monitor-process-filter proc chunk))
        (should (tramp-connection-property-p proc "file-monitor"))
        (should-not (tramp-get-connection-property proc "file-monitor"))
        (should-not events)))))

(ert-deftest fe-gio-startup-and-normalized-event ()
  (dolist (chunks (list (list (concat fe-gio--startup
                                     "gio: probe-file: attributes changed\n"))
                       (list fe-gio--startup "gio: probe-file: attributes " "changed\n")))
    (fe-gio--with-process (proc events)
      (dolist (chunk chunks) (tramp-sh-gio-monitor-process-filter proc chunk))
      (should (equal (nreverse events)
                     (list (list 'file-notify
                                 (list proc '(attribute-changed) "probe-file" nil)
                                 'file-notify-callback))))
      (should-not (process-get proc 'tramp-rest-string)))))

(ert-deftest fe-gio-complete-unsupported-startup ()
  (dolist (message '("Monitoring not supported\n" "No locations given\n"
                     "Unable to find default local file monitor type\n"))
    (fe-gio--with-process (proc events)
      (tramp-sh-gio-monitor-process-filter proc message)
      (should-not (process-live-p proc))
      (should-not events))))

(ert-deftest fe-gio-split-unsupported-startup ()
  (fe-gio--with-process (proc events)
    (tramp-sh-gio-monitor-process-filter proc "Monitoring not supp")
    (should (process-live-p proc))
    (tramp-sh-gio-monitor-process-filter proc "orted\n")
    (should-not (process-live-p proc))
    (should-not events)))

(defun fe-gio-protection-run ()
  "Run protection cases and write per-case ERT results to FE_GIO_RESULT."
  (let ((rows nil)
        (selector (if (getenv "FE_GIO_SELECTOR")
                      (intern (getenv "FE_GIO_SELECTOR")) "^fe-gio-")))
    (ert-run-tests
     selector
     (lambda (event &rest args)
       (when (eq event 'test-ended)
         (let ((test (nth 1 args)) (result (nth 2 args)))
           (push (list (cons "id" (symbol-name (ert-test-name test)))
                       (cons "status" (cond ((ert-test-result-type-p result :passed) "passed")
                                             ((ert-test-result-type-p result :skipped) "skipped")
                                             (t "failed")))
                       (cons "seconds" (ert-test-result-duration result))
                       (cons "condition" (and (ert-test-result-with-condition-p result)
                                              (prin1-to-string
                                               (ert-test-result-with-condition-condition result)))))
                 rows)))))
    (setq rows (nreverse rows))
    (let* ((passed (and rows (cl-every (lambda (row) (equal (cdr (assoc "status" row)) "passed")) rows)))
           (coding-system-for-write 'utf-8-unix))
      (with-temp-file (or (getenv "FE_GIO_RESULT") (error "FE_GIO_RESULT is required"))
        (insert (json-encode (list (cons "passed" (if passed t :json-false))
                                  (cons "library" (symbol-file 'tramp-sh-gio-monitor-process-filter 'defun))
                                  (cons "cases" (vconcat rows)))) "\n"))
      (kill-emacs (if passed 0 1)))))
