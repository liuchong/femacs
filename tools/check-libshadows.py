#!/usr/bin/env python3
"""Audit the thirteen lib/ units whose gnulib modules the frozen build excludes.

Thirteen `lib/` files port to units that define a *system* function's own name (`open`,
`fcntl`, `getdelim`, ...).  C never links them: `temacs` links `libgnu.a`'s member set, and the
frozen configuration does not build those modules at all.  In the unified Cargo workspace (`U1`),
`crates/femacs-sys/src/lib.rs` mounts only the active `lib/` modules; if one of these thirteen
units were added to the workspace module graph, its global C symbol would shadow libSystem.

This gate checks both layers without invoking pre-Cargo per-unit object builders:
  * every one of the thirteen `lib/<unit>.rs` sources exists and exports at least one C symbol;
  * none of the thirteen units is mounted in `crates/femacs-{sys,vm,host}/src/lib.rs`;
  * when auditing a linked binary (defaulting to `build/terminal/port/src/emacs` or
    `build/port/src/temacs`, or explicit `--program`), none of the global C symbols those units
    export is defined (`T`) in the linked program.

    python3 tools/check-libshadows.py
    python3 tools/check-libshadows.py --program build/terminal/port/src/emacs
    python3 tools/check-libshadows.py --source-only
    python3 tools/check-libshadows.py --selftest
"""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent

CRATE_ROOTS = (
    REPO / "crates/femacs-sys/src/lib.rs",
    REPO / "crates/femacs-vm/src/lib.rs",
    REPO / "crates/femacs-host/src/lib.rs",
)

DEFAULT_PROGRAMS = (
    REPO / "build/terminal/port/src/emacs",
    REPO / "build/port/src/temacs",
)

UNITS = [
    "open",
    "fcntl",
    "getdelim",
    "getline",
    "getgroups",
    "mkostemp",
    "stpcpy",
    "strnlen",
    "strtoimax",
    "strtoll",
    "time_r",
    "fchmodat",
    "cloexec",
]

NO_MANGLE_RE = re.compile(
    r"#\[(?:cfg\s*\(\s*not\s*\([^)]*\)\s*\)\s*\]\s*#\[)?(?:unsafe\s*\(\s*)?no_mangle(?:\s*\))?\]"
    r"\s*(?:#\[[^\]]*\]\s*)*pub\s+(?:unsafe\s+)?extern\s+\"C(?:-unwind)?\"\s+fn\s+([A-Za-z0-9_]+)",
    re.S,
)
EXPORT_NAME_RE = re.compile(
    r"(?<!#\[cfg \(feature = \"unsigned\"\)\]\n)"
    r"#\[(?:unsafe\s*\(\s*)?export_name\s*=\s*\"([A-Za-z0-9_]+)\"(?:\s*\))?\]"
)
STRTOL_MACRO_RE = re.compile(r"strtol_body!\s*\(\s*[A-Za-z0-9_]+\s*,\s*\"([A-Za-z0-9_]+)\"")


def strip_rust_comments_and_strings(text: str) -> str:
    """Blank comments and string/char literals while preserving newlines."""
    out: list[str] = []
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if ch == "/" and nxt == "/":
            i += 2
            while i < n and text[i] != "\n":
                i += 1
            continue
        if ch == "/" and nxt == "*":
            depth = 1
            i += 2
            while i < n and depth > 0:
                if text[i] == "/" and i + 1 < n and text[i + 1] == "*":
                    depth += 1
                    i += 2
                elif text[i] == "*" and i + 1 < n and text[i + 1] == "/":
                    depth -= 1
                    i += 2
                else:
                    if text[i] == "\n":
                        out.append("\n")
                    i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def unit_exported_symbols(source_text: str) -> list[str]:
    """Extract active C linkage symbol names exported by a ported lib/ source."""
    stripped = strip_rust_comments_and_strings(source_text)
    # Drop blocks explicitly gated behind non-default `#[cfg (feature = "unsigned")]`.
    cleaned = re.sub(
        r"#\[cfg\s*\(\s*feature\s*=\s*\"unsigned\"\s*\)\]\s*"
        r"#\[(?:unsafe\s*\(\s*)?export_name\s*=\s*\"[A-Za-z0-9_]+\"(?:\s*\))?\]",
        "",
        stripped,
    )
    found: list[str] = []
    for sym in (
        NO_MANGLE_RE.findall(cleaned)
        + EXPORT_NAME_RE.findall(cleaned)
        + STRTOL_MACRO_RE.findall(cleaned)
    ):
        if sym not in found:
            found.append(sym)
    return found


