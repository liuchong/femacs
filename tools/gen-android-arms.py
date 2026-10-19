#!/usr/bin/env python3
"""Turn the Android import ratchet into a work queue, classified by who defines each name.

`tools/check-android-imports.py` answers "which names does the Android `libemacs.so` import that
nothing in the process provides" - 119 of them, one per line in
`docs/baseline/android-imports-pending.txt`.  That list is a set of names; the work to close it is
not per name but per *definition site*, and the first question about any name is who defines it:

  * **`port_arm`** - the frozen build's C defines it and a port unit defines it too, but not for
    this target: the unit is missing from the Android link set, or the configuration takes a
    different arm.  (`mpz_add` was this class before `lib/mini-gmp.c` joined the Android lib list,
    status 4.708.)
  * **`port_gap`** - the frozen build's C defines it and *no* port unit does: a transcription gap
    or a gate that excludes the definition on both targets (`strmode` was this class, 4.708).
  * **`arm`** - the C source defines it, the frozen build does not: a configuration we can name
    compiles it (the `HAVE_WINDOW_SYSTEM` arms, `memset_s`'s `#elif`, `tputs` in the tree's own
    termcap).  The row records the definition site and the conditions that select it.
  * **`system`** - no C source in the tree defines it: the platform's own interface (`__error` on
    the frozen target, `__errno` on bionic), i.e. a mirror/declaration question, not a port gap.
  * **`runtime`** - a Rust-mangled name or a double-underscore platform interface: neither a C
    region nor a unit answers it (`rust_begin_unwind` was this class, closed in 4.708).

`nm` on the objects answers this exactly, which is why the tool reads them rather than scanning
sources with a regular expression (the first version did the latter and reported 66 names as
"not found" that the reference objects define perfectly well - `char *BC;` puts the type before
the name, status 4.709).

    python3 tools/gen-android-arms.py                     # write docs/baseline/android-arms.tsv
    python3 tools/gen-android-arms.py --class port_gap     # one class only
    python3 tools/gen-android-arms.py --out -              # print instead of writing
"""

from __future__ import annotations

import argparse
import collections
import glob
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PENDING = REPO / "docs/baseline/android-imports-pending.txt"
OUT = REPO / "docs/baseline/android-arms.tsv"
REFERENCE = "build/emacs-ref"
PORT = "build/femacs"
DEFAULT_CLASS = "port_arm"



SOURCES = ("emacs/src/**/*.c", "emacs/lib/**/*.c", "emacs/lib/**/*.h", "emacs/lib-src/**/*.c",
           "emacs/src/**/*.h")


def source_definition(name: str) -> tuple[str, int, str]:
    """(file, line, enclosing `#if*` conditions) for a definition in the C tree, or ("",0,"")."""
    fn = re.compile(rf"^(?:[A-Za-z_][\w \t*]*\W)?{re.escape(name)}\s*\(")
    obj = re.compile(rf"^(?:[A-Za-z_][\w \t*]*\W)?{re.escape(name)}\s*(?:\[[^\]]*\])?\s*(?:=|;)")
    for pattern in SOURCES:
        for path in sorted(glob.glob(str(REPO / pattern), recursive=True)):
            lines = _read(path)
            for i, line in enumerate(lines):
                if line.startswith(("#", " ", "\t", "}", "/*", "*")):
                    continue
                if fn.match(line) or obj.match(line):
                    return str(pathlib.Path(path).relative_to(REPO)), i + 1, conditions(lines, i)
    return "", 0, ""


_cache: dict[str, list[str]] = {}


def _read(path: str) -> list[str]:
    if path not in _cache:
        _cache[path] = pathlib.Path(path).read_text(errors="replace").splitlines()
    return _cache[path]


# A preprocessor directive is not always at the left margin, and reading only the lines that are
# left the conditions column empty for whole regions of C that indent their `#if`s
# (`src/sysdep.c`'s `# if defined DARWIN_OS` chain).  A row whose condition string is empty reads
# as "unguarded" to whoever picks the row up, which is the opposite of what C says (status 4.749).
DIRECTIVE = re.compile(r"^\s*#\s*(if|ifdef|ifndef|elif|else|endif)\b")


def directive(line: str) -> str | None:
    """The preprocessor directive `line` carries, or None."""
    m = DIRECTIVE.match(line)
    return m.group(1) if m else None


