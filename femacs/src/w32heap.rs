// femacs: port of src/w32heap.c @ fd7afced403.
/*
  Geoff Voelker (voelker@cs.washington.edu)                          7-29-94
*/

/*
  Heavily modified by Fabrice Popineau (fabrice.popineau@gmail.com) 28-02-2014
*/

/*
  Memory allocation scheme for w32/w64:

  - Buffers are mmap'ed using a very simple emulation of mmap/munmap
  - During the emacs phase, or always if pdumper is used:
    * we create a private heap for new memory blocks
    * we make sure that we never free a block that has been dumped.
      Freeing a dumped block could work in principle, but may prove
      unreliable if we distribute binaries of emacs.exe: MS does not
      guarantee that the heap data structures are the same across all
      versions of their OS, even though the API is available since XP.  */


/* femacs: emacs/src/w32heap.c (@ 554 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/w32heap.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