def program_defined_global_text_symbols(binary: pathlib.Path) -> set[str]:
    """Return global text symbols (`T`) defined by `binary`, stripped of leading `_`."""
    proc = subprocess.run(
        ["nm", "-g", str(binary)],
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(f"nm -g failed on {binary}: {proc.stderr.strip()}")
    names: set[str] = set()
    for line in proc.stdout.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[1] == "T":
            names.add(parts[2].lstrip("_"))
    return names


def resolve_program(explicit: pathlib.Path | None) -> pathlib.Path | None:
    if explicit is not None:
        return explicit
    for candidate in DEFAULT_PROGRAMS:
        if candidate.is_file():
            return candidate
    return None


def audit(
    repo: pathlib.Path = REPO,
    program: pathlib.Path | None = None,
    source_only: bool = False,
) -> tuple[int, list[str]]:
    crate_texts: dict[str, str] = {}
    for root in CRATE_ROOTS:
        rel = root.relative_to(REPO)
        target = repo / rel
        if not target.is_file():
            return 1, [f"MISSING {rel}"]
        crate_texts[str(rel)] = target.read_text(encoding="utf-8")

    defined_in_program: set[str] = set()
    chosen_program: pathlib.Path | None = None
    if not source_only:
        chosen_program = resolve_program(program)
        if chosen_program is None or not chosen_program.is_file():
            wanted = program if program is not None else DEFAULT_PROGRAMS[0]
            return 1, [f"MISSING linked program {wanted}; build a target or pass --source-only"]
        defined_in_program = program_defined_global_text_symbols(chosen_program)

    failures = 0
    messages: list[str] = []
    shadowing: list[str] = []

    for unit in UNITS:
        rs_path = repo / "lib" / f"{unit}.rs"
        if not rs_path.is_file():
            messages.append(f"MISSING lib/{unit}.rs")
            failures += 1
            continue
        for rel, text in crate_texts.items():
            if f"lib/{unit}.rs" in text or f"pub mod u_lib_{unit.replace('-', '_')};" in text:
                messages.append(f"LINKED  lib/{unit}.rs is mounted in {rel}")
                failures += 1
        names = unit_exported_symbols(rs_path.read_text(encoding="utf-8"))
        if not names:
            messages.append(f"EMPTY   lib/{unit}.rs exported 0 C symbols")
            failures += 1
            continue
        if chosen_program is not None:
            for name in names:
                if name in defined_in_program:
                    shadowing.append(f"{unit}: {name}")
        messages.append(
            f"ok      lib/{unit}.rs: excluded from workspace crates, exports {len(names)} symbol(s): "
            + ", ".join(names)
        )

    if shadowing:
        for row in shadowing:
            messages.append(f"SHADOW  {row}: defined by an excluded lib/ unit *and* by {chosen_program}")
        failures += len(shadowing)
    messages.append(
        f"{len(UNITS)} unit(s), {failures} failure(s), {len(shadowing)} shadowing symbol(s)"
    )
    return (1 if failures else 0), messages


def run_selftest() -> int:
    # Ensure commented-out attributes (e.g. lib/mkostemp.rs citing gen_tempname) are ignored.
    comment_sample = (
        '/* defines `#[unsafe(export_name = "gen_tempname")]` there */\n'
        '#[unsafe(no_mangle)]\npub unsafe extern "C" fn mkostemp() -> i32 { 0 }\n'
    )
    if unit_exported_symbols(comment_sample) != ["mkostemp"]:
        sys.stderr.write(f"comment-stripping selftest failed: {unit_exported_symbols(comment_sample)}\n")
        return 1
    code, msgs = audit(REPO, source_only=True)
    if code != 0:
        sys.stderr.write("\n".join(msgs) + "\n")
        return 1
    with tempfile.TemporaryDirectory(prefix="libshadows-selftest-") as tmp:
        fake = pathlib.Path(tmp)
        for root in CRATE_ROOTS:
            rel = root.relative_to(REPO)
            dst = fake / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text(root.read_text(encoding="utf-8"), encoding="utf-8")
        (fake / "lib").mkdir(parents=True, exist_ok=True)
        for unit in UNITS:
            src = REPO / "lib" / f"{unit}.rs"
            (fake / "lib" / f"{unit}.rs").write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
        # Inject an accidental mount into femacs-sys and verify rejection.
        sys_lib = fake / "crates/femacs-sys/src/lib.rs"
        sys_lib.write_text(
            sys_lib.read_text(encoding="utf-8") + '\n#[path = "../../../lib/open.rs"] pub mod u_lib_open;\n',
            encoding="utf-8",
        )
        bad_code, bad_msgs = audit(fake, source_only=True)
        if bad_code == 0 or not any("LINKED  lib/open.rs" in m for m in bad_msgs):
            sys.stderr.write("selftest failed to reject mounted lib/open.rs\n")
            return 1
    print("check-libshadows selftest: ok")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--program", type=pathlib.Path, help="explicit linked binary to inspect with nm -g")
    parser.add_argument("--source-only", action="store_true", help="audit workspace crate exclusion and symbol extraction without nm")
    parser.add_argument("--no-build", action="store_true", help="accepted for CLI compatibility; Cargo gate never invokes build-tu")
    parser.add_argument("--selftest", action="store_true", help="run unit self-tests")
    args = parser.parse_args(argv)
    if args.selftest:
        return run_selftest()
    code, messages = audit(REPO, program=args.program, source_only=args.source_only)
    for line in messages:
        print(line)
    return code


if __name__ == "__main__":
    sys.exit(main())
