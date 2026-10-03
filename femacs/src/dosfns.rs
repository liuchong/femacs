// femacs: port of src/dosfns.c @ fd7afced403.
/* femacs: emacs/src/dosfns.c (@ 804 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MSDOS` at C line 23, and the frozen target's
   configuration leaves MSDOS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1391, `MSDOS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the MS-DOS file-name primitives - needs
   <dos.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
