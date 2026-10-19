#!/usr/bin/env python3
"""Every `feature = "X"` the port gates on must be a name the frozen configuration defines.

`xtask config_features ()` builds the `--cfg` set from `docs/baseline/config/<stem>.json`: one
`feature="<lower-cased macro name>"` per macro the configuration calls true, and
`feature="not_<lower-cased name>"` for the ones it calls false/absent/undefined.  A gate that
spells the name differently is *silently dead* - the arm is compiled out while C's `#ifdef` is
live - and nothing else notices: `extract-abi.py` compares names, `layout-check` reads the host's
`spikes/layout_c.c`, and the device only sees the result.

That is not hypothetical: `struct sfnt_font_info` measures 608 bytes in the port and 632 in C on
the frozen Android configuration, and the difference is exactly its three `HAVE_HARFBUZZ` members,
which the port gates on `feature="harfbuzz"` (345 sites) while the configuration defines
`feature="have_harfbuzz"` (status 4.832).

    tools/check-feature-names.py                # the audit for the configuration in FEMACS_CONFIG
    tools/check-feature-names.py --config stem  # a specific docs/baseline/config/<stem>.json

This is an audit, not a gate: a name it reports may be a false positive of the stem match, and the
first run's six findings triaged as follows (status 4.832).

  use_pthread x22   `gmalloc.c` defines `USE_PTHREAD` itself (C 35), so C's arms are live - but
                    gmalloc is not built on this target (`SYSTEM_MALLOC` 1), so the gates are inert.
  have_xim x10      X11-only; `HAVE_XIM` is undefined for this configuration.
  utmp_h x8         `HAVE_UTMP_H` is 1 and `HAVE_UTMPXNAME` undefined, so C reaches
                    `UTMP_NAME_FUNCTION(x) utmpname (x)`; the port's `utmp_h` arm is what that is,
                    and the `android` arms beside it are shadowed in C as well.
  android x4        `__ANDROID__` in `readutmp.h` 181/270, both shadowed (HAVE_UTMP_H /
                    HAVE_UTMPXNAME decide first, and C's BOOT_TIME arm needs `!defined BOOT_TIME`).
  posix_memalign x1 C tests `USE_POSIX_MEMALIGN` (C 1222), which nothing defines - the stem match
                    against `HAVE_POSIX_MEMALIGN` is what makes it look live.
  not_usable_fionread x1  the C site is `#include <sys/filio.h>` under `USABLE_FIONREAD &&
                    USG5_4`; `USG5_4` is undefined, so C's include is dead too.
"""
import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
GATE = re.compile(r'feature\s*=\s*"([A-Za-z0-9_]+)"')


def stem(name):
    """`HAVE_HARFBUZZ`/`have_harfbuzz`/`harfbuzz` -> `harfbuzz`."""
    low = name.lower()
    for prefix in ("have_", "use_", "not_", "defined_"):
        if low.startswith(prefix):
            return low[len(prefix):]
    return low


def config_features(path):
    """(defined feature names, stem -> True for every macro the configuration calls true)."""
    data = json.loads(path.read_text())
    defined, truthy_stems = set(), {}
    for section in ("macros", "features"):
        for name, value in (data.get(section) or {}).items():
            low = name.lower()
            if value in ("1", "true"):
                defined.add(low)
                truthy_stems[stem(name)] = True
            else:
                defined.add("not_" + low)
    return defined, truthy_stems


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", default=None)
    ap.add_argument("--top", type=int, default=20)
    args = ap.parse_args()
    stem_name = args.config or "android-arm64"
    path = REPO / "docs/baseline/config" / (stem_name + ".json")
    if not path.is_file():
        print("check-feature-names: no %s" % path, file=sys.stderr)
        return 2
    defined, truthy = config_features(path)

    used = {}
    for src in sorted(list((REPO / "femacs/src").rglob("*.rs"))
                      + list((REPO / "femacs/lib").rglob("*.rs"))
                      + [p for d in ("lib-src", "lwlib", "oldXMenu", "exec", "nt")
                         for p in (REPO / "femacs" / d).rglob("*.rs")]):
        try:
            text = src.read_text()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        for m in GATE.finditer(text):
            name = m.group(1)
            used.setdefault(name, {"count": 0, "files": set()})
            used[name]["count"] += 1
            used[name]["files"].add(str(src.relative_to(REPO)))

    # Only a name whose *stem* the configuration calls true is a finding: a gate on a macro the
    # configuration leaves false is the "carry both arms" rule working as intended.
    missing = {k: v for k, v in used.items()
               if k not in defined and truthy.get(stem(k))}
    total_missing = sum(v["count"] for v in missing.values())
    print("check-feature-names: %d gate name(s) referenced; %d name(s) spell a macro the "
          "configuration calls true but that no feature defines (%d site(s))"
          % (len(used), len(missing), total_missing))
    if missing:
        rows = sorted(missing.items(), key=lambda kv: -kv[1]["count"])
        for name, info in rows[:args.top]:
            files = ", ".join(sorted(info["files"])[:3])
            print("  %-34s %5d site(s)  %s%s"
                  % (name, info["count"], files,
                     "" if len(info["files"]) <= 3 else " (+%d files)" % (len(info["files"]) - 3)))
        if len(rows) > args.top:
            print("  ... %d more name(s)" % (len(rows) - args.top))
        print("  total: %d dead gate site(s)" % total_missing)
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
