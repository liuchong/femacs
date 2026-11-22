//! `#![forbid(unsafe_code)]` implementation of interactive point movement, line scanning,
//! deletion, and self-insertion commands (`src/cmds.rs`, `U6` / `P-485`).

#![forbid(unsafe_code)]

use crate::common::*;

pub fn move_point(mut n: Lisp_Object, forward: bool) -> Lisp_Object {
    if NILP(n) {
        XSETFASTINT!(n, 1);
    } else {
        CHECK_FIXNUM(n);
    }

    let delta = if forward { XFIXNUM(n) } else { -XFIXNUM(n) };
    let new_point = PT!() + delta;
    let begv = current_buffer_begv();
    if new_point < begv {
        SET_PT(begv);
        xsignal0_safe(Qbeginning_of_buffer());
    }
    let zv = current_buffer_zv();
    if new_point > zv {
        SET_PT(zv);
        xsignal0_safe(Qend_of_buffer());
    }

    SET_PT(new_point);
    Qnil()
}

pub fn forward_line(n: Lisp_Object) -> Lisp_Object {
    let opoint: ptrdiff_t = PT!();
    let count: ptrdiff_t;
    let mut excessive = false;

    if NILP(n) {
        count = 1;
    } else {
        CHECK_INTEGER(n);
        if FIXNUMP(n) && -BUF_BYTES_MAX <= XFIXNUM(n) && XFIXNUM(n) <= BUF_BYTES_MAX {
            count = XFIXNUM(n);
        } else {
            count = if !NILP(Fnatnump(n)) {
                BUF_BYTES_MAX
            } else {
                -BUF_BYTES_MAX
            };
            excessive = true;
        }
    }

    let (counted, pos, pos_byte) = scan_newline_from_point_checked(count);
    SET_PT_BOTH(pos, pos_byte);

    let mut shortage: ptrdiff_t = count - (count <= 0) as ptrdiff_t - counted;
    if shortage != 0 {
        shortage -= if count <= 0 {
            -1
        } else {
            (current_buffer_begv() < current_buffer_zv()
                && PT!() != opoint
                && current_buffer_fetch_byte(PT_BYTE!() - 1) != b'\n') as ptrdiff_t
        };
    }
    if excessive {
        CALLN!(Fplus, make_fixnum(shortage - count), n)
    } else {
        make_fixnum(shortage)
    }
}

pub fn beginning_of_line(mut n: Lisp_Object) -> Lisp_Object {
    if NILP(n) {
        XSETFASTINT!(n, 1);
    } else {
        CHECK_FIXNUM(n);
    }
    SET_PT(XFIXNUM(Fline_beginning_position(n)));
    Qnil()
}

pub fn end_of_line(mut n: Lisp_Object) -> Lisp_Object {
    if NILP(n) {
        XSETFASTINT!(n, 1);
    } else {
        CHECK_FIXNUM(n);
    }

    loop {
        let newpos = XFIXNUM(Fline_end_position(n));
        SET_PT(newpos);

        if PT!() > newpos && current_buffer_fetch_byte(PT_BYTE!() - 1) == b'\n' {
            SET_PT(PT!() - 1);
            break;
        } else if PT!() > newpos
            && PT!() < current_buffer_zv()
            && current_buffer_fetch_byte(PT_BYTE!()) != b'\n'
        {
            n = make_fixnum(1);
        } else {
            break;
        }
    }
    Qnil()
}

pub fn delete_char(n: Lisp_Object, killflag: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(n);

    if eabs!(XFIXNUM(n)) < 2 {
        call0(Qundo_auto_amalgamate());
    }

    let pos = PT!() + XFIXNUM(n);
    if NILP(killflag) {
        if XFIXNUM(n) < 0 {
            if pos < current_buffer_begv() {
                xsignal0_safe(Qbeginning_of_buffer());
            } else {
                del_range_safe(pos, PT!());
            }
        } else if pos > current_buffer_zv() {
            xsignal0_safe(Qend_of_buffer());
        } else {
            del_range_safe(PT!(), pos);
        }
    } else {
        calln!(Qkill_forward_chars(), n);
    }
    Qnil()
}

pub fn self_insert_command(n: Lisp_Object, mut c: Lisp_Object) -> Lisp_Object {
    CHECK_FIXNUM(n);

    if NILP(c) {
        c = crate::keyboard::last_command_event.get();
    } else {
        crate::keyboard::last_command_event.set(c);
    }

    if XFIXNUM(n) < 0 {
        error_negative_repetition(XFIXNUM(n));
    }

    if XFIXNAT(n) < 2 {
        call0(Qundo_auto_amalgamate());
    }

    if !CHARACTERP(c) {
        bitch_at_user_safe();
    } else {
        let character = translate_char_safe(
            crate::coding::Vtranslation_table_for_input.get(),
            XFIXNUM(c) as c_int,
        );
        let val = internal_self_insert(character, XFIXNAT(n));
        if val == 2 {
            Fset(Qundo_auto__this_command_amalgamating(), Qnil());
        }
        frame_make_pointer_invisible_selected();
    }

    Qnil()
}

