#!/usr/bin/env python3
"""O2 / PGO / reference measurement for the PGO decision (not a DoD gate).

Why not tools/bench.sh: its inputs and outputs live at fixed /tmp paths, it folds
across rounds with a minimum before anything is reported, and its --gate compares
the port only with its own history.  This tool keeps every sample, isolates each
run, verifies the binary/dump pair before measuring, and reports median/MAD/IQR.

Usage:
  python3 tools/pgo_bench.py --o2 <phase-root> --pgo <phase-root> \
      --ref build/emacs-ref/src/emacs --out build/pgo/results [--rounds 7]

A phase root is a FEMACS_BUILD_ROOT tree containing port/src/{emacs,emacs.pdmp,
temacs,pair.sha256}.  Raw rows and the summary are written under --out.
"""
import argparse
import hashlib
import json
import math
import os
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

SCAN_CHUNK = 1 << 16
MICRO_CASES = ["bc-loop", "bc-sum", "bc-car", "regexp-repl", "buffer-scan", "sort-100k"]
TASK_CASES = ["startup", "load", "edit", "regex", "gc"]
SIDES = ["o2", "pgo", "ref"]
PAIR_FILES = ["temacs", "emacs", "emacs.pdmp", "bootstrap-emacs.pdmp"]

# femacs/lib/fingerprint.rs's placeholder array.  make-fingerprint must replace it
# exactly once per link; a surviving copy means the dump's fingerprint is not bound
# to this binary.
PLACEHOLDER = bytes([
    0xDE, 0x86, 0xBB, 0x99, 0xFF, 0xF5, 0x46, 0x9A,
    0x9E, 0x3F, 0x9F, 0x5D, 0x9A, 0xDF, 0xF0, 0x91,
    0xBD, 0xCD, 0xC1, 0xE8, 0x0C, 0x16, 0x1E, 0xAF,
    0xB8, 0x6C, 0xE2, 0x2B, 0xB1, 0x24, 0xCE, 0xB0,
])

MARKER_RE = re.compile(r"^(BENCH|TASK)\s+(\S+)\s+([0-9]+(?:\.[0-9]+)?(?:[eE][-+]?[0-9]+)?)\s+ms\s*$")


class MeasurementError(RuntimeError):
    pass


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while True:
            chunk = f.read(SCAN_CHUNK)
            if not chunk:
                break
            h.update(chunk)
    return h.hexdigest()


def count_placeholder(path, marker=PLACEHOLDER):
    """Count non-overlapping occurrences with a bounded chunk buffer."""
    total = 0
    tail = b""
    with open(path, "rb") as f:
        while True:
            chunk = f.read(SCAN_CHUNK)
            if not chunk:
                break
            data = tail + chunk
            pos = 0
            while True:
                hit = data.find(marker, pos)
                if hit < 0:
                    break
                total += 1
                pos = hit + len(marker)
            tail = data[-(len(marker) - 1):] if len(marker) > 1 else b""
    return total


def verify_pair(phase_root):
    """Recompute pair.sha256 for a phase; raise on any missing/stale artefact."""
    src = os.path.join(phase_root, "port", "src")
    manifest = os.path.join(src, "pair.sha256")
    if not os.path.isfile(manifest):
        raise MeasurementError(f"missing pair manifest: {manifest}")
    seen = {}
    with open(manifest, "r", encoding="utf-8") as f:
        for lineno, line in enumerate(f, 1):
            line = line.rstrip("\n")
            if not line:
                continue
            parts = line.split("  ")
            if len(parts) != 2 or len(parts[0]) != 64:
                raise MeasurementError(f"{manifest}:{lineno}: malformed row")
            name = parts[1]
            if name in seen:
                raise MeasurementError(f"{manifest}:{lineno}: duplicate {name}")
            seen[name] = parts[0]
    missing = [n for n in PAIR_FILES if n not in seen]
    if missing:
        raise MeasurementError(f"{manifest}: missing rows: {' '.join(missing)}")
    for name in PAIR_FILES:
        path = os.path.join(src, name)
        if not os.path.isfile(path):
            raise MeasurementError(f"missing artefact: {path}")
        actual = sha256_file(path)
        if actual != seen[name]:
            raise MeasurementError(f"stale artefact: {path} (manifest {seen[name][:12]} != {actual[:12]})")
    return src


def median(xs):
    return float(statistics.median(xs))


def mad(xs):
    m = median(xs)
    return float(statistics.median([abs(x - m) for x in xs]))


def iqr(xs):
    if len(xs) < 2:
        return 0.0
    q = statistics.quantiles(xs, n=4, method="inclusive")
    return float(q[2] - q[0])


