// femacs: port of exec/exec1.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// local, macro), the same control flow and the same error model.
//
// **The frozen configuration compiles none of `exec/`** (`docs/baseline/frozen-build-sources.txt`:
// the exec loader belongs to the unexec dump path, and this build is `HAVE_PDUMPER=1`), and PORTING.md
// P-86 governs a file in that state: the whole body is transcribed as it stands - this file's previous
// revision was the three-line P-08 note.  The file has no `#if` of its own, so "both arms" is vacuous;
// the three configuration-shaped surfaces it reaches are declared at their use sites, because no
// mirror in this tree carries them (P-88 ①, and P-90's seventh half records that a system macro with
// several consumers stays a use-site declaration in each unit rather than getting a mirror of its own):
// `<sys/wait.h>`'s `WIFEXITED`/`WEXITSTATUS`/`WTERMSIG` (the `src/process.rs` 1262-1283 shape),
// `<unistd.h>`'s `fork`/`getpid`/`setpgid`, `<stdio.h>`'s `perror`, `<stdlib.h>`'s `exit` and
// `<signal.h>`'s `raise`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar,
              c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");      // C 20: `#include <config.h>`
include!("../lib/include/unistd.in.rs");   // C 21: `#include <unistd.h>`
include!("../lib/include/stdlib.rs");      // C 22: `#include <stdlib.h>`
include!("../lib/include/stdio.rs");       // C 23: `#include <stdio.h>`
include!("../lib/include/signal.rs");      // C 24's `#include <sys/wait.h>` sibling: `raise`
include!("include/exec.rs");               // C 27: `#include "exec.h"`

/* C 24: `#include <sys/wait.h>`.  The system header is not part of the reference tree: the three
   macros this file uses are declared here with C's own bodies (PORTING.md P-88 ①; the same three
   `src/process.rs` 1262-1283 and `exec/test.rs` carry).  */
// C: `#define WIFEXITED(status) (WTERMSIG (status) == 0)`
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
// C: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }
// C: `#define WTERMSIG(status) ((status) & 0x7f)`
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0x7f }

/* C 21-23: the three entries no mirror carries yet - the `<unistd.h>` mirror already declares
   `getpid` (63) and the `<stdio.h>` mirror `perror` (63), so C's lines for those two have no item of
   their own here, while `exit` is the gap the `lib-src/emacsclient.rs` 292-295 note also records.  */
/// C: `pid_t fork (void);`
unsafe extern "C" {
  pub fn fork () -> pid_t;
  /// C: `int setpgid (pid_t, pid_t);`
  pub fn setpgid (pid: pid_t, pgid: pid_t) -> c_int;
  /// C: `void exit (int);`
  pub fn exit (status: c_int) -> !;
}

/* C 29-35: "exec1 is a program which takes another program and its arguments, forks, and executes
   that program, all while tracing it and its children to use the program execution mechanism defined
   in exec.c. …" - C's own commentary, carried in the ledger for this file.  */

/// C 39: `int main (int argc, char **argv)` - the process entry point, so the port keeps the name and
/// exports it (`#[unsafe(no_mangle)]`, PORTING.md 0.2).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  /* C 41-43: `pid_t pid, pid1; extern char **environ; int wstatus;` - C leaves `pid`/`pid1`/`wstatus`
     uninitialised and writes them before any read; the port spells C's own zero values
     (PORTING.md 0.1-13).  `environ` is declared by the `<unistd.h>` mirror (94), so C's own line has
     no item of its own here.  */
  let mut pid: pid_t = 0;
  let mut pid1: pid_t = 0;
  let mut wstatus: c_int = 0;

  /* Provide the file name of the loader.  */
  exec_init (*argv.offset (1));

  pid1 = getpid ();
  pid = fork ();

  if pid == 0
    {
      /* Set the process group used to the parent.  */
      if setpgid (0, pid1) != 0
	{ perror (c"setpgid".as_ptr ()); }

      tracing_execve (*argv.offset (2), argv.offset (2), environ);

      /* An error occurred.  Exit with failure.  */
      exit (127);
    }
  else
    {
      if after_fork (pid) != 0
	{ exit (127); }

      /* Start waiting for the process to exit.  */

      loop
	{
	  pid1 = exec_waitpid (-1, &raw mut wstatus, 0);

	  /* If the child process exits normally, exit with its status
	     code.  If not, raise the signal that caused it to
	     exit.  */

	  if pid == pid1
	    {
	      if WIFEXITED (wstatus)
		{ exit (WEXITSTATUS (wstatus)); }
	      else /* if WIFSIGNALED (wstatus) */
		{
		  raise (WTERMSIG (wstatus));

		  /* Just in case the signal raised doesn't cause an
		     exit.  */
		  exit (127);
		}
	    }

	  /* Otherwise, continue looping.  */
	}
    }
}
