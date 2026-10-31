#!/usr/bin/env python3
"""Find the port's own comment prose naming the reference by its bare product name.

PORTING.md 0.1-19 rules the product name out of a comment.  What it does *not* rule out is
C's own comment text: a ported unit copies its C file's comments, and those comments call the
program they belong to by name - changing them would change C's bytes (the nineteenth half,
status 3.796, counted 954 such lines and left them out of every gate because the table carries
the reference's *registered* names, not the bare word).

That count was an assertion about a class, not a measurement of it: whether a line is C's text
or this repository's is exactly what nobody read.  This tool reads it.  For every ported unit
and header mirror it resolves the reference the file's own first line names (and, through the
file's `include!` graph, the references of the mirrors it pulls in), and then reads every
comment occurrence of the bare name:

  * inside a quotation - a `` `…` `` span or a `"…"` span, including one the comment's hard
    wrap leaves open on the next line - is a citation of C's own bytes and stays;
  * text that occurs in the reference (the comment line, or any six-word window of it) is C's
    own comment, copied, and stays - the reference set is the file's own C file, the C files of
    every mirror in its `include!` graph, and each of those C files' own `#include "..."`ed
    headers, because a unit transcribes its C file's comments and those may come from a header;
  * anything else is written in this repository's voice and must call the reference `C` or
    `the reference`, never by the product's name.

The second face is this repository's own tooling: `tools/`, `spikes/`, `.github/` and
`xtask` are a scope the declaration gate has read since its sixth half (status 3.462)
and that no reader of *this* gate ever walked - `files()` is bound to the ported trees, so a
`#` comment in a Python tool, a probe's own C comment or a workflow's prose was read by nobody.
Nothing in those files is copied from the reference, so the face needs no reference set: every
bare name in their prose is written here (the three files whose subject matter is this rule's
own pattern table are exempt by name and counted).

    python3 tools/check-own-voice.py              # scan the ported trees and our own tooling
    python3 tools/check-own-voice.py --selftest   # the reader's own cases
"""

from __future__ import annotations

import argparse
import re
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REF = REPO / "emacs"

TREES = ("src", "lib", "lib-src", "nt",
         "lwlib", "oldXMenu", "exec", "msdos")

# The walk was bound to `*.rs` from its first round (status 3.803), and the platform tree is
# Swift/Kotlin/C++ (status 3.136 names it; status 3.250 writes the rule: a tree whose files no
# suffix of the walk matches is a tree the walk never reads).  A Kotlin, Swift or C++ file is
# copied from a `.java`, `.m` or `.cc` reference exactly the way a unit is copied from its C
# file - its own first line names that reference - so the same reading applies to its comments.
# The twenty-fourth half (status 3.887) binds the walk to these suffixes instead of to one.
PLATFORM_TREE = "platform"
PLATFORM_SUFFIXES = (".swift", ".kt", ".cc")

# The second face is this repository's own tooling - `tools/`, `spikes/`, `.github/` and
# `xtask` - the scope the declaration gate gained in its sixth half (status 3.462) and
# that no reader *of this gate* has ever walked: `files()` above is bound to the ported trees,
# so a `#` comment in a Python tool, a probe's own C comment or a CI workflow's prose was read
# by nobody.  Nothing in these files is copied from the reference, so this face needs no
# reference set at all - it is the one scope where the reading is structural: every bare name in
# their prose is written here.  Prose here is a comment, a docstring, or the string literal a
# tool prints (the ported face reads comments only because there the other two are C's bytes).
# Three files are exempt by name, for the reason the declaration gate's `TOOL_EXEMPT` gives for
# their `literal` class: writing the product's name down *is* their subject matter (pattern
# tables and self-test fixtures), so the name in them is notation, not a declaration.
OWN_TREES = ("tools", "spikes", ".github", "xtask")
OWN_SUFFIXES = (".py", ".sh", ".c", ".h", ".rs", ".yml", ".yaml", ".toml")
OWN_EXEMPT = ("tools/check-declarations.py", "tools/check-own-voice.py",
              "tools/check-verbatim-declarations.py")
