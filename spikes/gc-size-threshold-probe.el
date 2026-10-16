;; Where does the port start retaining a returned object?  (PORTING.md P-116 clause 5,
;; status 4.432.)  For each list length: 100 rounds that build a list, keep only its length,
;; then one explicit collection and the used-cons reading.  A length whose used count jumps by
;; ~100*L is retained; the smallest such length is the threshold, and the threshold names the
;; mechanism (an alloca/xmalloc switch, a scratch buffer, a size-bounded copy).
(dolist (len '(20 200 1000 2000 5000 20000))
  (garbage-collect)
  (let ((before (nth 2 (car (garbage-collect)))))
    (dotimes (i 100)
      (let ((x (make-list len i)))
        (setq probe-keep (length x))))
    (let* ((stats (car (garbage-collect)))
           (used (nth 2 stats))
           (free (nth 3 stats)))
      (princ (format "len=%6d used_before=%8d used_after=%8d free=%8d delta=%8d\n"
                     len before used free (- used before))))))
