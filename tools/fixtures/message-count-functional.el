;;; message-count-functional.el --- actual repeated message logs -*- lexical-binding: t; -*-

(require 'ert)

(defun message-count--capture-sequence (texts)
  "Send TEXTS through the real message primitive and capture each log state."
  (let ((messages-buffer-name (generate-new-buffer-name " *message-count-log*"))
        (message-log-max t)
        states)
    (unwind-protect
        (progn
          (dolist (text texts)
            (message "%s" text)
            (push (with-current-buffer messages-buffer-name
                    (buffer-string))
                  states))
          (nreverse states))
      (when (get-buffer messages-buffer-name)
        (kill-buffer messages-buffer-name)))))

(ert-deftest message-count-functional-undo-repeated ()
  (should (equal (message-count--capture-sequence '("Undo" "Undo" "Undo"))
                 '("Undo\n" "Undo [2 times]\n" "Undo [3 times]\n"))))

(ert-deftest message-count-functional-multibyte-repeated ()
  (should (equal (message-count--capture-sequence '("撤销 λ" "撤销 λ" "撤销 λ"))
                 '("撤销 λ\n" "撤销 λ [2 times]\n" "撤销 λ [3 times]\n"))))

(ert-deftest message-count-functional-literal-count-in-payload ()
  (should (equal (message-count--capture-sequence
                  '("value [2 times]" "value [2 times]" "value [2 times]"))
                 '("value [2 times]\n"
                   "value [2 times] [2 times]\n"
                   "value [2 times] [3 times]\n"))))

(ert-deftest message-count-functional-distinct-message-breaks-run ()
  (should (equal (message-count--capture-sequence
                  '("alpha" "alpha" "beta" "beta" "alpha"))
                 '("alpha\n" "alpha [2 times]\n"
                   "alpha [2 times]\nbeta\n"
                   "alpha [2 times]\nbeta [2 times]\n"
                   "alpha [2 times]\nbeta [2 times]\nalpha\n"))))

;;; message-count-functional.el ends here
