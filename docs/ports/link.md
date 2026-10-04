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
