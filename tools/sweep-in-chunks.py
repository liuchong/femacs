#!/usr/bin/env python3
"""Run a sweep tool in chunks, under the controlled runner, and merge its ledger.

`docs/port-plan.md` 7.2's sweeps walk hundreds of test files.  Two facts about running them here
(P-222, status 4.609):

  * the controlled runner's reported peak accumulates over the *child processes* of one
    invocation (45 files read 509 MB, 340 files read 3.5 GB), so a whole-sweep invocation trips
    its budget even though no single test case uses more than ~600 MB;
  * therefore the sweep has to be split, and the tool's own `--memory-mib` watchdog stays the
    per-case memory judgement.

This driver does the splitting and, when a chunk still trips the runner's budget, halves it and
retries - down to single files, which is the granularity at which the runner's number is a real
peak.  Chunks run through the runner (never beside it), so every heavy child stays monitored and
over-budget ones are killed.

    python3 tools/sweep-in-chunks.py --tool tools/check-ert.py \
        --tool-arg --gc-stress --tool-arg --selector-preset --tool-arg default \
        --tool-arg --memory-mib --tool-arg 700 \
        --out docs/baseline/gc-stress-all.tsv --total 564 --chunk 30
"""

from __future__ import annotations

import argparse
import json
import pathlib
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
RUNNER = pathlib.Path.home() / ".assistant/skills/controlled-run/scripts/run.py"


def run_chunk(tool: str, tool_args: list[str], start: int, count: int, tsv: pathlib.Path,
              profile: str, memory_mib: int, timeout: int) -> dict:
    """One runner invocation; returns the runner's own reading plus the exit code."""
    argv = [sys.executable, str(RUNNER), "--profile", profile, "--memory-mib", str(memory_mib),
            "--", sys.executable, tool, *tool_args,
            "--start-at", str(start), "--max-files", str(count), "--tsv", str(tsv)]
    done = subprocess.run(argv, cwd=REPO, capture_output=True, text=True, timeout=timeout)
    reading = {"exit": done.returncode, "reason": "?", "peak": 0, "seconds": 0.0}
    # The runner's report is read from *both* streams: this driver first searched stdout only and
    # printed `over budget (?, peak 0 MiB)` for every chunk, i.e. it could not tell a killed chunk
    # from a finished one (status 4.611).
    for line in (done.stdout + done.stderr).splitlines():
        if line.startswith("controlled-run: "):
            try:
                payload = json.loads(line[len("controlled-run: "):])
            except json.JSONDecodeError:
                continue
            reading.update(reason=payload.get("reason", "?"),
                           peak=payload.get("peak_rss_bytes", 0),
                           seconds=payload.get("elapsed_seconds", 0.0))
    reading["tail"] = done.stdout.strip().splitlines()[-1] if done.stdout.strip() else ""
    return reading


def sweep(tool: str, tool_args: list[str], start: int, count: int, work: pathlib.Path,
          profile: str, memory_mib: int, timeout: int, split_at: list[int]) -> list[pathlib.Path]:
    """Sweep [start, start+count) as chunks; halve a chunk that the runner had to kill."""
    pieces: list[pathlib.Path] = []
    index = [0]

    def one(first: int, n: int) -> None:
        tsv = work / f"chunk-{index[0]:04d}-{first}-{n}.tsv"
        index[0] += 1
        reading = run_chunk(tool, tool_args, first, n, tsv, profile, memory_mib, timeout)
        killed = reading["reason"] == "memory_limit" or reading["exit"] == 137
        where = f"files [{first}, {first + n})"
        if not killed and tsv.is_file():
            print(f"  {where}: ok, {reading['seconds']:.0f}s, runner peak "
                  f"{reading['peak'] / 2**20:.0f} MiB")
            pieces.append(tsv)
            return
        if n == 1:
            # The runner's number is a real peak at this granularity, so this is a genuine
            # over-budget case rather than an accumulation artefact: keep the runner's verdict
            # and move on instead of raising anything.
            print(f"  {where}: STILL over budget as a single file "
                  f"({reading['reason']}, peak {reading['peak'] / 2**20:.0f} MiB) - recorded")
            split_at.append(first)
            if tsv.is_file():
                pieces.append(tsv)
            return
        half = n // 2
        print(f"  {where}: over budget ({reading['reason']}, peak "
              f"{reading['peak'] / 2**20:.0f} MiB) - splitting into {half} + {n - half}")
        one(first, half)
        one(first + half, n - half)

    one(start, count)
    return pieces


def merge(tsv_paths: list[pathlib.Path], out: pathlib.Path) -> list[str]:
    """Concatenate the chunk ledgers in sweep order; return the verdict tallies."""
    header, rows = None, []
    for path in tsv_paths:
        lines = [l for l in path.read_text().splitlines() if l.strip()]
        if not lines:
            continue
        if header is None:
            header = lines[0]
        rows.extend(lines[1:])
    out.write_text("\n".join([header or "# no rows", *rows]) + "\n")
    return rows


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--tool", required=True, help="the sweep tool, repository-relative")
    ap.add_argument("--tool-arg", action="append", default=[],
                    help="an argument for the tool (repeat; use --tool-arg= for a dash value)")
    ap.add_argument("--out", required=True, help="the merged ledger to write")
    ap.add_argument("--total", type=int, required=True, help="how many files the sweep has")
    ap.add_argument("--chunk", type=int, default=30)
    ap.add_argument("--start", type=int, default=0)
    ap.add_argument("--profile", default="runtime")
    ap.add_argument("--memory-mib", type=int, default=900)
    ap.add_argument("--timeout", type=int, default=3600)
    args = ap.parse_args()
    if not RUNNER.is_file():
        raise SystemExit(f"the controlled runner is missing: {RUNNER}")

    split_at: list[int] = []
    with tempfile.TemporaryDirectory(prefix="sweep-chunks-") as tmp:
        work = pathlib.Path(tmp)
        print(f"{args.total} file(s), chunks of {args.chunk}:")
        pieces = sweep(args.tool, args.tool_arg, args.start, args.total - args.start, work,
                       args.profile, args.memory_mib, args.timeout, split_at)
        rows = merge(pieces, pathlib.Path(args.out))
    print(f"\n{len(rows)} row(s) merged into {args.out}")
    if split_at:
        print(f"chunks that needed splitting down to a single file: {split_at}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
