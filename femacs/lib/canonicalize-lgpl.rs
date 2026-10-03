// femacs: port of lib/canonicalize-lgpl.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// The frozen configuration leaves FUNC_REALPATH_WORKS undefined, so C 96-449's
// `#if defined _LIBC || !FUNC_REALPATH_WORKS` arm is the *live* text (that is why gnulib's
// canonicalize-lgpl module is in the build at all) and the port carries the whole
// implementation, not just the wrapper.  C 43-83's `#ifdef _LIBC` arm is dead (glibc
// internals: shlib-compat.h, `libc_hidden_def`, `versioned_symbol`, `weak_alias`); the live
// `#else` arm's nine identifier renames are the macros below.  C 452-466's
// `#if SHLIB_COMPAT (…)` arm is the glibc compat symbol and is dead for this target; it is
// kept as text under the feature that names the non-frozen side (P-51), which is also where
// the checklist's `__old_realpath` lives.
//
// Mirrors: <limits.h> (idx/PATH_MAX), <errno.h>, <fcntl.h> (AT_*), <sys/stat.h>,
// <unistd.h> (getcwd/readlink/faccessat), <string.h>, idx.h, filename.h,
// min-eloop-threshold.h, scratch_buffer.h.  The four entries this unit still needs -
// `strdup`, `strcpy`, `pathconf` and the `faccessat`/`_PC_PATH_MAX`/`F_OK` pair - are
// declared below with their C prototypes (the md5.rs shape, PORTING.md P-26), as is
// libc-config.h's `__set_errno` macro.
//
// Deviations (docs/ports/canonicalize-lgpl.md): `bool`/`false`/`true` stay Rust's, `NULL`
// is `std::ptr::null_mut ()`/`is_null ()`, C's forward `goto error` is the labelled block
// `'error` (PORTING.md 2.2), and `struct realpath_bufs bufs;` is zero-initialized
// (PORTING.md 1.5) before the three `scratch_buffer_init` calls C makes.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Return the canonical absolute name of a given file.
   Copyright (C) 1996-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/errno.rs");
include!("include/fcntl.in.rs");
include!("include/sys_stat.in.rs");   // <sys/stat.h> (and, through it, <sys/types.h>)
include!("include/unistd.in.rs");
include!("include/string.rs");
include!("include/idx.rs");
include!("include/filename.rs");
include!("include/min-eloop-threshold.rs");
include!("include/scratch_buffer.rs");

/* C 19-25's `#ifndef _LIBC` arm (live) is `<libc-config.h>` plus the empty
   `_GL_ARG_NONNULL(params)`; the macro has no Rust counterpart
   (docs/ports/excluded-macros.txt) and the header contributes only `__set_errno` (below).  */
/* C 43-46's `#ifdef _LIBC` arm is dead text: the port records the glibc-only items
   (`GCC_LINT`, `_GL_ATTRIBUTE_PURE` - the latter excluded) in the ledger.  */

/* C 48-51's renames of the three definitions below.  C's `#else` arm expands the
   *definition's own name* (`char * __realpath (…)` becomes `realpath`), which the port
   spells with `export_name` (the mempcpy precedent, PORTING.md 10.13); the macros are kept
   because they are part of the reference text and because the call sites C writes through
   them are the port's direct calls.  */
