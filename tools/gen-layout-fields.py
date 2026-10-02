#!/usr/bin/env python3
"""Generate spikes/layout_fields.h for the C layout probe.

`spikes/layout_c.c` prints the sizes/offsets of the types the mirrors declare;
the frozen baseline under docs/baseline/layout/ is produced from its output (see
docs/spikes/2026-10-02-three-spikes.md).  This script writes the per-field part
of that probe, so every non-bitfield field of every mirrored struct gets an
`offsetof` key while `sizeof`/`alignof` lock the whole layout.

    python3 tools/gen-layout-fields.py [--reference emacs] [--out spikes/layout_fields.h]
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("cinventory", HERE / "cinventory.py")
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

# The structs the port mirrors so far; each entry is the header that defines it.
STRUCTS = {
    "itree_node": "src/itree.h",
    "itree_tree": "src/itree.h",
    "itree_iterator": "src/itree.h",
    "buffer": "src/buffer.h",
    "buffer_text": "src/buffer.h",
    "sortvec": "src/buffer.h",
    "ts_linecol": "src/buffer.h",
    "Lisp_Bignum": "src/bignum.h",
    # lisp.h's symbol-value and hash-table structures: reviewer A's F1 (round 8) found
    # `hash_idx_t` mis-sized and the mirror's `Lisp_Hash_Table` tail carrying an extra
    # `u32`, and *nothing* could see either, because these types had no keys here.
    "Lisp_Hash_Table": "src/lisp.h",
    "hash_table_test": "src/lisp.h",
    "Lisp_Obarray": "src/lisp.h",
    # The nested-aggregate filter (`direct_field`, PORTING.md P-65) is what lets these two in:
    # `Lisp_Fwd`'s value union is reached as `u.intvar` in C, and `offsetof (struct Lisp_Fwd,
    # intvar)` does not compile.  A type is added only once its *mirror* declares it, because a
    # key whose type is missing counts as pending and the ratchet forbids pending from growing:
    # `glyph`/`glyph_slice` go in with the `dispextern.h` mirror.
    "Lisp_Fwd": "src/lisp.h",
    "Lisp_Buffer_Local_Value": "src/lisp.h",
    # The display headers' first type: its four 16-bit bitfields share one 64-bit unit, so
    # the port models it as `bits: u64` and `sizeof`/`alignof` (8/4) are what the gate can
    # assert (PORTING.md 1.3).
    "glyph_slice": "src/dispextern.h",
    # dispextern.h segment II's platform-free types: the position pairs every display
    # function passes around, the slice `struct it` embeds, and the bidi state it carries.
    # `bidi_saved_info` is also the first type whose key names a Rust keyword (`type`), which
    # `xtask`'s `rust_ident` spells `r#type` in the generated assertion.
    "text_pos": "src/dispextern.h",
    "display_pos": "src/dispextern.h",
    "it_slice": "src/dispextern.h",
    "run": "src/dispextern.h",
    "bidi_saved_info": "src/dispextern.h",
    "bidi_stack": "src/dispextern.h",
    "bidi_string_data": "src/dispextern.h",
    "composition": "src/composite.h",
    # coding.h's type layer: the three small specs, the two aggregating specs, and the
    # 616-byte `coding_system` whose flag word is 31 bits of one 4-byte unit at offset 8
    # (the port stores that unit as `bits: u32`; the measured word is in the mirror).
    "coding_detection_info": "src/coding.h",
    "composition_status": "src/coding.h",
    "iso_2022_spec": "src/coding.h",
    "emacs_mule_spec": "src/coding.h",
    "undecided_spec": "src/coding.h",
    "utf_16_spec": "src/coding.h",
    "coding_system": "src/coding.h",
    # process.h's pseudovector: 272 bytes, with two bitfield units (one at 232 for the 2-bit
    # adaptive_read_buffering plus read_output_skip, one at 248 for eight bool_bf:1 members).
    "Lisp_Process": "src/process.h",
    # termhooks.h's event record: the unit at offset 0 holds `kind : 16` and `part : 16`, so
    # the keyable members are the seven after it (code … device).
    "input_event": "src/termhooks.h",
    # keyboard.h's input stream state (200/8, with the `immediate_echo` unit at 185) and its
    # event accessors' type.
    "kboard": "src/keyboard.h",
    # termhooks.h's terminal record: 520/8, with `display_info` (an 8-byte union) at 80 and the
    # hook table from 112 to 512.
    "terminal": "src/termhooks.h",
    # window.h's record (520/8) and its cursor position; the nineteen `bool_bf : 1` members share
    # the 4-byte unit at 500, so the port keys everything else and stores that unit as `bits`.
    "window": "src/window.h",
    "cursor_pos": "src/window.h",
    # frame.h's record (568/8).  Its 37 flag members live in two 4-byte units at 240 and 244, so
    # the generator keys the other 92 members and the port stores the units as `bits1`/`bits2`.
    "frame": "src/frame.h",
}

# C unions the port mirrors as Rust unions.  They get `SZ (union …)`/`AL (union …)` keys and the
# same per-member offsets (C allows `offsetof` on a union's members: they all sit at 0).
UNIONS = {
    "buffered_input_event": "src/keyboard.h",
}


# Members `cinventory` does not report as *direct* fields of the container but which the C
# compiler can still key.  `struct Lisp_Fwd`'s value union is spelled `u` in C and
# `offsetof (struct Lisp_Fwd, u)` is legal, while its four inner members (`intvar`, …) are
# reached as `u.intvar` and are not (P-65).  Without this key the type had *no* offset at
# all, so swapping `type` and `u` in the mirror — or dropping `#[repr(C)]` — left both the
# layout gate and `extract-abi --check` green (review B's F3, round 12).
EXTRA = {"Lisp_Fwd": ["u"]}

# Types whose every member is a bitfield: `offsetof` is illegal for them (clang: "cannot
# compute offset of bit-field"), so `sizeof`/`alignof` plus the byte-pattern probes are the
# whole gate for them.  Listed explicitly so a struct that *should* have keys cannot
# silently join them: the invariant below fails instead (review B's F3).
BITFIELD_ONLY = {"glyph_slice": "four 16-bit bitfields share one 8-byte unit (P-67)"}


def live_bodies(ref: Path, cfg: Path, header: str) -> str:
    """The preprocessed text of HEADER.

    cinventory reads the whole header, including fields behind `#ifdef`s that the
    frozen configuration disables (tree-sitter, X11, …); C rejects `offsetof` on
    those, so the generator keeps only members the preprocessor leaves in.
    """
    # gnulib's <sys/types.h> and <time.h> wrappers require config.h first, so the
    # preprocessor is told to include it (as spikes/layout_c.c does).
    cmd = ["clang", "-E", "-std=gnu23", f"-include{cfg}/src/config.h",
           f"-I{cfg}/lib", "-idirafterspikes/shim", f"-I{cfg}/src",
           f"-I{ref}/src", f"-I{ref}/lib", str(ref / header)]
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout


def struct_body(text: str, name: str) -> str:
    m = re.search(rf"^(?:struct|union) {re.escape(name)}\s*\n?\s*\{{", text, re.M)
    if not m:
        return ""
    depth, i = 0, text.index("{", m.start())
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[m.start():i + 1]
        i += 1
    return ""


def direct_field(body: str, field: str) -> bool:
    """True when FIELD is a member of the struct itself, not of a nested aggregate.

    `cinventory.struct_fields` flattens a nested (named) `union`/`struct` into its enclosing
    container — `struct Lisp_Fwd`'s `intvar` comes back with `container == "Lisp_Fwd"` even
    though C reaches it as `u.intvar`.  Emitting `offsetof (struct Lisp_Fwd, intvar)` for it
    does not compile (PORTING.md P-65).  A member is direct when the name occurs at the body's
    own brace level (depth 1 inside `{ ... }`); anything deeper belongs to a nested aggregate.
    """
    for match in re.finditer(rf"\b{re.escape(field)}\b", body):
        depth = body[:match.start()].count("{") - body[:match.start()].count("}")
        if depth == 1:
            return True
    return False


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--reference", default="emacs")
    ap.add_argument("--configure", default="build/emacs-ref",
                    help="frozen configure output (see docs/spikes/2026-10-02-three-spikes.md)")
    ap.add_argument("--out", default="spikes/layout_fields.h")
    args = ap.parse_args()
    ref, out, cfg = Path(args.reference), Path(args.out), Path(args.configure)

    lines = [
        "/* Generated by tools/gen-layout-fields.py; do not edit.",
        "   One sizeof/alignof/offsetof line per non-bitfield field of every mirrored",
        "   struct, so the C compiler freezes the layout the mirrors must reproduce",
        "   (bitfields are covered by the neighbouring offsets and by sizeof).  */",
    ]
    total = 0
    for name, header in {**STRUCTS, **UNIONS}.items():
        kind = "union" if name in UNIONS else "struct"
        body = struct_body(live_bodies(ref, cfg, header), name)
        if not body:
            print(f"warning: struct {name} not found in the preprocessed {header}")
        inv = ci.inventory(ref / header)
        # `container` alone is not enough: the scanner reports a nested aggregate's members with
        # the enclosing container and a `nesting` path that names the anonymous aggregate
        # (`terminal > <anon@542>`), and C rejects `offsetof (struct terminal, tty)` — the member
        # is reached as `display_info.tty`.  The aggregate member itself (`display_info`) has a
        # one-element nesting path and is keyed.
        fields = [f for f in inv["struct_fields"]
                  if f["container"] == name and not f["bitfield"]
                  and " > " not in f["nesting"]
                  and direct_field(body, f["name"])]
        lines.append("")
        lines.append(f"  SZ ({kind} {name}); AL ({kind} {name});")
        for f in fields:
            lines.append(f"  OFF ({kind} {name}, {f['name']});")
            total += 1
        emitted = {f["name"] for f in fields}
        for member in EXTRA.get(name, []):
            if member in emitted:
                continue          # the scanner now reports the aggregate member itself
            lines.append(f"  OFF ({kind} {name}, {member});")
            total += 1
            emitted.add(member)
        if not emitted and name not in BITFIELD_ONLY:
            raise SystemExit(
                f"{name} has no offset key: its members are either all bitfields (then "
                f"record it in BITFIELD_ONLY with the reason) or nested aggregates that "
                f"need an entry in EXTRA (their key must name a member C can take "
                f"`offsetof` of) — a type with zero offset keys is a type whose field "
                f"order no gate checks (review B's F3)")
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {out}: {len(STRUCTS)} structs, {total} field offsets")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
