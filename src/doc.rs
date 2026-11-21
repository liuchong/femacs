// femacs: port of src/doc.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, global, statement), same control flow.  The six subrs use `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors doc.c 21-31 - config.h, errno.h, sys/types.h, sys/file.h,
// fcntl.h, unistd.h, c-ctype.h, lisp.h, character.h, coding.h, buffer.h, disptab.h,
// intervals.h, keymap.h - with the mirrors those pull in.  `<sys/file.h>` has no mirror of
// its own: it is included for `lseek`'s declaration on USG systems and every name doc.c uses
// from it (`O_RDONLY`, `lseek`) comes from fcntl/unistd; `intervals.h` declares nothing doc.c
// calls (doc.c only reads the DOC file's bytes), so no mirror is needed for it either.
//
// The two configurables are `#if !defined HAVE_ANDROID || ...` (C 42-70) and the block's
// `USE_ANDROID_ASSETS`, plus `HAVE_NATIVE_COMP`/`HAVE_MODULES` in `internal-subr-documentation`.
// HAVE_ANDROID/HAVE_MODULES are undefined and HAVE_NATIVE_COMP is false in the frozen
// configuration, so each non-frozen arm is kept as gated text (P-51, the status 3.9 convention
// of naming the feature after the non-frozen side).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;


const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/errno.rs imported via common
// [U1] ../lib/include/fcntl.in.rs imported via common
// [U1] ../lib/include/unistd.in.rs imported via common
// [U1] ../lib/include/sys_types.in.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/dynlib.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/disptab.rs imported via common
include!("include/buildobj.rs");
// [U1] include/keymap.rs imported via common

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vdoc_file_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vbuild_files: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtext_quoting_style: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut documentation_dynamic_reload: bool = false;
#[unsafe(no_mangle)]
pub static mut text_quoting_flag: bool = false;

// C 42-49: `doc_fd`/`doc_fd_p`/`doc_open`/`doc_read_quit`/`doc_lseek` for the arm
// `!defined HAVE_ANDROID || defined ANDROID_STUBIFY || __ANDROID_API__ < 9`.  The five are
// object-like or function-like macros over the file descriptor, exactly as C writes them; the
// port keeps the same names as `macro_rules!` (PORTING.md 2.1/P-37).  C 51-70's `#else` arm
// redefines the same five over `struct android_fd_or_asset`, so the condition that separates
// them is C's own (`USE_ANDROID_ASSETS`, derived in `xtask` from
// `HAVE_ANDROID && !ANDROID_STUBIFY && __ANDROID_API__ >= 9`) and the two arms cannot be live
// together.
#[cfg (not (feature = "use_android_assets"))]
macro_rules! doc_fd { () => { c_int }; }
#[cfg (not (feature = "use_android_assets"))]
macro_rules! doc_fd_p { ($fd:expr) => { ($fd) >= 0 }; }
#[cfg (not (feature = "use_android_assets"))]
macro_rules! doc_open { ($($args:tt)*) => { emacs_open ($($args)*) }; }
#[cfg (not (feature = "use_android_assets"))]
macro_rules! doc_read_quit { ($($args:tt)*) => { emacs_read_quit ($($args)*) }; }
#[cfg (not (feature = "use_android_assets"))]
macro_rules! doc_lseek { ($($args:tt)*) => { lseek ($($args)*) }; }

/* C 51-70's `#else` arm: the same five names over `android.h`'s
   `struct android_fd_or_asset`, which no mirror carries, so it is declared here with clang's
   frozen layout - the same shape `lread.rs` uses for the same type (C 88's include).  */
#[cfg (feature = "use_android_assets")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct android_fd_or_asset { pub fd: c_int, pub asset: *mut c_void }
#[cfg (feature = "use_android_assets")]
const _: () = core::assert! (std::mem::size_of::<android_fd_or_asset> () == 16);
#[cfg (feature = "use_android_assets")]
const _: () = core::assert! (std::mem::align_of::<android_fd_or_asset> () == 8);
#[cfg (feature = "use_android_assets")]
const _: () = core::assert! (std::mem::offset_of! (android_fd_or_asset, fd) == 0);
#[cfg (feature = "use_android_assets")]
const _: () = core::assert! (std::mem::offset_of! (android_fd_or_asset, asset) == 8);

/* C 61-65: the four `#define`s of the arm (`doc_fd struct android_fd_or_asset`,
   `doc_fd_p(fd) ((fd).asset != (void *) -1)`, `doc_open android_open_asset`,
   `doc_read_quit android_asset_read_quit`, `doc_lseek android_asset_lseek`).  C's
   `doc_read_quit` passes a `ptrdiff_t` where `android_asset_read_quit` takes a `size_t`; C
   converts implicitly, so the macro casts, the way `lread_read_quit` does.  */
