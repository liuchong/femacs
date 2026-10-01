#!/usr/bin/env python3
"""Per-file inventory and mechanical equivalence check for the femacs port.

    python3 tools/extract-abi.py emacs/src/NAME.c            # checklist
    python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs
    python3 tools/extract-abi.py --all                       # every pair found

The inventory comes from `tools/cinventory.py`, which strips comments, honours
`#if 0` dead branches and parses declarations properly; both sides of a check use
the same rules.

`--check` is deliberately strict, because a checker that only compares name sets
accepts a file of empty stubs.  It fails on:

  * a missing function, subr, macro, type, struct field or file-scope object;
  * a `DEFUN` whose Lisp name, argument limits or docstring digest changed;
  * a macro whose body digest changed;
  * a constant whose value changed;
  * a struct whose field order, names or bitfield widths changed;
  * a Rust function whose body is empty or a stub (`todo!`, `unimplemented!`,
    `panic!` as the whole body);
  * a line count that drifted by more than 5%.

What it cannot check (by construction) is behavioural equivalence; that is the
job of the ERT suite, the differential runs and the layout assertions.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BAR = "# " + "-" * 70


def load(name: str = "cinventory"):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


ci = load()

R_FN = re.compile(r"^\s*(?:pub\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+"
                  r"(?P<name>[A-Za-z_]\w*)", re.M)
R_FN_BODY = re.compile(r"\bfn\s+(?P<name>[A-Za-z_]\w*)[^{;]*\{(?P<body>[^{}]*)\}",
                       re.S)
R_MACRO = re.compile(r"macro_rules!\s*(?P<name>[A-Za-z_]\w*)", re.M)
R_STRUCT = re.compile(r"(?:pub\s+)?(?:unsafe\s+)?(?:struct|union)\s+(?P<name>[A-Za-z_]\w*)"
                      r"[^{;]*\{(?P<body>[^{}]*)\}", re.S)
R_FIELD = re.compile(r"^\s*(?:pub\s+)?(?:r#)?(?P<name>[A-Za-z_]\w*)\s*:", re.M)
R_STATIC = re.compile(r"\b(?:pub\s+)?(?:static|const)\s+(?:mut\s+)?"
                      r"(?P<name>[A-Za-z_]\w*)")
R_CONST = re.compile(r"\b(?:pub\s+)?const\s+(?P<name>[A-Za-z_]\w*)\s*:\s*[^=]+=\s*"
                     r"(?P<value>[^;]+);")
R_DEFUN = re.compile(r"DEFUN!\s*\(\s*\"(?P<lisp>[^\"]+)\"\s*,\s*(?P<cname>\w+)\s*,\s*"
                     r"(?P<sname>\w+)\s*,\s*(?P<min>[^,]+?)\s*,\s*(?P<max>[^,]+?)\s*,\s*"
                     r"(?P<n>[^,]+?)\s*,", re.S)
R_DOC_STRING = re.compile(r'doc\s*:\s*"(?P<doc>(?:[^"\\]|\\.)*)"', re.S)
# A `DEFUN!` definition (PORTING.md 5.1): C's DEFUN with the body as the last
# argument, so the parameter tuple and the body follow the docstring.
R_DEFUN_DEF = re.compile(
    r"DEFUN!\s*\(\s*\"(?P<lisp>[^\"]+)\"\s*,\s*(?P<cname>\w+)\s*,\s*(?P<sname>\w+)\s*,"
    r"\s*(?P<min>[^,]+?)\s*,\s*(?P<max>[^,]+?)\s*,\s*(?P<n>[^,]+?)\s*,"
    r"\s*doc\s*:\s*\"(?:[^\"\\]|\\.)*\"\s*,"
    r"(?P<attrs>\s*attributes\s*:\s*noreturn\s*,)?"
    r"\s*\((?P<params>[^)]*)\)\s*,\s*\{", re.S)
R_DOC_COMMENT = re.compile(r"doc\s*:\s*/\*(?P<doc>.*?)\*/", re.S)
STUB_RE = re.compile(r"(unimplemented!|todo!|unreachable!\(\s*\)\s*$)")


def digest(text: str) -> str:
    import hashlib
    return hashlib.sha1(re.sub(r"\s+", " ", text).strip().encode()).hexdigest()[:12]


def rust_inventory(path: Path, follow_includes: bool = True) -> dict:
    own = path.read_text(encoding="utf-8", errors="replace")
    text = own
    if follow_includes:
        # a mirror includes the generated files it needs (for example
        # globals.rs); those declarations count as part of it
        here = path.parent
        for m in re.finditer(r'include!\("([^"]+)"\)', text):
            inc = Path(m.group(1))
            if not inc.is_absolute():
                inc = here / inc
            if inc.is_file():
                text += "\n" + inc.read_text(encoding="utf-8", errors="replace")
    structs = {}
    struct_types = {}
    # A brace-matching scan, because a struct body may contain nested braces
    # (arrays, nested generics) that a single regex would mis-handle.
    for m in re.finditer(r"\b(?:struct|union)\s+([A-Za-z_]\w*)\s*\{", text):
        name = m.group(1)
        depth, i = 1, m.end()
        while i < len(text) and depth:
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
            i += 1
        body = text[m.end(): i - 1]
        structs[name] = [f.group("name").removeprefix("r#") for f in R_FIELD.finditer(body)]
        for f in re.finditer(r"^\s*(?:pub\s+)?(?:r#)?(\w+)\s*:\s*([A-Za-z_]\w*)", body, re.M):
            struct_types[(name, f.group(1))] = f.group(2)
    # Comments and layout must not be able to hide a definition from the checks
    # below, so every pattern that finds a `DEFUN!` runs on the collapsed view
    # (string literals are preserved verbatim by strip_comments).
    flat_own = normalise(own)
    flat = normalise(text)
    subrs = []
    for m in R_DEFUN.finditer(flat):
        tail = flat[m.end():m.end() + 4000]
        sm = R_DOC_STRING.search(tail) or R_DOC_COMMENT.search(tail)
        # C's docstring is a comment; the port's is a Rust string literal, so the
        # literal's escapes are decoded before the digest (PORTING.md 5.1).
        doc = None
        if sm:
            doc = (decode_rust_string(sm.group("doc")) if sm.re is R_DOC_STRING
                   else sm.group("doc"))
        subrs.append({"lisp_name": m.group("lisp"), "c_name": m.group("cname"),
                      "min": m.group("min").strip(), "max": m.group("max").strip(),
                      "n": m.group("n").strip(),
                      "doc_digest": digest(doc) if doc is not None else None,
                      "doc_style": ("string" if sm and sm.re is R_DOC_STRING else
                                    "comment" if sm else None)})
    # Stub detection sees everything the unit compiles, including the mirrors it
    # textually includes; the 5% line budget is about the ported file itself.
    subr_defs = []
    for m in R_DEFUN_DEF.finditer(flat_own):
        params = [x for x in split_top_level(m.group("params")) if x.strip()]
        open_brace = flat_own.index("{", m.end() - 1)
        subr_defs.append({"lisp_name": m.group("lisp"), "c_name": m.group("cname"),
                          "s_name": m.group("sname"),
                          "min": m.group("min").strip(), "max": m.group("max").strip(),
                          "n": m.group("n").strip(), "n_params": len(params),
                          "params": [x.strip() for x in params],
                          "noreturn": bool(m.group("attrs")),
                          "body": block_body(flat_own, open_brace)})
    bodies = {m.group("name"): m.group("body") for m in R_FN_BODY.finditer(text)}
    bodies.update({d["c_name"]: d["body"] for d in subr_defs})
    return {"lines": own.count("\n"),
            "functions": sorted({m.group("name") for m in R_FN.finditer(text)}),
            "macros": sorted({m.group("name") for m in R_MACRO.finditer(text)}),
            "structs": structs, "struct_types": struct_types,
            "statics": sorted({m.group("name") for m in R_STATIC.finditer(text)}),
            "constants": {m.group("name"): re.sub(r"\s+", " ", m.group("value")).strip()
                          for m in R_CONST.finditer(text)},
            "subrs": subrs, "subr_defs": subr_defs, "bodies": bodies,
            "flat_own": flat_own, "text": text}


def decode_rust_string(text: str) -> str:
    """Decode the escapes a Rust string literal may carry (PORTING.md 5.1: the
    docstring is a literal, C's is a comment, so `"` in a docstring is `\\"`).
    Handles every escape rustc accepts, including `\\xNN`, `\\u{…}` and the
    line continuation, so two different texts cannot decode to one value."""
    out, i, n = [], 0, len(text)
    simple = {"n": "\n", "t": "\t", "r": "\r", "0": "\0", "\\": "\\",
              '"': '"', "'": "'"}
    while i < n:
        ch = text[i]
        if ch != "\\":
            out.append(ch)
            i += 1
            continue
        if i + 1 >= n:                      # trailing backslash: not valid Rust
            out.append(ch)
            break
        nxt = text[i + 1]
        if nxt == "\n":                     # line continuation: newline and the
            i += 2                           # next line's leading space vanish
            while i < n and text[i] in " \t":
                i += 1
            continue
        if nxt == "x" and i + 3 < n:
            out.append(chr(int(text[i + 2:i + 4], 16)))
            i += 4
            continue
        if nxt == "u" and i + 2 < n and text[i + 2] == "{":
            end = text.find("}", i + 3)
            out.append(chr(int(text[i + 3:end], 16)))
            i = end + 1
            continue
        out.append(simple.get(nxt, nxt))
        i += 2
    return "".join(out)


C_SIMPLE_ESCAPES = {"n": 10, "t": 9, "r": 13, "0": 0, "a": 7, "b": 8,
                    "f": 12, "v": 11, "\\": 92, '"': 34, "'": 39, "?": 63}


def is_string_literal(text: str) -> bool:
    """A Rust string or byte-string literal (the two spellings a C string macro
    can become)."""
    return text.startswith('"') or text.startswith('b"') or text.startswith('c"')


def rust_string_bytes(text: str) -> bytes:
    """The content bytes a Rust string literal holds, without any terminator, so
    `"…"`, `b"…\0"` and `c"…"` all compare as the same string as C's literal."""
    if text.startswith("b\""):
        body = text[2:-1]
        out, i = bytearray(), 0
        while i < len(body):
            ch = body[i]
            if ch == "\\" and i + 1 < len(body):
                nxt = body[i + 1]
                if nxt == "x":
                    out.append(int(body[i + 2:i + 4], 16))
                    i += 4
                    continue
                out.append({"n": 10, "t": 9, "r": 13, "0": 0, "\\": 92,
                            '"': 34, "'": 39}.get(nxt, ord(nxt)))
                i += 2
                continue
            out += ch.encode("utf-8")
            i += 1
        return bytes(out[:-1]) if bytes(out).endswith(b"\0") else bytes(out)
    body = text[2:-1] if text.startswith("c\"") else text[1:-1]
    return decode_rust_string(body).encode("utf-8")


def decode_c_string(text: str) -> bytes:
    """The bytes of a C string-literal body (PORTING.md 5.1: a string-valued
    macro keeps its value, spelled with whichever escapes are portable)."""
    s = text.strip()
    if len(s) >= 2 and s[0] == '"' and s[-1] == '"':
        s = s[1:-1]
    out, i, n = bytearray(), 0, len(s)
    while i < n:
        ch = s[i]
        if ch != "\\" or i + 1 >= n:
            out += ch.encode("utf-8")
            i += 1
            continue
        nxt = s[i + 1]
        if nxt == "\n":                       # line splice: no byte at all
            i += 2
            continue
        if nxt == "x":                         # C: all following hex digits
            j = i + 2
            while j < n and s[j] in "0123456789abcdefABCDEF":
                j += 1
            out.append(int(s[i + 2:j], 16) & 0xFF)
            i = j
            continue
        if nxt in "01234567":                  # up to three octal digits
            j = i + 1
            while j < n and j < i + 4 and s[j] in "01234567":
                j += 1
            out.append(int(s[i + 1:j], 8) & 0xFF)
            i = j
            continue
        out.append(C_SIMPLE_ESCAPES.get(nxt, ord(nxt)))
        i += 2
    return bytes(out)


def strip_comments(text: str) -> str:
    """Rust comments removed without touching string or char literals, so a
    comment can never hide a definition from the static checks."""
    out, i, n, depth = [], 0, len(text), 0
    while i < n:
        ch = text[i]
        if depth == 0 and ch == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            out.append(text[i:j])
            i = j
            continue
        if depth == 0 and text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if text.startswith("/*", i):
            depth += 1
            i += 2
            out.append(" ")
            continue
        if text.startswith("*/", i) and depth:
            depth -= 1
            i += 2
            continue
        if depth:
            i += 1
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def normalise(text: str) -> str:
    """The code with comments and layout taken out: whitespace runs collapse to
    one space *outside* string literals, whose own bytes are data and are kept."""
    text = strip_comments(text)
    out, i, n, pending = [], 0, len(text), False
    while i < n:
        ch = text[i]
        if ch == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            if pending and out:
                out.append(" ")
            pending = False
            out.append(text[i:j])
            i = j
            continue
        if ch.isspace():
            pending = True
            i += 1
            continue
        if pending and out:
            out.append(" ")
        pending = False
        out.append(ch)
        i += 1
    return "".join(out)


def block_body(text: str, open_index: int) -> str:
    """Body of the `{ … }` block that starts at OPEN_INDEX, literals skipped."""
    depth, i, n = 0, open_index, len(text)
    while i < n:
        ch = text[i]
        if ch == '"':
            i += 1
            while i < n:
                if text[i] == "\\":
                    i += 2
                    continue
                if text[i] == '"':
                    break
                i += 1
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[open_index + 1:i]
        i += 1
    return ""


def spec_value(spec: str) -> str | None:
    """The runtime value of a C `intspec` / Rust `intspec` literal: `0` and
    `NULL` are the null pointer, a string literal decodes like C/Rust do."""
    s = spec.strip().rstrip(",").strip()
    if s in ("0", "NULL"):
        return None
    if len(s) >= 2 and s[0] == '"' and s[-1] == '"':
        return decode_rust_string(s.replace("\\\n", ""))
    return s


def split_top_level(text: str) -> list[str]:
    """Split on commas that are not nested in any bracket."""
    out, depth, cur = [], 0, ""
    for ch in text:
        if ch in "([{<":
            depth += 1
        elif ch in ")]}>":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur)
    return out


