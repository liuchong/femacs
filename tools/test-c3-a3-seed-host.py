#!/usr/bin/env python3
"""Phase II Stage C3/A3 dual-host Seed, Stage 0, and verifiable host cache acceptance.

All tests against the downstream compiler repository run on isolated read-only
`git archive HEAD` snapshots under `--out`; the downstream repository's active
working tree is never modified.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CACHE_TOOL = REPO / "tools" / "femacs-lisp-cache.py"


def run_cmd(
    argv: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    timeout: int = 300,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=str(cwd) if cwd else None,
        env=env,
        text=True,
        capture_output=True,
        timeout=timeout,
    )


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        while True:
            chunk = fh.read(65536)
            if not chunk:
                break
            h.update(chunk)
    return h.hexdigest()


def collect_regular_files(root: Path, *, skip_symlink_dirs: tuple[str, ...] = ("node_modules/eliscript",)) -> dict[str, bytes]:
    if not root.is_dir():
        raise AssertionError(f"missing output directory: {root}")
    out: dict[str, bytes] = {}
    skip_set = set(skip_symlink_dirs)

    def walk(cur: Path) -> None:
        for entry in sorted(cur.iterdir(), key=lambda p: p.name):
            rel = entry.relative_to(root).as_posix()
            if rel in skip_set:
                if not entry.is_symlink():
                    raise AssertionError(f"expected symlink at skipped entry {entry}")
                continue
            if entry.is_symlink():
                raise AssertionError(f"unexpected symlink in generated tree: {entry}")
            if entry.is_dir():
                walk(entry)
            elif entry.is_file():
                out[rel] = entry.read_bytes()
            else:
                raise AssertionError(f"unexpected non-regular entry in generated tree: {entry}")

    walk(root)
    return out


def compare_tree_bytes(left: Path, right: Path, *, label: str) -> tuple[int, int, dict[str, str]]:
    left_files = collect_regular_files(left)
    right_files = collect_regular_files(right)
    if sorted(left_files.keys()) != sorted(right_files.keys()):
        only_left = sorted(set(left_files.keys()) - set(right_files.keys()))
        only_right = sorted(set(right_files.keys()) - set(left_files.keys()))
        raise AssertionError(
            f"{label} file list mismatch: only_left={only_left[:8]}, only_right={only_right[:8]}"
        )
    if not left_files:
        raise AssertionError(f"{label} produced zero files")

    diffs: list[str] = []
    hashes: dict[str, str] = {}
    total_bytes = 0
    for rel in sorted(left_files.keys()):
        lb = left_files[rel]
        rb = right_files[rel]
        total_bytes += len(lb)
        if lb != rb:
            diffs.append(f"{rel} (left={len(lb)}B, right={len(rb)}B)")
        hashes[rel] = hashlib.sha256(lb).hexdigest()

    if diffs:
        raise AssertionError(f"{label} byte parity diffs ({len(diffs)}): {', '.join(diffs[:6])}")
    return len(left_files), total_bytes, hashes


def isolated_env(home_dir: Path) -> dict[str, str]:
    home_dir.mkdir(parents=True, exist_ok=True)
    return {
        "PATH": "/usr/bin:/bin",
        "HOME": str(home_dir),
        "LANG": "en_US.UTF-8",
        "LC_ALL": "en_US.UTF-8",
        "ELISCRIPT_JS_RUNTIME": "/nonexistent/no-js",
    }


def reference_env(home_dir: Path) -> dict[str, str]:
    home_dir.mkdir(parents=True, exist_ok=True)
    return {
        "PATH": os.environ.get("PATH", "/opt/homebrew/bin:/usr/bin:/bin"),
        "HOME": str(home_dir),
        "LANG": "en_US.UTF-8",
        "LC_ALL": "en_US.UTF-8",
        "ELISCRIPT_JS_RUNTIME": "/nonexistent/no-js",
    }


def extract_clean_git_snapshot(repo: Path, dst: Path) -> str:
    rev = run_cmd(["git", "-C", str(repo), "rev-parse", "HEAD"], timeout=30)
    if rev.returncode != 0:
        raise AssertionError(f"git rev-parse failed for {repo}: {rev.stderr}")
    commit = rev.stdout.strip()
    if len(commit) < 7:
        raise AssertionError(f"unexpected commit sha {commit!r}")

    dst = dst.resolve()
    dst.mkdir(parents=True, exist_ok=True)
    tar_path = dst.parent / f".{dst.name}.tar"
    arch = subprocess.run(
        ["git", "-C", str(repo), "archive", "--format=tar", f"--output={tar_path}", "HEAD"],
        text=True,
        capture_output=True,
        timeout=60,
    )
    if arch.returncode != 0:
        raise AssertionError(f"git archive failed for {repo}: {arch.stderr}")
    try:
        unpacked = run_cmd(["tar", "-xf", str(tar_path), "-C", str(dst)], timeout=60)
        if unpacked.returncode != 0:
            raise AssertionError(f"tar extract failed for {repo}: {unpacked.stderr}")
    finally:
        tar_path.unlink(missing_ok=True)
    return commit


def verify_codesign_if_darwin(binary: Path) -> None:
    if sys.platform != "darwin":
        return
    r = run_cmd(["codesign", "--verify", "--strict", str(binary)], timeout=30)
    if r.returncode != 0:
        raise AssertionError(f"codesign --verify --strict failed for {binary}: {r.stderr}")


def check_cli_contract(lisp_bin: Path, out_dir: Path, clean_env: dict[str, str]) -> dict[str, object]:
    ver = run_cmd([str(lisp_bin), "--version"], env=clean_env, timeout=30)
    if ver.returncode != 0 or ver.stdout.strip() != "femacs-lisp 0.1.0 (femacs 32.0.50)":
        raise AssertionError(f"unexpected --version output: rc={ver.returncode} out={ver.stdout!r} err={ver.stderr!r}")

    ident = run_cmd(
        [
            str(lisp_bin),
            "--batch",
            "-Q",
            "--eval",
            '(princ (json-serialize (list (cons (quote name) invocation-name) '
            '(cons (quote dir) (invocation-directory)) '
            '(cons (quote noninteractive) noninteractive))))',
        ],
        env=clean_env,
        timeout=30,
    )
    if ident.returncode != 0:
        raise AssertionError(f"identity probe failed: rc={ident.returncode} err={ident.stderr!r}")
    ident_json = json.loads(ident.stdout)
    assert ident_json["noninteractive"] is True, f"expected noninteractive=true, got {ident_json}"
    assert ident_json["name"] == "femacs-lisp", f"expected femacs-lisp, got {ident_json}"
    assert ident_json["dir"].rstrip("/").endswith("/bin"), f"unexpected invocation-dir {ident_json}"

    bad_bare = run_cmd([str(lisp_bin), "--batch", "--no-such-flag-c3"], env=clean_env, timeout=30)
    if bad_bare.returncode != 2:
        raise AssertionError(f"expected exit 2 on --no-such-flag-c3, got rc={bad_bare.returncode}")

    noop_el = out_dir / "noop.el"
    noop_el.write_text("(defvar c3-noop t)\n", encoding="utf-8")
    bad_left = run_cmd(
        [str(lisp_bin), "--batch", "-Q", "-l", str(noop_el), "--unconsumed-flag"],
        env=clean_env,
        timeout=30,
    )
    if bad_left.returncode != 2 or "--unconsumed-flag" not in bad_left.stderr:
        raise AssertionError(f"expected exit 2 for unconsumed flag, got rc={bad_left.returncode} err={bad_left.stderr!r}")

    consume_el = out_dir / "consume-some.el"
    consume_el.write_text(
        '(setq command-line-args-left (delete "--consume-me" command-line-args-left))\n',
        encoding="utf-8",
    )
    partial_left = run_cmd(
        [str(lisp_bin), "--batch", "-Q", "-l", str(consume_el), "--consume-me", "--still-left"],
        env=clean_env,
        timeout=30,
    )
    if partial_left.returncode != 2 or "--still-left" not in partial_left.stderr:
        raise AssertionError(
            f"expected exit 2 for remaining unconsumed arg, got rc={partial_left.returncode} err={partial_left.stderr!r}"
        )

    consumed_then_eval = run_cmd(
        [str(lisp_bin), "--batch", "-Q", "-l", str(consume_el), "--consume-me", "-e", '(princ "after-ok")'],
        env=clean_env,
        timeout=30,
    )
    if consumed_then_eval.returncode != 0 or consumed_then_eval.stdout != "after-ok":
        raise AssertionError(
            f"expected exit 0 after arg consumption + eval, got rc={consumed_then_eval.returncode} out={consumed_then_eval.stdout!r} err={consumed_then_eval.stderr!r}"
        )

    return ident_json


def check_seed_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_femacs: Path,
    snap_ref: Path,
    out_dir: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> tuple[int, int, dict[str, str]]:
    seed_femacs = snap_femacs.parent / "seed"
    seed_ref = snap_ref.parent / "seed"
    modules = ["reader", "expander", "analyzer", "emitter"]

    r_f = run_cmd(
        [
            str(lisp_bin),
            "--batch",
            "-Q",
            "--script",
            str(snap_femacs / "tools" / "bootstrap-seed.el"),
            "--",
            str(snap_femacs),
            str(seed_femacs),
            *modules,
        ],
        cwd=snap_femacs,
        env=clean_env,
        timeout=180,
    )
    if r_f.returncode != 0:
        raise AssertionError(f"femacs-lisp bootstrap-seed failed (rc={r_f.returncode}):\n{r_f.stdout}\n{r_f.stderr}")

    r_r = run_cmd(
        [
            str(ref_bin),
            "--batch",
            "-Q",
            "--script",
            str(snap_ref / "tools" / "bootstrap-seed.el"),
            "--",
            str(snap_ref),
            str(seed_ref),
            *modules,
        ],
        cwd=snap_ref,
        env=ref_env,
        timeout=180,
    )
    if r_r.returncode != 0:
        raise AssertionError(f"reference host bootstrap-seed failed (rc={r_r.returncode}):\n{r_r.stdout}\n{r_r.stderr}")

    return compare_tree_bytes(seed_femacs, seed_ref, label="seed-bootstrap")


def check_diagnostic_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_femacs: Path,
    snap_ref: Path,
    out_dir: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> list[dict[str, object]]:
    # 1. bootstrap-seed invalid module name diagnostic + exit code
    r_f = run_cmd(
        [
            str(lisp_bin),
            "--batch",
            "-Q",
            "--script",
            str(snap_femacs / "tools" / "bootstrap-seed.el"),
            "--",
            str(snap_femacs),
            str(out_dir / "seed-bad-femacs"),
            "BAD_NAME",
        ],
        cwd=snap_femacs,
        env=clean_env,
        timeout=60,
    )
    r_r = run_cmd(
        [
            str(ref_bin),
            "--batch",
            "-Q",
            "--script",
            str(snap_ref / "tools" / "bootstrap-seed.el"),
            "--",
            str(snap_ref),
            str(out_dir / "seed-bad-ref"),
            "BAD_NAME",
        ],
        cwd=snap_ref,
        env=ref_env,
        timeout=60,
    )
    if r_f.returncode != 1 or r_r.returncode != 1:
        raise AssertionError(f"invalid module exit code mismatch: femacs={r_f.returncode}, ref={r_r.returncode}")
    if r_f.stderr.strip() != r_r.stderr.strip() or "invalid bootstrap module name: BAD_NAME" not in r_f.stderr:
        raise AssertionError(f"invalid module stderr mismatch: femacs={r_f.stderr!r}, ref={r_r.stderr!r}")

    # 2. Source-located syntax & analyzer error parity via compiler entry
    cases = [
        ("syntax-eof", "(defconst ok 1)\n\n  (+ 1 2\n", "fixture/bad-syntax.eli"),
        ("duplicate-let-binding", "(defconst ok 1)\n(let ((dup 1)\n      (dup 2))\n  dup)\n", "fixture/bad-analyzer.eli"),
    ]
    results: list[dict[str, object]] = []
    for name, src_text, virtual_name in cases:
        elisp = (
            "(progn (require (quote eliscript)) "
            "(condition-case err "
            f"(progn (eliscript-compile-string-with-source-map {json.dumps(src_text)} {json.dumps(virtual_name)} "
            '"fixture/out.mjs" '
            f"{json.dumps(virtual_name)}) (kill-emacs 0)) "
            '(error (princ (format "DIAG:%s\\n" (eliscript-diagnostic-condition-message err))) '
            "(kill-emacs 1))))"
        )
        rf = run_cmd(
            [str(lisp_bin), "--batch", "-Q", "-L", str(snap_femacs / "compiler"), "--eval", elisp],
            env=clean_env,
            timeout=60,
        )
        rr = run_cmd(
            [str(ref_bin), "--batch", "-Q", "-L", str(snap_ref / "compiler"), "--eval", elisp],
            env=ref_env,
            timeout=60,
        )
        if rf.returncode != 1 or rr.returncode != 1:
            raise AssertionError(f"diagnostic case {name} exit mismatch: femacs={rf.returncode}, ref={rr.returncode}")
        df = rf.stdout.strip()
        dr = rr.stdout.strip()
        if not df.startswith("DIAG:") or df != dr:
            raise AssertionError(f"diagnostic case {name} output mismatch: femacs={df!r}, ref={dr!r}")
        if virtual_name not in df or ":" not in df:
            raise AssertionError(f"diagnostic case {name} missing source location: {df!r}")
        results.append({"case": name, "exit_code": 1, "diagnostic": df})
    return results


def check_stage0_parity(
    lisp_bin: Path,
    ref_bin: Path,
    snap_femacs: Path,
    snap_ref: Path,
    out_dir: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> dict[str, object]:
    st_femacs = snap_femacs.parent / "stage0"
    st_ref = snap_ref.parent / "stage0"

    rf = run_cmd(
        [
            str(lisp_bin),
            "--batch",
            "-Q",
            "-l",
            "tools/backends/bootstrap.el",
            "--stage0",
            "--output",
            str(st_femacs),
        ],
        cwd=snap_femacs,
        env=clean_env,
        timeout=300,
    )
    if rf.returncode != 0:
        raise AssertionError(f"femacs-lisp --stage0 failed (rc={rf.returncode}):\n{rf.stdout}\n{rf.stderr}")

    rr = run_cmd(
        [
            str(ref_bin),
            "--batch",
            "-Q",
            "-l",
            "tools/backends/bootstrap.el",
            "--stage0",
            "--output",
            str(st_ref),
        ],
        cwd=snap_ref,
        env=ref_env,
        timeout=300,
    )
    if rr.returncode != 0:
        raise AssertionError(f"reference host --stage0 failed (rc={rr.returncode}):\n{rr.stdout}\n{rr.stderr}")

    # Verify node_modules/eliscript symlinks point to each snapshot root before tree comparison
    link_f = st_femacs / "node_modules" / "eliscript"
    link_r = st_ref / "node_modules" / "eliscript"
    if not link_f.is_symlink() or link_f.resolve() != snap_femacs.resolve():
        raise AssertionError(f"stage0-femacs node_modules/eliscript symlink mismatch: {link_f} -> {link_f.resolve()}")
    if not link_r.is_symlink() or link_r.resolve() != snap_ref.resolve():
        raise AssertionError(f"stage0-ref node_modules/eliscript symlink mismatch: {link_r} -> {link_r.resolve()}")

    file_count, total_bytes, _ = compare_tree_bytes(st_femacs, st_ref, label="stage0")
    st_json_f = json.loads((st_femacs / "stage0.json").read_text(encoding="utf-8"))
    st_json_r = json.loads((st_ref / "stage0.json").read_text(encoding="utf-8"))
    assert st_json_f == st_json_r
    assert st_json_f.get("status") == "built"

    return {
        "file_count": file_count,
        "total_bytes": total_bytes,
        "formal_source_sha256": st_json_f["formalSourceIdentity"]["sha256"],
        "product_sha256": st_json_f["productIdentity"]["sha256"],
        "stage0_json_sha256": sha256_file(st_femacs / "stage0.json"),
        "compiler_artifact_json_sha256": sha256_file(st_femacs / "compiler-artifact.json"),
    }


def check_makefile_hooks(
    lisp_bin: Path,
    ref_bin: Path,
    snap_femacs: Path,
    snap_ref: Path,
    clean_env: dict[str, str],
    ref_env: dict[str, str],
) -> None:
    env_f = {**clean_env, "EMACS": str(lisp_bin), "PATH": "/usr/bin:/bin"}
    env_r = {**ref_env, "EMACS": str(ref_bin)}

    # snap_femacs still holds generated runtime artifacts from stage0 -> clean --check should exit 1
    pre = run_cmd([str(snap_femacs / "bin" / "eliscript-clean"), "--check", "--quiet"], cwd=snap_femacs, env=env_f, timeout=60)
    if pre.returncode != 1:
        raise AssertionError(f"expected dirty pre-clean check exit 1, got rc={pre.returncode}")

    do_clean_f = run_cmd([str(snap_femacs / "bin" / "eliscript-clean"), "--quiet"], cwd=snap_femacs, env=env_f, timeout=60)
    if do_clean_f.returncode != 0:
        raise AssertionError(f"eliscript-clean failed on femacs-lisp: {do_clean_f.stderr}")
    post_f = run_cmd([str(snap_femacs / "bin" / "eliscript-clean"), "--check", "--quiet"], cwd=snap_femacs, env=env_f, timeout=60)
    if post_f.returncode != 0:
        raise AssertionError(f"post-clean check failed on femacs-lisp: {post_f.stderr}")

    do_clean_r = run_cmd([str(snap_ref / "bin" / "eliscript-clean"), "--quiet"], cwd=snap_ref, env=env_r, timeout=60)
    if do_clean_r.returncode != 0:
        raise AssertionError(f"eliscript-clean failed on ref host: {do_clean_r.stderr}")

    # Strict byte-compile check on both hosts (runs in bash, so keep /bin and /usr/bin in PATH, NO /opt/homebrew/bin for femacs)
    bc_f = run_cmd(["/bin/bash", str(snap_femacs / "tools" / "ci" / "strict-byte-compile.sh")], cwd=snap_femacs, env=env_f, timeout=180)
    if bc_f.returncode != 0:
        raise AssertionError(f"strict-byte-compile failed on femacs-lisp (rc={bc_f.returncode}):\n{bc_f.stdout}\n{bc_f.stderr}")

    bc_r = run_cmd(["/bin/bash", str(snap_ref / "tools" / "ci" / "strict-byte-compile.sh")], cwd=snap_ref, env=env_r, timeout=180)
    if bc_r.returncode != 0:
        raise AssertionError(f"strict-byte-compile failed on ref host (rc={bc_r.returncode}):\n{bc_r.stdout}\n{bc_r.stderr}")


def check_cache_lifecycle(
    pkg_dir: Path,
    manifest: dict[str, object],
    snap_femacs: Path,
    out_dir: Path,
    clean_env: dict[str, str],
) -> dict[str, object]:
    cache_dir = out_dir / "host-cache"
    commit = str(manifest["source_commit"])
    target = str(manifest["target"])
    cfg = str(manifest["build_config"])
    pkg_sha = str(manifest["package_sha256"])

    # 1. Cold cache populate
    r1 = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "ensure",
            "--cache-dir",
            str(cache_dir),
            "--source-package",
            str(pkg_dir),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--json",
        ],
        timeout=60,
    )
    if r1.returncode != 0:
        raise AssertionError(f"cache ensure cold failed: {r1.stdout} {r1.stderr}")
    cold_res = json.loads(r1.stdout)
    assert cold_res["status"] == "rebuilt", f"expected rebuilt, got {cold_res}"
    cached_bin = Path(cold_res["lisp_bin"])
    cached_entry = Path(cold_res["cache_entry"])
    assert cached_entry.name == pkg_sha, f"expected full 64-hex cache entry name, got {cached_entry.name}"
    mtime_before = cached_bin.stat().st_mtime_ns

    # Run Seed from cached host and prove parity against reference seed output
    seed_cached = snap_femacs.parent / "seed-cached"
    rs = run_cmd(
        [
            str(cached_bin),
            "--batch",
            "-Q",
            "--script",
            str(snap_femacs / "tools" / "bootstrap-seed.el"),
            "--",
            str(snap_femacs),
            str(seed_cached),
            "reader",
            "expander",
            "analyzer",
            "emitter",
        ],
        cwd=snap_femacs,
        env=clean_env,
        timeout=180,
    )
    if rs.returncode != 0:
        raise AssertionError(f"cached femacs-lisp Seed failed: {rs.stderr}")
    compare_tree_bytes(seed_cached, out_dir / "host-ref" / "seed", label="cached-seed-vs-ref")

    # 2. Warm cache hit
    r2 = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "ensure",
            "--cache-dir",
            str(cache_dir),
            "--source-package",
            str(pkg_dir),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--json",
        ],
        timeout=60,
    )
    if r2.returncode != 0:
        raise AssertionError(f"cache ensure warm failed: {r2.stdout} {r2.stderr}")
    warm_res = json.loads(r2.stdout)
    assert warm_res["status"] == "hit"
    assert cached_bin.stat().st_mtime_ns == mtime_before, "warm cache hit modified binary mtime"

    # 3. Direct resolve verification
    r3 = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(cache_dir),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            pkg_sha,
            "--json",
        ],
        timeout=60,
    )
    if r3.returncode != 0:
        raise AssertionError(f"cache resolve failed: {r3.stdout} {r3.stderr}")
    assert json.loads(r3.stdout)["status"] == "verified"

    # 4. Negative rejections on wrong identity & tampered cached entries
    bad_target = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(cache_dir),
            "--expect-commit",
            commit,
            "--expect-target",
            "x86_64-unknown-linux-gnu",
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            pkg_sha,
        ],
        timeout=30,
    )
    assert bad_target.returncode != 0, "expected wrong target rejection"

    bad_commit = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(cache_dir),
            "--expect-commit",
            "0" * 40,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            pkg_sha,
        ],
        timeout=30,
    )
    assert bad_commit.returncode != 0, "expected wrong commit rejection"

    bad_pkg_sha = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(cache_dir),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            "0" * 64,
        ],
        timeout=30,
    )
    assert bad_pkg_sha.returncode != 0, "expected wrong package_sha256 rejection"

    tamper_cache = out_dir / "tamper-cache"
    shutil.copytree(cache_dir, tamper_cache, symlinks=False)
    tampered_entry = tamper_cache / cached_entry.relative_to(cache_dir)

    loadup = tampered_entry / "lisp" / "loadup.el"
    orig_loadup = loadup.read_bytes()
    loadup.write_bytes(orig_loadup + b"; tamper\n")
    tamper_lisp = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(tamper_cache),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            pkg_sha,
        ],
        timeout=30,
    )
    assert tamper_lisp.returncode != 0, "expected tampered lisp/ rejection"
    loadup.write_bytes(orig_loadup)

    extra_rt = tampered_entry / "runtime-extra"
    extra_rt.mkdir()
    (extra_rt / "evil.dylib").write_bytes(b"evil")
    tamper_extra_rt = run_cmd(
        [
            sys.executable,
            str(CACHE_TOOL),
            "resolve",
            "--cache-dir",
            str(tamper_cache),
            "--expect-commit",
            commit,
            "--expect-target",
            target,
            "--expect-build-config",
            cfg,
            "--expect-package-sha256",
            pkg_sha,
        ],
        timeout=30,
    )
    assert tamper_extra_rt.returncode != 0, "expected unverified runtime-extra/ rejection"
    shutil.rmtree(extra_rt)

    spoof_pkg = out_dir / "tamper-spoof-pkg"
    shutil.copytree(tampered_entry, spoof_pkg, symlinks=False)
    spoof_manifest = json.loads((spoof_pkg / "manifest.json").read_text(encoding="utf-8"))
    target_loadup = spoof_pkg / "lisp" / "loadup.el"
    spoof_manifest["binaries"]["femacs-lisp"] = {
        "bytes": target_loadup.stat().st_size,
        "relative_path": "lisp/loadup.el",
        "sha256": sha256_file(target_loadup),
    }
    canonical = {
        "binaries": spoof_manifest["binaries"],
        "build_config": spoof_manifest["build_config"],
        "resource_trees": spoof_manifest["resource_trees"],
        "source_commit": spoof_manifest["source_commit"],
        "target": spoof_manifest["target"],
    }
    spoof_manifest["package_sha256"] = hashlib.sha256(
        json.dumps(canonical, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    ).hexdigest()
    (spoof_pkg / "manifest.json").write_text(json.dumps(spoof_manifest, indent=2) + "\n", encoding="utf-8")
    tamper_spoof = run_cmd([sys.executable, str(CACHE_TOOL), "verify", "--package", str(spoof_pkg)], timeout=30)
    assert tamper_spoof.returncode != 0, "expected spoofed binary relative_path rejection"

    pdmp = tampered_entry / "bin" / "femacs-lisp.pdmp"
    orig_pdmp = bytearray(pdmp.read_bytes())
    orig_pdmp[-1] ^= 0x5A
    pdmp.write_bytes(bytes(orig_pdmp))
    tamper_pdmp = run_cmd([sys.executable, str(CACHE_TOOL), "verify", "--package", str(tampered_entry)], timeout=30)
    assert tamper_pdmp.returncode != 0, "expected tampered .pdmp rejection"
    orig_pdmp[-1] ^= 0x5A
    pdmp.write_bytes(bytes(orig_pdmp))

    return {
        "cold_status": cold_res["status"],
        "warm_status": warm_res["status"],
        "package_sha256": pkg_sha,
        "source_commit": commit,
        "target": target,
        "build_config": cfg,
    }


def claim_fresh_output_dir(out_arg: Path) -> Path:
    if out_arg.is_symlink() or out_arg.exists():
        raise AssertionError(f"--out destination already exists or is a symlink; pass a new directory: {out_arg}")
    out_dir = out_arg.resolve()
    if out_dir.is_symlink() or out_dir.exists():
        raise AssertionError(f"--out destination already exists; pass a new directory: {out_dir}")
    out_dir.parent.mkdir(parents=True, exist_ok=True)
    out_dir.mkdir(exist_ok=False)
    return out_dir


def run_selftest() -> int:
    with tempfile.TemporaryDirectory(prefix="femacs-c3-selftest-") as tmp:
        root = Path(tmp)
        a = root / "a"
        b = root / "b"
        (a / "sub").mkdir(parents=True)
        (b / "sub").mkdir(parents=True)
        (a / "sub" / "one.mjs").write_bytes(b"export const x = 1;\n")
        (b / "sub" / "one.mjs").write_bytes(b"export const x = 1;\n")

        count, nbytes, hashes = compare_tree_bytes(a, b, label="selftest-equal")
        assert count == 1 and nbytes == 20 and "sub/one.mjs" in hashes

        (b / "sub" / "one.mjs").write_bytes(b"export const x = 2;\n")
        try:
            compare_tree_bytes(a, b, label="selftest-diff")
            raise AssertionError("expected byte diff rejection")
        except AssertionError:
            pass

        existing = root / "existing-out"
        existing.mkdir()
        sentinel = existing / "keep.txt"
        sentinel.write_text("keep\n", encoding="utf-8")
        try:
            claim_fresh_output_dir(existing)
            raise AssertionError("expected claim_fresh_output_dir to reject existing dir")
        except AssertionError:
            pass
        assert sentinel.read_text(encoding="utf-8") == "keep\n"

        try:
            main([
                "--package",
                str(root / "missing-pkg"),
                "--eliscript-repo",
                str(existing),
                "--reference-host",
                str(sentinel),
                "--out",
                str(existing),
            ])
            raise AssertionError("expected main to reject invalid package without touching existing --out")
        except AssertionError:
            pass
        assert sentinel.read_text(encoding="utf-8") == "keep\n"

        fresh_out = claim_fresh_output_dir(root / "nested" / "fresh-out")
        assert fresh_out.is_dir()

    print("test-c3-a3-seed-host selftest PASS")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="C3/A3 Seed, Stage 0, and verifiable host cache acceptance")
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--package", type=Path, default=REPO / "dist" / "lisp")
    parser.add_argument("--eliscript-repo", type=Path, required=False)
    parser.add_argument("--reference-host", type=Path, default=Path("/opt/homebrew/bin/emacs"))
    parser.add_argument("--out", type=Path, default=REPO / "build" / "acceptance" / "c3-a3-run")
    args = parser.parse_args(argv)

    if args.selftest:
        return run_selftest()
    if not args.eliscript_repo:
        parser.error("--eliscript-repo is required unless --selftest is specified")

    pkg_dir = args.package.resolve()
    ref_bin = args.reference_host
    if not ref_bin.is_file():
        raise AssertionError(f"reference host not found: {ref_bin}")
    if not args.eliscript_repo.is_dir():
        raise AssertionError(f"eliscript repository directory not found: {args.eliscript_repo}")

    # 1. Verify standalone package and strict codesign BEFORE creating output directory
    ver = run_cmd([sys.executable, str(CACHE_TOOL), "verify", "--package", str(pkg_dir), "--json"], timeout=60)
    if ver.returncode != 0:
        raise AssertionError(f"package verify failed: {ver.stdout} {ver.stderr}")
    manifest = json.loads((pkg_dir / "manifest.json").read_text(encoding="utf-8"))
    lisp_bin = pkg_dir / "bin" / "femacs-lisp"
    http_bin = pkg_dir / "bin" / "http-host"
    verify_codesign_if_darwin(lisp_bin)
    verify_codesign_if_darwin(http_bin)

    out_dir = claim_fresh_output_dir(args.out)

    clean_env = isolated_env(out_dir / "home-femacs")
    ref_env = reference_env(out_dir / "home-ref")

    # 2. CLI contract & process identity
    ident = check_cli_contract(lisp_bin, out_dir, clean_env)

    # 3. Clean read-only snapshots from downstream compiler repository HEAD
    snap_femacs = out_dir / "host-femacs" / "src"
    snap_ref = out_dir / "host-ref" / "src"
    commit_f = extract_clean_git_snapshot(args.eliscript_repo, snap_femacs)
    commit_r = extract_clean_git_snapshot(args.eliscript_repo, snap_ref)
    assert commit_f == commit_r

    # 4. Seed & Diagnostic parity
    seed_count, seed_bytes, _ = check_seed_parity(
        lisp_bin, ref_bin, snap_femacs, snap_ref, out_dir, clean_env, ref_env
    )
    diag_results = check_diagnostic_parity(
        lisp_bin, ref_bin, snap_femacs, snap_ref, out_dir, clean_env, ref_env
    )

    # 5. Formal Stage 0 parity (zero JS runtime, byte-identical output + stage0.json)
    stage0_info = check_stage0_parity(
        lisp_bin, ref_bin, snap_femacs, snap_ref, out_dir, clean_env, ref_env
    )

    # 6. Downstream clean & strict-byte-compile Makefile hook parity
    check_makefile_hooks(lisp_bin, ref_bin, snap_femacs, snap_ref, clean_env, ref_env)

    # 7. Verifiable FEmacs Lisp host cache lifecycle + tamper rejections
    cache_info = check_cache_lifecycle(pkg_dir, manifest, snap_femacs, out_dir, clean_env)

    report = {
        "status": "PASS",
        "downstream_commit": commit_f,
        "process_identity": {
            "name": ident["name"],
            "noninteractive": ident["noninteractive"],
        },
        "package_manifest": {
            "source_commit": manifest["source_commit"],
            "target": manifest["target"],
            "build_config": manifest["build_config"],
            "package_sha256": manifest["package_sha256"],
            "binaries": manifest["binaries"],
            "resource_trees": manifest["resource_trees"],
        },
        "seed": {
            "modules": ["reader", "expander", "analyzer", "emitter"],
            "file_count": seed_count,
            "total_bytes": seed_bytes,
            "byte_diffs": 0,
        },
        "diagnostics": diag_results,
        "stage0": {
            **stage0_info,
            "byte_diffs": 0,
        },
        "cache": cache_info,
    }
    report_path = out_dir / "report.json"
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(
        f"test-c3-a3-seed-host PASS: downstream={commit_f[:12]} "
        f"seed={seed_count}files/{seed_bytes}B(0diffs) "
        f"stage0={stage0_info['file_count']}files/{stage0_info['total_bytes']}B(0diffs) "
        f"cache={cache_info['package_sha256'][:16]}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
