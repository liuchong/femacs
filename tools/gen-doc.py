#!/usr/bin/env python3
"""Generate femacs/etc/DOC from the port's own docstring carriers (PORTING.md P-105).

The reference builds `etc/DOC` by running `make-docfile -d <srcdir> <obj>…`, and that
program reads the *source* file named after each object (`<srcdir>/<stem>.c`, `.m` for
Objective-C).  The port's sources are `.rs`/`.swift`, so the port scans its own carriers
with this tool instead, and keeps `lib-src/make-docfile` (a 1:1 port, byte-faithful on the
same inputs) out of the DOC path (P-105 ②).

Output format is C's, byte for byte:

    \037S<object>\n            section header, one per object, in the reference order
    \037F<lisp-name>\n<doc>    a DEFUN record (`F`) or a DEFVAR_* record (`V`)
    \037V<lisp-name>\n<doc>

`<doc>` is make-docfile's rendering of the docstring: spaces and newlines are held back
and flushed only when a non-blank byte is written (so trailing blanks vanish), and a DEFUN
whose docstring carries no `usage:` keyword gets the argument list appended as
`\n\n(fn ARG1 &optional ARG2)` (C's `write_c_args`; `MANY`/`UNEVALLED` functions carry the
usage form inside the docstring instead, C 1195-1200/1607-1622).

Carriers, per P-106: `.rs` keeps C's shape - the docstring is a literal argument of the
`DEFUN!`/`DEFVAR_*!` invocation (`doc: "…"`) - and `.swift` spells it as a literal field of
the subr/variable registration.  Neither carrier is consumed by the code; they exist so
this scanner can read C's text.

Usage
-----
    python3 tools/gen-doc.py                 # build in memory, compare with the reference
    python3 tools/gen-doc.py --write         # also write femacs/etc/DOC
    python3 tools/gen-doc.py --section alloc # one section only (diagnostics)
    python3 tools/gen-doc.py --swift-worklist # what P-106 still owes the Swift units
    python3 tools/gen-doc.py --selftest      # P-106 ③: a .swift record against its .m source
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_REFERENCE = REPO / "build/emacs-ref/etc/DOC"
DEFAULT_OUT = REPO / "femacs/etc/DOC"
SWIFT_DIR = REPO / "femacs/platform/macos"
REFERENCE_SRC = REPO / "emacs/src"

# The invocations that carry records.  C's scanner recognizes the prefixes (make-docfile.c
# 852-1231): `DEFVAR` sets the DEFVAR flag, `DEFUN` the DEFUN flag.
DEFUN_MACRO = "DEFUN"
DEFVAR_MACROS = ("DEFVAR_LISP", "DEFVAR_LISP_NOPRO", "DEFVAR_INT", "DEFVAR_BOOL",
                 "DEFVAR_KBOARD", "DEFVAR_PER_BUFFER")
KNOWN_MACROS = (DEFUN_MACRO,) + DEFVAR_MACROS

# Objective-C units of the macOS backend are Swift in the port (gen-port-map.py).
SWIFT_STEMS = {"nsfns", "nsfont", "nsimage", "nsmenu", "nsselect", "nsterm",
               "nsxwidget", "macfont"}

BLANK = " \t\n\r\v\f"


class CarrierMissing(Exception):
    """A record this tool cannot read yet (the P-106 Swift carriers)."""


# --------------------------------------------------------------------------- lexing

def skip_trivia(s: str, i: int) -> int:
    n = len(s)
    while i < n:
        c = s[i]
        if c in " \t\r\n":
            i += 1
        elif c == "/" and i + 1 < n and s[i + 1] == "/":
            j = s.find("\n", i)
            i = n if j < 0 else j + 1
        elif c == "/" and i + 1 < n and s[i + 1] == "*":
            j = s.find("*/", i + 2)
            i = n if j < 0 else j + 2
        else:
            break
    return i


def raw_string_start(s: str, i: int) -> bool:
    """True for `r"…"`/`r#"…"#`; `r#type` is a raw *identifier*, not a string."""
    if i >= len(s) or s[i] != "r":
        return False
    j = i + 1
    while j < len(s) and s[j] == "#":
        j += 1
    return j < len(s) and s[j] == '"'


def string_literal(s: str, i: int) -> tuple[str, int]:
    """Decode the string literal that starts at `i` (`"…"`, `c"…"` or `r#"…"#`)."""
    if s[i] == "c":
        i += 1
    if s[i] == "r":
        j = i + 1
        hashes = 0
        while j < len(s) and s[j] == "#":
            hashes += 1
            j += 1
        if j < len(s) and s[j] == '"':
            close = '"' + "#" * hashes
            end = s.find(close, j + 1)
            if end < 0:
                raise ValueError("unterminated raw string")
            return s[j + 1:end], end + len(close)
    if s[i] != '"':
        raise ValueError(f"not a string literal at {i}: {s[i:i+20]!r}")
    i += 1
    out: list[str] = []
    n = len(s)
    while i < n:
        c = s[i]
        if c == "\\":
            e = s[i + 1]
            if e == "n":
                out.append("\n")
            elif e == "t":
                out.append("\t")
            elif e == "r":
                out.append("\r")
            elif e == "0":
                out.append("\0")
            elif e == "x":
                out.append(chr(int(s[i + 2:i + 4], 16)))
            elif e == "u":
                close = s.index("}", i)
                out.append(chr(int(s[i + 3:close], 16)))
                i = close + 1
                continue
            elif e == "\n":  # Rust's line continuation
                i += 2
                while i < n and s[i] in " \t":
                    i += 1
                continue
            else:  # \\ \" \' …
                out.append(e)
            i += 2
            continue
        if c == '"':
            return "".join(out), i + 1
        out.append(c)
        i += 1
    raise ValueError("unterminated string")


def split_args(s: str, i: int) -> tuple[list[str], int]:
    """Split the parenthesized argument list at `s[i] == '('` at top-level commas."""
    assert s[i] == "(", s[i:i + 20]
    i += 1
    depth = 0
    start = i
    args: list[str] = []
    n = len(s)
    while i < n:
        c = s[i]
        if c in "\"'":
            _, i = string_literal(s, i) if c == '"' else (None, _skip_char_or_lifetime(s, i))
            continue
        if c == "c" and i + 1 < n and s[i + 1] == '"':
            _, i = string_literal(s, i)
            continue
        if raw_string_start(s, i):
            _, i = string_literal(s, i)
            continue
        if c == "/" and i + 1 < n and s[i + 1] in "/*":
            i = skip_trivia(s, i)
            continue
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                args.append(s[start:i])
                return args, i + 1
            depth -= 1
        elif c == "," and depth == 0:
            args.append(s[start:i])
            start = i + 1
        i += 1
    raise ValueError("unterminated argument list")


def _skip_char_or_lifetime(s: str, i: int) -> int:
    """Skip a Rust char literal (`'a'`, `'\\n'`) or a lifetime (`'a`)."""
    if s[i] != "'":
        return i + 1
    j = i + 1
    if j < len(s) and s[j] == "\\":
        j += 2
        while j < len(s) and s[j] != "'":
            j += 1
        return j + 1
    if j + 1 < len(s) and s[j + 1] == "'":
        return j + 2
    while j < len(s) and (s[j].isalnum() or s[j] == "_"):
        j += 1
    return j


def name_of(arg: str) -> str:
    """The argument's name, C's last identifier == Rust's binding before the `:`."""
    depth = 0
    for k, c in enumerate(arg):
        if c in "([{<":
            depth += 1
        elif c in ")]}>":
            depth -= 1
        elif c == ":" and depth == 0:
            head = arg[:k].strip()
            m = re.findall(r"[A-Za-z_][A-Za-z_0-9]*", head)
            return m[-1] if m else ""
    m = re.findall(r"[A-Za-z_][A-Za-z_0-9]*", arg)
    return m[-1] if m else ""


