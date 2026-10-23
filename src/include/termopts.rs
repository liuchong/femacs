// femacs: port of src/termopts.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The whole header is its guard (C 20-21), two comments and
// two `extern bool` declarations (C 24 and C 27); there is no other `#if`
// (`tools/check-cond-eval.py src/termopts.h` -> `1 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`, the guard).  It has no `.c` of its own: the storage lives in the unit whose
// source defines the objects, `src/keyboard.c` 348/351, and C's other readers (`src/sysdep.c`
// 1296/1542, `src/process.c` 6041, `src/xdisp.c`, …) reach them through this header.
//
// **The two declarations sit in a module the mirror glob-imports, not at the crate root.**
// `src/keyboard.rs` 314/316 *defines* both objects at its crate root (C 348/351 are
// theirs), and a crate-root declaration of a name the unit also defines at the crate root is
// `E0428` (measured; the `frame.rs` `frame_globals` / `term.rs` `term_globals` shape, PORTING.md
// P-21).  Putting them in `termopts_objs` and glob-importing it keeps C's prototype-plus-
// definition pair: the unit's definition shadows the import, and a unit that has no definition
// resolves the name through the import.  The same reading is why `include/atimer.rs`'s seven
// prototypes live in `atimer_decls` - except that there the *definitions* are the mirror's
// names, while here it is the unit's.
//
// `src/sysdep.rs` 404 and 966 declare both names at their own crate roots today
// (`termopts.h` had no mirror when those lines were written); those use-site copies are not in
// this mirror's include graph, so nothing here shadows them.
//
// Nothing on this target compiles the header either: no `src/termopts.c` exists and no unit
// `include!`s this mirror yet.  Its compile evidence is `spikes/win_inc_probe.rs`, which takes
// it together with the platform mirrors of the same class (see docs/ports/include-termopts.md).

// C 20-21: the include guard has no Rust counterpart (PORTING.md 0.1-4).
pub const EMACS_TERMOPTS_H: bool = true;

/// C 23-27: the two objects, exactly as C declares them.  `bool` is C's `<stdbool.h>` bool
/// (one byte, PORTING.md 1.1); the port's `check-globals.py` does not apply to either - C's
/// `DEFVAR_*` family is not how these are declared, so they are ordinary file-scope objects.
pub mod termopts_objs {
  use super::*;

  unsafe extern "C" {
    /// C 24: `extern bool interrupt_input;` - nonzero means interrupt-driven input.
    pub static mut interrupt_input: bool;

    /// C 27: `extern bool interrupts_deferred;` - nonzero while interrupts are temporarily
    /// deferred during redisplay.
    pub static mut interrupts_deferred: bool;
  }
}
use termopts_objs::*;
