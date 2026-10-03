// femacs: port of src/dired.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The eight
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's two non-`static` definitions
// (`directory_files_internal`, `syms_of_dired`) carry `#[unsafe(no_mangle)]` (PORTING.md
// 0.2); the eleven `static` helpers are plain `fn`/`unsafe fn`, except
// `directory_files_internal_unwind`, which is `unsafe extern "C" fn` because C hands its
// address to `record_unwind_protect_ptr` (PORTING.md 1.4, the filelock.c shape).  C's
// `DEFVAR_LISP` variable (`Vcompletion_ignored_extensions`) is defined here and shadows the
// `globals.rs` declaration instead of clashing with it (P-21).
//
// The dead regions are recorded in docs/ports/dired.md rather than transcribed: `#ifdef
// MSDOS` (C 43-45's `<msdos.h>` include), the `#if !(defined HAVE_ANDROID && !defined
// ANDROID_STUBIFY)` alternative (C 51-60: `"android.h"`, the `android_vdir` typedef and the
// two Android `#define`s; C 113-146's whole DOS_NT/Android arm of `open_directory`),
// `#ifdef WINDOWSNT` (C 62-64's `is_slow_fs`, C 169-175's
// `directory_files_internal_w32_unwind`, C 198-205's `ENOENT`/`EACCES` arm, C 256-275's
// `w32_save` block, C 293-295's `case_table` assignment, C 359-362's restore, C 926-928's
// and C 941-943's `st_uname`/`st_gname` arms, C 1070-1080's two
// `w32_stat_get_owner_group` arms), `#ifdef _D_EXACT_NAMLEN` (C 74-76: the macro is
// undefined on the frozen target - measured with clang, which is what
// docs/baseline/config/cond-eval-verified.txt records for this file), the `#ifndef
// HAVE_STRUCT_DIRENT_D_TYPE` enum (C 81-92, with C 101-103's `return DT_UNKNOWN;` arm;
// HAVE_STRUCT_DIRENT_D_TYPE is 1 and the three DT_* values are macros), the `#if 0` block in
// `file_name_completion` (C 629-639) and the `#if defined O_PATH && !defined
// HAVE_CYGWIN_O_PATH_BUG && ...` block of `file_attributes` (C 1042-1066: O_PATH is
// undefined).  Every `#if` in this file is decided by
// docs/baseline/config/darwin-arm64.macros.tsv, and src/dired.c is already in
// cond-eval-verified.txt, so the extractor's branch selection is the clang-verified one; no
// source-header-macros.tsv row is needed.
//
// The `#include` list is C 21-41's, through the mirrors those pull in: config.h (21),
// `<sys/stat.h>` (23), `<dirent.h>` (34), `<filemode.h>` (35), `<stat-time.h>` (36),
// `<errno.h>` (30), `<fcntl.h>` (31), `<unistd.h>` (32), lisp.h (38), systime.h (39),
// buffer.h (40) and coding.h (41).  `<sys/stat.h>` pulls `<sys/types.h>`, and lisp.h 30 its
// `<string.h>` (strlen/strcmp/memcpy/strcpy), both of which the port includes explicitly.
// `<pwd.h>` (C 26, live: HAVE_PWD_H is 1) and `<grp.h>` (C 28) have no mirror, and every name
// this unit takes from them is declared at its use site below; so are the lisp.h entries the
// lisp.rs mirror does not carry yet (`sys_faccessat`, `check_emacs_readlinkat`,
// `file_attribute_errno`, `internal_condition_case_2`, `plist_get`, `re_match_object`), the
// `DIRECTORY_SEP`/`IS_ANY_SEP`/`IS_DIRECTORY_SEP` macros (config.h, not config.rs), the three
// `DT_*` values the mirror does not carry, and `struct dirent` itself (the dirent mirror's own
// note says it is not mirrored; the frozen target's layout is measured in docs/ports/dired.md).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");                 // C 21
include!("../lib/include/sys_types.in.rs");    // <sys/stat.h> (C 23) pulls it
include!("../lib/include/pwd.rs");           // C: `#include <pwd.h>`
include!("../lib/include/grp.rs");           // C: `#include <grp.h>`
include!("../lib/include/sys_utsname.rs");   // C: `#include <sys/utsname.h>`
include!("../lib/include/sys_stat.in.rs");     // C 23: <sys/stat.h>
include!("../lib/include/dirent.rs");          // C 34: <dirent.h>
include!("../lib/include/filemode.rs");        // C 35: <filemode.h>
include!("../lib/include/stat-time.rs");       // C 36: <stat-time.h>
include!("../lib/include/errno.rs");           // C 30: <errno.h>
include!("../lib/include/fcntl.in.rs");        // C 31: <fcntl.h>
include!("../lib/include/unistd.in.rs");       // C 32: <unistd.h>
include!("../lib/include/string.rs");          // lisp.h 30: <string.h>
include!("include/lisp.rs");                   // C 38
include!("include/systime.rs");                // C 39
include!("include/itree.rs");                  // buffer.h 27: <itree.h> (buffer.rs's members)
include!("include/character.rs");              // buffer.rs's CHAR_STRING family
include!("include/intervals.rs");              // buffer.rs's INTERVAL
include!("include/composite.rs");              // coding.h 284: <composite.h> (composition_method)
include!("include/buffer.rs");                 // C 40
include!("include/coding.rs");                 // C 41

/* The value C's `<dirent.h>` gives `struct dirent` on the frozen target.  The port's dirent
   mirror carries the opaque `DIR` and the directory calls but not the record itself, so it
   is declared here (the terminal.rs/filelock.rs shape).  Measured with clang against the
   frozen target's SDK (the probe is recorded in docs/ports/dired.md):
   sizeof = 1048, align 8, `d_ino@0`, `d_seekoff@8`, `d_reclen@16`, `d_namlen@18`,
   `d_type@20`, `d_name@21` (1024 bytes, `d_type` 1 byte).  */
#[repr(C)]
pub struct dirent
{
  pub d_ino: u64,
  pub d_seekoff: u64,
  pub d_reclen: u16,
  pub d_namlen: u16,
  pub d_type: u8,
  pub d_name: [c_char; 1024],
}
const _: () = assert!(std::mem::size_of::<dirent> () == 1048);
const _: () = assert!(std::mem::offset_of! (dirent, d_type) == 20);
const _: () = assert!(std::mem::offset_of! (dirent, d_name) == 21);

/// C: `<dirent.h>`'s `#define DT_UNKNOWN 0`, `DT_DIR 4`, `DT_LNK 10` on the frozen target
/// (docs/baseline/config/darwin-arm64.macros.tsv 781/776/778); the dirent mirror does not
/// carry them, and C 81-92's enum of the same names is dead here.
pub const DT_UNKNOWN: c_int = 0;
pub const DT_DIR: c_int = 4;
pub const DT_LNK: c_int = 10;

