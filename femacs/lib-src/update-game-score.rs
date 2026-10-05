// femacs: port of lib-src/update-game-score.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same error
// model.  `main` is C's entry point; C's `_Noreturn` functions become `-> !` (PORTING.md P-25), and
// C 83-92's forward declarations are dropped (PORTING.md 0.1-12).
//
// **The frozen configuration builds no `update-game-score` object.**  The reference's
// `lib-src/Makefile.in` 165 lists it only under `$(and $(use_gamedir), update-game-score${EXEEXT})`
// and this target has no shared game directory, so `docs/baseline/frozen-build-sources.txt` records
// no object for it - the whole text is carried under the **build-level file gate**
// `feature = "use_gamedir"`, on every item *and on the include graph* (P-86 ⑧).  Two nested
// conditions are carried as well (P-86 ①, both arms):
//
//   * C 51-53's `#ifdef WINDOWSNT` include of `ntlib.h` is `feature = "windowsnt"`;
//   * C 213-219's `#ifdef HAVE_SHARED_GAME_DIR` in `get_prefix` is `feature =
//     "have_shared_game_dir"`.  That macro's *value* is a configure parameter
//     (`--with-gameuser`'s directory), and the frozen table has it undefined, so the arm's path is a
//     **placeholder** marked as such (Owed 1) - the P-88 ② rule: no numbers from memory.
//
// This is the best-evidenced class of the P-86 files: its surface is POSIX, every reader exists on
// this machine, so the arm flip (`--cfg 'feature="use_gamedir"'`) is a real compile of the whole
// body - which is what `docs/ports/update-game-score.md`'s Evidence records (the frozen object is
// the empty 336-byte one; the arm object is not empty).
//
// Two names C takes from headers the port has no mirror for are declared at their use sites
// (P-88 ①): `<inttypes.h>`'s `intmax_t`/`strtoimax`, and `<string.h>`'s `strnlen` (the mirror
// carries `strlen`/`strcmp`/`strcpy`/`strchr`/`strrchr`/`stpcpy`, not this one).
// `<unlocked-io.h>` is handled the way the other units handle it (a note, no item).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_assignments, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

// The include graph's mirrors spell their parameters with the whole `c_*` family (the
// movemail.rs reading): the unit's `use` list has to bring every one of them.
use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
              c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");        // C 34: `#include <config.h>`

#[cfg (feature = "use_gamedir")]
include!("../lib/include/unistd.in.rs");     // C 36: `#include <unistd.h>`
#[cfg (feature = "use_gamedir")]
include!("../lib/include/errno.rs");         // C 37: `#include <errno.h>` (the port's `errno ()`)
/* C 38: `#include <inttypes.h>` - no mirror carries it; the two names the file uses are declared
   at their use sites below (P-88 ①).  */
#[cfg (feature = "use_gamedir")]
include!("../lib/include/limits.in.rs");     // C 39: `#include <limits.h>` (`CHAR_BIT` 8, measured)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/string.rs");        // C 40: `#include <string.h>`
#[cfg (feature = "use_gamedir")]
include!("../lib/include/stdlib.in.rs");     // C 41: `#include <stdlib.h>` (qsort, mkostemp)
include!("../lib/include/stdlib.rs");        // C 41's chain: the gnulib wrapper includes the
                                             // system `<stdlib.h>` (malloc, realloc, free, exit,
                                             // EXIT_SUCCESS/FAILURE), which the `.in` mirror does not
