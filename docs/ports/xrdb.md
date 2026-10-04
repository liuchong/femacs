# emacs/src/xrdb.c (513 lines)

`emacs/src/xrdb.c` @ `fd7afced403`; port `femacs/src/xrdb.rs` (533 lines).  The gates are
`build-tu src/xrdb.c` -> `built`, `layout-check` -> `passed`, and
`python3 tools/extract-abi.py --check emacs/src/xrdb.c femacs/src/xrdb.rs` -> `FAIL` on **two**
rows, both `docs/ports/macro-body-notes.tsv` rows this round requests (Decision 8): the file is
the first complete one on this lane that the frozen build does not compile, so the port is
1:1 text under `have_x_windows` and the checklist below is the real, unfiltered one.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 11
- [x] x_get_customization_string  (line 58, static)
- [x] magic_db  (line 102, static)
- [x] search_magic_path  (line 210, static)
- [x] get_system_app  (line 244, static)
- [x] get_fallback  (line 256, static)
- [x] get_user_app  (line 263, static)
- [x] get_user_db  (line 303, static)
- [x] get_environ_db  (line 340, static)
- [x] x_load_resources  (line 378)
- [x] x_get_resource  (line 475, static)
- [x] x_get_string_resource  (line 500)

## Macros defined — 1
- [x] XrmStringType  (line 372, body 4bb71e1c2a36)

## Types and their fields — 0

## File-scope objects — 2
- [x] static char x_customization_string  (line 51, static)
- [x] static char const xdefaults  (line 300, static)

## Decisions

### 1. The whole body carries the X11 arm

`src/xrdb.c` is not in `docs/baseline/frozen-build-sources.txt` (this target's platform object
variables are empty and its headers need Xlib), so nothing in it is compiled here and every item
in `femacs/src/xrdb.rs` carries `#[cfg (feature = "have_x_windows")]` - the `xterm.rs` shape
(P-51), not an inner `#![cfg]` (no unit in the tree uses one).  `tools/check-cond-eval.py
src/xrdb.c` -> `6 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so its `#if`s are all
decidable and Decisions 2-4 name the arm each one takes.

### 2. C 308-312 `#ifdef PBaseSize`: the one frozen reading that cannot be transcribed

```
#ifdef PBaseSize		/* Cheap way to test for X11R4 or later.  */
  xdefs = XResourceManagerString (display);
#else
  xdefs = display->xdefaults;
#endif
```

