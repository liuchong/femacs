# emacs/lib/fingerprint.h (28 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_FINGERPRINT_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/fingerprint.h` @ `fd7afced403`; port `femacs/lib/include/fingerprint.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/lib/fingerprint.h femacs/lib/include/fingerprint.rs`
-> `OK`.

- **There is no `fingerprint.c` to port.**  The reference *generates* it from the list of all
  sources (`lib-src/make-fingerprint` and the build rule that writes `fingerprint.c`), so the
  object `fingerprint[32]` belongs to the build scaffolding (`xtask`) and this mirror only
  declares it, exactly as the header does.  The declaration is
  `unsafe extern "C" { pub static mut fingerprint: [c_uchar; 32]; }`; C's `volatile` qualifier
  has no counterpart in a Rust declaration (every access already goes through `unsafe`).
- The include guard has no Rust counterpart (`pub const EMACS_FINGERPRINT_H: bool = true`,
  PORTING.md 0.1).
- `pdumper.h` 23 includes this header, so `femacs/src/include/pdumper.rs` pulls this mirror in;
  `dump_fingerprint`'s `unsigned char const fingerp[sizeof fingerprint]` parameter is C's array
  parameter decayed to `*const c_uchar`.
