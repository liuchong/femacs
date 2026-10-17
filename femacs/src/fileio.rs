// femacs: port of src/fileio.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  The 55 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); the non-`static` definitions C
// exports carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay
// plain `unsafe fn`.
//
// `src/fileio.c` *is* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/fileio.c` = `122 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen configuration does not
// compile are *recorded in `docs/ports/fileio.md`, not transcribed*.  See that file for the
// list of dead arms and for the mirror gaps this unit reports and does not touch.
//
// **Use-site declarations.**  Several headers this file includes have no mirror; each entry
// below carries C's own prototype in a `/// C:` line (PORTING.md 0.1-2/P-72 rule 3, the
// editfns.rs/sysdep.rs shape) and each record's layout is measured with the frozen clang and
// pinned by `const _` assertions.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                 // C 21: `#include <config.h>`
include!("../lib/include/limits.in.rs");       // C 22: `#include <limits.h>`
include!("../lib/include/fcntl.in.rs");        // C 23: `#include <fcntl.h>`
include!("../lib/include/stdio.rs");           // C 24: `sysstdio.h` 24's `#include <stdio.h>`
include!("../lib/include/sys_types.in.rs");    // C 25: `#include <sys/types.h>`
include!("../lib/include/pwd.rs");           // C: `#include <pwd.h>`
include!("../lib/include/grp.rs");           // C: `#include <grp.h>`
include!("../lib/include/sys_utsname.rs");   // C: `#include <sys/utsname.h>`
include!("../lib/include/sys_stat.in.rs");     // C 26: `#include <sys/stat.h>`
include!("../lib/include/unistd.in.rs");       // C 27: `#include <unistd.h>`
include!("../lib/include/errno.rs");           // C 40: `#include <errno.h>`
include!("../lib/include/c-ctype.rs");         // C 51: `#include <c-ctype.h>`
include!("../lib/include/stdlib.rs");          // lisp.h's `<stdlib.h>` (free, abort)
include!("../lib/include/string.rs");          // lisp.h 30's `<string.h>` (memcpy, strlen)
include!("../lib/include/stdcountof.rs");      // lisp.h 28: `#include <stdcountof.h>` (countof)
include!("include/lisp.rs");                   // C 54: `#include "lisp.h"`
include!("include/composite.rs");              // C 55: `#include "composite.h"`
include!("include/character.rs");              // C 56: `#include "character.h"`
include!("include/buffer.rs");                 // C 57: `#include "buffer.h"`
include!("include/coding.rs");                 // C 58: `#include "coding.h"`
include!("include/window.rs");                 // C 59: `#include "window.h"`
include!("include/blockinput.rs");             // C 60: `#include "blockinput.h"`
include!("include/region-cache.rs");           // C 61: `#include "region-cache.h"`
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"` (buffer.rs's fields)
include!("include/termhooks.rs");              // frame.h 22: `#include "termhooks.h"` (`terminal`)
include!("include/font.rs");                   // dispextern.h's font names
include!("include/dispextern.rs");             // frame.h's `text_cursor_kinds`/`output_method`
include!("include/frame.rs");                  // C 62: `#include "frame.h"`
include!("include/systime.rs");                // C 107: `#include "systime.h"`
include!("../lib/include/acl.rs");             // C 108: `#include <acl.h>`
include!("../lib/include/allocator.rs");       // C 109: `#include <allocator.h>`
include!("../lib/include/careadlinkat.rs");    // C 110: `#include <careadlinkat.h>`
include!("../lib/include/filename.rs");        // C 111: `#include <filename.h>`
include!("../lib/include/fsusage.rs");         // C 112: `#include <fsusage.h>`
include!("../lib/include/stat-time.rs");       // C 113: `#include <stat-time.h>`
include!("../lib/include/tempname.rs");        // C 114: `#include <tempname.h>`
include!("../lib/include/binary-io.rs");       // C 116: `#include <binary-io.h>`
include!("../lib/include/sys-limits.rs");      // lisp.h's `<sys-limits.h>` (SYS_BUFSIZE_MAX)
include!("include/commands.rs");               // C 122: `#include "commands.h"`
/* C 60: `#include "android.h"`, inside CC's `#ifdef HAVE_ANDROID`.  The mirror declares C's 85 prototypes in
   its two modules and glob-imports the second (`include/android.rs` 324), so including it is what
   supplies `android_is_special_directory`/`android_rewrite_spawn_argv`/`android_init_gui` - the
   names this unit read as `cannot find function` on `aarch64-linux-android` (measured).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
include!("include/android.rs");               // C 60: `#include "android.h"`

/* C 29-31: `#ifdef DARWIN_OS` / `#include <sys/attr.h>` (DARWIN_OS *is* defined on the
   frozen target, so the include is live).  This file names none of the header's entries, so
   it contributes no name; recorded in `docs/ports/fileio.md`.  */

/* C 42-49 (`HAVE_LIBSELINUX` and `USE_ACL && HAVE_ACL_SET_FILE`), C 64-66 (`HAVE_ANDROID`),
   C 68-71 (`HAVE_LINUX_FS_H`), C 73-87 (`WINDOWSNT`/`MSDOS`), C 89-105 (`DOS_NT`),
   C 118-120 (`HPUX`): dead `#include`s, recorded in `docs/ports/fileio.md`.  */

unsafe extern "C" {
  /// C: `lisp.h` 5310: `extern char *emacs_strerror (int);`
  pub fn emacs_strerror (error_number: c_int) -> *mut c_char;
  /// C: `lisp.h` 5305: `extern int sys_fstat (int, struct stat *);`
  pub fn sys_fstat (fd: c_int, buf: *mut stat) -> c_int;
  /// C: `lisp.h` 5306: `extern int sys_faccessat (int, const char *, int, int);`
  pub fn sys_faccessat (dirfd: c_int, file: *const c_char, amode: c_int, flags: c_int)
    -> c_int;
}

/* C 124-135: `#if !defined HAVE_ANDROID || defined ANDROID_STUBIFY` (HAVE_ANDROID is undef,
   so this is the live arm): the file-descriptor type and the seven function-like macros.
   The other arm (C 137-146, `struct android_fd_or_asset`) is recorded in
   `docs/ports/fileio.md`.  */

/// Type describing a file descriptor used by functions such as `insert-file-contents'.
type emacs_fd = c_int;

/// Function used to read and open from such a file descriptor.
macro_rules! emacs_fd_open { ($($args:tt)*) => { emacs_open ($($args)*) }; }
macro_rules! emacs_fd_close { ($($args:tt)*) => { emacs_close ($($args)*) }; }
macro_rules! emacs_fd_read { ($($args:tt)*) => { emacs_read_quit ($($args)*) }; }
macro_rules! emacs_fd_lseek { ($($args:tt)*) => { lseek ($($args)*) }; }
macro_rules! emacs_fd_fstat { ($($args:tt)*) => { sys_fstat ($($args)*) }; }
macro_rules! emacs_fd_valid_p { ($fd:expr) => { ($fd) >= 0 }; }
/* This is not used on MS Windows.  */
macro_rules! emacs_fd_to_int { ($fds:expr) => { ($fds) }; }

/* True during writing of auto-save files.  */
static mut auto_saving: bool = false;

/* The port's real umask.  */
static mut realmask: mode_t = 0;

/* Nonzero umask during creation of auto-save directories.  */
static mut auto_saving_dir_umask: mode_t = 0;

/* Set by auto_save_1 to mode of original file so Fwrite_region will create
   a new file with the same mode as the original.  */
static mut auto_save_mode_bits: mode_t = 0;

/* Set by auto_save_1 if an error occurred during the last auto-save.  */
static mut auto_save_error_occurred: bool = false;

/* If VALID_TIMESTAMP_FILE_SYSTEM, then TIMESTAMP_FILE_SYSTEM is the device
   number of a file system where time stamps were observed to work.  */
static mut valid_timestamp_file_system: bool = false;
static mut timestamp_file_system: dev_t = 0;

/* Each time an annotation function changes the buffer, the new buffer
   is added here.  */
static mut Vwrite_region_annotation_buffers: Lisp_Object = lisp_h_Qnil ();

/* The `DEFVAR_LISP`/`DEFVAR_BOOL` storage this unit owns (status 3.29): C keeps every
   DEFVAR_* variable in `struct emacs_globals` (alloc.c's `globals`), so the unit that
   registers one has to define it as well - the mirror's declaration is an `extern`, and a
   variable that only the declaration reaches is an object no gate can see (check-globals.py
   is that gate).  The names, types and initial values are `syms_of_fileio`'s by C's order.  */
#[unsafe(no_mangle)]
pub static mut Vfile_name_coding_system: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdefault_file_name_coding_system: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vfile_name_handler_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vset_auto_coding_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vafter_insert_file_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwrite_region_annotate_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwrite_region_post_annotation_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vwrite_region_annotations_so_far: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinhibit_file_name_handlers: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinhibit_file_name_operation: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_save_list_file_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_save_visited_file_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_save_include_big_deletions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut write_region_inhibit_fsync: bool = false;
#[unsafe(no_mangle)]
pub static mut delete_by_moving_to_trash: bool = false;

/* C 177-181's three forward declarations (`emacs_readlinkat`, `a_write`, `e_write`) are
   dropped (PORTING.md 0.1-12).  */

/* Establish that ENCODED is not contained within a special directory
   whose contents are not eligible for Unix VFS operations.  Signal a
   `file-error' with REASON if it does.  */

/// The whole of C's body is inside `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`
/// (HAVE_ANDROID undef), so the frozen configuration compiles an empty body; that arm is
/// recorded in `docs/ports/fileio.md` (P-08).
unsafe fn check_vfs_filename (encoded: Lisp_Object, reason: *const c_char)
{
  /* C 192-205's whole body is inside `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`
     (HAVE_ANDROID undef), so the frozen configuration compiles it as empty text and
     `docs/ports/fileio.md` records the arm (P-08).  The arm's own text is kept here behind a
     feature named for the non-frozen side (the P-51/3.49 shape): an empty body is the one
     thing the token check rejects outright, and "more C text is more 1:1".  */
  #[cfg (feature = "have_android")]
  {
    let mut name: *const c_char;

    name = SSDATA (encoded);

    if android_is_special_directory (name, c"/assets".as_ptr ())
	|| android_is_special_directory (name, c"/content".as_ptr ())
      { xsignal2 (Qfile_error (), build_string (reason), encoded); }
  }
}

/* `selinux_enabled_p` (C 194-208) is entirely inside `#ifdef HAVE_LIBSELINUX`, so the frozen
   configuration does not compile it; recorded in `docs/ports/fileio.md` (P-08).  */

/* Test whether FILE is accessible for AMODE.
   Return true if successful, false (setting errno) otherwise.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_access_p (file: *const c_char, amode: c_int) -> bool
{
  if sys_faccessat (AT_FDCWD, file, amode, AT_EACCESS) == 0
    { return true; }

  return false;
}

/* Signal a file-access failure.  STRING describes the failure,
   NAME the file involved, and ERRORNO the errno value.

   If NAME is neither null nor a pair, package it up as a singleton
   list before reporting it; this saves report_file_errno's caller the
   trouble of preserving errno before calling list1.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_file_errno_data (string: *const c_char, name: Lisp_Object,
					      errorno: c_int) -> Lisp_Object
{
  let mut data: Lisp_Object = if CONSP (name) || NILP (name) { name } else { list1 (name) };
  let mut str_: *mut c_char = emacs_strerror (errorno);
  AUTO_STRING! (unibyte_str, str_);
  let mut errstring: Lisp_Object
    = code_convert_string_norecord (unibyte_str, Vlocale_coding_system, false);
  let mut errdata: Lisp_Object = Fcons (errstring, data);

  if errorno == EEXIST
    { return Fcons (Qfile_already_exists (), errdata); }
  else
    { return Fcons (if errorno == ENOENT
		    { Qfile_missing () }
		    else if errorno == EACCES
		    { Qpermission_denied () }
		    else { Qfile_error () },
		    Fcons (build_string (string), errdata)); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_file_errno (string: *const c_char, name: Lisp_Object,
					    errorno: c_int) -> !
{
  let mut data: Lisp_Object = get_file_errno_data (string, name, errorno);
  xsignal (data, Qnil ());
}

/* Signal a file-access failure that set errno.  STRING describes the
   failure, NAME the file involved.  When invoking this function, take
   care to not use arguments such as build_string ("foo") that involve
   side effects that may set errno.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_file_error (string: *const c_char, name: Lisp_Object) -> !
{
  report_file_errno (string, name, *errno ());
}

/* `report_file_notify_error` (C 279-294) is entirely inside `#ifdef USE_FILE_NOTIFY`
   (undefined), so the frozen configuration does not compile it; recorded in
   `docs/ports/fileio.md` (P-08).  */

/* ACTION failed for FILE with errno ERR.  Signal an error if ERR
   means the file's metadata could not be retrieved even though it may
   exist, otherwise return nil.  */

unsafe fn file_metadata_errno (action: *const c_char, file: Lisp_Object,
				      err: c_int) -> Lisp_Object
{
  if err == ENOENT || err == ENOTDIR || err == 0
    { return Qnil (); }
  report_file_errno (action, file, err);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_attribute_errno (file: Lisp_Object, err: c_int) -> Lisp_Object
{
  return file_metadata_errno (c"Getting attributes".as_ptr (), file, err);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_file_unwind (fd: c_int)
{
  emacs_close (fd);
}

unsafe extern "C" fn close_file_unwind_emacs_fd (ptr: *mut c_void)
{
  let mut fd: *mut emacs_fd;

  fd = ptr as *mut emacs_fd;
  emacs_fd_close! (*fd);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose_unwind (arg: *mut c_void)
{
  let mut stream: *mut FILE = arg as *mut FILE;
  emacs_fclose! (stream);
}

/* Restore point, having saved it as a marker.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_point_unwind (location: Lisp_Object)
{
  Fgoto_char (location);
  unchain_marker (XMARKER (location) as *mut c_void);
}

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

/// `<config.h>` 120: `#define DIRECTORY_SEP '/'`; 2099: `#define IS_DIRECTORY_SEP(_c_)
/// ((_c_) == DIRECTORY_SEP)`.  `config.rs` carries neither, so they are declared here (the
/// buffer.rs/callproc.rs shape); the argument is C's promoted `int`.
const DIRECTORY_SEP: c_char = b'/' as c_char;
#[inline(always)]
fn IS_DIRECTORY_SEP (_c_: c_int) -> bool { _c_ == DIRECTORY_SEP as c_int }
/// `<config.h>` 2089: `#define IS_DEVICE_SEP(_c_) ((_c_) == ':')`.  `config.rs` carries no
/// such name, so it is declared here (the buffer.rs/callproc.rs shape); the argument is C's
/// promoted `int`.
#[inline(always)]
fn IS_DEVICE_SEP (_c_: c_int) -> bool { _c_ == b':' as c_int }
/// `<config.h>` 2091: `#define IS_ANY_SEP(_c_) (IS_DIRECTORY_SEP (_c_))`.
macro_rules! IS_ANY_SEP { ($c:expr) => { IS_DIRECTORY_SEP ($c) }; }

unsafe extern "C" {
  /// C: lisp.h 4566-4567: `extern Lisp_Object make_specified_string (const char *,
  /// ptrdiff_t, ptrdiff_t, bool);` - `lisp.rs` does not carry it (the ccl.rs note).
  pub fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t,
				nbytes: ptrdiff_t, multibyte: bool) -> Lisp_Object;
}

DEFUN! ("find-file-name-handler", Ffind_file_name_handler,
       Sfind_file_name_handler, 2, 2, 0,
       doc: "Return FILENAME's handler function for OPERATION, if it has one.\nOtherwise, return nil.\nA file name is handled if one of the regular expressions in\n`file-name-handler-alist' matches it.\n\nIf OPERATION equals `inhibit-file-name-operation', then ignore\nany handlers that are members of `inhibit-file-name-handlers',\nbut still do run any other handlers.  This lets handlers\nuse the standard functions without calling themselves recursively.",
       (filename: Lisp_Object, operation: Lisp_Object),
       {
  /* This function must not munge the match data.  */
  let mut chain: Lisp_Object;
  let mut inhibited_handlers: Lisp_Object;
  let mut result: Lisp_Object;
  let mut pos: ptrdiff_t = -1;

  result = Qnil ();
  CHECK_STRING (filename);

  if EQ (operation, Vinhibit_file_name_operation)
    { inhibited_handlers = Vinhibit_file_name_handlers; }
  else
    { inhibited_handlers = Qnil (); }

  chain = Vfile_name_handler_alist;
  while CONSP (chain)
    {
      let mut elt: Lisp_Object;
      elt = XCAR (chain);
      if CONSP (elt)
	{
	  let mut string: Lisp_Object = XCAR (elt);
	  let mut match_pos: ptrdiff_t;
	  let mut handler: Lisp_Object = XCDR (elt);
	  let mut operations: Lisp_Object = Qnil ();

	  if SYMBOLP (handler)
	    { operations = Fget (handler, Qoperations ()); }

	  if STRINGP (string)
	    && { match_pos = fast_string_match (string, filename); match_pos > pos }
	    && (NILP (operations) || ! NILP (Fmemq (operation, operations)))
	    {
	      let mut tem: Lisp_Object;

	      handler = XCDR (elt);
	      tem = Fmemq (handler, inhibited_handlers);
	      if NILP (tem)
		{
		  result = handler;
		  pos = match_pos;
		}
	    }
	}

      maybe_quit ();
      chain = XCDR (chain);
    }
  return result;
});

/* Return the directory component of FILENAME, or nil if FILENAME does
   not contain a directory component.  */

unsafe fn file_name_directory (filename: Lisp_Object) -> Lisp_Object
{
  let mut beg: *mut c_char = SSDATA (filename);
  let mut p: *const c_char = beg.offset (SBYTES (filename)) as *const c_char;

  while p != beg as *const c_char && ! IS_DIRECTORY_SEP (*p.offset (-1) as c_int)
    { p = p.offset (-1); }

  if p == beg as *const c_char
    { return Qnil (); }
  return make_specified_string (beg, -1, p.offset_from (beg as *const c_char),
				STRING_MULTIBYTE (filename));
}

DEFUN! ("file-name-directory", Ffile_name_directory, Sfile_name_directory,
       1, 1, 0,
       doc: "Return the directory component in file name FILENAME.\nReturn nil if FILENAME does not include a directory.\nOtherwise return a directory name.\nGiven a Unix syntax file name, returns a string ending in slash.",
       (filename: Lisp_Object),
       {
  let mut handler: Lisp_Object;

  CHECK_STRING (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qfile_name_directory ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qfile_name_directory (),
						  filename);
      return if STRINGP (handled_name) { handled_name } else { Qnil () };
    }

  return file_name_directory (filename);
});

DEFUN! ("file-name-nondirectory", Ffile_name_nondirectory,
       Sfile_name_nondirectory, 1, 1, 0,
       doc: "Return file name FILENAME sans its directory.\nFor example, in a Unix-syntax file name,\nthis is everything after the last slash,\nor the entire name if it contains no slash.",
       (filename: Lisp_Object),
       {
  let mut beg: *const c_char;
  let mut p: *const c_char;
  let mut end: *const c_char;
  let mut handler: Lisp_Object;

  CHECK_STRING (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qfile_name_nondirectory ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qfile_name_nondirectory (),
						  filename);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  beg = SSDATA (filename);
  end = beg.offset (SBYTES (filename));
  p = end;

  while p != beg && ! IS_DIRECTORY_SEP (*p.offset (-1) as c_int)
    { p = p.offset (-1); }

  return make_specified_string (p, -1, end.offset_from (p), STRING_MULTIBYTE (filename));
});

DEFUN! ("unhandled-file-name-directory", Funhandled_file_name_directory,
       Sunhandled_file_name_directory, 1, 1, 0,
       doc: "Return a directly usable directory name somehow associated with FILENAME.\nA `directly usable' directory name is one that may be used without the\nintervention of any file name handler.\nIf FILENAME is a directly usable file itself, return\n(file-name-as-directory FILENAME).\nIf FILENAME refers to a file which is not accessible from a local process,\nthen this should return nil.\nThe `call-process' and `start-process' functions use this function to\nget a current directory to run processes in.",
       (filename: Lisp_Object),
       {
  let mut handler: Lisp_Object;

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qunhandled_file_name_directory ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qunhandled_file_name_directory (),
						  filename);
      return if STRINGP (handled_name) { handled_name } else { Qnil () };
    }

  return Ffile_name_as_directory (filename);
});

/* Maximum number of bytes that DST will be longer than SRC
   in file_name_as_directory.  This occurs when SRCLEN == 0.  */
/* C 614: `enum { file_name_as_directory_slop = 2 };`  */
const file_name_as_directory_slop: c_int = 2;

/* Convert from file name SRC of length SRCLEN to directory name in
   DST.  MULTIBYTE non-zero means the file name in SRC is a multibyte
   string.  On UNIX, just make sure there is a terminating /.  Return
   the length of DST in bytes.  */

unsafe fn file_name_as_directory (dst: *mut c_char, src: *const c_char,
					 mut srclen: ptrdiff_t, multibyte: bool) -> ptrdiff_t
{
  if srclen == 0
    {
      *dst.offset (0) = b'.' as c_char;
      *dst.offset (1) = b'/' as c_char;
      *dst.offset (2) = b'\0' as c_char;
      return 2;
    }

  memcpy (dst as *mut c_void, src as *const c_void, srclen as usize);
  if ! IS_DIRECTORY_SEP (*dst.offset (srclen - 1) as c_int)
    { *dst.offset (srclen) = DIRECTORY_SEP; srclen += 1; }
  *dst.offset (srclen) = 0;
  return srclen;
}

DEFUN! ("file-name-as-directory", Ffile_name_as_directory,
       Sfile_name_as_directory, 1, 1, 0,
       doc: "Return a string representing the file name FILE interpreted as a directory.\nThis operation exists because a directory is also a file, but its name as\na directory is different from its name as a file.\nThe result can be used as the value of `default-directory'\nor passed as second argument to `expand-file-name'.\nFor a Unix-syntax file name, just appends a slash unless a trailing slash\nis already present.",
       (file: Lisp_Object),
       {
  let mut buf: *mut c_char;
  let mut length: ptrdiff_t;
  let mut handler: Lisp_Object;
  let mut val: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  CHECK_STRING (file);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qfile_name_as_directory ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qfile_name_as_directory (),
						  file);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  buf = SAFE_ALLOCA! (sa_avail,
		      SBYTES (file) + file_name_as_directory_slop as isize + 1) as *mut c_char;
  length = file_name_as_directory (buf, SSDATA (file), SBYTES (file),
				   STRING_MULTIBYTE (file));
  val = make_specified_string (buf, -1, length, STRING_MULTIBYTE (file));
  SAFE_FREE! (sa_count, sa_mark);
  return val;
});

/* Convert from directory name SRC of length SRCLEN to file name in
   DST.  MULTIBYTE non-zero means the file name in SRC is a multibyte
   string.  On UNIX, just make sure there isn't a terminating /.
   Return the length of DST in bytes.  */

unsafe fn directory_file_name (dst: *mut c_char, src: *mut c_char,
				      mut srclen: ptrdiff_t, multibyte: bool) -> ptrdiff_t
{
  /* In Unix-like systems, just remove any final slashes.  However, if
     they are all slashes, leave "/" and "//" alone, and treat "///"
     and longer as if they were "/".  */
  if ! (srclen == 2 && IS_DIRECTORY_SEP (*src.offset (0) as c_int))
    {
      while srclen > 1 && IS_DIRECTORY_SEP (*src.offset (srclen - 1) as c_int)
	{ srclen -= 1; }
    }

  memcpy (dst as *mut c_void, src as *const c_void, srclen as usize);
  *dst.offset (srclen) = 0;
  return srclen;
}

DEFUN! ("directory-name-p", Fdirectory_name_p, Sdirectory_name_p, 1, 1, 0,
       doc: "Return non-nil if NAME ends with a directory separator character.",
       (name: Lisp_Object),
       {
  CHECK_STRING (name);
  let mut namelen: ptrdiff_t = SBYTES (name);
  let mut c: c_uchar = if namelen != 0 { SREF (name, (namelen - 1) as usize) } else { 0 };
  return if IS_DIRECTORY_SEP (c as c_int) { Qt () } else { Qnil () };
});

/* Return the expansion of NEWNAME, except that if NEWNAME is a
   directory name then return the expansion of FILE's basename under
   NEWNAME.  This resembles how 'cp FILE NEWNAME' works, except that
   it requires NEWNAME to be a directory name (typically, by ending in
   "/").  */

unsafe fn expand_cp_target (file: Lisp_Object, newname: Lisp_Object) -> Lisp_Object
{
  return if !NILP (Fdirectory_name_p (newname))
    { Fexpand_file_name (Ffile_name_nondirectory (file), newname) }
  else
    { Fexpand_file_name (newname, Qnil ()) };
}

DEFUN! ("directory-file-name", Fdirectory_file_name, Sdirectory_file_name,
       1, 1, 0,
       doc: "Returns the file name of the directory named DIRECTORY.\nThis is the name of the file that holds the data for the directory DIRECTORY.\nThis operation exists because a directory is also a file, but its name as\na directory is different from its name as a file.\nIn Unix-syntax, this function just removes the final slash.",
       (directory: Lisp_Object),
       {
  let mut buf: *mut c_char;
  let mut length: ptrdiff_t;
  let mut handler: Lisp_Object;
  let mut val: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  CHECK_STRING (directory);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (directory, Qdirectory_file_name ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qdirectory_file_name (),
						  directory);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  buf = SAFE_ALLOCA! (sa_avail, SBYTES (directory) + 1) as *mut c_char;
  length = directory_file_name (buf, SSDATA (directory), SBYTES (directory),
				STRING_MULTIBYTE (directory));
  val = make_specified_string (buf, -1, length, STRING_MULTIBYTE (directory));
  SAFE_FREE! (sa_count, sa_mark);
  return val;
});

DEFUN! ("make-temp-file-internal", Fmake_temp_file_internal,
       Smake_temp_file_internal, 4, 4, 0,
       doc: "Generate a new file whose name starts with PREFIX, a string.\nReturn the name of the generated file.  If DIR-FLAG is zero, do not\ncreate the file, just its name.  Otherwise, if DIR-FLAG is non-nil,\ncreate an empty directory.  The file name should end in SUFFIX.\nDo not expand PREFIX; a non-absolute PREFIX is relative to the Emacs\nworking directory.  If TEXT is a string, insert it into the newly\ncreated file.\n\nOn Posix systems, the file/directory is created with access mode bits\nthat limit access to the current user.\n\nSignal an error if the file could not be created.\n\nThis function does not grok magic file names.",
       (prefix: Lisp_Object, dir_flag: Lisp_Object, suffix: Lisp_Object,
        text: Lisp_Object),
       {
  CHECK_STRING (prefix);
  CHECK_STRING (suffix);
  let mut encoded_prefix: Lisp_Object = ENCODE_FILE (prefix);
  let mut encoded_suffix: Lisp_Object = ENCODE_FILE (suffix);
  let mut prefix_len: ptrdiff_t = SBYTES (encoded_prefix);
  let mut suffix_len: ptrdiff_t = SBYTES (encoded_suffix);
  if (INT_MAX as isize) < suffix_len
    { args_out_of_range (prefix, suffix); }
  let mut nX: c_int = 6;
  let mut val: Lisp_Object = make_uninit_string (prefix_len + nX as isize + suffix_len);
  let mut data: *mut c_char = SSDATA (val);
  memcpy (data as *mut c_void, SSDATA (encoded_prefix) as *const c_void, prefix_len as usize);
  memset (data.offset (prefix_len) as *mut c_void, b'X' as c_int, nX as usize);
  memcpy (data.offset (prefix_len + nX as isize) as *mut c_void,
	  SSDATA (encoded_suffix) as *const c_void, suffix_len as usize);
  let mut kind: c_int = if NILP (dir_flag) { GT_FILE }
    else if BASE_EQ (dir_flag, make_fixnum (0)) { GT_NOCREATE }
    else { GT_DIR };
  let mut fd: c_int = gen_tempname (data, suffix_len as c_int, O_BINARY | O_CLOEXEC, kind);
  let mut failed: bool = fd < 0;
  if !failed
    {
      let mut count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_protect_int (Some (close_file_unwind), fd);
      val = DECODE_FILE (val);
      if STRINGP (text) && SBYTES (text) != 0
	{ write_region (text, Qnil (), val, Qnil (), Qnil (), Qnil (), Qnil (), fd); }
      failed = NILP (dir_flag) && emacs_close (fd) != 0;
      /* Discard the unwind protect.  */
      (specpdl_ptr! ()) = specpdl_ref_to_ptr (count);
    }
  if failed
    {
      /* C 800-806: `static char const kind_message[][32] = { [GT_FILE] = … };` */
      static kind_message: [[c_char; 32]; 3] =
	[
	  c_str_32 (b"Creating file with prefix"),
	  c_str_32 (b"Creating directory with prefix"),
	  c_str_32 (b"Creating file name with prefix"),
	];
      report_file_error (kind_message[kind as usize].as_ptr (), prefix);
    }
  return val;
});

DEFUN! ("make-temp-name", Fmake_temp_name, Smake_temp_name, 1, 1, 0,
       doc: "Generate temporary file name (string) starting with PREFIX (a string).\n\nThis function tries to choose a name that has no existing file.\nFor this to work, PREFIX should be an absolute file name, and PREFIX\nand the returned string should both be non-magic.\n\nThere is a race condition between calling `make-temp-name' and\nlater creating the file, which opens all kinds of security holes.\nFor that reason, you should normally use `make-temp-file' instead.",
       (prefix: Lisp_Object),
       {
  return Fmake_temp_file_internal (prefix, make_fixnum (0),
				   empty_unibyte_string, Qnil ());
});

DEFUN! ("file-name-concat", Ffile_name_concat, Sfile_name_concat, 1, MANY, 0,
       doc: "Append COMPONENTS to DIRECTORY and return the resulting string.\nEach element in COMPONENTS must be a string or nil.\nDIRECTORY or the non-final elements in COMPONENTS may or may not end\nwith a slash -- if they don't end with a slash, a slash will be\ninserted before concatenating.\nIn most cases, one or more calls to `expand-file-name' are better\nsuited for the job than this function.  Use this function only if\nsome of the special expansions done by `expand-file-name' get in\nthe way of what your program needs to do.\nusage: (file-name-concat DIRECTORY &rest COMPONENTS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut chars: ptrdiff_t = 0;
  let mut bytes: ptrdiff_t = 0;
  let mut multibytes: ptrdiff_t = 0;
  let mut eargs: ptrdiff_t = 0;
  let mut elements: *mut Lisp_Object = args;
  let mut result: Lisp_Object;
  let mut i: ptrdiff_t;

  /* First go through the list to check the types and see whether
     they're all of the same multibyteness. */
  i = 0;
  while i < nargs
    {
      let mut arg: Lisp_Object = *args.offset (i);
      /* Skip empty and nil elements. */
      if NILP (arg)
	{ i += 1; continue; }
      CHECK_STRING (arg);
      if SCHARS (arg) == 0
	{ i += 1; continue; }
      eargs += 1;
      /* Multibyte and non-ASCII. */
      if STRING_MULTIBYTE (arg) && SCHARS (arg) != SBYTES (arg)
	{ multibytes += 1; }
      /* We're not adding a slash to the final part. */
      if i == nargs - 1
	|| IS_DIRECTORY_SEP (*SSDATA (arg).offset (SBYTES (arg) - 1) as c_int)
	{
	  bytes += SBYTES (arg);
	  chars += SCHARS (arg);
	}
      else
	{
	  bytes += SBYTES (arg) + 1;
	  chars += SCHARS (arg) + 1;
	}
      i += 1;
    }

  /* Convert if needed. */
  if (multibytes != 0 && multibytes != nargs)
    || eargs != nargs
    {
      let mut j: c_int = 0;
      elements = xmalloc (eargs as usize * std::mem::size_of::<Lisp_Object> () as usize)
	as *mut Lisp_Object;
      bytes = 0;
      chars = 0;

      /* Filter out nil/"". */
      i = 0;
      while i < nargs
	{
	  let mut arg: Lisp_Object = *args.offset (i);
	  if !NILP (arg) && SCHARS (arg) != 0
	    { *elements.offset (j as isize) = arg; j += 1; }
	  i += 1;
	}

      i = 0;
      while i < eargs
	{
	  let mut arg: Lisp_Object = *elements.offset (i);
	  /* Use multibyte or all-ASCII strings as is. */
	  if !STRING_MULTIBYTE (arg) && !string_ascii_p (arg)
	    { *elements.offset (i) = Fstring_to_multibyte (arg); }
	  arg = *elements.offset (i);
	  /* We have to recompute the number of bytes. */
	  if i == eargs - 1
	    || IS_DIRECTORY_SEP (*SSDATA (arg).offset (SBYTES (arg) - 1) as c_int)
	    {
	      bytes += SBYTES (arg);
	      chars += SCHARS (arg);
	    }
	  else
	    {
	      bytes += SBYTES (arg) + 1;
	      chars += SCHARS (arg) + 1;
	    }
	  i += 1;
	}
    }

  /* Allocate an empty string. */
  if multibytes == 0
    { result = make_uninit_string (chars); }
  else
    { result = make_uninit_multibyte_string (chars, bytes); }
  /* Null-terminate the string. */
  *SSDATA (result).offset (SBYTES (result)) = 0;

  /* Copy over the data. */
  let mut p: *mut c_char = SSDATA (result);
  i = 0;
  while i < eargs
    {
      let mut arg: Lisp_Object = *elements.offset (i);
      memcpy (p as *mut c_void, SSDATA (arg) as *const c_void, SBYTES (arg) as usize);
      p = p.offset (SBYTES (arg));
      /* The last element shouldn't have a slash added at the end. */
      if i < eargs - 1 && ! IS_DIRECTORY_SEP (*p.offset (-1) as c_int)
	{ *p = DIRECTORY_SEP; p = p.offset (1); }
      i += 1;
    }

  if elements != args
    { xfree (elements as *mut c_void); }

  return result;
});

