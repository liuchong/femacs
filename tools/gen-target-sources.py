#!/usr/bin/env python3
"""Freeze the `src/*.c` set a frozen target's configuration compiles.

`docs/baseline/frozen-build-sources.txt` is the darwin list, and its own header records the
recipe: a `make -n` in the frozen build's `src/` and a grep for the `.c` files it names.  That
works because the darwin build directory *is* a host make directory.

The Android configuration is not: `configure` moves the resulting makefiles to `*.android`
(cross/ndk-build drives them), so there is no `make -n` to run, and the makefile includes
`../ndk-build/ndk-build.mk`, which the outer configure only generates when the NDK build glue is
set up.  The source lists themselves are plain variables, though, so this tool expands them:
`base_obj`, `ANDROID_OBJ`, `otherobj` and `obj`, with `$(NAME)` references resolved recursively.
Only objects that live in `src/` become entries; a reference to another directory's objects
(`$(lib)/…`) expands to nothing.

    tools/gen-android-config.sh                       # writes /tmp/femacs-android-config
    python3 tools/gen-target-sources.py --build-dir /tmp/femacs-android-config/src \\
        --out docs/baseline/frozen-build-sources-android.txt
"""

from __future__ import annotations

import argparse
import pathlib
import re

REPO = pathlib.Path(__file__).resolve().parent.parent
ROOTS = ("base_obj", "ANDROID_OBJ", "otherobj", "obj")


def variables(makefile: pathlib.Path) -> dict[str, str]:
    text = makefile.read_text().replace("\\\n", " ")
    out: dict[str, str] = {}
    for line in text.splitlines():
        m = re.match(r"^([A-Za-z_][A-Za-z0-9_]*)\s*([+:?]?=)\s*(.*)$", line)
        if not m:
            continue
        name, op, value = m.groups()
        if op == "+=" and name in out:
            out[name] += " " + value
        else:
            out[name] = value
    return out


def expand(text: str, table: dict[str, str], depth: int = 0) -> str:
    """`$(NAME)` -> its value, recursively; another directory's objects -> nothing."""
    if depth > 12:
        return text

    def sub(m: re.Match[str]) -> str:
        inner = m.group(1)
        if "/" in inner or inner in ("srcdir", "top_srcdir", "top_builddir", "VPATH"):
            return ""
        return expand(table.get(inner, ""), table, depth + 1)

    return re.sub(r"\$\(([^)]+)\)", sub, text)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--build-dir", required=True, type=pathlib.Path,
                    help="the frozen target's build `src/` directory")
    ap.add_argument("--makefile", default="Makefile")
    ap.add_argument("--out", required=True, type=pathlib.Path)
    ap.add_argument("--title", default=None)
    args = ap.parse_args()

    makefile = args.build_dir / args.makefile
    if not makefile.is_file():
        raise SystemExit(f"{makefile} is missing: configure that target first (see the docstring)")
    table = variables(makefile)
    objects = expand(" ".join(f"$({root})" for root in ROOTS), table)
    names = sorted({o for o in objects.split() if o.endswith(".o")})
    # The list's shape is the darwin list's: `src/NAME.c`, because that is what `build-tu` takes.
    # Emitting bare names would have made `check-all-units.sh <list>` look for files called
    # `alloc.c` at the repository root (caught before the first per-target build, status 4.612).
    sources = ["src/" + o[:-2] + ".c" for o in names]
    present = [c for c in sources if (REPO / "emacs" / c).is_file()]
    absent = [c for c in sources if c not in present]
    if absent:
        # A name the reference's own `src/` does not have means the expansion went wrong; say so
        # rather than freezing a list with holes in it.
        raise SystemExit(f"expanded {len(absent)} name(s) that are not in emacs/src: {absent[:8]}")

    recipe = (f"# Regenerate: tools/gen-android-config.sh (or the target's own configure), then\n"
              f"#   python3 tools/gen-target-sources.py --build-dir <build>/src \\\n"
              f"#       --makefile {args.makefile} --out {args.out}\n"
              f"# The recipe differs from the darwin list's `make -n` because this target's build\n"
              f"# directory is not a host make directory (see the tool's docstring).\n")
    title = args.title or f"# Sources the frozen {args.makefile} configuration compiles."
    # The title is given without a trailing newline by callers; the file's first line must not run
    # into the recipe comment (caught on the first generated list, status 4.612).
    args.out.write_text(title.rstrip("\n") + "\n" + recipe + "\n".join(present) + "\n")
    print(f"{len(present)} source(s) -> {args.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
