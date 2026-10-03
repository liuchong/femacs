// femacs: port of src/w32dwrite.c @ fd7afced403 (GNU Emacs 32.0.50).
/* Support for using DirectWrite on MS-Windows to draw text.  This
   allows for color fonts.
   Copyright (C) 2024-2026 Free Software Foundation, Inc.

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

/* This requires the HarfBuzz font backend to be available.

   It works by modifying the HarfBuzz backend to use DirectWrite at
   some points, if it is available:

   - When encoding characters: w32hb_encode_char
   - When measuring text: w32font_text_extents
   - When drawing text: w32font_draw

   DirectWrite is setup by calling w32_initialize_direct_write.  From
   that point, the function w32_use_direct_write will return true if
   DirectWrite is to be used.

   DirectWrite is available since Windows 7, but we don't activate it on
   versions before 8.1, because color fonts are only available since that.  */


/* femacs: emacs/src/w32dwrite.c (@ 1399 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/w32dwrite.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
