# emacs/src/sysdep.c (4766 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] get-internal-run-time  (Fget_internal_run_time, 0/0, doc 0d0022871dde, line 4527)

## Functions defined — 135
- [x] maybe_disable_address_randomization  (line 162)
- [x] emacs_exec_file  (line 212)
- [x] force_open  (line 227, static)
- [x] init_standard_fds  (line 247)
- [x] get_current_dir_name_or_unreachable  (line 272, static)
- [x] emacs_get_current_dir_name  (line 346)
- [x] discard_tty_input  (line 362)
- [x] stuff_char  (line 399)
- [x] init_baud_rate  (line 415)
- [x] get_child_status  (line 454, static)
- [x] wait_for_termination  (line 501)
- [x] child_status_changed  (line 516)
- [x] child_setup_tty  (line 528)
- [x] sys_suspend  (line 630)
- [x] sys_subshell  (line 646)
- [x] save_signal_handlers  (line 758, static)
- [x] restore_signal_handlers  (line 770, static)
- [x] init_sigio  (line 784)
- [x] reset_sigio  (line 797, static)
- [x] request_sigio  (line 808)
- [x] unrequest_sigio  (line 832)
- [x] block_child_signal  (line 858)
- [x] unblock_child_signal  (line 870)
- [x] block_interrupt_signal  (line 879)
- [x] restore_signal_mask  (line 889)
- [x] init_foreground_group  (line 919)
- [x] block_tty_out_signal  (line 928)
- [x] unblock_tty_out_signal  (line 939)
- [x] tcsetpgrp_without_stopping  (line 954, static)
- [x] narrow_foreground_group  (line 971, static)
- [x] widen_foreground_group  (line 981, static)
- [x] emacs_get_tty  (line 996)
- [x] emacs_set_tty  (line 1024)
- [x] init_all_sys_modes  (line 1096)
- [x] init_sys_modes  (line 1106)
- [x] tabs_safe_p  (line 1358)
- [x] suppress_echo_on_tty  (line 1377)
- [x] get_tty_size  (line 1397)
- [x] set_window_size  (line 1454)
- [x] reset_all_sys_modes  (line 1487)
- [x] reset_sys_modes  (line 1498)
- [x] setup_pty  (line 1571)
- [x] init_system_name  (line 1608)
- [x] emacs_sigaction_flags  (line 1659, static)
- [x] emacs_sigaction_init  (line 1680)
- [x] deliver_process_signal  (line 1731)
- [x] deliver_thread_signal  (line 1766, static)
- [x] handle_fatal_signal  (line 1790, static)
- [x] deliver_fatal_signal  (line 1796, static)
- [x] deliver_fatal_thread_signal  (line 1802, static)
- [x] handle_arith_signal  (line 1808, static)
- [x] handle_sigbus  (line 1817, static)
- [x] init_sigbus  (line 1832, static)
- [x] stack_overflow  (line 1866, static)
- [x] handle_sigsegv  (line 1918, static)
- [x] init_sigsegv  (line 1949, static)
- [x] init_sigsegv  (line 1972, static)
- [x] deliver_arith_signal  (line 1980, static)
- [x] handle_danger_signal  (line 1989, static)
- [x] deliver_danger_signal  (line 1998, static)
- [x] maybe_fatal_sig  (line 2007, static)
- [x] init_signals  (line 2021)
- [x] set_random_seed  (line 2205)
- [x] set_random_seed  (line 2208)
- [x] set_random_seed  (line 2211)
- [x] seed_random  (line 2215)
- [x] init_random  (line 2226)
- [x] get_random  (line 2258)
- [x] get_random_ulong  (line 2271)
- [x] snprintf  (line 2282)
- [x] emacs_backtrace  (line 2337)
- [x] renameat_noreplace  (line 2380)
- [x] emacs_abort  (line 2402)
- [x] emacs_fstatat  (line 2419)
- [x] sys_openat  (line 2437, static)
- [x] sys_fstat  (line 2453)
- [x] sys_faccessat  (line 2463)
- [x] emacs_openat  (line 2483)
- [x] emacs_open  (line 2497)
- [x] emacs_open_noquit  (line 2516)
- [x] emacs_fopen  (line 2538)
- [x] emacs_pipe  (line 2567)
- [x] posix_close  (line 2585, static)
- [x] emacs_close  (line 2620)
- [x] emacs_fdopen  (line 2647)
- [x] emacs_fclose  (line 2662)
- [x] emacs_unlink  (line 2673)
- [x] emacs_symlink  (line 2683)
- [x] emacs_rmdir  (line 2693)
- [x] emacs_mkdir  (line 2703)
- [x] emacs_renameat_noreplace  (line 2713)
- [x] emacs_rename  (line 2724)
- [x] emacs_fchmodat  (line 2734)
- [x] emacs_intr_read  (line 2765, static)
- [x] emacs_read  (line 2788)
- [x] emacs_read_quit  (line 2795)
- [x] emacs_full_write  (line 2807, static)
- [x] emacs_write  (line 2845)
- [x] emacs_write_sig  (line 2852)
- [x] emacs_write_quit  (line 2859)
- [x] emacs_perror  (line 2868)
- [x] safe_strsignal  (line 2895)
- [x] errstream  (line 2909, static)
- [x] errputc  (line 2928)
- [x] errwrite  (line 2941)
- [x] close_output_streams  (line 2949)
- [x] serial_open  (line 2981)
- [x] cfmakeraw  (line 2996, static)
- [x] cfsetspeed  (line 3009, static)
- [x] convert_speed  (line 3138, static)
- [x] serial_configure  (line 3152)
- [x] list_system_processes  (line 3319)
- [x] list_system_processes  (line 3349)
- [x] list_system_processes  (line 3399)
- [x] make_lisp_s_us  (line 3410, static)
- [x] make_lisp_timeval  (line 3424, static)
- [x] time_from_jiffies  (line 3434, static)
- [x] put_jiffies  (line 3440, static)
- [x] get_host_uptime  (line 3450, static)
- [x] procfs_ttyname  (line 3466, static)
- [x] procfs_get_total_memory  (line 3511, static)
- [x] system_process_attributes  (line 3552)
- [x] system_process_attributes  (line 3814)
- [x] system_process_attributes  (line 3930)
- [x] system_process_attributes  (line 4093)
- [x] system_process_attributes  (line 4273)
- [x] system_process_attributes  (line 4520)
- [x] newlocale  (line 4592, static)
- [x] freelocale  (line 4598, static)
- [x] emacs_setlocale  (line 4603, static)
- [x] wcscoll_l  (line 4614, static)
- [x] towlower_l  (line 4645, static)
- [x] str_collate  (line 4668)
- [x] str_collate  (line 4745)
- [x] syms_of_sysdep  (line 4763)

