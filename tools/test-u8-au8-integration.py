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


def _cleanup_strict_containers(s_dir: Path) -> None:
    """Remove owned `.noindex` containers referenced under one closed subset dir."""
    if not s_dir.is_dir():
        return
    noindex_root = (Path.home() / "AI/caches/.noindex").resolve()
    for env_json in s_dir.rglob("*.environment.json"):
        try:
            c = json.loads(env_json.read_text(encoding="utf-8")).get("container")
            if isinstance(c, str) and c:
                cp = Path(c).resolve()
                if cp != noindex_root and cp.is_relative_to(noindex_root) and cp.exists():
                    _rmtree_unlocked(cp)
        except (OSError, ValueError):
            pass


def _is_reference_filenotify_monitor_flake(dest_fn: Path, asm_res: dict) -> bool:
    """Return True iff scoped-assembly failed solely from a reference filenotify monitor skip."""
    if asm_res.get("structural_failure_count") != 0:
        return False
    if (asm_res.get("blocking_case_failure_count") or 0) <= 0:
        return False
    blocking = asm_res.get("blocking_cases") or []
    if not blocking or not all(
        isinstance(entry, dict)
        and entry.get("file") == "test/lisp/filenotify-tests.el"
        and entry.get("candidate_status") in ("passed", "skipped")
        for entry in blocking
    ):
        return False
    run_json = dest_fn / "stable-suite-run.json"
    if not run_json.is_file():
        return False
    try:
        run_doc = json.loads(run_json.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return False
    for row in run_doc.get("files") or []:
        if row.get("file") == "test/lisp/filenotify-tests.el":
            prog_rel = ((row.get("reference") or {}).get("progress") or {}).get("path")
            if not isinstance(prog_rel, str) or not prog_rel:
                return False
            prog_path = dest_fn / prog_rel
            if not prog_path.is_file():
                return False
            try:
                return "Cannot determine test monitor" in prog_path.read_text(
                    encoding="utf-8", errors="replace"
                )
            except OSError:
                return False
    return False


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


STRICT_PARITY_VERDICTS = frozenset({"same-pass", "same-skip", "both-fail-same-condition"})


def classify_case_pair(
    file_logical: str,
    case_id: str,
    ref_case: dict,
    cand_case: dict,
    rule_index: dict[tuple[str, str], dict] | None = None,
) -> tuple[str, str | None]:
    """Compare one case across reference and candidate strictly without permissive waivers."""
    del file_logical, case_id, rule_index
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
            return "both-fail-condition-diff", None
        return "port-worse", None

    if r_status in ("failed", "skipped") and c_status == "passed":
        return "port-resolved", None

    if r_status == "passed" and c_status == "skipped":
        return "unclassified-skip-diff", None

    return "port-worse", None


def adjudicate_555_runs(ref_dir: Path, cand_dir: Path, rules_json: Path | None = None) -> dict:
    del rules_json
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
    non_parity_files_set: set[str] = set()
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
            non_parity_files_set.add(logical)
            registration_mismatches.append({
                "file": logical,
                "only_ref": only_ref[:10],
                "only_cand": only_cand[:10],
            })
            for cid in sorted(set(r_cases) & set(c_cases)):
                total_cases += 1
                verdict, cat = classify_case_pair(logical, cid, r_cases[cid], c_cases[cid])
                case_totals[verdict] = case_totals.get(verdict, 0) + 1
                if cat:
                    rule_category_totals[cat] = rule_category_totals.get(cat, 0) + 1
                if verdict not in STRICT_PARITY_VERDICTS:
                    non_parity_files_set.add(logical)
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
            verdict, cat = classify_case_pair(logical, cid, r_cases[cid], c_cases[cid])
            case_totals[verdict] = case_totals.get(verdict, 0) + 1
            if cat:
                rule_category_totals[cat] = rule_category_totals.get(cat, 0) + 1
            if verdict not in STRICT_PARITY_VERDICTS:
                non_parity_files_set.add(logical)
            if verdict in ("port-worse", "unclassified-skip-diff", "both-fail-condition-diff"):
                blocking_cases.append({
                    "file": logical,
                    "case": cid,
                    "verdict": verdict,
                    "ref": r_cases[cid],
                    "cand": c_cases[cid],
                })

    followup_files = sorted(
        {row["file"] for row in incomplete_files}
        | {row["file"] for row in registration_mismatches}
        | non_parity_files_set
    )
    return {
        "file_count": len(all_files),
        "total_compared_cases": total_cases,
        "case_totals": case_totals,
        "rule_category_totals": rule_category_totals,
        "incomplete_files": incomplete_files,
        "registration_mismatches": registration_mismatches,
        "non_parity_files": sorted(non_parity_files_set),
        "followup_files": followup_files,
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


def _load_module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"unable to load module: {path}")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def _verified_case_ids_in_scoped_assembly(
    suite_run_dir: Path,
    assembly_accounting: dict,
) -> set[tuple[str, str]]:
    """Return all (file, id) cases cleanly passed or classified in a 0-blocking scoped assembly."""
    if (
        assembly_accounting.get("structural_failure_count") != 0
        or assembly_accounting.get("blocking_case_failure_count") != 0
        or not assembly_accounting.get("classified_corpus_accepted")
    ):
        return set()
    accounting = _load_module("u8_acct_check", REPO / "tools/stable-classification.py")
    verified: set[tuple[str, str]] = {
        (row["file"], row["id"]) for row in assembly_accounting.get("classified_cases", [])
    }
    for kind, item in accounting.detailed_rows(suite_run_dir / "stable-suite-run.json"):
        if kind == "cases" and item.get("passed") is True:
            verified.add((item["file"], item["id"]))
    return verified


def adjudicate_cocoa_strict_receipt(
    run_dir: Path,
    cocoa_bin: Path,
    cocoa_dump: Path,
    cocoa_lisp: Path,
    ref_selection_path: Path,
    adjudication_out: Path,
    *,
    subset_11_files: set[str],
    u8_native_verified_cases: dict[str, set[tuple[str, str]]],
    baseline_runtime_root: Path | None = None,
    control_runs: list[tuple[Path, Path]] | None = None,
    remote_contract_runs: list[tuple[Path, Path]] | None = None,
    library_contract_runs: list[Path] | None = None,
    eglot_insert_replace_probe: Path | None = None,
    eglot_text_edit_probe: Path | None = None,
) -> dict:
    """Adjudicate a closed Cocoa stable-suite receipt via stable-classification contracts."""
    accounting = _load_module("u8_cocoa_classification", REPO / "tools/stable-classification.py")
    suite, api = accounting._load_suite_api()

    receipt_path = run_dir / "stable-suite-completion.json"
    detailed_path = run_dir / "stable-suite-run.json"
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"), object_pairs_hook=accounting._pairs)
    cand_before = receipt["products_before"]["candidate"]
    expected_cocoa_env = {
        "FEMACS_GC_OBSERVATION": "off",
        "FEMACS_GC_COLLECTOR": "major",
        "FEMACS_STABLE_PRODUCT_LISP": str(cocoa_lisp),
    }
    if (
        cand_before["program"]["sha256"] != sha256_file(cocoa_bin)
        or cand_before["dump"]["sha256"] != sha256_file(cocoa_dump)
        or receipt["runtime_environment"]["candidate"] != expected_cocoa_env
    ):
        raise RuntimeError("cocoa strict receipt product or runtime_environment mismatch")

    if baseline_runtime_root is not None:
        cocoa_profile = accounting.scoped_candidate_profile(baseline_runtime_root, receipt)
    else:
        cocoa_profile = {
            "id": "cocoa-baseline-normal",
            "kind": "cocoa",
            "compiler_leg": "normal",
            "native": False,
            "observer": False,
            "test_hooks": False,
            "collector": "off",
            "program": str(cocoa_bin),
            "dump": str(cocoa_dump),
            "resource_roots": {"lisp": str(cocoa_lisp)},
        }
    selection = json.loads(ref_selection_path.read_text(encoding="utf-8"), object_pairs_hook=accounting._pairs)
    rules_path = REPO / "tools/fixtures/stable-classification-source-rules.json"
    definition = json.loads(rules_path.read_text(encoding="utf-8"), object_pairs_hook=accounting._pairs)
    accounting.require(
        definition.get("schema") == "femacs-stable-source-rule-specs-v1"
        and cocoa_profile["id"] in definition.get("profiles", []),
        "reviewed source rule specification missing cocoa-baseline-normal",
    )

    manifest_path = REPO / "compatibility/stable-31.1/sources.json"
    accounting.require(
        receipt["manifest_sha256"] == api._sha256_path(manifest_path),
        "cocoa strict receipt manifest identity differs from repository corpus",
    )
    manifest = suite.load_manifest(manifest_path, REPO)

    source_rules: dict[tuple[str, str], dict] = {}
    for rule in definition["rules"]:
        expanded = []
        for anchor in rule["source_evidence"]:
            relative = Path(anchor["path"])
            accounting.require(
                not relative.is_absolute() and ".." not in relative.parts and relative.as_posix() == anchor["path"],
                "source definitions must use canonical repository-relative anchors",
            )
            expanded.append(dict(anchor, path=str(REPO / relative)))
        source_rules[(rule["file"], rule["id"])] = dict(
            rule,
            profile=cocoa_profile["id"],
            source_evidence=expanded,
        )

    controls, _ = accounting._closed_controls(control_runs or [])
    remote_controls, _ = accounting._remote_collections(remote_contract_runs or [])
    library_contracts = [
        dict(path=str(p.resolve()), identity=accounting.bound_file(p.resolve(), limit=accounting.MAX_ITEM))
        for p in (library_contract_runs or [])
    ]
    eglot_probes = accounting._load_eglot_probes(
        argparse.Namespace(
            eglot_insert_replace_probe=eglot_insert_replace_probe,
            eglot_text_edit_probe=eglot_text_edit_probe,
        )
    )
    control_cache: dict = {}

    catalog = {
        "schema": accounting.SCHEMA,
        "manifest_sha256": receipt["manifest_sha256"],
        "reference_selection_sha256": selection["sha256"],
        "contexts": {
            cocoa_profile["id"]: {
                "context_sha256": accounting.context_identity(receipt, cocoa_profile, selection),
                "run_sha256": receipt["detailed"]["sha256"],
                "completion_sha256": api._sha256_path(receipt_path),
            }
        },
        "rules": [],
    }

    subset = receipt.get("subset")
    entry_rows = (
        api._load_batch().select_entrypoints(manifest, subset)
        if subset is not None
        else api._load_batch().select_entrypoints(manifest)
    )
    by_path = {entry["path"]: entry for entry in entry_rows}
    file_rows = {item["file"]: item for kind, item in accounting.detailed_rows(detailed_path) if kind == "files"}

    capture_failures: list[dict] = []
    skipped_passing: int = 0
    for kind, case in accounting.detailed_rows(detailed_path):
        if kind != "cases":
            continue
        key = (case.get("file"), case.get("id"))
        if key not in source_rules:
            continue
        if case.get("passed"):
            skipped_passing += 1
            continue
        rule = source_rules[key]
        try:
            correct_control = None
            if rule["category"] == "known-reference-library-defect":
                state = accounting.reference_defect_state(
                    rule,
                    case,
                    file_rows[case["file"]],
                    receipt,
                    {label: accounting._original_case_event(run_dir, suite, case, label) for label in ("reference", "candidate")},
                    run_dir,
                    suite,
                )
                labels = accounting.reference_contract_labels(state["state"])
                correct_controls = {}
                for label in labels:
                    target = cocoa_profile["id"] if label == "candidate" else "installed-reference"
                    matches = [
                        pin
                        for pin in library_contracts
                        if any(row.get("profile") == target for row in accounting._control_json(pin, control_cache).get("cases", []))
                    ]
                    if len(matches) != 1:
                        raise ValueError("reference-defect control unavailable for cocoa-baseline-normal")
                    correct_controls[label] = matches[0]
                if state["state"] != "active-reference-only-defect":
                    raise ValueError(f"reference-defect-not-active:{state['state']}")
                catalog["contexts"][cocoa_profile["id"]].setdefault("reference_defects", []).append(
                    {"definition": rule, "controls": correct_controls}
                )
                correct_control = correct_controls.get("candidate")
            captured = accounting._capture_failing_rule(
                rule,
                case,
                root=run_dir,
                api=suite,
                profile_id=cocoa_profile["id"],
                candidate_kind=cocoa_profile["kind"],
                controls=controls,
                remote_controls=remote_controls,
                control_cache=control_cache,
                correct_behavior_control=correct_control,
                eglot_probes=eglot_probes,
            )
            accounting._accept_rule(
                captured,
                case,
                file_rows[case["file"]],
                by_path[case["file"]],
                run_dir,
                suite,
                selection,
                cocoa_profile,
                receipt,
                control_cache,
                manifest=manifest,
            )
            catalog["rules"].append(captured)
        except ValueError as exc:
            capture_failures.append({"file": key[0], "id": key[1], "reason": str(exc)})
            if rule["category"] == "known-reference-library-defect":
                defs = catalog["contexts"][cocoa_profile["id"]].get("reference_defects")
                if isinstance(defs, list) and defs and defs[-1].get("definition") is rule:
                    defs.pop()

    _rmtree_unlocked(adjudication_out)
    adjudication_out.mkdir(parents=True, exist_ok=True)
    catalog_path = adjudication_out / "classification.json"
    accounting._write_catalog(catalog_path, catalog)
    accounting_res = accounting.assess(
        api,
        run_dir,
        receipt,
        manifest,
        catalog_path,
        cocoa_profile,
        selection,
    )
    (adjudication_out / "accounting.json").write_text(
        json.dumps(accounting_res, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    if accounting_res.get("structural_failure_count") != 0:
        raise RuntimeError(
            f"strict cocoa adjudication structural failure: {accounting_res.get('structural_failures')}"
        )

    deferred_to_native: list[dict] = []
    unexplained_blocking: list[dict] = []
    norm_verified = u8_native_verified_cases.get("u8-native-normal", set())
    chk_verified = u8_native_verified_cases.get("u8-native-checking", set())
    for blk in accounting_res.get("blocking_cases", []):
        bkey = (blk["file"], blk["id"])
        if blk["file"] in subset_11_files and bkey in norm_verified and bkey in chk_verified:
            deferred_to_native.append(blk)
        else:
            unexplained_blocking.append(blk)

    if unexplained_blocking:
        raise RuntimeError(
            f"strict cocoa adjudication blocking cases outside verified u8-native subset-11 deferral: "
            f"{unexplained_blocking[:10]}"
        )
    deferred_keys = {(blk["file"], blk["id"]) for blk in deferred_to_native}
    unexplained_capture = [cf for cf in capture_failures if (cf["file"], cf["id"]) not in deferred_keys]
    if unexplained_capture:
        raise RuntimeError(
            f"strict cocoa capture failures without verified u8-native subset-11 clearance: {unexplained_capture[:10]}"
        )

    summary = {
        "schema": "femacs-u8-cocoa-strict-adjudication-v1",
        "passed": True,
        "profile": cocoa_profile["id"],
        "entrypoint_count": accounting_res.get("original_file_count"),
        "raw_case_count": accounting_res.get("raw_case_count"),
        "raw_passed_count": accounting_res.get("raw_passed_count"),
        "classified_case_count": accounting_res.get("classified_case_count"),
        "raw_blocking_case_failure_count": accounting_res.get("blocking_case_failure_count"),
        "structural_failure_count": accounting_res.get("structural_failure_count"),
        "deferred_to_u8_native_count": len(deferred_to_native),
        "deferred_to_u8_native": deferred_to_native,
        "skipped_ordinarily_passing": skipped_passing,
        "accounting_path": str(adjudication_out / "accounting.json"),
    }
    (adjudication_out / "results.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def group_corpus_555(
    out: Path,
    ref_bin: Path,
    ref_dump: Path,
    ref_lisp: Path,
    native_root: Path,
    *,
    ref_data: Path | None = None,
    baseline_runtime_root: Path | None = None,
    reference_selection: Path | None = None,
    control_runs: list[tuple[Path, Path]] | None = None,
    remote_contract_runs: list[tuple[Path, Path]] | None = None,
    library_contract_runs: list[Path] | None = None,
    eglot_insert_replace_probe: Path | None = None,
    eglot_text_edit_probe: Path | None = None,
) -> dict:
    started = time.monotonic()
    out.mkdir(parents=True, exist_ok=True)
    logs_dir = out / "logs"
    corpus_tmp = out / "corpus-tmp"
    corpus_tmp.mkdir(parents=True, exist_ok=True)
    steps: list[dict] = []

    cocoa_bin = REPO / "dist/macos/Femacs.app/Contents/MacOS/Femacs"
    cocoa_dump = REPO / "dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"
    cocoa_lisp = (REPO / "dist/macos/Femacs.app/Contents/Resources/lisp").resolve()
    if baseline_runtime_root is not None:
        base_app = baseline_runtime_root / "dist/Femacs.app"
        strict_cocoa_bin = base_app / "Contents/MacOS/Femacs"
        strict_cocoa_dump = base_app / "Contents/Resources/libexec/Femacs.pdmp"
        strict_cocoa_lisp = (base_app / "Contents/Resources/lisp").resolve()
        for req_meta in (
            baseline_runtime_root / "runtime-binding.json",
            baseline_runtime_root / "runtime-product.json",
            baseline_runtime_root / "runtime-inputs.json",
        ):
            if not req_meta.is_file():
                raise FileNotFoundError(f"missing required baseline runtime metadata: {req_meta}")
    else:
        strict_cocoa_bin = cocoa_bin
        strict_cocoa_dump = cocoa_dump
        strict_cocoa_lisp = cocoa_lisp
    term_bin = REPO / "build/terminal/port/src/emacs"
    term_dump = REPO / "build/terminal/port/src/emacs.pdmp"
    u8_norm_root = native_root
    u8_chk_root = native_root / "checking"
    u8_norm_bin = u8_norm_root / "dist/Femacs.app/Contents/MacOS/Femacs"
    u8_norm_dump = u8_norm_root / "dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"
    u8_norm_lisp = (u8_norm_root / "dist/Femacs.app/Contents/Resources/lisp").resolve()
    u8_chk_bin = u8_chk_root / "dist/Femacs.app/Contents/MacOS/Femacs"
    u8_chk_dump = u8_chk_root / "dist/Femacs.app/Contents/Resources/libexec/Femacs.pdmp"
    u8_chk_lisp = (u8_chk_root / "dist/Femacs.app/Contents/Resources/lisp").resolve()
    resolved_ref_data = ref_data.resolve() if ref_data is not None else (ref_lisp.resolve().parent / "etc")

    for req_bin in (
        cocoa_bin,
        cocoa_dump,
        strict_cocoa_bin,
        strict_cocoa_dump,
        term_bin,
        term_dump,
        u8_norm_bin,
        u8_norm_dump,
        u8_chk_bin,
        u8_chk_dump,
    ):
        if not req_bin.is_file():
            raise FileNotFoundError(f"missing required candidate artifact for corpus-555: {req_bin}")
    for req_dir in (cocoa_lisp, strict_cocoa_lisp, u8_norm_lisp, u8_chk_lisp, ref_lisp, resolved_ref_data):
        if not req_dir.is_dir():
            raise FileNotFoundError(f"missing required directory for corpus-555: {req_dir}")

    # Put /usr/bin:/bin ahead of /opt/homebrew/bin so BSD /bin/stty precedes GNU coreutils stty
    # (preventing the Darwin PTY echo buffer overflow documented in docs/modernization.md:774),
    # while retaining trailing user PATH entries (e.g. ~/.cargo/bin for rust-analyzer).
    preferred_path = ["/usr/bin", "/bin", "/usr/sbin", "/sbin", "/opt/homebrew/bin", "/opt/homebrew/opt/llvm/bin"]
    trailing_path = [p for p in os.environ.get("PATH", "").split(os.pathsep) if p and p not in preferred_path]
    corpus_env = dict(os.environ)
    corpus_env["PATH"] = os.pathsep.join([*preferred_path, *trailing_path])
    corpus_env["LC_ALL"] = "C"
    corpus_env["TMPDIR"] = str(corpus_tmp)
    strict_cleanup_dirs: list[Path] = []

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
        meta_path = dest / ".side-artifact-meta.json"
        expected_meta = {
            "program_sha256": sha256_file(prog),
            "dump_sha256": sha256_file(dmp),
            "lisp_root": str(lisp_root) if lisp_root is not None else None,
        }
        if (dest / "summary.json").is_file() and meta_path.is_file():
            try:
                existing = json.loads((dest / "summary.json").read_text(encoding="utf-8"))
                saved_meta = json.loads(meta_path.read_text(encoding="utf-8"))
                if len(existing.get("files", [])) == 555 and saved_meta == expected_meta:
                    return {
                        "name": f"reuse-completed-{label}",
                        "passed": True,
                        "exit_code": 0,
                        "seconds": 0.0,
                        "reused_files": 555,
                    }
            except (OSError, ValueError):
                pass
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
        step_rec = run_step(
            f"functional-corpus-555-{label}",
            cmd,
            cwd=REPO,
            env=corpus_env,
            timeout=2700,
            allow_codes=(0, 1),
            log_path=log_file,
        )
        if (dest / "summary.json").is_file():
            meta_path.write_text(json.dumps(expected_meta, indent=2) + "\n", encoding="utf-8")
        return step_rec

    ref_env_dict = {
        "FEMACS_GC_OBSERVATION": "off",
        "FEMACS_STABLE_PRODUCT_LISP": str(ref_lisp),
    }

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
        comp_json = dest / "stable-suite-completion.json"
        if run_json.is_file() and comp_json.is_file():
            try:
                existing = json.loads(run_json.read_text(encoding="utf-8"))
                comp_existing = json.loads(comp_json.read_text(encoding="utf-8"))
                if (
                    comp_existing.get("execution_complete") is True
                    and strict_subset_run_reusable(
                        existing,
                        subset_files,
                        sha256_file(cand_prog),
                        sha256_file(cand_dmp),
                        sha256_file(ref_bin),
                        sha256_file(ref_dump),
                        allow_tramp_only_incomplete=allow_tramp_only_incomplete,
                        expected_runtime_env={"reference": ref_env_dict, "candidate": cand_env_dict},
                    )
                    and strict_subset_containers_present(dest, subset_files)
                ):
                    if dest not in strict_cleanup_dirs:
                        strict_cleanup_dirs.append(dest)
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
            _cleanup_strict_containers(dest)
            _rmtree_unlocked(dest)
        subset_spec = out / f"{step_name}-subset.json"
        subset_spec.write_text(json.dumps(subset_files, indent=2) + "\n", encoding="utf-8")
        # Defer .noindex/<container> removal until group_corpus_555 finishes classification
        # so native-bootstrap-identity.py can verify retained comp-tests.el closure state.
        if dest not in strict_cleanup_dirs:
            strict_cleanup_dirs.append(dest)
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
                json.dumps(ref_env_dict),
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
            cocoa_lisp,
        )
        steps.append(fut_ref.result())
        steps.append(fut_cand.result())

    adjudication = adjudicate_555_runs(ref_corpus_out, cand_corpus_out)
    (out / "adjudication-555.json").write_text(
        json.dumps(adjudication, indent=2) + "\n", encoding="utf-8"
    )

    followup_files: list[str] = adjudication["followup_files"]
    strict_out = out / "strict-incomplete-followup"
    if followup_files:
        steps.append(
            run_strict_subset(
                "strict-incomplete-followup",
                followup_files,
                strict_cocoa_bin,
                strict_cocoa_dump,
                {
                    "FEMACS_GC_OBSERVATION": "off",
                    "FEMACS_GC_COLLECTOR": "major",
                    "FEMACS_STABLE_PRODUCT_LISP": str(strict_cocoa_lisp),
                },
                strict_out,
                logs_dir / "02-strict-incomplete-followup.log",
                case_timeout=600,
            )
        )
    elif strict_out.exists():
        _cleanup_strict_containers(strict_out)
        _rmtree_unlocked(strict_out)

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
    subset_11_set = set(subset_11)
    subset_10 = [f for f in subset_11 if f != "test/lisp/filenotify-tests.el"]
    subset_fn = ["test/lisp/filenotify-tests.el"]
    receipt_paths: list[Path] = []
    if strict_out.is_dir() and (strict_out / "stable-suite-run.json").is_file():
        receipt_paths.append(strict_out / "stable-suite-run.json")

    def _slice_reusable(
        dest: Path, files: list[str], v_bin: Path, v_dump: Path, v_env: dict[str, str]
    ) -> bool:
        run_json = dest / "stable-suite-run.json"
        comp_json = dest / "stable-suite-completion.json"
        if not (run_json.is_file() and comp_json.is_file()):
            return False
        try:
            existing = json.loads(run_json.read_text(encoding="utf-8"))
            comp_existing = json.loads(comp_json.read_text(encoding="utf-8"))
            return bool(
                comp_existing.get("execution_complete") is True
                and strict_subset_run_reusable(
                    existing,
                    files,
                    sha256_file(v_bin),
                    sha256_file(v_dump),
                    sha256_file(ref_bin),
                    sha256_file(ref_dump),
                    allow_tramp_only_incomplete=False,
                    expected_runtime_env={"reference": ref_env_dict, "candidate": v_env},
                )
                and strict_subset_containers_present(dest, files)
            )
        except (OSError, ValueError, KeyError, TypeError):
            return False

    def _execute_slice_with_incomplete_retry(
        step_prefix: str,
        files: list[str],
        v_bin: Path,
        v_dump: Path,
        v_env: dict[str, str],
        dest: Path,
    ) -> None:
        steps.append(
            run_strict_subset(
                step_prefix,
                files,
                v_bin,
                v_dump,
                v_env,
                dest,
                logs_dir / f"05-{step_prefix}.log",
                case_timeout=600,
            )
        )
        comp = json.loads((dest / "stable-suite-completion.json").read_text(encoding="utf-8"))
        if not comp.get("execution_complete"):
            _cleanup_strict_containers(dest)
            _rmtree_unlocked(dest)
            steps.append(
                run_strict_subset(
                    f"{step_prefix}-retry",
                    files,
                    v_bin,
                    v_dump,
                    v_env,
                    dest,
                    logs_dir / f"05-{step_prefix}-retry.log",
                    case_timeout=900,
                )
            )

    subset_variant_slices: dict[str, tuple[Path, Path, Path, dict[str, str], list[tuple[str, list[str], Path]]]] = {}
    for variant_label, v_root, v_bin, v_dump, v_lisp in (
        ("u8-native-normal", u8_norm_root, u8_norm_bin, u8_norm_dump, u8_norm_lisp),
        ("u8-native-checking", u8_chk_root, u8_chk_bin, u8_chk_dump, u8_chk_lisp),
    ):
        binding_cfg = json.loads((v_root / "runtime-binding.json").read_text(encoding="utf-8"))
        v_env: dict[str, str] = {
            "FEMACS_GC_OBSERVATION": "off",
            "FEMACS_GC_COLLECTOR": "young" if "collector" in str(binding_cfg.get("variant", "")) else "major",
            "FEMACS_STABLE_PRODUCT_LISP": str(v_lisp),
        }
        dest_11 = out / f"subset-11-{variant_label}"
        if _slice_reusable(dest_11, subset_11, v_bin, v_dump, v_env):
            steps.append(
                run_strict_subset(
                    f"subset-11-{variant_label}",
                    subset_11,
                    v_bin,
                    v_dump,
                    v_env,
                    dest_11,
                    logs_dir / f"05-subset-11-{variant_label}.log",
                    case_timeout=600,
                )
            )
            slices = [("subset-11", subset_11, dest_11)]
            receipt_paths.append(dest_11 / "stable-suite-run.json")
        else:
            if dest_11.exists():
                _cleanup_strict_containers(dest_11)
                _rmtree_unlocked(dest_11)
            slices = []
            for slice_tag, slice_files in (("subset-10", subset_10), ("subset-filenotify", subset_fn)):
                s_dest = out / f"{slice_tag}-{variant_label}"
                _execute_slice_with_incomplete_retry(
                    f"{slice_tag}-{variant_label}", slice_files, v_bin, v_dump, v_env, s_dest
                )
                slices.append((slice_tag, slice_files, s_dest))
                receipt_paths.append(s_dest / "stable-suite-run.json")
        subset_variant_slices[variant_label] = (v_root, v_bin, v_dump, v_env, slices)

    ref_selection_path = (
        reference_selection.resolve()
        if reference_selection is not None
        else (out / "reference-selection.json")
    )
    reuse_ref_selection = False
    if ref_selection_path.is_file():
        try:
            sel_doc = json.loads(ref_selection_path.read_text(encoding="utf-8"))
            sel_prod = sel_doc.get("product") or {}
            sel_id = sel_doc.get("identity") or {}
            expected_ref_prog = sha256_file(ref_bin)
            expected_ref_dmp = sha256_file(ref_dump)
            if (
                sel_doc.get("schema") == "femacs-reference-selection-v1"
                and sel_prod.get("program") == str(ref_bin)
                and sel_prod.get("dump") == str(ref_dump)
                and (sel_id.get("program") or {}).get("sha256") == expected_ref_prog
                and (sel_id.get("dump") or {}).get("sha256") == expected_ref_dmp
                and (sel_prod.get("resource_roots") or {}).get("lisp") == str(ref_lisp)
                and (sel_prod.get("resource_roots") or {}).get("data") == str(resolved_ref_data)
                and all(
                    (json.loads(rp.read_text(encoding="utf-8")).get("products_before", {}).get("reference", {}).get("program", {}).get("sha256") == expected_ref_prog)
                    and (json.loads(rp.read_text(encoding="utf-8")).get("products_before", {}).get("reference", {}).get("dump", {}).get("sha256") == expected_ref_dmp)
                    for rp in receipt_paths
                )
            ):
                reuse_ref_selection = True
        except (OSError, ValueError, TypeError, KeyError):
            reuse_ref_selection = False
    if reuse_ref_selection:
        steps.append({
            "name": "reuse-reference-selection",
            "passed": True,
            "exit_code": 0,
            "seconds": 0.0,
        })
    else:
        if reference_selection is not None:
            raise RuntimeError(f"provided reference-selection missing or mismatched: {ref_selection_path}")
        bind_cmd = [
            sys.executable,
            "tools/g3-reference-binding.py",
            "--program",
            str(ref_bin),
            "--dump",
            str(ref_dump),
            "--lisp-root",
            str(ref_lisp),
            "--data-root",
            str(resolved_ref_data),
            "--epoch",
            "installed-32.0.50-fd7afced403",
            "--out",
            str(ref_selection_path),
        ]
        for rp in receipt_paths:
            bind_cmd.extend(["--receipt", str(rp)])
        steps.append(
            run_step(
                "bind-reference-selection",
                bind_cmd,
                cwd=REPO,
                timeout=120,
                log_path=logs_dir / "05a-bind-reference-selection.log",
            )
        )

    extra_classification_args: list[str] = []
    for res_p, ctrl_p in (control_runs or []):
        extra_classification_args.extend(["--control-run", str(res_p), str(ctrl_p)])
    for res_p, ctrl_p in (remote_contract_runs or []):
        extra_classification_args.extend(["--remote-contract-run", str(res_p), str(ctrl_p)])
    for lib_p in (library_contract_runs or []):
        extra_classification_args.extend(["--library-contract-run", str(lib_p)])
    if eglot_insert_replace_probe is not None:
        extra_classification_args.extend(["--eglot-insert-replace-probe", str(eglot_insert_replace_probe)])
    if eglot_text_edit_probe is not None:
        extra_classification_args.extend(["--eglot-text-edit-probe", str(eglot_text_edit_probe)])

    def _run_scoped_assembly(
        step_name: str,
        dest: Path,
        v_root: Path,
        asm_out: Path,
        log_path: Path,
    ) -> tuple[dict, dict]:
        _rmtree_unlocked(asm_out)
        steps.append(
            run_step(
                step_name,
                [
                    sys.executable,
                    "tools/stable-classification.py",
                    "--scoped-assembly",
                    "--receipt",
                    str(dest),
                    "--approved-rules",
                    str(REPO / "tools/fixtures/stable-classification-source-rules.json"),
                    "--reference-selection",
                    str(ref_selection_path),
                    "--product",
                    str(v_root),
                    "--out",
                    str(asm_out),
                    *extra_classification_args,
                ],
                cwd=REPO,
                timeout=300,
                allow_codes=(0, 1),
                log_path=log_path,
            )
        )
        asm_res = json.loads((asm_out / "results.json").read_text(encoding="utf-8"))
        acct_path = asm_out / "accounting.json"
        acct_res = json.loads(acct_path.read_text(encoding="utf-8")) if acct_path.is_file() else {}
        return asm_res, acct_res

    def _assembly_accepted(res: dict) -> bool:
        return (
            res.get("passed") is True
            and res.get("structural_failure_count") == 0
            and res.get("blocking_case_failure_count") == 0
            and res.get("classified_corpus_accepted") is True
        )

    u8_subset_verdicts: dict[str, dict] = {}
    u8_native_verified_cases: dict[str, set[tuple[str, str]]] = {}
    for variant_label, (v_root, v_bin, v_dump, v_env, slices) in subset_variant_slices.items():
        parts: list[dict] = []
        verified_cases: set[tuple[str, str]] = set()
        for slice_tag, slice_files, slice_dest in slices:
            asm_out = out / f"{slice_tag}-{variant_label}-classification"
            asm_step_name = (
                f"scoped-assembly-{variant_label}"
                if slice_tag == "subset-11"
                else f"scoped-assembly-{slice_tag}-{variant_label}"
            )
            asm_log_name = (
                f"05b-scoped-assembly-{variant_label}.log"
                if slice_tag == "subset-11"
                else f"05b-scoped-assembly-{slice_tag}-{variant_label}.log"
            )
            asm_res, acct_res = _run_scoped_assembly(
                asm_step_name, slice_dest, v_root, asm_out, logs_dir / asm_log_name
            )
            if not _assembly_accepted(asm_res) and slice_tag == "subset-filenotify":
                for attempt in range(1, 5):
                    if not _is_reference_filenotify_monitor_flake(slice_dest, asm_res):
                        break
                    _cleanup_strict_containers(slice_dest)
                    _rmtree_unlocked(slice_dest)
                    steps.append(
                        run_strict_subset(
                            f"subset-filenotify-{variant_label}-flake-retry-{attempt}",
                            slice_files,
                            v_bin,
                            v_dump,
                            v_env,
                            slice_dest,
                            logs_dir / f"05-subset-filenotify-{variant_label}-flake-retry-{attempt}.log",
                            case_timeout=600,
                        )
                    )
                    comp_retry = json.loads((slice_dest / "stable-suite-completion.json").read_text(encoding="utf-8"))
                    if not comp_retry.get("execution_complete"):
                        break
                    asm_res, acct_res = _run_scoped_assembly(
                        f"scoped-assembly-{slice_tag}-{variant_label}-flake-retry-{attempt}",
                        slice_dest,
                        v_root,
                        asm_out,
                        logs_dir / f"05b-scoped-assembly-{slice_tag}-{variant_label}-flake-retry-{attempt}.log",
                    )
                    if _assembly_accepted(asm_res):
                        break
            if not _assembly_accepted(asm_res):
                raise RuntimeError(f"{variant_label} ({slice_tag}) scoped-assembly failed: {asm_res}")
            parts.append(asm_res)
            verified_cases.update(_verified_case_ids_in_scoped_assembly(slice_dest, acct_res))

        u8_native_verified_cases[variant_label] = verified_cases
        if len(parts) == 1:
            u8_subset_verdicts[variant_label] = parts[0]
        else:
            u8_subset_verdicts[variant_label] = {
                "passed": True,
                "profile": parts[0]["profile"],
                "entrypoints": sum((p.get("scope") or {}).get("entrypoints", 0) for p in parts),
                "cases": sum((p.get("scope") or {}).get("cases", 0) for p in parts),
                "bound_rule_count": sum(p.get("bound_rule_count", 0) for p in parts),
                "classified_case_count": sum(p.get("classified_case_count", 0) for p in parts),
                "blocking_case_failure_count": 0,
                "structural_failure_count": 0,
                "classified_corpus_accepted": True,
                "parts": parts,
            }

    strict_followup: dict = {}
    if strict_out.is_dir():
        strict_followup["primary"] = adjudicate_cocoa_strict_receipt(
            strict_out,
            strict_cocoa_bin,
            strict_cocoa_dump,
            strict_cocoa_lisp,
            ref_selection_path,
            out / "strict-incomplete-followup-classification",
            subset_11_files=subset_11_set,
            u8_native_verified_cases=u8_native_verified_cases,
            baseline_runtime_root=baseline_runtime_root,
            control_runs=control_runs,
            remote_contract_runs=remote_contract_runs,
            library_contract_runs=library_contract_runs,
            eglot_insert_replace_probe=eglot_insert_replace_probe,
            eglot_text_edit_probe=eglot_text_edit_probe,
        )
    for s_dir in strict_cleanup_dirs:
        _cleanup_strict_containers(s_dir)

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
        "reference_selection": str(ref_selection_path),
        "adjudication_555": {
            "file_count": adjudication["file_count"],
            "total_compared_cases": adjudication["total_compared_cases"],
            "case_totals": adjudication["case_totals"],
            "rule_category_totals": adjudication["rule_category_totals"],
            "incomplete_file_count": len(adjudication["incomplete_files"]),
            "non_parity_file_count": len(adjudication["non_parity_files"]),
            "followup_file_count": len(followup_files),
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
    cocoa_gate = REPO / "docs/baseline/bench-cocoa-ga8.tsv"
    term_gate = REPO / "docs/baseline/bench-terminal-ga8.tsv"
    for gate_file in (cocoa_gate, term_gate):
        if not gate_file.is_file():
            raise FileNotFoundError(f"missing bench --gate baseline: {gate_file}")
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
                "--gate",
                str(cocoa_gate),
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
                "--gate",
                str(term_gate),
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

    cocoa_regs = bench_cocoa_json.get("regressions") or []
    term_regs = bench_term_json.get("regressions") or []
    summary = {
        "group": "performance",
        "passed": bool(
            bench_cocoa_json.get("passed")
            and bench_term_json.get("passed")
            and not cocoa_regs
            and not term_regs
        ),
        "seconds": round(time.monotonic() - started, 3),
        "delivered_comparisons": delivered_json.get("comparisons", []),
        "modes_comparisons": modes_json.get("comparisons", []),
        "bench_cocoa_baseline": str(cocoa_gate.relative_to(REPO)),
        "bench_cocoa_baseline_sha256": sha256_file(cocoa_gate),
        "bench_cocoa_regressions": cocoa_regs,
        "bench_cocoa_stats": bench_cocoa_json.get("stats", []),
        "bench_cocoa_load_per_round": bench_cocoa_json.get("load_per_round", []),
        "bench_terminal_baseline": str(term_gate.relative_to(REPO)),
        "bench_terminal_baseline_sha256": sha256_file(term_gate),
        "bench_terminal_regressions": term_regs,
        "bench_terminal_stats": bench_term_json.get("stats", []),
        "bench_terminal_load_per_round": bench_term_json.get("load_per_round", []),
        "steps": steps,
    }
    if not summary["passed"]:
        raise RuntimeError(
            f"bench.py --gate failed: cocoa_regs={cocoa_regs}, term_regs={term_regs}"
        )
    (out / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def strict_subset_containers_present(dest: Path, subset_files: list[str]) -> bool:
    if "test/src/comp-tests.el" not in subset_files:
        return True
    envs = list((dest / "results").rglob("*src-comp-tests*.environment.json"))
    if len(envs) < 2:
        return False
    for env_json in envs:
        c = json.loads(env_json.read_text(encoding="utf-8")).get("container")
        if not isinstance(c, str) or not c or not Path(c).is_dir():
            return False
    return True


def strict_subset_run_reusable(
    existing: dict,
    subset_files: list[str],
    cand_prog_sha: str,
    cand_dmp_sha: str,
    ref_prog_sha: str,
    ref_dmp_sha: str,
    allow_tramp_only_incomplete: bool = False,
    expected_runtime_env: dict | None = None,
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
    env_ok = expected_runtime_env is None or existing.get("runtime_environment") == expected_runtime_env
    return (
        sorted(existing.get("subset") or []) == sorted(subset_files)
        and len(ex_files) == len(subset_files)
        and completion_ok
        and env_ok
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
        rules = load_source_rule_index(REPO / "tools/fixtures/stable-classification-source-rules.json")
        known_key = next(iter(rules))
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
        # Blind (file, id) presence + "Femacs" diagnostic must NOT bypass port-worse in pre-screen.
        self.assertEqual(
            classify_case_pair(
                known_key[0],
                known_key[1],
                {"status": "passed", "diagnostic": ""},
                {"status": "failed", "diagnostic": '(ert-test-failed ("Femacs" "32.0.50"))'},
                rules,
            ),
            ("port-worse", None),
        )
        self.assertEqual(
            classify_case_pair(
                known_key[0],
                known_key[1],
                {"status": "passed", "diagnostic": ""},
                {"status": "skipped", "diagnostic": "(ert-test-skipped ((skip-unless nil)))"},
                rules,
            ),
            ("unclassified-skip-diff", None),
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
        # Ad-hoc native-compile skips must route to strict stable-classification, never self-waive.
        self.assertEqual(
            classify_case_pair(
                "test/lisp/emacs-lisp/comp-tests.el",
                "test-native-compile-prune-cache",
                {"status": "passed", "diagnostic": ""},
                {"status": "skipped", "diagnostic": "(ert-test-skipped ((skip-unless (featurep 'native-compile))))"},
                rules,
            ),
            ("unclassified-skip-diff", None),
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

    def test_adjudicate_555_runs_routes_non_parity_and_rejects_femacs_name_waiver(self):
        import tempfile

        rules = load_source_rule_index(REPO / "tools/fixtures/stable-classification-source-rules.json")
        known_file, known_id = next(iter(rules))
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            ref_res = root / "ref/results"
            cand_res = root / "cand/results"
            ref_res.mkdir(parents=True)
            cand_res.mkdir(parents=True)
            (ref_res / "a.summary.json").write_text(json.dumps({
                "file": known_file,
                "input_matches": True,
                "registered": True,
                "cases": {known_id: {"status": "passed", "diagnostic": ""}},
            }), encoding="utf-8")
            (cand_res / "a.summary.json").write_text(json.dumps({
                "file": known_file,
                "input_matches": True,
                "registered": True,
                "cases": {known_id: {"status": "failed", "diagnostic": '(ert-test-failed ("Femacs" "32.0.50"))'}},
            }), encoding="utf-8")
            adj = adjudicate_555_runs(root / "ref", root / "cand")
            self.assertEqual(adj["blocking_case_count"], 1)
            self.assertEqual(adj["followup_files"], [known_file])
            self.assertEqual(adj["non_parity_files"], [known_file])

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

    def test_bench_gate_rejects_regression_above_tolerance(self):
        spec = importlib.util.spec_from_file_location("femacs_bench_gate", REPO / "tools/bench.py")
        bench_mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(bench_mod)
        for tsv_name in ("bench-terminal-ga8.tsv", "bench-cocoa-ga8.tsv"):
            baseline = bench_mod.load_baseline(REPO / "docs/baseline" / tsv_name)
            summary = [
                {"side": "candidate", "case": case, "median": val}
                for case, val in baseline.items()
            ]
            self.assertEqual(bench_mod.gate_rows(summary, baseline, 0.15), [])
            regressed = [
                {"side": "candidate", "case": case, "median": val * 1.187 if case == "gc" else val}
                for case, val in baseline.items()
            ]
            self.assertEqual(bench_mod.gate_rows(regressed, baseline, 0.15), ["gc"])

    def test_is_reference_filenotify_monitor_flake_contract(self):
        import tempfile

        with tempfile.TemporaryDirectory() as td:
            dest = Path(td)
            prog_dir = dest / "results/reference"
            prog_dir.mkdir(parents=True)
            prog_path = prog_dir / "fn.progress.jsonl"
            (dest / "stable-suite-run.json").write_text(
                json.dumps({
                    "files": [
                        {
                            "file": "test/lisp/filenotify-tests.el",
                            "reference": {"progress": {"path": "results/reference/fn.progress.jsonl"}},
                        }
                    ]
                }),
                encoding="utf-8",
            )
            prog_path.write_text(
                '{"event":"test-ended","status":"skipped","condition":"(ert-test-skipped \\"Cannot determine test monitor\\")"}\n',
                encoding="utf-8",
            )
            good_flake = {
                "structural_failure_count": 0,
                "blocking_case_failure_count": 1,
                "blocking_cases": [
                    {
                        "file": "test/lisp/filenotify-tests.el",
                        "id": "file-notify-test03-events-remote",
                        "candidate_status": "passed",
                    }
                ],
            }
            self.assertTrue(_is_reference_filenotify_monitor_flake(dest, good_flake))
            self.assertFalse(
                _is_reference_filenotify_monitor_flake(
                    dest,
                    {
                        "structural_failure_count": 0,
                        "blocking_case_failure_count": 1,
                        "blocking_cases": [
                            {
                                "file": "test/lisp/filenotify-tests.el",
                                "id": "file-notify-test03-events-remote",
                                "candidate_status": "failed",
                            }
                        ],
                    },
                )
            )
            self.assertFalse(
                _is_reference_filenotify_monitor_flake(
                    dest, dict(good_flake, structural_failure_count=1)
                )
            )
            self.assertFalse(
                _is_reference_filenotify_monitor_flake(
                    dest,
                    {
                        "structural_failure_count": 0,
                        "blocking_case_failure_count": 1,
                        "blocking_cases": [
                            {
                                "file": "test/lisp/net/tramp-tests.el",
                                "id": "tramp-test30-make-process",
                                "candidate_status": "passed",
                            }
                        ],
                    },
                )
            )
            prog_path.write_text('{"event":"test-ended","status":"failed"}\n', encoding="utf-8")
            self.assertFalse(_is_reference_filenotify_monitor_flake(dest, good_flake))


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
    parser.add_argument("--reference-data", type=Path)
    parser.add_argument(
        "--control-run",
        type=Path,
        nargs=2,
        action="append",
        default=[],
        metavar=("RESULTS", "CONTROLLER"),
    )
    parser.add_argument(
        "--remote-contract-run",
        type=Path,
        nargs=2,
        action="append",
        default=[],
        metavar=("RESULTS", "CONTROLLER"),
    )
    parser.add_argument(
        "--library-contract-run",
        type=Path,
        action="append",
        default=[],
    )
    parser.add_argument("--eglot-insert-replace-probe", type=Path)
    parser.add_argument("--eglot-text-edit-probe", type=Path)
    parser.add_argument("--baseline-runtime-root", type=Path)
    parser.add_argument("--reference-selection", type=Path)
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
            ref_data=args.reference_data.resolve() if args.reference_data else None,
            baseline_runtime_root=(
                args.baseline_runtime_root.resolve() if args.baseline_runtime_root else None
            ),
            reference_selection=(
                args.reference_selection.resolve() if args.reference_selection else None
            ),
            control_runs=[(a.resolve(), b.resolve()) for a, b in args.control_run],
            remote_contract_runs=[(a.resolve(), b.resolve()) for a, b in args.remote_contract_run],
            library_contract_runs=[p.resolve() for p in args.library_contract_run],
            eglot_insert_replace_probe=(
                args.eglot_insert_replace_probe.resolve() if args.eglot_insert_replace_probe else None
            ),
            eglot_text_edit_probe=args.eglot_text_edit_probe.resolve() if args.eglot_text_edit_probe else None,
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
