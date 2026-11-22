#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
//! Host boundary crate linking the single VM owner (`femacs-vm`).

pub use femacs_vm;

/// Invoke the editor entry point with raw OS `argc` and `argv`.
///
/// # Safety
/// `argc` and `argv` must be the valid process argument vector supplied by the OS C runtime.
#[inline]
pub unsafe fn run_editor_main(
    argc: core::ffi::c_int,
    argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    // SAFETY: Caller passes the OS process argument vector directly from `main`.
    unsafe { femacs_vm::emacs::femacs_editor_main(argc, argv) }
}
