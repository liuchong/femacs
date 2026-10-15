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

The golden (`docs/baseline/l0-reference.txt`) records the reference binary's path, the version
number out of its `--version` first line and the first 16 hex digits of its sha256, so a golden
whose reference has moved is visible rather than silent (the name that line carries is quoted
above as evidence about the reference; `PORTING.md` 0.1-19 keeps it out of files we write, and
`tools/check-declarations.py` reads these two headers by name so it cannot come back, 3.413);
each stream sits between explicit `--- stdout`/`--- end stdout` markers, which is what makes its
bytes readable without guessing about the last newline.

**Readings (2026-10-05).**  The reference answers all nineteen (`19 expression(s), 0 failure(s)`,
a self-test of the harness against its own golden), and the negative control is the dump-less
`build/emacs-ref/src/temacs`: `19 expression(s), 19 failure(s)` - it prints the same "Lisp
directory … No such file or directory" warning this file records above, so the harness
discriminates.  The port's own binary does not link yet (`docs/ports/link.md`, reading five), so
the check side has nothing to run against; when it does, this is the first rung of the ladder and
it is one command.

## L1: the compile half, its harness, and the two load-path defects it had (2026-10-05)

```sh
python3 tools/acceptance-l1.py --golden build/emacs-ref/src/emacs
python3 tools/acceptance-l1.py --check <binary>
```

The plan's L1 is "every `lisp/emacs-lisp/*.elc` loads, with no error and no warning difference" -
a *load* rung whose inputs are the `.elc`s the build's own Lisp pass produces.  This tool measures
the step before that: it copies `lisp/emacs-lisp` (111 files; `--tree` picks the side, the port's
`femacs/lisp` by default) into `build/lisp-l1`, byte-compiles each file with `<binary> -Q --batch -L
<scratch> -L <tree> -L <the tree's subdirectories in the dumped program's order> -f
batch-byte-compile <file>`, and compares `(exit code, diagnostics)` per file with the golden.

**Reading, second attempt (2026-10-05, after the tree repair below).**  With the reference tree built
- its generated `subdirs.el`/`*loaddefs.el`/`.elc`s present - and the load path named the way C's own
build names it, the reference answers its own golden byte for byte twice in a row (`111 file(s), 0
failure(s)`; the golden itself records `0 non-zero exit(s), 0 file(s) with diagnostics`), and a
deliberately broken copy of one file fails exactly one record (`111 file(s), 1 failure(s)`, the copy
made with `cp -Rp` so its timestamps add no "newer than byte-compiled file" warnings of their own).

**What the first attempt measured, and what it was really measuring.**  The first golden recorded 4
non-zero exits - `bytecomp.el`, `checkdoc.el`, `eieio-opt.el`, `eieio-speedbar.el` - and they looked
like properties of those files.  They were properties of the *harness*: its `-L` list held the
scratch directory plus the tree's subdirectories, **without the tree's own root**, sorted rather than
in the order the dumped program searches.  Under that list, measured with `locate-library`:

```
require 'compile   -> emacs/lisp/cedet/srecode/compile.elc     (provides 'srecode-compile, not 'compile)
require 'speedbar  -> emacs/lisp/cedet/ede/speedbar.elc
require 'dired     -> emacs/lisp/cedet/ede/dired.elc
```

which is where the three `Recursive 'require'` errors and the one `failed to provide feature
'compile'` came from - a shadowing harness, not a defect in C or in the tree.  With the root first
and the subdirectories in the dumped program's own order (`vc use-package url textmodes progmodes …
- measured, and `progmodes` ahead of `cedet` is the whole difference), the same three names resolve
to `emacs/lisp/progmodes/compile.elc`, `emacs/lisp/speedbar.elc` and `emacs/lisp/dired.elc`, and all
111 files compile clean.

Two consequences for the ladder, both worth writing down before someone "fixes" it:

