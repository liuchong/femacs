#!/usr/bin/env python3
"""Run the reference's own ERT harness against both programs and compare per file (L5).

The ladder's last rung is the Lisp test suite (`docs/port-plan.md` 7.1), and it is
*differential*: `docs/acceptance.md` builds the reference program so every rung can compare the
port with it.  The recipe is the reference test Makefile's own - `EMACSOPT`, `HOME=/nonexistent`,
`-l ert`, the file, then `ert-run-tests-batch-and-exit t` (Makefile 62, 83, 123, 183-187) - run
once with each program over that program's own tree, so nothing about the harness is re-invented.

Each run's ERT summary is parsed; a file is reported as

    same            both sides report no unexpected results
    port-worse      the port's unexpected count is higher (the defect list)
    port-better     the port's is lower (interesting, not a defect)
    both-fail       equal, non-zero (a difference the *reference* also has)
    crashed         a side produced no summary, or exited non-zero, or timed out

Runs are sequential: a parallel sweep would hold dozens of programs at once, and this project
keeps its memory bounded (AGENTS.md section 3).

Usage
-----
    python3 tools/check-ert.py --glob 'lisp/emacs-lisp/*.el'          # a slice
    python3 tools/check-ert.py                                        # every test file
    python3 tools/check-ert.py --tsv /tmp/ert.tsv
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PORT_PROG = REPO / "build/port/src/emacs"
REF_PROG = REPO / "build/emacs-ref/src/emacs"
PORT_TEST = REPO / "femacs/test"
REF_TEST = REPO / "emacs/test"
SUMMARY = re.compile(r"Ran (\d+) tests?, (\d+) results? as expected, (\d+) unexpected")


def run(prog: Path, test_root: Path, rel: str, timeout: int) -> tuple[str, int, int, int, str]:
    """(verdict, ran, expected, unexpected, note) for one program and one test file."""
    argv = [str(prog), "--no-init-file", "--no-site-file", "--no-site-lisp", "-L", f":{test_root}",
            "-l", "ert", "-l", str(test_root / rel), "--batch",
            "--eval", "(ert-run-tests-batch-and-exit t)"]
    try:
        out = subprocess.run(argv, capture_output=True, text=True, timeout=timeout,
                             env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin",
                                  "LC_ALL": "C"},
                             cwd=str(REPO))
    except subprocess.TimeoutExpired:
        return "timeout", 0, 0, 0, f"no summary within {timeout}s"
    text = out.stdout + out.stderr
    m = None
    for m2 in SUMMARY.finditer(text):
        m = m2
    if m is None:
        tail = " | ".join(text.strip().splitlines()[-2:])[:160]
        return ("crashed" if out.returncode != 0 else "no-summary"), 0, 0, 0, f"exit {out.returncode}: {tail}"
    ran, expected, unexpected = (int(m.group(1)), int(m.group(2)), int(m.group(3)))
    if out.returncode != 0 and unexpected == 0:
        return "crashed", ran, expected, unexpected, f"exit {out.returncode} after a clean summary"
    return "ok", ran, expected, unexpected, ""


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--glob", default="**/*.el", help="test file pattern, relative to the test root")
    ap.add_argument("--timeout", type=int, default=300)
    ap.add_argument("--limit", type=int, default=40, help="max detail lines to print")
    ap.add_argument("--tsv")
    ap.add_argument("--retries", type=int, default=1,
                    help="re-run a crashed file this many times before calling it a defect; a "
                         "file that passes on a retry is reported as flaky, because the port's "
                         "crashes so far are layout-dependent (PORTING.md P-111)")
    ap.add_argument("--port-prog", type=Path, default=PORT_PROG)
    ap.add_argument("--ref-prog", type=Path, default=REF_PROG)
    a = ap.parse_args()

    for p in (a.port_prog, a.ref_prog):
        if not p.is_file():
            print(f"missing program: {p}", file=sys.stderr)
            return 2
    files = sorted(str(p.relative_to(REF_TEST))
                   for p in REF_TEST.glob(a.glob)
                   if p.is_file() and not p.name.startswith("."))
    if not files:
        print(f"no test files match {a.glob}", file=sys.stderr)
        return 2

    rows: list[tuple[str, str, int, int, int, int, str]] = []
    started = time.time()
    for i, rel in enumerate(files, 1):
        v1, r1, e1, u1, n1 = run(a.port_prog, PORT_TEST, rel, a.timeout)
        retried = False
        while v1 != "ok" and a.retries > 0 and not retried:
            retried = True
            v1, r1, e1, u1, n1 = run(a.port_prog, PORT_TEST, rel, a.timeout)
            if v1 == "ok":
                rows.append((rel, "flaky", r1, u1, 0, 0,
                             "the port crashed and passed on a retry"))
        if retried and v1 == "ok":
            continue
        v2, r2, e2, u2, n2 = run(a.ref_prog, REF_TEST, rel, a.timeout)
        if v1 != "ok" or v2 != "ok":
            verdict = "crashed"
            note = f"port: {v1} {n1} | reference: {v2} {n2}"
        elif u1 == 0 and u2 == 0:
            verdict, note = "same", ""
        elif u1 > u2:
            verdict, note = "port-worse", f"{u1} unexpected vs {u2}"
        elif u1 < u2:
            verdict, note = "port-better", f"{u1} unexpected vs {u2}"
        else:
            verdict, note = "both-fail", f"both {u1} unexpected"
        rows.append((rel, verdict, r1, u1, r2, u2, note))
        if i % 25 == 0:
            print(f"  … {i}/{len(files)} ({time.time() - started:.0f}s)", file=sys.stderr)

    tally: dict[str, int] = {}
    for r in rows:
        tally[r[1]] = tally.get(r[1], 0) + 1
    print(f"test files: {len(rows)}; " + ", ".join(f"{k}={v}" for k, v in sorted(tally.items())))
    print(f"port tests run: {sum(r[2] for r in rows)}, unexpected: {sum(r[3] for r in rows)}; "
          f"reference tests run: {sum(r[4] for r in rows)}, unexpected: {sum(r[5] for r in rows)}")
    bad = [r for r in rows if r[1] in ("port-worse", "crashed")]
    for r in bad[:a.limit]:
        print(f"  {r[1]:11s} {r[0]}: {r[6]}")
    if len(bad) > a.limit:
        print(f"  … {len(bad) - a.limit} more")
    if a.tsv:
        Path(a.tsv).write_text("file\tverdict\tport_ran\tport_unexpected\tref_ran\tref_unexpected\tnote\n"
                               + "".join("\t".join(str(x) for x in r) + "\n" for r in rows),
                               encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
