# emacs/lib/cdefs.h (900 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 125
- [x] _Noreturn  (line 697, body da39a3ee5e6b)
- [x] _SYS_CDEFS_H  (line 20, body 356a192b7913)
- [x] _Static_assert  (line 724, body 2ad1031480f0)
- [x] __ASMNAME  (line 423, body cc86501e8c2c)
- [x] __ASMNAME2  (line 424, body 5f069fd70e99)
- [x] __BEGIN_DECLS  (line 138, body da39a3ee5e6b)
- [x] __COLD  (line 114, body da39a3ee5e6b)
- [x] __CONCAT  (line 126, body 72559cbdecbc)
- [x] __END_DECLS  (line 139, body da39a3ee5e6b)
- [x] __HAVE_GENERIC_SELECTION  (line 830, body b6589fc6ab0d)
- [x] __LDBL_COMPAT  (line 768, body 356a192b7913)
- [x] __LDBL_REDIR  (line 791, body b66ecf5937bc)
- [x] __LDBL_REDIR1  (line 790, body b66ecf5937bc)
- [x] __LDBL_REDIR1_DECL  (line 778, body f13d58c76aab)
- [x] __LDBL_REDIR1_NTH  (line 792, body 6e3fa2b16098)
- [x] __LDBL_REDIR2_DECL  (line 794, body da39a3ee5e6b)
- [x] __LDBL_REDIR_DECL  (line 795, body da39a3ee5e6b)
- [x] __LDBL_REDIR_NTH  (line 793, body 6e3fa2b16098)
- [x] __LEAF  (line 63, body da39a3ee5e6b)
- [x] __LEAF_ATTR  (line 64, body da39a3ee5e6b)
- [x] __NTH  (line 113, body cfd3487e1038)
- [x] __NTHNL  (line 92, body cfd3487e1038)
- [x] __P  (line 120, body 3030e728f154)
- [x] __PMT  (line 121, body 3030e728f154)
- [x] __REDIRECT  (line 411, body fc7eb9b18917)
- [x] __REDIRECT_FORTIFY  (line 427, body 694e30ad9d1f)
- [x] __REDIRECT_FORTIFY_NTH  (line 431, body 1eccd7e50603)
- [x] __REDIRECT_LDBL  (line 797, body 0076b5eea80a)
- [x] __REDIRECT_NTH  (line 418, body 29634efba8aa)
- [x] __REDIRECT_NTHNL  (line 420, body 7a4bc4cd9ed8)
- [x] __REDIRECT_NTH_LDBL  (line 798, body 1db0c40e46e3)
- [x] __STRING  (line 127, body f1d202006571)
- [x] __THROW  (line 111, body da39a3ee5e6b)
- [x] __THROWNL  (line 112, body da39a3ee5e6b)
- [x] __always_inline  (line 599, body ccca8da80446)
- [x] __attr_access  (line 868, body da39a3ee5e6b)
- [x] __attr_access_none  (line 869, body da39a3ee5e6b)
- [x] __attr_dealloc  (line 879, body da39a3ee5e6b)
- [x] __attr_dealloc_free  (line 880, body da39a3ee5e6b)
- [x] __attribute__  (line 446, body da39a3ee5e6b)
- [x] __attribute_alloc_align__  (line 473, body da39a3ee5e6b)
- [x] __attribute_alloc_size__  (line 464, body da39a3ee5e6b)
- [x] __attribute_artificial__  (line 607, body da39a3ee5e6b)
- [x] __attribute_const__  (line 489, body da39a3ee5e6b)
- [x] __attribute_copy__  (line 717, body da39a3ee5e6b)
- [x] __attribute_deprecated__  (line 513, body da39a3ee5e6b)
- [x] __attribute_deprecated_msg__  (line 524, body 184ff05d702e)
- [x] __attribute_format_arg__  (line 536, body da39a3ee5e6b)
- [x] __attribute_format_strfmon__  (line 547, body da39a3ee5e6b)
- [x] __attribute_malloc__  (line 455, body da39a3ee5e6b)
- [x] __attribute_maybe_unused__  (line 495, body da39a3ee5e6b)
- [x] __attribute_noinline__  (line 506, body da39a3ee5e6b)
- [x] __attribute_nonnull__  (line 558, body da39a3ee5e6b)
- [x] __attribute_nonstring__  (line 707, body da39a3ee5e6b)
- [x] __attribute_overloadable__  (line 148, body da39a3ee5e6b)
- [x] __attribute_pure__  (line 482, body da39a3ee5e6b)
- [x] __attribute_returns_twice__  (line 888, body da39a3ee5e6b)
- [x] __attribute_struct_may_alias__  (line 897, body da39a3ee5e6b)
- [x] __attribute_used__  (line 505, body 05f0c97b596b)
- [x] __attribute_warn_unused_result__  (line 584, body da39a3ee5e6b)
- [x] __bos  (line 157, body b46e1951f6ff)
- [x] __bos0  (line 158, body 8b6510b6919a)
- [x] __errordecl  (line 372, body fbbf19d94845)
- [x] __extension__  (line 648, body da39a3ee5e6b)
- [x] __extern_always_inline  (line 628, body 94b81fdeb928)
- [x] __extern_inline  (line 627, body f22493eca0ab)
- [x] __flexarr  (line 394, body f629ae44b7b3)
- [x] __fortified_attr_access  (line 867, body da39a3ee5e6b)
- [x] __fortify_clang_bos0_static_lt  (line 251, body 936709953805)
- [x] __fortify_clang_bos0_static_lt2  (line 249, body 4180abd027a5)
- [x] __fortify_clang_bos_static_lt  (line 247, body 204c078216f0)
- [x] __fortify_clang_bos_static_lt2  (line 245, body 6fcbfeeda390)
- [x] __fortify_clang_bos_static_lt_impl  (line 243, body f916ab00a238)
- [x] __fortify_clang_bosn_args  (line 254, body fb25470868b1)
- [x] __fortify_clang_error  (line 260, body 988024f38009)
- [x] __fortify_clang_mul_may_overflow  (line 292, body 3684eb3a9dff)
- [x] __fortify_clang_overload_arg  (line 319, body 5d4f4449ec3c)
- [x] __fortify_clang_overload_arg0  (line 321, body 3cfb9650fa1a)
- [x] __fortify_clang_pass_dynamic_object_size  (line 240, body da39a3ee5e6b)
- [x] __fortify_clang_pass_dynamic_object_size0  (line 239, body da39a3ee5e6b)
- [x] __fortify_clang_pass_dynamic_object_size_n  (line 238, body da39a3ee5e6b)
- [x] __fortify_clang_pass_object_size  (line 227, body 58ce24be9f27)
- [x] __fortify_clang_pass_object_size0  (line 225, body a5a08ca67ba6)
- [x] __fortify_clang_pass_object_size_n  (line 223, body 8fe5f944b1ba)
- [x] __fortify_clang_prefer_this_overload  (line 275, body 09317aa05aa2)
- [x] __fortify_clang_size_too_small  (line 295, body 1d9e12096229)
- [x] __fortify_clang_unavailable  (line 277, body 26aee1f8953a)
- [x] __fortify_clang_warn_if_dest_too_small  (line 324, body da39a3ee5e6b)
- [x] __fortify_clang_warn_if_dest_too_small0  (line 325, body da39a3ee5e6b)
- [x] __fortify_clang_warn_if_src_too_large  (line 323, body da39a3ee5e6b)
- [x] __fortify_clang_warning  (line 314, body da39a3ee5e6b)
- [x] __fortify_clang_warning_only_if_bos0_lt  (line 315, body da39a3ee5e6b)
- [x] __fortify_clang_warning_only_if_bos0_lt2  (line 316, body da39a3ee5e6b)
- [x] __fortify_clang_warning_only_if_bos_lt  (line 317, body da39a3ee5e6b)
- [x] __fortify_clang_warning_only_if_bos_lt2  (line 318, body da39a3ee5e6b)
- [x] __fortify_function  (line 633, body d76b3df1e421)
- [x] __fortify_function_error_function  (line 221, body 1b6b63a9a356)
- [x] __fortify_use_clang  (line 313, body b6589fc6ab0d)
- [x] __glibc_c99_flexarr_available  (line 395, body b6589fc6ab0d)
- [x] __glibc_const_generic  (line 839, body 61208ba3a0e2)
- [x] __glibc_fortify  (line 340, body b0163d6adf83)
- [x] __glibc_fortify_n  (line 357, body 598643bfc934)
- [x] __glibc_has_attribute  (line 42, body b6589fc6ab0d)
- [x] __glibc_has_builtin  (line 47, body b6589fc6ab0d)
- [x] __glibc_has_extension  (line 52, body b6589fc6ab0d)
- [x] __glibc_likely  (line 687, body b64e6d7bb76b)
- [x] __glibc_macro_warning  (line 813, body da39a3ee5e6b)
- [x] __glibc_macro_warning1  (line 809, body f90bce4ddec0)
- [x] __glibc_objsize  (line 167, body 1e4f148808b5)
- [x] __glibc_objsize0  (line 166, body 6074a9f03a63)
- [x] __glibc_safe_len_cond  (line 175, body 66387a275a08)
- [x] __glibc_safe_or_unknown_len  (line 182, body 571e150c4008)
- [x] __glibc_unlikely  (line 686, body b64e6d7bb76b)
- [x] __glibc_unsafe_len  (line 192, body 10e74a92dd2b)
- [x] __glibc_unsigned_or_positive  (line 176, body 70673bed9619)
- [x] __inline  (line 108, body da39a3ee5e6b)
- [x] __nonnull  (line 562, body b65f94926045)
- [x] __ptr_t  (line 130, body a6c67a59fdf9)
- [x] __restrict  (line 658, body da39a3ee5e6b)
- [x] __restrict_arr  (line 677, body da39a3ee5e6b)
- [x] __returns_nonnull  (line 571, body da39a3ee5e6b)
- [x] __va_arg_pack  (line 639, body e4f762aa73d3)
- [x] __va_arg_pack_len  (line 640, body 6f362ea0a597)
- [x] __warnattr  (line 371, body da39a3ee5e6b)
- [x] __wur  (line 587, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

- **The mirror is transcribed by a committed tool, and that is the faithful form here.**  The
  reference is 900 lines of `#define` in which 83 of the 125 names carry one arm per compiler
  family; there is no judgement in the text to hand-copy.  `tools/gen-cdefs-mirror.py` does the
  copy under named rules (continuations joined, comments dropped, C parameter names rewritten to
  Rust metavariables, `...` -> `$($va:tt)*`, `__VA_ARGS__` -> `$($va)*`, integer suffixes
  stripped, `##` written as two spaced `#` tokens) and the mirror's own header repeats them:

      $ python3 tools/gen-cdefs-mirror.py --reference emacs/lib/cdefs.h \
          --dM /tmp/cdefs-dM.txt --out femacs/lib/include/cdefs.rs

- **The live arm is measured, not the file order.**  `c["macros"]` keeps the *last* definition and
  this header's arms are compiler families, so the decision is clang's: the frozen reachability
  path is `lib/libc-config.h` C 181's `#include <cdefs.h>` (`<features.h>` does not exist here -
  measured `fatal error: 'features.h' file not found`), so the probe is

      $ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
          -I emacs/lib -include config.h -include libc-config.h -include cdefs.h -x c /dev/null
      (exit 0; 1467 `#define`s)

  and each arm is matched against it: the arm clang survives is emitted ungated, every other arm
  behind `#[cfg (feature = "<slug from its own condition>")]` with the condition quoted above it.
  Rust parses cfg'd-out macro arms, so the whole header is in the file (P-51's shape) and exactly
  one arm of each name is compiled.

