//! `#![forbid(unsafe_code)]` Timsort + Powersort implementation (`src/sort.rs`, `U7` / `P-486`).
//!
//! Uses `RootedVec` workspaces so all temporary keys and values are synchronously rooted across
//! arbitrary GC cycles triggered by `:key` or `:lessp`, and RAII `Drop` guards relocate any
//! in-flight merge run back into `values` if a Lisp predicate signals `NonLocalExit`.

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

const MAX_MERGE_PENDING: usize = PTRDIFF_WIDTH;
const GALLOP_WIN_MIN: usize = 7;
const MERGESTATE_TEMP_SIZE: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PredKind {
    ValueLt,
    Lss,
    Gtr,
    Lisp,
}

#[derive(Clone, Copy, Default)]
struct Stretch {
    base: usize,
    len: usize,
    power: c_int,
}

struct MergeState {
    has_values: bool,
    keys: RootedVec,
    temp_keys: RootedVec,
    temp_vals: RootedVec,
    pending: [Stretch; MAX_MERGE_PENDING],
    n: usize,
    min_gallop: usize,
    listlen: usize,
    pred_kind: PredKind,
    predicate: Lisp_Object,
}

impl MergeState {
    fn new(listlen: usize, has_values: bool, predicate: Lisp_Object) -> Self {
        let pred_kind = match classify_sort_subr_predicate(predicate) {
            0 => PredKind::ValueLt,
            1 => PredKind::Lss,
            2 => PredKind::Gtr,
            _ => PredKind::Lisp,
        };
        let init_cap = ((listlen + 1) / 2).min(MERGESTATE_TEMP_SIZE);
        Self {
            has_values,
            keys: if has_values {
                RootedVec::filled(listlen, Qnil())
            } else {
                RootedVec::new()
            },
            temp_keys: RootedVec::filled(init_cap, Qnil()),
            temp_vals: if has_values {
                RootedVec::filled(init_cap, Qnil())
            } else {
                RootedVec::new()
            },
            pending: [Stretch::default(); MAX_MERGE_PENDING],
            n: 0,
            min_gallop: GALLOP_WIN_MIN,
            listlen,
            pred_kind,
            predicate,
        }
    }

    #[inline]
    fn ensure_temp(&mut self, need: usize) {
        if self.temp_keys.len() < need {
            self.temp_keys.resize(need, Qnil());
        }
        if self.has_values && self.temp_vals.len() < need {
            self.temp_vals.resize(need, Qnil());
        }
    }

    #[inline(always)]
    fn inorder(&self, a: Lisp_Object, b: Lisp_Object) -> bool {
        match self.pred_kind {
            PredKind::ValueLt => {
                if FIXNUMP(a) && FIXNUMP(b) {
                    XFIXNUM(a) < XFIXNUM(b)
                } else {
                    !NILP(Fvaluelt(a, b))
                }
            }
            PredKind::Lss => {
                if FIXNUMP(a) && FIXNUMP(b) && !debug_on_next_call.get() {
                    maybe_quit();
                    XFIXNUM(a) < XFIXNUM(b)
                } else {
                    !NILP(calln!(self.predicate, a, b))
                }
            }
            PredKind::Gtr => {
                if FIXNUMP(a) && FIXNUMP(b) && !debug_on_next_call.get() {
                    maybe_quit();
                    XFIXNUM(a) > XFIXNUM(b)
                } else {
                    !NILP(calln!(self.predicate, a, b))
                }
            }
            PredKind::Lisp => !NILP(calln!(self.predicate, a, b)),
        }
    }
}

#[inline(always)]
fn key_at(ms: &MergeState, seq: &[Lisp_Object], idx: usize) -> Lisp_Object {
    if ms.has_values {
        ms.keys[idx]
    } else {
        seq[idx]
    }
}

fn reverse_range(ms: &mut MergeState, seq: &mut [Lisp_Object], lo: usize, hi: usize) {
    if hi.saturating_sub(lo) < 2 {
        return;
    }
    if ms.has_values {
        ms.keys.as_mut_slice()[lo..hi].reverse();
    }
    seq[lo..hi].reverse();
}

