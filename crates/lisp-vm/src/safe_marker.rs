//! `#![forbid(unsafe_code)]` high-level Lisp marker primitives (`src/marker.rs`, `U7` / `P-486`).
//!
//! Low-level intrusive `Lisp_Marker` buffer-chain manipulation and byte/char position caches stay
//! behind the controlled kernel boundary in `src/marker.rs`.

#![forbid(unsafe_code)]

use crate::common::*;

#[inline(always)]
fn check_marker(x: Lisp_Object) {
    CHECK_TYPE(MARKERP(x), Qmarkerp(), x);
}

pub fn marker_buffer(marker: Lisp_Object) -> Lisp_Object {
    check_marker(marker);
    let buf = marker_buffer_object(marker);
    if !NILP(buf) {
        eassert!(buffer_live_p_obj(buf));
    }
    buf
}

pub fn marker_position_subr(marker: Lisp_Object) -> Lisp_Object {
    check_marker(marker);
    match marker_charpos_opt(marker) {
        Some(pos) => make_fixnum(pos),
        None => Qnil(),
    }
}

pub fn marker_last_position_subr(marker: Lisp_Object) -> Lisp_Object {
    check_marker(marker);
    make_fixnum(crate::marker::marker_raw_charpos(marker))
}

pub fn copy_marker(marker: Lisp_Object, r#type: Lisp_Object) -> Lisp_Object {
    if !NILP(marker) {
        CHECK_TYPE(
            FIXNUMP(marker) || MARKERP(marker),
            Qinteger_or_marker_p(),
            marker,
        );
    }

    let new = make_detached_marker();
    Fset_marker(
        new,
        marker,
        if MARKERP(marker) {
            marker_buffer(marker)
        } else {
            Qnil()
        },
    );
    set_marker_insertion_type_flag(new, !NILP(r#type));
    new
}

pub fn marker_insertion_type(marker: Lisp_Object) -> Lisp_Object {
    check_marker(marker);
    if marker_insertion_type_flag(marker) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn set_marker_insertion_type(marker: Lisp_Object, r#type: Lisp_Object) -> Lisp_Object {
    check_marker(marker);
    set_marker_insertion_type_flag(marker, !NILP(r#type));
    r#type
}

pub fn markers_in(mut beg: Lisp_Object, mut end: Lisp_Object) -> Lisp_Object {
    let begv = current_buffer_begv();
    let zv = current_buffer_zv();
    let ibeg = if NILP(beg) {
        begv
    } else {
        CHECK_FIXNUM_COERCE_MARKER!(beg);
        clip_to_bounds(begv, XFIXNUM(beg), zv)
    };
    let iend = if NILP(end) {
        zv
    } else {
        CHECK_FIXNUM_COERCE_MARKER!(end);
        clip_to_bounds(begv, XFIXNUM(end), zv)
    };
    current_buffer_markers_in_range(ibeg, iend)
}

#[inline(always)]
pub fn detach_marker_impl(marker: Lisp_Object) {
    Fset_marker(marker, Qnil(), Qnil());
}
