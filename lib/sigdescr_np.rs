// femacs: port of lib/sigdescr_np.c @ fd7afced403.
/* Written by Bruno Haible <bruno@clisp.org>, 2020.  */

/* emacs/lib/sigdescr_np.c: C's `<string.h>` include has no counterpart (PORTING.md 0.1).  The
   `#if defined SIGX` guards are decided by the *system* <signal.h>, not by the frozen
   configuration table, so the evaluator asks for every arm; this target defines only the
   signals lib/include/signal.rs carries (values measured from the frozen clang), and the
   arms for the ones it does not define were read out of C's text - the
   docs/ports/macro-body-notes.tsv row lists them.  C's `switch (sig)` becomes a `match`, so
   `switch`/`case` have no counterpart either.  */
use std::ffi::{c_char, c_int, c_uint, c_void};
include!("include/gnulib_config.rs");
include!("include/signal.rs");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sigdescr_np (sig: c_int) -> *const c_char
{
  /* Note: Some platforms (glibc, FreeBSD, NetBSD, OpenBSD, AIX, Haiku,
     Android) have an array 'sys_siglist'.  (On AIX, you need to declare it
     yourself, and it has fewer than NSIG elements.)  Its contents varies
     depending on the OS.
     On other OSes, you can invoke strsignal (sig) in the C locale.
     In the code below, we show the differences.
     You can see how cryptic some of these strings are.  We try to pick more
     understandable wordings.  */

  match sig
    {
	SIGABRT =>
	  {
	    /* glibc: "Aborted".  *BSD: "Abort trap".  Solaris: "Abort".  */
	    return c"Aborted".as_ptr ();
	  }
	SIGFPE =>
	  {
	    /* glibc, *BSD: "Floating point exception".  Solaris: "Arithmetic exception". The latter is more correct, because of integer division by 0 or -1.  */
	    return c"Arithmetic exception".as_ptr ();
	  }
	SIGILL =>
	  {
	    return c"Illegal instruction".as_ptr ();
	  }
	SIGINT =>
	  {
	    return c"Interrupt".as_ptr ();
	  }
	SIGSEGV =>
	  {
	    return c"Segmentation fault".as_ptr ();
	  }
	SIGTERM =>
	  {
	    return c"Terminated".as_ptr ();
	  }
	SIGALRM =>
	  {
	    return c"Alarm clock".as_ptr ();
	  }
	SIGBUS =>
	  {
	    return c"Bus error".as_ptr ();
	  }
	SIGCHLD =>
	  {
	    /* glibc, *BSD: "Child exited".  Solaris: "Child status changed".  */
	    return c"Child stopped or exited".as_ptr ();
	  }
	SIGCONT =>
	  {
	    return c"Continued".as_ptr ();
	  }
	SIGHUP =>
	  {
	    return c"Hangup".as_ptr ();
	  }
	SIGKILL =>
	  {
	    return c"Killed".as_ptr ();
	  }
	SIGPIPE =>
	  {
	    return c"Broken pipe".as_ptr ();
	  }
	SIGQUIT =>
	  {
	    return c"Quit".as_ptr ();
	  }
	SIGSTOP =>
	  {
	    /* glibc, Solaris: "Stopped (signal)".  *BSD: "Suspended (signal)".  */
	    return c"Stopped (signal)".as_ptr ();
	  }
	SIGTSTP =>
	  {
	    /* glibc: "Stopped".  *BSD: "Suspended".  Solaris: "Stopped (user)".  */
	    return c"Stopped".as_ptr ();
	  }
	SIGTTIN =>
	  {
	    return c"Stopped (tty input)".as_ptr ();
	  }
	SIGTTOU =>
	  {
	    return c"Stopped (tty output)".as_ptr ();
	  }
	SIGUSR1 =>
	  {
	    /* glibc, *BSD: "User defined signal 1".  Solaris: "User signal 1".  */
	    return c"User defined signal 1".as_ptr ();
	  }
	SIGUSR2 =>
	  {
	    /* glibc, *BSD: "User defined signal 2".  Solaris: "User signal 2".  */
	    return c"User defined signal 2".as_ptr ();
	  }
	/* C's `#if defined SIGPOLL` arm: SIGPOLL is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigpoll")]
	SIGPOLL =>
	  {
	    /* glibc: "I/O possible".  Solaris: "Pollable event".  */
	    return c"I/O possible".as_ptr ();
	  }
	SIGPROF =>
	  {
	    return c"Profiling timer expired".as_ptr ();
	  }
	SIGSYS =>
	  {
	    return c"Bad system call".as_ptr ();
	  }
	SIGTRAP =>
	  {
	    /* glibc, Solaris: "Trace/breakpoint trap".  *BSD: "Trace/BPT trap".  */
	    return c"Trace/breakpoint trap".as_ptr ();
	  }
	SIGURG =>
	  {
	    /* glibc, *BSD: "Urgent I/O condition".  Solaris: "Urgent socket condition".  */
	    return c"Urgent I/O condition".as_ptr ();
	  }
	SIGVTALRM =>
	  {
	    return c"Virtual timer expired".as_ptr ();
	  }
	SIGXCPU =>
	  {
	    /* glibc, *BSD: "CPU time limit exceeded".  Solaris: "Cpu limit exceeded".  */
	    return c"CPU time limit exceeded".as_ptr ();
	  }
	SIGXFSZ =>
	  {
	    return c"File size limit exceeded".as_ptr ();
	  }
	/* C's `#if defined SIGBREAK` arm: SIGBREAK is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigbreak")]
	SIGBREAK =>
	  {
	    return c"Ctrl-Break".as_ptr ();
	  }
	/* C's `#if defined SIGCLD` arm: SIGCLD is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigcld")]
	SIGCLD =>
	  {
	    return c"Child stopped or exited".as_ptr ();
	  }
	/* C's `#if defined SIGCPUFAIL` arm: SIGCPUFAIL is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigcpufail")]
	SIGCPUFAIL =>
	  {
	    /* AIX: "CPU failure predicted".  */
	    return c"CPU going down".as_ptr ();
	  }
	/* C's `#if defined SIGDANGER` arm: SIGDANGER is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigdanger")]
	SIGDANGER =>
	  {
	    /* AIX: "Paging space low".  */
	    return c"Swap space nearly exhausted".as_ptr ();
	  }
	SIGEMT =>
	  {
	    /* glibc/Hurd, *BSD: "EMT trap".  Solaris: "Emulation trap".  */
	    return c"Instruction emulation needed".as_ptr ();
	  }
	SIGINFO =>
	  {
	    return c"Information request".as_ptr ();
	  }
	SIGIO =>
	  {
	    return c"I/O possible".as_ptr ();
	  }
	/* C's `#if defined SIGIOT` arm: SIGIOT is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigiot")]
	SIGIOT =>
	  {
	    return c"IOT instruction".as_ptr ();
	  }
	/* C's `#if defined SIGKAP` arm: SIGKAP is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigkap")]
	SIGKAP =>
	  {
	    /* Process must issue a KSKAPACK ioctl, or will be killed in 30 seconds.  */
	    /* AIX: "Monitor mode granted".  */
	    return c"Keep Alive Poll".as_ptr ();
	  }
	/* C's `#if defined SIGKILLTHR` arm: SIGKILLTHR is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigkillthr")]
	SIGKILLTHR =>
	  {
	    return c"Kill thread".as_ptr ();
	  }
	/* C's `#if defined SIGKMEM` arm: SIGKMEM is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigkmem")]
	SIGKMEM =>
	  {
	    return c"Kernel memory request".as_ptr ();
	  }
	/* C's `#if defined SIGKMESS` arm: SIGKMESS is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigkmess")]
	SIGKMESS =>
	  {
	    return c"Kernel message".as_ptr ();
	  }
	/* C's `#if defined SIGKSIG` arm: SIGKSIG is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigksig")]
	SIGKSIG =>
	  {
	    return c"Kernel signal".as_ptr ();
	  }
	/* C's `#if defined SIGKSIGSM` arm: SIGKSIGSM is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigksigsm")]
	SIGKSIGSM =>
	  {
	    return c"Kernel signal for signal manager".as_ptr ();
	  }
	/* C's `#if defined SIGLIBRT` arm: SIGLIBRT is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_siglibrt")]
	SIGLIBRT =>
	  {
	    return c"Real-time library interrupt".as_ptr ();
	  }
	/* C's `#if defined SIGLOST` arm: SIGLOST is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_siglost")]
	SIGLOST =>
	  {
	    /* Solaris: "Resource lost".  */
	    return c"File lock lost".as_ptr ();
	  }
	/* C's `#if defined SIGMIGRATE` arm: SIGMIGRATE is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigmigrate")]
	SIGMIGRATE =>
	  {
	    return c"Process migration".as_ptr ();
	  }
	/* C's `#if defined SIGMSG` arm: SIGMSG is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigmsg")]
	SIGMSG =>
	  {
	    /* AIX: "Input device data".  */
	    return c"Message in the ring".as_ptr ();
	  }
	/* C's `#if defined SIGPLAN` arm: SIGPLAN is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigplan")]
	SIGPLAN =>
	  {
	    return c"Programming language anomaly".as_ptr ();
	  }
	/* C's `#if defined SIGPRE` arm: SIGPRE is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigpre")]
	SIGPRE =>
	  {
	    return c"Programmed exception".as_ptr ();
	  }
	/* C's `#if defined SIGPWR` arm: SIGPWR is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigpwr")]
	SIGPWR =>
	  {
	    /* glibc: "Power failure".  NetBSD: "Power fail/restart".  */
	    return c"Power failure".as_ptr ();
	  }
	/* C's `#if defined SIGRECONFIG` arm: SIGRECONFIG is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigreconfig")]
	SIGRECONFIG =>
	  {
	    return c"Dynamic logical partitioning changed".as_ptr ();
	  }
	/* C's `#if defined SIGRECOVERY` arm: SIGRECOVERY is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigrecovery")]
	SIGRECOVERY =>
	  {
	    return c"Kernel recovery".as_ptr ();
	  }
	/* C's `#if defined SIGRETRACT` arm: SIGRETRACT is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigretract")]
	SIGRETRACT =>
	  {
	    /* AIX: "Monitor mode retracted".  */
	    return c"Retracting Keep Alive Poll".as_ptr ();
	  }
	/* C's `#if defined SIGSAK` arm: SIGSAK is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigsak")]
	SIGSAK =>
	  {
	    /* AIX: "Secure attention".  */
	    return c"Secure Attention Key".as_ptr ();
	  }
	/* C's `#if defined SIGSAM` arm: SIGSAM is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigsam")]
	SIGSAM =>
	  {
	    return c"Symbolic computation failed".as_ptr ();
	  }
	/* C's `#if defined SIGSNDELAY` arm: SIGSNDELAY is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigsndelay")]
	SIGSNDELAY =>
	  {
	    return c"Done sending message".as_ptr ();
	  }
	/* C's `#if defined SIGSOUND` arm: SIGSOUND is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigsound")]
	SIGSOUND =>
	  {
	    /* AIX: "Sound completed".  */
	    return c"Sound configuration changed".as_ptr ();
	  }
	/* C's `#if defined SIGSTKFLT` arm: SIGSTKFLT is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigstkflt")]
	SIGSTKFLT =>
	  {
	    return c"Stack fault".as_ptr ();
	  }
	/* C's `#if defined SIGSYSERROR` arm: SIGSYSERROR is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigsyserror")]
	SIGSYSERROR =>
	  {
	    return c"Kernel error".as_ptr ();
	  }
	/* C's `#if defined SIGTALRM` arm: SIGTALRM is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigtalrm")]
	SIGTALRM =>
	  {
	    return c"Thread alarm clock".as_ptr ();
	  }
	/* C's `#if defined SIGTHR` arm: SIGTHR is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigthr")]
	SIGTHR =>
	  {
	    /* OpenBSD: "Thread AST".  */
	    return c"Thread library interrupt".as_ptr ();
	  }
	/* C's `#if defined SIGVIRT` arm: SIGVIRT is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigvirt")]
	SIGVIRT =>
	  {
	    return c"Virtual time alarm clock".as_ptr ();
	  }
	/* C's `#if defined SIGWAITING` arm: SIGWAITING is not defined on the frozen target, so the arm is
	   kept as text under a feature naming that platform (P-51).  */
	#[cfg (feature = "signal_sigwaiting")]
	SIGWAITING =>
	  {
	    /* AIX: "No runnable lwp".  */
	    return c"Thread waiting".as_ptr ();
	  }
	SIGWINCH =>
	  {
	    /* glibc: "Window changed".  *BSD: "Window size changed" or "Window size changes".  */
	    return c"Window size changed".as_ptr ();
	  }
	_ =>
	  { }

    }

  return std::ptr::null ();
}