#[cfg (feature = "use_android_assets")]
macro_rules! doc_fd { () => { android_fd_or_asset }; }
#[cfg (feature = "use_android_assets")]
macro_rules! doc_fd_p { ($fd:expr) => { ($fd).asset != (usize::MAX as *mut c_void) }; }
#[cfg (feature = "use_android_assets")]
macro_rules! doc_open { ($($args:tt)*) => { android_open_asset ($($args)*) }; }
#[cfg (feature = "use_android_assets")]
macro_rules! doc_read_quit { ($fd:expr, $buf:expr, $n:expr) => { android_asset_read_quit ($fd, $buf, ($n) as usize) }; }
#[cfg (feature = "use_android_assets")]
macro_rules! doc_lseek { ($($args:tt)*) => { android_asset_lseek ($($args)*) }; }

/* C 66's `#define USE_ANDROID_ASSETS` closes the arm; the file's uses are gated on that name
   rather than re-spelling the condition.  */
#[cfg (feature = "use_android_assets")]
macro_rules! USE_ANDROID_ASSETS { () => {} }

#[cfg (feature = "use_android_assets")]
unsafe extern "C" {
  /// C: android.h: `void android_close_asset (struct android_fd_or_asset);`
  fn android_close_asset (fd: android_fd_or_asset);
  /// C: android.h: `struct android_fd_or_asset android_open_asset (const char *, int, int);`
  fn android_open_asset (path: *const c_char, flags: c_int, mode: c_int) -> android_fd_or_asset;
  /// C: android.h: `ssize_t android_asset_read_quit (struct android_fd_or_asset, void *,
  /// size_t);`
  fn android_asset_read_quit (fd: android_fd_or_asset, buf: *mut c_void, n: usize) -> isize;
  /// C: android.h: `off_t android_asset_lseek (struct android_fd_or_asset, off_t, int);`
  fn android_asset_lseek (fd: android_fd_or_asset, off: off_t, whence: c_int) -> off_t;
}

/* Buffer used for reading from documentation file.  */
static mut get_doc_string_buffer: *mut c_char = std::ptr::null_mut ();
static mut get_doc_string_buffer_size: ptrdiff_t = 0;

static sibling_etc: [u8; 8] = *b"../etc/\0";

/* C 78-90: `#ifdef USE_ANDROID_ASSETS` - like `close_file_unwind`, but PTR points at an
   Android file descriptor instead of a system one.  `record_unwind_protect_ptr` takes a C
   function pointer, so this is an `unsafe extern "C" fn` (the same reason the other unwind
   helpers are).  */
#[cfg (feature = "use_android_assets")]
unsafe extern "C" fn close_file_unwind_android_fd (ptr: *mut c_void)
{
  let mut fd: *mut android_fd_or_asset;

  fd = ptr as *mut android_fd_or_asset;
  android_close_asset (*fd);
}

