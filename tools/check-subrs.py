#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Compare the two programs' subr tables, as the running programs see them.

DoD-2 names the subr table; the symbol gate covers linkage names, and this covers the table's
*contents*: every subr the port registers must have the reference's name, arity and interactive
spec, and the reference must not have one the port lacks.  Comparing at runtime avoids parsing
the `subrs` section's relocated pointers.
"""
import pathlib
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
REF = REPO / "build/emacs-ref/src/emacs"
PORT = REPO / "build/port/src/emacs"

PROBE = r"""
(let (out)
  (mapatoms
   (lambda (s)
     (let ((f (ignore-errors (symbol-function s))))
       (when (and f (subrp f))
         (push (list (symbol-name s) (subr-arity f)
                     (let ((i (ignore-errors (interactive-form f))))
                       (and i (prin1-to-string i))))
               out)))))
  (dolist (r (sort out (lambda (a b) (string< (car a) (car b)))))
    (message "SUBR %s %S %S" (nth 0 r) (nth 1 r) (nth 2 r))))
"""


def table(program: pathlib.Path) -> dict[str, str]:
    with tempfile.NamedTemporaryFile("w", suffix=".el", delete=False) as fh:
        fh.write(PROBE)
        probe = fh.name
    p = subprocess.run([str(program), "--no-init-file", "--no-site-file", "--no-site-lisp",
                        "--batch", "-l", probe], cwd=str(REPO), capture_output=True, text=True,
                       env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin", "LC_ALL": "C"})
    pathlib.Path(probe).unlink(missing_ok=True)
    out = {}
    for line in (p.stderr or "").splitlines():
        if line.startswith("SUBR "):
            _, name, rest = line.split(" ", 2)
            out[name] = rest
    return out


def main() -> int:
    for prog in (REF, PORT):
        if not prog.is_file():
            print(f"missing program: {prog}", file=sys.stderr)
            return 2
    ref, port = table(REF), table(PORT)
    missing = sorted(set(ref) - set(port))
    extra = sorted(set(port) - set(ref))
    differs = sorted(n for n in set(ref) & set(port) if ref[n] != port[n])
    for n in missing[:40]:
        print(f"MISSING  {n}: the reference registers it, the port does not")
    for n in extra[:40]:
        print(f"EXTRA    {n}: the port registers it, the reference does not")
    for n in differs[:40]:
        print(f"DIFFERS  {n}: port {port[n]} vs reference {ref[n]}")
    print(f"{len(port)} subr(s) on the port, {len(ref)} on the reference; "
          f"{len(missing)} missing, {len(extra)} extra, {len(differs)} with a different arity/intspec")
    return 1 if (missing or extra or differs) else 0


if __name__ == "__main__":
    sys.exit(main())
