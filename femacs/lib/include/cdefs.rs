// femacs: mirror of lib/cdefs.h @ fd7afced403 (glibc's <sys/cdefs.h>; PORTING.md P-26/P-51).
//
// **How this file was produced, and why that is the faithful form.**  The reference is 900 lines
// of `#define` in which 83 of the 125 names carry one arm per compiler family and there is not
// one line of judgement in the text, so the transcription is mechanical and reproducible:
//
//     $ python3 tools/gen-cdefs-mirror.py --reference emacs/lib/cdefs.h \
//         --dM /tmp/cdefs-dM.txt --out femacs/lib/include/cdefs.rs
//
// The tool's rules are its contract and are repeated here: backslash continuations joined (they
// are not Rust tokens), block comments dropped, C's parameter names rewritten to Rust
// metavariables, `...` to `$($va:tt)*` and `__VA_ARGS__` to `$($va)*`, integer suffixes stripped
// (Rust has no `1ULL`, and `extract-abi.py`'s token comparison folds them), the arm the frozen
// clang takes emitted **ungated** and every other arm emitted behind
// `#[cfg (feature = "<slug from its own condition>")]` with the condition quoted above the item.
// Rust parses cfg'd-out macro arms, so the whole header is present in this file and exactly one
// arm of each name compiles - P-51's shape.
//
// **The live arm is measured, not the file order.**  The C reachability path is
// `lib/libc-config.h` C 181's `#include <cdefs.h>`: `<features.h>` does not exist on this target
// (measured: `clang … -include cdefs.h` -> `fatal error: 'features.h' file not found`), and
// libc-config.h is what supplies `_FEATURES_H`, `__GNULIB_CDEFS` and clang's
// `__GNUC_PREREQ(maj,min) = ((maj) < 4 + ((min) <= 2))`, so the measurement is
//
//     $ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
//         -I emacs/lib -include config.h -include libc-config.h -include cdefs.h -x c /dev/null
//     (exit 0, 1467 `#define`s; the demanded names' surviving definitions are what each arm
//      below was matched against - 33 of the 122 survive defined, the rest are the arms this
//      target does not take)
//
// `python3 tools/check-cond-eval.py lib/cdefs.h` reads `85 conditions, 9 unknown, 30 disagreements
// [NOT VERIFIED]`: this is a blind-read unit (the disagreements are the
// `__has_attribute`/`__has_builtin`/`__GNUC_PREREQ` families, i.e. compiler capabilities the
// configuration table does not model), which is why the arms are carried instead of chosen by a
// table.  The gate asks for 122 macros and the two constants `_SYS_CDEFS_H` / `__LDBL_COMPAT`;
// `docs/ports/include-cdefs.md` carries the box list and the readings.  Nothing in the port
// includes this mirror yet (`rg -l cdefs emacs/` finds only lib/libc-config.h, whose Rust side
// declares the names it needs itself), so its compile evidence is that it stands as its own
// crate root.

/* C 4: `#define _SYS_CDEFS_H 1` - the gate asks for it among the constants.  */

pub const _SYS_CDEFS_H: core::ffi::c_int = 1;

/* C 17: `#define __glibc_has_attribute(attr) __has_attribute (attr)` - under `(defined __has_attribute       && (!defined __clang_minor__           || (defined __apple_build_version__               ? 7000000 <= __apple_build_version__               : 5 <= __clang_major__)))`.  */
macro_rules! __glibc_has_attribute { ($attr:expr) => { __has_attribute ($attr) }; }

/* C 19: `#define __glibc_has_attribute(attr) 0` - under `!(defined __has_attribute       && (!defined __clang_minor__           || (defined __apple_build_version__               ? 7000000 <= __apple_build_version__               : 5 <= __clang_major__)))`.  */
#[cfg (feature = "no_has_attribute_defined_clang_minor_defined_apple_build_version_7000000_apple_build_version_5_clang_major")]
macro_rules! __glibc_has_attribute { ($attr:expr) => { 0 }; }

/* C 22: `#define __glibc_has_builtin(name) __has_builtin (name)` - under `defined __has_builtin`.  */
macro_rules! __glibc_has_builtin { ($name:expr) => { __has_builtin ($name) }; }

/* C 24: `#define __glibc_has_builtin(name) 0` - under `!defined __has_builtin`.  */
#[cfg (feature = "no_has_builtin")]
macro_rules! __glibc_has_builtin { ($name:expr) => { 0 }; }

/* C 27: `#define __glibc_has_extension(ext) __has_extension (ext)` - under `defined __has_extension`.  */
macro_rules! __glibc_has_extension { ($ext:expr) => { __has_extension ($ext) }; }

/* C 29: `#define __glibc_has_extension(ext) 0` - under `!defined __has_extension`.  */
#[cfg (feature = "no_has_extension")]
macro_rules! __glibc_has_extension { ($ext:expr) => { 0 }; }

/* C 36: `#define __LEAF , __leaf__` - under `__GNUC_PREREQ (4, 6) && !defined _LIBC`.  */
#[cfg (feature = "gnuc_prereq_4_6_defined_libc")]
macro_rules! __LEAF { () => { , __leaf__ }; }

/* C 39: `#define __LEAF ` - under `!__GNUC_PREREQ (4, 6) && !defined _LIBC`.  */
macro_rules! __LEAF { () => {  }; }

/* C 37: `#define __LEAF_ATTR __attribute__ ((__leaf__))` - under `__GNUC_PREREQ (4, 6) && !defined _LIBC`.  */
#[cfg (feature = "gnuc_prereq_4_6_defined_libc")]
macro_rules! __LEAF_ATTR { () => { __attribute__ ((__leaf__)) }; }

/* C 40: `#define __LEAF_ATTR ` - under `!__GNUC_PREREQ (4, 6) && !defined _LIBC`.  */
macro_rules! __LEAF_ATTR { () => {  }; }

/* C 45: `#define __THROW __attribute__ ((__nothrow__ __LEAF))` - under `!defined __cplusplus       && (__GNUC_PREREQ (3, 4) || __glibc_has_attribute (__nothrow__))`.  */
macro_rules! __THROW { () => { __attribute__ ((__nothrow__ __LEAF)) }; }

/* C 52: `#define __THROW noexcept (true)` - under `__cplusplus >= 201103L`.  */
#[cfg (feature = "cplusplus_201103l")]
macro_rules! __THROW { () => { noexcept (true) }; }

/* C 54: `#define __THROW throw ()` - under `!__cplusplus >= 201103L`.  */
#[cfg (feature = "cplusplus_201103l")]
macro_rules! __THROW { () => { throw () }; }

/* C 60: `#define __THROW ` - under `!defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "no_cplusplus_gnuc_prereq_2_8_clang_major_4")]
macro_rules! __THROW { () => {  }; }

/* C 81: `#define __THROW ` - under `!defined __GNUC__ || defined __clang__`.  */
#[cfg (feature = "no_gnuc_defined_clang")]
macro_rules! __THROW { () => {  }; }

/* C 46: `#define __THROWNL __attribute__ ((__nothrow__))` - under `!defined __cplusplus       && (__GNUC_PREREQ (3, 4) || __glibc_has_attribute (__nothrow__))`.  */
macro_rules! __THROWNL { () => { __attribute__ ((__nothrow__)) }; }

/* C 56: `#define __THROWNL __THROW` - under `defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "cplusplus_gnuc_prereq_2_8_clang")]
macro_rules! __THROWNL { () => { __THROW }; }

/* C 61: `#define __THROWNL ` - under `!defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "no_cplusplus_gnuc_prereq_2_8_clang_major_4")]
macro_rules! __THROWNL { () => {  }; }

/* C 82: `#define __THROWNL ` - under `!defined __GNUC__ || defined __clang__`.  */
#[cfg (feature = "no_gnuc_defined_clang")]
macro_rules! __THROWNL { () => {  }; }

/* C 47: `#define __NTH(fct) __attribute__ ((__nothrow__ __LEAF)) fct` - under `!defined __cplusplus       && (__GNUC_PREREQ (3, 4) || __glibc_has_attribute (__nothrow__))`.  */
macro_rules! __NTH { ($fct:expr) => { __attribute__ ((__nothrow__ __LEAF)) $fct }; }

/* C 57: `#define __NTH(fct) __LEAF_ATTR fct __THROW` - under `defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "cplusplus_gnuc_prereq_2_8_clang")]
macro_rules! __NTH { ($fct:expr) => { __LEAF_ATTR $fct __THROW }; }

/* C 62: `#define __NTH(fct) fct` - under `!defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "no_cplusplus_gnuc_prereq_2_8_clang_major_4")]
macro_rules! __NTH { ($fct:expr) => { $fct }; }

/* C 83: `#define __NTH(fct) fct` - under `!defined __GNUC__ || defined __clang__`.  */
#[cfg (feature = "no_gnuc_defined_clang")]
macro_rules! __NTH { ($fct:expr) => { $fct }; }

