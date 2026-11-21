// femacs: port of src/filelock.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The five
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's three non-`static` definitions
// (`unlock_all_files`, `unlock_buffer`, `syms_of_filelock`) carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the fourteen `static` helpers stay plain `fn` (the value-only
// `within_one_second`) or `unsafe fn` (pointer arguments), and two of them (`unlock_file`,
// `unlock_file_handle_error`) are `unsafe extern "C" fn` because C hands their addresses to
// `internal_condition_case_1` (PORTING.md 1.4).  C's two `DEFVAR` variables
// (`Vtemporary_file_directory`, `create_lockfiles`) are defined here and shadow the
// `globals.rs` declarations instead of clashing with them (P-21).
//
// The dead regions are recorded in docs/ports/filelock.md rather than transcribed:
// `#ifdef WINDOWSNT` (C 47-51's three includes, C 161-162's `sys_rename_replace` arm, C
// 204-209's `ENOSYS` initializer), `#ifdef HAVE_ANDROID` (C 55-57's include, C 540-543's
// `char *name` and C 546-556's `/assets`/`/content` test; HAVE_ANDROID is undefined),
// `#ifdef MSDOS` (the `Ffile_locked_p` arm at C 776-777; MSDOS is undefined, so every
// `#ifndef MSDOS` region is live), `#ifndef ELOOP` (C 310-312: ELOOP is 62 in the frozen
// table) and the `#ifdef WINDOWSNT` alternative of `VALID_PROCESS_ID` (C 361-363: the
// `#ifndef` arm is live).
//
// The `#include` list is C 22-48's, through the mirrors those pull in: config.h (22),
// sys/types.h (23), sys/stat.h (24), signal.h (25), stdio.h (26), stdlib.h (27),
// `<boot-time.h>` (41), `<c-ctype.h>` (44), lisp.h (46), buffer.h (47) and coding.h (48).
// Three of C's includes have no mirror and no name this unit uses, so they are left out (the
// doc.rs `<sys/file.h>` precedent): `<pwd.h>` (C 33, live but empty for this file),
// `<sys/file.h>` (C 36) and `<errno.h>`'s mirror is `errno.rs` (which is included).
//
// C gets `emacs_fstatat`/`emacs_unlink`/`emacs_symlink`/`emacs_renameat_noreplace`/
// `emacs_rename`/`emacs_write` from lisp.h 5303-5334 and `kill`/`mkostemp`/`snprintf`/
// `strtoimax`/`memrchr`/`internal_condition_case_1` from <signal.h>/<stdlib.h>/<stdio.h>/
// <inttypes.h>/gnulib's <string.h>/lisp.h 4806; the mirrors this unit includes do not carry
// them yet, so each is declared at its use site (the terminal.rs/doc.rs shape).  `PRIdMAX`
// and `EDOM` come from <inttypes.h>/<errno.h> the same way.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/sys_types.in.rs imported via common
// [U1] ../lib/include/sys_stat.in.rs imported via common
// [U1] ../lib/include/signal.rs imported via common
// [U1] ../lib/include/fcntl.in.rs imported via common
// [U1] ../lib/include/unistd.in.rs imported via common
// [U1] ../lib/include/errno.rs imported via common
// [U1] ../lib/include/c-ctype.rs imported via common
// [U1] ../lib/include/boot-time.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/coding.rs imported via common

/// <inttypes.h>: `#define PRIdMAX "jd"` on the frozen target (`intmax_t` is `long long`;
/// measured with clang: `printf ("%s", PRIdMAX)` -> `jd`).  C concatenates it into the two
/// lock-info format strings, which Rust cannot concatenate a const into, so the port writes
/// the concatenated literals below and keeps the name here.
const PRIdMAX: &str = "jd";

/* The names C gets from lisp.h 5303-5334 and from the system headers above, declared at the
   use site (the terminal.rs shape).  `emacs_fstatat`'s third parameter is lisp.h's `void *`.  */
