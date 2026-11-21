// femacs: mirror of the frozen target's <stdbit.h> (C23; gnulib header, P-26).
//
// The frozen configuration has HAVE_STDBIT_H **undefined** (`build/emacs-ref/src/config.h`
// 1433 `/* #undef HAVE_STDBIT_H */`, `docs/baseline/config/darwin-arm64.macros.tsv` 1556
// `HAVE_STDBIT_H<TAB>undef`), so the system has no C23 `<stdbit.h>` and gnulib generates its
// own from `stdbit.in.h` with `configure`'s substitutions applied
// (`build/emacs-ref/lib/stdbit.h`, 2,230 lines - the judge `include-stdbit.in.md` uses): the
// `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/POPCOUNT` branches are the live ones (this target's compiler
// *does* have __builtin_clz/ctz/popcount - that is a fact about clang, not about the system
// header), so
// the `_gl_stdbit_*` helpers the macro arms call are Rust's leading_zeros,
// trailing_zeros and count_ones.  The three generic macros the ported files use are
// mirrored whole - each is C's dispatcher on `sizeof (n)`, and the narrow variants
// subtract the promoted width exactly as stdbit.in.h 318-345, 500-529 and 966-992 do.
//
// `_GL_STDBIT_HAS_BUILTIN_BSWAP16/32/64` are the live arms as well (this target's
// compiler has __builtin_bswap*), so the four `stdc_memreverse8u*` bodies below are
// `swap_bytes`; the header's hand-written masks are the dead arm (P-51).
//
// The header's leading-ones/trailing-ones/first-*/rotate/load8/store8 functions and the
// remaining generic macros are not mirrored: no ported file calls them yet, and a mirror
// is added when one does (the stdcountof.rs precedent).
//
// The four extern-inline units that include this header - lib/stdc_leading_zeros.rs,
// lib/stdc_trailing_zeros.rs, lib/stdc_bit_width.rs and lib/stdc_memreverse8u.rs - export
// their own share of these functions (PORTING.md P-81), so this mirror stays
// declaration-only: `#[inline(always)]` items and `macro_rules!`, never a symbol.
// Every unit that includes it brings the C names into scope itself, with
// `use std::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};`.

// **Ownership (integrator ruling, current round): the C23 API's *functions* live in
// `include/stdbit.in.rs`** - the mirror of the template that generated this header - because
// `tools/extract-abi.py` reads a mirror's *own* text for function bodies (measured: an empty
// mirror reports 278 missing items, this file's `include!` only 274, and the ten `fn`s it carried
// were still reported missing), and because two mirrors declaring one function is `E0428` in any
// crate that sees both.  So this file keeps the generated header's **facts** (the arm readings
// above) and the **generic dispatcher macros** below, and every unit reaches the functions through
// the owner: `include!("stdbit.in.rs")`.
//
// The functions that used to live here (`_gl_stdbit_clz/clzl/clzll`, `_gl_stdbit_ctz*`,
// `stdc_leading_zeros_*`, `stdc_trailing_zeros_*`, `stdc_count_ones_*`, `stdc_bit_width_*`,
// `stdc_memreverse8u{8,16,32,64}`) moved to `stdbit.in.rs` with their C-line comments; what
// remains to be written there is the rest of the API (the owner's ledger lists it family by
// family).

// [U1-MODULE-MIRROR]
use super::*;
/* stdbit.in.h 350-355: `#define stdc_leading_zeros(n) (sizeof (n) == 1 ? … )`.  Rust
   has no `?:` chain, so the same five-way dispatch is an if/else expression; the casts
   are C's implicit conversions to the chosen variant's parameter type.  */
macro_rules! stdc_leading_zeros {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_leading_zeros_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_leading_zeros_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_leading_zeros_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_leading_zeros_ul ($n as c_ulong) }
         else
           { stdc_leading_zeros_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 532-537.  */
macro_rules! stdc_trailing_zeros {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_trailing_zeros_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_trailing_zeros_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_trailing_zeros_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_trailing_zeros_ul ($n as c_ulong) }
         else
           { stdc_trailing_zeros_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 998-1003.  */
macro_rules! stdc_count_ones {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_count_ones_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_count_ones_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_count_ones_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_count_ones_ul ($n as c_ulong) }
         else
           { stdc_count_ones_ull ($n as c_ulonglong) })
    };
}

/* stdbit.in.h 1151-1156.  */
macro_rules! stdc_bit_width {
    ($n:expr) => {
        (if std::mem::size_of_val (&$n) == 1
           { stdc_bit_width_uc ($n as c_uchar) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ushort> ()
           { stdc_bit_width_us ($n as c_ushort) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_uint> ()
           { stdc_bit_width_ui ($n as c_uint) }
         else if std::mem::size_of_val (&$n) == std::mem::size_of::<c_ulong> ()
           { stdc_bit_width_ul ($n as c_ulong) }
         else
           { stdc_bit_width_ull ($n as c_ulonglong) })
    };
}