/* NAME must be a string.  */
unsafe fn file_name_absolute_no_tilde_p (name: Lisp_Object) -> bool
{
  return IS_ABSOLUTE_FILE_NAME! (SSDATA (name));
}

/* Return the home directory of the user NAME, or a null pointer if
   NAME is empty or the user does not exist or the user's home
   directory is not an absolute file name.  NAME is an array of bytes
   that continues up to (but not including) the next NUL byte or
   directory separator.  The returned string lives in storage good
   until the next call to this or similar functions.  */
unsafe fn user_homedir (name: *const c_char) -> *mut c_char
{
  let mut length: ptrdiff_t;
  length = 0;
  while *name.offset (length) != 0 && ! IS_DIRECTORY_SEP (*name.offset (length) as c_int)
    { length += 1; }
  if length == 0
    { return std::ptr::null_mut (); }
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut p: *mut c_char = SAFE_ALLOCA! (sa_avail, length + 1) as *mut c_char;
  memcpy (p as *mut c_void, name as *const c_void, length as usize);
  *p.offset (length) = 0;
  let mut pw: *mut passwd = getpwnam (p);
  SAFE_FREE! (sa_count, sa_mark);
  /* C 986-988: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (HAVE_ANDROID undef):
     recorded in `docs/ports/fileio.md` (P-08).  */
  if pw.is_null () || (!(*pw).pw_dir.is_null () && ! IS_ABSOLUTE_FILE_NAME! ((*pw).pw_dir))
    { return std::ptr::null_mut (); }
  return (*pw).pw_dir;
}

