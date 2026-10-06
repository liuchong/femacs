# emacs/src/gfilenotify.c (336 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 4

## Functions defined — 3

## Macros defined — 1

## Types and their fields — 0

## File-scope objects — 1

## Decisions

- **The file gate is the build-level `feature = "have_gfilenotify"` (P-86 ⑧)**, on every item
  *and on the include graph*: the frozen `configure` leaves `HAVE_GFILENOTIFY` undefined
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1376), so the reference builds no object from
  this file either and this target's `build-tu` object is the 336-byte empty one - while the text
  is carried **in full**, because a GLib configuration compiles it.  The earlier "empty unit plus
  a deferred-declaration note" reading (status 3.31) is withdrawn as a *conclusion*; its
  measurements (the empty object, the empty checklist) still hold.
- **GLib, GObject and GIO are on this machine** (`/opt/homebrew/include/glib-2.0`, GLib 2.90.0),
  so `<gio/gio.h>`'s declarations are quoted from the header and their values are clang
  measurements (see `## Evidence`), not recollections.  The GLib/GObject half (`GError`,
  `g_object_unref`, `g_signal_connect_data`, `g_free`, `GType`, `GCallback`, `G_CALLBACK!`) lives
  in `include/gtk.rs` (P-90 ②: one surface, one mirror); `<gio/gio.h>` has no mirror and this is
  its only consumer, so its declarations stand at C 21's position (P-88 ①) - and if a second
  consumer appears they move into a mirror of their own in the same commit (P-88 ⑤).
- **`G_TYPE_FROM_INSTANCE!`/`G_OBJECT_TYPE_NAME!` are the *real* chains, not approximations**:
  the two GObject prologue records are measured (`GTypeClass` 8 bytes with `g_type` at 0,
  `GTypeInstance` 8 bytes with `g_class` at 0), so `gfile-monitor-name`'s
  `intern (G_OBJECT_TYPE_NAME (monitor))` reads `((GTypeInstance *) monitor)->g_class->g_type`
  through `g_type_name` - unlike the `src/xfns.rs` `G_OBJECT_GET_CLASS!` precedent, which records
  its cast chain as owed.
