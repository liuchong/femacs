/* The C side of spikes/syshdrs-probe.sh: the values and macros the port's *system*
   header mirrors carry (`lib/include/math.rs`, `limits.in.rs`, `float.in.rs`).
   These mirrors have no reference file to be paired with, so this probe is their gate:
   the numbers and the four macros come from clang's own headers, and the Rust side must
   print the same lines (review B's B4).  */
#include <stdio.h>
#include <math.h>
#include <limits.h>
#include <float.h>
#include <stdint.h>
#include <inttypes.h>

/* The four machine widths come from this target's C23 <limits.h>; SIZE_WIDTH does
   not: the frozen build takes it from gnulib's stdint.in.h 728 (the macro table says
   `_GL_INTEGER_WIDTH (0, SIZE_MAX)`), so it is covered by the stdint.in.md checklist
   rather than by this system-header probe.  */
int
main (void)
{
  printf ("HUGE_VAL_is_pos_inf %d\n", isinf (HUGE_VAL) && 0 < HUGE_VAL);
  printf ("FP_ILOGB0 %d\n", FP_ILOGB0);
  printf ("INT_MIN %d\n", INT_MIN);
  printf ("INT_MAX %d\n", INT_MAX);
  printf ("FLT_RADIX %d\n", FLT_RADIX);
  printf ("FLT_MANT_DIG %d\n", FLT_MANT_DIG);
  printf ("FLT_MAX_EXP %d\n", FLT_MAX_EXP);
  printf ("FLT_MIN_EXP %d\n", FLT_MIN_EXP);
  printf ("DBL_MAX_10_EXP %d\n", DBL_MAX_10_EXP);
  printf ("DBL_MANT_DIG %d\n", DBL_MANT_DIG);
  printf ("DBL_MIN_EXP %d\n", DBL_MIN_EXP);
  printf ("DBL_DIG %d\n", DBL_DIG);
  printf ("INT_WIDTH %d\n", (int) INT_WIDTH);
  printf ("UINT_WIDTH %d\n", (int) UINT_WIDTH);
  printf ("LONG_WIDTH %d\n", (int) LONG_WIDTH);
  printf ("ULONG_WIDTH %d\n", (int) ULONG_WIDTH);
  printf ("ULONG_MAX %lu\n", ULONG_MAX);
  printf ("LONG_MIN %ld\n", LONG_MIN);
  printf ("LONG_MAX %ld\n", LONG_MAX);
  printf ("INTMAX_MIN %jd\n", (intmax_t) INTMAX_MIN);
  printf ("INTMAX_MAX %jd\n", (intmax_t) INTMAX_MAX);
  printf ("UINTMAX_MAX %ju\n", (uintmax_t) UINTMAX_MAX);

  double xs[] = { 0.0, -0.0, 1.5, -1.5, NAN, -NAN, -HUGE_VAL, HUGE_VAL };
  for (size_t i = 0; i < sizeof xs / sizeof *xs; i++)
    printf ("x %zu isnan %d isfinite %d isinf %d signbit %d\n", i,
	    !!isnan (xs[i]), !!isfinite (xs[i]), !!isinf (xs[i]), !!signbit (xs[i]));
  return 0;
}