def own_guard(lines: list[str]) -> int | None:
    """The index of the file's own include guard, or None.

    It is a condition like any other, but it is true for the whole file by construction, so it
    says nothing about whether an arm is live - and every row of a header would carry it.
    """
    for i, line in enumerate(lines[:80]):
        kind = directive(line)
        if kind in ("if", "ifdef"):
            break
        if kind == "ifndef":
            words = line.split()
            if len(words) > 1:
                name = words[1]
                if any(l.strip().lstrip("#").strip().startswith(f"define {name}")
                       for l in lines[i + 1:i + 25]):
                    return i
    return None


def conditions(lines: list[str], index: int) -> str:
    """The `#if*` conditions enclosing line `index`, outermost first."""
    guard = own_guard(lines)
    stack: list[str] = []
    for i, line in enumerate(lines[: index + 1]):
        kind = directive(line)
        if i == guard:
            continue
        if kind in ("if", "ifdef", "ifndef"):
            stack.append(line.strip())
        elif kind == "endif":
            if stack:
                stack.pop()
        elif kind in ("else", "elif") and stack:
            stack[-1] = line.strip()
    return " && ".join(stack)


def definitions(pattern: str, label: str) -> dict[str, set[str]]:
    """`name -> {object, …}` for the objects matching PATTERN (nm's global definitions)."""
    out: dict[str, set[str]] = {}
    files = sorted(glob.glob(str(REPO / pattern), recursive=True))
    for start in range(0, len(files), 40):
        text = subprocess.run(["nm", "-A", "-g", "--defined-only", *files[start:start + 40]],
                              capture_output=True, text=True).stdout
        for line in text.splitlines():
            fields = line.split()
            if len(fields) < 3:
                continue
            name = fields[-1].lstrip("_") if fields[-1].startswith("__") else fields[-1]
            out.setdefault(name, set()).add(label + ":" + pathlib.Path(fields[0]).name)
    return out


def classify(name: str, reference: dict[str, set[str]],
             port: dict[str, set[str]]) -> tuple[str, str, str]:
    """(class, where, conditions)."""
    if name.startswith("_R") or name.startswith("__"):
        return "runtime", (sorted(reference.get(name, ())) or [""])[0], ""
    if reference.get(name):
        where = sorted(reference[name])[0]
        return ("port_arm" if port.get(name) else "port_gap"), where, ""
    file, line, conds = source_definition(name)
    if file:
        # Some configuration of this C defines it; the frozen one does not, so it is an arm (with
        # the conditions that select it) rather than a name the platform provides.
        return "arm", f"{file}:{line}", conds
    return "system", "", ""


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pending", type=pathlib.Path, default=PENDING)
    ap.add_argument("--out", default=str(OUT))
    ap.add_argument("--class", dest="cls", default=None,
                    help="only this class: port_arm | port_gap | arm | system | runtime")
    a = ap.parse_args()

    names = [l.strip() for l in a.pending.read_text(encoding="utf-8").splitlines()
             if l.strip() and not l.startswith("#")]
    if not names:
        print("gen-android-arms: the ratchet is empty: nothing is missing")
        return 0

    reference = definitions(f"{REFERENCE}/**/*.o", "reference")
    for name, where in definitions(f"{REFERENCE}/**/*.a", "reference").items():
        reference.setdefault(name, set()).update(where)
    port = definitions(f"{PORT}/*.o", "port-darwin")

    rows = []
    for name in names:
        cls, where, conds = classify(name, reference, port)
        rows.append((name, cls, where, conds))
    if a.cls:
        rows = [r for r in rows if r[1] == a.cls]

    header = ("# The Android import ratchet as a work queue (tools/gen-android-arms.py).\n"
              "# The reading this queue drives to zero is `python3 tools/check-android-imports.py`\n"
              "# (docs/status.md 4.708).  Classes: port_arm (the port defines it, but not for this\n"
              "# target), port_gap (no port unit defines it), android_only (nothing in the frozen\n"
              "# build defines it), runtime (Rust/platform interface names), port_only (only the\n"
              "# Swift-facing shim defines it).\n"
              "#\n"
              "# name\tclass\tdefinition site\tconditions\n")
    text = header + "\n".join("\t".join(row) for row in rows) + "\n"
    if a.out == "-":
        sys.stdout.write(text)
    else:
        pathlib.Path(a.out).write_text(text, encoding="utf-8")
        print(f"wrote {pathlib.Path(a.out).name}: {len(rows)} row(s)")

    classes = collections.Counter(r[1] for r in rows)
    print("classes: " + ", ".join(f"{k} {v}" for k, v in classes.most_common()))
    objects = collections.Counter(r[2] for r in rows if r[2])
    if objects:
        print("top defining objects: " + ", ".join(f"{k} {v}" for k, v in objects.most_common(6)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
