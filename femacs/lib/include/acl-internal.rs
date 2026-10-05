// femacs: port of lib/acl-internal.h @ fd7afced403 (gnulib).
//
// Liveness: `tools/check-cond-eval.py lib/acl-internal.h` reports 36 conditions with **1
// disagreement** (C 63's `#ifndef ACL_INTERNAL_INLINE`: clang sees the macro already defined,
// because lib/acl-internal.c 20 does `#define ACL_INTERNAL_INLINE _GL_EXTERN_INLINE` before
// including this header, while the frozen macro table has no such entry), so the file is
// compared *blindly*: both branches of every condition stay, and the checklist asks for all
// 40 macros and every `permission_context` member of *every* configuration arm, not just the
// frozen one.
//
// The frozen configuration: USE_ACL is 0, so the whole C 67-238 block is dead; HAVE_SYS_ACL_H,
// HAVE_FACL, HAVE_ACL_LIBACL_H, HAVE_ACLV_H, HAVE_GETACL, HAVE_STATACL, HAVE_ACLSORT,
// HAVE_ACL_GET_FILE, HAVE_ACL_GET_FD, HAVE_ACL_SET_FD, HAVE_ACL_FREE_TEXT,
// HAVE_ACL_EXTENDED_FILE, HAVE_ACL_FROM_MODE, HAVE_ACL_FROM_TEXT, HAVE_ACL_ENTRIES,
// HAVE_ACL_TYPE_EXTENDED, HAVE_ACL_COPY_EXT_NATIVE, HAVE_ACL_CREATE_ENTRY_NP and ACE_GETACL
// are all undefined, and HAVE_FCHMOD is 1 (which kills C 57-60's `#ifndef` arm).
//
// Every dead arm is kept as text under a feature that names the non-frozen side (PORTING.md
// P-51).  Those features are *recorded* text: xtask's FLIPS list carries no `USE_ACL` entry
// and `src/include/config.rs` has none of these ACL constants (the curated list stops short of
// them), so nothing compiles the arms — which is what the target does with them as well.  The
// arms name their conditions' other side (`use_acl_have_acl_get_file`, `use_acl_getacl`,
// `use_acl_have_getacl`, `use_acl_have_statacl`, `use_acl_have_aclsort`, `use_acl_ace_getacl`,
// `have_facl`, `no_fchmod`, `have_aclv_h`); the struct's mutually exclusive members carry
// their own arm's feature so that no two `count`/`entries` members can ever coexist.  Types
// the C's other-platform headers supply and this target has not got (`aclent_t`, `ace_t`,
// `struct acl`, `struct acl_entry`, `NACLENTRIES`, `NACLVENTRIES`, `struct acl_entry`) are
// written in C's spelling inside those dead arms, exactly as C's own dead arms name them.
//
// What the frozen configuration keeps is `ACL_INTERNAL_INLINE`, `struct permission_context`'s
// `mode` member and the three prototypes at the end (C 280-282).

/* C 25: `#include "acl.h"`.  */
include!("acl.rs");
/* C 27: `<stdlib.h>`.  */
include!("stdlib.rs");
/* `mode_t`, which the struct below names.  */
include!("sys_types.in.rs");
/* `<sys/acl.h>`: C 29-32's `#if HAVE_SYS_ACL_H` include is dead on this target (the macro is
   undefined), but the dead `USE_ACL` arm below names its types, so the mirror is the header
   this file would see.  (Each `include!` sits on a line of its own: xtask rewrites the line
   it finds an `include!` on, PORTING.md 0.1-4.)  */
include!("sys_acl.rs");
/* C 52: `<errno.h>`.  */
include!("errno.rs");
/* C 54: `<limits.h>`.  */
include!("limits.in.rs");
/* C 55: `<stdint.h>`.  */
include!("stdint.in.rs");

/// C 20-23: `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

