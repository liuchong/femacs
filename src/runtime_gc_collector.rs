// P-346: opt-in nonmoving young-cons collector metadata. alloc.rs owns the
// original marker/sweeper; this module never replaces their object semantics.
use super::*;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

    const REGISTRY_CAPACITY: usize = 262144;
    const INDEX_CAPACITY: usize = 524288;
    const MAX_PROBES: usize = 1024;
    static MODE: AtomicU8 = AtomicU8::new(0); // 0 uninitialized,1 major,2 young
    static READY: AtomicBool = AtomicBool::new(false);
    static FOREIGN: AtomicBool = AtomicBool::new(false);
    static UNKNOWN_PRODUCER: AtomicBool = AtomicBool::new(false);
    // Set only from note_allocation; consumed at maybe_gc safepoints. Never
    // starts a collection from an allocation primitive.
    static CADENCE_PENDING: AtomicBool = AtomicBool::new(false);

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(super) enum GcKind { Full, Young }
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum CollectorPhase { Idle, Marking, Sweeping }
    pub(super) struct CollectorCycleGuard { token: u64, armed: bool }
    impl CollectorCycleGuard {
        pub(super) const fn empty() -> Self { Self { token: 0, armed: false } }
    }
    pub(super) mod numeric {
        use super::*;
        pub struct Registry {
            pub entries: [Lisp_Object; REGISTRY_CAPACITY],
            index: [u32; INDEX_CAPACITY],
            pub len: usize,
            pub frozen_len: usize,
            pub valid: bool,
            pub overflowed: bool,
        }
        impl Registry {
            pub const fn new() -> Self {
                Self { entries: [lisp_h_Qnil(); REGISTRY_CAPACITY],
                    index: [0; INDEX_CAPACITY], len: 0, frozen_len: 0,
                    valid: false, overflowed: false }
            }
            pub fn clear(&mut self) {
                // Unused entry slots are never traversed as roots; clearing
                // index/length does not allocate a large temporary on stack.
                // Every nonzero index slot comes from an insertion that also
                // increments len; an empty registry already has a zero index.
                if self.len != 0 { self.index.fill(0); }
                self.len = 0;
                self.frozen_len = 0;
                self.valid = false;
                self.overflowed = false;
            }
            pub fn freeze(&mut self) { self.frozen_len = self.len; }
            pub fn insert(&mut self, object: Lisp_Object) -> bool {
                let bits = XLI(object) as u64;
                let mixed = bits.wrapping_mul(0x9e3779b97f4a7c15);
                let origin = ((mixed ^ (mixed >> 32)) as usize) & (INDEX_CAPACITY - 1);
                for offset in 0..MAX_PROBES {
                    let slot = (origin + offset) & (INDEX_CAPACITY - 1);
                    let entry = self.index[slot];
                    if entry != 0 {
                        if self.entries[(entry - 1) as usize] == object { return true; }
                    } else {
                        if self.len == REGISTRY_CAPACITY { break; }
                        self.entries[self.len] = object;
                        self.index[slot] = (self.len + 1) as u32;
                        self.len += 1;
                        return true;
                    }
                }
                self.overflowed = true;
                self.valid = false;
                false
            }
        }
        pub fn valid_word_mask(first: usize, limit: usize) -> bits_word {
            if first >= limit { return 0; }
            let count = (limit - first).min(BITS_PER_BITS_WORD);
            if count == BITS_PER_BITS_WORD { BITS_WORD_MAX } else { ((1 as bits_word) << count) - 1 }
        }
        pub fn clear_age(old: &mut bits_word, dirty: &mut bits_word, bit: usize) {
            let mask = !((1 as bits_word) << bit);
            *old &= mask; *dirty &= mask;
        }
    }
    #[derive(Clone, Copy)]
    struct MarkedConsBlockProof { block: *mut cons_block, limit: usize, token: u64 }
    struct State {
        registry: numeric::Registry,
        phase: CollectorPhase,
        marked_cons_block: Option<MarkedConsBlockProof>,
        capture_registry: bool,
        active_token: u64,
        next_token: u64,
        totals: [u64; 24],
        reasons: [u64; 13],
        invalid_reason: u32,
        counters_exhausted: bool,
        cycle_valid: bool,
        export_owner: u64,
        next_export_owner: u64,
        // Growth origins for Automatic young cadence. Independent of the
        // retention counters' epoch label: both read totals[15]/[16], but
        // watermarks advance on Young so each minor measures new growth while
        // retention caps still accumulate since the last major.
        cadence_cons_base: u64,
        cadence_bytes_base: u64,
    }
    impl State {
        const fn new() -> Self {
            Self { registry: numeric::Registry::new(), phase: CollectorPhase::Idle,
                marked_cons_block: None,
                capture_registry: false, active_token: 0, next_token: 0, totals: [0; 24], reasons: [0; 13],
                invalid_reason: 1, counters_exhausted: false,
                cycle_valid: false,
                export_owner: 0, next_export_owner: 0,
                cadence_cons_base: 0, cadence_bytes_base: 0 }
        }
        fn count(&mut self, slot: usize, amount: u64) {
            if let Some(value) = self.totals[slot].checked_add(amount) { self.totals[slot] = value; }
            else {
                self.totals[slot] = u64::MAX;
                self.counters_exhausted = true;
                self.registry.valid = false;
                self.cycle_valid = false;
                self.invalid_reason = 11;
            }
        }
    }
    static STATE: VmGlobal<State> = VmGlobal::<State>::new (State::new());
    static DETACHED_TOTALS: VmGlobal<[u64; 24]> = VmGlobal::<[u64; 24]>::new ([0; 24]);
    static DETACHED_REASONS: VmGlobal<[u64; 13]> = VmGlobal::<[u64; 13]>::new ([0; 13]);
    unsafe extern "C" { fn in_current_thread() -> bool; }
    #[inline]
    unsafe fn state() -> &'static mut State {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { &mut *(STATE.as_mut_ptr ()) 
  }
}
    #[inline]
    unsafe fn owner() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !READY.load(Ordering::Relaxed) { return false; }
        if in_current_thread() { true } else { FOREIGN.store(true, Ordering::Relaxed); false }
    
  }
}
    #[inline]
    unsafe fn young_owner() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { MODE.load(Ordering::Relaxed) == 2 && owner() 
  }
}

    pub(super) unsafe fn reset_process() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        READY.store(false, Ordering::Relaxed);
        let value = getenv(c"FEMACS_GC_COLLECTOR".as_ptr());
        let mode = if value.is_null() { 2 } else {
            match std::ffi::CStr::from_ptr(value).to_bytes() {
                b"major" => 1, b"young" => 2,
                _ => fatal(c"Invalid FEMACS_GC_COLLECTOR mode".as_ptr()),
            }
        };
        let s = state();
        s.registry.clear(); s.phase = CollectorPhase::Idle;
        s.marked_cons_block = None;
        s.capture_registry = false;
        s.active_token = 0; s.next_token = 0;
        s.totals.fill(0); s.reasons.fill(0);
        s.totals[0] = u64::from(mode == 2); s.totals[2] = 2;
        s.totals[23] = CONS_BLOCK_SIZE as u64;
        s.invalid_reason = 1; s.counters_exhausted = false;
        s.cycle_valid = false;
        (*LAST_MAJOR_BYTES.as_mut_ptr ()) = 0;
        s.export_owner = 0; s.next_export_owner = 0;
        s.cadence_cons_base = 0; s.cadence_bytes_base = 0;
        // Normal process startup follows dump relocation. Cons age bitmaps
        // belong only to current heap blocks; dump objects use no block math.
        let mut block = cons_block.get();
        while !block.is_null() {
            (*block).oldbits.fill(0); (*block).dirtybits.fill(0);
            (*block).raw_tainted = 0;
            block = (*block).next;
        }
        FOREIGN.store(false, Ordering::Relaxed);
        UNKNOWN_PRODUCER.store(false, Ordering::Relaxed);
        CADENCE_PENDING.store(false, Ordering::Relaxed);
        MODE.store(mode, Ordering::Relaxed);
    
  }
}
    pub(super) unsafe fn activate_owner() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !in_current_thread() { fatal(c"GC collector thread initialization failed".as_ptr()); }
        READY.store(true, Ordering::Relaxed);
    
  }
}
    pub(super) unsafe fn note_allocation(bytes: usize, is_cons: bool) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        state().count(16, bytes as u64);
        if is_cons { state().count(15, 1); }
        // Arm only. Collection waits for maybe_gc / maybe_garbage_collect.
        if young_cadence_due() { CADENCE_PENDING.store(true, Ordering::Relaxed); }
    
  }
}

    #[inline]
    unsafe fn retention_cons_cap() -> Option<u64> {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        state().totals[6].checked_add(CONS_BLOCK_SIZE as u64)
    
  }
}

    #[inline]
    unsafe fn retention_bytes_cap() -> u64 {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        (*LAST_MAJOR_BYTES.as_mut_ptr ()).max(memory_full_cons_threshold as u64)
    
  }
}

    // Non-retention predicates shared with choose_cycle's Automatic Young path.
    // Retention itself is checked with lead time below, not at the cap edge.
    // Cadence bypasses a positive consing_until_gc, so GC-inhibit and an
    // already-running cycle must be refused here — not by the later early
    // return inside garbage_collect_impl after it has already asserted.
    unsafe fn young_selection_open() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if MODE.load(Ordering::Relaxed) != 2 || !READY.load(Ordering::Relaxed) || !owner() {
            return false;
        }
        if (*garbage_collection_inhibited.as_mut_ptr ()) != 0 || (*gc_in_progress.as_mut_ptr ()) {
            return false;
        }
        if !(*(doomed_finalizers.as_mut_ptr ())).next.is_null()
            && (*(doomed_finalizers.as_mut_ptr ())).next != doomed_finalizers.as_mut_ptr ()
        {
            return false;
        }
        if FOREIGN.load(Ordering::Relaxed) || UNKNOWN_PRODUCER.load(Ordering::Relaxed) {
            return false;
        }
        if state().counters_exhausted || state().next_token == u64::MAX {
            return false;
        }
        if !NILP((*Vmemory_full.as_mut_ptr ())) || !NILP((*Vpurify_flag.as_mut_ptr ())) {
            return false;
        }
        if !state().registry.valid {
            return false;
        }
        if state().totals[5] >= 3 {
            return false;
        }
        true
    
  }
}

    unsafe fn young_cadence_due() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_selection_open() {
            return false;
        }
        let Some(cons_cap) = retention_cons_cap() else { return false; };
        let bytes_cap = retention_bytes_cap();
        let cons_lead = CONS_BLOCK_SIZE as u64;
        let bytes_lead = memory_full_cons_threshold as u64;
        // Lead keeps choose_cycle able to return Young after arm→safepoint lag.
        if state().totals[15].saturating_add(cons_lead) > cons_cap {
            return false;
        }
        if state().totals[16].saturating_add(bytes_lead) > bytes_cap {
            return false;
        }
        let cons_room = cons_cap.saturating_sub(cons_lead);
        let bytes_room = bytes_cap.saturating_sub(bytes_lead);
        let cons_step = (cons_room / 3).max(1);
        let bytes_step = (bytes_room / 3).max(1);
        let cons_growth = state().totals[15].saturating_sub(state().cadence_cons_base);
        let bytes_growth = state().totals[16].saturating_sub(state().cadence_bytes_base);
        cons_growth >= cons_step || bytes_growth >= bytes_step
    
  }
}

    pub(super) unsafe fn young_cadence_pending() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !CADENCE_PENDING.load(Ordering::Relaxed) {
            return false;
        }
        if young_cadence_due() {
            true
        } else {
            // Stale arm after a race past the lead or a forcing predicate.
            CADENCE_PENDING.store(false, Ordering::Relaxed);
            false
        }
    
  }
}
    pub(super) unsafe fn note_marked_noncons(object: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() || !state().capture_registry { return; }
        if !state().registry.insert(object) {
            state().count(12, 1);
            state().cycle_valid = false;
            state().invalid_reason = 6;
        }
    
  }
}
    pub(super) unsafe fn unknown_producer_loaded() {
        // Persistent for this process: returning from a foreign call does not
        // prove that an uninstrumented producer has stopped owning aliases.
        UNKNOWN_PRODUCER.store(true, Ordering::Relaxed);
    }
    pub(super) unsafe fn choose_cycle(request: GcRequest) -> (GcKind, u32) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if MODE.load(Ordering::Relaxed) == 0 || !READY.load(Ordering::Relaxed) { return (GcKind::Full, 1); }
        if !owner() { return (GcKind::Full, 7); }
        let reason = if !(*(doomed_finalizers.as_mut_ptr ())).next.is_null()
                && (*(doomed_finalizers.as_mut_ptr ())).next != doomed_finalizers.as_mut_ptr () { 12 }
            else if matches!(request, GcRequest::Full) { 2 }
            else if MODE.load(Ordering::Relaxed) == 1 { 3 }
            else if FOREIGN.load(Ordering::Relaxed) { 7 }
            else if UNKNOWN_PRODUCER.load(Ordering::Relaxed) { 8 }
            else if state().counters_exhausted || state().next_token == u64::MAX { 11 }
            else if !NILP((*Vmemory_full.as_mut_ptr ())) || !NILP((*Vpurify_flag.as_mut_ptr ())) { 9 }
            else if !state().registry.valid { state().invalid_reason }
            else if state().totals[5] >= 3 { 4 }
            else {
                let cons_cap = state().totals[6].checked_add(CONS_BLOCK_SIZE as u64);
                if cons_cap.is_none() { 11 }
                else if state().totals[15] > cons_cap.unwrap()
                    || state().totals[16] > (*LAST_MAJOR_BYTES.as_mut_ptr ()).max(memory_full_cons_threshold as u64) { 5 }
                else { 0 }
            };
        (if reason == 0 { GcKind::Young } else { GcKind::Full }, reason)
    
  }
}
    static LAST_MAJOR_BYTES: VmGlobal<u64> = VmGlobal::<u64>::new (0);

    pub(super) unsafe fn prepare_cycle(kind: GcKind, reason: u32, guard: &mut CollectorCycleGuard) {
        // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
        unsafe {
        if !owner() { return; }
        // Consume the safepoint arm for this cycle. A nested Automatic from a
        // post-gc hook re-arms through note_allocation if growth is due again.
        CADENCE_PENDING.store(false, Ordering::Relaxed);
        state().marked_cons_block = None;
        // Guard points into the caller's stack and is registered before any
        // active token, registry or bitmap mutation.
        record_unwind_protect_ptr(Some(abort_cycle), (guard as *mut CollectorCycleGuard).cast());
        let Some(token) = state().next_token.checked_add(1) else {
            state().registry.valid = false; state().invalid_reason = 11; return;
        };
        guard.token = token; guard.armed = true;
        state().next_token = token; state().active_token = token;
        state().phase = CollectorPhase::Marking;
        // Persistent foreign/unknown producers force every later cycle Full.
        // Keep all marking, but do not rebuild an unusable remembered registry.
        // Latch once: a flag raised during this cycle takes effect next cycle,
        // without truncating the current Young frozen-prefix traversal.
        state().capture_registry = MODE.load(Ordering::Relaxed) == 2
            && !FOREIGN.load(Ordering::Relaxed)
            && !UNKNOWN_PRODUCER.load(Ordering::Relaxed);
        state().cycle_valid = true;
        state().totals[7] = 0; state().totals[9] = 0; state().totals[10] = 0;
        state().totals[18] = reason as u64;
        state().reasons[reason as usize] = state().reasons[reason as usize].saturating_add(1);
        if kind == GcKind::Full {
            state().count(17, 1);
            if reason == 1 { state().count(19, 1); }
            if reason == 2 { state().count(20, 1); }
            if reason == 4 || reason == 5 { state().count(21, 1); }
        }
        if MODE.load(Ordering::Relaxed) != 2 { return; }
        if kind == GcKind::Full { state().registry.clear(); }
        else { state().registry.freeze(); }
        let mut block = cons_block.get();
        let mut limit = cons_block_index.get() as usize;
        while !block.is_null() {
            for word in 0..(*block).gcmarkbits.len() {
                let valid = numeric::valid_word_mask(word * BITS_PER_BITS_WORD, limit);
                if kind == GcKind::Full { (*block).dirtybits[word] = 0; }
                else {
                    /* A minor cycle cannot sweep old objects the way `gc_sweep`
                       does, because `gc_sweep` re-decides liveness for every
                       class from this cycle's mark bits.  Old cons cells are
                       kept by ORing `oldbits` into `gcmarkbits`, but a cell
                       kept only that way was not traced, and the previous
                       sweep already cleared the mark bits of everything it
                       pointed to.  A non-cons child reachable solely through
                       such an old cell - an interval, string, symbol, float or
                       vector - would then be freed while still referenced, and
                       a later mark re-entering it aborts (the Cocoa checking
                       failure was `mark_interval_tree_1`).  Submitting the
                       children of exactly the resurrected cells keeps the
                       minor cycle real without sweeping live data.  */
                    /* Recomputed every cycle: `sweep_conses` clears the
                       gcmarkbit of old cells it keeps, so `oldbits` is the
                       only durable record and each minor must re-derive the
                       kept set and trace its children afresh. */
                    let mut revived = (*block).oldbits[word] & valid & !(*block).gcmarkbits[word];
                    (*block).gcmarkbits[word] |= (*block).oldbits[word] & valid;
                    while revived != 0 {
                        let bit = revived.trailing_zeros() as usize;
                        revived &= revived - 1;
                        let cell = &raw mut (*block).conses[word * BITS_PER_BITS_WORD + bit];
                        mark_object ((*cell).u.s.car);
                        mark_object ((*cell).u.s.u.cdr);
                    }
                }
            }
            block = (*block).next; limit = CONS_BLOCK_SIZE;
        }
        }
    }
    pub(super) unsafe fn trace_remembered_owners(kind: GcKind) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if kind != GcKind::Young || !young_owner() { return; }
        #[cfg(feature = "femacs_gc_observer")]
        let observation_cycle = gc_observation::current_cycle();
        #[cfg(feature = "femacs_gc_observer")]
        { gc_observation::phase_begin(observation_cycle, 23); }
        // Freeze is a prefix boundary, not a work queue: newly marked owners
        // append to the registry but are never made additional roots now.
        let frozen = state().registry.frozen_len;
        for index in 0..frozen {
            let object = state().registry.entries[index];
            mark_object(object);
        }
        #[cfg(feature = "femacs_gc_observer")]
        {
            gc_observation::phase_end(observation_cycle, 23);
            gc_observation::phase_begin(observation_cycle, 24);
        }
        let mut block = cons_block.get();
        let mut limit = cons_block_index.get() as usize;
        while !block.is_null() {
            let tainted = (*block).raw_tainted != 0;
            for word in 0..(*block).oldbits.len() {
                let valid = numeric::valid_word_mask(word * BITS_PER_BITS_WORD, limit);
                let mut selected = (*block).oldbits[word]
                    & (if tainted { BITS_WORD_MAX } else { (*block).dirtybits[word] }) & valid;
                while selected != 0 {
                    let bit = selected.trailing_zeros() as usize;
                    selected &= selected - 1;
                    let cell = &raw mut (*block).conses[word * BITS_PER_BITS_WORD + bit];
                    if tainted { state().count(10, 1); } else { state().count(9, 1); }
                    // Parent was premarked: submit its children, not parent.
                    let car = (*cell).u.s.car;
                    let cdr = (*cell).u.s.u.cdr;
                    mark_object(car); mark_object(cdr);
                }
            }
            block = (*block).next; limit = CONS_BLOCK_SIZE;
        }
        #[cfg(feature = "femacs_gc_observer")]
        { gc_observation::phase_end(observation_cycle, 24); }
    
  }
}
    pub(super) unsafe fn start_sweep() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if owner() && state().active_token != 0 {
            state().marked_cons_block = None;
            state().capture_registry = false;
            state().phase = CollectorPhase::Sweeping;
        }
    
  }
}
    pub(super) unsafe fn commit_cycle(kind: GcKind, guard: &mut CollectorCycleGuard) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !guard.armed || !owner() || state().active_token != guard.token { return; }
        state().marked_cons_block = None;
        state().totals[2] = u64::from(kind == GcKind::Young);
        if kind == GcKind::Full {
            state().count(3, 1); state().totals[5] = 0;
            state().totals[6] = (*gcstat.as_mut_ptr()).total_conses as u64;
            (*LAST_MAJOR_BYTES.as_mut_ptr ()) = total_bytes_of_live_objects() as u64;
            state().totals[15] = 0; state().totals[16] = 0;
            state().cadence_cons_base = 0; state().cadence_bytes_base = 0;
        } else {
            state().count(4, 1); state().count(5, 1);
            // Next young window measures growth after this minor; retention
            // totals keep accumulating until the next major.
            state().cadence_cons_base = state().totals[15];
            state().cadence_bytes_base = state().totals[16];
        }
        state().registry.valid = MODE.load(Ordering::Relaxed) == 2 && state().cycle_valid
            && !state().registry.overflowed && !state().counters_exhausted
            && !FOREIGN.load(Ordering::Relaxed) && !UNKNOWN_PRODUCER.load(Ordering::Relaxed);
        state().totals[11] = state().registry.len as u64;
        state().totals[22] = guard.token;
        state().capture_registry = false;
        state().phase = CollectorPhase::Idle;
        state().active_token = 0;
        guard.armed = false;
    
  }
}
    pub(super) unsafe extern "C-unwind" fn abort_cycle(data: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let guard = &mut *(data as *mut CollectorCycleGuard);
        if guard.armed && owner() && state().active_token == guard.token {
            state().marked_cons_block = None;
            state().registry.valid = false;
            state().cycle_valid = false;
            state().invalid_reason = 10;
            state().capture_registry = false;
            state().phase = CollectorPhase::Idle;
            state().active_token = 0;
        }
        guard.armed = false;
    
  }
}
    unsafe fn heap_cell(cell: *mut Lisp_Cons) -> Option<(*mut cons_block, usize)> {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if cell.is_null() || pdumper_object_p(cell.cast()) { return None; }
        let node = mem_find(cell.cast());
        if node == MEM_NIL!() || node.is_null() || (*node).r#type != mem_type::MEM_TYPE_CONS { return None; }
        let block = CONS_BLOCK!(cell);
        let index = CONS_INDEX!(cell);
        let limit = if block == cons_block.get() { cons_block_index.get() as usize } else { CONS_BLOCK_SIZE };
        if index >= limit || (*node).start as usize > block as usize
            || ((*node).end as usize).saturating_sub(block as usize) < core::mem::size_of::<cons_block>()
            || cell as usize % core::mem::size_of::<Lisp_Cons>() != 0 { return None; }
        Some((block, index))
    
  }
}
    unsafe fn marked_heap_cell(cell: *mut Lisp_Cons) -> Option<(*mut cons_block, usize)> {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        // note_cons_marked has already checked young_owner. Retain a location
        // proof only in this marking token, never across storage retirement.
        let token = state().active_token;
        if token == 0 || state().phase != CollectorPhase::Marking
            || FOREIGN.load(Ordering::Relaxed) { return heap_cell(cell); }
        if let Some(proof) = state().marked_cons_block {
            if proof.token == token && !cell.is_null() && !pdumper_object_p(cell.cast()) {
                let limit = if proof.block == cons_block.get() {
                    proof.limit.min(cons_block_index.get() as usize)
                } else { proof.limit };
                if let Some(offset) = (cell as usize).checked_sub(proof.block as usize) {
                    let size = core::mem::size_of::<Lisp_Cons>();
                    if offset < limit * size && offset % size == 0 {
                        return Some((proof.block, offset / size));
                    }
                }
            }
        }
        let location = heap_cell(cell);
        state().marked_cons_block = location.map(|(block, _)| MarkedConsBlockProof {
            block, limit: if block == cons_block.get() { cons_block_index.get() as usize } else { CONS_BLOCK_SIZE }, token,
        });
        location
    
  }
}
    pub(super) unsafe fn written(object: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() || !CONSP(object) { return; }
        let cell = lisp_h_XCONS_untracked(object);
        if let Some((block, index)) = heap_cell(cell) {
            let bit = (1 as bits_word) << (index % BITS_PER_BITS_WORD);
            if (*block).oldbits[index / BITS_PER_BITS_WORD] & bit != 0 { (*block).dirtybits[index / BITS_PER_BITS_WORD] |= bit; }
        }
    
  }
}
    pub(super) unsafe fn escaped(object: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() || !CONSP(object) { return; }
        if let Some((block, _)) = heap_cell(lisp_h_XCONS_untracked(object)) { (*block).raw_tainted = 1; }
    
  }
}
    pub(super) unsafe fn clear_cell(cell: *mut Lisp_Cons) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        if let Some((block, index)) = heap_cell(cell) {
            let word = index / BITS_PER_BITS_WORD; let mask = !((1 as bits_word) << (index % BITS_PER_BITS_WORD));
            (*block).oldbits[word] &= mask; (*block).dirtybits[word] &= mask;
        }
    
  }
}
    pub(super) unsafe fn note_cons_marked(cell: *mut Lisp_Cons) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        let Some((block, index)) = marked_heap_cell(cell) else { return; };
        let car = (*cell).u.s.car; let cdr = (*cell).u.s.u.cdr;
        let limit = if block == cons_block.get() { cons_block_index.get() as usize } else { CONS_BLOCK_SIZE };
        let nonheap_child = |object: Lisp_Object| -> bool {
            if !CONSP(object) { return false; }
            let child = lisp_h_XCONS_untracked(object);
            // heap_cell(cell) validated this entire cons allocation. Reuse
            // that proof only within its allocated, exactly aligned prefix.
            // Other blocks and dump/invalid addresses keep the original check.
            if let Some(offset) = (child as usize).checked_sub(block as usize) {
                let size = core::mem::size_of::<Lisp_Cons>();
                if offset < limit * size && offset % size == 0 { return false; }
            }
            heap_cell(child).is_none()
        };
        if nonheap_child(car) || nonheap_child(cdr) { (*block).dirtybits[index / BITS_PER_BITS_WORD] |= (1 as bits_word) << (index % BITS_PER_BITS_WORD); }
    
  }
}
    // sweep_conses owns this block and an allocated index until its cell scan
    // finishes. Preserve that location instead of looking it up in mem_find.
    pub(super) unsafe fn survivor(block: *mut cons_block, index: usize) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        let bit = (1 as bits_word) << (index % BITS_PER_BITS_WORD);
        if (*block).oldbits[index / BITS_PER_BITS_WORD] & bit == 0 { state().count(7, 1); }
        (*block).oldbits[index / BITS_PER_BITS_WORD] |= bit;
    
  }
}
    pub(super) unsafe fn survivor_word(block: *mut cons_block, word: usize, limit: usize) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        let live = (*block).gcmarkbits[word] & numeric::valid_word_mask(word * BITS_PER_BITS_WORD, limit);
        let promoted = live & !(*block).oldbits[word];
        state().count(7, promoted.count_ones() as u64);
        (*block).oldbits[word] |= live;
    
  }
}
    pub(super) unsafe fn swept_dead(block: *mut cons_block, index: usize) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() { return; }
        let word = index / BITS_PER_BITS_WORD;
        let bit = index % BITS_PER_BITS_WORD;
        let cell = &raw mut (*block).conses[index];
        if (*cell).u.s.car != dead_object() && (*block).oldbits[word] & ((1 as bits_word) << bit) == 0 {
            state().count(8, 1);
        }
        numeric::clear_age(&mut (*block).oldbits[word], &mut (*block).dirtybits[word], bit);
    
  }
}
    pub(super) unsafe fn before_storage_change(start: usize, end: usize) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if !young_owner() || start >= end { return; }
        // mem_delete also calls here for cons allocations and during sweep.
        // Location proofs must expire even when no registry work is needed.
        state().marked_cons_block = None;
        if state().registry.len == 0 { return; }
        // Owned Sweeping is narrower than gc_in_progress: original sweep
        // destroys only unmarked objects, absent from this cycle's registry.
        if state().phase == CollectorPhase::Sweeping { return; }
        if !state().registry.valid && state().phase == CollectorPhase::Idle { return; }
        for index in 0..state().registry.len {
            let object = state().registry.entries[index];
            let pointer = XPNTR(object) as usize;
            if start <= pointer && pointer < end {
                state().registry.valid = false;
                state().cycle_valid = false;
                state().invalid_reason = 6;
                state().count(13, 1);
                return;
            }
        }
    
  }
}
    pub(super) unsafe fn before_pointer_change(pointer: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        if MODE.load(Ordering::Relaxed) != 2 || pointer.is_null() || !owner() { return; }
        // Invalidate before the registry's cons-type exclusion, including the
        // lisp_align_free path, and before the allocator can reuse the block.
        state().marked_cons_block = None;
        let node = mem_find(pointer);
        if !node.is_null() && node != MEM_NIL!()
            && (*node).r#type != mem_type::MEM_TYPE_NON_LISP
            && (*node).r#type != mem_type::MEM_TYPE_CONS {
            before_storage_change((*node).start as usize, (*node).end as usize);
        }
    
  }
}

    struct StatusRoots { result: Lisp_Object, totals: Lisp_Object, reasons: Lisp_Object,
        current: Lisp_Object, token: u64 }
    unsafe extern "C-unwind" fn mark_status(data: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let roots = &*(data as *const StatusRoots);
        mark_object(roots.result); mark_object(roots.totals);
        mark_object(roots.reasons); mark_object(roots.current);
    
  }
}
    unsafe extern "C-unwind" fn release_status(data: *mut c_void) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
        let roots = &mut *(data as *mut StatusRoots);
        if owner() && roots.token != 0 && state().export_owner == roots.token { state().export_owner = 0; }
        roots.token = 0;
    
  }
}
    pub(super) unsafe fn status() -> Lisp_Object {
        // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
        unsafe {
        if !owner() || state().phase != CollectorPhase::Idle || state().export_owner != 0 {
            error(c"GC collector status is busy".as_ptr());
        }
        let count = SPECPDL_INDEX();
        let mut roots = StatusRoots { result: Qnil(), totals: Qnil(), reasons: Qnil(), current: Qnil(), token: 0 };
        record_unwind_protect_ptr_mark(Some(release_status), (&raw mut roots).cast(), Some(mark_status));
        let Some(token) = state().next_export_owner.checked_add(1) else {
            error(c"GC collector status identity exhausted".as_ptr());
        };
        roots.token = token; state().next_export_owner = token; state().export_owner = token;
        (*DETACHED_TOTALS.as_mut_ptr ()) = state().totals;
        (*DETACHED_TOTALS.as_mut_ptr ())[1] = u64::from(state().registry.valid
            && !FOREIGN.load(Ordering::Relaxed) && !UNKNOWN_PRODUCER.load(Ordering::Relaxed));
        (*DETACHED_TOTALS.as_mut_ptr ())[11] = state().registry.len as u64;
        (*DETACHED_TOTALS.as_mut_ptr ())[14] = u64::from(UNKNOWN_PRODUCER.load(Ordering::Relaxed));
        (*DETACHED_REASONS.as_mut_ptr ()) = state().reasons;
        roots.result = make_vector(3, Qnil());
        ASET(roots.result, 0, make_fixnum(1));
        roots.totals = make_vector(24, Qnil()); ASET(roots.result, 1, roots.totals);
        roots.reasons = make_vector(13, Qnil()); ASET(roots.result, 2, roots.reasons);
        for index in 0..24 {
            roots.current = make_uint((*DETACHED_TOTALS.as_mut_ptr ())[index]);
            ASET(roots.totals, index as isize, roots.current);
        }
        for index in 0..13 {
            roots.current = make_uint((*DETACHED_REASONS.as_mut_ptr ())[index]);
            ASET(roots.reasons, index as isize, roots.current);
        }
        unbind_to(count, roots.result)
        }
    }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_gc_cons_written(cell: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { written(cell); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_gc_cons_escaped(cell: Lisp_Object) {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { escaped(cell); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_gc_unknown_producer_loaded() {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe { unknown_producer_loaded(); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn femacs_gc_young_cadence_pending() -> bool {
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
    young_cadence_pending()

  }
}