fn binarysort(ms: &mut MergeState, seq: &mut [Lisp_Object], lo: usize, hi: usize, mut start: usize) {
    debug_assert!(lo <= start && start <= hi);
    if lo == start {
        start += 1;
    }
    while start < hi {
        let mut l = lo;
        let mut r = start;
        let pivot_key = key_at(ms, seq, r);
        debug_assert!(l < r);
        while l < r {
            let p = l + ((r - l) >> 1);
            let p_key = key_at(ms, seq, p);
            if ms.inorder(pivot_key, p_key) {
                r = p;
            } else {
                l = p + 1;
            }
        }
        debug_assert_eq!(l, r);
        if ms.has_values {
            ms.keys.as_mut_slice().copy_within(l..start, l + 1);
            ms.keys[l] = pivot_key;
        }
        let pivot_val = seq[start];
        seq.copy_within(l..start, l + 1);
        seq[l] = pivot_val;
        start += 1;
    }
}

fn count_run(ms: &MergeState, seq: &[Lisp_Object], lo: usize, hi: usize) -> (usize, bool) {
    debug_assert!(lo < hi);
    if lo + 1 == hi {
        return (1, false);
    }
    let mut idx = lo + 1;
    let mut n = 2usize;
    let descending = if ms.inorder(key_at(ms, seq, idx), key_at(ms, seq, idx - 1)) {
        idx += 1;
        while idx < hi {
            if !ms.inorder(key_at(ms, seq, idx), key_at(ms, seq, idx - 1)) {
                break;
            }
            idx += 1;
            n += 1;
        }
        true
    } else {
        idx += 1;
        while idx < hi {
            if ms.inorder(key_at(ms, seq, idx), key_at(ms, seq, idx - 1)) {
                break;
            }
            idx += 1;
            n += 1;
        }
        false
    };
    (n, descending)
}

