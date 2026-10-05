#!/usr/bin/env python3
"""The `Lisp_Fwd` objects the `DEFVAR_*` macros build must be initialized *statically*.

C spells every one of them as a static with its fields already in the initializer:

    static struct Lisp_Fwd const o_fwd = {Lisp_Fwd_Obj, .u.objvar = &globals.f_##vname};
    static const struct Lisp_Fwd bo_fwd = { .type = Lisp_Fwd_Buffer_Obj,
                                            .u.buf = { .offset = offsetof (struct buffer,
                                                                           vname##_),
                                                       .predicate = FWDPRED_##predicate_ } };

(lisp.h 3495-3531 for the five `DEFVAR_*`, buffer.c 5016-5031 for `DEFVAR_PER_BUFFER`), so the
pointers/offsets are link-time constants that are right in any process - including one that has
just *loaded a dump*, where `syms_of_*` never runs again.  The port assigned them in the macro
bodies instead, and each instance broke the loaded process in its own way:

  * `DEFVAR_LISP`/`_NOPRO`/`INT`/`BOOL`/`KBOARD`: every forwarding object held NULL after a load,
    and `do_symval_forwarding` dereferenced it (status 3.644).
  * `DEFVAR_PER_BUFFER`: `offset` stayed 0, so `store_symval_forwarding` wrote each buffer-local
    variable's value into the *buffer's first word* - `set_minibuffer_mode`'s `setq-local`s
    corrupted the minibuffer buffer's pseudovector header, and `Flocal_variable_p`'s
    `CHECK_BUFFER` then failed on it (status 3.646).

For each macro this reads: the `static …: Lisp_Fwd = Lisp_Fwd { … }` initializer names its union
member with a non-null/non-zero value (for `buf`, both `offset` and `predicate`), and the macro
body has no `*_FWD.u.<member> = …` assignment.  Readings: `6 macro(s), 0 failure(s)`;
`--selftest` runs five shapes against the real files.
"""
import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
LISP_MIRROR = REPO / "femacs" / "src" / "include" / "lisp.rs"
BUFFER_UNIT = REPO / "femacs" / "src" / "buffer.rs"
SITES = [(LISP_MIRROR, name) for name in
         ("DEFVAR_LISP", "DEFVAR_LISP_NOPRO", "DEFVAR_INT", "DEFVAR_BOOL", "DEFVAR_KBOARD")]
SITES.append((BUFFER_UNIT, "DEFVAR_PER_BUFFER"))

NULLISH = re.compile(r"^(std::ptr::null_mut\s*\(\)|0|null\s*\(\))$")
STATIC = re.compile(r"static (?:mut )?(\w+)\s*:\s*Lisp_Fwd\s*=\s*Lisp_Fwd\s*\{(.*?)\n\s*\};", re.S)


def macro_body(text: str, name: str):
    m = re.search(r"macro_rules!\s+%s\s*\{" % re.escape(name), text)
    if m is None:
        return None
    i, depth = m.end() - 1, 0
    for j in range(i, len(text)):
        if text[j] == "{":
            depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0:
                return text[i:j + 1]
    return None