## Macros defined — 23
- [x] HAVE_PROC_PIDINFO  (line 4270, body 990859b1946f)
- [x] HAVE_RANDOM  (line 2168, body da39a3ee5e6b)
- [x] HAVE_RUSAGE_INFO_CURRENT  (line 4269, body 65aac6a3b9af)
- [x] LC_COLLATE_MASK  (line 4571, body b6589fc6ab0d)
- [x] LC_CTYPE_MASK  (line 4574, body b6589fc6ab0d)
- [x] MAJOR  (line 3462, body 5ee4492d7d35)
- [x] MINOR  (line 3463, body 6e393eb12400)
- [x] POSIX_CLOSE_RESTART  (line 2630, body 356a192b7913)
- [x] PROCFS_FILE_OFFSET_BITS_HACK  (line 3802, body b6589fc6ab0d)
- [x] RAND_BITS  (line 2185, body f1abd670358e)
- [x] SSIZE_MAX  (line 2745, body 5730b5723c51)
- [x] STDERR_FILENO  (line 82, body 51e221ce5d8c)
- [x] _FILE_OFFSET_BITS  (line 3808, body c66c65175fec)
- [x] _P_WAIT  (line 129, body b6589fc6ab0d)
- [x] frame  (line 66, body aa0e9691b194)
- [x] freelocale  (line 4583, body 046e6a874e80)
- [x] locale_t  (line 4584, body 95acfee8fc6c)
- [x] newlocale  (line 4585, body c9c172012715)
- [x] random  (line 2195, body 46dc5809ee96)
- [x] read  (line 79, body b1b03607d3d5)
- [x] towlower_l  (line 4587, body 00f16fb648e6)
- [x] wcscoll_l  (line 4586, body c6f1b66282ce)
- [x] write  (line 80, body ec2711839143)