#[cfg (feature = "libc_internal")] macro_rules! __canonicalize_file_name_noop { ($name:expr) => { $name }; }
macro_rules! __canonicalize_file_name {
    ($name:expr) => { canonicalize_file_name ($name) };
}
/* C 50's `# define __realpath realpath`.  */
macro_rules! __realpath {
    ($name:expr, $resolved:expr) => { realpath ($name, $resolved) };
}
/* C 51's `# define __strdup strdup`.  */
macro_rules! __strdup {
    ($s:expr) => { strdup ($s) };
}
/* C 53's `# define __faccessat faccessat`.  */
macro_rules! __faccessat {
    ($fd:expr, $file:expr, $mode:expr, $flag:expr) => {
        faccessat ($fd, $file, $mode, $flag)
    };
}
/* C 72's `# define __getcwd getcwd` (C 56-71 pick the Windows/VMS/`getwd` arms instead).  */
macro_rules! __getcwd {
    ($buf:expr, $max:expr) => { getcwd ($buf, $max) };
}
/* C 74's `# define __mempcpy mempcpy`.  */
macro_rules! __mempcpy {
    ($dest:expr, $src:expr, $n:expr) => { mempcpy ($dest, $src, $n) };
}
/* C 75's `# define __pathconf pathconf`.  */
macro_rules! __pathconf {
    ($path:expr, $name:expr) => { pathconf ($path, $name) };
}
/* C 76's `# define __readlink readlink`.  */
macro_rules! __readlink {
    ($file:expr, $buf:expr, $bufsize:expr) => { readlink ($file, $buf, $bufsize) };
}

/* C 78-82's `#if IN_RELOCWRAPPER` arm (the `#undef memmove` pair) is dead: the frozen
   configuration has no relocatable-wrapper build.  */

/* Suppress bogus GCC -Wmaybe-uninitialized warnings.  */
/* #if defined GCC_LINT || defined lint
   # define IF_LINT(Code) Code
   #else
   # define IF_LINT(Code)  */
/* The frozen branch is the `#else` one (GCC_LINT is defined only in the dead `_LIBC` arm),
   so `IF_LINT (…)` expands to nothing; the port still spells the invocation at its one use
   site below, as C does.  */
macro_rules! IF_LINT {
    ($($code:tt)*) => { };
}

/* #ifndef DOUBLE_SLASH_IS_DISTINCT_ROOT
   # define DOUBLE_SLASH_IS_DISTINCT_ROOT false
   #endif  */
pub const DOUBLE_SLASH_IS_DISTINCT_ROOT: bool = false;

/* C 46's `# define GCC_LINT 1` (the dead `_LIBC` arm's macro; the portable GNU C lint
   spelling is not needed for anything the frozen arm compiles).  */
pub const GCC_LINT: c_int = 1;

/* The four declarations this unit needs beyond the mirrors (the md5.rs shape; `strcpy`
   moved into the string mirror, which now carries it).  */