/* Extract a doc string from a file.  FILEPOS says where to get it.
   If it is an integer, use that position in the standard DOC file.
   If it is (FILE . INTEGER), use FILE as the file name
   and INTEGER as the position in that file.

   If the location does not point to the beginning of a docstring
   (e.g. because the file has been modified and the location is stale),
   return nil.

   If UNIBYTE, always make a unibyte string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_doc_string (filepos: Lisp_Object, unibyte: bool) -> Lisp_Object
{
  // C declares these as uninitialized pointers and fills them below; the port gives the C
  // locals that are "written through a pointer or before their first read" the zero
  // initializer the ledger prescribes (PORTING.md 1.5).
  let mut from: *mut c_char = std::ptr::null_mut ();
  let mut to: *mut c_char = std::ptr::null_mut ();
  let mut name: *mut c_char = std::ptr::null_mut ();
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut p1: *mut c_char = std::ptr::null_mut ();
  let mut file: Lisp_Object;
  let mut pos: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut dir: Lisp_Object;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  if FIXNUMP (filepos)
    {
      file = Vdoc_file_name;
      dir = Vdoc_directory;
      pos = filepos;
    }
  else if CONSP (filepos)
    {
      file = XCAR (filepos);
      dir = Fsymbol_value (Qlisp_directory ());
      pos = XCDR (filepos);
    }
  else
    { return Qnil (); }

  /* We used to emit negative positions for 'user variables' (whose doc
     strings started with an asterisk); take the absolute value for
     compatibility with bytecode from Emacs <29.  */
  let position: EMACS_INT = eabs! (XFIXNUM (pos));

  if !STRINGP (dir)
    { return Qnil (); }

  if !STRINGP (file)
    { return Qnil (); }

  /* Put the file name in NAME as a C string.
     If it is relative, combine it with Vdoc_directory.  */

  let tem: Lisp_Object = Ffile_name_absolute_p (file);
  file = ENCODE_FILE (file);
  let docdir: Lisp_Object
    = if NILP (tem) { ENCODE_FILE (dir) } else { empty_unibyte_string };
  let mut docdir_sizemax: ptrdiff_t = SBYTES (docdir) + 1;
  if will_dump_p ()
    { docdir_sizemax = max (docdir_sizemax, std::mem::size_of_val (&sibling_etc) as isize); }
  name = SAFE_ALLOCA! (sa_avail, docdir_sizemax + SBYTES (file)) as *mut c_char;
  lispstpcpy (lispstpcpy (name, docdir), file);

  let mut fd: doc_fd! () = doc_open! (name, O_RDONLY, 0);
  if !doc_fd_p! (fd)
    {
      if will_dump_p ()
	{
	  /* Preparing to dump; DOC file is probably not installed.
	     So check in ../etc.  */
	  lispstpcpy (stpcpy (name, sibling_etc.as_ptr () as *const c_char), file);

	  fd = doc_open! (name, O_RDONLY, 0);
	}
      if !doc_fd_p! (fd)
	{
	  if *errno () != ENOENT && *errno () != ENOTDIR
	    { report_file_error (b"Read error on documentation file\0".as_ptr () as *const c_char, file); }

	  SAFE_FREE! (sa_count, sa_mark);
	  AUTO_STRING! (cannot_open, b"Cannot open doc string file \"\0".as_ptr () as *const c_char);
	  AUTO_STRING! (quote_nl, b"\"\n\0".as_ptr () as *const c_char);
	  return concat3 (cannot_open, file, quote_nl);
	}
    }
  // C 148-153: `#ifndef USE_ANDROID_ASSETS` / `#else` - the descriptor is an `int` in the first
  // arm and a `struct android_fd_or_asset` in the second, so each arm records its own unwind.
  #[cfg (not (feature = "use_android_assets"))]
  record_unwind_protect_int (Some (close_file_unwind), fd);
  #[cfg (feature = "use_android_assets")]
  { record_unwind_protect_ptr (Some (close_file_unwind_android_fd), &raw mut fd as *mut c_void); }

  /* Seek only to beginning of disk block.  */
  /* Make sure we read at least 1024 bytes before `position'
     so we can check the leading text for consistency.  */
  // C's `min (position, max (1024, position % (8 * 1024)))` with an `int` destination; the
  // result is narrowed the way C narrows it.
  let offset: c_int = min (position, max (1024, position % (8 * 1024))) as c_int;
  if off_t::MAX < position as off_t
      || doc_lseek! (fd, (position - offset as ptrdiff_t) as off_t, 0) < 0
    { error (b"Position %ld out of range in doc string file \"%s\"\0".as_ptr () as *const c_char,
	     position, name); }

  /* Read the doc string into get_doc_string_buffer.
     P points beyond the data just read.  */

  p = get_doc_string_buffer;
  loop
    {
      let mut space_left: ptrdiff_t = (get_doc_string_buffer_size - 1
				       - p.offset_from (get_doc_string_buffer));

      /* Allocate or grow the buffer if we need to.  */
      if space_left <= 0
	{
	  let in_buffer: ptrdiff_t = p.offset_from (get_doc_string_buffer);
	  get_doc_string_buffer
	    = xpalloc (get_doc_string_buffer as *mut c_void, &raw mut get_doc_string_buffer_size,
		       16 * 1024, -1, 1) as *mut c_char;
	  p = get_doc_string_buffer.offset (in_buffer);
	  space_left = (get_doc_string_buffer_size - 1
			- p.offset_from (get_doc_string_buffer));
	}

      /* Read a disk block at a time.
         If we read the same block last time, maybe skip this?  */
      if space_left > 1024 * 8
	{ space_left = 1024 * 8; }
      let nread: c_int = doc_read_quit! (fd, p as *mut c_void, space_left) as c_int;
      if nread < 0
	{ report_file_error (b"Read error on documentation file\0".as_ptr () as *const c_char,
			     file); }
      *p.offset (nread as isize) = 0;
      if nread == 0
	{ break; }
      if p == get_doc_string_buffer
	{ p1 = strchr (p.offset (offset as isize), 31); }
      else
	{ p1 = strchr (p, 31); }
      if !p1.is_null ()
	{
	  *p1 = 0;
	  p = p1;
	  break;
	}
      p = p.offset (nread as isize);
    }
  SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil (), sa_mark);

  /* Sanity checking.  */
  if CONSP (filepos)
    {
      let mut test: c_int = 1;
      if *get_doc_string_buffer.offset (offset as isize - test as isize) != 31
	{
	  // C's `get_doc_string_buffer[offset - test++]`: the index uses TEST's old value.
	  let idx: isize = offset as isize - test as isize;
	  test += 1;
	  if *get_doc_string_buffer.offset (idx) != b' ' as c_char
	    { return Qnil (); }
	  while *get_doc_string_buffer.offset (offset as isize - test as isize) >= b'0' as c_char
		&& *get_doc_string_buffer.offset (offset as isize - test as isize) <= b'9' as c_char
	    { test += 1; }
	  let idx: isize = offset as isize - test as isize;
	  test += 1;
	  if *get_doc_string_buffer.offset (idx) != b'@' as c_char
	      || *get_doc_string_buffer.offset (offset as isize - test as isize) != b'#' as c_char
	    { return Qnil (); }
	}
    }
  else
    {
      let mut test: c_int = 1;
      let idx: isize = offset as isize - test as isize;
      test += 1;
      if *get_doc_string_buffer.offset (idx) != b'\n' as c_char
	{ return Qnil (); }
      while *get_doc_string_buffer.offset (offset as isize - test as isize) > b' ' as c_char
	{ test += 1; }
      if *get_doc_string_buffer.offset (offset as isize - test as isize) != 31
	{ return Qnil (); }
    }

  /* Scan the text and perform quoting with ^A (char code 1).
     ^A^A becomes ^A, ^A0 becomes a null char, and ^A_ becomes a ^_.  */
  from = get_doc_string_buffer.offset (offset as isize);
  to = get_doc_string_buffer.offset (offset as isize);
  while from != p
    {
      if *from == 1
	{
	  from = from.offset (1);
	  let c: c_int = *from as c_int;
	  from = from.offset (1);
	  if c == 1
	    { *to = c as c_char; to = to.offset (1); }
	  else if c == b'0' as c_int
	    { *to = 0; to = to.offset (1); }
	  else if c == b'_' as c_int
	    { *to = 31; to = to.offset (1); }
	  else
	    {
	      let uc: u8 = c as u8;
	      /* The literal is written the way C writes it - a backslash-newline continuation with
		 the message starting at column 0 - because the two languages remove the same bytes
		 (the newline and, for Rust, the next line's indentation, of which there is none).  */
	      error (b"\
Invalid data in documentation file -- %c followed by code %03o\0".as_ptr () as *const c_char,
		     1 as c_int, uc as c_int);
	    }
	}
      else
	{ *to = *from; to = to.offset (1); from = from.offset (1); }
    }

  if unibyte
    { return make_unibyte_string (get_doc_string_buffer.offset (offset as isize),
				  to.offset_from (get_doc_string_buffer.offset (offset as isize))); }
  else
    {
      /* The data determines whether the string is multibyte.  */
      let nchars: ptrdiff_t
	= multibyte_chars_in_text (get_doc_string_buffer.offset (offset as isize) as *const u8,
				   to.offset_from (get_doc_string_buffer.offset (offset as isize)));
      return make_string_from_bytes (get_doc_string_buffer.offset (offset as isize),
				     nchars,
				     to.offset_from (get_doc_string_buffer.offset (offset as isize)));
    }
}

