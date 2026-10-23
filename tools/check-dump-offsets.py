#!/usr/bin/env python3
"""Check every `dump_offsetof!` site against its C counterpart.

`pdumper.c` reaches a field's offset through `#define dump_offsetof (type, member)
(ptrdiff_t_to_dump_off (offsetof (type, member)))` (C 177-178) plus a few direct
`offsetof (struct X, path)` uses, and the *path* is what matters: `Lisp_String`'s data
pointer sits at `u.s.data`, not at `u`.  The port's macro took a single identifier at
first, so three sites silently collapsed to the union's offset (0) and wrote their fixup
into the wrong field - `Lisp_String`'s interval tree over `size`, and the cold string's
data pointer over `size`, which left every dumped string with a NULL `data` and made the
dump unloadable (`hash_char_array (NULL, len)`, status 3.644).

Readings: `N C site(s), N port site(s), 0 failure(s)`.  Failures are a port site whose
`(type, path)` is not one C reaches, a C `dump_offsetof` site the port does not carry, or
a C site out of order.  `--selftest` proves the four shapes against the real C text.
"""
import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
REFERENCE = REPO / "emacs" / "src" / "pdumper.c"
PORT = REPO / "src" / "pdumper.rs"

# `dump_offsetof (struct X, path)` - the macro's own call sites.
C_MACRO = re.compile(r"dump_offsetof\s*\(\s*(?:struct\s+)?([A-Za-z_]\w*)\s*,\s*([\w.]+)\s*\)")
# Every other `offsetof (struct X, path)` (the macro body itself is filtered out by name).
C_DIRECT = re.compile(r"(?<![\w])(?<!dump_)offsetof\s*\(\s*struct\s+([A-Za-z_]\w*)\s*,\s*([\w.]+)\s*\)")
PORT_SITE = re.compile(r"dump_offsetof!\s*\(\s*([A-Za-z_]\w*)\s*,\s*([\w.]+)\s*\)")


def without_macro_definition(text: str) -> str:
    """C's `#define dump_offsetof (type, member) ...` is not a call site: drop it and its
    continuation lines, or the macro's own parameter names look like a site named `type`."""
    out, skipping = [], False
    for line in text.splitlines():
        if skipping:
            skipping = line.rstrip().endswith("\\")
            continue
        if re.match(r"\s*#\s*define\s+dump_offsetof\b", line):
            skipping = line.rstrip().endswith("\\")
            continue
        out.append(line)
    return "\n".join(out)


def pairs(pattern: re.Pattern, text: str) -> list[tuple[str, str]]:
    return [(m.group(1), m.group(2).strip()) for m in pattern.finditer(text)]


def is_subsequence(needle: list, haystack: list) -> bool:
    it = iter(haystack)
    return all(any(x == y for y in it) for x in needle)


def findings(c_text: str, port_text: str) -> list[str]:
    c_macro = pairs(C_MACRO, without_macro_definition(c_text))
    c_direct = pairs(C_DIRECT, c_text)
    port = pairs(PORT_SITE, port_text)
    known = set(c_macro) | set(c_direct)
    out = []
    for kind, path in port:
        if (kind, path) not in known:
            out.append(f"port site ({kind}, {path}) is not an offset any C site reaches")
    for item in c_macro:
        if item not in port:
            out.append(f"C {item[0]}.{item[1]} has no port site")
    if not is_subsequence([p for p in port if p in set(c_macro)], c_macro):
        out.append("the port's sites are not in the reference's order")
    return out


def selftest() -> int:
    c_text = REFERENCE.read_text(encoding="utf-8")
    port_text = PORT.read_text(encoding="utf-8")
    cases = [
        ("the real pair", c_text, port_text, 0),
        # Two findings: the collapsed site is one C never reaches, and C's own site is gone.
        ("union path collapsed back to the union",
         c_text, port_text.replace("dump_offsetof! (Lisp_String, u.s.data)",
                                   "dump_offsetof! (Lisp_String, u)"), 2),
        ("a site dropped", c_text,
         port_text.replace("         offset + dump_offsetof! (Lisp_String, u.s.intervals),",
                           "         offset,"), 1),
        ("two sites swapped", c_text,
         port_text.replace("dump_offsetof! (interval, left)", "dump_offsetof! (interval, TMP)")
                  .replace("dump_offsetof! (interval, right)", "dump_offsetof! (interval, left)")
                  .replace("dump_offsetof! (interval, TMP)", "dump_offsetof! (interval, right)"), 1),
    ]
    bad = 0
    for name, c, p, want in cases:
        got = len(findings(c, p))
        ok = got == want
        bad += 0 if ok else 1
        print(f"{'ok  ' if ok else 'FAIL'} {name}: {got} failure(s) (want {want})")
    print(f"selftest: {len(cases)} case(s), {bad} failure(s)")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args()
    if args.selftest:
        return selftest()

    c_text = REFERENCE.read_text(encoding="utf-8")
    port_text = PORT.read_text(encoding="utf-8")
    rows = findings(c_text, port_text)
    print(f"read {REFERENCE.relative_to(REPO)} and {PORT.relative_to(REPO)}: "
          f"{len(pairs(C_MACRO, without_macro_definition(c_text)))} C dump_offsetof site(s), "
          f"{len(pairs(C_DIRECT, c_text))} C direct offsetof site(s), "
          f"{len(pairs(PORT_SITE, port_text))} port site(s)")
    for row in rows:
        print(f"FAIL {row}")
    print(f"{len(rows)} failure(s)")
    return 1 if rows else 0


if __name__ == "__main__":
    sys.exit(main())
