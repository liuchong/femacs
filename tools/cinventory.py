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
import os
from pathlib import Path

CFUNC = re.compile(r"^(?:\((?P<pname>[A-Za-z_]\w*)\)|(?P<name>[A-Za-z_]\w*))\s*\(")


# A definition whose declaration specifiers share the line with the name:
# `void openat_save_fail (int errnum) { abort (); }` (gnulib's one-line form, openat-die.c
# 7-8) and the header form `INLINE int GLYPH_CHAR (GLYPH glyph) { return glyph.ch; }`
# (dispextern.h 388).  CFUNC keys on the name being the line's *first* token, so it saw
# neither, and a file whose only definitions are written this way looked like a file with no
# definitions at all - which then made the port's own definitions read as invented exported
# symbols.  This is a scanner rather than a regex on purpose: the equivalent
# `^(?:spec[\s\*]+)+(?P<name>\w*)\s*\(` backtracks exponentially on a line of many
# identifiers, and this runs on every line of every ported file.
_TYPED_TOKEN = re.compile(r"[ \t]*(?:\*|(?P<word>[A-Za-z_]\w*))")


def typed_def_name(line: str):
    """`(name, index of the opening paren)` for a same-line definition, else None.

    The shape is one or more declaration specifiers (identifiers, `struct TAG`, `*`), then
    the name, then `(`.  At least two tokens are required: a line that opens with the name
    itself is CFUNC's (Emacs's K&R style).  The caller requires the body's `{` to share this
    physical line, so this never re-reads a declarator the multi-line rules already own.
    """
    pos = tokens = 0
    while True:
        m = _TYPED_TOKEN.match(line, pos)
        if m is None:
            return None
        pos, tokens = m.end(), tokens + 1
        q = pos
        while q < len(line) and line[q] in " \t":
            q += 1
        if q < len(line) and line[q] == "(":
            return (m.group("word"), q) if tokens >= 2 and m.group("word") else None
        if pos >= len(line) or line[pos] not in " \t" or tokens > 12:
            return None


DEFUN = re.compile(r'^DEFUN\s*\(\s*"(?P<lisp>[^"]+)"\s*,\s*(?P<cname>\w+)\s*,\s*'
                   r'(?P<sname>\w+)\s*,\s*(?P<min>[^,]+?)\s*,\s*(?P<max>[^,]+?)\s*,\s*'
                   r'(?P<n>[^,]+?)\s*,\s*doc\s*:\s*/\*(?P<doc>.*?)\*/', re.S | re.M)
ATTRIBUTES = re.compile(r"^[^)]*?attributes\s*:\s*(?P<list>[A-Za-z_ ]*)", re.S)
DEFINE = re.compile(r"^#\s*define\s+(?P<name>[A-Za-z_]\w*)(?P<args>\([^)]*\))?\s*(?P<body>.*)$")
# `typedef struct X { … } Y;` and `typedef struct X\n{ … } Y;` are as common in this tree as
# the untagged forms, and without the optional `typedef` the tagged ones were invisible: every
# field of `src/haikugui.h`'s `haiku_char_struct` was carried by the mirror and never compared
# (status 3.146 - the same class of blind spot as 3.143's file-scope objects).  The trailing
# alias the closing line names (`} Y;`) is not a field and is not collected.
STRUCT_OPEN_INLINE = re.compile(r"^(?P<td>typedef\s+)?(?P<kind>struct|union)\s+(?P<name>[A-Za-z_]\w*)?\s*\{")
STRUCT_OPEN_NAME = re.compile(r"^(?P<td>typedef\s+)?(?P<kind>struct|union)(?:\s+(?P<name>[A-Za-z_]\w*))?\s*$")
ENUM_OPEN = re.compile(r"^enum\s+(?P<name>[A-Za-z_]\w*)?\s*\{")
TYPE_WORDS = {
    "static", "extern", "const", "volatile", "inline", "INLINE", "register",
    "struct", "union", "enum", "unsigned", "signed", "long", "short", "int",
    "char", "float", "double", "void", "bool", "size_t", "ptrdiff_t", "bool_bf",
    "Lisp_Object", "EMACS_INT", "EMACS_UINT", "Lisp_Word", "specpdl_ref",
}
# A declarator that is nothing but the function's name, at column 0, with its parameter
# list on the next line (src/eval.c 2465's `load_with_autoload_queue`).
BARE_NAME = re.compile(r"^[A-Za-z_]\w*$")

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
        # A view with character/string literals blanked too: a brace inside one (`case '{':`,
        # json.c 1565) is not a brace, and counting it made the span run to EOF and swallowed
        # every function after it.  blank_literals preserves length and line count, so the
        # row indices still line up with self.rows.
        self.scan_rows = blank_literals(self.clean).split("\n")

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


# The frozen configuration's macro table (`docs/baseline/config/*.macros.tsv`, written by
# tools/gen-cond-macros.py).  When it is loaded, `#if` conditions are evaluated against it, so
# the inventory sees the text the frozen configuration compiles and not the platform
# branches (PORTING.md P-08/P-66).  Without it the behaviour is the old one: only literal
# `#if 0` is dead and every other condition leaves both branches live.
# `(line_index, kind, expression, verdict)` for the conditions of the last
# `dead_branch_map` call — read by tools/check-cond-eval.py.
CONDITION_LOG: list = []

# Names this file defines through its own `#ifndef NAME` default, in the order the scan met
# them.  `dead_branch_map` uses them for a second pass (status 3.76): the frozen table's value
# for such a name *is* that define's net effect (why the source-header row exists), so applying
# it from line 0 made the guard itself answer false where clang answers true.
DEFAULT_DEFINES: list = []
CONDITION_MACROS: dict[str, str] | None = None
CONDITION_MACROS_BASE: dict[str, str] | None = None
CONDITION_EVAL_CACHE: dict[str, bool | None] = {}


def set_condition_macros(macros: dict[str, str] | None) -> None:
    """Install the frozen configuration's macro table (name -> value or `undef`)."""
    global CONDITION_MACROS, CONDITION_MACROS_BASE, CONDITION_EVAL_CACHE
    CONDITION_MACROS = macros
    CONDITION_MACROS_BASE = macros
    CONDITION_EVAL_CACHE = {}


# Which reference files may consult that table.  `tools/check-cond-eval.py` compares the
# evaluator's branch verdicts with clang's for every condition in a header; only headers it
# reports zero disagreements for are listed in `docs/baseline/config/cond-eval-verified.txt`,
# and the table is used for those and nothing else.  The reason is gnulib's `*.in.h`
# **templates**: their conditions name `@PLACEHOLDER@`s and the *generated* header's state
# (`#if @WINDOWS_STAT_INODES@ == 2`), so the frozen table cannot decide them and an
# evaluator that answers wrongly is worse than one that answers `unknown` (P-66).
# `(reference root, {absolute path})`: the match is *identity* after normalisation, not a
# suffix.  A suffix match let `test/manual/etags/c-src/emacs/src/lisp.h` pass for
# `src/lisp.h` (review A's F5); no such file is paired today, but the list is the thing
# standing between a wrong branch verdict and the gate, so it has to be exact.
CONDITION_MACROS_PATHS: tuple | None = None

# True while `_BlindConditions` has dropped the macro table for an unverified file.  A scanner
# that wants to trust the liveness map (only one does: the `static`-declaration rule in
# `function_spans`) has to ask, because in blind mode a dead branch looks live and a prototype
# inside `#if _LIBC` would make a definition look internal that the reference object exports
# (docs/status.md 3.67, lib/tempname.c).
BLINDED: bool = False


def set_condition_paths(paths, reference=None) -> None:
    """Restrict the macro table to these reference paths (None = every file)."""
    global CONDITION_MACROS_PATHS
    if paths is None:
        CONDITION_MACROS_PATHS = None
        return
    root = None if reference is None else os.path.abspath(str(reference))
    entries = set()
    for entry in paths:
        text = str(entry)
        entries.add(os.path.abspath(text) if os.path.isabs(text)
                    else os.path.abspath(os.path.join(root or ".", text)))
    CONDITION_MACROS_PATHS = (root, entries)


def blind_conditions(path):
    """The blind scope for one reference path (`tools/extract-abi.py` uses it too)."""
    return _BlindConditions(path)


