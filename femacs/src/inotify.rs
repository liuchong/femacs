// femacs: port of src/inotify.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names, the same
// control flow.  The *build-level* file gate is `feature = "have_inotify"` (P-86 ⑧), on every
// item *and on the include graph*: the frozen configuration leaves `HAVE_INOTIFY` undefined
// (`emacs/configure.ac` 4097-4104 is what puts this file in `NOTIFY_OBJ`, under
// `AC_DEFINE([HAVE_INOTIFY])`, and this target is not Linux), so the reference builds no object
// from this file either - while the text is carried **in full**, because a Linux configuration
// compiles it.  The earlier "empty unit plus a deferred-declaration note" reading (status 3.31)
// is withdrawn as a *conclusion*; its measurements (the empty object, the empty checklist) still
// hold, and the coverage record is therefore the C range table in `docs/ports/inotify.md` rather
// than ticked boxes (P-86 ②).
//
// **No Linux SDK is on this machine**, so `<sys/inotify.h>` and `<sys/ioctl.h>` stand in-unit
// (P-88 ①: no mirror exists for either and this file is their only consumer) and their values are
// a *cross-target* clang measurement of the upstream header text - not this machine's SDK.  The
// arm flip in `## Evidence` is a real compile, but every constant and offset of those two headers
// is UNVERIFIED against a Linux toolchain (`## Owed` 3).  `<errno.h>`, `<fcntl.h>`, `<unistd.h>`,
// `<limits.h>` and `<string.h>` do have mirrors and are reached through them (P-26: one spelling
// per name) even though those mirrors carry the frozen target's values; `## Deviations` 5 records
// the four constants where that difference is observable.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

include!("include/config.rs");                  // C 20: `#include <config.h>`

#[cfg (feature = "have_inotify")]
include!("include/globals.rs");                 // C 22's chain: lisp.h's `F*` declarations
#[cfg (feature = "have_inotify")]
include!("include/lisp.rs");                    // C 22: `#include "lisp.h"`
#[cfg (feature = "have_inotify")]
include!("include/coding.rs");                  // C 23: `#include "coding.h"` (`ENCODE_FILE`,
                                                // `DECODE_FILE`)
#[cfg (feature = "have_inotify")]
include!("include/process.rs");                 // C 24: `#include "process.h"` (`add_read_fd`,
                                                // `delete_read_fd`)
#[cfg (feature = "have_inotify")]
include!("include/keyboard.rs");                // C 25: `#include "keyboard.h"`
                                                // (`kbd_buffer_store_event`)
#[cfg (feature = "have_inotify")]
include!("include/termhooks.rs");               // C 26: `#include "termhooks.h"` (`input_event`,
                                                // `event_kind`, `EVENT_INIT`)
/* The four edges C 22-26's chain reaches through other headers, spelled here (P-86 ⑧, the
   `src/kqueue.rs` precedent): `termhooks.h` names `Time` (systime.h) and `Emacs_Color`
   (dispextern.h), `lisp.h` 30's `<string.h>` and 33's `<limits.h>` carry the C entry points this
   unit's chain needs (`strcmp`), and `include/keyboard.rs` 32 reaches `buffer.rs`, whose text
   names character.h's helpers.  */
#[cfg (feature = "have_inotify")]
include!("include/systime.rs");                 // termhooks.h's `Time`
#[cfg (feature = "have_inotify")]
include!("include/dispextern.rs");              // termhooks.h's `Emacs_Color`
#[cfg (feature = "have_inotify")]
include!("include/character.rs");               // buffer.rs's five character.h helpers
#[cfg (feature = "have_inotify")]
include!("../lib/include/string.rs");           // C 22's chain: lisp.h 30's `<string.h>`
#[cfg (feature = "have_inotify")]
include!("../lib/include/limits.in.rs");        // C 22's chain: lisp.h 33's `<limits.h>`
                                                // (`INT_MAX`, C 482/485)

#[cfg (feature = "have_inotify")]
include!("../lib/include/errno.rs");            // C 28: `#include <errno.h>` (`errno`, `EINVAL`)
#[cfg (feature = "have_inotify")]
include!("../lib/include/fcntl.in.rs");         // C 29: `#include <fcntl.h>` (`fcntl`, `F_SETFL`,
                                                // `F_SETFD`, `O_NONBLOCK`, `O_CLOEXEC`)
#[cfg (feature = "have_inotify")]
include!("../lib/include/unistd.in.rs");        // C 335's chain: `<unistd.h>` (`read`, `ssize_t`)

// C 31: `#include <sys/inotify.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<sys/inotify.h>`: the event record, the mask constants and the four entry points.

   C 31's `#include <sys/inotify.h>` has no file in the reference tree (P-88 ①: the declaration
   site is C's own `#include` line, and no file is added under `femacs/src/include/`).  **No Linux
   header is on this machine**, so the values below are a *cross-target* clang measurement of the
   glibc/UAPI header text (P-88 ②'s measurement shape, run with `-target x86_64-linux-gnu`); the
   reading and the two files it was taken from are recorded in `docs/ports/inotify.md` `## Evidence`
   7, and the whole section is **UNVERIFIED** against a real Linux SDK (`## Owed` 3).

   C's `IN_*` masks have C type `int` (except `IN_ONESHOT`, `0x80000000`, which is `unsigned int`),
   and every use in this file is a `uint32_t` mask expression (`mask` in `mask_to_aspects`/
   `inotifyevent_to_event`, `imask`/`mask` in `Finotify_add_watch`) or a `uint32_t` return value,
   so the port types them `uint32_t` - a deviation of Rust spelling, not of value (`## Deviations`
   2).  `IN_CLOEXEC`/`IN_NONBLOCK` are this header's own enum constants with that name (they are
   *not* `<fcntl.h>`'s `O_CLOEXEC`/`O_NONBLOCK`), and they are passed to `inotify_init1 (int)`, so
   they keep C's `int`.  */
/// C: sys/inotify.h: `struct inotify_event { int wd; uint32_t mask; uint32_t cookie;
/// uint32_t len; char name __flexarr; };` - measured `sizeof` 16 with wd@0, mask@4, cookie@8,
/// len@12 and the flexible `name` member at 16 (the record's end; C's `ev->name` decays to that
/// address, so the port keeps the member as a zero-length array and reads it the same way).
#[cfg (feature = "have_inotify")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct inotify_event
{
  pub wd: c_int,
  pub mask: uint32_t,
  pub cookie: uint32_t,
  pub len: uint32_t,
  pub name: [c_char; 0],
}
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::size_of::<inotify_event> () == 16);
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::offset_of! (inotify_event, wd) == 0);
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::offset_of! (inotify_event, mask) == 4);
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::offset_of! (inotify_event, cookie) == 8);
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::offset_of! (inotify_event, len) == 12);
#[cfg (feature = "have_inotify")]
const _: () = core::assert! (core::mem::offset_of! (inotify_event, name) == 16);

