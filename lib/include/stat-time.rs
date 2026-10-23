// femacs: port of lib/stat-time.h @ fd7afced403.
//
// The header's nine functions are `_GL_STAT_TIME_INLINE` definitions (the C99 inline model;
// PORTING.md 10.6/10.12), so they are ordinary Rust items here and the `.c` wrapper
// `lib/stat-time.rs` keeps the `_GL_STAT_TIME_INLINE`/`_GL_EXTERN_INLINE` relationship.
// Its two selects are **not** in `docs/baseline/config/cond-eval-verified.txt`, so
// `extract-abi.py` reads every arm as live (P-08's blind default) and the mirror carries the
// dead text as P-51 requires.
//
// The frozen configuration defines `HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC` and
// `HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC` (both 1; `HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC`,
// `HAVE_STRUCT_STAT_ST_ATIMENSEC`, `HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC`,
// `HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC` and `_GL_WINDOWS_STAT_TIMESPEC` are all `undef`), so
// C 58-67's arm in force is `#elif defined HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC`:
// `STAT_TIMESPEC (st, st_xtim)` is `(st)->st_xtim##espec` and `STAT_TIMESPEC_OFFSETOF
// (st_xtim)` is `offsetof (struct stat, st_xtim##espec)`.
//
// Rust cannot paste an identifier, so the two macros are written out with one arm per
// argument C's own uses pass (`st_atim`, `st_ctim`, `st_mtim`, `st_birthtim` - the four the
// header's document comment names; the `NUMNAME`/`DYNARRAY_NAME` shape).  A call site then
// keeps C's text: `STAT_TIMESPEC! (st, st_atim)`.
//
// The arms no port configuration takes are P-51 text under two port-local feature names,
// spelled out once here rather than repeated:
//
//    "stat_time_stat_timespec_ns"  = C 70-80's `STAT_TIMESPEC_NS` arm, i.e.
//         `(_GL_WINDOWS_STAT_TIMESPEC || HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC)`
//         `&& !(_GL_WINDOWS_STAT_TIMESPEC || TYPEOF_STRUCT_STAT_ST_ATIM_IS_STRUCT_TIMESPEC)`
//         `|| HAVE_STRUCT_STAT_ST_ATIMENSEC || HAVE_STRUCT_STAT_ST_ATIM_ST__TIM_TV_NSEC`;
//    "stat_time_member_absent"     = C 81-82's `# else`, i.e. neither `STAT_TIMESPEC` nor
//         `STAT_TIMESPEC_NS` is defined, and C 155/166/177's `# else return
//         _gl_make_timespec (st->st_atime, get_stat_atime_ns (st));` arms.
//
// Both are off for the frozen target and no ported unit sets them; the asserts below keep
// that from changing silently (P-51).

include!("time.in.rs");        // <time.h> (`struct timespec`)
include!("sys_stat.in.rs");    // <sys/stat.h> (`struct stat`)
include!("errno.rs");          // <errno.h> (EOVERFLOW)
include!("stdcountof.rs");     // <stdcountof.h> (countof)

const _: () = core::assert! (!cfg! (feature = "stat_time_stat_timespec_ns"),
                       "the frozen configuration defines HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC");
const _: () = core::assert! (!cfg! (feature = "stat_time_member_absent"),
                       "the frozen configuration's struct stat carries the timespec members");

/* stat-time.h 23: the include guard has no Rust counterpart (PORTING.md 0.1).  */
pub const STAT_TIME_H: c_int = 1;

/* stat-time.h 61-62 (the arm in force): `STAT_TIMESPEC (st, st_xtim)` is
   `(st)->st_xtim##espec`.  The pasting is enumerated over the arguments the header's own
   functions pass.  */
/* The `##` paste is target-dependent in a way the header hides: C's `STAT_TIMESPEC (st,
   st_mtim)` produces `st_mtimespec` only where `struct stat` spells the member that way (the
   host), and `st_mtim` where it does not (bionic's `<sys/stat.h>`, status 4.794).  Two macro
   definitions under complementary cfgs keep every call site spelled as C spells it.  */
#[cfg (not (feature = "have_android"))]
macro_rules! STAT_TIMESPEC {
  ($st:expr, st_atim) => { (*$st).st_atimespec };
  ($st:expr, st_ctim) => { (*$st).st_ctimespec };
  ($st:expr, st_mtim) => { (*$st).st_mtimespec };
  ($st:expr, st_birthtim) => { (*$st).st_birthtimespec };
}
#[cfg (feature = "have_android")]
macro_rules! STAT_TIMESPEC {
  ($st:expr, st_atim) => { (*$st).st_atim };
  ($st:expr, st_ctim) => { (*$st).st_ctim };
  ($st:expr, st_mtim) => { (*$st).st_mtim };
  /* bionic's `struct stat` has no birth time member at all; C's `st_birthtim` arm is the
     host's, so on this target the name is not a field.  */
}
/* stat-time.h 62: `STAT_TIMESPEC_OFFSETOF (st_xtim)` is
   `offsetof (struct stat, st_xtim##espec)`; `offsetof` is Rust's `std::mem::offset_of!`.  */
