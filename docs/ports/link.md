# The link rung: what a runnable `emacs` still owes

Not a port of a reference file.  This is the record for the **link stage** - the first rung of
"the port can run at all" (`docs/port-plan.md` §7.1's L0-L5 sit above it) - and it exists because
every gate in this tree reads `.o` files or text: `build-tu` stops at one object per unit,
`layout-check` compiles probes, `check-undef.py` looks at each unit's own undefined list, and
`tools/check-swift-imports.py` covers the Swift side of the C-ABI seam.  **Nothing measured the
program.**  This file records how to measure it and what the first measurement said.

## The input set is C's, not ours

C's own recipe (`build/emacs-ref/src/Makefile`):

```
temacs$(EXEEXT): $(LIBXMENU) $(ALLOBJS) $(LIBEGNU_ARCHIVE) $(EMACSRES) … $(LIBES)
LIBEGNU_ARCHIVE = $(lib)/libgnu.a
```

so the objects of the `emacs` program are

* the frozen build's own sources (`docs/baseline/frozen-build-sources.txt`, 83 entries) mapped to
  `build/femacs/<stem>.o`, **minus** every object that defines `_main` (the frozen list also names
  the separate programs: `emacsclient.c`, `ebrowse.c`, `etags.c`, `hexl.c`, `make-docfile.c`,
  `make-fingerprint.c`, `movemail.c`; `src/thread.c` is *not* one of them - a `nm` filter that
  matches `_main` as a prefix wrongly drops it, which is how this was learnt), **plus**
* the members of `build/emacs-ref/lib/libgnu.a` mapped to `build/femacs/<stem>.o`.

Attempting the narrower set (frozen sources only) shows 459 undefined symbols; the wider, correct
set shows 427.  `build/femacs/*.checking.o` must be excluded: the `enable_checking` builds define
every symbol a second time (a first attempt without that filter reported 4,978 duplicates).

## The measurement

```sh
# 1. C's object set (see above) -> /tmp/link_set.txt, one path per line
# 2. a partial link merges them and reports what no member defines:
ld -r -o /tmp/merged.o $(cat /tmp/link_set.txt)
nm -u /tmp/merged.o | awk '{print $NF}' | sort -u > /tmp/link_undef.txt
wc -l < /tmp/link_undef.txt
```

`ld -r` needs no runtime libraries and no successful executable link, which is what makes it a
useful first rung: it answers "which symbols does the program reference and the port never define"
without pretending the program is ready to run.

## First reading (2026-10-04, 162 objects: 76 program + 86 libgnu)

**427 undefined symbols.**  Classified against the reference objects
(`nm -g build/emacs-ref/**/*.o`, split into definitions and undefined) and the SDK's `.tbd`:

| class | count | what it means |
| --- | ---: | --- |
| `rust-runtime` | 20 | `_RNv…4core…panic*` - provided when the objects are linked by rustc/the Rust runtime, not this port's debt |
| `libc-or-platform` | 269 | the reference objects reference these too, so libc/libSystem/the SDK provides them |
| `port-owed` | 59 | **the reference defines them and no unit of ours does** |
| `unknown` | 79 | neither: mostly C's `globals.h`-generated `V…` carrier symbols (the port's *documented* deviation - `check-symbols.py` counts 467 of them as folded into Rust statics) plus `src/xdisp.c`'s unported remainder (`calc_pixel_width_or_height`, `automatic_hscrolling`, `composition_break_at_point`, `handle_line_prefix`, `maybe_produce_line_number`, …) and a few libc names |

Two caveats, both measured rather than assumed:

