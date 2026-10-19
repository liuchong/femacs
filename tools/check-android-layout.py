#!/usr/bin/env python3
"""Compare the Android (bionic) record layouts the port asserts against the NDK's own answers.

The port mirrors C structs and types whose size differs per target; three of them were found the
hard way on the device (status 4.785-4.792): `sigjmp_buf` written 68 bytes past a host-sized
`return_to_command_loop`, a host-shaped `struct dirent` read out of bionic's 280-byte record, and
`struct sfnt_interpreter`'s offsets measured on the host.  This tool makes that comparison
mechanical for the records whose sizes the port asserts.

Baseline: docs/baseline/layout/android-arm64.tsv, one row per entry:

    <stem>\\t<link-name>\\t<header>\\t<c expression>\\t<value>\\t<note>

`<stem>` is the name the port's assertion is found under (`size_of::<STEM>` /
`offset_of! (STEM, FIELD)`), `<c expression>` is what the NDK measures (e.g. `sizeof (jmp_buf)`,
`offsetof (struct dirent, d_name)`), and `<value>` is the last measured number.

    tools/check-android-layout.py             # compare the table against a fresh measurement
    tools/check-android-layout.py --update    # rewrite the table's values from the measurement
    tools/check-android-layout.py --list      # print the table
"""
import argparse
import os
import pathlib
import re
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
BASELINE = REPO / "docs/baseline/layout/android-arm64.tsv"
CLANG_GLOB = "ndk/*/toolchains/llvm/prebuilt/*/bin"


def sdk_root():
    env = os.environ.get("ANDROID_SDK_ROOT") or os.environ.get("ANDROID_HOME")
    return pathlib.Path(env) if env else pathlib.Path.home() / "Library/Android/sdk"


def toolchain():
    bins = sorted((sdk_root() / "ndk").glob("*/toolchains/llvm/prebuilt/*/bin"))
    if not bins:
        return None
    return bins[-1]


def rows():
    out = []
    for line in BASELINE.read_text().splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 5:
            continue
        out.append(parts[:6] + [""] * (6 - len(parts)))
    return out


def measure(bindir, body, includes):
    """Compile one probe and return the object symbol's size."""
    src = "#include <stddef.h>\n" + "".join("#include <%s>\n" % h for h in includes)
    src += "char probe[%s];\n" % body
    with tempfile.TemporaryDirectory() as tmp:
        c = pathlib.Path(tmp) / "p.c"
        o = pathlib.Path(tmp) / "p.o"
        c.write_text(src)
        clang = bindir / "clang"
        api_clang = bindir / "aarch64-linux-android21-clang"
        cc = api_clang if api_clang.exists() else clang
        args = [str(cc), "-c", str(c), "-o", str(o)]
        if cc == clang:
            args[1:1] = ["--target=aarch64-linux-android21"]
        r = subprocess.run(args, capture_output=True, text=True)
        if r.returncode != 0:
            raise SystemExit("measurement failed for %r:\\n%s" % (body, r.stderr.strip()))
        nm = bindir / "llvm-nm"
        out = subprocess.run([str(nm), "-S", "--defined-only", str(o)], capture_output=True,
                             text=True).stdout
        for line in out.splitlines():
            p = line.split()
            if len(p) >= 4 and p[3] == "probe":
                return int(p[1], 16)
    raise SystemExit("no probe symbol for %r" % body)


def rust_assertions():
    """Every asserted size/offset in the port, as {(stem, field-or-None): value}."""
    found = []
    for path in list((REPO / "femacs/src").rglob("*.rs")) + list((REPO / "femacs/lib").rglob("*.rs")):
        try:
            text = path.read_text()
        except UnicodeDecodeError:
            continue
        for m in re.finditer(r"size_of::<(\w+)>\s*\(\)\s*==\s*(\d+)", text):
            found.append((m.group(1), None, int(m.group(2))))
        for m in re.finditer(r"offset_of!\s*\(\s*(\w+)\s*,\s*(\w+)\s*\)\s*==\s*(\d+)", text):
            found.append((m.group(1), m.group(2), int(m.group(3))))
    return found


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--update", action="store_true", help="rewrite the table's measured values")
    ap.add_argument("--list", action="store_true", help="print the table")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    table = rows()
    if args.list:
        for stem, link, header, expr, value, note in table:
            print("%-26s %-40s %s" % (stem, expr, value))
        return 0
    bins = toolchain()
    if bins is None:
        print("check-android-layout: no NDK toolchain under %s" % sdk_root(), file=sys.stderr)
        return 2

    measured = []
    for stem, link, header, expr, value, note in table:
        includes = [h for h in header.split(",") if h]
        body = expr.replace("STEM", stem)
        # `sizeof`/`offsetof` expressions are what the table records; a link-name row measures a
        # typedef the port spells differently, so `expr` carries the C spelling itself.
        got = measure(bins, body, includes)
        measured.append((stem, link, header, expr, got, note))

    if args.update:
        lines = ["# Bionic layout baseline for the records whose sizes the port asserts.",
                 "# Regenerate: tools/check-android-layout.py --update",
                 "# stem\tlink-name\theader\texpression\tvalue\tnote"]
        for stem, link, header, expr, value, note in measured:
            lines.append("\t".join([stem, link, header, expr, str(value), note]))
        BASELINE.write_text("\n".join(lines) + "\n")
        print("check-android-layout: rewrote %s (%d row(s))" % (BASELINE, len(measured)))
        return 0

    asserts = rust_assertions()
    bad = 0
    for i, (stem, link, header, expr, value, note) in enumerate(measured):
        recorded = int(table[i][4])
        if recorded != value:
            print("MISMATCH %s: the table says %d, the NDK measures %d (%s)"
                  % (expr, recorded, value, note))
            bad += 1
        # The port may carry one assertion per target (the host's arm and the Android one), so a
        # row passes when *some* assertion for that stem/field names the measured value; when none
        # does, the values it does assert are printed for the reader.
        field = None
        m = re.search(r"offsetof[^,]*,\s*(\w+)\s*\)", expr)
        if m:
            field = m.group(1)
        hits = [v for (s, f, v) in asserts if s == stem and f == field]
        if hits and value not in hits:
            print("MISMATCH %s%s: the port asserts %s, the NDK measures %d"
                  % (stem, "" if field is None else "." + field,
                     "/".join(str(v) for v in sorted(set(hits))), value))
            bad += 1
        elif not hits and not args.quiet:
            print("note %s: no port assertion (%s) - the NDK measures %d" % (stem, expr, value))
    print("check-android-layout: %d row(s), %d mismatch(es)" % (len(measured), bad))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