- **Blind read, by measurement**: `python3 tools/check-cond-eval.py lib/cdefs.h` -> `85 conditions,
  9 unknown, 30 disagreements [NOT VERIFIED]`.  The disagreements are the
  `__has_attribute` / `__has_builtin` / `__GNUC_PREREQ` families - compiler capabilities the
  configuration table does not model - which is why the arms are carried instead of chosen by a
  table.

- **What the gate compares**: 122 macros and the two constants `_SYS_CDEFS_H` / `__LDBL_COMPAT`.
  Of the 125 `#define`s, 80 names have several arms and the gate asks only for their presence
  (their bodies are the configuration's business); the other 42 have one definition and their
  bodies are compared token by token, which is what the transcription above satisfies.  One
  body-note row was needed and is in `docs/ports/macro-body-notes.tsv`:
  `__fortify_clang_bos_static_lt_impl`'s `ull` (C's `-1ULL`; Rust has no such literal).

- **This header has no consumer in the port yet** - `rg -l cdefs emacs/` finds only
  `lib/libc-config.h`, whose Rust side (`femacs/lib/libc-config.rs`) declares the names it needs
  itself - so the compile evidence is that the mirror stands as its own crate root:
  `build-tu lib/cdefs.h` -> `built lib/include/cdefs.rs`.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/lib/cdefs.h femacs/lib/include/cdefs.rs
    OK emacs/lib/cdefs.h -> femacs/lib/include/cdefs.rs

    $ python3 tools/check-cond-eval.py lib/cdefs.h
    lib/cdefs.h: 85 conditions, 9 unknown, 30 disagreements [NOT VERIFIED]

    $ python3 tools/gen-cdefs-mirror.py --reference emacs/lib/cdefs.h --dM /tmp/cdefs-dM.txt --out /tmp/cdefs-draft.rs
    wrote /tmp/cdefs-draft.rs: 816 lines, 125 names, 235 arms

    $ cd femacs && cargo run -q -p xtask -- build-tu lib/cdefs.h
    built lib/include/cdefs.rs -> ../build/femacs/include_cdefs.o

    $ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
        -I emacs/lib -include config.h -include libc-config.h -include cdefs.h -x c /dev/null | wc -l
    1467

    $ clang -dM -E -std=gnu23 -I emacs/lib -include cdefs.h -x c /dev/null
    emacs/lib/cdefs.h:24:11: fatal error: 'features.h' file not found