* the `libgnu.a` member -> `build/femacs/<stem>.o` mapping needs care, and the reason is
  structural rather than a typo: gnulib amalgamates sources with `#include`, and this port splits
  them into one unit per **file**.  `ar t build/emacs-ref/lib/libgnu.a` names `mini-gmp-gnulib.o`,
  and `nm -g build/femacs/mini-gmp-gnulib.o` shows no `mpz_*` at all - those live in
  `build/femacs/mini-gmp.o` (`nm -g build/femacs/mini-gmp.o | grep -c ' T _mpz_'` = **127**), which is a *separate* unit here
  because `lib/mini-gmp.c` is its own port file.  So the link set for this port is not simply
  "the archive members mapped by stem": it must also add the units whose sources the amalgamating
  member `#include`s.  Until that rule is written down, the `port-owed` count is a lead rather
  than a verdict.
* the `V…` symbols are the port's known and documented deviation, so a link driver must decide
  whether to (a) export them after all, (b) accept them as a ratchet, or (c) prove no unit
  references them by that name.  This file records the question; it does not answer it.

## The tool

`python3 tools/check-link.py` (default: compare with the ratchet; `--write-pending`; `--report`
prints every name with its class).  It refuses to compare when a frozen source has no built
object (`gate-fast`/`build-tu` first), because an incomplete link set inflates the list.

## Reading two (2026-10-04, after the amalgamation rule): 369 undefined, and `port-owed` is zero

With the `X-gnulib.o` -> `X.o` rule the partial link of 76 program + 87 libgnu objects leaves
**369** names, classified as **`libc-or-platform` 270**, **`rust-runtime` 20** and **`unknown` 79** -
**nothing is `port-owed`**: every symbol the reference's own units define, this port's units define
too.  The 79 `unknown` break down (measured, `--report` plus the two comparisons) as

* **19** C carriers of the form `V...` - the port's *documented* deviation (`check-symbols.py`
  counts 467 of them as folded into Rust statics),
* **7** names `src/xdisp.c` still owes (`redisplay_internal`, `message_truncate_lines`,
  `inhibit_message`, ...), which resolve as that unit lands,
* **53** tail: libc/BSD entries the reference-based classifier cannot see because C reaches them
  through inlined or macro paths (`clearerr`, `environ`, `fabs`, `fdopen`, `feof`, `ferror`,
  `fsync`, `getc`, `getchar`, `htons`, `lrint`, `ntohs`, `pathconf`, `pipe2`, `pselect`, `putc`,
  `putchar`, `realpath`, `sqrt`, `sync`, `umask`, `sigaddset`, `sigemptyset`, ...), plus the port's
  deliberate structural deviations (`current_buffer`, `specpdl`/`specpdl_ptr`/`specpdl_end`, which
  are Rust structures here) and `src/xwidget.c`'s unported `syms_of_xwidget`.

So the honest state of the link rung is: **the object set closes over itself**; what is left is one
*decision* (do the `V...`/`specpdl`/`current_buffer` deviations get C-ABI carriers after all, or do
they stay the documented deviation?), `xdisp`'s remainder, and the real link driver's `cc`
invocation with `-lSystem` and the frameworks.

## What the next step is

`xtask link` (a new subcommand, not yet written): build C's object set as above, run the partial
link, print the undefined list, and compare it against a ratchet
(`docs/baseline/link-pending.txt`) the way `check-undef.py` and `check-swift-imports.py` compare
theirs - **a new name fails, a name that disappeared is stale and is deleted**.  Its first job is
to resolve the two caveats above, and its second is to intersect its list with
`docs/baseline/swift-import-pending.txt` (533 names the Swift units import): the intersection is
the set of C-ABI entries a runnable macOS build needs *first*, and `femacs/src/abi_shim.rs` is
where they land.

## Reading three (2026-10-04): the first real link, and the four names only it could see

C's own recipe is the `temacs` rule of `build/emacs-ref/src/Makefile`; `make -n temacs` in
`build/emacs-ref/src` prints it (line 457 of the expansion):

    gcc -std=gnu23 -o temacs.tmp <ALLOBJS> ../lib/libgnu.a -lncurses
    ../lib-src/make-fingerprint temacs.tmp
    codesign -s - -f temacs.tmp && mv temacs.tmp temacs