class _BlindConditions:
    """Context manager: drop the macro table unless `path` is a verified file.

    Both globals go: `dead_branch_map` reads `CONDITION_MACROS_BASE` and re-installs it
    through `set_condition_macros`, so clearing `CONDITION_MACROS` alone left the table in
    force and made this whole scope a no-op (review A's F1 — the fix for it is the pair of
    assignments below, not a documentation change).
    """

    def __init__(self, path) -> None:
        self.path = None if path is None else os.path.abspath(str(path))
        self.saved = None
        self.saved_base = None
        self.saved_blinded = False
        self.blind = False

    def __enter__(self):
        global CONDITION_MACROS, CONDITION_MACROS_BASE, BLINDED
        if CONDITION_MACROS_PATHS is None or self.path is None:
            return self
        root, entries = CONDITION_MACROS_PATHS
        if root is not None and self.path in entries:
            return self
        self.saved, self.saved_base = CONDITION_MACROS, CONDITION_MACROS_BASE
        self.saved_blinded = BLINDED
        CONDITION_MACROS = CONDITION_MACROS_BASE = None
        BLINDED = True
        self.blind = True
        return self

    def __exit__(self, *_exc) -> bool:
        global CONDITION_MACROS, CONDITION_MACROS_BASE, BLINDED
        if self.blind:
            CONDITION_MACROS, CONDITION_MACROS_BASE = self.saved, self.saved_base
            BLINDED = self.saved_blinded
        return False


def read_condition_macros(path) -> dict[str, str]:
    macros: dict[str, str] = {}
    for line in Path(path).read_text(encoding="utf-8").split("\n"):
        if not line or line.startswith("#"):
            continue
        name, _, value = line.partition("\t")
        if name:
            macros[name] = value
    return macros


_COND_TOKENS = re.compile(
    r"\s*(defined|&&|\|\||==|!=|<=|>=|[-+*/%<>&|^!~(),?:]|\d+|[A-Za-z_]\w*)")


def _cond_tokens(text: str) -> list[str]:
    out, rest = [], text
    while rest.strip():
        m = _COND_TOKENS.match(rest)
        if not m:
            return out + ["?"]          # unparsable tail: the caller sees "unknown"
        out.append(m.group(1))
        rest = rest[m.end():]
    return out


# C's "blue paint": a macro that is being expanded is not expanded again, and an identifier
# that survives expansion in `#if` is 0.  This is what decides `USE_LSB_TAG`:
# `# define USE_LSB_TAG (VAL_MAX / 2 < INTPTR_MAX)` and `# define VAL_MAX (USE_LSB_TAG ? … )`
# are mutually recursive, so C computes `0 / 2 < INTPTR_MAX` = true — and `GCALIGNMENT`
# follows.  Without the rule the evaluator answered "unknown", kept both branches live, and
# `macros()`'s last-definition-wins picked the wrong `GCALIGNMENT`.
_COND_EXPANDING: set[str] = set()


def _cond_macro_params(value: str) -> tuple[list[str], str] | None:
    """`(a, b) body` for a function-like macro's frozen value, else None."""
    if not value.startswith("("):
        return None
    close = value.find(")")
    if close < 0:
        return None
    params = [p.strip() for p in value[1:close].split(",") if p.strip()]
    if not params:
        return None
    return params, value[close + 1:].strip()


def _cond_call_args(tokens: list[str]) -> tuple[list[str] | None, list[str]]:
    """The comma-separated arguments of a call whose `(` has been consumed."""
    args, current, nesting = [], [], 0
    for index, token in enumerate(tokens):
        if token == "(":
            nesting += 1
        elif token == ")":
            if nesting == 0:
                args.append(" ".join(current))
                return args, tokens[index + 1:]
            nesting -= 1
        if token == "," and nesting == 0:
            args.append(" ".join(current))
            current = []
            continue
        current.append(token)
    return None, []


def _cond_expand_call(name: str, args: list[str], depth: int) -> int | None:
    """Expand `NAME (args…)` textually, the way the preprocessor would."""
    macros = CONDITION_MACROS or {}
    parsed = _cond_macro_params(macros.get(name, ""))
    if parsed is None:
        return None
    params, body = parsed
    if len(params) != len(args):
        return None
    for param, arg in zip(params, args):
        body = re.sub(rf"\b{re.escape(param)}\b", f"({arg})", body)
    tokens = _cond_tokens(body)
    if not tokens or "?" in tokens:
        return None
    if name in _COND_EXPANDING:
        return None
    _COND_EXPANDING.add(name)
    try:
        return _cond_parse(tokens, depth + 1)[0]
    finally:
        _COND_EXPANDING.discard(name)


def _cond_ident(name: str, depth: int = 0) -> int | None:
    """C's rule: an undefined identifier is 0; a macro's value is itself evaluated."""
    macros = CONDITION_MACROS or {}
    # C23 (and the frozen build's <stdbool.h>) makes these keywords, not macros: `#if true`
    # is 1.  Without this a mirror of `# define WRAP_SPECPDL_REF true` evaluated to 0 and the
    # gate kept both branches of lisp.h's `#ifdef WRAP_SPECPDL_REF` live.
    if name == "true":
        return 1
    if name == "false":
        return 0
    if name not in macros or macros[name] == "undef":
        return 0
    if name in _COND_EXPANDING:
        return 0
    value = macros[name].strip()
    if value == "":
        return 1
    try:
        return int(value, 0)
    except ValueError:
        pass
    if depth > 8:
        return None
    tokens = _cond_tokens(value)
    if not tokens:
        return None
    # A macro whose value is a conditional expression (`HAVE_NATIVE_TIME_Z` in strftime.c
    # 434) is expanded like any other: `_cond_parse` evaluates only the taken branch, so
    # the old `"?" in tokens` bail-out made every `#if HAVE_NATIVE_TIME_Z` unknown.
    _COND_EXPANDING.add(name)
    try:
        result = _cond_parse(tokens, depth + 1)[0]
    finally:
        _COND_EXPANDING.discard(name)
    return result


def _cond_parse(tokens: list[str], depth: int = 0) -> tuple[int | None, list[str]]:
    """The condition grammar's entry point: a conditional expression.

    `?:` is needed because Emacs defines macros that way — `VAL_MAX` is
    `(USE_LSB_TAG ? - (1 << GCTYPEBITS) : VAL_MAX)` (lisp.h 262), and that mutually recursive
    pair decides `GCALIGNMENT` (see `_COND_EXPANDING`).  Only the taken branch is evaluated,
    as in C.
    """
    value, rest = _cond_parse_or(tokens, depth)
    if rest and rest[0] == "?":
        if value is None:
            return None, []
        # Only the taken side is parsed: `VAL_MAX`'s untaken half is
        # `- (1 << GCTYPEBITS)` and the evaluator has no shift operator, which would make it
        # answer "unknown" for a macro C evaluates fine.  The `:` is found by depth so a
        # nested conditional does not confuse the scan.
        nesting, colon = 0, None
        for k, token in enumerate(rest[1:], start=1):
            if token in ("(", "?"):
                nesting += 1
            elif token in (")", ":"):
                if token == ")" and nesting == 0:
                    break
                if token == ":" and nesting == 0:
                    colon = k
                    break
                nesting -= 1
        if colon is None:
            return None, []
        tail = rest[colon + 1:]
        if value:
            taken, _ = _cond_parse_or(rest[1:colon], depth)
            return taken, tail
        return _cond_parse(tail, depth)
    return value, rest


