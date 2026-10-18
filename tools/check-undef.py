#!/usr/bin/env python3
"""Fail when a built unit references a symbol that nothing in the frozen build can provide.

Why this exists (docs/status.md 3.57): every other gate compares *text* (the inventory gate), a
*type layout* (layout-check), a unit's *exported* symbol table (check-symbols.py) or the
declarations a unit owns (check-globals.py).  None of them ever looks at the symbols a unit
**references**, and a wrong name there is invisible until the link stage of the loading ladder -
which is the one stage no gate covers yet.  `emacs_fclose` is the worked example: `lisp.h`
`#define`s it to `fclose`, so C's object references `_fclose`, and a port that emits
`_emacs_fclose` is textually faithful, exports exactly the right symbols, and cannot link.

The measurement is differential, not a hand-written whitelist of libc names:

  * the **universe** is the union of
      - every symbol the built units define (`build/femacs/**/*.o`, both configurations),
      - every symbol the frozen reference objects define *or reference*
        (`build/emacs-ref/**/*.o` plus `lib/libgnu.a`; a reference object may legitimately
        reference a C library entry point that is not exported by libSystem itself, such as
        `_nc_*` from termcap, and the reference build is the proof that such a name resolves),
      - every name the SDK's `libSystem.B.tbd` exports, read from the `.tbd` the *linked*
        programs would use (read generously: the file also lists re-exported symbols, and a
        false friend here only ever widens the universe),
  * and a unit's `nm -u` entry that lands in none of them is an orphan: a name that no
    configuration of this program can satisfy.

Orphans are not automatically errors while the tree is being ported: a subr defined by a unit
that has not been ported yet is expected to be undefined (526 of them at the time this tool
landed, essentially all `_F*`).  `docs/baseline/undef-pending.txt` is therefore a **ratchet**,
the same instrument `docs/baseline/symbol-pending.txt` is for check-symbols.py: this tool fails
on a symbol that is *new* (a misspelling, an invented helper, a `#[link_name]` that is missing)
and reports the entries that have become stale so the file can shrink as units land.

Regenerate the ratchet with `--write-pending` after a batch has landed, and only after reading
what changed; the file is a record of what is still owed, not a silencer.
"""

from __future__ import annotations

import argparse
import glob
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent
OURS = "build/femacs"
REF = "build/emacs-ref"
PENDING = "docs/baseline/undef-pending.txt"


def nm(path: str) -> tuple[set[str], set[str]]:
    """(defined, undefined) global symbols of one object or archive."""
    out = subprocess.run(["nm", "-g", path], capture_output=True, text=True).stdout
    defined: set[str] = set()
    undefined: set[str] = set()
    for line in out.splitlines():
        field = line.split()
        if len(field) == 2 and field[0] == "U":
            undefined.add(field[1])
        elif len(field) == 3 and field[1] != "U":
            defined.add(field[2])
    return defined, undefined


def platform_symbols(sdk_lib: str) -> set[str]:
    """Every name the SDK's libSystem exports, read from its `.tbd`."""
    try:
        text = Path(sdk_lib).read_text(encoding="utf-8", errors="replace")
    except OSError:
        return set()
    return set(re.findall(r"_[A-Za-z_][A-Za-z_0-9]*", text))


