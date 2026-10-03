// femacs: port of lib/fingerprint.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Header mirror (PORTING.md 0.1): the include guard and the one object the build
// generates from every source file.  There is no `fingerprint.c` to port - the
// reference writes it and compiles it from the list of all sources - so the object
// itself belongs to the build scaffolding (`xtask`), and this mirror only declares
// it, the way the header does.

// C's `unsigned char` needs no include.  The type is written fully qualified rather than
// imported, because this mirror is included by `pdumper.rs` alongside `lisp.rs`, whose own
// `use std::ffi::{... c_uchar ...}` would make a second import of the name an E0252.
// fingerprint.h 20-21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_FINGERPRINT_H: bool = true;

/* We generate fingerprint.c and fingerprint.o from all the sources in
   Emacs.  This way, we have a unique value that we can use to pair
   data files (like a dump file) with a specific build of Emacs.  */
unsafe extern "C" { pub static mut fingerprint: [core::ffi::c_uchar; 32]; }             // 26
