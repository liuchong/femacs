#!/usr/bin/env python3
"""Fail when the numbers the hand-off documents quote disagree with the generated tables.

The same four counts appear in several places - the unresolved imports, of which the window-system
arms and the upstream gaps are parts, and the number of rows in the window-arm work order.  Every
one of them is derivable from a table under `docs/baseline/`, so a document that quotes a stale
number is a red gate rather than a note someone has to notice (PORTING.md P-320's "stale table"
class).

    python3 tools/check-doc-numbers.py
"""

from __future__ import annotations

import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
PENDING = REPO / "docs/baseline/android-imports-pending.txt"
# The ratchet is read against the API 21 stubs (the strictest basis the link has), and the verdict
# against the delivery device's own libraries.  Both are real numbers a document may quote
# (71 and 53 at status 4.749), so the gate accepts either and only reports a third value.
DEVICE = REPO / "docs/baseline/android-imports-device.txt"
ARMS = REPO / "docs/baseline/android-arms.tsv"
PLAN = REPO / "docs/baseline/window-arms-plan.tsv"
DOCS = ("docs/current-state.md", "docs/handoff.md", "docs/android-app-readiness.md",
        "docs/window-arms-plan.md", "docs/decisions-needed.md", "docs/acceptance.md",
        "docs/goal.md")
WINDOW = ("WINDOW_SYSTEM", "HAVE_X_WINDOWS", "HAVE_NS", "HAVE_PGTK", "HAVE_HAIKU", "HAVE_NTGUI",
          "WINDOWSNT", "MSDOS")


def read_names(path: pathlib.Path) -> list[str]:
    return [l.strip() for l in path.read_text(encoding="utf-8").splitlines()
            if l.strip() and not l.startswith("#")]


def rows(path: pathlib.Path) -> list[list[str]]:
    return [l.split("\t") for l in path.read_text(encoding="utf-8").splitlines()
            if l.strip() and not l.startswith("#")]


def facts() -> tuple[dict[str, int], list[str], dict[str, set[int]]]:
    names = read_names(PENDING)
    # Accepted values per count: the ratchet's, plus the device reading for the total.
    accepted: dict[str, set[int]] = {}
    if DEVICE.is_file():
        accepted["unresolved"] = {len(read_names(DEVICE))}
    arms = rows(ARMS)
    conds = {r[0]: r[3] for r in arms if len(r) >= 4}
    window = [n for n in names if any(w in conds.get(n, "") for w in WINDOW)
              or n.startswith(("Vx_", "x_")) or n == "tty_draw_row_with_mouse_face"]
    plan = rows(PLAN)
    # A queue row for a name the ratchet already lists as provided is a stale table (P-320): the
    # work it describes is done, yet it keeps counting in every document that quotes the row count.
    stale = [r[0] for r in arms if r[0] not in names]
    return ({"unresolved": len(names), "window": len(window),
             "upstream": len(names) - len(window), "plan": len(plan)}, stale, accepted)


def main() -> int:
    f, stale, accepted = facts()
    print(f"tables: {f['unresolved']} unresolved = {f['window']} window + {f['upstream']} upstream; "
          f"plan rows {f['plan']}")
    failures = []
    for name in stale:
        failures.append(f"{ARMS.name} still has a row for {name}, which the ratchet no longer lists")
    if f["plan"] != f["window"]:
        failures.append(f"{PLAN.name} has {f['plan']} rows for {f['window']} window names")
    # Each pattern is a claim a document makes; the group must equal the table's number.
    claims = (
        (re.compile(r"(\d+)\s*条未决"), "unresolved"),
        (re.compile(r"(\d+)\s*个无人提供"), "unresolved"),
        (re.compile(r"窗口臂\s*(\d+)"), "window"),
        (re.compile(r"窗口系统臂\s*(\d+)"), "window"),
        (re.compile(r"上游缺口\s*(\d+)"), "upstream"),
    )
    quoted: set[str] = set()
    for rel in DOCS:
        if not (REPO / rel).exists():
            failures.append(f"{rel} is listed for checking but does not exist")
            continue
        text = (REPO / rel).read_text(encoding="utf-8")
        for pattern, key in claims:
            for m in pattern.finditer(text):
                quoted.add(key)
                got = int(m.group(1))
                if got != f[key] and got not in accepted.get(key, set()):
                    also = (" or " + "/".join(str(v) for v in sorted(accepted[key]))
                            if accepted.get(key) else "")
                    failures.append(f"{rel}: says {got} for {key}, the table says {f[key]}{also}")
    # Deleting the sentence must not be a way to pass: every count has to be quoted somewhere.
    for key in ("unresolved", "window", "upstream"):
        if key not in quoted:
            failures.append(f"no document quotes the {key} count ({f[key]}) any more")
    # The completion-definition fraction is quoted in more than one document, and 4.972 found the
    # table and its own conclusion disagreeing (a row had been updated to done while the paragraph
    # under it still counted it partial).  Whichever fraction a document states must match the one
    # `docs/current-state.md`'s table implies, so a row can never be updated alone.
    dod = re.compile(r"(\d+)/8\s*=\s*([0-9.]+)%")
    stated: dict[str, set[str]] = {}
    for rel in DOCS:
        path = REPO / rel
        if not path.exists():
            continue
        for m in dod.finditer(path.read_text(encoding="utf-8")):
            stated.setdefault(rel, set()).add(m.group(1) + "/8")
    current = REPO / "docs" / "current-state.md"
    if current.exists():
        head = dod.search(current.read_text(encoding="utf-8"))
        if head:
            want = head.group(1) + "/8"
            for rel, got in stated.items():
                for one in sorted(got):
                    if one != want:
                        failures.append(f"{rel}: says {one} for the completion definition, "
                                        f"docs/current-state.md's headline says {want}")
    if failures:
        print("doc numbers disagree with the tables:")
        for line in failures:
            print("  " + line)
        return 1
    print(f"document numbers agree with the tables ({len(DOCS)} doc(s) checked)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