`LIBXMENU`, `LIBX_BASE`, `PGTK_LIBS`, `LIBS_GNUSTEP`, `LIBS_SYSTEM` and `EMACSRES` are empty in the
frozen configuration, so the program's whole external library set is **`-lncurses`**.

The port's equivalent, object for object (the same 163 objects `tools/check-link.py` links):

    cc -o /tmp/femacs-link $(cat /tmp/link_objs.txt) -L/opt/homebrew/opt/ncurses/lib -lncurses \
       $NL/libcore-*.rlib $NL/libpanic_abort-*.rlib $NL/libcompiler_builtins-*.rlib   # $NL = rustc nightly-2026-10-01 --print target-libdir

| what the command line has | undefined |
| --- | ---: |
| `cc` + the implicit `-lSystem` | 118 |
| + C's own `-lncurses` | 106 |
| + the Rust runtime's three rlibs | 88 |
| + the two fixes below | **84** |

**What the rung caught - four names every text gate was blind to.** They were *invented* by the
port, not ported, and each is a call C's own object does not make:

* `_fflush_unlocked`, `_fputc_unlocked`, `_fwrite_unlocked`: `src/sysdep.c` 2914-2943 spells those
  names, but on this target they are `lib/unlocked-io.h`'s macros mapping onto `fflush`, `fputc`
  and `fwrite` (the mirror `lib/include/unlocked-io.rs` carries the measured live arm), and this
  target's `<stdio.h>` declares none of them - measured: a probe that includes `<stdio.h>` and
  calls them does not compile, and `nm -u build/emacs-ref/src/sysdep.o` names `_fflush`, `_fputc`,
  `_fwrite`.  `sysdep.rs` had turned each macro into an `extern` declaration, so its text matched
  C's text and no text gate could see the difference.  Fixed: the call sites invoke the mirror's
  macros and the three declarations are gone.
* `_VFORK`: `src/callproc.c` 1534 spells `VFORK ()`, `conf_post.h` 280-291's macro, whose
  `DARWIN_OS` arm is `vfork ()` - `nm -u build/emacs-ref/src/callproc.o` names `_vfork`.
  `callproc.rs` had declared a *function* `VFORK` instead.  Fixed: `VFORK! ()`, plus `<unistd.h>`'s
  own `vfork` declaration next to the file's `fork`.

**What is left (84).** 82 are names of units that are still being ported, plus the decisions
already recorded, plus 2 that are the driver's:

* `src/xdisp.c`'s remainder - about 62 - is the live work (`docs/ports/xdisp.md`).  The classifier
  calls most of them `unknown` rather than `port-owed` for a measurement reason worth fixing
  first: **`build/emacs-ref/src/xdisp.o` was never built**, so the reference-definition set the
  classifier compares against has no `xdisp` in it.
* the 19 `V...` carriers (referenced by `alloc.o`, `keyboard.o`, `bidi.o`, ...), `_current_buffer`,
  `_specpdl`, `_specpdl_ptr`, `_specpdl_end` and `_syms_of_xwidget` are the port's documented
  deviations (`check-symbols.py` counts 467 `V...` as folded into Rust statics) and the unported
  `src/xwidget.c`; they are a decision, not a discovery.
* `__RNvCs8kYXCe6bXIp_7___rustc17rust_begin_unwind` and `_rust_eh_personality` are the Rust
  runtime's own seam: a `--emit obj` unit crate carries no `#[panic_handler]`, so whoever *drives*
  the final link owes them (rustc as the driver, or one small root crate that owns the handler).
  That is the driver question this rung leaves open, and it is the last non-`xdisp` step before an
  `emacs` executable exists.

### Reading three, the composition refined (same round)

