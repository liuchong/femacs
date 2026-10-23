# emacs/lib-src/emacsclient.c (2524 lines)

# STATUS (read this first).  The unit is complete: every item below is implemented in
# lib-src/emacsclient.rs and every live statement of the C file is carried.  The
# boxes are ticked on that basis, and the one open item is a *shared-file* one:
# `tools/extract-abi.py --check emacs/lib-src/emacsclient.c lib-src/emacsclient.rs`
# currently reports 9 problem lines, all of them of the form "does not mention …": each is
# a row `docs/ports/macro-body-notes.tsv` must carry (the nine rows are listed verbatim under
# "Owed note rows" below, with the reason each token cannot reappear).  `docs/ports/*.tsv` is the
# integrator's set of shared tables, so this lane did not write them; add the nine rows
# listed below and the same command prints OK (the port itself needs no change).

# The gate's own output with the nine rows absent, kept verbatim:
#
#   - constant socknamesize does not mention null (C value: sizeof ((struct sockaddr_un *) NULL)->sun_path)
#   - function message does not mention va_end, va_list, va_start
#   - function decode_options does not mention case, default, switch
#   - function print_help_and_exit does not mention <the 100 words of the usage message>
#   - function connect_with_timeout does not mention case, default, goto, switch
#   - function install_handler does not mention sa_handler
#   - function local_sockname does not mention priumax
#   - function set_local_socket does not mention priumax
#   - function set_socket does not mention alternate, alternate_editor, editor, emacs_server_file, environment, file, variable
#   FAIL emacs/lib-src/emacsclient.c -> lib-src/emacsclient.rs


# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 39
- [x] xmalloc  (line 209, static)
- [x] xrealloc  (line 223, static)
- [x] xstrdup  (line 237, static)
- [x] get_current_dir_name  (line 257)
- [x] message  (line 466, static)
- [x] decode_options  (line 500, static)
- [x] print_help_and_exit  (line 682, static)
- [x] fail  (line 737, static)
- [x] sock_err_message  (line 802, static)
- [x] send_to_emacs_len  (line 832, static)
- [x] send_to_emacs  (line 876, static)
- [x] quote_argument_len  (line 887, static)
- [x] quote_argument  (line 916, static)
- [x] unquote_argument  (line 925, static)
- [x] open_config  (line 994, static)
- [x] get_server_config  (line 1030, static)
- [x] cloexec_socket  (line 1085, static)
- [x] connect_with_timeout  (line 1103, static)
- [x] set_tcp_socket  (line 1231, static)
- [x] strprefix  (line 1284, static)
- [x] find_tty  (line 1294, static)
- [x] process_grouping  (line 1342, static)
- [x] connect_socket  (line 1376, static)
- [x] reinstall_handler_if_needed  (line 1450, static)
- [x] handle_sigcont  (line 1464, static)
- [x] handle_sigtstp  (line 1470, static)
- [x] handle_sigttou  (line 1476, static)
- [x] handle_sigwinch  (line 1482, static)
- [x] install_handler  (line 1493, static)
- [x] init_signals  (line 1523, static)
- [x] act_on_tty_signal  (line 1538, static)
- [x] act_on_signals  (line 1558, static)
- [x] local_sockname  (line 1619, static)
- [x] set_local_socket  (line 1658, static)
- [x] set_socket  (line 1831, static)
- [x] start_daemon_and_retry_set_socket  (line 1986, static)
- [x] set_socket_timeout  (line 2119, static)
- [x] check_socket_timeout  (line 2140, static)
- [x] main  (line 2154)

## Macros defined — 8
- [x] CLOSE_SOCKET  (line 64, body da38860cb875)
- [x] DEFAULT_TIMEOUT  (line 71, body 2bc448c5b86a)
- [x] HSOCKET  (line 63, body 46f8ab7c0cff)
- [x] INITIALIZE  (line 65, body da39a3ee5e6b)
- [x] INVALID_SOCKET  (line 62, body eb883130e83d)
- [x] O_PATH  (line 1360, body f374a12b51cb)
- [x] SOCKETS_IN_FILE_SYSTEM  (line 60, body da39a3ee5e6b)
- [x] egetenv  (line 67, body b8fbb9873ecd)

## Types and their fields — 2
- [x] <anon@1233>: struct sockaddr_in in, struct sockaddr sa
- [x] local_sockaddr: struct sockaddr_un un, struct sockaddr sa