unsafe fn reread_doc_file (file: Lisp_Object)
{
  if NILP (file)
    { Fsnarf_documentation (Vdoc_file_name); }
  else
    { save_match_data_load (file, Qt (), Qt (), Qt (), Qnil ()); }
}

DEFUN! ("documentation-stringp", Fdocumentation_stringp, Sdocumentation_stringp,
       1, 1, 0,
       doc: "Return non-nil if OBJECT is a well-formed docstring object.\nOBJECT can be either a string or a reference if it's kept externally.",
       (object: Lisp_Object),
       {
  return if STRINGP (object)
          || FIXNUMP (object)   /* Reference to DOC.  */
          || (CONSP (object)    /* Reference to .elc.  */
              && STRINGP (XCAR (object))
              && FIXNUMP (XCDR (object)))
          { Qt () } else { Qnil () };
});

DEFUN! ("documentation", Fdocumentation, Sdocumentation, 1, 2, 0,
       doc: "Return the documentation string of FUNCTION.\nUnless a non-nil second argument RAW is given, the\nstring is passed through `substitute-command-keys'.",
       (function: Lisp_Object, raw: Lisp_Object),
       {
  let mut doc: Lisp_Object;
  let mut try_reload: bool = documentation_dynamic_reload;

 'retry: loop {

  doc = Qnil ();

  if SYMBOLP (function)
    {
      let tem: Lisp_Object = Fget (function, Qfunction_documentation ());
      if !NILP (tem)
	{ return Fdocumentation_property (function, Qfunction_documentation (), raw); }
    }

  let mut fun: Lisp_Object = Findirect_function (function, Qnil ());
  if NILP (fun)
    { xsignal1 (Qvoid_function (), function); }
  if CONSP (fun) && EQ (XCAR (fun), Qmacro ())
    { fun = XCDR (fun); }
  doc = calln! (Qfunction_documentation (), fun);

  /* If DOC is 0, it's typically because of a dumped file missing
     from the DOC file (bug in src/Makefile.in).  */
  if BASE_EQ (doc, make_fixnum (0))
    { doc = Qnil (); }
  if FIXNUMP (doc) || (CONSP (doc) && FIXNUMP (XCDR (doc)))
    {
      let tem: Lisp_Object = get_doc_string (doc, false);
      if NILP (tem) && try_reload
	{
	  /* The file is newer, we need to reset the pointers.  */
	  reread_doc_file (Fcar_safe (doc));
	  try_reload = false;
	  continue 'retry;
	}
      doc = tem;
    }

  if NILP (raw)
    { doc = calln! (Qsubstitute_command_keys (), doc); }
  return doc;
 }
});

