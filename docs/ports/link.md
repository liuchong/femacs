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
  `build/femacs/mini-gmp.o` (`nm -g … | grep -c ' T _mpz_'` = 60), which is a *separate* unit here
  because `lib/mini-gmp.c` is its own port file.  So the link set for this port is not simply
  "the archive members mapped by stem": it must also add the units whose sources the amalgamating
  member `#include`s.  Until that rule is written down, the `port-owed` count is a lead rather
  than a verdict.
* the `V…` symbols are the port's known and documented deviation, so a link driver must decide
  whether to (a) export them after all, (b) accept them as a ratchet, or (c) prove no unit
  references them by that name.  This file records the question; it does not answer it.

## What the next step is

`xtask link` (a new subcommand, not yet written): build C's object set as above, run the partial
link, print the undefined list, and compare it against a ratchet
(`docs/baseline/link-pending.txt`) the way `check-undef.py` and `check-swift-imports.py` compare
theirs - **a new name fails, a name that disappeared is stale and is deleted**.  Its first job is
to resolve the two caveats above, and its second is to intersect its list with
`docs/baseline/swift-import-pending.txt` (533 names the Swift units import): the intersection is
the set of C-ABI entries a runnable macOS build needs *first*, and `femacs/src/abi_shim.rs` is
where they land.