C_PROSE = (".c", ".h", ".rs")

# The declaration is a *prefix* of the file's first line, not the whole of it (the twenty-third
# half, status 3.886): `// femacs: port of lib/c-strcasecmp.c @ fd7afced403.  The file is` names
# the reference and carries on - the strict `\s*$` form this regex used to require left those
# files out of every gate's reading while their first line did name a reference.
FIRST_LINE = re.compile(r"^// femacs: (?:port|mirror) of (\S+) @ (\S+)\.")
INCLUDE = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')
BARE_PRODUCT_NAME = "Emacs"
BARE_NAME = re.compile(r"(?<!GNU )\b" + re.escape(BARE_PRODUCT_NAME) + r"\b")
WHITESPACE = re.compile(r"\s+")

WINDOW = 6


def normalize(text: str) -> str:
    return WHITESPACE.sub(" ", text).strip()


def reference_of(path: Path) -> str | None:
    """The reference path a ported file's first line names, or None."""
    try:
        first = path.read_text(encoding="utf-8", errors="replace").split("\n", 1)[0]
    except OSError:
        return None
    m = FIRST_LINE.match(first)
    return m.group(1) if m else None


C_INCLUDE = re.compile(r'^\s*#\s*include\s+"([^"]+)"', re.M)


def reference_set(path: Path, depth: int = 4) -> list[str]:
    """The reference files a ported file's text may carry: its own, plus its include graph's.

    A unit's comments are its C file's comments, and a C file's comments may sit in a header it
    includes (`ccl.c`'s charset comment is `charset.h`'s), so each reference file contributes
    the `#include "..."` names it names as well.
    """
    out: list[str] = []
    walk: list[Path] = [path]
    seen: set[Path] = set()
    while walk:
        p = walk.pop()
        if p in seen:
            continue
        seen.add(p)
        ref = reference_of(p)
        if ref and ref not in out:
            out.append(ref)
            c_file = REF / ref
            if c_file.exists():
                for name in C_INCLUDE.findall(c_file.read_text(encoding="utf-8",
                                                             errors="replace")):
                    for candidate in (Path("src") / name, Path("lib") / name, Path(name)):
                        if (REF / candidate).exists():
                            entry = str(candidate)
                            if entry not in out:
                                out.append(entry)
                            break
        if depth > 0:
            try:
                text = p.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for target in INCLUDE.findall(text):
                q = (p.parent / target).resolve()
                if q.exists() and q.is_relative_to(REPO):
                    walk.append(q)
    return out


class References:
    """Whitespace-collapsed reference text, read once per file."""

    def __init__(self) -> None:
        self._cache: dict[str, str] = {}
        self._limit = 24

    def blob(self, ref: str) -> str:
        if len(self._cache) >= self._limit:
            self._cache.clear()
        if ref not in self._cache:
            p = REF / ref
            self._cache[ref] = (normalize(p.read_text(encoding="utf-8", errors="replace"))
                                if p.exists() else "")
        return self._cache[ref]

    def carries(self, refs: list[str], piece: str) -> bool:
        """Is this comment text (or a six-word window of it) the reference's own bytes?"""
        if not piece:
            return False
        words = piece.split()
        if len(words) < WINDOW:
            return any(piece in self.blob(r) for r in refs)
        for i in range(len(words) - WINDOW + 1):
            window = " ".join(words[i:i + WINDOW])
            if any(window in self.blob(r) for r in refs):
                return True
        return False


def strip_strings(line: str) -> str:
    return re.sub(r'"(?:[^"\\]|\\.)*"', lambda m: "\x00" * len(m.group(0)), line)


