// femacs: port of src/msdos.c @ fd7afced403.
/* Contributed by Morten Welinder */
/* New display, keyboard, and mouse control by Kim F. Storm */

/* Note: This file MUST use a unibyte encoding, to both display the
   keys on the non-US keyboard layout as their respective labels, and
   provide the correct byte values for the keyboard input to inject
   into Emacs.  See 'struct dos_keyboard_map' below.  As long as there
   are only European keyboard layouts here, we are OK with DOS
   codepage 850 encoding.  */

/* Note: some of the stuff here was taken from end of sysdep.c in demacs. */

/* femacs: emacs/src/msdos.c (@ 4364 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MSDOS` at C line 34, and the frozen target's
   configuration leaves MSDOS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1391, `MSDOS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the whole MS-DOS (djgpp) system layer - needs
   <setjmp.h> and the rest of the MS-DOS port, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