DEFUN! ("expand-file-name", Fexpand_file_name, Sexpand_file_name, 1, 2, 0,
       doc: "Convert filename NAME to absolute, and canonicalize it.\nSecond arg DEFAULT-DIRECTORY is directory to start with if NAME is relative\n(does not start with slash or tilde); both the directory name and\na directory's file name are accepted.  If DEFAULT-DIRECTORY is nil or\nmissing, the current buffer's value of `default-directory' is used.\nNAME should be a string that is a valid file name for the underlying\nfilesystem.\n\nFile name components that are `.' are removed, and so are file name\ncomponents followed by `..', along with the `..' itself; note that\nthese simplifications are done without checking the resulting file\nnames in the file system.\n\nMultiple consecutive slashes are collapsed into a single slash, except\nat the beginning of the file name when they are significant (e.g., UNC\nfile names on MS-Windows.)\n\nAn initial \"~\" in NAME expands to your home directory.\n\nAn initial \"~USER\" in NAME expands to USER's home directory.  If\nUSER doesn't exist, \"~USER\" is not expanded.\n\nTo do other file name substitutions, see `substitute-in-file-name'.\n\nFor technical reasons, this function can return correct but\nnon-intuitive results for the root directory; for instance,\n(expand-file-name \"..\" \"/\") returns \"/..\".  For this reason, use\n(directory-file-name (file-name-directory dirname)) to traverse a\nfilesystem tree, not (expand-file-name \"..\" dirname).  Note: make\nsure DIRNAME in this example doesn't end in a slash, unless it's\nthe root directory.",
       (name: Lisp_Object, default_directory: Lisp_Object),
       {
  /* These point to SDATA and need to be careful with string-relocation
     during GC (via DECODE_FILE).  */
  let mut nm: *mut c_char;
  let mut nmlim: *mut c_char;
  let mut newdir: *const c_char;
  let mut newdirlim: *const c_char;
  /* This should only point to alloca'd data.  */
  let mut target: *mut c_char;

  let mut tlen: ptrdiff_t;
  let mut length: ptrdiff_t;
  let mut nbytes: ptrdiff_t;
  let mut handler: Lisp_Object;
  let mut result: Lisp_Object;
  let mut handled_name: Lisp_Object;
  let mut multibyte: bool;
  let mut hdir: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut name = name;                        /* C reassigns `name` below.  */
  let mut default_directory = default_directory;  /* C reassigns it below.  */

  CHECK_STRING (name);
  CHECK_STRING_NULL_BYTES (name);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (name, Qexpand_file_name ());
  if !NILP (handler)
    {
      handled_name = calln! (handler, Qexpand_file_name (),
			     name, default_directory);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  /* As a last resort, we may have to use the root as
     default_directory below.  */
  let mut root: Lisp_Object;
  root = build_string (c"/".as_ptr ());

  /* Use the buffer's default-directory if DEFAULT_DIRECTORY is omitted.  */
  if NILP (default_directory)
    {
      let mut dir: Lisp_Object = BVAR! ((current_buffer! ()), directory_);
      /* The buffer's default-directory should be absolute or should
	 start with `~'.  If it isn't absolute, we replace it by its
	 expansion relative to a known absolute name ABSDIR, which is
	 the invocation-directory if the latter is absolute, or the
	 root otherwise.

	 In case default-directory starts with `~' or `~user', where
	 USER is a valid user name, this correctly expands it (and
	 ABSDIR plays no role).  If USER is not a valid user name, the
	 leading `~' loses its special meaning and is retained as part
	 of the expanded name.  */
      if STRINGP (dir)
	{
	  if file_name_absolute_no_tilde_p (dir)
	    {
	      CHECK_STRING_NULL_BYTES (dir);
	      default_directory = dir;
	    }
	  else
	    {
	      let mut absdir: Lisp_Object
		= if STRINGP (Vinvocation_directory)
		  && file_name_absolute_no_tilde_p (Vinvocation_directory)
		  { Vinvocation_directory } else { root };
	      default_directory = Fexpand_file_name (dir, absdir);
	    }
	}
    }
  if ! STRINGP (default_directory)
    { default_directory = root; }

  handler = Ffind_file_name_handler (default_directory, Qexpand_file_name ());
  if !NILP (handler)
    {
      handled_name = calln! (handler, Qexpand_file_name (),
			     name, default_directory);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  {
    let mut o: *mut c_char = SSDATA (default_directory);

    /* Make sure DEFAULT_DIRECTORY is properly expanded.
       It would be better to do this down below where we actually use
       default_directory.  Unfortunately, calling Fexpand_file_name recursively
       could invoke GC, and the strings might be relocated.  This would
       be annoying because we have pointers into strings lying around
       that would need adjusting, and people would add new pointers to
       the code and forget to adjust them, resulting in intermittent bugs.
       Putting this call here avoids all that crud.

       The EQ test avoids infinite recursion.  */
    if ! NILP (default_directory) && !EQ (default_directory, name)
	/* Save time in some common cases - as long as default_directory
	   is not relative, it can be canonicalized with name below (if it
	   is needed at all) without requiring it to be expanded now.  */
	&& ! (IS_DIRECTORY_SEP (*o.offset (0) as c_int))
      {
	default_directory = Fexpand_file_name (default_directory, Qnil ());

	/* The above expansion might have produced a remote file name,
	   so give the handlers one last chance to DTRT.  This can
	   happen when both NAME and DEFAULT-DIRECTORY arguments are
	   relative file names, and the buffer's default-directory is
	   remote.  */
	handler = Ffind_file_name_handler (default_directory,
					   Qexpand_file_name ());
	if !NILP (handler)
	  {
	    handled_name = calln! (handler, Qexpand_file_name (),
				   name, default_directory);
	    if STRINGP (handled_name)
	      { return handled_name; }
	    error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
	  }
      }
  }
  multibyte = STRING_MULTIBYTE (name);
  let mut defdir_multibyte: bool = STRING_MULTIBYTE (default_directory);
  if multibyte != defdir_multibyte
    {
      /* We want to make both NAME and DEFAULT_DIRECTORY have the same
	 multibyteness.  Strategy:
	 . If either NAME or DEFAULT_DIRECTORY is pure-ASCII, they
	   can be converted to the multibyteness of the other one
	   while keeping the same byte sequence.
	 . If both are non-ASCII, the only safe conversion is to
	   convert the multibyte one to be unibyte, because the
	   reverse conversion potentially adds bytes while raw bytes
	   are converted to their multibyte forms, which we will be
	   unable to account for, since the information about the
	   original multibyteness is lost.  If those additional bytes
	   later leak to system APIs because they are not encoded or
	   because they are converted to unibyte strings by keeping
	   the data, file APIs will fail.

	 Note: One could argue that if we see a multibyte string, it
	 is evidence that file-name decoding was already set up, and
	 we could convert unibyte strings to multibyte using
	 DECODE_FILE.  However, this is risky, because the likes of
	 string_to_multibyte are able of creating multibyte strings
	 without any decoding.  */
      if multibyte
	{
	  let mut name_ascii_p: bool = SCHARS (name) == SBYTES (name);
	  let mut p: *mut u8 = SDATA (default_directory);

	  if !name_ascii_p
	    {
	      while *p != 0 && ASCII_CHAR_P (*p as isize)
		{ p = p.offset (1); }
	    }
	  if name_ascii_p || *p != 0
	    {
	      /* DEFAULT_DIRECTORY is unibyte and possibly non-ASCII.
		 Make a unibyte string out of NAME, and arrange for
		 the result of this function to be a unibyte string.
		 This is needed during bootstrapping and dumping, when
		 the program cannot decode file names, because the locale
		 environment is not set up.  */
	      name = make_unibyte_string (SSDATA (name), SBYTES (name));
	      multibyte = false;
	    }
	  else
	    {
	      /* NAME is non-ASCII and multibyte, and
		 DEFAULT_DIRECTORY is unibyte and pure-ASCII: make a
		 multibyte string out of DEFAULT_DIRECTORY's data.  */
	      default_directory =
		make_multibyte_string (SSDATA (default_directory),
				       SCHARS (default_directory),
				       SCHARS (default_directory));
	    }
	}
      else
	{
	  let mut p: *mut u8 = SDATA (name);

	  while *p != 0 && ASCII_CHAR_P (*p as isize)
	    { p = p.offset (1); }
	  if *p == 0
	    {
	      /* DEFAULT_DIRECTORY is multibyte and NAME is unibyte
		 and pure-ASCII.  Make a multibyte string out of
		 NAME's data.  */
	      name = make_multibyte_string (SSDATA (name),
					    SCHARS (name), SCHARS (name));
	      multibyte = true;
	    }
	  else
	    { default_directory = make_unibyte_string (SSDATA (default_directory),
						       SBYTES (default_directory)); }
	}
    }

  /* Make a local copy of NAME to protect it from GC in DECODE_FILE below.  */
  SAFE_ALLOCA_STRING! (sa_avail, nm, name);
  nmlim = nm.offset (SBYTES (name));

  /* If nm is absolute, look for `/./' or `/../' or `//''sequences; if
     none are found, we can probably return right away.  We will avoid
     allocating a new string if name is already fully expanded.  */
  if IS_DIRECTORY_SEP (*nm.offset (0) as c_int)
    {
      /* If it turns out that the filename we want to return is just a
	 suffix of FILENAME, we don't need to go through and edit
	 things; we just need to construct a new string using data
	 starting at the middle of FILENAME.  If we set LOSE, that
	 means we've discovered that we can't do that cool trick.  */
      let mut lose: bool = false;
      let mut p: *mut c_char = nm;

      while *p != 0
	{
	  /* Since we know the name is absolute, we can assume that each
	     element starts with a "/".  */

	  /* "." and ".." are hairy.  */
	  if IS_DIRECTORY_SEP (*p.offset (0) as c_int)
	    && *p.offset (1) == b'.' as c_char
	    && (IS_DIRECTORY_SEP (*p.offset (2) as c_int)
		|| *p.offset (2) == 0
		|| (*p.offset (2) == b'.' as c_char
		    && (IS_DIRECTORY_SEP (*p.offset (3) as c_int)
			|| *p.offset (3) == 0)))
	    { lose = true; }
	  /* Replace multiple slashes with a single one, except
	     leave leading "//" alone.  */
	  else if IS_DIRECTORY_SEP (*p.offset (0) as c_int)
	    && IS_DIRECTORY_SEP (*p.offset (1) as c_int)
	    && (p != nm || IS_DIRECTORY_SEP (*p.offset (2) as c_int))
	    { lose = true; }
	  p = p.offset (1);
	}
      if !lose
	{
	  if strcmp (nm, SSDATA (name)) != 0
	    { name = make_specified_string (nm, -1, nmlim.offset_from (nm), multibyte); }
	  SAFE_FREE! (sa_count, sa_mark);
	  return name;
	}
    }

  /* At this point, nm might or might not be an absolute file name.  We
     need to expand ~ or ~user if present, otherwise prefix nm with
     default_directory if nm is not absolute, and finally collapse /./
     and /foo/../ sequences.

     We set newdir to be the appropriate prefix if one is needed:
       - the relevant user directory if nm starts with ~ or ~user
       - the specified drive's working dir (DOS/NT only) if nm does not
         start with /
       - the value of default_directory.

     Note that these prefixes are not guaranteed to be absolute (except
     for the working dir of a drive).  Therefore, to ensure we always
     return an absolute name, if the final prefix is not absolute we
     append it to the current working directory.  */

  newdir = std::ptr::null ();
  newdirlim = std::ptr::null ();

  if *nm.offset (0) == b'~' as c_char		/* prefix ~ */
    {
      if IS_DIRECTORY_SEP (*nm.offset (1) as c_int)
	|| *nm.offset (1) == 0	/* ~ by itself */
	{
	  let mut tem: Lisp_Object;

	  newdir = get_homedir ();
	  nm = nm.offset (1);
	  tem = build_string (newdir);
	  newdirlim = newdir.offset (SBYTES (tem));
	  /* get_homedir may return a unibyte string, which will bite us
	     if we expect the directory to be multibyte.  */
	  if multibyte && !STRING_MULTIBYTE (tem)
	    {
	      hdir = DECODE_FILE (tem);
	      newdir = SSDATA (hdir);
	      newdirlim = newdir.offset (SBYTES (hdir));
	    }
	  else if !multibyte && STRING_MULTIBYTE (tem)
	    { multibyte = true; }
	}
      else			/* ~user/filename */
	{
	  let mut nmhome: *mut c_char = user_homedir (nm.offset (1));
	  if !nmhome.is_null ()
	    {
	      let mut nmhomelen: ptrdiff_t = strlen (nmhome) as ptrdiff_t;
	      newdir = nmhome;
	      newdirlim = newdir.offset (nmhomelen);
	      if multibyte
		{
		  AUTO_STRING_WITH_LEN! (lisp_nmhome, nmhome, nmhomelen);
		  hdir = DECODE_FILE (lisp_nmhome);
		  newdir = SSDATA (hdir);
		  newdirlim = newdir.offset (SBYTES (hdir));
		}

	      nm = nm.offset (1);
	      while *nm != 0 && ! IS_DIRECTORY_SEP (*nm as c_int)
		{ nm = nm.offset (1); }
	    }

	  /* If we don't find a user of that name, leave the name
	     unchanged.  */
	}
    }

  /* Finally, if no prefix has been specified and nm is not absolute,
     then it must be expanded relative to default_directory.  */

  if ! IS_DIRECTORY_SEP (*nm.offset (0) as c_int)
    && newdir.is_null ()
    {
      newdir = SSDATA (default_directory);
      newdirlim = newdir.offset (SBYTES (default_directory));
    }

  /* Ignore any slash at the end of newdir, unless newdir is
     just "/" or "//".  */
  length = newdirlim.offset_from (newdir);
  while length > 1 && IS_DIRECTORY_SEP (*newdir.offset (length - 1) as c_int)
    && ! (length == 2 && IS_DIRECTORY_SEP (*newdir.offset (0) as c_int))
    { length -= 1; }

  /* Now concatenate the directory and name to new space in the stack frame.  */
  tlen = length + file_name_as_directory_slop as isize + nmlim.offset_from (nm) + 1;
  eassert! (tlen >= file_name_as_directory_slop as isize + 1);
  target = SAFE_ALLOCA! (sa_avail, tlen) as *mut c_char;
  *target = 0;
  nbytes = 0;

  if !newdir.is_null ()
    {
      if *nm.offset (0) == 0 || IS_DIRECTORY_SEP (*nm.offset (0) as c_int)
	{
	  memcpy (target as *mut c_void, newdir as *const c_void, length as usize);
	  *target.offset (length) = 0;
	  nbytes = length;
	}
      else
	{ nbytes = file_name_as_directory (target, newdir, length, multibyte); }
    }

  memcpy (target.offset (nbytes) as *mut c_void, nm as *const c_void,
	  nmlim.offset_from (nm) as usize + 1);

  /* Now canonicalize by removing `//', `/.' and `/foo/..' if they
     appear.  */
  {
    let mut p: *mut c_char = target;
    let mut o: *mut c_char = target;

    while *p != 0
      {
	if ! IS_DIRECTORY_SEP (*p as c_int)
	  {
	    *o = *p; o = o.offset (1); p = p.offset (1);
	  }
	else if *p.offset (1) == b'.' as c_char
		 && (IS_DIRECTORY_SEP (*p.offset (2) as c_int)
		     || *p.offset (2) == 0)
	  {
	    /* If "/." is the entire filename, keep the "/".  Otherwise,
	       just delete the whole "/.".  */
	    if o == target && *p.offset (2) == 0
	      { *o = *p; o = o.offset (1); }
	    p = p.offset (2);
	  }
	else if *p.offset (1) == b'.' as c_char && *p.offset (2) == b'.' as c_char
		 /* `/../' is the "superroot" on certain file systems.  */
		 && o != target
		 && (IS_DIRECTORY_SEP (*p.offset (3) as c_int) || *p.offset (3) == 0)
	  {
	    while o != target && { o = o.offset (-1); ! IS_DIRECTORY_SEP (*o as c_int) }
	      { }
	    /* Keep initial / only if this is the whole name.  */
	    if o == target && IS_ANY_SEP! (*o as c_int) && *p.offset (3) == 0
	      { o = o.offset (1); }
	    p = p.offset (3);
	  }
	else if IS_DIRECTORY_SEP (*p.offset (1) as c_int)
		 && (p != target || IS_DIRECTORY_SEP (*p.offset (2) as c_int))
	  /* Collapse multiple "/", except leave leading "//" alone.  */
	  { p = p.offset (1); }
	else
	  {
	    *o = *p; o = o.offset (1); p = p.offset (1);
	  }
      }

    result = make_specified_string (target, -1, o.offset_from (target), multibyte);
  }

  /* Again look to see if the file name has special constructs in it
     and perhaps call the corresponding file name handler.  This is needed
     for filenames such as "/foo/../user@host:/bar/../baz".  Expanding
     the ".." component gives us "/user@host:/bar/../baz" which needs
     to be expanded again.  */
  handler = Ffind_file_name_handler (result, Qexpand_file_name ());
  if !NILP (handler)
    {
      handled_name = calln! (handler, Qexpand_file_name (),
			     result, default_directory);
      if ! STRINGP (handled_name)
	{ error (c"Invalid handler in `file-name-handler-alist'".as_ptr ()); }
      result = handled_name;
    }

  SAFE_FREE! (sa_count, sa_mark);
  return result;
});

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

/// C's `static char kind_message[][32]` needs its element type; this const fn is the port's
/// way to spell an initializer for `[c_char; 32]` (no C name).
const fn c_str_32 (s: &[u8]) -> [c_char; 32] {
  let mut out: [c_char; 32] = [0; 32];
  let mut i = 0;
  while i < s.len () { out[i] = s[i] as c_char; i += 1; }
  out
}

unsafe extern "C" {
  /// C: lisp.h 4223: `extern bool string_ascii_p (Lisp_Object);`
  pub fn string_ascii_p (string: Lisp_Object) -> bool;
  /// C: lisp.h 4267: `extern Lisp_Object do_yes_or_no_p (Lisp_Object);`
  pub fn do_yes_or_no_p (arg: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5303: `extern int emacs_fstatat (int, char const *, void *, int);`
  pub fn emacs_fstatat (dirfd: c_int, file: *const c_char, buf: *mut c_void, flag: c_int)
    -> c_int;
  /// C: lisp.h 5131: `extern char const *emacs_wd;` - emacs.c defines it, and `lisp.rs` does
  /// not carry the declaration (the buffer.rs note; same type as that file's declaration of
  /// the emacs.rs definition).
  pub static mut emacs_wd: *const c_char;
}

/* Put into BUF the concatenation of DIR and FILE, with an intervening
   directory separator if needed.  Return a pointer to the null byte
   at the end of the concatenated string.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn splice_dir_file (buf: *mut c_char, dir: *const c_char,
					  file: *const c_char) -> *mut c_char
{
  let mut e: *mut c_char = stpcpy (buf, dir);
  *e = DIRECTORY_SEP;
  e = e.offset ((! (buf < e && IS_DIRECTORY_SEP (*e.offset (-1) as c_int))) as isize);
  return stpcpy (e, file);
}

/* Get the home directory, an absolute file name.  Return the empty
   string on failure.  The returned value does not survive garbage
   collection, calls to this function, or calls to the getpwnam class
   of functions.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_homedir () -> *const c_char
{
  let mut home: *const c_char = egetenv (c"HOME".as_ptr ());

  /* C 1954-1965: `#ifdef WINDOWSNT` (undefined): recorded in `docs/ports/fileio.md`.  */

  if home.is_null ()
    {
      static mut userenv: [*const c_char; 2] = [c"LOGNAME".as_ptr (), c"USER".as_ptr ()];
      let mut pw: *mut passwd = std::ptr::null_mut ();
      let mut i: c_int = 0;
      while i < countof! (userenv) as c_int
	{
	  let mut user: *mut c_char = egetenv (userenv[i as usize]);
	  if !user.is_null ()
	    {
	      pw = getpwnam (user);
	      if !pw.is_null ()
		{ break; }
	    }
	  i += 1;
	}
      if pw.is_null ()
	{ pw = getpwuid (getuid ()); }
      if !pw.is_null ()
	{ home = (*pw).pw_dir; }

      /* C 1975-1978: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (HAVE_ANDROID
	 undef): recorded in `docs/ports/fileio.md`.  */
      if home.is_null ()
	{ return c"".as_ptr (); }
    }
  /* C 1980-2007: `#ifdef DOS_NT` (undefined): recorded in `docs/ports/fileio.md`.  */
  if IS_ABSOLUTE_FILE_NAME! (home)
    { return home; }
  let wd: *const c_char = emacs_wd;
  if wd.is_null ()
    { error (c"$HOME is relative to unknown directory".as_ptr ()); }
  static mut ahome: *mut c_char = std::ptr::null_mut ();
  static mut ahomesize: ptrdiff_t = 0;
  let mut ahomelenbound: ptrdiff_t = strlen (wd) as ptrdiff_t + 1 + strlen (home) as ptrdiff_t + 1;
  if ahomesize <= ahomelenbound
    { ahome = xpalloc (ahome as *mut c_void, &raw mut ahomesize,
		       ahomelenbound + 1 - ahomesize, -1, 1) as *mut c_char; }
  splice_dir_file (ahome, wd, home);
  return ahome;
}

/* If a directory separator followed by an absolute file name (e.g.,
   "//foo", "/~", "/~someuser") appears in NM, return the address of
   the absolute file name.  Otherwise return NULL.  ENDP is the
   address of the null byte at the end of NM.  */
unsafe fn search_embedded_absfilename (nm: *mut c_char, endp: *mut c_char)
  -> *mut c_char
{
  let mut p: *mut c_char = nm.offset (1);
  /* C 2027-2029: `#ifdef DOUBLE_SLASH_IS_DISTINCT_ROOT` (undefined on the frozen target):
     recorded in `docs/ports/fileio.md`.  */
  while p < endp
    {
      if IS_DIRECTORY_SEP (*p.offset (-1) as c_int) && file_name_absolute_p (p)
	{ return p; }
      p = p.offset (1);
    }
  return std::ptr::null_mut ();
}

DEFUN! ("substitute-in-file-name", Fsubstitute_in_file_name,
       Ssubstitute_in_file_name, 1, 1, 0,
       doc: "Substitute environment variables referred to in FILENAME.\n`$FOO' where FOO is an environment variable name means to substitute\nthe value of that variable.  The variable name should be terminated\nwith a character not a letter, digit or underscore; otherwise, enclose\nthe entire variable name in braces.\n\nIf FOO is not defined in the environment, `$FOO' is left unchanged in\nthe value of this function.\n\nIf `/~' appears, all of FILENAME through that `/' is discarded.\nIf `//' appears, everything up to and including the first of\nthose `/' is discarded.  More generally, if a variable substitution\nproduces an absolute file name, everything before that file name\nis discarded.",
       (filename: Lisp_Object),
       {
  let mut nm: *mut c_char;
  let mut p: *mut c_char;
  let mut x: *mut c_char;
  let mut endp: *mut c_char;
  let mut substituted: bool = false;
  let mut multibyte: bool;
  let mut xnm: *mut c_char;
  let mut handler: Lisp_Object;
  let mut filename = filename;                /* C reassigns `filename` below.  */

  CHECK_STRING (filename);

  multibyte = STRING_MULTIBYTE (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qsubstitute_in_file_name ());
  if !NILP (handler)
    {
      let mut handled_name: Lisp_Object = calln! (handler, Qsubstitute_in_file_name (),
						  filename);
      if STRINGP (handled_name)
	{ return handled_name; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  /* Always work on a copy of the string, in case GC happens during
     decode of environment variables, causing the original Lisp_String
     data to be relocated.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  SAFE_ALLOCA_STRING! (sa_avail, nm, filename);

  /* C 2088-2091: `#ifdef DOS_NT` (undefined): recorded in `docs/ports/fileio.md`.  */
  endp = nm.offset (SBYTES (filename));

  /* If /~ or // appears, discard everything through first slash.  */
  p = search_embedded_absfilename (nm, endp);
  if !p.is_null ()
    /* Start over with the new string, so we check the file-name-handler
       again.  Important with filenames like "/home/foo//:/hello///there"
       which would substitute to "/:/hello///there" rather than "/there".  */
    {
      let mut result: Lisp_Object
	= Fsubstitute_in_file_name
	  (make_specified_string (p, -1, endp.offset_from (p), multibyte));
      SAFE_FREE! (sa_count, sa_mark);
      return result;
    }

  /* See if any variables are substituted into the string.  */

  if !NILP (Ffboundp (Qsubstitute_env_in_file_name ()))
    {
      let mut name: Lisp_Object
	= if !substituted { filename }
	  else { make_specified_string (nm, -1, endp.offset_from (nm), multibyte) };
      let mut tmp: Lisp_Object = calln! (Qsubstitute_env_in_file_name (), name);
      CHECK_STRING (tmp);
      if !EQ (tmp, name)
	{ substituted = true; }
      filename = tmp;
    }

  if !substituted
    {
      /* C 2130-2133: `#ifdef WINDOWSNT` (undefined): recorded.  */
      SAFE_FREE! (sa_count, sa_mark);
      return filename;
    }

  xnm = SSDATA (filename) as *mut c_char;
  x = xnm.offset (SBYTES (filename));

  /* If /~ or // appears, discard everything through first slash.  */
  while { p = search_embedded_absfilename (xnm, x); !p.is_null () }
    /* This time we do not start over because we've already expanded envvars
       and replaced $$ with $.  Maybe we should start over as well, but we'd
       need to quote some $ to $$ first.  */
    { xnm = p; }

  /* C 2150-2158: `#ifdef WINDOWSNT` (undefined): recorded.  */
  if xnm != SSDATA (filename) as *mut c_char
    { filename = make_specified_string (xnm, -1, x.offset_from (xnm), multibyte); }
  SAFE_FREE! (sa_count, sa_mark);
  return filename;
});

/* A slightly faster and more convenient way to get
   (directory-file-name (expand-file-name FOO)).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn expand_and_dir_to_file (filename: Lisp_Object) -> Lisp_Object
{
  let mut absname: Lisp_Object = Fexpand_file_name (filename, Qnil ());

  /* Remove final slash, if any (unless this is the root dir).
     stat behaves differently depending!  */
  if SCHARS (absname) > 1
    && IS_DIRECTORY_SEP (SREF (absname, (SBYTES (absname) - 1) as usize) as c_int)
    && ! IS_DEVICE_SEP (SREF (absname, (SBYTES (absname) - 2) as usize) as c_int)
    /* We cannot take shortcuts; they might be wrong for magic file names.  */
    { absname = Fdirectory_file_name (absname); }
  return absname;
}

/* Signal an error if the file ABSNAME already exists.
   If KNOWN_TO_EXIST, the file is known to exist.
   QUERYSTRING is a name for the action that is being considered
   to alter the file.
   If INTERACTIVE, ask the user whether to proceed,
   and bypass the error if the user says to go ahead.
   If QUICK, ask for y or n, not yes or no.  */

unsafe fn barf_or_query_if_file_exists (absname: Lisp_Object,
					       mut known_to_exist: bool,
					       querystring: *const c_char,
					       interactive: bool, quick: bool)
{
  let mut tem: Lisp_Object;
  let mut encoded_filename: Lisp_Object;
  let mut statbuf: stat = std::mem::zeroed ();

  encoded_filename = ENCODE_FILE (absname);

  if ! known_to_exist
    && (emacs_fstatat (AT_FDCWD, SSDATA (encoded_filename),
		       &raw mut statbuf as *mut c_void, AT_SYMLINK_NOFOLLOW)
	== 0)
    {
      if S_ISDIR! (statbuf.st_mode)
	{ xsignal2 (Qfile_error (),
		    build_string (c"File is a directory".as_ptr ()), absname); }
      known_to_exist = true;
    }

  if known_to_exist
    {
      if ! interactive
	{ xsignal2 (Qfile_already_exists (),
		    build_string (c"File already exists".as_ptr ()), absname); }
      AUTO_STRING! (format, c"File %s already exists; %s anyway? ".as_ptr ());
      tem = CALLN! (Fformat, format, absname, build_string (querystring));
      if quick
	{ tem = calln! (Qy_or_n_p (), tem); }
      else
	{ tem = do_yes_or_no_p (tem); }
      if NILP (tem)
	{ xsignal2 (Qfile_already_exists (),
		    build_string (c"File already exists".as_ptr ()), absname); }
    }
}

/* Read a full buffer of bytes from FD into BUF, which is of size BUFSIZE.
   FD should be a regular file, as special files might need quit processing.
   On success, return the read count, which is less than BUFSIZE at EOF.
   Return -1 on failure, setting errno and possibly setting BUF.  */
unsafe fn emacs_full_read (fd: emacs_fd, buf: *mut c_void, bufsize: ptrdiff_t)
  -> ptrdiff_t
{
  let mut b: *mut c_char = buf as *mut c_char;
  let mut nread: ptrdiff_t = 0;
  let mut r: ptrdiff_t;
  while 0 < { r = emacs_fd_read! (fd, b.offset (nread) as *mut c_void, bufsize - nread); r }
	&& { nread += r; nread < bufsize }
    { }
  return if r < 0 { r } else { nread };
}

/* C 2242: `#ifndef WINDOWSNT` (undefined, so the text is live).  */
/* Copy data to DEST from SOURCE if possible.  Return true if OK.  */
unsafe fn clone_file (dest: c_int, source: c_int) -> bool
{
  /* C 2245-2248: `#ifdef FICLONE` (undefined on the frozen target): recorded.  */
  return false;
}

DEFUN! ("copy-file", Fcopy_file, Scopy_file, 2, 6,
       "fCopy file: \nGCopy %s to file: \np\nP",
       doc: "Copy FILE to NEWNAME.  Both args must be strings.\nIf NEWNAME is a directory name, copy FILE to a like-named file under\nNEWNAME.  For NEWNAME to be recognized as a directory name, it should\nend in a slash.\n\nThis function always sets the file modes of the output file to match\nthe input file.\n\nThe optional third argument OK-IF-ALREADY-EXISTS specifies what to do\nif file NEWNAME already exists.  If OK-IF-ALREADY-EXISTS is nil,\nsignal a `file-already-exists' error without overwriting.  If\nOK-IF-ALREADY-EXISTS is an integer, request confirmation from the user\nabout overwriting; this is what happens in interactive use with M-x.\nAny other value for OK-IF-ALREADY-EXISTS means to overwrite the\nexisting file.\n\nFourth arg KEEP-TIME non-nil means give the output file the same\nlast-modified time as the old one.  (This works on only some systems.)\n\nA prefix arg makes KEEP-TIME non-nil.\n\nIf PRESERVE-UID-GID is non-nil, try to transfer the uid and gid of\nFILE to NEWNAME.\n\nIf PRESERVE-PERMISSIONS is non-nil, copy permissions of FILE to NEWNAME;\nthis includes the file modes, along with ACL entries and SELinux\ncontext if present.  Otherwise, if NEWNAME is created its file\npermission bits are those of FILE, masked by the default file\npermissions.",
       (file: Lisp_Object, newname: Lisp_Object, ok_if_already_exists: Lisp_Object,
        keep_time: Lisp_Object, preserve_uid_gid: Lisp_Object,
        preserve_permissions: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut encoded_file: Lisp_Object;
  let mut encoded_newname: Lisp_Object;
  let mut already_exists: bool = false;
  let mut new_mask: mode_t;
  let mut ifd: emacs_fd;
  let mut ofd: c_int;
  let mut st: stat = std::mem::zeroed ();
  let mut file = file;                        /* C reassigns `file` below.  */
  let mut newname = newname;                  /* C reassigns `newname` below.  */

  file = Fexpand_file_name (file, Qnil ());
  newname = expand_cp_target (file, newname);

  /* If the input file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qcopy_file ());
  /* Likewise for output file name.  */
  if NILP (handler)
    { handler = Ffind_file_name_handler (newname, Qcopy_file ()); }
  if !NILP (handler)
    { return calln! (handler, Qcopy_file (), file, newname,
		     ok_if_already_exists, keep_time, preserve_uid_gid,
		     preserve_permissions); }

  encoded_file = ENCODE_FILE (file);
  encoded_newname = ENCODE_FILE (newname);

  /* C 2314-2334: `#ifdef WINDOWSNT` (undefined): recorded in `docs/ports/fileio.md`.  */
  ifd = emacs_fd_open! (SSDATA (encoded_file), O_RDONLY | O_NONBLOCK, 0);

  if !emacs_fd_valid_p! (ifd)
    { report_file_error (c"Opening input file".as_ptr (), file); }

  record_unwind_protect_ptr (Some (close_file_unwind_emacs_fd), &raw mut ifd as *mut c_void);

  if emacs_fd_fstat! (ifd, &raw mut st) != 0
    { report_file_error (c"Input file status".as_ptr (), file); }

  if !NILP (preserve_permissions)
    {
      /* C 2349-2371: `#if HAVE_LIBSELINUX` (undefined): recorded in `docs/ports/fileio.md`.  */
    }

  /* We can copy only regular files.  */
  if !S_ISREG! (st.st_mode)
    { report_file_errno (c"Non-regular file".as_ptr (), file,
			 if S_ISDIR! (st.st_mode) { EISDIR } else { EINVAL }); }

  /* C 2380-2384: `#ifndef MSDOS` (live) / `#else` (MSDOS undef).  */
  new_mask = st.st_mode & (if !NILP (preserve_uid_gid) { 0o700 } else { 0o777 });

  ofd = emacs_open (SSDATA (encoded_newname), O_WRONLY | O_CREAT | O_EXCL,
		    new_mask as c_int);
  if ofd < 0 && *errno () == EEXIST
    {
      if NILP (ok_if_already_exists) || FIXNUMP (ok_if_already_exists)
	{ barf_or_query_if_file_exists (newname, true, c"copy to it".as_ptr (),
					FIXNUMP (ok_if_already_exists), false); }
      already_exists = true;
      ofd = emacs_open (SSDATA (encoded_newname), O_WRONLY | O_TRUNC, 0);
    }
  if ofd < 0
    { report_file_error (c"Opening output file".as_ptr (), newname); }

  record_unwind_protect_int (Some (close_file_unwind), ofd);

  if already_exists
    {
      let mut out_st: stat = std::mem::zeroed ();
      if sys_fstat (ofd, &raw mut out_st) != 0
	{ report_file_error (c"Output file status".as_ptr (), newname); }
      if st.st_dev == out_st.st_dev && st.st_ino == out_st.st_ino
	{ report_file_errno (c"Input and output files are the same".as_ptr (),
			     list2 (file, newname), 0); }
    }

  maybe_quit ();

  if emacs_fd_to_int! (ifd) == -1
    || !clone_file (ofd, emacs_fd_to_int! (ifd))
    {
      let mut newsize: off_t = 0;      /* C: `MAYBE_UNUSED off_t newsize = 0;`  */

      /* C 2416: `#ifndef MSDOS` (live).  */
      if emacs_fd_to_int! (ifd) != -1
	{
	  let mut copied: ssize_t;
	  loop
	    {
	      /* Copy at most COPY_MAX bytes at a time; this is min
		 (SSIZE_MAX, SIZE_MAX) truncated to a value that is
		 surely aligned well.  */
	      let mut copy_max: ssize_t
		= (std::cmp::min (SSIZE_MAX as usize, SIZE_MAX) >> 30 << 30) as ssize_t;
	      copied = copy_file_range (emacs_fd_to_int! (ifd), std::ptr::null_mut (),
					ofd, std::ptr::null_mut (), copy_max as usize, 0);
	      if copied <= 0
		{ break; }
	      maybe_quit ();
	      newsize += copied as off_t;
	    }
	}

      /* Follow up with read+write regardless of any copy_file_range failure.
	 Many copy_file_range implementations fail for no good reason,
	 or "succeed" even when they did nothing (e.g., in /proc files).
	 Also, if read+write fails it will report an error more
	 precisely than copy_file_range would.  */
      let mut buf: [c_char; MAX_ALLOCA as usize] = std::mem::zeroed ();

      let mut copied: ptrdiff_t;
      copied = emacs_full_read (ifd, buf.as_mut_ptr () as *mut c_void,
				std::mem::size_of_val (&buf) as ptrdiff_t);
      while copied != 0
	{
	  if copied < 0
	    { report_file_error (c"Read error".as_ptr (), file); }
	  if emacs_write_quit (ofd, buf.as_mut_ptr () as *const c_void, copied) != copied
	    { report_file_error (c"Write error".as_ptr (), newname); }
	  if copied < std::mem::size_of_val (&buf) as ptrdiff_t
	    { break; }
	  newsize += copied as off_t;
	  copied = emacs_full_read (ifd, buf.as_mut_ptr () as *mut c_void,
				    std::mem::size_of_val (&buf) as ptrdiff_t);
	}
    }

  /* C 2455: `#ifndef MSDOS` (live).  */
  /* Preserve the original file permissions, and if requested, also its
     owner and group.  */
  {
    let mut preserved_permissions: mode_t = st.st_mode & 0o7777;
    let mut default_permissions: mode_t = st.st_mode & 0o777 & !realmask;
    if !NILP (preserve_uid_gid)
      {
	/* Attempt to change owner and group.  If that doesn't work
	   attempt to change just the group, as that is sometimes allowed.
	   Adjust the mode mask to eliminate setuid or setgid bits
	   or group permissions bits that are inappropriate if the
	   owner or group are wrong.  */
	if fchown (ofd, st.st_uid, st.st_gid) != 0
	  {
	    if fchown (ofd, !0u32, st.st_gid) == 0
	      { preserved_permissions &= !0o4000; }
	    else
	      {
		preserved_permissions &= !0o6000;

		/* Copy the other bits to the group bits, since the
		   group is wrong.  */
		preserved_permissions &= !0o70;
		preserved_permissions |= (preserved_permissions & 7) << 3;
		default_permissions &= !0o70;
		default_permissions |= (default_permissions & 7) << 3;
	      }
	  }
      }

    let switch_val: c_int
      = if !NILP (preserve_permissions)
	   && emacs_fd_to_int! (ifd) != -1
	{ qcopy_acl (SSDATA (encoded_file) as *const c_char,
		     emacs_fd_to_int! (ifd),
		     SSDATA (encoded_newname) as *const c_char, ofd,
		     preserved_permissions) }
      else if already_exists
	   || (new_mask & !realmask) == default_permissions
	{ 0 }
      else
	{ fchmod (ofd, default_permissions) };
    match switch_val
      {
	-2 => report_file_error (c"Copying permissions from".as_ptr (), file),
	-1 => report_file_error (c"Copying permissions to".as_ptr (), newname),
	_ => {}
      }
  }

  /* C 2509-2525: `#if HAVE_LIBSELINUX` (undefined): recorded in `docs/ports/fileio.md`.  */

  if !NILP (keep_time)
    {
      let mut ts: [timespec; 2] = std::mem::zeroed ();
      ts[0] = get_stat_atime (&raw const st);
      ts[1] = get_stat_mtime (&raw const st);
      if futimens (ofd, ts.as_ptr ()) != 0
	/* C 2532-2539: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`
	   (HAVE_ANDROID undef): recorded.  */
	{
	  xsignal2 (Qfile_date_error (),
		    build_string (c"Cannot set file date".as_ptr ()), newname);
	}
    }

  if emacs_close (ofd) < 0
    { report_file_error (c"Write error".as_ptr (), newname); }

  /* Note that ifd is not closed twice because unwind_protects are
     discarded at the end of this function.  */
  emacs_fd_close! (ifd);

  /* C 2545-2553: `#ifdef MSDOS` (undefined): recorded.  */

  /* Discard the unwind protects.  */
  (specpdl_ptr! ()) = specpdl_ref_to_ptr (count);

  return Qnil ();
});

DEFUN! ("make-directory-internal", Fmake_directory_internal,
       Smake_directory_internal, 1, 1, 0,
       doc: "Create a new directory named DIRECTORY.",
       (directory: Lisp_Object),
       {
  let mut dir: *const c_char;
  let mut encoded_dir: Lisp_Object;
  let mut directory = directory;              /* C reassigns `directory` below.  */

  CHECK_STRING (directory);
  directory = Fexpand_file_name (directory, Qnil ());

  encoded_dir = ENCODE_FILE (directory);

  dir = SSDATA (encoded_dir);

  if emacs_mkdir (dir, 0o777 & !auto_saving_dir_umask) != 0
    { report_file_error (c"Creating directory".as_ptr (), directory); }

  return Qnil ();
});

DEFUN! ("delete-directory-internal", Fdelete_directory_internal,
       Sdelete_directory_internal, 1, 1, 0,
       doc: "Delete the directory named DIRECTORY.  Does not follow symlinks.",
       (directory: Lisp_Object),
       {
  let mut dir: *const c_char;
  let mut encoded_dir: Lisp_Object;
  let mut directory = directory;              /* C reassigns `directory` below.  */

  CHECK_STRING (directory);
  directory = Fdirectory_file_name (Fexpand_file_name (directory, Qnil ()));
  encoded_dir = ENCODE_FILE (directory);
  dir = SSDATA (encoded_dir);

  if emacs_rmdir (dir) != 0
    { report_file_error (c"Removing directory".as_ptr (), directory); }

  return Qnil ();
});

DEFUN! ("delete-file-internal", Fdelete_file_internal, Sdelete_file_internal, 1, 1, 0,
       doc: "Delete file named FILENAME; internal use only.\nIf it is a symlink, remove the symlink.\nIf file has multiple names, it continues to exist with the other names.",
       (filename: Lisp_Object),
       {
  let mut encoded_file: Lisp_Object;
  let mut filename = filename;                /* C reassigns `filename` below.  */

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());
  encoded_file = ENCODE_FILE (filename);

  if emacs_unlink (SSDATA (encoded_file)) != 0
    && *errno () != ENOENT
    { report_file_error (c"Removing old name".as_ptr (), filename); }
  return Qnil ();
});

/* C 2620-2639: `#if defined HAVE_NATIVE_COMP && defined WINDOWSNT` (both undefined):
   `internal_delete_file_1` and `internal_delete_file` are recorded in
   `docs/ports/fileio.md`, not transcribed (P-08).  */

/* Return -1 if FILE is a case-insensitive file name, 0 if not,
   and 1 if the result cannot be determined.  */

unsafe fn file_name_case_insensitive_err (file: Lisp_Object) -> c_int
{
  /* Filesystems are case-sensitive on all supported systems except
     MS-Windows, MS-DOS, Cygwin, and macOS.  They are always
     case-insensitive on the first two, but they may or may not be
     case-insensitive on Cygwin and macOS so do a runtime test on
     those two systems.  If the test is not conclusive, assume
     case-insensitivity on Cygwin and case-sensitivity on macOS.

     FIXME: Mounted filesystems on Posix hosts, like Samba shares or
     NFS-mounted Windows volumes, might be case-insensitive.  Can we
     detect this?

     Use pathconf with _PC_CASE_INSENSITIVE or _PC_CASE_SENSITIVE if
     those flags are available.  As of this writing (2019-09-15),
     Cygwin is the only platform known to support the former (starting
     with Cygwin-2.6.1), and macOS is the only platform known to
     support the latter.  */

  /* C 2656-2661: `#if defined _PC_CASE_INSENSITIVE || defined _PC_CASE_SENSITIVE` is live
     (_PC_CASE_SENSITIVE is 11 on the frozen target, macros.tsv 4380); the
     `#ifdef _PC_CASE_INSENSITIVE` arm (C 2668-2671) is dead.  */
  let mut filename: *mut c_char = SSDATA (ENCODE_FILE (file));
  let mut res: c_long = pathconf (filename, _PC_CASE_SENSITIVE);
  if res >= 0
    { return - ((res == 0) as c_int); }
  if *errno () != EINVAL
    { return 1; }

  /* C 2678-2682: `#if defined CYGWIN || defined DOS_NT` (both undefined): recorded.  */
  return 0;
}

DEFUN! ("file-name-case-insensitive-p", Ffile_name_case_insensitive_p,
       Sfile_name_case_insensitive_p, 1, 1, 0,
       doc: "Return t if file FILENAME is on a case-insensitive filesystem.\nReturn nil if FILENAME does not exist or is not on a case-insensitive\nfilesystem, or if there was trouble determining whether the filesystem\nis case-insensitive.",
       (filename: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut filename = filename;                /* C reassigns `filename` below.  */

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qfile_name_case_insensitive_p ());
  if !NILP (handler)
    { return calln! (handler, Qfile_name_case_insensitive_p (), filename); }

  /* If the file doesn't exist or there is trouble checking its
     filesystem, move up the filesystem tree until we reach an
     existing, trouble-free directory or the root.  */
  loop
    {
      let mut err: c_int = file_name_case_insensitive_err (filename);
      if err <= 0
	{ return if err < 0 { Qt () } else { Qnil () }; }
      let mut parent: Lisp_Object = file_name_directory (filename);
      /* Avoid infinite loop if the root has trouble (if that's even possible).
	 Without a parent, we just don't know and return nil as well.  */
      if !STRINGP (parent) || !NILP (Fstring_equal (parent, filename))
	{ return Qnil (); }
      filename = parent;
    }
});

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

/// C's `struct allocator.die` is `void (*) (size_t)`, and lisp.rs declares the port's
/// `memory_full` as `-> !` (the reference marks it AVOID); a `-> !` fn does not coerce to a
/// `fn (…)` field (measured), so the port passes this same-signature wrapper - the lisp.rs
/// `xfree_wrapper` shape, and the only name here with no C counterpart.
unsafe extern "C" fn memory_full_wrapper (size: usize) { memory_full (size) }

unsafe extern "C" {
  /// C: lisp.h 5328: `extern int emacs_renameat_noreplace (int, const char *, int, const char *);`
  pub fn emacs_renameat_noreplace (fd1: c_int, from: *const c_char, fd2: c_int,
				   to: *const c_char) -> c_int;
  /// C: lisp.h 5330: `extern int emacs_rename (const char *, const char *);`
  pub fn emacs_rename (from: *const c_char, to: *const c_char) -> c_int;
  /// C: lisp.h 5325: `extern int emacs_symlink (const char *, const char *);`
  pub fn emacs_symlink (target: *const c_char, linkname: *const c_char) -> c_int;
  /// C: lisp.h 5326: `extern int emacs_rmdir (const char *);`
  pub fn emacs_rmdir (file: *const c_char) -> c_int;
  /// C: lisp.h 5327: `extern int emacs_mkdir (const char *, mode_t);`
  pub fn emacs_mkdir (file: *const c_char, mode: mode_t) -> c_int;
  /// C: lisp.h 5324: `extern int emacs_unlink (const char *);`
  pub fn emacs_unlink (file: *const c_char) -> c_int;
  /// C: lisp.h 5336: `extern ptrdiff_t emacs_write_quit (int, void const *, ptrdiff_t);`
  pub fn emacs_write_quit (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
  /// C: `<unistd.h>`: `ssize_t copy_file_range (int, off_t *, int, off_t *, size_t, unsigned int);`
  pub fn copy_file_range (infd: c_int, off_in: *mut off_t, outfd: c_int, off_out: *mut off_t,
			  len: usize, flags: c_uint) -> ssize_t;
  /// C: `<unistd.h>`: `long pathconf (const char *, int);`
  pub fn pathconf (path: *const c_char, name: c_int) -> c_long;
}

DEFUN! ("rename-file", Frename_file, Srename_file, 2, 3,
       "fRename file: \nGRename %s to file: \np",
       doc: "Rename FILE as NEWNAME.  Both args must be strings.\nIf file has names other than FILE, it continues to have those names.\nIf NEWNAME is a directory name, rename FILE to a like-named file under\nNEWNAME.  For NEWNAME to be recognized as a directory name, it should\nend in a slash.\n\nSignal a `file-already-exists' error if a file NEWNAME already exists\nunless optional third argument OK-IF-ALREADY-EXISTS is non-nil.\nAn integer third arg means request confirmation if NEWNAME already exists.\nThis is what happens in interactive use with M-x.",
       (file: Lisp_Object, newname: Lisp_Object, ok_if_already_exists: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut encoded_file: Lisp_Object;
  let mut encoded_newname: Lisp_Object;
  let mut file = file;                        /* C reassigns `file` below.  */
  let mut newname = newname;                  /* C reassigns `newname` below.  */
  let mut ok_if_already_exists = ok_if_already_exists;  /* C reassigns it below.  */

  file = Fexpand_file_name (file, Qnil ());

  /* If the filesystem is case-insensitive and the file names are
     identical but for case, treat it as a change-case request, and do
     not worry whether NEWNAME exists or whether it is a directory, as
     it is already another name for FILE.  */
  let mut case_only_rename: bool = false;
  /* C 2755-2763: `#if defined CYGWIN || defined DOS_NT` (both undefined): recorded.  */

  if !case_only_rename
    { newname = expand_cp_target (Fdirectory_file_name (file), newname); }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qrename_file ());
  if NILP (handler)
    { handler = Ffind_file_name_handler (newname, Qrename_file ()); }
  if !NILP (handler)
    { return calln! (handler, Qrename_file (),
		     file, newname, ok_if_already_exists); }

  encoded_file = ENCODE_FILE (file);
  encoded_newname = ENCODE_FILE (newname);

  let mut plain_rename: bool = case_only_rename
    || (!NILP (ok_if_already_exists) && !FIXNUMP (ok_if_already_exists));
  let mut rename_errno: c_int = 0;   /* C: `int rename_errno UNINIT;` (PORTING.md 1.5)  */
  if !plain_rename
    {
      if emacs_renameat_noreplace (AT_FDCWD,
				   SSDATA (encoded_file),
				   AT_FDCWD,
				   SSDATA (encoded_newname))
	  == 0
	{ return Qnil (); }

      rename_errno = *errno ();
      match rename_errno
	{
	  EEXIST | EINVAL | ENOSYS | ENOTSUP =>
	    {
	      barf_or_query_if_file_exists (newname, rename_errno == EEXIST,
					    c"rename to it".as_ptr (),
					    FIXNUMP (ok_if_already_exists),
					    false);
	      plain_rename = true;
	    }
	  _ => {}
	}
    }

  if plain_rename
    {
      if emacs_rename (SSDATA (encoded_file),
		       SSDATA (encoded_newname)) == 0
	{ return Qnil (); }
      rename_errno = *errno ();
      /* Don't prompt again.  */
      ok_if_already_exists = Qt ();
    }
  else if !NILP (ok_if_already_exists)
    { ok_if_already_exists = Qt (); }

  if rename_errno != EXDEV
    { report_file_errno (c"Renaming".as_ptr (), list2 (file, newname), rename_errno); }

  let mut file_st: stat = std::mem::zeroed ();
  let mut dirp: bool = !NILP (Fdirectory_name_p (file));
  if !dirp
    {
      if emacs_fstatat (AT_FDCWD, SSDATA (encoded_file),
			&raw mut file_st as *mut c_void, AT_SYMLINK_NOFOLLOW)
	  != 0
	{ report_file_error (c"Renaming".as_ptr (), list2 (file, newname)); }
      dirp = S_ISDIR! (file_st.st_mode);
    }
  if dirp
    { calln! (Qcopy_directory (), file, newname, Qt (), Qnil ()); }
  else if S_ISREG! (file_st.st_mode)
    { Fcopy_file (file, newname, ok_if_already_exists, Qt (), Qt (), Qt ()); }
  else if S_ISLNK! (file_st.st_mode)
    {
      let mut target: Lisp_Object = emacs_readlinkat (AT_FDCWD,
						      SSDATA (encoded_file));
      if !NILP (target)
	{ Fmake_symbolic_link (target, newname, ok_if_already_exists); }
      else
	{ report_file_error (c"Renaming".as_ptr (), list2 (file, newname)); }
    }
  else
    { report_file_errno (c"Renaming".as_ptr (), list2 (file, newname), rename_errno); }

  let count: specpdl_ref = SPECPDL_INDEX ();
  specbind (Qdelete_by_moving_to_trash (), Qnil ());
  if dirp
    { calln! (Qdelete_directory (), file, Qt ()); }
  else
    { calln! (Qdelete_file (), file, Qnil ()); }
  return unbind_to (count, Qnil ());
});

DEFUN! ("add-name-to-file", Fadd_name_to_file, Sadd_name_to_file, 2, 3,
       "fAdd name to file: \nGName to add to %s: \np",
       doc: "Give FILE additional name NEWNAME.  Both args must be strings.\nIf NEWNAME is a directory name, give FILE a like-named new name under\nNEWNAME.\n\nSignal a `file-already-exists' error if a file NEWNAME already exists\nunless optional third argument OK-IF-ALREADY-EXISTS is non-nil.\nAn integer third arg means request confirmation if NEWNAME already exists.\nThis is what happens in interactive use with M-x.",
       (file: Lisp_Object, newname: Lisp_Object, ok_if_already_exists: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut encoded_file: Lisp_Object;
  let mut encoded_newname: Lisp_Object;
  let mut file = file;                        /* C reassigns `file` below.  */
  let mut newname = newname;                  /* C reassigns `newname` below.  */

  file = Fexpand_file_name (file, Qnil ());
  newname = expand_cp_target (file, newname);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qadd_name_to_file ());
  if !NILP (handler)
    { return calln! (handler, Qadd_name_to_file (), file,
		     newname, ok_if_already_exists); }

  /* If the new name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (newname, Qadd_name_to_file ());
  if !NILP (handler)
    { return calln! (handler, Qadd_name_to_file (), file,
		     newname, ok_if_already_exists); }

  encoded_file = ENCODE_FILE (file);
  encoded_newname = ENCODE_FILE (newname);
  check_vfs_filename (encoded_file, c"Trying to create hard link to file within special directory".as_ptr ());
  check_vfs_filename (encoded_newname, c"Trying to create hard link within special directory".as_ptr ());

  if link (SSDATA (encoded_file), SSDATA (encoded_newname)) == 0
    { return Qnil (); }

  if *errno () == EEXIST
    {
      if NILP (ok_if_already_exists)
	  || FIXNUMP (ok_if_already_exists)
	{ barf_or_query_if_file_exists (newname, true, c"make it a new name".as_ptr (),
					FIXNUMP (ok_if_already_exists), false); }
      emacs_unlink (SSDATA (newname));
      if link (SSDATA (encoded_file), SSDATA (encoded_newname)) == 0
	{ return Qnil (); }
    }

  report_file_error (c"Adding new name".as_ptr (), list2 (file, newname));
});

DEFUN! ("make-symbolic-link", Fmake_symbolic_link, Smake_symbolic_link, 2, 3,
       "FMake symbolic link to file: \nGMake symbolic link to file %s: \np",
       doc: "Make a symbolic link to TARGET, named LINKNAME.\nIf LINKNAME is a directory name, make a like-named symbolic link under\nLINKNAME.\n\nSignal a `file-already-exists' error if a file LINKNAME already exists\nunless optional third argument OK-IF-ALREADY-EXISTS is non-nil.\nAn integer third arg means request confirmation if LINKNAME already\nexists, and expand leading \"~\" or strip leading \"/:\" in TARGET.\nThis happens for interactive use with M-x.",
       (target: Lisp_Object, linkname: Lisp_Object, ok_if_already_exists: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut encoded_target: Lisp_Object;
  let mut encoded_linkname: Lisp_Object;
  let mut target = target;                    /* C reassigns `target` below.  */
  let mut linkname = linkname;                /* C reassigns `linkname` below.  */

  CHECK_STRING (target);
  if FIXNUMP (ok_if_already_exists)
    {
      if SREF (target, 0) == b'~' as u8
	{ target = Fexpand_file_name (target, Qnil ()); }
      else if SREF (target, 0) == b'/' as u8 && SREF (target, 1) == b':' as u8
	{ target = Fsubstring_no_properties (target, make_fixnum (2), Qnil ()); }
    }
  linkname = expand_cp_target (target, linkname);

  /* If the new link name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (linkname, Qmake_symbolic_link ());
  if !NILP (handler)
    { return calln! (handler, Qmake_symbolic_link (), target,
		     linkname, ok_if_already_exists); }

  encoded_target = ENCODE_FILE (target);
  encoded_linkname = ENCODE_FILE (linkname);

  if emacs_symlink (SSDATA (encoded_target),
		    SSDATA (encoded_linkname)) == 0
    { return Qnil (); }

  if *errno () == ENOSYS
    {
      xsignal1 (Qfile_error (),
		build_string (c"Symbolic links are not supported".as_ptr ()));
    }

  if *errno () == EEXIST
    {
      if NILP (ok_if_already_exists)
	  || FIXNUMP (ok_if_already_exists)
	{ barf_or_query_if_file_exists (linkname, true, c"make it a link".as_ptr (),
					FIXNUMP (ok_if_already_exists), false); }
      emacs_unlink (SSDATA (encoded_linkname));
      if emacs_symlink (SSDATA (encoded_target),
			SSDATA (encoded_linkname)) == 0
	{ return Qnil (); }
    }

  report_file_error (c"Making symbolic link".as_ptr (), list2 (target, linkname));
});

DEFUN! ("file-name-absolute-p", Ffile_name_absolute_p, Sfile_name_absolute_p,
       1, 1, 0,
       doc: "Return t if FILENAME is an absolute file name.\nOn Unix, absolute file names start with `/'.  In Emacs, an absolute\nfile name can also start with an initial `~' or `~USER' component,\nwhere USER is a valid login name.",
       (filename: Lisp_Object),
       {
  CHECK_STRING (filename);
  return if file_name_absolute_p (SSDATA (filename)) { Qt () } else { Qnil () };
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_name_absolute_p (filename: *const c_char) -> bool
{
  return IS_ABSOLUTE_FILE_NAME! (filename)
    || (*filename.offset (0) == b'~' as c_char
	&& (*filename.offset (1) == 0 || IS_DIRECTORY_SEP (*filename.offset (1) as c_int)
	    || !user_homedir (filename.offset (1)).is_null ()));
}

/* Return t if FILE exists and is accessible via OPERATION and AMODE,
   nil (setting errno) if not.  */

unsafe fn check_file_access (file: Lisp_Object, operation: Lisp_Object,
				    amode: c_int) -> Lisp_Object
{
  let mut file = file;                        /* C reassigns `file` below.  */
  file = Fexpand_file_name (file, Qnil ());
  let mut handler: Lisp_Object = Ffind_file_name_handler (file, operation);
  if !NILP (handler)
    {
      let mut ok: Lisp_Object = calln! (handler, operation, file);
      /* This errno value is bogus.  Any caller that depends on errno
	 should be rethought anyway, to avoid a race between testing a
	 handled file's accessibility and using the file.  */
      *errno () = 0;
      return ok;
    }

  let mut encoded_file: *mut c_char = SSDATA (ENCODE_FILE (file));
  return if file_access_p (encoded_file, amode) { Qt () } else { Qnil () };
}

DEFUN! ("file-exists-p", Ffile_exists_p, Sfile_exists_p, 1, 1, 0,
       doc: "Return t if file FILENAME exists (whether or not you can read it).\nReturn nil if FILENAME does not exist, or if there was trouble\ndetermining whether the file exists.\nSee also `file-readable-p' and `file-attributes'.\nThis returns nil for a symlink to a nonexistent file.\nUse `file-symlink-p' to test for such links.",
       (filename: Lisp_Object),
       {
  return check_file_access (filename, Qfile_exists_p (), F_OK);
});

DEFUN! ("file-executable-p", Ffile_executable_p, Sfile_executable_p, 1, 1, 0,
       doc: "Return t if FILENAME can be executed by you.\nFor a directory, this means you can access files in that directory.\n(It is generally better to use `file-accessible-directory-p' for that\npurpose, though.)",
       (filename: Lisp_Object),
       {
  return check_file_access (filename, Qfile_executable_p (), X_OK);
});

DEFUN! ("file-readable-p", Ffile_readable_p, Sfile_readable_p, 1, 1, 0,
       doc: "Return t if file FILENAME exists and you can read it.\nSee also `file-exists-p' and `file-attributes'.",
       (filename: Lisp_Object),
       {
  return check_file_access (filename, Qfile_readable_p (), R_OK);
});

DEFUN! ("file-writable-p", Ffile_writable_p, Sfile_writable_p, 1, 1, 0,
       doc: "Return t if file FILENAME can be written or created by you.",
       (filename: Lisp_Object),
       {
  let mut absname: Lisp_Object;
  let mut dir: Lisp_Object;
  let mut encoded: Lisp_Object;
  let mut handler: Lisp_Object;

  absname = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (absname, Qfile_writable_p ());
  if !NILP (handler)
    { return calln! (handler, Qfile_writable_p (), absname); }

  encoded = ENCODE_FILE (absname);
  if file_access_p (SSDATA (encoded), W_OK)
    { return Qt (); }
  if *errno () != ENOENT
    { return Qnil (); }

  dir = file_name_directory (absname);
  eassert! (!NILP (dir));
  /* C 3063-3065: `#ifdef MSDOS` (undefined): recorded.  */

  encoded = ENCODE_FILE (dir);
  /* C 3067-3074: `#ifdef WINDOWSNT` (undefined): recorded.  */
  return if file_access_p (SSDATA (encoded), W_OK | X_OK) { Qt () } else { Qnil () };
});

DEFUN! ("access-file", Faccess_file, Saccess_file, 2, 2, 0,
       doc: "Access file FILENAME, and get an error if that does not work.\nThe second argument STRING is prepended to the error message.\nIf there is no error, returns nil.",
       (filename: Lisp_Object, string: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut encoded_filename: Lisp_Object;
  let mut absname: Lisp_Object;

  CHECK_STRING (filename);
  absname = Fexpand_file_name (filename, Qnil ());

  CHECK_STRING (string);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (absname, Qaccess_file ());
  if !NILP (handler)
    { return calln! (handler, Qaccess_file (), absname, string); }

  encoded_filename = ENCODE_FILE (absname);

  if sys_faccessat (AT_FDCWD, SSDATA (encoded_filename), R_OK,
		    AT_EACCESS) != 0
    { report_file_error (SSDATA (string), filename); }

  return Qnil ();
});

/* Relative to directory FD, return the symbolic link value of FILENAME.
   On failure, return nil (setting errno).  */

unsafe fn emacs_readlinkat (fd: c_int, filename: *const c_char) -> Lisp_Object
{
  static emacs_norealloc_allocator: allocator = allocator {
    allocate: Some (xmalloc),
    reallocate: None,
    free: Some (xfree),
    die: Some (memory_full_wrapper),
  };

  let mut val: Lisp_Object;
  let mut readlink_buf: [c_char; 1024] = std::mem::zeroed ();
  let mut buf: *mut c_char;

  buf = careadlinkat (fd, filename, readlink_buf.as_mut_ptr (),
		      std::mem::size_of_val (&readlink_buf),
		      &raw const emacs_norealloc_allocator,
		      /* C 3130-3134: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`
			 (HAVE_ANDROID undef): the live argument is `readlinkat`.  */
		      Some (readlinkat));
  if buf.is_null ()
    { return Qnil (); }

  val = build_unibyte_string (buf);
  if buf != readlink_buf.as_mut_ptr ()
    { xfree (buf as *mut c_void); }
  val = DECODE_FILE (val);
  return val;
}

/* Relative to directory FD, return the symbolic link value of FILE.
   If FILE is not a symbolic link, return nil (setting errno).
   Signal an error if the result cannot be determined.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_emacs_readlinkat (fd: c_int, file: Lisp_Object,
						 encoded_file: *const c_char)
  -> Lisp_Object
{
  let mut val: Lisp_Object = emacs_readlinkat (fd, encoded_file);
  if NILP (val)
    {
      if *errno () == EINVAL
	{ return val; }
      /* C 3164-3168: `#ifdef CYGWIN` (undefined): recorded.  */
      return file_metadata_errno (c"Reading symbolic link".as_ptr (), file, *errno ());
    }
  return val;
}

