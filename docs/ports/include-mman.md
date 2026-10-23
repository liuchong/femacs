# emacs/nt/inc/sys/mman.h (31 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 9
- [x] MAP_ANON  (line 23, body 49916ab8e20b)
- [x] MAP_ANONYMOUS  (line 22, body a6d133efc0d7)
- [x] MAP_FAILED  (line 25, body ea78d814400b)
- [x] MAP_FILE  (line 17, body b6589fc6ab0d)
- [x] MAP_FIXED  (line 21, body 9042d4228c01)
- [x] MAP_PRIVATE  (line 19, body da4b9237bacc)
- [x] MAP_SHARED  (line 18, body 356a192b7913)
- [x] MAP_TYPE  (line 20, body 16d4376c32dd)
- [x] _SYS_MMAN_H_  (line 7, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/sys/mman.h (mman-win32), complete for this target: the include guard, the nine `MAP_*` constants including the `MAP_ANON` alias C's own comment says `src/buffer.c` needs, and `MAP_FAILED`, whose `((void *) -1)` C spells as a cast.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it - `spikes/f_inc_probe.rs`
compiles it instead, and that probe found three real defects that every text gate had passed
(docs/status.md 3.102).  Its values are C's own and need
no clang measurement: they are literals and typedefs from the Windows SDK's own spellings.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/mman.h
nt/inc/sys/include/mman.rs` -> `OK`.
  `../spikes/f_inc_probe.rs` compiles this mirror: it is the only compiler it has.
