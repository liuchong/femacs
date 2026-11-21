#!/usr/bin/env python3
"""Check Fe launcher options against the port and upstream startup sources."""

import argparse
import json
from pathlib import Path
import re
import sys
from urllib.request import urlopen


ROOT = Path(__file__).resolve().parent.parent
UPSTREAM_REPO = "emacs-mirror/emacs"
STARTUP_FILES = ("lisp/startup.el", "lisp/term/ns-win.el")
LAUNCHER_OPTIONS = ("-R", "--reset-args", "-1", "--sequential")
QUOTED_OPTION = re.compile(r'"(--?[A-Za-z0-9][A-Za-z0-9-]*)"')
MAX_SOURCE_BYTES = 1_000_000


def download(url):
    with urlopen(url, timeout=10) as response:
        data = response.read(MAX_SOURCE_BYTES + 1)
    if len(data) > MAX_SOURCE_BYTES:
        raise ValueError(f"source response too large: {url}")
    return data.decode("utf-8")


def check(label, files, read):
    conflicts = []
    for filename in files:
        source = read(filename)
        if filename.startswith("src/") and '"--help"' not in source:
            raise ValueError(f"startup option table missing from {label}/{filename}")
        if filename == "lisp/startup.el" and "command-switch-alist" not in source:
            raise ValueError(f"startup code missing from {label}/{filename}")
        for line, statement in enumerate(source.splitlines(), 1):
            for native in QUOTED_OPTION.findall(statement):
                for launcher in LAUNCHER_OPTIONS:
                    # The editor also accepts unambiguous abbreviated long options.
                    if native == launcher or (
                        launcher.startswith("--") and native.startswith(launcher)
                    ):
                        conflicts.append(f"{label}/{filename}:{line}: {launcher} matches {native}")
    if conflicts:
        raise ValueError("\n".join(conflicts))
    print(f"{label}: no launcher option conflicts")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", action="store_true", help="also check current upstream master")
    args = parser.parse_args()

    check("port", ("src/emacs.rs", *STARTUP_FILES), lambda name: (ROOT / name).read_text())
    upstream_files = ("src/emacs.c", *STARTUP_FILES)
    reference = ROOT / "emacs"
    if (reference / "src/emacs.c").is_file():
        check("reference", upstream_files, lambda name: (reference / name).read_text())
    if args.upstream:
        response = download(f"https://api.github.com/repos/{UPSTREAM_REPO}/git/ref/heads/master")
        commit = json.loads(response)["object"]["sha"]
        if not re.fullmatch(r"[0-9a-f]{40}", commit):
            raise ValueError("invalid upstream commit identity")
        base = f"https://raw.githubusercontent.com/{UPSTREAM_REPO}/{commit}"
        check(f"upstream {commit}", upstream_files, lambda name: download(f"{base}/{name}"))


if __name__ == "__main__":
    try:
        main()
    except (KeyError, OSError, ValueError) as error:
        print(f"CLI option check failed: {error}", file=sys.stderr)
        raise SystemExit(1)
