#!/usr/bin/env python3
"""The `DEFVAR_*` forwarding objects must be initialized in their *static* initializer.

C's macros spell the forwarding object as a static with the pointer already in it:

    static struct Lisp_Fwd const o_fwd = {Lisp_Fwd_Obj, .u.objvar = &globals.f_##vname};

(lisp.h 3495-3531, and `DEFVAR_KBOARD`'s `offsetof` the same way), so the pointer is a
link-time constant that is right in any process - including one that has just *loaded a
dump*, where `syms_of_*` never runs again.  The port assigned the union member in the macro
body instead; every one of these objects then held NULL after a dump load and
`do_symval_forwarding` dereferenced it (status 3.644).

For each of the five macros in `femacs/src/include/lisp.rs` this reads: the `static mut
{ O,I,B,KO }_FWD` initializer names the macro's own member and does not set it to a null/zero
constant, and the macro body has no `*_FWD.u.<member> = …` assignment.  Readings: `5 macro(s),
0 failure(s)`; `--selftest` runs the three shapes (the fixed spelling, the runtime assignment,
and a null initializer).
"""
import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MIRROR = REPO / "femacs" / "src" / "include" / "lisp.rs"
MACROS = ("DEFVAR_LISP", "DEFVAR_LISP_NOPRO", "DEFVAR_INT", "DEFVAR_BOOL", "DEFVAR_KBOARD")
NULLISH = re.compile(r"^(std::ptr::null_mut\s*\(\)|0|null\s*\(\))$")


def macro_bodies(text: str) -> dict[str, str]:
    """`DEFVAR_*` -> its `{{ … }}` body, in file order."""
    out: dict[str, str] = {}
    for name in MACROS:
        m = re.search(r"macro_rules!\s+%s\s*\{" % name, text)
        if not m:
            continue
        i, depth = m.end() - 1, 0
        for j in range(i, len(text)):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    out[name] = text[i:j + 1]
                    break
    return out


def findings(text: str) -> list[str]:
    bodies = macro_bodies(text)
    out = []
    for name in MACROS:
        body = bodies.get(name)
        if body is None:
            out.append(f"macro {name} is gone from the mirror")
            continue
        static = re.search(r"static mut (\w+): Lisp_Fwd = Lisp_Fwd \{(.*?)\n        \};", body, re.S)
        if static is None:
            out.append(f"{name}: no `static mut …: Lisp_Fwd` initializer")
            continue
        maker, init = static.group(1), static.group(2)
        member = re.search(r"u: Lisp_Fwd_u \{ (\w+): ([^}]*)\}", init, re.S)
        if member is None:
            out.append(f"{name}: the initializer does not name a union member")
            continue
        field, value = member.group(1), member.group(2).strip()
        if NULLISH.match(value):
            out.append(f"{name}: {maker}.u.{field} is initialized to a null/zero value")
        assign = re.search(r"%s\.u\.\w+\s*=" % re.escape(maker), body)
        if assign is not None:
            out.append(f"{name}: {maker}'s member is assigned in the macro body "
                       f"(a loaded dump never runs `syms_of_*`)")
    return out


def selftest() -> int:
    text = MIRROR.read_text(encoding="utf-8")
    cases = [
        ("the mirror as it stands", text, 0),
        ("both LISP macros nulled and one assigned at run time",
         text.replace("            u: Lisp_Fwd_u { objvar: std::ptr::addr_of_mut! ($vname) },",
                      "            u: Lisp_Fwd_u { objvar: std::ptr::null_mut () },")
             .replace("        let _: &core::ffi::CStr = $lname;\n        unsafe {\n            defvar_lisp",
                      "        let _: &core::ffi::CStr = $lname;\n        unsafe {\n"
                      "            O_FWD.u.objvar = std::ptr::addr_of_mut! ($vname);\n            defvar_lisp", 1), 3),
        ("one macro's initializer nulled",
         text.replace("            u: Lisp_Fwd_u { boolvar: std::ptr::addr_of_mut! ($vname) },",
                      "            u: Lisp_Fwd_u { boolvar: std::ptr::null_mut () },"), 1),
    ]
    bad = 0
    for name, variant, want in cases:
        got = len(findings(variant))
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
    text = MIRROR.read_text(encoding="utf-8")
    rows = findings(text)
    print(f"read {MIRROR.relative_to(REPO)}: {len(macro_bodies(text))} macro(s) checked")
    for row in rows:
        print(f"FAIL {row}")
    print(f"{len(rows)} failure(s)")
    return 1 if rows else 0


if __name__ == "__main__":
    sys.exit(main())
