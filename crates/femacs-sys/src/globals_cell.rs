//! Transparent interior-mutable wrapper for process-global VM and C-ABI static storage (`U6`).
//!
//! `VmGlobal<T>` is `#[repr(transparent)]` over `core::cell::UnsafeCell<T>`, guaranteeing:
//! - Identical size, alignment, symbol address, and mutable `.data`/`.bss`/`__DATA,subrs` section
//!   placement as C globals (preserving `.pdmp` hot/cold relocations, `.eln` ABI, and `nm` checks).
//! - Elimination of Rust `static mut` (`&mut` aliasing hazards) across active crates.
//! - Lock-free, provenance-preserving `Relaxed` atomic `.get()` / `.set()` accessors for primitive
//!   scalars and raw pointers (including tagged `Lisp_Object` values).

#![deny(unsafe_op_in_unsafe_fn, static_mut_refs)]

use core::cell::UnsafeCell;
use core::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};

#[repr(transparent)]
pub struct VmGlobal<T>(UnsafeCell<T>);

// SAFETY: `VmGlobal<T>` backs single-VM / C-ABI static storage verified unique by `nm` gates;
// non-atomic compound accesses occur under the VM global lock or single-threaded init/pdump phases,
// while scalar/pointer `.get()` and `.set()` execute hardware-atomic relaxed loads and stores.
unsafe impl<T> Sync for VmGlobal<T> {}

impl<T> VmGlobal<T> {
    #[inline(always)]
    pub const fn new(val: T) -> Self {
        Self(UnsafeCell::new(val))
    }

    #[inline(always)]
    pub const fn as_mut_ptr(&self) -> *mut T {
        self.0.get()
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.0.get() as *const T
    }
}

impl VmGlobal<bool> {
    #[inline(always)]
    pub fn get(&self) -> bool {
        const {
            assert!(core::mem::size_of::<bool>() == core::mem::size_of::<AtomicBool>());
            assert!(core::mem::align_of::<bool>() == core::mem::align_of::<AtomicBool>());
        }
        // SAFETY: `self.0.get()` is non-null, properly aligned for `AtomicBool`, and points to
        // live static `bool` storage initialized to `0` or `1`.
        unsafe { AtomicBool::from_ptr(self.0.get()).load(Ordering::Relaxed) }
    }

    #[inline(always)]
    pub fn set(&self, val: bool) {
        const {
            assert!(core::mem::size_of::<bool>() == core::mem::size_of::<AtomicBool>());
            assert!(core::mem::align_of::<bool>() == core::mem::align_of::<AtomicBool>());
        }
        // SAFETY: `self.0.get()` is non-null, properly aligned for `AtomicBool`, and points to
        // live mutable static storage.
        unsafe { AtomicBool::from_ptr(self.0.get()).store(val, Ordering::Relaxed) }
    }
}

macro_rules! impl_vm_global_int {
    ($($ty:ty => $atomic:ty),* $(,)?) => {
        $(
            impl VmGlobal<$ty> {
                #[inline(always)]
                pub fn get(&self) -> $ty {
                    const {
                        assert!(core::mem::size_of::<$ty>() == core::mem::size_of::<$atomic>());
                        assert!(core::mem::align_of::<$ty>() >= core::mem::align_of::<$atomic>());
                    }
                    // SAFETY: `self.0.get()` is non-null, aligned to `$atomic`, and backed by
                    // live static storage in `.data`/`.bss`.
                    unsafe { <$atomic>::from_ptr(self.0.get()).load(Ordering::Relaxed) }
                }

                #[inline(always)]
                pub fn set(&self, val: $ty) {
                    const {
                        assert!(core::mem::size_of::<$ty>() == core::mem::size_of::<$atomic>());
                        assert!(core::mem::align_of::<$ty>() >= core::mem::align_of::<$atomic>());
                    }
                    // SAFETY: `self.0.get()` is non-null, aligned to `$atomic`, and backed by
                    // live mutable static storage in `.data`/`.bss`.
                    unsafe { <$atomic>::from_ptr(self.0.get()).store(val, Ordering::Relaxed) }
                }

                #[inline(always)]
                pub fn wrapping_add(&self, delta: $ty) -> $ty {
                    const {
                        assert!(core::mem::size_of::<$ty>() == core::mem::size_of::<$atomic>());
                        assert!(core::mem::align_of::<$ty>() >= core::mem::align_of::<$atomic>());
                    }
                    // SAFETY: `self.0.get()` is non-null, aligned to `$atomic`, and backed by
                    // live mutable static storage in `.data`/`.bss`.
                    unsafe { <$atomic>::from_ptr(self.0.get()).fetch_add(delta, Ordering::Relaxed).wrapping_add(delta) }
                }
            }
        )*
    };
}

