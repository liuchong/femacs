#!/usr/bin/env python3
"""Which `#[repr(C)]` records have layout assertions, but none that the Android build compiles?

Layout bugs on this port have all had the same shape: a record mirrored from the *host* while the
target's header lays it out differently (`sigjmp_buf` 68 bytes short, `struct dirent` with
`d_name` at 19 instead of 11, `stack_t` with `ss_size` and `ss_flags` swapped, `siginfo_t` 104 vs
128 - statuses 4.788, 4.792, 4.830, 4.831).  Each was caught only when the device crashed.

`sfnt_font_info` (status 4.832) showed the gap class: it *had* assertions, and every one of them
was gated `not (feature = "have_window_system")` - the configuration the Android leg builds has
`HAVE_WINDOW_SYSTEM` 1, so nothing checked that record at all.

This tool answers, for one configuration: which records appear in an assertion `const` block, but
never in a block whose `#[cfg]` the configuration satisfies.  It is a work list, not a verdict: a
record may be layout-independent across the two targets (a pair of `*mut c_void`), and the port
may assert it somewhere this parser does not see.

    tools/check-android-assertions.py                 # android-arm64.json
    tools/check-android-assertions.py --config stem
    tools/check-android-assertions.py --units frozen-build-sources-android.txt

The `--units` form is the useful one, and its first full run triaged as follows (status 4.832):
395 records carry assertions, 53 of them none that the Android configuration compiles.  One was a
real gap and is now closed - `MonitorInfo` (`include/frame.rs`) had its numbers only in
`androidfns.rs`'s local copy, which is gated `not (have_window_system)`, while
`android_make_monitor_attribute_list` fills the record by value in the configuration Android builds.
The rest fall in four groups, none of them a hazard for this target:

  * X11/xcb/Xrm records (`include/xlib.rs`, `image.rs`'s XpmAttributes) - those units are not built
    for Android at all;
  * BSD/Darwin process records (`pcred`, `ucred`, `eproc`, `kinfo_proc` in `sysdep.rs`) - C reaches
    them under `#ifdef BSD_SYSTEM`/`HAVE_SYS_USER_H`, which this configuration leaves false;
  * records and unions C defines itself (`emacs_align_type`, `emacs_pix_container`, `it_stack`,
    `__sigaction_u`) whose layout cannot differ between the two targets;
  * units outside the Android build (`comp.rs`'s relocation records, `treesit.rs`, `sqlite.rs`,
    `sound.rs`).

Re-run it after any change to a `#[cfg]` around a layout assertion: the class it is here to catch
is exactly "the live record and the asserted record are different arms of one `#if`".
"""
import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
STRUCT = re.compile(r"size_of::<(\w+)>\s*\(\)|offset_of!\s*\(\s*(\w+)\s*,", re.S)


def features(path):
    data = json.loads(path.read_text())
    out = set()
    for section in ("macros", "features"):
        for name, value in (data.get(section) or {}).items():
            low = name.lower()
            out.add(low if value in ("1", "true") else "not_" + low)
    return out


class Cfg:
    """A tiny evaluator for `all(..)`, `any(..)`, `not(..)` and `feature = "x"`."""

    def __init__(self, text):
        self.t = text
        self.i = 0

    def ws(self):
        while self.i < len(self.t) and self.t[self.i] in " \t\n,":
            self.i += 1

    def eat(self, s):
        self.ws()
        if self.t.startswith(s, self.i):
            self.i += len(s)
            return True
        return False

    def expr(self, feats):
        self.ws()
        for kw in ("all", "any", "not"):
            if self.eat(kw) and self.eat("("):
                vals = []
                while True:
                    self.ws()
                    if self.eat(")"):
                        break
                    vals.append(self.expr(feats))
                if kw == "all":
                    return all(vals)
                if kw == "any":
                    return any(vals)
                return not (vals[0] if vals else True)
        m = re.match(r'feature\s*=\s*"([^"]+)"', self.t[self.i:])
        if m:
            self.i += m.end()
            return m.group(1) in feats
        # anything else (`target_os = "..."`, `test`, ...) is taken as not satisfied
        while self.i < len(self.t) and self.t[self.i] not in ",)":
            self.i += 1
        return False

    def value(self, feats):
        return self.expr(feats)


