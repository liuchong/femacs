# emacs/lib/boot-time-aux.h (426 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 11
- [x] get_linux_uptime  (line 25, static)
- [x] get_linux_boot_time_fallback  (line 85, static)
- [x] get_linux_boot_time_final_fallback  (line 132, static)
- [x] get_android_boot_time  (line 174, static)
- [x] get_openbsd_boot_time  (line 205, static)
- [x] get_bsd_boot_time_final_fallback  (line 241, static)
- [x] get_haiku_boot_time  (line 261, static)
- [x] get_haiku_boot_time_final_fallback  (line 291, static)
- [x] get_windows_boot_time  (line 306, static)
- [x] initialize  (line 364, static)
- [x] get_windows_boot_time_fallback  (line 390, static)

## Macros defined — 3
- [x] GetProcAddress  (line 354, body f09ee83f58e4)
- [x] GetTickCount64Func  (line 377, body f3d5b9549b09)
- [x] LoadLibrary  (line 351, body 2ff89c407367)

## Types and their fields — 0

## File-scope objects — 2
- [x] static GetTickCount64FuncType GetTickCount64Func  (line 360, static)
- [x] static BOOL initialized  (line 361, static)
## Decisions

Mirror of lib/boot-time-aux.h, complete for this target: the eleven `static` helpers of the
header's six platform arms, the three `#define`s (`LoadLibrary`, `GetProcAddress`,
`GetTickCount64Func`) and the two Windows statics.  C's opening block (C 1-15) and C 16's
author line are the file's own declaration and are not copied (PORTING.md 0.1-19);
`_GL_ATTRIBUTE_MAYBE_UNUSED` on C 23 is gnulib's attribute macro and is recorded, not spelled.

**The header is the reason this mirror exists.** Only its sysctl arm is live on the frozen target
(C 230-256, the gate `lib/boot-time.c` 230-232 repeats), and until 3.104 that one function was
transcribed *inside* the unit (`lib/boot-time.rs`) while the header's other arms were recorded in
`docs/ports/boot-time.md`.  The port's own `xtask dead-arms` then failed: the unit's carried
`haiku` arm calls `get_haiku_boot_time` (C 239-244), which the header defines, and nothing
defined it there.  The mirror is the honest fix, and it also brings the header's own 426 lines
into the inventory as a file of their own instead of "a ledger-to-be".

**Features.** `linux_or_android` (C 19), `linux_no_android` (C 82), `android` (C 171),
`openbsd` (C 202), `haiku` (C 258), `os_h` (C 284), `cygwin_win32` (C 302) with `no_cygwin`
(C 313) - the names `lib/boot-time.rs` already uses - plus four this header adds:
`no_glibc_boottime` (C 30's glibc predicate), `no_glibc_realtime` (C 138's), `have_decl_sysinfo`
(C 66), `win32_vista` (C 344).  Only `haiku` is flipped by `xtask dead-arms`, so it is the one
arm whose Rust is compiled; the others carry C's text with the platform names they name
(`sysinfo`/`struct sysinfo`, `system_info`/`get_system_info`, `ULONGLONG`/`BOOL`/`HMODULE`,
`clock_gettime`/`CLOCK_BOOTTIME`/`timespec_get`/`TIME_UTC`, `gettimeofday`, `LoadLibrary`,
`TRUE`/`FALSE`) declared nowhere, exactly as `lib/boot-time.rs` already names `fopen` in its dead
arm (PORTING.md P-51; no mirror exists for `<sys/sysinfo.h>`, `<OS.h>` or `<windows.h>`).

**Two Rust-forced shapes**, both recorded here because no gate can see them: `trunc`-style
`#define` renames of the two Windows calls are written as C's text spells them (`LoadLibrary`,
`GetProcAddress`) with the rename recorded at the top of the arm; and
`get_windows_boot_time_fallback` selects the source of `GetTickCount64Func` by cfg instead of C's
`#define`-supplied name (C 377-384), with `is_some`/`unwrap` standing for C's own
`GetTickCount64Func != NULL` test and the call it guards.

Evidence: `python3 tools/extract-abi.py --check emacs/lib/boot-time-aux.h
femacs/lib/include/boot-time-aux.rs` -> `OK`; `cargo run -q -p xtask -- build-tu lib/boot-time.c`
-> `built`; `cargo run -q -p xtask -- layout-check` -> `layout-check passed`;
`cargo run -q -p xtask -- dead-arms` -> `322 translation unit(s) compiled with the non-frozen
arms` (it was the failing unit before this mirror existed).

