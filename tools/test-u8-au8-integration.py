#!/usr/bin/env python3
"""Stage U8 / AU8 consolidated integration, relocation, UI/TTY, 555-entrypoint, and performance driver.

Groups:
  clean-export  Export staged tracked sources, run static/unsafe/shadow/inventory/global/carrier
                gates (including zero `static mut`, >= 143 `#![forbid(unsafe_code)]` files, 100%
                `SAFETY:` contracts, and optional U1 baseline ratchet), build terminal & macOS
                bundle from empty output, verify `nm` singleton/ABI invariants and hidden-source
                relocation (batch + NSWorkspace GUI + codesign + spctl), publish rebuilt products
                to repository build/terminal and dist/macos, and run relocation retirement checks.
  macos-ui      Run local-acceptance (--gui --ime), multi-window IME (--mode multi),
                external Finder drag-and-drop, and abnormal/no-controlling-TTY stability.
  corpus-555    Run official-scale 555-entrypoint functional comparison against the bound
                32.0.50 reference, adjudicate per-case outcomes with reviewed source rules,
                and run targeted affected-matrix audits across terminal, Cocoa, and
                u8-native (normal, checking, collector=young, collector=major).
  performance   Run same-host 2+7-round runtime-comparison (delivered products + native/GC
                modes) and paired 7-round bench.py on Cocoa and terminal deliverables.
  selftest      Pure unit tests for verdict adjudication, source-rule matching, U8 safety
                threshold validation, and path relocation leak checks (starts no compiler/editor).
"""

from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import sys
import time
import unittest

REPO = Path(__file__).resolve().parent.parent


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with Path(path).open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def _rmtree_unlocked(path: Path) -> None:
    p = Path(path)
    if p.is_symlink():
        p.unlink(missing_ok=True)
        return
    if not p.exists():
        return
    for dirpath, dirnames, filenames in os.walk(p, topdown=False, followlinks=False):
        dp = Path(dirpath)
        for name in filenames:
            fp = dp / name
            if not fp.is_symlink():
                try:
                    os.chmod(fp, stat.S_IRWXU)
                except OSError:
                    pass
        for name in dirnames:
            sub = dp / name
            if not sub.is_symlink():
                try:
                    os.chmod(sub, stat.S_IRWXU)
                except OSError:
                    pass
        try:
            os.chmod(dp, stat.S_IRWXU)
        except OSError:
            pass
    shutil.rmtree(p, ignore_errors=True)


def artifact_meta(path: Path) -> dict:
    p = Path(path)
    return {
        "path": str(p),
        "bytes": p.stat().st_size,
        "sha256": sha256_file(p),
    }


def run_step(
    name: str,
    argv: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    timeout: float = 900.0,
    allow_codes: tuple[int, ...] = (0,),
    log_path: Path | None = None,
) -> dict:
    started = time.monotonic()
    cmd_str = [str(x) for x in argv]
    if log_path is not None:
        log_path.parent.mkdir(parents=True, exist_ok=True)
        with log_path.open("wb") as out_fh:
            proc = subprocess.run(
                cmd_str,
                cwd=str(cwd),
                env=env,
                stdin=subprocess.DEVNULL,
                stdout=out_fh,
                stderr=subprocess.STDOUT,
                timeout=timeout,
                check=False,
            )
        tail = ""
        if proc.returncode not in allow_codes and log_path.is_file():
            raw = log_path.read_text(encoding="utf-8", errors="replace")
            tail = "\n".join(raw.splitlines()[-40:])
    else:
        proc = subprocess.run(
            cmd_str,
            cwd=str(cwd),
            env=env,
            stdin=subprocess.DEVNULL,
            capture_output=True,
            text=True,
            errors="replace",
            timeout=timeout,
            check=False,
        )
        tail = "\n".join(((proc.stdout or "") + "\n" + (proc.stderr or "")).splitlines()[-40:])

    elapsed = round(time.monotonic() - started, 3)
    passed = proc.returncode in allow_codes
    rec = {
        "name": name,
        "passed": passed,
        "exit_code": proc.returncode,
        "seconds": elapsed,
        "argv": cmd_str,
    }
    if log_path is not None:
        rec["log"] = str(log_path)
    if not passed:
        rec["tail"] = tail
        raise RuntimeError(f"step {name} failed (exit={proc.returncode}, {elapsed}s):\n{tail}")
    return rec


def copy_tree_preserving_symlinks(src: Path, dst: Path) -> None:
    if dst.exists() or dst.is_symlink():
        for ro_item in dst.rglob("*") if dst.is_dir() and not dst.is_symlink() else ():
            if not ro_item.is_symlink():
                try:
                    ro_item.chmod(ro_item.stat().st_mode | stat.S_IWUSR)
                except OSError:
                    pass
        shutil.rmtree(dst)
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(src, dst, symlinks=True)


def check_no_build_path_leaks(payload: dict, forbidden_substring: str) -> list[str]:
    """Verify relocated paths contain no reference to the hidden build/export directory."""
    leaks: list[str] = []
    for key in ("data_directory", "exec_directory", "doc_directory"):
        val = str(payload.get(key, ""))
        if not val or forbidden_substring in val:
            leaks.append(f"{key}={val!r}")
    for lp in payload.get("load_path", []):
        if isinstance(lp, str) and forbidden_substring in lp:
            leaks.append(f"load_path:{lp}")
    return leaks


def verify_u8_active_metrics(active_product: dict) -> list[str]:
    """Validate final U8 de-unsafe closure invariants on active_product metrics."""
    errs: list[str] = []
    if active_product.get("static_mut_items", -1) != 0:
        errs.append(f"expected 0 static_mut_items, got {active_product.get('static_mut_items')}")
    if active_product.get("allow_unsafe_op_files", -1) != 0:
        errs.append(f"expected 0 allow_unsafe_op_files, got {active_product.get('allow_unsafe_op_files')}")
    if active_product.get("allow_static_mut_refs_files", -1) != 0:
        errs.append(
            f"expected 0 allow_static_mut_refs_files, got {active_product.get('allow_static_mut_refs_files')}"
        )
    if active_product.get("stitching_includes", -1) != 0:
        errs.append(f"expected 0 stitching_includes, got {active_product.get('stitching_includes')}")
    if active_product.get("forbid_unsafe_files", 0) < 143:
        errs.append(f"expected >= 143 forbid_unsafe_files, got {active_product.get('forbid_unsafe_files')}")
    blocks = active_product.get("unsafe_blocks", -1)
    documented = active_product.get("safety_documented_blocks", -2)
    if blocks < 0 or blocks != documented:
        errs.append(f"expected 100% SAFETY docs, got {documented}/{blocks}")
    return errs


def load_source_rule_index(rules_json: Path) -> dict[tuple[str, str], dict]:
    raw = json.loads(rules_json.read_text(encoding="utf-8"))
    index: dict[tuple[str, str], dict] = {}
    for rule in raw.get("rules", []):
        index[(rule["file"], rule["id"])] = {
            "category": rule["category"],
            "reason": rule["reason"],
        }
    return index


VOLATILE_PATTERNS = (
    (re.compile(r"#<[^<>]*?\s[0-9a-f]{6,}>"), "#<object>"),
    (re.compile(r"/\.noindex/[A-Za-z0-9_]+/"), "/.noindex/<container>/"),
    (re.compile(r"\*temp\*-\d+"), "*temp*-<n>"),
    (re.compile(r"\*-\d{5,}"), "*-<n>"),
    (re.compile(r"\b\d+%"), "<n>%"),
    (re.compile(r"\bin \d+ secs"), "in <n> secs"),
    (re.compile(r"eglot--fixture-[A-Za-z0-9]{6}"), "eglot--fixture-<id>"),
    (re.compile(r"emacs-test-[A-Za-z0-9]{6}-"), "emacs-test-<id>-"),
    (re.compile(r"femacs-functional-[A-Za-z0-9_]+"), "femacs-functional-<id>"),
    (re.compile(r"compiler-output/32\.0\.50-[0-9a-f]+"), "compiler-output/32.0.50-<hash>"),
    (re.compile(r"-[0-9a-f]{8}(?:-[0-9a-f]{8})?\.eln"), "-<hash>.eln"),
    (re.compile(r"corpus-555-(?:ref|cand)/corpus"), "corpus-555-<lane>/corpus"),
    (re.compile(r"/opt/homebrew/[Cc]ellar/[^/\s\"]+/32\.0\.50/share/[^/\s\"]+/32\.0\.50/lisp/", re.I), "<product-lisp>/"),
    (re.compile(r"[^\s\"]+?/dist/macos/[Ff]emacs\.app/[Cc]ontents/[Rr]esources/lisp/", re.I), "<product-lisp>/"),
    (re.compile(r'\(arrays-of-different-length \d+ (\d+) "<product-lisp>/([^"]+)"'), r'(arrays-of-different-length <len> \1 "<product-lisp>/\2"'),
    (re.compile(r"first-mismatch-at \d+\)\)\)\)\)"), "first-mismatch-at <idx>)))))"),
)


def normalize_diag(text: str) -> str:
    out = text or ""
    for pat, repl in VOLATILE_PATTERNS:
        out = pat.sub(repl, out)
    return out


def classify_case_pair(
    file_logical: str,
    case_id: str,
    ref_case: dict,
    cand_case: dict,
    rule_index: dict[tuple[str, str], dict],
) -> tuple[str, str | None]:
    """Compare one case across reference and candidate under docs/testing-strategy.md §2."""
    r_status = ref_case.get("status")
    c_status = cand_case.get("status")
    r_diag = ref_case.get("diagnostic", "")
    c_diag = cand_case.get("diagnostic", "")

    if r_status == c_status:
        if c_status == "passed":
            return "same-pass", None
        if c_status == "skipped":
            return "same-skip", None
        if c_status == "failed":
            if normalize_diag(r_diag) == normalize_diag(c_diag):
                return "both-fail-same-condition", None
            rule = rule_index.get((file_logical, case_id))
            if rule is not None:
                return "classified-rule", rule["category"]
            if "Femacs" in c_diag:
                return "classified-d01-name-only", "D-01-product-name"
            return "both-fail-condition-diff", None

    if r_status in ("failed", "skipped") and c_status == "passed":
        return "port-resolved", None

    rule = rule_index.get((file_logical, case_id))
    if rule is not None:
        return "classified-rule", rule["category"]

    if r_status == "passed" and c_status == "skipped":
        if "(skip-unless (featurep 'native-compile))" in c_diag:
            return "classified-rule", "explicit-optional-capability"
        return "unclassified-skip-diff", None

    if r_status == "passed" and c_status == "failed":
        if "Femacs" in c_diag:
            return "classified-d01-name-only", "D-01-product-name"
        return "port-worse", None

    return "port-worse", None


