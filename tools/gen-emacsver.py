#!/usr/bin/env python3
"""Generate the port's two version-stamp files with the reference's own recipes.

The reference's build writes two small files that carry its version number into the copied
trees, and the port has to produce the same bytes (completion definition item 4: the generated
artefacts are byte-identical):

  * `doc/emacs/emacsver.texi`  - `${version}` substituted into `@set EMACSVER @version@`
                                (rule `doc-emacsver` in `doc/emacs/Makefile.in`).
  * `etc/refcards/emacsver.tex` - the major version substituted into
                                `\\def\\versionemacs{@majorversion@}` (rule `etc-emacsver` in the
                                top-level `Makefile.in`), where the major version is
                                `${version}` with everything from the first dot stripped.

Both used to be written by `configure`; the port has no configure, so this tool runs the two
`sed` scripts *the reference's own make rules run* over the port's own `.in` files, and compares
the result with the file the reference's own build produced.  Which version string goes in comes
from the reference's `AC_INIT` and must agree with the port's own `PACKAGE_VERSION`.

    python3 tools/gen-emacsver.py             # write the two files and compare with the reference
    python3 tools/gen-emacsver.py --check     # compare only, write nothing

Nothing under `emacs/` is ever written: the output path is asserted to be inside `femacs/`, and
the two recipes are read back from the reference's makefiles so a moved rule fails loudly instead
of silently generating the wrong bytes.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "emacs"
PORT = REPO / "femacs"

# The reference's two `configure`-generated version stamps, with the make rule that writes each
# one.  `sed` is the substitution that rule performs; `value` says which of the two derived
# strings (`version` or `major`) is substituted in; `line` is the rule's own line in the
# reference's makefile, read back as the provenance check.
RECIPES = (
    {
        "target": "doc/emacs/emacsver.texi",
        "source": "doc/emacs/emacsver.texi.in",
        "makefile": "doc/emacs/Makefile.in",
        "line": "sed 's/[@]version@/${version}/'",
        "sed": "s/[@]version@/{version}/",
        "value": "version",
    },
    {
        "target": "etc/refcards/emacsver.tex",
        "source": "etc/refcards/emacsver.tex.in",
        "makefile": "Makefile.in",
        "line": 'sed "s/[@]majorversion@/$${majorversion}/"',
        "sed": "s/[@]majorversion@/{version}/",
        "value": "major",
    },
)


def reference_version() -> str:
    """The version the reference's own `configure.ac` declares (`AC_INIT`)."""
    text = (REF / "configure.ac").read_text(encoding="utf-8", errors="replace")
    match = re.search(r"^AC_INIT\(\[[^\]]*\], \[([^\]]+)\]", text, re.MULTILINE)
    if not match:
        raise SystemExit("the reference's configure.ac has no AC_INIT version")
    return match.group(1)


def port_version() -> str:
    """The version the port's own `src/emacs.rs` declares (`PACKAGE_VERSION`)."""
    text = (PORT / "src/emacs.rs").read_text(encoding="utf-8", errors="replace")
    match = re.search(r'PACKAGE_VERSION: &CStr = c"([^"]+)"', text)
    if not match:
        raise SystemExit("the port's src/emacs.rs has no PACKAGE_VERSION")
    return match.group(1)


def first_difference(a: bytes, b: bytes) -> str:
    """Describe where two byte strings first differ (offset and the surrounding text)."""
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            lo = max(0, i - 40)
            return (f"first difference at offset {i}: "
                    f"received {a[lo:i + 40]!r} vs expected {b[lo:i + 40]!r}")
    return f"equal over the common prefix; lengths {len(a)} vs {len(b)}"


def render(recipe: dict, value: str) -> bytes:
    """Run the reference's `sed` over the port's input file and return the bytes it writes."""
    source = PORT / recipe["source"]
    if not source.is_file():
        raise SystemExit(f"{source} is missing: the copied tree is incomplete")
    result = subprocess.run(["sed", "-e", recipe["sed"].format(version=value), str(source)],
                            capture_output=True)
    if result.returncode != 0:
        raise SystemExit(f"{recipe['source']}: sed failed: {result.stderr.decode(errors='replace')}")
    return result.stdout


def check_provenance(recipe: dict) -> None:
    """The recipe must still be the one the reference's makefile carries."""
    makefile = REF / recipe["makefile"]
    if not makefile.is_file():
        raise SystemExit(f"{makefile} is missing: the reference checkout is incomplete")
    text = makefile.read_text(encoding="utf-8", errors="replace")
    if recipe["line"] not in text:
        raise SystemExit(f"{recipe['makefile']} no longer carries the rule {recipe['line']!r}: "
                         f"the reference's recipe moved")


def stamp(recipe: dict, values: dict[str, str], write: bool) -> int:
    """Write one version stamp into the port's tree and compare it with the reference's file."""
    target = PORT / recipe["target"]
    if REF in target.resolve().parents or target.resolve() == REF.resolve():
        raise SystemExit(f"{target} is inside the reference tree: refusing to write")
    if PORT not in target.resolve().parents:
        raise SystemExit(f"{target} is outside the port tree: refusing to write")

    received = render(recipe, values[recipe["value"]])
    if write:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(received)

    reference_file = REF / recipe["target"]
    if not reference_file.is_file():
        print(f"{recipe['target']}: written, but the reference has no such file to compare with")
        return 1
    expected = reference_file.read_bytes()
    if received == expected:
        print(f"{recipe['target']}: identical ({len(received)} bytes)")
        return 0
    print(f"{recipe['target']}: DIFFERENT")
    print(f"  {first_difference(received, expected)}")
    return 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true",
                        help="render and compare only; write nothing into the port tree")
    parser.add_argument("--list", action="store_true", help="print the two targets and exit")
    args = parser.parse_args()

    if args.list:
        for recipe in RECIPES:
            print(recipe["target"])
        return 0

    reference = reference_version()
    port = port_version()
    print(f"versions: the reference's AC_INIT says {reference}, the port's PACKAGE_VERSION says {port}")
    if reference != port:
        print(f"versions DISAGREE: {reference} vs {port}: the two trees stamped different numbers")
        return 1
    values = {"version": reference, "major": reference.split(".")[0]}
    print(f"substitutions: {values['version']} (texi) / {values['major']} (tex, major version)")

    failures = 0
    for recipe in RECIPES:
        check_provenance(recipe)
        failures += stamp(recipe, values, write=not args.check) != 0

    print(f"{len(RECIPES)} target(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
