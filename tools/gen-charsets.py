#!/usr/bin/env python3
"""Generate the charsets/charset-maps family with the reference's own recipes.

`admin/charsets/Makefile` builds every `etc/charsets/*.map` and the two
`lisp/international/*.el` translation tables with plain `mapconv`/`gawk`/`sed`
pipelines whose inputs are the reference's own data files.  The port has to produce the same
files (completion definition item 4: the generated artefacts are byte-identical), and the port's
build scaffolding is `xtask` plus the tools in this directory rather than the reference's
Makefiles - so this tool asks the *frozen reference build* what it would run and runs exactly
that, with only the **output paths** rewritten into the port's tree (the inputs stay the
reference's read-only data, which is what the recipes name).

    python3 tools/gen-charsets.py                # every .map plus the two translation tables
    python3 tools/gen-charsets.py 8859-2.map ... # named targets
    python3 tools/gen-charsets.py --list         # the target list only

Outputs go to `femacs/etc/charsets/` and `femacs/lisp/international/` and are compared with the
reference's own copies (which its build already wrote into `emacs/`) - the comparison is the
reading; a byte difference is printed with the first differing offset.
"""

from __future__ import annotations

import argparse
import pathlib
import shutil
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF_BUILD = REPO / "build/emacs-ref/admin/charsets"
REF_SRC = REPO / "emacs"
PORT = REPO / "femacs"


def targets() -> list[str]:
    """The `.map` and translation-table targets the reference's Makefile names."""
    makefile = REF_BUILD / "Makefile"
    if not makefile.is_file():
        raise SystemExit(f"{makefile} is missing: configure the frozen reference build first")
    out = subprocess.run(["make", "-s", "-p"], cwd=REF_BUILD, capture_output=True, text=True).stdout
    names: list[str] = []
    for line in out.split("\n"):
        if line.startswith("CHARSETS :=") or line.startswith("TRANS_TABLE :="):
            for tok in line.split("=", 1)[1].split():
                names.append(pathlib.Path(tok).name)
    return sorted(dict.fromkeys(names))


def recipes(targets_: list[str]) -> list[tuple[str, str]]:
    """(output, command) for every target, asked one target at a time.

    `make -B -n <target>` prints the recipe although the target exists, and its first line is the
    `GEN      <output>` progress line: that is make itself naming the file, which is more reliable
    than parsing the command (the `cp` recipes have no redirect at all, and a recipe written with
    `\\`-continuations is printed across lines - both cost a round of failures before this shape).
    """
    cmds: list[tuple[str, str]] = []
    for target in targets_:
        r = subprocess.run(["make", "-B", "-n", target], cwd=REF_BUILD,
                           capture_output=True, text=True)
        lines: list[str] = []
        for line in r.stdout.split("\n"):
            if lines and lines[-1].endswith("\\"):
                lines[-1] = lines[-1][:-1].rstrip() + " " + line.strip()
            else:
                lines.append(line.rstrip())
        out = None
        body: list[str] = []
        for line in lines:
            stripped = line.strip()
            if not stripped:
                continue
            if stripped.startswith("GEN "):
                out = stripped.split(None, 1)[1].strip()
                continue
            if out is None:
                continue
            if stripped.startswith("make") or "Nothing to be done" in stripped:
                continue
            body.append(stripped)
        if out and body:
            cmds.append((out, " && ".join(body)))
    return cmds


def port_output(out: str) -> str:
    """The port-side path for the reference-source path OUT (make's own `GEN` line names it)."""
    for prefix, dest in (("../../../../emacs/etc/charsets/", f"{PORT}/etc/charsets/"),
                         ("../../../../emacs/lisp/international/", f"{PORT}/lisp/international/")):
        if out.startswith(prefix):
            return dest + out[len(prefix):]
    return out