DEFUN! ("file-symlink-p", Ffile_symlink_p, Sfile_symlink_p, 1, 1, 0,
       doc: "Return non-nil if file FILENAME is the name of a symbolic link.\nThe value is the link target, as a string.\nReturn nil if FILENAME does not exist or is not a symbolic link,\nof there was trouble determining whether the file is a symbolic link.\n\nThis function does not check whether the link target exists.",
       (filename: Lisp_Object),
       {
  let mut handler: Lisp_Object;
  let mut filename = filename;                /* C reassigns `filename` below.  */

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qfile_symlink_p ());
  if !NILP (handler)
    { return calln! (handler, Qfile_symlink_p (), filename); }

  return emacs_readlinkat (AT_FDCWD, SSDATA (ENCODE_FILE (filename)));
});

DEFUN! ("file-directory-p", Ffile_directory_p, Sfile_directory_p, 1, 1, 0,
       doc: "Return t if FILENAME names an existing directory.\nReturn nil if FILENAME does not name a directory, or if there\nwas trouble determining whether FILENAME is a directory.\n\nAs a special case, this function will also return t if FILENAME is the\nempty string (\"\").  This quirk is due to Emacs interpreting the\nempty string (in some cases) as the current directory.\n\nSymbolic links to directories count as directories.\nSee `file-symlink-p' to distinguish symlinks.",
       (filename: Lisp_Object),
       {
  let mut absname: Lisp_Object = expand_and_dir_to_file (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname, Qfile_directory_p ());
  if !NILP (handler)
    { return calln! (handler, Qfile_directory_p (), absname); }

  return if file_directory_p (ENCODE_FILE (absname)) { Qt () } else { Qnil () };
});

/* Return true if FILE is a directory or a symlink to a directory.
   Otherwise return false and set errno.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_directory_p (file: Lisp_Object) -> bool
{
  /* C 3194-3203: `#ifdef DOS_NT` (undefined): recorded in `docs/ports/fileio.md`.  */
  /* C 3204-3219: `# if defined O_PATH && !(defined HAVE_ANDROID && !defined
     ANDROID_STUBIFY)`: O_PATH is not defined on the frozen target: recorded.  */
  /* Use file_accessible_directory_p, as it avoids fstatat EOVERFLOW
     problems and could be cheaper.  However, if it fails because FILE
     is inaccessible, fall back on fstatat; if the latter fails with
     EOVERFLOW then FILE must have been a directory unless a race
     condition occurred (a problem hard to work around portably).  */
  if file_accessible_directory_p (file)
    { return true; }
  if *errno () != EACCES
    { return false; }
  let mut st: stat = std::mem::zeroed ();
  if emacs_fstatat (AT_FDCWD, SSDATA (file), &raw mut st as *mut c_void, 0) != 0
    { return *errno () == EOVERFLOW; }
  if S_ISDIR! (st.st_mode)
    { return true; }
  *errno () = ENOTDIR;
  return false;
}

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

/// `<intprops.h>` has no mirror (PORTING.md P-26): C's `TYPE_MAXIMUM (t)` is the type's own
/// largest value (the editfns.rs/syntax.rs shape).
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

unsafe extern "C" {
  /// C: lisp.h 4359: `extern void adjust_markers_for_delete (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t);`
  pub fn adjust_markers_for_delete (from: ptrdiff_t, from_byte: ptrdiff_t,
				    to: ptrdiff_t, to_byte: ptrdiff_t);
  /// C: lisp.h 4940: `extern void adjust_overlays_for_delete (ptrdiff_t, ptrdiff_t);`
  pub fn adjust_overlays_for_delete (from: ptrdiff_t, to: ptrdiff_t);
  /// C: `<unistd.h>`: `extern void sync (void);`
  pub fn sync ();
  /// C: lisp.h 5331: `extern int emacs_fchmodat (int, const char *, mode_t, int);`
  pub fn emacs_fchmodat (dirfd: c_int, file: *const c_char, mode: mode_t, flag: c_int)
    -> c_int;
}

DEFUN! ("file-accessible-directory-p", Ffile_accessible_directory_p,
       Sfile_accessible_directory_p, 1, 1, 0,
       doc: "Return t if FILENAME names a directory you can open.\nThis means that FILENAME must specify the name of a directory, and the\ndirectory must allow you to open files in it.  If this isn't the case,\nreturn nil.\n\nFILENAME can either be a directory name (eg. \"/tmp/foo/\") or the\nfile name of a file which is a directory (eg. \"/tmp/foo\", without\nthe final slash).\n\nIn order to use a directory as a buffer's current directory, this\npredicate must return true.",
       (filename: Lisp_Object),
       {
  let mut absname: Lisp_Object;
  let mut handler: Lisp_Object;

  CHECK_STRING (filename);
  absname = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (absname, Qfile_accessible_directory_p ());
  if !NILP (handler)
    {
      let mut r: Lisp_Object = calln! (handler, Qfile_accessible_directory_p (), absname);

      /* Set errno in case the handler failed.  EACCES might be a lie
	 (e.g., the directory might not exist, or be a regular file),
	 but at least it does TRT in the "usual" case of an existing
	 directory that is not accessible by the current user, and
	 avoids reporting "Success" for a failed operation.  Perhaps
	 someday we can fix this in a better way, by improving
	 file-accessible-directory-p's API; see Bug#25419.  */
      if !EQ (r, Qt ())
	{ *errno () = EACCES; }

      return r;
    }

  let mut encoded_absname: Lisp_Object = ENCODE_FILE (absname);
  return if file_accessible_directory_p (encoded_absname) { Qt () } else { Qnil () };
});

/* If FILE is a searchable directory or a symlink to a
   searchable directory, return true.  Otherwise return
   false and set errno to an error number.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_accessible_directory_p (file: Lisp_Object) -> bool
{
  /* C 3304-3317: `#ifdef DOS_NT` (undefined): recorded in `docs/ports/fileio.md`.  */
  /* On POSIXish platforms, use just one system call; this avoids a
     race and is typically faster.  */
  let mut data: *const c_char = SSDATA (file);
  let mut len: ptrdiff_t = SBYTES (file);
  let mut dir: *const c_char;
  let mut ok: bool;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* Normally a file "FOO" is an accessible directory if "FOO/." exists.
     There are three exceptions: "", "/", and "//".  Leave "" alone,
     as it's invalid.  Append only "." to the other two exceptions as
     "/" and "//" are distinct on some platforms, whereas "/", "///",
     "////", etc. are all equivalent.

     Android has a special directory named "/assets".  There is no "."
     directory there, but appending a "/" is sufficient to check
     whether or not it is a directory.  */
  if len == 0
    { dir = data; }
  else
    {
      /* Just check for trailing '/' when deciding whether append '/'
	 before appending '.'.  That's simpler than testing the two
	 special cases "/" and "//", and it's a safe optimization
	 here.  After appending '.', append another '/' to work around
	 a macOS bug (Bug#30350).  */

      static appended: [c_char; 5] = [b'/' as c_char, b'.' as c_char, b'/' as c_char,
				      0, 0];
      let mut buf: *mut c_char = SAFE_ALLOCA! (sa_avail, len + 5) as *mut c_char;
      memcpy (buf as *mut c_void, data as *const c_void, len as usize);
      strcpy (buf.offset (len),
	      appended.as_ptr ().offset ((*data.offset (len - 1) == b'/' as c_char) as isize));
      dir = buf;
    }

  ok = file_access_p (dir, F_OK);
  SAFE_FREE! (sa_count, sa_mark);
  return ok;
}

DEFUN! ("file-regular-p", Ffile_regular_p, Sfile_regular_p, 1, 1, 0,
       doc: "Return t if FILENAME names a regular file.\nThis is the sort of file that holds an ordinary stream of data bytes.\nReturn nil if FILENAME does not exist or is not a regular file,\nor there was trouble determining whether FILENAME is a regular file.\nSymbolic links to regular files count as regular files.\nSee `file-symlink-p' to distinguish symlinks.",
       (filename: Lisp_Object),
       {
  let mut st: stat = std::mem::zeroed ();
  let mut absname: Lisp_Object = expand_and_dir_to_file (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname, Qfile_regular_p ());
  if !NILP (handler)
    { return calln! (handler, Qfile_regular_p (), absname); }

  /* C 3364-3369: `#ifdef WINDOWSNT` (undefined): recorded.  */
  let mut stat_result: c_int = emacs_fstatat (AT_FDCWD, SSDATA (absname),
					     &raw mut st as *mut c_void, 0);

  /* C 3371-3374: `#ifdef WINDOWSNT` (undefined): recorded.  */
  return if stat_result == 0 && S_ISREG! (st.st_mode) { Qt () } else { Qnil () };
});

DEFUN! ("file-selinux-context", Ffile_selinux_context,
       Sfile_selinux_context, 1, 1, 0,
       doc: "Return SELinux context of file named FILENAME.\nThe return value is a list (USER ROLE TYPE RANGE), where the list\nelements are strings naming the user, role, type, and range of the\nfile's SELinux security context.\n\nReturn (nil nil nil nil) if the file is nonexistent,\nor if SELinux is disabled, or if Emacs lacks SELinux support.",
       (filename: Lisp_Object),
       {
  let mut user: Lisp_Object = Qnil ();
  let mut role: Lisp_Object = Qnil ();
  let mut r#type: Lisp_Object = Qnil ();
  let mut range: Lisp_Object = Qnil ();
  let mut absname: Lisp_Object = expand_and_dir_to_file (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname,
							 Qfile_selinux_context ());
  if !NILP (handler)
    { return calln! (handler, Qfile_selinux_context (), absname); }

  /* C 3385-3419: `#ifdef HAVE_LIBSELINUX` (undefined): recorded.  */

  return list4 (user, role, r#type, range);
});

DEFUN! ("set-file-selinux-context", Fset_file_selinux_context,
       Sset_file_selinux_context, 2, 2, 0,
       doc: "Set SELinux context of file named FILENAME to CONTEXT.\nCONTEXT should be a list (USER ROLE TYPE RANGE), where the list\nelements are strings naming the components of a SELinux context.\n\nValue is t if setting of SELinux context was successful, nil otherwise.\n\nThis function does nothing and returns nil if SELinux is disabled,\nor if Emacs was not compiled with SELinux support.",
       (filename: Lisp_Object, context: Lisp_Object),
       {
  let mut absname: Lisp_Object;
  let mut handler: Lisp_Object;
  /* C 3437-3449: `#if HAVE_LIBSELINUX` (undefined): recorded.  */

  absname = Fexpand_file_name (filename, BVAR! ((current_buffer! ()), directory_));

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (absname, Qset_file_selinux_context ());
  if !NILP (handler)
    { return calln! (handler, Qset_file_selinux_context (), absname, context); }

  /* C 3482-3531: `#if HAVE_LIBSELINUX` (undefined): recorded.  */

  return Qnil ();
});

DEFUN! ("file-acl", Ffile_acl, Sfile_acl, 1, 1, 0,
       doc: "Return ACL entries of file named FILENAME.\nThe entries are returned in a format suitable for use in `set-file-acl'\nbut is otherwise undocumented and subject to change.\nReturn nil if file does not exist.",
       (filename: Lisp_Object),
       {
  let mut acl_string: Lisp_Object = Qnil ();

  /* C 3542-3576: `#if USE_ACL` (0 on the frozen target): recorded.  */

  return acl_string;
});

DEFUN! ("set-file-acl", Fset_file_acl, Sset_file_acl,
       2, 2, 0,
       doc: "Set ACL of file named FILENAME to ACL-STRING.\nACL-STRING should contain the textual representation of the ACL\nentries in a format suitable for the platform.\n\nValue is t if setting of ACL was successful, nil otherwise.\n\nSetting ACL for local files requires Emacs to be built with ACL\nsupport.",
       (filename: Lisp_Object, acl_string: Lisp_Object),
       {
  /* C 3588-3628: `#if USE_ACL` (0 on the frozen target): recorded.  */

  return Qnil ();
});

unsafe fn symlink_nofollow_flag (flag: Lisp_Object) -> c_int
{
  /* For now, treat all non-nil FLAGs like 'nofollow'.  */
  return if !NILP (flag) { AT_SYMLINK_NOFOLLOW } else { 0 };
}

DEFUN! ("file-modes", Ffile_modes, Sfile_modes, 1, 2, 0,
       doc: "Return mode bits of file named FILENAME, as an integer.\nReturn nil if FILENAME does not exist.  If optional FLAG is `nofollow',\ndo not follow FILENAME if it is a symbolic link.",
       (filename: Lisp_Object, flag: Lisp_Object),
       {
  let mut st: stat = std::mem::zeroed ();
  let mut nofollow: c_int = symlink_nofollow_flag (flag);
  let mut absname: Lisp_Object = expand_and_dir_to_file (filename);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname, Qfile_modes ());
  if !NILP (handler)
    { return calln! (handler, Qfile_modes (), absname, flag); }

  let mut fname: *mut c_char = SSDATA (ENCODE_FILE (absname));
  if emacs_fstatat (AT_FDCWD, fname, &raw mut st as *mut c_void, nofollow) != 0
    { return file_attribute_errno (absname, *errno ()); }
  return make_fixnum ((st.st_mode & 0o7777) as isize);
});

DEFUN! ("set-file-modes", Fset_file_modes, Sset_file_modes, 2, 3,
       "(let ((file (read-file-name \"File: \")))\t\t\t\t  (list file (read-file-modes nil file)))",
       doc: "Set mode bits of file named FILENAME to MODE (an integer).\nOnly the 12 low bits of MODE are used.  If optional FLAG is `nofollow',\ndo not follow FILENAME if it is a symbolic link.\n\nInteractively, prompt for FILENAME, and read MODE with\n`read-file-modes', which accepts symbolic notation, like the `chmod'\ncommand from GNU Coreutils.",
       (filename: Lisp_Object, mode: Lisp_Object, flag: Lisp_Object),
       {
  let mut encoded: Lisp_Object;

  CHECK_FIXNUM (mode);
  let mut nofollow: c_int = symlink_nofollow_flag (flag);
  let mut absname: Lisp_Object = Fexpand_file_name (filename,
						   BVAR! ((current_buffer! ()), directory_));

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname, Qset_file_modes ());
  if !NILP (handler)
    { return calln! (handler, Qset_file_modes (), absname, mode, flag); }

  encoded = ENCODE_FILE (absname);
  let mut fname: *mut c_char = SSDATA (encoded);
  let mut imode: mode_t = (XFIXNUM (mode) & 0o7777) as mode_t;
  if emacs_fchmodat (AT_FDCWD, fname, imode, nofollow) != 0
    { report_file_error (c"Doing chmod".as_ptr (), absname); }

  return Qnil ();
});

DEFUN! ("set-default-file-modes", Fset_default_file_modes, Sset_default_file_modes, 1, 1, 0,
       doc: "Set the file permission bits for newly created files.\nThe argument MODE should be an integer; only the low 9 bits are used.\nOn Posix hosts, this setting is inherited by subprocesses.\n\nThis function works by setting the Emacs's file mode creation mask.\nEach bit that is set in the mask means that the corresponding bit\nin the permissions of newly created files will be disabled.\n\nNote that when `write-region' creates a file, it resets the\nexecute bit, even if the mask set by this function allows that bit\nby having the corresponding bit in the mask reset.\n\nSee also `with-file-modes'.",
       (mode: Lisp_Object),
       {
  let mut oldrealmask: mode_t;
  let mut oldumask: mode_t;
  let mut newumask: mode_t;
  CHECK_FIXNUM (mode);
  oldrealmask = realmask;
  newumask = (! XFIXNUM (mode)) as mode_t & 0o777;

  block_input ();
  realmask = newumask;
  oldumask = umask (newumask);
  unblock_input ();

  eassert! (oldumask == oldrealmask);
  return Qnil ();
});

DEFUN! ("default-file-modes", Fdefault_file_modes, Sdefault_file_modes, 0, 0, 0,
       doc: "Return the default file protection for created files.\nThe value is an integer.",
       (),
       {
  let mut value: Lisp_Object;
  XSETINT! (value, ((! realmask) & 0o777) as isize);
  return value;
});

DEFUN! ("set-file-times", Fset_file_times, Sset_file_times, 1, 3, 0,
       doc: "Set times of file FILENAME to TIMESTAMP.\nIf optional FLAG is `nofollow', do not follow FILENAME if it is a\nsymbolic link.  Set both access and modification times.  Return t on\nsuccess, else nil.  Use the current time if TIMESTAMP is nil.\nTIMESTAMP is in the format of `current-time'.",
       (filename: Lisp_Object, timestamp: Lisp_Object, flag: Lisp_Object),
       {
  let mut nofollow: c_int = symlink_nofollow_flag (flag);

  let mut ts: [timespec; 2] = std::mem::zeroed ();
  if !NILP (timestamp)
    { ts[0] = lisp_time_argument (timestamp); ts[1] = ts[0]; }
  else
    { ts[0].tv_nsec = UTIME_NOW; ts[1].tv_nsec = UTIME_NOW; }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut absname: Lisp_Object = Fexpand_file_name (filename, BVAR! ((current_buffer! ()), directory_));
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname, Qset_file_times ());
  if !NILP (handler)
    { return calln! (handler, Qset_file_times (), absname, timestamp, flag); }

  let mut encoded_absname: Lisp_Object = ENCODE_FILE (absname);
  check_vfs_filename (encoded_absname, c"Trying to set access times of file within special directory".as_ptr ());

  if utimensat (AT_FDCWD, SSDATA (encoded_absname), ts.as_ptr (), nofollow) != 0
    {
      /* C 3775-3779: `#ifdef MSDOS` (undefined): recorded.  */
      report_file_error (c"Setting file times".as_ptr (), absname);
    }

  return Qt ();
});

/* C 3786: `#ifdef HAVE_SYNC` (HAVE_SYNC is 1 on the frozen target, so the text is live).  */
DEFUN! ("unix-sync", Funix_sync, Sunix_sync, 0, 0, "",
       doc: "Tell Unix to finish all pending disk updates.",
       (),
       {
  sync ();
  return Qnil ();
});

DEFUN! ("file-newer-than-file-p", Ffile_newer_than_file_p, Sfile_newer_than_file_p, 2, 2, 0,
       doc: "Return t if file FILE1 is newer than file FILE2.\nIf FILE1 does not exist, the return value is nil;\nif FILE2 does not exist, the return value is t.\nFor existing files, this compares their last-modified times.",
       (file1: Lisp_Object, file2: Lisp_Object),
       {
  let mut st1: stat = std::mem::zeroed ();
  let mut st2: stat = std::mem::zeroed ();
  let mut encoded: Lisp_Object;

  CHECK_STRING (file1);
  CHECK_STRING (file2);

  let mut absname1: Lisp_Object = expand_and_dir_to_file (file1);
  let mut absname2: Lisp_Object = expand_and_dir_to_file (file2);

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (absname1,
							  Qfile_newer_than_file_p ());
  if NILP (handler)
    { handler = Ffind_file_name_handler (absname2, Qfile_newer_than_file_p ()); }
  if !NILP (handler)
    { return calln! (handler, Qfile_newer_than_file_p (), absname1, absname2); }

  encoded = ENCODE_FILE (absname1);

  let mut err1: c_int;
  if emacs_fstatat (AT_FDCWD, SSDATA (encoded), &raw mut st1 as *mut c_void, 0) == 0
    { err1 = 0; }
  else
    {
      err1 = *errno ();
      if err1 != EOVERFLOW
	{ return file_attribute_errno (absname1, err1); }
    }
  if emacs_fstatat (AT_FDCWD, SSDATA (ENCODE_FILE (absname2)),
		    &raw mut st2 as *mut c_void, 0) != 0
    {
      file_attribute_errno (absname2, *errno ());
      return Qt ();
    }
  if err1 != 0
    { file_attribute_errno (absname1, err1); }

  return if timespec_cmp (get_stat_mtime (&raw const st2), get_stat_mtime (&raw const st1)) < 0
    { Qt () } else { Qnil () };
});

/* This function is called after Lisp functions to decide a coding
   system are called, or when they cause an error.  Before they are
   called, the current buffer is set unibyte and it contains only a
   newly inserted text (thus the buffer was empty before the
   insertion).

   The functions may set markers, overlays, text properties, or even
   alter the buffer contents, change the current buffer.

   Here, we reset all those changes by:
	o set back the current buffer.
	o move all markers and overlays to BEG.
	o remove all text properties.
	o set back the buffer multibyteness.  */

unsafe extern "C" fn decide_coding_unwind (unwind_data: Lisp_Object)
{
  let mut multibyte: Lisp_Object = XCAR (unwind_data);
  let mut tmp: Lisp_Object = XCDR (unwind_data);
  let mut undo_list: Lisp_Object = XCAR (tmp);
  let mut buffer: Lisp_Object = XCDR (tmp);

  set_buffer_internal (XBUFFER (buffer));

  /* We're about to "delete" the text by moving it back into the gap.
     So move markers that set-auto-coding might have created to BEG,
     just in case.  */
  adjust_markers_for_delete (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t, Z! (), Z_BYTE! ());
  adjust_overlays_for_delete (BEG as ptrdiff_t, Z! () - BEG as ptrdiff_t);
  set_buffer_intervals ((current_buffer! ()), std::ptr::null_mut ());
  TEMP_SET_PT_BOTH (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);

  /* In case of a non-local exit from set_auto_coding_function, in order not
     to end up with potentially invalid byte sequences in a multibyte buffer,
     we have the following options:
     1- decode the bytes in some arbitrary coding-system.
     2- erase the buffer.
     3- leave the buffer unibyte (which is actually the same as option (1)
        where the coding-system is `raw-text-unix`).
     Here we choose 2.  */

  /* Move the bytes back to (the beginning of) the gap.
     In general this may have to move all the bytes, but here
     this can't move more bytes than were moved during the execution
     of Vset_auto_coding_function, which is normally 0 (because it
     normally doesn't modify the buffer).  */
  move_gap_both (Z! (), Z_BYTE! ());
  let mut inserted: ptrdiff_t = Z_BYTE! () - BEG_BYTE as ptrdiff_t;
  GAP_SIZE! () += inserted;
  ZV! () = BEG as ptrdiff_t;
  Z! () = BEG as ptrdiff_t;
  GPT! () = BEG as ptrdiff_t;
  ZV_BYTE! () = BEG_BYTE as ptrdiff_t;
  Z_BYTE! () = BEG_BYTE as ptrdiff_t;
  GPT_BYTE! () = BEG_BYTE as ptrdiff_t;

  /* Pass the new `inserted` back.  */
  XSETCAR (unwind_data, make_fixnum (inserted));

  /* Now we are safe to change the buffer's multibyteness directly.  */
  bset_enable_multibyte_characters ((current_buffer! ()), multibyte);
  bset_undo_list ((current_buffer! ()), undo_list);
}

/* Read from a non-regular file.  Return the number of bytes read.  */

/// C 3904-3921: `union read_non_regular`.  The anonymous `struct` is named
/// `read_non_regular_s` (PORTING.md 1.3: a nested anonymous struct must be named) and the
/// union carries `#[repr(align(8))]` in place of `GCALIGNED_UNION_MEMBER`
/// (`char alignas (GCALIGNMENT) gcaligned`, lisp.h 303; `GCALIGNMENT` is 8 on this target),
/// the keymap.rs/thread.rs shape.  clang measures the record on the frozen target:
/// `sizeof (union read_non_regular)` 32, `alignof` 8, `fd`@0, `buf`@8, `inserted`@16,
/// `bufsize`@24.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct read_non_regular_s
{
  /// File descriptor to read from.
  pub fd: emacs_fd,

  /// If BUF is non-null, read into BUF; otherwise, read into
  /// gap + INSERTED (done this way because the gap can relocate).
  pub buf: *mut c_char,
  pub inserted: ptrdiff_t,

  /// Number of bytes to try to read.
  pub bufsize: ptrdiff_t,
}

#[repr(C, align(8))]
pub union read_non_regular
{
  pub s: read_non_regular_s,
}
const _: () = core::assert! (std::mem::align_of::<read_non_regular> () % 8 == 0);  // static_assert (GCALIGNED (union read_non_regular));
const _: () = core::assert! (std::mem::size_of::<read_non_regular_s> () == 32);
const _: () = core::assert! (std::mem::align_of::<read_non_regular_s> () == 8);
const _: () = core::assert! (std::mem::offset_of! (read_non_regular_s, fd) == 0);
const _: () = core::assert! (std::mem::offset_of! (read_non_regular_s, buf) == 8);
const _: () = core::assert! (std::mem::offset_of! (read_non_regular_s, inserted) == 16);
const _: () = core::assert! (std::mem::offset_of! (read_non_regular_s, bufsize) == 24);
const _: () = core::assert! (std::mem::size_of::<read_non_regular> () == 32);

unsafe extern "C" fn read_non_regular (state: Lisp_Object) -> Lisp_Object
{
  let mut data: *mut read_non_regular = XFIXNUMPTR (state) as *mut read_non_regular;
  let mut nbytes: intmax_t
    = emacs_fd_read! ((*data).s.fd,
		      (if !(*data).s.buf.is_null () { (*data).s.buf }
		       else { (BEG_ADDR! ()).offset (PT_BYTE! () - BEG_BYTE as ptrdiff_t
						     + (*data).s.inserted) as *mut c_char }) as *mut c_void,
		      (*data).s.bufsize) as intmax_t;
  return make_int (if nbytes < 0 { - (*errno ()) as intmax_t } else { nbytes });
}

/* Condition-case handler used when reading from non-regular files
   in insert-file-contents.  */

unsafe extern "C" fn read_non_regular_quit (ignore: Lisp_Object) -> Lisp_Object
{
  return Qnil ();
}

/* Return the file offset that VAL represents, checking for type
   errors and overflow.  */
unsafe fn file_offset (val: Lisp_Object) -> off_t
{
  if INTEGERP (val)
    {
      let mut v: intmax_t = 0;
      if integer_to_intmax (val, &raw mut v) && 0 <= v && v <= TYPE_MAXIMUM! (off_t)
	{ return v; }
    }
  else if FLOATP (val)
    {
      let mut v: c_double = XFLOAT_DATA (val);
      if 0.0 <= v && v < 1.0 + TYPE_MAXIMUM! (off_t) as c_double
	{
	  let mut o: off_t = v as off_t;
	  if o as c_double == v
	    { return o; }
	}
    }

  wrong_type_argument (Qfile_offset (), val);
}

/* Return a special time value indicating the error number ERRNUM.  */
unsafe fn time_error_value (errnum: c_int) -> timespec
{
  let mut ns: c_int = if errnum == ENOENT || errnum == ENOTDIR
    { NONEXISTENT_MODTIME_NSECS } else { UNKNOWN_MODTIME_NSECS };
  return make_timespec (0, ns as c_long);
}

unsafe fn get_window_points_and_markers () -> Lisp_Object
{
  let mut pt_marker: Lisp_Object = Fpoint_marker ();
  let mut windows: Lisp_Object
    = calln! (Qget_buffer_window_list (), Fcurrent_buffer (), Qnil (), Qt ());
  let mut window_markers: Lisp_Object = windows;
  /* Window markers (and point) are handled specially: rather than move to
     just before or just after the modified text, we try to keep the
     markers at the same distance (bug#19161).
     In general, this is wrong, but for window-markers, this should be harmless
     and is convenient for the end user when most of the file is unmodified,
     except for a few minor details near the beginning and near the end.  */
  while CONSP (windows)
    {
      if WINDOWP (XCAR (windows))
	{
	  let mut window_marker: Lisp_Object = (*XWINDOW (XCAR (windows))).pointm;
	  XSETCAR (windows,
		    Fcons (window_marker, Fmarker_position (window_marker)));
	}
      windows = XCDR (windows);
    }
  return Fcons (Fcons (pt_marker, Fpoint ()), window_markers);
}

unsafe fn restore_window_points (mut window_markers: Lisp_Object, inserted: ptrdiff_t,
					same_at_start: ptrdiff_t, same_at_end: ptrdiff_t)
{
  while CONSP (window_markers)
    {
      if CONSP (XCAR (window_markers))
	{
	  let mut car: Lisp_Object = XCAR (window_markers);
	  let mut marker: Lisp_Object = XCAR (car);
	  let mut oldpos: Lisp_Object = XCDR (car);
	  if MARKERP (marker) && FIXNUMP (oldpos)
	    && XFIXNUM (oldpos) > same_at_start
	    && XFIXNUM (oldpos) <= same_at_end
	    {
	      let mut oldsize: ptrdiff_t = same_at_end - same_at_start;
	      let mut newsize: ptrdiff_t = inserted;
	      let mut growth: c_double = newsize as c_double / oldsize as c_double;
	      let mut newpos: ptrdiff_t
		= same_at_start + (growth * (XFIXNUM (oldpos) - same_at_start) as c_double)
		  as ptrdiff_t;
	      Fset_marker (marker, make_fixnum (newpos), Qnil ());
	    }
	}
      window_markers = XCDR (window_markers);
    }
}

/* Make sure the gap is at Z_BYTE.  This is required to treat buffer
   text as a linear C char array.  */
unsafe fn maybe_move_gap (b: *mut buffer)
{
  if BUF_GPT_BYTE! (b) != BUF_Z_BYTE! (b)
    {
      let mut cb: *mut buffer = (current_buffer! ());

      set_buffer_internal (b);
      move_gap_both (Z! (), Z_BYTE! ());
      set_buffer_internal (cb);
    }
}

/* In FD, position to POS relative to WHENCE.  Return the resulting
   position if successful, otherwise signal an error with FILENAME.  */
unsafe fn xlseek (fd: emacs_fd, pos: off_t, whence: c_int, filename: Lisp_Object)
  -> off_t
{
  let mut newpos: off_t = emacs_fd_lseek! (fd, pos, whence);
  if newpos < 0
    { report_file_error (c"Setting file position".as_ptr (), filename); }
  return newpos;
}

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

/// C: `<limits.h>`'s `#define SSIZE_MAX LONG_MAX` (the frozen target's value) comes from
/// the limits.in.rs mirror, which this unit includes; the use-site copy is gone
/// (PORTING.md 0.1-3).
/// C: `<unistd.h>` 4380: `#define _PC_CASE_SENSITIVE 11` - `unistd.in.rs` carries no
/// `_PC_*` name, so it is declared here (the frozen target's measured value).
pub const _PC_CASE_SENSITIVE: c_int = 11;

