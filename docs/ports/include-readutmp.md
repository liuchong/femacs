# emacs/lib/readutmp.h (352 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 32
- [x] BOOT_TIME  (line 279, body da4b9237bacc)
- [x] END_UTMP_ENT  (line 213, body a46bf5295171)
- [x] GET_UTMP_ENT  (line 212, body f41235c0e9c7)
- [x] HAVE_STRUCT_XTMP_UT_HOST  (line 231, body 6ba89e7749d1)
- [x] HAVE_STRUCT_XTMP_UT_ID  (line 221, body 3d238c27c29f)
- [x] HAVE_STRUCT_XTMP_UT_PID  (line 226, body f752976ae12d)
- [x] IS_USER_PROCESS  (line 302, body 10db438a435f)
- [x] LOGIN_PROCESS  (line 281, body c1dfd96eea8c)
- [x] READ_UTMP_SUPPORTED  (line 307, body 356a192b7913)
- [x] SET_UTMP_ENT  (line 211, body 4d041d113afc)
- [x] USER_PROCESS  (line 280, body b6589fc6ab0d)
- [x] UTMP_FILE  (line 245, body ae7c7c503c8d)
- [x] UTMP_NAME_FUNCTION  (line 215, body b2c5d88c2736)
- [x] UTMP_STRUCT_NAME  (line 210, body a0b8338d17c9)
- [x] UT_EXIT_E_EXIT  (line 89, body a150c734b2f5)
- [x] UT_EXIT_E_TERMINATION  (line 88, body 602e8b896e7b)
- [x] UT_PID  (line 85, body 5c4b5c615e47)
- [x] UT_TIME_MEMBER  (line 84, body eeedf454a58a)
- [x] UT_TYPE_BOOT_TIME  (line 288, body b6589fc6ab0d)
- [x] UT_TYPE_EQ  (line 86, body ddb73971f184)
- [x] UT_TYPE_LOGIN_PROCESS  (line 298, body b6589fc6ab0d)
- [x] UT_TYPE_NOT_DEFINED  (line 87, body b6589fc6ab0d)
- [x] UT_TYPE_USER_PROCESS  (line 293, body b6589fc6ab0d)
- [x] UT_USER  (line 83, body 7c4bc79987e8)
- [x] WTMPDB_FILE  (line 264, body 837884b93afd)
- [x] WTMP_FILE  (line 258, body 7907e387f14b)
- [x] _GL_UT_HOST_SIZE  (line 143, body a7a0c314ce7f)
- [x] _GL_UT_ID_SIZE  (line 141, body 5d79af969b5f)
- [x] _GL_UT_LINE_SIZE  (line 142, body e566989a0e2d)
- [x] _GL_UT_USER_SIZE  (line 140, body da73f2c9b465)
- [x] __READUTMP_H__  (line 21, body da39a3ee5e6b)
- [x] utmp_data  (line 53, body 3af5c4cf5e67)

## Types and their fields — 2
- [x] gl_utmp: char* ut_user, char* ut_id, char* ut_line, char* ut_host, struct timespec ut_ts, pid_t ut_pid, pid_t ut_session, short ut_type
- [x] utmpx32: short int ut_type, pid_t ut_pid, char ut_line, char ut_id, char ut_user, char ut_host, struct __exit_status ut_exit, int ut_session, unsigned int tv_sec, int tv_usec, <aggregate> ut_tv, int ut_addr_v6, char ut_reserved

## File-scope objects — 0

## Decisions

