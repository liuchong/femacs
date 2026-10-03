// femacs: port of src/androidfont.c @ fd7afced403.
/* Due to the terrible nature of the Android Typeface subsystems, this
   font driver is only used as a fallback when sfntfont-android.c
   fails to enumerate any fonts at all.  */


/* femacs: emacs/src/androidfont.c (@ 1102 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/androidfont.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
