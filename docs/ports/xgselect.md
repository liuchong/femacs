# emacs/src/xgselect.c (296 lines)

# ----------------------------------------------------------------------
## Decisions

- **The file gate is `feature = "have_glib"` (C 24), on every item *and on C 26-31's include graph*
  (P-86 ⑧).**  The frozen configuration leaves `HAVE_GLIB` undefined
  (`docs/baseline/config/darwin-arm64.macros.tsv`), so C's whole body is dead text and this target's
  `build-tu` object is the 336-byte empty one - while the *text* is carried in full, because the dead
  body is what a GLib build compiles.  This is the P-86 class: the checklist `extract-abi.py`
  generates is empty (clause 2), so the range table below is the accounting.
- **Four nested conditions, both arms carried** (P-86 ①): C 42/54's `#if GNUC_PREREQ (4, 7, 0)` is
  spelled with the port's own `GNUC_PREREQ!` (`include/conf_post.rs` 50, reached through config.h's
  AH_BOTTOM edge at C 20); C 119/129's `#ifdef USE_GTK` is `feature = "use_gtk"`; C 131's
  `#ifndef HAVE_PGTK` is `not (have_pgtk)`; C 267's `#ifdef HAVE_XWIDGETS` is
  `feature = "have_xwidgets"`.
- **`__atomic_*` (C 43/55) follow PORTING.md §6**: `threads_holding_glib_lock` is an `AtomicIsize` and
  C 43's `__atomic_sub_fetch` / C 55's `__atomic_fetch_add` get the same orderings.  §6 names this
  file as one of the two *mixed* places (C 46's plain `--` reads the same object C 43 reads
  atomically), so the `#else` arm's `++`/`--` become that object's atomic ops with `SeqCst` -
  Deviation 2.
- **C 103's `xg_select` is the biggest body of the file** (C 104-294) and is carried statement for
  statement: the `pselect` fallback (C 123-124), the GLib context lock (C 126-127), the
  pending-events probe (C 129-134), the `fd_set` copies (C 136-139), `g_main_context_query`
  (C 141-142 and 152-153), the `xnmalloc` growth path (C 144-154), the `GPollFD` loop (C 156-169),
  the `make_timespec`/`timespec_cmp` timeout (C 174-180), the two `thread_select` arms
  (C 182-186 `#ifndef USE_GTK`, C 187-220 `#else`), the result translation (C 222-247), the
  dispatch block (C 271-285) and the final early return (C 287-293).
- **Edges the C text reaches through other headers, spelled by the unit** (P-86 ⑧, the
  `src/msdos.c` C 36 reading): `include/thread.rs` (`thread_select`, C 31's chain),
  `include/xterm.rs` (`x_gtk_use_native_input`, C 132), `lib/include/stdcountof.rs` (lisp.h 28, the
  `countof` at C 115), `lib/include/sys_types.in.rs` (process.h 22-24's `<sys/types.h>`, where
  `pid_t` comes from - process.h 134 is the field that names it).  `include/gtk.rs` is
  `<glib.h>`'s home (P-90 ②: one surface, one mirror); its gdkx/gtkx entries carry
  `feature = "use_gtk"`, because C reaches *those* only through `<gtk/gtk.h>`.
- **The mirror's own four declarations live in `pub mod xgselect_decls` + glob import** (P-39,
  PORTING.md 0.1-10): this unit *defines* all four (`release_select_lock`, `acquire_select_lock`,
  `suppress_xg_select`, `release_xg_select`, plus `xg_select`), and a same-named `extern "C"`
  declaration at the same scope is `E0428` in Rust - an explicit item shadows a glob import.

## C line ranges

