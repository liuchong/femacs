// Actual-source component adapter skeleton. tools/runtime-components.py copies
// this file to an owned build tree and includes mechanically generated copies of
// the two actual runtime modules with appended private-state wrappers.
#![allow(dead_code, non_snake_case, non_camel_case_types, unused_imports, unused_variables)]

use serde_json::{json, Value};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};

mod actual {
    use super::*;
    use serde_json::{json, Value};
    use core::ffi::{c_char, c_int, c_void};
    use std::ptr::null_mut;

    pub type Lisp_Object = u64;
    pub type uintmax_t = u64;
    pub const QNIL: Lisp_Object = 0;
    pub const QT: Lisp_Object = 1;
    pub fn Qnil() -> Lisp_Object { QNIL }
    pub fn Qt() -> Lisp_Object { QT }
    pub const fn lisp_h_Qnil() -> Lisp_Object { QNIL }
    pub fn NILP(value: Lisp_Object) -> bool { value == QNIL }
    pub fn EQ(left: Lisp_Object, right: Lisp_Object) -> bool { left == right }
    pub fn XLI(value: Lisp_Object) -> i64 { value as i64 }
    pub fn XPNTR(value: Lisp_Object) -> *mut c_void { (value & !7u64) as *mut c_void }
    pub fn CONSP(value: Lisp_Object) -> bool { value & 7 == 3 }
    pub fn dead_object() -> Lisp_Object { 0x0DED }
    pub fn lisp_h_XCONS_untracked(value: Lisp_Object) -> *mut Lisp_Cons {
        XPNTR(value) as *mut Lisp_Cons
    }
    pub fn make_lisp_symbol(value: usize) -> Lisp_Object {
        ((value as u64) << 3) | 4
    }
    pub fn make_cons_ptr(value: *mut Lisp_Cons) -> Lisp_Object {
        (value as u64) | 3
    }
    pub fn pdumper_object_p(_value: *mut c_void) -> bool { false }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Lisp_Cons_s_u { pub cdr: Lisp_Object, pub chain: *mut Lisp_Cons }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Lisp_Cons_s { pub car: Lisp_Object, pub u: Lisp_Cons_s_u }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union Lisp_Cons_u { pub s: Lisp_Cons_s }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Lisp_Cons { pub u: Lisp_Cons_u }

    // @actual-layout@
    // Target configuration primitive; bitmap aliases/constants above come
    // directly from the production header rather than host substitutions.
    pub const SIZE_WIDTH: usize = usize::BITS as usize;
    pub const memory_full_cons_threshold: usize = 32752;

    const _: () = assert!(core::mem::size_of::<cons_block>() == 32752);
    const _: () = assert!(core::mem::offset_of!(cons_block, gcmarkbits) == 31968);
    const _: () = assert!(core::mem::offset_of!(cons_block, oldbits) == 32224);
    const _: () = assert!(core::mem::offset_of!(cons_block, dirtybits) == 32480);
    const _: () = assert!(core::mem::offset_of!(cons_block, raw_tainted) == 32736);
    const _: () = assert!(core::mem::offset_of!(cons_block, next) == 32744);

    pub static mut cons_block: *mut cons_block = null_mut();
    pub static mut cons_block_index: i32 = CONS_BLOCK_SIZE as i32;

