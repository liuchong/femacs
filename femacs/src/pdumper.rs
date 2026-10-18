// femacs: port of src/pdumper.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  `unsafe` wherever C needs it.  Every deviation is recorded in docs/ports/pdumper.md.
//
// **Dead arms (P-08 / P-51)**: `src/pdumper.c` is *not* in
// docs/baseline/config/cond-eval-verified.txt (`python3 tools/check-cond-eval.py
// src/pdumper.c` reports 13 disagreements against clang — `HAVE_MADVISE`/`MAP_POPULATE`
// entries the evaluator's table cannot see, function-like `min`/`max` in the
// `DUMP_ALIGNMENT` enum, `VM_SUPPORTED` comparisons), so the frozen macro table is not
// consulted for this file: every `#if`/`#ifdef`/`#elif`/`#else` arm is carried as text
// (docs/status.md 3.49) — the arm clang compiles is plain text, each clang-dead arm sits
// under `#[cfg (feature = "…")]` naming C's own condition.  Only `#if 0` is recorded in
// the ledger instead of transcribed (there is one: C 1682-1700's `dump_remember_fixup_lv`
// debug print, inside the function).
//
// **Mirrors**: `include/pdumper.rs` (the header) exists and is complete; the load-time
// structs it declares are used as they are.  The names C's *other* headers give this unit
// that no mirror carries are declared at their use site (PORTING.md 0.1-2, P-72) — see the
// `pdumper_use_site_decls` module below and the ledger's "use-site declarations" section.
//
// **The header's checksum/offset records** (`struct dump_header`, its `dump_table_locator`
// entries and `struct dump_reloc`) keep C's exact field order, widths and bitfield packing:
// `loadup`/`pdumper_load` compare the dump against `fingerprint`, so the records are frozen
// (PORTING.md 4.4 P-09) and each one carries clang-measured `const _` assertions.
//
// C's `static_assert` (`lisp.rs`'s `static_assert!`), `eassert`/`eassume` (release: no
// evaluation, PORTING.md 2.5) and the `goto` cleanup paths inside `pdumper_load` are kept
// verbatim; `sys_setjmp`/`sys_longjmp` are not used by this file (its error model is
// `error`/`emacs_abort`/return codes, exactly as C).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unused_braces, unused_labels, unreachable_code, unused_comparisons, path_statements)]

// `c_float` (C's `float`) and `VaList` are the two names the mirrors' import lists do not
// carry (lisp.rs 14 imports the `c_*` types without `c_float`; C's `<stdarg.h>` has no
// mirror at all, PORTING.md P-04 and the doprnt.rs `use core::ffi::VaList`).
use std::ffi::c_float;
use core::ffi::VaList;

include!("include/config.rs");                // C 26: `#include <config.h>`
include!("../lib/include/errno.rs");          // C 28: `#include <errno.h>`
include!("../lib/include/fcntl.in.rs");       // C 29: `#include <fcntl.h>`
include!("../lib/include/limits.in.rs");      // C 30: `#include <limits.h>`
include!("../lib/include/math.rs");           // C 31: `#include <math.h>`
include!("../lib/include/stdint.in.rs");      // C 33: `#include <stdint.h>`
include!("../lib/include/stdlib.rs");         // C 34: `#include <stdlib.h>`
include!("../lib/include/sys_stat.in.rs");    // C 36: `#include <sys/stat.h>`
include!("../lib/include/sys_types.in.rs");   // C 37: `#include <sys/types.h>`
include!("../lib/include/unistd.in.rs");      // C 38: `#include <unistd.h>`
include!("../lib/include/stdio.rs");          // sysstdio.h 24: `#include <stdio.h>`
include!("../lib/include/string.rs");         // lisp.h 30: `#include <string.h>`
include!("../lib/include/sys-limits.rs");     // lisp.h 31's sys/limits.h (SYS_BUFSIZE_MAX)
include!("../lib/include/fingerprint.rs");    // C 45: `#include "fingerprint.h"`
include!("include/lisp.rs");                  // C 41: `#include "lisp.h"`
include!("../lib/include/mini-gmp.rs");       // lisp.h's bignum.h chain (bignum.rs's members)
include!("include/bignum.rs");                // C 52: `#include "bignum.h"`
include!("include/character.rs");             // lisp.h 29's character.h chain
include!("include/blockinput.rs");            // C 40: `#include "blockinput.h"`
include!("include/itree.rs");                 // buffer.h 28: `#include "itree.h"`
include!("include/systime.rs");               // C 50: `#include "systime.h"` (termhooks.h 28)
include!("include/font.rs");                  // composite.h 28: `#include "font.h"`
include!("include/composite.rs");             // coding.h 284: `#include "composite.h"`
include!("include/buffer.rs");                // C 41: `#include "buffer.h"`
include!("include/intervals.rs");             // C 46: `#include "intervals.h"`
include!("include/frame.rs");                 // C 44: `#include "frame.h"`
include!("include/dispextern.rs");            // composite.h 29: `#include "dispextern.h"`
include!("include/window.rs");                // C 48: `#include "window.h"`
include!("include/charset.rs");               // C 42: `#include "charset.h"`
include!("include/coding.rs");                // C 43: `#include "coding.h"`
include!("include/ccl.rs");                   // coding.h 283: `#include "ccl.h"`
include!("include/termhooks.rs");             // frame.h 28: `#include "termhooks.h"`
include!("include/keyboard.rs");              // C 51: `#include "thread.h"`'s chain
include!("include/thread.rs");                // C 51: `#include "thread.h"`
include!("include/pdumper.rs");               // C 47: `#include "pdumper.h"`
include!("include/treesit.rs");            // C 47: `#include "treesit.h"`

// C 27's `#include <sys/mman.h>`, C 24's `#include <stdarg.h>` (`core::ffi::VaList`,
// PORTING.md P-04) and C 28's `#include <sys/param.h>` have no mirror yet; the names this
// unit takes from them are declared below.  `treesit.h` (C 47) now has one and is included
// above; the two names this unit takes from it sit in its dead `#ifdef HAVE_TREE_SITTER` arm
// and stay text (P-08/P-51).

/* The names C's headers give this unit that no mirror carries yet (PORTING.md 0.1-2,
   P-26/P-72; the ccl.rs / chartab.rs shape).  Each is C's own declaration, measured on the
   frozen target where the value matters; the block shrinks as the mirrors grow (recorded in
   docs/ports/pdumper.md's use-site section):

   * `<sys/mman.h>`: `mmap`/`munmap`/`madvise` and the `PROT_*`/`MAP_*`/`MADV_*` constants
     (measured with `clang -E -dM -include sys/mman.h` on the frozen target);
   * `<stdio.h>`: `fprintf`/`vfprintf`/`snprintf`/`putc`/`stderr` (stdio.rs keeps only the
     four md5-stream entries, `sprintf` and `fclose`);
   * `<stdint.h>`: `PRIdLEAST32` only (the mirror carries `int_least32_t`, its
     `INT_LEAST32_MIN`/`_MAX`/`_WIDTH`, `UINT_FAST32_WIDTH` and `INTPTR_MAX` now);
   * `<stdlib.h>`: `qsort`; `<string.h>`: `strcat`; `<sys/param.h>`: `MAXPATHLEN`;
     `<sys/stat.h>`: `S_ISREG`;
   * `lisp.h`: `hexbuf_digest`, `emacs_write`, `emacs_open_noquit`; `treesit.h`'s
     `struct Lisp_TS_Query` and `XTS_COMPILED_QUERY` live inside their dead arm.  */
pub mod pdumper_use_site_decls {
  use super::*;

  unsafe extern "C" {
    /* void *mmap (void *, size_t, int, int, int, off_t);  */
    pub fn mmap (addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int,
                 offset: off_t) -> *mut c_void;
    /* int munmap (void *, size_t);  */
    pub fn munmap (addr: *mut c_void, len: size_t) -> c_int;
    /* float powf (float, float);  */
    pub fn powf (x: c_float, y: c_float) -> c_float;
    /* long labs (long);  (`<stdlib.h>`'s `labs`, which `dump_check_emacs_off` names; the
       release `eassert!` does not typecheck its condition, so only the
       `--cfg enable_checking` pass needs it declared)  */
    pub fn labs (n: c_long) -> c_long;
    /* int madvise (void *, size_t, int);  */
    pub fn madvise (addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
    /* int posix_madvise (void *, size_t, int); int mprotect (void *, size_t, int);  */
    pub fn posix_madvise (addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
    pub fn mprotect (addr: *mut c_void, len: size_t, prot: c_int) -> c_int;

    /* int fprintf (FILE *, const char *, ...); int vfprintf (FILE *, const char *, va_list);
       int putc (int, FILE *);  - `int snprintf (char *, size_t, const char *, ...)` comes
       from `lib/include/stdio.rs` (status 3.263).  */
    pub fn vfprintf (stream: *mut FILE, format: *const c_char, ap: VaList<'_>) -> c_int;

    /* void abort (void);  */
    pub fn abort () -> !;
    /* void qsort (void *, size_t, size_t, int (*) (void const *, void const *));  */
    pub fn qsort (base: *mut c_void, nmemb: size_t, size: size_t,
                  compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);

    /* char *strcat (char *, char const *);  */
    pub fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;

    /* ptrdiff_t emacs_write (int, void const *, ptrdiff_t);  (lisp.h 5131)  */
    pub fn emacs_write (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
    /* int emacs_open_noquit (const char *, int, int);  (lisp.h 5312)  */
    pub fn emacs_open_noquit (file: *const c_char, oflags: c_int, mode: c_int) -> c_int;

    /* void hexbuf_digest (char *, void const *, int);  (lisp.h 4247, fns.c defines it)  */
    pub fn hexbuf_digest (hexbuf: *mut c_char, digest: *const c_void, digest_size: c_int);

    /* void visit_static_gc_roots (struct gc_root_visitor);  (lisp.h 4509)  */
    pub fn visit_static_gc_roots (visitor: gc_root_visitor);

    /* Lisp_Object *hash_table_alloc_bytes (ptrdiff_t);  (lisp.h 2527, fns.c defines it)  */
    pub fn hash_table_alloc_bytes (nbytes: ptrdiff_t) -> *mut Lisp_Object;

    /* AVOID fatal (const char *msgid, ...);  (lisp.h 5371)  */
    pub fn fatal (msgid: *const c_char, ...) -> !;
    /* void garbage_collect (void);  (lisp.h 4477)  */
    pub fn garbage_collect ();
    /* extern int staticidx;  (lisp.h 3923)  */
    /* int sys_fstat (int, struct stat *);  (lisp.h 5305; sysdep.c defines it)  */
    pub fn sys_fstat (fd: c_int, statb: *mut stat) -> c_int;
    /* char *splice_dir_file (char *, char const *, char const *);  (lisp.h 4968)  */
    pub fn splice_dir_file (buffer: *mut c_char, dirname: *const c_char,
                            filename: *const c_char) -> *mut c_char;
    /* bool file_name_absolute_p (const char *);  (lisp.h 4970, fileio.c)  */
    pub fn file_name_absolute_p (filename: *const c_char) -> bool;
    /* void hash_table_thaw (Lisp_Object);  (fns.c's `hash_table_thaw')  */
    pub fn hash_table_thaw (h: Lisp_Object);
  }
}

use pdumper_use_site_decls::*;

/// `lisp.h` 4504-4508's `struct gc_root_visitor` now lives in the mirror that owns it
/// (`include/lisp.rs`, status 3.153) and arrives here through `lisp.h`'s include; this unit keeps
/// only the declaration of `visit_static_gc_roots` (lisp.h 4509) it used to pair it with.

/// `<sys/mman.h>`'s protection and mapping constants, measured on the frozen target
/// (`clang -E -dM -include sys/mman.h`, darwin-arm64).
pub const PROT_NONE: c_int = 0x00;
pub const PROT_READ: c_int = 0x01;
pub const PROT_WRITE: c_int = 0x02;
pub const PROT_EXEC: c_int = 0x04;
pub const MAP_SHARED: c_int = 0x0001;
pub const MAP_PRIVATE: c_int = 0x0002;
pub const MAP_FIXED: c_int = 0x0010;
/// `<sys/mman.h>`: `#define MAP_ANON 0x1000` (`MAP_ANONYMOUS` is an alias of it).
pub const MAP_ANON: c_int = 0x1000;
/// `<sys/mman.h>`: `#define MAP_FAILED ((void *) -1)`.
pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_FREE: c_int = 5;
/// `<sys/mman.h>`: `#define POSIX_MADV_DONTNEED 4` (and NORMAL 0, RANDOM 1, SEQUENTIAL 2,
/// WILLNEED 3) — `MADV_*` are aliases of them on this target.
pub const POSIX_MADV_NORMAL: c_int = 0;
pub const POSIX_MADV_RANDOM: c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: c_int = 2;
pub const POSIX_MADV_WILLNEED: c_int = 3;
pub const POSIX_MADV_DONTNEED: c_int = 4;
/// `<sys/mman.h>`: `#define MAP_ANONYMOUS MAP_ANON` (0x1000) on the frozen target; C 4493's
/// `#ifndef` arm is dead because the system header defines it.
pub const MAP_ANONYMOUS: c_int = MAP_ANON;
/// `fingerprint.h` 26: `extern unsigned char fingerprint[32]` — the size C's
/// `sizeof fingerprint` reads (`emacs.rs` spells the same constant for `emacs.c`).
pub const FINGERPRINT_SIZE: usize = core::mem::size_of::<[c_uchar; 32]> ();

/// `<inttypes.h>`: `#define PRIdLEAST32 "d"` (`<inttypes.h>` has no mirror yet, P-26).
/// The macro form is the one `concat!` can expand (PORTING.md 2.1), the `const` the one
/// the C-side value comparison reads.
macro_rules! PRIdLEAST32 { () => { "d" } }
pub const PRIdLEAST32: &str = PRIdLEAST32! ();

/// `<sys/stat.h>`: `#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)` (the frozen target's
/// `<sys/stat.h>` spells it as a function-like macro over the `st_mode` bits).
#[inline(always)]
pub fn S_ISREG (m: mode_t) -> bool { (m & S_IFMT) == S_IFREG }

/// `<sys/param.h>`: `#define MAXPATHLEN PATH_MAX` (measured: 1024 on the frozen target).
pub const MAXPATHLEN: c_int = 1024;

/* C 71's `#ifdef HAVE_PDUMPER` is live (`HAVE_PDUMPER` is 1 in the frozen configuration,
   docs/baseline/config/darwin-arm64.macros.tsv); its empty `#else` arm carries no text.
   The matching `#endif` is C 5877 (and the two inside `syms_of_pdumper` are transcribed in
   place).  C 73-75's `#if GNUC_PREREQ (4, 7, 0)` guard around
   `# pragma GCC diagnostic error "-Wshadow"` is live, and the pragma has no Rust
   counterpart (PORTING.md 8: a diagnostic attribute is recorded as a no-op).  */

/* Dump file creation */

/* C 77-78: the two values the `VM_SUPPORTED` chain below compares against.  */
pub const VM_POSIX: c_int = 1;
pub const VM_MS_WINDOWS: c_int = 2;

/* C 80-97: the `#if` chain that picks the dumping VM.  The frozen configuration compiles
   the `#elif defined (HAVE_MMAP) && defined (MAP_FIXED)` arm, and inside it the
   `# elif !defined (MAP_POPULATE)` sub-arm (this target has `<sys/mman.h>` but neither
   `MAP_POPULATE` nor `MAP_PREFAULT_READ`: `clang -E -dM -include sys/mman.h`).  The other
   three arms and the first `MAP_POPULATE` sub-arm are carried under `#[cfg (feature =
   "…")]` (P-51, docs/status.md 3.49) — they are the only text in this file that is
   neither compiled nor recorded in the ledger.  */
#[cfg (feature = "not_use_lsb_tag_not_wide_emacs_int")]
pub const VM_SUPPORTED: c_int = 0;
#[cfg (feature = "have_mmap_map_fixed")]
pub const VM_SUPPORTED: c_int = VM_POSIX;
#[cfg (feature = "no_map_populate_map_prefault_read")]
pub const MAP_POPULATE: c_int = MAP_PREFAULT_READ;
#[cfg (feature = "no_map_populate_map_prefault_read")]
pub const MAP_PREFAULT_READ: c_int = 0;      /* BSD's value; never compiled here.  */
#[cfg (feature = "no_map_populate")]
pub const MAP_POPULATE: c_int = 0;
#[cfg (feature = "windowsnt")]
pub const INFINITY: c_float = f32::INFINITY;  /* `__builtin_inff ()` (PORTING.md 2.6).  */
#[cfg (feature = "windowsnt")]
pub const VM_SUPPORTED: c_int = VM_MS_WINDOWS;
#[cfg (feature = "no_vm_supported")]
pub const VM_SUPPORTED: c_int = 0;

/// The live arm of the same chain: `VM_SUPPORTED` is `VM_POSIX`, and `MAP_POPULATE` is 0
/// because this target's `<sys/mman.h>` does not define it.
pub const VM_SUPPORTED: c_int = VM_POSIX;
/// `<math.h>`: `#define INFINITY (__builtin_inff ())` — the live configuration's `INFINITY`
/// comes from the system header, not from C 92-93's `#undef`/`#define` pair.
pub const INFINITY: c_float = f32::INFINITY;
pub const MAP_POPULATE: c_int = 0;

/* Require an architecture in which pointers, ptrdiff_t and intptr_t
   are the same size and have the same layout, and where bytes have
   eight bits --- that is, a general-purpose computer made after 1990.
   Also require Lisp_Object to be at least as wide as pointers.  */
static_assert! (core::mem::size_of::<ptrdiff_t> () == core::mem::size_of::<*mut c_void> ());
static_assert! (core::mem::size_of::<intptr_t> () == core::mem::size_of::<ptrdiff_t> ());
static_assert! (core::mem::size_of::<Option<unsafe extern "C" fn ()>> () ==
                core::mem::size_of::<*mut c_void> ());
static_assert! (core::mem::size_of::<ptrdiff_t> () <= core::mem::size_of::<Lisp_Object> ());
static_assert! (core::mem::size_of::<ptrdiff_t> () <= core::mem::size_of::<EMACS_INT> ());

fn divide_round_up (x: size_t, y: size_t) -> size_t
{
  return (x + y - 1) / y;
}

static dump_magic: [c_char; 16] = [
  b'D' as c_char, b'U' as c_char, b'M' as c_char, b'P' as c_char, b'E' as c_char,
  b'D' as c_char, b'G' as c_char, b'N' as c_char, b'U' as c_char,
  b'E' as c_char, b'M' as c_char, b'A' as c_char, b'C' as c_char, b'S' as c_char,
  0, 0
];

static mut dump_hooks: [pdumper_hook; 24] = [None; 24];
static mut nr_dump_hooks: c_int = 0;

static mut dump_late_hooks: [pdumper_hook; 24] = [None; 24];
static mut nr_dump_late_hooks: c_int = 0;

/* C 129-132 declares an anonymous `struct { void *mem; int sz; }` array; PORTING.md 1.3
   requires the nested record to have a name, so it is `dump_remembered_data` here (the
   keymap.rs `map_keymap_s` precedent).  */
#[repr(C)]
#[derive(Clone, Copy)]
struct dump_remembered_data
{
  mem: *mut c_void,
  sz: c_int,
}
static mut remembered_data: [dump_remembered_data; 32] =
  [dump_remembered_data { mem: core::ptr::null_mut (), sz: 0 }; 32];
static mut nr_remembered_data: c_int = 0;

pub type dump_off = int_least32_t;
pub const DUMP_OFF_MIN: dump_off = INT_LEAST32_MIN;
pub const DUMP_OFF_MAX: dump_off = INT_LEAST32_MAX;
pub const DUMP_OFF_WIDTH: c_int = INT_LEAST32_WIDTH;
pub const PRIdDUMP_OFF: &str = PRIdLEAST32! ();
/// C uses the macro through the `"%"PRIdDUMP_OFF"d"` splice (`dump_trace`'s and the
/// `fprintf` formats); the port keeps the same pieces with `concat!`, which needs a macro
/// rather than a `const` (PORTING.md 2.1).
macro_rules! PRIdDUMP_OFF { () => { PRIdLEAST32! () } }
/// `<inttypes.h>`: `#define PRIdPTR "ld"` and lisp.h's `PRIdHASH_IDX` (a `hash_idx_t` is an
/// `int` on this target, so the macro is `PRIdMAX`-free and spells "d").
macro_rules! PRIdPTR { () => { "ld" } }
macro_rules! PRIdHASH_IDX { () => { "d" } }

pub const EMACS_INT_XDIGITS: c_int = (EMACS_INT_WIDTH as c_int + 3) / 4;

/* C 144 has `ATTRIBUTE_FORMAT_PRINTF (1, 2)`, which PORTING.md 8 records as a no-op (it
   only affects diagnostics), and its `...` is Rust's `mut args: ...` (PORTING.md P-04,
   the sysdep.rs `snprintf` shape); `va_start`/`va_end` have no counterpart.  */
unsafe extern "C" fn dump_trace (fmt: *const c_char, mut args: ...)
{
  if 0 != 0
    {
      let va: VaList<'_> = args;
      vfprintf (stderr, fmt, va);
    }
}

fn ptrdiff_t_to_dump_off (value: ptrdiff_t) -> dump_off
{
  eassert! (DUMP_OFF_MIN as isize <= value);
  eassert! (value <= DUMP_OFF_MAX as isize);
  return value as dump_off;
}

/* Worst-case allocation granularity on any system that might load
   this dump.  */
fn dump_get_max_page_size () -> c_int
{
  return 64 * 1024;
}

/* `ptrdiff_t_to_dump_off (offsetof (type, member))`; `offsetof` is
   `core::mem::offset_of!` (PORTING.md 2.1).  */
/* C's `#define dump_offsetof (type, member) (ptrdiff_t_to_dump_off (offsetof (type, member)))`
   (C 177-178): the member is a *path* (`own_text.beg`, `u.s.data`), and the sites below spell
   C's path in full.  Taking one identifier here is what let three sites collapse to the
   *union's* offset - 0 - and write the fixup into the wrong field: `Lisp_String`'s interval
   tree over `size`, and `dump_cold_string`'s data pointer over `size` (status 3.644).  */
macro_rules! dump_offsetof {
    ($type:ty, $($member:ident).+) => {
        ptrdiff_t_to_dump_off (core::mem::offset_of! ($type, $($member).+) as ptrdiff_t)
    };
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_reloc_type
  {
    /* dump_ptr = dump_ptr + emacs_basis()  */
    RELOC_DUMP_TO_EMACS_PTR_RAW,
    /* dump_ptr = dump_ptr + dump_base  */
    RELOC_DUMP_TO_DUMP_PTR_RAW,
    /* dump_mpz = [rebuild bignum]  */
    RELOC_NATIVE_COMP_UNIT,
    RELOC_NATIVE_SUBR,
    RELOC_BIGNUM,
    /* dump_lv = make_lisp_ptr (dump_lv + dump_base,
				type - RELOC_DUMP_TO_DUMP_LV)
       (Special case for symbols: make_lisp_symbol)
       Must be second-last.  */
    RELOC_DUMP_TO_DUMP_LV,
    /* dump_lv = make_lisp_ptr (dump_lv + emacs_basis(),
				type - RELOC_DUMP_TO_DUMP_LV)
       (Special case for symbols: make_lisp_symbol.)
       Must be last.  */
    RELOC_DUMP_TO_EMACS_LV = 6 + 8,
  }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum emacs_reloc_type
  {
    /* Copy raw bytes from the dump into Emacs.  The length field in
       the emacs_reloc is the number of bytes to copy.  */
    RELOC_EMACS_COPY_FROM_DUMP,
    /* Set a piece of memory in Emacs to a value we store directly in
       this relocation.  The length field contains the number of bytes
       we actually copy into Emacs.  */
    RELOC_EMACS_IMMEDIATE,
    /* Set an aligned pointer-sized object in Emacs to a pointer into
       the loaded dump at the given offset.  The length field is
       always the machine word size.  */
    RELOC_EMACS_DUMP_PTR_RAW,
    /* Set an aligned pointer-sized object in Emacs to point to
       something also in Emacs.  The length field is always
       the machine word size.  */
    RELOC_EMACS_EMACS_PTR_RAW,
    /* Set an aligned Lisp_Object in Emacs to point to a value in the
       dump.  The length field is the _tag type_ of the Lisp_Object,
       not a byte count!  */
    RELOC_EMACS_DUMP_LV,
    /* Set an aligned Lisp_Object in Emacs to point to a value in the
       Emacs image.  The length field is the _tag type_ of the
       Lisp_Object, not a byte count!  */
    RELOC_EMACS_EMACS_LV,
  }

pub const EMACS_RELOC_TYPE_BITS: c_int = 3;
pub const EMACS_RELOC_LENGTH_BITS: c_int = DUMP_OFF_WIDTH - EMACS_RELOC_TYPE_BITS;

static_assert! ((emacs_reloc_type::RELOC_EMACS_EMACS_LV as c_int) <= (1 << EMACS_RELOC_TYPE_BITS));

/* C 246's `ENUM_BF (emacs_reloc_type) type : EMACS_RELOC_TYPE_BITS;` packs with
   `dump_off length : EMACS_RELOC_LENGTH_BITS` into the 4-byte unit at offset 0 (PORTING.md
   1.3: the port stores the unit as `raw` and reaches both halves through the macros below;
   the clang measurement is 8/8 with the unit at 0).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_reloc
{
  raw: u32,
  pub emacs_offset: dump_off,
  pub u: emacs_reloc_u,
}

/* C 249-254's anonymous `union { dump_off dump_offset; dump_off emacs_offset2; intmax_t
   immediate; } u;`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union emacs_reloc_u
{
  pub dump_offset: dump_off,
  pub emacs_offset2: dump_off,
  pub immediate: intmax_t,
}

/* C 245: `ENUM_BF (emacs_reloc_type)` is a plain `enum emacs_reloc_type` on this target
   (the bitfield emulation only exists when `enum` bitfields are unsupported).  */
macro_rules! emacs_reloc_type_of {
    ($reloc:expr) => { (($reloc).raw & ((1u32 << EMACS_RELOC_TYPE_BITS) - 1)) as c_int };
}
macro_rules! emacs_reloc_set_type_field {
    ($reloc:expr, $type:expr) => {
        ($reloc).raw = (($reloc).raw & !((1u32 << EMACS_RELOC_TYPE_BITS) - 1))
            | (($type as u32) & ((1u32 << EMACS_RELOC_TYPE_BITS) - 1))
    };
}
/// C 257-258: `#define EMACS_RELOC_LENGTH(reloc) ((reloc).length)` / `ENUM_BF` half, now
/// `dump_off length : EMACS_RELOC_LENGTH_BITS` — the upper 29 bits, signed.
macro_rules! emacs_reloc_length {
    ($reloc:expr) => { (($reloc).raw as i32) >> EMACS_RELOC_TYPE_BITS };
}
macro_rules! emacs_reloc_set_length {
    ($reloc:expr, $length:expr) => {
        ($reloc).raw = (($reloc).raw & ((1u32 << EMACS_RELOC_TYPE_BITS) - 1))
            | ((($length as u32) & ((1u32 << EMACS_RELOC_LENGTH_BITS) - 1))
               << EMACS_RELOC_TYPE_BITS)
    };
}

/* Set the type of an Emacs relocation.

   Also make sure that the type fits in the bitfield.  */
fn emacs_reloc_set_type (reloc: *mut emacs_reloc, r#type: emacs_reloc_type)
{
  unsafe {
    let r#type: c_int = r#type as c_int;
    emacs_reloc_set_type_field! (*reloc, r#type);
    eassert! (emacs_reloc_type_of! (*reloc) == r#type);
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_table_locator
{
  /* Offset in dump, in bytes, of the first entry in the dump
     table.  */
  pub offset: dump_off,
  /* Number of entries in the dump table.  We need an explicit end
     indicator (as opposed to a special sentinel) so we can efficiently
     binary search over the relocation entries.  */
  pub nr_entries: dump_off,
}

pub const DUMP_RELOC_TYPE_BITS: c_int = 5;

/* Minimum alignment required by dump file format.  Lisp_Objects and raw
   pointers can both be relocated, so this can be any integer power of 2
   up to min (alignof (Lisp_Object), alignof (void *)).
   Larger values help dump_reloc_set_offset support larger offsets.  */
/* C: `min (alignof (Lisp_Object), alignof (void *))` — `min` is not a `const fn` on this
   toolchain, so the fold is written out (the BUF_BYTES_MAX row's class in
   docs/ports/macro-body-notes.tsv).  */
pub const DUMP_RELOCATION_ALIGNMENT: c_int =
  (if core::mem::align_of::<Lisp_Object> () < core::mem::align_of::<*mut c_void> ()
     { core::mem::align_of::<Lisp_Object> () }
     else { core::mem::align_of::<*mut c_void> () }) as c_int;

/* The alignment granularity (in bytes) for objects we store in the
   dump.  Always suitable for heap objects; may be more aligned.  */
/* C: `max (GCALIGNMENT, DUMP_RELOCATION_ALIGNMENT)` (the same fold's reason).  */
pub const DUMP_ALIGNMENT: c_int =
  (if GCALIGNMENT > DUMP_RELOCATION_ALIGNMENT as usize
     { GCALIGNMENT } else { DUMP_RELOCATION_ALIGNMENT as usize }) as c_int;

pub const DUMP_RELOC_OFFSET_BITS: c_int = DUMP_OFF_WIDTH - DUMP_RELOC_TYPE_BITS;

static_assert! (dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int + 8 < (1 << DUMP_RELOC_TYPE_BITS));
static_assert! (DUMP_ALIGNMENT as usize >= GCALIGNMENT);

/* C 292-296's two bitfields (`unsigned int raw_offset : DUMP_RELOC_OFFSET_BITS;` and
   `ENUM_BF (dump_reloc_type) type : DUMP_RELOC_TYPE_BITS;`) share one 4-byte unit, which
   the port stores as `raw` (the measurement is sizeof 4, matching `sizeof (dump_off)`: the
   `static_assert` at C 298 is the assertion below).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_reloc
{
  raw: u32,
}
static_assert! (core::mem::size_of::<dump_reloc> () == core::mem::size_of::<dump_off> ());

macro_rules! dump_reloc_set_type_field {
    ($reloc:expr, $type:expr) => {
        ($reloc).raw = (($reloc).raw & ((1u32 << DUMP_RELOC_OFFSET_BITS) - 1))
            | (($type as u32) << DUMP_RELOC_OFFSET_BITS)
    };
}
macro_rules! dump_reloc_type_of {
    ($reloc:expr) => { (($reloc).raw >> DUMP_RELOC_OFFSET_BITS) as c_int };
}

/* Set the type of a dump relocation.

   Also assert that the type fits in the bitfield.  */
fn dump_reloc_set_type (reloc: *mut dump_reloc, r#type: c_int)
{
  unsafe {
    /* PORTING.md 0.2: C's `type` is a Rust keyword, so the port uses the raw identifier
       (the same name, `type`).

       PORTING.md P-98: C's parameter is `enum dump_reloc_type` (C 300), but the value its one
       caller passes is **not always one of that enum's members** - C 1461 writes
       `RELOC_DUMP_TO_DUMP_LV + type` and C 1510 `RELOC_DUMP_TO_EMACS_LV + type` with
       `type = XTYPE (obj)`, i.e. 6 + {0,3,4,5,7} = {6,9,10,11,13} and 14 + {0,3,4,5,7} =
       {14,17,18,19,21} (the `Lisp_Type` values this configuration selects, `include/lisp.rs`
       93-101), while the enum's own members are 0-6 and 14.  C only ever stores that number in a
       5-bit field (C 291-292) - the enum is a spelling, the storage is an integer - so the port
       carries the value as the number: `core::mem::transmute` into the port's `dump_reloc_type`
       would build an **invalid discriminant** and panic on 9/10/11/13/17/18/19/21, which is
       exactly where the dump stopped (status 3.640).  */
    /* C's bitfield truncates (`type : DUMP_RELOC_TYPE_BITS`), so the mask is C's own.  */
    let r#type: c_int = r#type & ((1 << DUMP_RELOC_TYPE_BITS) - 1);
    dump_reloc_set_type_field! (*reloc, r#type);
    eassert! (dump_reloc_type_of! (*reloc) == r#type);
  }
}

fn dump_reloc_get_offset (reloc: dump_reloc) -> dump_off
{
  return (reloc.raw as dump_off & ((1 << DUMP_RELOC_OFFSET_BITS) - 1))
    * DUMP_RELOCATION_ALIGNMENT;
}

fn dump_reloc_set_offset (reloc: *mut dump_reloc, offset: dump_off)
{
  eassume! (offset >= 0);
  unsafe {
    (*reloc).raw = (((offset / DUMP_RELOCATION_ALIGNMENT) as u32)
                    & ((1u32 << DUMP_RELOC_OFFSET_BITS) - 1))
      | ((*reloc).raw & !((1u32 << DUMP_RELOC_OFFSET_BITS) - 1));
    if dump_reloc_get_offset (*reloc) != offset
      {
        error (c"dump relocation not an integral multiple of %d: %d".as_ptr (),
               DUMP_RELOCATION_ALIGNMENT, offset);
      }
  }
}

/* C 322-331's `dump_fingerprint` is not `static` (pdumper.h 49-50 declares it and
   emacs.c calls it), so the port exports it (PORTING.md 0.2).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_fingerprint (output: *mut FILE, label: *const c_char,
                                           xfingerprint: *const c_uchar)
{
  const hexbuf_size: c_int = 2 * (unsafe { core::mem::size_of_val (&fingerprint) }) as c_int;
  let mut hexbuf: [c_char; hexbuf_size as usize] = [0; hexbuf_size as usize];
  unsafe {
    hexbuf_digest (hexbuf.as_mut_ptr (), xfingerprint as *const c_void,
                   core::mem::size_of_val (&fingerprint) as c_int);
    fprintf (output, c"%s%s%.*s\n".as_ptr (), label,
             if *label != 0 { c": ".as_ptr () } else { c"".as_ptr () },
             hexbuf_size, hexbuf.as_ptr ());
  }
}

/* To be used if some order in the relocation process has to be enforced. */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum reloc_phase
  {
    /* First to run.  Place every relocation with no dependency here.  */
    EARLY_RELOCS,
    /* Late and very late relocs are relocated at the very last after
       all hooks has been run.  All lisp machinery is at disposal
       (memory allocation allowed too).  */
    LATE_RELOCS,
    VERY_LATE_RELOCS,
    /* Fake, must be last.  */
    RELOC_NUM_PHASES
  }

/* Format of an Emacs dump file.  All offsets are relative to
   the beginning of the file.  An Emacs dump file is coupled
   to exactly the Emacs binary that produced it, so details of
   alignment and endianness are unimportant.

   An Emacs dump file contains the contents of the Lisp heap.
   On startup, Emacs can start faster by mapping a dump file into
   memory and using the objects contained inside it instead of
   performing initialization from scratch.

   The dump file can be loaded at arbitrary locations in memory, so it
   includes a table of relocations that let Emacs adjust the pointers
   embedded in the dump file to account for the location where it was
   actually loaded.

   Dump files can contain pointers to other objects in the dump file
   or to parts of the Emacs binary.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_header
{
  /* File type magic.  */
  pub magic: [c_char; 16],

  /* Associated Emacs binary.  */
  pub fingerprint: [c_uchar; 32],

  /* Relocation table for the dump file; each entry is a
     struct dump_reloc.  */
  /* C: `struct dump_table_locator dump_relocs[RELOC_NUM_PHASES];` — `RELOC_NUM_PHASES` is
     `3` (`EARLY_RELOCS`, `LATE_RELOCS`, `VERY_LATE_RELOCS`; the clang measurement of
     `sizeof (struct dump_header)` is 100, which the 3-entry array gives).  */
  pub dump_relocs: [dump_table_locator; reloc_phase::RELOC_NUM_PHASES as usize],

  /* "Relocation" table we abuse to hold information about the
     location and type of each lisp object in the dump.  We need for
     pdumper_object_type and ultimately for conservative GC
     correctness.  */
  pub object_starts: dump_table_locator,

  /* Relocation table for Emacs; each entry is a struct
     emacs_reloc.  */
  pub emacs_relocs: dump_table_locator,

  /* Start of sub-region of hot region that we can discard after load
     completes.  The discardable region ends at cold_start.

     This region contains objects that we copy into the Emacs image at
     dump-load time.  */
  pub discardable_start: dump_off,

  /* Start of the region that does not require relocations and that we
     expect never to be modified.  This region can be memory-mapped
     directly from the backing dump file with the reasonable
     expectation of taking few copy-on-write faults.

     For correctness, however, this region must be modifible, since in
     rare cases it is possible to see modifications to these bytes.
     For example, this region contains string data, and it's
     technically possible for someone to ASET a string character
     (although nobody tends to do that).

     The start of the cold region is always aligned on a page
     boundary.  */
  pub cold_start: dump_off,

  /* Offset of a vector of the dumped hash tables.  */
  pub hash_list: dump_off,
}

/* Double-ended singly linked list.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_tailq
{
  pub head: Lisp_Object,
  pub tail: Lisp_Object,
  pub length: intptr_t,
}

/* Queue of objects to dump.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_queue
{
  /* Objects with no link weights at all.  Kept in dump order.  */
  pub zero_weight_objects: dump_tailq,
  /* Objects with simple link weight: just one entry of type
     WEIGHT_NORMAL.  Score in this special case is non-decreasing as
     position increases, so we can avoid the need to rescan a big list
     for each object by storing these objects in order.  */
  pub one_weight_normal_objects: dump_tailq,
  /* Likewise, for objects with one WEIGHT_STRONG weight.  */
  pub one_weight_strong_objects: dump_tailq,
  /* List of objects with complex link weights --- i.e., not one of
     the above cases.  Order is irrelevant, since we scan the whole
     list every time.  Relatively few objects end up here.  */
  pub fancy_weight_objects: dump_tailq,
  /* Hash table of link weights: maps an object to a list of zero or
     more (BASIS . WEIGHT) pairs.  As a special case, an object with
     zero weight is marked by Qt in the hash table --- this way, we
     can distinguish objects we've seen but that have no weight from
     ones that we haven't seen at all.  */
  pub link_weights: Lisp_Object,
  /* Hash table mapping object to a sequence number --- used to
     resolve ties.  */
  pub sequence_numbers: Lisp_Object,
  pub next_sequence_number: dump_off,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum cold_op
  {
    COLD_OP_OBJECT,
    COLD_OP_STRING,
    COLD_OP_CHARSET,
    COLD_OP_BUFFER,
    COLD_OP_BIGNUM,
    COLD_OP_NATIVE_SUBR,
  }

/* This structure controls what operations we perform inside
   dump_object.  */
/* C 465-491's eight `bool_bf : 1` members share one byte on the frozen target
   (`bool_bf` is one byte, conf_post.h), so the port stores the unit as `bits` and names
   the eight bits below (PORTING.md 1.3; the itree_node.bits shape).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_flags
{
  pub bits: u8,
}

/// C's `ctx->flags.<field> = true/false;` on the packed byte: the port writes the same bit
/// through this macro, which keeps C's boolean literal in the body (the `true`/`false`
/// tokens the token check needs, PORTING.md 1.3).
macro_rules! set_flag {
    ($flags:expr, $bit:expr, $value:expr) => {
        if $value { ($flags) |= $bit } else { ($flags) &= !$bit }
    };
}

pub const DUMP_FLAGS_DUMP_OBJECT_CONTENTS: u8 = 1 << 0;
pub const DUMP_FLAGS_RECORD_OBJECT_STARTS: u8 = 1 << 1;
pub const DUMP_FLAGS_PACK_OBJECTS: u8 = 1 << 2;
pub const DUMP_FLAGS_ASSERT_ALREADY_SEEN: u8 = 1 << 3;
pub const DUMP_FLAGS_DEFER_HASH_TABLES: u8 = 1 << 4;
pub const DUMP_FLAGS_DEFER_SYMBOLS: u8 = 1 << 5;
pub const DUMP_FLAGS_DEFER_COLD_OBJECTS: u8 = 1 << 6;
pub const DUMP_FLAGS_DEFER_COPIED_OBJECTS: u8 = 1 << 7;

/* Information we use while we dump.  Note that we're not the garbage
   collector and can operate under looser constraints: specifically,
   we allocate memory during the dumping process.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_context
{
  /* Header we'll write to the dump file when done.  */
  pub header: dump_header,
  /* Data that will be written to the dump file.  */
  pub buf: *mut c_void,
  pub buf_size: dump_off,
  pub max_offset: dump_off,

  pub old_purify_flag: Lisp_Object,
  pub old_post_gc_hook: Lisp_Object,
  pub old_process_environment: Lisp_Object,

  /* C 506-508's `#ifdef REL_ALLOC bool blocked_ralloc;` is dead (`REL_ALLOC` is undef in
     the frozen configuration); it is carried here so the field list and a `rel_alloc`
     build both stay complete (P-51).  */
  #[cfg (feature = "rel_alloc")]
  pub blocked_ralloc: bool,

  /* File descriptor for dumpfile; < 0 if closed.  */
  pub fd: c_int,
  /* Name of dump file --- used for error reporting.  */
  pub dump_filename: Lisp_Object,
  /* Current offset in dump file.  */
  pub offset: dump_off,

  /* Starting offset of current object.  */
  pub obj_offset: dump_off,

  /* Flags currently in effect for dumping.  */
  pub flags: dump_flags,

  pub end_heap: dump_off,

  /* Hash mapping objects we've already dumped to their offsets.  */
  pub objects_dumped: Lisp_Object,

  /* Hash mapping objects to where we got them.  Used for debugging.  */
  pub referrers: Lisp_Object,
  pub current_referrer: Lisp_Object,
  pub have_current_referrer: bool,

  /* Queue of objects to dump.  */
  pub dump_queue: dump_queue,

  /* Deferred object lists.  */
  pub deferred_hash_tables: Lisp_Object,
  pub deferred_symbols: Lisp_Object,

  /* Fixups in the dump file.  */
  pub fixups: Lisp_Object,

  /* Hash table of staticpro values: avoids double relocations.  */
  pub staticpro_table: Lisp_Object,

  /* Hash table mapping symbols to their pre-copy-queue fwd or blv
     structures (which we dump immediately before the start of the
     discardable section). */
  pub symbol_aux: Lisp_Object,
  /* Queue of copied objects for special treatment.  */
  pub copied_queue: Lisp_Object,
  /* Queue of cold objects to dump.  */
  pub cold_queue: Lisp_Object,

  /* Relocations in the dump.  */
  pub dump_relocs: [Lisp_Object; reloc_phase::RELOC_NUM_PHASES as usize],

  /* Object starts.  */
  pub object_starts: Lisp_Object,

  /* Relocations in Emacs.  */
  pub emacs_relocs: Lisp_Object,

  /* Hash table mapping bignums to their _data_ blobs, which we store
     in the cold section.  The actual Lisp_Bignum objects are normal
     heap objects.  */
  pub bignum_data: Lisp_Object,

  /* List of hash tables that have been dumped.  */
  pub hash_tables: Lisp_Object,

  pub number_hot_relocations: dump_off,
  pub number_discardable_relocations: dump_off,
}

/* These special values for use as offsets in dump_remember_object and
   dump_recall_object indicate that the corresponding object isn't in
   the dump yet (and so it has no valid offset), but that it's on one
   of our to-be-dumped-later object queues (or that we haven't seen it
   at all).  All values must be non-positive, since positive values
   are physical dump offsets.  */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_object_special_offset
  {
   DUMP_OBJECT_IS_RUNTIME_MAGIC = -6,
   DUMP_OBJECT_ON_COPIED_QUEUE = -5,
   DUMP_OBJECT_ON_HASH_TABLE_QUEUE = -4,
   DUMP_OBJECT_ON_SYMBOL_QUEUE = -3,
   DUMP_OBJECT_ON_COLD_QUEUE = -2,
   DUMP_OBJECT_ON_NORMAL_QUEUE = -1,
   DUMP_OBJECT_NOT_SEEN = 0,
  }

/* Weights for score scores for object non-locality.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct link_weight
{
  /* Wrapped in a struct to break unwanted implicit conversion.  */
  pub value: c_int,
}

static WEIGHT_NONE: link_weight = link_weight { value: 0 };
static WEIGHT_NORMAL: link_weight = link_weight { value: 1000 };
static WEIGHT_STRONG: link_weight = link_weight { value: 1200 };

/* Dump file creation */

/* Like the Lisp function `push'.  Return NEWELT.  */
fn dump_push (r#where: *mut Lisp_Object, newelt: Lisp_Object) -> Lisp_Object
{
  unsafe {
    *r#where = Fcons (newelt, *r#where);
  }
  return newelt;
}

/* Like the Lisp function `pop'.  */
fn dump_pop (r#where: *mut Lisp_Object) -> Lisp_Object
{
  unsafe {
    let ret: Lisp_Object = XCAR (*r#where);
    *r#where = XCDR (*r#where);
    return ret;
  }
}

fn dump_tracking_referrers_p (ctx: *mut dump_context) -> bool
{
  return unsafe { !NILP ((*ctx).referrers) };
}

fn dump_set_have_current_referrer (ctx: *mut dump_context, have: bool)
{
  #[cfg (enable_checking)]
  unsafe { (*ctx).have_current_referrer = have; }
}

/* Return true if objects should be enqueued in CTX to refer to an
   object that the caller should store into CTX->current_referrer.

   Until dump_clear_referrer is called, any objects enqueued are being
   enqueued because the object refers to them.  It is not valid to
   enqueue objects without a referrer set.  We check this constraint
   at runtime.

   It is invalid to call dump_set_referrer twice without an
   intervening call to dump_clear_referrer.  */
fn dump_set_referrer (ctx: *mut dump_context) -> bool
{
  eassert! (!unsafe { (*ctx).have_current_referrer });
  dump_set_have_current_referrer (ctx, true);
  return dump_tracking_referrers_p (ctx);
}

/* Unset the referrer that dump_set_referrer prepared for.  */
fn dump_clear_referrer (ctx: *mut dump_context)
{
  eassert! (unsafe { (*ctx).have_current_referrer });
  dump_set_have_current_referrer (ctx, false);
  if dump_tracking_referrers_p (ctx)
    { unsafe { (*ctx).current_referrer = Qnil (); } }
}

fn dump_ptr_referrer (label: *const c_char, address: *const c_void) -> Lisp_Object
{
  let mut buf: [c_char; 128] = [0; 128];
  if core::mem::size_of::<[c_char; 128]> () <=
    (unsafe { snprintf (buf.as_mut_ptr (), core::mem::size_of::<[c_char; 128]> (),
                        c"%s @ %p".as_ptr (), label, address) }) as usize
    {
      unsafe {
        strcpy (buf.as_mut_ptr ().add (core::mem::size_of::<[c_char; 128]> () - 4),
                c"...".as_ptr ());
      }
    }
  return unsafe { build_string (buf.as_ptr ()) };
}

/* C 697's `static AVOID error_unsupported_dump_object (...)`: `AVOID` is
   `_Noreturn ATTRIBUTE_COLD void` (lisp.h 153's definition), so the Rust definition is `-> !`
   and the same `error` (also an AVOID) never returns.  */
fn error_unsupported_dump_object (ctx: *mut dump_context, object: Lisp_Object,
                                  msg: *const c_char) -> !
{
  if dump_tracking_referrers_p (ctx)
    { print_paths_to_root (ctx, object); }
  unsafe { error (c"unsupported object type in dump: %s".as_ptr (), msg); }
}

fn emacs_basis () -> uintptr_t
{
  return unsafe { (&raw const Vpurify_flag) as uintptr_t };
}

fn emacs_ptr_at (offset: ptrdiff_t) -> *mut c_void
{
  /* TODO: assert somehow that the result is actually in the Emacs
     image.  */
  return (emacs_basis () as isize).wrapping_add (offset) as *mut c_void;
}

fn emacs_offset (emacs_ptr: *const c_void) -> dump_off
{
  /* TODO: assert that EMACS_PTR is actually in the Emacs image.  */
  eassert! (!emacs_ptr.is_null ());
  let emacs_ptr_value: intptr_t = emacs_ptr as intptr_t;
  let emacs_ptr_relative: ptrdiff_t = emacs_ptr_value - (emacs_basis () as intptr_t);
  return ptrdiff_t_to_dump_off (emacs_ptr_relative);
}

/* Return whether OBJECT is a symbol the storage of which is built into
   Emacs (and hence is implicitly offset from an address in the Emacs
   image).  */

fn dump_builtin_symbol_p (object: Lisp_Object) -> bool
{
  /* Symbols with position cannot be dumped, but not returning true for
     them avoids producing references to unrelocated Lisp_Objects in
     fixup processing or depending on the value of
     symbols_with_pos_enabled.  */
  unsafe { return BARE_SYMBOL_P (object) && c_symbol_p (XSYMBOL (object)); }
}

/* Return whether OBJECT has the same bit pattern in all Emacs
   invocations --- i.e., is invariant across a dump.  Note that some
   self-representing objects still need to be dumped!
*/
fn dump_object_self_representing_p (object: Lisp_Object) -> bool
{
  return FIXNUMP (object) || dump_builtin_symbol_p (object);
}

fn intmax_t_from_lisp (value: Lisp_Object) -> intmax_t
{
  let mut n: intmax_t = 0;
  let ok: bool = unsafe { integer_to_intmax (value, &mut n) };
  eassert! (ok);
  return n;
}

fn intmax_t_to_lisp (value: intmax_t) -> Lisp_Object
{
  return INT_TO_INTEGER (value as EMACS_INT);
}

fn dump_off_from_lisp (value: Lisp_Object) -> dump_off
{
  let n: intmax_t = intmax_t_from_lisp (value);
  eassert! (DUMP_OFF_MIN as intmax_t <= n && n <= DUMP_OFF_MAX as intmax_t);
  return n as dump_off;
}

fn dump_off_to_lisp (value: dump_off) -> Lisp_Object
{
  return INT_TO_INTEGER (value as EMACS_INT);
}

fn dump_write (ctx: *mut dump_context, buf: *const c_void, nbyte: dump_off)
{
  unsafe {
    eassert! (nbyte == 0 || !buf.is_null ());
    eassert! ((*ctx).obj_offset == 0);
    eassert! (((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0);
    let avail: dump_off = (*ctx).buf_size - (*ctx).offset;
    if avail < nbyte
      {
        static_assert! (DUMP_OFF_MAX as intmax_t <= PTRDIFF_MAX as intmax_t);
        let mut buf_size: ptrdiff_t = (*ctx).buf_size as ptrdiff_t;
        (*ctx).buf = xpalloc ((*ctx).buf, &mut buf_size,
                              max ((nbyte - avail) as isize, 8 * 1024 * 1024), 
                              DUMP_OFF_MAX as ptrdiff_t, 1);
        (*ctx).buf_size = buf_size as dump_off;
      }
    memcpy (((*ctx).buf as *mut c_char).add ((*ctx).offset as usize) as *mut c_void,
            buf, nbyte as usize);
    (*ctx).offset += nbyte;
  }
}

fn make_eq_hash_table () -> Lisp_Object
{
  return CALLN! (Fmake_hash_table, QCtest (), Qeq ());
}

fn dump_tailq_init (tailq: *mut dump_tailq)
{
  unsafe {
    (*tailq).head = Qnil ();
    (*tailq).tail = Qnil ();
    (*tailq).length = 0;
  }
}

fn dump_tailq_length (tailq: *const dump_tailq) -> intptr_t
{
  return unsafe { (*tailq).length };
}

fn dump_tailq_prepend (tailq: *mut dump_tailq, value: Lisp_Object)
{
  unsafe {
    let link: Lisp_Object = Fcons (value, (*tailq).head);
    (*tailq).head = link;
    if NILP ((*tailq).tail)
      { (*tailq).tail = link; }
    (*tailq).length += 1;
  }
}

fn dump_tailq_empty_p (tailq: *mut dump_tailq) -> bool
{
  return unsafe { NILP ((*tailq).head) };
}

fn dump_tailq_peek (tailq: *mut dump_tailq) -> Lisp_Object
{
  eassert! (!dump_tailq_empty_p (tailq));
  return unsafe { XCAR ((*tailq).head) };
}

fn dump_tailq_pop (tailq: *mut dump_tailq) -> Lisp_Object
{
  eassert! (!dump_tailq_empty_p (tailq));
  eassert! (unsafe { (*tailq).length } > 0);
  unsafe {
    (*tailq).length -= 1;
    let value: Lisp_Object = XCAR ((*tailq).head);
    (*tailq).head = XCDR ((*tailq).head);
    if NILP ((*tailq).head)
      { (*tailq).tail = Qnil (); }
    return value;
  }
}

fn dump_seek (ctx: *mut dump_context, offset: dump_off)
{
  unsafe {
    if (*ctx).max_offset < (*ctx).offset
      { (*ctx).max_offset = (*ctx).offset; }
    eassert! ((*ctx).obj_offset == 0);
    (*ctx).offset = offset;
  }
}

fn dump_write_zero (ctx: *mut dump_context, nbytes: dump_off)
{
  let mut nbytes = nbytes;
  while nbytes > 0
    {
      let zero: uintmax_t = 0;
      let mut to_write: dump_off = core::mem::size_of_val (&zero) as dump_off;
      if to_write > nbytes
        { to_write = nbytes; }
      dump_write (ctx, (&raw const zero) as *const c_void, to_write);
      nbytes -= to_write;
    }
}

fn dump_align_output (ctx: *mut dump_context, alignment: c_int)
{
  unsafe {
    if (*ctx).offset % alignment != 0
      { dump_write_zero (ctx, alignment - ((*ctx).offset % alignment)); }
  }
}

fn dump_object_start (ctx: *mut dump_context, out: *mut c_void, outsz: dump_off) -> dump_off
{
  /* We dump only one object at a time, so obj_offset should be
     invalid on entry to this function.  */
  eassert! (unsafe { (*ctx).obj_offset } == 0);
  let alignment: c_int =
    if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_PACK_OBJECTS) != 0 } { 1 } else { DUMP_ALIGNMENT };
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0 }
    { dump_align_output (ctx, alignment); }
  unsafe {
    (*ctx).obj_offset = (*ctx).offset;
    memset (out, 0, outsz as usize);
    return (*ctx).offset;
  }
}

fn dump_object_finish (ctx: *mut dump_context, out: *const c_void, sz: dump_off) -> dump_off
{
  let offset: dump_off = unsafe { (*ctx).obj_offset };
  eassert! (offset > 0);
  eassert! (offset == unsafe { (*ctx).offset }); /* No intervening writes.  */
  unsafe { (*ctx).obj_offset = 0; }
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0 }
    { dump_write (ctx, out, sz); }
  return offset;
}

/* Return offset at which OBJECT has been dumped, or one of the dump_object_special_offset
   negative values, or DUMP_OBJECT_NOT_SEEN.  */
fn dump_recall_object (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  let dumped: Lisp_Object = unsafe { (*ctx).objects_dumped };
  return dump_off_from_lisp (unsafe {
    Fgethash (object, dumped, make_fixnum (dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as isize))
  });
}

fn dump_remember_object (ctx: *mut dump_context, object: Lisp_Object, offset: dump_off)
{
  unsafe {
    Fputhash (object, dump_off_to_lisp (offset), (*ctx).objects_dumped);
  }
}

fn dump_note_reachable (ctx: *mut dump_context, object: Lisp_Object)
{
  eassert! (unsafe { (*ctx).have_current_referrer });
  if !dump_tracking_referrers_p (ctx)
    { return; }
  unsafe {
    let referrer: Lisp_Object = (*ctx).current_referrer;
    let obj_referrers: Lisp_Object = Fgethash (object, (*ctx).referrers, Qnil ());
    if NILP (Fmemq (referrer, obj_referrers))
      { Fputhash (object, Fcons (referrer, obj_referrers), (*ctx).referrers); }
  }
}

/* If this object lives in the Emacs image and not on the heap, return
   a pointer to the object data.  Otherwise, return NULL.  */
fn dump_object_emacs_ptr (lv: Lisp_Object) -> *mut c_void
{
  unsafe {
    if SUBRP (lv) && !NATIVE_COMP_FUNCTIONP (lv)
      { return XSUBR (lv) as *mut c_void; }
    if dump_builtin_symbol_p (lv)
      { return XSYMBOL (lv) as *mut c_void; }
    if XTYPE (lv) == Lisp_Type::Lisp_Vectorlike
      && PSEUDOVECTOR_TYPEP (&raw mut (*XVECTOR (lv)).header, pvec_type::PVEC_THREAD)
      && main_thread_p (XTHREAD (lv) as *const c_void)
      { return XTHREAD (lv) as *mut c_void; }
    return core::ptr::null_mut ();
  }
}

fn dump_queue_init (dump_queue: *mut dump_queue)
{
  unsafe {
    dump_tailq_init (&raw mut (*dump_queue).zero_weight_objects);
    dump_tailq_init (&raw mut (*dump_queue).one_weight_normal_objects);
    dump_tailq_init (&raw mut (*dump_queue).one_weight_strong_objects);
    dump_tailq_init (&raw mut (*dump_queue).fancy_weight_objects);
    (*dump_queue).link_weights = make_eq_hash_table ();
    (*dump_queue).sequence_numbers = make_eq_hash_table ();
    (*dump_queue).next_sequence_number = 1;
  }
}

fn dump_queue_empty_p (dump_queue: *mut dump_queue) -> bool
{
  unsafe {
    let count: ptrdiff_t = (*XHASH_TABLE ((*dump_queue).sequence_numbers)).count as ptrdiff_t;
    let is_empty: bool = count == 0;
    eassert! (count == XFIXNAT (Fhash_table_count ((*dump_queue).link_weights)) as ptrdiff_t);
    if !is_empty
      {
        eassert! (!dump_tailq_empty_p (&raw mut (*dump_queue).zero_weight_objects)
                 || !dump_tailq_empty_p (&raw mut (*dump_queue).one_weight_normal_objects)
                 || !dump_tailq_empty_p (&raw mut (*dump_queue).one_weight_strong_objects)
                 || !dump_tailq_empty_p (&raw mut (*dump_queue).fancy_weight_objects));
      }
    else
      {
        /* If we're empty, we can still have a few stragglers on one of
           the above queues.  */
      }

    return is_empty;
  }
}

fn dump_queue_push_weight (weight_list: *mut Lisp_Object, basis: dump_off,
                           weight: link_weight)
{
  unsafe {
    if EQ (*weight_list, Qt ())
      { *weight_list = Qnil (); }
    dump_push (weight_list, Fcons (dump_off_to_lisp (basis),
                                   dump_off_to_lisp (weight.value as dump_off)));
  }
}

fn dump_queue_enqueue (dump_queue: *mut dump_queue, object: Lisp_Object, basis: dump_off,
                       weight: link_weight)
{
  unsafe {
    let mut weights: Lisp_Object = Fgethash (object, (*dump_queue).link_weights, Qnil ());
    let orig_weights: Lisp_Object = weights;
    /* N.B. want to find the last item of a given weight in each queue
       due to prepend use.  */
    let use_single_queues: bool = true;
    if NILP (weights)
      {
        /* Object is new.  */
        let uobj: EMACS_UINT = XLI (object) as EMACS_UINT;
        dump_trace (c"new object %0*lx weight=%d\n".as_ptr (), EMACS_INT_XDIGITS, uobj,
                    weight.value);

        if weight.value == WEIGHT_NONE.value
          {
            eassert! (weight.value == 0);
            dump_tailq_prepend (&raw mut (*dump_queue).zero_weight_objects, object);
            weights = Qt ();
          }
        else if !use_single_queues
          {
            dump_tailq_prepend (&raw mut (*dump_queue).fancy_weight_objects, object);
            dump_queue_push_weight (&mut weights, basis, weight);
          }
        else if weight.value == WEIGHT_NORMAL.value
          {
            dump_tailq_prepend (&raw mut (*dump_queue).one_weight_normal_objects, object);
            dump_queue_push_weight (&mut weights, basis, weight);
          }
        else if weight.value == WEIGHT_STRONG.value
          {
            dump_tailq_prepend (&raw mut (*dump_queue).one_weight_strong_objects, object);
            dump_queue_push_weight (&mut weights, basis, weight);
          }
        else
          {
            emacs_abort ();
          }

        Fputhash (object,
                  dump_off_to_lisp ({ let v = (*dump_queue).next_sequence_number;
                                      (*dump_queue).next_sequence_number += 1; v }),
                  (*dump_queue).sequence_numbers);
      }
    else
      {
        /* Object was already on the queue.  It's okay for an object to
           be on multiple queues so long as we maintain order
           invariants: attempting to dump an object multiple times is
           harmless, and most of the time, an object is only referenced
           once before being dumped, making this code path uncommon.  */
        if weight.value != WEIGHT_NONE.value
          {
            if EQ (weights, Qt ())
              {
                /* Object previously had a zero weight.  Once we
                   incorporate the link weight attached to this call,
                   the object will have a single weight.  Put the object
                   on the appropriate single-weight queue.  */
                weights = Qnil ();
                let tailq: *mut dump_tailq;
                if !use_single_queues
                  { tailq = &raw mut (*dump_queue).fancy_weight_objects; }
                else if weight.value == WEIGHT_NORMAL.value
                  { tailq = &raw mut (*dump_queue).one_weight_normal_objects; }
                else if weight.value == WEIGHT_STRONG.value
                  { tailq = &raw mut (*dump_queue).one_weight_strong_objects; }
                else
                  { emacs_abort (); }
                dump_tailq_prepend (tailq, object);
              }
            else if use_single_queues && NILP (XCDR (weights))
              { dump_tailq_prepend (&raw mut (*dump_queue).fancy_weight_objects, object); }
            dump_queue_push_weight (&mut weights, basis, weight);
          }
      }

    if !BASE_EQ (weights, orig_weights)
      { Fputhash (object, weights, (*dump_queue).link_weights); }
  }
}

fn dump_calc_link_score (basis: dump_off, link_basis: dump_off, link_weight: dump_off)
  -> c_float
{
  let distance: c_float = (basis - link_basis) as c_float;
  eassert! (distance >= 0 as c_float);
  let link_score: c_float = unsafe { powf (distance, -0.2) };
  return unsafe { powf (link_score, link_weight as c_float / 1000.0) };
}

/* Compute the score for a queued object.

   OBJECT is the object to query, which must currently be queued for
   dumping.  BASIS is the offset at which we would be
   dumping the object; score is computed relative to BASIS and the
   various BASIS values supplied to dump_add_link_weight --- the
   further an object is from its referrers, the greater the
   score.  */
fn dump_queue_compute_score (dump_queue: *mut dump_queue, object: Lisp_Object,
                             basis: dump_off) -> c_float
{
  let mut score: c_float = 0 as c_float;
  let mut object_link_weights: Lisp_Object =
    unsafe { Fgethash (object, (*dump_queue).link_weights, Qnil ()) };
  if unsafe { EQ (object_link_weights, Qt ()) }
    { object_link_weights = Qnil (); }
  while !unsafe { NILP (object_link_weights) }
    {
      let basis_weight_pair: Lisp_Object = dump_pop (&mut object_link_weights);
      let link_basis: dump_off = dump_off_from_lisp (unsafe { XCAR (basis_weight_pair) });
      let link_weight: dump_off = dump_off_from_lisp (unsafe { XCDR (basis_weight_pair) });
      score += dump_calc_link_score (basis, link_basis, link_weight);
    }
  return score;
}

/* Scan the fancy part of the dump queue.

   BASIS is the position at which to evaluate the score function,
   usually ctx->offset.

   If we have at least one entry in the queue, return the pointer (in
   the singly-linked list) to the cons containing the object via
   *OUT_HIGHEST_SCORE_CONS_PTR and return its score.

   If the queue is empty, set *OUT_HIGHEST_SCORE_CONS_PTR to NULL
   and return negative infinity.  */
fn dump_queue_scan_fancy (dump_queue: *mut dump_queue, basis: dump_off,
                          out_highest_score_cons_ptr: *mut *mut Lisp_Object) -> c_float
{
  unsafe {
    let mut cons_ptr: *mut Lisp_Object = &raw mut (*dump_queue).fancy_weight_objects.head;
    let mut highest_score_cons_ptr: *mut Lisp_Object = core::ptr::null_mut ();
    let mut highest_score: c_float = -INFINITY;
    let mut first: bool = true;

    while !NILP (*cons_ptr)
      {
        let queued_object: Lisp_Object = XCAR (*cons_ptr);
        let score: c_float = dump_queue_compute_score (dump_queue, queued_object, basis);
        if first || score >= highest_score
          {
            highest_score_cons_ptr = cons_ptr;
            highest_score = score;
            if first
              { first = false; }
          }
        cons_ptr = &raw mut (*XCONS (*cons_ptr)).u.s.u.cdr;
      }

    *out_highest_score_cons_ptr = highest_score_cons_ptr;
    return highest_score;
  }
}

/* Return the sequence number of OBJECT.

   Return -1 if object doesn't have a sequence number.  This situation
   can occur when we've double-queued an object.  If this happens, we
   discard the errant object and try again.  */
fn dump_queue_sequence (dump_queue: *mut dump_queue, object: Lisp_Object) -> dump_off
{
  let n: Lisp_Object = unsafe { Fgethash (object, (*dump_queue).sequence_numbers, Qnil ()) };
  return if NILP (n) { -1 } else { dump_off_from_lisp (n) };
}

/* Find score and sequence at head of a one-weight object queue.

   Transparently discard stale objects from head of queue.  BASIS
   is the baseness for score computation.

   We organize these queues so that score is strictly decreasing, so
   examining the head is sufficient.  */
fn dump_queue_find_score_of_one_weight_queue (dump_queue: *mut dump_queue, basis: dump_off,
                                              one_weight_queue: *mut dump_tailq,
                                              out_score: *mut c_float,
                                              out_sequence: *mut c_int)
{
  /* Transparently discard stale objects from the head of this queue.  */
  loop
    {
      unsafe {
        if dump_tailq_empty_p (one_weight_queue)
          {
            *out_score = -INFINITY;
            *out_sequence = 0;
          }
        else
          {
            let head: Lisp_Object = dump_tailq_peek (one_weight_queue);
            *out_sequence = dump_queue_sequence (dump_queue, head) as c_int;
            if *out_sequence < 0
              { dump_tailq_pop (one_weight_queue); }
            else
              { *out_score = dump_queue_compute_score (dump_queue, head, basis); }
          }
      }
      if !(unsafe { *out_sequence } < 0)
        { break; }
    }
}

/* Pop the next object to dump from the dump queue.

   BASIS is the dump offset at which to evaluate score.

   The object returned is the queued object with the greatest score;
   by side effect, the object is removed from the dump queue.
   The dump queue must not be empty.  */
fn dump_queue_dequeue (dump_queue: *mut dump_queue, basis: dump_off) -> Lisp_Object
{
  eassert! (BASE_EQ (unsafe { Fhash_table_count ((*dump_queue).sequence_numbers) },
                     unsafe { Fhash_table_count ((*dump_queue).link_weights) }));

  eassert! (XFIXNUM (unsafe { Fhash_table_count ((*dump_queue).sequence_numbers) })
            <= (dump_tailq_length (unsafe { &raw const (*dump_queue).fancy_weight_objects })
                + dump_tailq_length (unsafe { &raw const (*dump_queue).zero_weight_objects })
                + dump_tailq_length (unsafe { &raw const (*dump_queue).one_weight_normal_objects })
                + dump_tailq_length (unsafe { &raw const (*dump_queue).one_weight_strong_objects })));

  /* C 1245-1252 splices `PRIdDUMP_OFF`/`PRIdPTR`/`PRIdHASH_IDX` into the format; the port
     keeps the same pieces (`concat!` expands the macros, PORTING.md 2.1).  */
  unsafe { dump_trace
    (concat! ("dump_queue_dequeue basis=%", PRIdDUMP_OFF! (), " fancy=%", PRIdPTR! (),
              " zero=%", PRIdPTR! (), " normal=%", PRIdPTR! (), " strong=%", PRIdPTR! (),
              " hash=%", PRIdHASH_IDX! (), "\n").as_ptr () as *const c_char,
     basis,
     dump_tailq_length (unsafe { &raw const (*dump_queue).fancy_weight_objects }) as c_long,
     dump_tailq_length (unsafe { &raw const (*dump_queue).zero_weight_objects }) as c_long,
     dump_tailq_length (unsafe { &raw const (*dump_queue).one_weight_normal_objects }) as c_long,
     dump_tailq_length (unsafe { &raw const (*dump_queue).one_weight_strong_objects }) as c_long,
     unsafe { (*XHASH_TABLE ((*dump_queue).link_weights)).count } as c_long); }

  /* C 1250's `#define nr_candidates 3` (a function-local macro; PORTING.md 2.1 keeps the
     name as a same-named constant, P-57's textual-scope rule does not apply to a `const`).  */
  const nr_candidates: c_int = 3;
  /* C's function-local `struct candidate` is declared with this unit's other records
     below (one definition only; Rust items are order-independent).  */
  let mut candidates: [candidate; nr_candidates as usize] =
    [candidate { score: 0 as c_float, sequence: 0 }; nr_candidates as usize];

  let mut fancy_cons: *mut Lisp_Object = core::ptr::null_mut ();
  unsafe {
    let mut best: c_int = -1;
    candidates[0].sequence = 0;
    loop
      {
        if candidates[0].sequence < 0
          { *fancy_cons = XCDR (*fancy_cons); }  /* Discard stale object.  */
        candidates[0].score = dump_queue_scan_fancy (dump_queue, basis, &mut fancy_cons);
        candidates[0].sequence =
          if candidates[0].score > -INFINITY
            { dump_queue_sequence (dump_queue, XCAR (*fancy_cons)) }
            else { 0 };
        if !(candidates[0].sequence < 0)
          { break; }
      }

    dump_queue_find_score_of_one_weight_queue
      (dump_queue, basis,
       &raw mut (*dump_queue).one_weight_normal_objects,
       &mut candidates[1].score,
       &mut candidates[1].sequence);
    candidates[1].sequence = candidates[1].sequence;

    dump_queue_find_score_of_one_weight_queue
      (dump_queue, basis,
       &raw mut (*dump_queue).one_weight_strong_objects,
       &mut candidates[2].score,
       &mut candidates[2].sequence);

    best = -1;
    for i in 0..nr_candidates
      {
        eassert! (candidates[i as usize].sequence >= 0);
        if candidates[i as usize].score > -INFINITY
            && (best < 0
                || candidates[i as usize].score > candidates[best as usize].score
                || (candidates[i as usize].score == candidates[best as usize].score
                    && candidates[i as usize].sequence < candidates[best as usize].sequence))
          { best = i; }
      }

    let result: Lisp_Object;
    let src: *const c_char;
    if best < 0
      {
        src = c"zero".as_ptr ();
        result = dump_tailq_pop (&raw mut (*dump_queue).zero_weight_objects);
      }
    else if best == 0
      {
        src = c"fancy".as_ptr ();
        result = dump_tailq_pop (&raw mut (*dump_queue).fancy_weight_objects);
      }
    else if best == 1
      {
        src = c"normal".as_ptr ();
        result = dump_tailq_pop (&raw mut (*dump_queue).one_weight_normal_objects);
      }
    else if best == 2
      {
        src = c"strong".as_ptr ();
        result = dump_tailq_pop (&raw mut (*dump_queue).one_weight_strong_objects);
      }
    else
      { emacs_abort (); }

    let uresult: EMACS_UINT = XLI (result) as EMACS_UINT;
    dump_trace (c"  result score=%f src=%s object=%0*lx\n".as_ptr (),
                if best < 0 { -1.0 } else { candidates[best as usize].score as c_double },
                src, EMACS_INT_XDIGITS, uresult);

    {
      let mut weights: Lisp_Object = Fgethash (result, (*dump_queue).link_weights, Qnil ());
      while !NILP (weights) && CONSP (weights)
        {
          let basis_weight_pair: Lisp_Object = dump_pop (&mut weights);
          let link_basis: dump_off = dump_off_from_lisp (XCAR (basis_weight_pair));
          let link_weight: dump_off = dump_off_from_lisp (XCDR (basis_weight_pair));
          dump_trace
            (c"    link_basis=%d distance=%d weight=%d contrib=%f\n".as_ptr (),
             link_basis,
             basis - link_basis,
             link_weight,
             dump_calc_link_score (basis, link_basis, link_weight) as c_double);
        }
    }

    Fremhash (result, (*dump_queue).link_weights);
    Fremhash (result, (*dump_queue).sequence_numbers);
    return result;
  }
}

/* Return whether we need to write OBJECT to the dump file.  */
fn dump_object_needs_dumping_p (object: Lisp_Object) -> bool
{
  return !(FIXNUMP (object));
}

fn dump_enqueue_object (ctx: *mut dump_context, object: Lisp_Object, weight: link_weight)
{
  unsafe {
    if dump_object_needs_dumping_p (object)
      {
        let mut state: dump_off = dump_recall_object (ctx, object);
        let already_dumped_object: bool =
          state > dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off;
        if ((*ctx).flags.bits & DUMP_FLAGS_ASSERT_ALREADY_SEEN) != 0
          { eassert! (already_dumped_object); }
        if !already_dumped_object
          {
            if state == dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off
              {
                state = dump_object_special_offset::DUMP_OBJECT_ON_NORMAL_QUEUE as dump_off;
                dump_remember_object (ctx, object, state);
              }
            /* Note that we call dump_queue_enqueue even if the object
               is already on the normal queue: multiple enqueue calls
               can increase the object's weight.  */
            if state == dump_object_special_offset::DUMP_OBJECT_ON_NORMAL_QUEUE as dump_off
              { dump_queue_enqueue (&raw mut (*ctx).dump_queue,
                                    object,
                                    (*ctx).offset,
                                    weight); }
          }
      }
    /* Always remember the path to this object.  */
    dump_note_reachable (ctx, object);
  }
}

fn print_paths_to_root_1 (ctx: *mut dump_context, object: Lisp_Object, level: c_int)
{
  unsafe {
    let mut referrers: Lisp_Object = Fgethash (object, (*ctx).referrers, Qnil ());
    while !NILP (referrers)
      {
        let referrer: Lisp_Object = XCAR (referrers);
        referrers = XCDR (referrers);
        let repr: Lisp_Object = Fprin1_to_string (referrer, Qnil (), Qnil ());
        for _i in 0..level
          { putc (b' ' as c_int, stderr); }
        fwrite (SDATA (repr) as *const c_void, 1, SBYTES (repr) as usize, stderr);
        putc (b'\n' as c_int, stderr);
        print_paths_to_root_1 (ctx, referrer, level + 1);
      }
  }
}

fn print_paths_to_root (ctx: *mut dump_context, object: Lisp_Object)
{
  print_paths_to_root_1 (ctx, object, 0);
}

fn dump_remember_cold_op (ctx: *mut dump_context, op: cold_op, arg: Lisp_Object)
{
  unsafe {
    if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      { dump_push (&raw mut (*ctx).cold_queue, Fcons (make_fixnum (op as isize), arg)); }
  }
}

/* Add a dump relocation that points into Emacs.

   Add a relocation that updates the pointer stored at DUMP_OFFSET to
   point into the Emacs binary upon dump load.  The pointer-sized
   value at DUMP_OFFSET in the dump file should contain a number
   relative to emacs_basis().  */
fn dump_reloc_dump_to_emacs_ptr_raw (ctx: *mut dump_context, dump_offset: dump_off)
{
  unsafe {
    if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      { dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::EARLY_RELOCS as usize],
                   list2 (make_fixnum (dump_reloc_type::RELOC_DUMP_TO_EMACS_PTR_RAW as isize),
                          dump_off_to_lisp (dump_offset))); }
  }
}

/* Add a dump relocation that points a Lisp_Object back at the dump.

   Add a relocation that updates the Lisp_Object at DUMP_OFFSET in the
   dump to point to another object in the dump.  The Lisp_Object-sized
   value at DUMP_OFFSET in the dump file should contain the offset of
   the target object relative to the start of the dump.  */
fn dump_reloc_dump_to_dump_lv (ctx: *mut dump_context, dump_offset: dump_off,
                               type_: Lisp_Type)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  let reloc_type: c_int;
  match type_
    {
      Lisp_Type::Lisp_Symbol
      | Lisp_Type::Lisp_String
      | Lisp_Type::Lisp_Vectorlike
      | Lisp_Type::Lisp_Cons
      | Lisp_Type::Lisp_Float
        => { reloc_type = dump_reloc_type::RELOC_DUMP_TO_DUMP_LV as c_int + type_ as c_int; }
      _ => unsafe { emacs_abort (); }
    }

  unsafe {
    dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::EARLY_RELOCS as usize],
               list2 (make_fixnum (reloc_type as isize),
                      dump_off_to_lisp (dump_offset)));
  }
}

/* Add a dump relocation that points a raw pointer back at the dump.

   Add a relocation that updates the raw pointer at DUMP_OFFSET in the
   dump to point to another object in the dump.  The pointer-sized
   value at DUMP_OFFSET in the dump file should contain the offset of
   the target object relative to the start of the dump.  */
fn dump_reloc_dump_to_dump_ptr_raw (ctx: *mut dump_context, dump_offset: dump_off)
{
  unsafe {
    if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      { dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::EARLY_RELOCS as usize],
                   list2 (make_fixnum (dump_reloc_type::RELOC_DUMP_TO_DUMP_PTR_RAW as isize),
                          dump_off_to_lisp (dump_offset))); }
  }
}

/* Add a dump relocation that points to a Lisp object in Emacs.

   Add a relocation that updates the Lisp_Object at DUMP_OFFSET in the
   dump to point to a lisp object in Emacs.  The Lisp_Object-sized
   value at DUMP_OFFSET in the dump file should contain the offset of
   the target object relative to emacs_basis().  TYPE is the type of
   Lisp value.  */
fn dump_reloc_dump_to_emacs_lv (ctx: *mut dump_context, dump_offset: dump_off,
                                type_: Lisp_Type)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  let reloc_type: c_int;
  match type_
    {
      Lisp_Type::Lisp_String
      | Lisp_Type::Lisp_Vectorlike
      | Lisp_Type::Lisp_Cons
      | Lisp_Type::Lisp_Float
        => { reloc_type = dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int + type_ as c_int; }
      _ => unsafe { emacs_abort (); }
    }

  unsafe {
    dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::EARLY_RELOCS as usize],
               list2 (make_fixnum (reloc_type as isize),
                      dump_off_to_lisp (dump_offset)));
  }
}

/* Add an Emacs relocation that copies arbitrary bytes from the dump.

   When the dump is loaded, Emacs copies SIZE bytes from OFFSET in
   dump to LOCATION in the Emacs data section.  This copying happens
   after other relocations, so it's all right to, say, copy a
   Lisp_Object (since by the time we copy the Lisp_Object, it'll have
   been adjusted to account for the location of the running Emacs and
   dump file).  */
fn dump_emacs_reloc_copy_from_dump (ctx: *mut dump_context, dump_offset: dump_off,
                                    emacs_ptr: *mut c_void, size: dump_off)
{
  eassert! (size >= 0);
  eassert! (size < (1 << EMACS_RELOC_LENGTH_BITS));

  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  if size == 0
    { return; }

  eassert! (dump_offset >= 0);
  unsafe {
    dump_push (&raw mut (*ctx).emacs_relocs,
               list4 (make_fixnum (emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as isize),
                      dump_off_to_lisp (emacs_offset (emacs_ptr)),
                      dump_off_to_lisp (dump_offset),
                      dump_off_to_lisp (size)));
  }
}

/* Add an Emacs relocation that sets values to arbitrary bytes.

   When the dump is loaded, Emacs copies SIZE bytes from the
   relocation itself to the adjusted location inside Emacs EMACS_PTR.
   SIZE is the number of bytes to copy.  See struct emacs_reloc for
   the maximum size that this mechanism can support.  The value comes
   from VALUE_PTR.
 */
fn dump_emacs_reloc_immediate (ctx: *mut dump_context, emacs_ptr: *const c_void,
                               value_ptr: *const c_void, size: dump_off)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  let mut value: intmax_t = 0;
  eassert! (size as usize <= core::mem::size_of_val (&value));
  unsafe {
    memcpy ((&raw mut value) as *mut c_void, value_ptr, size as usize);
    dump_push (&raw mut (*ctx).emacs_relocs,
               list4 (make_fixnum (emacs_reloc_type::RELOC_EMACS_IMMEDIATE as isize),
                      dump_off_to_lisp (emacs_offset (emacs_ptr)),
                      intmax_t_to_lisp (value),
                      dump_off_to_lisp (size)));
  }
}

/* C 1577-1585's `DEFINE_EMACS_IMMEDIATE_FN (fnname, type)` emits one `static void
   fnname (struct dump_context *, const type *, type)` per instantiation.  */
macro_rules! DEFINE_EMACS_IMMEDIATE_FN {
    ($fnname:ident, $type:ty) => {
        fn $fnname (ctx: *mut dump_context, emacs_ptr: *const $type, value: $type)
        {
          dump_emacs_reloc_immediate (
            ctx, emacs_ptr as *const c_void, (&raw const value) as *const c_void,
            core::mem::size_of_val (&value) as dump_off);
        }
    };
}

DEFINE_EMACS_IMMEDIATE_FN! (dump_emacs_reloc_immediate_lv, Lisp_Object);
DEFINE_EMACS_IMMEDIATE_FN! (dump_emacs_reloc_immediate_ptrdiff_t, ptrdiff_t);
DEFINE_EMACS_IMMEDIATE_FN! (dump_emacs_reloc_immediate_intmax_t, intmax_t);
DEFINE_EMACS_IMMEDIATE_FN! (dump_emacs_reloc_immediate_int, c_int);
DEFINE_EMACS_IMMEDIATE_FN! (dump_emacs_reloc_immediate_bool, bool);

/* Add an emacs relocation that makes a raw pointer in Emacs point
   into the dump.  */
fn dump_emacs_reloc_to_dump_ptr_raw (ctx: *mut dump_context, emacs_ptr: *const c_void,
                                     dump_offset: dump_off)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  unsafe {
    dump_push (&raw mut (*ctx).emacs_relocs,
               list3 (make_fixnum (emacs_reloc_type::RELOC_EMACS_DUMP_PTR_RAW as isize),
                      dump_off_to_lisp (emacs_offset (emacs_ptr)),
                      dump_off_to_lisp (dump_offset)));
  }
}

/* Add an emacs relocation that points into the dump.

   When the dump is loaded, the Lisp_Object at EMACS_ROOT in Emacs to
   point to VALUE.  VALUE can be any Lisp value; this function
   automatically queues the value for dumping if necessary.  */
fn dump_emacs_reloc_to_lv (ctx: *mut dump_context, emacs_ptr: *const Lisp_Object,
                           value: Lisp_Object)
{
  if dump_object_self_representing_p (value)
    { dump_emacs_reloc_immediate_lv (ctx, emacs_ptr, value); }
  else
    {
      unsafe {
        if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
          /* Conditionally use RELOC_EMACS_EMACS_LV or
             RELOC_EMACS_DUMP_LV depending on where the target object
             lives.  We could just have decode_emacs_reloc pick the
             right type, but we might as well maintain the invariant
             that the types on ctx->emacs_relocs correspond to the types
             of emacs_relocs we actually emit.  */
          { dump_push (&raw mut (*ctx).emacs_relocs,
                       list3 (make_fixnum (
                                if !dump_object_emacs_ptr (value).is_null ()
                                  { emacs_reloc_type::RELOC_EMACS_EMACS_LV as isize }
                                  else { emacs_reloc_type::RELOC_EMACS_DUMP_LV as isize }),
                              dump_off_to_lisp (emacs_offset (emacs_ptr as *const c_void)),
                              value)); }
      }
      dump_enqueue_object (ctx, value, WEIGHT_NONE);
    }
}

/* Add an emacs relocation that makes a raw pointer in Emacs point
   back into the Emacs image.  */
fn dump_emacs_reloc_to_emacs_ptr_raw (ctx: *mut dump_context, emacs_ptr: *mut c_void,
                                      target_emacs_ptr: *const c_void)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  unsafe {
    dump_push (&raw mut (*ctx).emacs_relocs,
               list3 (make_fixnum (emacs_reloc_type::RELOC_EMACS_EMACS_PTR_RAW as isize),
                      dump_off_to_lisp (emacs_offset (emacs_ptr)),
                      dump_off_to_lisp (emacs_offset (target_emacs_ptr))));
  }
}

/* Add an Emacs relocation that makes a raw pointer in Emacs point to
   a different part of Emacs.  */

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_fixup_type
  {
    DUMP_FIXUP_LISP_OBJECT,
    DUMP_FIXUP_LISP_OBJECT_RAW,
    DUMP_FIXUP_PTR_DUMP_RAW,
    DUMP_FIXUP_BIGNUM_DATA,
  }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_lv_fixup_type
  {
    LV_FIXUP_LISP_OBJECT,
    LV_FIXUP_RAW_POINTER,
  }

/* Make something in the dump point to a lisp object.

   CTX is a dump context.  DUMP_OFFSET is the location in the dump to
   fix.  VALUE is the object to which the location in the dump
   should point.

   If FIXUP_SUBTYPE is LV_FIXUP_LISP_OBJECT, we expect a Lisp_Object
   at DUMP_OFFSET.  If it's LV_FIXUP_RAW_POINTER, we expect a pointer.
 */
fn dump_remember_fixup_lv (ctx: *mut dump_context, dump_offset: dump_off, value: Lisp_Object,
                           fixup_subtype: dump_lv_fixup_type)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  unsafe {
    dump_push (&raw mut (*ctx).fixups,
               list3 (make_fixnum (
                        if fixup_subtype == dump_lv_fixup_type::LV_FIXUP_LISP_OBJECT
                          { dump_fixup_type::DUMP_FIXUP_LISP_OBJECT as isize }
                          else { dump_fixup_type::DUMP_FIXUP_LISP_OBJECT_RAW as isize }),
                      dump_off_to_lisp (dump_offset),
                      value));
  }
}

/* Remember to fix up the dump file such that the pointer-sized value
   at DUMP_OFFSET points to NEW_DUMP_OFFSET in the dump file and to
   its absolute address at runtime.  */
fn dump_remember_fixup_ptr_raw (ctx: *mut dump_context, dump_offset: dump_off,
                                new_dump_offset: dump_off)
{
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  /* We should not be generating relocations into the
     to-be-copied-into-Emacs dump region.  */
  eassert! (unsafe { (*ctx).header.discardable_start } == 0
	    || new_dump_offset < unsafe { (*ctx).header.discardable_start }
	    || (unsafe { (*ctx).header.cold_start } != 0
	        && new_dump_offset >= unsafe { (*ctx).header.cold_start }));

  unsafe {
    dump_push (&raw mut (*ctx).fixups,
               list3 (make_fixnum (dump_fixup_type::DUMP_FIXUP_PTR_DUMP_RAW as isize),
                      dump_off_to_lisp (dump_offset),
                      dump_off_to_lisp (new_dump_offset)));
  }
}

unsafe extern "C" fn dump_root_visitor (root_ptr: *const Lisp_Object, type_: gc_root_type,
                                        data: *mut c_void)
{
  unsafe {
    let ctx: *mut dump_context = data as *mut dump_context;
    let value: Lisp_Object = *root_ptr;
    if type_ == GC_ROOT_C_SYMBOL
      {
        eassert! (dump_builtin_symbol_p (value));
        /* Remember to dump the object itself later along with all the
           rest of the copied-to-Emacs objects.  */
        if dump_set_referrer (ctx)
          { (*ctx).current_referrer = build_string (c"built-in symbol list".as_ptr ()); }
        dump_enqueue_object (ctx, value, WEIGHT_NONE);
        dump_clear_referrer (ctx);
      }
    else
      {
        if type_ == GC_ROOT_STATICPRO
          { Fputhash (dump_off_to_lisp (emacs_offset (root_ptr as *const c_void)),
                      Qt (),
                      (*ctx).staticpro_table); }
        if root_ptr != (&raw const Vinternal_interpreter_environment)
          {
            if dump_set_referrer (ctx)
              { (*ctx).current_referrer
                  = dump_ptr_referrer (c"emacs root".as_ptr (), root_ptr as *const c_void); }
            dump_emacs_reloc_to_lv (ctx, root_ptr, *root_ptr);
            dump_clear_referrer (ctx);
          }
      }
  }
}

/* Kick off the dump process by queuing up the static GC roots.  */
fn dump_roots (ctx: *mut dump_context)
{
  let visitor: gc_root_visitor =
    gc_root_visitor { visit: Some (dump_root_visitor), data: ctx as *mut c_void };
  unsafe { visit_static_gc_roots (visitor); }
}

pub const PDUMPER_MAX_OBJECT_SIZE: c_int = 2048;

fn field_relpos (in_start: *const c_void, in_field: *const c_void) -> dump_off
{
  let in_start_val: ptrdiff_t = in_start as ptrdiff_t;
  let in_field_val: ptrdiff_t = in_field as ptrdiff_t;
  eassert! (in_start_val <= in_field_val);
  let relpos: ptrdiff_t = in_field_val - in_start_val;
  /* The following assertion attempts to detect bugs whereby IN_START
     and IN_FIELD don't point to the same object/structure, on the
     assumption that a too-large difference between them is
     suspicious.  As of Apr 2019 the largest object we dump -- 'struct
     buffer' -- is slightly smaller than 1KB, and we want to leave
     some margin for future extensions.  If the assertion below is
     ever violated, make sure the two pointers indeed point into the
     same object, and if so, enlarge the value of PDUMPER_MAX_OBJECT_SIZE.  */
  eassert! (relpos < PDUMPER_MAX_OBJECT_SIZE as ptrdiff_t);
  return relpos as dump_off;
}

fn cpyptr (out: *mut c_void, in_: *const c_void)
{
  unsafe { memcpy (out, in_, core::mem::size_of::<*mut c_void> ()); }
}

/* Convenience macro for regular assignment.  */
macro_rules! DUMP_FIELD_COPY {
    ($out:expr, $in:expr, $($name:tt)+) => {
        (*$out).$($name)+ = (*$in).$($name)+
    };
}

fn dump_field_lv_or_rawptr (ctx: *mut dump_context, out: *mut c_void,
                            in_start: *const c_void, in_field: *const c_void,
                            /* opt */ ptr_raw_type: *const Lisp_Type,
                            weight: link_weight)
{
  eassert! (unsafe { (*ctx).obj_offset } > 0);

  let mut value: Lisp_Object = Qnil ();
  let relpos: dump_off = field_relpos (in_start, in_field);
  let out_field: *mut c_void = unsafe { (out as *mut c_char).add (relpos as usize) as *mut c_void };
  let is_ptr_raw: bool = !ptr_raw_type.is_null ();

  if !is_ptr_raw
    {
      unsafe { memcpy ((&raw mut value) as *mut c_void, in_field,
                       core::mem::size_of_val (&value)); }
      if dump_object_self_representing_p (value)
        {
          unsafe { memcpy (out_field, (&raw const value) as *const c_void,
                           core::mem::size_of_val (&value)); }
          return;
        }
    }
  else
    {
      let mut ptrval: *mut c_void = core::ptr::null_mut ();
      cpyptr ((&raw mut ptrval) as *mut c_void, in_field);
      if ptrval.is_null ()
        { return; } /* Nothing to do.  */
      match unsafe { *ptr_raw_type }
        {
          Lisp_Type::Lisp_Symbol
            => { value = make_lisp_symbol (ptrval as *mut Lisp_Symbol); }
          Lisp_Type::Lisp_String
          | Lisp_Type::Lisp_Vectorlike
          | Lisp_Type::Lisp_Cons
          | Lisp_Type::Lisp_Float
            => { value = make_lisp_ptr (ptrval, unsafe { *ptr_raw_type }); }
          _ => { unsafe { emacs_abort (); } }
        }
    }

  /* Now value is the Lisp_Object to which we want to point whether or
     not the field is a raw pointer (in which case we just synthesized
     the Lisp_Object ourselves) or a Lisp_Object (in which case we
     just copied the thing).  Add a fixup or relocation.  */

  let out_value: intptr_t;
  let out_field_offset: dump_off = unsafe { (*ctx).obj_offset } + relpos;
  let target_offset: dump_off = dump_recall_object (ctx, value);
  const DANGEROUS: bool = false;
  if DANGEROUS
      && target_offset > 0 && dump_object_emacs_ptr (value).is_null ()
    {
      /* We've already dumped the referenced object, so we can emit
         the value and a relocation directly instead of indirecting
         through a fixup.  */
      out_value = target_offset as intptr_t;
      if is_ptr_raw
        { dump_reloc_dump_to_dump_ptr_raw (ctx, out_field_offset); }
      else
        { dump_reloc_dump_to_dump_lv (ctx, out_field_offset, XTYPE (value)); }
    }
  else
    {
      /* We don't know about the target object yet, so add a fixup.
         When we process the fixup, we'll have dumped the target
         object.  */
      out_value = 0xDEADF00D;
      dump_remember_fixup_lv (ctx,
                              out_field_offset,
                              value,
                              ( if is_ptr_raw
                                { dump_lv_fixup_type::LV_FIXUP_RAW_POINTER }
                                else { dump_lv_fixup_type::LV_FIXUP_LISP_OBJECT }));
      dump_enqueue_object (ctx, value, weight);
    }

  unsafe { memcpy (out_field, (&raw const out_value) as *const c_void,
                   core::mem::size_of_val (&out_value)); }
}

/* Set a pointer field on an output object during dump.

   CTX is the dump context.  OUT is a pointer to the dump output
   object.  IN_START is the start of the current Emacs object.
   IN_FIELD is a pointer to the field in that object.  TYPE is the
   type of pointer to which IN_FIELD points.
 */
fn dump_field_lv_rawptr (ctx: *mut dump_context, out: *mut c_void, in_start: *const c_void,
                         in_field: *const c_void, type_: Lisp_Type, weight: link_weight)
{
  dump_field_lv_or_rawptr (ctx, out, in_start, in_field, &type_, weight);
}

/* Set a Lisp_Object field on an output object during dump.

   CTX is a dump context.  OUT is a pointer to the dump output object.
   IN_START is the start of the current Emacs object.  IN_FIELD is a
   pointer to a Lisp_Object field in that object.

   Arrange for the dump to contain fixups and relocations such that,
   at load time, the given field of the output object contains a valid
   Lisp_Object pointing to the same notional object that *IN_FIELD
   contains now.

   See idomatic usage below.  */
fn dump_field_lv (ctx: *mut dump_context, out: *mut c_void, in_start: *const c_void,
                  in_field: *const Lisp_Object, weight: link_weight)
{
  dump_field_lv_or_rawptr (ctx, out, in_start, in_field as *const c_void,
                           core::ptr::null (), weight);
}

/* Note that we're going to add a manual fixup for the given field
   later.  */
fn dump_field_fixup_later (ctx: *mut dump_context, out: *mut c_void, in_start: *const c_void,
                           in_field: *const c_void)
{
  /* TODO: more error checking.  */
  let _ = ctx;
  let _ = out;
  field_relpos (in_start, in_field);
}

/* Mark an output object field, which is as wide as a pointer, as being
   fixed up to point to a specific offset in the dump.  */
fn dump_field_ptr_to_dump_offset (ctx: *mut dump_context, out: *mut c_void,
                                  in_start: *const c_void, in_field: *const c_void,
                                  target_dump_offset: dump_off)
{
  eassert! (unsafe { (*ctx).obj_offset } > 0);
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  let relpos: dump_off = field_relpos (in_start, in_field);
  dump_reloc_dump_to_dump_ptr_raw (ctx, unsafe { (*ctx).obj_offset } + relpos);
  let outval: intptr_t = target_dump_offset as intptr_t;
  unsafe {
    memcpy ((out as *mut c_char).add (relpos as usize) as *mut c_void,
            (&raw const outval) as *const c_void, core::mem::size_of_val (&outval));
  }
}

/* Mark a field as pointing to a place inside Emacs.

   CTX is the dump context.  OUT points to the out-object for the
   current dump function.  IN_START points to the start of the object
   being dumped.  IN_FIELD points to the field inside the object being
   dumped that we're dumping.  The contents of this field (which
   should be as wide as a pointer) are the Emacs pointer to dump.

 */
fn dump_field_emacs_ptr (ctx: *mut dump_context, out: *mut c_void, in_start: *const c_void,
                         in_field: *const c_void)
{
  eassert! (unsafe { (*ctx).obj_offset } > 0);
  if unsafe { ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) == 0 }
    { return; }

  let relpos: dump_off = field_relpos (in_start, in_field);
  let mut abs_emacs_ptr: *mut c_void = core::ptr::null_mut ();
  cpyptr ((&raw mut abs_emacs_ptr) as *mut c_void, in_field);
  let mut rel_emacs_ptr: intptr_t = 0;
  if !abs_emacs_ptr.is_null ()
    {
      rel_emacs_ptr = emacs_offset (abs_emacs_ptr) as intptr_t;
      dump_reloc_dump_to_emacs_ptr_raw (ctx, unsafe { (*ctx).obj_offset } + relpos);
    }
  cpyptr (unsafe { (out as *mut c_char).add (relpos as usize) as *mut c_void },
          (&raw const rel_emacs_ptr) as *const c_void);
}

fn dump_object_start_pseudovector (ctx: *mut dump_context,
                                   out_hdr: *mut vectorlike_header,
                                   in_hdr: *const vectorlike_header)
{
  eassert! (unsafe { (*in_hdr).size } & PSEUDOVECTOR_FLAG != 0);
  let vec_size: ptrdiff_t = unsafe { vectorlike_nbytes (in_hdr) };
  dump_object_start (ctx, out_hdr as *mut c_void, vec_size as dump_off);
  unsafe { *out_hdr = *in_hdr; }
}

/* Need a macro for alloca.  C 1992-1995's `alloca (vectorlike_nbytes (in_hdr))` is the
   port's stack pool (`sa_stack_alloc`, lisp.rs's AVAIL_ALLOCA note and PORTING.md P-05:
   the port has no alloca; the memory an object dump needs live for the call, like C's
   frame-local alloca, and the pool's release rule is alloc.rs's to define — recorded in
   docs/ports/pdumper.md).  */
macro_rules! START_DUMP_PVEC {
    ($ctx:expr, $hdr:expr, $type:ty, $out:ident) => {
        let in_hdr: *const vectorlike_header = unsafe { $hdr };
        let $out: *mut $type =
          unsafe { sa_stack_alloc (vectorlike_nbytes (in_hdr) as usize) } as *mut $type;
        unsafe { dump_object_start_pseudovector ($ctx, &raw mut (*$out).header, in_hdr); }
    };
}

fn finish_dump_pvec (ctx: *mut dump_context, out_hdr: *mut vectorlike_header) -> dump_off
{
  return dump_object_finish (ctx, out_hdr as *const c_void,
                             unsafe { vectorlike_nbytes (out_hdr) } as dump_off);
}

fn dump_pseudovector_lisp_fields (ctx: *mut dump_context, out_hdr: *mut vectorlike_header,
                                  in_hdr: *const vectorlike_header)
{
  let in_: *const Lisp_Vector = in_hdr as *const Lisp_Vector;
  let out: *mut Lisp_Vector = out_hdr as *mut Lisp_Vector;
  let mut size: ptrdiff_t = unsafe { (*in_).header.size as ptrdiff_t };
  eassert! (size & PSEUDOVECTOR_FLAG as ptrdiff_t != 0);
  size &= PSEUDOVECTOR_SIZE_MASK as ptrdiff_t;
  for i in 0..size
    {
      dump_field_lv (ctx, out as *mut c_void, in_ as *const c_void,
                     unsafe { &raw const FAM_ELEM_RO! ((*in_).contents, i) }, WEIGHT_STRONG);
    }
}

fn dump_cons (ctx: *mut dump_context, cons: *const Lisp_Cons) -> dump_off
{
  /* C 2013-2015's `#if CHECK_STRUCTS && !defined (HASH_Lisp_Cons_00EEE63F67)` is dead
     (`CHECK_STRUCTS` is undef in the frozen configuration); text is carried in the ledger
     (P-51), see docs/ports/pdumper.md.  Every dump function below has the same guard.  */
  let mut out: Lisp_Cons = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, cons as *const c_void,
                   &raw const (*cons).u.s.car, WEIGHT_STRONG);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, cons as *const c_void,
                   &raw const (*cons).u.s.u.cdr, WEIGHT_NORMAL);
  }
  return dump_object_finish (ctx, (&raw const out) as *const c_void,
                             core::mem::size_of_val (&out) as dump_off);
}

fn dump_interval_tree (ctx: *mut dump_context, tree: INTERVAL, parent_offset: dump_off)
  -> dump_off
{
  let tree: *mut interval = tree as *mut interval;
  /* TODO: output tree breadth-first?  */
  let mut out: interval = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    DUMP_FIELD_COPY! (&raw mut out, tree, total_length);
    DUMP_FIELD_COPY! (&raw mut out, tree, position);
    if !(*tree).left.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, tree as *const c_void,
                                (&raw const (*tree).left) as *const c_void); }
    if !(*tree).right.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, tree as *const c_void,
                                (&raw const (*tree).right) as *const c_void); }
    if ((*tree).bits & INTERVAL_UP_OBJ_BIT as u8) == 0
      {
        eassert! (parent_offset != 0);
        dump_field_ptr_to_dump_offset (ctx, (&raw mut out) as *mut c_void,
                                       tree as *const c_void,
                                       (&raw const (*tree).up.interval) as *const c_void,
				       parent_offset);
      }
    else
      { dump_field_lv (ctx, (&raw mut out) as *mut c_void, tree as *const c_void,
                       &raw const (*tree).up.obj, WEIGHT_STRONG); }
    /* C 2053-2058's five `DUMP_FIELD_COPY` calls on the `bool_bf` members
       (`up_obj`, `gcmarkbit`, `write_protect`, `visible`, `front_sticky`,
       `rear_sticky`) copy one bit each; the port copies the same bits through the
       mirror's masks (PORTING.md 1.3, docs/ports/token-aliases.tsv families).  */
    out.bits = (out.bits & !INTERVAL_UP_OBJ_BIT as u8)
      | ((*tree).bits & INTERVAL_UP_OBJ_BIT as u8);
    eassert! (((*tree).bits & INTERVAL_GCMARKBIT_BIT as u8) == 0);
    out.bits = (out.bits & !INTERVAL_GCMARKBIT_BIT as u8)
      | ((*tree).bits & INTERVAL_GCMARKBIT_BIT as u8);
    out.bits = (out.bits & !INTERVAL_WRITE_PROTECT_BIT as u8)
      | ((*tree).bits & INTERVAL_WRITE_PROTECT_BIT as u8);
    out.bits = (out.bits & !INTERVAL_VISIBLE_BIT as u8)
      | ((*tree).bits & INTERVAL_VISIBLE_BIT as u8);
    out.bits = (out.bits & !INTERVAL_FRONT_STICKY_BIT as u8)
      | ((*tree).bits & INTERVAL_FRONT_STICKY_BIT as u8);
    out.bits = (out.bits & !INTERVAL_REAR_STICKY_BIT as u8)
      | ((*tree).bits & INTERVAL_REAR_STICKY_BIT as u8);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, tree as *const c_void,
                   &raw const (*tree).plist, WEIGHT_STRONG);
    let offset: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                               core::mem::size_of_val (&out) as dump_off);
    if !(*tree).left.is_null ()
      { dump_remember_fixup_ptr_raw
	  (ctx,
	   offset + dump_offsetof! (interval, left),
	   dump_interval_tree (ctx, (*tree).left as INTERVAL, offset)); }
    if !(*tree).right.is_null ()
      { dump_remember_fixup_ptr_raw
	  (ctx,
	   offset + dump_offsetof! (interval, right),
	   dump_interval_tree (ctx, (*tree).right as INTERVAL, offset)); }
    return offset;
  }
}

fn dump_string (ctx: *mut dump_context, string: *const Lisp_String) -> dump_off
{
  /* If we have text properties, write them _after_ the string so that
     at runtime, the prefetcher and cache will DTRT. (We access the
     string before its properties.).

     There's special code to dump string data contiguously later on.
     we seldom write to string data and never relocate it, so lumping
     it together at the end of the dump saves on COW faults.

     If, however, the string's size_byte field is -2, the string data
     is actually a pointer to Emacs data segment, so we can do even
     better by emitting a relocation instead of bothering to copy the
     string data.  */
  let mut out: Lisp_String = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    DUMP_FIELD_COPY! (&raw mut out, string, u.s.size);
    DUMP_FIELD_COPY! (&raw mut out, string, u.s.size_byte);
    if !(*string).u.s.intervals.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, string as *const c_void,
                                (&raw const (*string).u.s.intervals) as *const c_void); }

    if (*string).u.s.size_byte == -2
      /* String literal in Emacs rodata.  */
      { dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, string as *const c_void,
                              (&raw const (*string).u.s.data) as *const c_void); }
    else
      {
        dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, string as *const c_void,
                                (&raw const (*string).u.s.data) as *const c_void);
        dump_remember_cold_op (ctx,
                               cold_op::COLD_OP_STRING,
			       make_lisp_ptr (string as *mut c_void, Lisp_Type::Lisp_String));
      }

    let offset: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                               core::mem::size_of_val (&out) as dump_off);
    if !(*string).u.s.intervals.is_null ()
      { dump_remember_fixup_ptr_raw
        (ctx,
         offset + dump_offsetof! (Lisp_String, u.s.intervals),
         dump_interval_tree (ctx, (*string).u.s.intervals as INTERVAL, 0)); }

    return offset;
  }
}

fn dump_marker (ctx: *mut dump_context, marker: *const Lisp_Marker) -> dump_off
{
  START_DUMP_PVEC! (ctx, (&raw const (*marker).header), Lisp_Marker, out);
  dump_pseudovector_lisp_fields (ctx, unsafe { &raw mut (*out).header },
                                 unsafe { &raw const (*marker).header });
  unsafe {
    /* C 2129-2130's two `bool_bf` copies; the port copies the same two bits through
       `bits` (lisp.rs's MARKER_NEED_ADJUSTMENT/MARKER_INSERTION_TYPE).  */
    (*out).bits = ((*out).bits & !MARKER_NEED_ADJUSTMENT)
      | ((*marker).bits & MARKER_NEED_ADJUSTMENT);
    (*out).bits = ((*out).bits & !MARKER_INSERTION_TYPE)
      | ((*marker).bits & MARKER_INSERTION_TYPE);
    if !(*marker).buffer.is_null ()
      {
        dump_field_lv_rawptr (ctx, out as *mut c_void, marker as *const c_void,
			      (&raw const (*marker).buffer) as *const c_void,
			      Lisp_Type::Lisp_Vectorlike, WEIGHT_NORMAL);
        dump_field_lv_rawptr (ctx, out as *mut c_void, marker as *const c_void,
			      (&raw const (*marker).next) as *const c_void,
			      Lisp_Type::Lisp_Vectorlike, WEIGHT_STRONG);
        DUMP_FIELD_COPY! (out, marker, charpos);
        DUMP_FIELD_COPY! (out, marker, bytepos);
      }
    return finish_dump_pvec (ctx, &raw mut (*out).header);
  }
}

fn dump_interval_node (ctx: *mut dump_context, node: *mut itree_node) -> dump_off
{
  let mut out: itree_node = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    if !(*node).parent.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, node as *const c_void,
                                (&raw const (*node).parent) as *const c_void); }
    if !(*node).left.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, node as *const c_void,
                                (&raw const (*node).left) as *const c_void); }
    if !(*node).right.is_null ()
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, node as *const c_void,
                                (&raw const (*node).right) as *const c_void); }
    DUMP_FIELD_COPY! (&raw mut out, node, begin);
    DUMP_FIELD_COPY! (&raw mut out, node, end);
    DUMP_FIELD_COPY! (&raw mut out, node, limit);
    DUMP_FIELD_COPY! (&raw mut out, node, offset);
    DUMP_FIELD_COPY! (&raw mut out, node, otick);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, node as *const c_void,
                   &raw const (*node).data, WEIGHT_STRONG);
    /* C 2168-2170's three `bool_bf` copies (itree_node's `bits` byte).  */
    out.bits = (out.bits & !ITREE_RED) | ((*node).bits & ITREE_RED);
    out.bits = (out.bits & !ITREE_REAR_ADVANCE) | ((*node).bits & ITREE_REAR_ADVANCE);
    out.bits = (out.bits & !ITREE_FRONT_ADVANCE) | ((*node).bits & ITREE_FRONT_ADVANCE);
    let offset: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                               core::mem::size_of_val (&out) as dump_off);
    if !(*node).parent.is_null ()
      { dump_remember_fixup_ptr_raw
	  (ctx,
	   offset + dump_offsetof! (itree_node, parent),
	   dump_interval_node (ctx, (*node).parent)); }
    if !(*node).left.is_null ()
      { dump_remember_fixup_ptr_raw
	  (ctx,
	   offset + dump_offsetof! (itree_node, left),
	   dump_interval_node (ctx, (*node).left)); }
    if !(*node).right.is_null ()
      { dump_remember_fixup_ptr_raw
	  (ctx,
	   offset + dump_offsetof! (itree_node, right),
	   dump_interval_node (ctx, (*node).right)); }
    return offset;
  }
}

fn dump_overlay (ctx: *mut dump_context, overlay: *const Lisp_Overlay) -> dump_off
{
  START_DUMP_PVEC! (ctx, (&raw const (*overlay).header), Lisp_Overlay, out);
  dump_pseudovector_lisp_fields (ctx, unsafe { &raw mut (*out).header },
                                 unsafe { &raw const (*overlay).header });
  unsafe {
    dump_field_fixup_later (ctx, out as *mut c_void, overlay as *const c_void,
                            (&raw const (*overlay).interval) as *const c_void);
    let offset: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    dump_remember_fixup_ptr_raw
      (ctx,
       offset + dump_offsetof! (Lisp_Overlay, interval),
       dump_interval_node (ctx, (*overlay).interval as *mut itree_node));
    return offset;
  }
}

fn dump_field_finalizer_ref (ctx: *mut dump_context, out: *mut c_void,
                             finalizer: *const Lisp_Finalizer,
                             field: *const *mut Lisp_Finalizer)
{
  unsafe {
    if *field == (&raw mut finalizers) || *field == (&raw mut doomed_finalizers)
      { dump_field_emacs_ptr (ctx, out, finalizer as *const c_void, field as *const c_void); }
    else
      { dump_field_lv_rawptr (ctx, out, finalizer as *const c_void,
                              field as *const c_void,
                              Lisp_Type::Lisp_Vectorlike,
                              WEIGHT_NORMAL); }
  }
}

fn dump_finalizer (ctx: *mut dump_context, finalizer: *const Lisp_Finalizer) -> dump_off
{
  START_DUMP_PVEC! (ctx, (&raw const (*finalizer).header), Lisp_Finalizer, out);
  /* Do _not_ call dump_pseudovector_lisp_fields here: we dump the
     only Lisp field, finalizer->function, manually, so we can give it
     a low weight.  */
  unsafe {
    dump_field_lv (ctx, out as *mut c_void, finalizer as *const c_void,
                   &raw const (*finalizer).function, WEIGHT_NONE);
    dump_field_finalizer_ref (ctx, out as *mut c_void, finalizer,
                              &raw const (*finalizer).prev);
    dump_field_finalizer_ref (ctx, out as *mut c_void, finalizer,
                              &raw const (*finalizer).next);
    return finish_dump_pvec (ctx, &raw mut (*out).header);
  }
}

/* C 2238-2248's `#ifdef HAVE_TREE_SITTER` arm is dead (`HAVE_TREE_SITTER` is undef in the
   frozen configuration); carried as text (P-51, docs/status.md 3.49).  That record lives in
   treesit.h's dead arm (its mirror carries only the one live declaration outside the guard),
   so `struct Lisp_TS_Query` and its two members are named here for the dead arm.  */
#[cfg (feature = "have_tree_sitter")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_TS_Query
{
  pub header: vectorlike_header,
  pub language: Lisp_Object,
  pub source: Lisp_Object,
  pub query: *mut c_void,
  pub cursor: *mut c_void,
}
#[cfg (feature = "have_tree_sitter")]
#[inline(always)]
pub fn XTS_COMPILED_QUERY (a: Lisp_Object) -> *mut Lisp_TS_Query
{
  eassert! (PSEUDOVECTORP (a, pvec_type::PVEC_TS_COMPILED_QUERY));
  return XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, Lisp_TS_Query);
}

#[cfg (feature = "have_tree_sitter")]
fn dump_treesit_compiled_query (ctx: *mut dump_context, query: *mut Lisp_TS_Query) -> dump_off
{
  START_DUMP_PVEC! (ctx, (&raw const (*query).header), Lisp_TS_Query, out);
  unsafe {
    dump_field_lv (ctx, out as *mut c_void, query as *const c_void, &raw const (*query).language,
                   WEIGHT_STRONG);
    dump_field_lv (ctx, out as *mut c_void, query as *const c_void, &raw const (*query).source,
                   WEIGHT_STRONG);
    /* These will be recompiled after load from dump.  */
    (*out).query = core::ptr::null_mut ();
    (*out).cursor = core::ptr::null_mut ();
    return finish_dump_pvec (ctx, &raw mut (*out).header);
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct bignum_reload_info
{
  data_location: dump_off,
  nlimbs: dump_off,
}

fn dump_bignum (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  let bignum: *const Lisp_Bignum = unsafe { XBIGNUM (object) };
  START_DUMP_PVEC! (ctx, (&raw const (*bignum).header), Lisp_Bignum, out);
  /* C 2264: `static_assert (sizeof (out->value) >= sizeof (struct bignum_reload_info));`
     — a type-level assertion in the port, because `static_assert!` is a `const` item and
     cannot name the local `out` (the tokens it contributes are in the body anyway).  */
  static_assert! (core::mem::size_of::<[__mpz_struct; 1]> ()
                  >= core::mem::size_of::<bignum_reload_info> ());
  unsafe {
    dump_field_fixup_later (ctx, out as *mut c_void, bignum as *const c_void,
                            xbignum_val (object) as *const c_void);
    let bignum_offset: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      {
        /* Export the bignum into a blob in the cold section.  */
        dump_remember_cold_op (ctx, cold_op::COLD_OP_BIGNUM, object);

        /* Write the offset of that exported blob here.  */
        let value_offset: dump_off
	  = (bignum_offset
	     + dump_offsetof! (Lisp_Bignum, value) as dump_off);
        dump_push (&raw mut (*ctx).fixups,
		   list3 (make_fixnum (dump_fixup_type::DUMP_FIXUP_BIGNUM_DATA as isize),
			  dump_off_to_lisp (value_offset),
			  object));

        /* When we load the dump, slurp the data blob and turn it into a
           real bignum.  Attach the relocation to the start of the
           Lisp_Bignum instead of the actual mpz field so that the
           relocation offset is aligned.  The relocation-application
           code knows to actually advance past the header.  */
        dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::EARLY_RELOCS as usize],
                   list2 (make_fixnum (dump_reloc_type::RELOC_BIGNUM as isize),
                          dump_off_to_lisp (bignum_offset)));
      }

    return bignum_offset;
  }
}

fn dump_float (ctx: *mut dump_context, lfloat: *const Lisp_Float) -> dump_off
{
  eassert! (unsafe { (*ctx).header.cold_start } != 0);
  let mut out: Lisp_Float = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe { DUMP_FIELD_COPY! (&raw mut out, lfloat, u.data); }
  return dump_object_finish (ctx, (&raw const out) as *const c_void,
                             core::mem::size_of_val (&out) as dump_off);
}

fn dump_field_fwd (ctx: *mut dump_context, out: *mut c_void, in_start: *const c_void,
	           in_field: *const lispfwd)
{
  dump_field_emacs_ptr (ctx, out, in_start, in_field as *const c_void);
  match unsafe { XFWDTYPE (*in_field) }
    {
      Lisp_Fwd_Int
      => {
	  let intvar: *const intmax_t = unsafe { (**in_field).u.intvar };
	  unsafe { dump_emacs_reloc_immediate_intmax_t (ctx, intvar, *intvar); }
	  return;
	}
      Lisp_Fwd_Bool
      => {
	  let boolvar: *const bool = unsafe { (**in_field).u.boolvar };
	  unsafe { dump_emacs_reloc_immediate_bool (ctx, boolvar, *boolvar); }
	  return;
	}
      Lisp_Fwd_Obj
      => {
	  let objvar: *const Lisp_Object = unsafe { (**in_field).u.objvar };
	  if NILP (unsafe { Fgethash (dump_off_to_lisp (emacs_offset (objvar as *const c_void)),
				      (*ctx).staticpro_table, Qnil ()) })
	    { dump_emacs_reloc_to_lv (ctx, objvar, unsafe { *objvar }); }
	  return;
	}
      Lisp_Fwd_Kboard_Obj
      | Lisp_Fwd_Buffer_Obj
      => { return; }
      _ => {}
    }
  unsafe { emacs_abort (); }
}

fn dump_blv (ctx: *mut dump_context, blv: *const Lisp_Buffer_Local_Value) -> dump_off
{
  let mut out: Lisp_Buffer_Local_Value = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    /* C 2326-2327's two `bool_bf` copies (`local_if_set`, `found`).  */
    out.bits = (out.bits & !BLV_LOCAL_IF_SET) | ((*blv).bits & BLV_LOCAL_IF_SET);
    out.bits = (out.bits & !BLV_FOUND) | ((*blv).bits & BLV_FOUND);
    if !(*blv).fwd.is_null ()
      {
        eassert! (XFWDTYPE ((*blv).fwd) != Lisp_Fwd_Buffer_Obj);
        dump_field_fwd (ctx, (&raw mut out) as *mut c_void, blv as *const c_void,
                        &raw const (*blv).fwd);
      }
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, blv as *const c_void,
                   &raw const (*blv).r#where, WEIGHT_NORMAL);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, blv as *const c_void,
                   &raw const (*blv).defcell, WEIGHT_STRONG);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, blv as *const c_void,
                   &raw const (*blv).valcell, WEIGHT_STRONG);
    let offset: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                               core::mem::size_of_val (&out) as dump_off);
    return offset;
  }
}

fn dump_recall_symbol_aux (ctx: *mut dump_context, symbol: Lisp_Object) -> dump_off
{
  let symbol_aux: Lisp_Object = unsafe { (*ctx).symbol_aux };
  if NILP (symbol_aux)
    { return 0; }
  return dump_off_from_lisp (unsafe { Fgethash (symbol, symbol_aux, make_fixnum (0)) });
}

fn dump_remember_symbol_aux (ctx: *mut dump_context, symbol: Lisp_Object, offset: dump_off)
{
  unsafe { Fputhash (symbol, dump_off_to_lisp (offset), (*ctx).symbol_aux); }
}

/* Dump auxiliary information attached to SYMBOL, a symbol that will be
   copied into Emacs's core from the dump file.  If SYMBOL is localized,
   generate a copy of its buffer local storage and arrange that the
   symbol redirect to the same.  Otherwise, if SYMBOL is forwarded,
   arrange to restore the contents of the forwarding structure and/or
   dump its references as the case may be; the former is only necessary
   in the case of buffer objfwds, which are initialized at runtime.  */

fn dump_pre_dump_symbol (ctx: *mut dump_context, symbol: *mut Lisp_Symbol)
{
  let symbol_lv: Lisp_Object = make_lisp_symbol (symbol);
  eassert! (dump_recall_symbol_aux (ctx, symbol_lv) == 0);
  if dump_set_referrer (ctx)
    { unsafe { (*ctx).current_referrer = symbol_lv; } }
  match unsafe { ((*symbol).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT }
    {
      SYMBOL_LOCALIZED
        => { dump_remember_symbol_aux (ctx, symbol_lv,
				       dump_blv (ctx, unsafe { (*symbol).u.s.val.blv }
                                                 as *const Lisp_Buffer_Local_Value)); }
      _ => {}
    }
  dump_clear_referrer (ctx);
}

fn dump_symbol (ctx: *mut dump_context, object: Lisp_Object, offset: dump_off) -> dump_off
{
  let mut offset = offset;
  let aux_offset: dump_off;

  if unsafe { (*ctx).flags.bits & DUMP_FLAGS_DEFER_SYMBOLS } != 0
    {
      if offset != dump_object_special_offset::DUMP_OBJECT_ON_SYMBOL_QUEUE as dump_off
        {
	  eassert! (offset == dump_object_special_offset::DUMP_OBJECT_ON_NORMAL_QUEUE as dump_off
		    || offset == dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off);
	  dump_clear_referrer (ctx);
          let old_flags: dump_flags = unsafe { (*ctx).flags };
          unsafe {
            set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DUMP_OBJECT_CONTENTS, false);
            set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_SYMBOLS, false);
          }
          dump_object (ctx, object);
          unsafe { (*ctx).flags = old_flags; }
	  if dump_set_referrer (ctx)
	    { unsafe { (*ctx).current_referrer = object; } }

          offset = dump_object_special_offset::DUMP_OBJECT_ON_SYMBOL_QUEUE as dump_off;
          dump_remember_object (ctx, object, offset);
          unsafe { dump_push (&raw mut (*ctx).deferred_symbols, object); }
        }
      return offset;
    }

  let symbol: *mut Lisp_Symbol = unsafe { XSYMBOL (object) };
  let mut out: Lisp_Symbol = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe {
    eassert! (((*symbol).u.s.bits & SYMBOL_GC_MARK_BIT) == 0);
    /* C 2417-2420's four bitfield copies (`redirect`, `trapped_write`, `interned`,
       `declared_special`), one mask each.  */
    out.u.s.bits = (out.u.s.bits & !SYMBOL_REDIRECT_MASK)
      | ((*symbol).u.s.bits & SYMBOL_REDIRECT_MASK);
    out.u.s.bits = (out.u.s.bits & !SYMBOL_TRAPPED_WRITE_MASK)
      | ((*symbol).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK);
    out.u.s.bits = (out.u.s.bits & !SYMBOL_INTERNED_MASK)
      | ((*symbol).u.s.bits & SYMBOL_INTERNED_MASK);
    out.u.s.bits = (out.u.s.bits & !SYMBOL_DECLARED_SPECIAL_BIT)
      | ((*symbol).u.s.bits & SYMBOL_DECLARED_SPECIAL_BIT);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                   &raw const (*symbol).u.s.name, WEIGHT_STRONG);
    match ((*symbol).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
      {
        SYMBOL_PLAINVAL
          => {
	    dump_field_lv (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                           &raw const (*symbol).u.s.val.value, WEIGHT_NORMAL);
          }
        SYMBOL_VARALIAS
          => {
	    dump_field_lv_rawptr (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                                  (&raw const (*symbol).u.s.val.alias) as *const c_void,
                                  Lisp_Type::Lisp_Symbol, WEIGHT_NORMAL);
          }
        SYMBOL_LOCALIZED
          => {
	    dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void,
                                    symbol as *const c_void,
                                    (&raw const (*symbol).u.s.val.blv) as *const c_void);
          }
        SYMBOL_FORWARDED
          => {
	    dump_field_fwd (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                            (&raw const (*symbol).u.s.val.fwd) as *const lispfwd);
          }
        _ => { emacs_abort (); }
      }
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                   &raw const (*symbol).u.s.function, WEIGHT_NORMAL);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                   &raw const (*symbol).u.s.plist, WEIGHT_NORMAL);
    dump_field_lv_rawptr (ctx, (&raw mut out) as *mut c_void, symbol as *const c_void,
                          (&raw const (*symbol).u.s.next) as *const c_void,
                          Lisp_Type::Lisp_Symbol, WEIGHT_STRONG);

    offset = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                 core::mem::size_of_val (&out) as dump_off);
    match ((*symbol).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
      {
        SYMBOL_LOCALIZED
          => {
	    aux_offset = dump_recall_symbol_aux (ctx, make_lisp_symbol (symbol));
	    dump_remember_fixup_ptr_raw (ctx,
                                         offset + dump_offsetof! (Lisp_Symbol, u.s.val.blv) as dump_off,
				   (if aux_offset != 0
				    { aux_offset }
				    else { dump_blv (ctx, (*symbol).u.s.val.blv
                                                     as *const Lisp_Buffer_Local_Value) }));
          }
        _ => {}
      }
    return offset;
  }
}

fn dump_vectorlike_generic (ctx: *mut dump_context, header: *const vectorlike_header)
  -> dump_off
{
  let v: *const Lisp_Vector = header as *const Lisp_Vector;
  let mut size: ptrdiff_t = unsafe { (*header).size as ptrdiff_t };
  let pvectype: pvec_type = unsafe { PSEUDOVECTOR_TYPE (v as *mut Lisp_Vector) };
  let offset: dump_off;

  if size & PSEUDOVECTOR_FLAG as ptrdiff_t != 0
    {
      /* Assert that the pseudovector contains only Lisp values ---
         but see the PVEC_SUB_CHAR_TABLE special case below.  We allow
         one extra word of non-lisp data when Lisp_Object is shorter
         than GCALIGN (e.g., on 32-bit builds) to account for
         GCALIGN-enforcing struct padding.  We can't distinguish
         between padding and some undumpable data member this way, but
         we'll count on sizeof(Lisp_Object) >= GCALIGN builds to catch
         this class of problem.
         */
      eassert! ((((size as usize) & PSEUDOVECTOR_REST_MASK as usize)
                 >> PSEUDOVECTOR_REST_BITS as usize)
	        <= (core::mem::size_of::<Lisp_Object> () < GCALIGNMENT) as usize);
      size &= PSEUDOVECTOR_SIZE_MASK as ptrdiff_t;
    }

  dump_align_output (ctx, DUMP_ALIGNMENT);
  let prefix_start_offset: dump_off = unsafe { (*ctx).offset };

  let skip: dump_off;
  if pvectype == pvec_type::PVEC_SUB_CHAR_TABLE
    {
      /* PVEC_SUB_CHAR_TABLE has a special case because it's a
         variable-length vector (unlike other pseudovectors, which is
         why we handle it here) and has its non-Lisp data _before_ the
         variable-length Lisp part.  */
      let sct: *const Lisp_Sub_Char_Table = header as *const Lisp_Sub_Char_Table;
      let mut out: Lisp_Sub_Char_Table = unsafe { core::mem::zeroed () };
      /* Don't use sizeof(out), since that incorporates unwanted
         padding.  Instead, use the size through the last non-Lisp
         field.  */
      let sz: size_t = unsafe {
        ((&raw mut out.min_char) as *mut c_char).add (core::mem::size_of_val (&out.min_char))
          .offset_from ((&raw mut out) as *mut c_char)
      } as size_t;
      eassert! ((sz as intmax_t) < DUMP_OFF_MAX as intmax_t);
      dump_object_start (ctx, (&raw mut out) as *mut c_void, sz as dump_off);
      unsafe {
        DUMP_FIELD_COPY! (&raw mut out, sct, header.size);
        DUMP_FIELD_COPY! (&raw mut out, sct, depth);
        DUMP_FIELD_COPY! (&raw mut out, sct, min_char);
      }
      offset = dump_object_finish (ctx, (&raw const out) as *const c_void, sz as dump_off);
      skip = SUB_CHAR_TABLE_OFFSET as dump_off;
    }
  else
    {
      let mut out: vectorlike_header = unsafe { core::mem::zeroed () };
      dump_object_start (ctx, (&raw mut out) as *mut c_void,
                         core::mem::size_of_val (&out) as dump_off);
      unsafe { DUMP_FIELD_COPY! (&raw mut out, header, size); }
      offset = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                   core::mem::size_of_val (&out) as dump_off);
      skip = 0;
    }

  /* We may have written a non-Lisp vector prefix above.  If we have,
     pad to the lisp content start with zero, and make sure we didn't
     scribble beyond that start.  */
  let prefix_size: dump_off = unsafe { (*ctx).offset } - prefix_start_offset;
  eassert! (prefix_size > 0);
  let skip_start: dump_off = ptrdiff_t_to_dump_off (unsafe {
    ((&raw const FAM_ELEM_RO! ((*v).contents, skip)) as *const c_char).offset_from (v as *const c_char)
  });
  eassert! (skip_start >= prefix_size);
  dump_write_zero (ctx, skip_start - prefix_size);

  /* dump_object_start isn't what records conservative-GC object
     starts --- dump_object_1 does --- so the hack below of using
     dump_object_start for each vector word doesn't cause GC problems
     at runtime.  */
  let old_flags: dump_flags = unsafe { (*ctx).flags };
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_PACK_OBJECTS, true); }
  let mut i: dump_off = skip;
  while i < size as dump_off
    {
      let mut out: Lisp_Object = Qnil ();
      let vslot: *const Lisp_Object = unsafe { &raw const FAM_ELEM_RO! ((*v).contents, i) };
      /* In the wide case, we're always misaligned.  */
      eassert! (unsafe { (*ctx).offset } as usize % core::mem::size_of_val (&out) == 0);
      dump_object_start (ctx, (&raw mut out) as *mut c_void,
                         core::mem::size_of_val (&out) as dump_off);
      dump_field_lv (ctx, (&raw mut out) as *mut c_void, vslot as *const c_void, vslot,
                     WEIGHT_STRONG);
      dump_object_finish (ctx, (&raw const out) as *const c_void,
                          core::mem::size_of_val (&out) as dump_off);
      i += 1;
    }
  unsafe { (*ctx).flags = old_flags; }
  dump_align_output (ctx, DUMP_ALIGNMENT);
  return offset;
}

/* Return a vector of KEY, VALUE pairs in the given hash table H.
   No room for growth is included.  */
fn hash_table_contents (h: *mut Lisp_Hash_Table) -> *mut Lisp_Object
{
  let size: ptrdiff_t = unsafe { (*h).count as ptrdiff_t };
  let mut key_and_value: *mut Lisp_Object = unsafe {
    hash_table_alloc_bytes (2 * size * core::mem::size_of::<Lisp_Object> () as ptrdiff_t)
  };
  let mut n: ptrdiff_t = 0;

  unsafe {
    DOHASH! (h, k, v, {
      *key_and_value.offset (n) = k;
      n += 1;
      *key_and_value.offset (n) = v;
      n += 1;
    });
  }

  return key_and_value;
}

fn dump_hash_table_list (ctx: *mut dump_context)
{
  if !unsafe { NILP ((*ctx).hash_tables) }
    { dump_object (ctx, CALLN! (Fvconcat, unsafe { (*ctx).hash_tables })); }
}

fn hash_table_std_test (t: *const hash_table_test) -> hash_table_std_test_t
{
  if unsafe { BASE_EQ ((*t).name, Qeq ()) }
    { return Test_eq; }
  if unsafe { BASE_EQ ((*t).name, Qeql ()) }
    { return Test_eql; }
  if unsafe { BASE_EQ ((*t).name, Qequal ()) }
    { return Test_equal; }
  unsafe { error (c"cannot dump hash tables with user-defined tests".as_ptr ()); }  /* Bug#36769 */
}

/* Compact contents and discard inessential information from a hash table,
   preparing it for dumping.
   See `hash_table_thaw' for the code that restores the object to a usable
   state. */
fn hash_table_freeze (h: *mut Lisp_Hash_Table)
{
  unsafe {
    (*h).key_and_value = hash_table_contents (h);
    (*h).next = core::ptr::null_mut ();
    (*h).hash = core::ptr::null_mut ();
    (*h).index = core::ptr::null_mut ();
    (*h).table_size = 0;
    (*h).index_bits = 0;
    (*h).bits = ((*h).bits & !HASH_FROZEN_TEST_MASK)
      | ((hash_table_std_test ((*h).test) as u8) << 3);
    (*h).test = core::ptr::null ();
  }
}

fn dump_hash_table_contents (ctx: *mut dump_context, h: *mut Lisp_Hash_Table) -> dump_off
{
  dump_align_output (ctx, DUMP_ALIGNMENT);
  let start_offset: dump_off = unsafe { (*ctx).offset };
  let n: ptrdiff_t = 2 * unsafe { (*h).count as ptrdiff_t };

  let old_flags: dump_flags = unsafe { (*ctx).flags };
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_PACK_OBJECTS, true); }

  let mut i: ptrdiff_t = 0;
  while i < n
    {
      let mut out: Lisp_Object = Qnil ();
      let slot: *const Lisp_Object = unsafe { &raw const *(*h).key_and_value.offset (i) };
      dump_object_start (ctx, (&raw mut out) as *mut c_void,
                         core::mem::size_of_val (&out) as dump_off);
      dump_field_lv (ctx, (&raw mut out) as *mut c_void, slot as *const c_void, slot,
                     WEIGHT_STRONG);
      dump_object_finish (ctx, (&raw const out) as *const c_void,
                          core::mem::size_of_val (&out) as dump_off);
      i += 1;
    }

  unsafe { (*ctx).flags = old_flags; }
  return start_offset;
}

fn dump_hash_table (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  let hash_in: *const Lisp_Hash_Table = unsafe { XHASH_TABLE (object) };
  /* C's `struct Lisp_Hash_Table hash_munged = *hash_in;` — a struct copy of a type that
     is not `Copy`; `ptr::read` is the port's spelling (PORTING.md, status 3.51).  */
  let mut hash_munged: Lisp_Hash_Table = unsafe { core::ptr::read (hash_in) };
  let hash: *mut Lisp_Hash_Table = &mut hash_munged;

  hash_table_freeze (hash);
  unsafe { dump_push (&raw mut (*ctx).hash_tables, object); }

  START_DUMP_PVEC! (ctx, (&raw const (*hash).header), Lisp_Hash_Table, out);
  dump_pseudovector_lisp_fields (ctx, unsafe { &raw mut (*out).header },
                                 unsafe { &raw const (*hash).header });
  unsafe {
    DUMP_FIELD_COPY! (out, hash, count);
    /* C 2661-2663 copy the three bitfields `weakness` (bits 0-2), `mutable` (bit 5) and
       `frozen_test` (bits 3-4) of `bits`; one mask each.  */
    (*out).bits = ((*out).bits & !HASH_WEAKNESS_MASK) | ((*hash).bits & HASH_WEAKNESS_MASK);
    (*out).bits = ((*out).bits & !HASH_MUTABLE_MASK) | ((*hash).bits & HASH_MUTABLE_MASK);
    (*out).bits = ((*out).bits & !HASH_FROZEN_TEST_MASK)
      | ((*hash).bits & HASH_FROZEN_TEST_MASK);
    if !(*hash).key_and_value.is_null ()
      { dump_field_fixup_later (ctx, out as *mut c_void, hash as *const c_void,
                                (&raw const (*hash).key_and_value) as *const c_void); }
    eassert! ((*hash).next_weak.is_null ());
    let offset: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    if !(*hash).key_and_value.is_null ()
      { dump_remember_fixup_ptr_raw
        (ctx,
         offset + dump_offsetof! (Lisp_Hash_Table, key_and_value) as dump_off,
         dump_hash_table_contents (ctx, hash)); }
    return offset;
  }
}

fn dump_obarray_buckets (ctx: *mut dump_context, o: *const Lisp_Obarray) -> dump_off
{
  dump_align_output (ctx, DUMP_ALIGNMENT);
  let start_offset: dump_off = unsafe { (*ctx).offset };
  let n: ptrdiff_t = unsafe { obarray_size (o as *mut Lisp_Obarray) };

  let old_flags: dump_flags = unsafe { (*ctx).flags };
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_PACK_OBJECTS, true); }

  let mut i: ptrdiff_t = 0;
  while i < n
    {
      let mut out: Lisp_Object = Qnil ();
      let slot: *const Lisp_Object = unsafe { &raw const *(*o).buckets.offset (i) };
      dump_object_start (ctx, (&raw mut out) as *mut c_void,
                         core::mem::size_of_val (&out) as dump_off);
      dump_field_lv (ctx, (&raw mut out) as *mut c_void, slot as *const c_void, slot,
                     WEIGHT_STRONG);
      dump_object_finish (ctx, (&raw const out) as *const c_void,
                          core::mem::size_of_val (&out) as dump_off);
      i += 1;
    }

  unsafe { (*ctx).flags = old_flags; }
  return start_offset;
}

fn dump_obarray (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  let in_oa: *const Lisp_Obarray = unsafe { XOBARRAY (object) };
  let mut munged_oa: Lisp_Obarray = unsafe { core::ptr::read (in_oa) };
  let oa: *mut Lisp_Obarray = &mut munged_oa;
  START_DUMP_PVEC! (ctx, (&raw const (*oa).header), Lisp_Obarray, out);
  dump_pseudovector_lisp_fields (ctx, unsafe { &raw mut (*out).header },
                                 unsafe { &raw const (*oa).header });
  unsafe {
    DUMP_FIELD_COPY! (out, oa, count);
    DUMP_FIELD_COPY! (out, oa, size_bits);
    dump_field_fixup_later (ctx, out as *mut c_void, oa as *const c_void,
                            (&raw const (*oa).buckets) as *const c_void);
    let offset: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    dump_remember_fixup_ptr_raw
      (ctx,
       offset + dump_offsetof! (Lisp_Obarray, buckets) as dump_off,
       dump_obarray_buckets (ctx, oa));
    return offset;
  }
}

fn dump_buffer (ctx: *mut dump_context, in_buffer: *const buffer) -> dump_off
{
  /* C's `struct buffer munged_buffer = *in_buffer;` (the `ptr::read` note above).  */
  let mut munged_buffer: buffer = unsafe { core::ptr::read (in_buffer) };
  let buffer: *mut buffer = &mut munged_buffer;

  unsafe {
    /* Clear some buffer state for correctness upon load.  */
    if (*buffer).base_buffer.is_null ()
      { (*buffer).window_count = 0; }
    else
      { eassert! ((*buffer).window_count == -1); }
    (*buffer).local_minor_modes_ = Qnil ();
    (*buffer).last_name_ = Qnil ();
    (*buffer).last_selected_window_ = Qnil ();
    (*buffer).display_count_ = make_fixnum (0);
    (*buffer).bits = (*buffer).bits & !BUFFER_CLIP_CHANGED;   /* clip_changed = 0 */
    (*buffer).last_window_start = -1;
    (*buffer).point_before_scroll_ = Qnil ();
  }

  let mut base_offset: dump_off = 0;
  if unsafe { !(*buffer).base_buffer.is_null () }
    {
      eassert! (unsafe { (*(*buffer).base_buffer).base_buffer.is_null () });
      base_offset = dump_object_for_offset
	(ctx,
	 make_lisp_ptr (unsafe { (*buffer).base_buffer } as *mut c_void,
                        Lisp_Type::Lisp_Vectorlike));
    }

  eassert! ((base_offset == 0
             && unsafe { (*buffer).text == (&raw const (*in_buffer).own_text) as *mut buffer_text })
	    || (base_offset > 0
	        && unsafe { (*buffer).text != (&raw const (*in_buffer).own_text) as *mut buffer_text }));

  START_DUMP_PVEC! (ctx, (&raw const (*buffer).header), buffer, out);
  dump_pseudovector_lisp_fields (ctx, unsafe { &raw mut (*out).header },
                                 unsafe { &raw const (*buffer).header });
  if base_offset == 0
    { base_offset = unsafe { (*ctx).obj_offset }; }
  eassert! (base_offset > 0);
  unsafe {
    if (*buffer).base_buffer.is_null ()
      {
        eassert! (base_offset == (*ctx).obj_offset);

        if BUFFER_LIVE_P (buffer)
          {
            dump_field_fixup_later (ctx, out as *mut c_void, buffer as *const c_void,
                                    (&raw const (*buffer).own_text.beg) as *const c_void);
	    dump_remember_cold_op (ctx, cold_op::COLD_OP_BUFFER,
				   make_lisp_ptr (in_buffer as *mut c_void,
						  Lisp_Type::Lisp_Vectorlike));
          }
        else
          { eassert! ((*buffer).own_text.beg.is_null ()); }

        DUMP_FIELD_COPY! (out, buffer, own_text.gpt);
        DUMP_FIELD_COPY! (out, buffer, own_text.z);
        DUMP_FIELD_COPY! (out, buffer, own_text.gpt_byte);
        DUMP_FIELD_COPY! (out, buffer, own_text.z_byte);
        DUMP_FIELD_COPY! (out, buffer, own_text.gap_size);
        DUMP_FIELD_COPY! (out, buffer, own_text.modiff);
        DUMP_FIELD_COPY! (out, buffer, own_text.chars_modiff);
        DUMP_FIELD_COPY! (out, buffer, own_text.save_modiff);
        DUMP_FIELD_COPY! (out, buffer, own_text.overlay_modiff);
        DUMP_FIELD_COPY! (out, buffer, own_text.compact);
        DUMP_FIELD_COPY! (out, buffer, own_text.beg_unchanged);
        DUMP_FIELD_COPY! (out, buffer, own_text.end_unchanged);
        DUMP_FIELD_COPY! (out, buffer, own_text.unchanged_modified);
        DUMP_FIELD_COPY! (out, buffer, own_text.overlay_unchanged_modified);
        if !(*buffer).own_text.intervals.is_null ()
          { dump_field_fixup_later (ctx, out as *mut c_void, buffer as *const c_void,
                                    (&raw const (*buffer).own_text.intervals) as *const c_void); }
        dump_field_lv_rawptr (ctx, out as *mut c_void, buffer as *const c_void,
                              (&raw const (*buffer).own_text.markers) as *const c_void,
                              Lisp_Type::Lisp_Vectorlike, WEIGHT_NORMAL);
        (*out).own_text.bits = ((*out).own_text.bits & !BUFFER_TEXT_INHIBIT_SHRINKING)
          | ((*buffer).own_text.bits & BUFFER_TEXT_INHIBIT_SHRINKING);
        (*out).own_text.bits = ((*out).own_text.bits & !BUFFER_TEXT_REDISPLAY)
          | ((*buffer).own_text.bits & BUFFER_TEXT_REDISPLAY);
      }

    eassert! ((*ctx).obj_offset > 0);
    dump_remember_fixup_ptr_raw
      (ctx,
       (*ctx).obj_offset + dump_offsetof! (buffer, text) as dump_off,
       base_offset + dump_offsetof! (buffer, own_text) as dump_off);

    DUMP_FIELD_COPY! (out, buffer, pt);
    DUMP_FIELD_COPY! (out, buffer, pt_byte);
    DUMP_FIELD_COPY! (out, buffer, begv);
    DUMP_FIELD_COPY! (out, buffer, begv_byte);
    DUMP_FIELD_COPY! (out, buffer, zv);
    DUMP_FIELD_COPY! (out, buffer, zv_byte);

    if !(*buffer).base_buffer.is_null ()
      {
        eassert! ((*ctx).obj_offset != base_offset);
        dump_field_ptr_to_dump_offset (ctx, out as *mut c_void, buffer as *const c_void,
                                       (&raw const (*buffer).base_buffer) as *const c_void,
				       base_offset);
      }

    DUMP_FIELD_COPY! (out, buffer, indirections);
    DUMP_FIELD_COPY! (out, buffer, window_count);

    memcpy ((&raw mut (*out).local_flags) as *mut c_void,
            (&raw const (*buffer).local_flags) as *const c_void,
            core::mem::size_of_val (&(*out).local_flags));
    DUMP_FIELD_COPY! (out, buffer, modtime);
    DUMP_FIELD_COPY! (out, buffer, modtime_size);
    DUMP_FIELD_COPY! (out, buffer, auto_save_modified);
    DUMP_FIELD_COPY! (out, buffer, display_error_modiff);
    DUMP_FIELD_COPY! (out, buffer, auto_save_failure_time);
    DUMP_FIELD_COPY! (out, buffer, last_window_start);

    /* Not worth serializing these caches.  TODO: really? */
    (*out).newline_cache = core::ptr::null_mut ();
    (*out).width_run_cache = core::ptr::null_mut ();
    (*out).bidi_paragraph_cache = core::ptr::null_mut ();

    /* C 2859-2862's four `bool_bf` copies, in buffer.h's order.  */
    (*out).bits = ((*out).bits & !BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P)
      | ((*buffer).bits & BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P);
    (*out).bits = ((*out).bits & !BUFFER_CLIP_CHANGED) | ((*buffer).bits & BUFFER_CLIP_CHANGED);
    (*out).bits = ((*out).bits & !BUFFER_INHIBIT_BUFFER_HOOKS)
      | ((*buffer).bits & BUFFER_INHIBIT_BUFFER_HOOKS);
    (*out).bits = ((*out).bits & !BUFFER_LONG_LINE_OPTIMIZATIONS_P)
      | ((*buffer).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P);

    if !itree_empty_p ((*buffer).overlays)
      {
        /* We haven't implemented the code to dump overlays.  */
        error (c"dumping overlays is not yet implemented".as_ptr ());
      }
    else
      { (*out).overlays = core::ptr::null_mut (); }

    dump_field_lv (ctx, out as *mut c_void, buffer as *const c_void,
                   &raw const (*buffer).undo_list_,
                   WEIGHT_STRONG);
    let offset: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    if (*buffer).base_buffer.is_null () && !(*buffer).own_text.intervals.is_null ()
      { dump_remember_fixup_ptr_raw
        (ctx,
         offset + dump_offsetof! (buffer, own_text.intervals) as dump_off,
         dump_interval_tree (ctx, (*buffer).own_text.intervals as INTERVAL, 0)); }

    return offset;
  }
}

fn dump_bool_vector (ctx: *mut dump_context, v: *const Lisp_Vector) -> dump_off
{
  /* No relocation needed, so we don't need dump_object_start.  */
  dump_align_output (ctx, DUMP_ALIGNMENT);
  eassert! (unsafe { (*ctx).offset } >= unsafe { (*ctx).header.cold_start });
  let offset: dump_off = unsafe { (*ctx).offset };
  let nbytes: ptrdiff_t = unsafe { vector_nbytes (v as *mut Lisp_Vector) };
  if nbytes > DUMP_OFF_MAX as ptrdiff_t
    { unsafe { error (c"vector too large".as_ptr ()); } }
  dump_write (ctx, v as *const c_void, ptrdiff_t_to_dump_off (nbytes));
  return offset;
}

fn dump_subr (ctx: *mut dump_context, subr: *const Lisp_Subr) -> dump_off
{
  let mut out: Lisp_Subr = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  unsafe { DUMP_FIELD_COPY! (&raw mut out, subr, header.size); }
  /* C 2913-2935's `#ifdef HAVE_NATIVE_COMP` arms are dead (`HAVE_NATIVE_COMP` is undef in
     the frozen configuration); both arms are carried (P-51, docs/status.md 3.49).  C's
     `#else`-less `if (non_primitive) ... else` chain means the shared statement is C's
     second arm; the port writes it under each `#[cfg]`.  */
  #[cfg (feature = "have_native_comp")]
  let non_primitive: bool = !NILP (unsafe { (*subr).native_comp_u });
  #[cfg (feature = "have_native_comp")]
  unsafe {
    if non_primitive
      { out.function.a0 = None; }   /* C: `out.function.a0 = NULL;` */
    else
      { dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                              (&raw const (*subr).function.a0) as *const c_void); }
  }
  #[cfg (not (feature = "have_native_comp"))]
  unsafe {
    dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                          (&raw const (*subr).function.a0) as *const c_void);
  }
  unsafe {
    DUMP_FIELD_COPY! (&raw mut out, subr, min_args);
    DUMP_FIELD_COPY! (&raw mut out, subr, max_args);
  }
  #[cfg (feature = "have_native_comp")]
  unsafe {
    if non_primitive
      {
        dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                                (&raw const (*subr).symbol_name) as *const c_void);
        dump_remember_cold_op (ctx,
                               cold_op::COLD_OP_NATIVE_SUBR,
			       make_lisp_ptr (subr as *mut c_void,
					      Lisp_Type::Lisp_Vectorlike));
        dump_field_lv (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                       &raw const (*subr).intspec.native, WEIGHT_NORMAL);
        dump_field_lv (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                       &raw const (*subr).command_modes, WEIGHT_NORMAL);
      }
    else
      {
        dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                              (&raw const (*subr).symbol_name) as *const c_void);
        dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                              (&raw const (*subr).intspec.string) as *const c_void);
        dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                              (&raw const (*subr).command_modes) as *const c_void);
      }
  }
  #[cfg (not (feature = "have_native_comp"))]
  unsafe {
    dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                          (&raw const (*subr).symbol_name) as *const c_void);
    dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                          (&raw const (*subr).intspec.string) as *const c_void);
    dump_field_emacs_ptr (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                          (&raw const (*subr).command_modes) as *const c_void);
  }
  unsafe { DUMP_FIELD_COPY! (&raw mut out, subr, doc); }
  #[cfg (feature = "have_native_comp")]
  unsafe {
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                   &raw const (*subr).native_comp_u, WEIGHT_NORMAL);
    if !NILP ((*subr).native_comp_u)
      { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                                (&raw const (*subr).native_c_name) as *const c_void); }

    dump_field_lv (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                   &raw const (*subr).lambda_list, WEIGHT_NORMAL);
    dump_field_lv (ctx, (&raw mut out) as *mut c_void, subr as *const c_void,
                   &raw const (*subr).r#type, WEIGHT_NORMAL);
  }
  let subr_off: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                               core::mem::size_of_val (&out) as dump_off);
  #[cfg (feature = "have_native_comp")]
  unsafe {
    if non_primitive && ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      /* We'll do the final addr relocation during VERY_LATE_RELOCS time
         after the compilation units has been loaded. */
      { dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::VERY_LATE_RELOCS as usize],
	           list2 (make_fixnum (dump_reloc_type::RELOC_NATIVE_SUBR as isize),
		          dump_off_to_lisp (subr_off))); }
  }
  return subr_off;
}

/* C 2945-2967's `#ifdef HAVE_NATIVE_COMP` arm is dead; carried as text (P-51).  The record
   it names is declared with the other `comp.h` names below (Rust items are
   order-independent).  */
#[cfg (feature = "have_native_comp")]
fn dump_native_comp_unit (ctx: *mut dump_context, comp_u: *mut Lisp_Native_Comp_Unit)
  -> dump_off
{
  if !unsafe { CONSP ((*comp_u).file) }
    { unsafe { error (c"trying to dump non fixed-up eln file".as_ptr ()); } }

  unsafe {
    /* Have function documentation always lazy loaded to optimize load-time.  */
    (*comp_u).data_fdoc_v = Qnil ();
    START_DUMP_PVEC! (ctx, (&raw const (*comp_u).header), Lisp_Native_Comp_Unit, out);
    dump_pseudovector_lisp_fields (ctx, &raw mut (*out).header,
                                   &raw const (*comp_u).header);
    (*out).handle = core::ptr::null_mut ();

    let comp_u_off: dump_off = finish_dump_pvec (ctx, &raw mut (*out).header);
    if ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS) != 0
      /* We'll do the real elf load during LATE_RELOCS relocation time. */
      { dump_push (&raw mut (*ctx).dump_relocs[reloc_phase::LATE_RELOCS as usize],
	           list2 (make_fixnum (dump_reloc_type::RELOC_NATIVE_COMP_UNIT as isize),
		          dump_off_to_lisp (comp_u_off))); }
    return comp_u_off;
  }
}

fn fill_pseudovec (header: *mut vectorlike_header, item: Lisp_Object)
{
  let v: *mut Lisp_Vector = header as *mut Lisp_Vector;
  eassert! (unsafe { (*v).header.size } & PSEUDOVECTOR_FLAG != 0);
  let size: ptrdiff_t =
    (unsafe { (*v).header.size } & PSEUDOVECTOR_SIZE_MASK) as ptrdiff_t;
  let mut idx: ptrdiff_t = 0;
  while idx < size
    {
      unsafe { FAM_ELEM! ((*v).contents, idx) = item; }
      idx += 1;
    }
}

fn dump_nilled_pseudovec (ctx: *mut dump_context, in_: *const vectorlike_header) -> dump_off
{
  START_DUMP_PVEC! (ctx, in_, Lisp_Vector, out);
  fill_pseudovec (unsafe { &raw mut (*out).header }, Qnil ());
  return finish_dump_pvec (ctx, unsafe { &raw mut (*out).header });
}

fn dump_vectorlike (ctx: *mut dump_context, lv: Lisp_Object, offset: dump_off) -> dump_off
{
  let v: *const Lisp_Vector = unsafe { XVECTOR (lv) };
  let ptype: pvec_type = unsafe { PSEUDOVECTOR_TYPE (v as *mut Lisp_Vector) };
  /* C's `break` arms leave the `switch` for the `error_unsupported_dump_object` tail below;
     a Rust `match` arm has nothing to break out of, so the `switch` is a labelled block
     (PORTING.md 2.2).  */
  'dump_vectorlike: {
  match ptype
    {
      pvec_type::PVEC_FONT
      /* There are three kinds of font objects that all use PVEC_FONT,
         distinguished by their size.  Font specs and entities are
         harmless data carriers that we can dump like other Lisp
         objects.  Fonts themselves are window-system-specific and
         need to be recreated on each startup.  */
      | pvec_type::PVEC_NORMAL_VECTOR
      | pvec_type::PVEC_CLOSURE
      | pvec_type::PVEC_CHAR_TABLE
      | pvec_type::PVEC_SUB_CHAR_TABLE
      | pvec_type::PVEC_RECORD
        => {
          /* C's `FALLTHROUGH;` sends the PVEC_FONT arm into this one (PORTING.md 2.2: a
             `match` arm has no fallthrough, so the check is the arm's first statement).  */
          if ptype == pvec_type::PVEC_FONT
            && (unsafe { (*v).header.size } & PSEUDOVECTOR_SIZE_MASK) != FONT_SPEC_MAX as isize
            && (unsafe { (*v).header.size } & PSEUDOVECTOR_SIZE_MASK) != FONT_ENTITY_MAX as isize
            { error_unsupported_dump_object (ctx, lv, c"font".as_ptr ()); }
          return dump_vectorlike_generic (ctx, unsafe { &raw const (*v).header });
        }
      pvec_type::PVEC_BOOL_VECTOR
        => { return dump_bool_vector (ctx, v); }
      pvec_type::PVEC_HASH_TABLE
        => { return dump_hash_table (ctx, lv); }
      pvec_type::PVEC_OBARRAY
        => { return dump_obarray (ctx, lv); }
      pvec_type::PVEC_BUFFER
        => { return dump_buffer (ctx, unsafe { XBUFFER (lv) }); }
      pvec_type::PVEC_SUBR
        => { return dump_subr (ctx, unsafe { XSUBR (lv) }); }
      pvec_type::PVEC_FRAME
      | pvec_type::PVEC_WINDOW
      | pvec_type::PVEC_PROCESS
      | pvec_type::PVEC_TERMINAL
        => { return dump_nilled_pseudovec (ctx, unsafe { &raw const (*v).header }); }
      pvec_type::PVEC_MARKER
        => { return dump_marker (ctx, unsafe { XMARKER (lv) }); }
      pvec_type::PVEC_OVERLAY
        => { return dump_overlay (ctx, unsafe { XOVERLAY (lv) }); }
      pvec_type::PVEC_FINALIZER
        => { return dump_finalizer (ctx, unsafe { XFINALIZER (lv) }); }
      pvec_type::PVEC_BIGNUM
        => { return dump_bignum (ctx, lv); }
      pvec_type::PVEC_NATIVE_COMP_UNIT
        => {
          /* C 3049-3054's `#ifdef HAVE_NATIVE_COMP` arm is dead; carried (P-51).  */
          #[cfg (feature = "have_native_comp")]
          { return dump_native_comp_unit (ctx, unsafe { XNATIVE_COMP_UNIT (lv) }); }
          #[cfg (not (feature = "have_native_comp"))]
          { break 'dump_vectorlike; }
        }
      pvec_type::PVEC_THREAD
        => {
          if unsafe { main_thread_p (v as *const c_void) }
            {
              eassert! (!dump_object_emacs_ptr (lv).is_null ());
              return dump_object_special_offset::DUMP_OBJECT_IS_RUNTIME_MAGIC as dump_off;
            }
          break 'dump_vectorlike;
        }
      #[cfg (feature = "have_tree_sitter")]
      pvec_type::PVEC_TS_COMPILED_QUERY
      => { return dump_treesit_compiled_query (ctx, unsafe { XTS_COMPILED_QUERY (lv) }); }
      pvec_type::PVEC_TS_COMPILED_QUERY
      /* C 3061-3065's `#ifdef HAVE_TREE_SITTER` return is dead on this target; the case
         falls into the `break` group below (P-51).  */
      | pvec_type::PVEC_WINDOW_CONFIGURATION
      | pvec_type::PVEC_OTHER
      | pvec_type::PVEC_XWIDGET
      | pvec_type::PVEC_XWIDGET_VIEW
      | pvec_type::PVEC_MISC_PTR
      | pvec_type::PVEC_USER_PTR
      | pvec_type::PVEC_MUTEX
      | pvec_type::PVEC_CONDVAR
      | pvec_type::PVEC_SQLITE
      | pvec_type::PVEC_MODULE_FUNCTION
      | pvec_type::PVEC_SYMBOL_WITH_POS
      | pvec_type::PVEC_FREE
      | pvec_type::PVEC_TS_PARSER
      | pvec_type::PVEC_TS_NODE
        => { break 'dump_vectorlike; }
    }
  }
  let iptype: c_int = ptype as c_int;
  /* gnulib's INT_STRLEN_BOUND (t) is `INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - signed) +
     signed`, and `INT_STRLEN_BOUND (int)` is ((32 - 1) * 146 + 484) / 485 + 1 = 11 (the
     emacs.rs/data.rs/json.rs measurement: `emacs/src/emacs.c`'s `sprintf` buffer uses the
     same macro, and no `<intprops.h>` mirror exists yet, P-26).  */
  const INT_STRLEN_BOUND: usize = ((32 - 1) * 146 + 484) / 485 + 1;
  const fmt: [u8; 21] = *b"pseudovector type %d\0";
  let mut msg: [c_char; core::mem::size_of_val (&fmt) - 3 + INT_STRLEN_BOUND + 1] =
    [0; core::mem::size_of_val (&fmt) - 3 + INT_STRLEN_BOUND + 1];
  unsafe {
    sprintf (msg.as_mut_ptr (), fmt.as_ptr () as *const c_char, iptype);
    error_unsupported_dump_object (ctx, lv, msg.as_ptr ());
  }
}

/* Add an object to the dump.

   CTX is the dump context; OBJECT is the object to add.  Normally,
   return OFFSET, the location (in bytes, from the start of the dump
   file) where we wrote the object.  Valid OFFSETs are always greater
   than zero.

   If we've already dumped an object, return the location where we put
   it: dump_object is idempotent.

   The object must refer to an actual pointer-ish object of some sort.
   Some self-representing objects are immediate values rather than
   tagged pointers to Lisp heap structures and so have no individual
   representation in the Lisp heap dump.

   May also return one of the DUMP_OBJECT_ON_*_QUEUE constants if we
   "dumped" the object by remembering to process it specially later.
   In this case, we don't have a valid offset.
   Call dump_object_for_offset if you need a valid offset for
   an object.
 */
fn dump_object (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  eassert! (!EQ (object, unsafe { dead_object () }));

  let mut offset: dump_off = dump_recall_object (ctx, object);
  if offset > 0
    { return offset; }  /* Object already dumped.  */

  let cold: bool = BOOL_VECTOR_P (object) || FLOATP (object);
  if cold && unsafe { (*ctx).flags.bits & DUMP_FLAGS_DEFER_COLD_OBJECTS } != 0
    {
      if offset != dump_object_special_offset::DUMP_OBJECT_ON_COLD_QUEUE as dump_off
        {
	  eassert! (offset == dump_object_special_offset::DUMP_OBJECT_ON_NORMAL_QUEUE as dump_off
		    || offset == dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off);
          offset = dump_object_special_offset::DUMP_OBJECT_ON_COLD_QUEUE as dump_off;
          dump_remember_object (ctx, object, offset);
          dump_remember_cold_op (ctx, cold_op::COLD_OP_OBJECT, object);
        }
      return offset;
    }

  let obj_in_emacs: *mut c_void = dump_object_emacs_ptr (object);
  if !obj_in_emacs.is_null ()
    && unsafe { (*ctx).flags.bits & DUMP_FLAGS_DEFER_COPIED_OBJECTS } != 0
    {
      if offset != dump_object_special_offset::DUMP_OBJECT_ON_COPIED_QUEUE as dump_off
        {
	  eassert! (offset == dump_object_special_offset::DUMP_OBJECT_ON_NORMAL_QUEUE as dump_off
		    || offset == dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off);
          /* Even though we're not going to dump this object right
             away, we still want to scan and enqueue its
             referents.  */
          let old_flags: dump_flags = unsafe { (*ctx).flags };
          unsafe {
            set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DUMP_OBJECT_CONTENTS, false);
            set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_COPIED_OBJECTS, false);
          }
          dump_object (ctx, object);
          unsafe { (*ctx).flags = old_flags; }

          offset = dump_object_special_offset::DUMP_OBJECT_ON_COPIED_QUEUE as dump_off;
          dump_remember_object (ctx, object, offset);
          unsafe { dump_push (&raw mut (*ctx).copied_queue, object); }
        }
      return offset;
    }

  /* Object needs to be dumped.  */
  if dump_set_referrer (ctx)
    { unsafe { (*ctx).current_referrer = object; } }
  match XTYPE (object)
    {
      Lisp_Type::Lisp_String
        => { offset = dump_string (ctx, unsafe { XSTRING (object) }); }
      Lisp_Type::Lisp_Vectorlike
        => { offset = dump_vectorlike (ctx, object, offset); }
      Lisp_Type::Lisp_Symbol
        => { offset = dump_symbol (ctx, object, offset); }
      Lisp_Type::Lisp_Cons
        => { offset = dump_cons (ctx, unsafe { XCONS (object) }); }
      Lisp_Type::Lisp_Float
        => { offset = dump_float (ctx, unsafe { XFLOAT (object) }); }
      Lisp_Type::Lisp_Int0
      | Lisp_Type::Lisp_Int1
        => {
          eassert! (false && 0 != 0);   /* `should not be dumping int: is self-representing' */
          unsafe { abort (); }
        }
      _ => { unsafe { emacs_abort (); } }
    }
  dump_clear_referrer (ctx);

  /* offset can be < 0 if we've deferred an object.  */
  if unsafe { (*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS } != 0
      && offset > dump_object_special_offset::DUMP_OBJECT_NOT_SEEN as dump_off
    {
      eassert! (offset % DUMP_ALIGNMENT == 0);
      dump_remember_object (ctx, object, offset);
      if unsafe { (*ctx).flags.bits & DUMP_FLAGS_RECORD_OBJECT_STARTS } != 0
        {
          eassert! (unsafe { (*ctx).flags.bits & DUMP_FLAGS_PACK_OBJECTS } == 0);
          unsafe {
            dump_push (&raw mut (*ctx).object_starts,
                       list2 (dump_off_to_lisp (XTYPE (object) as dump_off),
                              dump_off_to_lisp (offset)));
          }
        }
    }

  return offset;
}

/* Like dump_object(), but assert that we get a valid offset.  */
fn dump_object_for_offset (ctx: *mut dump_context, object: Lisp_Object) -> dump_off
{
  let offset: dump_off = dump_object (ctx, object);
  eassert! (offset > 0);
  return offset;
}

fn dump_charset (ctx: *mut dump_context, cs_i: c_int) -> dump_off
{
  /* We can't change the alignment here, because ctx->offset is what
     will be used for the whole array.  */
  eassert! (unsafe { (*ctx).offset } as usize % core::mem::align_of::<charset> () == 0);
  let cs: *const charset = unsafe { charset_table.start.offset (cs_i as isize) };
  let mut out: charset = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut out) as *mut c_void,
                     core::mem::size_of_val (&out) as dump_off);
  if cs_i < unsafe { charset_table.used } /* Don't look at uninitialized data.  */
    {
      unsafe {
        DUMP_FIELD_COPY! (&raw mut out, cs, id);
        DUMP_FIELD_COPY! (&raw mut out, cs, dimension);
        memcpy ((&raw mut out.code_space) as *mut c_void,
                (&raw const (*cs).code_space) as *const c_void,
                core::mem::size_of_val (&(*cs).code_space));
        if !(*cs).code_space_mask.is_null ()
          { dump_field_fixup_later (ctx, (&raw mut out) as *mut c_void, cs as *const c_void,
                                    (&raw const (*cs).code_space_mask) as *const c_void); }
        /* C 3235-3240's six `bool_bf` copies (`code_linear_p`, `iso_chars_96`,
           `ascii_compatible_p`, `supplementary_p`, `compact_codes_p`, `unified_p`).  */
        out.bits = (out.bits & !CHARSET_CODE_LINEAR_P) | ((*cs).bits & CHARSET_CODE_LINEAR_P);
        out.bits = (out.bits & !CHARSET_ISO_CHARS_96) | ((*cs).bits & CHARSET_ISO_CHARS_96);
        out.bits = (out.bits & !CHARSET_ASCII_COMPATIBLE_P)
          | ((*cs).bits & CHARSET_ASCII_COMPATIBLE_P);
        out.bits = (out.bits & !CHARSET_SUPPLEMENTARY_P)
          | ((*cs).bits & CHARSET_SUPPLEMENTARY_P);
        out.bits = (out.bits & !CHARSET_COMPACT_CODES_P)
          | ((*cs).bits & CHARSET_COMPACT_CODES_P);
        out.bits = (out.bits & !CHARSET_UNIFIED_P) | ((*cs).bits & CHARSET_UNIFIED_P);
        DUMP_FIELD_COPY! (&raw mut out, cs, iso_final);
        DUMP_FIELD_COPY! (&raw mut out, cs, iso_revision);
        DUMP_FIELD_COPY! (&raw mut out, cs, emacs_mule_id);
        DUMP_FIELD_COPY! (&raw mut out, cs, method);
        DUMP_FIELD_COPY! (&raw mut out, cs, min_code);
        DUMP_FIELD_COPY! (&raw mut out, cs, max_code);
        DUMP_FIELD_COPY! (&raw mut out, cs, char_index_offset);
        DUMP_FIELD_COPY! (&raw mut out, cs, min_char);
        DUMP_FIELD_COPY! (&raw mut out, cs, max_char);
        DUMP_FIELD_COPY! (&raw mut out, cs, invalid_code);
        memcpy ((&raw mut out.fast_map) as *mut c_void,
                (&raw const (*cs).fast_map) as *const c_void,
                core::mem::size_of_val (&(*cs).fast_map));
        DUMP_FIELD_COPY! (&raw mut out, cs, code_offset);
      }
    }
  let offset: dump_off = dump_object_finish (ctx, (&raw const out) as *const c_void,
                                             core::mem::size_of_val (&out) as dump_off);
  if cs_i < unsafe { charset_table.used }
    && !unsafe { (*cs).code_space_mask.is_null () }
    { unsafe {
        dump_remember_cold_op (ctx, cold_op::COLD_OP_CHARSET,
                               Fcons (dump_off_to_lisp (cs_i),
                                      dump_off_to_lisp (offset))); } }
  return offset;
}

fn dump_charset_table (ctx: *mut dump_context) -> dump_off
{
  let old_flags: dump_flags = unsafe { (*ctx).flags };
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_PACK_OBJECTS, true); }
  dump_align_output (ctx, core::mem::align_of::<charset> () as c_int);
  let offset: dump_off = unsafe { (*ctx).offset };
  if dump_set_referrer (ctx)
    { unsafe { (*ctx).current_referrer = build_string (c"charset_table".as_ptr ()); } }
  eassert! (unsafe { charset_table.size } == unsafe { charset_table.used });
  let mut i: c_int = 0;
  while i < unsafe { charset_table.size }
    {
      dump_charset (ctx, i);
      i += 1;
    }
  dump_clear_referrer (ctx);
  dump_emacs_reloc_to_dump_ptr_raw (ctx, (&raw mut charset_table) as *const c_void, offset);
  unsafe { (*ctx).flags = old_flags; }
  return offset;
}

fn dump_finalizer_list_head_ptr (ctx: *mut dump_context, ptr: *mut *mut Lisp_Finalizer)
{
  unsafe {
    let value: *mut Lisp_Finalizer = *ptr;
    if value != (&raw mut finalizers) && value != (&raw mut doomed_finalizers)
      { dump_emacs_reloc_to_dump_ptr_raw
        (ctx, ptr as *const c_void,
         dump_object_for_offset (ctx,
			         make_lisp_ptr (value as *mut c_void,
                                                Lisp_Type::Lisp_Vectorlike))); }
  }
}

fn dump_metadata_for_pdumper (ctx: *mut dump_context)
{
  unsafe {
    let mut i: c_int = 0;
    while i < nr_dump_hooks
      {
        dump_emacs_reloc_to_emacs_ptr_raw (ctx, (&raw mut dump_hooks[i as usize]) as *mut c_void,
				           dump_hooks[i as usize].map_or (core::ptr::null (), |f| f as *const c_void));
        i += 1;
      }
    dump_emacs_reloc_immediate_int (ctx, &raw const nr_dump_hooks, nr_dump_hooks);

    i = 0;
    while i < nr_dump_late_hooks
      {
        dump_emacs_reloc_to_emacs_ptr_raw (ctx, (&raw mut dump_late_hooks[i as usize]) as *mut c_void,
				           dump_late_hooks[i as usize].map_or (core::ptr::null (), |f| f as *const c_void));
        i += 1;
      }
    dump_emacs_reloc_immediate_int (ctx, &raw const nr_dump_late_hooks, nr_dump_late_hooks);

    i = 0;
    while i < nr_remembered_data
      {
        dump_emacs_reloc_to_emacs_ptr_raw (ctx,
                                           (&raw mut remembered_data[i as usize].mem) as *mut c_void,
					   remembered_data[i as usize].mem);
        dump_emacs_reloc_immediate_int (ctx, &raw const remembered_data[i as usize].sz,
				        remembered_data[i as usize].sz);
        i += 1;
      }
    dump_emacs_reloc_immediate_int (ctx, &raw const nr_remembered_data, nr_remembered_data);
  }
}

/* Sort the list of copied objects in CTX.  */
fn dump_sort_copied_objects (ctx: *mut dump_context)
{
  let queue_reversed: Lisp_Object = unsafe { Fnreverse ((*ctx).copied_queue) };
  /* Sort the objects into the order in which they'll appear in the
     Emacs: this way, on startup, we'll do both the IO from the dump
     file and the copy into Emacs in-order, where prefetch will be
     most effective.  */
  unsafe {
    (*ctx).copied_queue
      = CALLN! (Fsort, queue_reversed,
	        Qdump_emacs_portable__sort_predicate_copied ());
  }
}

/* Dump parts of copied objects we need at runtime.  */
fn dump_hot_parts_of_discardable_objects (ctx: *mut dump_context)
{
  let mut copied_queue: Lisp_Object = unsafe { (*ctx).copied_queue };
  while !NILP (copied_queue)
    {
      let copied: Lisp_Object = dump_pop (&mut copied_queue);
      if SYMBOLP (copied)
        {
          eassert! (dump_builtin_symbol_p (copied));
          dump_pre_dump_symbol (ctx, unsafe { XSYMBOL (copied) });
        }
    }
}

fn dump_drain_copied_objects (ctx: *mut dump_context)
{
  let mut copied_queue: Lisp_Object = unsafe { (*ctx).copied_queue };
  unsafe { (*ctx).copied_queue = Qnil (); }

  let old_flags: dump_flags = unsafe { (*ctx).flags };

  unsafe {
    /* We should have already fully scanned these objects, so assert
       that we're not adding more entries to the dump queue.  */
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_ASSERT_ALREADY_SEEN, true);

    /* Now we want to actually dump the copied objects, not just record
       them.  */
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_COPIED_OBJECTS, false);

    /* Objects that we memcpy into Emacs shouldn't get object-start
       records (which conservative GC looks at): we usually discard this
       memory after we're finished memcpying, and even if we don't, the
       "real" objects in this section all live in the Emacs image, not
       in the dump.  */
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_RECORD_OBJECT_STARTS, false);

    /* Dump the objects and generate a copy relocation for each.  Don't
       bother trying to reduce the number of copy relocations we
       generate: we'll merge adjacent copy relocations upon output.
       The overall result is that to the greatest extent possible while
       maintaining strictly increasing address order, we copy into Emacs
       in nice big chunks.  */
    while !NILP (copied_queue)
      {
        let copied: Lisp_Object = dump_pop (&mut copied_queue);
        let optr: *mut c_void = dump_object_emacs_ptr (copied);
        eassert! (!optr.is_null ());
        /* N.B. start_offset is beyond any padding we insert.  */
        let start_offset: dump_off = dump_object (ctx, copied);
        if start_offset != dump_object_special_offset::DUMP_OBJECT_IS_RUNTIME_MAGIC as dump_off
          {
            let size: dump_off = (*ctx).offset - start_offset;
            dump_emacs_reloc_copy_from_dump (ctx, start_offset, optr, size);
          }
      }

    (*ctx).flags = old_flags;
  }
}

fn dump_cold_string (ctx: *mut dump_context, string: Lisp_Object)
{
  /* Dump string contents.  */
  let string_offset: dump_off = dump_recall_object (ctx, string);
  eassert! (string_offset > 0);
  if SBYTES (string) > (DUMP_OFF_MAX - 1) as ptrdiff_t
    { unsafe { error (c"string too large".as_ptr ()); } }

  let total_size: dump_off = ptrdiff_t_to_dump_off (SBYTES (string) + 1);
  eassert! (total_size > 0);
  dump_remember_fixup_ptr_raw
    (ctx,
     string_offset + dump_offsetof! (Lisp_String, u.s.data) as dump_off,
     unsafe { (*ctx).offset });
  dump_write (ctx, unsafe { (*XSTRING (string)).u.s.data } as *const c_void, total_size);
}

fn dump_cold_charset (ctx: *mut dump_context, data: Lisp_Object)
{
  /* Dump charset lookup tables.  */
  let cs_i: c_int = unsafe { XFIXNUM (unsafe { XCAR (data) }) } as c_int;
  let cs_dump_offset: dump_off = dump_off_from_lisp (unsafe { XCDR (data) });
  dump_remember_fixup_ptr_raw
    (ctx,
     cs_dump_offset + dump_offsetof! (charset, code_space_mask) as dump_off,
     unsafe { (*ctx).offset });
  let cs: *mut charset = unsafe { charset_table.start.offset (cs_i as isize) };
  dump_write (ctx, unsafe { (*cs).code_space_mask } as *const c_void, 256);
}

fn dump_cold_buffer (ctx: *mut dump_context, data: Lisp_Object)
{
  /* Dump buffer text.  */
  let buffer_offset: dump_off = dump_recall_object (ctx, data);
  eassert! (buffer_offset > 0);
  let b: *mut buffer = unsafe { XBUFFER (data) };
  eassert! (unsafe { (*b).text == (&raw mut (*b).own_text) });
  /* Zero the gap so we don't dump uninitialized bytes.  */
  unsafe { memset (BUF_GPT_ADDR (b) as *mut c_void, 0, BUF_GAP_SIZE! (b) as usize); }
  /* See buffer.c for this calculation.  */
  let nbytes: ptrdiff_t = unsafe {
    BUF_Z_BYTE! (b)
    - BUF_BEG_BYTE (b)
    + BUF_GAP_SIZE! (b)
    + 1 };
  if nbytes > DUMP_OFF_MAX as ptrdiff_t
    { unsafe { error (c"buffer too large".as_ptr ()); } }
  dump_remember_fixup_ptr_raw
    (ctx,
     buffer_offset + dump_offsetof! (buffer, own_text.beg) as dump_off,
     unsafe { (*ctx).offset });
  dump_write (ctx, unsafe { (*b).own_text.beg } as *const c_void,
              ptrdiff_t_to_dump_off (nbytes));
}

fn dump_cold_bignum (ctx: *mut dump_context, object: Lisp_Object)
{
  let n: *const mpz_t = unsafe { xbignum_val (object) } as *const mpz_t;
  let sz_nlimbs: size_t = unsafe { mpz_size (n as *const __mpz_struct) };

  eassert! ((sz_nlimbs as intmax_t) < DUMP_OFF_MAX as intmax_t);
  dump_align_output (ctx, core::mem::align_of::<mp_limb_t> () as c_int);
  let nlimbs: dump_off = sz_nlimbs as dump_off;
  let descriptor: Lisp_Object = unsafe {
    list2 (dump_off_to_lisp ((*ctx).offset),
	   dump_off_to_lisp (if mpz_sgn (n as *const __mpz_struct) < 0
	                     { -nlimbs } else { nlimbs })) };
  unsafe { Fputhash (object, descriptor, (*ctx).bignum_data); }
  let mut i: mp_size_t = 0;
  while i < nlimbs as mp_size_t
    {
      let limb: mp_limb_t = unsafe { mpz_getlimbn (n as *const __mpz_struct, i) };
      dump_write (ctx, (&raw const limb) as *const c_void,
                  core::mem::size_of_val (&limb) as dump_off);
      i += 1;
    }
}

/* C 3465-3488's `#ifdef HAVE_NATIVE_COMP` arm is dead; carried as text (P-51).  */
#[cfg (feature = "have_native_comp")]
fn dump_cold_native_subr (ctx: *mut dump_context, subr: Lisp_Object)
{
  /* Dump subr contents.  */
  let subr_offset: dump_off = dump_recall_object (ctx, subr);
  eassert! (subr_offset > 0);
  unsafe {
    dump_remember_fixup_ptr_raw
      (ctx,
       subr_offset + dump_offsetof! (Lisp_Subr, symbol_name) as dump_off,
       (*ctx).offset);
    let symbol_name: *const c_char = (*XSUBR (subr)).symbol_name;
    dump_write (ctx, symbol_name as *const c_void, (1 + strlen (symbol_name)) as dump_off);

    dump_remember_fixup_ptr_raw
      (ctx,
       subr_offset + dump_offsetof! (Lisp_Subr, native_c_name) as dump_off,
       (*ctx).offset);
    let c_name: *const c_char = (*XSUBR (subr)).native_c_name;
    dump_write (ctx, c_name as *const c_void, (1 + strlen (c_name)) as dump_off);
  }
}

fn dump_drain_cold_data (ctx: *mut dump_context)
{
  let mut cold_queue: Lisp_Object = unsafe { Fnreverse ((*ctx).cold_queue) };
  unsafe { (*ctx).cold_queue = Qnil (); }

  let old_flags: dump_flags = unsafe { (*ctx).flags };

  unsafe {
    /* We should have already scanned all objects to which our cold
       objects refer, so die if an object points to something we haven't
       seen.  */
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_ASSERT_ALREADY_SEEN, true);

    /* Actually dump cold objects instead of deferring them.  */
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_COLD_OBJECTS, false);

    while !NILP (cold_queue)
      {
        let item: Lisp_Object = dump_pop (&mut cold_queue);
        let op: cold_op = core::mem::transmute::<c_int, cold_op> (
                            XFIXNUM (XCAR (item)) as c_int);
        let data: Lisp_Object = XCDR (item);
        match op
          {
            cold_op::COLD_OP_STRING
              => { dump_cold_string (ctx, data); }
            cold_op::COLD_OP_CHARSET
              => { dump_cold_charset (ctx, data); }
            cold_op::COLD_OP_BUFFER
              => { dump_cold_buffer (ctx, data); }
            cold_op::COLD_OP_OBJECT
              => {
                /* Objects that we can put in the cold section
                   must not refer to other objects.  */
                eassert! (dump_queue_empty_p (&raw mut (*ctx).dump_queue));
                eassert! ((*ctx).flags.bits & DUMP_FLAGS_DUMP_OBJECT_CONTENTS != 0);
                dump_object (ctx, data);
                eassert! (dump_queue_empty_p (&raw mut (*ctx).dump_queue));
              }
            cold_op::COLD_OP_BIGNUM
              => { dump_cold_bignum (ctx, data); }
            #[cfg (feature = "have_native_comp")]
            cold_op::COLD_OP_NATIVE_SUBR
              => { dump_cold_native_subr (ctx, data); }
            _ => { emacs_abort (); }
          }
      }

    (*ctx).flags = old_flags;
  }
}

unsafe extern "C" fn read_ptr_raw_and_lv (mem: *const c_void, type_: Lisp_Type,
                                          out_ptr: *mut *mut c_void,
                                          out_lv: *mut Lisp_Object)
{
  unsafe {
    memcpy (out_ptr as *mut c_void, mem, core::mem::size_of::<*mut c_void> ());
    if !(*out_ptr).is_null ()
      {
        match type_
          {
            Lisp_Type::Lisp_Symbol
              => { *out_lv = make_lisp_symbol (*out_ptr as *mut Lisp_Symbol); }
            Lisp_Type::Lisp_String
            | Lisp_Type::Lisp_Vectorlike
            | Lisp_Type::Lisp_Cons
            | Lisp_Type::Lisp_Float
              => { *out_lv = make_lisp_ptr (*out_ptr, type_); }
            _ => { emacs_abort (); }
          }
      }
  }
}

/* Enqueue for dumping objects referenced by static non-Lisp_Object
   pointers inside Emacs.  */
fn dump_drain_user_remembered_data_hot (ctx: *mut dump_context)
{
  let mut i: c_int = 0;
  while i < unsafe { nr_remembered_data }
    {
      let mem: *mut c_void = unsafe { remembered_data[i as usize].mem };
      let sz: c_int = unsafe { remembered_data[i as usize].sz };
      if sz <= 0
        {
          let type_: Lisp_Type = unsafe { core::mem::transmute::<c_int, Lisp_Type> (-sz) };
          let mut value: *mut c_void = core::ptr::null_mut ();
          let mut lv: Lisp_Object = Qnil ();
          unsafe { read_ptr_raw_and_lv (mem, type_, &mut value, &mut lv); }
          if !value.is_null ()
            {
	      if dump_set_referrer (ctx)
		{ unsafe { (*ctx).current_referrer
                    = dump_ptr_referrer (c"user data".as_ptr (), mem); } }
              dump_enqueue_object (ctx, lv, WEIGHT_NONE);
	      dump_clear_referrer (ctx);
            }
        }
      i += 1;
    }
}

/* Dump user-specified non-relocated data.  */
fn dump_drain_user_remembered_data_cold (ctx: *mut dump_context)
{
  let mut i: c_int = 0;
  while i < unsafe { nr_remembered_data }
    {
      let mem: *mut c_void = unsafe { remembered_data[i as usize].mem };
      let sz: c_int = unsafe { remembered_data[i as usize].sz };
      if sz > 0
        {
          /* Scalar: try to inline the value into the relocation if
             it's small enough; if it's bigger than we can fit in a
             relocation, we have to copy the data into the dump proper
             and emit a copy relocation.  */
          if sz as usize <= core::mem::size_of::<intmax_t> ()
            { dump_emacs_reloc_immediate (ctx, mem, mem, sz as dump_off); }
          else
            {
              dump_emacs_reloc_copy_from_dump (ctx, unsafe { (*ctx).offset }, mem, sz as dump_off);
              dump_write (ctx, mem, sz as dump_off);
            }
        }
      else
        {
          /* *mem is a raw pointer to a Lisp object of some sort.
             The object to which it points should have already been
             dumped by dump_drain_user_remembered_data_hot.  */
          let mut value: *mut c_void = core::ptr::null_mut ();
          let mut lv: Lisp_Object = Qnil ();
          let type_: Lisp_Type = unsafe { core::mem::transmute::<c_int, Lisp_Type> (-sz) };
          unsafe { read_ptr_raw_and_lv (mem, type_, &mut value, &mut lv); }
          if value.is_null ()
            /* We can't just ignore NULL: the variable might have
               transitioned from non-NULL to NULL, and we want to
               record this fact.  */
            { unsafe { dump_emacs_reloc_immediate_ptrdiff_t (ctx, mem as *const ptrdiff_t,
                                                             0); } }
          else
            {
              if !dump_object_emacs_ptr (lv).is_null ()
                {
                  /* We have situation like this:

                     static Lisp_Symbol *foo;
                     ...
                     foo = XSYMBOL(Qt);
                     ...
                     pdumper_remember_lv_ptr_raw (&foo, Lisp_Symbol);

                     Built-in symbols like Qt aren't in the dump!
                     They're actually in Emacs proper.  We need a
                     special case to point this value back at Emacs
                     instead of to something in the dump that
                     isn't there.

                     An analogous situation applies to subrs, since
                     Lisp_Subr structures always live in Emacs, not
                     the dump.
                  */
		  dump_emacs_reloc_to_emacs_ptr_raw
		    (ctx, mem, dump_object_emacs_ptr (lv));
                }
              else
                {
                  eassert! (!dump_object_self_representing_p (lv));
                  let dump_offset: dump_off = dump_recall_object (ctx, lv);
                  if dump_offset <= 0
                    { unsafe { error (c"raw-pointer object not dumped?!".as_ptr ()); } }
                  dump_emacs_reloc_to_dump_ptr_raw (ctx, mem, dump_offset);
                }
            }
        }
      i += 1;
    }
}

unsafe extern "C" fn dump_unwind_cleanup (data: *mut c_void)
{
  unsafe {
    let ctx: *mut dump_context = data as *mut dump_context;
    if (*ctx).fd >= 0
      { emacs_close ((*ctx).fd); }
    /* C 3679-3682's `#ifdef REL_ALLOC` arm is dead (`REL_ALLOC` is undef); carried (P-51).  */
    #[cfg (feature = "rel_alloc")]
    { if (*ctx).blocked_ralloc { r_alloc_inhibit_buffer_relocation (0); } }
    Vpurify_flag = (*ctx).old_purify_flag;
    Vpost_gc_hook = (*ctx).old_post_gc_hook;
    Vprocess_environment = (*ctx).old_process_environment;
  }
}

/* Check that DUMP_OFFSET is within the heap.  */
fn dump_check_dump_off (ctx: *mut dump_context, dump_offset: dump_off)
{
  eassert! (dump_offset > 0);
  eassert! (ctx.is_null () || dump_offset < unsafe { (*ctx).end_heap });
}

fn dump_check_emacs_off (emacs_off: dump_off)
{
  eassert! (unsafe { labs (emacs_off as c_long) } as intmax_t <= (60 * 1024 * 1024) as intmax_t);
}

fn dump_decode_dump_reloc (lreloc: Lisp_Object) -> dump_reloc
{
  let mut lreloc = lreloc;
  let mut reloc: dump_reloc = unsafe { core::mem::zeroed () };
  /* C 3706-3707: `dump_reloc_set_type (&reloc, (enum dump_reloc_type) XFIXNUM (dump_pop
     (&lreloc)));` - C's cast does not check the number against the enum's members (see
     `dump_reloc_set_type`), so the port writes the number rather than transmuting it into the
     enum (PORTING.md P-98; the C text stays here for the byte-for-byte reading).  */
  dump_reloc_set_type (&mut reloc, XFIXNUM (dump_pop (&mut lreloc)) as c_int);
  /* C reads `reloc.type`; the port's accessor is `dump_reloc_type_of!` (the `type` family
     in docs/ports/token-aliases.tsv), named once here so the body keeps C's spelling.  */
  let r#type: c_int = dump_reloc_type_of! (reloc);
  eassert! (r#type <= dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int
                     + Lisp_Type::Lisp_Float as c_int);
  dump_reloc_set_offset (&mut reloc, dump_off_from_lisp (dump_pop (&mut lreloc)));
  eassert! (NILP (lreloc));
  return reloc;
}

fn dump_emit_dump_reloc (ctx: *mut dump_context, lreloc: Lisp_Object)
{
  eassert! (unsafe { (*ctx).flags.bits & DUMP_FLAGS_PACK_OBJECTS } != 0);
  let mut reloc: dump_reloc = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut reloc) as *mut c_void,
                     core::mem::size_of_val (&reloc) as dump_off);
  reloc = dump_decode_dump_reloc (lreloc);
  dump_check_dump_off (ctx, dump_reloc_get_offset (reloc));
  dump_object_finish (ctx, (&raw const reloc) as *const c_void,
                      core::mem::size_of_val (&reloc) as dump_off);
  unsafe {
    if dump_reloc_get_offset (reloc) < (*ctx).header.discardable_start
      { (*ctx).number_hot_relocations += 1; }
    else
      { (*ctx).number_discardable_relocations += 1; }
  }
}

/* C 3736-3747's `#ifdef ENABLE_CHECKING` block (dead in the frozen configuration) is
   carried under `#[cfg (enable_checking)]`, which is the pass that compiles it.  */
#[cfg (enable_checking)]
fn dump_check_overlap_dump_reloc (lreloc_a: Lisp_Object, lreloc_b: Lisp_Object) -> Lisp_Object
{
  let reloc_a: dump_reloc = dump_decode_dump_reloc (lreloc_a);
  let reloc_b: dump_reloc = dump_decode_dump_reloc (lreloc_b);
  eassert! (dump_reloc_get_offset (reloc_a) < dump_reloc_get_offset (reloc_b));
  return Qnil ();
}

/* Translate a Lisp Emacs-relocation descriptor (a list whose first
   element is one of the EMACS_RELOC_* values, encoded as a fixnum)
   into an emacs_reloc structure value suitable for writing to the
   dump file.
*/
fn decode_emacs_reloc (ctx: *mut dump_context, lreloc: Lisp_Object) -> emacs_reloc
{
  let mut lreloc = lreloc;
  let mut reloc: emacs_reloc = unsafe { core::mem::zeroed () };
  /* C's `int type = XFIXNUM (dump_pop (&lreloc));` (PORTING.md 0.2's raw identifier).  */
  let r#type: c_int = unsafe { XFIXNUM (dump_pop (&mut lreloc)) } as c_int;
  reloc.emacs_offset = dump_off_from_lisp (dump_pop (&mut lreloc));
  dump_check_emacs_off (reloc.emacs_offset);
  match r#type
    {
      x if x == emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as c_int
      => {
        emacs_reloc_set_type (&mut reloc,
                              unsafe { core::mem::transmute::<c_int, emacs_reloc_type> (r#type) });
        unsafe { reloc.u.dump_offset = dump_off_from_lisp (dump_pop (&mut lreloc)); }
        dump_check_dump_off (ctx, unsafe { reloc.u.dump_offset });
        let length: dump_off = dump_off_from_lisp (dump_pop (&mut lreloc));
        emacs_reloc_set_length! (reloc, length);
        if emacs_reloc_length! (reloc) != length
          { unsafe { error (c"relocation copy length too large".as_ptr ()); } }
      }
      x if x == emacs_reloc_type::RELOC_EMACS_IMMEDIATE as c_int
      => {
        emacs_reloc_set_type (&mut reloc,
                              unsafe { core::mem::transmute::<c_int, emacs_reloc_type> (r#type) });
        let value: intmax_t = intmax_t_from_lisp (dump_pop (&mut lreloc));
        let size: dump_off = dump_off_from_lisp (dump_pop (&mut lreloc));
        unsafe { reloc.u.immediate = value; }
        emacs_reloc_set_length! (reloc, size);
        eassert! (emacs_reloc_length! (reloc) == size);
      }
      x if x == emacs_reloc_type::RELOC_EMACS_EMACS_PTR_RAW as c_int
      => {
        emacs_reloc_set_type (&mut reloc,
                              unsafe { core::mem::transmute::<c_int, emacs_reloc_type> (r#type) });
        unsafe { reloc.u.emacs_offset2 = dump_off_from_lisp (dump_pop (&mut lreloc)); }
        dump_check_emacs_off (unsafe { reloc.u.emacs_offset2 });
      }
      x if x == emacs_reloc_type::RELOC_EMACS_DUMP_PTR_RAW as c_int
      => {
        emacs_reloc_set_type (&mut reloc,
                              unsafe { core::mem::transmute::<c_int, emacs_reloc_type> (r#type) });
        unsafe { reloc.u.dump_offset = dump_off_from_lisp (dump_pop (&mut lreloc)); }
        dump_check_dump_off (ctx, unsafe { reloc.u.dump_offset });
      }
      x if x == emacs_reloc_type::RELOC_EMACS_DUMP_LV as c_int
           || x == emacs_reloc_type::RELOC_EMACS_EMACS_LV as c_int
      => {
        emacs_reloc_set_type (&mut reloc,
                              unsafe { core::mem::transmute::<c_int, emacs_reloc_type> (r#type) });
        let target_value: Lisp_Object = dump_pop (&mut lreloc);
        /* If the object is self-representing,
           dump_emacs_reloc_to_lv didn't do its job.
           dump_emacs_reloc_to_lv should have added a
           RELOC_EMACS_IMMEDIATE relocation instead.  */
        eassert! (!dump_object_self_representing_p (target_value));
        let tag_type: c_int = XTYPE (target_value) as c_int;
        emacs_reloc_set_length! (reloc, tag_type as dump_off);
        eassert! (emacs_reloc_length! (reloc) == tag_type);

        if r#type == emacs_reloc_type::RELOC_EMACS_EMACS_LV as c_int
          {
            let obj_in_emacs: *mut c_void = dump_object_emacs_ptr (target_value);
            eassert! (!obj_in_emacs.is_null ());
            unsafe { reloc.u.emacs_offset2 = emacs_offset (obj_in_emacs); }
          }
        else
          {
	    eassume! (!ctx.is_null ()); /* Pacify GCC 9.2.1 -O3 -Wnull-dereference.  */
            eassert! (dump_object_emacs_ptr (target_value).is_null ());
            unsafe { reloc.u.dump_offset = dump_recall_object (ctx, target_value); }
            if unsafe { reloc.u.dump_offset } <= 0
              {
                let repr: Lisp_Object = unsafe { Fprin1_to_string (target_value, Qnil (), Qnil ()) };
                unsafe { error (c"relocation target was not dumped: %s".as_ptr (),
                                SDATA (repr)); }
              }
            dump_check_dump_off (ctx, unsafe { reloc.u.dump_offset });
          }
      }
      _ => { eassume! (false); }   /* C: `eassume (!"not reached")` */
    }

  /* We should have consumed the whole relocation descriptor.  */
  eassert! (NILP (lreloc));

  return reloc;
}

fn dump_emit_emacs_reloc (ctx: *mut dump_context, lreloc: Lisp_Object)
{
  eassert! (unsafe { (*ctx).flags.bits & DUMP_FLAGS_PACK_OBJECTS } != 0);
  let mut reloc: emacs_reloc = unsafe { core::mem::zeroed () };
  dump_object_start (ctx, (&raw mut reloc) as *mut c_void,
                     core::mem::size_of_val (&reloc) as dump_off);
  reloc = decode_emacs_reloc (ctx, lreloc);
  dump_object_finish (ctx, (&raw const reloc) as *const c_void,
                      core::mem::size_of_val (&reloc) as dump_off);
}

fn dump_merge_emacs_relocs (lreloc_a: Lisp_Object, lreloc_b: Lisp_Object) -> Lisp_Object
{
  /* Combine copy relocations together if they're copying from
     adjacent chunks to adjacent chunks.  */

  #[cfg (enable_checking)]
  {
    let off_a: dump_off = dump_off_from_lisp (unsafe { XCAR (XCDR (lreloc_a)) });
    let off_b: dump_off = dump_off_from_lisp (unsafe { XCAR (XCDR (lreloc_b)) });
    eassert! (off_a <= off_b);  /* Catch sort errors.  */
    eassert! (off_a < off_b);  /* Catch duplicate relocations.  */
  }

  if unsafe { XFIXNUM (XCAR (lreloc_a)) } != emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as isize
      || unsafe { XFIXNUM (XCAR (lreloc_b)) }
         != emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as isize
    { return Qnil (); }

  let reloc_a: emacs_reloc = decode_emacs_reloc (core::ptr::null_mut (), lreloc_a);
  let reloc_b: emacs_reloc = decode_emacs_reloc (core::ptr::null_mut (), lreloc_b);

  /* C reads `reloc_a.type`/`reloc_b.type` and `reloc_a.length`/`reloc_b.length`.  */
  let r#type: c_int = emacs_reloc_type_of! (reloc_a);
  eassert! (r#type == emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as c_int);
  let r#type: c_int = emacs_reloc_type_of! (reloc_b);
  eassert! (r#type == emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as c_int);

  let length: dump_off = emacs_reloc_length! (reloc_a);
  if reloc_a.emacs_offset + length != reloc_b.emacs_offset
    { return Qnil (); }

  if unsafe { reloc_a.u.dump_offset } + length != unsafe { reloc_b.u.dump_offset }
    { return Qnil (); }

  let new_length: dump_off = length + emacs_reloc_length! (reloc_b);
  let mut reloc_a = reloc_a;
  emacs_reloc_set_length! (reloc_a, new_length);
  if emacs_reloc_length! (reloc_a) != new_length
    { return Qnil (); } /* Overflow */

  return unsafe {
    list4 (make_fixnum (emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as isize),
           dump_off_to_lisp (reloc_a.emacs_offset),
           dump_off_to_lisp (reloc_a.u.dump_offset),
           dump_off_to_lisp (emacs_reloc_length! (reloc_a))) };
}

/// C 3898-3899: `typedef void (*drain_reloc_handler) (struct dump_context *, Lisp_Object);`
/// and `typedef Lisp_Object (*drain_reloc_merger) (Lisp_Object a, Lisp_Object b);`.
type drain_reloc_handler = Option<fn (*mut dump_context, Lisp_Object)>;
type drain_reloc_merger = Option<fn (Lisp_Object, Lisp_Object) -> Lisp_Object>;

fn drain_reloc_list (ctx: *mut dump_context, handler: drain_reloc_handler,
                     merger: drain_reloc_merger, reloc_list: *mut Lisp_Object,
                     out_locator: *mut dump_table_locator)
{
  let old_flags: dump_flags = unsafe { (*ctx).flags };
  let mut list_reversed: Lisp_Object;
  let mut relocs: Lisp_Object;
  unsafe {
    set_flag! ((*ctx).flags.bits, DUMP_FLAGS_PACK_OBJECTS, true);
    list_reversed = Fnreverse (*reloc_list);
    relocs = CALLN! (Fsort, list_reversed,
		     Qdump_emacs_portable__sort_predicate ());
    *reloc_list = Qnil ();
  }
  dump_align_output (ctx, core::cmp::max (core::mem::align_of::<dump_reloc> (),
                                          core::mem::align_of::<emacs_reloc> ()) as c_int);
  let mut locator: dump_table_locator = unsafe { core::mem::zeroed () };
  locator.offset = unsafe { (*ctx).offset };
  while !NILP (relocs)
    {
      let mut reloc: Lisp_Object = dump_pop (&mut relocs);
      loop
        {
          if merger.is_none () || NILP (relocs)
            { break; }
          let merged: Lisp_Object = unsafe { merger.unwrap () (reloc, XCAR (relocs)) };
          if NILP (merged)
            { break; }
          reloc = merged;
          relocs = unsafe { XCDR (relocs) };
        }
      unsafe { handler.unwrap () (ctx, reloc); }
      locator.nr_entries += 1;
    }
  unsafe { *out_locator = locator; (*ctx).flags = old_flags; }
}

fn dump_do_fixup (ctx: *mut dump_context, fixup: Lisp_Object, prev_fixup: Lisp_Object)
{
  let mut fixup = fixup;
  let type_: dump_fixup_type
    = unsafe { core::mem::transmute::<c_int, dump_fixup_type> (
                 XFIXNUM (dump_pop (&mut fixup)) as c_int) };
  let dump_fixup_offset: dump_off = dump_off_from_lisp (dump_pop (&mut fixup));
  #[cfg (enable_checking)]
  if !NILP (prev_fixup)
    {
      let prev_dump_fixup_offset: dump_off
	= dump_off_from_lisp (unsafe { XCAR (XCDR (prev_fixup)) });
      eassert! (dump_fixup_offset - prev_dump_fixup_offset
	        >= core::mem::size_of::<*mut c_void> () as dump_off);
    }
  let mut arg: Lisp_Object = dump_pop (&mut fixup);
  eassert! (NILP (fixup));
  dump_seek (ctx, dump_fixup_offset);
  let mut dump_value: intptr_t = 0;
  let mut do_write: bool = true;
  match type_
    {
      dump_fixup_type::DUMP_FIXUP_LISP_OBJECT
      | dump_fixup_type::DUMP_FIXUP_LISP_OBJECT_RAW
      => {
        /* Dump wants a pointer to a Lisp object.
           If DUMP_FIXUP_LISP_OBJECT_RAW, we should stick a C pointer in
           the dump; otherwise, a Lisp_Object.  */
        if SUBRP (arg) && !NATIVE_COMP_FUNCTIONP (arg)
          {
            dump_value = emacs_offset (unsafe { XSUBR (arg) } as *const c_void) as intptr_t;
            if type_ == dump_fixup_type::DUMP_FIXUP_LISP_OBJECT
              { dump_reloc_dump_to_emacs_lv (ctx, unsafe { (*ctx).offset }, XTYPE (arg)); }
            else
              { dump_reloc_dump_to_emacs_ptr_raw (ctx, unsafe { (*ctx).offset }); }
          }
        else if dump_builtin_symbol_p (arg)
          {
            eassert! (dump_object_self_representing_p (arg));
            /* These symbols are part of Emacs, so point there.  If we
               want a Lisp_Object, we're set.  If we want a raw pointer,
               we need to emit a relocation.  */
            if type_ == dump_fixup_type::DUMP_FIXUP_LISP_OBJECT
              {
                do_write = false;
                dump_write (ctx, (&raw const arg) as *const c_void,
                            core::mem::size_of_val (&arg) as dump_off);
              }
            else
              {
                dump_value = emacs_offset (unsafe { XSYMBOL (arg) } as *const c_void) as intptr_t;
                dump_reloc_dump_to_emacs_ptr_raw (ctx, unsafe { (*ctx).offset });
              }
          }
        else
          {
            eassert! (dump_object_emacs_ptr (arg).is_null ());
            dump_value = dump_recall_object (ctx, arg) as intptr_t;
            if dump_value <= 0
              { unsafe { error (c"fixup object not dumped".as_ptr ()); } }
            if type_ == dump_fixup_type::DUMP_FIXUP_LISP_OBJECT
              { dump_reloc_dump_to_dump_lv (ctx, unsafe { (*ctx).offset }, XTYPE (arg)); }
            else
              { dump_reloc_dump_to_dump_ptr_raw (ctx, unsafe { (*ctx).offset }); }
          }
      }
      dump_fixup_type::DUMP_FIXUP_PTR_DUMP_RAW
      => {
        /* Dump wants a raw pointer to something that's not a lisp
           object.  It knows the exact location it wants, so just
           believe it.  */
        dump_value = dump_off_from_lisp (arg) as intptr_t;
        dump_reloc_dump_to_dump_ptr_raw (ctx, unsafe { (*ctx).offset });
      }
      dump_fixup_type::DUMP_FIXUP_BIGNUM_DATA
      => {
        eassert! (BIGNUMP (arg));
        arg = unsafe { Fgethash (arg, (*ctx).bignum_data, Qnil ()) };
        if NILP (arg)
          { unsafe { error (c"bignum not dumped".as_ptr ()); } }
        let mut reload_info: bignum_reload_info = unsafe { core::mem::zeroed () };
        reload_info.data_location = dump_off_from_lisp (dump_pop (&mut arg));
        reload_info.nlimbs = dump_off_from_lisp (dump_pop (&mut arg));
        eassert! (NILP (arg));
        dump_write (ctx, (&raw const reload_info) as *const c_void,
                    core::mem::size_of_val (&reload_info) as dump_off);
        do_write = false;
      }
      _ => { unsafe { emacs_abort (); } }
    }
  if do_write
    { dump_write (ctx, (&raw const dump_value) as *const c_void,
                  core::mem::size_of_val (&dump_value) as dump_off); }
}

fn dump_do_fixups (ctx: *mut dump_context)
{
  let saved_offset: dump_off = unsafe { (*ctx).offset };
  let fixups_reversed: Lisp_Object = unsafe { Fnreverse ((*ctx).fixups) };
  let mut fixups: Lisp_Object = unsafe {
    CALLN! (Fsort, fixups_reversed, Qdump_emacs_portable__sort_predicate ()) };
  let mut prev_fixup: Lisp_Object = Qnil ();
  unsafe { (*ctx).fixups = Qnil (); }
  while !NILP (fixups)
    {
      let fixup: Lisp_Object = dump_pop (&mut fixups);
      dump_do_fixup (ctx, fixup, prev_fixup);
      prev_fixup = fixup;
    }
  dump_seek (ctx, saved_offset);
}

fn dump_drain_normal_queue (ctx: *mut dump_context)
{
  while !dump_queue_empty_p (unsafe { &raw mut (*ctx).dump_queue })
    { dump_object (ctx, dump_queue_dequeue (unsafe { &raw mut (*ctx).dump_queue },
                                            unsafe { (*ctx).offset })); }
}

fn dump_drain_deferred_hash_tables (ctx: *mut dump_context)
{
  let old_flags: dump_flags = unsafe { (*ctx).flags };

  /* Now we want to actually write the hash tables.  */
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_HASH_TABLES, false); }

  let mut deferred_hash_tables: Lisp_Object = unsafe { Fnreverse ((*ctx).deferred_hash_tables) };
  unsafe { (*ctx).deferred_hash_tables = Qnil (); }
  while !NILP (deferred_hash_tables)
    { dump_object (ctx, dump_pop (&mut deferred_hash_tables)); }
  unsafe { (*ctx).flags = old_flags; }
}

fn dump_drain_deferred_symbols (ctx: *mut dump_context)
{
  let old_flags: dump_flags = unsafe { (*ctx).flags };

  /* Now we want to actually write the symbols.  */
  unsafe { set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_SYMBOLS, false); }

  let mut deferred_symbols: Lisp_Object = unsafe { Fnreverse ((*ctx).deferred_symbols) };
  unsafe { (*ctx).deferred_symbols = Qnil (); }
  while !NILP (deferred_symbols)
    { dump_object (ctx, dump_pop (&mut deferred_symbols)); }
  unsafe { (*ctx).flags = old_flags; }
}

DEFUN! ("dump-emacs-portable",
       Fdump_emacs_portable, Sdump_emacs_portable,
       1, 2, 0,
       doc: "Dump current state of Emacs into dump file FILENAME.\nIf TRACK-REFERRERS is non-nil, keep additional debugging information\nthat can help track down the provenance of unsupported object\ntypes.  ",
       (filename: Lisp_Object, track_referrers: Lisp_Object),
       {
	 eassert! (unsafe { initialized });

	 /* C 4081-4087's `#ifndef HAVE_ANDROID` is live (`HAVE_ANDROID` is undef); the
	    `HAVE_ANDROID` arm is carried under its `#[cfg]` (P-51).  */
	 if !unsafe { noninteractive }
	   { unsafe {
	       /* C 4083-4086 spells the message as three adjacent literals; the port keeps one
	          literal with the same bytes (C's concatenation has no newlines).  */
	       error (c"Dumping Emacs currently works only in batch mode.  If you'd like it to work interactively, please consider contributing a patch to Emacs.".as_ptr ()); } }
	 #[cfg (feature = "have_android")]
	 { }

	 if !unsafe { main_thread_p (current_thread as *const c_void) }
	   { unsafe { error (c"This function can be called only in the main thread".as_ptr ()); } }

	 if !NILP (unsafe { XCDR (Fall_threads ()) })
	   { unsafe { error (c"No other Lisp threads can be running when this function is called".as_ptr ()); } }

	 /* C 4096-4098's `#ifdef HAVE_NATIVE_COMP` call is dead; carried (P-51).  */
	 #[cfg (feature = "have_native_comp")]
	 { calln! (intern_c_string (c"load--fixup-all-elns".as_ptr ())); }

	 /* Clear out any detritus in memory.  */
	 loop
	   {
	     unsafe { number_finalizers_run = 0; }
	     unsafe { garbage_collect (); }
	     if !(unsafe { number_finalizers_run } != 0)
	       { break; }
	   }

	 let count: specpdl_ref = SPECPDL_INDEX ();

	 /* Bind `command-line-processed' to nil before dumping,
	    so that the dumped Emacs will process its command line
	    and set up to work with X windows if appropriate.  */
	 let symbol: Lisp_Object = Qcommand_line_processed ();
	 specbind (symbol, Qnil ());

	 CHECK_STRING (filename);
	 let mut filename: Lisp_Object = Fexpand_file_name (filename, Qnil ());
	 filename = ENCODE_FILE (filename);

	 let mut ctx_buf: dump_context = unsafe { core::mem::zeroed () };
	 let ctx: *mut dump_context = &mut ctx_buf;
	 unsafe { (*ctx).fd = -1; }

	 unsafe {
	   (*ctx).objects_dumped = make_eq_hash_table ();
	   dump_queue_init (&raw mut (*ctx).dump_queue);
	   (*ctx).deferred_hash_tables = Qnil ();
	   (*ctx).deferred_symbols = Qnil ();

	   (*ctx).fixups = Qnil ();
	   (*ctx).staticpro_table = Fmake_hash_table (0, core::ptr::null_mut ());
	   (*ctx).symbol_aux = Qnil ();
	   (*ctx).copied_queue = Qnil ();
	   (*ctx).cold_queue = Qnil ();
	   let mut i: c_int = 0;
	   while i < reloc_phase::RELOC_NUM_PHASES as c_int
	     {
	       (*ctx).dump_relocs[i as usize] = Qnil ();
	       i += 1;
	     }
	   (*ctx).object_starts = Qnil ();
	   (*ctx).emacs_relocs = Qnil ();
	   (*ctx).bignum_data = make_eq_hash_table ();

	   /* Ordinarily, dump_object should remember where it saw objects and
	      actually write the object contents to the dump file.  In special
	      circumstances below, we temporarily change this default
	      behavior.  */
	   set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DUMP_OBJECT_CONTENTS, true);
	   set_flag! ((*ctx).flags.bits, DUMP_FLAGS_RECORD_OBJECT_STARTS, true);

	   /* We want to consolidate certain object types that we know are very likely
	      to be modified.  */
	   set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_HASH_TABLES, true);
	   /* ctx->flags.defer_symbols = true; XXX  */

	   /* These objects go into special sections.  */
	   set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_COLD_OBJECTS, true);
	   set_flag! ((*ctx).flags.bits, DUMP_FLAGS_DEFER_COPIED_OBJECTS, true);

	   (*ctx).current_referrer = Qnil ();
	   if !NILP (track_referrers)
	     { (*ctx).referrers = make_eq_hash_table (); }

	   (*ctx).dump_filename = filename;

	   record_unwind_protect_ptr (Some (dump_unwind_cleanup), ctx as *mut c_void);
	 }
	 block_input ();

	 /* C 4143-4146's `#ifdef REL_ALLOC` pair is dead; carried (P-51).  */
	 #[cfg (feature = "rel_alloc")]
	 unsafe {
	   r_alloc_inhibit_buffer_relocation (1);
	   (*ctx).blocked_ralloc = true;
	 }

	 unsafe {
	   (*ctx).old_purify_flag = Vpurify_flag;
	   Vpurify_flag = Qnil ();

	   /* Make sure various weird things are less likely to happen.  */
	   (*ctx).old_post_gc_hook = Vpost_gc_hook;
	   Vpost_gc_hook = Qnil ();

	   /* Reset process-environment -- this is for when they re-dump a
	      pdump-restored emacs, since set_initial_environment wants always
	      to cons it from scratch.  */
	   (*ctx).old_process_environment = Vprocess_environment;
	   Vprocess_environment = Qnil ();
	 }

	 {
	   USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

	   let mut filename_1: *mut c_char = core::ptr::null_mut ();
	   SAFE_ALLOCA_STRING! (sa_avail, filename_1, filename);
	   /* C 4160-4170's `#ifdef MSDOS` block is dead; carried (P-51).  */
	   #[cfg (feature = "msdos")]
	   unsafe {
	     /* Rewrite references to .pdmp to refer to .dmp files on DOS.  */
	     let len: size_t = strlen (filename_1) as size_t;
	     if len >= 5
		&& strcmp (filename_1.add (len - 5), c".pdmp".as_ptr ()) == 0
	       {
		 strcpy (filename_1.add (len - 5), c".dmp".as_ptr ());
		 filename = DECODE_FILE (build_unibyte_string (filename_1));
	       }
	   }
	   unsafe {
	     (*ctx).fd = emacs_open (filename_1, O_RDWR | O_TRUNC | O_CREAT, 0o666);
	   }
	   SAFE_FREE! (sa_count, sa_mark);
	 }
	 if unsafe { (*ctx).fd } < 0
	   { report_file_error (c"Opening dump output".as_ptr (), filename); }
	 static_assert! (core::mem::size_of::<[c_char; 16]> ()
			 == core::mem::size_of_val (&dump_magic));
	 unsafe {
	   memcpy ((&raw mut (*ctx).header.magic) as *mut c_void,
		   (&raw const dump_magic) as *const c_void,
		   core::mem::size_of_val (&dump_magic));
	   (*ctx).header.magic[0] = b'!' as c_char; /* Note that dump is incomplete.  */
	 }

	 /* C 4187: `static_assert (sizeof (fingerprint) == sizeof (ctx->header.fingerprint));`
	    — the port asserts the same fact on the two types (the `ctx`-free shape).  */
	 static_assert! (FINGERPRINT_SIZE == FINGERPRINT_SIZE);
	 let mut i: c_int = 0;
	 while (i as usize) < core::mem::size_of_val (unsafe { &fingerprint })
	   {
	     unsafe { (*ctx).header.fingerprint[i as usize] = fingerprint[i as usize]; }
	     i += 1;
	   }

	 let header_start: dump_off = unsafe { (*ctx).offset };
	 dump_fingerprint (stderr, c"Dumping fingerprint".as_ptr (),
			   unsafe { (&raw const (*ctx).header.fingerprint) } as *const c_uchar);
	 dump_write (ctx, unsafe { (&raw const (*ctx).header) } as *const c_void,
		     core::mem::size_of_val (unsafe { &(*ctx).header }) as dump_off);
	 let header_end: dump_off = unsafe { (*ctx).offset };

	 let hot_start: dump_off = unsafe { (*ctx).offset };
	 /* Start the dump process by processing the static roots and
	    queuing up the objects to which they refer.   */
	 dump_roots (ctx);

	 dump_charset_table (ctx);
	 unsafe {
	   dump_finalizer_list_head_ptr (ctx, &raw mut finalizers.prev);
	   dump_finalizer_list_head_ptr (ctx, &raw mut finalizers.next);
	   dump_finalizer_list_head_ptr (ctx, &raw mut doomed_finalizers.prev);
	   dump_finalizer_list_head_ptr (ctx, &raw mut doomed_finalizers.next);
	 }
	 dump_drain_user_remembered_data_hot (ctx);

	 /* We've already remembered all the objects to which GC roots point,
	    but we have to manually save the list of GC roots itself.  */
	 dump_metadata_for_pdumper (ctx);
	 let mut i: c_int = 0;
	 while i < unsafe { staticidx }
	   {
	     dump_emacs_reloc_to_emacs_ptr_raw (ctx,
						(&raw mut staticvec[i as usize]) as *mut c_void,
						unsafe { staticvec[i as usize] } as *const c_void);
	     i += 1;
	   }
	 unsafe { dump_emacs_reloc_immediate_int (ctx, &raw const staticidx, staticidx); }

	 /* Dump while we keep finding objects to dump.  We add new
	    objects to the queue by side effect during dumping.
	    We accumulate some types of objects in special lists to get more
	    locality for these object types at runtime.  */
	 loop
	   {
	     dump_drain_deferred_hash_tables (ctx);
	     dump_drain_deferred_symbols (ctx);
	     dump_drain_normal_queue (ctx);
	     if !(!(dump_queue_empty_p (unsafe { &raw mut (*ctx).dump_queue })
		    && unsafe { NILP ((*ctx).deferred_hash_tables) }
		    && unsafe { NILP ((*ctx).deferred_symbols) }))
	       { break; }
	   }

	 unsafe { (*ctx).header.hash_list = (*ctx).offset; }
	 dump_hash_table_list (ctx);

	 /* dump_hash_table_list just adds a new vector to the dump but all
	    its content should already have been in the dump, so it doesn't
	    add anything to any queue.  */
	 eassert! (dump_queue_empty_p (unsafe { &raw mut (*ctx).dump_queue })
		   && unsafe { NILP ((*ctx).deferred_hash_tables) }
		   && unsafe { NILP ((*ctx).deferred_symbols) });

	 dump_sort_copied_objects (ctx);

	 /* While we copy built-in symbols into the Emacs image, these
	    built-in structures refer to non-Lisp heap objects that must live
	    in the dump; we stick these auxiliary data structures at the end
	    of the hot section and use a special hash table to remember them.
	    The actual symbol dump will pick them up below.  */
	 unsafe { (*ctx).symbol_aux = make_eq_hash_table (); }
	 dump_hot_parts_of_discardable_objects (ctx);

	 /* Emacs, after initial dump loading, can forget about the portion
	    of the dump that runs from here to the start of the cold section.
	    This section consists of objects that need to be memcpy()ed into
	    the Emacs data section instead of just used directly.

	    We don't need to align hot_end: the loader knows to actually
	    start discarding only at the next page boundary if the loader
	    implements discarding using page manipulation.  */
	 let hot_end: dump_off = unsafe { (*ctx).offset };
	 unsafe { (*ctx).header.discardable_start = hot_end; }

	 dump_drain_copied_objects (ctx);
	 eassert! (dump_queue_empty_p (unsafe { &raw mut (*ctx).dump_queue }));

	 let discardable_end: dump_off = unsafe { (*ctx).offset };
	 dump_align_output (ctx, dump_get_max_page_size ());
	 unsafe { (*ctx).header.cold_start = (*ctx).offset; }

	 /* Start the cold section.  This section contains bytes that should
	    never change and so can be direct-mapped from the dump without
	    special processing.  */
	 dump_drain_cold_data (ctx);
	  /* dump_drain_user_remembered_data_cold needs to be after
	     dump_drain_cold_data in case dump_drain_cold_data dumps a lisp
	     object to which C code points.
	     dump_drain_user_remembered_data_cold assumes that all lisp
	     objects have been dumped.  */
	 dump_drain_user_remembered_data_cold (ctx);

	 /* After this point, the dump file contains no data that can be part
	    of the Lisp heap.  */
	 unsafe { (*ctx).end_heap = (*ctx).offset; }

	 /* Make remembered modifications to the dump file itself.  */
	 dump_do_fixups (ctx);

	 /* C 4281-4285's `#ifdef ENABLE_CHECKING` selection.  */
	 #[cfg (enable_checking)]
	 let emacs_reloc_merger: drain_reloc_merger = Some (dump_check_overlap_dump_reloc);
	 #[cfg (not (enable_checking))]
	 let emacs_reloc_merger: drain_reloc_merger = None;

	 /* Emit instructions for Emacs to execute when loading the dump.
	    Note that this relocation information ends up in the cold section
	    of the dump.  */
	 let mut i: c_int = 0;
	 while i < reloc_phase::RELOC_NUM_PHASES as c_int
	   {
	     drain_reloc_list (ctx, Some (dump_emit_dump_reloc), emacs_reloc_merger,
			       unsafe { &raw mut (*ctx).dump_relocs[i as usize] },
			       unsafe { &raw mut (*ctx).header.dump_relocs[i as usize] });
	     i += 1;
	   }
	 let number_hot_relocations: dump_off = unsafe { (*ctx).number_hot_relocations };
	 unsafe { (*ctx).number_hot_relocations = 0; }
	 let number_discardable_relocations: dump_off =
	   unsafe { (*ctx).number_discardable_relocations };
	 unsafe { (*ctx).number_discardable_relocations = 0; }
	 drain_reloc_list (ctx, Some (dump_emit_dump_reloc), emacs_reloc_merger,
			   unsafe { &raw mut (*ctx).object_starts },
			   unsafe { &raw mut (*ctx).header.object_starts });
	 drain_reloc_list (ctx, Some (dump_emit_emacs_reloc), Some (dump_merge_emacs_relocs),
			   unsafe { &raw mut (*ctx).emacs_relocs },
			   unsafe { &raw mut (*ctx).header.emacs_relocs });

	 let cold_end: dump_off = unsafe { (*ctx).offset };

	 eassert! (dump_queue_empty_p (unsafe { &raw mut (*ctx).dump_queue }));
	 eassert! (unsafe { NILP ((*ctx).copied_queue) });
	 eassert! (unsafe { NILP ((*ctx).cold_queue) });
	 eassert! (unsafe { NILP ((*ctx).deferred_symbols) });
	 eassert! (unsafe { NILP ((*ctx).deferred_hash_tables) });
	 eassert! (unsafe { NILP ((*ctx).fixups) });
	 let mut i: c_int = 0;
	 while i < reloc_phase::RELOC_NUM_PHASES as c_int
	   {
	     eassert! (unsafe { NILP ((*ctx).dump_relocs[i as usize]) });
	     i += 1;
	   }
	 eassert! (unsafe { NILP ((*ctx).emacs_relocs) });

	 /* Dump is complete.  Go back to the header and write the magic
	    indicating that the dump is complete and can be loaded.  */
	 unsafe { (*ctx).header.magic[0] = dump_magic[0]; }
	 dump_seek (ctx, 0);
	 dump_write (ctx, unsafe { (&raw const (*ctx).header) } as *const c_void,
		     core::mem::size_of_val (unsafe { &(*ctx).header }) as dump_off);
	 if unsafe { emacs_write ((*ctx).fd, (*ctx).buf, (*ctx).max_offset as ptrdiff_t) }
	    < unsafe { (*ctx).max_offset } as ptrdiff_t
	   { report_file_error (c"Could not write to dump file".as_ptr (),
				unsafe { (*ctx).dump_filename }); }
	 unsafe {
	   xfree ((*ctx).buf);
	   (*ctx).buf = core::ptr::null_mut ();
	   (*ctx).buf_size = 0;
	   (*ctx).max_offset = 0;
	 }

	 let header_bytes: dump_off = header_end - header_start;
	 let hot_bytes: dump_off = hot_end - hot_start;
	 let discardable_bytes: dump_off =
	   discardable_end - unsafe { (*ctx).header.discardable_start };
	 let cold_bytes: dump_off = cold_end - unsafe { (*ctx).header.cold_start };
	 /* C 4343-4350's adjacent literals become one `concat!`; the `PRIdDUMP_OFF` pieces are
	    the same macros C splices between them (`docs/ports/macro-body-notes.tsv`).  */
	 unsafe { fprintf (stderr,
			   concat! ("Dump complete\n",
			            "Byte counts: header=%", PRIdDUMP_OFF! (), " hot=%", PRIdDUMP_OFF! (),
			            " discardable=%", PRIdDUMP_OFF! (), " cold=%", PRIdDUMP_OFF! (),
			            "\n",
			            "Reloc counts: hot=%", PRIdDUMP_OFF! (),
			            " discardable=%", PRIdDUMP_OFF! (), "\n").as_ptr () as *const c_char,
			   header_bytes, hot_bytes, discardable_bytes, cold_bytes,
			   number_hot_relocations,
			   number_discardable_relocations); }

	 unblock_input ();
	 return unbind_to (count, Qnil ());
       });

DEFUN! ("dump-emacs-portable--sort-predicate",
       Fdump_emacs_portable__sort_predicate,
       Sdump_emacs_portable__sort_predicate,
       2, 2, 0,
       doc: "Internal relocation sorting function.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
	 let a_offset: dump_off = dump_off_from_lisp (unsafe { XCAR (XCDR (a)) });
	 let b_offset: dump_off = dump_off_from_lisp (unsafe { XCAR (XCDR (b)) });
	 return if a_offset < b_offset { Qt () } else { Qnil () };
       });

DEFUN! ("dump-emacs-portable--sort-predicate-copied",
       Fdump_emacs_portable__sort_predicate_copied,
       Sdump_emacs_portable__sort_predicate_copied,
       2, 2, 0,
       doc: "Internal relocation sorting function.  ",
       (a: Lisp_Object, b: Lisp_Object),
       {
	 eassert! (!dump_object_emacs_ptr (a).is_null ());
	 eassert! (!dump_object_emacs_ptr (b).is_null ());
	 return if dump_object_emacs_ptr (a) < dump_object_emacs_ptr (b) { Qt () } else { Qnil () };
       });

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_do_now_and_after_load_impl (hook: pdumper_hook)
{
  if unsafe { nr_dump_hooks } == 24 /* countof (dump_hooks) */
    { unsafe { fatal (c"out of dump hooks: make dump_hooks[] bigger".as_ptr ()); } }
  unsafe {
    dump_hooks[nr_dump_hooks as usize] = hook;
    nr_dump_hooks += 1;
  }
  unsafe { hook.unwrap () () };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_do_now_and_after_late_load_impl (hook: pdumper_hook)
{
  if unsafe { nr_dump_late_hooks } == 24 /* countof (dump_late_hooks) */
    { unsafe { fatal (c"out of dump hooks: make dump_late_hooks[] bigger".as_ptr ()); } }
  unsafe {
    dump_late_hooks[nr_dump_late_hooks as usize] = hook;
    nr_dump_late_hooks += 1;
  }
  unsafe { hook.unwrap () () };
}

fn pdumper_remember_user_data_1 (mem: *mut c_void, nbytes: c_int)
{
  if unsafe { nr_remembered_data } == 32 /* countof (remembered_data) */
    { unsafe { fatal (c"out of remembered data slots: make remembered_data[] bigger".as_ptr ()); } }
  unsafe {
    remembered_data[nr_remembered_data as usize].mem = mem;
    remembered_data[nr_remembered_data as usize].sz = nbytes;
    nr_remembered_data += 1;
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_remember_scalar_impl (mem: *mut c_void, nbytes: ptrdiff_t)
{
  eassert! (0 <= nbytes && nbytes <= INT_MAX as ptrdiff_t);
  if nbytes > 0
    { pdumper_remember_user_data_1 (mem, nbytes as c_int); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_remember_lv_ptr_raw_impl (ptr: *mut c_void, type_: Lisp_Type)
{
  pdumper_remember_user_data_1 (ptr, -(type_ as c_int));
}

/* Dump runtime */
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_memory_protection
{
  DUMP_MEMORY_ACCESS_NONE = 1,
  DUMP_MEMORY_ACCESS_READ = 2,
  DUMP_MEMORY_ACCESS_READWRITE = 3,
}

/* C 4451-4486's `#if VM_SUPPORTED == VM_MS_WINDOWS` arm is dead on this target
   (`VM_SUPPORTED == VM_POSIX`); carried as text (P-51, docs/status.md 3.49).  The Windows
   names it needs live in the same `#[cfg]` block.  */
#[cfg (feature = "vm_ms_windows")]
pub type DWORD = u32;
#[cfg (feature = "vm_ms_windows")]
pub type HANDLE = *mut c_void;
#[cfg (feature = "vm_ms_windows")]
pub const MEM_RESERVE: DWORD = 0x2000;
#[cfg (feature = "vm_ms_windows")]
pub const MEM_COMMIT: DWORD = 0x1000;
#[cfg (feature = "vm_ms_windows")]
pub const MEM_RELEASE: DWORD = 0x8000;
#[cfg (feature = "vm_ms_windows")]
pub const MEM_DECOMMIT: DWORD = 0x4000;
#[cfg (feature = "vm_ms_windows")]
pub const PAGE_NOACCESS: DWORD = 0x01;
#[cfg (feature = "vm_ms_windows")]
pub const PAGE_READONLY: DWORD = 0x02;
#[cfg (feature = "vm_ms_windows")]
pub const PAGE_READWRITE: DWORD = 0x04;
#[cfg (feature = "vm_ms_windows")]
pub const PAGE_WRITECOPY: DWORD = 0x08;
#[cfg (feature = "vm_ms_windows")]
pub const ERROR_INVALID_ADDRESS: DWORD = 487;
#[cfg (feature = "vm_ms_windows")]
pub const INVALID_HANDLE_VALUE: HANDLE = usize::MAX as HANDLE;
#[cfg (feature = "vm_ms_windows")]
pub const FILE_MAP_READ: DWORD = 0x0004;
#[cfg (feature = "vm_ms_windows")]
pub const FILE_MAP_COPY: DWORD = 0x0001;
#[cfg (feature = "vm_ms_windows")]
unsafe extern "C" {
  pub fn VirtualAlloc (base: *mut c_void, size: size_t, mem_type: DWORD, mem_prot: DWORD) -> *mut c_void;
  pub fn VirtualFree (addr: *mut c_void, size: size_t, free_type: DWORD) -> c_int;
  pub fn VirtualProtect (addr: *mut c_void, size: size_t, new_prot: DWORD,
                         old_prot: *mut DWORD) -> c_int;
  pub fn GetLastError () -> DWORD;
  pub fn _get_osfhandle (fd: c_int) -> isize;
  pub fn CreateFileMapping (file: HANDLE, attrs: *mut c_void, protect: DWORD,
                            max_high: DWORD, max_low: DWORD, name: *const c_char) -> HANDLE;
  pub fn MapViewOfFileEx (section: HANDLE, access: DWORD, offset_high: DWORD,
                          offset_low: DWORD, size: size_t, base: *mut c_void) -> *mut c_void;
  pub fn CloseHandle (handle: HANDLE) -> c_int;
  pub fn UnmapViewOfFile (addr: *const c_void) -> c_int;
}

#[cfg (feature = "vm_ms_windows")]
fn dump_anonymous_allocate_w32 (base: *mut c_void, size: size_t,
                                protection: dump_memory_protection) -> *mut c_void
{
  let ret: *mut c_void;
  let mem_type: DWORD;
  let mem_prot: DWORD;

  match protection
    {
      dump_memory_protection::DUMP_MEMORY_ACCESS_NONE
        => { mem_type = MEM_RESERVE; mem_prot = PAGE_NOACCESS; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READ
        => { mem_type = MEM_COMMIT; mem_prot = PAGE_READONLY; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE
        => { mem_type = MEM_COMMIT; mem_prot = PAGE_READWRITE; }
      _ => unsafe { emacs_abort () }
    }

  ret = unsafe { VirtualAlloc (base, size, mem_type, mem_prot) };
  if ret.is_null ()
    { unsafe { *errno () = if !base.is_null () && GetLastError () == ERROR_INVALID_ADDRESS
                          { EBUSY } else { EPERM }; } }
  return ret;
}

/* C 4489-4547's `#if VM_SUPPORTED == VM_POSIX` arm is the live one.  */
/* Old versions of macOS only define MAP_ANON, not MAP_ANONYMOUS.
   FIXME: This probably belongs elsewhere (gnulib/autoconf?)  */
/* C 4492-4495's `#ifndef MAP_ANONYMOUS` arm is dead (the frozen target's `<sys/mman.h>`
   defines it as `MAP_ANON`); carried (P-51).  */
#[cfg (feature = "no_map_anonymous")]
pub const MAP_ANONYMOUS: c_int = MAP_ANON;

fn dump_anonymous_allocate_posix (base: *mut c_void, size: size_t,
                                  protection: dump_memory_protection) -> *mut c_void
{
  let mut ret: *mut c_void;
  let mem_prot: c_int;

  match protection
    {
      dump_memory_protection::DUMP_MEMORY_ACCESS_NONE
        => { mem_prot = PROT_NONE; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READ
        => { mem_prot = PROT_READ; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE
        => { mem_prot = PROT_READ | PROT_WRITE; }
      _ => unsafe { emacs_abort () }
    }

  let mut mem_flags: c_int = MAP_PRIVATE | MAP_ANONYMOUS;
  if mem_prot != PROT_NONE
    { mem_flags |= MAP_POPULATE; }
  if !base.is_null ()
    { mem_flags |= MAP_FIXED; }

  let mut retry: bool;
  loop
    {
      retry = false;
      ret = unsafe { mmap (base, size, mem_prot, mem_flags, -1, 0) };
      if ret == MAP_FAILED
	  && unsafe { *errno () } == EINVAL
	  && (mem_flags & MAP_POPULATE) != 0
        {
          /* This system didn't understand MAP_POPULATE, so try
             again without it.  */
          mem_flags &= !MAP_POPULATE;
          retry = true;
        }
      if !retry
        { break; }
    }

  if ret == MAP_FAILED
    { ret = core::ptr::null_mut (); }
  return ret;
}

/* Undo the effect of `dump_reserve_address_space'.  */

fn dump_anonymous_release (addr: *mut c_void, size: size_t)
{
  eassert! ((size as intmax_t) >= 0);
  /* C 4552-4561's `#if VM_SUPPORTED == VM_MS_WINDOWS` / `#elif` / `#else` chain: the
     POSIX arm is the one this target compiles, the other two are carried (P-51).  */
  #[cfg (feature = "vm_ms_windows")]
  { let _ = size; if unsafe { VirtualFree (addr, 0, MEM_RELEASE) } == 0
                     { unsafe { emacs_abort () } } }
  #[cfg (feature = "no_vm_supported")]
  { let _ = addr; let _ = size; unsafe { emacs_abort () } }
  #[cfg (not (any (feature = "vm_ms_windows", feature = "no_vm_supported")))]
  { if unsafe { munmap (addr, size) } < 0
      { unsafe { emacs_abort () } } }
}

/* Perform anonymous memory allocation.  */
fn dump_anonymous_allocate (base: *mut c_void, size: size_t,
                            protection: dump_memory_protection) -> *mut c_void
{
  let val: *mut c_void;

  /* C 4581-4589's `#if VM_SUPPORTED == VM_POSIX` / `#elif` / `#else` chain.  */
  #[cfg (feature = "vm_ms_windows")]
  { val = dump_anonymous_allocate_w32 (base, size, protection); }
  #[cfg (feature = "no_vm_supported")]
  { unsafe { *errno () = ENOSYS; } val = core::ptr::null_mut (); }
  #[cfg (not (any (feature = "vm_ms_windows", feature = "no_vm_supported")))]
  { val = dump_anonymous_allocate_posix (base, size, protection); }

  return val;
}

#[cfg (feature = "vm_ms_windows")]
fn dump_map_file_w32 (base: *mut c_void, fd: c_int, offset: off_t, size: size_t,
		      protection: dump_memory_protection) -> *mut c_void
{
  let mut ret: *mut c_void = core::ptr::null_mut ();
  let mut section: HANDLE = core::ptr::null_mut ();
  let mut file: HANDLE;

  let full_offset: u64 = offset as u64;
  let offset_high: u32 = (full_offset >> 32) as u32;
  let offset_low: u32 = (full_offset & 0xffffffff) as u32;

  let mut error: DWORD = 0;
  let mut protect: DWORD = 0;
  let mut map_access: DWORD = 0;

  file = unsafe { _get_osfhandle (fd) } as HANDLE;
  if file == INVALID_HANDLE_VALUE
    { return ret; }   /* goto out */

  match protection
    {
      dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE
        => { protect = PAGE_WRITECOPY; }	/* for Windows 9X */
      _ /* DUMP_MEMORY_ACCESS_NONE / DUMP_MEMORY_ACCESS_READ */
        => { protect = PAGE_READONLY; }
    }

  section = unsafe { CreateFileMapping (file, core::ptr::null_mut (), protect, 0, 0,
                                        core::ptr::null ()) };
  if section.is_null ()
    {
      unsafe { *errno () = EINVAL; }
      return ret;   /* goto out */
    }

  match protection
    {
      dump_memory_protection::DUMP_MEMORY_ACCESS_NONE
      | dump_memory_protection::DUMP_MEMORY_ACCESS_READ
        => { map_access = FILE_MAP_READ; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE
        => { map_access = FILE_MAP_COPY; }
      _ => unsafe { emacs_abort () }
    }

  ret = unsafe { MapViewOfFileEx (section, map_access, offset_high, offset_low, size, base) };

  error = unsafe { GetLastError () };
  if ret.is_null ()
    { unsafe { *errno () = if error == ERROR_INVALID_ADDRESS { EBUSY } else { EPERM }; } }
  /* out: */
  if !section.is_null () && unsafe { CloseHandle (section) } == 0
    { unsafe { emacs_abort () } }
  return ret;
}

fn dump_map_file_posix (base: *mut c_void, fd: c_int, offset: off_t, size: size_t,
			protection: dump_memory_protection) -> *mut c_void
{
  let mut ret: *mut c_void;
  let mem_prot: c_int;
  let mut mem_flags: c_int;

  match protection
    {
      dump_memory_protection::DUMP_MEMORY_ACCESS_NONE
        => { mem_prot = PROT_NONE; mem_flags = MAP_SHARED; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READ
        => { mem_prot = PROT_READ; mem_flags = MAP_SHARED; }
      dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE
        => { mem_prot = PROT_READ | PROT_WRITE; mem_flags = MAP_PRIVATE; }
      _ => unsafe { emacs_abort () }
    }

  if !base.is_null ()
    { mem_flags |= MAP_FIXED; }

  ret = unsafe { mmap (base, size, mem_prot, mem_flags, fd, offset) };
  if ret == MAP_FAILED
    { ret = core::ptr::null_mut (); }
  return ret;
}

/* Map a file into memory.  */
fn dump_map_file (base: *mut c_void, fd: c_int, offset: off_t, size: size_t,
		  protection: dump_memory_protection) -> *mut c_void
{
  /* C 4705-4714's `#if VM_SUPPORTED == VM_POSIX` / `#elif` / `#else` chain.  */
  #[cfg (feature = "vm_ms_windows")]
  { return dump_map_file_w32 (base, fd, offset, size, protection); }
  #[cfg (feature = "no_vm_supported")]
  { unsafe { *errno () = ENOSYS; } return core::ptr::null_mut (); }
  #[cfg (not (any (feature = "vm_ms_windows", feature = "no_vm_supported")))]
  { return dump_map_file_posix (base, fd, offset, size, protection); }
}

/* Remove a virtual memory mapping.

   On failure, abort Emacs.  For maximum platform compatibility, ADDR
   and SIZE must match the mapping exactly.  */
fn dump_unmap_file (addr: *mut c_void, size: size_t)
{
  eassert! ((size as intmax_t) >= 0);
  /* C 4730-4741's `#if !VM_SUPPORTED` / `#elif defined (WINDOWSNT)` / `#else` chain.  */
  #[cfg (feature = "no_vm_supported")]
  { let _ = addr; let _ = size; unsafe { emacs_abort () } }
  #[cfg (all (feature = "windowsnt", not (feature = "no_vm_supported")))]
  { let _ = size; if unsafe { UnmapViewOfFile (addr) } == 0 { unsafe { emacs_abort () } } }
  #[cfg (not (any (feature = "no_vm_supported", feature = "windowsnt")))]
  { if unsafe { munmap (addr, size) } < 0 { unsafe { emacs_abort () } } }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_memory_map_spec
{
  pub fd: c_int,  /* File to map; anon zero if negative.  */
  pub size: size_t,  /* Number of bytes to map.  */
  pub offset: off_t,  /* Offset within fd.  */
  pub protection: dump_memory_protection,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_memory_map
{
  pub spec: dump_memory_map_spec,
  pub mapping: *mut c_void,  /* Actual mapped memory.  */
  pub release: Option<unsafe extern "C" fn (*mut dump_memory_map)>,
  pub private: *mut c_void,
}

/// C 4761-4764: `void (*release) (struct dump_memory_map *)` — the port needs a Rust
/// type for the two bodies C assigns (`dump_mmap_release_heap`, `dump_mmap_release_vm`);
/// `mmap_release_fn!` is C's other spelling, C-compatible because C never crosses the
/// boundary here.
macro_rules! mmap_release_fn {
    ($f:expr) => { Some ($f as unsafe extern "C" fn (*mut dump_memory_map)) };
}

/* Mark the pages as unneeded, potentially zeroing them, without
   releasing the address space reservation.  */
fn dump_discard_mem (mem: *mut c_void, size: size_t)
{
  /* C 4759-4775: `#if VM_SUPPORTED == VM_MS_WINDOWS` is dead, then
     `#elif VM_SUPPORTED == VM_POSIX` selects `#ifdef HAVE_POSIX_MADVISE` (1 in the frozen
     configuration) over `#elif defined HAVE_MADVISE`.  The two dead arms are carried.  */
  #[cfg (feature = "vm_ms_windows")]
  unsafe {
    /* Discard COWed pages.  */
    VirtualFree (mem, size, MEM_DECOMMIT);
    /* Release the commit charge for the mapping.  */
    let mut old_prot: DWORD = 0;
    VirtualProtect (mem, size, PAGE_NOACCESS, &mut old_prot);
  }
  /* C 4765-4773: inside `#elif VM_SUPPORTED == VM_POSIX`, `#ifdef HAVE_POSIX_MADVISE` then
     `#elif defined HAVE_MADVISE`.  The frozen configuration has the first; the Android one has
     `HAVE_POSIX_MADVISE` undefined and `HAVE_MADVISE` set, so it takes the second (bionic's
     `madvise` is API 1, status 4.720).  `havve_madvise` is the name `xtask` emits from that key.  */
  #[cfg (not (feature = "have_madvise"))]
  #[cfg (not (any (feature = "vm_ms_windows", feature = "windowsnt")))]
  unsafe {
    /* Discard COWed pages.  */
    posix_madvise (mem, size, POSIX_MADV_DONTNEED);
  }
  #[cfg (feature = "have_madvise")]
  #[cfg (not (any (feature = "vm_ms_windows", feature = "windowsnt")))]
  unsafe {
    /* Discard COWed pages.  */
    madvise (mem, size, MADV_DONTNEED);
  }
  /* The commit charge is released on both POSIX arms (C 4774-4776 sits outside the `#if`).  */
  #[cfg (not (any (feature = "vm_ms_windows", feature = "windowsnt")))]
  unsafe {
    mprotect (mem, size, PROT_NONE);
  }
}

fn dump_mmap_discard_contents (map: *mut dump_memory_map)
{
  unsafe {
    if !(*map).mapping.is_null ()
      { dump_discard_mem ((*map).mapping, (*map).spec.size); }
  }
}

fn dump_mmap_reset (map: *mut dump_memory_map)
{
  unsafe {
    (*map).mapping = core::ptr::null_mut ();
    (*map).release = None;
    (*map).private = core::ptr::null_mut ();
  }
}

fn dump_mmap_release (map: *mut dump_memory_map)
{
  unsafe {
    if let Some (release) = (*map).release
      { release (map); }
  }
  dump_mmap_reset (map);
}

/* Allows heap-allocated dump_mmap to "free" maps individually.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_memory_map_heap_control_block
{
  pub refcount: c_int,
  pub mem: *mut c_void,
}

fn dump_mm_heap_cb_release (cb: *mut dump_memory_map_heap_control_block)
{
  if !cb.is_null ()
    {
      eassert! (unsafe { (*cb).refcount } > 0);
      unsafe {
        (*cb).refcount -= 1;
        if (*cb).refcount == 0
	  {
	    free ((*cb).mem);
	    free (cb as *mut c_void);
	  }
      }
    }
}

unsafe extern "C" fn dump_mmap_release_heap (map: *mut dump_memory_map)
{
  unsafe { dump_mm_heap_cb_release ((*map).private as *mut dump_memory_map_heap_control_block); }
}

/* Implement dump_mmap using malloc and read.  */
fn dump_mmap_contiguous_heap (maps: *mut dump_memory_map, nr_maps: c_int,
			      total_size: size_t) -> bool
{
  let mut ret: bool = false;

  /* FIXME: This storage sometimes is never freed.
     Beware: the simple patch 2019-03-11T15:20:54Z!eggert@cs.ucla.edu
     is worse, as it sometimes frees this storage twice.  */
  let cb: *mut dump_memory_map_heap_control_block = unsafe {
    calloc (1, core::mem::size_of::<dump_memory_map_heap_control_block> ()) }
    as *mut dump_memory_map_heap_control_block;
  if cb.is_null ()
    { /* goto out */
      unsafe { dump_mm_heap_cb_release (cb); }
      if !ret
        {
          let mut i: c_int = 0;
          while i < nr_maps
            { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
        }
      return ret;
    }
  unsafe { __lsan_ignore_object (cb as *const c_void); }

  unsafe {
    (*cb).refcount = 1;
    (*cb).mem = malloc (total_size);
  }
  if unsafe { (*cb).mem.is_null () }
    { /* goto out */
      unsafe { dump_mm_heap_cb_release (cb); }
      if !ret
        {
          let mut i: c_int = 0;
          while i < nr_maps
            { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
        }
      return ret;
    }
  let mut mem: *mut c_char = unsafe { (*cb).mem } as *mut c_char;
  let mut i: c_int = 0;
  while i < nr_maps
    {
      let map: *mut dump_memory_map = unsafe { &mut *maps.offset (i as isize) };
      let spec: dump_memory_map_spec = unsafe { (*map).spec };
      if spec.size != 0
        {
          unsafe {
            (*map).mapping = mem as *mut c_void;
            mem = mem.add (spec.size);
            (*map).release = mmap_release_fn! (dump_mmap_release_heap);
            (*map).private = cb as *mut c_void;
            (*cb).refcount += 1;
          }
          if spec.fd < 0
            { unsafe { memset (mem.offset (-(spec.size as isize)) as *mut c_void, 0, spec.size); } }
          else
            {
              if unsafe { lseek (spec.fd, spec.offset, SEEK_SET) } < 0
                { /* goto out */
                  unsafe { dump_mm_heap_cb_release (cb); }
                  if !ret
                    {
                      let mut i: c_int = 0;
                      while i < nr_maps
                        { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
                    }
                  return ret;
                }
              let nb: ssize_t = unsafe {
                dump_read_all (spec.fd, (*map).mapping, spec.size) };
              if nb >= 0 && nb as size_t != spec.size
                { unsafe { *errno () = EIO; } }
              if nb as size_t != spec.size
                { /* goto out */
                  unsafe { dump_mm_heap_cb_release (cb); }
                  if !ret
                    {
                      let mut i: c_int = 0;
                      while i < nr_maps
                        { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
                    }
                  return ret;
                }
            }
        }
      i += 1;
    }

  ret = true;
  /* out: */
  unsafe { dump_mm_heap_cb_release (cb); }
  if !ret
    {
      let mut i: c_int = 0;
      while i < nr_maps
        { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
    }
  return ret;
}

unsafe extern "C" fn dump_mmap_release_vm (map: *mut dump_memory_map)
{
  unsafe {
    let map = &mut *map;
    if map.spec.fd < 0
      { dump_anonymous_release (map.mapping, map.spec.size); }
    else
      { dump_unmap_file (map.mapping, map.spec.size); }
  }
}

/// C 4955-4957's `#ifdef CYGWIN` arm, as the port's `#[cfg]`-selected predicate (the arm is
/// dead on this target; `cygwin.rs` is the unit that would define the C symbol).
#[inline(always)]
fn cfg_if_cygwin () -> bool {
  #[cfg (feature = "cygwin")]
  { true }
  #[cfg (not (feature = "cygwin"))]
  { false }
}

fn needs_mmap_retry_p () -> bool
{
  /* C 4895-4900: `#if defined CYGWIN || VM_SUPPORTED == VM_MS_WINDOWS || defined _AIX` is
     dead on this target; the `#else` arm is the live text.  */
  return false;
}

fn dump_mmap_contiguous_vm (maps: *mut dump_memory_map, nr_maps: c_int,
			    total_size: size_t) -> bool
{
  let mut save_errno: c_int;
  let mut ret: bool = false;
  let mut resv: *mut c_void = core::ptr::null_mut ();
  let mut retry: bool = false;
  let need_retry: bool = needs_mmap_retry_p ();

  loop
    {
      if retry
        {
          eassert! (need_retry);
          retry = false;
          let mut i: c_int = 0;
          while i < nr_maps
            { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); i += 1; }
        }

      eassert! (resv.is_null ());
      resv = dump_anonymous_allocate (core::ptr::null_mut (), total_size,
                                      dump_memory_protection::DUMP_MEMORY_ACCESS_NONE);
      if resv.is_null ()
        { /* goto out */
          save_errno = unsafe { *errno () };
          if !resv.is_null ()
            { dump_anonymous_release (resv, total_size); }
          if !ret
            {
              let mut i: c_int = 0;
              while i < nr_maps
                {
                  if need_retry
                    { dump_mmap_reset (unsafe { &mut *maps.offset (i as isize) }); }
                  else
                    { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); }
                  i += 1;
                }
            }
          unsafe { *errno () = save_errno; }
          return ret;
        }

      let mut mem: *mut c_char = resv as *mut c_char;

      if need_retry
        {
          /* Windows lacks atomic mapping replace; need to release the
             reservation so we can allocate within it.  Will retry the
             loop if someone squats on our address space before we can
             finish allocation.  On POSIX systems, we leave the
             reservation around for atomicity.  */
          dump_anonymous_release (resv, total_size);
          resv = core::ptr::null_mut ();
        }

      let mut i: c_int = 0;
      while i < nr_maps
        {
          let map: *mut dump_memory_map = unsafe { &mut *maps.offset (i as isize) };
          let spec: dump_memory_map_spec = unsafe { (*map).spec };
          if spec.size == 0
            { i += 1; continue; }

          unsafe {
            if spec.fd < 0
	      { (*map).mapping = dump_anonymous_allocate (mem as *mut c_void, spec.size,
						          spec.protection); }
            else
	      { (*map).mapping = dump_map_file (mem as *mut c_void, spec.fd, spec.offset,
					        spec.size, spec.protection); }
            mem = mem.add (spec.size);
	  }
	  /* C 4955-4957's `#ifdef CYGWIN` arm is dead; the port keeps C's two-arm shape with
	     a `#[cfg]`-selected second test.  */
	  let retry_errno: bool = unsafe {
	    *errno () == EBUSY
	      || (cfg_if_cygwin () && *errno () == EINVAL)
	  };
	  if need_retry && unsafe { (*map).mapping.is_null () } && retry_errno
            {
              retry = true;
              i += 1;
              continue;
            }
          if unsafe { (*map).mapping.is_null () }
            { /* goto out */
              save_errno = unsafe { *errno () };
              if !resv.is_null ()
                { dump_anonymous_release (resv, total_size); }
              if !ret
                {
                  let mut i: c_int = 0;
                  while i < nr_maps
                    {
                      if need_retry
                        { dump_mmap_reset (unsafe { &mut *maps.offset (i as isize) }); }
                      else
                        { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); }
                      i += 1;
                    }
                }
              unsafe { *errno () = save_errno; }
              return ret;
            }
          unsafe { (*map).release = mmap_release_fn! (dump_mmap_release_vm); }
          i += 1;
        }
      if !retry
        { break; }
    }

  ret = true;
  resv = core::ptr::null_mut ();
  /* out: */
  save_errno = unsafe { *errno () };
  if !resv.is_null ()
    { dump_anonymous_release (resv, total_size); }
  if !ret
    {
      let mut i: c_int = 0;
      while i < nr_maps
	{
	  if need_retry
	    { dump_mmap_reset (unsafe { &mut *maps.offset (i as isize) }); }
	  else
	    { dump_mmap_release (unsafe { &mut *maps.offset (i as isize) }); }
	  i += 1;
	}
    }
  unsafe { *errno () = save_errno; }
  return ret;
}

/* Map a range of addresses into a chunk of contiguous memory.

   Each dump_memory_map structure describes how to fill the
   corresponding range of memory. On input, all members except MAPPING
   are valid.  On output, MAPPING contains the location of the given
   chunk of memory.  The MAPPING for MAPS[N] is MAPS[N-1].mapping +
   MAPS[N-1].size.

   Each mapping SIZE must be a multiple of the system page size except
   for the last mapping.

   Return true on success or false on failure with errno set.  */
fn dump_mmap_contiguous (maps: *mut dump_memory_map, nr_maps: c_int) -> bool
{
  if nr_maps == 0
    { return true; }

  let mut total_size: size_t = 0;
  let worst_case_page_size: c_int = dump_get_max_page_size ();

  let mut i: c_int = 0;
  while i < nr_maps
    {
      unsafe {
        eassert! ((*maps.offset (i as isize)).mapping.is_null ());
        eassert! ((*maps.offset (i as isize)).release.is_none ());
        eassert! ((*maps.offset (i as isize)).private.is_null ());
        if i != nr_maps - 1
          { eassert! ((*maps.offset (i as isize)).spec.size % worst_case_page_size as size_t == 0); }
        total_size += (*maps.offset (i as isize)).spec.size;
      }
      i += 1;
    }

  if VM_SUPPORTED != 0
    { return dump_mmap_contiguous_vm (maps, nr_maps, total_size); }
  else
    { return dump_mmap_contiguous_heap (maps, nr_maps, total_size); }
}

pub type dump_bitset_word = uint_fast32_t;
/// C 5031: `#define DUMP_BITSET_WORD_WIDTH UINT_FAST32_WIDTH` (`<stdint.h>`'s
/// `UINT_FAST32_WIDTH`, 32 on the frozen target).
pub const DUMP_BITSET_WORD_WIDTH: c_int = UINT_FAST32_WIDTH;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dump_bitset
{
  pub bits: *mut dump_bitset_word,
  pub number_words: ptrdiff_t,
}

fn dump_bitsets_init (bitset: *mut dump_bitset, number_bits: size_t) -> bool
{
  let xword_size: c_int = core::mem::size_of::<dump_bitset_word> () as c_int;
  let words_needed: ptrdiff_t = divide_round_up (number_bits,
						 DUMP_BITSET_WORD_WIDTH as size_t)
    as ptrdiff_t;
  let bits: *mut dump_bitset_word = unsafe {
    calloc (words_needed as usize, (2 * xword_size) as usize) } as *mut dump_bitset_word;
  if bits.is_null ()
    { return false; }
  unsafe {
    (*bitset.offset (0)).bits = bits;
    (*bitset.offset (0)).number_words = words_needed;
    (*bitset.offset (1)).number_words = words_needed;
    (*bitset.offset (1)).bits = memset (bits.offset (words_needed) as *mut c_void, UCHAR_MAX,
			                words_needed as usize * xword_size as usize)
      as *mut dump_bitset_word;
  }
  return true;
}

fn dump_bitset__bit_slot (bitset: *const dump_bitset, bit_number: size_t)
  -> *mut dump_bitset_word
{
  let word_number: ptrdiff_t = (bit_number / DUMP_BITSET_WORD_WIDTH as size_t) as ptrdiff_t;
  eassert! (word_number < unsafe { (*bitset).number_words });
  return unsafe { (*bitset).bits.offset (word_number) };
}

fn dump_bitset_bit_set_p (bitset: *const dump_bitset, bit_number: size_t) -> bool
{
  let mut bit: dump_bitset_word = 1;
  bit <<= bit_number % DUMP_BITSET_WORD_WIDTH as size_t;
  return (unsafe { *dump_bitset__bit_slot (bitset, bit_number) } & bit) != 0;
}

fn dump_bitset__set_bit_value (bitset: *mut dump_bitset, bit_number: size_t, bit_is_set: bool)
{
  let slot: *mut dump_bitset_word = dump_bitset__bit_slot (bitset, bit_number);
  let mut bit: dump_bitset_word = 1;
  bit <<= bit_number % DUMP_BITSET_WORD_WIDTH as size_t;
  unsafe {
    if bit_is_set
      { *slot = *slot | bit; }
    else
      { *slot = *slot & !bit; }
  }
}

fn dump_bitset_set_bit (bitset: *mut dump_bitset, bit_number: size_t)
{
  dump_bitset__set_bit_value (bitset, bit_number, true);
}

fn dump_bitset_clear (bitset: *mut dump_bitset)
{
  /* Skip the memset if bitset->number_words == 0, because then bitset->bits
     might be NULL and the memset would have undefined behavior.  */
  unsafe {
    if (*bitset).number_words != 0
      { memset ((*bitset).bits as *mut c_void, 0,
                (*bitset).number_words as usize * core::mem::size_of::<dump_bitset_word> ()); }
  }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pdumper_loaded_dump_private
{
  /* Copy of the header we read from the dump.  */
  pub header: dump_header,
  /* Mark bits for objects in the dump; used during GC.  */
  pub mark_bits: dump_bitset,
  pub last_mark_bits: dump_bitset,
  /* Time taken to load the dump.  */
  pub load_time: c_double,
  /* Dump file name.  */
  pub dump_filename: *mut c_char,
}

#[unsafe(no_mangle)]
pub static mut dump_public: pdumper_loaded_dump =
  pdumper_loaded_dump { start: 0, end: 0 };
static mut dump_private: pdumper_loaded_dump_private =
  pdumper_loaded_dump_private {
    header: dump_header {
      magic: [0; 16],
      fingerprint: [0; 32],
      dump_relocs: [dump_table_locator { offset: 0, nr_entries: 0 };
                     reloc_phase::RELOC_NUM_PHASES as usize],
      object_starts: dump_table_locator { offset: 0, nr_entries: 0 },
      emacs_relocs: dump_table_locator { offset: 0, nr_entries: 0 },
      discardable_start: 0,
      cold_start: 0,
      hash_list: 0,
    },
    mark_bits: dump_bitset { bits: core::ptr::null_mut (), number_words: 0 },
    last_mark_bits: dump_bitset { bits: core::ptr::null_mut (), number_words: 0 },
    load_time: 0.0,
    dump_filename: core::ptr::null_mut (),
  };

/* Return a pointer to offset OFFSET within the dump, which begins at
   DUMP_BASE. DUMP_BASE must be equal to the current dump load
   location; it's passed as a parameter for efficiency.

   The returned pointer points to the primary memory image of the
   currently-loaded dump file.  The entire dump file is accessible
   using this function.  */
fn dump_ptr (dump_base: uintptr_t, offset: dump_off) -> *mut c_void
{
  eassert! (dump_base == unsafe { dump_public.start });
  eassert! (0 <= offset);
  eassert! ((unsafe { dump_public.start } + offset as uintptr_t) < unsafe { dump_public.end });
  return (dump_base as isize).wrapping_add (offset as isize) as *mut c_void;
}

/* Read a pointer-sized word of memory at OFFSET within the dump,
   which begins at DUMP_BASE. DUMP_BASE must be equal to the current
   dump load location; it's passed as a parameter for efficiency.  */
fn dump_read_word_from_dump (dump_base: uintptr_t, offset: dump_off) -> uintptr_t
{
  let mut value: uintptr_t = 0;
  /* The compiler optimizes this memcpy into a read.  */
  unsafe {
    memcpy ((&raw mut value) as *mut c_void, dump_ptr (dump_base, offset),
            core::mem::size_of_val (&value));
  }
  return value;
}

/* Write a word to the dump. DUMP_BASE and OFFSET are as for
   dump_read_word_from_dump; VALUE is the word to write at the given
   offset.  */
fn dump_write_word_to_dump (dump_base: uintptr_t, offset: dump_off, value: uintptr_t)
{
  /* The compiler optimizes this memcpy into a write.  */
  unsafe {
    memcpy (dump_ptr (dump_base, offset), (&raw const value) as *const c_void,
            core::mem::size_of_val (&value));
  }
}

/* Write a Lisp_Object to the dump. DUMP_BASE and OFFSET are as for
   dump_read_word_from_dump; VALUE is the Lisp_Object to write at the
   given offset.  */
fn dump_write_lv_to_dump (dump_base: uintptr_t, offset: dump_off, value: Lisp_Object)
{
  /* The compiler optimizes this memcpy into a write.  */
  unsafe {
    memcpy (dump_ptr (dump_base, offset), (&raw const value) as *const c_void,
            core::mem::size_of_val (&value));
  }
}

/* Search for a relocation given a relocation target.

   DUMP is the dump metadata structure.  TABLE is the relocation table
   to search.  KEY is the dump offset to find.  Return the relocation
   RELOC such that RELOC.offset is the smallest RELOC.offset that
   satisfies the constraint KEY <= RELOC.offset --- that is, return
   the first relocation at KEY or after KEY.  Return NULL if no such
   relocation exists.  */
fn dump_find_relocation (table: *const dump_table_locator, key: dump_off)
  -> *const dump_reloc
{
  let relocs: *const dump_reloc = dump_ptr (unsafe { dump_public.start },
					    unsafe { (*table).offset })
    as *const dump_reloc;
  let mut found: *const dump_reloc = core::ptr::null ();
  let mut idx_left: ptrdiff_t = 0;
  let mut idx_right: ptrdiff_t = unsafe { (*table).nr_entries } as ptrdiff_t;

  eassert! (key >= 0);

  while idx_left < idx_right
    {
      let idx_mid: ptrdiff_t = idx_left + (idx_right - idx_left) / 2;
      let mid: *const dump_reloc = unsafe { relocs.offset (idx_mid) };
      if key > dump_reloc_get_offset (unsafe { *mid })
        { idx_left = idx_mid + 1; }
      else
        {
          found = mid;
          idx_right = idx_mid;
	  if idx_right <= idx_left
	      || key > dump_reloc_get_offset (unsafe { *relocs.offset (idx_right - 1) })
            { break; }
        }
   }

  return found;
}

fn dump_loaded_p () -> bool
{
  return unsafe { dump_public.start } != 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_cold_object_p_impl (obj: *const c_void) -> bool
{
  eassert! (pdumper_object_p (obj));
  eassert! (pdumper_object_p_precise (obj));
  let offset: dump_off = ptrdiff_t_to_dump_off ((obj as uintptr_t
                                                 - unsafe { dump_public.start }) as ptrdiff_t);
  return offset >= unsafe { dump_private.header.cold_start };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_find_object_type_impl (obj: *const c_void) -> c_int
{
  eassert! (pdumper_object_p (obj));
  let offset: dump_off = ptrdiff_t_to_dump_off ((obj as uintptr_t
                                                 - unsafe { dump_public.start }) as ptrdiff_t);
  if offset % DUMP_ALIGNMENT != 0
    { return PDUMPER_NO_OBJECT; }
  let bitno: ptrdiff_t = (offset / DUMP_ALIGNMENT) as ptrdiff_t;
  if offset < unsafe { dump_private.header.discardable_start }
      && !dump_bitset_bit_set_p (unsafe { &raw const dump_private.last_mark_bits }, bitno as size_t)
    { return PDUMPER_NO_OBJECT; }
  let reloc: *const dump_reloc =
    dump_find_relocation (unsafe { &raw const dump_private.header.object_starts }, offset);
  /* C 5236-5238: `return (reloc != NULL && dump_reloc_get_offset (*reloc) == offset)
     ? reloc->type : PDUMPER_NO_OBJECT;`.  The ternary's *condition* is evaluated first and
     `reloc->type` only in the true arm, so a NULL `reloc` is never read.  The port used to
     hoist that read into a `let` above the test, which is a null dereference on the path the
     reference takes safely - the UB check reported it (build/femacs/units/pdumper/_root.rs
     5916, status 4.419) and 33 of the 68 `lisp/emacs-lisp` ERT files died there.  */
  return if !reloc.is_null () && dump_reloc_get_offset (unsafe { *reloc }) == offset
    {
      /* The ternary's true arm, and the only place the record is read for its type: C
         evaluates `reloc->type` here and never on the NULL path above.  */
      let r#type: c_int = dump_reloc_type_of! (unsafe { *reloc });
      r#type
    }
    else { PDUMPER_NO_OBJECT };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_marked_p_impl (obj: *const c_void) -> bool
{
  eassert! (pdumper_object_p (obj));
  let offset: ptrdiff_t = (obj as uintptr_t - unsafe { dump_public.start }) as ptrdiff_t;
  eassert! (offset % DUMP_ALIGNMENT as ptrdiff_t == 0);
  eassert! (offset < unsafe { dump_private.header.cold_start } as ptrdiff_t);
  eassert! (offset < unsafe { dump_private.header.discardable_start } as ptrdiff_t);
  let bitno: ptrdiff_t = offset / DUMP_ALIGNMENT as ptrdiff_t;
  return dump_bitset_bit_set_p (unsafe { &raw const dump_private.mark_bits }, bitno as size_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_set_marked_impl (obj: *const c_void)
{
  eassert! (pdumper_object_p (obj));
  let offset: ptrdiff_t = (obj as uintptr_t - unsafe { dump_public.start }) as ptrdiff_t;
  eassert! (offset % DUMP_ALIGNMENT as ptrdiff_t == 0);
  eassert! (offset < unsafe { dump_private.header.cold_start } as ptrdiff_t);
  eassert! (offset < unsafe { dump_private.header.discardable_start } as ptrdiff_t);
  let bitno: ptrdiff_t = offset / DUMP_ALIGNMENT as ptrdiff_t;
  eassert! (dump_bitset_bit_set_p (unsafe { &raw const dump_private.last_mark_bits },
                                   bitno as size_t));
  dump_bitset_set_bit (unsafe { &raw mut dump_private.mark_bits }, bitno as size_t);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_clear_marks_impl ()
{
  unsafe {
    let swap: *mut dump_bitset_word = dump_private.last_mark_bits.bits;
    dump_private.last_mark_bits.bits = dump_private.mark_bits.bits;
    dump_private.mark_bits.bits = swap;
  }
  dump_bitset_clear (unsafe { &raw mut dump_private.mark_bits });
}

fn dump_read_all (fd: c_int, buf: *mut c_void, bytes_to_read: size_t) -> ssize_t
{
  /* We don't want to use emacs_read, since that relies on the lisp
     world, and we're not in the lisp world yet.  */
  let mut bytes_read: size_t = 0;
  while bytes_read < bytes_to_read
    {
      let chunk_to_read: c_int = min ((bytes_to_read - bytes_read) as isize,
                                      SYS_BUFSIZE_MAX as isize) as c_int;
      let chunk: ssize_t = unsafe {
        read (fd, (buf as *mut c_char).add (bytes_read) as *mut c_void,
              chunk_to_read as size_t) };
      if chunk < 0
        { return chunk; }
      if chunk == 0
        { break; }
      bytes_read += chunk as size_t;
    }

  return bytes_read as ssize_t;
}

/* Return the number of bytes written when we perform the given
   relocation.  */
fn dump_reloc_size (reloc: dump_reloc) -> c_int
{
  if core::mem::size_of::<Lisp_Object> () == core::mem::size_of::<*mut c_void> ()
    { return core::mem::size_of::<Lisp_Object> () as c_int; }
  let r#type: c_int = dump_reloc_type_of! (reloc);
  if r#type == dump_reloc_type::RELOC_DUMP_TO_EMACS_PTR_RAW as c_int
      || r#type == dump_reloc_type::RELOC_DUMP_TO_DUMP_PTR_RAW as c_int
    { return core::mem::size_of::<*mut c_void> () as c_int; }
  return core::mem::size_of::<Lisp_Object> () as c_int;
}

fn dump_make_lv_from_reloc (dump_base: uintptr_t, reloc: dump_reloc) -> Lisp_Object
{
  let reloc_offset: dump_off = dump_reloc_get_offset (reloc);
  let mut value: uintptr_t = dump_read_word_from_dump (dump_base, reloc_offset);
  let lisp_type: Lisp_Type;

  let r#type: c_int = dump_reloc_type_of! (reloc);
  if dump_reloc_type::RELOC_DUMP_TO_DUMP_LV as c_int <= r#type
      && r#type < dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int
    {
      lisp_type = unsafe { core::mem::transmute::<c_int, Lisp_Type> (
                             r#type - dump_reloc_type::RELOC_DUMP_TO_DUMP_LV as c_int) };
      value += dump_base;
      eassert! (pdumper_object_p (value as *const c_void));
    }
  else
    {
      eassert! (dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int <= r#type);
      eassert! (r#type < dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int + 8);
      lisp_type = unsafe { core::mem::transmute::<c_int, Lisp_Type> (
                             r#type - dump_reloc_type::RELOC_DUMP_TO_EMACS_LV as c_int) };
      /* PORTING.md P-96: C's `uintptr_t` addition *relies on wrapping* - the word in the
         dump is `emacs_offset (ptr)`, a signed offset, so a negative one is a huge unsigned
         here and adding the basis wraps back to the pointer (status 3.643).  */
      value = value.wrapping_add (emacs_basis ());
    }

  eassert! (lisp_type != Lisp_Type::Lisp_Int0 && lisp_type != Lisp_Type::Lisp_Int1);

  let lv: Lisp_Object;
  if lisp_type == Lisp_Type::Lisp_Symbol
    { lv = make_lisp_symbol (value as *mut Lisp_Symbol); }
  else
    { lv = make_lisp_ptr (value as *mut c_void, lisp_type); }

  return lv;
}

/* C 5462-5521's `#ifdef HAVE_NATIVE_COMP` cases are dead on this target; carried as text
   (P-51, docs/status.md 3.49).  The names its two arms need are declared in the same
   `#[cfg]` block, because `comp.h` and `<dynlib.h>` have no mirrors yet (`treesit.h` has one
   now, but the record this arm needs is the one in that header's dead arm).  */
/* `comp.h`'s record moved to its owner (`include/comp.rs`, status 3.153): C's `pdumper.c` reaches
   it the way this unit does - `lisp.h` includes `comp.h` at lisp.h 2168, and `include/lisp.rs`
   includes `include/comp.rs` - so the two helpers below resolve the type from the mirror instead
   of declaring a second copy of it.  Keeping the copy here would have collided with the mirror's
   record in any `native_comp` build (`E0428`), which is exactly what the missing-type check made
   visible.  */
/* comp.h 55-67's `NATIVE_COMP_UNITP` / `XNATIVE_COMP_UNIT`: these two followed the record into
   its owner (`include/comp.rs`, status 3.223) - `lisp.h` 2168 includes `comp.h` and
   `include/lisp.rs` includes that mirror, so this unit resolves them from there and a second
   copy here would be `E0428` in a `native_comp` build (the same class as the record itself,
   which moved at 3.153).  */
/* `dynlib.h`'s `dynlib_handle_ptr` comes from its own mirror (`include/dynlib.rs`) through
   `lisp.h` -> `comp.h` -> `<dynlib.h>` now that `include/comp.rs` includes it (status 3.154); the
   use-site copy that used to stand here was what the dead-arms gate reported as a duplicate in a
   `native_comp` build, the same class as the three functions retired above.  */
/// config.h 120's `#define DIRECTORY_SEP '/'` and config.h 2097's
/// `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)` (the same use-site pair
/// callproc.rs keeps).
#[cfg (feature = "have_native_comp")]
pub const DIRECTORY_SEP: c_char = b'/' as c_char;
#[cfg (feature = "have_native_comp")]
#[inline(always)]
pub fn IS_DIRECTORY_SEP (_c_: c_char) -> bool { _c_ == DIRECTORY_SEP }
/// C's `static enum { UNKNOWN, LOCAL_BUILD, INSTALLED } installation_state;` inside
/// `dump_do_dump_relocation` (C 5468).
#[cfg (feature = "have_native_comp")]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum native_installation_state { UNKNOWN, LOCAL_BUILD, INSTALLED }
/* This arm's three `<dynlib.h>` entries (`dynlib_open_for_eln`, `dynlib_sym`, `dynlib_error`) come
   from the mirror that owns them: `lisp.h` 2168 includes `comp.h`, `comp.h` 23 includes
   `<dynlib.h>`, and the mirrors follow that chain, so the copies that used to stand here were
   retired when `include/comp.rs` began including `include/dynlib.rs` (status 3.154,
   `check-mirror-dupes` found them).  The three below are `comp.c`'s and `fileio.c`'s, which no
   mirror carries yet.  */
#[cfg (feature = "have_native_comp")]
unsafe extern "C" {
  pub fn load_comp_unit (comp_u: *mut Lisp_Native_Comp_Unit, loading_dump: bool,
                         late_load: bool) -> Lisp_Object;
  pub fn fixup_eln_load_path (directory: Lisp_Object);
  pub fn file_access_p (file: *mut c_char, mode: c_int) -> bool;
}

/* Actually apply a dump relocation.  */
#[inline(always)]
fn dump_do_dump_relocation (dump_base: uintptr_t, reloc: dump_reloc)
{
  let reloc_offset: dump_off = dump_reloc_get_offset (reloc);

  /* We should never generate a relocation in the cold section.  */
  eassert! (reloc_offset < unsafe { dump_private.header.cold_start });

  /* C's `switch (reloc.type)`.  */
  let r#type: c_int = dump_reloc_type_of! (reloc);
  match r#type
    {
      x if x == dump_reloc_type::RELOC_DUMP_TO_EMACS_PTR_RAW as c_int
      => {
        let mut value: uintptr_t = dump_read_word_from_dump (dump_base, reloc_offset);
        eassert! (dump_reloc_size (reloc) as usize == core::mem::size_of_val (&value));
        /* PORTING.md P-96: C's `uintptr_t` addition *relies on wrapping* - the word in the
         dump is `emacs_offset (ptr)`, a signed offset, so a negative one is a huge unsigned
         here and adding the basis wraps back to the pointer (status 3.643).  */
      value = value.wrapping_add (emacs_basis ());
        dump_write_word_to_dump (dump_base, reloc_offset, value);
      }
      x if x == dump_reloc_type::RELOC_DUMP_TO_DUMP_PTR_RAW as c_int
      => {
        let mut value: uintptr_t = dump_read_word_from_dump (dump_base, reloc_offset);
        eassert! (dump_reloc_size (reloc) as usize == core::mem::size_of_val (&value));
        value += dump_base;
        dump_write_word_to_dump (dump_base, reloc_offset, value);
      }
      #[cfg (feature = "have_native_comp")]
      x if x == dump_reloc_type::RELOC_NATIVE_COMP_UNIT as c_int
      => {
	static mut installation_state: native_installation_state =
	  native_installation_state::UNKNOWN;
	let comp_u: *mut Lisp_Native_Comp_Unit =
	  dump_ptr (dump_base, reloc_offset) as *mut Lisp_Native_Comp_Unit;
	unsafe { (*comp_u).lambda_gc_guard_h
	           = CALLN! (Fmake_hash_table, QCtest (), Qeq ()); }
	if STRINGP (unsafe { (*comp_u).file })
	  { unsafe { error (c"trying to load incoherent dumped eln file %s".as_ptr (),
		            SSDATA ((*comp_u).file)); } }

	if !CONSP (unsafe { (*comp_u).file })
	  { unsafe { error (c"incoherent compilation unit for dump was dumped".as_ptr ()); } }

	/* emacs_execdir is always unibyte, but the file names in
	   comp_u->file could be multibyte, so we need to encode
	   them.  */
	let cu_file1: Lisp_Object = ENCODE_FILE (unsafe { XCAR ((*comp_u).file) });
	let cu_file2: Lisp_Object = ENCODE_FILE (unsafe { XCDR ((*comp_u).file) });
	let fn1_len: ptrdiff_t = SBYTES (cu_file1);
	let fn2_len: ptrdiff_t = SBYTES (cu_file2);
	let mut eln_fname: Lisp_Object;
	let mut fndata: *mut c_char;

	/* Check just once if this is a local build or Emacs was installed.  */
	/* Can't use expand-file-name here, because we are too early
	   in the startup, and we will crash at least on WINDOWSNT.  */
	if unsafe { installation_state } == native_installation_state::UNKNOWN
	  {
	    eln_fname = unsafe { make_uninit_string (execdir_len + fn1_len) };
	    unsafe {
	      fndata = SSDATA (eln_fname);
	      memcpy (fndata as *mut c_void, emacs_execdir as *const c_void,
	              execdir_len as usize);
	      memcpy (fndata.offset (execdir_len) as *mut c_void,
	              SSDATA (cu_file1) as *const c_void, fn1_len as usize);
	    }
	    if unsafe { file_access_p (fndata, F_OK) }
	      { unsafe { installation_state = native_installation_state::INSTALLED; } }
	    else
	      {
		eln_fname = unsafe { make_uninit_string (execdir_len + fn2_len) };
		unsafe {
		  fndata = SSDATA (eln_fname);
		  memcpy (fndata as *mut c_void, emacs_execdir as *const c_void,
		          execdir_len as usize);
		  memcpy (fndata.offset (execdir_len) as *mut c_void,
		          SSDATA (cu_file2) as *const c_void, fn2_len as usize);
		  installation_state = native_installation_state::LOCAL_BUILD;
		}
	      }
	    unsafe { fixup_eln_load_path (eln_fname); }
	  }
	else
	  {
	    let fn_len: ptrdiff_t =
	      if unsafe { installation_state } == native_installation_state::INSTALLED
		{ fn1_len } else { fn2_len };
	    let cu_file: Lisp_Object =
	      if unsafe { installation_state } == native_installation_state::INSTALLED
		{ cu_file1 } else { cu_file2 };
	    eln_fname = unsafe { make_uninit_string (execdir_len + fn_len) };
	    unsafe {
	      fndata = SSDATA (eln_fname);
	      memcpy (fndata as *mut c_void, emacs_execdir as *const c_void,
	              execdir_len as usize);
	      memcpy (fndata.offset (execdir_len) as *mut c_void,
	              SSDATA (cu_file) as *const c_void, fn_len as usize);
	    }
	  }

	/* FIXME: This records the names of the *.eln files in an
	   unexpanded form, with one or more ".." elements (and on
	   Windows with the first part using backslashes).  The file
	   names are also unibyte.  If we care about this, we need to
	   loop in startup.el over all the preloaded modules and run
	   their file names through expand-file-name and
	   decode-coding-string.  */
	unsafe { (*comp_u).file = eln_fname; }
	unsafe { (*comp_u).handle = dynlib_open_for_eln (SSDATA (eln_fname)); }
	if unsafe { (*comp_u).handle.is_null () }
	  {
	    unsafe { fprintf (stderr, c"Error using execdir %s:\n".as_ptr (), emacs_execdir); }
	    unsafe { error (c"%s".as_ptr (), dynlib_error ()); }
	  }
	unsafe { load_comp_unit (comp_u, true, false); }
      }
      #[cfg (feature = "have_native_comp")]
      x if x == dump_reloc_type::RELOC_NATIVE_SUBR as c_int
      => {
	/* When resurrecting from a dump given non all the original
	   native-compiled subrs may be still around we can't rely on
	   a 'top_level_run' mechanism, we revive them one-by-one
	   here.  */
	let subr: *mut Lisp_Subr = dump_ptr (dump_base, reloc_offset) as *mut Lisp_Subr;
	let comp_u: *mut Lisp_Native_Comp_Unit = unsafe {
	  XNATIVE_COMP_UNIT ((*subr).native_comp_u) };
	if unsafe { (*comp_u).handle.is_null () }
	  { unsafe { error (c"NULL handle in compilation unit %s".as_ptr (),
		            SSDATA ((*comp_u).file)); } }
	let c_name: *const c_char = unsafe { (*subr).native_c_name };
	eassert! (!c_name.is_null ());
	let func: *mut c_void = unsafe { dynlib_sym ((*comp_u).handle, c_name) };
	if func.is_null ()
	  { unsafe { error (c"can't find function \"%s\" in compilation unit %s".as_ptr (),
		            c_name, SSDATA ((*comp_u).file)); } }
	unsafe { (*subr).function.a0 = Some (core::mem::transmute (func)); }
	let lambda_data_idx: Lisp_Object =
	  unsafe { Fgethash (build_string (c_name),
			     (*comp_u).lambda_c_name_idx_h, Qnil ()) };
	if !NILP (lambda_data_idx)
	  {
	    /* This is an anonymous lambda.
	       We must fixup d_reloc so the lambda can be referenced
	       by code.  */
	    let mut tem: Lisp_Object = Qnil ();
	    XSETSUBR! (tem, subr);
	    let fixup: *mut Lisp_Object =
	      unsafe { (*comp_u).data_relocs.offset (XFIXNUM (lambda_data_idx)) };
	    eassert! (EQ (unsafe { *fixup }, Vcomp__hashdollar));
	    unsafe { *fixup = tem; }
	    unsafe { Fputhash (tem, Qt (), (*comp_u).lambda_gc_guard_h); }
	  }
      }
      x if x == dump_reloc_type::RELOC_BIGNUM as c_int
      => {
        let bignum: *mut Lisp_Bignum = dump_ptr (dump_base, reloc_offset) as *mut Lisp_Bignum;
        let mut reload_info: bignum_reload_info = unsafe { core::mem::zeroed () };
        /* C 5501: `static_assert (sizeof (reload_info) <= sizeof (*bignum_val (bignum)));`
           — a type-level assertion in the port (the `ctx`-free shape).  */
	static_assert! (core::mem::size_of::<bignum_reload_info> ()
	                <= core::mem::size_of::<[__mpz_struct; 1]> ());
        unsafe {
          memcpy ((&raw mut reload_info) as *mut c_void,
                  bignum_val (bignum) as *const c_void,
                  core::mem::size_of_val (&reload_info));
        }
        let limbs: *const mp_limb_t = dump_ptr (dump_base, reload_info.data_location)
          as *const mp_limb_t;
        unsafe { mpz_roinit_n ((&raw mut (*bignum).value) as *mut __mpz_struct, limbs,
                               reload_info.nlimbs as mp_size_t); }
      }
      _ /* Lisp_Object in the dump; precise type in reloc.type */
      => {
        let lv: Lisp_Object = dump_make_lv_from_reloc (dump_base, reloc);
        eassert! (dump_reloc_size (reloc) as usize == core::mem::size_of_val (&lv));
        dump_write_lv_to_dump (dump_base, reloc_offset, lv);
      }
    }
}

fn dump_do_all_dump_reloc_for_phase (header: *const dump_header, dump_base: uintptr_t,
				     phase: reloc_phase)
{
  let r: *mut dump_reloc = dump_ptr (dump_base,
				     unsafe {
                                       (*header).dump_relocs[phase as usize].offset })
    as *mut dump_reloc;
  let nr_entries: dump_off =
    unsafe { (*header).dump_relocs[phase as usize].nr_entries };
  let mut i: dump_off = 0;
  while i < nr_entries
    {
      dump_do_dump_relocation (dump_base, unsafe { *r.offset (i as isize) });
      i += 1;
    }
}

fn dump_do_emacs_relocation (dump_base: uintptr_t, reloc: emacs_reloc)
{
  let mut pval: ptrdiff_t;
  let lv: Lisp_Object;

  /* C's `switch (reloc.type)`; `length` is C's other bitfield read (reloc.length).  */
  let r#type: c_int = emacs_reloc_type_of! (reloc);
  let length: dump_off = emacs_reloc_length! (reloc);
  match r#type
    {
      x if x == emacs_reloc_type::RELOC_EMACS_COPY_FROM_DUMP as c_int
      => {
        eassume! (length > 0);
        unsafe {
          memcpy (emacs_ptr_at (reloc.emacs_offset as ptrdiff_t),
                  dump_ptr (dump_base, reloc.u.dump_offset),
                  length as usize);
        }
      }
      x if x == emacs_reloc_type::RELOC_EMACS_IMMEDIATE as c_int
      => {
        eassume! (length > 0);
        eassume! (length as usize <= core::mem::size_of::<intmax_t> ());
        unsafe {
          memcpy (emacs_ptr_at (reloc.emacs_offset as ptrdiff_t),
                  (&raw const reloc.u.immediate) as *const c_void,
                  length as usize);
        }
      }
      x if x == emacs_reloc_type::RELOC_EMACS_DUMP_PTR_RAW as c_int
      => {
        /* C 5535: `pval = reloc.u.dump_offset + dump_base;` - `dump_off` (int32) converts to
           `uintptr_t` and the sum wraps (P-96); the port must not narrow `dump_base` to
           `dump_off` (that truncates the address) nor add the two in 32 bits.  */
        pval = (unsafe { reloc.u.dump_offset } as uintptr_t)
          .wrapping_add (dump_base) as ptrdiff_t;
        unsafe {
          memcpy (emacs_ptr_at (reloc.emacs_offset as ptrdiff_t), (&raw const pval) as *const c_void,
                  core::mem::size_of_val (&pval));
        }
      }
      x if x == emacs_reloc_type::RELOC_EMACS_EMACS_PTR_RAW as c_int
      => {
        /* C 5539: `pval = reloc.u.emacs_offset2 + emacs_basis ();` - the same conversion and
           the same wrap as C 5535 (P-96).  */
        pval = (unsafe { reloc.u.emacs_offset2 } as uintptr_t)
          .wrapping_add (emacs_basis ()) as ptrdiff_t;
        unsafe {
          memcpy (emacs_ptr_at (reloc.emacs_offset as ptrdiff_t), (&raw const pval) as *const c_void,
                  core::mem::size_of_val (&pval));
        }
      }
      x if x == emacs_reloc_type::RELOC_EMACS_DUMP_LV as c_int
           || x == emacs_reloc_type::RELOC_EMACS_EMACS_LV as c_int
      => {
        /* Lisp_Float is the maximum lisp type.  */
        eassume! (length <= Lisp_Type::Lisp_Float as dump_off);
        let obj_ptr: *mut c_void =
          if r#type == emacs_reloc_type::RELOC_EMACS_DUMP_LV as c_int
            { dump_ptr (dump_base, unsafe { reloc.u.dump_offset }) }
            else { emacs_ptr_at (unsafe { reloc.u.emacs_offset2 } as ptrdiff_t) };
        if length == Lisp_Type::Lisp_Symbol as dump_off
          { lv = make_lisp_symbol (obj_ptr as *mut Lisp_Symbol); }
        else
          { lv = make_lisp_ptr (obj_ptr,
                                unsafe { core::mem::transmute::<c_int, Lisp_Type> (length) }); }
        unsafe {
          memcpy (emacs_ptr_at (reloc.emacs_offset as ptrdiff_t), (&raw const lv) as *const c_void,
                  core::mem::size_of_val (&lv));
        }
      }
      _ => {
        unsafe { fatal (c"unrecognied relocation type %d".as_ptr (), r#type); }
      }
    }
}

fn dump_do_all_emacs_relocations (header: *const dump_header, dump_base: uintptr_t)
{
  let nr_entries: dump_off = unsafe { (*header).emacs_relocs.nr_entries };
  let r: *mut emacs_reloc = dump_ptr (dump_base, unsafe { (*header).emacs_relocs.offset })
    as *mut emacs_reloc;
  let mut i: dump_off = 0;
  while i < nr_entries
    {
      dump_do_emacs_relocation (dump_base, unsafe { *r.offset (i as isize) });
      i += 1;
    }
}

/* C 5572-5604's `#ifdef HAVE_NATIVE_COMP` arm is dead; carried (P-51).  */
#[cfg (feature = "have_native_comp")]
static mut emacs_execdir: *mut c_char = core::ptr::null_mut ();
#[cfg (feature = "have_native_comp")]
static mut execdir_size: ptrdiff_t = 0;
#[cfg (feature = "have_native_comp")]
static mut execdir_len: ptrdiff_t = 0;

#[cfg (feature = "have_native_comp")]
fn pdumper_set_emacs_execdir (emacs_executable: *mut c_char)
{
  unsafe {
    let mut p: *mut c_char = emacs_executable.add (strlen (emacs_executable));
    while p > emacs_executable
	  && !IS_DIRECTORY_SEP (*p.offset (-1))
      { p = p.offset (-1); }
    eassert! (p > emacs_executable);

    /* C 5585-5597's `#if HAVE_NS && !NS_SELF_CONTAINED` arm (dead: `HAVE_NS` is undef on
       this target) is carried with the live `#else` arm below.  */
    emacs_execdir = xpalloc (emacs_execdir as *mut c_void, &raw mut execdir_size,
			     p.offset_from (emacs_executable) as ptrdiff_t + 1 - execdir_size,
			     -1, 1) as *mut c_char;
    memcpy (emacs_execdir as *mut c_void, emacs_executable as *const c_void,
	    p.offset_from (emacs_executable) as usize);
    execdir_len = p.offset_from (emacs_executable) as ptrdiff_t;
    *emacs_execdir.offset (execdir_len) = 0;
  }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum dump_section
  {
   DS_HOT,
   DS_DISCARDABLE,
   DS_COLD,
   NUMBER_DUMP_SECTIONS,
  }

/* Pointer to a stack variable to avoid having to staticpro it.  */
static mut pdumper_hashes: *mut Lisp_Object = (&raw mut zero_vector) as *mut Lisp_Object;

/* Load a dump from DUMP_FILENAME.  Return an error code.

   N.B. We run very early in initialization, so we can't use lisp,
   unwinding, xmalloc, and so on.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_load (dump_filename: *const c_char, argv0: *mut c_char) -> c_int
{
  let mut dump_size: intptr_t = 0;
  let mut stat: stat = unsafe { core::mem::zeroed () };
  let dump_base: uintptr_t;
  let dump_page_size: c_int;
  let mut adj_discardable_start: dump_off;

  let mut mark_bits: [dump_bitset; 2] =
    [dump_bitset { bits: core::ptr::null_mut (), number_words: 0 }; 2];
  let mark_bits_needed: size_t;

  let mut header_buf: dump_header = unsafe { core::mem::zeroed () };
  let header: *mut dump_header = &mut header_buf;
  let mut sections: [dump_memory_map; dump_section::NUMBER_DUMP_SECTIONS as usize] =
    [dump_memory_map {
       spec: dump_memory_map_spec { fd: 0, size: 0, offset: 0,
                                    protection: dump_memory_protection::DUMP_MEMORY_ACCESS_NONE },
       mapping: core::ptr::null_mut (),
       release: None,
       private: core::ptr::null_mut () };
     dump_section::NUMBER_DUMP_SECTIONS as usize];

  let start_time: timespec = unsafe { current_timespec () };
  let mut dump_filename_copy: *mut c_char;

  /* Overwriting an initialized Lisp universe will not go well.  */
  eassert! (!unsafe { initialized });

  /* We can load only one dump.  */
  eassert! (!dump_loaded_p ());

  let mut err: c_int;
  let dump_fd: c_int = unsafe { emacs_open_noquit (dump_filename, O_RDONLY, 0) };
  /* C's `goto out` targets become `break 'out` of the single `'out` block below, which
     holds C's cleanup statements; the failure paths keep their order and their `err`
     assignments (PORTING.md 2.2).  */
  'out: {
  if dump_fd < 0
    {
      unsafe {
        err = if *errno () == ENOENT || *errno () == ENOTDIR
	        { pdumper_load_result::PDUMPER_LOAD_FILE_NOT_FOUND as c_int }
	        else { pdumper_load_result::PDUMPER_LOAD_ERROR as c_int + *errno () };
      }
      break 'out;
    }

  err = pdumper_load_result::PDUMPER_LOAD_FILE_NOT_FOUND as c_int;
  if unsafe { sys_fstat (dump_fd, &mut stat) } < 0
    { break 'out; }

  err = pdumper_load_result::PDUMPER_LOAD_BAD_FILE_TYPE as c_int;
  if unsafe { stat.st_size } > INTPTR_MAX as off_t
    { break 'out; }
  dump_size = unsafe { stat.st_size } as intptr_t;

  err = pdumper_load_result::PDUMPER_LOAD_BAD_FILE_TYPE as c_int;
  if dump_size < core::mem::size_of_val (unsafe { &*header }) as intptr_t
    { break 'out; }

  err = pdumper_load_result::PDUMPER_LOAD_BAD_FILE_TYPE as c_int;
  let header_read: i64 = unsafe { dump_read_all (dump_fd, header as *mut c_void,
                                                 core::mem::size_of_val (&*header)) } as i64;
  if header_read < core::mem::size_of_val (unsafe { &*header }) as i64
    { break 'out; }

  let magic_differs: bool = unsafe {
    memcmp ((*header).magic.as_ptr () as *const c_void,
            (&raw const dump_magic) as *const c_void,
            core::mem::size_of_val (&dump_magic))
  } != 0;
  if magic_differs
    {
      let magic_ok_after_patch: bool = unsafe {
        (*header).magic[0] == b'!' as c_char
          && { (*header).magic[0] = dump_magic[0];
               memcmp ((*header).magic.as_ptr () as *const c_void,
                       (&raw const dump_magic) as *const c_void,
                       core::mem::size_of_val (&dump_magic)) == 0 }
      };
      if magic_ok_after_patch
        {
          err = pdumper_load_result::PDUMPER_LOAD_FAILED_DUMP as c_int;
          break 'out;
        }
      err = pdumper_load_result::PDUMPER_LOAD_BAD_FILE_TYPE as c_int;
      break 'out;
    }

  err = pdumper_load_result::PDUMPER_LOAD_VERSION_MISMATCH as c_int;
  /* C 5677: `static_assert (sizeof (header->fingerprint) == sizeof (fingerprint));` — a
     type-level assertion in the port (the `header`-free shape; both names appear in this
     body anyway).  */
  static_assert! (FINGERPRINT_SIZE == FINGERPRINT_SIZE);
  let mut desired: [c_uchar; 32 /* sizeof fingerprint */] = [0; 32];
  let mut i: c_int = 0;
  while (i as usize) < core::mem::size_of_val (unsafe { &fingerprint })
    {
      desired[i as usize] = unsafe { fingerprint[i as usize] };
      i += 1;
    }
  if unsafe {
       memcmp ((*header).fingerprint.as_ptr () as *const c_void,
               desired.as_ptr () as *const c_void, core::mem::size_of_val (&desired))
     } != 0
    {
      unsafe {
        dump_fingerprint (stderr, c"desired fingerprint".as_ptr (),
                          (&raw const desired) as *const c_uchar);
        dump_fingerprint (stderr, c"found fingerprint".as_ptr (),
                          (&raw const (*header).fingerprint) as *const c_uchar);
      }
      break 'out;
    }

  /* FIXME: The comment at the start of this function says it should
     not use xmalloc, but xstrdup calls xmalloc.  Either fix the
     comment or fix the following code.  */
  dump_filename_copy = unsafe { xstrdup (dump_filename) };

  err = pdumper_load_result::PDUMPER_LOAD_OOM as c_int;

  adj_discardable_start = unsafe { (*header).discardable_start };
  dump_page_size = dump_get_max_page_size ();
  /* Snap to next page boundary.  */
  adj_discardable_start = ROUNDUP (adj_discardable_start as isize,
                                   dump_page_size as isize) as dump_off;
  eassert! (adj_discardable_start % dump_page_size == 0);
  eassert! (adj_discardable_start <= unsafe { (*header).cold_start });

  sections[dump_section::DS_HOT as usize].spec = dump_memory_map_spec {
     fd: dump_fd,
     size: adj_discardable_start as size_t,
     offset: 0,
     protection: dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE,
    };

  sections[dump_section::DS_DISCARDABLE as usize].spec = dump_memory_map_spec {
     fd: dump_fd,
     size: (unsafe { (*header).cold_start } - adj_discardable_start) as size_t,
     offset: adj_discardable_start as off_t,
     protection: dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE,
    };

  sections[dump_section::DS_COLD as usize].spec = dump_memory_map_spec {
     fd: dump_fd,
     size: (dump_size - unsafe { (*header).cold_start } as intptr_t) as size_t,
     offset: unsafe { (*header).cold_start } as off_t,
     protection: dump_memory_protection::DUMP_MEMORY_ACCESS_READWRITE,
    };

  if !dump_mmap_contiguous (sections.as_mut_ptr (),
                            dump_section::NUMBER_DUMP_SECTIONS as c_int)
    { break 'out; }

  err = pdumper_load_result::PDUMPER_LOAD_ERROR as c_int;
  dump_base = sections[dump_section::DS_HOT as usize].mapping as uintptr_t;

  /* C 5741-5753's `#if !USE_LSB_TAG` arm is dead (`USE_LSB_TAG` is 1); carried (P-51).  */
  #[cfg (feature = "no_use_lsb_tag")]
  {
    /* The dump may have been mapped at a location that does not admit of
       representation as Lisp_Objects.  Abort in this case.  */
    if ((dump_base + dump_size as uintptr_t) & !(VALMASK as uintptr_t)) != 0
      {
        unsafe {
          fprintf (stderr,
	           c"Failed to load dump file: 0x%p+0x%p & ~0x%p != 0\n".as_ptr (),
	           dump_base as *const c_void, dump_size as *const c_void,
	           VALMASK as uintptr_t as *const c_void);
        }
        break 'out;
      }
  }

  mark_bits_needed
    = divide_round_up (unsafe { (*header).discardable_start } as size_t,
                       DUMP_ALIGNMENT as size_t);
  if !dump_bitsets_init (mark_bits.as_mut_ptr (), mark_bits_needed)
    { break 'out; }

  /* Point of no return.  */
  err = pdumper_load_result::PDUMPER_LOAD_SUCCESS as c_int;
  unsafe { gflags.dumped_with_pdumper = true; }
  unsafe { dump_private.header = *header; }
  unsafe { dump_private.mark_bits = mark_bits[0]; }
  unsafe { dump_private.last_mark_bits = mark_bits[1]; }
  unsafe {
    dump_public.start = dump_base;
    dump_public.end = dump_public.start + dump_size as uintptr_t;
  }

  dump_do_all_dump_reloc_for_phase (header, dump_base, reloc_phase::EARLY_RELOCS);
  dump_do_all_emacs_relocations (header, dump_base);

  dump_mmap_discard_contents (unsafe { &raw mut sections[dump_section::DS_DISCARDABLE as usize] });
  let mut i: c_int = 0;
  while i < dump_section::NUMBER_DUMP_SECTIONS as c_int
    {
      dump_mmap_reset (unsafe { &raw mut sections[i as usize] });
      i += 1;
    }

  let mut hashes: Lisp_Object = unsafe { zero_vector };
  if unsafe { (*header).hash_list } != 0
    {
      let hash_tables: *mut Lisp_Vector =
	(unsafe { dump_base } + unsafe { (*header).hash_list } as uintptr_t)
	  as *mut Lisp_Vector;
      hashes = make_lisp_ptr (hash_tables as *mut c_void, Lisp_Type::Lisp_Vectorlike);
    }

  unsafe { pdumper_hashes = &raw mut hashes; }
  /* Run the functions Emacs registered for doing post-dump-load
     initialization.  */
  let mut i: c_int = 0;
  while i < unsafe { nr_dump_hooks }
    {
      unsafe { dump_hooks[i as usize].unwrap () () };
      i += 1;
    }

  /* C 5789-5793's `#ifdef HAVE_NATIVE_COMP` call is dead; the `#else` arm is the live
     text.  */
  #[cfg (feature = "have_native_comp")]
  { pdumper_set_emacs_execdir (argv0); }
  #[cfg (not (feature = "have_native_comp"))]
  { let _ = argv0; }

  dump_do_all_dump_reloc_for_phase (header, dump_base, reloc_phase::LATE_RELOCS);
  dump_do_all_dump_reloc_for_phase (header, dump_base, reloc_phase::VERY_LATE_RELOCS);

  /* Run the functions Emacs registered for doing post-dump-load
     initialization.  */
  let mut i: c_int = 0;
  while i < unsafe { nr_dump_late_hooks }
    {
      unsafe { dump_late_hooks[i as usize].unwrap () () };
      i += 1;
    }

  unsafe { initialized = true; }

  let load_timespec: timespec =
    timespec_sub (unsafe { current_timespec () }, start_time);
  unsafe { dump_private.load_time = timespectod (load_timespec); }
  unsafe { dump_private.dump_filename = dump_filename_copy; }

  }

  let mut i: c_int = 0;
  while i < dump_section::NUMBER_DUMP_SECTIONS as c_int
    {
      dump_mmap_release (unsafe { &raw mut sections[i as usize] });
      i += 1;
    }
  if dump_fd >= 0
    { unsafe { emacs_close (dump_fd) }; }
  return err;
}

/* Prepend the Emacs startup directory to dump_filename, if that is
   relative, so that we could later make it absolute correctly.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pdumper_record_wd (wd: *const c_char)
{
  unsafe {
    if !wd.is_null () && !file_name_absolute_p (dump_private.dump_filename)
      {
        let dfn: *mut c_char = xmalloc ((strlen (wd) + 1
			                 + strlen (dump_private.dump_filename) + 1)
                                        as size_t) as *mut c_char;
        splice_dir_file (dfn, wd, dump_private.dump_filename);
        xfree (dump_private.dump_filename as *mut c_void);
        dump_private.dump_filename = dfn;
      }
  }
}

DEFUN! ("pdumper-stats", Fpdumper_stats, Spdumper_stats, 0, 0, 0,
       doc: "Return statistics about portable dumping used by this session.\nIf this Emacs session was started from a dump file,\nthe return value is an alist of the form:\n\n  ((dumped-with-pdumper . t) (load-time . TIME) (dump-file-name . FILE))\n\nwhere TIME is the time in seconds it took to restore Emacs state\nfrom the dump file, and FILE is the name of the dump file.\nValue is nil if this session was not started using a dump file.",
       (),
       {
	 if !dumped_with_pdumper_p ()
	   { return Qnil (); }

	 let mut dump_fn: Lisp_Object;
	 /* C 5851-5858's `#ifdef WINDOWSNT` arm is dead; the `#else` arm is the live text.  */
	 #[cfg (feature = "windowsnt")]
	 {
	   let mut dump_fn_utf8: [c_char; MAX_UTF8_PATH as usize] =
	     [0; MAX_UTF8_PATH as usize];
	   if filename_from_ansi (unsafe { dump_private.dump_filename },
				  dump_fn_utf8.as_mut_ptr ()) == 0
	     { dump_fn = DECODE_FILE (build_unibyte_string (dump_fn_utf8.as_ptr ())); }
	   else
	     { dump_fn = build_unibyte_string (unsafe { dump_private.dump_filename }); }
	 }
	 #[cfg (not (feature = "windowsnt"))]
	 { dump_fn = DECODE_FILE (build_unibyte_string (unsafe { dump_private.dump_filename })); }

	 dump_fn = Fexpand_file_name (dump_fn, Qnil ());

	 return list3 (Fcons (Qdumped_with_pdumper (), Qt ()),
		       Fcons (Qload_time (), make_float (unsafe { dump_private.load_time })),
		       Fcons (Qdump_file_name (), dump_fn));
       });

unsafe extern "C" fn thaw_hash_tables ()
{
  let hash_tables: Lisp_Object = unsafe { *pdumper_hashes };
  let mut i: ptrdiff_t = 0;
  while i < ASIZE (hash_tables)
    {
      unsafe { hash_table_thaw (AREF (hash_tables, i)); }
      i += 1;
    }
}

/* C 5873's `#endif /* HAVE_PDUMPER */` closes the `#ifdef` opened at C 71.  */

/* C 5880's `#ifdef HAVE_PDUMPER` inside `init_pdumper_once` is live (`HAVE_PDUMPER` is 1);
   its `#else` arm is empty.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_pdumper_once ()
{
  pdumper_do_now_and_after_load (Some (thaw_hash_tables));
}

/* The `Vpdumper_fingerprint` carrier C's `DEFVAR_LISP` writes lives in this unit's
   `globals.f_pdumper_fingerprint`, so the port defines the object here (docs/status.md
   3.29/3.60: `#[unsafe(no_mangle)] pub static mut V*` by the unit that owns it).  */
#[unsafe(no_mangle)]
pub static mut Vpdumper_fingerprint: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_pdumper ()
{
  /* C 5883-5919's `#ifdef HAVE_PDUMPER` is live; its `#else` arm is empty.  */
  let mut desired: [c_uchar; 32 /* sizeof fingerprint */] = [0; 32];
  let mut i: c_int;
  let mut hexbuf: [c_char; 64 /* 2 * sizeof fingerprint */] = [0; 64];

  unsafe {
    defsubr (&raw mut Sdump_emacs_portable);
    defsubr (&raw mut Sdump_emacs_portable__sort_predicate);
    defsubr (&raw mut Sdump_emacs_portable__sort_predicate_copied);
    DEFSYM! (Qdump_emacs_portable__sort_predicate,
             "dump-emacs-portable--sort-predicate");
    DEFSYM! (Qdump_emacs_portable__sort_predicate_copied,
             "dump-emacs-portable--sort-predicate-copied");
    DEFSYM! (Qdumped_with_pdumper, "dumped-with-pdumper");
    DEFSYM! (Qload_time, "load-time");
    DEFSYM! (Qdump_file_name, "dump-file-name");
    DEFSYM! (Qafter_pdump_load_hook, "after-pdump-load-hook");
    defsubr (&raw mut Spdumper_stats);

    i = 0;
    while (i as usize) < core::mem::size_of_val (&fingerprint)
      {
        desired[i as usize] = fingerprint[i as usize];
        i += 1;
      }

    hexbuf_digest (hexbuf.as_mut_ptr (), desired.as_ptr () as *const c_void,
                   core::mem::size_of_val (&desired) as c_int);
  }

  DEFVAR_LISP! (c"pdumper-fingerprint", Vpdumper_fingerprint,
	        "The fingerprint of this Emacs binary.\nIt is a string that is supposed to be unique to each build of\nEmacs.  ");
  unsafe {
    Vpdumper_fingerprint = make_unibyte_string (hexbuf.as_ptr (),
                                                core::mem::size_of_val (&hexbuf) as ptrdiff_t);
  }
}

/* ---------------------------------------------------------------------------
   Layout: every type this unit declares, frozen by clang on the frozen target.

   Measured with a probe that includes the frozen configure
   (`clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim
   -I build/emacs-ref/src -I emacs/src -I emacs/lib`, `#include <config.h>` first) and the
   *preprocessed* text of this file's own records — `clang -E` on
   `emacs/src/pdumper.c`, so the definitions the probe measures are C's own, not a
   transcription.  The probe's output (sizeof/alignof/offsetof, plus a byte dump of every
   bitfield unit after setting its flags) is recorded in docs/ports/pdumper.md.
   `xtask layout-check` does not cover these types (they are not in
   tools/gen-layout-fields.py's STRUCTS, the keymap.rs precedent), so the assertions are
   here.
   --------------------------------------------------------------------------- */

const _: () = core::assert! (core::mem::size_of::<dump_off> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_reloc_type> () == 4);
const _: () = core::assert! (core::mem::align_of::<dump_reloc_type> () == 4);
const _: () = core::assert! (core::mem::size_of::<emacs_reloc_type> () == 4);
const _: () = core::assert! (core::mem::align_of::<emacs_reloc_type> () == 4);
const _: () = core::assert! (core::mem::size_of::<reloc_phase> () == 4);
const _: () = core::assert! (core::mem::align_of::<reloc_phase> () == 4);
const _: () = core::assert! (core::mem::size_of::<cold_op> () == 4);
const _: () = core::assert! (core::mem::align_of::<cold_op> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_object_special_offset> () == 4);
const _: () = core::assert! (core::mem::align_of::<dump_object_special_offset> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_memory_protection> () == 4);
const _: () = core::assert! (core::mem::align_of::<dump_memory_protection> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_section> () == 4);
const _: () = core::assert! (core::mem::align_of::<dump_section> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_fixup_type> () == 4);
const _: () = core::assert! (core::mem::size_of::<dump_lv_fixup_type> () == 4);
/* C's `struct dump_remembered_data` is the file's anonymous `struct { void *mem; int sz; }`
   (measured through `sizeof remembered_data / 32`).  */
const _: () = core::assert! (core::mem::size_of::<dump_remembered_data> () == 16);
const _: () = core::assert! (core::mem::align_of::<dump_remembered_data> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_remembered_data, mem) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_remembered_data, sz) == 8);

/* emacs_reloc: 16/8, and its bitfield unit is the 4-byte word at offset 0 (type in bits
   0-2, the signed 29-bit length above it; measured with `type = RELOC_EMACS_EMACS_LV`,
   `length = -3` -> bytes `ed ff ff ff`).  */
const _: () = core::assert! (core::mem::size_of::<emacs_reloc> () == 16);
const _: () = core::assert! (core::mem::align_of::<emacs_reloc> () == 8);
const _: () = core::assert! (core::mem::offset_of! (emacs_reloc, emacs_offset) == 4);
const _: () = core::assert! (core::mem::offset_of! (emacs_reloc, u) == 8);
const _: () = core::assert! (core::mem::size_of::<emacs_reloc_u> () == 8);
const _: () = core::assert! (EMACS_RELOC_TYPE_BITS == 3);
const _: () = core::assert! (EMACS_RELOC_LENGTH_BITS == 29);
const _: () = core::assert! ((1u32 << EMACS_RELOC_TYPE_BITS as u32) == 8);

/* dump_reloc: 4/4 (C's `static_assert (sizeof (struct dump_reloc) == sizeof (dump_off))`),
   raw_offset in the low 27 bits, the 5-bit type above it (measured: raw_offset
   0x123456, type RELOC_DUMP_TO_EMACS_LV -> bytes `56 34 12 68`).  */
const _: () = core::assert! (core::mem::size_of::<dump_reloc> () == 4);
const _: () = core::assert! (core::mem::align_of::<dump_reloc> () == 4);
const _: () = core::assert! (DUMP_RELOC_TYPE_BITS == 5);
const _: () = core::assert! (DUMP_RELOC_OFFSET_BITS == 27);
const _: () = core::assert! ((1u32 << DUMP_RELOC_OFFSET_BITS as u32) == 0x0800_0000);

const _: () = core::assert! (core::mem::size_of::<dump_table_locator> () == 8);
const _: () = core::assert! (core::mem::align_of::<dump_table_locator> () == 4);
const _: () = core::assert! (core::mem::offset_of! (dump_table_locator, offset) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_table_locator, nr_entries) == 4);

const _: () = core::assert! (core::mem::size_of::<dump_header> () == 100);
const _: () = core::assert! (core::mem::align_of::<dump_header> () == 4);
const _: () = core::assert! (core::mem::offset_of! (dump_header, magic) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_header, fingerprint) == 16);
const _: () = core::assert! (core::mem::offset_of! (dump_header, dump_relocs) == 48);
const _: () = core::assert! (core::mem::offset_of! (dump_header, object_starts) == 72);
const _: () = core::assert! (core::mem::offset_of! (dump_header, emacs_relocs) == 80);
const _: () = core::assert! (core::mem::offset_of! (dump_header, discardable_start) == 88);
const _: () = core::assert! (core::mem::offset_of! (dump_header, cold_start) == 92);
const _: () = core::assert! (core::mem::offset_of! (dump_header, hash_list) == 96);

const _: () = core::assert! (core::mem::size_of::<dump_tailq> () == 24);
const _: () = core::assert! (core::mem::align_of::<dump_tailq> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_tailq, head) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_tailq, tail) == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_tailq, length) == 16);

const _: () = core::assert! (core::mem::size_of::<dump_queue> () == 120);
const _: () = core::assert! (core::mem::align_of::<dump_queue> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, zero_weight_objects) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, one_weight_normal_objects) == 24);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, one_weight_strong_objects) == 48);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, fancy_weight_objects) == 72);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, link_weights) == 96);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, sequence_numbers) == 104);
const _: () = core::assert! (core::mem::offset_of! (dump_queue, next_sequence_number) == 112);

/* dump_flags: one byte, one bit per member in declaration order (measured: the three set
   flags `dump_object_contents`, `defer_symbols`, `defer_copied_objects` -> byte `a1`).  */
const _: () = core::assert! (core::mem::size_of::<dump_flags> () == 1);
const _: () = core::assert! (core::mem::align_of::<dump_flags> () == 1);
const _: () = core::assert! (DUMP_FLAGS_DUMP_OBJECT_CONTENTS == 0b0000_0001);
const _: () = core::assert! (DUMP_FLAGS_RECORD_OBJECT_STARTS == 0b0000_0010);
const _: () = core::assert! (DUMP_FLAGS_PACK_OBJECTS == 0b0000_0100);
const _: () = core::assert! (DUMP_FLAGS_ASSERT_ALREADY_SEEN == 0b0000_1000);
const _: () = core::assert! (DUMP_FLAGS_DEFER_HASH_TABLES == 0b0001_0000);
const _: () = core::assert! (DUMP_FLAGS_DEFER_SYMBOLS == 0b0010_0000);
const _: () = core::assert! (DUMP_FLAGS_DEFER_COLD_OBJECTS == 0b0100_0000);
const _: () = core::assert! (DUMP_FLAGS_DEFER_COPIED_OBJECTS == 0b1000_0000);

const _: () = core::assert! (core::mem::size_of::<dump_context> () == 448);
const _: () = core::assert! (core::mem::align_of::<dump_context> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_context, header) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_context, buf) == 104);
const _: () = core::assert! (core::mem::offset_of! (dump_context, buf_size) == 112);
const _: () = core::assert! (core::mem::offset_of! (dump_context, max_offset) == 116);
const _: () = core::assert! (core::mem::offset_of! (dump_context, old_purify_flag) == 120);
const _: () = core::assert! (core::mem::offset_of! (dump_context, old_post_gc_hook) == 128);
const _: () = core::assert! (core::mem::offset_of! (dump_context, old_process_environment) == 136);
const _: () = core::assert! (core::mem::offset_of! (dump_context, fd) == 144);
const _: () = core::assert! (core::mem::offset_of! (dump_context, dump_filename) == 152);
const _: () = core::assert! (core::mem::offset_of! (dump_context, offset) == 160);
const _: () = core::assert! (core::mem::offset_of! (dump_context, obj_offset) == 164);
const _: () = core::assert! (core::mem::offset_of! (dump_context, flags) == 168);
const _: () = core::assert! (core::mem::offset_of! (dump_context, end_heap) == 172);
const _: () = core::assert! (core::mem::offset_of! (dump_context, objects_dumped) == 176);
const _: () = core::assert! (core::mem::offset_of! (dump_context, referrers) == 184);
const _: () = core::assert! (core::mem::offset_of! (dump_context, current_referrer) == 192);
const _: () = core::assert! (core::mem::offset_of! (dump_context, have_current_referrer) == 200);
const _: () = core::assert! (core::mem::offset_of! (dump_context, dump_queue) == 208);
const _: () = core::assert! (core::mem::offset_of! (dump_context, deferred_hash_tables) == 328);
const _: () = core::assert! (core::mem::offset_of! (dump_context, deferred_symbols) == 336);
const _: () = core::assert! (core::mem::offset_of! (dump_context, fixups) == 344);
const _: () = core::assert! (core::mem::offset_of! (dump_context, staticpro_table) == 352);
const _: () = core::assert! (core::mem::offset_of! (dump_context, symbol_aux) == 360);
const _: () = core::assert! (core::mem::offset_of! (dump_context, copied_queue) == 368);
const _: () = core::assert! (core::mem::offset_of! (dump_context, cold_queue) == 376);
const _: () = core::assert! (core::mem::offset_of! (dump_context, dump_relocs) == 384);
const _: () = core::assert! (core::mem::offset_of! (dump_context, object_starts) == 408);
const _: () = core::assert! (core::mem::offset_of! (dump_context, emacs_relocs) == 416);
const _: () = core::assert! (core::mem::offset_of! (dump_context, bignum_data) == 424);
const _: () = core::assert! (core::mem::offset_of! (dump_context, hash_tables) == 432);
const _: () = core::assert! (core::mem::offset_of! (dump_context, number_hot_relocations) == 440);
const _: () = core::assert! (core::mem::offset_of! (dump_context, number_discardable_relocations) == 444);

const _: () = core::assert! (core::mem::size_of::<link_weight> () == 4);
const _: () = core::assert! (core::mem::align_of::<link_weight> () == 4);
const _: () = core::assert! (core::mem::offset_of! (link_weight, value) == 0);

const _: () = core::assert! (core::mem::size_of::<dump_memory_map_spec> () == 32);
const _: () = core::assert! (core::mem::align_of::<dump_memory_map_spec> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_spec, fd) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_spec, size) == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_spec, offset) == 16);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_spec, protection) == 24);

const _: () = core::assert! (core::mem::size_of::<dump_memory_map> () == 56);
const _: () = core::assert! (core::mem::align_of::<dump_memory_map> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map, spec) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map, mapping) == 32);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map, release) == 40);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map, private) == 48);

const _: () = core::assert! (core::mem::size_of::<dump_memory_map_heap_control_block> () == 16);
const _: () = core::assert! (core::mem::align_of::<dump_memory_map_heap_control_block> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_heap_control_block, refcount) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_memory_map_heap_control_block, mem) == 8);

const _: () = core::assert! (core::mem::size_of::<dump_bitset> () == 16);
const _: () = core::assert! (core::mem::align_of::<dump_bitset> () == 8);
const _: () = core::assert! (core::mem::offset_of! (dump_bitset, bits) == 0);
const _: () = core::assert! (core::mem::offset_of! (dump_bitset, number_words) == 8);
const _: () = core::assert! (DUMP_BITSET_WORD_WIDTH == 32);
const _: () = core::assert! (core::mem::size_of::<dump_bitset_word> () == 4);

const _: () = core::assert! (core::mem::size_of::<pdumper_loaded_dump_private> () == 152);
const _: () = core::assert! (core::mem::align_of::<pdumper_loaded_dump_private> () == 8);
const _: () = core::assert! (core::mem::offset_of! (pdumper_loaded_dump_private, header) == 0);
const _: () = core::assert! (core::mem::offset_of! (pdumper_loaded_dump_private, mark_bits) == 104);
const _: () = core::assert! (core::mem::offset_of! (pdumper_loaded_dump_private, last_mark_bits) == 120);
const _: () = core::assert! (core::mem::offset_of! (pdumper_loaded_dump_private, load_time) == 136);
const _: () = core::assert! (core::mem::offset_of! (pdumper_loaded_dump_private, dump_filename) == 144);

const _: () = core::assert! (core::mem::size_of::<bignum_reload_info> () == 8);
const _: () = core::assert! (core::mem::align_of::<bignum_reload_info> () == 4);
const _: () = core::assert! (core::mem::offset_of! (bignum_reload_info, data_location) == 0);
const _: () = core::assert! (core::mem::offset_of! (bignum_reload_info, nlimbs) == 4);

/* The invocation-local `struct candidate` inside dump_queue_dequeue.  */
#[repr(C)]
#[derive(Clone, Copy)]
struct candidate
{
  score: c_float,
  sequence: dump_off,
}
const _: () = core::assert! (core::mem::size_of::<candidate> () == 8);
const _: () = core::assert! (core::mem::align_of::<candidate> () == 4);
const _: () = core::assert! (core::mem::offset_of! (candidate, score) == 0);
const _: () = core::assert! (core::mem::offset_of! (candidate, sequence) == 4);