impl_vm_global_int! {
    i8 => AtomicI8,
    u8 => AtomicU8,
    i16 => AtomicI16,
    u16 => AtomicU16,
    i32 => AtomicI32,
    u32 => AtomicU32,
    i64 => AtomicI64,
    u64 => AtomicU64,
    isize => AtomicIsize,
    usize => AtomicUsize,
}

impl VmGlobal<f64> {
    #[inline(always)]
    pub fn get(&self) -> f64 {
        const {
            assert!(core::mem::size_of::<f64>() == core::mem::size_of::<AtomicU64>());
            assert!(core::mem::align_of::<f64>() >= core::mem::align_of::<AtomicU64>());
        }
        // SAFETY: `f64` and `AtomicU64` have identical 8-byte size and alignment on 64-bit targets.
        let bits = unsafe { AtomicU64::from_ptr(self.0.get() as *mut u64).load(Ordering::Relaxed) };
        f64::from_bits(bits)
    }

    #[inline(always)]
    pub fn set(&self, val: f64) {
        const {
            assert!(core::mem::size_of::<f64>() == core::mem::size_of::<AtomicU64>());
            assert!(core::mem::align_of::<f64>() >= core::mem::align_of::<AtomicU64>());
        }
        // SAFETY: `f64` and `AtomicU64` have identical 8-byte size and alignment on 64-bit targets.
        unsafe { AtomicU64::from_ptr(self.0.get() as *mut u64).store(val.to_bits(), Ordering::Relaxed) }
    }
}

impl<P> VmGlobal<*mut P> {
    #[inline(always)]
    pub fn get(&self) -> *mut P {
        const {
            assert!(core::mem::size_of::<*mut P>() == core::mem::size_of::<AtomicPtr<P>>());
            assert!(core::mem::align_of::<*mut P>() == core::mem::align_of::<AtomicPtr<P>>());
        }
        // SAFETY: `AtomicPtr::from_ptr` preserves pointer provenance (including tagged `Lisp_Object`
        // words) and performs an atomic word load without dereferencing the pointee `P`.
        unsafe { AtomicPtr::from_ptr(self.0.get()).load(Ordering::Relaxed) }
    }

    #[inline(always)]
    pub fn set(&self, val: *mut P) {
        const {
            assert!(core::mem::size_of::<*mut P>() == core::mem::size_of::<AtomicPtr<P>>());
            assert!(core::mem::align_of::<*mut P>() == core::mem::align_of::<AtomicPtr<P>>());
        }
        // SAFETY: `AtomicPtr::from_ptr` preserves pointer provenance (including tagged `Lisp_Object`
        // words) and performs an atomic word store without dereferencing the pointee `P`.
        unsafe { AtomicPtr::from_ptr(self.0.get()).store(val, Ordering::Relaxed) }
    }
}

impl<P> VmGlobal<*const P> {
    #[inline(always)]
    pub fn get(&self) -> *const P {
        const {
            assert!(core::mem::size_of::<*const P>() == core::mem::size_of::<AtomicPtr<P>>());
            assert!(core::mem::align_of::<*const P>() == core::mem::align_of::<AtomicPtr<P>>());
        }
        // SAFETY: `*const P` and `*mut P` have identical representation; atomic load preserves provenance.
        unsafe { AtomicPtr::from_ptr(self.0.get() as *mut *mut P).load(Ordering::Relaxed) as *const P }
    }

    #[inline(always)]
    pub fn set(&self, val: *const P) {
        const {
            assert!(core::mem::size_of::<*const P>() == core::mem::size_of::<AtomicPtr<P>>());
            assert!(core::mem::align_of::<*const P>() == core::mem::align_of::<AtomicPtr<P>>());
        }
        // SAFETY: `*const P` and `*mut P` have identical representation; atomic store preserves provenance.
        unsafe { AtomicPtr::from_ptr(self.0.get() as *mut *mut P).store(val as *mut P, Ordering::Relaxed) }
    }
}

