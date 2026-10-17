#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Do the port's `DEFVAR_LISP`/`BOOL`/`INT` variables hold the reference's values?

P-178 found one variable (`ctags-program-name`) whose C initialiser line had been dropped: the
port still *declared* it, so nothing failed until a consumer read it (`org-ctags`), and only the
expensive-test selector noticed.  This gate looks for the whole class directly: it collects every
variable that C's `syms_of_*` initialises and then asks **both programs** for its default value.

The comparison is on values, not source, so the port's macro forms (`DEFVAR_PER_BUFFER!` and the
like call their own initialisation) never appear as noise.  Five differences are expected and
listed below with their reason; anything else is a defect.
"""
import io
import pathlib
import re
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
REF_PROGRAM = REPO / "build/emacs-ref/src/emacs"
PORT_PROGRAM = REPO / "build/port/src/emacs"

# name -> why this program's value may differ from the reference's.
EXPECTED = {
    "after-init-time": "a timestamp of this very run",
    "before-init-time": "a timestamp of this very run",
    "pdumper-fingerprint": "the dump's own fingerprint",
    "report-emacs-bug-address": "this program's own bug address (PORTING.md 0.1-19)",
    "system-configuration-options": "the configure line this build was made with",
}


def syms_of_bodies(ctext: str) -> str:
    """Every `syms_of_*` function body, concatenated."""
    out = []
    for m in re.finditer(r"\nsyms_of_\w+\s*\([^)]*\)\s*\n\{", ctext):
        start = m.end() - 1
        depth, i = 0, start
        while i < len(ctext):
            if ctext[i] == "{":
                depth += 1
            elif ctext[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        out.append(ctext[start:i])
    return "\n".join(out)


def candidates() -> list[str]:
    """The Lisp names C initialises in `syms_of_*` but the port never assigns in its own file."""
    names = set()
    for rs in sorted((REPO / "femacs/src").glob("*.rs")):
        c = REPO / "emacs/src" / (rs.stem + ".c")
        if not c.is_file():
            continue
        rtext = rs.read_text(encoding="utf-8", errors="replace")
        ctext = c.read_text(encoding="utf-8", errors="replace")
        bodies = syms_of_bodies(ctext)
        for m in re.finditer(r"DEFVAR_(LISP|BOOL|INT)\s*\(\s*\"([^\"]+)\"\s*,\s*(V\w+)", ctext):
            lisp_name, name = m.group(2), m.group(3)
            if not re.search(r"\b" + name + r"\s*=(?!=)", bodies):
                continue
            declared = re.search(r"pub static mut " + name + r"\b", rtext) is not None
            assigned = len(re.findall(r"\b" + name + r"\s*=(?!=)", rtext))
            if not declared or assigned <= 1:
                names.add(lisp_name)
    return sorted(names)


def values(program: pathlib.Path, names: list[str]) -> dict[str, str]:
    body = "\n".join(
        f"(message \"SYM %s %S\" '{n} (condition-case e (default-value '{n}) (error (list :err (car e)))))"
        for n in names)
    with tempfile.NamedTemporaryFile("w", suffix=".el", delete=False) as fh:
        fh.write(body + "\n")
        probe = fh.name
    p = subprocess.run([str(program), "--no-init-file", "--no-site-file", "--no-site-lisp",
                        "--batch", "-l", probe], cwd=str(REPO), capture_output=True, text=True,
                       env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin", "LC_ALL": "C"})
    out = {}
    for line in (p.stderr or "").splitlines():
        if line.startswith("SYM "):
            parts = line.split(" ", 2)
            if len(parts) == 3:
                out[parts[1]] = parts[2]
    pathlib.Path(probe).unlink(missing_ok=True)
    return out


def main() -> int:
    for prog in (REF_PROGRAM, PORT_PROGRAM):
        if not prog.is_file():
            print(f"missing program: {prog}", file=sys.stderr)
            return 2
    names = candidates()
    ref = values(REF_PROGRAM, names)
    port = values(PORT_PROGRAM, names)
    allowed, defects = 0, []
    for n in names:
        a, b = port.get(n), ref.get(n)
        if a == b:
            continue
        if n in EXPECTED:
            allowed += 1
            print(f"expected  {n}: {EXPECTED[n]}")
        else:
            defects.append(n)
            print(f"DEFECT    {n}: the port has {a} where the reference has {b}")
    print(f"{len(names)} variable(s) compared, {allowed} expected difference(s), "
          f"{len(defects)} defect(s)")
    return 1 if defects else 0


if __name__ == "__main__":
    sys.exit(main())
