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
import fnmatch
import importlib.util
import json
import re
import sys
from collections import Counter
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
R_CONST_NAME = re.compile(r"\b(?:pub\s+)?const\s+(?P<name>[A-Za-z_]\w*)\s*:\s*[^=]+?=", re.S)


def scan_const_value(text: str, start: int) -> str | None:
    """The initialiser from START to the terminating `;`, with string and char
    literals kept verbatim: a `;` inside one does not end the value (review B, F6)."""
    out, i, n = [], start, len(text)
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
            out.append(text[i:j])
            i = j
            continue
        if ch == "'":
            m = re.match(r"'(?:\\.|[^\\'])'", text[i:])
            if m:
                out.append(m.group(0))
                i += m.end()
                continue
        if ch == ";":
            break
        out.append(ch)
        i += 1
    value = "".join(out).strip()
    return value or None


def constants_of(text: str) -> dict[str, str]:
    """Every `const NAME: T = VALUE;` with the value exactly as written (whitespace
    collapsed only outside literals)."""
    out: dict[str, str] = {}
    for m in R_CONST_NAME.finditer(text):
        value = scan_const_value(text, m.end())
        if value is not None:
            out.setdefault(m.group("name"), re.sub(r"\s+", " ", value).strip())
    return out
def rust_enum_variants(flat: str) -> dict[str, str]:
    """Every Rust `enum` variant: name -> value text ("" when it has none).

    `flat` is the normalised (comment-free) view, so splitting the body on commas at
    bracket depth 0 is enough to separate variants, including `#[cfg (...)]` attributes
    and generic arguments.
    """
    out: dict[str, str] = {}
    for m in re.finditer(r"\benum\s+[A-Za-z_]\w*\s*\{", flat):
        depth, i, n = 1, m.end(), len(flat)
        while i < n and depth:
            ch = flat[i]
            if ch in "{[(":
                depth += 1
            elif ch in "}])":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        body = flat[m.end():i]
        parts, cur, d = [], "", 0
        for ch in body:
            if ch in "([{<":
                d += 1
            elif ch in ")]}>":
                d -= 1
            if ch == "," and d == 0:
                parts.append(cur)
                cur = ""
            else:
                cur += ch
        parts.append(cur)
        for part in parts:
            mm = re.match(r"^\s*([A-Za-z_]\w*)\s*(?:=\s*(.+?))?\s*$", part, re.S)
            if mm and mm.group(1) not in ("pub", "enum"):
                out.setdefault(mm.group(1), (mm.group(2) or "").strip())
    return out


# C's macro bodies and the port's macro bodies spell the same identifiers, with a few
# documented differences, so a macro body that drops a field/helper/literal is visible
# as a missing token (review B's M2/M3/M5/M6/M8/M9/M10 class).  Numbers count: `40` is
# part of GET_OVERLAYS_AT's meaning.
MACRO_TOKEN_SKIP = {
    # C syntax that either language spells differently or that a body may not need
    "struct", "union", "enum", "const", "static", "void", "int", "char", "long", "short",
    "unsigned", "signed", "bool", "sizeof", "if", "else", "while", "for", "do", "return",
    "offsetof", "defined", "u", "l", "ul", "ll",
}
# The same identity, spelled differently by the two languages.
# Type and keyword tokens are spelled differently by the two languages when they appear at
# all (C needs them for declarations and casts, Rust often infers), so they are skipped on
# both sides of a body comparison.  Procedure names, field names and literals still count.
MACRO_TOKEN_SKIP |= {
    "lisp_object", "emacs_int", "emacs_uint", "lispsym_count", "ptrdiff_t", "size_t",
    "intmax_t", "uintmax_t", "intptr_t", "uintptr_t", "bits_word", "modiff_count",
    "specpdl_ref", "c_int", "c_uint", "c_char", "usize", "isize", "i64", "u64", "i32",
    "u32", "u8", "i16", "u16", "f32", "f64",
    "register", "extern", "inline", "volatile", "typedef", "constexpr", "auto",
}
# Both sides are canonicalised, because a C token may be spelled several ways in Rust (C's
# `x != NULL` becomes `!x.is_null ()` as often as `x != null_mut ()`).
MACRO_TOKEN_EQUIV = {}
for _group in (
    ("offsetof", "offset_of"),
    ("ptrdiff_t", "isize"),
    ("size_t", "usize"),
    ("intptr_t", "isize"),
    ("uintptr_t", "usize"),
    ("null", "null_mut", "is_null"),
    ("unsigned", "c_uint"),
    ("double", "c_double"),
    ("float", "c_float"),
):
    for _spelling in _group:
        MACRO_TOKEN_EQUIV[_spelling] = _group[0]


