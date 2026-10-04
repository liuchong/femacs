// Semantic probe for `femacs/lib/include/stdbit.in.rs` (the C23 <stdbit.h> closure).
//
// The token gate (`tools/extract-abi.py --check`) is a *token* gate: it cannot see a wrong value
// whose token is still present somewhere in the body (measured - see the row below).  This probe
// therefore checks the family layer against the **standard library's own operation for the same
// function** (`is_power_of_two`, `rotate_left`/`rotate_right`, `from_le_bytes`/`to_le_bytes`,
// `from_be_bytes`/`to_be_bytes`, `leading_ones`/`leading_zeros`, `trailing_*`, `count_ones`), never
// against a literal typed by hand: an assertion that disagrees with Rust is the port's fault or
// mine, and the machine decides which.
//
// Run it from the repository root:
//   rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort \
//         -o /tmp/stdbit_family_probe spikes/stdbit_family_probe.rs
//   /tmp/stdbit_family_probe
// Expected: `stdbit probe: 544 semantic checks passed`, exit 0.  Making
// `stdc_bit_floor_ui`'s `else { 0 }` an `else { 1 }` keeps `--check` green and makes this probe
// panic on `bit_floor_ui 0` (left: 1, right: 0) - the two gates are not substitutes.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
                c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};

include!("../femacs/lib/include/stdbit.in.rs");