def findings(text: str, name: str) -> list[str]:
    body = macro_body(text, name)
    if body is None:
        return [f"macro {name} is gone from the file"]
    static = STATIC.search(body)
    if static is None:
        return [f"{name}: no `static …: Lisp_Fwd = Lisp_Fwd {{ … }}` initializer"]
    maker, init = static.group(1), static.group(2)
    out = []
    member = re.search(r"u:\s*Lisp_Fwd_u\s*\{\s*(\w+):\s*(.*)", init, re.S)
    if member is None:
        out.append(f"{name}: the initializer does not name a union member")
    else:
        field, value = member.group(1), member.group(2)
        if value.strip().startswith("Lisp_Fwd_buf"):        # DEFVAR_PER_BUFFER's nested record
            for sub in ("offset", "predicate"):
                got = re.search(r"%s:\s*([^,}]+)" % sub, value)
                if got is None:
                    out.append(f"{name}: the initializer does not set `buf.{sub}`")
                elif NULLISH.match(got.group(1).strip()):
                    out.append(f"{name}: buf.{sub} is initialized to a null/zero value")
        else:
            # The value up to the first `,` or `}` - the union's own braces follow it.
            head = re.match(r"[^,}]*", value.strip()).group(0).strip()
            if NULLISH.match(head):
                out.append(f"{name}: {maker}.u.{field} is initialized to a null/zero value")
    tail = body[static.end():]
    if re.search(r"\b%s\.u\.\w+(\.\w+)*\s*=" % re.escape(maker), tail):
        out.append(f"{name}: {maker}'s member is assigned in the macro body "
                   f"(a loaded dump never runs `syms_of_*`)")
    return out


def selftest() -> int:
    lisp = LISP_MIRROR.read_text(encoding="utf-8")
    buffer = BUFFER_UNIT.read_text(encoding="utf-8")
    old_buffer = buffer.replace(
        """      u: Lisp_Fwd_u { buf: Lisp_Fwd_buf {
        offset: std::mem::offset_of! (buffer, $vname) as u16,
        predicate: FWDPRED! ($predicate) as u8,
        _pad: 0 } },""",
        "      u: Lisp_Fwd_u { buf: Lisp_Fwd_buf { offset: 0, predicate: 0, _pad: 0 } },").replace(
        "      defvar_per_buffer (&raw const BO_FWD, $lname.as_ptr () as *const c_char);",
        "      BO_FWD.u.buf.offset = std::mem::offset_of! (buffer, $vname) as u16;\n"
        "      BO_FWD.u.buf.predicate = FWDPRED! ($predicate) as u8;\n"
        "      defvar_per_buffer (&raw const BO_FWD, $lname.as_ptr () as *const c_char);")
    cases = [
        ("the two files as they stand", [(LISP_MIRROR, lisp), (BUFFER_UNIT, buffer)], 0),
        ("DEFVAR_PER_BUFFER's offset/predicate assigned at run time",
         [(LISP_MIRROR, lisp), (BUFFER_UNIT, old_buffer)], 3),
        ("one DEFVAR_* initializer nulled",
         [(LISP_MIRROR, lisp.replace(
             "            u: Lisp_Fwd_u { boolvar: std::ptr::addr_of_mut! ($vname) },",
             "            u: Lisp_Fwd_u { boolvar: std::ptr::null_mut () },")), (BUFFER_UNIT, buffer)], 1),
        ("DEFVAR_PER_BUFFER removed from the unit",
         [(LISP_MIRROR, lisp), (BUFFER_UNIT, buffer.replace("macro_rules! DEFVAR_PER_BUFFER", "macro_rules! XX_DEFVAR_PER_BUFFER"))], 1),
        ("the whole macro gone from the mirror",
         [(LISP_MIRROR, lisp.replace("macro_rules! DEFVAR_INT", "macro_rules! XX_DEFVAR_INT")), (BUFFER_UNIT, buffer)], 1),
    ]
    bad = 0
    for name, files, want in cases:
        got = sum(len(findings(text, macro)) for path, text in files
                  for macro in [m for p, m in SITES if p == path])
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
    rows, seen = [], set()
    for path, name in SITES:
        text = path.read_text(encoding="utf-8")
        rows.extend(f"{path.relative_to(REPO)}: {row}" for row in findings(text, name))
        seen.add(path)
    print("read " + ", ".join(sorted(str(p.relative_to(REPO)) for p in seen))
          + f": {len(SITES)} macro(s) checked")
    for row in rows:
        print(f"FAIL {row}")
    print(f"{len(rows)} failure(s)")
    return 1 if rows else 0


if __name__ == "__main__":
    sys.exit(main())