def comment_pieces(lines: list[str]) -> list[tuple[int, int, int, str]]:
    """Every comment piece on every line: (line number, start, end, text)."""
    out: list[tuple[int, int, int, str]] = []
    in_block = False
    for number, line in enumerate(lines, 1):
        stripped = strip_strings(line)
        pos = 0
        while pos < len(stripped):
            if in_block:
                end = stripped.find("*/", pos)
                if end < 0:
                    out.append((number, pos, len(line), line[pos:]))
                    pos = len(stripped)
                else:
                    out.append((number, pos, end + 2, line[pos:end + 2]))
                    in_block = False
                    pos = end + 2
                continue
            line_comment = stripped.find("//", pos)
            block_comment = stripped.find("/*", pos)
            if line_comment < 0 and block_comment < 0:
                break
            if line_comment >= 0 and (block_comment < 0 or line_comment < block_comment):
                out.append((number, line_comment, len(line), line[line_comment:]))
                break
            end = stripped.find("*/", block_comment)
            if end < 0:
                out.append((number, block_comment, len(line), line[block_comment:]))
                in_block = True
                pos = len(stripped)
            else:
                out.append((number, block_comment, end + 2, line[block_comment:end + 2]))
                pos = end + 2
    return out


def comment_blocks(lines: list[str]) -> list[list[tuple[int, int, int, str]]]:
    """Comment pieces grouped into blocks, so a quotation may span the block's hard wraps."""
    blocks: list[list[tuple[int, int, int, str]]] = []
    current: list[tuple[int, int, int, str]] = []
    for piece in comment_pieces(lines):
        if current and piece[0] - current[-1][0] <= 1:
            current.append(piece)
        else:
            if current:
                blocks.append(current)
            current = [piece]
    if current:
        blocks.append(current)
    return blocks


def quoted_mask(text: str, state: dict[str, str], quotes: bool = True) -> list[bool]:
    """Per-character mask of a prose piece: is this character inside a citation?

    `state` carries the quotation a hard wrap left open ("tick" or "quote") from the piece
    before it in the same block, and is updated in place.  `quotes` is false for a string
    literal: there the `"` are the delimiters of the text the program prints, not a citation
    of somebody else's words, while a `` `...` `` span inside it still is one.
    """
    mask = [False] * len(text)
    marker = state.get("open", "")
    i = 0
    while i < len(text):
        char = text[i]
        if marker:
            mask[i] = True
            if char == marker and (i == 0 or text[i - 1] != "\\"):
                marker = ""
            i += 1
            continue
        if (char == "`" or (quotes and char == '"')) and (i == 0 or text[i - 1] != "\\"):
            marker = char
        i += 1
    state["open"] = marker
    return mask


def comment_text(piece: str) -> str:
    body = re.sub(r"^(//+|/\*+|\*+)\s?", "", piece)
    return normalize(body.rstrip("*/\n").strip())


def scan_file(path: Path, references: References) -> list[tuple[int, str]]:
    lines = path.read_text(encoding="utf-8", errors="replace").split("\n")
    refs = reference_set(path)
    findings: list[tuple[int, str]] = []
    for block in comment_blocks(lines):
        state: dict[str, str] = {}
        for number, start, _end, text in block:
            mask = quoted_mask(text, state)
            for match in BARE_NAME.finditer(text):
                if mask[match.start()]:
                    continue
                if references.carries(refs, comment_text(text)):
                    continue
                findings.append((number, normalize(lines[number - 1])[:180]))
    return findings


def files() -> list[Path]:
    out: list[Path] = []
    for tree in TREES:
        out.extend(sorted((REPO / tree).rglob("*.rs")))
    for suffix in PLATFORM_SUFFIXES:
        out.extend(sorted((REPO / PLATFORM_TREE).rglob(f"*{suffix}")))
    return out


def own_files() -> list[Path]:
    out: set[Path] = set()
    for tree in OWN_TREES:
        for suffix in OWN_SUFFIXES:
            for path in sorted((REPO / tree).rglob(f"*{suffix}")):
                if "target" not in path.parts:
                    out.add(path)
    return sorted(out)


