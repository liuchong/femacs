# emacs/src/gmalloc.c (1908 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 87 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 6
- [x] <anon@116>: int type, <aggregate> info
- [x] <anon@126>: <aggregate> frag, ptrdiff_t size
- [x] <anon@128>: size_t nfree, size_t first
- [x] <anon@140>: size_t size, size_t next, size_t prev
- [x] alignlist: struct alignlist* next, void* aligned, void* exact
- [x] list: struct list* next, struct list* prev

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/gmalloc.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
