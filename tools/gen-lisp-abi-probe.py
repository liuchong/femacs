#!/usr/bin/env python3
"""Type-check lisp.rs's `extern "C"` declarations against the reference lisp.h.

`extract-abi.py` compares *names*, so a mirror declaration can carry the wrong type
forever (round 6's reviewer B found six: `make_string`/`make_unibyte_string`/`intern_1`/
`intern_c_string_1`/`fast_c_string_match_internal`/`egetenv_internal` all said `usize`
where lisp.h says `ptrdiff_t`).  This tool reads those declarations back out of lisp.rs,
reverses the type mapping, and prints

    static RET (*p_NAME) (ARGS) = NAME;

for every declaration it can map, so `clang -Werror=incompatible-pointer-types` against
`emacs/src/lisp.h` turns any wrong width / const-ness / return type into an error.
Declarations whose types it cannot map are counted and reported, not silently dropped.

    python3 tools/gen-lisp-abi-probe.py [--mirror femacs/src/include/lisp.rs]
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REFERENCE_LISP_H = Path("emacs/src/lisp.h")

# Rust -> C, longest spellings first.
RULES = [
    ("*const c_char", "const char *"),
    ("*mut c_char", "char *"),
    ("*const c_void", "const void *"),
    ("*mut c_void", "void *"),
    ("*const Lisp_Object", "const Lisp_Object *"),
    ("*mut Lisp_Object", "Lisp_Object *"),
    ("*mut Lisp_Vector", "struct Lisp_Vector *"),
    ("*mut Lisp_Symbol", "struct Lisp_Symbol *"),
    ("*const Lisp_Symbol", "const struct Lisp_Symbol *"),
    ("Lisp_Object", "Lisp_Object"),
    ("EMACS_INT", "EMACS_INT"),
    ("ptrdiff_t", "ptrdiff_t"),
    ("specpdl_ref", "specpdl_ref"),
    ("isize", "ptrdiff_t"),
    ("usize", "size_t"),
    ("c_double", "double"),
    ("f64", "double"),
    ("i64", "intmax_t"),
    ("u64", "uintmax_t"),
    ("core::ffi::VaList<'_>", "va_list"),
    ("...", "..."),
    ("c_int", "int"),
    ("c_uint", "unsigned int"),
    ("c_long", "long"),
    ("c_ulong", "unsigned long"),
    ("c_char", "char"),
    ("bool", "bool"),
    ("pvec_type", "enum pvec_type"),
    ("specpdl_ref", "specpdl_ref"),
]


# Declarations the harness must not type-check, each with the reason it is out of
# reach.  They are printed with their reasons, so an addition here is visible.
SKIP = {
    # src/sysstdio.h declares it; lisp.h is what this harness includes.
    "die": "declared in src/sysstdio.h, which the harness does not include",
    # P-05: C's `alloca` has no Rust counterpart, so AVAIL_ALLOCA calls a port-side
    # allocator (alloc.rs).  lisp.h has no such declaration to check against.
    "sa_stack_alloc": "port-side alloca replacement (P-05), not a C name in lisp.h",
}

TAG_RE = re.compile(r"\b(struct|union|enum)\s+(\w+)")
TYPEDEF_RE = re.compile(r"\btypedef\b[^;]*?(\w+)\s*;", re.S)
# Rust spellings with no C counterpart; seeing one means the mapping is incomplete.
RUST_ONLY = re.compile(r"\b(usize|isize|Option|c_[a-z_]+|f64|i64|u64|never)\b")


def strip_comments(text: str) -> str:
    """TEXT without comments or string literals.

    English prose mentions "a struct and a union", and the reference's own dead `#if`
    branches can define a tag the frozen configuration never uses.
    """
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.S)
    text = re.sub(r'"(?:\\.|[^"\\])*"', '""', text)
    return re.sub(r"//[^\n]*", " ", text)


def reference_c_tables() -> tuple:
    """`(tags, typedefs)` read out of lisp.h.

    lisp.h never writes a struct/union tag without its keyword, but the port's mirror
    names the Rust type after the tag (`union Aligned_Lisp_Subr` -> `Aligned_Lisp_Subr`),
    so the reverse mapping has to know which keyword C used.  A name that is *also* a
    typedef is spelled as the typedef: lisp.h 589 defines `struct Lisp_Object` only in
    the `CHECK_LISP_OBJECT_TYPE` branch, which the frozen configuration does not take
    (lisp.h 595 defines `Lisp_Object` as `Lisp_Word`), so `struct Lisp_Object` is not the
    type the reference compiles.
    """
    tags: dict = {}
    text = strip_comments(REFERENCE_LISP_H.read_text(encoding="utf-8", errors="replace"))
    for kind, name in TAG_RE.findall(text):
        if tags.setdefault(name, kind) != kind:
            print(f"warning: tag {name} is both {tags[name]} and {kind}", file=sys.stderr)
    return tags, set(TYPEDEF_RE.findall(text))


def to_c(rust_type: str, tags: dict, typedefs: set) -> str | None:
    t = " ".join(rust_type.split()).strip()
    if t in ("()", "!"):
        return "void"
    # A Rust type named after a C tag has to be spelled with the tag's keyword.
    m = re.fullmatch(r"\*(mut|const)\s+(\w+)", t)
    if m and m.group(2) in tags and m.group(2) not in typedefs:
        qual = "" if m.group(1) == "mut" else "const "
        return f"{qual}{tags[m.group(2)]} {m.group(2)} *"
    if t in tags and t not in typedefs:
        return f"{tags[t]} {t}"
    for pat, rep in RULES:
        t = t.replace(pat, rep)
    # Leftovers.  A Rust-only type name, or a C tag spelled without its keyword, would
    # silently compile as *something else* in C (`Lisp_Vector *` is not
    # `struct Lisp_Vector *`), so the declaration is reported unmapped rather than
    # checked.  Names that the reverse mapping did spell (`struct Lisp_Vector *`,
    # `Lisp_Object *`) are fine and are checked.
    if RUST_ONLY.search(t):
        return None
    for name in set(re.findall(r"[A-Za-z_]\w*", t)):
        if (name in tags and name not in typedefs
                and not re.search(rf"\b{tags[name]}\s+{name}\b", t)):
            return None
    return t.strip()




def split_top_level(args: str) -> list:
    """Split a parameter list on commas that are not inside parentheses."""
    out, depth, cur = [], 0, ""
    for ch in args:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(cur)
            cur = ""
            continue
        cur += ch
    if cur.strip():
        out.append(cur)
    return out


def extern_blocks(text: str) -> list:
    """The text of every `unsafe extern "C" { … }` block in TEXT."""
    out = []
    for m in re.finditer(r"unsafe extern \"C\"\s*\{", text):
        depth, i = 0, m.end() - 1
        while i < len(text):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    break
            i += 1
        out.append(text[m.end():i])
    return out


def logical_lines(block: str) -> list:
    """BLOCK's `(annotation, declaration)` pairs, continuations joined.

    A declaration whose C type the mirror cannot spell (an incomplete pointee such as
    `struct buffer *`) records the reference prototype in a `/// C: <prototype>` line
    right above it; the annotation is checked against lisp.h instead of the Rust
    spelling (PORTING.md 0.1-2, P-55).
    """
    out, cur, annotation = [], "", None
    for line in block.split("\n"):
        if line.strip().startswith("/// C:"):
            annotation = line.strip()[len("/// C:"):].strip()
            cur = ""
            continue
        if "//" in line:
            line = line[:line.index("//")]
        cur = (cur + " " + line.strip()) if cur else line.strip()
        if cur.endswith(";"):
            out.append((annotation, " ".join(cur.split())))
            cur, annotation = "", None
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--mirror", default="femacs/src/include/lisp.rs")
    ap.add_argument("--baseline", default="docs/ports/lisp-decl-abi-baseline.txt",
                    help="declarations whose type is known to differ from lisp.h's; "
                         "listed with a reason, so a *new* deviation still fails")
    args = ap.parse_args()
    text = Path(args.mirror).read_text(encoding="utf-8")

    baselined = set()
    base_path = Path(args.baseline)
    if base_path.is_file():
        for line in base_path.read_text(encoding="utf-8").splitlines():
            line = line.split("#")[0].strip()
            if line:
                baselined.add(line)
    baselined_used: list[str] = []

    print("/* Generated by tools/gen-lisp-abi-probe.py from femacs/src/include/lisp.rs;")
    print("   do not edit.  Each line's type must equal the reference lisp.h's.  */")
    print("#include <config.h>")
    print("#include \"lisp.h\"")
    print()
    tags, typedefs = reference_c_tables()
    checked: list[str] = []
    skipped: list[str] = []
    unmapped: list[str] = []
    for block in extern_blocks(text):
        for annotation, decl in logical_lines(block):
            m = re.match(r"pub fn (\w+)\s*\((.*)\)\s*(?:->\s*([^;]+?))?;\s*$", decl)
            if not m:
                continue
            name, args, ret = m.group(1), m.group(2).strip(), (m.group(3) or "()").strip()
            if name in SKIP:
                skipped.append(name)
                continue
            if name in baselined:
                baselined_used.append(name)
                continue
            if annotation:
                am = re.match(r"(.+?)\s*(\w+)\s*\((.*)\)\s*$", annotation)
                if not am or am.group(2) != name:
                    print(f"error: {name}: unparsable `/// C:` annotation "
                          f"{annotation!r} (it must name {name})", file=sys.stderr)
                    return 1
                print(f"static {am.group(1).strip()} (*p_{name}) "
                      f"({am.group(3).strip()}) = {name};   /* /// C: */")
                checked.append(name)
                continue
            ret_c = to_c(ret, tags, typedefs)
            params = []
            if args:
                for arg in split_top_level(args):
                    ty = arg.split(":", 1)[1] if ":" in arg else arg
                    mapped = to_c(ty, tags, typedefs)
                    if mapped is None:
                        params = None
                        break
                    params.append(mapped)
            if ret_c is None or params is None or not params and args:
                unmapped.append(name)
                continue
            print(f"static {ret_c} (*p_{name}) ({', '.join(params) or 'void'}) = {name};")
            checked.append(name)
    print()
    print("int main (void)")
    print("{")
    for name in checked:
        print(f"  (void) p_{name};")
    print("  return 0;")
    print("}")
    print(f"/* {len(checked)} declarations type-checked, {len(unmapped)} unmapped, "
          f"{len(baselined_used)} baselined, {len(skipped)} skipped */", file=sys.stderr)
    if unmapped:
        print("/* unmapped (no C type inferred, not checked): "
              + ", ".join(sorted(unmapped)) + " */", file=sys.stderr)
    for name in sorted(skipped):
        print(f"/* skipped {name}: {SKIP[name]} */", file=sys.stderr)
    if len(checked) < 40:
        print(f"error: only {len(checked)} declarations mapped (floor 40)", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
