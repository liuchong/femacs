#!/usr/bin/env python3
"""Shared C-source extractor for the femacs tooling.

Every tool that needs to read the reference C sources uses this module, so the
C side and the Rust side are always extracted by the same rules and can be
compared mechanically.

What it gets right on purpose (each was a real defect before):

* comments are stripped, so `Copyright (C)` and docstring prose are never
  mistaken for code;
* `#if 0` / `#if false` branches are dead: their declarations are not reported;
* a function definition is `NAME (...)` at column 0 whose parameter list is
  followed by `{`; declarations (`...;`) and macro invocations (`DEFUN (...)` with
  an argument list and a docstring) are not definitions, whether or not the name
  is all caps;
* struct fields handle tab indentation, comma declarator lists
  (`int a, b;`), `ENUM_BF (T) name : 2` bitfields, function pointers
  (`Lisp_Object (*fn) (void);`), pointer depth and arrays;
* file-scope objects are only reported outside function bodies;
* macro bodies are captured with their continuations and fingerprinted.
"""

from __future__ import annotations

import hashlib
import re
from pathlib import Path

CFUNC = re.compile(r"^(?:\((?P<pname>[A-Za-z_]\w*)\)|(?P<name>[A-Za-z_]\w*))\s*\(")
DEFUN = re.compile(r'^DEFUN\s*\(\s*"(?P<lisp>[^"]+)"\s*,\s*(?P<cname>\w+)\s*,\s*'
                   r'(?P<sname>\w+)\s*,\s*(?P<min>[^,]+?)\s*,\s*(?P<max>[^,]+?)\s*,\s*'
                   r'(?P<n>[^,]+?)\s*,\s*doc\s*:\s*/\*(?P<doc>.*?)\*/', re.S | re.M)
DEFINE = re.compile(r"^#\s*define\s+(?P<name>[A-Za-z_]\w*)(?P<args>\([^)]*\))?\s*(?P<body>.*)$")
STRUCT_OPEN_INLINE = re.compile(r"^(?P<kind>struct|union)\s+(?P<name>[A-Za-z_]\w*)?\s*\{")
STRUCT_OPEN_NAME = re.compile(r"^(?P<kind>struct|union)\s+(?P<name>[A-Za-z_]\w*)?\s*$")
ENUM_OPEN = re.compile(r"^enum\s+(?P<name>[A-Za-z_]\w*)?\s*\{")
TYPE_WORDS = {
    "static", "extern", "const", "volatile", "inline", "INLINE", "register",
    "struct", "union", "enum", "unsigned", "signed", "long", "short", "int",
    "char", "float", "double", "void", "bool", "size_t", "ptrdiff_t", "bool_bf",
    "Lisp_Object", "EMACS_INT", "EMACS_UINT", "Lisp_Word", "specpdl_ref",
}
STATEMENT_WORDS = {
    "if", "else", "for", "while", "do", "switch", "case", "default", "return",
    "goto", "break", "continue", "sizeof", "typedef", "static_assert", "defined",
}
FIELD_LINE = re.compile(
    r"^(?P<type>[A-Za-z_][\w\s\*\(\),]*?)\s+(?P<decls>[^;{}()]*)\s*;\s*$")
FUNC_PTR_FIELD = re.compile(
    r"^(?P<type>[A-Za-z_][\w\s\*]*?)\s*\(\s*\*\s*(?P<name>[A-Za-z_]\w*)\s*\)\s*"
    r"\((?P<args>[^)]*)\)\s*(?P<tail>.*?);\s*$")
GLOBAL_LINE = re.compile(
    r"^(?P<type>(?:static\s+|extern\s+|const\s+)*"
    r"(?:struct\s+\w+\s*|union\s+\w+\s*|enum\s+\w+\s*|[A-Za-z_]\w*)"
    r"[\w\s\*]*?)\s*(?P<decls>[^;{}()=]*?)\s*(?:=\s*(?P<init>[^;]*))?;\s*$")
DECLARATOR = re.compile(r"^(?P<stars>\**)\s*(?P<name>[A-Za-z_]\w*)\s*"
                        r"(?P<array>(?:\[[^\]]*\])*)\s*(?::\s*(?P<bits>\d+))?$")
ENUM_MEMBER = re.compile(r"^\s*(?P<name>[A-Za-z_]\w*)\s*(?:=\s*(?P<value>[^,]+))?,?\s*$")


class Lines:
    """Line-accurate view of a C file: comments blanked, dead branches marked."""

    def __init__(self, text: str):
        self.raw = text
        self.clean = blank_comments(text)
        self.live: list[bool] = dead_branch_map(self.clean)
        self.rows = self.clean.split("\n")

    def __iter__(self):
        for i, line in enumerate(self.rows):
            yield i + 1, line, self.live[i]

    def at(self, lineno: int) -> str:
        return self.rows[lineno - 1]


