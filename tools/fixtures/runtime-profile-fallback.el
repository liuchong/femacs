;;; runtime-profile-fallback.el --- Real workloads after permanent Full fallback -*- lexical-binding: t; -*-
;; Measurement input only. The failed load is the real runtime path, not a hook.
(require 'json)

(defun femacs-profile-fallback--status ()
  (when (fboundp 'femacs--gc-collector-status)
    (let* ((gc-cons-threshold most-positive-fixnum)
           (value (femacs--gc-collector-status)))
      (unless (and (vectorp value) (= (length value) 3) (= (aref value 0) 1)
                   (vectorp (aref value 1)) (= (length (aref value 1)) 24)
                   (vectorp (aref value 2)) (= (length (aref value 2)) 13))
        (error "Invalid real collector status in fallback performance mode"))
      value)))

(let* ((root (getenv "FEMACS_RUNTIME_TEST_ROOT"))
       (_root (unless (and root (> (length root) 0))
                (error "Owned runtime root is required")))
       (basename "nonexistent-performance-producer.so")
       (missing (expand-file-name basename root))
       (fixture (expand-file-name "runtime-profile.el"
                                  (file-name-directory load-file-name)))
       (before (femacs-profile-fallback--status))
       failure latched after)
  (unless (fboundp 'module-load)
    (error "Actual module-load support is required for this measurement"))
  (when (file-exists-p missing) (error "Fallback input must be an absent owned path"))
  (condition-case value
      (module-load missing)
    (module-open-failed (setq failure value)))
  (unless (and failure (equal (nth 1 failure) missing)
               (stringp (nth 2 failure)) (> (length (nth 2 failure)) 0))
    (error "The real module load did not produce its expected public error"))
  (setq latched (femacs-profile-fallback--status))
  (when latched
    (unless (and (= (aref (aref latched 1) 14) 1)
                 (= (aref (aref latched 1) 1) 0))
      (error "Experimental collector did not enter permanent invalid fallback")))
  ;; This is the unchanged actual workload fixture, not another benchmark body.
  (load fixture nil t t)
  (setq after (femacs-profile-fallback--status))
  (when after
    (unless (and (= (aref (aref after 1) 14) 1)
                 (= (aref (aref after 1) 1) 0)
                 (= (aref (aref after 1) 4) (aref (aref latched 1) 4)))
      (error "Permanent fallback changed or ran a Young collection")))
  (with-temp-file (expand-file-name "fallback-context.json" root)
    (insert
     (json-encode
      `((schema . "femacs-runtime-performance-fallback-v1")
        (module_error . ((symbol . "module-open-failed") (basename . ,basename)
                         (path_matched . t) (message_nonempty . t)))
        (before . ,before) (latched . ,latched) (after . ,after))))
    (insert "\n")))
;;; runtime-profile-fallback.el ends here