def parse_markers(text, prefix, expected):
    found = {}
    for line in text.splitlines():
        m = MARKER_RE.match(line.strip())
        if not m or m.group(1) != prefix:
            continue
        name = m.group(2)
        value = float(m.group(3))
        if not math.isfinite(value):
            raise MeasurementError(f"non-finite {prefix} value for {name}")
        if name in found:
            raise MeasurementError(f"duplicate {prefix} line for {name}")
        found[name] = value
    missing = [n for n in expected if n not in found]
    if missing:
        raise MeasurementError(f"missing {prefix} line(s): {' '.join(missing)}")
    return found


def summarize(rows, cases, sides):
    out = []
    for case in cases:
        for side in sides:
            xs = [r["value_ms"] for r in rows if r["case"] == case and r["side"] == side]
            if not xs:
                raise MeasurementError(f"no samples for {case}/{side}")
            out.append({
                "case": case,
                "side": side,
                "n": len(xs),
                "min": min(xs),
                "median": median(xs),
                "mad": mad(xs),
                "iqr": iqr(xs),
            })
    return out


def run_command(cmd, cwd, env, timeout=300):
    started = time.perf_counter()
    try:
        proc = subprocess.run(cmd, cwd=cwd, env=env, stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE, timeout=timeout, check=False)
    except subprocess.TimeoutExpired:
        raise MeasurementError(f"timeout after {timeout}s: {' '.join(cmd)}")
    wall_ms = (time.perf_counter() - started) * 1000.0
    out = (proc.stdout + proc.stderr).decode("utf-8", "replace")
    if proc.returncode != 0:
        tail = "\n".join(out.splitlines()[-12:])
        raise MeasurementError(f"exit {proc.returncode}: {' '.join(cmd)}\n{tail}")
    lowered = out.lower()
    for fatal in ("fatal error", "segmentation fault", "abort trap"):
        if fatal in lowered:
            raise MeasurementError(f"{fatal!r} in output: {' '.join(cmd)}")
    return out, wall_ms


def isolated_env(home):
    env = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": home,
        "TMPDIR": home,
        "LC_ALL": "C",
    }
    return env


def round_order(index):
    if index % 2 == 0:
        return ["o2", "pgo", "ref"]
    return ["ref", "pgo", "o2"]


def check_codesign(path):
    proc = subprocess.run(["codesign", "--verify", path], stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, check=False)
    if proc.returncode != 0:
        raise MeasurementError(f"codesign --verify failed for {path}: "
                               f"{proc.stderr.decode('utf-8', 'replace').strip()}")


def check_pair_identity(src, label):
    if count_placeholder(os.path.join(src, "temacs")) != 0:
        raise MeasurementError(f"{label}: fingerprint placeholder survived in temacs")
    if count_placeholder(os.path.join(src, "emacs")) != 0:
        raise MeasurementError(f"{label}: fingerprint placeholder survived in emacs")
    if sha256_file(os.path.join(src, "temacs")) != sha256_file(os.path.join(src, "emacs")):
        # dump copies temacs to emacs; a mismatch means something rewrote one of them.
        raise MeasurementError(f"{label}: temacs and emacs differ before the dump copy rule")
    check_codesign(os.path.join(src, "temacs"))


def smoke(binary, cwd, env, label):
    out, _ = run_command(
        [binary, "--no-init-file", "--no-site-file", "--no-site-lisp", "--batch",
         "--eval", "(progn (require 'bytecomp) (princ \"PGO-SMOKE-OK\\n\") (kill-emacs 0))"],
        cwd, env, timeout=300)
    if "PGO-SMOKE-OK" not in out:
        raise MeasurementError(f"{label}: smoke marker missing")