#[cfg (not (feature = "have_android"))]
macro_rules! STAT_TIMESPEC_OFFSETOF {
  (st_atim) => { std::mem::offset_of! (stat, st_atimespec) };
  (st_mtim) => { std::mem::offset_of! (stat, st_mtimespec) };
  (st_ctim) => { std::mem::offset_of! (stat, st_ctimespec) };
}
#[cfg (feature = "have_android")]
macro_rules! STAT_TIMESPEC_OFFSETOF {
  (st_atim) => { std::mem::offset_of! (stat, st_atim) };
  (st_mtim) => { std::mem::offset_of! (stat, st_mtim) };
  (st_ctim) => { std::mem::offset_of! (stat, st_ctim) };
}

/* Return the nanosecond component of *ST's access time.  */
#[inline(always)]
pub unsafe fn get_stat_atime_ns (st: *const stat) -> c_long
{
  /* stat-time.h 76-80: the `STAT_TIMESPEC_NS` arm is dead text (see the file note).  */
  #[cfg (feature = "stat_time_stat_timespec_ns")]
  { return STAT_TIMESPEC_NS! (st, st_atim); }
  /* stat-time.h 81-82's `# else return 0;`.  */
  #[cfg (feature = "stat_time_member_absent")]
  { return 0; }
  return STAT_TIMESPEC! (st, st_atim).tv_nsec;
}

/* Return the nanosecond component of *ST's status change time.  */
#[inline(always)]
pub unsafe fn get_stat_ctime_ns (st: *const stat) -> c_long
{
  /* stat-time.h 89-93: the same two arms.  */
  #[cfg (feature = "stat_time_stat_timespec_ns")]
  { return STAT_TIMESPEC_NS! (st, st_ctim); }
  #[cfg (feature = "stat_time_member_absent")]
  { return 0; }
  return STAT_TIMESPEC! (st, st_ctim).tv_nsec;
}

/* Return the nanosecond component of *ST's data modification time.  */
#[inline(always)]
pub unsafe fn get_stat_mtime_ns (st: *const stat) -> c_long
{
  /* stat-time.h 102-106: the same two arms.  */
  #[cfg (feature = "stat_time_stat_timespec_ns")]
  { return STAT_TIMESPEC_NS! (st, st_mtim); }
  #[cfg (feature = "stat_time_member_absent")]
  { return 0; }
  return STAT_TIMESPEC! (st, st_mtim).tv_nsec;
}

/* Return the nanosecond component of *ST's birth time.  */
#[inline(always)]
pub unsafe fn get_stat_birthtime_ns (st: *const stat) -> c_long
{
  /* stat-time.h 115-121: `# if defined HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC` is the
     frozen arm; the `STAT_TIMESPEC_NS` arm and the `# else return 0;` arm are dead text.  */
  #[cfg (feature = "stat_time_stat_timespec_ns")]
  { return STAT_TIMESPEC_NS! (st, st_birthtim); }
  #[cfg (feature = "stat_time_member_absent")]
  { return 0; }
  /* bionic's `struct stat` has no birth time member, which is exactly C's `#else return 0;`
     arm (stat-time.h 115-121), status 4.794.  */
  #[cfg (feature = "have_android")]
  { return 0; }
  #[cfg (not (feature = "have_android"))]
  { return STAT_TIMESPEC! (st, st_birthtim).tv_nsec; }
}

/* stat-time.h 128-135: C's C99 compound literal and its C++ inline function say the same
   thing; the port writes the function the C++ arm spells (a compound literal would assume
   the fields' order, which the reference's own comment rejects).  */
#[inline(always)]
pub unsafe fn _gl_make_timespec (sec: time_t, nsec: c_long) -> timespec
{
  let mut ts: timespec = std::mem::zeroed ();
  ts.tv_sec = sec;
  ts.tv_nsec = nsec;
  return ts;
}

/* Return *ST's access time.  */
#[inline(always)]
pub unsafe fn get_stat_atime (st: *const stat) -> timespec
{
  /* stat-time.h 155: the `# else` arm is dead text (see the file note).  */
  #[cfg (feature = "stat_time_member_absent")]
  { return _gl_make_timespec ((*st).st_atime, get_stat_atime_ns (st)); }
  return STAT_TIMESPEC! (st, st_atim);
}