- **The file is not in the verified list, so every arm was decided by measurement.**  `python3
  tools/check-cond-eval.py lib/readutmp.h` -> `lib/readutmp.h: 25 conditions, 1 unknown,
  6 disagreements [NOT VERIFIED]`:

  | line | condition | clang | evaluator |
  | --- | --- | --- | --- |
  | 237 | `#if !defined UTMP_FILE && defined _PATH_UTMP` | True | False |
  | 240 | `#ifdef UTMPX_FILE` | True | False |
  | 244 | `#ifndef UTMP_FILE` | False | True |
  | 250 | `#if !defined WTMP_FILE && defined _PATH_WTMP` | True | False |
  | 257 | `#ifndef WTMP_FILE` | False | True |
  | 278 | `#if !(HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_TYPE : HAVE_STRUCT_UTMP_UT_TYPE)` | False | unknown (kept live) |

  Every disagreement is a *system-header* macro the configuration's table cannot see
  (`_PATH_UTMP`, `UTMPX_FILE`, `_PATH_WTMP`, `WTMPX_FILE` and the two "already defined?"
  guards), plus the ternary at C 278; none of them is a disagreement about a configuration
  macro.  `tools/extract-abi.py` therefore reads the file blind (P-66), which is why the
  mirror carries both arms of every `#if` below and why the live arm per name had to be
  measured.

- **The criterion: the frozen clang's own preprocessor, with `config.h` alone as the control.**
  The command (Evidence) prints *nothing* for these names with `config.h` alone and the
  following after `readutmp.h`: `UT_USER (UT) ((UT)->ut_user)`, `UT_TIME_MEMBER`, `UT_PID`,
  `UT_TYPE_EQ`, `UT_TYPE_NOT_DEFINED 0`, `UT_EXIT_E_TERMINATION`, `UT_EXIT_E_EXIT`,
  `UTMP_STRUCT_NAME utmpx`, `SET_UTMP_ENT setutxent`, `GET_UTMP_ENT getutxent`,
  `END_UTMP_ENT endutxent`, `UTMP_NAME_FUNCTION utmpxname`, `UTMP_FILE UTMPX_FILE` with
  `UTMPX_FILE _PATH_UTMPX` and `_PATH_UTMPX "/var/run/utmpx"`, `WTMP_FILE _PATH_WTMP` with
  `_PATH_WTMP "/var/log/wtmp"`, `WTMPDB_FILE "/var/log/wtmp.db"`,
  `UT_TYPE_BOOT_TIME (UT) ((UT)->ut_type == BOOT_TIME)`,
  `UT_TYPE_USER_PROCESS (UT) ((UT)->ut_type == USER_PROCESS)`,
  `UT_TYPE_LOGIN_PROCESS (UT) ((UT)->ut_type == LOGIN_PROCESS)`, `IS_USER_PROCESS (UT)
  ((UT)->ut_user[0] && UT_TYPE_USER_PROCESS (UT))` and `READ_UTMP_SUPPORTED 1`.  The three
  `ut_type` values (`BOOT_TIME` 2, `USER_PROCESS` 7, `LOGIN_PROCESS` 6) and the four
  `_PATH_*` strings come from the system `<utmpx.h>`/`<utmp.h>`, which is why the header's own
  C 270-273 and C 278-282 blocks are dead here.

- **The dead arms, each under the feature that names the condition selecting it** (P-51; the
  status 3.9 note applies - none of these names is in `xtask dead-arms`' FLIPS list, they are
  compiler/platform predicates):

  | feature | C lines it gates |
  | --- | --- |
  | `utmp_h_ut_exit_no_utmpx_ut_exit` | C 40-41's `# undef HAVE_UTMPX_H` (AIX 4.3.3 workaround; `HAVE_STRUCT_UTMP_UT_EXIT` and `HAVE_STRUCT_UTMPX_UT_EXIT` are both `undef`) |
  | `glibc_time_bits_64` | C 136-143's `_GL_UT_*_SIZE` and C 146-168's `struct utmpx32` (`__GLIBC__` is 0 on the frozen target) |
  | `utmp_h` | C 210's `UTMP_STRUCT_NAME utmp`, C 211-213's `setutent`/`getutent`/`endutent` and C 215's `utmpname` - the whole `#elif HAVE_UTMP_H` branch, dead because `HAVE_UTMPX_H` is 1 |
  | `utxdb_active` | C 180's `UTMP_NAME_FUNCTION(x) setutxdb (UTXDB_ACTIVE, x)` (FreeBSD) |
  | `android` | C 181-184's `UTMP_NAME_FUNCTION(x) ((void) (x))` and C 270-273's Android `BOOT_TIME`/`LOGIN_PROCESS` |
  | `no_utmp_x_ut_type` | C 278-282's "no `ut_type` field" block |
  | `no_boot_time_defined`, `no_user_process_defined`, `no_login_process_defined` | C 287-289/292-294/297-299's `0` arms |
  | `no_utmp_file`, `no_utmpx_file` | C 245's "/etc/utmp" fallback and C 237-238's `_PATH_UTMP` value |
  | `no_wtmp_file`, `no_path_wtmp` | C 258's "/etc/wtmp" fallback and the `_PATH_WTMP` value it replaces |
  | `thread_safe_utmp_data` | C 50-54's AIX `utmp_data` rename (`_THREAD_SAFE`/`UTMP_DATA_INIT`) |

  The four duplicate-name groups use mutually exclusive arms (the live one marked
  `not (...)`) so that exactly one definition is ever compiled and no combination can produce
  an `E0428`.

