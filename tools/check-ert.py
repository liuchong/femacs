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
import resource
import subprocess
import sys
import threading
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PORT_PROG = REPO / "build/port/src/emacs"
REF_PROG = REPO / "build/emacs-ref/src/emacs"
PORT_TEST = REPO / "femacs/test"
REF_TEST = REPO / "emacs/test"
SUMMARY = re.compile(r"Ran (\d+) tests?, (\d+) results? as expected, (\d+) unexpected")

# The reference's `test/Makefile.in` 139-145 spells three selectors, and `make check` runs the
# first: `SELECTOR_DEFAULT = (not (or (tag :expensive-test) (tag :unstable)))`.  The harness
# passed `t` (= `SELECTOR_ALL`) instead, which pulls in the files' `:unstable` tests; on
# `lisp/emacs-lisp/ert-tests.el` that makes *both* programs climb past a gigabyte (measured
# 2026-10-16: the reference reached 1.26 GiB in 3.4 s, the port 1.1 GiB in 20 s, either killed
# by the cap), so the runaway read as a port defect when it is the selector.  `--selector`
# overrides the text; `--selector-preset default|expensive|all` picks one of the three.
SELECTOR_DEFAULT = "(not (or (tag :expensive-test) (tag :unstable)))"
SELECTOR_PRESETS = {"default": SELECTOR_DEFAULT,
                    "expensive": "(not (tag :unstable))",
                    "all": "t"}


def ensure_port_tree_is_a_project_root() -> None:
    """`emacs/` is its own git repository (the pinned checkout), and two `project-tests`/
    `elisp-mode-tests` cases depend on that: they compute `(file-relative-name FILE
    (project-root ...))` and expect it to start at `test/`.  The port's tree lives inside
    this repository, so without the same marker its project root is the repository and the
    relative name starts at `femacs/`.  Writing the gitfile makes the two trees equivalent
    (PORTING.md P-135); it is ignored by the outer repository, so the worktree stays clean.
    """
    gitfile = PORT_TEST.parent / ".git"                 # femacs/.git
    if not gitfile.exists():
        gitfile.write_text("gitdir: ../.git\n")


PRUNE_DIRS = {"manual", "data", "infra"}
PRUNE_SUFFIX = "resources"


def enumerate_tests(root: Path) -> list[str]:
    """The reference's own list of test files: `test/Makefile.in` 206-210's `find`, i.e. every
    `*.el` under the test root except the pruned directories (`manual`, `data`, `infra`,
    `*resources`) and dotfiles.  Using the same recipe is what makes a sweep's denominator the
    reference's own (PORTING.md P-138), instead of a hand-written glob that can silently skip
    a subdirectory (P-137's `textmodes/ispell-tests`) or a differently-named file."""
    out = []
    for path in root.rglob("*.el"):
        if path.name.startswith("."):
            continue
        parts = path.relative_to(root).parts[:-1]
        if any(part in PRUNE_DIRS or part.endswith(PRUNE_SUFFIX) for part in parts):
            continue
        out.append(str(path.relative_to(root)))
    return sorted(out)


def cap_memory(mib: int):
    """A `preexec_fn` that caps the child's address space.  Linux honours it; Darwin refuses to
    lower `RLIMIT_AS` below the current address space ("Invalid argument"), which is why the
    watchdog below exists as well - the two together give a bounded reading on both."""
    def limit():
        try:
            resource.setrlimit(resource.RLIMIT_AS, (mib << 20, mib << 20))
        except (ValueError, OSError):
            pass
    return limit


def watchdog(proc: subprocess.Popen, mib: int) -> bool:
    """Kill `proc` once its RSS passes `mib`.  Returns whether it fired.  `ps` is sampled rather
    than the limit enforced, because a *bounded* reading is all this needs: the point is that a
    runaway file costs the executor nothing."""
    while proc.poll() is None:
        time.sleep(0.5)
        try:
            out = subprocess.run(["ps", "-o", "rss=", "-p", str(proc.pid)],
                                 capture_output=True, text=True, timeout=5)
            rss_kib = int(out.stdout.strip() or 0)
        except (ValueError, subprocess.SubprocessError):
            continue
        if rss_kib > (mib << 10):
            proc.kill()
            return True
    return False


def run(prog: Path, test_root: Path, rel: str, timeout: int,
        selector: str = SELECTOR_DEFAULT,
        memory_mib: int = 0) -> tuple[str, int, int, int, str]:
    """(verdict, ran, expected, unexpected, note) for one program and one test file."""
    argv = [str(prog), "--no-init-file", "--no-site-file", "--no-site-lisp", "-L", f":{test_root}",
            "-l", "ert", "-l", str(test_root / rel), "--batch",
            "--eval", f"(ert-run-tests-batch-and-exit (quote {selector}))"]
    proc = subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                            env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin",
                                 "LC_ALL": "C"},
                            cwd=str(REPO),
                            preexec_fn=cap_memory(memory_mib) if memory_mib else None)
    fired = threading.Thread(target=lambda: None)
    over = {"fired": False}
    if memory_mib:
        def watch():
            over["fired"] = watchdog(proc, memory_mib)
        fired = threading.Thread(target=watch, daemon=True)
        fired.start()
    try:
        out_text, _ = proc.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.communicate()
        return "timeout", 0, 0, 0, f"no summary within {timeout}s"
    if memory_mib:
        fired.join(timeout=5)
    text = out_text or ""
    m = None
    for m2 in SUMMARY.finditer(text):
        m = m2
    if m is None:
        tail = " | ".join(text.strip().splitlines()[-2:])[:160]
        if over["fired"]:
            return "out-of-memory", 0, 0, 0, f"no summary within a {memory_mib} MiB cap"
        # A SIGKILL the watchdog did not fire for is not an out-of-memory reading: the port
        # aborts with SIGKILL when libplatform catches a recursive `os_unfair_lock` abort
        # (PORTING.md P-126 - `getenv` inside the SIGCHLD handler), and that abort used to be
        # reported as "out-of-memory", which sent a whole round after the wrong phenomenon.
        if proc.returncode == -9:
            return "crashed", 0, 0, 0, f"exit -9 (SIGKILL, watchdog silent): {tail}"
        return ("crashed" if proc.returncode != 0 else "no-summary"), 0, 0, 0, f"exit {proc.returncode}: {tail}"
    ran, expected, unexpected = (int(m.group(1)), int(m.group(2)), int(m.group(3)))
    if proc.returncode != 0 and unexpected == 0:
        return "crashed", ran, expected, unexpected, f"exit {proc.returncode} after a clean summary"
    return "ok", ran, expected, unexpected, ""