DEFUN! ("internal-subr-documentation", Fsubr_documentation, Ssubr_documentation, 1, 1, 0,
       doc: "Return the raw documentation info of a C primitive.",
       (function: Lisp_Object),
       {
  // C 387-393: the `#ifdef HAVE_NATIVE_COMP` arm.  HAVE_NATIVE_COMP is false in the frozen
  // configuration, so the arm is kept as gated text (P-51); `native_function_doc` is comp.h
  // 78's declaration, which no mirror carries, so it is declared here (the vm-limit precedent).
  #[cfg (feature = "have_native_comp")]
  {
    unsafe extern "C" {
      /// C: Lisp_Object native_function_doc (Lisp_Object)
      pub fn native_function_doc (function: Lisp_Object) -> Lisp_Object;
    }
    if !NILP (Fnative_comp_function_p (function))
      { return native_function_doc (function); }
  }
  // C 394-397: the `#ifdef HAVE_MODULES` arm, dead on the frozen target and live for the Android
  // configuration (its `HAVE_MODULES` is 1).
  #[cfg (feature = "have_modules")]
  {
    /* `MODULE_FUNCTIONP` and `XMODULE_FUNCTION` are lisp.h *macros* (C 4860-4870), not C
       functions: the port has them as the mirror's own Rust functions and this unit reaches them
       through `include/lisp.rs`.  Declaring them `extern "C"` here made the Android link import
       two symbols nothing defines (status 4.713).  */
    if MODULE_FUNCTIONP (function)
      /* C 1344: C's `XMODULE_FUNCTION` yields `struct Lisp_Module_Function *` and lisp.h
	 4897's declaration takes that pointer const-qualified - C converts implicitly, Rust's
	 mirror spells the parameter `*const c_void` (P-72 rule 3), so the conversion is here.  */
      { return module_function_documentation (XMODULE_FUNCTION (function) as *const c_void); }
  }
  if SUBRP (function)
    { return make_fixnum ((*XSUBR (function)).doc as EMACS_INT); }
  else
    { return Qt (); }
});

DEFUN! ("documentation-property", Fdocumentation_property,
       Sdocumentation_property, 2, 3, 0,
       doc: "Return the documentation string that is SYMBOL's PROP property.\nThird argument RAW omitted or nil means pass the result through\n`substitute-command-keys' if it is a string.\n\nThis differs from `get' in that it can refer to strings stored in the\n`etc/DOC' file; and that it evaluates documentation properties that\naren't strings.",
       (symbol: Lisp_Object, prop: Lisp_Object, raw: Lisp_Object),
       {
  let mut try_reload: bool = documentation_dynamic_reload;
  let mut tem: Lisp_Object;

 'retry: loop {

  tem = Fget (symbol, prop);

  /* If we don't have any documentation for this symbol (and we're asking for
     the variable documentation), try to see whether it's an indirect variable
     and get the documentation from there instead. */
  if EQ (prop, Qvariable_documentation ())
      && NILP (tem)
    {
      let indirect: Lisp_Object = Findirect_variable (symbol);
      if !NILP (indirect)
	{ tem = Fget (indirect, prop); }
    }

  if BASE_EQ (tem, make_fixnum (0))
    { tem = Qnil (); }

  /* See if we want to look for the string in the DOC file. */
  if FIXNUMP (tem) || (CONSP (tem) && FIXNUMP (XCDR (tem)))
    {
      let doc: Lisp_Object = tem;
      tem = get_doc_string (tem, false);
      if NILP (tem) && try_reload
	{
	  /* The file is newer, we need to reset the pointers.  */
	  reread_doc_file (Fcar_safe (doc));
	  try_reload = false;
	  continue 'retry;
	}
    }
  else if !STRINGP (tem)
    /* Feval protects its argument.  */
    { tem = Feval (tem, Qnil ()); }

  if NILP (raw) && STRINGP (tem)
    { tem = calln! (Qsubstitute_command_keys (), tem); }
  return tem;
 }
});

