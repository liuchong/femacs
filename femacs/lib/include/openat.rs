// femacs: mirror of lib/openat.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The header is the `openat` replacement's interface:
// two entry points for the `!HAVE_OPENAT` case, two `_Noreturn` failure hooks, and three
// families of `fchownat`/`fchmodat`/`fstatat` conveniences that gnulib only compiles when its
// module is wanted.
//
// `lib/openat.h` is in docs/baseline/config/cond-eval-verified.txt (line 118), so the
// extractor decides its `#if`s against the frozen configuration:
// `python3 tools/check-cond-eval.py lib/openat.h` prints `lib/openat.h: 11 conditions,
// 0 unknown, 0 disagreements [VERIFIED]`.  The frozen target does **not** have `HAVE_OPENAT`
// (`grep -n HAVE_OPENAT build/emacs-ref/src/config.h` prints nothing, and the file is not in
// the macro table either), so C 41's `#if !HAVE_OPENAT` arm is the live one and the two
// `# define`s of C 49-51 are dead text.  The three `GNULIB_*AT` selectors (C 62, 82, 102) are
// not in the configuration either, so the six inline `chownat`/`lchownat`, `chmodat`/
// `lchmodat` and `statat`/`lstatat` definitions are dead too.  All five dead arms are recorded
// in docs/ports/include-openat.md rather than transcribed (this file's arms are decided, so
// the gate does not ask for them - the ftoastr.h arrangement in this batch).
//
// `--check` asks only for the guard name; the four declarations are what the header is *for*,
// and they are declared (never defined - `openat-die.c` defines the two hooks and the two
// entry points come from gnulib's `openat.c`, which the frozen build does not compile: there
// is no `build/emacs-ref/lib/openat.o`).
//
// Three units already carry these prototypes by hand, because this mirror did not exist:
// `lib/utimensat.rs` 66-70 declares `openat_save_fail`/`openat_restore_fail` and
// `openat_proc_name`, `lib/at-func.rs` calls the two hooks, and `lib/openat-die.rs` 10-13
// declares `abort` where C 4 includes this header.  Wiring them to this mirror is a unit-side
// change for those files' owners; the declarations sit in a whole-import module so a unit that
// keeps its own copy shadows the glob instead of clashing with it (PORTING.md 0.1-10).

/* C 20: `#ifndef _GL_HEADER_OPENAT` - the guard has no Rust counterpart (PORTING.md 0.1-4);
   the name stays as its marker.  */
pub const _GL_HEADER_OPENAT: bool = true;

pub mod openat_decls {
  use super::*;

  /* C 43-45: the `!HAVE_OPENAT` interface.  C's `mode_t` is 16-bit on the frozen target
     (`include/sys_types.in.rs` measures `pub type mode_t = u16` from `__uint16_t`), and the
     width is written directly here so that this mirror does not depend on the including
     unit's scope - a mirror may not carry a `use` (E0252 against the includer's), and
     `build-tu lib/openat.h` compiles this file as its own crate root.  */
  unsafe extern "C" {
    pub fn openat_permissive (fd: core::ffi::c_int, file: *const core::ffi::c_char,
                              flags: core::ffi::c_int, mode: u16,
                              cwd_errno: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn openat_needs_fchdir () -> bool;
  }

  /* C 55-56: `_Noreturn void openat_restore_fail (int);` and its save-side twin.  C's
     `_Noreturn` is Rust's `-> !`; the definitions are `lib/openat-die.rs`'s.  */
  unsafe extern "C" {
    pub fn openat_restore_fail (arg: core::ffi::c_int) -> !;
    pub fn openat_save_fail (arg: core::ffi::c_int) -> !;
  }
}
use openat_decls::*;
