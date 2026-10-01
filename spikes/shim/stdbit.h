/* spikes/shim/stdbit.h — compile-only stand-in for the C23 <stdbit.h>.
 *
 * This host has no C23 <stdbit.h> (clang 21 and Homebrew GCC 16 do not ship it)
 * and configure did not instantiate gnulib's lib/stdbit.in.h, so the reference
 * headers cannot be compiled here without it.  The layout spike never calls
 * these, so the behaviour is irrelevant; the port implements the real semantics
 * in Rust.  Recorded in docs/spikes/ as an environment finding.
 */
#ifndef SPIKE_STDBIT_H
#define SPIKE_STDBIT_H
#define stdc_leading_zeros(x)     ((unsigned) __builtin_clzll ((unsigned long long) (x)))
#define stdc_trailing_zeros(x)    ((unsigned) __builtin_ctzll ((unsigned long long) (x)))
#define stdc_count_ones(x)        ((unsigned) __builtin_popcountll ((unsigned long long) (x)))
#define stdc_bit_width(x)         ((unsigned) (64 - __builtin_clzll ((unsigned long long) (x))))
#define stdc_rotate_left_uc(x, n) ((unsigned char) ((((x) << ((n) % 8)) | ((x) >> ((8 - ((n) % 8)) % 8)))))
#define stdc_rotate_left(a, b)    ((a) << ((b) % (8 * sizeof (a))))
#define stdc_memreverse8u(x)      ((unsigned char) (x))
#endif
