# Android layout gaps in the redisplay records (measured 2026-10-19, status 4.812)

The port's records are `#[repr(C)]` mirrors of the reference headers, but several of them were
modelled from arms that this target does not take.  Nothing in the port allocated from the wrong
size until the first frame existed, so the difference never showed as a compile error - it shows as
a crash, because these are exactly the records the redisplay allocates and walks.

## How these numbers were measured

    CC=$(ls ~/Library/Android/sdk/ndk/*/toolchains/llvm/prebuilt/*/bin/aarch64-linux-android21-clang | tail -1)
    mkdir -p /tmp/andinc && cp /tmp/femacs-android-config/src/config.h.android /tmp/andinc/config.h
    printf '#include "lisp.h"\n#include "frame.h"\n' > /tmp/probe_frame.c
    $CC -std=gnu23 -Xclang -fdump-record-layouts-complete -fsyntax-only \
        -include /tmp/andinc/config.h -I/tmp/andinc -Iemacs/src -Ibuild/emacs-ref/src \
        -Ibuild/emacs-ref/lib -I/tmp/femacs-android-config/lib -Iemacs/lib /tmp/probe_frame.c

(Full method and the `struct frame` offsets: `docs/baseline/android-layout-probe.md`.)

## Gaps

| record | port | NDK (bionic aarch64) | delta | status |
|---|---|---|---|---|
| `struct window` | 536 → **544** | 544 | -8 | **fixed** (missing `ephemeral_last_point`, `HAVE_TEXT_CONVERSION`) |
| `struct glyph_string` | 224 → **248** | 248 | -24 | **fixed** (`clip` is `Emacs_Rectangle[2]`, not `int[2]`) |
| `struct glyph_row` | 272 → **264** | 264 | +8 | **fixed** (`continuation_lines_width` precedes `clip`) |
| `struct face` (canonical, `include/dispextern.rs`) | 296 | 296 | 0 | **ok** - 10 offsets + size asserted in `xfaces.rs` |
| `struct glyph` | 56 | 56 | ok | `femacs/src/include/dispextern.rs` |
| `struct glyph_pool` | 24 | 24 | ok | `femacs/src/include/dispextern.rs` |
| `struct glyph_matrix` | 104 | 104 | ok | `femacs/src/include/dispextern.rs` |

`struct frame`'s sizes/offsets were checked the same way and **agree** (six assertions in
`femacs/src/include/frame.rs`); the three records that agree carry `const _: () = assert!` now, so
they cannot drift silently.

## Why it matters

`make_face_cache` sizes its vector from `sizeof (struct face)`, `make_glyph_pool` from
`sizeof (struct glyph_pool)`, `build_frame_matrix`/`allocate_glyph_matrix` walk `glyph_row` by
`sizeof`, and every `struct window` is one Lisp pseudovector sized by `sizeof (struct window)`.
A short record means the next field of the neighbouring object is overwritten; a long one means the
last field read is past the allocation.  Both are exactly the SIGSEGV/SIGBUS the device shows when
the first frame is realized.

## Fix order

1. ~~`struct window` (-8)~~ - **done**: `ephemeral_last_point` was missing (C 288-303,
   `HAVE_TEXT_CONVERSION`, live on Android).
2. ~~`struct glyph_string` (-24)~~ - **done**: `clip` is `NativeRectangle clip[2]`, and
   `androidgui.h` 218 makes `NativeRectangle` `Emacs_Rectangle`, i.e. 32 bytes, not 8.
3. ~~`struct glyph_row` (+8)~~ - **done**: the port had `clip` *before* `continuation_lines_width`;
   C's order is the other way round (CLW 252, clip 256).
4. ~~`struct face` (-112)~~ - the **canonical** record was already right (296); the 112 bytes were
   a measurement artifact of evaluating the assertions in a unit that shadows `face`.  What *was*
   wrong: `dispnew.rs`, `alloc.rs` and `font.rs` each carried a `face` copy pinned to the **host**
   offsets (`background` 176, `font` 216, size 280, and `font.rs` had no window-system block at
   all).  All three now shift by that block (`+16`, `cfg!(have_window_system)`) and assert the
   Android numbers (`dispnew`: `background` 192; `alloc`: `font` 232, size 296; `font`: size
   296 and every offset past `id` shifted).  `androidterm.rs` already had the block.

## Every `face` in the Android build is now target-correct

| unit | what it carries | asserted |
|---|---|---|
| `xfaces` (canonical) | full record | size 296, `id` 160, `font` 232, `ascii_face` 288 |
| `androidterm`, `fontset`, `fringe`, `image` | full record with the window-system block | size 296, `font` 232 |
| `dispnew` | `background` only | `background` 192 |
| `alloc` | `font` only | `font` 232, size 296 |
| `font` | full record, block added this round | size 280 + block, every offset past `id` shifted |
| `term` | `lface`/colours/`bits` | `foreground` 184, size 296 |
| `window` | `font` only | `font` 232, size 296 |
| `xdisp` | full record | size 296, `font` 232, `background` 192 |

## Why four copies existed at all

`include/dispextern.rs` defines the canonical `struct face` under
`#[cfg (feature = "dispextern_face_records")]`, a synthetic feature nothing turns on, so each unit
that needs the record declares its own partial copy instead (P-88).  That is how three of them kept
the host numbers: nothing compared them with the target.  The follow-up is to drop the synthetic
gate and let every unit use the canonical record (`androidterm.rs` already carries it verbatim), so
there is exactly one `face` in the tree.

Each one is fixed the way `struct dirent`/`struct stat` were: compare the field list with the NDK
dump, add the per-target arm, and turn the size into a `const` assertion so the next drift fails the
build instead of the device.
