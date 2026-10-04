#!/usr/bin/env python3
"""Transcribe `emacs/lib/cdefs.h`'s `#define` directives into the port's mirror.

`lib/cdefs.h` is glibc's compiler-attribute header: 900 lines that are almost entirely
`#define`, 83 of its 125 names carrying one arm per compiler family.  Hand-transcription is
1,200 lines of copy with no judgement in it, so this tool does the copy and the mirror keeps
the judgement (which arm is live, and why).

    python3 tools/gen-cdefs-mirror.py --dM <clang -dM output> > femacs/lib/include/cdefs.rs

Rules, all of them mechanical and named in the mirror's own header:

  * Backslash continuations are joined - they are not Rust tokens.
  * Block comments are dropped (the mirror's header records what the C comment said where it
    matters; `tools/check-declarations.py` is the gate for the prose that must not travel).
  * Each arm becomes one `macro_rules! NAME { (params) => { body }; }`, with C's parameter
    names rewritten to Rust metavariables (`$e`), `...` to `$($va:tt)*` and `__VA_ARGS__` to
    `$($va)*`.  Parameter spelling is not semantics: `extract-abi.py` strips a macro's own
    parameter names before comparing bodies.
  * The arm the frozen clang takes is emitted **ungated**; every other arm is emitted behind
    `#[cfg (feature = "<slug>")]`, where the slug comes from the arm's own condition
    (`defined __cplusplus` -> `cplusplus`, `!defined __GNUC__` -> `no_gnu_c`, ...) and the
    condition itself is quoted above the item.  Rust parses cfg'd-out macro arms, so the
    whole header is present in the file and only the live arm is compiled - P-51's shape.
  * The live arm is found by matching the arm's body against `clang -dM -E`'s surviving
    `#define`, not by trusting the file order (`c["macros"]` keeps the *last* definition, which
    for a frozen configuration is often not the live one).
  * C's integer suffixes (`1ULL`, `-1UL`) are stripped: Rust has no such literal, and
    `extract-abi.py`'s token comparison folds them anyway.
  * C's token paste `##` is written as two `#` tokens with spaces around them: Rust reserves
    the multi-hash token outright, cannot spell a paste in a `macro_rules!` body, and also
    reserves an identifier directly followed by `#` (measured: `__nldbl_##name` produced
    `error: prefix \`__nldbl_\` is unknown`).  These arms are never expanded, so the split is
    lexical only.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REFERENCE = Path("emacs/lib/cdefs.h")

DEFINE = re.compile(r"^\s*#\s*define\s+([A-Za-z_]\w*)(\s*\([^)]*\))?\s*(.*)$")
SUFFIX = re.compile(r"\b(\d+)(ULL|LL|UL|LU|U|L|F)\b")
SLUG_WORD = re.compile(r"[^A-Za-z0-9]+")


def join_continuations(text: str) -> str:
    return re.sub(r"\\\n", " ", text)


def strip_comments(text: str) -> str:
    return re.sub(r"/\*.*?\*/", " ", text, flags=re.S)


def slug(condition: str, index: int) -> str:
    """A feature name for an arm's condition: readable, stable, and never empty."""
    text = condition.strip()
    negated = text.startswith("!")
    text = text.lstrip("!")
    words = [w for w in SLUG_WORD.split(text) if w]
    if not words:
        return f"arm_{index}"
    if negated and words[0] in ("defined", "def"):
        return "no_" + ("_".join(words[1:]) or f"arm_{index}").lower()
    if words[0] in ("defined", "def"):
        words = words[1:]
    slug_text = "_".join(words[:6]).lower()[:60].strip("_")
    return slug_text or f"arm_{index}"