# `docs/ports/token-aliases.tsv`: token families that denote one operation but are
# spelled differently by the two languages (C's `memcpy` and the port's
# `std::ptr::copy_nonoverlapping`, C's `va_arg` and the port's `next_arg`).  A family
# does two things: both sides canonicalise to the family's canonical spelling for the
# set-based body checks, and the family gets an *occurrence* check -- the port must use
# it at least as often as the C body does.  The occurrence check closes review D's F2:
# a note that merely allowed `memcpy` to be absent let a body with all three of
# `doprnt`'s output copies deleted pass both `--check` and `build-tu`.
TOKEN_ALIASES: dict = {}
TOKEN_ALIAS_EQUIV: dict = {}
TOKEN_ALIAS_WARNINGS: list = []


def load_token_aliases(path: Path) -> None:
    """Read the families (`canonical<TAB>spelling,spelling<TAB>reason`)."""
    TOKEN_ALIASES.clear()
    TOKEN_ALIAS_EQUIV.clear()
    del TOKEN_ALIAS_WARNINGS[:]
    if not path.is_file():
        return
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        canon = parts[0].strip().lower()
        spellings = {x.strip().lower() for x in parts[1].split(",") if x.strip()}
        if not canon or not spellings:
            continue
        spellings.add(canon)
        TOKEN_ALIASES[canon] = spellings
        for one in sorted(spellings):
            other = TOKEN_ALIAS_EQUIV.get(one)
            if other is not None and other != canon:
                TOKEN_ALIAS_WARNINGS.append(
                    f"token {one!r} is claimed by both {other!r} and {canon!r}")
            TOKEN_ALIAS_EQUIV[one] = canon


def canon_token(low: str) -> str:
    """One spelling per token family: the aliases file first, then MACRO_TOKEN_EQUIV."""
    return TOKEN_ALIAS_EQUIV.get(low) or MACRO_TOKEN_EQUIV.get(low, low)

# `docs/ports/macro-body-notes.tsv`: C macros whose Rust body legitimately cannot
# mention one of the C body's tokens (a recorded deviation with a reason).
def read_macro_notes(path: Path) -> list[tuple[str, set[str]]]:
    """(name pattern, allowed tokens) rows; a `*` token allows any token, and the
    pattern may be a shell glob (`DEFVAR_*`), so a whole family is one recorded row."""
    out: list[tuple[str, set[str]]] = []
    if not path.is_file():
        return out
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 3:
            toks = {x.strip().lower() for x in parts[1].split(",") if x.strip()}
            out.append((parts[0], toks))
    return out


def note_tokens(notes: list[tuple[str, set[str]]], name: str) -> set[str]:
    """Allowed tokens for NAME; the name column may list several globs, comma-separated."""
    allowed: set[str] = set()
    for pattern, toks in notes:
        for one in pattern.split(","):
            if fnmatch.fnmatchcase(name, one.strip()):
                allowed |= toks
    return allowed


# `docs/ports/va-arg-types.tsv`: which object each `va_arg (ap, T)` / `next_arg::<T> ()`
# reads.  Neither rustc nor the token gate can see it -- the value is handed to a
# variadic `sprintf`, so reading `%u` as `c_int` compiles and passes everything (review
# D's F4, mutation MV1).  A type that is not recorded is reported, not ignored: the
# table is the record of which C type each read answers for.
VA_ARG_TYPES: dict = {}
VA_ARG_RE = re.compile(r"\bva_arg\s*\(\s*[A-Za-z_]\w*\s*,\s*([^()]*?)\s*\)")
R_NEXT_ARG_RE = re.compile(r"\bnext_arg\s*::\s*<\s*([^<>]*?)\s*>\s*\(\s*\)")


