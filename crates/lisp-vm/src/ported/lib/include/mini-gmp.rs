// femacs: port of lib/mini-gmp.h @ fd7afced403.
//! GMP/mini-gmp ABI struct definitions and active `mpz_*` boundary declarations.

use super::*;
pub const __MINI_GMP_H__: bool = true;

#[cfg (not (target_os = "windows"))]
pub type mp_limb_t = c_ulong;
#[cfg (not (target_os = "windows"))]
pub type mp_size_t = c_long;
#[cfg (not (target_os = "windows"))]
pub type mp_bitcnt_t = c_ulong;
#[cfg (target_os = "windows")]
pub type mp_limb_t = u64;
#[cfg (target_os = "windows")]
pub type mp_size_t = isize;
#[cfg (target_os = "windows")]
pub type mp_bitcnt_t = u64;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct __mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut mp_limb_t,
}

pub type mpz_t = [__mpz_struct; 1];
pub type mpz_ptr = *mut __mpz_struct;
pub type mpz_srcptr = *const __mpz_struct;

macro_rules! mpn_invert_limb {
    ($x:expr) => {
        crate::sys::mini_gmp::mpn_invert_3by2($x, 0)
    };
}

macro_rules! mpz_odd_p {
    ($z:expr) => {{
        // SAFETY: Caller passes a valid `__mpz_struct` pointer; `_mp_d` is readable when `_mp_size != 0`.
        unsafe {
            if (*$z)._mp_size != 0 {
                (*(*$z)._mp_d & 1) as c_int
            } else {
                0
            }
        }
    }};
}

macro_rules! mpz_even_p {
    ($z:expr) => {
        (!(mpz_odd_p!($z) != 0)) as c_int
    };
}

macro_rules! MPZ_ROINIT_N {
    ($xp:expr, $xs:expr) => {
        __mpz_struct {
            _mp_alloc: 0,
            _mp_size: $xs,
            _mp_d: $xp,
        }
    };
}

pub mod gmp {
    use super::*;