/// C: `<config.h>` 120: `#define DIRECTORY_SEP '/'` (config.h splices the `s/` header's
/// value; measured in build/emacs-ref/src/config.h).
pub const DIRECTORY_SEP: u8 = b'/';

/// C: `<config.h>` 2097: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)`.
macro_rules! IS_DIRECTORY_SEP { ($c:expr) => { ($c) == DIRECTORY_SEP }; }

/// C: `<config.h>` 2091: `#define IS_ANY_SEP(_c_) (IS_DIRECTORY_SEP (_c_))`.
macro_rules! IS_ANY_SEP { ($c:expr) => { IS_DIRECTORY_SEP! ($c) }; }

/* The names C gets from headers without a mirror and from lisp.h entries the mirror does not
   carry yet, declared at the use site (the terminal.rs/filelock.rs shape).  `getpwuid`'s and
   `getgrgid`'s parameters are the frozen target's `uid_t`/`gid_t`.  */
unsafe extern "C" {
  /// C: `<pwd.h>`: `void endpwent (void)`
  pub fn endpwent ();
  /// C: `<grp.h>`: `struct group *getgrent (void)`
  pub fn getgrent () -> *mut group;
  /// C: `<grp.h>`: `void endgrent (void)`
  pub fn endgrent ();

  /// C: lisp.h 5303: `int emacs_fstatat (int, char const *, void *, int)`
  pub fn emacs_fstatat (fd: c_int, name: *const c_char, buf: *mut c_void, flag: c_int) -> c_int;
  /// C: lisp.h 5307: `int sys_faccessat (int, const char *, int, int)`
  pub fn sys_faccessat (fd: c_int, name: *const c_char, mode: c_int, flag: c_int) -> c_int;
  /// C: lisp.h 4986: `Lisp_Object check_emacs_readlinkat (int, Lisp_Object, char const *)`
  pub fn check_emacs_readlinkat (fd: c_int, filename: Lisp_Object,
                                 name: *const c_char) -> Lisp_Object;
  /// C: lisp.h 4984: `Lisp_Object file_attribute_errno (Lisp_Object, int)`
  pub fn file_attribute_errno (file: Lisp_Object, err: c_int) -> Lisp_Object;
  /// C: lisp.h 4807: `Lisp_Object internal_condition_case_2 (Lisp_Object (*) (Lisp_Object, Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object (*) (Lisp_Object))`
  pub fn internal_condition_case_2 (bfun: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object) -> Lisp_Object>,
                                    arg1: Lisp_Object, arg2: Lisp_Object, handlers: Lisp_Object,
                                    hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>) -> Lisp_Object;
  /// C: lisp.h 4280: `Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop)`
  pub fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;

  /// C: regex-emacs.h 50: `extern Lisp_Object re_match_object;`
  pub static mut re_match_object: Lisp_Object;
}

/* C 47-50's live arm of the `emacs_dir` typedef and the two directory calls.  The two macros
   keep C's names (PORTING.md P-37): the port's `readdir` returns `*mut c_void` (the dirent
   mirror's declaration, which no unit had needed to look inside before) and the unwind
   callback below receives a `void *`, so each macro spells the cast C's implicit conversion
   makes at the call site.  The Android alternative (C 51-60) is dead and recorded.  */
type emacs_dir = DIR;
macro_rules! emacs_readdir { ($dirp:expr) => { readdir ($dirp) as *mut dirent }; }
macro_rules! emacs_closedir { ($dirp:expr) => { closedir ($dirp as *mut DIR) }; }

/* Return the number of bytes in DP's name.  */
unsafe fn dirent_namelen (dp: *mut dirent) -> ptrdiff_t
{
  /* C 74-75's `#ifdef _D_EXACT_NAMLEN` arm is dead: the macro is not defined on the frozen
     target (measured with clang: the `#else` arm is the one the C preprocessor takes).  */
  return strlen ((*dp).d_name.as_ptr ()) as ptrdiff_t;
}

/* Return the file type of DP.  */
unsafe fn dirent_type (dp: *mut dirent) -> c_int
{
  /* HAVE_STRUCT_DIRENT_D_TYPE is 1 on the frozen target, so C 99's arm is the live one and
     the `return DT_UNKNOWN;` of C 102 is dead.  */
  return (*dp).d_type as c_int;
}

unsafe fn open_directory (dirname: Lisp_Object, encoded_dirname: Lisp_Object,
                          fdp: *mut c_int) -> *mut emacs_dir
{
  let name: *mut c_char = SSDATA (encoded_dirname);
  let d: *mut emacs_dir;
  let fd: c_int;
  let opendir_errno: c_int;

  /* C 113-147's `#if defined DOS_NT || (defined HAVE_ANDROID && !defined ANDROID_STUBIFY)`
     arm (`fd = 0`, `opendir` and the Android `again:` loop) is dead on this target; C 148's
     `#else` body below is the live one.  */

  fd = emacs_open (name, O_RDONLY | O_DIRECTORY, 0);
  if fd < 0
    {
      opendir_errno = *errno ();
      d = std::ptr::null_mut ();
    }
  else
    {
      d = fdopendir (fd);
      opendir_errno = *errno ();
      if d.is_null ()
	{ emacs_close (fd); }
    }

  if d.is_null ()
    { report_file_errno (c"Opening directory".as_ptr (), dirname, opendir_errno); }
  *fdp = fd;
  return d;
}

/* C 169-175's `#ifdef WINDOWSNT` `directory_files_internal_w32_unwind` is dead.  */

unsafe extern "C" fn directory_files_internal_unwind (d: *mut c_void)
{
  emacs_closedir! (d);
}

/* Return the next directory entry from DIR; DIR's name is DIRNAME.
   If there are no more directory entries, return a null pointer.
   Signal any unrecoverable errors.  */

unsafe fn read_dirent (dir: *mut emacs_dir, dirname: Lisp_Object) -> *mut dirent
{
  /* C's `while (true)`: a Rust `while` loop has type `()` even with a literal condition, and
     this loop is the function's tail (its only exit is the `return dp;` inside), so the port
     spells C's infinite loop as Rust's diverging `loop` (docs/ports/macro-body-notes.tsv).  */
  loop
    {
      *errno () = 0;
      let dp: *mut dirent = emacs_readdir! (dir);
      if !dp.is_null () || *errno () == 0
	{ return dp; }
      if !(*errno () == EAGAIN || *errno () == EINTR)
	{
	  /* C 198-204's `#ifdef WINDOWSNT` arm (`report_file_error ("Opening directory",
	     dirname)` for ENOENT/EACCES) is dead on this target.  */
	  report_file_error (c"Reading directory".as_ptr (), dirname);
	}
      maybe_quit ();
    }
}