- **Recorded here, not carried.**  C 259's `WTMPX_FILE` arm (C 253-255) is dead and its value
  is *another* system's path, so its bytes cannot be measured on this target - the mirror
  carries the `_PATH_WTMP` value and the ledger records the arm (the ftoastr.h dead-arm
  policy).  C 26-28's `#error`, C 30-34's includes (kept as `include!`s), C 44-46's
  `<utmp.h>`/C 49-56's `<utmpx.h>` (the port gets their values from `include/utmpx.rs`), C
  59-61/348-350's C++ guards, C 65's `#undef UT_USER`, C 91-92's `typedef`, C 94-103's
  `UT_*_SIZE` enums (carried as constants), C 310-317's options enum (carried), and C 321-323/
  344-345's `_GL_ATTRIBUTE_*` spellings (attributes, PORTING.md 0.3/8) are recorded in the
  mirror's comments or dropped structurally.

- **`struct gl_utmp`'s layout is measured and pinned.**  `clang -std=gnu23 ... probe.c` (the
  probe is in Evidence) prints `sizeof=72 align=8` and `ut_user=0 ut_id=8 ut_line=16 ut_host=24
  ut_ts=32 ut_pid=48 ut_session=52 ut_type=56 ut_exit=60`, with `ut_exit` 8 bytes,
  `struct timespec` 16, `pid_t` 4 and `short` 2.  The mirror repeats those numbers as `const _`
  assertions, so a wrong field type or order fails the compile (the probe compile in Evidence
  is what runs them).  C 79's anonymous `struct { int e_termination; int e_exit; }` member has
  no Rust spelling, so the mirror names it `gl_utmp_ut_exit` - the one invented shape in the
  file.

- **The mirror takes its scalars and the three port types from the including unit** (PORTING.md
  0.1-9: a mirror must not carry a `use`), exactly as C 30-34's own includes do: it
  `include!`s `idx.rs`, `sys_types.in.rs` (`pid_t`) and `time.in.rs` (`struct timespec`), and
  `c_char`/`c_int`/`c_long`/`c_short`/`c_void` come from the unit's `use` line.  The real
  consumer already satisfies that: `lib/boot-time.rs` 39 has
  `use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_void};` and its include list
  already carries `sys_types.in.rs`/`time.in.rs`, so wiring this mirror in is one `include!`
  line.  Consequence, measured: `build-tu lib/readutmp.h` **fails** (the crate root is the
  mirror and nothing supplies the scalars - `error[E0425]`), which is the state
  `include-lib-malloc-scratch_buffer.md` records for every mirror of this shape; the compile
  proof is the probe root in Evidence (`use std::ffi::{...}; include!("<abs>/readutmp.rs")`,
  exit 0, no warnings - and it is what evaluates the layout assertions).

- **Wiring, deliberately not done here.**  `lib/boot-time.rs` 73-74 and 124 declare
  `UTMP_STRUCT_NAME`/`UTMP_FILE`/the four entry macros at the use site; they become this
  mirror's consumers when that unit's owner adds the `include!` (the parent said the wiring is
  theirs).  The three `readutmp_decls` declarations (`extract_trimmed_name`, `read_utmp`) sit
  in a whole-import module so that a unit keeping its own copy shadows the glob instead of
  clashing with it (PORTING.md 0.1-10).