def int_or_none(text: str) -> int | None:
    """C's `fscanf ("%d")` over the DEFUN's arity field."""
    text = clean(text)
    if "MANY" in text or "UNEVALLED" in text:
        return None  # C maps both to -1 and appends no argument list
    m = re.search(r"\b(\d+)\b", text)
    return int(m.group(1)) if m else None


# --------------------------------------------------------------------- record scan

class Record:
    __slots__ = ("kind", "name", "doc", "minargs", "maxargs", "args", "where")

    def __init__(self, kind, name, doc, minargs=None, maxargs=None, args=None, where=""):
        self.kind = kind          # 'F' (DEFUN) or 'V' (DEFVAR_*)
        self.name = name
        self.doc = doc            # raw carrier text (C's literal content)
        self.minargs = minargs
        self.maxargs = maxargs    # int, or None for MANY/UNEVALLED
        self.args = args or []    # argument names, in C's order
        self.where = where

    def __repr__(self):
        return f"<{self.kind} {self.name} @{self.where}>"


def clean(arg: str) -> str:
    """An argument's text with C's trailing comment removed (`0,  // C 3612-3663`)."""
    return arg[skip_trivia(arg, 0):].strip()


def _literal_arg(text: str) -> str:
    text = text.strip()
    i = skip_trivia(text, 0)
    return string_literal(text, i)[0]


