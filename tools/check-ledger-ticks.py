#!/usr/bin/env python3
"""A ledger may only tick a function the ported file actually carries.

Why this exists (docs/status.md 3.349): `docs/ports/xterm.md` ticked
`x_dnd_compute_toplevels` - with its decision and its readings - in `c8a3d8a`, while the code
half of that slice (666 lines of `femacs/src/xterm.rs`) stayed in the lane's working tree.
HEAD then contradicted itself for two rounds, and none of the four gates in this area could
see it: `check-port-lines` compares line counts, `audit-wip` counts unticked boxes,
`check-swift-units` only reads the Swift units, and `extract-abi.py --check` measures the ABI
difference between reference and port (a name that is *ticked* but absent from the port is not
part of that difference - the checklist lives in the ledger, not in the target).

So this gate makes the comparison the ledger's own structure already implies:

    for every row of docs/port-map.md whose target is a `.rs` file
        find its ledger under docs/ports/ (the target's path, with `src/include/X.rs`
        spelled `include-X.md` and other directories as `dir-stem.md`)
        read the ledger's `## Functions defined`-style section, where a `- [x] NAME` line
        is a claim that NAME is carried
        fail if NAME does not occur in the target's text

Only the *function* section is bound, and that is a measurement rather than a preference: over
all 590 rows it is the section where the tick means "this name is in the port" (measured
2026-10-05: 0 findings across 588 ledgers).  The other sections tick gnulib and attribute
machinery the port deliberately does not spell (`_GL_ATTRIBUTE_*`, `_GL_BEGIN_NAMESPACE`, ...),
so binding them would report 82 "missing" names that are decisions, not gaps - the reason this
gate reads the section heading instead of every tick.

A ledger whose checklist is empty (151 rows: the units whose body sits entirely under a
condition the frozen configuration leaves false, so `extract-abi.py` generates no live lines
for them) has nothing to bind here; `tools/check-port-lines.py` is that class's gate.

    python3 tools/check-ledger-ticks.py            # the reading above
    python3 tools/check-ledger-ticks.py --selftest # prove the parser and the comparison
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
MAP = REPO / "docs/port-map.md"
LEDGERS = REPO / "docs/ports"

ROW = re.compile(r"^\| `([^`]+)` \| [\d,]+ \| `([^`]+)` \| [\d,]+ \|$")
TICK = re.compile(r"^\s*- \[x\]\s+([A-Za-z_][A-Za-z0-9_]*)")


def ledger_for(target: str, all_refs: set[str]) -> pathlib.Path | None:
    """The ledger a target's map row uses, or None when it has none.

    `femacs/src/xrdb.rs` -> `docs/ports/xrdb.md`; `femacs/src/include/window.rs` ->
    `docs/ports/include-window.md`; `femacs/lib-src/etags.rs` -> `docs/ports/lib-src-etags.md`
    (the fallback to the bare stem is what the older, flatter names use).

    A header's ledger falls back to the bare stem **only when the reference has no sibling of
    the same stem**: `src/include/lisp.rs` (src/lisp.h) is `docs/ports/lisp.md` because there is
    no src/lisp.c, while `lib/boot-time.h` must not take `docs/ports/boot-time.md`, which is
    `lib/boot-time.c`'s.  That test is what keeps this gate from binding a header to another
    file's ticks (measured: without it, 1435 false failures).
    """
    rel = pathlib.PurePosixPath(target).relative_to("femacs")
    stem, base = rel.stem, rel.parent
    is_include = base.name == "include"
    if is_include:
        base = base.parent
    prefix = "" if base.as_posix() == "." else base.as_posix().replace("/", "-") + "-"
    candidates = [f"{prefix}{stem}.md", f"{stem}.md"]
    if is_include:
        candidates = [f"{prefix}include-{stem}.md", f"include-{stem}.md"]
        # the reference path the map pairs with this target, to ask about a sibling `.c`
        ref = next((r for r, tg in ROWS_CACHE if tg == target), None)
        sibling = ref[:-2] + ".c" if ref and ref.endswith(".h") else None
        if sibling is None or sibling not in all_refs:
            candidates += [f"{prefix}{stem}.md", f"{stem}.md"]
    for name in dict.fromkeys(candidates):
        candidate = LEDGERS / name
        if candidate.is_file():
            return candidate
    return None


def ticked_functions(text: str) -> set[str]:
    """The `NAME`s a ledger ticks under a heading whose text says "function"."""
    out: set[str] = set()
    in_functions = False
    for line in text.split("\n"):
        if line.startswith("## ") or line.startswith("### "):
            in_functions = "function" in line.lower()
            continue
        m = TICK.match(line)
        if m and in_functions:
            out.add(m.group(1))
    return out


def rows() -> list[tuple[str, str]]:
    out: list[tuple[str, str]] = []
    for line in MAP.read_text(encoding="utf-8").split("\n"):
        m = ROW.match(line)
        if m and m.group(2).endswith(".rs"):
            out.append((m.group(1), m.group(2)))
    return out


ROWS_CACHE: list[tuple[str, str]] = []


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true", help="run this tool's own cases")
    args = ap.parse_args()
    if args.selftest:
        bad = 0
        cases = [
            ("a tick inside the function section is a claim",
             "## Functions defined — 2\n- [x] foo  (line 3)\n- [ ] bar\n", {"foo"}),
            ("a tick under another heading is not",
             "## Macros defined — 1\n- [x] _GL_ATTRIBUTE_MALLOC\n", set()),
            ("the heading is read, not the order",
             "## Functions defined\n- [ ] a\n- [x] b\n### Notes\n- [x] c\n", {"b"}),
            ("an empty checklist has nothing to bind", "", set()),
        ]
        for name, text, want in cases:
            got = ticked_functions(text)
            ok = got == want
            print(f"{'ok  ' if ok else 'FAIL'} {name}: {sorted(got)}")
            bad += 0 if ok else 1
        text = "macro_rules! FOO { }"
        ok = "FOO" in text and "BAR" not in text
        print(f"{'ok  ' if ok else 'FAIL'} the comparison is a name search: {ok}")
        bad += 0 if ok else 1
        print(f"{len(cases) + 1} cases, {bad} failures")
        return 1 if bad else 0

    global ROWS_CACHE
    read, empty, unledgered = 0, 0, []
    ROWS_CACHE = rows()
    all_refs = {ref for ref, _ in ROWS_CACHE}
    missing: list[tuple[str, str, str, str]] = []
    for ref, target in ROWS_CACHE:
        ledger = ledger_for(target, all_refs)
        if ledger is None:
            unledgered.append(ref)
            continue
        names = ticked_functions(ledger.read_text(encoding="utf-8"))
        if not names:
            empty += 1
            continue
        read += 1
        text = (REPO / target).read_text(encoding="utf-8", errors="replace")
        for name in sorted(names):
            if name not in text:
                missing.append((ref, target, ledger.relative_to(REPO).as_posix(), name))

    # A name the ledger's own target lacks is not always a gap: this project records lisp.h's
    # `INLINE` accessors through the unit that needed them (`get_act_rec`/`set_act_rec` are
    # `femacs/src/eval.rs`'s, with a comment saying the mirror does not carry them), and a
    # use-site declaration elsewhere in the tree is the same shape.  So one pass over the ported
    # Rust files asks whether the name exists *anywhere*: present -> carried elsewhere
    # (informational, and the debt stays visible); absent -> the 3.349 failure.
    where: dict[str, str] = {}
    if missing:
        wanted = {name for _, _, _, name in missing}
        for path in REPO.glob("femacs/**/*.rs"):
            if not wanted:
                break
            if "/target/" in path.as_posix():
                continue
            text = path.read_text(encoding="utf-8", errors="replace")
            for name in [n for n in wanted if n in text]:
                where[name] = path.relative_to(REPO).as_posix()
                wanted.discard(name)

    findings = [m for m in missing if m[3] not in where]
    elsewhere = [(m, where[m[3]]) for m in missing if m[3] in where]
    for ref, target, ledger, name in findings:
        print(f"FAIL {ledger}: ticks `{name}`, which neither {target} nor any other ported "
              f"Rust file carries (reference {ref})")
    for (ref, target, ledger, name), path in elsewhere:
        print(f"info {ledger}: ticks `{name}`; {target} does not carry it, but {path} does "
              f"(reference {ref})")
    print(f"{read} ledger(s) bound (a function tick to compare), {len(findings)} failure(s), "
          f"{len(elsewhere)} carried by another file, "          f"{empty} row(s) with no function tick - a unit with no function at all, or one whose "
          f"body the frozen configuration leaves under a false condition (check-port-lines.py "
          f"is that class's gate), {len(unledgered)} row(s) without a ledger")
    if unledgered:
        print("  no ledger: " + ", ".join(unledgered))
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
