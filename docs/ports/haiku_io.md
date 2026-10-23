# emacs/src/haiku_io.c (217 lines)

> **Not compiled by any configuration the port builds.**  `build/emacs-ref/src/Makefile`'s platform
> object variables are empty for this target (`HAIKU_OBJ =`) and `docs/baseline/frozen-build-sources.txt`
> lists no such object.  PORTING.md P-86 governs the class - the body is **transcribed as it stands**
> (the previous revision was the P-08 note), the file gate `have_haiku` is carried on every item and on
> the include edges (P-86 ⑧) - and P-86 ②'s C line-range table is below.  Port:
> `src/haiku_io.rs`.

# ----------------------------------------------------------------------
## Gate mapping

The file has **no `#if` of its own**; `python3 tools/check-cond-eval.py src/haiku_io.c` reads
`0 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  The file gate is external (the Makefile's
`HAIKU_OBJ`), and the Haiku SDK's `<OS.h>` is not on this machine, so the five port calls and the two
values are use-site declarations with the standing `UNVERIFIED` mark (P-88 ①/②).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | licence and ownership block | dropped (0.1-19) |
| 19-31 | the include block: `config.h`, `<signal.h>`, `<stdio.h>`, `<pthread.h>`, `<unistd.h>`, `<OS.h>`, `"haiku_support.h"`, `"lisp.h"`, `"haikuterm.h"`, `"blockinput.h"` | `have_haiku` |
| 32-33 | `#define PORT_CAP 1200` | `have_haiku` |
| 34-46 | the three port objects (`port_application_to_emacs`, `port_popup_menu_to_emacs`, `port_emacs_to_session_manager`) | `have_haiku` |
| 47-54 | `void haiku_io_init (void)` | `have_haiku` |
| 55-116 | `static ssize_t haiku_len (enum haiku_event_type type)` - the twenty-seven arm size table | `have_haiku` |
| 117-135 | `void haiku_read_size (ssize_t *len, bool popup_menu_p)` | `have_haiku` |
| 136-152 | `int haiku_read (enum haiku_event_type *type, void *buf, ssize_t len)` | `have_haiku` |
| 153-181 | `int haiku_read_with_timeout (…)` | `have_haiku` |
| 182-195 | `int haiku_write (enum haiku_event_type type, void *buf)` | `have_haiku` |
| 196-209 | `int haiku_write_without_signal (…)` | `have_haiku` |
| 210-217 | `void haiku_io_init_in_app_thread (void)` | `have_haiku` |

## Evidence

* Frozen: `cargo run -q -p xtask -- build-tu src/haiku_io.c` → exit 0, object **336 bytes** (the empty
  object the file gate must produce).
* Flipped arm (`--cfg 'feature="have_haiku"'`): **7 errors, and that is 5 real ones** - four
  `coding_system` and one `Emacs_Cursor` duplicate, every one of them **inside include mirrors**
  (`_root.rs` anchors: 0, the `Emacs_Cursor` duplicate being the one 3.431 recorded).  The reading went
  254-class -> 23 -> 12 -> 7 as the unit's own text was fixed (a `pthread.rs` include that does not
  exist, the `c_*` names that the `lisp.rs` mirror supplies, `uint32` -> `c_uint`, and `kill`/`SIGPOLL`
  declared at the use site).
* `python3 tools/extract-abi.py --check emacs/src/haiku_io.c src/haiku_io.rs` → one token row
  (`function haiku_len does not mention case, switch`, exempted in `macro-body-notes.tsv`) plus the line
  drift declared in `line-drift-tolerance.tsv`.
* `python3 tools/check-cond-eval.py src/haiku_io.c` → `0 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.

## Deviations

1. **C's `switch` in `haiku_len`** is the port's `match` (the twenty-seven arms, with C's implicit
   fall-through to `emacs_abort ()` spelled as the `_` arm); the `case`/`switch` tokens have no site
   (the `macro-body-notes.tsv` row).
2. **The `c_*` names come from `include/lisp.rs`**, which this unit includes; a unit that includes that
   mirror must not import them again (`E0252` in one module, measured).
3. **Use-site declarations** (P-88 ①): the five `<OS.h>` calls, `B_OK`/`B_TIMEOUT`, `sigfillset`,
   `kill` and `SIGPOLL`; none of them is in a mirror, and `SIGPOLL` is `UNVERIFIED` because this
   host's `<signal.h>` does not define it (the clang probe fails).
4. **C's uninitialised locals** are written with C's own zero values (PORTING.md 0.1-13), and the
   `match` is a `return match …` so every arm keeps C's `return` shape.

## Owed

* The arm's five real errors are mirror-side and named: `coding_system` (four sites, a mirror naming a
  type its includers may not have - P-87 ③) and the `Emacs_Cursor` duplicate 3.431 recorded.  They are
  the mirror round's, not this file's.
* No runtime evidence: no `create_port`, no `read_port`/`write_port`, no message ever sent or received
  (see `docs/status.md` 3.453).

### The `coding_system` diagnosis (2026-10-05, status 3.454)

The arm reading above names five real mirror errors; four of them are one defect and it is worth
stating exactly, because it blocks every platform arm that includes `src/termhooks.h`:

* `src/include/termhooks.rs` **has no `include!` lines at all**, and names `*mut coding_system`
  four times (215/216, the two `struct terminal` fields, and 354/357, `TERMINAL_TERMINAL_CODING` /
  `TERMINAL_KEYBOARD_CODING`).  The type is defined in `src/include/coding.rs` 133, which is
  *not* in that mirror's graph, so any unit that includes `termhooks.rs` without `coding.rs` fails on
  it - `haiku_io.rs` is one, and P-86's own notes list these four errors as the blocker for the
  `have_android` flip arms (status 3.413/3.431).
* The sanctioned remedy is P-87 ③'s: a mirror must not name a type its includers may not have, and the
  two worked answers are "spell the underlying type" (`window_part` -> `c_int`) or P-72 rule 3's
  `*mut c_void` - the shape `src/include/window.rs` 104 already carries for `current_matrix`, and the
  `rif` field two lines below this one carries for `struct redisplay_interface *`.
* The cost is churn in the three units that *do* have the type and use the accessors today -
  `src/coding.rs` (6 sites), `src/composite.rs` (2) and `src/keyboard.rs` (2) -
  each needing a cast at its use site; that is why this is a mirror round of its own rather than a
  line in this file's landing, and why it is recorded here with the counts instead of being attempted
  half-way.