#[cfg (feature = "use_gamedir")]
include!("../lib/include/time.in.rs");       // C 42: `#include <time.h>` (`time`)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/pwd.rs");           // C 43: `#include <pwd.h>` (`getpwuid`, `struct passwd`)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/fcntl.in.rs");      // C 44: `#include <fcntl.h>` (`O_CREAT` …)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/sys_stat.in.rs");   // C 45: `#include <sys/stat.h>` (stat, fstat, fchmod)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/getopt.in.rs");     // C 46: `#include <getopt.h>` (getopt, optarg, optind)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/c-ctype.rs");       // C 48: `#include <c-ctype.h>` (c_isspace)
/* C 49: `#include <unlocked-io.h>` - and this is where the whole file reaches `<stdio.h>`:
   `lib/unlocked-io.h` 39 is `# include <stdio.h>`, so `fprintf`/`stdout`/`fopen`/`fdopen`/`fileno`/
   `fread`/`ferror`/`fclose`/`rename` all come through this line, not through an include the C file
   writes itself.  The port includes the gnulib wrapper's mirror (which maps the `*_unlocked`
   macros onto the ordinary entries for this target, `lib/include/unlocked-io.rs`).  */
#[cfg (feature = "use_gamedir")]
include!("../lib/include/unlocked-io.rs");   // C 49: `#include <unlocked-io.h>` (-> <stdio.h>)
#[cfg (feature = "use_gamedir")]
include!("../lib/include/stdio.in.rs");      // C 49's chain: `lib/unlocked-io.h` 39 -> `<stdio.h>`

/* C 51-53: `#ifdef WINDOWSNT / #include "ntlib.h" / #endif`  */
#[cfg (all (feature = "use_gamedir", feature = "windowsnt"))]
include!("include/ntlib.rs");                // C 52: `#include "ntlib.h"`

/* C 55-57: `#ifndef min / # define min(a,b) ((a) < (b) ? (a) : (b)) / #endif` - the macro C's own
   text defines when the platform has none; the port declares the same-shaped `min!` (the
   `lib/include/min-max.rs` shape, one `## Deviations` entry).  */
#[cfg (feature = "use_gamedir")]
macro_rules! min {
  ($a:expr, $b:expr) => { if ($a) < ($b) { $a } else { $b } };
}

/// C: <inttypes.h>: `typedef long int intmax_t;` on this target (`sizeof` 8, measured with clang),
/// reached through no mirror (P-88 ①).
#[cfg (feature = "use_gamedir")]
pub type intmax_t = c_long;

