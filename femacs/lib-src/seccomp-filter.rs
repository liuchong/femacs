// femacs: port of lib-src/seccomp-filter.c @ fd7afced403.

/* femacs: emacs/lib-src/seccomp-filter.c (@ 409 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/lib-src/Makefile`'s program lists select this directory's units -
   `INSTALLABLES` (etags, emacsclient, ebrowse), `UTILITIES` (hexl, movemail, and
   update-game-score only `$(and $(use_gamedir), ...)`) and `DONT_INSTALL` (make-docfile,
   make-fingerprint) - and this file is in none of them: the program is built only with `--with-seccomp`, and this configuration has no seccomp.  The authority is
   `docs/baseline/frozen-build-sources.txt` (its batch-F section records the derivation from
   those lists) and the policy is docs/status.md 3.31: the port is an empty unit with this note,
   and `docs/ports/seccomp-filter.md` records the deferred declarations so a future configuration that
   builds it knows where to start.  */
