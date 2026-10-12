// femacs: port of src/androidvfs.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C
// `static` helpers stay plain `unsafe fn` except the sixteen whose address C takes (the thirteen
// `unix_vfs_ops` members and the three `android_vdir` ones), which are `unsafe extern "C" fn`
// items - a function-pointer field cannot hold a Rust-ABI item (the androidmenu.rs /
// androidselect.rs shape).
//
// **This file is P-86's largest unit (C 7,950 lines) and is being ported in slices.**  This is
// slice 1: C 1-995, from the file's first line through `android_unix_opendir`'s closing brace.
// The next live definition is C 1016's `android_extract_long`, preceded by C 1005-1011's three
// file-scope objects (`directory_tree`, `directory_tree_size`, `asset_manager`) and the
// asset-directory comment block at C 999-1002.  The ledger `docs/ports/androidvfs.md` carries the
// C line-range table, the `## Evidence` readings and the `## Deviations`.
//
// The frozen configuration compiles no `androidvfs.o`: `configure.ac` 2647-2648 adds it to
// `ANDROID_OBJ` only in the really-Android branch (`XCONFIGURE = android`), while 2597's list
// (`androidmenu.o`) is in both, and 2605-2606 defines `ANDROID_STUBIFY` in the *other* branch, so
// this file's build condition is C's `HAVE_ANDROID && !defined ANDROID_STUBIFY` -
// `docs/baseline/frozen-build-sources.txt` lists no `androidvfs.o` for this target and
// `extract-abi.py`'s checklist for the file is empty.  Every item of this slice therefore sits
// under the pair `all (feature = "have_android", feature = "not_android_stubify")`, as does C 20-55's
// include graph, and that is why the frozen configuration compiles only `include/config.rs` and
// the `#![allow]` attribute here.
//
// The gates, in C's order:
//
//   * the file gate is the pair above (`configure.ac` 2647-2648, the androidselect.rs shape);
//   * C 46's `#if __ANDROID_API__ >= 9` / C 50's `#else` is this file's only other `#if`.  Its two
//     arms include two different headers and define `OLD_ANDROID_ASSETS` as 0 or 1.  The port
//     gates them on `android_api_ge_9`, a feature name this tree has *not* registered in
//     `femacs/xtask/Cargo.toml` (the table carries `android_api_lt_21`, `android_api_le_11` and
//     `android_api_le_30` only) - a port lane does not add feature names to that manifest, so the
//     ledger names it as owed by the integrator and `#[cfg]` on an unregistered name is simply
//     false, which is what makes the `>= 9` arm the one a build gets today.
//
// Two constructor-level facts this slice records rather than invents:
//
//   * **`struct android_vdir` cannot come from its mirror.**  C 40's `#include "android.h"` has
//     the mirror `include/android.rs`, which carries C 164's forward declaration of that struct
//     as the P-70 zero-sized stand-in (`pub struct android_vdir;`), while C's *definition* of the
//     same struct is this file's (C 114-126).  One translation unit cannot hold both: a
//     crate-root item and that stand-in are one `E0428` (measured), and P-70 says the stand-in
//     goes in the same commit as the definition, in that mirror.  That file is another lane's, so
//     this unit carries the one name it takes from android.h itself - the
//     `android_parcel_file_descriptor_class` record C 308's `fd_class` is an object of - in an
//     in-unit section at C 40's line (PORTING.md P-88 ① / P-72 rule ③), and the ledger records
//     the conflict and the owed removal of the stand-in.
//   * `<assert.h>` (C 23) carries no item: its gnulib template (`lib/include/assert.in.rs`) holds
//     only preprocessor text, and the units that need C's `assert` mirror the macro themselves
//     (the `include/android-asset.rs` / `lib-src/ebrowse.rs` shape) - which this file does at C
//     23's line, with the frozen `NDEBUG` body.  Every layout assertion below therefore spells
//     `core::assert!`, because that `macro_rules!` shadows the prelude's.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-55: the file's own include list.  None of it is reached by a translation unit the frozen
   configuration builds, so the whole graph sits on the file gate (see the header).  C 33's
   `<systime.h>` and C 42's `"systime.h"` are the same header reached twice - the include guard
   makes the second a no-op, which is what `xtask`'s per-file dedupe reproduces - so the single
   edge stands at C 29's line, where C reaches it first.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/fcntl.in.rs");         // C 21: `#include <fcntl.h>` (open, AT_FDCWD)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/unistd.in.rs");        // C 22: `#include <unistd.h>` (access, readlink, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/dirent.rs");           // C 25: `#include <dirent.h>` (DIR, opendir, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/errno.rs");            // C 26: `#include <errno.h>` (errno, EXDEV)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/minmax.rs");           // C 27: `#include <minmax.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/string.rs");           // C 28: `#include <string.h>` (memcpy, memchr, …)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/systime.rs");                 // C 33: `#include <systime.h>` (src/systime.h)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_types.in.rs");     // `mode_t`/`ssize_t`/`off_t` (the two above)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stdlib.rs");           // C's `<stdlib.h>` via lisp.h (no name used here)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/sys_stat.in.rs");      // C 32: `#include <sys/stat.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/stat-time.rs");        // C 35: `#include <stat-time.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("../lib/include/md5.rs");              // C 36: `#include <md5.h>`
/* C 41's edge stands after android.h's chain, which is where C puts it and where
   `lisp.rs`'s `macro_rules!` must already be in scope for `androidterm.rs` and `coding.rs`
   (PORTING.md 0.1-9: macro scope is textual).  */
/* C 40: `#include "android.h"`.  The mirror is `include/android.rs`; this unit does not include it
   because the mirror's C 164 forward declaration of `struct android_vdir` is a zero-sized stand-in
   that C's *definition* of the same struct (this file's C 114-126) cannot coexist with in one
   translation unit (measured: `E0428`).  P-70 puts the stand-in's removal in the same commit as
   the definition's landing, in that mirror; the mirror is another lane's file, so this unit
   carries the android.h name its own text uses and the ledger owes the move (P-88 ①/P-72 ③).
   The name is exactly one: the record `fd_class` is an object of.  */
/* C 40's two `include!` edges stand at C's own line, before C 41's and before C 46-53's: the
   materializer marks a file as included at its *first* textual reach regardless of the `#[cfg]`
   on that edge (PORTING.md P-89), and C 51's `android-asset.h` mirror includes `jni.rs` itself -
   so the gated edge below would otherwise consume `jni.rs`'s single inclusion and leave this arm
   without the JNI types.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/jni.rs");                     // C 40's chain: `android.h` -> `<jni.h>`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/lisp.rs");                    // C 40's chain: `android.h` -> `lisp.h`

// C 58-64.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_parcel_file_descriptor_class
{
  pub class: jclass,
  pub close: jmethodID,
  pub get_fd: jmethodID,
  pub detach_fd: jmethodID,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_parcel_file_descriptor_class> () == 32);

/* C 46-53's `#if __ANDROID_API__ >= 9` chain.  Both arms are carried: the real one includes the
   NDK's `<android/asset_manager.h>`/`<android/asset_manager_jni.h>` (no mirror in this tree, so
   the opaque `AAsset` its two declarations and `struct android_vops.open` name is declared here,
   P-88 ①) and the old one includes `"android-asset.h"` (whose mirror is
   `include/android-asset.rs`).  `OLD_ANDROID_ASSETS` is a two-valued macro; C's uses of its value
   are C 1046-1278's (slice 2), and this slice carries the two definitions C 49/C 52 write.  */
/* The `>= 9` arm (C 47-48) reaches the two NDK headers directly; they have no mirror, and their
   entries are declared at their use sites in the slices that call them (only `AAsset` is named by
   C 1-995, below).  The `#else` arm (C 51) reaches the reference header `android-asset.h`, whose
   mirror is `include/android-asset.rs`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_ge_9"))]
include!("include/android-asset.rs");           // C 51: `#include "android-asset.h"`

/* C's `<assert.h>` is one header read once per translation unit, so the port must give the name
   exactly one definition per TU at the crate root (PORTING.md 0.1-5).  C 51's `android-asset.h`
   mirror mirrors the same macro (its own `<assert.h>` read), and a `macro_rules!` that arrives
   through an `include!` is "macro-expanded" for name resolution: with no crate-root definition
   *after* it, every sibling mirror's `assert!` is `E0659` (measured).  This definition is the
   unit's own C 23, its body is `android-asset.rs` 47's and `lib-src/ebrowse.rs` 93's, and it
   stands after the C 51 edge in C's include order so the same-level later definition wins.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! assert {
  ($cond:expr) => { () };
}

/* C 49's `#define OLD_ANDROID_ASSETS 0` and C 52's `= 1`: C's macro is defined twice, once per
   arm, so the port keeps C's name with the arm's value (PORTING.md P-08).  C's uses of the value
   are C 1046-1278's; this slice carries the two definitions.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub const OLD_ANDROID_ASSETS: c_int = 0;                                     // C 49
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (not (feature = "android_api_ge_9"))]
pub const OLD_ANDROID_ASSETS: c_int = 1;                                     // C 52

/* `struct AAsset`, the NDK's own opaque type (`<android/asset_manager.h>` 70-77:
   `struct AAsset;` / `typedef struct AAsset AAsset;` - never defined, so it is only ever a
   pointer, which is how this file names it).  Measured with the NDK's clang against the
   installed header: `sizeof (AAsset *)` is 8 for `--target=aarch64-linux-android24`.  */
/// C: struct AAsset;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub struct AAsset;

/* The JNI ABI this unit's text names comes from the NDK's `<jni.h>`, which C reaches through
   android.h 30 (`#include <jni.h>`); that mirror (`include/jni.rs`, the measured NDK ABI) is
   included here for the same reason - the android.h edge above is the one that cannot be carried
   (P-72 rule 4: a unit that needs the names must have the mirror in its own list).  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/androidterm.rs");             // C 41: `#include "androidterm.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/blockinput.rs");              // C 43: `#include "blockinput.h"`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/coding.rs");                  // C 44: `#include "coding.h"`

/* C 24 `<dlfcn.h>`, C 30 `<semaphore.h>`, C 33 `<sys/mman.h>`, C 38 `<linux/ashmem.h>` and C 55
   `<android/log.h>` have no mirror in this tree, and no item of theirs is named by C 1-995: the
   four `dlsym` entries and the two `RTLD_*` values are C 1295/C 1364/C 1471's (slice 2), the
   `sem_*` calls are C 6508's, `mmap`/`munmap` are C 1330-1360's, and `__android_log_print` is
   first called at C 1181.  Each is declared at its own use site when a later slice reaches it
   (PORTING.md P-88 ①), and this slice carries no text for them.  */

/* C 23: `<assert.h>`'s `assert`.  No mirror carries the macro, and the frozen configuration
   defines `NDEBUG` (`macros.tsv` 2892), so C's `assert (cond)` expands to `((void) 0)` and its
   operand is never evaluated; the port keeps C's name with the same body (the
   `include/android-asset.rs` 47 and `lib-src/ebrowse.rs` 93 precedent), which is also why every
   `const _` assertion below spells `core::assert!` - this macro shadows the prelude's.  C 340,
   371 and 404 are the three uses in this slice.  */



/* `<string.h>`: the gnulib entry `lib/include/string.rs` does not carry (`HAVE_MEMRCHR` is
   undefined for this configuration).  C 522 is the one caller in this slice.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *memrchr (const void *, int, size_t);`
  pub fn memrchr (s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
}

/* `<string.h>`: the same gnulib entry, not in the mirror.  C 724, 739 and 783 name it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *mempcpy (void *, const void *, size_t);`
  pub fn mempcpy (dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

/* `<string.h>`: the mirror carries only the dead `rpl_strtok_r` rename
   (`string.in.rs` 264-270), so the system entry is declared here.  C 1080 (slice 2) is its
   only caller in the file.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `char *strtok_r (char *, const char *, char **);`
  pub fn strtok_r (s: *mut c_char, delim: *const c_char,
                                   save_ptr: *mut *mut c_char) -> *mut c_char;
}

/* `<stdlib.h>`: C reaches it through lisp.h's own chain (the edge above), and
   `lib/include/stdlib.rs` carries `malloc` and friends but not `abort`.  C 605 is its one use in
   this slice.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `_Noreturn void abort (void);`
  pub fn abort () -> !;
}

/* `<stdio.h>`: `lib/include/stdio.rs` does not carry `rename` (the `sysdep.rs` 766 shape).
   C 896 names it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int rename (const char *, const char *);`
  pub fn rename (old: *const c_char, new: *const c_char) -> c_int;
}

/* lisp.h 5339's `renameat_noreplace`: the declaration `lisp.rs` does not carry, and whose
   definition is `sysdep.rs`'s in an Android configuration (`sysdep.c` 2376-2378's
   `#ifndef HAVE_ANDROID` makes it external there).  C 894 names it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `int renameat_noreplace (int, char const *, int, char const *);`
  pub fn renameat_noreplace (srcfd: c_int, src: *const c_char,
                                             dstfd: c_int, dst: *const c_char) -> c_int;
}

/* Structure describing an array of VFS operations.  */

/* C 110-112: the comment above and C's forward declaration `struct android_vnode;`.  Rust needs no
   forward declaration for a type defined later in the same unit (PORTING.md 10.10).  */

// C 114-126.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vdir
{
  /* Return a `struct dirent' describing the next file in this
     directory stream, or NULL if the stream has reached its end.  */
  /// C: struct dirent *(*readdir) (struct android_vdir *); - `struct dirent` has no mirror
  /// (`lib/include/dirent.rs` keeps it opaque: bionic's is 280 bytes, measured, and no ported unit
  /// reads its fields), so the pointer keeps the untyped form (PORTING.md P-55).
  pub readdir: Option<unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>,

  /* Close and release all resources allocated for this directory
     stream.  */
  pub closedir: Option<unsafe extern "C" fn (*mut android_vdir)>,

  /* Return a ``file descriptor'' tied to this directory stream.  */
  pub dirfd: Option<unsafe extern "C" fn (*mut android_vdir) -> c_int>,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vdir> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, readdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, closedir) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vdir, dirfd) == 16);

// C 128-231.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vops
{
  /* Name a child of the given VFS node, which should be a
     directory.

     LENGTH should be the length of NAME, excluding that of any
     trailing NULL byte.

     NAME should be a normalized and NULL-terminated relative file
     name; it may contain a leading separator characters, but no
     consecutive ones.

     If NAME is empty, create another VFS node designating the same
     file instead.

     NAME should also be located within writable storage; it may be
     overwritten as the vnode sees fit.

     Value is a VFS node corresponding to the child, or NULL upon
     failure.

     A VFS node may be returned even if NAME does not exist, the
     expectation being that either a later filesystem operation will
     fail, or will create the file.  */
  pub name: Option<unsafe extern "C" fn (*mut android_vnode, *mut c_char, size_t)
                                        -> *mut android_vnode>,

  /* Open the specified VNODE, returning either a file descriptor or
     an asset file descriptor.

     FLAGS and MODE mean the same as they do to the Unix `open' system
     call.

     ASSET_P stipulates if an asset file descriptor may be returned;
     if true, *ASSET may be set to an asset file descriptor.

     If an asset file descriptor is unavailable or ASSET_P is false,
     *FD will be set to a file descriptor.

     If the vnode cannot be opened, value is -1 with errno set
     accordingly.  Otherwise, value is 0 if a file descriptor was
     returned, and 1 if an asset file descriptor was returned.  */
  pub open: Option<unsafe extern "C" fn (*mut android_vnode, c_int, mode_t, bool, *mut c_int,
                                         *mut *mut AAsset) -> c_int>,

  /* Close the specified VNODE, releasing all of its resources.
     Save errno before making system calls that may set it, and
     restore it to its original value before returning.

     This is unrelated to `android_close', which primarily releases on
     stat buffers linked to file or asset file descriptors.  */
  pub close: Option<unsafe extern "C" fn (*mut android_vnode)>,

  /* Unlink the file and the specified VNODE.  Value and errno are the
     same as Unix `unlink'.  */
  pub unlink: Option<unsafe extern "C" fn (*mut android_vnode) -> c_int>,

  /* Create a symlink from the specified VNODE to the target TARGET.
     Value and errno are the same as `symlink' on Linux (which notably
     means that errno is set to EPERM if VNODE doesn't support
     symlinks.)  */
  pub symlink: Option<unsafe extern "C" fn (*const c_char, *mut android_vnode) -> c_int>,

  /* Remove VNODE from its parent directory.  VNODE must be an empty
     directory.  Value and errno are the same as Unix `rmdir'.  */
  pub rmdir: Option<unsafe extern "C" fn (*mut android_vnode) -> c_int>,

  /* Move the file designated by SRC to DST, overwriting DST if
     KEEP_EXISTING is false.

     If KEEP_EXISTING is true and DST already exists, value is -1 with
     errno set to EEXIST.

     If VNODE does not natively support checking for a preexisting DST
     and KEEP_EXISTING is true, value is -1 with errno set to ENOSYS.

     Value is otherwise the same as `rename'.  */
  pub rename: Option<unsafe extern "C" fn (*mut android_vnode, *mut android_vnode, bool)
                                           -> c_int>,

  /* Return statistics for the specified VNODE, and FLAGS, as in a call
     to `fstatat'.  Value and errno are the same as with Unix
     `stat'.  */
  pub stat: Option<unsafe extern "C" fn (*mut android_vnode, *mut stat, c_int) -> c_int>,

  /* Return whether or not VNODE is accessible.
     Value, errno and MODE are the same as with Unix `access'.  */
  pub access: Option<unsafe extern "C" fn (*mut android_vnode, c_int) -> c_int>,

  /* Make a directory designated by VNODE, like Unix `mkdir'.  */
  pub mkdir: Option<unsafe extern "C" fn (*mut android_vnode, mode_t) -> c_int>,

  /* Change the access mode of the provided VNODE to MODE.  Value is
     the same as with `chmod'.  FLAGS is passed verbatim from the call
     to the delegating at-func, and is probably
     AT_SYMLINK_NOFOLLOW.  */
  pub chmod: Option<unsafe extern "C" fn (*mut android_vnode, mode_t, c_int) -> c_int>,

  /* Return the target of VNODE if it is a symbolic link, or -1.
     Value and errno are the same as with `readlink'.  */
  pub readlink: Option<unsafe extern "C" fn (*mut android_vnode, *mut c_char, size_t) -> ssize_t>,

  /* Open the specified VNODE as a directory.
     Value is a ``directory handle'', or NULL upon failure.  */
  pub opendir: Option<unsafe extern "C" fn (*mut android_vnode) -> *mut android_vdir>,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vops> () == 104);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, name) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, open) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vops, opendir) == 96);

// C 233-240.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_vnode
{
  /* Operations associated with this vnode.  */
  pub ops: *mut android_vops,

  /* Type of this vnode and its flags.  */
  pub r#type: c_short,
  pub flags: c_short,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_vnode> () == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, ops) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, r#type) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_vnode, flags) == 10);

/* Structure describing a special named vnode relative to the root
   vnode, or another directory vnode.  */

// C 245-260.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_special_vnode
{
  /* The name of the special file.  */
  pub name: *const c_char,

  /* The length of that name.  */
  pub length: size_t,

  /* Function called to create the initial vnode from the rest of the
     component.  */
  pub initial: Option<unsafe extern "C" fn (*mut c_char, size_t) -> *mut android_vnode>,

  /* If non-nil, an encoding system into which file name buffers are to
     be re-encoded before being handed to VFS functions.  */
  pub special_coding_system: Lisp_Object,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`;
   `Lisp_Object` stands in for `lisp.h`'s pointer typedef, 8 bytes).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_special_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, name) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, length) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, initial) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_special_vnode, special_coding_system)
                                           == 24);

/* C 262: `static_assert (NIL_IS_ZERO); /* special_coding_system above.  */` - C's
   `static_assert` becomes `const _: () = core::assert! (…)` (PORTING.md P-60), and
   `NIL_IS_ZERO` is `lisp.rs`'s (lisp.h 1999).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (NIL_IS_ZERO); /* special_coding_system above.  */

/* C 264-275: `enum android_vnode_type`.  C's enum ABI is `int` (PORTING.md 1.3), so the port
   writes the alias plus C's members with their implicit values (0, 1, … 8), the
   `dispextern.rs` `draw_glyphs_face` shape.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type android_vnode_type = c_int;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_UNIX: android_vnode_type = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_AFS: android_vnode_type = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT: android_vnode_type = 2;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT_AUTHORITY: android_vnode_type = 3;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_CONTENT_AUTHORITY_NAMED: android_vnode_type = 4;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_ROOT: android_vnode_type = 5;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_TREE: android_vnode_type = 6;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_FILE: android_vnode_type = 7;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_VNODE_SAF_NEW: android_vnode_type = 8;

/* Structure describing the android.database.Cursor class.  */

// C 281-285.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_cursor_class
{
  pub class: jclass,
  pub close: jmethodID,
}

/* Structure describing the EmacsDirectoryEntry class.  */

// C 289-294.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_directory_entry_class
{
  pub class: jclass,
  pub d_type: jfieldID,
  pub d_name: jfieldID,
}

/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`;
   `jclass`/`jmethodID`/`jfieldID` are `jni.h`'s pointers).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_cursor_class> () == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_cursor_class, class) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_cursor_class, close) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<emacs_directory_entry_class> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, class) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, d_type) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (emacs_directory_entry_class, d_name) == 16);

/* The java.lang.String class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut java_string_class: jclass = std::ptr::null_mut ();             // C 297

/* Fields and methods associated with the Cursor class.  C's `static struct` is a tentative
   definition, i.e. every member zero (the androidmenu.rs `menu_class` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut cursor_class: android_cursor_class = unsafe { std::mem::zeroed () };         // C 300

/* Fields and methods associated with the EmacsDirectoryEntry
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut entry_class: emacs_directory_entry_class
  = unsafe { std::mem::zeroed () };                                           // C 304

/* Fields and methods associated with the ParcelFileDescriptor
   class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut fd_class: android_parcel_file_descriptor_class
  = unsafe { std::mem::zeroed () };                                           // C 308

/* Global references to several exception classes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut file_not_found_exception: jclass = std::ptr::null_mut ();         // C 311
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut security_exception: jclass = std::ptr::null_mut ();               // C 311
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut operation_canceled_exception: jclass = std::ptr::null_mut ();     // C 312
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut unsupported_operation_exception: jclass = std::ptr::null_mut ();  // C 313
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut out_of_memory_error: jclass = std::ptr::null_mut ();              // C 313

/* C 315-317: the comment `/* Initialize `cursor_class' using the given JNI environment ENV.
   Calling this function is not necessary on Android 4.4 and earlier.  */`.  */

/* C 319: `static void android_init_cursor_class (JNIEnv *env)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_cursor_class (env: *mut JNIEnv)
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  cursor_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"android/database/Cursor".as_ptr ());
  eassert! (! cursor_class.class.is_null ());

  old = cursor_class.class;
  cursor_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if cursor_class.class.is_null ()
    { emacs_abort (); }

  /* C 336-340: `FIND_METHOD`, the macro this function defines and `#undef`s at C 342.  Rust's
     `macro_rules!` scope ends with the block, so the `#undef` has no counterpart (the
     androidselect.rs 213-223 / android.rs 1979-1990 shape), and the macro's parameter names are
     C's so the body's tokens survive.  C 340's `assert` is this file's macro above.  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      cursor_class.$c_name
	= ((*(*env)).GetMethodID.unwrap ()) (env, cursor_class.class,
					     $name, $signature);
      assert! (unsafe { cursor_class.$c_name });
    };
  }

  FIND_METHOD! (close, c"close".as_ptr (), c"()V".as_ptr ());
}

/* C 345-347: the comment `/* Initialize `entry_class' using the given JNI environment ENV.
   Calling this function is not necessary on Android 4.4 and earlier.  */`.  */

/* C 349: `static void android_init_entry_class (JNIEnv *env)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_entry_class (env: *mut JNIEnv)
{
  let mut old: jclass;

  /* C leaves `old` uninitialised (PORTING.md 1.5).  */
  old = std::ptr::null_mut ();

  entry_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"org/femacs/EmacsDirectoryEntry".as_ptr ());
  eassert! (! entry_class.class.is_null ());

  old = entry_class.class;
  entry_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if entry_class.class.is_null ()
    { emacs_abort (); }

  entry_class.d_type = ((*(*env)).GetFieldID.unwrap ()) (env, entry_class.class,
							  c"d_type".as_ptr (),
							  c"I".as_ptr ());
  entry_class.d_name = ((*(*env)).GetFieldID.unwrap ()) (env, entry_class.class,
							  c"d_name".as_ptr (),
							  c"Ljava/lang/String;".as_ptr ());
  assert! (unsafe { entry_class.d_type } != std::ptr::null_mut ()
	   && unsafe { entry_class.d_name } != std::ptr::null_mut ());
}

/* C 374-377: the comment `/* Initialize `fd_class' using the given JNI environment ENV.  Called
   on API 12 (Android 3.1) and later by androidselect.c and on 5.0 and later in this file.  */`.  */

/* C 379: `void android_init_fd_class (JNIEnv *env)` - non-`static` in C, so the port exports C's
   symbol (PORTING.md 0.2; `android.h` 72 declares it and androidselect.rs 623 calls it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_init_fd_class (env: *mut JNIEnv)
{
  let mut old: jclass;
  static mut fd_class_initialized: bool = false;

  if fd_class_initialized
    { return; }

  fd_class.class
    = ((*(*env)).FindClass.unwrap ()) (env, c"android/os/ParcelFileDescriptor".as_ptr ());
  eassert! (! fd_class.class.is_null ());

  old = fd_class.class;
  fd_class.class
    = ((*(*env)).NewGlobalRef.unwrap ()) (env, old as jobject);
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, old);

  if fd_class.class.is_null ()
    { emacs_abort (); }

  /* C 400-408: the same `FIND_METHOD` macro with three uses (C 405-407).  */
  macro_rules! FIND_METHOD {
    ($c_name:ident, $name:expr, $signature:expr) => {
      fd_class.$c_name
	= ((*(*env)).GetMethodID.unwrap ()) (env, fd_class.class,
					     $name, $signature);
      assert! (unsafe { fd_class.$c_name });
    };
  }

  FIND_METHOD! (close, c"close".as_ptr (), c"()V".as_ptr ());
  FIND_METHOD! (get_fd, c"getFd".as_ptr (), c"()I".as_ptr ());
  FIND_METHOD! (detach_fd, c"detachFd".as_ptr (), c"()I".as_ptr ());

  fd_class_initialized = true;
}

/* Account for SAF file names two times as large as PATH_MAX; larger
   values are prohibitively slow, but smaller values can't face up to
   some long file names within several nested layers of directories.

   Buffers holding components or other similar file name constituents
   which don't represent SAF files must continue to use PATH_MAX, for
   that is the restriction imposed by the Unix file system.  */

/* C 423: `#define EMACS_PATH_MAX (PATH_MAX * 2)`.  `PATH_MAX` is the frozen target's
   (`lib/include/limits.in.rs` 131: 1024), reached through lisp.h's own `<limits.h>`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const EMACS_PATH_MAX: c_int = PATH_MAX as c_int * 2;                     // C 423

/* Delete redundant instances of `.' and `..' from NAME in-place.
   NAME must be *LENGTH long, excluding a mandatory trailing NULL
   byte.

   Transform each directory component in NAME to avoid instances
   of the `.' and `..' directories.  For example, turn:

     a/../b/c/.

   into

     b/c/

   and return NULL, writing the new length of NAME into *LENGTH.

   If there are more `..' components in NAME than there are normal
   file name components, return NAME incremented to the position after
   the first `..' component that cannot be transformed.  For example,
   if NAME is

     a/../../a

   value will be

     a

   If NAME is a directory separator and LENGTH is 1, return without
   modifying NAME.  In any other case, omit any leading directory
   separator when writing to NAME.  This is useful when a vnode that
   can only be opened as a directory is desired, as this status is
   made clear by suffixing the file name with a trailing
   directory separator.  */

/* C 458: `static char *android_vfs_canonicalize_name (char *name, size_t *length)`.  C's two
   `goto parent_vnode` sites (C 511 and C 579) and the `parent_vnode:` label at C 605 become one
   labelled block (PORTING.md 2.2, the androidselect.rs `'fail:` shape): `goto parent_vnode` is
   `break 'parent_vnode`, and the block's end is where C's label body starts.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_vfs_canonicalize_name (mut name: *mut c_char, length: *mut size_t)
  -> *mut c_char
{
  /* C's `size_t nellipsis` is compared against `-1` and `-2` (C 476's comment, C 554, C 559,
     C 583, C 589, C 597): converting a negative constant to `size_t` gives SIZE_MAX and
     SIZE_MAX - 1, which is what the port spells explicitly so the two comparisons compile.  */
  let mut nellipsis: size_t;
  let mut i: size_t;
  let mut last_component: *mut c_char;
  let mut prec_component: *mut c_char;
  let mut fill: *mut c_char;
  let mut orig_name: *mut c_char;
  let mut size: size_t;

  'parent_vnode: {
    /* Special case described in the last paragraph of the comment
       above.  */

    size = *length;
    orig_name = name;

    if *name == b'/' as c_char && size == 1
      { return std::ptr::null_mut (); }
    else if *name == b'/' as c_char
      { size -= 1; }

    nellipsis = 0; /* Number of ellipsis encountered within the current
		      file name component, or -1.  */
    prec_component = std::ptr::null_mut (); /* Pointer to the separator character of the
					       component immediately preceding the
					       component currently being written.  */
    last_component = name; /* Pointer to the separator character of
			      the component currently being read.  */
    fill = name; /* Pointer to the next character that will be written
		    within NAME.  */

    /* Adjust name to skip the leading directory separator.  But only
       after fill is set.  */
    if *name == b'/' as c_char
      { name = name.offset (1); }

    i = 0;
    while i < size
      {
	/* C 493-561's `switch (name[i])` with its two labels and its `default`.  Rust patterns
	   admit no cast, so the port matches C's own promoted value (`char` promotes to `int`,
	   sign-extending: 47 is `'/'`, 46 is `'.'`) and names each label in a comment - the
	   android.rs 1511-1519 shape for `switch (*name)`.  */
	match *name.offset (i as isize) as c_int
	  {
	    /* C 495: `case '/':` */
	    47 =>
	      {
		/* See if the previous component was `..' or `.'.

		   If it is .., and if no previous directory separator was
		   encountered, return or look up a vnode representing the
		   parent.  */

		if nellipsis == 2
		  {
		    /* .. */

		    if prec_component.is_null ()
		      {
			/* Return the content of the component, i.e. the text
			   _after_ this separator.  */
			i += 1;
			break 'parent_vnode;
		      }

		    /* Return to the last component.  */
		    fill = prec_component;

		    /* Restore last_component to prec_component, and
		       prec_component back to the component before that.  */
		    last_component = prec_component;

		    if last_component != orig_name
		      { prec_component
			  = memrchr (orig_name as *const c_void, b'/' as c_int,
				     (last_component.offset_from (orig_name) - 1) as size_t)
			    as *mut c_char; }
		    else
		      { prec_component = std::ptr::null_mut (); }

		    /* prec_component may now be NULL.  If last_component is
		       identical to the initial value of NAME, then fill has
		       really been returned to the beginning of the string, so
		       leave it be.  But if it's something else, then it must
		       be the first separator character in the string, so set
		       prec_component to this initial value itself.  */

		    if prec_component.is_null () && last_component != orig_name
		      { prec_component = orig_name; }
		  }
		else if nellipsis == 1
		  /* If it's ., return to this component.  */
		  { fill = last_component; }
		else
		  {
		    /* Record the position of the last directory separator,
		       so NAME can be overwritten from there onwards if `..'
		       or `.' are encountered.  */
		    prec_component = last_component;
		    last_component = fill;
		  }

		/* Allow tracking ellipses again.  */
		nellipsis = 0;
	      }

	    /* C 553: `case '.':` */
	    46 =>
	      {
		if nellipsis != size_t::MAX
		  { nellipsis += 1; }
	      }

	    /* C 558: `default:` */
	    _ =>
	      {
		nellipsis = size_t::MAX;
	      }
	  }

	/* Now copy this character over from NAME.  */
	*fill = *name.offset (i as isize);
	fill = fill.offset (1);

	i += 1;
      }

    /* See if the previous component was `..' or `.'.

       If it is .., and if no previous directory separator was
       encountered, return or look up a vnode representing the
       parent.  */

    if nellipsis == 2
      {
	/* .. */

	if prec_component.is_null ()
	  /* Look up the rest of the vnode in its parent.  */
	  { break 'parent_vnode; }

	/* Return to the last component.  */
	fill = prec_component;
	nellipsis = size_t::MAX - 1;
      }
    else if nellipsis == 1
      {
	/* If it's ., return to this component.  */
	fill = last_component;
	nellipsis = size_t::MAX - 1;
      }

    /* Now, if there's enough room and an ellipsis file name was the
       last component of END, append a trailing `/' before NULL
       terminating it, indicating that the file name must be a
       directory.  */

    if fill.offset (1) < name.offset (size as isize) && nellipsis == size_t::MAX - 1
      { *fill = b'/' as c_char; fill = fill.offset (1); }

    /* NULL terminate fill.  */
    *fill = 0;
    *length = fill.offset_from (orig_name) as size_t;
    return std::ptr::null_mut ();
  }

  /* parent_vnode: */
  /* .. was encountered and the parent couldn't be found through
     stripping off preceding components.

     Find the parent vnode and name the rest of NAME starting from
     there.  */
  return name.offset (i as isize);
}

/* Unix vnode implementation.  These VFS nodes directly wrap around
   the Unix filesystem, with the exception of the root vnode.  */

// C 619-629.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_unix_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,

  /* Length of the name without a trailing null byte.  */
  pub name_length: size_t,

  /* Name of the vnode.  */
  pub name: *mut c_char,
}

// C 631-638.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_unix_vdir
{
  /* The directory function table.  */
  pub vdir: android_vdir,

  /* The directory stream.  */
  pub directory: *mut DIR,
}

/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_unix_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, name_length) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vnode, name) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_unix_vdir> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_unix_vdir, directory) == 24);

