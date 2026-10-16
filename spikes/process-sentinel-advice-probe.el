;; -*- lexical-binding: t -*-
;; A differential probe for advice-wrapped process sentinels (PORTING.md P-122).
;;
;; `vc-exec-after` attaches its queued work to a *live* process's sentinel with
;; `(add-function :after (process-sentinel proc) fun)` (vc-dispatcher.el 239-320), and
;; `vc-run-delayed` is built on it, which is how the `vc-post-command-functions` hook of an
;; async `git pull` reaches its callers.  This probe performs exactly that shape - create a
;; process, wrap its sentinel with an `:after` advice, then wait for both the original sentinel
;; and the wrapper - twenty times, and reports the misses.
;;
;; Readings (2026-10-16, `--batch`, this file alone, 20 iterations each):
;;
;;   port      missed 14 of 20   (each miss: the original sentinel never ran *and* the process
;;                                was still `run` after 2.5 s of `accept-process-output`)
;;   reference missed  0 of 20
;;
;; Without the advice the same shape completes in 0.022-0.028 s on both programs (status
;; 4.443), so this is where the intermittent `package-vc-tests` failure lives.
;;
;; Run:  ./build/port/src/emacs -Q --batch -l spikes/process-sentinel-advice-probe.el
(let ((missed 0))
  (dotimes (i 20)
    (let* ((after nil) (plain nil)
           (p (make-process :name (format "sa%d" i) :command '("sh" "-c" "exit 0")
                            :noquery t :buffer nil
                            :sentinel (lambda (_proc _msg) (setq plain t)))))
      (add-function :after (process-sentinel p) (lambda (&rest _) (setq after t)))
      (let ((n 0))
        (while (and (not (and plain after)) (< n 500))
          (accept-process-output nil 0.005)
          (setq n (+ n 1))))
      (unless (and plain after)
        (setq missed (+ missed 1))
        (princ (format "MISS i=%d plain=%S after=%S status=%S\n"
                       i plain after (process-status p))))))
  (princ (format "RESULT missed=%d of 20\n" missed)))
