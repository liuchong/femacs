#!/usr/bin/env python3
"""Build/train/merge/measure driver for the PGO decision.

Every phase is a separate FEMACS_BUILD_ROOT; the default O2 tree is never used as
a PGO output.  Heavy commands (cargo/rustc/the reference program) must be launched through the
controlled-run executor in the centralized pass; this script only sequences them.

  python3 tools/pgo_pipeline.py o2
  python3 tools/pgo_pipeline.py generate
  python3 tools/pgo_pipeline.py train
  python3 tools/pgo_pipeline.py merge
  python3 tools/pgo_pipeline.py use
  python3 tools/pgo_pipeline.py verify
  python3 tools/pgo_pipeline.py measure [--rounds 7]
  python3 tools/pgo_pipeline.py all
"""
import argparse
import glob
import glob
import os
import pwd
import shutil
import subprocess
import sys

import pgo_bench

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FEMACS = os.path.join(REPO, "femacs")
PGO = os.path.join(REPO, "build", "pgo")
O2_ROOT = os.path.join(REPO, "build")
GENERATE_ROOT = os.path.join(PGO, "generate")
USE_ROOT = os.path.join(PGO, "use")
RESULTS = os.path.join(PGO, "results")


def run(cmd, cwd, env=None, timeout=None):
    print("+", " ".join(cmd), flush=True)
    proc = subprocess.run(cmd, cwd=cwd, env=env, timeout=timeout, check=False)
    if proc.returncode != 0:
        raise RuntimeError(f"command failed ({proc.returncode}): {' '.join(cmd)}")


def xtask(command, root=None, pgo=None):
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = "/tmp/xt-cargo"
    if root is not None:
        env["FEMACS_BUILD_ROOT"] = os.path.abspath(root)
    if pgo is not None:
        env["FEMACS_PGO"] = pgo
    else:
        env.pop("FEMACS_PGO", None)
    run(["cargo", "run", "-q", "-p", "xtask", "--", command], FEMACS, env)


def llvm_profdata():
    candidates = []
    # The executor may replace HOME, so expanduser("~") can point at a nonexistent tree.
    # Resolve the login home as well; the profile format follows the toolchain that
    # compiled the objects, not whatever system llvm-profdata happens to be first.
    home = os.path.expanduser("~")
    if not os.path.isfile(os.path.join(home, ".cargo", "bin", "rustc")):
        try:
            home = pwd.getpwuid(os.getuid()).pw_dir
        except KeyError:
            pass
    rustc = os.path.join(home, ".cargo", "bin", "rustc")
    if not os.path.isfile(rustc):
        rustc = shutil.which("rustc") or "rustc"
    try:
        sysroot = subprocess.check_output([rustc, "--print", "sysroot"], text=True, cwd=FEMACS).strip()
        host = ""
        for line in subprocess.check_output([rustc, "-vV"], text=True, cwd=FEMACS).splitlines():
            if line.startswith("host:"):
                host = line.split(":", 1)[1].strip()
        if host:
            candidates.append(os.path.join(sysroot, "lib", "rustlib", host, "bin", "llvm-profdata"))
    except (OSError, subprocess.SubprocessError):
        pass
    candidates += sorted(glob.glob(os.path.join(
        home, ".rustup", "toolchains", "*", "lib", "rustlib", "*", "bin", "llvm-profdata")))
    candidates += [
        "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/llvm-profdata",
        "/opt/homebrew/opt/llvm/bin/llvm-profdata",
    ]
    for path in candidates:
        if os.path.isfile(path) and os.access(path, os.X_OK):
            print(f"llvm-profdata: {path}", flush=True)
            return path
    found = shutil.which("llvm-profdata")
    if found:
        return found
    raise RuntimeError("llvm-profdata not found (rustc sysroot, Xcode, Homebrew, PATH)")


def o2():
    xtask("build-units", O2_ROOT)
    xtask("gate-load", O2_ROOT)
    xtask("dump", O2_ROOT)


def generate():
    raw = os.path.join(GENERATE_ROOT, "profiles", "raw")
    os.makedirs(raw, exist_ok=True)
    xtask("build-units", GENERATE_ROOT, f"generate:{raw}")
    xtask("gate-load", GENERATE_ROOT, f"generate:{raw}")
    xtask("dump", GENERATE_ROOT, f"generate:{raw}")
    # gate and dump runs are instrumented too, but their profiles are evidence of the
    # build machinery, not training data: never merge them.
    for role in ("gate", "dump", "raw"):
        shutil.rmtree(os.path.join(GENERATE_ROOT, "profiles", role), ignore_errors=True)