## Evidence

```
$ python3 tools/extract-abi.py emacs/lib/readutmp.h
# emacs/lib/readutmp.h (352 lines)
## Subrs (DEFUN) - 0
## Functions defined - 0
## Macros defined - 32   (32/32 ticked above)
## Types and their fields - 2   (gl_utmp 8 fields, utmpx32 11 fields - both ticked)
## File-scope objects - 0

$ python3 tools/extract-abi.py --check emacs/lib/readutmp.h femacs/lib/include/readutmp.rs
  (12 C macros have several definitions (configuration branches), body not compared: BOOT_TIME, END_UTMP_ENT, GET_UTMP_ENT, LOGIN_PROCESS, SET_UTMP_ENT, UTMP_FILE ...)
OK emacs/lib/readutmp.h -> femacs/lib/include/readutmp.rs
# no `- ` problem line (`| grep -c '^- '` -> 0)

$ python3 tools/check-cond-eval.py lib/readutmp.h
DISAGREE lib/readutmp.h:237 `if !defined UTMP_FILE && defined _PATH_UTMP` — clang True, evaluator False
DISAGREE lib/readutmp.h:240 `ifdef UTMPX_FILE` — clang True, evaluator False
DISAGREE lib/readutmp.h:244 `ifndef UTMP_FILE` — clang False, evaluator True
DISAGREE lib/readutmp.h:250 `if !defined WTMP_FILE && defined _PATH_WTMP` — clang True, evaluator False
DISAGREE lib/readutmp.h:257 `ifndef WTMP_FILE` — clang False, evaluator True
DISAGREE(unknown) lib/readutmp.h:278 `if !(HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_TYPE : HAVE_STRUCT_UTMP_UT_TYPE)` — clang False, evaluator keeps the branch live
lib/readutmp.h: 25 conditions, 1 unknown, 6 disagreements [NOT VERIFIED]

$ grep -nE '^(HAVE_UTMPX_H|HAVE_UTMP_H|HAVE_STRUCT_UTMP_UT_EXIT|HAVE_STRUCT_UTMPX_UT_EXIT|HAVE_STRUCT_UTMPX_UT_NAME|HAVE_STRUCT_UTMPX_UT_TYPE|HAVE_UTMPXNAME|HAVE_UTMPNAME)' docs/baseline/config/darwin-arm64.macros.tsv
1592:HAVE_STRUCT_UTMPX_UT_EXIT	undef
1597:HAVE_STRUCT_UTMPX_UT_NAME	undef
1600:HAVE_STRUCT_UTMPX_UT_TYPE	1
1602:HAVE_STRUCT_UTMP_UT_EXIT	undef
1611:HAVE_STRUCT_UTMP_UT_TYPE	undef
1667:HAVE_UTMPNAME	undef
1668:HAVE_UTMPXNAME	1
1669:HAVE_UTMPX_H	1
1670:HAVE_UTMP_H	1

# the criterion: config.h alone defines none of these names, readutmp.h's own live text does
$ PAT='^#define (UTMP|SET_UTMP|GET_UTMP|END_UTMP|UT_USER|UT_TIME|UT_PID|UT_TYPE|UT_EXIT|IS_USER|READ_UTMP|WTMP|BOOT_TIME|USER_PROCESS|LOGIN_PROCESS|HAVE_STRUCT_XTMP|utmp_data|__READUTMP)'
$ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
    -include config.h -x c /dev/null | grep -E "$PAT"                          # no output
$ clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
    -include config.h -include readutmp.h -x c /dev/null | grep -E "$PAT"
#define BOOT_TIME 2
#define END_UTMP_ENT endutxent
#define GET_UTMP_ENT getutxent
#define HAVE_STRUCT_XTMP_UT_HOST (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_HOST : HAVE_STRUCT_UTMP_UT_HOST))
#define HAVE_STRUCT_XTMP_UT_ID (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_ID : HAVE_STRUCT_UTMP_UT_ID))
#define HAVE_STRUCT_XTMP_UT_PID (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_PID : HAVE_STRUCT_UTMP_UT_PID))
#define IS_USER_PROCESS(UT) ((UT)->ut_user[0] && UT_TYPE_USER_PROCESS (UT))
#define LOGIN_PROCESS 6
#define READ_UTMP_SUPPORTED 1
#define SET_UTMP_ENT setutxent
#define USER_PROCESS 7
#define UTMPX_FILE _PATH_UTMPX
#define UTMP_FILE UTMPX_FILE
#define UTMP_NAME_FUNCTION utmpxname
#define UTMP_STRUCT_NAME utmpx
#define UT_EXIT_E_EXIT(UT) ((UT)->ut_exit.e_exit)
#define UT_EXIT_E_TERMINATION(UT) ((UT)->ut_exit.e_termination)
#define UT_PID(UT) ((UT)->ut_pid)
#define UT_TIME_MEMBER(UT) ((UT)->ut_ts.tv_sec)
#define UT_TYPE_BOOT_TIME(UT) ((UT)->ut_type == BOOT_TIME)
#define UT_TYPE_EQ(UT,V) ((UT)->ut_type == (V))
#define UT_TYPE_LOGIN_PROCESS(UT) ((UT)->ut_type == LOGIN_PROCESS)
#define UT_TYPE_NOT_DEFINED 0
#define UT_TYPE_USER_PROCESS(UT) ((UT)->ut_type == USER_PROCESS)
#define UT_USER(UT) ((UT)->ut_user)
#define WTMPDB_FILE "/var/log/wtmp.db"
#define WTMP_FILE _PATH_WTMP
#define __READUTMP_H__ 
# (no utmp_data and no _GL_UT_*_SIZE: both arms are dead, and no `#define USER_PROCESS`/
#  `BOOT_TIME`/`LOGIN_PROCESS` from this header - those survive from the system <utmpx.h>)