* the rung compiles the tree's own `.el` sources, but the closure they `require` comes from the
  *built* tree around them: the `*loaddefs.el`, `subdirs.el` and `.elc`s C's driver writes
  (`build/emacs-ref/lisp/Makefile` 69-71 finds them and `gen-lisp` produces them).  The reference
  side has them because the baseline was built; the port side needs its own build pass before the
  same tool can be pointed at it - which is also why the plan puts `loaddefs.elc` in L2 and the full
  `loadup` set in L3;
* the comparison is **differential**: the port has to reproduce the reference's readings file for
  file, and the golden is the arbiter.

The invocation details that took measuring are recorded in the tool: the scratch directory is fixed
(`build/lisp-l1`) so a diagnostic that mentions a path mentions the same one on both sides, and the
`-L` list is the scratch, then the tree's root, then its subdirectories in the order the measured
binary searches them.  That order is asked of the binary itself (`dump_lisp_dirs()`), because `-L`
directories are searched in *command-line order*, ahead of the path the dump carries - the other half
of the shadowing above.

## The reference build's generated Lisp files, and the repair (2026-10-05)

The baseline (above) was built by C's own driver, which writes **generated** Lisp files *into the
reference tree* - `subdirs.el` in every directory, the `*loaddefs.el` files, the `.elc`s - and the
dumped program **needs** them: they are what puts the subdirectories on `load-path`
(`subdirs.el`) and what satisfies a `require` of anything the dump does not preload.  The
2026-10-04 cleanup that removed the build's ignored outputs (`docs/status.md` 3.256) took these with
it, and 3.266 measured what that costs:

```
$ emacs -Q --batch -L <tree>            -f batch-byte-compile <abs>/macroexp.el
Error: file-missing ("Cannot open load file" "No such file or directory" "bytecomp")
$ emacs -Q --batch -L <tree> -L <tree>/emacs-lisp -f batch-byte-compile <abs>/macroexp.el
Error: error ("Eager macro-expansion failure: … "compile"")        # compile.el lives in progmodes/
$ emacs -Q --batch -L <every subdirectory> -f batch-byte-compile <abs>/macroexp.el
Error: error ("Eager macro-expansion failure: … "srecode/loaddefs"")   # a generated file
```

So the *tracked* reference tree is untouched (`git -C emacs status --short` empty, and
`tools/check-verbatim.py` reads `6 trees, 4609 files, 0 failures`; 3.267 then stopped the *port*
from carrying five generated files and the gate asks each side's repository, so the reading is
`6 trees, 4604 files` now) and the port is unaffected - but
the *acceptance* side is: L1's compile half, and anything that loads Lisp beyond the dump, cannot run
until those files exist again.

**The repair, and its two blockers (both measured).**  C's top-level driver is what produced them, so
the repair is to run it again - and it starts with `./config.status --recheck`, which re-runs
`../../emacs/configure`:

1. `configure` is one of the ignored files the same cleanup removed.  Regenerating it needs the full
   `autoreconf` with this tree's `m4/` directory: `autoconf` alone stops with
   `configure.ac:2396: error: undefined or overquoted macro: m4_default` and
   `configure.ac:6405: error: undefined or overquoted macro: AM_LANGINFO_CODESET`.  `autoreconf`
   also rewrites *tracked* files (`src/config.in`, the `Makefile.in`s), so the repair has to check
   `git -C emacs status` afterwards and restore any of them.
2. Driving `lisp/Makefile` by hand is not equivalent: `make -C build/emacs-ref/lisp all
   EMACS=../src/emacs` stops at `semantic`
   (`make[1]: *** [../../../../emacs/lisp/cedet/semantic/bovine/make-by.el] Error 255`), because the
   grammar `.el` generation is a bootstrap step the top-level driver orders differently (it runs the
   first passes with `temacs`/`bootstrap-emacs`).

The safe sequence, then: `autoreconf` in the reference tree, restore any tracked file it changed, put
a zero-byte `emacs/info/emacs` back (the `configure` release-versus-checkout test), `touch
build/emacs-ref/config.status` so no recheck runs, and `make -C build/emacs-ref -j3` - the driver
that built the baseline in the first place.