def unscanned_own() -> list[str]:
    """The files of those four trees that no suffix of the walk matches (status 3.250's rule).

    A tree named in a scope whose files the walk does not match is a tree the walk only
    *claims* to read, so the bound is printed rather than left to be inferred - the same line
    `tools/check-declarations.py` prints for its own walks.
    """
    out: list[str] = []
    for tree in OWN_TREES:
        for path in sorted((REPO / tree).rglob("*")):
            if path.is_file() and "target" not in path.parts and path.suffix not in OWN_SUFFIXES:
                out.append(str(path.relative_to(REPO)))
    return out


def prose_pieces(lines: list[str], suffix: str) -> list[tuple[int, int, int, str, bool]]:
    """The prose a file of ours carries: (line, start, end, text, quotes_are_citations).

    Two dialects.  A C-style file (`.c`, `.h`, `.rs`) rereads what the ported face reads -
    `comment_pieces`' `//` and `/* */` spans - and adds the `"..."` literals *outside* them,
    because a message a tool prints is prose too.  A `#`-comment file (`.py`, `.sh`, `.yml`,
    `.yaml`, `.toml`) reads its `#` comments and its string literals, carrying Python's triple
    quotes across lines: a module docstring is that file's opening block, the same thing the
    ported face reads in a C file's comment.
    """
    out: list[tuple[int, int, int, str, bool]] = []
    if suffix in C_PROSE:
        in_block = False
        in_string = False
        for number, line in enumerate(lines, 1):
            index = 0
            while index < len(line):
                if in_string:
                    end = index
                    while end < len(line):
                        if line[end] == "\\":
                            end += 2
                            continue
                        if line[end] == '"':
                            break
                        end += 1
                    stop = min(end + 1, len(line))
                    out.append((number, index, stop, line[index:stop], False))
                    if end < len(line):
                        in_string = False
                    index = stop
                    continue
                if in_block:
                    end = line.find("*/", index)
                    stop = len(line) if end < 0 else end + 2
                    out.append((number, index, stop, line[index:stop], True))
                    if end >= 0:
                        in_block = False
                    index = stop
                    continue
                line_comment = line.find("//", index)
                block_comment = line.find("/*", index)
                quote = line.find('"', index)
                first = min((p for p in (line_comment, block_comment, quote) if p >= 0),
                            default=-1)
                if first < 0:
                    break
                if first == quote:
                    # A message the program prints is prose, and it may run past this line:
                    # `"..." \` on one line and the rest inside the quotes on the next (the
                    # xtask's `gate-full` message is written that way) is one string.
                    in_string = True
                    index = first + 1
                    continue
                if first == line_comment:
                    out.append((number, first, len(line), line[first:], True))
                    break
                end = line.find("*/", first)
                if end < 0:
                    out.append((number, first, len(line), line[first:], True))
                    in_block = True
                    break
                out.append((number, first, end + 2, line[first:end + 2], True))
                index = end + 2
        return sorted(out)
    triple = ""
    for number, line in enumerate(lines, 1):
        index = 0
        while index < len(line):
            if triple:
                end = line.find(triple, index)
                stop = len(line) if end < 0 else end + len(triple)
                out.append((number, index, stop, line[index:stop], False))
                if end >= 0:
                    triple = ""
                index = stop
                continue
            char = line[index]
            if char == "#":
                out.append((number, index, len(line), line[index:], True))
                break
            if char in ("'", '"'):
                if line.startswith(char * 3, index):
                    triple = char * 3
                    index += 3
                    continue
                end = index + 1
                while end < len(line):
                    if line[end] == "\\":
                        end += 2
                        continue
                    if line[end] == char:
                        break
                    end += 1
                stop = min(end + 1, len(line))
                out.append((number, index, stop, line[index:stop], False))
                index = stop
                continue
            index += 1
    return out


def prose_blocks(pieces: list[tuple[int, int, int, str, bool]]):
    """Prose pieces grouped, so a quotation may span a block's hard wraps."""
    blocks: list[list[tuple[int, int, int, str, bool]]] = []
    current: list[tuple[int, int, int, str, bool]] = []
    for piece in pieces:
        if current and piece[0] - current[-1][0] <= 1 and piece[4] == current[-1][4]:
            current.append(piece)
        else:
            if current:
                blocks.append(current)
            current = [piece]
    if current:
        blocks.append(current)
    return blocks