/* Scanning the DOC files and placing docstring offsets into functions.  */

unsafe fn store_function_docstring (obj: Lisp_Object, offset: EMACS_INT)
{
  /* Don't use indirect_function here, or defaliases will apply their
     docstrings to the base functions (Bug#2603).  */
  let mut fun: Lisp_Object = if SYMBOLP (obj) { (*XSYMBOL (obj)).u.s.function } else { obj };

  /* The type determines where the docstring is stored.  */

  /* If it's a lisp form, stick it in the form.  */
  if CONSP (fun) && EQ (XCAR (fun), Qmacro ())
    { fun = XCDR (fun); }
  /* Lisp_Subrs have a slot for it.  */
  if SUBRP (fun)
    {
      (*XSUBR (fun)).doc = offset as ptrdiff_t;
      eassert! ((*XSUBR (fun)).doc >= 0);
    }
  else if CLOSUREP (fun)
    {
      /* This bytecode object must have a slot for the docstring, since
	 we've found a docstring for it.  */
      if PVSIZE (fun) > CLOSURE_DOC_STRING as isize
	  /* Don't overwrite a non-docstring value placed there, such as
             the symbols used for Oclosures.  */
	  && VALID_DOCSTRING_P (AREF (fun, CLOSURE_DOC_STRING as isize))
	{ ASET (fun, CLOSURE_DOC_STRING as isize, make_fixnum (offset)); }
      else
	{
	  AUTO_STRING! (format, b"No doc string slot for compiled: %S\0".as_ptr () as *const c_char);
	  CALLN! (Fmessage, format, obj);
	}
    }
  else
    {
      AUTO_STRING! (format, b"Ignoring DOC string on non-compiled non-subr: %S\0".as_ptr () as *const c_char);
      CALLN! (Fmessage, format, obj);
    }
}


