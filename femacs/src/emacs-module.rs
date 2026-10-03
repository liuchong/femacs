// femacs: port of src/emacs-module.c @ fd7afced403.
/* emacs-module.c - Module loading and runtime implementation

Copyright (C) 2015-2026 Free Software Foundation, Inc.

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

/*
The public module API is defined in the header emacs-module.h.  The
configure script generates emacs-module.h from emacs-module.in.h and
the version-specific environment fragments in module-env-*.h.

If you want to change the module API, please abide to the following
rules:

- Don't remove publicly documented declarations from the headers.

- Don't remove, reorder, or rename structure fields, as such changes
  break ABI compatibility.

- Don't change the types of structure fields.

- Likewise, the presence, order, and type of structure fields may not
  depend on preprocessor macros.

- Add structure fields only at the end of structures.

- For every Emacs major version there is a new fragment file
  module-env-VER.h.  Add functions solely at the end of the fragment
  file for the next (not yet released) major version of Emacs.  For
  example, if the current Emacs release is 26.2, add functions only to
  module-env-27.h.

- emacs-module.h should only depend on standard C headers.  In
  particular, don't include config.h or lisp.h from emacs-module.h.

- The contents of emacs-module.h should be the same on all platforms
  and architectures.

- emacs-module.h may not depend on Emacs configuration options.

- Prefix all names in emacs-module.h with "emacs_" or "EMACS_".

To add a new module function, proceed as follows:

1. Add a new function pointer field at the end of the module-env-*.h
   file for the next major version of Emacs.

2. Run config.status or configure to regenerate emacs-module.h.

3. Create a corresponding implementation function in this file.  See
   "Implementation of runtime and environment functions" below for
   further rules.

4. Assign the new field in the initialize_environment function.

5. Add a test function that calls your new function to
   test/data/emacs-module/mod-test.c.  Add a unit test that invokes
   your new test function to test/src/emacs-module-tests.el.

6. Document your new function in the manual and in etc/NEWS.
*/


/* femacs: emacs/src/emacs-module.c (@ 1798 lines) is not compiled by the frozen configuration.

   `build/emacs-ref/src/Makefile`'s platform object variables are empty for this target
   (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`), so the file is not in
   the build's source list at all; its text also needs headers this target has not got.  The
   authority is `docs/baseline/frozen-build-sources.txt` (generated from `make -n` in that
   build directory) and the policy is docs/status.md 3.31: the port is an empty unit with this
   note, and `docs/ports/emacs-module.md` records the deferred declarations so a future configuration
   that builds it knows where to start.  */
