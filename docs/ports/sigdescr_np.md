# emacs/lib/sigdescr_np.c (351 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] sigdescr_np  (line 27)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **A new mirror for the system `<signal.h>`.**  C's arms are guarded by `#if defined SIGX`,
  which the *system* header decides and the frozen configuration table does not carry, so the
  evaluator asks for every arm.  `femacs/lib/include/signal.rs` records the 31 signals the
  frozen target defines with the values measured from its clang (`-dM -E`); the other 32
  (SIGPWR, SIGSTKFLT, the AIX/IRIX/Solaris/HP-UX ones, ...) do not exist here, so their arms are
  kept as text under `#[cfg (feature = "signal_…")]` with C's string, the P-51 shape: the text
  and its tokens stay in the port (the checker's row in docs/ports/macro-body-notes.tsv lists
  them), and the feature cannot be enabled on this target because the signal's own value does
  not exist.
- **C's `switch (sig)` becomes a `match`** with the same arms in the same order, falling out to
  the same trailing `return NULL` - the port's `_ => { }` arm, since C's switch has no
  `default:` and simply leaves the switch.  The `switch`/`case`/`default` tokens are in
  docs/ports/macro-body-notes.tsv.
- **One arm's `return` carries a trailing comment in C** (`return "CPU going down"; /* See man
  bindprocessor */`, the SIGCPUFAIL arm); the port keeps it after the return, as C does.
- **The strings are C's, character for character** (the file's whole point is the wording), and
  the arms are generated from C's own text, so no wording was retyped.
- **Call-relationship differences (D7)**: none - the function calls nothing; C's `<string.h>`
  include has no counterpart (PORTING.md 0.1).
