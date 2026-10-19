#!/usr/bin/env python3
"""Turn a reference `config.h` into the frozen configuration `xtask gen-config` reads.

`docs/baseline/config/*.json` is the frozen input of PORTING.md P-08: for each macro the port
cares about it records one of three states, because the port's condition evaluator needs the
*difference* between "the header does not mention it" and "the header says `#undef`":

    "1"          `#define NAME 1`           (any value, verbatim)
    "undefined"  `/* #undef NAME */`        (the header knows the name and disabled it)
    "absent"     the header says nothing    (configure never tested it)

So a second target - the Android one the platform rows need - is one `configure` run away,
provided the header is turned into this file with the same rules:

    python3 tools/gen-config-json.py build/emacs-ref/src/config.h \
        --target "darwin-arm64 ..." --out /tmp/check.json     # reproduce the frozen one
    python3 tools/gen-config-json.py <android build>/src/config.h \
        --target "android-arm64 ..." --out docs/baseline/config/android-arm64.json

`--keys-from` (default: the committed darwin file) fixes the key set, so two targets are
comparable key for key.

`--preprocessed FILE` makes the *preprocessor* the authority for every key the header mentions:

    clang --target=aarch64-linux-android21 -E -dM -x c config.h.android > /tmp/dM.txt
    python3 tools/gen-config-json.py config.h.android --keys-from-header \
        --preprocessed /tmp/dM.txt --out .../android-arm64.json

Reading the header line by line is wrong for a `#define` inside an `#if` arm that the compiler
does not take: the line is there, the macro is not.  That mistake was live (status 4.805): the
Android configuration recorded `WORDS_BIGENDIAN` as `1` from the `#  define WORDS_BIGENDIAN 1`
line inside `#if defined AC_APPLE_UNIVERSAL_BUILD` - a comment-documented arm the Android
compiler never takes - so the port compiled the *empty* `sfnt_swap16_1`/`sfnt_swap32_1`
(`#ifndef WORDS_BIGENDIAN`), every font table came back byte-swapped, and no font ever
registered.  `_DONT_USE_CTYPE_INLINE_`, `_FORTIFY_SOURCE`, `_GL_BRACKET_BEFORE_ATTRIBUTE`,
`CC`, `_GL_STDALIGN_NEEDS_STDDEF`, `_GL_EXTERN_INLINE_STDHEADER_BUG`,
`_GL_INLINE_HEADER_CONST_PRAGMA`, `_GL_INLINE`, `ND`/`WUR`/`BD`/`AD` and
`_GL_HAVE___HAS_C_ATTRIBUTE` were wrong the same way.

A name the header mentions but the dump does not define is recorded as `undefined` (the third
state P-08 needs: the header knows the name and this configuration disables it), and every
correction is listed on stdout.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
DEFAULT_KEYS = REPO / "docs/baseline/config/darwin-arm64.json"

DEFINE = re.compile(r"^#\s*define\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+(.*?))?\s*$")
UNDEF = re.compile(r"^/\*\s*#\s*undef\s+([A-Za-z_][A-Za-z0-9_]*)\s*\*/")


def read_header(path: pathlib.Path) -> dict[str, str]:
    defines: dict[str, str] = {}
    undefs: set[str] = set()
    for line in path.read_text("utf-8", "surrogateescape").split("\n"):
        m = UNDEF.match(line.strip())
        if m:
            undefs.add(m.group(1))
            continue
        m = DEFINE.match(line.strip())
        if m:
            defines[m.group(1)] = (m.group(2) or "1").strip()
    out: dict[str, str] = {}
    for name, value in defines.items():
        out[name] = value
    for name in undefs:
        out.setdefault(name, "undefined")
    return out


DUMP_DEFINE = re.compile(r"#define\s+([A-Za-z_][A-Za-z0-9_]*)(?:\s+(.*))?$")


def read_dump(path: pathlib.Path) -> dict[str, str]:
    """The macro set a `clang -E -dM` run actually produced, with each body verbatim."""
    out: dict[str, str] = {}
    for line in path.read_text("utf-8", "surrogateescape").split("\n"):
        m = DUMP_DEFINE.match(line)
        if m:
            # The body is kept *verbatim, empty included*: `#define X /**/` and `#define X`
            # both expand to nothing, and the three-state convention records neither as `1`
            # (`FLEXIBLE_ARRAY_MEMBER`/`PTY_NAME_SPRINTF` are the port's two).  `""` means
            # "defined, with an empty body" and the caller keeps the header's own spelling.
            out[m.group(1)] = (m.group(2) or "").strip()
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("header", type=pathlib.Path)
    ap.add_argument("--target", default="unnamed target")
    ap.add_argument("--keys-from", type=pathlib.Path, default=DEFAULT_KEYS)
    ap.add_argument("--out", type=pathlib.Path, default=None)
    # `--macro NAME=VALUE` records a value that is *not* a config.h macro.  `HAVE_SIGNBIT` is the
    # case that needed it: floatfns.c tests `#ifndef signbit`, which is a preprocessor fact about
    # the compiler's <math.h>, and the darwin JSON's note has always said it was measured that
    # way - but nothing carried the measurement into a second target, so Android's configuration
    # lacked the constant and `floatfns.rs` would not compile there (P-227).
    ap.add_argument("--macro", action="append", default=[], metavar="NAME=VALUE")
    # The key list used to come from darwin's JSON alone, so a macro only another target defines
    # (`HAVE_ANDROID`) could not appear anywhere - and the port's `#ifdef` gate for it could never
    # be opened (P-229).  `--keys-from-header` unions in every name this header defines or undefs.
    ap.add_argument("--keys-from-header", action="store_true")
    # The preprocessor's answer, for the `#define`s that sit in arms the compiler does not take.
    ap.add_argument("--preprocessed", type=pathlib.Path, default=None)
    # Which file's key order the output follows.  Default: the file being written, so a
    # regeneration of an unchanged configuration is byte-identical.
    ap.add_argument("--order-from", type=pathlib.Path, default=None)
    a = ap.parse_args()

    header = read_header(a.header)
    corrected: list[tuple[str, str]] = []
    if a.preprocessed is not None:
        dump = read_dump(a.preprocessed)
        for name, value in list(header.items()):
            if name in dump:
                header[name] = dump[name] or header.get(name, "1")
            elif value != "undefined":
                corrected.append((name, value))
                header[name] = "undefined"
    if corrected:
        print(f"{len(corrected)} macro(s) the header spells but this configuration never "
              f"defines (recorded as undefined):")
        for name, value in corrected:
            print(f"  {name}: {value!r} -> undefined")
    keys = json.loads(a.keys_from.read_text("utf-8"))["macros"]
    # Header macros beyond the shared key list are *cfg features*, never constants: `gen-config`
    # emits `pub const <name>` for every key, and the mirrors define plenty of these names
    # themselves (`CHECK_LISP_OBJECT_TYPE`, `__STDC_CONSTANT_MACROS`...), so putting them in
    # `macros` made the port define them twice (status 4.633).  They go in their own section,
    # which `config_features()` reads and `gen-config` ignores.
    features = {}
    if a.keys_from_header:
        import re as _re
        for name, value in header.items():
            if not _re.fullmatch(r"[A-Z_][A-Z0-9_]*", name):
                continue
            if name in keys or name in features:
                continue
            features[name] = value
    macros = {k: header.get(k, "absent") for k in keys}
    for spec in a.macro:
        name, _, value = spec.partition("=")
        if not name or not value:
            raise SystemExit(f"--macro wants NAME=VALUE, not {spec!r}")
        if name in macros:
            # A name the shared key set already carries is a configuration *constant*
            # (`HAVE_SIGNBIT` is the one), so it stays in `macros`.
            macros[name] = value
            continue
        # A target-only name is a *gate*, not a constant: the mirrors define `RENAME_EXCL`
        # (`sysdep.rs` 3591, from `<stdio.h>`) and `_CS_PATH` (`emacs.rs` 518) themselves, so a
        # `pub const` of the same name in `config.rs` is a duplicate definition (E0428, measured
        # 4.805 in `src/sysdep.c`'s checking configuration).  `features` is read by
        # `xtask config_features()` and ignored by `gen-config`, which is what a gate wants.
        keys = [*keys, name]
        features[name] = value
    # The key order of an existing `--out` file is kept, so regenerating an unchanged
    # configuration writes the same bytes and a real change shows up as the changed line alone.
    # Without this the section order follows the current text scan, and a one-line correction
    # arrives as a whole-file rewrite (measured 4.805: 387 changed lines around 36 corrections).
    order: list[str] = []
    order_from = a.order_from if a.order_from is not None else a.out
    if order_from is not None and order_from.is_file():
        try:
            previous = json.loads(order_from.read_text("utf-8"))
        except (OSError, ValueError):
            previous = {}
        for section in ("macros", "features"):
            for name in previous.get(section, {}):
                if name not in order:
                    order.append(name)

    def reordered(section: dict[str, str]) -> dict[str, str]:
        out = {name: section[name] for name in order if name in section}
        for name, value in section.items():
            out.setdefault(name, value)
        return out

    doc = {"target": a.target, "source": str(a.header), "macros": reordered(macros)}
    if features:
        doc["features"] = reordered(features)
    text = json.dumps(doc, indent=2) + "\n"
    if a.out is None:
        sys.stdout.write(text)
        return 0
    a.out.parent.mkdir(parents=True, exist_ok=True)
    a.out.write_text(text, encoding="utf-8")
    # A target-only macro (`_CS_PATH`, `ANDROID_API_GE_9`) has no counterpart in the file this run
    # is compared against; counting it as a difference hid the reading behind a `KeyError`
    # (status 4.783).  Only the keys both documents carry are compared.
    other = json.loads(a.keys_from.read_text("utf-8"))["macros"]
    shared = [k for k in keys if k in other]
    differing = sum(1 for k in shared if macros[k] != other[k])
    print(f"wrote {a.out} ({len(macros)} macro(s), {differing} differ from {a.keys_from.name})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
