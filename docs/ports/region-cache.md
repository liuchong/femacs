# emacs/src/region-cache.c (781 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12
- [x] new_region_cache  (line 131)
- [x] free_region_cache  (line 157)
- [x] find_cache_boundary  (line 174, static)
- [x] move_cache_gap  (line 207, static)
- [x] insert_cache_boundary  (line 281, static)
- [x] delete_cache_boundaries  (line 311, static)
- [x] set_cache_region  (line 359, static)
- [x] invalidate_region_cache  (line 458)
- [x] revalidate_region_cache  (line 537, static)
- [x] know_region_cache  (line 665)
- [x] region_cache_forward  (line 681)
- [x] region_cache_backward  (line 719)

## Macros defined — 5
- [x] BOUNDARY_POS  (line 98, body 3120265778fb)
- [x] BOUNDARY_VALUE  (line 104, body aaa8a8fdab5a)
- [x] NEW_CACHE_GAP  (line 117, body ae823cded21b)
- [x] PRESERVE_THRESHOLD  (line 123, body e97869f00dae)
- [x] SET_BOUNDARY_VALUE  (line 110, body 6b12c7b141da)

## Types and their fields — 2
- [x] boundary: ptrdiff_t pos, int value
- [x] region_cache: struct boundary* boundaries, ptrdiff_t gap_start, ptrdiff_t gap_len, ptrdiff_t cache_len, ptrdiff_t beg_unchanged, ptrdiff_t end_unchanged, ptrdiff_t buffer_beg, ptrdiff_t buffer_end

## File-scope objects — 0

## Decisions

`emacs/src/region-cache.c` @ `fd7afced403`; port `femacs/src/region-cache.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/region-cache.c femacs/src/region-cache.rs`
-> `OK` (6 exported functions, 5 macros, 2 structs, 0 subrs, 0 file-scope objects).

- **Includes (C 22-27).**  The port mirrors `config.h`, `<stdio.h>`, `lisp.h`, `character.h`,
  `itree.h` and `buffer.h`; C 27's `#include "region-cache.h"` is `include/region-cache.rs`
  since status 3.74.  The header's six prototypes are this unit's own definitions (a C
  prototype/definition pair has no Rust counterpart, PORTING.md 0.1-12) and the header's
  `struct region_cache` is incomplete (C 69-95, defined here), so the mirror carries the six
  declarations in the P-55 shape (`*mut c_void` for the cache - what `buffer.h`'s mirror already
  spells for the three cache fields - and `*mut buffer` for the buffer, each with its `/// C:`
  prototype).  This unit includes the mirror exactly as C includes the header: its definitions
  shadow the mirror's declarations (0.1-10).  The three consumers - `bidi.rs` (C 247),
  `insdel.rs` (C 30) and `search.rs` (C 32) - moved their own copies over in the same change
  (`docs/ports/include-region-cache.md`).
- **C 125's forward declaration** (`static void revalidate_region_cache (...)`) is dropped:
  Rust items are order-free, so the declaration has no counterpart and the definition stays at
  C's position (PORTING.md 0.1-12).
- **Macros.**  `BOUNDARY_POS`/`BOUNDARY_VALUE`/`SET_BOUNDARY_VALUE` are `macro_rules!`; C's
  ternary is Rust's `if`/`else` expression, and `boundaries[i]` is the port's raw-pointer
  form `(*(*c).boundaries.add (i as usize))` (`as usize` is the index conversion, the
  `mini-gmp.rs`/`lisp.rs` shape).  `SET_BOUNDARY_VALUE` keeps C's assignment **inside** the
  branches, so it is a statement macro (used as `{ SET_BOUNDARY_VALUE! (…); }`); every arm is
  a place expression, so C's l-value semantics survive (the `buffer.h` `GPT` lesson).
  `NEW_CACHE_GAP` and `PRESERVE_THRESHOLD` are `const ptrdiff_t`s carrying C's parentheses.