    unsafe extern "C-unwind" {
        pub static mp_bits_per_limb: c_int;
        pub fn mpz_init(a: *mut __mpz_struct);
        pub fn mpz_clear(a: *mut __mpz_struct);
        pub fn mpz_sgn(a: *const __mpz_struct) -> c_int;
        pub fn mpz_cmp_ui(a: *const __mpz_struct, b: c_ulong) -> c_int;
        pub fn mpz_cmp(a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;
        pub fn mpz_cmpabs(a: *const __mpz_struct, b: *const __mpz_struct) -> c_int;
        pub fn mpz_cmp_d(a: *const __mpz_struct, b: c_double) -> c_int;
        pub fn mpz_abs(a: *mut __mpz_struct, b: *const __mpz_struct);
        pub fn mpz_neg(a: *mut __mpz_struct, b: *const __mpz_struct);
        pub fn mpz_swap(a: *mut __mpz_struct, b: *mut __mpz_struct);
        pub fn mpz_add_ui(a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
        pub fn mpz_add(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_sub_ui(a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
        pub fn mpz_sub(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_mul_ui(a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
        pub fn mpz_mul(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_mul_2exp(a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
        pub fn mpz_addmul_ui(a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
        pub fn mpz_addmul(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_submul(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_fdiv_qr(
            a: *mut __mpz_struct,
            b: *mut __mpz_struct,
            c: *const __mpz_struct,
            d: *const __mpz_struct,
        );
        pub fn mpz_tdiv_qr(
            a: *mut __mpz_struct,
            b: *mut __mpz_struct,
            c: *const __mpz_struct,
            d: *const __mpz_struct,
        );
        pub fn mpz_cdiv_q(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_fdiv_q(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_tdiv_q(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_fdiv_r(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_tdiv_r(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_fdiv_q_2exp(a: *mut __mpz_struct, b: *const __mpz_struct, c: mp_bitcnt_t);
        pub fn mpz_divexact(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_fdiv_q_ui(
            a: *mut __mpz_struct,
            b: *const __mpz_struct,
            c: c_ulong,
        ) -> c_ulong;
        pub fn mpz_tdiv_ui(a: *const __mpz_struct, b: c_ulong) -> c_ulong;
        pub fn mpz_gcd(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_pow_ui(a: *mut __mpz_struct, b: *const __mpz_struct, c: c_ulong);
        pub fn mpz_ui_pow_ui(a: *mut __mpz_struct, b: c_ulong, c: c_ulong);
        pub fn mpz_com(a: *mut __mpz_struct, b: *const __mpz_struct);
        pub fn mpz_and(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_ior(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_xor(a: *mut __mpz_struct, b: *const __mpz_struct, c: *const __mpz_struct);
        pub fn mpz_popcount(a: *const __mpz_struct) -> mp_bitcnt_t;
        pub fn mpz_scan1(a: *const __mpz_struct, b: mp_bitcnt_t) -> mp_bitcnt_t;
        pub fn mpz_fits_slong_p(a: *const __mpz_struct) -> c_int;
        pub fn mpz_fits_ulong_p(a: *const __mpz_struct) -> c_int;
        pub fn mpz_fits_sint_p(a: *const __mpz_struct) -> c_int;
        pub fn mpz_get_si(a: *const __mpz_struct) -> c_long;
        pub fn mpz_get_ui(a: *const __mpz_struct) -> c_ulong;
        pub fn mpz_get_d(a: *const __mpz_struct) -> c_double;
        pub fn mpz_size(a: *const __mpz_struct) -> usize;
        pub fn mpz_getlimbn(a: *const __mpz_struct, b: mp_size_t) -> mp_limb_t;
        pub fn mpz_limbs_read(a: *const __mpz_struct) -> *const mp_limb_t;
        pub fn mpz_limbs_write(a: *mut __mpz_struct, b: mp_size_t) -> *mut mp_limb_t;
        pub fn mpz_limbs_finish(a: *mut __mpz_struct, b: mp_size_t);
        pub fn mpz_roinit_n(
            a: *mut __mpz_struct,
            b: *const mp_limb_t,
            c: mp_size_t,
        ) -> *const __mpz_struct;
        pub fn mpz_set_si(a: *mut __mpz_struct, b: c_long);
        pub fn mpz_set_ui(a: *mut __mpz_struct, b: c_ulong);
        pub fn mpz_set(a: *mut __mpz_struct, b: *const __mpz_struct);
        pub fn mpz_set_d(a: *mut __mpz_struct, b: c_double);
        pub fn mpz_init_set_ui(a: *mut __mpz_struct, b: c_ulong);
        pub fn mpz_sizeinbase(a: *const __mpz_struct, b: c_int) -> usize;
        pub fn mpz_get_str(a: *mut c_char, b: c_int, c: *const __mpz_struct) -> *mut c_char;
        pub fn mpz_set_str(a: *mut __mpz_struct, b: *const c_char, c: c_int) -> c_int;
        pub fn mpz_import(
            a: *mut __mpz_struct,
            b: usize,
            c: c_int,
            d: usize,
            e: c_int,
            f: usize,
            g: *const c_void,
        );
        pub fn mpz_export(
            a: *mut c_void,
            b: *mut usize,
            c: c_int,
            d: usize,
            e: c_int,
            f: usize,
            g: *const __mpz_struct,
        ) -> *mut c_void;
        pub fn mp_set_memory_functions(
            a: unsafe extern "C-unwind" fn(usize) -> *mut c_void,
            b: unsafe extern "C-unwind" fn(*mut c_void, usize, usize) -> *mut c_void,
            c: unsafe extern "C-unwind" fn(*mut c_void, usize),
        );
    }
}

pub use self::gmp::*;
