// femacs: mirror of lib/stat-time.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header's functions are `_GL_STAT_TIME_INLINE`-qualified definitions (the C99 inline
// model; PORTING.md 10.6/10.12), so they are ordinary Rust items here and the `.c` wrapper
// `femacs/lib/stat-time.rs` keeps the `_GL_STAT_TIME_INLINE`/`_GL_EXTERN_INLINE` relationship.
//
// The frozen configuration defines HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC (1) and
// HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC (1), so the header's `STAT_TIMESPEC (st, st_atim)`
// is `(st)->st_atimespec` - the port reads the `*timespec` members of `sys_stat.in.rs`'s
// `struct stat` directly, and the private macros (STAT_TIMESPEC, STAT_TIMESPEC_NS,
// STAT_TIMESPEC_OFFSETOF, _gl_make_timespec) are recorded in
// docs/ports/excluded-macros.txt because Rust spells them as member access and
// `mem::offset_of!`.  `HAVE_STRUCT_STAT_ST_ATIMENSEC` and the other arms are dead text
// (docs/ports/macro-body-notes.tsv).

include!("time.in.rs");        // <time.h> (`struct timespec`)
include!("sys_stat.in.rs");    // <sys/stat.h> (`struct stat`)
include!("errno.rs");          // <errno.h> (EOVERFLOW)
include!("stdcountof.rs");     // <stdcountof.h> (countof)

pub const STAT_TIME_H: bool = true;

/* Return the nanosecond component of *ST's access time.  */
#[inline(always)]
pub unsafe fn get_stat_atime_ns (st: *const stat) -> c_long
{
  return (*st).st_atimespec.tv_nsec;
}

/* Return the nanosecond component of *ST's status change time.  */
#[inline(always)]
pub unsafe fn get_stat_ctime_ns (st: *const stat) -> c_long
{
  return (*st).st_ctimespec.tv_nsec;
}

/* Return the nanosecond component of *ST's data modification time.  */
#[inline(always)]
pub unsafe fn get_stat_mtime_ns (st: *const stat) -> c_long
{
  return (*st).st_mtimespec.tv_nsec;
}

/* Return the nanosecond component of *ST's birth time.  */
#[inline(always)]
pub unsafe fn get_stat_birthtime_ns (st: *const stat) -> c_long
{
  return (*st).st_birthtimespec.tv_nsec;
}

/* Return *ST's access time.  */
#[inline(always)]
pub unsafe fn get_stat_atime (st: *const stat) -> timespec
{
  return (*st).st_atimespec;
}

/* Return *ST's status change time.  */
#[inline(always)]
pub unsafe fn get_stat_ctime (st: *const stat) -> timespec
{
  return (*st).st_ctimespec;
}

/* Return *ST's data modification time.  */
#[inline(always)]
pub unsafe fn get_stat_mtime (st: *const stat) -> timespec
{
  return (*st).st_mtimespec;
}

/* Return *ST's birth time, if available; otherwise return a value
   with tv_sec and tv_nsec both equal to -1.  */
#[inline(always)]
pub unsafe fn get_stat_birthtime (st: *const stat) -> timespec
{
  let mut t: timespec;

  /* C 186-188's live arm: HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC is 1, so `t` is the
     struct's st_birthtimespec.  */
  t = (*st).st_birthtimespec;

  /* C 205-215's arm (the same macro is defined), which guards against FreeBSD/NetBSD
     reporting the absence of knowledge as zero.  */
  if !(t.tv_sec != 0 && 0 <= t.tv_nsec && t.tv_nsec < 1000000000)
    { t = timespec { tv_sec: -1, tv_nsec: -1 }; }

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
  /* C 228-229's condition `((defined __APPLE__ && defined __MACH__) || defined __sun)
     && defined STAT_TIMESPEC_OFFSETOF` holds on the frozen target (darwin-arm64), so the
     body below is the live text; it is what `lib/lstat.c` and `lib/fstatat.c` call.  */
  if result == 0
    {
      let timespec_hz: c_long = 1000000000;
      let ts_off: [c_short; 3] = [ std::mem::offset_of! (stat, st_atimespec) as c_short,
                                   std::mem::offset_of! (stat, st_mtimespec) as c_short,
                                   std::mem::offset_of! (stat, st_ctimespec) as c_short ];
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