def blank_comments(text: str) -> str:
    """Replace comments with spaces, preserving length and newlines."""
    out = []
    i, n = 0, len(text)
    state = "code"
    while i < n:
        ch = text[i]
        nxt = text[i + 1] if i + 1 < n else ""
        if state == "code":
            if ch == "/" and nxt == "*":
                state = "block"
                out.append("  ")
                i += 2
                continue
            if ch == "/" and nxt == "/":
                state = "line"
                out.append("  ")
                i += 2
                continue
            out.append(ch)
            i += 1
        elif state == "block":
            if ch == "*" and nxt == "/":
                state = "code"
                out.append("  ")
                i += 2
                continue
            out.append("\n" if ch == "\n" else " ")
            i += 1
        else:  # line comment
            if ch == "\n":
                state = "code"
                out.append("\n")
            else:
                out.append(" ")
            i += 1
    return "".join(out)


def dead_branch_map(text: str) -> list[bool]:
    """True for lines that are not inside an `#if 0` / `#if false` branch.

    Only literal-false conditions are treated as dead.  Everything else
    (`#ifdef`, `#if defined`, `#if HAVE_X`) is unknown at extraction time, so both
    branches stay live; that is why `#else`/`#elif` are not assumed dead.
    """
    live, stack = [], []      # stack of (outer_dead, branch_dead)
    for line in text.split("\n"):
        s = line.strip()
        dead = any(outer or branch for outer, branch in stack)
        if s.startswith("#"):
            body = s[1:].strip()
            word = body.split(maxsplit=1)[0] if body else ""
            rest = body[len(word):].strip()
            if word in ("if", "ifdef", "ifndef"):
                outer = dead
                branch = word == "if" and rest in ("0", "false")
                stack.append((outer, branch))
                dead = outer or branch
            elif word == "elif":
                if stack:
                    outer, _ = stack[-1]
                    stack[-1] = (outer, False)
                dead = any(o or b for o, b in stack)
            elif word == "else":
                if stack:
                    outer, _ = stack[-1]
                    stack[-1] = (outer, False)
                dead = any(o or b for o, b in stack)
            elif word == "endif":
                if stack:
                    stack.pop()
                dead = any(o or b for o, b in stack)
            else:
                dead = any(o or b for o, b in stack)
        live.append(not dead)
    while len(live) < len(text.split("\n")):
        live.append(True)
    return live


def strip_strings(line: str) -> str:
    return re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)


def function_spans(text: str) -> list[dict]:
    """Every function definition, with the body span it occupies.

    A definition is `NAME (...)` at column 0 whose parameter list is followed by
    `{` (on the same line or a later one, as Emacs writes it).  Declarations end
    in `;` and macro invocations such as `DEFUN (...)` are followed by another
    argument list, so neither is reported.
    """
    lines = Lines(text)
    rows = lines.rows
    spans: list[dict] = []
    i = 1
    while i <= len(rows):
        line, live = rows[i - 1], lines.live[i - 1]
        if not live or line[:1] in (" ", "\t", "#", ""):
            i += 1
            continue
        m = CFUNC.match(line)
        if not m or (m.group("name") or m.group("pname")) in STATEMENT_WORDS:
            i += 1
            continue
        # walk the parameter list to its matching close paren
        depth, j, k = 0, i - 1, m.end() - 1
        while j < len(rows):
            row = rows[j]
            while k < len(row):
                ch = row[k]
                if ch == "(":
                    depth += 1
                elif ch == ")":
                    depth -= 1
                    if depth == 0:
                        break
                k += 1
            if depth == 0 and k < len(row):
                break
            j += 1
            k = 0
        if j >= len(rows):
            break
        rest = rows[j][k + 1:].strip()
        body_line = None
        if rest.startswith("{"):
            body_line = j
        elif rest == "" or rest.startswith("__attribute__"):
            for jj in range(j + 1, min(j + 4, len(rows))):
                nxt = rows[jj].strip()
                if not nxt:
                    continue
                if nxt.startswith("{"):
                    body_line = jj
                break
        if body_line is None:
            i += 1
            continue
        depth2, end_line = 0, None
        for jj in range(body_line, len(rows)):
            row = rows[jj]
            depth2 += row.count("{") - row.count("}")
            if depth2 <= 0 and jj > body_line:
                end_line = jj
                break
            if depth2 <= 0 and "}" in row:
                end_line = jj
                break
        if end_line is None:
            end_line = len(rows)
        prev = ""
        p = i - 2
        while p >= 0:
            cand = rows[p].strip()
            if cand:
                prev = cand
                break
            p -= 1
        spans.append({"name": m.group("name") or m.group("pname"), "line": i,
                      "static": prev.startswith("static"),
                      "return_type": prev, "body_start": body_line + 1,
                      "body_end": end_line + 1})
        i = end_line + 1
    return spans


