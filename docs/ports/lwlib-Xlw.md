# emacs/lwlib/lwlib-Xlw.c (271 lines)

## Decisions

`emacs/lwlib/lwlib-Xlw.c` @ `fd7afced403`; port `femacs/lwlib/lwlib-Xlw.rs` (**484 lines**); the gate
is `feature = "use_lucid"` (P-86 clause 8), the build's condition rather than one the C text spells:
`lwlib/Makefile.in` 52 (`LUCID_OBJS = lwlib-Xlw.o xlwmenu.o lwlib-Xaw.o`) and `configure.ac` 7231's
`AC_DEFINE ([USE_LUCID], [1], [Define to 1 if using the Lucid X toolkit.])`.  `USE_LUCID`,
`USE_X_TOOLKIT` and `HAVE_X11` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so the
frozen configuration compiles none of this file and the port's frozen object is the empty one.

- **This file is `text-only`** (P-86 clause 3(2)): there is no X11 header on this machine, so the
  `<X11/*>` surface below is C's spelling, not a measurement.  What the round shows is that the arm
  **type-checks** (`## Evidence`).
- **C 34-58's `#if 0` block** (`x_print_complete_resource_name`) is dead text and is **not
  transcribed** (the `src/xfns.rs` 958 precedent); `## Findings` 1 records what it holds.
- **C's `#ifdef emacs`** (C 140-144) is the build's own `-Demacs`, always defined for this file, so
  its three `XtSetArg` lines are live and carry no gate.
- **The X11 surface stands in the unit** (P-88 clause 1): `lwlib/include/lwlib.rs` names `Widget`,
  `Boolean`, `XEvent` and `Window` and says they are left to the unit that includes it, and no
  mirror carries `<X11/Intrinsic.h>`, `<X11/StringDefs.h>`, `<X11/ObjectP.h>`,
  `<X11/CompositeP.h>` or `<X11/Shell.h>`.  The `XtN*` resource names and `xlwMenuWidgetClass` come
  from `lwlib/include/xlwmenu.rs` (they are `&CStr` constants there), and the records come from
  `lwlib/include/{lwlib,lwlib-int,lwlib-widget,xlwmenu,xlwmenuP}.rs`.

## C line ranges

The rows are contiguous and ascending; `docs/port-map.md` binds this ledger to `src/haiku.c`-style
single references - `lwlib/lwlib-Xlw.c` alone (its header is bound to
`docs/ports/include-lwlib-Xlw.md`).

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's licence/declaration block | dropped (P-86 clause 1) |
| 21-33 | `config.h`, `setjmp.h`, `lisp.h`, `lwlib-Xlw.h`, the five `<X11/*>` headers, `xlwmenu.h` | file gate |
| 34-59 | `#if 0` … `#endif /* 0 */`: `x_print_complete_resource_name` | **dead**, not transcribed (recorded here) |
| 60-77 | the menu-callback comment and `highlight_hook` (static) | file gate |
| 78-83 | `enter_hook` (static) | file gate |
| 84-89 | `leave_hook` (static) | file gate |
| 90-106 | `pre_hook` (static) | file gate |
| 107-131 | `pick_hook` (static) | file gate |
| 132-158 | `xlw_create_menubar` (static; C 140-144's `#ifdef emacs` is live) | file gate |
| 159-185 | `xlw_create_popup_menu` (static) | file gate |
| 186-193 | `xlw_creation_table` | file gate |
| 194-206 | `lw_lucid_widget_p` | file gate |
| 207-219 | `xlw_update_one_widget` | file gate |
| 220-227 | `xlw_update_one_value` | file gate |
| 228-232 | `xlw_pop_instance` | file gate |
| 233-265 | `xlw_popup_menu` (C 245-262 the `dummy.xbutton` branch) | file gate |
| 266-271 | `xlw_destroy_instance` | file gate |

## Findings

1. **The `#if 0` block's function** (`x_print_complete_resource_name`, C 41-56) walks a widget's
   parent chain with `XtName`/`XtParent` and prints the resource path to `stderr`; it is inside
   `#if 0` in the reference, so no configuration compiles it and the port's ledger records it
   instead of transcribing it (the same disposition as status 3.431's dead branches).
2. **`lw_lucid_widget_p` answers "is this an `xlwMenu` widget"** by comparing the class and, for a
   shell, its first child's class - and for `overrideShellWidgetClass` it reads
   `((CompositeWidget) widget)->composite.children[0]` without first checking the child count; an
   empty shell would read past the list.  That is the reference's own text, carried as it stands.
3. **`xlw_create_menubar`/`xlw_create_popup_menu` pass `instance->info->val` as the `XtNmenu`
   resource** and hook four callbacks (`XtNopen`, `XtNselect`, `XtNleaveCallback`,
   `XtNenterCallback`; the popup drops `XtNopen`), which is why the five hooks above exist.

## Evidence

- frozen `build-tu ../emacs/lwlib/lwlib-Xlw.c` -> **exit 0**, `build/femacs/lwlib-Xlw.o` **336 bytes**
  (the empty object the file gate leaves).
- **arm flip (status 3.500)**: `probe lwlib/lwlib-Xlw.rs --crate-type=lib -C panic=abort
  --cfg 'feature="use_lucid"'` -> **exit 0, 0 errors, 4 warnings, a 7,748,568-byte rlib**.
- `python3 tools/extract-abi.py --check emacs/lwlib/lwlib-Xlw.c femacs/lwlib/lwlib-Xlw.rs` ->
  **OK, zero `-` rows** (after the `total`/0.80 tolerance row).
- `layout-check` **passed**; `tools/check-all-units.sh` (the mirror round this file needed) and the
  other gates are recorded in `docs/status.md` 3.500.

## Deviations

1. **`XEvent` is a Rust `union`** carrying `type`, the `xbutton` member C writes and a padding
   member that keeps the 192-byte size; the members C does not name are **UNVERIFIED** and owed.
2. **`XButtonPressedEvent` carries the members this file writes**, in C's order, with offsets
   **UNVERIFIED** (no X11 header here to measure against).
3. **`XtSetArg` is a `macro_rules!`** spelling C's `((arg).name = (n), (arg).value = (XtArgVal) (d))`
   with the `&CStr` resource constants the `xlwmenu.rs` mirror carries.
4. **`XtIsShell`** is a C macro (`XtIsSubclass (w, shellWidgetClass)`); the port declares it as an
   entry point and carries `shellWidgetClass` beside it, the `lwlib-utils.rs` mirror's treatment of
   the other `XtIs*` macros.
5. **C's `Boolean` truth tests** are spelled (`(*w).core.being_destroyed == 0`,
   `XtIsShell (widget) == 0`) and `widget_value.enabled` stays a Rust `bool` because that is how the
   `lwlib-widget.rs` mirror records it.

## Owed

1. **The `<X11/*>` surface's real shapes**: with an X11 toolchain, measure `WidgetRec`/`CorePart`/
   `CompositePart`/`XEvent`/`XButtonPressedEvent` and replace the placeholder layouts (P-88 clause 2).
2. **`x_print_complete_resource_name`** (the `#if 0` block) is owed to a future round only if the
   reference ever enables it; today it is dead text.
3. `docs/baseline/feature-coverage.tsv` needs no regeneration (this unit adds no feature name).

## Not claimed

- **Zero run evidence**: nothing was linked or run; no widget was created, no callback fired, no
  menu was popped up.
- **No X11 header was consulted**: every type, constant and layout above is C's spelling, and the
  records' omitted members mean their offsets are knowingly not the real ones.
- The frozen object is the 336-byte empty one, so it says nothing about the body.