unsafe extern "C" {
  /* void *mempcpy (void *dest, const void *src, size_t n);  (lib/mempcpy.c defines the
     symbol in this build; gnulib's <string.h> declares it as a system function, so no
     `rpl_` spelling is needed here.)  */
  pub fn mempcpy (dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
  /* char *strdup (const char *s);  */
  pub fn strdup (s: *const c_char) -> *mut c_char;
  /* long pathconf (const char *path, int name);  */
  pub fn pathconf (path: *const c_char, name: c_int) -> c_long;
}
/* <libc-config.h> 46: `# define __set_errno(val) (errno = (val))`.  */
macro_rules! __set_errno {
    ($val:expr) => { *errno () = $val };
}
/* <intprops.h>'s `INT_ADD_OVERFLOW (a, b)`: true when a + b wraps (PORTING.md 2.6's
   `overflowing_` family, which is also how the ckd_ helpers and intprops are spelled).  */
macro_rules! INT_ADD_OVERFLOW {
    ($a:expr, $b:expr) => { ($a).overflowing_add ($b).1 };
}

pub const F_OK: c_int = 0;
pub const _PC_PATH_MAX: c_int = 4;

/* C: `<fcntl.h>`'s `access` modes and `<unistd.h>`'s `R_OK`/… are not needed beyond
   `F_OK` above; `AT_FDCWD`/`AT_EACCESS` come from the fcntl mirror.  */

/* C 96's live arm.  */

/* Return true if FILE's existence can be shown, false (setting errno)
   otherwise.  Follow symbolic links.  */
unsafe extern "C" fn file_accessible (file: *const c_char) -> bool
{
  /* C 105-107's `# else` arm, kept as text (P-51; dead: HAVE_FACCESSAT is 1).  */
  #[cfg (feature = "no_faccessat")]
  {
    let mut st: stat = std::mem::zeroed ();
    return stat (file, &mut st) == 0 || *errno () == EOVERFLOW;
  }
  return __faccessat! (AT_FDCWD, file, F_OK, AT_EACCESS) == 0;
}

/* True if concatenating END as a suffix to a file name means that the
   code needs to check that the file name is that of a searchable
   directory, since the canonicalize_filename_mode_stk code won't
   check this later anyway when it checks an ordinary file name
   component within END.  END must either be empty, or start with a
   slash.  */

unsafe extern "C" fn suffix_requires_dir_check (end: *const c_char) -> bool
{
  let mut end = end;
  /* If END does not start with a slash, the suffix is OK.  */
  while ISSLASH! (*end)
    {
      /* Two or more slashes act like a single slash.  */
      loop
        { end = end.offset (1);
          if ! ISSLASH! (*end) { break; } }

      let c: c_char = *end;
      end = end.offset (1);
      match c as u8
        {
        0 => { return true; }                /* Trailing "/" is trouble.  */
        b'.' => { }                          /* Possibly "." or "..".  */
        _ => { return false; }               /* An ordinary file name component is OK.  */
        }
      /* Trailing "/.", or "/.." even if not trailing, is trouble.  */
      if *end == 0 || (*end == b'.' as c_char
                       && (*end.offset (1) == 0 || ISSLASH! (*end.offset (1))))
        { return true; }
    }

  return false;
}

/* Append this to a file name to test whether it is a searchable directory.
   On POSIX platforms "/" suffices, but "/./" is sometimes needed on
   macOS 10.13 <https://bugs.gnu.org/30350>, and should also work on
   platforms like AIX 7.2 that need at least "/.".  */

/* C 148-152's `# if defined _LIBC || defined LSTAT_FOLLOWS_SLASHED_SYMLINK` arm is live
   (LSTAT_FOLLOWS_SLASHED_SYMLINK is 1).  */
static dir_suffix: [c_char; 2] = [b'/' as c_char, 0];
/* C 150-151's `# else` arm, kept as text (P-51; dead).  */
#[cfg (feature = "no_lstat_follows_slashed_symlink")]
static dir_suffix_slashed: [c_char; 4]
  = [b'/' as c_char, b'.' as c_char, b'/' as c_char, 0];

/* Return true if DIR is a searchable dir, false (setting errno) otherwise.
   DIREND points to the NUL byte at the end of the DIR string.
   Store garbage into DIREND[0 .. strlen (dir_suffix)].  */

unsafe extern "C" fn dir_check (dir: *mut c_char, dirend: *mut c_char) -> bool
{
  let _ = strcpy (dirend, &dir_suffix as *const c_char);
  return file_accessible (dir);
}

unsafe extern "C" fn get_path_max () -> idx_t
{
  let mut path_max: c_long;
  /* C 172-173's `# ifdef PATH_MAX` arm is live (the frozen value is 1024).  */
  path_max = PATH_MAX;
  /* C 174-181's `# else` arm, kept as text (P-51; dead).  */
  #[cfg (feature = "no_path_max")]
  {
    /* The caller invoked realpath with a null RESOLVED, even though
       PATH_MAX is not defined as a constant.  The glibc manual says
       programs should not do this, and POSIX says the behavior is undefined.
       Historically, glibc here used the result of pathconf, or 1024 if that
       failed; stay consistent with this (dubious) historical practice.  */
    let err: c_int = *errno ();
    path_max = __pathconf! (c"/".as_ptr (), _PC_PATH_MAX);
    __set_errno! (err);
  }
  return (if path_max < 0 { 1024 } else if path_max <= IDX_MAX as c_long { path_max }
          else { IDX_MAX as c_long }) as idx_t;
}

/* Scratch buffers used by realpath_stk and managed by __realpath.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct realpath_bufs
{
  pub rname: scratch_buffer,
  pub extra: scratch_buffer,
  pub link: scratch_buffer,
}

unsafe extern "C" fn realpath_stk (name: *const c_char, resolved: *mut c_char,
                                   bufs: *mut realpath_bufs) -> *mut c_char
{
  let mut name = name;
  let mut start: *const c_char;
  let mut end: *const c_char;

  if name == std::ptr::null ()
    {
      /* As per Single Unix Specification V2 we must return an error if
         either parameter is a null pointer.  We extend this to allow
         the RESOLVED parameter to be NULL in case the we are expected to
         allocate the room for the return value.  */
      __set_errno! (EINVAL);
      return std::ptr::null_mut ();
    }

  if *name == 0
    {
      /* As per Single Unix Specification V2 we must return an error if
         the name argument points to an empty string.  */
      __set_errno! (ENOENT);
      return std::ptr::null_mut ();
    }

  let mut rname: *mut c_char = (*bufs).rname.data as *mut c_char;

  /* This is always zero for Posix hosts, but can be 2 for MS-Windows
     and MS-DOS X:/foo/bar file names.  */
  let mut prefix_len: idx_t;

  let mut dest: *mut c_char;

  let mut failed: bool = true;

  'error: {

  if ! IS_ABSOLUTE_FILE_NAME! (name)
    {
      while __getcwd! ((*bufs).rname.data as *mut c_char,
                       (*bufs).rname.length).is_null ()
        {
          if *errno () != ERANGE
            {
              dest = rname;
              break 'error;
            }
          if ! scratch_buffer_grow (&mut (*bufs).rname)
            { return std::ptr::null_mut (); }
          rname = (*bufs).rname.data as *mut c_char;
        }
      dest = strchr (rname, 0);
      start = name;
      prefix_len = FILE_SYSTEM_PREFIX_LEN! (rname) as idx_t;
    }
  else
    {
      prefix_len = FILE_SYSTEM_PREFIX_LEN! (name) as idx_t;
      dest = __mempcpy! (rname as *mut c_void, name as *const c_void,
                         prefix_len as usize) as *mut c_char;
      *dest = b'/' as c_char; dest = dest.offset (1);
      if DOUBLE_SLASH_IS_DISTINCT_ROOT
        {
          if prefix_len == 0 /* implies ISSLASH (name[0]) */
              && ISSLASH! (*name.offset (1)) && ! ISSLASH! (*name.offset (2))
            { *dest = b'/' as c_char; dest = dest.offset (1); }
          *dest = 0;
        }
      start = name.offset (prefix_len as isize);
    }

  let mut num_links: c_int = 0;
  let mut end_in_extra_buffer: bool = false;

  while *start != 0
    {
      /* Skip sequence of multiple file name separators.  */
      while ISSLASH! (*start)
        { start = start.offset (1); }

      /* Find end of component.  */
      end = start;
      while *end != 0 && ! ISSLASH! (*end)
        { end = end.offset (1); }              /* Nothing.  */

      /* Length of this file name component; it can be zero if a file
         name ends in '/'.  */
      let startlen: idx_t = end.offset_from (start);

      if startlen == 0
        { break; }
      else if startlen == 1 && *start == b'.' as c_char
        { }                                    /* nothing */
      else if startlen == 2 && *start == b'.' as c_char
              && *start.offset (1) == b'.' as c_char
        {
          /* Back up to previous component, ignore if at root already.  */
          if dest > rname.offset ((prefix_len + 1) as isize)
            { dest = dest.offset (-1);
              while dest > rname && ! ISSLASH! (*dest.offset (-1))
                { dest = dest.offset (-1); } }
          if DOUBLE_SLASH_IS_DISTINCT_ROOT
              && dest == rname.offset (1) && prefix_len == 0
              && ISSLASH! (*dest) && ! ISSLASH! (*dest.offset (1))
            { dest = dest.offset (1); }
        }
      else
        {
          if ! ISSLASH! (*dest.offset (-1))
            { *dest = b'/' as c_char; dest = dest.offset (1); }

          while rname.offset ((*bufs).rname.length as isize).offset_from (dest)
                  < startlen + std::mem::size_of_val (&dir_suffix) as idx_t
            {
              let dest_offset: idx_t = dest.offset_from (rname);
              if ! scratch_buffer_grow_preserve (&mut (*bufs).rname)
                { return std::ptr::null_mut (); }
              rname = (*bufs).rname.data as *mut c_char;
              dest = rname.offset (dest_offset as isize);
            }

          dest = __mempcpy! (dest as *mut c_void, start as *const c_void,
                             startlen as usize) as *mut c_char;
          *dest = 0;

          let mut buf: *mut c_char;
          let mut n: ssize_t = 0;
          loop
            {
              buf = (*bufs).link.data as *mut c_char;
              let bufsize: idx_t = (*bufs).link.length as idx_t;
              n = __readlink! (rname, buf, (bufsize - 1) as usize) as ssize_t;
              if n < (bufsize - 1) as ssize_t
                { break; }
              if ! scratch_buffer_grow (&mut (*bufs).link)
                { return std::ptr::null_mut (); }
            }
          if 0 <= n
            {
              num_links += 1;
              if num_links > MIN_ELOOP_THRESHOLD
                {
                  __set_errno! (ELOOP);
                  break 'error;
                }

              *buf.offset (n as isize) = 0;

              let mut extra_buf: *mut c_char = (*bufs).extra.data as *mut c_char;
              let mut end_idx: idx_t = 0;
              IF_LINT! (end_idx = 0;);
              if end_in_extra_buffer
                { end_idx = end.offset_from (extra_buf); }
              let len: usize = strlen (end);
              if INT_ADD_OVERFLOW! (len as usize, n as usize)
                {
                  __set_errno! (ENOMEM);
                  return std::ptr::null_mut ();
                }
              while (*bufs).extra.length <= len + n as usize
                {
                  if ! scratch_buffer_grow_preserve (&mut (*bufs).extra)
                    { return std::ptr::null_mut (); }
                  extra_buf = (*bufs).extra.data as *mut c_char;
                }
              if end_in_extra_buffer
                { end = extra_buf.offset (end_idx as isize); }

              /* Careful here, end may be a pointer into extra_buf... */
              let _ = memmove (extra_buf.offset (n as isize) as *mut c_void,
                               end as *const c_void, len + 1);
              name = memcpy (extra_buf as *mut c_void, buf as *const c_void,
                             n as usize) as *const c_char;
              end = name;
              end_in_extra_buffer = true;

              if IS_ABSOLUTE_FILE_NAME! (buf)
                {
                  let pfxlen: idx_t = FILE_SYSTEM_PREFIX_LEN! (buf) as idx_t;

                  dest = __mempcpy! (rname as *mut c_void, buf as *const c_void,
                                     pfxlen as usize) as *mut c_char;
                  *dest = b'/' as c_char; dest = dest.offset (1); /* It's an absolute symlink */
                  if DOUBLE_SLASH_IS_DISTINCT_ROOT
                    {
                      if ISSLASH! (*buf.offset (1)) && ! ISSLASH! (*buf.offset (2))
                          && pfxlen == 0
                        { *dest = b'/' as c_char; dest = dest.offset (1); }
                      *dest = 0;
                    }
                  /* Install the new prefix to be in effect hereafter.  */
                  prefix_len = pfxlen;
                }
              else
                {
                  /* Back up to previous component, ignore if at root
                     already: */
                  if dest > rname.offset ((prefix_len + 1) as isize)
                    { dest = dest.offset (-1);
                      while dest > rname && ! ISSLASH! (*dest.offset (-1))
                        { dest = dest.offset (-1); } }
                  if DOUBLE_SLASH_IS_DISTINCT_ROOT && dest == rname.offset (1)
                      && ISSLASH! (*dest) && ! ISSLASH! (*dest.offset (1))
                      && prefix_len == 0
                    { dest = dest.offset (1); }
                }
            }
          else if ! (if suffix_requires_dir_check (end) { dir_check (rname, dest) }
                     else { *errno () == EINVAL })
            { break 'error; }
        }

      start = end;
    }
  if dest > rname.offset ((prefix_len + 1) as isize) && ISSLASH! (*dest.offset (-1))
    { dest = dest.offset (-1); }
  if DOUBLE_SLASH_IS_DISTINCT_ROOT && dest == rname.offset (1) && prefix_len == 0
      && ISSLASH! (*dest) && ! ISSLASH! (*dest.offset (1))
    { dest = dest.offset (1); }
  failed = false;

  } /* error: */

  *dest = 0; dest = dest.offset (1);
  if resolved != std::ptr::null_mut ()
    {
      /* Copy the full result on success or partial result if failure was due
         to the path not existing or not being accessible.  */
      if ((!failed || *errno () == ENOENT || *errno () == EACCES)
          && dest.offset_from (rname) <= get_path_max ())
        {
          let _ = strcpy (resolved, rname);
          if failed
            { return std::ptr::null_mut (); }
          else
            { return resolved; }
        }
      if !failed
        { __set_errno! (ENAMETOOLONG); }
      return std::ptr::null_mut ();
    }
  else
    {
      if failed
        { return std::ptr::null_mut (); }
      else
        { return __strdup! ((*bufs).rname.data as *const c_char); }
    }
}