unsafe extern "C" {
  /// C: lisp.h 4320: `extern AVOID buffer_overflow (void);` - insdel.rs defines it, so the
  /// declaration copies that definition's signature (no `-> !`).
  pub fn buffer_overflow ();
  /// C: lisp.h 4786: `extern Lisp_Object quit (void);` - eval.rs defines it.
  pub fn quit () -> Lisp_Object;
  /// C: coding.h 684: `extern Lisp_Object code_conversion_save (bool, bool);`
  pub fn code_conversion_save (buffer: bool, multibyte: bool) -> Lisp_Object;
  /// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
  pub fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
  /// C: coding.h 700: `extern Lisp_Object raw_text_coding_system (Lisp_Object);`
  pub fn raw_text_coding_system (coding_system: Lisp_Object) -> Lisp_Object;
  /// C: coding.h 702: `extern Lisp_Object coding_inherit_eol_type (Lisp_Object, Lisp_Object);`
  pub fn coding_inherit_eol_type (coding_system: Lisp_Object, eol: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4218: `extern Lisp_Object detect_coding_system (const unsigned char *,
  /// ptrdiff_t, ptrdiff_t, bool, bool, Lisp_Object);`
  pub fn detect_coding_system (src: *const c_uchar, src_bytes: ptrdiff_t,
			       src_chars: ptrdiff_t, src_multibyte: bool,
			       highest: bool, coding_system: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4331: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t, bool,
  /// bool, bool);`
  pub fn insert_1_both (string: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
			inherit: bool, prepare: bool, before_markers: bool);
  /// C: lisp.h 4337: `extern void insert_from_buffer (struct buffer *, ptrdiff_t,
  /// ptrdiff_t, bool);`
  pub fn insert_from_buffer (buf: *mut buffer, charpos: ptrdiff_t, nchars: ptrdiff_t,
			     inherit: bool);
  /// C: lisp.h 4347: `extern void del_range_byte (ptrdiff_t, ptrdiff_t);`
  pub fn del_range_byte (from: ptrdiff_t, to: ptrdiff_t);
  /// C: lisp.h 4806: `extern Lisp_Object internal_condition_case_1 (Lisp_Object (*)
  /// (Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  pub fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
				    arg: Lisp_Object, handlers: Lisp_Object,
				    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  /// C: lisp.h 4829: `extern void clear_unwind_protect (specpdl_ref);`
  pub fn clear_unwind_protect (count: specpdl_ref);
}

DEFUN! ("insert-file-contents", Finsert_file_contents, Sinsert_file_contents,
       1, 5, 0,
       doc: "Insert contents of file FILENAME after point.\nReturns list of absolute file name and number of characters inserted.\nIf second argument VISIT is non-nil, the buffer's visited filename and\nlast save file modtime are set, and it is marked unmodified.  Signal an\nerror if FILENAME cannot be read or is a directory.  If visiting and the\nfile does not exist, visiting is completed before the error is signaled.\n\nThe optional third and fourth arguments BEG and END specify what portion\nof the file to insert.  These arguments count bytes in the file, not\ncharacters in the buffer.  If VISIT is non-nil, BEG and END must be nil.\n\nWhen inserting data from a special file (e.g., /dev/urandom), you\ncan't specify VISIT or BEG, and END should be specified to avoid\ninserting unlimited data into the buffer from some special files\nwhich otherwise could supply infinite amounts of data.\n\nIf optional fifth argument REPLACE is non-nil and FILENAME names a\nregular file, replace the current buffer contents (in the accessible\nportion) with the file's contents.  This is better than simply\ndeleting and inserting the whole thing because (1) it preserves some\nmarker positions (in unchanged portions at the start and end of the\nbuffer) and (2) it puts less data in the undo list.  When REPLACE is\nnon-nil, the second element of the return value is the number of\ncharacters that replace the previous buffer contents.\n\nIf FILENAME is not a regular file and REPLACE is `if-regular', erase\nthe accessible portion of the buffer and insert the new contents.  Any\nother non-nil value of REPLACE will signal an error if FILENAME is not\na regular file.\n\nThis function does code conversion according to the value of\n`coding-system-for-read' or `file-coding-system-alist', and sets the\nvariable `last-coding-system-used' to the coding system actually used.\n\nIn addition, this function decodes the inserted text from known formats\nby calling `format-decode', which see.",
       (filename: Lisp_Object, visit: Lisp_Object, beg: Lisp_Object, end: Lisp_Object,
        replace: Lisp_Object),
       {
  /* A good read blocksize for insert-file-contents.
     It is for reading a big chunk of a file into memory,
     as opposed to coreutils IO_BUFSIZE which is for 'cat'-like stream reads.
     If too small, insert-file-contents has more syscall overhead.
     If too large, insert-file-contents might take too long respond to a quit.
     1 MiB should be reasonable even for older, slower devices circa 2025.  */
  /* C 4073: `enum { INSERT_READ_SIZE_MAX = min (1024 * 1024, SYS_BUFSIZE_MAX) };` - the
     frozen target's SYS_BUFSIZE_MAX is 2146435072 (sys-limits.rs), so the first arm wins.  */
  const INSERT_READ_SIZE_MAX: c_int
    = if 1024 * 1024 < SYS_BUFSIZE_MAX { 1024 * 1024 } else { SYS_BUFSIZE_MAX };

  let mut mtime: timespec = std::mem::zeroed ();  /* C leaves it uninitialised (PORTING.md 1.5) */
  let mut fd: emacs_fd;
  let mut inserted: ptrdiff_t = 0;
  let mut unprocessed: c_int;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut handler: Lisp_Object;
  let mut val: Lisp_Object;
  let mut insval: Lisp_Object;
  let mut orig_filename: Lisp_Object;
  let mut old_undo: Lisp_Object;
  let mut p: Lisp_Object;
  let mut total: off_t = 0;
  let mut regular: bool;
  let mut save_errno: c_int = 0;
  /* C: `char read_buf[min (+MAX_ALLOCA, +INSERT_READ_SIZE_MAX)];` - MAX_ALLOCA (16384) is
     the smaller of the two on the frozen target, so the array's length is MAX_ALLOCA.  */
  let mut read_buf: [c_char; MAX_ALLOCA as usize] = std::mem::zeroed ();
  let mut coding: coding_system = std::mem::zeroed ();
  let mut replace_handled: bool = false;
  let mut set_coding_system: bool = false;
  let mut coding_system: Lisp_Object;
  /* errno if read error, 0 if OK so far, negative if quit.  */
  let mut read_quit: c_int = 0;
  /* If the undo log only contains the insertion, there's no point
     keeping it.  It's typically when we first fill a file-buffer.  */
  let mut empty_undo_list_p: bool
    = (!NILP (visit) && NILP (BVAR! ((current_buffer! ()), undo_list_))
       && BEG as ptrdiff_t == Z! ());
  let mut old_Vdeactivate_mark: Lisp_Object = Vdeactivate_mark;
  let mut we_locked_file: bool = false;
  let mut window_markers: Lisp_Object = Qnil ();
  /* same_at_start and same_at_end count bytes, because file access counts
     bytes and BEG and END count bytes.  */
  let mut same_at_start: ptrdiff_t = BEGV_BYTE! ();
  let mut same_at_end: ptrdiff_t = ZV_BYTE! ();
  /* SAME_AT_END_CHARPOS counts characters, because
     restore_window_points needs the old character count.  */
  let mut same_at_end_charpos: ptrdiff_t = ZV! ();

  /* The size reported by fstat, or -1 if the file was not found or its
     size is meaningless.  */
  let mut st_size: off_t = -1;

  let mut filename = filename;                /* C reassigns `filename` below.  */
  let mut replace = replace;                  /* C reassigns `replace` below.  */

  if !(*(current_buffer! ())).base_buffer.is_null () && ! NILP (visit)
    { error (c"Cannot do file visiting in an indirect buffer".as_ptr ()); }

  if !NILP (BVAR! ((current_buffer! ()), read_only_))
    { Fbarf_if_buffer_read_only (Qnil ()); }

  val = Qnil ();
  p = Qnil ();
  old_undo = Qnil ();

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());
  orig_filename = filename;

  /* The value Qnil means that the coding system is not yet
     decided.  */
  coding_system = Qnil ();

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qinsert_file_contents ());

  /* `notfound:`/`handled:` are C labels; the port keeps them as labelled blocks.  `goto
     notfound` is `break 'chunk_a` (the block of C's code before the `notfound:` label) and
     `goto handled` is `break 'notfound` (the block of the `notfound:` text itself, which the
     `handled:` text follows); falling out of `'chunk_a` reaches the `notfound:` text.  */
  /* C's `handled:` label: the port's outer labelled loop carries the name (its `break` is
     not used - `goto handled` is `break 'notfound` below, which lands at the `handled:` text);
     `goto notfound` is `break 'chunk_a`.  See `docs/ports/fileio.md`.  */
  'handled: loop
    {
      'notfound: {
	'chunk_a: {
      if !NILP (handler)
	{
	  val = calln! (handler, Qinsert_file_contents (), filename,
			visit, beg, end, replace);
	  if CONSP (val) && CONSP (XCDR (val))
	    && RANGED_FIXNUMP (0, XCAR (XCDR (val)), ZV! () - PT! ())
	    { inserted = XFIXNUM (XCAR (XCDR (val))); }
	  break 'notfound;                       /* goto handled  */
	}

      if !NILP (visit)
	{
	  if !NILP (beg) || !NILP (end)
	    { error (c"Attempt to visit less than an entire file".as_ptr ()); }
	  if (BEG as ptrdiff_t) < Z! () && NILP (replace)
	    { error (c"Cannot do file visiting in a non-empty buffer".as_ptr ()); }
	}

      let mut beg_offset: off_t = if !NILP (beg) { file_offset (beg) } else { 0 };
      let mut end_offset: off_t = if !NILP (end) { file_offset (end) }
	else { TYPE_MAXIMUM! (off_t) };

      /* END_OFFSET should never be less than BEG_OFFSET,
	 so that code can assume that END_OFFSET - BEG_OFFSET is nonnegative.
	 So set END_OFFSET = max (END_OFFSET, BEG_OFFSET)
	 whenever setting either variable,
	 unless it is already known that BEG_OFFSET <= END_OFFSET.  */
      end_offset = std::cmp::max (end_offset, beg_offset);

      filename = ENCODE_FILE (filename);

      fd = emacs_fd_open! (SSDATA (filename), O_RDONLY, 0);
      if !emacs_fd_valid_p! (fd)
	{
	  save_errno = *errno ();
	  if NILP (visit)
	    { report_file_error (c"Opening input file".as_ptr (), orig_filename); }
	  mtime = time_error_value (save_errno);
	  if !NILP (Vcoding_system_for_read)
	    {
	      /* Don't let invalid values into buffer-file-coding-system.  */
	      CHECK_CODING_SYSTEM (Vcoding_system_for_read);
	      Fset (Qbuffer_file_coding_system (), Vcoding_system_for_read);
	    }
	  eassert! (inserted == 0);
	  break 'chunk_a;                        /* goto notfound  */
	}

      let fd_index: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_protect_ptr (Some (close_file_unwind_emacs_fd), &raw mut fd as *mut c_void);

      /* Replacement should preserve point as it preserves markers.  */
      if !NILP (replace)
	{
	  window_markers = get_window_points_and_markers ();
	  record_unwind_protect (Some (restore_point_unwind),
				 XCAR (XCAR (window_markers)));
	}

      {
	let mut st: stat = std::mem::zeroed ();
	if emacs_fd_fstat! (fd, &raw mut st) < 0
	  { report_file_error (c"Input file status".as_ptr (), orig_filename); }

	/* Normally there is no need for an S_ISDIR test here,
	   as the first 'read' syscall will fail with EISDIR.
	   However, for backwards compatibility to traditional Unix,
	   POSIX allows 'read' to succeed on directories.
	   So do an explicit S_ISDIR test now, so that callers can rely on
	   this function rejecting directories on all platforms.  */
	if S_ISDIR! (st.st_mode)
	  { report_file_errno (c"Read error".as_ptr (), orig_filename, EISDIR); }

	regular = S_ISREG! (st.st_mode);
	let mut memory_object: bool = S_TYPEISSHM! (&raw mut st) != 0 || S_TYPEISTMO! (&raw mut st) != 0;

	if regular || memory_object
	  {
	    st_size = st.st_size;

	    /* On some ancient platforms, a regular file with a negative size means
	       the actual file size is greater than the maximum off_t value.
	       Fail now rather than doing a lot of work and exhausting memory.  */
	    if st_size < 0
	      { buffer_overflow (); }
	  }

	mtime = if memory_object
	  { make_timespec (0, UNKNOWN_MODTIME_NSECS as c_long) }
	else { get_stat_mtime (&raw mut st) };
      }

      /* The REPLACE code will need to be changed in order to work on
	 named pipes, and it's probably just not worth it.  So we should
	 at least signal an error.  */

      if !regular
	{
	  if !NILP (visit) || !NILP (replace)
	    {
	      if !NILP (visit) || !EQ (replace, Qif_regular ())
		{ xsignal2 (Qfile_error (),
			    build_string (c"not a regular file".as_ptr ()), orig_filename); }
	      else
		/* Set REPLACE to Qunbound, indicating that we are trying
		   to replace the buffer contents with that of a
		   non-regular file.  */
		{ replace = Qunbound (); }
	    }

	  /* Forbid specifying BEG unless the file is regular, as per
	     the doc string.  */

	  if !NILP (beg)
	    { xsignal2 (Qfile_error (),
			build_string (c"can use a start position only in a regular file".as_ptr ()),
			orig_filename); }
	}

      /* Check now whether the buffer will become too large,
	 in the likely case where the file's length is not changing.
	 This saves a lot of needless work before a buffer overflow.
	 If LIKELY_END is nonnegative, it is likely where we will stop reading.
	 We could read more (or less), if the file grows (or shrinks).  */
      let mut likely_end: off_t = std::cmp::min (end_offset, st_size);
      if beg_offset < likely_end
	{
	  let mut buf_bytes: ptrdiff_t
	    = Z_BYTE! () - (if !NILP (replace) { ZV_BYTE! () - BEGV_BYTE! () } else { 0 });
	  let mut buf_growth_max: ptrdiff_t = BUF_BYTES_MAX - buf_bytes;
	  let mut likely_growth: off_t = likely_end - beg_offset;
	  if buf_growth_max < likely_growth as ptrdiff_t
	    { buffer_overflow (); }
	}

      /* Prevent redisplay optimizations.  */
      (*(current_buffer! ())).bits |= BUFFER_CLIP_CHANGED;   /* C: `->clip_changed = true` */

      /* The current input position if known, -1 otherwise.  */
      let mut curpos: off_t = -1;

      if EQ (Vcoding_system_for_read, Qauto_save_coding ())
	{
	  coding_system = coding_inherit_eol_type (Qutf_8_emacs (), Qunix ());
	  setup_coding_system (coding_system, &raw mut coding);
	  /* Ensure we set Vlast_coding_system_used.  */
	  set_coding_system = true;
	}
      else if (BEG as ptrdiff_t) < Z! ()
	{
	  /* Decide the coding system to use for reading the file now
	     because we can't use an optimized method for handling
	     `coding:' tag if the current buffer is not empty.  */
	  if !NILP (Vcoding_system_for_read)
	    { coding_system = Vcoding_system_for_read; }
	  else
	    {
	      /* Don't try looking inside a file for a coding system
		 specification if it is not a regular file.  */
	      if regular && !NILP (Vset_auto_coding_function)
		{
		  if !NILP (beg)
		    { curpos = xlseek (fd, beg_offset, SEEK_SET, orig_filename); }
		  else
		    {
		      beg_offset = xlseek (fd, 0, SEEK_CUR, orig_filename);
		      curpos = beg_offset;
		      end_offset = std::cmp::max (end_offset, beg_offset);
		    }

		  /* Find a coding system specified in the heading two
		     lines or in the tailing several lines of the file.
		     Assume that the 1 KiB and 3 KiB for heading
		     and tailing respectively are sufficient for this
		     purpose.  Because the file may be in /proc,
		     do not use st_size or report any SEEK_END failure.  */
		  /* C 4386: `static_assert (4 * 1024 < sizeof read_buf);`  */
		  static_assert! (4 * 1024 < std::mem::size_of::<[c_char; MAX_ALLOCA as usize]> ());
		  let mut trial: ptrdiff_t = std::cmp::min ((end_offset - beg_offset) as ptrdiff_t, 4 * 1024);
		  let mut nread: ptrdiff_t
		    = if trial != 0 { emacs_full_read (fd, read_buf.as_mut_ptr () as *mut c_void, trial) } else { 0 };
		  curpos += nread as off_t;

		  if nread == 4 * 1024 && curpos < end_offset
		    {
		      curpos = emacs_fd_lseek! (fd, - 3 * 1024, SEEK_END);
		      if curpos < 0
			{ curpos = beg_offset + nread as off_t; }
		      else
			{
			  let mut tailbeg: off_t
			    = if curpos <= beg_offset + nread as off_t { beg_offset + nread as off_t }
			    else { std::cmp::min (curpos, end_offset - 3 * 1024) };
			  if tailbeg != curpos
			    { curpos = xlseek (fd, tailbeg, SEEK_SET, orig_filename); }
			}

		      /* When appending the last 3 KiB, read extra bytes
			 without trusting SEEK_END, as the file may be growing.
			 Although this may yield more than 4 KiB of data total,
			 and the trailing data may not be from file end if
			 the file is growing, it is good enough.  */
		      let mut trial: ptrdiff_t = std::cmp::min ((end_offset - curpos) as ptrdiff_t,
								std::mem::size_of_val (&read_buf) as ptrdiff_t - 1024);
		      nread = emacs_full_read (fd,
					       read_buf.as_mut_ptr ().offset (1024) as *mut c_void,
					       trial);
		      if 0 <= nread
			{
			  curpos += nread as off_t;
			  nread += 1024;
			}
		    }

		  if nread < 0
		    { report_file_error (c"Read error".as_ptr (), orig_filename); }
		  else if nread > 0
		    {
		      AUTO_STRING! (name, c" *code-converting-work*".as_ptr ());
		      let mut prev: *mut buffer = (current_buffer! ());
		      let mut workbuf: Lisp_Object;
		      let mut buf: *mut buffer;

		      record_unwind_current_buffer ();

		      workbuf = Fget_buffer_create (name, Qt ());
		      buf = XBUFFER (workbuf);

		      delete_all_overlays (buf);
		      bset_directory (buf, BVAR! ((current_buffer! ()), directory_));
		      bset_read_only (buf, Qnil ());
		      bset_filename (buf, Qnil ());
		      bset_undo_list (buf, Qt ());
		      eassert! ((*buf).overlays.is_null ());

		      set_buffer_internal (buf);
		      Ferase_buffer ();
		      bset_enable_multibyte_characters (buf, Qnil ());

		      insert_1_both (read_buf.as_mut_ptr (), nread, nread, false, false, false);
		      TEMP_SET_PT_BOTH (BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t);
		      coding_system = calln! (Vset_auto_coding_function,
					      filename, make_fixnum (nread));
		      set_buffer_internal (prev);

		      /* Discard the unwind protect for recovering the
                         current buffer.  */
		      (specpdl_ptr! ()) = (specpdl_ptr! ()).offset (-1);
		    }
		}

	      if NILP (coding_system)
		{
		  /* If we have not yet decided a coding system, check
                     file-coding-system-alist.  */
		  coding_system = CALLN! (Ffind_operation_coding_system,
					  Qinsert_file_contents (), orig_filename,
					  visit, beg, end, replace);
		  if CONSP (coding_system)
		    { coding_system = XCAR (coding_system); }
		}
	    }

	  if NILP (coding_system)
	    { coding_system = Qundecided (); }
	  else
	    { CHECK_CODING_SYSTEM (coding_system); }

	  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	    /* We must suppress all character code conversion except for
	       end-of-line conversion.  */
	    { coding_system = raw_text_coding_system (coding_system); }

	  setup_coding_system (coding_system, &raw mut coding);
	  /* Ensure we set Vlast_coding_system_used.  */
	  set_coding_system = true;
	}

      /* If requested, replace the accessible part of the buffer
	 with the file contents.  Avoid replacing text at the
	 beginning or end of the buffer that matches the file contents;
	 that preserves markers pointing to the unchanged parts.

	 Here we implement this feature in an optimized way
	 for the case where code conversion is NOT needed.
	 The following if-statement handles the case of conversion
	 in a less optimal way.

	 If the code conversion is "automatic" then we try using this
	 method and hope for the best.
	 But if we discover the need for conversion, we give up on this method
	 and let the following if-statement handle the replace job.  */
      if (!NILP (replace)
	   && !BASE_EQ (replace, Qunbound ()))
	  && BEGV! () < ZV! ()
	  && (NILP (coding_system)
	      || CODING_REQUIRE_DECODING (&raw mut coding) == 0)
	{
	  /* There is still a possibility we will find the need to do code
	     conversion.  If that happens, set this variable to
	     give up on handling REPLACE in the optimized way.  */
	  let mut giveup_match_end: bool = false;

	  if beg_offset != curpos
	    {
	      if !NILP (beg) || 0 <= curpos
		{ curpos = xlseek (fd, beg_offset, SEEK_SET, orig_filename); }
	      else
		{
		  beg_offset = xlseek (fd, 0, SEEK_CUR, orig_filename);
		  curpos = beg_offset;
		  end_offset = std::cmp::max (end_offset, beg_offset);
		}
	    }

	  /* Count how many chars at the start of the file
	     match the text at the beginning of the buffer.  */
	  loop
	    {
	      let mut trial: ptrdiff_t = std::cmp::min ((end_offset - curpos) as ptrdiff_t,
							std::mem::size_of_val (&read_buf) as ptrdiff_t);
	      let mut nread: ptrdiff_t
		= if trial != 0
		{ emacs_fd_read! (fd, read_buf.as_mut_ptr () as *mut c_void, trial) } else { 0 };
	      if nread < 0
		{ report_file_error (c"Read error".as_ptr (), orig_filename); }
	      else if nread == 0
		{
		  /* Data inserted from the file match the buffer's leading bytes,
		     so there's no need to replace anything.  */
		  emacs_fd_close! (fd);
		  clear_unwind_protect (fd_index);

		  /* Truncate the buffer to the size of the file.  */
		  del_range_byte (same_at_start, same_at_end);
		  break 'notfound;                     /* goto handled  */
		}

	      curpos += nread as off_t;

	      if CODING_REQUIRE_DETECTION (&raw mut coding) != 0
		{
		  coding_system = detect_coding_system (read_buf.as_mut_ptr () as *mut c_uchar,
							nread, nread, true, false,
							coding_system);
		  setup_coding_system (coding_system, &raw mut coding);
		}

	      if CODING_REQUIRE_DECODING (&raw mut coding) != 0
		/* We found that the file should be decoded somehow.
                   Let's give up here.  */
		{
		  giveup_match_end = true;
		  break;
		}

	      let mut bufpos: ptrdiff_t = 0;
	      let mut bufposlim: ptrdiff_t = std::cmp::min (nread, same_at_end - same_at_start);
	      while bufpos < bufposlim
		&& FETCH_BYTE! (same_at_start) == read_buf[bufpos as usize] as u8
		{ same_at_start += 1; bufpos += 1; }
	      /* If we found a discrepancy, stop the scan.  */
	      if bufpos != nread
		{ break; }
	    }

	  /* Find the end position, which is end_offset if given,
	     the file's end otherwise.  */

	  let mut endpos: off_t = 0;   /* C: `off_t endpos UNINIT;` (PORTING.md 1.5)  */
	  if !giveup_match_end
	    {
	      endpos = end_offset;
	      if endpos == TYPE_MAXIMUM! (off_t)
		{
		  endpos = emacs_fd_lseek! (fd, 0, SEEK_END);
		  if endpos < 0
		    { endpos = curpos; }

		  /* Check that read reports EOF soon, to catch platforms
		     where SEEK_END fails or reports too-small offsets.  */
		  let mut n: ptrdiff_t = emacs_full_read (fd, read_buf.as_mut_ptr () as *mut c_void,
							  std::mem::size_of_val (&read_buf) as ptrdiff_t);
		  if n < 0
		    { report_file_error (c"Read error".as_ptr (), orig_filename); }
		  endpos += n as off_t;
		  curpos = endpos;

		  /* Give up if the file extends past the test read.  */
		  giveup_match_end = n == std::mem::size_of_val (&read_buf) as ptrdiff_t;

		  if !giveup_match_end
		    {
		      /* Shrink the file's head if the file shrank to
			 be smaller than its head.  */
		      let mut offset_from_beg: off_t = endpos - beg_offset;
		      if offset_from_beg < (same_at_start - BEGV_BYTE! ()) as off_t
			{
			  /* Give up if the file shrank to less than BEG.  */
			  giveup_match_end = offset_from_beg < 0;

			  if !giveup_match_end
			    { same_at_start = (offset_from_beg + BEGV_BYTE! () as off_t) as ptrdiff_t; }
			}
		    }
		}
	    }

	  /* Count how many bytes in the file's end match the buffer's end.
	     However, don't waste time if decoding is necessary.  */

	  while !giveup_match_end
	    {
	      let mut nread: ptrdiff_t;
	      let mut bufpos: ptrdiff_t;
	      let mut trial: ptrdiff_t;

	      /* How much can we scan in the next step?  Compare with poslim
		 to prevent overlap of the matching head with the matching tail.
		 The 'same_at_start_pos' limit prevents overlap in the buffer's
		 head and tail, and the 'file_overlap_pos' limit prevents
		 overlap in the inserted file's head and tail.  */
	      let mut same_at_start_pos: off_t = beg_offset + (same_at_start - BEGV_BYTE! ()) as off_t;
	      let mut file_overlap_pos: off_t = endpos - (same_at_end - same_at_start) as off_t;
	      let mut poslim: off_t = std::cmp::max (same_at_start_pos, file_overlap_pos);
	      /* Do not scan more than sizeof read_buf at a time, and stop
		 the scan if it can go no more.  */
	      trial = std::cmp::min ((curpos - poslim) as ptrdiff_t,
				     std::mem::size_of_val (&read_buf) as ptrdiff_t);
	      if trial == 0
		{ break; }

	      curpos = xlseek (fd, -(trial as off_t), SEEK_CUR, orig_filename);

	      nread = emacs_full_read (fd, read_buf.as_mut_ptr () as *mut c_void, trial);
	      curpos += nread as off_t;
	      if nread < trial
		{
		  if nread < 0
		    { report_file_error (c"Read error".as_ptr (), orig_filename); }
		  /* The file unexpectedly shrank.  */
		  giveup_match_end = true;
		  break;
		}

	      /* Scan this bufferful from the end, comparing with
		 the buffer.  */
	      bufpos = nread;

	      while bufpos > 0
		&& FETCH_BYTE! (same_at_end - 1) == read_buf[(bufpos - 1) as usize] as u8
		{ same_at_end -= 1; bufpos -= 1; }

	      /* If we found a discrepancy, stop the scan.
		 Otherwise loop around and scan the preceding bufferful.  */
	      if bufpos != 0
		{
		  /* If this discrepancy is because of code conversion,
		     we cannot use this method; giveup and try the other.  */
		  if same_at_end > same_at_start
		    && FETCH_BYTE! (same_at_end - 1) >= 0o200
		    && ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		    && CODING_MAY_REQUIRE_DECODING (&raw mut coding) != 0
		    { giveup_match_end = true; }
		  break;
		}
	    }

	  if ! giveup_match_end
	    {
	      let mut temp: ptrdiff_t;
	      let this_count: specpdl_ref = SPECPDL_INDEX ();

	      /* We win!  We can handle REPLACE the optimized way.  */

	      /* Extend the start of non-matching text area to multibyte
                 character boundary.  */
	      if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		{
		  while same_at_start > BEGV_BYTE! ()
		    && ! CHAR_HEAD_P (FETCH_BYTE! (same_at_start) as c_int)
		    { same_at_start -= 1; }
		}

	      /* Extend the end of non-matching text area to multibyte
                 character boundary.  */
	      if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
		{
		  while same_at_end < ZV_BYTE! ()
		    && ! CHAR_HEAD_P (FETCH_BYTE! (same_at_end) as c_int)
		    { same_at_end += 1; }
		}

	      same_at_end_charpos = BYTE_TO_CHAR (same_at_end);

	      /* Arrange to read only the nonmatching middle part of the file.  */
	      beg_offset += (same_at_start - BEGV_BYTE! ()) as off_t;
	      end_offset = endpos - (ZV_BYTE! () - same_at_end) as off_t;
	      eassert! (beg_offset <= end_offset);

	      if !NILP (visit) && BEG as ptrdiff_t == BEGV! () && Z! () == ZV! ()
		/* This binding is to avoid ask-user-about-supersession-threat
		   being called in insert_from_buffer or del_range_bytes (via
		   prepare_to_modify_buffer).
		   Such a prompt makes no sense if we're VISITing the file,
		   since the insertion makes the buffer *more* like the file
		   rather than the reverse.
                   AFAICT we could avoid ask-user-about-supersession-threat by
                   setting current_buffer->modtime earlier, but we could still
                   end up calling ask-user-about-supersession-threat if the file
                   is modified while we read it, so we bind buffer-file-name
                   instead.  */
		{ specbind (Qbuffer_file_name (), Qnil ()); }
	      del_range_byte (same_at_start, same_at_end);
	      /* Insert from the file at the proper position.  */
	      temp = BYTE_TO_CHAR (same_at_start);
	      SET_PT_BOTH (temp, same_at_start);
	      unbind_to (this_count, Qnil ());

	      /* If display currently starts at beginning of line,
		 keep it that way.  */
	      if XBUFFER ((*XWINDOW (selected_window)).contents) == (current_buffer! ())
		{ (*XWINDOW (selected_window)).bits = if !NILP (Fbolp ()) { (*XWINDOW (selected_window)).bits | (1u32 << WINDOW_START_AT_LINE_BEG_SHIFT) } else { (*XWINDOW (selected_window)).bits & !(1u32 << WINDOW_START_AT_LINE_BEG_SHIFT) }; }   /* C: `->start_at_line_beg = !NILP (Fbolp ())` */

	      replace_handled = true;
	    }
	}

      /* If requested, replace the accessible part of the buffer
	 with the file contents.  Avoid replacing text at the
	 beginning or end of the buffer that matches the file contents;
	 that preserves markers pointing to the unchanged parts.

	 Here we implement this feature for the case where code conversion
	 is needed, in a simple way that needs a lot of memory.
	 The preceding if-statement handles the case of no conversion
	 in a more optimized way.  */
      if (!NILP (replace)
	   && !BASE_EQ (replace, Qunbound ()))
	  && ! replace_handled && BEGV! () < ZV! ()
	{
	  let mut same_at_start_charpos: ptrdiff_t;
	  let mut inserted_chars: ptrdiff_t;
	  let mut bufpos: ptrdiff_t;
	  let mut decoded: *mut c_uchar;
	  let mut temp: ptrdiff_t;
	  let mut this: ptrdiff_t;
	  let this_count: specpdl_ref = SPECPDL_INDEX ();
	  let mut multibyte: bool
	    = ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
	  let mut conversion_buffer: Lisp_Object;

	  conversion_buffer = code_conversion_save (true, multibyte);

	  /* First read the whole file, performing code conversion into
	     CONVERSION_BUFFER.  */

	  if beg_offset != curpos
	    {
	      if !NILP (beg) || 0 <= curpos
		{ curpos = xlseek (fd, beg_offset, SEEK_SET, orig_filename); }
	      else
		{
		  beg_offset = xlseek (fd, 0, SEEK_CUR, orig_filename);
		  curpos = beg_offset;
		  end_offset = std::cmp::max (end_offset, beg_offset);
		}
	    }

	  inserted = 0;		/* Bytes put into CONVERSION_BUFFER so far.  */
	  unprocessed = 0;	/* Bytes not processed in previous loop.  */

	  loop
	    {
	      /* Read one buffer a time, to allow
		 quitting while reading a huge file.  */

	      let mut trial: ptrdiff_t
		= std::cmp::min ((end_offset - curpos) as ptrdiff_t,
				 std::mem::size_of_val (&read_buf) as ptrdiff_t - unprocessed as ptrdiff_t);
	      this = if trial != 0
		{ emacs_fd_read! (fd, read_buf.as_mut_ptr ().offset (unprocessed as isize) as *mut c_void, trial) }
	      else { 0 };
	      if this < 0
		{ report_file_error (c"Read error".as_ptr (), orig_filename); }
	      if this == 0
		{ break; }
	      curpos += this as off_t;

	      BUF_TEMP_SET_PT (XBUFFER (conversion_buffer),
			       BUF_Z! (XBUFFER (conversion_buffer)));
	      decode_coding_c_string (&raw mut coding, read_buf.as_mut_ptr () as *mut c_uchar,
				      unprocessed as ptrdiff_t + this, conversion_buffer);
	      unprocessed = coding.carryover_bytes;
	      if coding.carryover_bytes > 0
		{ memcpy (read_buf.as_mut_ptr () as *mut c_void,
			  coding.carryover.as_ptr () as *const c_void,
			  unprocessed as usize); }
	    }

	  emacs_fd_close! (fd);
	  clear_unwind_protect (fd_index);

	  if unprocessed > 0
	    {
	      BUF_TEMP_SET_PT (XBUFFER (conversion_buffer),
			       BUF_Z! (XBUFFER (conversion_buffer)));
	      coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;
	      decode_coding_c_string (&raw mut coding, read_buf.as_mut_ptr () as *mut c_uchar,
				      unprocessed as ptrdiff_t, conversion_buffer);
	      coding.bits &= !((CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT);
	    }

	  coding_system = CODING_ID_NAME (coding.id as hash_idx_t);
	  set_coding_system = true;
	  maybe_move_gap (XBUFFER (conversion_buffer));
	  decoded = BUF_BEG_ADDR! (XBUFFER (conversion_buffer)) as *mut c_uchar;
	  inserted = (BUF_Z_BYTE! (XBUFFER (conversion_buffer))
		      - BUF_BEG_BYTE (XBUFFER (conversion_buffer)));

	  /* Compare the beginning of the converted string with the buffer
	     text.  */

	  bufpos = 0;
	  let mut bufposlim: ptrdiff_t = std::cmp::min (inserted, same_at_end - same_at_start);
	  while bufpos < bufposlim
	    && FETCH_BYTE! (same_at_start) == *decoded.offset (bufpos)
	    { same_at_start += 1; bufpos += 1; }

	  /* If the file matches the head of buffer completely,
	     there's no need to replace anything.  */

	  if bufpos == inserted
	    {
	      /* Truncate the buffer to the size of the file.  */
	      if same_at_start != same_at_end
		{
		  if !NILP (visit) && BEG as ptrdiff_t == BEGV! () && Z! () == ZV! ()
		    /* See previous specbind for the reason behind this.  */
		    { specbind (Qbuffer_file_name (), Qnil ()); }
		  del_range_byte (same_at_start, same_at_end);
		}
	      inserted = 0;

	      unbind_to (this_count, Qnil ());
	      break 'notfound;                       /* goto handled  */
	    }

	  /* Extend the start of non-matching text area to the previous
	     multibyte character boundary.  */
	  if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	    {
	      while same_at_start > BEGV_BYTE! ()
		&& ! CHAR_HEAD_P (FETCH_BYTE! (same_at_start) as c_int)
		{ same_at_start -= 1; }
	    }

	  /* Scan this bufferful from the end, comparing with the
	     buffer.  Compare with bufposlim to prevent overlap of the
	     matching head with the matching tail.  The 'same_at_start -
	     BEGV_BYTE' limit prevents overlap in the buffer's head and
	     tail, and the 'inserted - (same_at_end - same_at_start)' limit
	     prevents overlap in the inserted file's head and tail.  */
	  bufposlim = std::cmp::max (same_at_start - BEGV_BYTE! (),
				     inserted - (same_at_end - same_at_start));
	  bufpos = inserted;
	  while bufposlim < bufpos
	    && FETCH_BYTE! (same_at_end - 1) == *decoded.offset (bufpos - 1)
	    { same_at_end -= 1; bufpos -= 1; }

	  /* Extend the end of non-matching text area to the next
	     multibyte character boundary.  */
	  if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	    {
	      while same_at_end < ZV_BYTE! ()
		&& ! CHAR_HEAD_P (FETCH_BYTE! (same_at_end) as c_int)
		{ same_at_end += 1; }
	    }

	  same_at_end_charpos = BYTE_TO_CHAR (same_at_end);

	  /* If display currently starts at beginning of line,
	     keep it that way.  */
	  if XBUFFER ((*XWINDOW (selected_window)).contents) == (current_buffer! ())
	    { (*XWINDOW (selected_window)).bits = if !NILP (Fbolp ()) { (*XWINDOW (selected_window)).bits | (1u32 << WINDOW_START_AT_LINE_BEG_SHIFT) } else { (*XWINDOW (selected_window)).bits & !(1u32 << WINDOW_START_AT_LINE_BEG_SHIFT) }; }   /* C: `->start_at_line_beg = !NILP (Fbolp ())` */

	  /* Replace the chars that we need to replace,
	     and update INSERTED to equal the number of bytes
	     we are taking from the decoded string.  */
	  inserted -= (ZV_BYTE! () - same_at_end) + (same_at_start - BEGV_BYTE! ());

	  if !NILP (visit) && BEG as ptrdiff_t == BEGV! () && Z! () == ZV! ()
	    /* See previous specbind for the reason behind this.  */
	    { specbind (Qbuffer_file_name (), Qnil ()); }
	  if same_at_end != same_at_start
	    {
	      del_range_byte (same_at_start, same_at_end);
	      temp = GPT! ();
	      eassert! (same_at_start == GPT_BYTE! ());
	      same_at_start = GPT_BYTE! ();
	    }
	  else
	    {
	      temp = same_at_end_charpos;
	    }
	  /* Insert from the file at the proper position.  */
	  SET_PT_BOTH (temp, same_at_start);
	  same_at_start_charpos
	    = buf_bytepos_to_charpos (XBUFFER (conversion_buffer) as *mut c_void,
				      same_at_start - BEGV_BYTE! ()
				      + BUF_BEG_BYTE (XBUFFER (conversion_buffer)));
	  eassert! (same_at_start_charpos == temp - (BEGV! () - BEG as ptrdiff_t));
	  inserted_chars
	    = (buf_bytepos_to_charpos (XBUFFER (conversion_buffer) as *mut c_void,
				       same_at_start + inserted - BEGV_BYTE! ()
				       + BUF_BEG_BYTE (XBUFFER (conversion_buffer)))
	       - same_at_start_charpos);
	  insert_from_buffer (XBUFFER (conversion_buffer),
			      same_at_start_charpos, inserted_chars, false);
	  /* Set `inserted' to the number of inserted characters.  */
	  inserted = PT! () - temp;
	  /* Set point before the inserted characters.  */
	  SET_PT_BOTH (temp, same_at_start);

	  unbind_to (this_count, Qnil ());

	  break 'notfound;                             /* goto handled  */
	}

      if beg_offset != curpos
	{
	  if !NILP (beg) || 0 <= curpos
	    { xlseek (fd, beg_offset, SEEK_SET, orig_filename); }
	  else if end_offset != TYPE_MAXIMUM! (off_t)
	    {
	      curpos = emacs_fd_lseek! (fd, 0, SEEK_CUR);
	      beg_offset = std::cmp::max (0, curpos);
	      end_offset = std::cmp::max (end_offset, beg_offset);
	    }
	}
      /* curpos effectively goes out of scope now, as it is no longer needed,
	 so do not bother to update curpos from now on.  */

      total = end_offset - beg_offset;

      if NILP (visit) && total > 0
	{
	  if !NILP (BVAR! ((current_buffer! ()), file_truename_))
	      /* Make binding buffer-file-name to nil effective.  */
	      && !NILP (BVAR! ((current_buffer! ()), filename_))
	      && SAVE_MODIFF! () >= MODIFF! ()
	    { we_locked_file = true; }
	  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());
	}

      /* If REPLACE is Qunbound, buffer contents are being replaced with
	 text read from a FIFO or a device.  Erase the entire accessible
	 portion of the buffer.  */

      if BASE_EQ (replace, Qunbound ())
	{ del_range (BEGV! (), ZV! ()); }

      move_gap_both (PT! (), PT_BYTE! ());

      /* Ensure the gap is at least one byte larger than needed for the
	 estimated insertion, so that in the usual case we read
	 without reallocating.  */
      let mut inserted_estimate: off_t = likely_end - beg_offset;
      if (GAP_SIZE! () as off_t) <= inserted_estimate
	{
	  let mut growth: ptrdiff_t = 0;
	  if { let (g, ovf) = inserted_estimate.overflowing_sub (GAP_SIZE! () as off_t - 1);
	       growth = g as ptrdiff_t; ovf }
	    { buffer_overflow (); }
	  make_gap (growth);
	}

      /* Total bytes inserted.  */
      inserted = 0;

      /* Here, we don't do code conversion in the loop.  It is done by
	 decode_coding_gap after all data are read into the buffer.  */
      {
	let mut gap_size: ptrdiff_t = GAP_SIZE! ();

	while (inserted as off_t) < total
	  {
	    let mut buf: *mut c_char;
	    let mut bufsize: ptrdiff_t;
	    let mut this: ptrdiff_t;
	    if (gap_size as off_t) < total - inserted as off_t
	      && gap_size < std::mem::size_of_val (&read_buf) as ptrdiff_t
	      {
		buf = read_buf.as_mut_ptr ();
		bufsize = std::mem::size_of_val (&read_buf) as ptrdiff_t;
	      }
	    else
	      {
		buf = (BEG_ADDR! ()).offset (PT_BYTE! () - BEG_BYTE as ptrdiff_t + inserted) as *mut c_char;
		bufsize = std::cmp::min (gap_size, INSERT_READ_SIZE_MAX as ptrdiff_t);
	      }
	    bufsize = std::cmp::min (bufsize, (total - inserted as off_t) as ptrdiff_t);

	    if !regular
	      {
		let mut nbytes: Lisp_Object;
		let mut number: intmax_t = 0;

		/* Read from the file, capturing `quit'.  When an
		   error occurs, end the loop, and arrange for a quit
		   to be signaled after decoding the text we read.
		   This way, we do not lose any data read.  */
		let mut data: read_non_regular
		  = read_non_regular { s: read_non_regular_s { fd: fd,
			buf: if buf == read_buf.as_mut_ptr () { buf } else { std::ptr::null_mut () },
			inserted: inserted, bufsize: bufsize } };
		nbytes = internal_condition_case_1
		  (Some (read_non_regular), make_pointer_integer (&raw mut data as *mut c_void),
		   Qerror (), Some (read_non_regular_quit));

		if NILP (nbytes)
		  {
		    read_quit = -1;
		    break;
		  }

		integer_to_intmax (nbytes, &raw mut number);
		this = number as ptrdiff_t;
	      }
	    else
	      /* Allow quitting out of the actual I/O.  We don't make text
		 part of the buffer until all the reading is done, so a
		 C-g here doesn't do any harm: though the data read are discarded,
		 the original data is still in the input file.  */
	      {
		this = emacs_fd_read! (fd, buf as *mut c_void, bufsize);
		if this < 0
		  { this = - (*errno ()) as ptrdiff_t; }
	      }

	    if this <= 0
	      {
		read_quit = -this as c_int;
		break;
	      }

	    if buf == read_buf.as_mut_ptr ()
	      {
		if gap_size < this
		  {
		    /* Size estimate was low.  Make the gap at least 50% larger,
		       and big enough so that the next loop iteration will
		       use the gap directly instead of copying via read_buf.  */
		    make_gap (std::cmp::max (GAP_SIZE! () >> 1,
					     this - gap_size
					     + std::mem::size_of_val (&read_buf) as ptrdiff_t));
		    gap_size = GAP_SIZE! () - inserted;
		  }
		memcpy ((BEG_ADDR! ()).offset (PT_BYTE! () - BEG_BYTE as ptrdiff_t + inserted) as *mut c_void,
			read_buf.as_mut_ptr () as *const c_void, this as usize);
	      }
	    gap_size -= this;
	    inserted += this;
	  }
      }

      /* Now we have either read all the file data into the gap,
	 or stop reading on I/O error or quit.  If nothing was
	 read, undo marking the buffer modified.  */

      if inserted == 0
	{
	  if we_locked_file
	    { Funlock_file (BVAR! ((current_buffer! ()), file_truename_)); }
	  Vdeactivate_mark = old_Vdeactivate_mark;
	}
      else
	{ Fset (Qdeactivate_mark (), Qt ()); }

      emacs_fd_close! (fd);
      clear_unwind_protect (fd_index);

      if 0 < read_quit
	{ report_file_errno (c"Read error".as_ptr (), orig_filename, read_quit); }
    }
    /* `notfound:`  */

    if NILP (coding_system)
      {
	/* The coding system is not yet decided.  Decide it by an
	   optimized method for handling `coding:' tag.

	   Note that we can get here only if the buffer was empty
	   before the insertion.  */
	eassert! (Z! () == BEG as ptrdiff_t);

	if !NILP (Vcoding_system_for_read)
	  { coding_system = Vcoding_system_for_read; }
	else
	  {
	    /* Since we are sure that the current buffer was empty
	       before the insertion, we can toggle
	       enable-multibyte-characters directly here without taking
	       care of marker adjustment.  By this way, we can run Lisp
	       program safely before decoding the inserted text.  */
	    let mut multibyte: Lisp_Object
	      = BVAR! ((current_buffer! ()), enable_multibyte_characters_);
	    let mut unwind_data: Lisp_Object
	      = Fcons (multibyte,
		       Fcons (BVAR! ((current_buffer! ()), undo_list_),
			      Fcurrent_buffer ()));
	    let count1: specpdl_ref = SPECPDL_INDEX ();

	    bset_enable_multibyte_characters ((current_buffer! ()), Qnil ());
	    bset_undo_list ((current_buffer! ()), Qt ());
	    record_unwind_protect (Some (decide_coding_unwind), unwind_data);

	    /* Make the text read part of the buffer.  */
	    insert_from_gap_1 (inserted, inserted, false);

	    if inserted > 0 && ! NILP (Vset_auto_coding_function)
	      {
		coding_system = calln! (Vset_auto_coding_function,
					filename, make_fixnum (inserted));
	      }

	    if NILP (coding_system)
	      {
		/* If the coding system is not yet decided, check
		   file-coding-system-alist.  */
		coding_system = CALLN! (Ffind_operation_coding_system,
					Qinsert_file_contents (), orig_filename,
					visit, beg, end, Qnil ());
		if CONSP (coding_system)
		  { coding_system = XCAR (coding_system); }
	      }
	    /* Move the text back to the gap.  */
	    unbind_to (count1, Qnil ());
	    inserted = XFIXNUM (XCAR (unwind_data));
	  }

	if NILP (coding_system)
	  { coding_system = Qundecided (); }
	else
	  { CHECK_CODING_SYSTEM (coding_system); }

	if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	  /* We must suppress all character code conversion except for
	     end-of-line conversion.  */
	  { coding_system = raw_text_coding_system (coding_system); }
	setup_coding_system (coding_system, &raw mut coding);
	/* Ensure we set Vlast_coding_system_used.  */
	set_coding_system = true;
      }

    if !NILP (visit)
      {
	/* When we visit a file by raw-text, we change the buffer to
	   unibyte.  */
	if CODING_FOR_UNIBYTE (&raw mut coding) != 0
	    /* Can't do this if part of the buffer might be preserved.  */
	    && NILP (replace)
	  {
	    /* Visiting a file with these coding system makes the buffer
	       unibyte.  */
	    if inserted > 0
	      { bset_enable_multibyte_characters ((current_buffer! ()), Qnil ()); }
	    else
	      { Fset_buffer_multibyte (Qnil ()); }
	  }
      }

    eassert! (PT! () == GPT! ());

    coding.bits
      = if !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
      { coding.bits | (1u32 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT) }
      else { coding.bits & !(1u32 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT) };
    if CODING_MAY_REQUIRE_DECODING (&raw mut coding) != 0
	&& (inserted > 0 || CODING_REQUIRE_FLUSHING (&raw mut coding) != 0)
      {
	/* Now we have all the new bytes at the beginning of the gap,
           but `decode_coding_gap` can't have them at the beginning of the gap,
           so we need to move them.  */
	memmove ((GAP_END_ADDR! ()).offset (-inserted) as *mut c_void,
		 GPT_ADDR! () as *const c_void, inserted as usize);
	decode_coding_gap (&raw mut coding, inserted);
	inserted = coding.produced_char;
	coding_system = CODING_ID_NAME (coding.id as hash_idx_t);
      }
    else if inserted > 0
      {
	/* Make the text read part of the buffer.  */
	eassert! (NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));
	insert_from_gap_1 (inserted, inserted, false);

	invalidate_buffer_caches ((current_buffer! ()) as *mut c_void, PT! (), PT! () + inserted);
	adjust_after_insert (PT! (), PT! () + 0, PT! () + inserted, PT_BYTE! () + inserted,
			     inserted);
      }

    /* Call after-change hooks for the inserted text, aside from the case
       of normal visiting (not with REPLACE), which is done in a new buffer
       "before" the buffer is changed.  */
    if inserted > 0 && total > 0
	&& (NILP (visit) || !NILP (replace))
      {
	signal_after_change (PT! (), 0, inserted);
	update_compositions (PT! (), PT! (), CHECK_BORDER);
      }

    /* Now INSERTED is measured in characters.  */

  }   /* end of `notfound:`  */

  /* `handled:`  */

  if inserted > 0
    { restore_window_points (window_markers, inserted,
			     BYTE_TO_CHAR (same_at_start),
			     same_at_end_charpos); }

  if !NILP (visit)
    {
      if empty_undo_list_p
	{ bset_undo_list ((current_buffer! ()), Qnil ()); }

      if NILP (handler)
	{
	  (*(current_buffer! ())).modtime = mtime;
	  (*(current_buffer! ())).modtime_size = st_size;
	  bset_filename ((current_buffer! ()), orig_filename);
	}

      SAVE_MODIFF! () = MODIFF! ();
      BUF_AUTOSAVE_MODIFF! ((current_buffer! ())) = MODIFF! ();
      XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), Z! () - BEG as ptrdiff_t);
      if NILP (handler)
	{
	  if !NILP (BVAR! ((current_buffer! ()), file_truename_))
	    { Funlock_file (BVAR! ((current_buffer! ()), file_truename_)); }
	  Funlock_file (filename);
	}
    }

  if set_coding_system
    { Vlast_coding_system_used = coding_system; }

  if ! NILP (Ffboundp (Qafter_insert_file_set_coding ()))
    {
      insval = calln! (Qafter_insert_file_set_coding (), make_fixnum (inserted),
		       visit);
      if ! NILP (insval)
	{
	  if ! RANGED_FIXNUMP (0, insval, ZV! () - PT! ())
	    { wrong_type_argument (Qinserted_chars (), insval); }
	  inserted = XFIXNAT (insval);
	}
    }

  /* Decode file format.  Don't do this if Qformat_decode is not
     bound, which can happen when called early during loadup.  */

  if inserted > 0 && !NILP (Ffboundp (Qformat_decode ()))
    {
      /* Don't run point motion or modification hooks when decoding.  */
      let count1: specpdl_ref = SPECPDL_INDEX ();
      let mut old_inserted: ptrdiff_t = inserted;
      specbind (Qinhibit_point_motion_hooks (), Qt ());
      specbind (Qinhibit_modification_hooks (), Qt ());

      /* Save old undo list and don't record undo for decoding.  */
      old_undo = BVAR! ((current_buffer! ()), undo_list_);
      bset_undo_list ((current_buffer! ()), Qt ());

      if NILP (replace)
	{
	  insval = calln! (Qformat_decode (),
			   Qnil (), make_fixnum (inserted), visit);
	  if ! RANGED_FIXNUMP (0, insval, ZV! () - PT! ())
	    { wrong_type_argument (Qinserted_chars (), insval); }
	  inserted = XFIXNAT (insval);
	}
      else
	{
	  /* If REPLACE is non-nil and we succeeded in not replacing the
	     beginning or end of the buffer text with the file's contents,
	     call format-decode with `point' positioned at the beginning
	     of the buffer and `inserted' equaling the number of
	     characters in the buffer.  Otherwise, format-decode might
	     fail to correctly analyze the beginning or end of the buffer.
	     Hence we temporarily save `point' and `inserted' here and
	     restore `point' iff format-decode did not insert or delete
	     any text.  Otherwise we leave `point' at point-min.  */
	  let mut opoint: ptrdiff_t = PT! ();
	  let mut opoint_byte: ptrdiff_t = PT_BYTE! ();
	  let mut oinserted: ptrdiff_t = ZV! () - BEGV! ();
	  let mut ochars_modiff: modiff_count = CHARS_MODIFF! ();

	  TEMP_SET_PT_BOTH (BEGV! (), BEGV_BYTE! ());
	  insval = calln! (Qformat_decode (),
			   Qnil (), make_fixnum (oinserted), visit);
	  if ! RANGED_FIXNUMP (0, insval, ZV! () - PT! ())
	    { wrong_type_argument (Qinserted_chars (), insval); }
	  if ochars_modiff == CHARS_MODIFF! ()
	    /* format_decode didn't modify buffer's characters => move
	       point back to position before inserted text and leave
	       value of inserted alone.  */
	    { SET_PT_BOTH (opoint, opoint_byte); }
	  else
	    /* format_decode modified buffer's characters => consider
	       entire buffer changed and leave point at point-min.  */
	    { inserted = XFIXNAT (insval); }
	}

      /* For consistency with format-decode call these now iff inserted > 0
	 (martin 2007-06-28).  */
      p = Vafter_insert_file_functions;
      FOR_EACH_TAIL! (p, {
	  if NILP (replace)
	    {
	      insval = calln! (XCAR (p), make_fixnum (inserted));
	      if !NILP (insval)
		{
		  if ! RANGED_FIXNUMP (0, insval, ZV! () - PT! ())
		    { wrong_type_argument (Qinserted_chars (), insval); }
		  inserted = XFIXNAT (insval);
		}
	    }
	  else
	    {
	      /* For the rationale of this see the comment on
		 format-decode above.  */
	      let mut opoint: ptrdiff_t = PT! ();
	      let mut opoint_byte: ptrdiff_t = PT_BYTE! ();
	      let mut oinserted: ptrdiff_t = ZV! () - BEGV! ();
	      let mut ochars_modiff: modiff_count = CHARS_MODIFF! ();

	      TEMP_SET_PT_BOTH (BEGV! (), BEGV_BYTE! ());
	      insval = calln! (XCAR (p), make_fixnum (oinserted));
	      if !NILP (insval)
		{
		  if ! RANGED_FIXNUMP (0, insval, ZV! () - PT! ())
		    { wrong_type_argument (Qinserted_chars (), insval); }
		  if ochars_modiff == CHARS_MODIFF! ()
		    /* after_insert_file_functions didn't modify
		       buffer's characters => move point back to
		       position before inserted text and leave value of
		       inserted alone.  */
		    { SET_PT_BOTH (opoint, opoint_byte); }
		  else
		    /* after_insert_file_functions did modify buffer's
	               characters => consider entire buffer changed and
	               leave point at point-min.  */
		    { inserted = XFIXNAT (insval); }
		}
	    }
	});

      if !empty_undo_list_p
	{
	  bset_undo_list ((current_buffer! ()), old_undo);
	  if CONSP (old_undo) && inserted != old_inserted
	    {
	      /* Adjust the last undo record for the size change during
		 the format conversion.  */
	      let mut tem: Lisp_Object = XCAR (old_undo);
	      if CONSP (tem) && FIXNUMP (XCAR (tem))
		  && FIXNUMP (XCDR (tem))
		  && XFIXNUM (XCDR (tem)) == PT! () + old_inserted
		{ XSETCDR (tem, make_fixnum (PT! () + inserted)); }
	    }
	}
      else
	/* If undo_list was Qt before, keep it that way.
	   Otherwise start with an empty undo_list.  */
	{ bset_undo_list ((current_buffer! ()), if EQ (old_undo, Qt ()) { Qt () } else { Qnil () }); }

      unbind_to (count1, Qnil ());
    }

  if save_errno != 0
    {
      /* Signal an error if visiting a file that could not be opened.  */
      eassert! (!NILP (visit) && NILP (handler));
      report_file_errno (c"Opening input file".as_ptr (), orig_filename, save_errno);
    }

  /* We made a lot of deletions and insertions above, so invalidate
     the newline cache for the entire region of the inserted
     characters.  */
  if !(*(current_buffer! ())).base_buffer.is_null ()
      && !(*(*(current_buffer! ())).base_buffer).newline_cache.is_null ()
    { invalidate_region_cache ((*(current_buffer! ())).base_buffer,
			       (*(*(current_buffer! ())).base_buffer).newline_cache,
			       PT! () - BEG as ptrdiff_t, Z! () - PT! () - inserted); }
  else if !(*(current_buffer! ())).newline_cache.is_null ()
    { invalidate_region_cache ((current_buffer! ()),
			       (*(current_buffer! ())).newline_cache,
			       PT! () - BEG as ptrdiff_t, Z! () - PT! () - inserted); }

  if read_quit != 0
    { quit (); }

  /* Retval needs to be dealt with in all cases consistently.  */
  if NILP (val)
    { val = list2 (orig_filename, make_fixnum (inserted)); }

  return unbind_to (count, val);
    }   /* end of the `'handled` loop  */
});

