#!/usr/bin/env python3
"""Generate `lisp/leim/ja-dic/ja-dic.el` and its `.elc` with the port's own program.

`lisp/Makefile.in` 172-174 builds the Japanese dictionary table by delegating to `leim/Makefile`'s
`generate-ja-dic`, whose recipe is

    <emacs> -batch --no-site-file --no-site-lisp -batch -l ja-dic-cnv \
        -f batch-skkdic-convert -dir "<lisp>/leim/ja-dic" --no-reduction "<leim>/SKK-DIC/SKK-JISYO.L"

followed by a byte-compilation of the result.  Both products are *generated*: neither is in the
verbatim copy, and the reference's own tree has them only because its build wrote them there.
`tools/gen-elc.py` therefore counted one target as `missing` (1,645 targets: 1,587 identical /
56 state-only / 1 missing) - `lisp/leim/ja-dic/ja-dic.elc`, whose source did not exist on the port
side.  This tool produces both, which is what closes that reading.

Like `tools/gen-leim.py` this asks the *frozen reference build* what it would run
(`make -B -n generate-ja-dic`) and rewrites only

  * the program: `../src/emacs` becomes the port's binary - the generated file carries the name of
    the program that built it (PORTING.md 0.1-19), so the port cannot borrow the reference's;
  * the output directory and the input dictionary: the port's own copies.

The comparison is the reading: `ja-dic.el` and `ja-dic.elc` against the reference's own copies,
using `tools/gen-elc.py`'s `rewrite_only` - the shared definition of "only the program name and the
doc-string offsets differ" (P-145/P-186) - so a difference here is a defect or a finding, never a
silent pass.

    python3 tools/gen-ja-dic.py             # generate, byte-compile, compare
    python3 tools/gen-ja-dic.py --list      # print the commands and exit
    python3 tools/gen-ja-dic.py --show 40   # how many differing lines to print per file
"""

from __future__ import annotations

import argparse
import importlib.util
import pathlib
import shlex
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"
REF_BUILD_LEIM = REPO / "build/emacs-ref/leim"
DEFAULT_PROGRAM = REPO / "build/port/src/temacs"
FALLBACK_PROGRAM = REPO / "build/port/src/emacs"
REL = "lisp/leim/ja-dic/ja-dic.el"

# `tools/gen-elc.py` owns the definition of "the only difference is the program's own name".
_spec = importlib.util.spec_from_file_location("gen_elc", REPO / "tools/gen-elc.py")
assert _spec and _spec.loader
gen_elc = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(gen_elc)


def recipe() -> list[str]:
    """The ja-dic command the frozen reference build would run, as an argv list."""
    out = subprocess.run(["make", "-B", "-n", "generate-ja-dic", "EMACS=../src/emacs"],
                         cwd=REF_BUILD_LEIM, capture_output=True, text=True)
    if out.returncode != 0:
        raise SystemExit(f"the reference build's make failed: {out.stderr.strip()[-300:]}")
    joined = out.stdout.replace("\\\n", " ")
    for line in joined.splitlines():
        if "batch-skkdic-convert" in line:
            return shlex.split(line.strip())
    raise SystemExit("the reference build no longer has a ja-dic recipe: " + out.stdout[-300:])


def rewrite(argv: list[str], program: pathlib.Path) -> list[str]:
    """Point the recipe at the port: its program, its output directory, its dictionary."""
    out = list(argv)
    out[0] = str(program)
    out[out.index("-dir") + 1] = str(PORT / "lisp/leim/ja-dic")
    out[-1] = str(PORT / "leim/SKK-DIC/SKK-JISYO.L")
    return out


def run(argv: list[str], timeout: int) -> tuple[int, str]:
    env = {"EMACSLOADPATH": str(PORT / "lisp"), "PATH": "/usr/bin:/bin"}
    done = subprocess.run(argv, capture_output=True, text=True, timeout=timeout, env=env)
    tail = (done.stdout + done.stderr).strip()
    return done.returncode, tail[-2000:]


def compare(rel: str, show: int) -> str:
    """Compare one generated product with the reference's copy; return a reading line."""
    produced = PORT / rel
    expected = REF / rel
    if not produced.is_file():
        return f"MISSING      {rel}: the port produced nothing"
    if not expected.is_file():
        return f"NO-REFERENCE {rel}: the reference has no copy to compare with"
    got, want = produced.read_bytes(), expected.read_bytes()
    if got == want:
        return f"identical    {rel} ({len(got)} bytes)"
    if gen_elc.rewrite_only(got, want, gen_elc.PORT_NAME):
        return f"rewrite-only {rel} ({len(got)} bytes; only the program name and doc offsets differ)"
    got_lines = got.splitlines()
    want_lines = want.splitlines()
    differing = sum(1 for a, b in zip(got_lines, want_lines) if a != b) + abs(
        len(got_lines) - len(want_lines))
    lines = [f"DIFFERENT    {rel} ({differing} differing line(s), {len(got_lines)} vs "
             f"{len(want_lines)} lines)", "  " + gen_elc.first_difference(got, want)]
    for i, (a, b) in enumerate(zip(got_lines, want_lines)):
        if a != b and len(lines) < 2 + show:
            lines.append(f"  line {i + 1}:\n    port: {a[:200]!r}\n    ref:  {b[:200]!r}")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--program", type=pathlib.Path, default=None)
    parser.add_argument("--timeout", type=int, default=1800)
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--show", type=int, default=6)
    parser.add_argument("--no-compile", action="store_true")
    args = parser.parse_args()

    program = args.program or (DEFAULT_PROGRAM if DEFAULT_PROGRAM.is_file() else FALLBACK_PROGRAM)
    if not program.is_file():
        raise SystemExit(f"the port's program is missing: {program} (build it first)")
    convert = rewrite(recipe(), program)
    (PORT / "lisp/leim/ja-dic").mkdir(parents=True, exist_ok=True)
    compile_argv = [str(program), "-batch", "--no-site-file", "--no-site-lisp",
                    "-f", "batch-byte-compile", str(PORT / REL)]
    print("program:", program)
    print("convert:", " ".join(shlex.quote(a) for a in convert))
    if not args.no_compile:
        print("compile:", " ".join(shlex.quote(a) for a in compile_argv))
    if args.list:
        return 0

    code, tail = run(convert, args.timeout)
    print(f"convert: exit {code}" + (f"\n  {tail}" if code else ""))
    if code != 0:
        return 1
    readings = [compare(REL, args.show)]
    if not args.no_compile:
        code, tail = run(compile_argv, args.timeout)
        print(f"compile: exit {code}" + (f"\n  {tail}" if code else ""))
        if code != 0:
            return 1
        readings.append(compare(REL + "c", args.show))
    for line in readings:
        print(line)
    return 1 if any(line.startswith("DIFFERENT") for line in readings) else 0


if __name__ == "__main__":
    sys.exit(main())