DEFUN! ("Snarf-documentation", Fsnarf_documentation, Ssnarf_documentation,
       1, 1, 0,
       doc: "Used during Emacs initialization to scan the `etc/DOC...' file.\nThis searches the `etc/DOC...' file for doc strings and\nrecords them in function and variable definitions.\nThe function takes one argument, FILENAME, a string;\nit specifies the file name (without a directory) of the DOC file.\nThat file is found in `../etc' now; later, when the dumped Emacs is run,\nthe same file name is found in the `doc-directory'.",
       (filename: Lisp_Object),
       {
  let mut buf: [c_char; 1024 + 1] = [0; 1024 + 1];
  let mut sym: Lisp_Object;
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut name: *mut c_char = std::ptr::null_mut ();
  let mut dirname: *const c_char = std::ptr::null ();
  let mut dirlen: ptrdiff_t;
  /* Preloaded defcustoms using custom-initialize-delay are added to
     this list, but kept unbound.  See https://debbugs.gnu.org/11565  */
  let mut delayed_init: Lisp_Object
    = find_symbol_value (Qcustom_delayed_init_variables ());

  if !CONSP (delayed_init) { delayed_init = Qnil (); }

  CHECK_STRING (filename);

  if will_dump_p ()
    {
      dirname = sibling_etc.as_ptr () as *const c_char;
      dirlen = std::mem::size_of_val (&sibling_etc) as isize - 1;
    }
  else
    {
      CHECK_STRING (Vdoc_directory);
      dirname = SSDATA (Vdoc_directory);
      dirlen = SBYTES (Vdoc_directory);
    }

  let count: specpdl_ref = SPECPDL_INDEX ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  name = SAFE_ALLOCA! (sa_avail, dirlen + SBYTES (filename) + 1) as *mut c_char;
  lispstpcpy (stpcpy (name, dirname), filename); 	/*** Add this line ***/

  /* Vbuild_files is nil when temacs is run, and non-nil after that.  */
  if NILP (Vbuild_files)
    {
      // C's `static char const *const buildobj[] = { #include "buildobj.h" };` - the object
      // list is the generated mirror `buildobj.rs` (PORTING.md 0.1, the config.rs precedent).
      let mut i: c_int = countof! (buildobj) as c_int;
      while 0 <= { i -= 1; i }
	{ Vbuild_files = Fcons (build_string (buildobj[i as usize].as_ptr ()), Vbuild_files); }
    }

  let mut fd: doc_fd! () = doc_open! (name, O_RDONLY, 0);
  if !doc_fd_p! (fd)
    {
      let open_errno: c_int = *errno ();
      report_file_errno (b"Opening doc string file\0".as_ptr () as *const c_char,
			 build_string (name), open_errno);
    }
  // C 553-558: the same two arms as C 148-153 (`#ifndef USE_ANDROID_ASSETS` / `#else`).
  #[cfg (not (feature = "use_android_assets"))]
  record_unwind_protect_int (Some (close_file_unwind), fd);
  #[cfg (feature = "use_android_assets")]
  { record_unwind_protect_ptr (Some (close_file_unwind_android_fd), &raw mut fd as *mut c_void); }
  Vdoc_file_name = filename;
  let mut filled: c_int = 0;
  let mut pos: EMACS_INT = 0;
  loop
    {
      if filled < 512
	{ filled += doc_read_quit! (fd, buf.as_mut_ptr ().offset (filled as isize) as *mut c_void,
				    buf.len () as ptrdiff_t - 1 - filled as ptrdiff_t) as c_int; }
      if filled == 0
	{ break; }

      *buf.as_mut_ptr ().offset (filled as isize) = 0;
      let mut end: *mut c_char = buf.as_mut_ptr ()
	.offset (if filled < 512 { filled } else { filled - 128 } as isize);
      p = memchr (buf.as_ptr () as *const c_void, 31,
		  end.offset_from (buf.as_ptr () as *const c_char) as usize) as *mut c_char;
      /* p points to ^_Ffunctionname\n or ^_Vvarname\n or ^_Sfilename\n.  */
      if !p.is_null ()
	{
	  end = strchr (p, b'\n' as c_int);
	  if end.is_null ()
	    { error (b"DOC file invalid at position %ld\0".as_ptr () as *const c_char, pos); }

	  /* We used to skip files not in build_files, so that when a
	     function was defined several times in different files
	     (typically, once in xterm, once in w32term, ...), we only
	     paid attention to the relevant one.

	     But this meant the doc had to be kept and updated in
	     multiple files.  Nowadays we keep the doc only in eg xterm.
	     The (f)boundp checks below ensure we don't report
	     docs for eg w32-specific items on X.
	  */

	  sym = oblookup (Vobarray, p.offset (2),
			  multibyte_chars_in_text (p.offset (2) as *const u8,
						   end.offset_from (p) - 2),
			  end.offset_from (p) - 2);
          /* Ignore docs that start with SKIP.  These mark
             placeholders where the real doc is elsewhere.  */
	  if SYMBOLP (sym)
	    {
	      /* Attach a docstring to a variable?  */
	      if *p.offset (1) == b'V' as c_char
		{
		  /* Install file-position as variable-documentation
		     property.  */
                  if (!NILP (Fboundp (sym))
                      || !NILP (Fmemq (sym, delayed_init)))
                      && strncmp (end, b"\nSKIP\0".as_ptr () as *const c_char, 5) != 0
                    { Fput (sym, Qvariable_documentation (),
                          make_fixnum (pos + end.offset_from (buf.as_mut_ptr ()) + 1)); }
		}

	      /* Attach a docstring to a function?  */
	      else if *p.offset (1) == b'F' as c_char
                {
                  if !NILP (Ffboundp (sym))
                      && strncmp (end, b"\nSKIP\0".as_ptr () as *const c_char, 5) != 0
                    { store_function_docstring (sym, pos + end.offset_from (buf.as_mut_ptr ()) + 1); }
                }
	      else if *p.offset (1) == b'S' as c_char
		{ } /* Just a source file name boundary marker.  Ignore it.  */

	      else
		{ error (b"DOC file invalid at position %ld\0".as_ptr () as *const c_char, pos); }
	    }
	}
      pos += end.offset_from (buf.as_mut_ptr ());
      filled -= end.offset_from (buf.as_mut_ptr ()) as c_int;
      memmove (buf.as_mut_ptr () as *mut c_void, end as *const c_void, filled as usize);
    }

  return SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil (), sa_mark);
});