def main() -> int:
    ensure_port_tree_is_a_project_root()
    ap = argparse.ArgumentParser()
    ap.add_argument("--glob", default="**/*.el",
                    help="test file pattern, relative to the test root; the default is the "
                         "reference's own ELFILES list (see enumerate_tests)")
    ap.add_argument("--list", action="store_true",
                    help="print the file list the run would use and exit (no Emacs needed)")
    ap.add_argument("--timeout", type=int, default=300)
    ap.add_argument("--limit", type=int, default=40, help="max detail lines to print")
    ap.add_argument("--tsv")
    ap.add_argument("--max-files", type=int, default=0,
                    help="stop after this many files (chunked sweeps, so one heavy file cannot "
                         "cut a long run short); pair with --start-at to resume")
    ap.add_argument("--start-at", type=int, default=0,
                    help="skip this many files (resume a chunked sweep)")
    ap.add_argument("--retries", type=int, default=1,
                    help="re-run a crashed file this many times before calling it a defect; a "
                         "file that passes on a retry is reported as flaky, because the port's "
                         "crashes so far are layout-dependent (PORTING.md P-111)")
    ap.add_argument("--selector", default=SELECTOR_DEFAULT,
                    help="the ERT selector text both programs run; the default is the "
                         "reference's own SELECTOR_DEFAULT (test/Makefile.in 139)")
    ap.add_argument("--selector-preset", choices=sorted(SELECTOR_PRESETS), default=None,
                    help="the reference's shorthand: default|expensive|all")
    ap.add_argument("--memory-mib", type=int, default=0,
                    help="cap each side's address space (RLIMIT_AS); a file that runs out of "
                         "memory is then a bounded reading instead of an executor kill")
    ap.add_argument("--port-prog", type=Path, default=PORT_PROG)
    ap.add_argument("--ref-prog", type=Path, default=REF_PROG)
    a = ap.parse_args()

    if a.selector_preset:
        a.selector = SELECTOR_PRESETS[a.selector_preset]
    for p in (a.port_prog, a.ref_prog):
        if not p.is_file():
            print(f"missing program: {p}", file=sys.stderr)
            return 2
    if a.glob == "**/*.el":
        files = enumerate_tests(REF_TEST)
        print(f"enumerated {len(files)} file(s) from test/Makefile.in's ELFILES recipe")
    else:
        files = sorted(str(p.relative_to(REF_TEST))
                       for p in REF_TEST.glob(a.glob)
                       if p.is_file() and not p.name.startswith("."))
    if not files:
        print(f"no test files match {a.glob}", file=sys.stderr)
        return 2
    if a.list:
        for f in files:
            print(f)
        return 0

    rows: list[tuple[str, str, int, int, int, int, str]] = []
    started = time.time()
    high_water = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss   # KiB on Darwin
    if a.start_at:
        files = files[a.start_at:]
    if a.max_files:
        files = files[:a.max_files]
    for i, rel in enumerate(files, 1):
        v1, r1, e1, u1, n1 = run(a.port_prog, PORT_TEST, rel, a.timeout, a.selector, a.memory_mib)
        retried = False
        while v1 != "ok" and a.retries > 0 and not retried:
            retried = True
            v1, r1, e1, u1, n1 = run(a.port_prog, PORT_TEST, rel, a.timeout, a.selector, a.memory_mib)
            if v1 == "ok":
                rows.append((rel, "flaky", r1, u1, 0, 0,
                             "the port crashed and passed on a retry"))
        if retried and v1 == "ok":
            continue
        v2, r2, e2, u2, n2 = run(a.ref_prog, REF_TEST, rel, a.timeout, a.selector, a.memory_mib)
        if v1 == "out-of-memory" and v2 == "out-of-memory":
            rows.append((rel, "both-capped", r1, u1, r2, u2,
                         "the file is too heavy for the harness's cap on both sides"))
            continue
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
        # The sweep's own memory: a run that lifts the children high-water mark is the one to
        # look at, and the executor's budget kills the whole sweep otherwise (status 4.427).
        now = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
        if now > high_water:
            high_water = now
            note = (note + " " if note else "") + f"new RSS high-water {now // (1024 * 1024)} MiB"
        rows.append((rel, verdict, r1, u1, r2, u2, note))
        if i % 10 == 0:
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
