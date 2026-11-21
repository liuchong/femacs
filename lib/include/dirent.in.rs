// femacs: port of lib/dirent.in.h @ fd7afced403 (gnulib's <dirent.h>; PORTING.md P-26).
//
// The frozen substitution is build/emacs-ref/lib/dirent.h, and the configuration decides
// almost every arm against the system header:
//
//     $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/dirent.in.h) build/emacs-ref/lib/dirent.h
//     25,26c26,27
//     < #if @HAVE_DIRENT_H@
//     < # @INCLUDE_NEXT@ @NEXT_DIRENT_H@
//     ---
//     > #if 1
//     > # include_next <dirent.h>
//     41c42
//     < #if !@HAVE_DIRENT_H@
//     ---
//     > #if !1
//     104c105
//     < #if !@DIR_HAS_FD_MEMBER@
//     ---
//     > #if !1
//     109c110
//     < #  if @HAVE_DIRENT_H@
//     ---
//     > #  if 1
//
// (the generated file also holds the text the template's `/* … copied here.  */` comments
// stand for, with `#if 0` around closedir, opendir, readdir, rewinddir, dirfd, scandir and
// alphasort and `#if 1` around fdopendir, whose `@REPLACE_FDOPENDIR@` is 0 while
// `@HAVE_FDOPENDIR@` and `@HAVE_DECL_FDOPENDIR@` are 1.)
//
//     $ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
//         -I emacs/lib -include config.h -include dirent.h -E -dM /tmp/empty.c \
//         | grep -E '^(DT_|IFTODT|DTTOIF|_GL_)'
//     #define DT_BLK 6         #define DT_CHR 2          #define DT_DIR 4
//     #define DT_FIFO 1        #define DT_LNK 10         #define DT_REG 8
//     #define DT_SOCK 12       #define DT_UNKNOWN 0      #define DT_WHT 14
//     #define DTTOIF(dirtype) ((dirtype) << 12)
//     #define IFTODT(mode) (((mode) & 0170000) >> 12)
//     #define _GL_ATTRIBUTE_DEALLOC(f,i)      (empty)
//     #define _GL_ATTRIBUTE_MALLOC __attribute__ ((__malloc__))
//     #define _GL_ATTRIBUTE_PURE __attribute__ ((__pure__))
//     #define _GL_DT_NOTDIR 0x100
//
// So `@HAVE_DIRENT_H@` and `@DIR_HAS_FD_MEMBER@` are 1, which makes C 41-51's fallback
// `struct dirent` and C 104-116's `struct gl_directory`/`DIR` dead; the system defines the
// nine `DT_*` values (the very ones the template's fallbacks spell) and `IFTODT`/`DTTOIF`,
// so those `#ifndef` fallbacks are dead as well; and `@GNULIB_FDOPENDIR@`'s live arm is a
// `_GL_CXXALIAS_SYS` - the system entry `dirent.rs` declares.
//
// The port has no `#include_next`, so this mirror carries both halves: the system entries
// (`dirent.rs`, plus `attribute.rs` for the three `_GL_ATTRIBUTE_*` names C 118-149 defines,
// which the generated header takes from attribute.h) and the template's own text - the live
// `DT_*` values, and the dead arms as P-51 text under a `#[cfg]` that names the C condition
// which is not the frozen side.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] dirent.rs imported via super
// [U1] attribute.rs imported via super

/* C 29-30: the double-inclusion guard, `_@GUARD_PREFIX@_DIRENT_H`, whose frozen substitution
   is `_GL_DIRENT_H`.  The template's own spelling reads as `_` to the extractor, which
   docs/ports/excluded-macros.txt records.  */
pub const _GL_DIRENT_H: c_int = 1;

/* ---- C 41-51: the `#if !@HAVE_DIRENT_H@` fallback (dead: that substitution is 1) --------- */

/* C 44-48's minimal `struct dirent`.  The frozen target's own record is the system's
   (`src/dired.rs` measures it - 1048 bytes - and declares it at its use site); this arm is
   what a target without `<dirent.h>` gets.  */