def measure() -> tuple[set[str], set[str], int, int, dict[str, set[str]]]:
    """Return (orphans, known, #our objects, #reference inputs, referrers).

    `referrers` maps each orphan to the objects that reference it, which is what makes the
    ratchet's delta reviewable: a new orphan whose referrer is the unit that just landed is a
    dependency on a unit that is still unported, while a new orphan whose referrer has been
    built for weeks is a defect in that unit.
    """
    ours = sorted(glob.glob(os.path.join(REPO, OURS, "**", "*.o"), recursive=True))
    # A cross-target object (`<stem>.<target>.o`, `xtask build-tu` under FEMACS_TARGET) holds
    # another libc's symbols: its `__error`, `accept` and `abort` are bionic's, not libSystem's,
    # so measuring it against this machine's SDK list is a category error - and it made
    # `gate-fast` read 446 new unresolved names the moment the Android sweep had run (measured).
    # Those objects are checked by their own target's link (`xtask link-shared`, whose 69 JNI
    # exports and the APK carrying them are the evidence), so this gate reads host objects.
    import re as _re
    _cross = _re.compile(r"\.[a-z0-9_]+-[a-z0-9-]*android\.o$|\.[a-z0-9_]+-linux-[a-z0-9-]+\.o$")
    ours = [o for o in ours if not _cross.search(o)]
    refs = sorted(glob.glob(os.path.join(REPO, REF, "**", "*.o"), recursive=True))
    refs += sorted(glob.glob(os.path.join(REPO, REF, "**", "*.a"), recursive=True))
    if not ours:
        sys.exit(f"{OURS}/ has no objects: build the units first (`xtask gate-fast`).")
    defined, undefined = set(), set()
    referrers: dict[str, set[str]] = {}
    for path in ours:
        d, u = nm(path)
        defined |= d
        undefined |= u
        base = os.path.basename(path)
        for name in u:
            referrers.setdefault(name, set()).add(base)
    ref_defined, ref_undefined = set(), set()
    for path in refs:
        d, u = nm(path)
        ref_defined |= d
        ref_undefined |= u
    sdk = os.popen("xcrun --show-sdk-path").read().strip()
    plat = platform_symbols(f"{sdk}/usr/lib/libSystem.B.tbd")
    if not plat:
        sys.exit("libSystem.B.tbd is unreadable: the platform half of the universe is unknown.")
    known = defined | ref_defined | ref_undefined | plat
    return undefined - known, known, len(ours), len(refs), referrers


def read_pending(path: Path) -> list[str]:
    if not path.is_file():
        return []
    return [line.strip() for line in path.read_text(encoding="utf-8").splitlines()
            if line.strip() and not line.startswith("#")]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pending", default=PENDING)
    ap.add_argument("--write-pending", action="store_true",
                    help="rewrite the ratchet from the current measurement")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    orphans, known, n_our, n_ref, referrers = measure()
    path = REPO / args.pending
    if args.write_pending:
        header = ("# Undefined references no configuration of this program can satisfy, yet.\n"
                  "# Written by `python3 tools/check-undef.py --write-pending` (docs/status.md\n"
                  "# 3.59).  One symbol per line, `nm` spelling.  An entry disappears when the\n"
                  "# unit that owns the definition lands; a NEW symbol here is a defect (a\n"
                  "# misspelling, an invented helper, a missing #[link_name]).\n")
        path.write_text(header + "\n".join(sorted(orphans)) + "\n")
        print(f"wrote {path.relative_to(REPO)}: {len(orphans)} pending symbol(s)")
        return 0

    pending = set(read_pending(path))
    new = sorted(orphans - pending)
    stale = sorted(pending - orphans)
    if args.json:
        import json
        print(json.dumps({"orphans": sorted(orphans), "new": new, "stale": stale,
                          "known": len(known)}, indent=2))
        return 1 if new else 0
    if stale:
        print(f"check-undef: {len(stale)} ratchet entr{('y' if len(stale) == 1 else 'ies')} "
              f"are stale (now satisfied): {' '.join(stale[:8])}"
              + (" …" if len(stale) > 8 else ""))
    if new:
        print(f"check-undef: {len(new)} NEW undefined reference(s) that nothing can satisfy:")
        for name in new[:40]:
            who = ", ".join(sorted(referrers.get(name, ())))
            print(f"  - {name:38s} referenced by {who}")
        if len(new) > 40:
            print(f"  … {len(new) - 40} more")
        print("  A new entry is either a dependency on a unit that is not ported yet (then run\n"
              "  `--write-pending` after reading the list) or a defect in the referring unit\n"
              "  (a misspelling, an invented helper, a missing #[link_name]).")
        return 1
    print(f"check-undef: {n_our} unit object(s), {n_ref} reference object(s)/archive(s): "
          f"{len(orphans)} pending, 0 new")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