fn gallop_left_slice(
    ms: &MergeState,
    key: Lisp_Object,
    a: &[Lisp_Object],
    hint: usize,
) -> usize {
    let n = a.len();
    debug_assert!(n > 0 && hint < n);

    let mut lastofs: isize = 0;
    let mut ofs: isize = 1;
    if ms.inorder(a[hint], key) {
        let maxofs = (n - hint) as isize;
        while ofs < maxofs {
            if ms.inorder(a[(hint as isize + ofs) as usize], key) {
                lastofs = ofs;
                ofs = (ofs << 1) + 1;
            } else {
                break;
            }
        }
        if ofs > maxofs {
            ofs = maxofs;
        }
        lastofs += hint as isize;
        ofs += hint as isize;
    } else {
        let maxofs = (hint + 1) as isize;
        while ofs < maxofs {
            if ms.inorder(a[(hint as isize - ofs) as usize], key) {
                break;
            }
            lastofs = ofs;
            ofs = (ofs << 1) + 1;
        }
        if ofs > maxofs {
            ofs = maxofs;
        }
        let k = lastofs;
        lastofs = hint as isize - ofs;
        ofs = hint as isize - k;
    }

    debug_assert!(-1 <= lastofs && lastofs < ofs && ofs <= n as isize);
    lastofs += 1;
    let mut lo = lastofs as usize;
    let mut hi = ofs as usize;
    while lo < hi {
        let m = lo + ((hi - lo) >> 1);
        if ms.inorder(a[m], key) {
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    debug_assert_eq!(lo, hi);
    hi
}

fn gallop_right_slice(
    ms: &MergeState,
    key: Lisp_Object,
    a: &[Lisp_Object],
    hint: usize,
) -> usize {
    let n = a.len();
    debug_assert!(n > 0 && hint < n);

    let mut lastofs: isize = 0;
    let mut ofs: isize = 1;
    if ms.inorder(key, a[hint]) {
        let maxofs = (hint + 1) as isize;
        while ofs < maxofs {
            if ms.inorder(key, a[(hint as isize - ofs) as usize]) {
                lastofs = ofs;
                ofs = (ofs << 1) + 1;
            } else {
                break;
            }
        }
        if ofs > maxofs {
            ofs = maxofs;
        }
        let k = lastofs;
        lastofs = hint as isize - ofs;
        ofs = hint as isize - k;
    } else {
        let maxofs = (n - hint) as isize;
        while ofs < maxofs {
            if ms.inorder(key, a[(hint as isize + ofs) as usize]) {
                break;
            }
            lastofs = ofs;
            ofs = (ofs << 1) + 1;
        }
        if ofs > maxofs {
            ofs = maxofs;
        }
        lastofs += hint as isize;
        ofs += hint as isize;
    }

    debug_assert!(-1 <= lastofs && lastofs < ofs && ofs <= n as isize);
    lastofs += 1;
    let mut lo = lastofs as usize;
    let mut hi = ofs as usize;
    while lo < hi {
        let m = lo + ((hi - lo) >> 1);
        if ms.inorder(key, a[m]) {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    debug_assert_eq!(lo, hi);
    hi
}

struct MergeLoGuard<'a> {
    seq: &'a mut [Lisp_Object],
    ms: &'a mut MergeState,
    ssa_off: usize,
    dest_idx: usize,
    na: usize,
    active: bool,
}

impl Drop for MergeLoGuard<'_> {
    fn drop(&mut self) {
        if self.active && self.na > 0 {
            if self.ms.has_values {
                self.seq[self.dest_idx..self.dest_idx + self.na]
                    .copy_from_slice(&self.ms.temp_vals[self.ssa_off..self.ssa_off + self.na]);
            } else {
                self.seq[self.dest_idx..self.dest_idx + self.na]
                    .copy_from_slice(&self.ms.temp_keys[self.ssa_off..self.ssa_off + self.na]);
            }
        }
    }
}

fn merge_lo(ms: &mut MergeState, seq: &mut [Lisp_Object], base_a: usize, na: usize, nb: usize) {
    debug_assert!(na > 0 && nb > 0);
    let base_b = base_a + na;
    ms.ensure_temp(na);
    if ms.has_values {
        ms.temp_keys.as_mut_slice()[..na].copy_from_slice(&ms.keys[base_a..base_a + na]);
        ms.temp_vals.as_mut_slice()[..na].copy_from_slice(&seq[base_a..base_a + na]);
    } else {
        ms.temp_keys.as_mut_slice()[..na].copy_from_slice(&seq[base_a..base_a + na]);
    }

    let mut g = MergeLoGuard {
        seq,
        ms,
        ssa_off: 0,
        dest_idx: base_a,
        na,
        active: true,
    };
    let mut ssb_idx = base_b;
    let mut rem_b = nb;

    // Move first element of B into dest.
    if g.ms.has_values {
        g.ms.keys[g.dest_idx] = g.ms.keys[ssb_idx];
    }
    g.seq[g.dest_idx] = g.seq[ssb_idx];
    g.dest_idx += 1;
    ssb_idx += 1;
    rem_b -= 1;

    enum Label {
        Succeed,
        CopyB,
    }

    let label = 'flow: {
        if rem_b == 0 {
            break 'flow Label::Succeed;
        }
        if g.na == 1 {
            break 'flow Label::CopyB;
        }

        let mut min_gallop = g.ms.min_gallop;
        loop {
            let mut acount = 0usize;
            let mut bcount = 0usize;

            loop {
                debug_assert!(g.na > 1 && rem_b > 0);
                let b_key = if g.ms.has_values {
                    g.ms.keys[ssb_idx]
                } else {
                    g.seq[ssb_idx]
                };
                let a_key = g.ms.temp_keys[g.ssa_off];
                if g.ms.inorder(b_key, a_key) {
                    if g.ms.has_values {
                        g.ms.keys[g.dest_idx] = b_key;
                    }
                    g.seq[g.dest_idx] = g.seq[ssb_idx];
                    g.dest_idx += 1;
                    ssb_idx += 1;
                    bcount += 1;
                    acount = 0;
                    rem_b -= 1;
                    if rem_b == 0 {
                        break 'flow Label::Succeed;
                    }
                    if bcount >= min_gallop {
                        break;
                    }
                } else {
                    if g.ms.has_values {
                        g.ms.keys[g.dest_idx] = a_key;
                        g.seq[g.dest_idx] = g.ms.temp_vals[g.ssa_off];
                    } else {
                        g.seq[g.dest_idx] = a_key;
                    }
                    g.dest_idx += 1;
                    g.ssa_off += 1;
                    acount += 1;
                    bcount = 0;
                    g.na -= 1;
                    if g.na == 1 {
                        break 'flow Label::CopyB;
                    }
                    if acount >= min_gallop {
                        break;
                    }
                }
            }

            min_gallop += 1;
            loop {
                debug_assert!(g.na > 1 && rem_b > 0);
                min_gallop -= usize::from(min_gallop > 1);
                g.ms.min_gallop = min_gallop;

                let b_key = if g.ms.has_values {
                    g.ms.keys[ssb_idx]
                } else {
                    g.seq[ssb_idx]
                };
                let k_a = {
                    let ssa_slice = &g.ms.temp_keys[g.ssa_off..g.ssa_off + g.na];
                    gallop_right_slice(g.ms, b_key, ssa_slice, 0)
                };
                acount = k_a;
                if k_a != 0 {
                    if g.ms.has_values {
                        g.ms.keys.as_mut_slice()[g.dest_idx..g.dest_idx + k_a]
                            .copy_from_slice(&g.ms.temp_keys[g.ssa_off..g.ssa_off + k_a]);
                        g.seq[g.dest_idx..g.dest_idx + k_a]
                            .copy_from_slice(&g.ms.temp_vals[g.ssa_off..g.ssa_off + k_a]);
                    } else {
                        g.seq[g.dest_idx..g.dest_idx + k_a]
                            .copy_from_slice(&g.ms.temp_keys[g.ssa_off..g.ssa_off + k_a]);
                    }
                    g.dest_idx += k_a;
                    g.ssa_off += k_a;
                    g.na -= k_a;
                    if g.na == 1 {
                        break 'flow Label::CopyB;
                    }
                    if g.na == 0 {
                        break 'flow Label::Succeed;
                    }
                }
                if g.ms.has_values {
                    g.ms.keys[g.dest_idx] = g.ms.keys[ssb_idx];
                }
                g.seq[g.dest_idx] = g.seq[ssb_idx];
                g.dest_idx += 1;
                ssb_idx += 1;
                rem_b -= 1;
                if rem_b == 0 {
                    break 'flow Label::Succeed;
                }

                let a_key = g.ms.temp_keys[g.ssa_off];
                let k_b = if g.ms.has_values {
                    gallop_left_slice(g.ms, a_key, &g.ms.keys[ssb_idx..ssb_idx + rem_b], 0)
                } else {
                    gallop_left_slice(g.ms, a_key, &g.seq[ssb_idx..ssb_idx + rem_b], 0)
                };
                bcount = k_b;
                if k_b != 0 {
                    if g.ms.has_values {
                        g.ms.keys
                            .as_mut_slice()
                            .copy_within(ssb_idx..ssb_idx + k_b, g.dest_idx);
                    }
                    g.seq.copy_within(ssb_idx..ssb_idx + k_b, g.dest_idx);
                    g.dest_idx += k_b;
                    ssb_idx += k_b;
                    rem_b -= k_b;
                    if rem_b == 0 {
                        break 'flow Label::Succeed;
                    }
                }
                if g.ms.has_values {
                    g.ms.keys[g.dest_idx] = g.ms.temp_keys[g.ssa_off];
                    g.seq[g.dest_idx] = g.ms.temp_vals[g.ssa_off];
                } else {
                    g.seq[g.dest_idx] = g.ms.temp_keys[g.ssa_off];
                }
                g.dest_idx += 1;
                g.ssa_off += 1;
                g.na -= 1;
                if g.na == 1 {
                    break 'flow Label::CopyB;
                }
                if !(acount >= GALLOP_WIN_MIN || bcount >= GALLOP_WIN_MIN) {
                    break;
                }
            }
            min_gallop += 1;
            g.ms.min_gallop = min_gallop;
        }
    };

    g.active = false;
    match label {
        Label::Succeed => {
            if g.na != 0 {
                if g.ms.has_values {
                    g.ms.keys.as_mut_slice()[g.dest_idx..g.dest_idx + g.na]
                        .copy_from_slice(&g.ms.temp_keys[g.ssa_off..g.ssa_off + g.na]);
                    g.seq[g.dest_idx..g.dest_idx + g.na]
                        .copy_from_slice(&g.ms.temp_vals[g.ssa_off..g.ssa_off + g.na]);
                } else {
                    g.seq[g.dest_idx..g.dest_idx + g.na]
                        .copy_from_slice(&g.ms.temp_keys[g.ssa_off..g.ssa_off + g.na]);
                }
            }
        }
        Label::CopyB => {
            debug_assert!(g.na == 1 && rem_b > 0);
            if g.ms.has_values {
                g.ms.keys
                    .as_mut_slice()
                    .copy_within(ssb_idx..ssb_idx + rem_b, g.dest_idx);
                g.ms.keys[g.dest_idx + rem_b] = g.ms.temp_keys[g.ssa_off];
                g.seq.copy_within(ssb_idx..ssb_idx + rem_b, g.dest_idx);
                g.seq[g.dest_idx + rem_b] = g.ms.temp_vals[g.ssa_off];
            } else {
                g.seq.copy_within(ssb_idx..ssb_idx + rem_b, g.dest_idx);
                g.seq[g.dest_idx + rem_b] = g.ms.temp_keys[g.ssa_off];
            }
        }
    }
}

struct MergeHiGuard<'a> {
    seq: &'a mut [Lisp_Object],
    ms: &'a mut MergeState,
    dest_end_excl: usize,
    nb: usize,
    active: bool,
}

