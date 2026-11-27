// Private, opt-in GC observation. See PORTING.md P-346 and the runtime manual.
// Mounted as `mod gc_observation` by alloc.rs. Numeric storage never holds a Lisp root.
use super::*;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};

    pub(super) mod numeric {
        pub const CAPACITY: usize = 4096;
        pub const DEPTH: usize = 16;
        pub const MAP_CAPACITY: usize = 262144;
        const PROBES: usize = 1024;
        const ABSENT: usize = usize::MAX;
        pub const COMPLETE: u32 = 1;
        pub const CLOCK: u32 = 2;
        pub const INCOMPLETE: u32 = 4;
        pub const EXPORT: u32 = 8;
        pub const DEPTH_ERROR: u32 = 16;
        pub const COUNTER: u32 = 64;
        pub const DROPPED: u32 = 128;

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct GcPhaseRecord {
            pub cycle: u64,
            pub parent_cycle: u64,
            pub phase: u32,
            pub flags: u32,
            pub start_ns: u64,
            pub end_ns: u64,
        }
        impl GcPhaseRecord {
            pub const ZERO: Self = Self {
                cycle: 0, parent_cycle: 0, phase: 0, flags: 0,
                start_ns: 0, end_ns: 0,
            };
        }
        const _: () = {
            assert!(core::mem::size_of::<GcPhaseRecord>() == 40);
            assert!(core::mem::offset_of!(GcPhaseRecord, phase) == 16);
            assert!(core::mem::offset_of!(GcPhaseRecord, start_ns) == 24);
        };
        #[derive(Clone, Copy)]
        struct Frame {
            cycle: u64,
            parent: u64,
            slots: [usize; DEPTH],
            phases: [u32; DEPTH],
            open: usize,
            overflow: usize,
        }
        impl Frame {
            const EMPTY: Self = Self {
                cycle: 0, parent: 0, slots: [ABSENT; DEPTH],
                phases: [0; DEPTH], open: 0, overflow: 0,
            };
        }
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Birth { address: u64, sequence: u64, epoch: u64 }
        impl Birth {
            const EMPTY: Self = Self { address: 0, sequence: 0, epoch: 0 };
        }

        // This type is shared with the actual-source component adapter, not a
        // second implementation of the recorder or allocation identity map.
        pub struct Recorder {
            pub records: [GcPhaseRecord; CAPACITY],
            pub count: usize,
            frames: [Frame; DEPTH],
            pub depth: usize,
            pub totals: [u64; 21],
            map: [Birth; MAP_CAPACITY],
            sequence: u64,
            pub enabled: bool,
            pub phases: bool,
            pub survival: bool,
            complete_sample: bool,
            pub export_owner: u64,
            next_export_owner: u64,
            pub live: [u64; 5],
            pub dead: [u64; 5],
        }
        impl Recorder {
            pub const fn new() -> Self {
                Self {
                    records: [GcPhaseRecord::ZERO; CAPACITY], count: 0,
                    frames: [Frame::EMPTY; DEPTH], depth: 0, totals: [0; 21],
                    map: [Birth::EMPTY; MAP_CAPACITY], sequence: 0,
                    enabled: false, phases: false, survival: false,
                    complete_sample: true, export_owner: 0, next_export_owner: 0,
                    live: [0; 5], dead: [0; 5],
                }
            }
            pub fn reset_process(&mut self, mode: u8) {
                // Do not construct/copy a multi-megabyte temporary on C stack.
                self.count = 0;
                self.depth = 0;
                self.frames.fill(Frame::EMPTY);
                self.records.fill(GcPhaseRecord::ZERO);
                self.map.fill(Birth::EMPTY);
                self.totals.fill(0);
                self.totals[0] = 1;
                self.totals[1] = CAPACITY as u64;
                self.sequence = 0;
                self.next_export_owner = 0;
                self.export_owner = 0;
                self.phases = mode == 1 || mode == 3;
                self.survival = mode == 2 || mode == 3;
                self.enabled = false;
                self.complete_sample = true;
                self.live.fill(0);
                self.dead.fill(0);
            }
            pub fn busy(&self) -> bool { self.depth != 0 || self.export_owner != 0 }
            pub fn current_cycle(&self) -> u64 {
                if self.depth == 0 { 0 } else { self.frames[self.depth - 1].cycle }
            }
            fn increment(&mut self, slot: usize) {
                match self.totals[slot].checked_add(1) {
                    Some(value) => self.totals[slot] = value,
                    None => {
                        self.totals[4] = self.totals[4].saturating_add(1);
                        self.totals[9] = self.totals[9].saturating_add(1);
                        self.enabled = false;
                    }
                }
            }
            pub fn reset_window(&mut self) -> bool {
                if self.busy() { return false; }
                let Some(window) = self.totals[0].checked_add(1) else {
                    self.increment(9); return false;
                };
                let epoch = self.totals[10];
                let last_cycle = self.totals[11];
                let occupancy = self.totals[15];
                self.totals.fill(0);
                self.totals[0] = window;
                self.totals[1] = CAPACITY as u64;
                self.totals[10] = epoch;
                self.totals[11] = last_cycle;
                self.totals[15] = occupancy;
                self.count = 0;
                self.dead.fill(0);
                // Live ages refer to process sweep epochs, never window IDs.
                self.live.fill(0);
                for item in &self.map {
                    if item.address > 1 {
                        let bucket = Self::bucket(epoch.saturating_sub(item.epoch));
                        self.live[bucket] = self.live[bucket].saturating_add(1);
                    }
                }
                true
            }
            pub fn reserve_cycle(&mut self) -> u64 {
                if !self.enabled || !self.phases { return 0; }
                if self.totals[11] >= i64::MAX as u64 {
                    self.increment(4); self.increment(9);
                    self.enabled = false; return 0;
                }
                self.totals[11] += 1;
                self.totals[11]
            }
            pub fn enter_cycle(&mut self, token: u64) -> bool {
                if token == 0 { return false; }
                if self.depth == DEPTH {
                    self.increment(4); self.increment(7); return false;
                }
                let parent = if self.depth == 0 { 0 } else { self.frames[self.depth - 1].cycle };
                self.frames[self.depth] = Frame { cycle: token, parent, ..Frame::EMPTY };
                self.depth += 1;
                if self.export_owner != 0 { self.increment(18); }
                true
            }
            fn append(&mut self, record: GcPhaseRecord) -> usize {
                if self.count == CAPACITY {
                    self.increment(3);
                    return ABSENT;
                }
                let slot = self.count;
                self.records[slot] = record;
                self.count += 1;
                slot
            }
            pub fn phase_begin(&mut self, token: u64, phase: u32, now: Option<u64>) {
                if token == 0 { return; }
                if self.depth == 0 || self.frames[self.depth - 1].cycle != token {
                    self.increment(4); return;
                }
                let index = self.depth - 1;
                if self.frames[index].open == DEPTH {
                    self.frames[index].overflow = self.frames[index].overflow.saturating_add(1);
                    self.increment(4); self.increment(7); return;
                }
                let mut flags = if self.export_owner == 0 { 0 } else { EXPORT };
                if now.is_none() { flags |= CLOCK; self.increment(4); self.increment(6); }
                let slot = self.append(GcPhaseRecord {
                    cycle: token, parent_cycle: self.frames[index].parent,
                    phase, flags, start_ns: now.unwrap_or(0), end_ns: 0,
                });
                let at = self.frames[index].open;
                self.frames[index].slots[at] = slot;
                self.frames[index].phases[at] = phase;
                self.frames[index].open += 1;
            }
            pub fn phase_end(&mut self, token: u64, phase: u32, now: Option<u64>) {
                if token == 0 { return; }
                if self.depth == 0 || self.frames[self.depth - 1].cycle != token {
                    self.increment(4); return;
                }
                let index = self.depth - 1;
                if self.frames[index].overflow != 0 {
                    self.frames[index].overflow -= 1;
                    return;
                }
                let open = self.frames[index].open;
                if open == 0 || self.frames[index].phases[open - 1] != phase {
                    self.increment(4); return;
                }
                self.frames[index].open -= 1;
                let slot = self.frames[index].slots[open - 1];
                if slot == ABSENT { return; }
                let valid = now.is_some_and(|end| end >= self.records[slot].start_ns)
                    && self.records[slot].flags & CLOCK == 0;
                if valid {
                    self.records[slot].end_ns = now.unwrap();
                    self.records[slot].flags |= COMPLETE;
                } else {
                    self.records[slot].flags |= CLOCK;
                    self.increment(4); self.increment(6);
                }
            }
            pub fn classification(&mut self, token: u64, young: bool, reason: u32, now: Option<u64>) {
                if token == 0 || self.depth == 0 || self.frames[self.depth - 1].cycle != token { return; }
                let mut flags = COMPLETE | if young { 256 } else { 512 } | ((reason & 255) << 16);
                if self.export_owner != 0 { flags |= EXPORT; }
                if now.is_none() { flags = (flags & !COMPLETE) | CLOCK; self.increment(4); self.increment(6); }
                self.append(GcPhaseRecord { cycle: token,
                    parent_cycle: self.frames[self.depth - 1].parent,
                    phase: 21, flags, start_ns: now.unwrap_or(0), end_ns: now.unwrap_or(0) });
            }
            pub fn end_cycle(&mut self, token: u64) {
                if token == 0 { return; }
                if self.depth == 0 || self.frames[self.depth - 1].cycle != token { return; }
                if self.frames[self.depth - 1].open != 0 || self.frames[self.depth - 1].overflow != 0 {
                    self.abort_cycle(token); return;
                }
                self.depth -= 1;
                self.frames[self.depth] = Frame::EMPTY;
            }
            pub fn abort_cycle(&mut self, token: u64) {
                if token == 0 { return; }
                let Some(index) = self.frames[..self.depth].iter().position(|frame| frame.cycle == token) else { return; };
                // Only the exact owner may be popped. An unrelated child is
                // never discarded to make an out-of-order callback look valid.
                for record in &mut self.records[..self.count] {
                    if record.cycle == token && record.flags & COMPLETE == 0 { record.flags |= INCOMPLETE; }
                }
                self.increment(4); self.increment(5);
                if index + 1 == self.depth {
                    self.frames[index] = Frame::EMPTY;
                    self.depth -= 1;
                }
            }
            pub fn acquire_export(&mut self) -> Option<u64> {
                if self.busy() { return None; }
                let Some(token) = self.next_export_owner.checked_add(1) else {
                    self.increment(9); return None;
                };
                self.next_export_owner = token;
                self.export_owner = token;
                Some(token)
            }
            pub fn release_export(&mut self, token: u64) {
                if token != 0 && self.export_owner == token { self.export_owner = 0; }
            }
            fn bucket(age: u64) -> usize {
                match age { 0 => 0, 1 => 1, 2..=3 => 2, 4..=7 => 3, _ => 4 }
            }
            fn origin(address: u64) -> usize {
                let mixed = (address >> 4).wrapping_mul(0x9e3779b97f4a7c15);
                ((mixed ^ (mixed >> 32)) as usize) & (MAP_CAPACITY - 1)
            }
            fn find(&mut self, address: u64) -> Option<usize> {
                if address <= 1 { return None; }
                let origin = Self::origin(address);
                for offset in 0..PROBES {
                    let index = (origin + offset) & (MAP_CAPACITY - 1);
                    let key = self.map[index].address;
                    if key == address { return Some(index); }
                    if key == 0 { return None; }
                }
                self.incomplete_map();
                None
            }
            fn incomplete_map(&mut self) {
                self.complete_sample = false;
                self.increment(16);
            }
            pub fn cons_birth(&mut self, address: u64) {
                if !self.survival { return; }
                self.increment(12);
                let Some(sequence) = self.sequence.checked_add(1) else {
                    self.incomplete_map(); self.increment(9); return;
                };
                self.sequence = sequence;
                if address <= 1 { self.incomplete_map(); return; }
                let origin = Self::origin(address);
                let mut tombstone = None;
                for offset in 0..PROBES {
                    let index = (origin + offset) & (MAP_CAPACITY - 1);
                    let key = self.map[index].address;
                    if key == 1 && tombstone.is_none() { tombstone = Some(index); }
                    if key == address || key == 0 {
                        let target = if key == address { index } else { tombstone.unwrap_or(index) };
                        if key != address { self.totals[15] += 1; }
                        self.map[target] = Birth { address, sequence, epoch: self.totals[10] };
                        return;
                    }
                }
                if let Some(target) = tombstone {
                    self.map[target] = Birth { address, sequence, epoch: self.totals[10] };
                    self.totals[15] += 1;
                } else { self.incomplete_map(); }
            }
            pub fn cons_retire(&mut self, address: u64, explicit: bool) {
                if !self.survival { return; }
                if explicit { self.increment(13); }
                if let Some(index) = self.find(address) {
                    let epoch = self.totals[10].saturating_add(u64::from(!explicit));
                    let bucket = Self::bucket(epoch.saturating_sub(self.map[index].epoch));
                    self.dead[bucket] = self.dead[bucket].saturating_add(1);
                    if !explicit { self.increment(14); }
                    self.map[index] = Birth { address: 1, ..Birth::EMPTY };
                    self.totals[15] -= 1;
                }
            }
            pub fn sweep_begin(&mut self) {
                if self.survival { self.live.fill(0); }
            }
            pub fn cons_survivor(&mut self, address: u64) {
                if !self.survival { return; }
                if let Some(index) = self.find(address) {
                    let age = self.totals[10].saturating_add(1).saturating_sub(self.map[index].epoch);
                    let bucket = Self::bucket(age);
                    self.live[bucket] = self.live[bucket].saturating_add(1);
                } else { self.increment(17); }
            }
            pub fn completed_sweep(&mut self) {
                if self.survival { self.increment(10); }
            }
            pub fn export_totals(&self, owner_errors: u64) -> [u64; 21] {
                let mut totals = self.totals;
                totals[2] = self.count as u64;
                totals[4] = totals[4].saturating_add(owner_errors);
                totals[8] = totals[8].saturating_add(owner_errors);
                totals[19] = u64::from(self.enabled);
                totals[20] = u64::from(self.survival && self.complete_sample);
                totals
            }
            pub fn live_ages(&self) -> [u64; 5] {
                let mut ages = [0_u64; 5];
                if self.survival {
                    for item in &self.map {
                        if item.address > 1 {
                            let bucket = Self::bucket(self.totals[10].saturating_sub(item.epoch));
                            ages[bucket] = ages[bucket].saturating_add(1);
                        }
                    }
                }
                ages
            }
        }
        pub fn checked_ns(rc: i32, sec: i64, nsec: i64) -> Option<u64> {
            if rc != 0 || sec < 0 || !(0..1_000_000_000).contains(&nsec) { return None; }
            (sec as u64).checked_mul(1_000_000_000)?.checked_add(nsec as u64)
        }
    }

    static MODE: AtomicU8 = AtomicU8::new(0);
    static READY: AtomicBool = AtomicBool::new(false);
    static OWNER_ERRORS: AtomicU64 = AtomicU64::new(0);
    static RECORDER: VmGlobal<numeric::Recorder> = VmGlobal::<numeric::Recorder>::new (numeric::Recorder::new());
    static DETACHED_RECORDS: VmGlobal<[numeric::GcPhaseRecord; numeric::CAPACITY]> = VmGlobal::<[numeric::GcPhaseRecord; numeric::CAPACITY]>::new ([numeric::GcPhaseRecord::ZERO; numeric::CAPACITY]);
    static DETACHED_TOTALS: VmGlobal<[u64; 21]> = VmGlobal::<[u64; 21]>::new ([0; 21]);
    static DETACHED_AGES: VmGlobal<[[u64; 5]; 2]> = VmGlobal::<[[u64; 5]; 2]>::new ([[0; 5]; 2]);

    unsafe extern "C" {
        fn in_current_thread() -> bool;
        #[link_name = "clock_gettime"]
        fn observer_clock_gettime(clock: c_int, value: *mut timespec) -> c_int;
    }
    #[inline]
    unsafe fn owner() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !READY.load(Ordering::Relaxed) { return false; }
        if in_current_thread() { true } else {
            let _ = OWNER_ERRORS.fetch_update(Ordering::Relaxed, Ordering::Relaxed,
                |value| Some(value.saturating_add(1)));
            false
        }
    
  }
}
    #[inline]
    unsafe fn state() -> &'static mut numeric::Recorder {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { &mut *(RECORDER.as_mut_ptr ()) 
  }
}
    unsafe fn now() -> Option<u64> {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let mut value = timespec { tv_sec: 0, tv_nsec: 0 };
        let rc = observer_clock_gettime(6, &raw mut value);
        numeric::checked_ns(rc, value.tv_sec as i64, value.tv_nsec as i64)
    
  }
}
    pub(super) unsafe fn reset_process() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        READY.store(false, Ordering::Relaxed);
        let value = getenv(c"FEMACS_GC_OBSERVATION".as_ptr());
        let mode = if value.is_null() { 0 } else {
            match std::ffi::CStr::from_ptr(value).to_bytes() {
                b"off" => 0, b"phases" => 1, b"survival" => 2, b"all" => 3,
                _ => fatal(c"Invalid FEMACS_GC_OBSERVATION mode".as_ptr()),
            }
        };
        // init_alloc is process setup, before user Lisp or foreign hooks run.
        state().reset_process(mode);
        OWNER_ERRORS.store(0, Ordering::Relaxed);
        MODE.store(mode, Ordering::Relaxed);
    
  }
}
    pub(super) unsafe fn activate_owner() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !in_current_thread() { fatal(c"GC observer thread initialization failed".as_ptr()); }
        READY.store(true, Ordering::Relaxed);
    
  }
}
    pub(super) unsafe fn begin_cycle() -> u64 {
        // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
        unsafe {
            if MODE.load(Ordering::Relaxed) == 0 || !owner() { return 0; }
            let token = state().reserve_cycle();
            if token == 0 { return 0; }
            // Guard may allocate specpdl storage; enter only after it is installed.
            record_unwind_protect_intmax(Some(abort_cycle), token as i64);
            if state().enter_cycle(token) { token } else { 0 }
        }
    }
    pub(super) unsafe extern "C-unwind" fn abort_cycle(token: i64) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if token > 0 && owner() { state().abort_cycle(token as u64); }
    
  }
}
    pub(super) unsafe fn phase_begin(token: u64, phase: u32) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if token != 0 && owner() { let clock = now(); state().phase_begin(token, phase, clock); }
    
  }
}
    pub(super) unsafe fn phase_end(token: u64, phase: u32) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if token != 0 && owner() { let clock = now(); state().phase_end(token, phase, clock); }
    
  }
}
    pub(super) unsafe fn classification(token: u64, young: bool, reason: u32) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if token != 0 && owner() { let clock = now(); state().classification(token, young, reason, clock); }
    
  }
}
    pub(super) unsafe fn current_cycle() -> u64 {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if MODE.load(Ordering::Relaxed) == 0 || !owner() { return 0; }
        state().current_cycle()
    
  }
}
    pub(super) unsafe fn end_cycle(token: u64) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if token != 0 && owner() { state().end_cycle(token); }
    
  }
}
    #[inline]
    unsafe fn survival_owner() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let mode = MODE.load(Ordering::Relaxed);
        (mode == 2 || mode == 3) && owner()
    
  }
}
    pub(super) unsafe fn cons_birth(ptr: *mut Lisp_Cons) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if survival_owner() { state().cons_birth(ptr as u64); }
    
  }
}
    pub(super) unsafe fn cons_retire(ptr: *mut Lisp_Cons, explicit: bool) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if survival_owner() { state().cons_retire(ptr as u64, explicit); }
    
  }
}
    pub(super) unsafe fn cons_survivor(ptr: *mut Lisp_Cons) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if survival_owner() { state().cons_survivor(ptr as u64); }
    
  }
}
    pub(super) unsafe fn sweep_begin() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if survival_owner() { state().sweep_begin(); }
    
  }
}
    pub(super) unsafe fn completed_sweep() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if survival_owner() { state().completed_sweep(); }
    
  }
}
    pub(super) unsafe fn reset_window_idle() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !owner() || !state().reset_window() { return false; }
        OWNER_ERRORS.swap(0, Ordering::Relaxed);
        true
    
  }
}
    pub(super) unsafe fn control(enabled: bool) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !owner() || state().busy() { error(c"GC observer is busy".as_ptr()); }
        if !state().phases { error(c"GC observer mode does not support phase control".as_ptr()); }
        let previous = state().enabled;
        state().enabled = enabled;
        if previous { Qt() } else { Qnil() }
    
  }
}

    struct SnapshotRoots {
        result: Lisp_Object, totals: Lisp_Object, records: Lisp_Object,
        ages: Lisp_Object, current: Lisp_Object, owner_token: u64,
    }
    unsafe extern "C-unwind" fn mark_snapshot(data: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let roots = &*(data as *const SnapshotRoots);
        mark_object(roots.result); mark_object(roots.totals);
        mark_object(roots.records); mark_object(roots.ages); mark_object(roots.current);
    
  }
}
    unsafe extern "C-unwind" fn release_snapshot(data: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let roots = &mut *(data as *mut SnapshotRoots);
        if owner() { state().release_export(roots.owner_token); }
        roots.owner_token = 0;
    
  }
}
    pub(super) unsafe fn snapshot_idle() -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { snapshot_impl(0) 
  }
}
    #[cfg(feature = "femacs_gc_observer_test_hooks")]
    pub(super) unsafe fn snapshot_with_fault(fault: u32) -> Lisp_Object {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { snapshot_impl(fault) 
  }
}

    unsafe fn snapshot_impl(fault: u32) -> Lisp_Object {
        // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
        unsafe {
        if !owner() || state().busy() { error(c"GC observer is busy".as_ptr()); }
        let count = SPECPDL_INDEX();
        let mut roots = SnapshotRoots {
            result: Qnil(), totals: Qnil(), records: Qnil(), ages: Qnil(),
            current: Qnil(), owner_token: 0,
        };
        record_unwind_protect_ptr_mark(Some(release_snapshot),
            (&raw mut roots).cast(), Some(mark_snapshot));
        #[cfg(feature = "femacs_gc_observer_test_hooks")]
        if fault == 1 { error(c"GC observer test error before ownership".as_ptr()); }
        let Some(token) = state().acquire_export() else { error(c"GC observer export unavailable".as_ptr()); };
        roots.owner_token = token;
        (*DETACHED_TOTALS.as_mut_ptr ()) = state().export_totals(OWNER_ERRORS.load(Ordering::Relaxed));
        let length = state().count;
        for index in 0..length { (*DETACHED_RECORDS.as_mut_ptr ())[index] = state().records[index]; }
        (*DETACHED_AGES.as_mut_ptr ())[0] = state().live_ages();
        (*DETACHED_AGES.as_mut_ptr ())[1] = state().dead;
        // Every Lisp intermediate has a guard-traced field or is already
        // reachable through the result before the next allocating operation.
        roots.result = make_vector(4, Qnil());
        ASET(roots.result, 0, make_fixnum(1));
        roots.totals = make_vector(21, Qnil());
        ASET(roots.result, 1, roots.totals);
        roots.records = make_vector(length as isize, Qnil());
        ASET(roots.result, 2, roots.records);
        roots.ages = make_vector(2, Qnil());
        ASET(roots.result, 3, roots.ages);
        #[cfg(feature = "femacs_gc_observer_test_hooks")]
        {
            if fault == 2 { garbage_collect(); }
            if fault == 3 { error(c"GC observer test error after ownership".as_ptr()); }
        }
        for index in 0..21 {
            let value = (*DETACHED_TOTALS.as_mut_ptr ())[index];
            roots.current = make_uint(value as uintmax_t);
            ASET(roots.totals, index as isize, roots.current);
        }
        for index in 0..length {
            let record = (*DETACHED_RECORDS.as_mut_ptr ())[index];
            roots.current = make_vector(6, Qnil());
            ASET(roots.records, index as isize, roots.current);
            let values = [record.cycle, record.parent_cycle, record.phase as u64,
                record.flags as u64, record.start_ns, record.end_ns];
            for slot in 0..6 {
                // The record vector is already reachable from roots.records.
                roots.current = make_uint(values[slot] as uintmax_t);
                ASET(AREF(roots.records, index as isize), slot as isize, roots.current);
            }
        }
        for family in 0..2 {
            roots.current = make_vector(5, Qnil());
            ASET(roots.ages, family as isize, roots.current);
            for bucket in 0..5 {
                roots.current = make_uint((*DETACHED_AGES.as_mut_ptr ())[family][bucket] as uintmax_t);
                ASET(AREF(roots.ages, family as isize), bucket as isize, roots.current);
            }
        }
        unbind_to(count, roots.result)
        }
    }