The rows are **contiguous** on purpose: `docs/port-map.md` binds this ledger to `src/xgselect.c`
*and* to `src/xgselect.h` (both map to a file named `xgselect`), and
`tools/check-dead-file-coverage.py` reads every range against **both** references - so each row opens
on the blank line that precedes its block, and the union covers C 1-296 without a gap (which covers
the header's own 35 lines with it).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block (`/* Function for handling the GLib event loop.` + GPL) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-25 | the blank and C 20 `#include <config.h>` (+ its AH_BOTTOM `<conf_post.h>`, so `GNUC_PREREQ` is `include/conf_post.rs`), C 22 `#include "xgselect.h"`, C 24 `#ifdef HAVE_GLIB` | config.h and its AH_BOTTOM are ungated in C too; **C 24 is the file gate** |
| 26-32 | the blank and the six includes: `<glib.h>`, `<errno.h>`, `lisp.h`, `blockinput.h`, `systime.h`, `process.h` | file gate; `<glib.h>`'s home is `include/gtk.rs` (P-90 ②); `lisp.h`'s `<stdcountof.h>` and `process.h`'s `<sys/types.h>` are the two spelled edges |
| 33-38 | C 33 `static ptrdiff_t threads_holding_glib_lock;`, C 34 `static GMainContext *glib_main_context;`, C 36's comment and C 37 `static int xg_select_suppress_count;`, and the blank | file gate; the first is `AtomicIsize` (§6) |
| 39-50 | the blank and `release_select_lock` (C 39-49, name at C 40) | file gate; C 42/45's two `GNUC_PREREQ` arms, both carried |
| 51-74 | the blank and `acquire_select_lock` (C 51-73, name at C 52) | file gate; the same two arms (C 54/63), the `Spin.` loops at C 60 and C 69 |
| 75-82 | the blank, C 75-76's comment and `suppress_xg_select` (C 77-81) | file gate; its `++` is the plain arm's (Deviation 2) |
| 83-91 | the blank and `release_xg_select` (C 83-90) | file gate |
| 92-102 | the `xg_select` doc comment (C 92-102) | carried as the port's comment at the same place |
| 103-295 | C 103's `int` and `xg_select` (C 104-295): the `pselect` fallback (C 123-124), the lock (C 126-127), the pending probe (C 129-134), the `fd_set` copies (C 136-139), `g_main_context_query` (C 141-142/152-153), the `xnmalloc` path (C 144-154), the `GPollFD` loop (C 156-169), `xfree` (C 171-172), the timeout (C 174-180), the two `thread_select` arms (C 182-186 `#ifndef USE_GTK` / C 187-220 `#else`), the result translation (C 222-247), `need_to_dispatch` (C 251-255), `HAVE_XWIDGETS` (C 267-269), the dispatch block (C 271-285) and the early return (C 287-291) | file gate + `use_gtk`/`have_pgtk`/`have_xwidgets` arms as noted per line |
| 296 | `#endif /* HAVE_GLIB */` | carried as the port's last line |

## Evidence

- frozen `build-tu src/xgselect.c` -> `built src/xgselect.rs -> ../build/femacs/xgselect.o`,
  **exit 0, 336 bytes** (the empty object the file gate leaves);
- **arm flip**: `probe src/xgselect.rs --crate-type=lib -C panic=abort --cfg 'feature="have_glib"'`
  -> **exit 0, 0 errors, 6 warnings, 8,795,712-byte rlib**.  The six warnings are all
  `unused doc comment` on `include/lisp.rs`'s `unsafe extern "C"` blocks (mirror-side, pre-existing).
- **The first flip reported 15 errors; four defect classes were the unit's own text and were fixed**
  (the rest were this unit's missing include edges or the mirror's): `xnmalloc`'s arguments
  (`usize` -> `isize`, the `src/emacs.rs` 2308 shape), `xfree (gfds as *mut c_void)`, five
  `FD_ISSET (…) != 0` comparisons (the mirror's `lib/include/sys_select.rs` 64 returns `bool`),
  `tmo_in_millisec` initialised (C 117 declares it valueless; `&raw mut` needs a live value), and
  `mut fds_lim` (C 183/204 reassign the parameter).  Also fixed: `use std::ffi::{c_char, c_int,
  c_void}` duplicated `include/lisp.rs` 14-15's import (`E0252` x3), `GNUC_PREREQ` x2 and `countof`
  x1 were missing their mirrors' edges, `pid_t` needed `sys_types.in.rs`, and the unit's own
  definitions clashed with `include/xgselect.rs`'s declarations (`E0428` x4 -> the glob module).
- **The four `Window`/`Display` errors the first flip reported are `include/gtk.rs`'s**: its
  gdkx/gtkx entries name X11 types whose home is `include/xlib.rs` (847/880, gated
  `have_x_windows`), which a GLib-only graph does not have.  They now carry
  `feature = "use_gtk"` (C's reachability: `<gtk/gtk.h>`), so this unit's graph no longer sees them.
- **Citation repair**: a first pass found the inline `C n` cite map drifted from the frozen file
  (the locals were cited as `C 98-109` and are `C 107-118`; the statics, `#if` lines, block ranges
  and the ones in the tail were off by 3 to 13 lines).  **66 citation lines were rewritten** and the
  result re-derived against `emacs/src/xgselect.c` cite by cite; `check-citations.py` (which reads
  `<file> <line>` cites) went from 1 drift to **0 drift** with it.
- `extract-abi.py --check emacs/src/xgselect.c femacs/src/xgselect.rs` -> **OK** (the unit defines no
  DEFUN, so this is the declaration-level reading);
- `xtask layout-check` -> **exit 0** (`include/gtk.rs` and `include/xgselect.rs` changed);
- `tools/check-all-units.sh` (the P-87 two legs, both required for a mirror change) ->
  **`checked 83 unit(s), 0 failure(s)` + `checking 83 unit(s), 0 unit failure(s)`**;
- `check-declarations.py` -> 10 declaration sites (0 forbidden, 10 allowed literals), 0 failures.

## Deviations

1. The two `GNUC_PREREQ (4, 7, 0)` arms are carried as a runtime `if` on the port's macro (P-86 ①);
   C's `#if` chooses one at compile time, and both arms' bodies are carried verbatim.
2. `threads_holding_glib_lock` is one object of one kind in Rust: the atomic arm uses
   `fetch_sub`/`fetch_add` with C's orderings, and the plain arm's `--`/`++` (C 46/64) become the
   same object's atomic ops with `SeqCst` (PORTING.md §6's mixed-use clause).
3. `tmo_in_millisec` is initialised to 0 at C 117 (C leaves it uninitialised; `&raw mut` needs a
   live value) - PORTING.md 1.5.
4. `fds_lim` is `mut` (C 183 and 204 assign it).
5. `FD_ISSET`'s `!= 0` is dropped in five places (the mirror's function is `bool`).
6. `xnmalloc (n_gfds as isize, size_of::<GPollFD> () as isize)` and `xfree (gfds as *mut c_void)`
   spell C's implicit conversions; macro spelling `countof!`/`GNUC_PREREQ!`.
7. `<glib.h>`'s names come from the GTK mirror's GLib section (P-90 ②), and the mirror's own four
   declarations moved into `pub mod xgselect_decls` + glob (P-39, PORTING.md 0.1-10).

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. `docs/baseline/port-lines-pending.tsv`: the `src/xgselect.c` row is dropped and
   `docs/ports/line-drift-tolerance.tsv`'s row is rewritten from the empty-unit reading to the
   landed one.  **DONE this round.**
2. `include/gtk.rs`'s four gdkx/gtkx entries: `use_gtk` is the minimal gate C's reachability asks
   for, but they still *name* X11's `Window`/`Display` while the types are declared only under
   `have_x_windows` (`include/xlib.rs` 847/880).  A `use_gtk`-without-`have_x_windows` graph (the
   PGTK shape) would still be red there - the same X11-name class `src/gtkutil.rs`'s and
   `src/pgtkterm.rs`'s flips already report.  Two candidate fixes, both one edit: add
   `have_x_windows` to those four gates, or spell the types the P-55 way (`Window` -> `c_ulong`,
   `Display` -> `*mut c_void`, the `VisualID` precedent 30 lines above in the same file) with a
   call-site cast in `src/gtkutil.rs`.  **NOT DONE.**
3. `docs/ports/include-gtk.md`'s `## Owed` should carry item 2; that file belonged to the P-90 lane
   in flight when this landed, so the line is left to whoever closes that lane.
4. The six warnings the arm flip emits are `include/lisp.rs`'s (`///` before an `extern "C"` block);
   mirror-side and pre-existing, named rather than silenced.

## Not claimed

- **Zero run evidence.**  The rlib is a compile reading: nothing was linked, started or exercised;
  the GLib lock, the timeout reduction and the descriptor forwarding have no observed behaviour.
- **Not a GLib build.**  The flip compiles against the port's own mirror.  `include/gtk.rs`'s GLib
  part quotes a real GLib 2.90.0 header on this host, but its GDK/GTK part is header-only
  placeholders, and every GDK/GTK value and size stays **UNVERIFIED**.
- The frozen `build-tu` object is the 336-byte empty one, so it says nothing about the body; the
  "0 errors" above is a *name-and-type* reading of the mirror set, which is weaker than a build
  against the real SDK (P-86 ⑦).
- No differential or behavioural evidence exists for this unit, and no C reference object exists to
  differ against on this target (the file is dead in the frozen configuration).