/// C: sys/inotify.h: `#define IN_ACCESS 0x00000001` - measured 1.
#[cfg (feature = "have_inotify")]
pub const IN_ACCESS: uint32_t = 0x00000001;
/// C: sys/inotify.h: `#define IN_MODIFY 0x00000002` - measured 2.
#[cfg (feature = "have_inotify")]
pub const IN_MODIFY: uint32_t = 0x00000002;
/// C: sys/inotify.h: `#define IN_ATTRIB 0x00000004` - measured 4.
#[cfg (feature = "have_inotify")]
pub const IN_ATTRIB: uint32_t = 0x00000004;
/// C: sys/inotify.h: `#define IN_CLOSE_WRITE 0x00000008` - measured 8.
#[cfg (feature = "have_inotify")]
pub const IN_CLOSE_WRITE: uint32_t = 0x00000008;
/// C: sys/inotify.h: `#define IN_CLOSE_NOWRITE 0x00000010` - measured 16.
#[cfg (feature = "have_inotify")]
pub const IN_CLOSE_NOWRITE: uint32_t = 0x00000010;
/// C: sys/inotify.h: `#define IN_CLOSE (IN_CLOSE_WRITE | IN_CLOSE_NOWRITE)` - C's own
/// expression; measured 24.
#[cfg (feature = "have_inotify")]
pub const IN_CLOSE: uint32_t = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE;
/// C: sys/inotify.h: `#define IN_OPEN 0x00000020` - measured 32.
#[cfg (feature = "have_inotify")]
pub const IN_OPEN: uint32_t = 0x00000020;
/// C: sys/inotify.h: `#define IN_MOVED_FROM 0x00000040` - measured 64.
#[cfg (feature = "have_inotify")]
pub const IN_MOVED_FROM: uint32_t = 0x00000040;
/// C: sys/inotify.h: `#define IN_MOVED_TO 0x00000080` - measured 128.
#[cfg (feature = "have_inotify")]
pub const IN_MOVED_TO: uint32_t = 0x00000080;
/// C: sys/inotify.h: `#define IN_MOVE (IN_MOVED_FROM | IN_MOVED_TO)` - C's own expression;
/// measured 192.
#[cfg (feature = "have_inotify")]
pub const IN_MOVE: uint32_t = IN_MOVED_FROM | IN_MOVED_TO;
/// C: sys/inotify.h: `#define IN_CREATE 0x00000100` - measured 256.
#[cfg (feature = "have_inotify")]
pub const IN_CREATE: uint32_t = 0x00000100;
/// C: sys/inotify.h: `#define IN_DELETE 0x00000200` - measured 512.
#[cfg (feature = "have_inotify")]
pub const IN_DELETE: uint32_t = 0x00000200;
/// C: sys/inotify.h: `#define IN_DELETE_SELF 0x00000400` - measured 1024.
#[cfg (feature = "have_inotify")]
pub const IN_DELETE_SELF: uint32_t = 0x00000400;
/// C: sys/inotify.h: `#define IN_MOVE_SELF 0x00000800` - measured 2048.
#[cfg (feature = "have_inotify")]
pub const IN_MOVE_SELF: uint32_t = 0x00000800;
/// C: sys/inotify.h: `#define IN_UNMOUNT 0x00002000` - measured 8192.
#[cfg (feature = "have_inotify")]
pub const IN_UNMOUNT: uint32_t = 0x00002000;
/// C: sys/inotify.h: `#define IN_Q_OVERFLOW 0x00004000` - measured 16384.
#[cfg (feature = "have_inotify")]
pub const IN_Q_OVERFLOW: uint32_t = 0x00004000;
/// C: sys/inotify.h: `#define IN_IGNORED 0x00008000` - measured 32768.
#[cfg (feature = "have_inotify")]
pub const IN_IGNORED: uint32_t = 0x00008000;
/// C: sys/inotify.h: `#define IN_ISDIR 0x40000000` - measured 1073741824.
#[cfg (feature = "have_inotify")]
pub const IN_ISDIR: uint32_t = 0x40000000;
/// C: sys/inotify.h: `#define IN_ALL_EVENTS (IN_ACCESS | IN_MODIFY | IN_ATTRIB |
/// IN_CLOSE_WRITE | IN_CLOSE_NOWRITE | IN_OPEN | IN_MOVED_FROM | IN_MOVED_TO | IN_CREATE |
/// IN_DELETE | IN_DELETE_SELF | IN_MOVE_SELF)` - C's own expression; measured 4095.
#[cfg (feature = "have_inotify")]
pub const IN_ALL_EVENTS: uint32_t = IN_ACCESS | IN_MODIFY | IN_ATTRIB | IN_CLOSE_WRITE
  | IN_CLOSE_NOWRITE | IN_OPEN | IN_MOVED_FROM | IN_MOVED_TO | IN_CREATE | IN_DELETE
  | IN_DELETE_SELF | IN_MOVE_SELF;
