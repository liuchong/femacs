// femacs: port of src/w32console.c @ fd7afced403.
/*
   Tim Fleehart (apollo@online.com)		1-17-92
   Geoff Voelker (voelker@cs.washington.edu)	9-12-93
   Ewan Townshend (ewan@etown.dev)              2025-08
   * c. ~ 2025: 24bit RGB support in Windows (10+) Terminal
   * https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
*/


/* femacs: emacs/src/w32console.c (@ 1262 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/w32console.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