# the layout measurement (the probe's own source is below)
$ clang -std=gnu23 -DHAVE_CONFIG_H -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src \
    -I emacs/lib -o /tmp/g188r/probe /tmp/g188r/probe.c && /tmp/g188r/probe
sizeof=72 align=8
ut_user=0 ut_id=8 ut_line=16 ut_host=24 ut_ts=32 ut_pid=48 ut_session=52 ut_type=56 ut_exit=60
sizes: ut_exit=8 ut_ts=16 pid_t=4 short=2
UT_USER_SIZE=-1 UT_ID_SIZE=-1 UT_LINE_SIZE=-1 UT_HOST_SIZE=-1 UT_TYPE_NOT_DEFINED=0
READ_UTMP options: 1 2 4 8
UTMP_FILE=</var/run/utmpx> WTMP_FILE=</var/log/wtmp> WTMPDB_FILE=</var/log/wtmp.db>
BOOT_TIME=2 USER_PROCESS=7 LOGIN_PROCESS=6
# /tmp/g188r/probe.c is `#include <config.h>` + `#include "readutmp.h"` + stdio/stddef and
# prints sizeof/_Alignof/offsetof of the nine members, the four UT_*_SIZE enums, the four
# READ_UTMP_* options and the three path macros.

# the compile proof: the header form cannot work (the mirror takes the includer's scalars) ...
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188 cargo run -q -p xtask -- build-tu lib/readutmp.h
error[E0425]: cannot find type `c_int` in this scope      # sys_types.in.rs 25, from the mirror's own include
error[E0425]: cannot find type `c_int` in this scope
error[E0425]: cannot find type `c_uint` in this scope
error: rustc failed for lib/include/readutmp.rs (exit status: 1)
# ... so the mirror is compiled as a probe root that supplies the `use` line the including unit
# has (`lib/boot-time.rs` 39), which also evaluates the layout assertions above:
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj \
    --crate-name readutmp_probe -C panic=abort -C codegen-units=1 -C opt-level=0 \
    /tmp/g188r/root.rs -o /tmp/g188r/readutmp_probe.o
(exit 0, no warnings; 336-byte object)
# /tmp/g188r/root.rs is the unit allow-list + `use std::ffi::{c_char, c_int, c_long, c_short,
# c_uint, c_void};` + `include! ("/…/femacs/lib/include/readutmp.rs");`
```
