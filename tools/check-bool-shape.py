#!/usr/bin/env python3
"""Compare how `&&` and `||` are grouped, per function, between the reference and the port.

Why this exists (`PORTING.md` P-321): the token gate compares identifiers and numbers, and
punctuation never enters its sequence.  So `a && (b || c)` and `a && b || c` compare equal -
which is exactly how `kbd_buffer_get_event`'s PINCH branch reached a device, dereferenced nil on
the first key press, and left 590/590 green (status 4.960).

The comparison is structural, not textual, because the two languages spell things differently:

  * only `(`, `)`, `&&` and `||` are significant; every other run of characters collapses to one
    placeholder, so `(*x).field` and `x->field` are the same operand;
  * a parenthesised group with no operator inside is *transparent* - it is just a placeholder
    written with brackets - so wrapping an operand in another pair of parentheses changes nothing;
  * a group that does contain operators becomes a nested expression, which is what makes a
    regrouping visible;
  * `&&` binds tighter than `||`, as in both languages, so the tree is built with that precedence.

A function is reported only when the two sides contain the same operators and the trees differ.
A different operator set means a different expression, which is another question and not this
check's business.

This is a **diagnostic, not a gate** (status 4.968, `PORTING.md` P-322).  Its self-test pins the
real bug shape and three control cases, but across a whole unit it is too noisy to fail a build:
`src/keyboard.c` alone reports 24 hits, and the sample inspected by hand were all rewrite noise -
C's `for (...; !bad_stuff && 0 < i; ...)` against the port's `while bad_stuff == 0 && 0 < i`, for
instance, where the shapes differ only because a `for` header carries semicolons.  Reading a
handful of hits on one unit when a regrouping is suspected is what it is good for; wiring it into
`gate-fast` would trade a rare true positive for a permanent background of false ones.

Usage
-----
    python3 tools/check-bool-shape.py --self-test
    python3 tools/check-bool-shape.py --pair src/keyboard.c  # one unit: read the hits by eye
"""

from __future__ import annotations

import argparse
import importlib.util
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def load_extract_abi():
    spec = importlib.util.spec_from_file_location("extract_abi", REPO / "tools" / "extract-abi.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def matching(text: str, start: int) -> int:
    """Index of the `)` that closes the `(` at START, or -1."""
    depth = 0
    for i in range(start, len(text)):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i
    return -1


def parse_expr(text: str) -> str:
    """A canonical rendering of the `&&`/`||` structure of TEXT ('' when it has none)."""
    operands: list[str] = []
    ops: list[str] = []
    i = 0
    n = len(text)
    saw_operand = False
    while i < n:
        if text.startswith("&&", i) or text.startswith("||", i):
            ops.append(text[i:i + 2])
            i += 2
            saw_operand = False
            continue
        if text[i] == "(":
            close = matching(text, i)
            if close < 0:
                i += 1
                continue
            inner = parse_expr(text[i + 1:close])
            if inner:
                operands.append(inner)
            elif saw_operand:
                pass
            else:
                operands.append("X")
            saw_operand = True
            i = close + 1
            continue
        if text[i].isalnum() or text[i] in "_.[]*&!~-+<>=":
            if not saw_operand:
                operands.append("X")
                saw_operand = True
            i += 1
            continue
        i += 1
    if not ops:
        # A body's boolean expression usually sits inside a keyword's parentheses
        # (`if (a && b)`, `while (...)`, `return (...)`), and the keyword itself reads as a
        # placeholder operand.  With no operator at this level, the expression of interest is the
        # largest nested tree among the operands.
        nested = [o for o in operands if o.startswith(("AND(", "OR("))]
        return max(nested, key=len) if nested else ""
    if len(operands) != len(ops) + 1:
        return ""
    # `&&` binds tighter than `||`: fold the ands first, then the ors.
    parts: list[str] = [operands[0]]
    for op, operand in zip(ops, operands[1:]):
        if op == "&&":
            parts[-1] = f"AND({parts[-1]},{operand})"
        else:
            parts.append(operand)
    return "OR(" + ",".join(parts) + ")"


def shape_of(text: str) -> tuple[str, int]:
    """(tree, operator count) for a body: the tree of its outermost boolean expression."""
    tree = parse_expr(text)
    return tree, text.count("&&") + text.count("||")


def compare(c_body: str, r_body: str) -> str | None:
    c_tree, c_ops = shape_of(c_body)
    r_tree, r_ops = shape_of(r_body)
    if c_ops == 0:
        return None
    # Same number of operators is not enough: `a && b` and `a || b` both have one.  The two
    # sides must use the same mix, otherwise the expression is a different one.
    if (c_body.count("&&"), c_body.count("||")) != (r_body.count("&&"), r_body.count("||")):
        return None
    if not c_tree or not r_tree:
        return None
    if c_tree == r_tree:
        return None
    return f"C {c_tree[:120]} vs port {r_tree[:120]}"


def self_test() -> int:
    cases = [
        ("if (a && (b || c)) x;", "if (a && b || c) x;", True),
        ("if (a && (b || c)) x;", "if (((a) && ((b || c)))) { x; }", False),
        ("if (a && b) x;", "if (a || b) x;", None),
        ("if (x->y || f (a, b)) z;", "if ((*x).y || f (a, b)) z;", False),
    ]
    bad = 0
    for c, r, want in cases:
        got = compare(c, r)
        flagged = got is not None
        ok = (want is None and got is None) or (want is not None and flagged == want)
        if not ok:
            bad += 1
            print(f"SELFTEST FAIL want={want} got={got!r} for {c!r} / {r!r}")
    print(f"self-test: {len(cases) - bad}/{len(cases)} case(s) as expected")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--pair", action="append", default=[])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    ea = load_extract_abi()
    if a.pair:
        pairs = [(REPO / "emacs" / p, (REPO / p).with_suffix(".rs")) for p in a.pair]
        pairs = [(c, r) for c, r in pairs if c.is_file() and r.is_file()]
    else:
        pairs = []
        for c in sorted((REPO / "emacs").rglob("*")):
            if c.suffix not in (".c", ".h"):
                continue
            rs = (REPO / c.relative_to(REPO / "emacs")).with_suffix(".rs")
            if rs.is_file():
                pairs.append((c, rs))
    bad = 0
    checked = 0
    for c_path, rs_path in pairs:
        c_text = ea._text_for(c_path)
        rs = ea.rust_inventory(rs_path)
        for name in sorted(set(rs["bodies_all"])):
            c_body = ea.c_body_text(c_text, name)
            if c_body is None:
                continue
            for r_body in rs["bodies_all"][name]:
                checked += 1
                reason = compare(c_body, r_body)
                if reason:
                    bad += 1
                    print(f"SHAPE  {c_path.relative_to(REPO)} {name}: {reason}")
                    break
    print(f"boolean shapes: {checked} body comparison(s), {bad} with a different grouping")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