pub fn internal_self_insert(mut c: c_int, n: EMACS_INT) -> c_int {
    let mut hairy: c_int = 0;
    let mut str_buf: [u8; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
    let mut chars_to_delete: ptrdiff_t = 0;
    let mut spaces_to_insert: ptrdiff_t = 0;

    let overwrite = current_buffer_overwrite_mode();
    if !NILP(crate::buffer::Vbefore_change_functions.get())
        || !NILP(crate::buffer::Vafter_change_functions.get())
    {
        hairy = 1;
    }

    let len: usize;
    if current_buffer_multibyte_p() {
        len = encode_char_bytes(c, &mut str_buf);
        if len == 1 {
            c = str_buf[0] as c_int;
        }
    } else {
        str_buf[0] = if SINGLE_BYTE_CHAR_P(c as intmax_t) {
            c as u8
        } else {
            CHAR_TO_BYTE8(c) as u8
        };
        len = 1;
    }

    if !NILP(overwrite) && PT!() < current_buffer_zv() {
        let c2: c_int = FETCH_CHAR(PT_BYTE!());
        let mut cwidth: c_int = 0;

        if EQ(overwrite, Qoverwrite_mode_binary()) {
            chars_to_delete = n.min(PTRDIFF_MAX);
        } else if c != b'\n' as c_int
            && c2 != b'\n' as c_int
            && {
                cwidth = XFIXNAT(Fchar_width(make_fixnum(c as EMACS_INT))) as c_int;
                cwidth != 0
            }
        {
            let pos = PT!();
            let pos_byte = PT_BYTE!();
            let curcol = current_column_safe();

            if n <= (MOST_POSITIVE_FIXNUM.min(PTRDIFF_MAX) - curcol) / cwidth as ptrdiff_t {
                let target_clm = curcol + n * cwidth as ptrdiff_t;
                let actual_clm =
                    XFIXNAT(Fmove_to_column(make_fixnum(target_clm), Qnil())) as ptrdiff_t;
                chars_to_delete = PT!() - pos;

                if actual_clm > target_clm {
                    let mut actual = PT_BYTE!();
                    actual -= prev_char_len(actual) as ptrdiff_t;
                    if current_buffer_fetch_byte(actual) == b'\t' {
                        chars_to_delete -= 1;
                    } else {
                        spaces_to_insert = actual_clm - target_clm;
                    }
                }
                SET_PT_BOTH(pos, pos_byte);
            }
        }
        hairy = 2;
    }

    let synt = SYNTAX(c);
    if current_buffer_abbrev_mode_p()
        && synt != Sword
        && !current_buffer_read_only_p()
        && PT!() > current_buffer_begv()
        && {
            let prev_c = XFIXNAT(Fprevious_char()) as c_int;
            let prev_char = if current_buffer_multibyte_p() {
                prev_c
            } else {
                UNIBYTE_TO_CHAR(prev_c)
            };
            SYNTAX(prev_char) == Sword
        }
    {
        let modiff = current_buffer_modiff();
        let sym = call0(Qexpand_abbrev());

        if SYMBOLP(sym) && !NILP(sym) {
            let sym_fn = symbol_function_slot(sym);
            if !NILP(sym_fn) && SYMBOLP(sym_fn) {
                let prop = Fget(sym_fn, Qno_self_insert());
                if !NILP(prop) {
                    return 1;
                }
            }
        }

        if current_buffer_modiff() != modiff {
            hairy = 2;
        }
    }

    if chars_to_delete != 0 {
        let mc = if !current_buffer_multibyte_p() && SINGLE_BYTE_CHAR_P(c as intmax_t) {
            UNIBYTE_TO_CHAR(c)
        } else {
            c
        };
        let mut string = Fmake_string(make_fixnum(n), make_fixnum(mc as EMACS_INT), Qnil());

        if spaces_to_insert != 0 {
            let tem = Fmake_string(
                make_fixnum(spaces_to_insert),
                make_fixnum(b' ' as EMACS_INT),
                Qnil(),
            );
            string = concat2_safe(string, tem);
        }

        let (sum, overflowed) = PT!().overflowing_add(chars_to_delete);
        let to = if overflowed { PTRDIFF_MAX } else { sum };
        replace_range_safe(PT!(), to, string, true, true, false);
        Fforward_char(make_fixnum(n));
    } else if n > 1 {
        let repeated = str_buf[..len].repeat(n as usize);
        insert_and_inherit_bytes(&repeated);
    } else if n > 0 {
        insert_and_inherit_bytes(&str_buf[..len]);
    }

    let auto_fill_chars = crate::character::Vauto_fill_chars.get();
    let auto_fill_char_p = if CHAR_TABLE_P(auto_fill_chars) {
        !NILP(CHAR_TABLE_REF(auto_fill_chars, c as isize))
    } else {
        c == b' ' as c_int || c == b'\n' as c_int
    };
    if auto_fill_char_p && current_buffer_auto_fill_p() && n > 0 {
        if c == b'\n' as c_int {
            SET_PT_BOTH(PT!() - 1, PT_BYTE!() - 1);
        }
        let auto_fill_result = call0(Qinternal_auto_fill());
        if c == b'\n' as c_int
            && PT!() < current_buffer_zv()
            && current_buffer_fetch_byte(PT_BYTE!()) == b'\n'
        {
            SET_PT_BOTH(PT!() + 1, PT_BYTE!() + 1);
        }
        if !NILP(auto_fill_result) {
            hairy = 2;
        }
    }

    run_hook_safe(Qpost_self_insert_hook());
    hairy
}
