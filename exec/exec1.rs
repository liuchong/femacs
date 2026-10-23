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
// this gate; this round extends it to this file and `exec/test.rs`, which is the fix for the gap
// `tools/check-undef.py` read on 2026-10-15 - its seven new rows are the four names the gated sibling
// defines (`after_fork`, `exec_init`, `exec_waitpid`, `tracing_execve`) referenced by this unit's
// object and `test.o` (the other three, `user_alloca`, `user_copy` and `__errno_location`, come from
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
include!("../lib/include/unistd.in.rs");   // C 21: `#include <unistd.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");      // C 22: `#include <stdlib.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdio.rs");       // C 23: `#include <stdio.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/signal.rs");      // C 24's `#include <sys/wait.h>` sibling: `raise`
#[cfg (feature = "have_android")]
include!("include/exec.rs");               // C 27: `#include "exec.h"`

/* C 24: `#include <sys/wait.h>`.  The system header is not part of the reference tree: the three
   macros this file uses are declared here with C's own bodies (PORTING.md P-88 ①; the same three
   `src/process.rs` 1262-1283 and `exec/test.rs` carry).  */
// C: `#define WIFEXITED(status) (WTERMSIG (status) == 0)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
// C: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }
// C: `#define WTERMSIG(status) ((status) & 0x7f)`
#[cfg (feature = "have_android")]
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0x7f }

/* C 21-23: the three entries no mirror carries yet - the `<unistd.h>` mirror already declares
   `getpid` (63) and the `<stdio.h>` mirror `perror` (63), so C's lines for those two have no item of
   their own here, while `exit` is the gap the `lib-src/emacsclient.rs` 292-295 note also records.  */
/// C: `pid_t fork (void);`
#[cfg (feature = "have_android")]
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
#[cfg (feature = "have_android")]
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
