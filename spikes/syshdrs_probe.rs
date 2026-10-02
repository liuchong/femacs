// The Rust side of spikes/syshdrs_probe.c: the same lines from the system-header
// mirrors, so a wrong constant or macro body in `math.rs`/`limits.in.rs`/`float.in.rs`
// is visible (review B's B4: those files were outside every gate).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_macros)]
use core::ffi::{c_double, c_int, c_long, c_ulong};

include!("../femacs/lib/include/stdint.in.rs");
include!("../femacs/lib/include/float.in.rs");
include!("../femacs/lib/include/math.rs");

fn main () {
  println! ("HUGE_VAL_is_pos_inf {}", (isinf! (HUGE_VAL) && 0.0 < HUGE_VAL) as c_int);
  println! ("FP_ILOGB0 {}", FP_ILOGB0);
  println! ("INT_MIN {}", INT_MIN);
  println! ("INT_MAX {}", INT_MAX);
  println! ("FLT_RADIX {}", FLT_RADIX);
  println! ("FLT_MANT_DIG {}", FLT_MANT_DIG);
  println! ("FLT_MAX_EXP {}", FLT_MAX_EXP);
  println! ("FLT_MIN_EXP {}", FLT_MIN_EXP);
  println! ("DBL_MAX_10_EXP {}", DBL_MAX_10_EXP);
  println! ("DBL_MANT_DIG {}", DBL_MANT_DIG);
  println! ("DBL_MIN_EXP {}", DBL_MIN_EXP);
  println! ("DBL_DIG {}", DBL_DIG);
  println! ("ULONG_MAX {}", ULONG_MAX);
  println! ("LONG_MIN {}", LONG_MIN);
  println! ("LONG_MAX {}", LONG_MAX);
  println! ("INTMAX_MIN {}", INTMAX_MIN);
  println! ("INTMAX_MAX {}", INTMAX_MAX);
  println! ("UINTMAX_MAX {}", UINTMAX_MAX);

  let xs: [c_double; 8] = [0.0, -0.0, 1.5, -1.5, c_double::NAN, -c_double::NAN,
                           -HUGE_VAL, HUGE_VAL];
  for (i, x) in xs.iter ().enumerate () {
    println! ("x {} isnan {} isfinite {} isinf {} signbit {}", i, isnan! (*x) as c_int,
              isfinite! (*x) as c_int, isinf! (*x) as c_int, signbit! (*x) as c_int);
  }
}
