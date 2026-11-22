// femacs: port of lib/mini-gmp.c @ fd7afced403.
//! Thin C-ABI boundary adapter delegating all arbitrary-precision arithmetic to
//! `#![forbid(unsafe_code)]` `femacs_pure::mini_gmp`.
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports
)]

use crate::common::*;
use core::sync::atomic::{AtomicPtr, Ordering};
use femacs_pure::mini_gmp::{self as pure_gmp, DivRoundMode, Mpz, MpzRef};

pub const MINI_GMP_ENABLE_FLOAT: c_int = 1;
pub const GMP_LIMB_BITS: usize = pure_gmp::GMP_LIMB_BITS;
pub const GMP_LIMB_MAX: mp_limb_t = pure_gmp::GMP_LIMB_MAX;
pub const GMP_LIMB_HIGHBIT: mp_limb_t = pure_gmp::GMP_LIMB_HIGHBIT;

#[unsafe(no_mangle)]
pub static mp_bits_per_limb: c_int = GMP_LIMB_BITS as c_int;

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    #[link_name = "realloc"]
    fn realloc_system(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn abort() -> !;
}

type AllocFn = unsafe extern "C-unwind" fn(usize) -> *mut c_void;
type ReallocFn = unsafe extern "C-unwind" fn(*mut c_void, usize, usize) -> *mut c_void;
type FreeFn = unsafe extern "C-unwind" fn(*mut c_void, usize);

unsafe extern "C-unwind" fn gmp_default_alloc(size: usize) -> *mut c_void {
    assert!(size > 0);
    // SAFETY: Requests `size > 0` heap bytes from libc `malloc` and aborts on exhaustion.
    let p = unsafe { malloc(size) };
    if p.is_null() {
        // SAFETY: Standard C abort on default GMP allocator exhaustion.
        unsafe { abort() };
    }
    p
}

unsafe extern "C-unwind" fn gmp_default_realloc(
    old: *mut c_void,
    _unused_old_size: usize,
    new_size: usize,
) -> *mut c_void {
    let req = if new_size == 0 { 1 } else { new_size };
    // SAFETY: Resizes `old` via libc `realloc` and aborts on exhaustion.
    let p = unsafe { realloc_system(old, req) };
    if p.is_null() {
        // SAFETY: Standard C abort on default GMP allocator exhaustion.
        unsafe { abort() };
    }
    p
}

unsafe extern "C-unwind" fn gmp_default_free(p: *mut c_void, _unused_size: usize) {
    // SAFETY: Releases heap block `p` previously allocated by `gmp_default_alloc`/`realloc`.
    unsafe { free(p) };
}

static GMP_ALLOC_PTR: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
static GMP_REALLOC_PTR: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
static GMP_FREE_PTR: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());

#[inline]
fn current_alloc_fn() -> AllocFn {
    let p = GMP_ALLOC_PTR.load(Ordering::Acquire);
    if p.is_null() {
        gmp_default_alloc
    } else {
        // SAFETY: Only valid `AllocFn` function pointers or null are stored in `GMP_ALLOC_PTR`.
        unsafe { core::mem::transmute::<*mut (), AllocFn>(p) }
    }
}

#[inline]
fn current_realloc_fn() -> ReallocFn {
    let p = GMP_REALLOC_PTR.load(Ordering::Acquire);
    if p.is_null() {
        gmp_default_realloc
    } else {
        // SAFETY: Only valid `ReallocFn` function pointers or null are stored in `GMP_REALLOC_PTR`.
        unsafe { core::mem::transmute::<*mut (), ReallocFn>(p) }
    }
}

