;; A differential probe for the port's conservative stack scan (PORTING.md P-116).
;;
;; Both programs run this and print `(car (garbage-collect))` every 100 iterations:
;;
;;   port      i=0   26,730 used   i=100  2,026,727   i=200  4,026,730
;;   reference i=0   24,822 used   i=100     24,822   i=200     24,822
;;
;; The port retains about one discarded 20,000-element list per iteration.  The cause is the
;; build's frame size, not the collector: at `-C opt-level=0` the interpreter's frames are
;; about fourteen times C's (P-101), so each iteration leaves distinct stale pointers at many
;; stack offsets and the conservative scan (which C does the same way) keeps one object per
;; call chain alive.  It is why the memory-heavy ERT files (ert-tests, package-tests,
;; package-vc-tests) climb to the executor's limit while the reference runs them (status 4.429).
;;
;; Run:  ./build/port/src/emacs -Q --batch -l spikes/gc-retention-probe.el
(dotimes (i 300)
  (let ((x (make-list 20000 i)))
    (setq probe-keep (length x)))
  (when (= 0 (% i 100)) (princ (format "i=%d conses=%S\n" i (car (garbage-collect))))))
