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

/// The frozen target's `struct tm`; strftime.c and mktime.c read and write it field by
/// field, and the `HAVE_STRUCT_TM_TM_GMTOFF`/`_ZONE` values (both 1 for this target)
/// make `tm_gmtoff`/`tm_zone` live.  The layout is the frozen target's, measured with
/// the frozen configuration:
///
///     $ cat > /tmp/tm.c <<'EOF'
///     #include <stddef.h>
///     #include <time.h>
///     #include <stdio.h>
///     int main (void) { printf ("%zu %zu %zu %zu\n", sizeof (struct tm),
///         offsetof (struct tm, tm_isdst), offsetof (struct tm, tm_gmtoff),
///         offsetof (struct tm, tm_zone)); return 0; }
///     EOF
///     $ clang -o /tmp/tm /tmp/tm.c && /tmp/tm
///     56 32 40 48
///
/// i.e. nine `int` members at 0..32 (with `tm_isdst` last), four bytes of padding, the
/// `long tm_gmtoff` at 40 and the `char *tm_zone` at 48.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tm
{
  pub tm_sec: c_int,
  pub tm_min: c_int,
  pub tm_hour: c_int,
  pub tm_mday: c_int,
  pub tm_mon: c_int,
  pub tm_year: c_int,
  pub tm_wday: c_int,
  pub tm_yday: c_int,
  pub tm_isdst: c_int,
  pub tm_gmtoff: c_long,
  pub tm_zone: *const c_char,
}

const _: () = assert! (std::mem::size_of::<tm> () == 56);
const _: () = assert! (std::mem::offset_of! (tm, tm_sec) == 0);
const _: () = assert! (std::mem::offset_of! (tm, tm_isdst) == 32);
const _: () = assert! (std::mem::offset_of! (tm, tm_gmtoff) == 40);
const _: () = assert! (std::mem::offset_of! (tm, tm_zone) == 48);

/// `time.in.h` 494's `typedef struct tm_zone *timezone_t;` - the template only
/// forward-declares `struct tm_zone` (its definition is `lib/time-internal.h`'s), so the
/// port spells the pointer as an opaque `*mut c_void` (PORTING.md 0.1-2); the cookie
/// `local_tz` is `(timezone_t) 1` in the time-internal mirror, which is never
/// dereferenced (time-internal.h 51-54).
pub type timezone_t = *mut c_void;

unsafe extern "C" {
  /* struct tm *gmtime (time_t const *);  */
  pub fn gmtime (t: *const time_t) -> *mut tm;
  /* struct tm *localtime (time_t const *);  */
  pub fn localtime (t: *const time_t) -> *mut tm;
  /* struct tm *gmtime_r (time_t const *restrict, struct tm *restrict);  */
  pub fn gmtime_r (t: *const time_t, tp: *mut tm) -> *mut tm;
  /* struct tm *localtime_r (time_t const *restrict, struct tm *restrict);  */
  pub fn localtime_r (t: *const time_t, tp: *mut tm) -> *mut tm;
  /* time_t mktime (struct tm *);  */
  pub fn mktime (tp: *mut tm) -> time_t;
  /* time_t timegm (struct tm *);  (HAVE_TIMEGM is 1 for this target)  */
  pub fn timegm (tp: *mut tm) -> time_t;
  /* void tzset (void);  */
  pub fn tzset ();
  /* size_t strftime (char *restrict, size_t, char const *restrict, struct tm const *restrict);  */
  pub fn strftime (s: *mut c_char, maxsize: usize, format: *const c_char, tp: *const tm)
    -> usize;
  /* time.h 1088-1120: the two entries the time_rz module adds (`REPLACE_LOCALTIME_RZ`
     and `REPLACE_MKTIME_Z` are undefined, so these are the sys spellings).  */
  pub fn localtime_rz (tz: timezone_t, t: *const time_t, tp: *mut tm) -> *mut tm;
  pub fn mktime_z (tz: timezone_t, tp: *mut tm) -> time_t;
  /* extern char *tzname[2];  */
  pub static mut tzname: [*mut c_char; 2];
}