/* C 33-35: `#if defined HAVE_FACL && ! defined GETACLCNT && defined ACL_CNT` — dead, HAVE_FACL
   is undefined on this target; its `ACL_CNT` comes from the Solaris <sys/acl.h>.  */
#[cfg (feature = "have_facl")]
macro_rules! GETACLCNT { () => { ACL_CNT }; }

/* C 37-41's `#ifdef HAVE_ACL_LIBACL_H` include (`<acl/libacl.h>`, Linux) is dead, and this
   target has no such header to mirror.  C 43-50's `#if HAVE_ACLV_H` arm (HP-UX) declares the
   two functions below; `<aclv.h>` and `struct acl` do not exist here, so the arm is recorded
   as text.  */
#[cfg (feature = "have_aclv_h")]
unsafe extern "C" {
  /* int acl (char *, int, int, struct acl *);
     int aclsort (int, int, struct acl *);  */
  pub fn acl (arg1: *mut c_char, arg2: c_int, arg3: c_int, arg4: *mut acl) -> c_int;
  pub fn aclsort (arg1: c_int, arg2: c_int, arg3: *mut acl) -> c_int;
}

/* C 57-60: `#ifndef HAVE_FCHMOD` is dead (the configuration defines HAVE_FCHMOD as 1, so C
   keeps the system `fchmod`); the arm's two macros are kept as text.  */
#[cfg (feature = "no_fchmod")]
pub const HAVE_FCHMOD: bool = false;
#[cfg (feature = "no_fchmod")]
macro_rules! fchmod { ($fd:expr, $mode:expr) => { -1 }; }

/* C 62/284's `_GL_INLINE_HEADER_BEGIN`/`_GL_INLINE_HEADER_END` are `extern "C" {`/`}` for C++
   only (docs/ports/excluded-macros.txt).  */

/* C 63-65: `#ifndef ACL_INTERNAL_INLINE / # define ACL_INTERNAL_INLINE _GL_INLINE`.  The
   wrapper unit lib/acl-internal.c defines the macro itself (to `_GL_EXTERN_INLINE`) before
   including this header, which is why clang's `#ifndef` is false there; the name is therefore
   in the whole-import module below, so that unit's own definition shadows it (PORTING.md
   0.1-10, the strnul.c shape).  */

pub mod acl_internal_decls {
  use super::*;

  /* C 63-65's arm for every other translation unit.  */
  pub const ACL_INTERNAL_INLINE: () = _GL_INLINE;

  /* C 280-282: the three entry points the ACL units define.  */
  unsafe extern "C" {
    pub fn get_permissions (arg1: *const c_char, arg2: c_int, arg3: mode_t,
                            arg4: *mut permission_context) -> c_int;
    pub fn set_permissions (arg1: *mut permission_context, arg2: *const c_char,
                            arg3: c_int) -> c_int;
    pub fn free_permission_context (arg1: *mut permission_context);
  }
}
use acl_internal_decls::*;

/* ------------------------------------------------------------------
   C 67-238: `#if USE_ACL` — dead on this target (USE_ACL is 0).  The text below keeps the
   C's arms, each under the feature that names the non-frozen side of its own condition.
   ------------------------------------------------------------------ */

/* C 69's `# if HAVE_ACL_GET_FILE`: POSIX 1003.1e draft-17 systems (Linux, FreeBSD, macOS,
   Cygwin >= 2.5).  */