/* The vnode representing the root filesystem.  C's `static struct` is a tentative definition,
   i.e. every member zero (the android.rs `pixmap_class` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut root_vnode: android_unix_vnode = unsafe { std::mem::zeroed () };  // C 641

/* C 643-659: C's eleven forward declarations of this file's own `static` functions
   (`android_unix_name` … `android_unix_opendir`).  Rust needs no forward declaration for a
   function defined later in the same unit (PORTING.md 10.10), so this block has no text.  */

/* Vector of VFS operations associated with Unix filesystem VFS
   nodes.  */

/* C 664-679: `static struct android_vfs_ops unix_vfs_ops = { … }`.  C's initialiser is
   positional; Rust spells the same members in C's order with their names (the androidselect.rs
   `clipboard_class` shape).  C's eleven prototypes above are what this definition needs in C;
   Rust's item order is free.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut unix_vfs_ops: android_vops = android_vops
  {
    name: Some (android_unix_name),
    open: Some (android_unix_open),
    close: Some (android_unix_close),
    unlink: Some (android_unix_unlink),
    symlink: Some (android_unix_symlink),
    rmdir: Some (android_unix_rmdir),
    rename: Some (android_unix_rename),
    stat: Some (android_unix_stat),
    access: Some (android_unix_access),
    mkdir: Some (android_unix_mkdir),
    chmod: Some (android_unix_chmod),
    readlink: Some (android_unix_readlink),
    opendir: Some (android_unix_opendir),
  };

/* C 681: `static struct android_vnode *android_unix_name (struct android_vnode *vnode, char
   *name, size_t length)`.  C's `goto parent_vnode` (C 695) and the `parent_vnode:` label at
   C 745 become one labelled block (PORTING.md 2.2): the jump is `break 'parent_vnode` and the
   block's end is where C's label body starts.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_name (mut vnode: *mut android_vnode, name: *mut c_char,
					mut length: size_t) -> *mut android_vnode
{
  let mut vp: *mut android_unix_vnode;
  let mut input: *mut android_unix_vnode;
  let mut temp: android_unix_vnode;
  let mut fill: *mut c_char;
  let mut remainder: *mut c_char;
  let mut j: size_t;

  /* C leaves `vp` and `temp` uninitialised (PORTING.md 1.5); `input`, `fill` and `remainder` are
     assigned before any use on every path.  */
  vp = std::ptr::null_mut ();
  temp = unsafe { std::mem::zeroed () };
  input = std::ptr::null_mut ();
  fill = std::ptr::null_mut ();
  remainder = std::ptr::null_mut ();
  j = 0;

  'parent_vnode: {
    /* Canonicalize NAME.  */
    input = vnode as *mut android_unix_vnode;
    remainder = android_vfs_canonicalize_name (name, &raw mut length);

    /* If remainder is set, it's a name relative to the parent vnode.  */
    if ! remainder.is_null ()
      { break 'parent_vnode; }

    /* Create a new unix vnode.  */
    vp = xmalloc (std::mem::size_of::<android_unix_vnode> ()) as *mut android_unix_vnode;

    /* If name is empty, duplicate the current vnode, but reset its file
       operation vector to that for Unix vnodes.  */

    if length < 1
      {
	memcpy (vp as *mut c_void, vnode as *const c_void,
		std::mem::size_of::<android_unix_vnode> ());
	(*vp).vnode.ops = &raw mut unix_vfs_ops;
	(*vp).name = xstrdup ((*vp).name);
	return &raw mut (*vp).vnode;
      }

    /* Otherwise, fill in the vnode.  C assigns the enum member to a `short` field, which
       truncates the `int`; the port spells that conversion (PORTING.md 1.3).  */

    (*vp).vnode.ops = &raw mut unix_vfs_ops;
    (*vp).vnode.r#type = ANDROID_VNODE_UNIX as c_short;
    (*vp).vnode.flags = 0;

    /* Generate the new name of the vnode.  Remove any trailing slash
       from vp->name.  */

    (*vp).name_length = (*input).name_length + length;
    (*vp).name = xmalloc ((*vp).name_length + 2) as *mut c_char;

    /* Copy the parent name over.  */
    fill = mempcpy ((*vp).name as *mut c_void, (*input).name as *const c_void,
		    (*input).name_length) as *mut c_char;

    /* Check if it contains a trailing slash.  input->name cannot be
       empty, as the root vnode's name is `/'.  */

    if *fill.offset (-1) != b'/' as c_char && *name != b'/' as c_char
      /* If not, append a trailing slash and adjust vp->name_length
	 correspondingly.  */
      { *fill = b'/' as c_char; fill = fill.offset (1); (*vp).name_length += 1; }
    else if *fill.offset (-1) == b'/' as c_char && *name == b'/' as c_char
      /* If name has a leading slash and fill does too, move fill
	 backwards so that name's slash will override that of fill.  */
      { fill = fill.offset (-1); (*vp).name_length -= 1; }

    /* Now copy NAME.  */
    fill = mempcpy (fill as *mut c_void, name as *const c_void, length) as *mut c_char;

    /* And NULL terminate fill.  */
    *fill = 0;
    return &raw mut (*vp).vnode;
  }

  /* parent_vnode: */
  /* .. was encountered and the parent couldn't be found through
     stripping off preceding components.

     Find the parent vnode and name the rest of NAME starting from
     there.  */

  if (*input).name_length == 1
    /* This is the vnode representing the root directory; just look
       within itself... */
    { vnode = &raw mut root_vnode.vnode; }
  else
    {
      /* Create a temporary unix vnode within the parent and use it
	 instead.  First, establish the length of vp->name before its
	 last component.  */

      j = (*input).name_length - 1;
      while j != 0
	{
	  if *(*input).name.offset ((j - 1) as isize) == b'/' as c_char
	    { break; }
	  j -= 1;
	}

      /* There must be at least one leading directory separator in an
	 asset vnode's `name' field.  */

      if j == 0
	{ abort (); }

      /* j is now the length of the string minus the size of its last
	 component.  Create a temporary vnode with that as its
	 name.  */

      temp.vnode.ops = &raw mut unix_vfs_ops;
      temp.vnode.r#type = ANDROID_VNODE_UNIX as c_short;
      temp.vnode.flags = 0;
      temp.name_length = j;
      temp.name = xmalloc (j + 1) as *mut c_char;
      fill = mempcpy (temp.name as *mut c_void, (*input).name as *const c_void,
		      j) as *mut c_char;
      *fill = 0;

      /* Search for the remainder of NAME relative to its parent.  */
      vnode = android_unix_name (&raw mut temp.vnode, remainder,
				 strlen (remainder));
      xfree (temp.name as *mut c_void);
      return vnode;
    }

  /* Virtual directories must be ignored in accessing the root directory
     through a Unix subdirectory of the root, as, `/../' */
  return android_unix_name (vnode, remainder, strlen (remainder));
}

/* Create a Unix vnode representing the given file NAME.  Use this
   function to create vnodes that aren't rooted in the root VFS
   node.  */

/* C 802-814.  C calls it directly (C 6928), never through a pointer, so it stays a Rust-ABI
   `unsafe fn` (the android.rs `android_run_debug_thread` rule).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_unix_vnode (name: *const c_char) -> *mut android_vnode
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = xmalloc (std::mem::size_of::<android_unix_vnode> ()) as *mut android_unix_vnode;
  (*vp).vnode.ops = &raw mut unix_vfs_ops;
  (*vp).vnode.r#type = ANDROID_VNODE_UNIX as c_short;
  (*vp).vnode.flags = 0;
  (*vp).name_length = strlen (name);
  (*vp).name = xstrdup (name);
  return &raw mut (*vp).vnode;
}

/* C 816: `static int android_unix_open (struct android_vnode *vnode, int flags, mode_t mode,
   bool asset_p, int *fd, AAsset **asset)`.  ASSET_P and ASSET are unused by this implementation
   (C's own body never reads them) but the signature is `struct android_vops.open`'s.  Its
   address is taken at C 667, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_open (vnode: *mut android_vnode, flags: c_int,
					mode: mode_t, asset_p: bool, fd: *mut c_int,
					asset: *mut *mut AAsset) -> c_int
{
  let mut vp: *mut android_unix_vnode;
  let mut fds: c_int;

  /* C leaves `vp` and `fds` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  fds = 0;

  vp = vnode as *mut android_unix_vnode;
  /* C's `open` is variadic and its `mode` argument is `mode_t` (an `unsigned short`) promoted
     to `int` by the default argument promotions; Rust requires the promoted type there.  */
  fds = open ((*vp).name, flags, mode as c_int);

  if fds < 0
    { return -1; }

  *fd = fds;
  return 0;
}

/* C 834-845.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_unix_vnode;
  let mut save_errno: c_int;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  save_errno = 0;

  save_errno = *errno ();
  vp = vnode as *mut android_unix_vnode;
  xfree ((*vp).name as *mut c_void);
  xfree (vp as *mut c_void);
  *errno () = save_errno;
}

/* C 847-854.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_unlink (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return unlink ((*vp).name);
}

/* C 856-863.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_symlink (target: *const c_char,
					   vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return symlink (target, (*vp).name);
}

/* C 865-872.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_rmdir (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return rmdir ((*vp).name);
}

/* C 874-897.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_rename (src: *mut android_vnode, dst: *mut android_vnode,
					  keep_existing: bool) -> c_int
{
  let mut vp: *mut android_unix_vnode;
  let mut dest: *mut android_unix_vnode;

  /* C leaves both uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  dest = std::ptr::null_mut ();

  if (*src).r#type != (*dst).r#type
    {
      /* If the types of both vnodes differ, complain that they're on
	 two different filesystems (which is correct from a abstract
	 viewpoint.)  */
      *errno () = EXDEV;
      return -1;
    }

  vp = src as *mut android_unix_vnode;
  dest = dst as *mut android_unix_vnode;

  return
    if keep_existing
      { renameat_noreplace (AT_FDCWD, (*vp).name, AT_FDCWD, (*dest).name) }
    else
      { rename ((*vp).name, (*dest).name) };
}

/* C 899-907.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_stat (vnode: *mut android_vnode, statb: *mut stat,
					flags: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return fstatat (AT_FDCWD, (*vp).name, statb, flags);
}

/* C 909-916.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return access ((*vp).name, mode);
}

/* C 918-925.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return mkdir ((*vp).name, mode);
}

/* C 927-935.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_chmod (vnode: *mut android_vnode, mode: mode_t,
					 flags: c_int) -> c_int
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return fchmodat (AT_FDCWD, (*vp).name, mode, flags);
}

/* C 937-945.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
					    size: size_t) -> ssize_t
{
  let mut vp: *mut android_unix_vnode;

  /* C leaves `vp` uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();

  vp = vnode as *mut android_unix_vnode;
  return readlink ((*vp).name, buffer, size);
}

/* C 947-954.  The return type is `*mut c_void`: `struct dirent` has no mirror (see the
   `android_vdir.readdir` field).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_readdir (vdir: *mut android_vdir) -> *mut c_void
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  return readdir ((*dir).directory);
}

/* C 956-964.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  closedir ((*dir).directory);
  xfree (vdir as *mut c_void);
}

/* C 966-973.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_unix_vdir;

  /* C leaves `dir` uninitialised (PORTING.md 1.5).  */
  dir = std::ptr::null_mut ();

  dir = vdir as *mut android_unix_vdir;
  return dirfd ((*dir).directory);
}

/* C 975-995.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_unix_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut vp: *mut android_unix_vnode;
  let mut dir: *mut android_unix_vdir;
  let mut directory: *mut DIR;

  /* C leaves all three uninitialised (PORTING.md 1.5).  */
  vp = std::ptr::null_mut ();
  dir = std::ptr::null_mut ();
  directory = std::ptr::null_mut ();

  /* Try to opendir the vnode.  */
  vp = vnode as *mut android_unix_vnode;
  directory = opendir ((*vp).name);

  if directory.is_null ()
    { return std::ptr::null_mut (); }

  dir = xmalloc (std::mem::size_of::<android_unix_vdir> ()) as *mut android_unix_vdir;
  (*dir).vdir.readdir = Some (android_unix_readdir);
  (*dir).vdir.closedir = Some (android_unix_closedir);
  (*dir).vdir.dirfd = Some (android_unix_dirfd);
  (*dir).directory = directory;
  return &raw mut (*dir).vdir;
}

/* C 4897-7950: the remainder of the file, plus the six items this lane still defers:
   C 2520-2537's `content_vfs_ops`, C 2549-2639's `android_content_name` and C 2887-2906's
   `android_content_initial` (one clique), C 3688-3705's `saf_root_vfs_ops`, C 3748-3848's
   `android_saf_root_name` (it calls `android_saf_tree_from_name`, C 5773) and C 4209-4229's
   `android_saf_root_initial` (it calls `android_saf_root_name`).  The six close together in the
   round that reaches C 5773: `content_vnodes[]` needs `android_saf_root_initial`, and that needs
   `android_saf_root_name`, and that needs `android_saf_tree_from_name` - so the whole dependency
   chain lands with the SAF tree family.  `extract-abi.py --check`'s remaining `missing …` list is
   exactly that text.  */

/* Asset directory handling functions.  ``directory-tree'' is a file in
   the root of the assets directory describing its contents.

   See lib-src/asset-directory-tool for more details.  */               // C 999-1002

/* The Android directory tree.  */                                       // C 1004
/* C 1005's `static const char *directory_tree;` is a **tentative definition**, and the object it
   declares is the one C's own header already defines: `android-asset.h` 26 carries
   `static const char *directory_tree;` and this unit includes that header (C 51).  So the mirror
   owns the object (`include/android-asset.rs` 61, whose own comment says exactly that) and the
   port's rule of one spelling per construct keeps this line out of the unit - measured: a second
   definition here is `E0428 the name \`directory_tree\` is defined multiple times`.  C's own
   `directory_tree_size` (C 1008) and `asset_manager` (C 1011) are *not* in the header, so they are
   the unit's.  */

/* The size of the directory tree.  */                                   // C 1007
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut directory_tree_size: size_t = 0;                             // C 1008

/* C 47's `<android/asset_manager.h>` 70-77 spells the manager's opaque tag, which the `>= 9` arm
   needs and the `#else` arm gets from `android-asset.h`'s mirror instead (`include/android-asset.rs`
   104 has that arm's `AAssetManager`; this arm's C 47-48 headers have no mirror at all).  Measured:
   without it the `>= 9` arm's reading is `E0425 cannot find type \`AAssetManager\`` here - which is
   the gap the round's two-arm probing found.  */
/// C: struct AAssetManager;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub enum AAssetManager {}

/* The asset manager being used.  */                                     // C 1010
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut asset_manager: *mut AAssetManager = std::ptr::null_mut ();    // C 1011

/* Read an unaligned (32-bit) long from the address POINTER.  */         // C 1013

/* C 1013's comment is the function's contract: the asset tree stores 32-bit lengths unaligned, so
   the port keeps C's own `memcpy` rather than a `ptr::read_unaligned` (PORTING.md 3.3 binds the
   libc call instead of replacing it, and this unit's C 1048 call is the same shape), with
   `sizeof number` as `std::mem::size_of_val (&number)` - the `c_uint` local is what fixes the
   read's width, as C's own `sizeof` operand does.  */
// C 1015-1022.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_extract_long (pointer: *const c_char) -> c_uint
{
  let mut number: c_uint = 0;                                           // C 1018 (PORTING.md 1.5:
									//  its address is handed
									//  to C 1020)

  memcpy (&raw mut number as *mut c_void, pointer as *const c_void,     // C 1020
	  std::mem::size_of_val (&number));
  return number;                                                        // C 1021
}

/* Scan to the file FILE in the asset directory tree.  Return a
   pointer to the end of that file (immediately before any children)
   in the directory tree, or NULL if that file does not exist.

   If returning non-NULL, also return the offset to the end of the
   last subdirectory or file in *LIMIT_RETURN.  LIMIT_RETURN may be
   NULL.

   FILE must have less than 11 levels of nesting.  If it ends with a
   trailing slash, then NULL will be returned if it is not actually a
   directory.  */                                                       // C 1024-1034

/* `<android/log.h>`'s one entry this file calls (C 1170) and the priority it passes: the header has
   no mirror in this tree (slice 1's `<dlfcn.h>`… comment names it), and the number is the NDK's
   measured enum (`src/android.rs` 135-143 measured `ANDROID_LOG_WARN == 5` with this machine's
   NDK clang), so both stand at their use site (P-88 ①).  */
/// C: `int __android_log_print (int prio, const char *tag, const char *fmt, ...);` (log.h 112)
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn __android_log_print (prio: c_int, tag: *const c_char, fmt: *const c_char, ...)
			      -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_WARN: c_int = 5;

/* C 1005's tentative definition, **for the `>= 9` arm only**: that arm's C 47-48 includes the
   NDK's asset-manager headers instead of `android-asset.h`, so the object the `#else` arm gets
   from the mirror (`include/android-asset.rs` 61) has no other definition there - in C the two
   lines are one object, and this is C's own structure, one definition per arm.  (The `#else`
   arm must not carry it: that would be `E0428`, measured in slice 2.)  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
static mut directory_tree: *const c_char = std::ptr::null ();

// C 1036-1202.  C's three labels are the port's three jumps (PORTING.md 2.2): `fail` (C 1199) is
// the forward cleanup path, so the body is a labelled block whose end falls into C's label text;
// `again` (C 1106) is a *backward* jump, so it is a labelled `loop` that repeats; and
// `false_positive` (C 1176) is the tail of one token's iteration, reached by the no-match
// fall-through and by C 1126's jump, so the matched path sits in a labelled block that
// `break 'matched` leaves.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_scan_directory_tree (file: *const c_char, limit_return: *mut size_t)
				       -> *const c_char
{
  let mut token: *mut c_char;                                           // C 1039
  let mut saveptr: *mut c_char;
  let mut copy: *mut c_char;
  let mut start: *mut c_char;
  let mut max: *mut c_char;
  let mut limit: *mut c_char;
  let mut token_length: size_t = 0;                                     // C 1040
  let mut ntokens: size_t = 0;
  let mut i: size_t = 0;
  let mut len: size_t = 0;
  let mut tokens: [*mut c_char; 20] = [std::ptr::null_mut (); 20];      // C 1041

  USE_SAFE_ALLOCA! (sa_avail, sa_count);                                // C 1043

  'fail: {
  /* Skip past the 5 or 9 byte header.  */                              // C 1045
  #[cfg (feature = "android_api_ge_9")]                                 // C 1046: C's `#if !OLD_ANDROID_ASSETS`
  { start = (directory_tree as *mut c_char).offset (5); }               // C 1047
  #[cfg (not (feature = "android_api_ge_9"))]                           // C 1048: C's `#else`
  { start = (directory_tree as *mut c_char).offset (9); }               // C 1049

  /* Figure out the current limit.  */                                  // C 1052
  limit = (directory_tree as *mut c_char).offset (directory_tree_size as isize);   // C 1053

  /* Now, split `file' into tokens, with the delimiter being the file
     name separator.  Look for the file and seek past it.  Create a copy
     of FILE for the enjoyment of `strtok_r'.  */                       // C 1055-1057

  ntokens = 0;                                                          // C 1059
  saveptr = std::ptr::null_mut ();                                      // C 1060
  len = strlen (file) + 1;                                              // C 1061
  copy = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;                   // C 1062
  memcpy (copy as *mut c_void, file as *const c_void, len);             // C 1063
  memset (&raw mut tokens as *mut c_void, 0, std::mem::size_of_val (&tokens));   // C 1064

  while { token = strtok_r (copy, c"/".as_ptr (), &raw mut saveptr);    // C 1066
	  ! token.is_null () }
    {
      copy = std::ptr::null_mut ();                                     // C 1068

      /* Make sure ntokens is within bounds.  */                        // C 1070
      if ntokens == countof! (tokens)                                   // C 1071
	{ break 'fail; }                                                // C 1072

      len = strlen (token) + 1;                                         // C 1074
      tokens[ntokens as usize] = SAFE_ALLOCA! (sa_avail, len) as *mut c_char;   // C 1075
      memcpy (tokens[ntokens as usize] as *mut c_void, token as *const c_void, len);   // C 1076
      ntokens += 1;                                                     // C 1077
    }

  /* If there are no tokens, just return the start of the directory
     tree.  */                                                          // C 1080-1081

  if ntokens == 0                                                       // C 1083
    {
      SAFE_FREE! (sa_count);                                            // C 1085

      /* Return the size of the directory tree as the limit.
         Do not subtract the initial header bytes, as the limit
         is an offset from the start of the file.  */                  // C 1087-1089

      if ! limit_return.is_null ()                                      // C 1091
	{ *limit_return = directory_tree_size; }                        // C 1092

      return start;                                                     // C 1094
    }

  /* Loop through tokens, indexing the directory tree each time.  */    // C 1097

  i = 0;                                                                // C 1099 (the `for` init)
  while i < ntokens
    {
      /* C's `continue` (C 1146) jumps to the `for`'s increment, which runs at the body's end, so
	 the body is a labelled block and `break 'continue_;` leaves it - the `src/androidterm.rs`
	 275-290 shape.  */
      'continue_: {
      token = tokens[i as usize];                                       // C 1101

      /* Figure out how many bytes to compare.  */                      // C 1103
      token_length = strlen (token);                                    // C 1104

      'again: loop                                                      // C 1106
	{
	  /* If this would be past the directory, return NULL.  */      // C 1108
	  if start.offset (token_length as isize) > limit               // C 1109
	    { break 'fail; }                                            // C 1110

	  /* Now compare the file name.  */                             // C 1112
	  if memcmp (start as *const c_void, token as *const c_void, token_length) == 0   // C 1113
	    {
	      /* They probably match.  Find the NULL byte.  It must be
		 either one byte past start + token_length, with the last
		 byte a trailing slash (indicating that it is a directory),
		 or just start + token_length.  Return 4 or 8 bytes past the
		 next NULL byte.  */                                    // C 1115-1119

	      'matched: {
	      max = memchr (start as *const c_void, 0, limit.offset_from (start) as usize) as *mut c_char;   // C 1121

	      if max != start.offset (token_length as isize)            // C 1123
		  && ! (max == start.offset (token_length as isize + 1)   // C 1124
			&& *max.offset (-1) == b'/' as c_char)          // C 1125
		{ break 'matched; }                                     // C 1126

	      /* Return it if it exists and is in range, and this is the
		 last token.  Otherwise, set it as start and the limit as
		 start + the offset and continue the loop.  */          // C 1128-1130

	      if ! max.is_null ()                                       // C 1132
		  && max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize) <= limit
		{
		  if i < ntokens - 1                                    // C 1134
		    {
		      start = max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize);   // C 1136
		      limit = ((directory_tree as *mut c_char)           // C 1137
			       .offset (android_extract_long (max.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as isize));   // C 1138-1139

		      /* Make sure limit is still in range.  */         // C 1141
		      if limit > (directory_tree as *mut c_char).offset (directory_tree_size as isize)   // C 1142
			  || start > (directory_tree as *mut c_char).offset (directory_tree_size as isize)   // C 1143
			{ break 'fail; }                                // C 1144

		      break 'continue_;                                 // C 1146
		    }

		  /* Now see if max is not a directory and file is.  If
		     file is a directory, then return NULL.  */         // C 1149-1150
		  if *max.offset (-1) != b'/' as c_char                 // C 1151
		      && *file.offset (strlen (file) as isize - 1) == b'/' as c_char
		    { max = std::ptr::null_mut (); }                    // C 1152
		  else
		    {
		      /* Figure out the limit.  */                      // C 1155
		      if ! limit_return.is_null ()                      // C 1156
			{
			  *limit_return                                 // C 1157
			    = android_extract_long (max.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as size_t;   // C 1158-1159
			}

		      /* Go to the end of this file.  */                // C 1161
		      max = max.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize);   // C 1162
		    }

		  SAFE_FREE! (sa_count);                                // C 1165
		  return max;                                           // C 1166
		}

	      /* Return NULL otherwise.  */                             // C 1169
	      let __func__: *const c_char = c"android_scan_directory_tree".as_ptr ();   // C 1170
	      __android_log_print (ANDROID_LOG_WARN, __func__,          // C 1170
				   c"could not scan to end of directory tree".as_ptr (),
				   c": %s".as_ptr (), file);            // C 1171-1172
	      break 'fail;                                              // C 1173
	      }
	    }

	  /* No match was found.  Set start to the next sibling and try
	     again.  */                                                 // C 1178-1179

	  start = memchr (start as *const c_void, 0, limit.offset_from (start) as usize) as *mut c_char;   // C 1181

	  if start.is_null ()                                           // C 1183
	      || start.offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize) > limit
	    { break 'fail; }                                            // C 1184

	  start = (directory_tree as *mut c_char)                       // C 1186
	    .offset (android_extract_long (start.offset ((if OLD_ANDROID_ASSETS != 0 { 5 } else { 1 }) as isize)) as isize);   // C 1187-1188

	  /* Make sure start is still in bounds.  */                    // C 1190

	  if start > limit                                              // C 1192
	    { break 'fail; }                                            // C 1193

	  /* Continue the loop.  */                                     // C 1195
	  /* C 1196's `goto again` is this `loop`'s own repeat.  */
	}
      }
      i += 1;                                                           // C 1099's `++i`
    }
  }

  /* C 1199's `fail:` */
  SAFE_FREE! (sa_count);                                                // C 1200
  return std::ptr::null ();                                             // C 1201
}

/* Return whether or not the directory tree entry DIR is a
   directory.

   DIR should be a value returned by
   `android_scan_directory_tree'.  */                                   // C 1204-1208

// C 1210-1229.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_is_directory (dir: *const c_char) -> bool
{
  /* If the directory is the directory tree, then it is a
     directory.  */                                                     // C 1213-1214
  if dir == (directory_tree as *const c_char)                            // C 1215
	     .offset ((if OLD_ANDROID_ASSETS != 0 { 9 } else { 5 }) as isize)
    { return true; }                                                    // C 1216

  /* Otherwise, look 5 bytes behind.  If it is `/', then it is a
     directory.  */                                                     // C 1219-1220
  #[cfg (feature = "android_api_ge_9")]                                  // C 1218: C's `#if !OLD_ANDROID_ASSETS`
  return dir.offset (-6) >= directory_tree as *const c_char              // C 1221
	 && *dir.offset (-6) == b'/' as c_char;                          // C 1222
  /* Otherwise, look 9 bytes behind.  If it is `/', then it is a
     directory.  */                                                     // C 1224-1225
  #[cfg (not (feature = "android_api_ge_9"))]                            // C 1223: C's `#else`
  return dir.offset (-10) >= directory_tree as *const c_char             // C 1226
	 && *dir.offset (-10) == b'/' as c_char;                         // C 1227
}

/* Initialize asset retrieval.  ENV should be a JNI environment for
   the Emacs thread, and MANAGER should be a local reference to a Java
   asset manager object created for the Emacs service context.  */     // C 1231-1233

/* The five `<android/asset_manager.h>` entries this function calls, **for the `>= 9` arm only**:
   that arm reaches the NDK's header directly (C 47-48) and it has no mirror in this tree, while the
   `#else` arm gets all five from C 51's `android-asset.h` mirror (`include/android-asset.rs`
   136/138/257/565/587 - that arm's emulation *defines* the four functions, this arm's header
   declares them).  Measured: the arm that lacks them reads `E0425` at each call (the slice-3
   lesson, applied before writing this time).  */
/* The four's return types are the NDK header's own; `off_t` is already in scope from C 32's
   `sys_types.in.rs` edge (`femacs/lib/include/sys_types.in.rs`), so it is not re-declared here
   (measured: a local alias is `E0428 the name \`off_t\` is defined multiple times`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
unsafe extern "C" {
  /// C: `AAssetManager *AAssetManager_fromJava (JNIEnv *env, jobject assetManager);`
  pub fn AAssetManager_fromJava (env: *mut JNIEnv, java_manager: jobject) -> *mut AAssetManager;
  /// C: `AAsset *AAssetManager_open (AAssetManager *mgr, const char *filename, int mode);`
  pub fn AAssetManager_open (manager: *mut AAssetManager, c_name: *const c_char, mode: c_int)
			     -> *mut AAsset;
  /// C: `const void *AAsset_getBuffer (AAsset *asset);`
  pub fn AAsset_getBuffer (asset: *mut AAsset) -> *const c_void;
  /// C: `off_t AAsset_getLength (AAsset *asset);`
  pub fn AAsset_getLength (asset: *mut AAsset) -> off_t;
}
/// C 171-176's anonymous enum in `<android/asset_manager.h>`: `AASSET_MODE_BUFFER == 1` (the
/// `#else` arm's `include/android-asset.rs` 136 carries the same value; measured with this
/// machine's NDK clang, the `src/android.rs` 135 table's reading).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
pub const AASSET_MODE_BUFFER: c_int = 1;
/// C 1249's priority: `<android/log.h>`'s `ANDROID_LOG_FATAL == 7` (the measured value
/// `src/android.rs` 141-143 records).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_FATAL: c_int = 7;

// C 1235-1287.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_init_assets (env: *mut JNIEnv, manager: jobject)
{
  let mut asset: *mut AAsset;                                           // C 1238

  /* Set the asset manager.  */                                         // C 1240
  asset_manager = AAssetManager_fromJava (env, manager);                // C 1241

  /* Initialize the directory tree.  */                                 // C 1243
  asset = AAssetManager_open (asset_manager, c"directory-tree".as_ptr (),   // C 1244
			      AASSET_MODE_BUFFER);                      // C 1245

  if asset.is_null ()                                                   // C 1247
    {
      let __func__: *const c_char = c"android_init_assets".as_ptr ();    // C 1249
      __android_log_print (ANDROID_LOG_FATAL, __func__,                 // C 1249
			   c"Failed to open directory tree".as_ptr ());  // C 1250
      emacs_abort ();                                                   // C 1251
    }

  directory_tree = AAsset_getBuffer (asset) as *const c_char;            // C 1254

  if directory_tree.is_null ()                                          // C 1256
    { emacs_abort (); }                                                 // C 1257

  /* Now figure out how big the directory tree is, and compare the
     first few bytes.  */                                              // C 1259-1260
  directory_tree_size = AAsset_getLength (asset) as size_t;             // C 1261
  #[cfg (feature = "android_api_ge_9")]                                  // C 1262: C's `#if !OLD_ANDROID_ASSETS`
  {
    if directory_tree_size < 5                                          // C 1263
	|| memcmp (directory_tree as *const c_void, c"EMACS".as_ptr () as *const c_void, 5) != 0   // C 1264
      {
	let __func__: *const c_char = c"android_init_assets".as_ptr ();  // C 1266
	__android_log_print (ANDROID_LOG_FATAL, __func__,                // C 1266
			     c"Directory tree has bad magic".as_ptr ());  // C 1267
	emacs_abort ();                                                  // C 1268
      }
  }
  #[cfg (not (feature = "android_api_ge_9"))]                            // C 1270: C's `#else`
  {
    if directory_tree_size < 9                                           // C 1271
	|| memcmp (directory_tree as *const c_void, c"EMACS____".as_ptr () as *const c_void, 9) != 0   // C 1272
      {
	let __func__: *const c_char = c"android_init_assets".as_ptr ();  // C 1274
	__android_log_print (ANDROID_LOG_FATAL, __func__,                // C 1274
			     c"Directory tree has bad magic".as_ptr ());  // C 1275
	emacs_abort ();                                                  // C 1276
      }
  }

  /* Hold a VM reference to the asset manager to prevent the native
     object from being deleted.  */                                    // C 1280-1281
  ((*(*env)).NewGlobalRef.unwrap ()) (env, manager);                     // C 1282

  /* Abort if there's no more memory for the global reference.  */       // C 1284
  if ((*(*env)).ExceptionCheck.unwrap ()) (env) != 0                     // C 1285 (C's implicit
									//  bool test: the JNI entry returns
									//  `jboolean`, a `u8`)
    { abort (); }                                                       // C 1286
}

/* Asset-to-file descriptor conversion.  */                            // C 1291

/* Pointer to the `ASharedMemory_create' function which is loaded
   dynamically.  */                                                    // C 1293-1294
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut asharedmemory_create: Option<unsafe extern "C" fn (name: *const c_char,
							      size: size_t) -> c_int> = None;   // C 1295

/* The `<sys/mman.h>` and `<dlfcn.h>` names this function and the next use, none of which has a
   mirror in this tree (slice 1's head lists both headers as mirrorless): the two calls keep
   `src/pdumper.rs` 104-112's declarations, the three constants are the frozen target's
   (`MAP_FAILED` as `(void *) -1`, `src/pdumper.rs` 186; `PROT_WRITE` 0x02, `src/gmalloc.rs` 573,
   here the Linux/Android value the NDK's own `<sys/mman.h>` gives).  `android_cache_dir` is C's
   `android.h` 49 object: `include/android.rs` 227 carries it, but that mirror is not in this
   unit's include graph (its own head says which `android.h` edges this file takes), so the
   declaration stands here (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `void *mmap (void *, size_t, int, int, int, off_t);`
  pub fn mmap (addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int,
	       offset: off_t) -> *mut c_void;
  /// C: `int munmap (void *, size_t);`
  pub fn munmap (addr: *mut c_void, len: size_t) -> c_int;
}
/// C: `<sys/mman.h>`: `#define PROT_WRITE 0x2`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const PROT_WRITE: c_int = 0x2;
/// C: `<sys/mman.h>`: `#define MAP_SHARED 0x01`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_SHARED: c_int = 0x01;
/// C: `<sys/mman.h>`: `#define MAP_FAILED ((void *) -1)`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const MAP_FAILED: *mut c_void = usize::MAX as *mut c_void;
/// C: android.h 49: `extern char *android_cache_dir;`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub static mut android_cache_dir: *mut c_char;
}
/// C 1344's priority: `<android/log.h>`'s `ANDROID_LOG_ERROR == 6` - C 1263/1273's `WARN`/`FATAL`
/// neighbours are already declared above, and all four values are the NDK enum this machine's
/// headers give (measured: `NDK 25.2.9519653`'s `android/log.h` 73-90 numbers `INFO` 4, `WARN` 5,
/// `ERROR` 6, `FATAL` 7).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_LOG_ERROR: c_int = 6;
/* C 1340's `AAsset_read`: the `#else` arm's mirror defines it (with the emulation's own body,
   `include/android-asset.rs` 613), the `>= 9` arm's NDK header declares it (asset_manager.h 131,
   measured) - so the declaration stands under that arm, the slice-4 rule.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
unsafe extern "C" {
  /// C: `int AAsset_read (AAsset *asset, void *buf, size_t count);`
  pub fn AAsset_read (asset: *mut AAsset, buffer: *mut c_void, count: size_t) -> c_int;
}

/* Do the same as android_hack_asset_fd, but use an unlinked temporary
   file to cater to old Android kernels where ashmem files are not
   readable.  */                                                       // C 1297-1299

