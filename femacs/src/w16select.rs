/* 16-bit Windows Selection processing for emacs on MS-Windows

Copyright (C) 1996-1997, 2001-2026 Free Software Foundation, Inc.

Author: Dale P. Smith <dpsm@en.com>

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

/* These functions work by using WinOldAp interface.  WinOldAp
   (WINOLDAP.MOD) is a Microsoft Windows extension supporting
   "old" (character-mode) application access to Dynamic Data Exchange,
   menus, and the Windows clipboard.  */

/* Adapted to DJGPP by Eli Zaretskii <eliz@gnu.org>  */

/* femacs: emacs/src/w16select.c (@ 694 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef MSDOS` at C line 29, and the frozen target's
   configuration leaves MSDOS undefined (docs/baseline/config/darwin-arm64.macros.tsv
   1391, `MSDOS<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the MS-DOS (djgpp) clipboard support - needs
   <dpmi.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
