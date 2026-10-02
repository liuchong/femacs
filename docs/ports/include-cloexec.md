# emacs/lib/cloexec.h (44 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/cloexec.h` @ `fd7afced403`; port `femacs/lib/include/cloexec.rs`.  The header is the
two prototypes (`set_cloexec_flag`, `dup_cloexec`) and their comments; `femacs/lib/cloexec.rs`
defines both.  The `extern "C"` wrapper C 17-19/37-39 spells is a no-op in Rust.
