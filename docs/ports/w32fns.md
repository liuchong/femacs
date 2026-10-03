# emacs/src/w32fns.c (12488 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 319 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 7
- [x] <anon@10044>: <aggregate> info, DWORD data
- [x] <anon@8284>: OPENFILENAMEW details, PVOID pvReserved, DWORD dwReserved, DWORD FlagsEx
- [x] <anon@8298>: OPENFILENAMEA details, PVOID pvReserved, DWORD dwReserved, DWORD FlagsEx
- [x] MONITOR_INFO: DWORD cbSize, RECT rcMonitor, RECT rcWork, DWORD dwFlags
- [x] MONITOR_INFO_EX: DWORD cbSize, RECT rcMonitor, RECT rcWork, DWORD dwFlags, char szDevice
- [x] info: char major, char minor, short platform
- [x] w32_drop_target: IDropTarget i_drop_target, HWND hwnd, int ref_count

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/w32fns.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
