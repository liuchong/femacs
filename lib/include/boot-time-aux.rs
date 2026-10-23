// femacs: mirror of lib/boot-time-aux.h @ fd7afced403.
//
// The auxiliary boot-time helpers `lib/boot-time.c` gets by including this header (C 61 of
// that file; measured as its only includer).  The header has **no include guard** (C 18 is
// blank and C 19 opens the first arm), so the mirror has no guard constant either
// (PORTING.md 0.1).
//
// The frozen darwin-arm64 configuration defines none of `__linux__`, `__ANDROID__`,
// `__OpenBSD__`, `__HAIKU__`, `__CYGWIN__`, `_WIN32`, its `__GLIBC__`/`__GLIBC_MINOR__` are
// undefined and HAVE_OS_H is 0, so the one arm C compiles is C 230-256's sysctl fallback
// (`get_bsd_boot_time_final_fallback`, the same gate `lib/boot-time.c` 230-232 repeats);
// every other arm is dead text carried under the feature that names the C condition's
// non-frozen side (PORTING.md P-51), with the feature names `lib/boot-time.rs` already
// uses (`linux_or_android`, `linux_no_android`, `android`, `openbsd`, `haiku`, `os_h`,
// `cygwin_win32`, `no_cygwin`) plus the four the header adds (`no_glibc_boottime` for
// C 30's glibc predicate, `no_glibc_realtime` for C 138's, `win32_vista` for C 354's
// `_WIN32_WINNT` arm and `gnuc_4_2` for C 383's `_GL_GNUC_PREREQ`).
//
// Their platform names have no mirror - `sysinfo`/`struct sysinfo` (`<sys/sysinfo.h>`),
// `system_info`/`get_system_info` (`<OS.h>`), `LoadLibrary`/`GetProcAddress`/`GetTickCount64`
// and the `ULONGLONG`/`BOOL`/`HMODULE` spellings (`<windows.h>`), `clock_gettime`/
// `CLOCK_BOOTTIME`/`timespec_get`/`TIME_UTC`/`gettimeofday` (`<time.h>`/`<sys/time.h>`'s
// clock and timer half, which the port's mirrors do not carry yet), `_WIN32_WINNT` - and
// nothing compiles the arms that name them, so they are named at the use site without
// declarations, the `fopen`/`_setmode` precedent (`lib/boot-time.rs`'s note, PORTING.md
// P-51).  What the `have_haiku` arm needs *is* declared, because `xtask dead-arms` compiles that
// one: `struct stat`, `stat` and `get_stat_mtime`, included below.
//
// C 16's `Written by Bruno Haible` comment and C 1-15's opening block are the file's own
// text and its declaration, not code; C 23's `_GL_ATTRIBUTE_MAYBE_UNUSED` is gnulib's
// attribute macro, recorded rather than spelled (docs/ports/excluded-macros.txt).

include!("time.in.rs");        // <time.h> (`struct timespec`, `time_t`)
include!("sys_time.rs");       // <sys/time.h> (`struct timeval`, `gettimeofday`)
include!("sys_types.in.rs");   // <sys/types.h>
include!("sys_stat.in.rs");    // <sys/stat.h> (`struct stat`, `stat`, `S_ISDIR!`)
include!("stat-time.rs");      // "stat-time.h" (`get_stat_mtime`)
include!("idx.rs");            // "idx.h" (`idx_t`)
include!("stdcountof.rs");     // <stdcountof.h> (`countof!`)
include!("stdio.rs");          // <stdio.h> (`FILE`, `fopen`, `fread`, `fclose`)
include!("sys_sysctl.rs");     // <sys/sysctl.h> (`sysctl`)

/* `<sys/sysinfo.h>`, which gives C 68's arm its `struct sysinfo`: gnulib's `boot-time-aux.h` uses
   it without including the header (the configuration's own chain brings it in - `HAVE_SYS_SYSINFO_H`,
   and bionic's file includes `<linux/kernel.h>` for the struct).  The arm is dead on the frozen
   target, so nothing here needed the mirror until the Android configuration compiled it.  */
#[cfg (feature = "have_sys_sysinfo_h")]
include!("sys_sysinfo.rs");