/* Return true if text quoting style should default to quote `like this'.  */
fn default_to_grave_quoting_style () -> bool
{
  if !unsafe { text_quoting_flag }
    { return true; }
  if ! DISP_TABLE_P (unsafe { Vstandard_display_table })
    { return false; }
  let dv: Lisp_Object = DISP_CHAR_VECTOR (XCHAR_TABLE (unsafe { Vstandard_display_table }),
					  LEFT_SINGLE_QUOTATION_MARK);
  return VECTORP (dv) && ASIZE (dv) == 1
	  && BASE_EQ (AREF (dv, 0), make_fixnum (b'`' as EMACS_INT));
}

DEFUN! ("text-quoting-style", Ftext_quoting_style,
       Stext_quoting_style, 0, 0, 0,
       doc: "Return the current effective text quoting style.\nIf the variable `text-quoting-style' is `grave', `straight' or\n`curve', just return that value.  If it is nil (the default), return\n`grave' if curved quotes cannot be displayed (for instance, on a\nterminal with no support for these characters), otherwise return\n`curve'.  Any other value is treated as `curve'.\n\nNote that in contrast to the variable `text-quoting-style', this\nfunction will never return nil.",
       (),
       {
  /* Use grave accent and apostrophe `like this'.  */
  if if NILP (Vtext_quoting_style)
       { default_to_grave_quoting_style () }
     else { EQ (Vtext_quoting_style, Qgrave ()) }
    { return Qgrave (); }

  /* Use apostrophes 'like this'.  */
  else if EQ (Vtext_quoting_style, Qstraight ())
    { return Qstraight (); }

  /* Use curved single quotes ‘like this’.  */
  else
    { return Qcurve (); }
});


#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_doc ()
{
  DEFSYM! (Qlisp_directory, "lisp-directory");
  DEFSYM! (Qsubstitute_command_keys, "substitute-command-keys");
  DEFSYM! (Qfunction_documentation, "function-documentation");
  DEFSYM! (Qgrave, "grave");
  DEFSYM! (Qstraight, "straight");
  DEFSYM! (Qcurve, "curve");

  DEFVAR_LISP! (c"internal-doc-file-name", Vdoc_file_name,
	       "Name of file containing documentation strings of built-in symbols.  ");
  Vdoc_file_name = Qnil ();

  DEFVAR_LISP! (c"build-files", Vbuild_files,
               "A list of files used to build this Emacs binary.  ");
  Vbuild_files = Qnil ();

  DEFVAR_LISP! (c"text-quoting-style", Vtext_quoting_style,
               "Style to use for single quotes in help and messages.\n\nThe value of this variable determines substitution of grave accents\nand apostrophes in help output (but not for display of Info\nmanuals) and in functions like `message' and `format-message', but not\nin `format'.\n\nThe value should be one of these symbols:\n  `curve':    quote with curved single quotes ‘like this’.\n  `straight': quote with straight apostrophes \\='like this\\='.\n  `grave':    quote with grave accent and apostrophe \\=`like this\\=';\n	      i.e., do not alter the original quote marks.\n  nil:        like `curve' if curved single quotes are displayable,\n	      and like `grave' otherwise.  This is the default.\n\nYou should never read the value of this variable directly from a Lisp\nprogram.  Use the function `text-quoting-style' instead, as that will\ncompute the correct value for the current terminal in the nil case.  ");
  Vtext_quoting_style = Qnil ();

  DEFVAR_BOOL! (c"documentation-dynamic-reload", documentation_dynamic_reload,
	       "If non-nil, reload changed `DOC' and Lisp files when calling `documentation'.\n\nFor `etc/DOC' and for files byte-compiled with non-nil\n`byte-compile-dynamic-docstring' (the default), documentation strings\nare loaded on-demand only when `documentation' is called for a symbol.\n\nIf these files have changed since they were initially loaded, reading\nthe documentation string for that symbol out of the files may fail.  If\nit fails, and this variable is non-nil, then the files will be loaded\nagain to redefine all its functions and variables.  `documentation' will\nthen retry; if the symbol was redefined by reloading the file, reading\nthe documentation string will then succeed.  ");
  documentation_dynamic_reload = true;

  DEFVAR_BOOL! (c"internal--text-quoting-flag", text_quoting_flag,
	       "If nil, a nil `text-quoting-style' is treated as `grave'.  ");
  /* Initialized by ‘main’.  */

  defsubr (&raw mut Sdocumentation_stringp);
  defsubr (&raw mut Sdocumentation);
  defsubr (&raw mut Ssubr_documentation);
  defsubr (&raw mut Sdocumentation_property);
  defsubr (&raw mut Ssnarf_documentation);
  defsubr (&raw mut Stext_quoting_style);
  DEFSYM! (Qcustom_delayed_init_variables, "custom-delayed-init-variables");
}
