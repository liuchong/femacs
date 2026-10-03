# emacs/lib-src/hexl.c (221 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] output_error  (line 34, static)
- [x] hexchar  (line 41, static)
- [x] main  (line 47)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] static char progname  (line 31, static)

## Decisions

Port of lib-src/hexl.c, complete: three functions (`output_error` static `_Noreturn`, `hexchar`
static, `main`) and one file-scope object (`static char *progname`) - the ledger's four boxes are
ticked.  The frozen configuration builds it (it is in the reference `lib-src/Makefile`'s
`UTILITIES`), so this is a live program, the second real batch-F unit after `make-fingerprint`.

Deviations, all recorded rather than hidden:

1. **`PRIxMAX` (C 163).**  `<inttypes.h>` has no mirror; the frozen target defines the macro as
   `"jx"` (measured with clang: `printf ("%s\n", PRIxMAX)` prints `jx`), so the port writes
   `c"%08jx: "` and `docs/ports/macro-body-notes.tsv` carries the row for the missing token.
2. **`uintmax_t`.**  C gets it from `<inttypes.h>`; the port declares the alias in this unit
   (`pub type uintmax_t = u64;`, measured 8 bytes) exactly as `make-fingerprint.rs` declares
   `ptrdiff_t` - the core units take these names from the `lisp.h` mirror, which a lib-src
   program does not include.
3. **`<unlocked-io.h>` (C 29) has no mirror.**  The renames it performs on stdio calls are
   recorded at the use site by every unit that needs them (docs/ports/dispnew.md 374,
   docs/ports/bidi.md 269), so this program calls `getc`/`putchar`/`fputs` as C's text spells
   them.
4. **`exit` is a use-site declaration** (`<stdlib.h>`'s entry; no mirror carries it yet - the
   `emacs.rs`/`sysdep.rs` precedent).
5. **Two Rust-forced shapes.**  `int i;` declared outside C 189's loop and read at C 224 is
   zero-initialized (PORTING.md 1.5), and C's `printf ("%02x", c + 0u)` needs the unsigned
   promotion written out (`c as c_uint + 0u32`, the 1.1 rule).
6. **The include graph.**  `<config.h>`, `<stdint.h>`, `<stdlib.h>`, `<string.h>` and
   `<binary-io.h>` are the mirrors; `binary-io.rs` brings `<fcntl.h>`'s `O_BINARY`/`O_TEXT` with
   it, which is what `set_binary_mode (fileno (stdout), O_BINARY)` needs.

7. **C 1-23's opening block is not ported** (PORTING.md 0.1-19; the same rule deleted that block
   from 268 files at 3.58).  The block is also where C 8's one-sentence provenance note lives,
   and nothing in `hexl.rs` quotes it: the unit's first line is the `// femacs: port of …`
   declaration, and the fact is recorded here.  (3.100 removed the four-line note that had
   recited it in the unit; the file went from 332 to 328 lines.)

Evidence: `cargo run -q -p xtask -- build-tu lib-src/hexl.c` ->
`built lib-src/hexl.rs -> ../build/femacs/hexl.o`; `python3 tools/extract-abi.py --check
emacs/lib-src/hexl.c femacs/lib-src/hexl.rs` -> `OK` (4/4 boxes, the `prixmax` row in place);
`rustc --cfg enable_checking` on the materialized unit -> exit 0; `check-globals` 273 units /
0 missing; `check-declarations` 0; `check-verbatim` 6 trees / 0 failures.  There is **no
behavioural evidence**: the port cannot run this program yet (it needs the build chain), so the
ledger claims text and object agreement only.
