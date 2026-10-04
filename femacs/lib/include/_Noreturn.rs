// femacs: port of lib/_Noreturn.h @ fd7afced403 (gnulib's <_Noreturn.h>; PORTING.md P-26).
//
// The header is not a template, so the criterion is the header itself as the frozen compiler
// reads it: C 29's `#ifndef _Noreturn` asks whether the *macro* exists, C 30-31's first arm
// (`(!defined __cplusplus || defined __clang__) && 201112 <= __STDC_VERSION__`) is true for
// the frozen configuration, and that arm defines nothing - `_Noreturn` is the C11 keyword
// itself.  Measured:
//
//     $ printf '#include "_Noreturn.h"\n_Noreturn void f (void);\n' > /tmp/nr.c
//     $ clang -std=gnu23 -I emacs/lib -E /tmp/nr.c | tail -1
//     _Noreturn void f (void);
//     $ clang -std=gnu23 -I emacs/lib -dM -E -include _Noreturn.h -x c /dev/null | grep -i noreturn
//     (no output: the header defines no macro at all on this target)
//
// So the file has one name in `tools/extract-abi.py`'s item list (`_Noreturn`, whose recorded
// body is C 41's empty `# define _Noreturn`) and three dead arms, which the mirror carries as
// the text P-51 asks for: C 37's `__attribute__ ((__noreturn__))`, C 39's
// `__declspec (noreturn)` and C 41's empty definition.  The port's own counterpart for the
// attribute is `-> !` at the use sites (PORTING.md 8/P-13; measured: `femacs/lib-src/hexl.rs`
// 45, `femacs/lib-src/emacsclient.rs` 800, `femacs/src/data.rs` 115 spell it that way), so
// each arm is the no-op its attribute is - the `attribute.rs` shape - and the ungated item is
// the marker that says the frozen arm defines no macro.
//
// C's arms are written first (`tools/extract-abi.py` compares the first definition of a name)
// and the ungated marker last (`rustc` compiles that one); the marker is a `const`, not a
// `macro_rules!`, because an empty-expansion macro body would be the one thing the extractor
// refuses (`macro _Noreturn has an empty body`) - the `gnulib_config.rs` `_GL_INLINE` /
// `_GL_EXTERN_INLINE` markers are the precedent.

/* C 30-33: the frozen arm, `_Noreturn` works as-is - the name is the keyword, and no macro is
   defined.  The port has no spelling for it as a name (`-> !` is the function's return type),
   so this is the marker the extractor's item list asks for.  */
pub const _Noreturn: () = ();

/* C 33-37: `#define _Noreturn __attribute__ ((__noreturn__))` - the GCC 2.8+/clang/SunPro arm,
   dead here because the arm above it is the one this configuration takes.  */
#[cfg (feature = "gnu_noreturn_attribute")]
macro_rules! _Noreturn {
  () => {};
}

/* C 38-39: `#define _Noreturn __declspec (noreturn)` - the MSVC arm.  */
#[cfg (feature = "msvc_noreturn_declspec")]
macro_rules! _Noreturn {
  () => {};
}

/* C 40-41: `#define _Noreturn` - the arm for a compiler with none of the three spellings; it
   expands to nothing, which is why the port's marker above is what a site sees.  */
#[cfg (feature = "no_noreturn_spelling")]
macro_rules! _Noreturn {
  () => {};
}

/* C 26-28's note about LLVM bug 59792 and C 17-25's `[[noreturn]]` rationale are comment text
   with no Rust counterpart (PORTING.md 0.1), and C 43's `#endif` closes the test.  */