#[cfg (feature = "no_have_dirent_h")]
pub struct dirent
{
  pub d_type: core::ffi::c_char,
  pub d_name: [core::ffi::c_char; 1],
}
/* C 49's witness.  */
#[cfg (feature = "no_have_dirent_h")]
pub const GNULIB_defined_struct_dirent: c_int = 1;

/* ---- C 60-101: the `d_type` values ------------------------------------------------------ */

/* All nine are live, and the system's values are the ones C's fallbacks spell
   (`darwin-arm64.macros.tsv` 800-808), so one ungated constant answers for both arms - the
   `fcntl.in.rs` `FD_CLOEXEC` shape.  */
pub const DT_UNKNOWN: c_int = 0;   // C 61
pub const DT_FIFO: c_int = 1;      // C 64
pub const DT_CHR: c_int = 2;       // C 67
pub const DT_DIR: c_int = 4;       // C 70
pub const DT_BLK: c_int = 6;       // C 73
pub const DT_REG: c_int = 8;       // C 76
pub const DT_LNK: c_int = 10;      // C 79
pub const DT_SOCK: c_int = 12;     // C 82
pub const DT_WHT: c_int = 14;      // C 85
/* C 102's `_GL_DT_NOTDIR 0x100` is live and stands in `dirent.rs`, the half of this header
   the port's units already include.  */

/* ---- C 104-116: the `#if !@DIR_HAS_FD_MEMBER@` fallback (dead: that substitution is 1) --- */

/* C 108's `struct gl_directory;`, C 110's `#  define DIR struct gl_directory` (`@HAVE_DIRENT_H@`
   is 1 there, so the `# define` arm is the one C records) and C 114's witness.  The live
   `DIR` is `dirent.rs`'s opaque enum; a macro and a type live in different namespaces, which
   is what lets both spellings stand.  */
#[cfg (feature = "no_dir_has_fd_member")]
pub enum gl_directory {}
#[cfg (feature = "no_dir_has_fd_member")]
macro_rules! DIR
{
  () => { gl_directory }
}
#[cfg (feature = "no_dir_has_fd_member")]
pub const GNULIB_defined_DIR: c_int = 1;   // C 114

/* ---- C 156-338: the `@GNULIB_*@` function blocks ---------------------------------------- */

/* Every one of them is dead except fdopendir's (C 307's `#if 1`).  Each block's live arm is
   a `_GL_CXXALIAS_SYS` (or, for the dead ones, would be) - the system entry `dirent.rs`
   declares - and C's `#   define closedir rpl_closedir` rename is the text carried here.  */
#[cfg (feature = "replace_closedir")]
macro_rules! closedir { ($dirp:expr) => { rpl_closedir ($dirp) } }
#[cfg (feature = "replace_closedir")]
pub const GNULIB_defined_closedir: c_int = 1;   // C 164
#[cfg (feature = "replace_opendir")]
macro_rules! opendir { ($dir_name:expr) => { rpl_opendir ($dir_name) } }
#[cfg (feature = "replace_opendir")]
pub const GNULIB_defined_opendir: c_int = 1;   // C 187
#[cfg (feature = "replace_readdir")]
macro_rules! readdir { ($dirp:expr) => { rpl_readdir ($dirp) } }
#[cfg (feature = "replace_rewinddir")]
macro_rules! rewinddir { ($dirp:expr) => { rpl_rewinddir ($dirp) } }
#[cfg (feature = "replace_dirfd")]
macro_rules! dirfd { ($dirp:expr) => { rpl_dirfd ($dirp) } }
#[cfg (feature = "replace_fdopendir")]
macro_rules! fdopendir { ($fd:expr) => { rpl_fdopendir ($fd) } }
/* C 340-364's `scandir` and C 366-384's `alphasort` blocks define no name of their own:
   their `# if !@HAVE_SCANDIR@`/`# if !@HAVE_ALPHASORT@` arms are `_GL_FUNCDECL_SYS`
   declarations and their live arms `_GL_CXXALIAS_SYS_CAST` aliases.  */

