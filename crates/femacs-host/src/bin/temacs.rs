#![no_main]
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *mut *mut core::ffi::c_char,
) -> core::ffi::c_int {
    // SAFETY: Invoked directly by the OS C startup runtime with valid `argc`/`argv`.
    unsafe { femacs_host::femacs_vm::editor_main(argc, argv) }
}