// C 1301-1355.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_hack_asset_fd_fallback (asset: *mut AAsset) -> c_int
{
  let mut fd: c_int;                                                    // C 1304
  let mut filename: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];   // C 1305
  let mut size: size_t;                                                 // C 1306
  let mut mem: *mut c_void;                                             // C 1307

  'fail: {

  /* Assets must be small enough to fit in size_t, if off_t is
     larger.  */                                                        // C 1309-1310
  size = AAsset_getLength (asset) as size_t;                            // C 1311 (the mirror takes
									// C's `off_t` back as `c_long`, the NDK's
									// as `off_t`; both are 64-bit here)

  /* Get an unlinked file descriptor from a file in the cache
     directory, which is guaranteed to only be written to by Emacs.
     Creating an ashmem file descriptor and reading from it doesn't
     work on these old Android versions.  */                            // C 1313-1316

  snprintf (&raw mut filename as *mut c_char, PATH_MAX as usize,          // C 1318
	    c"%s/temp~unlinked.%d".as_ptr (),                            // C 1318
	    android_cache_dir, getpid ());                               // C 1319
  fd = open (&raw mut filename as *const c_char,                         // C 1320
	     O_CREAT | O_RDWR | O_TRUNC,                                 // C 1320
	     (S_IRUSR | S_IWUSR) as c_int);                               // C 1321 (C's own variadic
									//  promotion of `mode_t` to
									//  `int`, the C 1190 call's spelling)

  if fd < 0                                                             // C 1323
    { return -1; }                                                      // C 1324

  if unlink (&raw mut filename as *const c_char) != 0 && *errno () != ENOENT   // C 1326
    { break 'fail; }                                                     // C 1327

  if ftruncate (fd, size as off_t) != 0                                  // C 1329
    { break 'fail; }                                                     // C 1330

  mem = mmap (std::ptr::null_mut (), size, PROT_WRITE, MAP_SHARED, fd, 0);   // C 1332
  if mem == MAP_FAILED                                                   // C 1333
    {
      let __func__: *const c_char = c"android_hack_asset_fd_fallback".as_ptr ();   // C 1335
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1335
			   c"mmap: %s".as_ptr (),                        // C 1336
			   strerror (*errno ()));                        // C 1336
      break 'fail;                                                       // C 1337
    }

  if AAsset_read (asset, mem, size) as i64 != size as i64                // C 1340
    {
      /* Too little was read.  Close the file descriptor and
	 report an error.  */                                           // C 1342-1343
      let __func__: *const c_char = c"android_hack_asset_fd_fallback".as_ptr ();   // C 1344
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1344
			   c"AAsset_read: %s".as_ptr (),                 // C 1345
			   strerror (*errno ()));                        // C 1345
      break 'fail;                                                       // C 1346
    }

  munmap (mem, size);                                                   // C 1349
  return fd;                                                            // C 1350
  }

  /* C 1352's `fail:` */
  close (fd);                                                           // C 1353
  return -1;                                                            // C 1354
}

/* C 1363-1365's function needs five names no mirror this unit includes carries: the `<dlfcn.h>`
   pair C 1413/1567 names (whose two `RTLD_*` values the head's note already lists as this file's),
   the `<sys/ioctl.h>` entry C 1387/1395/1509/1520 calls, the two `<linux/ashmem.h>` constants it
   passes, and android.h 148's `android_get_current_api_level`, which C 1377 and C 1499 test.
   Each stands at its use site (P-88 rule 1), the `femacs/exec/trace.rs` 503-513 and
   `femacs/src/inotify.rs` 222 shapes.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: `<dlfcn.h>`: `void *dlsym (void *, const char *);`
  pub fn dlsym (handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
  /// C: `<sys/ioctl.h>`: `int ioctl (int, unsigned long, ...);`
  pub fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /// C: android.h 148: `extern int android_get_current_api_level (void) __attribute__ ((pure));`
  pub fn android_get_current_api_level () -> c_int;
}
/// C: `<dlfcn.h>`: `#define RTLD_DEFAULT ((void *) 0)` - measured 0 in bionic's header with this
/// machine's NDK (the `exec/trace.rs` 508 precedent).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut ();
/// C: `<linux/ashmem.h>` 22: `#define ASHMEM_SET_NAME _IOW (__ASHMEMIOC, 1, char[ASHMEM_NAME_LEN])` -
/// measured **0x41007701** with this machine's NDK 29.0.14206865 (its `clang`,
/// `--target=aarch64-linux-android24`: `__ASHMEMIOC` is 0x77, `ASHMEM_NAME_LEN` 256, and `_IOW`
/// packs `1U << 30 | sizeof << 16 | type << 8 | nr`).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ASHMEM_SET_NAME: c_ulong = 0x41007701;
/// C: `<linux/ashmem.h>` 24: `#define ASHMEM_SET_SIZE _IOW (__ASHMEMIOC, 3, size_t)` - measured
/// **0x40087703** (the same measurement; `size_t` is 8 bytes on the target it expands for).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ASHMEM_SET_SIZE: c_ulong = 0x40087703;

/* Return whether or not shared memory file descriptors can also be
   read from, and are thus suitable for creating asset files.

   This does not work on some ancient Android systems running old
   versions of the kernel.  */                                        // C 1357-1361

// C 1363-1465.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_detect_ashmem () -> bool
{
  let mut fd: c_int;                                                   // C 1366
  let mut rc: c_int;                                                   // C 1366
  let mut mem: *mut c_void;                                            // C 1367
  let mut test_buffer: [c_char; 10] = [0; 10];                         // C 1368

  memcpy (&raw mut test_buffer as *mut c_void,                         // C 1370
	  c"abcdefghi".as_ptr () as *const c_void, 10);                // C 1370

  /* Create the file descriptor to be used for the test.  */            // C 1372

  /* Android 28 and earlier let Emacs access /dev/ashmem directly, so
     prefer that over using ASharedMemory.  */                         // C 1374-1375

  if android_get_current_api_level () <= 28                            // C 1377
    {
      fd = open (c"/dev/ashmem".as_ptr (), O_RDWR);                     // C 1379

      if fd < 0                                                        // C 1381
	{ return false; }                                              // C 1382

      /* An empty name means the memory area will exist until the file
	 descriptor is closed, because no other process can
	 attach.  */                                                   // C 1384-1386
      rc = ioctl (fd, ASHMEM_SET_NAME, c"".as_ptr ());                  // C 1387

      if rc < 0                                                        // C 1389
	{
	  close (fd);                                                  // C 1391
	  return false;                                                // C 1392
	}

      rc = ioctl (fd, ASHMEM_SET_SIZE,                                 // C 1395
		  std::mem::size_of_val (&test_buffer));               // C 1395

      if rc < 0                                                        // C 1397
	{
	  close (fd);                                                  // C 1399
	  return false;                                                // C 1400
	}
    }
  else
    {
      /* On the other hand, SELinux restrictions on Android 29 and
	 later require that Emacs use a system service to obtain
	 shared memory.  Load this dynamically, as this service is not
	 available on all versions of the NDK.  */                     // C 1405-1408

      if asharedmemory_create.is_none ()                               // C 1410
	{
	  *((&raw mut asharedmemory_create) as *mut *mut c_void)       // C 1412
	    = dlsym (RTLD_DEFAULT, c"ASharedMemory_create".as_ptr ()); // C 1413

	  if asharedmemory_create.is_none ()                           // C 1415
	    {
	      let __func__: *const c_char = c"android_detect_ashmem".as_ptr ();   // C 1417
	      __android_log_print (ANDROID_LOG_FATAL, __func__,        // C 1417
				   c"dlsym: %s\n".as_ptr (),           // C 1418
				   strerror (*errno ()));              // C 1419
	      emacs_abort ();                                          // C 1420
	    }
	}

      fd = (asharedmemory_create.unwrap ()) (c"".as_ptr (),            // C 1424
					     std::mem::size_of_val (&test_buffer));   // C 1424

      if fd < 0                                                        // C 1426
	{ return false; }                                              // C 1427
    }

  /* Now map the resource and write the test contents.  */              // C 1430

  mem = mmap (std::ptr::null_mut (), std::mem::size_of_val (&test_buffer), PROT_WRITE,
	      MAP_SHARED, fd, 0);                                      // C 1432-1433
  if mem == MAP_FAILED                                                 // C 1434
    {
      close (fd);                                                      // C 1436
      return false;                                                    // C 1437
    }

  /* Copy over the test contents.  */                                   // C 1440
  memcpy (mem, &raw const test_buffer as *const c_void,                // C 1442
	  std::mem::size_of_val (&test_buffer));                       // C 1441

  /* Return anyway even if munmap fails.  */                            // C 1443
  munmap (mem, std::mem::size_of_val (&test_buffer));                  // C 1444

  /* Try to read the content back into test_buffer.  If this does not
     compare equal to the original string, or the read fails, then
     ashmem descriptors are not readable on this system.  */           // C 1446-1448

  if (read (fd, &raw mut test_buffer as *mut c_void,                   // C 1452
	    std::mem::size_of_val (&test_buffer)) as i64               // C 1450
      != std::mem::size_of_val (&test_buffer) as i64                   // C 1451
      || memcmp (&raw const test_buffer as *const c_void,              // C 1452
		 c"abcdefghi".as_ptr () as *const c_void,              // C 1452
		 std::mem::size_of_val (&test_buffer)) != 0)           // C 1452
    {
      let __func__: *const c_char = c"android_detect_ashmem".as_ptr ();   // C 1454
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1454
			   concat! ("/dev/ashmem does not produce real",
				    " temporary files on this system, so",
				    " Emacs will fall back to creating",
				    " unlinked temporary files.", "\0").as_ptr () as *const c_char);   // C 1455-1458
      close (fd);                                                      // C 1459
      return false;                                                    // C 1460
    }

  close (fd);                                                          // C 1463
  return true;                                                         // C 1464
}

/* Get a file descriptor backed by a temporary in-memory file for the
   given asset.  */                                                    // C 1467-1468

// C 1470-1611.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_hack_asset_fd (asset: *mut AAsset) -> c_int
{
  /* C's `static bool` pair is zero-initialized in C, and a block-scoped `static mut` is where
     those two objects live here (the `include/android-asset.rs` 61 tentative-definition rule's
     other half: these two are the function's own, not a header's).  */
  static mut ashmem_readable_p: bool = false;                          // C 1473
  static mut ashmem_initialized: bool = false;                         // C 1474
  let mut fd: c_int;                                                   // C 1475
  let mut rc: c_int;                                                   // C 1475
  let mut mem: *mut c_uchar;                                           // C 1476
  let mut size: size_t;                                                // C 1477

  /* The first time this function is called, try to determine whether
     or not ashmem file descriptors can be read from.  */              // C 1479-1480

  if !ashmem_initialized                                                // C 1482
    { ashmem_readable_p = android_detect_ashmem (); }                  // C 1483-1484
  ashmem_initialized = true;                                            // C 1485

  /* If it isn't, fall back.  */                                        // C 1487

  if !ashmem_readable_p                                                 // C 1489
    { return android_hack_asset_fd_fallback (asset); }                  // C 1490

  /* Assets must be small enough to fit in size_t, if off_t is
     larger.  */                                                        // C 1492-1493
  size = AAsset_getLength (asset) as size_t;                            // C 1494

  /* Android 28 and earlier let Emacs access /dev/ashmem directly, so
     prefer that over using ASharedMemory.  */                         // C 1496-1497

  if android_get_current_api_level () <= 28                            // C 1499
    {
      fd = open (c"/dev/ashmem".as_ptr (), O_RDWR);                     // C 1501

      if fd < 0                                                        // C 1503
	{ return -1; }                                                 // C 1504

      /* An empty name means the memory area will exist until the file
	 descriptor is closed, because no other process can
	 attach.  */                                                   // C 1506-1508
      rc = ioctl (fd, ASHMEM_SET_NAME, c"".as_ptr ());                  // C 1509

      if rc < 0                                                        // C 1511
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1513
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1513
			       c"ioctl ASHMEM_SET_NAME: %s".as_ptr (),   // C 1514
			       strerror (*errno ()));                    // C 1515
	  close (fd);                                                    // C 1516
	  return -1;                                                     // C 1517
	}

      rc = ioctl (fd, ASHMEM_SET_SIZE, size);                            // C 1520

      if rc < 0                                                        // C 1522
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1524
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1524
			       c"ioctl ASHMEM_SET_SIZE: %s".as_ptr (),   // C 1525
			       strerror (*errno ()));                    // C 1526
	  close (fd);                                                    // C 1527
	  return -1;                                                     // C 1528
	}

      if size == 0                                                       // C 1531
	{ return fd; }                                                   // C 1532

      /* Now map the resource.  */                                      // C 1534
      mem = mmap (std::ptr::null_mut (), size, PROT_WRITE,              // C 1535
		  MAP_SHARED, fd, 0) as *mut c_uchar;                    // C 1535
      if mem as *mut c_void == MAP_FAILED                                // C 1536
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1538
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1538
			       c"mmap: %s".as_ptr (),                    // C 1539
			       strerror (*errno ()));                    // C 1539
	  close (fd);                                                    // C 1540
	  return -1;                                                     // C 1541
	}

      if AAsset_read (asset, mem as *mut c_void, size) as i64 != size as i64   // C 1544
	{
	  /* Too little was read.  Close the file descriptor and
	     report an error.  */                                        // C 1546-1547
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1548
	  __android_log_print (ANDROID_LOG_ERROR, __func__,              // C 1548
			       c"AAsset_read: %s".as_ptr (),             // C 1549
			       strerror (*errno ()));                    // C 1549
	  close (fd);                                                    // C 1550
	  return -1;                                                     // C 1551
	}

      /* Return anyway even if munmap fails.  */                         // C 1553
      munmap (mem as *mut c_void, size);                                 // C 1554
      return fd;                                                         // C 1555
    }

  /* On the other hand, SELinux restrictions on Android 29 and later
     require that Emacs use a system service to obtain shared memory.
     Load this dynamically, as this service is not available on all
     versions of the NDK.  */                                           // C 1557-1561

  if asharedmemory_create.is_none ()                                    // C 1563
    {
      *((&raw mut asharedmemory_create) as *mut *mut c_void)            // C 1566
	= dlsym (RTLD_DEFAULT, c"ASharedMemory_create".as_ptr ());      // C 1567

      if asharedmemory_create.is_none ()                                // C 1569
	{
	  let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1571
	  __android_log_print (ANDROID_LOG_FATAL, __func__,                  // C 1571
			       c"dlsym: %s\n".as_ptr (),                        // C 1572
			       strerror (*errno ()));                           // C 1573
	  emacs_abort ();                                                    // C 1574
	}
    }

  fd = (asharedmemory_create.unwrap ()) (c"".as_ptr (), size);           // C 1578

  if fd < 0                                                              // C 1580
    {
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1582
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1582
			   c"ASharedMemory_create: %s".as_ptr (),         // C 1583
			   strerror (*errno ()));                        // C 1583
      return -1;                                                         // C 1584
    }

  /* Now map the resource.  */                                           // C 1587
  mem = mmap (std::ptr::null_mut (), size, PROT_WRITE, MAP_SHARED, fd, 0) as *mut c_uchar;   // C 1588
  if mem as *mut c_void == MAP_FAILED                                    // C 1589
    {
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1591
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1591
			   c"mmap: %s".as_ptr (),                           // C 1592
			   strerror (*errno ()));                           // C 1592
      close (fd);                                                        // C 1593
      return -1;                                                         // C 1594
    }

  if AAsset_read (asset, mem as *mut c_void, size) as i64 != size as i64  // C 1597
    {
      /* Too little was read.  Close the file descriptor and
	 report an error.  */                                            // C 1599-1600
      let __func__: *const c_char = c"android_hack_asset_fd".as_ptr ();   // C 1601
      __android_log_print (ANDROID_LOG_ERROR, __func__,                  // C 1601
			   c"AAsset_read: %s".as_ptr (),                 // C 1602
			   strerror (*errno ()));                        // C 1602
      close (fd);                                                        // C 1603
      return -1;                                                         // C 1604
    }

  /* Return anyway even if munmap fails.  */                             // C 1606
  munmap (mem as *mut c_void, size);                                     // C 1607
  return fd;                                                             // C 1608
}

/* ``Asset file system'' vnode implementation.  These vnodes map to
   asset files within the application package, provided by the Android
   ``asset manager''.  */                                              // C 1615-1617

/* C 1619-1629: `struct android_afs_vnode'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,                                            // C 1622

  /* Length of the name without a trailing null byte.  */
  pub name_length: size_t,                                             // C 1625

  /* Name of the vnode.  */
  pub name: *mut c_char,                                               // C 1628
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_vnode> () == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, name_length) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vnode, name) == 24);

/* C 1631-1650: `struct android_afs_vdir'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_vdir
{
  /* The directory function table.  */
  pub vdir: android_vdir,                                              // C 1634

  /* The next directory stream in `all_afs_vdirs'.  */
  pub next: *mut android_afs_vdir,                                     // C 1637

  /* Pointer to the directory in directory_tree.  */
  pub asset_dir: *mut c_char,                                          // C 1640

  /* And the end of the files in asset_dir.  */
  pub asset_limit: *mut c_char,                                        // C 1643

  /* Path to the directory relative to /.  */
  pub asset_file: *mut c_char,                                         // C 1646

  /* File descriptor representing this directory stream, or NULL.  */
  pub fd: c_int,                                                       // C 1649
}
/* Measured with the NDK's clang (the three above).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_vdir> () == 64);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, next) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_dir) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_limit) == 40);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, asset_file) == 48);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_vdir, fd) == 56);

/* C 1652-1662: `struct android_afs_open_fd'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_afs_open_fd
{
  /* The next table entry.  */
  pub next: *mut android_afs_open_fd,                                  // C 1655

  /* The open file descriptor.  */
  pub fd: c_int,                                                       // C 1658

  /* The stat buffer associated with this entry.  */
  pub statb: stat,                                                     // C 1661
}
/* Measured with the NDK's clang (the three above).  This struct embeds the *mirror's* `stat`,
   which models the frozen target: the NDK's own `struct stat` is 128 bytes on
   `aarch64-linux-android24` (measured), while `lib/include/sys_stat.in.rs` 108 asserts 144 and the
   port has only that one type - so the Rust layout here is 16 + 144 = 160, and the C-side number
   this file's fields come from is recorded in this note (the C 1471-1611 `AAsset_getLength`
   situation's shape: the field offsets the reference's `struct stat` gives are unchanged for the
   three members before it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_afs_open_fd> () == 160);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, next) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, fd) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_afs_open_fd, statb) == 16);

/* C 1664-1680: C's twelve forward declarations of this file's own `static` functions
   (`android_afs_name` … `android_afs_opendir`).  Rust needs no forward declaration for a function
   defined later in the same unit (PORTING.md 10.10), so this block has no text.  */

/* Chain consisting of all open asset directory streams.  */             // C 1701
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut all_afs_vdirs: *mut android_afs_vdir = std::ptr::null_mut ();   // C 1702

/* List linking open file descriptors to asset information.  This
   assumes Emacs does not use dup on regular files.  */                 // C 1704-1705
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut afs_file_descriptors: *mut android_afs_open_fd = std::ptr::null_mut ();   // C 1706

/* Make FD close-on-exec.  If any system call fails, do not abort, but
   log a warning to the system log.  */                                // C 1845-1846

/* C 1848-1870.  C calls it directly (C 1952/3391), never through a pointer, so it stays a
   Rust-ABI `unsafe fn` (the `android_unix_vnode` rule).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_close_on_exec (fd: c_int)
{
  let mut flags: c_int;                                                // C 1851
  let mut rc: c_int;                                                   // C 1851

  flags = fcntl (fd, F_GETFD);                                         // C 1853

  if flags < 0                                                         // C 1855
    {
      let __func__: *const c_char = c"android_close_on_exec".as_ptr ();   // C 1857
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1857
			   c"fcntl: %s".as_ptr (),                     // C 1858
			   strerror (*errno ()));                      // C 1858
      return;                                                          // C 1859
    }

  rc = fcntl (fd, F_SETFD, flags | O_CLOEXEC);                         // C 1862

  if rc < 0                                                            // C 1864
    {
      let __func__: *const c_char = c"android_close_on_exec".as_ptr ();   // C 1866
      __android_log_print (ANDROID_LOG_WARN, __func__,                 // C 1866
			   c"fcntl: %s".as_ptr (),                     // C 1867
			   strerror (*errno ()));                      // C 1867
      return;                                                          // C 1868
    }
}

/* `<dirent.h>`'s `struct dirent`, which the mirror does not carry (`lib/include/dirent.rs` 6: "not
   mirrored yet - no ported unit reads its fields"; this file is the first that does, at C
   2300-2336).  Declared here at the use site (P-88 ①), and measured with the NDK's clang
   (`-Xclang -fdump-record-layouts-complete`, `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dirent
{
  pub d_ino: u64,                                                      // C: `ino_t d_ino`
  pub d_off: i64,                                                      // C: `off64_t d_off`
  pub d_reclen: c_ushort,                                              // C: `unsigned short`
  pub d_type: c_uchar,                                                 // C: `unsigned char`
  pub d_name: [c_char; 256],                                           // C: `char d_name[256]`
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<dirent> () == 280);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (dirent, d_ino) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (dirent, d_off) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (dirent, d_reclen) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (dirent, d_type) == 18);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (dirent, d_name) == 19);

/* C 1903/2134's `AASSET_MODE_STREAMING`: the eighth name of the `<android/asset_manager.h>`
   family, and here the arm question again answers "the `>= 9` arm alone" - the `#else` arm's
   mirror defines it (`include/android-asset.rs` 135), the NDK's header declares it (measured).  */
/* The constant stands under the `>= 9` arm alone: no extern block is needed, because `#define`d
   names have no linkage (the slice-4 five and slice-5's `AAsset_read` were declarations; this one
   is a constant, so the arm's whole text is the constant).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
/// C: `<android/asset_manager.h>`: `#define AASSET_MODE_STREAMING 0` (the mirror's 135 has the
/// same value; measured from this machine's NDK 29.0.14206865).
pub const AASSET_MODE_STREAMING: c_int = 0;
/* C 1943/1993/2153's `AAsset_close`: the same arm split again - the `#else` arm's mirror defines it
   (`include/android-asset.rs` 385), the `>= 9` arm's NDK header declares it (measured).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[cfg (feature = "android_api_ge_9")]
unsafe extern "C" {
  /// C: `<android/asset_manager.h>`: `void AAsset_close (AAsset *asset);`
  pub fn AAsset_close (asset: *mut AAsset);
}

/* C 2323/2325's `DT_DIR`/`DT_REG`: `<dirent.h>`'s `d_type` values, which `lib/include/dirent.rs`
   does not carry (it keeps the opaque `DIR` and gnulib's `_GL_DT_NOTDIR` only).  Declared at the
   use site (P-88 ①), and measured with this machine's NDK 29.0.14206865: `DT_UNKNOWN` 0,
   `DT_DIR` 4, `DT_REG` 8 (the values `lib/include/dirent.in.rs` 87/89 records for the template).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const DT_DIR: c_int = 4;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const DT_REG: c_int = 8;
/// C 5617's `DT_UNKNOWN`: the same header's `d_type` value for "not known", measured 0 with this
/// machine's NDK (the block above's note).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const DT_UNKNOWN: c_int = 0;

/* C 1978/2160's `emacs_installation_time`: android.h 338's `extern struct timespec
   emacs_installation_time;`.  `include/android.rs` 238 carries it, but that mirror is not in
   this unit's include graph (the slice-5 `android_cache_dir` case), so the declaration stands
   here (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: android.h 338: `extern struct timespec emacs_installation_time;`
  pub static mut emacs_installation_time: timespec;
}

/* C 1708: `static struct android_vnode *android_afs_name (struct android_vnode *vnode, char
   *name, size_t length)`.  C's `goto parent_vnode` (C 1725) and the `parent_vnode:` label at
   C 1772 become one labelled block (PORTING.md 2.2), the `android_unix_name` shape above.  C 1819
   calls through the vnode's own operation vector, the port's first such call (the
   `femacs/src/androidfns.rs` 2943 `.unwrap () (…)` spelling).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_name (mut vnode: *mut android_vnode, name: *mut c_char,
				       mut length: size_t) -> *mut android_vnode
{
  let mut j: size_t;                                                   // C 1712
  let mut remainder: *mut c_char;                                      // C 1713
  let mut fill: *mut c_char;                                           // C 1713
  let mut vp: *mut android_afs_vnode;                                  // C 1714
  let mut input: *mut android_afs_vnode;                               // C 1714
  let mut temp: android_afs_vnode;                                     // C 1715

  /* C leaves `vp` and `temp` uninitialised (PORTING.md 1.5); `input`, `fill`, `remainder` and `j`
     are assigned before any use on every path.  */
  j = 0;
  remainder = std::ptr::null_mut ();
  fill = std::ptr::null_mut ();
  vp = std::ptr::null_mut ();
  input = std::ptr::null_mut ();
  temp = unsafe { std::mem::zeroed () };

  'parent_vnode: {
    input = vnode as *mut android_afs_vnode;                           // C 1717

    /* Canonicalize NAME.  */                                          // C 1719
    remainder = android_vfs_canonicalize_name (name, &raw mut length); // C 1720

    /* If remainder is set, it's a name relative to the parent
       vnode.  */                                                      // C 1722-1723
    if ! remainder.is_null ()                                          // C 1724
      { break 'parent_vnode; }                                         // C 1725

    /* Allocate a new vnode.  */                                       // C 1727
    vp = xmalloc (std::mem::size_of::<android_afs_vnode> ()) as *mut android_afs_vnode;   // C 1728

    /* See the specified name is empty.  */                            // C 1730

    if length < 1                                                      // C 1732
      {
	memcpy (vp as *mut c_void, vnode as *const c_void,             // C 1734
		std::mem::size_of::<android_afs_vnode> ());            // C 1734
	(*vp).name = xstrdup ((*vp).name);                             // C 1735
	return &raw mut (*vp).vnode;                                   // C 1736
      }

    /* Recompute length.  */                                           // C 1739
    (*vp).vnode.ops = &raw mut afs_vfs_ops;                            // C 1740
    (*vp).vnode.r#type = ANDROID_VNODE_AFS as c_short;                 // C 1741
    (*vp).vnode.flags = 0;                                             // C 1742

    /* Generate the new name of the vnode.  Remove any trailing slash
       from vp->name.  */                                              // C 1744-1745

    (*vp).name_length = (*input).name_length + length;                 // C 1747
    (*vp).name = xmalloc ((*vp).name_length + 2) as *mut c_char;       // C 1748

    /* Copy the parent name over.  */                                  // C 1750
    fill = mempcpy ((*vp).name as *mut c_void,                         // C 1751
		    (*input).name as *const c_void,
		    (*input).name_length) as *mut c_char;              // C 1751

    /* Check if it contains a trailing slash.  input->name cannot be
       empty, as the root vnode's name is `/'.  */                     // C 1753-1754

    if *fill.offset (-1) != b'/' as c_char && *name != b'/' as c_char // C 1756
      /* If not, append a trailing slash and adjust vp->name_length
	 correspondingly.  */
      { *fill = b'/' as c_char; fill = fill.offset (1); (*vp).name_length += 1; }   // C 1759
    else if *fill.offset (-1) == b'/' as c_char && *name == b'/' as c_char   // C 1760
      /* If name has a leading slash and fill does too, move fill
	 backwards so that name's slash will override that of fill.  */
      { fill = fill.offset (-1); (*vp).name_length -= 1; }             // C 1763

    /* Now copy NAME.  */                                              // C 1765
    fill = mempcpy (fill as *mut c_void, name as *const c_void,        // C 1766
		    length) as *mut c_char;                            // C 1766

    /* And NULL terminate fill.  */                                    // C 1768
    *fill = 0;                                                         // C 1769
    return &raw mut (*vp).vnode;                                       // C 1770
  }

  /* parent_vnode: */
  /* .. was encountered and the parent couldn't be found through
     stripping off preceding components.

     Find the parent vnode and name the rest of NAME starting from
     there.  */                                                        // C 1772-1777

  if (*input).name_length == 1                                         // C 1779
    /* This is the vnode representing the /assets directory... */
    { vnode = &raw mut root_vnode.vnode; }                             // C 1781
  else
    {
      /* Create a temporary asset vnode within the parent and use it
	 instead.  First, establish the length of vp->name before its
	 last component.  */                                           // C 1783-1786

      j = (*input).name_length - 1;                                    // C 1788
      while j != 0
	{
	  if *(*input).name.offset ((j - 1) as isize) == b'/' as c_char   // C 1790
	    { break; }
	  j -= 1;                                                      // C 1788
	}

      /* There must be at least one leading directory separator in an
	 asset vnode's `name' field.  */                              // C 1794-1795

      if j == 0                                                        // C 1797
	{ abort (); }                                                  // C 1798

      /* j is now the length of the string minus the size of its last
	 component.  Create a temporary vnode with that as its
	 name.  */                                                     // C 1800-1802

      temp.vnode.ops = &raw mut afs_vfs_ops;                           // C 1804
      temp.vnode.r#type = ANDROID_VNODE_AFS as c_short;                // C 1805
      temp.vnode.flags = 0;                                            // C 1806
      temp.name_length = j;                                            // C 1807
      temp.name = xmalloc (j + 1) as *mut c_char;                      // C 1808
      fill = mempcpy (temp.name as *mut c_void, (*input).name as *const c_void,   // C 1809
		      j) as *mut c_char;                               // C 1809
      *fill = 0;                                                       // C 1810

      /* Search for the remainder of NAME relative to its parent.  */   // C 1812
      vnode = android_afs_name (&raw mut temp.vnode, remainder,        // C 1813
				strlen (remainder));                   // C 1814
      xfree (temp.name as *mut c_void);                                // C 1815
      return vnode;                                                    // C 1816
    }

  return ((*(*vnode).ops).name).unwrap () (vnode, remainder,           // C 1819
					   strlen (remainder));        // C 1819
}

/* Find the vnode designated by the normalized NAME relative to the
   root of the asset file system.  NAME may be modified, and must be
   LENGTH bytes long, excluding its terminating NULL byte.  */          // C 1822-1824

/* C 1826-1843.  C takes its address at C 6634 (`android_vfs_entries'), so it is an
   `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_initial (name: *mut c_char, length: size_t) -> *mut android_vnode
{
  let mut temp: android_afs_vnode;                                     // C 1829

  /* C leaves `temp` uninitialised (PORTING.md 1.5).  */
  temp = unsafe { std::mem::zeroed () };

  /* Create a temporary vnode at the root of the asset file
     system.  */                                                       // C 1831-1832

  temp.vnode.ops = &raw mut afs_vfs_ops;                               // C 1834
  temp.vnode.r#type = ANDROID_VNODE_AFS as c_short;                    // C 1835
  temp.vnode.flags = 0;                                                // C 1836
  temp.name_length = 1;                                                // C 1837
  temp.name = c"/".as_ptr () as *mut c_char;                           // C 1838

  /* Try to name this vnode.  If NAME is empty, it will be duplicated
     instead.  */                                                      // C 1840-1841
  return android_afs_name (&raw mut temp.vnode, name, length);         // C 1842
}