fn main () {
    let mut n_checks: u64 = 0;
    let vals_u32: [c_uint; 12] = [0, 1, 2, 3, 7, 8, 255, 256, 0x8000_0000, 0xFFFF_FFFF,
                                  0xDEAD_BEEF, 0x4000_0000];
    let vals_u16: [c_ushort; 8] = [0, 1, 2, 3, 255, 256, 0x8000, 0xFFFF];
    let vals_u8: [c_uchar; 7] = [0, 1, 2, 3, 127, 128, 255];
    let vals_u64: [c_ulonglong; 8] = [0, 1, 2, 3, 0x8000_0000_0000_0000, u64::MAX,
                                      0x0123_4567_89AB_CDEF, 0xFFFF_0000_FFFF_0000];

    // --- has_single_bit: Rust's own is_power_of_two
    for &n in &vals_u32 {
        assert_eq! (stdc_has_single_bit_ui (n), n.is_power_of_two (), "has_single_bit_ui {}", n);
        n_checks += 1;
    }
    for &n in &vals_u16 {
        assert_eq! (stdc_has_single_bit_us (n), n.is_power_of_two (), "has_single_bit_us {}", n);
        n_checks += 1;
    }
    for &n in &vals_u8 {
        assert_eq! (stdc_has_single_bit_uc (n), n.is_power_of_two (), "has_single_bit_uc {}", n);
        n_checks += 1;
    }
    for &n in &vals_u64 {
        assert_eq! (stdc_has_single_bit_ull (n), n.is_power_of_two (), "has_single_bit_ull {}", n);
        n_checks += 1;
    }

    // --- bit_floor / bit_ceil: the C23 definitions written with the standard library alone
    fn floor_ref32 (n: u32) -> u32 { if n == 0 { 0 } else { 1u32 << (31 - n.leading_zeros ()) } }
    fn ceil_ref32 (n: u32) -> u32 {
        if n <= 1 { 1 }
        else { let b = 32 - (n - 1).leading_zeros (); if b >= 32 { 0 } else { 1u32 << b } }
    }
    fn floor_ref8 (n: u8) -> u8 { if n == 0 { 0 } else { 1u8 << (7 - n.leading_zeros ()) } }
    fn ceil_ref8 (n: u8) -> u8 {
        if n <= 1 { 1 }
        else { let b = 8 - (n - 1).leading_zeros (); if b >= 8 { 0 } else { 1u8 << b } }
    }
    fn ceil_ref64 (n: u64) -> u64 {
        if n <= 1 { 1 }
        else { let b = 64 - (n - 1).leading_zeros (); if b >= 64 { 0 } else { 1u64 << b } }
    }
    for &n in &vals_u32 {
        assert_eq! (stdc_bit_floor_ui (n), floor_ref32 (n), "bit_floor_ui {}", n);
        assert_eq! (stdc_bit_ceil_ui (n), ceil_ref32 (n), "bit_ceil_ui {}", n);
        n_checks += 2;
    }
    for &n in &vals_u8 {
        assert_eq! (stdc_bit_floor_uc (n), floor_ref8 (n), "bit_floor_uc {}", n);
        assert_eq! (stdc_bit_ceil_uc (n), ceil_ref8 (n), "bit_ceil_uc {}", n);
        n_checks += 2;
    }
    for &n in &vals_u64 {
        assert_eq! (stdc_bit_ceil_ull (n), ceil_ref64 (n), "bit_ceil_ull {}", n);
        n_checks += 1;
    }

    // --- first_*: the 1-based index of the first bit, computed from the standard library's
    //     leading_ones/leading_zeros/trailing_ones/trailing_zeros
    for &n in &vals_u32 {
        let flz = if n == u32::MAX { 0 } else { n.leading_ones () + 1 };
        let flo = if n == 0 { 0 } else { n.leading_zeros () + 1 };
        let ftz = if n == u32::MAX { 0 } else { n.trailing_ones () + 1 };
        let fto = if n == 0 { 0 } else { n.trailing_zeros () + 1 };
        assert_eq! (stdc_first_leading_zero_ui (n), flz, "first_leading_zero_ui {}", n);
        assert_eq! (stdc_first_leading_one_ui (n), flo, "first_leading_one_ui {}", n);
        assert_eq! (stdc_first_trailing_zero_ui (n), ftz, "first_trailing_zero_ui {}", n);
        assert_eq! (stdc_first_trailing_one_ui (n), fto, "first_trailing_one_ui {}", n);
        n_checks += 4;
    }
    for &n in &vals_u8 {
        let flz = if n == u8::MAX { 0 } else { n.leading_ones () + 1 };
        assert_eq! (stdc_first_leading_zero_uc (n), flz, "first_leading_zero_uc {}", n);
        n_checks += 1;
    }

    // --- rotate: the standard library's own rotate (which masks the count itself)
    for &v in &vals_u32 {
        for c in [0u32, 1, 5, 31, 32, 33, 100, 0xFFFF_FFFF] {
            assert_eq! (stdc_rotate_left_ui (v, c), v.rotate_left (c), "rotate_left_ui {} {}", v, c);
            assert_eq! (stdc_rotate_right_ui (v, c), v.rotate_right (c), "rotate_right_ui {} {}", v, c);
            n_checks += 2;
        }
    }
    for &v in &vals_u8 {
        for c in [0u32, 3, 8, 9, 0xFFFF_FFFF] {
            assert_eq! (stdc_rotate_left_uc (v, c), v.rotate_left (c), "rotate_left_uc {} {}", v, c);
            assert_eq! (stdc_rotate_right_uc (v, c), v.rotate_right (c), "rotate_right_uc {} {}", v, c);
            n_checks += 2;
        }
    }

    // --- the `ul` members (the same width as `ull` on this target, but a separate body each)
    for &n in &vals_u64 {
        assert_eq! (stdc_has_single_bit_ul (n), n.is_power_of_two (), "has_single_bit_ul {}", n);
        assert_eq! (stdc_bit_floor_ul (n), if n == 0 { 0 } else { 1u64 << (63 - n.leading_zeros ()) },
                    "bit_floor_ul {}", n);
        assert_eq! (stdc_bit_ceil_ul (n), ceil_ref64 (n), "bit_ceil_ul {}", n);
        assert_eq! (stdc_first_leading_zero_ul (n),
                    if n == u64::MAX { 0 } else { n.leading_ones () + 1 }, "first_leading_zero_ul {}", n);
        assert_eq! (stdc_first_trailing_one_ul (n),
                    if n == 0 { 0 } else { n.trailing_zeros () + 1 }, "first_trailing_one_ul {}", n);
        assert_eq! (stdc_rotate_left_ul (n, 7), n.rotate_left (7), "rotate_left_ul {}", n);
        assert_eq! (stdc_rotate_right_ul (n, 70), n.rotate_right (70), "rotate_right_ul {}", n);
        n_checks += 7;
    }

    // --- memreverse8u*: from_be_bytes/to_le_bytes, not from swap_bytes
    for &v in &vals_u16 {
        assert_eq! (stdc_memreverse8u16 (v), u16::from_be_bytes (v.to_le_bytes ()), "memreverse8u16 {}", v);
        n_checks += 1;
    }
    for &v in &vals_u32 {
        assert_eq! (stdc_memreverse8u32 (v), u32::from_be_bytes (v.to_le_bytes ()), "memreverse8u32 {}", v);
        n_checks += 1;
    }
    for &v in &vals_u64 {
        assert_eq! (stdc_memreverse8u64 (v), u64::from_be_bytes (v.to_le_bytes ()), "memreverse8u64 {}", v);
        n_checks += 1;
    }

    // --- load8/store8: the standard library's own from_le_bytes/to_le_bytes on the same buffer
    let buf32: [u8; 4] = [0x01, 0x23, 0x45, 0x67];
    let buf16: [u8; 2] = [0x89, 0xAB];
    let buf64: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq! (stdc_load8_leu32 (buf32.as_ptr ()), u32::from_le_bytes (buf32), "load8_leu32");
    assert_eq! (stdc_load8_beu32 (buf32.as_ptr ()), u32::from_be_bytes (buf32), "load8_beu32");
    assert_eq! (stdc_load8_leu16 (buf16.as_ptr ()), u16::from_le_bytes (buf16), "load8_leu16");
    assert_eq! (stdc_load8_beu16 (buf16.as_ptr ()), u16::from_be_bytes (buf16), "load8_beu16");
    assert_eq! (stdc_load8_leu64 (buf64.as_ptr ()), u64::from_le_bytes (buf64), "load8_leu64");
    assert_eq! (stdc_load8_beu64 (buf64.as_ptr ()), u64::from_be_bytes (buf64), "load8_beu64");
    assert_eq! (stdc_load8_leu8 (buf32.as_ptr ()), buf32[0], "load8_leu8");
    assert_eq! (stdc_load8_beu8 (buf32.as_ptr ()), buf32[0], "load8_beu8");
    n_checks += 8;
    // the aligned family is C's `else` arm (the frozen _GL_STDBIT_OPTIMIZE_VIA_MEMCPY is 0)
    assert_eq! (stdc_load8_aligned_leu32 (buf32.as_ptr ()), stdc_load8_leu32 (buf32.as_ptr ()), "aligned_leu32");
    assert_eq! (stdc_load8_aligned_beu64 (buf64.as_ptr ()), stdc_load8_beu64 (buf64.as_ptr ()), "aligned_beu64");
    assert_eq! (stdc_load8_aligned_les16 (buf16.as_ptr ()), stdc_load8_les16 (buf16.as_ptr ()), "aligned_les16");
    assert_eq! (stdc_load8_aligned_bes64 (buf64.as_ptr ()), stdc_load8_bes64 (buf64.as_ptr ()), "aligned_bes64");
    n_checks += 4;

    let mut out32: [u8; 4] = [0; 4];
    stdc_store8_leu32 (0x0123_4567, out32.as_mut_ptr ());
    assert_eq! (out32, 0x0123_4567u32.to_le_bytes (), "store8_leu32");
    stdc_store8_beu32 (0x0123_4567, out32.as_mut_ptr ());
    assert_eq! (out32, 0x0123_4567u32.to_be_bytes (), "store8_beu32");
    let mut out64: [u8; 8] = [0; 8];
    stdc_store8_leu64 (0x0123_4567_89AB_CDEF, out64.as_mut_ptr ());
    assert_eq! (out64, 0x0123_4567_89AB_CDEFu64.to_le_bytes (), "store8_leu64");
    stdc_store8_beu64 (0x0123_4567_89AB_CDEF, out64.as_mut_ptr ());
    assert_eq! (out64, 0x0123_4567_89AB_CDEFu64.to_be_bytes (), "store8_beu64");
    let mut out16: [u8; 2] = [0; 2];
    stdc_store8_les16 (-2i16, out16.as_mut_ptr ());
    assert_eq! (out16, (-2i16 as u16).to_le_bytes (), "store8_les16");
    stdc_store8_beu8 (0x5A, out16.as_mut_ptr ());
    assert_eq! (out16[0], 0x5A, "store8_beu8");
    stdc_store8_aligned_leu32 (0x0123_4567, out32.as_mut_ptr ());
    assert_eq! (out32, 0x0123_4567u32.to_le_bytes (), "store8_aligned_leu32");
    stdc_store8_aligned_beu32 (0x0123_4567, out32.as_mut_ptr ());
    assert_eq! (out32, 0x0123_4567u32.to_be_bytes (), "store8_aligned_beu32");
    n_checks += 8;

    // --- memreverse8: reverse the buffer
    let mut rev: [u8; 5] = [1, 2, 3, 4, 5];
    stdc_memreverse8 (5, rev.as_mut_ptr ());
    let mut want: [u8; 5] = [1, 2, 3, 4, 5];
    want.reverse ();
    assert_eq! (rev, want, "memreverse8");
    let mut none: [u8; 3] = [7, 8, 9];
    stdc_memreverse8 (0, none.as_mut_ptr ());
    assert_eq! (none, [7, 8, 9], "memreverse8 (0)");
    n_checks += 2;

    // --- count_ones / count_zeros (the families that moved with the popcount macros)
    for &n in &vals_u32 {
        assert_eq! (stdc_count_ones_ui (n) as u32, n.count_ones (), "count_ones_ui {}", n);
        assert_eq! (stdc_count_zeros_ui (n) as u32, n.count_zeros (), "count_zeros_ui {}", n);
        n_checks += 2;
    }
    for &n in &vals_u64 {
        assert_eq! (stdc_count_ones_ull (n) as u32, n.count_ones (), "count_ones_ull {}", n);
        assert_eq! (stdc_count_zeros_ull (n) as u32, n.count_zeros (), "count_zeros_ull {}", n);
        n_checks += 2;
    }

    println! ("stdbit probe: {} semantic checks passed", n_checks);
}
