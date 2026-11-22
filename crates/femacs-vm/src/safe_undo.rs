//! `#![forbid(unsafe_code)]` implementation of buffer undo recording, boundaries,
//! and GC-time undo list truncation (`src/undo.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

#[inline(always)]
fn prepare_record() {
    if NILP(crate::undo::pending_boundary.get()) {
        crate::undo::pending_boundary.set(Fcons(Qnil(), Qnil()));
    }
}

fn record_point(beg: ptrdiff_t) {
    if undo_inhibit_record_point.get() {
        return;
    }

    let undo_list = current_buffer_undo_list();
    let at_boundary = !CONSP(undo_list) || NILP(XCAR(undo_list));

    if current_buffer_modiff() <= current_buffer_save_modiff() {
        record_first_change_impl();
    }

    if at_boundary
        && point_before_last_command_or_undo.get() != beg
        && same_buffer_as_last_command()
    {
        set_current_buffer_undo_list(Fcons(
            make_fixnum(point_before_last_command_or_undo.get()),
            current_buffer_undo_list(),
        ));
    }
}

pub fn record_insert_impl(beg: ptrdiff_t, length: ptrdiff_t) {
    if EQ(current_buffer_undo_list(), Qt()) {
        return;
    }

    prepare_record();
    record_point(beg);

    let undo_list = current_buffer_undo_list();
    if CONSP(undo_list) {
        let elt = XCAR(undo_list);
        if CONSP(elt) && FIXNUMP(XCAR(elt)) && FIXNUMP(XCDR(elt)) && XFIXNUM(XCDR(elt)) == beg {
            XSETCDR(elt, make_fixnum(beg + length));
            return;
        }
    }

    let lbeg = make_fixnum(beg);
    let lend = make_fixnum(beg + length);
    set_current_buffer_undo_list(Fcons(Fcons(lbeg, lend), current_buffer_undo_list()));
}

fn record_marker_adjustments(from: ptrdiff_t, to: ptrdiff_t) {
    prepare_record();

    let mut markers = RootedVec::new();
    let mut adjustments = Vec::new();
    collect_current_buffer_marker_adjustments(from, to, &mut markers, &mut adjustments);

    for i in 0..markers.len() {
        set_current_buffer_undo_list(Fcons(
            Fcons(markers[i], make_fixnum(adjustments[i])),
            current_buffer_undo_list(),
        ));
    }
}

pub fn record_delete_impl(beg: ptrdiff_t, string: Lisp_Object, record_markers: bool) {
    if EQ(current_buffer_undo_list(), Qt()) {
        return;
    }

    prepare_record();
    record_point(beg);

    let sbeg = if current_buffer_pt() == beg + SCHARS(string) {
        make_fixnum(-beg)
    } else {
        make_fixnum(beg)
    };

    if record_markers {
        record_marker_adjustments(beg, beg + SCHARS(string));
    }

    set_current_buffer_undo_list(Fcons(Fcons(string, sbeg), current_buffer_undo_list()));
}

pub fn record_change_impl(beg: ptrdiff_t, length: ptrdiff_t) {
    record_delete_impl(beg, make_buffer_string(beg, beg + length, true), false);
    record_insert_impl(beg, length);
}

pub fn record_first_change_impl() {
    if EQ(current_buffer_undo_list(), Qt()) {
        return;
    }

    let base_obj = current_buffer_base_or_self_object();
    let modtime = {
        let count = SPECPDL_INDEX();
        record_unwind_current_buffer_safe();
        set_buffer_internal_obj(base_obj);
        let t = current_buffer_visited_file_modtime();
        unbind_to(count, t)
    };

    set_current_buffer_undo_list(Fcons(Fcons(Qt(), modtime), current_buffer_undo_list()));
}