impl Drop for MergeHiGuard<'_> {
    fn drop(&mut self) {
        if self.active && self.nb > 0 {
            let start = self.dest_end_excl - self.nb;
            if self.ms.has_values {
                self.seq[start..self.dest_end_excl].copy_from_slice(&self.ms.temp_vals[..self.nb]);
            } else {
                self.seq[start..self.dest_end_excl].copy_from_slice(&self.ms.temp_keys[..self.nb]);
            }
        }
    }
}

fn merge_hi(ms: &mut MergeState, seq: &mut [Lisp_Object], base_a: usize, na: usize, nb: usize) {
    debug_assert!(na > 0 && nb > 0);
    let base_b = base_a + na;
    ms.ensure_temp(nb);
    if ms.has_values {
        ms.temp_keys.as_mut_slice()[..nb].copy_from_slice(&ms.keys[base_b..base_b + nb]);
        ms.temp_vals.as_mut_slice()[..nb].copy_from_slice(&seq[base_b..base_b + nb]);
    } else {
        ms.temp_keys.as_mut_slice()[..nb].copy_from_slice(&seq[base_b..base_b + nb]);
    }

    let mut g = MergeHiGuard {
        seq,
        ms,
        dest_end_excl: base_b + nb,
        nb,
        active: true,
    };
    let mut rem_a = na;

    // Move last element of A into dest_end_excl - 1.
    g.dest_end_excl -= 1;
    rem_a -= 1;
    if g.ms.has_values {
        g.ms.keys[g.dest_end_excl] = g.ms.keys[base_a + rem_a];
    }
    g.seq[g.dest_end_excl] = g.seq[base_a + rem_a];

    enum Label {
        Succeed,
        CopyA,
    }

    let label = 'flow: {
        if rem_a == 0 {
            break 'flow Label::Succeed;
        }
        if g.nb == 1 {
            break 'flow Label::CopyA;
        }

        let mut min_gallop = g.ms.min_gallop;
        loop {
            let mut acount = 0usize;
            let mut bcount = 0usize;

            loop {
                debug_assert!(rem_a > 0 && g.nb > 1);
                let b_key = g.ms.temp_keys[g.nb - 1];
                let a_idx = base_a + rem_a - 1;
                let a_key = if g.ms.has_values {
                    g.ms.keys[a_idx]
                } else {
                    g.seq[a_idx]
                };
                if g.ms.inorder(b_key, a_key) {
                    g.dest_end_excl -= 1;
                    if g.ms.has_values {
                        g.ms.keys[g.dest_end_excl] = a_key;
                    }
                    g.seq[g.dest_end_excl] = g.seq[a_idx];
                    acount += 1;
                    bcount = 0;
                    rem_a -= 1;
                    if rem_a == 0 {
                        break 'flow Label::Succeed;
                    }
                    if acount >= min_gallop {
                        break;
                    }
                } else {
                    g.dest_end_excl -= 1;
                    g.nb -= 1;
                    if g.ms.has_values {
                        g.ms.keys[g.dest_end_excl] = b_key;
                        g.seq[g.dest_end_excl] = g.ms.temp_vals[g.nb];
                    } else {
                        g.seq[g.dest_end_excl] = b_key;
                    }
                    bcount += 1;
                    acount = 0;
                    if g.nb == 1 {
                        break 'flow Label::CopyA;
                    }
                    if bcount >= min_gallop {
                        break;
                    }
                }
            }

            min_gallop += 1;
            loop {
                debug_assert!(rem_a > 0 && g.nb > 1);
                min_gallop -= usize::from(min_gallop > 1);
                g.ms.min_gallop = min_gallop;

                let b_key = g.ms.temp_keys[g.nb - 1];
                let pos_a = if g.ms.has_values {
                    gallop_right_slice(
                        g.ms,
                        b_key,
                        &g.ms.keys[base_a..base_a + rem_a],
                        rem_a - 1,
                    )
                } else {
                    gallop_right_slice(g.ms, b_key, &g.seq[base_a..base_a + rem_a], rem_a - 1)
                };
                let k_a = rem_a - pos_a;
                acount = k_a;
                if k_a != 0 {
                    g.dest_end_excl -= k_a;
                    rem_a -= k_a;
                    if g.ms.has_values {
                        g.ms.keys.as_mut_slice().copy_within(
                            base_a + rem_a..base_a + rem_a + k_a,
                            g.dest_end_excl,
                        );
                    }
                    g.seq
                        .copy_within(base_a + rem_a..base_a + rem_a + k_a, g.dest_end_excl);
                    if rem_a == 0 {
                        break 'flow Label::Succeed;
                    }
                }
                g.dest_end_excl -= 1;
                g.nb -= 1;
                if g.ms.has_values {
                    g.ms.keys[g.dest_end_excl] = g.ms.temp_keys[g.nb];
                    g.seq[g.dest_end_excl] = g.ms.temp_vals[g.nb];
                } else {
                    g.seq[g.dest_end_excl] = g.ms.temp_keys[g.nb];
                }
                if g.nb == 1 {
                    break 'flow Label::CopyA;
                }

                let a_idx = base_a + rem_a - 1;
                let a_key = if g.ms.has_values {
                    g.ms.keys[a_idx]
                } else {
                    g.seq[a_idx]
                };
                let pos_b = gallop_left_slice(g.ms, a_key, &g.ms.temp_keys[..g.nb], g.nb - 1);
                let k_b = g.nb - pos_b;
                bcount = k_b;
                if k_b != 0 {
                    g.dest_end_excl -= k_b;
                    g.nb -= k_b;
                    if g.ms.has_values {
                        g.ms.keys.as_mut_slice()[g.dest_end_excl..g.dest_end_excl + k_b]
                            .copy_from_slice(&g.ms.temp_keys[g.nb..g.nb + k_b]);
                        g.seq[g.dest_end_excl..g.dest_end_excl + k_b]
                            .copy_from_slice(&g.ms.temp_vals[g.nb..g.nb + k_b]);
                    } else {
                        g.seq[g.dest_end_excl..g.dest_end_excl + k_b]
                            .copy_from_slice(&g.ms.temp_keys[g.nb..g.nb + k_b]);
                    }
                    if g.nb == 1 {
                        break 'flow Label::CopyA;
                    }
                    if g.nb == 0 {
                        break 'flow Label::Succeed;
                    }
                }
                g.dest_end_excl -= 1;
                rem_a -= 1;
                if g.ms.has_values {
                    g.ms.keys[g.dest_end_excl] = g.ms.keys[base_a + rem_a];
                }
                g.seq[g.dest_end_excl] = g.seq[base_a + rem_a];
                if rem_a == 0 {
                    break 'flow Label::Succeed;
                }
                if !(acount >= GALLOP_WIN_MIN || bcount >= GALLOP_WIN_MIN) {
                    break;
                }
            }
            min_gallop += 1;
            g.ms.min_gallop = min_gallop;
        }
    };

    g.active = false;
    match label {
        Label::Succeed => {
            if g.nb != 0 {
                let start = g.dest_end_excl - g.nb;
                if g.ms.has_values {
                    g.ms.keys.as_mut_slice()[start..g.dest_end_excl]
                        .copy_from_slice(&g.ms.temp_keys[..g.nb]);
                    g.seq[start..g.dest_end_excl].copy_from_slice(&g.ms.temp_vals[..g.nb]);
                } else {
                    g.seq[start..g.dest_end_excl].copy_from_slice(&g.ms.temp_keys[..g.nb]);
                }
            }
        }
        Label::CopyA => {
            debug_assert!(g.nb == 1 && rem_a > 0);
            let start = g.dest_end_excl - rem_a;
            if g.ms.has_values {
                g.ms.keys
                    .as_mut_slice()
                    .copy_within(base_a..base_a + rem_a, start);
                g.ms.keys[start - 1] = g.ms.temp_keys[0];
                g.seq.copy_within(base_a..base_a + rem_a, start);
                g.seq[start - 1] = g.ms.temp_vals[0];
            } else {
                g.seq.copy_within(base_a..base_a + rem_a, start);
                g.seq[start - 1] = g.ms.temp_keys[0];
            }
        }
    }
}

