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

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar,
              c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");      // C 20: `#include <config.h>`
include!("../lib/include/signal.rs");      // C 22: `#include <signal.h>`
include!("../lib/include/stdio.rs");       // C 23: `#include <stdio.h>`
include!("../lib/include/unistd.in.rs");   // C 24: `#include <unistd.h>`
include!("../lib/include/stdlib.rs");      // C 25: `#include <stdlib.h>`
include!("../lib/include/errno.rs");       // C 26: `#include <errno.h>`
include!("../lib/include/string.rs");      // C 27: `#include <string.h>`
include!("include/exec.rs");               // C 31: `#include "exec.h"`

/* C 29: `#include <sys/wait.h>`.  The system header is not part of the reference tree and no mirror
   carries it, so the four macros this file uses are declared here with C's own bodies (PORTING.md
   P-88 ①; the `src/process.rs` 1262-1283 shape, which measured the same values).  */
// C: `#define WIFEXITED(status) (WTERMSIG (status) == 0)`
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
// C: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }
// C: `#define WIFSTOPPED(status) (((status) & 0xff) == 0x7f)`
#[inline(always)] fn WIFSTOPPED (status: c_int) -> bool { (status & 0xff) == 0x7f }
// C: `#define WTERMSIG(status) ((status) & 0x7f)`
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0x7f }

/* C 32-33: `<unistd.h>`'s process entry, which no mirror carries yet (the `lib-src/emacsclient.rs`
   292-295 note records the same gap for `exit`).  */
/// C: `pid_t fork (void);`
unsafe extern "C" {
  pub fn fork () -> pid_t;
}

/* C 32-33: `<stdlib.h>`'s `exit`.  `_Noreturn` is the port's `-> !` (PORTING.md 0.2).  */
/// C: `void exit (int);`
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

/// C 36: `static void print_usage (void)` - C's `static`, so the port keeps it a plain `unsafe fn`
/// (no `#[unsafe(no_mangle)]`, PORTING.md 0.2).
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