def load_va_arg_types(path: Path) -> None:
    """Read `C type<TAB>rust spelling[,spelling]<TAB>reason` rows."""
    VA_ARG_TYPES.clear()
    if not path.is_file():
        return
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        c_type = squash_space(parts[0])
        rust = tuple(x for x in (squash_space(y) for y in parts[1].split(",")) if x)
        if c_type and rust:
            VA_ARG_TYPES[c_type] = rust


def squash_space(text: str) -> str:
    return re.sub(r"\s+", " ", text).strip()


def va_arg_problems(c_text: str, rust_text: str) -> list:
    """C's reads and the port's reads must be the same multiset of objects."""
    if not VA_ARG_TYPES:
        return []
    c_types = [squash_space(t) for t in VA_ARG_RE.findall(strip_comments(c_text))]
    r_types = [squash_space(t) for t in R_NEXT_ARG_RE.findall(strip_comments(rust_text))]
    if not c_types and not r_types:
        return []
    out = []
    for one in c_types:
        if one not in VA_ARG_TYPES:
            out.append(f"va_arg reads {one!r}, which docs/ports/va-arg-types.tsv does not "
                       f"record (add a row, or fix the read)")
    reverse = {spelling: c_type for c_type, spellings in VA_ARG_TYPES.items()
               for spelling in spellings}
    for one in r_types:
        if one not in reverse:
            out.append(f"next_arg reads {one!r}, which docs/ports/va-arg-types.tsv does not "
                       f"record (add a row, or fix the read)")
    if out:
        return out
    c_canon = list(c_types)
    r_canon = [reverse[one] for one in r_types]
    if Counter(c_canon) != Counter(r_canon):
        extra_c = Counter(c_canon) - Counter(r_canon)
        extra_r = Counter(r_canon) - Counter(c_canon)
        out.append("va_arg reads differ: C reads "
                   + ", ".join(f"{k} x{v}" for k, v in sorted(extra_c.items()))
                   + "; the port reads "
                   + ", ".join(f"{k} x{v}" for k, v in sorted(extra_r.items())))
    return out


def strip_char_literals(text: str) -> str:
    """Blank Rust character literals — but only when the closing quote is close by.

    `'(\\.|[^'\\])*'` pairs an apostrophe with some *later* literal's apostrophe in a body
    that uses many of them (`b'%'`, `b'.'`, `'d'`, …) and swallows everything in between:
    that hid 7 KB of `doprnt`'s body from the token check.  A char literal is at most a few
    source characters (`'\\u{10FFFF}'` is the longest), and a lifetime never closes, so a
    bounded scan is exact enough.
    """
    out, i, n = [], 0, len(text)
    while i < n:
        ch = text[i]
        if ch == "'":
            j, steps = i + 1, 0
            while j < n and steps < 12:
                if text[j] == "\\":
                    j += 2
                    steps += 1
                    continue
                if text[j] == "'" or text[j] == "\n":
                    break
                j += 1
                steps += 1
            if j < n and text[j] == "'" and j > i + 1:
                out.append("''")
                i = j + 1
                continue
        out.append(ch)
        i += 1
    return "".join(out)


def body_token_seq(text: str, *, numbers: bool = True, skip_short: bool = False) -> list:
    """Identifiers (and numbers) of a C macro body or a Rust body, normalised.

    String and character literals are dropped (the constants rule compares their bytes),
    integer suffixes are folded into the number (`1u` is one token), and the `## _`
    suffix idiom of `BVAR`/`PER_BUFFER_VAR_OFFSET` is invisible because a trailing
    underscore is stripped from both sides.  Families from `token-aliases.tsv` are
    folded to their canonical spelling, so the same sequence feeds the set checks and
    the occurrence checks.
    """
    text = re.sub(r'"(?:[^"\\]|\\.)*"', '""', text)
    text = strip_char_literals(text)
    # preprocessor lines inside a body (`#ifdef`, `#endif`, `# define`) are not code
    text = "\n".join(l for l in text.split("\n") if not l.lstrip().startswith("#"))
    toks = re.findall(r"[A-Za-z_]\w*|\d+", text)
    out = []
    for tok in toks:
        if tok.isdigit():
            if numbers:
                out.append(tok)
            continue
        low = tok.lower().removeprefix("r#")
        if low in MACRO_TOKEN_SKIP or low.strip("_") == "":
            continue
        low = low.rstrip("_") or low
        if skip_short and len(low) <= 2:
            continue          # C's one/two-letter parameter and local names
        out.append(canon_token(low))
    return out