/* C 1872-2003.  Its address is taken at C 1686, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_open (vnode: *mut android_vnode, flags: c_int,
				       mode: mode_t, asset_p: bool, fd_return: *mut c_int,
				       asset_return: *mut *mut AAsset) -> c_int
{
  let mut asset: *mut AAsset;                                          // C 1877
  let mut vp: *mut android_afs_vnode;                                  // C 1878
  let mut asset_dir: *const c_char;                                    // C 1879
  let mut fd: c_int;                                                   // C 1880
  let mut info: *mut android_afs_open_fd;                              // C 1881

  /* C leaves `asset`, `vp`, `asset_dir`, `fd` and `info` uninitialised (PORTING.md 1.5); every
     one of them is assigned before its first read on every path.  */
  asset = std::ptr::null_mut ();
  vp = std::ptr::null_mut ();
  asset_dir = std::ptr::null ();
  fd = 0;
  info = std::ptr::null_mut ();

  vp = vnode as *mut android_afs_vnode;                                // C 1883

  /* Return suitable error indications for unsupported file
     operations.  */                                                   // C 1885-1886

  if (flags & O_WRONLY) != 0 || (flags & O_RDWR) != 0                  // C 1888
    {
      *errno () = EROFS;                                               // C 1890
      return -1;                                                       // C 1891
    }

  if (flags & O_DIRECTORY) != 0                                        // C 1894
    {
      *errno () = ENOSYS;                                              // C 1896
      return -1;                                                       // C 1897
    }

  /* Now try to open this asset.  Asset manager APIs expect there to
     be no trailing directory separator.  */                           // C 1900-1901
  asset = AAssetManager_open (asset_manager, (*vp).name.offset (1),    // C 1902
			      AASSET_MODE_STREAMING);                  // C 1903

  /* If it can't be opened, return an error indication.  */             // C 1905

  if asset.is_null ()                                                  // C 1907
    {
      /* Scan the directory tree for this file.  */                    // C 1909
      asset_dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 1910

      /* Default errno to ENOTENT.  */                                 // C 1912
      *errno () = ENOENT;                                              // C 1913

      /* Maybe the caller wants to open a directory vnode as a
	 file?  */                                                     // C 1915-1916

      if ! asset_dir.is_null () && android_is_directory (asset_dir)    // C 1918
	/* In that case, set errno to ENOSYS.  */
	{ *errno () = ENOSYS; }                                        // C 1920

      return -1;                                                       // C 1922
    }

  /* An asset has been opened.  If the caller wants a file descriptor,
     a temporary one must be created and the file contents read
     inside.  */                                                       // C 1925-1927

  if ! asset_p                                                         // C 1929
    {
      /* Create a shared memory file descriptor containing the asset
	 contents.

         The documentation misleads people into thinking that
         AAsset_openFileDescriptor does precisely this.  However, it
         instead returns an offset into any uncompressed assets in the
         ZIP archive.  This cannot be found in its documentation.  */  // C 1931-1937

      fd = android_hack_asset_fd (asset);                              // C 1939

      if fd == -1                                                      // C 1941
	{
	  AAsset_close (asset);                                        // C 1943
	  *errno () = EIO;                                             // C 1944
	  return -1;                                                   // C 1945
	}

      /* If O_CLOEXEC is specified, make the file descriptor close on
	 exec too.  */                                                 // C 1948-1949

      if (flags & O_CLOEXEC) != 0                                      // C 1951
	{ android_close_on_exec (fd); }                                // C 1952

      /* Keep a record linking ``hacked'' file descriptors with
	 their file status.  */                                        // C 1954-1955
      info = xzalloc (std::mem::size_of::<android_afs_open_fd> ()) as *mut android_afs_open_fd;   // C 1956
      (*info).fd = fd;                                                 // C 1957
      (*info).next = afs_file_descriptors;                             // C 1958

      /* Fill in some information that will be reported to
	 callers of android_fstat, among others.  */                   // C 1960-1961
      (*info).statb.st_mode = S_IFREG | S_IRUSR | S_IRGRP | S_IROTH;   // C 1962

      /* Owned by root.  */                                            // C 1964
      (*info).statb.st_uid = 0;                                        // C 1965
      (*info).statb.st_gid = 0;                                        // C 1966

      /* Concoct a nonexistent device and an inode number.  */         // C 1968
      (*info).statb.st_dev = -1;                                       // C 1969
      (*info).statb.st_ino = 0;                                        // C 1970

      /* Size of the file.  */                                         // C 1972
      (*info).statb.st_size = AAsset_getLength (asset) as off_t;       // C 1973

      /* If the installation date can be ascertained, return that as
	 the file's modification time.  */                            // C 1975-1976

      /* C 1980-1987's `#ifdef STAT_TIMESPEC` pair.  `STAT_TIMESPEC (st, st_mtim)` is
	 lib/stat-time.h 61's `(st)->st_xtim##espec` - a member name the macro composes from its
	 argument, which Rust cannot do (the `STAT_TIMESPEC` row in `docs/ports/excluded-macros.txt`),
	 so the live arm reads the mirror's member directly.  The `#else` arm names gnulib
	 `stat.h`'s override fields: the mirror carries `st_mtime` under `no_windows_stat_timespec`
	 but has no `st_mtime_nsec` at all, so that arm is **carried as a record** (the haikuterm
	 Deviation 1 device) and the gap is an Owed item.  */
      if timespec_valid_p (emacs_installation_time)                    // C 1978
	{
	  #[cfg (feature = "stat_timespec")]
	  { (*info).statb.st_mtimespec = emacs_installation_time; }    // C 1981 (`STAT_TIMESPEC
								       //  (&info->statb, st_mtim)`)
	  #[cfg (not (feature = "stat_timespec"))]
	  {
	    /* C 1985-1986:
	         info->statb.st_mtime = emacs_installation_time.tv_sec;
	         info->statb.st_mtime_nsec = emacs_installation_time.tv_nsec;  */
	  }
	}

      /* Chain info onto afs_file_descriptors.  */                     // C 1990
      afs_file_descriptors = info;                                     // C 1991

      AAsset_close (asset);                                            // C 1993

      /* Return the file descriptor.  */                               // C 1995
      *fd_return = fd;                                                 // C 1996
      return 0;                                                        // C 1997
    }

  /* Return the asset itself.  */                                      // C 2000
  *asset_return = asset;                                               // C 2001
  return 1;                                                            // C 2002
}

/* C 2005-2016.  Its address is taken at C 1687, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_afs_vnode;                                  // C 2008
  let mut save_errno: c_int;                                           // C 2009

  save_errno = *errno ();                                              // C 2011
  vp = vnode as *mut android_afs_vnode;                                // C 2012
  xfree ((*vp).name as *mut c_void);                                   // C 2013
  xfree (vp as *mut c_void);                                           // C 2014
  *errno () = save_errno;                                              // C 2015
}

/* C 2018-2036.  Its address is taken at C 1688.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_unlink (vnode: *mut android_vnode) -> c_int
{
  let mut dir: *const c_char;                                          // C 2021
  let mut vp: *mut android_afs_vnode;                                  // C 2022

  /* If the vnode already exists, return EROFS.  Else, return
     ENOENT.  */                                                       // C 2024-2025

  vp = vnode as *mut android_afs_vnode;                                // C 2027
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2028

  if ! dir.is_null ()                                                  // C 2030
    { *errno () = EROFS; }                                             // C 2031
  else
    { *errno () = ENOENT; }                                            // C 2033

  return -1;                                                           // C 2035
}

/* C 2038-2056.  Its address is taken at C 1689.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_symlink (linkname: *const c_char,
					  vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_afs_vnode;                                  // C 2041

  /* If this vnode already exists, return EEXIST.  */                   // C 2043
  vp = vnode as *mut android_afs_vnode;                                // C 2044

  if ! android_scan_directory_tree ((*vp).name, std::ptr::null_mut ()).is_null ()   // C 2046
    {
      *errno () = EEXIST;                                              // C 2048
      return -1;                                                       // C 2049
    }

  /* Symlinks aren't supported on this (read-only) ``file system'',
     so return -1 with EROFS.  */                                      // C 2052-2053
  *errno () = EROFS;                                                   // C 2054
  return -1;                                                           // C 2055
}

/* C 2058-2078.  Its address is taken at C 1690.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_rmdir (vnode: *mut android_vnode) -> c_int
{
  let mut dir: *const c_char;                                          // C 2061
  let mut vp: *mut android_afs_vnode;                                  // C 2062

  /* If the vnode already exists and is a directory, return EROFS.
     Else, return ENOTDIR or ENOENT.  */                               // C 2064-2065

  vp = vnode as *mut android_afs_vnode;                                // C 2067
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2068

  if ! dir.is_null () && android_is_directory (dir)                    // C 2070
    { *errno () = EROFS; }                                             // C 2071
  else if ! dir.is_null ()                                             // C 2072
    { *errno () = ENOTDIR; }                                           // C 2073
  else
    { *errno () = ENOENT; }                                            // C 2075

  return -1;                                                           // C 2077
}

/* C 2080-2092.  Its address is taken at C 1691.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_rename (src: *mut android_vnode, dst: *mut android_vnode,
					 keep_existing: bool) -> c_int
{
  /* If src and dst are different kinds of vnodes, return EXDEV.
     Else, return EROFS.  */                                           // C 2084-2085

  *errno () = EROFS;                                                   // C 2087
  if (*src).r#type != (*dst).r#type                                    // C 2088
    { *errno () = EXDEV; }                                             // C 2089

  return -1;                                                           // C 2091
}

/* C 2094-2173.  Its address is taken at C 1692.  C's `goto set_file_times` (C 2129) and the
   `set_file_times:` label at C 2155 become one labelled block (PORTING.md 2.2).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_stat (vnode: *mut android_vnode, statb: *mut stat,
				       flags: c_int) -> c_int
{
  let mut dir: *const c_char;                                          // C 2098
  let mut vp: *mut android_afs_vnode;                                  // C 2099
  let mut asset_desc: *mut AAsset;                                     // C 2100

  /* C leaves `dir`, `vp` and `asset_desc` uninitialised (PORTING.md 1.5); `dir` and `vp` are
     assigned before any read, and `asset_desc` is read only on the path that assigns it.  */
  dir = std::ptr::null ();
  vp = std::ptr::null_mut ();
  asset_desc = std::ptr::null_mut ();

  /* Scan for the vnode to see whether or not it exists.  */            // C 2102

  vp = vnode as *mut android_afs_vnode;                                // C 2104
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2105

  'set_file_times: {
  if dir.is_null ()                                                    // C 2107
    {
      /* Return ENOENT; whether the lookup failed because directory
	 components within vp->path weren't really directories is not
	 important to Emacs's error reporting.  */                     // C 2109-2111
      *errno () = ENOENT;                                              // C 2112
      return -1;                                                       // C 2113
    }

  if android_is_directory (dir)                                        // C 2116
    {
      memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());  // C 2118

      /* Fill in the stat buffer.  */                                  // C 2120
      (*statb).st_mode = S_IFDIR | S_IRUSR | S_IRGRP | S_IROTH;        // C 2121

      /* Grant search permissions as well.  */                         // C 2123
      (*statb).st_mode |= S_IXUSR | S_IXGRP | S_IXOTH;                 // C 2124

      /* Concoct a nonexistent device and an inode number.  */         // C 2126
      (*statb).st_dev = -1;                                            // C 2127
      (*statb).st_ino = 0;                                             // C 2128
      break 'set_file_times;                                           // C 2129
    }

  /* AASSET_MODE_STREAMING is fastest here.  */                         // C 2132
  asset_desc = AAssetManager_open (asset_manager, (*vp).name.offset (1),   // C 2133
				   AASSET_MODE_STREAMING);             // C 2134

  if asset_desc.is_null ()                                             // C 2136
    {
      /* If the asset exists in the directory tree but can't be
	 located by the asset manager, report OOM.  */                 // C 2138-2139
      *errno () = ENOMEM;                                              // C 2140
      return 1;                                                        // C 2141
    }

  memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());      // C 2144

  /* Fill in the stat buffer.  */                                      // C 2146
  (*statb).st_mode = S_IFREG | S_IRUSR | S_IRGRP | S_IROTH;            // C 2147
  (*statb).st_dev = -1;                                                // C 2148
  (*statb).st_ino = 0;                                                 // C 2149
  (*statb).st_size = AAsset_getLength (asset_desc) as off_t;           // C 2150

  /* Close the asset.  */                                              // C 2152
  AAsset_close (asset_desc);                                           // C 2153
  }

  /* C 2162-2169's `#ifdef STAT_TIMESPEC` pair: the same two arms as `android_afs_open`'s, the
     live one reading the mirror's member and the `#else` one carried as a record (Deviation 22
     and the Owed item it names).  */

  /* If the installation date can be ascertained, return that as the
     file's modification time.  */                                    // C 2157-2158

  if timespec_valid_p (emacs_installation_time)                        // C 2160
    {
      #[cfg (feature = "stat_timespec")]
      { (*statb).st_mtimespec = emacs_installation_time; }             // C 2163 (`STAT_TIMESPEC
								       //  (statb, st_mtim)`)
      #[cfg (not (feature = "stat_timespec"))]
      {
	/* C 2167-2168:
	     statb->st_mtime = emacs_installation_time.tv_sec;
	     statb->st_mtime_nsec = emacs_installation_time.tv_nsec;  */
      }
    }

  return 0;                                                            // C 2172
}

/* C 2175-2219.  Its address is taken at C 1693.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut dir: *const c_char;                                          // C 2178
  let mut vp: *mut android_afs_vnode;                                  // C 2179

  /* Validate MODE.  */                                                // C 2181

  if mode != F_OK && (mode & (W_OK | X_OK | R_OK)) == 0                 // C 2183
    {
      *errno () = EINVAL;                                              // C 2185
      return -1;                                                       // C 2186
    }

  /* Scan for the vnode to see whether or not it exists.  */            // C 2188

  vp = vnode as *mut android_afs_vnode;                                // C 2191
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2192

  if ! dir.is_null ()                                                  // C 2194
    {
      /* It exists.  If MODE contains W_OK or X_OK, return
	 EACCESS.  */                                                  // C 2196-2197

      if (mode & (W_OK | X_OK)) != 0                                   // C 2199
	{
	  *errno () = EACCES;                                          // C 2201
	  return -1;                                                   // C 2202
	}

      /* If vp->name is a directory and DIR isn't, return ENOTDIR.  */   // C 2205

      if *(*vp).name.offset ((*vp).name_length as isize) == b'/' as c_char   // C 2207
	  && ! android_is_directory (dir)                              // C 2208
	{
	  *errno () = ENOTDIR;                                         // C 2210
	  return -1;                                                   // C 2211
	}

      return 0;                                                        // C 2214
    }

  *errno () = ENOENT;                                                  // C 2217
  return -1;                                                           // C 2218
}

/* C 2221-2238.  Its address is taken at C 1694.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  let mut vp: *mut android_afs_vnode;                                  // C 2224
  let mut dir: *const c_char;                                          // C 2225

  /* If the vnode already exists, return EEXIST in lieu of EROFS.  */   // C 2227

  vp = vnode as *mut android_afs_vnode;                                // C 2229
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2230

  if ! dir.is_null ()                                                  // C 2232
    { *errno () = EEXIST; }                                            // C 2233
  else
    { *errno () = EROFS; }                                             // C 2235

  return -1;                                                           // C 2237
}

/* C 2240-2246.  Its address is taken at C 1695; the body is C's two statements, so this is the
   one member whose whole text fits on the declaration.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_chmod (vnode: *mut android_vnode, mode: mode_t,
					flags: c_int) -> c_int
{
  *errno () = EROFS;                                                   // C 2244
  return -1;                                                           // C 2245
}

/* C 2248-2268.  Its address is taken at C 1696.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
					   size: size_t) -> ssize_t
{
  let mut vp: *mut android_afs_vnode;                                  // C 2252
  let mut dir: *const c_char;                                          // C 2253

  vp = vnode as *mut android_afs_vnode;                                // C 2255
  dir = android_scan_directory_tree ((*vp).name, std::ptr::null_mut ());   // C 2256

  /* As there are no symlinks in /assets, just return -1 with errno
     set to a reasonable value contingent upon whether VP->name
     actually exists.  */                                              // C 2258-2260

  if ! dir.is_null ()                                                  // C 2262
    { *errno () = EINVAL; }                                            // C 2263
  else
    { *errno () = ENOENT; }                                            // C 2265

  return -1;                                                           // C 2267
}

/* C 2270-2337.  C's `static struct dirent dirent;` is a function-scope `static mut` here, zero
   initialised as C initialises it.  C returns `struct dirent *`, while `struct android_vdir`'s
   `readdir` member is the mirror's untyped `*mut c_void` (`lib/include/dirent.rs` 6's reason), so
   the cast sits at the one assignment (`android_afs_opendir`, C 2421) rather than here.  Its
   address is taken there, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_readdir (vdir: *mut android_vdir) -> *mut dirent
{
  static mut dirent: dirent =
    dirent { d_ino: 0, d_off: 0, d_reclen: 0, d_type: 0, d_name: [0; 256] };   // C 2273
  let mut last: *const c_char;                                         // C 2274
  let mut dir: *mut android_afs_vdir;                                  // C 2275

  dir = vdir as *mut android_afs_vdir;                                 // C 2277

  /* There are no more files to read.  */                               // C 2279
  if (*dir).asset_dir >= (*dir).asset_limit                            // C 2280
    { return std::ptr::null_mut (); }                                  // C 2281

  /* Otherwise, scan forward looking for the next NULL byte.  */        // C 2283
  last = memchr ((*dir).asset_dir as *const c_void, 0,                 // C 2284
		  (*dir).asset_limit.offset_from ((*dir).asset_dir) as usize) as *const c_char;   // C 2285

  /* No more NULL bytes remain.  */                                     // C 2287
  if last.is_null ()                                                   // C 2288
    { return std::ptr::null_mut (); }                                  // C 2289

  /* Forward last past the NULL byte.  */                               // C 2291
  last = last.offset (1);                                              // C 2292

  /* Make sure it is still within the directory tree.  */               // C 2294
  if last >= (directory_tree as *const c_char).offset (directory_tree_size as isize)   // C 2295
    { return std::ptr::null_mut (); }                                  // C 2296

  /* Now, fill in the dirent with the name.  */                         // C 2298
  memset (&raw mut dirent as *mut c_void, 0, std::mem::size_of::<dirent> ());   // C 2299
  dirent.d_ino = 0;                                                    // C 2300
  dirent.d_off = 0;                                                    // C 2301
  dirent.d_reclen = std::mem::size_of::<dirent> () as c_ushort;        // C 2302

  /* Note that dir->asset_dir is actually a NULL terminated
     string.  */                                                       // C 2304-2305
  memcpy (dirent.d_name.as_mut_ptr () as *mut c_void,                  // C 2306
	  (*dir).asset_dir as *const c_void,                           // C 2306
	  MIN! (std::mem::size_of_val (&dirent.d_name),                // C 2307
		last.offset_from ((*dir).asset_dir) as usize));        // C 2308
  dirent.d_name[std::mem::size_of_val (&dirent.d_name) - 1] = 0;       // C 2309

  /* Strip off the trailing slash, if any.  */                          // C 2311
  if dirent.d_name[MIN! (std::mem::size_of_val (&dirent.d_name),       // C 2312-2314
			 last.offset_from ((*dir).asset_dir) as usize) - 2] == b'/' as c_char
    {
      dirent.d_name[MIN! (std::mem::size_of_val (&dirent.d_name),      // C 2315-2317
			  last.offset_from ((*dir).asset_dir) as usize) - 2] = 0;
    }

  /* If this is not a directory, return DT_REG.  Otherwise, return
     DT_DIR.  */                                                       // C 2319-2320

  if last.offset (-2) >= directory_tree as *const c_char               // C 2322
      && *last.offset (-2) == b'/' as c_char                           // C 2322
    { dirent.d_type = DT_DIR as c_uchar; }                             // C 2323
  else
    { dirent.d_type = DT_REG as c_uchar; }                             // C 2325

  /* Forward dir->asset_dir to the file past last.  */                  // C 2327
  #[cfg (not (feature = "old_android_assets"))]
  {                                                                    // C 2328
    (*dir).asset_dir = (directory_tree as *mut c_char)                 // C 2329
      .offset (android_extract_long (last) as isize);                  // C 2330
  }
  #[cfg (feature = "old_android_assets")]
  {                                                                    // C 2331
    (*dir).asset_dir = (directory_tree as *mut c_char)                 // C 2332
      .offset (android_extract_long (last.offset (4)) as isize);       // C 2333
  }

  return &raw mut dirent;                                              // C 2336
}

/* C 2339-2367.  Its address is taken at C 2422, so it is an `unsafe extern "C" fn`.  C's
   `for (next = &all_afs_vdirs; (tem = *next);)` is the load-then-test pair below (the assignment
   in the loop's condition is `loop { tem = *next; if tem.is_null () { break; } … }`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_afs_vdir;                                  // C 2342
  let mut next: *mut *mut android_afs_vdir;                            // C 2342
  let mut tem: *mut android_afs_vdir;                                  // C 2342

  dir = vdir as *mut android_afs_vdir;                                 // C 2344

  /* If the ``directory file descriptor'' has been opened, close
     it.  */                                                           // C 2346-2347

  if (*dir).fd != -1                                                   // C 2349
    { close ((*dir).fd); }                                             // C 2350

  xfree ((*dir).asset_file as *mut c_void);                            // C 2352

  /* Now unlink this directory.  */                                    // C 2354

  next = &raw mut all_afs_vdirs;                                       // C 2356
  loop
    {
      tem = *next;                                                     // C 2356
      if tem.is_null ()
	{ break; }
      if tem == dir                                                    // C 2358
	{ *next = (*dir).next; }                                       // C 2359
      else
	{ next = &raw mut (**next).next; }                             // C 2361
    }

  /* Free the directory itself.  */                                    // C 2364

  xfree (dir as *mut c_void);                                          // C 2366
}

/* C 2369-2388.  Its address is taken at C 2423.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_afs_vdir;                                  // C 2372

  dir = vdir as *mut android_afs_vdir;                                 // C 2374

  /* Since `android_afs_opendir' tries to avoid opening a file
     descriptor if readdir isn't called, dirfd can fail if open fails.

     open sets errno to a set of errors different from what POSIX
     stipulates for dirfd, but for ease of implementation the open
     errors are used instead.  */                                      // C 2376-2381

  if (*dir).fd >= 0                                                    // C 2383
    { return (*dir).fd; }                                              // C 2384

  (*dir).fd = open (c"/dev/null".as_ptr (), O_RDONLY | O_CLOEXEC);     // C 2386
  return (*dir).fd;                                                    // C 2387
}

/* C 2390-2447.  Its address is taken at C 1698, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_afs_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut asset_dir: *mut c_char;                                      // C 2393
  let mut dir: *mut android_afs_vdir;                                  // C 2394
  let mut vp: *mut android_afs_vnode;                                  // C 2395
  let mut limit: size_t;                                               // C 2396

  /* C leaves `asset_dir`, `dir`, `vp` and `limit` uninitialised (PORTING.md 1.5); each is
     assigned before its first read on every path that reads it.  */
  asset_dir = std::ptr::null_mut ();
  dir = std::ptr::null_mut ();
  vp = std::ptr::null_mut ();
  limit = 0;

  vp = vnode as *mut android_afs_vnode;                                // C 2398

  /* Scan for the asset directory by vp->name.  */                      // C 2400

  asset_dir
    = android_scan_directory_tree ((*vp).name, &raw mut limit) as *mut c_char;   // C 2402-2403

  if asset_dir.is_null ()                                              // C 2405
    {
      *errno () = ENOENT;                                              // C 2407
      return std::ptr::null_mut ();                                    // C 2408
    }

  /* Verify that asset_dir is indeed a directory.  */                   // C 2411

  if ! android_is_directory (asset_dir)                                // C 2413
    {
      *errno () = ENOTDIR;                                             // C 2415
      return std::ptr::null_mut ();                                    // C 2416
    }

  /* Fill in the directory stream.  */                                  // C 2419
  dir = xmalloc (std::mem::size_of::<android_afs_vdir> ()) as *mut android_afs_vdir;   // C 2420
  (*dir).vdir.readdir
    = Some (std::mem::transmute::<unsafe extern "C" fn (*mut android_vdir) -> *mut dirent,
				  unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>
	    (android_afs_readdir));                                    // C 2421
  (*dir).vdir.closedir = Some (android_afs_closedir);                  // C 2422
  (*dir).vdir.dirfd = Some (android_afs_dirfd);                        // C 2423
  (*dir).asset_dir = asset_dir;                                        // C 2424
  (*dir).asset_limit = (directory_tree as *mut c_char).offset (limit as isize);   // C 2425
  (*dir).fd = -1;                                                      // C 2426
  (*dir).asset_file = xzalloc ((*vp).name_length + 2) as *mut c_char;  // C 2427
  strcpy ((*dir).asset_file, (*vp).name);                              // C 2428

  /* Make sure dir->asset_file is terminated with /.  */                // C 2430
  if *(*dir).asset_file.offset (((*vp).name_length - 1) as isize) != b'/' as c_char   // C 2431
    { *(*dir).asset_file.offset ((*vp).name_length as isize) = b'/' as c_char; }   // C 2432

  /* Make sure dir->asset_limit is within bounds.  It is a limit,
     and as such can be exactly one byte past directory_tree.  */      // C 2434-2435
  if (*dir).asset_limit > (directory_tree as *mut c_char).offset (directory_tree_size as isize)   // C 2436
    {
      xfree ((*dir).asset_file as *mut c_void);                        // C 2438
      xfree (dir as *mut c_void);                                      // C 2439
      *errno () = EACCES;                                              // C 2440
      return std::ptr::null_mut ();                                    // C 2441
    }

  (*dir).next = all_afs_vdirs;                                         // C 2444
  all_afs_vdirs = dir;                                                 // C 2445
  return &raw mut (*dir).vdir;                                         // C 2446
}

/* Return the file name corresponding to DIRFD if it is a
   ``directory'' file descriptor returned by `android_afs_dirfd' or
   NULL otherwise.  These file names are relative to the `/assets'
   directory, but with a leading separator character.  */              // C 2449-2452

/* C 2454-2466.  C calls it directly (C 7354), never through a pointer, so it stays a Rust-ABI
   `unsafe fn` (the `android_unix_vnode` rule).  Its `for (dir = …; dir; dir = dir->next)` is the
   load-then-test pair below.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_afs_get_directory_name (dirfd: c_int) -> *mut c_char
{
  let mut dir: *mut android_afs_vdir;                                  // C 2457

  dir = all_afs_vdirs;                                                 // C 2459
  while ! dir.is_null ()
    {
      if (*dir).fd == dirfd && dirfd != -1                             // C 2461
	{ return (*dir).asset_file; }                                  // C 2462
      dir = (*dir).next;                                               // C 2459
    }

  return std::ptr::null_mut ();                                        // C 2465
}

/* C 1682-1699: `static struct android_vops afs_vfs_ops = { … }`, the asset family's operation
   vector.  C's initialiser is positional; Rust spells the same members in C's order with their
   names (the `unix_vfs_ops` shape).  **This object sits at the foot of the unit, out of C's
   order** (Deviation 21): its text names thirteen functions C defines after it (C 1708-2454),
   Rust has no forward declaration (PORTING.md 10.10, the P-91 class), and two of those functions
   name the object in return (C 1740 and C 1834) - so the object and the thirteen are one
   dependency clique that has to land in one round, with the object's text moved to the round's
   end (the haikuterm slice-22 out-of-order device, its Deviation 34).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut afs_vfs_ops: android_vops = android_vops
  {
    name: Some (android_afs_name),
    open: Some (android_afs_open),
    close: Some (android_afs_close),
    unlink: Some (android_afs_unlink),
    symlink: Some (android_afs_symlink),
    rmdir: Some (android_afs_rmdir),
    rename: Some (android_afs_rename),
    stat: Some (android_afs_stat),
    access: Some (android_afs_access),
    mkdir: Some (android_afs_mkdir),
    chmod: Some (android_afs_chmod),
    readlink: Some (android_afs_readlink),
    opendir: Some (android_afs_opendir),
  };

/* C 2470-2484: `struct android_content_vdir'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_content_vdir
{
  /* The directory function table.  */
  pub vdir: android_vdir,                                              // C 2473

  /* The next directory stream in `all_content_vdirs'.  */
  pub next: *mut android_content_vdir,                                 // C 2476

  /* Pointer to the next file to return.  */
  pub next_name: *mut *const c_char,                                   // C 2479

  /* Temporary file descriptor used to identify this directory to
     at-funcs, or -1.  */
  pub fd: c_int,                                                       // C 2483
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_content_vdir> () == 48);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_content_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_content_vdir, next) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_content_vdir, next_name) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_content_vdir, fd) == 40);

/* C 2486-2488 (`android_authority_initial`, `android_authority_initial_name`,
   `android_saf_root_initial`), C 2502-2518 (the twelve `android_content_*` functions) and C
   2490-2500's comment on the content meta-interface: C's forward declarations, for which Rust
   needs no text (PORTING.md 10.10).  The three functions C 2486-2488 names are the authority and
   SAF families' (C 3577/3603/4219).  */

/* C 2520-2537 - `content_vfs_ops` - is **deferred with `android_content_name`**
   (Deviation 23): the object names `android_content_name`, and `android_content_name`'s own
   function-scope `content_vnodes[]` names C 3577/3603/4219's three initial functions, so the
   object, that function and those three land together in the round that reaches C 3577.  */

/* Table of directories contained within a top-level vnode.  */         // C 2539

/* C 2541-2544: `static const char *content_directory_contents[] = { … }`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut content_directory_contents: [*const c_char; 3] =
  [ c"storage".as_ptr (), c"by-authority".as_ptr (), c"by-authority-named".as_ptr () ];   // C 2543

/* Chain consisting of all open content directory streams.  */          // C 2546
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut all_content_vdirs: *mut android_content_vdir = std::ptr::null_mut ();   // C 2547

/* C 2549-2639 - `android_content_name` - is **deferred** (see the note above C 2520): it is an
   `unsafe extern "C" fn`, because C takes its address at C 2524.  */

/* C 2641-2649.  Its address is taken at C 2525, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_open (vnode: *mut android_vnode, flags: c_int,
					   mode: mode_t, asset_p: bool, fd: *mut c_int,
					   asset: *mut *mut AAsset) -> c_int
{
  /* Don't allow opening this special directory.  */                    // C 2646
  *errno () = ENOSYS;                                                  // C 2647
  return -1;                                                           // C 2648
}

/* C 2651-2659.  Its address is taken at C 2526.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_close (vnode: *mut android_vnode)
{
  let mut save_errno: c_int;                                           // C 2654

  save_errno = *errno ();                                              // C 2656
  xfree (vnode as *mut c_void);                                        // C 2657
  *errno () = save_errno;                                              // C 2658
}

/* C 2661-2666.  Its address is taken at C 2527.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_unlink (vnode: *mut android_vnode) -> c_int
{
  *errno () = ENOSYS;                                                  // C 2664
  return -1;                                                           // C 2665
}

/* C 2668-2673.  Its address is taken at C 2528.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_symlink (target: *const c_char,
					     vnode: *mut android_vnode) -> c_int
{
  *errno () = ENOSYS;                                                  // C 2671
  return -1;                                                           // C 2672
}

/* C 2675-2680.  Its address is taken at C 2529.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_rmdir (vnode: *mut android_vnode) -> c_int
{
  *errno () = ENOSYS;                                                  // C 2678
  return -1;                                                           // C 2679
}

/* C 2682-2699.  Its address is taken at C 2530.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_rename (src: *mut android_vnode, dst: *mut android_vnode,
					     keep_existing: bool) -> c_int
{
  if (*src).r#type != (*dst).r#type                                    // C 2687
    {
      /* If the types of both vnodes differ, complain that they're on
	 two different filesystems (which is correct from a abstract
	 viewpoint.)  */                                               // C 2689-2691
      *errno () = EXDEV;                                               // C 2692
      return -1;                                                       // C 2693
    }

  /* Otherwise, return ENOSYS.  */                                     // C 2696
  *errno () = ENOSYS;                                                  // C 2697
  return -1;                                                           // C 2698
}

/* C 2701-2713.  Its address is taken at C 2531.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_stat (vnode: *mut android_vnode, statb: *mut stat,
					   flags: c_int) -> c_int
{
  memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());      // C 2705

  (*statb).st_uid = getuid ();                                         // C 2707
  (*statb).st_gid = getgid ();                                         // C 2708
  (*statb).st_ino = 0;                                                 // C 2709
  (*statb).st_dev = -2;                                                // C 2710
  (*statb).st_mode = S_IFDIR | S_IRUSR | S_IXUSR;                      // C 2711
  return 0;                                                            // C 2712
}

/* C 2715-2736.  Its address is taken at C 2532.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  /* Validate MODE.  */                                                // C 2718

  if mode != F_OK && (mode & (W_OK | X_OK | R_OK)) == 0                 // C 2720
    {
      *errno () = EINVAL;                                              // C 2722
      return -1;                                                       // C 2723
    }

  /* Return EROFS if the caller is trying to check for write access to
     this vnode.  */                                                   // C 2726-2727

  if mode != F_OK && (mode & (W_OK | X_OK)) != 0                        // C 2729
    {
      *errno () = EROFS;                                               // C 2731
      return -1;                                                       // C 2732
    }

  return 0;                                                            // C 2735
}

/* C 2738-2743.  Its address is taken at C 2533.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  *errno () = EEXIST;                                                  // C 2741
  return -1;                                                           // C 2742
}

/* C 2745-2751.  Its address is taken at C 2534.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_chmod (vnode: *mut android_vnode, mode: mode_t,
					    flags: c_int) -> c_int
{
  *errno () = EACCES;                                                  // C 2749
  return -1;                                                           // C 2750
}

/* C 2753-2759.  Its address is taken at C 2535.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
					       size: size_t) -> ssize_t
{
  *errno () = EINVAL;                                                  // C 2757
  return -1;                                                           // C 2758
}

/* C 2761-2786.  The same shape as `android_afs_readdir`'s static (`static struct dirent dirent;`)
   and the same `*mut c_void` cast at the one assignment (C 2843).  Its address is taken there, so
   it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_readdir (vdir: *mut android_vdir) -> *mut dirent
{
  static mut dirent: dirent =
    dirent { d_ino: 0, d_off: 0, d_reclen: 0, d_type: 0, d_name: [0; 256] };   // C 2764
  let mut dir: *mut android_content_vdir;                              // C 2765
  let mut name: *const c_char;                                         // C 2766

  dir = vdir as *mut android_content_vdir;                             // C 2768

  /* There are no more files to be read.  */                            // C 2770
  if (*dir).next_name == (&raw mut content_directory_contents as *mut *const c_char)
      .offset (countof! (content_directory_contents) as isize)         // C 2771-2772
    { return std::ptr::null_mut (); }                                  // C 2773

  /* Get the next child.  */                                           // C 2775
  name = *(*dir).next_name;                                            // C 2776
  (*dir).next_name = (*dir).next_name.offset (1);                      // C 2776

  /* Now, fill in the dirent with the name.  */                         // C 2778
  memset (&raw mut dirent as *mut c_void, 0, std::mem::size_of::<dirent> ());   // C 2779
  dirent.d_ino = 0;                                                    // C 2780
  dirent.d_off = 0;                                                    // C 2781
  dirent.d_reclen = std::mem::size_of::<dirent> () as c_ushort;        // C 2782
  dirent.d_type = DT_DIR as c_uchar;                                   // C 2783
  strcpy (dirent.d_name.as_mut_ptr (), name);                          // C 2784
  return &raw mut dirent;                                              // C 2785
}

/* C 2788-2812.  Its address is taken at C 2844; the `for (next = …; (tem = *next);)` loop is the
   load-then-test pair (`android_afs_closedir`'s shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_content_vdir;                              // C 2791
  let mut next: *mut *mut android_content_vdir;                        // C 2791
  let mut tem: *mut android_content_vdir;                              // C 2791

  dir = vdir as *mut android_content_vdir;                             // C 2793

  /* If the ``directory file descriptor'' has been opened, close
     it.  */                                                           // C 2795-2796

  if (*dir).fd != -1                                                   // C 2798
    { close ((*dir).fd); }                                             // C 2799

  /* Now unlink this directory.  */                                    // C 2801

  next = &raw mut all_content_vdirs;                                   // C 2803
  loop
    {
      tem = *next;                                                     // C 2803
      if tem.is_null ()
	{ break; }
      if tem == dir                                                    // C 2805
	{ *next = (*dir).next; }                                       // C 2806
      else
	{ next = &raw mut (**next).next; }                             // C 2808
    }

  xfree (dir as *mut c_void);                                          // C 2811
}

