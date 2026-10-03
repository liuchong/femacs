// femacs: port of src/decompress.c @ fd7afced403.
/* femacs: emacs/src/decompress.c (@ 352 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef HAVE_ZLIB` at C line 21, and the frozen target's
   configuration leaves HAVE_ZLIB undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1124, `HAVE_ZLIB<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the zlib-backed `zlib-decompress-region` - needs
   <zlib.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
