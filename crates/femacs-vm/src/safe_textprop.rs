//! `#![forbid(unsafe_code)]` implementation of high-level text-property query, navigation,
//! stickiness, and property-range list operations (`src/textprop.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

const SOFT: bool = false;

#[inline(always)]
fn resolve_object_or_current_buffer(object: Lisp_Object) -> Lisp_Object {
    if NILP(object) {
        Fcurrent_buffer()
    } else {
        object
    }
}

#[inline(always)]
fn object_end_pos(object: Lisp_Object) -> ptrdiff_t {
    if STRINGP(object) {
        SCHARS(object)
    } else {
        buffer_zv_obj(object)
    }
}

#[inline(always)]
fn object_beg_pos(object: Lisp_Object) -> ptrdiff_t {
    if STRINGP(object) {
        0
    } else {
        buffer_begv_obj(object)
    }
}

pub fn text_properties_at(position: Lisp_Object, object: Lisp_Object) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    let (Some(i), position) = validate_interval_single_pos(object, position, SOFT) else {
        return Qnil();
    };
    let (ipos, ilen) = interval_handle_pos_and_len(i);
    if XFIXNUM(position) == ilen + ipos {
        return Qnil();
    }
    interval_handle_plist(i)
}

pub fn get_text_property(
    position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    textget(text_properties_at(position, object), prop)
}

pub fn get_char_property(
    position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    get_char_property_only(position, prop, object)
}

pub fn get_char_property_and_overlay_subr(
    position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    let (val, overlay) = get_char_property_and_overlay_pair(position, prop, object);
    Fcons(val, overlay)
}

pub fn next_char_property_change(position: Lisp_Object, mut limit: Lisp_Object) -> Lisp_Object {
    let mut temp = Fnext_overlay_change(position);
    if !NILP(limit) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
        if XFIXNUM(limit) < XFIXNUM(temp) {
            temp = limit;
        }
    }
    next_property_change(position, Qnil(), temp)
}

pub fn previous_char_property_change(position: Lisp_Object, mut limit: Lisp_Object) -> Lisp_Object {
    let mut temp = Fprevious_overlay_change(position);
    if !NILP(limit) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
        if XFIXNUM(limit) > XFIXNUM(temp) {
            temp = limit;
        }
    }
    previous_property_change(position, Qnil(), temp)
}

