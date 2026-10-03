// femacs: port of lib/group-member.c @ fd7afced403.
/* emacs/lib/group-member.c: C 24's `<stdckdint.h>` is the `ckd_mul` alias family (P-15), C 25's
   `<stdio.h>` and C 26's `<sys/types.h>` are the mirrors below, and C 27's `<stdlib.h>` supplies
   `malloc`/`free` (declared in this unit).  C 31's `enum { GROUPBUF_SIZE = 100 }` is a constant
   and the two groups functions come from <unistd.h>, so this unit declares them too.  */
use std::ffi::{c_char, c_int, c_uint, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/stdio.rs");

unsafe extern "C" {
  pub fn getgroups (n: c_int, groups: *mut gid_t) -> c_int;
  pub fn malloc (n: usize) -> *mut c_void;
  pub fn free (p: *mut c_void);
}

/* Most processes have no more than this many groups, and for these
   processes we can avoid using malloc.  */
pub const GROUPBUF_SIZE: usize = 100;

#[repr(C)]
pub struct group_info
{
  pub group: *mut gid_t,
  pub groupbuf: [gid_t; GROUPBUF_SIZE],
}

unsafe fn free_group_info (g: *const group_info)
{
  if (*g).group != (*g).groupbuf.as_ptr () as *mut gid_t
    { free ((*g).group as *mut c_void); }
}

unsafe fn get_group_info (gi: *mut group_info) -> c_int
{
  let mut n_groups: c_int = getgroups (GROUPBUF_SIZE as c_int, (*gi).groupbuf.as_mut_ptr ());
  (*gi).group = (*gi).groupbuf.as_mut_ptr ();

  if n_groups < 0
    {
      let n_group_slots: c_int = getgroups (0, std::ptr::null_mut ());
      let mut nbytes: usize;
      let (product, overflowed)
	= (n_group_slots as usize).overflowing_mul (size_of::<gid_t> ());
      nbytes = product;
      if !overflowed
        {
          (*gi).group = malloc (nbytes) as *mut gid_t;
          if !(*gi).group.is_null ()
            { n_groups = getgroups (n_group_slots, (*gi).group); }
        }
    }

  /* In case of error, the user loses.  */
  return n_groups;
}

/* Return non-zero if GID is one that we have in our groups list.
   Note that the groups list is not guaranteed to contain the current
   or effective group ID, so they should generally be checked
   separately.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn group_member (gid: gid_t) -> c_int
{
  let mut gi: group_info = std::mem::zeroed ();
  let n_groups: c_int = get_group_info (&raw mut gi);

  /* Search through the list looking for GID. */
  let mut found: c_int = 0;
  let mut i: c_int = 0;
  while i < n_groups
    {
      if gid == *(gi.group).offset (i as isize)
	{
	  found = 1;
	  break;
	}
      i += 1;
    }

  free_group_info (&raw const gi);

  return found;
}

/* C 95-108's `#ifdef TEST` arm: TEST is the macro the reference's own harness defines, so the
   main it guards is dead text here; it is kept under a feature naming it (P-51), and it needs
   `atoi`, `printf` and `exit`, which the port declares for it.  */
#[cfg (feature = "test")]
unsafe extern "C" {
  pub fn atoi (s: *const c_char) -> c_int;
  pub fn printf (format: *const c_char, ...) -> c_int;
  pub fn exit (status: c_int) -> !;
}

#[cfg (feature = "test")]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut i: c_int = 1;
  while i < argc
    {
      let gid: gid_t = atoi (*argv.offset (i as isize)) as gid_t;
      printf (c"%d: %s\n".as_ptr (), gid as c_int,
	      if group_member (gid) != 0 { c"yes".as_ptr () } else { c"no".as_ptr () });
      i += 1;
    }
  exit (0);
}