unsafe extern "C" {
  /// C: int emacs_fstatat (int, char const *, void *, int)
  pub fn emacs_fstatat (fd: c_int, name: *const c_char, buf: *mut c_void, flag: c_int) -> c_int;
  /// C: int emacs_unlink (const char *)
  pub fn emacs_unlink (name: *const c_char) -> c_int;
  /// C: int emacs_symlink (const char *, const char *)
  pub fn emacs_symlink (contents: *const c_char, name: *const c_char) -> c_int;
  /// C: int emacs_renameat_noreplace (int, const char *, int, const char *)
  pub fn emacs_renameat_noreplace (fd1: c_int, old: *const c_char,
                                   fd2: c_int, new: *const c_char) -> c_int;
  /// C: int emacs_rename (const char *, const char *)
  pub fn emacs_rename (old: *const c_char, new: *const c_char) -> c_int;
  /// C: ptrdiff_t emacs_write (int, void const *, ptrdiff_t)
  pub fn emacs_write (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
  /// C: int kill (pid_t, int)   -- <signal.h>
  pub fn kill (pid: pid_t, sig: c_int) -> c_int;
  /// C: int mkostemp (char *, int)   -- <stdlib.h>
  pub fn mkostemp (template: *mut c_char, flags: c_int) -> c_int;
  // `<stdio.h>`'s `int snprintf (char *, size_t, const char *, ...)` comes from
  // `lib/include/stdio.rs` (status 3.263): the mirror carries it, so this unit does not
  // declare it a second time (PORTING.md 0.1-3).
  /// C: intmax_t strtoimax (const char *, char **, int)   -- <inttypes.h>
  pub fn strtoimax (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t;
  /// C: void *memrchr (const void *, int, size_t)   -- gnulib <string.h> (HAVE_MEMRCHR undef)
  pub fn memrchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
  /// C: Lisp_Object internal_condition_case_1 (Lisp_Object (*) (Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object))
  pub fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
                                    arg: Lisp_Object, handlers: Lisp_Object,
                                    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
}

/* Normally use a symbolic link to represent a lock.
   The strategy: to lock a file FN, create a symlink .#FN in FN's
   directory, with link data USER@HOST.PID:BOOT.  This avoids a single
   mount (== failure) point for lock files.  The :BOOT is omitted if
   the boot time is not available.

   When the host in the lock data is the current host, we can check if
   the pid is valid with kill.

   Otherwise, we could look at a separate file that maps hostnames to
   reboot times to see if the remote pid can possibly be valid, since we
   don't want Emacs to have to communicate via pipes or sockets or
   whatever to other processes, either locally or remotely; rms says
   that's too unreliable.  Hence the separate file, which could
   theoretically be updated by daemons running separately -- but this
   whole idea is unimplemented; in practice, at least in our
   environment, it seems such stale locks arise fairly infrequently, and
   Emacs's standard methods of dealing with clashes suffice.

   We use symlinks instead of normal files because (1) they can be
   stored more efficiently on the filesystem, since the kernel knows
   they will be small, and (2) all the info about the lock can be read
   in a single system call (readlink).  Although we could use regular
   files to be useful on old systems lacking symlinks, nowadays
   virtually all such systems are probably single-user anyway, so it
   didn't seem worth the complication.

   Similarly, we don't worry about a possible 14-character limit on
   file names, because those are all the same systems that don't have
   symlinks.

   This is compatible with the locking scheme used by Interleaf (which
   has contributed this implementation for Emacs), and was designed by
   Karl Berry, Ethan Jacobson, Kimbo Mundy, and others.

   On some file systems, notably those of MS-Windows, symbolic links
   do not work well, so instead of a symlink .#FN -> USER@HOST.PID:BOOT,
   the lock is a regular file .#FN with contents USER@HOST.PID:BOOT.  To
   establish a lock, a nonce file is created and then renamed to .#FN.
   On MS-Windows this renaming is atomic unless the lock is forcibly
   acquired.  On other systems the renaming is atomic if the lock is
   forcibly acquired; if not, the renaming is done via hard links,
   which is good enough for lock-file purposes.

   To summarize, race conditions can occur with either:

   * Forced locks on MS-Windows systems.

   * Non-forced locks on non-MS-Windows systems that support neither
     hard nor symbolic links.  */

/* Return the time of the last system boot, or 0 if that information
   is unavailable.  */

unsafe fn get_boot_sec () -> time_t
{
  /* get_boot_time maintains static state.  Don't touch that state
     if we are going to dump, since it might not survive dumping.  */
  if will_dump_p ()
    { return 0; }

  // C declares `struct timespec boot_time` uninitialized and fills it through the pointer;
  // the port gives the C local that is written through a pointer the zero initializer the
  // ledger prescribes (PORTING.md 1.5).
  let mut boot_time: timespec = std::mem::zeroed ();
  boot_time.tv_sec = 0;
  get_boot_time (&mut boot_time);
  return boot_time.tv_sec;
}

/* An arbitrary limit on lock contents length.  8 K should be plenty
   big enough in practice.  */
pub const MAX_LFINFO: c_int = 8 * 1024;

/* Here is the structure that stores information about a lock.  */

#[repr(C)]
pub struct lock_info_type
{
  /* Location of '@', '.', and ':' (or equivalent) in USER.  If there's
     no colon or equivalent, COLON points to the end of USER.  */
  pub at: *mut c_char, pub dot: *mut c_char, pub colon: *mut c_char,

  /* Lock file contents USER@HOST.PID with an optional :BOOT_TIME
     appended.  This memory is used as a lock file contents buffer, so
     it needs room for MAX_LFINFO + 1 bytes.  A string " (pid NNNN)"
     may be appended to the USER@HOST while generating a diagnostic,
     so make room for its extra bytes (as opposed to ".NNNN") too.  */
  /* C: `char user[MAX_LFINFO + 1 + sizeof " (pid )" - sizeof "."]`; `sizeof " (pid )"` is 8
     and `sizeof "."` is 2, so the frozen target's member is 8199 bytes.  */
  pub user: [c_char; MAX_LFINFO as usize + 1 + 8 - 2],
}

/* For some reason Linux kernels return EPERM on file systems that do
   not support hard or symbolic links.  This symbol documents the quirk.
   There is no way to tell whether a symlink call fails due to
   permissions issues or because links are not supported, but luckily
   the lock file code should work either way.  */
pub const LINKS_MIGHT_NOT_WORK: c_int = EPERM;

/* Rename OLD to NEW.  If FORCE, replace any existing NEW.
   It is OK if there are temporarily two hard links to OLD.
   Return 0 if successful, -1 (setting errno) otherwise.  */
unsafe fn rename_lock_file (old: *const c_char, new: *const c_char, force: bool) -> c_int
{
  // C 161-162's `#ifdef WINDOWSNT` arm (`return sys_rename_replace (old, new, force);`) is dead
  // on this target (docs/ports/filelock.md).
  if !force
    {
      let mut st: stat = std::mem::zeroed ();

      let r: c_int = emacs_renameat_noreplace (AT_FDCWD, old,
					       AT_FDCWD, new);
      if !(r < 0 && *errno () == ENOSYS)
	{ return r; }
      if link (old, new) == 0
	{ return if emacs_unlink (old) == 0 || *errno () == ENOENT { 0 } else { -1 }; }
      if *errno () != ENOSYS && *errno () != LINKS_MIGHT_NOT_WORK
	{ return -1; }

      /* 'link' does not work on this file system.  This can occur on
	 a GNU/Linux host mounting a FAT32 file system.  Fall back on
	 'rename' after checking that NEW does not exist.  There is a
	 potential race condition since some other process may create
	 NEW immediately after the existence check, but it's the best
	 we can portably do here.  */
      if emacs_fstatat (AT_FDCWD, new, &mut st as *mut stat as *mut c_void, AT_SYMLINK_NOFOLLOW) == 0
	  || *errno () == EOVERFLOW
	{
	  *errno () = EEXIST;
	  return -1;
	}
      if *errno () != ENOENT
	{ return -1; }
    }

  return emacs_rename (old, new);
}

/* Create the lock file LFNAME with contents LOCK_INFO_STR.  Return 0 if
   successful, an errno value on failure.  If FORCE, remove any
   existing LFNAME if necessary.  */

unsafe fn create_lock_file (lfname: *mut c_char, lock_info_str: *mut c_char, force: bool) -> c_int
{
  // C 204-209's `#ifdef WINDOWSNT` arm (`int err = ENOSYS;`) is dead on this target; the live
  // initializer is the symlink attempt below.
  let mut err: c_int = if emacs_symlink (lock_info_str, lfname) == 0 { 0 } else { *errno () };

  if err == EEXIST && force
    {
      emacs_unlink (lfname);
      err = if emacs_symlink (lock_info_str, lfname) == 0 { 0 } else { *errno () };
    }

  if err == ENOSYS || err == LINKS_MIGHT_NOT_WORK || err == ENAMETOOLONG
    {
      /* C's `static char const nonce_base[]`; the port keeps C's bytes in a `[u8; 15]`
	 and casts at the two uses (the byte array is a string literal, not a char array).  */
      static nonce_base: [u8; 15] = *b".#-emacsXXXXXX\0";
      let last_slash: *mut c_char = strrchr (lfname, b'/' as c_int);
      let lfdirlen: ptrdiff_t = last_slash.offset (1) as ptrdiff_t - lfname as ptrdiff_t;
      USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
      let nonce: *mut c_char
	= SAFE_ALLOCA! (sa_avail, lfdirlen + std::mem::size_of_val (&nonce_base) as ptrdiff_t)
	  as *mut c_char;
      let fd: c_int;
      memcpy (nonce as *mut c_void, lfname as *const c_void, lfdirlen as usize);
      strcpy (nonce.offset (lfdirlen), nonce_base.as_ptr () as *const c_char);

      fd = mkostemp (nonce, O_BINARY | O_CLOEXEC);
      if fd < 0
	{ err = *errno (); }
      else
	{
	  let lock_info_len: ptrdiff_t;
	  lock_info_len = strlen (lock_info_str) as ptrdiff_t;
	  err = 0;

	  /* Make the lock file readable to others, so that others' sessions
	     can read it.  Even though nobody should write to the lock file,
	     keep it user-writable to work around problems on nonstandard file
	     systems that prohibit unlinking readonly files (Bug#37884).  */
	  if emacs_write (fd, lock_info_str as *const c_void, lock_info_len) != lock_info_len
	      || fchmod (fd, S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) != 0
	    { err = *errno (); }

	  /* There is no need to call fsync here, as the contents of
	     the lock file need not survive system crashes.  */
	  if emacs_close (fd) != 0
	    { err = *errno (); }
	  if err == 0 && rename_lock_file (nonce, lfname, force) != 0
	    { err = *errno (); }
	  if err != 0
	    { emacs_unlink (nonce); }
	}

      SAFE_FREE! (sa_count, sa_mark);
    }

  return err;
}

/* Lock the lock file named LFNAME.
   If FORCE, do so even if it is already locked.
   Return 0 if successful, an error number on failure.  */

unsafe fn lock_file_1 (lfname: Lisp_Object, force: bool) -> c_int
{
  let boot: intmax_t = get_boot_sec () as intmax_t;
  let luser_name: Lisp_Object = Fuser_login_name (Qnil ());
  let lhost_name: Lisp_Object = Fsystem_name ();
  let user_name: *const c_char = if STRINGP (luser_name) { SSDATA (luser_name) as *const c_char } else { c"".as_ptr () };
  let mut host_name: *const c_char = if STRINGP (lhost_name) { SSDATA (lhost_name) as *const c_char } else { c"".as_ptr () };
  // C leaves the buffer uninitialized and snprintf fills it; the port zero-initializes it
  // (PORTING.md 1.5).
  let mut lock_info_str: [c_char; MAX_LFINFO as usize + 1] = [0; MAX_LFINFO as usize + 1];
  let pid: intmax_t = getpid () as intmax_t;

  let mut room: c_int = std::mem::size_of_val (&lock_info_str) as c_int;
  let mut len: c_int = snprintf (lock_info_str.as_mut_ptr (), room as usize, c"%s@".as_ptr (), user_name);
  if !(0 <= len && len < std::mem::size_of_val (&lock_info_str) as c_int)
    { return ENAMETOOLONG; }
  /* Protect against the extremely unlikely case of the host name
     containing an @ character.  */
  while *host_name != 0
    {
      if !(len < std::mem::size_of_val (&lock_info_str) as c_int - 1)
	{ return ENAMETOOLONG; }
      lock_info_str[len as usize] = if *host_name == b'@' as c_char { b'-' as c_char } else { *host_name };
      len += 1;
      host_name = host_name.offset (1);
    }
  /* C: `char const *lock_info_fmt = boot ? ".%"PRIdMAX":%"PRIdMAX : ".%"PRIdMAX;` - the
     PRIdMAX macro is `"jd"` on this target (see the const above), so the two branches are
     the concatenated literals below (docs/ports/macro-body-notes.tsv, `lock_file_1`).  */
  let lock_info_fmt: *const c_char = if boot != 0 { c".%jd:%jd".as_ptr () } else { c".%jd".as_ptr () };
  room = std::mem::size_of_val (&lock_info_str) as c_int - len;
  let suffixlen: c_int = snprintf (lock_info_str.as_mut_ptr ().offset (len as isize), room as usize,
				   lock_info_fmt, pid, boot);
  if !(0 <= suffixlen && suffixlen < room)
    { return ENAMETOOLONG; }
  return create_lock_file (SSDATA (lfname), lock_info_str.as_mut_ptr (), force);
}

/* Return true if times A and B are no more than one second apart.  */

fn within_one_second (a: intmax_t, b: time_t) -> bool
{
  let diff: intmax_t;
  // C: `ckd_sub (&diff, a, b)` - gnulib's ckd_sub stores the wrapped difference and reports
  // overflow; Rust's `overflowing_sub` is that pair (P-15, docs/ports/token-aliases.tsv).
  let (ckd_sub_value, ckd_sub_overflow) = a.overflowing_sub (b);
  diff = ckd_sub_value;
  return !ckd_sub_overflow && -1 <= diff && diff <= 1;
}

/* On systems lacking ELOOP, test for an errno value that shouldn't occur.  */
// C 310-312's `#ifndef ELOOP / # define ELOOP (-1) / #endif` is dead text on this target
// (ELOOP is 62, docs/ports/filelock.md).

/* Read the data for the lock file LFNAME into LFINFO.  Read at most
   MAX_LFINFO + 1 bytes.  Return the number of bytes read, or -1
   (setting errno) on error.  */

unsafe fn read_lock_data (lfname: *mut c_char, lfinfo: *mut c_char) -> ptrdiff_t
{
  let mut nbytes: ptrdiff_t;

  while { nbytes = readlinkat (AT_FDCWD, lfname, lfinfo, (MAX_LFINFO + 1) as usize) as ptrdiff_t;
	  nbytes < 0 && *errno () == EINVAL }
    {
      let fd: c_int = emacs_open (lfname, O_RDONLY | O_NOFOLLOW, 0);
      if 0 <= fd
	{
	  let read_bytes: ptrdiff_t = emacs_read (fd, lfinfo as *mut c_void, (MAX_LFINFO + 1) as ptrdiff_t);
	  let read_errno: c_int = *errno ();
	  if emacs_close (fd) != 0
	    { return -1; }
	  *errno () = read_errno;
	  return read_bytes;
	}

      if *errno () != ELOOP
	{ return -1; }

      /* readlinkat saw a non-symlink, but emacs_open saw a symlink.
	 The former must have been removed and replaced by the latter.
	 Try again.  */
      maybe_quit ();
    }

  return nbytes;
}

/* Whether the string S starts with a decimal integer, optionally
   negative.  */
unsafe fn integer_prefixed (s: *const c_char) -> bool
{
  /* Doing it this way avoids a conditional branch on most platforms.  */
  return c_isdigit (*s.offset ((*s.offset (0) == b'-' as c_char) as isize) as c_int);
}

/* Whether the integer P could identify an individual process.  On most
   platforms this simply checks for positive pid_t, but on some
   MS-Windows ports our headers #define it to to some other test.  */
// C 361-363's `#ifdef WINDOWSNT` alternative (the other build's test) is dead on this target;
// the live `#ifndef` arm is the macro below.  `TYPE_MAXIMUM` is gnulib's <intprops.h> macro,
// which no mirror carries, so the port writes it here (the profiler.rs precedent) - it must
// precede the `VALID_PROCESS_ID!` expansion in `current_lock_owner` below (PORTING.md 0.1-18).
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

macro_rules! VALID_PROCESS_ID { ($p:expr) => { 0 < ($p) && ($p) <= TYPE_MAXIMUM! (pid_t) as intmax_t }; }

/* True if errno values are negative.  Although the C standard
   requires them to be positive, they are negative in Haiku.  */
pub const NEGATIVE_ERRNO: c_int = if EDOM < 0 { 1 } else { 0 };

/* Nonzero values that are not errno values.  */
/* Another process on this machine owns it.  */
pub const ANOTHER_OWNS_IT: c_int = if NEGATIVE_ERRNO != 0 { 1 } else { -1 };

/* This Emacs process owns it.  */
pub const I_OWN_IT: c_int = 2 * ANOTHER_OWNS_IT;

/* Return 0 if nobody owns the lock file LFNAME or the lock is obsolete,
   ANOTHER_OWNS_IT if another process owns it
     (and set OWNER (if non-null) to info),
   I_OWN_IT if the current process owns it,
   or an errno value if something is wrong with the locking mechanism.  */

unsafe fn current_lock_owner (owner: *mut lock_info_type, lfname: Lisp_Object) -> c_int
{
  let mut local_owner: lock_info_type = std::mem::zeroed ();

  /* Even if the caller doesn't want the owner info, we still have to
     read it to determine return value.  */
  let mut owner: *mut lock_info_type = owner;
  if owner.is_null ()
    { owner = &mut local_owner; }

  /* If nonexistent lock file, all is well; otherwise, got strange error. */
  let lfinfolen: ptrdiff_t = read_lock_data (SSDATA (lfname), (*owner).user.as_mut_ptr ());
  if lfinfolen < 0
    { return if *errno () == ENOENT || *errno () == ENOTDIR { 0 } else { *errno () }; }

  /* If the lock file seems valid, return a value based on its contents.  */
  if lfinfolen != 0
    {
      if (MAX_LFINFO as ptrdiff_t) < lfinfolen
	{ return ENAMETOOLONG; }
      (*owner).user[lfinfolen as usize] = 0;

      /* Parse USER@HOST.PID:BOOT_TIME.  If can't parse, return EINVAL.  */
      /* The USER is everything before the last @.  */
      let at: *mut c_char
	= memrchr ((*owner).user.as_mut_ptr () as *const c_void, b'@' as c_int,
		   lfinfolen as usize) as *mut c_char;
      if at.is_null ()
	{ return EINVAL; }
      (*owner).at = at;
      let dot: *mut c_char = strrchr (at, b'.' as c_int);
      if dot.is_null ()
	{ return EINVAL; }
      (*owner).dot = dot;

      /* The PID is everything from the last '.' to the ':' or equivalent.  */
      if !integer_prefixed (dot.offset (1))
	{ return EINVAL; }
      *errno () = 0;
      let mut pid: intmax_t = strtoimax (dot.offset (1), &mut (*owner).colon, 10);
      if *errno () == ERANGE
	{ pid = -1; }

      /* After the ':' or equivalent, if there is one, comes the boot time.  */
      // C leaves `boot_time` and `lfinfo_end` uninitialized; the port gives the two locals
      // that are written on some paths the zero initializer the ledger prescribes
      // (PORTING.md 1.5).
      let mut boot_time: intmax_t = 0;
      let mut boot: *mut c_char = (*owner).colon.offset (1);
      let mut lfinfo_end: *mut c_char = std::ptr::null_mut ();
      let mut fallthrough_colon: bool = false;
      /* C 429's `switch (owner->colon[0])`; the '\357' arm falls through into ':' (PORTING.md
	 2.2), which `match` cannot do, so the ':' body follows the arm selection.  */
      if *(*owner).colon == 0
	{
	  boot_time = 0;
	  lfinfo_end = (*owner).colon;
	}
      else if *(*owner).colon == 0o357u8 as c_char
	{
	  /* Treat "\357\200\242" (U+F022 in UTF-8) like ":" (Bug#24656).
	     This works around a bug in the Linux CIFS kernel client, which can
	     mistakenly transliterate ':' to U+F022 in symlink contents.
	     See <https://bugzilla.redhat.com/show_bug.cgi?id=1384153>.  */
	  if !(*boot.offset (0) == 0o200u8 as c_char && *boot.offset (1) == 0o242u8 as c_char)
	    { return EINVAL; }
	  boot = boot.offset (2);
	  fallthrough_colon = true;
	}
      else if *(*owner).colon == b':' as c_char
	{ fallthrough_colon = true; }
      else
	{ return EINVAL; }

      if fallthrough_colon
	{
	  if !integer_prefixed (boot)
	    { return EINVAL; }
	  boot_time = strtoimax (boot, &mut lfinfo_end, 10);
	}
      if lfinfo_end != (*owner).user.as_mut_ptr ().offset (lfinfolen)
	{ return EINVAL; }

      let linkhost: *mut c_char = at.offset (1);
      let linkhostlen: ptrdiff_t = dot as ptrdiff_t - linkhost as ptrdiff_t;
      let system_name: Lisp_Object = Fsystem_name ();
      /* If `system-name' returns nil, that means we're in a
	 --no-build-details Emacs, and the name part of the link (e.g.,
	 .#test.txt -> larsi@.118961:1646577954) is an empty string.  */
      let mut on_current_host: bool;
      if NILP (system_name)
	{ on_current_host = linkhostlen == 0; }
      else
	{
	  on_current_host = linkhostlen == SBYTES (system_name);
	  if on_current_host
	    {
	      /* Protect against the extremely unlikely case of the host
		 name containing '@'.  */
	      let sysname: *mut c_char = SSDATA (system_name);
	      let mut i: ptrdiff_t = 0;
	      while i < linkhostlen
		{
		  if *linkhost.offset (i) != (if *sysname.offset (i) == b'@' as c_char { b'-' as c_char } else { *sysname.offset (i) })
		    {
		      on_current_host = false;
		      break;
		    }
		  i += 1;
		}
	    }
	}
      if !on_current_host
	{
	  /* Not on current host.  If we wanted to support the check for
	     stale locks on remote machines, here's where we'd do it.  */
	  return ANOTHER_OWNS_IT;
	}

      if pid == getpid () as intmax_t
	{ return I_OWN_IT; }

      if VALID_PROCESS_ID! (pid)
	  && !(kill (pid as pid_t, 0) < 0 && *errno () != EPERM)
	  && (boot_time == 0
	      || within_one_second (boot_time, get_boot_sec ()))
	{ return ANOTHER_OWNS_IT; }
    }

  /* The owner process is dead or has a strange pid, or the lock file is empty.
     Try to zap the lockfile.  If the lock file is empty, this assumes
     the file system is buggy, e.g., <https://bugs.gnu.org/72641>.
     Emacs never creates empty lock files even temporarily, so removing
     an empty lock file should be harmless.  */
  return if emacs_unlink (SSDATA (lfname)) < 0 && *errno () != ENOENT { *errno () } else { 0 };
}

/* Lock the lock named LFNAME if possible.
   Return 0 in that case.
   Return ANOTHER_OWNS_IT if some other process owns the lock, and info about
     that process in CLASHER.
   Return errno value if cannot lock for any other reason.  */

unsafe fn lock_if_free (clasher: *mut lock_info_type, lfname: Lisp_Object) -> c_int
{
  let mut err: c_int;
  while { err = lock_file_1 (lfname, false); err == EEXIST }
    {
      err = current_lock_owner (clasher, lfname);

      /* Return if we locked it, or another process owns it, or it is
	 a strange error.  */
      if err != 0
	{ return if err == I_OWN_IT { 0 } else { err }; }

      /* We deleted a stale lock or some other process deleted the lock;
	 try again to lock the file.  */
    }

  return err;
}

/* Return the encoded name of the lock file for FN, or nil if none.  */

unsafe fn make_lock_file_name (mut r#fn: Lisp_Object) -> Lisp_Object
{
  let lock_file_name: Lisp_Object;
  // C 540-543's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` declares `char *name`
  // and C 546-556 is its body (`android_is_special_directory` over "/assets"/"/content");
  // HAVE_ANDROID is undefined on this target (docs/ports/filelock.md).

  r#fn = Fexpand_file_name (r#fn, Qnil ());

  lock_file_name = calln! (Qmake_lock_file_name (), r#fn);

  return if !NILP (lock_file_name) { ENCODE_FILE (lock_file_name) } else { Qnil () };
}

/* lock_file locks file FN,
   meaning it serves notice on the world that you intend to edit that file.
   This should be done only when about to modify a file-visiting
   buffer previously unmodified.
   Do not (normally) call this for a buffer already modified,
   as either the file is already locked, or the user has already
   decided to go ahead without locking.

   When this returns, either the lock is locked for us,
   or lock creation failed,
   or the user has said to go ahead without locking.

   If the file is locked by someone else, this calls
   ask-user-about-lock (a Lisp function) with two arguments,
   the file name and info about the user who did the locking.
   This function can signal an error, or return t meaning
   take away the lock, or return nil meaning ignore the lock.  */

unsafe fn lock_file (r#fn: Lisp_Object) -> Lisp_Object
{
  let mut lock_info: lock_info_type = std::mem::zeroed ();

  /* Don't do locking while dumping Emacs.
     Uncompressing wtmp files uses call-process, which does not work
     in an uninitialized Emacs.  */
  if will_dump_p ()
    { return Qnil (); }

  let mut lfname: Lisp_Object = Qnil ();
  if create_lockfiles
    {
      /* Create the name of the lock-file for file fn */
      lfname = make_lock_file_name (r#fn);
      if NILP (lfname)
	{ return Qnil (); }
    }

  /* See if this file is visited and has changed on disk since it was
     visited.  */
  let subject_buf: Lisp_Object = Fget_truename_buffer (r#fn);
  if !NILP (subject_buf)
      && NILP (Fverify_visited_file_modtime (subject_buf))
      && !NILP (Ffile_exists_p (r#fn))
      && !(!NILP (lfname) && current_lock_owner (std::ptr::null_mut (), lfname) == I_OWN_IT)
    { calln! (Quserlock__ask_user_about_supersession_threat (), r#fn); }

  /* Don't do locking if the user has opted out.  */
  if !NILP (lfname)
    {
      /* Try to lock the lock.  FIXME: This ignores errors when
	 lock_if_free returns an errno value.  */
      if lock_if_free (&mut lock_info, lfname) == ANOTHER_OWNS_IT
	{
	  /* Someone else has the lock.  Consider breaking it.  */
	  let attack: Lisp_Object;
	  let dot: *mut c_char = lock_info.dot;
	  let pidlen: ptrdiff_t = lock_info.colon as ptrdiff_t - dot.offset (1) as ptrdiff_t;
	  static replacement: [u8; 7] = *b" (pid \0";
	  let replacementlen: c_int = std::mem::size_of_val (&replacement) as c_int - 1;
	  memmove (dot.offset (replacementlen as isize) as *mut c_void,
		   dot.offset (1) as *const c_void, pidlen as usize);
	  strcpy (dot.offset (replacementlen as isize + pidlen), c")".as_ptr ());
	  memcpy (dot as *mut c_void, replacement.as_ptr () as *const c_void,
		  replacementlen as usize);
	  attack = calln! (Qask_user_about_lock (), r#fn,
			  build_string (lock_info.user.as_ptr ()));
	  /* Take the lock if the user said so.  */
	  if !NILP (attack)
	    { lock_file_1 (lfname, true); }
	}
    }
  return Qnil ();
}

unsafe extern "C" fn unlock_file (r#fn: Lisp_Object) -> Lisp_Object
{
  let lfname: Lisp_Object = make_lock_file_name (r#fn);
  if NILP (lfname)
    { return Qnil (); }

  let mut err: c_int = current_lock_owner (std::ptr::null_mut (), lfname);
  if !(err == 0 || err == ANOTHER_OWNS_IT
	|| (err == I_OWN_IT
	    && (emacs_unlink (SSDATA (lfname)) == 0
		|| { err = *errno (); err == ENOENT })))
    { report_file_errno (c"Unlocking file".as_ptr (), r#fn, err); }

  return Qnil ();
}

unsafe extern "C" fn unlock_file_handle_error (err: Lisp_Object) -> Lisp_Object
{
  calln! (Quserlock__handle_unlock_error (), err);
  return Qnil ();
}

// C 660's `#endif` closes the `#ifndef MSDOS` guard whose text above is this target's.

/* C's two DEFVAR variables live in the generated globals.h in C; the port's globals.rs
   mirrors them as declarations and the owning unit defines them (P-21, the undo.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vtemporary_file_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut create_lockfiles: bool = false;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlock_all_files ()
{
  let mut tail: Lisp_Object;
  let mut buf: Lisp_Object;
  let mut b: *mut buffer;

  /* C: `FOR_EACH_LIVE_BUFFER (tail, buf)`, i.e. `FOR_EACH_ALIST_VALUE (Vbuffer_alist, tail,
     buf)` = `for ((tail) = (Vbuffer_alist); (CONSP (tail) && ((buf) = XCDR (XCAR (tail)),
     true)); (tail) = XCDR (tail))`.  The loop macro has no Rust counterpart yet (P-22), so
     the expansion is written here (the ITREE_FOREACH precedent, P-33); the `tail`/`buf` names
     and the statement order are C's, and the test's assignment is the `while` condition.  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      b = XBUFFER (buf);
      if STRINGP (BVAR! (b, file_truename_))
	  && BUF_SAVE_MODIFF! (b) < BUF_MODIFF! (b)
	{ Funlock_file (BVAR! (b, file_truename_)); }
      tail = XCDR (tail);
    }
}

DEFUN! ("lock-file", Flock_file, Slock_file, 1, 1, 0,
       doc: "Check whether FILE was modified since it was visited, and lock it.\nIf user option `create-lockfiles' is nil, this does not create\na lock file for FILE, but it still checks whether FILE was modified\noutside of the current Emacs session, and if so, asks the user\nwhether to modify FILE.",
       (file: Lisp_Object),
       {
  // C 685's `#ifndef MSDOS` and C 696's `#endif` are live here: MSDOS is undefined,
  // so the guarded text below is this target's, and only the dead `#else` arm would be C's.
  CHECK_STRING (file);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object;
  handler = Ffind_file_name_handler (file, Qlock_file ());
  if !NILP (handler)
    { return calln! (handler, Qlock_file (), file); }

  lock_file (file);
  return Qnil ();
});

DEFUN! ("unlock-file", Funlock_file, Sunlock_file, 1, 1, 0,
       doc: "Unlock FILE.",
       (file: Lisp_Object),
       {
  CHECK_STRING (file);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object;
  handler = Ffind_file_name_handler (file, Qunlock_file ());
  if !NILP (handler)
    {
      calln! (handler, Qunlock_file (), file);
      return Qnil ();
    }

  internal_condition_case_1 (Some (unlock_file),
			     file,
			     list1 (Qfile_error ()),
			     Some (unlock_file_handle_error));
  return Qnil ();
});

DEFUN! ("lock-buffer", Flock_buffer, Slock_buffer,
       0, 1, 0,
       doc: "Lock FILE, if current buffer is modified.\nFILE defaults to current buffer's visited file,\nor else nothing is done if current buffer isn't visiting a file.\n\nIf the option `create-lockfiles' is nil, this does nothing.",
       (file: Lisp_Object),
       {
  let mut file = file;   // C's `file = BVAR (current_buffer, file_truename);` below rebinds the argument
  if NILP (file)
    { file = BVAR! ((current_buffer! ()), file_truename_); }
  else
    { CHECK_STRING (file); }
  if SAVE_MODIFF! () < MODIFF! ()
      && !NILP (file)
    { Flock_file (file); }
  return Qnil ();
});

DEFUN! ("unlock-buffer", Funlock_buffer, Sunlock_buffer,
       0, 0, 0,
       doc: "Unlock the file visited in the current buffer.\nIf the buffer is not modified, this does nothing because the file\nshould not be locked in that case.  It also does nothing if the\ncurrent buffer is not visiting a file, or is not locked.  Handles file\nsystem errors by calling `display-warning' and continuing as if the\nerror did not occur.",
       (),
       {
  if SAVE_MODIFF! () < MODIFF! ()
      && STRINGP (BVAR! ((current_buffer! ()), file_truename_))
    { Funlock_file (BVAR! ((current_buffer! ()), file_truename_)); }
  return Qnil ();
});

/* Unlock the file visited in buffer BUFFER.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlock_buffer (buffer: *mut buffer)
{
  if BUF_SAVE_MODIFF! (buffer) < BUF_MODIFF! (buffer)
      && STRINGP (BVAR! (buffer, file_truename_))
    { Funlock_file (BVAR! (buffer, file_truename_)); }
}

DEFUN! ("file-locked-p", Ffile_locked_p, Sfile_locked_p, 1, 1, 0,
       doc: "Return a value indicating whether FILENAME is locked.\nThe value is nil if the FILENAME is not locked,\nt if it is locked by you, else a string saying which user has locked it.",
       (filename: Lisp_Object),
       {
  // C 776-777's `#ifdef MSDOS` arm (`return Qnil;`) is dead on this target; the `#else` body
  // below is the live one.
  let ret: Lisp_Object;
  let owner: c_int;
  let mut locker: lock_info_type = std::mem::zeroed ();

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object;
  handler = Ffind_file_name_handler (filename, Qfile_locked_p ());
  if !NILP (handler)
    {
      return calln! (handler, Qfile_locked_p (), filename);
    }

  let lfname: Lisp_Object = make_lock_file_name (filename);
  if NILP (lfname)
    { return Qnil (); }

  owner = current_lock_owner (&mut locker, lfname);
  match owner
    {
      I_OWN_IT => { ret = Qt (); }

      ANOTHER_OWNS_IT =>
	{
	  ret = make_string (locker.user.as_ptr (),
			     locker.at as ptrdiff_t - locker.user.as_ptr () as ptrdiff_t);
	}

      0 => { ret = Qnil (); }

      _ => { report_file_errno (c"Testing file lock".as_ptr (), filename, owner); }
    }

  return ret;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_filelock ()
{
  DEFVAR_LISP! (c"temporary-file-directory", Vtemporary_file_directory,
		"The directory for writing temporary files.");
  Vtemporary_file_directory = Qnil ();

  DEFVAR_BOOL! (c"create-lockfiles", create_lockfiles,
		"Non-nil means use lockfiles to avoid editing collisions.\nThe name of the (per-buffer) lockfile is constructed by prepending\n\".#\" to the name of the file being locked.  See also `lock-buffer' and\nInfo node `(emacs)Interlocking'.");
  create_lockfiles = true;

  DEFSYM! (Qlock_file, "lock-file");
  DEFSYM! (Qunlock_file, "unlock-file");
  DEFSYM! (Qfile_locked_p, "file-locked-p");
  DEFSYM! (Qmake_lock_file_name, "make-lock-file-name");
  DEFSYM! (Qstring_replace, "string-replace");
  DEFSYM! (Qask_user_about_lock, "ask-user-about-lock");
  DEFSYM! (Quserlock__handle_unlock_error, "userlock--handle-unlock-error");
  DEFSYM! (Quserlock__ask_user_about_supersession_threat,
	   "userlock--ask-user-about-supersession-threat");

  defsubr (&raw mut Slock_file);
  defsubr (&raw mut Sunlock_file);
  defsubr (&raw mut Slock_buffer);
  defsubr (&raw mut Sunlock_buffer);
  defsubr (&raw mut Sfile_locked_p);
}
