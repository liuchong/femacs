# emacs/lib/diffseq.h (578 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] diag  (line 194, static)
- [x] compareseq  (line 465, static)

## Macros defined — 5
- [x] EARLY_ABORT  (line 94, body 7cb6efb98ba5)
- [x] NOTE_ORDERED  (line 98, body 7cb6efb98ba5)
- [x] OFFSET_MAX  (line 88, body 305ddbe01fc5)
- [x] SNAKE_LIMIT  (line 149, body 91032ad7bbcb)
- [x] XREF_YREF_EQUAL  (line 473, body 205e4c5805fe)

## Types and their fields — 2
- [x] context: EXTRA_CONTEXT_FIELDS OFFSET* fdiag, OFFSET* bdiag, OFFSET too_expensive
- [x] partition: OFFSET xmid, OFFSET ymid, bool lo_minimal, bool hi_minimal

## File-scope objects — 0

## Decisions

- **A template header, carried as a template.**  gnulib's contract (C 49-84) has the includer
  define `ELEMENT`, `EQUAL`, `OFFSET`, `EXTRA_CONTEXT_FIELDS`, `NOTE_DELETE`, `NOTE_INSERT`
  (optionally `NOTE_ORDERED`, `EARLY_ABORT`, `USE_HEURISTIC`, or the abstract-array pair
  `XVECREF_YVECREF_EQUAL` instead of `ELEMENT`).  The mirror keeps C's names for them: the
  includer supplies Rust items in the unit that includes this file (`type OFFSET = …;`,
  `type ELEMENT = …;`, `macro_rules! EQUAL`, `macro_rules! XVECREF_YVECREF_EQUAL`,
  `macro_rules! NOTE_DELETE`/`NOTE_INSERT`, `type EXTRA_CONTEXT_FIELDS = …;`), and its `NOTE_*`
  macros are invoked from the bodies here exactly as C's are.  The two functions stay plain
  `fn`s (C's `static`: one copy per including translation unit, no symbol - PORTING.md 0.2), so
  the ELEMENT arm and `USE_HEURISTIC`'s `ctxt->heuristic` are the only text with no counterpart
  and are recorded below rather than transcribed (P-08).
- **The frozen arms.**  `tools/check-cond-eval.py lib/diffseq.h` -> `13 conditions, 0 unknown,
  0 disagreements [VERIFIED]`, and the frozen table has `ELEMENT`, `USE_HEURISTIC`,
  `NOTE_ORDERED`, `EARLY_ABORT` and `OFFSET_MAX` all `undef`, so: `struct context` has the three
  fields the ledger lists (no `xvec`/`yvec`), `diag`/`compareseq` have no `xv`/`yv` locals and
  `XREF_YREF_EQUAL` is `XVECREF_YVECREF_EQUAL (ctxt, x, y)`, `diag`'s `heuristic` is `false`,
  `NOTE_ORDERED`/`EARLY_ABORT` are `false` and `OFFSET_MAX` is C 86-90's portable expression.
- **`EXTRA_CONTEXT_FIELDS` has no Rust counterpart** (it splices *fields* into `struct context`):
  the port gives them one member `pub extra: EXTRA_CONTEXT_FIELDS`, whose type comes from the
  includer, and the includer's macros write `(*ctxt).extra.<field>` instead of C's
  `ctxt-><field>`.  Recorded here for the integrator's ruling.
- **`XREF_YREF_EQUAL` takes three arguments in the port.**  C defines it inside each function,
  next to the locals it names (`ctxt`), while a Rust `macro_rules!` cannot name its caller's
  locals (PORTING.md P-33's hygiene note: the macro body's identifiers resolve at the
  *definition* site).  The port therefore defines it once at the top (P-57's ordering rule) and
  passes the context: the call sites read `XREF_YREF_EQUAL! (ctxt, x, y)`, which also keeps C's
  `ctxt` token in the function bodies the gate compares.
- **`SNAKE_LIMIT` is `pub const SNAKE_LIMIT: OFFSET = 20`.**  C declares it as an `int` literal
  (`#define SNAKE_LIMIT 20`, oddly placed inside `struct context`) and every use promotes it to
  `OFFSET`; the port spells the promoted type so the includer's `OFFSET` decides (C's own
  promotion), and the token comparison sees C's `20`.
- **`NOTE_ORDERED` is a `pub const bool = false`** (C's `# define NOTE_ORDERED false` default)
  and **`EARLY_ABORT` a `macro_rules!` taking C's `ctxt` parameter** (`($ctxt:expr) => { false }`);
  `OFFSET_MAX` is a nullary `macro_rules!` whose body is C 86-90's expression with the includer's
  `OFFSET` and `limits.in.rs`'s `CHAR_BIT`.
- **Locals C leaves uninitialised** (`fxbest`, `bxbest`, C 397-398 and 411) are zero-initialised
  per PORTING.md 1.5 and named in the bodies' comments.
- **`while (true)` stays `while true`**, not `loop`: the gate's token comparison keeps C's
  `true`.  Likewise C's `for (…; …; …) continue;` snake-extension loops keep their `continue`
  (`continue` is not one of the tokens the gate skips), the way `buffer.rs`/`intervals.rs` do.
- **No compiler, by measurement.**  `grep -rn 'diffseq.h' emacs/` finds only
  `lib/gnulib.mk.in` 1901's `libgnu_a_SOURCES += diffseq.h` (plus `configure`/`ChangeLog`
  noise), i.e. gnulib ships the module and **no translation unit includes it**: the reference has
  no consumer, so there is no unit to smoke-test and no probe to add, and `--check` plus the
  contract above are the whole gate.  When a file does include it, that unit's build is its
  compile proof (and its `OFFSET`/`ELEMENT`/`EXTRA_CONTEXT_FIELDS` items settle the shapes).
- **C's `#pragma GCC diagnostic push`/`ignored …`/`pop`** (C 106-116, C 571-573) are diagnostics
  with no Rust counterpart, and the closing `#undef` block (C 573-577) is C's own hygiene - in
  Rust the `macro_rules!`s are defined once for the whole unit, so there is nothing to undefine.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/diffseq.h femacs/lib/include/diffseq.rs`
  -> `OK emacs/lib/diffseq.h -> femacs/lib/include/diffseq.rs` (no `- ` problem lines), with the
  gate's own line `(1 C macros have several definitions (configuration branches), body not
  compared: XREF_YREF_EQUAL)` - C defines that macro twice (once per function), so only its
  presence is compared.
- The two function bodies are compared token-for-token against C's *frozen* arms, which is the
  mechanical proof that no construct of `diag` (C 194-465) or `compareseq` (C 467-570) was
  dropped; it is not a behavioural proof (this file has no ERT/behavioural test, and no unit
  compiles it yet).
- Consumer smoke: **not applicable** - no translation unit includes `lib/diffseq.h` in the
  reference either (measured above), so there is no `.c` to build for it.
- Gate readings (repo root): `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0;
  `check-verbatim` 6 trees / 4607 files / 0; `check-mirror-dupes` 1 duplicate declaration / 1
  ratchet row / 0 failures; `check-globals` 283 units / 0; `check-target-names` 0 findings.
- **Parse-clean, unresolved only by the includer's names** (extra evidence, since no unit
  compiles this file yet): `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib
  --emit=metadata -o /tmp/diffseq.rmeta femacs/lib/include/diffseq.rs` reports
  `122 x E0425` (the includer's `OFFSET`, `ELEMENT`, `XVECREF_YVECREF_EQUAL`, `NOTE_DELETE`,
  `NOTE_INSERT`, `EXTRA_CONTEXT_FIELDS`, `EQUAL`) and `4 x E0433`, and **0 syntax errors** - so
  the transcription is a syntactically valid Rust unit whose only missing items are exactly the
  names C's contract says the includer defines.