def _cond_parse_cond(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    """C's `?:`: `strftime.c` 434 defines `HAVE_NATIVE_TIME_Z` as a ternary over three
    configuration macros, and `USE_C_LOCALE && HAVE_STRFTIME_L ? … : …` decides whether the
    file's `#if HAVE_NATIVE_TIME_Z` arms are live.  Only the *taken* branch decides the
    value; an unknown condition yields a value only when both branches agree."""
    cond, rest = _cond_parse_or(tokens, depth)
    if not (rest and rest[0] == "?"):
        return cond, rest
    then_value, rest2 = _cond_parse_cond(rest[1:], depth)
    if not (rest2 and rest2[0] == ":"):
        return None, []
    else_value, rest3 = _cond_parse_cond(rest2[1:], depth)
    if cond is not None:
        return (then_value if cond else else_value), rest3
    if then_value is not None and then_value == else_value:
        return then_value, rest3
    return None, rest3


def _cond_parse_or(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    left, rest = _cond_parse_and(tokens, depth)
    while rest and rest[0] == "||":
        right, rest2 = _cond_parse_and(rest[1:], depth)
        if left is True or right is True:
            left = True
        elif left is None or right is None:
            left = None
        else:
            left = bool(left) or bool(right)
        rest = rest2
    return left, rest


def _cond_parse_and(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    left, rest = _cond_parse_cmp(tokens, depth)
    while rest and rest[0] == "&&":
        right, rest2 = _cond_parse_cmp(rest[1:], depth)
        if left is False or right is False:
            left = False
        elif left is None or right is None:
            left = None
        else:
            left = bool(left) and bool(right)
        rest = rest2
    return left, rest


def _cond_parse_cmp(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    left, rest = _cond_parse_shift(tokens, depth)
    while rest and rest[0] in ("==", "!=", "<", ">", "<=", ">=") \
            and not (len(rest) >= 2 and rest[1] == rest[0] and rest[0] in ("<", ">")):
        op = rest[0]
        right, rest2 = _cond_parse_shift(rest[1:], depth)
        if left is None or right is None:
            left = None
        else:
            left = bool(eval(f"left {op} right", {"left": left, "right": right}))  # noqa: S307
        rest = rest2
    return left, rest


def _cond_parse_shift(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    """`<<` and `>>`: `#if GCALIGNMENT != 1 << GCTYPEBITS` (lisp.h 276) needs them."""
    left, rest = _cond_parse_add(tokens, depth)
    while len(rest) >= 2 and rest[0] == rest[1] and rest[0] in ("<", ">"):
        op = rest[0] * 2
        right, rest2 = _cond_parse_add(rest[2:], depth)
        if left is None or right is None or right < 0:
            left = None
        else:
            left = (left << right) if op == "<<" else (left >> right)
        rest = rest2
    return left, rest


def _cond_parse_add(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    left, rest = _cond_parse_mul(tokens, depth)
    while rest and rest[0] in ("+", "-"):
        op = rest[0]
        right, rest2 = _cond_parse_mul(rest[1:], depth)
        if left is None or right is None:
            left = None
        else:
            left = left + right if op == "+" else left - right
        rest = rest2
    return left, rest


def _cond_parse_mul(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    """`/` and `%` matter: `USE_LSB_TAG` is `(VAL_MAX / 2 < INTPTR_MAX)` (lisp.h 256)."""
    left, rest = _cond_parse_unary(tokens, depth)
    while rest and rest[0] in ("*", "/", "%"):
        op = rest[0]
        right, rest2 = _cond_parse_unary(rest[1:], depth)
        if left is None or right is None or (op in ("/", "%") and right == 0):
            left = None
        elif op == "*":
            left = left * right
        elif op == "/":
            left = int(left / right) if left * right >= 0 else -int(abs(left) / abs(right))
        else:
            left = left - right * int(left / right)
        rest = rest2
    return left, rest


def _cond_parse_unary(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    if tokens and tokens[0] == "!":
        value, rest = _cond_parse_unary(tokens[1:], depth)
        return (None if value is None else int(not value)), rest
    if tokens and tokens[0] in ("-", "+"):
        # C writes a negative constant as `(-1)` (`# define FAILURE (-1)`,
        # strftime.c 229) and `grep`-style conditions compare it with `==`; without the
        # unary operators the evaluator answered "unknown" and the file could not join
        # the verified list.
        sign, rest = tokens[0], tokens[1:]
        value, rest2 = _cond_parse_unary(rest, depth)
        if value is None:
            return None, rest2
        return (-value if sign == "-" else value), rest2
    return _cond_parse_primary(tokens, depth)


def _cond_int_value(text: str) -> int | None:
    """A C integer constant: decimal, `0x` hex or `0`-prefixed octal, plus suffixes.

    `int (text, 0)` is Python's rule and rejects C's octal (`0004000` in gnulib's
    `S_ISUID`), which crashed the evaluator instead of answering `None` (review A's F4);
    a crashed evaluator cannot even be measured by `tools/check-cond-eval.py`.
    """
    body = re.sub(r"[uUlL]+$", "", text)
    try:
        if body.lower().startswith("0x"):
            return int(body, 16)
        if len(body) > 1 and body[0] == "0":
            return int(body, 8)          # C's `0`-prefixed octal
        return int(body, 10)
    except ValueError:
        return None


def _cond_parse_primary(tokens: list[str], depth: int) -> tuple[int | None, list[str]]:
    if not tokens:
        return None, []
    head, rest = tokens[0], tokens[1:]
    if head == "(":
        value, rest2 = _cond_parse(rest, depth)
        if rest2 and rest2[0] == ")":
            return value, rest2[1:]
        return None, []
    if head == "defined":
        if rest and rest[0] == "(":
            if len(rest) >= 3 and rest[2] == ")":
                return _cond_ident_value(rest[1]), rest[3:]
            return None, []
        if rest:
            return _cond_ident_value(rest[0]), rest[1:]
        return None, []
    if head[0].isdigit():
        return _cond_int_value(head), rest
    if head == "?":
        return None, []
    # A function-like macro's call: `GNUC_PREREQ (4, 3, 2)` (lisp.h 109/5731).
    macros = CONDITION_MACROS or {}
    if rest and rest[0] == "(" and _cond_macro_params(macros.get(head, "")) is not None:
        args, rest2 = _cond_call_args(rest[1:])
        if args is None:
            return None, []
        return _cond_expand_call(head, args, depth), rest2
    return _cond_ident(head, depth), rest


def _cond_ident_value(name: str) -> int:
    macros = CONDITION_MACROS or {}
    return int(name in macros and macros[name] != "undef")


def eval_condition(text: str) -> bool | None:
    """C's `#if` condition TEXT against the frozen configuration: True, False or None."""
    if CONDITION_MACROS is None:
        return None
    text = re.sub(r"/\*.*?\*/", " ", text)
    text = text.strip()
    if text in ("0", "false", "FALSE"):
        return False
    if text in ("1", "true", "TRUE"):
        return True
    if text in CONDITION_EVAL_CACHE:
        return CONDITION_EVAL_CACHE[text]
    tokens = _cond_tokens(text)
    value: bool | None = None
    if tokens:
        result, rest = _cond_parse_cond(tokens, 0)
        if result is not None and not rest:
            value = bool(result)
    CONDITION_EVAL_CACHE[text] = value
    return value


def logical_lines(text: str) -> list[tuple[str, int]]:
    """TEXT's logical lines with the physical lines they span.

    A C directive (and a macro definition) may continue with backslashes; the display
    headers' `#if`s wrap onto two or three lines, and the whole condition is what has to be
    evaluated.  Returns `(joined_text, physical_line_count)` per logical line, in order.
    """
    out: list[tuple[str, int]] = []
    current, count = None, 0
    for line in text.split("\n"):
        count += 1
        current = line if current is None else current + " " + line.strip()
        if current.rstrip().endswith("\\"):
            current = current.rstrip()[:-1]
            continue
        out.append((current, count))
        current, count = None, 0
    if current is not None:
        out.append((current, count))
    return out


def source_header_macros() -> dict[str, set[str]]:
    """`{header name: macros that header defines itself}`, from the measured row list.

    `docs/baseline/config/source-header-macros.tsv` exists for macros a *source header* brings
    in where config.h cannot decide them (PORTING.md P-66's status 3.35 rule).  For the header
    that owns such a row, the macro's table value is the *net* effect of that header's own
    define - so a condition the header writes *before* it (`font.h` 997's `#ifndef FONT_DEBUG`)
    must be answered as "not yet defined" (status 3.76).
    """
    global _SOURCE_HEADER_MACROS
    if _SOURCE_HEADER_MACROS is None:
        rows: dict[str, set[str]] = {}
        path = (Path(__file__).resolve().parent.parent
                / "docs/baseline/config/source-header-macros.tsv")
        if path.is_file():
            for raw in path.read_text(encoding="utf-8").splitlines():
                if not raw.strip() or raw.startswith("#") or "\t" not in raw:
                    continue
                header, macro = raw.split("\t", 1)
                rows.setdefault(header.strip(), set()).add(macro.strip())
        _SOURCE_HEADER_MACROS = rows
    return _SOURCE_HEADER_MACROS


_SOURCE_HEADER_MACROS: dict[str, set[str]] | None = None


def dead_branch_map(text: str, path: str | None = None) -> list[bool]:
    """True for lines that are not inside a branch the frozen configuration disables.

    Literal-false conditions (`#if 0` / `#if false`) are always dead.  With the frozen
    configuration's macro table loaded (`set_condition_macros`), the conditions Emacs
    actually writes are evaluated against it, so `#ifdef HAVE_X_WINDOWS`'s body is dead for
    the minimal macOS build instead of being demanded of the mirror (P-08/P-66).  A condition
    the evaluator does not understand (`eval_condition` returns None) leaves both branches
    live — the conservative behaviour the port had before — and `#elif`/`#else` are only
    assumed dead once a sibling branch was taken.
    """
    live: list[bool] = []
    # Each entry: [outer_dead, a_branch_was_taken, this_branch_is_live]
    stack: list[list[bool]] = []
    # C's own model: a `#define` in this file applies to the conditions after it, so the
    # table is copied per file and the copy is what the evaluator sees while this file is
    # parsed (`USE_LSB_TAG` is defined by lisp.h itself, at lisp.h 254/256, and decides
    # `GCALIGNMENT` 268-281).  The previous table is restored on the way out.
    base = CONDITION_MACROS_BASE
    local_macros = dict(base) if base is not None else None
    if local_macros is not None:
        set_condition_macros(local_macros)
    own = set()
    if path:
        own = source_header_macros().get(Path(path).name, set())
    try:
        live = _dead_branch_scan(text, local_macros, own)
        # Second pass, only when this file defines - through its own `#ifndef NAME` default - a
        # name the frozen table also carries: start that name out undefined, so the guard
        # answers what clang answers, and the file's own `#define` supplies the value from
        # there on.
        deferred = [name for name in DEFAULT_DEFINES
                    if base is not None and base.get(name, "undef") != "undef"]
        if deferred:
            # A *fresh* copy: the first scan has already written the file's own defines into
            # `local_macros`, and starting pass two from those would apply every one of them
            # from line 0 (which un-defined half the header's guards).
            local_macros = dict(base)
            for name in deferred:
                local_macros[name] = "undef"
            set_condition_macros(local_macros)
            live = _dead_branch_scan(text, local_macros, own)
        return live
    finally:
        if base is not None:
            set_condition_macros(base)


def _dead_branch_scan(text: str, local_macros: dict[str, str] | None,
                      own: set[str] | None = None) -> list[bool]:
    """The `dead_branch_map` state machine, with the working macro table installed.

    Every condition it meets is recorded in `CONDITION_LOG` as
    `(line_index, kind, expression, verdict)` — the verdict being what this evaluator answers
    *with the file-local macro state of that line*, which is what
    `tools/check-cond-eval.py` compares against the C preprocessor (PORTING.md P-66).
    """
    global CONDITION_LOG, DEFAULT_DEFINES
    CONDITION_LOG = []
    DEFAULT_DEFINES = []
    live: list[bool] = []
    stack: list[list[bool]] = []
    line_index = 0
    for logical, physical in logical_lines(text):
        outer_dead = any(frame[0] or not frame[2] for frame in stack)
        dead = outer_dead
        s = logical.strip()
        if s.startswith("#"):
            body = s[1:].strip()
            word = body.split(maxsplit=1)[0] if body else ""
            rest = body[len(word):].strip()
            # C removes a comment in translation phase 3, before a directive is read, so a
            # comment this *logical line* leaves open ends what the directive says: the rest
            # of the comment sits on the following physical lines, which `logical_lines`
            # does not join into this one.  `src/menu.c` 1420's
            # `#ifdef HAVE_NTGUI /* W32 specific because ... */` is the tree's example: with
            # the comment text kept, the tokenizer met its `/` and the condition answered
            # `unknown`, so the branch stayed live although clang compiles it dead - the one
            # disagreement that kept that file out of cond-eval-verified.txt.  The log keeps
            # the unblanked `rest`, because that is the reference line's own text.
            cond_text = re.sub(r"/\*.*$", "", rest).rstrip()
            if word in ("if", "ifdef", "ifndef"):
                if outer_dead:
                    # A condition inside a branch the configuration already disabled has no
                    # effect: the nested branch is dead whatever it says, and that is what
                    # the C preprocessor reports (`regexec.c` 225's `SHLIB_COMPAT (…)` sits
                    # inside `#ifdef _LIBC`).  Evaluating it anyway made the file disagree
                    # with clang and kept it off the verified list (P-66).
                    cond = False
                elif word == "ifdef":
                    cond = eval_condition (f"defined ({cond_text})")
                elif word == "ifndef":
                    cond = eval_condition (f"! defined ({cond_text})")
                else:
                    cond = eval_condition (cond_text)
                if cond is None and word == "if" and rest in ("0", "false"):
                    cond = False
                CONDITION_LOG.append((line_index, word, rest, cond))
                branch = cond is not False
                stack.append([outer_dead, cond is True, branch])
                # A `#ifndef X` guard on a macro *this very header* is on the row list for
                # is the second pass's candidate: the table's value for X is that header's
                # own define's net effect, so it must not apply before this guard (font.h
                # 997-999's FONT_DEBUG; status 3.76).
                if word == "ifndef" and own and cond_text.strip() in own:
                    DEFAULT_DEFINES.append(cond_text.strip())
                dead = outer_dead or not branch
            elif word == "elif":
                if stack:
                    entry = stack[-1]
                    if entry[1]:
                        entry[2] = False
                    else:
                        cond = eval_condition (cond_text)
                        CONDITION_LOG.append((line_index, word, rest, cond))
                        entry[2] = cond is not False
                        entry[1] = cond is True
                    dead = entry[0] or not entry[2]
            elif word == "else":
                if stack:
                    entry = stack[-1]
                    if entry[1]:
                        entry[2] = False
                    else:
                        entry[2] = True
                        entry[1] = True
                    dead = entry[0] or not entry[2]
            elif word == "endif":
                if stack:
                    stack.pop()
                dead = any(frame[0] or not frame[2] for frame in stack)
            elif word == "define":
                # Only a define in a *live* branch applies: lisp.h 254's `# define
                # USE_LSB_TAG false` sits inside the `#if` at 251 that this configuration
                # makes false, and recording it gave `GCALIGNMENT` the wrong branch.
                # The stored name is the identifier alone, not `NAME (params)`: C's
                # `defined (NAME)` is true for a function-like macro too, and getloadavg.c
                # 190's `# define LDAV_CVT(n) …` has to answer 267's `#ifndef LDAV_CVT`.
                match = re.match (r"[A-Za-z_][A-Za-z0-9_]*", rest)
                name = match.group (0) if match else ""
                if not dead and name and local_macros is not None:
                    local_macros[name] = rest[len(name):].strip()
                    CONDITION_EVAL_CACHE.clear()

            elif word == "undef":
                # Same rule as `#define` above, and the C preprocessor's: the `#undef` in a
                # branch the configuration disabled never runs.  getloadavg.c 104-110 sits
                # in `#ifdef NeXT`/`#if defined __gnu_hurd__ || defined NeXT` (both false)
                # and does `#undef BSD`/`#undef FSCALE`; applying them un-defended the
                # file's own <sys/param.h> macros and made 20 conditions disagree with clang
                # (the whole nlist chain read as dead).
                name = rest.strip()
                if not dead and name and local_macros is not None:
                    local_macros[name] = "undef"
                    CONDITION_EVAL_CACHE.clear()
            # every other directive (`#include`, …) inherits the current state
        live.extend([not dead] * physical)
        line_index += physical
    return live


def strip_strings(line: str) -> str:
    return re.sub(r'"(?:[^"\\]|\\.)*"', '""', line)


def strip_char_literals(line: str) -> str:
    """LINE with C character literals blanked.

    A brace inside one (`image.c` has `'{'` and `'}'`) walks `brace_depth` off, which is how
    `ct_colors_allocated_max` was skipped (review A's F3 diagnosis).  The scan is bounded the
    way tools/extract-abi.py's is: a character literal is a few characters long, so an
    unmatched apostrophe must not swallow the rest of the line.
    """
    out, i, n = [], 0, len(line)
    while i < n:
        ch = line[i]
        if ch == "'":
            j = i + 1
            while j < n and j - i <= 8:
                if line[j] == "\\":
                    j += 2
                    continue
                if line[j] == "'":
                    break
                j += 1
            if j < n and j - i <= 8 and line[j] == "'":
                out.append(" " * (j - i + 1))
                i = j + 1
                continue
        out.append(ch)
        i += 1
    return "".join(out)


DOC_ESCAPE = {"t": "\t", "r": "\r", "n": "\n", "f": "\f", "b": "\b", "v": "\v",
              "a": "\x07", "\\": "\\"}


def doc_comment_value(comment: str) -> str:
    """The value make-docfile stores for a `doc: /* … */` comment.

    lib-src/make-docfile.c's read_c_string_or_comment drops the comment's leading and
    trailing whitespace; a backslash escapes, so `\n`/`\t`/… are the control characters
    and any other escaped character keeps just itself (C's `\"extra\"` is `"extra"`).
    Comparing the raw comment text instead hid trailing whitespace in nearly every
    docstring of the tree.
    """
    out, i = [], 0
    while i < len(comment):
        ch = comment[i]
        if ch == "\\" and i + 1 < len(comment):
            nxt = comment[i + 1]
            out.append(DOC_ESCAPE.get(nxt, nxt))
            i += 2
            continue
        out.append(ch)
        i += 1
    # `strip(" \t")` left a leading newline in place, so a `doc: /*` whose text starts on
    # the next line kept it: normalised to a leading space on this side but stripped on the
    # Rust side (`extract-abi.digest`), the two digests could never agree (character.c's
    # `Fstring` is the first `doc: /*\n…` subr in a ported file).  make-docfile drops the
    # comment's leading whitespace, which is what `.strip ()` does.
    return "".join(out).strip()


def blank_literals(text: str) -> str:
    """Replace string and character literals with spaces, preserving length and lines.

    `blank_comments` leaves literals alone, and a brace inside one would throw the
    depth count off: `printf ("}")` truncated a body and `char c = '{'` swallowed the
    rest of the file (review B's D1).  The corpus happens to contain no such DEFUN
    body, but the scan must not depend on that luck; `extract-abi.strip_char_literals`
    is the Rust-side counterpart of the character half.
    """
    out, i, n = list(text), 0, len(text)
    while i < n:
        ch = text[i]
        if ch == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"' or text[j] == "\n":
                    break
                j += 1
            for k in range(i + 1, min(j, n)):
                if out[k] != "\n":
                    out[k] = " "
            i = j + 1
            continue
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
                for k in range(i + 1, j):
                    if out[k] != "\n":
                        out[k] = " "
                i = j + 1
                continue
        i += 1
    return "".join(out)


def _matching(text: str, pos: int, open_ch: str, close_ch: str) -> int:
    """The index of the `close_ch` that matches the `open_ch` at POS, or len(text)."""
    depth = 0
    i = pos
    while i < len(text):
        ch = text[i]
        if ch == open_ch:
            depth += 1
        elif ch == close_ch:
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return len(text)


def _defun_heads(text: str) -> list[tuple[str, str, int]]:
    """`(lisp_name, c_name, index of the macro's '(')` for every `DEFUN (...)` head.

    Scanned instead of matched: a lisp name may contain an escaped quote
    (`"foo\\"bar"`), and the pattern this replaces (`"[^"]*"`) silently produced no
    span for such a DEFUN, so its body would have been compared by nothing at all --
    the failure mode is silence, not an error (review B's D2).  A name containing
    parentheses must not be mistaken for the macro's parameter list either, so the
    head is parsed character by character.
    """
    heads: list[tuple[str, str, int]] = []
    for m in re.finditer(r"^DEFUN\s*\(", text, re.M):
        open_paren = text.index("(", m.start())
        j = open_paren + 1
        while j < len(text) and text[j] in " \t":
            j += 1
        if j >= len(text) or text[j] != '"':
            continue
        k, chars = j + 1, []
        while k < len(text) and text[k] != "\n":
            if text[k] == "\\" and k + 1 < len(text):
                chars.append(text[k:k + 2])
                k += 2
                continue
            if text[k] == '"':
                break
            chars.append(text[k])
            k += 1
        k += 1
        # The C name may sit on the next line: Emacs's own style puts
        # `DEFUN ("name",\n       Fname, Sname, ...)` (treesit.c, xwidget.c, process.c,
        # w32fns.c, xfaces.c, ...).  Skipping only ", \t" silently dropped every such
        # DEFUN from the body comparison, which is exactly the blind spot P-78 exists to
        # close (review B, round 9: 1708 of 1849 DEFUN bodies were reachable).
        while k < len(text) and text[k] in ", \t\r\n":
            k += 1
        nm = re.match(r"[A-Za-z_]\w*", text[k:])
        if not nm:
            continue
        heads.append(("".join(chars), nm.group(0), open_paren))
    return heads


def defun_spans(text: str) -> list[dict]:
    """Every `DEFUN (...)` subr, with the body span its `fnname` defines.

    `function_spans` skips these on purpose: the C text spells the *subr object*
    (`Sname`) at the position a C parser expects the name, and the function's own
    name is only the macro's second argument, so a body lookup by name never
    found one -- which left the body of every subr uncompared (review of
    data.c, P-78).  The body is the braced block after the parameter list that
    follows the macro invocation.
    """
    scan = blank_literals(blank_comments(text))
    spans: list[dict] = []
    for lisp_name, name, macro_open in _defun_heads(scan):
        macro_end = _matching(scan, macro_open, "(", ")")
        open_paren = scan.find("(", macro_end + 1)
        if open_paren == -1:
            continue
        args_end = _matching(scan, open_paren, "(", ")")
        brace = scan.find("{", args_end)
        if brace == -1:
            continue
        body_end = _matching(scan, brace, "{", "}")
        spans.append({"name": name, "lisp_name": lisp_name,
                      "body_start": text.count("\n", 0, brace) + 1,
                      "body_end": text.count("\n", 0, body_end) + 1})
    return spans


def function_spans(text: str) -> list[dict]:
    """Every function definition, with the body span it occupies.

    A definition is `NAME (...)` at column 0 whose parameter list is followed by
    `{` (on the same line or a later one, as Emacs writes it).  Declarations end
    in `;` and macro invocations such as `DEFUN (...)` are followed by another
    argument list, so neither is reported.
    """
    lines = Lines(text)
    rows = lines.rows
    # C 6.2.2p4: a prior `static` declaration gives the later definition internal linkage, which
    # is how emacs.c's `static void syms_of_emacs (void);` (268) governs its definition (3477) - a
    # definition written without the keyword.  Without this the flag said "extern", and the
    # checker demanded a `#[unsafe(no_mangle)]` the reference object does not have (status 3.67).
    # Only *live* lines count: lib/tempname.c declares `static int try_tempname_len (…);` at 123
    # inside an arm the frozen configuration does not compile, and its definition is the object's
    # exported `_try_tempname_len` (measured) - reading the raw text made that a false internal.
    # ... and only where the map is trustworthy: with the table dropped (an unverified file)
    # every line looks live, so a prototype in a dead arm would wrongly make a definition
    # internal - `lib/tempname.c`'s `#if _LIBC` prototype against its exported
    # `_try_tempname_len` (docs/status.md 3.67).  The specifier window below carries the rest.
    static_prior: set[str] = set()
    if not BLINDED:
        live_text = "\n".join(line for _, line, live in lines if live)
        static_prior = set(
            re.findall(r"\bstatic\b[^;(){}]*?\b(\w+)\s*\([^;{}]*?\)\s*;", live_text))
    spans: list[dict] = []
    i = 1
    while i <= len(rows):
        line, live = rows[i - 1], lines.live[i - 1]
        if not live or line[:1] in (" ", "\t", "#", ""):
            i += 1
            continue
        m = CFUNC.match(line)
        typed = False
        if m is not None and (m.group("name") or m.group("pname")) not in STATEMENT_WORDS:
            gd = m.groupdict()
            name = m.group("name") if gd.get("name") is not None else m.group("pname")
            k = m.end() - 1
        else:
            # The one-line form: declaration specifiers, name and `(` share the line.
            hit = typed_def_name(line)
            typed = True
            if hit is None or hit[0] in STATEMENT_WORDS:
                # A definition whose *name* has a line of its own, with the parameter list on
                # the next one: src/eval.c 2464-2467 writes
                #     Lisp_Object
                #     load_with_autoload_queue
                #       (Lisp_Object file, …)
                #     {
                # (Emacs's layout for a long name).  Neither CFUNC nor `typed_def_name` keys
                # it: the name line has no `(`, and the parameter line starts with one.  The
                # scanner treats the name as the declarator and lets the walk below consume
                # the parameter list, which is also what makes the `{`-on-the-next-line rule
                # just after apply.  Measured over every `*.c` of the reference tree:
                # src/eval.c is the only file that gains a name this way.
                if not BARE_NAME.match(line.strip()) or line.strip() in STATEMENT_WORDS:
                    i += 1
                    continue
                jj = i
                while jj < len(rows):
                    if lines.live[jj] and rows[jj].strip() and not rows[jj].strip().startswith("#"):
                        break
                    jj += 1
                if jj >= len(rows) or not rows[jj].lstrip().startswith("("):
                    i += 1
                    continue
                name, k = line.strip(), len(line)
            else:
                name, k = hit
        j = i - 1
        # walk the parameter list to its matching close paren
        depth = 0
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
            # A declarator whose parameter list ends its line takes the body from the next
            # live, non-blank, non-preprocessor line - whether or not the declaration
            # specifiers share that line.  `Lisp_Object backtrace_top_function (void)` (eval.c
            # 4415) is the typed case that used to be dropped here ("a typed declarator is a
            # definition only when `{` shares its line"), and losing it meant the file's
            # `#[unsafe(no_mangle)]` on the definition read as an invented exported symbol
            # while nothing checked the body.  Measured over every `*.c` of the reference tree
            # (test/ excluded): the verdict gains a name for eval.c, buffer.c, fns.c,
            # keyboard.c, minibuf.c, window.c, w32fns.c and oldXMenu/XLookAssoc.c, and every
            # one of them is either not ported, not built on this target, or already carries
            # that function.
            for jj in range(j + 1, min(j + 9, len(rows))):
                # A line the frozen configuration does not compile is not part of the
                # declarator: src/emacs.c's `main` is written as
                #   #if !defined HAVE_ANDROID || defined ANDROID_STUBIFY
                #   int
                #   main (int argc, char **argv)
                #   #else
                #   int
                #   android_emacs_init (int argc, char **argv, char *dump_file)
                #   #endif
                #   {
                # and the dead arm's `int` between the live declarator and the body used to
                # stop this lookahead, so the file's entry point was not a function of the
                # file at all: `#[unsafe(no_mangle)]` on its definition was reported as an
                # invented exported symbol, and nothing checked the body.  Measured over
                # every `*.c` of the reference tree (test/ excluded): the verdict changes
                # for emacs.c only.
                if not lines.live[jj]:
                    continue
                nxt = rows[jj].strip()
                if not nxt:
                    continue
                # A preprocessor line between the declarator and the body has no effect on
                # it: gnulib writes `# undef nanosleep` there (nanosleep.c 44, so that the
                # body's `nanosleep (&intermediate, …)` is the *system* call), and stopping
                # at it lost the file's only live function - the checker then reported the
                # port's `nanosleep` as an invented exported symbol.
                if nxt.startswith("#"):
                    continue
                if nxt.startswith("{"):
                    body_line = jj
                break
        if body_line is None:
            i += 1
            continue
        depth2, end_line = 0, None
        for jj in range(body_line, len(rows)):
            row = lines.scan_rows[jj]
            depth2 += row.count("{") - row.count("}")
            if depth2 <= 0 and jj > body_line:
                end_line = jj
                break
            if depth2 <= 0 and "}" in row:
                end_line = jj
                break
        if end_line is None:
            end_line = len(rows)
        # The typed form carries its own specifiers on the declarator line, so that line is
        # what says whether the definition is `static`.
        prev = line if typed else ""
        # The specifiers are a *window*, not the single previous line: sysdep.c writes
        #     static
        #     #endif
        #     int
        #     renameat_noreplace (int srcfd, …)
        # so the keyword sits three lines above the declarator, and stopping at `int` lost it -
        # the port then gave an internal function an external name, where the reference object
        # exports none (status 3.67).  Walk over blank-free specifier lines (a preprocessor
        # directive, an attribute, a bare type or qualifier) and take `static` from any of them.
        # An attribute on its own line was already handled here (regex_internal.c 40-46's
        # `static reg_errcode_t __attribute_warn_unused_result__ name (…)`).
        # The window starts with the declarator's own line: a one-line definition carries its
        # specifiers there (`static void set_random_seed (…) { … }`), which the walk above the
        # declarator would never see.
        spec_lines: list[str] = [line]
        prev = line if typed else ""
        saw_directive = False
        p = i - 2
        while p >= 0 and len(spec_lines) < 7:
            cand = rows[p].strip()
            if not cand:
                break
            if cand.startswith("#"):
                saw_directive = True
                p -= 1
                continue
            if re.match(r"^(?:__)?(?:attribute|_GL_ATTRIBUTE)|^ATTRIBUTE_", cand):
                p -= 1
                continue
            if not prev:
                prev = cand
            spec_lines.append(cand)
            if "(" in cand or ")" in cand or ";" in cand or "{" in cand or "," in cand:
                break
            p -= 1
        spec = " ".join(spec_lines)
        is_static = bool(re.search(r"\bstatic\b", spec)) or name in static_prior
        # When the *preprocessor* decides the linkage, the text cannot: lib/tempname.c writes
        #     #ifdef _LIBC
        #     static
        #     #endif
        #     int
        #     try_tempname_len (…)
        # and `_LIBC` is undefined here, so the definition is external and the reference object
        # exports `_try_tempname_len` - while the same shape in sysdep.c (`#ifndef HAVE_ANDROID`,
        # live) really is internal.  Neither reading is safe from the source alone, so the span
        # says the linkage is *conditional* and the checker leaves it to the object-level gates
        # (`tools/check-symbols.py`/`check-linkage.py`), which compare against the frozen object
        # (docs/status.md 3.67).
        conditional = is_static and (saw_directive or name in static_prior)
        spans.append({"name": name, "line": i,
                      "static": is_static,
                      "conditional_static": conditional,
                      "return_type": prev, "body_start": body_line + 1,
                      "body_end": end_line + 1})
        # `max`: a typed definition's body sits on the declarator's own line, so its
        # `end_line` is `i - 1` and `end_line + 1` would not advance (the scanner looped
        # forever on openat-die.c until this was written as a max).
        i = max (end_line + 1, i + 1)
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


def integer_literal(text: str) -> int | None:
    """The value of a C integer literal (decimal or hex, with u/U/l/L suffixes)."""
    t = re.sub(r"[uUlL]+$", "", text.strip())
    try:
        return int(t, 0) if t.lower().startswith(("0x", "-0x")) or re.fullmatch(r"-?\d+", t) \
            else None
    except ValueError:
        return None


def enum_constants(text: str) -> dict[str, str]:
    """Enum members that carry an explicit value: name -> value.

    Emacs writes both one-line enums (`enum { BEG = 1, BEG_BYTE = BEG };`) and
    multi-member lines, so this scans the comment-blanked text with brace matching
    instead of assuming one member per line (which silently extracted nothing from
    buffer.h, and left the enum's `}` unread).

    The scanner reads a flattened text, not one line at a time, because Emacs also
    writes the opening brace on the *next* line:

        enum char_bits
          {
            CHAR_ALT = 0x0400000,
            ...
            CHARACTERBITS = 22
          };

    A line-anchored scan (`^enum\s+NAME\s*\{`) misses that form entirely, which hid
    `enum char_bits` and `enum Lisp_Closure` (14 constants of lisp.h) from the
    constant gate — the hole `port(keymap.h)`'s `CHARACTERBITS` fell into.  The
    `\benum\b` search below matches the declaration wherever its `{` sits, and the
    brace matcher then reads the body across lines.

    Only *file-scope* enums are read: C also declares enums inside functions
    (doprnt.c 218-225's `pDlen`/`pIlen`/`pMlen`/`maxmlen`), whose members are locals
    the enclosing function's body-token check already covers.  The text it reads comes
    from `flat_code`, which drops preprocessor lines so that neither an `extern "C" {`
    nor a multi-line `#define`'s continuation can move the depth (F3/F4).
    """
    out: dict[str, str] = {}
    code = flat_code(text)
    # The keyword is followed by an optional tag and then the brace, possibly on the
    # next line — but *only* that: `enum foo bar (void) {` is a function definition,
    # and letting the search cross arbitrary text made it swallow the bodies after it.
    open_re = re.compile(r"\benum\b[ \t\r\n]*(?:[A-Za-z_]\w*)?[ \t\r\n]*\{")
    body: list[str] = []
    inside = False
    i = 0
    while i < len(code):
        if not inside:
            m = open_re.search(code, i)
            if not m:
                break
            if brace_depth(code, m.start()) != 0:
                i = m.end()
                continue      # an enum inside a function or struct body
            i = m.end()
            inside, body = True, []
            continue
        ch = code[i]
        if ch == "{":
            body.append("{")
        elif ch == "}":
            # Members without `=` take the predecessor's value plus one (C's rule).  They
            # used to be dropped, and that is how `draw_glyphs_face`'s four implicit
            # members (and `glyph_row_area`'s) could disagree with C with every gate green
            # (review B's F1/F2): a name C compiles must be a name the gate compares.
            # An implicit member after a non-literal (an alias, a cast, an expression) is
            # left out: its value is not decidable from the text, and guessing it would
            # make the gate wrong instead of silent.
            running: int | None = -1     # C starts an enum at 0
            for member in split_top_level_commas("".join(body)):
                m = re.match(r"\s*([A-Za-z_]\w*)\s*=\s*(.+?)\s*$", member, re.S)
                if m:
                    value = " ".join(m.group(2).split())
                    out[m.group(1)] = value
                    running = integer_literal(value)
                    continue
                n = re.match(r"\s*([A-Za-z_]\w*)\s*$", member, re.S)
                if n and running is not None:
                    running += 1
                    out[n.group(1)] = str(running)
                else:
                    running = None
            inside, body = False, []
        else:
            body.append(ch)
        i += 1
    return out


def flat_code(text: str) -> str:
    """The live, comment- and string-stripped code with preprocessor lines dropped.

    Preprocessor lines are not code, and leaving them in the text broke three things
    (review A's F3, 2026-10-02):

    * the `extern "C" {` of a `#ifdef __cplusplus` block is a real live line, so the brace
      depth stayed 1 for the rest of the file and *every* file-scope enum behind it was
      discarded as "inside a function or struct" (lib/timespec.h, lib/readutmp.h,
      lib/sha256.h, lib/sha3.h, lib/sha512.h, src/emacs-module.in.h, src/macfont.h,
      src/xterm.h);
    * the continuation lines of a multi-line `#define` were counted as braces
      (`src/xterm.h`'s `do { … } while (false)` left a permanent depth of -1);
    * an `#if`/`#else` *inside* an enum body glued onto the preceding member, so the
      `name = value` regex failed on the leading `#` (`src/macfont.h`'s last member).

    A logical preprocessor line starts with `#` (after optional blanks) and continues while
    the physical line ends in a backslash.
    """
    out, skipping = [], False
    for _, line, live in Lines(text):
        if not live:
            continue
        # Character literals too: image.c has `'{'`/`'}'`, and a counted brace inside one
        # walks the depth off (review A's F3 diagnosis).
        stripped = strip_char_literals(strip_strings(line))
        if skipping:
            skipping = stripped.rstrip().endswith("\\")
            continue
        if stripped.lstrip().startswith("#"):
            skipping = stripped.rstrip().endswith("\\")
            continue
        out.append(stripped)
    return "\n".join(out)


def brace_depth(code: str, pos: int) -> int:
    """The `{` nesting depth at POS in flat code, not counting `extern "C" {`.

    That brace is C++ linkage syntax, not a body: counting it hid the file-scope enums
    behind it (see `flat_code`).  Its matching `}` is skipped too.
    """
    depth = 0
    stack: list[bool] = []
    for i, ch in enumerate(code[:pos]):
        if ch == "{":
            linkage = code[max(0, i - 12):i].rstrip().endswith('extern "C"')
            stack.append(not linkage)
            if not linkage:
                depth += 1
        elif ch == "}":
            if stack.pop() if stack else False:
                depth -= 1
    return depth


def split_top_level_commas(body: str) -> list[str]:
    """BODY's members, split on the commas that are not inside brackets.

    `CHAR_TABLE_STANDARD_SLOTS = (PSEUDOVECSIZE (struct Lisp_Char_Table, contents) - 1 …)`
    has commas of its own; splitting on every comma truncated the value and, in
    `NLIMBS_LIMIT = min (min (GMP_NLIMBS_MAX, min (PTRDIFF_MAX, SIZE_MAX) / …) …)`
    (bignum.c 330), invented a member named `lim` out of `mp_limb_t`'s neighbours.
    """
    out, cur, depth = [], "", 0
    for ch in body:
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
            continue
        cur += ch
    if cur.strip():
        out.append(cur)
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
    for name, value in enum_constants(text).items():
        out.setdefault(name, value)
    return out


def struct_fields(text: str) -> list[dict]:
    """Every field of every struct/union, with container and nesting path."""
    lines = Lines(text)
    rows = lines.rows
    stack: list[tuple[str, str]] = []
    pending = None
    out: list[dict] = []
    # A `typedef struct X { … } Y;` names the aggregate twice - the tag X and the alias Y - and the
    # port may put either on the Rust type (etags.c 275's `typedef struct node_st { … } node;` is
    # `pub struct node`, while lwlib-widget.h's `_widget_value` keeps the tag).  Both names are
    # therefore recorded, so a comparison can accept either (status 3.152); the alias is only read
    # for aggregates a `typedef` introduced, because in `struct Z { struct { … } spec; };` the same
    # `} spec;` shape names a *member* of the enclosing type, which the code below already records.
    typedef_keys: set[str] = set()
    typedef_pending: set[str] = set()
    aliases: dict[str, str] = {}
    # A C declaration may span lines (`void (*set_window_size_hook) (struct frame *f,\n int width,
    # int height);`), and reading one line at a time invented fields out of the *parameters* of
    # such a member (`width`, `yoff`, `title` appeared as members of `struct terminal`) while the
    # member itself was lost.  Declarations are therefore joined until their `;` before being
    # parsed — the same rule the preprocessor uses (P-73).
    buf = ""
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
            buf = ""
            key = m.group("name") or f"<anon@{i}>"
            stack.append((m.group("kind"), key))
            if m.group("td"):
                typedef_keys.add(key)
            if stripped.count("{") == stripped.count("}"):
                stack.pop()
            continue
        m = STRUCT_OPEN_NAME.match(stripped)
        if m:
            pending = (m.group("kind"), m.group("name") or f"<anon@{i}>")
            if m.group("td"):
                typedef_pending.add(pending[1])
            continue
        if stripped == "{":
            if pending:
                stack.append(pending)
                if pending[1] in typedef_pending:
                    typedef_keys.add(pending[1])
                pending = None
            continue
        if stripped.startswith("}"):
            buf = ""
            closed: list[tuple[str, str]] = []
            for _ in range(stripped.count("}")):
                if stack:
                    closed.append(stack.pop())
            # `} spec;` names the aggregate that just closed: it is a field of whatever
            # contains it, and C lets it be keyed (`offsetof (struct coding_system, spec)`)
            # even though its own members cannot.  Without this the type had no key for the
            # member, and — because the closing brace popped the *outer* frame — every field
            # after the aggregate vanished from the scan (see `coding_system`: `spec` and the
            # 17 members after it were invisible, so the 616-byte layout was unchecked).
            tail = re.match(r"^}\s*(?P<name>[A-Za-z_]\w*)\s*(?P<array>(?:\[[^\]]*\])*)\s*;", stripped)
            if tail is None and re.fullmatch(r"}+", stripped):
                # `}` and the declarator may be on separate lines (frame.h 653-663's
                # `union output_data { … }\n  output_data;`), in which case the name is on the
                # *next* line; remember to read it there rather than lose the member.
                closers = stripped.count("}")
                for _ in range(0):
                    pass
                if closers == 1 and stack:
                    following = rows[i].strip() if i < len(rows) else ""
                    tail2 = re.fullmatch(r"(?P<name>[A-Za-z_]\w*)\s*(?P<array>(?:\[[^\]]*\])*)\s*;",
                                         following)
                    if tail2:
                        outer = next((e for e in stack if not e[1].startswith("<anon")), stack[-1])
                        out.append({"container": outer[1], "container_kind": outer[0],
                                    "nesting": " > ".join(e[1] for e in stack),
                                    "line": i + 1, "name": tail2.group("name"),
                                    "array": tail2.group("array"), "pointer_depth": 0,
                                    "bitfield": False, "c_type": "<aggregate>",
                                    "raw": following})
                        i += 1          # consume the declarator line
            if tail and closed and closed[-1][1] in typedef_keys:
                # `typedef struct X { … } Y;` (or the anonymous twin): Y is the alias of the
                # aggregate that just closed, and it is the name the port may have used.
                aliases[closed[-1][1]] = tail.group("name")
            if tail and stack:
                outer = next((e for e in stack if not e[1].startswith("<anon")), stack[-1])
                out.append({"container": outer[1], "container_kind": outer[0],
                            "nesting": " > ".join(e[1] for e in stack),
                            "line": i, "name": tail.group("name"), "array": tail.group("array"),
                            "pointer_depth": 0, "bitfield": False, "c_type": "<aggregate>",
                            "raw": tail.group(0)})
            pending = None
            continue
        if not stack:
            buf = ""
            continue
        buf = f"{buf} {stripped}".strip() if buf else stripped
        if ";" not in stripped:
            continue                      # the declaration continues on the next line
        for field in _fields_on_line(buf):
            outer = next((e for e in stack if not e[1].startswith("<anon")), stack[-1])
            out.append({"container": outer[1], "container_kind": outer[0],
                        "nesting": " > ".join(e[1] for e in stack),
                        "line": i, **field})
        buf = ""
    for record in out:
        alias = aliases.get(record["container"])
        if alias:
            record["alias"] = alias
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
        # `Lines.scan_rows` is the cross-line-aware view: `strip_strings` is a per-line regex
        # and cannot see a literal that *starts* on an earlier line, which is how C's
        # backslash-continued string literals (translation phase 2 splices them) leaked their
        # words into this scan - etags.c's `Forth_help` produced five "file-scope objects"
        # (`code`, `create`, `defer`, `value`, `variable`) that C never declares (status 3.130).
        stripped = lines.scan_rows[i - 1]
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
        split = _specifiers_and_declarators(ctype, m.group("decls"))
        if split is None:
            continue
        ctype, decls = split
        for d in decls:
            out.append({"c_type": ctype, "name": d.group("name"),
                        "array": d.group("array") or "",
                        "pointer_depth": len(d.group("stars") or ""),
                        "static": ctype.startswith("static"),
                        "const": "const" in ctype,
                        "init": (m.group("init") or "").strip(), "line": i})
    return out


# C writes some attributes *after* the declarator (`void *lisp_malloc_loser EXTERNALLY_VISIBLE;`,
# `bool print_output_debug_flag EXTERNALLY_VISIBLE = 1;`), so the object's name is what precedes
# them.  The set is measured, not guessed: a scan of all 590 reference files for objects whose
# name matches C's attribute spelling returned 17 ALL-CAPS names, and exactly this one is an
# attribute rather than an object (status 3.143).
TRAILING_ATTRIBUTES = frozenset({"EXTERNALLY_VISIBLE"})


def _specifiers_and_declarators(ctype: str, decls: str):
    """Give back (specifiers, [DECLARATOR match, ...]) or None.

    C's declaration specifiers are several words, and `GLOBAL_LINE`'s type group is lazy:
    `extern unsigned int a_one;` parses as type "extern unsigned" and declarator "int a_one",
    which `DECLARATOR` then refuses - so the object was **silently dropped** from the C side of
    the file-scope-object gate.  Every multi-word type (`unsigned int`, `unsigned long`,
    `long long`, `unsigned short`, a pointer with a trailing `const`, ...) was invisible that
    way, which is what status 3.143 fixed after a lane hit it on `src/androidterm.h`'s
    `current_menu_serial` (an `extern unsigned int`).  Walk specifier words across the boundary
    until every declarator parses, and keep the last such split; if none parses, report None so
    the caller skips the line rather than inventing an object.
    """
    words = decls.split()
    while len(words) > 1 and words[-1] in TRAILING_ATTRIBUTES:
        words.pop()
    decls = " ".join(words)
    while True:
        parts = [d.strip() for d in split_declarators(decls)]
        matches = [DECLARATOR.match(part) for part in parts]
        if parts and all(matches):
            return ctype, matches
        if len(words) <= 1:
            return None
        ctype = (ctype + " " + words.pop(0)).strip()
        decls = " ".join(words)


def defuns(text: str) -> list[dict]:
    """Every DEFUN, matched over the whole raw file (docstrings are comments)."""
    clean = blank_comments(text)
    live = dead_branch_map(clean)
    out = []
    for m in DEFUN.finditer(text):
        line_no = text.count("\n", 0, m.start()) + 1
        if not live[line_no - 1]:
            continue
        # make-docfile scans the text after the docstring for an
        # `attributes: …` list and turns it into _Noreturn / NO_INLINE /
        # ATTRIBUTE_CONST / ATTRIBUTE_PURE prefixes and suffixes on the EXFUN
        # it writes to globals.h (lib-src/make-docfile.c 1081-1136), so the
        # flags belong to the frozen DEFUN table.
        tail = text[m.end(): m.end() + 200]
        attr = ATTRIBUTES.search(tail)
        flags = " ".join(attr.group("list").split()) if attr else ""
        out.append({"lisp_name": m.group("lisp"), "c_name": m.group("cname"),
                    "s_name": m.group("sname"), "min": m.group("min").strip(),
                    "max": m.group("max").strip(), "n": m.group("n").strip(),
                    "flags": flags,
                    "line": line_no,
                    "doc_digest": hashlib.sha1(
                        re.sub(r"\s+", " ", doc_comment_value(m.group("doc"))).encode()
                    ).hexdigest()[:12]})
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
    with _BlindConditions(path):
        return _inventory(path)


def _inventory(path: Path) -> dict:
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


def selftest() -> int:
    """The `globals_` classifier cases an earlier version got wrong (status 3.143).

    The bug this pins down was silent: `GLOBAL_LINE`'s lazy type group left the last specifier
    at the front of the declarator (`extern unsigned int a_one;` parsed as type "extern unsigned",
    declarator "int a_one"), `DECLARATOR` then refused it, and the object simply vanished from
    the C side of the file-scope-object gate.  A lane hit it on `src/androidterm.h`'s
    `current_menu_serial`; the second case is the fix's own false positive (C's post-declarator
    attributes), and the third and fourth are the shapes that must not regress.
    """
    cases = [
        ("multi-word specifiers",
         "extern unsigned int a_one;\nextern unsigned long a_two;\n"
         "extern long long a_three;\nunsigned short a_four;\n",
         ["a_one", "a_two", "a_three", "a_four"]),
        ("trailing attribute names the object before it",
         "void *lisp_malloc_loser EXTERNALLY_VISIBLE;\n"
         "bool print_output_debug_flag EXTERNALLY_VISIBLE = 1;\n",
         ["lisp_malloc_loser", "print_output_debug_flag"]),
        ("ordinary shapes do not regress",
         "extern Lisp_Object Qt, Qnil;\nstatic const char *a_six;\nFILE *a_seven, *a_eight;\n",
         ["Qt", "Qnil", "a_six", "a_seven", "a_eight"]),
        ("a function declaration is not an object",
         "extern void debug_output_compilation_hack (bool) EXTERNALLY_VISIBLE;\n",
         []),
        ("an array of pointers to const objects",
         "Lisp_Object const *staticvec[NSTATICS];\n",
         ["staticvec"]),
        ("typedef'd struct and union bodies are not invisible",
         "typedef struct X\n{\n  int a;\n} Y;\ntypedef union U {\n  int b;\n} V;\n",
         ["X.a", "U.b"]),
        # `typedef struct X { … } Y;` names the aggregate twice; both names are recorded, and the
        # same `} name;` shape inside a *non*-typedef aggregate stays a member (status 3.152).
        ("a typedef'd aggregate records its alias, a member does not",
         "typedef struct node_st\n{\n  int a;\n} node;\n"
         "struct Z\n{\n  struct\n  {\n    int b;\n  } spec;\n};\n",
         ["node_st.a@node", "Z.b", "Z.spec"]),
    ]
    bad = 0
    for name, src, want in cases:
        if name.startswith("typedef'd"):
            got = [f"{f['container']}.{f['name']}" for f in struct_fields(src)]
        elif name.startswith("a typedef'd aggregate"):
            got = [f"{f['container']}.{f['name']}"
                   + (f"@{f['alias']}" if f.get("alias") else "") for f in struct_fields(src)]
        else:
            got = [g["name"] for g in globals_(src)]
        ok = got == want
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got}" + ("" if ok else f" (want {want})"))
        bad += 0 if ok else 1
    print(f"{len(cases)} cases, {bad} failures")
    return 1 if bad else 0


if __name__ == "__main__":
    import sys

    if "--selftest" in sys.argv[1:]:
        sys.exit(selftest())
    print("cinventory.py: import this module (extract-abi.py does), or pass --selftest")
    sys.exit(2)
