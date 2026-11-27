//! `#![forbid(unsafe_code)]` implementation of case-table management (`src/casetab.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

pub fn case_table_p(object: Lisp_Object) -> Lisp_Object {
    if !CHAR_TABLE_P(object) {
        return Qnil();
    }
    if !EQ(char_table_purpose(object), Qcase_table()) {
        return Qnil();
    }
    if char_table_extra_slots_count(object) < 3 {
        return Qnil();
    }

    let up = char_table_extra_slot_ref(object, 0);
    let canon = char_table_extra_slot_ref(object, 1);
    let eqv = char_table_extra_slot_ref(object, 2);

    if (NILP(up) || CHAR_TABLE_P(up))
        && ((NILP(canon) && NILP(eqv))
            || (CHAR_TABLE_P(canon) && (NILP(eqv) || CHAR_TABLE_P(eqv))))
    {
        Qt()
    } else {
        Qnil()
    }
}

pub fn check_case_table(obj: Lisp_Object) -> Lisp_Object {
    CHECK_TYPE(!NILP(Fcase_table_p(obj)), Qcase_table_p(), obj);
    obj
}

pub fn current_case_table() -> Lisp_Object {
    current_buffer_downcase_table()
}

pub fn standard_case_table() -> Lisp_Object {
    Vascii_downcase_table.get()
}

pub fn set_case_table(table: Lisp_Object, standard: bool) -> Lisp_Object {
    check_case_table(table);

    let mut up = char_table_extra_slot_ref(table, 0);
    let mut canon = char_table_extra_slot_ref(table, 1);
    let mut eqv = char_table_extra_slot_ref(table, 2);

    if NILP(up) {
        up = Fmake_char_table(Qcase_table(), Qnil());
        map_char_table_c_fn(set_identity, table, up);
        map_char_table_c_fn(shuffle, table, up);
        set_char_table_extras(table, 0, up);
    }

    if NILP(canon) {
        canon = Fmake_char_table(Qcase_table(), Qnil());
        set_char_table_extras(table, 1, canon);
        map_char_table_c_fn(set_canon, table, table);
    }

    if NILP(eqv) {
        eqv = Fmake_char_table(Qcase_table(), Qnil());
        map_char_table_c_fn(set_identity, canon, eqv);
        map_char_table_c_fn(shuffle, canon, eqv);
        set_char_table_extras(table, 2, eqv);
    }

    set_char_table_extras(canon, 2, eqv);

    if standard {
        Vascii_downcase_table.set(table);
        crate::casetab::Vascii_upcase_table.set(up);
        Vascii_canon_table.set(canon);
        crate::casetab::Vascii_eqv_table.set(eqv);
    } else {
        set_case_tables_on_buffer_or_defaults(false, table, up, canon, eqv);
    }

    table
}

pub extern "C-unwind" fn set_canon(
    case_table: Lisp_Object,
    range: Lisp_Object,
    elt: Lisp_Object,
) {
    let up = char_table_extra_slot_ref(case_table, 0);
    let canon = char_table_extra_slot_ref(case_table, 1);

    if FIXNATP(elt) {
        Fset_char_table_range(canon, range, Faref(case_table, Faref(up, elt)));
    }
}

pub extern "C-unwind" fn set_identity(
    table: Lisp_Object,
    c: Lisp_Object,
    elt: Lisp_Object,
) {
    if FIXNATP(elt) {
        let (mut from, mut to) = if CONSP(c) {
            (XFIXNUM(XCAR(c)) as c_int, XFIXNUM(XCDR(c)) as c_int)
        } else {
            let v = XFIXNUM(c) as c_int;
            (v, v)
        };

        to += 1;
        while from < to {
            CHAR_TABLE_SET(table, from as isize, make_fixnum(from as EMACS_INT));
            from += 1;
        }
    }
}

pub extern "C-unwind" fn shuffle(
    table: Lisp_Object,
    c: Lisp_Object,
    elt: Lisp_Object,
) {
    if FIXNATP(elt) {
        let (mut from, mut to) = if CONSP(c) {
            (XFIXNUM(XCAR(c)) as c_int, XFIXNUM(XCDR(c)) as c_int)
        } else {
            let v = XFIXNUM(c) as c_int;
            (v, v)
        };

        to += 1;
        while from < to {
            let tem = Faref(table, elt);
            Faset(table, elt, make_fixnum(from as EMACS_INT));
            Faset(table, make_fixnum(from as EMACS_INT), tem);
            from += 1;
        }
    }
}

pub fn init_casetab_once_impl() {
    Fput(Qcase_table(), Qchar_table_extra_slots(), make_fixnum(3));

    let down = Fmake_char_table(Qcase_table(), Qnil());
    Vascii_downcase_table.set(down);
    set_char_table_purpose(down, Qcase_table());

    for i in 0..128 {
        let c = if (b'A' as c_int..=b'Z' as c_int).contains(&i) {
            i + (b'a' as c_int - b'A' as c_int)
        } else {
            i
        };
        CHAR_TABLE_SET(down, i as isize, make_fixnum(c as EMACS_INT));
    }

    set_char_table_extras(down, 1, Fcopy_sequence(down));

    let up = Fmake_char_table(Qcase_table(), Qnil());
    set_char_table_extras(down, 0, up);

    for i in 0..128 {
        let c = if (b'a' as c_int..=b'z' as c_int).contains(&i) {
            i + (b'A' as c_int - b'a' as c_int)
        } else {
            i
        };
        CHAR_TABLE_SET(up, i as isize, make_fixnum(c as EMACS_INT));
    }

    let eqv = Fmake_char_table(Qcase_table(), Qnil());
    for i in 0..128 {
        let c = if (b'A' as c_int..=b'Z' as c_int).contains(&i) {
            i + (b'a' as c_int - b'A' as c_int)
        } else if (b'a' as c_int..=b'z' as c_int).contains(&i) {
            i + (b'A' as c_int - b'a' as c_int)
        } else {
            i
        };
        CHAR_TABLE_SET(eqv, i as isize, make_fixnum(c as EMACS_INT));
    }

    set_char_table_extras(down, 2, eqv);
    set_case_table(down, true);
}
