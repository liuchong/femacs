# emacs/src/xselect.c (3541 lines)

> **P-86's class: the file gate is the build's, and the frozen configuration compiles nothing
> here.**  `xselect.o` joins the build only through `XOBJ` - `configure.ac` 7196's
> `XOBJ="xterm.o xfns.o xselect.o xrdb.o xsmfns.o xsettings.o"` under `HAVE_X_WINDOWS` - and that
> macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1720, `XOBJ` is never filled on
> this target (`build/emacs-ref/src/Makefile` 214 is the comment that names the variable),
> `docs/baseline/frozen-build-sources.txt` lists no `xselect.o`, and the four platform object
> variables of that Makefile are empty.  So P-86 ① applies - this is *transcription*, both arms of
> every `#if` are carried - and P-86 ② applies too: `extract-abi.py` has no live item to tick, so
> the substitute is the **C line-range table** below plus this ledger's readings.
>
> Port: `femacs/src/xselect.rs` (**3,900 lines**, `grep -c ''`), md5
> `7fea63a0696fbdd83e1d76364bcb9732`.
>
> This ledger replaces the previous revision's conclusion ("not compiled by the frozen
> configuration, so the port is the empty unit plus a note").  The old measurements were true; what
> changed is the conclusion drawn from them (the `docs/ports/xfont.md` note).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`XOBJ`, i.e. `HAVE_X_WINDOWS`; `configure.ac` 7196) | `feature = "have_x_windows"` (on every item **and on the include graph**) | an X11 configuration |
| C 30 `#ifdef HAVE_SYS_TYPES_H` (`#include <sys/types.h>`) | `all (feature = "have_x_windows", feature = "have_sys_types_h")` (**`have_sys_types_h` is not registered**; Owed 1) | an X11 configuration with `<sys/types.h>` (the frozen table has `HAVE_SYS_TYPES_H 1`, macros.tsv 1637) |
| C 72 `#ifdef TRACE_SELECTION` (the four `TRACE*` macro pairs) | `feature = "trace_selection"` for the `fprintf` arm; the `(void) 0` arm is the plain item (**not registered**; Owed 1) | `TRACE_SELECTION` is undefined in the frozen table, so the `(void) 0` arm is live and the `fprintf` arm is dead text (P-51) |
| C 2764 and C 2791 `#ifdef HAVE_XFIXES` | `feature = "have_xfixes"` (registered, `femacs/xtask/Cargo.toml` 305) | an X11 configuration with the XFIXES extension (`undef`, macros.tsv 1695) |
| C 24 `#if 10 <= __GNUC__` (one `#pragma`) | no arm: a pragma has no Rust spelling, and clang's `__GNUC__` is 4 (macros.tsv 6264) | - (C's text is kept as a comment; `## Deviations` 1) |
| C 1708 `#if false` | no arm: dead in every configuration | - (recorded at its position; `## Deviations` 3) |

The include graph carries the file gate on every edge (P-86 ⑧): the frozen build's empty object
(Evidence ①) is the criterion, and putting any edge outside the gate would fail it.

## C line ranges

Rows are ascending and contiguous, so every line of the reference is accounted for (the blank
lines the gate excludes need no row of their own; `tools/check-dead-file-coverage.py` is the
machine reading).