/* C 2814-2833.  Its address is taken at C 2845.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_content_vdir;                              // C 2817

  dir = vdir as *mut android_content_vdir;                             // C 2819

  /* Since `android_content_opendir' tries to avoid opening a file
     descriptor if readdir isn't called, dirfd can fail if open fails.

     open sets errno to a set of errors different from what POSIX
     stipulates for dirfd, but for ease of implementation the open
     errors are used instead.  */                                      // C 2821-2826

  if (*dir).fd >= 0                                                    // C 2828
    { return (*dir).fd; }                                              // C 2829

  (*dir).fd = open (c"/dev/null".as_ptr (), O_RDONLY | O_CLOEXEC);     // C 2831
  return (*dir).fd;                                                    // C 2832
}

/* C 2835-2867.  Its address is taken at C 2536, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_content_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut dir: *mut android_content_vdir;                              // C 2838
  let mut api: c_int;                                                  // C 2839

  /* C leaves `dir` and `api` uninitialised (PORTING.md 1.5); both are assigned before any read.  */
  dir = std::ptr::null_mut ();
  api = 0;

  /* Allocate the virtual directory.  */                               // C 2841
  dir = xmalloc (std::mem::size_of::<android_content_vdir> ()) as *mut android_content_vdir;   // C 2842
  (*dir).vdir.readdir
    = Some (std::mem::transmute::<unsafe extern "C" fn (*mut android_vdir) -> *mut dirent,
				  unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>
	    (android_content_readdir));                                // C 2843
  (*dir).vdir.closedir = Some (android_content_closedir);              // C 2844
  (*dir).vdir.dirfd = Some (android_content_dirfd);                    // C 2845
  (*dir).fd = -1;                                                      // C 2846

  /* Fill in the directory contents.  */                               // C 2848
  (*dir).next_name = &raw mut content_directory_contents as *mut *const c_char;   // C 2849
  api = android_get_current_api_level ();                              // C 2850

  /* Android 4.4 and earlier don't support /content/storage.  */        // C 2852

  if api < 21                                                          // C 2854
    { (*dir).next_name = (*dir).next_name.offset (1); }                // C 2855

  /* Android 4.3 and earlier don't support /content/by-authority.  */   // C 2857

  if api < 19                                                          // C 2859
    { (*dir).next_name = (*dir).next_name.offset (2); }                // C 2860

  /* Link this stream onto the list of all content directory
     streams.  */                                                      // C 2862-2863
  (*dir).next = all_content_vdirs;                                     // C 2864
  all_content_vdirs = dir;                                             // C 2865
  return &raw mut (*dir).vdir;                                         // C 2866
}

/* Return the file name corresponding to DIRFD if it is a
   ``directory'' file descriptor returned by `android_content_dirfd'
   or NULL otherwise.  */                                              // C 2869-2871

/* C 2873-2885.  C calls it directly (C 7354's sibling), never through a pointer, so it stays a
   Rust-ABI `unsafe fn`; its `for (dir = …; dir; dir = dir->next)` is the load-then-test pair.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_content_get_directory_name (dirfd: c_int) -> *mut c_char
{
  let mut dir: *mut android_content_vdir;                              // C 2876

  dir = all_content_vdirs;                                             // C 2878
  while ! dir.is_null ()
    {
      if (*dir).fd == dirfd && dirfd != -1                             // C 2880
	{ return c"/content".as_ptr () as *mut c_char; }               // C 2881
      dir = (*dir).next;                                               // C 2878
    }

  return std::ptr::null_mut ();                                        // C 2884
}

/* C 2887-2906 - `android_content_initial` - is **deferred** with the object and
   `android_content_name` (the note above C 2520); it is an `unsafe extern "C" fn`, because C takes
   its address at C 6634's sibling table.  */

/* C 2908-2916 and C 2942-2946: C 2909's form feed, the `#ifdef __clang__` / `#pragma clang
   diagnostic push` / `#pragma … ignored "-Wmissing-prototypes"` / `#else` / `#pragma GCC …` /
   `#endif` pair and its pop.  A pragma has no Rust counterpart and the port emits none either way
   (the android.rs 1650-1655 precedent).  */

/* Content URI management functions.  */                                // C 2918

/* C 2920-2940: `JNIEXPORT jstring JNICALL NATIVE_NAME (displayNameHash) (JNIEnv *env, jobject
   object, jbyteArray display_name)`.  `NATIVE_NAME` pastes the symbol name (android.h 344), so
   the port exports its own class's name, `Java_org_femacs_EmacsNative_displayNameHash`
   (PORTING.md 0.1-19's name half / status 3.503, the macro's row in
   `docs/ports/excluded-macros.txt`); `JNIEXPORT`/`JNICALL` have no Rust text.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_displayNameHash")]
pub unsafe extern "C" fn displayNameHash (env: *mut JNIEnv, object: jobject,
					  display_name: jbyteArray) -> jstring
{
  let mut checksum: [c_char; 9] = [0; 9];                              // C 2924
  let mut block: [c_char; MD5_DIGEST_SIZE as usize] = [0; MD5_DIGEST_SIZE as usize];   // C 2924
  let mut data: *mut jbyte;                                            // C 2925

  data = ((*(*env)).GetByteArrayElements.unwrap ()) (env, display_name,   // C 2927
						     std::ptr::null_mut ());   // C 2927
  if data.is_null ()                                                   // C 2928
    { return std::ptr::null_mut (); }                                  // C 2929

  /* Hash the buffer.  */                                              // C 2931
  md5_buffer (data as *const c_char,                                   // C 2932
	      ((*(*env)).GetArrayLength.unwrap ()) (env, display_name) as usize,   // C 2932
	      &raw mut block as *mut c_void);                          // C 2933
  ((*(*env)).ReleaseByteArrayElements.unwrap ()) (env, display_name, data, JNI_ABORT);   // C 2934

  /* Generate the digest string.  */                                   // C 2936
  hexbuf_digest (&raw mut checksum as *mut c_char,                     // C 2937
		 &raw mut block as *const c_void, 4);                  // C 2937
  checksum[8] = 0;                                                     // C 2938
  return ((*(*env)).NewStringUTF.unwrap ()) (env, &raw const checksum as *const c_char);   // C 2939
}

/* C 51's `lisp.h` 4247 declares `hexbuf_digest` and the `lisp.rs` mirror does not carry it
   (the android.rs 1386-1392 declaration, this unit's own copy under its gate).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: lisp.h 4247: `extern void hexbuf_digest (char *, void const *, int);`
  pub fn hexbuf_digest (buf: *mut c_char, data: *const c_void, size: c_int);
}

/* C 3048-3060's names: android.h 321/331/334's three globals, android.h 129/130's two exception
   checks and lisp.h 4432's `memory_full`.  None of them is in this unit's include graph - the
   `android.h` mirror `include/android.rs` carries all of them, but the edge that would put it
   here is Owed 2 (its `android_vdir` stand-in is deleted when the edge lands) - so each stands at
   its use site (P-72 rule 4, the `android_cache_dir` / `emacs_installation_time` class).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: android.h 321: `extern JNIEnv *android_java_env;`
  pub static mut android_java_env: *mut JNIEnv;
  /// C: android.h 331: `extern jobject emacs_service;`
  pub static mut emacs_service: jobject;
  /// C: android.h 334: `extern struct android_emacs_service service_class;`
  pub static mut service_class: android_emacs_service;
  /// C: android.h 129: `extern void android_exception_check (void);`
  pub fn android_exception_check ();
  /// C: android.h 130: `extern void android_exception_check_1 (jobject);`
  pub fn android_exception_check_1 (arg0: jobject);
  /// C: android.h 138: `extern void android_exception_check_nonnull (void *, jobject);`
  pub fn android_exception_check_nonnull (arg0: *mut c_void, arg1: jobject);
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
  pub fn memory_full (size: size_t) -> !;
}
/* C 275-319's `struct android_emacs_service`, whose text is `include/android.rs` 121-167's - the
   mirror that is not in this unit's graph (the note above).  Declared here at the use site for the
   same reason, and this declaration retires with the mirror edge (Owed 2).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_emacs_service
{
  pub class: jclass,                                                   // C: android.h 275
  pub fill_rectangle: jmethodID,
  pub fill_polygon: jmethodID,
  pub draw_rectangle: jmethodID,
  pub draw_line: jmethodID,
  pub draw_point: jmethodID,
  pub ring_bell: jmethodID,
  pub query_tree: jmethodID,
  pub get_screen_width: jmethodID,
  pub get_screen_height: jmethodID,
  pub detect_mouse: jmethodID,
  pub detect_keyboard: jmethodID,
  pub name_keysym: jmethodID,
  pub browse_url: jmethodID,
  pub restart_emacs: jmethodID,
  pub update_ic: jmethodID,
  pub reset_ic: jmethodID,
  pub open_content_uri: jmethodID,
  pub check_content_uri: jmethodID,
  pub query_battery: jmethodID,
  pub update_extracted_text: jmethodID,
  pub update_cursor_anchor_info: jmethodID,
  pub get_document_authorities: jmethodID,
  pub request_directory_access: jmethodID,
  pub get_document_trees: jmethodID,
  pub document_id_from_name: jmethodID,
  pub get_tree_uri: jmethodID,
  pub stat_document: jmethodID,
  pub access_document: jmethodID,
  pub open_document_directory: jmethodID,
  pub read_directory_entry: jmethodID,
  pub open_document: jmethodID,
  pub create_document: jmethodID,
  pub create_directory: jmethodID,
  pub delete_document: jmethodID,
  pub rename_document: jmethodID,
  pub move_document: jmethodID,
  pub valid_authority: jmethodID,
  pub external_storage_available: jmethodID,
  pub request_storage_access: jmethodID,
  pub cancel_notification: jmethodID,
  pub relinquish_uri_rights: jmethodID,
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`): 42 eight-byte words, i.e. 336 bytes, the last field at 328
   (`jclass` and `jmethodID` are both 8 bytes there).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_emacs_service> () == 336);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_emacs_service, class) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_emacs_service, check_content_uri) == 144);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_emacs_service, relinquish_uri_rights)
					    == 328);

/* C 2956-3034: `static char *android_get_content_name (const char *filename, bool
   with_checksum)`.  C's `goto no_checksum` (C 2985) and the `no_checksum:` label at C 3022 become
   one labelled block (PORTING.md 2.2).  C calls it directly (C 3314/3316), never through a
   pointer, so it stays a Rust-ABI `unsafe fn` (the `android_unix_vnode` rule).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_get_content_name (filename: *const c_char, with_checksum: bool) -> *mut c_char
{
  let mut fill: *mut c_char;                                           // C 2959
  let mut buffer: *mut c_char;                                         // C 2959
  let mut length: size_t;                                              // C 2960
  let mut checksum: [c_char; 9] = [0; 9];                              // C 2961
  let mut new_checksum: [c_char; 9] = [0; 9];                          // C 2961
  let mut block: [c_char; MD5_DIGEST_SIZE as usize] = [0; MD5_DIGEST_SIZE as usize];   // C 2961
  let mut p2: *const c_char;                                           // C 2962
  let mut p1: *const c_char;                                           // C 2962

  /* C leaves `fill`, `buffer`, `length`, `p2` and `p1` uninitialised (PORTING.md 1.5); each is
     assigned before its first read on every path that reads it.  */
  fill = std::ptr::null_mut ();
  buffer = std::ptr::null_mut ();
  length = 0;
  p2 = std::ptr::null ();
  p1 = std::ptr::null ();

  /* Make sure FILENAME isn't obviously invalid: it must contain an
     authority name and a file name component.  */                     // C 2964-2965

  fill = strchr (filename, b'/' as c_int);                             // C 2967
  if fill.is_null () || *(fill.offset (1)) == 0                        // C 2968
    {
      *errno () = ENOENT;                                              // C 2970
      return std::ptr::null_mut ();                                    // C 2971
    }

  /* FILENAME must also not be a directory.  Accessing content
     provider directories is not supported by this interface.  */      // C 2974-2975

  length = strlen (filename);                                          // C 2977
  if *filename.offset (length as isize) == b'/' as c_char              // C 2978
    {
      *errno () = ENOTDIR;                                             // C 2980
      return std::ptr::null_mut ();                                    // C 2981
    }

  'no_checksum: {
  if ! with_checksum                                                   // C 2984
    { break 'no_checksum; }                                            // C 2985

  /* Content file names hold two components providing a display name and
     a short checksum that protects against files being opened under
     display names besides those provided in the content file name at
     the time of generation.  */                                      // C 2987-2990

  p1 = strrchr (filename, b'/' as c_int);                              // C 2992
  p2 = memrchr (filename as *const c_void, b'/' as c_int,             // C 2993
		p1.offset_from (filename) as usize) as *const c_char;  // C 2993

  /* If the name be excessively short or the checksum of an invalid
     length, return.  */                                               // C 2995-2996
  if p2.is_null () || p1.offset_from (p2) != 9                         // C 2997
    {
      *errno () = ENOENT;                                              // C 2999
      return std::ptr::null_mut ();                                    // C 3000
    }

  /* Copy the checksum into CHECKSUM.  */                              // C 3003
  memcpy (&raw mut checksum as *mut c_void, p2.offset (1) as *const c_void, 8);   // C 3004
  checksum[8] = 0;                                                     // C 3005 (C's chained
  new_checksum[8] = 0;                                                 // C 3005  assignment, split)

  /* Hash this string and store 8 bytes of the resulting digest into
     new_checksum.  */                                                 // C 3007-3008
  md5_buffer (p1.offset (1), strlen (p1.offset (1)) as usize,          // C 3009
	      &raw mut block as *mut c_void);                          // C 3009
  hexbuf_digest (&raw mut new_checksum as *mut c_char,                 // C 3010
		 &raw mut block as *const c_void, 4);                  // C 3010

  /* Compare both checksums.  */                                       // C 3012
  if strcmp (&raw const new_checksum as *const c_char,                 // C 3013
	     &raw const checksum as *const c_char) != 0               // C 3013
    {
      *errno () = ENOENT;                                              // C 3015
      return std::ptr::null_mut ();                                    // C 3016
    }

  /* Remove the checksum and file display name from the URI.  */       // C 3019
  length = p2.offset_from (filename) as size_t;                        // C 3020
  }

  /* no_checksum: */
  if length > INT_MAX as usize                                          // C 3023
    {
      *errno () = ENOMEM;                                              // C 3025
      return std::ptr::null_mut ();                                    // C 3026
    }

  /* Prefix FILENAME with content:// and return the buffer containing
     that URI.  */                                                     // C 3029-3030
  buffer = xmalloc (std::mem::size_of::<[c_char; 11]> () + length + 1) as *mut c_char;   // C 3031
  sprintf (buffer, c"content://%.*s".as_ptr (), length as c_int, filename);   // C 3032
  return buffer;                                                       // C 3033
}

/* Return whether or not the specified URI is an accessible content
   URI.  MODE specifies what to check.

   URI must be a string in the JVM's extended UTF-8 format.  */         // C 3036-3039

/* C 3041-3066.  C calls it directly (C 3525), never through a pointer, so it stays a Rust-ABI
   `unsafe fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_check_content_access (uri: *const c_char, mode: c_int) -> bool
{
  let mut string: jobject;                                             // C 3044
  let mut rc: jboolean;                                                // C 3045
  let mut read: jboolean;                                              // C 3045
  let mut write: jboolean;                                             // C 3045
  let mut method: jmethodID;                                           // C 3046

  /* C leaves `string`, `rc`, `read`, `write` and `method` uninitialised (PORTING.md 1.5); each is
     assigned before its first read.  */
  string = std::ptr::null_mut ();
  rc = 0;
  read = 0;
  write = 0;
  method = std::ptr::null_mut ();

  string = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, uri);   // C 3048
  android_exception_check ();                                          // C 3049

  /* Establish what is being checked.  Checking for read access is
     identical to checking if the file exists.  */                     // C 3051-3052

  read = ((mode & R_OK) != 0 || mode == F_OK) as jboolean;             // C 3054
  write = ((mode & W_OK) != 0) as jboolean;                            // C 3055
  method = service_class.check_content_uri;                            // C 3056

  rc = ((*(*android_java_env)).CallNonvirtualBooleanMethod.unwrap ())   // C 3058
    (android_java_env,                                                 // C 3058
     emacs_service,                                                    // C 3059
     service_class.class,                                              // C 3060
     method, string, (read as c_int),                                  // C 3061 (C's own
									//  variadic promotion of
									//  `jboolean` to `int`)
     (write as c_int));                                                // C 3062
  android_exception_check_1 (string);                                  // C 3063
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, string);   // C 3064
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  return rc != 0;                                                      // C 3065
}

/* C 3068-3070: C 3069's form feed and the comment `/* Functions shared by authority and SAF
   nodes.  */`.  */

/* Check for JNI exceptions, clear them, and set errno accordingly.
   Also, free each of the N local references given as arguments if an
   exception takes place.

   Value is 1 if an exception has taken place, 0 otherwise.

   If the exception thrown derives from FileNotFoundException, set
   errno to ENOENT.

   If the exception thrown derives from SecurityException, set errno
   to EACCES.

   If the exception thrown derives from OperationCanceledException,
   set errno to EINTR.

   If the exception thrown derives from UnsupportedOperationException,
   set errno to ENOSYS.

   If the exception thrown derives from OutOfMemoryException, call
   `memory_full'.

   If the exception thrown is anything else, set errno to EIO.  */     // C 3072-3093

/* C 3095-3164.  C's `va_list ap; va_start (ap, n); … va_end (ap);` has no port counterpart: the
   `...` parameter *is* the list and the one `va_arg (ap, jobject)` read is `ap.next_arg::<jobject>
   ()` (P-04, the `lib/fcntl.rs` 142 precedent, whose `jobject` row is in
   `docs/ports/va-arg-types.tsv`).  C calls it directly (C 3377 onward), never through a pointer -
   but a variadic *definition* is `extern "C"` in Rust (measured: `unsafe fn … (…, ...)` is
   `\`...\` is not supported for non-extern functions`), which is also C's own ABI here.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_exception_check (mut n: c_int, mut ap: ...) -> c_int
{
  let mut exception: jthrowable;                                       // C 3098
  let mut env: *mut JNIEnv;                                            // C 3099
  let mut new_errno: c_int;                                            // C 3101

  env = android_java_env;                                              // C 3103

  /* First, check for an exception.  */                                // C 3106

  if ((*(*env)).ExceptionCheck.unwrap ()) (env) == 0                   // C 3108
    {
      /* No exception has taken place.  Return 0.  */                  // C 3110
      return 0;                                                        // C 3112
    }

  /* Print the exception.  */                                          // C 3115
  ((*(*env)).ExceptionDescribe.unwrap ()) (env);                       // C 3116

  exception = ((*(*env)).ExceptionOccurred.unwrap ()) (env);           // C 3118

  if exception.is_null ()                                              // C 3120
    /* JNI couldn't return a local reference to the exception.  */
    { memory_full (0); }                                               // C 3122

  /* Clear the exception, making it safe to subsequently call other
     JNI functions.  */                                                // C 3124-3125
  ((*(*env)).ExceptionClear.unwrap ()) (env);                          // C 3126

  /* Delete each of the N arguments.  */                               // C 3128

  while n > 0
    {
      let reference: jobject = ap.next_arg::<jobject> ();              // C 3132
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, reference);   // C 3132
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
      n -= 1;                                                          // C 3133
    }

  /* Now set errno or signal memory_full as required.  */               // C 3136

  if ((*(*env)).IsInstanceOf.unwrap ()) (env, exception as jobject,    // C 3138
					 file_not_found_exception) != 0   // C 3139
    { new_errno = ENOENT; }                                            // C 3140
  else if ((*(*env)).IsInstanceOf.unwrap ()) (env, exception as jobject,   // C 3141
					      security_exception) != 0   // C 3142
    { new_errno = EACCES; }                                            // C 3143
  else if ((*(*env)).IsInstanceOf.unwrap ()) (env, exception as jobject,   // C 3144
					      operation_canceled_exception) != 0   // C 3145
    { new_errno = EINTR; }                                             // C 3146
  else if ((*(*env)).IsInstanceOf.unwrap ()) (env, exception as jobject,   // C 3147
					      unsupported_operation_exception) != 0   // C 3148
    { new_errno = ENOSYS; }                                            // C 3149
  else if ((*(*env)).IsInstanceOf.unwrap ()) (env, exception as jobject,   // C 3150
					      out_of_memory_error) != 0   // C 3151
    {
      ((*(*env)).DeleteLocalRef.unwrap ()) (env, exception as jobject);   // C 3153
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
      memory_full (0);                                                 // C 3154
    }
  else
    { new_errno = EIO; }                                               // C 3157

  /* expression is still a local reference! */
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, exception as jobject);    // C 3160
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  *errno () = new_errno;                                               // C 3161
  return 1;                                                            // C 3163
}

/* Verify that OBJECT is non-NULL.  If NULL, free each of the N local
   references given as arguments, and clear exceptions.

   Value is 1 if it be NULL, 0 otherwise.  */                          // C 3166-3169

/* C 3171-3195.  The same variadic shape as the checker above; C calls it directly (C 4311
   onward), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_check_nonnull (object: *const c_void, mut n: c_int,
						  mut ap: ...) -> c_int
{
  if ! object.is_null ()                                               // C 3176
    { return 0; }                                                      // C 3177

  /* Clear the active exception, making it safe to subsequently call
     other JNI functions.  */                                          // C 3181-3182
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 3183

  /* Delete each of the N arguments.  */                               // C 3185

  while n > 0
    {
      let reference: jobject = ap.next_arg::<jobject> ();              // C 3189
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, reference);   // C 3189
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
      n -= 1;                                                          // C 3190
    }

  return 1;                                                            // C 3194
}

/* Content authority-based vnode implementation.

   /content/by-authority is a simple vnode implementation that converts
   components to content:// URIs.

   It does not canonicalize file names by removing parent directory
   separators, as these characters can appear in legitimate content
   file names.  */                                                      // C 3199-3206

/* C 3208-3216: `struct android_authority_vnode'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_authority_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,                                            // C 3211

  /* URI associated with this vnode, or NULL if this is the root of
     the content authority tree.  */
  pub uri: *mut c_char,                                                // C 3215
}
/* Measured with the NDK's clang (the `android_afs_vnode` shape: 16 + 0 + 8).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_authority_vnode> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_authority_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_authority_vnode, uri) == 16);

/* C 3218-3234: C's twelve forward declarations of this file's own `static` functions
   (`android_authority_name` … `android_authority_opendir`); Rust needs no text for them
   (PORTING.md 10.10).  */

/* C 3236's comment `/* Vector of VFS operations associated with the content VFS node.  */`.  */

/* C 3238-3253 - `authority_vfs_ops` - sits at this file's foot (Deviation 26): its text names the
   twelve functions C defines at C 3256-3561, and Rust has no forward declaration (PORTING.md
   10.10, the P-91 class `afs_vfs_ops`'s Deviation 21 records).  */

/* C 3255-3335.  C's `goto no_entry` (C 3308) and `error:` (C 3333) become one labelled block with
   the `error:` path written out after it (PORTING.md 2.2, the `android_unix_name` family).  Its
   address is taken at C 3240, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_name (vnode: *mut android_vnode, mut name: *mut c_char,
					     mut length: size_t) -> *mut android_vnode
{
  let mut vp: *mut android_authority_vnode;                            // C 3259
  let mut uri_name: *mut c_char;                                       // C 3260

  /* C leaves `vp` and `uri_name` uninitialised (PORTING.md 1.5); `vp` is assigned before any read
     and `uri_name` only on the path that reads it.  */
  vp = std::ptr::null_mut ();
  uri_name = std::ptr::null_mut ();

  if ! android_init_gui                                              // C 3262
    {
      *errno () = EIO;                                                 // C 3264
      return std::ptr::null_mut ();                                    // C 3265
    }

  /* If NAME is empty or consists of a single directory separator
     _and_ VP->uri is NULL, return a copy of VNODE.  */                // C 3268-3269

  vp = vnode as *mut android_authority_vnode;                          // C 3271

  'no_entry: {
    if length < 1
	|| (*name == b'/' as c_char && length == 1 && (*vp).uri.is_null ())   // C 3273
      {
	vp = xmalloc (std::mem::size_of::<android_authority_vnode> ()) as *mut android_authority_vnode;   // C 3275
	memcpy (vp as *mut c_void, vnode as *const c_void,             // C 3276
		std::mem::size_of::<android_authority_vnode> ());      // C 3276

	if ! (*vp).uri.is_null ()                                      // C 3278
	  { (*vp).uri = xstrdup ((*vp).uri); }                         // C 3279

	return &raw mut (*vp).vnode;                                   // C 3281
      }

    /* Else, if VP->uri is NULL, then it is the root of the by-authority
       tree.  If NAME starts with a directory separator character,
       remove it.  */                                                  // C 3284-3286

    if (*vp).uri.is_null ()                                            // C 3288
      {
	if *name == b'/' as c_char                                     // C 3290
	  { name = name.offset (1); length -= 1; }                     // C 3291

	/* If the provided URI is a directory, return NULL and set errno
	   to ENOTDIR.  Content files are never directories.  */       // C 3293-3294

	if *name.offset ((length - 1) as isize) == b'/' as c_char      // C 3296
	  {
	    *errno () = ENOTDIR;                                       // C 3298
	    return std::ptr::null_mut ();                              // C 3299
	  }

	/* If the URI is not a valid JNI string, return immediately.  This
	   should not be possible, since /content file names are encoded
	   into JNI strings at the naming stage; the check is performed
	   only out of an abundance of caution.  */                    // C 3302-3305

	if android_verify_jni_string (name) != 0                       // C 3307
	  { break 'no_entry; }                                         // C 3308 (goto no_entry)

	if (*vp).vnode.r#type == ANDROID_VNODE_CONTENT_AUTHORITY_NAMED as c_short   // C 3310
	  /* This indicates that the two trailing components of NAME
	     provide a checksum and a file display name, to be verified,
	     then excluded from the content URI.  */
	  { uri_name = android_get_content_name (name, true); }        // C 3314
	else
	  { uri_name = android_get_content_name (name, false); }       // C 3316

	if uri_name.is_null ()                                         // C 3318
	  { return std::ptr::null_mut (); }                            // C 3319 (goto error)

	/* Now fill in the vnode.  */                                   // C 3321
	vp = xmalloc (std::mem::size_of::<android_authority_vnode> ()) as *mut android_authority_vnode;   // C 3322
	(*vp).vnode.ops = &raw mut authority_vfs_ops;                  // C 3323
	(*vp).vnode.r#type = ANDROID_VNODE_CONTENT_AUTHORITY as c_short;   // C 3324
	(*vp).vnode.flags = 0;                                         // C 3325
	(*vp).uri = uri_name;                                          // C 3326
	return &raw mut (*vp).vnode;                                   // C 3327
      }
  }

  /* Content files can't have children.  */
  /* no_entry: */
  *errno () = ENOENT;                                                  // C 3332
  /* error: */
  return std::ptr::null_mut ();                                        // C 3334
}

/* C 3337-3395.  Its address is taken at C 3241.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_open (vnode: *mut android_vnode, flags: c_int,
					     mode: mode_t, asset_p: bool, fd_return: *mut c_int,
					     asset: *mut *mut AAsset) -> c_int
{
  let mut vp: *mut android_authority_vnode;                            // C 3342
  let mut string: jobject;                                             // C 3343
  let mut fd: c_int;                                                   // C 3344
  let mut env: *mut JNIEnv;                                            // C 3345

  /* C leaves all four uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  vp = std::ptr::null_mut ();
  string = std::ptr::null_mut ();
  fd = 0;
  env = std::ptr::null_mut ();

  vp = vnode as *mut android_authority_vnode;                          // C 3347

  if (*vp).uri.is_null ()                                              // C 3349
    {
      /* This is the `by-authority' directory itself, which can't be
	 opened.  */                                                   // C 3351-3352
      *errno () = ENOSYS;                                              // C 3353
      return -1;                                                       // C 3354
    }

  /* Save the JNI environment within `env', to make wrapping
     subsequent lines referencing CallNonvirtualIntMethod
     feasible.  */                                                     // C 3357-3359
  env = android_java_env;                                              // C 3360

  /* Allocate a JNI string to hold VP->uri.  */                         // C 3362
  string = ((*(*env)).NewStringUTF.unwrap ()) (env, (*vp).uri);        // C 3363
  android_exception_check ();                                          // C 3364

  /* Try to open the file descriptor.  */                              // C 3366
  fd = ((*(*env)).CallNonvirtualIntMethod.unwrap ()) (env, emacs_service,   // C 3367
						      service_class.class,      // C 3368
						      service_class.open_content_uri,   // C 3369
						      string,               // C 3370
						      (((mode as c_int) & O_WRONLY) != 0
						       || ((mode as c_int) & O_RDWR) != 0) as c_int,   // C 3371-3373
						      (!(((mode as c_int) & O_WRONLY) != 0)) as c_int,   // C 3374
						      ((((mode as c_int) & O_TRUNC) != 0)) as c_int);   // C 3375-3376
  if android_saf_exception_check (1, string) != 0                      // C 3377
    { return -1; }                                                     // C 3378
  ((*(*env)).DeleteLocalRef.unwrap ()) (env, string);                  // C 3379 (C's
									//  ANDROID_DELETE_LOCAL_REF)
  /* If fd is -1, just assume that the file does not exist,
     and return -1 with errno set to ENOENT.  */                       // C 3381-3382

  if fd == -1                                                          // C 3384
    {
      *errno () = ENOENT;                                              // C 3386
      return -1;                                                       // C 3387
    }

  if ((mode as c_int) & O_CLOEXEC) != 0                                // C 3390 (C promotes
									//  the `mode_t`)
    { android_close_on_exec (fd); }                                    // C 3391

  *fd_return = fd;                                                     // C 3393
  return 0;                                                            // C 3394
}

/* C 3397-3408.  Its address is taken at C 3242.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_authority_vnode;                            // C 3400
  let mut save_errno: c_int;                                           // C 3401

  vp = vnode as *mut android_authority_vnode;                          // C 3403
  save_errno = *errno ();                                              // C 3404
  xfree ((*vp).uri as *mut c_void);                                    // C 3405
  xfree (vp as *mut c_void);                                           // C 3406
  *errno () = save_errno;                                              // C 3407
}

/* C 3410-3415.  Its address is taken at C 3243.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_unlink (vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3413
  return -1;                                                           // C 3414
}

/* C 3417-3423.  Its address is taken at C 3244.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_symlink (target: *const c_char,
						vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3421
  return -1;                                                           // C 3422
}

/* C 3425-3430.  Its address is taken at C 3245.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_rmdir (vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3428
  return -1;                                                           // C 3429
}

/* C 3432-3449.  Its address is taken at C 3246.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_rename (src: *mut android_vnode, dst: *mut android_vnode,
					       keep_existing: bool) -> c_int
{
  if (*src).r#type != (*dst).r#type                                    // C 3437
    {
      /* If the types of both vnodes differ, complain that they're on
	 two different filesystems (which is correct from a abstract
	 viewpoint.)  */                                               // C 3439-3441
      *errno () = EXDEV;                                               // C 3442
      return -1;                                                       // C 3443
    }

  /* Otherwise, return ENOSYS.  */                                     // C 3446
  *errno () = ENOSYS;                                                  // C 3447
  return -1;                                                           // C 3448
}

/* C 3451-3494.  Its address is taken at C 3247.  C 3475 calls through the vnode's own operation
   vector (the `android_afs_name` 1819 call's second site in this file).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_stat (vnode: *mut android_vnode, statb: *mut stat,
					     flags: c_int) -> c_int
{
  let mut rc: c_int;                                                   // C 3455
  let mut fd: c_int;                                                   // C 3455
  let mut save_errno: c_int;                                           // C 3455
  let mut vp: *mut android_authority_vnode;                            // C 3456

  /* C leaves `rc`, `fd`, `save_errno` and `vp` uninitialised (PORTING.md 1.5); `vp` is assigned
     before any read and the other three only on the path that first assigns them.  */
  rc = 0;
  fd = 0;
  save_errno = 0;
  vp = std::ptr::null_mut ();

  /* If this is a vnode representing `by-authority', return some
     information about this directory.  */                             // C 3458-3459

  vp = vnode as *mut android_authority_vnode;                          // C 3461

  if (*vp).uri.is_null ()                                              // C 3463
    {
      memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());  // C 3465
      (*statb).st_uid = getuid ();                                     // C 3466
      (*statb).st_gid = getgid ();                                     // C 3467
      (*statb).st_ino = 0;                                             // C 3468
      (*statb).st_dev = -3;                                            // C 3469
      (*statb).st_mode = S_IFDIR | S_IRUSR;                            // C 3470
      return 0;                                                        // C 3471
    }

  /* Try to open the file and call fstat.  */                          // C 3474
  rc = ((*(*vnode).ops).open).unwrap () (vnode, O_RDONLY, 0, false,    // C 3475
					 &raw mut fd, std::ptr::null_mut ());   // C 3475

  if rc < 0                                                            // C 3477
    { return -1; }                                                     // C 3478

  /* If rc is 1, then an asset file descriptor has been returned.
     This is impossible, so assert that it doesn't transpire.  */      // C 3480-3481
  assert! (rc != 1);                                                   // C 3482

  /* Now, try to stat the file.  */                                    // C 3484
  rc = fstat (fd, statb);                                              // C 3485
  save_errno = *errno ();                                              // C 3486

  /* Close the file descriptor.  */                                    // C 3488
  close (fd);                                                          // C 3489

  /* Restore errno.  */                                                // C 3491
  *errno () = save_errno;                                              // C 3492
  return rc;                                                           // C 3493
}

