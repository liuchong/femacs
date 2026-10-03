// femacs: port of src/xmenu.c @ fd7afced403.
/* X pop-up deck-of-cards menu facility.
 *
 */

/* Modified by Fred Pierresteguy on December 93
   to make the popup menus and menubar use the Xt.  */

/* Rewritten for clarity and GC protection by rms in Feb 94.  */


/* femacs: emacs/src/xmenu.c (@ 2932 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/xmenu.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