/* `IN_ONLYDIR`, `IN_DONT_FOLLOW` and `IN_EXCL_UNLINK` are this header's names as well, but this
   file's own `#ifndef` fallbacks are where their two arms are written (C 35-43 below): one name,
   one place (P-26), and the arm C's own text spells comes first there.  */

/// C: sys/inotify.h: `#define IN_MASK_ADD 0x20000000` - measured 536870912.
#[cfg (feature = "have_inotify")]
pub const IN_MASK_ADD: uint32_t = 0x20000000;
/// C: sys/inotify.h: `#define IN_ONESHOT 0x80000000` - measured 2147483648 (named by C 429's
/// docstring only; declared with the rest of the header's surface this file names).
#[cfg (feature = "have_inotify")]
pub const IN_ONESHOT: uint32_t = 0x80000000;
/// C: bits/inotify.h: `enum { IN_CLOEXEC = 02000000, IN_NONBLOCK = 00004000 };` - measured
/// 524288 and 2048 (these are this header's own constants, *not* `<fcntl.h>`'s `O_*`).
#[cfg (feature = "have_inotify")]
pub const IN_CLOEXEC: c_int = 0o2000000;
#[cfg (feature = "have_inotify")]
pub const IN_NONBLOCK: c_int = 0o4000;

#[cfg (feature = "have_inotify")]
unsafe extern "C" {
  /// C: sys/inotify.h: `extern int inotify_init (void) __THROW;`
  pub fn inotify_init () -> c_int;
  /// C: sys/inotify.h: `extern int inotify_init1 (int __flags) __THROW;`
  pub fn inotify_init1 (__flags: c_int) -> c_int;
  /// C: sys/inotify.h: `extern int inotify_add_watch (int __fd, const char *__name,
  /// uint32_t __mask) __THROW;`
  pub fn inotify_add_watch (__fd: c_int, __name: *const c_char, __mask: uint32_t) -> c_int;
  /// C: sys/inotify.h: `extern int inotify_rm_watch (int __fd, int __wd) __THROW;`
  pub fn inotify_rm_watch (__fd: c_int, __wd: c_int) -> c_int;
}

// C 32: `#include <sys/ioctl.h>` - the header this section stands for.
/* ----------------------------------------------------------------------------------------------
   `<sys/ioctl.h>`: the one entry point and the one request number this file takes from it.

   C 32's `#include <sys/ioctl.h>` has no file in the reference tree (P-88 ①), so the declarations
   stand at C's own `#include` line - the `src/term.rs` 208 / `src/sound.rs` 187 shape for `ioctl`.
   `FIONREAD` is this machine's *cross-target* measurement (0x541B, `asm-generic/ioctls.h`); the
   frozen target's own value for the same name is the one `src/keyboard.rs` 8908 carries, and the
   difference is recorded in `## Deviations` 5.  */
#[cfg (feature = "have_inotify")]
unsafe extern "C" {
  /// C: sys/ioctl.h: `extern int ioctl (int __fd, unsigned long int __request, ...) __THROW;`
  pub fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
}
/// C: sys/ioctl.h (`asm-generic/ioctls.h` 46): `#define FIONREAD 0x541B` - measured 21531.
#[cfg (feature = "have_inotify")]
pub const FIONREAD: c_ulong = 0x541B;

// C 34: C's own comment line: "Ignore bits that might be undefined on old GNU/Linux systems."
/* C 35-43: the three `#ifndef` fallbacks that follow it, each with both of its arms (P-86 ①).
   The arms stand **here**, not in C 31's header section, because the file's own text is where
   these three names get their only definition (the `src/sound.rs` 499-503 reading of the same
   idiom); C's own arm is written first, because a second declaration of one name in one unit is
   `E0428` and `extract-abi.py --check` reads the first (`## Deviations` 3).  The `no_in_*`
   feature names C's arm (P-51's naming: the feature names the side that is not the port's
   default): this file's configuration is a Linux one, where `<sys/inotify.h>` defines all three,
   and the measured value is the second arm of each pair (`## Evidence` 7).  */
/* Ignore bits that might be undefined on old GNU/Linux systems.  */
// C 35-37: `#ifndef IN_EXCL_UNLINK` / `# define IN_EXCL_UNLINK 0` / `#endif`
#[cfg (all (feature = "have_inotify", feature = "no_in_excl_unlink"))]
pub const IN_EXCL_UNLINK: uint32_t = 0;
/// C: sys/inotify.h: `#define IN_EXCL_UNLINK 0x04000000` - measured 67108864.
#[cfg (all (feature = "have_inotify", not (feature = "no_in_excl_unlink")))]
pub const IN_EXCL_UNLINK: uint32_t = 0x04000000;
// C 38-40: `#ifndef IN_DONT_FOLLOW` / `# define IN_DONT_FOLLOW 0` / `#endif`
#[cfg (all (feature = "have_inotify", feature = "no_in_dont_follow"))]
pub const IN_DONT_FOLLOW: uint32_t = 0;
/// C: sys/inotify.h: `#define IN_DONT_FOLLOW 0x02000000` - measured 33554432.
#[cfg (all (feature = "have_inotify", not (feature = "no_in_dont_follow")))]
pub const IN_DONT_FOLLOW: uint32_t = 0x02000000;
// C 41-43: `#ifndef IN_ONLYDIR` / `# define IN_ONLYDIR 0` / `#endif`
#[cfg (all (feature = "have_inotify", feature = "no_in_onlydir"))]
pub const IN_ONLYDIR: uint32_t = 0;
/// C: sys/inotify.h: `#define IN_ONLYDIR 0x01000000` - measured 16777216.
#[cfg (all (feature = "have_inotify", not (feature = "no_in_onlydir")))]
pub const IN_ONLYDIR: uint32_t = 0x01000000;