/* `validate_region` (buffer.h 1260) and `coding_system_require_warning` (coding.c's own
   DEFVAR_BOOL carrier) both come from the included mirrors, so they are not declared here
   (PORTING.md P-26 / the mirror-dupes gate).  */

/* Decide the coding-system to encode the data with.  */

unsafe fn choose_write_coding_system (start: Lisp_Object, end: Lisp_Object,
					     filename: Lisp_Object, append: Lisp_Object,
					     visit: Lisp_Object, lockname: Lisp_Object,
					     coding: *mut coding_system) -> Lisp_Object
{
  let mut val: Lisp_Object;
  let mut eol_parent: Lisp_Object = Qnil ();

  if auto_saving
      && NILP (Fstring_equal (BVAR! ((current_buffer! ()), filename_),
			      BVAR! ((current_buffer! ()), auto_save_file_name_)))
    {
      val = Qutf_8_emacs ();
      eol_parent = Qunix ();
    }
  else if !NILP (Vcoding_system_for_write)
    {
      val = Vcoding_system_for_write;
      if coding_system_require_warning
	  && !NILP (Ffboundp (Vselect_safe_coding_system_function))
	/* Confirm that VAL can surely encode the current region.  */
	{ val = calln! (Vselect_safe_coding_system_function,
			start, end, list2 (Qt (), val),
			Qnil (), filename); }
    }
  else
    {
      /* If the variable `buffer-file-coding-system' is set locally,
	 it means that the file was read with some kind of code
	 conversion or the variable is explicitly set by users.  We
	 had better write it out with the same coding system even if
	 `enable-multibyte-characters' is nil.

	 If it is not set locally, we anyway have to convert EOL
	 format if the default value of `buffer-file-coding-system'
	 tells that it is not Unix-like (LF only) format.  */
      let mut using_default_coding: bool = false;
      let mut force_raw_text: bool = false;

      val = BVAR! ((current_buffer! ()), buffer_file_coding_system_);
      if NILP (val)
	  || NILP (Flocal_variable_p (Qbuffer_file_coding_system (), Qnil ()))
	{
	  val = Qnil ();
	  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	    { force_raw_text = true; }
	}

      if NILP (val)
	{
	  /* Check file-coding-system-alist.  */
	  let mut coding_systems: Lisp_Object
	    = CALLN! (Ffind_operation_coding_system, Qwrite_region (), start, end,
		      filename, append, visit, lockname);
	  if CONSP (coding_systems) && !NILP (XCDR (coding_systems))
	    { val = XCDR (coding_systems); }
	}

      if NILP (val)
	{
	  /* If we still have not decided a coding system, use the
	     current buffer's value of buffer-file-coding-system.  */
	  val = BVAR! ((current_buffer! ()), buffer_file_coding_system_);
	  using_default_coding = true;
	}

      if ! NILP (val) && ! force_raw_text
	{
	  let mut spec: Lisp_Object;
	  let mut attrs: Lisp_Object;

	  CHECK_CODING_SYSTEM (val);
	  spec = CHECK_CODING_SYSTEM_GET_SPEC (val);
	  attrs = AREF (spec, 0);
	  if EQ (CODING_ATTR_TYPE (attrs), Qraw_text ())
	    { force_raw_text = true; }
	}

      if !force_raw_text
	  && !NILP (Ffboundp (Vselect_safe_coding_system_function))
	{
	  /* Confirm that VAL can surely encode the current region.  */
	  val = calln! (Vselect_safe_coding_system_function,
			start, end, val, Qnil (), filename);
	  /* As the function specified by select-safe-coding-system-function
	     is out of our control, make sure we are not fed by bogus
	     values.  */
	  if !NILP (val)
	    { CHECK_CODING_SYSTEM (val); }
	}

      /* If the decided coding-system doesn't specify end-of-line
	 format, we use that of `buffer-file-coding-system'.  */
      if ! using_default_coding
	{
	  let mut dflt: Lisp_Object = BVAR! (&raw mut buffer_defaults, buffer_file_coding_system_);

	  if ! NILP (dflt)
	    { val = coding_inherit_eol_type (val, dflt); }
	}

      /* If we decide not to encode text, use `raw-text' or one of its
	 subsidiaries.  */
      if force_raw_text
	{ val = raw_text_coding_system (val); }
    }

  val = coding_inherit_eol_type (val, eol_parent);
  setup_coding_system (val, coding);

  if !STRINGP (start) && EQ (Qt (), BVAR! ((current_buffer! ()), selective_display_))
    { (*coding).bits |= (CODING_MODE_SELECTIVE_DISPLAY as u32) << CODING_SYSTEM_MODE_SHIFT; }
  return val;
}

DEFUN! ("write-region", Fwrite_region, Swrite_region, 3, 7,
       "r\nFWrite region to file: \ni\ni\ni\np",
       doc: "Write current region into specified file.\nWhen called from a program, requires three arguments:\nSTART, END and FILENAME.  START and END are normally buffer positions\nspecifying the part of the buffer to write.\nIf START is nil, that means to use the entire buffer contents; END is\nignored.\nIf START is a string, then output that string to the file\ninstead of any buffer contents; END is ignored.\n\nOptional fourth argument APPEND if non-nil means\n  append to existing file contents (if any).  If it is a number,\n  seek to that offset in the file before writing.\nOptional fifth argument VISIT, if t or a string, means\n  set the last-save-file-modtime of buffer to this file's modtime\n  and mark buffer not modified.\nIf VISIT is t, the buffer is marked as visiting FILENAME.\nIf VISIT is a string, it is a second file name;\n  the output goes to FILENAME, but the buffer is marked as visiting VISIT.\n  VISIT is also the file name to lock and unlock for clash detection.\nIf VISIT is neither t nor nil nor a string, or if Emacs is in batch mode,\n  do not display the \"Wrote file\" message.\nThe optional sixth arg LOCKNAME, if non-nil, specifies the name to\n  use for locking and unlocking, overriding FILENAME and VISIT.\nThe optional seventh arg MUSTBENEW, if non-nil, insists on a check\n  for an existing file with the same name.  If MUSTBENEW is `excl',\n  that means to get an error if the file already exists; never overwrite.\n  If MUSTBENEW is neither nil nor `excl', that means ask for\n  confirmation before overwriting, but do go ahead and overwrite the file\n  if the user confirms.\n\nThis does code conversion according to the value of\n`coding-system-for-write', `buffer-file-coding-system', or\n`file-coding-system-alist', and sets the variable\n`last-coding-system-used' to the coding system actually used.\n\nThis calls `write-region-annotate-functions' at the start, and\n`write-region-post-annotation-function' at the end.",
       (start: Lisp_Object, end: Lisp_Object, filename: Lisp_Object, append: Lisp_Object,
        visit: Lisp_Object, lockname: Lisp_Object, mustbenew: Lisp_Object),
       {
  return write_region (start, end, filename, append, visit, lockname, mustbenew,
		       -1);
});

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

unsafe extern "C" {
  /// C: lisp.h 4924: `extern Lisp_Object save_restriction_save (void);`
  pub fn save_restriction_save () -> Lisp_Object;
  /// C: lisp.h 4925: `extern void save_restriction_restore (Lisp_Object);`
  pub fn save_restriction_restore (data: Lisp_Object);
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);`
  pub fn FUNCTIONP (a: Lisp_Object) -> bool;
  /// C: lisp.h 4397: `extern void message_with_string (const char *, Lisp_Object, bool);`
  pub fn message_with_string (m: *const c_char, string: Lisp_Object, log: bool);
  /// C: editfns.c's own definition (lisp.h 4939's `labeled_restrictions_*` family).
  pub fn labeled_restrictions_remove_in_current_buffer ();
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);` - eval.rs defines it as `-> !`,
  /// so this declaration copies that definition (no return type difference).
  pub fn memory_full (size: usize) -> !;
}