/* C 48: `#define __NTHNL(fct) __attribute__ ((__nothrow__)) fct` - under `!defined __cplusplus       && (__GNUC_PREREQ (3, 4) || __glibc_has_attribute (__nothrow__))`.  */
macro_rules! __NTHNL { ($fct:expr) => { __attribute__ ((__nothrow__)) $fct }; }

/* C 58: `#define __NTHNL(fct) fct __THROW` - under `defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "cplusplus_gnuc_prereq_2_8_clang")]
macro_rules! __NTHNL { ($fct:expr) => { $fct __THROW }; }

/* C 63: `#define __NTHNL(fct) fct` - under `!defined __cplusplus && (__GNUC_PREREQ (2,8) || __clang_major__ >= 4)`.  */
#[cfg (feature = "no_cplusplus_gnuc_prereq_2_8_clang_major_4")]
macro_rules! __NTHNL { ($fct:expr) => { $fct }; }

/* C 68: `#define __COLD __attribute__ ((__cold__))` - under `__GNUC_PREREQ (4, 3) || __glibc_has_attribute (__cold__)`.  */
macro_rules! __COLD { () => { __attribute__ ((__cold__)) }; }

/* C 70: `#define __COLD ` - under `!__GNUC_PREREQ (4, 3) || __glibc_has_attribute (__cold__)`.  */
#[cfg (feature = "gnuc_prereq_4_3_glibc_has")]
macro_rules! __COLD { () => {  }; }

/* C 84: `#define __COLD ` - under `!defined __GNUC__ || defined __clang__`.  */
#[cfg (feature = "no_gnuc_defined_clang")]
macro_rules! __COLD { () => {  }; }

/* C 76: `#define __inline inline` - under `(defined __cplusplus						       || (defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L))`.  */
#[cfg (feature = "cplusplus_defined_stdc_version_stdc_version")]
macro_rules! __inline { () => { inline }; }

/* C 78: `#define __inline ` - under `!(defined __cplusplus						       || (defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L))`.  */
macro_rules! __inline { () => {  }; }

/* C 89: `#define __P(args) args` - under `!defined _SYS_CDEFS_H`.  */
macro_rules! __P { ($args:expr) => { $args }; }

/* C 90: `#define __PMT(args) args` - under `!defined _SYS_CDEFS_H`.  */
macro_rules! __PMT { ($args:expr) => { $args }; }

