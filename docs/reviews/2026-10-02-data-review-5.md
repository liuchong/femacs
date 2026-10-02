# Adversarial review: `src/data.c` segment V (data.c 1678-1960) and the `local_types` fix

Scope: `set_internal` (the biggest function in the file), `harmonize_variable_watchers`,
`notify_variable_watchers` and the three watcher subrs (`add-variable-watcher`,
`remove-variable-watcher`, `get-variable-watchers`), plus the four `lisp.rs` prototypes
they need (`map_obarray`, `funcall_subr`, `let_shadows_buffer_binding_p`,
`set_default_internal`) and the two `macro-body-notes.tsv` exemptions the token gate asked
for.

Method: reviewer A attacked the code (ordered token alignment with `difflib`, short-token
occurrence counts, per-arm mapping, 24 argument orders, byte-exact docstrings re-derived
from `make-docfile`), reviewer B attacked the gate/ledger/dead-text surface with five
mutation probes in `/tmp`.  Both read-only, no builds.

## Reviewer A: no semantic defect

Verified: both switches (`SYMBOL_NOWRITE`'s if/else leaves the switch on both paths, so
`match` needs no fall-through; the `start:`/`goto start` loop re-evaluates the scrutinee
each round because `sym` is reassigned before `continue`), the bit-mask writes against
`lisp.h`'s `local_if_set:1; found:1` and `lisp.rs`'s `BLV_*`, `bindflag != 0` against
`SET_INTERNAL_SET == 0`, the four parameter rebindings (all at function-body scope - the
defect class of the previous round does not recur here), the variadic `error` calls
(format strings byte-identical), the `SYMBOL_FORWARDED` branch (`XBUFFER_OFFSET`'s `u16` is
the C field's own type, so the `as c_int` widening is lossless), the watcher trio
(`map_obarray`'s callback order checked against `lread.c`'s call site, `countof!` = 4,
`calln!` = `Ffuncall`, the `restore_symbol_trapped_write` callback ABI that this segment
had to change), and all three docstrings byte for byte.

## Reviewer B: one gate weakening, three ledger items

| # | Finding | Disposition |
| --- | --- | --- |
| 1 | **`extract-abi.py`'s `local_types` regex crossed lines** (B-D1, medium-high): a `}` at one line followed by `return;`/`break;` on the next registered those words as "a type this file declares", silently exempting `break` from *every* body in the file (66 checked C files affected).  Its mutation deleting a `return` produced byte-identical gate output. | Fixed: `[ \t]*` instead of `\s*`.  It surfaced exactly one hidden token, `set_internal`'s `break`, which is now in `macro-body-notes.tsv` with its reason; `--all` stayed 17 done / 10 WIP / 0 failing. |
| 2 | The `set_internal` notes row cited `lisp.rs 1965-1966` for the `BLV_*` flags; they are at 1970-1971 (B-D2). | Corrected, and the row now also says that C's `1` argument is carried by the Rust `true` on that call (B-D4). |
| 3 | The ledger's "Ported so far" numbers were a round stale (55/29, Rust 1362, 69.0%) and the segment's "no preprocessor conditions" finding was unrecorded (B-D3). | Both updated (52 subrs / 26 functions / Rust 1637 / 62.8%, plus the explicit bullet). |
| 4 | Three more instances of the "token set intact, semantics changed" class: `bits \| BLV_FOUND` -> `\| BLV_LOCAL_IF_SET`, `& BLV_LOCAL_IF_SET` -> `& BLV_FOUND`, and deleting the bit write entirely are all silent (B-D5). | Recorded in P-78's residual (4), whose root cause is that the notes mechanism can only *remove* requirements, never require the port's replacement spellings. |

Reviewer B also confirmed: the range has no preprocessor conditions (line scan and the
frozen evaluator agree, against the file's 74 dead lines elsewhere); the four prototypes
are byte-equivalent to lisp.h 4748/4800/4864/4208-4209 and all type-check (the probe went
from 89 to 93 declarations, 0 unmapped); the ledger is 1:1 with the implementation (98
ticked, the four dead subrs still unticked); `--check`'s 264 items are the expected
categories; and it wrote four behavioural probes (in `/tmp`, not run) for the paths no
token rule can see.

## Evidence

- `xtask build-tu src/data.c` -> built (this is the first use of `XSETBUFFER!` in the
  tree, so the macro path is now exercised), and `--cfg enable_checking` -> 0 errors
  (190,744-byte object).
- `extract-abi.py --check data.c` -> 264 items, all "missing" except the partial file's
  62.8% drift; `--all` -> 17 done / 10 WIP / 0 failing (re-run after the tool fix).
- `xtask gate-fast` -> 0; `xtask gate-probes` -> 0 (lisp-abi 93 declarations).

## Residual, recorded rather than fixed

The mirror's `Lisp_Buffer_Local_Value.bits: u32` covers C's one-byte `bool_bf` pair, so the
mask writes read and rewrite the surrounding padding bytes (the pre-existing convention,
same as `set_blv_found` and `swap_in_symval_forwarding`); `XBUFFER_OFFSET`'s mirror returns
`u16` where C's inline widens to `int` (the field is `uint16_t`, so no value is lost, and
inline return types are outside the ABI probe).
