# Retained invocation and receipt contracts

The P387 thread lifecycle already calls the JNI attach/detach entries. Complete
its retained C invocation table declaration with the three reserved pointers,
five function entries and `JNI_OK == 0`. The arm64 NDK 29.0 header independently
confirms size 64 and entry offsets 24/32/40/48/56, plus all pointer signatures.
The actual Rust header compiles with its layout assertions and the thread-call
signatures. This is ABI/declaration acceptance, not Android device acceptance.

The required Android source-namespace matcher now uses an ASCII escape with
identical runtime bytes and mapping. The host tool passes `cargo check`; the
previous existing declaration-prose gate failure is closed, with zero current
findings. No Android package name or original input data is changed.

Seven synthetic client-receipt protections close the already implemented
version-3 file-set contract: both product layouts, absent built/packaged clients,
version-2 preservation/extra-file rejection, and unknown-version rejection.
They pass after the original client integration; this file-set check does not
substitute for executable, signing, OS-preservation or device acceptance.

All checks are local. No remote CI ran, and the seven-part Goal remains open.
