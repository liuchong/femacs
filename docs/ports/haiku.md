# emacs/src/haiku.c (286 lines)

## Decisions

`emacs/src/haiku.c` @ `fd7afced403`; port `src/haiku.rs` (**487 lines**); the gate is
`feature = "have_haiku"` (P-86 clause 8), which is the **build's** condition rather than one the C
text spells: `configure.ac` 3097-3107 builds `HAIKU_OBJ="$HAIKU_OBJ haiku.o"` and 3107 defines
`HAVE_HAIKU`, and `src/Makefile.in` 378/469 put `$(HAIKU_OBJ)` into the object list.
`docs/baseline/frozen-build-sources.txt` has no `src/haiku.c`, so the frozen configuration compiles
nothing from this file and the port's frozen object is the empty one.

- **This file is `text-only`** (P-86 clause 3(2)): there is no Haiku SDK on this machine, so
  `<kernel/OS.h>` was never seen and **every type width and record offset below is C's spelling, not
  a measurement**.  What the round *can* show is that the arm type-checks (see `## Evidence`).
- **The `<kernel/OS.h>` surface stands in the unit** (P-88 clause 1): no mirror in this tree carries
  it and this file is its only consumer.  Each record carries **only the members the reference
  reads**, in C's order, and the records' other members are named as owed in `## Owed` - the
  `include/gtk.rs` `GtkWidgetClass` shape (status 3.481), which is the house answer to "the header
  is not on this machine".
- **Three chain edges are named in the unit** (P-72 clause 4): `<sys/types.h>`
  (`uid_t`/`gid_t`), `<unistd.h>` (`ssize_t`) and `<string.h>` (`strncmp`) reach this file through
  `"lisp.h"`/`"process.h"` in C, and the mirrors that carry the names are named where the C header
  is.

## C line ranges

The rows are contiguous and ascending; `docs/port-map.md` binds this ledger to `src/haiku.c` alone
(the reference has no `src/haiku.h`).

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block | dropped (P-86 clause 1) |
| 18-19 | `#include <config.h>` | no gate |
| 20-24 | `lisp.h`, `process.h`, `coding.h` | file gate |
| 25 | `#include <kernel/OS.h>` | file gate; the surface is the in-unit section (P-88 clause 1) |
| 26-29 | `<pwd.h>` (`getpwuid`), `<stdlib.h>` | file gate |
| 30-41 | `list_system_processes` | file gate |
| 43-120 | `system_process_attributes` (C 84/94 the two sample loops, C 89 the area walk) | file gate |
| 122-131 | `struct load_sample` and the four file-scope objects | file gate |
| 133-137 | `buf_next`/`buf_prev` (static) | file gate |
| 139-191 | `getavg` (static; C 175's `DBL_EPSILON`) | file gate |
| 193-214 | `sample_sys_load` (static) | file gate |
| 216-286 | `getloadavg` (C 236's backwards-clock prune, C 253 the sample store, C 283 the 0.09 floor) | file gate |

## Findings

1. **C's `sleep (0.05)` (C 67) is converted to 0 by whichever prototype is in scope.**  The file
   includes `<kernel/OS.h>` and reaches `<unistd.h>` through `lisp.h`; the port has exactly one
   `sleep` (`<unistd.h>`'s `unsigned int sleep (unsigned int)`, `lib/include/unistd.in.rs`), so it
   spells the conversion C performs - `sleep (0.05 as c_uint)`, i.e. `sleep (0)` - and records the
   reading here rather than inventing a Haiku prototype the machine cannot check.
2. **`countof (samples)` (C 136) is `sizeof (a) / sizeof (a[0])`**, i.e. 960; the port writes the
   number with the citation.
3. **C declares its locals uninitialized** (`team_info info;` C 31, `thread_info inf;`, `area_info
   area;`, `system_info sinfo;`): the port zeroes them for the E0381 reason `src/systhread.rs` 3.498
   records, and each first read follows the C call that fills it.

## Evidence

- frozen `build-tu src/haiku.c` -> **exit 0**, `build/femacs/haiku.o` **336 bytes** (the empty
  object the file gate leaves; the reference compiles nothing from this file either).
- **arm flip (status 3.499)**: `probe src/haiku.rs --crate-type=lib -C panic=abort
  --cfg 'feature="have_haiku"'` -> **exit 0, 0 errors, 14 warnings, an 8,837,304-byte rlib**.  The
  warnings are the house classes (`unnecessary parentheses` around C's parenthesised conditions,
  `unused doc comment` on the in-unit `extern` block); none is in the unit's statements.
- `python3 tools/extract-abi.py --check emacs/src/haiku.c src/haiku.rs` -> **OK, zero `-`
  rows** (after two `docs/ports/macro-body-notes.tsv` rows for `DBL_EPSILON`, see `## Deviations`).
- `check-mirror-dupes` **17/17/0** with no row naming this unit; `check-declarations` 10 sites /
  0 failures; `check-citations` 0 drift; `check-dead-file-coverage` binds this table with 0 failures.

## Deviations

1. **`DBL_EPSILON` is `f64::EPSILON`** - Rust has no such constant; the token is recorded in
   `docs/ports/macro-body-notes.tsv` (rows for `getavg` and `getloadloadavg`'s sibling) so the
   checker knows the omission is deliberate.
2. **`long double` is the port's `c_longdouble`**, aliased to `c_double` the way `src/editfns.rs`
   197 does (PORTING.md's `long double` row: never downgrade silently).
3. **The records carry the reference's read members only**, with C's other members named here and in
   `## Owed`; their offsets are therefore **not** the Haiku ones and are not asserted.
4. **C's implicit conversions are spelled**: `team_id`/`uid_t`/`gid_t`/`int64` widenings at `make_fixnum`,
   the `c_double` casts in the two `make_float` expressions, and the `0.05` conversion of `## Findings` 1.

## Owed

1. **A Haiku round**: with `<kernel/OS.h>` in hand, complete `team_info`, `thread_info`, `area_info`
   and `system_info` (the members this port omits - `debugger_nub_*`, `image_count`/`area_count` are
   carried only to keep `args`'s order), measure `B_PAGE_SIZE` per architecture, and confirm the
   `port_id`/`thread_state` spellings (`c_int` here).
2. **`getloadavg`'s collision**: `lib/getloadavg.rs` (gnulib's) and this unit both define
   `getloadavg`; each is behind its own gate, so no single translation unit sees both - a Haiku
   build must pick one, which the reference's own build does by not building gnulib's.
3. `docs/baseline/feature-coverage.tsv` needs no regeneration (this unit adds no feature name;
   `have_haiku` is registered).

## Not claimed

- **Zero run evidence**: nothing was linked, nothing ran; no Haiku process, team, area or load
  average was ever read.
- **No Haiku SDK was consulted**: every width, offset and record member below `## Decisions` is C's
  spelling, and the records' omitted members mean their offsets are knowingly not the Haiku ones.
- The frozen object is the 336-byte empty one, so it says nothing about the body.