/// C: <inttypes.h>: `intmax_t strtoimax (const char *nptr, char **endptr, int base);`
#[cfg (feature = "use_gamedir")]
unsafe extern "C" {
  pub fn strtoimax (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t;
}

/// C: <stdlib.h>: `void exit (int status);` - no mirror carries it (the `movemail.rs` 96-99
/// reading keeps it at the use site).
#[cfg (feature = "use_gamedir")]
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

/// C: <string.h>: `size_t strnlen (const char *s, size_t maxlen);` - the mirror carries
/// `strlen`/`strcmp`/`strcpy`/`strchr`/`strrchr`/`stpcpy` and not this one (P-88 ①).
#[cfg (feature = "use_gamedir")]
unsafe extern "C" {
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
}

/// C 214-215: `#ifdef HAVE_SHARED_GAME_DIR / return HAVE_SHARED_GAME_DIR;` - the configure
/// parameter's directory (`--with-gameuser`).  **Placeholder, not this target's value**: the frozen
/// table records the macro as `undef`, so no measurement exists here (Owed 1 names the configure
/// run that would supply it).
#[cfg (all (feature = "use_gamedir", feature = "have_shared_game_dir"))]
const HAVE_SHARED_GAME_DIR: &core::ffi::CStr = c"/usr/local/var/games";   // placeholder

// C 59: `#define MAX_ATTEMPTS 5`
#[cfg (feature = "use_gamedir")]
const MAX_ATTEMPTS: c_int = 5;
// C 60: `#define MAX_DATA_LEN 1024`
#[cfg (feature = "use_gamedir")]
const MAX_DATA_LEN: usize = 1024;

/* C 62's forward declarations (`lock_file`/`unlock_file`) are dropped with C 83-92's four: the
   definitions below are the only spelling Rust needs (PORTING.md 0.1-12).  */

/// C 64-68: `struct score_entry { char *score; char *user_data; };` - measured with clang against
/// this target: `sizeof` 16, `score`@0, `user_data`@8.
#[cfg (feature = "use_gamedir")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct score_entry
{
  pub score: *mut c_char,
  pub user_data: *mut c_char,
}
#[cfg (feature = "use_gamedir")]
const _: () = core::assert! (std::mem::size_of::<score_entry> () == 16);
#[cfg (feature = "use_gamedir")]
const _: () = core::assert! (std::mem::offset_of! (score_entry, score) == 0);
#[cfg (feature = "use_gamedir")]
const _: () = core::assert! (std::mem::offset_of! (score_entry, user_data) == 8);

/// C 83: `#define MAX_SCORES min (PTRDIFF_MAX, SIZE_MAX / sizeof (struct score_entry))` - the
/// port keeps C's expression with its own two spellings (`PTRDIFF_MAX` = `c_long::MAX`,
/// `SIZE_MAX` = `usize::MAX`).
#[cfg (feature = "use_gamedir")]
pub const MAX_SCORES: c_long = min! (c_long::MAX, (usize::MAX / 16) as c_long);

// C 94-99: `static _Noreturn void usage (int err)`
#[cfg (feature = "use_gamedir")]
unsafe fn usage (err: c_int) -> !
{
  // C 96-100: the five `fprintf (stdout, …)` calls, C's adjacent literals being one each.
  fprintf (stdout, c"Usage: update-game-score [-m MAX] [-r] [-d DIR] game/scorefile SCORE DATA\n".as_ptr ());
  fprintf (stdout, c"       update-game-score -h\n".as_ptr ());
  fprintf (stdout, c" -h\t\tDisplay this help.\n".as_ptr ());
  fprintf (stdout, c" -m MAX\t\tLimit the maximum number of scores to MAX.\n".as_ptr ());
  fprintf (stdout, c" -r\t\tSort the scores in increasing order.\n".as_ptr ());
  fprintf (stdout, c" -d DIR\t\tStore scores in DIR (only if not setuid).\n".as_ptr ());
  exit (err);
}

// C 101-106: `static _Noreturn void lose (const char *msg)`
#[cfg (feature = "use_gamedir")]
unsafe fn lose (msg: *const c_char) -> !
{
  fprintf (stderr, c"%s\n".as_ptr (), msg);
  exit (EXIT_FAILURE);
}

// C 108-114: `static _Noreturn void lose_syserr (const char *msg)`
#[cfg (feature = "use_gamedir")]
unsafe fn lose_syserr (msg: *const c_char) -> !
{
  fprintf (stderr, c"%s: %s\n".as_ptr (), msg,
           if *errno () != 0 { strerror (*errno ()) } else { c"Invalid data in score file".as_ptr () as *mut c_char });
  exit (EXIT_FAILURE);
}

// C 116-129: `static char *get_user_id (void)`
#[cfg (feature = "use_gamedir")]
unsafe fn get_user_id () -> *mut c_char
{
  let buf: *mut passwd = getpwuid (getuid ());
  if buf.is_null () || !strchr ((*buf).pw_name, b' ' as c_int).is_null ()
    || !strchr ((*buf).pw_name, b'\n' as c_int).is_null ()
    {
      let uid: intmax_t = getuid () as intmax_t;
      let name: *mut c_char = malloc ((core::mem::size_of::<intmax_t> ()
                                       * CHAR_BIT as usize / 3 + 4) as usize) as *mut c_char;
      if !name.is_null ()
        {
          // C 124: `sprintf (name, "%"PRIdMAX, uid);` - `PRIdMAX` is `"jd"` on this target
          // (measured with clang: `printf '#include <inttypes.h>\nPRIdMAX\n' | clang -E -`).
          sprintf (name, c"%jd".as_ptr (), uid);
        }
      return name;
    }
  (*buf).pw_name
}

// C 131-147: `static const char *get_prefix (bool privileged, const char *user_prefix)`
#[cfg (feature = "use_gamedir")]
unsafe fn get_prefix (privileged: bool, user_prefix: *const c_char) -> *const c_char
{
  if privileged
    {
      /* C 213-219: the two arms are `feature = "have_shared_game_dir"` and its absence (P-86 ①);
         the value the arm returns is the configure parameter's directory (a placeholder, Owed 1).  */
      #[cfg (feature = "have_shared_game_dir")]
      {
        return HAVE_SHARED_GAME_DIR.as_ptr ();
      }
      #[cfg (not (feature = "have_shared_game_dir"))]
      {
        lose (c"This program was compiled without HAVE_SHARED_GAME_DIR,\nand should not run with elevated privileges.".as_ptr ());
      }
    }
  if user_prefix.is_null ()
    {
      lose (c"Not using a shared game directory, and no prefix given.".as_ptr ());
    }
  user_prefix
}

// C 150-179: `static char *normalize_integer (char *num)`
#[cfg (feature = "use_gamedir")]
unsafe fn normalize_integer (num: *mut c_char) -> *mut c_char
{
  let mut num = num;
  let mut neg: bool;
  let mut p: *mut c_char;
  // C 153-155
  while *num != b'\n' as c_char && c_isspace (*num as c_int)
    {
      num = num.offset (1);
    }
  neg = *num == b'-' as c_char;
  if neg || *num == b'-' as c_char
    {
      num = num.offset (1);
    }

  // C 158-165
  if *num == b'0' as c_char
    {
      loop
        {
          num = num.offset (1);
          if ! (*num == b'0' as c_char)
            {
              break;
            }
        }
      neg = neg && *num != 0;
      if *num == 0
        {
          num = num.offset (-1);
        }
    }

  // C 167-168
  p = num;
  while b'0' as c_char <= *p && *p <= b'9' as c_char
    {
      p = p.offset (1);
    }

  // C 170-175
  if *p != 0 || p == num
    {
      *errno () = 0;
      return core::ptr::null_mut ();
    }

  // C 177-178
  if neg
    {
      num = num.offset (-1);
      *num = b'-' as c_char;
    }
  num
}

/// C: <stdlib.h>: `void qsort (void *base, size_t nmemb, size_t size, int (*compar) (const void *,
/// const void *));` - no mirror carries it (`stdlib.in.rs` 221 has only `qsort_r`, the BSD-signature
/// form), so the unit declares it at the use site (P-88 ①).
#[cfg (feature = "use_gamedir")]
unsafe extern "C" {
  pub fn qsort (base: *mut c_void, nmemb: usize, size: usize,
                compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
}
// C 181-287: `int main (int argc, char **argv)`
#[cfg (feature = "use_gamedir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  // C 183-193: the locals.  PORTING.md 1.5: `newscore`, `scorecount`/`scorealloc` and `lockstate`
  // are written by their first reader before use, so the port zero-initialises them and says so.
  let mut c: c_int;                                              /* C 183 */
  let running_suid: bool;                                        /* C 184 */
  let running_sgid: bool;
  let mut lockstate: *mut c_void = core::ptr::null_mut ();       /* C 185 */
  let scorefile: *mut c_char;                                    /* C 186 */
  let mut end: *mut c_char = core::ptr::null_mut ();             /* C 187 (PORTING.md 1.5) */
  let mut nl: *mut c_char;
  let user: *mut c_char;
  let mut data: *mut c_char;
  let prefix: *const c_char;                                     /* C 188 */
  let mut user_prefix: *const c_char = core::ptr::null ();       /* C 188: `= NULL` */
  let mut scores: *mut score_entry = core::ptr::null_mut ();     /* C 189 (PORTING.md 1.5) */
  let mut newscore: score_entry = core::mem::zeroed ();          /* C 190 */
  let mut reverse: bool = false;                                 /* C 191 */
  let mut scorecount: c_long = 0;                                /* C 192 */
  let mut scorealloc: c_long = 0;
  let mut max_scores: c_long = MAX_SCORES;                       /* C 193 */

  /* C 195-214: `while ((c = getopt (argc, argv, "hrm:d:")) != -1) switch (c) { … }` - the port's
     `loop` with the test moved to the end keeps C's one call per iteration and the switch is the
     `match` (PORTING.md 2.2); the `case` labels keep C's characters as the port spells them.  */
  loop
    {
      c = getopt (argc, argv, c"hrm:d:".as_ptr ());
      if c == -1
        {
          break;
        }
      match c
        {
          // C 199-201: `case 'h': usage (EXIT_SUCCESS); break;`
          104 /* 'h' */ => { usage (EXIT_SUCCESS); }
          // C 202-204: `case 'd': user_prefix = optarg; break;`
          100 /* 'd' */ => { user_prefix = optarg; }
          // C 205-207: `case 'r': reverse = 1; break;`
          114 /* 'r' */ => { reverse = true; }
          // C 208-216: `case 'm': { intmax_t m = strtoimax (optarg, &end, 10); … }`
          109 /* 'm' */ =>
            {
              let m: intmax_t = strtoimax (optarg, &raw mut end, 10);
              if optarg == end || *end != 0 || m < 0
                {
                  usage (EXIT_FAILURE);
                }
              max_scores = min! (m as c_long, MAX_SCORES);
            }
          // C 217-218: `default: usage (EXIT_FAILURE);`
          _ => { usage (EXIT_FAILURE); }
        }
    }

  // C 220-221: `if (argc - optind != 3) usage (EXIT_FAILURE);`
  if argc - optind != 3
    {
      usage (EXIT_FAILURE);
    }

  // C 223-226
  running_suid = getuid () != geteuid ();
  running_sgid = getgid () != getegid ();
  if running_suid && running_sgid
    {
      lose (c"This program can run either suid or sgid, but not both.".as_ptr ());
    }

  // C 228
  prefix = get_prefix (running_suid || running_sgid, user_prefix);

  // C 230-236
  scorefile = malloc ((strlen (prefix) + strlen (*argv.offset (optind as isize)) + 2) as usize)
    as *mut c_char;
  if scorefile.is_null ()
    {
      lose_syserr (c"Couldn't allocate score file".as_ptr ());
    }

  /* C 238-240: `char *z = stpcpy (scorefile, prefix); *z++ = '/'; strcpy (z, argv[optind]);` - the
     post-increment is two statements (Rust has no postfix operator).  */
  let mut z: *mut c_char = stpcpy (scorefile, prefix);
  *z = b'/' as c_char;
  z = z.offset (1);
  strcpy (z, *argv.offset (optind as isize));

  // C 242-247
  newscore.score = normalize_integer (*argv.offset ((optind + 1) as isize));
  if newscore.score.is_null ()
    {
      fprintf (stderr, c"%s: Invalid score\n".as_ptr (), *argv.offset ((optind + 1) as isize));
      return EXIT_FAILURE;
    }

  // C 249-255
  user = get_user_id ();
  if user.is_null ()
    {
      lose_syserr (c"Couldn't determine user id".as_ptr ());
    }
  data = *argv.offset ((optind + 2) as isize);
  if strnlen (data, MAX_DATA_LEN + 1) == MAX_DATA_LEN + 1
    {
      *data.offset (MAX_DATA_LEN as isize) = 0;
    }
  nl = strchr (data, b'\n' as c_int);
  if !nl.is_null ()
    {
      *nl = 0;
    }
  newscore.user_data = malloc ((strlen (user) + 1 + strlen (data) + 1) as usize) as *mut c_char;
  if newscore.user_data.is_null ()
    || sprintf (newscore.user_data, c"%s %s".as_ptr (), user, data) < 0
    {
      lose_syserr (c"Memory exhausted".as_ptr ());
    }

  // C 257-258
  if lock_file (scorefile, &raw mut lockstate) < 0
    {
      lose_syserr (c"Failed to lock scores file".as_ptr ());
    }

  // C 260-265
  if read_scores (scorefile, &raw mut scores, &raw mut scorecount, &raw mut scorealloc) < 0
    {
      unlock_file (scorefile, lockstate);
      lose_syserr (c"Failed to read scores file".as_ptr ());
    }
  if push_score (&raw mut scores, &raw mut scorecount, &raw mut scorealloc, &raw const newscore) < 0
    {
      unlock_file (scorefile, lockstate);
      lose_syserr (c"Failed to add score".as_ptr ());
    }
  sort_scores (scores, scorecount, reverse);
  /* C 269-277: limit the number of scores; with reverse sorting the array is advanced past the
     *smallest* ones, otherwise decrementing the count suffices (the smallest is at the end).  */
  if scorecount > max_scores
    {
      if reverse
        {
          scores = scores.offset ((scorecount - max_scores) as isize);
        }
      scorecount = max_scores;
    }
  // C 278-283
  if write_scores (scorefile, if running_sgid { 0o664 } else { 0o644 }, scores, scorecount) < 0
    {
      unlock_file (scorefile, lockstate);
      lose_syserr (c"Failed to write scores file".as_ptr ());
    }
  // C 284-286
  if unlock_file (scorefile, lockstate) < 0
    {
      lose_syserr (c"Failed to unlock scores file".as_ptr ());
    }
  exit (EXIT_SUCCESS);
}

// C 289-303: `static char *read_score (char *p, struct score_entry *score)`
#[cfg (feature = "use_gamedir")]
unsafe fn read_score (p: *mut c_char, score: *mut score_entry) -> *mut c_char
{
  let mut p = p;
  (*score).score = p;
  p = strchr (p, b' ' as c_int);
  if p.is_null ()
    {
      return p;
    }
  *p = 0;
  p = p.offset (1);
  (*score).user_data = p;
  p = strchr (p, b'\n' as c_int);
  if p.is_null ()
    {
      return p;
    }
  *p = 0;
  p = p.offset (1);
  p
}

// C 308-359: `static int read_scores (const char *filename, struct score_entry **scores,
//             ptrdiff_t *count, ptrdiff_t *alloc)`
#[cfg (feature = "use_gamedir")]
unsafe fn read_scores (filename: *const c_char, scores: *mut *mut score_entry,
                       count: *mut c_long, alloc: *mut c_long) -> c_int
{
  let mut p: *mut c_char;
  let filedata: *mut c_char;
  let mut filesize: c_long;
  let nread: c_long;
  let mut st: stat = core::mem::zeroed ();                    /* C 314 (PORTING.md 1.5) */
  let f: *mut FILE = fopen (filename, c"r".as_ptr ());
  if f.is_null ()
    {
      return -1;
    }
  if fstat (fileno (f), &raw mut st) != 0
    {
      return -1;
    }
  // C 320-324: `if (! (0 <= st.st_size && st.st_size < min (PTRDIFF_MAX, SIZE_MAX))) { errno = EOVERFLOW; return -1; }`
  if ! (0 <= st.st_size
        && (st.st_size as u64) < min! (c_long::MAX as u64, usize::MAX as u64))
    {
      *errno () = EOVERFLOW;
      return -1;
    }
  filesize = st.st_size as c_long;
  filedata = malloc ((filesize + 1) as usize) as *mut c_char;
  if filedata.is_null ()
    {
      return -1;
    }
  nread = fread (filedata as *mut c_void, 1, (filesize + 1) as usize, f) as c_long;
  // C 331-335
  if filesize < nread
    {
      *errno () = 0;
      return -1;
    }
  if nread < filesize
    {
      filesize = nread;
    }
  // C 336-337
  if ferror (f) != 0 || fclose (f) != 0
    {
      return -1;
    }
  *filedata.offset (filesize as isize) = 0;
  // C 339-343
  if strlen (filedata) != filesize as usize
    {
      *errno () = 0;
      return -1;
    }

  // C 345-347
  *scores = core::ptr::null_mut ();
  *count = 0;
  *alloc = 0;
  // C 348-358
  p = filedata;
  while p < filedata.offset (filesize as isize)
    {
      let mut entry: score_entry = core::mem::zeroed ();        /* C 350 (PORTING.md 1.5) */
      p = read_score (p, &raw mut entry);
      if p.is_null ()
        {
          *errno () = 0;
          return -1;
        }
      if push_score (scores, count, alloc, &raw const entry) < 0
        {
          return -1;
        }
    }
  0
}

// C 361-390: `static int score_compare (const void *a, const void *b)` - a `qsort` comparator, so
// the port keeps C's linkage (`extern "C"`, the `sort_scores` call below takes its address).
#[cfg (feature = "use_gamedir")]
unsafe extern "C" fn score_compare (a: *const c_void, b: *const c_void) -> c_int
{
  let sa: *const score_entry = a as *const score_entry;
  let sb: *const score_entry = b as *const score_entry;
  let mut sca: *mut c_char = (*sa).score;
  let mut scb: *mut c_char = (*sb).score;
  let lena: usize;
  let lenb: usize;
  let nega: bool = *sca == b'-' as c_char;
  let negb: bool = *scb == b'-' as c_char;
  let diff: c_int = nega as c_int - negb as c_int;
  if diff != 0
    {
      return diff;
    }
  if nega
    {
      let tmp: *mut c_char = sca;
      sca = scb.offset (1);
      scb = tmp.offset (1);
    }
  lena = strlen (sca);
  lenb = strlen (scb);
  if lena != lenb
    {
      return if lenb < lena { -1 } else { 1 };
    }
  strcmp (scb, sca)
}

// C 392-397: `static int score_compare_reverse (const void *a, const void *b)`
#[cfg (feature = "use_gamedir")]
unsafe extern "C" fn score_compare_reverse (a: *const c_void, b: *const c_void) -> c_int
{
  score_compare (b, a)
}

// C 399-431: `int push_score (struct score_entry **scores, ptrdiff_t *count, ptrdiff_t *size,
//             struct score_entry const *newscore)` - non-`static` in C, so `#[unsafe(no_mangle)]`.
#[cfg (feature = "use_gamedir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn push_score (scores: *mut *mut score_entry, count: *mut c_long,
                                     size: *mut c_long, newscore: *const score_entry) -> c_int
{
  let mut newscores: *mut score_entry = *scores;
  if *count == *size
    {
      let mut newsize: c_long = *size;
      if newsize <= 0
        {
          newsize = 1;
        }
      else if newsize <= MAX_SCORES / 2
        {
          newsize *= 2;
        }
      else if newsize < MAX_SCORES
        {
          newsize = MAX_SCORES;
        }
      else
        {
          *errno () = ENOMEM;
          return -1;
        }
      newscores = realloc (newscores as *mut c_void,
                           core::mem::size_of::<score_entry> () * newsize as usize) as *mut score_entry;
      if newscores.is_null ()
        {
          return -1;
        }
      *scores = newscores;
      *size = newsize;
    }
  *newscores.offset (*count as isize) = *newscore;
  *count += 1;
  0
}