def train():
    src = os.path.join(GENERATE_ROOT, "port", "src")
    binary = os.path.join(src, "emacs")
    if not os.path.isfile(binary):
        raise RuntimeError(f"missing generate binary: {binary}")
    pgo_bench.verify_pair(GENERATE_ROOT)
    train_dir = os.path.join(GENERATE_ROOT, "profiles", "train")
    shutil.rmtree(train_dir, ignore_errors=True)
    os.makedirs(train_dir, exist_ok=True)
    env = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": train_dir,
        "TMPDIR": train_dir,
        "LC_ALL": "C",
        "LLVM_PROFILE_FILE": os.path.join(train_dir, "%p-%m.profraw"),
    }
    corpus = os.path.join(REPO, "tools", "pgo", "train.el")
    out = subprocess.run(
        [binary, "--no-init-file", "--no-site-file", "--no-site-lisp", "--batch", "-l", corpus],
        cwd=src, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    text = (out.stdout + out.stderr).decode("utf-8", "replace")
    if out.returncode != 0 or "PGO-TRAIN-DONE" not in text:
        tail = "\n".join(text.splitlines()[-20:])
        raise RuntimeError(f"training run failed ({out.returncode})\n{tail}")
    print("training completed")


def merge():
    train_dir = os.path.join(GENERATE_ROOT, "profiles", "train")
    profiles = sorted(glob.glob(os.path.join(train_dir, "*.profraw")))
    if not profiles:
        raise RuntimeError(f"no training profiles under {train_dir}")
    empty = [p for p in profiles if os.path.getsize(p) == 0]
    if empty:
        raise RuntimeError(f"empty training profile(s): {' '.join(empty)}")
    merged = os.path.join(GENERATE_ROOT, "merged.profdata")
    run([llvm_profdata(), "merge", "--failure-mode=all", "-o", merged] + profiles,
        REPO, timeout=600)
    if not os.path.isfile(merged) or os.path.getsize(merged) == 0:
        raise RuntimeError(f"llvm-profdata produced no data: {merged}")
    os.makedirs(USE_ROOT, exist_ok=True)
    shutil.copy2(merged, os.path.join(USE_ROOT, "merged.profdata"))
    print(f"merged {len(profiles)} profile(s) -> {merged}")


def use():
    merged = os.path.join(USE_ROOT, "merged.profdata")
    if not os.path.isfile(merged):
        raise RuntimeError(f"missing merged profile: {merged}")
    xtask("build-units", USE_ROOT, f"use:{merged}")
    xtask("gate-load", USE_ROOT, f"use:{merged}")
    xtask("dump", USE_ROOT, f"use:{merged}")


def verify():
    for label, root in (("o2", O2_ROOT), ("generate", GENERATE_ROOT), ("use", USE_ROOT)):
        pgo_bench.verify_pair(root)
        src = os.path.join(root, "port", "src")
        pgo_bench.check_pair_identity(src, label)
        env = {"PATH": "/usr/bin:/bin:/usr/sbin:/sbin", "HOME": src, "TMPDIR": src, "LC_ALL": "C"}
        pgo_bench.smoke(os.path.join(src, "emacs"), src, env, label)
        print(f"{label}: pair, fingerprint, signature and smoke ok")


def measure(rounds):
    run([sys.executable, os.path.join(REPO, "tools", "pgo_bench.py"),
         "--o2", O2_ROOT, "--pgo", USE_ROOT,
         "--ref", os.path.join(REPO, "build", "emacs-ref", "src", "emacs"),
         "--out", RESULTS, "--rounds", str(rounds)], REPO)


def all_steps(rounds):
    o2()
    generate()
    train()
    merge()
    use()
    verify()
    measure(rounds)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("step", choices=["o2", "generate", "train", "merge", "use", "verify",
                                     "measure", "all"])
    ap.add_argument("--rounds", type=int, default=7)
    args = ap.parse_args(argv)
    try:
        if args.step == "o2":
            o2()
        elif args.step == "generate":
            generate()
        elif args.step == "train":
            train()
        elif args.step == "merge":
            merge()
        elif args.step == "use":
            use()
        elif args.step == "verify":
            verify()
        elif args.step == "measure":
            measure(args.rounds)
        else:
            all_steps(args.rounds)
    except (RuntimeError, pgo_bench.MeasurementError) as e:
        print(f"pgo-pipeline: {e}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
