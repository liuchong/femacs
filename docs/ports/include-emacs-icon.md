# emacs/src/emacs-icon.h (353 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] gnu_xbm_height  (line 322, body e1822db470e6)
- [x] gnu_xbm_width  (line 321, body e1822db470e6)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The XPM half is dead text, and the reference says why it exists at all.**  C 26's
  `#if (defined (HAVE_XPM) && defined (HAVE_X_WINDOWS)) || defined (USE_GTK)` is false for the
  frozen target: all three names are undefined (docs/baseline/config/darwin-arm64.macros.tsv
  1708, 1720, 3861) and `tools/check-cond-eval.py src/emacs-icon.h` agrees with clang on that
  condition.  C 27-318 (the `gnu_xpm_bits` XPM array with its 255-colour table) is therefore
  recorded here and not transcribed (P-08); the reference's own comment at C 19-20 explains the
  arrangement the guard encodes - the GTK port paints the icon with gdk, so an XPM-capable build
  is not required for it.
- **The XBM half is live and unconditional**: the two size macros and the bit array below the
  guard.  C writes the array `static` (internal linkage, one copy per including unit), so the
  port keeps that spelling - `pub static mut gnu_xbm_bits: [c_uchar; 348]` with the length C's
  own initializer list measures - and no `#[no_mangle]`: the `nm` line in Evidence shows the
  symbol mangled, i.e. internal, which is what C's file-scope `static` is.  C's element type is
  `unsigned char` -> `c_uchar`, and the two macros' bodies are the bare integer 50, hence their
  identical body hash (e1822db470e6).
- **The header has no include guard** (C goes straight from the opening comment to the first
  `#if`), so there is no guard name to carry - recorded rather than invented.
- **No consumer compiles this mirror**: its only includer in the reference is `src/xterm.c`, the
  X11 backend, whose unit is an empty shape on this target (docs/status.md 3.31).  The compile
  proof is therefore a standalone `rustc` run (Evidence), which is also what proves the array
  literal is well formed.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/emacs-icon.h femacs/src/include/emacs-icon.rs`
  -> `OK emacs/src/emacs-icon.h -> femacs/src/include/emacs-icon.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/emacs-icon.h`
  -> `1 conditions, 0 unknown to the evaluator, 0 disagreements`; `src/emacs-icon.h` is in
  `docs/baseline/config/cond-eval-verified.txt`, so the frozen table decides it.
- Compile proof (standalone crate, the mirror included with `c_int`/`c_uchar` in scope):
  `rustc --edition 2024 --crate-type=lib --emit=obj -o /tmp/emi/probe.o probe.rs` -> exit 0
  (3 warnings, all `static_mut_refs`-class allowances), and `nm -g /tmp/emi/probe.o` prints
  `D __RNvCs20UcKX0XH6B_5probe12gnu_xbm_bits` - mangled, i.e. internal linkage, as designed.
- `python3 tools/extract-abi.py emacs/src/emacs-icon.h` -> this ledger: 0 functions, 2 macros,
  0 types, 0 file-scope objects to mirror (the `static` array is the mirror's own item).
