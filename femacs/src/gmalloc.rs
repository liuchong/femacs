// femacs: port of src/gmalloc.c @ fd7afced403.
/* Declarations for `malloc' and friends.
   Copyright (C) 1990-1993, 1995-1996, 1999, 2002-2007, 2013-2026 Free
   Software Foundation, Inc.
		  Written May 1989 by Mike Haertel.

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the
License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public
License along with this library.  If not, see <https://www.gnu.org/licenses/>.

   The author may be reached (Email) at the address mike@ai.mit.edu,
   or (US mail) as Mike Haertel c/o Free Software Foundation.  */


/* femacs: emacs/src/gmalloc.c (@ 1908 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/gmalloc.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