def _doc_arg(args: list[str], start: int) -> str:
    for a in args[start:]:
        t = clean(a)
        if t.startswith("doc:"):
            return _literal_arg(t[4:])
    for a in args[start:]:
        t = clean(a)
        if t.startswith('c"') or t.startswith('"') or raw_string_start(t, 0):
            return _literal_arg(t)
    raise ValueError("no docstring carrier among: " + " | ".join(a.strip()[:30] for a in args))


def interpret(macro: str, args: list[str], where: str) -> Record:
    if macro == DEFUN_MACRO:
        name = _literal_arg(args[0])
        minargs = int_or_none(args[3])
        maxargs = int_or_none(args[4])
        doc = None
        argnames: list[str] = []
        for a in args[6:]:
            t = clean(a)
            if t.startswith("doc:"):
                doc = _literal_arg(t[4:])
            elif t.startswith("attributes:"):
                continue
            elif t.startswith("("):
                inner, _ = split_args(t, 0)
                argnames = [name_of(x) for x in inner]
        if doc is None:
            raise ValueError("DEFUN without docstring: " + name)
        return Record("F", name, doc, minargs, maxargs, argnames, where)
    name = _literal_arg(args[0])
    doc = _doc_arg(args, 2)
    return Record("V", name, doc, where=where)


def scan_rust(path: Path) -> list[Record]:
    s = path.read_text(encoding="utf-8")
    n = len(s)
    i = 0
    out: list[Record] = []
    while i < n:
        c = s[i]
        if c in " \t\r\n":
            i += 1
            continue
        if c == "/" and i + 1 < n and s[i + 1] in "/*":
            i = skip_trivia(s, i)
            continue
        if c == '"':
            _, i = string_literal(s, i)
            continue
        if c == "'":
            i = _skip_char_or_lifetime(s, i)
            continue
        if (c == "c" and i + 1 < n and s[i + 1] == '"') or raw_string_start(s, i):
            _, i = string_literal(s, i)
            continue
        if c.isalpha() or c == "_":
            j = i
            while j < n and (s[j].isalnum() or s[j] == "_"):
                j += 1
            word = s[i:j]
            if word in KNOWN_MACROS:
                k = skip_trivia(s, j)
                if k < n and s[k] == "!":
                    k = skip_trivia(s, k + 1)
                    if k < n and s[k] == "(":
                        args, end = split_args(s, k)
                        line = s.count("\n", 0, i) + 1
                        out.append(interpret(word, args, f"{path.name}:{line}"))
                        i = end
                        continue
            i = j
            continue
        i += 1
    return out


# ------------------------------------------------------------------ C's rendering

def put(out: bytearray, pending: list[int], ch: str) -> None:
    """C's `put_char`: flush held-back newlines then spaces, then the character.

    A docstring's non-ASCII characters are UTF-8 in the port's literal, which is how
    they sit in the reference's bytes as well.
    """
    while pending[0] > 0:
        pending[0] -= 1
        out.append(0x0A)
    while pending[1] > 0:
        pending[1] -= 1
        out.append(0x20)
    out.extend(ch.encode("utf-8"))


def render_doc(rec: Record, warn: list[str]) -> bytes:
    text = rec.doc
    i = 0
    while i < len(text) and text[i] in BLANK:  # C skips leading blanks of a /* … */ doc
        i += 1
    text = text[i:]
    out = bytearray()
    pending = [0, 0]            # newlines, spaces
    keyword = "usage:" if rec.kind == "F" else None
    cur = 0
    saw_usage = False
    i = 0
    n = len(text)
    while i < n:
        ch = text[i]
        if ch == " ":
            pending[1] += 1
        elif ch == "\n":
            pending[0] += 1
            pending[1] = 0
        elif keyword and cur < len(keyword) and ch == keyword[cur] and (cur > 0 or pending[0] > 0):
            cur += 1
            if cur == len(keyword):
                saw_usage = True
                cur = 0
                pending[0] = 2      # canonicalise the blank before the usage string
                pending[1] = 0
                j = i + 1
                while j < n and text[j] in " \n":
                    j += 1
                if j >= n or text[j] != "(":
                    raise ValueError(f"{rec.where}: no '(' after usage: keyword")
                put(out, pending, "(")
                j += 1
                while j < n and text[j] not in " )":
                    j += 1
                if j >= n:
                    raise ValueError(f"{rec.where}: unexpected end of usage form")
                put(out, pending, "f")
                put(out, pending, "n")
                i = j
                continue
        else:
            if cur > 0:              # false alarm: replay the prefix we scanned
                for p in range(cur):
                    put(out, pending, keyword[p])
                cur = 0
            put(out, pending, ch)
        i += 1

    if rec.kind == "F":
        if rec.maxargs is None:      # MANY/UNEVALLED: the docstring carries the usage form
            if not saw_usage:
                warn.append(f"{rec.where}: missing usage for {rec.name}")
        elif not saw_usage:
            out += b"\n\n"
            out += render_args(rec)
    return bytes(out)


