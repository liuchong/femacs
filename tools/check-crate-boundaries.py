#!/usr/bin/env python3
"""Verify crate isolation between editor targets and standalone language crates.

Checks:
1. Standalone language crates (`lisp-vm`, `lisp-core`, and `lisp-std` if present)
   never transitively pull `cc`, `cmake`, `pkg-config`, `bindgen`, C TLS/crypto (`ring`,
   `aws-lc-*`, `openssl-sys`), C SQLite (`rusqlite`, `libsqlite3-sys`), or other C `-sys` crates.
2. Editor/tooling crates (`femacs-host`, `xtask`, `femacs`, `femacs-tools`) never
   transitively pull standalone language batteries (`lisp-std`, `rustyline`, `ureq`,
   `rustls`, `rustls-rustcrypto`, `webpki-roots`, `redb`).
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

FORBIDDEN_C_BUILD_CRATES = frozenset({
    "cc",
    "cmake",
    "pkg-config",
    "bindgen",
    "ring",
    "aws-lc-rs",
    "aws-lc-sys",
    "rusqlite",
    "libsqlite3-sys",
    "openssl-sys",
    "curl-sys",
    "libgit2-sys",
    "zstd-sys",
    "zlib-sys",
    "bzip2-sys",
    "lzma-sys",
    "libz-sys",
})

ALLOWED_FFI_BINDING_SYS_CRATES = frozenset({
    "windows-sys",
})

FORBIDDEN_EDITOR_BATTERIES = frozenset({
    "lisp-std",
    "rustyline",
    "ureq",
    "rustls",
    "rustls-rustcrypto",
    "webpki-roots",
    "redb",
})


def is_forbidden_c_dep(pkg_name: str) -> bool:
    if pkg_name in FORBIDDEN_C_BUILD_CRATES:
        return True
    if pkg_name.endswith("-sys") and pkg_name not in ALLOWED_FFI_BINDING_SYS_CRATES:
        return True
    return False


def resolve_tree(pkg: str, extra_args: list[str] | None = None) -> set[str]:
    cmd = ["cargo", "tree", "-p", pkg, "-e", "normal,build", "--prefix", "none"]
    if extra_args:
        cmd.extend(extra_args)
    proc = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True, check=False)
    if proc.returncode != 0:
        raise RuntimeError(
            f"`{' '.join(cmd)}` failed (rc={proc.returncode}):\n{proc.stderr.strip()}"
        )
    pkgs: set[str] = set()
    for line in proc.stdout.splitlines():
        token = line.strip().split(" ", 1)[0]
        if token:
            pkgs.add(token)
    return pkgs


def workspace_package_names() -> set[str]:
    proc = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=REPO,
        capture_output=True,
        text=True,
        check=True,
    )
    import json

    meta = json.loads(proc.stdout)
    return {pkg["name"] for pkg in meta.get("packages", [])}


def main() -> int:
    existing = workspace_package_names()
    failures: list[str] = []

    standalone_profiles: list[tuple[str, list[str]]] = [
        ("lisp-vm", []),
        ("lisp-core", []),
        ("lisp-core", ["--no-default-features", "--features", "vm"]),
    ]
    if "lisp-std" in existing:
        standalone_profiles.append(("lisp-std", []))

    for pkg, args in standalone_profiles:
        if pkg not in existing:
            failures.append(f"missing required standalone package: {pkg}")
            continue
        deps = resolve_tree(pkg, args)
        bad_c = sorted(name for name in deps if is_forbidden_c_dep(name))
        if bad_c:
            label = f"{pkg} {' '.join(args)}".strip()
            failures.append(f"{label} pulled C/build dependency(s): {', '.join(bad_c)}")

    for pkg in ("femacs-host", "femacs-tools", "xtask", "femacs"):
        if pkg not in existing:
            continue
        deps = resolve_tree(pkg)
        bad_batteries = sorted(deps & FORBIDDEN_EDITOR_BATTERIES)
        if bad_batteries:
            failures.append(
                f"editor/tooling package `{pkg}` pulled standalone language deps: "
                f"{', '.join(bad_batteries)}"
            )

    if failures:
        for item in failures:
            print(f"ERROR: {item}", file=sys.stderr)
        return 1

    print(
        "check-crate-boundaries: ok "
        f"(checked standalone={len(standalone_profiles)}, editor isolation verified)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