/* C 3496-3527.  Its address is taken at C 3248.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut vp: *mut android_authority_vnode;                            // C 3499

  vp = vnode as *mut android_authority_vnode;                          // C 3501

  /* Validate MODE.  */                                                // C 3503

  if mode != F_OK && (mode & (W_OK | X_OK | R_OK)) == 0                 // C 3505
    {
      *errno () = EINVAL;                                              // C 3507
      return -1;                                                       // C 3508
    }

  if (*vp).uri.is_null ()                                              // C 3511
    {
      /* Return EACCES if the caller is trying to check for write
	 access to `by-authority'.  */                                 // C 3513-3514

      if mode != F_OK && (mode & (W_OK | X_OK)) != 0                   // C 3516
	{
	  *errno () = EACCES;                                          // C 3518
	  return -1;                                                   // C 3519
	}

      return 0;                                                        // C 3522
    }

  return if android_check_content_access ((*vp).uri, mode)             // C 3525
    { 0 } else { -1 };                                                 // C 3526
}

/* C 3529-3534.  Its address is taken at C 3249.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  *errno () = EACCES;                                                  // C 3532
  return -1;                                                           // C 3533
}

/* C 3536-3542.  Its address is taken at C 3250.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_chmod (vnode: *mut android_vnode, mode: mode_t,
					      flags: c_int) -> c_int
{
  *errno () = EACCES;                                                  // C 3540
  return -1;                                                           // C 3541
}

/* C 3544-3550.  Its address is taken at C 3251.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
						 size: size_t) -> ssize_t
{
  *errno () = EINVAL;                                                  // C 3548
  return -1;                                                           // C 3549
}

/* C 3552-3561.  Its address is taken at C 3252.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut vp: *mut android_authority_vnode;                            // C 3555

  /* Forbid listing the `by-authority' directory.  */                   // C 3557
  vp = vnode as *mut android_authority_vnode;                          // C 3558
  *errno () = if ! (*vp).uri.is_null () { ENOTDIR } else { EACCES };   // C 3559
  return std::ptr::null_mut ();                                        // C 3560
}

/* Find the vnode designated by NAME relative to the root of the
   by-authority directory.

   If NAME is empty or a single leading separator character, return
   a vnode representing the by-authority directory itself.

   Otherwise, represent the remainder of NAME as a URI (without
   normalizing it) and return a vnode corresponding to that.

   Value may also be NULL with errno set if the designated vnode is
   not available, such as when Android windowing has not been
   initialized.  */                                                    // C 3563-3574

/* C 3576-3587.  C takes its address at C 6634's sibling table, so it is an `unsafe extern "C"
   fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_initial (name: *mut c_char, length: size_t)
						-> *mut android_vnode
{
  let mut temp: android_authority_vnode;                               // C 3579

  /* C leaves `temp` uninitialised (PORTING.md 1.5).  */
  temp = unsafe { std::mem::zeroed () };

  temp.vnode.ops = &raw mut authority_vfs_ops;                         // C 3581
  temp.vnode.r#type = ANDROID_VNODE_CONTENT_AUTHORITY as c_short;      // C 3582
  temp.vnode.flags = 0;                                                // C 3583
  temp.uri = std::ptr::null_mut ();                                    // C 3584

  return android_authority_name (&raw mut temp.vnode, name, length);   // C 3586
}

/* Find the vnode designated by NAME relative to the root of the
   by-authority-named directory.

   If NAME is empty or a single leading separator character, return
   a vnode representing the by-authority directory itself.

   Otherwise, represent the remainder of NAME as a URI (without
   normalizing it) and return a vnode corresponding to that.

   Value may also be NULL with errno set if the designated vnode is
   not available, such as when Android windowing has not been
   initialized.  */                                                    // C 3589-3600

/* C 3602-3613.  The `_name` twin of the function above.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_authority_initial_name (name: *mut c_char, length: size_t)
						     -> *mut android_vnode
{
  let mut temp: android_authority_vnode;                               // C 3605

  /* C leaves `temp` uninitialised (PORTING.md 1.5).  */
  temp = unsafe { std::mem::zeroed () };

  temp.vnode.ops = &raw mut authority_vfs_ops;                         // C 3607
  temp.vnode.r#type = ANDROID_VNODE_CONTENT_AUTHORITY_NAMED as c_short;   // C 3608
  temp.vnode.flags = 0;                                                // C 3609
  temp.uri = std::ptr::null_mut ();                                    // C 3610

  return android_authority_name (&raw mut temp.vnode, name, length);   // C 3612
}

/* C 3238-3253: `static struct android_vops authority_vfs_ops = { … }`.  **Out of C's order, at
   this file's foot** (Deviation 26): the object names the twelve `android_authority_*` functions
   C defines at C 3256-3561, Rust has no forward declaration (PORTING.md 10.10, the P-91 class),
   so the object's text moves to the end of the round that lands the last of them - the
   `afs_vfs_ops` device (Deviation 21).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut authority_vfs_ops: android_vops = android_vops
  {
    name: Some (android_authority_name),
    open: Some (android_authority_open),
    close: Some (android_authority_close),
    unlink: Some (android_authority_unlink),
    symlink: Some (android_authority_symlink),
    rmdir: Some (android_authority_rmdir),
    rename: Some (android_authority_rename),
    stat: Some (android_authority_stat),
    access: Some (android_authority_access),
    mkdir: Some (android_authority_mkdir),
    chmod: Some (android_authority_chmod),
    readlink: Some (android_authority_readlink),
    opendir: Some (android_authority_opendir),
  };

/* C 3262/3307's two remaining names for this round: android.h 45's `android_init_gui` and
   android.h 126's `android_verify_jni_string`, declared at the use site for the same reason the
   slice-10 block records (the `android.h` mirror is not in this unit's include graph).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: android.h 45: `extern bool android_init_gui;`
  pub static mut android_init_gui: bool;
  /// C: android.h 126: `extern int android_verify_jni_string (const char *);`
  pub fn android_verify_jni_string (arg0: *const c_char) -> c_int;
}

/* SAF ``root'' vnode implementation.

   The Storage Access Framework is a system service that manages a
   registry of document providers, which are a type of file system
   server.

   Normally, document providers can only provide individual files
   through preestablished ``content URIs''.  Android 5.0 and later add
   to that ``tree URIs'', which designate entire file system trees.

   Authorization to access document trees and content URIs is granted
   transiently by default when an Intent is received, but Emacs can
   also receive persistent authorization for a given document tree.

   /content/storage returns a list of directories, each representing a
   single authority providing at least one tree URI Emacs holds
   persistent authorization for.

   Each one of those directories then contains one document tree that
   Emacs is authorized to access.  */                                   // C 3617-3636

/* C 3638-3646: `struct android_saf_root_vnode'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_saf_root_vnode
{
  /* The vnode data.  */
  pub vnode: android_vnode,                                            // C 3641

  /* The name of the document authority this directory represents, or
     NULL.  */
  pub authority: *mut c_char,                                          // C 3645
}
/* Measured with the NDK's clang (the `android_authority_vnode` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_saf_root_vnode> () == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vnode, authority) == 16);

/* C 3648-3668: `struct android_saf_root_vdir'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_saf_root_vdir
{
  /* The directory stream function table.  */
  pub vdir: android_vdir,                                              // C 3651

  /* The next directory stream in `all_saf_root_vdirs'.  */
  pub next: *mut android_saf_root_vdir,                                // C 3654

  /* Array of strings, one for each directory to return.  */
  pub array: jobjectArray,                                             // C 3657

  /* Name of the authority this directory lists, or NULL.  */
  pub authority: *mut c_char,                                          // C 3660

  /* Length of that array, and the current within it.  */
  pub length: jsize,                                                   // C 3663
  pub i: jsize,                                                        // C 3663

  /* ``Directory'' file descriptor associated with this stream, or
     -1.  */
  pub fd: c_int,                                                       // C 3667
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_saf_root_vdir> () == 64);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, next) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, array) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, authority) == 40);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, length) == 48);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, i) == 52);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_root_vdir, fd) == 56);

/* C 3670-3686: C's twelve forward declarations of this file's own `static` functions
   (`android_saf_root_name` … `android_saf_root_opendir`); no Rust text (PORTING.md 10.10).  */

/* C 3688-3705 - `saf_root_vfs_ops` - is **deferred** (see the six-item note above): it names
   `android_saf_root_name` (C 3748), which the round that reaches C 5773 lands.  */

/* Chain containing all SAF root directories.  */                       // C 3707
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut all_saf_root_vdirs: *mut android_saf_root_vdir = std::ptr::null_mut ();   // C 3708

/* C 3710-3712: C's `static struct android_vnode *android_saf_tree_from_name (char *, const char *,
   const char *);` under the comment `/* Defined in the next page.  */` - a forward declaration for
   a function C defines at C 5773, which is one of the six deferred items this lane still owes.
   Rust needs no text for it here (PORTING.md 10.10).  */

/* Ascertain and return whether or not AUTHORITY designates a content
   provider offering at least one directory tree accessible to
   Emacs.  */                                                           // C 3714-3716

/* C 3718-3746.  C calls it directly (C 3920/3966), never through a pointer, so it stays a Rust-ABI
   `unsafe fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_valid_authority_p (authority: *const c_char) -> bool
{
  let mut string: jobject;                                             // C 3721
  let mut valid: jboolean;                                             // C 3722
  let mut method: jmethodID;                                           // C 3723

  /* C leaves `string`, `valid` and `method` uninitialised (PORTING.md 1.5); each is assigned
     before its first read.  */
  string = std::ptr::null_mut ();
  valid = 0;
  method = std::ptr::null_mut ();

  /* Make certain AUTHORITY can actually be represented as a Java
     string.  */                                                       // C 3725-3726

  if android_verify_jni_string (authority) != 0                        // C 3728
    { return false; }                                                  // C 3729

  /* Build a string containing AUTHORITY.  */                           // C 3731

  string = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 3733
							    authority);    // C 3734
  android_exception_check ();                                          // C 3735

  method = service_class.valid_authority;                              // C 3737
  valid
    = ((*(*android_java_env)).CallNonvirtualBooleanMethod.unwrap ())   // C 3739
    (android_java_env,                                                 // C 3739
     emacs_service,                                                    // C 3740
     service_class.class,                                              // C 3741
     method, string);                                                  // C 3742
  android_exception_check_1 (string);                                  // C 3743
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, string);   // C 3744
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  return valid != 0;                                                   // C 3745
}

/* C 3748-3848 - `android_saf_root_name` - is **deferred** (the six-item note above); it is an
   `unsafe extern "C" fn`, because C takes its address at C 3692.  */

/* C 3850-3860.  Its address is taken at C 3693.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_open (vnode: *mut android_vnode, flags: c_int,
					    mode: mode_t, asset_p: bool, fd_return: *mut c_int,
					    asset: *mut *mut AAsset) -> c_int
{
  /* /content/storage or one of its authority children cannot be
     opened, as they are virtual directories.  */                      // C 3855-3856

  *errno () = ENOSYS;                                                  // C 3858
  return -1;                                                           // C 3859
}

/* C 3862-3873.  Its address is taken at C 3694.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_saf_root_vnode;                             // C 3865
  let mut save_errno: c_int;                                           // C 3866

  vp = vnode as *mut android_saf_root_vnode;                           // C 3868
  save_errno = *errno ();                                              // C 3869
  xfree ((*vp).authority as *mut c_void);                              // C 3870
  xfree (vp as *mut c_void);                                           // C 3871
  *errno () = save_errno;                                              // C 3872
}

/* C 3875-3880.  Its address is taken at C 3695.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_unlink (vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3878
  return -1;                                                           // C 3879
}

/* C 3882-3888.  Its address is taken at C 3696.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_symlink (target: *const c_char,
					       vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3886
  return -1;                                                           // C 3887
}

/* C 3890-3895.  Its address is taken at C 3697.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_rmdir (vnode: *mut android_vnode) -> c_int
{
  *errno () = EROFS;                                                   // C 3893
  return -1;                                                           // C 3894
}

/* C 3897-3904.  Its address is taken at C 3698.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_rename (src: *mut android_vnode, dst: *mut android_vnode,
					      keep_existing: bool) -> c_int
{
  *errno () = EROFS;                                                   // C 3902
  return -1;                                                           // C 3903
}

/* C 3906-3935.  Its address is taken at C 3699.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_stat (vnode: *mut android_vnode, statb: *mut stat,
					    flags: c_int) -> c_int
{
  let mut vp: *mut android_saf_root_vnode;                             // C 3910

  /* Verify that the authority actually exists and return ENOENT
     otherwise, lest `locate-dominating-file' & co call an operation
     that doesn't require listing URIs under this authority, such as
     access.  */                                                        // C 3912-3915

  vp = vnode as *mut android_saf_root_vnode;                           // C 3917

  if ! (*vp).authority.is_null ()                                      // C 3919
      && ! android_saf_valid_authority_p ((*vp).authority)             // C 3920
    {
      *errno () = ENOENT;                                              // C 3922
      return -1;                                                       // C 3923
    }

  /* Make up some imaginary statistics for this vnode.  */              // C 3926

  memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());      // C 3928
  (*statb).st_uid = getuid ();                                         // C 3929
  (*statb).st_gid = getgid ();                                         // C 3930
  (*statb).st_ino = 0;                                                 // C 3931
  (*statb).st_dev = -4;                                                // C 3932
  (*statb).st_mode = S_IFDIR | S_IRUSR | S_IXUSR;                      // C 3933
  return 0;                                                            // C 3934
}

/* C 3937-3973.  Its address is taken at C 3700.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut vp: *mut android_saf_root_vnode;                             // C 3940

  /* Validate MODE.  */                                                // C 3942

  if mode != F_OK && (mode & (W_OK | X_OK | R_OK)) == 0                 // C 3944
    {
      *errno () = EINVAL;                                              // C 3946
      return -1;                                                       // C 3947
    }

  /* Now, don't allow writing or executing this directory.  */          // C 3950

  if mode != F_OK && (mode & (W_OK | X_OK)) != 0                        // C 3952
    {
      *errno () = EROFS;                                               // C 3954
      return -1;                                                       // C 3955
    }

  /* Verify that the authority actually exists and return ENOENT
     otherwise, lest `locate-dominating-file' & co call an operation
     that doesn't require listing URIs under this authority, such as
     access.  */                                                        // C 3958-3961

  vp = vnode as *mut android_saf_root_vnode;                           // C 3963

  if ! (*vp).authority.is_null ()                                      // C 3965
      && ! android_saf_valid_authority_p ((*vp).authority)             // C 3966
    {
      *errno () = ENOENT;                                              // C 3968
      return -1;                                                       // C 3969
    }

  return 0;                                                            // C 3972
}

/* C 3975-3980.  Its address is taken at C 3701.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  *errno () = EROFS;                                                   // C 3978
  return -1;                                                           // C 3979
}

/* C 3982-3988.  Its address is taken at C 3702.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_chmod (vnode: *mut android_vnode, mode: mode_t,
					     flags: c_int) -> c_int
{
  *errno () = EACCES;                                                  // C 3986
  return -1;                                                           // C 3987
}

/* C 3990-3996.  Its address is taken at C 3703.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
						size: size_t) -> ssize_t
{
  *errno () = EINVAL;                                                  // C 3994
  return -1;                                                           // C 3995
}

/* C 3998-4049.  C's `static struct dirent *dirent;` is a function-scope `static mut` pointer here
   (it is allocated by `xrealloc` and freed at the end of the stream, which is why C's declaration
   is a pointer and not a struct).  Its address is taken at C 4159, so it is an
   `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_readdir (vdir: *mut android_vdir) -> *mut dirent
{
  static mut dirent: *mut dirent = std::ptr::null_mut ();              // C 4001
  let mut string: jobject;                                             // C 4002
  let mut chars: *const c_char;                                        // C 4003
  let mut length: size_t;                                              // C 4004
  let mut size: size_t;                                                // C 4004
  let mut dir: *mut android_saf_root_vdir;                             // C 4005

  /* C leaves `string`, `chars`, `length` and `size` uninitialised (PORTING.md 1.5); each is
     assigned before its first read.  */
  string = std::ptr::null_mut ();
  chars = std::ptr::null ();
  length = 0;
  size = 0;

  dir = vdir as *mut android_saf_root_vdir;                            // C 4007

  if (*dir).i == (*dir).length                                         // C 4009
    {
      /* At the end of the stream.  Free dirent and return NULL.  */   // C 4011

      xfree (dirent as *mut c_void);                                   // C 4013
      dirent = std::ptr::null_mut ();                                  // C 4014
      return std::ptr::null_mut ();                                    // C 4015
    }

  /* Get this string.  */                                              // C 4018
  string = ((*(*android_java_env)).GetObjectArrayElement.unwrap ())    // C 4019
    (android_java_env,                                                 // C 4019
     (*dir).array, (*dir).i);                                          // C 4020
  (*dir).i += 1;                                                       // C 4020 (C's post-increment)
  android_exception_check ();                                          // C 4021
  chars = ((*(*android_java_env)).GetStringUTFChars.unwrap ())         // C 4022
    (android_java_env,                                                 // C 4022
     string as jstring,                                                 // C 4023
     std::ptr::null_mut ());                                           // C 4024
  android_exception_check_nonnull (chars as *mut c_void, string);      // C 4025

  /* Figure out how large it is, and then resize dirent to fit--this
     string is always ASCII.  */                                       // C 4027-4028
  length = strlen (chars) + 1;                                         // C 4029
  size   = std::mem::offset_of! (dirent, d_name) + length;             // C 4030
  dirent = xrealloc (dirent as *mut c_void, size) as *mut dirent;      // C 4031

  /* Clear dirent.  */                                                 // C 4033
  memset (dirent as *mut c_void, 0, size);                             // C 4034

  /* Fill in the generic directory information and copy the string
     over.  */                                                         // C 4036-4037
  (*dirent).d_ino = 0;                                                 // C 4038
  (*dirent).d_off = 0;                                                 // C 4039
  (*dirent).d_reclen = size as c_ushort;                               // C 4040
  (*dirent).d_type = DT_DIR as c_uchar;                                // C 4041
  strcpy ((*dirent).d_name.as_mut_ptr (), chars);                      // C 4042

  /* Release the string data and the local reference to STRING.  */     // C 4044
  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ())             // C 4045
    (android_java_env,                                                 // C 4045
     string as jstring, chars);                                        // C 4046
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, string);   // C 4047
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  return dirent;                                                       // C 4048
}

/* C 4051-4082.  Its address is taken at C 4160; the `for (next = …; (tem = *next);)` loop is the
   load-then-test pair (`android_afs_closedir`'s shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_saf_root_vdir;                             // C 4054
  let mut next: *mut *mut android_saf_root_vdir;                       // C 4054
  let mut tem: *mut android_saf_root_vdir;                             // C 4054

  dir = vdir as *mut android_saf_root_vdir;                            // C 4056

  /* If the ``directory file descriptor'' has been opened, close
     it.  */                                                           // C 4058-4059

  if (*dir).fd != -1                                                   // C 4061
    { close ((*dir).fd); }                                             // C 4062

  /* Delete the local reference to the file name array.  */             // C 4064
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, (*dir).array);   // C 4065
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Free the authority name if set.  */                               // C 4067
  xfree ((*dir).authority as *mut c_void);                             // C 4068

  /* Now unlink this directory.  */                                    // C 4070

  next = &raw mut all_saf_root_vdirs;                                  // C 4072
  loop
    {
      tem = *next;                                                     // C 4072
      if tem.is_null ()
	{ break; }
      if tem == dir                                                    // C 4074
	{ *next = (*dir).next; }                                       // C 4075
      else
	{ next = &raw mut (**next).next; }                             // C 4077
    }

  /* Free the directory itself.  */                                    // C 4080
  xfree (dir as *mut c_void);                                          // C 4081
}

/* C 4084-4103.  Its address is taken at C 4161.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_saf_root_vdir;                             // C 4087

  dir = vdir as *mut android_saf_root_vdir;                            // C 4089

  /* Since `android_saf_root_opendir' tries to avoid opening a file
     descriptor if readdir isn't called, dirfd can fail if open fails.

     open sets errno to a set of errors different from what POSIX
     stipulates for dirfd, but for ease of implementation the open
     errors are used instead.  */                                      // C 4091-4096

  if (*dir).fd >= 0                                                    // C 4098
    { return (*dir).fd; }                                              // C 4099

  (*dir).fd = open (c"/dev/null".as_ptr (), O_RDONLY | O_CLOEXEC);     // C 4101
  return (*dir).fd;                                                    // C 4102
}

/* C 4105-4207.  Its address is taken at C 3704, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_root_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut vp: *mut android_saf_root_vnode;                             // C 4108
  let mut array: jobjectArray;                                         // C 4109
  let mut method: jmethodID;                                           // C 4110
  let mut authority: jstring;                                          // C 4111
  let mut dir: *mut android_saf_root_vdir;                             // C 4112
  let mut length: size_t;                                              // C 4113

  /* C leaves all six uninitialised (PORTING.md 1.5); each is assigned before its first read on the
     path that reads it.  */
  vp = std::ptr::null_mut ();
  array = std::ptr::null_mut ();
  method = std::ptr::null_mut ();
  authority = std::ptr::null_mut ();
  dir = std::ptr::null_mut ();
  length = 0;

  vp = vnode as *mut android_saf_root_vnode;                           // C 4115

  if ! (*vp).authority.is_null ()                                      // C 4117
    {
      /* Build a string containing the authority.  */                  // C 4119
      length = strlen ((*vp).authority);                               // C 4120
      authority = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4121
								   (*vp).authority);   // C 4122
      android_exception_check ();                                      // C 4123

      /* Acquire a list of every tree provided by this authority.  */   // C 4125

      method = service_class.get_document_trees;                       // C 4127
      array
	= ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())   // C 4129
	(android_java_env,                                             // C 4129
	 emacs_service,                                                // C 4130
	 service_class.class,                                          // C 4131
	 method, authority);                                           // C 4132
      android_exception_check_1 (authority);                           // C 4133
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, authority);   // C 4134
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

      /* Ascertain the length of the array.  If it is empty or NULL,
         return ENOENT.  */                                            // C 4136-4137

      if array.is_null ()                                              // C 4139
	{
	  *errno () = ENOENT;                                          // C 4141
	  return std::ptr::null_mut ();                                // C 4142
	}

      length = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env, array) as size_t;   // C 4145

      if length == 0                                                   // C 4147
	{
	  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, array);   // C 4149
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
	  *errno () = ENOENT;                                          // C 4150
	  return std::ptr::null_mut ();                                // C 4151
	}

      /* Now allocate the directory stream.  It will retain a local
	 reference to the array, and should thus only be used within the
	 same JNI local reference frame.  */                           // C 4154-4156

      dir = xmalloc (std::mem::size_of::<android_saf_root_vdir> ()) as *mut android_saf_root_vdir;   // C 4158
      (*dir).vdir.readdir
	= Some (std::mem::transmute::<unsafe extern "C" fn (*mut android_vdir) -> *mut dirent,
				      unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>
		(android_saf_root_readdir));                           // C 4159
      (*dir).vdir.closedir = Some (android_saf_root_closedir);         // C 4160
      (*dir).vdir.dirfd = Some (android_saf_root_dirfd);               // C 4161
      (*dir).fd = -1;                                                  // C 4162
      (*dir).array = array;                                            // C 4163
      (*dir).length = length as jsize;                                 // C 4164
      (*dir).i = 0;                                                    // C 4165
      (*dir).authority = xstrdup ((*vp).authority);                    // C 4166

      /* Link this stream onto the list of all SAF root directory
	 streams.  */                                                  // C 4168-4169
      (*dir).next = all_saf_root_vdirs;                                // C 4170
      all_saf_root_vdirs = dir;                                        // C 4171
      return &raw mut (*dir).vdir;                                     // C 4172
    }

  /* Acquire a list of every document authority.  */                    // C 4175

  method = service_class.get_document_authorities;                     // C 4177
  array = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())   // C 4178
    (android_java_env,                                                 // C 4178
     emacs_service,                                                    // C 4179
     service_class.class,                                              // C 4180
     method);                                                          // C 4181
  android_exception_check ();                                          // C 4182

  if array.is_null ()                                                  // C 4184
    { emacs_abort (); }                                                // C 4185

  /* Now allocate the directory stream.  It will retain a local
     reference to the array, and should thus only be used within the
     same JNI local reference frame.  */                               // C 4187-4189

  dir = xmalloc (std::mem::size_of::<android_saf_root_vdir> ()) as *mut android_saf_root_vdir;   // C 4191
  (*dir).vdir.readdir
    = Some (std::mem::transmute::<unsafe extern "C" fn (*mut android_vdir) -> *mut dirent,
				  unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>
	    (android_saf_root_readdir));                               // C 4192
  (*dir).vdir.closedir = Some (android_saf_root_closedir);             // C 4193
  (*dir).vdir.dirfd = Some (android_saf_root_dirfd);                   // C 4194
  (*dir).fd = -1;                                                      // C 4195
  (*dir).array = array;                                                // C 4196
  (*dir).length = ((*(*android_java_env)).GetArrayLength.unwrap ()) (android_java_env,   // C 4197
								     array) as jsize;   // C 4198
  (*dir).i = 0;                                                        // C 4199
  (*dir).authority = std::ptr::null_mut ();                            // C 4200

  /* Link this stream onto the list of all SAF root directory
     streams.  */                                                      // C 4202-4203
  (*dir).next = all_saf_root_vdirs;                                    // C 4204
  all_saf_root_vdirs = dir;                                            // C 4205
  return &raw mut (*dir).vdir;                                         // C 4206
}

/* C 4209-4229 - `android_saf_root_initial` - is **deferred** (the six-item note above): it calls
   `android_saf_root_name`, and it is an `unsafe extern "C" fn` (C takes its address at the
   `android_vfs_entries` table).  */

/* Return any open SAF root directory stream for which dirfd has
   returned the file descriptor DIRFD.  Return NULL otherwise.  */     // C 4231-4232

/* C 4234-4246.  C calls it directly (C 7380), never through a pointer, so it stays a Rust-ABI
   `unsafe fn`; the `for (dir = …; dir; dir = dir->next)` loop is the load-then-test pair.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_root_get_directory (dirfd: c_int) -> *mut android_saf_root_vdir
{
  let mut dir: *mut android_saf_root_vdir;                             // C 4237

  dir = all_saf_root_vdirs;                                            // C 4239
  while ! dir.is_null ()
    {
      if (*dir).fd == dirfd && dirfd != -1                             // C 4241
	{ return dir; }                                                // C 4242
      dir = (*dir).next;                                               // C 4239
    }

  return std::ptr::null_mut ();                                        // C 4245
}

/* C 4343's `TYPE_MAXIMUM (off_t)`: `<intprops.h>`'s macro, which the frozen mirror carries
   (`lib/include/intprops.rs` 80/96, whose body computes the signed/unsigned maximum of a type).
   This unit does not include that header, so the name stands here at the use site (P-72 rule 4);
   for the integer type the one call site uses, `<$t>::MAX` is that body's value.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MAXIMUM {
  ($t:ty) => { <$t>::MAX };
}

/* Functions common to both SAF directory and file nodes.  */           // C 4250

/* Whether or not Emacs is within an operation running from the SAF
   thread.  */                                                         // C 4252-4253
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut inside_saf_critical_section: bool = false;                  // C 4254

/* Return file status for the document designated by ID_NAME within
   the document tree identified by URI_NAME.

   If NO_CACHE, don't cache the resulting file status.  Enable this
   option if the file status is subject to imminent change.

   If the file status is available, place it within *STATB and return
   0.  If not, return -1 and set errno to EPERM.  */                    // C 4256-4263

/* C 4265-4358.  C calls it directly (the tree and file families, C 5520 onward), never through a
   pointer, so it stays a Rust-ABI `unsafe fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_stat (uri_name: *const c_char, id_name: *const c_char, statb: *mut stat,
			    no_cache: bool) -> c_int
{
  let mut method: jmethodID;                                           // C 4269
  let mut uri: jstring;                                                // C 4270
  let mut id: jstring;                                                 // C 4270
  let mut status: jobject;                                             // C 4271
  let mut mode: jlong;                                                 // C 4272
  let mut size: jlong;                                                 // C 4272
  let mut mtim: jlong;                                                 // C 4272
  let mut longs: *mut jlong;                                           // C 4272

  /* C leaves all eight uninitialised (PORTING.md 1.5); each is assigned before its first read on
     every path that reads it.  */
  method = std::ptr::null_mut ();
  uri = std::ptr::null_mut ();
  id = std::ptr::null_mut ();
  status = std::ptr::null_mut ();
  mode = 0;
  size = 0;
  mtim = 0;
  longs = std::ptr::null_mut ();

  /* Now guarantee that it is safe to call functions which
     synchronize with the SAF thread.  */                              // C 4274-4275

  if inside_saf_critical_section                                       // C 4277
    {
      *errno () = EIO;                                                 // C 4279
      return -1;                                                       // C 4280
    }

  /* Build strings for both URI and ID.  */                             // C 4283
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, uri_name);   // C 4284
  android_exception_check ();                                          // C 4285

  if ! id_name.is_null ()                                              // C 4287
    {
      id = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4289
							    id_name);          // C 4290
      android_exception_check_1 (uri);                                 // C 4291
    }
  else
    { id = std::ptr::null_mut (); }                                    // C 4294

  /* Try to retrieve the file status.  */                              // C 4296
  method = service_class.stat_document;                                // C 4297
  inside_saf_critical_section = true;                                  // C 4298
  status
    = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())    // C 4300
    (android_java_env,                                                 // C 4300
     emacs_service,                                                    // C 4301
     service_class.class,                                              // C 4302
     method, uri, id,                                                  // C 4303
     (no_cache) as c_int);                                             // C 4304 (C's own variadic
									//  promotion of `jboolean`)
  inside_saf_critical_section = false;                                 // C 4305

  /* Check for exceptions and release unneeded local references.  */    // C 4307

  if ! id.is_null ()                                                   // C 4309
    {
      if android_saf_exception_check (2, uri, id) != 0                 // C 4311
	{ return -1; }                                                 // C 4312

      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, id);   // C 4314
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
    }
  else if android_saf_exception_check (1, uri) != 0                    // C 4316
    { return -1; }                                                     // C 4317

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 4319
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Check for failure.  */                                            // C 4321

  if status.is_null ()                                                 // C 4323
    {
      *errno () = EPERM;                                               // C 4325
      return -1;                                                       // C 4326
    }

  /* Read the file status from the array returned.  */                 // C 4329

  longs = ((*(*android_java_env)).GetLongArrayElements.unwrap ())      // C 4331
    (android_java_env,                                                 // C 4331
     status, std::ptr::null_mut ());                                   // C 4332
  android_exception_check_nonnull (longs as *mut c_void, status);      // C 4333
  mode = *longs;                                                       // C 4334
  size = *longs.offset (1);                                            // C 4335
  mtim = *longs.offset (2);                                            // C 4336
  ((*(*android_java_env)).ReleaseLongArrayElements.unwrap ()) (android_java_env, status,   // C 4337
							       longs, JNI_ABORT);   // C 4338
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, status);   // C 4339
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Fill in STATB with this information.  */                          // C 4341
  memset (statb as *mut c_void, 0, std::mem::size_of::<stat> ());      // C 4342
  (*statb).st_size = MAX! (0, MIN! (TYPE_MAXIMUM! (off_t), size) as off_t) as off_t;   // C 4343
  (*statb).st_mode = mode as mode_t;                                   // C 4344
  (*statb).st_dev = -4;                                                // C 4345
  /* C 4346-4354's `#ifdef STAT_TIMESPEC` pair: the live arm is `STAT_TIMESPEC (statb, st_mtim)`,
     lib/stat-time.h 61's `##`-composed member name (the `android_afs_open` note), and the `#else`
     arm's `st_mtime`/`st_mtime_nsec` are the gnulib override's fields the mirror does not carry,
     so that arm is carried as a record (Deviation 22's Owed item).  */
  #[cfg (feature = "stat_timespec")]
  {
    (*statb).st_mtimespec.tv_sec = mtim / 1000;                        // C 4347
    (*statb).st_mtimespec.tv_nsec = (mtim % 1000) * 1000000;           // C 4348
  }
  #[cfg (not (feature = "stat_timespec"))]
  {
    /* C 4352-4353:
	 statb->st_mtime = mtim / 1000;
	 statb->st_mtime_nsec = (mtim % 1000) * 1000000;  */
  }
  (*statb).st_uid = getuid ();                                         // C 4355
  (*statb).st_gid = getgid ();                                         // C 4356
  return 0;                                                            // C 4357
}

/* Detect if Emacs has access to the document designated by the
   document ID ID_NAME within the tree URI_NAME.  If ID_NAME is NULL,
   use the document ID in URI_NAME itself.

   If WRITABLE, also check that the file is writable, which is true
   if it is either a directory or its flags contains
   FLAG_SUPPORTS_WRITE.

   Value is 0 if the file is accessible, and -1 with errno set
   appropriately if not.  */                                           // C 4360-4369