// C 45-47: `#ifdef HAVE_ANDROID` / `#include "android.h"` / `#endif` - `android_is_special
// _directory`'s declaration comes from the `android.h` mirror (C 46's own file, so this is an
// include edge, not an in-unit section).
#[cfg (all (feature = "have_inotify", feature = "have_android"))]
include!("include/android.rs");                 // C 46: `#include "android.h"`

/* C 22's chain: lisp.h 4274's two `*_no_quit` declarations.  The mirror does not carry them (the
   `src/xfaces.rs` 4302 / `src/font.rs` 469 reading), so this unit declares them here; the C
   prototypes are quoted with their lisp.h lines.  */
#[cfg (feature = "have_inotify")]
unsafe extern "C" {
  /// C: lisp.h 4274: `extern Lisp_Object assoc_no_quit (Lisp_Object, Lisp_Object);`
  pub fn assoc_no_quit (key: Lisp_Object, list: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4282: `extern bool equal_no_quit (Lisp_Object, Lisp_Object);` (fns.c).
  pub fn equal_no_quit (o1: Lisp_Object, o2: Lisp_Object) -> bool;
  /// C: lisp.h 30's `<string.h>` via `lib/include/string.in.rs` 240-246: the mirror keeps only
  /// the dead `rpl_strnlen` arm, so the system entry is declared here (the `src/process.rs` 3336
  /// shape); `lib/strnlen.rs` defines it.
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
}

/// C: termhooks.h 291: `, FILE_NOTIFY_EVENT` - an `enum input_event_kind` member inside C's
/// `#ifdef USE_FILE_NOTIFY`, which the frozen `include/termhooks.rs` does not carry (it carries
/// the frozen configuration's members, so the member before `SLEEP_EVENT` shifts everything
/// after it), so it is declared here with the value this configuration gives it - the
/// `src/gfilenotify.rs` 169 / `src/kqueue.rs` 153 reading (28), owed to the mirror round.
#[cfg (feature = "have_inotify")]
pub const FILE_NOTIFY_EVENT: c_int = 28;
/// termhooks.h's `enum event_kind kind : 16` shares `include/termhooks.rs`'s `input_event::bits`;
/// C 341 writes `event.kind = FILE_NOTIFY_EVENT;`, so the port keeps C's field name in a per-unit
/// macro (the `src/gtkutil.rs` 1757-1770 shape, body-identical to `src/kqueue.rs` 156-165).
#[cfg (feature = "have_inotify")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

/* C 49-50: `static int inotifyfd = -1;` - C's own initialiser.  */
#[cfg (feature = "have_inotify")]
static mut inotifyfd: c_int = -1;

/* C 52-73: C's own comment on the watch list.  */
/* Alist of files being watched.  We want the returned descriptor to
   be unique for every watch, but inotify returns the same descriptor
   WD for multiple calls to inotify_add_watch with the same file.
   Supply a nonnegative integer ID, so that WD and ID together
   uniquely identify a watch/file combination.

   For the same reason, we also need to store the watch's mask and we
   can't allow the following flags to be used.

   IN_EXCL_UNLINK
   IN_MASK_ADD
   IN_ONESHOT

   Each element of this list is of the form (DESCRIPTOR . WATCHES)
   where no two DESCRIPTOR values are the same.  DESCRIPTOR represents
   the inotify watch descriptor and WATCHES is a list with elements of
   the form (ID FILENAME CALLBACK MASK), where ID is the integer
   described above, FILENAME names the file being watched, CALLBACK is
   invoked when the event occurs, and MASK represents the aspects
   being watched.  The WATCHES list is sorted by ID.  Although
   DESCRIPTOR and MASK are ordinarily integers, they are conses when
   representing integers outside of fixnum range.  */
/* C 75: the tentative definition `static Lisp_Object watch_list;` is zero, which is `Qnil`
   (`NIL_IS_ZERO`).  */
#[cfg (feature = "have_inotify")]
static mut watch_list: Lisp_Object = lisp_h_Qnil ();

/* C 77-114: `static Lisp_Object mask_to_aspects (uint32_t mask)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn mask_to_aspects (mask: uint32_t) -> Lisp_Object
{
  let mut aspects: Lisp_Object = Qnil ();
  if mask & IN_ACCESS != 0
    { aspects = Fcons (Qaccess (), aspects); }
  if mask & IN_ATTRIB != 0
    { aspects = Fcons (Qattrib (), aspects); }
  if mask & IN_CLOSE_WRITE != 0
    { aspects = Fcons (Qclose_write (), aspects); }
  if mask & IN_CLOSE_NOWRITE != 0
    { aspects = Fcons (Qclose_nowrite (), aspects); }
  if mask & IN_CREATE != 0
    { aspects = Fcons (Qcreate (), aspects); }
  if mask & IN_DELETE != 0
    { aspects = Fcons (Qdelete (), aspects); }
  if mask & IN_DELETE_SELF != 0
    { aspects = Fcons (Qdelete_self (), aspects); }
  if mask & IN_MODIFY != 0
    { aspects = Fcons (Qmodify (), aspects); }
  if mask & IN_MOVE_SELF != 0
    { aspects = Fcons (Qmove_self (), aspects); }
  if mask & IN_MOVED_FROM != 0
    { aspects = Fcons (Qmoved_from (), aspects); }
  if mask & IN_MOVED_TO != 0
    { aspects = Fcons (Qmoved_to (), aspects); }
  if mask & IN_OPEN != 0
    { aspects = Fcons (Qopen (), aspects); }
  if mask & IN_IGNORED != 0
    { aspects = Fcons (Qignored (), aspects); }
  if mask & IN_ISDIR != 0
    { aspects = Fcons (Qisdir (), aspects); }
  if mask & IN_Q_OVERFLOW != 0
    { aspects = Fcons (Qq_overflow (), aspects); }
  if mask & IN_UNMOUNT != 0
    { aspects = Fcons (Qunmount (), aspects); }
  aspects
}

/* C 116-165: `static uint32_t symbol_to_inotifymask (Lisp_Object symb)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn symbol_to_inotifymask (symb: Lisp_Object) -> uint32_t
{
  if EQ (symb, Qaccess ())
    { return IN_ACCESS; }
  else if EQ (symb, Qattrib ())
    { return IN_ATTRIB; }
  else if EQ (symb, Qclose_write ())
    { return IN_CLOSE_WRITE; }
  else if EQ (symb, Qclose_nowrite ())
    { return IN_CLOSE_NOWRITE; }
  else if EQ (symb, Qcreate ())
    { return IN_CREATE; }
  else if EQ (symb, Qdelete ())
    { return IN_DELETE; }
  else if EQ (symb, Qdelete_self ())
    { return IN_DELETE_SELF; }
  else if EQ (symb, Qmodify ())
    { return IN_MODIFY; }
  else if EQ (symb, Qmove_self ())
    { return IN_MOVE_SELF; }
  else if EQ (symb, Qmoved_from ())
    { return IN_MOVED_FROM; }
  else if EQ (symb, Qmoved_to ())
    { return IN_MOVED_TO; }
  else if EQ (symb, Qopen ())
    { return IN_OPEN; }
  else if EQ (symb, Qmove ())
    { return IN_MOVE; }
  else if EQ (symb, Qclose ())
    { return IN_CLOSE; }

  else if EQ (symb, Qdont_follow ())
    { return IN_DONT_FOLLOW; }
  else if EQ (symb, Qonlydir ())
    { return IN_ONLYDIR; }

  else if EQ (symb, Qignored ())
    { return IN_IGNORED; }
  else if EQ (symb, Qunmount ())
    { return IN_UNMOUNT; }

  else if EQ (symb, Qt ()) || EQ (symb, Qall_events ())
    { return IN_ALL_EVENTS; }
  else
    {
      *errno () = EINVAL;
      report_file_notify_error (c"Unknown aspect".as_ptr (), symb);
    }
}

/* C 167-181: `static uint32_t aspect_to_inotifymask (Lisp_Object aspect)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn aspect_to_inotifymask (aspect: Lisp_Object) -> uint32_t
{
  if CONSP (aspect) || NILP (aspect)
    {
      let mut x: Lisp_Object = aspect;
      let mut mask: uint32_t = 0;
      /* C 174-176's `FOR_EACH_TAIL (x) mask |= …; CHECK_LIST_END (x, aspect);`.  The port's
         `FOR_EACH_TAIL!` re-binds the loop variable inside its own block, so C's terminating
         non-cons cannot be handed back to the caller: the body records it instead (the
         `src/fns.rs` 243-251 shape, PORTING.md 2.2 / docs/ports/fns.md Decisions).  */
      let mut x_end: Lisp_Object = x;
      FOR_EACH_TAIL! (x, {
        mask |= symbol_to_inotifymask (XCAR (x));
        x_end = XCDR (x);
      });
      CHECK_LIST_END (x_end, aspect);
      mask
    }
  else
    { symbol_to_inotifymask (aspect) }
}

