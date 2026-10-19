#!/usr/bin/env python3
"""Stage the `assets/` tree C's `java/Makefile.in` packs into the Android APK.

The Android port reads its Lisp from the APK's assets (it has no dump file), so an APK without
`assets/` cannot start however well its native library links.  C builds that tree in
`java/Makefile.in` 200-297:

  * `cp -r` the four trees into `install_temp/assets` (206-219);
  * delete the junk the reference's own packaging deletes (220-235: `.gitignore`, `.DS_Store`, `#*`, `.#*`,
    `*~`, `*.orig`, `ChangeLog*`, `[mM]akefile*[.-]in`, `Makefile`);
  * gzip `.el` files whose `.elc` is at least 1536 bytes (236-250) and split the largest files
    through `lib-src/asset-directory-tool` (250-297), because `aapt2` cannot compress what the
    loader expects to be stored.

This tool does the first two steps and reports what it staged.  The gzip and large-file split are
the next step and are named as missing rather than approximated (`docs/android-app-readiness.md`).

    python3 tools/gen-android-assets.py --out DIR      # stage DIR/assets/...
    python3 tools/gen-android-assets.py --report       # what would be staged, and how big
"""

from __future__ import annotations

import argparse
import gzip
import pathlib
import shutil
import subprocess
import time
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
# C's four sources.  `src/bitmaps` is where this reference keeps the bitmaps; the port's copy of
# that tree lives under `etc/images` (they are the same files, and `femacs/src/bitmaps` does not
# exist - recorded rather than invented).
TREES = (("lisp", REPO / "femacs/lisp"), ("etc", REPO / "femacs/etc"),
         ("info", REPO / "femacs/info"), ("bitmaps", REPO / "femacs/src/bitmaps"))
# `info/` is a *generated* tree (makeinfo, P-102/P-223): when it is not built yet the other three
# are still staged, and the absence is reported rather than treated as an error.
OPTIONAL = {"info"}
JUNK = (".gitignore", ".DS_Store")
JUNK_GLOB = ("#*", ".#*", "*~", "*.orig", "ChangeLog*", "Makefile", "makefile.in", "Makefile.in")


def junk(path: pathlib.Path) -> bool:
    if path.name in JUNK:
        return True
    return any(path.match(p) for p in JUNK_GLOB)


def stage(out: pathlib.Path, report_only: bool) -> tuple[int, int]:
    files = total = 0
    for name, src in TREES:
        if not src.is_dir():
            if name in OPTIONAL:
                print(f"note: {src} is not built yet (generated); staging the rest")
                continue
            print(f"missing source tree: {src}", file=sys.stderr)
            return 0, 0
        dst_root = out / "assets" / name
        for path in sorted(src.rglob("*")):
            if path.is_dir() or junk(path):
                continue
            files += 1
            total += path.stat().st_size
            if not report_only:
                dst = dst_root / path.relative_to(src)
                dst.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, dst)
    return files, total


def gzip_large_el(out: pathlib.Path) -> int:
    """C 236-250: gzip the `.el` of every `.elc` at least 1536 bytes, plus `etc/publicsuffix.txt`.

    `gzip -9n` is what C runs; `mtime=0` is Python's way to leave the timestamp out, which is the
    `-n` half of that flag and keeps two builds of the same tree byte-identical.
    """
    count = 0
    lisp = out / "assets" / "lisp"
    for elc in sorted(lisp.rglob("*.elc")):
        if elc.stat().st_size < 1536:
            continue
        el = elc.with_suffix(".el")
        if not el.is_file() or el.with_suffix(".el.gz").exists():
            continue
        with el.open("rb") as src, gzip.GzipFile(str(el) + ".gz", "wb", 9, mtime=0) as dst:
            shutil.copyfileobj(src, dst)
        el.unlink()
        count += 1
    suffix = out / "assets" / "etc" / "publicsuffix.txt"
    if suffix.is_file() and not suffix.with_suffix(".txt.gz").exists():
        with suffix.open("rb") as src, gzip.GzipFile(str(suffix) + ".gz", "wb", 9, mtime=0) as dst:
            shutil.copyfileobj(src, dst)
        suffix.unlink()
        count += 1
    return count


def build_metadata(out: pathlib.Path) -> None:
    """C 252-260: `assets/version` and `assets/build_info`, from the checkout and the host."""
    assets = out / "assets"
    try:
        head = subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"],
                              capture_output=True, text=True).stdout.strip() or "Unknown"
        branch = subprocess.run(["git", "-C", str(REPO), "rev-parse", "--abbrev-ref", "HEAD"],
                                capture_output=True, text=True).stdout.strip() or "Unknown"
    except OSError:
        head = branch = "Unknown"
    (assets / "version").write_text(f"{head}\n{branch}\n", encoding="utf-8")
    # C 261: `{ hostname; date +%s; }` - **two** lines, and the second one is the file's whole
    # point: `version.el`'s `android-read-build-time` is `(end-of-line)` followed by
    # `(read (current-buffer))`, i.e. it reads the line *after* the host name.  Staging only the
    # host name made that `read` hit the end of the buffer, and the load of `version.elc` died with
    # `End of file during parsing: #<killed buffer>` (status 4.808).
    (assets / "build_info").write_text(
        f"{subprocess.run(['hostname'], capture_output=True, text=True).stdout.strip()}\n"
        f"{int(time.time ())}\n",
        encoding="utf-8")


def directory_tree(out: pathlib.Path) -> str:
    """C 261-263: `asset-directory-tool assets assets/directory-tree` - the port's own copy."""
    tool = REPO / "build" / "port" / "lib-src" / "asset-directory-tool"
    if not tool.is_file():
        return ("asset-directory-tool is not built (xtask link/`build-tu lib-src/asset-directory-tool.c` "
                "first): directory-tree not written")
    run = subprocess.run([str(tool), str(out / "assets"), str(out / "assets" / "directory-tree")],
                         capture_output=True, text=True)
    if run.returncode != 0:
        return f"asset-directory-tool failed: {(run.stderr or run.stdout).strip()[:200]}"
    return "directory-tree written"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=pathlib.Path, default=None,
                    help="directory to stage assets/ into (the APK builder's working dir)")
    ap.add_argument("--report", action="store_true", help="count only, write nothing")
    a = ap.parse_args()
    if a.out is None and not a.report:
        sys.exit("give --out DIR (to stage) or --report (to count)")
    files, total = stage(a.out or pathlib.Path("/nonexistent"), a.report or a.out is None)
    if a.report or a.out is None:
        print(f"assets: {files} file(s), {total / (1024 * 1024):.1f} MiB (report only)")
        return 0 if files else 1
    gz = gzip_large_el(a.out)
    build_metadata(a.out)
    note = directory_tree(a.out)
    print(f"assets: {files} file(s), {total / (1024 * 1024):.1f} MiB staged in {a.out}; "
          f"{gz} file(s) gzipped (C 236-250); {note}")
    return 0 if files else 1


if __name__ == "__main__":
    raise SystemExit(main())
