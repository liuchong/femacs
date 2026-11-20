;;; macos-default-font.el --- Cocoa default font integration check -*- lexical-binding: t; -*-

(defun fe-test-default-font-state (frame)
  (list (frame-parameter frame 'font)
        (face-attribute 'default :family frame)
        (face-attribute 'default :height frame)
        (frame-char-height frame)))

(defun fe-test-check-font (frame family points minimum-height)
  (let ((state (fe-test-default-font-state frame)))
    (unless (and (equal (nth 1 state) family)
                 (<= (abs (- (nth 2 state) (* 10 points))) 2)
                 (> (nth 3 state) minimum-height))
      (error "Expected %s %s pt and a taller frame glyph, got %S" family points state))
    (princ (format "FONT %S\n" state) 'external-debugging-output)
    state))

(let* ((family (or (getenv "FEMACS_TEST_FONT") "Monaco"))
       (font (font-spec :name family :size 15 :weight 'normal :width 'normal))
       (original-height (frame-char-height)))
  (unless (and (eq window-system 'ns) (find-font font))
    (error "Expected a Cocoa frame and the %s font" family))
  ;; Apply the default font to the current frame and to future frames.
  (set-face-attribute 'default nil :font font)
  (add-hook 'window-setup-hook
            (lambda ()
              (condition-case err
                  (let* ((initial (selected-frame))
                         (state (fe-test-check-font initial family 15 original-height))
                         (next (make-frame '((visibility . nil)))))
                    (unwind-protect
                        (progn
                          (fe-test-check-font next family 15 original-height)
                          (with-selected-frame next
                            (set-frame-font (concat family "-18") t))
                          (fe-test-check-font next family 18 (nth 3 state))
                          (unless (equal (fe-test-default-font-state initial) state)
                            (error "Changing one frame affected the initial frame"))
                          (set-face-attribute 'default next :height 210)
                          (fe-test-check-font next family 21 0)
                          (princ "FONT-PASS\n" 'external-debugging-output))
                      (delete-frame next t))
                    (kill-emacs 0))
                (error
                 (princ (format "FONT-FAIL %S\n" err) 'external-debugging-output)
                 (kill-emacs 1))))))
