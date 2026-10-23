"""The eol-conversion gate: decode the same bytes with both programs and compare the characters.

`decode_eol` (`src/coding.rs`, C coding.c 6759) is where the CRs of a DOS-style file are removed,
and a port transcription of it kept them: measured 2026-10-16, the port's loop advanced its byte
position by `1 + incr` instead of `incr` and never advanced the character position (C's `pos++`
was dropped when the `let` was written without `mut`), so `(decode-coding-string "a\\r\\nb\\r\\n"
'utf-8-dos)` came back with the *first* CR still in it, and `insert-file-contents` of a
GB2312 CRLF file kept both.

The reading is differential: the same bytes are decoded by the reference's program and by the
port's, and what is compared is the decoded text as a list of character codes - printing the
string itself would hide the bug, because `prin1` writes a bare CR and the terminal then
overwrites the line it is on (that is how the first probe of this defect mis-read two identical
answers).

    python3 tools/check-coding-eol.py                     # every case, both programs
    python3 tools/check-coding-eol.py --show-all          # print every case, not only failures
    python3 tools/check-coding-eol.py --port <binary> --reference <binary>

Cases cover the three places the conversion can happen: the coding driver while it decodes
(`insert-file-contents`, `decode-coding-region`), the shared tail of `decode_coding`, and the
string path (`decode-coding-string`), across the codings whose eol types are declared
(`utf-8-dos`, `iso-latin-1-dos`), detected (`undecided`) and unqualified (`utf-8`).
"""

from __future__ import annotations

import argparse
import os
import pathlib
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
PORT_PROGRAM = REPO / "build/port/src/temacs"
REFERENCE_PROGRAM = REPO / "build/emacs-ref/src/emacs"

# (label, file bytes or None, elisp expression producing the text to compare).  `{file}` is the
# temp file this case writes; expressions are run in a batch program with `print-escape-newlines`
# irrelevant (the comparison is by character code, never by printed text).
CASES: list[tuple[str, bytes | None, str]] = [
    ("ascii-crlf-file", b"a\r\nb\r\nc\r\n", "(with-temp-buffer (insert-file-contents \"{file}\") (buffer-string))"),
    ("ascii-lf-file", b"a\nb\nc\n", "(with-temp-buffer (insert-file-contents \"{file}\") (buffer-string))"),
    ("ascii-cr-only-file", b"a\rb\rc\r", "(with-temp-buffer (insert-file-contents \"{file}\") (buffer-string))"),
    ("ascii-crlf-forced-utf-8-dos", b"a\r\nb\r\nc\r\n",
     "(with-temp-buffer (let ((coding-system-for-read 'utf-8-dos)) (insert-file-contents \"{file}\") (buffer-string)))"),
    ("ascii-crlf-forced-utf-8-unix", b"a\r\nb\r\nc\r\n",
     "(with-temp-buffer (let ((coding-system-for-read 'utf-8-unix)) (insert-file-contents \"{file}\") (buffer-string)))"),
    # A GB2312 file with no coding cookie: the detection decides (iso-latin-1-dos on this machine),
    # and the charset driver plus `decode_eol` have to remove the CRs.
    ("gb2312-crlf-file", "阿啊呵\r\n爱哀挨\r\n".encode("gb2312"),
     "(with-temp-buffer (insert-file-contents \"{file}\") (buffer-string))"),
    ("gb2312-lf-file", "阿啊呵\n爱哀挨\n".encode("gb2312"),
     "(with-temp-buffer (insert-file-contents \"{file}\") (buffer-string))"),
    ("gb2312-forced-dos", "阿啊呵\r\n爱哀挨\r\n".encode("gb2312"),
     "(with-temp-buffer (let ((coding-system-for-read 'chinese-iso-8bit-dos)) (insert-file-contents \"{file}\") (buffer-string)))"),
    ("latin1-crlf-forced-dos", b"\xb0\xa2\xb0\xa1\r\n\xb0\xae\r\n",
     "(with-temp-buffer (let ((coding-system-for-read 'iso-latin-1-dos)) (insert-file-contents \"{file}\") (buffer-string)))"),
    ("string-utf-8-dos", None, "(decode-coding-string (string-make-unibyte \"a\\r\\nb\\r\\n\") 'utf-8-dos)"),
    ("string-utf-8", None, "(decode-coding-string (string-make-unibyte \"a\\r\\nb\\r\\n\") 'utf-8)"),
    ("string-undecided", None, "(decode-coding-string (string-make-unibyte \"a\\r\\nb\\r\\n\") 'undecided)"),
    ("string-latin-1-dos", None, "(decode-coding-string (string-make-unibyte \"a\\r\\nb\\r\\n\") 'iso-latin-1-dos)"),
    ("region-utf-8-dos", None, "(with-temp-buffer (insert \"a\\r\\nb\\r\\n\") (decode-coding-region (point-min) (point-max) 'utf-8-dos) (buffer-string))"),
    ("encode-utf-8-dos", None, "(encode-coding-string \"a\\nb\\n\" 'utf-8-dos)"),
    ("encode-utf-8-mac", None, "(encode-coding-string \"a\\nb\\n\" 'utf-8-mac)"),
]


