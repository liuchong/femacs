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

`--check` is the gate form (`tools/check-authorized-platforms.sh` runs it): the 52 rows above are
`docs/baseline/android-unasserted.tsv`, and it fails both on a record that is neither asserted for
the target nor listed, and on a row that has become asserted (so the table cannot go stale).

Two limits worth knowing.  The unit is the *record*: a record with one surviving live assertion
counts as covered, so disabling a single assertion of a multi-assertion record does not show up
(measured: disabling `MonitorInfo`'s `size_of` alone left it "covered"; disabling all six made the
checker report `NEW MonitorInfo`, which is the self-test this tool was validated with).
And the `#[cfg]` evaluator understands `all`/`any`/`not`/`feature = "…"`; any other predicate
(`target_os = …`, `test`) is taken as unsatisfied, which biases towards reporting a record.
"""
import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
BASELINE = REPO / "docs/baseline/android-unasserted.tsv"
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


X11 = {"XExtData", "Visual", "Depth", "Screen", "XImage", "XColor", "XErrorEvent",
       "XModifierKeymap", "XSizeHintsAspect", "XSizeHints", "XWMHints", "XVisualInfo", "XEvent"}
BSD = {"pcred", "ucred", "eproc", "kinfo_proc"}
NOT_BUILT = {"comp_t", "f_reloc_t", "reloc_array_t", "static_obj_t", "imm_reloc_t"}


def classify(name, path):
    """(category, why) for a record that the target's configuration does not assert."""
    if ("/xlib.rs" in path or name in X11 or name.startswith("xcb_") or name.startswith("Xrm")
            or name.startswith("Xpm")):
        return "x11", "that unit is not part of the Android build"
    if name in NOT_BUILT or "/treesit.rs" in path or "/sqlite.rs" in path or "/sound.rs" in path:
        return "not-built", "that unit is not part of the Android build"
    if name in BSD:
        return "bsd", "C reaches these under #ifdef BSD_SYSTEM / HAVE_SYS_USER_H, false here"
    return "own", "C defines this record itself, so its layout cannot differ between targets"


def read_baseline():
    rows = {}
    if not BASELINE.is_file():
        return rows
    for line in BASELINE.read_text().splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) >= 2:
            rows[parts[0]] = (parts[1], parts[2] if len(parts) > 2 else "", parts[3] if len(parts) > 3 else "")
    return rows


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
    ap.add_argument("--check", action="store_true",
                    help="fail on a record that is neither asserted for the target nor in the "
                         "baseline, and on a baseline row that has become asserted")
    ap.add_argument("--write-baseline", action="store_true")
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
            # `src/keyboard.c` -> src/keyboard.rs, `lib/regex.c` -> lib/regex.rs
            stem = line[:-2] if line.endswith(".c") else line
            listed.add("%s.rs" % stem)
        unit_files = listed

    seen, live = {}, {}
    for src in sorted(list((REPO / "src").rglob("*.rs"))
                      + list((REPO / "lib").rglob("*.rs"))
                      + [p for d in ("lib-src", "lwlib", "oldXMenu", "exec", "nt")
                         for p in (REPO / d).rglob("*.rs")]):
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

    if args.write_baseline:
        lines = ["# Records whose layout assertions are all gated to an arm this configuration does "
                 "not",
                 "# compile.  Written by `python3 tools/check-android-assertions.py "
                 "--write-baseline`;",
                 "# `--check` fails on a record that is neither asserted for the target nor listed "
                 "here.",
                 "# record\tfile\tcategory\twhy it is not a hazard for " + args.config]
        for name in sorted(missing):
            path = sorted(missing[name])[0]
            cat, why = classify(name, path)
            lines.append("\t".join([name, path, cat, why]))
        BASELINE.write_text("\n".join(lines) + "\n")
        print("check-android-assertions: wrote %s (%d row(s))" % (BASELINE, len(missing)))
        return 0

    if args.check:
        base = read_baseline()
        new = sorted(k for k in missing if k not in base)
        stale = sorted(k for k in base if k not in missing)
        for name in new:
            print("NEW %s (%s): no assertion this configuration compiles, and no baseline row"
                  % (name, sorted(missing[name])[0]))
        for name in stale:
            print("STALE %s: the baseline lists it, but it now has an assertion that compiles - "
                  "drop the row" % name)
        print("check-android-assertions: %d new, %d stale, %d baselined"
              % (len(new), len(stale), len(base) - len(stale)))
        return 1 if (new or stale) else 0

    rows = sorted(missing.items(), key=lambda kv: sorted(kv[1])[0])
    for name, files in (rows if args.list else rows[:args.top]):
        print("  %-34s %s" % (name, ", ".join(sorted(files)[:2])))
    if not args.list and len(rows) > args.top:
        print("  ... %d more (--list for all)" % (len(rows) - args.top))
    return 0


if __name__ == "__main__":
    sys.exit(main())