def render_args(rec: Record) -> bytes:
    """C's `write_c_args` over the port's argument names."""
    out = bytearray(b"(fn")
    minargs = 0 if rec.minargs is None else rec.minargs
    maxargs = rec.maxargs
    for name in rec.args:
        if not name or name == "void":
            continue
        out += b" "
        if minargs == 0 and maxargs > 0:
            out += b"&optional "
        minargs -= 1
        maxargs -= 1
        if name == "defalt":         # C's reserved-word demangle
            out += b"DEFAULT"
        else:
            out += name.upper().replace("_", "-").encode()
    out += b")"          # C 553: `putchar (')')`, after the identifier loop
    return bytes(out)


# ----------------------------------------------------------- sections and assembly

def section_stem(obj: str) -> str:
    """`alloc.o`/`androidselect.c` -> `alloc`/`androidselect` (the Makefile spells four of the
    machine objects with `.c`, which is why those section names carry it)."""
    return re.sub(r"\.(o|c)$", "", obj)


def port_source(stem: str) -> Path:
    if stem in SWIFT_STEMS:
        return SWIFT_DIR / f"{stem}.swift"
    for d in ("src", "lib", "lib-src", "lwlib", "oldXMenu"):
        p = REPO / "femacs" / d / f"{stem}.rs"
        if p.exists():
            return p
    return REPO / "femacs/src" / f"{stem}.rs"


def reference_sections(ref: bytes) -> list[str]:
    return [m.group(1).decode() for m in re.finditer(rb"\x1fS([^\n]*)\n", ref)]


def build(ref: bytes, only: str | None = None) -> tuple[bytes, list[str], list[str]]:
    """Return (bytes, warnings, sections with no carrier yet)."""
    out = bytearray()
    warn: list[str] = []
    pending: list[str] = []
    for obj in reference_sections(ref):
        if only and not obj.startswith(only):
            continue
        stem = section_stem(obj)
        src = port_source(stem)
        out += b"\x1fS" + obj.encode() + b"\n"
        if not src.exists():
            pending.append(f"{obj}: no port source ({src.name})")
            continue
        if src.suffix == ".swift":
            recs, missing = scan_swift(src)
            pending.extend(f"{obj}: {m}" for m in missing)
        else:
            recs = scan_rust(src)
        for r in recs:
            out += b"\x1f" + r.kind.encode() + r.name.encode() + b"\n"
            out += render_doc(r, warn)
    return bytes(out), warn, pending


def scan_swift(path: Path) -> tuple[list[Record], list[str]]:
    """Read the port's Swift carriers (P-106); report the ones still missing."""
    s = path.read_text(encoding="utf-8")
    recs: list[Record] = []
    missing: list[str] = []
    for m in re.finditer(r"fileprivate var (S[A-Za-z_0-9]+)\s*:\s*Aligned_Lisp_Subr", s):
        line = s.count("\n", 0, m.start()) + 1
        tail = s[m.end():s.find("\n\n", m.end()) if s.find("\n\n", m.end()) > 0 else m.end() + 800]
        missing.append(f"{path.name}:{line} {m.group(1)}: docstring carrier not landed (P-106)")
    for m in re.finditer(r"\b(defvar_lisp|defvar_bool|defvar_int|defvar_kboard)\s*\(([^;]*?)\)\s*\n", s, re.S):
        line = s.count("\n", 0, m.start()) + 1
        missing.append(f"{path.name}:{line} {m.group(1)}: docstring carrier not landed (P-106)")
    return recs, missing



# ------------------------------------------------------------------- the report

