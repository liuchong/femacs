// femacs: port of exec/test.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// local, macro), the same control flow and the same error model.
//
// **The frozen configuration compiles none of `exec/`** (`docs/baseline/frozen-build-sources.txt`:
// the exec loader belongs to the unexec dump path, and this build is `HAVE_PDUMPER=1`), and PORTING.md
// P-86 governs a file in that state: the whole body is transcribed as it stands instead of being
// recorded as an empty unit - this file's previous revision was the three-line P-08 note.  The file
// has no `#if` of its own, so "both arms" is vacuous here.  The two configuration-shaped surfaces it
// reaches are declared at their use sites, because no mirror in this tree carries them (P-88 ①):
// `<sys/wait.h>`'s four `W*` macros (the `src/process.rs` 1262-1283 shape) and `<unistd.h>`'s `fork`
// (the `lib-src/emacsclient.rs` 292-295 `exit` shape, which records the same gap).
//
// **The file gate is `feature = "have_android"`, on every item *and* on the include graph (P-86 8)** -
// the registered name for the build this file belongs to (`docs/baseline/feature-coverage.tsv` 11, the
// gate `src/androidmenu.rs`, `src/sfntfont-android.rs`, `src/sfnt.rs` and `exec/trace.rs` use).  The
// reading is `configure.ac` 178-220, which configures `exec/` with the build directory set to `exec`
// only inside the `XCONFIGURE=android` block (1343 sets that variable from `--with-android`), and
// `exec/configure.ac` 235, which then puts this object into the `OBJS` list that `exec/Makefile.in`'s
// `libexec.a` rule - and the `loader` rule that links it - are built from; so the frozen configuration
// compiles nothing here, `docs/baseline/frozen-build-sources.txt`'s batch-F section records no `exec/`
// object for this target, `tools/extract-abi.py`'s checklist for this file is empty (P-86 2) and the
// port's frozen object is the 336-byte empty one.  The sibling unit `exec/trace.rs` already carries
// this gate; this round extends it to this file and `exec/exec1.rs`, which is the fix for the gap
// `tools/check-undef.py` read on 2026-10-15 - its seven new rows are the four names the gated sibling
// defines (`after_fork`, `exec_init`, `exec_waitpid`, `tracing_execve`) referenced by this unit's
// object and `exec1.o` (the other three, `user_alloca`, `user_copy` and `__errno_location`, come from
// `exec/exec.rs`, the same class in that unit's own round).  No item is exempt: `main`
// (`#[unsafe(no_mangle)] pub unsafe extern "C" fn main`) carries the gate like every other item, so
// the frozen object defines nothing - the frozen reading, since the reference build compiles no
// `exec/` object at all.  Every `include!` edge is written as the two-line form - the `#[cfg]` line,
// then the `include!` line - which P-86 12 makes a hard requirement until `xtask`'s `gate_states`
// reads the one-line form.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar,
              c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");      // C 20: `#include <config.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/signal.rs");      // C 22: `#include <signal.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdio.rs");       // C 23: `#include <stdio.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/unistd.in.rs");   // C 24: `#include <unistd.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");      // C 25: `#include <stdlib.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/errno.rs");       // C 26: `#include <errno.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");      // C 27: `#include <string.h>`
#[cfg (feature = "have_android")]
include!("include/exec.rs");               // C 31: `#include "exec.h"`

/* C 29: `#include <sys/wait.h>`.  The system header is not part of the reference tree and no mirror
   carries it, so the four macros this file uses are declared here with C's own bodies (PORTING.md
   P-88 ①; the `src/process.rs` 1262-1283 shape, which measured the same values).  */
// C: `#define WIFEXITED(status) (WTERMSIG (status) == 0)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
// C: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }
// C: `#define WIFSTOPPED(status) (((status) & 0xff) == 0x7f)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WIFSTOPPED (status: c_int) -> bool { (status & 0xff) == 0x7f }
// C: `#define WTERMSIG(status) ((status) & 0x7f)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0x7f }

/* C 32-33: `<unistd.h>`'s process entry, which no mirror carries yet (the `lib-src/emacsclient.rs`
   292-295 note records the same gap for `exit`).  */
/// C: `pid_t fork (void);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn fork () -> pid_t;
}

/* C 32-33: `<stdlib.h>`'s `exit`.  `_Noreturn` is the port's `-> !` (PORTING.md 0.2).  */
/// C: `void exit (int);`
#[cfg (feature = "have_android")]
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

/// C 36: `static void print_usage (void)` - C's `static`, so the port keeps it a plain `unsafe fn`
/// (no `#[unsafe(no_mangle)]`, PORTING.md 0.2).
#[cfg (feature = "have_android")]
unsafe fn print_usage ()
{
  fprintf (stderr, c"test loader-name program [args...]\nRun the given program using the specified loader.\n".as_ptr ());
}

/* C 44: `extern char **environ;` - the object is declared by the `<unistd.h>` mirror
   (`../lib/include/unistd.in.rs` 94, the same declaration C's header carries), so C's line has no
   item of its own here.  */

/* C 46-48: "This program uses libexec to wrap the execution of a child process."  */

/// C 50: `int main (int argc, char **argv)` - the process entry point, so the port keeps the name
/// and exports it (`#[unsafe(no_mangle)]`, PORTING.md 0.2).
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  /* C 52: `pid_t pid, child; int sig; sigset_t sigset;` - C leaves the first three uninitialised and
     writes them before any read; the port spells C's own zero value instead (PORTING.md 0.1-13).  */
  let mut pid: pid_t = 0;
  let mut child: pid_t = 0;
  let mut sig: c_int = 0;
  let mut sigset: sigset_t = sigset_t { __val: 0 };

  /* Check that there are a sufficient number of arguments.  */

  if argc < 3
    {
      print_usage ();
      return 1;
    }

  exec_init (*argv.offset (1));

  /* Block SIGCHLD to avoid reentrant modification of the child
     process list.  */

  sigemptyset (&raw mut sigset);
  sigaddset (&raw mut sigset, SIGCHLD);
  sigprocmask (SIG_BLOCK, &raw const sigset, std::ptr::null_mut ());

  pid = fork ();
  if pid == 0
    {
      tracing_execve (*argv.offset (2), argv.offset (2), environ);
      fprintf (stderr, c"tracing_execve: %s\n".as_ptr (),
	       strerror (*errno ()));
      exit (1);
    }
  else if after_fork (pid) != 0
    {
      fprintf (stderr, c"after_fork: %s\n".as_ptr (),
	       strerror (*errno ()));
      exit (1);
    }

  /* Now start waiting for child processes to exit.  */

  loop
    {
      child = exec_waitpid (-1, &raw mut sig, 0);

      /* If pid is -1, a system call has been handled.  */

      if child == -1
	{ continue; }

      /* If the main process exits, then exit as well.  */

      if child == pid && !WIFSTOPPED (sig)
	{ return if WIFEXITED (sig) { WEXITSTATUS (sig) } else { WTERMSIG (sig) }; }
    }
}
