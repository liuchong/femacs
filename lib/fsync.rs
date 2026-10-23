// femacs: port of lib/fsync.c @ fd7afced403.
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