def split_doc(data: bytes) -> list[tuple[str, list[tuple[str, str, bytes]]]]:
    """Parse a DOC into [(object, [(kind, name, doc), …]), …]."""
    out: list[tuple[str, list[tuple[str, str, bytes]]]] = []
    marks = list(re.finditer(rb"\x1f([SVF])([^\n]*)\n", data))
    for i, m in enumerate(marks):
        if m.group(1) == b"S":
            out.append((m.group(2).decode(), []))
            continue
        if not out:
            continue
        end = marks[i + 1].start() if i + 1 < len(marks) else len(data)
        out[-1][1].append((m.group(1).decode(), m.group(2).decode(), data[m.end():end]))
    return out


def report(ref: bytes, built: bytes, limit: int) -> None:
    want = split_doc(ref)
    got = split_doc(built)
    gained = {obj: recs for obj, recs in got}
    for obj, wrecs in want:
        grecs = gained.get(obj, [])
        wnames = [n for _, n, _ in wrecs]
        gnames = [n for _, n, _ in grecs]
        if wnames == gnames:
            continue
        print(f"\n{obj}: {len(wrecs)} record(s) expected, {len(grecs)} produced")
        missing = [n for n in wnames if n not in gnames]
        extra = [n for n in gnames if n not in wnames]
        if missing:
            print(f"  missing ({len(missing)}): {', '.join(missing[:8])}"
                  + (" …" if len(missing) > 8 else ""))
        if extra:
            print(f"  extra   ({len(extra)}): {', '.join(extra[:8])}"
                  + (" …" if len(extra) > 8 else ""))
        if not missing and not extra:
            for k, (w, g) in enumerate(zip(wnames, gnames)):
                if w != g:
                    print(f"  order differs at record {k}: expected {w}, produced {g}")
                    break
            continue
        for k, (w, g) in enumerate(zip(wnames, gnames)):
            if w != g:
                print(f"  first divergence at record {k}: expected {w}, produced {g}")
                break


def report_text(ref: bytes, built: bytes, limit: int) -> None:
    """Docstrings that both sides carry in the same section but that differ in bytes.

    Keyed by (section, name): several platform files define the same subr name
    (`x-change-window-property` in both `xfns.c` and `w32fns.c`), and the reference DOC
    carries one record per section, so a name-only key compares the wrong pair.
    """
    want = {(o, n): d for o, recs in split_doc(ref) for _, n, d in recs}
    got = {(o, n): d for o, recs in split_doc(built) for _, n, d in recs}
    bad = [(f"{o} {n}", want[k], got[k]) for k in want if k in got and want[k] != got[k]
           for o, n in (k,)]
    print(f"\nrecord text differing: {len(bad)}")
    for n, w, g in bad[:limit]:
        j = next((k for k in range(min(len(w), len(g))) if w[k] != g[k]), min(len(w), len(g)))
        print(f"  {n}: {len(w)} vs {len(g)} bytes, first difference at {j}")
        print(f"    reference {w[max(0, j-30):j+30]!r}")
        print(f"    port      {g[max(0, j-30):j+30]!r}")
    if len(bad) > limit:
        print(f"  … {len(bad) - limit} more")


# --------------------------------------------------------------- gap worklist

COMMENT_RE = re.compile(r"//[^\n]*|/\*.*?\*/", re.S)


def comment_spans(text: str) -> list[tuple[int, int]]:
    return [m.span() for m in COMMENT_RE.finditer(text)]


def in_spans(pos: int, spans: list[tuple[int, int]]) -> bool:
    for a, b in spans:
        if a <= pos < b:
            return True
        if a > pos:
            break
    return False


def occurrence(text: str, spans: list[tuple[int, int]], name: str) -> tuple[int, bool] | None:
    """First place `name` appears as a quoted literal, and whether it sits in a comment."""
    needle = '"' + name + '"'
    i = text.find(needle)
    if i < 0:
        i = text.find(name)
        if i < 0:
            return None
    line = text.count("\n", 0, i) + 1
    return line, in_spans(i, spans)


def c_origin(stem: str, name: str) -> tuple[Path | None, int, int]:
    """Where the reference defines this name: (file, line, offset), or (None, 0, 0)."""
    for cand in (REFERENCE_SRC / f"{stem}.c", REFERENCE_SRC / f"{stem}.m"):
        if cand.exists():
            t = cand.read_text(encoding="utf-8", errors="replace")
            m = re.search(r"\b(DEFUN|DEFVAR_[A-Z_0-9]+)\s*\(\s*\"" + re.escape(name) + r'\"', t)
            if m:
                return cand, t.count("\n", 0, m.start()) + 1, m.start()
    return None, 0, 0


