// femacs: port of lib-src/asset-directory-tool.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct field, file-scope object, local), the same control flow and the same error model.
//
// **The frozen configuration compiles no `asset-directory-tool.o`**: the `lib-src` Makefile's program
// lists (`INSTALLABLES`, `UTILITIES`, `DONT_INSTALL`) do not name it - it is built only in the Android
// configuration - and `docs/baseline/frozen-build-sources.txt` 92 records that derivation.  PORTING.md
// P-86 governs the class: the whole body is transcribed as it stands (this file's previous revision was
// the P-08 note), the file gate `have_android` is carried down to the include edges (P-86 ⑧) so the
// frozen translation unit stays empty, and the file's one `#if` (C 231's `#ifdef WORDS_BIGENDIAN`) has
// both of its arms under the `words_bigendian` feature.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar,
              c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

#[cfg (feature = "have_android")]
include!("../src/include/config.rs");      // C 20: `#include <config.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdio.rs");       // C 22: `#include <stdio.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/assert.in.rs");   // C 23: `#include <assert.h>` (`static_assert`)
#[cfg (feature = "have_android")]
include!("../lib/include/fcntl.in.rs");    // C 24: `#include <fcntl.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/errno.in.rs");    // C 25: `#include <errno.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/byteswap.in.rs"); // C 26: `#include <byteswap.h>` (`bswap_32`)
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");      // C 27: `#include <stdlib.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/dirent.in.rs");   // C 28: `#include <dirent.h>` (`struct dirent`)
#[cfg (feature = "have_android")]
include!("../lib/include/dirent.rs");      // C 28: that header's `DIR` and its five calls
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");      // C 29: `#include <string.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/unistd.in.rs");   // C 30: `#include <unistd.h>` (`write`, `ssize_t`)
#[cfg (feature = "have_android")]
include!("../lib/include/sys_stat.in.rs"); // C 32: `#include <sys/stat.h>`

/// `<stddef.h>`'s `size_t`, written out the way `lib-src/emacsclient.rs` 68 does (measured: 8
/// bytes); no mirror in this unit's include graph carries it.
pub type size_t = usize;

/* The value C's `<dirent.h>` gives `struct dirent` on the frozen target.  The port's dirent mirror
   carries the opaque `DIR` and the directory calls but not the record itself (`dirent.in.rs`'s own
   note says so), so it is declared here - the `src/dired.rs` 72-87 shape, whose clang probe
   (recorded in docs/ports/dired.md) measured `sizeof = 1048`, `d_type@20`, `d_name@21`.  */
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
const _: () = core::assert! (std::mem::size_of::<dirent> () == 1048);
const _: () = core::assert! (std::mem::offset_of! (dirent, d_type) == 20);
const _: () = core::assert! (std::mem::offset_of! (dirent, d_name) == 21);

/* `<stdlib.h>`'s `abort` and `exit`, which no mirror carries yet (the `lib-src/emacsclient.rs`
   292-295 and `src/sfnt.rs` notes record the same gap).  */
/// C: `void abort (void);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn abort () -> !;
  /// C: `void exit (int);`
  pub fn exit (status: c_int) -> !;
}

/// C 60: `struct directory_tree` - "The offset to the next sibling, the name of this directory or
/// file, `st_size` of this entry, and the subdirectories and files inside this directory."
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct directory_tree
{
  /// The offset to the next sibling.
  pub offset: size_t,

  /// The name of this directory or file.
  pub name: *mut c_char,

  /// `st_size` of this entry.
  pub st_size: off_t,

  /// Subdirectories and files inside this directory.
  pub children: *mut directory_tree,
  pub next: *mut directory_tree,
}

/// C 80-81: `static bool need_file_size;` - "Whether the size of each entry should be prepended to the
/// start pointer."
#[cfg (feature = "have_android")]
static mut need_file_size: bool = false;

/// C 90-95: `static _Noreturn void croak (const char *what)` - "Exit with EXIT_FAILURE, after printing
/// a description of a failing function WHAT along with the details of the error."
#[cfg (feature = "have_android")]
unsafe fn croak (what: *const c_char) -> !
{
  perror (what);
  exit (EXIT_FAILURE);
}

/// C 98-112: `static void *xmalloc (size_t size)` - "Like malloc, but aborts on failure."  This file's
/// own `static xmalloc` is not `lisp.h`'s object of the same name: that header's mirror is not in this
/// unit's include graph.
#[cfg (feature = "have_android")]
unsafe fn xmalloc (size: size_t) -> *mut c_void
{
  let ptr: *mut c_void;

  ptr = malloc (size);

  if ptr.is_null ()
    { croak (c"malloc".as_ptr ()); }

  return ptr;
}