/* C 4371-4445.  C calls it directly (C 5425 onward), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_access (uri_name: *const c_char, id_name: *const c_char,
			      writable: bool) -> c_int
{
  let mut method: jmethodID;                                           // C 4375
  let mut uri: jstring;                                                // C 4376
  let mut id: jstring;                                                 // C 4376
  let mut rc: jint;                                                    // C 4377

  /* C leaves all four uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  method = std::ptr::null_mut ();
  uri = std::ptr::null_mut ();
  id = std::ptr::null_mut ();
  rc = 0;

  /* Now guarantee that it is safe to call functions which
     synchronize with the SAF thread.  */                              // C 4379-4380

  if inside_saf_critical_section                                       // C 4382
    {
      *errno () = EIO;                                                 // C 4384
      return -1;                                                       // C 4385
    }

  /* Build strings for both URI and ID.  */                             // C 4388
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, uri_name);   // C 4389
  android_exception_check ();                                          // C 4390

  if ! id_name.is_null ()                                              // C 4392
    {
      id = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4394
							    id_name);          // C 4395
      android_exception_check_1 (uri);                                 // C 4396
    }
  else
    { id = std::ptr::null_mut (); }                                    // C 4399

  /* Try to retrieve the file status.  */                              // C 4401
  method = service_class.access_document;                              // C 4402
  inside_saf_critical_section = true;                                  // C 4403
  rc = ((*(*android_java_env)).CallNonvirtualIntMethod.unwrap ())      // C 4404
    (android_java_env,                                                 // C 4404
     emacs_service,                                                    // C 4405
     service_class.class,                                              // C 4406
     method, uri, id,                                                  // C 4407
     (writable) as c_int);                                             // C 4408 (C's variadic
									//  promotion)
  inside_saf_critical_section = false;                                 // C 4409

  /* Check for exceptions and release unneeded local references.  */    // C 4411

  if ! id.is_null ()                                                   // C 4413
    {
      if android_saf_exception_check (2, uri, id) != 0                 // C 4415
	{ return -1; }                                                 // C 4416

      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, id);   // C 4418
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
    }
  else if android_saf_exception_check (1, uri) != 0                    // C 4420
    { return -1; }                                                     // C 4421

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 4423
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  match rc                                                             // C 4425
    {
      -1 =>                                                            // C 4427
	{
	  /* -1 means it doesn't exist.  */                            // C 4428
	  *errno () = ENOENT;                                          // C 4429
	  return -1;                                                   // C 4430
	}

      -2 =>                                                            // C 4432
	{
	  /* -2 means access has been denied.  */                      // C 4433
	  *errno () = EACCES;                                          // C 4434
	  return -1;                                                   // C 4435
	}

      -3 =>                                                            // C 4437
	{
	  /* -3 refers to an internal error.  */                       // C 4438
	  *errno () = EIO;                                             // C 4439
	  return -1;                                                   // C 4440
	}

      _ => {}
    }

  /* Return success.  */                                               // C 4443
  return 0;                                                            // C 4444
}

/* Delete the document designated by DOC_ID within the tree identified
   through the URI TREE.  Return 0 if the document has been deleted,
   set errno and return -1 upon failure.

   DOC_NAME should be the name of the file itself, as a file name
   whose constituent components lead to a document named DOC_ID.  It
   isn't used to search for a document ID, but is used to invalidate
   the file cache.  */                                                  // C 4447-4454

/* C 4456-4497.  C calls it directly (C 5297), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_delete_document (tree: *const c_char, doc_id: *const c_char,
				       doc_name: *const c_char) -> c_int
{
  let mut id: jobject;                                                 // C 4460
  let mut uri: jobject;                                                // C 4460
  let mut name: jobject;                                               // C 4460
  let mut method: jmethodID;                                           // C 4461
  let mut rc: jint;                                                    // C 4462

  /* C leaves all five uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  id = std::ptr::null_mut ();
  uri = std::ptr::null_mut ();
  name = std::ptr::null_mut ();
  method = std::ptr::null_mut ();
  rc = 0;

  /* Build the strings holding the ID, URI and NAME.  */                // C 4464
  id = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4465
							doc_id);        // C 4466
  android_exception_check ();                                          // C 4467
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4468
							 tree);         // C 4469
  android_exception_check_1 (id);                                      // C 4470
  name = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4471
							  doc_name);    // C 4472
  android_exception_check_2 (id, name);                                // C 4473

  /* Now, try to delete the document.  */                               // C 4475
  method = service_class.delete_document;                              // C 4476
  rc = ((*(*android_java_env)).CallNonvirtualIntMethod.unwrap ())      // C 4477
    (android_java_env,                                                 // C 4477
     emacs_service,                                                    // C 4478
     service_class.class,                                              // C 4479
     method, uri, id,                                                  // C 4480
     name);                                                            // C 4481

  if android_saf_exception_check (3, id, uri, name) != 0               // C 4483
    { return -1; }                                                     // C 4484

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, id);   // C 4486
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);  // C 4487 (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, name);  // C 4488 (C's
									//  ANDROID_DELETE_LOCAL_REF)

  if rc != 0                                                           // C 4490
    {
      *errno () = EACCES;                                              // C 4492
      return -1;                                                       // C 4493
    }

  return 0;                                                            // C 4496
}

/* C 4499-4501: C's `static int android_document_id_from_name (const char *, const char *,
   char **);` under the comment `/* Declared further below.  */` - a forward declaration for a
   function C defines at C 4806.  Rust needs no text for it (PORTING.md 10.10).  */

/* Rename the document designated by DOC_ID inside the directory tree
   identified by URI, which should be within the directory by the name
   of DIR, to NAME.  If the document can't be renamed, return -1 and
   set errno to a value describing the error.  Return 0 if the rename
   is successful.

   Android permits the same document to appear in multiple
   directories, but stores the display name inside the document
   ``inode'' itself instead of the directory entries that refer to it.
   Because of this, this operation may cause other directory entries
   outside DIR to be renamed.  */                                      // C 4503-4513

/* C 4515-4572.  C calls it directly (C 5251), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_rename_document (uri: *const c_char, doc_id: *const c_char,
				       dir: *const c_char, name: *const c_char) -> c_int
{
  let mut rc: c_int;                                                   // C 4519
  let mut uri1: jstring;                                               // C 4520
  let mut doc_id1: jstring;                                            // C 4520
  let mut dir1: jstring;                                               // C 4520
  let mut name1: jstring;                                              // C 4520
  let mut method: jmethodID;                                           // C 4521

  /* C leaves all six uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  rc = 0;
  uri1 = std::ptr::null_mut ();
  doc_id1 = std::ptr::null_mut ();
  dir1 = std::ptr::null_mut ();
  name1 = std::ptr::null_mut ();
  method = std::ptr::null_mut ();

  /* Now build the strings for the URI, document ID, directory name
     and directory ID.  */                                             // C 4523-4524

  uri1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, uri);   // C 4526
  android_exception_check ();                                          // C 4527
  doc_id1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, doc_id);   // C 4528
  android_exception_check_1 (uri1);                                    // C 4529
  dir1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, dir);   // C 4530
  android_exception_check_2 (doc_id1, uri1);                           // C 4531
  name1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, name);   // C 4532
  android_exception_check_3 (dir1, doc_id1, uri1);                     // C 4533

  method = service_class.rename_document;                              // C 4535
  rc = ((*(*android_java_env)).CallNonvirtualIntMethod.unwrap ())      // C 4536
    (android_java_env,                                                 // C 4536
     emacs_service,                                                    // C 4537
     service_class.class,                                              // C 4538
     method, uri1, doc_id1,                                            // C 4539
     dir1, name1);                                                     // C 4540

  /* Check for exceptions.  */                                         // C 4542

  if android_saf_exception_check (4, uri1, doc_id1, dir1, name1) != 0   // C 4544
    {
      /* Substitute EXDEV for ENOSYS, so callers fall back on
	 delete-then-copy.  */                                         // C 4546-4547

      if *errno () == ENOSYS                                           // C 4549
	{ *errno () = EXDEV; }                                         // C 4550

      return -1;                                                       // C 4552
    }

  /* Delete unused local references.  */                               // C 4555
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri1);   // C 4556
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, doc_id1);   // C 4557
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, dir1);   // C 4558
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, name1);   // C 4559
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Then check for errors handled within the Java code.  */            // C 4561

  if rc == -1                                                          // C 4563
    {
      /* UnsupportedOperationException.  Trick the caller into falling
	 back on delete-then-copy code.  */                            // C 4565-4566
      *errno () = EXDEV;                                               // C 4567
      return -1;                                                       // C 4568
    }

  return 0;                                                            // C 4571
}

/* C 4574-4677 - `android_saf_move_document` - is **deferred** with the SAF tree family: it calls
   C 4806's `android_document_id_from_name`, whose forward declaration C writes at C 4499-4501.
   C calls it directly (C 4720's sibling), never through a pointer.  */

/* C 4499's and this round's three remaining names: android.h 131-133's `android_exception_check_2`
   / `_3` / `_4`, declared at the use site like their `_1` sibling (the `android.h` mirror is not in
   this unit's include graph).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: android.h 131: `extern void android_exception_check_2 (jobject, jobject);`
  pub fn android_exception_check_2 (arg0: jobject, arg1: jobject);
  /// C: android.h 132: `extern void android_exception_check_3 (jobject, jobject, jobject);`
  pub fn android_exception_check_3 (arg0: jobject, arg1: jobject, arg2: jobject);
  /// C: android.h 133: `extern void android_exception_check_4 (jobject, jobject, jobject, jobject);`
  pub fn android_exception_check_4 (arg0: jobject, arg1: jobject, arg2: jobject, arg3: jobject);
}

/* Move the document designated by *DOC_ID from the directory under
   DIR_NAME to the directory designated by DST_ID.  All three
   directories are located within the tree identified by the given
   URI.

   If the document's ID changes as a result of the movement, free
   *DOC_ID and store the new document ID within.

   Value is 0 upon success, -1 otherwise with errno set.  */            // C 4574-4582

/* C 4584-4677.  C calls it directly (C 5300), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_move_document (uri: *const c_char, doc_id: *mut *mut c_char,
				     dir_name: *const c_char, dst_id: *const c_char) -> c_int
{
  let mut src_id: *mut c_char;                                         // C 4588
  let mut id: *mut c_char;                                             // C 4588
  let mut uri1: jobject;                                               // C 4589
  let mut doc_id1: jobject;                                            // C 4589
  let mut dir_name1: jobject;                                          // C 4589
  let mut dst_id1: jobject;                                            // C 4589
  let mut src_id1: jobject;                                            // C 4589
  let mut result: jstring;                                             // C 4590
  let mut method: jmethodID;                                           // C 4591
  let mut rc: c_int;                                                   // C 4592
  let mut new_id: *const c_char;                                       // C 4593

  /* C leaves every one of them uninitialised (PORTING.md 1.5); each is assigned before its first
     read on the path that reads it.  */
  src_id = std::ptr::null_mut ();
  id = std::ptr::null_mut ();
  uri1 = std::ptr::null_mut ();
  doc_id1 = std::ptr::null_mut ();
  dir_name1 = std::ptr::null_mut ();
  dst_id1 = std::ptr::null_mut ();
  src_id1 = std::ptr::null_mut ();
  result = std::ptr::null_mut ();
  method = std::ptr::null_mut ();
  rc = 0;
  new_id = std::ptr::null ();

  /* Obtain the name of the source directory.  */                       // C 4595
  src_id = std::ptr::null_mut ();                                      // C 4596
  rc = android_document_id_from_name (uri, dir_name, &raw mut src_id); // C 4597

  if rc != 1                                                           // C 4599
    {
      /* This file is either not a directory or nonexistent.  */       // C 4601
      xfree (src_id as *mut c_void);                                   // C 4602

      match rc                                                         // C 4604
	{
	  0 =>                                                         // C 4606
	    {
	      *errno () = ENOTDIR;                                     // C 4607
	      return -1;                                               // C 4608
	    }

	  -1 | -2 =>                                                   // C 4610-4611
	    {
	      *errno () = ENOENT;                                      // C 4612
	      return -1;                                               // C 4613
	    }

	  _ =>                                                         // C 4615
	    {
	      emacs_abort ();                                          // C 4616
	    }
	}
    }

  /* Build Java strings for all five arguments.  */                     // C 4620
  id = *doc_id;                                                        // C 4621
  uri1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, uri);   // C 4622
  android_exception_check ();                                          // C 4623
  doc_id1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, id);   // C 4624
  android_exception_check_1 (uri1);                                    // C 4625
  dir_name1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, dir_name);   // C 4626
  android_exception_check_2 (doc_id1, uri1);                           // C 4627
  dst_id1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, dst_id);   // C 4628
  android_exception_check_3 (dir_name1, doc_id1, uri1);                // C 4629
  src_id1 = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, src_id);   // C 4630
  xfree (src_id as *mut c_void);                                       // C 4631
  android_exception_check_4 (dst_id1, dir_name1, doc_id1, uri1);       // C 4632

  /* Do the rename.  */                                                // C 4634
  method = service_class.move_document;                                // C 4635
  result
    = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())    // C 4637
    (android_java_env,                                                 // C 4637
     emacs_service,                                                    // C 4638
     service_class.class,                                              // C 4639
     method, uri1,                                                     // C 4640
     doc_id1, dir_name1,                                               // C 4641
     dst_id1, src_id1);                                                // C 4642
  if android_saf_exception_check (5, src_id1, dst_id1, dir_name1,      // C 4643
				  doc_id1, uri1) != 0                  // C 4644
    {
      /* Substitute EXDEV for ENOSYS, so callers fall back on
	 delete-then-copy.  */                                         // C 4646-4647

      if *errno () == ENOSYS                                           // C 4649
	{ *errno () = EXDEV; }                                         // C 4650

      return -1;                                                       // C 4652
    }

  /* Delete unused local references.  */                               // C 4655
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, src_id1);   // C 4656
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, dst_id1);   // C 4657
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, dir_name1);   // C 4658
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, doc_id1);   // C 4659
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri1);   // C 4660
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  if ! result.is_null ()                                               // C 4662
    {
      /* The document ID changed.  Free id and replace *DOC_ID with
	 the new ID.  */                                               // C 4664-4665
      xfree (id as *mut c_void);                                       // C 4666
      new_id = ((*(*android_java_env)).GetStringUTFChars.unwrap ())    // C 4667
	(android_java_env,                                             // C 4667
	 result, std::ptr::null_mut ());                               // C 4668
      android_exception_check_nonnull (new_id as *mut c_void, result);   // C 4669
      *doc_id = xstrdup (new_id);                                      // C 4670
      ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env, result,   // C 4671
								new_id); // C 4672
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, result);   // C 4673
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
    }

  return 0;                                                            // C 4676
}

/* SAF directory vnode.  A file within a SAF directory tree is
   identified by the URI of the directory tree itself, an opaque
   ``file identifier'' value, and a display name.  This information is
   recorded in each vnode representing either a directory or a file
   itself.  */                                                         // C 4681-4685

/* C 4687-4712: `struct android_saf_tree_vnode'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_saf_tree_vnode
{
  /* The vnode data itself.  */
  pub vnode: android_vnode,                                            // C 4690

  /* The URI of the directory tree represented.  This is Java string
     data in ``modified UTF format'', which is essentially a modified
     UTF-8 format capable of storing NULL bytes while also utilizing
     NULL termination.  */
  pub tree_uri: *const c_char,                                         // C 4696

  /* The ID of the document tree designated by TREE_URI.  */
  pub tree_id: *mut c_char,                                            // C 4699

  /* The document ID of the directory represented, or NULL if this is
     the root directory of the tree.  Since file and new vnodes don't
     represent the root directory, this field is always set in
     them.  */
  pub document_id: *mut c_char,                                        // C 4705

  /* The file name of this tree vnode.  This is a ``path'' to the
     file, where each directory component consists of the display name
     of a directory leading up to a file within, terminated with a
     directory separator character.  */
  pub name: *mut c_char,                                               // C 4711
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_saf_tree_vnode> () == 48);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vnode, vnode) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vnode, tree_uri) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vnode, tree_id) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vnode, document_id) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vnode, name) == 40);

/* C 4714-4732: `struct android_saf_tree_vdir'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_saf_tree_vdir
{
  /* The virtual directory stream function table.  */
  pub vdir: android_vdir,                                              // C 4717

  /* The next directory in `all_saf_tree_vdirs'.  */
  pub next: *mut android_saf_tree_vdir,                                // C 4720

  /* Name of this directory relative to the root file system.  */
  pub name: *mut c_char,                                               // C 4723

  /* Local reference to the cursor representing the directory
     stream.  */
  pub cursor: jobject,                                                 // C 4727

  /* The ``directory'' file descriptor used to identify this directory
     stream, or -1.  */
  pub fd: c_int,                                                       // C 4731
}
/* Measured with the NDK's clang (the `android_saf_root_vdir` shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_saf_tree_vdir> () == 56);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vdir, vdir) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vdir, next) == 24);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vdir, name) == 32);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vdir, cursor) == 40);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_saf_tree_vdir, fd) == 48);

/* C 4734-4750: C's twelve forward declarations of this file's own `static` functions
   (`android_saf_tree_name` … `android_saf_tree_opendir`); no Rust text (PORTING.md 10.10).  */

/* C 4752-4769 - `saf_tree_vfs_ops` - is **deferred** (the note above C 4897): it names
   `android_saf_tree_name` (C 4898) and the eleven members after it.  */

/* C 4771-4777: C's two more operation vectors, `saf_file_vfs_ops` and `saf_new_vfs_ops`, under
   their own comments (`/* Vector of VFS operations associated with SAF file VFS nodes.  Defined
   later in the next page.  */` and the ``new'' twin): two file-scope objects C defines in the
   pages after this one, which this lane lands with those families.  */

/* Chain of all open SAF directory streams.  */                         // C 4779
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut all_saf_tree_vdirs: *mut android_saf_tree_vdir = std::ptr::null_mut ();   // C 4780

/* Find the document ID of the file within TREE_URI designated by
   NAME.

   NAME is a ``file name'' comprised of the display names of
   individual files.  Each constituent component prior to the last
   must name a directory file within TREE_URI.

   If NAME is not correct for the Java ``modified UTF-8'' coding
   system, return -1 and set errno to ENOENT.

   Upon success, return 0 or 1 (contingent upon whether or not the
   last component within NAME is a directory) and place the document
   ID of the named file in ID.

   If the designated file doesn't exist, but the penultimate component
   within NAME does and is also a directory, return -2 and place the
   document ID of that directory within *ID.

   If the designated file can't be located, return -1 and set errno
   accordingly.  The reasons for which a file can't be located are not
   all immediately obvious: quitting, for example, can cause document
   ID lookup to be canceled.  */                                       // C 4782-4803

/* C 4805-4895.  C calls it directly (C 4597/5251 and the file family), never through a pointer.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_document_id_from_name (tree_uri: *const c_char, name: *const c_char,
					 id: *mut *mut c_char) -> c_int
{
  let mut result: jobjectArray;                                        // C 4809
  let mut uri: jstring;                                                // C 4810
  let mut java_name: jbyteArray;                                       // C 4811
  let mut rc: jint;                                                    // C 4812
  let mut method: jmethodID;                                           // C 4813
  let mut doc_id: *const c_char;                                       // C 4814

  /* C leaves all six uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  result = std::ptr::null_mut ();
  uri = std::ptr::null_mut ();
  java_name = std::ptr::null_mut ();
  rc = 0;
  method = std::ptr::null_mut ();
  doc_id = std::ptr::null ();

  /* Verify the format of NAME.  Don't allow creating files that
     contain characters that can't be encoded in Java.  */             // C 4816-4817

  if android_verify_jni_string (name) != 0                             // C 4819
    {
      *errno () = ENOENT;                                              // C 4821
      return -1;                                                       // C 4822
    }

  /* Now guarantee that it is safe to call
     `document_id_from_name'.  */                                      // C 4825-4826

  if inside_saf_critical_section                                       // C 4828
    {
      *errno () = EIO;                                                 // C 4830
      return -1;                                                       // C 4831
    }

  /* First, create the array that will hold the result.  */             // C 4834
  result = ((*(*android_java_env)).NewObjectArray.unwrap ()) (android_java_env, 1,   // C 4835
							      java_string_class,   // C 4836
							      std::ptr::null_mut ());   // C 4837
  android_exception_check ();                                          // C 4838

  /* Next, create the string for the tree URI and name.  */             // C 4840
  java_name = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 4841
							       name);  // C 4842
  android_exception_check_1 (result);                                  // C 4843
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env, tree_uri);   // C 4844
  android_exception_check_2 (result, java_name);                       // C 4845

  /* Now, call documentIdFromName.  This will synchronize with the SAF
     thread, so make sure reentrant calls don't happen.  */            // C 4847-4848
  method = service_class.document_id_from_name;                        // C 4849
  inside_saf_critical_section = true;                                  // C 4850
  rc = ((*(*android_java_env)).CallNonvirtualIntMethod.unwrap ())      // C 4851
    (android_java_env,                                                 // C 4851
     emacs_service,                                                    // C 4852
     service_class.class,                                              // C 4853
     method,                                                           // C 4854
     uri, java_name,                                                   // C 4855
     result);                                                          // C 4856
  inside_saf_critical_section = false;                                 // C 4857

  if android_saf_exception_check (3, result, uri, java_name) != 0      // C 4859
    { return -1; }                                                     // C 4860

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 4862
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, java_name);   // C 4863
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* If rc indicates failure, don't try to copy from result.  */        // C 4865

  if rc == -1                                                          // C 4867
    {
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, result);   // C 4869
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
      *errno () = ENOENT;                                              // C 4870
      return -1;                                                       // C 4871
    }

  eassert! (rc == -2 || rc >= 0);                                      // C 4874

  /* Otherwise, obtain the contents of the string returned in Java
     ``UTF-8'' encoding.  */                                           // C 4876-4877
  uri = ((*(*android_java_env)).GetObjectArrayElement.unwrap ()) (android_java_env,   // C 4878
								  result, 0) as jstring;   // C 4879
  android_exception_check_nonnull (uri as *mut c_void, result);        // C 4880
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, result);   // C 4881
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  doc_id = ((*(*android_java_env)).GetStringUTFChars.unwrap ())        // C 4883
    (android_java_env,                                                 // C 4883
     uri, std::ptr::null_mut ());                                      // C 4884
  android_exception_check_nonnull (doc_id as *mut c_void, uri);        // C 4885

  /* Make *ID its copy.  */                                            // C 4887
  *id = xstrdup (doc_id);                                              // C 4888

  /* And release it.  */                                               // C 4890
  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ())             // C 4891
    (android_java_env,                                                 // C 4891
     uri, doc_id);                                                     // C 4892
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 4893
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  return rc;                                                           // C 4894
}

/* C 5114-5122.  Its address is taken at C 4756.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_open (vnode: *mut android_vnode, flags: c_int,
					    mode: mode_t, asset_p: bool, fd: *mut c_int,
					    asset: *mut *mut AAsset) -> c_int
{
  /* Don't allow opening this special directory.  */                    // C 5119
  *errno () = ENOSYS;                                                  // C 5120
  return -1;                                                           // C 5121
}

/* C 5124-5139.  Its address is taken at C 4757.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_close (vnode: *mut android_vnode)
{
  let mut vp: *mut android_saf_tree_vnode;                             // C 5127
  let mut save_errno: c_int;                                           // C 5128

  vp = vnode as *mut android_saf_tree_vnode;                           // C 5130

  save_errno = *errno ();                                              // C 5132
  xfree ((*vp).tree_uri as *mut c_void);                               // C 5133
  xfree ((*vp).tree_id as *mut c_void);                                // C 5134
  xfree ((*vp).name as *mut c_void);                                   // C 5135
  xfree ((*vp).document_id as *mut c_void);                            // C 5136
  xfree (vp as *mut c_void);                                           // C 5137
  *errno () = save_errno;                                              // C 5138
}

/* C 5141-5146.  Its address is taken at C 4758.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_unlink (vnode: *mut android_vnode) -> c_int
{
  *errno () = EISDIR;                                                  // C 5144
  return -1;                                                           // C 5145
}

/* C 5148-5153.  Its address is taken at C 4759.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_symlink (target: *const c_char,
					       vnode: *mut android_vnode) -> c_int
{
  *errno () = EPERM;                                                   // C 5151
  return -1;                                                           // C 5152
}

/* C 5155-5173.  Its address is taken at C 4760.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_rmdir (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_saf_tree_vnode;                             // C 5158

  vp = vnode as *mut android_saf_tree_vnode;                           // C 5160

  /* Don't allow deleting the root directory.  */                       // C 5162

  if (*vp).document_id.is_null ()                                      // C 5164
    {
      *errno () = EROFS;                                               // C 5166
      return -1;                                                       // C 5167
    }

  return android_saf_delete_document ((*vp).tree_uri,                  // C 5170
				      (*vp).document_id,               // C 5171
				      (*vp).name);                     // C 5172
}

/* C 5175-5403.  Its address is taken at C 4762.  C's four `goto error` sites and the `error:`
   label at C 5374 become one labelled block with the label's two statements after it
   (PORTING.md 2.2, the `android_unix_name` family).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_rename (src: *mut android_vnode, dst: *mut android_vnode,
					      keep_existing: bool) -> c_int
{
  let mut last: *mut c_char;                                           // C 5180
  let mut dst_last: *mut c_char;                                       // C 5180
  let mut vp: *mut android_saf_tree_vnode;                             // C 5181
  let mut vdst: *mut android_saf_tree_vnode;                           // C 5181
  let mut path: [c_char; EMACS_PATH_MAX as usize] = [0; EMACS_PATH_MAX as usize];   // C 5182
  let mut path1: [c_char; EMACS_PATH_MAX as usize] = [0; EMACS_PATH_MAX as usize];   // C 5182
  let mut fill: *mut c_char;                                           // C 5183
  let mut dst_id: *mut c_char;                                         // C 5183
  let mut rc: c_int;                                                   // C 5184

  /* C leaves `last`, `dst_last`, `fill`, `dst_id` and `rc` uninitialised (PORTING.md 1.5); each is
     assigned before its first read on the path that reads it.  */
  last = std::ptr::null_mut ();
  dst_last = std::ptr::null_mut ();
  vp = std::ptr::null_mut ();
  vdst = std::ptr::null_mut ();
  fill = std::ptr::null_mut ();
  dst_id = std::ptr::null_mut ();
  rc = 0;

  /* If dst isn't a tree, file or new vnode, return EXDEV.  */          // C 5186

  if (*dst).r#type != ANDROID_VNODE_SAF_TREE as c_short                // C 5188
      && (*dst).r#type != ANDROID_VNODE_SAF_FILE as c_short            // C 5189
      && (*dst).r#type != ANDROID_VNODE_SAF_NEW as c_short             // C 5190
    {
      *errno () = EXDEV;                                               // C 5192
      return -1;                                                       // C 5193
    }

  vp = src as *mut android_saf_tree_vnode;                             // C 5196
  vdst = dst as *mut android_saf_tree_vnode;                           // C 5197

  /* if vp and vdst refer to different tree URIs, return EXDEV.  */     // C 5199

  if strcmp ((*vp).tree_uri, (*vdst).tree_uri) != 0                    // C 5201
    {
      *errno () = EXDEV;                                               // C 5203
      return -1;                                                       // C 5204
    }

  /* If `keep_existing' and the destination vnode designates an
     existing file, return EEXIST.  */                                 // C 5207-5208

  if keep_existing && (*dst).r#type != ANDROID_VNODE_SAF_NEW as c_short   // C 5210
    {
      *errno () = EEXIST;                                              // C 5212
      return -1;                                                       // C 5213
    }

  /* Unix `rename' maps to two Android content provider operations.
     The first case is a simple rename, where src and dst are both
     located within the same directory.  Compare the file names of
     both up to the component before the last.  */                     // C 5216-5219

  last = strrchr ((*vp).name, b'/' as c_int);                          // C 5221
  eassert! (! last.is_null ());                                        // C 5222

  if *last.offset (1) == 0                                             // C 5224
    {
      if last == (*vp).name                                           // C 5226
	{
	  /* This means the caller is trying to rename the root
	     directory of the tree.  */                                // C 5228-5229
	  *errno () = EROFS;                                           // C 5230
	  return -1;                                                   // C 5231
	}

      /* The name is terminated by a trailing directory separator.
         Search backwards for the preceding directory separator.  */   // C 5234-5235
      last = memrchr ((*vp).name as *const c_void, b'/' as c_int,     // C 5236
		      last.offset_from ((*vp).name) as usize) as *mut c_char;   // C 5236
      eassert! (! last.is_null ());                                    // C 5237
    }

  /* Find the end of the second-to-last component in vdst's name.  */   // C 5240

  dst_last = strrchr ((*vdst).name, b'/' as c_int);                    // C 5242
  eassert! (! dst_last.is_null ());                                    // C 5243

  if *dst_last.offset (1) == 0                                         // C 5245
    {
      if dst_last == (*vdst).name                                      // C 5247
	{
	  /* Forbid overwriting the root of the tree either.  */       // C 5249
	  *errno () = EROFS;                                           // C 5250
	  return -1;                                                   // C 5251
	}

      dst_last = memrchr ((*vdst).name as *const c_void, b'/' as c_int,   // C 5254
			  dst_last.offset_from ((*vdst).name) as usize) as *mut c_char;   // C 5254
      eassert! (! dst_last.is_null ());                                // C 5255
    }

  if dst_last.offset_from ((*vdst).name) != last.offset_from ((*vp).name)   // C 5258
      || memcmp ((*vp).name as *const c_void, (*vdst).name as *const c_void,   // C 5259
		 last.offset_from ((*vp).name) as usize) != 0          // C 5259
    {
      'error: {
      /* The second case is where the file must be moved from one
         directory to the other, and possibly then recreated under a
         new name.  */                                                 // C 5261-5263

      /* The names of the source and destination directories will have
	 to be copied to path.  */                                     // C 5265-5266

      if last.offset_from ((*vp).name) >= EMACS_PATH_MAX as isize       // C 5268
	  || dst_last.offset_from ((*vdst).name) >= EMACS_PATH_MAX as isize   // C 5269
	{
	  *errno () = ENAMETOOLONG;                                     // C 5271
	  return -1;                                                   // C 5272
	}

      fill = mempcpy (path.as_mut_ptr () as *mut c_void, (*vp).name as *const c_void,   // C 5275
		      last.offset_from ((*vp).name) as usize) as *mut c_char;   // C 5275
      *fill = 0;                                                        // C 5276

      /* If vdst doesn't already exist, its document_id field is
	 already the name of its parent directory.  */                 // C 5278-5279

      if (*dst).r#type == ANDROID_VNODE_SAF_NEW as c_short              // C 5281
	{
	  /* First, move the document.  This will update
	     VP->document_id if it changes.  */                        // C 5283-5284

	  if android_saf_move_document ((*vp).tree_uri,                 // C 5286
					&raw mut (*vp).document_id,     // C 5287
					path.as_mut_ptr (),             // C 5288
					(*vdst).document_id) != 0       // C 5289
	    { return -1; }                                             // C 5290

	  fill = mempcpy (path.as_mut_ptr () as *mut c_void, (*vdst).name as *const c_void,   // C 5292
			  dst_last.offset_from ((*vdst).name) as usize) as *mut c_char;   // C 5292
	  *fill = 0;                                                    // C 5293

	  /* Next, rename the document, if its display name differs
	     from that of the source.  */                              // C 5295-5296

	  if strcmp (dst_last.offset (1), last.offset (1)) != 0         // C 5298
	      /* By now vp->document_id is already in the destination
		 directory.  */
	      && android_saf_rename_document ((*vp).tree_uri,           // C 5301
					      (*vp).document_id,        // C 5302
					      path.as_mut_ptr (),       // C 5303
					      dst_last.offset (1)) != 0   // C 5304
	    { return -1; }                                              // C 5305

	  return 0;                                                     // C 5307
	}

      /* Retrieve the ID designating the destination document's parent
	 directory.  */                                                // C 5310-5311

      fill = mempcpy (path1.as_mut_ptr () as *mut c_void, (*vdst).name as *const c_void,   // C 5313
		      dst_last.offset_from ((*vdst).name) as usize) as *mut c_char;   // C 5313
      *fill = 0;                                                       // C 5314

      rc = android_document_id_from_name ((*vp).tree_uri,              // C 5316
					  path1.as_mut_ptr (),         // C 5317
					  &raw mut dst_id);            // C 5317

      if rc != 1                                                       // C 5319
	{
	  /* This file is either not a directory or nonexistent.  */    // C 5321

	  match rc                                                      // C 5323
	    {
	      0 =>                                                      // C 5325
		{
		  *errno () = ENOTDIR;                                  // C 5326
		  break 'error;                                         // C 5327 (goto error)
		}

	      -1 =>                                                     // C 5329
		{
		  /* dst_id is not set here, as the penultimate component
		     also couldn't be located.  */                     // C 5330-5331
		  *errno () = ENOENT;                                   // C 5332
		  return -1;                                            // C 5333
		}

	      -2 =>                                                     // C 5335
		{
		  *errno () = ENOENT;                                   // C 5336
		  break 'error;                                         // C 5337 (goto error)
		}

	      _ =>                                                      // C 5339
		{
		  emacs_abort ();                                       // C 5340
		}
	    }
	}

      /* vdst already exists, so it needs to be deleted first.  */      // C 5344

      if android_saf_delete_document ((*vdst).tree_uri,                 // C 5346
				      (*vdst).document_id,              // C 5347
				      (*vdst).name) != 0                // C 5348
	{ break 'error; }                                               // C 5349 (goto error)

      /* First, move the document.  This will update
	 VP->document_id if it changes.  */                             // C 5351-5352

      if android_saf_move_document ((*vp).tree_uri,                     // C 5354
				    &raw mut (*vp).document_id,         // C 5355
				    path.as_mut_ptr (), dst_id) != 0    // C 5356
	{ break 'error; }                                               // C 5357 (goto error)

      /* Next, rename the document, if its display name differs from
	 that of the source.  */                                        // C 5359-5360

      if strcmp (dst_last.offset (1), last.offset (1)) != 0             // C 5362
	  /* By now vp->document_id is already in the destination
	     directory.  */
	  && android_saf_rename_document ((*vp).tree_uri,               // C 5365
					  (*vp).document_id,            // C 5366
					  path1.as_mut_ptr (),          // C 5367
					  dst_last.offset (1)) != 0     // C 5368
	{ break 'error; }                                               // C 5369 (goto error)

      xfree (dst_id as *mut c_void);                                    // C 5371
      return 0;                                                         // C 5372
      }

      /* error: */
      xfree (dst_id as *mut c_void);                                    // C 5375
      return 1;                                                         // C 5376
    }

  /* Otherwise, do this simple rename.  The name of the parent
     directory is required, as it provides the directory whose entries
     will be modified.  */                                              // C 5379-5381

  if last.offset_from ((*vp).name) >= EMACS_PATH_MAX as isize           // C 5383
    {
      *errno () = ENAMETOOLONG;                                         // C 5385
      return -1;                                                        // C 5386
    }

  /* If the destination document exists, delete it.  */                 // C 5389

  if (*dst).r#type != ANDROID_VNODE_SAF_NEW as c_short                  // C 5391
      && android_saf_delete_document ((*vdst).tree_uri,                 // C 5392
				      (*vdst).document_id,              // C 5393
				      (*vdst).name) != 0                // C 5394
    { return -1; }                                                      // C 5395

  fill = mempcpy (path.as_mut_ptr () as *mut c_void, (*vp).name as *const c_void,   // C 5397
		  last.offset_from ((*vp).name) as usize) as *mut c_char;   // C 5397
  *fill = 0;                                                            // C 5398
  return android_saf_rename_document ((*vp).tree_uri,                   // C 5399
				      (*vp).document_id,                // C 5400
				      path.as_mut_ptr (),               // C 5401
				      dst_last.offset (1));             // C 5402
}

/* C 5405-5415.  Its address is taken at C 4763.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_stat (vnode: *mut android_vnode, statb: *mut stat,
					    flags: c_int) -> c_int
{
  let mut vp: *mut android_saf_tree_vnode;                             // C 5409

  vp = vnode as *mut android_saf_tree_vnode;                           // C 5411

  return android_saf_stat ((*vp).tree_uri, (*vp).document_id,          // C 5413
			   statb, false);                               // C 5414
}

/* C 5417-5434.  Its address is taken at C 4764.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_access (vnode: *mut android_vnode, mode: c_int) -> c_int
{
  let mut vp: *mut android_saf_tree_vnode;                             // C 5420

  vp = vnode as *mut android_saf_tree_vnode;                           // C 5422

  /* Validate MODE.  */                                                // C 5424

  if mode != F_OK && (mode & (W_OK | X_OK | R_OK)) == 0                 // C 5426
    {
      *errno () = EINVAL;                                              // C 5428
      return -1;                                                       // C 5429
    }

  return android_saf_access ((*vp).tree_uri, (*vp).document_id,        // C 5432
			     (mode & W_OK) != 0);                      // C 5433
}