fn merge_at(ms: &mut MergeState, seq: &mut [Lisp_Object], i: usize) {
    debug_assert!(ms.n >= 2);
    debug_assert!(i == ms.n - 2 || i == ms.n - 3);

    let mut base_a = ms.pending[i].base;
    let mut na = ms.pending[i].len;
    let base_b = ms.pending[i + 1].base;
    let mut nb = ms.pending[i + 1].len;
    debug_assert!(na > 0 && nb > 0 && base_a + na == base_b);

    ms.pending[i].len = na + nb;
    if i + 3 == ms.n {
        ms.pending[i + 1] = ms.pending[i + 2];
    }
    ms.n -= 1;

    let b0_key = key_at(ms, seq, base_b);
    let k = if ms.has_values {
        gallop_right_slice(ms, b0_key, &ms.keys[base_a..base_a + na], 0)
    } else {
        gallop_right_slice(ms, b0_key, &seq[base_a..base_a + na], 0)
    };
    base_a += k;
    na -= k;
    if na == 0 {
        return;
    }

    let a_last_key = key_at(ms, seq, base_a + na - 1);
    nb = if ms.has_values {
        gallop_left_slice(ms, a_last_key, &ms.keys[base_b..base_b + nb], nb - 1)
    } else {
        gallop_left_slice(ms, a_last_key, &seq[base_b..base_b + nb], nb - 1)
    };
    if nb == 0 {
        return;
    }

    if na <= nb {
        merge_lo(ms, seq, base_a, na, nb);
    } else {
        merge_hi(ms, seq, base_a, na, nb);
    }
}