#[inline]
fn current_free_fn() -> FreeFn {
    let p = GMP_FREE_PTR.load(Ordering::Acquire);
    if p.is_null() {
        gmp_default_free
    } else {
        // SAFETY: Only valid `FreeFn` function pointers or null are stored in `GMP_FREE_PTR`.
        unsafe { core::mem::transmute::<*mut (), FreeFn>(p) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mp_set_memory_functions(
    alloc_func: Option<AllocFn>,
    realloc_func: Option<ReallocFn>,
    free_func: Option<FreeFn>,
) {
    GMP_ALLOC_PTR.store(
        alloc_func.map_or(core::ptr::null_mut(), |f| f as *mut ()),
        Ordering::Release,
    );
    GMP_REALLOC_PTR.store(
        realloc_func.map_or(core::ptr::null_mut(), |f| f as *mut ()),
        Ordering::Release,
    );
    GMP_FREE_PTR.store(
        free_func.map_or(core::ptr::null_mut(), |f| f as *mut ()),
        Ordering::Release,
    );
}

pub fn mp_get_memory_functions() -> (AllocFn, ReallocFn, FreeFn) {
    (current_alloc_fn(), current_realloc_fn(), current_free_fn())
}

static DUMMY_LIMB: mp_limb_t = GMP_LIMB_MAX & 0xc1a0;

#[inline]
unsafe fn read_mpz<'a>(u: *const __mpz_struct) -> MpzRef<'a> {
    // SAFETY: Caller passes a valid initialized or `mpz_roinit_n` `__mpz_struct` pointer.
    unsafe {
        let sz = (*u)._mp_size;
        let len = sz.unsigned_abs() as usize;
        let limbs = if len == 0 {
            &[]
        } else {
            core::slice::from_raw_parts((*u)._mp_d, len)
        };
        MpzRef::normalized(sz < 0, limbs)
    }
}

unsafe fn ensure_mpz_limbs(r: *mut __mpz_struct, needed: usize) -> *mut mp_limb_t {
    let cap = needed.max(1);
    // SAFETY: Caller passes a valid mutable `__mpz_struct`; `_mp_alloc == 0` denotes
    // unallocated or `mpz_roinit_n` storage that must allocate fresh limbs rather than realloc.
    unsafe {
        let cur_alloc = (*r)._mp_alloc as usize;
        if cap > cur_alloc {
            let new_bytes = cap * core::mem::size_of::<mp_limb_t>();
            let new_ptr = if cur_alloc != 0 {
                let old_bytes = cur_alloc * core::mem::size_of::<mp_limb_t>();
                (current_realloc_fn())((*r)._mp_d.cast::<c_void>(), old_bytes, new_bytes)
            } else {
                (current_alloc_fn())(new_bytes)
            }
            .cast::<mp_limb_t>();
            (*r)._mp_d = new_ptr;
            (*r)._mp_alloc = cap as c_int;
        }
        (*r)._mp_d
    }
}

unsafe fn write_mpz(r: *mut __mpz_struct, val: Mpz) {
    let n = pure_gmp::mpn_normalized_size(&val.limbs);
    // SAFETY: Writes normalized limbs into `r`, allocating via registered GMP memory functions if needed.
    unsafe {
        if n == 0 {
            (*r)._mp_size = 0;
            return;
        }
        let dst = ensure_mpz_limbs(r, n);
        core::ptr::copy_nonoverlapping(val.limbs.as_ptr(), dst, n);
        let signed = n as c_int;
        (*r)._mp_size = if val.neg { -signed } else { signed };
    }
}

// ---- Active C-ABI `mpz_*` entrypoints used by `src/` ----

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init(r: *mut __mpz_struct) {
    // SAFETY: Initializes `*r` to canonical zero with static dummy limb pointer (`_mp_alloc = 0`).
    unsafe {
        (*r)._mp_alloc = 0;
        (*r)._mp_size = 0;
        (*r)._mp_d = (&raw const DUMMY_LIMB).cast_mut();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_clear(r: *mut __mpz_struct) {
    // SAFETY: Releases heap limb array iff `_mp_alloc != 0` (leaving `mpz_roinit_n` cold limbs untouched).
    unsafe {
        let alloc = (*r)._mp_alloc as usize;
        if alloc != 0 {
            (current_free_fn())(
                (*r)._mp_d.cast::<c_void>(),
                alloc * core::mem::size_of::<mp_limb_t>(),
            );
            (*r)._mp_alloc = 0;
            (*r)._mp_size = 0;
            (*r)._mp_d = (&raw const DUMMY_LIMB).cast_mut();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_si(r: *mut __mpz_struct, x: c_long) {
    // SAFETY: Caller passes valid writable `r`.
    unsafe { write_mpz(r, Mpz::from_si(x)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_ui(r: *mut __mpz_struct, x: c_ulong) {
    // SAFETY: Caller passes valid writable `r`.
    unsafe { write_mpz(r, Mpz::from_ui(x)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set(r: *mut __mpz_struct, x: *const __mpz_struct) {
    if core::ptr::eq(r.cast_const(), x) {
        return;
    }
    // SAFETY: Caller passes valid `r` and `x`; `x` is copied before mutating `r`.
    unsafe {
        let owned = read_mpz(x).to_owned_mpz();
        write_mpz(r, owned);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_init_set_ui(r: *mut __mpz_struct, x: c_ulong) {
    // SAFETY: Initializes `*r` then stores `x`.
    unsafe {
        mpz_init(r);
        mpz_set_ui(r, x);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_slong_p(u: *const __mpz_struct) -> c_int {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_fits_slong_p(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_ulong_p(u: *const __mpz_struct) -> c_int {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_fits_ulong_p(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fits_sint_p(u: *const __mpz_struct) -> c_int {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_fits_sint_p(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_si(u: *const __mpz_struct) -> c_long {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_get_si(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_ui(u: *const __mpz_struct) -> c_ulong {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_get_ui(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_size(u: *const __mpz_struct) -> usize {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { (*u)._mp_size.unsigned_abs() as usize }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_getlimbn(u: *const __mpz_struct, n: mp_size_t) -> mp_limb_t {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_getlimbn(read_mpz(u), n as isize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_read(x: *const __mpz_struct) -> mp_srcptr {
    // SAFETY: Caller passes valid readable `x`.
    unsafe { (*x)._mp_d.cast_const() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_write(x: *mut __mpz_struct, n: mp_size_t) -> mp_ptr {
    assert!(n > 0);
    // SAFETY: Ensures `x` has capacity for at least `n` writable limbs.
    unsafe { ensure_mpz_limbs(x, n as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_limbs_finish(x: *mut __mpz_struct, xs: mp_size_t) {
    let abs_n = xs.unsigned_abs() as usize;
    // SAFETY: Caller populated at least `|xs|` limbs in `(*x)._mp_d`.
    unsafe {
        let slice = if abs_n == 0 {
            &[]
        } else {
            core::slice::from_raw_parts((*x)._mp_d, abs_n)
        };
        let xn = pure_gmp::mpn_normalized_size(slice) as c_int;
        (*x)._mp_size = if xs < 0 { -xn } else { xn };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_roinit_n(
    x: *mut __mpz_struct,
    xp: mp_srcptr,
    xs: mp_size_t,
) -> mpz_srcptr {
    // SAFETY: Initializes `*x` as a non-owning (`_mp_alloc = 0`) read-only view over `xp[..|xs|]`.
    unsafe {
        (*x)._mp_alloc = 0;
        (*x)._mp_d = xp.cast_mut();
        mpz_limbs_finish(x, xs);
        x.cast_const()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_d(r: *mut __mpz_struct, x: c_double) {
    // SAFETY: Caller passes valid writable `r`.
    unsafe { write_mpz(r, pure_gmp::mpz_from_d(x)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_d(u: *const __mpz_struct) -> c_double {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_get_d(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp_d(x: *const __mpz_struct, d: c_double) -> c_int {
    // SAFETY: Caller passes valid readable `x`.
    unsafe { pure_gmp::mpz_cmp_d(read_mpz(x), d) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sgn(u: *const __mpz_struct) -> c_int {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { read_mpz(u).sgn() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp_ui(u: *const __mpz_struct, v: c_ulong) -> c_int {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_cmp_ui(read_mpz(u), v) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmp(
    a: *const __mpz_struct,
    b: *const __mpz_struct,
) -> c_int {
    // SAFETY: Caller passes valid readable `a` and `b`.
    unsafe { pure_gmp::mpz_cmp(read_mpz(a), read_mpz(b)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cmpabs(
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) -> c_int {
    // SAFETY: Caller passes valid readable `u` and `v`.
    unsafe { pure_gmp::mpz_cmpabs(read_mpz(u), read_mpz(v)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_abs(r: *mut __mpz_struct, u: *const __mpz_struct) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_abs(read_mpz(u));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_neg(r: *mut __mpz_struct, u: *const __mpz_struct) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_neg(read_mpz(u));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_swap(u: *mut __mpz_struct, v: *mut __mpz_struct) {
    if core::ptr::eq(u, v) {
        return;
    }
    // SAFETY: Caller passes two distinct valid mutable `__mpz_struct` pointers.
    unsafe { core::ptr::swap(u, v) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_add_ui(
    r: *mut __mpz_struct,
    a: *const __mpz_struct,
    b: c_ulong,
) {
    // SAFETY: Caller passes valid `r` and `a` (may alias).
    unsafe {
        let res = pure_gmp::mpz_add_ui(read_mpz(a), b);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sub_ui(
    r: *mut __mpz_struct,
    a: *const __mpz_struct,
    b: c_ulong,
) {
    // SAFETY: Caller passes valid `r` and `a` (may alias).
    unsafe {
        let res = pure_gmp::mpz_sub_ui(read_mpz(a), b);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_add(
    r: *mut __mpz_struct,
    a: *const __mpz_struct,
    b: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `a`, `b` (may alias).
    unsafe {
        let res = pure_gmp::mpz_add(read_mpz(a), read_mpz(b));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sub(
    r: *mut __mpz_struct,
    a: *const __mpz_struct,
    b: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `a`, `b` (may alias).
    unsafe {
        let res = pure_gmp::mpz_sub(read_mpz(a), read_mpz(b));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul_ui(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: c_ulong,
) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_mul_ui(read_mpz(u), v);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_mul(read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_mul_2exp(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    bits: mp_bitcnt_t,
) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_mul_2exp(read_mpz(u), bits);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_addmul_ui(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: c_ulong,
) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_addmul_ui(read_mpz(r.cast_const()), read_mpz(u), v);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_addmul(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_addmul(read_mpz(r.cast_const()), read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_submul(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_submul(read_mpz(r.cast_const()), read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[inline]
unsafe fn write_div_qr(
    q: *mut __mpz_struct,
    r: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
    mode: DivRoundMode,
) {
    // SAFETY: Reads `n` and `d` before writing non-null `q`/`r` outputs (supporting aliasing).
    unsafe {
        let (q_val, r_val, _) =
            pure_gmp::mpz_div_qr(read_mpz(n), read_mpz(d), mode, !q.is_null(), !r.is_null());
        if let Some(qv) = q_val {
            write_mpz(q, qv);
        }
        if let Some(rv) = r_val {
            write_mpz(r, rv);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_qr(
    q: *mut __mpz_struct,
    r: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Floor` rounding.
    unsafe { write_div_qr(q, r, n, d, DivRoundMode::Floor) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_qr(
    q: *mut __mpz_struct,
    r: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Trunc` rounding.
    unsafe { write_div_qr(q, r, n, d, DivRoundMode::Trunc) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_cdiv_q(
    q: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Ceil` rounding.
    unsafe { write_div_qr(q, core::ptr::null_mut(), n, d, DivRoundMode::Ceil) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q(
    q: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Floor` rounding.
    unsafe { write_div_qr(q, core::ptr::null_mut(), n, d, DivRoundMode::Floor) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_q(
    q: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Trunc` rounding.
    unsafe { write_div_qr(q, core::ptr::null_mut(), n, d, DivRoundMode::Trunc) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_r(
    r: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Floor` rounding.
    unsafe { write_div_qr(core::ptr::null_mut(), r, n, d, DivRoundMode::Floor) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_r(
    r: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Delegates to `write_div_qr` under `Trunc` rounding.
    unsafe { write_div_qr(core::ptr::null_mut(), r, n, d, DivRoundMode::Trunc) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q_2exp(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    cnt: mp_bitcnt_t,
) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_div_q_2exp(read_mpz(u), cnt, DivRoundMode::Floor);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_divexact(
    q: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `q`, `n`, `d` (may alias).
    unsafe {
        let res = pure_gmp::mpz_divexact(read_mpz(n), read_mpz(d));
        write_mpz(q, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_fdiv_q_ui(
    q: *mut __mpz_struct,
    n: *const __mpz_struct,
    d: c_ulong,
) -> c_ulong {
    // SAFETY: Caller passes valid `n` and optional/valid `q`.
    unsafe {
        let (qv, _, rem) = pure_gmp::mpz_div_qr_ui(read_mpz(n), d, DivRoundMode::Floor, !q.is_null(), false);
        if let Some(val) = qv {
            write_mpz(q, val);
        }
        rem
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_tdiv_ui(n: *const __mpz_struct, d: c_ulong) -> c_ulong {
    // SAFETY: Caller passes valid readable `n`.
    unsafe {
        let (_, _, rem) = pure_gmp::mpz_div_qr_ui(read_mpz(n), d, DivRoundMode::Trunc, false, false);
        rem
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_gcd(
    g: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `g`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_gcd(read_mpz(u), read_mpz(v));
        write_mpz(g, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_pow_ui(
    r: *mut __mpz_struct,
    b: *const __mpz_struct,
    e: c_ulong,
) {
    // SAFETY: Caller passes valid `r` and `b` (may alias).
    unsafe {
        let res = pure_gmp::mpz_pow_ui(read_mpz(b), e);
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_ui_pow_ui(r: *mut __mpz_struct, blimb: c_ulong, e: c_ulong) {
    // SAFETY: Caller passes valid writable `r`.
    unsafe { write_mpz(r, pure_gmp::mpz_ui_pow_ui(blimb, e)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_com(r: *mut __mpz_struct, u: *const __mpz_struct) {
    // SAFETY: Caller passes valid `r` and `u` (may alias).
    unsafe {
        let res = pure_gmp::mpz_com(read_mpz(u));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_and(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_and(read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_ior(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_ior(read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_xor(
    r: *mut __mpz_struct,
    u: *const __mpz_struct,
    v: *const __mpz_struct,
) {
    // SAFETY: Caller passes valid `r`, `u`, `v` (may alias).
    unsafe {
        let res = pure_gmp::mpz_xor(read_mpz(u), read_mpz(v));
        write_mpz(r, res);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_popcount(u: *const __mpz_struct) -> mp_bitcnt_t {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_popcount(read_mpz(u)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_scan1(
    u: *const __mpz_struct,
    starting_bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    // SAFETY: Caller passes valid readable `u`.
    unsafe { pure_gmp::mpz_scan1(read_mpz(u), starting_bit) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_sizeinbase(u: *const __mpz_struct, base: c_int) -> usize {
    // SAFETY: Caller passes valid readable `u` and `2 <= base <= 62`.
    unsafe { pure_gmp::mpz_sizeinbase(read_mpz(u), base) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_get_str(
    sp: *mut c_char,
    base: c_int,
    u: *const __mpz_struct,
) -> *mut c_char {
    // SAFETY: Formats `u` via `femacs_pure::mini_gmp` and copies NUL-terminated bytes into `sp`
    // (or allocates via registered GMP allocator when `sp` is null).
    unsafe {
        let Some(bytes) = pure_gmp::mpz_get_str_bytes(base, read_mpz(u)) else {
            return core::ptr::null_mut();
        };
        let total = bytes.len() + 1;
        let out = if sp.is_null() {
            (current_alloc_fn())(total).cast::<c_char>()
        } else {
            sp
        };
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), out.cast::<u8>(), bytes.len());
        *out.add(bytes.len()) = 0;
        out
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_set_str(
    r: *mut __mpz_struct,
    sp: *const c_char,
    base: c_int,
) -> c_int {
    // SAFETY: Caller passes NUL-terminated C string `sp` and writable `r`.
    unsafe {
        let bytes = CStr::from_ptr(sp).to_bytes();
        match pure_gmp::mpz_set_str_bytes(bytes, base) {
            Ok(val) => {
                write_mpz(r, val);
                0
            }
            Err(()) => {
                (*r)._mp_size = 0;
                -1
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_import(
    r: *mut __mpz_struct,
    count: usize,
    order: c_int,
    size: usize,
    endian: c_int,
    nails: usize,
    src: *const c_void,
) {
    let total = count.saturating_mul(size);
    // SAFETY: Caller passes readable `src[..count * size]` when `total > 0` and writable `r`.
    unsafe {
        let slice = if total == 0 || src.is_null() {
            &[]
        } else {
            core::slice::from_raw_parts(src.cast::<u8>(), total)
        };
        let val = pure_gmp::mpz_import_bytes(count, order, size, endian, nails, slice);
        write_mpz(r, val);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mpz_export(
    mut r: *mut c_void,
    countp: *mut usize,
    order: c_int,
    size: usize,
    endian: c_int,
    nails: usize,
    u: *const __mpz_struct,
) -> *mut c_void {
    // SAFETY: Exports `u` via `femacs_pure::mini_gmp` into caller buffer `r` (or GMP-allocated buffer when null).
    unsafe {
        let (bytes, count) = pure_gmp::mpz_export_bytes(order, size, endian, nails, read_mpz(u));
        if count > 0 {
            if r.is_null() {
                r = (current_alloc_fn())(bytes.len());
            }
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), r.cast::<u8>(), bytes.len());
        }
        if !countp.is_null() {
            *countp = count;
        }
        r
    }
}

// ---- Safe Rust re-exports & wrappers for the remaining 95 `mini-gmp` functions ----

pub use femacs_pure::mini_gmp::{
    mpn_add, mpn_add_1, mpn_add_n, mpn_addmul_1, mpn_cmp, mpn_com, mpn_copyd, mpn_copyi, mpn_gcd,
    mpn_get_str, mpn_invert_3by2, mpn_lshift, mpn_mul, mpn_mul_1, mpn_mul_n, mpn_neg,
    mpn_perfect_square_p, mpn_popcount, mpn_rshift, mpn_scan0, mpn_scan1, mpn_set_str, mpn_sqr,
    mpn_sqrtrem, mpn_sub, mpn_sub_1, mpn_sub_n, mpn_submul_1, mpn_zero, mpn_zero_p, mpz_2fac_ui,
    mpz_bin_uiui, mpz_clrbit, mpz_cmp_si, mpz_cmpabs_d, mpz_cmpabs_ui, mpz_combit, mpz_congruent_p,
    mpz_divisible_p, mpz_fac_ui, mpz_fits_sshort_p, mpz_fits_uint_p, mpz_fits_ushort_p,
    mpz_gcd_ui, mpz_gcdext, mpz_hamdist, mpz_invert, mpz_lcm, mpz_lcm_ui, mpz_mfac_uiui,
    mpz_mul_si, mpz_perfect_square_p, mpz_perfect_square_root, mpz_powm, mpz_powm_ui,
    mpz_probab_prime_p, mpz_root, mpz_rootrem, mpz_scan0, mpz_setbit, mpz_sqrt, mpz_sqrtrem,
    mpz_submul_ui, mpz_tstbit, mpz_ui_sub,
};

pub fn mpz_init2(bits: u64) -> Mpz {
    let mut z = Mpz::zero();
    z.limbs.reserve(((bits.saturating_sub(1)) as usize / GMP_LIMB_BITS) + 1);
    z
}
pub fn mpz_init_set_si(x: i64) -> Mpz {
    Mpz::from_si(x)
}
pub fn mpz_init_set(x: MpzRef<'_>) -> Mpz {
    x.to_owned_mpz()
}
pub fn mpz_realloc2(z: &mut Mpz, bits: u64) {
    z.limbs.reserve(((bits.saturating_sub(1)) as usize / GMP_LIMB_BITS) + 1);
}
pub fn mpz_limbs_modify(z: &mut Mpz, n: usize) -> &mut [mp_limb_t] {
    if z.limbs.len() < n {
        z.limbs.resize(n, 0);
    }
    &mut z.limbs[..n]
}
pub fn mpz_init_set_d(x: f64) -> Mpz {
    pure_gmp::mpz_from_d(x)
}
pub fn mpz_cdiv_qr(n: MpzRef<'_>, d: MpzRef<'_>) -> (Mpz, Mpz) {
    let (q, r, _) = pure_gmp::mpz_div_qr(n, d, DivRoundMode::Ceil, true, true);
    (q.unwrap(), r.unwrap())
}
pub fn mpz_cdiv_r(n: MpzRef<'_>, d: MpzRef<'_>) -> Mpz {
    pure_gmp::mpz_div_qr(n, d, DivRoundMode::Ceil, false, true).1.unwrap()
}
pub fn mpz_mod(n: MpzRef<'_>, d: MpzRef<'_>) -> Mpz {
    let mode = if !d.neg {
        DivRoundMode::Floor
    } else {
        DivRoundMode::Ceil
    };
    pure_gmp::mpz_div_qr(n, d, mode, false, true).1.unwrap()
}
pub fn mpz_cdiv_q_2exp(u: MpzRef<'_>, cnt: u64) -> Mpz {
    pure_gmp::mpz_div_q_2exp(u, cnt, DivRoundMode::Ceil)
}
pub fn mpz_tdiv_q_2exp(u: MpzRef<'_>, cnt: u64) -> Mpz {
    pure_gmp::mpz_div_q_2exp(u, cnt, DivRoundMode::Trunc)
}
pub fn mpz_cdiv_r_2exp(u: MpzRef<'_>, cnt: u64) -> Mpz {
    pure_gmp::mpz_div_r_2exp(u, cnt, DivRoundMode::Ceil)
}
pub fn mpz_fdiv_r_2exp(u: MpzRef<'_>, cnt: u64) -> Mpz {
    pure_gmp::mpz_div_r_2exp(u, cnt, DivRoundMode::Floor)
}
pub fn mpz_tdiv_r_2exp(u: MpzRef<'_>, cnt: u64) -> Mpz {
    pure_gmp::mpz_div_r_2exp(u, cnt, DivRoundMode::Trunc)
}
pub fn mpz_cdiv_qr_ui(n: MpzRef<'_>, d: u64) -> (Mpz, Mpz, u64) {
    let (q, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Ceil, true, true);
    (q.unwrap(), r.unwrap(), rem)
}
pub fn mpz_fdiv_qr_ui(n: MpzRef<'_>, d: u64) -> (Mpz, Mpz, u64) {
    let (q, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Floor, true, true);
    (q.unwrap(), r.unwrap(), rem)
}
pub fn mpz_tdiv_qr_ui(n: MpzRef<'_>, d: u64) -> (Mpz, Mpz, u64) {
    let (q, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Trunc, true, true);
    (q.unwrap(), r.unwrap(), rem)
}
pub fn mpz_cdiv_q_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (q, _, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Ceil, true, false);
    (q.unwrap(), rem)
}
pub fn mpz_tdiv_q_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (q, _, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Trunc, true, false);
    (q.unwrap(), rem)
}
pub fn mpz_cdiv_r_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (_, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Ceil, false, true);
    (r.unwrap(), rem)
}
pub fn mpz_fdiv_r_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (_, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Floor, false, true);
    (r.unwrap(), rem)
}
pub fn mpz_tdiv_r_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (_, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Trunc, false, true);
    (r.unwrap(), rem)
}
pub fn mpz_cdiv_ui(n: MpzRef<'_>, d: u64) -> u64 {
    pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Ceil, false, false).2
}
pub fn mpz_fdiv_ui(n: MpzRef<'_>, d: u64) -> u64 {
    pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Floor, false, false).2
}
pub fn mpz_mod_ui(n: MpzRef<'_>, d: u64) -> (Mpz, u64) {
    let (_, r, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Floor, false, true);
    (r.unwrap(), rem)
}
pub fn mpz_divexact_ui(n: MpzRef<'_>, d: u64) -> Mpz {
    let (q, _, rem) = pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Trunc, true, false);
    assert_eq!(rem, 0);
    q.unwrap()
}
pub fn mpz_divisible_ui_p(n: MpzRef<'_>, d: u64) -> i32 {
    i32::from(pure_gmp::mpz_div_qr_ui(n, d, DivRoundMode::Trunc, false, false).2 == 0)
}
pub fn mpz_init_setbit(b: u64) -> Mpz {
    let mut z = Mpz::zero();
    pure_gmp::mpz_setbit(&mut z, b);
    z
}
pub fn mpz_init_set_str(bytes: &[u8], base: i32) -> Result<Mpz, ()> {
    pure_gmp::mpz_set_str_bytes(bytes, base)
}
pub fn mpz_out_str(out: &mut impl std::io::Write, base: i32, x: MpzRef<'_>) -> usize {
    let Some(bytes) = pure_gmp::mpz_get_str_bytes(base, x) else {
        return 0;
    };
    out.write(&bytes).unwrap_or(0)
}