def functions(text: str) -> list[dict]:
    return function_spans(text)


def macros(text: str) -> dict[str, dict]:
    lines = Lines(text)
    out: dict[str, dict] = {}
    i = 0
    rows = lines.rows
    while i < len(rows):
        line, live = rows[i], lines.live[i]
        if live:
            m = DEFINE.match(line.strip()) if line.lstrip().startswith("#") else None
            if line.startswith("#"):
                m = DEFINE.match(line)
            if m:
                body = [m.group("body")]
                j = i
                while body[-1].rstrip().endswith("\\") and j + 1 < len(rows):
                    j += 1
                    body.append(rows[j])
                joined = "\n".join(body)
                token = re.sub(r"\s+", " ", joined.replace("\\\n", " ")).strip()
                out[m.group("name")] = {
                    "line": i + 1,
                    "args": m.group("args") or "",
                    "body": token,
                    "digest": hashlib.sha1(token.encode()).hexdigest()[:12],
                }
                i = j + 1
                continue
        i += 1
    return out


def constants(text: str) -> dict[str, str]:
    """Object-like macros with a literal value, plus enum members assigned a value."""
    out = {}
    for name, macro in macros(text).items():
        if macro["args"]:
            continue
        value = macro["body"].strip()
        if re.fullmatch(r"-?(0[xX][0-9a-fA-F]+|\d+)([uUlL]*)", value) or \
                re.fullmatch(r"'.*'", value) or re.fullmatch(r'"(?:[^"\\]|\\.)*"', value):
            out[name] = value
    # enum members with explicit values, scanned inside enum bodies
    stack = []
    for i, line, live in Lines(text):
        s = strip_strings(line)
        if not live:
            continue
        if ENUM_OPEN.match(s.strip()):
            stack.append(i)
            continue
        if stack and s.strip().startswith("}"):
            stack.pop()
            continue
        if stack:
            m = ENUM_MEMBER.match(s)
            if m and m.group("value"):
                out[m.group("name")] = m.group("value").strip()
    return out


def struct_fields(text: str) -> list[dict]:
    """Every field of every struct/union, with container and nesting path."""
    lines = Lines(text)
    rows = lines.rows
    stack: list[tuple[str, str]] = []
    pending = None
    out: list[dict] = []
    i = 0
    while i < len(rows):
        raw = rows[i]
        live = lines.live[i]
        stripped = raw.strip()
        i += 1
        if not live or not stripped or stripped.startswith("#"):
            continue
        m = STRUCT_OPEN_INLINE.match(stripped)
        if m:
            stack.append((m.group("kind"), m.group("name") or f"<anon@{i}>"))
            if stripped.count("{") == stripped.count("}"):
                stack.pop()
            continue
        m = STRUCT_OPEN_NAME.match(stripped)
        if m:
            pending = (m.group("kind"), m.group("name") or f"<anon@{i}>")
            continue
        if stripped == "{":
            if pending:
                stack.append(pending)
                pending = None
            continue
        if stripped.startswith("}"):
            for _ in range(stripped.count("}")):
                if stack:
                    stack.pop()
            pending = None
            continue
        if not stack:
            continue
        for field in _fields_on_line(stripped):
            outer = next((e for e in stack if not e[1].startswith("<anon")), stack[-1])
            out.append({"container": outer[1], "container_kind": outer[0],
                        "nesting": " > ".join(e[1] for e in stack),
                        "line": i, **field})
    return out


def _field_declarator(segment: str) -> dict | None:
    """Parse one declarator segment: `*name[3] : 7`."""
    seg = segment.strip()
    if not seg or "=" in seg:
        return None
    bits = ""
    mb = re.search(r":\s*([A-Za-z_0-9]+)\s*$", seg)   # width may be CHAR_BIT, not a number
    if mb:
        bits = mb.group(1)
        seg = seg[:mb.start()].strip()
    m = re.search(r"(\**)\s*(?P<name>[A-Za-z_]\w*)\s*"
                  r"(?P<array>(?:\[[^\]]*\])*)\s*$", seg)
    if not m or m.group("name") in STATEMENT_WORDS:
        return None
    return {"name": m.group("name"), "stars": m.group(1) or "",
            "array": m.group("array") or "", "bits": bits,
            "type_part": seg[:m.start()].strip()}


