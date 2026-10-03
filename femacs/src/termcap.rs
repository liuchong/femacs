// femacs: port of src/termcap.c @ fd7afced403 (GNU Emacs 32.0.50).
/* Work-alike for termcap, plus extra features.
   Copyright (C) 1985-1986, 1993-1995, 2000-2008, 2011, 2013-2026 Free
   Software Foundation, Inc.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Since 2010-03, 073589f4, Emacs 24.1, this file is only used
   by the MS-DOS port of Emacs.  */


/* femacs: emacs/src/termcap.c (@ 640 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/termcap.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
