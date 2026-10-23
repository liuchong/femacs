# emacs/exec/exec1.c (94 lines)

> **Not compiled by any configuration the port builds.**  `docs/baseline/frozen-build-sources.txt`
> lists no `exec/` object: the loader belongs to the unexec dump path and this build is
> `HAVE_PDUMPER=1`.  PORTING.md P-86 governs this class - the body is **transcribed as it stands**
> (the previous revision was the three-line P-08 note) - and P-86 ②'s C line-range table is below.
> Port: `exec/exec1.rs` (119 lines), first line
> `// femacs: port of exec/exec1.c @ fd7afced403.`

# ----------------------------------------------------------------------
## Gate mapping

The file has **no `#if` of its own**; `python3 tools/check-cond-eval.py exec/exec1.c` reads
`0 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  The configuration-shaped surfaces are declared
at their use sites (PORTING.md P-88 ①, and P-90's seventh half records that a system macro with
several consumers stays a use-site declaration in each unit): `<sys/wait.h>`'s
`WIFEXITED`/`WEXITSTATUS`/`WTERMSIG`, `<unistd.h>`'s `fork` and `setpgid`, `<stdlib.h>`'s `exit`;
`getpid` and `perror` are taken from the mirrors that carry them (`lib/include/unistd.in.rs` 63,
`lib/include/stdio.rs` 63), and `raise` from the `<signal.h>` mirror.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-19 | licence and ownership block | dropped (0.1-19) |
| 20-27 | the include block: `config.h`, `<unistd.h>`, `<stdlib.h>`, `<stdio.h>`, `<sys/wait.h>`, `"exec.h"` | — |
| 28-38 | the "exec1 is a program which takes another program and its arguments …" commentary and `main`'s banner | — |
| 39-94 | `int main (int argc, char **argv)` | — |

## Evidence

* `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-exec4/cargo cargo run -q -p xtask -- build-tu exec/exec1.c`
  → `built exec/exec1.rs -> ../build/femacs/exec1.o`, **exit 0**, object **3,616 bytes**.
* `python3 tools/extract-abi.py --check emacs/exec/exec1.c exec/exec1.rs` → **no named rows**;
  the only line is the 26.6% drift, declared in `line-drift-tolerance.tsv`.
* `python3 tools/check-cond-eval.py exec/exec1.c` → `0 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.

## Deviations

1. **C's uninitialised locals** (C 41-43: `pid_t pid, pid1; int wstatus;`) are written with C's own
   zero values (PORTING.md 0.1-13).
2. **C's `if (!pid)`** is the port's `if pid == 0` (PORTING.md 2.2).
3. **`while (true)`** is the port's `loop`.
4. **`extern char **environ;`** (C 42) has no item of its own: the `<unistd.h>` mirror declares the
   object (94).
5. **`setpgid`'s failure path** keeps C's `perror ("setpgid")` and C's ignored return value: the
   `!= 0` test is the truthiness C spells implicitly.

## Owed

* `fork`, `setpgid` and `exit` are the "no mirror carries it yet" class (the
  `lib-src/emacsclient.rs` 292-295 note).  Moving them into `lib/include/{unistd.in,stdlib}.rs` is a
  move-back round across every unit that declares them (26 declarations in a dozen files, measured
  2026-10-05), not this file's job; P-90's seventh half is what keeps the current shape lawful.
* No runtime evidence of any kind: the object was never linked and never run (see `docs/status.md`
  3.449).