/* C 73-75.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const MIN_ACL_ENTRIES: c_int = 4;

/* POSIX 1003.1e (draft 17) */
/* C 78-84: `#ifdef HAVE_ACL_GET_FD` (one argument) `#else` the replacement below.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const HAVE_ACL_GET_FD: bool = false;
#[cfg (feature = "use_acl_have_acl_get_file")]
macro_rules! acl_get_fd { ($fd:expr) => { std::ptr::null_mut () }; }

/* POSIX 1003.1e (draft 17) */
/* C 86-93: `#ifdef HAVE_ACL_SET_FD` (two arguments) `#else` the replacement below.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const HAVE_ACL_SET_FD: bool = false;
#[cfg (feature = "use_acl_have_acl_get_file")]
macro_rules! acl_set_fd { ($fd:expr, $acl:expr) => { -1 }; }

/* POSIX 1003.1e (draft 13) */
/* C 95-98: `# if ! HAVE_ACL_FREE_TEXT`.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
macro_rules! acl_free_text { ($buf:expr) => { acl_free ($buf) }; }

/* Linux-specific */
/* Cygwin >= 2.5 implements acl_extended_file(), but it returns 1 for nearly all
   directories --- for reasons explained in
   <https://sourceware.org/pipermail/cygwin/2025-March/257762.html> ---, thus is
   unusable.  For the user, 'ls' should not print a '+' sign, indicating the
   presence of an ACL, for 99,9% of the files; this would not be useful.
   Therefore, on Cygwin, we ignore the acl_extended_file function and instead
   use our own acl_access_nontrivial function.  */
/* C 100-112: `# if !defined HAVE_ACL_EXTENDED_FILE || defined __CYGWIN__`.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const HAVE_ACL_EXTENDED_FILE: bool = false;
#[cfg (feature = "use_acl_have_acl_get_file")]
macro_rules! acl_extended_file { ($name:expr) => { -1 }; }

/* C 114-116: `# if ! defined HAVE_ACL_FROM_MODE && ! defined HAVE_ACL_FROM_TEXT` — an
   object-like macro in C (`# define acl_from_mode (NULL)`), so the port keeps it as the value
   C's own text makes it.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const acl_from_mode: *mut acl_t = std::ptr::null_mut ();

/* Set to 0 if a file's mode is stored independently from the ACL.  */
/* C 118-121: `# if HAVE_ACL_COPY_EXT_NATIVE && HAVE_ACL_CREATE_ENTRY_NP` (macOS).  */
#[cfg (feature = "use_acl_have_acl_get_file")]
pub const MODE_INSIDE_ACL: c_int = 0;

/* Return the number of entries in ACL.
   Return -1 and set errno upon failure to determine it.  */
/* Define a replacement for acl_entries if needed. (Only Linux has it.)  */
/* C 123-129: `# if !HAVE_ACL_ENTRIES` — the replacement is `rpl_acl_entries`, which the units
   declare and define.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
macro_rules! acl_entries { () => { rpl_acl_entries } }

/* C 131-148: `# if HAVE_ACL_TYPE_EXTENDED` (macOS) `# else` the two access/default
   predicates.  */
/* ACL is an ACL, from a file, stored as type ACL_TYPE_EXTENDED.
   Return 1 if the given ACL is non-trivial.
   Return 0 if it is trivial.  */
/* ACL is an ACL, from a file, stored as type ACL_TYPE_ACCESS.
   Return 1 if the given ACL is non-trivial.
   Return 0 if it is trivial, i.e. equivalent to a simple stat() mode.
   Return -1 and set errno upon failure to determine it.  */
/* ACL is an ACL, from a file, stored as type ACL_TYPE_DEFAULT: the same, for the file's
   default ACL.  */
#[cfg (feature = "use_acl_have_acl_get_file")]
unsafe extern "C" {
  pub fn acl_extended_nontrivial (acl: acl_t) -> c_int;
  pub fn acl_access_nontrivial (acl: acl_t) -> c_int;
  pub fn acl_default_nontrivial (acl: acl_t) -> c_int;
}

/* C 150's `# elif HAVE_FACL && defined GETACL`: Solaris and Cygwin < 2.5.  */

/* C 152-155: `#  if defined __CYGWIN__`.  **C's `#if`/`#elif` makes this arm the other side of
   C 118-121**, so the port's gate says the same thing: a configuration cannot be macOS's
   `HAVE_ACL_GET_FILE` and Cygwin's `HAVE_FACL` at once, and opening both features without the
   negation is an arm that is not a configuration (status 3.510 measured its `E0428`).  */