def measure(args):
    roots = {"o2": os.path.abspath(args.o2), "pgo": os.path.abspath(args.pgo)}
    binaries = {
        "o2": os.path.join(roots["o2"], "port", "src", "emacs"),
        "pgo": os.path.join(roots["pgo"], "port", "src", "emacs"),
        "ref": os.path.abspath(args.ref),
    }
    for side in ("o2", "pgo"):
        verify_pair(roots[side])
        check_pair_identity(os.path.join(roots[side], "port", "src"), side)
    for side, path in binaries.items():
        if not os.path.isfile(path) or not os.access(path, os.X_OK):
            raise MeasurementError(f"{side}: not executable: {path}")

    eval_el = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pgo", "eval.el")
    if not os.path.isfile(eval_el):
        raise MeasurementError(f"missing evaluation corpus: {eval_el}")

    os.makedirs(args.out, exist_ok=True)
    raw_path = os.path.join(args.out, "raw.tsv")
    rounds_path = os.path.join(args.out, "rounds.tsv")
    summary_path = os.path.join(args.out, "summary.tsv")
    rows = []

    with open(rounds_path, "w", encoding="utf-8") as rf:
        rf.write("round\tload1\tsize\ttime\n")
        for rnd in range(1, args.rounds + 1):
            for side in ("o2", "pgo"):
                verify_pair(roots[side])
            work = tempfile.mkdtemp(prefix="pgo-bench-")
            try:
                env = isolated_env(work)
                for side in round_order(rnd - 1):
                    binary = binaries[side]
                    cwd = os.path.dirname(binary)
                    label = f"round {rnd} {side}"
                    if side == "ref":
                        check_codesign(binary) if sys.platform == "darwin" else None
                    else:
                        check_pair_identity(os.path.join(roots[side], "port", "src"), label)
                    smoke(binary, cwd, env, label)

                    out, wall = run_command(
                        [binary, "--no-init-file", "--no-site-file", "--no-site-lisp",
                         "--batch", "-l", eval_el], cwd, env)
                    bench = parse_markers(out, "BENCH", MICRO_CASES)
                    tasks = parse_markers(out, "TASK", [c for c in TASK_CASES if c != "startup"])
                    for case, value in bench.items():
                        rows.append({"case": case, "side": side, "round": rnd,
                                     "value_ms": value, "wall_ms": wall, "kind": "micro"})
                    for case, value in tasks.items():
                        rows.append({"case": case, "side": side, "round": rnd,
                                     "value_ms": value, "wall_ms": wall, "kind": "task"})

                    _, startup_wall = run_command(
                        [binary, "--no-init-file", "--no-site-file", "--no-site-lisp",
                         "--batch", "--eval", "(kill-emacs 0)"], cwd, env)
                    rows.append({"case": "startup", "side": side, "round": rnd,
                                 "value_ms": startup_wall, "wall_ms": startup_wall,
                                 "kind": "startup"})
            finally:
                shutil.rmtree(work, ignore_errors=True)
            try:
                with open("/proc/loadavg", "r", encoding="utf-8") as f:
                    load1 = f.read().split()[0]
            except OSError:
                load1 = ""
                try:
                    load1 = subprocess.check_output(["sysctl", "-n", "vm.loadavg"],
                                                    text=True).split()[1]
                except (OSError, subprocess.SubprocessError, IndexError):
                    pass
            rf.write(f"{rnd}\t{load1}\t{len(rows)}\t{time.strftime('%Y-%m-%dT%H:%M:%S')}\n")
            print(f"round {rnd}/{args.rounds}: {len(rows)} samples", flush=True)

    with open(raw_path, "w", encoding="utf-8") as f:
        f.write("case\tside\tround\tvalue_ms\twall_ms\tkind\n")
        for r in rows:
            f.write(f"{r['case']}\t{r['side']}\t{r['round']}\t{r['value_ms']:.6f}\t"
                    f"{r['wall_ms']:.6f}\t{r['kind']}\n")

    summary = summarize(rows, MICRO_CASES + TASK_CASES, SIDES)
    by_key = {(s["case"], s["side"]): s for s in summary}
    with open(summary_path, "w", encoding="utf-8") as f:
        f.write("case\tside\tn\tmin\tmedian\tmad\tiqr\tvs_o2\tvs_ref\n")
        for s in summary:
            vs_o2 = by_key[(s["case"], "o2")]["median"]
            vs_ref = by_key[(s["case"], "ref")]["median"]
            f.write(f"{s['case']}\t{s['side']}\t{s['n']}\t{s['min']:.6f}\t{s['median']:.6f}\t"
                    f"{s['mad']:.6f}\t{s['iqr']:.6f}\t{s['median']/vs_o2:.4f}\t"
                    f"{s['median']/vs_ref:.4f}\n")

    print("\ncase         side  median(ms)   MAD   vs O2  vs ref")
    for s in summary:
        m = s["median"]
        print(f"{s['case']:<12} {s['side']:<4} {m:10.4f} {s['mad']:7.4f} "
              f"{m / by_key[(s['case'], 'o2')]['median']:6.3f} "
              f"{m / by_key[(s['case'], 'ref')]['median']:6.3f}")
    print(f"\nwrote {raw_path}\nwrote {summary_path}\nwrote {rounds_path}")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--o2", required=True, help="O2 phase root (contains port/src)")
    ap.add_argument("--pgo", required=True, help="PGO phase root (contains port/src)")
    ap.add_argument("--ref", default="build/emacs-ref/src/emacs")
    ap.add_argument("--out", default="build/pgo/results")
    ap.add_argument("--rounds", type=int, default=7)
    args = ap.parse_args(argv)
    if args.rounds < 1:
        raise SystemExit("--rounds must be positive")
    try:
        return measure(args)
    except MeasurementError as e:
        print(f"pgo-bench: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
