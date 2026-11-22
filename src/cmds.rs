// femacs: port of src/cmds.c @ fd7afced403.
//
// Safe Rust command dispatch delegating all core point-movement, line-motion, deletion, and
// self-insertion logic to `#![forbid(unsafe_code)]` `crate::safe_cmds` (`U6` / `P-485`).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables)]
use crate::common::*;

macro_rules! __defun_body {
    ($body:block) => {
        $body
    };
}

#[inline]
fn move_point (n: Lisp_Object, forward: bool) -> Lisp_Object {
    crate::safe_cmds::move_point (n, forward)
}

DEFUN! ("forward-char", Fforward_char, Sforward_char, 0, 1, "^p",
       doc: "Move point N characters forward (backward if N is negative).\nOn reaching end or beginning of buffer, stop and signal error.\nInteractively, N is the numeric prefix argument.\nIf N is omitted or nil, move point 1 character forward.\n\nDepending on the bidirectional context, the movement may be to the\nright or to the left on the screen.  This is in contrast with\n\\[right-char], which see.",
       (n: Lisp_Object),
       {
  move_point (n, true)
});

DEFUN! ("backward-char", Fbackward_char, Sbackward_char, 0, 1, "^p",
       doc: "Move point N characters backward (forward if N is negative).\nOn attempt to pass beginning or end of buffer, stop and signal error.\nInteractively, N is the numeric prefix argument.\nIf N is omitted or nil, move point 1 character backward.\n\nDepending on the bidirectional context, the movement may be to the\nright or to the left on the screen.  This is in contrast with\n\\[left-char], which see.",
       (n: Lisp_Object),
       {
  move_point (n, false)
});

DEFUN! ("forward-line", Fforward_line, Sforward_line, 0, 1, "^p",
       doc: "Move N lines forward (backward if N is negative).\nPrecisely, if point is on line I, move to the start of line I + N\n(\"start of line\" in the logical order).\nIf there isn't room, go as far as possible (no error).\nInteractively, N is the numeric prefix argument and defaults to 1.\n\nReturns the count of lines left to move.  If moving forward,\nthat is N minus number of lines moved; if backward, N plus number\nmoved.\n\nException: With positive N, a non-empty line at the end of the\nbuffer, or of its accessible portion, counts as one line\nsuccessfully moved (for the return value).  This means that the\nfunction will move point to the end of such a line and will count\nit as a line moved across, even though there is no next line to\ngo to its beginning.",
       (n: Lisp_Object),
       {
  crate::safe_cmds::forward_line (n)
});

DEFUN! ("beginning-of-line", Fbeginning_of_line, Sbeginning_of_line, 0, 1, "^p",
       doc: "Move point to beginning of current line (in the logical order).\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf point reaches the beginning or end of buffer, it stops there.\n\nThis function constrains point to the current field unless this moves\npoint to a different line from the original, unconstrained result.\nIf N is nil or 1, and a front-sticky field starts at point, the point\ndoes not move.  To ignore field boundaries bind\n`inhibit-field-text-motion' to t, or use the `forward-line' function\ninstead.  For instance, `(forward-line 0)' does the same thing as\n`(beginning-of-line)', except that it ignores field boundaries.",
       (n: Lisp_Object),
       {
  crate::safe_cmds::beginning_of_line (n)
});

DEFUN! ("end-of-line", Fend_of_line, Send_of_line, 0, 1, "^p",
       doc: "Move point to end of current line (in the logical order).\nWith argument N not nil or 1, move forward N - 1 lines first.\nIf point reaches the beginning or end of buffer, it stops there.\nTo ignore intangibility, bind `inhibit-point-motion-hooks' to t.\n\nThis function constrains point to the current field unless this moves\npoint to a different line from the original, unconstrained result.  If\nN is nil or 1, and a rear-sticky field ends at point, the point does\nnot move.  To ignore field boundaries bind `inhibit-field-text-motion'\nto t.",
       (n: Lisp_Object),
       {
  crate::safe_cmds::end_of_line (n)
});

DEFUN! ("delete-char", Fdelete_char, Sdelete_char, 1, 2, "p\nP",
       doc: "Delete the following N characters (previous if N is negative).\nOptional second arg KILLFLAG non-nil means kill instead (save in kill ring).\nInteractively, N is the prefix arg, and KILLFLAG is set if\nN was explicitly specified.\n\nThe command `delete-forward-char' is preferable for interactive use, e.g.\nbecause it respects values of `delete-active-region' and `overwrite-mode'.",
       (n: Lisp_Object, killflag: Lisp_Object),
       {
  crate::safe_cmds::delete_char (n, killflag)
});

DEFUN! ("self-insert-command", Fself_insert_command, Sself_insert_command, 1, 2,
       "(list (prefix-numeric-value current-prefix-arg) last-command-event)",
       doc: "Insert the character you type.\nWhichever character C you type to run this command is inserted.\nThe numeric prefix argument N says how many times to repeat the insertion.\nBefore insertion, `expand-abbrev' is executed if the inserted character does\nnot have word syntax and the previous character in the buffer does.\nAfter insertion, `internal-auto-fill' is called if\n`auto-fill-function' is non-nil and if the `auto-fill-chars' table has\na non-nil value for the inserted character.  At the end, it runs\n`post-self-insert-hook'.",
       (n: Lisp_Object, c: Lisp_Object),
       {
  crate::safe_cmds::self_insert_command (n, c)
});

#[inline]
fn internal_self_insert (c: c_int, n: EMACS_INT) -> c_int {
    crate::safe_cmds::internal_self_insert (c, n)
}

#[unsafe(no_mangle)]
pub static Vpost_self_insert_hook: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_cmds () {
    DEFSYM! (Qinternal_auto_fill, "internal-auto-fill");
    DEFSYM! (Qundo_auto_amalgamate, "undo-auto-amalgamate");
    DEFSYM! (Qundo_auto__this_command_amalgamating, "undo-auto--this-command-amalgamating");
    DEFSYM! (Qno_self_insert, "no-self-insert");
    DEFSYM! (Qkill_forward_chars, "kill-forward-chars");
    DEFSYM! (Qoverwrite_mode_binary, "overwrite-mode-binary");
    DEFSYM! (Qexpand_abbrev, "expand-abbrev");
    DEFSYM! (Qpost_self_insert_hook, "post-self-insert-hook");

    DEFVAR_LISP! (c"post-self-insert-hook", Vpost_self_insert_hook,
                 "Hook run at the end of `self-insert-command'.\nThis is run after inserting a character.\nThe hook can access the inserted character via `last-command-event'.");
    Vpost_self_insert_hook.set (Qnil ());

    defsubr_global (&Sforward_char);
    defsubr_global (&Sbackward_char);
    defsubr_global (&Sforward_line);
    defsubr_global (&Sbeginning_of_line);
    defsubr_global (&Send_of_line);
    defsubr_global (&Sdelete_char);
    defsubr_global (&Sself_insert_command);
}
