//! `#![forbid(unsafe_code)]` high-level window tree inspection, parameters, geometry, and cyclic
//! traversal primitives (`src/window.rs`, `U7` / `P-486`).
//!
//! Low-level `struct window` / `glyph_matrix` / `glyph_row` pointer layout, redisplay matrix
//! walks, and native frame bridges stay behind the controlled boundary in `src/window.rs` and
//! `src/include/window.rs`.

#![forbid(unsafe_code)]

use crate::common::*;

#[inline(always)]
pub fn windowp(object: Lisp_Object) -> Lisp_Object {
    if WINDOWP(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn window_valid_p(object: Lisp_Object) -> Lisp_Object {
    if WINDOW_VALID_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn window_live_p(object: Lisp_Object) -> Lisp_Object {
    if WINDOW_LIVE_P(object) { Qt() } else { Qnil() }
}

#[inline(always)]
pub fn window_frame(window: Lisp_Object) -> Lisp_Object {
    window_frame_obj(window)
}

#[inline(always)]
pub fn minibuffer_window(frame: Lisp_Object) -> Lisp_Object {
    minibuffer_window_for_frame_obj(frame)
}

#[inline(always)]
pub fn window_minibuffer_p(window: Lisp_Object) -> Lisp_Object {
    if window_minibuffer_p_safe(window) {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn selected_window_subr() -> Lisp_Object {
    selected_window.get()
}

#[inline(always)]
pub fn old_selected_window_subr() -> Lisp_Object {
    crate::window::old_selected_window.get()
}

#[inline(always)]
pub fn window_buffer(window: Lisp_Object) -> Lisp_Object {
    window_buffer_obj(window)
}

#[inline(always)]
pub fn window_parent(window: Lisp_Object) -> Lisp_Object {
    window_parent_obj(window)
}

#[inline(always)]
pub fn window_top_child(window: Lisp_Object) -> Lisp_Object {
    window_top_child_obj(window)
}

#[inline(always)]
pub fn window_left_child(window: Lisp_Object) -> Lisp_Object {
    window_left_child_obj(window)
}

#[inline(always)]
pub fn window_next_sibling(window: Lisp_Object) -> Lisp_Object {
    window_next_sibling_obj(window)
}

#[inline(always)]
pub fn window_prev_sibling(window: Lisp_Object) -> Lisp_Object {
    window_prev_sibling_obj(window)
}

#[inline(always)]
pub fn window_combination_limit(window: Lisp_Object) -> Lisp_Object {
    window_combination_limit_obj(window)
}

#[inline(always)]
pub fn set_window_combination_limit(window: Lisp_Object, limit: Lisp_Object) -> Lisp_Object {
    set_window_combination_limit_obj(window, limit)
}

#[inline(always)]
pub fn window_use_time(window: Lisp_Object) -> Lisp_Object {
    make_fixnum(window_use_time_safe(window))
}

#[inline(always)]
pub fn window_pixel_width(window: Lisp_Object) -> Lisp_Object {
    let (pw, _, _, _, _, _, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(pw)
}

#[inline(always)]
pub fn window_pixel_height(window: Lisp_Object) -> Lisp_Object {
    let (_, ph, _, _, _, _, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(ph)
}

#[inline(always)]
pub fn window_old_pixel_width(window: Lisp_Object) -> Lisp_Object {
    let (_, _, opw, _, _, _, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(opw)
}

#[inline(always)]
pub fn window_old_pixel_height(window: Lisp_Object) -> Lisp_Object {
    let (_, _, _, oph, _, _, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(oph)
}

#[inline(always)]
pub fn window_total_height(window: Lisp_Object, round: Lisp_Object) -> Lisp_Object {
    make_fixnum(window_total_height_safe(window, round))
}

#[inline(always)]
pub fn window_total_width(window: Lisp_Object, round: Lisp_Object) -> Lisp_Object {
    make_fixnum(window_total_width_safe(window, round))
}

#[inline(always)]
pub fn window_new_total(window: Lisp_Object) -> Lisp_Object {
    window_new_total_obj(window)
}

#[inline(always)]
pub fn window_normal_size(window: Lisp_Object, horizontal: Lisp_Object) -> Lisp_Object {
    window_normal_size_obj(window, horizontal)
}

#[inline(always)]
pub fn window_new_normal(window: Lisp_Object) -> Lisp_Object {
    window_new_normal_obj(window)
}

#[inline(always)]
pub fn window_new_pixel(window: Lisp_Object) -> Lisp_Object {
    window_new_pixel_obj(window)
}

#[inline(always)]
pub fn window_pixel_left(window: Lisp_Object) -> Lisp_Object {
    let (_, _, _, _, pl, _, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(pl)
}

#[inline(always)]
pub fn window_pixel_top(window: Lisp_Object) -> Lisp_Object {
    let (_, _, _, _, _, pt, _, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(pt)
}

#[inline(always)]
pub fn window_left_column(window: Lisp_Object) -> Lisp_Object {
    let (_, _, _, _, _, _, lc, _) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(lc)
}

#[inline(always)]
pub fn window_top_line(window: Lisp_Object) -> Lisp_Object {
    let (_, _, _, _, _, _, _, tl) = window_pixel_and_edge_dims_valid(window);
    make_fixnum(tl)
}

#[inline(always)]
pub fn window_point_subr(window: Lisp_Object) -> Lisp_Object {
    make_fixnum(window_point_safe(window))
}

#[inline(always)]
pub fn window_old_point(window: Lisp_Object) -> Lisp_Object {
    window_old_point_obj(window)
}

#[inline(always)]
pub fn window_start(window: Lisp_Object) -> Lisp_Object {
    window_start_pos_obj(window)
}

#[inline(always)]
pub fn window_hscroll(window: Lisp_Object) -> Lisp_Object {
    make_fixnum(window_hscroll_safe(window))
}

#[inline(always)]
pub fn window_dedicated_p(window: Lisp_Object) -> Lisp_Object {
    window_dedicated_p_obj(window)
}

#[inline(always)]
pub fn set_window_dedicated_p(window: Lisp_Object, flag: Lisp_Object) -> Lisp_Object {
    set_window_dedicated_p_obj(window, flag)
}

#[inline(always)]
pub fn window_prev_buffers(window: Lisp_Object) -> Lisp_Object {
    window_prev_buffers_obj(window)
}

#[inline(always)]
pub fn set_window_prev_buffers(window: Lisp_Object, prev_buffers: Lisp_Object) -> Lisp_Object {
    set_window_prev_buffers_obj(window, prev_buffers)
}

#[inline(always)]
pub fn window_next_buffers(window: Lisp_Object) -> Lisp_Object {
    window_next_buffers_obj(window)
}

#[inline(always)]
pub fn set_window_next_buffers(window: Lisp_Object, next_buffers: Lisp_Object) -> Lisp_Object {
    set_window_next_buffers_obj(window, next_buffers)
}

#[inline(always)]
pub fn window_parameters(window: Lisp_Object) -> Lisp_Object {
    window_parameters_obj(window)
}

#[inline(always)]
pub fn window_parameter_subr(window: Lisp_Object, parameter: Lisp_Object) -> Lisp_Object {
    window_parameter_safe(window, parameter)
}

#[inline(always)]
pub fn set_window_parameter(
    window: Lisp_Object,
    parameter: Lisp_Object,
    value: Lisp_Object,
) -> Lisp_Object {
    set_window_parameter_safe(window, parameter, value)
}

#[inline(always)]
pub fn window_display_table_subr(window: Lisp_Object) -> Lisp_Object {
    window_display_table_obj(window)
}

#[inline(always)]
pub fn set_window_display_table(window: Lisp_Object, table: Lisp_Object) -> Lisp_Object {
    set_window_display_table_obj(window, table)
}

pub fn next_window_impl(
    window: Lisp_Object,
    minibuf: Lisp_Object,
    all_frames: Lisp_Object,
    next_p: bool,
) -> Lisp_Object {
    let count = SPECPDL_INDEX();
    let (mut window, minibuf, all_frames) =
        crate::window::decode_next_window_args_safe(window, minibuf, all_frames);

    if FRAMEP(all_frames) && !EQ(all_frames, crate::window::window_raw_frame_obj(window)) {
        return Fframe_first_window(all_frames);
    }

    specbind(Qinhibit_quit(), Qt());

    if next_p {
        let mut list = crate::safe_data::memq_subr(window, crate::window::window_list_safe());
        if CONSP(list) {
            list = XCDR(list);
            while CONSP(list) {
                if crate::window::candidate_window_p_safe(XCAR(list), window, minibuf, all_frames) {
                    break;
                }
                list = XCDR(list);
            }
        }
        if !CONSP(list) {
            list = Vwindow_list.get();
            while CONSP(list) && !EQ(XCAR(list), window) {
                if crate::window::candidate_window_p_safe(XCAR(list), window, minibuf, all_frames) {
                    break;
                }
                list = XCDR(list);
            }
        }
        if CONSP(list) {
            window = XCAR(list);
        }
    } else {
        let mut candidate = Qnil();
        let mut list = crate::window::window_list_safe();
        while CONSP(list) {
            if EQ(XCAR(list), window) {
                if WINDOWP(candidate) {
                    break;
                }
            } else if crate::window::candidate_window_p_safe(
                XCAR(list),
                window,
                minibuf,
                all_frames,
            ) {
                candidate = XCAR(list);
            }
            list = XCDR(list);
        }
        if WINDOWP(candidate) {
            window = candidate;
        }
    }

    unbind_to(count, Qnil());
    window
}

#[inline(always)]
pub fn next_window_subr(
    window: Lisp_Object,
    minibuf: Lisp_Object,
    all_frames: Lisp_Object,
) -> Lisp_Object {
    next_window_impl(window, minibuf, all_frames, true)
}

#[inline(always)]
pub fn previous_window_subr(
    window: Lisp_Object,
    minibuf: Lisp_Object,
    all_frames: Lisp_Object,
) -> Lisp_Object {
    next_window_impl(window, minibuf, all_frames, false)
}

pub fn window_list_1_safe(
    window: Lisp_Object,
    minibuf: Lisp_Object,
    all_frames: Lisp_Object,
) -> Lisp_Object {
    let count = SPECPDL_INDEX();
    let (window, minibuf, all_frames) =
        crate::window::decode_next_window_args_safe(window, minibuf, all_frames);
    let mut list = Qnil();

    specbind(Qinhibit_quit(), Qt());

    let mut tail = crate::window::window_list_safe();
    while CONSP(tail) {
        if crate::window::candidate_window_p_safe(XCAR(tail), window, minibuf, all_frames) {
            list = Fcons(XCAR(tail), list);
        }
        tail = XCDR(tail);
    }

    list = crate::safe_data::nreverse_subr(list);
    let rest = crate::safe_data::memq_subr(window, list);
    if !NILP(rest) && !EQ(rest, list) {
        tail = list;
        while !EQ(XCDR(tail), rest) {
            tail = XCDR(tail);
        }
        XSETCDR(tail, Qnil());
        let mut pair = [rest, list];
        list = Fnconc_slice(&mut pair);
    }

    unbind_to(count, Qnil());
    list
}

pub fn window_list_subr(
    mut frame: Lisp_Object,
    minibuf: Lisp_Object,
    mut window: Lisp_Object,
) -> Lisp_Object {
    if NILP(window) {
        window = if FRAMEP(frame) {
            crate::window::frame_selected_window_obj(frame)
        } else {
            selected_window.get()
        };
    }
    CHECK_WINDOW(window);
    if NILP(frame) {
        frame = selected_frame.get();
    }
    if !EQ(frame, crate::window::window_raw_frame_obj(window)) {
        crate::window::error_window_different_frame();
    }
    window_list_1_safe(window, minibuf, frame)
}