def rewrite(cmd: str) -> str:
    """Point the recipe's *output* at the port's tree and keep every input as it is.

    The recipes redirect into `${top_srcdir}/...`, which for the frozen build resolves to the
    reference's checkout; the port's build must write its own tree instead.  The inputs
    (`mapconv`, `glibc/`, `mapfiles/`, the `.awk` scripts) stay the reference's data.
    """
    out = cmd
    # No `pathlib` here: `str(PORT / "etc/charsets/")` drops the trailing slash and would glue
    # the file name to the directory (measured before the first run - the pre-flight below).
    for prefix, dest in (("../../../../emacs/etc/charsets/", f"{PORT}/etc/charsets/"),
                         ("../../../../emacs/lisp/international/", f"{PORT}/lisp/international/")):
        out = out.replace("> " + prefix, "> " + dest)
    return out


def output_of(cmd: str) -> str | None:
    """The path a recipe redirects into, or None."""
    if "> " not in cmd:
        return None
    return cmd.split("> ", 1)[1].split()[0]


def is_reference_helper(out: str) -> bool:
    """A recipe that writes a *helper* into the reference's own source tree.

    `admin/charsets/jisx2131-filter` is a generated sed script that the other recipes only ever
    *read* (as `SED_SCRIPT`), and it already exists in the reference checkout from its own build.
    Generating it is the reference build's business, not an artefact the port has to match, so
    this tool leaves it alone rather than writing into the reference tree.
    """
    return out.startswith("../../../../emacs/admin/")


def check(out_path: pathlib.Path) -> str:
    """Compare OUT_PATH with the reference's own copy of the same file."""
    try:
        rel = out_path.relative_to(PORT)
    except ValueError:
        return "outside the port's tree (skipped)"
    ref = REF_SRC / rel
    if not ref.is_file():
        return "no reference copy (skipped)"
    a, b = out_path.read_bytes(), ref.read_bytes()
    if a == b:
        return f"identical ({len(a)} bytes)"
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            return f"DIFFERS at byte {i}: port {x:#04x} reference {y:#04x}"
    return f"DIFFERS in length: port {len(a)} reference {len(b)}"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("targets", nargs="*", help="Makefile targets (default: all of them)")
    ap.add_argument("--list", action="store_true", help="print the target list and exit")
    args = ap.parse_args()

    all_targets = targets()
    if args.list:
        print("\n".join(all_targets))
        return 0
    chosen = args.targets or all_targets
    if not REF_BUILD.is_dir():
        raise SystemExit(f"{REF_BUILD} is missing: configure the frozen reference build first")

    kept, skipped = [], []
    for out, cmd in recipes(chosen):
        if out.startswith("../../../../emacs/admin/"):
            skipped.append(cmd)
            continue
        kept.append((out, cmd))
    if skipped:
        print(f"skipped {len(skipped)} recipe(s) that write into the reference tree, "
              f"e.g. {skipped[0][:90]}")
    cmds = kept
    print(f"{len(chosen)} target(s) asked for, {len(cmds)} recipe(s) to run")
    made: list[pathlib.Path] = []
    for out, cmd in cmds:
        # Replace the exact output path make named: that covers the `> path` redirects and the
        # `cp`-style recipes, which name their output without any redirect.
        line = cmd.replace(out, port_output(out))
        m = port_output(out)
        r = subprocess.run(["/bin/sh", "-c", line], cwd=REF_BUILD, capture_output=True, text=True)
        if r.returncode != 0:
            print(f"FAILED ({r.returncode}): {line[:120]}\n   {r.stderr.strip()[:200]}")
            continue
        if m:
            made.append(pathlib.Path(m))
    groups: dict[str, list[str]] = {}
    for path in made:
        verdict = check(path)
        kind = verdict.split(" ", 1)[0]
        groups.setdefault(kind, []).append(f"{path.name} [{verdict}]")
    total = sum(len(v) for v in groups.values())
    print(f"{total} file(s) generated")
    for kind, rows in sorted(groups.items(), key=lambda kv: -len(kv[1])):
        more = "" if len(rows) <= 4 else f" (+{len(rows) - 4} more)"
        print(f"  {len(rows):3d}  {kind}: {'; '.join(rows[:4])}{more}")
    failures = [k for k in groups if k.startswith("DIFFERS") or k == "FAILED"]
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
