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

## Measured for android/aarch64 (NDK 29)

Probe: `#include "lisp.h"` + `#include "thread.h"`, then `_Static_assert((EXPR)==N,"")`.
C needs the `struct` keyword - `sizeof(thread_state)` alone is a C error, which is what made the
first attempt look like a missing declaration.

    sizeof(struct thread_state)                     592   (darwin 472)
    offsetof(struct thread_state, not_holding_lock) 552   (darwin 432)
    offsetof(struct thread_state, bc)               568   (darwin 448)
    sizeof(lisp_mutex_t)                             64   (darwin 16)
    sizeof(struct Lisp_Mutex)                        80   (darwin 32)

The five are now per-target in `src/include/thread.rs` (`feature = "have_android"`), and
darwin still builds.

## What is still open

With those numbers in place the Android unit still fails the first assertion, and the port's own
`thread_state` reads **520** against C's **592** - a 72-byte gap.  The `systhread` members are
right (`sys_cond_t` 48, `sys_mutex_t` 40, `sys_thread_t` 8, all matching C), and both structs
count 32 fields, so the difference is in a member's shape rather than a missing type: the C field
list names `string` and `java_env`, and the port's names differ (`m_`-prefixed) but the pair does
not obviously account for 72 bytes.  Next: compare the two structs field by field with the offsets
C reports (`offsetof` bisect per field), which pins the differing member exactly.

## Numbers already measured (P-232, unchanged)

    pthread_cond_t 48   pthread_mutex_t 40   pthread_attr_t 56
    pthread_mutexattr_t 8   pthread_condattr_t 8   pthread_t 8

## `struct frame`, measured 2026-10-19 (status 4.811)

    CC=$(ls ~/Library/Android/sdk/ndk/*/toolchains/llvm/prebuilt/*/bin/aarch64-linux-android21-clang | tail -1)
    mkdir -p /tmp/andinc && cp /tmp/femacs-android-config/src/config.h.android /tmp/andinc/config.h
    printf '#include "lisp.h"\n#include "frame.h"\n' > /tmp/probe_frame.c
    $CC -std=gnu23 -Xclang -fdump-record-layouts-complete -fsyntax-only \
        -include /tmp/andinc/config.h -I/tmp/andinc -Iemacs/src -Ibuild/emacs-ref/src \
        -Ibuild/emacs-ref/lib -I/tmp/femacs-android-config/lib -Iemacs/lib /tmp/probe_frame.c

(The run stops at `stdcountof.h`, which only the Android gnulib build generates; the record dump is
still written, and `struct frame` is complete in it.)

    param_alist        88
    face_cache        240
    image_cache       248
    desired_matrix    296
    current_matrix    304
    output_data       520

These six are now `const _: () = core::assert! (core::mem::offset_of! (frame, …) == N);` in
`src/include/frame.rs` under `have_android`/`not_android_stubify`, so a `#[cfg]` arm that
disagrees with the configuration breaks the Android build instead of shifting every later field -
`output_data.android` is what `android_new_font` writes `FRAME_FONTSET` through.
