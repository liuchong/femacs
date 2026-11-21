// femacs: port of lib/fingerprint.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): the include guard and the one object the build
// generates from every source file.  There is no `fingerprint.c` to port - the
// reference writes it and compiles it from the list of all sources - so the object
// itself belongs to the build scaffolding (`xtask`), and this mirror only declares
// it, the way the header does.

// C's `unsigned char` needs no include, and this mirror does not import the Rust spelling
// either: like every other mirror, the including unit brings the C names into scope (the
// `lib/include/stdbit.rs` comment says the same), so `pdumper.rs`'s unit provides `c_uchar`
// through `lisp.rs` and a second `use` here would be an E0252.
// fingerprint.h 20-21: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_FINGERPRINT_H: bool = true;

/* We generate fingerprint.c and fingerprint.o from all the sources in
   Emacs.  This way, we have a unique value that we can use to pair
   data files (like a dump file) with a specific build of Emacs.  */
unsafe extern "C" { pub static mut fingerprint: [c_uchar; 32]; }             // 26
