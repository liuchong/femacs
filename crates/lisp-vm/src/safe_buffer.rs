//! `#![forbid(unsafe_code)]` high-level buffer lookup, point/region/field/line motion, and
//! character inspection primitives (`src/buffer.rs` + `src/editfns.rs`, `U7` / `P-486`).
//!
//! Low-level `struct buffer` / `buffer_text` gap allocation, overlay interval trees, per-buffer
//! forwarding slots, and OS user/time FFI stay behind the controlled boundary in `src/buffer.rs`,
//! `src/editfns.rs`, and `src/include/buffer.rs`.

#![forbid(unsafe_code)]

use crate::common::*;

// ---------------------------------------------------------------------------
// `src/buffer.rs` — buffer lookup, state queries, and selection
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn buffer_live_p(object: Lisp_Object) -> Lisp_Object {
    if buffer_live_p_obj(object) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn buffer_list(frame: Lisp_Object) -> Lisp_Object {
    let mut general = Fmapcar(Qcdr(), Vbuffer_alist.get());
    if FRAMEP(frame) {
        let (raw_flist, raw_buried) = frame_buffer_lists_safe(frame);
        let framelist = crate::safe_data::copy_sequence(raw_flist);
        let prevlist =
            crate::safe_data::nreverse_subr(crate::safe_data::copy_sequence(raw_buried));

        let mut tail = framelist;
        while CONSP(tail) {
            general = crate::safe_data::delq_subr(XCAR(tail), general);
            tail = XCDR(tail);
        }
        tail = prevlist;
        while CONSP(tail) {
            general = crate::safe_data::delq_subr(XCAR(tail), general);
            tail = XCDR(tail);
        }
        let mut lists = [framelist, general, prevlist];
        Fnconc_slice(&mut lists)
    } else {
        general
    }
}

pub fn assoc_ignore_text_properties(key: Lisp_Object, list: Lisp_Object) -> Lisp_Object {
    let mut tail = list;
    while CONSP(tail) {
        let elt = XCAR(tail);
        if !NILP(crate::safe_data::string_equal(
            crate::safe_data::car_subr(elt),
            key,
        )) {
            return elt;
        }
        tail = XCDR(tail);
    }
    Qnil()
}

pub fn get_buffer(buffer_or_name: Lisp_Object) -> Lisp_Object {
    if BUFFERP(buffer_or_name) {
        return buffer_or_name;
    }
    CHECK_STRING(buffer_or_name);
    crate::safe_data::cdr_subr(assoc_ignore_text_properties(
        buffer_or_name,
        Vbuffer_alist.get(),
    ))
}

pub fn get_file_buffer(mut filename: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(filename);
    filename = Fexpand_file_name(filename, Qnil());

    let handler = Ffind_file_name_handler(filename, Qget_file_buffer());
    if !NILP(handler) {
        let handled_buf = calln!(handler, Qget_file_buffer(), filename);
        return if BUFFERP(handled_buf) {
            handled_buf
        } else {
            Qnil()
        };
    }

    let mut tail = Vbuffer_alist.get();
    while CONSP(tail) {
        let buf = XCDR(XCAR(tail));
        let slot = buffer_filename_slot_obj(buf);
        if STRINGP(slot) && !NILP(crate::safe_data::string_equal(slot, filename)) {
            return buf;
        }
        tail = XCDR(tail);
    }
    Qnil()
}

pub fn get_truename_buffer(filename: Lisp_Object) -> Lisp_Object {
    let mut tail = Vbuffer_alist.get();
    while CONSP(tail) {
        let buf = XCDR(XCAR(tail));
        let truename = buffer_file_truename_obj(buf);
        if STRINGP(truename) && !NILP(crate::safe_data::string_equal(truename, filename)) {
            return buf;
        }
        tail = XCDR(tail);
    }
    Qnil()
}

pub fn find_buffer(variable: Lisp_Object, value: Lisp_Object) -> Lisp_Object {
    let mut tail = Vbuffer_alist.get();
    while CONSP(tail) {
        let buf = XCDR(XCAR(tail));
        if !NILP(Fequal(value, Fbuffer_local_value(variable, buf))) {
            return buf;
        }
        tail = XCDR(tail);
    }
    Qnil()
}

#[inline(always)]
pub fn buffer_name(buffer: Lisp_Object) -> Lisp_Object {
    buffer_name_obj(buffer)
}

#[inline(always)]
pub fn buffer_last_name(buffer: Lisp_Object) -> Lisp_Object {
    crate::buffer::buffer_last_name_obj(buffer)
}

#[inline(always)]
pub fn buffer_file_name(buffer: Lisp_Object) -> Lisp_Object {
    buffer_file_name_obj(buffer)
}

#[inline(always)]
pub fn buffer_base_buffer(buffer: Lisp_Object) -> Lisp_Object {
    crate::buffer::decode_buffer_base_or_nil(buffer)
}

#[inline(always)]
pub fn buffer_modified_p(buffer: Lisp_Object) -> Lisp_Object {
    crate::buffer::buffer_modified_p_decoded(buffer)
}

#[inline(always)]
pub fn set_buffer_modified_p(flag: Lisp_Object) -> Lisp_Object {
    Frestore_buffer_modified_p(flag);
    Fforce_mode_line_update(Qnil())
}

#[inline(always)]
pub fn buffer_modified_tick(buffer: Lisp_Object) -> Lisp_Object {
    crate::buffer::buffer_modified_tick_integer(buffer)
}

#[inline(always)]
pub fn internal_set_buffer_modified_tick(tick: Lisp_Object, buffer: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(tick);
    crate::buffer::set_buffer_modified_tick_safe(buffer, XFIXNUM(tick));
    Qnil()
}

#[inline(always)]
pub fn buffer_chars_modified_tick(buffer: Lisp_Object) -> Lisp_Object {
    crate::buffer::buffer_chars_modified_tick_integer(buffer)
}

pub fn other_buffer(
    buffer: Lisp_Object,
    visible_ok: Lisp_Object,
    frame: Lisp_Object,
) -> Lisp_Object {
    let (mut tail, pred) = crate::buffer::frame_buffer_list_and_pred_safe(frame);
    let mut notsogood = Qnil();

    while CONSP(tail) {
        let buf = XCAR(tail);
        if crate::buffer::candidate_buffer_safe(buf, buffer)
            && (NILP(pred) || !NILP(calln!(pred, buf)))
        {
            if !NILP(visible_ok) || NILP(Fget_buffer_window(buf, Qvisible())) {
                return buf;
            } else if NILP(notsogood) {
                notsogood = buf;
            }
        }
        tail = XCDR(tail);
    }

    tail = Vbuffer_alist.get();
    while CONSP(tail) {
        let buf = XCDR(XCAR(tail));
        if crate::buffer::candidate_buffer_safe(buf, buffer)
            && (NILP(pred) || !NILP(calln!(pred, buf)))
        {
            if !NILP(visible_ok) || NILP(Fget_buffer_window(buf, Qvisible())) {
                return buf;
            } else if NILP(notsogood) {
                notsogood = buf;
            }
        }
        tail = XCDR(tail);
    }

    if !NILP(notsogood) {
        notsogood
    } else {
        safe_calln!(Qget_scratch_buffer_create())
    }
}

pub fn other_buffer_safely_impl(buffer: Lisp_Object) -> Lisp_Object {
    let mut tail = Vbuffer_alist.get();
    while CONSP(tail) {
        let buf = XCDR(XCAR(tail));
        if crate::buffer::candidate_buffer_safe(buf, buffer) {
            return buf;
        }
        tail = XCDR(tail);
    }

    let mut buf = safe_calln!(Qget_scratch_buffer_create());
    if NILP(buf) {
        let scratch = make_specified_string_bytes(b"*scratch*", 9, false);
        buf = Fget_buffer_create(scratch, Qnil());
        Fset_buffer_major_mode(buf);
    }
    buf
}

pub fn buffer_enable_undo(buffer: Lisp_Object) -> Lisp_Object {
    let real_buffer = if NILP(buffer) {
        current_buffer_subr()
    } else {
        let b = get_buffer(buffer);
        if NILP(b) {
            nsberror_safe(buffer);
        }
        b
    };
    crate::buffer::enable_buffer_undo_obj(real_buffer);
    Qnil()
}

#[inline(always)]
pub fn current_buffer_subr() -> Lisp_Object {
    crate::buffer::current_buffer_object_safe()
}

pub fn set_buffer_subr(buffer_or_name: Lisp_Object) -> Lisp_Object {
    let buffer = get_buffer(buffer_or_name);
    if NILP(buffer) {
        nsberror_safe(buffer_or_name);
    }
    if !buffer_live_p_obj(buffer) {
        error_selecting_deleted_buffer();
    }
    set_buffer_internal_obj(buffer);
    buffer
}

pub fn barf_if_buffer_read_only(mut position: Lisp_Object) -> Lisp_Object {
    if NILP(position) {
        position = make_fixnum(current_buffer_pt());
    } else {
        CHECK_FIXNUM(position);
    }

    if !NILP(current_buffer_read_only_val())
        && NILP(Vinhibit_read_only.get())
        && NILP(Fget_text_property(position, Qinhibit_read_only(), Qnil()))
    {
        xsignal1(Qbuffer_read_only(), current_buffer_subr());
    }
    Qnil()
}

// ---------------------------------------------------------------------------
// `src/editfns.rs` — string/char conversions, point, region, fields, & lines
// ---------------------------------------------------------------------------

pub fn char_to_string(character: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(character);
    let c = XFIXNAT(character) as c_int;
    let (bytes, len) = encode_char_multibyte_buf(c);
    make_string_from_bytes_safe(&bytes[..len], 1)
}

pub fn byte_to_string(byte: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(byte);
    let v = XFIXNUM(byte);
    if !(0..=255).contains(&v) {
        crate::editfns::error_invalid_byte();
    }
    let buf = [v as u8];
    make_specified_string_bytes(&buf, 1, false)
}

#[inline(always)]
pub fn string_to_char(string: Lisp_Object) -> Lisp_Object {
    CHECK_STRING(string);
    make_fixnum(lisp_string_first_char_or_zero(string) as EMACS_INT)
}

#[inline(always)]
pub fn point_subr() -> Lisp_Object {
    make_fixnum(current_buffer_pt())
}

#[inline(always)]
pub fn point_marker() -> Lisp_Object {
    make_current_buffer_point_marker()
}

pub fn goto_char(position: Lisp_Object) -> Lisp_Object {
    if MARKERP(position) {
        set_point_from_marker(position);
    } else if FIXNUMP(position) {
        let clamped = clip_to_bounds(
            current_buffer_begv(),
            XFIXNUM(position),
            current_buffer_zv(),
        );
        SET_PT(clamped);
    } else {
        wrong_type_argument(Qinteger_or_marker_p(), position);
    }
    position
}

pub fn region_limit(beginningp: bool) -> Lisp_Object {
    if !NILP(Vtransient_mark_mode.get())
        && NILP(Vmark_even_if_inactive.get())
        && NILP(current_buffer_mark_active_obj())
    {
        xsignal0(Qmark_inactive());
    }

    let m = Fmarker_position(current_buffer_mark_obj());
    if NILP(m) {
        crate::editfns::error_mark_not_set();
    }

    let pt = current_buffer_pt();
    let mark_pos = XFIXNAT(m);
    make_fixnum(if (pt < mark_pos) == beginningp {
        pt
    } else {
        clip_to_bounds(current_buffer_begv(), mark_pos, current_buffer_zv())
    })
}

#[inline(always)]
pub fn region_beginning() -> Lisp_Object {
    region_limit(true)
}

#[inline(always)]
pub fn region_end() -> Lisp_Object {
    region_limit(false)
}

#[inline(always)]
pub fn mark_marker() -> Lisp_Object {
    current_buffer_mark_obj()
}

pub fn find_field_safe(
    mut pos: Lisp_Object,
    merge_at_boundary: Lisp_Object,
    beg_limit: Lisp_Object,
    want_beg: bool,
    end_limit: Lisp_Object,
    want_end: bool,
) -> (ptrdiff_t, ptrdiff_t) {
    let mut at_field_start = false;
    let mut at_field_end = false;

    if NILP(pos) {
        pos = make_fixnum(current_buffer_pt());
    } else {
        CHECK_FIXNUM_COERCE_MARKER!(pos);
    }

    let after_field = get_char_property_only(pos, Qfield(), Qnil());
    let before_field = if XFIXNAT(pos) > current_buffer_begv() {
        get_char_property_only(make_fixnum(XFIXNUM(pos) - 1), Qfield(), Qnil())
    } else {
        after_field
    };

    if NILP(merge_at_boundary) {
        let field = Fget_pos_property(pos, Qfield(), Qnil());
        if !EQ(field, after_field) {
            at_field_end = true;
        }
        if !EQ(field, before_field) {
            at_field_start = true;
        }
        if NILP(field) && at_field_start && at_field_end {
            at_field_end = false;
            at_field_start = false;
        }
    }

    let beg = if want_beg {
        if at_field_start {
            XFIXNAT(pos)
        } else {
            let mut p = pos;
            if !NILP(merge_at_boundary) && EQ(before_field, Qboundary()) {
                p = Fprevious_single_char_property_change(p, Qfield(), Qnil(), beg_limit);
            }
            p = Fprevious_single_char_property_change(p, Qfield(), Qnil(), beg_limit);
            if NILP(p) {
                current_buffer_begv()
            } else {
                XFIXNAT(p)
            }
        }
    } else {
        0
    };

    let end = if want_end {
        if at_field_end {
            XFIXNAT(pos)
        } else {
            if !NILP(merge_at_boundary) && EQ(after_field, Qboundary()) {
                pos = Fnext_single_char_property_change(pos, Qfield(), Qnil(), end_limit);
            }
            pos = Fnext_single_char_property_change(pos, Qfield(), Qnil(), end_limit);
            if NILP(pos) {
                current_buffer_zv()
            } else {
                XFIXNAT(pos)
            }
        }
    } else {
        0
    };

    (beg, end)
}

pub fn delete_field(pos: Lisp_Object) -> Lisp_Object {
    let (beg, end) = find_field_safe(pos, Qnil(), Qnil(), true, Qnil(), true);
    if beg != end {
        del_range(beg, end);
    }
    Qnil()
}

pub fn field_string(pos: Lisp_Object) -> Lisp_Object {
    let (beg, end) = find_field_safe(pos, Qnil(), Qnil(), true, Qnil(), true);
    make_buffer_string(beg, end, true)
}

pub fn field_string_no_properties(pos: Lisp_Object) -> Lisp_Object {
    let (beg, end) = find_field_safe(pos, Qnil(), Qnil(), true, Qnil(), true);
    make_buffer_string(beg, end, false)
}

pub fn field_beginning(
    pos: Lisp_Object,
    escape_from_edge: Lisp_Object,
    limit: Lisp_Object,
) -> Lisp_Object {
    let (beg, _) = find_field_safe(pos, escape_from_edge, limit, true, Qnil(), false);
    make_fixnum(beg)
}

pub fn field_end(
    pos: Lisp_Object,
    escape_from_edge: Lisp_Object,
    limit: Lisp_Object,
) -> Lisp_Object {
    let (_, end) = find_field_safe(pos, escape_from_edge, Qnil(), false, limit, true);
    make_fixnum(end)
}

pub fn constrain_to_field(
    mut new_pos: Lisp_Object,
    mut old_pos: Lisp_Object,
    escape_from_edge: Lisp_Object,
    only_in_line: Lisp_Object,
    inhibit_capture_property: Lisp_Object,
) -> Lisp_Object {
    let mut orig_point: ptrdiff_t = 0;
    if NILP(new_pos) {
        let pt = current_buffer_pt();
        orig_point = pt;
        new_pos = make_fixnum(pt);
    }

    CHECK_FIXNUM_COERCE_MARKER!(new_pos);
    CHECK_FIXNUM_COERCE_MARKER!(old_pos);

    let fwd = XFIXNUM(new_pos) > XFIXNUM(old_pos);
    let prev_old = make_fixnum(XFIXNUM(old_pos) - 1);
    let prev_new = make_fixnum(XFIXNUM(new_pos) - 1);
    let begv = current_buffer_begv();

    if NILP(Vinhibit_field_text_motion.get())
        && !BASE_EQ(new_pos, old_pos)
        && (!NILP(Fget_char_property(new_pos, Qfield(), Qnil()))
            || !NILP(Fget_char_property(old_pos, Qfield(), Qnil()))
            || (XFIXNAT(new_pos) > begv
                && !NILP(Fget_char_property(prev_new, Qfield(), Qnil())))
            || (XFIXNAT(old_pos) > begv
                && !NILP(Fget_char_property(prev_old, Qfield(), Qnil()))))
        && (NILP(inhibit_capture_property)
            || (NILP(Fget_pos_property(old_pos, inhibit_capture_property, Qnil()))
                && (XFIXNAT(old_pos) <= current_buffer_begv()
                    || NILP(Fget_char_property(old_pos, inhibit_capture_property, Qnil()))
                    || NILP(Fget_char_property(
                        prev_old,
                        inhibit_capture_property,
                        Qnil(),
                    )))))
    {
        let field_bound = if fwd {
            field_end(old_pos, escape_from_edge, new_pos)
        } else {
            field_beginning(old_pos, escape_from_edge, new_pos)
        };

        let should_constrain = if XFIXNAT(field_bound) < XFIXNAT(new_pos) {
            fwd
        } else {
            !fwd
        };
        if should_constrain
            && (NILP(only_in_line)
                || find_newline_counted_safe(
                    XFIXNAT(new_pos),
                    XFIXNAT(field_bound),
                    if fwd { -1 } else { 1 },
                ) == 0)
        {
            new_pos = field_bound;
        }

        if orig_point != 0 && XFIXNAT(new_pos) != orig_point {
            SET_PT(XFIXNAT(new_pos));
        }
    }

    new_pos
}

fn bol_safe(n: Lisp_Object) -> (ptrdiff_t, ptrdiff_t) {
    let count: ptrdiff_t = if NILP(n) {
        0
    } else if FIXNUMP(n) {
        clip_to_bounds(-BUF_BYTES_MAX, XFIXNUM(n) - 1, BUF_BYTES_MAX)
    } else {
        CHECK_INTEGER(n);
        if NILP(crate::safe_data::natnump(n)) {
            -BUF_BYTES_MAX
        } else {
            BUF_BYTES_MAX
        }
    };
    let (charpos, _) = scan_newline_from_point_safe(count);
    (charpos, count)
}

#[inline(always)]
pub fn pos_bol(n: Lisp_Object) -> Lisp_Object {
    let (charpos, _) = bol_safe(n);
    make_fixnum(charpos)
}

pub fn line_beginning_position(n: Lisp_Object) -> Lisp_Object {
    let (charpos, count) = bol_safe(n);
    constrain_to_field(
        make_fixnum(charpos),
        make_fixnum(current_buffer_pt()),
        if count != 0 { Qt() } else { Qnil() },
        Qt(),
        Qnil(),
    )
}

fn eol_safe(n: Lisp_Object) -> ptrdiff_t {
    let count: ptrdiff_t = if NILP(n) {
        1
    } else if FIXNUMP(n) {
        clip_to_bounds(-BUF_BYTES_MAX, XFIXNUM(n), BUF_BYTES_MAX)
    } else {
        CHECK_INTEGER(n);
        if NILP(crate::safe_data::natnump(n)) {
            -BUF_BYTES_MAX
        } else {
            BUF_BYTES_MAX
        }
    };
    find_before_next_newline_safe(
        current_buffer_pt(),
        0,
        count - (count <= 0) as ptrdiff_t,
    )
}

#[inline(always)]
pub fn pos_eol(n: Lisp_Object) -> Lisp_Object {
    make_fixnum(eol_safe(n))
}

pub fn line_end_position(n: Lisp_Object) -> Lisp_Object {
    let charpos = eol_safe(n);
    constrain_to_field(
        make_fixnum(charpos),
        make_fixnum(current_buffer_pt()),
        Qnil(),
        Qt(),
        Qnil(),
    )
}

pub fn save_excursion(args: Lisp_Object) -> Lisp_Object {
    let count = SPECPDL_INDEX();
    record_unwind_protect_excursion_safe();
    let val = Fprogn(args);
    unbind_to(count, val)
}

pub fn save_current_buffer(args: Lisp_Object) -> Lisp_Object {
    let count = SPECPDL_INDEX();
    record_unwind_current_buffer_safe();
    unbind_to(count, Fprogn(args))
}

#[inline(always)]
pub fn buffer_size(buffer: Lisp_Object) -> Lisp_Object {
    if NILP(buffer) {
        make_fixnum(current_buffer_total_chars())
    } else {
        make_fixnum(buffer_total_chars_obj(buffer))
    }
}

#[inline(always)]
pub fn point_min() -> Lisp_Object {
    make_fixnum(current_buffer_begv())
}

#[inline(always)]
pub fn point_min_marker() -> Lisp_Object {
    make_current_buffer_marker_at(current_buffer_begv(), current_buffer_begv_byte())
}

#[inline(always)]
pub fn point_max() -> Lisp_Object {
    make_fixnum(current_buffer_zv())
}

#[inline(always)]
pub fn point_max_marker() -> Lisp_Object {
    make_current_buffer_marker_at(current_buffer_zv(), current_buffer_zv_byte())
}

#[inline(always)]
pub fn gap_position() -> Lisp_Object {
    make_fixnum(current_buffer_gpt())
}

#[inline(always)]
pub fn gap_size() -> Lisp_Object {
    make_fixnum(current_buffer_gap_size())
}

pub fn position_bytes(position: Lisp_Object) -> Lisp_Object {
    let pos = fix_position_safe(position);
    if !((BEG as EMACS_INT) <= pos && pos <= current_buffer_z()) {
        return Qnil();
    }
    make_fixnum(current_buffer_char_to_byte(pos))
}

pub fn byte_to_position(bytepos: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(bytepos);
    let mut pos_byte = XFIXNUM(bytepos);
    let z_byte = current_buffer_z_byte();
    if pos_byte < BEG_BYTE as ptrdiff_t || pos_byte > z_byte {
        return Qnil();
    }
    if current_buffer_z() != z_byte {
        while !CHAR_HEAD_P(current_buffer_fetch_byte(pos_byte) as c_int) {
            pos_byte -= 1;
        }
    }
    make_fixnum(current_buffer_byte_to_char(pos_byte))
}

pub fn following_char() -> Lisp_Object {
    if current_buffer_pt() >= current_buffer_zv() {
        make_fixnum(0)
    } else {
        make_fixnum(current_buffer_fetch_char(current_buffer_pt_byte()) as EMACS_INT)
    }
}

pub fn previous_char() -> Lisp_Object {
    if current_buffer_pt() <= current_buffer_begv() {
        make_fixnum(0)
    } else if current_buffer_multibyte_p() {
        let mut pos = current_buffer_pt_byte();
        pos -= current_buffer_prev_char_len(pos) as ptrdiff_t;
        make_fixnum(current_buffer_fetch_char(pos) as EMACS_INT)
    } else {
        make_fixnum(current_buffer_fetch_byte(current_buffer_pt_byte() - 1) as EMACS_INT)
    }
}

#[inline(always)]
pub fn bobp() -> Lisp_Object {
    if current_buffer_pt() == current_buffer_begv() {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn eobp() -> Lisp_Object {
    if current_buffer_pt() == current_buffer_zv() {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn bolp() -> Lisp_Object {
    if current_buffer_pt() == current_buffer_begv()
        || current_buffer_fetch_byte(current_buffer_pt_byte() - 1) == b'\n'
    {
        Qt()
    } else {
        Qnil()
    }
}

#[inline(always)]
pub fn eolp() -> Lisp_Object {
    if current_buffer_pt() == current_buffer_zv()
        || current_buffer_fetch_byte(current_buffer_pt_byte()) == b'\n'
    {
        Qt()
    } else {
        Qnil()
    }
}

pub fn char_after(pos: Lisp_Object) -> Lisp_Object {
    let begv = current_buffer_begv();
    let zv = current_buffer_zv();
    let begv_byte = current_buffer_begv_byte();
    let zv_byte = current_buffer_zv_byte();

    let pos_byte = if NILP(pos) {
        let pb = current_buffer_pt_byte();
        if pb < begv_byte || pb >= zv_byte {
            return Qnil();
        }
        pb
    } else if MARKERP(pos) {
        let pb = crate::marker::marker_byte_position_safe(pos);
        if pb < begv_byte || pb >= zv_byte {
            return Qnil();
        }
        pb
    } else {
        let p = fix_position_safe(pos);
        if !(begv <= p && p < zv) {
            return Qnil();
        }
        current_buffer_char_to_byte(p)
    };

    make_fixnum(current_buffer_fetch_char(pos_byte) as EMACS_INT)
}

pub fn char_before(mut pos: Lisp_Object) -> Lisp_Object {
    let begv = current_buffer_begv();
    let zv = current_buffer_zv();
    let begv_byte = current_buffer_begv_byte();
    let zv_byte = current_buffer_zv_byte();

    let mut pos_byte: ptrdiff_t = 0;
    if NILP(pos) {
        pos_byte = current_buffer_pt_byte();
        pos = make_fixnum(current_buffer_pt());
    }

    if MARKERP(pos) {
        pos_byte = crate::marker::marker_byte_position_safe(pos);
        if pos_byte <= begv_byte || pos_byte > zv_byte {
            return Qnil();
        }
    } else {
        let p = fix_position_safe(pos);
        if !(begv < p && p <= zv) {
            return Qnil();
        }
        if pos_byte == 0 {
            pos_byte = current_buffer_char_to_byte(p);
        }
    }

    if current_buffer_multibyte_p() {
        pos_byte -= current_buffer_prev_char_len(pos_byte) as ptrdiff_t;
        make_fixnum(current_buffer_fetch_char(pos_byte) as EMACS_INT)
    } else {
        pos_byte -= 1;
        make_fixnum(current_buffer_fetch_byte(pos_byte) as EMACS_INT)
    }
}