/* Return *ST's status change time.  */
#[inline(always)]
pub unsafe fn get_stat_ctime (st: *const stat) -> timespec
{
  /* stat-time.h 166: the same arm.  */
  #[cfg (feature = "stat_time_member_absent")]
  { return _gl_make_timespec ((*st).st_ctime, get_stat_ctime_ns (st)); }
  return STAT_TIMESPEC! (st, st_ctim);
}

/* Return *ST's data modification time.  */
#[inline(always)]
pub unsafe fn get_stat_mtime (st: *const stat) -> timespec
{
  /* stat-time.h 177: the same arm.  */
  #[cfg (feature = "stat_time_member_absent")]
  { return _gl_make_timespec ((*st).st_mtime, get_stat_mtime_ns (st)); }
  return STAT_TIMESPEC! (st, st_mtim);
}

/* Return *ST's birth time, if available; otherwise return a value
   with tv_sec and tv_nsec both equal to -1.  */
#[inline(always)]
pub unsafe fn get_stat_birthtime (st: *const stat) -> timespec
{
  /* C's `struct timespec t;` is uninitialized and every arm assigns it; PORTING.md 0.1-13
     has the port zero it first.  */
  let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };

  /* stat-time.h 190-204: `# if (defined HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC ||
     defined HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC)` is the frozen arm; the
     `HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC` arm, the `_WIN32` arm and the "not supported"
     `# else` are dead text (the first two are P-51 text here).  */
  #[cfg (feature = "stat_time_birthtime_nsec_arm")]
  { t = _gl_make_timespec ((*st).st_birthtime, (*st).st_birthtimensec); }
  #[cfg (feature = "windowsnt")]
  {
    #[cfg (feature = "windows_stat_timespec")]
    { t = (*st).st_ctim; }
    #[cfg (not (feature = "windows_stat_timespec"))]
    { t = _gl_make_timespec ((*st).st_ctime, 0); }
  }
  #[cfg (feature = "stat_time_no_birthtime")]
  { t = _gl_make_timespec (-1, -1); }
  /* C's `#else` arm for a `struct stat` without the member (bionic's, status 4.794) says the
     birth time is unknown; the host's arm reads the member.  */
  #[cfg (feature = "have_android")]
  { t = _gl_make_timespec (-1, -1); }
  #[cfg (not (feature = "have_android"))]
  { t = STAT_TIMESPEC! (st, st_birthtim); }

  /* stat-time.h 213-215's arm (the same configuration selects it): FreeBSD and NetBSD
     sometimes signal the absence of knowledge by using zero.  */
  if !(t.tv_sec != 0 && 0 <= t.tv_nsec && t.tv_nsec < 1000000000)
    { t = _gl_make_timespec (-1, -1); }

  return t;
}

/* If a stat-like function returned RESULT, normalize the timestamps
   in *ST, if this platform suffers from a macOS and Solaris bug where
   tv_nsec might be negative.  Return the adjusted RESULT, setting
   errno to EOVERFLOW if normalization overflowed.  This function
   is intended to be private to Gnulib.  */
#[inline(always)]
pub unsafe fn stat_time_normalize (result: c_int, st: *mut stat) -> c_int
{
  /* stat-time.h 228-229's condition `((defined __APPLE__ && defined __MACH__) || defined
     __sun) && defined STAT_TIMESPEC_OFFSETOF` holds on the frozen target (darwin-arm64), so
     the body below is the live text; it is what `lib/lstat.c` and `lib/fstatat.c` call.  */
  if result == 0
    {
      let timespec_hz: c_long = 1000000000;
      let ts_off: [c_short; 3] = [ STAT_TIMESPEC_OFFSETOF! (st_atim) as c_short,
                                   STAT_TIMESPEC_OFFSETOF! (st_mtim) as c_short,
                                   STAT_TIMESPEC_OFFSETOF! (st_ctim) as c_short ];
      let mut i: c_int = 0;
      while i < countof! (ts_off) as c_int
        {
          let ts: *mut timespec = st.cast::<c_char> ()
            .offset (ts_off[i as usize] as isize).cast::<timespec> ();
          let mut q: c_long = (*ts).tv_nsec / timespec_hz;
          let mut r: c_long = (*ts).tv_nsec % timespec_hz;
          if r < 0
            {
              r += timespec_hz;
              q -= 1;
            }
          (*ts).tv_nsec = r;
          /* Overflow is possible, as Solaris 11 stat can yield
             tv_sec == TYPE_MINIMUM (time_t) && tv_nsec == -1000000000.  */
          let (sum, overflow) = q.overflowing_add ((*ts).tv_sec);
          (*ts).tv_sec = sum;
          if overflow
            {
              *errno () = EOVERFLOW;
              return -1;
            }
          i += 1;
        }
    }
  return result;
}