def _fields_on_line(stripped: str) -> list[dict]:
    if not stripped.endswith(";") or "=" in stripped:
        return []
    body = stripped[:-1].strip()
    if not body or body.startswith("return ") or body.startswith("typedef "):
        return []
    m = FUNC_PTR_FIELD.match(stripped)
    if m:
        return [{"c_type": m.group("type").strip() + " (*)", "name": m.group("name"),
                 "pointer_depth": 0, "array": "", "bitfield": "", "raw": stripped}]
    parts = split_declarators(body)
    if not parts:
        return []
    first = _field_declarator(parts[0])
    if not first or not first["type_part"]:
        return []
    ctype = first["type_part"]
    words = re.findall(r"[A-Za-z_]\w*", ctype)
    if not words:
        return []
    if any(w in STATEMENT_WORDS for w in words) and not any(w in TYPE_WORDS for w in words):
        return []
    out = [{"c_type": ctype, "name": first["name"],
            "pointer_depth": len(first["stars"]), "array": first["array"],
            "bitfield": first["bits"], "raw": stripped}]
    for seg in parts[1:]:
        d = _field_declarator(seg)
        if not d:
            continue
        out.append({"c_type": ctype, "name": d["name"],
                    "pointer_depth": len(d["stars"]), "array": d["array"],
                    "bitfield": d["bits"], "raw": stripped})
    return out


def split_declarators(decls: str) -> list[str]:
    """Split `a, *b, c[3]` on top-level commas."""
    parts, depth, cur = [], 0, ""
    for ch in decls:
        if ch in "([":
            depth += 1
        elif ch in ")]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return [p for p in (x.strip() for x in parts) if p]


def globals_(text: str) -> list[dict]:
    """File-scope objects outside any function body."""
    spans = function_spans(text)
    inside = set()
    for sp in spans:
        inside.update(range(sp["body_start"], sp["body_end"] + 1))
    lines = Lines(text)
    struct_depth = 0
    out = []
    for i, line, live in lines:
        stripped = strip_strings(line)
        if not live:
            continue
        if i in inside:
            continue
        if STRUCT_OPEN_INLINE.match(stripped.strip()) or ENUM_OPEN.match(stripped.strip()):
            struct_depth += stripped.count("{") - stripped.count("}")
            continue
        if struct_depth:
            struct_depth += stripped.count("{") - stripped.count("}")
            if struct_depth < 0:
                struct_depth = 0
            continue
        if not stripped or stripped[:1] in (" ", "\t") or stripped.startswith("#"):
            continue
        if not stripped.endswith(";"):
            continue
        m = GLOBAL_LINE.match(stripped)
        if not m:
            continue
        ctype = m.group("type").strip()
        if not ctype or ctype in STATEMENT_WORDS:
            continue
        for decl in split_declarators(m.group("decls")):
            d = DECLARATOR.match(decl.strip())
            if not d:
                continue
            out.append({"c_type": ctype, "name": d.group("name"),
                        "array": d.group("array") or "",
                        "pointer_depth": len(d.group("stars") or ""),
                        "static": ctype.startswith("static"),
                        "const": "const" in ctype,
                        "init": (m.group("init") or "").strip(), "line": i})
    return out


def defuns(text: str) -> list[dict]:
    """Every DEFUN, matched over the whole raw file (docstrings are comments)."""
    clean = blank_comments(text)
    live = dead_branch_map(clean)
    out = []
    for m in DEFUN.finditer(text):
        line_no = text.count("\n", 0, m.start()) + 1
        if not live[line_no - 1]:
            continue
        out.append({"lisp_name": m.group("lisp"), "c_name": m.group("cname"),
                    "s_name": m.group("sname"), "min": m.group("min").strip(),
                    "max": m.group("max").strip(), "n": m.group("n").strip(),
                    "line": line_no,
                    "doc_digest": hashlib.sha1(
                        re.sub(r"\s+", " ", m.group("doc")).strip().encode()).hexdigest()[:12]})
    return out


def macro_definitions (text: str) -> dict[str, int]:
    """How often each macro name is defined (more than once means the C picks a
    definition in a configuration branch)."""
    counts: dict[str, int] = {}
    for i, line, live in Lines(text):
        if not live or not line.lstrip ().startswith ("#"):
            continue
        m = DEFINE.match (line.strip ())
        if m:
            counts[m.group ("name")] = counts.get (m.group ("name"), 0) + 1
    return counts


def inventory(path: Path) -> dict:
    text = path.read_text(encoding="utf-8", errors="replace")
    funcs = functions(text)
    defs = defuns(text)
    structs = struct_fields(text)
    return {
        "path": str(path),
        "lines": text.count("\n"),
        "functions": funcs,
        "defuns": defs,
        "macros": macros(text),
        "macro_duplicates": macro_definitions(text),
        "constants": constants(text),
        "struct_fields": structs,
        "globals": globals_(text),
    }