def probe_script(files: dict[str, pathlib.Path]) -> str:
    """The elisp both programs run: each case prints `MARK <label> <chars>` (character codes)."""
    out = ["(setq print-escape-newlines t)", "(defun mark (label s) (princ (format \"MARK %s %S\\n\" label (append s nil))))"]
    for label, _, expression in CASES:
        expr = expression.format(file=str(files[label])) if "{file}" in expression else expression
        # The label is a *string*: a `'symbol` label would leave a second quote for the
        # reader macro to apply to the case's own form, which passes the form itself to
        # `mark` as data - measured 2026-10-16, every case then compared the same way.
        out.append(f'(mark "{label}" {expr})')
    return "\n".join(out) + "\n"


def run(program: pathlib.Path, script: pathlib.Path, load_path: pathlib.Path, locale: str) -> dict[str, str]:
    result = subprocess.run([str(program), "-Q", "--batch", "--eval", "(setq load-prefer-newer t)",
                             "-l", str(script)],
                            capture_output=True, text=True, timeout=600,
                            env={"EMACSLOADPATH": str(load_path), "PATH": os.environ.get("PATH", os.defpath),
                                 "LANG": locale, "LC_ALL": locale})
    marks: dict[str, str] = {}
    for line in result.stdout.splitlines():
        if line.startswith("MARK "):
            _, label, chars = line.split(" ", 2)
            marks[label] = chars.strip()
    if not marks:
        print(f"{program}: no readings (exit {result.returncode}): "
              f"{(result.stderr or result.stdout).strip().splitlines()[-1:]}")
    return marks


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--port", type=pathlib.Path, default=PORT_PROGRAM)
    parser.add_argument("--reference", type=pathlib.Path, default=REFERENCE_PROGRAM)
    parser.add_argument("--show-all", action="store_true", help="print every case, not only failures")
    parser.add_argument("--locale", default="en_US.UTF-8", help="same locale for both programs, including Cocoa startup")
    args = parser.parse_args()

    for path, what in ((args.port, "the port's program (xtask gate-fast, then gate-load)"),
                       (args.reference, "the reference's program")):
        if not path.is_file():
            raise SystemExit(f"{path} is missing: build {what}")

    evidence_root = REPO / "build/coding-eol"
    evidence_root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=evidence_root) as tmp:
        tmpdir = pathlib.Path(tmp)
        files: dict[str, pathlib.Path] = {}
        for label, data, _ in CASES:
            if data is None:
                continue
            path = tmpdir / f"{label}.txt"
            path.write_bytes(data)
            files[label] = path
        script = tmpdir / "probe.el"
        script.write_text(probe_script(files), encoding="utf-8")

        reference = run(args.reference, script, REPO / "emacs/lisp", args.locale)
        port = run(args.port, script, REPO / "femacs/lisp", args.locale)

    failures = 0
    for label, _, _ in CASES:
        want, got = reference.get(label), port.get(label)
        if want == got and want is not None:
            if args.show_all:
                print(f"ok        {label}: {got}")
            continue
        failures += 1
        print(f"DIFFERENT {label}:")
        print(f"    the reference {want}")
        print(f"    the port      {got}")
    print(f"{len(CASES)} case(s), {failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
