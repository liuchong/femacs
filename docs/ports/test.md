# emacs/exec/test.c (105 lines)

> **Not compiled by any configuration the port builds.**  `docs/baseline/frozen-build-sources.txt`
> lists no `exec/` object: the loader belongs to the unexec dump path and this build is
> `HAVE_PDUMPER=1`.  PORTING.md P-86 governs this class - the body is **transcribed as it stands**
> (the previous revision was the three-line P-08 note) - and P-86 ②'s C line-range table is below.
> Port: `femacs/exec/test.rs` (129 lines), first line
> `// femacs: port of exec/test.c @ fd7afced403.`

# ----------------------------------------------------------------------
## Gate mapping

The file has **no `#if` of its own**; `python3 tools/check-cond-eval.py exec/test.c` reads
`0 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  The two configuration-shaped surfaces are
declared at their use sites instead (PORTING.md P-88 ①): `<sys/wait.h>`'s four `W*` macros (no
mirror in this tree; the `src/process.rs` 1262-1283 shape) and `<unistd.h>`/`<stdlib.h>`'s `fork`
and `exit` (the `lib-src/emacsclient.rs` 292-295 note records the same gap).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | licence and ownership block | dropped (0.1-19) |
| 20-31 | the include block: `config.h`, `<signal.h>`, `<stdio.h>`, `<unistd.h>`, `<stdlib.h>`, `<errno.h>`, `<string.h>`, `<sys/wait.h>`, `"exec.h"` | — |
| 32-41 | `static void print_usage (void)` and its message | — |
| 42-44 | `extern char **environ;` | — |
| 45-49 | the "This program uses libexec …" banner | — |
| 50-105 | `int main (int argc, char **argv)` | — |

## Evidence

* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-exec2/cargo cargo run -q -p xtask -- build-tu exec/test.c`
  → `built exec/test.rs -> ../build/femacs/test.o`, **exit 0**, object **6,072 bytes**.
* `nm -g build/femacs/test.o` → `_main` defined; `_exec_init`, `_tracing_execve`, `_after_fork`,
  `_exec_waitpid`, `_environ`, `___error`, `___stderrp`, `_fork`, `_exit`, `_sigemptyset`,
  `_sigaddset`, `_sigprocmask` undefined (the mirrors' and C's own declarations).
* `python3 tools/extract-abi.py --check emacs/exec/test.c femacs/exec/test.rs` → one token row
  (`function main does not mention true`, exempted in `macro-body-notes.tsv`) and the line drift
  (22.9%, declared in `line-drift-tolerance.tsv`).

## Deviations

1. **C's uninitialised locals** (C 52: `pid_t pid, child; int sig; sigset_t sigset;`) are written
   with C's own zero values, because Rust has no partial initialisation (PORTING.md 0.1-13).
2. **C's `if (pid = fork ())`** is the port's `pid = fork (); if pid == 0` - Rust has no
   assignment-expression in a condition (PORTING.md 2.2).
3. **C's two adjacent string literals** in `fprintf` are one Rust literal (the preprocessor
   concatenates them in C; the `lib-src/etags.rs` banner precedent).
4. **`while (true)`** is the port's `loop` (the `true` token has no site; the `main` row of
   `macro-body-notes.tsv`).
5. **The C names `environ`, `fork`, `exit` and the four `W*` macros** are declared at the use site
   or taken from the `<unistd.h>` mirror; see Gate mapping.

## Owed

* `femacs/exec/include/exec.rs`'s two nested `include!` paths were **one level too deep**
  (`../../../lib/include/…`, i.e. the repository root) and were corrected to `../../lib/include/…`
  in this round: nothing had included that mirror before, so no gate read the broken edge (the
  P-86 ⑦/P-89 class).  `USER_WORD` - the mirror's own file note said it is `uintptr_t`, but no
  declaration existed anywhere in the tree - is now declared there as `usize`.
* No runtime evidence of any kind: this program was never linked and never run (see the ledger's
  boundary in `docs/status.md` 3.446).
