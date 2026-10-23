# emacs/lib-src/pop.c (1703 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

> **One of P-86's class: the frozen configuration compiles nothing here.**  The whole body sits
> inside C 26's `#ifdef MAIL_USE_POP`; `MAIL_USE_POP` is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` 2691, so the checklist above reads zero in every
> section and `tools/extract-abi.py --check` has no live item to compare — P-86 ②/⑩: the
> checklist is vacuous here and this table, not a tick box, is the coverage evidence.  What the
> frozen build *does* say is the measurement: `lib-src/Makefile` 431/435 lists `pop.o` for the
> movemail program and compiles it, and the resulting object carries no symbol (`nm` prints only
> its own `ltmp0` local marker) and no text (`size` prints `0 0 0 0 0 0`).
>
> **This ledger replaces, not extends, what it said before.**  Until this revision it read "a file
> whose whole body is inside a disabled `#if` is ported as an empty unit with the note that names
> the gate" (the P-08 reading).  P-86 ① replaces that with *transcribe the text, both arms, no
> stub*: the port is `lib-src/pop.rs` (**2204 lines**, 84,117 bytes, md5
> `ebe410a3dcf2f6a2fb95d7d29b6694b9`), carrying C 1-1703 — the reference's every line — with both
> arms of every `#if`, and the frozen object is *still* the 336-byte empty one because every item
> sits under the file gate (Evidence ①).
>
> The file implements the POP3 client when the feature is on: the eighteen functions
> `lib-src/pop.h` declares (`pop_open`, `pop_stat`, `pop_list`, `pop_retrieve`/`_first`/`_next`/
> `_flush`, `pop_top_first`/`_next`/`_flush`, `pop_multi_first`/`_next`/`_flush`, `pop_delete`,
> `pop_noop`, `pop_last`, `pop_reset`, `pop_quit`, `pop_close`), the seven static helpers
> (`socket_connection`, `pop_getline`, `sendline`, `fullwrite`, `getok`, `pop_trash`,
> `find_crlf`), the two file-scope objects `pop_error` (`ERROR_MAX` 160) and `pop_debug`, and the
> three Windows-only entries `load_ws2`, `sys_getaddrinfo`, `sys_freeaddrinfo` (C 1572-1701).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself, C 26's `#ifdef MAIL_USE_POP` | `feature = "mail_use_pop"` — on every item **and on C 27-100's include graph** (P-86 ⑧) | `MAIL_USE_POP` is defined |
| C 29-49 (`WINDOWSNT`: `ntlib.h`, `_WIN32_WINNT`, `<windows.h>`/`<winsock2.h>`, `<ws2tcpip.h>`, the `sys_*` prototypes, the three macros) | `feature = "windowsnt"` | a Windows build |
| C 33 (`__MINGW32_VERSION >= 5000002L`) | `feature = "mingw32_version_ge_5000002"` | a MinGW at least that new |
| C 50-56 (`#else`: `<netinet/in.h>`, `<sys/socket.h>`, `read`/`write`/`close`) | `not (feature = "windowsnt")` | every non-Windows build (the frozen one included) |
| C 59-68 (`HESIOD`) | `feature = "hesiod"` | `HESIOD` |
| C 79-97 (`KERBEROS` + its five header selectors) | `feature = "kerberos"` with `have_krb5_h`, `have_krb_h`, `have_kerberosiv_krb_h`, `have_kerberos_krb_h`, `have_com_err_h` | `KERBEROS` |
| C 102-110 (`KERBEROS && !KERBEROS5`) | `all (feature = "kerberos", not (feature = "kerberos5"))` | Kerberos V4 |
| C 128-131, 233-237, 984-1000, 1014-1018, 1043-1048, 1106-1198 (`KERBEROS`) | `feature = "kerberos"` (C 985/1111's `KERBEROS5` arm nested) | `KERBEROS` |
| C 1161/1168 (`HAVE_KRB5_ERROR_TEXT` / `HAVE_KRB5_ERROR_E_TEXT`) | `feature = "have_krb5_error_text"` / `"have_krb5_error_e_text"` | each header |
| C 204-217, 1022-1032 (`HESIOD`) | `feature = "hesiod"` | `HESIOD` |
| C 219-224 (`MAILHOST`) | `feature = "mailhost"` | `MAILHOST` |
| C 956-958, 1005-1011, 1537-1540, 1563-1702 (`WINDOWSNT`) | `feature = "windowsnt"` | a Windows build |
| C 1455-1485 (`#if 0`) | no gate — dead in every configuration | never |

`windowsnt` is already registered (`docs/baseline/feature-coverage.tsv` 8); the other thirteen
names are Owed 1.  `MAIL_USE_LOCKF`, `MAIL_USE_SSL`, `MAIL_USE_TLS` and `HAVE_GETADDRINFO`
appear nowhere in this file (they belong to `lib-src/movemail.c`): the 31 conditions the
evaluator reports are the whole set.

## Decisions

- **The gate is measured, not inferred.**  `python3 tools/check-cond-eval.py lib-src/pop.c` ->
  `31 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and every macro those 31 conditions
  name is `undef` in the frozen table: `MAIL_USE_POP` 2691, `WINDOWSNT` 4010, `HESIOD` 1731,
  `MAILHOST` 2687, `KERBEROS` 2280, `KERBEROS5` 2281, `HAVE_KRB5_H` 1407, `HAVE_KRB_H` 1408,
  `HAVE_KERBEROSIV_KRB_H` 1402, `HAVE_KERBEROS_KRB_H` 1403, `HAVE_COM_ERR_H` 1251,
  `HAVE_KRB5_ERROR_TEXT` 1406, `HAVE_KRB5_ERROR_E_TEXT` 1405.  `__MINGW32_VERSION` is undefined
  too, which is what decides C 33.
- **The reference object is the measurement.**  Both objects print the same two lines:
  `build/emacs-ref/lib-src/pop.o` and the port's own `build/femacs/pop.o` are both 336 bytes with
  `nm` -> `0000000000000000 t ltmp0` and `size` -> `0 0 0 0 0 0` (Evidence ①).  That is what
  "one-to-one" means for a translation unit whose frozen text is zero definitions - and it is now
  a *consequence* of the file gate rather than a reason not to write the text.
- **Use-point declarations (PORTING.md 0.1-3): nothing to route.**  C's only use of the `<pop.h>`
  declarations is `#include <pop.h>` at C 53, and the port includes the mirror exactly there
  (`include!("include/pop.rs")` inside the file gate).  The unit defines every one of the
  eighteen prototypes and both objects, so the mirror's own `use pop_decls::*` /`pop_objs::*`
  globs are shadowed by definitions (0.1-10), and no declaration is repeated from the mirror.
- **The two `#if 0` regions** (C 117-119's `gettermination` prototype, C 1455-1485's definition)
  are dead in every configuration.  C 117-119 is a forward declaration and follows the 0.1-12
  rule with its six live siblings; C 1455-1485 is a definition, carried as C's own text inside a
  comment (the `src/cm.rs` precedent, Deviation 6).
- **Line-count relation**: C 1703 -> Rust 2204 (+29.4%).  The ratchet now reports
  `stale lib-src/pop.c: no longer under the threshold`, so the
  `docs/baseline/port-lines-pending.tsv` row is the integrator's to remove (Owed 2).
- **What the file needs that the tree does not carry**: `<sys/socket.h>`, `<netinet/in.h>`,
  `<netdb.h>`, `<hesiod.h>`, the five Kerberos headers and the three Windows socket headers have
  no mirror, so their records, constants and entries are declared **in the unit** at the C
  `#include` line that brings them (P-88 ①) - `src/include/` and `lib/include/`
  are untouched (Deviation 10; Owed 6).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-21 | the file's licence/declaration block | dropped (P-86 ①/0.1-19; the port opens with `// femacs: port of lib-src/pop.c @ fd7afced403.`) |
| 22-24 | the blank pair + `#include <config.h>` | ungated (every unit includes it) |
| 25-28 | blank, `#ifdef MAIL_USE_POP`, blank, `#include <sys/types.h>` | file gate |
| 29-56 | the two socket surfaces: C 29-49 (`WINDOWSNT`: `ntlib.h`, `_WIN32_WINNT`, `<windows.h>`/`<winsock2.h>`, `<ws2tcpip.h>`, the `sys_*` prototypes, `RECV`/`SEND`/`CLOSESOCKET`) and C 50-56 (`#else`: `<netinet/in.h>`, `<sys/socket.h>`, `read`/`write`/`close`) | C 29-49 `windowsnt` (C 33 `mingw32_version_ge_5000002`), C 50-56 `not (windowsnt)` |
| 57 | `#include <pop.h>` | file gate |
| 58-68 | the `HESIOD` block (`<hesiod.h>`, the `hes_getservbyname` declaration) | `hesiod` |
| 69-77 | `<alloca.h>`, `<pwd.h>`, `<netdb.h>`, `<errno.h>`, `<stdio.h>`, `<stdlib.h>`, `<string.h>`, `<unistd.h>` | file gate |
| 78-97 | the `KERBEROS` header chain (the five selectors) | `kerberos` + the five `have_*` gates |
| 98-100 | blank + `<c-ctype.h>` + `<min-max.h>` | file gate |
| 101-110 | `KERBEROS`+`!KERBEROS5`: the `krb_sendauth`/`krb_realmofhost` declarations | `all (kerberos, not (kerberos5))` |
| 111-121 | the seven static forward declarations (C 117-119's `#if 0` among them) | dropped (0.1-12, Deviation 2) |
| 122-131 | `ERROR_MAX` 160, `POP_PORT` 110, `POP_SERVICE` "pop3", `KPOP_PORT`/`KPOP_SERVICE` | file gate; C 128-131 `kerberos` |
| 132-134 | `pop_error[ERROR_MAX]`, `pop_debug` | file gate |
| 135-164 | `pop_open`'s doc comment | carried as the port's prose block |
| 165-317 | `pop_open` (C 204-217 `hesiod`; C 219-224 `mailhost`; C 233-237 `kerberos`) | file gate + those |
| 318-331 | `pop_stat`'s doc comment | carried |
| 332-382 | `pop_stat` | file gate |
| 383-402 | `pop_list`'s doc comment | carried |
| 403-523 | `pop_list` | file gate |
| 524-544 | `pop_retrieve`'s doc comment | carried |
| 545-617 | `pop_retrieve` | file gate |
| 618-624 | `pop_retrieve_first` | file gate |
| 625-638 | the retrieve-next comment + `pop_retrieve_next` | carried / file gate |
| 639-644 | `pop_retrieve_flush` | file gate |
| 645-651 | `pop_top_first` | file gate |
| 652-665 | the top-next comment + `pop_top_next` | carried / file gate |
| 666-671 | `pop_top_flush` | file gate |
| 672-705 | `pop_multi_first` (C 695's empty-body `for`) | file gate (Deviation 4) |
| 706-753 | the multi-next comment + `pop_multi_next` | carried / file gate |
| 754-773 | `pop_multi_flush` | file gate |
| 774-801 | `pop_delete`'s comment + `pop_delete` | carried / file gate |
| 802-829 | `pop_noop`'s comment + `pop_noop` | carried / file gate |
| 830-887 | `pop_last`'s comment + `pop_last` | carried / file gate |
| 888-914 | `pop_reset`'s comment + `pop_reset` | carried / file gate |
| 915-954 | `pop_quit`'s comment + `pop_quit` | carried / file gate |
| 955-958 | blank + `#ifdef WINDOWSNT` + `have_winsock` + `#endif` | `windowsnt` |
| 959-973 | `socket_connection`'s doc comment | carried |
| 974-1201 | `socket_connection` (C 984-1000 the two Kerberos local sets; C 1005-1011 `WSAStartup`; C 1014-1018 the service choice; C 1020 `addr`; C 1022-1032 Hesiod; C 1033-1050 `getservbyname`/`POP_PORT`; C 1052 `POP_SOCKET_ERROR`; C 1054 `socket`; C 1063-1068 `hints`; C 1069-1078 `getaddrinfo`; C 1080-1087 the connect loop; C 1091-1092 `alloca`; C 1094 `freeaddrinfo`; C 1096 `CONNECT_ERROR`; C 1098-1104 the close; C 1106-1198 the KPOP block) | file gate; `windowsnt`/`not (windowsnt)`; `kerberos`; `kerberos5`; `have_krb5_error_text`/`have_krb5_error_e_text`; `hesiod` |
| 1202-1225 | `pop_getline`'s doc comment | carried |
| 1226-1335 | `pop_getline` (C 1229 `GETLINE_ERROR`; C 1250/1323 the two debug prints) | file gate |
| 1336-1354 | `sendline`'s doc comment | carried |
| 1355-1389 | `sendline` (C 1358 `SENDLINE_ERROR`; C 1374 `alloca`) | file gate |
| 1390-1398 | `fullwrite`'s doc comment | carried |
| 1399-1413 | `fullwrite` | file gate |
| 1414-1428 | `getok`'s doc comment | carried |
| 1429-1453 | `getok` | file gate |
| 1454-1485 | C 1455-1485's `#if 0` `gettermination` | dead in every configuration; carried as C's text in a comment (Deviation 6) |
| 1486-1506 | `pop_close`'s comment + `pop_close` | carried / file gate |
| 1507-1541 | `pop_trash`'s comment + `pop_trash` (C 1537-1540 the winsock cleanup) | carried / file gate; `windowsnt` |
| 1542-1561 | `find_crlf`'s comment + `find_crlf` | carried / file gate |
| 1562-1570 | the `#ifdef WINDOWSNT` head, its comment, the two function-pointer statics | `windowsnt` |
| 1571-1602 | `load_ws2` | `windowsnt` |
| 1603-1682 | `sys_getaddrinfo` (C 1621-1624 the local `gai_storage` record) | `windowsnt` |
| 1683-1701 | `sys_freeaddrinfo` | `windowsnt` |
| 1702-1703 | the two `#endif`s | recorded as the port's last two comment lines |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/pop-lane/cargo cargo run -q -p xtask -- build-tu
lib-src/pop.c` -> `built lib-src/pop.rs -> ../build/femacs/pop.o`, **exit 0**, and the object is
**336 bytes**.  `nm build/femacs/pop.o` -> `0000000000000000 t ltmp0`;
`size build/femacs/pop.o` -> `__TEXT __DATA __OBJC others dec hex` / `0 0 0 0 0 0`.  The
reference's own object prints the two lines unchanged:
`nm build/emacs-ref/lib-src/pop.o` -> the same `ltmp0`, `size` -> the same zeros.  A direct
`rustc --emit obj` on xtask's final materialisation of the frozen feature set also gives 336
bytes with those two lines.

**② The arm flip is a real compile of the whole body** (P-86 ③⑴: this file's surface is POSIX,
and every reader exists on this machine).
`cd femacs && CARGO_TARGET_DIR=/tmp/pop-lane/cargo cargo run -q -p xtask -- probe lib-src/pop.rs
/tmp/pop-lane/pop.rlib --crate-type=lib -C panic=abort --cfg 'feature="mail_use_pop"'` ->
**exit 0**, **2 warnings**, a **274,640-byte rlib**.  With every POSIX-side sub-gate as well
(`--cfg 'feature="mail_use_pop"' --cfg 'feature="kerberos"' --cfg 'feature="kerberos5"'
--cfg 'feature="hesiod"' --cfg 'feature="mailhost"'`, what P-86 ⑥ asks for) -> **exit 0**,
**2 warnings**, a **291,336-byte rlib**.  Both warnings are
`unused`-class `suspicious_runtime_symbol_definitions` on `read`/`write` in the
`lib/include/unistd.in.rs` mirror (`ssize_t` spelled `c_long` there), **not** in this unit's text
– the same mirror-side warning class `docs/ports/update-game-score.md` records.

**③ The other arms.**  Each was flipped on the same materialisation, one gate at a time:

| gates | exit | errors | where |
| --- | --- | --- | --- |
| `mail_use_pop` + `mailhost` | 0 | 0 | — |
| `mail_use_pop` + `hesiod` | 0 | 0 | — |
| `mail_use_pop` + `kerberos` | 0 | 0 | — |
| `mail_use_pop` + `kerberos` + `kerberos5` | 0 | 0 | — |
| `mail_use_pop` + `windowsnt` | 1 | 6 | **all six in the include graph**: `lib/include/unistd.in.rs` defines `getwd`, `getppid`, `getgid`, `getegid`, `fchown`, `sleep`, each already declared by the `lib-src/include/ntlib.rs` mirror this arm includes - 6 x `E0428`, 0 in the unit's own text |

The `windowsnt` reading is checked against P-86 ⑦'s trap rather than trusted: injecting a
deliberate type error into the unit's own text (`pop_delete`'s `return -1;` ->
`return c"mutation".as_ptr ();`) **does** change that run's output (6 -> 7 errors, with the
injected `E0308` in the unit's own line), so the "0 in the unit's text" above is a real reading
and not the name-resolution abort that rule describes.  The arm still cannot be *compiled*
here: no Windows SDK (`which x86_64-w64-mingw32-gcc` is empty), and the six mirror-side
duplicates are Owed 7.

**④ Inventory and the ledger's own gates.**
`python3 tools/extract-abi.py --check emacs/lib-src/pop.c lib-src/pop.rs` ->
`OK emacs/lib-src/pop.c -> lib-src/pop.rs`, **no `- ` line at all** - unlike
`src/androidmenu.c`'s two (a `goto` token and a 17.6% drift) and
`lib-src/update-game-score.c`'s five token rows.  The reason is P-86 ⑩'s own boundary: the
token rows are keyed to *live* functions, and the frozen live set of this file is empty, so the
`OK` is vacuous for the port's text (which is why this table is the coverage evidence).
`python3 tools/check-dead-file-coverage.py` -> the table above accounts for C 1-1703.
`python3 tools/check-cond-eval.py lib-src/pop.c` -> `31 conditions, 0 unknown, 0 disagreements
[VERIFIED]`.  `python3 tools/check-port-lines.py` -> `stale lib-src/pop.c: no longer under the
threshold` (0 new, 2 stale rows; the other stale row belongs to another lane).
`python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range, 720 external, 35
allowed, 0 stale`, and **no `pop` row at all** in any class.  `python3
tools/check-declarations.py` -> no `pop` row (the run's one `recital` failure is in
`src/sound.rs`, another lane's unit).  `python3 tools/check-target-names.py` -> `629 .rs
files, 591 map targets, 0 finding(s)`; `tools/check-mirror-dupes.py` -> `19 duplicate
declaration(s), 19 ratchet row(s), 0 failure(s)`; `tools/check-ledger-ticks.py` -> `0 failure(s)`
(this ledger's five counting sections are empty, which that tool counts as 356-row class "a unit
whose body the frozen configuration leaves under a false condition").

**⑤ The layouts and values measured with clang** (P-88 ②: measured, not recalled) for the
surfaces the port declares in-unit:
`sizeof struct sockaddr_in` 16 (`sin_family`@1, `sin_port`@2, `sin_addr`@4), `struct in_addr` 4,
`struct sockaddr` 16, `struct addrinfo` 48 (`ai_flags`@0, `ai_family`@4, `ai_socktype`@8,
`ai_protocol`@12, `ai_addrlen`@16, `ai_canonname`@24, `ai_addr`@32, `ai_next`@40),
`struct servent` 32 (`s_port`@16, `s_proto`@24), `struct hostent` 32 (`h_addrtype`@16,
`h_length`@20, `h_addr_list`@24), `socklen_t` 4, `in_port_t` 2, `sa_family_t` 1; `AF_INET` 2,
`PF_INET` 2, `SOCK_STREAM` 1, `SOCK_DGRAM` 2, `AI_CANONNAME` 2, `EAI_AGAIN` 2, `ENETDOWN` 50.
The Windows and Kerberos surfaces cannot be measured here and are marked UNVERIFIED (Owed 8).

## Deviations

1. C 1-21's licence/declaration block is not carried (0.1-19); the unit opens with the
   `// femacs: port of lib-src/pop.c @ fd7afced403.` line.
2. C 112-121's seven static forward declarations are dropped (0.1-12), the C 118 one included;
   every definition sits at its own C position.  C 39-45's two `sys_*` prototypes are dropped the
   same way (their definitions are at C 1603/1684).
3. C's uninitialised locals are zero-initialised (1.5): `fromserver`, `end_ptr`, `fromline`,
   `line`, `realhost`, `IDs`, `sizes`, `res`, `it`, `servent`, `kcontext`, `auth_context`,
   `ccdef`, `client`, `server`, `err_ret`, `cp`, `ticket`, `msg_data`, `cred`, `schedule`.  The
   designated initialisers of C 1020 and C 1063-1068 are a whole-record zero followed by the
   three field writes (0.1-13).
4. Statement shapes Rust has no counterpart for: `RECV`/`SEND`/`CLOSESOCKET` are `macro_rules!`
   invoked with `!`; C 695's empty-body `for` is the `while` that tests `**response`; C 1550's
   `while (len--)` is a loop that tests the old value and then decrements; the `do { } while` of
   C 1069-1078 and the `while ((ret = f ()))` of C 584/766/1406 are `loop` + `break`.
5. C's implicit conversions are explicit `as` casts: `strtol`'s `c_long` -> `c_int`
   (C 361/373/878), `read`/`write`'s `ssize_t` -> `c_int`, `servent.s_port` `c_int` -> `u16`
   (C 1028/1038), `AF_INET` -> `sa_family_t` (C 1020), `*mut c_char` -> `*const c_char` at every
   `const char *` parameter, and `host` -> `*const c_char` at C 1071.  C 53-54's POSIX
   `RECV`/`SEND` macros hand `read`/`write` **three** arguments (the macro's `flags` is unused on
   that arm); the port's POSIX macro carries the two casts the prototype leaves implicit.
6. C 117-119 and C 1455-1485 are the file's two literal `#if 0` regions, dead in every
   configuration.  C 117-119 is a forward declaration and is dropped as Deviations 2; the
   C 1455-1485 definition is carried as C's text inside a comment (the `src/cm.rs` precedent).
7. C 33's `#if defined __MINGW32_VERSION && __MINGW32_VERSION >= 5000002L` is a value test, not
   a `defined` gate, so it takes the `mingw32_version_ge_5000002` name (`src/w32.rs`'s
   `mingw32_w32api_lt_3_15` shape) and the `0x0501` re-pin is carried as a named constant.
8. C 39-42's `#define getaddrinfo sys_getaddrinfo` / `#define freeaddrinfo sys_freeaddrinfo` are
   renames of a call site, which `macro_rules!` cannot express; the two call sites (C 1071,
   C 1094) write the `sys_` spelling inside the `windowsnt` gate, with the `not (windowsnt)` arm
   calling the declared entry.
9. C 1115-1144's five `goto krb5error` jumps are the flag-and-tail state machine D9 asks for
   (`krb5_failed` plus the shared tail); the arm is text-only here anyway.
10. The surfaces with no mirror are declared in the unit (P-88 ①) at the C `#include` line that
    brings them: `<sys/types.h>`/`<pwd.h>`/`<errno.h>`/`<stdio.h>`/`<stdlib.h>`/`<string.h>`/
    `<unistd.h>`/`<alloca.h>`/`<c-ctype.h>`/`<min-max.h>` are the tree's mirrors, and
    `<netinet/in.h>`+`<sys/socket.h>`, `<netdb.h>`, `<hesiod.h>`, the five Kerberos headers and
    the three Windows socket headers are declared in place with `/// C:` lines.  Each such
    section records "this section is an external header's stand-in; the reference tree has no
    such file" (P-88 ④).
11. `atoi`, `strnlen`, `getlogin`, `getpass`, `strdup` and the port's `sa_stack_alloc` (the
    `alloca` spelling, P-05) are declared at their use sites because no mirror in this graph
    carries them; the two Windows names come from the `ntlib.h` mirror instead, so they are
    gated.
12. C's `/** ... **/` comments (C 255, 1013, 1042) are written `/* ... */`: Rust reads `/**` as
    a doc comment, which attaches to the following expression (`E0658`).
13. `<pwd.h>`'s `struct passwd` spells two fields with `time_t`, which the `pwd.rs` mirror asks
    its includer for; the port includes `time.in.rs` at C 71's section (the
    `lib-src/update-game-score.rs` 42 shape).
14. `MAILHOST`'s value (C 219-224) is a configure parameter with no frozen value: the arm
    carries a placeholder (Owed 3).  The Windows/Kerberos layouts and values the port declares
    are UNVERIFIED placeholders or POSIX measurements (Owed 8).

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`xtask/Cargo.toml`'s feature registration** for the thirteen new names:
   `mail_use_pop`, `hesiod`, `mailhost`, `kerberos`, `kerberos5`, `have_krb5_h`, `have_krb_h`,
   `have_kerberosiv_krb_h`, `have_kerberos_krb_h`, `have_com_err_h`, `have_krb5_error_text`,
   `have_krb5_error_e_text`, `mingw32_version_ge_5000002` (`windowsnt` is already there).  Until
   that lands, a Cargo-driven build cannot pass `--cfg feature="mail_use_pop"` and the
   `unexpected_cfgs` lint will warn; the readings above used `rustc --cfg` directly, which
   needs no registration.
2. **The ratchet row**: `docs/baseline/port-lines-pending.tsv`'s `lib-src/pop.c` row is now
   `stale` ("no longer under the threshold") and comes out with
   `python3 tools/check-port-lines.py --write-pending`.  This lane may not write that file.
3. **`MAILHOST`'s value** - a configure run with `--with-mailhost` supplies the string the
   `mailhost` arm assigns at C 222; the port's `c""` is a placeholder, so that arm is *compiled*
   but not *correct*.
4. **`docs/ports/macro-body-notes.tsv`: no row is owed.**  Measured, not assumed:
   `extract-abi.py --check` printed **no `- ` line at all** for this pair, so the `goto`
   token class that gave `src/androidmenu.c` and `src/sfnt.c` rows is not reached here - the
   checklist's live set is empty for a file the frozen `#if` closes (P-86 ⑩).  Named because
   the brief lists it.
5. **`docs/ports/line-drift-tolerance.tsv`: no row is owed** by the same reading.  The numbers,
   if the table later binds this pair, are C 1703 -> Rust 2204 (2204/1703 = 129.4%, +29.4%).
6. **The mirror gap this file names and does not close**: no mirror carries `<netdb.h>`,
   `<sys/socket.h>` or `<netinet/in.h>`, so the socket records live in the unit (Deviation 10);
   and `docs/ports/include-pop.md`'s layout assertion block is still reached by no unit - the
   `pop.h` mirror's two consumers (this file and `lib-src/movemail.c`) both include it only
   inside false conditions, so the `build-tu` proof item that ledger's 4 expects does not
   materialise.  Unchanged from the previous revision of this ledger.
7. **The `windowsnt` arm's blocker is in the mirror set, not in this unit's text**:
   `lib/include/unistd.in.rs` defines `getwd`/`getppid`/`getgid`/`getegid`/`fchown`/`sleep` and
   the `lib-src/include/ntlib.rs` mirror declares the same six, so any arm that includes both
   reads 6 x `E0428` (Evidence ③).  Until that is resolved, no Windows flip can claim the arm's
   text compiles (its *own* text is checked - Evidence ③'s mutation reading).
   ✅ **Closed by the integrator in 3.483**: the six declarations in `lib/include/unistd.in.rs` are
   masked **for the `windowsnt` build only** (`#[cfg (not (feature = "windowsnt"))]`), because C
   lets one name carry two compatible declarations (Windows spells them `int`/`unsigned`, the
   generic mirror spells them `pid_t`/`gid_t`/`uid_t` - the same types under the Windows headers)
   while Rust does not, and the *platform* header's own spelling is what a Windows unit should
   see.  Measured: this unit's `windowsnt` arm went from **6 errors to `exit 0`** (a 298,144-byte
   rlib, `--cfg 'feature="mail_use_pop"' --cfg 'feature="windowsnt"'`), while the `not(windowsnt)`
   arms are untouched by construction.  What stays owed: the Windows surface is still `text-only`
   (no mingw here) and Owed 8's values/layouts remain UNVERIFIED.
8. **The UNVERIFIED surfaces** (no Windows SDK, no Kerberos SDK, no Hesiod library on this
   machine): `WSADATA`, `HANDLE`, the three Windows constants and `WSA*` codes, `EAI_AGAIN`'s
   Windows value (a placeholder), `addrinfo.ai_addrlen`'s width on Win64 (the port carries the
   measured 32-bit `socklen_t` for both arms), `struct hes_postoffice`, and the `krb5_*` /
   `KTEXT_ST` / `MSG_DAT` / `CREDENTIALS` / `Key_schedule` / `KSUCCESS` /
   `AP_OPTS_MUTUAL_REQUIRED` surface.  Each is marked in the unit next to its declaration.