/* C 230-232's gate names two <sys/sysctl.h> rows that the port has no mirror for: `CTL_KERN`
   (1) and `KERN_BOOTTIME` (21) are the frozen target's values (the clang probe in
   docs/ports/boot-time.md), and lib/boot-time.rs declared them until the header became a
   mirror of its own.  */
pub const CTL_KERN: c_int = 1;
pub const KERN_BOOTTIME: c_int = 21;

/* C 19's gate is a *platform* predicate - `#if defined __linux__ || defined __ANDROID__` - so the
   port spells it as one (PORTING.md P-290: platform conditions are `target_os`, not config
   macros).  It was a derived `feature = "linux_or_android"` that no configuration emitted, which
   made the whole block dead everywhere and left the Android sweep failing on `get_linux_uptime`
   (status 4.708).  */

/* C 21-22: "Store the uptime counter, as managed by the Linux kernel, in *P_UPTIME.  Return 0
   upon success, -1 upon failure."  */
#[cfg (any (target_os = "linux", target_os = "android"))]
unsafe fn get_linux_uptime (p_uptime: *mut timespec) -> c_int
{
  /* C 27-29: "The clock_gettime facility returns the uptime with a resolution of 1 µsec.  It
     is available with glibc >= 2.14, Android, or musl libc.  In glibc < 2.17 it required
     linking with librt."  C 30's `# if !defined __GLIBC__ || 2 < __GLIBC__ + (17 <=
     __GLIBC_MINOR__)` is the predicate the feature stands for.  */
  #[cfg (feature = "no_glibc_boottime")]
  if clock_gettime (CLOCK_BOOTTIME, p_uptime) >= 0
    { return 0; }

  /* C 36: `# if !defined __ANDROID__` - /proc/uptime has no read permission on Android.  */
  #[cfg (not (feature = "have_android"))]
  {
    let mut fp: *mut FILE = fopen (c"/proc/uptime".as_ptr (), c"re".as_ptr ());
    if !fp.is_null ()
      {
        let mut buf: [c_char; 32 + 1] = [0; 32 + 1];
        let n: usize = fread (buf.as_mut_ptr () as *mut c_void, 1,
                              std::mem::size_of_val (&buf) - 1, fp);
        fclose (fp);
        if n > 0
          {
            buf[n] = 0;      /* C 47: `buf[n] = '\0';` */
            /* C 48: "buf now contains two values: the uptime and the idle time."  */
            let mut s: time_t = 0;
            let mut p: *mut c_char = buf.as_mut_ptr ();
            while b'0' as c_char <= *p && *p <= b'9' as c_char
              {
                s = 10 * s + (*p - b'0' as c_char) as time_t;
                p = p.add (1);
              }
            if buf.as_ptr () < p as *const c_char
              {
                let mut ns: c_long = 0;
                if *p == b'.' as c_char
                  {
                    p = p.add (1);
                    let mut i: c_int = 0;
                    while i < 9
                      {
                        ns = 10 * ns
                          + (if b'0' as c_char <= *p && *p <= b'9' as c_char
                             { let d: c_long = (*p - b'0' as c_char) as c_long;
                               p = p.add (1); d }
                             else { 0 });
                        i += 1;
                      }
                  }
                (*p_uptime).tv_sec = s;
                (*p_uptime).tv_nsec = ns;
                return 0;
              }
          }
      }
  }

  /* C 66: `# if HAVE_DECL_SYSINFO /* not available in Android API < 9 */`.  */
  #[cfg (feature = "have_decl_sysinfo")]
  {
    /* C 68: "The sysinfo call returns the uptime with a resolution of 1 sec only."  */
    let mut info: sysinfo = std::mem::zeroed ();
    if sysinfo (&mut info) >= 0
      {
        (*p_uptime).tv_sec = info.uptime;
        (*p_uptime).tv_nsec = 0;
        return 0;
      }
  }

  return -1;
}

/* C 82-169: `#if defined __linux__ && !defined __ANDROID__` - dead on this target, carried.  */

/* C 87-98: "On Devuan with the 'runit' init system and on Artix with the 's6' init system,
   UTMP_FILE contains USER_PROCESS and other entries, but no BOOT_TIME entry.  On Alpine
   Linux, UTMP_FILE is not filled.  It is always empty.  So, in both cases, get the time
   stamp of a file that gets touched only during the boot process."  */
#[cfg (feature = "linux_no_android")]
unsafe fn get_linux_boot_time_fallback (p_boot_time: *mut timespec) -> c_int
{
  let boot_touched_files: [&[u8]; 4] =
    [
      b"/var/lib/systemd/random-seed\0",   /* seen on distros with systemd */
      b"/var/lib/urandom/random-seed\0",   /* seen on Devuan with runit */
      b"/var/lib/random-seed\0",           /* seen on Artix with s6 */
      /* C 105: "This must come last, since on several distros /var/run/utmp is modified when
         a user logs in, i.e. long after boot." - seen on Alpine Linux with OpenRC */
      b"/var/run/utmp\0",
    ];
  let mut i: idx_t = 0;
  while i < countof! (boot_touched_files) as idx_t
    {
      let filename: *const c_char = boot_touched_files[i as usize].as_ptr () as *const c_char;
      let mut statbuf: stat = std::mem::zeroed ();
      if stat (filename, &mut statbuf) >= 0
        {
          let boot_time: timespec = get_stat_mtime (&statbuf);
          /* C 114-118: "On Alpine 3.20.0_rc2 /var/run/utmp was observed with bogus
             timestamps of ~10 s.  Reject timestamps before 2005-07-25 23:34:15 UTC
             (1122334455), as neither Alpine nor Devuan existed then."  */
          if boot_time.tv_sec >= 1122334455
            {
              *p_boot_time = boot_time;
              return 0;
            }
        }
      i += 1;
    }
  return -1;
}

/* C 125-130: "The following approach is only usable as a fallback, because it is of the form
     boot_time = (time now) - (kernel's ktime_get_boottime[_ts64] ())
   and therefore produces wrong values after the date has been bumped in the running system,
   which happens frequently if the system is running in a virtual machine and this VM has been
   put into "saved" or "sleep" state and then resumed."  */
#[cfg (feature = "linux_no_android")]
unsafe fn get_linux_boot_time_final_fallback (p_boot_time: *mut timespec) -> c_int
{
  let mut uptime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
  if get_linux_uptime (&mut uptime) >= 0
    {
      let mut result: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
      /* C 138: `# if !defined __GLIBC__ || 2 < __GLIBC__ + (16 <= __GLIBC_MINOR__)`.  */
      #[cfg (feature = "no_glibc_realtime")]
      {
        /* C 139-142: "Better than: `if (0 <= clock_gettime (CLOCK_REALTIME, &result))`
           because timespec_get does not need -lrt in glibc 2.16."  */
        if timespec_get (&mut result, TIME_UTC) == 0
          { return -1; }
      }
      #[cfg (not (feature = "no_glibc_realtime"))]
      {
        /* C 146-147: "Fall back on lower-res approach that does not need -lrt.  This is good
           enough; on these hosts UPTIME is even lower-res."  */
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let r: c_int = gettimeofday (&mut tv, std::ptr::null_mut ());
        if r < 0
          { return r; }
        result.tv_sec = tv.tv_sec;
        result.tv_nsec = tv.tv_usec * 1000;
      }

      if result.tv_nsec < uptime.tv_nsec
        {
          result.tv_nsec += 1000000000;
          result.tv_sec -= 1;
        }
      result.tv_sec -= uptime.tv_sec;
      result.tv_nsec -= uptime.tv_nsec;
      *p_boot_time = result;
      return 0;
    }
  return -1;
}

/* C 171-200: `#if defined __ANDROID__` - dead on this target, carried.  */

/* C 176-180: "On Android, there is no /var, and normal processes don't have access to system
   files.  Therefore use the kernel's uptime counter, although it produces wrong values after
   the date has been bumped in the running system."  */
#[cfg (feature = "have_android")]
unsafe fn get_android_boot_time (p_boot_time: *mut timespec) -> c_int
{
  let mut uptime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
  if get_linux_uptime (&mut uptime) >= 0
    {
      let mut result: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
      if clock_gettime (CLOCK_REALTIME, &mut result) >= 0
        {
          if result.tv_nsec < uptime.tv_nsec
            {
              result.tv_nsec += 1000000000;
              result.tv_sec -= 1;
            }
          result.tv_sec -= uptime.tv_sec;
          result.tv_nsec -= uptime.tv_nsec;
          *p_boot_time = result;
          return 0;
        }
    }
  return -1;
}

/* C 202-228: `#if defined __OpenBSD__` - dead on this target, carried.  */

/* C 207-209: "On OpenBSD, UTMP_FILE is not filled.  It contains only dummy entries.  So, get
   the time stamp of a file that gets touched only during the boot process."  */
#[cfg (feature = "openbsd")]
unsafe fn get_openbsd_boot_time (p_boot_time: *mut timespec) -> c_int
{
  let boot_touched_files: [&[u8]; 2] = [b"/var/db/host.random\0", b"/var/run/utmp\0"];
  let mut i: idx_t = 0;
  while i < countof! (boot_touched_files) as idx_t
    {
      let filename: *const c_char = boot_touched_files[i as usize].as_ptr () as *const c_char;
      let mut statbuf: stat = std::mem::zeroed ();
      if stat (filename, &mut statbuf) >= 0
        {
          *p_boot_time = get_stat_mtime (&statbuf);
          return 0;
        }
      i += 1;
    }
  return -1;
}

/* C 230-256: `#if HAVE_SYS_SYSCTL_H && HAVE_SYSCTL && defined CTL_KERN && defined
   KERN_BOOTTIME && !defined __minix` - **the live arm** (macOS, FreeBSD, GNU/kFreeBSD,
   NetBSD, OpenBSD; C 234: "On Minix 3.3 this sysctl produces garbage results.  Therefore
   avoid it.").  */

/* C 236-239: "The following approach is only usable as a fallback, because it produces wrong
   values after the date has been bumped in the running system, which happens frequently if
   the system is running in a virtual machine and this VM has been put into "saved" or
   "sleep" state and then resumed."  It sits under C 230-232's `#if`, so it carries that gate:
   with the body compiled on Android the unit imported `sysctl` (status 4.749).  */
#[cfg (all (not (feature = "not_have_sys_sysctl_h"), not (feature = "not_have_sysctl")))]
unsafe fn get_bsd_boot_time_final_fallback (p_boot_time: *mut timespec) -> c_int
{
  static mut request: [c_int; 2] = [CTL_KERN, KERN_BOOTTIME];
  let mut result: timeval = timeval { tv_sec: 0, tv_usec: 0 };
  let mut result_len: usize = std::mem::size_of_val (&result);

  if sysctl ((*std::ptr::addr_of_mut! (request)).as_mut_ptr (), 2,
             &mut result as *mut timeval as *mut c_void, &mut result_len,
             std::ptr::null_mut (), 0) >= 0
    {
      (*p_boot_time).tv_sec = result.tv_sec;
      (*p_boot_time).tv_nsec = (result.tv_usec * 1000) as c_long;
      return 0;
    }
  return -1;
}

/* C 258-282: `#if defined __HAIKU__` - dead on this target, carried; `xtask dead-arms` is
   what compiles it (feature `have_haiku`), which is why the names it needs are declared above.  */

/* C 265-272: "On Haiku, /etc/utmp does not exist.  During boot, 1. the current time is
   restored, but possibly with a wrong time zone, that is, with an offset of a few hours,
   2. some symlinks and files get created, 3. the various devices are brought up, in
   particular the network device, 4. the correct date and time is set, 5. some more device
   nodes get created.  The boot time can be retrieved by looking at a directory created
   during phase 5, such as /dev/input."  */
#[cfg (feature = "have_haiku")]
unsafe fn get_haiku_boot_time (p_boot_time: *mut timespec) -> c_int
{
  let boot_touched_file: *const c_char = b"/dev/input\0".as_ptr () as *const c_char;
  let mut statbuf: stat = std::mem::zeroed ();
  if stat (boot_touched_file, &mut statbuf) >= 0
    {
      *p_boot_time = get_stat_mtime (&statbuf);
      return 0;
    }
  return -1;
}

/* C 284-300: `#if HAVE_OS_H /* BeOS, Haiku */` - dead on this target, carried (feature
   `os_h`); `system_info` and `get_system_info` are `<OS.h>`'s, named without a declaration
   because nothing compiles this arm.  */

/* C 286-289 repeats the "only usable as a fallback" note above.  */
#[cfg (feature = "os_h")]
unsafe fn get_haiku_boot_time_final_fallback (p_boot_time: *mut timespec) -> c_int
{
  let mut si: system_info = std::mem::zeroed ();

  get_system_info (&mut si);
  (*p_boot_time).tv_sec = si.boot_time / 1000000;
  (*p_boot_time).tv_nsec = (si.boot_time % 1000000) * 1000;
  return 0;
}

/* C 302-426: `#if defined __CYGWIN__ || defined _WIN32` - dead on this target, carried.  */

/* C 307-311: "On Cygwin, /var/run/utmp is empty.  On native Windows, <utmpx.h> and <utmp.h>
   don't exist.  Instead, on Windows, the boot time can be retrieved by looking at the time
   stamp of a file that (normally) gets touched only during the boot process, namely
   C:\\pagefile.sys."  */
#[cfg (feature = "cygwin_win32")]
unsafe fn get_windows_boot_time (p_boot_time: *mut timespec) -> c_int
{
  /* C 313-321: the two lists C's `#if defined __CYGWIN__ && !defined _WIN32` selects between
     ("It is more portable to use /proc/cygdrive/c than /cygdrive/c" and the Cygwin 3.5.3
     workaround entry / the native `C:\\pagefile.sys`).  */
  #[cfg (not (feature = "no_cygwin"))]
  let boot_touched_files: [&[u8]; 2] =
    [
      b"/proc/cygdrive/c/pagefile.sys\0",
      /* C 317-318: "A fallback, working around a Cygwin 3.5.3 bug.  It has a modification
         time about 1.5 minutes after the last boot; but that's better than nothing."  */
      b"/proc/cygdrive/c/ProgramData/Microsoft/Windows/DeviceMetadataCache/dmrc.idx\0",
    ];
  #[cfg (feature = "no_cygwin")]
  let boot_touched_files: [&[u8]; 1] = [b"C:\\pagefile.sys\0"];

  let mut i: idx_t = 0;
  while i < countof! (boot_touched_files) as idx_t
    {
      let filename: *const c_char = boot_touched_files[i as usize].as_ptr () as *const c_char;
      let mut statbuf: stat = std::mem::zeroed ();
      if stat (filename, &mut statbuf) >= 0
        {
          /* C 330-333: "Work around a Cygwin 3.5.3 bug.
             <https://cygwin.com/pipermail/cygwin/2024-May/255931.html>"  */
          #[cfg (not (feature = "no_cygwin"))]
          if !S_ISDIR! (statbuf.st_mode)
            {
              *p_boot_time = get_stat_mtime (&statbuf);
              return 0;
            }
          #[cfg (feature = "no_cygwin")]
          {
            *p_boot_time = get_stat_mtime (&statbuf);
            return 0;
          }
        }
      i += 1;
    }
  return -1;
}

/* C 341-343: `# ifndef __CYGWIN__` (the whole GetTickCount64 half is native Windows only).  */

/* C 344-384: `#  if !(_WIN32_WINNT >= _WIN32_WINNT_VISTA)` is C 372's "GetTickCount64 is
   only available on Windows Vista and later"; its `#  else` (C 377-384) is
   `#   define GetTickCount64Func GetTickCount64`, whose `-Waddress` suppression (C 383-385)
   the port cannot spell.  */

/* C 346-351: "Don't assume that UNICODE is not defined" - `#   undef LoadLibrary` /
   `#   define LoadLibrary LoadLibraryA` and `#   define GetProcAddress (void *)
   GetProcAddress`, the second "avoiding warnings from gcc -Wcast-function-type".  Both are
   `<windows.h>` renames; the port writes the names C's text uses (`LoadLibrary`,
   `GetProcAddress`) and records the rename here (PORTING.md 0.1: a `#define` has no Rust
   counterpart, and the `unlocked-io.h` precedent records renames at the use site).  */

/* C 377-384: `#  else` of C 344's `#if !(_WIN32_WINNT >= _WIN32_WINNT_VISTA)` -
   `#   define GetTickCount64Func GetTickCount64` (C 383-385's `_GL_GNUC_PREREQ (4, 2)` arm and
   its `#    pragma GCC diagnostic ignored "-Waddress"` are the C compiler's business).  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
#[cfg (feature = "win32_vista")]
macro_rules! GetTickCount64Func { () => { GetTickCount64 () } }

/* C 373: `typedef ULONGLONG (WINAPI * GetTickCount64FuncType) (void);` - `<windows.h>`'s
   `ULONGLONG` and the `WINAPI` (stdcall) convention are named as written; nothing compiles
   this arm.  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
#[cfg (not (feature = "win32_vista"))]
pub type GetTickCount64FuncType = unsafe extern "system" fn () -> ULONGLONG;

/* C 375: `static GetTickCount64FuncType GetTickCount64Func = NULL;` - a C function pointer
   that may be NULL is `Option<...>` in the port.  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
#[cfg (not (feature = "win32_vista"))]
pub static mut GetTickCount64Func: Option<GetTickCount64FuncType> = None;

/* C 376: `static BOOL initialized = FALSE;` - `FALSE`/`TRUE` are `<windows.h>`'s 0 and 1, and
   the arm that names them is dead in every configuration the port builds (the
   `fopen`/`_setmode` precedent).  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
#[cfg (not (feature = "win32_vista"))]
pub static mut initialized: BOOL = FALSE;

/* C 378-387: `static void initialize (void)` - the one-time lookup of the Vista+ entry
   point ("GetTickCount64 is only available on Windows Vista and later", C 372).  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
#[cfg (not (feature = "win32_vista"))]
pub unsafe fn initialize ()
{
  let kernel32: HMODULE = LoadLibrary (b"kernel32.dll\0".as_ptr () as *const c_char);
  if !kernel32.is_null ()
    {
      GetTickCount64Func =
        Some (std::mem::transmute::<*const c_void, GetTickCount64FuncType> (
                GetProcAddress (kernel32, b"GetTickCount64\0".as_ptr () as *const c_char)));
    }
  initialized = TRUE;      /* C 386: `initialized = TRUE;` */
}

