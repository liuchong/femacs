// femacs: port of src/w16select.c @ fd7afced403.
/* These functions work by using WinOldAp interface.  WinOldAp
   (WINOLDAP.MOD) is a Microsoft Windows extension supporting
   "old" (character-mode) application access to Dynamic Data Exchange,
   menus, and the Windows clipboard.  */

/* Adapted to DJGPP by Eli Zaretskii <eliz@gnu.org>  */

/* femacs: emacs/src/w16select.c (@ 694 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MSDOS` at C line 29, and the frozen target's
   configuration leaves MSDOS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1391, `MSDOS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the MS-DOS (djgpp) clipboard support - needs
   <dpmi.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
