//! `#![forbid(unsafe_code)]` implementation of interactive command specification
//! parsing and invocation (`src/callint.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;
use crate::roots::RootedVec;

const CALLINT_ARGFUNS: [&core::ffi::CStr; 7] = [
    c"",
    c"point",
    c"mark",
    c"region-beginning",
    c"region-end",
    c"use-region-beginning",
    c"use-region-end",
];

fn invalid_control_letter(spec_tail: &[u8]) -> ! {
    let letter: c_uint = if spec_tail.is_empty() {
        0
    } else {
        let head = spec_tail[0] as c_int;
        let need = BYTES_BY_CHAR_HEAD(head) as usize;
        if spec_tail.len() >= need {
            string_char_and_length_slice(spec_tail).0 as c_uint
        } else {
            spec_tail[0] as c_uint
        }
    };
    error_callint_invalid_letter(letter)
}

fn quotify_arg(exp: Lisp_Object) -> Lisp_Object {
    if CONSP(exp) || (SYMBOLP(exp) && !NILP(exp) && !EQ(exp, Qt())) {
        list2(Qquote(), exp)
    } else {
        exp
    }
}

fn quotify_args(exp: Lisp_Object) -> Lisp_Object {
    let mut tail = exp;
    while CONSP(tail) {
        let next = XCDR(tail);
        XSETCAR(tail, quotify_arg(XCAR(tail)));
        tail = next;
    }
    exp
}

fn check_mark(for_region: bool) {
    let mark = current_buffer_mark();
    let tem = Fmarker_buffer(mark);
    if NILP(tem) || !current_buffer_is_same_obj(tem) {
        error_mark_not_set(for_region);
    }
    if !NILP(Vtransient_mark_mode.get())
        && NILP(Vmark_even_if_inactive.get())
        && NILP(current_buffer_mark_active())
    {
        xsignal0(Qmark_inactive());
    }
}

fn fix_command(function: Lisp_Object, values: Lisp_Object) {
    if !CONSP(values) || !SYMBOLP(function) {
        return;
    }

    let reps = Fget(function, Qinteractive_args());
    if CONSP(reps) {
        let mut i: c_int = 0;
        let mut vals = values;
        while !NILP(vals) {
            let rep = Fassq(make_fixnum(i as EMACS_INT), reps);
            if !NILP(rep) {
                Fsetcar(vals, XCDR(rep));
            }
            vals = XCDR(vals);
            i += 1;
        }
    }

    let arity = Ffunc_arity(function);
    if FIXNUMP(XCAR(arity)) && FIXNUMP(XCDR(arity)) {
        let mut final_cell = Qnil();
        let mut final_i: ptrdiff_t = 0;
        let mut i: ptrdiff_t = 0;
        let mut tail = values;
        while CONSP(tail) {
            if !NILP(XCAR(tail)) {
                final_cell = tail;
                final_i = i;
            }
            tail = XCDR(tail);
            i += 1;
        }
        if final_i > 0 && final_i >= XFIXNUM(XCAR(arity)) - 1 {
            XSETCDR(final_cell, Qnil());
        }
    }
}

#[inline(always)]
fn read_file_name(
    default_filename: Lisp_Object,
    mustmatch: Lisp_Object,
    initial: Lisp_Object,
    predicate: Lisp_Object,
) -> Lisp_Object {
    calln!(
        Qread_file_name(),
        crate::callint::callint_message.get(),
        Qnil(),
        default_filename,
        mustmatch,
        initial,
        predicate
    )
}

#[inline(always)]
pub fn interactive_subr(_args: Lisp_Object) -> Lisp_Object {
    Qnil()
}

pub fn funcall_interactively_slice(args: &[Lisp_Object]) -> Lisp_Object {
    let speccount = SPECPDL_INDEX();
    if !multiple_terminals_merge_keyboards.get() {
        temporarily_switch_to_single_kboard_null();
    }
    let mut call_args = RootedVec::from_slice(args);
    unbind_to(speccount, Ffuncall_slice(call_args.as_mut_slice()))
}