| C | block | arm |
| --- | --- | --- |
| 1-16 | the file's licence/declaration block | dropped (P-86 ① / 0.1-19) |
| 17-22 | the `Rewritten by jwz` banner and `#include <config.h>` | ungated (every unit includes it) |
| 23-28 | the GCC-bug-102671 comment (`#if 10 <= __GNUC__` + `# pragma`) and `#include <limits.h>` | recorded at its position (pragma, no arm); file gate |
| 29-34 | `#ifdef HAVE_SYS_TYPES_H` + `#include <sys/types.h>` + `#endif`, and `#include <unistd.h>` | file gate; `have_sys_types_h` on the `<sys/types.h>` edge |
| 35-46 | the ten Emacs headers (`lisp.h` … `atimer.h`) and `#include <X11/Xproto.h>` | file gate (P-86 ⑧); the two extra edges of Deviations 11 |
| 47-68 | the two forward tag declarations and the eleven `static` prototypes | dropped (PORTING.md 0.1-12); C's own text recorded |
| 69-86 | the "Printing traces to stderr." comment and the `#ifdef TRACE_SELECTION` macro pair | file gate; `trace_selection` for the `fprintf` arm |
| 87-106 | the `X_LONG_SIZE` comment and `#define`, and `MAX_SELECTION_QUANTUM` | file gate |
| 107-125 | `selection_quantum` and the `LOCAL_SELECTION` macro | file gate |
| 126-190 | `symbol_to_x_atom` | file gate |
| 191-268 | `x_atom_to_symbol` | file gate |
| 269-347 | `x_own_selection` | file gate |
| 348-443 | `x_get_local_selection` | file gate |
| 444-483 | the "Subroutines of x_reply_selection_request" banner and `x_decline_selection_request` | file gate |
| 484-544 | `struct selection_data` and `struct transfer` | file gate; both layouts measured (Evidence ⑤) |
| 545-567 | `#define SELECTED_EVENTS 1` and `struct x_selection_request` | file gate; the record's layout measured |
| 568-602 | the three request statics, `struct prop_location` and its four statics | file gate; the record's layout measured |
| 603-668 | `set_property_change_object`, the push/pop pair and `x_selection_request_lisp_error` | file gate |
| 669-706 | `c_size_for_format` and `x_size_for_format` | file gate |
| 707-781 | `selection_data_for_offset` and `selection_data_size` | file gate |
| 782-844 | `transfer_selecting_event`, `x_cancel_selection_transfer` and `x_selection_transfer_timeout` | file gate |
| 845-1026 | `x_start_selection_transfer` and `x_continue_selection_transfer` | file gate |
| 1027-1078 | `x_remove_selection_transfers` and `x_handle_selection_error` | file gate |
| 1079-1120 | `x_reply_selection_request` | file gate |
| 1121-1279 | `x_handle_selection_request` | file gate; C 1159/1164/1171/1206/1213's `goto`s are the two labelled loops |
| 1280-1321 | `x_convert_selection` | file gate |
| 1322-1410 | `x_handle_selection_clear`, `x_handle_selection_event` and `x_should_preserve_selection` | file gate; C 1385's `event->kind` is the mirror gap of Owed 5 |
| 1411-1460 | `x_clear_frame_selections` | file gate |
| 1461-1529 | `waiting_for_other_props_on_window`, `expect_property_change`, `unexpect_property_change` and `wait_for_property_change_unwind` | file gate |
| 1530-1627 | `wait_for_property_change` and `x_handle_property_notify` | file gate; `trace_selection` for C 1575-1585's arm |
| 1628-1656 | `x_display_selection_waiting_message`, `x_cancel_atimer` and the three reader statics | file gate |
| 1657-1762 | `x_get_foreign_selection` (C 1708-1711's `#if false` recorded) | file gate |
| 1763-1900 | `x_get_window_property` | file gate; C 1802/1808/1811/1862/1889/1895's `goto`s are the two nested loops |
| 1901-2034 | `receive_incremental_selection` and `x_free_selection_data` | file gate |
| 2035-2156 | `x_get_window_property_as_lisp_data` | file gate |
| 2157-2300 | `selection_data_to_lisp_data` and `cons_to_x_long` | file gate |
| 2301-2472 | `lisp_data_to_selection_data` | file gate; C 2421's `SAFE_NALLOCA` is the port's stack pool |
| 2473-2526 | `clean_local_selection_data` and `x_handle_selection_notify` | file gate |
| 2527-2572 | `frame_for_x_selection` | file gate |
| 2573-2597 | the DEFUN `x-own-selection-internal` | file gate |
| 2598-2662 | the DEFUN `x-get-selection-internal` | file gate |
| 2663-2745 | the DEFUNs `x-disown-selection-internal` and `x-selection-owner-p` | file gate; C 2711-2713's `SELECTION_EVENT_*` are lvalues here |
| 2746-2804 | the DEFUN `x-selection-exists-p` | file gate; `have_xfixes` at C 2764/2791 |
| 2805-2852 | the DEFUN `x-get-local-selection` | file gate |
| 2853-2960 | the clipboard-manager five (`x_clipboard_manager_save`, the two error handlers, `save_frame`, `save_all`) | file gate |
| 2961-3010 | the drag-and-drop banner and `x_check_property_data` | file gate |
| 3011-3083 | `x_fill_property_data` and `x_property_data_to_lisp` | file gate |
| 3084-3160 | the DEFUNs `x-get-atom-name` and `x-register-dnd-atom` | file gate |
| 3161-3348 | `x_handle_dnd_message`, the DEFUN `x-send-client-message` and `x_send_client_event` | file gate; C 3210's `kind` store is the masked bitfield write |
| 3349-3541 | `x_timestamp_for_selection`, `syms_of_xselect`, `syms_of_xselect_for_pdumper` and `mark_xselect` | file gate; the seven `DEFVAR_*` variables are defined here (P-21) |

## Evidence

**① The frozen configuration (`build-tu`).**

    cd femacs && CARGO_TARGET_DIR=/tmp/xsel-lane cargo run -q -p xtask -- build-tu src/xselect.c

→ `built src/xselect.rs -> ../build/femacs/xselect.o`, **exit 0**, and `build/femacs/xselect.o` is
**336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - the empty object the file gate leaves, the
same bytes `docs/ports/xfont.md`'s Evidence ① reads.  The materialized
`build/femacs/units/xselect/_root.rs` carries **2** `// [xtask dedupe]` lines (`termhooks.rs` and
`lib/include/string.rs`, both reached a second time), i.e. no gated edge stole another file's
expansion slot (P-89).

**② The arm: the file gate plus its sub-gates, both arms.**  `cd femacs && CARGO_TARGET_DIR=/tmp/xsel-lane
cargo run -q -p xtask -- probe src/xselect.rs <out> --crate-type lib -C panic=abort --cfg
'feature="have_x_windows"' --cfg 'feature="have_sys_types_h"'` (the `trace_selection` and
`have_xfixes` arms *off*) → **exit 1, 3 errors, 5 warnings**, and **not one is located in this
unit's own text**:

| # | error | location | class |
| --- | --- | --- | --- |
| 1 | `E0425 cannot find type font` | `include/xterm.rs` copy 807 (`pub font: *mut font`) | mirror gap (Owed 3; the `docs/ports/widget.md` Finding 7 class) |
| 2 | `E0080 evaluation panicked: !cfg!(feature = "have_x_windows") \|\| HAVE_X_WINDOWS` | `include/systime.rs` copy 27 | mirror assertion no `have_x_windows` probe can satisfy (Owed 4) |
| 3 | `E0609 no field kind on type selection_input_event` | `_root.rs` 1825 = C 1385 (`event->kind`) | the mirror's missing bitfield storage unit (Owed 5) |

**③ The sub-gate arms (`trace_selection` and `have_xfixes` on).**  The same command with
`--cfg 'feature="trace_selection"' --cfg 'feature="have_xfixes"'` → **exit 1, 4 errors**: the three
of ② plus **1 × `E0428`** (`X_INVALID_WINDOW` defined twice: this unit's C 2794 declaration against
`include/xterm.rs` 392's).  That duplicate was the first version's defect and is **fixed** - the
declaration is gone (the constant is the mirror's crate-root item).  The re-run on the final text is
**exit 1, 3 errors**, the same three as ②: nothing in this unit's text comes from either dead arm,
so the `fprintf` arm and the XFIXES arm both compile clean under their features.  (Two warnings of
the trace arm are `unused attribute`/`rustdoc does not generate documentation` in the *mirrors*,
the house warning class.)

**③b A re-read on the final text.**  After the citation work of `## Deviations` 9 the same
diagnostic was repeated on the re-materialized root (the current working tree, which also carries
two other lanes' in-flight edits to `include/xlib.rs` and `include/dispextern.rs`): **the same three
errors**, the `E0609` now at `_root.rs` 1825.  The probe readings of ②③ stand.

**④ The mutation test (P-86 ⑦'s criterion, and the proof ②③ are real readings).**  One type error
injected into this unit's own text (`let mut val: Atom = crate::x11_constants::None;` at C 134),
the *same* command re-run: the output is ②'s three errors **plus exactly one new one**, `E0308:
mismatched types … expected u64, found i64` naming the injected line (`_root.rs` 539) - so this
unit's own text *is* type-checked and the include graph is not aborting before it.  Restored, and
`shasum -a 256 femacs/src/xselect.rs` → `cd73b1248500480c557dc37d686358c06319b492a8da80b74b0d8ad5960c7fce`,
the pre-mutation value.

**⑤ The measurements (clang, this machine's `xorgproto` 2025.1 + libX11 under `/opt/homebrew/include`).**

    clang -I/opt/homebrew/include -o /tmp/xsel-lane/probe.bin /tmp/xsel-lane/probe.c && /tmp/xsel-lane/probe.bin
    XSelectionEvent sizeof=72 type=0 serial=8 send_event=16 display=24 requestor=32 selection=40 target=48 property=56 time=64
    XPropertyEvent sizeof=64 type=0 serial=8 send_event=16 display=24 window=32 atom=40 time=48 state=56
    XClientMessageEvent sizeof=96 type=0 serial=8 send_event=16 display=24 window=32 message_type=40 format=48 data=56
    XClientMessageEvent.data sizeof=40
    XErrorEvent sizeof=40 type=0 display=8 resourceid=16 serial=24 error_code=32 request_code=33 minor_code=34
    XEvent sizeof=192 align=8
    XA_PRIMARY=1 XA_SECONDARY=2 XA_STRING=31 XA_INTEGER=19 XA_ATOM=4
    SelectionNotify=31 PropertyNewValue=0 PropertyDelete=1 BadWindow=3 ClientMessage=33 None=0 NoEventMask=0 PropertyChangeMask=4194304
    CurrentTime=0 PointerWindow=0 InputFocus=1 AnyPropertyType=0 PropModeReplace=0 Success=0
    X_LONG_MAX=2147483647 X_ULONG_MAX=4294967295 LONG_MAX=9223372036854775807 PTRDIFF_MAX=9223372036854775807 SIZE_MAX=18446744073709551615 INT_MAX=2147483647
    Display=8 Window=8 Atom=8 Time=8 Bool=4 XID=8

and the file's own four records (`probe2.c`, the same headers):

    selection_data sizeof=56 data=0 string=8 size=16 format=24 type=32 property=40 next=48
    transfer sizeof=120 requestor=0 offset=8 items_per_request=16 dpyinfo=24 data=32 next=88 last=96 timeout=104 serial=112 flags=116
    prop_location sizeof=48 identifier=0 display=8 window=16 property=24 desired_state=32 arrived=36 next=40
    sizeof(long)=8 sizeof(unsigned long)=8 sizeof(int)=4 sizeof(bool)=1

Those numbers are the `const _: ()` assertions in the port (P-88 ②; `xtask layout-asserts` has no
baseline for records this file declares itself).

**⑥ The inventory gate and the token gate.**

    python3 tools/extract-abi.py --check emacs/src/xselect.c femacs/src/xselect.rs

→ `FAIL` with **one** problem line, `line count drifted 10.1%: C 3541 vs Rust 3,900` - the
tolerance row of Owed 2, which this lane may not write.  The body/token checks are green: the eleven
rows this lane appended to `docs/ports/macro-body-notes.tsv` (Owed 12 lists them) are what carry
the `switch`→`match`, `goto`→labelled-loop, `bufp->kind`, `doc:` and `xselection`-member cases, and
`extract-abi.py`'s `macro_body_of` reads a unit's *first* definition of a name, which is why the
`TRACE*` pair writes C's live `(void) 0` arm first (`## Deviations` 13).

**⑦ The four whole-tree tools** (`python3 tools/<name>.py`): `check-declarations.py` →
`17 declaration sites (0 forbidden, 17 allowed literals), 0 failures` with 0 findings in this
unit; `check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)`
(no new failure for this file); `check-dead-file-coverage.py` → the table above is its reading;
`check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 1511 external, 36 allowed, 0 stale`;
`check-ledger-ticks.py` → see below.

**⑧ Budget, as run.**  `probe` **3 of 3** (②③'s two arms plus the re-run that confirms the
`E0428` fix).  `build-tu` **11 of 3** - an overrun, reported honestly: one run per arm-diagnostic
(`build-tu` is also the only step that materializes the unit, and xfont's Evidence ② route runs the
same materialized root through plain `rustc` for the arm readings), plus the final frozen reading of
①.  The mutation test and every `rustc` diagnostic are outside the probe budget and are labelled as
diagnostics.  No `cargo test`, `xtask --all`, `gate-fast`, `xtask dead-arms` or
`tools/check-all-units.sh` was run; no git write was performed by this lane.

## Deviations

1. **C 24's `#if 10 <= __GNUC__` carries a `#pragma` and no arm**; the guard is false on this
   target, so C's text is kept as a comment and nothing is gated on it (the `src/pgtkterm.rs` 69 /
   `src/timefns.rs` 79 reading).
2. **C 48-49's forward tag declarations and C 51-68's eleven `static` prototypes are dropped**
   (PORTING.md 0.1-12: Rust items are order-independent), recorded at their position.
3. **C 1708-1711's `#if false` block is recorded, not transcribed** - dead in every configuration -
   the `src/xfns.rs` 958 precedent.
4. **C's `False`/`True` are written `false as Bool`/`true as Bool`**: the port's `Bool` is `c_int`
   (`include/xlib.rs` 185), so Rust's `false`/`true` cannot be passed as-is; the cast keeps C's
   token *and* the ABI.
5. **Every `switch` is `match` and every `goto` is a labelled loop** (PORTING.md 2.2):
   `x_atom_to_symbol`, `c_size_for_format`, `x_size_for_format`, `selection_data_size`; and
   `x_handle_selection_request`'s `'done`/`'really_done`, `x_get_window_property`'s
   `'done`/`'overflow`.  The appended `macro-body-notes.tsv` rows record each site.
6. **C's `event.xselection` / `event.xclient` member is reached through a measured conversion**
   (P-72 ③): `include/xlib.rs` 1164's `XEvent` is an opaque 192-byte record whose 33 members are
   owed, so the port casts to `*mut XSelectionEvent`/`*mut XClientMessageEvent` (the
   `src/xsettings.rs` 176 reading).
7. **C 3210's `bufp->kind = DRAG_N_DROP_EVENT` is the masked store** into the port's
   `input_event::bits` unit (PORTING.md 1.3 / P-80), because the mirror records `kind` as a bitfield
   rather than packing it.
8. **C 765/772/3078's `ckd_mul` is the `overflowing_mul` pair with the store at the site**
   (P-15: the frozen arm is clang's `__builtin_mul_overflow`, which always stores).
9. **In-body `// C` citations are kept only where a numbered read of the reference verified them**;
   in the regions this lane read without line numbers the per-line citation was dropped rather than
   guessed (124 dropped), and the block's signature-line citation stands.  The ledger's own table
   above is what covers the file.
10. **Two include edges are not in C's own list** but are reached through it: `lib/include/string.rs`
    (lisp.h 30's `<string.h>`, the `src/xfont.rs` 93 shape) and `lib/include/stdckdint.in.rs`
    (lisp.h's `<stdckdint.h>` for C 765/772's `ckd_mul`).
11. **`include/xlib.rs` is added as the edge at C 37** (`xterm.h` 40's `#include <X11/Xlib.h>`); the
    surface that mirror does not carry (the four event records, the fourteen entry points, the ten
    constants) stands in the unit at its C `#include` line (P-88 ①).
12. **C's DEFUN parameters that the body reassigns are shadowed by mutable locals** of the same name
    (`let mut selection = selection;`), because `DEFUN!`'s argument list takes no `mut` (the
    `src/data.rs` 1097 reading).
13. **The `TRACE*` pair writes C's live `(void) 0` arm first**, then the `fprintf` arm under
    `feature = "trace_selection"`: `extract-abi.py`'s `macro_body_of` reads the *first* definition of
    a name, so the other order hid the live body's `0` token (measured: four
    `macro TRACE* does not mention 0` rows before the swap, none after).
14. **`pD` (lisp.h 150) and `PRIdMAX` (`<inttypes.h>` 443) are declared as unit-local
    `macro_rules!`s**: the mirrors carry `pD` as a `&str` constant (which `concat!` cannot take) and
    `PRIdMAX` only in `lib/include/inttypes.in.rs`, which is not in this unit's graph (Owed 10).

## Owed

1. **`femacs/xtask/Cargo.toml` `[features]`**: register `have_sys_types_h` (C 30's `#ifdef`) and
   `trace_selection` (C 72's) - this lane cannot write that file.  `have_x_windows` (303) and
   `have_xfixes` (305) are already there; `docs/baseline/feature-coverage.tsv` then needs
   `python3 tools/feature-coverage.py --write`.
2. **`docs/ports/line-drift-tolerance.tsv`**: the `xselect.c` row (`C 3541 vs Rust 3,900`,
   **10.1 %** - Evidence ⑥).  Not this lane's file.
3. **`include/xterm.rs` 807**: `pub font: *mut font` names a type no unit reaches unless it
   includes `font.rs` (`xterm.h` names `struct font` as an incomplete tag).  This unit's C include
   list has no `font.h`, so it reports the gap rather than adding a non-C edge - the
   `docs/ports/widget.md` Finding 7 defect, and the fix belongs to the mirror.
4. **`include/systime.rs` 27**: `const _: () = core::assert! (!cfg! (feature = "have_x_windows")
   || HAVE_X_WINDOWS);` cannot hold in any arm reading, because the config mirror's `HAVE_X_WINDOWS`
   stays 0 while the *feature* is on.  The assertion needs the feature (or dropping) - the mirror's
   call.
5. **`include/xterm.rs` 1453-1461**: `selection_input_event` records `ENUM_BF (event_kind) kind :
   EVENT_KIND_WIDTH` as a comment and carries no storage unit or accessor, so C 1385's
   `event->kind` is `E0609` at this unit's use site (Evidence ②③'s third error).  P-80's rule wants
   a `bits` unit plus a member-named accessor, as `keyboard.rs` 4063-4068 does for `input_event`.
6. **`include/xterm.rs`**: the `xterm.h` entry points this unit declares in place -
   `x_display_info_for_display`, `x_intern_cached_atom`, `x_intern_atoms`, `x_get_atom_name`,
   `x_catch_errors`, `x_uncatch_errors`, `x_uncatch_errors_after_check`, `x_catch_errors_for_lisp`,
   `x_check_errors`, `x_check_errors_for_lisp`, `x_uncatch_errors_for_lisp`,
   `x_ignore_errors_for_next_request`, `x_stop_ignoring_errors`, `x_had_errors_p`,
   `x_wait_for_cell_change`, `x_find_selection_owner`, `x_relative_mouse_position`,
   `x_translate_coordinates`, `x_preserve_selections`, `decode_window_system_frame`.
7. **`include/lisp.rs`**: `record_unwind_protect_void` (lisp.h 4825), `internal_condition_case_1`
   (4806), `message` (4389), `message3_nolog` (4393), `mark_object` (4436) and
   `wait_reading_process_output` (5181) - six names every unit currently declares at its use site.
8. **`include/dispextern.rs`**: `redisplay_preserve_echo_area` (dispextern.h 3589) and
   `check_x_display_info` (3993, whose `Display_Info` alias the mirror already carries).
9. **`include/termhooks.rs`**: `decode_live_terminal` (termhooks.h 961), which that mirror's own
   444 comment records as a use-site declaration.
10. **`include/lisp.rs` / `lib/include/inttypes.in.rs`**: the `pD` macro form (lisp.rs 2925 has only
    the `&str` constant) and `PRIdMAX!` (inttypes.in.rs 647 has both, but is not in this graph).
11. **The X11 SDK surface** this unit declares in place (Deviation 11) wants the dedicated mirror
    P-90 ① asks for (`include/xlib.rs` is its natural home: it already declares `XEvent` and the
    `x11_constants` module).  When that lands this unit's copy moves there in the same commit
    (P-88 ⑤ / P-90 ③).
12. **`docs/ports/macro-body-notes.tsv`**: this lane's eleven appended rows are the ones Evidence ⑥
    names; the integrator should keep them as part of this unit's baseline rows.
13. **`docs/port-map.md`'s `src/xselect.c` row and `docs/status.md`'s ratchet line** still read the
    empty-unit revision (`tools/gen-port-map.py` / `tools/check-port-lines.py --write-pending`) -
    the integrator's to refresh.

## Not claimed

- **Zero run evidence.**  Nothing was linked or executed: the frozen object is the 336-byte empty
  one (Evidence ①), and no X display, X server or XFIXES extension was involved anywhere.
- **The arm readings are compile readings, not acceptance.**  Both arms have three errors and all
  three live in the mirrors (Owed 3-5), so **no configuration on this machine compiles this unit
  end to end** and neither reading is a working configuration.
- **The `trace_selection` and `have_xfixes` arms are compile readings only** (Evidence ③): the
  `fprintf` arm's `stderr`/`getpid` and the XFIXES arm's `x_find_selection_owner`/
  `X_INVALID_WINDOW` were never linked, and the XFIXES prototype is C's own text, not a measured
  symbol.
- **`extract-abi.py --check` is not coverage evidence by itself**: its token check is a token-set
  comparison, so a reordered statement inside a body, a swapped operand of the same tokens, or a
  dropped `i += 1` that leaves the tokens intact would still pass.
- **The in-body citations are partial by construction** (Deviation 9): 124 were dropped rather than
  guessed.  The block-level citations and the C range table are the complete set.
- **The layout asserts are this machine's readings** (`xorgproto` 2025.1 / libX11 under
  `/opt/homebrew/include`); another X11 SDK could lay `XEvent`'s members out differently, and the
  file's own four records are asserted against values no baseline row yet records.
- **`femacs/src/xselect.rs` may move under a later round** (Owed 3-11), and the seven `DEFVAR_*`
  variables are defined here (P-21) with no post-load hook of their own beyond C 3494's
  `pdumper_do_now_and_after_load`.