## Types and their fields — 2
- [x] save_signal: int code, struct sigaction action
- [x] speed_struct: speed_t value, speed_t internal

## File-scope objects — 11
- [x] static int exec_personality  (line 157, static)
- [x] static FILE buferr  (line 242, static)
- [x] static int old_fcntl_flags  (line 780, static)
- [x] static pid_t inherited_pgroup  (line 916, static)
- [x] static int old_fcntl_owner  (line 1089, static)
- [x] sigset_t empty_mask  (line 1654)
- [x] static struct sigaction process_fatal_action  (line 1656, static)
- [x] static pthread_t main_thread_id  (line 1715, static)
- [x] static void thread_backtrace_buffer  (line 1759, static)
- [x] static int thread_backtrace_npointers  (line 1760, static)
- [x] static struct sigaction old_sigsegv_handler  (line 1913, static)

## Decisions

- **What the file carries.** C's one `DEFUN` (`get-internal-run-time`), its 130 function
  definitions (the ledger's `Functions defined — 135` counts the four names that C defines in
  more than one configuration branch: three `set_random_seed` arms, three
  `list_system_processes` arms, six `system_process_attributes` arms, two `init_sigsegv` and
  two `str_collate`), its 23 `#define`s, its two structs (`save_signal`, `speed_struct`) and
  its eleven file-scope objects.  C's non-`static` definitions carry `#[unsafe(no_mangle)]`
  (PORTING.md 0.2); the `static` helpers stay plain `unsafe fn`, except the ones whose address
  becomes a signal action or an unwind protect (`handle_fatal_signal`, `deliver_fatal_signal`,
  `deliver_fatal_thread_signal`, `handle_arith_signal`, `deliver_thread_signal`,
  `deliver_arith_signal`, `handle_sigsegv`, `handle_sigbus`, `save_signal_handlers`,
  `restore_signal_handlers`, `set_random_seed`, `deliver_danger_signal`,
  `handle_danger_signal`), which are `unsafe extern "C" fn` (PORTING.md 1.4).
  `src/sysdep.c` also *defines* two objects other units declare and read: `empty_mask`
  (`syssignal.h` 54) and `interrupts_deferred`/`bail`-style globals are keyboard.c's, so only
  `empty_mask` is defined here.
- **Dead arms carried, not recorded (P-51/status 3.49).**  `src/sysdep.c` is *not* in
  `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py`'s
  `blind_conditions` keeps both arms of every `#if` live and the port carries the text the
  frozen build does not compile, each arm under a `#[cfg (feature = "…")]` that names the C
  condition's non-frozen side.  The live/dead split was *measured* per condition with clang
  (the same probe `tools/check-cond-eval.py` uses): 165 of the file's 294 conditions are
  reached in the frozen build.  The carried arms are, in C order: `HAVE_LINUX_FS_H`,
  `CYGWIN`, `__OpenBSD__`, `__FreeBSD__`, `WINDOWSNT`, `HAVE_SYS_SYSTEMINFO_H`, `MSDOS`,
  `HAVE_ANDROID` (the 14 syscall wrappers) and `HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` (the whole
  `maybe_disable_address_randomization`), `_PC_PIPE_BUF`'s `#else`, MS-DOS's `discard_tty_input`
  arm, `DOS_NT`/`MSDOS`/`WINDOWSNT` in `sys_subshell`, `USABLE_SIGPOLL`, `IUCLC`/`OLCUC`/`HPUX`/
  `AIX` in `child_setup_tty`, `VSWTCH`/`V_DSUSP`/`AIX`, `HAVE_GPM`, `TCXONC`/`TIOCGSIZE`/
  `WINDOWSNT`/`MSDOS` in `get_tty_size` and `TIOCSSIZE` in `set_window_size`, `HAVE_GETHOSTNAME`'s
  `#else`, `SIGDANGER`, `HAVE_ANDROID && HAVE_MMAP`'s SIGBUS arm, `HAVE_ANDROID`'s SIGSEGV arm,
  the eleven system-specific signals of `init_signals`, `HAVE_RANDOM`'s two `#else` arms,
  `WINDOWSNT`'s `init_random` and `snprintf` with its `EOVERFLOW` arm, `HAVE_RENAMEAT2`/
  `SYS_renameat2`/`#else` in `renameat_noreplace`, `HAVE_NTGUI`'s `emacs_abort` guard,
  `WINDOWSNT`'s `HANDLE`-based `emacs_get_tty`/`emacs_set_tty`, `SSIZE_MAX`/`HAVE_SNPRINTF`,
  `__ANDROID__` in `close_output_streams`, `TIOCSSIZE`/`TIOCGSIZE`/`TCXONC`/`CNEW_RTSCTS`,
  `HAVE_PROCFS`, `SOLARIS2`, `__FreeBSD__`, `__OpenBSD__` and the no-tty arm of
  `list_system_processes`/`system_process_attributes` (with their vendor structs), `__vax__`,
  `HAVE_TINY_SPEED_T`'s `#else`, the fourteen `#ifdef Bnnn` speed arms, `__STDC_ISO_10646__`
  with the five `#ifndef HAVE_NEWLOCALE` renames, and `NO_HAVE_GETRUSAGE`'s two arms.
