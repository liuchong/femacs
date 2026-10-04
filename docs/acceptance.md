# The acceptance baseline: the reference program the ladder compares against

`docs/port-plan.md` 7.1's ladder (L0 self-check expressions, L1-L3 `loadup`, L4-L5 the Lisp test
suite) is *differential*: every rung compares the port with the reference program, so the
reference program has to exist as a runnable artifact.  This file is the record of building it, and
of the two halves it has.

## What exists now (2026-10-04)

```sh
make -C build/emacs-ref/src temacs      # ~35 s; 74 objects were already there from earlier rounds
```

`build/emacs-ref/src/temacs` is built from the frozen configuration - `--without-all --with-x=no
--with-ns=no`, which is why the program's whole external library set is `-lncurses`
(`docs/ports/link.md`) and why no Objective-C source is compiled.  The artifact runs:

```
$ build/emacs-ref/src/temacs --version
GNU Emacs 32.0.50
```

and its `config.h` still agrees with the frozen baseline (`SYSTEM_TYPE "darwin"`,
`PACKAGE_STRING "GNU Emacs 32.0.50"`, `HAVE_NS`/`HAVE_NATIVE_COMP` absent), which is the property
the port's own measurement stands on - re-checked after a `config.status --recheck` ran.

## The whole baseline exists now (2026-10-04, later the same evening)

```sh
make -C build/emacs-ref -j3        # src -> lisp -> dump; the top-level driver
```

`build/emacs-ref/src/emacs` (the dumped program) and `build/emacs-ref/src/emacs.pdmp` (13 MB) are
built, and the rungs have their reference side:

```
$ build/emacs-ref/src/emacs --version
GNU Emacs 32.0.50
Development version fd7afced4036 on master branch; build date 2026-10-04.

$ build/emacs-ref/src/emacs -Q --batch --eval '(princ (+ 1 2))'
3
$ build/emacs-ref/src/emacs -Q --batch --eval '(princ (mapcar (lambda (x) (* x x)) (list 1 2 3)))'
(1 4 9)
```

The version string names the frozen revision itself (`fd7afced4036`), so the baseline and the
port's reference are the same commit.

### Two things the first attempt measured (recorded because they will recur)

* **Driving `lisp/Makefile` directly is not the same driver.**  `make -C build/emacs-ref/lisp
  EMACS=../src/bootstrap-emacs` (a copy of `temacs`, which is what Emacs's own bootstrap does) gets
  to `main-first`/`semantic` and fails with `Symbol's function definition is void:
  debug-early--handler`, because the load path it builds (`.../lisp/lisp`) is the one the top-level
  driver sets up and the `.el`-generating rules expect the dumped program.
* **The top-level driver first re-runs `config.status`,** which re-runs `configure`; that stops with
  *"source tree does not seem to have pre-built manuals in the 'info' directory. Please install a
  suitable version of makeinfo."* unless `$srcdir/info/emacs` exists (`emacs/configure` 14231-14235
  makes that the test for a release tarball versus a repository checkout).  A zero-byte
  `emacs/info/emacs` satisfies it and the build then finds the `makeinfo` in the configuration's own
  `MAKEINFO` variable (`makeinfo (GNU texinfo) 7.1`).

### The reference tree is left pristine, and the gate is what proves it

The build writes **git-ignored** outputs *into the reference tree* (1,878 files measured: 1,646
`.elc`, `info/*.info`, generated grammar `.el`, stamps).  The verbatim gate compares file *sets*
(`tools/check-verbatim.py`), so an over-broad cleanup is visible: `git -C emacs clean -fX` removed
those outputs **plus** ignored files that predated the build and that the port's verbatim copies
mirror (`etc/refcards/emacsver.tex`, `doc/emacs/emacsver.texi`, `doc/man/emacs.1`) - the gate went
from 0 to 3 failures, and re-copying in the right direction per path
(`emacs/lisp/leim/ja-dic/ja-dic.el{c,}` came the other way) put it back to `6 trees, 4609 files, 0
failures`.  Two consequences worth knowing:

* after building the baseline, clean up *only* what the build created, or expect to re-sync with
  the port's copies; `docs/status.md` 3.256 records the episode;
* `emacs/configure` and `emacs/aclocal.m4` are ignored generated files too, and the same cleanup
  removed them.  Nothing in the port's measurement path needs them (the frozen configuration is
  `docs/baseline/config/darwin-arm64.macros.tsv` plus `build/emacs-ref/**`), and regenerating them
  with a different `autoconf` would risk touching *tracked* files (`src/config.in`), so they are
  left absent on purpose.

## L0: the harness and its golden (2026-10-04/05)

```sh
python3 tools/acceptance-l0.py --golden build/emacs-ref/src/emacs   # record the golden
python3 tools/acceptance-l0.py --check <binary>                     # compare, byte for byte
```

Nineteen expressions, each run as `<binary> -Q --batch --eval <program>`, compared as the triple
`(exit code, stdout, stderr)`: arithmetic (including the bignum path), `prin1`/`princ`/`format`,
closures and `mapcar`, `condition-case`, `unwind-protect`, string mutation and `string-bytes`,
`sort`/`append`, hash tables, `read`/`prin1` round-trip, `char-to-string`, a `with-temp-buffer`
round trip, a `message` that goes to stderr, `error-message-string`, and `(kill-emacs 3)` so the
*exit code* is part of the comparison.

The golden (`docs/baseline/l0-reference.txt`) records the reference binary's path, its `--version`
first line and the first 16 hex digits of its sha256, so a golden whose reference has moved is
visible rather than silent; each stream sits between explicit `--- stdout`/`--- end stdout`
markers, which is what makes its bytes readable without guessing about the last newline.

**Readings (2026-10-05).**  The reference answers all nineteen (`19 expression(s), 0 failure(s)`,
a self-test of the harness against its own golden), and the negative control is the dump-less
`build/emacs-ref/src/temacs`: `19 expression(s), 19 failure(s)` - it prints the same "Lisp
directory … No such file or directory" warning this file records above, so the harness
discriminates.  The port's own binary does not link yet (`docs/ports/link.md`, reading five), so
the check side has nothing to run against; when it does, this is the first rung of the ladder and
it is one command.