def attributes_before(text, start):
    """The `#[...]` attributes directly above `text[start]`, as their inner text.

    Multi-line attributes are the norm here (`#[cfg (all (feature = "have_android",\n\t
    feature = "not_android_stubify"))]`), so this walks balanced brackets instead of lines.
    """
    out = []
    i = start
    while True:
        # skip whitespace before the item
        while i > 0 and text[i - 1] in " \t\n\r":
            i -= 1
        if i < 2 or text[i - 1] != "]":
            break
        depth, j = 0, i - 1
        while j >= 0:
            if text[j] == "]":
                depth += 1
            elif text[j] == "[":
                depth -= 1
                if depth == 0:
                    break
            j -= 1
        if j < 1 or text[j - 1] != "#":
            break
        out.append(text[j + 1:i - 1].strip())
        i = j - 1
    return list(reversed(out))


def _item_body(text, start):
    """The text of the item that starts at `start`, up to its terminating `;` at depth 0."""
    i, depth = start, 0
    while i < len(text):
        c = text[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == ";" and depth == 0:
            break
        i += 1
    return text[start:i]


def blocks(text):
    """(cfg expressions, body) for every layout assertion the port spells, in any of its forms:

        const _: () = core::assert! (...);        const _: () = { ... };
        static_assert! (...);                     (lisp.h's macro, `include/lisp.rs`)
    """
    starts = [(m.start(), m.end()) for m in re.finditer(r"const _: \(\) = ", text)]
    starts += [(m.start(), m.start()) for m in re.finditer(r"static_assert! *\(", text)]
    for item_start, body_start in starts:
        body = _item_body(text, body_start)
        if "size_of::<" not in body and "offset_of!" not in body:
            continue
        cfgs = []
        for attr in attributes_before(text, item_start):
            interior = attr[3:].strip() if attr.startswith("cfg") else None
            if interior is None:
                continue
            if interior.startswith("(") and interior.endswith(")"):
                interior = interior[1:-1]
            cfgs.append(interior)
        yield cfgs, body


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", default="android-arm64")
    ap.add_argument("--units", default=None,
                    help="restrict to the files that build for this configuration "
                         "(docs/baseline/frozen-build-sources-*.txt)")
    ap.add_argument("--top", type=int, default=15)
    ap.add_argument("--list", action="store_true")
    args = ap.parse_args()
    path = REPO / "docs/baseline/config" / (args.config + ".json")
    if not path.is_file():
        print("check-android-assertions: no %s" % path, file=sys.stderr)
        return 2
    feats = features(path)
    unit_files = None
    if args.units:
        listed = set()
        for line in (REPO / "docs/baseline" / args.units).read_text().split():
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            # `src/keyboard.c` -> femacs/src/keyboard.rs, `lib/regex.c` -> femacs/lib/regex.rs
            stem = line[:-2] if line.endswith(".c") else line
            listed.add("femacs/%s.rs" % stem)
        unit_files = listed

    seen, live = {}, {}
    for src in sorted(list((REPO / "femacs/src").rglob("*.rs"))
                      + list((REPO / "femacs/lib").rglob("*.rs"))
                      + [p for d in ("lib-src", "lwlib", "oldXMenu", "exec", "nt")
                         for p in (REPO / "femacs" / d).rglob("*.rs")]):
        try:
            text = src.read_text()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        rel = str(src.relative_to(REPO))
        for cfgs, body in blocks(text):
            names = {a or b for a, b in STRUCT.findall(body)}
            ok = all(Cfg(expr).value(feats) for expr in cfgs)
            if unit_files is not None and rel not in unit_files and "/include/" not in rel:
                continue
            for name in names:
                seen.setdefault(name, set()).add(rel)
                if ok:
                    live.setdefault(name, set()).add(rel)

    missing = {k: v for k, v in seen.items() if k not in live}
    print("check-android-assertions: %d record(s) carry assertions; %d of them have none that "
          "%s compiles" % (len(seen), len(missing), args.config))
    rows = sorted(missing.items(), key=lambda kv: sorted(kv[1])[0])
    for name, files in (rows if args.list else rows[:args.top]):
        print("  %-34s %s" % (name, ", ".join(sorted(files)[:2])))
    if not args.list and len(rows) > args.top:
        print("  ... %d more (--list for all)" % (len(rows) - args.top))
    return 0


if __name__ == "__main__":
    sys.exit(main())