/* C 183-209: `static Lisp_Object inotifyevent_to_event (Lisp_Object watch,
   struct inotify_event const *ev)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn inotifyevent_to_event (watch: Lisp_Object, ev: *const inotify_event) -> Lisp_Object
{
  let mut name: Lisp_Object;
  /* C 188's `CONS_TO_INTEGER (Fnth (make_fixnum (3), watch), uint32_t, mask);` - the port's
     trait picks the helper from the variable's type and has the two signed/unsigned 64-bit
     impls, so C's `uint32_t` conversion is the explicit cast (the `src/editfns.rs` 1170-1171
     shape).  */
  let mut mask_wide: i64;
  CONS_TO_INTEGER! (Fnth (make_fixnum (3), watch), mask_wide);
  let mask: uint32_t = mask_wide as uint32_t;

  if (mask & (*ev).mask) == 0
      /* These event types are supposed to be reported whether or not
	 they appeared in the ASPECT list when monitoring commenced.  */
      && ((*ev).mask & (IN_IGNORED | IN_Q_OVERFLOW | IN_ISDIR | IN_UNMOUNT)) == 0
    { return Qnil (); }

  if (*ev).len > 0
    {
      name = make_unibyte_string ((*ev).name.as_ptr (),
                                  strnlen ((*ev).name.as_ptr (), (*ev).len as usize) as ptrdiff_t);
      name = DECODE_FILE (name);
    }
  else
    { name = XCAR (XCDR (watch)); }

  list2 (list4 (Fcons (INT_TO_INTEGER ((*ev).wd as EMACS_INT), XCAR (watch)),
                       mask_to_aspects ((*ev).mask),
                       name,
		       INT_TO_INTEGER ((*ev).cookie as EMACS_INT)),
		Fnth (make_fixnum (2), watch))
}

/* Add a new watch to watch-descriptor WD watching FILENAME and using
   IMASK and CALLBACK.  Return a cons (DESCRIPTOR . ID) uniquely
   identifying the new watch.  */
/* C 214-248: `static Lisp_Object add_watch (int wd, Lisp_Object filename, uint32_t imask,
   Lisp_Object callback)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn add_watch (wd: c_int, filename: Lisp_Object, imask: uint32_t, callback: Lisp_Object)
                     -> Lisp_Object
{
  let descriptor: Lisp_Object = INT_TO_INTEGER (wd as EMACS_INT);
  let mut tail: Lisp_Object = assoc_no_quit (descriptor, watch_list);
  let watch: Lisp_Object;
  let watch_id: Lisp_Object;
  let mask: Lisp_Object = INT_TO_INTEGER (imask as EMACS_INT);

  let mut id: EMACS_INT = 0;
  if NILP (tail)
    {
      tail = list1 (descriptor);
      watch_list = Fcons (tail, watch_list);
    }
  else
    {
      /* Assign a watch ID that is not already in use, by looking
	 for a gap in the existing sorted list.  */
      loop
        {
          if NILP (XCDR (tail))
            { break; }
          if !BASE_EQ (XCAR (XCAR (XCDR (tail))), make_fixnum (id))
            { break; }
          tail = XCDR (tail);
          id += 1;
        }
      if MOST_POSITIVE_FIXNUM < id
	{ emacs_abort (); }
    }

  /* Insert the newly-assigned ID into the previously-discovered gap,
     which is possibly at the end of the list.  Inserting it there
     keeps the list sorted.  */
  watch_id = make_fixnum (id);
  watch = list4 (watch_id, filename, callback, mask);
  XSETCDR (tail, Fcons (watch, XCDR (tail)));

  Fcons (descriptor, watch_id)
}