/* C 94: `#define __CONCAT(x,y) x ## y` - under `!defined _SYS_CDEFS_H`.  */
macro_rules! __CONCAT { ($x:expr, $y:expr) => { $x  # #  $y }; }

/* C 95: `#define __STRING(x) #x` - under `!defined _SYS_CDEFS_H`.  */
macro_rules! __STRING { ($x:expr) => { #$x }; }

/* C 98: `#define __ptr_t void *` - under `!defined _SYS_CDEFS_H`.  */
macro_rules! __ptr_t { () => { void * }; }

/* C 103: `#define __BEGIN_DECLS extern "C" {` - under `defined __cplusplus`.  */
#[cfg (feature = "cplusplus")]
macro_rules! __BEGIN_DECLS { () => { extern "C" { }; }

/* C 106: `#define __BEGIN_DECLS ` - under `!defined __cplusplus`.  */
macro_rules! __BEGIN_DECLS { () => {  }; }

/* C 104: `#define __END_DECLS }` - under `defined __cplusplus`.  */
#[cfg (feature = "cplusplus")]
macro_rules! __END_DECLS { () => { } }; }

/* C 107: `#define __END_DECLS ` - under `!defined __cplusplus`.  */
macro_rules! __END_DECLS { () => {  }; }

/* C 113: `#define __attribute_overloadable__ __attribute__((__overloadable__))` - under `defined __clang_major__      && (__clang_major__ + (__clang_minor__ >= 6) > 2)`.  */
macro_rules! __attribute_overloadable__ { () => { __attribute__((__overloadable__)) }; }

/* C 115: `#define __attribute_overloadable__ ` - under `!defined __clang_major__      && (__clang_major__ + (__clang_minor__ >= 6) > 2)`.  */
#[cfg (feature = "no_clang_major_clang_major_clang_minor_6_2")]
macro_rules! __attribute_overloadable__ { () => {  }; }

/* C 122: `#define __bos(ptr) __builtin_object_size (ptr, __USE_FORTIFY_LEVEL > 1)` - under `!defined __GNULIB_CDEFS`.  */
macro_rules! __bos { ($ptr:expr) => { __builtin_object_size ($ptr, __USE_FORTIFY_LEVEL > 1) }; }

/* C 123: `#define __bos0(ptr) __builtin_object_size (ptr, 0)` - under `!defined __GNULIB_CDEFS`.  */
macro_rules! __bos0 { ($ptr:expr) => { __builtin_object_size ($ptr, 0) }; }

/* C 127: `#define __glibc_objsize0(__o) __builtin_dynamic_object_size (__o, 0)` - under `__USE_FORTIFY_LEVEL == 3 && (__glibc_clang_prereq (9, 0)		       				 || __GNUC_PREREQ (12, 0))`.  */
#[cfg (feature = "use_fortify_level_3_glibc_clang")]
macro_rules! __glibc_objsize0 { ($__o:expr) => { __builtin_dynamic_object_size ($__o, 0) }; }

/* C 130: `#define __glibc_objsize0(__o) __bos0 (__o)` - under `!__USE_FORTIFY_LEVEL == 3 && (__glibc_clang_prereq (9, 0)		       				 || __GNUC_PREREQ (12, 0))`.  */
macro_rules! __glibc_objsize0 { ($__o:expr) => { __bos0 ($__o) }; }

/* C 128: `#define __glibc_objsize(__o) __builtin_dynamic_object_size (__o, 1)` - under `__USE_FORTIFY_LEVEL == 3 && (__glibc_clang_prereq (9, 0)		       				 || __GNUC_PREREQ (12, 0))`.  */
#[cfg (feature = "use_fortify_level_3_glibc_clang")]
macro_rules! __glibc_objsize { ($__o:expr) => { __builtin_dynamic_object_size ($__o, 1) }; }

/* C 131: `#define __glibc_objsize(__o) __bos (__o)` - under `!__USE_FORTIFY_LEVEL == 3 && (__glibc_clang_prereq (9, 0)		       				 || __GNUC_PREREQ (12, 0))`.  */
macro_rules! __glibc_objsize { ($__o:expr) => { __bos ($__o) }; }

/* C 137: `#define __glibc_safe_len_cond(__l, __s, __osz) ((__l) <= (__osz) / (__s))` - under `__USE_FORTIFY_LEVEL > 0`.  */
macro_rules! __glibc_safe_len_cond { ($__l:expr, $__s:expr, $__osz:expr) => { (($__l) <= ($__osz) / ($__s)) }; }

/* C 138: `#define __glibc_unsigned_or_positive(__l) ((__typeof (__l)) 0 < (__typeof (__l)) -1				          || (__builtin_constant_p (__l) && (__l) > 0))` - under `__USE_FORTIFY_LEVEL > 0`.  */
macro_rules! __glibc_unsigned_or_positive { ($__l:expr) => { ((__typeof ($__l)) 0 < (__typeof ($__l)) -1				          || (__builtin_constant_p ($__l) && ($__l) > 0)) }; }

/* C 141: `#define __glibc_safe_or_unknown_len(__l, __s, __osz) ((__builtin_constant_p (__osz) && (__osz) == (__SIZE_TYPE__) -1)	          || (__glibc_unsigned_or_positive (__l)				              && __builtin_constant_p (__glibc_safe_len_cond ((__SIZE_TYPE__) (__l),  						       (__s), (__osz)))	              && __glibc_safe_len_cond ((__SIZE_TYPE__) (__l), (__s), (__osz))))` - under `__USE_FORTIFY_LEVEL > 0`.  */
macro_rules! __glibc_safe_or_unknown_len { ($__l:expr, $__s:expr, $__osz:expr) => { ((__builtin_constant_p ($__osz) && ($__osz) == (__SIZE_TYPE__) -1)	          || (__glibc_unsigned_or_positive ($__l)				              && __builtin_constant_p (__glibc_safe_len_cond ((__SIZE_TYPE__) ($__l),  						       ($__s), ($__osz)))	              && __glibc_safe_len_cond ((__SIZE_TYPE__) ($__l), ($__s), ($__osz)))) }; }

/* C 144: `#define __glibc_unsafe_len(__l, __s, __osz) (__glibc_unsigned_or_positive (__l)					          && __builtin_constant_p (__glibc_safe_len_cond ((__SIZE_TYPE__) (__l),      						   __s, __osz))		          && !__glibc_safe_len_cond ((__SIZE_TYPE__) (__l), __s, __osz))` - under `__USE_FORTIFY_LEVEL > 0`.  */
macro_rules! __glibc_unsafe_len { ($__l:expr, $__s:expr, $__osz:expr) => { (__glibc_unsigned_or_positive ($__l)					          && __builtin_constant_p (__glibc_safe_len_cond ((__SIZE_TYPE__) ($__l),      						   $__s, $__osz))		          && !__glibc_safe_len_cond ((__SIZE_TYPE__) ($__l), $__s, $__osz)) }; }

/* C 148: `#define __fortify_use_clang 1` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_use_clang { () => { 1 }; }

/* C 199: `#define __fortify_use_clang 0` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_use_clang { () => { 0 }; }

/* C 150: `#define __fortify_function_error_function static __attribute__((__unused__))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_function_error_function { () => { static __attribute__((__unused__)) }; }

/* C 152: `#define __fortify_clang_pass_object_size_n(n) __attribute__ ((__pass_object_size__ (n)))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_pass_object_size_n { ($n:expr) => { __attribute__ ((__pass_object_size__ ($n))) }; }

/* C 153: `#define __fortify_clang_pass_object_size0 __fortify_clang_pass_object_size_n (0)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_pass_object_size0 { () => { __fortify_clang_pass_object_size_n (0) }; }

/* C 154: `#define __fortify_clang_pass_object_size __fortify_clang_pass_object_size_n (__USE_FORTIFY_LEVEL > 1)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_pass_object_size { () => { __fortify_clang_pass_object_size_n (__USE_FORTIFY_LEVEL > 1) }; }

/* C 157: `#define __fortify_clang_pass_dynamic_object_size_n(n) __attribute__ ((__pass_dynamic_object_size__ (n)))` - under `__clang_major__ >= 9`.  */
#[cfg (feature = "clang_major_9")]
macro_rules! __fortify_clang_pass_dynamic_object_size_n { ($n:expr) => { __attribute__ ((__pass_dynamic_object_size__ ($n))) }; }

/* C 161: `#define __fortify_clang_pass_dynamic_object_size_n(n) ` - under `!__clang_major__ >= 9`.  */
macro_rules! __fortify_clang_pass_dynamic_object_size_n { ($n:expr) => {  }; }

/* C 158: `#define __fortify_clang_pass_dynamic_object_size0 __fortify_clang_pass_dynamic_object_size_n (0)` - under `__clang_major__ >= 9`.  */
#[cfg (feature = "clang_major_9")]
macro_rules! __fortify_clang_pass_dynamic_object_size0 { () => { __fortify_clang_pass_dynamic_object_size_n (0) }; }

/* C 162: `#define __fortify_clang_pass_dynamic_object_size0 ` - under `!__clang_major__ >= 9`.  */
macro_rules! __fortify_clang_pass_dynamic_object_size0 { () => {  }; }

/* C 159: `#define __fortify_clang_pass_dynamic_object_size __fortify_clang_pass_dynamic_object_size_n (1)` - under `__clang_major__ >= 9`.  */
#[cfg (feature = "clang_major_9")]
macro_rules! __fortify_clang_pass_dynamic_object_size { () => { __fortify_clang_pass_dynamic_object_size_n (1) }; }

/* C 163: `#define __fortify_clang_pass_dynamic_object_size ` - under `!__clang_major__ >= 9`.  */
macro_rules! __fortify_clang_pass_dynamic_object_size { () => {  }; }

/* C 166: `#define __fortify_clang_bos_static_lt_impl(bos_val, n, s) ((bos_val) != -1ULL && (n) > (bos_val) / (s))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bos_static_lt_impl { ($bos_val:expr, $n:expr, $s:expr) => { (($bos_val) != -1 && ($n) > ($bos_val) / ($s)) }; }

/* C 167: `#define __fortify_clang_bos_static_lt2(__n, __e, __s) __fortify_clang_bos_static_lt_impl (__bos (__e), __n, __s)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bos_static_lt2 { ($__n:expr, $__e:expr, $__s:expr) => { __fortify_clang_bos_static_lt_impl (__bos ($__e), $__n, $__s) }; }

/* C 168: `#define __fortify_clang_bos_static_lt(__n, __e) __fortify_clang_bos_static_lt2 (__n, __e, 1)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bos_static_lt { ($__n:expr, $__e:expr) => { __fortify_clang_bos_static_lt2 ($__n, $__e, 1) }; }

/* C 169: `#define __fortify_clang_bos0_static_lt2(__n, __e, __s) __fortify_clang_bos_static_lt_impl (__bos0 (__e), __n, __s)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bos0_static_lt2 { ($__n:expr, $__e:expr, $__s:expr) => { __fortify_clang_bos_static_lt_impl (__bos0 ($__e), $__n, $__s) }; }

/* C 170: `#define __fortify_clang_bos0_static_lt(__n, __e) __fortify_clang_bos0_static_lt2 (__n, __e, 1)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bos0_static_lt { ($__n:expr, $__e:expr) => { __fortify_clang_bos0_static_lt2 ($__n, $__e, 1) }; }

/* C 172: `#define __fortify_clang_bosn_args(bos_fn, n, buf, div, complaint) (__fortify_clang_bos_static_lt_impl (bos_fn (buf), n, div)), (complaint),    "warning"` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_bosn_args { ($bos_fn:expr, $n:expr, $buf:expr, $div:expr, $complaint:expr) => { (__fortify_clang_bos_static_lt_impl ($bos_fn ($buf), $n, $div)), ($complaint),    "warning" }; }

/* C 174: `#define __fortify_clang_warning(__c, __msg) __attribute__ ((__diagnose_if__ ((__c), (__msg), "warning")))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warning { ($__c:expr, $__msg:expr) => { __attribute__ ((__diagnose_if__ (($__c), ($__msg), "warning"))) }; }

/* C 200: `#define __fortify_clang_warning(__c, __msg) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warning { ($__c:expr, $__msg:expr) => {  }; }

/* C 175: `#define __fortify_clang_error(__c, __msg) __attribute__ ((__diagnose_if__ ((__c), (__msg), "error")))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_error { ($__c:expr, $__msg:expr) => { __attribute__ ((__diagnose_if__ (($__c), ($__msg), "error"))) }; }

/* C 176: `#define __fortify_clang_warning_only_if_bos0_lt(n, buf, complaint) __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos0, n, buf, 1, complaint))))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warning_only_if_bos0_lt { ($n:expr, $buf:expr, $complaint:expr) => { __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos0, $n, $buf, 1, $complaint)))) }; }

/* C 201: `#define __fortify_clang_warning_only_if_bos0_lt(__n, __buf, __complaint) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warning_only_if_bos0_lt { ($__n:expr, $__buf:expr, $__complaint:expr) => {  }; }

/* C 177: `#define __fortify_clang_warning_only_if_bos0_lt2(n, buf, div, complaint) __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos0, n, buf, div, complaint))))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warning_only_if_bos0_lt2 { ($n:expr, $buf:expr, $div:expr, $complaint:expr) => { __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos0, $n, $buf, $div, $complaint)))) }; }

/* C 202: `#define __fortify_clang_warning_only_if_bos0_lt2(__n, __buf, __div, complaint) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warning_only_if_bos0_lt2 { ($__n:expr, $__buf:expr, $__div:expr, $complaint:expr) => {  }; }

/* C 178: `#define __fortify_clang_warning_only_if_bos_lt(n, buf, complaint) __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos, n, buf, 1, complaint))))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warning_only_if_bos_lt { ($n:expr, $buf:expr, $complaint:expr) => { __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos, $n, $buf, 1, $complaint)))) }; }

/* C 203: `#define __fortify_clang_warning_only_if_bos_lt(__n, __buf, __complaint) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warning_only_if_bos_lt { ($__n:expr, $__buf:expr, $__complaint:expr) => {  }; }

/* C 179: `#define __fortify_clang_warning_only_if_bos_lt2(n, buf, div, complaint) __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos, n, buf, div, complaint))))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warning_only_if_bos_lt2 { ($n:expr, $buf:expr, $div:expr, $complaint:expr) => { __attribute__ ((__diagnose_if__  		  (__fortify_clang_bosn_args (__bos, $n, $buf, $div, $complaint)))) }; }

/* C 204: `#define __fortify_clang_warning_only_if_bos_lt2(__n, __buf, div, __complaint) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warning_only_if_bos_lt2 { ($__n:expr, $__buf:expr, $div:expr, $__complaint:expr) => {  }; }

/* C 181: `#define __fortify_clang_prefer_this_overload __attribute__ ((enable_if (1, "")))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_prefer_this_overload { () => { __attribute__ ((enable_if (1, ""))) }; }

/* C 182: `#define __fortify_clang_unavailable(__msg) __attribute__ ((unavailable(__msg)))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_unavailable { ($__msg:expr) => { __attribute__ ((unavailable($__msg))) }; }

/* C 185: `#define __fortify_clang_overload_arg(__type, __attr, __name) __type __attr const __fortify_clang_pass_dynamic_object_size __name` - under `__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortify_clang_overload_arg { ($__type:expr, $__attr:expr, $__name:expr) => { $__type $__attr const __fortify_clang_pass_dynamic_object_size $__name }; }

/* C 188: `#define __fortify_clang_overload_arg(__type, __attr, __name) __type __attr const __fortify_clang_pass_object_size __name` - under `!__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortify_clang_overload_arg { ($__type:expr, $__attr:expr, $__name:expr) => { $__type $__attr const __fortify_clang_pass_object_size $__name }; }

/* C 205: `#define __fortify_clang_overload_arg(__type, __attr, __name) __type __attr __name` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_overload_arg { ($__type:expr, $__attr:expr, $__name:expr) => { $__type $__attr $__name }; }

/* C 186: `#define __fortify_clang_overload_arg0(__type, __attr, __name) __type __attr const __fortify_clang_pass_dynamic_object_size0 __name` - under `__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortify_clang_overload_arg0 { ($__type:expr, $__attr:expr, $__name:expr) => { $__type $__attr const __fortify_clang_pass_dynamic_object_size0 $__name }; }

/* C 189: `#define __fortify_clang_overload_arg0(__type, __attr, __name) __type __attr const __fortify_clang_pass_object_size0 __name` - under `!__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortify_clang_overload_arg0 { ($__type:expr, $__attr:expr, $__name:expr) => { $__type $__attr const __fortify_clang_pass_object_size0 $__name }; }

/* C 206: `#define __fortify_clang_overload_arg0(__type, __attr, __name) __fortify_clang_overload_arg (__type, __attr, __name)` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_overload_arg0 { ($__type:expr, $__attr:expr, $__name:expr) => { __fortify_clang_overload_arg ($__type, $__attr, $__name) }; }

/* C 192: `#define __fortify_clang_mul_may_overflow(size, n) ((size | n) >= (((size_t)1) << (8 * sizeof (size_t) / 2)))` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_mul_may_overflow { ($size:expr, $n:expr) => { (($size | $n) >= (((size_t)1) << (8 * sizeof (size_t) / 2))) }; }

/* C 194: `#define __fortify_clang_size_too_small(__bos, __dest, __len) (__bos (__dest) != (size_t) -1 && __bos (__dest) < __len)` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_size_too_small { ($__bos:expr, $__dest:expr, $__len:expr) => { ($__bos ($__dest) != (size_t) -1 && $__bos ($__dest) < $__len) }; }

/* C 195: `#define __fortify_clang_warn_if_src_too_large(__dest, __src) __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize,  							   __dest,  							   __builtin_strlen (__src) + 1),  			   "destination buffer will always be overflown by source")` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warn_if_src_too_large { ($__dest:expr, $__src:expr) => { __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize,  							   $__dest,  							   __builtin_strlen ($__src) + 1),  			   "destination buffer will always be overflown by source") }; }

/* C 207: `#define __fortify_clang_warn_if_src_too_large(__dest, __src) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warn_if_src_too_large { ($__dest:expr, $__src:expr) => {  }; }

/* C 196: `#define __fortify_clang_warn_if_dest_too_small(__dest, __len) __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize,                                                             __dest,                                                             __len),                             "function called with bigger length than the destination buffer")` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warn_if_dest_too_small { ($__dest:expr, $__len:expr) => { __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize,                                                             $__dest,                                                             $__len),                             "function called with bigger length than the destination buffer") }; }

/* C 208: `#define __fortify_clang_warn_if_dest_too_small(__dest, __len) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warn_if_dest_too_small { ($__dest:expr, $__len:expr) => {  }; }

/* C 197: `#define __fortify_clang_warn_if_dest_too_small0(__dest, __len) __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize0,                                                             __dest,                                                             __len),                             "function called with bigger length than the destination buffer")` - under `defined __clang_major__ && __clang_major__ >= 5`.  */
#[cfg (feature = "clang_major_clang_major_5")]
macro_rules! __fortify_clang_warn_if_dest_too_small0 { ($__dest:expr, $__len:expr) => { __fortify_clang_warning (__fortify_clang_size_too_small (__glibc_objsize0,                                                             $__dest,                                                             $__len),                             "function called with bigger length than the destination buffer") }; }

/* C 209: `#define __fortify_clang_warn_if_dest_too_small0(__dest, __len) ` - under `!defined __clang_major__ && __clang_major__ >= 5`.  */
macro_rules! __fortify_clang_warn_if_dest_too_small0 { ($__dest:expr, $__len:expr) => {  }; }

/* C 216: `#define __glibc_fortify(f, __l, __s, __osz, ...) (__glibc_safe_or_unknown_len (__l, __s, __osz)			          ? __ ## f ## _alias (__VA_ARGS__)					          : (__glibc_unsafe_len (__l, __s, __osz)				             ? __ ## f ## _chk_warn (__VA_ARGS__, __osz)			             : __ ## f ## _chk (__VA_ARGS__, __osz)))` - under `!__fortify_use_clang`.  */
#[cfg (feature = "fortify_use_clang")]
macro_rules! __glibc_fortify { ($f:expr, $__l:expr, $__s:expr, $__osz:expr, $($va:tt)*) => { (__glibc_safe_or_unknown_len ($__l, $__s, $__osz)			          ? __  # #  $f  # #  _alias ($($va)*)					          : (__glibc_unsafe_len ($__l, $__s, $__osz)				             ? __  # #  $f  # #  _chk_warn ($($va)*, $__osz)			             : __  # #  $f  # #  _chk ($($va)*, $__osz))) }; }

/* C 218: `#define __glibc_fortify(f, __l, __s, __osz, ...) (__osz == (__SIZE_TYPE__) -1)						          ? __ ## f ## _alias (__VA_ARGS__)					          : __ ## f ## _chk (__VA_ARGS__, __osz)` - under `!!__fortify_use_clang`.  */
macro_rules! __glibc_fortify { ($f:expr, $__l:expr, $__s:expr, $__osz:expr, $($va:tt)*) => { ($__osz == (__SIZE_TYPE__) -1)						          ? __  # #  $f  # #  _alias ($($va)*)					          : __  # #  $f  # #  _chk ($($va)*, $__osz) }; }

/* C 224: `#define __glibc_fortify_n(f, __l, __s, __osz, ...) (__glibc_safe_or_unknown_len (__l, __s, __osz)			          ? __ ## f ## _alias (__VA_ARGS__)					          : (__glibc_unsafe_len (__l, __s, __osz)				             ? __ ## f ## _chk_warn (__VA_ARGS__, (__osz) / (__s))		             : __ ## f ## _chk (__VA_ARGS__, (__osz) / (__s))))` - under `!__fortify_use_clang`.  */
#[cfg (feature = "fortify_use_clang")]
macro_rules! __glibc_fortify_n { ($f:expr, $__l:expr, $__s:expr, $__osz:expr, $($va:tt)*) => { (__glibc_safe_or_unknown_len ($__l, $__s, $__osz)			          ? __  # #  $f  # #  _alias ($($va)*)					          : (__glibc_unsafe_len ($__l, $__s, $__osz)				             ? __  # #  $f  # #  _chk_warn ($($va)*, ($__osz) / ($__s))		             : __  # #  $f  # #  _chk ($($va)*, ($__osz) / ($__s)))) }; }

/* C 226: `#define __glibc_fortify_n(f, __l, __s, __osz, ...) (__osz == (__SIZE_TYPE__) -1)						          ? __ ## f ## _alias (__VA_ARGS__)					          : __ ## f ## _chk (__VA_ARGS__, (__osz) / (__s))` - under `!!__fortify_use_clang`.  */
macro_rules! __glibc_fortify_n { ($f:expr, $__l:expr, $__s:expr, $__osz:expr, $($va:tt)*) => { ($__osz == (__SIZE_TYPE__) -1)						          ? __  # #  $f  # #  _alias ($($va)*)					          : __  # #  $f  # #  _chk ($($va)*, ($__osz) / ($__s)) }; }

/* C 233: `#define __warnattr(msg) __attribute__((__warning__ (msg)))` - under `__GNUC_PREREQ (4,3)`.  */
#[cfg (feature = "gnuc_prereq_4_3")]
macro_rules! __warnattr { ($msg:expr) => { __attribute__((__warning__ ($msg))) }; }

/* C 236: `#define __warnattr(msg) ` - under `!__GNUC_PREREQ (4,3)`.  */
macro_rules! __warnattr { ($msg:expr) => {  }; }

/* C 234: `#define __errordecl(name, msg) extern void name (void) __attribute__((__error__ (msg)))` - under `__GNUC_PREREQ (4,3)`.  */
#[cfg (feature = "gnuc_prereq_4_3")]
macro_rules! __errordecl { ($name:expr, $msg:expr) => { extern void $name (void) __attribute__((__error__ ($msg))) }; }

/* C 237: `#define __errordecl(name, msg) extern void name (void)` - under `!__GNUC_PREREQ (4,3)`.  */
macro_rules! __errordecl { ($name:expr, $msg:expr) => { extern void $name (void) }; }

/* C 242: `#define __flexarr []` - under `defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L && !defined __HP_cc`.  */
#[cfg (feature = "stdc_version_stdc_version_199901l_defined")]
macro_rules! __flexarr { () => { [] }; }

/* C 246: `#define __flexarr []` - under `__GNUC_PREREQ (2,97) || defined __clang__`.  */
macro_rules! __flexarr { () => { [] }; }

/* C 250: `#define __flexarr [0]` - under `defined __GNUC__`.  */
#[cfg (feature = "gnuc")]
macro_rules! __flexarr { () => { [0] }; }

/* C 254: `#define __flexarr [1]` - under `!defined __GNUC__`.  */
#[cfg (feature = "no_gnuc")]
macro_rules! __flexarr { () => { [1] }; }

/* C 243: `#define __glibc_c99_flexarr_available 1` - under `defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L && !defined __HP_cc`.  */
#[cfg (feature = "stdc_version_stdc_version_199901l_defined")]
macro_rules! __glibc_c99_flexarr_available { () => { 1 }; }

/* C 247: `#define __glibc_c99_flexarr_available 1` - under `__GNUC_PREREQ (2,97) || defined __clang__`.  */
#[cfg (feature = "gnuc_prereq_2_97_defined_clang")]
macro_rules! __glibc_c99_flexarr_available { () => { 1 }; }

/* C 251: `#define __glibc_c99_flexarr_available 1` - under `defined __GNUC__`.  */
macro_rules! __glibc_c99_flexarr_available { () => { 1 }; }

/* C 255: `#define __glibc_c99_flexarr_available 0` - under `!defined __GNUC__`.  */
#[cfg (feature = "no_gnuc")]
macro_rules! __glibc_c99_flexarr_available { () => { 0 }; }

/* C 263: `#define __REDIRECT(name, proto, alias) name proto __asm__ (__ASMNAME (#alias))` - under `(defined __GNUC__ && __GNUC__ >= 2) || (__clang_major__ >= 4)`.  */
macro_rules! __REDIRECT { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __asm__ (__ASMNAME (#$alias)) }; }

/* C 265: `#define __REDIRECT_NTH(name, proto, alias) name proto __THROW __asm__ (__ASMNAME (#alias))` - under `defined __cplusplus`.  */
#[cfg (feature = "cplusplus")]
macro_rules! __REDIRECT_NTH { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __THROW __asm__ (__ASMNAME (#$alias)) }; }

/* C 268: `#define __REDIRECT_NTH(name, proto, alias) name proto __asm__ (__ASMNAME (#alias)) __THROW` - under `!defined __cplusplus`.  */
macro_rules! __REDIRECT_NTH { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __asm__ (__ASMNAME (#$alias)) __THROW }; }

/* C 266: `#define __REDIRECT_NTHNL(name, proto, alias) name proto __THROWNL __asm__ (__ASMNAME (#alias))` - under `defined __cplusplus`.  */
#[cfg (feature = "cplusplus")]
macro_rules! __REDIRECT_NTHNL { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __THROWNL __asm__ (__ASMNAME (#$alias)) }; }

/* C 269: `#define __REDIRECT_NTHNL(name, proto, alias) name proto __asm__ (__ASMNAME (#alias)) __THROWNL` - under `!defined __cplusplus`.  */
macro_rules! __REDIRECT_NTHNL { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __asm__ (__ASMNAME (#$alias)) __THROWNL }; }

/* C 271: `#define __ASMNAME(cname) __ASMNAME2 (__USER_LABEL_PREFIX__, cname)` - under `(defined __GNUC__ && __GNUC__ >= 2) || (__clang_major__ >= 4)`.  */
macro_rules! __ASMNAME { ($cname:expr) => { __ASMNAME2 (__USER_LABEL_PREFIX__, $cname) }; }

/* C 272: `#define __ASMNAME2(prefix, cname) __STRING (prefix) cname` - under `(defined __GNUC__ && __GNUC__ >= 2) || (__clang_major__ >= 4)`.  */
macro_rules! __ASMNAME2 { ($prefix:expr, $cname:expr) => { __STRING ($prefix) $cname }; }

/* C 275: `#define __REDIRECT_FORTIFY __REDIRECT` - under `!defined __REDIRECT_FORTIFY`.  */
macro_rules! __REDIRECT_FORTIFY { () => { __REDIRECT }; }

/* C 279: `#define __REDIRECT_FORTIFY_NTH __REDIRECT_NTH` - under `!defined __REDIRECT_FORTIFY_NTH`.  */
macro_rules! __REDIRECT_FORTIFY_NTH { () => { __REDIRECT_NTH }; }

/* C 287: `#define __attribute__(xyz) ` - under `!(defined __GNUC__ || defined __clang__ || defined __TINYC__)`.  */
macro_rules! __attribute__ { ($xyz:expr) => {  }; }

/* C 292: `#define __attribute_malloc__ __attribute__ ((__malloc__))` - under `__GNUC_PREREQ (2,96) || __glibc_has_attribute (__malloc__)`.  */
macro_rules! __attribute_malloc__ { () => { __attribute__ ((__malloc__)) }; }

/* C 294: `#define __attribute_malloc__ ` - under `!__GNUC_PREREQ (2,96) || __glibc_has_attribute (__malloc__)`.  */
#[cfg (feature = "gnuc_prereq_2_96_glibc_has")]
macro_rules! __attribute_malloc__ { () => {  }; }

/* C 299: `#define __attribute_alloc_size__(params) __attribute__ ((__alloc_size__ params))` - under `__GNUC_PREREQ (4, 3)`.  */
#[cfg (feature = "gnuc_prereq_4_3")]
macro_rules! __attribute_alloc_size__ { ($params:expr) => { __attribute__ ((__alloc_size__ $params)) }; }

/* C 301: `#define __attribute_alloc_size__(params) ` - under `!__GNUC_PREREQ (4, 3)`.  */
macro_rules! __attribute_alloc_size__ { ($params:expr) => {  }; }

/* C 306: `#define __attribute_alloc_align__(param) __attribute__ ((__alloc_align__ param))` - under `__GNUC_PREREQ (4, 9) || __glibc_has_attribute (__alloc_align__)`.  */
macro_rules! __attribute_alloc_align__ { ($param:expr) => { __attribute__ ((__alloc_align__ $param)) }; }

/* C 308: `#define __attribute_alloc_align__(param) ` - under `!__GNUC_PREREQ (4, 9) || __glibc_has_attribute (__alloc_align__)`.  */
#[cfg (feature = "gnuc_prereq_4_9_glibc_has")]
macro_rules! __attribute_alloc_align__ { ($param:expr) => {  }; }

/* C 313: `#define __attribute_pure__ __attribute__ ((__pure__))` - under `__GNUC_PREREQ (2,96) || __glibc_has_attribute (__pure__)`.  */
macro_rules! __attribute_pure__ { () => { __attribute__ ((__pure__)) }; }

/* C 315: `#define __attribute_pure__ ` - under `!__GNUC_PREREQ (2,96) || __glibc_has_attribute (__pure__)`.  */
#[cfg (feature = "gnuc_prereq_2_96_glibc_has")]
macro_rules! __attribute_pure__ { () => {  }; }

/* C 320: `#define __attribute_const__ __attribute__ ((__const__))` - under `__GNUC_PREREQ (2,5) || __glibc_has_attribute (__const__)`.  */
macro_rules! __attribute_const__ { () => { __attribute__ ((__const__)) }; }

/* C 322: `#define __attribute_const__ ` - under `!__GNUC_PREREQ (2,5) || __glibc_has_attribute (__const__)`.  */
#[cfg (feature = "gnuc_prereq_2_5_glibc_has")]
macro_rules! __attribute_const__ { () => {  }; }

/* C 326: `#define __attribute_maybe_unused__ __attribute__ ((__unused__))` - under `__GNUC_PREREQ (2,7) || __glibc_has_attribute (__unused__)`.  */
macro_rules! __attribute_maybe_unused__ { () => { __attribute__ ((__unused__)) }; }

/* C 328: `#define __attribute_maybe_unused__ ` - under `!__GNUC_PREREQ (2,7) || __glibc_has_attribute (__unused__)`.  */
#[cfg (feature = "gnuc_prereq_2_7_glibc_has")]
macro_rules! __attribute_maybe_unused__ { () => {  }; }

/* C 333: `#define __attribute_used__ __attribute__ ((__used__))` - under `__GNUC_PREREQ (3,1) || __glibc_has_attribute (__used__)`.  */
macro_rules! __attribute_used__ { () => { __attribute__ ((__used__)) }; }

/* C 336: `#define __attribute_used__ __attribute__ ((__unused__))` - under `!__GNUC_PREREQ (3,1) || __glibc_has_attribute (__used__)`.  */
#[cfg (feature = "gnuc_prereq_3_1_glibc_has")]
macro_rules! __attribute_used__ { () => { __attribute__ ((__unused__)) }; }

/* C 334: `#define __attribute_noinline__ __attribute__ ((__noinline__))` - under `__GNUC_PREREQ (3,1) || __glibc_has_attribute (__used__)`.  */
macro_rules! __attribute_noinline__ { () => { __attribute__ ((__noinline__)) }; }

/* C 337: `#define __attribute_noinline__ ` - under `!__GNUC_PREREQ (3,1) || __glibc_has_attribute (__used__)`.  */
#[cfg (feature = "gnuc_prereq_3_1_glibc_has")]
macro_rules! __attribute_noinline__ { () => {  }; }

/* C 342: `#define __attribute_deprecated__ __attribute__ ((__deprecated__))` - under `__GNUC_PREREQ (3,2) || __glibc_has_attribute (__deprecated__)`.  */
macro_rules! __attribute_deprecated__ { () => { __attribute__ ((__deprecated__)) }; }

/* C 344: `#define __attribute_deprecated__ ` - under `!__GNUC_PREREQ (3,2) || __glibc_has_attribute (__deprecated__)`.  */
#[cfg (feature = "gnuc_prereq_3_2_glibc_has")]
macro_rules! __attribute_deprecated__ { () => {  }; }

/* C 349: `#define __attribute_deprecated_msg__(msg) __attribute__ ((__deprecated__ (msg)))` - under `__GNUC_PREREQ (4,5)      || __glibc_has_extension (__attribute_deprecated_with_message__)`.  */
macro_rules! __attribute_deprecated_msg__ { ($msg:expr) => { __attribute__ ((__deprecated__ ($msg))) }; }

/* C 351: `#define __attribute_deprecated_msg__(msg) __attribute_deprecated__` - under `!__GNUC_PREREQ (4,5)      || __glibc_has_extension (__attribute_deprecated_with_message__)`.  */
#[cfg (feature = "gnuc_prereq_4_5_glibc_has")]
macro_rules! __attribute_deprecated_msg__ { ($msg:expr) => { __attribute_deprecated__ }; }

/* C 356: `#define __attribute_format_arg__(x) __attribute__ ((__format_arg__ (x)))` - under `__GNUC_PREREQ (2,8) || __glibc_has_attribute (__format_arg__)`.  */
macro_rules! __attribute_format_arg__ { ($x:expr) => { __attribute__ ((__format_arg__ ($x))) }; }

/* C 358: `#define __attribute_format_arg__(x) ` - under `!__GNUC_PREREQ (2,8) || __glibc_has_attribute (__format_arg__)`.  */
#[cfg (feature = "gnuc_prereq_2_8_glibc_has")]
macro_rules! __attribute_format_arg__ { ($x:expr) => {  }; }

/* C 363: `#define __attribute_format_strfmon__(a,b) __attribute__ ((__format__ (__strfmon__, a, b)))` - under `__GNUC_PREREQ (2,97) || __glibc_has_attribute (__format__)`.  */
macro_rules! __attribute_format_strfmon__ { ($a:expr, $b:expr) => { __attribute__ ((__format__ (__strfmon__, $a, $b))) }; }

/* C 365: `#define __attribute_format_strfmon__(a,b) ` - under `!__GNUC_PREREQ (2,97) || __glibc_has_attribute (__format__)`.  */
#[cfg (feature = "gnuc_prereq_2_97_glibc_has")]
macro_rules! __attribute_format_strfmon__ { ($a:expr, $b:expr) => {  }; }

/* C 371: `#define __attribute_nonnull__(params) __attribute__ ((__nonnull__ params))` - under `__GNUC_PREREQ (3,3) || __glibc_has_attribute (__nonnull__)`.  */
macro_rules! __attribute_nonnull__ { ($params:expr) => { __attribute__ ((__nonnull__ $params)) }; }

/* C 373: `#define __attribute_nonnull__(params) ` - under `!__GNUC_PREREQ (3,3) || __glibc_has_attribute (__nonnull__)`.  */
#[cfg (feature = "gnuc_prereq_3_3_glibc_has")]
macro_rules! __attribute_nonnull__ { ($params:expr) => {  }; }

/* C 377: `#define __nonnull(params) __attribute_nonnull__ (params)` - under `!defined __nonnull`.  */
macro_rules! __nonnull { ($params:expr) => { __attribute_nonnull__ ($params) }; }

/* C 383: `#define __returns_nonnull __attribute__ ((__returns_nonnull__))` - under `__GNUC_PREREQ (4, 9) || __glibc_has_attribute (__returns_nonnull__)`.  */
#[cfg (feature = "gnuc_prereq_4_9_glibc_has")]
macro_rules! __returns_nonnull { () => { __attribute__ ((__returns_nonnull__)) }; }

/* C 385: `#define __returns_nonnull ` - under `!__GNUC_PREREQ (4, 9) || __glibc_has_attribute (__returns_nonnull__)`.  */
macro_rules! __returns_nonnull { () => {  }; }

/* C 391: `#define __attribute_warn_unused_result__ __attribute__ ((__warn_unused_result__))` - under `__GNUC_PREREQ (3,4) || __glibc_has_attribute (__warn_unused_result__)`.  */
macro_rules! __attribute_warn_unused_result__ { () => { __attribute__ ((__warn_unused_result__)) }; }

/* C 396: `#define __attribute_warn_unused_result__ ` - under `!__GNUC_PREREQ (3,4) || __glibc_has_attribute (__warn_unused_result__)`.  */
#[cfg (feature = "gnuc_prereq_3_4_glibc_has")]
macro_rules! __attribute_warn_unused_result__ { () => {  }; }

/* C 393: `#define __wur __attribute_warn_unused_result__` - under `defined __USE_FORTIFY_LEVEL && __USE_FORTIFY_LEVEL > 0`.  */
#[cfg (feature = "use_fortify_level_use_fortify_level")]
macro_rules! __wur { () => { __attribute_warn_unused_result__ }; }

/* C 399: `#define __wur ` - under `!defined __wur`.  */
macro_rules! __wur { () => {  }; }

/* C 406: `#define __always_inline __inline __attribute__ ((__always_inline__))` - under `__GNUC_PREREQ (3,2) || __glibc_has_attribute (__always_inline__)`.  */
macro_rules! __always_inline { () => { __inline __attribute__ ((__always_inline__)) }; }

/* C 409: `#define __always_inline __inline` - under `!__GNUC_PREREQ (3,2) || __glibc_has_attribute (__always_inline__)`.  */
#[cfg (feature = "gnuc_prereq_3_2_glibc_has")]
macro_rules! __always_inline { () => { __inline }; }

/* C 414: `#define __attribute_artificial__ __attribute__ ((__artificial__))` - under `__GNUC_PREREQ (4,3) || __glibc_has_attribute (__artificial__)`.  */
macro_rules! __attribute_artificial__ { () => { __attribute__ ((__artificial__)) }; }

/* C 416: `#define __attribute_artificial__ ` - under `!__GNUC_PREREQ (4,3) || __glibc_has_attribute (__artificial__)`.  */
#[cfg (feature = "gnuc_prereq_4_3_glibc_has")]
macro_rules! __attribute_artificial__ { () => {  }; }

/* C 422: `#define __extern_inline extern __inline __attribute__ ((__gnu_inline__))` - under `defined __GNUC_STDC_INLINE__ || defined __cplusplus`.  */
macro_rules! __extern_inline { () => { extern __inline __attribute__ ((__gnu_inline__)) }; }

/* C 425: `#define __extern_inline extern __inline` - under `!defined __GNUC_STDC_INLINE__ || defined __cplusplus`.  */
#[cfg (feature = "no_gnuc_stdc_inline_defined_cplusplus")]
macro_rules! __extern_inline { () => { extern __inline }; }

/* C 423: `#define __extern_always_inline extern __always_inline __attribute__ ((__gnu_inline__))` - under `defined __GNUC_STDC_INLINE__ || defined __cplusplus`.  */
macro_rules! __extern_always_inline { () => { extern __always_inline __attribute__ ((__gnu_inline__)) }; }

/* C 426: `#define __extern_always_inline extern __always_inline` - under `!defined __GNUC_STDC_INLINE__ || defined __cplusplus`.  */
#[cfg (feature = "no_gnuc_stdc_inline_defined_cplusplus")]
macro_rules! __extern_always_inline { () => { extern __always_inline }; }

/* C 431: `#define __fortify_function __extern_always_inline __attribute_artificial__` - under `defined __extern_always_inline`.  */
macro_rules! __fortify_function { () => { __extern_always_inline __attribute_artificial__ }; }

/* C 436: `#define __va_arg_pack() __builtin_va_arg_pack ()` - under `__GNUC_PREREQ (4,3)`.  */
macro_rules! __va_arg_pack { () => { __builtin_va_arg_pack () }; }

/* C 437: `#define __va_arg_pack_len() __builtin_va_arg_pack_len ()` - under `__GNUC_PREREQ (4,3)`.  */
macro_rules! __va_arg_pack_len { () => { __builtin_va_arg_pack_len () }; }

/* C 442: `#define __extension__ ` - under `! (__GNUC_PREREQ (2,8) || defined __clang__ || 0x5150 <= __SUNPRO_C)`.  */
macro_rules! __extension__ { () => {  }; }

/* C 448: `#define __restrict restrict` - under `defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L`.  */
#[cfg (feature = "stdc_version_stdc_version_199901l")]
macro_rules! __restrict { () => { restrict }; }

/* C 450: `#define __restrict ` - under `!defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L`.  */
macro_rules! __restrict { () => {  }; }

/* C 456: `#define __restrict_arr __restrict` - under `(__GNUC_PREREQ (3,1) || __clang_major__ >= 3) && !defined __cplusplus`.  */
macro_rules! __restrict_arr { () => { __restrict }; }

/* C 459: `#define __restrict_arr ` - under `defined __GNUC__`.  */
#[cfg (feature = "gnuc")]
macro_rules! __restrict_arr { () => {  }; }

/* C 462: `#define __restrict_arr restrict` - under `(defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L         && !defined _MSC_VER)`.  */
#[cfg (feature = "stdc_version_stdc_version_199901l_defined")]
macro_rules! __restrict_arr { () => { restrict }; }

/* C 465: `#define __restrict_arr ` - under `!(defined __STDC_VERSION__ && __STDC_VERSION__ >= 199901L         && !defined _MSC_VER)`.  */
#[cfg (feature = "no_stdc_version_stdc_version_199901l_defined_msc_ver")]
macro_rules! __restrict_arr { () => {  }; }

/* C 471: `#define __glibc_unlikely(cond) __builtin_expect ((cond), 0)` - under `(__GNUC__ >= 3) || __glibc_has_builtin (__builtin_expect)`.  */
macro_rules! __glibc_unlikely { ($cond:expr) => { __builtin_expect (($cond), 0) }; }

/* C 474: `#define __glibc_unlikely(cond) (cond)` - under `!(__GNUC__ >= 3) || __glibc_has_builtin (__builtin_expect)`.  */
#[cfg (feature = "gnuc_3_glibc_has_builtin_builtin")]
macro_rules! __glibc_unlikely { ($cond:expr) => { ($cond) }; }

/* C 472: `#define __glibc_likely(cond) __builtin_expect ((cond), 1)` - under `(__GNUC__ >= 3) || __glibc_has_builtin (__builtin_expect)`.  */
macro_rules! __glibc_likely { ($cond:expr) => { __builtin_expect (($cond), 1) }; }

/* C 475: `#define __glibc_likely(cond) (cond)` - under `!(__GNUC__ >= 3) || __glibc_has_builtin (__builtin_expect)`.  */
#[cfg (feature = "gnuc_3_glibc_has_builtin_builtin")]
macro_rules! __glibc_likely { ($cond:expr) => { ($cond) }; }

/* C 480: `#define _Noreturn __attribute__ ((__noreturn__))` - under `__GNUC_PREREQ (2,8)`.  */
#[cfg (feature = "gnuc_prereq_2_8")]
macro_rules! _Noreturn { () => { __attribute__ ((__noreturn__)) }; }

/* C 482: `#define _Noreturn ` - under `!__GNUC_PREREQ (2,8)`.  */
macro_rules! _Noreturn { () => {  }; }

/* C 488: `#define __attribute_nonstring__ __attribute__ ((__nonstring__))` - under `__GNUC_PREREQ (8, 0)`.  */
#[cfg (feature = "gnuc_prereq_8_0")]
macro_rules! __attribute_nonstring__ { () => { __attribute__ ((__nonstring__)) }; }

/* C 490: `#define __attribute_nonstring__ ` - under `!__GNUC_PREREQ (8, 0)`.  */
macro_rules! __attribute_nonstring__ { () => {  }; }

/* C 497: `#define __attribute_copy__(arg) __attribute__ ((__copy__ (arg)))` - under `__GNUC_PREREQ (9, 0)`.  */
#[cfg (feature = "gnuc_prereq_9_0")]
macro_rules! __attribute_copy__ { ($arg:expr) => { __attribute__ ((__copy__ ($arg))) }; }

/* C 499: `#define __attribute_copy__(arg) ` - under `!__GNUC_PREREQ (9, 0)`.  */
macro_rules! __attribute_copy__ { ($arg:expr) => {  }; }

/* C 503: `#define _Static_assert(expr, diagnostic) extern int (*__Static_assert_function (void))        [!!sizeof (struct { int __error_if_negative: (expr) ? 2 : -1; })]` - under `(!defined _Static_assert && !defined __cplusplus       && (defined __STDC_VERSION__ ? __STDC_VERSION__ : 0) < 201112       && (!(__GNUC_PREREQ (4, 6) || __clang_major__ >= 4)           || defined __STRICT_ANSI__))`.  */
macro_rules! _Static_assert { ($expr:expr, $diagnostic:expr) => { extern int (*__Static_assert_function (void))        [!!sizeof (struct { int __error_if_negative: ($expr) ? 2 : -1; })] }; }

/* C 516: `#define __LDBL_REDIR(name, proto) ... unused__ldbl_redir` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR { ($name:expr, $proto:expr) => { ... unused__ldbl_redir }; }

/* C 540: `#define __LDBL_REDIR(name, proto) __LDBL_REDIR1 (name, proto, __nldbl_##name)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR { ($name:expr, $proto:expr) => { __LDBL_REDIR1 ($name, $proto, __nldbl_ # # $name) }; }

/* C 552: `#define __LDBL_REDIR(name, proto) name proto` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR { ($name:expr, $proto:expr) => { $name $proto }; }

/* C 517: `#define __LDBL_REDIR_DECL(name) extern __typeof (name) name __asm (__ASMNAME ("__" #name "ieee128"));` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR_DECL { ($name:expr) => { extern __typeof ($name) $name __asm (__ASMNAME ("__" #$name "ieee128")); }; }

/* C 545: `#define __LDBL_REDIR_DECL(name) extern __typeof (name) name __asm (__ASMNAME ("__nldbl_" #name));` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR_DECL { ($name:expr) => { extern __typeof ($name) $name __asm (__ASMNAME ("__nldbl_" #$name)); }; }

/* C 556: `#define __LDBL_REDIR_DECL(name) ` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR_DECL { ($name:expr) => {  }; }

/* C 518: `#define __REDIRECT_LDBL(name, proto, alias) name proto __asm (__ASMNAME ("__" #alias "ieee128"))` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __REDIRECT_LDBL { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __asm (__ASMNAME ("__" #$alias "ieee128")) }; }

/* C 546: `#define __REDIRECT_LDBL(name, proto, alias) __LDBL_REDIR1 (name, proto, __nldbl_##alias)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __REDIRECT_LDBL { ($name:expr, $proto:expr, $alias:expr) => { __LDBL_REDIR1 ($name, $proto, __nldbl_ # # $alias) }; }

/* C 558: `#define __REDIRECT_LDBL(name, proto, alias) __REDIRECT (name, proto, alias)` - under `defined __REDIRECT`.  */
macro_rules! __REDIRECT_LDBL { ($name:expr, $proto:expr, $alias:expr) => { __REDIRECT ($name, $proto, $alias) }; }

/* C 521: `#define __LDBL_REDIR2_DECL(name) extern __typeof (__##name) __##name      __asm (__ASMNAME ("__" #name "ieee128"));` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR2_DECL { ($name:expr) => { extern __typeof (__ # # $name) __ # # $name      __asm (__ASMNAME ("__" #$name "ieee128")); }; }

/* C 543: `#define __LDBL_REDIR2_DECL(name) extern __typeof (__##name) __##name __asm (__ASMNAME ("__nldbl___" #name));` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR2_DECL { ($name:expr) => { extern __typeof (__ # # $name) __ # # $name __asm (__ASMNAME ("__nldbl___" #$name)); }; }

/* C 555: `#define __LDBL_REDIR2_DECL(name) ` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR2_DECL { ($name:expr) => {  }; }

/* C 524: `#define __LDBL_REDIR1(name, proto, alias) ... unused__ldbl_redir1` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR1 { ($name:expr, $proto:expr, $alias:expr) => { ... unused__ldbl_redir1 }; }

/* C 539: `#define __LDBL_REDIR1(name, proto, alias) __REDIRECT (name, proto, alias)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR1 { ($name:expr, $proto:expr, $alias:expr) => { __REDIRECT ($name, $proto, $alias) }; }

/* C 551: `#define __LDBL_REDIR1(name, proto, alias) name proto` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR1 { ($name:expr, $proto:expr, $alias:expr) => { $name $proto }; }

/* C 525: `#define __LDBL_REDIR1_DECL(name, alias) extern __typeof (name) name __asm (__ASMNAME (#alias));` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR1_DECL { ($name:expr, $alias:expr) => { extern __typeof ($name) $name __asm (__ASMNAME (#$alias)); }; }

/* C 544: `#define __LDBL_REDIR1_DECL(name, alias) extern __typeof (name) name __asm (__ASMNAME (#alias));` - under `defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR1_DECL { ($name:expr, $alias:expr) => { extern __typeof ($name) $name __asm (__ASMNAME (#$alias)); }; }

/* C 527: `#define __LDBL_REDIR1_NTH(name, proto, alias) __REDIRECT_NTH (name, proto, alias)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR1_NTH { ($name:expr, $proto:expr, $alias:expr) => { __REDIRECT_NTH ($name, $proto, $alias) }; }

/* C 541: `#define __LDBL_REDIR1_NTH(name, proto, alias) __REDIRECT_NTH (name, proto, alias)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR1_NTH { ($name:expr, $proto:expr, $alias:expr) => { __REDIRECT_NTH ($name, $proto, $alias) }; }

/* C 553: `#define __LDBL_REDIR1_NTH(name, proto, alias) name proto __THROW` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR1_NTH { ($name:expr, $proto:expr, $alias:expr) => { $name $proto __THROW }; }

/* C 528: `#define __REDIRECT_NTH_LDBL(name, proto, alias) __LDBL_REDIR1_NTH (name, proto, __##alias##ieee128)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __REDIRECT_NTH_LDBL { ($name:expr, $proto:expr, $alias:expr) => { __LDBL_REDIR1_NTH ($name, $proto, __ # # $alias # # ieee128) }; }

/* C 547: `#define __REDIRECT_NTH_LDBL(name, proto, alias) __LDBL_REDIR1_NTH (name, proto, __nldbl_##alias)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __REDIRECT_NTH_LDBL { ($name:expr, $proto:expr, $alias:expr) => { __LDBL_REDIR1_NTH ($name, $proto, __nldbl_ # # $alias) }; }

/* C 559: `#define __REDIRECT_NTH_LDBL(name, proto, alias) __REDIRECT_NTH (name, proto, alias)` - under `defined __REDIRECT`.  */
macro_rules! __REDIRECT_NTH_LDBL { ($name:expr, $proto:expr, $alias:expr) => { __REDIRECT_NTH ($name, $proto, $alias) }; }

/* C 531: `#define __LDBL_REDIR_NTH(name, proto) ... unused__ldbl_redir_nth` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR_NTH { ($name:expr, $proto:expr) => { ... unused__ldbl_redir_nth }; }

/* C 542: `#define __LDBL_REDIR_NTH(name, proto) __LDBL_REDIR1_NTH (name, proto, __nldbl_##name)` - under `defined __REDIRECT`.  */
#[cfg (feature = "redirect")]
macro_rules! __LDBL_REDIR_NTH { ($name:expr, $proto:expr) => { __LDBL_REDIR1_NTH ($name, $proto, __nldbl_ # # $name) }; }

/* C 554: `#define __LDBL_REDIR_NTH(name, proto) name proto __THROW` - under `(!defined __LDBL_COMPAT && __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI == 0)      || !defined __REDIRECT`.  */
macro_rules! __LDBL_REDIR_NTH { ($name:expr, $proto:expr) => { $name $proto __THROW }; }

/* C 537: `#define __LDBL_COMPAT 1` - the gate asks for it among the constants.  */

pub const __LDBL_COMPAT: core::ffi::c_int = 1;

/* C 565: `#define __glibc_macro_warning1(message) _Pragma (#message)` - under `__GNUC_PREREQ (4,8) || __glibc_clang_prereq (3,5)`.  */
macro_rules! __glibc_macro_warning1 { ($message:expr) => { _Pragma (#$message) }; }

/* C 566: `#define __glibc_macro_warning(message) __glibc_macro_warning1 (GCC warning message)` - under `__GNUC_PREREQ (4,8) || __glibc_clang_prereq (3,5)`.  */
macro_rules! __glibc_macro_warning { ($message:expr) => { __glibc_macro_warning1 (GCC warning $message) }; }

/* C 568: `#define __glibc_macro_warning(msg) ` - under `!__GNUC_PREREQ (4,8) || __glibc_clang_prereq (3,5)`.  */
#[cfg (feature = "gnuc_prereq_4_8_glibc_clang")]
macro_rules! __glibc_macro_warning { ($msg:expr) => {  }; }

/* C 573: `#define __HAVE_GENERIC_SELECTION 1` - under `!defined __cplusplus      && (__GNUC_PREREQ (4, 9)  	|| __glibc_has_extension (c_generic_selections)  	|| (!defined __GNUC__ && defined __STDC_VERSION__  	    && __STDC_VERSION__ >= 201112L))`.  */
macro_rules! __HAVE_GENERIC_SELECTION { () => { 1 }; }

/* C 575: `#define __HAVE_GENERIC_SELECTION 0` - under `!!defined __cplusplus      && (__GNUC_PREREQ (4, 9)  	|| __glibc_has_extension (c_generic_selections)  	|| (!defined __GNUC__ && defined __STDC_VERSION__  	    && __STDC_VERSION__ >= 201112L))`.  */
#[cfg (feature = "no_cplusplus_gnuc_prereq_4_9_glibc_has_extension_c_generic_selections_defined_gnuc_defined_stdc_version_stdc_version_201112l")]
macro_rules! __HAVE_GENERIC_SELECTION { () => { 0 }; }

/* C 580: `#define __glibc_const_generic(PTR, CTYPE, CALL) _Generic (0 ? (PTR) : (void *) 1,			 	    const void *: (CTYPE) (CALL),		 	    default: CALL)` - under `__HAVE_GENERIC_SELECTION`.  */
macro_rules! __glibc_const_generic { ($PTR:expr, $CTYPE:expr, $CALL:expr) => { _Generic (0 ? ($PTR) : (void *) 1,			 	    const void *: ($CTYPE) ($CALL),		 	    default: $CALL) }; }

/* C 585: `#define __attr_access(x) __attribute__ ((__access__ x))` - under `__GNUC_PREREQ (10, 0)`.  */
#[cfg (feature = "gnuc_prereq_10_0")]
macro_rules! __attr_access { ($x:expr) => { __attribute__ ((__access__ $x)) }; }

/* C 599: `#define __attr_access(x) ` - under `!__GNUC_PREREQ (10, 0)`.  */
macro_rules! __attr_access { ($x:expr) => {  }; }

/* C 588: `#define __fortified_attr_access(a, o, s) ` - under `__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortified_attr_access { ($a:expr, $o:expr, $s:expr) => {  }; }

/* C 590: `#define __fortified_attr_access(a, o, s) __attr_access ((a, o, s))` - under `!__USE_FORTIFY_LEVEL == 3`.  */
#[cfg (feature = "use_fortify_level_3")]
macro_rules! __fortified_attr_access { ($a:expr, $o:expr, $s:expr) => { __attr_access (($a, $o, $s)) }; }

/* C 598: `#define __fortified_attr_access(a, o, s) ` - under `!__GNUC_PREREQ (10, 0)`.  */
macro_rules! __fortified_attr_access { ($a:expr, $o:expr, $s:expr) => {  }; }

/* C 593: `#define __attr_access_none(argno) __attribute__ ((__access__ (__none__, argno)))` - under `__GNUC_PREREQ (11, 0)`.  */
#[cfg (feature = "gnuc_prereq_11_0")]
macro_rules! __attr_access_none { ($argno:expr) => { __attribute__ ((__access__ (__none__, $argno))) }; }

/* C 595: `#define __attr_access_none(argno) ` - under `!__GNUC_PREREQ (11, 0)`.  */
#[cfg (feature = "gnuc_prereq_11_0")]
macro_rules! __attr_access_none { ($argno:expr) => {  }; }

/* C 600: `#define __attr_access_none(argno) ` - under `!__GNUC_PREREQ (10, 0)`.  */
macro_rules! __attr_access_none { ($argno:expr) => {  }; }

/* C 605: `#define __attr_dealloc(dealloc, argno) __attribute__ ((__malloc__ (dealloc, argno)))` - under `__GNUC_PREREQ (11, 0)`.  */
#[cfg (feature = "gnuc_prereq_11_0")]
macro_rules! __attr_dealloc { ($dealloc:expr, $argno:expr) => { __attribute__ ((__malloc__ ($dealloc, $argno))) }; }

/* C 608: `#define __attr_dealloc(dealloc, argno) ` - under `!__GNUC_PREREQ (11, 0)`.  */
macro_rules! __attr_dealloc { ($dealloc:expr, $argno:expr) => {  }; }

/* C 606: `#define __attr_dealloc_free __attr_dealloc (__builtin_free, 1)` - under `__GNUC_PREREQ (11, 0)`.  */
#[cfg (feature = "gnuc_prereq_11_0")]
macro_rules! __attr_dealloc_free { () => { __attr_dealloc (__builtin_free, 1) }; }

/* C 609: `#define __attr_dealloc_free ` - under `!__GNUC_PREREQ (11, 0)`.  */
macro_rules! __attr_dealloc_free { () => {  }; }

/* C 614: `#define __attribute_returns_twice__ __attribute__ ((__returns_twice__))` - under `__GNUC_PREREQ (4, 1)`.  */
macro_rules! __attribute_returns_twice__ { () => { __attribute__ ((__returns_twice__)) }; }

/* C 616: `#define __attribute_returns_twice__ ` - under `!__GNUC_PREREQ (4, 1)`.  */
#[cfg (feature = "gnuc_prereq_4_1")]
macro_rules! __attribute_returns_twice__ { () => {  }; }

/* C 621: `#define __attribute_struct_may_alias__ __attribute__ ((__may_alias__))` - under `__GNUC_PREREQ (7, 1) || defined __clang__`.  */
macro_rules! __attribute_struct_may_alias__ { () => { __attribute__ ((__may_alias__)) }; }

/* C 623: `#define __attribute_struct_may_alias__ ` - under `!__GNUC_PREREQ (7, 1) || defined __clang__`.  */
#[cfg (feature = "gnuc_prereq_7_1_defined_clang")]
macro_rules! __attribute_struct_may_alias__ { () => {  }; }