def arms(text: str) -> list[dict]:
    """Every `#define`, in file order, each tagged with the condition it sits under."""
    stack: list[dict] = []
    out: list[dict] = []
    for number, line in enumerate(text.splitlines(), 1):
        stripped = line.strip()
        head = re.match(r"#\s*(if|ifdef|ifndef|elif|else|endif)\b\s*(.*)$", stripped)
        if head:
            kind, rest = head.group(1), head.group(2).strip()
            if kind in ("if", "ifdef", "ifndef"):
                cond = rest if kind == "if" else f"{'!' if kind == 'ifndef' else ''}defined {rest}"
                stack.append({"cond": cond, "index": len(out)})
            elif kind == "elif" and stack:
                stack[-1]["cond"] = rest
                stack[-1]["negated"] = False
            elif kind == "else" and stack:
                stack[-1]["cond"] = "!" + stack[-1]["cond"]
                stack[-1]["negated"] = True
            elif kind == "endif" and stack:
                stack.pop()
            continue
        m = DEFINE.match(line)
        if m:
            out.append({
                "name": m.group(1),
                "args": (m.group(2) or "").strip(),
                "body": m.group(3).strip(),
                "line": number,
                "cond": stack[-1]["cond"] if stack else None,
            })
    return out


def measured(path: Path | None) -> dict[str, str]:
    if path is None:
        return {}
    out: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        m = re.match(r"#define\s+([A-Za-z_]\w*)(\([^)]*\))?\s*(.*)$", line)
        if m:
            out[m.group(1)] = m.group(3).strip()
    return out


def normalise(body: str) -> str:
    return re.sub(r"\s+", " ", SUFFIX.sub(r"\1", body)).strip()


def rust_item(arm: dict, gated: bool) -> str:
    name, args, body = arm["name"], arm["args"], arm["body"]
    params = [p.strip() for p in re.sub(r"^\(|\)$", "", args).split(",") if p.strip()] if args else []
    variadic = "..." in params
    params = [p for p in params if p != "..."]
    pattern = []
    for p in params:
        pattern.append(f"${p}:expr")
    if variadic:
        pattern.append("$($va:tt)*")
    out = body
    for p in params:
        out = re.sub(rf"\b{re.escape(p)}\b", f"${p}", out)
    out = out.replace("__VA_ARGS__", "$($va)*")
    # Rust forbids `##` outright ("reserved multi-hash token"), and the paste cannot be spelled
    # in a macro_rules body at all; these arms are never expanded, so C's token is written as
    # two `#` tokens, which Rust accepts.  The gate compares tokens, so the arms whose bodies
    # carry it and are compared (single-definition names) get a macro-body note.
    out = out.replace("##", " # # ")
    out = SUFFIX.sub(r"\1", out)
    where = f"under `{arm['cond']}`" if arm["cond"] else "unconditionally"
    lines = [f"/* C {arm['line']}: `#define {name}{args} {body}` - {where}.  */"]
    if gated:
        lines.append(f'#[cfg (feature = "{slug(arm["cond"] or "", arm["line"])}")]')
    if args:
        lines.append(f"macro_rules! {name} {{ ({', '.join(pattern)}) => {{ {out} }}; }}")
    else:
        lines.append(f"macro_rules! {name} {{ () => {{ {out} }}; }}")
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--reference", default=str(REFERENCE))
    ap.add_argument("--dM", type=Path, default=None,
                    help="clang -dM -E output, for the live-arm decision")
    ap.add_argument("--out", type=Path, default=None)
    args = ap.parse_args()

    text = strip_comments(join_continuations(Path(args.reference).read_text(encoding="utf-8")))
    every = arms(text)
    live = measured(args.dM)

    const_names = {"_SYS_CDEFS_H", "__LDBL_COMPAT"}
    chunks = []
    seen: set[str] = set()
    for arm in every:
        same = [a for a in every if a["name"] == arm["name"]]
        if arm["name"] in seen:
            continue
        seen.add(arm["name"])
        chosen = None
        if arm["name"] in live:
            want = normalise(live[arm["name"]])
            for a in same:
                if normalise(a["body"]) == want:
                    chosen = a
        chosen = chosen or same[-1]
        if arm["name"] in const_names:
            chunks.append(f"/* C {chosen['line']}: `#define {arm['name']} {chosen['body']}` - "
                          f"the gate asks for it among the constants.  */")
            chunks.append(f"pub const {arm['name']}: core::ffi::c_int = {chosen['body']};")
        else:
            for a in same:
                chunks.append(rust_item(a, gated=(a is not chosen)))
    body = "\n\n".join(c for c in chunks if c).rstrip() + "\n"
    if args.out:
        args.out.write_text(body, encoding="utf-8")
        print(f"wrote {args.out}: {len(body.splitlines())} lines, "
              f"{len(seen)} names, {len(every)} arms", file=sys.stderr)
    else:
        sys.stdout.write(body)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