## File-scope objects — 12
- [x] static int main_argc  (line 105, static)
- [x] static bool nowait  (line 111, static)
- [x] static bool quiet  (line 114, static)
- [x] static bool suppress_output  (line 117, static)
- [x] static bool eval  (line 120, static)
- [x] static bool create_frame  (line 123, static)
- [x] static bool reuse_frame  (line 126, static)
- [x] static char parent_id  (line 135, static)
- [x] static bool tty  (line 138, static)
- [x] static char alternate_editor  (line 142, static)
- [x] static intmax_t timeout  (line 154, static)
- [x] static pid_t emacs_pid  (line 160, static)

## Decisions

- **Shape.** One unit, `lib-src/emacsclient.rs`, written in the `lib-src/make-docfile.rs`
  / `lib-src/hexl.rs` shape: `#![allow (…)]`, `use std::ffi::{…}`, the `include!` list in C's
  own order, then C's file-scope objects and functions in C's order, with `main` as
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char)
  -> c_int`.  The 37 `static` helpers stay plain `unsafe fn`, except the four signal handlers,
  which are `unsafe extern "C" fn` because C passes their addresses to `sigaction`
  (PORTING.md 1.4).
- **Symbols (measured, not assumed).**  The reference `lib-src/Makefile` cannot build the
  object in this checkout (`make emacsclient.o` omits `-I../src`, so `#include <config.h>`
  fails), so the object was produced with the frozen configuration directly:
  `clang -std=gnu23 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
  -I emacs/src -I emacs/lib -c -o /tmp/emacsclient-ref.o emacs/lib-src/emacsclient.c`, then
  `nm -g`: the object **defines** exactly `_get_current_dir_name` and `_main`, and imports the
  system/gnulib entries (`_rpl_free`, `_fopen$DARWIN_EXTSN`, `_pselect$DARWIN_EXTSN`, `_getline`,
  `_snprintf`, `_strnlen`, `_strdup`, `_strstr`, `_strtoimax`, `_strtoumax`, `_atoi`, `_confstr`,
  `_ttyname`, `_getpgrp`, `_tcgetpgrp`, `_inet_addr`, `_inet_ntoa`, `_socket`, `_connect`,
  `_send`, `_recv`, `_setsockopt`, `_getsockopt`, `_kill`, `_fork`, `_waitpid`, `_execvp`,
  `_faccessat`, `_fchdir`, `_stat`, `_fstat`, `_open`, `_close`, `_fopen`, `_fclose`,
  `_file_has_acl`, `_current_timespec`, `_make_timespec`, `_timespec_add`, `_timespec_sub`,
  `_timespec_cmp`, `_rpl_getopt_long_only`, `_rpl_optarg`, `_rpl_optind`,
  `___darwin_check_fd_set_overflow`, `__NSGetEnviron`, …).  Two `#[unsafe(no_mangle)]` names
  are therefore all this unit exports; there is no `signal` import, which is what proves the
  `SA_RESETHAND` arm below is the live one.
- **Dead arms (P-08: recorded, not transcribed).**  `WINDOWSNT`, `DOS_NT` and `HAVE_NTGUI` are
  `undef` in the frozen configuration and `SOCK_CLOEXEC` does not exist on this target
  (measured), so C 23-43, 46-47, 303-459 (`w32_get_resource` … `ttyname`), 472-484, 665-676,
  804-820, 950-986 (`close_winsock`/`initialize_sockets`/`w32_connect_timer`), 1041-1043,
  1087-1088, 1195-1217 (the `w32` timer thread), 1452-1454, 1505-1516, 1704-1705,
  1720-1742 (the Android arm), 1892-1980, 2037-2100, 2130-2135, 2147-2149, 2167-2172 and
  2217-2219 are preprocessor-only arms this target does not compile.  Evidence:
  `python3 tools/check-cond-eval.py lib-src/emacsclient.c` ->
  `46 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the live set is the frozen
  configuration's.
- **`reinstall_handler_if_needed`'s live body is empty** (`SA_RESETHAND` *is* defined, 0x0004),
  which PORTING.md 0.3's no-stub rule and the extractor's `empty_in_c` carve-out both have to
  meet: the port keeps the dead arm's text under `#[cfg (feature = "no_sa_resethand")]`
  (P-51, the `lib/acl-internal.rs` 37 shape) instead of writing `{}`, and the arm's `signal`
  call spells the handler the way `signal.rs` 165 declares it (a `usize`) so the carried text
  would still compile if that switch were ever flipped.
