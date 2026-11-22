//! `#![forbid(unsafe_code)]` implementation of high-level keymap construction, lookup,
//! active-map selection, and key-description formatting (`src/keymap.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

pub fn make_keymap(string: Lisp_Object) -> Lisp_Object {
    let tail = if !NILP(string) {
        list1(string)
    } else {
        Qnil()
    };
    Fcons(
        Qkeymap(),
        Fcons(Fmake_char_table(Qkeymap(), Qnil()), tail),
    )
}

pub fn make_sparse_keymap(string: Lisp_Object) -> Lisp_Object {
    if !NILP(string) {
        return list2(Qkeymap(), string);
    }
    list1(Qkeymap())
}

pub fn keymapp(object: Lisp_Object) -> Lisp_Object {
    if !NILP(get_keymap(object, false, false)) {
        Qt()
    } else {
        Qnil()
    }
}

pub fn keymap_prompt(mut map: Lisp_Object) -> Lisp_Object {
    map = get_keymap(map, false, false);
    while CONSP(map) {
        let mut tem = XCAR(map);
        if STRINGP(tem) {
            return tem;
        }
        if !NILP(get_keymap(tem, false, false)) {
            tem = keymap_prompt(tem);
            if !NILP(tem) {
                return tem;
            }
        }
        map = XCDR(map);
    }
    Qnil()
}

pub fn keymap_parent_impl(keymap: Lisp_Object, autoload: bool) -> Lisp_Object {
    let map = get_keymap(keymap, true, autoload);
    let mut list = XCDR(map);
    while CONSP(list) {
        if !NILP(get_keymap(list, false, false)) {
            return list;
        }
        list = XCDR(list);
    }
    get_keymap(list, false, autoload)
}

pub fn keymap_parent(keymap: Lisp_Object) -> Lisp_Object {
    keymap_parent_impl(keymap, true)
}

pub fn keymap_memberp(map: Lisp_Object, mut maps: Lisp_Object) -> bool {
    if NILP(map) {
        return false;
    }
    while !NILP(get_keymap(maps, false, false)) && !EQ(map, maps) {
        maps = keymap_parent_impl(maps, false);
    }
    EQ(map, maps)
}

pub fn set_keymap_parent(keymap: Lisp_Object, parent: Lisp_Object) -> Lisp_Object {
    crate::keymap::flush_where_is_cache_for_parent_safe();
    let keymap = get_keymap(keymap, true, true);
    let parent = if !NILP(parent) {
        let p = get_keymap(parent, true, false);
        if keymap_memberp(keymap, p) {
            crate::keymap::error_cyclic_keymap_inheritance();
        }
        p
    } else {
        parent
    };

    let mut prev = keymap;
    loop {
        let list = XCDR(prev);
        if !CONSP(list) || !NILP(get_keymap(list, false, false)) {
            Fsetcdr(prev, parent);
            return parent;
        }
        prev = list;
    }
}

pub fn key_binding(
    key: Lisp_Object,
    accept_default: Lisp_Object,
    no_remap: Lisp_Object,
    mut position: Lisp_Object,
) -> Lisp_Object {
    if NILP(position) && VECTORP(key) {
        if ASIZE(key) == 0 {
            return Qnil();
        }
        let idx = if SYMBOLP(AREF(key, 0)) && ASIZE(key) > 1 {
            1
        } else {
            0
        };
        let event = AREF(key, idx);
        if EVENT_HAS_PARAMETERS(event) && CONSP(XCDR(event)) {
            let kind = EVENT_HEAD_KIND(EVENT_HEAD(event));
            if EQ(kind, Qmouse_click()) {
                position = EVENT_START(event);
            }
        }
    }

    let mut value = Flookup_key(Fcurrent_active_maps(Qt(), position), key, accept_default);
    if NILP(value) || FIXNUMP(value) {
        return Qnil();
    }

    if NILP(no_remap) && SYMBOLP(value) {
        let value1 = Fcommand_remapping(value, position, Qnil());
        if !NILP(value1) {
            value = value1;
        }
    }
    value
}

pub fn use_global_map(keymap: Lisp_Object) -> Lisp_Object {
    let keymap = get_keymap(keymap, true, true);
    current_global_map.set(keymap);
    Qnil()
}

pub fn use_local_map(keymap: Lisp_Object) -> Lisp_Object {
    let keymap = if !NILP(keymap) {
        get_keymap(keymap, true, true)
    } else {
        keymap
    };
    set_current_buffer_keymap_obj(keymap);
    Qnil()
}

pub fn current_local_map() -> Lisp_Object {
    current_buffer_keymap_obj()
}

pub fn current_global_map_subr() -> Lisp_Object {
    current_global_map.get()
}