/* Find the watch list element (if any) matching DESCRIPTOR.  Return
   nil if not found.  If found, return t if the first element matches
   DESCRIPTOR; otherwise, return the cons whose cdr matches
   DESCRIPTOR.  This lets the caller easily remove the element
   matching DESCRIPTOR without having to search for it again, and
   without calling Fdelete (which might quit).  */

/* C 257-265: `static Lisp_Object find_descriptor (Lisp_Object descriptor)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn find_descriptor (descriptor: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object;
  let mut prevtail: Lisp_Object = Qt ();
  tail = watch_list;
  while !NILP (tail)
    {
      if equal_no_quit (XCAR (XCAR (tail)), descriptor)
        { return prevtail; }
      prevtail = tail;
      tail = XCDR (tail);
    }
  Qnil ()
}

/*  Remove all watches associated with the watch list element after
    PREVTAIL, or after the first element if PREVTAIL is t.  If INVALID_P
    is true, the descriptor is already invalid, i.e., it received a
    IN_IGNORED event.  In this case skip calling inotify_rm_watch.  */
/* C 271-303: `static void remove_descriptor (Lisp_Object prevtail, bool invalid_p)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn remove_descriptor (prevtail: Lisp_Object, invalid_p: bool)
{
  let tail: Lisp_Object = if CONSP (prevtail) { XCDR (prevtail) } else { watch_list };

  let mut inotify_errno: c_int = 0;
  if !invalid_p
    {
      /* C 279-280: `int wd; CONS_TO_INTEGER (XCAR (XCAR (tail)), int, wd);` - the same
         explicit cast as C 188's reading above.  */
      let mut wd_wide: i64;
      CONS_TO_INTEGER! (XCAR (XCAR (tail)), wd_wide);
      let wd: c_int = wd_wide as c_int;
      if inotify_rm_watch (inotifyfd, wd) != 0
	{ inotify_errno = *errno (); }
    }

  if CONSP (prevtail)
    { XSETCDR (prevtail, XCDR (tail)); }
  else
    {
      watch_list = XCDR (tail);
      if NILP (watch_list)
	{
	  delete_read_fd (inotifyfd);
	  emacs_close (inotifyfd);
	  inotifyfd = -1;
	}
    }

  if inotify_errno != 0
    {
      *errno () = inotify_errno;
      report_file_notify_error (c"Could not rm watch".as_ptr (), XCAR (tail));
    }
}

/*  Remove watch associated with (descriptor, id).  */
/* C 306-322: `static void remove_watch (Lisp_Object descriptor, Lisp_Object id)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn remove_watch (descriptor: Lisp_Object, id: Lisp_Object)
{
  let prevtail: Lisp_Object = find_descriptor (descriptor);
  if NILP (prevtail)
    { return; }

  let elt: Lisp_Object = XCAR (if CONSP (prevtail) { XCDR (prevtail) } else { watch_list });
  let mut prev: Lisp_Object = elt;
  while !NILP (XCDR (prev))
    {
      if EQ (id, XCAR (XCAR (XCDR (prev))))
	{
	  XSETCDR (prev, XCDR (XCDR (prev)));
	  if NILP (XCDR (elt))
	    { remove_descriptor (prevtail, false); }
	  break;
	}
      prev = XCDR (prev);
    }
}

/* This callback is called when the FD is available for read.  The inotify
   events are read from FD and converted into input_events.  */
/* C 326-368: `static void inotify_callback (int fd, void *_)`.  */
#[cfg (feature = "have_inotify")]
unsafe extern "C" fn inotify_callback (fd: c_int, _: *mut c_void)
{
  /* C 329's `int to_read;` is written by the callee, and Rust needs an initial value to take
     its address (PORTING.md 1.5: zero-initialisation, named in the ledger's `## Deviations`).  */
  let mut to_read: c_int = 0;
  if ioctl (fd, FIONREAD, &raw mut to_read) < 0
    { report_file_notify_error (c"Error while retrieving file system events".as_ptr (), Qnil ()); }
  /* C 333-334: `USE_SAFE_ALLOCA; char *buffer = SAFE_ALLOCA (to_read);` - the port's macros
     name C's `sa_avail`/`sa_count` explicitly (the one shape `include/lisp.rs` 2974-2985
     documents).  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let buffer: *mut c_char = SAFE_ALLOCA! (sa_avail, to_read) as *mut c_char;
  let n: ssize_t = read (fd, buffer as *mut c_void, to_read as usize);
  if n < 0
    { report_file_notify_error (c"Error while reading file system events".as_ptr (), Qnil ()); }

  let mut event: input_event = core::mem::zeroed ();
  EVENT_INIT (&raw mut event);
  input_event_bits! (kind, &raw mut event, FILE_NOTIFY_EVENT);
  event.frame_or_window = Qnil ();

  let mut i: ssize_t = 0;
  while i < n
    {
      let ev: *mut inotify_event = buffer.add (i as usize).cast ();
      let descriptor: Lisp_Object = INT_TO_INTEGER ((*ev).wd as EMACS_INT);
      let prevtail: Lisp_Object = find_descriptor (descriptor);

      if !NILP (prevtail)
        {
	  let tail: Lisp_Object = if CONSP (prevtail) { XCDR (prevtail) } else { watch_list };
	  let mut watches: Lisp_Object = XCDR (XCAR (tail));
	  while !NILP (watches)
	    {
	      event.arg = inotifyevent_to_event (XCAR (watches), ev);
	      if !NILP (event.arg)
		{ kbd_buffer_store_event (&raw mut event as *mut c_void); }
	      watches = XCDR (watches);
	    }
          /* If event was removed automatically: Drop it from watch list.  */
          if (*ev).mask & IN_IGNORED != 0
	    { remove_descriptor (prevtail, true); }
        }
      i += core::mem::size_of::<inotify_event> () as ssize_t + (*ev).len as ssize_t;
    }

  SAFE_FREE! (sa_count);
}

