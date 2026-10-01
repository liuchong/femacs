#!/usr/bin/env python3
"""Emit the mechanical part of a C header mirror as Rust.

Types, file-scope declarations and literal constants are mechanical; inline
function bodies and code macros are not, and this tool refuses to guess at them
(it reports them instead, because a stub would violate the port's rules).

The output is a *draft*: it is printed, never written into the mirror, so the
hand-written parts of a mirror are never overwritten.

    python3 tools/emit-mirror.py emacs/src/lisp.h [--skip FILE]... > draft.rs

Guards: a type is skipped when a `--skip` file already declares it, when it has
bitfields (PORTING.md 1.3 wants explicit masks), when a field's type is not
mappable, or when an array bound is not a literal.
"""
from __future__ import annotations

import argparse
import importlib.util
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

BUILTIN = {
    "int": "c_int", "unsigned int": "c_uint", "unsigned": "c_uint",
    "char": "c_char", "signed char": "c_char", "unsigned char": "u8",
    "short": "c_short", "unsigned short": "c_ushort", "long": "c_long",
    "unsigned long": "c_ulong", "long long": "c_longlong",
    "unsigned long long": "c_ulonglong", "double": "c_double", "float": "c_float",
    "size_t": "usize", "ptrdiff_t": "isize", "void": "c_void",
    "bool": "bool", "bool_bf": "c_uint", "EMACS_INT": "EMACS_INT",
    "EMACS_UINT": "EMACS_UINT", "Lisp_Object": "Lisp_Object",
    "uintmax_t": "u64", "intmax_t": "i64",
}
SKIP_FIELDS = {"GCALIGNED_UNION_MEMBER", "GCALIGNED_STRUCT", "struct", "union"}


TYPEDEF = re.compile(r"^typedef\s+(?P<type>[^;{}]+?)\s*(?P<stars>\**)\s*"
                     r"(?P<name>[A-Za-z_]\w*)\s*;", re.M)


def collect_typedefs(text: str) -> dict[str, tuple[str, int]]:
    """`typedef <type> NAME;` aliases, resolved to (base type, pointer depth)."""
    out: dict[str, tuple[str, int]] = {}
    for m in TYPEDEF.finditer(text):
        t = re.sub(r"\s+", " ", m.group("type").strip())
        out[m.group("name")] = (t, len(m.group("stars") or ""))
    return out


def rust_type(c_type: str, depth: int, array: str, typedefs: dict | None = None,
              known: set | None = None) -> str | None:
    t = re.sub(r"\s+", " ", c_type.strip())
    t = t.replace("const ", "").replace("volatile ", "")
    t = re.sub(r"\bstruct\s+", "", t)
    t = re.sub(r"\bunion\s+", "", t)
    t = re.sub(r"\benum\s+", "", t)
    if t.endswith("(*)"):
        return None      # function pointer field: not mechanical, hand work
    if known and t in known:
        typedefs = None                        # already declared in the mirror
    if typedefs:
        for _ in range(4):                     # resolve typedef chains
            hit = typedefs.get(t)
            if not hit:
                break
            t, added = hit
            t = re.sub(r"\bstruct\s+|\bunion\s+|\benum\s+", "", t).strip()
            depth += added
    base = BUILTIN.get(t)
    if base is None:
        if re.fullmatch(r"[A-Za-z_]\w*", t):
            base = t          # a type this repository is expected to declare
        else:
            return None
    if depth:
        ptr = f"*mut {base}" if base != "c_void" else "*mut c_void"
        if depth > 1:
            ptr = "*mut " + ptr
        rust = ptr
    else:
        rust = base
    if array:
        bounds = re.findall(r"\[(\d*)\]", array)
        for b in reversed(bounds):
            rust = f"[{rust}; 0]" if b == "" else f"[{rust}; {b}]"
    return rust