# ---------------------------------------------------- frozen-configuration conditions

MACRO_TABLE = REPO / "docs/baseline/config/darwin-arm64.macros.tsv"
FROZEN_SOURCES = REPO / "docs/baseline/frozen-build-sources.txt"

_COND_CACHE: dict[Path, list[bool | None]] = {}


def frozen_macros() -> dict[str, str | None]:
    out: dict[str, str | None] = {}
    if not MACRO_TABLE.exists():
        return out
    for line in MACRO_TABLE.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 2:
            out[parts[0]] = None if parts[1] == "undef" else parts[1]
    return out


def frozen_source_files() -> set[str]:
    if not FROZEN_SOURCES.exists():
        return set()
    return {l.strip().rsplit("/", 1)[-1] for l in FROZEN_SOURCES.read_text().splitlines() if l.strip()}


def _eval_cond(expr: str, macros: dict[str, str | None]) -> bool | None:
    e = expr.strip()
    if e in ("0", "false", "FALSE"):
        return False
    if e in ("1", "true", "TRUE"):
        return True
    m = re.fullmatch(r"defined\s*\(?\s*([A-Za-z_][A-Za-z_0-9]*)\s*\)?", e)
    if m:
        return macros.get(m.group(1)) is not None
    if re.fullmatch(r"[A-Za-z_][A-Za-z_0-9]*", e):
        v = macros.get(e)
        if v is None:
            return False if e in macros else None
        try:
            return bool(int(v, 0))
        except ValueError:
            return True
    py = re.sub(r"\bdefined\s*\(?\s*([A-Za-z_][A-Za-z_0-9]*)\s*\)?",
                lambda mm: "1" if macros.get(mm.group(1)) is not None else "0", e)
    py = re.sub(r"\b([A-Za-z_][A-Za-z_0-9]*)\b", lambda mm: str(macros.get(mm.group(1)) or 0)
                if mm.group(1) not in ("not", "is", "None", "and", "or") else mm.group(0), py)
    py = py.replace("&&", " and ").replace("||", " or ")
    py = re.sub(r"!(?!=)", " not ", py)
    if re.search(r"[^0-9A-Za-z_ \(\)\[\].+\-*/%<>=!&|^~ ]", py):
        return None
    try:
        return bool(eval(py, {"__builtins__": {}}, {}))     # noqa: S307 - sanitised above
    except Exception:
        return None


def line_conditions(path: Path) -> list[bool | None]:
    """For every line of a C file, whether the frozen configuration compiles it."""
    if path in _COND_CACHE:
        return _COND_CACHE[path]
    macros = frozen_macros()
    text = path.read_text(encoding="utf-8", errors="replace")
    out: list[bool | None] = []
    stack: list[bool | None] = []
    taken: list[bool] = []
    # `split("\n")`, not `splitlines()`: these C sources carry form feeds (^L), which
    # `splitlines` would turn into extra line breaks and shift every line number.
    for line in text.split("\n"):
        # `#if 0 /* TODO … */` is still `#if 0`, and the comment need not close on this line.
        st = re.sub(r"/\*.*?\*/", " ", line)
        cut = st.find("/*")
        if cut >= 0:
            st = st[:cut]
        st = st.strip()
        cond = all(s is True for s in stack)
        out.append(cond if all(s is not None for s in stack) else (False if any(s is False for s in stack) else None))
        if st.startswith("#ifdef "):
            stack.append(macros.get(st.split()[1].strip()) is not None)
            taken.append(stack[-1] is True)
        elif st.startswith("#ifndef "):
            stack.append(macros.get(st.split()[1].strip()) is None)
            taken.append(stack[-1] is True)
        elif st.startswith("#if "):
            stack.append(_eval_cond(st[4:], macros))
            taken.append(stack[-1] is True)
        elif st.startswith("#elif ") and stack:
            stack[-1] = False if taken[-1] else _eval_cond(st[6:], macros)
            taken[-1] = taken[-1] or stack[-1] is True
        elif st.startswith("#else") and stack:
            stack[-1] = False if taken[-1] else True
            taken[-1] = True
        elif st.startswith("#endif") and stack:
            stack.pop()
            if taken:
                taken.pop()
    _COND_CACHE[path] = out
    return out