/// C 119-215: `static void main_1 (DIR *dir, struct directory_tree *parent)` - "Recursively build a
/// struct directory_tree structure for each subdirectory or file in DIR, in preparation for writing it
/// out to disk."
#[cfg (feature = "have_android")]
unsafe fn main_1 (dir: *mut DIR, parent: *mut directory_tree)
{
  /* C 121-124: `struct dirent *dirent; int dir_fd, fd; struct stat statb; struct directory_tree
     *this, **last; size_t length; DIR *otherdir;` - C leaves these uninitialised and writes them
     before any read; the port spells C's own zero values (PORTING.md 0.1-13).  */
  let mut dirent: *mut dirent = std::ptr::null_mut ();
  let mut dir_fd: c_int = 0;
  let mut fd: c_int = 0;
  let mut statb: stat = std::mem::zeroed ();
  let mut this: *mut directory_tree = std::ptr::null_mut ();
  let mut last: *mut *mut directory_tree = std::ptr::null_mut ();
  let mut length: size_t = 0;
  let mut otherdir: *mut DIR = std::ptr::null_mut ();

  dir_fd = dirfd (dir);
  last = &raw mut (*parent).children;

  /* C 127: `while ((dirent = readdir (dir)))` - the mirror declares `readdir` the way the reference's
     header does (`void *`), so the port takes the record pointer it hands back.  */
  loop
    {
      dirent = readdir (dir) as *mut dirent;
      if dirent.is_null ()
	{ break; }

      /* Determine what kind of file DIRENT is.  */

      if fstatat (dir_fd, (*dirent).d_name.as_ptr (), &raw mut statb,
		  AT_SYMLINK_NOFOLLOW) == -1
	{ croak (c"fstatat".as_ptr ()); }

      /* Ignore . and ...  */

      if strcmp ((*dirent).d_name.as_ptr (), c".".as_ptr ()) == 0
	  || strcmp ((*dirent).d_name.as_ptr (), c"..".as_ptr ()) == 0
	{ continue; }

      length = strlen ((*dirent).d_name.as_ptr ());

      if statb.st_mode & S_IFDIR != 0
	{
	  /* This is a directory.  Write its name followed by a
	     trailing slash, then a NULL byte, and the offset to the
	     next sibling.  */
	  this = xmalloc (std::mem::size_of::<directory_tree> ()) as *mut directory_tree;
	  (*this).children = std::ptr::null_mut ();
	  (*this).next = std::ptr::null_mut ();
	  *last = this;
	  last = &raw mut (*this).next;
	  (*this).name = xmalloc (length + 2) as *mut c_char;
	  strcpy ((*this).name, (*dirent).d_name.as_ptr ());
	  (*this).st_size = 0;

	  /* Now record the offset to the end of this directory.  This
	     is length + 1, for the file name, 5 more bytes for the
	     trailing NULL and long, and 4 further bytes if a file size
	     is required.  */
	  (*this).offset = (*parent).offset
			   + length + 6 + if need_file_size { 4 } else { 0 };

	  /* Terminate that with a slash and trailing NULL byte.  */
	  *(*this).name.offset (length as isize) = b'/' as c_char;
	  *(*this).name.offset (length as isize + 1) = 0;

	  /* Open and build that directory recursively.  */

	  fd = openat (dir_fd, (*dirent).d_name.as_ptr (), O_DIRECTORY,
		       O_RDONLY);
	  if fd < 0
	    { croak (c"openat".as_ptr ()); }
	  otherdir = fdopendir (fd);
	  if otherdir.is_null ()
	    { croak (c"fdopendir".as_ptr ()); }

	  main_1 (otherdir, this);

	  /* Close this directory.  */
	  closedir (otherdir);

	  /* Finally, set parent->offset to this->offset as well.  */
	  (*parent).offset = (*this).offset;
	}
      else if statb.st_mode & S_IFREG != 0
	{
	  /* This is a regular file.  */
	  this = xmalloc (std::mem::size_of::<directory_tree> ()) as *mut directory_tree;
	  (*this).children = std::ptr::null_mut ();
	  (*this).next = std::ptr::null_mut ();
	  *last = this;
	  last = &raw mut (*this).next;
	  (*this).name = xmalloc (length + 1) as *mut c_char;
	  (*this).st_size = statb.st_size;
	  strcpy ((*this).name, (*dirent).d_name.as_ptr ());

	  if (*this).st_size >= 0x1ffffff
	    {
	      fprintf (stderr,
		       c"asset-directory-tool: file size exceeds maximum representable in a directory-tree: %s\n".as_ptr (),
		       (*dirent).d_name.as_ptr ());
	      exit (EXIT_FAILURE);
	    }

	  /* This is one byte the shorter because there is no trailing
	     slash.  */
	  (*this).offset = (*parent).offset + length + 5
			   + if need_file_size { 4 } else { 0 };
	  (*parent).offset = (*this).offset;
	}
    }
}