    macro_rules! CONS_BLOCK { ($fptr:expr) => {
        (($fptr as usize) & !(BLOCK_ALIGN - 1)) as *mut cons_block
    } }
    macro_rules! CONS_INDEX { ($fptr:expr) => {
        (((($fptr as usize) & (BLOCK_ALIGN - 1)) / core::mem::size_of::<Lisp_Cons>()) as usize)
    } }
    macro_rules! MEM_NIL { () => { mem_nil_ptr() } }
    macro_rules! eassert { ($condition:expr) => { if !($condition) { panic!("eassert failed") } } }
    pub fn ASET(_vector: Lisp_Object, _index: isize, _value: Lisp_Object) {
        panic!("Lisp vector export is outside the isolated numeric component");
    }
    pub fn AREF(_vector: Lisp_Object, _index: isize) -> Lisp_Object {
        panic!("Lisp vector export is outside the isolated numeric component");
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum mem_type {
        MEM_TYPE_NON_LISP, MEM_TYPE_CONS, MEM_TYPE_STRING, MEM_TYPE_SYMBOL,
        MEM_TYPE_FLOAT, MEM_TYPE_VECTORLIKE, MEM_TYPE_VECTOR_BLOCK, MEM_TYPE_SPARE,
    }
    #[repr(C)]
    pub struct mem_node {
        pub start: *mut c_void, pub end: *mut c_void, pub r#type: mem_type,
    }
    pub static mut MEM_Z: mem_node = mem_node { start: null_mut(), end: null_mut(), r#type: mem_type::MEM_TYPE_NON_LISP };
    pub static mut MEM_NODE: mem_node = mem_node { start: null_mut(), end: null_mut(), r#type: mem_type::MEM_TYPE_CONS };
    pub fn mem_nil_ptr() -> *mut mem_node { &raw mut MEM_Z }
    pub unsafe fn mem_find(pointer: *mut c_void) -> *mut mem_node {
        let address = pointer as usize;
        let start = MEM_NODE.start as usize;
        let end = MEM_NODE.end as usize;
        if start <= address && address < end { &raw mut MEM_NODE } else { mem_nil_ptr() }
    }

    #[repr(C)]
    pub struct Lisp_Finalizer { pub prev: *mut Lisp_Finalizer, pub next: *mut Lisp_Finalizer }
    pub static mut finalizers: Lisp_Finalizer = Lisp_Finalizer { prev: null_mut(), next: null_mut() };
    pub static mut doomed_finalizers: Lisp_Finalizer = Lisp_Finalizer { prev: null_mut(), next: null_mut() };
    pub static mut DUMMY_FINALIZER: Lisp_Finalizer = Lisp_Finalizer { prev: null_mut(), next: null_mut() };

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum GcRequest { Full, Automatic }

    #[repr(C)]
    pub struct timespec { pub tv_sec: i64, pub tv_nsec: i64 }
    pub static CURRENT_THREAD: AtomicBool = AtomicBool::new(true);
    #[no_mangle]
    pub extern "C" fn in_current_thread() -> bool { CURRENT_THREAD.load(Ordering::Relaxed) }
    #[no_mangle]
    pub unsafe extern "C" fn clock_gettime(_clock: c_int, value: *mut timespec) -> c_int {
        if !value.is_null() { (*value).tv_sec = 0; (*value).tv_nsec = 0; }
        0
    }

    pub fn getenv(name: *const c_char) -> *mut c_char {
        let name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned();
        match std::env::var(&name) {
            Ok(value) => CString::new(value).map(|value| value.into_raw()).unwrap_or(null_mut()),
            Err(_) => null_mut(),
        }
    }
    pub fn fatal(message: *const c_char) -> ! {
        let message = unsafe { CStr::from_ptr(message) }.to_string_lossy();
        panic!("fatal: {message}")
    }
    pub fn error(message: *const c_char) -> ! {
        let message = unsafe { CStr::from_ptr(message) }.to_string_lossy();
        panic!("error: {message}")
    }

    #[repr(C)]
    pub struct GcStat { pub total_conses: i64 }
    pub static mut gcstat: GcStat = GcStat { total_conses: 0 };
    pub static mut Vmemory_full: Lisp_Object = QNIL;
    pub static mut Vpurify_flag: Lisp_Object = QNIL;
    pub fn total_bytes_of_live_objects() -> i64 { 0 }
    pub static mut MARK_HISTORY: Vec<Lisp_Object> = Vec::new();
    pub fn mark_object(value: Lisp_Object) { unsafe { MARK_HISTORY.push(value); } }
    pub fn mark_objects(_values: *mut Lisp_Object, _count: isize) {}
    pub fn make_vector(_size: isize, _init: Lisp_Object) -> Lisp_Object { QNIL }
    pub fn make_fixnum(_value: i64) -> Lisp_Object { QNIL }
    pub fn make_uint(_value: u64) -> Lisp_Object { QNIL }
    pub fn SPECPDL_INDEX() -> usize { 0 }
    pub fn unbind_to(_count: usize, value: Lisp_Object) -> Lisp_Object { value }
    pub fn record_unwind_protect_ptr(_function: Option<unsafe extern "C" fn(*mut c_void)>, _data: *mut c_void) {}
    pub fn record_unwind_protect_intmax(_function: Option<unsafe extern "C" fn(i64)>, _value: i64) {}
    pub fn record_unwind_protect_ptr_mark(_function: Option<unsafe extern "C" fn(*mut c_void)>, _data: *mut c_void, _mark: Option<unsafe extern "C" fn(*mut c_void)>) {}

    pub unsafe fn reset_test_block() {
        // Mach-O may lower large static alignment to the segment maximum.
        // Use the real required allocator alignment instead of a static that
        // can silently violate CONS_BLOCK pointer masking.
        let layout = std::alloc::Layout::from_size_align(core::mem::size_of::<cons_block>(), BLOCK_ALIGN).expect("block layout");
        let block = std::alloc::alloc_zeroed(layout) as *mut cons_block;
        assert!(!block.is_null());
        (*block).gcmarkbits = [0; 32];
        (*block).oldbits = [0; 32];
        (*block).dirtybits = [0; 32];
        (*block).raw_tainted = 0;
        (*block).next = null_mut();
        for cell in &mut (*block).conses {
            cell.u.s.car = 0;
            cell.u.s.u.cdr = 0;
        }
        cons_block = block;
        cons_block_index = CONS_BLOCK_SIZE as i32;
        MEM_NODE.start = block as *mut c_void;
        MEM_NODE.end = (block as *mut u8).add(core::mem::size_of::<cons_block>()) as *mut c_void;
        MEM_NODE.r#type = mem_type::MEM_TYPE_CONS;
    }
    pub unsafe fn test_cell(index: usize) -> *mut Lisp_Cons {
        &raw mut (*cons_block).conses[index]
    }
    pub fn test_object(index: usize) -> Lisp_Object {
        unsafe { make_cons_ptr(test_cell(index)) }
    }
    pub unsafe fn test_mark(index: usize) { (*cons_block).gcmarkbits[index / BITS_PER_BITS_WORD] |= (1 as bits_word) << (index % BITS_PER_BITS_WORD); }
    pub unsafe fn test_old_indices() -> Vec<u64> {
        let mut result = Vec::new();
        for word in 0..32 { let mut bits = (*cons_block).oldbits[word]; while bits != 0 { let bit = bits.trailing_zeros(); result.push((word * 64 + bit as usize) as u64); bits &= bits - 1; } }
        result
    }
    pub unsafe fn test_dirty_indices() -> Vec<u64> {
        let mut result = Vec::new();
        for word in 0..32 { let mut bits = (*cons_block).dirtybits[word]; while bits != 0 { let bit = bits.trailing_zeros(); result.push((word * 64 + bit as usize) as u64); bits &= bits - 1; } }
        result
    }
    pub unsafe fn test_marked_count() -> u64 { (*cons_block).gcmarkbits.iter().map(|word| word.count_ones() as u64).sum() }
    pub unsafe fn test_old_count() -> u64 { (*cons_block).oldbits.iter().map(|word| word.count_ones() as u64).sum() }
    pub unsafe fn test_dirty_count() -> u64 { (*cons_block).dirtybits.iter().map(|word| word.count_ones() as u64).sum() }

    include!("actual_observation.rs");
    include!("actual_collector.rs");
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union vectorlike_header { pub size: isize }
    #[repr(C)]
    pub struct Lisp_Vector { pub header: vectorlike_header }
    pub enum Lisp_Type { Lisp_Vectorlike = 5 }
    #[derive(PartialEq, Eq)]
    pub enum pvec_type { PVEC_BOOL_VECTOR, Other }
    pub fn PSEUDOVECTOR_TYPE(_vector: *mut Lisp_Vector) -> pvec_type { pvec_type::Other }
    pub fn pdumper_set_marked(_pointer: *mut c_void) { panic!("cold marking is outside host component") }
    pub fn make_lisp_ptr(pointer: *mut c_void, tag: Lisp_Type) -> Lisp_Object { pointer as u64 | tag as u64 }
    // @actual-vector-mark@

    pub fn dispatch(request: &Value) -> Result<Value, String> {
        let component = request.get("component").and_then(Value::as_str).ok_or("missing component")?;
        let steps = request.get("steps").and_then(Value::as_array).cloned().unwrap_or_default();
        let inputs = request.get("inputs").and_then(Value::as_array).cloned().unwrap_or_default();
        let observations = match component {
            "observer" => gc_observation::component_run(&steps, &inputs)?,
            "collector" => gc_collector::component_run(&steps, &inputs)?,
            _ => return Err("unknown component".into()),
        };
        Ok(json!({
            "schema": 1,
            "case_id": request.get("id").and_then(Value::as_str).unwrap_or(""),
            "provider": "actual-source-components",
            "sources": {
                "src/runtime_gc_observation.rs": env!("ADAPTER_OBSERVATION_SHA256"),
                "src/runtime_gc_collector.rs": env!("ADAPTER_COLLECTOR_SHA256"),
            },
            "consumed_steps": steps.len(),
            "consumed_inputs": inputs.len(),
            "observations": observations,
        }))
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("stdin");
    let request: Value = serde_json::from_str(&input).expect("JSON request");
    match actual::dispatch(&request) {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON result")),
        Err(error) => { eprintln!("component adapter: {error}"); std::process::exit(2); }
    }
}
