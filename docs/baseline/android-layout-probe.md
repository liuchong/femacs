# Measuring C's Android layout numbers without running the target

The port's layout assertions (`size_of::<thread_state>()`, `offset_of!`, `sizeof(lisp_mutex_t)`,
`sizeof(Lisp_Mutex)`) carry **darwin** numbers, so they fail on Android (P-233).  The numbers have
to be measured, and the target cannot run here - but a layout number does not need running:
`_Static_assert((EXPR)==N, "")` + `-fsyntax-only` + bisect over N reports it in tens of
milliseconds per candidate (the same method P-232 used for the pthread types).

## What works so far (2026-10-18)

Interpreter: NDK 29's `aarch64-linux-android21-clang`, `-std=gnu23`.

    tools/gen-android-config.sh            # gives /tmp/femacs-android-config/{src,lib}
    cp /tmp/femacs-android-config/src/config.h.android /tmp/andinc/config.h
    (cd /tmp/femacs-android-config/lib && make -j2)   # generates the gnulib headers
                                                      # (it stops at regex.o, but the headers exist)

Include set:

    -include /tmp/andinc/config.h \
    -I/tmp/andinc -Iemacs/src -Ibuild/emacs-ref/src -I/tmp/femacs-android-config/lib -Iemacs/lib

`build/emacs-ref/src` is needed for `globals.h`; `/tmp/femacs-android-config/lib` supplies the
gnulib headers generated *for this configuration* (pointing at the darwin build's copies instead
makes `time.h` redefine `struct tm_zone` and breaks the probe).

## What is still open

With that set a probe including `lisp.h` and `thread.h` compiles cleanly, but
`sizeof(thread_state)` reports `use of undeclared identifier 'thread_state'` - `struct
thread_state` sits unguarded at `emacs/src/thread.h` 62, so something in the include chain is
ending early.  Next round: find that (start with `-H` / `-E` on the probe and look for where
`thread.h` stops), then the six numbers, then write them per target in `include/thread.rs`.

## Numbers already measured (P-232, unchanged)

    pthread_cond_t 48   pthread_mutex_t 40   pthread_attr_t 56
    pthread_mutexattr_t 8   pthread_condattr_t 8   pthread_t 8
