#!/usr/bin/env python3
"""L0 of the acceptance ladder: run the self-check expressions against a binary and compare.

`docs/port-plan.md` 7.1's L0 is "self-check expressions (`(+ 1 2)`, `prin1`, `condition-case`,
`unwind-protect`) agree with the reference byte for byte".  The reference program exists
(`docs/acceptance.md`), so the *golden* side can be recorded today even though the port's binary
does not link yet - and when it does, the check side is one command.

    python3 tools/acceptance-l0.py --golden build/emacs-ref/src/emacs
    python3 tools/acceptance-l0.py --check <binary>        # compares with the golden

Every expression is run as `<binary> -Q --batch --eval <program>` and what is compared is the
triple `(exit code, stdout, stderr)`, byte for byte - the plan's wording.  The golden file records
the binary's path, its `--version` first line and its sha256, so a golden whose reference has moved
is visible rather than silent.  The expressions are chosen to be *path independent* on purpose:
anything that prints a build or data directory belongs to a later rung, where the port and the
reference are expected to differ and the comparison has to normalize first.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
# The default golden is the one recorded from the reference's *dumped* program.  The ladder has
# two flavours of "the reference program" - the dumped `emacs` and the bare `temacs` that loads
# `loadup.el` from source - and their stderr differs by construction (the dump is silent, the
# bare program prints its load trace).  `--golden-path` records a second golden beside this one
# so a port binary is compared with the flavour it can actually be at its rung: L0-L3 are the
# bare program (there is no dump yet), so the reading that matters today is
# `docs/baseline/l0-reference-temacs.txt` (docs/acceptance.md's L0 section records both).
GOLDEN = REPO / "docs/baseline/l0-reference.txt"

# (label, program).  Ordered, and each label is unique: the check side compares by label so a
# reordering cannot silently turn a mismatch into "no difference".
EXPRESSIONS: list[tuple[str, str]] = [
    ("arith", "(princ (+ 1 2))"),
    ("bignum", "(princ (* 123456789 100000000000))"),
    ("ratio", "(prin1 (list (/ 7 2) (/ 7.0 2)))"),
    ("prin1-list", "(prin1 (list 1 2 3))"),
    ("closure-mapcar", "(prin1 (mapcar (lambda (x) (* x x)) (list 1 2 3)))"),
    ("format", "(princ (format \"%s|%d|%S\" \"a\" 42 'sym))"),
    ("condition-case", "(princ (condition-case e (car 5) (error (format \"caught:%s\" (car e)))))"),
    ("unwind-protect", "(princ (unwind-protect (condition-case nil (error \"x\") (error 'inner)) (princ \"[cleanup]\")))"),
    ("string-mutate", "(prin1 (let ((s (make-string 3 ?a))) (aset s 1 ?b) s))"),
    ("string-bytes", "(princ (list (length \"héllo\") (string-bytes \"héllo\")))"),
    ("sort-and-append", "(prin1 (sort (append (list 3 1) (list 2)) #'<))"),
    ("hash-table", "(princ (let ((h (make-hash-table))) (puthash 'a 1 h) (puthash 'b 2 h) (gethash 'b h)))"),
    ("read-print", "(prin1 (read \"(1 2 . 3)\"))"),
    ("symbol-name", "(princ (format \"%s:%s\" (symbol-name 'foo) (symbolp 'foo)))"),
    ("char-to-string", "(prin1 (char-to-string 65))"),
    ("buffer-roundtrip", "(princ (with-temp-buffer (insert \"abc\") (goto-char 2) (list (buffer-string) (point))))"),
    ("message-stderr", "(progn (message \"to stderr\") (princ \"to stdout\"))"),
    ("signal-message", "(princ (condition-case e (signal 'wrong-type-argument (list 'integerp \"x\")) (error (error-message-string e))))"),
    ("exit-code", "(kill-emacs 3)"),
]


def run(binary: Path, program: str) -> tuple[int, str, str]:
    r = subprocess.run([str(binary), "-Q", "--batch", "--eval", program],
                       capture_output=True, text=True)
    return r.returncode, r.stdout, r.stderr


def binary_identity(binary: Path) -> tuple[str, str]:
    version = subprocess.run([str(binary), "--version"], capture_output=True, text=True).stdout
    first = version.split("\n")[0] if version else ""
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()[:16]
    return first, digest


def version_number(first: str) -> str:
    """The reference's version *number*, out of its `--version` first line.

    `PORTING.md` 0.1-19: a version declaration this repository writes carries the revision and
    the version number, never the product's name - the reference prints `<name> <version>`, so
    the last whitespace-separated token is the number.  A line with no separate token (a build
    that prints the number alone) stays as it is, so the golden shows the difference instead of
    an empty field.
    """
    tokens = first.split()
    return tokens[-1] if len(tokens) > 1 else first


def version_header_lines(first: str) -> list[str]:
    """The `# version:` header line this repository writes into the golden, and why it is bare.

    The reference's own banner is quoted in `docs/acceptance.md`, where it is evidence *about*
    the reference; repeating the name it carries in a file of ours is the declaration 0.1-19
    removes.  The revision and the sha256 below identify the measured binary without it.
    """
    return [f"# version: {version_number(first)}",
            "#   the number only; PORTING.md 0.1-19: the name the reference prints is quoted in",
            "#   docs/acceptance.md as evidence about the reference, and is not repeated here"]


def write_golden(binary: Path, golden: Path) -> int:
    first, digest = binary_identity(binary)
    lines = ["# L0 golden: the self-check expressions' outputs on the reference program.",
             "# Written by `python3 tools/acceptance-l0.py --golden <binary>` (docs/status.md 3.261,",
             "# docs/acceptance.md).  Compared by `--check <binary>`, which re-runs each expression",
             "# as `<binary> -Q --batch --eval <program>` and compares (exit code, stdout, stderr)",
             "# byte for byte.  One record per expression, in this order.",
             f"# binary: {binary}",
             *version_header_lines(first),
             f"# sha256[0:16]: {digest}",
             *([f"# EMACSLOADPATH: {os.environ['EMACSLOADPATH']}"]
               if os.environ.get("EMACSLOADPATH") else
               ["# EMACSLOADPATH: (unset - the binary uses its own tree)"]),
             ""]
    for label, program in EXPRESSIONS:
        code, out, err = run(binary, program)
        lines += [f"@@ {label}", f"$ {program}", f"--- exit: {code}",
                  "--- stdout:", out, "--- end stdout", "--- stderr:", err, "--- end stderr"]
    golden.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {golden}: {len(EXPRESSIONS)} expression(s) "
          f"(reference {version_number(first)!r}, sha256[0:16] {digest})")
    return 0


def read_golden(golden: Path) -> dict[str, tuple[int, str, str]]:
    """Every record of the golden, keyed by label, as `(exit code, stdout, stderr)`.

    The format is the one `write_golden()` writes: `@@ label`, the program line, the exit code,
    then the two streams between explicit `--- stdout:` / `--- end stdout` and
    `--- stderr:` / `--- end stderr` markers - which is what makes the bytes of a stream readable
    without guessing whether its last newline belongs to it.
    """
    if not golden.is_file():
        raise SystemExit(f"{golden} does not exist: run --golden first")
    records: dict[str, tuple[int, str, str]] = {}
    label: str | None = None
    code = 0
    mode: str | None = None
    out: list[str] = []
    err: list[str] = []

    def flush() -> None:
        if label is not None:
            records[label] = (code, "\n".join(out), "\n".join(err))

    for line in golden.read_text(encoding="utf-8").split("\n"):
        if line.startswith("@@"):
            flush()
            label, code, mode, out, err = line[3:].strip(), 0, None, [], []
        elif label is None:
            continue
        elif line.startswith("$ ") or line.startswith("--- exit: ") and not line.startswith("--- exit: -"):
            if line.startswith("--- exit: "):
                code = int(line[len("--- exit: "):])
        elif line == "--- stdout:":
            mode = "out"
        elif line == "--- end stdout":
            mode = None
        elif line == "--- stderr:":
            mode = "err"
        elif line == "--- end stderr":
            mode = None
        elif mode == "out":
            out.append(line)
        elif mode == "err":
            err.append(line)
    flush()
    return records


def check(binary: Path, golden_path: Path) -> int:
    golden = read_golden(golden_path)
    first, digest = binary_identity(binary)
    print(f"binary: {binary}\n  version: {version_number(first)}\n  sha256[0:16]: {digest}")
    print(f"golden: {golden_path}\n  load path: "
          f"{os.environ.get('EMACSLOADPATH', '(unset - the binary uses its own tree)')}")
    failures = 0
    for label, program in EXPRESSIONS:
        want = golden.get(label)
        if want is None:
            print(f"GONE  {label}: the golden has no record under this label")
            failures += 1
            continue
        got = run(binary, program)
        if got == want:
            print(f"ok    {label}")
            continue
        failures += 1
        print(f"FAIL  {label}")
        for field, a, b in (("exit", want[0], got[0]), ("stdout", want[1], got[1]),
                            ("stderr", want[2], got[2])):
            if a != b:
                print(f"      {field}: golden {a!r}\n"
                      f"      {field}: binary {b!r}")
    print(f"{len(EXPRESSIONS)} expression(s), {failures} failure(s)")
    return 1 if failures else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--golden", metavar="BINARY", help="record the golden from this reference binary")
    ap.add_argument("--check", metavar="BINARY", help="compare this binary with the golden")
    ap.add_argument("--golden-path", metavar="PATH",
                    help="the golden file to write or read (default: "
                         f"{GOLDEN.relative_to(REPO)}); the ladder's two flavours keep one each")
    args = ap.parse_args()
    golden_path = Path(args.golden_path) if args.golden_path else GOLDEN
    if args.golden:
        return write_golden(Path(args.golden), golden_path)
    if args.check:
        return check(Path(args.check), golden_path)
    ap.print_help()
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
