// femacs: port of src/dosfns.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The header is the MS-DOS file-name layer's interface:
// C 23 is the one macro, C 24 the one object, C 25-29 the four declarations `src/dosfns.c`
// defines, and C 31-35 three more declarations the header guards with `#ifndef HAVE_X_WINDOWS`.
//
// The header has **no include guard** at all: C 23 is the file's first line after the opening
// comment, so there is no guard name for the mirror to carry - and the extractor's item list
// says the same (its one macro is C 23 and not a guard; unlike `include/cygw32.rs`, whose single
// macro *is* its guard).
//
// The one conditional is C 31's `#ifndef HAVE_X_WINDOWS`.  The frozen configuration leaves
// HAVE_X_WINDOWS undefined (`docs/baseline/config/darwin-arm64.macros.tsv`; the character
// terminal build, docs/status.md 3.31), so the `#ifndef` is *satisfied* here and C 32-34 are
// live text - the mirror therefore carries them unconditionally, and the arm C's preprocessor
// drops on this target (the `#else` the header does not have) is empty text: nothing is
// deferred, nothing needs a `#[cfg]` (PORTING.md P-51 would apply only to a dead arm).
// `tools/check-cond-eval.py src/dosfns.h` -> `1 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
//
// Nothing on this target compiles it: the frozen configuration's `MSDOS_OBJ` is empty
// (docs/baseline/frozen-build-sources.txt), so `src/dosfns.c`'s port is the empty shape
// `femacs/src/dosfns.rs` records and no unit `include!`s this mirror yet.  The mirror carries
// the header's live text regardless, and the definitions are deferred to the MS-DOS target.
//
// The header includes nothing, so it names `struct frame` and `Lisp_Object` as C's translation
// units happen to have them: `struct frame` is only ever a pointer here and has no owner mirror
// in this file's reach, so it is `*mut c_void` with the C text in a `/// C:` line (PORTING.md
// P-55/P-72 rule 3, the `include/w32inevt.rs` spelling); `Lisp_Object` stays named, because
// lisp.h is the one header every unit reaches through its own includes (PORTING.md 0.1-9).

/* C 23: `#define DOS_COUNTRY_INFO 34` - the size of the array C 24 declares.  C's macro has
   no type of its own; the port spells it `usize` because an array length is what every use of
   the name is (the `lib/root-uid.h` reading, status 3.156).  */
pub const DOS_COUNTRY_INFO: usize = 34;

/* Every name the block below declares is `src/dosfns.c`'s *definition*, so at this mirror's crate
   root the `msdos` arm read eight `E0428 ... is defined multiple times` (the class 3.575 fixed in
   `include/w32term.rs`: measured 2026-10-05, status 3.590).  They sit in a module the mirror
   glob-imports, and a crate-root definition shadows a glob import (PORTING.md 0.1-10 / P-21).  */
pub mod dosfns_decls {
  use super::*;

  /// C 24: `extern unsigned char dos_country_info[DOS_COUNTRY_INFO];` - the country
  /// information block `dos_memory_info`'s neighbours fill and the DOS Lisp layer reads.
  /// C: extern unsigned char dos_country_info[DOS_COUNTRY_INFO];
  unsafe extern "C" {
    pub static mut dos_country_info: [u8; DOS_COUNTRY_INFO];

    /// C 25-26: `dos_memory_info` reports the four DOS memory sizes through its out-parameters -
    /// the parameter names are the ones `src/dosfns.c` 633-634 gives the definition.
    /// C: extern int dos_memory_info (unsigned long *, unsigned long *, unsigned long *,
    ///                               unsigned long *);
    pub fn dos_memory_info (totalram: *mut c_ulong, freeram: *mut c_ulong,
                            totalswap: *mut c_ulong, freeswap: *mut c_ulong) -> c_int;

    /// C 27-29: the three entry points `src/dosfns.c` defines.
    /// C: void dos_cleanup (void);
    /// C: void syms_of_dosfns (void);
    /// C: void init_dosfns (void);
    pub fn dos_cleanup ();
    pub fn syms_of_dosfns ();
    pub fn init_dosfns ();

    /// C 31-35: the arm the header guards with `#ifndef HAVE_X_WINDOWS`, which this target's
    /// configuration satisfies (see the note above) - so these three are live text, not P-51
    /// dead arms.  `struct frame *` is `*mut c_void` (the header has no such record and reaches
    /// no frame.h).
    /// C: extern int         msdos_stdcolor_idx  (const char *);
    /// C: extern Lisp_Object msdos_stdcolor_name (int);
    /// C: extern void        x_set_title (struct frame *, Lisp_Object);
    pub fn msdos_stdcolor_idx (name: *const c_char) -> c_int;
    pub fn msdos_stdcolor_name (idx: c_int) -> Lisp_Object;
    pub fn x_set_title (f: *mut c_void, name: Lisp_Object);
  }
}
use dosfns_decls::*;