#[cfg (all (feature = "use_acl_getacl", not (feature = "use_acl_have_acl_get_file")))]
pub const MODE_INSIDE_ACL: c_int = 0;

/* Return 1 if the given ACL is non-trivial.
   Return 0 if it is trivial, i.e. equivalent to a simple stat() mode.  */
/* C 157-159.  */
#[cfg (feature = "use_acl_getacl")]
unsafe extern "C" {
  pub fn acl_nontrivial (count: c_int, entries: *const aclent_t) -> c_int;
}

/* Test an ACL retrieved with ACE_GETACL.
   Return 1 if the given ACL, consisting of COUNT entries, is non-trivial.
   Return 0 if it is trivial, i.e. equivalent to a simple stat() mode.  */
/* Definitions for when the built executable is executed on Solaris 10
   (newer version) or Solaris 11.  */
/* C 161-199: `#  ifdef ACE_GETACL` (Solaris 10) — the ace_t predicate and, for a binary that
   runs on Solaris 10 or 11, the two generations' type/flag/mask names.  */
#[cfg (feature = "use_acl_ace_getacl")]
unsafe extern "C" {
  pub fn acl_ace_nontrivial (count: c_int, entries: *const ace_t) -> c_int;
}

#[cfg (feature = "use_acl_ace_getacl")]
pub const OLD_ALLOW: c_int = 0;
#[cfg (feature = "use_acl_ace_getacl")]
pub const OLD_DENY: c_int = 1;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_ACCESS_ALLOWED_ACE_TYPE: c_int = 0;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_ACCESS_DENIED_ACE_TYPE: c_int = 1;
#[cfg (feature = "use_acl_ace_getacl")]
pub const OLD_ACE_OWNER: c_int = 0x0100;
#[cfg (feature = "use_acl_ace_getacl")]
pub const OLD_ACE_GROUP: c_int = 0x0200;
#[cfg (feature = "use_acl_ace_getacl")]
pub const OLD_ACE_OTHER: c_int = 0x0400;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_OWNER: c_int = 0x1000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_GROUP: c_int = 0x2000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_IDENTIFIER_GROUP: c_int = 0x0040;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_EVERYONE: c_int = 0x4000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_READ_DATA: c_int = 0x001;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_WRITE_DATA: c_int = 0x002;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_APPEND_DATA: c_int = 0x004;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_READ_NAMED_ATTRS: c_int = 0x008;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_WRITE_NAMED_ATTRS: c_int = 0x010;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_EXECUTE: c_int = 0x020;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_DELETE_CHILD: c_int = 0x040;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_READ_ATTRIBUTES: c_int = 0x080;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_WRITE_ATTRIBUTES: c_int = 0x100;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_DELETE: c_int = 0x10000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_READ_ACL: c_int = 0x20000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_WRITE_ACL: c_int = 0x40000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_WRITE_OWNER: c_int = 0x80000;
#[cfg (feature = "use_acl_ace_getacl")]
pub const NEW_ACE_SYNCHRONIZE: c_int = 0x100000;

/* C 201's `# elif HAVE_GETACL`: HP-UX.  */
/* Return 1 if the given ACL is non-trivial.
   Return 0 if it is trivial, i.e. equivalent to a simple stat() mode.  */
#[cfg (feature = "use_acl_have_getacl")]
unsafe extern "C" {
  pub fn acl_nontrivial (count: c_int, entries: *const acl_entry) -> c_int;
}
/* C 207-213: `#  if HAVE_ACLV_H` (HP-UX >= 11.11).  */
#[cfg (feature = "use_acl_have_getacl")]
unsafe extern "C" {
  pub fn aclv_nontrivial (count: c_int, entries: *const acl) -> c_int;
}