- **Types.**  `struct boundary` and `struct region_cache` are `#[repr(C)]`, with C's field
  order and one field per line (the port's struct parser, and its mirrors, read one field per
  declaration); `int` is `c_int`, `ptrdiff_t` the `lisp.rs` alias.  The structs are private to
  this unit in C, and private here — `buffer.rs` already types the three buffer fields that
  point at a `region_cache` as `*mut c_void`.
- **Empty statements.**  C's two empty `if` bodies (`if (len == 0) ;` at C 326-327 and
  `if (end == c->buffer_end) ;` at C 607-609) become `{ }` blocks; Rust has no empty
  statement that reads as C's `;` without inventing one.
- **Chained assignment (C 653-655).**  `c->beg_unchanged = c->end_unchanged = Z - B` has no
  Rust spelling (a chained `=` gives `()` for the inner one, E0308); the port names the value
  once and assigns it to both fields, which is what C does (no field is read by the RHS).
- **`for` loops -> `while`.**  `move_cache_gap`'s decrementing loop and the two scans in
  `region_cache_forward`/`region_cache_backward` (and `pp_cache`'s incrementing one) become
  `while` with the step at the end of the body; C's `break` keeps its place, so the iteration
  and the short-circuit order are unchanged.  `mut min_size` in `move_cache_gap`: C assigns to
  its parameter, which Rust needs `mut` for (`doprnt.rs`'s `mut ap` precedent).
- **`#ifdef ENABLE_CHECKING` (C 754-781).**  The frozen configuration leaves ENABLE_CHECKING
  undefined (`docs/baseline/config/darwin-arm64.macros.tsv`), so C compiles nothing from the
  arm and the inventory does not demand it; the port keeps the text under
  `#[cfg (enable_checking)]` — the same key `lisp.rs`'s `eassert!` uses — so `xtask`'s
  checking pass (`rustc --cfg enable_checking`) compiles it, and the plain pass does not.
  The reference's `EXTERNALLY_VISIBLE` declaration is **not** carried as `#[unsafe(no_mangle)]`:
  a definition in a configuration-dead arm emits no symbol (docs/status.md 3.26), and the
  C-name gate would read the attribute as an invented exported symbol.  `fprintf`/`stderr` are
  declared next to the arm (the `getopt.rs`/`mini-gmp.rs` shape), and C's `%"pD"d` string
  concatenation (`lisp.h` 150: `pD` is `"t"`) is written as the concatenated `%td`.
- **Line count.**  781 C lines vs 798 Rust (2.2%, inside the rule's 5%): the cost is the two
  `#[repr(C)]`s and the six `#[unsafe(no_mangle)]` lines C does not carry, the one-field-per-line
  struct split (3), the `let` splits C writes as one declaration (2), the `while` conversions
  (8) and the checking arm's `extern` block (6); `#include "region-cache.h"`'s missing mirror
  and C 125's forward declaration are the two lines the port does not spend.  No
  `line-drift-tolerance.tsv` row is needed.
- **Evidence.**  `xtask build-tu src/region-cache.c` -> `built src/region-cache.rs`
  (plain pass); the same materialized unit compiled with `rustc --cfg enable_checking` (the
  pass `gate-fast` runs) exits 0, which is what puts the `pp_cache` arm through rustc.
  `nm` on the plain object shows `_new_region_cache`, `_free_region_cache`,
  `_invalidate_region_cache`, `_know_region_cache`, `_region_cache_forward`,
  `_region_cache_backward`; the checking object additionally carries `pp_cache` under its
  mangled name and **no** `_pp_cache`, the P-51/3.26 combination the configuration asks for.
  Call relationships: the six public functions call the six private helpers exactly as C
  does; no helper is inlined away.  The checking-pass compile was run by hand during this
  round because a parallel edit to `lisp.rs` (a duplicated `string_overflow` declaration in
  `other_units`, since removed) made every unit fail with E0428; the official `build-tu` and
  `--all` above were re-run after that edit landed and are clean for this file.