**The repair, run (2026-10-05).**  That sequence was executed and works, with one addition the
machine forced:

```sh
cd emacs && autoreconf -fi                 # configure back; git status stays empty afterwards
touch emacs/info/emacs                     # the release-versus-checkout test (0 bytes is enough)
touch build/emacs-ref/config.status        # so no --recheck runs
cd build/emacs-ref && PATH=/tmp/fakebin:$PATH make -j3 bootstrap      # exit 0
```

`makeinfo` is not installed here and `configure`'s own `MAKEINFO` fallback does not run either, so a
three-line stub was placed *first* on `PATH` for that one command (`/tmp/fakebin/makeinfo`:
`--version` prints a GNU texinfo version line, everything else exits 0).  It is a lie about a tool,
kept only for this build and recorded here; `make all` - the manuals in `info/`, and the `leim` and
`admin` products - was **not** run, because with no real `makeinfo` it would fabricate empty manuals
instead of failing loudly, and nothing in the ladder needs them yet.  What the ladder did need came
back: `lisp/subdirs.el`, the `*loaddefs.el`s, **1645 `.elc`**, and `(require 'compile)` succeeds.
L0 still reads `19 expression(s), 0 failure(s)`, `check-verbatim` reads `6 trees, 4604 files, 0
failures` (plus the 24 sanctioned identity lines of §5), and L1's compile half has the reading in its
own section above.

The five steps that repair has to reproduce are written down here rather than in code, because a tool
that cannot run is not evidence: `update-subdirs` per directory (which writes `subdirs.el` **in that
directory**, so it must be run with that directory as the working directory), `COMPILE_FIRST`
(macroexp, cconv, byte-opt, bytecomp, loaddefs-gen, radix-tree), `MAIN_FIRST`, the autoloads pass
(`-l emacs-lisp/loaddefs-gen.elc -f loaddefs-generate--emacs-batch <subdirs>`), and the rest in
compile passes that repeat until they stop making progress.

## 完成定义第 5 条（`lisp/`/`test/` 逐字节一致）：判据与实测（2026-10-05，第十一半后复读 10-06）

`docs/port-plan.md` §8 第 5 条要的是"与参照逐字节一致（`diff -r` 无差异）"。**本轮把它量了一遍，
并把"字面 `diff -r` 与真正的判据差在哪里"钉住**——因为参照检出**在本地被构建过**（冻结基线就是
在 `emacs/` 里 `make` 出来的，见本文上一节），构建产物留在树里，而它们**不是参照的内容**。

实测（`diff -r emacs/<tree> femacs/<tree>`，六棵树）：

| 树 | 字面 `diff -r` | 参照侧多出的东西 | 参照 `.gitignore` 的规则 |
| --- | --- | --- | --- |
| `lisp` | 1727 条 "Only in emacs/lisp" | 全部是 `*.elc`（就地字节编译产物） | `.gitignore:180` `*.elc` |
| `etc` | 132 条 | 全部是 `etc/charsets/*.map` | `.gitignore:265` `etc/charsets/*.map` |
| `doc` | 4 条 | `doc/emacs/emacsver.texi`、`doc/man/emacs.1` 等 | `.gitignore:314`/`:315` |
| `admin` | 4 条 | `*.stamp`、`admin/unidata/*.elc` | `.gitignore:171` `*.stamp`、`:180` `*.elc` |
| `test` | **无差异** | — | — |
| `leim` | 3 条 "Files … differ"（第十一半的 3 处声明行，见下） | — | — |

**被参照自己的 `.gitignore` 忽略的文件不是参照内容**（`git -C emacs check-ignore -v` 逐条给出上面的规则行），
所以第 5 条的判据是**已跟踪内容**，而且它已经是绿的：