fn powerloop(s1: usize, n1: usize, n2: usize, n: usize) -> c_int {
    debug_assert!(n1 > 0 && n2 > 0 && s1 + n1 + n2 <= n);
    let mut a = 2 * s1 + n1;
    let mut b = a + n1 + n2;
    let mut result: c_int = 0;
    loop {
        result += 1;
        if a >= n {
            debug_assert!(b >= a);
            a -= n;
            b -= n;
        } else if b >= n {
            break;
        }
        debug_assert!(a < b && b < n);
        a <<= 1;
        b <<= 1;
    }
    result
}

fn found_new_run(ms: &mut MergeState, seq: &mut [Lisp_Object], n2: usize) {
    if ms.n != 0 {
        let top = ms.pending[ms.n - 1];
        let power = powerloop(top.base, top.len, n2, ms.listlen);
        while ms.n > 1 && ms.pending[ms.n - 2].power > power {
            let merge_idx = ms.n - 2;
            merge_at(ms, seq, merge_idx);
        }
        debug_assert!(ms.n < 2 || ms.pending[ms.n - 2].power < power);
        ms.pending[ms.n - 1].power = power;
    }
}

fn merge_force_collapse(ms: &mut MergeState, seq: &mut [Lisp_Object]) {
    while ms.n > 1 {
        let mut idx = ms.n - 2;
        if idx > 0 && ms.pending[idx - 1].len < ms.pending[idx + 1].len {
            idx -= 1;
        }
        merge_at(ms, seq, idx);
    }
}

