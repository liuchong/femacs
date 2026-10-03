/* MS-DOS specific C utilities.          -*- coding: cp850 -*-

Copyright (C) 1993-1997, 1999-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

/* Contributed by Morten Welinder */
/* New display, keyboard, and mouse control by Kim F. Storm */

/* Note: This file MUST use a unibyte encoding, to both display the
   keys on the non-US keyboard layout as their respective labels, and
   provide the correct byte values for the keyboard input to inject
   into Emacs.  See 'struct dos_keyboard_map' below.  As long as there
   are only European keyboard layouts here, we are OK with DOS
   codepage 850 encoding.  */

/* Note: some of the stuff here was taken from end of sysdep.c in demacs. */

/* femacs: emacs/src/msdos.c (@ 4364 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MSDOS` at C line 34, and the frozen target's
   configuration leaves MSDOS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1391, `MSDOS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the whole MS-DOS (djgpp) system layer - needs
   <setjmp.h> and the rest of the MS-DOS port, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