def read_own(path: Path):
    """Read one file of ours: (read, quoted, own) and the findings."""
    lines = path.read_text(encoding="utf-8", errors="replace").split("\n")
    read = quoted = own = 0
    findings: list[tuple[int, str]] = []
    for block in prose_blocks(prose_pieces(lines, path.suffix)):
        state: dict[str, str] = {}
        for number, _start, _end, text, quotes in block:
            mask = quoted_mask(text, state, quotes)
            for match in BARE_NAME.finditer(text):
                read += 1
                if mask[match.start()]:
                    quoted += 1
                else:
                    own += 1
                    findings.append((number, normalize(lines[number - 1])[:180]))
    return (read, quoted, own), findings


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args(argv)
    if args.selftest:
        return selftest()

    references = References()
    read = quoted = own = 0
    failures = 0
    walked = files()
    no_reference: list[str] = []
    for path in walked:
        # A file that declares no reference - a generated mirror, or a unit whose first line is
        # C's own comment rather than the licence block the rule replaced - has no reference
        # bytes to compare against, and that is the reading, not an exemption: nothing in such a
        # file can be *C's copied comment*, so every bare name in it is written here.  The
        # twenty-third half (status 3.886) reads them instead of skipping them and prints the
        # count, which is the class the nineteenth half left unmeasured.
        if reference_of(path) is None:
            no_reference.append(str(path.relative_to(REPO)))
        lines = path.read_text(encoding="utf-8", errors="replace").split("\n")
        refs = reference_set(path)
        hits = scan_file(path, references)
        for block in comment_blocks(lines):
            state: dict[str, str] = {}
            for _number, _start, _end, text in block:
                mask = quoted_mask(text, state)
                for match in BARE_NAME.finditer(text):
                    read += 1
                    if mask[match.start()]:
                        quoted += 1
                    elif references.carries(refs, comment_text(text)):
                        pass
                    else:
                        own += 1
        for number, text in hits:
            relative = path.relative_to(REPO)
            print(f"FAIL {relative}:{number}: this repository's own comment names the reference "
                  f"`{BARE_PRODUCT_NAME}` - call it C or the reference")
            failures += 1
    print(f"read {read} bare-name occurrence(s) in ported comment(s): {read - quoted - own} "
          f"copied from the reference, {quoted} inside a citation, {own} this repository's own "
          f"voice; {len(walked)} file(s) walked, {len(no_reference)} with no reference "
          f"declaration on the file's own first line (read against an empty reference set, so "
          f"their comment prose is this repository's)")
    own_walked = own_files()
    own_read = own_quoted = own_voice = 0
    exempt = 0
    for path in own_walked:
        relative = str(path.relative_to(REPO))
        if relative in OWN_EXEMPT:
            exempt += 1
            continue
        counts, findings = read_own(path)
        own_read += counts[0]
        own_quoted += counts[1]
        own_voice += counts[2]
        for number, text in findings:
            print(f"FAIL {relative}:{number}: this repository's own tool names the reference "
                  f"`{BARE_PRODUCT_NAME}` - call it C or the reference")
            failures += 1
    print(f"read {own_read} bare-name occurrence(s) in this repository's own tool, probe and CI "
          f"prose: {own_quoted} inside a citation, {own_voice} this repository's own voice; "
          f"{len(own_walked)} file(s) walked, {exempt} exempt by name (the rule's own notation: "
          f"{', '.join(OWN_EXEMPT)})")
    left = unscanned_own()
    print(f"walked past {len(left)} file(s) of an unscanned kind: "
          f"{', '.join(left) if left else 'none'}")
    print(f"{failures} failure(s)")
    return 1 if failures else 0