def adjudicate_555_runs(ref_dir: Path, cand_dir: Path, rules_json: Path) -> dict:
    rule_index = load_source_rule_index(rules_json)
    ref_summaries = {
        json.loads(p.read_text(encoding="utf-8"))["file"]: json.loads(p.read_text(encoding="utf-8"))
        for p in sorted((ref_dir / "results").glob("*.summary.json"))
    }
    cand_summaries = {
        json.loads(p.read_text(encoding="utf-8"))["file"]: json.loads(p.read_text(encoding="utf-8"))
        for p in sorted((cand_dir / "results").glob("*.summary.json"))
    }
    all_files = sorted(set(ref_summaries) | set(cand_summaries))
    case_totals: dict[str, int] = {}
    rule_category_totals: dict[str, int] = {}
    incomplete_files: list[dict] = []
    registration_mismatches: list[dict] = []
    blocking_cases: list[dict] = []
    total_cases = 0

    for logical in all_files:
        r_row = ref_summaries.get(logical)
        c_row = cand_summaries.get(logical)
        if r_row is None or c_row is None:
            incomplete_files.append({"file": logical, "reason": "missing-side"})
            continue
        r_ok = bool(r_row.get("input_matches") and r_row.get("registered"))
        c_ok = bool(c_row.get("input_matches") and c_row.get("registered"))
        if not c_ok or not r_ok:
            incomplete_files.append({
                "file": logical,
                "ref_ok": r_ok,
                "cand_ok": c_ok,
                "ref_exit": r_row.get("exit_code"),
                "cand_exit": c_row.get("exit_code"),
                "ref_timed_out": r_row.get("timed_out"),
                "cand_timed_out": c_row.get("timed_out"),
            })
            continue
        r_cases = r_row.get("cases", {})
        c_cases = c_row.get("cases", {})
        if set(r_cases) != set(c_cases):
            only_ref = sorted(set(r_cases) - set(c_cases))
            only_cand = sorted(set(c_cases) - set(r_cases))
            unclassified = [
                cid for cid in (only_ref + only_cand)
                if (logical, cid) not in rule_index
            ]
            if unclassified:
                registration_mismatches.append({
                    "file": logical,
                    "only_ref": only_ref[:10],
                    "only_cand": only_cand[:10],
                })
            for cid in sorted(set(r_cases) & set(c_cases)):
                total_cases += 1
                verdict, cat = classify_case_pair(logical, cid, r_cases[cid], c_cases[cid], rule_index)
                case_totals[verdict] = case_totals.get(verdict, 0) + 1
                if cat:
                    rule_category_totals[cat] = rule_category_totals.get(verdict if not cat else cat, 0) + 1
                if verdict in ("port-worse", "unclassified-skip-diff", "both-fail-condition-diff"):
                    blocking_cases.append({
                        "file": logical,
                        "case": cid,
                        "verdict": verdict,
                        "ref": r_cases[cid],
                        "cand": c_cases[cid],
                    })
            continue

        for cid in sorted(c_cases):
            total_cases += 1
            verdict, cat = classify_case_pair(logical, cid, r_cases[cid], c_cases[cid], rule_index)
            case_totals[verdict] = case_totals.get(verdict, 0) + 1
            if cat:
                rule_category_totals[cat] = rule_category_totals.get(cat, 0) + 1
            if verdict in ("port-worse", "unclassified-skip-diff", "both-fail-condition-diff"):
                blocking_cases.append({
                    "file": logical,
                    "case": cid,
                    "verdict": verdict,
                    "ref": r_cases[cid],
                    "cand": c_cases[cid],
                })

    return {
        "file_count": len(all_files),
        "total_compared_cases": total_cases,
        "case_totals": case_totals,
        "rule_category_totals": rule_category_totals,
        "incomplete_files": incomplete_files,
        "registration_mismatches": registration_mismatches,
        "blocking_case_count": len(blocking_cases),
        "blocking_cases": blocking_cases,
    }