/* C 370-476: `DEFUN ("inotify-add-watch", Finotify_add_watch, Sinotify_add_watch,
   3, 3, 0, doc: /* Add a watch for FILE-NAME to inotify.  …  */)`.  */
#[cfg (feature = "have_inotify")]
DEFUN! ("inotify-add-watch", Finotify_add_watch, Sinotify_add_watch, 3, 3, 0,
       doc: "Add a watch for FILE-NAME to inotify.\n\nReturn a watch descriptor.  The watch will look for ASPECT events and\ninvoke CALLBACK when an event occurs.\n\nASPECT might be one of the following symbols or a list of those symbols:\n\naccess\nattrib\nclose-write\nclose-nowrite\ncreate\ndelete\ndelete-self\nmodify\nmove-self\nmoved-from\nmoved-to\nopen\n\nall-events or t\nmove\nclose\n\nASPECT can also contain the following symbols, which control whether\nthe watch descriptor will be created:\n\ndont-follow\nonlydir\n\nWatching a directory is not recursive.  CALLBACK is passed a single argument\nEVENT which contains an event structure of the format\n\n(WATCH-DESCRIPTOR ASPECTS NAME COOKIE)\n\nWATCH-DESCRIPTOR is the same object that was returned by this function.  It can\nbe tested for equality using `equal'.  ASPECTS describes the event.  It is a\nlist of ASPECT symbols described above and can also contain one of the following\nsymbols\n\nignored\nisdir\nq-overflow\nunmount\n\nIf a directory is watched then NAME is the name of file that caused the event.\n\nCOOKIE is an object that can be compared using `equal' to identify two matching\nrenames (moved-from and moved-to).\n\nSee inotify(7) and inotify_add_watch(2) for further information.  The\ninotify fd is managed internally and there is no corresponding\ninotify_init.  Use `inotify-rm-watch' to remove a watch.\n\nThe following inotify bit-masks cannot be used because descriptors are\nshared across different callers.\n\nIN_EXCL_UNLINK\nIN_MASK_ADD\nIN_ONESHOT",
       (filename: Lisp_Object, aspect: Lisp_Object, callback: Lisp_Object),
       {
  let mut encoded_file_name: Lisp_Object;
  let mut wd: c_int = -1;
  let imask: uint32_t = aspect_to_inotifymask (aspect);
  let mask: uint32_t = imask | IN_MASK_ADD | IN_EXCL_UNLINK;
  let mut name: *mut c_char;

  CHECK_STRING (filename);

  if inotifyfd < 0
    {
      // C 443-444: `#ifdef HAVE_INOTIFY_INIT1` - the arm a glibc that has the
      // function takes; the other arm is `have_inotify_init1` off.
      #[cfg (feature = "have_inotify_init1")]
      {
	inotifyfd = inotify_init1 (IN_NONBLOCK | IN_CLOEXEC);
      }
      // C 445-451: `#else` / `inotify_init ()` plus the two `fcntl` calls / `#endif`.
      #[cfg (not (feature = "have_inotify_init1"))]
      {
	/* This is prey to race conditions with other threads calling
	   exec.  */
	inotifyfd = inotify_init ();
	fcntl (inotifyfd, F_SETFL, O_NONBLOCK);
	fcntl (inotifyfd, F_SETFD, O_CLOEXEC);
      }
      if inotifyfd < 0
	{ report_file_notify_error (c"File watching is not available".as_ptr (), Qnil ()); }
      watch_list = Qnil ();
      add_read_fd (inotifyfd, Some (inotify_callback), core::ptr::null_mut ());
    }

  encoded_file_name = ENCODE_FILE (filename);
  name = SSDATA (encoded_file_name);

  // C 461-469: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` … `#endif`
#[cfg (all (feature = "have_inotify", feature = "have_android"))]
  {
  /* If FILENAME actually lies in a special directory, return now
     instead of letting inotify fail.  These directories cannot
     receive file notifications as they are read only.  */

  if android_is_special_directory (name, c"/assets".as_ptr ())
      || android_is_special_directory (name, c"/content".as_ptr ())
    { return Qnil (); }
  }

  wd = inotify_add_watch (inotifyfd, name, mask);
  if wd < 0
    { report_file_notify_error (c"Could not add watch for file".as_ptr (), filename); }

  add_watch (wd, filename, imask, callback)
});

/* C 478-488: `static bool valid_watch_descriptor (Lisp_Object wd)`.  */
#[cfg (feature = "have_inotify")]
unsafe fn valid_watch_descriptor (wd: Lisp_Object) -> bool
{
  (CONSP (wd)
	  && (RANGED_FIXNUMP (0, XCAR (wd), INT_MAX as EMACS_INT)
	      || (CONSP (XCAR (wd))
		  && RANGED_FIXNUMP ((MOST_POSITIVE_FIXNUM >> 16) + 1,
				      XCAR (XCAR (wd)), (INT_MAX >> 16) as EMACS_INT)
		  && RANGED_FIXNUMP (0, XCDR (XCAR (wd)), (1 << 16) - 1)))
	  && FIXNATP (XCDR (wd)))
}

