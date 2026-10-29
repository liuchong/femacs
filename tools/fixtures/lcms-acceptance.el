;;; lcms-acceptance.el --- native LCMS integration acceptance -*- lexical-binding: t; -*-
;; SPDX-License-Identifier: GPL-3.0-or-later

;; These are native-LCMS integration checks, not a replacement for the six
;; original `lcms-tests.el' gold tests.  All inputs are small literal triples;
;; no external files, subprocess, mock, network, or user directory is used.

(require 'cl-lib)
(require 'ert)

(defconst lcms-acceptance--functions
  '(lcms-cie-de2000
    lcms-xyz->jch
    lcms-jch->xyz
    lcms-jch->jab
    lcms-jab->jch
    lcms-cam02-ucs
    lcms-temp->white-point
    lcms2-available-p))

(cl-defun lcms-acceptance--close (a b &optional (tolerance 1e-9))
  "Return non-nil when numbers A and B differ by at most TOLERANCE."
  (and (numberp a) (numberp b)
       (<= (abs (- a b)) tolerance)))

(cl-defun lcms-acceptance--triple-close (a b &optional (tolerance 1e-6))
  "Return non-nil when two three-number lists agree within TOLERANCE."
  (and (listp a) (listp b)
       (= (length a) 3) (= (length b) 3)
       (cl-every (lambda (x y) (lcms-acceptance--close x y tolerance)) a b)))

(ert-deftest lcms-acceptance-native-api ()
  "All public LCMS entry points are native subrs and LCMS is present."
  (dolist (function lcms-acceptance--functions)
    (should (fboundp function))
    (when (fboundp function)
      (should (subrp (symbol-function function)))))
  (should (featurep 'lcms2))
  (should (lcms2-available-p)))

(ert-deftest lcms-acceptance-temperature-boundaries ()
  "The native black-body conversion accepts its documented endpoints."
  (dolist (temperature '(4000 25000))
    (let ((whitepoint (lcms-temp->white-point temperature)))
      (should (= (length whitepoint) 3))
      (should (cl-every #'numberp whitepoint))
      ;; cmsWhitePointFromTemp returns xyY with Y = 1, and cmsxyY2XYZ
      ;; preserves that scale.
      (should (lcms-acceptance--close (nth 1 whitepoint) 1.0 1e-9))))
  (should-error (lcms-temp->white-point 3999) :type 'error)
  (should-error (lcms-temp->white-point 25001) :type 'error))

(ert-deftest lcms-acceptance-cie2000-invariants ()
  "CIEDE2000 zero distance, symmetry, and weight/type boundaries."
  (let ((a '(52 18 -12))
        (b '(48 10 -8)))
    (should (lcms-acceptance--close (lcms-cie-de2000 a a) 0.0 1e-9))
    (should (lcms-acceptance--close (lcms-cie-de2000 a b 1 1 1)
                                    (lcms-cie-de2000 b a 1 1 1) 1e-7))
    (should (> (lcms-cie-de2000 a b) 0.0))
    (should-error (lcms-cie-de2000 '(52 18) b) :type 'error)
    (should-error (lcms-cie-de2000 a b 0) :type 'wrong-type-argument)))

(ert-deftest lcms-acceptance-xyz-jch-roundtrip ()
  "XYZ to JCh and back with explicit white point and viewing conditions."
  (let ((whitepoint '(0.9642 1.0 0.8249))
        (view '(20 100 1 1))
        (xyz '(0.21 0.17 0.13)))
    (should
     (lcms-acceptance--triple-close
      (lcms-jch->xyz (lcms-xyz->jch xyz whitepoint view)
                     whitepoint view)
      xyz
      1e-4))))

(ert-deftest lcms-acceptance-jch-jab-roundtrip ()
  "JCh roundtrip preserves the C reference's numeric result."
  (let ((jch '(41 17 305))
        (view '(20 100 2 1)))
    (should
     (lcms-acceptance--triple-close
      (lcms-jab->jch (lcms-jch->jab jch nil view) nil view)
      ;; Original C uses the rounded forward scale 43.86 and inverse scale
      ;; 0.0228, which are not exact reciprocals.  This is the C reference's
      ;; result for the fixed tuple above, captured by the first integration
      ;; run; preserve that behavior rather than requiring extra accuracy.
      '(41.00000000000001 17.00015830066704 305.0)
      1e-9))))

(ert-deftest lcms-acceptance-cam02-ucs-invariants ()
  "CAM02-UCS self-distance, positivity, and symmetry."
  (let ((c1 '(0.31 0.42 0.23))
        (c2 '(0.27 0.36 0.19))
        (view '(20 100 2 1)))
    (should (lcms-acceptance--close
             (lcms-cam02-ucs c1 c1 nil view) 0.0 1e-9))
    (let ((forward (lcms-cam02-ucs c1 c2 nil view))
          (reverse (lcms-cam02-ucs c2 c1 nil view)))
      (should (> forward 0.0))
      (should (lcms-acceptance--close forward reverse 1e-7)))))

(ert-deftest lcms-acceptance-error-boundaries ()
  "Malformed colors, views, and lengths signal the documented error kinds."
  (should-error (lcms-cam02-ucs '(0.3 0.4) '(0.3 0.4 0.2)) :type 'error)
  (should-error (lcms-cam02-ucs '(0.3 0.4 0.2) '(0.3 0.4 0.2)
                                nil '(20 100 0 1))
                :type 'error)
  (should-error (lcms-xyz->jch "not-a-color") :type 'error)
  (should-error (lcms-jab->jch '(1 2)) :type 'error))

(provide 'lcms-acceptance)

;;; lcms-acceptance.el ends here
