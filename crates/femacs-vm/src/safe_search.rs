//! `#![forbid(unsafe_code)]` high-level buffer search, match-data, and regexp-quoting
//! primitives (`src/search.rs`, `U7` / `P-486`).
//!
//! Low-level Boyer-Moore / regex engine execution over gap-buffer byte spans and `re_registers`
//! raw pointer arrays stays behind the controlled boundary in `src/search.rs`.

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

pub fn search_command(
    string: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
    direction: c_int,
    re: bool,
    posix: bool,
) -> Lisp_Object {
    let mut n: EMACS_INT = direction as EMACS_INT;
    if !NILP(count) {
        CHECK_FIXNUM(count);
        n *= XFIXNUM(count);
    }

    CHECK_STRING(string);
    let begv = current_buffer_begv();
    let begv_byte = current_buffer_begv_byte();
    let zv = current_buffer_zv();
    let zv_byte = current_buffer_zv_byte();
    let pt = current_buffer_pt();
    let pt_byte = current_buffer_pt_byte();

    let (lim, lim_byte) = if NILP(bound) {
        if n > 0 {
            (zv, zv_byte)
        } else {
            (begv, begv_byte)
        }
    } else {
        let raw_lim = fix_position_safe(bound);
        if if n > 0 { raw_lim < pt } else { raw_lim > pt } {
            error_invalid_search_bound();
        }
        if raw_lim > zv {
            (zv, zv_byte)
        } else if raw_lim < begv {
            (begv, begv_byte)
        } else {
            (raw_lim, current_buffer_char_to_byte(raw_lim))
        }
    };

    let (canon_tab, eqv_tab) = current_buffer_case_tables();
    set_char_table_extras(canon_tab, 2, eqv_tab);

    let fold = !NILP(Vcase_fold_search.get());
    let np = crate::search::search_buffer_safe(
        string,
        pt,
        pt_byte,
        lim,
        lim_byte,
        n,
        re,
        if fold { canon_tab } else { Qnil() },
        if fold { eqv_tab } else { Qnil() },
        posix,
    );

    if np <= 0 {
        if NILP(noerror) {
            xsignal1(Qsearch_failed(), string);
        }
        if !EQ(noerror, Qt()) {
            let cur_begv = current_buffer_begv();
            let cur_zv = current_buffer_zv();
            eassert!(cur_begv <= lim && lim <= cur_zv);
            SET_PT_BOTH(lim, lim_byte);
        }
        return Qnil();
    }

    let cur_begv = current_buffer_begv();
    let cur_zv = current_buffer_zv();
    eassert!(cur_begv <= np && np <= cur_zv);
    SET_PT(np);
    make_fixnum(np)
}

#[inline(always)]
pub fn search_backward(
    string: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(string, bound, noerror, count, -1, false, false)
}

#[inline(always)]
pub fn search_forward(
    string: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(string, bound, noerror, count, 1, false, false)
}

#[inline(always)]
pub fn re_search_backward(
    regexp: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(regexp, bound, noerror, count, -1, true, false)
}

#[inline(always)]
pub fn re_search_forward(
    regexp: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(regexp, bound, noerror, count, 1, true, false)
}

#[inline(always)]
pub fn posix_search_backward(
    regexp: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(regexp, bound, noerror, count, -1, true, true)
}

#[inline(always)]
pub fn posix_search_forward(
    regexp: Lisp_Object,
    bound: Lisp_Object,
    noerror: Lisp_Object,
    count: Lisp_Object,
) -> Lisp_Object {
    search_command(regexp, bound, noerror, count, 1, true, true)
}

pub fn match_limit(num: Lisp_Object, beginningp: bool) -> Lisp_Object {
    CHECK_FIXNUM(num);
    let n = XFIXNUM(num);
    if n < 0 {
        args_out_of_range(num, make_fixnum(0));
    }
    let num_regs = crate::search::search_regs_num_regs();
    if num_regs <= 0 {
        crate::search::error_no_match_data();
    }
    if n >= num_regs {
        return Qnil();
    }
    let (start, end) = crate::search::search_regs_pair(n);
    if start < 0 {
        return Qnil();
    }
    make_fixnum(if beginningp { start } else { end })
}

#[inline(always)]
pub fn match_beginning(subexp: Lisp_Object) -> Lisp_Object {
    match_limit(subexp, true)
}

#[inline(always)]
pub fn match_end(subexp: Lisp_Object) -> Lisp_Object {
    match_limit(subexp, false)
}

#[inline(always)]
fn build_list_from_slice(items: &[Lisp_Object]) -> Lisp_Object {
    let mut acc = Qnil();
    let mut idx = items.len();
    while idx > 0 {
        idx -= 1;
        acc = Fcons(items[idx], acc);
    }
    acc
}