// C 433-439: `static void sort_scores (struct score_entry *scores, ptrdiff_t count, bool reverse)`
#[cfg (feature = "use_gamedir")]
unsafe fn sort_scores (scores: *mut score_entry, count: c_long, reverse: bool)
{
  qsort (scores as *mut c_void, count as usize, core::mem::size_of::<score_entry> (),
         Some (if reverse { score_compare_reverse } else { score_compare }));
}

// C 441-469: `static int write_scores (const char *filename, mode_t mode,
//             const struct score_entry *scores, ptrdiff_t count)`
#[cfg (feature = "use_gamedir")]
unsafe fn write_scores (filename: *const c_char, mode: u32,
                        scores: *const score_entry, count: c_long) -> c_int
{
  let fd: c_int;
  let f: *mut FILE;
  let mut i: c_long;
  let tempfile: *mut c_char
    = malloc ((strlen (filename) + strlen (c".tempXXXXXX".as_ptr ()) + 1) as usize) as *mut c_char;
  if tempfile.is_null ()
    {
      return -1;
    }
  strcpy (stpcpy (tempfile, filename), c".tempXXXXXX".as_ptr ());
  fd = mkostemp (tempfile, 0);
  if fd < 0
    {
      return -1;
    }
  /* C 453-455: `#ifndef DOS_NT / if (fchmod (fd, mode) != 0) return -1; / #endif` - `DOS_NT` is a
     registered feature (`dos_nt`), so both arms are carried (P-86 ①).  */
  #[cfg (not (feature = "dos_nt"))]
  {
    if fchmod (fd, mode as mode_t) != 0
      {
        return -1;
      }
  }
  f = fdopen (fd, c"w".as_ptr ());
  if f.is_null ()
    {
      return -1;
    }
  i = 0;
  while i < count
    {
      if fprintf (f, c"%s %s\n".as_ptr (), (*scores.offset (i as isize)).score,
                  (*scores.offset (i as isize)).user_data) < 0
        {
          return -1;
        }
      i += 1;
    }
  if fclose (f) != 0
    {
      return -1;
    }
  if rename (tempfile, filename) != 0
    {
      return -1;
    }
  0
}