pub fn next_single_char_property_change(
    mut position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    if STRINGP(object) {
        position = next_single_property_change(position, prop, object, limit);
        if NILP(position) {
            if NILP(limit) {
                position = make_fixnum(SCHARS(object));
            } else {
                CHECK_FIXNUM(limit);
                position = limit;
            }
        }
        return position;
    }

    let count = SPECPDL_INDEX();
    if !NILP(object) {
        CHECK_BUFFER(object);
    }
    if BUFFERP(object) && !current_buffer_is_same_obj(object) {
        record_unwind_current_buffer_safe();
        Fset_buffer(object);
    }

    CHECK_FIXNUM_COERCE_MARKER!(position);
    let initial_value = get_char_property(position, prop, object);

    if NILP(limit) {
        limit = make_fixnum(current_buffer_zv());
    } else {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    if XFIXNUM(position) >= XFIXNUM(limit) {
        position = limit;
        if XFIXNUM(position) > current_buffer_zv() {
            position = make_fixnum(current_buffer_zv());
        }
    } else {
        loop {
            position = next_char_property_change(position, limit);
            if XFIXNAT(position) >= XFIXNAT(limit) {
                position = limit;
                break;
            }
            let value = get_char_property(position, prop, object);
            if !EQ(value, initial_value) {
                break;
            }
            if XFIXNAT(position) >= current_buffer_zv() {
                break;
            }
        }
    }

    unbind_to(count, position)
}

pub fn previous_single_char_property_change(
    mut position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    if STRINGP(object) {
        position = previous_single_property_change(position, prop, object, limit);
        if NILP(position) {
            if NILP(limit) {
                position = make_fixnum(0);
            } else {
                CHECK_FIXNUM(limit);
                position = limit;
            }
        }
        return position;
    }

    let count = SPECPDL_INDEX();
    if !NILP(object) {
        CHECK_BUFFER(object);
    }
    if BUFFERP(object) && !current_buffer_is_same_obj(object) {
        record_unwind_current_buffer_safe();
        Fset_buffer(object);
    }

    CHECK_FIXNUM_COERCE_MARKER!(position);
    if NILP(limit) {
        limit = make_fixnum(current_buffer_begv());
    } else {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    if XFIXNUM(position) <= XFIXNUM(limit) {
        position = limit;
        if XFIXNUM(position) < current_buffer_begv() {
            position = make_fixnum(current_buffer_begv());
        }
    } else {
        let initial_value = get_char_property(
            make_fixnum(XFIXNUM(position) - (0 <= XFIXNUM(position)) as ptrdiff_t),
            prop,
            object,
        );
        loop {
            position = previous_char_property_change(position, limit);
            if XFIXNAT(position) <= XFIXNAT(limit) {
                position = limit;
                break;
            }
            let value = get_char_property(make_fixnum(XFIXNAT(position) - 1), prop, object);
            if !EQ(value, initial_value) {
                break;
            }
        }
    }

    unbind_to(count, position)
}

pub fn next_property_change(
    position: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    if !NILP(limit) && !EQ(limit, Qt()) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    let (i_opt, _) = validate_interval_single_pos(object, position, SOFT);
    if EQ(limit, Qt()) {
        let next = i_opt.and_then(next_interval_handle);
        return match next {
            None => make_fixnum(object_end_pos(object)),
            Some(nh) => make_fixnum(interval_handle_pos_and_len(nh).0),
        };
    }

    let Some(i) = i_opt else {
        return limit;
    };

    let mut next = next_interval_handle(i);
    while let Some(nh) = next {
        let (npos, _) = interval_handle_pos_and_len(nh);
        if intervals_equal_handle(i, nh) && (NILP(limit) || npos < XFIXNUM(limit)) {
            next = next_interval_handle(nh);
        } else {
            break;
        }
    }

    let stop = if FIXNUMP(limit) {
        XFIXNUM(limit)
    } else {
        object_end_pos(object)
    };
    match next {
        None => limit,
        Some(nh) => {
            let (npos, _) = interval_handle_pos_and_len(nh);
            if npos >= stop {
                limit
            } else {
                make_fixnum(npos)
            }
        }
    }
}

pub fn next_single_property_change(
    position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    if !NILP(limit) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    let (Some(i), _) = validate_interval_single_pos(object, position, SOFT) else {
        return limit;
    };

    let here_val = textget(interval_handle_plist(i), prop);
    let mut next = next_interval_handle(i);
    while let Some(nh) = next {
        let (npos, _) = interval_handle_pos_and_len(nh);
        if EQ(here_val, textget(interval_handle_plist(nh), prop))
            && (NILP(limit) || npos < XFIXNUM(limit))
        {
            next = next_interval_handle(nh);
        } else {
            break;
        }
    }

    let stop = if FIXNUMP(limit) {
        XFIXNUM(limit)
    } else {
        object_end_pos(object)
    };
    match next {
        None => limit,
        Some(nh) => {
            let (npos, _) = interval_handle_pos_and_len(nh);
            if npos >= stop {
                limit
            } else {
                make_fixnum(npos)
            }
        }
    }
}

pub fn previous_property_change(
    position: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    if !NILP(limit) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    let (Some(mut i), position) = validate_interval_single_pos(object, position, SOFT) else {
        return limit;
    };

    if interval_handle_pos_and_len(i).0 == XFIXNAT(position) {
        let Some(prev_i) = previous_interval_handle(i) else {
            return limit;
        };
        i = prev_i;
    }

    let mut previous = previous_interval_handle(i);
    while let Some(ph) = previous {
        let (ppos, plen) = interval_handle_pos_and_len(ph);
        if intervals_equal_handle(ph, i) && (NILP(limit) || ppos + plen > XFIXNUM(limit)) {
            previous = previous_interval_handle(ph);
        } else {
            break;
        }
    }

    let stop = if FIXNUMP(limit) {
        XFIXNUM(limit)
    } else {
        object_beg_pos(object)
    };
    match previous {
        None => limit,
        Some(ph) => {
            let (ppos, plen) = interval_handle_pos_and_len(ph);
            if ppos + plen <= stop {
                limit
            } else {
                make_fixnum(ppos + plen)
            }
        }
    }
}

pub fn previous_single_property_change(
    position: Lisp_Object,
    prop: Lisp_Object,
    object: Lisp_Object,
    mut limit: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    if !NILP(limit) {
        CHECK_FIXNUM_COERCE_MARKER!(limit);
    }

    let (mut i_opt, position) = validate_interval_single_pos(object, position, SOFT);
    if let Some(ih) = i_opt {
        if interval_handle_pos_and_len(ih).0 == XFIXNAT(position) {
            i_opt = previous_interval_handle(ih);
        }
    }

    let Some(i) = i_opt else {
        return limit;
    };

    let here_val = textget(interval_handle_plist(i), prop);
    let mut previous = previous_interval_handle(i);
    while let Some(ph) = previous {
        let (ppos, plen) = interval_handle_pos_and_len(ph);
        if EQ(here_val, textget(interval_handle_plist(ph), prop))
            && (NILP(limit) || ppos + plen > XFIXNUM(limit))
        {
            previous = previous_interval_handle(ph);
        } else {
            break;
        }
    }

    let stop = if FIXNUMP(limit) {
        XFIXNUM(limit)
    } else {
        object_beg_pos(object)
    };
    match previous {
        None => limit,
        Some(ph) => {
            let (ppos, plen) = interval_handle_pos_and_len(ph);
            if ppos + plen <= stop {
                limit
            } else {
                make_fixnum(ppos + plen)
            }
        }
    }
}

pub fn put_text_property(
    start: Lisp_Object,
    end: Lisp_Object,
    property: Lisp_Object,
    value: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    let properties = list2(property, value);
    Fadd_text_properties(start, end, properties, object);
    Qnil()
}

pub fn set_text_properties_subr(
    start: Lisp_Object,
    end: Lisp_Object,
    properties: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    set_text_properties(start, end, properties, object, Qt())
}

pub fn add_face_text_property(
    start: Lisp_Object,
    end: Lisp_Object,
    face: Lisp_Object,
    append: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    let properties = list2(Qface(), face);
    crate::textprop::add_face_text_property_helper(start, end, properties, !NILP(append), object);
    Qnil()
}

pub fn text_property_any(
    start: Lisp_Object,
    end: Lisp_Object,
    property: Lisp_Object,
    value: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    let (i_opt, start, end) = validate_interval_range_handle(object, start, end, SOFT);
    let Some(mut i) = i_opt else {
        return if !NILP(value) || EQ(start, end) {
            Qnil()
        } else {
            start
        };
    };
    let e = XFIXNUM(end);
    loop {
        let (ipos, _) = interval_handle_pos_and_len(i);
        if ipos >= e {
            break;
        }
        if EQ(textget(interval_handle_plist(i), property), value) {
            let pos = ipos.max(XFIXNUM(start));
            return make_fixnum(pos);
        }
        let Some(nh) = next_interval_handle(i) else {
            break;
        };
        i = nh;
    }
    Qnil()
}

pub fn text_property_not_all(
    start: Lisp_Object,
    end: Lisp_Object,
    property: Lisp_Object,
    value: Lisp_Object,
    object: Lisp_Object,
) -> Lisp_Object {
    let object = resolve_object_or_current_buffer(object);
    let (i_opt, start, end) = validate_interval_range_handle(object, start, end, SOFT);
    let Some(mut i) = i_opt else {
        return if NILP(value) || EQ(start, end) {
            Qnil()
        } else {
            start
        };
    };
    let mut s = XFIXNUM(start);
    let e = XFIXNUM(end);
    loop {
        let (ipos, _) = interval_handle_pos_and_len(i);
        if ipos >= e {
            break;
        }
        if !EQ(textget(interval_handle_plist(i), property), value) {
            if ipos > s {
                s = ipos;
            }
            return make_fixnum(s);
        }
        let Some(nh) = next_interval_handle(i) else {
            break;
        };
        i = nh;
    }
    Qnil()
}

pub fn text_property_stickiness_impl(
    prop: Lisp_Object,
    pos: Lisp_Object,
    buffer: Lisp_Object,
) -> c_int {
    let prev_pos = make_fixnum(XFIXNUM(pos) - 1);
    let mut is_rear_sticky = true;
    let mut is_front_sticky = false;
    let defalt = Fassq(prop, Vtext_property_default_nonsticky.get());
    let buffer = resolve_object_or_current_buffer(buffer);

    let ignore_previous_character = XFIXNUM(pos) <= buffer_begv_obj(buffer);
    if ignore_previous_character || (CONSP(defalt) && !NILP(XCDR(defalt))) {
        is_rear_sticky = false;
    } else {
        let rear_non_sticky = get_text_property(prev_pos, Qrear_nonsticky(), buffer);
        let match_rns = if CONSP(rear_non_sticky) {
            Fmemq(prop, rear_non_sticky)
        } else {
            rear_non_sticky
        };
        if !NILP(match_rns) {
            is_rear_sticky = false;
        }
    }

    let front_sticky = get_text_property(pos, Qfront_sticky(), buffer);
    if EQ(front_sticky, Qt()) || (CONSP(front_sticky) && !NILP(Fmemq(prop, front_sticky))) {
        is_front_sticky = true;
    }

    if is_rear_sticky && !is_front_sticky {
        -1
    } else if !is_rear_sticky && is_front_sticky {
        1
    } else if !is_rear_sticky && !is_front_sticky {
        0
    } else if ignore_previous_character || NILP(get_text_property(prev_pos, prop, buffer)) {
        1
    } else {
        -1
    }
}

pub fn copy_text_properties_impl(
    start: Lisp_Object,
    end: Lisp_Object,
    src: Lisp_Object,
    mut pos: Lisp_Object,
    dest: Lisp_Object,
    prop: Lisp_Object,
) -> Lisp_Object {
    let (i_opt, start, end) = validate_interval_range_handle(src, start, end, SOFT);
    let Some(mut i) = i_opt else {
        return Qnil();
    };

    CHECK_FIXNUM_COERCE_MARKER!(pos);
    let dest_e = XFIXNUM(pos) + (XFIXNUM(end) - XFIXNUM(start));
    if MOST_POSITIVE_FIXNUM < dest_e {
        args_out_of_range(pos, end);
    }
    let dest_end = make_fixnum(dest_e);
    let (_, pos_val, _) = validate_interval_range_handle(dest, pos, dest_end, SOFT);
    pos = pos_val;

    let mut s = XFIXNUM(start);
    let e = XFIXNUM(end);
    let mut p = XFIXNUM(pos);
    let mut stuff = Qnil();

    while s < e {
        let (ipos, ilen) = interval_handle_pos_and_len(i);
        let e2 = (ipos + ilen).min(e);
        let len = e2 - s;

        let mut plist = interval_handle_plist(i);
        if !NILP(prop) {
            while !NILP(plist) {
                if EQ(Fcar(plist), prop) {
                    plist = list2(prop, Fcar(Fcdr(plist)));
                    break;
                }
                plist = Fcdr(Fcdr(plist));
            }
        }
        if !NILP(plist) {
            stuff = Fcons(list3(make_fixnum(p), make_fixnum(p + len), plist), stuff);
        }

        let Some(nh) = next_interval_handle(i) else {
            break;
        };
        i = nh;
        p += len;
        s = interval_handle_pos_and_len(i).0;
    }

    let mut modified = false;
    while !NILP(stuff) {
        let res = Fcar(stuff);
        let changed = Fadd_text_properties(
            Fcar(res),
            Fcar(Fcdr(res)),
            Fcar(Fcdr(Fcdr(res))),
            dest,
        );
        if !NILP(changed) {
            modified = true;
        }
        stuff = Fcdr(stuff);
    }

    if modified { Qt() } else { Qnil() }
}

pub fn text_property_list_impl(
    object: Lisp_Object,
    start: Lisp_Object,
    end: Lisp_Object,
    prop: Lisp_Object,
) -> Lisp_Object {
    let mut result = Qnil();
    let (i_opt, start, end) = validate_interval_range_handle(object, start, end, SOFT);
    if let Some(mut i) = i_opt {
        let mut s = XFIXNUM(start);
        let e = XFIXNUM(end);
        while s < e {
            let (ipos, ilen) = interval_handle_pos_and_len(i);
            let interval_end = (ipos + ilen).min(e);
            let len = interval_end - s;
            let mut plist = interval_handle_plist(i);
            if !NILP(prop) {
                while CONSP(plist) {
                    if EQ(XCAR(plist), prop) {
                        plist = list2(prop, Fcar(XCDR(plist)));
                        break;
                    }
                    plist = Fcdr(XCDR(plist));
                }
            }
            if !NILP(plist) {
                result = Fcons(list3(make_fixnum(s), make_fixnum(s + len), plist), result);
            }
            let Some(nh) = next_interval_handle(i) else {
                break;
            };
            i = nh;
            s = interval_handle_pos_and_len(i).0;
        }
    }
    result
}

pub fn add_text_properties_from_list_impl(
    object: Lisp_Object,
    mut list: Lisp_Object,
    delta: Lisp_Object,
) {
    while CONSP(list) {
        let item = XCAR(list);
        let start = make_fixnum(XFIXNUM(XCAR(item)) + XFIXNUM(delta));
        let end = make_fixnum(XFIXNUM(XCAR(XCDR(item))) + XFIXNUM(delta));
        let plist = XCAR(XCDR(XCDR(item)));
        Fadd_text_properties(start, end, plist, object);
        list = XCDR(list);
    }
}

pub fn extend_property_ranges_impl(
    mut list: Lisp_Object,
    old_end: Lisp_Object,
    new_end: Lisp_Object,
) -> Lisp_Object {
    let mut prev = Qnil();
    let mut head = list;
    let max = XFIXNUM(new_end);

    while CONSP(list) {
        let item = XCAR(list);
        let beg = XCAR(item);
        let end = XFIXNUM(XCAR(XCDR(item)));

        if XFIXNUM(beg) >= max {
            if EQ(head, list) {
                head = XCDR(list);
            } else {
                XSETCDR(prev, XCDR(list));
            }
        } else if (end == XFIXNUM(old_end) && end != max) || end > max {
            XSETCAR(XCDR(item), new_end);
        }

        prev = list;
        list = XCDR(list);
    }

    head
}
