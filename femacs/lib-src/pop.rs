// femacs: port of lib-src/pop.c @ fd7afced403.
/* femacs: emacs/lib-src/pop.c (@ 1703 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MAIL_USE_POP` at C line 26, and the frozen target's
   configuration leaves MAIL_USE_POP undefined (docs/baseline/config/darwin-arm64.macros.tsv
   2691, `MAIL_USE_POP<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its one include (C 24, `<config.h>`).  The reference object is the measurement: the
   `lib-src/Makefile` recipe builds `pop.o` and links it into the program, yet `nm
   build/emacs-ref/lib-src/pop.o` reports no symbol and `size` reports zero text bytes.

   What the file implements when the feature is on is the POP3 client - `pop_open`, `pop_stat`,
   `pop_list`, `pop_retrieve`/`_first`/`_next`/`_flush`, `pop_top_first`/`_next`/`_flush`,
   `pop_multi_first`/`_next`/`_flush`, `pop_delete`, `pop_noop`, `pop_last`, `pop_reset`,
   `pop_quit`, `pop_close` plus the static `socket_connection`, `pop_getline`, `sendline`,
   `fullwrite`, `getok`, `pop_trash` and `find_crlf` - and it defines the two objects
   `lib-src/pop.h` declares, `pop_error` (C 133, `ERROR_MAX` 160) and `pop_debug` (C 134).  That
   text needs `<sys/socket.h>`/`<netinet/in.h>`/`<netdb.h>`/`<pwd.h>`/`<alloca.h>`, and its
   HESIOD, KERBEROS (v4/v5) and WINDOWSNT arms add `<hesiod.h>`, `<krb5.h>`/`<krb.h>` and
   `<winsock2.h>`/`<ws2tcpip.h>`; the frozen configuration defines none of those three
   selectors.

   Under docs/status.md 3.25 a file whose *whole* body is inside a disabled `#if` is ported as
   an empty unit, and 3.49 keeps a file that the condition evaluator can decide (this one is in
   docs/baseline/config/cond-eval-verified.txt) on the P-08 basis: the dead region is recorded,
   not carried, because an arm this configuration does not compile is an invented item - the D3
   class `src/data.rs` records for the `#[cfg (feature = "modules")]` arm it reverted.  The port
   therefore has the same zero definitions the reference compiles here, and the gating macro is
   recorded so a future configuration that enables it knows which `#if` to re-read.  The
   `<pop.h>` mirror's declarations are not reached from this unit either, since C's
   `#include <pop.h>` (C 53) sits inside the same dead region.  */