/* C 5436-5443.  Its address is taken at C 4765.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_mkdir (vnode: *mut android_vnode, mode: mode_t) -> c_int
{
  /* Since tree vnodes represent files that already exist, return
     EEXIST.  */                                                        // C 5439-5440
  *errno () = EEXIST;                                                  // C 5441
  return -1;                                                           // C 5442
}

/* C 5445-5461.  Its address is taken at C 4766.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_chmod (vnode: *mut android_vnode, mode: mode_t,
					     flags: c_int) -> c_int
{
  /* Return EACCESS should MODE contain unusual bits besides the
     standard file access permissions.  */                             // C 5449-5450

  if (mode as c_int) & !0o777 != 0                                     // C 5452
    {
      *errno () = EACCES;                                              // C 5454
      return -1;                                                       // C 5455
    }

  /* Otherwise, no further action is necessary, as SAF nodes already
     pretend to be S_IRUSR | S_IWUSR.  */                              // C 5458-5459
  return 0;                                                            // C 5460
}

/* C 5463-5471.  Its address is taken at C 4767.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_readlink (vnode: *mut android_vnode, buffer: *mut c_char,
						size: size_t) -> ssize_t
{
  /* Return EINVAL.  Symlinks aren't exposed to clients by the
     SAF.  */                                                           // C 5467-5468
  *errno () = EINVAL;                                                  // C 5469
  return -1;                                                           // C 5470
}

/* Open a database Cursor containing each directory entry within the
   supplied SAF tree vnode VP.

   Value is NULL upon failure with errno set to a suitable value, a
   local reference to the Cursor object otherwise.  */                  // C 5473-5477

/* C 5479-5529.  C calls it directly (C 5745), never through a pointer, so it stays a Rust-ABI
   `unsafe fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_tree_opendir_1 (vp: *mut android_saf_tree_vnode) -> jobject
{
  let mut uri: jobject;                                                // C 5482
  let mut id: jobject;                                                 // C 5482
  let mut cursor: jobject;                                             // C 5482
  let mut method: jmethodID;                                           // C 5483

  /* C leaves all four uninitialised (PORTING.md 1.5); each is assigned before its first read.  */
  uri = std::ptr::null_mut ();
  id = std::ptr::null_mut ();
  cursor = std::ptr::null_mut ();
  method = std::ptr::null_mut ();

  if inside_saf_critical_section                                       // C 5485
    {
      *errno () = EIO;                                                 // C 5487
      return std::ptr::null_mut ();                                    // C 5488
    }

  /* Build strings for both URI and ID.  */                             // C 5491
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 5492
							 (*vp).tree_uri);   // C 5493
  android_exception_check ();                                          // C 5494

  if ! (*vp).document_id.is_null ()                                    // C 5496
    {
      id = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 5498
							    (*vp).document_id);   // C 5499
      android_exception_check_1 (uri);                                 // C 5500
    }
  else
    { id = std::ptr::null_mut (); }                                    // C 5503

  /* Try to open the cursor.  */                                       // C 5505
  method = service_class.open_document_directory;                      // C 5506
  inside_saf_critical_section = true;                                  // C 5507
  cursor
    = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())    // C 5509
    (android_java_env,                                                 // C 5509
     emacs_service,                                                    // C 5510
     service_class.class,                                              // C 5511
     method, uri, id);                                                 // C 5512
  inside_saf_critical_section = false;                                 // C 5513

  if ! id.is_null ()                                                   // C 5515
    {
      if android_saf_exception_check (2, id, uri) != 0                 // C 5517
	{ return std::ptr::null_mut (); }                              // C 5518

      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, id);   // C 5520
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
    }
  else if android_saf_exception_check (1, uri) != 0                    // C 5522
    { return std::ptr::null_mut (); }                                  // C 5523

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 5525
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Return the resulting cursor.  */                                  // C 5527
  return cursor;                                                       // C 5528
}

/* C 5531-5626.  Its address is taken at C 5700, so it is an `unsafe extern "C" fn`.  C 5594's
   `coding.mode |= CODING_MODE_LAST_BLOCK` is the mirror's bitfield write (the `femacs/src/
   callproc.rs` 998-1001 spelling: the `coding_system` bitfields are a `bits` word the port packs
   with the shift/mask constants, PORTING.md 1.3).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_readdir (vdir: *mut android_vdir) -> *mut dirent
{
  let mut dir: *mut android_saf_tree_vdir;                             // C 5534
  static mut dirent: *mut dirent = std::ptr::null_mut ();              // C 5535
  let mut entry: jobject;                                              // C 5536
  let mut d_name: jobject;                                             // C 5536
  let mut d_type: jint;                                                // C 5537
  let mut method: jmethodID;                                           // C 5538
  let mut length: size_t;                                              // C 5539
  let mut size: size_t;                                                // C 5539
  let mut chars: *const c_char;                                        // C 5540
  let mut coding: coding_system;                                       // C 5541

  /* C leaves everything but `dir` uninitialised (PORTING.md 1.5); each is assigned before its
     first read on the path that reads it.  */
  entry = std::ptr::null_mut ();
  d_name = std::ptr::null_mut ();
  d_type = 0;
  method = std::ptr::null_mut ();
  length = 0;
  size = 0;
  chars = std::ptr::null ();
  coding = unsafe { std::mem::zeroed () };

  dir = vdir as *mut android_saf_tree_vdir;                            // C 5543

  /* Try to read one entry from the cursor.  */                         // C 5545
  method = service_class.read_directory_entry;                         // C 5546
  entry
    = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())    // C 5548
    (android_java_env,                                                 // C 5548
     emacs_service,                                                    // C 5549
     service_class.class,                                              // C 5550
     method, (*dir).cursor);                                           // C 5551
  android_exception_check ();                                          // C 5552

  /* If ENTRY is NULL, we're at the end of the directory.  */           // C 5554

  if entry.is_null ()                                                  // C 5556
    {
      xfree (entry as *mut c_void);                                    // C 5558
      entry = std::ptr::null_mut ();                                   // C 5559
      return std::ptr::null_mut ();                                    // C 5560
    }

  /* Load both fields from ENTRY.  */                                   // C 5563
  d_name = ((*(*android_java_env)).GetObjectField.unwrap ()) (android_java_env, entry,   // C 5564
							      entry_class.d_name);   // C 5565
  if d_name.is_null ()                                                 // C 5566
    {
      /* If an error transpires, d_name is set to NULL.  */            // C 5568
      ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 5569
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, entry);   // C 5570
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

      /* XXX: what would be a better error indication? */              // C 5572
      *errno () = EIO;                                                 // C 5573
      return std::ptr::null_mut ();                                    // C 5574
    }

  /* d_type is 1 if this is a directory, and 0 if it's a regular
     file.  */                                                         // C 5577-5578
  d_type = ((*(*android_java_env)).GetIntField.unwrap ()) (android_java_env, entry,   // C 5579
							   entry_class.d_type);   // C 5580
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, entry);   // C 5581
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Copy the name of the directory over.  */                           // C 5583
  chars = ((*(*android_java_env)).GetStringUTFChars.unwrap ())         // C 5584
    (android_java_env,                                                 // C 5584
     d_name as jstring,                                                // C 5585
     std::ptr::null_mut ());                                           // C 5586
  android_exception_check_nonnull (chars as *mut c_void, d_name);      // C 5587

  /* Decode this JNI string into utf-8-emacs; see
     android_vfs_convert_name for considerations regarding coding
     systems.  */                                                      // C 5589-5591
  length = strlen (chars);                                             // C 5592
  setup_coding_system (Qandroid_jni (), &raw mut coding);              // C 5593
  coding.bits = (coding.bits & ! (CODING_SYSTEM_MODE_MASK << CODING_SYSTEM_MODE_SHIFT))
    | ((CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT);   // C 5594
  coding.source = chars as *const u8;                                  // C 5595
  coding.dst_bytes = 0;                                                // C 5596
  coding.destination = std::ptr::null_mut ();                          // C 5597
  decode_coding_object (&raw mut coding, Qnil (), 0, 0, length as ptrdiff_t,   // C 5598
			length as ptrdiff_t, Qnil ());                 // C 5598

  /* Release the string data and the local reference to STRING.  */     // C 5600
  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ())             // C 5601
    (android_java_env,                                                 // C 5601
     d_name as jstring,                                                // C 5602
     chars);                                                           // C 5603

  /* Resize dirent to accommodate the decoded text.  */                 // C 5605
  size   = std::mem::offset_of! (dirent, d_name) + 1 + coding.produced as usize;   // C 5606
  dirent = xrealloc (dirent as *mut c_void, size) as *mut dirent;      // C 5607

  /* Clear dirent.  */                                                 // C 5609
  memset (dirent as *mut c_void, 0, size);                             // C 5610

  /* Fill in the generic directory information and copy the string
     over.  */                                                         // C 5612-5613
  (*dirent).d_ino = 0;                                                 // C 5614
  (*dirent).d_off = 0;                                                 // C 5615
  (*dirent).d_reclen = size as c_ushort;                               // C 5616
  (*dirent).d_type = if d_type != 0 { DT_DIR } else { DT_UNKNOWN } as c_uchar;   // C 5617
  memcpy ((*dirent).d_name.as_mut_ptr () as *mut c_void,               // C 5618
	  coding.destination as *const c_void, coding.produced as usize);   // C 5618
  (*dirent).d_name[coding.produced as usize] = 0;                      // C 5619

  /* Free the coding system destination buffer.  */                     // C 5621
  xfree (coding.destination as *mut c_void);                           // C 5622

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, d_name);   // C 5624
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  return dirent;                                                       // C 5625
}

/* C 5628-5662.  Its address is taken at C 5701; the `for (next = …; (tem = *next);)` loop is the
   load-then-test pair.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_closedir (vdir: *mut android_vdir)
{
  let mut dir: *mut android_saf_tree_vdir;                             // C 5631
  let mut next: *mut *mut android_saf_tree_vdir;                       // C 5631
  let mut tem: *mut android_saf_tree_vdir;                             // C 5631

  dir = vdir as *mut android_saf_tree_vdir;                            // C 5633

  /* dir->name is allocated by asprintf, which uses regular
     malloc.  */                                                       // C 5635-5636
  free ((*dir).name as *mut c_void);                                   // C 5637

  /* Yes, DIR->cursor is a local reference.  */                         // C 5639
  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,   // C 5640
						     (*dir).cursor,    // C 5641
						     cursor_class.close);   // C 5642
  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 5643
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, (*dir).cursor);   // C 5644
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* If the ``directory file descriptor'' has been opened, close
     it.  */                                                           // C 5646-5647
  if (*dir).fd != -1                                                   // C 5648
    { close ((*dir).fd); }                                             // C 5649

  /* Now unlink this directory.  */                                    // C 5651

  next = &raw mut all_saf_tree_vdirs;                                  // C 5653
  loop
    {
      tem = *next;                                                     // C 5653
      if tem.is_null ()
	{ break; }
      if tem == dir                                                    // C 5655
	{ *next = (*dir).next; }                                       // C 5656
      else
	{ next = &raw mut (**next).next; }                             // C 5658
    }

  xfree (dir as *mut c_void);                                          // C 5661
}

/* C 5664-5685.  Its address is taken at C 5702.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_dirfd (vdir: *mut android_vdir) -> c_int
{
  let mut dir: *mut android_saf_tree_vdir;                             // C 5667

  dir = vdir as *mut android_saf_tree_vdir;                            // C 5669

  /* Since `android_saf_tree_opendir' tries to avoid opening a file
     descriptor if readdir isn't called, dirfd can fail if open fails.

     open sets errno to a set of errors different from what POSIX
     stipulates for dirfd, but for ease of implementation the open
     errors are used instead.  */                                      // C 5671-5676

  if (*dir).fd >= 0                                                    // C 5678
    { return (*dir).fd; }                                              // C 5679

  (*dir).fd = open (c"/dev/null".as_ptr (), O_RDONLY | O_CLOEXEC);     // C 5681
  return (*dir).fd;                                                    // C 5682
}

/* C 5686-5759.  Its address is taken at C 4768, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_tree_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  let mut vp: *mut android_saf_tree_vnode;                             // C 5688
  let mut dir: *mut android_saf_tree_vdir;                             // C 5689
  let mut fill: *mut c_char;                                           // C 5690
  let mut end: *mut c_char;                                            // C 5690
  let mut cursor: jobject;                                             // C 5691
  let mut component: [c_char; EMACS_PATH_MAX as usize] = [0; EMACS_PATH_MAX as usize];   // C 5692

  /* C leaves `vp`, `dir`, `fill`, `end` and `cursor` uninitialised (PORTING.md 1.5); each is
     assigned before its first read on the path that reads it.  */
  vp = std::ptr::null_mut ();
  dir = std::ptr::null_mut ();
  fill = std::ptr::null_mut ();
  end = std::ptr::null_mut ();
  cursor = std::ptr::null_mut ();

  vp = vnode as *mut android_saf_tree_vnode;                           // C 5694

  /* First, fill the directory stream with the right functions and
     file name.  */                                                    // C 5696-5697

  dir = xmalloc (std::mem::size_of::<android_saf_tree_vdir> ()) as *mut android_saf_tree_vdir;   // C 5699
  (*dir).vdir.readdir
    = Some (std::mem::transmute::<unsafe extern "C" fn (*mut android_vdir) -> *mut dirent,
				  unsafe extern "C" fn (*mut android_vdir) -> *mut c_void>
	    (android_saf_tree_readdir));                               // C 5700
  (*dir).vdir.closedir = Some (android_saf_tree_closedir);             // C 5701
  (*dir).vdir.dirfd = Some (android_saf_tree_dirfd);                   // C 5702

  /* Derive the authority from the URI.  */                             // C 5704

  fill = (*vp).tree_uri as *mut c_char;                                // C 5706

  if strncmp (fill, c"content://".as_ptr (), 10) != 0                  // C 5708
    { emacs_abort (); }                                                // C 5709

  /* Skip the content header.  */
  fill = fill.offset ((std::mem::size_of::<[c_char; 11]> () - 1) as isize);   // C 5712

  /* The authority segment of the URI is between here and the
     next slash.  */                                                   // C 5714-5715

  end = strchr (fill, b'/' as c_int);                                  // C 5717

  if end.is_null ()                                                    // C 5719
    { emacs_abort (); }                                                // C 5720

  if end.offset_from (fill) >= EMACS_PATH_MAX as isize                 // C 5722
    {
      *errno () = ENAMETOOLONG;                                        // C 5724
      xfree (dir as *mut c_void);                                      // C 5725
      return std::ptr::null_mut ();                                    // C 5726
    }

  /* Copy the authority over.  */                                      // C 5729

  memcpy (component.as_mut_ptr () as *mut c_void, fill as *const c_void,   // C 5731
	  end.offset_from (fill) as usize);                            // C 5731
  component[end.offset_from (fill) as usize] = 0;                      // C 5732

  if asprintf (&raw mut (*dir).name, c"/content/storage/%s/%s%s".as_ptr (),   // C 5734
	       component.as_mut_ptr (), (*vp).tree_id, (*vp).name) < 0    // C 5735
    {
      /* Out of memory.  */                                            // C 5737
      xfree (dir as *mut c_void);                                      // C 5738
      memory_full (0);                                                 // C 5739
    }

  /* Now open a cursor that iterates through each file in this
     directory.  */                                                    // C 5742-5743

  cursor = android_saf_tree_opendir_1 (vp);                            // C 5745

  if cursor.is_null ()                                                 // C 5747
    {
      xfree ((*dir).name as *mut c_void);                              // C 5749
      xfree (dir as *mut c_void);                                      // C 5750
      return std::ptr::null_mut ();                                    // C 5751
    }

  (*dir).cursor = cursor;                                              // C 5754
  (*dir).fd = -1;                                                      // C 5755
  (*dir).next = all_saf_tree_vdirs;                                    // C 5756
  all_saf_tree_vdirs = dir;                                            // C 5757
  return &raw mut (*dir).vdir;                                         // C 5758
}

/* C 5761-5846 - `android_saf_tree_from_name` - is **deferred** (the note above C 4897): its C 5833
   writes `root.vnode.ops = &saf_tree_vfs_ops`, and that object is one of the items waiting on the
   file/new families' vectors.  C calls it directly (C 3807), never through a pointer.  */

/* Return any open SAF tree directory stream for which dirfd has
   returned the file descriptor DIRFD.  Return NULL otherwise.  */     // C 5848-5849

/* C 5851-5863.  C calls it directly (C 7380's sibling), never through a pointer, so it stays a
   Rust-ABI `unsafe fn`; the `for (dir = …; dir; dir = dir->next)` loop is the load-then-test pair.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_saf_tree_get_directory (dirfd: c_int) -> *mut android_saf_tree_vdir
{
  let mut dir: *mut android_saf_tree_vdir;                             // C 5854

  dir = all_saf_tree_vdirs;                                            // C 5856
  while ! dir.is_null ()
    {
      if (*dir).fd == dirfd && dirfd != -1                             // C 5858
	{ return dir; }                                                // C 5859
      dir = (*dir).next;                                               // C 5856
    }

  return std::ptr::null_mut ();                                        // C 5862
}

/* C 5865-5874: C 5866's form feed, the comment `/* SAF file vnode.  …  */` and C 5871-5872's
   comment with the alias definition `#define android_saf_file_vnode android_saf_tree_vnode`.  The
   alias is the port's `type` alias (PORTING.md 1.3; the two structs are the same in C), and it
   stands with the SAF file family.  */

/* C 5541's `setup_coding_system` (coding.h 686) and C 5734's `asprintf`, whose declarations this
   unit does not carry (coding.h's is another unit's function and `<stdio.h>`'s is the gnulib
   entry `lib/include/stdio.rs` does not bring in), so both stand at the use site (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
  pub fn setup_coding_system (arg0: Lisp_Object, arg1: *mut coding_system);
  /// C: `<stdio.h>`: `int asprintf (char **, const char *, ...);`
  pub fn asprintf (arg0: *mut *mut c_char, arg1: *const c_char, ...) -> c_int;
}

/* Structure describing an open ParcelFileDescriptor.  */               // C 5876

/* C 5878-5892: `struct android_parcel_fd'.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_parcel_fd
{
  /* The next open parcel file descriptor.  */
  pub next: *mut android_parcel_fd,                                    // C 5881

  /* Global reference to this parcel file descriptor.  */
  pub descriptor: jobject,                                             // C 5884

  /* The modification time of this parcel file descriptor, or
     `invalid_timespec'.  */
  pub mtime: timespec,                                                 // C 5888

  /* The file descriptor itself.  */
  pub fd: c_int,                                                       // C 5891
}
/* Measured with the NDK's clang (`-Xclang -fdump-record-layouts-complete`,
   `--target=aarch64-linux-android24`; `struct timespec` is 16 bytes there).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::size_of::<android_parcel_fd> () == 40);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_parcel_fd, next) == 0);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_parcel_fd, descriptor) == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_parcel_fd, mtime) == 16);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (core::mem::offset_of! (android_parcel_fd, fd) == 32);

/* C 5894-5900: C's five forward declarations of the SAF file members; no Rust text
   (PORTING.md 10.10).  */

/* Vector of VFS operations associated with SAF tree VFS nodes.  */     // C 5902

/* C 5904-5919: `static struct android_vops saf_file_vfs_ops = { … }` - the object C 4773 declares
   as a tentative definition, whose initialiser is here.  Its thirteen members are half the SAF file
   family's and half the SAF tree family's, and both are in this unit by the time this object is
   written, so - unlike `afs_vfs_ops`/`authority_vfs_ops` - it needs no out-of-order landing.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut saf_file_vfs_ops: android_vops = android_vops
  {
    name: Some (android_saf_file_name),
    open: Some (android_saf_file_open),
    close: Some (android_saf_tree_close),
    unlink: Some (android_saf_file_unlink),
    symlink: Some (android_saf_tree_symlink),
    rmdir: Some (android_saf_file_rmdir),
    rename: Some (android_saf_tree_rename),
    stat: Some (android_saf_tree_stat),
    access: Some (android_saf_tree_access),
    mkdir: Some (android_saf_tree_mkdir),
    chmod: Some (android_saf_tree_chmod),
    readlink: Some (android_saf_tree_readlink),
    opendir: Some (android_saf_file_opendir),
  };

/* Chain of all parcel file descriptors currently open.  */             // C 5921
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut open_parcel_fds: *mut android_parcel_fd = std::ptr::null_mut ();   // C 5922

/* C 5871-5874: C's `/* Define `struct android_saf_file_vnode' to be identical to a file vnode.  */
   #define android_saf_file_vnode android_saf_tree_vnode`.  The alias is a Rust `type` alias
   (PORTING.md 1.3), and it stands here with the family that uses it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub type android_saf_file_vnode = android_saf_tree_vnode;

/* C 5924-5950.  Its address is taken at C 5906, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_file_name (vnode: *mut android_vnode, name: *mut c_char,
					    length: size_t) -> *mut android_vnode
{
  let mut vp: *mut android_saf_file_vnode;                             // C 5928

  /* C leaves `vp` uninitialised (PORTING.md 1.5); it is assigned before any read.  */
  vp = std::ptr::null_mut ();

  /* If LENGTH is empty, make a copy of this vnode and return it.  */    // C 5930

  if length < 1                                                        // C 5932
    {
      vp = xmalloc (std::mem::size_of::<android_saf_file_vnode> ()) as *mut android_saf_file_vnode;   // C 5934
      memcpy (vp as *mut c_void, vnode as *const c_void,               // C 5935
	      std::mem::size_of::<android_saf_file_vnode> ());         // C 5935

      /* Duplicate the information contained within VNODE.  */          // C 5937

      (*vp).tree_uri = xstrdup ((*vp).tree_uri) as *const c_char;       // C 5939
      (*vp).tree_id = xstrdup ((*vp).tree_id);                          // C 5940
      (*vp).name = xstrdup ((*vp).name);                                // C 5941
      (*vp).document_id = xstrdup ((*vp).name);                         // C 5942

      return &raw mut (*vp).vnode;                                      // C 5944
    }

  /* A file vnode has no children of its own.  */                       // C 5947
  *errno () = ENOTDIR;                                                  // C 5948
  return std::ptr::null_mut ();                                         // C 5949
}

/* C 5952-6106.  Its address is taken at C 5907.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_file_open (vnode: *mut android_vnode, flags: c_int,
					    mode: mode_t, asset_p: bool, fd_return: *mut c_int,
					    asset: *mut *mut AAsset) -> c_int
{
  let mut vp: *mut android_saf_file_vnode;                             // C 5957
  let mut uri: jobject;                                                // C 5958
  let mut id: jobject;                                                 // C 5958
  let mut descriptor: jobject;                                         // C 5958
  let mut method: jmethodID;                                           // C 5959
  let mut read: jboolean;                                              // C 5960
  let mut trunc: jboolean;                                             // C 5960
  let mut write: jboolean;                                             // C 5960
  let mut fd: jint;                                                    // C 5961
  let mut info: *mut android_parcel_fd;                                // C 5962
  let mut statb: stat;                                                 // C 5963

  /* C leaves every local uninitialised (PORTING.md 1.5); each is assigned before its first read on
     the path that reads it.  */
  vp = std::ptr::null_mut ();
  uri = std::ptr::null_mut ();
  id = std::ptr::null_mut ();
  descriptor = std::ptr::null_mut ();
  method = std::ptr::null_mut ();
  read = 0;
  trunc = 0;
  write = 0;
  fd = 0;
  info = std::ptr::null_mut ();
  statb = unsafe { std::mem::zeroed () };

  if inside_saf_critical_section                                       // C 5965
    {
      *errno () = EIO;                                                 // C 5967
      return -1;                                                       // C 5968
    }

  /* O_APPEND isn't supported as a consequence of Android content
     providers defaulting to truncating the file.  */                  // C 5971-5972

  if (flags & O_APPEND) != 0                                           // C 5974
    {
      *errno () = EOPNOTSUPP;                                          // C 5976
      return -1;                                                       // C 5977
    }

  /* Build strings for both the URI and ID.  */                         // C 5980

  vp = vnode as *mut android_saf_file_vnode;                           // C 5982
  uri = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 5983
							 (*vp).tree_uri);   // C 5984
  android_exception_check ();                                          // C 5985
  id = ((*(*android_java_env)).NewStringUTF.unwrap ()) (android_java_env,   // C 5986
							(*vp).document_id);   // C 5987
  android_exception_check_1 (uri);                                     // C 5988

  /* Open a parcel file descriptor according to flags.  Documentation
     for the SAF openDocument operation is scant and seldom helpful.
     From observations made, it is clear that their file access modes
     are inconsistently implemented, and that at least:

       r   = either an FIFO or a real file, without truncation.
       w   = either an FIFO or a real file, with OR without truncation.
       wt  = either an FIFO or a real file, with truncation.
       rw  = a real file, without truncation.
       rwt = a real file, with truncation.

     This diverges from the self-contradicting documentation, where
     openDocument says nothing about truncation, and openFile mentions
     that w can elect not to truncate and programs which rely on
     truncation should use wt.

     Since Emacs is prepared to handle FIFOs within fileio.c, simply
     specify the straightforward relationship between FLAGS and the
     file access modes listed above.  */                              // C 5990-6008

  method = service_class.open_document;                                // C 6010
  read = 0;                                                            // C 6011
  trunc = 0;                                                           // C 6011
  write = 0;                                                           // C 6011

  if (flags & O_RDWR) == O_RDWR || (flags & O_WRONLY) != 0             // C 6013
    { write = 1; }                                                     // C 6014

  if (flags & O_TRUNC) != 0                                            // C 6016
    { trunc = 1; }                                                     // C 6017

  if (flags & O_RDWR) == O_RDWR || write == 0                          // C 6019
    { read = 1; }                                                      // C 6020

  inside_saf_critical_section = true;                                  // C 6022
  descriptor
    = ((*(*android_java_env)).CallNonvirtualObjectMethod.unwrap ())    // C 6024
    (android_java_env,                                                 // C 6024
     emacs_service,                                                    // C 6025
     service_class.class,                                              // C 6026
     method, uri, id,                                                  // C 6027
     (read) as c_int, (write) as c_int, (trunc) as c_int);             // C 6028 (C's variadic
									//  promotion of the three
									//  `jboolean`s)
  inside_saf_critical_section = false;                                 // C 6029

  if android_saf_exception_check (2, uri, id) != 0                     // C 6031
    { return -1; }                                                     // C 6032

  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, uri);   // C 6034
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, id);   // C 6035
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  if descriptor.is_null ()                                             // C 6037
    {
      /* Assume that permission has been denied if DESCRIPTOR cannot
	 be opened.  */                                                // C 6039-6040
      *errno () = EPERM;                                               // C 6041
      return -1;                                                       // C 6042
    }

  /* Allocate a record for this file descriptor.  Parcel file
     descriptors should be closed using their own `close' function,
     which takes care of notifying the source that it has been
     closed.  */                                                       // C 6045-6048
  info = xmalloc (std::mem::size_of::<android_parcel_fd> ()) as *mut android_parcel_fd;   // C 6049

  /* Now obtain the file descriptor.  */                               // C 6051
  fd = ((*(*android_java_env)).CallIntMethod.unwrap ()) (android_java_env,   // C 6052
							 descriptor,   // C 6053
							 fd_class.get_fd);   // C 6054
  android_exception_check_1 (descriptor);                              // C 6055

  /* Create a global reference to descriptor.  */                      // C 6057
  (*info).descriptor
    = ((*(*android_java_env)).NewGlobalRef.unwrap ()) (android_java_env,   // C 6059
						       descriptor);    // C 6060

  if (*info).descriptor.is_null ()                                     // C 6062
    {
      /* If the global reference can't be created, delete
	 descriptor.  */                                               // C 6064-6065
      ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6066
      ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,   // C 6067
							 descriptor,   // C 6068
							 fd_class.close);   // C 6069
      ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6070
      ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, descriptor);   // C 6071
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

      /* Free INFO.  */                                               // C 6073
      xfree (info as *mut c_void);                                    // C 6074

      /* Set errno to EMFILE and return.  */                          // C 6076
      *errno () = EMFILE;                                             // C 6077
      return -1;                                                      // C 6078
    }

  /* Delete the local ref to DESCRIPTOR.  */                           // C 6081
  ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, descriptor);   // C 6082
									//  (C's
									//  ANDROID_DELETE_LOCAL_REF)

  /* Try to retrieve the modification time of this file from the
     content provider.

     Refrain from introducing the file status into the file status
     cache if FLAGS & O_RDWR or FLAGS & O_WRONLY: the cached file
     status will contain a size and modification time inconsistent
     with the result of any modifications that later transpire.  */   // C 6084-6090

  if android_saf_stat ((*vp).tree_uri, (*vp).document_id,             // C 6092
		       &raw mut statb, write != 0) == 0                // C 6093
    { (*info).mtime = get_stat_mtime (&raw mut statb); }              // C 6094
  else
    { (*info).mtime = invalid_timespec (); }                          // C 6096

  /* Set info->fd and chain it onto the list.  */                      // C 6098
  (*info).fd = fd;                                                     // C 6099
  (*info).next = open_parcel_fds;                                      // C 6100
  open_parcel_fds = info;                                              // C 6101

  /* Return the file descriptor.  */                                   // C 6103
  *fd_return = fd;                                                     // C 6104
  return 0;                                                            // C 6105
}

/* C 6108-6117.  Its address is taken at C 5909.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_file_unlink (vnode: *mut android_vnode) -> c_int
{
  let mut vp: *mut android_saf_file_vnode;                             // C 6111

  vp = vnode as *mut android_saf_file_vnode;                           // C 6113
  return android_saf_delete_document ((*vp).tree_uri,                  // C 6114
				      (*vp).document_id,               // C 6115
				      (*vp).name);                     // C 6116
}

/* C 6119-6124.  Its address is taken at C 5911.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_file_rmdir (vnode: *mut android_vnode) -> c_int
{
  *errno () = ENOTDIR;                                                 // C 6122
  return -1;                                                           // C 6123
}

/* C 6126-6131.  Its address is taken at C 5918.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_saf_file_opendir (vnode: *mut android_vnode) -> *mut android_vdir
{
  *errno () = ENOTDIR;                                                 // C 6129
  return std::ptr::null_mut ();                                        // C 6130
}

/* Close FD if it's a parcel file descriptor and return true.
   If FD isn't, return false.

   Such file descriptors need to be closed using a function
   written in Java, to tell the sender that it has been
   closed.  */                                                         // C 6133-6138

/* C 6140-6170.  C calls it directly (C 7624/7660), never through a pointer, so it stays a Rust-ABI
   `unsafe fn`; the `for (next = …; (tem = *next);)` loop is the load-then-test pair, and C's
   `else` step is the same device the closedir loops use.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_close_parcel_fd (fd: c_int) -> bool
{
  let mut tem: *mut android_parcel_fd;                                 // C 6143
  let mut next: *mut *mut android_parcel_fd;                           // C 6143
  let mut temp: *mut android_parcel_fd;                                // C 6143

  next = &raw mut open_parcel_fds;                                     // C 6145
  loop
    {
      tem = *next;                                                     // C 6145
      if tem.is_null ()
	{ break; }
      if (*tem).fd == fd                                               // C 6147
	{
	  ((*(*android_java_env)).CallVoidMethod.unwrap ()) (android_java_env,   // C 6149
							     (*tem).descriptor,   // C 6150
							     fd_class.close);   // C 6151

	  /* Ignore exceptions for the same reason EINTR errors from
	     `close' should be ignored.  */                            // C 6153-6154
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6155
	  ((*(*android_java_env)).DeleteGlobalRef.unwrap ()) (android_java_env,   // C 6156
							      (*tem).descriptor);   // C 6157

	  temp = (*tem).next;                                          // C 6159
	  xfree (tem as *mut c_void);                                  // C 6160
	  *next = temp;                                                // C 6161

	  return true;                                                 // C 6163
	}
      else
	{ next = &raw mut (**next).next; }                             // C 6166
    }

  return false;                                                        // C 6169
}

/* C 6172-6200: C 6173's form feed, C 6174-6181's comment on the SAF ``new'' vnodes, C 6183's
   `#define android_saf_new_vnode android_saf_tree_vnode` alias (whose port spelling is a `type`
   alias, and which stands with that family's round) and C 6185-6200's twelve forward declarations
   (no Rust text).  This round carries the comment; the alias and the family land together.  */