/* Like Fwrite_region, except that if DESC is nonnegative, it is a file
   descriptor for FILENAME, so do not open or close FILENAME.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_region (start: Lisp_Object, end: Lisp_Object,
				       filename: Lisp_Object, append: Lisp_Object,
				       visit: Lisp_Object, lockname: Lisp_Object,
				       mustbenew: Lisp_Object, mut desc: c_int) -> Lisp_Object
{
  let mut open_flags: c_int;
  let mut mode: c_int;
  let mut offset: off_t = 0;                  /* C: `off_t offset UNINIT;`  */
  let mut open_and_close_file: bool = desc < 0;
  let mut ok: bool;
  let mut save_errno: c_int = 0;
  let mut fn_: *const c_char;
  let mut st: stat = std::mem::zeroed ();
  let mut modtime: timespec;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut count1: specpdl_ref = std::mem::zeroed ();  /* C: `specpdl_ref count1 UNINIT;`  */
  let mut handler: Lisp_Object;
  let mut visit_file: Lisp_Object;
  let mut annotations: Lisp_Object;
  let mut encoded_filename: Lisp_Object;
  let mut visiting: bool = EQ (visit, Qt ()) || STRINGP (visit);
  let mut quietly: bool = !NILP (visit);
  let mut file_locked: bool = false;
  let mut given_buffer: *mut buffer;
  let mut coding: coding_system = std::mem::zeroed ();
  let mut filename = filename;                /* C reassigns `filename` below.  */
  let mut lockname = lockname;                /* C reassigns `lockname` below.  */
  let mut start = start;                      /* C's `XSETFASTINT (start, …)` rebinds it.  */
  let mut end = end;                          /* C's `XSETFASTINT (end, …)` rebinds it.  */

  if !(*(current_buffer! ())).base_buffer.is_null () && visiting
    { error (c"Cannot do file visiting in an indirect buffer".as_ptr ()); }

  if !NILP (start) && !STRINGP (start)
    { validate_region (&raw mut start, &raw mut end); }

  visit_file = Qnil ();

  filename = Fexpand_file_name (filename, Qnil ());

  if !NILP (mustbenew) && !EQ (mustbenew, Qexcl ())
    { barf_or_query_if_file_exists (filename, false, c"overwrite".as_ptr (), true, true); }

  if STRINGP (visit)
    { visit_file = Fexpand_file_name (visit, Qnil ()); }
  else
    { visit_file = filename; }

  if NILP (lockname)
    { lockname = visit_file; }

  annotations = Qnil ();

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qwrite_region ());
  /* If FILENAME has no handler, see if VISIT has one.  */
  if NILP (handler) && STRINGP (visit)
    { handler = Ffind_file_name_handler (visit, Qwrite_region ()); }

  if !NILP (handler)
    {
      let mut val: Lisp_Object;
      val = calln! (handler, Qwrite_region (), start, end,
		    filename, append, visit, lockname, mustbenew);

      if visiting
	{
	  SAVE_MODIFF! () = MODIFF! ();
	  XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), Z! () - BEG as ptrdiff_t);
	  bset_filename ((current_buffer! ()), visit_file);
	}

      return val;
    }

  record_unwind_protect (Some (save_restriction_restore), save_restriction_save ());
  labeled_restrictions_remove_in_current_buffer ();

  /* Special kludge to simplify auto-saving.  */
  if NILP (start)
    {
      /* Do it later, so write-region-annotate-function can work differently
	 if we save "the buffer" vs "a region".
	 This is useful in tar-mode.  --Stef
      XSETFASTINT (start, BEG);
      XSETFASTINT (end, Z); */
      Fwiden ();
    }

  record_unwind_protect (Some (build_annotations_unwind),
			 Vwrite_region_annotation_buffers);
  Vwrite_region_annotation_buffers = list1 (Fcurrent_buffer ());

  given_buffer = (current_buffer! ());

  if !STRINGP (start)
    {
      annotations = build_annotations (start, end);

      if (current_buffer! ()) != given_buffer
	{
	  XSETFASTINT! (start, BEGV! ());
	  XSETFASTINT! (end, ZV! ());
	}
    }

  if NILP (start)
    {
      XSETFASTINT! (start, BEGV! ());
      XSETFASTINT! (end, ZV! ());
    }

  /* Decide the coding-system to encode the data with.
     We used to make this choice before calling build_annotations, but that
     leads to problems when a write-annotate-function takes care of
     unsavable chars (as was the case with X-Symbol).  */
  Vlast_coding_system_used
    = choose_write_coding_system (start, end, filename,
				  append, visit, lockname, &raw mut coding);

  if open_and_close_file && !auto_saving
    {
      Flock_file (lockname);
      file_locked = true;
    }

  encoded_filename = ENCODE_FILE (filename);

  fn_ = SSDATA (encoded_filename);
  open_flags = O_WRONLY | O_CREAT;
  open_flags |= if EQ (mustbenew, Qexcl ()) { O_EXCL }
  else if !NILP (append) { 0 } else { O_TRUNC };
  if NUMBERP (append)
    { offset = file_offset (append); }
  else if !NILP (append)
    { open_flags |= O_APPEND; }
  /* C 5631-5635: `#ifdef DOS_NT` (undefined) / `#else`: the live arm is below.  */
  mode = if auto_saving { auto_save_mode_bits as c_int } else { 0o666 };

  if open_and_close_file
    {
      desc = emacs_open (fn_, open_flags, mode);
      if desc < 0
	{
	  let mut open_errno: c_int = *errno ();
	  if file_locked
	    { Funlock_file (lockname); }
	  report_file_errno (c"Opening output file".as_ptr (), filename, open_errno);
	}

      count1 = SPECPDL_INDEX ();
      record_unwind_protect_int (Some (close_file_unwind), desc);
    }

  if NUMBERP (append)
    {
      let mut ret: off_t = lseek (desc, offset, SEEK_SET);
      if ret < 0
	{
	  let mut lseek_errno: c_int = *errno ();
	  if file_locked
	    { Funlock_file (lockname); }
	  report_file_errno (c"Setting file position".as_ptr (), filename, lseek_errno);
	}
    }

  if STRINGP (start)
    { ok = a_write (desc, start, 0, SCHARS (start), &raw mut annotations, &raw mut coding); }
  else if XFIXNUM (start) != XFIXNUM (end)
    { ok = a_write (desc, Qnil (), XFIXNUM (start), XFIXNUM (end) - XFIXNUM (start),
		    &raw mut annotations, &raw mut coding); }
  else
    {
      /* If file was empty, still need to write the annotations.  */
      coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;
      ok = a_write (desc, Qnil (), XFIXNUM (end), 0, &raw mut annotations, &raw mut coding);
    }
  save_errno = *errno ();

  if ok && CODING_REQUIRE_FLUSHING (&raw mut coding) != 0
      && (coding.bits & ((CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT)) == 0
    {
      /* We have to flush out a data. */
      coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT;
      ok = e_write (desc, Qnil (), 1, 1, &raw mut coding);
      save_errno = *errno ();
    }

  /* fsync is not crucial for temporary files.  Nor for auto-save
     files, since they might lose some work anyway.  */
  if open_and_close_file && !auto_saving && !write_region_inhibit_fsync
    {
      /* Transfer data and metadata to disk, retrying if interrupted.
	 fsync can report a write failure here, e.g., due to disk full
	 under NFS.  But ignore EINVAL (and EBADF on Windows), which
	 means fsync is not supported on this file.  */
      while fsync (desc) != 0
	{
	  if *errno () != EINTR
	    {
	      if *errno () != EINVAL
		/* C 5684-5686: `#ifdef WINDOWSNT` (undefined): recorded.  */
		{ ok = false; save_errno = *errno (); }
	      break;
	    }
	}
    }

  modtime = invalid_timespec ();
  if visiting
    {
      if sys_fstat (desc, &raw mut st) == 0
	{ modtime = get_stat_mtime (&raw mut st); }
      else
	{ ok = false; save_errno = *errno (); }
    }

  if open_and_close_file
    {
      /* NFS can report a write failure now.  */
      if emacs_close (desc) < 0
	{ ok = false; save_errno = *errno (); }

      /* Discard the unwind protect for close_file_unwind.  */
      (specpdl_ptr! ()) = specpdl_ref_to_ptr (count1);
    }

  /* Some file systems have a bug where st_mtime is not updated
     properly after a write.  For example, CIFS might not see the
     st_mtime change until after the file is opened again.

     Attempt to detect this file system bug, and update MODTIME to the
     newer st_mtime if the bug appears to be present.  This introduces
     a race condition, so to avoid most instances of the race condition
     on non-buggy file systems, skip this check if the most recently
     encountered non-buggy file system was the current file system.

     A race condition can occur if some other process modifies the
     file between the fstat above and the fstat below, but the race is
     unlikely and a similar race between the last write and the fstat
     above cannot possibly be closed anyway.  */

  if timespec_valid_p (modtime)
      && ! (valid_timestamp_file_system && st.st_dev == timestamp_file_system)
    {
      let mut st1: stat = std::mem::zeroed ();

      /* The code below previously tried to open FN O_WRONLY,
         subsequently calling fstat on the opened file descriptor.
         This proved inefficient and resulted in FN being truncated
         under several Android filesystems, and as such has been
         changed to a call to `stat'.  */

      if emacs_fstatat (AT_FDCWD, fn_, &raw mut st1 as *mut c_void, 0) == 0
	  && st.st_dev == st1.st_dev
	  && (st.st_ino == st1.st_ino
	      /* C 5767-5773: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`
		 (HAVE_ANDROID undef): recorded.  */
	      )
	{
	  /* Use the heuristic if it appears to be valid.  With neither
	     O_EXCL nor O_TRUNC, if the program happened to write nothing to the
	     file, the time stamp won't change.  Also, some non-POSIX
	     systems don't update an empty file's time stamp when
	     truncating it.  Finally, file systems with 100 ns or worse
	     resolution sometimes seem to have bugs: on a system with ns
	     resolution, checking ns % 100 incorrectly avoids the heuristic
	     1% of the time, but the problem should be temporary as we will
	     try again on the next time stamp.  */
	  let mut use_heuristic: bool
	    = (open_flags & (O_EXCL | O_TRUNC)) != 0
	       && st.st_size != 0
	       && modtime.tv_nsec % 100 != 0;

	  let mut modtime1: timespec = get_stat_mtime (&raw mut st1);
	  if use_heuristic
	      && timespec_cmp (modtime, modtime1) == 0
	      && st.st_size == st1.st_size
	    {
	      timestamp_file_system = st.st_dev;
	      valid_timestamp_file_system = true;
	    }
	  else
	    {
	      st.st_size = st1.st_size;
	      modtime = modtime1;
	    }
	}
    }

  /* Call write-region-post-annotation-function. */
  while CONSP (Vwrite_region_annotation_buffers)
    {
      let mut buf: Lisp_Object = XCAR (Vwrite_region_annotation_buffers);
      if !NILP (Fbuffer_live_p (buf))
	{
	  Fset_buffer (buf);
	  if FUNCTIONP (Vwrite_region_post_annotation_function)
	    { call0 (Vwrite_region_post_annotation_function); }
	}
      Vwrite_region_annotation_buffers
	= XCDR (Vwrite_region_annotation_buffers);
    }

  unbind_to (count, Qnil ());

  if file_locked
    { Funlock_file (lockname); }

  /* Do this before reporting IO error
     to avoid a "file has changed on disk" warning on
     next attempt to save.  */
  if timespec_valid_p (modtime)
    {
      (*(current_buffer! ())).modtime = modtime;
      (*(current_buffer! ())).modtime_size = st.st_size;
    }

  if ! ok
    { report_file_errno (c"Write error".as_ptr (), filename, save_errno); }

  let mut auto_saving_into_visited_file: bool
    = auto_saving
    && ! NILP (Fstring_equal (BVAR! ((current_buffer! ()), filename_),
			      BVAR! ((current_buffer! ()), auto_save_file_name_)));
  if visiting
    {
      SAVE_MODIFF! () = MODIFF! ();
      XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), Z! () - BEG as ptrdiff_t);
      bset_filename ((current_buffer! ()), visit_file);
      update_mode_lines = 14;
      if auto_saving_into_visited_file
	{ Funlock_file (lockname); }
    }
  else if quietly
    {
      if auto_saving_into_visited_file
	{
	  SAVE_MODIFF! () = MODIFF! ();
	  Funlock_file (lockname);
	}

      return Qnil ();
    }

  if !auto_saving && !noninteractive
    { message_with_string (if NUMBERP (append)
			   { c"Updated %s".as_ptr () }
			   else if ! NILP (append)
			   { c"Added to %s".as_ptr () }
			   else { c"Wrote %s".as_ptr () },
			   visit_file, true); }

  return Qnil ();
}

DEFUN! ("car-less-than-car", Fcar_less_than_car, Scar_less_than_car, 2, 2, 0,
       doc: "Return t if (car A) is numerically less than (car B).",
       (a: Lisp_Object, b: Lisp_Object),
       {
  let mut ca: Lisp_Object = Fcar (a);
  let mut cb: Lisp_Object = Fcar (b);
  if FIXNUMP (ca) && FIXNUMP (cb)
    { return if XFIXNUM (ca) < XFIXNUM (cb) { Qt () } else { Qnil () }; }
  return if (arithcompare (ca, cb) & Cmp_LT as cmp_bits_t) != 0 { Qt () } else { Qnil () };
});

unsafe extern "C" fn build_annotations_unwind (arg: Lisp_Object)
{
  Vwrite_region_annotation_buffers = arg;
}

/* Build the complete list of annotations appropriate for writing out
   the text between START and END, by calling all the functions in
   write-region-annotate-functions and merging the lists they return.
   If one of these functions switches to a different buffer, we assume
   that buffer contains altered text.  Therefore, the caller must
   make sure to restore the current buffer in all cases,
   as save-excursion would do.  */

unsafe fn build_annotations (start: Lisp_Object, end: Lisp_Object) -> Lisp_Object
{
  let mut annotations: Lisp_Object;
  let mut p: Lisp_Object;
  let mut res: Lisp_Object;
  let mut original_buffer: Lisp_Object;
  let mut used_global: bool = false;
  let mut start = start;                      /* C's `XSETFASTINT (start, …)` rebinds it.  */
  let mut end = end;                          /* C's `XSETFASTINT (end, …)` rebinds it.  */

  XSETBUFFER! (original_buffer, (current_buffer! ()));

  annotations = Qnil ();
  p = Vwrite_region_annotate_functions;
  'loop_over_p: loop
    {
      /* C's `FOR_EACH_TAIL (p)` is written out here, because the `goto loop_over_p` below
         re-enters it with a new P (the ITREE_FOREACH "expand at the use site" shape,
         PORTING.md 3.7).  */
      let mut li: for_each_tail_internal
	= for_each_tail_internal { tortoise: p, l: 0 };
      let mut goto_loop_over_p: bool = false;
      while CONSP (p) || { likely (NILP (p)); false }
	{
	  let mut given_buffer: *mut buffer = (current_buffer! ());
	  if EQ (Qt (), XCAR (p)) && !used_global
	    { /* Use the global value of the hook.  */
	      used_global = true;
	      p = CALLN! (Fappend,
			  Fdefault_value (Qwrite_region_annotate_functions ()),
			  XCDR (p));
	      goto_loop_over_p = true;
	      break;
	    }
	  Vwrite_region_annotations_so_far = annotations;
	  res = calln! (XCAR (p), start, end);
	  /* If the function makes a different buffer current,
	     assume that means this buffer contains altered text to be output.
	     Reset START and END from the buffer bounds
	     and discard all previous annotations because they should have
	     been dealt with by this function.  */
	  if (current_buffer! ()) != given_buffer
	    {
	      Vwrite_region_annotation_buffers
		= Fcons (Fcurrent_buffer (),
			 Vwrite_region_annotation_buffers);
	      XSETFASTINT! (start, BEGV! ());
	      XSETFASTINT! (end, ZV! ());
	      annotations = Qnil ();
	    }
	  Flength (res);   /* Check basic validity of return value */
	  annotations = merge (annotations, res, Qcar_less_than_car ());

	  /* Stepper (lisp.h 5909-5912) with CHECK_QUIT true.  */
	  p = XCDR (p);
	  if BASE_EQ (p, li.tortoise)
	    { circular_list (p); }
	  else if ! likely (({ li.l += 1; li.l } & (FOR_EACH_TAIL_THRESHOLD - 1)) != 0)
	    {
	      maybe_quit ();
	      if POWER_OF_2 (li.l)
		{ li.tortoise = p; }
	    }
	}
      if !goto_loop_over_p
	{ break 'loop_over_p; }
    }

  /* Now do the same for annotation functions implied by the file-format */
  if auto_saving && (!EQ (BVAR! ((current_buffer! ()), auto_save_file_format_), Qt ()))
    { p = BVAR! ((current_buffer! ()), auto_save_file_format_); }
  else
    { p = BVAR! ((current_buffer! ()), file_format_); }
  let mut i: EMACS_INT = 0;
  FOR_EACH_TAIL! (p, {
      let mut given_buffer: *mut buffer = (current_buffer! ());

      Vwrite_region_annotations_so_far = annotations;

      /* Value is either a list of annotations or nil if the function
         has written annotations to a temporary buffer, which is now
         current.  */
      res = calln! (Qformat_annotate_function (), XCAR (p), start, end,
		    original_buffer, make_fixnum (i));
      i += 1;
      if (current_buffer! ()) != given_buffer
	{
	  XSETFASTINT! (start, BEGV! ());
	  XSETFASTINT! (end, ZV! ());
	  annotations = Qnil ();
	}

      if CONSP (res)
	{ annotations = merge (annotations, res, Qcar_less_than_car ()); }
    });

  return annotations;
}

/* ---- further use-site declarations (PORTING.md 0.1-2/P-72 rule 3) ---------------------- */

unsafe extern "C" {
  /// C: lisp.h 4383: `extern bool push_message (void);`
  pub fn push_message () -> bool;
  /// C: lisp.h 4384: `extern void pop_message_unwind (void);`
  pub fn pop_message_unwind ();
  /// C: lisp.h 4386: `extern void restore_message (void);`
  pub fn restore_message ();
  /// C: lisp.h 4373: `extern Lisp_Object sit_for (Lisp_Object, bool, int);`
  pub fn sit_for (value: Lisp_Object, reading: bool, display: c_int) -> Lisp_Object;
  /// C: lisp.h 5097: `extern void safe_run_hooks (Lisp_Object);`
  pub fn safe_run_hooks (hook: Lisp_Object);
  /// C: lisp.h 5103: `extern void record_auto_save (void);`
  pub fn record_auto_save ();
  /// C: lisp.h 4805: `extern Lisp_Object internal_condition_case (Lisp_Object (*) (void),
  /// Lisp_Object, Lisp_Object (*) (Lisp_Object));`
  pub fn internal_condition_case (bfun: Option<unsafe extern "C" fn () -> Lisp_Object>,
				  handlers: Lisp_Object,
				  hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  /// C: lisp.h 5322: `extern FILE *emacs_fopen (char const *, char const *);`
  pub fn emacs_fopen (file: *const c_char, mode: *const c_char) -> *mut FILE;
  /// C: dispextern.h 3933: `extern void ring_bell (struct frame *);`
  pub fn ring_bell (f: *mut frame);
}

/* Write to descriptor DESC the NCHARS chars starting at POS of STRING.
   If STRING is nil, POS is the character position in the current buffer.
   Intersperse with them the annotations from *ANNOT
   which fall within the range of POS to POS + NCHARS,
   each at its appropriate position.

   We modify *ANNOT by discarding elements as we use them up.

   Return true if successful.  */

unsafe fn a_write (desc: c_int, string: Lisp_Object, mut pos: ptrdiff_t,
			  nchars: ptrdiff_t, annot: *mut Lisp_Object,
			  coding: *mut coding_system) -> bool
{
  let mut tem: Lisp_Object;
  let mut nextpos: ptrdiff_t;
  let mut lastpos: ptrdiff_t = pos + nchars;

  while NILP (*annot) || CONSP (*annot)
    {
      tem = Fcar_safe (Fcar (*annot));
      nextpos = pos - 1;
      if FIXNUMP (tem)
	{ nextpos = XFIXNUM (tem); }

      /* If there are no more annotations in this range,
	 output the rest of the range all at once.  */
      if ! (nextpos >= pos && nextpos <= lastpos)
	{ return e_write (desc, string, pos, lastpos, coding); }

      /* Output buffer text up to the next annotation's position.  */
      if nextpos > pos
	{
	  if !e_write (desc, string, pos, nextpos, coding)
	    { return false; }
	  pos = nextpos;
	}
      /* Output the annotation.  */
      tem = Fcdr (Fcar (*annot));
      if STRINGP (tem)
	{
	  if !e_write (desc, tem, 0, SCHARS (tem), coding)
	    { return false; }
	}
      *annot = Fcdr (*annot);
    }
  return true;
}

/* Maximum number of characters that the next
   function encodes per one loop iteration.  */

/* C 5952: `enum { E_WRITE_MAX = 8 * 1024 * 1024 };`  */
const E_WRITE_MAX: ptrdiff_t = 8 * 1024 * 1024;

/* Write text in the range START and END into descriptor DESC,
   encoding them with coding system CODING.  If STRING is nil, START
   and END are character positions of the current buffer, else they
   are indexes to the string STRING.  Return true if successful.  */

unsafe fn e_write (desc: c_int, string: Lisp_Object, mut start: ptrdiff_t,
			  mut end: ptrdiff_t, coding: *mut coding_system) -> bool
{
  if STRINGP (string)
    {
      start = 0;
      end = SCHARS (string);
    }

  /* We used to have a code for handling selective display here.  But,
     now it is handled within encode_coding.  */

  while start < end
    {
      if STRINGP (string)
	{
	  (*coding).bits = if SCHARS (string) < SBYTES (string) { (*coding).bits | (1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) } else { (*coding).bits & !(1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) };
	  if CODING_REQUIRE_ENCODING (coding) != 0
	    {
	      let mut nchars: ptrdiff_t = std::cmp::min (end - start, E_WRITE_MAX);

	      /* Avoid creating huge Lisp string in encode_coding_object.  */
	      if nchars == E_WRITE_MAX
		{ (*coding).bits |= 1u32 << CODING_SYSTEM_RAW_DESTINATION_SHIFT; }

	      encode_coding_object
		(coding, string, start, string_char_to_byte (string, start),
		 start + nchars, string_char_to_byte (string, start + nchars),
		 Qt ());
	    }
	  else
	    {
	      (*coding).dst_object = string;
	      (*coding).consumed_char = SCHARS (string);
	      (*coding).produced = SBYTES (string);
	    }
	}
      else
	{
	  let mut start_byte: ptrdiff_t = CHAR_TO_BYTE (start);
	  let mut end_byte: ptrdiff_t = CHAR_TO_BYTE (end);

	  (*coding).bits = if (end - start) < (end_byte - start_byte) { (*coding).bits | (1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) } else { (*coding).bits & !(1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) };
	  if CODING_REQUIRE_ENCODING (coding) != 0
	    {
	      let mut nchars: ptrdiff_t = std::cmp::min (end - start, E_WRITE_MAX);

	      /* Likewise.  */
	      if nchars == E_WRITE_MAX
		{ (*coding).bits |= 1u32 << CODING_SYSTEM_RAW_DESTINATION_SHIFT; }

	      encode_coding_object
		(coding, Fcurrent_buffer (), start, start_byte,
		 start + nchars, CHAR_TO_BYTE (start + nchars), Qt ());
	    }
	  else
	    {
	      (*coding).dst_object = Qnil ();
	      (*coding).dst_pos_byte = start_byte;
	      if start >= GPT! () || end <= GPT! ()
		{
		  (*coding).consumed_char = end - start;
		  (*coding).produced = end_byte - start_byte;
		}
	      else
		{
		  (*coding).consumed_char = GPT! () - start;
		  (*coding).produced = GPT_BYTE! () - start_byte;
		}
	    }
	}

      if (*coding).produced > 0
	{
	  let mut buf: *mut c_char
	    = if ((*coding).bits >> CODING_SYSTEM_RAW_DESTINATION_SHIFT) & 1 != 0 { (*coding).destination as *mut c_char }
	  else if STRINGP ((*coding).dst_object) { SSDATA ((*coding).dst_object) }
	  else { BYTE_POS_ADDR ((*coding).dst_pos_byte) as *mut c_char };
	  (*coding).produced -= emacs_write_quit (desc, buf as *const c_void, (*coding).produced);

	  if ((*coding).bits >> CODING_SYSTEM_RAW_DESTINATION_SHIFT) & 1 != 0
	    {
	      /* We're responsible for freeing this, see
		 encode_coding_object to check why.  */
	      xfree ((*coding).destination as *mut c_void);
	      (*coding).bits &= !(1u32 << CODING_SYSTEM_RAW_DESTINATION_SHIFT);
	    }
	  if (*coding).produced != 0
	    { return false; }
	}
      start += (*coding).consumed_char;
    }

  return true;
}

DEFUN! ("verify-visited-file-modtime", Fverify_visited_file_modtime,
       Sverify_visited_file_modtime, 0, 1, 0,
       doc: "Return t if last mod time of BUF's visited file matches what BUF records.\nThis means that the file has not been changed since it was visited or saved.\nIf BUF is omitted or nil, it defaults to the current buffer.\nSee Info node `(elisp)Modification Time' for more details.",
       (buf: Lisp_Object),
       {
  let mut b: *mut buffer = decode_buffer (buf);
  let mut st: stat = std::mem::zeroed ();
  let mut handler: Lisp_Object;
  let mut filename: Lisp_Object;
  let mut mtime: timespec = std::mem::zeroed ();  /* C leaves it uninitialised (PORTING.md 1.5) */

  if !STRINGP (BVAR! (b, filename_)) { return Qt (); }
  if (*b).modtime.tv_nsec == UNKNOWN_MODTIME_NSECS as c_long { return Qt (); }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (BVAR! (b, filename_),
				     Qverify_visited_file_modtime ());
  if !NILP (handler)
    { return calln! (handler, Qverify_visited_file_modtime (), buf); }

  filename = ENCODE_FILE (BVAR! (b, filename_));
  mtime = if emacs_fstatat (AT_FDCWD, SSDATA (filename), &raw mut st as *mut c_void, 0) == 0
    { get_stat_mtime (&raw mut st) }
    else { time_error_value (*errno ()) };
  if timespec_cmp (mtime, (*b).modtime) == 0
      && ((*b).modtime_size < 0
	  || st.st_size == (*b).modtime_size)
    { return Qt (); }
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn buffer_visited_file_modtime (buf: *mut buffer) -> Lisp_Object
{
  let mut ns: c_int = (*buf).modtime.tv_nsec as c_int;
  if ns < 0
    { return make_fixnum ((UNKNOWN_MODTIME_NSECS - ns) as isize); }
  return make_lisp_time ((*buf).modtime);
}

DEFUN! ("visited-file-modtime", Fvisited_file_modtime,
       Svisited_file_modtime, 0, 0, 0,
       doc: "Return the current buffer's recorded visited file modification time.\nReturn a Lisp timestamp (as in `current-time') if the current buffer\nhas a recorded file modification time, 0 if it doesn't, and -1 if the\nvisited file doesn't exist.\nSee Info node `(elisp)Modification Time' for more details.",
       (),
       {
  return buffer_visited_file_modtime ((current_buffer! ()));
});

DEFUN! ("set-visited-file-modtime", Fset_visited_file_modtime,
       Sset_visited_file_modtime, 0, 1, 0,
       doc: "Update buffer's recorded modification time from the visited file's time.\nUseful if the buffer was not read from the file normally\nor if the file itself has been changed for some known benign reason.\nAn argument specifies the modification time value to use\n(instead of that of the visited file), in the form of a time value as\nin `current-time' or an integer flag as returned by `visited-file-modtime'.",
       (time_flag: Lisp_Object),
       {
  if !NILP (time_flag)
    {
      let mut mtime: timespec = std::mem::zeroed ();  /* C leaves it uninitialised (PORTING.md 1.5) */
      if FIXNUMP (time_flag)
	{
	  let mut flag: c_int = check_integer_range (time_flag, -1, 0) as c_int;
	  mtime = make_timespec (0, (UNKNOWN_MODTIME_NSECS - flag) as c_long);
	}
      else
	{ mtime = lisp_time_argument (time_flag); }

      (*(current_buffer! ())).modtime = mtime;
      (*(current_buffer! ())).modtime_size = -1;
    }
  else if !(*(current_buffer! ())).base_buffer.is_null ()
    { error (c"An indirect buffer does not have a visited file".as_ptr ()); }
  else
    {
      let mut filename: Lisp_Object;
      let mut encoded: Lisp_Object;
      let mut st: stat = std::mem::zeroed ();
      let mut handler: Lisp_Object;

      filename = Fexpand_file_name (BVAR! ((current_buffer! ()), filename_), Qnil ());

      /* If the file name has special constructs in it,
	 call the corresponding file name handler.  */
      handler = Ffind_file_name_handler (filename, Qset_visited_file_modtime ());
      if !NILP (handler)
	/* The handler can find the file name the same way we did.  */
	{ return calln! (handler, Qset_visited_file_modtime (), Qnil ()); }

      encoded = ENCODE_FILE (filename);

      if emacs_fstatat (AT_FDCWD, SSDATA (encoded), &raw mut st as *mut c_void, 0)
	  == 0
	{
	  (*(current_buffer! ())).modtime = get_stat_mtime (&raw mut st);
	  (*(current_buffer! ())).modtime_size = st.st_size;
	}
      else
	{ file_attribute_errno (filename, *errno ()); }
    }

  return Qnil ();
});

unsafe extern "C" fn auto_save_error (error_val: Lisp_Object) -> Lisp_Object
{
  auto_save_error_occurred = true;

  ring_bell (XFRAME (selected_frame));

  AUTO_STRING! (format, c"Auto-saving %s: %s".as_ptr ());
  let mut msg: Lisp_Object = CALLN! (Fformat, format, BVAR! ((current_buffer! ()), name_),
				     Ferror_message_string (error_val));
  calln! (Qdisplay_warning (), Qauto_save (), msg, QCerror ());

  return Qnil ();
}

unsafe extern "C" fn auto_save_1 () -> Lisp_Object
{
  let mut st: stat = std::mem::zeroed ();
  let mut modes: Lisp_Object;

  auto_save_mode_bits = 0o666;

  /* Get visited file's mode to become the auto save file's mode.  */
  if ! NILP (BVAR! ((current_buffer! ()), filename_))
    {
      if emacs_fstatat (AT_FDCWD, SSDATA (BVAR! ((current_buffer! ()), filename_)),
			&raw mut st as *mut c_void, 0)
	  == 0
	/* But make sure we can overwrite it later!  */
	{ auto_save_mode_bits = (st.st_mode | 0o600) & 0o777; }
      else if { modes = Ffile_modes (BVAR! ((current_buffer! ()), filename_), Qnil ());
		FIXNUMP (modes) }
	/* Remote files don't cooperate with fstatat.  */
	{ auto_save_mode_bits = (XFIXNUM (modes) as mode_t | 0o600) & 0o777; }
    }

  return
    Fwrite_region (Qnil (), Qnil (), BVAR! ((current_buffer! ()), auto_save_file_name_), Qnil (),
		   if NILP (Vauto_save_visited_file_name) { Qlambda () } else { Qt () },
		   Qnil (), Qnil ());
}

/// C 6276-6280: `struct auto_save_unwind { FILE *stream; bool auto_raise; };` - clang measures
/// it on the frozen target: `sizeof` 16, `alignof` 8, `stream`@0, `auto_raise`@8.
#[repr(C)]
pub struct auto_save_unwind
{
  pub stream: *mut FILE,
  pub auto_raise: bool,
}
const _: () = core::assert! (std::mem::size_of::<auto_save_unwind> () == 16);
const _: () = core::assert! (std::mem::align_of::<auto_save_unwind> () == 8);
const _: () = core::assert! (std::mem::offset_of! (auto_save_unwind, stream) == 0);
const _: () = core::assert! (std::mem::offset_of! (auto_save_unwind, auto_raise) == 8);

unsafe extern "C" fn do_auto_save_unwind (arg: *mut c_void)
{
  let mut p: *mut auto_save_unwind = arg as *mut auto_save_unwind;
  let mut stream: *mut FILE = (*p).stream;
  minibuffer_auto_raise = (*p).auto_raise;
  auto_saving = false;
  if !stream.is_null ()
    {
      block_input ();
      emacs_fclose! (stream);
      unblock_input ();
    }
}

unsafe extern "C" fn do_auto_save_make_dir (dir: Lisp_Object) -> Lisp_Object
{
  let mut result: Lisp_Object;

  auto_saving_dir_umask = 0o77;
  result = calln! (Qmake_directory (), dir, Qt ());
  auto_saving_dir_umask = 0;
  return result;
}

unsafe extern "C" fn do_auto_save_eh (ignore: Lisp_Object) -> Lisp_Object
{
  auto_saving_dir_umask = 0;
  return Qnil ();
}

DEFUN! ("do-auto-save", Fdo_auto_save, Sdo_auto_save, 0, 2, "",
       doc: "Auto-save all buffers that need it.\nThis auto-saves all buffers that have auto-saving enabled and\nwere changed since last auto-saved.\n\nAuto-saving writes the buffer into a file so that your edits are\nnot lost if the system crashes.\n\nThe auto-save file is not the file you visited; that changes only\nwhen you save.\n\nNormally, run the normal hook `auto-save-hook' before saving.\n\nA non-nil NO-MESSAGE argument means do not print any message if successful.\n\nA non-nil CURRENT-ONLY argument means save only current buffer.",
       (no_message: Lisp_Object, current_only: Lisp_Object),
       {
  let mut old: *mut buffer = (current_buffer! ());
  let mut b: *mut buffer;
  let mut tail: Lisp_Object;
  let mut buf: Lisp_Object;
  let mut hook: Lisp_Object;
  let mut auto_saved: bool = false;
  let mut do_handled_files: c_int;
  let mut oquit: Lisp_Object;
  let mut stream: *mut FILE = std::ptr::null_mut ();
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut orig_minibuffer_auto_raise: bool = minibuffer_auto_raise;
  let mut old_message_p: bool = false;
  let mut auto_save_unwind: auto_save_unwind = std::mem::zeroed ();
  let mut no_message = no_message;            /* C reassigns `no_message` below.  */

  if minibuf_level != 0
    { no_message = Qt (); }

  if NILP (no_message)
    {
      old_message_p = push_message ();
      record_unwind_protect_void (Some (pop_message_unwind));
    }

  /* Ordinarily don't quit within this function,
     but don't make it impossible to quit (in case we get hung in I/O).  */
  oquit = Vquit_flag;
  Vquit_flag = Qnil ();

  hook = Qauto_save_hook ();
  safe_run_hooks (hook);

  if STRINGP (Vauto_save_list_file_name)
    {
      let mut listfile: Lisp_Object;

      listfile = Fexpand_file_name (Vauto_save_list_file_name, Qnil ());

      /* Don't try to create the directory when shutting down the program,
         because creating the directory might signal an error, and
         that would leave the program in a strange state.  */
      if !NILP (Vrun_hooks)
	{
	  let mut dir: Lisp_Object;
	  dir = file_name_directory (listfile);
	  if NILP (Ffile_directory_p (dir))
	    { internal_condition_case_1 (Some (do_auto_save_make_dir),
					 dir, Qt (),
					 Some (do_auto_save_eh)); }
	}

      stream = emacs_fopen (SSDATA (listfile), c"w".as_ptr ());
    }

  auto_save_unwind.stream = stream;
  auto_save_unwind.auto_raise = minibuffer_auto_raise;
  record_unwind_protect_ptr (Some (do_auto_save_unwind),
			     &raw mut auto_save_unwind as *mut c_void);
  minibuffer_auto_raise = false;
  auto_saving = true;
  auto_save_error_occurred = false;

  /* On first pass, save all files that don't have handlers.
     On second pass, save all files that do have handlers.

     If the program is crashing, the handlers may tweak what is causing
     the program to crash in the first place, and it would be a shame if
     the program failed to autosave perfectly ordinary files because it
     couldn't handle some ange-ftp'd file.  */

  /* C: `for (do_handled_files = 0; do_handled_files < 2; do_handled_files++)
     FOR_EACH_LIVE_BUFFER (tail, buf) {...}`; the loop macro has no Rust counterpart (P-22,
     docs/ports/excluded-macros.txt), so its expansion is written here (the buffer.rs
     unlock_all_files shape) and C's two `continue`s become the enclosing `if`s, because the
     loop's increment runs after the body on every path.  */
  do_handled_files = 0;
  while do_handled_files < 2
    {
      tail = Vbuffer_alist;
      while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
	{
	  b = XBUFFER (buf);

	  /* Record all the buffers that have auto save mode
	     in the special file that lists them.  For each of these buffers,
	     Record visited name (if any) and auto save name.  */
	  if STRINGP (BVAR! (b, auto_save_file_name_))
	      && !stream.is_null () && do_handled_files == 0
	    {
	      block_input ();
	      if !NILP (BVAR! (b, filename_))
		{ fwrite (SDATA (BVAR! (b, filename_)) as *const c_void, 1,
			  SBYTES (BVAR! (b, filename_)) as usize, stream); }
	      putc (b'\n' as c_int, stream);
	      fwrite (SDATA (BVAR! (b, auto_save_file_name_)) as *const c_void, 1,
		      SBYTES (BVAR! (b, auto_save_file_name_)) as usize, stream);
	      putc (b'\n' as c_int, stream);
	      unblock_input ();
	    }

	  if !(!NILP (current_only) && b != (current_buffer! ()))
	      /* Don't auto-save indirect buffers.
		 The base buffer takes care of it.  */
	      && (*b).base_buffer.is_null ()
	    {
	      /* Check for auto save enabled
		 and file changed since last auto save
		 and file changed since last real save.  */
	      if STRINGP (BVAR! (b, auto_save_file_name_))
		  && BUF_SAVE_MODIFF! (b) < BUF_MODIFF! (b)
		  && BUF_AUTOSAVE_MODIFF! (b) < BUF_MODIFF! (b)
		  /* -1 means we've turned off autosaving for a while--see below.  */
		  && FIXNUMP (BVAR! (b, save_length_))
		  && XFIXNUM (BVAR! (b, save_length_)) >= 0
		  && (do_handled_files != 0
		      || NILP (Ffind_file_name_handler (BVAR! (b, auto_save_file_name_),
							Qwrite_region ())))
		{
		  let mut before_time: timespec = current_timespec ();
		  let mut after_time: timespec;

		  /* If we had a failure, don't try again for 20 minutes.  */
		  if !((*b).auto_save_failure_time > 0
		       && before_time.tv_sec - (*b).auto_save_failure_time < 1200)
		    {
		      /* C 6427: `enum { growth_factor = 4 };`
			 C 6428: `static_assert (BUF_BYTES_MAX <= EMACS_INT_MAX / growth_factor);`  */
		      const growth_factor: c_int = 4;
		      static_assert! (BUF_BYTES_MAX <= EMACS_INT_MAX / growth_factor as isize);

		      set_buffer_internal (b);
		      if NILP (Vauto_save_include_big_deletions)
			  || !FIXNUMP (BVAR! (b, save_length_))
			  /* A short file is likely to change a large fraction;
			     spare the user annoying messages.  */
			  || XFIXNUM (BVAR! (b, save_length_)) <= 5000
			  || !(growth_factor as isize * (BUF_Z! (b) - BUF_BEG (b))
			       < (growth_factor as isize - 1) * XFIXNUM (BVAR! (b, save_length_)))
			  /* These messages are frequent and annoying for `*mail*'.  */
			  || NILP (BVAR! (b, filename_))
			  || !NILP (no_message)
			{
			  if !auto_saved && NILP (no_message)
			    { message1 (c"Auto-saving...".as_ptr ()); }
			  internal_condition_case (Some (auto_save_1), Qt (),
						   Some (auto_save_error));
			  auto_saved = true;
			  BUF_AUTOSAVE_MODIFF! (b) = BUF_MODIFF! (b);
			  XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), Z! () - BEG as ptrdiff_t);
			  set_buffer_internal (old);
			}
		      else
			{
			  /* It has shrunk too much; turn off auto-saving here.  */
			  minibuffer_auto_raise = orig_minibuffer_auto_raise;
			  message_with_string (c"Buffer %s has shrunk a lot; auto save disabled in that buffer until next real save".as_ptr (),
					       BVAR! (b, name_), true);
			  minibuffer_auto_raise = false;
			  /* Turn off auto-saving until there's a real save,
			     and prevent any more warnings.  */
			  XSETINT! (BVAR! (b, save_length_), -1);
			  Fsleep_for (make_fixnum (1), Qnil ());
			}

		      after_time = current_timespec ();

		      /* If auto-save took more than 60 seconds,
			 assume it was an NFS failure that got a timeout.  */
		      if after_time.tv_sec - before_time.tv_sec > 60
			{ (*b).auto_save_failure_time = after_time.tv_sec; }
		    }
		}
	    }
	  tail = XCDR (tail);
	}
      do_handled_files += 1;
    }

  /* Prevent another auto save till enough input events come in.  */
  record_auto_save ();

  if auto_saved && NILP (no_message)
    {
      if old_message_p
	{
	  /* If we are going to restore an old message,
	     give time to read ours.  */
	  sit_for (make_fixnum (1), false, 0);
	  restore_message ();
	}
      else if !auto_save_error_occurred
	/* Don't overwrite the error message if an error occurred.
	   If we displayed a message and then restored a state
	   with no message, leave a "done" message on the screen.  */
	{ message1 (c"Auto-saving...done".as_ptr ()); }
    }

  Vquit_flag = oquit;

  /* This restores the message-stack status.  */
  return unbind_to (count, Qnil ());
});

DEFUN! ("set-buffer-auto-saved", Fset_buffer_auto_saved,
       Sset_buffer_auto_saved, 0, 0, 0,
       doc: "Mark current buffer as auto-saved with its current text.\nNo auto-save file will be written until the buffer changes again.",
       (),
       {
  /* FIXME: This should not be called in indirect buffers, since
     they're not autosaved.  */
  BUF_AUTOSAVE_MODIFF! ((current_buffer! ())) = MODIFF! ();
  XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), Z! () - BEG as ptrdiff_t);
  (*(current_buffer! ())).auto_save_failure_time = 0;
  return Qnil ();
});