/* ---- C 392-423: the `S_IF*`/`DT_*` conversions ------------------------------------------ */

/* C 392's `#if ! (defined IFTODT && defined DTTOIF)` is false here (the system's `<dirent.h>`
   defines both), so C 393-403 - the `<sys/stat.h>` include and the two helpers - is dead text.
   Each helper has two arms: `darwin-arm64.macros.tsv` 3658/3628 defines both `S_ISWHT` and
   `S_IFWHT`, so the `#ifdef` arms are the ones this target's preprocessor would pick, while
   the definition `tools/cinventory.py` records is the `#else` one (C 397/402), which is what
   C's text ends with.  Both arms are carried, one name with one `#[cfg]` each (the
   `acl-internal.rs` shape).  */
#[cfg (feature = "no_iftodt_or_dttoif")]
#[cfg (feature = "s_iswht")]
macro_rules! _GL_DIRENT_S_ISWHT { ($mode:expr) => { S_ISWHT ($mode) } }   // C 395
#[cfg (feature = "no_iftodt_or_dttoif")]
#[cfg (not (feature = "s_iswht"))]
macro_rules! _GL_DIRENT_S_ISWHT { ($mode:expr) => { 0 } }                 // C 397
#[cfg (feature = "no_iftodt_or_dttoif")]
#[cfg (feature = "s_ifwht")]
macro_rules! _GL_DIRENT_S_IFWHT { () => { S_IFWHT } }                     // C 400
#[cfg (feature = "no_iftodt_or_dttoif")]
#[cfg (not (feature = "s_ifwht"))]
macro_rules! _GL_DIRENT_S_IFWHT { () => { DT_WHT << 12 } }                // C 402

/* C 406-413's `# ifndef IFTODT` fallback: dead (the system's is
   `(mode) (((mode) & 0170000) >> 12)`, which is `dirent.rs`'s macro).  C's chain is carried
   under P-51; it names the `S_IS*` macros `sys_stat.in.rs` spells.  */
#[cfg (feature = "no_iftodt")]
macro_rules! IFTODT
{
  ($mode:expr) => {
    if S_ISREG ($mode) { DT_REG }
    else if S_ISDIR ($mode) { DT_DIR }
    else if S_ISLNK ($mode) { DT_LNK }
    else if S_ISBLK ($mode) { DT_BLK }
    else if S_ISCHR ($mode) { DT_CHR }
    else if S_ISFIFO ($mode) { DT_FIFO }
    else if S_ISSOCK ($mode) { DT_SOCK }
    else if _GL_DIRENT_S_ISWHT ($mode) { DT_WHT }
    else { DT_UNKNOWN }
  }
}

/* C 414-423's `# ifndef DTTOIF` fallback: dead the same way.  Nothing in the port names the
   system's `DTTOIF`, so this chain is its only spelling here.  `dirtype` is C's parameter
   name (C 419 writes it without the parentheses of its siblings).  */
#[cfg (feature = "no_dttoif")]
macro_rules! DTTOIF
{
  ($dirtype:expr) => {
    if (($dirtype) == DT_REG) { S_IFREG }
    else if (($dirtype) == DT_DIR) { S_IFDIR }
    else if (($dirtype) == DT_LNK) { S_IFLNK }
    else if (($dirtype) == DT_BLK) { S_IFBLK }
    else if (($dirtype) == DT_CHR) { S_IFCHR }
    else if (($dirtype) == DT_FIFO) { S_IFIFO }
    else if (($dirtype) == DT_SOCK) { S_IFSOCK }
    else if (($dirtype) == DT_WHT) { _GL_DIRENT_S_IFWHT () }
    else { (($dirtype) << 12) }
  }
}

/* The template's remaining text has no name to carry: C 32-36's `_GL_CONFIG_H_INCLUDED`
   check, C 38-39's `<sys/types.h>` (the port reaches it through `dirent.rs`), C 53-59's
   `d_type` comment, C 87-99's `static_assert` chain, C 101/118-120/128-131/140-145/352-353/
   374-375/387-390's comments and C 425-427's guard pair.  */
