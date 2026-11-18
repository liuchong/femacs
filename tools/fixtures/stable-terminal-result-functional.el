;;; stable-terminal-result-functional.el --- owned result transport -*- lexical-binding: t; -*-

(require 'ert)

(ert-deftest fe-original-terminal-result-sentinel ()
  "Emit one deliberate owned failure so the observer has a real ERT condition."
  (error "owned terminal result sentinel"))

(defun fe-original-terminal-result-functional ()
  "Verify result observation does not replace the original ERT outcome."
  (let* ((stats (ert-run-tests-batch 'fe-original-terminal-result-sentinel))
         (test (ert-get-test 'fe-original-terminal-result-sentinel))
         (result (ert-test-most-recent-result test)))
    (unless (and (= (ert-stats-completed-unexpected stats) 1)
                 (ert-test-result-type-p result :failed)
                 (equal (ert-test-result-with-condition-condition result)
                        '(error "owned terminal result sentinel")))
      (error "Original ERT failure was altered by observation"))
    (kill-emacs 0)))

(provide 'stable-terminal-result-functional)
