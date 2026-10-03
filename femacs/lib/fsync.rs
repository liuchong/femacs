/* Emulate fsync on platforms that lack it, primarily Windows and
   cross-compilers like MinGW.

   This is derived from sqlite3 sources.
   https://www.sqlite.org/src/finfo?name=src/os_win.c
   https://www.sqlite.org/copyright.html

   Written by Richard W.M. Jones <rjones.at.redhat.com>

   Copyright (C) 2008-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* femacs: emacs/lib/fsync.c (@ 86 lines) has no live text in the frozen configuration.

   C 28's `#if defined _WIN32 && ! defined __CYGWIN__` arm - the whole `FlushFileBuffers`
   implementation with its `<windows.h>`, `msvc-nothrow.h`/`<io.h>` includes and its DWORD
   error mapping - is dead on darwin-arm64, and the `#else` arm the frozen configuration selects
   is C 84-85's
       #error "This platform lacks fsync function, and Gnulib doesn't provide a replacement."
   i.e. the reference *refuses to compile* this file here (which is why `lib/fsync.c` is not in
   the frozen build's object list).  The port is an empty unit that records exactly that: the
   zero definitions the reference contributes, with the two arms named so a future configuration
   knows which one to re-read.  docs/status.md 3.25 records the policy.  */
