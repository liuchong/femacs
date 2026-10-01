// femacs: port of lib/time.in.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror: only the types buffer.h names by value or reference
// (`struct timespec`, `time_t`); the rest of the header is tracked in
// docs/ports/include-time.in.md (PORTING.md P-26).

/// The frozen target's `time_t` (a 64-bit signed integer).
pub type time_t = i64;

/// The frozen target's `struct timespec`; `buffer.h` embeds one (`modtime`), and
/// its size and the offsets inside `struct buffer` are asserted against the C
/// compiler's numbers.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timespec
{
  pub tv_sec: time_t,
  pub tv_nsec: c_long,
}