pub fn key_description(keys: Lisp_Object, prefix: Lisp_Object) -> Lisp_Object {
    let nkeys = XFIXNUM(Flength(keys));
    let nprefix = XFIXNUM(Flength(prefix));
    let (size4, over) = (nkeys + nprefix).overflowing_mul(4);
    if over || size4 < 0 {
        memory_full_up();
    }

    let sep = build_cstr(c" ");
    let mut args = RootedVec::with_capacity((size4 as usize).saturating_add(1));
    let mut add_meta = false;

    let lists = [prefix, keys];
    let listlens = [nprefix, nkeys];
    for li in 0..2 {
        let mut list = lists[li];
        let listlen = listlens[li];
        let mut i_byte: ptrdiff_t = 0;

        if !(NILP(list) || STRINGP(list) || VECTORP(list) || CONSP(list)) {
            wrong_type_argument(Qarrayp(), list);
        }

        let mut i: ptrdiff_t = 0;
        while i < listlen {
            let mut key = if STRINGP(list) {
                let (mut c, next_i, next_i_byte) = fetch_string_char_step(list, i, i_byte);
                i = next_i;
                i_byte = next_i_byte;
                if !STRING_MULTIBYTE(list) && (c & 0o200) != 0 {
                    c ^= 0o200 | meta_modifier;
                }
                make_fixnum(c as EMACS_INT)
            } else if VECTORP(list) {
                let k = AREF(list, i);
                i += 1;
                k
            } else {
                let k = XCAR(list);
                list = XCDR(list);
                i += 1;
                k
            };

            if add_meta {
                let mpc = meta_prefix_char.get();
                if !FIXNUMP(key)
                    || EQ(key, mpc)
                    || (XFIXNUM(key) & meta_modifier as EMACS_INT) != 0
                {
                    args.push(single_key_description(mpc, Qnil()));
                    args.push(sep);
                    if EQ(key, mpc) {
                        continue;
                    }
                } else {
                    key = make_fixnum(XFIXNUM(key) | meta_modifier as EMACS_INT);
                }
                add_meta = false;
            } else if EQ(key, meta_prefix_char.get()) {
                add_meta = true;
                continue;
            }
            args.push(single_key_description(key, Qnil()));
            args.push(sep);
        }
    }

    if add_meta {
        args.push(single_key_description(meta_prefix_char.get(), Qnil()));
        Fconcat_slice(&mut args)
    } else if args.is_empty() {
        empty_unibyte_string.get()
    } else {
        let used = args.len() - 1;
        Fconcat_slice(&mut args[..used])
    }
}

pub fn single_key_description(mut key: Lisp_Object, no_angles: Lisp_Object) -> Lisp_Object {
    if CONSP(key) && lucid_event_type_list_p(key) {
        key = Fevent_convert_list(key);
    }

    if CONSP(key) && FIXNUMP(XCAR(key)) && FIXNUMP(XCDR(key)) {
        let left = single_key_description(XCAR(key), no_angles);
        let dot_dot = build_cstr(c"..");
        let right = single_key_description(XCDR(key), no_angles);
        let mut parts = [left, dot_dot, right];
        return Fconcat_slice(&mut parts);
    }

    key = EVENT_HEAD(key);

    if FIXNUMP(key) {
        push_key_description_str(XFIXNUM(key))
    } else if SYMBOLP(key) {
        if NILP(no_angles) {
            let namestr = SYMBOL_NAME(key);
            let sym_bytes = lisp_string_bytes_vec(namestr);
            let len = sym_bytes.len();
            let mut i: usize = 0;
            while i + 3 < len
                && sym_bytes[i + 1] == b'-'
                && matches!(
                    sym_bytes[i],
                    b'C' | b'M' | b'S' | b's' | b'H' | b'A'
                )
            {
                i += 2;
            }
            let mut out = Vec::with_capacity(len + 2);
            out.extend_from_slice(&sym_bytes[..i]);
            out.push(b'<');
            out.extend_from_slice(&sym_bytes[i..]);
            out.push(b'>');
            let nul_pos = out.iter().position(|&b| b == 0).unwrap_or(out.len());
            make_string_bytes(&out[..nul_pos])
        } else {
            Fsymbol_name(key)
        }
    } else if STRINGP(key) {
        Fcopy_sequence(key)
    } else {
        error_single_key_description_type()
    }
}

pub fn text_char_description(character: Lisp_Object) -> Lisp_Object {
    CHECK_CHARACTER(character);
    let c = XFIXNUM(character) as c_int;
    if !ASCII_CHAR_P(c as isize) {
        let (buf, len) = encode_char_multibyte_buf(c);
        make_specified_string_bytes(&buf[..len], 1, true)
    } else {
        let cu = c as u32;
        let mut desc = [0u8; 4];
        let len = if cu < 0o40 {
            desc[0] = b'^';
            desc[1] = (cu + 64) as u8;
            2
        } else if cu == 0o177 {
            desc[0] = b'^';
            desc[1] = b'?';
            2
        } else {
            desc[0] = cu as u8;
            1
        };
        make_string_bytes(&desc[..len])
    }
}

pub fn preferred_sequence_p_impl(seq: Lisp_Object, preferred_mod: c_int) -> c_int {
    let len = XFIXNAT(Flength(seq));
    let mut result: c_int = 1;
    for i in 0..len {
        let elt = Faref(seq, make_fixnum(i));
        if !FIXNUMP(elt) {
            return 0;
        }
        let modifiers =
            (XFIXNUM(elt) & ((CHAR_MODIFIER_MASK & !CHAR_META) as EMACS_INT)) as c_int;
        if modifiers == preferred_mod {
            result = 2;
        } else if modifiers != 0 {
            return 0;
        }
    }
    result
}
