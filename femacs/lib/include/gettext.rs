// femacs: WIP mirror of lib/gettext.h @ fd7afced403 (PORTING.md P-26).
//
// The regex cluster only names `gettext_noop` (regcomp.c's `__re_error_msgid` table);
// the rest of the header is tracked in docs/ports/include-gettext.md.  `gettext_noop`
// is the identity (gettext.h 350), and `gettext`/`_`/`N_` are the same here because the
// port has no message catalog yet.

/* gettext.h 350: `#define gettext_noop(String) String`  */
macro_rules! gettext_noop {
    ($string:expr) => { $string };
}

/* gettext.h 296 (the non-`_libgettext` arm): the identity until a catalog exists.  */
macro_rules! gettext {
    ($msgid:expr) => { $msgid };
}

/* gettext.h's conventional short spellings.  */
macro_rules! N_ {
    ($string:expr) => { $string };
}
/* gettext.h's `_(Msgid)`: `_` is not a Rust identifier, so the port names the call
   sites' helper `gettext!` (the ledger records the rename, PORTING.md 0.2).  */
