// femacs: port of lib/sys_types.in.h @ fd7afced403 (gnulib).
//
// gnulib template.  The frozen target's generated header is
// build/emacs-ref/lib/sys/types.h, whose substitutions are the guard `_GL_SYS_TYPES_H`,
// `@WINDOWS_64_BIT_OFF_T@` = 0, `@HAVE_OFF64_T@` = 1, `@WINDOWS_STAT_INODES@` = 0,
// `@HAVE_BLKSIZE_T@` = 1 and `@HAVE_BLKCNT_T@` = 1.  The port has neither `#include_next`
// nor m4 placeholders, so - like the endian/byteswap/sys_random `.in.rs` mirrors - this file
// carries both halves the template spells:
//
//   * the system half: the types the frozen target's <sys/types.h> (the file the template
//     includes next) provides.  Every block that would define a type is dead text in the
//     generated header (the substitutions above spell `#if 0` / `#if !1` there), so the
//     aliases below are the *system* header's types, measured with the frozen compiler;
//   * the template's own half: the guard, the `_GL_INCLUDING_SYS_TYPES_H` marker C defines
//     around the include-next, and the `GNULIB_defined_*` / Windows-rename macros of the
//     blocks.  Those are dead text in the generated header too; they are carried because
//     `tools/extract-abi.py` reads this template blind (P-66: a `@PLACEHOLDER@` condition
//     cannot be evaluated, so both arms of every block count as required names) and because
//     the mirror is the whole header.  Each item below names the arm it mirrors.

// [U1-MODULE-MIRROR]
use super::*;
/* C 41/48-49: the double-inclusion guard, `_@GUARD_PREFIX@_SYS_TYPES_H`; the frozen
   substitution is `_GL_SYS_TYPES_H` (build/emacs-ref/lib/sys/types.h 41).  Under the
   template's own spelling the extractor reads the name as `_`, which
   docs/ports/excluded-macros.txt records.  */
pub const _GL_SYS_TYPES_H: c_int = 1;

/* C 44-46: `# define _GL_INCLUDING_SYS_TYPES_H` / `# undef _GL_INCLUDING_SYS_TYPES_H`
   around C 45's `#include_next <sys/types.h>`.  The port has no include-next, so the name
   has no use site; it is carried as the value C's preprocessor holds between the two
   lines.  */
pub const _GL_INCLUDING_SYS_TYPES_H: c_int = 1;

/* ---- the include-next half: the frozen target's <sys/types.h> (PORTING.md P-26) --------
 *
 * `lib/sys_stat.in.rs` needs `struct stat`'s field types, and C gets every one of them from
 * <sys/types.h> on this target: a probe that includes only <sys/types.h> and prints
 * `sizeof` for each compiles, and prints
 *
 *     $ cat > /tmp/tprobe.c <<'EOF'
 *     #include <sys/types.h>
 *     #include <stdio.h>
 *     int main(void){
 *       mode_t m = 1; nlink_t n = 1; blksize_t b = 1; blkcnt_t c = 1; ino_t i = 1; dev_t d = 1; off_t o = 1;
 *       printf("%zu %zu %zu %zu %zu %zu %zu\n", sizeof m, sizeof n, sizeof b, sizeof c, sizeof i, sizeof d, sizeof o);
 *       return 0; }
 *     EOF
 *     $ clang -o /tmp/tprobe /tmp/tprobe.c && /tmp/tprobe
 *     2 2 4 8 8 4 8
 *
 * i.e. mode_t and nlink_t are 16-bit, blksize_t and dev_t 32-bit, blkcnt_t and ino_t 64-bit
 * and off_t (below) 64-bit - the widths the aliases below carry.  The template's own type
 * blocks (`typedef long long off64_t`, `rpl_dev_t`/`rpl_ino_t`, `typedef int blksize_t`,
 * `typedef long long blkcnt_t`) are `#if !1`/`#if 0` in the generated header, so their text
 * is not the frozen arms; the `GNULIB_defined_*` markers they set are carried below.
 * `off64_t` itself is *not* declared here: no frozen translation unit names it, and the
 * system <sys/types.h> does not define it (PORTING.md 0.1-2, "only what is named").  */

/// The frozen target's `off_t` (a 64-bit signed integer).
pub type off_t = i64;
/* The frozen target's `off64_t` has no frozen arm (C 63-70 is `#if !1`); the alias is
   deliberately absent - see the note above.  */
/// The frozen target's `pid_t` — `process.h` 160 uses it as `Lisp_Process.pid`.  The frozen
/// target's `<sys/types.h>` makes it a 32-bit signed integer (`sizeof (pid_t) == 4`, measured).
pub type pid_t = c_int;
/* sys/types.h: the group id type gnulib's group-member.c and getgroups.c take.  */
pub type gid_t = c_uint;
/* sys/types.h + gnulib: the type getgroups.c's groups come back in (gid_t on this target).  */
pub type GETGROUPS_T = gid_t;
/// The frozen target's `mode_t` (`__uint16_t`, measured above).
/* C's `<sys/types.h>` differs per target and the difference is not cosmetic: `struct stat`,
   `mkdir`, `chmod`, `umask` and `mknod` all take these types, so a host-shaped alias shifts a
   record field or truncates an argument.  Measured with the NDK for aarch64-linux-android21
   (`char a[sizeof (mode_t)];` … read back with `llvm-nm -S`): `dev_t` 8, `mode_t` 4, `nlink_t` 4,
   `blksize_t` 8 (status 4.794).  */