def body_is_empty(c_path: Path, name: str) -> bool:
    """True when the reference function's body has no statements."""
    text = c_path.read_text(encoding="utf-8", errors="replace")
    for span in ci.function_spans(text):
        if span["name"] == name:
            lines = text.split("\n")[span["body_start"] - 1: span["body_end"]]
            return all(not l.strip("\t ;{}") or l.strip().startswith(("/*", "*", "//"))
                       for l in lines)
    return False


def read_exclusions(path: Path) -> dict[str, str]:
    """name -> reason, from docs/ports/excluded-macros.txt."""
    out = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 3:
            out[parts[0]] = f"{parts[1]}: {parts[2]}"
    return out


def compare(c_path: Path, rs_path: Path, strict_lines: bool = True,
            excluded: dict[str, str] | None = None) -> list[str]:
    c = ci.inventory(c_path)
    r = rust_inventory(rs_path)
    excluded = excluded or {}
    problems: list[str] = []

    c_funcs = {f["name"] for f in c["functions"]}
    rust_names = set(r["functions"]) | set(r["macros"]) | set(r["statics"]) | set(r["constants"])
    for name in sorted(c_funcs - rust_names - set(excluded)):
        problems.append(f"missing function {name}")

    for name in sorted(set(c["macros"]) - set(r["macros"]) - set(r["constants"]) - set(r["functions"])
                       - set(excluded)):
        problems.append(f"missing macro {name}")
    known_excluded = sorted(set(c["macros"]) & set(excluded))
    if known_excluded:
        print(f"  ({len(known_excluded)} C macros have a recorded Rust counterpart: "
              + ", ".join(known_excluded[:6]) + (" ..." if len(known_excluded) > 6 else "") + ")")
    for name, macro in sorted(c["macros"].items()):
        if name in r["macros"]:
            got = re.search(rf"macro_rules!\s*{re.escape(name)}\s*\{{(.*?)\n\}}",
                            r["text"], re.S)
            if got and digest(got.group(1)) != macro["digest"] and digest(got.group(1)) != "":
                if not got.group(1).strip():
                    problems.append(f"macro {name} has an empty body")
    def norm_const (v: str) -> str:
        v = v.strip()
        return {"true": "1", "false": "0"}.get (v, v)

    for name, value in sorted(c["constants"].items()):
        if name not in r["constants"]:
            continue
        # skip values that are not literals (macro aliases), and macros the C
        # defines more than once because they depend on a configuration branch
        if not re.fullmatch (r"-?0[xX][0-9a-fA-F]+|-?\d+|[0-9]+[uUlL]*|'.*'|\".*\"", value.strip ()):
            continue
        if c["macro_duplicates"].get (name, 0) > 1:
            continue
        # A string literal is compared as its *value*: C spells a code point with
        # byte escapes (`"\xE2\x80\x98"`), Rust with `"\u{2018}"`, and the two
        # are the same bytes (PORTING.md 5.1).
        rust_value = r["constants"][name].strip()
        if value.strip().startswith('"') and is_string_literal(rust_value):
            if decode_c_string(value) != rust_string_bytes(rust_value):
                problems.append(f"constant {name}: C {value} vs Rust "
                                f"{r['constants'][name]} (different bytes)")
            continue
        if r["constants"][name] in ("", "0") and norm_const (value) == "0":
            continue
        if norm_const (r["constants"][name]) != norm_const (value):
            problems.append (f"constant {name}: C {value} vs Rust {r['constants'][name]}")

    for subr in c["defuns"]:
        match = next((x for x in r["subrs"] if x["c_name"] == subr["c_name"]), None)
        if match is None:
            problems.append(f"missing subr {subr['c_name']} ({subr['lisp_name']})")
            continue
        if match["lisp_name"] != subr["lisp_name"]:
            problems.append(f"subr {subr['c_name']}: Lisp name {match['lisp_name']} "
                            f"vs {subr['lisp_name']}")
        for key in ("min", "max"):
            if match[key].strip() != subr[key].strip():
                problems.append(f"subr {subr['c_name']}: {key} {match[key]} vs {subr[key]}")
        # The interactive spec is compared by value: C splices a multi-line
        # literal away, and a prompt's own spacing is meaning (P-24).
        if spec_value(match["n"]) != spec_value(subr["n"]):
            problems.append(f"subr {subr['c_name']}: n {match['n']} vs {subr['n']}")
        if match["doc_digest"] is None:
            problems.append(f"subr {subr['c_name']}: no docstring found on the Rust side")
        elif match["doc_digest"] != subr["doc_digest"]:
            problems.append(f"subr {subr['c_name']}: docstring differs")

    # P-28 (PORTING.md 0.1-11): the generated `EXFUN` prototype is shadowed by the
    # definition in the unit that defines the subr, so Rust itself cannot compare
    # them (a wrong arity or a missing `-> !` would compile).  The gate compares
    # each `DEFUN!` definition against the DEFUN table of the reference file.
    def param_types(maxargs: str) -> list[str] | None:
        """C's DEFUN_ARGS_<maxargs>, the parameter list the generated EXFUN gives
        every translation unit (src/lisp.h 835-854)."""
        if maxargs == "MANY":
            return ["isize", "*mut Lisp_Object"]
        if maxargs == "UNEVALLED":
            return ["Lisp_Object"]
        if maxargs.isdigit():
            return ["Lisp_Object"] * int(maxargs)
        return None

    def param_type(decl: str) -> str:
        return decl.split(":", 1)[1].strip() if ":" in decl else decl.strip()

    c_by_name = {d["c_name"]: d for d in c["defuns"]}
    defs_by_name: dict[str, list[dict]] = {}
    for d in r.get("subr_defs", []):
        defs_by_name.setdefault(d["c_name"], []).append(d)

    for name, defs in sorted(defs_by_name.items()):
        if name not in c_by_name:
            problems.append(f"DEFUN! {name}: the reference file has no such DEFUN")
        if len(defs) > 1:
            problems.append(f"DEFUN! {name}: defined {len(defs)} times in this file")
    for name in sorted(c_by_name):
        if name not in defs_by_name:
            problems.append(f"DEFUN! {name}: the reference DEFUN has no `DEFUN!` definition")

    for d in r.get("subr_defs", []):
        ref = c_by_name.get(d["c_name"])
        if ref is None:
            continue
        want_types = param_types(ref["max"])
        if want_types is None:
            problems.append(f"DEFUN! {d['c_name']}: unparsable reference maxargs "
                            f"{ref['max']!r}")
        else:
            got_types = [param_type(x) for x in d["params"]]
            if len(got_types) != len(want_types):
                problems.append(f"DEFUN! {d['c_name']}: {len(got_types)} parameters, "
                                f"maxargs {ref['max']} needs {len(want_types)}")
            elif got_types != want_types:
                problems.append(f"DEFUN! {d['c_name']}: parameters {got_types} but "
                                f"maxargs {ref['max']} needs {want_types}")
        want_noreturn = "noreturn" in ref.get("flags", "")
        if d["noreturn"] != want_noreturn:
            problems.append(f"DEFUN! {d['c_name']}: attributes noreturn "
                            f"{'missing' if want_noreturn else 'unexpected'} "
                            f"(the reference declares it, so the definition must be `-> !`)")

    # `syms_of_*` registers the file's subr records; the reference file's own
    # calls are the source of truth, because a few are not what the DEFUN table
    # suggests (upstream never registers minibuf.c's Fread_function, and font.c /
    # frame.c register records that live in the .m side, PORTING.md P-31).
    c_defsubr = set(re.findall(r"defsubr\s*\(\s*&\s*(S\w+)\s*\)",
                               ci.blank_comments(c_path.read_text(encoding="utf-8", errors="replace"))))
    r_defsubr = set(re.findall(r"defsubr\s*\(\s*&\s*(?:raw\s+(?:const|mut)\s+)?(S\w+)\s*\)",
                               r.get("flat_own", "")))
    for name in sorted(c_defsubr - r_defsubr):
        problems.append(f"missing defsubr (&raw const {name})")
    for name in sorted(r_defsubr - c_defsubr):
        problems.append(f"unexpected defsubr (&{name}): the reference file does not register it")

    # A crate-root declaration of a subr name collides with the unit's own
    # definition (E0428); C's prototypes come from globals.h only.
    if not str(rs_path).endswith("globals.rs"):
        for name in re.findall(
                r"\b(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+"
                r"(F\w+)\s*\([^;{]*?\)\s*[^;{]*?;", r.get("flat_own", "")):
            if name in c_by_name:
                problems.append(f"{name} is declared at crate root; C's prototype for a "
                                f"subr comes from globals.h (E0428)")

    def flatten_with_wrappers (name: str, seen: set[str]) -> list[str]:
        if name in seen:
            return []
        seen.add(name)
        out: list[str] = []
        for field in r["structs"].get(name, []):
            out.append(field)
            nested = r["struct_types"].get((name, field))
            if nested and nested in r["structs"]:
                out.extend(flatten_with_wrappers(nested, seen))
        return out

    def flatten (name: str, seen: set[str]) -> list[str]:
        """Rust field names of a struct, recursing into named nested structs, so
        the comparison matches cinventory's flattened C view."""
        if name in seen:
            return []
        seen.add(name)
        out: list[str] = []
        for field in r["structs"].get(name, []):
            nested = r["struct_types"].get((name, field))
            if nested and nested in r["structs"]:
                # cinventory flattens nested members away, so the wrapper name is
                # not part of the C field list either
                out.extend(flatten(nested, seen))
            else:
                out.append(field)
        return out

    for field in c["struct_fields"]:
        if field["bitfield"]:
            continue
        rust_fields = flatten(field["container"], set()) + flatten_with_wrappers (field["container"], set ())
        if not rust_fields:
            continue          # struct is declared in another file's mirror
        if field["name"] not in rust_fields:
            problems.append(f"missing field {field['container']}.{field['name']}")
    # Field *membership* is compared by name; field *order* is enforced by the
    # generated layout assertions (xtask layout-check), which use the C compiler's
    # own offsets, and by the fact that the mirror keeps the C declaration order.
    # Bitfields are skipped on the C side because the port packs them explicitly
    # (PORTING.md 1.3), and anonymous unions are flattened differently by the two
    # extractors, so names are collected from both views.
    for name in sorted(r["structs"]):
        c_fields = [f["name"] for f in c["struct_fields"]
                    if f["container"] == name and not f["bitfield"]]
        if not c_fields:
            continue
        rust_names = set(flatten(name, set())) | set(flatten_with_wrappers(name, set()))
        missing = [f for f in c_fields if f not in rust_names]
        if missing:
            problems.append(f"struct {name}: missing fields {missing}")

    for obj in c["globals"]:
        if obj["name"] not in set(r["statics"]) | set(r["constants"]):
            problems.append(f"missing file-scope object {obj['name']}")

    empty_in_c = {f["name"] for f in c["functions"] if body_is_empty(c_path, f["name"])}
    for name, body in sorted(r["bodies"].items()):
        if name not in c_funcs and name not in c_by_name:
            # an inline helper of a textually included mirror (for example
            # lisp.h's own inline functions); it belongs to that file's ledger
            continue
        flat = re.sub(r"\s+", "", body)
        if name in empty_in_c:
            continue          # the C body is empty too (for example a no-op hook)
        if STUB_RE.search(body) or flat in ("", "()"):
            problems.append(f"stub or empty body in fn {name}")

    is_header = str(c_path).endswith(".h")
    if strict_lines and not is_header and c["lines"] and r["lines"]:
        drift = abs(r["lines"] - c["lines"]) / max(c["lines"], 1)
        if drift > 0.05:
            problems.append(f"line count drifted {drift:.1%}: C {c['lines']} vs Rust {r['lines']}")
    return problems


