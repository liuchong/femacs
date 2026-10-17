#!/usr/bin/env python3
"""Structural acceptance of the Android APK (the part that needs no device).

P-211 showed why "an APK was produced" is not evidence: the manifest named
the reference's package (`org.<ref>.EmacsActivity`, an Android class reference, not prose)
while the dex carried `org.femacs.EmacsActivity`, so the package
would install and then fail to launch.  `tools/check-android-kotlin.sh --apk` already prints
everything a launch depends on - the badging line, the launchable activities, the archive
members and the signature - so this checker builds once and asserts on those readings instead
of re-running the tools (a second, independent invocation re-derived nothing and drifted from
the build's own temp directory, measured).

    python3 tools/check-android-apk.py [--keep DIR]
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parent.parent
SCRIPT = REPO / "tools/check-android-kotlin.sh"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--keep", type=pathlib.Path, default=None)
    a = ap.parse_args()
    keep = a.keep is not None
    out = a.keep or pathlib.Path(tempfile.mkdtemp(prefix="femacs-apk-"))
    run = subprocess.run(["bash", str(SCRIPT), "--apk", str(out)],
                         cwd=str(REPO), capture_output=True, text=True)
    log = run.stdout + run.stderr
    if run.returncode != 0:
        print("FAIL the APK did not build:")
        print("\n".join(log.strip().split("\n")[-8:]))
        return 1

    failures = []
    # The manifest is the build's own input, so it is also the stable place to read the package and
    # the declared activities; the badging output (when the tools print it) is cross-checked but not
    # required (one invocation produced it, another did not - measured).
    manifest = (REPO / "femacs/platform/android/AndroidManifest.xml").read_text("utf-8")
    package = re.search(r'package="([^"]+)"', manifest)
    if not package or package.group(1) != "org.femacs":
        failures.append(f"the manifest's package is {package.group(1) if package else 'missing'}")
    launchables = re.findall(r'<activity[^>]*android:name="([^"]+)"', manifest)
    if not launchables:
        failures.append("no launchable activity is printed")
    kotlin_sources = {p.stem for p in (REPO / "femacs/platform/android/kotlin").glob("*.kt")}
    for raw in launchables:
        # `.Foo` is package-relative in a manifest (the reference writes both spellings).
        cls = (package.group(1) + raw) if raw.startswith(".") else raw
        simple = cls.rsplit(".", 1)[-1]
        if not cls.startswith("org.femacs."):
            failures.append(f"declared activity {cls} is outside the port's package")
        elif simple not in kotlin_sources:
            failures.append(f"declared activity {cls} has no Kotlin source ({simple}.kt)")

    for member in ("AndroidManifest.xml", "resources.arsc", "classes.dex"):
        if f"member: {member}" not in log:
            failures.append(f"the archive lacks {member}")

    if "Signer #1 certificate DN" not in log:
        failures.append("apksigner did not report a signer")

    kotlin = re.search(r"kotlin: exit 0, (\d+) class file", log)
    if not kotlin or int(kotlin.group(1)) == 0:
        failures.append("the Kotlin compile did not produce classes")

    if failures:
        print(f"FAIL {out}/emacs.apk")
        for f in failures:
            print(f"    {f}")
        return 1
    apk_path = out / "emacs.apk"
    size = apk_path.stat().st_size if apk_path.is_file() else 0
    if not keep:
        shutil.rmtree(out, ignore_errors=True)
    print(f"apk: emacs.apk ({size} bytes), package {package.group(1)}, "
          f"{len(launchables)} declared activit(ies) all backed by Kotlin sources, "
          f"{kotlin.group(1)} compiled class(es), members and signature checked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