def selftest() -> int:
    """The reader's own cases: what counts as a citation, what counts as our own voice."""
    cases = 0
    failed = 0

    def check(label: str, body: str, expected: int) -> None:
        nonlocal cases, failed
        cases += 1
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "emacs/src").mkdir(parents=True)
            (root / "src").mkdir(parents=True)
            (root / "emacs/src/demo.c").write_text(
                "/* Demo does a thing.\n   Demo may also do another thing.  */\n", encoding="utf-8")
            target = root / "src/demo.rs"
            target.write_text(f"// femacs: port of src/demo.c @ deadbeef.\n{body}\n",
                              encoding="utf-8")
            global REPO, REF
            saved = (REPO, REF)
            REPO, REF = root, root / "emacs"
            try:
                hits = scan_file(target, References())
            finally:
                REPO, REF = saved
            if len(hits) != expected:
                failed += 1
                print(f"FAIL selftest {label}: {len(hits)} finding(s), expected {expected}")
            else:
                print(f"ok   selftest {label}")

    check("C's own comment, copied", "/* Demo does a thing.  */", 0)
    check("a citation in backticks", "// C 3: `the Emacs program` is the name it prints", 0)
    check("a citation in quotes", '// C 3: "the Emacs program" is the name it prints', 0)
    check("a citation a hard wrap leaves open",
          '// C 3: "the name this program prints is\n// Emacs itself, as C spells it" stays', 0)
    check("this repository's own voice", "// the two Emacs headers this slice names", 1)
    check("our own voice even next to a citation",
          "// `x` is ours but the Emacs names below are not", 1)

    def check_pair(label: str, header: str, body: str, expected: int) -> None:
        nonlocal cases, failed
        cases += 1
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "emacs/src").mkdir(parents=True)
            (root / "src").mkdir(parents=True)
            (root / "emacs/src/demo.c").write_text(
                "/* Demo does a thing.\n   Demo may also do another thing.  */\n", encoding="utf-8")
            target = root / "src/demo.rs"
            target.write_text(f"{header}\n{body}\n", encoding="utf-8")
            global REPO, REF
            saved = (REPO, REF)
            REPO, REF = root, root / "emacs"
            try:
                found = len(scan_file(target, References()))
            finally:
                REPO, REF = saved
            if found != expected:
                failed += 1
                print(f"FAIL selftest {label}: {found} finding(s), expected {expected}")
            else:
                print(f"ok   selftest {label}")

    # The twenty-third half's two cases: the declaration is a prefix of the first line, and a
    # file that declares none is read (its comment prose is ours, so nothing in it is C's).
    # The twenty-fourth half's case: the walk the platform files were never in.  A `.swift`,
    # `.kt` or `.cc` file is named by a suffix the walk binds now; before this half every tree
    # was walked as `*.rs`, so a Kotlin, Swift or C++ file was a file no walk read (status
    # 3.250's rule for a tree whose files no suffix of the walk matches).
    global REPO, REF
    cases += 1
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for tree, name in (("platform/macos", "demo.swift"),
                           ("platform/android/kotlin", "Demo.kt"),
                           ("platform/haiku", "demo.cc"),
                           ("src", "demo.rs"),
                           ("platform/macos", "demo.m")):
            (root / tree).mkdir(parents=True, exist_ok=True)
            (root / tree / name).write_text("// x\n", encoding="utf-8")
        saved = (REPO, REF)
        REPO = root
        try:
            walked = sorted(p.name for p in files())
        finally:
            REPO, REF = saved
        if walked != ["Demo.kt", "demo.cc", "demo.rs", "demo.swift"]:
            failed += 1
            print(f"FAIL selftest the walk binds the platform suffixes: {walked}")
        else:
            print("ok   selftest the walk binds the platform suffixes")

    check_pair("a first line that names a reference and carries on",
               "// femacs: port of src/demo.c @ deadbeef.  The file is a demo.",
               "/* Demo does a thing.  */", 0)
    check_pair("no reference declared: C's comment is not C's here",
               "/* Demo does a thing.  */",
               "// the Emacs program's own note", 1)

    def check_own(label: str, name: str, body: str, expected: int) -> None:
        """The second face's cases: our own tooling needs no reference set."""
        nonlocal cases, failed
        cases += 1
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "tools").mkdir(parents=True)
            target = root / "tools" / name
            target.write_text(body, encoding="utf-8")
            global REPO, REF
            saved = (REPO, REF)
            REPO, REF = root, root / "emacs"
            try:
                _counts, found = read_own(target)
            finally:
                REPO, REF = saved
            if len(found) != expected:
                failed += 1
                print(f"FAIL selftest {label}: {len(found)} finding(s), expected {expected}")
            else:
                print(f"ok   selftest {label}")

    # The twenty-fifth half's cases: this repository's own tooling, where a comment, a
    # docstring and the message a tool prints are all prose written here.
    check_own("a hash comment in our own tool", "demo.py",
              "# C 1: Emacs's K&R style is what this reads\n", 1)
    check_own("a docstring is prose here", "demo.py",
              '"""The macros Emacs defines here.\n"""\n', 1)
    check_own("a message the tool prints", "demo.py",
              'print(f"error: {ref} does not look like an Emacs checkout")\n', 1)
    check_own("a citation in our own tool's prose", "demo.py",
              "# C 1: `Emacs` is what C calls itself\n", 0)
    check_own("our own C probe's comment", "demo.c",
              "/* probe of doprnt.c; not about the Emacs mirrors.  */\n", 1)
    check_own("a multi-line string is one message", "demo.rs",
              "    eprintln!(\"needs Emacs configured, \\\n                 built\");\n", 1)

    def check_reporting(label: str, snippet: str, ported: bool = False) -> None:
        """The reporter consumes a finding; clipped prose is not a second detector."""
        from contextlib import redirect_stdout
        from io import StringIO
        from unittest.mock import patch

        nonlocal cases, failed
        cases += 1
        relative = Path("src/demo.rs" if ported else "tools/demo.py")

        class Source:
            def relative_to(self, _root):
                return relative

            def read_text(self, **_options):
                return "// " + BARE_PRODUCT_NAME

        target = Source() if ported else REPO / relative
        replacements = {
            "files": lambda: [target] if ported else [],
            "own_files": lambda: [] if ported else [target],
            "unscanned_own": lambda: [],
            "read_own": lambda _path: ((1, 0, 1), [(1, snippet)]),
            "reference_of": lambda _path: None,
            "reference_set": lambda _path: [],
            "scan_file": lambda _path, _references: [(1, snippet)],
        }
        output = StringIO()
        with patch.dict(globals(), replacements), redirect_stdout(output):
            status = main([])
        text = output.getvalue()
        if (BARE_NAME.search(snippet) is not None or status != 1
                or f"FAIL {relative}:1:" not in text
                or f"`{BARE_PRODUCT_NAME}`" not in text
                or not text.endswith("1 failure(s)\n")):
            failed += 1
            print(f"FAIL selftest {label}: reporting lost or re-parsed a finding")
        else:
            print(f"ok   selftest {label}")

    check_reporting("a tool finding beyond the clipped prefix", "x" * 180)
    check_reporting("normalized context is not a second detector",
                    normalize("GNU   " + BARE_PRODUCT_NAME))
    check_reporting("a ported finding beyond the clipped prefix", "x" * 180, ported=True)

    cases += 1
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for tree, name in (("tools", "demo.py"), ("tools", "demo.sh"), ("tools", "demo.txt"),
                           ("spikes/miri", "Cargo.lock"), ("xtask/src", "main.rs")):
            (root / tree).mkdir(parents=True, exist_ok=True)
            (root / tree / name).write_text("# x\n", encoding="utf-8")
        saved = (REPO, REF)
        REPO = root
        try:
            walked = sorted(p.name for p in own_files())
            left = unscanned_own()
        finally:
            REPO, REF = saved
        if walked != ["demo.py", "demo.sh", "main.rs"] or \
           sorted(left) != ["spikes/miri/Cargo.lock", "tools/demo.txt"]:
            failed += 1
            print(f"FAIL selftest the walk binds the tool suffixes: {walked} / {left}")
        else:
            print("ok   selftest the walk binds the tool suffixes")

    print(f"{cases} case(s), {failed} failure(s)")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