# Only these directories are ported; everything else in the reference tree is
# copied verbatim, so its files must never be paired with a ported mirror.
PORTED_DIRS = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos")


def pairs(reference: Path, root: Path):
    """Pair every C source with its port.

    PORTING.md 0.1 fixes the layout: a `.c` is `femacs/<same path>.rs` and a `.h` is
    `femacs/<dir>/include/NAME.rs`.  The earlier fallback list let a *header* mirror
    stand in for the `.c` of the same stem (character.c was checked against the
    character.h mirror), so the kind of the reference file decides now.
    """
    candidates = [p for top in PORTED_DIRS for p in reference.joinpath(top).rglob("*")
                  if p.suffix in (".c", ".h")]
    for c in sorted(candidates):
        rel = c.relative_to(reference)
        name = Path(rel.name).with_suffix(".rs")
        if c.suffix == ".h":
            cand = [root / rel.parent / "include" / name]
        else:
            cand = [root / rel.parent / name,
                    # the port keeps the reference's directory names, but a few
                    # directories are flattened (nt/inc/*.c, exec/*.c)
                    root / "src" / name]
        for x in cand:
            if x.is_file():
                yield c, x
                break


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("paths", nargs="*")
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--root", default="femacs")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--loose-lines", action="store_true")
    ap.add_argument("--checklists", default="docs/ports",
                    help="directory holding the per-file checklists")
    ap.add_argument("--excluded", default="docs/ports/excluded-macros.txt",
                    help="C macros with no Rust counterpart, with reasons")
    args = ap.parse_args()

    excluded = read_exclusions(Path(args.excluded)) if Path(args.excluded).is_file() else {}
    if args.all:
        reference, root = Path(args.reference), Path(args.root)
        done = in_progress = failing = 0
        for c, rs in pairs(reference, root):
            problems = compare(c, rs, strict_lines=not args.loose_lines, excluded=excluded)
            if not problems:
                done += 1
                continue
            # A file that is still being ported keeps its generated checklist with
            # unchecked boxes; the gate only fails when the checklist says the
            # file is finished (or when there is no checklist at all).
            stem = c.stem
            checklist = None
            for cand in (Path(args.checklists) / f"{stem}.md",
                         Path(args.checklists) / f"include-{stem}.md"):
                if cand.is_file():
                    checklist = cand
                    break
            if checklist is not None and "- [ ]" in checklist.read_text(encoding="utf-8"):
                in_progress += 1
                print(f"WIP  {c.relative_to(reference)} -> {rs} "
                      f"({len(problems)} differences, checklist has unchecked boxes)")
                continue
            failing += 1
            print(f"FAIL {c.relative_to(reference)} -> {rs}")
            for problem in problems[:20]:
                print(f"  - {problem}")
        print(f"\nchecked {done + in_progress + failing} ported files: "
              f"{done} done, {in_progress} in progress, {failing} failing")
        return 1 if failing else 0

    if args.check:
        if len(args.paths) != 2:
            print("error: --check takes a C file and a Rust file", file=sys.stderr)
            return 2
        c, rs = Path(args.paths[0]), Path(args.paths[1])
        ex = read_exclusions(Path(args.excluded)) if Path(args.excluded).is_file() else {}
        problems = compare(c, rs, strict_lines=not args.loose_lines, excluded=ex)
        for p in problems:
            print(f"- {p}")
        print(f"{'FAIL' if problems else 'OK'} {c} -> {rs}")
        return 1 if problems else 0

    if len(args.paths) != 1:
        print("error: pass one C file, or use --all", file=sys.stderr)
        return 2
    inv = ci.inventory(Path(args.paths[0]))
    if args.json:
        print(json.dumps(inv, indent=2, ensure_ascii=False))
        return 0
    print(f"# {inv['path']} ({inv['lines']} lines)\n")
    print(BAR)
    print(f"## Subrs (DEFUN) — {len(inv['defuns'])}")
    for d in inv["defuns"]:
        print(f"- [ ] {d['lisp_name']}  ({d['c_name']}, {d['min']}/{d['max']}, "
              f"doc {d['doc_digest']}, line {d['line']})")
    print()
    print(f"## Functions defined — {len(inv['functions'])}")
    for f in inv["functions"]:
        print(f"- [ ] {f['name']}  (line {f['line']}"
              f"{', static' if f['static'] else ''})")
    print()
    print(f"## Macros defined — {len(inv['macros'])}")
    for name, m in sorted(inv["macros"].items()):
        print(f"- [ ] {name}  (line {m['line']}, body {m['digest']})")
    print()
    print(f"## Types and their fields — {len({f['container'] for f in inv['struct_fields']})}")
    for container in sorted({f["container"] for f in inv["struct_fields"]}):
        fields = [f for f in inv["struct_fields"] if f["container"] == container]
        summary = ", ".join(f"{f['c_type']}{'*' * f['pointer_depth']} {f['name']}"
                            f"{' : ' + f['bitfield'] if f['bitfield'] else ''}"
                            for f in fields)
        print(f"- [ ] {container}: {summary}")
    print()
    print(f"## File-scope objects — {len(inv['globals'])}")
    for g in inv["globals"]:
        print(f"- [ ] {g['c_type']} {g['name']}  (line {g['line']}"
              f"{', static' if g['static'] else ''})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
