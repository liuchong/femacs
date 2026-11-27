// femacs: port of lib/getrandom.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::sys_common::*;
/* emacs/lib/getrandom.c (@ 193 lines) is on the verified list: this is not a native Windows
   target and HAVE_GETRANDOM is undefined, so C 30-95's `_WIN32` setup, C 122-176's Windows arm
   and C 177-178's `#elif HAVE_GETRANDOM` arm are dead by measurement, and the live text is
   C 99-108's two device names and C 181-192's /dev/urandom reader.  C 22's <sys/random.h> is
   the new sys_random mirror (GRND_NONBLOCK/GRND_RANDOM), C 24-26's <errno.h>, <fcntl.h> and
   <unistd.h> are the errno, fcntl and unistd mirrors, and `MAX` comes from the minmax mirror.  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_random.in.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/minmax.rs imported via common
// [U1] include/limits.in.rs imported via common

/* C 99-108's `# ifndef NAME_OF_RANDOM_DEVICE / #  define NAME_OF_RANDOM_DEVICE
   "/dev/random"` and its `NAME_OF_NONCE_DEVICE` twin: gnulib's system-header arm is dead here
   (there is no <sys/random.h>), so the fallbacks are the live values.  */
pub const NAME_OF_RANDOM_DEVICE: &[u8; 12] = b"/dev/random\0";
pub const NAME_OF_NONCE_DEVICE: &[u8; 13] = b"/dev/urandom\0";

/* Set BUFFER (of size LENGTH) to random bytes under the control of FLAGS.
   Return the number of bytes written (> 0).
   Upon error, return -1 and set errno.  */

/* C 112-193's `ssize_t getrandom (void *buffer, size_t length, unsigned int flags)`.  C 116's
   `#undef getrandom` restores the system declaration for the dead `#elif HAVE_GETRANDOM` arm,
   so the port keeps that entry in a `system` module and defines the replacement under C's name
   here (the getgroups.c/cloexec.c shape).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getrandom (buffer: *mut c_void, length: usize, flags: c_uint) -> ssize_t
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* C 118-176's `#if defined _WIN32 && ! defined __CYGWIN__` arm is dead on this target.  */

  /* C 177-178's `#elif HAVE_GETRANDOM` arm is dead: this target has no system getrandom.  */

  /* C 181-192's `#else` arm, the live one.  */
  static randfd: VmGlobal<[c_int; 2]> = VmGlobal::<[c_int; 2]>::new ([-1, -1]);

  let devrandom: bool = (flags & GRND_RANDOM as c_uint) != 0;
  let mut fd: c_int = (*randfd.as_mut_ptr ())[devrandom as usize];

  if fd < 0
    {
      /* C 141-145's `static char const randdevice[][MAX (sizeof NAME_OF_NONCE_DEVICE,
         sizeof NAME_OF_RANDOM_DEVICE)] = { NAME_OF_NONCE_DEVICE, NAME_OF_RANDOM_DEVICE };`
         - the table is a local binding of the two names' addresses, which is the same read-only
         data C's array holds (the `MAX` row length needed there is recorded in the notes).  */
      let randdevice: [*const c_char; 2] =
        [ NAME_OF_NONCE_DEVICE.as_ptr () as *const c_char,
          NAME_OF_RANDOM_DEVICE.as_ptr () as *const c_char ];
      let oflags: c_int = O_RDONLY + O_CLOEXEC
        + (if flags & GRND_NONBLOCK as c_uint != 0 { O_NONBLOCK } else { 0 });
      fd = open (randdevice[devrandom as usize], oflags);
      if fd < 0
        {
          if *errno () == ENOENT || *errno () == ENOTDIR
            { *errno () = ENOSYS; }
          return -1;
        }
      (*randfd.as_mut_ptr ())[devrandom as usize] = fd;
    }

  return read (fd, buffer, length);

  }
}

/* C 116's `#undef getrandom`: the system entry the dead `#elif HAVE_GETRANDOM` arm calls.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn getrandom (buffer: *mut c_void, length: usize, flags: c_uint) -> ssize_t;
  }
}