`build/emacs-ref/src/xdisp.o` was the one frozen source with no reference object (`make xdisp.o`
in `build/emacs-ref/src` builds it in seconds; `check-symbols.py` and `check-undef.py` read the
same either way - measured by moving it aside and back).  With it in place the 84 classify as
**`port-owed` 15, `unknown` 67, `rust-runtime` 2**; the 15 are `xdisp`'s *exported* definitions
(`_disp_char_vector`, `_init_xdisp`, `_count_lines`, `_mark_window_display_accurate`,
`_note_mouse_highlight`, ...).  The `unknown` 67 are three classes, and the referrers are the
point - every one of them is one of *our* objects:

| class | names | referenced from | what it is |
| --- | ---: | --- | --- |
| `xdisp`'s statics | ~47 | `xdisp.o` | `static` in C, so `nm -g` cannot see them (an all-symbols pass reads 28 `port-owed`); they land with the unit |
| C's generated globals | ~20 | `xdisp.o`, `indent.o`, `term.o`, `alloc.o`, `bidi.o`, `window.o`, `dispnew.o` | `hscroll_margin`, `inhibit_message`, `automatic_hscrolling`, the `V...` family, `current_buffer`, `specpdl`/`specpdl_ptr`/`specpdl_end` |
| `src/xwidget.c` | 1 | `emacs.o` | `syms_of_xwidget`, unported |

The middle row is the interesting one, and it is a *structural* finding rather than a name list.
C has no symbol `_hscroll_margin` at all: `build/emacs-ref/src/globals.h` 893 is
`#define hscroll_margin globals.f_hscroll_margin`, and the storage is one object,
`struct emacs_globals globals;` in `src/alloc.c` 278.  This port's mirror instead declares a
flattened `pub static mut hscroll_margin: EMACS_INT;` (`src/include/globals.rs` 7015), and
`check-symbols.py` records the family as 467 names "folded into Rust statics" - but nothing
*defines* them, so every cross-unit reference to one is undefined at link time.  Two ways out,
both faithful in a different sense, and the choice is the same one `docs/status.md` 3.244's
caveat named: reproduce C's carrier (`struct emacs_globals globals;` plus the generated field
list, which is C's own text and which the field-layout gate can check), or keep the flattened
names and give each one storage at a single point (`femacs/src/abi_shim.rs` is where the port
already puts C-ABI entries nobody else defines, 3.248).  Either way it is one decision made once,
not 467 small ones - and it is the last input-set-shaped gap this rung found.

### Reading three, corrected: the family is not a missing carrier

The section above called the ~20 generated-globals names "the last input-set-shaped gap" and
said nothing defines them.  Measuring further in the same round gives the sharper account, and
the two sentences it corrects are the ones about a *structural* gap and about *one decision*.

* **44 of the 46 belong to `src/xdisp.c`.**  Their `DEFVAR_*` calls are live text, so the unit
  that owns them defines them as it lands (`pub static mut Vdisplay_line_numbers: Lisp_Object =
  lisp_h_Qnil ();` - the `callint.rs`/`casetab.rs` shape of status 3.29).  Nothing is owed to a
  decision here; it is the in-flight unit's own work.
* **`current_buffer`, `specpdl`, `specpdl_ptr`, `specpdl_end`** are C's *thread* macros
  (`thread.h` 131/139/147, e.g. `#define specpdl (current_thread->m_specpdl)`).  The mirrors
  declare the storage as an interim measure and say so on the spot, with `thread.rs` as the
  landing point.