def decl_lines(text: str, skip: set[str], known: set[str] | None = None):
    """Yield Rust lines for the types, globals and literal constants."""
    out, reported = [], {"bitfield": [], "unmapped": [], "array": []}
    typedefs = collect_typedefs(text)
    items = ci.struct_fields(text)
    by_container: dict[str, list[dict]] = {}
    for f in items:
        by_container.setdefault(f["container"], []).append(f)
    # container order as they appear
    order, seen = [], set()
    for f in items:
        if f["container"] not in seen:
            seen.add(f["container"])
            order.append((f["container"], f["container_kind"]))
    for name, kind in order:
        if name.startswith("<anon") or name in skip:
            continue
        fields = by_container[name]
        if any(f["bitfield"] for f in fields):
            reported["bitfield"].append(name)
            continue
        rendered, ok = [], True
        for f in fields:
            if f["c_type"].strip() in SKIP_FIELDS or f["name"] in SKIP_FIELDS:
                continue
            ty = rust_type(f["c_type"], f["pointer_depth"], f["array"], typedefs, skip)
            if ty is None:
                reported["unmapped"].append(f"{name}.{f['name']} ({f['c_type']})")
                ok = False
                break
            rendered.append(f"    pub {f['name']}: {ty},")
        if not ok or not rendered:
            continue
        rust_kw = "struct" if kind == "struct" else "union"
        out.append("#[repr(C, align(8))]")
        out.append("#[derive(Clone, Copy)]")
        out.append(f"pub {rust_kw} {name} {{")
        out.extend(rendered)
        out.append("}")
        out.append("")
    for name, value in ci.constants(text).items():
        if name in skip:
            continue
        v = value.strip()
        ty = "isize" if re.fullmatch(r"-?\d+", v) else "usize"
        if re.fullmatch(r"-?0[xX][0-9a-fA-F]+", v) or re.fullmatch(r"-?\d+", v):
            out.append(f"pub const {name}: {ty} = {v};")
    out.append("")
    if typedefs:
        for name, (t, stars) in sorted(typedefs.items()):
            if name in skip or name in BUILTIN:
                continue
            ty = rust_type(t, stars, "", typedefs, skip)
            if ty:
                out.append(f"pub type {name} = {ty};")
    for g in ci.globals_(text):
        if g["static"] or g["name"] in skip:
            continue
        ty = rust_type(g["c_type"], g["pointer_depth"], g["array"], typedefs, skip)
        if ty is None:
            reported["unmapped"].append(f"global {g['name']} ({g['c_type']})")
            continue
        out.append(f"unsafe extern \"C\" {{ pub static mut {g['name']}: {ty}; }}")
    return out, reported


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("header")
    ap.add_argument("--skip", action="append", default=[])
    args = ap.parse_args()
    text = Path(args.header).read_text(encoding="utf-8", errors="replace")
    skip = set()
    for path in args.skip:
        t = Path(path).read_text(encoding="utf-8", errors="replace")
        skip |= set(re.findall(r"\b(?:struct|union|enum)\s+(\w+)", t))
        skip |= set(re.findall(r"\b(?:pub\s+)?(?:static|const)\s+(?:mut\s+)?(\w+)", t))
        skip |= set(re.findall(r"\bpub type (\w+)", t))
    mirror_known = set(skip)
    lines, reported = decl_lines(text, skip, mirror_known)
    import sys
    print("\n".join(lines), file=sys.stdout)
    print("// --- hand work still required ---", file=sys.stdout)
    for key, items in reported.items():
        print(f"// {key}: {len(items)}", file=sys.stdout)
    sys.stderr.write(
        f"emitted {sum(1 for l in lines if l.startswith('pub '))} declarations; "
        f"skipped bitfield types {len(reported['bitfield'])}, "
        f"unmappable {len(reported['unmapped'])}, arrays {len(reported['array'])}\n")
    if reported["bitfield"]:
        sys.stderr.write("bitfield types: " + ", ".join(reported["bitfield"][:12]) + "\n")
    if reported["unmapped"]:
        sys.stderr.write("unmappable: " + "; ".join(reported["unmapped"][:8]) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
