// femacs: port of src/textconv.c @ fd7afced403.
/* String conversion support.

   Many input methods require access to text surrounding the cursor.
   They may then request that the text editor remove or substitute
   that text for something else, for example when providing the
   ability to ``undo'' or ``edit'' previously composed text.  This is
   most commonly seen in input methods for CJK languages for X Windows,
   and is extensively used throughout Android by input methods for all
   kinds of scripts.

   In addition, these input methods may also need to make detailed
   edits to the content of a buffer.  That is also handled here.  */


/* femacs: emacs/src/textconv.c (@ 2640 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/textconv.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