* **`Vcomp_eln_to_el_h` was the one real gap, and it is a *liveness* gap rather than a structural
  one.**  C 5766's `DEFVAR_LISP` is dead text (`HAVE_NATIVE_COMP` is undef), but C's *storage* is
  the generated struct's field and exists in every configuration - and `src/lread.c` 1019, a
  line with no enclosing `#if` at all (measured against the preprocessor's own nesting), reads
  it.  Fixed in `femacs/src/comp.rs`: the storage stands there even though the registration does
  not, which is the "the owner defines it" rule of 3.29 applied to a member whose `DEFVAR` never
  compiles.  The executable link's undefined list goes 84 → 83 and the name is gone.

So the per-unit carrier does not need redesigning.  What it needed was the rule that a member's
*storage* follows the generated struct and is unconditional, while only its *registration*
follows the frozen configuration's live text - and one member was the whole of the difference
today.

### The rung is a gate now

`tools/check-link.py` is wired into `xtask gate-fast`, next to the symbol and linkage gates: the
class it found - a name that is referenced, defined by C, and defined nowhere here - is invisible
at every lower level (`build-tu` stops at one object, `check-undef.py` reads one unit's own
undefined list, `check-linkage.py` asks the opposite direction).  The ratchet is regenerated for
the two fixes: **383 rows, 0 failures**.

## Reading four (2026-10-04): the executable level, and the runtime seam is closed

```sh
python3 tools/check-link.py --executable              # link the program, compare the ratchet
python3 tools/check-link.py --executable --write-pending
```

`--executable` links the program instead of merging objects: the partial link's 163 objects plus
two things no unit owns, both taken from the authority rather than written down here.

* **The Rust runtime seam.**  A unit is compiled `--crate-type lib --emit obj`, so no unit carries
  a `#[panic_handler]`, and the program's link owes `__rust_begin_unwind` and
  `_rust_eh_personality`.  The mode materializes a four-line `#![no_std]` crate at
  `build/femacs/rust-runtime-root.rs`, compiles it with the pinned toolchain (the subprocess runs
  from `femacs/`, so `rust-toolchain.toml` decides which `rustc`), and links the result with
  `libcore`/`compiler_builtins` from that toolchain's `target-libdir`.  This is the driver question
  reading three left open, and it is now answered and measured: the two names are gone.
* **C's own library line.**  `make -n temacs` in `build/emacs-ref/src` prints C's recipe, and the
  mode reads its `-L`/`-l`/`-framework` flags - in the frozen configuration `LIBXMENU`,
  `LIBX_BASE`, `PGTK_LIBS`, `LIBS_GNUSTEP`, `LIBS_SYSTEM` and `EMACSRES` are empty, so the set is
  the sixteen Homebrew `-L` directories and `-lncurses`.

**Reading four: 71 undefined names** in `docs/baseline/link-exec-pending.txt` (`port-owed` 10,
`unknown` 61), 0 failures against that ratchet.  The classes are reading three's, minus the two
runtime names:

* `src/xdisp.c`'s remainder - its live `DEFVAR_*`s and its `static`s - which lands with that unit;
* `_current_buffer`, `_specpdl`, `_specpdl_ptr`, `_specpdl_end`, the recorded interim deviation
  whose landing point is `thread.rs`;
* `_syms_of_xwidget`, the unported unit.

So an `emacs` executable is *one unit deep* rather than missing infrastructure: the link driver,
the runtime seam and C's library line are all in place and measured, and what is left is port
text.

Two caveats, both measured rather than assumed:

* both ratchets are taken on the tree as it stands, and this worktree has lane objects in flight.
  A *clean* build at HEAD can therefore show a name this ratchet lacks (the row is then added) or
  a row it no longer needs (the row is deleted) - the reconciliation is the ordinary ratchet
  churn, and the place to do it is a `git worktree` at HEAD, which is also where `gate-fast` is
  run.
* `--executable` is deliberately **not** wired into `gate-fast` yet: the partial link's ratchet is
  (see above), and the executable level should first be measured on a clean HEAD build so its
  rows are not per-turn churn.

**The stale-row policy (same round).**  A ratchet row that no longer holds is the ratchet
*shrinking*, and a lane's object in flight does exactly that: `src/xdisp.rs` landing ten more of
its own definitions made ten rows informational within the hour (measured: 373 undefined against
the 383-row ratchet, 11 rows that no longer hold, 1 new name from the same lane's in-flight
slice).  So `check-link.py` fails on a **new** name in either mode, and reports stale rows as
information unless `--strict` is passed - and `--strict` is what a clean worktree at HEAD is
reconciled with, where the file has to be exact.  `gate-fast` uses the default, so a shrinking
ratchet cannot make the fast gate red while five lanes are rebuilding objects.

## Reconciling the ratchets at HEAD (2026-10-04)

Both ratchets are `nm` readings of `build/femacs/*.o`, and this repository always has lane objects
in flight: a unit being ported defines some of its own names and references others before defining
them.  Regenerating a ratchet in the working tree therefore bakes the lanes' half-finished state
into a file every later gate compares against.  The reading that counts is the one at HEAD, and it
is a two-minute measurement:

```sh
tools/reconcile-link-ratchets.sh
```

A throwaway worktree at HEAD, `emacs/` and `build/emacs-ref/` symlinked in (both are git-ignored,
so a worktree has neither), the current objects copied over, and **only** the units the working
tree has in flight rebuilt from HEAD's own sources.  Then the partial ratchet and the executable
ratchet are written and re-checked with `--strict`, which is the post-condition - the file must be
*exact* for HEAD's objects, not approximate.

**Reading five (HEAD `ee92417`, reconciled):** partial link **365 names**, executable link **63
names** (`port-owed` 15, `unknown` 48).  On the working tree at that moment the same two gates read
373 and 71, with 14 names that only the in-flight `src/xdisp.rs` references - so the difference
between the two readings is the lanes' progress and nothing else.  That is why the stale-row policy
above exists: a ratchet regenerated in the working tree is *smaller* than HEAD's, and the next
clean run would report HEAD's names as new.

**What the link set does *not* contain (measured, same round).**  `docs/baseline/frozen-build-sources.txt`
lists 106 entries and **not one `.m`/`.mm`**: the frozen configuration builds a terminal-only
program, so the Swift ports of `src/*.m` (`femacs/platform/macos/`) are another target's text and
enter a program only when such a target is built - which is what `docs/ports/nsmenu.md`'s decision
records ("the port carries the target's text rather than a compilable unit").  The executable link
set is therefore the 163 C-ABI objects and nothing else, and `check-link.py`'s "0 frozen source(s)
have no built object" is a statement about a complete set rather than a filtered one.

## 2026-10-05 (status 3.594): the `lib/` half of the link set had never been built

Everything above reads `build/femacs/*.o`, and until today **nothing built the port's `lib/` units**:
`tools/check-all-units.sh` sweeps `docs/baseline/frozen-build-sources.txt`, which is the *program*'s
source list - the gnulib layer is not in it (it ships as `libgnu.a`).  `gate-load` assembles
`build/load/libgnu-port.a` out of whatever `lib/*.o` happen to exist, so with nothing built the
archive held **one** member, and `check-link.py` read **101 `port-owed` symbols that the port
defines** - 59 of them mini-gmp's alone, whose port is `femacs/lib/mini-gmp.rs` (5,237 lines).

Measured, in this order:

* one extra object (`lib/mini-gmp.c`): `port-owed` **101 -> 47**, undefined 456 -> 398, failures 118 -> 59;
* the 86 sources of **`docs/baseline/lib-link-sources.txt`** (derived from `ar t
  build/emacs-ref/lib/libgnu.a`'s 87 members - `lib/<m>.c`, `lib/<m-minus-`-gnulib`>.c`,
  `lib/malloc/<m>.c`): `port-owed` -> **5**, undefined -> **381**, failures -> **17**, and the sweep
  itself is clean (`checked 86 unit(s), 0 failure(s)` + `checking 86 unit(s), 0 unit failure(s)`).

So the link set is `build/femacs/<stem>.o` for the program's 76 objects **plus the 86 lib objects**;
before `check-link.py`, run `KEEP_UNITS=1 tools/check-all-units.sh docs/baseline/lib-link-sources.txt`.

**The 17 that remain are all `src/xdisp.c`**: nine `V...` carriers the unlanded `syms_of_xdisp`
defines (`Vglobal_mode_string`, `Vline_number_display_limit`, `Vline_prefix`, `Vmode_line_compact`,
`Vwrap_prefix`, `Vdisplay_line_numbers_{current_absolute,width}`,
`Vdisplay_fill_column_indicator_{character,column}`), `pad_mode_line` (C 33,406), and six
`#ifdef HAVE_WINDOW_SYSTEM` helpers (`display_fill_column_indicator`,
`display_line_numbers_{major_tick,minor_tick,offset,widen}`, `line_number_display_limit_width`) that
**the port's own `xdisp.o` references where the reference object does not** - a review item for the
xdisp slice in flight (is the port's `#[cfg]` on those call sites the same as C's?), not a link-set
artifact.  `_strcat` is a libc name the classifier files under `unknown`.
## Reading five (2026-10-15): the archive is an input, and a missing one now says so

`gate-fast` reached this gate for the first time at full scale in the round of `docs/status.md`
4.370 and read `378 undefined symbol(s) from 76 program + 0 libgnu object(s), 102 failure(s)` -
every one of the 102 a gnulib entry point (`_mpz_*`, `_sha*_buffer`, `_careadlinkat`,
`_md5_buffer`, `_sig2str`, ...).  The cause is this gate's own second input: C's `temacs` rule
links `$(lib)/libgnu.a` (see "The input set is C's, not ours"), and
`build/emacs-ref/lib/libgnu.a` had never been built in this tree, so `lib_objects()` mapped no
member onto a port object and every name the port's `lib/` units own read as undefined.  Built
once (`make -C build/emacs-ref/lib libgnu.a`, 2.6s, 86 members):

    302 undefined symbol(s) from 76 program + 87 libgnu object(s), 302 ratchet row(s), 0 failure(s)

so the gate is green on its own measurement, and the ratchet shed the seven rows nothing
references any more (`_current_buffer`, `_pipe2`, `_reset_mouse_highlight`, `_specpdl`,
`_specpdl_end`, `_specpdl_ptr`, `_syms_of_xwidget`: 309 -> 302 rows,
`docs/baseline/link-pending.txt`).

A missing archive must not read as port debt, so `lib_objects()` now raises a setup error (exit
2) naming the fix and both readings, instead of classifying 102 names against an incomplete
object set.  Measured with the archive moved aside:

    setup: build/emacs-ref/lib/libgnu.a is missing: C's own `temacs` rule links it ... without
    it, 378 from 76 + 0 object(s) and 102 failure(s) - all of them gnulib entry points.  Build
    it once: `make -C build/emacs-ref/lib libgnu.a`.

**Open (not claimed to be understood)**: the archive was present in the round's four gate runs
and absent in one, and no single tool run reproduces the loss - each of
`check-cond-eval.py`, `check-globals.py`, `check-ledger-ticks.py`, `check-dead-file-coverage.py`,
`extract-abi.py --all`, `check-symbols.py` (both forms), `check-undef.py`, `check-link.py`, the
link gate's own `make -n -W emacs.o temacs` and `build-tu` leaves 161 objects and the archive in
place.  The guard is what makes a next occurrence read as a setup gap rather than as debt.

## Reading six (2026-10-16, status 4.371): the port has to own C's member set - and the first run

`gate-fast` went green end to end (status 4.370), so `gate-load` became the next rung's evidence -
and it failed on its first full-scale run with one name:

    undefined symbols: 1 (referenced from 1 object(s))
         1  libgnu-port.a[46](getloadavg.o)

### What the archive stands in for

The gate substitutes the port's objects for the reference archive's members, and until this round
it substituted **every ported `lib/` unit** ("the port has no archive of its own", the call site's
older comment).  That is not C's object set.  The reference archive names 87 members with `ar t`,
and `libgnulib`'s own module list is what decides them: a `lib/` file whose module this
configuration does not use (*the system provides the function*) is **not compiled** - `open.c`,
`fcntl.c`, `getdelim.c`, `getline.c`, `getgroups.c`, `getloadavg.c`, `mkostemp.c`, `stpcpy.c`,
`strnlen.c`, `strtoimax.c`, `strtoll.c`, `time_r.c`, `fchmodat.c`, `cloexec.c` and more.

Linking those objects is not harmless, because gnulib writes such a replacement under the
**plain** name of the function (not a `rpl_` rename, which is why `_rpl_dup2` and friends are
inert).  The port's object then *defines a libc name* and shadows libSystem's implementation for
the whole program.  Measured two ways:

* an object-set audit - port objects whose definitions the whole C build (its `src/*.o`, its
  `lib-src/` objects and its archive) does not define, and which some object references: **14
  definitions across 11 units** (`open`, `fcntl`, `getdelim`, `getline`, `getgroups`, `mkostemp`,
  `stpcpy`, `strnlen`, `strtoimax`, `strtoll`, `time_r`);
* the program itself: it died before running any Lisp, in the port's own `open` -
  `lldb` on `build/load/temacs-port` stopped at `EXC_BAD_ACCESS` in `open + 16`.

The gate now archives the port's objects for the reference's members (with the amalgam rule:
`X-gnulib.o` also takes `X.o`), and names any member it has no object for.  Reading: `75 port
object(s) + 87 lib object(s)`, `undefined symbols: 0`, `link: temacs linked from the port's
objects alone`.

### The executable, and its tree

The link's output moved from `build/load/temacs-port` to `build/port/src/temacs`, with the trees
the port owns linked beside it (`build/port/lisp -> ../../femacs/lisp`, `etc`, `leim`, `doc`,
`test`, `admin`; `lib-src/` is a real directory).  That is not cosmetic either: C's
`init_cmdargs` walks up from the executable's directory looking for a sibling `lib-src` and `etc`
(emacs.c 585-630) and sets `installation-directory` from what it finds, which is what
`init_callproc` turns into `exec-directory` and the default load path.  A binary in a bare
directory therefore behaves like an *installed* program whose prefix does not exist: measured,
`exec-directory` was `/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0/` and the program
printed `Warning: arch-dependent data dir ...` that the reference does not - the **only** stderr
difference between the two runs.  With the tree in place the walk resolves to the port's own tree
and the warning is gone.

### What the program does now

* `build/port/src/temacs -Q --batch --eval '(princ (+ 1 2))'` with `EMACSLOADPATH` on the
  reference's `lisp/`: stdout `3`, exit 0, and **3,734 bytes of stderr byte-identical** with the
  reference's own run - the whole `loadup.el` trace, from `Loading loadup.el (source)...` through
  `Making lexical-binding buffer-local while locally let-bound!`.
* `tools/acceptance-l0.py --check build/port/src/temacs --golden-path
  docs/baseline/l0-reference-temacs.txt`: `19 expression(s), 0 failure(s)`.
* On its **own** tree (no `EMACSLOADPATH`): the port loads its own `lisp/` - the load path reads
  `build/port/lisp/...`, sources are loaded as `(source)`, ~40 files - and stops at the first
  file the verbatim copy does not have: `Cannot open load file: No such file or directory,
  charscript`, required by `international/characters.el`.  That is the generated-Lisp gap
  (`admin/unidata`'s outputs, `lisp/loaddefs.el`, `subdirs.el`), not a code defect, and it is the
  next rung's work (docs/acceptance.md's generated-artifact section).

### Owed (measured, not fixed)

The **per-unit** file gates P-86 clause 8 asks for: each of the 11 units above keeps its text but
should carry the reference's own build condition, so its object is the empty shape in the frozen
configuration rather than a definition the archive happens not to link.  `getloadavg.rs` is the
first one done (its gate is in `docs/ports/getloadavg.md`); the other ten are owed, with the audit
above as the acceptance instrument (after all eleven, the audit's list is empty).
