/* Function for handling the GLib event loop.

Copyright (C) 2009-2026 Free Software Foundation, Inc.

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

/* femacs: emacs/src/xgselect.c (@ 296 lines) has no live text in the frozen configuration.

   Its whole body is inside `#ifdef HAVE_GLIB` at C line 24, and the frozen target's
   configuration leaves HAVE_GLIB undefined (docs/baseline/config/darwin-arm64.macros.tsv
   781, `HAVE_GLIB<TAB>undef`), so the C preprocessor compiles nothing from the file beyond
   its two includes.  What the file implements when the feature is on - the GLib main-loop integration (xgselect_acquire and friends) - needs
   <glib.h>, which this target has not got; carrying that text would be the class
   docs/status.md 3.5 records for a wholly dead file, and its tokens are therefore not written
   out.  The port is an empty unit: the same zero definitions the reference compiles here, with
   the gating macro recorded so a future configuration that enables it knows which `#if` to
   re-read.  */