- **The four includes C 22-25 reach through other headers are spelled here** (P-86 ⑧):
  `include/systime.rs` (termhooks.h's `Time`), `include/dispextern.rs` (`Emacs_Color`),
  `include/character.rs` (lisp.h's `string_char_and_length`/`raw_prev_char_len`) and
  `lib/include/string.rs` (lisp.h 30's `<string.h>`: `strcmp`/`strcpy`).
- **Names no mirror carries**: `FILE_NOTIFY_EVENT` (termhooks.h 291, inside `#ifdef
  USE_FILE_NOTIFY` - this file's own gate implies it; the frozen `include/termhooks.rs` carries
  the *frozen* configuration's members, so this one is declared in-unit and its value (28) is
  owed to the mirror round, `## Owed` 1) and `FUNCTIONP` (lisp.h's inline predicate over
  `Ffunctionp`, spelled here as lisp.h spells it).  `g_signal_connect` is **not** in that list:
  it is GLib's macro and `include/gtk.rs` carries it (`## Findings` 1).

## C line ranges

The rows are contiguous; `docs/port-map.md` binds this ledger to `src/gfilenotify.c` alone (the
reference has no `src/gfilenotify.h`), so the table covers C 1-336.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19 | `#include <config.h>` | ungated |
| 21-25 | `<gio/gio.h>`, `lisp.h`, `coding.h`, `termhooks.h`, `keyboard.h` | file gate; the GIO surface is the in-unit section (P-88 ①), the four chain edges are spelled (see `## Decisions`) |
| 26-29 | the watch list (`static Lisp_Object watch_list;`) | file gate; C's tentative definition is `Qnil` |
| 31-123 | `dir_monitor_callback` (C 46-74 the `switch`, C 73's `goto cleanup`, C 119-120 the frees) | file gate; `match` + one labelled block (PORTING.md 2.2) |
| 125-227 | `DEFUN ("gfile-add-watch", …)` (C 171-218 the flags, the monitor and the rate limit, C 219 the `GCallback` cast) | file gate |
| 229-255 | `DEFUN ("gfile-rm-watch", …)` | file gate |
| 257-276 | `DEFUN ("gfile-valid-p", …)` | file gate |
| 278-295 | `DEFUN ("gfile-monitor-name", …)` (C 293 `G_OBJECT_TYPE_NAME`) | file gate |
| 298-305 | `globals_of_gfilenotify` (C 301-303's `#if ! GLIB_CHECK_VERSION (2, 36, 0)`) | file gate + `glib_check_version_lt_2_36` (both arms carried, P-51) |
| 307-336 | `syms_of_gfilenotify` (twelve `DEFSYM`s, `staticpro`, `Fprovide`) | file gate |

## Findings

1. **The reference has no `g_signal_connect` symbol, and the port's first edition declared one.**
   C 219's `g_signal_connect (monitor, "changed", …)` is GLib's *macro* (gsignal.h 554-556:
   `#define g_signal_connect(instance, detailed_signal, c_handler, data) g_signal_connect_data
   (…, NULL, (GConnectFlags) 0)`), so the reference program calls `g_signal_connect_data` and
   nothing else.  `tools/check-mirror-dupes.py` failed the tree over the in-unit
   `unsafe extern "C" { fn g_signal_connect … }` this ledger's first edition had put at C 219's
   position (a second spelling of what `include/gtk.rs` already carries as the macro); the fix is
   C's own spelling - the call goes through the mirror's `g_signal_connect!`, and `nm` on the
   flipped rlib shows `U _g_signal_connect_data` and no `_g_signal_connect` (status 3.486).

## Evidence

- frozen `build-tu src/gfilenotify.c` -> exit 0, `build/femacs/gfilenotify.o` **336 bytes** (the
  empty object the file gate leaves);
- **arm flip (a real compile, GLib is on this machine)**:
  `probe src/gfilenotify.rs --crate-type=lib -C panic=abort --cfg 'feature="have_gfilenotify"'`
  -> **exit 0, 0 errors, 19 warnings, a 13,070,856-byte rlib**.  The warnings are the house
  classes (`unnecessary parentheses` around the C's parenthesised conditions, `unused doc comment`
  on the `extern` blocks); none is in the unit's own statements' semantics.
- **re-run after the `g_signal_connect` fix (status 3.486)**: the same flip -> **exit 0, 0 errors,
  18 warnings, a 13,069,752-byte rlib**; `nm` on that rlib is `U _g_signal_connect_data` with no
  `_g_signal_connect` at all - the emitted reference is C's own macro expansion;
  `python3 tools/check-mirror-dupes.py` -> **17 duplicate declaration(s), 17 ratchet row(s),
  0 failure(s)**.
- **clang measurements** (`/opt/homebrew/include/glib-2.0`, GIO 2.90.0):
  `G_FILE_MONITOR_EVENT_*` = 0..7 in declaration order (CHANGED 0, CHANGES_DONE_HINT 1,
  DELETED 2, CREATED 3, ATTRIBUTE_CHANGED 4, PRE_UNMOUNT 5, UNMOUNTED 6, MOVED 7);
  `G_FILE_MONITOR_NONE/WATCH_MOUNTS/SEND_MOVED` = 0/1/2; `sizeof (GTypeInstance)` 8 with
  `g_class`@0; `sizeof (GTypeClass)` 8 with `g_type`@0.
- `extract-abi --check emacs/src/gfilenotify.c femacs/src/gfilenotify.rs` -> **OK, zero `-`
  rows** (the four DEFUNs' Lisp names, argument counts, interactive specs and **docstrings** are
  compared byte-for-byte; `dir_monitor_callback` and `Fgfile_add_watch`'s token rows are recorded
  in `docs/ports/macro-body-notes.tsv`);
- `check-cond-eval src/gfilenotify.c` -> every condition decided by the frozen table;
  `check-declarations` / `check-citations` / `check-dead-file-coverage` -> 0 failures, 0 drift;
  `tools/check-all-units.sh` **83/0 + 83/0**.

## Deviations

1. **`GFileMonitorFlags gflags` is `gflags_`** (C 171): the name collides with lisp.h's
   `struct gflags` at crate-root scope, which Rust forbids (PORTING.md 0.2's renaming rule); the
   trailing `_` is the whole difference.
2. **C's `event.kind = FILE_NOTIFY_EVENT;` is a bitfield write** (termhooks.h 373-412):
   `include/termhooks.rs` packs `kind : 16` into `input_event.bits`, so the port writes it with
   the per-unit `input_event_bits!` macro (the `src/gtkutil.rs` 1757-1770 shape, body-identical).
3. **`g_free`/`g_object_unref` take `gpointer`**, so the C's implicit conversions are casts here
   (`name as gpointer`, `monitor as *mut c_void` for `make_pointer_integer*`), and C's
   `!gboolean` tests are `== 0` (PORTING.md 1.1).
4. **`XOBJECT`-style pointer equality**: C 208's `XFIXNUMPTR (watch_descriptor) == monitor` is
   `XFIXNUMPTR (…) as *mut GFileMonitor == monitor` (the mirror returns `*mut c_char`).

## Owed by the integrator (nothing here is claimed as done)

1. **`include/termhooks.rs`: `FILE_NOTIFY_EVENT`'s value per configuration.**  The frozen mirror
   carries the frozen configuration's members; in a `USE_FILE_NOTIFY` build the member sits before
   `SLEEP_EVENT` and shifts the members after it by one.  The declaration is in-unit for now
   (value 28 for this configuration).
2. **The `<gio/gio.h>` surface**: if a second consumer appears, move the in-unit section into
   `include/gio.rs` in the same commit (P-88 ⑤).

## Not claimed

- **Zero run evidence**: the 13 MB rlib is a compile artifact; no file event was ever watched, no
  `GFileMonitor` created, no `g_file_monitor` call executed, no callback fired.
- The `glib_check_version_lt_2_36` arm **has never been compiled** (the frozen configuration takes
  the empty arm), and neither has the `#ifdef USE_FILE_NOTIFY`-derived `FILE_NOTIFY_EVENT` value.
- The frozen `build-tu` object is the 336-byte empty one, i.e. it says nothing about the body.

- **The GObject records, macros and `g_type_name` are `include/gtk.rs`'s now (status 3.726).**  They
  stood here under `have_gfilenotify`; this unit's include graph reaches that mirror (its line 27),
  so when the pgtkfns twelfth slice needed the same chain there, the copies here became duplicates
  (P-26's one spelling per construct) and are dropped with the C 330 call site unchanged.
  Measured: this unit's own probe (`have_gfilenotify`) reads **exit 0, 0 errors, 0 warnings** on
  both sides of the drop, and `check-mirror-dupes.py` stays 16/16/0.