```
$ for t in lisp test etc doc leim admin; do echo "$t $(git -C emacs ls-files $t | wc -l) $(git ls-files femacs/$t | wc -l)"; done
lisp 1627/1627   test 1555/1555   etc 886/886   doc 255/255   leim 28/28   admin 253/253

$ python3 tools/check-verbatim.py
OK      lisp: 1627 files, byte-identical, 6472 sanctioned line(s)
OK      test: 1555 files, byte-identical
OK      etc: 886 files, byte-identical, 473 sanctioned line(s)
OK      doc: 255 files, byte-identical, 953 sanctioned line(s)
OK      leim: 28 files, byte-identical, 15 sanctioned line(s)
OK      admin: 253 files, byte-identical, 260 sanctioned line(s)
6 trees, 4604 files, 8173 sanctioned line(s), 0 failures
```

即：六棵树的**已跟踪文件数逐个相等**，`check-verbatim`（按树、逐文件比对）读 **0 failures**。
**8,173 行例外**（`docs/baseline/verbatim-substitutions.tsv` 逐行登记，每行一条理由；这张表随第十～第三十半
逐面增长，下面先点名最早的几批）：其中 24 行是
`lisp/` 里被程序**打印出来**（或以帧/splash 自称、或写进它生成的文件）的产品名字符串（PORTING.md 0.1-19 的
第十半，status 3.619）；另外 1,772 行是照抄树里**文件自称属于参照产品**的那句话
（`This file is part of GNU Emacs.` -> `This file is part of femacs.`，只换产品名那一个 token）、
含三个**生成器**（它们把这句话写进生成的文件）与八个**缓冲区名**站点（`lisp/startup.el`、
`doc/emacs/entering.texi`）——第十一半（status 3.632，2026-10-06）；**19 行**是**同一句话的其它拼法**——少了句点的两行、`These files are part of …`、`This version of the tutorial is a part of …` 与十种语言的同一句
（第十二半，status 3.641，2026-10-06）。**两处按判据不动**：`test/`
（goal 文本禁止改测试文件；它的 647 个文件仍逐字节相同）与 `ChangeLog*`（把这句话当历史引用）；
版权行与许可正文三句同样不动（第十半 ⑥⑶ 的理由不变）。此后每一半把它的那批行登记进来：许可三句的**第二十半**
（status 3.765，`femacs is free software` 一族）、平台面与工具面的**第二十四/二十五半**、安装登记面上的**第二十七/二十八半**（AppStream 名、`.TH` 第四栏与 `NAME` 段、手册页与 metainfo 的 id）、
**第二十九半**（status 3.974，菜单标签与桌面/服务显示名）与**第三十半**（status 4.008，这棵树自己的手册名——
两本手册的标题、116 行成员句与指向它们的行，963 行 / 207 个文件），本表当前 **8,173** 行。
除此之外仍是逐字节相同：一行要不同就必须在表里有行，有行就必被读。
第十一半之后 `diff -r` 还会在 `lisp`/`etc`/`doc`/`admin` 上打印一批 `Files … differ`，就是这 8,173 行；
`test` 仍是**无差异**。
**注意**：`emacs/` 是**外部只读检出，不在本仓库的索引里**（`git ls-files emacs/lisp` = 0），
所以这条判据只能在有该检出的机器上跑（`docs/port-plan.md` §10 已记 CI 的这一限制）。

**未声称**：这只是"复制"这一条的读数；§8 的其余各条（清单/layout/符号/subr、L0-L5、双向互换与
生成物、平台冒烟、性能）**一条都还没点亮**。

### 参照树比 `femacs/` 多的 1,867 个文件，全部是**构建产物**（2026-10-05 现测，status 3.583）

上面那条"逐文件 `cmp`"只覆盖**已复制的** 4,604 个文件。参照树里另外还有 **1,867** 个文件在 `femacs/` 里没有，
逐条对过参照检出自己的索引（`git -C emacs ls-files`）：