/// C 218-272: `static void main_2 (int fd, struct directory_tree *tree, size_t *offset)` - "Write the
/// struct directory_tree TREE and all of is children to the file descriptor FD."
#[cfg (feature = "have_android")]
unsafe fn main_2 (fd: c_int, tree: *mut directory_tree, offset: *mut size_t)
{
  let mut size: ssize_t = 0;
  let mut child: *mut directory_tree = std::ptr::null_mut ();
  let mut output: [c_uint; 2] = [0; 2];

  /* Write tree->name with the trailing NULL byte.  */
  size = (strlen ((*tree).name) + 1) as ssize_t;
  if write (fd, (*tree).name as *const c_void, size as size_t) < size
    { croak (c"write".as_ptr ()); }

  /* Write the offset.  C 231-237's `#ifdef WORDS_BIGENDIAN` pair, both arms carried (P-51/P-86).  */
  #[cfg (feature = "words_bigendian")]
  {
    output[1] = bswap_32 ((*tree).offset as uint_least32_t);
    output[0] = bswap_32 ((*tree).st_size as c_uint as uint_least32_t);
  }
  #[cfg (not (feature = "words_bigendian"))]
  {
    output[1] = (*tree).offset as c_uint;
    output[0] = (*tree).st_size as c_uint;
  }

  /* C 239: `static_assert (sizeof output == 8 && sizeof output[0] == 4);`  */
  const _: () = core::assert! (std::mem::size_of::<[c_uint; 2]> () == 8
			       && std::mem::size_of::<c_uint> () == 4);
  if !need_file_size
    {
      if write (fd, (&raw const output).cast::<c_void> ().offset (4), 4) < 1
	{ croak (c"write".as_ptr ()); }
      size += 4;
    }
  else
    {
      if write (fd, (&raw const output).cast::<c_void> (), 8) < 1
	{ croak (c"write".as_ptr ()); }
      size += 8;
    }

  /* Now update offset.  */
  *offset += size as size_t;

  /* Write out each child.  */
  child = (*tree).children;
  while !child.is_null ()
    {
      main_2 (fd, child, offset);
      child = (*child).next;
    }

  /* Verify the offset is correct.  */
  if (*tree).offset != *offset
    {
      fprintf (stderr,
	       c"asset-directory-tool: invalid offset: expected %tu, got %tu.\nPlease report this bug to bug-gnu-emacs@gnu.org, along\nwith an archive containing the contents of the java/install_temp directory.\n".as_ptr (),
	       (*tree).offset, *offset);
      abort ();
    }
}

/// C 274-338: `int main (int argc, char **argv)` - the program's entry point, so the port keeps the
/// name and exports it (`#[unsafe(no_mangle)]`, PORTING.md 0.2).
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut fd: c_int = 0;
  let mut indir: *mut DIR = std::ptr::null_mut ();
  let mut tree: directory_tree = std::mem::zeroed ();
  let mut offset: size_t = 0;

  if argc != 3 && argc != 4
    {
      fprintf (stderr, c"usage: %s directory output-file [--api-8]\n".as_ptr (),
	       *argv);
      return EXIT_FAILURE;
    }

  if argc == 4 && strcmp (*argv.offset (3), c"--api-8".as_ptr ()) == 0
    { need_file_size = true; }

  /* C's `mode_t` (16-bit here) is widened by C's own default argument promotion at this variadic
     call, so the port passes C's promoted type (PORTING.md 2.4).  */
  fd = open (*argv.offset (2), O_CREAT | O_TRUNC | O_RDWR,
	     (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP) as c_int);

  if fd < 0
    {
      perror (c"open".as_ptr ());
      return EXIT_FAILURE;
    }

  indir = opendir (*argv.offset (1));

  if indir.is_null ()
    {
      perror (c"opendir".as_ptr ());
      return EXIT_FAILURE;
    }

  /* Write the first 5 byte header to FD.  */

  if write (fd, c"EMACS".as_ptr () as *const c_void, 5) < 5
    {
      perror (c"write".as_ptr ());
      return EXIT_FAILURE;
    }

  /* And a further 4 bytes of padding if need_file_size.  */
  if need_file_size && write (fd, c"____".as_ptr () as *const c_void, 4) < 4
    {
      perror (c"write".as_ptr ());
      return EXIT_FAILURE;
    }

  /* Now iterate through children of INDIR, building the directory tree.  */
  tree.offset = 5 + if need_file_size { 4 } else { 0 };
  tree.children = std::ptr::null_mut ();

  main_1 (indir, &raw mut tree);
  closedir (indir);

  /* Finally, write the directory tree to the output file.  */
  offset = 5 + if need_file_size { 4 } else { 0 };
  while !tree.children.is_null ()
    {
      main_2 (fd, tree.children, &raw mut offset);
      tree.children = (*tree.children).next;
    }

  return 0;
}
