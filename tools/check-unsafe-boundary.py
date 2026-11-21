#!/usr/bin/env python3
"""Audit Rust unsafe boundaries, Cargo source scope, and binary VM/ABI invariants.

Implements the static and binary verification gates for U1-U5 / AU1-AU5:
  * Discovers actual Cargo workspace packages and target-compiled source files.
  * Measures `#![forbid(unsafe_code)]`, file-level `#![allow(unsafe_op_in_unsafe_fn)]`
    and `#![allow(static_mut_refs)]`, `unsafe fn` definitions, `unsafe { ... }` blocks,
    `SAFETY:` documentation coverage in explicit low-level boundary modules, `static mut`
    items, `extern "C"` definitions/declarations, and non-generated `include!` edges.
  * Verifies on linked binaries (`nm`) that VM/pdump singleton state (`lispsym`,
    `globals`, `dump_hooks`, `nr_dump_hooks`, `main_thread`) exists as a single
    instance and that required external ABI entrypoints (`Fmodule_load`, Cocoa
    Swift-imported symbols actually referenced by the link, and native `.eln`
    helper exports when enabled) survive link optimization.

Streaming, file-bounded, stdlib-only.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

GENERATED_INCLUDE_BASENAMES = frozenset({
    "config.rs",
    "globals.rs",
    "symbol-names.rs",
    "epaths.rs",
    "swift-exports.rs",
    "runtime.rs",
    "buildobj.rs",
    "macuvs.rs",
    "charprop.rs",
    "ftoastr.rs",
    "strftime.rs",
})

GENERATED_INCLUDE_PREFIXES = ("uni-",)

PORTED_TREES = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "nt", "exec", "msdos")
NATIVE_CRATE_TREES = ("crates", "xtask", "cli", "packaging")

PATH_ATTR_RE = re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]')
INNER_ALLOW_RE = re.compile(r"#!\s*\[\s*allow\s*\((.*?)\)\s*\]", re.DOTALL)
FORBID_UNSAFE_RE = re.compile(r"#!?\s*\[\s*forbid\s*\(\s*unsafe_code\s*\)\s*\]")
EXTERN_BLOCK_RE = re.compile(r"\b(?:unsafe\s+)?extern\s+\{")
STATIC_MUT_RE = re.compile(r"\bstatic\s+mut\s+[A-Za-z_]\w*")
INCLUDE_RE = re.compile(r'\binclude!\s*\(\s*"([^"]+)"\s*\)')
FN_SIG_RE = re.compile(
    r"(?m)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?"
    r"(?:const\s+|async\s+)*(?P<unsafe>unsafe\s+)?"
    r"(?P<extern>extern\s+)?fn\s+(?P<name>[A-Za-z_]\w*)\b"
)
UNSAFE_BLOCK_RE = re.compile(r"\bunsafe\s*\{")
SAFETY_COMMENT_RE = re.compile(r"(?i)\bSAFETY\s*:")

NATIVE_COMP_REQUIRED_EXPORTS = (
    "Fcomp_el_to_eln_filename",
    "Fnative_elisp_load",
    "hash_native_abi",
    "load_comp_unit",
    "syms_of_comp",
    "wrong_type_argument",
    "push_handler",
    "record_unwind_protect_excursion",
    "slow_eq",
    "set_internal",
    "specbind",
)


def strip_comments_and_strings(source: str) -> str:
    """Replace comments and string literals with spaces while preserving newlines."""
    out: list[str] = []
    n = len(source)
    i = 0
    block_depth = 0
    while i < n:
        ch = source[i]
        if block_depth > 0:
            if ch == "/" and i + 1 < n and source[i + 1] == "*":
                block_depth += 1
                out.append("  ")
                i += 2
            elif ch == "*" and i + 1 < n and source[i + 1] == "/":
                block_depth -= 1
                out.append("  ")
                i += 2
            else:
                out.append("\n" if ch == "\n" else " ")
                i += 1
            continue
        if ch == "/" and i + 1 < n and source[i + 1] == "/":
            while i < n and source[i] != "\n":
                out.append(" ")
                i += 1
            continue
        if ch == "/" and i + 1 < n and source[i + 1] == "*":
            block_depth = 1
            out.append("  ")
            i += 2
            continue
        if ch in ("b", "c") and i + 1 < n and source[i + 1] == "r":
            j = i + 2
            hashes = 0
            while j < n and source[j] == "#":
                hashes += 1
                j += 1
            if j < n and source[j] == '"':
                out.append(" " * (j - i + 1))
                i = j + 1
                closer = '"' + ("#" * hashes)
                end = source.find(closer, i)
                if end == -1:
                    end = n
                for k in range(i, min(end + len(closer), n)):
                    out.append("\n" if source[k] == "\n" else " ")
                i = min(end + len(closer), n)
                continue
        if ch == "r":
            j = i + 1
            hashes = 0
            while j < n and source[j] == "#":
                hashes += 1
                j += 1
            if j < n and source[j] == '"' and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")):
                out.append(" " * (j - i + 1))
                i = j + 1
                closer = '"' + ("#" * hashes)
                end = source.find(closer, i)
                if end == -1:
                    end = n
                for k in range(i, min(end + len(closer), n)):
                    out.append("\n" if source[k] == "\n" else " ")
                i = min(end + len(closer), n)
                continue
        if ch == '"':
            out.append(" ")
            i += 1
            while i < n:
                c2 = source[i]
                if c2 == "\\":
                    out.append("  ")
                    i += 2
                    continue
                out.append("\n" if c2 == "\n" else " ")
                i += 1
                if c2 == '"':
                    break
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def extract_include_targets(raw: str, scrubbed: str) -> list[str]:
    targets: list[str] = []
    for m in re.finditer(r"\binclude!\s*\(", scrubbed):
        tail = raw[m.end() :]
        m_str = re.match(r'\s*"([^"]+)"\s*\)', tail)
        if m_str:
            targets.append(m_str.group(1))
    return targets


def extract_path_attrs(raw: str, scrubbed: str) -> list[str]:
    targets: list[str] = []
    for m in re.finditer(r"#\s*\[\s*path\s*=", scrubbed):
        tail = raw[m.end() :]
        m_str = re.match(r'\s*"([^"]+)"\s*\]', tail)
        if m_str:
            targets.append(m_str.group(1))
    return targets


def is_generated_data_include(target: str) -> bool:
    base = Path(target).name
    if base in GENERATED_INCLUDE_BASENAMES:
        return True
    return any(base.startswith(prefix) for prefix in GENERATED_INCLUDE_PREFIXES)


def is_fn_definition(scrubbed: str, match_end: int) -> bool:
    """Distinguish `fn foo(...) { ... }` definitions from foreign `fn foo(...);` decls."""
    n = len(scrubbed)
    i = match_end
    paren_depth = 0
    started_params = False
    while i < n:
        ch = scrubbed[i]
        if ch == "(":
            paren_depth += 1
            started_params = True
        elif ch == ")":
            paren_depth -= 1
            if started_params and paren_depth <= 0:
                i += 1
                break
        i += 1
    angle_depth = 0
    while i < n:
        ch = scrubbed[i]
        if ch == "<":
            angle_depth += 1
        elif ch == ">" and (i == 0 or scrubbed[i - 1] != "-"):
            if angle_depth > 0:
                angle_depth -= 1
        elif angle_depth == 0:
            if ch == "{":
                return True
            if ch == ";":
                return False
        i += 1
    return False


def analyze_rust_file(path: Path, rel: str) -> dict:
    raw = path.read_text(encoding="utf-8", errors="replace")
    scrubbed = strip_comments_and_strings(raw)
    lines_raw = raw.splitlines()

    allow_tokens: set[str] = set()
    for m in INNER_ALLOW_RE.finditer(scrubbed):
        for tok in m.group(1).split(","):
            tok = tok.strip()
            if tok:
                allow_tokens.add(tok)

    forbid_unsafe = bool(FORBID_UNSAFE_RE.search(scrubbed))
    allow_unsafe_op = "unsafe_op_in_unsafe_fn" in allow_tokens
    allow_static_mut_refs = "static_mut_refs" in allow_tokens

    fn_defs = 0
    unsafe_fn_defs = 0
    extern_c_fn_defs = 0
    for m in FN_SIG_RE.finditer(scrubbed):
        if not is_fn_definition(scrubbed, m.end()):
            continue
        fn_defs += 1
        if m.group("unsafe"):
            unsafe_fn_defs += 1
        if m.group("extern"):
            extern_c_fn_defs += 1

    # Locate unsafe blocks (excluding `unsafe fn` and `unsafe extern` headers).
    unsafe_blocks = 0
    safety_documented_blocks = 0
    for m in UNSAFE_BLOCK_RE.finditer(scrubbed):
        unsafe_blocks += 1
        line_idx = scrubbed.count("\n", 0, m.start())
        win_start = max(0, line_idx - 5)
        win_end = min(len(lines_raw), line_idx + 4)
        window_text = "\n".join(lines_raw[win_start:win_end])
        if SAFETY_COMMENT_RE.search(window_text):
            safety_documented_blocks += 1

    all_includes = extract_include_targets(raw, scrubbed)
    stitching_includes = [inc for inc in all_includes if not is_generated_data_include(inc)]
    generated_includes = [inc for inc in all_includes if is_generated_data_include(inc)]

    return {
        "file": rel,
        "lines": len(lines_raw),
        "forbid_unsafe_code": forbid_unsafe,
        "allow_unsafe_op_in_unsafe_fn": allow_unsafe_op,
        "allow_static_mut_refs": allow_static_mut_refs,
        "fn_defs": fn_defs,
        "unsafe_fn_defs": unsafe_fn_defs,
        "extern_c_fn_defs": extern_c_fn_defs,
        "extern_c_blocks": len(EXTERN_BLOCK_RE.findall(scrubbed)),
        "unsafe_blocks": unsafe_blocks,
        "safety_documented_blocks": safety_documented_blocks,
        "static_mut_items": len(STATIC_MUT_RE.findall(scrubbed)),
        "stitching_includes": len(stitching_includes),
        "generated_includes": len(generated_includes),
    }


def parse_workspace_members(repo: Path) -> list[str]:
    cargo_toml = repo / "Cargo.toml"
    if not cargo_toml.is_file():
        return []
    text = strip_comments_and_strings(cargo_toml.read_text(encoding="utf-8"))
    m = re.search(r"members\s*=\s*\[(.*?)\]", text, re.DOTALL)
    if not m:
        return []
    raw_cargo = cargo_toml.read_text(encoding="utf-8")
    m_raw = re.search(r"members\s*=\s*\[(.*?)\]", raw_cargo, re.DOTALL)
    if not m_raw:
        return []
    return re.findall(r'"([^"]+)"', m_raw.group(1))


def summarize_rows(rows: list[dict]) -> dict:
    return {
        "total_files": len(rows),
        "forbid_unsafe_files": sum(1 for r in rows if r["forbid_unsafe_code"]),
        "allow_unsafe_op_files": sum(1 for r in rows if r["allow_unsafe_op_in_unsafe_fn"]),
        "allow_static_mut_refs_files": sum(1 for r in rows if r["allow_static_mut_refs"]),
        "fn_defs": sum(r["fn_defs"] for r in rows),
        "unsafe_fn_defs": sum(r["unsafe_fn_defs"] for r in rows),
        "extern_c_fn_defs": sum(r["extern_c_fn_defs"] for r in rows),
        "extern_c_blocks": sum(r["extern_c_blocks"] for r in rows),
        "unsafe_blocks": sum(r["unsafe_blocks"] for r in rows),
        "safety_documented_blocks": sum(r["safety_documented_blocks"] for r in rows),
        "static_mut_items": sum(r["static_mut_items"] for r in rows),
        "stitching_includes": sum(r["stitching_includes"] for r in rows),
        "generated_includes": sum(r["generated_includes"] for r in rows),
    }


def discover_active_product_files(repo: Path, workspace_members: list[str]) -> set[str]:
    """Return repo-relative POSIX paths of Rust files mounted by active Cargo workspace crates."""
    active: set[str] = set()
    queue: list[Path] = []
    for member in workspace_members:
        member_dir = repo / member
        if not member_dir.exists():
            continue
        member_toml = member_dir / "Cargo.toml"
        if member_toml.is_file():
            for rel_bin in re.findall(r'path\s*=\s*"([^"]+\.rs)"', member_toml.read_text(encoding="utf-8")):
                cand = (member_dir / rel_bin).resolve()
                if cand.is_file() and cand.is_relative_to(repo):
                    rel = cand.relative_to(repo).as_posix()
                    if rel not in active:
                        active.add(rel)
                        queue.append(cand)
        for rs in sorted(member_dir.rglob("*.rs")):
            if "target" in rs.parts or "build" in rs.parts:
                continue
            resolved = rs.resolve()
            if resolved.is_relative_to(repo):
                rel = resolved.relative_to(repo).as_posix()
                if rel not in active:
                    active.add(rel)
                    queue.append(resolved)

    while queue:
        cur = queue.pop()
        raw = cur.read_text(encoding="utf-8", errors="replace")
        scrubbed = strip_comments_and_strings(raw)
        parent = cur.parent
        for ref in (*extract_path_attrs(raw, scrubbed), *extract_include_targets(raw, scrubbed)):
            cand = (parent / ref).resolve()
            if cand.is_file() and cand.is_relative_to(repo):
                rel = cand.relative_to(repo).as_posix()
                if rel not in active:
                    active.add(rel)
                    queue.append(cand)
    return active


def collect_repo_metrics(repo: Path) -> dict:
    workspace_members = parse_workspace_members(repo)
    active_set = discover_active_product_files(repo, workspace_members)
    files_data: list[dict] = []
    seen: set[str] = set()

    roots: list[Path] = []
    for member in workspace_members:
        p = repo / member
        if p.exists():
            roots.append(p)
    for name in (*PORTED_TREES, *NATIVE_CRATE_TREES):
        p = repo / name
        if p.exists():
            roots.append(p)

    for root in roots:
        for rs in sorted(root.rglob("*.rs")):
            if "target" in rs.parts or "build" in rs.parts:
                continue
            rel = rs.relative_to(repo).as_posix()
            if rel in seen:
                continue
            seen.add(rel)
            row = analyze_rust_file(rs, rel)
            row["active_product"] = rel in active_set
            files_data.append(row)

    summary = summarize_rows(files_data)
    active_product = summarize_rows([r for r in files_data if r["active_product"]])
    inactive_sources = summarize_rows([r for r in files_data if not r["active_product"]])
    return {
        "schema": "femacs-unsafe-boundary-v1",
        "workspace_members": workspace_members,
        "summary": summary,
        "active_product": active_product,
        "inactive_sources": inactive_sources,
        "files": files_data,
    }


def check_nm_symbols(nm_lines: list[str], kind: str, native_comp: bool, require_modules: bool) -> list[str]:
    """Validate single-VM-state uniqueness and required external ABI exports from `nm` lines."""
    errors: list[str] = []
    defined_unmangled: dict[str, int] = {}
    defined_any: set[str] = set()
    exported_global: set[str] = set()
    undefined_syms: set[str] = set()

    mangled_singletons = {
        "dump_hooks": 0,
        "nr_dump_hooks": 0,
        "main_thread": 0,
    }

    for line in nm_lines:
        parts = line.split()
        if not parts:
            continue
        if len(parts) == 2 and parts[0] in ("U", "u"):
            undefined_syms.add(parts[1].removeprefix("_"))
            continue
        if len(parts) < 3:
            continue
        sym_type = parts[1]
        raw_sym = parts[2]
        sym = raw_sym.removeprefix("_")
        if sym_type in ("U", "u"):
            undefined_syms.add(sym)
            continue
        defined_any.add(sym)
        defined_unmangled[sym] = defined_unmangled.get(sym, 0) + 1
        if sym_type.isupper():
            exported_global.add(sym)
        if sym.startswith("_RNv"):
            for key in mangled_singletons:
                needle = f"{len(key)}{key}"
                if sym.endswith(needle):
                    mangled_singletons[key] += 1

    lispsym_count = defined_unmangled.get("lispsym", 0)
    if lispsym_count != 1:
        errors.append(f"expected exactly 1 defined `lispsym` instance, found {lispsym_count}")

    for unmangled_singleton in ("Vmain_thread",):
        count = defined_unmangled.get(unmangled_singleton, 0)
        if count > 1:
            errors.append(f"duplicate VM singleton `{unmangled_singleton}`: found {count} instances")

    for key, count in mangled_singletons.items():
        unmangled_count = defined_unmangled.get(key, 0)
        total = count + unmangled_count
        if total > 1:
            errors.append(f"duplicate VM/pdumper static `{key}` across crates: found {total} instances")

    if require_modules and "Fmodule_load" not in defined_any:
        errors.append("dynamic module entrypoint `Fmodule_load` missing from linked binary")

    if kind == "cocoa":
        if "ns_term_init" not in defined_any and "Fx_open_connection" not in defined_any:
            errors.append("Cocoa display initialization entrypoints missing from linked binary")

    if native_comp:
        for req in NATIVE_COMP_REQUIRED_EXPORTS:
            if req not in defined_any:
                errors.append(f"native-comp required helper `{req}` missing from linked binary")

    return errors


def inspect_binary(binary: Path, kind: str, native_comp: bool, require_modules: bool) -> list[str]:
    if not binary.is_file():
        return [f"binary does not exist: {binary}"]
    proc = subprocess.run(
        ["nm", str(binary)],
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        return [f"nm failed on {binary}: {proc.stderr.strip()}"]
    return check_nm_symbols(proc.stdout.splitlines(), kind, native_comp, require_modules)


def run_selftest() -> int:
    failures = 0

    def case(name: str, fn) -> None:
        nonlocal failures
        t0 = time.monotonic()
        try:
            fn()
            dt = time.monotonic() - t0
            print(f"CASE {name} seconds={dt:.6f} status=passed")
        except AssertionError as exc:
            dt = time.monotonic() - t0
            failures += 1
            print(f"CASE {name} seconds={dt:.6f} status=failed error={exc}")

    def test_source_metrics() -> None:
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / "sample.rs"
            p.write_text(
                '#![allow(static_mut_refs, unsafe_op_in_unsafe_fn)]\n'
                'include!("include/config.rs");\n'
                'include!("include/lisp.rs");\n'
                'unsafe extern "C" { pub fn foreign_call(x: i32) -> i32; }\n'
                'static mut COUNTER: i32 = 0;\n'
                'pub unsafe extern "C" fn exported(x: i32) -> i32 {\n'
                '    // SAFETY: single-threaded test fixture.\n'
                '    unsafe { COUNTER += foreign_call(x); COUNTER }\n'
                '}\n'
                'pub fn pure_add(a: i32, b: i32) -> i32 { a + b }\n',
                encoding="utf-8",
            )
            res = analyze_rust_file(p, "src/sample.rs")
            assert res["allow_unsafe_op_in_unsafe_fn"] is True, res
            assert res["allow_static_mut_refs"] is True, res
            assert res["forbid_unsafe_code"] is False, res
            assert res["fn_defs"] == 2, res
            assert res["unsafe_fn_defs"] == 1, res
            assert res["extern_c_fn_defs"] == 1, res
            assert res["extern_c_blocks"] == 1, res
            assert res["unsafe_blocks"] == 1, res
            assert res["safety_documented_blocks"] == 1, res
            assert res["static_mut_items"] == 1, res
            assert res["generated_includes"] == 1, res
            assert res["stitching_includes"] == 1, res

    def test_forbid_detection() -> None:
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / "pure.rs"
            p.write_text(
                "#![forbid(unsafe_code)]\n"
                'const NOTE: &str = "unsafe fn fake() {}";\n'
                "pub fn sum(xs: &[u32]) -> u32 { xs.iter().sum() }\n",
                encoding="utf-8",
            )
            res = analyze_rust_file(p, "crates/pure.rs")
            assert res["forbid_unsafe_code"] is True, res
            assert res["unsafe_fn_defs"] == 0, res
            assert res["unsafe_blocks"] == 0, res

    def test_nm_singleton_and_abi() -> None:
        good_nm = [
            "000000010035d7a8 B _lispsym",
            "00000001001b35d4 T _Fmodule_load",
            "00000001004159d8 b __RNvCs111_7pdumper10dump_hooks",
        ]
        assert check_nm_symbols(good_nm, "terminal", False, True) == []

        dup_nm = good_nm + [
            "00000001005159d8 b __RNvCs222_5other10dump_hooks",
            "000000010065d7a8 B _lispsym",
        ]
        errs = check_nm_symbols(dup_nm, "terminal", False, True)
        assert len(errs) == 2, errs

    def test_comment_and_string_includes_ignored() -> None:
        with tempfile.TemporaryDirectory() as tmp:
            p = Path(tmp) / "driver.rs"
            p.write_text(
                '// comment mentioning include!("include/lisp.rs");\n'
                '/* block comment include!("other.rs"); */\n'
                'pub fn emit() -> String { format!("include!({:?});", "foo.rs") }\n',
                encoding="utf-8",
            )
            res = analyze_rust_file(p, "xtask/src/driver.rs")
            assert res["stitching_includes"] == 0, res
            assert res["generated_includes"] == 0, res

    case("source_metrics_and_foreign_decl_separation", test_source_metrics)
    case("forbid_unsafe_ignores_string_literals", test_forbid_detection)
    case("nm_duplicate_vm_state_and_abi_gate", test_nm_singleton_and_abi)
    case("comment_and_string_includes_ignored", test_comment_and_string_includes_ignored)
    return 1 if failures else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--repo", type=Path, default=REPO)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--out", type=Path, help="Write full JSON report to path")
    ap.add_argument("--baseline", type=Path, help="Compare summary against baseline JSON")
    ap.add_argument("--ratchet", action="store_true", help="Fail if unsafe metrics regress above baseline")
    ap.add_argument(
        "--require-no-file-allows",
        action="store_true",
        help="Fail if any active product file retains #![allow(unsafe_op_in_unsafe_fn)] or #![allow(static_mut_refs)]",
    )
    ap.add_argument(
        "--require-zero-active-stitching",
        action="store_true",
        help="Fail if any active Cargo product file retains unit/mirror stitching include!(...)",
    )
    ap.add_argument(
        "--require-forbid",
        action="append",
        default=[],
        help="Relative file or directory prefix that must carry #![forbid(unsafe_code)] and 0 unsafe",
    )
    ap.add_argument("--binary", type=Path, help="Linked executable to check with nm")
    ap.add_argument("--kind", choices=("terminal", "cocoa"), default="terminal")
    ap.add_argument("--native-comp", action="store_true")
    ap.add_argument("--require-modules", action="store_true")
    args = ap.parse_args()

    if args.selftest:
        return run_selftest()

    report = collect_repo_metrics(args.repo.resolve())
    summary = report["summary"]
    active_product = report["active_product"]
    errors: list[str] = []

    if args.require_no_file_allows:
        if active_product["allow_unsafe_op_files"] > 0:
            errors.append(
                f"found {active_product['allow_unsafe_op_files']} active file(s) with #![allow(unsafe_op_in_unsafe_fn)]"
            )
        if active_product["allow_static_mut_refs_files"] > 0:
            errors.append(
                f"found {active_product['allow_static_mut_refs_files']} active file(s) with #![allow(static_mut_refs)]"
            )

    if args.require_zero_active_stitching and active_product["stitching_includes"] > 0:
        bad = [
            r["file"]
            for r in report["files"]
            if r["active_product"] and r["stitching_includes"] > 0
        ]
        errors.append(
            f"found {active_product['stitching_includes']} stitching include!(...) in active product files: {bad}"
        )

    for prefix in args.require_forbid:
        matched = [
            f for f in report["files"] if f["file"] == prefix or f["file"].startswith(prefix.rstrip("/") + "/")
        ]
        if not matched:
            errors.append(f"--require-forbid target matched no Rust files: {prefix}")
            continue
        has_crate_forbid = any(f["forbid_unsafe_code"] for f in matched)
        if not has_crate_forbid:
            errors.append(f"--require-forbid target lacks `#![forbid(unsafe_code)]`: {prefix}")
        for f in matched:
            if f["unsafe_fn_defs"] > 0 or f["unsafe_blocks"] > 0 or f["extern_c_blocks"] > 0:
                errors.append(f"--require-forbid target contains unsafe/FFI items: {f['file']}")

    if args.baseline:
        base = json.loads(args.baseline.read_text(encoding="utf-8"))
        base_sum = base["summary"]
        deltas = {k: summary[k] - base_sum.get(k, 0) for k in summary if isinstance(summary[k], int)}
        report["baseline_deltas"] = deltas
        if args.ratchet:
            for key in (
                "allow_unsafe_op_files",
                "allow_static_mut_refs_files",
                "unsafe_fn_defs",
                "static_mut_items",
                "stitching_includes",
            ):
                if deltas.get(key, 0) > 0:
                    errors.append(f"metric `{key}` regressed above baseline by +{deltas[key]}")

    if args.binary:
        bin_errors = inspect_binary(args.binary, args.kind, args.native_comp, args.require_modules)
        report["binary_check"] = {
            "binary": str(args.binary),
            "kind": args.kind,
            "errors": bin_errors,
        }
        errors.extend(bin_errors)

    report["errors"] = errors
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(
        json.dumps(
            {
                "active_product": active_product,
                "summary": summary,
                "error_count": len(errors),
                "errors": errors,
            },
            indent=2,
        )
    )
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
