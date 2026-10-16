;; -*- lexical-binding: t -*-
;; A differential probe for the reap of a fast-exiting child process (PORTING.md P-122).
;;
;; The shape is `vc-exec-after`'s (vc-dispatcher.el 239-320): create a process, optionally wrap
;; its sentinel with an `:after` advice the way `vc-exec-after` does, then wait for the sentinel
;; with `accept-process-output'.  `N` sets the iteration count, `ADVICE=1` adds the wrapper.
;;
;; Readings (2026-10-16, `--batch`; `RESULT ... missed=N` lines):
;;
;;   child                        iterations   port        reference
;;   sh -c "exit 0"               20           14          0
;;   sh -c "sleep 0.05; exit 0"   20            0          0
;;   sh -c "exit 0"               10            9          -
;;   sh -c "sleep 0.05; exit 0"   10            2          -
;;   advice-wrapped, "exit 0"      6            4          0
;;
;; The child's speed dominates but is not the whole story: a child that is already gone before
;; the wait begins is missed about nine times in ten, and even a 50 ms child is missed
;; occasionally (2 of 10 in the second round, 0 of 20 in the first).  Every miss has the same
;; signature - the sentinel never runs and `process-status` still answers `run` after 2.5 s of
;; `accept-process-output` - which is what the intermittent `package-vc-tests` failure sees.
;; The advice wrapper of round 4.444 was the same defect observed through a wrapper; the knobs
;; below keep both shapes so the next session can tell them apart.
;;
;; Run:  ./build/port/src/emacs -Q --batch -l spikes/process-sentinel-advice-probe.el
;;       N=20 ADVICE=0 ... ;  N=20 CHILD=slow ...   (see the two knobs below)
(let* ((n (string-to-number (or (getenv "N") "20")))
       (advice (equal (getenv "ADVICE") "1"))
       (cmd (if (equal (getenv "CHILD") "slow") '("sh" "-c" "sleep 0.05; exit 0")
              '("sh" "-c" "exit 0")))
       (missed 0))
  (dotimes (i n)
    (let* ((done nil) (after nil)
           (p (make-process :name (format "sa%d" i) :command cmd :noquery t :buffer nil
                            :sentinel (lambda (_proc _msg) (setq done t)))))
      (when advice
        (add-function :after (process-sentinel p) (lambda (&rest _) (setq after t))))
      (let ((k 0))
        (while (and (not (and done (or after (not advice)))) (< k 500))
          (accept-process-output nil 0.005)
          (setq k (+ k 1))))
      (unless (and done (or after (not advice)))
        (setq missed (+ missed 1))
        (princ (format "MISS i=%d sentinel=%S advice=%S status=%S\n"
                       i done after (process-status p))))))
  (princ (format "RESULT child=%s advice=%S n=%d missed=%d\n"
                 (or (getenv "CHILD") "fast") advice n missed)))