## Not claimed

- **Nothing runs.**  Both arm readings are *compile* readings (a 274,640- and a 291,336-byte
  rlib that no one linked): no POP3 conversation, no socket, no name lookup, no password read,
  no message retrieved, no Kerberos exchange, no Windows call.
- The frozen object is empty *by construction* (the file gate), so it says nothing about the
  body; the two `nm`/`size` lines say only that this file defines nothing in this configuration,
  in the reference and in the port alike.
- `extract-abi.py --check`'s `OK` is **vacuous for the port's text** (Evidence ④); the coverage
  evidence is the C line-range table plus `check-dead-file-coverage.py`, and the arm readings.
- The `windowsnt` arm is text-only and blocked by the mirror-set duplicates (Owed 7); the
  `hesiod`/`mailhost` arms compile but their inputs do not exist here (a placeholder host, no
  Hesiod library).
- The 2 warnings of each arm reading are in the `unistd.in.rs` mirror, not in this unit's text,
  and are not claimed to be harmless beyond being warnings.
- This ledger does not claim the earlier P-08 note was *wrong* about the measurement (the empty
  object, the empty checklist); it withdraws its *conclusion* (that the text need not be
  written), as `docs/ports/msdos.md`, `docs/ports/w32proc.md` and `docs/ports/androidmenu.md`
  record for their own files.