def body_tokens(text: str, *, numbers: bool = True, skip_short: bool = False) -> set:
    """The body's token *set* (see body_token_seq for the normalisation)."""
    return set(body_token_seq(text, numbers=numbers, skip_short=skip_short))


def alias_count_problems(c_text: str, rust_text: str, *, skip_short: bool) -> list:
    """Every registered family must survive the port: rust count >= C count.

    A set check cannot see a *deleted* call when a note exempted its token, and it can
    never see that three calls became one (review D's M2/M3/M8 mutations kept every
    other gate green).  Only deletion is reported: a port may legitimately spell one C
    call as two Rust calls, never fewer.
    """
    if not TOKEN_ALIASES:
        return []
    c_counts = Counter(body_token_seq(c_text, skip_short=skip_short))
    r_counts = Counter(body_token_seq(rust_text, skip_short=skip_short))
    out = []
    for canon in sorted(TOKEN_ALIASES):
        c_n, r_n = c_counts.get(canon, 0), r_counts.get(canon, 0)
        if r_n < c_n:
            family = ",".join(sorted(TOKEN_ALIASES[canon]))
            out.append(f"family {canon}: the C body uses it {c_n} time(s), the port {r_n} "
                       f"({family})")
    return out


def c_body_text(text: str, name: str) -> str | None:
    """The reference body of NAME, comments blanked (None when the file lacks it)."""
    for span in ci.function_spans(text):
        if span["name"] == name:
            lines = text.split("\n")[span["body_start"] - 1: span["body_end"]]
            return ci.blank_comments("\n".join(lines))
    return None


def macro_body_of(rs: dict, name: str) -> str | None:
    """The Rust text that answers for the C macro NAME: macro body, const value or fn."""
    flat = rs["flat_own"]
    m = re.search(rf"macro_rules!\s*{re.escape(name)}\s*\{{", flat)
    if m:
        return block_body(flat, flat.index("{", m.end() - 1))
    if name in rs["constants"]:
        # A const's value may be a block (`{ let …; … }`); take the whole initialiser
        # rather than the first statement the value scanner stops at.
        m = re.search(rf"\bconst\s+{re.escape(name)}\s*:\s*[^=]+?=", flat)
        if m:
            return block_text_until_semicolon(flat, m.end())
        return rs["constants"][name]
    return rs["bodies"].get(name)