pub fn call_interactively(
    function: Lisp_Object,
    record_flag: Lisp_Object,
    mut keys: Lisp_Object,
) -> Lisp_Object {
    let mut arg_from_tty = false;
    let mut record_then_fail = false;

    let save_this_command = Vthis_command.get();
    let save_this_original_command = Vthis_original_command.get();
    let save_real_this_command = Vreal_this_command.get();
    let save_last_command = current_kboard_last_command();

    let key_count: ptrdiff_t = if NILP(keys) {
        keys = this_command_keys.get();
        this_command_key_count.get()
    } else {
        CHECK_VECTOR(keys);
        ASIZE(keys)
    };

    let prefix_arg = Vcurrent_prefix_arg.get();
    let enable = if SYMBOLP(function) {
        Fget(function, Qenable_recursive_minibuffers())
    } else {
        Qnil()
    };

    let mut up_event = Qnil();

    let form = calln!(Qinteractive_form(), function);
    if !CONSP(form) {
        wrong_type_argument(Qcommandp(), function);
    }
    let mut specs = Fcar(XCDR(form));

    if !STRINGP(specs) {
        let funval = Findirect_function(function, Qt());
        let events = num_input_events.get();
        let env = if CLOSUREP(funval) && CONSP(AREF(funval, CLOSURE_CODE as isize)) {
            AREF(funval, CLOSURE_CONSTANTS as isize)
        } else {
            Qnil()
        };
        specs = Feval(specs, env);
        if events != num_input_events.get() || !NILP(record_flag) {
            let values = quotify_args(Fcopy_sequence(specs));
            fix_command(function, values);
            calln!(
                Qadd_to_history(),
                Qcommand_history(),
                Fcons(function, values),
                Qnil(),
                Qt()
            );
        }

        Vthis_command.set(save_this_command);
        Vthis_original_command.set(save_this_original_command);
        Vreal_this_command.set(save_real_this_command);
        set_current_kboard_last_command(save_last_command);

        return CALLN!(Fapply, Qfuncall_interactively(), function, specs);
    }

    let spec_bytes = lisp_string_bytes_vec(specs);
    let string_len = spec_bytes.len();

    let mut next_event: ptrdiff_t = 0;
    if !inhibit_mouse_event_check.get() {
        while next_event < key_count {
            if EVENT_HAS_PARAMETERS(AREF(keys, next_event)) {
                break;
            }
            next_event += 1;
        }
    }

    let mut start_idx: usize = 0;
    while start_idx < string_len {
        match spec_bytes[start_idx] {
            b'+' => error_callint_plus_reserved(),
            b'*' => {
                if current_buffer_read_only_p() {
                    if !NILP(record_flag) {
                        for &b in &spec_bytes[(start_idx + 1)..] {
                            if !matches!(b, b'r' | b'R' | b'p' | b'P' | b'\n') {
                                Fbarf_if_buffer_read_only(Qnil());
                            }
                        }
                        record_then_fail = true;
                    } else {
                        Fbarf_if_buffer_read_only(Qnil());
                    }
                }
            }
            b'-' => {}
            b'@' => {
                let event = if next_event < key_count {
                    AREF(keys, next_event)
                } else {
                    Qnil()
                };
                if EVENT_HAS_PARAMETERS(event) {
                    let w1 = XCDR(event);
                    if CONSP(w1) {
                        let w2 = XCAR(w1);
                        if CONSP(w2) {
                            let w = XCAR(w2);
                            if WINDOWP(w) {
                                if MINI_WINDOW_P(XWINDOW(w))
                                    && !(minibuf_level.get() > 0
                                        && BASE_EQ(w, minibuf_window.get()))
                                {
                                    error_callint_inactive_minibuf();
                                }
                                run_hook(Qmouse_leave_buffer_hook());
                                Fselect_window(w, Qnil());
                            }
                        }
                    }
                }
            }
            b'^' => {
                call0(Qhandle_shift_selection());
            }
            _ => break,
        }
        start_idx += 1;
    }

    let mut nargs: usize = 2;
    let mut scan_idx = start_idx;
    while scan_idx < string_len {
        let code = spec_bytes[scan_idx];
        nargs += 1 + usize::from(code == b'r' || code == b'R');
        if let Some(rel_nl) = spec_bytes[scan_idx..].iter().position(|&b| b == b'\n') {
            scan_idx += rel_nl + 1;
        } else {
            break;
        }
    }

    if (MOST_POSITIVE_FIXNUM as usize) < (PTRDIFF_MAX as usize) / (word_size as usize)
        && (MOST_POSITIVE_FIXNUM as usize) < nargs
    {
        memory_full_up();
    }

    let mut args = RootedVec::filled(nargs, Qnil());
    let mut visargs = RootedVec::filled(nargs, Qnil());
    let mut varies = vec![0i8; nargs];

    let speccount = SPECPDL_INDEX();
    if !NILP(enable) {
        specbind(Qenable_recursive_minibuffers(), Qt());
    }

    let mut tem_idx = start_idx;
    let mut i: usize = 2;
    while tem_idx < string_len {
        let prompt_start = tem_idx + 1;
        let prompt_end = spec_bytes[prompt_start..]
            .iter()
            .position(|&b| b == b'\n')
            .map_or(string_len, |rel| prompt_start + rel);

        visargs[1] = make_string_bytes(&spec_bytes[prompt_start..prompt_end]);
        let msg = Fformat_message_slice(&mut visargs.as_mut_slice()[1..i]);
        crate::callint::callint_message.set(msg);

        let code = spec_bytes[tem_idx];
        match code {
            b'a' => {
                let v = Fcompleting_read(
                    crate::callint::callint_message.get(),
                    Vobarray.get(),
                    Qfboundp(),
                    Qt(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                );
                visargs[i] = v;
                args[i] = Fintern(v, Qnil());
            }
            b'b' => {
                let mut def = Fcurrent_buffer();
                if BASE_EQ(selected_window.get(), minibuf_window.get()) {
                    def = Fother_buffer(def, Qnil(), Qnil());
                }
                args[i] = Fread_buffer(crate::callint::callint_message.get(), def, Qt(), Qnil());
            }
            b'B' => {
                args[i] = Fread_buffer(
                    crate::callint::callint_message.get(),
                    Fother_buffer(Fcurrent_buffer(), Qnil(), Qnil()),
                    Qnil(),
                    Qnil(),
                );
            }
            b'c' => {
                let msg_cur = crate::callint::callint_message.get();
                Fput_text_property(
                    make_fixnum(0),
                    make_fixnum(SCHARS(msg_cur) as EMACS_INT),
                    Qface(),
                    Qminibuffer_prompt(),
                    msg_cur,
                );
                let ch = Fread_char(crate::callint::callint_message.get(), Qnil(), Qnil());
                args[i] = ch;
                message1_nolog_clear();
                if !CHARACTERP(ch) {
                    error_callint_non_char_event();
                }
                visargs[i] = Fchar_to_string(ch);
            }
            b'C' => {
                let v = Fcompleting_read(
                    crate::callint::callint_message.get(),
                    Vobarray.get(),
                    Qcommandp(),
                    Qt(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                );
                visargs[i] = v;
                args[i] = Fintern(v, Qnil());
            }
            b'd' => {
                let pm = crate::callint::point_marker.get();
                set_marker_both(pm, Qnil(), current_buffer_pt(), current_buffer_pt_byte());
                args[i] = pm;
                varies[i] = 1;
            }
            b'D' => {
                args[i] = read_file_name(
                    current_buffer_directory(),
                    Qlambda(),
                    Qnil(),
                    Qfile_directory_p(),
                );
            }
            b'f' => {
                args[i] = read_file_name(Qnil(), Qlambda(), Qnil(), Qnil());
            }
            b'F' => {
                args[i] = read_file_name(Qnil(), Qnil(), Qnil(), Qnil());
            }
            b'G' => {
                args[i] = read_file_name(Qnil(), Qnil(), empty_unibyte_string.get(), Qnil());
            }
            b'i' => {
                varies[i] = -1;
            }
            b'k' | b'K' => {
                let speccount1 = SPECPDL_INDEX();
                specbind(Qcursor_in_echo_area(), Qt());
                let msg_cur = crate::callint::callint_message.get();
                Fput_text_property(
                    make_fixnum(0),
                    make_fixnum(SCHARS(msg_cur) as EMACS_INT),
                    Qface(),
                    Qminibuffer_prompt(),
                    msg_cur,
                );
                if code == b'k' {
                    let kseq = Fread_key_sequence(
                        crate::callint::callint_message.get(),
                        Qnil(),
                        Qnil(),
                        Qnil(),
                        Qnil(),
                        Qnil(),
                    );
                    args[i] = kseq;
                    unbind_to(speccount1, Qnil());
                    visargs[i] = Fkey_description(kseq, Qnil());

                    let mut teml = Faref(kseq, make_fixnum(XFIXNUM(Flength(kseq)) - 1));
                    if CONSP(teml) {
                        teml = XCAR(teml);
                    }
                    if SYMBOLP(teml) {
                        teml = Fget(teml, Qevent_symbol_elements());
                        if !NILP(Fmemq(Qdown(), Fcdr(teml))) {
                            up_event = Fread_event(Qnil(), Qnil(), Qnil());
                        }
                    }
                } else {
                    let kvec = Fread_key_sequence_vector(
                        crate::callint::callint_message.get(),
                        Qnil(),
                        Qt(),
                        Qnil(),
                        Qnil(),
                        Qnil(),
                    );
                    args[i] = kvec;
                    visargs[i] = Fkey_description(kvec, Qnil());
                    unbind_to(speccount1, Qnil());

                    let mut teml = Faref(kvec, make_fixnum(ASIZE(kvec) - 1));
                    if CONSP(teml) {
                        teml = XCAR(teml);
                    }
                    if SYMBOLP(teml) {
                        teml = Fget(teml, Qevent_symbol_elements());
                        if !NILP(Fmemq(Qdown(), Fcdr(teml))) {
                            up_event = Fread_event(Qnil(), Qnil(), Qnil());
                        }
                    }
                }
            }
            b'U' => {
                if !NILP(up_event) {
                    let v = make_vector(1, up_event);
                    args[i] = v;
                    up_event = Qnil();
                    visargs[i] = Fkey_description(v, Qnil());
                }
            }
            b'e' => {
                if next_event >= key_count {
                    error_callint_event_params(function);
                }
                args[i] = AREF(keys, next_event);
                varies[i] = -1;
                if inhibit_mouse_event_check.get() {
                    next_event += 1;
                } else {
                    loop {
                        next_event += 1;
                        if !(next_event < key_count
                            && !EVENT_HAS_PARAMETERS(AREF(keys, next_event)))
                        {
                            break;
                        }
                    }
                }
            }
            b'm' => {
                check_mark(false);
                args[i] = current_buffer_mark();
                varies[i] = 2;
            }
            b'M' => {
                args[i] = Fread_string(
                    crate::callint::callint_message.get(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                    Qt(),
                );
            }
            b'N' | b'n' | b'p' => {
                let take_prefix_arg = code == b'p' || (code == b'N' && !NILP(prefix_arg));
                if take_prefix_arg {
                    args[i] = prefix_numeric_value(prefix_arg);
                    varies[i] = -1;
                } else {
                    let nval = calln!(Qread_number(), crate::callint::callint_message.get());
                    args[i] = nval;
                    visargs[i] = Fnumber_to_string(nval);
                }
            }
            b'P' => {
                args[i] = prefix_arg;
                varies[i] = -1;
            }
            b'R' | b'r' => {
                let mut do_region = code == b'r';
                if code == b'R' {
                    if NILP(calln!(Quse_region_p())) {
                        varies[i] = 5;
                        i += 1;
                        varies[i] = 6;
                    } else {
                        do_region = true;
                    }
                }
                if do_region {
                    check_mark(true);
                    let pm = crate::callint::point_marker.get();
                    let pt = current_buffer_pt();
                    set_marker_both(pm, Qnil(), pt, current_buffer_pt_byte());
                    let cur_mark = current_buffer_mark();
                    let mark_pos = marker_position(cur_mark);
                    args[i] = if pt < mark_pos { pm } else { cur_mark };
                    varies[i] = if code == b'R' { 5 } else { 3 };
                    i += 1;
                    args[i] = if pt > mark_pos { pm } else { cur_mark };
                    varies[i] = if code == b'R' { 6 } else { 4 };
                }
            }
            b's' => {
                args[i] = Fread_string(
                    crate::callint::callint_message.get(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                );
            }
            b'S' => {
                let s = Fread_string(
                    crate::callint::callint_message.get(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                    Qnil(),
                );
                visargs[i] = s;
                args[i] = Fintern(s, Qnil());
            }
            b'v' => {
                args[i] = Fread_variable(crate::callint::callint_message.get(), Qnil());
                visargs[i] = last_minibuf_string.get();
            }
            b'x' => {
                args[i] = calln!(Qread_minibuffer(), crate::callint::callint_message.get());
                visargs[i] = last_minibuf_string.get();
            }
            b'X' => {
                args[i] = calln!(Qeval_minibuffer(), crate::callint::callint_message.get());
                visargs[i] = last_minibuf_string.get();
            }
            b'Z' => {
                if NILP(prefix_arg) {
                    varies[i] = -1;
                } else {
                    args[i] =
                        Fread_non_nil_coding_system(crate::callint::callint_message.get());
                    visargs[i] = last_minibuf_string.get();
                }
            }
            b'z' => {
                args[i] = Fread_coding_system(crate::callint::callint_message.get(), Qnil());
                visargs[i] = last_minibuf_string.get();
            }
            b'+' | _ => {
                invalid_control_letter(&spec_bytes[tem_idx..]);
            }
        }

        if varies[i] == 0 {
            arg_from_tty = true;
        }
        if NILP(visargs[i]) && STRINGP(args[i]) {
            visargs[i] = args[i];
        }

        if prompt_end < string_len {
            tem_idx = prompt_end + 1;
        } else {
            tem_idx = string_len;
        }
        i += 1;
    }
    unbind_to(speccount, Qnil());

    maybe_quit();

    args[0] = Qfuncall_interactively();
    args[1] = function;

    if arg_from_tty || !NILP(record_flag) {
        visargs[1] = function;
        for idx in 2..nargs {
            visargs[idx] = if varies[idx] > 0 {
                list1(intern_cstr(CALLINT_ARGFUNS[varies[idx] as usize]))
            } else {
                quotify_arg(args[idx])
            };
        }
        let hist_form = Flist_slice(&mut visargs.as_mut_slice()[1..nargs]);
        calln!(Qadd_to_history(), Qcommand_history(), hist_form, Qnil(), Qt());
    }

    for idx in 2..nargs {
        if (1..=4).contains(&varies[idx]) {
            args[idx] = make_fixnum(marker_position(args[idx]));
        }
    }

    if record_then_fail {
        Fbarf_if_buffer_read_only(Qnil());
    }

    Vthis_command.set(save_this_command);
    Vthis_original_command.set(save_this_original_command);
    Vreal_this_command.set(save_real_this_command);
    set_current_kboard_last_command(save_last_command);

    specbind(Qcommand_debug_status(), Qnil());
    let val = Ffuncall_slice(&mut args.as_mut_slice()[..nargs]);
    unbind_to(speccount, val)
}

pub fn prefix_numeric_value(raw: Lisp_Object) -> Lisp_Object {
    if NILP(raw) {
        make_fixnum(1)
    } else if EQ(raw, Qminus()) {
        make_fixnum(-1)
    } else if CONSP(raw) && FIXNUMP(XCAR(raw)) {
        XCAR(raw)
    } else if FIXNUMP(raw) {
        raw
    } else {
        make_fixnum(1)
    }
}

pub fn init_syms_of_callint() {
    crate::callint::point_marker.set(Fmake_marker());
    staticpro_global(&crate::callint::point_marker);

    crate::callint::callint_message.set(Qnil());
    staticpro_global(&crate::callint::callint_message);

    let mut fns = [
        intern_cstr(c"use-region-beginning"),
        intern_cstr(c"use-region-end"),
        intern_cstr(c"region-beginning"),
        intern_cstr(c"region-end"),
        intern_cstr(c"point"),
        intern_cstr(c"mark"),
    ];
    crate::callint::preserved_fns.set(Flist_slice(&mut fns));
    staticpro_global(&crate::callint::preserved_fns);
}