/* C 215's `# elif HAVE_ACLX_GET && 0`: AIX with the ACLX API — C's `&& 0` makes it dead even
   there (`/* TODO */`).  */

/* C 219's `# elif HAVE_STATACL`: older AIX.  */
#[cfg (feature = "use_acl_have_statacl")]
unsafe extern "C" {
  pub fn acl_nontrivial (a: *mut acl) -> c_int;
}

/* C 225's `# elif HAVE_ACLSORT`: NonStop Kernel.  */
#[cfg (feature = "use_acl_have_aclsort")]
unsafe extern "C" {
  pub fn acl_nontrivial (count: c_int, entries: *const acl) -> c_int;
}

/* Set to 1 if a file's mode is implicit by the ACL.  */
/* C 233-236: `# ifndef MODE_INSIDE_ACL / #  define MODE_INSIDE_ACL 1` - the arm that follows
   every branch, so it is the one a system without its own definition keeps.  **The `#ifndef` is
   load-bearing in Rust too**: the two arms above (`use_acl_have_acl_get_file` for C 118-121's
   macOS condition and `use_acl_getacl` for C 152-155's Cygwin one) are *other* definitions of
   the same name, so an ungated copy here is `E0428` in any arm that opens either - measured by
   the `lib/file-has-acl.rs` lane (status 3.510), whose arm opening both features stopped
   rustc inside this mirror before the unit was even read (a mutation in the unit changed
   nothing, i.e. an empty reading).  */
#[cfg (not (any (feature = "use_acl_have_acl_get_file", feature = "use_acl_getacl")))]
pub const MODE_INSIDE_ACL: c_int = 1;

/* C 240-278.  The three entry points take a pointer to it, so it needs C's layout
   (PORTING.md 1.3); the members the frozen configuration keeps are only `mode`.  */
#[repr(C)]
pub struct permission_context
{
  pub mode: mode_t,
  /* `#if USE_ACL / # if HAVE_ACL_GET_FILE` (Linux, FreeBSD, macOS, Cygwin >= 2.5).  */
  #[cfg (feature = "use_acl_have_acl_get_file")]
  pub acl: acl_t,
  /* C 245-247: `#  if !HAVE_ACL_TYPE_EXTENDED`.  */
  #[cfg (feature = "use_acl_have_acl_get_file")]
  pub default_acl: acl_t,
  #[cfg (feature = "use_acl_have_acl_get_file")]
  pub acls_not_supported: bool,
  /* `# elif defined GETACL` (Solaris, Cygwin < 2.5).  */
  #[cfg (feature = "use_acl_getacl")]
  pub count: c_int,
  #[cfg (feature = "use_acl_getacl")]
  pub entries: *mut aclent_t,
  /* C 254-258: `#  ifdef ACE_GETACL` (Solaris 10).  */
  #[cfg (feature = "use_acl_ace_getacl")]
  pub ace_count: c_int,
  #[cfg (feature = "use_acl_ace_getacl")]
  pub ace_entries: *mut ace_t,
  /* `# elif HAVE_GETACL` (HP-UX).  */
  #[cfg (feature = "use_acl_have_getacl")]
  pub entries: [acl_entry; NACLENTRIES],
  #[cfg (feature = "use_acl_have_getacl")]
  pub count: c_int,
  /* C 263-266: `#  if HAVE_ACLV_H`.  */
  #[cfg (feature = "use_acl_have_getacl")]
  pub aclv_entries: [acl; NACLVENTRIES],
  #[cfg (feature = "use_acl_have_getacl")]
  pub aclv_count: c_int,
  /* `# elif HAVE_STATACL` (older AIX): `union { struct acl a; char room[4096]; } u;`.  */
  #[cfg (feature = "use_acl_have_statacl")]
  pub have_u: bool,
  /* `# elif HAVE_ACLSORT` (NonStop Kernel).  */
  #[cfg (feature = "use_acl_have_aclsort")]
  pub entries: [acl; NACLENTRIES],
  #[cfg (feature = "use_acl_have_aclsort")]
  pub count: c_int,
}

/* C 284's `_GL_INLINE_HEADER_END`.  */
