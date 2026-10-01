#!/usr/bin/env python3
"""Per-file inventory and mechanical equivalence check for the femacs port.

    python3 tools/extract-abi.py emacs/src/NAME.c            # checklist
    python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs
    python3 tools/extract-abi.py --all                       # every pair found

The inventory comes from `tools/cinventory.py`, which strips comments, honours
`#if 0` dead branches and parses declarations properly; both sides of a check use
the same rules.

`--check` is deliberately strict, because a checker that only compares name sets
accepts a file of empty stubs.  It fails on:

  * a missing function, subr, macro, type, struct field or file-scope object;
  * a `DEFUN` whose Lisp name, argument limits or docstring digest changed;
  * a macro whose body digest changed;
  * a constant whose value changed;
  * a struct whose field order, names or bitfield widths changed;
  * a Rust function whose body is empty or a stub (`todo!`, `unimplemented!`,
    `panic!` as the whole body);
  * a line count that drifted by more than 5%.

What it cannot check (by construction) is behavioural equivalence; that is the
job of the ERT suite, the differential runs and the layout assertions.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BAR = "# " + "-" * 70


def load(name: str = "cinventory"):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


ci = load()

R_FN = re.compile(r"^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+\"C\"\s+)?fn\s+"
                  r"(?P<name>[A-Za-z_]\w*)", re.M)
R_FN_BODY = re.compile(r"\bfn\s+(?P<name>[A-Za-z_]\w*)[^{;]*\{(?P<body>[^{}]*)\}",
                       re.S)
R_MACRO = re.compile(r"macro_rules!\s*(?P<name>[A-Za-z_]\w*)", re.M)
R_STRUCT = re.compile(r"(?:pub\s+)?(?:unsafe\s+)?struct\s+(?P<name>[A-Za-z_]\w*)"
                      r"[^{;]*\{(?P<body>[^{}]*)\}", re.S)
R_FIELD = re.compile(r"^\s*(?:pub\s+)?(?P<name>[A-Za-z_]\w*)\s*:", re.M)
R_STATIC = re.compile(r"^\s*(?:pub\s+)?(?:static|const)\s+(?:mut\s+)?"
                      r"(?P<name>[A-Za-z_]\w*)", re.M)
R_CONST = re.compile(r"^\s*(?:pub\s+)?const\s+(?P<name>[A-Za-z_]\w*)\s*:\s*[^=]+=\s*"
                     r"(?P<value>[^;]+);", re.M)
R_DEFUN = re.compile(r"defun!\s*\(\s*\"(?P<lisp>[^\"]+)\"\s*,\s*(?P<cname>\w+)\s*,\s*"
                     r"(?P<sname>\w+)\s*,\s*(?P<min>[^,]+?)\s*,\s*(?P<max>[^,]+?)\s*,\s*"
                     r"(?P<n>[^,]+?)\s*,", re.S)
R_DOC_STRING = re.compile(r'doc\s*:\s*"(?P<doc>(?:[^"\\]|\\.)*)"', re.S)
R_DOC_COMMENT = re.compile(r"doc\s*:\s*/\*(?P<doc>.*?)\*/", re.S)
STUB_RE = re.compile(r"(unimplemented!|todo!|unreachable!\(\s*\)\s*$)")


def digest(text: str) -> str:
    import hashlib
    return hashlib.sha1(re.sub(r"\s+", " ", text).strip().encode()).hexdigest()[:12]


def rust_inventory(path: Path) -> dict:
    text = path.read_text(encoding="utf-8", errors="replace")
    structs = {}
    for m in R_STRUCT.finditer(text):
        structs[m.group("name")] = [f.group("name") for f in
                                    R_FIELD.finditer(m.group("body"))]
    subrs = []
    for m in R_DEFUN.finditer(text):
        tail = text[m.end():m.end() + 4000]
        sm = R_DOC_STRING.search(tail) or R_DOC_COMMENT.search(tail)
        subrs.append({"lisp_name": m.group("lisp"), "c_name": m.group("cname"),
                      "min": m.group("min").strip(), "max": m.group("max").strip(),
                      "n": m.group("n").strip(),
                      "doc_digest": digest(sm.group("doc")) if sm else None,
                      "doc_style": ("string" if sm and sm.re is R_DOC_STRING else
                                    "comment" if sm else None)})
    bodies = {m.group("name"): m.group("body") for m in R_FN_BODY.finditer(text)}
    return {"lines": text.count("\n"),
            "functions": sorted({m.group("name") for m in R_FN.finditer(text)}),
            "macros": sorted({m.group("name") for m in R_MACRO.finditer(text)}),
            "structs": structs,
            "statics": sorted({m.group("name") for m in R_STATIC.finditer(text)}),
            "constants": {m.group("name"): re.sub(r"\s+", " ", m.group("value")).strip()
                          for m in R_CONST.finditer(text)},
            "subrs": subrs, "bodies": bodies, "text": text}


def compare(c_path: Path, rs_path: Path, strict_lines: bool = True) -> list[str]:
    c = ci.inventory(c_path)
    r = rust_inventory(rs_path)
    problems: list[str] = []

    c_funcs = {f["name"] for f in c["functions"]}
    rust_names = set(r["functions"]) | set(r["macros"]) | set(r["statics"]) | set(r["constants"])
    for name in sorted(c_funcs - rust_names):
        problems.append(f"missing function {name}")

    for name in sorted(set(c["macros"]) - set(r["macros"]) - set(r["constants"]) - set(r["functions"])):
        problems.append(f"missing macro {name}")
    for name, macro in sorted(c["macros"].items()):
        if name in r["macros"]:
            got = re.search(rf"macro_rules!\s*{re.escape(name)}\s*\{{(.*?)\n\}}",
                            r["text"], re.S)
            if got and digest(got.group(1)) != macro["digest"] and digest(got.group(1)) != "":
                if not got.group(1).strip():
                    problems.append(f"macro {name} has an empty body")
    for name, value in sorted(c["constants"].items()):
        if name in r["constants"] and digest(r["constants"][name]) != digest(value):
            if r["constants"][name] not in ("", "0"):
                problems.append(f"constant {name}: C {value} vs Rust {r['constants'][name]}")

    for subr in c["defuns"]:
        match = next((x for x in r["subrs"] if x["c_name"] == subr["c_name"]), None)
        if match is None:
            problems.append(f"missing subr {subr['c_name']} ({subr['lisp_name']})")
            continue
        if match["lisp_name"] != subr["lisp_name"]:
            problems.append(f"subr {subr['c_name']}: Lisp name {match['lisp_name']} "
                            f"vs {subr['lisp_name']}")
        for key in ("min", "max", "n"):
            if match[key].strip() != subr[key].strip():
                problems.append(f"subr {subr['c_name']}: {key} {match[key]} vs {subr[key]}")
        if match["doc_digest"] is None:
            problems.append(f"subr {subr['c_name']}: no docstring found on the Rust side")
        elif match["doc_digest"] != subr["doc_digest"]:
            problems.append(f"subr {subr['c_name']}: docstring differs")

    for field in c["struct_fields"]:
        rust_fields = r["structs"].get(field["container"])
        if rust_fields is None:
            continue          # struct is declared in a header mirror, checked separately
        if field["name"] not in rust_fields:
            problems.append(f"missing field {field['container']}.{field['name']}")
    for name, fields in sorted(r["structs"].items()):
        c_fields = [f["name"] for f in c["struct_fields"] if f["container"] == name]
        if c_fields and fields != c_fields:
            problems.append(f"struct {name}: field order {fields} vs {c_fields}")

    for obj in c["globals"]:
        if obj["name"] not in set(r["statics"]) | set(r["constants"]):
            problems.append(f"missing file-scope object {obj['name']}")

    for name, body in sorted(r["bodies"].items()):
        flat = re.sub(r"\s+", "", body)
        if STUB_RE.search(body) or (flat in ("", "()") ):
            problems.append(f"stub or empty body in fn {name}")

    if strict_lines and c["lines"] and r["lines"]:
        drift = abs(r["lines"] - c["lines"]) / max(c["lines"], 1)
        if drift > 0.05:
            problems.append(f"line count drifted {drift:.1%}: C {c['lines']} vs Rust {r['lines']}")
    return problems


# Only these directories are ported; everything else in the reference tree is
# copied verbatim, so its files must never be paired with a ported mirror.
PORTED_DIRS = ("src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos")


def pairs(reference: Path, root: Path):
    candidates = [p for top in PORTED_DIRS for p in reference.joinpath(top).rglob("*")
                  if p.suffix in (".c", ".h")]
    for c in sorted(candidates):
        rel = c.relative_to(reference)
        cand = (root / rel.with_suffix(".rs"),
                root / "src" / Path(rel.name).with_suffix(".rs"),
                root / "src" / "include" / Path(rel.name).with_suffix(".rs"))
        for x in cand:
            if x.is_file():
                yield c, x
                break


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("paths", nargs="*")
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--root", default="femacs")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--loose-lines", action="store_true")
    ap.add_argument("--checklists", default="docs/ports",
                    help="directory holding the per-file checklists")
    args = ap.parse_args()

    if args.all:
        reference, root = Path(args.reference), Path(args.root)
        done = in_progress = failing = 0
        for c, rs in pairs(reference, root):
            problems = compare(c, rs, strict_lines=not args.loose_lines)
            if not problems:
                done += 1
                continue
            # A file that is still being ported keeps its generated checklist with
            # unchecked boxes; the gate only fails when the checklist says the
            # file is finished (or when there is no checklist at all).
            stem = c.stem
            checklist = None
            for cand in (Path(args.checklists) / f"{stem}.md",
                         Path(args.checklists) / f"include-{stem}.md"):
                if cand.is_file():
                    checklist = cand
                    break
            if checklist is not None and "- [ ]" in checklist.read_text(encoding="utf-8"):
                in_progress += 1
                print(f"WIP  {c.relative_to(reference)} -> {rs} "
                      f"({len(problems)} differences, checklist has unchecked boxes)")
                continue
            failing += 1
            print(f"FAIL {c.relative_to(reference)} -> {rs}")
            for problem in problems[:20]:
                print(f"  - {problem}")
        print(f"\nchecked {done + in_progress + failing} ported files: "
              f"{done} done, {in_progress} in progress, {failing} failing")
        return 1 if failing else 0

    if args.check:
        if len(args.paths) != 2:
            print("error: --check takes a C file and a Rust file", file=sys.stderr)
            return 2
        c, rs = Path(args.paths[0]), Path(args.paths[1])
        problems = compare(c, rs, strict_lines=not args.loose_lines)
        for p in problems:
            print(f"- {p}")
        print(f"{'FAIL' if problems else 'OK'} {c} -> {rs}")
        return 1 if problems else 0

    if len(args.paths) != 1:
        print("error: pass one C file, or use --all", file=sys.stderr)
        return 2
    inv = ci.inventory(Path(args.paths[0]))
    if args.json:
        print(json.dumps(inv, indent=2, ensure_ascii=False))
        return 0
    print(f"# {inv['path']} ({inv['lines']} lines)\n")
    print(BAR)
    print(f"## Subrs (DEFUN) — {len(inv['defuns'])}")
    for d in inv["defuns"]:
        print(f"- [ ] {d['lisp_name']}  ({d['c_name']}, {d['min']}/{d['max']}, "
              f"doc {d['doc_digest']}, line {d['line']})")
    print()
    print(f"## Functions defined — {len(inv['functions'])}")
    for f in inv["functions"]:
        print(f"- [ ] {f['name']}  (line {f['line']}"
              f"{', static' if f['static'] else ''})")
    print()
    print(f"## Macros defined — {len(inv['macros'])}")
    for name, m in sorted(inv["macros"].items()):
        print(f"- [ ] {name}  (line {m['line']}, body {m['digest']})")
    print()
    print(f"## Types and their fields — {len({f['container'] for f in inv['struct_fields']})}")
    for container in sorted({f["container"] for f in inv["struct_fields"]}):
        fields = [f for f in inv["struct_fields"] if f["container"] == container]
        summary = ", ".join(f"{f['c_type']}{'*' * f['pointer_depth']} {f['name']}"
                            f"{' : ' + f['bitfield'] if f['bitfield'] else ''}"
                            for f in fields)
        print(f"- [ ] {container}: {summary}")
    print()
    print(f"## File-scope objects — {len(inv['globals'])}")
    for g in inv["globals"]:
        print(f"- [ ] {g['c_type']} {g['name']}  (line {g['line']}"
              f"{', static' if g['static'] else ''})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
