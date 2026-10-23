#!/usr/bin/env python3
"""The Kotlin targets' compile order, and how far the batch has gone.

A `.kt` target that names another target cannot compile before it (PORTING.md P-83 ⑥), so the batch
goes leaves-first; the graph comes from the reference's own sources, an identifier scan over
`java/org/gnu/emacs/*.java` for the target names.

    spikes/kotlin-target-order.py            # write docs/baseline/kotlin-order.tsv
    spikes/kotlin-target-order.py --check    # fail if the file is out of date or a ported file is missing
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
JAVA = REPO / "emacs" / "java" / "org" / "gnu" / "emacs"
KOTLIN = REPO / "platform" / "android" / "kotlin"
OUT = REPO / "docs" / "baseline" / "kotlin-order.tsv"


def deps() -> dict[str, list[str]]:
    sources = {p.stem: p.read_text(encoding="utf-8", errors="replace")
               for p in sorted(JAVA.glob("*.java"))}
    names = set(sources)
    targets = {n for n, text in sources.items()
               if f"platform/android/kotlin/{n}.kt" in
               (REPO / "docs" / "port-map.md").read_text(encoding="utf-8")}
    return {n: sorted({m for m in names - {n} if m in targets
                       and re.search(rf"\b{m}\b", sources[n])})
            for n in sorted(targets)}


def order(graph: dict[str, list[str]]) -> list[str]:
    out, done = [], set()
    while len(out) < len(graph):
        ready = sorted(n for n in graph if n not in done and all(d in done for d in graph[n]))
        if not ready:                       # a cycle: keep the rest in name order rather than loop
            ready = sorted(n for n in graph if n not in done)
        out += ready
        done.update(ready)
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    graph = deps()
    ported = {p.stem for p in KOTLIN.glob("*.kt")}
    lines = ["# The Kotlin targets' compile order (PORTING.md P-83 ⑥): a `.kt` file that names",
             "# another target cannot compile before it, so the batch goes leaves-first.  Written by",
             "# `spikes/kotlin-target-order.py`.  Columns: order, target, references, ported.",
             "order\ttarget\treferences\tported"]
    for i, name in enumerate(order(graph), 1):
        lines.append(f"{i}\tfemacs/platform/android/kotlin/{name}.kt\t"
                     f"{','.join(graph[name])}\t{'yes' if name in ported else ''}")
    text = "\n".join(lines) + "\n"
    if args.check:
        problems = []
        if not OUT.exists() or OUT.read_text(encoding="utf-8") != text:
            problems.append("kotlin-order.tsv is out of date (run the tool to refresh it)")
        for name in sorted(ported):
            if name not in graph:
                problems.append(f"{name}.kt exists but is not a Kotlin target in the port map")
        for p in problems:
            print(f"FAIL {p}")
        print(f"{len(graph)} target(s), {len(ported)} ported, {len(graph) - len(ported)} to go")
        return 1 if problems else 0
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}: {len(graph)} target(s), {len(ported)} ported")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