pub fn record_property_change_impl(
    beg: ptrdiff_t,
    length: ptrdiff_t,
    prop: Lisp_Object,
    value: Lisp_Object,
    buffer: Lisp_Object,
) {
    if EQ(buffer_undo_list_of(buffer), Qt()) {
        return;
    }

    prepare_record();

    if current_buffer_modiff() <= current_buffer_save_modiff() {
        record_first_change_impl();
    }

    let lbeg = make_fixnum(beg);
    let lend = make_fixnum(beg + length);
    let entry = Fcons(Qnil(), Fcons(prop, Fcons(value, Fcons(lbeg, lend))));
    set_current_buffer_undo_list(Fcons(entry, current_buffer_undo_list()));
}

pub fn undo_boundary_subr() -> Lisp_Object {
    let undo_list = current_buffer_undo_list();
    if EQ(undo_list, Qt()) {
        return Qnil();
    }
    let tem = Fcar(undo_list);
    if !NILP(tem) {
        let pending = crate::undo::pending_boundary.get();
        if !NILP(pending) {
            XSETCDR(pending, undo_list);
            set_current_buffer_undo_list(pending);
            crate::undo::pending_boundary.set(Qnil());
        } else {
            set_current_buffer_undo_list(Fcons(Qnil(), undo_list));
        }
    }

    Fset(Qundo_auto__last_boundary_cause(), Qexplicit());
    point_before_last_command_or_undo.set(current_buffer_pt());
    set_buffer_before_last_command_or_undo_current();

    Qnil()
}

pub fn truncate_undo_list_obj(buf_obj: Lisp_Object) {
    let count = inhibit_garbage_collection();

    record_unwind_current_buffer_safe();
    set_buffer_internal_obj(buf_obj);

    let list = buffer_undo_list_of(buf_obj);
    let mut prev = Qnil();
    let mut next = list;
    let mut last_boundary = Qnil();
    let mut size_so_far: intmax_t = 0;

    if CONSP(next) && NILP(XCAR(next)) {
        size_so_far += core::mem::size_of::<Lisp_Cons>() as intmax_t;
        prev = next;
        next = XCDR(next);
    }

    while CONSP(next) && !NILP(XCAR(next)) {
        let elt = XCAR(next);
        size_so_far += core::mem::size_of::<Lisp_Cons>() as intmax_t;
        if CONSP(elt) {
            size_so_far += core::mem::size_of::<Lisp_Cons>() as intmax_t;
            if STRINGP(XCAR(elt)) {
                size_so_far += (core::mem::size_of::<Lisp_String>() - 1
                    + SCHARS(XCAR(elt)) as usize) as intmax_t;
            }
        }
        prev = next;
        next = XCDR(next);
    }

    let outer = Vundo_outer_limit.get();
    let outer_exceeded = if INTEGERP(outer) {
        if let Some(lim) = integer_to_intmax_opt(outer) {
            lim < size_so_far
        } else {
            NILP(Fnatnump(outer))
        }
    } else {
        false
    };

    let outer_fn = Vundo_outer_limit_function.get();
    if outer_exceeded && !NILP(outer_fn) {
        let tem = calln!(outer_fn, make_int(size_so_far));
        if !NILP(tem) {
            unbind_to(count, Qnil());
            return;
        }
    }

    if CONSP(next) {
        last_boundary = prev;
    }

    while CONSP(next) {
        let elt = XCAR(next);
        if NILP(elt) {
            if size_so_far > undo_strong_limit.get() as intmax_t {
                break;
            }
            last_boundary = prev;
            if size_so_far > undo_limit.get() as intmax_t {
                break;
            }
        }

        size_so_far += core::mem::size_of::<Lisp_Cons>() as intmax_t;
        if CONSP(elt) {
            size_so_far += core::mem::size_of::<Lisp_Cons>() as intmax_t;
            if STRINGP(XCAR(elt)) {
                size_so_far += (core::mem::size_of::<Lisp_String>() - 1
                    + SCHARS(XCAR(elt)) as usize) as intmax_t;
            }
        }

        prev = next;
        next = XCDR(next);
    }

    if NILP(next) {
        // Whole list fits within limits.
    } else if !NILP(last_boundary) {
        XSETCDR(last_boundary, Qnil());
    } else {
        set_buffer_undo_list_of(buf_obj, Qnil());
    }

    unbind_to(count, Qnil());
}