/* Return the canonical absolute name of file NAME.  A canonical name
   does not contain any ".", ".." components nor any repeated file name
   separators ('/') or symlinks.  All file name components must exist.  If
   RESOLVED is null, the result is malloc'd; otherwise, if the
   canonical name is PATH_MAX chars or more, returns null with 'errno'
   set to ENAMETOOLONG; if the name fits in fewer than PATH_MAX chars,
   returns the name in RESOLVED.  If the name cannot be resolved and
   RESOLVED is non-NULL, it contains the name of the first component
   that cannot be resolved.  If the name can be resolved, RESOLVED
   holds the same value as the value returned.  */

#[unsafe(export_name = "rpl_realpath")]
pub unsafe extern "C" fn __realpath (name: *const c_char, resolved: *mut c_char)
  -> *mut c_char
{
  /* C declares `struct realpath_bufs bufs;` uninitialized; the three
     `scratch_buffer_init` calls below define every field the code reads (PORTING.md 1.5).  */
  let mut bufs: realpath_bufs = std::mem::zeroed ();
  scratch_buffer_init (&mut bufs.rname);
  scratch_buffer_init (&mut bufs.extra);
  scratch_buffer_init (&mut bufs.link);
  let result: *mut c_char = realpath_stk (name, resolved, &mut bufs);
  scratch_buffer_free (&mut bufs.link);
  scratch_buffer_free (&mut bufs.extra);
  scratch_buffer_free (&mut bufs.rname);
  return result;
}
/* C 447-448's `libc_hidden_def (__realpath)` / `versioned_symbol (…)` are glibc-only
   (dead); the ledger records them.  */

/* C 452-466's glibc compat symbol, kept as text (P-51; dead: SHLIB_COMPAT names a
   non-frozen configuration).  */
#[cfg (feature = "libc_shlib_compat")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __old_realpath (name: *const c_char, resolved: *mut c_char)
  -> *mut c_char
{
  if resolved == std::ptr::null_mut ()
    {
      __set_errno! (EINVAL);
      return std::ptr::null_mut ();
    }

  return __realpath (name, resolved);
}

/* C 469-474's wrapper: the live arm's `#define __canonicalize_file_name
   canonicalize_file_name` makes the emitted symbol the macro's spelling, which the port
   carries with `export_name`; C 474's `weak_alias (…)` is glibc-only (dead).  */
#[unsafe(export_name = "canonicalize_file_name")]
pub unsafe extern "C" fn __canonicalize_file_name (name: *const c_char) -> *mut c_char
{
  return __realpath (name, std::ptr::null_mut ());
}