DEFUN! ("clear-buffer-auto-save-failure", Fclear_buffer_auto_save_failure,
       Sclear_buffer_auto_save_failure, 0, 0, 0,
       doc: "Clear any record of a recent auto-save failure in the current buffer.",
       (),
       {
  (*(current_buffer! ())).auto_save_failure_time = 0;
  return Qnil ();
});

DEFUN! ("recent-auto-save-p", Frecent_auto_save_p, Srecent_auto_save_p,
       0, 0, 0,
       doc: "Return t if current buffer has been auto-saved recently.\nMore precisely, if it has been auto-saved since last read from or saved\nin the visited file.  If the buffer has no visited file,\nthen any auto-save counts as \"recent\".",
       (),
       {
  /* FIXME: maybe we should return nil for indirect buffers since
     they're never autosaved.  */
  return if SAVE_MODIFF! () < BUF_AUTOSAVE_MODIFF! ((current_buffer! ())) { Qt () } else { Qnil () };
});

/* C's `<stdio.h>` `stdin`/`stdout`/`stderr` are `#define`s for `__stdinp`/`__stdoutp`/
   `__stderrp` on this target, and the `stdio.rs` mirror now carries those three objects with
   the `link_name` spelling (its `stdio_objs` module, glob-imported), so this unit uses them
   directly: a declaration named `stdin` would emit a reference nothing can satisfy
   (`check-undef.py` reported exactly that while this unit carried one - the `emacs_fclose`
   shape, status 3.68).  */

DEFUN! ("next-read-file-uses-dialog-p", Fnext_read_file_uses_dialog_p,
       Snext_read_file_uses_dialog_p, 0, 0, 0,
       doc: "Return t if a call to `read-file-name' will use a dialog.\nThe return value is only relevant for a call to `read-file-name' that happens\nbefore any other event (mouse or keypress) is handled.",
       (),
       {
  /* C 6565-6573: `#if (defined USE_GTK || defined USE_MOTIF || defined HAVE_NS ||
     defined HAVE_NTGUI || defined HAVE_HAIKU)` (all undefined): recorded.  */
  return Qnil ();
});

unsafe fn file_for_stream (stream: Lisp_Object) -> *mut FILE
{
  if EQ (stream, Qstdin ())
    { return stdin; }
  else if EQ (stream, Qstdout ())
    { return stdout; }
  else if EQ (stream, Qstderr ())
    { return stderr; }
  else
    { xsignal2 (Qerror (), build_string (c"unsupported stream".as_ptr ()), stream); }
}

DEFUN! ("set-binary-mode", Fset_binary_mode, Sset_binary_mode, 2, 2, 0,
       doc: "Switch STREAM to binary I/O mode or text I/O mode.\nSTREAM can be one of the symbols `stdin', `stdout', or `stderr'.\nIf MODE is non-nil, switch STREAM to binary mode, otherwise switch\nit to text mode.\n\nAs a side effect, this function flushes any pending STREAM's data.\n\nValue is the previous value of STREAM's I/O mode, nil for text mode,\nnon-nil for binary mode.\n\nOn MS-Windows and MS-DOS, binary mode is needed to read or write\narbitrary binary data, and for disabling translation between CR-LF\npairs and a single newline character.  Examples include generation\nof text files with Unix-style end-of-line format using `princ' in\nbatch mode, with standard output redirected to a file.\n\nOn Posix systems, this function always returns non-nil, and has no\neffect except for flushing STREAM's data.",
       (stream: Lisp_Object, mode: Lisp_Object),
       {
  CHECK_SYMBOL (stream);
  let mut fp: *mut FILE = file_for_stream (stream);
  let mut binmode: c_int;

  binmode = if NILP (mode) { O_TEXT } else { O_BINARY };
  if fp != stdin
    { fflush (fp); }

  return if set_binary_mode (fileno (fp), binmode) == O_BINARY { Qt () } else { Qnil () };
});

DEFUN! ("file--close-stream", Ffile__close_stream,
       Sfile__close_stream, 1, 1, 0,
       doc: "Close the standard STREAM of the Emacs process.\nSTREAM can be one of the symbols `stdin', `stdout', or `stderr'.\n\nThis function is primarily intended for testing process machinery within\nEmacs.",
       (stream: Lisp_Object),
       {
  CHECK_SYMBOL (stream);
  let mut fp: *mut FILE = file_for_stream (stream);
  fclose (fp);
  return Qnil ();
});

/* C 6655: `#ifndef DOS_NT` (undefined, so the text is live).  */

/* C 6657-6664: `#if defined STAT_STATFS2_BSIZE || … || defined STAT_STATVFS64`: the frozen
   target defines STAT_STATFS2_BSIZE (1, macros.tsv 3594), so the text is live.  */

/* Yield a Lisp number equal to BLOCKSIZE * BLOCKS, with the result
   negated if NEGATE.  */
unsafe fn blocks_to_bytes (blocksize: u64, blocks: u64, negate: bool) -> Lisp_Object
{
  let mut n: intmax_t = 0;
  if { let (nv, ovf) = blocksize.overflowing_mul (blocks); n = nv as intmax_t; !ovf }
    { return make_int (if negate { -n } else { n }); }
  let mut bs: Lisp_Object = make_uint (blocksize);
  if negate
    { bs = CALLN! (Fminus, bs); }
  return CALLN! (Ftimes, bs, make_uint (blocks));
}

DEFUN! ("file-system-info", Ffile_system_info, Sfile_system_info, 1, 1, 0,
       doc: "Return storage information about the file system FILENAME is on.\nValue is a list of numbers (TOTAL FREE AVAIL), where TOTAL is the total\nstorage of the file system, FREE is the free storage, and AVAIL is the\nstorage available to a non-superuser.  All 3 numbers are in bytes.\nIf the underlying system call fails, value is nil.",
       (filename: Lisp_Object),
       {
  let mut filename = filename;                /* C reassigns `filename` below.  */
  filename = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let mut handler: Lisp_Object = Ffind_file_name_handler (filename, Qfile_system_info ());
  if !NILP (handler)
    {
      let mut result: Lisp_Object = calln! (handler, Qfile_system_info (), filename);
      if CONSP (result) || NILP (result)
	{ return result; }
      error (c"Invalid handler in `file-name-handler-alist'".as_ptr ());
    }

  /* Try to detect whether or not fsusage.o is actually built.  */
  /* C 6690-6696: `#if defined STAT_STATFS2_BSIZE || … || defined STAT_STATVFS64`: live
     (STAT_STATFS2_BSIZE is 1).  */
  let mut u: fs_usage = std::mem::zeroed ();
  let mut name: *const c_char;

  name = SSDATA (ENCODE_FILE (filename));

  /* C 6703-6708: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (HAVE_ANDROID
     undef): recorded.  */

  if get_fs_usage (name, std::ptr::null (), &raw mut u) != 0
    { return if *errno () == ENOSYS { Qnil () }
	else { file_attribute_errno (filename, *errno ()) }; }
  return list3 (blocks_to_bytes (u.fsu_blocksize, u.fsu_blocks, false),
		blocks_to_bytes (u.fsu_blocksize, u.fsu_bfree, false),
		blocks_to_bytes (u.fsu_blocksize, u.fsu_bavail,
				 u.fsu_bavail_top_bit_set));
});

/* C 6725: the `#endif` that closes `#ifndef DOS_NT`.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_fileio ()
{
  realmask = umask (0);
  umask (realmask);

  valid_timestamp_file_system = false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_fileio ()
{
  /* Property name of a file name handler,
     which gives a list of operations it handles.  */
  DEFSYM! (Qoperations, c"operations");
  DEFSYM! (Qexpand_file_name, c"expand-file-name");
  DEFSYM! (Qsubstitute_in_file_name, c"substitute-in-file-name");
  DEFSYM! (Qdirectory_file_name, c"directory-file-name");
  DEFSYM! (Qfile_name_directory, c"file-name-directory");
  DEFSYM! (Qfile_name_nondirectory, c"file-name-nondirectory");
  DEFSYM! (Qunhandled_file_name_directory, c"unhandled-file-name-directory");
  DEFSYM! (Qfile_name_as_directory, c"file-name-as-directory");
  DEFSYM! (Qcopy_file, c"copy-file");
  DEFSYM! (Qmake_directory_internal, c"make-directory-internal");
  DEFSYM! (Qmake_directory, c"make-directory");
  DEFSYM! (Qdelete_file_internal, c"delete-file-internal");
  DEFSYM! (Qfile_name_case_insensitive_p, c"file-name-case-insensitive-p");
  DEFSYM! (Qrename_file, c"rename-file");
  DEFSYM! (Qadd_name_to_file, c"add-name-to-file");
  DEFSYM! (Qmake_symbolic_link, c"make-symbolic-link");
  DEFSYM! (Qfile_exists_p, c"file-exists-p");
  DEFSYM! (Qfile_executable_p, c"file-executable-p");
  DEFSYM! (Qfile_readable_p, c"file-readable-p");
  DEFSYM! (Qfile_writable_p, c"file-writable-p");
  DEFSYM! (Qfile_symlink_p, c"file-symlink-p");
  DEFSYM! (Qaccess_file, c"access-file");
  DEFSYM! (Qfile_directory_p, c"file-directory-p");
  DEFSYM! (Qfile_regular_p, c"file-regular-p");
  DEFSYM! (Qfile_accessible_directory_p, c"file-accessible-directory-p");
  DEFSYM! (Qfile_modes, c"file-modes");
  DEFSYM! (Qset_file_modes, c"set-file-modes");
  DEFSYM! (Qset_file_times, c"set-file-times");
  DEFSYM! (Qfile_selinux_context, c"file-selinux-context");
  DEFSYM! (Qset_file_selinux_context, c"set-file-selinux-context");
  DEFSYM! (Qfile_acl, c"file-acl");
  DEFSYM! (Qset_file_acl, c"set-file-acl");
  DEFSYM! (Qfile_newer_than_file_p, c"file-newer-than-file-p");
  DEFSYM! (Qinsert_file_contents, c"insert-file-contents");
  DEFSYM! (Qwrite_region, c"write-region");
  DEFSYM! (Qverify_visited_file_modtime, c"verify-visited-file-modtime");
  DEFSYM! (Qset_visited_file_modtime, c"set-visited-file-modtime");
  DEFSYM! (Qfile_system_info, c"file-system-info");
  /* The symbol bound to coding-system-for-read when insert-file-contents is
     called for recovering a file: it is not an actual coding system name, but an
     indicator telling insert-file-contents to use `emacs-mule` with a special flag
     for auto saving and recovering a file.  */
  DEFSYM! (Qauto_save_coding, c"auto-save-coding");
  DEFSYM! (Qfile_name_history, c"file-name-history");
  Fset (Qfile_name_history (), Qnil ());
  DEFSYM! (Qfile_error, c"file-error");
  DEFSYM! (Qfile_already_exists, c"file-already-exists");
  DEFSYM! (Qfile_date_error, c"file-date-error");
  DEFSYM! (Qfile_missing, c"file-missing");
  DEFSYM! (Qpermission_denied, c"permission-denied");
  DEFSYM! (Qfile_offset, c"file-offset");
  DEFSYM! (Qfile_notify_error, c"file-notify-error");
  DEFSYM! (Qremote_file_error, c"remote-file-error");
  DEFSYM! (Qexcl, c"excl");
  DEFSYM! (Qinserted_chars, c"inserted-chars");
  DEFVAR_LISP! (c"file-name-coding-system", Vfile_name_coding_system,
	       "Coding system for encoding file names.\nIf it is nil, `default-file-name-coding-system' (which see) is used.\n\nOn MS-Windows, the value of this variable is largely ignored if\n`w32-unicode-filenames' (which see) is non-nil.  Emacs on Windows\nbehaves as if file names were encoded in `utf-8'.");
  Vfile_name_coding_system = Qnil ();

  Vfile_name_coding_system = Qnil ();
  DEFVAR_LISP! (c"default-file-name-coding-system", Vdefault_file_name_coding_system,
	       "Default coding system for encoding file names.\nThis variable is used only when `file-name-coding-system' is nil.\n\nThis variable is set/changed by the command `set-language-environment'.\nUser should not set this variable manually,\ninstead use `file-name-coding-system' to get a constant encoding\nof file names regardless of the current language environment.\n\nOn MS-Windows, the value of this variable is largely ignored if\n`w32-unicode-filenames' (which see) is non-nil.  Emacs on Windows\nbehaves as if file names were encoded in `utf-8'.");
  Vdefault_file_name_coding_system = Qnil ();

  Vdefault_file_name_coding_system = Qnil ();
  /* Lisp functions for translating file formats.  */
  DEFSYM! (Qformat_decode, c"format-decode");
  DEFSYM! (Qformat_annotate_function, c"format-annotate-function");
  /* Lisp function for setting buffer-file-coding-system and the
  multibyteness of the current buffer after inserting a file.  */
  DEFSYM! (Qafter_insert_file_set_coding, c"after-insert-file-set-coding");
  DEFSYM! (Qcar_less_than_car, c"car-less-than-car");
  Fput (Qfile_error (), Qerror_conditions (),
	list2 (Qfile_error (), Qerror ()));
  Fput (Qfile_error (), Qerror_message (),
	build_string (c"File error".as_ptr ()));
  Fput (Qfile_already_exists (), Qerror_conditions (),
	list3 (Qfile_already_exists (), Qfile_error (), Qerror ()));
  Fput (Qfile_already_exists (), Qerror_message (),
	build_string (c"File already exists".as_ptr ()));
  Fput (Qfile_date_error (), Qerror_conditions (),
	list3 (Qfile_date_error (), Qfile_error (), Qerror ()));
  Fput (Qfile_date_error (), Qerror_message (),
	build_string (c"Cannot set file date".as_ptr ()));
  Fput (Qfile_missing (), Qerror_conditions (),
	list3 (Qfile_missing (), Qfile_error (), Qerror ()));
  Fput (Qfile_missing (), Qerror_message (),
	build_string (c"File is missing".as_ptr ()));
  Fput (Qpermission_denied (), Qerror_conditions (),
	list3 (Qpermission_denied (), Qfile_error (), Qerror ()));
  Fput (Qpermission_denied (), Qerror_message (),
	build_string (c"Cannot access file or directory".as_ptr ()));
  Fput (Qfile_notify_error (), Qerror_conditions (),
	list3 (Qfile_notify_error (), Qfile_error (), Qerror ()));
  Fput (Qfile_notify_error (), Qerror_message (),
	build_string (c"File notification error".as_ptr ()));
  Fput (Qremote_file_error (), Qerror_conditions (),
	list3 (Qremote_file_error (), Qfile_error (), Qerror ()));
  Fput (Qremote_file_error (), Qerror_message (),
	build_string (c"Remote file error".as_ptr ()));
  DEFVAR_LISP! (c"file-name-handler-alist", Vfile_name_handler_alist,
	       "Alist of elements (REGEXP . HANDLER) for file names handled specially.\nIf a file name matches REGEXP, all I/O on that file is done by calling\nHANDLER.  If a file name matches more than one handler, the handler\nwhose match starts last in the file name gets precedence.  The\nfunction `find-file-name-handler' checks this list for a handler for\nits argument.\n\nHANDLER should be a function.  The first argument given to it is the\nname of the I/O primitive to be handled; the remaining arguments are\nthe arguments that were passed to that primitive.  For example, if you\ndo (file-exists-p FILENAME) and FILENAME is handled by HANDLER, then\nHANDLER is called like this:\n\n  (funcall HANDLER \\='file-exists-p FILENAME)\n\nNote that HANDLER must be able to handle all I/O primitives; if it has\nnothing special to do for a primitive, it should reinvoke the\nprimitive to handle the operation \"the usual way\".\nSee Info node `(elisp)Magic File Names' for more details.");
  Vfile_name_handler_alist = Qnil ();

  Vfile_name_handler_alist = Qnil ();
  DEFVAR_LISP! (c"set-auto-coding-function", Vset_auto_coding_function,
	       "If non-nil, a function to call to decide a coding system of file.\nTwo arguments are passed to this function: the file name\nand the length of a file contents following the point.\nThis function should return a coding system to decode the file contents.\nIt should check the file name against `auto-coding-alist'.\nIf no coding system is decided, it should check a coding system\nspecified in the heading lines with the format:\n	-*- ... coding: CODING-SYSTEM; ... -*-\nor local variable spec of the tailing lines with `coding:' tag.");
  Vset_auto_coding_function = Qnil ();

  Vset_auto_coding_function = Qnil ();
  DEFVAR_LISP! (c"after-insert-file-functions", Vafter_insert_file_functions,
	       "A list of functions to be called at the end of `insert-file-contents'.\nEach is passed one argument, the number of characters inserted,\nwith point at the start of the inserted text.  Each function\nshould leave point the same, and return the new character count.\nIf `insert-file-contents' is intercepted by a handler from\n`file-name-handler-alist', that handler is responsible for calling the\nfunctions in `after-insert-file-functions' if appropriate.");
  Vafter_insert_file_functions = Qnil ();

  Vafter_insert_file_functions = Qnil ();
  DEFVAR_LISP! (c"write-region-annotate-functions", Vwrite_region_annotate_functions,
	       "A list of functions to be called at the start of `write-region'.\nEach is passed two arguments, START and END as for `write-region'.\nThese are usually two numbers but not always; see the documentation\nfor `write-region'.  The function should return a list of pairs\nof the form (POSITION . STRING), consisting of strings to be effectively\ninserted at the specified positions of the file being written (1 means to\ninsert before the first byte written).  The POSITIONs must be sorted into\nincreasing order.\n\nIf there are several annotation functions, the lists returned by these\nfunctions are merged destructively.  As each annotation function runs,\nthe variable `write-region-annotations-so-far' contains a list of all\nannotations returned by previous annotation functions.\n\nAn annotation function can return with a different buffer current.\nDoing so removes the annotations returned by previous functions, and\nresets START and END to `point-min' and `point-max' of the new buffer.\n\nAfter `write-region' completes, Emacs calls the function stored in\n`write-region-post-annotation-function', once for each buffer that was\ncurrent when building the annotations (i.e., at least once), with that\nbuffer current.");
  Vwrite_region_annotate_functions = Qnil ();

  Vwrite_region_annotate_functions = Qnil ();
  DEFSYM! (Qwrite_region_annotate_functions, c"write-region-annotate-functions");
  DEFVAR_LISP! (c"write-region-post-annotation-function", Vwrite_region_post_annotation_function,
	       "Function to call after `write-region' completes.\nThe function is called with no arguments.  If one or more of the\nannotation functions in `write-region-annotate-functions' changed the\ncurrent buffer, the function stored in this variable is called for\neach of those additional buffers as well, in addition to the original\nbuffer.  The relevant buffer is current during each function call.");
  Vwrite_region_post_annotation_function = Qnil ();

  Vwrite_region_post_annotation_function = Qnil ();
  staticpro (&raw const Vwrite_region_annotation_buffers);
  DEFVAR_LISP! (c"write-region-annotations-so-far", Vwrite_region_annotations_so_far,
	       "When an annotation function is called, this holds the previous annotations.\nThese are the annotations made by other annotation functions\nthat were already called.  See also `write-region-annotate-functions'.");
  Vwrite_region_annotations_so_far = Qnil ();

  Vwrite_region_annotations_so_far = Qnil ();
  DEFVAR_LISP! (c"inhibit-file-name-handlers", Vinhibit_file_name_handlers,
	       "A list of file name handlers that temporarily should not be used.\nThis applies only to the operation `inhibit-file-name-operation'.");
  Vinhibit_file_name_handlers = Qnil ();

  Vinhibit_file_name_handlers = Qnil ();
  DEFVAR_LISP! (c"inhibit-file-name-operation", Vinhibit_file_name_operation,
	       "The operation for which `inhibit-file-name-handlers' is applicable.");
  Vinhibit_file_name_operation = Qnil ();

  Vinhibit_file_name_operation = Qnil ();
  DEFVAR_LISP! (c"auto-save-list-file-name", Vauto_save_list_file_name,
	       "File name in which to write a list of all auto save file names.\nThis variable is initialized automatically from `auto-save-list-file-prefix'\nshortly after Emacs reads your init file, if you have not yet given it\na non-nil value.");
  Vauto_save_list_file_name = Qnil ();

  Vauto_save_list_file_name = Qnil ();
  DEFVAR_LISP! (c"auto-save-visited-file-name", Vauto_save_visited_file_name,
	       "Non-nil says auto-save a buffer in the file it is visiting, when practical.\nNormally auto-save files are written under other names.");
  Vauto_save_visited_file_name = Qnil ();

  Vauto_save_visited_file_name = Qnil ();
  DEFVAR_LISP! (c"auto-save-include-big-deletions", Vauto_save_include_big_deletions,
	       "If non-nil, auto-save even if a large part of the text is deleted.\nIf nil, deleting a substantial portion of the text disables auto-save\nin the buffer; this is the default behavior, because the auto-save\nfile is usually more useful if it contains the deleted text.");
  Vauto_save_include_big_deletions = Qnil ();

  Vauto_save_include_big_deletions = Qnil ();
  DEFVAR_BOOL! (c"write-region-inhibit-fsync", write_region_inhibit_fsync,
	       "Non-nil means don't call fsync in `write-region'.\nThis variable affects calls to `write-region' as well as save commands.\nBy default, it is non-nil.\n\nAlthough setting this to nil may avoid data loss if the system loses power,\nit can be a significant performance hit in the usual case, and it doesn't\nnecessarily cause file-save operations to actually survive a crash.");

  /* For more on why fsync often fails to work on today's hardware, see:
     Zheng M et al. Understanding the robustness of SSDs under power fault.
     11th USENIX Conf. on File and Storage Technologies, 2013 (FAST '13), 271-84
     https://www.usenix.org/system/files/conference/fast13/fast13-final80.pdf

     For more on why fsync does not suffice even if it works properly, see:
     Roche X. Necessary step(s) to synchronize filename operations on disk.
     Austin Group Defect 672, 2013-03-19
     https://austingroupbugs.net/view.php?id=672  */
  write_region_inhibit_fsync = true;
  DEFVAR_BOOL! (c"delete-by-moving-to-trash", delete_by_moving_to_trash,
	       "Specifies whether to use the system's trash can.\nWhen non-nil, certain file deletion commands use the function\n`move-file-to-trash' instead of deleting files outright.\nThis includes interactive calls to `delete-file' and\n`delete-directory' and the Dired deletion commands.");

  delete_by_moving_to_trash = false;
  DEFSYM! (Qdelete_by_moving_to_trash, c"delete-by-moving-to-trash");
  /* Lisp function for interactive file delete with trashing */
  DEFSYM! (Qdelete_file, c"delete-file");
  /* Lisp function for recursively copying directories.  */
  DEFSYM! (Qcopy_directory, c"copy-directory");
  /* Lisp function for recursively deleting directories.  */
  DEFSYM! (Qdelete_directory, c"delete-directory");
  DEFSYM! (Qsubstitute_env_in_file_name, c"substitute-env-in-file-name");
  DEFSYM! (Qget_buffer_window_list, c"get-buffer-window-list");
  DEFSYM! (Qstdin, c"stdin");
  DEFSYM! (Qstdout, c"stdout");
  DEFSYM! (Qstderr, c"stderr");
  defsubr (&raw mut Sfind_file_name_handler);
  defsubr (&raw mut Sfile_name_directory);
  defsubr (&raw mut Sfile_name_nondirectory);
  defsubr (&raw mut Sunhandled_file_name_directory);
  defsubr (&raw mut Sfile_name_as_directory);
  defsubr (&raw mut Sdirectory_name_p);
  defsubr (&raw mut Sdirectory_file_name);
  defsubr (&raw mut Smake_temp_file_internal);
  defsubr (&raw mut Smake_temp_name);
  defsubr (&raw mut Sfile_name_concat);
  defsubr (&raw mut Sexpand_file_name);
  defsubr (&raw mut Ssubstitute_in_file_name);
  defsubr (&raw mut Scopy_file);
  defsubr (&raw mut Smake_directory_internal);
  defsubr (&raw mut Sdelete_directory_internal);
  defsubr (&raw mut Sdelete_file_internal);
  defsubr (&raw mut Sfile_name_case_insensitive_p);
  defsubr (&raw mut Srename_file);
  defsubr (&raw mut Sadd_name_to_file);
  defsubr (&raw mut Smake_symbolic_link);
  defsubr (&raw mut Sfile_name_absolute_p);
  defsubr (&raw mut Sfile_exists_p);
  defsubr (&raw mut Sfile_executable_p);
  defsubr (&raw mut Sfile_readable_p);
  defsubr (&raw mut Sfile_writable_p);
  defsubr (&raw mut Saccess_file);
  defsubr (&raw mut Sfile_symlink_p);
  defsubr (&raw mut Sfile_directory_p);
  defsubr (&raw mut Sfile_accessible_directory_p);
  defsubr (&raw mut Sfile_regular_p);
  defsubr (&raw mut Sfile_modes);
  defsubr (&raw mut Sset_file_modes);
  defsubr (&raw mut Sset_file_times);
  defsubr (&raw mut Sfile_selinux_context);
  defsubr (&raw mut Sfile_acl);
  defsubr (&raw mut Sset_file_acl);
  defsubr (&raw mut Sset_file_selinux_context);
  defsubr (&raw mut Sset_default_file_modes);
  defsubr (&raw mut Sdefault_file_modes);
  defsubr (&raw mut Sfile_newer_than_file_p);
  defsubr (&raw mut Sinsert_file_contents);
  defsubr (&raw mut Swrite_region);
  defsubr (&raw mut Scar_less_than_car);
  defsubr (&raw mut Sverify_visited_file_modtime);
  defsubr (&raw mut Svisited_file_modtime);
  defsubr (&raw mut Sset_visited_file_modtime);
  defsubr (&raw mut Sdo_auto_save);
  defsubr (&raw mut Sset_buffer_auto_saved);
  defsubr (&raw mut Sclear_buffer_auto_save_failure);
  defsubr (&raw mut Srecent_auto_save_p);
  defsubr (&raw mut Snext_read_file_uses_dialog_p);
  defsubr (&raw mut Sset_binary_mode);
  defsubr (&raw mut Sfile__close_stream);
  /* C 7043/7048/7052: `#ifndef DOS_NT` - the frozen configuration takes the live arm.  */
  defsubr (&raw mut Sfile_system_info);
  /* C 7043/7048/7052: `#ifdef HAVE_SYNC` - the frozen configuration takes the live arm.  */
  defsubr (&raw mut Sunix_sync);
  DEFSYM! (Qif_regular, c"if-regular");
  DEFSYM! (Qbuffer_file_name, c"buffer-file-name");
  DEFSYM! (Qauto_save, c"auto-save");
  DEFSYM! (QCerror, c":error");
  DEFSYM! (Qauto_save_hook, c"auto-save-hook");
}

unsafe extern "C" {
  /// C: lisp.h 4265: `extern Lisp_Object merge (Lisp_Object, Lisp_Object, Lisp_Object);`
  pub fn merge (org_l1: Lisp_Object, l2: Lisp_Object, pred: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));` - eval.rs
  /// defines it and `lisp.rs` carries no declaration.
  pub fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
}
