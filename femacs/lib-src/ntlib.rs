// femacs: port of lib-src/ntlib.c @ fd7afced403.

/* femacs: emacs/lib-src/ntlib.c (@ 601 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/lib-src/Makefile`'s program lists select this directory's units -
   `INSTALLABLES` (etags, emacsclient, ebrowse), `UTILITIES` (hexl, movemail, and
   update-game-score only `$(and $(use_gamedir), ...)`) and `DONT_INSTALL` (make-docfile,
   make-fingerprint) - and this file is in none of them: this file is the Windows port's support library (it is reached by `nt/` builds and by WINDOWSNT units only) and appears in no program list.  The authority is
   `docs/baseline/frozen-build-sources.txt` (its batch-F section records the derivation from
   those lists) and the policy is docs/status.md 3.31: the port is an empty unit with this note,
   and `docs/ports/ntlib.md` records the deferred declarations so a future configuration that
   builds it knows where to start.  */