/* C 388-426: the GetTickCount64 fallback, "boot time = current time - uptime" (C 388-389:
   "This uses the GetTickCount64 function which is only available on Windows Vista and
   later.").  */
#[cfg (feature = "cygwin_win32")]
#[cfg (not (feature = "no_cygwin"))]
unsafe fn get_windows_boot_time_fallback (p_boot_time: *mut timespec) -> c_int
{
  /* C 392-395: `#  if !(_WIN32_WINNT >= _WIN32_WINNT_VISTA) / if (! initialized) /
     initialize (); / #  endif`.  */
  #[cfg (not (feature = "win32_vista"))]
  if ! initialized
    { initialize (); }

  /* C 396: `if (GetTickCount64Func != NULL)`.  C's two arms reach the same call by name:
     the stateful one tests the pointer, the `win32_vista` one is C 377-384's `# define
     GetTickCount64Func GetTickCount64`.  A Rust-forced shape: the source of the pointer is
     selected by cfg, and `is_some`/`unwrap` is C's own NULL test and call (rule 5.2's
     documented deviations).  */
  #[cfg (not (feature = "win32_vista"))]
  let get_tick_count_64: Option<GetTickCount64FuncType> = GetTickCount64Func;
  #[cfg (feature = "win32_vista")]
  let get_tick_count_64: Option<GetTickCount64FuncType> = Some (GetTickCount64);

  if get_tick_count_64.is_some ()
    {
      let uptime_ms: ULONGLONG = (get_tick_count_64.unwrap ()) ();

      let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
      if gettimeofday (&mut tv, std::ptr::null_mut ()) >= 0
        {
          let mut uptime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
          uptime.tv_sec = (uptime_ms / 1000) as time_t;
          uptime.tv_nsec = ((uptime_ms % 1000) * 1000000) as c_long;

          let mut result: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
          result.tv_sec = tv.tv_sec;
          result.tv_nsec = tv.tv_usec * 1000;
          if result.tv_nsec < uptime.tv_nsec
            {
              result.tv_nsec += 1000000000;
              result.tv_sec -= 1;
            }
          result.tv_sec -= uptime.tv_sec;
          result.tv_nsec -= uptime.tv_nsec;

          *p_boot_time = result;
          return 0;
        }
    }
  return -1;
}
