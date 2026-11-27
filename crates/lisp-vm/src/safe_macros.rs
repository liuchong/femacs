//! `#![forbid(unsafe_code)]` implementation of keyboard macro recording and execution
//! (`src/macros.rs`, `U7` / `P-486`).

#![forbid(unsafe_code)]

use crate::common::*;

pub fn start_kbd_macro_subr(append: Lisp_Object, no_exec: Lisp_Object) -> Lisp_Object {
    if !NILP(current_kboard_defining_kbd_macro()) {
        error_cstr(c"Already defining kbd macro");
    }

    update_mode_lines.set(19);
    if NILP(append) {
        current_kboard_reset_macro_buffer_for_start();
        message1_cstr(c"Defining kbd macro...");
    } else {
        let last_macro = current_kboard_last_kbd_macro();
        let len = CHECK_VECTOR_OR_STRING(last_macro);
        current_kboard_prepare_append_macro_buffer(len);

        let cvt = STRINGP(last_macro);
        for i in 0..len {
            let mut c = Faref(last_macro, make_fixnum(i));
            if cvt && FIXNATP(c) && (XFIXNAT(c) & 0x80) != 0 {
                c = make_fixnum(CHAR_META as EMACS_INT | (XFIXNAT(c) & !0x80));
            }
            current_kboard_write_macro_slot(i, c);
        }
        current_kboard_finish_append_macro_copy(len);

        if NILP(no_exec) {
            Fexecute_kbd_macro(current_kboard_last_kbd_macro(), make_fixnum(1), Qnil());
        }

        message1_cstr(c"Appending to kbd macro...");
    }

    set_current_kboard_defining_kbd_macro(Qt());
    Qnil()
}

pub fn end_kbd_macro_impl() {
    set_current_kboard_defining_kbd_macro(Qnil());
    update_mode_lines.set(20);
    let vec = current_kboard_kbd_macro_to_vector();
    set_current_kboard_last_kbd_macro(vec);
}

pub fn end_kbd_macro_subr(mut repeat: Lisp_Object, loopfunc: Lisp_Object) -> Lisp_Object {
    if NILP(current_kboard_defining_kbd_macro()) {
        error_cstr(c"Not defining kbd macro");
    }

    if NILP(repeat) {
        repeat = make_fixnum(1);
    } else {
        CHECK_FIXNUM(repeat);
    }

    if !NILP(current_kboard_defining_kbd_macro()) {
        end_kbd_macro_impl();
        message1_cstr(c"Keyboard macro defined");
    }

    if XFIXNAT(repeat) == 0 {
        Fexecute_kbd_macro(current_kboard_last_kbd_macro(), repeat, loopfunc);
    } else if XFIXNUM(repeat) > 1 {
        repeat = make_fixnum(XFIXNUM(repeat) - 1);
        Fexecute_kbd_macro(current_kboard_last_kbd_macro(), repeat, loopfunc);
    }
    Qnil()
}

pub fn store_kbd_macro_char_impl(c: Lisp_Object) {
    current_kboard_store_kbd_macro_char(c);
}

pub fn finalize_kbd_macro_chars_impl() {
    current_kboard_finalize_kbd_macro();
}

pub fn cancel_kbd_macro_events_subr() -> Lisp_Object {
    current_kboard_cancel_macro_events();
    Qnil()
}

pub fn store_kbd_macro_event_subr(event: Lisp_Object) -> Lisp_Object {
    store_kbd_macro_char_impl(event);
    Qnil()
}

pub fn call_last_kbd_macro_subr(prefix: Lisp_Object, loopfunc: Lisp_Object) -> Lisp_Object {
    Vthis_command.set(current_kboard_last_command());
    Vreal_this_command.set(current_kboard_last_kbd_macro());

    if !NILP(current_kboard_defining_kbd_macro()) {
        error_cstr(c"Can't execute anonymous macro while defining one");
    } else if NILP(current_kboard_last_kbd_macro()) {
        error_cstr(c"No kbd macro has been defined");
    } else {
        Fexecute_kbd_macro(current_kboard_last_kbd_macro(), prefix, loopfunc);
    }

    Vthis_command.set(current_kboard_last_command());
    Qnil()
}

pub extern "C-unwind" fn pop_kbd_macro(info: Lisp_Object) {
    Vexecuting_kbd_macro.set(XCAR(info));
    let tem = XCDR(info);
    if let Some(idx) = integer_to_intmax_opt(XCAR(tem)) {
        executing_kbd_macro_index.set(idx as EMACS_INT);
    }
    Vreal_this_command.set(XCDR(tem));
    run_hook(Qkbd_macro_termination_hook());
}

pub fn execute_kbd_macro_subr(
    macro_: Lisp_Object,
    mut count: Lisp_Object,
    loopfunc: Lisp_Object,
) -> Lisp_Object {
    let pdlcount = SPECPDL_INDEX();
    let mut repeat: EMACS_INT = 1;
    let mut success_count: EMACS_INT = 0;

    executing_kbd_macro_iterations.set(0);

    if !NILP(count) {
        count = Fprefix_numeric_value(count);
        repeat = XFIXNUM(count);
    }

    let final_ = indirect_function(macro_);
    if !STRINGP(final_) && !VECTORP(final_) {
        error_cstr(c"Keyboard macros must be strings or vectors");
    }

    let tem = Fcons(
        Vexecuting_kbd_macro.get(),
        Fcons(
            make_int(executing_kbd_macro_index.get() as intmax_t),
            Vreal_this_command.get(),
        ),
    );
    record_unwind_protect_lisp_fn(pop_kbd_macro, tem);

    loop {
        Vexecuting_kbd_macro.set(final_);
        executing_kbd_macro.set(final_);
        executing_kbd_macro_index.set(0);

        set_current_kboard_prefix_arg(Qnil());

        if !NILP(loopfunc) {
            let cont = call0(loopfunc);
            if NILP(cont) {
                break;
            }
        }

        command_loop_2(list1(Qminibuffer_quit()));

        success_count += 1;
        executing_kbd_macro_iterations.set(success_count);

        maybe_quit();

        repeat -= 1;
        let cur = Vexecuting_kbd_macro.get();
        if !(repeat != 0 && (STRINGP(cur) || VECTORP(cur))) {
            break;
        }
    }

    executing_kbd_macro.set(Qnil());
    Vreal_this_command.set(Vexecuting_kbd_macro.get());

    unbind_to(pdlcount, Qnil())
}

pub fn init_macros_impl() {
    Vexecuting_kbd_macro.set(Qnil());
    executing_kbd_macro.set(Qnil());
}

pub fn at_end_of_macro_p_impl() -> bool {
    let cur = Vexecuting_kbd_macro.get();
    eassume!(!NILP(cur));
    EQ(cur, Qt()) || executing_kbd_macro_index.get() >= XFIXNAT(Flength(cur))
}