`PBaseSize` is `<X11/Xutil.h>`'s (measured: this machine's `Xutil.h` 111 is
`#define PBaseSize (1L << 8)`), so every X11R4+ build - every build that compiles this file -
takes the `#ifdef` arm.  The frozen macro table has **no `PBaseSize` row** (measured through
`tools/cinventory.py`'s reader: `<absent>`), and the arm its reading takes (`#else`) dereferences
Xlib's *private* display record (`_XPrivDisplay`, Xlib.h 489-546), which no mirror here carries.
So the port takes the `#ifdef` arm (the arm the file's own platform takes) and records the
`#else` text in place rather than transcribing it (P-08).  This is the only condition in the file
where the two readings differ; the frozen reading decides the other five.

### 3. C 326-334 `#ifdef HAVE_XSCREENRESOURCESTRING`

The macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` (it is a configure result
of a build that has Xlib), so this is the arm the frozen reading leaves out and it carries
`#[cfg (feature = "have_xscreenresourcestring")]`.  The arm's own two names
(`XScreenResourceString`, `DefaultScreenOfDisplay`) come from `include/xlib.rs`; `XFree` does not
exist anywhere in the tree yet (measured: `rg -n 'fn XFree' femacs/` finds only *uses* in
`src/xterm.rs`'s XCB arms, which no gate compiles), so the arm is the first thing a target that
turns that feature on must supply.

### 4. The toolkit, Motif and cairo conditions (C 387-391, 394-425)

* C 387-391 `#ifndef USE_X_TOOLKIT` around `XrmInitialize ()`: the frozen table has
  `USE_X_TOOLKIT undef`, so C's condition holds and the statement is live; the port writes C's
  condition as `#[cfg (not (feature = "use_x_toolkit"))]` on it, which is the shape
  `include/xterm.rs`'s own `#ifndef USE_GTK` guard already uses.
* C 394-406 `#ifdef USE_MOTIF` / C 406-425 `#else`: `USE_MOTIF` is `undef`, so the `#else` arm is
  the live one.  Both arms get their cfg (`#[cfg (feature = "use_motif")]` and its negation) -
  an if/else pair needs both, or a Motif build would run both arms.
* C 411-418 `#if !(defined USE_CAIRO || defined HAVE_XFT) || !defined (USE_LUCID)`: carried as
  the *exact* condition, `#[cfg (any (not (any (feature = "use_cairo", feature = "have_xft")),
  not (feature = "use_lucid")))]`, not its De Morgan simplification (`!(A||B) || !C` and
  `!(A||B||C)` differ when only `USE_CAIRO` is defined).
* C's `%"pI"d` conversions in the Motif arm are `%ld` (lisp.h's `pI` is `"l"` here).

### 5. Bare `alloca` is the stack pool (C 60-61)

C's two `alloca` calls become `sa_stack_alloc (n) as *mut c_char` with P-05's comment in place,
the `Freplace_region_contents`/`internal_lisp_condition_case` shape.  The sizes are C's
`strlen (name) + sizeof "customization" + 3` and its `"Customization"` twin; `sizeof` of either
literal is 14, so the port writes `+ 14 + 3` with the reading in the comment.  The deviation the
pool documents applies: C's bytes die with the frame, the pool's with a `safe_free` at a
shallower specpdl depth.

### 6. `XrmStringType` is the mirror's, not this unit's

C 372 defines the macro *here*, but the port keeps one spelling: `include/xlib.rs` carries
`pub const XrmStringType: &core::ffi::CStr = c"String";` with C's text in its doc comment (that
file's Decision 17 and the previous round's measurements), and this unit only uses it -
`XrmStringToQuark (XrmStringType.as_ptr ())`, which is what C 386 means and what the byte
comparison in `--check` compares C 372 against.  A second `const` here would be `E0428`.

### 7. `True` (C 484) went to `include/xterm.rs`

C writes `XrmQGetResource (...) == True`; `True`/`False` are Xlib.h 79-80's macros, and no file
in the tree declared them (measured: `rg -n 'pub const True|pub const False' femacs/` found
nothing).  The 3.270 ruling's home is the mirror, and after Decision 16 of `docs/ports/xterm.md`
the X11 macros' *module* lives in `include/xlib.rs`'s `x11_constants` - which is frozen for this
round (the integrator's), so the pair is declared at `include/xterm.rs`'s top level and **moving
those four lines into `x11_constants` is the follow-up**; that module is not glob-imported, so
the constants inside it are reached as `x11_constants::NAME`, while this pair is at the unit's
crate root.  C's spelling is kept (3.270).

### 8. Two `macro-body-notes.tsv` rows are requested (the two remaining `--check` rows)

```
x_get_customization_string	alloca	…
magic_db	switch,case,default,break	…
```

* `alloca`: C 60-61's bare `alloca` cannot survive (P-05); the precedent is
  `Freplace_region_contents	alloca` and `internal_lisp_condition_case	alloca`.
* `switch,case,default,break`: C 116-164's `switch (*p)` becomes a `match` with `_` for C's
  shared `default:`/`case 'S':` body and `b'L' | b'l'`-style or-patterns for C's fall-through
  cases; the precedent is `do_symval_forwarding	switch,case,default`.  Residual, recorded: with
  those four tokens exempted, an arm the port *deleted* inside that `switch` is invisible to the
  token check - the arm-by-arm correspondence is the reading in Decision 4 and the function's own
  `// C 116-164` comment.

No other row is needed: C's three `0` null pointers (C 230's `search_magic_path (…, 0)`,
C 265's `XrmDatabase db = 0`, C 345's `char *filename = 0`) are written `0 as *const c_char` /
`0 as XrmDatabase` / `0 as *mut c_char`, so C's token survives (the `alloc.rs` 681 spelling) and
no exemption is asked for.

### 9. `x_rm_string` (C 374) is carried although the checklist does not name it

`tools/cinventory.py`'s `globals_` drops a file-scope object whose declaration line carries a
trailing comment: it tests the raw line's `endswith (";")`, so `static XrmRepresentation
x_rm_string;	/* Quark representation */` is invisible while C 51's and C 300's (no trailing
comment) are not.  Measured on three-line probes (Evidence).  The port carries the object anyway
- it is what C 374 declares and C 396/C 502 read - and the tool's artifact is reported here, not
compensated for.

### 10. The include list, and the five names the X11 arm probe still owes

The unit names C's own includes in C's order (`config.h`, `unistd.h`, `errno.h`, `epaths.h` ->
the local `PATH_X_DEFAULTS`, `stdlib.h`, `stdio.h`, `lisp.h`, `xterm.h`, the five X11 headers,
`pwd.h`) and, in addition, four mirrors that C reaches through `xterm.h`'s chain and whose names
the port's mirrors spell: `include/systime.rs` (`Time`), `include/character.rs` (`MAX_CHAR`,
`CHARACTERP`, `CHAR_VALID_P`), `include/coding.rs` (`coding_system`) and `include/composite.rs`
(`composition_method`, `MAX_COMPOSITION_COMPONENTS`).  P-72 rule 4 puts those on the unit, and
the four are exactly what `build-tu` demands (each was a compile error until it was named).
`include/xlib.rs` is named *directly* for two reasons: C includes those five headers directly,
and `tools/extract-abi.py`'s include follower is **one level deep** (measured in its
`rust_inventory`), so a name that only a mirror-of-a-mirror carries (`XrmStringType` in
`xlib.rs`, reached through `xterm.rs` -> `dispextern.rs`) is invisible to `--check` - naming
`xlib.rs` is what turned the two `missing macro/constant XrmStringType` rows green.

`PATH_X_DEFAULTS` is the generated header's measured value on this machine
(`build/emacs-ref/src/epaths.h` 82: `#define PATH_X_DEFAULTS ""`), the `callproc.rs` shape; a
configuration that builds this file splices `epaths.in`'s X search path there instead, and that
is recorded rather than guessed.

Five names the X11 arm probe still reports are **not** this unit's to fix: C declares
`get_homedir`/`splice_dir_file` in `lisp.h` 4968/4971 (their definitions are `src/fileio.rs`'s),
and `include/lisp.rs` - lisp.h's mirror - does not carry the two prototypes, so the arm cannot
resolve the three + two call sites.  The request is two lines in `include/lisp.rs`:
`/// C: lisp.h 4971: extern char const *get_homedir (void) ATTRIBUTE_RETURNS_NONNULL;` +
`pub unsafe extern "C" fn get_homedir () -> *const c_char;` and the `splice_dir_file` pair from
lisp.h 4968.  Every other arm error is the mirrors' recorded X11 gap (212 of them are
`include/xterm.rs`'s bare `Atom`/`Window`/`Cursor`/`GC`/`Pixmap` names).

## Evidence

Readings, in the order they were taken (round 188, 2026-10-05; commands verbatim).

```
$ python3 tools/check-cond-eval.py src/xrdb.c
src/xrdb.c: 6 conditions, 0 unknown, 0 disagreements [VERIFIED]

$ python3 tools/extract-abi.py emacs/src/xrdb.c            # the checklist, before it was ticked
## Functions defined — 11   (x_get_customization_string line 58 static, magic_db line 102 …)
## Macros defined — 1       (XrmStringType line 372, body 4bb71e1c2a36)
## File-scope objects — 2   (static char x_customization_string line 51,
                            static char const xdefaults line 300)
# The four boxes' contents are quoted in the lists above, ticked after the port landed; the
# tool's own `[ ]` lines are not repeated here so that the ledger's box count stays its own.
```

The `PBaseSize` reading (Decision 2) and the `x_rm_string` artifact (Decision 9), from
`tools/cinventory.py` used as a module:

```
$ grep -n PBaseSize /opt/homebrew/include/X11/Xutil.h
111:#define PBaseSize	(1L << 8) /* program specified base for incrementing */
$ python3 -c "…ci.read_condition_macros ('docs/baseline/config/darwin-arm64.macros.tsv')…"
PBaseSize                        <absent>
$ python3 -c "…ci.globals_ ('static XrmRepresentation x_rm_string;\n')"
[{'c_type': 'static XrmRepresentation', 'name': 'x_rm_string', …}]      # without the comment
$ python3 -c "…ci.globals_ ('static XrmRepresentation x_rm_string;\t/* Quark representation */\n')"
[]                                                                      # with it: dropped
```

The unit's own gate readings (this round's five):

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-g188-j cargo run -q -p xtask -- build-tu src/xrdb.c
built src/xrdb.rs -> ../build/femacs/xrdb.o

$ python3 tools/extract-abi.py --check emacs/src/xrdb.c femacs/src/xrdb.rs
- function x_get_customization_string does not mention alloca
- function magic_db does not mention break, case, default, switch
FAIL emacs/src/xrdb.c -> femacs/src/xrdb.rs

$ cd femacs && … layout-check
layout probe ok
layout-check passed (see build/femacs/layout_asserts.rs)

$ python3 tools/check-target-names.py     -> 628 .rs files, 591 map targets, 0 finding(s)
$ python3 tools/check-declarations.py     -> 13 declaration sites (0 forbidden, 13 allowed
                                             literals), 0 failures
$ python3 tools/check-citations.py        -> 670 file(s) scanned, 0 drift, 0 range, 349
                                             external, 33 allowed, 0 stale
$ python3 tools/check-mirror-dupes.py     -> 31 duplicate declaration(s), 18 ratchet row(s),
                                             13 failure(s)  (all of them `src/xdisp.rs`/
                                             `src/frame.rs`/`src/keyboard.rs`, other lanes'
                                             in-flight work; measured: none names this round's
                                             files)
```

The X11 arm, type-checked as a probe (the unit `build-tu` materialised, then rustc with the
feature on) - this is the only reading that compiles the file's *own* text:

```
$ rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
      --cfg 'feature="have_x_windows"' -o /tmp/xrdb_x.o build/femacs/units/xrdb/_root.rs
219 errors: 212 in include/xterm.rs (Atom 124, Cursor 19, Window 13, GC 6, Pixmap 4, … - the
           mirrors' recorded X11 gap), 5 in this unit's own text (get_homedir 3,
           splice_dir_file 2 - Decision 10's request), 1 the intended
           systime.rs `assert! (!cfg! (have_x_windows) || HAVE_X_WINDOWS)` (E0080), 1 plain

# the first probe run (before the fix it found):

error[E0608]: cannot index into a value of type `*mut i8`
193 |   path[path_len] = '\0';
    |       ^^^^^^^^^^
# -> `*path.offset (path_len) = 0;` (C 184), the port's spelling for `path[i] = c` on a pointer
```
