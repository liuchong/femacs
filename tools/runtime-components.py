"""Build the actual-source component adapter for the 28 B component vectors.

The command is intentionally a build owner, not a test runner.  It copies the
two actual runtime modules unchanged except for appended private-state wrappers,
adds disclosed primitive host stubs, compiles the native adapter, and writes a
schema-1 receipt consumed by tools/test-runtime-contracts.py.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

OBS = 'src/runtime_gc_observation.rs'
COL = 'src/runtime_gc_collector.rs'
STUBS = [
    'primitive Lisp_Object/Lisp_Cons tags and target SIZE_WIDTH; actual bitmap aliases and cons_block definition included',
    'fixed allocator cons block, mem_find, gcstat and live-byte counters',
    'getenv/fatal/error/clock_gettime/in_current_thread host stubs',
    'unwind-protect/specpdl registration no-ops',
    'Lisp vector/mark helpers used only by non-selected snapshot/status code',
]

OBSERVER = r'''
use serde_json::{json, Value};
use std::collections::HashMap;

pub(super) fn component_run(steps: &[Value], inputs: &[Value]) -> Result<Value, String> {
    let mut labels: HashMap<String, u64> = HashMap::new();
    let mut addresses: HashMap<String, u64> = HashMap::new();
    let mut next_address = 0x100000u64;
    let mut max_depth = 0usize;
    let mut before_activate = 0u64;
    let mut snapshot: Option<Vec<[u64; 6]>> = None;
    let mut first_record: Option<[u64; 6]> = None;
    let mut records_at_reset: Vec<[u64; 6]> = Vec::new();
    let mut outer_done = false;
    let mut inner_aborted = false;
    let mut repeat_noop = true;
    let mut occupancy_before_reset = 0u64;
    let mut protected_births = Vec::new();
    let mut arrays_at_reset = 0u64;
    let mut totals_at_reset = [0u64;21];
    let mut disabled_id = 0u64;
    let mut exhausted_id = 0u64;
    let mut rejected_id = 0u64;
    let rows = || unsafe {
        state().records[..state().count]
            .iter()
            .map(|r| [r.cycle, r.parent_cycle, r.phase as u64,
                      r.flags as u64, r.start_ns, r.end_ns])
            .collect::<Vec<_>>()
    };
    for step in steps {
        let op = step.get("op").and_then(Value::as_str).ok_or("observer step op")?;
        match op {
            "reset" | "reset_unready" => {
                let mode = step.get("mode").and_then(Value::as_str).unwrap_or("off");
                unsafe {
                    std::env::set_var("FEMACS_GC_OBSERVATION", mode);
                    reset_process();
                    if state().phases { state().enabled = true; }
                    if op == "reset" { activate_owner(); }
                }
                records_at_reset = rows();
                arrays_at_reset = unsafe { state().component_arrays_digest() };
                totals_at_reset = unsafe { state().totals };
                before_activate = 0;
            }
            "activate_owner" => {
                before_activate = unsafe { state().totals[12] };
                unsafe { activate_owner(); }
            }
            "owner" => {
                let current = step.get("current").and_then(Value::as_bool).unwrap_or(true);
                super::CURRENT_THREAD.store(current, Ordering::Relaxed);
            }
            "cons_birth" => {
                let identity = step.get("identity").and_then(Value::as_str).unwrap_or("cell");
                let address = *addresses.entry(identity.to_owned()).or_insert_with(|| {
                    let value = next_address;
                    next_address += 16;
                    value
                });
                unsafe { cons_birth(address as *mut Lisp_Cons); }
            }
            "begin" => {
                let label = step.get("as").and_then(Value::as_str).unwrap_or("cycle");
                let token = unsafe { begin_cycle() };
                labels.insert(label.to_owned(), token);
                max_depth = max_depth.max(unsafe { state().depth });
                if label == "rejected" { rejected_id = token; }
                if label == "disabled" { disabled_id = token; }
                if label == "exhausted" { exhausted_id = token; }
            }
            "phase" => {
                let label = step.get("cycle").and_then(Value::as_str);
                let (token, synthetic) = if let Some(label) = label {
                    (labels.get(label).copied().unwrap_or(0), false)
                } else {
                    (unsafe { begin_cycle() }, true)
                };
                let phase = step.get("phase").and_then(Value::as_u64).unwrap_or(0) as u32;
                let start = step.get("start_ns").and_then(Value::as_u64).unwrap_or(0);
                let end = step.get("end_ns").and_then(Value::as_u64).unwrap_or(0);
                unsafe {
                    if owner() {
                        let recorder = state();
                        recorder.phase_begin(token, phase, Some(start));
                        recorder.phase_end(token, phase, Some(end));
                    }
                    if synthetic { end_cycle(token); }
                }
                if first_record.is_none() { first_record = rows().first().copied(); }
            }
            "end" => {
                let label = step.get("cycle").and_then(Value::as_str).unwrap_or("");
                let token = labels.get(label).copied().unwrap_or(0);
                unsafe { end_cycle(token); }
                if label == "outer" { outer_done = true; }
            }
            "abort" => {
                let label = step.get("cycle").and_then(Value::as_str).unwrap_or("");
                if let Some(token) = labels.get(label).copied() {
                    let before = unsafe { state().totals[5] };
                    unsafe { abort_cycle(token as i64); }
                    if label == "inner" {
                        if inner_aborted {
                            repeat_noop = unsafe { state().totals[5] } == before;
                        } else { inner_aborted = true; }
                    }
                }
            }
            "complete_phase_records" => {
                let count = step.get("count").and_then(Value::as_u64).unwrap_or(0);
                let token = unsafe { begin_cycle() };
                for index in 0..count {
                    unsafe {
                        if owner() {
                            let recorder = state();
                            recorder.phase_begin(token, 2, Some(index));
                            recorder.phase_end(token, 2, Some(index + 1));
                        }
                    }
                    if first_record.is_none() { first_record = rows().first().copied(); }
                }
                unsafe { end_cycle(token); }
            }
            "reset_window" => {
                occupancy_before_reset = unsafe { state().totals[15] };
                unsafe { reset_window_idle(); }
            }
            "owner_false_marker" => { super::CURRENT_THREAD.store(false, Ordering::Relaxed); }
            "completed_sweep" => {
                unsafe { sweep_begin(); }
                if let Some(survivors) = step.get("survivors").and_then(Value::as_array) {
                    for identity in survivors {
                        if let Some(name) = identity.as_str() {
                            if let Some(address) = addresses.get(name) {
                                unsafe { cons_survivor(*address as *mut Lisp_Cons); }
                            }
                        }
                    }
                }
                unsafe { completed_sweep(); }
            }
            "cons_free" => {
                let identity = step.get("identity").and_then(Value::as_str).unwrap_or("");
                if let Some(address) = addresses.get(identity) {
                    unsafe { cons_retire(*address as *mut Lisp_Cons, true); }
                }
            }
            "set_last_sequence" => {
                let value = step.get("value").and_then(Value::as_u64).unwrap_or(0);
                unsafe { state().totals[11] = value; }
            }
            "detach_snapshot" => {
                unsafe {
                    state().acquire_export().ok_or("snapshot ownership unavailable")?;
                    component_detach_numeric();
                    snapshot = Some(DETACHED_RECORDS[..DETACHED_TOTALS[2] as usize].iter()
                        .map(|r| [r.cycle,r.parent_cycle,r.phase as u64,r.flags as u64,r.start_ns,r.end_ns]).collect());
                }
            }
            "export_gc" => {
                let count = step.get("phase_records").and_then(Value::as_u64).unwrap_or(0);
                unsafe {
                    let token = begin_cycle();
                    for index in 0..count {
                        let recorder = state();
                        recorder.phase_begin(token, 4, Some(index));
                        recorder.phase_end(token, 4, Some(index + 1));
                    }
                    end_cycle(token);
                }
            }
            "release_snapshot" => {
                unsafe {
                    let owner = state().export_owner;
                    state().release_export(owner);
                }
            }
            "distinct_cons_births" => {
                let count = step.get("count").and_then(Value::as_u64).unwrap_or(0);
                for index in 0..count {
                    unsafe { cons_birth((0x200000u64 + index * 16) as *mut Lisp_Cons); }
                    if protected_births.is_empty() && unsafe { state().export_totals(0)[20] == 0 } {
                        protected_births = unsafe { state().component_births() };
                    }
                }
            }
            "nested_begins" => {
                let count = step.get("count").and_then(Value::as_u64).unwrap_or(0);
                for index in 0..count {
                    let token = unsafe { begin_cycle() };
                    labels.insert(format!("nested-{index}"), token);
                    max_depth = max_depth.max(unsafe { state().depth });
                }
            }
            "unwind_all" => {
                let mut tokens = labels.values().copied().filter(|token| *token != 0).collect::<Vec<_>>();
                tokens.sort_unstable();
                for token in tokens.into_iter().rev() {
                    unsafe { abort_cycle(token as i64); }
                }
                labels.clear();
            }
            other => return Err(format!("unsupported observer op: {other}")),
        }
    }
    let raw = unsafe { state().totals };
    let exported = unsafe { state().export_totals(OWNER_ERRORS.load(Ordering::Relaxed)) };
    let live = unsafe { state().live_ages() };
    let records = rows();
    let inner = labels.get("inner").copied();
    let outer = labels.get("outer").copied();
    let inner_parent = records.iter().any(|r| Some(r[0]) == inner && Some(r[1]) == outer);
    let outer_duration = records.iter().find(|r| Some(r[0]) == outer && r[2] == 2)
        .map(|r| r[5].saturating_sub(r[4]));
    let inner_duration = records.iter().find(|r| Some(r[0]) == inner && r[2] == 3)
        .map(|r| r[5].saturating_sub(r[4]));
    let nested_not_summed = outer_duration.zip(inner_duration)
        .map(|(outer, inner)| outer != inner).unwrap_or(false);
    let invalid_clock = records.iter().any(|r| r[3] & 2 != 0);
    let no_complete = records.iter().all(|r| r[3] & 1 == 0);
    let first_ok = first_record.map(|value| records.first() == Some(&value)).unwrap_or(true);
    let snapshot_ok = snapshot.as_ref().map(|old| unsafe {
        let detached = DETACHED_RECORDS[..DETACHED_TOTALS[2] as usize].iter()
            .map(|r| [r.cycle,r.parent_cycle,r.phase as u64,r.flags as u64,r.start_ns,r.end_ns]).collect::<Vec<_>>();
        *old == detached
    }).unwrap_or(false);
    let export_flags = snapshot.as_ref().map(|old| {
        records.iter().skip(old.len()).any(|r| r[3] & 8 != 0)
    }).unwrap_or(false);
    let age = live.iter().position(|value| *value != 0).unwrap_or(0) as u64;
    let all_invalid = inputs.iter().all(|input| {
        let rc = input.get("rc").and_then(Value::as_i64).unwrap_or(0) as i32;
        let sec = input.get("sec").and_then(Value::as_i64).unwrap_or(0);
        let nsec = input.get("nsec").and_then(Value::as_i64).unwrap_or(0);
        unsafe { numeric::checked_ns(rc, sec, nsec).is_none() }
    });
    let retained_births = unsafe { state().component_births() }.into_iter().collect::<std::collections::HashSet<_>>();
    Ok(json!({
        "early_is_unknown": before_activate == 0,
        "owned_births": raw[12].saturating_sub(before_activate),
        "atomic_owner_errors": OWNER_ERRORS.load(Ordering::Relaxed),
        "distinct_positive_ids": outer.zip(inner).map(|(a,b)| a > 0 && b > 0 && a != b).unwrap_or(false),
        "inner_parent_is_outer": inner_parent,
        "open_cycles": unsafe { state().depth },
        "invalid": raw[4],
        "nested_durations_are_not_summed": nested_not_summed,
        "outer_completed": outer_done,
        "inner_incomplete": inner_aborted,
        "repeat_abort_is_noop": repeat_noop,
        "records": records.len(),
        "dropped": raw[3],
        "first_record_preserved": first_ok,
        "all_invalid": all_invalid,
        "no_usable_duration": if inputs.is_empty() { no_complete } else { all_invalid },
        "invalid_clock": invalid_clock,
        "rejected_id": rejected_id,
        "nonatomic_storage_unchanged": rows() == records_at_reset && raw == totals_at_reset
            && unsafe { state().component_arrays_digest() } == arrays_at_reset,
        "atomic_owner_error": unsafe { OWNER_ERRORS.load(Ordering::Relaxed) } > 0,
        "sweep_epoch": raw[10],
        "cell_a_age": age,
        "live_histogram": live.to_vec(),
        "new_birth_sequence": raw[12] == 2,
        "occupancy": raw[15],
        "occupancy_at_most": raw[15],
        "overflow_latched": unsafe { state().survival } && exported[20] == 0,
        "complete_cons_sample": exported[20] != 0,
        "no_live_eviction": raw[15] == occupancy_before_reset && !protected_births.is_empty()
            && protected_births.iter().all(|entry| retained_births.contains(entry)),
        "disabled_id": disabled_id,
        "array_storage_unchanged": rows() == records_at_reset && unsafe { state().component_arrays_digest() } == arrays_at_reset,
        "maximum_active": max_depth,
        "depth_overflow": raw[7] > 0,
        "exhausted_id": exhausted_id,
        "counter_exhaustion": raw[4] > 0,
        "no_reused_id": exhausted_id == 0,
        "snapshot_records": snapshot.as_ref().map(Vec::len).unwrap_or(0),
        "snapshot_unchanged": snapshot_ok,
        "export_records_have_flag8": export_flags,
        "export_owner_released": unsafe { state().export_owner } == 0,
    }))
}
'''

COLLECTOR = r'''
use serde_json::{json, Value};
use std::collections::HashMap;

pub(super) fn component_run(steps: &[Value], inputs: &[Value]) -> Result<Value, String> {
    unsafe {
        std::env::set_var("FEMACS_GC_COLLECTOR", "young");
        reset_process();
        activate_owner();
    }
    let mut guards: HashMap<String, *mut CollectorCycleGuard> = HashMap::new();
    let mut last_guard: *mut CollectorCycleGuard = std::ptr::null_mut();
    let mut objects: HashMap<String, Lisp_Object> = HashMap::new();
    let mut free_index = 0u64;
    let mut next_kind = String::new();
    let mut next_reason = 0u64;
    let mut invalidated = false;
    let mut selected_cycles = Vec::new();
    let mut source_vectors: Vec<Lisp_Vector> = Vec::new();
    unsafe { super::MARK_HISTORY.clear(); }
    let mut prepare = |kind: GcKind, label: &str, reason: u32, guards: &mut HashMap<String, *mut CollectorCycleGuard>| -> Result<*mut CollectorCycleGuard, String> {
        let guard = Box::into_raw(Box::new(CollectorCycleGuard::empty()));
        unsafe { prepare_cycle(kind, reason, &mut *guard); }
        guards.insert(label.to_owned(), guard);
        Ok(guard)
    };
    for step in steps {
        let op = step.get("op").and_then(Value::as_str).ok_or("collector step op")?;
        match op {
            "reset" => {
                let mode = step.get("mode").and_then(Value::as_str).unwrap_or("young");
                unsafe {
                    std::env::set_var("FEMACS_GC_COLLECTOR", mode);
                    reset_process();
                    activate_owner();
                }
            }
            "new_block" => unsafe { super::reset_test_block(); },
            "mark_indices" => {
                if let Some(array) = step.get("indices").and_then(Value::as_array) {
                    for item in array {
                        let index = item.as_u64().unwrap_or(0) as usize;
                        if index < CONS_BLOCK_SIZE { unsafe { super::test_mark(index); } }
                    }
                }
            }
            "promote_marked" => unsafe {
                for word in 0..32 { survivor_word(cons_block, word, CONS_BLOCK_SIZE); }
            },
            "new_registry" => unsafe {
                state().registry.clear();
                state().phase = CollectorPhase::Marking;
                state().cycle_valid = true;
                state().registry.valid = true;
            },
            "note_noncons" => {
                let identity = step.get("identity").and_then(Value::as_str).unwrap_or("object");
                let object = if let Some(existing) = objects.get(identity) { *existing } else {
                    let value = make_lisp_symbol(objects.len() + 1);
                    objects.insert(identity.to_owned(), value);
                    value
                };
                unsafe { note_marked_noncons(object); }
            }
            "freeze_registry" => unsafe { state().registry.freeze(); trace_remembered_owners(GcKind::Young); },
            "prepare_full" => {
                let label = step.get("as").and_then(Value::as_str).unwrap_or("cycle");
                last_guard = prepare(GcKind::Full, label, 2, &mut guards)?;
            }
            "prepare_young" => {
                let label = step.get("as").and_then(Value::as_str).unwrap_or("cycle");
                last_guard = prepare(GcKind::Young, label, 0, &mut guards)?;
            }
            "commit_full" => unsafe { if !last_guard.is_null() { commit_cycle(GcKind::Full, &mut *last_guard); } },
            "commit_young" => unsafe { if !last_guard.is_null() { commit_cycle(GcKind::Young, &mut *last_guard); } },
            "abort" => {
                let label = step.get("cycle").and_then(Value::as_str).unwrap_or("");
                if let Some(ptr) = guards.get(label).copied() {
                    unsafe { abort_cycle(ptr.cast()); }
                }
            }
            "outer_cleanup" => {
                let label = step.get("cycle").and_then(Value::as_str).unwrap_or("");
                if let Some(ptr) = guards.get(label).copied() {
                    unsafe { abort_cycle(ptr.cast()); }
                }
            }
            "choose" => {
                let request = if step.get("request").and_then(Value::as_str) == Some("full") { GcRequest::Full } else { GcRequest::Automatic };
                let (kind, reason) = unsafe { choose_cycle(request) };
                next_kind = if kind == GcKind::Full { "full".into() } else { "young".into() };
                next_reason = reason as u64;
                unsafe { trace_remembered_owners(kind); }
            }
            "storage_change" => {
                let identity = step.get("identity").and_then(Value::as_str).unwrap_or("");
                if let Some(object) = objects.get(identity).copied() {
                    let address = XPNTR(object) as usize;
                    unsafe {
                        state().phase = CollectorPhase::Idle;
                        before_storage_change(address, address + 16);
                    }
                    invalidated = unsafe { !state().registry.valid };
                }
            }
            "promote_indices" => {
                if let Some(array) = step.get("indices").and_then(Value::as_array) {
                    for item in array {
                        let index = item.as_u64().unwrap_or(0) as usize;
                        if index < CONS_BLOCK_SIZE { unsafe { survivor(super::test_cell(index)); } }
                    }
                }
            }
            "write" => {
                let index = step.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                if index < CONS_BLOCK_SIZE { unsafe { written(super::test_object(index)); } }
            }
            "escape" => {
                let index = step.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                if index < CONS_BLOCK_SIZE { unsafe { escaped(super::test_object(index)); } }
            }
            "free" => {
                let index = step.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                free_index = index as u64;
                if index < CONS_BLOCK_SIZE { unsafe { clear_cell(super::test_cell(index)); (*super::test_cell(index)).u.s.car = dead_object(); } }
            }
            "allocate_reused" => {
                let index = step.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                if index < CONS_BLOCK_SIZE { unsafe { clear_cell(super::test_cell(index)); (*super::test_cell(index)).u.s.car = 0; (*super::test_cell(index)).u.s.u.cdr = 0; } }
            }
            "premark_old" => {
                let guard = Box::into_raw(Box::new(CollectorCycleGuard::empty()));
                unsafe { prepare_cycle(GcKind::Young, 0, &mut *guard); }
            }
            "mark_cons" => {
                let index = step.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
                if index < CONS_BLOCK_SIZE {
                    let car = match step.get("car_kind").and_then(Value::as_str) {
                        Some("dump-cons") => make_cons_ptr(0x1000 as *mut Lisp_Cons),
                        Some("fixnum") => 2,
                        Some("nil") | None => 0,
                        _ => 2,
                    };
                    let cdr = match step.get("cdr_kind").and_then(Value::as_str) {
                        Some("dump-cons") => make_cons_ptr(0x1010 as *mut Lisp_Cons),
                        Some("fixnum") => 2,
                        Some("nil") | None => 0,
                        _ => 0,
                    };
                    unsafe {
                        let cell = super::test_cell(index);
                        (*cell).u.s.car = car;
                        (*cell).u.s.u.cdr = cdr;
                        super::test_mark(index);
                        note_cons_marked(cell);
                    }
                }
            }
            "distinct_noncons_marks" => {
                let count = step.get("count").and_then(Value::as_u64).unwrap_or(0) as usize;
                source_vectors = (0..count).map(|_| Lisp_Vector { header: vectorlike_header { size: 0 } }).collect();
                for vector in &mut source_vectors { unsafe { super::set_vector_marked(vector); } }
            }
            other => return Err(format!("unsupported collector op: {other}")),
        }
    }
    for input in inputs {
        unsafe {
            std::env::set_var("FEMACS_GC_COLLECTOR", "young");
            reset_process();
            activate_owner();
            let valid = input.get("valid").and_then(Value::as_bool).unwrap_or(true);
            let consecutive = input.get("consecutive_minors").and_then(Value::as_u64).unwrap_or(0);
            let cap = input.get("allocation_cap_exceeded").and_then(Value::as_bool).unwrap_or(false);
            let unknown = input.get("unknown_producer").and_then(Value::as_bool).unwrap_or(false);
            let pending = input.get("pending_doomed").and_then(Value::as_bool).unwrap_or(false);
            state().totals[5] = consecutive;
            state().totals[6] = 0;
            state().totals[15] = if cap { (CONS_BLOCK_SIZE + 1) as u64 } else { 0 };
            state().totals[16] = 0;
            state().registry.valid = valid;
            state().cycle_valid = true;
            state().counters_exhausted = false;
            UNKNOWN_PRODUCER.store(unknown, Ordering::Relaxed);
            FOREIGN.store(false, Ordering::Relaxed);
            LAST_MAJOR_BYTES = 0;
            if pending { super::doomed_finalizers.next = &raw mut super::DUMMY_FINALIZER; }
            else { super::doomed_finalizers.next = &raw mut super::doomed_finalizers; }
            let request = if input.get("request").and_then(Value::as_str) == Some("full") { GcRequest::Full } else { GcRequest::Automatic };
            let (kind, reason) = choose_cycle(request);
            let actual_kind = if kind == GcKind::Full { "full" } else { "young" };
            selected_cycles.push(json!({"kind":actual_kind,"reason":reason}));
        }
    }
    let (len, frozen_len, valid, entries, current_roots, old_used) = unsafe {
        let registry = &state().registry;
        let all_entries = if registry.len <= 64 {
            registry.entries[..registry.len].iter()
                .map(|object| objects.iter().find(|(_, value)| **value == *object).map(|(name, _)| name.clone()).unwrap_or_else(|| format!("{object:x}")))
                .collect::<Vec<_>>()
        } else { Vec::new() };
        let current = all_entries.iter().take(registry.frozen_len).cloned().collect::<Vec<_>>();
        let old = objects.get("unreachable-old").is_some_and(|object| super::MARK_HISTORY.contains(object));
        (registry.len, registry.frozen_len, registry.valid, all_entries, current, old)
    };
    let has_block = unsafe { !super::cons_block.is_null() };
    let old_indices = if has_block { unsafe { super::test_old_indices() } } else { Vec::new() };
    let dirty_indices = if has_block { unsafe { super::test_dirty_indices() } } else { Vec::new() };
    let padding_clear = if has_block {
        unsafe {
            let block = super::cons_block;
            let last = CONS_BLOCK_SIZE;
            let mut clear = true;
            for index in last..(32 * 64) {
                if ((*block).oldbits[index / BITS_PER_BITS_WORD] & ((1 as bits_word) << (index % BITS_PER_BITS_WORD))) != 0 { clear = false; }
                if ((*block).gcmarkbits[index / BITS_PER_BITS_WORD] & ((1 as bits_word) << (index % BITS_PER_BITS_WORD))) != 0 { clear = false; }
            }
            clear
        }
    } else { true };
    Ok(json!({
        "size": core::mem::size_of::<cons_block>() as u64,
        "capacity": CONS_BLOCK_SIZE as u64,
        "bitmap_words": if has_block { unsafe { (*super::cons_block).gcmarkbits.len() } } else { 0 },
        "old_indices": old_indices,
        "dirty_indices": dirty_indices,
        "padding_bits_clear": padding_clear,
        "marked": if has_block { unsafe { super::test_marked_count() } } else { 0 },
        "old": if has_block { unsafe { super::test_old_count() } } else { 0 },
        "dirty": if has_block { unsafe { super::test_dirty_count() } } else { 0 },
        "raw_tainted": if has_block { unsafe { (*super::cons_block).raw_tainted != 0 } } else { false },
        "reused_cell_is_young": if has_block && free_index < 1998 { unsafe { ((*super::cons_block).oldbits[(free_index as usize) / BITS_PER_BITS_WORD] & ((1 as bits_word) << (free_index as usize % BITS_PER_BITS_WORD))) == 0 } } else { true },
        "free_index": free_index,
        "len": len as u64,
        "frozen_len": frozen_len as u64,
        "valid": valid,
        "entries": entries,
        "current_roots": current_roots,
        "newly_appended_is_not_current_root": entries.last().map(|value| !current_roots.contains(value)).unwrap_or(true),
        "old_registry_used_as_root": old_used,
        "len_at_most": len,
        "next_kind": next_kind,
        "next_reason": next_reason,
        "original_mark_work_preserved": !source_vectors.is_empty() && source_vectors.iter().all(|vector| unsafe { vector.header.size & ARRAY_MARK_FLAG != 0 }),
        "invalidated_before_release": invalidated,
        "old_registry_dereferences": unsafe { super::MARK_HISTORY.len() },
        "selected_cycles": selected_cycles,
        "stale_abort_did_not_invalidate_second": valid,
        "active": unsafe { state().active_token != 0 || state().phase != CollectorPhase::Idle },
        "completed_full": unsafe { state().totals[3] },
        "completed_young": unsafe { state().totals[4] },
        "nested_state_preserved": valid && unsafe { state().totals[3] } > 0 && unsafe { state().totals[4] } > 0,
    }))
}
'''

def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def insert_wrapper(text, wrapper):
    # Test-only introspection inside the actual numeric implementation. These
    # read initialized scalar fields; no struct padding or fabricated model.
    methods = '''pub fn component_births(&self) -> Vec<(u64,u64,u64)> {
        self.map.iter().filter(|item| item.address > 1)
            .map(|item| (item.address,item.sequence,item.epoch)).collect()
    }
    pub fn component_arrays_digest(&self) -> u64 {
        let mut digest = 14695981039346656037u64;
        let mut feed = |value:u64| { digest = (digest ^ value).wrapping_mul(1099511628211); };
        for item in &self.map { feed(item.address);feed(item.sequence);feed(item.epoch); }
        for item in &self.records { feed(item.cycle);feed(item.parent_cycle);feed(item.phase as u64);feed(item.flags as u64);feed(item.start_ns);feed(item.end_ns); }
        for frame in &self.frames { feed(frame.cycle);feed(frame.parent);feed(frame.open as u64);feed(frame.overflow as u64);
            for value in &frame.slots { feed(*value as u64); } for value in &frame.phases { feed(*value as u64); } }
        for value in self.live.iter().chain(self.dead.iter()) { feed(*value); }
        digest
    }
    '''
    anchor = 'pub fn busy(&self) -> bool'
    if text.count(anchor) != 1:
        raise RuntimeError('actual numeric recorder introspection anchor changed')
    start = text.index('DETACHED_TOTALS = state().export_totals(')
    end = text.index('DETACHED_AGES[1] = state().dead;', start) + len('DETACHED_AGES[1] = state().dead;')
    detach = text[start:end]
    text = text.replace(anchor, methods + anchor)
    wrapper = 'unsafe fn component_detach_numeric() {\n' + detach + '\n}\n' + wrapper
    index = text.rfind('\n}')
    if index < 0:
        raise RuntimeError('actual module closing brace not found')
    return text[:index] + '\n' + wrapper + '\n' + text[index:]


def insert_collector_wrapper(text, wrapper):
    marker = '\n}\n\n#[unsafe(no_mangle)]'
    index = text.find(marker)
    if index < 0:
        raise RuntimeError('collector module closing brace not found')
    brace = index + 1
    return text[:brace] + '\n' + wrapper + '\n' + text[brace:]


def write(path, data, executable=False):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(data, encoding='utf-8')
    if executable:
        path.chmod(0o755)


def supporting_source(repo, fixture):
    alloc = (repo / 'src/alloc.rs').read_text(encoding='utf-8')
    lisp = (repo / 'src/include/lisp.rs').read_text(encoding='utf-8')
    spans = {}
    def item(text, anchor, closing):
        start = text.index(anchor)
        end = text.index(closing, start) + len(closing)
        spans[anchor] = [start, end]
        return text[start:end]
    layout = '\n'.join([
        item(lisp, 'pub type bits_word =', ';'),
        item(lisp, 'pub const BITS_PER_BITS_WORD:', ';'),
        item(lisp, 'pub const BITS_WORD_MAX:', ';'),
        item(alloc, 'pub const BLOCK_ALIGN:', ';'),
        item(alloc, '#[cfg(feature = "femacs_gc_generational")]\npub const CONS_BLOCK_SIZE:', ';'),
        item(alloc, '#[repr(C)]\npub struct cons_block\n{', '\n}'),
    ])
    marking = '\n'.join([
        item(lisp, 'pub const ARRAY_MARK_FLAG:', ';'),
        item(alloc, 'macro_rules! XMARK_VECTOR {', '\n}'),
        item(alloc, 'unsafe fn set_vector_marked (', '\n}'),
    ])
    if fixture.count('// @actual-layout@') != 1 or fixture.count('// @actual-vector-mark@') != 1:
        raise RuntimeError('missing exact component support insertion points')
    return (fixture.replace('// @actual-layout@', layout).replace('// @actual-vector-mark@', marking),
            {'sources': {'src/alloc.rs': digest(repo / 'src/alloc.rs'),
                         'src/include/lisp.rs': digest(repo / 'src/include/lisp.rs')},
             'spans': spans})


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', required=True)
    parser.add_argument('--repo', default=str(Path(__file__).resolve().parent.parent))
    args = parser.parse_args(argv)
    repo = Path(args.repo).resolve()
    out = Path(os.path.abspath(os.fspath(args.out)))
    build = (repo / 'build').resolve()
    if not out.is_relative_to(build) or out.exists() or out.is_symlink():
        print('runtime-components: output must be a fresh child of build', file=sys.stderr)
        return 2
    obs = repo / OBS
    col = repo / COL
    fixture = repo / 'tools/fixtures/runtime-component-adapter.rs'
    obs_hash = digest(obs)
    col_hash = digest(col)
    try:
        out.mkdir(parents=True)
        write(out / 'Cargo.toml', '''[package]\nname = "runtime-component-adapter"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n\n[features]\ndefault = ["femacs_gc_generational"]\nfemacs_gc_generational = []\nfemacs_gc_observer_test_hooks = []\n\n[dependencies]\nserde_json = "1"\n''')
        write(out / 'src/actual_observation.rs', insert_wrapper(obs.read_text(encoding='utf-8'), OBSERVER))
        write(out / 'src/actual_collector.rs', insert_collector_wrapper(col.read_text(encoding='utf-8'), COLLECTOR))
        host, support = supporting_source(repo, fixture.read_text(encoding='utf-8'))
        write(out / 'src/main.rs', host)
        env = dict(os.environ)
        env['ADAPTER_OBSERVATION_SHA256'] = obs_hash
        env['ADAPTER_COLLECTOR_SHA256'] = col_hash
        toolchain = {}
        for program in ('cargo', 'rustc'):
            probe = subprocess.run([program, '--version'], cwd=out, env=env,
                                   capture_output=True, text=True)
            toolchain[program] = probe.stdout.strip() if probe.returncode == 0 else probe.stderr.strip()
        log = out / 'build.log'
        build_args = ['cargo', 'build', '--release', '--manifest-path',
                      str(out / 'Cargo.toml'), '--target-dir', str(out / 'target')]
        recorded_args = ['build', '--release', '--manifest-path', 'Cargo.toml',
                         '--target-dir', 'target']
        with log.open('xb') as stream:
            process = subprocess.run(build_args, cwd=out, env=env,
                                     stdout=stream, stderr=subprocess.STDOUT)
        if process.returncode != 0:
            print('runtime-components: cargo build failed; see build.log', file=sys.stderr)
            return 3
        if digest(obs) != obs_hash or digest(col) != col_hash:
            print('runtime-components: actual source changed during build', file=sys.stderr)
            return 4
        if any(digest(repo / name) != value for name,value in support['sources'].items()):
            print('runtime-components: supporting source changed during build', file=sys.stderr)
            return 4
        binary = out / 'target/release/runtime-component-adapter'
        if not binary.is_file() or binary.is_symlink():
            print('runtime-components: adapter binary missing', file=sys.stderr)
            return 5
        receipt = {
            'schema': 1,
            'provider': 'actual-source-components',
            'sources': {OBS: obs_hash, COL: col_hash},
            'source_spans': {OBS: [0, obs.stat().st_size], COL: [0, col.stat().st_size]},
            'supporting_source': support,
            'binary': 'target/release/runtime-component-adapter',
            'binary_sha256': digest(binary),
            'stubs': STUBS,
            'build': {
                'command': 'cargo build --release',
                'args': recorded_args,
                'log': 'build.log',
                'toolchain': toolchain,
                'cargo_lock_sha256': digest(out / 'Cargo.lock'),
                'manifest_sha256': digest(out / 'Cargo.toml'),
                'generated': {
                    'src/main.rs': digest(out / 'src/main.rs'),
                    'src/actual_observation.rs': digest(out / 'src/actual_observation.rs'),
                    'src/actual_collector.rs': digest(out / 'src/actual_collector.rs'),
                },
            },
        }
        (out / 'receipt.json').write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n', encoding='utf-8')
        print(json.dumps({'receipt': 'receipt.json', 'binary_sha256': receipt['binary_sha256']}, sort_keys=True))
        return 0
    except (OSError, RuntimeError) as exc:
        print('runtime-components: {0}'.format(exc), file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
