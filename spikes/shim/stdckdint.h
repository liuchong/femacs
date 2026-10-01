/* compile-only stand-in for the C23 <stdckdint.h>; the port implements the real
   overflow semantics in Rust (see PORTING.md 2.6). */
#ifndef SPIKE_STDCKDINT_H
#define SPIKE_STDCKDINT_H
#define ckd_add(r, a, b) (*(r) = (a) + (b), 0)
#define ckd_sub(r, a, b) (*(r) = (a) - (b), 0)
#define ckd_mul(r, a, b) (*(r) = (a) * (b), 0)
#endif