- **Type macros.**  `HSOCKET` is C's `#define HSOCKET int`, so the port keeps C's name as a
  `macro_rules!` over the target's type and uses it in type position (the `src/doc.rs` 67
  `doc_fd` shape); `INITIALIZE` is the empty-valued `macro_rules!`; `CLOSE_SOCKET` and
  `egetenv` expand to the mirror calls; `INVALID_SOCKET`, `DEFAULT_TIMEOUT`, `O_PATH` (C's own
  live `#define O_PATH O_SEARCH`, because this target's `<fcntl.h>` has no `O_PATH`) and
  `SOCKETS_IN_FILE_SYSTEM` are constants.
- **The two unions.**  `local_sockaddr` (C 1363-1368, inside the live `SOCKETS_IN_FILE_SYSTEM`
  arm) keeps C's name and field order, `#[repr(C)]`, asserted 106/0.  C 1233-1236's anonymous
  `union { struct sockaddr_in in; struct sockaddr sa; }` is declared **inside** `set_tcp_socket`
  where C declares it, named after the variable it declares (P-67's `<owner>_<member>` shape;
  the extractor skips the anon container, so the name is the port's own), asserted 16.
- **`socknamesize`** is C's array-size idiom (`sizeof ((struct sockaddr_un *) NULL)->sun_path`)
  written as the same measurement over the port's type — `size_of::<sockaddr_un> ()` minus
  `offset_of! (sockaddr_un, sun_path)` — pinned with `const _: () = assert! (socknamesize ==
  104)`.  C's null-pointer cast cannot reappear (`NULL` is a token the gate looks for), which is
  one of the owed note rows.
- **Use-site declarations (P-26/P-72 (3)).**  C names these from headers this program has no
  mirror for, so each is declared with C's prototype in a `/// C:` line and the measurement:
  `<sys/socket.h>`/`<netinet/in.h>`/`<sys/un.h>`/`<arpa/inet.h>` (`sockaddr`, `sockaddr_in`,
  `sockaddr_un`, `in_addr`, `linger`, `socklen_t`, `AF_INET` 2, `AF_UNIX` 1, `SOCK_STREAM` 1,
  `IPPROTO_TCP` 6, `SOL_SOCKET` 0xffff, `SO_LINGER` 0x80, `SO_ERROR` 0x1007, `SO_RCVTIMEO`
  0x1006, `socket`/`connect`/`send`/`recv`/`setsockopt`/`getsockopt`/`inet_addr`/`inet_ntoa`/
  `htons`), `<unistd.h>` (`execvp`, `fork`, `getpgrp`, `tcgetpgrp`, `ttyname`, `confstr`,
  `STDOUT_FILENO` 1, `environ`), `<sys/wait.h>` (`waitpid`, `WIFEXITED`/`WEXITSTATUS` as the
  `src/callproc.rs` 267 inline pair, `WUNTRACED` 2, `WCONTINUED` 0x10), `<stdlib.h>`
  (`exit`, `atoi`, `strdup`), `<string.h>` (`strnlen`, `strstr`), `<stdio.h>`
  (`getline`, `snprintf`, `vfprintf`), `<inttypes.h>` (`strtoimax`, `strtoumax`, and
  `PRIuMAX` = `"ju"` written into the format literals), `<signal.h>` (`sig_atomic_t` = `c_int`
  (measured 4), `SIG_DFL` as the `signal_handler_t` `None` — the `src/process.rs` 8851 shape —
  and `signal_handler_t` itself, because C's `void (*) (int)` parameters are the port's
  `Option<unsafe extern "C" fn (c_int)>`), `<config.h>` (`PACKAGE_VERSION` `"32.0.50"`,
  `DIRECTORY_SEP`/`IS_DIRECTORY_SEP`/`IS_DEVICE_SEP` — the `src/dired.rs` 99 / `src/callproc.rs`
  295 precedents), and `<stddef.h>`/`<stdint.h>` (`ptrdiff_t`, `size_t`, `intmax_t`, `uintmax_t`
  written out the `lib-src/make-docfile.rs` 51 way; `PTRDIFF_MAX`/`SIZE_MAX`/`PATH_MAX` come
  from the `stdint.in.rs`/`limits.in.rs` mirrors).  `<unlocked-io.h>` renames nothing on this
  target (the object imports `_fread`/`_fprintf`/`_fflush`).  `<stdckdint.h>`'s `ckd_add` is the
  token-aliases family: `set_socket_timeout` writes `seconds.overflowing_add (0)` and its
  `(value, overflow)` pair, as `src/timefns.rs` 611 does.
- **`environ`** is declared as the object (`pub static mut environ: *mut *mut c_char;`, the
  `src/callproc.rs` 191 shape) although the reference object imports `__NSGetEnviron` — on this
  target `<unistd.h>` defines `environ` as `(*_NSGetEnviron ())`, and a use-site `_NSGetEnviron`
  call would lose C's `environ` token as well.  Reported to the integrator as a symbol-level
  difference for `tools/check-symbols.py`.
- **Control flow (PORTING.md 2.2).**  The three `switch`es become `match` (arms keep C's order
  and its `case` comments; the `switch`'s trailing `break` has no `match` site); `goto done` /
  `goto timeout` become the labelled blocks `'done`/`'timeout` (`break 'done` is the same
  forward jump; `break 'timeout` leaves the enclosing block); the backward `goto retry` becomes
  `'retry: loop` with `continue 'retry`, and the port writes one `break` where C's control
  falls out of the labelled region into the statements after the `while` (the
  `src/editfns.rs` 4590 shape).  The `goto`/`switch`/`case`/`default` tokens therefore have no
  Rust site (`main`'s `goto` is already covered by the existing `main\tgoto` row; the other
  four functions need rows, listed below).
- **Statement-level rewrites C's text does not have in Rust.**  C's `if (cond) stmt;` without
  braces got braces; C's comma expressions (`*q++ = '&', c = '_';`) became two statements;
  C's `do { … } while (x)` became `loop { …; if !(x) { break; } }` (`unquote_argument`, and the
  `recv` retry loop); `while ((sent = send (…)) < 0)` became `loop { sent = …; if !(sent < 0)
  { break; } … }`; `for (char *end_p = p; end_p < …; p = end_p)` became a `while` whose body
  ends with C's `p = end_p` update; `fail`'s `for (char *tok = s;;)` became `loop`;
  `main`'s `for (int i = optind; i < argc; i++)` became a `while` with the update written at
  the end of the body **and** on both `continue` paths (the for's `i++` runs before re-testing);
  `printf (&"\n%s"[skiplf], str)` became `printf (c"\n%s".as_ptr ().offset (if skiplf { 1 }
  else { 0 }), str)`, the same byte of the same literal.
- **Compound literals and initializers.**  The `sockaddr_in` compound literal became the
  field-complete literal (C zero-fills `sin_len`/`sin_zero`); `struct linger` and the two
  `struct timespec` values are literals; `struct sigaction act = { .sa_handler = handler };`
  became `zeroed ()` + the field write (P-27's `Option` member), and the `SIG_IGN` comparison
  is the `src/process.rs` 8697 transmute.  C's `char emacs[] = "emacs"` / `char daemon_option[]
  = "--daemon"` are `[u8; 6]`/`[u8; 9]` initialised from the byte literals, because Rust cannot
  initialise a `[c_char; N]` from a `&CStr`; `execvp` reads the same bytes.
- **Uninitialized C locals (1.5).**  `sock_status`, `socknamelen`, `rc`, `sent`, `retry`,
  `c` (twice), `endptr`, `pwdlen`, `status`, `s` and `dotted`/`dottedsize` are zero-initialized;
  that is where the build's `value assigned … is never read` warnings come from.
- **`let mut timeout_: timeval`** in `set_socket_timeout` is C's local `timeout`, which shadows
  the file-scope object; Rust cannot shadow a static with a `let`, so the port spells the
  trailing-underscore form (token-equal; the `src/editfns.rs` 3483 precedent).
- **Deviations recorded.**
  1. C 162/251/793's forward declarations are dropped: Rust items are order-independent and a
     declaration plus a definition is `E0428` (PORTING.md 0.1-12).
  2. `longopts` is a `static mut` and `shortopts` a `static &CStr`: a Rust `static` of a
     raw-pointer-bearing type is not `Sync`, so C's `const` array is the `static mut` the other
     pointer tables use (`src/alloc.rs` 412); the two tables are otherwise C's, entry for entry
     and in C's order, with the live `--socket-name` row included.
  3. `int`-typed helper aliases (`ptrdiff_t`, `size_t`, `intmax_t`, `uintmax_t`, `sig_atomic_t`,
     `socklen_t`, `signal_handler_t`) are written out at the use site because this program has no
     `lisp.h`/`syssignal.h` mirror to take them from.
  4. Line count: C 2524 vs Rust 2399 (4.9%, inside the default 5% tolerance the rule allows
     every file, so no `docs/ports/line-drift-tolerance.tsv` row is owed).  The whole
     difference is the ~330 lines of dead platform arms plus the three dropped prototypes;
     every live statement is present.
- **Extractor gaps (carried, but absent from this checklist).**  `cinventory.globals_`'s
  declarator splitter drops a *mid-declarator* `const` and never reads an initialised array, so
  the checklist's "File-scope objects — 12" misses C 102 `progname`, C 108 `main_argv`, C 129
  `display`, C 132 `alt_display`, C 145-147 `socket_name`, C 150 `server_file`, C 157
  `tramp_prefix`, C 164 `frame_parameters` and the two option tables (C 168-190, C 191-198); it
  also has no row for the `enum` constants (`AUTH_KEY_LENGTH` C 799, `socknamesize` C 1609, and
  the two function-local ones `SEND_BUFFER_SIZE` C 834 / `DEFAULT_RECV_BUFSIZE` C 2388).  All of
  them are implemented in the port; the gap is in the tool, and it is reported.
- **Owed note rows.**  `docs/ports/macro-body-notes.tsv` is one of the integrator's shared
  tables, so these nine rows are handed over rather than written by this lane (each names the
  functions of *this* file, and a row's name column matches every unit, so a reader should
  check the collisions when merging):
```
socknamesize	null	emacsclient: C 1609's `enum { socknamesize = sizeof ((struct sockaddr_un *) NULL)->sun_path };` is C's array-size idiom (docs/ports/emacsclient.md)
message	va_end,va_list,va_start	emacsclient: C 468-493's `va_list args; va_start (args, format); … va_end (args);` have no port counterpart -- the `...` parameter *is* the list (PORTING.md P-04, the make-docfile `error`/`fatal` rows)
decode_options	case,default,switch	emacsclient: C 512's `switch (opt)` is the port's `match opt`, whose arms carry C's `case` labels as numeric patterns with the character in a comment; the arms' `break`s have no `match` site (PORTING.md 2.2, the make-docfile `write_globals` row)
connect_with_timeout	case,default,goto,switch	emacsclient: C 1158's `switch (rc)` is a `match`, and C 1125/1132/1139/1151/1163/1174/1179's `goto done`/`goto timeout` are the labelled blocks `'done`/`'timeout` whose `break` is the same jump (PORTING.md 2.2, the getdelim/realpath_stk precedent)
print_help_and_exit	accepted,again,alist,alternate,and,are,arguments,authentication,before,bug,bugs,can,column,communication,connecting,create,current,daemon,display,don,editor,either,elisp,emacs,emacsclient,empty,eval,evaluate,every,exists,expressions,fallback,file,filename,filenames,files,following,frame,from,given,help,info,information,instead,just,line,locating,messages,mode,name,new,none,not,open,options,otherwise,out,output,parameters,parent,prefix,prepend,print,quiet,remotely,report,reuse,running,seconds,sent,set,socket,specified,start,string,success,suppress,system,tcp,tell,terminal,the,this,timeout,timing,tramp,try,trying,tty,unix,usage,use,values,version,via,visit,wait,window,with,xembed	emacsclient: C 689-728's usage message is one C string literal written with backslash-newline continuations, which `cinventory`'s literal blanker does not see, so every word of the message turns into a body token; the port carries the message as four Rust literals whose bytes the `message` call keeps (the words are the message's own text, not code)
install_handler	sa_handler	emacsclient: C 1499/1502's `oact.sa_handler` / `.sa_handler = handler` is the `sigaction` union's `__sa_handler` member, which the signal.rs mirror spells `__sigaction_u.__sa_handler` (PORTING.md P-27's `Option` shape; the src/process.rs 8697 row)
local_sockname	priumax	emacsclient: C 1631-1632's `"/emacs%"PRIuMAX"/%s"` uses <inttypes.h>'s `PRIuMAX`, which the frozen target defines as "ju" (measured); the port writes the measured format as one literal, so the macro name cannot reappear (the lib-src/hexl.rs PRIxMAX row)
set_local_socket	priumax	emacsclient: see local_sockname (C 1800-1801's `"/run/user/%"PRIuMAX`)
set_socket	alternate,alternate_editor,editor,emacs_server_file,environment,file,variable	emacsclient: C 1882-1888's message is a concatenation whose last literal is written with a backslash-newline continuation, which the literal blanker does not see, so the message's words turn into body tokens; the port carries them as one Rust literal
```

## Evidence

- **The file's conditions.**  `python3 tools/check-cond-eval.py lib-src/emacsclient.c` ->
  `46 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file is in
  `docs/baseline/config/cond-eval-verified.txt`, so `extract-abi.py --check` evaluates its `#if`s
  against the frozen macro table).
- **Reference object and exported names.**  `make -C build/emacs-ref/lib-src emacsclient.o`
  fails (the generated Makefile's compile line has no `-I../src`), so the object was built by
  hand as recorded in Decisions; `nm -g /tmp/emacsclient-ref.o` prints the two `T` definitions
  and the import list quoted there.  The port's two `#[unsafe(no_mangle)]` names are exactly
  those two.
- **The port compiles.**  `CARGO_TARGET_DIR=/tmp/femacs-target-ec cargo run -q -p xtask --
  build-tu lib-src/emacsclient.c` ends with
  `built lib-src/emacsclient.rs -> ../build/femacs/emacsclient.o` (14 warnings: the 1.5
  "never read" ones, the `stdlib.rs`/`unistd.in.rs` mirrors' `read`/`write` lint, the
  `sa_handler == sig_ign` function-pointer comparison, and two `unnecessary parentheses` where
  C's own parentheses were kept).
- **The manual gate.**  `cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata
  --crate-name ec_check -C panic=abort -C codegen-units=1 -C opt-level=0
  ../build/femacs/units/emacsclient/_root.rs -o /tmp/ec.rmeta` -> exit 0.
- **The ABI checklist.**  `python3 tools/extract-abi.py --check emacs/lib-src/emacsclient.c
  lib-src/emacsclient.rs` -> the ten problem lines quoted in STATUS above; nine are the
  owed rows.  Adding the rows makes the command print `OK` (no port change needed); the
  line-count drift is 4.9%, inside the 5% the rule allows every file.
- **The measurements** every use-site declaration quotes were taken with the frozen target's
  clang (`printf` probes): `AF_INET` 2 / `AF_UNIX` 1 / `SOCK_STREAM` 1 / `IPPROTO_TCP` 6 /
  `SOL_SOCKET` 0xffff / `SO_LINGER` 0x80 / `SO_ERROR` 0x1007 / `SO_RCVTIMEO` 0x1006;
  `sockaddr` 16 (`sa_len`@0, `sa_family`@1), `sockaddr_in` 16 (`sin_addr`@4, `sin_zero`@8),
  `sockaddr_un` 106 (`sun_path`@2, 104 bytes), `linger` 8, `in_addr` 4, `socklen_t` 4;
  `sig_atomic_t` 4; `WUNTRACED` 0x2 / `WCONTINUED` 0x10 / `STDOUT_FILENO` 1 /
  `_CS_DARWIN_USER_TEMP_DIR` 65537, `PRIuMAX` "ju" / `PRIxMAX` "jx", `sizeof (uintmax_t)` 8,
  `PTRDIFF_MAX`/`SIZE_MAX`, `O_PATH` undefined on this target, `SOCK_CLOEXEC` undefined.
  The three `sizeof "…"` values `open_config` uses are 15 (`"/emacs/server/"`), 23
  (`"/.config/emacs/server/"`) and 18 (`"/.emacs.d/server/"`).
- **The nine rows are sufficient, verified rather than asserted.**  Writing them to a *copy*
  of the table (`/tmp/ec-notes.tsv`, not the integrator's file) and running
  `python3 tools/extract-abi.py --check --macro-notes /tmp/ec-notes.tsv
  emacs/lib-src/emacsclient.c lib-src/emacsclient.rs` prints
  `OK emacs/lib-src/emacsclient.c -> lib-src/emacsclient.rs` with no problem line at
  all.  So the port needs no change for the gate; the nine rows above are the whole
  remainder.
- **The four house gates** (each from the repository root, exit status 0):
  `python3 tools/check-globals.py` -> `check-globals: 274 units, 0 DEFVAR_* variables without a
  definition in their own unit`; `python3 tools/check-declarations.py` -> `8 declaration sites
  (0 forbidden, 8 allowed literals), 0 failures` (this unit adds none: its comments carry no
  product name, and the usage message's own "Emacs" is the *word*, not the product name the
  rule reads); `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet
  row(s), 0 failure(s)` (the row is `src/insdel.rs`'s, pre-existing -- this unit declares
  nothing a mirror in its include graph already carries); `python3 tools/check-verbatim.py` ->
  `6 trees, 4607 files, 0 failures`.