| 类 | 数量 | 例 |
| --- | ---: | --- |
| `.elc`（字节编译产物） | 1,645 | `lisp/abbrev.elc`、`admin/unidata/unidata-gen.elc` |
| 生成的 `.el` | 82 | `lisp/loaddefs.el`、`lisp/subdirs.el`、`lisp/international/uni-*.el`、`lisp/leim/**`、`lisp/cedet/**/*-by.el`/`*-wy.el`、各 `*-loaddefs.el` |
| `etc/charsets/*.map` + 1 `.tex` | 132 | `etc/charsets/8859-10.map` |
| `doc/`（生成的 texi/man） | 4 | `doc/emacs/emacsver.texi`、`doc/misc/org.texi`、`doc/misc/modus-themes.texi`、`doc/man/emacs.1` |
| `admin/`（产物与 stamp） | 4 | `admin/charsets/charsets.stamp`、`admin/unidata/uvs.elc`、`admin/charsets/jisx2131-filter` |

**实测判据**：这 1,867 个路径在参照检出里**全部未被 git 跟踪**（`git -C emacs ls-files` 命中 0 个），
反过来**没有任何一个被跟踪的源文件缺失**。⇒ §8 那条 `diff -r emacs/lisp femacs/lisp 无差异` 的**复制那一半已经做完**；
`diff -r` 现在仍显示出来的，恰好就是**构建输出**，属于同一条里的"**生成物逐字节一致**"（只能等移植端自己的生成步骤
——L3 的 `loadup`/`dump` 与 `admin/unidata`、`admin/charsets` —— 跑起来之后才可能出现）。
`femacs/lisp` 里**没有任何手写或被改过的 `.el`/`.elc`**。

**未声称**：这是"复制"与"参照树多出来的到底是什么"的读数，**不是**生成物一致性的读数——生成物一致性今天**一条都没有**。


## L0's second golden: the bare program (2026-10-16, status 4.371)

The ladder's L0-L3 are about the **bare** program (`temacs` loading `loadup.el` from source: L1
loads `lisp/emacs-lisp/*.elc`, L3 is the full `loadup.el`), and a dumped binary is silent on stderr
where the bare one prints its load trace - so the two flavours cannot share one golden.  The
harness now takes `--golden-path`, and the bare program's golden is recorded beside the dumped
one:

```sh
EMACSLOADPATH=$PWD/emacs/lisp python3 tools/acceptance-l0.py --golden \
    build/emacs-ref/src/temacs --golden-path docs/baseline/l0-reference-temacs.txt
EMACSLOADPATH=$PWD/emacs/lisp python3 tools/acceptance-l0.py --check \
    build/port/src/temacs --golden-path docs/baseline/l0-reference-temacs.txt
```

The golden records the `EMACSLOADPATH` it was written under (`# EMACSLOADPATH:`), because the
expressions' stderr *is* the load trace and therefore depends on which Lisp tree the program
walks.  `build/emacs-ref/src/temacs` at `fd7afced403`'s frozen configuration has
sha256[0:16] `a959f1d7a8bb4e11`.

**Reading (2026-10-16)**: the port's binary - `build/port/src/temacs`, linked by `xtask gate-load`
from the port's objects alone - prints `19 expression(s), 0 failure(s)` against that golden.  The
dumped golden (`docs/baseline/l0-reference.txt`) is unchanged and still has no check side: a
dumped port binary needs L3's `dump` step, which needs the generated Lisp (next section).


## 生成物（完成定义第 4 条）：端口自己产出的第一批读数（2026-10-16）

`femacs/lisp/` 是参照 `lisp/` 的逐字副本，因此参照构建**生成**的那些文件在端口树里没有：`lisp/`
下 **82 个生成的 `.el`**（`international/uni-*.el`、`charscript.el`、`charprop.el`、
`*-loaddefs.el`、`leim/quail/*.el`、`cedet` 的 `*-by/-wy.el`、`subdirs.el` 等）与 **1,645 个
`.elc`**。它们由参照构建自己的配方产出，这些配方按目标写进**源码树**（`.gitignore` 里对它们有
规则）；端口用同一批工具、同一批输入、同一份配方，写进 `femacs/` 的对应路径。

**AWK 生成的两族（无 emacs 参与，gawk = 冻结构建的 `AWK`）**