/* Function shared by Fdirectory_files and Fdirectory_files_and_attributes.
   If not ATTRS, return a list of directory filenames;
   if ATTRS, return a list of directory filenames and their attributes.
   In the latter case, pass ID_FORMAT to file_attributes.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn directory_files_internal (directory: Lisp_Object, full: Lisp_Object,
			  r#match: Lisp_Object, nosort: Lisp_Object, attrs: bool,
			  id_format: Lisp_Object, return_count: Lisp_Object) -> Lisp_Object
{
  let mut ind: EMACS_INT = 0;
  let mut last: EMACS_INT = MOST_POSITIVE_FIXNUM;

  if !NILP (return_count)
    {
      CHECK_FIXNAT (return_count);
      last = XFIXNAT (return_count);
    }

  if !NILP (r#match)
    { CHECK_STRING (r#match); }

  /* Don't let the compiler optimize away all copies of DIRECTORY,
     which would break GC; see Bug#16986.  */
  let directory_volatile: Lisp_Object = directory;

  let dirfilename: Lisp_Object = Fdirectory_file_name (directory);

  /* Note: ENCODE_FILE and DECODE_FILE can GC because they can run
     run_pre_post_conversion_on_str which calls Lisp directly and
     indirectly.  */
  let encoded_dirfilename: Lisp_Object = ENCODE_FILE (dirfilename);

  let mut fd: c_int = 0;

  /* Keep in mind that FD is not always a real file descriptor on
     Android.  */
  let d: *mut emacs_dir = open_directory (dirfilename, encoded_dirfilename, &raw mut fd);

  /* Unfortunately, we can now invoke expand-file-name and
     file-attributes on filenames, both of which can throw, so we must
     do a proper unwind-protect.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_ptr (Some (directory_files_internal_unwind), d as *mut c_void);

  /* C 256-275's `#ifdef WINDOWSNT` `w32_save` block is dead on this target.  */

  /* C's `directory = DECODE_FILE (directory);` below rebinds the argument.  */
  let mut directory = directory;
  if !NILP (full) && !STRING_MULTIBYTE (directory)
    { /* We will be concatenating 'directory' with local file name.
         We always decode local file names, so in order to safely concatenate
         them we need 'directory' to be decoded as well (bug#56469).  */
      directory = DECODE_FILE (directory);
    }

  let directory_nbytes: ptrdiff_t = SBYTES (directory);
  re_match_object = Qt ();

  /* Decide whether we need to add a directory separator.  */
  let needsep: bool = (directory_nbytes == 0
		  || !IS_ANY_SEP! (SREF (directory, (directory_nbytes - 1) as usize)));

  /* Windows users want case-insensitive wildcards.  */
  let case_table: Lisp_Object = Qnil ();
  /* C 293-294's `#ifdef WINDOWSNT` `case_table = BVAR (&buffer_defaults,
     case_canon_table);` is dead on this target.  */

  /* Read directory entries and accumulate them into LIST.  */
  let mut list: Lisp_Object = Qnil ();
  let mut dp: *mut dirent = std::ptr::null_mut ();
  while { dp = read_dirent (d, directory); !dp.is_null () }
    {
      let len: ptrdiff_t = dirent_namelen (dp);
      let mut name: Lisp_Object = make_unibyte_string ((*dp).d_name.as_ptr (), len);
      let mut finalname: Lisp_Object = name;

      /* This can GC.  */
      name = DECODE_FILE (name);

      maybe_quit ();

      if !NILP (r#match)
	  && fast_string_match_internal (r#match, name, case_table) < 0
	{ continue; }

      /* C's `Lisp_Object fileattrs UNINIT;` starts the port's local at zero: the `UNINIT`
         marker has no Rust counterpart (docs/ports/macro-body-notes.tsv).  */
      let mut fileattrs: Lisp_Object = std::ptr::null_mut ();
      if attrs
	{
	  fileattrs = file_attributes (fd, (*dp).d_name.as_ptr (), directory, name,
				       id_format);
	  if NILP (fileattrs)
	    { continue; }
	}

      if !NILP (full)
	{
	  let name_nbytes: ptrdiff_t = SBYTES (name);
	  let nbytes: ptrdiff_t = directory_nbytes + needsep as ptrdiff_t + name_nbytes;
	  let nchars: ptrdiff_t = SCHARS (directory) + needsep as ptrdiff_t + SCHARS (name);
	  /* DECODE_FILE may return non-ASCII unibyte strings (e.g. when
             file-name-coding-system is 'binary'), so we don't know for sure
             that the bytes we have follow our internal utf-8 representation
             for multibyte strings.  If nchars == nbytes we don't need to
             care and just return a unibyte string; and if not, that means
             one of 'name' or 'directory' is multibyte, in which case we
             presume that the other one would also be multibyte if it
             contained non-ASCII.
             FIXME: This last presumption is broken when 'directory' is
             multibyte (with non-ASCII), and 'name' is unibyte with non-ASCII
             (because file-name-coding-system is 'binary').  */
	  finalname = if nchars == nbytes
	              { make_uninit_string (nbytes) }
	              else { make_uninit_multibyte_string (nchars, nbytes) };
	  memcpy (SDATA (finalname) as *mut c_void, SDATA (directory) as *const c_void,
		  directory_nbytes as usize);
	  if needsep
	    { SSET (finalname, directory_nbytes as usize, DIRECTORY_SEP); }
	  memcpy (SDATA (finalname).offset (directory_nbytes + needsep as ptrdiff_t)
		    as *mut c_void,
		  SDATA (name) as *const c_void, name_nbytes as usize);
	}
      else
	{ finalname = name; }

      if ind == last
          { break; }
      ind += 1;

      list = Fcons (if attrs { Fcons (finalname, fileattrs) } else { finalname }, list);
    }

  emacs_closedir! (d);
  /* C 359-361's `#ifdef WINDOWSNT` `Vw32_get_true_file_attributes = w32_save;` is dead.  */

  /* Discard the unwind protect.  */
  specpdl_ptr = specpdl_ref_to_ptr (count);

  if NILP (nosort)
    {
      let ordered: Lisp_Object = Fnreverse (list);
      list = CALLN! (Fsort, ordered,
		    if attrs { Qfile_attributes_lessp () } else { Qstring_lessp () });
    }

  /* C's `(void) directory_volatile;` keeps the volatile copy of DIRECTORY alive (Bug#16986);
     reading it is the port's counterpart of the cast to void.  */
  let _ = directory_volatile;
  return list;
}

DEFUN! ("directory-files", Fdirectory_files, Sdirectory_files, 1, 5, 0,
       doc: "Return a list of names of files in DIRECTORY.\nThere are four optional arguments:\nIf FULL is non-nil, return absolute file names.  Otherwise return names\n that are relative to the specified directory.\nIf MATCH is non-nil, mention only file names whose non-directory part\n matches the regexp MATCH.\nIf NOSORT is non-nil, the list is not sorted--its order is unpredictable.\n Otherwise, the list returned is sorted with `string-lessp'.\n NOSORT is useful if you plan to sort the result yourself.\nIf COUNT is non-nil and a natural number, the function will return\n COUNT number of file names (if so many are present).",
       (directory: Lisp_Object, full: Lisp_Object, r#match: Lisp_Object,
	nosort: Lisp_Object, count: Lisp_Object),
       {
  /* C's `directory = Fexpand_file_name (directory, Qnil);` rebinds the argument.  */
  let mut directory = directory;
  directory = Fexpand_file_name (directory, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object;
  handler = Ffind_file_name_handler (directory, Qdirectory_files ());
  if !NILP (handler)
    { return calln! (handler, Qdirectory_files (), directory,
                     full, r#match, nosort, count); }

  return directory_files_internal (directory, full, r#match, nosort,
                                   false, Qnil (), count);
});

DEFUN! ("directory-files-and-attributes", Fdirectory_files_and_attributes,
       Sdirectory_files_and_attributes, 1, 6, 0,
       doc: "Return a list of names of files and their attributes in DIRECTORY.\nValue is a list of the form:\n\n  ((FILE1 . FILE1-ATTRS) (FILE2 . FILE2-ATTRS) ...)\n\nwhere each FILEn-ATTRS is the attributes of FILEn as returned\nby `file-attributes'.\n\nThis function accepts five optional arguments:\nIf FULL is non-nil, return absolute file names.  Otherwise return names\n that are relative to the specified directory.\nIf MATCH is non-nil, mention only file names whose non-directory part\n matches the regexp MATCH.\nIf NOSORT is non-nil, the list is not sorted--its order is unpredictable.\n NOSORT is useful if you plan to sort the result yourself.\nID-FORMAT specifies the preferred format of attributes uid and gid, see\n `file-attributes' for further documentation.\nIf COUNT is non-nil and a natural number, the function will return\n COUNT number of file names (if so many are present).\nOn MS-Windows, performance depends on `w32-get-true-file-attributes',\nwhich see.",
       (directory: Lisp_Object, full: Lisp_Object, r#match: Lisp_Object,
	nosort: Lisp_Object, id_format: Lisp_Object, count: Lisp_Object),
       {
  /* C's `directory = Fexpand_file_name (directory, Qnil);` rebinds the argument.  */
  let mut directory = directory;
  directory = Fexpand_file_name (directory, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  let handler: Lisp_Object;
  handler
    = Ffind_file_name_handler (directory, Qdirectory_files_and_attributes ());
  if !NILP (handler)
    { return calln! (handler, Qdirectory_files_and_attributes (),
                     directory, full, r#match, nosort, id_format, count); }

  return directory_files_internal (directory, full, r#match, nosort,
				   true, id_format, count);
});

DEFUN! ("file-name-completion", Ffile_name_completion, Sfile_name_completion,
       2, 3, 0,
       doc: "Complete file name FILE in directory DIRECTORY.\nReturns the longest string\ncommon to all file names in DIRECTORY that start with FILE.\nIf there is only one and FILE matches it exactly, returns t.\nReturns nil if DIRECTORY contains no name starting with FILE.\n\nIf PREDICATE is non-nil, call PREDICATE with each possible\ncompletion (in absolute form) and ignore it if PREDICATE returns nil.\n\nThis function ignores some of the possible completions as determined\nby the variables `completion-regexp-list' and\n`completion-ignored-extensions', which see.  `completion-regexp-list'\nis matched against file and directory names relative to DIRECTORY.",
       (file: Lisp_Object, directory: Lisp_Object, predicate: Lisp_Object),
       {
  /* C's `directory = Fexpand_file_name (directory, Qnil);` rebinds the argument.  */
  let mut directory = directory;
  let mut handler: Lisp_Object;
  directory = Fexpand_file_name (directory, Qnil ());

  /* If the directory name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (directory, Qfile_name_completion ());
  if !NILP (handler)
    { return calln! (handler, Qfile_name_completion (), file, directory, predicate); }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qfile_name_completion ());
  if !NILP (handler)
    { return calln! (handler, Qfile_name_completion (), file, directory, predicate); }

  return file_name_completion (file, directory, false, predicate);
});

DEFUN! ("file-name-all-completions", Ffile_name_all_completions,
       Sfile_name_all_completions, 2, 2, 0,
       doc: "Return a list of all completions of file name FILE in directory DIRECTORY.\nThese are all file names in directory DIRECTORY which begin with FILE.\n\nThis function ignores some of the possible completions as determined\nby `completion-regexp-list', which see.  `completion-regexp-list'\nis matched against file and directory names relative to DIRECTORY.",
       (file: Lisp_Object, directory: Lisp_Object),
       {
  /* C's `directory = Fexpand_file_name (directory, Qnil);` rebinds the argument.  */
  let mut directory = directory;
  let mut handler: Lisp_Object;
  directory = Fexpand_file_name (directory, Qnil ());

  /* If the directory name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (directory, Qfile_name_all_completions ());
  if !NILP (handler)
    { return calln! (handler, Qfile_name_all_completions (), file, directory); }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (file, Qfile_name_all_completions ());
  if !NILP (handler)
    { return calln! (handler, Qfile_name_all_completions (), file, directory); }

  return file_name_completion (file, directory, true, Qnil ());
});

unsafe fn file_name_completion (file: Lisp_Object, dirname: Lisp_Object, all_flag: bool,
				predicate: Lisp_Object) -> Lisp_Object
{
  let mut bestmatchsize: ptrdiff_t = 0;
  let mut matchcount: c_int = 0;
  /* If ALL_FLAG is 1, BESTMATCH is the list of all matches, decoded.
     If ALL_FLAG is 0, BESTMATCH is either nil
     or the best match so far, not decoded.  */
  let mut bestmatch: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut elt: Lisp_Object;
  let mut name: Lisp_Object;
  let mut encoded_file: Lisp_Object;
  let mut encoded_dir: Lisp_Object;
  let mut directoryp: bool = false;
  /* If not INCLUDEALL, exclude files in completion-ignored-extensions as
     well as "." and "..".  Until shown otherwise, assume we can't exclude
     anything.  */
  let mut includeall: bool = true;
  let mut check_decoded: bool = false;
  let count: specpdl_ref = SPECPDL_INDEX ();

  elt = Qnil ();

  CHECK_STRING (file);

  bestmatch = Qnil ();
  encoded_dir = Qnil ();
  encoded_file = encoded_dir;
  specbind (Qdefault_directory (), dirname);

  /* Do completion on the encoded file name
     because the other names in the directory are (we presume)
     encoded likewise.  We decode the completed string at the end.  */
  /* Actually, this is not quite true any more: we do most of the completion
     work with decoded file names, but we still do some filtering based
     on the encoded file name.  */
  encoded_file = ENCODE_FILE (file);
  encoded_dir = ENCODE_FILE (Fdirectory_file_name (dirname));

  let mut file_encoding: Lisp_Object = Vfile_name_coding_system;
  if NILP (Vfile_name_coding_system)
    { file_encoding = Vdefault_file_name_coding_system; }
  /* If the file-name encoding decomposes characters, as we do for
     HFS+ filesystems, we need to make an additional comparison of
     decoded names in order to filter false positives, such as "a"
     falsely matching "a-ring".  */
  /* C's `file = DECODE_FILE (encoded_file);` below rebinds the argument.  */
  let mut file = file;
  if !NILP (file_encoding)
      && !NILP (plist_get (Fcoding_system_plist (file_encoding),
			   Qdecomposed_characters ()))
    {
      check_decoded = true;
      if STRING_MULTIBYTE (file)
	{
	  /* Recompute FILE to make sure any decomposed characters in
	     it are re-composed by the post-read-conversion.
	     Otherwise, any decomposed characters will be rejected by
	     the additional check below.  */
	  file = DECODE_FILE (encoded_file);
	}
    }
  let mut fd: c_int = 0;
  let d: *mut emacs_dir = open_directory (dirname, encoded_dir, &raw mut fd);
  record_unwind_protect_ptr (Some (directory_files_internal_unwind), d as *mut c_void);

  /* Loop reading directory entries.  */
  let zero: Lisp_Object = make_fixnum (0);
  let enc_file_len: ptrdiff_t = SCHARS (encoded_file);
  let file_len: Lisp_Object = make_fixnum (SCHARS (file));
  let mut dp: *mut dirent = std::ptr::null_mut ();
  while { dp = read_dirent (d, dirname); !dp.is_null () }
    {
      let len: ptrdiff_t = dirent_namelen (dp);
      let mut canexclude: bool = false;

      maybe_quit ();

      if len < enc_file_len
	  /* scmp cannot reliably compare non-ASCII strings while
	     ignoring letter-case.  */
	  || (!completion_ignore_case
	      && scmp ((*dp).d_name.as_ptr (), SSDATA (encoded_file), enc_file_len) >= 0)
	{ continue; }

      name = make_unibyte_string ((*dp).d_name.as_ptr (), len);
      name = DECODE_FILE (name);
      let name_blen: ptrdiff_t = SBYTES (name);
      let name_len: ptrdiff_t = SCHARS (name);
      if completion_ignore_case
	  && !BASE_EQ (Fcompare_strings (name, zero, file_len, file, zero,
					 file_len, Qt ()),
		       Qt ())
	{ continue; }

      /* C 601's `switch` becomes a `match` whose arms keep C's order and shared body
         (docs/ports/macro-body-notes.tsv); Rust has no `switch`/`case`/`break` here.  */
      match dirent_type (dp)
	{
	  DT_DIR =>
	    { directoryp = true; }

	  DT_LNK | DT_UNKNOWN =>
	    { directoryp = file_name_completion_dirp (fd, dp, len); }

	  _ =>
	    { directoryp = false; }
	}

      tem = Qnil ();
      /* If all_flag is set, always include all.
	 It would not actually be helpful to the user to ignore any possible
	 completions when making a list of them.  */
      if !all_flag
	{
	  let mut skip: ptrdiff_t = 0;
	  let mut cmp_len: Lisp_Object = make_fixnum (name_len);

	  /* C 629-639's `#if 0` block (a FIXME about comparing an encoded with a decoded
	     string through `scmp`) is dead text, recorded in docs/ports/dired.md.  */

	  if directoryp
	    {
	      /* C 644-646's `#ifndef TRIVIAL_DIRECTORY_ENTRY` defines the macro here, for
	         the one call below.  C's `!strcmp (n, ".")` is the port's `== 0`.  */
	      macro_rules! TRIVIAL_DIRECTORY_ENTRY {
	          ($n:expr) => { strcmp ($n, c".".as_ptr ()) == 0
	                         || strcmp ($n, c"..".as_ptr ()) == 0 };
	      }
	      /* "." and ".." are never interesting as completions, and are
		 actually in the way in a directory with only one file.  */
	      if TRIVIAL_DIRECTORY_ENTRY! ((*dp).d_name.as_ptr ())
		{ canexclude = true; }
	      else if len > enc_file_len
		/* Ignore directories if they match an element of
		   completion-ignored-extensions which ends in a slash.  */
		{ tem = Vcompletion_ignored_extensions;
		  /* C's `for (...; CONSP (tem); tem = XCDR (tem))`: Rust has no
		     for-increment, and C's body ends in `break` (which skips it), so C's step is
		     written before every `continue`, where C's for-loop would have run it (the
		     rawmemchr/strnlen precedent).  */
		  while CONSP (tem)
		    {
		      let mut elt_len: ptrdiff_t = 0;
		      let mut p1: *mut c_char = std::ptr::null_mut ();

		      elt = XCAR (tem);
		      if !STRINGP (elt)
			{ tem = XCDR (tem); continue; }
		      elt_len = SBYTES (elt) - 1; /* -1 for trailing / */
		      if elt_len <= 0
			{ tem = XCDR (tem); continue; }
		      p1 = SSDATA (elt);
		      if *p1.offset (elt_len) != DIRECTORY_SEP as c_char
			{ tem = XCDR (tem); continue; }
		      skip = name_blen - elt_len;
		      if skip < 0
			{ tem = XCDR (tem); continue; }

		      if !completion_ignore_case
			  && scmp (SSDATA (name).offset (skip), p1, elt_len) >= 0
			{ tem = XCDR (tem); continue; }
		      if completion_ignore_case
			{
			  elt_len = SCHARS (elt) - 1;
			  skip = name_len - elt_len;
			  cmp_len = make_fixnum (elt_len);
			  if skip < 0
			      || !BASE_EQ (Fcompare_strings (name,
							     make_fixnum (skip),
							     Qnil (),
							     elt, zero, cmp_len,
							     Qt ()),
					   Qt ())
			    { tem = XCDR (tem); continue; }
			}
		      break;
		    }
		}
	    }
	  else
	    {
	      /* Compare extensions-to-be-ignored against end of this file name */
	      /* if name is not an exact match against specified string */
	      if len > enc_file_len
		/* and exit this for loop if a match is found */
		{ tem = Vcompletion_ignored_extensions;
		  while CONSP (tem)
		    {
		      elt = XCAR (tem);
		      if !STRINGP (elt) { tem = XCDR (tem); continue; }
		      let mut elt_len: ptrdiff_t = SBYTES (elt);
		      skip = len - elt_len;
		      if skip < 0 { tem = XCDR (tem); continue; }

		      if !completion_ignore_case
			  && (scmp (SSDATA (name).offset (skip), SSDATA (elt), elt_len)
			      >= 0)
			{ tem = XCDR (tem); continue; }
		      if completion_ignore_case
			{
			  elt_len = SCHARS (elt);
			  skip = name_len - elt_len;
			  cmp_len = make_fixnum (elt_len);
			  if skip < 0
			      || !BASE_EQ (Fcompare_strings (name,
							     make_fixnum (skip),
							     Qnil (),
							     elt, zero, cmp_len,
							     Qt ()),
					   Qt ())
			    { tem = XCDR (tem); continue; }
			}
		      break;
		    }
		}
	    }

	  /* If an ignored-extensions match was found,
	     don't process this name as a completion.  */
	  if CONSP (tem)
	    { canexclude = true; }

	  if !includeall && canexclude
	    /* We're not including all files and this file can be excluded.  */
	    { continue; }

	  if includeall && !canexclude
	    { /* If we have one non-excludable file, we want to exclude the
		 excludable files.  */
	      includeall = false;
	      /* Throw away any previous excludable match found.  */
	      bestmatch = Qnil ();
	      bestmatchsize = 0;
	      matchcount = 0;
	    }
	}

      let mut regexps: Lisp_Object;
      let table: Lisp_Object = if completion_ignore_case
				{ Vascii_canon_table } else { Qnil () };

      /* Ignore this element if it fails to match all the regexps.  */
      regexps = Vcompletion_regexp_list;
      while CONSP (regexps)
	{
	  if fast_string_match_internal (XCAR (regexps), name, table) < 0
	    { break; }
	  regexps = XCDR (regexps);
	}

      if CONSP (regexps)
	{ continue; }

      /* This is a possible completion */
      if directoryp
	/* This completion is a directory; make it end with '/'.  */
	{ name = Ffile_name_as_directory (name); }

      /* Test the predicate, if any.  */
      if !NILP (predicate) && NILP (calln! (predicate, name))
	{ continue; }

      /* Reject entries where the encoded strings match, but the
         decoded don't.  For example, "a" should not match "a-ring" on
         file systems that store decomposed characters. */
      if check_decoded && SCHARS (file) <= SCHARS (name)
	{
	  /* FIXME: This is a copy of the code below.  */
	  let compare: ptrdiff_t = SCHARS (file);
	  let cmp: Lisp_Object
	    = Fcompare_strings (name, zero, make_fixnum (compare),
				file, zero, make_fixnum (compare),
				if completion_ignore_case { Qt () } else { Qnil () });
	  if !BASE_EQ (cmp, Qt ())
	    { continue; }
	}

      /* Suitably record this match.  */

      matchcount += if matchcount <= 1 { 1 } else { 0 };

      if all_flag
	{ bestmatch = Fcons (name, bestmatch); }
      else if NILP (bestmatch)
	{
	  bestmatch = name;
	  bestmatchsize = SCHARS (name);
	}
      else
	{
	  /* FIXME: This is a copy of the code in Ftry_completion.  */
	  let compare: ptrdiff_t = min (bestmatchsize, SCHARS (name));
	  let mut cmp: Lisp_Object
	    = Fcompare_strings (bestmatch, zero, make_fixnum (compare),
				name, zero, make_fixnum (compare),
				if completion_ignore_case { Qt () } else { Qnil () });
	  let matchsize: ptrdiff_t = if BASE_EQ (cmp, Qt ())
	                        { compare } else { eabs! (XFIXNUM (cmp)) - 1 };

	  if completion_ignore_case
	    {
	      /* If this is an exact match except for case,
		 use it as the best match rather than one that is not
		 an exact match.  This way, we get the case pattern
		 of the actual match.  */
	      /* This tests that the current file is an exact match
		 but BESTMATCH is not (it is too long).  */
	      if (matchsize == SCHARS (name)
		   && (matchsize + directoryp as ptrdiff_t) < SCHARS (bestmatch))
		  ||
		  /* If there is no exact match ignoring case,
		     prefer a match that does not change the case
		     of the input.  */
		  /* If there is more than one exact match aside from
		     case, and one of them is exact including case,
		     prefer that one.  */
		  /* This == checks that, of current file and BESTMATCH,
		     either both or neither are exact.  */
		  (((matchsize == SCHARS (name))
		    ==
		    ((matchsize + directoryp as ptrdiff_t) == SCHARS (bestmatch)))
		   && { cmp = Fcompare_strings (name, zero,
					       make_fixnum (SCHARS (file)),
					       file, zero,
					       Qnil (),
					       Qnil ());
			BASE_EQ (Qt (), cmp) }
		   && { cmp = Fcompare_strings (bestmatch, zero,
					       make_fixnum (SCHARS (file)),
					       file, zero,
					       Qnil (),
					       Qnil ());
			! BASE_EQ (Qt (), cmp) })
		{ bestmatch = name; }
	    }
	  bestmatchsize = matchsize;

	  /* If the best completion so far is reduced to the string
	     we're trying to complete, then we already know there's no
	     other completion, so there's no point looking any further.  */
	  if matchsize <= SCHARS (file)
	      && !includeall /* A future match may allow includeall to 0.  */
	      /* If completion-ignore-case is non-nil, don't
		 short-circuit because we want to find the best
		 possible match *including* case differences.  */
	      && (!completion_ignore_case || matchsize == 0)
	      /* The return value depends on whether it's the sole match.  */
	      && matchcount > 1
	    { break; }

	}
    }

  /* This closes the directory.  */
  bestmatch = unbind_to (count, bestmatch);

  if all_flag || NILP (bestmatch)
    { return bestmatch; }
  /* Return t if the supplied string is an exact match (counting case);
     it does not require any change to be made.  */
  if matchcount == 1 && !NILP (Fequal (bestmatch, file))
    { return Qt (); }
  bestmatch = Fsubstring (bestmatch, make_fixnum (0),
			  make_fixnum (bestmatchsize));
  return bestmatch;
}

/* Compare exactly LEN chars of strings at S1 and S2,
   ignoring case if appropriate.
   Return -1 if strings match,
   else number of chars that match at the beginning.  */

unsafe fn scmp (mut s1: *const c_char, mut s2: *const c_char, len: ptrdiff_t) -> ptrdiff_t
{
  let mut l: ptrdiff_t = len;

  if completion_ignore_case
    {
      /* WARNING: This only works for pure ASCII strings, as we
	 compare bytes, not characters!  Use Fcompare_strings for
	 comparing non-ASCII strings case-insensitively.  */
      while l != 0
	     && (downcase (*s1 as c_uchar as c_int)
		 == downcase (*s2 as c_uchar as c_int))
	{ s1 = s1.add (1); s2 = s2.add (1); l -= 1; }
    }
  else
    {
      while l != 0 && *s1 == *s2
	{ s1 = s1.add (1); s2 = s2.add (1); l -= 1; }
    }
  if l == 0
    { return -1; }
  else
    { return len - l; }
}

/* Return true if in the directory FD the directory entry DP, whose
   string length is LEN, is that of a subdirectory that can be searched.  */
unsafe fn file_name_completion_dirp (fd: c_int, dp: *mut dirent, len: ptrdiff_t) -> bool
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let subdir_name: *mut c_char = SAFE_ALLOCA! (sa_avail, len + 2) as *mut c_char;
  memcpy (subdir_name as *mut c_void, (*dp).d_name.as_ptr () as *const c_void,
	  len as usize);
  strcpy (subdir_name.offset (len), c"/".as_ptr ());

  let dirp: bool = sys_faccessat (fd, subdir_name,
				  F_OK, AT_EACCESS) == 0;
  SAFE_FREE! (sa_count);
  return dirp;
}

unsafe fn stat_uname (st: *mut stat) -> *mut c_char
{
  /* C 926-927's `#ifdef WINDOWSNT` arm (`return st->st_uname;`) is dead on this target.  */
  let pw: *mut passwd = getpwuid ((*st).st_uid);

  if !pw.is_null ()
    { return (*pw).pw_name; }
  else
    { return std::ptr::null_mut (); }
}

unsafe fn stat_gname (st: *mut stat) -> *mut c_char
{
  /* C 941-942's `#ifdef WINDOWSNT` arm (`return st->st_gname;`) is dead on this target.  */
  let gr: *mut group = getgrgid ((*st).st_gid);

  if !gr.is_null ()
    { return (*gr).gr_name; }
  else
    { return std::ptr::null_mut (); }
}

DEFUN! ("file-attributes", Ffile_attributes, Sfile_attributes, 1, 2, 0,
       doc: "Return a list of attributes of file FILENAME.\nValue is nil if specified file does not exist.\n\nID-FORMAT specifies the preferred format of attributes uid and gid (see\nbelow) - valid values are `string' and `integer'.  The latter is the\ndefault, but we plan to change that, so you should specify a non-nil value\nfor ID-FORMAT if you use the returned uid or gid.\n\nTo access the elements returned, the following access functions are\nprovided: `file-attribute-type', `file-attribute-link-number',\n`file-attribute-user-id', `file-attribute-group-id',\n`file-attribute-access-time', `file-attribute-modification-time',\n`file-attribute-status-change-time', `file-attribute-size',\n`file-attribute-modes', `file-attribute-inode-number', and\n`file-attribute-device-number'.\n\nElements of the attribute list are:\n 0. t for directory, string (name linked to) for symbolic link, or nil.\n 1. Number of links to file.\n 2. File uid as a string or (if ID-FORMAT is `integer' or a string value\n  cannot be looked up) as an integer.\n 3. File gid, likewise.\n 4. Last access time, in the style of `current-time'.\n  (See a note below about access time on FAT-based filesystems.)\n 5. Last modification time, likewise.  This is the time of the last\n  change to the file's contents.\n 6. Last status change time, likewise.  This is the time of last change\n  to the file's attributes: owner and group, access mode bits, etc.\n 7. Size in bytes, as an integer.\n 8. File modes, as a string of ten letters or dashes as in ls -l.\n 9. An unspecified value, present only for backward compatibility.\n10. inode number, as a nonnegative integer.\n11. Filesystem device identifier, as an integer or a cons cell of integers.\n\nLarge integers are bignums, so `eq' might not work on them.\nOn most filesystems, the combination of the inode and the device\nidentifier uniquely identifies the file.  This unique file identification\nis provided by the access function `file-attribute-file-identifier'.\n\nOn MS-Windows, performance depends on `w32-get-true-file-attributes',\nwhich see.\n\nOn some FAT-based filesystems, only the date of last access is recorded,\nso last access time will always be midnight of that day.",
       (filename: Lisp_Object, id_format: Lisp_Object),
       {
  /* C's `filename = internal_condition_case_2 (...)` below rebinds the argument.  */
  let mut filename = filename;
  let encoded: Lisp_Object;
  let handler: Lisp_Object;

  filename = internal_condition_case_2 (Some (Fexpand_file_name), filename, Qnil (),
					Qt (), Some (Fidentity));
  if !STRINGP (filename)
    { return Qnil (); }

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qfile_attributes ());
  if !NILP (handler)
    { /* Only pass the extra arg if it is used to help backward
	 compatibility with old file name handlers which do not
	 implement the new arg.  --Stef */
      if NILP (id_format)
	{ return calln! (handler, Qfile_attributes (), filename); }
      else
	{ return calln! (handler, Qfile_attributes (), filename, id_format); }
    }

  encoded = ENCODE_FILE (filename);
  return file_attributes (AT_FDCWD, SSDATA (encoded), Qnil (), filename,
			  id_format);
});

unsafe fn file_attributes (fd: c_int, name: *const c_char,
			   dirname: Lisp_Object, filename: Lisp_Object,
			   id_format: Lisp_Object) -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut s: stat = std::mem::zeroed ();

  /* An array to hold the mode string generated by filemodestring,
     including its terminating space and null byte.  */
  let mut modes: [c_char; 12] = [0; 12];   /* C's `sizeof "-rwxr-xr-x "` */

  let mut uname: *mut c_char = std::ptr::null_mut ();
  let mut gname: *mut c_char = std::ptr::null_mut ();

  let mut err: c_int = EINVAL;

  /* C 1042-1065's `#if defined O_PATH && !defined HAVE_CYGWIN_O_PATH_BUG &&
     !(defined HAVE_ANDROID && !defined ANDROID_STUBIFY)` block (`emacs_openat`, `sys_fstat`,
     `close_file_unwind`, `EBADF`) is dead on this target: O_PATH is undefined.  */

  if err == EINVAL
    {
      /* C 1070-1076's `#ifdef WINDOWSNT` arm (`w32_stat_get_owner_group = 1;`) is dead.  */
      err = if emacs_fstatat (fd, name, &raw mut s as *mut c_void,
			      AT_SYMLINK_NOFOLLOW) == 0
	    { 0 } else { *errno () };
      /* C 1078-1080's `#ifdef WINDOWSNT` arm (`w32_stat_get_owner_group = 0;`) is dead.  */
    }

  if err != 0
    { return unbind_to (count, file_attribute_errno (filename, err)); }

  let file_type: Lisp_Object;
  if S_ISLNK! (s.st_mode)
    {
      /* On systems lacking O_PATH support there is a race if the
	 symlink is replaced between the call to fstatat and the call
	 to emacs_readlinkat.  Detect this race unless the replacement
	 is also a symlink.  */
      file_type = check_emacs_readlinkat (fd, filename, name);
      if NILP (file_type)
	{ return unbind_to (count, Qnil ()); }
    }
  else
    { file_type = if S_ISDIR! (s.st_mode) { Qt () } else { Qnil () }; }

  unbind_to (count, Qnil ());

  if !(NILP (id_format) || EQ (id_format, Qinteger ()))
    {
      uname = stat_uname (&raw mut s);
      gname = stat_gname (&raw mut s);
    }

  filemodestring (&s, modes.as_mut_ptr ());

  /* C's `INT_TO_INTEGER (expr)` is `(EXPR_SIGNED (expr) ? make_int (expr) : make_uint
     (expr))` (lisp.h 3052-3053), and the lisp.rs `INT_TO_INTEGER` is the signed arm: the
     unsigned expressions below (nlink_t, uid_t, gid_t, ino_t) keep C's `make_uint`
     spelling.  Recorded in docs/ports/dired.md.  */
  return CALLN! (Flist,
		file_type,
		make_fixnum (s.st_nlink as isize),
		if !uname.is_null ()
		 { DECODE_SYSTEM (build_unibyte_string (uname)) }
		 else { make_uint (s.st_uid as uintmax_t) },
		if !gname.is_null ()
		 { DECODE_SYSTEM (build_unibyte_string (gname)) }
		 else { make_uint (s.st_gid as uintmax_t) },
		make_lisp_time (get_stat_atime (&s)),
		make_lisp_time (get_stat_mtime (&s)),
		make_lisp_time (get_stat_ctime (&s)),

		/* If the file size is a 4-byte type, assume that
		   files of sizes in the 2-4 GiB range wrap around to
		   negative values, as this is a common bug on older
		   32-bit platforms.  */
		INT_TO_INTEGER (if std::mem::size_of_val (&s.st_size) == 4
			    { (s.st_size & 0xffffffff) as isize }
			    else { s.st_size as isize }),

		make_string (modes.as_ptr (), 10),
		Qt (),
		make_uint (s.st_ino),
		INT_TO_INTEGER (s.st_dev as isize));
}

DEFUN! ("file-attributes-lessp", Ffile_attributes_lessp,
       Sfile_attributes_lessp, 2, 2, 0,
       doc: "Return t if first arg file attributes list is less than second.\nComparison is in lexicographic order and case is significant.",
       (f1: Lisp_Object, f2: Lisp_Object),
       {
  return Fstring_lessp (Fcar (f1), Fcar (f2));
});

DEFUN! ("system-users", Fsystem_users, Ssystem_users, 0, 0, 0,
       doc: "Return a list of user names currently registered in the system.\nIf we don't know how to determine that on this platform, just\nreturn a list with one element, taken from `user-real-login-name'.",
       (),
       {
  let mut users: Lisp_Object = Qnil ();
  /* HAVE_GETPWENT and HAVE_ENDPWENT are 1 on the frozen target, so C 1153's arm is live.  */
  let mut pw: *mut passwd;

  while { pw = getpwent (); !pw.is_null () }
    { users = Fcons (DECODE_SYSTEM (build_string ((*pw).pw_name)), users); }

  endpwent ();

  if NILP (users)
    /* At least current user is always known. */
    { users = list1 (Vuser_real_login_name); }
  return users;
});

DEFUN! ("system-groups", Fsystem_groups, Ssystem_groups, 0, 0, 0,
       doc: "Return a list of user group names currently registered in the system.\nThe value may be nil if not supported on this platform.",
       (),
       {
  let mut groups: Lisp_Object = Qnil ();
  /* HAVE_GETGRENT and HAVE_ENDGRENT are 1 on the frozen target, so C 1173's arm is live.  */
  let mut gr: *mut group;

  while { gr = getgrent (); !gr.is_null () }
    { groups = Fcons (DECODE_SYSTEM (build_string ((*gr).gr_name)), groups); }

  endgrent ();

  return groups;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_dired ()
{
  DEFSYM! (Qdirectory_files, c"directory-files");
  DEFSYM! (Qdirectory_files_and_attributes, c"directory-files-and-attributes");
  DEFSYM! (Qfile_name_completion, c"file-name-completion");
  DEFSYM! (Qfile_name_all_completions, c"file-name-all-completions");
  DEFSYM! (Qfile_attributes, c"file-attributes");
  DEFSYM! (Qfile_attributes_lessp, c"file-attributes-lessp");
  DEFSYM! (Qdefault_directory, c"default-directory");
  DEFSYM! (Qdecomposed_characters, c"decomposed-characters");

  defsubr (&raw mut Sdirectory_files);
  defsubr (&raw mut Sdirectory_files_and_attributes);
  defsubr (&raw mut Sfile_name_completion);
  defsubr (&raw mut Sfile_name_all_completions);
  defsubr (&raw mut Sfile_attributes);
  defsubr (&raw mut Sfile_attributes_lessp);
  defsubr (&raw mut Ssystem_users);
  defsubr (&raw mut Ssystem_groups);

  DEFVAR_LISP! (c"completion-ignored-extensions", Vcompletion_ignored_extensions,
	       "Completion ignores file names ending in any string in this list.\nIt does not ignore them if all possible completions end in one of\nthese strings or when displaying a list of completions.\nIt ignores directory names if they match any string in this list which\nends in a slash.");
  Vcompletion_ignored_extensions = Qnil ();
}

/* C's `DEFVAR_LISP` variable: defined here and shadowing the `globals.rs` declaration
   (P-21, the filelock.c/undo.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vcompletion_ignored_extensions: Lisp_Object = lisp_h_Qnil ();