def classify(ref: bytes, built: bytes) -> list[tuple[str, ...]]:
    """Every missing record, classified against the frozen configuration."""
    rows = worklist(ref, built)
    frozen = frozen_source_files()
    out: list[tuple[str, ...]] = []
    for sec, kind, name, cref, port, status in rows:
        stem = sec[:-2] if sec.endswith(".o") else sec
        cfile, cline, _ = c_origin(stem, name)
        if status == "swift-carrier-pending":
            verdict = "swift-pending"
        elif cfile is None:
            verdict = "c-origin-unknown"
        elif frozen and cfile.name not in frozen:
            verdict = "dead-file"
        else:
            conds = line_conditions(cfile)
            v = conds[cline - 1] if 0 < cline <= len(conds) else None
            verdict = "live-missing" if v is True else ("dead-region" if v is False else "condition-unknown")
        out.append((sec, kind, name, f"{cfile.name}:{cline}" if cfile else "", port, verdict))
    return out





def carrier_site(text: str, name: str) -> tuple[int, bool] | None:
    """Where the port carries a record for `name`, and whether it is only a comment.

    None means there is no carrier at all; the name may still appear as a symbol
    registration (`DEFSYM!`) or in prose.
    """
    macro = re.compile(r"\b(DEFUN|DEFVAR_[A-Z_0-9]+)!\s*\(\s*(?:doc:\s*)?c?\"" + re.escape(name) + r'\"')
    m = macro.search(text)
    if not m:
        return None
    return text.count("\n", 0, m.start()) + 1, in_spans(m.start(), comment_spans(text))


def worklist(ref: bytes, built: bytes) -> list[tuple[str, ...]]:
    """Every record the reference has and the port does not, classified."""
    want = split_doc(ref)
    got = {obj: recs for obj, recs in split_doc(built)}
    rows: list[tuple[str, ...]] = []
    for obj, wrecs in want:
        produced = {n for _, n, _ in got.get(obj, [])}
        have = {n for _, n, _ in got.get(obj, [])}
        produced_names = have
        stem = section_stem(obj)
        src = port_source(stem)
        if obj.startswith("ns") or obj.startswith("macfont"):
            pass
        text = src.read_text(encoding="utf-8", errors="replace") if src.exists() else ""
        spans = comment_spans(text) if text else []
        for kind, name, _doc in wrecs:
            if name in produced_names:
                continue
            where = f"{src.name}:{src.parent.name}" if src.exists() else "(no unit)"
            if src.suffix == ".swift":
                status = "swift-carrier-pending"     # P-106
            else:
                hit = carrier_site(text, name) if text else None
                if hit is None:
                    status = "no-carrier"
                    seen = occurrence(text, spans, name) if text else None
                    if seen:
                        where = f"{src.name}:{seen[0]} (name only)"
                elif hit[1]:
                    status = "carrier-in-comment"
                    where = f"{src.name}:{hit[0]}"
                else:
                    status = "carrier-unscanned"   # scanner bug: carrier is live code
                    where = f"{src.name}:{hit[0]}"
            rows.append((obj, kind, name, c_origin(stem, name), where, status))
    return rows



