# emacs/lib/boot-time.c (306 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] get_boot_time_uncached  (line 84, static)
- [x] get_boot_time  (line 275)
- [x] get_boot_time  (line 301, the dead `#else` arm - one definition answers both rows)

## Macros defined — 2
- [x] UT_USER  (line 72, body 7c4bc79987e8 - the frozen arm of C 68-73; two C definitions, body not compared)
- [x] WIN32_LEAN_AND_MEAN  (line 48, body da39a3ee5e6b - the empty body of C 48, kept as a marker macro)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/boot-time.c` @ `fd7afced403`; port `femacs/lib/boot-time.rs` (378 lines vs C's
306, 23.5% - the tolerance is `boot-time 0.25` on the `total` basis in
`docs/ports/line-drift-tolerance.tsv`); the gates are `build-tu lib/boot-time.c` -> `built`,
`python3 tools/extract-abi.py --check emacs/lib/boot-time.c femacs/lib/boot-time.rs` -> `OK`,
and `python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt`, where
`build/femacs/boot-time.o` carries exactly the reference object's interface
(`nm -g build/emacs-ref/lib/boot-time.o`: `T _get_boot_time`, `U _endutxent`, `U _getutxent`,
`U _setutxent`, `U _sysctl`, `U _utmpxname`).  One new mirror, `femacs/lib/include/utmpx.rs`,
was created (a system header, so it has no reference file to pair with - the
`sys_sysctl.rs`/`sys_time.rs` shape); `readutmp.h`, `unlocked-io.h` and `boot-time-aux.h`
were *not* mirrored, see below.

### Which text is live (P-08)

`docs/baseline/config/darwin-arm64.macros.tsv`: HAVE_UTMPX_H 1, HAVE_UTMP_H 1,
HAVE_STRUCT_UTMPX_UT_NAME undef, HAVE_STRUCT_UTMPX_UT_TYPE 1, HAVE_UTMPXNAME 1,
HAVE_DECL_ENDUTENT 0, HAVE_SYS_SYSCTL_H 1, HAVE_SYS_PARAM_H 1, HAVE_SYSCTL 1, CTL_KERN 1,
KERN_BOOTTIME 21, HAVE_OS_H undef; the compiler defines none of `__linux__`, `__ANDROID__`,
`__minix`, `__OpenBSD__`, `__HAIKU__`, `__CYGWIN__`, `_WIN32` (measured with clang -dM -E on
the frozen configuration, the probe in `femacs/lib/include/utmpx.rs`).  The file is **not**
in `docs/baseline/config/cond-eval-verified.txt`, so `extract-abi.py` compares the port with
*every* branch live (P-66's conservative mode) and the dead arms are therefore kept as text
(P-51) rather than dropped - the status 3.9 policy for `lib/*.c`.

- C 81's `#if defined __linux__ || HAVE_UTMPX_H || HAVE_UTMP_H || defined __CYGWIN__ ||
  defined _WIN32` is true, so C 83-296 is transcribed.  C 298-306's `#else` arm
  (`get_boot_time` returning `-1`) is dead and is **recorded**, not carried: a second
  `#[unsafe(no_mangle)]` definition of the same name is the only Rust spelling it has.  Both
  `get_boot_time` rows above are answered by the C 275 definition, whose body the token check
  compares.
- C 88's `#if (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_TYPE : HAVE_STRUCT_UTMP_UT_TYPE)` is true;
  C 214-228's `#else` (Adélie Linux / old FreeBSD / OpenBSD / native Windows) is dead.
- C 92's `#if defined UTMP_NAME_FUNCTION` half of the scan is live (macOS defines
  HAVE_UTMPXNAME, so readutmp.h 170-178 binds `utmpxname`/`setutxent`/`getutxent`/`endutxent`
  and `struct utmpx`); C 178's `#else` (HP-UX, Haiku: `fopen`/`fread`/`fclose` over
  `UTMP_FILE`) is dead.
- C 115-119's `#if (HAVE_UTMPX_H ? 1 : HAVE_STRUCT_UTMP_UT_TV)` takes the `ut_tv` arm;
  C 117-119's `ut_time` arm is dead.  The same condition inside the `fopen` arm (C 191-196)
  is the same branch.
- C 230-232's `#if HAVE_SYS_SYSCTL_H && HAVE_SYSCTL && defined CTL_KERN && defined
  KERN_BOOTTIME && !defined __minix` is live, and `boot-time-aux.h`'s matching arm -
  `get_bsd_boot_time_final_fallback` - is the only arm of that header this configuration
  compiles.  The reference object proves it: it imports `_sysctl` and no other helper.
- Dead and not live: C 31-34 (`<sys/sysinfo.h>`, `<time.h>`), C 43-45 (`<OS.h>`), C 47-51
  (`<windows.h>`, `<sys/time.h>`), C 70 (`((UT)->ut_name)`), C 75-79 (`void endutent (void);`),
  C 103/124/129/138/158/166/207/216 (the Linux/Android/Minix arms), C 223 (OpenBSD),
  C 239 (Haiku), C 246 (HAVE_OS_H), C 253/258 (Cygwin/Windows).

### Dead arms kept as text (P-51)

Every dead arm of this file is carried under a feature that names its non-frozen side, so
`--check` sees C's tokens and the reader sees C's text; the status 3.9 note applies here too -
**none of these features is in `xtask dead-arms`**, which can only flip the keys
`femacs/src/include/config.rs` carries and these are compiler/platform predicates:

| feature | C lines it gates |
| --- | --- |
| `utmp_ut_name` | C 68-73's `# define UT_USER(UT) ((UT)->ut_name)` arm |
| `win32` | C 47-51's `WIN32_LEAN_AND_MEAN` marker (the arm's own `_WIN32 && !__CYGWIN__` guard; its two includes are recorded below) |
| `linux_no_android` | C 103-106, 124-128, 138-156, 207-212, 216-221 |
| `android` | C 158-164 |
| `minix` | C 129-133, 166-176 |
| `no_utmp_name_function` | C 178-205's `#else` (`fopen`/`fread`/`fclose`) |
| `no_utmp_x_ut_type` | C 214-228's `#else` |
| `openbsd` | C 223-226 |
| `haiku` | C 239-244 |
| `os_h` | C 246-251 |
| `cygwin_win32` (with `no_cygwin` for C 258) | C 253-263 |
| `utmp_h` | C 117-119's `ut_time` arm (and the same arm in C 191-196) |

C 31-34's `linux_or_android` includes and C 47-51's two Windows includes are the exception:
an `#include` cannot be kept as text without the mirror it names, and there is no mirror for
`<sys/sysinfo.h>`, `OS.h` or `<windows.h>`, so those *includes* are recorded here (the
binary-io.rs dead-arm note) while the macros and the code they guard are carried.

### Recorded, not carried

- **C 75-79's `#if !HAVE_UTMPX_H && HAVE_UTMP_H && defined UTMP_NAME_FUNCTION` arm**, whose
  `void endutent (void);` is the Android declaration; HAVE_UTMPX_H is 1, so it is dead.
- **C 65's `#undef UT_USER`**: the port has no `#undef`; the mirror's `UT_USER` is not included
  here, this unit defines it (the mirror rule 0.1-2 keeps definitions out of mirrors).
- **C 298-306's alternative `get_boot_time`**, as above.
- ~~**`boot-time-aux.h`'s other arms** (Linux/Android/OpenBSD/Haiku/Windows)~~ - **superseded by
  status 3.104**: the header is now the mirror `femacs/lib/include/boot-time-aux.rs`, which the
  unit includes at C 61, so its arms are neither transcribed here nor merely recorded - they live
  in the mirror's ledger, `docs/ports/include-boot-time-aux.md` (and `xtask dead-arms` compiles
  the `haiku` one there, which is what forced the mirror into existence).
- **`volatile`** on C 278-279's two statics: Rust has no qualifier on a `static`, and the
  reads/writes keep C's order (`cached_result < 0`, the two stores, the two loads).  Recorded
  as a deviation; no gate covers it.

### Names declared at the use site (no mirror yet)

`readutmp.h`, `unlocked-io.h` and `boot-time-aux.h` are included by **this file alone**
(measured: `grep -l` over `emacs/lib/*.[ch]` and `emacs/src/*.[ch]`), so their live text is
part of this translation unit exactly as C compiles it, and the names are written where they
are used (the fstatat.rs/lstat.rs in-unit declaration shape) instead of inventing partial
mirrors that `extract-abi.py --all` would then have to track as WIP:

- `UTMP_STRUCT_NAME` is `type UTMP_STRUCT_NAME = utmpx;` (readutmp.h 172's frozen arm),
  `UTMP_FILE` is `b"/var/run/utmpx\0"` (readutmp.h 237-246: `_PATH_UTMPX`), and the four
  entries are `macro_rules!` whose bodies are the frozen functions (`utmpxname`, `setutxent`,
  `getutxent`, `endutxent`) - measured with clang, and the same four symbols the reference
  object imports.
- `CTL_KERN` (1) and `KERN_BOOTTIME` (21) are the frozen `<sys/sysctl.h>` values; the mirror
  `sys_sysctl.rs` carries only the two rows lib/nproc.c names (the filelock.rs EDOM note).
- ~~`boot-time-aux.h`'s live function is defined in this unit~~ - **superseded by status 3.104**:
  C 61 is an `include!` of the header's mirror now, which defines
  `get_bsd_boot_time_final_fallback` (with the sysctl mib and the `size_of_val` the C `sizeof`
  spells) together with `CTL_KERN`/`KERN_BOOTTIME`.
- The dead `fopen` arm names `fopen` without a declaration (it is not in `stdio.rs`), the way
  the binary-io.rs dead arm names `_setmode`; `FILE`/`fread`/`fclose` do come from
  `include/stdio.rs`, and `memeq`/`strlen` from `include/string.rs`.

### The macros the ledger asks for

- **`UT_USER`** has two C definitions (C 70 and C 72), so the extractor records the body as
  "not compared"; the port spells the frozen arm (C 72's `ut_user`) and carries C 70's
  `ut_name` arm under `utmp_ut_name` (P-51).  Called as `UT_USER! (ut)`.
- **`WIN32_LEAN_AND_MEAN`** is C 48's empty `#define`; it is carried as a marker
  `macro_rules! WIN32_LEAN_AND_MEAN { () => {}; }` under `win32`.

### Structural deviations

- `utmp_file` (C 98) keeps C's name, static storage and NUL-terminated bytes as a
  `[u8; 15]` static built from the `UTMP_FILE` constant; the call site takes its `as_ptr ()`
  the way C passes the array (it decays to `char *`).
- `struct timespec`/`struct timeval`/`struct utmpx` come from `time.in.rs`, `sys_time.rs` and
  the new `utmpx.rs`; the utmpx layout is the frozen target's, measured with clang and pinned
  by nine `const _` assertions (size 640, `ut_user` 0, `ut_id` 256, `ut_line` 260, `ut_pid`
  292, `ut_type` 296, `ut_tv` 304, `ut_host` 320, `ut_pad` 576).
- `entry` (C 108) is an uninitialized C local; it is zero-initialized (PORTING.md 1.5), which
  is where the `unused_assignments` warning comes from.
- The C macros are called with Rust's `!` (`SET_UTMP_ENT! ()`, `UT_USER! (ut)`) and the
  `while ((entry = GET_UTMP_ENT ()) != NULL)` condition keeps C's shape as a `while { ...; ... }`
  block expression, C's statement order intact; `for (;;)` is `loop`.
- `struct UTMP_STRUCT_NAME ut;` in the dead `fopen` arm is `std::mem::zeroed ()` (Rust has no
  uninitialized `let`), and its `sizeof ut` is `std::mem::size_of::<UTMP_STRUCT_NAME> ()`
  (the `sizeof` token family).
- C's `else` + one statement (C 270-271, 294-295) is `else { return -1; }`.

### The token check's one note

`docs/ports/macro-body-notes.tsv` records `ctl_kern`, `kern_boottime` and `__minix` for
`get_boot_time_uncached`: C 230-232's `#if` is a directive, but its two *continuation* lines
do not start with `#`, so the tokeniser keeps them and the C body holds those three names;
the port spells the live gate as a cfg and keeps the call it guards.  Every other token of
both function bodies is compared, including the dead arms'.
