//! Format controls and negative units only. Never a real Lisp compiler or a
//! performance candidate. Compile distinct owned cdylibs in D, never in B.
#![allow(dead_code)]
use core::ffi::c_void;

#[cfg(not(any(
    negative = "valid-format", negative = "missing-comp-unit", negative = "missing-top-level",
    negative = "null-reloc", negative = "null-eph-reloc", negative = "missing-reloc-count",
    negative = "missing-eph-count", negative = "bad-reloc-count", negative = "bad-eph-count",
    negative = "negative-reloc-count", negative = "wrong-abi", negative = "null-blob",
    negative = "negative-blob-length", negative = "malformed-blob", negative = "data-not-vector",
    negative = "eph-not-vector", negative = "missing-thread-reloc", negative = "missing-symbols-reloc",
    negative = "missing-function-table"
)))]
compile_error!("select exactly one documented negative unit kind");

#[repr(C)]
struct StaticObject<const N: usize> {
    len: isize,
    data: [u8; N],
}

const fn object<const N: usize>(bytes: &[u8; N]) -> StaticObject<N> {
    StaticObject { len: N as isize, data: *bytes }
}

// The D fixture builder obtains this serialized Lisp string from the actual
// selected native product, not a guessed original ABI or a shared cache.
const ABI_BYTES: &[u8; include_bytes!(env!("FEMACS_RUNTIME_ABI_LITERAL_INPUT")).len()] =
    include_bytes!(env!("FEMACS_RUNTIME_ABI_LITERAL_INPUT"));
static mut ABI: StaticObject<{ ABI_BYTES.len() }> = object(ABI_BYTES);
static mut WRONG_ABI: StaticObject<21> = object(b"\"not-the-runtime-abi\"");
static mut MALFORMED: StaticObject<1> = object(b"(");
static mut VECTOR: StaticObject<5> = object(b"[nil]");
static mut NONVECTOR: StaticObject<2> = object(b"42");
static mut NIL: StaticObject<3> = object(b"nil");

#[cfg(not(negative = "missing-comp-unit"))]
#[unsafe(no_mangle)]
pub static mut comp_unit: *mut c_void = core::ptr::null_mut();

#[cfg(not(negative = "missing-thread-reloc"))]
#[unsafe(no_mangle)]
pub static mut current_thread_reloc: *mut c_void = core::ptr::null_mut();

#[cfg(not(negative = "missing-symbols-reloc"))]
#[unsafe(no_mangle)]
pub static mut f_symbols_with_pos_enabled_reloc: *mut c_void = core::ptr::null_mut();

#[cfg(not(negative = "missing-function-table"))]
#[unsafe(no_mangle)]
pub static mut freloc_link_table: *mut c_void = core::ptr::null_mut();

static mut DATA: [usize; 1] = [0];
static mut EPHEMERAL: [usize; 1] = [0];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn d_reloc_addr() -> *mut c_void {
    if cfg!(negative = "null-reloc") {
        core::ptr::null_mut()
    } else {
        core::ptr::addr_of_mut!(DATA).cast()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn d_reloc_eph_addr() -> *mut c_void {
    if cfg!(negative = "null-eph-reloc") {
        core::ptr::null_mut()
    } else {
        core::ptr::addr_of_mut!(EPHEMERAL).cast()
    }
}

#[cfg(not(negative = "missing-reloc-count"))]
#[unsafe(no_mangle)]
pub extern "C" fn d_reloc_count() -> isize {
    if cfg!(negative = "negative-reloc-count") { -1 }
    else if cfg!(negative = "bad-reloc-count") { 0 }
    else { 1 }
}

#[cfg(not(negative = "missing-eph-count"))]
#[unsafe(no_mangle)]
pub extern "C" fn d_reloc_eph_count() -> isize {
    if cfg!(negative = "bad-eph-count") { 0 } else { 1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn freloc_hash() -> *mut c_void {
    if cfg!(negative = "null-blob") { return core::ptr::null_mut(); }
    if cfg!(negative = "wrong-abi") { return core::ptr::addr_of_mut!(WRONG_ABI).cast(); }
    if cfg!(negative = "malformed-blob") { return core::ptr::addr_of_mut!(MALFORMED).cast(); }
    if cfg!(negative = "negative-blob-length") {
        unsafe { (*core::ptr::addr_of_mut!(ABI)).len = -1; }
    }
    core::ptr::addr_of_mut!(ABI).cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_data_reloc() -> *mut c_void {
    if cfg!(negative = "data-not-vector") { core::ptr::addr_of_mut!(NONVECTOR).cast() }
    else { core::ptr::addr_of_mut!(VECTOR).cast() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_data_reloc_eph() -> *mut c_void {
    if cfg!(negative = "eph-not-vector") { core::ptr::addr_of_mut!(NONVECTOR).cast() }
    else { core::ptr::addr_of_mut!(VECTOR).cast() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_optim_qly() -> *mut c_void { core::ptr::addr_of_mut!(NIL).cast() }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_data_fdoc() -> *mut c_void { core::ptr::addr_of_mut!(NIL).cast() }

#[cfg(not(negative = "missing-top-level"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn top_level_run(_unit: *mut c_void) -> *mut c_void {
    core::ptr::null_mut()
}

#[cfg(not(negative = "missing-top-level"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn late_top_level_run(unit: *mut c_void) -> *mut c_void {
    unsafe { top_level_run(unit) }
}