#[cfg(test)]
mod tests {
    use super::VmGlobal;
    use core::mem::{align_of, size_of};

    static G_BOOL: VmGlobal<bool> = VmGlobal::new(false);
    static G_I64: VmGlobal<i64> = VmGlobal::new(-7);
    static G_ISIZE: VmGlobal<isize> = VmGlobal::new(0);
    static G_F64: VmGlobal<f64> = VmGlobal::new(1.5);
    static G_MUT_PTR: VmGlobal<*mut u8> = VmGlobal::new(core::ptr::null_mut());
    static G_CONST_PTR: VmGlobal<*const u8> = VmGlobal::new(core::ptr::null());

    #[test]
    fn repr_transparent_layout_and_pointer_identity() {
        assert_eq!(size_of::<VmGlobal<bool>>(), size_of::<bool>());
        assert_eq!(align_of::<VmGlobal<bool>>(), align_of::<bool>());
        assert_eq!(size_of::<VmGlobal<i64>>(), size_of::<i64>());
        assert_eq!(align_of::<VmGlobal<i64>>(), align_of::<i64>());
        assert_eq!(size_of::<VmGlobal<f64>>(), size_of::<f64>());
        assert_eq!(align_of::<VmGlobal<f64>>(), align_of::<f64>());
        assert_eq!(size_of::<VmGlobal<*mut u8>>(), size_of::<*mut u8>());
        assert_eq!(align_of::<VmGlobal<*mut u8>>(), align_of::<*mut u8>());
        assert_eq!(size_of::<VmGlobal<[u64; 4]>>(), size_of::<[u64; 4]>());
        assert_eq!(align_of::<VmGlobal<[u64; 4]>>(), align_of::<[u64; 4]>());

        assert_eq!(
            G_I64.as_ptr() as usize,
            (&raw const G_I64) as usize,
            "VmGlobal::as_ptr must equal the static symbol base address"
        );
        assert_eq!(G_I64.as_mut_ptr() as usize, G_I64.as_ptr() as usize);
    }

    #[test]
    fn atomic_scalar_and_float_accessors_preserve_bits_and_wrap() {
        assert!(!G_BOOL.get());
        G_BOOL.set(true);
        assert!(G_BOOL.get());
        G_BOOL.set(false);

        assert_eq!(G_I64.get(), -7);
        assert_eq!(G_I64.wrapping_add(10), 3);
        assert_eq!(G_I64.get(), 3);
        G_I64.set(i64::MAX);
        assert_eq!(G_I64.wrapping_add(1), i64::MIN);
        G_I64.set(-7);

        G_ISIZE.set(41);
        assert_eq!(G_ISIZE.wrapping_add(1), 42);
        G_ISIZE.set(0);

        G_F64.set(-0.0_f64);
        assert_eq!(G_F64.get().to_bits(), (-0.0_f64).to_bits());
        let nan = f64::from_bits(0x7ff8_0000_0000_1234);
        G_F64.set(nan);
        assert_eq!(G_F64.get().to_bits(), nan.to_bits());
        G_F64.set(1.5);
    }

    #[test]
    fn atomic_pointer_accessors_preserve_tagged_addresses() {
        let mut slot = 99u8;
        let p = &raw mut slot;
        G_MUT_PTR.set(p);
        assert_eq!(G_MUT_PTR.get(), p);
        // SAFETY: `p` points to live stack `slot` in this frame.
        assert_eq!(unsafe { *G_MUT_PTR.get() }, 99);

        let tagged = core::ptr::without_provenance_mut::<u8>(0x1234_5678_0000_0003);
        G_MUT_PTR.set(tagged);
        assert_eq!(G_MUT_PTR.get() as usize, 0x1234_5678_0000_0003);
        G_MUT_PTR.set(core::ptr::null_mut());

        let cp: *const u8 = &raw const slot;
        G_CONST_PTR.set(cp);
        assert_eq!(G_CONST_PTR.get(), cp);
        G_CONST_PTR.set(core::ptr::null());
    }
}