/* C 490-509: `DEFUN ("inotify-rm-watch", Finotify_rm_watch, Sinotify_rm_watch, 1, 1, 0,
   doc: /* Remove an existing WATCH-DESCRIPTOR.  …  */)`.  */
#[cfg (feature = "have_inotify")]
DEFUN! ("inotify-rm-watch", Finotify_rm_watch, Sinotify_rm_watch, 1, 1, 0,
       doc: "Remove an existing WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `inotify-add-watch'.\n\nSee inotify_rm_watch(2) for more information.",
       (watch_descriptor: Lisp_Object),
       {

  let descriptor: Lisp_Object;
  let id: Lisp_Object;

  if ! valid_watch_descriptor (watch_descriptor)
    { report_file_notify_error (c"Invalid descriptor ".as_ptr (), watch_descriptor); }

  descriptor = XCAR (watch_descriptor);
  id = XCDR (watch_descriptor);
  remove_watch (descriptor, id);

  Qt ()
});

/* C 511-529: `DEFUN ("inotify-valid-p", Finotify_valid_p, Sinotify_valid_p, 1, 1, 0,
   doc: /* Check a watch specified by its WATCH-DESCRIPTOR.  …  */)`.  */
#[cfg (feature = "have_inotify")]
DEFUN! ("inotify-valid-p", Finotify_valid_p, Sinotify_valid_p, 1, 1, 0,
       doc: "Check a watch specified by its WATCH-DESCRIPTOR.\n\nWATCH-DESCRIPTOR should be an object returned by `inotify-add-watch'.\n\nA watch can become invalid if the file or directory it watches is\ndeleted, or if the watcher thread exits abnormally for any other\nreason.  Removing the watch by calling `inotify-rm-watch' also makes\nit invalid.",
       (watch_descriptor: Lisp_Object),
       {
  if ! valid_watch_descriptor (watch_descriptor)
    { return Qnil (); }
  let tail: Lisp_Object = assoc_no_quit (XCAR (watch_descriptor), watch_list);
  if NILP (tail)
    { return Qnil (); }
  let watch: Lisp_Object = assq_no_quit (XCDR (watch_descriptor), XCDR (tail));
  if !NILP (watch) { Qt () } else { Qnil () }
});

/* C 531-545: `#ifdef INOTIFY_DEBUG` - the two debugging subrs, both arms carried
   (P-86 ①): the frozen configuration takes the empty arm.  */
#[cfg (all (feature = "have_inotify", feature = "inotify_debug"))]
DEFUN! ("inotify-watch-list", Finotify_watch_list, Sinotify_watch_list, 0, 0, 0,
       doc: "Return a copy of the internal watch_list.",
       (),
       {
  Fcopy_sequence (watch_list)
});

#[cfg (all (feature = "have_inotify", feature = "inotify_debug"))]
DEFUN! ("inotify-allocated-p", Finotify_allocated_p, Sinotify_allocated_p, 0, 0, 0,
       doc: "Return non-nil, if an inotify instance is allocated.",
       (),
       {
  if inotifyfd < 0 { Qnil () } else { Qt () }
});

/* C 547-590: `void syms_of_inotify (void)`.  */
#[cfg (feature = "have_inotify")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_inotify ()
{
  DEFSYM! (Qaccess, "access");		/* IN_ACCESS */
  DEFSYM! (Qattrib, "attrib");		/* IN_ATTRIB */
  DEFSYM! (Qclose_write, "close-write");	/* IN_CLOSE_WRITE */
  DEFSYM! (Qclose_nowrite, "close-nowrite");
					/* IN_CLOSE_NOWRITE */
  DEFSYM! (Qcreate, "create");		/* IN_CREATE */
  DEFSYM! (Qdelete, "delete");		/* IN_DELETE */
  DEFSYM! (Qdelete_self, "delete-self");	/* IN_DELETE_SELF */
  DEFSYM! (Qmodify, "modify");		/* IN_MODIFY */
  DEFSYM! (Qmove_self, "move-self");	/* IN_MOVE_SELF */
  DEFSYM! (Qmoved_from, "moved-from");	/* IN_MOVED_FROM */
  DEFSYM! (Qmoved_to, "moved-to");	/* IN_MOVED_TO */
  DEFSYM! (Qopen, "open");		/* IN_OPEN */

  DEFSYM! (Qall_events, "all-events");	/* IN_ALL_EVENTS */
  DEFSYM! (Qmove, "move");		/* IN_MOVE */
  DEFSYM! (Qclose, "close");		/* IN_CLOSE */

  DEFSYM! (Qdont_follow, "dont-follow");	/* IN_DONT_FOLLOW */
  DEFSYM! (Qonlydir, "onlydir");		/* IN_ONLYDIR */

  // C 571-574: `#if 0` / `DEFSYM (Qignored, "ignored")` / `#endif` - dead in every
  // configuration (`coding.c` defines it), recorded here and not transcribed (the
  // `src/xfns.rs` 958 / `src/androidfns.rs` `#if 0` precedent, P-86 ②).
  DEFSYM! (Qisdir, "isdir");		/* IN_ISDIR */
  DEFSYM! (Qq_overflow, "q-overflow");	/* IN_Q_OVERFLOW */
  DEFSYM! (Qunmount, "unmount");	/* IN_UNMOUNT */

  defsubr (&raw mut Sinotify_add_watch);
  defsubr (&raw mut Sinotify_rm_watch);
  defsubr (&raw mut Sinotify_valid_p);

  // C 583-586: `#ifdef INOTIFY_DEBUG` - the two debugging subrs.
  #[cfg (feature = "inotify_debug")]
  {
    defsubr (&raw mut Sinotify_watch_list);
    defsubr (&raw mut Sinotify_allocated_p);
  }
  staticpro (&raw mut watch_list);

  Fprovide (intern_c_string (c"inotify".as_ptr ()), Qnil ());
}
