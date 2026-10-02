// femacs: mirror of the frozen target's <sys/time.h> (system header, P-26/P-50).
//
// There is no reference file to pair with and gnulib's `sys_time.in.h` is not used by this
// build, so this carries what the ported units name.  So far that is `struct timeval`
// (src/systime.h declares `make_timeval` and `set_waiting_for_input`, and timefns.c fills
// one in).  `time_t` and `struct timespec` live in lib/include/time.in.rs, the mirror of
// gnulib's `time.in.h`; in C they come from <time.h>, a different header, so they keep
// their own mirror.
//
// The layout is the frozen target's, measured with the frozen configuration:
//
//     $ cat > /tmp/tv.c <<'EOF'
//     #include <stddef.h>
//     #include <sys/time.h>
//     #include <time.h>
//     #include <stdio.h>
//     int main (void) { printf ("%zu %zu %zu | %zu %zu\n", sizeof (struct timeval),
//         offsetof (struct timeval, tv_sec), offsetof (struct timeval, tv_usec),
//         sizeof (time_t), sizeof (suseconds_t)); return 0; }
//     EOF
//     $ clang -std=gnu23 -o /tmp/tv /tmp/tv.c && /tmp/tv
//     16 0 8 | 8 4
//
// i.e. `time_t` is 8 bytes at 0 and `suseconds_t` is 4 bytes at 8, with 4 bytes of tail
// padding.  The assertions below pin that, the same way spikes/syshdrs-probe.sh pins the
// other system mirrors' constants (a lib mirror is compiled without lisp.rs in that probe,
// so it uses `const _` rather than the port's `static_assert!`).

/// The frozen target's `suseconds_t`: a 32-bit signed integer (`time.in.rs` has `time_t`).
/// `c_int` comes from the includer, as it does for every lib mirror (lisp.rs's
/// `use std::ffi::{…}` at the crate root); a probe that includes no src mirror brings its
/// own (spikes/syshdrs_probe.rs does).
pub type suseconds_t = c_int;

/* struct timeval
   {
     __darwin_time_t tv_sec;
     __darwin_suseconds_t tv_usec;
   };  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval
{
  pub tv_sec: time_t,
  pub tv_usec: suseconds_t,
}

const _: () = assert! (std::mem::size_of::<timeval> () == 16);
const _: () = assert! (std::mem::offset_of! (timeval, tv_sec) == 0);
const _: () = assert! (std::mem::offset_of! (timeval, tv_usec) == 8);
const _: () = assert! (std::mem::size_of::<time_t> () == 8);
const _: () = assert! (std::mem::size_of::<suseconds_t> () == 4);