// C 471-502: `static int lock_file (const char *filename, void **state)`
#[cfg (feature = "use_gamedir")]
unsafe fn lock_file (filename: *const c_char, state: *mut *mut c_void) -> c_int
{
  let mut fd: c_int;
  let mut buf: stat = core::mem::zeroed ();                     /* C 474 (PORTING.md 1.5) */
  let mut attempts: c_int = 0;
  let lockext: *const c_char = c".lockfile".as_ptr ();
  let lockpath: *mut c_char
    = malloc ((strlen (filename) + strlen (lockext) + 60) as usize) as *mut c_char;
  if lockpath.is_null ()
    {
      return -1;
    }
  strcpy (stpcpy (lockpath, filename), lockext);
  *state = lockpath as *mut c_void;

  // C 484-500: `while ((fd = open (lockpath, O_CREAT | O_EXCL, 0600)) < 0) { … }`
  loop
    {
      fd = open (lockpath, O_CREAT | O_EXCL, 0o600 as c_int);
      if ! (fd < 0)
        {
          break;
        }
      if *errno () != EEXIST
        {
          return -1;
        }
      attempts += 1;

      /* C 488-499: break the lock if it is over an hour old, or after MAX_ATTEMPTS tries; the
         port keeps C's one condition and its two actions.  */
      if MAX_ATTEMPTS < attempts
        /* C 494: `buf.st_ctime` - macOS spells the macro `#define st_ctime st_ctimespec.tv_sec`,
           so the port names the member C's macro names (the mirror's `stat` has `st_ctimespec`).  */
        || (stat (lockpath, &raw mut buf) == 0
            && 60 * 60 < time (core::ptr::null_mut ()) - buf.st_ctimespec.tv_sec)
        {
          if unlink (lockpath) != 0 && *errno () != ENOENT
            {
              return -1;
            }
          attempts = 0;
        }
      else
        {
          sleep (1);
        }
    }

  close (fd);
  0
}

// C 504-514: `static int unlock_file (const char *filename, void *state)`
#[cfg (feature = "use_gamedir")]
unsafe fn unlock_file (filename: *const c_char, state: *mut c_void) -> c_int
{
  let lockpath: *mut c_char = state as *mut c_char;
  let saved_errno: c_int = *errno ();
  let ret: c_int = unlink (lockpath);
  if ! (ret < 0 && *errno () != ENOENT)
    {
      *errno () = saved_errno;
    }
  free (lockpath as *mut c_void);
  ret
}

/* update-game-score.c ends here */