- **The two `check-cond-eval.py` disagreements are a tool defect, not a table gap.**  C 1135
  (`#ifdef INLCR /* I'm just being cautious,`) and C 1279 (`#ifndef HAIKU /* On Haiku, TCXONC
  is a no-op …`) end in a comment; `tools/cinventory.py`'s condition text keeps the comment,
  so the probe it hands clang is ill-formed (`unterminated /* comment`, then `missing ')' after
  'defined'`) and clang answers "dead" for a branch the frozen build compiles.  The evaluator
  is right in both, the port follows clang, and both are **live** in the port.  The defect is
  reported here rather than fixed (it is a shared tool: `docs/status.md`'s 3.28 class).
- **The condition table (step 5's rule).**  Because the file's conditions are
  `<termios.h>`/`<sys/ioctl.h>`/`<syssignal.h>` text, `docs/baseline/config/source-header-macros.tsv`
  gained 57 rows (49 `termios.h` flags and speeds, seven `sys/ioctl.h` request words, and
  `syssignal.h`'s `FORWARD_SIGNAL_TO_MAIN_THREAD`), and `TABDLY`'s existing row changed value:
  the frozen `config.h` has `/* #undef TABDLY */` (configure defines `TAB3`, not `TABDLY`, for
  darwin — configure.ac 6828-6830) while `<termios.h>` defines it as `0x00000c04`, which is
  what clang reports for C 1364.  `INLCR` is one of the 49 and is the row the comment defect
  above turns into a false disagreement; the whole-table rerun reports 5717 conditions with
  `0 disagreements` and `cond-eval-verified.txt` unchanged (430 files).  With them,
  `python3 tools/check-cond-eval.py src/sysdep.c` reports `283 conditions, 0 unknown,
  2 disagreements` (63 before), the whole-table rerun reports `5717 conditions, 4 unknown,
  0 disagreements`, and `docs/baseline/config/cond-eval-verified.txt` is **byte-identical**
  (430 files, 0 dropped).  `src/sysdep.c` is nevertheless **not** added to that list: this port
  carries both arms of every `#if`, and a verified file's checker demands only the live one
  (docs/ports/callproc.md's Integrator note is the precedent, status 3.51 ②).
- **Use-site declarations and mirror gaps.**  The headers this file includes that the port has
  **no mirror** for are declared at their use site with C's own prototype in a `/// C:` line:
  `<execinfo.h>`, `<pwd.h>`/`<grp.h>` (with `struct passwd`/`struct group`), `<sys/utsname.h>`,
  `<sys/param.h>`, `<sys/file.h>`, `"syswait.h"`'s `WNOHANG` (a *system* macro the mirror does
  not carry), `"termopts.h"` (`interrupt_input`, `interrupts_deferred`),
  `<sys/resource.h>`'s `struct rusage`/`getrusage`, `<libproc.h>`, `<mach/mach_time.h>`,
  `<pthread.h>`'s `pthread_self`/`pthread_equal`/`pthread_kill`, `<termios.h>`/`<sys/ioctl.h>`
  (the 60 constants), `<wchar.h>`'s `wcscoll`, and `"cm.h"`'s `current_tty` (imported from the
  mirror's `cm_globals` module, which `cm.rs`'s own definition keeps out of the glob).
  **Mirror gaps the port did *not* close** (they belong to the owning units):
  * `src/include/keyboard.rs` has no `get_current_directory`, `add_user_signal`,
    `process_pending_signals`, `interrupt_input`, `interrupts_deferred`;
  * `src/include/frame.rs` has no `init_frame_faces` (xfaces.c's) or `is_tty_frame`, and no
    setter for `FRAME_GARBAGED_P` (C's lvalue bit macro — the port writes the same bit through
    a unit-local `FRAME_GARBAGED_P!`, the `wset_redisplay` shape status 3.53 records);
  * `src/include/termchar.rs` has no `tty_turn_off_insert`/`cursorX` (terminal.c's) and no
    member-name macro for `term_initted`/`flow_control`, so this unit adds `tty_bits!` with
    exactly those two members (P-80; the masks are the mirror's `TTY_TERM_INITTED`/
    `TTY_FLOW_CONTROL`);
  * `src/include/coding.rs` has `code_convert_string_norecord` but the file also needs
    `set_unwind_protect_ptr`/`record_unwind_protect_nothing` (lisp.rs) and `emacs_strerror`
    (emacs.c's), which are declared here;
  * `lib/include/stdio.rs` has no `fflush`/`fflush_unlocked`/`fputc_unlocked`/
    `fwrite_unlocked`/`setvbuf`/`fdopen`/`fileno`/`getc`/`putc`/`fscanf`/`sscanf`/`snprintf`/
    `strerror`/`fprintf`/`perror`/`rename`/`stdout`/`stderr`/`BUFSIZ`/`_IOFBF`/`_IOLBF`;
    `errno.rs` has no `EDOM`/`EINPROGRESS`/`EPIPE`/`EAGAIN`/`ECHILD`; `signal.rs` has no
    `sigfillset`/`sigaltstack`/`sigsuspend`/`siglongjmp`/`sigdescr_np`/`SA_RESTART`/
    `SA_ONSTACK`/`SA_SIGINFO`/the `SIGDANGER`-class names; `string.rs` has no
    `strdup`/`strnlen`/`strcasecmp`; `stdlib.rs` has no `exit`/`EXIT_FAILURE`/`random`/
    `srandom`/`srand48`/`srand`/`system`/`setenv`/`putenv`/`unsetenv`; `unistd.in.rs` has no
    `pipe2`/`kill`/`waitpid`/`vfork`/`getpgrp`/`setpgid`/`tcsetpgrp`/`tcgetattr`/
    `tcsetattr`/`tcflow`/`tcdrain`/`cfgetospeed`/`cfsetispeed`/`cfsetospeed`/`ioctl`/`uname`/
    `gethostname`/`_exit`/`execve`/`execvp`/`execlp`.
- **`#[link_name]` needs: none in this unit.**  C's `emacs_fclose` is lisp.h 5316's
  `#define emacs_fclose fclose` on this target and the port's call sites go through the
  mirror's `emacs_fclose!` macro (charset.c's finding, status 3.57); the one definition of the
  name here is the *carried* Android arm, which is never compiled.
- **Layout.**  Every record this unit declares carries clang-measured `const _` assertions for
  `sizeof` and every field the code reaches: `winsize` (8), `passwd` (72, `pw_uid`@16, `pw_shell`@56, `pw_expire`@64), `group` (32),
  `utsname` (1280, `nodename`@256), `rusage` (144, `ru_minflt`@64), `siginfo_t`
  (`si_addr`@24), `stack_t` (24), `save_signal` (24, `action`@8), `speed_struct` (16),
  `proc_taskinfo` (96, `pti_threadnum`@84), `rusage_info_current` (464, `ri_pageins`@48),
  `mach_timebase_info_data_t` (8), `kinfo_proc` (648, `kp_eproc`@296, `kp_eproc.e_ucred.cr_uid`@420),
  `extern_proc` (296, `p_stat`@36, `p_pid`@40, `p_nice`@242, `p_comm`@243), `eproc` (352,
  `e_ppid`@264, `e_tdev`@276), `pcred` (104, `p_svgid`@92) and `ucred` (76, `cr_uid`@4).
  The records that exist *only* inside a carried arm (`utsname`, `psinfo`/`lwpsinfo`, the
  FreeBSD/OpenBSD `kinfo_proc` and `uvmexp`) cannot be measured with clang — the frozen target
  has neither the headers nor the platforms — so they carry the members the arm names and **no**
  assertion; the arm is never compiled, and this is the one place the port's layout is not
  machine-checked.
- **Spelling deviations** (all recorded in `docs/ports/macro-body-notes.tsv`, 23 rows): C's
  `0`/`1` for a `bool` parameter or a `bool` return is `false`/`true` (eleven rows: the
  `emacs_set_tty`'s `flushp`, `get_child_status`'s `interruptible`, the four
  `pthread_sigmask` null masks, `tabs_safe_p`'s two dead returns, `unrequest_sigio`'s
  `interrupts_deferred = 1`, `handle_arith_signal`'s null mask, `init_sigsegv`'s `return 1`);
  the `__sigaction_u` union's `sa_handler`/`sa_sigaction` are the mirror's `__sa_handler`/
  `__sa_sigaction` (five rows) with the ABI-equal fn-pointer `transmute` at the two
  `sa_sigaction` assignments (the `siginfo_t *` parameter the mirror cannot name, P-72);
  `while (1)`/`while (true)` are `while true` (two rows); `switch`/`case` are `match` (two
  rows); `va_list`/`va_start`/`va_end` have no counterpart (P-04, one row); `min (PIPE_BUF,
  MAX_ALLOCA)` in a `const` item is the same comparison spelled out (one row); `PRIdMAX`/
  `SCNuMAX` are the concatenated `c"/proc/%jd"`/`c"MemTotal: %ju"` literals (P-45, two rows);
  and C's `set_random_seed`'s three one-line definitions make the extractor read the declarator
  as part of the body (one row).  `Qnil` is written `Qnil ()` everywhere in a body (the mirror
  exports the name; `lisp_h_Qnil` was the port's internal spelling) and `lisp_h_Qnil ()` only
  in the two `static mut` initialisers.
- **`unsafe` that changes no meaning:** `handler.unwrap () (sig)` is C's indirect call through
  a handler that C's callers always fill (the mirror types it `Option<…>`, P-27), and
  `std::mem::transmute::<usize, signal_handler_t> (SIG_IGN)` is C's own
  `(void (*) (int)) SIG_IGN` cast (a `const` cannot hold it: const-eval rejects a fn pointer
  from an integer, so the cast sits at C 763's call site).
- **Line count.**  C 4766 (live 2855) vs the port's 6237: 30.9% over C's total, declared in
  `docs/ports/line-drift-tolerance.tsv` with the `total` basis (the dead arms are carried, not
  recorded).  The per-function token comparison is the real guard and it is `OK`.
- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 537-538's
  `save_signal_handlers`/`restore_signal_handlers` prototypes (their definitions follow) and
  C 145's `tputs` declaration is kept, because it is this file's own declaration of a system
  entry (the port declares it with C's text).  C 606-609's `#if 0` block (the non-ICANON
  `VMIN`/`VTIME` settings) is dead text with **no** Rust counterpart.

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the same
argument order, and the port adds no call the reference does not make: the mirror expansions
(`tty_bits!`, `FRAME_TTY!`, `CURTTY!`, `FRAME_GARBAGED_P!`, `FOR_EACH_FRAME!`, `XSETINT!`,
`XSETTERMINAL!`, `CONS_TO_INTEGER!`, `AUTO_STRING!`, `SAFE_ALLOCA!`, `emacs_tputs!`,
`countof!`, `min`, `TYPE_MAXIMUM!`) are C's own macros expanding at the same points, and the
carried arms' calls (`android_*`, the w32 `GetConsoleMode` family, `renameat2`/`syscall`/
`sys_rename_replace`, `w32_init_random`, `sfntfont_detect_sigbus`, `dos_keyread`,
`internal_terminal_init`, `dos_ttraw`/`dos_ttcooked`, `ScreenCols`/`ScreenRows`,
`GetConsoleScreenBufferInfo`) are calls the reference makes in the arms it does not compile
here.  The one place a *function* moved is the three `set_random_seed` arms: C has three
definitions of the name and the port has three `#[cfg]`-gated ones (the same callee set:
`srandom`, `srand48`, `srand`).

## 2026-10-04: the systty/syswait/termios mirrors landed and this unit converged on them

`"sysstdio.h"`, `"sysselect.h"`, `"systty.h"` and `"syswait.h"` now have mirrors
(`femacs/src/include/sysstdio.rs`, `lib/include/sys_select.rs`, `src/include/systty.rs`,
`src/include/syswait.rs`), and `systty.rs` brings the new system mirror
`lib/include/termios.rs` with it.  This unit therefore:

- added C 106/C 107's includes (`include!("include/systty.rs")`, `include!("include/syswait.rs")`)
  at their place in the include list, and pointed C 23 (`#include "sysstdio.h"`) at
  `include/sysstdio.rs` instead of flattening it to the `<stdio.h>` mirror - the unit
  *defines* `errputc`/`errwrite`/`close_output_streams`, and the mirror's `sysstdio_decls`
  glob import is what those definitions shadow (P-21);
- **deleted** its local `<termios.h>` layer (`tcflag_t`/`speed_t`/`cc_t`/`NCCS`/`struct termios`
  plus eight `const _` assertions), its local `struct emacs_tty` and its local `CDISABLE` - all
  three were exact duplicates of what the mirrors now own (PORTING.md 0.1-3: one definition
  point per type), and the mirror carries the same clang-measured assertions;
- pointed C 38 (`#include "sysselect.h"`) at `include/sysselect.rs` as well (it brings the
  system mirror, so the unit still gets `fd_set`/`FD_SETSIZE`); the checked `fd_*` wrappers and
  the re-pointed `FD_*` macros arrive with it, exactly as in C.  `femacs/src/include/thread.rs`
  (thread.h's own `#include "sysselect.h"`) still includes the system mirror directly - that
  mirror is shared with units that include `<sys/select.h>` themselves (`lib/pselect.rs`), so
  the switch there belongs to a separate, wider convergence;
- kept every other use-site declaration of the list above, since those headers still have no
  mirror;
- took one explicit cast where C converts implicitly: `memcmp (…, NCCS)` became
  `memcmp (…, NCCS as usize)`, because the mirror keeps C's `int`-typed `NCCS`
  (`limits.in.rs`'s `CHAR_BIT: c_int` precedent) while `memcmp`'s third parameter is `size_t`.

Evidence: `build-tu src/sysdep.c` -> `built`; `extract-abi.py --check emacs/src/sysdep.c
femacs/src/sysdep.rs` -> `OK`; `rustc --cfg enable_checking` on the materialized unit -> exit 0;
`check-globals` 273 units / 0 missing; `check-mirror-dupes` 0 failures (the two mirrors removed
duplicates rather than adding any); `check-declarations` 0.

## 2026-10-04: perror/printf/EXIT_FAILURE come from the mirrors

Same convergence as `emacs.md`: the `stdio.rs` and `stdlib.rs` mirrors now carry those three
names, so the unit's own `EXIT_FAILURE` constant and its `perror`/`printf` declarations are
deleted.  The remaining use-site block keeps `exit`, which no mirror declares yet.

## 2026-10-04: the real link found three invented `<stdio.h>` entries (3.251)

`src/sysdep.c` 2914-2943 calls `fflush_unlocked`, `fputc_unlocked` and `fwrite_unlocked`, and this
unit used to answer that with three `extern` declarations of its own.  They are not entries of
`<stdio.h>`: they are `lib/unlocked-io.h`'s macros for this target, mapping onto `fflush`, `fputc`
and `fwrite` (the measured live arm is in `lib/include/unlocked-io.rs`, and
`nm -u build/emacs-ref/src/sysdep.o` names `_fflush`/`_fputc`/`_fwrite`, not the `_unlocked`
names).  `src/sysstdio.h` 29's `#include <unlocked-io.h>` is now carried by the
`include/sysstdio.rs` mirror, the four call sites invoke those macros, and the three declarations
are deleted.  Evidence: `build-tu src/sysdep.c` built, `--check` OK, and the executable link's
undefined list drops by those three names (`docs/ports/link.md`, reading three).

- **Integrator note (3.263): `snprintf` is declared by the mirror, not here.**  `lib/include/stdio.rs`
  gained the `<stdio.h>` entry so the `GLYPH_DEBUG` arms (which call it) compile - `xtask dead-arms`
  with the `glyph_debug` flip is what found it - and this unit's own declaration of the same name was
  what made the name ambiguous once the mirror carried it.  The declaration is gone; the mirror is
  the single declaration point (PORTING.md 0.1-3).