pub(crate) fn merge_compute_minrun(mut n: usize) -> usize {
    let mut r = 0usize;
    while n >= 64 {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn resolve_fun(mut fun: Lisp_Object) -> Lisp_Object {
    if SYMBOLP(fun) {
        let mut f = symbol_function_slot(fun);
        if SYMBOLP(f) {
            f = indirect_function(f);
        }
        if !NILP(f) && !(CONSP(f) && EQ(XCAR(f), Qautoload())) {
            fun = f;
        }
    }
    fun
}

pub fn tim_sort_rooted(
    mut predicate: Lisp_Object,
    mut keyfunc: Lisp_Object,
    seq: &mut RootedVec,
    reverse: bool,
) {
    let length = seq.len();
    if EQ(predicate, Qvaluelt()) {
        predicate = Qnil();
    }
    if !NILP(predicate) {
        predicate = resolve_fun(predicate);
    }
    if EQ(keyfunc, Qidentity()) {
        keyfunc = Qnil();
    }

    if reverse && length > 0 {
        seq.as_mut_slice().reverse();
    }

    let has_values = !NILP(keyfunc);
    if has_values {
        keyfunc = resolve_fun(keyfunc);
    }

    let mut ms = MergeState::new(length, has_values, predicate);

    if has_values {
        for i in 0..length {
            let elem = seq[i];
            ms.keys[i] = calln!(keyfunc, elem);
        }
    }

    if length == 0 {
        return;
    }

    let minrun = merge_compute_minrun(length);
    let mut lo = 0usize;
    let mut nremaining = length;
    loop {
        let (mut n, descending) = count_run(&ms, seq.as_slice(), lo, lo + nremaining);
        if descending {
            reverse_range(&mut ms, seq.as_mut_slice(), lo, lo + n);
        }
        if n < minrun {
            let force = nremaining.min(minrun);
            binarysort(&mut ms, seq.as_mut_slice(), lo, lo + force, lo + n);
            n = force;
        }
        debug_assert!(ms.n == 0 || ms.pending[ms.n - 1].base + ms.pending[ms.n - 1].len == lo);
        found_new_run(&mut ms, seq.as_mut_slice(), n);
        debug_assert!(ms.n < MAX_MERGE_PENDING);
        ms.pending[ms.n] = Stretch {
            base: lo,
            len: n,
            power: 0,
        };
        ms.n += 1;
        lo += n;
        nremaining -= n;
        if nremaining == 0 {
            break;
        }
    }

    merge_force_collapse(&mut ms, seq.as_mut_slice());
    debug_assert!(ms.n == 1 && ms.pending[0].len == length);

    if reverse {
        seq.as_mut_slice().reverse();
    }
}
