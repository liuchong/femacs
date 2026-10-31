;;; stable-suite-event-contract.el --- bridge state contracts -*- lexical-binding: t; -*-

(require 'ert)
(require 'cl-lib)

(defun stable-event-contract--emit (payload)
  "Call the real event writer with a separate owned component output."
  (let ((process-environment (copy-sequence process-environment)))
    (setenv "STABLE_SUITE_PROGRESS"
            (expand-file-name "event-contract.jsonl"
                              (getenv "FEMACS_RUNTIME_OUT")))
    (stable-suite--event payload)))

(ert-deftest stable-event-contract-preserves-nil-command-flag ()
  (with-temp-buffer
    (let ((undo-auto--this-command-amalgamating nil)
          (undo-auto--last-boundary-cause '(command)))
      ;; Map separators and escaped characters exercise delete-char in the
      ;; real JSON encoder, rather than a replacement serializer.
      (stable-event-contract--emit
       '(("first" . "quote\"newline\n") ("second" . 2)))
      (should-not undo-auto--this-command-amalgamating)
      (should (equal undo-auto--last-boundary-cause '(command))))))

(ert-deftest stable-event-contract-preserves-true-command-flag ()
  (with-temp-buffer
    (let ((undo-auto--this-command-amalgamating t)
          (undo-auto--last-boundary-cause '(timer)))
      (stable-event-contract--emit '(("first" . 1) ("second" . 2)))
      (should (eq undo-auto--this-command-amalgamating t))
      (should (equal undo-auto--last-boundary-cause '(timer))))))

(ert-deftest stable-event-contract-encoding-error-unwinds-command-flag ()
  (with-temp-buffer
    (let ((undo-auto--this-command-amalgamating nil)
          (undo-auto--last-boundary-cause '(command)))
      (cl-letf (((symbol-function 'json-encode)
                 (lambda (_payload)
                   (setq undo-auto--this-command-amalgamating t
                         undo-auto--last-boundary-cause 'changed)
                   (error "component encoding failure"))))
        (should-error (stable-event-contract--emit '(("first" . 1)))))
      (should-not undo-auto--this-command-amalgamating)
      (should (equal undo-auto--last-boundary-cause '(command))))))

;;; stable-suite-event-contract.el ends here