pub fn match_data(
    integers: Lisp_Object,
    mut reuse: Lisp_Object,
    reseat: Lisp_Object,
) -> Lisp_Object {
    if !NILP(reseat) {
        let mut tail = reuse;
        while CONSP(tail) {
            let car = XCAR(tail);
            if MARKERP(car) {
                unchain_marker_obj(car);
                XSETCAR(tail, Qnil());
            }
            tail = XCDR(tail);
        }
    }

    let searched = crate::search::last_thing_searched_get();
    if NILP(searched) {
        return Qnil();
    }

    let num_regs = crate::search::search_regs_num_regs().max(0);
    let mut data = RootedVec::filled((2 * num_regs + 1) as usize, Qnil());
    let mut len: usize = 0;

    let mut i: ptrdiff_t = 0;
    while i < num_regs {
        let (start, end) = crate::search::search_regs_pair(i);
        let slot = (2 * i) as usize;
        if start >= 0 {
            let cur_searched = crate::search::last_thing_searched_get();
            if BASE_EQ(cur_searched, Qt()) || !NILP(integers) {
                data[slot] = make_fixnum(start);
                data[slot + 1] = make_fixnum(end);
            } else if BUFFERP(cur_searched) {
                let m0 = Fmake_marker();
                data[slot] = m0;
                Fset_marker(m0, make_fixnum(start), cur_searched);
                let m1 = Fmake_marker();
                data[slot + 1] = m1;
                Fset_marker(m1, make_fixnum(end), cur_searched);
            } else {
                emacs_abort();
            }
            len = slot + 2;
        } else {
            data[slot] = Qnil();
            data[slot + 1] = Qnil();
        }
        i += 1;
    }

    let final_searched = crate::search::last_thing_searched_get();
    if BUFFERP(final_searched) && !NILP(integers) {
        data[len] = final_searched;
        len += 1;
    }

    if !CONSP(reuse) {
        reuse = build_list_from_slice(&data[..len]);
    } else {
        let mut prev = Qnil();
        let mut idx: usize = 0;
        let mut tail = reuse;
        while CONSP(tail) {
            if idx < len {
                XSETCAR(tail, data[idx]);
            } else {
                XSETCAR(tail, Qnil());
            }
            prev = tail;
            idx += 1;
            tail = XCDR(tail);
        }
        if idx < len {
            XSETCDR(prev, build_list_from_slice(&data[idx..len]));
        }
    }

    reuse
}

pub fn set_match_data(mut list: Lisp_Object, reseat: Lisp_Object) -> Lisp_Object {
    if running_asynch_code.get() {
        crate::search::maybe_save_search_regs_for_async();
    }

    CHECK_LIST(list);
    crate::search::last_thing_searched_set(Qt());

    let length: ptrdiff_t = crate::safe_data::list_length_impl(list) / 2;
    crate::search::search_regs_ensure_capacity(length);

    let mut i: ptrdiff_t = 0;
    while CONSP(list) {
        let mut marker = XCAR(list);
        if BUFFERP(marker) {
            crate::search::last_thing_searched_set(marker);
            break;
        }
        if i >= length {
            break;
        }
        if NILP(marker) {
            crate::search::search_regs_set_unmatched(i);
            list = XCDR(list);
        } else {
            let m0 = marker;
            if MARKERP(marker) {
                let mbuf = Fmarker_buffer(marker);
                if NILP(mbuf) {
                    marker = make_fixnum(0);
                } else {
                    crate::search::last_thing_searched_set(mbuf);
                }
            }
            CHECK_FIXNUM_COERCE_MARKER!(marker);
            let from = marker;

            if !NILP(reseat) && MARKERP(m0) {
                unchain_marker_obj(m0);
                XSETCAR(list, Qnil());
            }

            list = XCDR(list);
            if !CONSP(list) {
                break;
            }

            marker = XCAR(list);
            let m1 = marker;
            if MARKERP(marker) && NILP(Fmarker_buffer(marker)) {
                marker = make_fixnum(0);
            }
            CHECK_FIXNUM_COERCE_MARKER!(marker);

            let from_val = XFIXNUM(from);
            let end_val = XFIXNUM(marker);
            if (PTRDIFF_MIN..=PTRDIFF_MAX).contains(&from_val)
                && (PTRDIFF_MIN..=PTRDIFF_MAX).contains(&end_val)
            {
                crate::search::search_regs_set_pair(i, from_val, end_val);
            } else {
                crate::search::search_regs_set_unmatched(i);
            }

            if !NILP(reseat) && MARKERP(m1) {
                unchain_marker_obj(m1);
                XSETCAR(list, Qnil());
            }
        }
        list = XCDR(list);
        i += 1;
    }

    crate::search::search_regs_clear_from(i);
    Qnil()
}

pub fn match_data_translate(n: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(n);
    let delta = XFIXNUM(n);
    if !NILP(crate::search::last_thing_searched_get()) {
        crate::search::search_regs_translate_all(delta);
    }
    Qnil()
}

pub fn regexp_quote(string: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(string);
    let nbytes = SBYTES(string) as usize;
    let mut out: Vec<u8> = Vec::with_capacity(nbytes);
    let mut backslashes_added: ptrdiff_t = 0;

    let mut idx: usize = 0;
    while idx < nbytes {
        let b = SREF(string, idx);
        if matches!(b, b'[' | b'*' | b'.' | b'\\' | b'?' | b'+' | b'^' | b'$') {
            out.push(b'\\');
            backslashes_added += 1;
        }
        out.push(b);
        idx += 1;
    }

    if backslashes_added > 0 {
        make_specified_string_bytes(
            &out,
            SCHARS(string) + backslashes_added,
            STRING_MULTIBYTE(string),
        )
    } else {
        string
    }
}
