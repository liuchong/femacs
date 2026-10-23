# emacs/msdos/langinfo.h (20 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] nl_langinfo  (line 20, body ea42a7f48c12)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of msdos/langinfo.h, complete for this target: the whole file is C 16's
`# define nl_langinfo(ignore)  "cp437"`, carried as a `macro_rules!` whose body is the C string
literal (status 3.91's string-macro rule).  The frozen darwin-arm64 configuration never puts
`msdos/` on an include path and compiles no `msdos/` translation unit
(docs/baseline/frozen-build-sources.txt's batch-F section: "nt/ msdos/: the Windows and MS-DOS
ports; this target is darwin"), so the mirror is text: `xtask layout-check` covers
`src/include` and `lib/include` only, and this header's own name is all the port needs.

**Ledger name**: `include-msdos-langinfo.md`, not the stem's `include-langinfo.md`, because
`nt/inc/langinfo.h` has the same stem and is being mirrored in the same round: status 3.103's
third candidate carries the reference path's directories, and the readers accept a ledger only
when its first line names the file.

Evidence: `python3 tools/extract-abi.py --check emacs/msdos/langinfo.h
msdos/include/langinfo.rs` -> `OK` (1 ledger item).