```sh
cd emacs/admin/unidata && gawk -f blocks.awk Blocks.txt emoji-data.txt \
    > <port>/femacs/lisp/international/charscript.el
cd emacs/admin/unidata && gawk -f emoji-zwj.awk emoji-zwj-sequences.txt emoji-sequences.txt \
    emoji-variation-sequences.txt emoji-test.txt > <port>/femacs/lisp/international/emoji-zwj.el
cd emacs/admin/charsets && gawk -f cp51932.awk < ../../../emacs/etc/charsets/CP932-2BYTE.map \
    > <port>/femacs/lisp/international/cp51932.el
cd emacs/admin/charsets && gunzip -c glibc/EUC-JP-MS.gz | gawk -f eucjp-ms.awk \
    > <port>/femacs/lisp/international/eucjp-ms.el
```

| 产物 | 字节 | 与参照产物 |
| --- | ---: | --- |
| `international/charscript.el` | 28,303 | **逐字节一致** |
| `international/emoji-zwj.el` | 191,950 | **逐字节一致** |
| `international/cp51932.el` | 9,631 | **逐字节一致** |
| `international/eucjp-ms.el` | 39,937 | **逐字节一致** |

这四件是完成定义第 4 条的第一批读数，也让端口**在自己的树上跑完整个 `loadup.el`**（下一节）。

**emacs 生成的族（`admin/unidata` 的 `unidata-gen-*`）**

配方（与 `admin/unidata/Makefile.in` 同形；`unidata.txt` 由 `sed` 从 `UnicodeData.txt` 产出）：

```sh
sed -e 's/\([^;]*\);\（.*\)/(#x\1 "\2")/' -e 's/;/" "/g' < UnicodeData.txt > unidata.txt
<emacs> -batch --no-site-file --no-site-lisp -L <admin/unidata> -l unidata-gen \
    -f unidata-gen-file <target> <admin/unidata> <unidata.txt>
<emacs> -batch --no-site-file --no-site-lisp -L <admin/unidata> -l unidata-gen \
    -f unidata-gen-charprop <target>
<emacs> -batch --no-site-file --no-site-lisp -L <admin/unidata> -l unidata-gen \
    -f unidata-gen-scripts|unidata-gen-confusable|unidata-gen-idna-mapping <target>
<emacs> -batch --no-site-file --no-site-lisp -L <lisp/international> -l emoji.el \
    -f emoji--generate-file <target>
```

**端口自己跑通了这条配方**（`build/port/src/temacs` 生成 `uni-bidi.el`：exit 0，12,387 字节），
与参照的 `emacs/lisp/international/uni-bidi.el` 相比**只在程序自己的名字那一行不同**
（`;; This file is part of femacs.` 对 `;; This file is part of GNU Emacs.`，3 字节 =
PORTING.md 0.1-19 那条：端口用自己的名字，不许写参照的名字；`unidata-gen.el` 第 9 行是
`check-verbatim.py` 的 8,463 条 sanctioned 改名之一）。**剩下的差异来自生成器自身的形态**：参照的
配方把 `unidata-gen.elc` 作为 order-only 前置（`-l unidata-gen` 因此加载**字节编译过的**生成器 ✓），
端口当时还没有 `.elc`，所以它加载源码版 ✓，生成物里内嵌的 compiled-function 字面量因此不同。
**结论：生成物逐字节一致这一条，先决条件是端口的字节编译器产出与参照一致的 `.elc`**（下一节）。

## 端口在自己的树上跑完 `loadup.el`（2026-10-16）

```sh
build/port/src/temacs -Q --batch --eval '(princ (+ 1 2))'   # 不设 EMACSLOADPATH
```

`exit 0`、stdout `3`、stderr **108 个 `Loading ...`**（端口自己的 `lisp/`，源码形式
`(source)`；load-path 读作 `build/port/lisp/...`）。这需要上面四件 AWK 生成物：端口此前依次停在
`charscript`、`cp51932`（`Cannot open load file`），补上后 loadup 一路走完
（`cus-start.el` 是最后一个）。`.elc` 与其余 78 个生成 `.el` **不是**这一级的先决条件
（端口按 `.el` 源码加载）。