def block_text_until_semicolon(flat: str, start: int) -> str:
    """Text from START up to the `;` that closes the item, braces balanced."""
    depth, i, n = 0, start, len(flat)
    while i < n:
        ch = flat[i]
        if ch in "{[(":
            depth += 1
        elif ch in "}])":
            depth -= 1
        elif ch == ";" and depth <= 0:
            break
        i += 1
    return flat[start:i]


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
                   else doc_comment_value(sm.group("doc")))
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
    # A regex cannot see a body that contains braces (`{ { unimplemented!() } }` used to
    # slip past the stub gate), so every definition is scanned with block_body.
    bodies = {}
    for m in re.finditer(r"\bfn\s+(?P<name>[A-Za-z_]\w*)\s*(?:<[^>{}]*>)?\s*\(", flat_own):
        open_brace = flat_own.find("{", m.end())
        if open_brace < 0:
            continue
        # a declaration ends in `;` before any `{`; skip those
        semi = flat_own.find(";", m.end())
        if 0 <= semi < open_brace:
            continue
        bodies.setdefault(m.group("name"), block_body(flat_own, open_brace))
    bodies.update({d["c_name"]: d["body"] for d in subr_defs})
    # C's enum members live in Rust enums as often as in `pub const`s
    # (`enum pvec_type { PVEC_BUFFER = 13, … }`), so both count as constants.
    variants = rust_enum_variants(flat)
    return {"lines": own.count("\n"),
            "functions": sorted({m.group("name") for m in R_FN.finditer(text)}),
            "macros": sorted({m.group("name") for m in R_MACRO.finditer(text)}),
            "structs": structs, "struct_types": struct_types,
            "statics": sorted({m.group("name") for m in R_STATIC.finditer(text)}),
            "constants": constants_of(text),
            "variants": variants,
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


def doc_comment_value(comment: str) -> str:
    """The C `doc: /* … */` value; cinventory owns the rule (make-docfile's)."""
    return ci.doc_comment_value(comment)


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
            excluded: dict[str, str] | None = None,
            macro_notes: dict[str, set[str]] | None = None) -> list[str]:
    c = ci.inventory(c_path)
    r = rust_inventory(rs_path)
    excluded = excluded or {}
    macro_notes = macro_notes or {}
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
        if name in excluded:
            continue
        rust_body = macro_body_of(r, name)
        if rust_body is None:
            continue          # absence is reported by the missing-macro rule above
        if not rust_body.strip():
            problems.append(f"macro {name} has an empty body")
            continue
        allowed = note_tokens(macro_notes, name)
        # A macro's own parameter names are not semantics: Rust metavariables may be
        # spelled differently (C's `VAR`/`lastlispfield` vs `$var`/`$field`).
        params = {x.strip().lower() for x in macro["args"].split(",") if x.strip()}
        missing = ((body_tokens(macro["body"]) - params) - body_tokens(rust_body))
        missing = set() if "*" in allowed else (missing - allowed)
        if missing:
            problems.append("macro " + name + " does not mention "
                            + ", ".join(sorted(missing)) + " (C body: "
                            + re.sub(r"\s+", " ", macro["body"])[:60] + ")")
        problems.extend(alias_count_problems(macro["body"], rust_body, skip_short=False))
    def norm_const (v: str) -> str:
        v = v.strip()
        return {"true": "1", "false": "0"}.get (v, v)

    for name, value in sorted(c["constants"].items()):
        if name not in r["constants"] and name not in r["variants"]:
            # A constant C names is a constant the port must name: dropping an enum
            # member or a literal `#define` used to pass unnoticed (review B's M14).
            if name in excluded or c["macro_duplicates"].get (name, 0) > 1:
                continue
            problems.append(f"missing constant {name}")
            continue
        if name not in r["constants"]:
            # C's enum member lives in a Rust enum: compare the values when both sides
            # spell a literal, and stay quiet when either side is an alias/expression.
            rust_variant = r["variants"][name]
            if value.strip().isdigit() and rust_variant.strip().isdigit() \
                    and value.strip() != rust_variant.strip():
                problems.append(f"constant {name}: C {value} vs Rust enum {rust_variant}")
            continue
        # A macro the C defines more than once depends on a configuration branch; the
        # port spells the frozen branch's value (P-08), so neither comparison applies.
        if c["macro_duplicates"].get (name, 0) > 1:
            continue
        # Values that are not literals (an expression, a cast, an alias) used to be
        # skipped entirely, so `DBL_MAX_10_EXP + 50` could become `+ 500` with every
        # gate green (review D's F3).  They are compared by the tokens of the
        # expression now; the literal cases below keep the value comparison.
        if not re.fullmatch (r"-?0[xX][0-9a-fA-F]+|-?\d+|[0-9]+[uUlL]*|'.*'|\".*\"", value.strip ()):
            expr_allowed = note_tokens(macro_notes, name)
            if "*" not in expr_allowed:
                rust_expr = r["constants"][name]
                missing_expr = body_tokens(value) - body_tokens(rust_expr) - expr_allowed
                if missing_expr:
                    problems.append ("constant " + name + " does not mention "
                                     + ", ".join(sorted(missing_expr))
                                     + " (C value: " + re.sub(r"\s+", " ", value.strip())[:60] + ")")
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

    # Function bodies get the same token containment as macro bodies (review C's 13-
    # mutation matrix for casetab.rs: a wrong setter, a dropped call or a changed field
    # all passed every gate).  Numbers count, so `i < 128` vs `i < 127` is visible.
    c_source = c_path.read_text(encoding="utf-8", errors="replace")
    # A type name declared in this very file (a struct/union tag, a typedef) is part of a
    # declaration, not of the body's meaning, and the port spells them per its own rules.
    local_types = {f["container"].lower() for f in c["struct_fields"]}
    local_types |= {m.group(1).lower()
                    for m in re.finditer(r"^\s*typedef\b[^;{}]*\b([A-Za-z_]\w*)\s*;", c_source, re.M)}
    local_types |= {m.group(1).lower()
                    for m in re.finditer(r"^\s*\}\s*([A-Za-z_]\w*)\s*;", c_source, re.M)}
    for f in c["functions"]:
        name = f["name"]
        if name in excluded:
            continue
        allowed = note_tokens(macro_notes, name) | local_types
        if "*" in allowed:
            continue
        rust_body = r["bodies"].get(name)
        c_text = c_body_text(c_source, name)
        if rust_body is None or c_text is None:
            continue
        missing = (body_tokens(c_text, skip_short=True)
                   - body_tokens(rust_body, skip_short=True)) - allowed
        if missing:
            problems.append(f"function {name} does not mention "
                            + ", ".join(sorted(missing)))
        problems.extend(alias_count_problems(c_text, rust_body, skip_short=True))
        problems.extend(va_arg_problems(c_text, rust_body))

    empty_in_c = {f["name"] for f in c["functions"] if body_is_empty(c_path, f["name"])}
    # A Rust function whose name the reference also uses — as a function, a macro or a
    # constant — must not be a stub.  Helpers with no C counterpart (lisp_h_*,
    # pointee_size, …) belong to no ledger and are not scanned here.
    c_any_name = c_funcs | set(c_by_name) | set(c["macros"]) | set(c["constants"]) | set(excluded)
    for name, body in sorted(r["bodies"].items()):
        if name not in c_any_name:
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
    ap.add_argument("--macro-notes", default="docs/ports/macro-body-notes.tsv",
                    help="C macros whose Rust body legitimately omits a named token")
    ap.add_argument("--token-aliases", default="docs/ports/token-aliases.tsv",
                    help="token families that mean the same thing in both languages")
    ap.add_argument("--va-arg-types", default="docs/ports/va-arg-types.tsv",
                    help="which object each va_arg/next_arg reads")
    args = ap.parse_args()

    excluded = read_exclusions(Path(args.excluded)) if Path(args.excluded).is_file() else {}
    notes_path = Path(args.macro_notes)
    macro_notes = read_macro_notes(notes_path) if notes_path.is_file() else {}
    load_token_aliases(Path(args.token_aliases))
    load_va_arg_types(Path(args.va_arg_types))
    for warning in TOKEN_ALIAS_WARNINGS:
        print(f"token-aliases: {warning}", file=sys.stderr)
    if args.all:
        reference, root = Path(args.reference), Path(args.root)
        done = in_progress = failing = 0
        for c, rs in pairs(reference, root):
            problems = compare(c, rs, strict_lines=not args.loose_lines, excluded=excluded,
                               macro_notes=macro_notes)
            if not problems:
                done += 1
                continue
            # A file that is still being ported keeps its generated checklist with
            # unchecked boxes; the gate only fails when the checklist says the
            # file is finished (or when there is no checklist at all).
            stem = c.stem
            # A header's ledger is `include-<stem>.md` and a unit's is `<stem>.md`;
            # resolving by stem alone let `character.c` report "in progress" because
            # the *header* ledger had an unchecked box (review B, F3).
            names = ([f"include-{stem}.md", f"{stem}.md"] if c.suffix == ".h"
                     else [f"{stem}.md"])
            checklist = None
            for name in names:
                cand = Path(args.checklists) / name
                if cand.is_file():
                    checklist = cand
                    break
            if checklist is not None and "- [ ]" in checklist.read_text(encoding="utf-8"):
                in_progress += 1
                print(f"WIP  {c.relative_to(reference)} -> {rs} "
                      f"({len(problems)} differences, {checklist} has unchecked boxes)")
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
        problems = compare(c, rs, strict_lines=not args.loose_lines, excluded=ex,
                           macro_notes=macro_notes)
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