def group_clean_export(
    out: Path,
    treesit_prefix: Path,
    u1_baseline: Path | None = None,
    resume_after_build: bool = False,
) -> dict:
    started = time.monotonic()
    out.mkdir(parents=True, exist_ok=True)
    export_dir = out / "clean-export"
    hidden_dir = out / "clean-export-hidden"
    reloc_dir = out / "relocated"
    logs_dir = out / "logs"
    steps: list[dict] = []

    reloc_term = reloc_dir / "terminal/port"
    reloc_app = reloc_dir / "Femacs.app"

    if not resume_after_build:
        steps.append(
            run_step(
                "export-tracked-sources",
                [sys.executable, "tools/test-repository-bootstrap.py", "--export", str(export_dir)],
                cwd=REPO,
                timeout=120,
                log_path=logs_dir / "01-export.log",
            )
        )
    elif export_dir.is_dir():
        if not (export_dir / "build/terminal/port/src/emacs").is_file() or not (
            export_dir / "dist/macos/Femacs.app/Contents/MacOS/Femacs"
        ).is_file():
            raise RuntimeError("--resume-after-build requires existing clean-export built deliverables")
        for sync_rel in (
            "tools/check-libshadows.py",
            "tools/check-unsafe-boundary.py",
            "tools/test-u8-au8-integration.py",
            "maintenance/tooling-inventory.json",
        ):
            shutil.copy2(REPO / sync_rel, export_dir / sync_rel)
        for check_tree in ("crates", "src", "lib", "lib-src", "platform", "xtask", "build-support"):
            for repo_file in (REPO / check_tree).rglob("*"):
                if repo_file.is_file():
                    rel = repo_file.relative_to(REPO)
                    exp_file = export_dir / rel
                    if not exp_file.is_file() or sha256_file(repo_file) != sha256_file(exp_file):
                        raise RuntimeError(f"build input {rel} drifted; full rebuild required")
    elif not (hidden_dir.is_dir() and reloc_app.is_dir() and reloc_term.is_dir()):
        raise RuntimeError("--resume-after-build found neither clean-export nor relocated+hidden state")

    active_source_dir = export_dir if export_dir.is_dir() else hidden_dir
    export_file_count = sum(
        1
        for p in active_source_dir.rglob("*")
        if (p.is_file() or p.is_symlink())
        and "build" not in p.relative_to(active_source_dir).parts
        and "dist" not in p.relative_to(active_source_dir).parts
        and ".femacs-cache" not in p.relative_to(active_source_dir).parts
    )

    if (REPO / "emacs").is_dir():
        steps.append(
            run_step(
                "repo-own-voice-differential",
                [sys.executable, "tools/check-own-voice.py"],
                cwd=REPO,
                timeout=120,
                log_path=logs_dir / "01b-repo-own-voice.log",
            )
        )
        steps.append(
            run_step(
                "repo-ledger-ticks",
                [sys.executable, "tools/check-ledger-ticks.py"],
                cwd=REPO,
                timeout=180,
                log_path=logs_dir / "01c-repo-ledger-ticks.log",
            )
        )

    unsafe_report_json = out / "unsafe-boundary-report.json"
    if export_dir.is_dir():
        unsafe_cmd = [
            sys.executable,
            "tools/check-unsafe-boundary.py",
            "--manifest",
            "tools/unsafe-boundary-manifest.json",
            "--require-no-file-allows",
            "--require-zero-active-stitching",
            "--require-100pct-safety-docs",
            "--require-zero-static-mut",
            "--min-forbid-files",
            "143",
            "--out",
            str(unsafe_report_json),
        ]
        if u1_baseline is not None:
            unsafe_cmd.extend(["--baseline", str(u1_baseline.resolve()), "--ratchet"])

        for gate_name, gate_args in (
            ("export-unsafe-boundary", unsafe_cmd),
            ("export-defvar-carriers", [sys.executable, "tools/check-defvar-carriers.py"]),
            ("export-globals", [sys.executable, "tools/check-globals.py"]),
            ("export-libshadows-source", [sys.executable, "tools/check-libshadows.py", "--source-only"]),
            ("export-declarations", [sys.executable, "tools/check-declarations.py"]),
            ("export-mirror-dupes", [sys.executable, "tools/check-mirror-dupes.py"]),
        ):
            steps.append(
                run_step(
                    gate_name,
                    gate_args,
                    cwd=export_dir,
                    timeout=180,
                    log_path=logs_dir / f"02-{gate_name}.log",
                )
            )

        unsafe_data = json.loads(unsafe_report_json.read_text(encoding="utf-8"))
        u8_errs = verify_u8_active_metrics(unsafe_data.get("active_product", {}))
        if u8_errs:
            raise RuntimeError(f"U8 unsafe closure check failed: {u8_errs}")

        build_env = dict(os.environ)
        build_env["FEMACS_TREE_SITTER_PREFIX"] = str(treesit_prefix.resolve())
        build_env["CARGO_TARGET_DIR"] = str((export_dir / "build/cargo").resolve())

        if not resume_after_build:
            steps.append(
                run_step(
                    "clean-export-build-terminal",
                    ["cargo", "run", "--release", "-p", "xtask", "--", "build-terminal"],
                    cwd=export_dir,
                    env=build_env,
                    timeout=1200,
                    log_path=logs_dir / "03-build-terminal.log",
                )
            )
            steps.append(
                run_step(
                    "clean-export-build-macos",
                    ["cargo", "run", "--release", "-p", "xtask", "--", "build-macos"],
                    cwd=export_dir,
                    env=build_env,
                    timeout=1200,
                    log_path=logs_dir / "04-build-macos.log",
                )
            )

        exp_term_bin = export_dir / "build/terminal/port/src/emacs"
        exp_app = export_dir / "dist/macos/Femacs.app"
        exp_gui_bin = exp_app / "Contents/MacOS/Femacs"

        for label, bin_path, kind in (
            ("terminal", exp_term_bin, "terminal"),
            ("cocoa", exp_gui_bin, "cocoa"),
        ):
            steps.append(
                run_step(
                    f"export-libshadows-{label}",
                    [sys.executable, "tools/check-libshadows.py", "--program", str(bin_path)],
                    cwd=export_dir,
                    timeout=60,
                    log_path=logs_dir / f"05-libshadows-{label}.log",
                )
            )
            steps.append(
                run_step(
                    f"export-nm-singleton-abi-{label}",
                    [
                        sys.executable,
                        "tools/check-unsafe-boundary.py",
                        "--binary",
                        str(bin_path),
                        "--kind",
                        kind,
                        "--require-modules",
                    ],
                    cwd=export_dir,
                    timeout=60,
                    log_path=logs_dir / f"05b-nm-{label}.log",
                )
            )

        copy_tree_preserving_symlinks(export_dir / "build/terminal/port", reloc_term)
        copy_tree_preserving_symlinks(exp_app, reloc_app)
        copy_tree_preserving_symlinks(exp_app, REPO / "dist/macos/Femacs.app")

        for ro_item in (export_dir / "build").rglob("*"):
            if not ro_item.is_symlink():
                try:
                    ro_item.chmod(ro_item.stat().st_mode | stat.S_IWUSR)
                except OSError:
                    pass
        shutil.rmtree(export_dir / "build", ignore_errors=True)
        shutil.rmtree(export_dir / ".femacs-cache", ignore_errors=True)
        export_dir.rename(hidden_dir)

    assert not export_dir.exists(), "original clean-export path must be physically absent"

    reloc_gui_bin = reloc_app / "Contents/MacOS/Femacs"
    reloc_term_bin = reloc_term / "src/emacs"

    steps.append(
        run_step(
            "relocated-codesign-strict",
            ["codesign", "--verify", "--deep", "--strict", "--verbose=2", str(reloc_app)],
            cwd=reloc_dir,
            timeout=60,
            log_path=logs_dir / "06-codesign.log",
        )
    )
    spctl_step = run_step(
        "relocated-spctl-boundary",
        ["spctl", "--assess", "--type", "execute", "-vv", str(reloc_app)],
        cwd=reloc_dir,
        timeout=60,
        allow_codes=(0, 3),
        log_path=logs_dir / "07-spctl.log",
    )
    steps.append(spctl_step)

    otool_out = subprocess.check_output(["otool", "-L", str(reloc_gui_bin)], text=True)
    bundled_dylibs = {p.name: p for p in (reloc_app / "Contents/Frameworks").rglob("*.dylib") if p.is_file()}
    rpath_libs = [
        ln.strip().split()[0]
        for ln in otool_out.splitlines()[1:]
        if ln.strip().startswith("@rpath/")
    ]
    missing_rpaths = [
        rp for rp in rpath_libs
        if rp.removeprefix("@rpath/") not in bundled_dylibs
    ]
    if missing_rpaths or str(export_dir) in otool_out:
        raise RuntimeError(f"relocated @rpath dylib error: missing={missing_rpaths}")

    probe_json = out / "relocated-cocoa-batch.json"
    lisp_probe = (
        "(progn (require 'json) "
        "(let ((res `((data_directory . ,data-directory) "
        "(exec_directory . ,exec-directory) "
        "(doc_directory . ,doc-directory) "
        "(load_path . ,load-path) "
        "(treesit . ,(and (fboundp 'treesit-available-p) (treesit-available-p))) "
        "(sqlite . ,(and (fboundp 'sqlite-available-p) (sqlite-available-p))) "
        "(png . ,(image-type-available-p 'png)) "
        "(jpeg . ,(image-type-available-p 'jpeg)) "
        "(webp . ,(image-type-available-p 'webp))))) "
        f"(with-temp-file {json.dumps(str(probe_json))} (insert (json-encode res)))))"
    )
    clean_env = {
        "HOME": str(out / "reloc-home"),
        "TMPDIR": str(out / "reloc-tmp"),
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "LC_ALL": "C",
    }
    (out / "reloc-home").mkdir(parents=True, exist_ok=True)
    (out / "reloc-tmp").mkdir(parents=True, exist_ok=True)

    steps.append(
        run_step(
            "relocated-cocoa-batch",
            [str(reloc_gui_bin), "-Q", "--batch", "--eval", lisp_probe],
            cwd=reloc_dir,
            env=clean_env,
            timeout=60,
            log_path=logs_dir / "08-reloc-cocoa-batch.log",
        )
    )
    cocoa_batch_data = json.loads(probe_json.read_text(encoding="utf-8"))
    leaks = check_no_build_path_leaks(cocoa_batch_data, str(export_dir))
    if leaks:
        raise RuntimeError(f"relocated Cocoa leaked export path: {leaks}")
    for feat in ("treesit", "sqlite", "png", "jpeg", "webp"):
        if not cocoa_batch_data.get(feat):
            raise RuntimeError(f"relocated Cocoa missing capability {feat}: {cocoa_batch_data}")

    gui_receipt = out / "relocated-gui-receipt.json"
    gui_init = out / "relocated-gui-check.el"
    gui_init.write_text(
        "(require 'json)\n"
        "(add-hook 'emacs-startup-hook\n"
        "  (lambda ()\n"
        f"    (with-temp-file {json.dumps(str(gui_receipt))}\n"
        "      (insert (json-encode `((window_system . ,(symbol-name window-system))\n"
        "                             (data_directory . ,data-directory)\n"
        "                             (pid . ,(emacs-pid))))))\n"
        "    (run-with-timer 0.2 nil (lambda () (kill-emacs 0)))))\n",
        encoding="utf-8",
    )
    launcher_bin = out / "ime-launch"
    steps.append(
        run_step(
            "compile-ime-launch",
            ["swiftc", "-O", str(REPO / "tools/fixtures/ime-launch.swift"), "-o", str(launcher_bin)],
            cwd=reloc_dir,
            timeout=120,
            log_path=logs_dir / "09-compile-ime-launch.log",
        )
    )
    launch_spec = out / "relocated-gui-launch-spec.json"
    launch_spec.write_text(
        json.dumps(
            {
                "arguments": ["-Q", "-l", str(gui_init)],
                "environment": clean_env,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    launch_proc = subprocess.run(
        [str(launcher_bin), str(reloc_app), str(launch_spec)],
        cwd=str(reloc_dir),
        capture_output=True,
        text=True,
        timeout=30,
        check=True,
    )
    launched_pid = int(launch_proc.stdout.strip().splitlines()[-1])
    deadline = time.monotonic() + 20.0
    while time.monotonic() < deadline:
        try:
            os.kill(launched_pid, 0)
        except ProcessLookupError:
            break
        time.sleep(0.1)
    if not gui_receipt.is_file():
        raise RuntimeError(f"relocated NSWorkspace GUI did not write receipt (pid={launched_pid})")
    gui_data = json.loads(gui_receipt.read_text(encoding="utf-8"))
    if gui_data.get("window_system") != "ns" or str(export_dir) in str(gui_data.get("data_directory", "")):
        raise RuntimeError(f"unexpected relocated GUI state: {gui_data}")

    term_probe_json = out / "relocated-terminal-batch.json"
    term_expr = (
        "(progn (require 'json) "
        "(with-temp-buffer (insert \"reloc-你好\") (goto-char (point-min)) (forward-char 6) "
        "(unless (= (point) 7) (error \"utf8 point mismatch\"))) "
        "(garbage-collect) "
        f"(with-temp-file {json.dumps(str(term_probe_json))} "
        "(insert (json-encode `((data_directory . ,data-directory) "
        "(exec_directory . ,exec-directory) (doc_directory . ,doc-directory) "
        "(load_path . ,load-path))))))"
    )
    term_reloc_env = dict(clean_env)
    term_reloc_env["EMACSDATA"] = str(reloc_app / "Contents/Resources/etc")
    term_reloc_env["EMACSDOC"] = str(reloc_app / "Contents/Resources/etc")
    term_reloc_env["EMACSLOADPATH"] = str(reloc_app / "Contents/Resources/lisp")
    term_reloc_env["EMACSPATH"] = str(reloc_term / "lib-src")
    steps.append(
        run_step(
            "relocated-terminal-batch",
            [str(reloc_term_bin), "-Q", "--batch", "--eval", term_expr],
            cwd=reloc_dir,
            env=term_reloc_env,
            timeout=60,
            log_path=logs_dir / "10-reloc-terminal-batch.log",
        )
    )
    term_data = json.loads(term_probe_json.read_text(encoding="utf-8"))
    if check_no_build_path_leaks(term_data, str(export_dir)):
        raise RuntimeError(f"relocated terminal leaked export path: {term_data}")

    need_repo_terminal_rebuild = True
    if resume_after_build and (REPO / "build/terminal/port/src/emacs").is_file():
        probe = subprocess.run(
            [
                str(REPO / "build/terminal/port/src/emacs"),
                "-Q",
                "--batch",
                "--eval",
                "(princ data-directory)",
            ],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        if probe.returncode == 0 and str(REPO / "etc") in (probe.stdout or ""):
            need_repo_terminal_rebuild = False

    if need_repo_terminal_rebuild:
        if (REPO / "build/terminal").exists():
            for ro_item in (REPO / "build/terminal").rglob("*"):
                if not ro_item.is_symlink():
                    try:
                        ro_item.chmod(ro_item.stat().st_mode | stat.S_IWUSR)
                    except OSError:
                        pass
            shutil.rmtree(REPO / "build/terminal")
        repo_build_env = dict(os.environ)
        repo_build_env["FEMACS_TREE_SITTER_PREFIX"] = str(treesit_prefix.resolve())
        repo_build_env["CARGO_TARGET_DIR"] = str((REPO / "build/cargo").resolve())
        steps.append(
            run_step(
                "repo-rebuild-terminal-deliverable",
                ["cargo", "run", "--release", "-p", "xtask", "--", "build-terminal"],
                cwd=REPO,
                env=repo_build_env,
                timeout=900,
                log_path=logs_dir / "10b-repo-build-terminal.log",
            )
        )
        for scratch_sub in (
            "cargo",
            "config",
            "image",
            "image-staging",
            "lcms",
            "load",
            "objects",
            "cargo-tools",
            "tls",
            "treesit",
        ):
            target_sub = REPO / "build/terminal" / scratch_sub
            if target_sub.exists():
                for ro_item in target_sub.rglob("*"):
                    if not ro_item.is_symlink():
                        try:
                            ro_item.chmod(ro_item.stat().st_mode | stat.S_IWUSR)
                        except OSError:
                            pass
                shutil.rmtree(target_sub, ignore_errors=True)
        for dump_alias in (REPO / "build/terminal/port/src").glob("emacs-32.0.50.*"):
            dump_alias.unlink(missing_ok=True)
        for root_scratch in ("cargo-tools", "generation", "manuals"):
            shutil.rmtree(REPO / "build" / root_scratch, ignore_errors=True)

    steps.append(
        run_step(
            "repo-libshadows-terminal",
            [sys.executable, "tools/check-libshadows.py", "--program", str(REPO / "build/terminal/port/src/emacs")],
            cwd=REPO,
            timeout=60,
            log_path=logs_dir / "10c-repo-libshadows-terminal.log",
        )
    )

    owned_retire_out = REPO / "build/acceptance/u8-relocation-retirement"
    if owned_retire_out.exists():
        shutil.rmtree(owned_retire_out)
    steps.append(
        run_step(
            "relocation-retirement-functional",
            [
                sys.executable,
                "tools/test-relocation-retirement-functional.py",
                "--out",
                str(owned_retire_out),
            ],
            cwd=REPO,
            timeout=120,
            log_path=logs_dir / "11-relocation-retirement.log",
        )
    )
    shutil.rmtree(owned_retire_out, ignore_errors=True)
    if (REPO / "build/acceptance").is_dir() and not any((REPO / "build/acceptance").iterdir()):
        (REPO / "build/acceptance").rmdir()

    clean_export_term_sha = sha256_file(reloc_term_bin)
    published = {
        "clean_export_terminal_emacs_sha256": clean_export_term_sha,
        "terminal_emacs": artifact_meta(REPO / "build/terminal/port/src/emacs"),
        "terminal_pdmp": artifact_meta(REPO / "build/terminal/port/src/emacs.pdmp"),
        "cocoa_femacs": artifact_meta(REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"),
        "cocoa_pdmp": artifact_meta(REPO / "dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"),
    }
    assert published["cocoa_femacs"]["sha256"] == sha256_file(reloc_gui_bin)

    for cleanup_dir in (hidden_dir, reloc_dir, out / "reloc-home", out / "reloc-tmp"):
        if cleanup_dir.exists():
            for ro_item in cleanup_dir.rglob("*"):
                if not ro_item.is_symlink():
                    try:
                        ro_item.chmod(ro_item.stat().st_mode | stat.S_IWUSR)
                    except OSError:
                        pass
            shutil.rmtree(cleanup_dir, ignore_errors=True)
    launcher_bin.unlink(missing_ok=True)

    unsafe_summary = {}
    if unsafe_report_json.is_file():
        rep = json.loads(unsafe_report_json.read_text(encoding="utf-8"))
        unsafe_summary = {
            "active_product": rep.get("active_product"),
            "summary": rep.get("summary"),
            "baseline_deltas": rep.get("baseline_deltas"),
        }

    summary = {
        "group": "clean-export",
        "passed": True,
        "seconds": round(time.monotonic() - started, 3),
        "exported_entries": export_file_count,
        "rpath_dylib_count": len(rpath_libs),
        "bundled_framework_dylib_count": len(bundled_dylibs),
        "spctl_exit_code": spctl_step["exit_code"],
        "relocated_gui": gui_data,
        "unsafe_closure": unsafe_summary,
        "published_deliverables": published,
        "steps": steps,
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def group_macos_ui(out: Path, sender: Path, sources: Path) -> dict:
    started = time.monotonic()
    out.mkdir(parents=True, exist_ok=True)
    logs_dir = out / "logs"
    gui_bin = REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"
    app_bundle = REPO / "dist/macos/Femacs.app"
    term_bin = REPO / "build/terminal/port/src/emacs"
    steps: list[dict] = []

    local_acc_out = REPO / "build/acceptance/u8-local-acceptance"
    if local_acc_out.exists():
        shutil.rmtree(local_acc_out)
    local_acc_out.parent.mkdir(parents=True, exist_ok=True)

    try:
        steps.append(
            run_step(
                "local-acceptance-gui-ime",
                [
                    sys.executable,
                    "tools/local-acceptance.py",
                    "--terminal",
                    str(term_bin),
                    "--app",
                    str(app_bundle),
                    "--gui",
                    "--ime",
                    "--sender",
                    str(sender),
                    "--sources",
                    str(sources),
                    "--out",
                    str(local_acc_out),
                ],
                cwd=REPO,
                timeout=420,
                log_path=logs_dir / "01-local-acceptance.log",
            )
        )
        local_results_src = local_acc_out / "results.json"
        shutil.copy2(local_results_src, out / "local-acceptance-results.json")
        local_res = json.loads(local_results_src.read_text(encoding="utf-8"))
    finally:
        if local_acc_out.exists():
            if not (out / "local-acceptance-results.json").exists():
                shutil.copytree(local_acc_out, out / "local-acceptance-failed", dirs_exist_ok=True)
            shutil.rmtree(local_acc_out)
        if local_acc_out.parent.exists() and not any(local_acc_out.parent.iterdir()):
            local_acc_out.parent.rmdir()
    steps.append(
        run_step(
            "macos-ime-multi-window",
            [
                sys.executable,
                "tools/test-macos-ime.py",
                "--binary",
                str(gui_bin),
                "--sender",
                str(sender),
                "--sources",
                str(sources),
                "--mode",
                "multi",
                "--out",
                str(out / "ime-multi"),
            ],
            cwd=REPO,
            timeout=180,
            log_path=logs_dir / "02-ime-multi.log",
        )
    )
    steps.append(
        run_step(
            "macos-finder-drag",
            [
                sys.executable,
                "tools/test-macos-drag.py",
                "--app",
                str(app_bundle),
                "--out",
                str(out / "finder-drag"),
            ],
            cwd=REPO,
            timeout=120,
            log_path=logs_dir / "03-finder-drag.log",
        )
    )
    steps.append(
        run_step(
            "abnormal-tty-stability",
            [
                sys.executable,
                "tools/test-goal-stability.py",
                "--mode",
                "all",
                "--out",
                str(out / "tty-stability"),
            ],
            cwd=REPO,
            timeout=120,
            log_path=logs_dir / "04-tty-stability.log",
        )
    )

    ime_res = json.loads((out / "ime-multi/results.json").read_text(encoding="utf-8"))
    drag_res = json.loads((out / "finder-drag/results.json").read_text(encoding="utf-8"))
    tty_res = json.loads((out / "tty-stability/results.json").read_text(encoding="utf-8"))

    summary = {
        "group": "macos-ui",
        "passed": True,
        "seconds": round(time.monotonic() - started, 3),
        "local_acceptance_checks": len(local_res.get("checks", [])),
        "ime_multi_callbacks": ime_res["scenarios"][0].get("callback_count"),
        "ime_multi_focus_loss": ime_res["scenarios"][0].get("focus_loss_observation"),
        "finder_drag_motion_events": drag_res.get("motion_event_count"),
        "finder_drag_callbacks": drag_res.get("callback_count"),
        "tty_stability_cases": len(tty_res.get("cases", [])),
        "steps": steps,
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def _run_ert_fixture(
    name: str,
    binary: Path,
    dump: Path,
    fixture: Path,
    log_path: Path,
    extra_env: dict[str, str] | None = None,
    timeout: float = 180.0,
) -> dict:
    eln_dir = log_path.parent / f"eln-{name}"
    env = {
        "HOME": str(log_path.parent / f"home-{name}"),
        "TMPDIR": str(log_path.parent / f"tmp-{name}"),
        "FEMACS_RUNTIME_NATIVE_CACHE": str(eln_dir),
        "EMACSNATIVELOADPATH": str(eln_dir),
        "PATH": "/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin",
        "LC_ALL": "C",
    }
    if extra_env:
        env.update(extra_env)
    for dkey in ("HOME", "TMPDIR", "FEMACS_RUNTIME_NATIVE_CACHE"):
        Path(env[dkey]).mkdir(parents=True, exist_ok=True)
    try:
        return run_step(
            name,
            [
                str(binary),
                f"--dump-file={dump}",
                "-Q",
                "--batch",
                "-l",
                "ert",
                "-l",
                str(fixture),
                "-f",
                "ert-run-tests-batch-and-exit",
            ],
            cwd=REPO,
            env=env,
            timeout=timeout,
            log_path=log_path,
        )
    finally:
        for dkey in ("HOME", "TMPDIR", "FEMACS_RUNTIME_NATIVE_CACHE"):
            _rmtree_unlocked(Path(env[dkey]))


def group_corpus_555(
    out: Path,
    ref_bin: Path,
    ref_dump: Path,
    ref_lisp: Path,
    native_root: Path,
) -> dict:
    started = time.monotonic()
    out.mkdir(parents=True, exist_ok=True)
    logs_dir = out / "logs"
    corpus_tmp = out / "corpus-tmp"
    corpus_tmp.mkdir(parents=True, exist_ok=True)
    steps: list[dict] = []

    cocoa_bin = REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"
    cocoa_dump = REPO / "dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"
    term_bin = REPO / "build/terminal/port/src/emacs"
    term_dump = REPO / "build/terminal/port/src/emacs.pdmp"
    u8_norm_bin = native_root / "port/src/emacs"
    u8_norm_dump = native_root / "port/src/emacs.pdmp"
    u8_chk_bin = native_root / "checking/port/src/emacs"
    u8_chk_dump = native_root / "checking/port/src/emacs.pdmp"
    repo_lisp = (REPO / "lisp").resolve()

    for req_bin in (cocoa_bin, cocoa_dump, term_bin, term_dump, u8_norm_bin, u8_norm_dump, u8_chk_bin, u8_chk_dump):
        if not req_bin.is_file():
            raise FileNotFoundError(f"missing required candidate artifact for corpus-555: {req_bin}")

    # Put /usr/bin:/bin ahead of /opt/homebrew/bin so BSD /bin/stty precedes GNU coreutils stty
    # (preventing the Darwin PTY echo buffer overflow documented in docs/modernization.md:774).
    corpus_env = dict(os.environ)
    corpus_env["PATH"] = "/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/opt/homebrew/opt/llvm/bin"
    corpus_env["LC_ALL"] = "C"
    corpus_env["TMPDIR"] = str(corpus_tmp)

    ref_corpus_out = out / "corpus-555-ref"
    cand_corpus_out = out / "corpus-555-cand"

    def run_side(
        label: str,
        prog: Path,
        dmp: Path,
        dest: Path,
        log_file: Path,
        lisp_root: Path | None = None,
    ) -> dict:
        if (dest / "summary.json").is_file():
            existing = json.loads((dest / "summary.json").read_text(encoding="utf-8"))
            if len(existing.get("files", [])) == 555:
                return {
                    "name": f"reuse-completed-{label}",
                    "passed": True,
                    "exit_code": 0,
                    "seconds": 0.0,
                    "reused_files": 555,
                }
        if dest.exists():
            shutil.rmtree(dest)
        cmd = [
            sys.executable,
            "tools/functional-corpus.py",
            "--program",
            str(prog),
            "--dump",
            str(dmp),
            "--corpus",
            str(REPO),
            "--materialize-fixtures",
            "--timeout",
            "120",
            "--label",
            label,
            "--out",
            str(dest),
        ]
        if lisp_root is not None:
            cmd.extend(["--lisp-root", str(lisp_root)])
        return run_step(
            f"functional-corpus-555-{label}",
            cmd,
            cwd=REPO,
            env=corpus_env,
            timeout=2700,
            allow_codes=(0, 1),
            log_path=log_file,
        )

    def run_strict_subset(
        step_name: str,
        subset_files: list[str],
        cand_prog: Path,
        cand_dmp: Path,
        cand_env_dict: dict[str, str],
        dest: Path,
        log_file: Path,
        allow_tramp_only_incomplete: bool = False,
        case_timeout: int = 360,
    ) -> dict:
        run_json = dest / "stable-suite-run.json"
        if run_json.is_file():
            try:
                existing = json.loads(run_json.read_text(encoding="utf-8"))
                if strict_subset_run_reusable(
                    existing,
                    subset_files,
                    sha256_file(cand_prog),
                    sha256_file(cand_dmp),
                    sha256_file(ref_bin),
                    sha256_file(ref_dump),
                    allow_tramp_only_incomplete=allow_tramp_only_incomplete,
                ):
                    return {
                        "name": f"reuse-completed-{step_name}",
                        "passed": True,
                        "exit_code": 0,
                        "seconds": 0.0,
                        "reused_files": len(subset_files),
                    }
            except (OSError, ValueError, KeyError, TypeError):
                pass
        if dest.exists():
            _rmtree_unlocked(dest)
        subset_spec = out / f"{step_name}-subset.json"
        subset_spec.write_text(json.dumps(subset_files, indent=2) + "\n", encoding="utf-8")
        try:
            return run_step(
                step_name,
                [
                    sys.executable,
                    "tools/stable-suite.py",
                    "--repo",
                    str(REPO),
                    "--manifest",
                    str(REPO / "compatibility/stable-31.1/sources.json"),
                    "--reference",
                    str(ref_bin),
                    "--reference-dump-args",
                    json.dumps([f"--dump-file={ref_dump}"]),
                    "--reference-env",
                    json.dumps({"FEMACS_STABLE_PRODUCT_LISP": str(ref_lisp)}),
                    "--candidate",
                    str(cand_prog),
                    "--candidate-dump-args",
                    json.dumps([f"--dump-file={cand_dmp}"]),
                    "--candidate-env",
                    json.dumps(cand_env_dict),
                    "--subset-file",
                    str(subset_spec),
                    "--timeout",
                    str(case_timeout),
                    "--out",
                    str(dest),
                ],
                cwd=REPO,
                env=corpus_env,
                timeout=3600,
                allow_codes=(0, 1),
                log_path=log_file,
            )
        finally:
            noindex_root = (Path.home() / "AI/caches/.noindex").resolve()
            if dest.is_dir():
                for env_json in dest.rglob("*.environment.json"):
                    try:
                        c = json.loads(env_json.read_text(encoding="utf-8")).get("container")
                        if isinstance(c, str) and c:
                            cp = Path(c).resolve()
                            if cp != noindex_root and cp.is_relative_to(noindex_root) and cp.exists():
                                _rmtree_unlocked(cp)
                    except (OSError, ValueError):
                        pass

    cand_pre_incomplete: list[str] = []
    ref_already_done = (ref_corpus_out / "summary.json").is_file() and len(
        json.loads((ref_corpus_out / "summary.json").read_text(encoding="utf-8")).get("files", [])
    ) == 555
    if (cand_corpus_out / "summary.json").is_file() and not ref_already_done:
        cand_pre = json.loads((cand_corpus_out / "summary.json").read_text(encoding="utf-8"))
        if len(cand_pre.get("files", [])) == 555:
            cand_pre_incomplete = [
                f["file"]
                for f in cand_pre["files"]
                if not (f.get("registered") and f.get("input_matches"))
            ]

    strict_out = out / "strict-incomplete-followup"
    if cand_pre_incomplete:
        steps.append(
            run_side("cand-cocoa", cocoa_bin, cocoa_dump, cand_corpus_out, logs_dir / "01-corpus-cand.log")
        )
        with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
            fut_ref = pool.submit(
                run_side,
                "ref-32.0.50",
                ref_bin,
                ref_dump,
                ref_corpus_out,
                logs_dir / "01-corpus-ref.log",
                ref_lisp,
            )
            fut_strict = pool.submit(
                run_strict_subset,
                "strict-incomplete-followup",
                cand_pre_incomplete,
                cocoa_bin,
                cocoa_dump,
                {},
                strict_out,
                logs_dir / "02-strict-incomplete-followup.log",
            )
            steps.append(fut_ref.result())
            steps.append(fut_strict.result())
    else:
        with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
            fut_ref = pool.submit(
                run_side,
                "ref-32.0.50",
                ref_bin,
                ref_dump,
                ref_corpus_out,
                logs_dir / "01-corpus-ref.log",
                ref_lisp,
            )
            fut_cand = pool.submit(
                run_side,
                "cand-cocoa",
                cocoa_bin,
                cocoa_dump,
                cand_corpus_out,
                logs_dir / "01-corpus-cand.log",
            )
            steps.append(fut_ref.result())
            steps.append(fut_cand.result())

    adjudication = adjudicate_555_runs(
        ref_corpus_out,
        cand_corpus_out,
        REPO / "tools/fixtures/stable-classification-source-rules.json",
    )
    (out / "adjudication-555.json").write_text(
        json.dumps(adjudication, indent=2) + "\n", encoding="utf-8"
    )

    all_incomplete = sorted({row["file"] for row in adjudication["incomplete_files"]})
    remaining_delta = sorted(set(all_incomplete) - set(cand_pre_incomplete))
    if all_incomplete and not cand_pre_incomplete:
        steps.append(
            run_strict_subset(
                "strict-incomplete-followup",
                all_incomplete,
                cocoa_bin,
                cocoa_dump,
                {},
                strict_out,
                logs_dir / "02-strict-incomplete-followup.log",
            )
        )
    elif remaining_delta:
        steps.append(
            run_strict_subset(
                "strict-incomplete-delta",
                remaining_delta,
                cocoa_bin,
                cocoa_dump,
                {},
                out / "strict-incomplete-delta",
                logs_dir / "02b-strict-incomplete-delta.log",
            )
        )

    def evaluate_strict_run(run_dir: Path) -> dict:
        strict_run = json.loads((run_dir / "stable-suite-run.json").read_text(encoding="utf-8"))
        rule_index = load_source_rule_index(REPO / "tools/fixtures/stable-classification-source-rules.json")
        return evaluate_strict_run_data(strict_run, rule_index)

    strict_followup: dict = {}
    if strict_out.is_dir():
        strict_followup["primary"] = evaluate_strict_run(strict_out)
    if (out / "strict-incomplete-delta").is_dir():
        strict_followup["delta"] = evaluate_strict_run(out / "strict-incomplete-delta")
    for part_name, part in strict_followup.items():
        if part["blocking_count"] != 0 or not part["execution_complete"]:
            raise RuntimeError(f"strict followup ({part_name}) failed: {part}")

    if adjudication["blocking_case_count"] != 0 or adjudication["registration_mismatches"]:
        raise RuntimeError(
            f"555-entrypoint adjudication found blocking cases: "
            f"blocking={adjudication['blocking_case_count']}, "
            f"reg_mismatch={adjudication['registration_mismatches']}, "
            f"samples={adjudication['blocking_cases'][:5]}"
        )

    # Targeted Affected-Matrix Audit across standard deliverables and u8-native (normal & checking):
    for compat_dir in (out / "stable-compat-standard", out / "stable-compat-u8-native"):
        shutil.rmtree(compat_dir, ignore_errors=True)
    steps.append(
        run_step(
            "stable-compat-standard-products",
            [
                sys.executable,
                "tools/stable-compat.py",
                "--terminal",
                str(term_bin),
                "--macos",
                str(REPO / "dist/macos/Femacs.app"),
                "--reference",
                str(ref_bin),
                "--reference-version",
                "32.0.50",
                "--out",
                str(out / "stable-compat-standard"),
            ],
            cwd=REPO,
            timeout=240,
            log_path=logs_dir / "03-stable-compat-standard.log",
        )
    )
    steps.append(
        run_step(
            "stable-compat-u8-native-normal-checking",
            [
                sys.executable,
                "tools/stable-compat.py",
                "--terminal",
                str(u8_norm_bin),
                "--macos",
                str(u8_chk_bin),
                "--reference",
                str(ref_bin),
                "--reference-version",
                "32.0.50",
                "--out",
                str(out / "stable-compat-u8-native"),
            ],
            cwd=REPO,
            timeout=240,
            log_path=logs_dir / "04-stable-compat-u8-native.log",
        )
    )

    subset_11 = json.loads((REPO / "tools/fixtures/stable-subset-native-affected.json").read_text(encoding="utf-8"))
    tramp_relpath = "test/lisp/net/tramp-tests.el"
    u8_subset_verdicts = {}
    for variant_label, v_bin, v_dump in (
        ("u8-native-normal", u8_norm_bin, u8_norm_dump),
        ("u8-native-checking", u8_chk_bin, u8_chk_dump),
    ):
        dest = out / f"subset-11-{variant_label}"
        steps.append(
            run_strict_subset(
                f"subset-11-{variant_label}",
                subset_11,
                v_bin,
                v_dump,
                {"FEMACS_STABLE_PRODUCT_LISP": str(repo_lisp)},
                dest,
                logs_dir / f"05-subset-11-{variant_label}.log",
                allow_tramp_only_incomplete=True,
            )
        )
        eval_11 = evaluate_strict_run(dest)
        u8_subset_verdicts[variant_label] = eval_11
        if is_tramp_timeout_only(eval_11):
            retry_dest = out / f"subset-11-{variant_label}-tramp-retry"
            retry_attempts: list[dict] = []
            retry_eval: dict = {}
            for attempt in range(1, 5):
                steps.append(
                    run_strict_subset(
                        f"subset-11-{variant_label}-tramp-retry",
                        [tramp_relpath],
                        v_bin,
                        v_dump,
                        {"FEMACS_STABLE_PRODUCT_LISP": str(repo_lisp)},
                        retry_dest,
                        logs_dir / f"05-subset-11-{variant_label}-tramp-retry-attempt-{attempt}.log",
                        case_timeout=60,
                    )
                )
                retry_eval = evaluate_strict_run(retry_dest)
                retry_attempts.append(
                    {
                        "attempt": attempt,
                        "execution_complete": retry_eval["execution_complete"],
                        "blocking_count": retry_eval["blocking_count"],
                        "incomplete_strict_files": retry_eval["incomplete_strict_files"],
                        "case_totals": retry_eval["case_totals"],
                    }
                )
                if (retry_eval["execution_complete"] and retry_eval["blocking_count"] == 0) or not is_tramp_timeout_only(
                    retry_eval
                ):
                    break
            u8_subset_verdicts[variant_label]["tramp_retry"] = retry_eval
            u8_subset_verdicts[variant_label]["tramp_retry_attempts"] = retry_attempts
            if retry_eval["blocking_count"] != 0 or not retry_eval["execution_complete"]:
                raise RuntimeError(f"{variant_label} 11-file tramp retry blocking: {retry_eval}")
        elif eval_11["blocking_count"] != 0 or not eval_11["execution_complete"]:
            raise RuntimeError(f"{variant_label} 11-file subset blocking: {eval_11}")

    for prod_name, b_path, d_path in (
        ("terminal", term_bin, term_dump),
        ("cocoa", cocoa_bin, cocoa_dump),
        ("u8-native", u8_norm_bin, u8_norm_dump),
    ):
        for fix_stem in (
            "runtime-hotpath-protection.el",
            "runtime-call-frames-functional.el",
            "bytecode-runtime.el",
        ):
            steps.append(
                _run_ert_fixture(
                    f"ert-{prod_name}-{fix_stem}",
                    b_path,
                    d_path,
                    REPO / "tools/fixtures" / fix_stem,
                    logs_dir / f"06-{prod_name}-{fix_stem}.log",
                )
            )
    steps.append(
        _run_ert_fixture(
            "ert-u8-native-g7-native-write-barrier.el",
            u8_norm_bin,
            u8_norm_dump,
            REPO / "tools/fixtures/g7-native-write-barrier.el",
            logs_dir / "07-u8-native-g7-native-write-barrier.el.log",
            extra_env={"FEMACS_GC_COLLECTOR": "young", "FEMACS_GC_OBSERVATION": "all"},
        )
    )

    young_spec = out / "u8-runtime-products-young.json"
    major_spec = out / "u8-runtime-products-major.json"
    young_spec.write_text(
        json.dumps(build_runtime_execution_products(native_root, "young", "u8-native-young"), indent=2) + "\n",
        encoding="utf-8",
    )
    major_spec.write_text(
        json.dumps(build_runtime_execution_products(native_root, "major", "u8-native-major"), indent=2) + "\n",
        encoding="utf-8",
    )

    def run_runtime_exec(
        step_name: str, target_dir: Path, cmd_args: list[str], log_file: Path, step_timeout: float
    ) -> dict:
        res_json = target_dir / "results.json"
        if res_json.is_file():
            try:
                existing = json.loads(res_json.read_text(encoding="utf-8"))
                if existing.get("passed") is True:
                    steps.append({
                        "name": f"reuse-completed-{step_name}",
                        "passed": True,
                        "exit_code": 0,
                        "seconds": 0.0,
                    })
                    return existing
            except (OSError, ValueError):
                pass
        _rmtree_unlocked(target_dir)
        steps.append(
            run_step(
                step_name,
                [sys.executable, "tools/runtime-execution.py", *cmd_args, "--out", str(target_dir)],
                cwd=REPO,
                timeout=step_timeout,
                log_path=log_file,
            )
        )
        data = json.loads(res_json.read_text(encoding="utf-8"))
        if data.get("passed") is not True:
            raise RuntimeError(f"{step_name} did not report passed=true: {res_json}")
        return data

    rt_out_base = REPO / "build/acceptance/u8-runtime-contracts"
    neg_out = rt_out_base / "negative"
    ert_young_out = rt_out_base / "ert-young"
    ert_major_out = rt_out_base / "ert-major"

    neg_res = run_runtime_exec(
        "prepare-negative",
        neg_out,
        ["--action", "prepare-negative", "--products", str(young_spec)],
        logs_dir / "07-runtime-prepare-negative.log",
        300.0,
    )
    ert_young_res = run_runtime_exec(
        "ert-native-collector-young",
        ert_young_out,
        [
            "--action",
            "ert",
            "--products",
            str(young_spec),
            "--negative-root",
            str(neg_out),
            "--groups",
            "native,collector",
        ],
        logs_dir / "08-runtime-ert-young.log",
        900.0,
    )
    ert_major_res = run_runtime_exec(
        "ert-collector-major",
        ert_major_out,
        [
            "--action",
            "ert",
            "--products",
            str(major_spec),
            "--groups",
            "collector",
        ],
        logs_dir / "08-runtime-ert-major.log",
        600.0,
    )
    runtime_contracts = {
        "prepare_negative": {
            "passed": bool(neg_res.get("passed")),
            "products": len(neg_res.get("products", [])),
        },
        "ert_native_collector_young": {
            "passed": bool(ert_young_res.get("passed")),
            "cases": len(ert_young_res.get("cases", [])),
            "registrations": len(ert_young_res.get("registrations", [])),
        },
        "ert_collector_major": {
            "passed": bool(ert_major_res.get("passed")),
            "cases": len(ert_major_res.get("cases", [])),
            "registrations": len(ert_major_res.get("registrations", [])),
        },
    }

    shutil.rmtree(ref_corpus_out / "corpus", ignore_errors=True)
    shutil.rmtree(cand_corpus_out / "corpus", ignore_errors=True)
    shutil.rmtree(corpus_tmp, ignore_errors=True)

    summary = {
        "group": "corpus-555",
        "passed": True,
        "seconds": round(time.monotonic() - started, 3),
        "adjudication_555": {
            "file_count": adjudication["file_count"],
            "total_compared_cases": adjudication["total_compared_cases"],
            "case_totals": adjudication["case_totals"],
            "rule_category_totals": adjudication["rule_category_totals"],
            "incomplete_file_count": len(adjudication["incomplete_files"]),
            "blocking_case_count": adjudication["blocking_case_count"],
        },
        "strict_incomplete_followup": strict_followup,
        "u8_native_11_subset": u8_subset_verdicts,
        "runtime_execution_contracts": runtime_contracts,
        "steps": steps,
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def group_performance(out: Path, ref_bin: Path, native_root: Path) -> dict:
    started = time.monotonic()
    out.mkdir(parents=True, exist_ok=True)
    logs_dir = out / "logs"
    steps: list[dict] = []

    u8_norm_bin = native_root / "port/src/emacs"
    if not u8_norm_bin.is_file():
        raise FileNotFoundError(f"missing native-collector binary for performance: {u8_norm_bin}")

    delivered_out = out / "delivered-comparison"
    steps.append(
        run_step(
            "runtime-comparison-delivered",
            [
                sys.executable,
                "tools/runtime-comparison.py",
                "--plan",
                str(REPO / "tools/fixtures/ga8-delivered-comparison-plan.json"),
                "--out",
                str(delivered_out),
            ],
            cwd=REPO,
            timeout=420,
            log_path=logs_dir / "01-delivered-comparison.log",
        )
    )

    base_plan = json.loads((REPO / "tools/fixtures/runtime-comparison-plan.json").read_text(encoding="utf-8"))
    rel_u8_bin = (
        str(u8_norm_bin.relative_to(REPO))
        if u8_norm_bin.is_relative_to(REPO)
        else str(u8_norm_bin)
    )
    for cand in base_plan["candidates"]:
        if cand["name"].startswith(("native-product-", "collector-")):
            cand["binary"] = rel_u8_bin
        elif cand["name"].startswith("reference-"):
            cand["binary"] = str(ref_bin)
    u8_plan_path = out / "u8-runtime-modes-plan.json"
    u8_plan_path.write_text(json.dumps(base_plan, indent=2) + "\n", encoding="utf-8")

    modes_out = out / "runtime-modes-comparison"
    steps.append(
        run_step(
            "runtime-comparison-modes-and-gc",
            [
                sys.executable,
                "tools/runtime-comparison.py",
                "--plan",
                str(u8_plan_path),
                "--out",
                str(modes_out),
            ],
            cwd=REPO,
            timeout=600,
            log_path=logs_dir / "02-runtime-modes-comparison.log",
        )
    )

    bench_cocoa_out = out / "bench-cocoa"
    bench_term_out = out / "bench-terminal"
    steps.append(
        run_step(
            "bench-cocoa-7rounds",
            [
                sys.executable,
                "tools/bench.py",
                "--prog",
                str(REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"),
                "--ref",
                str(ref_bin),
                "--rounds",
                "7",
                "--out",
                str(bench_cocoa_out),
            ],
            cwd=REPO,
            timeout=300,
            log_path=logs_dir / "03-bench-cocoa.log",
        )
    )
    steps.append(
        run_step(
            "bench-terminal-7rounds",
            [
                sys.executable,
                "tools/bench.py",
                "--prog",
                str(REPO / "build/terminal/port/src/emacs"),
                "--ref",
                str(ref_bin),
                "--rounds",
                "7",
                "--out",
                str(bench_term_out),
            ],
            cwd=REPO,
            timeout=300,
            log_path=logs_dir / "04-bench-terminal.log",
        )
    )

    delivered_json = json.loads((delivered_out / "results.json").read_text(encoding="utf-8"))
    modes_json = json.loads((modes_out / "results.json").read_text(encoding="utf-8"))
    bench_cocoa_json = json.loads((bench_cocoa_out / "results.json").read_text(encoding="utf-8"))
    bench_term_json = json.loads((bench_term_out / "results.json").read_text(encoding="utf-8"))

    summary = {
        "group": "performance",
        "passed": bool(bench_cocoa_json.get("passed") and bench_term_json.get("passed")),
        "seconds": round(time.monotonic() - started, 3),
        "delivered_comparisons": delivered_json.get("comparisons", []),
        "modes_comparisons": modes_json.get("comparisons", []),
        "bench_cocoa_stats": bench_cocoa_json.get("stats", []),
        "bench_cocoa_load_per_round": bench_cocoa_json.get("load_per_round", []),
        "bench_terminal_stats": bench_term_json.get("stats", []),
        "bench_terminal_load_per_round": bench_term_json.get("load_per_round", []),
        "steps": steps,
    }
    if not summary["passed"]:
        raise RuntimeError("bench.py reported failure")
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def strict_subset_run_reusable(
    existing: dict,
    subset_files: list[str],
    cand_prog_sha: str,
    cand_dmp_sha: str,
    ref_prog_sha: str,
    ref_dmp_sha: str,
    allow_tramp_only_incomplete: bool = False,
) -> bool:
    ex_files = existing.get("files", [])
    ex_before = existing.get("products_before", {})
    ex_cand = ex_before.get("candidate", {})
    ex_ref = ex_before.get("reference", {})
    incomplete_files = [
        row.get("file")
        for row in ex_files
        if not ((row.get("reference") or {}).get("complete") and (row.get("candidate") or {}).get("complete"))
    ]
    completion_ok = bool(ex_files) and (
        not incomplete_files
        or (
            allow_tramp_only_incomplete
            and incomplete_files == ["test/lisp/net/tramp-tests.el"]
        )
    )
    return (
        sorted(existing.get("subset") or []) == sorted(subset_files)
        and len(ex_files) == len(subset_files)
        and completion_ok
        and bool((existing.get("registration") or {}).get("passed"))
        and bool(existing.get("corpus_inputs_unchanged"))
        and bool(existing.get("identities_unchanged"))
        and (ex_cand.get("program") or {}).get("sha256") == cand_prog_sha
        and (ex_cand.get("dump") or {}).get("sha256") == cand_dmp_sha
        and (ex_ref.get("program") or {}).get("sha256") == ref_prog_sha
        and (ex_ref.get("dump") or {}).get("sha256") == ref_dmp_sha
    )


def evaluate_strict_run_data(strict_run: dict, rule_index: dict) -> dict:
    s_totals: dict[str, int] = {}
    s_blocking: list[dict] = []
    for case_row in strict_run.get("cases", []):
        f_name = case_row["file"]
        cid = case_row["id"]
        r_obs = case_row.get("reference") or {}
        c_obs = case_row.get("candidate") or {}
        verdict, _ = classify_case_pair(
            f_name,
            cid,
            {"status": r_obs.get("status"), "diagnostic": r_obs.get("diagnostic") or r_obs.get("condition") or ""},
            {"status": c_obs.get("status"), "diagnostic": c_obs.get("diagnostic") or c_obs.get("condition") or ""},
            rule_index,
        )
        s_totals[verdict] = s_totals.get(verdict, 0) + 1
        if verdict in ("port-worse", "unclassified-skip-diff", "both-fail-condition-diff"):
            s_blocking.append({
                "file": f_name,
                "case": cid,
                "verdict": verdict,
                "ref": r_obs,
                "cand": c_obs,
            })
    file_rows = strict_run.get("files", [])
    incomplete_strict_files = [
        row["file"]
        for row in file_rows
        if not ((row.get("reference") or {}).get("complete") and (row.get("candidate") or {}).get("complete"))
    ]
    incomplete_set = set(incomplete_strict_files)
    registration_passed = bool((strict_run.get("registration") or {}).get("passed"))
    inputs_unchanged = bool(strict_run.get("corpus_inputs_unchanged")) and bool(
        strict_run.get("identities_unchanged")
    )
    execution_complete = (
        len(file_rows) == strict_run.get("entrypoint_count")
        and len(file_rows) > 0
        and not incomplete_strict_files
        and registration_passed
        and inputs_unchanged
    )
    return {
        "entrypoint_count": strict_run.get("entrypoint_count"),
        "subset_complete": strict_run.get("subset_complete"),
        "execution_complete": execution_complete,
        "incomplete_strict_files": incomplete_strict_files,
        "registration_passed": registration_passed,
        "inputs_unchanged": inputs_unchanged,
        "case_count": len(strict_run.get("cases", [])),
        "case_totals": s_totals,
        "blocking_count": len(s_blocking),
        "blocking_on_complete_files": len([b for b in s_blocking if b["file"] not in incomplete_set]),
        "blocking": s_blocking,
    }


def is_tramp_timeout_only(eval_dict: dict) -> bool:
    return (
        eval_dict.get("incomplete_strict_files") == ["test/lisp/net/tramp-tests.el"]
        and eval_dict.get("blocking_on_complete_files") == 0
        and bool(eval_dict.get("registration_passed"))
        and bool(eval_dict.get("inputs_unchanged"))
        and all(
            b.get("file") == "test/lisp/net/tramp-tests.el"
            and (b.get("cand") or {}).get("status") in ("timeout", "not-run")
            for b in eval_dict.get("blocking", [])
        )
    )


def build_runtime_execution_products(native_root: Path, collector_mode: str, profile_id: str) -> dict:
    receipt_path = Path(native_root) / "runtime-product.json"
    if not receipt_path.is_file():
        raise FileNotFoundError(f"missing runtime product receipt: {receipt_path}")
    mod_path = REPO / "tools/runtime-execution.py"
    spec = importlib.util.spec_from_file_location("femacs_runtime_execution", mod_path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"unable to load runtime-execution helper: {mod_path}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    binding = receipt["binding"]
    program_rel, dump_rel, _, expected_roots = mod.product_receipt_layout(receipt, Path(native_root))
    return {
        "schema": 1,
        "products": [
            {
                "id": profile_id,
                "kind": binding["product"],
                "compiler_leg": binding["compiler_leg"],
                "native": "native" in binding["variant"],
                "collector": collector_mode,
                "observer": bool(binding["observer"]),
                "test_hooks": bool(binding["test_hooks"]),
                "program": str(Path(native_root) / program_rel),
                "dump": str(Path(native_root) / dump_rel),
                "resource_roots": expected_roots,
                "product_receipt": str(receipt_path),
            }
        ],
    }


class U8UnitTests(unittest.TestCase):
    def test_relocation_leak_detector_catches_hidden_export_paths(self):
        good = {
            "data_directory": "/reloc/Femacs.app/Contents/Resources/etc/",
            "exec_directory": "/reloc/Femacs.app/Contents/MacOS/libexec/",
            "doc_directory": "/reloc/Femacs.app/Contents/Resources/etc/",
            "load_path": ["/reloc/Femacs.app/Contents/Resources/lisp"],
        }
        self.assertEqual(check_no_build_path_leaks(good, "/cache/clean-export"), [])
        bad = dict(good, load_path=["/cache/clean-export/lisp"])
        self.assertEqual(
            check_no_build_path_leaks(bad, "/cache/clean-export"),
            ["load_path:/cache/clean-export/lisp"],
        )

    def test_case_pair_adjudication_distinguishes_worse_from_d01_and_rules(self):
        rules = {("test/lisp/foo-tests.el", "opt-case"): {"category": "explicit-optional-capability", "reason": "rsvg"}}
        self.assertEqual(
            classify_case_pair(
                "test/lisp/foo-tests.el",
                "c1",
                {"status": "passed", "diagnostic": ""},
                {"status": "passed", "diagnostic": ""},
                rules,
            ),
            ("same-pass", None),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/foo-tests.el",
                "opt-case",
                {"status": "passed", "diagnostic": ""},
                {"status": "skipped", "diagnostic": ""},
                rules,
            ),
            ("classified-rule", "explicit-optional-capability"),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/foo-tests.el",
                "name-case",
                {"status": "passed", "diagnostic": ""},
                {"status": "failed", "diagnostic": '(ert-test-failed ("Femacs" "32.0.50"))'},
                rules,
            ),
            ("classified-d01-name-only", "D-01-product-name"),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/foo-tests.el",
                "real-reg",
                {"status": "passed", "diagnostic": ""},
                {"status": "failed", "diagnostic": "(arith-error)"},
                rules,
            ),
            ("port-worse", None),
        )

    def test_u8_active_metrics_enforces_zero_static_mut_and_forbid_floor(self):
        good = {
            "static_mut_items": 0,
            "allow_unsafe_op_files": 0,
            "allow_static_mut_refs_files": 0,
            "stitching_includes": 0,
            "forbid_unsafe_files": 143,
            "unsafe_blocks": 6242,
            "safety_documented_blocks": 6242,
        }
        self.assertEqual(verify_u8_active_metrics(good), [])
        self.assertTrue(verify_u8_active_metrics(dict(good, static_mut_items=1)))
        self.assertTrue(verify_u8_active_metrics(dict(good, forbid_unsafe_files=142)))
        self.assertTrue(verify_u8_active_metrics(dict(good, safety_documented_blocks=6241)))

    def test_macos_runtime_forwarders_use_writable_vmglobal(self):
        gen_src = (REPO / "tools/gen-macos-runtime.py").read_text(encoding="utf-8")
        self.assertIn("femacs_fwd_{name}: VmGlobal<Lisp_Fwd> = VmGlobal::new(", gen_src)
        self.assertNotIn("femacs_fwd_{name}: SyncLispFwd", gen_src)

    def test_lane_and_native_compile_skip_classification(self):
        rules = load_source_rule_index(REPO / "tools/fixtures/stable-classification-source-rules.json")
        self.assertEqual(
            classify_case_pair(
                "test/lisp/international/ucs-normalize-tests.el",
                "ucs-normalize-part0",
                {"status": "failed", "diagnostic": '(file-missing "/tmp/corpus-555-ref/corpus/x")'},
                {"status": "failed", "diagnostic": '(file-missing "/tmp/corpus-555-cand/corpus/x")'},
                rules,
            ),
            ("both-fail-same-condition", None),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/progmodes/elisp-mode-tests.el",
                "xref-elisp-test-find-defs-face-el",
                {
                    "status": "failed",
                    "diagnostic": '(arrays-of-different-length 80 68 "/opt/homebrew/cellar/p/32.0.50/share/p/32.0.50/lisp/font-lock.el" "/x/lisp/font-lock.el" first-mismatch-at 1)))))',
                },
                {
                    "status": "failed",
                    "diagnostic": '(arrays-of-different-length 109 68 "/x/dist/macos/femacs.app/contents/resources/lisp/font-lock.el" "/x/lisp/font-lock.el" first-mismatch-at 51)))))',
                },
                rules,
            ),
            ("both-fail-same-condition", None),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/international/ucs-normalize-tests.el",
                "ucs-normalize-part0",
                {"status": "failed", "diagnostic": "(file-missing \"a\")"},
                {"status": "failed", "diagnostic": "(arith-error)"},
                rules,
            ),
            ("both-fail-condition-diff", None),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/emacs-lisp/comp-tests.el",
                "test-native-compile-prune-cache",
                {"status": "passed", "diagnostic": ""},
                {"status": "skipped", "diagnostic": "(ert-test-skipped ((skip-unless (featurep 'native-compile))))"},
                rules,
            ),
            ("classified-rule", "explicit-optional-capability"),
        )
        self.assertEqual(
            classify_case_pair(
                "test/lisp/foo-tests.el",
                "other-skip",
                {"status": "passed", "diagnostic": ""},
                {"status": "skipped", "diagnostic": "(ert-test-skipped ((skip-unless t)))"},
                rules,
            ),
            ("unclassified-skip-diff", None),
        )

    def test_sa_stack_block_preserves_recycle_capacity(self):
        alloc_src = (REPO / "src/alloc.rs").read_text(encoding="utf-8")
        self.assertIn("capacity: usize,", alloc_src)
        self.assertIn("frame_depth: usize,", alloc_src)
        self.assertIn("sa_stack_frame_enter", alloc_src)
        self.assertIn("(*header).capacity >= bytes", alloc_src)
        self.assertIn("(*block).used = 0;", alloc_src)
        eval_src = (REPO / "src/eval.rs").read_text(encoding="utf-8")
        self.assertNotIn("MaybeUninit", eval_src)
        self.assertIn("apply_spread_slow", eval_src)
        self.assertIn("apply1_spread_slow", eval_src)
        self.assertIn("funcall_subr_4_to_8", eval_src)

    def test_tramp_only_incomplete_reuse_and_blocking_split(self):
        subset = ["test/lisp/data-tests.el", "test/lisp/net/tramp-tests.el"]
        run_payload = {
            "subset": subset,
            "entrypoint_count": 2,
            "subset_complete": False,
            "files": [
                {"file": "test/lisp/data-tests.el", "reference": {"complete": True}, "candidate": {"complete": True}},
                {"file": "test/lisp/net/tramp-tests.el", "reference": {"complete": True}, "candidate": {"complete": False}},
            ],
            "registration": {"passed": True},
            "corpus_inputs_unchanged": True,
            "identities_unchanged": True,
            "products_before": {
                "candidate": {"program": {"sha256": "cp"}, "dump": {"sha256": "cd"}},
                "reference": {"program": {"sha256": "rp"}, "dump": {"sha256": "rd"}},
            },
            "cases": [
                {
                    "file": "test/lisp/data-tests.el",
                    "id": "ok-case",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": "passed", "diagnostic": ""},
                },
                {
                    "file": "test/lisp/net/tramp-tests.el",
                    "id": "tramp-unrun",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": None, "diagnostic": ""},
                },
            ],
        }
        self.assertFalse(
            strict_subset_run_reusable(run_payload, subset, "cp", "cd", "rp", "rd", allow_tramp_only_incomplete=False)
        )
        self.assertTrue(
            strict_subset_run_reusable(run_payload, subset, "cp", "cd", "rp", "rd", allow_tramp_only_incomplete=True)
        )
        other_incomplete = {
            **run_payload,
            "files": [
                {"file": "test/lisp/data-tests.el", "reference": {"complete": True}, "candidate": {"complete": False}},
                {"file": "test/lisp/net/tramp-tests.el", "reference": {"complete": True}, "candidate": {"complete": True}},
            ],
        }
        self.assertFalse(
            strict_subset_run_reusable(other_incomplete, subset, "cp", "cd", "rp", "rd", allow_tramp_only_incomplete=True)
        )
        eval_res = evaluate_strict_run_data(run_payload, {})
        self.assertFalse(eval_res["execution_complete"])
        self.assertEqual(eval_res["incomplete_strict_files"], ["test/lisp/net/tramp-tests.el"])
        self.assertEqual(eval_res["blocking_count"], 1)
        self.assertEqual(eval_res["blocking_on_complete_files"], 0)

        complete_bad = {
            **run_payload,
            "cases": run_payload["cases"]
            + [
                {
                    "file": "test/lisp/data-tests.el",
                    "id": "real-fail",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": "failed", "diagnostic": "(arith-error)"},
                }
            ],
        }
        eval_bad = evaluate_strict_run_data(complete_bad, {})
        self.assertEqual(eval_bad["blocking_count"], 2)
        self.assertEqual(eval_bad["blocking_on_complete_files"], 1)
        self.assertFalse(is_tramp_timeout_only(eval_bad))

    def test_is_tramp_timeout_only_rejects_non_timeout_failures(self):
        base_payload = {
            "subset": ["test/lisp/data-tests.el", "test/lisp/net/tramp-tests.el"],
            "entrypoint_count": 2,
            "subset_complete": False,
            "files": [
                {"file": "test/lisp/data-tests.el", "reference": {"complete": True}, "candidate": {"complete": True}},
                {"file": "test/lisp/net/tramp-tests.el", "reference": {"complete": True}, "candidate": {"complete": False}},
            ],
            "registration": {"passed": True},
            "corpus_inputs_unchanged": True,
            "identities_unchanged": True,
            "cases": [
                {
                    "file": "test/lisp/net/tramp-tests.el",
                    "id": "tramp-t1",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": "timeout", "diagnostic": "case did not complete"},
                },
                {
                    "file": "test/lisp/net/tramp-tests.el",
                    "id": "tramp-t2",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": "not-run", "diagnostic": "case did not complete"},
                },
            ],
        }
        self.assertTrue(is_tramp_timeout_only(evaluate_strict_run_data(base_payload, {})))

        tramp_real_fail = {
            **base_payload,
            "cases": base_payload["cases"]
            + [
                {
                    "file": "test/lisp/net/tramp-tests.el",
                    "id": "tramp-real-fail",
                    "reference": {"status": "passed", "diagnostic": ""},
                    "candidate": {"status": "failed", "diagnostic": "(ert-test-failed)"},
                }
            ],
        }
        self.assertFalse(is_tramp_timeout_only(evaluate_strict_run_data(tramp_real_fail, {})))

        other_file_eval = dict(evaluate_strict_run_data(base_payload, {}))
        other_file_eval["blocking"] = [
            {"file": "test/lisp/data-tests.el", "cand": {"status": "timeout"}},
        ]
        self.assertFalse(is_tramp_timeout_only(other_file_eval))

    def test_build_runtime_execution_products_contract(self):
        import tempfile

        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            with self.assertRaises(FileNotFoundError):
                build_runtime_execution_products(root, "young", "u8-native-young")
            (root / "runtime-config.json").write_text(json.dumps({"macros": {}, "features": {}}), encoding="utf-8")
            receipt = {
                "schema": 1,
                "identity_version": 3,
                "binding": {
                    "product": "cocoa",
                    "compiler_leg": "normal",
                    "variant": "native-collector",
                    "observer": True,
                    "test_hooks": False,
                },
                "files": {
                    "port/src/temacs": "a",
                    "port/src/emacs": "b",
                    "port/src/emacs.pdmp": "c",
                    "port/src/bootstrap-emacs.pdmp": "d",
                    "port/lib-src/emacsclient": "e",
                    "dist/Femacs.app/Contents/MacOS/Femacs": "f",
                    "dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp": "g",
                    "dist/Femacs.app/Contents/MacOS/bin/emacsclient": "h",
                },
                "resources": {
                    "generated-config": {},
                    "bundle": {},
                    "runtime-native": {},
                    "licenses": {},
                },
            }
            (root / "runtime-product.json").write_text(json.dumps(receipt), encoding="utf-8")
            doc = build_runtime_execution_products(root, "young", "u8-native-young")
            self.assertEqual(doc["schema"], 1)
            prod = doc["products"][0]
            self.assertEqual(prod["id"], "u8-native-young")
            self.assertEqual(prod["kind"], "cocoa")
            self.assertEqual(prod["collector"], "young")
            self.assertTrue(prod["native"])
            self.assertEqual(prod["program"], str(root / "dist/Femacs.app/Contents/MacOS/Femacs"))
            self.assertEqual(prod["dump"], str(root / "dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--group",
        choices=["clean-export", "macos-ui", "corpus-555", "performance", "selftest"],
        required=True,
    )
    parser.add_argument("--out", type=Path)
    parser.add_argument("--treesit-prefix", type=Path)
    parser.add_argument("--u1-baseline", type=Path)
    parser.add_argument("--resume-after-build", action="store_true")
    parser.add_argument("--sender", type=Path)
    parser.add_argument("--sources", type=Path)
    parser.add_argument("--reference", type=Path)
    parser.add_argument("--reference-dump", type=Path)
    parser.add_argument("--reference-lisp", type=Path)
    parser.add_argument(
        "--native-runtime-root",
        type=Path,
        default=REPO / "build/runtime/u8-native",
    )
    args = parser.parse_args(argv)

    if args.group == "selftest":
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(U8UnitTests)
        res = unittest.TextTestRunner(verbosity=2).run(suite)
        return 0 if res.wasSuccessful() else 1

    if args.out is None:
        parser.error("--out is required for integration groups")

    if args.group == "clean-export":
        if args.treesit_prefix is None:
            parser.error("--treesit-prefix is required for clean-export")
        res = group_clean_export(
            args.out.resolve(),
            args.treesit_prefix.resolve(),
            u1_baseline=args.u1_baseline.resolve() if args.u1_baseline else None,
            resume_after_build=args.resume_after_build,
        )
    elif args.group == "macos-ui":
        if args.sender is None or args.sources is None:
            parser.error("--sender and --sources are required for macos-ui")
        res = group_macos_ui(args.out.resolve(), args.sender.resolve(), args.sources.resolve())
    elif args.group == "corpus-555":
        if args.reference is None or args.reference_dump is None or args.reference_lisp is None:
            parser.error("--reference, --reference-dump, and --reference-lisp are required for corpus-555")
        res = group_corpus_555(
            args.out.resolve(),
            args.reference.resolve(),
            args.reference_dump.resolve(),
            args.reference_lisp.resolve(),
            args.native_runtime_root.resolve(),
        )
    elif args.group == "performance":
        if args.reference is None:
            parser.error("--reference is required for performance")
        res = group_performance(
            args.out.resolve(),
            args.reference.resolve(),
            args.native_runtime_root.resolve(),
        )
    else:
        raise AssertionError(args.group)

    print(json.dumps({"group": res["group"], "passed": res["passed"], "seconds": res["seconds"]}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