#[cfg (not (feature = "have_android"))]
pub type mode_t = u16;
#[cfg (feature = "have_android")]
pub type mode_t = c_uint;
/// The frozen target's `nlink_t` (`__uint16_t`, measured above).
#[cfg (not (feature = "have_android"))]
pub type nlink_t = u16;
#[cfg (feature = "have_android")]
pub type nlink_t = c_uint;
/// The frozen target's `blksize_t` (`__int32_t`, measured above).
#[cfg (not (feature = "have_android"))]
pub type blksize_t = c_int;
#[cfg (feature = "have_android")]
pub type blksize_t = i64;
/// The frozen target's `blkcnt_t` (`__int64_t`, measured above).
pub type blkcnt_t = i64;
/// The frozen target's `ino_t` (`__uint64_t`, measured above).
pub type ino_t = u64;
/// The frozen target's `dev_t` (`__int32_t`, measured above).
#[cfg (not (feature = "have_android"))]
pub type dev_t = c_int;
#[cfg (feature = "have_android")]
pub type dev_t = u64;

/* The four aliases bionic spells differently, asserted so `tools/check-android-layout.py` can
   compare them with the NDK's own answers (the host's are 2/2/4/4).  */
#[cfg (feature = "have_android")]
const _: () = core::assert! (core::mem::size_of::<dev_t> () == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (core::mem::size_of::<mode_t> () == 4);
#[cfg (feature = "have_android")]
const _: () = core::assert! (core::mem::size_of::<nlink_t> () == 4);
#[cfg (feature = "have_android")]
const _: () = core::assert! (core::mem::size_of::<blksize_t> () == 8);
/// The frozen target's `uid_t` (`__uint32_t`; `struct stat`'s `st_uid`).
pub type uid_t = c_uint;

/* ---- the template's own half: the blocks C guards with placeholders ------------------- */

/* C 51-61's `#if @WINDOWS_64_BIT_OFF_T@` arm: the frozen substitution is 0, so the whole
   arm is dead (`#if 0` in the generated header) and this target's `off_t` is the system's
   (the alias above).  The arm's non-MSVC `#define off_t long long int` is kept under the
   feature that names the non-frozen side (P-51); the name is one the blind extractor asks
   for.  (`_MSC_VER`'s sibling spelling is `__int64`; the macro has several definitions, so
   the extractor does not compare its body.)  */
#[cfg (feature = "windows_64_bit_off_t")]
macro_rules! off_t { () => { long long int } }

/* C 59-60: the arm's indicator.  Dead here (see above), kept as the value C's preprocessor
   defines in that arm.  */
pub const _GL_WINDOWS_64_BIT_OFF_T: c_int = 1;

/* C 63-70's `#if !@HAVE_OFF64_T@` block: @HAVE_OFF64_T@ is 1 on this target, so the
   `typedef long long off64_t` is dead text (`#if !1`) and the type comes from the system
   header (which on this target does not define it, so no unit may name it).  The guard
   macro C sets is carried.  */
pub const GNULIB_defined_off64_t: c_int = 1;

/* C 72-112's `#if @WINDOWS_STAT_INODES@` arm: the frozen substitution is 0, so it is dead
   (`#if 0` in the generated header).  The two renames it defines
   (`# define dev_t rpl_dev_t`, `# define ino_t rpl_ino_t`) are its text and are kept under
   the feature naming the non-frozen side (P-51); the two marker macros are carried as the
   values C's preprocessor defines in that arm, and `_GL_WINDOWS_STAT_INODES` keeps the
   template's placeholder token as its body (the m4 substitution is the 0 the generated
   header spells, so a value constant would lose the token the template writes).  */
#[cfg (feature = "windows_stat_inodes")]
macro_rules! dev_t { () => { rpl_dev_t } }
#[cfg (feature = "windows_stat_inodes")]
macro_rules! ino_t { () => { rpl_ino_t } }
pub const GNULIB_defined_dev_t: c_int = 1;
pub const GNULIB_defined_ino_t: c_int = 1;
#[cfg (feature = "windows_stat_inodes")]
macro_rules! _GL_WINDOWS_STAT_INODES { () => { WINDOWS_STAT_INODES } }

/* C 114-118's `#if (defined _WIN32 && ! defined __CYGWIN__) && ! defined __GLIBC__` arm
   (`#include <stddef.h>`): dead on this target.  */

/* C 120-126's `#if !@HAVE_BLKSIZE_T@` block: @HAVE_BLKSIZE_T@ is 1, so the `typedef int
   blksize_t` is dead (`#if !1`) and the type is the system's alias above - which spells the
   same width.  The guard macro C sets is carried.  */
pub const GNULIB_defined_blksize_t: c_int = 1;

/* C 128-134's `#if !@HAVE_BLKCNT_T@` block: @HAVE_BLKCNT_T@ is 1, so the
   `typedef long long blkcnt_t` is dead (`#if !1`) and the type is the system's alias above.
   The guard macro C sets is carried.  */
pub const GNULIB_defined_blkcnt_t: c_int = 1;

/* C 41-138 close with three `#endif`s (the two split guards and the `__need_XXX` arm); the
   port has no preprocessor, so they have no counterpart (PORTING.md 0.1).  */