def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--reference", type=Path, default=DEFAULT_REFERENCE)
    ap.add_argument("--out", type=Path, default=DEFAULT_OUT)
    ap.add_argument("--write", action="store_true", help="write the generated DOC")
    ap.add_argument("--section", help="only this object (prefix match), for diagnostics")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--strict", action="store_true",
                    help="require byte-identity with the reference DOC (the stretch reading)")
    ap.add_argument("--swift-worklist", action="store_true")
    ap.add_argument("--classify", action="store_true",
                    help="missing records judged against the frozen configuration")
    ap.add_argument("--worklist", action="store_true",
                    help="TSV of every reference record the port is missing, classified")
    ap.add_argument("--limit", type=int, default=40, help="max lines per report section")
    a = ap.parse_args()

    if not a.reference.exists():
        print(f"reference DOC not found: {a.reference}", file=sys.stderr)
        return 2
    ref = a.reference.read_bytes()
    built, warn, pending = build(ref, a.section)

    if a.write:
        a.out.parent.mkdir(parents=True, exist_ok=True)
        a.out.write_bytes(built)

    if a.section:
        print(f"section {a.section}: {len(built)} bytes built, {len(warn)} warning(s)")
    label = "written" if a.write else "built in memory"
    print(f"reference {a.reference}: {len(ref)} bytes, "
          f"{len(reference_sections(ref))} sections")
    print(f"port      {label}: {len(built)} bytes")
    identical = built == ref
    print(f"byte-identical: {'yes' if identical else 'no'}")
    if not identical:
        j = next((k for k in range(min(len(built), len(ref))) if built[k] != ref[k]),
                 min(len(built), len(ref)))
        print(f"first difference at byte {j}:")
        print(f"  reference {ref[max(0, j-40):j+40]!r}")
        print(f"  port      {built[max(0, j-40):j+40]!r}")
    if not identical and not a.worklist:
        report(ref, built, a.limit)
        report_text(ref, built, 6)
    if a.classify:
        rows = classify(ref, built)
        print("\t".join(("section", "kind", "name", "reference", "port", "verdict")))
        for r in rows:
            print("\t".join(r))
        tally: dict[str, int] = {}
        for r in rows:
            tally[r[5]] = tally.get(r[5], 0) + 1
        print("# " + ", ".join(f"{k}={v}" for k, v in sorted(tally.items())), file=sys.stderr)
        if a.worklist:
            return 0
    if a.worklist:
        rows = worklist(ref, built)
        print("\t".join(("section", "kind", "name", "reference", "port", "status")))
        for r in rows:
            print("\t".join(r))
        tally: dict[str, int] = {}
        for r in rows:
            tally[r[5]] = tally.get(r[5], 0) + 1
        print("# missing records: " + ", ".join(f"{k}={v}" for k, v in sorted(tally.items())),
              file=sys.stderr)
    if pending:
        print(f"\npending carriers ({len(pending)}), P-106:")
        for p in pending[:a.limit]:
            print(f"  {p}")
        if len(pending) > a.limit:
            print(f"  … {len(pending) - a.limit} more")
    if warn:
        print(f"\nwarnings ({len(warn)}):")
        for w in warn[:a.limit]:
            print(f"  {w}")

    if a.selftest:
        return 1 if selftest(ref, built) else 0
    if a.swift_worklist:
        for p in pending:
            print(p)
    if a.strict:
        return 0 if identical else 1
    verdicts = classify(ref, built)
    bad = [v for v in verdicts if v[5] in ("live-missing", "condition-unknown", "c-origin-unknown")]
    tally: dict[str, int] = {}
    for v in verdicts:
        tally[v[5]] = tally.get(v[5], 0) + 1
    print("\nDOC gate: " + ", ".join(f"{k}={v}" for k, v in sorted(tally.items())))
    for v in bad[:a.limit]:
        print(f"  DEFECT {v[5]}: {v[0]} {v[2]} ({v[3]})")
    return 1 if bad else 0


def doc_records(data: bytes) -> dict[tuple[str, str], bytes]:
    """{(section, name): rendered record bytes}, for the whole file."""
    out: dict[tuple[str, str], bytes] = {}
    for obj, recs in split_doc(data):
        for kind, name, doc in recs:
            out[(obj, name)] = b"\x1f" + kind.encode() + name.encode() + b"\n" + doc
    return out


def selftest(ref: bytes, built: bytes) -> int:
    """A record of each carrier kind, byte-compared with the reference DOC.

    P-106 ③ asks for a `.swift` record against its `.m` source; while the Swift carriers are
    pending (P-106) the case is reported as pending, and the `.rs` case stands in - it is the
    same pipeline (scan the carrier, render C's bytes), so it exercises the same code.
    """
    print("\nselftest:")
    refr = doc_records(ref)
    gotr = doc_records(built)
    failures = 0
    cases: list[tuple[str, str, str]] = []
    for obj, name in (("alloc.o", "make-string"), ("bytecode.o", "byte-code"),
                      ("keyboard.o", "keyboard-translate"), ("nsfns.o", "x-create-frame")):
        if (obj, name) not in refr:
            continue
        kind = ".swift" if obj.startswith("ns") else ".rs"
        cases.append((obj, name, kind))
    for obj, name, kind in cases:
        want = refr.get((obj, name))
        got = gotr.get((obj, name))
        if got is None:
            note = "carrier pending (P-106)" if kind == ".swift" else "MISSING"
            print(f"  [{kind}] {obj} {name}: {note}")
            if kind != ".swift":
                failures += 1
            continue
        ok = want == got
        print(f"  [{kind}] {obj} {name}: {'byte-identical' if ok else 'DIFFERS'} "
              f"({len(got)} bytes)")
        if not ok and want is not None:
            j = next((k for k in range(min(len(want), len(got))) if want[k] != got[k]), -1)
            print(f"      first difference at {j}: {got[max(0, j-20):j+20]!r}")
            failures += 1
    if not cases:
        print("  no case found")
    return failures


if __name__ == "__main__":
    sys.exit(main())
