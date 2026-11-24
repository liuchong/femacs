#!/usr/bin/env python3
"""Verifiable standalone FEmacs Lisp host cache (`C3`/`A3`).

Validates relocatable `femacs-lisp` packages against their cryptographic
`manifest.json` identity (source commit, target triple, build config, binaries,
resource directory trees, and `package_sha256`), and manages an immutable
content-addressed host cache that rejects missing, wrong-platform,
wrong-commit, wrong-config, or tampered entries.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import sys
import tempfile
import uuid
from pathlib import Path

REQUIRED_BINARIES = ("femacs-lisp", "femacs-lisp.pdmp", "http-host")
CANONICAL_BINARIES = {
    "femacs-lisp": "bin/femacs-lisp",
    "femacs-lisp.pdmp": "bin/femacs-lisp.pdmp",
    "http-host": "bin/http-host",
}
REQUIRED_RESOURCE_TREES = ("lisp", "etc")
SAFE_SEGMENT_RE = re.compile(r"^[A-Za-z0-9._-]+$")
CLEAN_COMMIT_RE = re.compile(r"^[0-9a-f]{40}$")
SHA256_HEX_RE = re.compile(r"^[0-9a-f]{64}$")


class CacheError(RuntimeError):
    """Raised when a package or cache entry fails identity or integrity checks."""

    def __init__(self, reason: str, message: str) -> None:
        super().__init__(message)
        self.reason = reason
        self.message = message


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        while True:
            chunk = fh.read(65536)
            if not chunk:
                break
            h.update(chunk)
    return h.hexdigest()


def ensure_no_symlinks(root: Path) -> None:
    if root.is_symlink():
        raise CacheError("symlink", f"package root must not be a symlink: {root}")
    for walk_root, dirs, files in os.walk(root):
        base = Path(walk_root)
        for name in (*dirs, *files):
            candidate = base / name
            if candidate.is_symlink():
                raise CacheError("symlink", f"package contains forbidden symlink: {candidate}")


def hash_resource_tree(root: Path) -> dict[str, object]:
    if root.is_symlink() or not root.is_dir():
        raise CacheError("missing-resource-tree", f"resource root is not a regular directory: {root}")

    records: list[tuple[str, int, str]] = []
    for walk_root, _dirs, files in os.walk(root):
        walk_path = Path(walk_root)
        if walk_path.is_symlink():
            raise CacheError("symlink", f"resource directory contains symlink: {walk_path}")
        for fname in files:
            fpath = walk_path / fname
            if fpath.is_symlink() or not fpath.is_file():
                raise CacheError("symlink", f"resource tree contains non-regular file: {fpath}")
            rel = fpath.relative_to(root).as_posix()
            size = fpath.stat().st_size
            digest = sha256_file(fpath)
            records.append((rel, size, digest))

    records.sort(key=lambda item: item[0])
    tree_hash = hashlib.sha256()
    total_bytes = 0
    for rel, size, digest in records:
        total_bytes += size
        tree_hash.update(f"{rel}\0{size}\0{digest}\n".encode("utf-8"))

    return {
        "file_count": len(records),
        "sha256": tree_hash.hexdigest(),
        "total_bytes": total_bytes,
    }


def sanitize_segment(label: str, raw: str) -> str:
    if not raw or raw in (".", "..") or not SAFE_SEGMENT_RE.match(raw):
        raise CacheError("invalid-segment", f"invalid {label} segment: {raw!r}")
    return raw


def slugify_build_config(build_config: str) -> str:
    slug = build_config.replace("/", "__")
    return sanitize_segment("build_config", slug)


def validate_clean_commit(label: str, raw: str) -> str:
    if not CLEAN_COMMIT_RE.match(raw):
        raise CacheError(
            "unverified-source-commit",
            f"{label} must be a 40-hex verified clean commit, got {raw!r}",
        )
    return raw


def validate_sha256_hex(label: str, raw: str) -> str:
    if not SHA256_HEX_RE.match(raw):
        raise CacheError(
            "invalid-package-sha256",
            f"{label} must be a 64-hex SHA-256 digest, got {raw!r}",
        )
    return raw


def _package_binary_paths(pkg_dir: Path) -> tuple[str, str]:
    resolved_pkg = pkg_dir.resolve()
    lisp_bin = (resolved_pkg / CANONICAL_BINARIES["femacs-lisp"]).resolve()
    http_bin = (resolved_pkg / CANONICAL_BINARIES["http-host"]).resolve()
    return str(lisp_bin), str(http_bin)


def compute_package_sha256(manifest: dict[str, object]) -> str:
    canonical = {
        "binaries": manifest["binaries"],
        "build_config": manifest["build_config"],
        "resource_trees": manifest["resource_trees"],
        "source_commit": manifest["source_commit"],
        "target": manifest["target"],
    }
    raw = json.dumps(canonical, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return hashlib.sha256(raw).hexdigest()


def verify_package(
    pkg_dir: Path,
    *,
    expect_commit: str | None = None,
    expect_target: str | None = None,
    expect_build_config: str | None = None,
    expect_package_sha256: str | None = None,
) -> dict[str, object]:
    if not pkg_dir.exists() or pkg_dir.is_symlink() or not pkg_dir.is_dir():
        raise CacheError("missing-package", f"package directory missing or invalid: {pkg_dir}")

    ensure_no_symlinks(pkg_dir)

    manifest_path = pkg_dir / "manifest.json"
    if not manifest_path.is_file():
        raise CacheError("missing-manifest", f"manifest.json not found under {pkg_dir}")

    raw_text = manifest_path.read_text(encoding="utf-8")
    if "/Users/" in raw_text or "/home/" in raw_text:
        raise CacheError("host-path-leak", f"manifest leaks personal host path: {manifest_path}")

    try:
        manifest = json.loads(raw_text)
    except json.JSONDecodeError as exc:
        raise CacheError("invalid-manifest", f"manifest JSON decode failed: {exc}") from exc

    if not isinstance(manifest, dict):
        raise CacheError("invalid-manifest", "manifest root must be a JSON object")
    if manifest.get("schema") != 1:
        raise CacheError("invalid-schema", f"unsupported manifest schema: {manifest.get('schema')!r}")
    if manifest.get("product") != "femacs-lisp":
        raise CacheError("invalid-product", f"unexpected product: {manifest.get('product')!r}")

    for field in ("source_commit", "target", "build_config", "package_sha256"):
        val = manifest.get(field)
        if not isinstance(val, str) or not val:
            raise CacheError("missing-identity", f"manifest missing required non-empty `{field}`")

    validate_clean_commit("manifest source_commit", manifest["source_commit"])
    validate_sha256_hex("manifest package_sha256", manifest["package_sha256"])

    if expect_commit is not None:
        validate_clean_commit("expect_commit", expect_commit)
        if manifest["source_commit"] != expect_commit:
            raise CacheError(
                "commit-mismatch",
                f"source_commit mismatch: expected {expect_commit!r}, got {manifest['source_commit']!r}",
            )
    if expect_target is not None and manifest["target"] != expect_target:
        raise CacheError(
            "target-mismatch",
            f"target mismatch: expected {expect_target!r}, got {manifest['target']!r}",
        )
    if expect_build_config is not None and manifest["build_config"] != expect_build_config:
        raise CacheError(
            "build-config-mismatch",
            f"build_config mismatch: expected {expect_build_config!r}, got {manifest['build_config']!r}",
        )
    if expect_package_sha256 is not None:
        validate_sha256_hex("expect_package_sha256", expect_package_sha256)
        if manifest["package_sha256"] != expect_package_sha256:
            raise CacheError(
                "package-sha256-mismatch",
                f"package_sha256 mismatch: expected {expect_package_sha256!r}, got {manifest['package_sha256']!r}",
            )

    resolved_pkg = pkg_dir.resolve()
    binaries = manifest.get("binaries")
    if not isinstance(binaries, dict) or set(binaries.keys()) != set(CANONICAL_BINARIES.keys()):
        raise CacheError("invalid-binaries", "manifest `binaries` must contain exact canonical entries")
    for name, expected_rel in CANONICAL_BINARIES.items():
        entry = binaries.get(name)
        if not isinstance(entry, dict):
            raise CacheError("missing-binary-entry", f"manifest missing binary entry `{name}`")
        rel_path = entry.get("relative_path")
        expected_bytes = entry.get("bytes")
        expected_sha = entry.get("sha256")
        if rel_path != expected_rel:
            raise CacheError(
                "invalid-binary-path",
                f"binary `{name}` must use canonical relative_path {expected_rel!r}, got {rel_path!r}",
            )
        if (
            not isinstance(expected_bytes, int)
            or isinstance(expected_bytes, bool)
            or expected_bytes < 0
            or not isinstance(expected_sha, str)
            or not SHA256_HEX_RE.match(expected_sha)
        ):
            raise CacheError("invalid-binary-entry", f"malformed binary metadata for `{name}`")
        candidate = (resolved_pkg / expected_rel).resolve()
        if resolved_pkg not in candidate.parents or candidate.is_symlink() or not candidate.is_file():
            raise CacheError("missing-binary-file", f"packaged binary missing or invalid: {expected_rel}")
        actual_bytes = candidate.stat().st_size
        if actual_bytes != expected_bytes:
            raise CacheError(
                "binary-bytes-mismatch",
                f"binary `{name}` size mismatch: manifest={expected_bytes}, actual={actual_bytes}",
            )
        actual_sha = sha256_file(candidate)
        if actual_sha != expected_sha:
            raise CacheError(
                "binary-sha256-mismatch",
                f"binary `{name}` sha256 mismatch: manifest={expected_sha}, actual={actual_sha}",
            )

    bin_dir = resolved_pkg / "bin"
    if bin_dir.is_symlink() or not bin_dir.is_dir():
        raise CacheError("missing-binary-file", f"package bin directory missing or invalid: {bin_dir}")
    actual_bin_entries = {p.name for p in bin_dir.iterdir()}
    if actual_bin_entries != set(CANONICAL_BINARIES.keys()):
        raise CacheError(
            "unexpected-bin-entry",
            f"package bin/ entries {sorted(actual_bin_entries)} do not match canonical layout {sorted(CANONICAL_BINARIES)}",
        )

    actual_top_dirs: set[str] = set()
    for entry in resolved_pkg.iterdir():
        if entry.is_symlink():
            raise CacheError("symlink", f"package contains forbidden symlink: {entry}")
        if entry.name == "manifest.json":
            if not entry.is_file():
                raise CacheError("invalid-manifest", f"manifest.json is not a regular file: {entry}")
        elif entry.name == "bin":
            continue
        elif entry.is_dir():
            sanitize_segment("resource_tree", entry.name)
            actual_top_dirs.add(entry.name)
        else:
            raise CacheError(
                "unexpected-package-entry",
                f"unexpected top-level file in package root: {entry.name}",
            )

    resource_trees = manifest.get("resource_trees")
    if not isinstance(resource_trees, dict):
        raise CacheError("missing-resource-trees", "manifest missing `resource_trees` object")
    for req_tree in REQUIRED_RESOURCE_TREES:
        if req_tree not in resource_trees:
            raise CacheError("missing-resource-tree", f"manifest missing required resource tree `{req_tree}`")

    unverified_dirs = sorted(actual_top_dirs - set(resource_trees.keys()))
    if unverified_dirs:
        raise CacheError(
            "unverified-resource-tree",
            f"package contains runtime resource directories omitted from manifest: {unverified_dirs}",
        )
    missing_dirs = sorted(set(resource_trees.keys()) - actual_top_dirs)
    if missing_dirs:
        raise CacheError(
            "missing-resource-tree",
            f"manifest declares resource directories missing from package: {missing_dirs}",
        )

    for dir_name, recorded in sorted(resource_trees.items()):
        if not isinstance(dir_name, str) or not isinstance(recorded, dict):
            raise CacheError("invalid-resource-tree", f"malformed resource tree entry `{dir_name}`")
        sanitize_segment("resource_tree", dir_name)
        actual = hash_resource_tree(resolved_pkg / dir_name)
        if actual != recorded:
            raise CacheError(
                "resource-tree-mismatch",
                f"resource tree `{dir_name}` tampered or drifted: expected={recorded}, actual={actual}",
            )

    recomputed = compute_package_sha256(manifest)
    if recomputed != manifest["package_sha256"]:
        raise CacheError(
            "package-digest-mismatch",
            f"manifest package_sha256 mismatch: recorded={manifest['package_sha256']}, recomputed={recomputed}",
        )

    return manifest


def cache_entry_path(cache_dir: Path, manifest: dict[str, object]) -> Path:
    commit = validate_clean_commit("source_commit", str(manifest["source_commit"]))
    target = sanitize_segment("target", str(manifest["target"]))
    cfg_slug = slugify_build_config(str(manifest["build_config"]))
    full_sha = validate_sha256_hex("package_sha256", str(manifest["package_sha256"]))
    return cache_dir / commit / target / cfg_slug / full_sha


def ensure_cache(
    cache_dir: Path,
    source_package: Path,
    *,
    expect_commit: str | None = None,
    expect_target: str | None = None,
    expect_build_config: str | None = None,
    expect_package_sha256: str | None = None,
) -> dict[str, object]:
    src_manifest = verify_package(
        source_package,
        expect_commit=expect_commit,
        expect_target=expect_target,
        expect_build_config=expect_build_config,
        expect_package_sha256=expect_package_sha256,
    )
    target_dir = cache_entry_path(cache_dir, src_manifest)

    if target_dir.exists() or target_dir.is_symlink():
        cached_manifest = verify_package(
            target_dir,
            expect_commit=str(src_manifest["source_commit"]),
            expect_target=str(src_manifest["target"]),
            expect_build_config=str(src_manifest["build_config"]),
            expect_package_sha256=str(src_manifest["package_sha256"]),
        )
        lisp_bin, http_bin = _package_binary_paths(target_dir)
        return {
            "ok": True,
            "status": "hit",
            "cache_entry": str(target_dir.resolve()),
            "lisp_bin": lisp_bin,
            "http_bin": http_bin,
            "source_commit": cached_manifest["source_commit"],
            "target": cached_manifest["target"],
            "build_config": cached_manifest["build_config"],
            "package_sha256": cached_manifest["package_sha256"],
        }

    target_dir.parent.mkdir(parents=True, exist_ok=True)
    staging_dir = target_dir.parent / f".staging-{os.getpid()}-{uuid.uuid4().hex[:8]}"
    try:
        shutil.copytree(source_package, staging_dir, symlinks=False)
        verify_package(
            staging_dir,
            expect_commit=str(src_manifest["source_commit"]),
            expect_target=str(src_manifest["target"]),
            expect_build_config=str(src_manifest["build_config"]),
            expect_package_sha256=str(src_manifest["package_sha256"]),
        )
        os.replace(staging_dir, target_dir)
    except Exception:
        if staging_dir.exists():
            shutil.rmtree(staging_dir, ignore_errors=True)
        raise

    lisp_bin, http_bin = _package_binary_paths(target_dir)
    return {
        "ok": True,
        "status": "rebuilt",
        "cache_entry": str(target_dir.resolve()),
        "lisp_bin": lisp_bin,
        "http_bin": http_bin,
        "source_commit": src_manifest["source_commit"],
        "target": src_manifest["target"],
        "build_config": src_manifest["build_config"],
        "package_sha256": src_manifest["package_sha256"],
    }


def resolve_cache(
    cache_dir: Path,
    *,
    expect_commit: str,
    expect_target: str,
    expect_build_config: str,
    expect_package_sha256: str,
) -> dict[str, object]:
    commit = validate_clean_commit("expect_commit", expect_commit)
    target = sanitize_segment("target", expect_target)
    cfg_slug = slugify_build_config(expect_build_config)
    full_sha = validate_sha256_hex("expect_package_sha256", expect_package_sha256)
    bucket = cache_dir / commit / target / cfg_slug
    if not bucket.is_dir():
        raise CacheError("cache-miss", f"no cached host bucket for {(commit, target, expect_build_config)} under {cache_dir}")

    entry = bucket / full_sha
    if not entry.exists() and not entry.is_symlink():
        raise CacheError("cache-miss", f"no cached host entry matching package_sha256={full_sha} under {bucket}")
    manifest = verify_package(
        entry,
        expect_commit=commit,
        expect_target=target,
        expect_build_config=expect_build_config,
        expect_package_sha256=full_sha,
    )
    lisp_bin, http_bin = _package_binary_paths(entry)
    return {
        "ok": True,
        "status": "verified",
        "cache_entry": str(entry.resolve()),
        "lisp_bin": lisp_bin,
        "http_bin": http_bin,
        "source_commit": manifest["source_commit"],
        "target": manifest["target"],
        "build_config": manifest["build_config"],
        "package_sha256": manifest["package_sha256"],
    }


def _make_mock_package(root: Path, *, commit: str = "deadbeef" * 5, target: str = "aarch64-apple-darwin") -> dict[str, object]:
    bin_dir = root / "bin"
    lisp_dir = root / "lisp"
    etc_dir = root / "etc"
    bin_dir.mkdir(parents=True, exist_ok=True)
    lisp_dir.mkdir(parents=True, exist_ok=True)
    (etc_dir / "charsets").mkdir(parents=True, exist_ok=True)

    (bin_dir / "femacs-lisp").write_bytes(b"#!/bin/sh\necho femacs-lisp\n")
    (bin_dir / "femacs-lisp").chmod(0o755)
    (bin_dir / "femacs-lisp.pdmp").write_bytes(b"DUMP\x00mock-pdmp")
    (bin_dir / "http-host").write_bytes(b"#!/bin/sh\necho http-host\n")
    (bin_dir / "http-host").chmod(0o755)
    (lisp_dir / "loadup.el").write_text("(provide 'loadup)\n", encoding="utf-8")
    (etc_dir / "charsets" / "ascii.map").write_text("0x41 0x41\n", encoding="utf-8")

    binaries = {}
    for name in REQUIRED_BINARIES:
        p = bin_dir / name
        binaries[name] = {
            "bytes": p.stat().st_size,
            "relative_path": f"bin/{name}",
            "sha256": sha256_file(p),
        }
    resource_trees = {
        "lisp": hash_resource_tree(lisp_dir),
        "etc": hash_resource_tree(etc_dir),
    }
    manifest: dict[str, object] = {
        "schema": 1,
        "product": "femacs-lisp",
        "source_commit": commit,
        "target": target,
        "build_config": "macos-terminal/darwin-arm64/release",
        "directories": ["bin", "lisp", "etc"],
        "binaries": binaries,
        "resource_trees": resource_trees,
    }
    manifest["package_sha256"] = compute_package_sha256(manifest)
    (root / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return manifest


def _assert_cache_fails(reason_sub: str, fn) -> None:
    try:
        fn()
    except CacheError as exc:
        if reason_sub not in exc.reason:
            raise AssertionError(f"expected reason containing {reason_sub!r}, got {exc.reason!r}: {exc.message}") from exc
        return
    raise AssertionError(f"expected CacheError({reason_sub!r}) but succeeded")


def run_selftest() -> int:
    with tempfile.TemporaryDirectory(prefix="femacs-lisp-cache-selftest-") as tmp:
        tmp_path = Path(tmp)
        pkg_src = tmp_path / "src-pkg"
        cache_dir = tmp_path / "cache"
        manifest = _make_mock_package(pkg_src)
        commit = str(manifest["source_commit"])
        target = str(manifest["target"])
        cfg = str(manifest["build_config"])
        pkg_sha = str(manifest["package_sha256"])

        cold = ensure_cache(
            cache_dir,
            pkg_src,
            expect_commit=commit,
            expect_target=target,
            expect_build_config=cfg,
        )
        assert cold["status"] == "rebuilt", f"expected rebuilt, got {cold}"

        warm = ensure_cache(
            cache_dir,
            pkg_src,
            expect_commit=commit,
            expect_target=target,
            expect_build_config=cfg,
        )
        assert warm["status"] == "hit", f"expected hit, got {warm}"
        assert warm["cache_entry"] == cold["cache_entry"]

        resolved = resolve_cache(
            cache_dir,
            expect_commit=commit,
            expect_target=target,
            expect_build_config=cfg,
            expect_package_sha256=pkg_sha,
        )
        assert resolved["status"] == "verified"

        cached_entry = Path(str(resolved["cache_entry"]))
        assert cached_entry.name == pkg_sha, f"expected full 64-hex cache dir, got {cached_entry.name}"
        assert resolved["lisp_bin"] == str((cached_entry / CANONICAL_BINARIES["femacs-lisp"]).resolve())
        assert resolved["http_bin"] == str((cached_entry / CANONICAL_BINARIES["http-host"]).resolve())

        _assert_cache_fails(
            "target-mismatch",
            lambda: verify_package(cached_entry, expect_target="x86_64-unknown-linux-gnu"),
        )
        _assert_cache_fails(
            "commit-mismatch",
            lambda: verify_package(cached_entry, expect_commit="0" * 40),
        )
        _assert_cache_fails(
            "build-config-mismatch",
            lambda: verify_package(cached_entry, expect_build_config="linux/release"),
        )
        _assert_cache_fails(
            "invalid-package-sha256",
            lambda: resolve_cache(
                cache_dir,
                expect_commit=commit,
                expect_target=target,
                expect_build_config=cfg,
                expect_package_sha256=pkg_sha[:16],
            ),
        )

        # Require --expect-package-sha256 on resolve CLI
        try:
            build_parser().parse_args(
                [
                    "resolve",
                    "--cache-dir",
                    str(cache_dir),
                    "--expect-commit",
                    commit,
                    "--expect-target",
                    target,
                    "--expect-build-config",
                    cfg,
                ]
            )
            raise AssertionError("expected resolve CLI parser to require --expect-package-sha256")
        except SystemExit:
            pass

        # Tamper resource file inside lisp/
        lisp_file = cached_entry / "lisp" / "loadup.el"
        orig_lisp = lisp_file.read_text(encoding="utf-8")
        lisp_file.write_text(orig_lisp + "; tampered\n", encoding="utf-8")
        _assert_cache_fails("resource-tree-mismatch", lambda: verify_package(cached_entry))
        lisp_file.write_text(orig_lisp, encoding="utf-8")
        verify_package(cached_entry)

        # Inject unexpected file inside lisp/
        extra_file = cached_entry / "lisp" / "injected.el"
        extra_file.write_text("(message \"extra\")\n", encoding="utf-8")
        _assert_cache_fails("resource-tree-mismatch", lambda: verify_package(cached_entry))
        extra_file.unlink()
        verify_package(cached_entry)

        # Inject unverified runtime-* directory omitted from manifest
        extra_rt = cached_entry / "runtime-extra"
        extra_rt.mkdir()
        (extra_rt / "evil.dylib").write_bytes(b"evil")
        _assert_cache_fails("unverified-resource-tree", lambda: verify_package(cached_entry))
        shutil.rmtree(extra_rt)
        verify_package(cached_entry)

        # Inject unexpected top-level file
        stray_file = cached_entry / "stray.txt"
        stray_file.write_text("stray\n", encoding="utf-8")
        _assert_cache_fails("unexpected-package-entry", lambda: verify_package(cached_entry))
        stray_file.unlink()
        verify_package(cached_entry)

        # Inject unexpected file inside bin/
        extra_bin = cached_entry / "bin" / "extra"
        extra_bin.write_bytes(b"extra")
        _assert_cache_fails("unexpected-bin-entry", lambda: verify_package(cached_entry))
        extra_bin.unlink()
        verify_package(cached_entry)

        # Spoof binary relative_path with matching bytes/sha256 and recomputed package_sha256
        spoof_pkg = tmp_path / "spoof-pkg"
        shutil.copytree(pkg_src, spoof_pkg, symlinks=False)
        spoof_manifest = json.loads((spoof_pkg / "manifest.json").read_text(encoding="utf-8"))
        target_loadup = spoof_pkg / "lisp" / "loadup.el"
        spoof_manifest["binaries"]["femacs-lisp"] = {
            "bytes": target_loadup.stat().st_size,
            "relative_path": "lisp/loadup.el",
            "sha256": sha256_file(target_loadup),
        }
        spoof_manifest["package_sha256"] = compute_package_sha256(spoof_manifest)
        (spoof_pkg / "manifest.json").write_text(json.dumps(spoof_manifest, indent=2) + "\n", encoding="utf-8")
        _assert_cache_fails("invalid-binary-path", lambda: verify_package(spoof_pkg))

        # Reject dirty and unverified source commits
        for bad_commit in ("dirty-" + "a" * 40, "unverified-export"):
            bad_pkg = tmp_path / f"bad-{bad_commit[:10]}"
            shutil.copytree(pkg_src, bad_pkg, symlinks=False)
            bad_manifest = json.loads((bad_pkg / "manifest.json").read_text(encoding="utf-8"))
            bad_manifest["source_commit"] = bad_commit
            bad_manifest["package_sha256"] = compute_package_sha256(bad_manifest)
            (bad_pkg / "manifest.json").write_text(json.dumps(bad_manifest, indent=2) + "\n", encoding="utf-8")
            _assert_cache_fails("unverified-source-commit", lambda: verify_package(bad_pkg))

        # Tamper .pdmp (same byte length to check sha256 verification)
        pdmp_file = cached_entry / "bin" / "femacs-lisp.pdmp"
        orig_pdmp = pdmp_file.read_bytes()
        tampered_pdmp = bytearray(orig_pdmp)
        tampered_pdmp[-1] ^= 0xFF
        pdmp_file.write_bytes(bytes(tampered_pdmp))
        _assert_cache_fails("binary-sha256-mismatch", lambda: verify_package(cached_entry))
        pdmp_file.write_bytes(orig_pdmp)

        # Tamper executable binary
        bin_file = cached_entry / "bin" / "femacs-lisp"
        orig_bin = bin_file.read_bytes()
        bin_file.write_bytes(orig_bin + b"x")
        _assert_cache_fails("binary-bytes-mismatch", lambda: verify_package(cached_entry))
        bin_file.write_bytes(orig_bin)

        # Symlink injection inside cache entry
        sym_file = cached_entry / "lisp" / "evil-link"
        sym_file.symlink_to(cached_entry / "manifest.json")
        _assert_cache_fails("symlink", lambda: verify_package(cached_entry))
        sym_file.unlink()

        # Verify ensure rejects corrupted existing cache entry rather than ignoring it
        pdmp_file.write_bytes(bytes(tampered_pdmp))
        _assert_cache_fails("binary-sha256-mismatch", lambda: ensure_cache(cache_dir, pkg_src))

    print("femacs-lisp-cache selftest PASS (18 checks)")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Manage verified FEmacs Lisp host caches")
    parser.add_argument("--selftest", action="store_true", help="Run built-in integrity self-tests")
    sub = parser.add_subparsers(dest="cmd")

    def add_expect_flags(
        p: argparse.ArgumentParser,
        *,
        required_identity: bool = False,
        require_package_sha256: bool = False,
    ) -> None:
        p.add_argument("--expect-commit", required=required_identity, default=None)
        p.add_argument("--expect-target", required=required_identity, default=None)
        p.add_argument("--expect-build-config", required=required_identity, default=None)
        p.add_argument("--expect-package-sha256", required=require_package_sha256, default=None)
        p.add_argument("--json", action="store_true")

    p_verify = sub.add_parser("verify", help="Verify an unpacked femacs-lisp package")
    p_verify.add_argument("--package", required=True, type=Path)
    add_expect_flags(p_verify)

    p_ensure = sub.add_parser("ensure", help="Populate or reuse a verified cached femacs-lisp package")
    p_ensure.add_argument("--cache-dir", required=True, type=Path)
    p_ensure.add_argument("--source-package", required=True, type=Path)
    add_expect_flags(p_ensure)

    p_resolve = sub.add_parser("resolve", help="Verify and resolve an existing cached femacs-lisp package")
    p_resolve.add_argument("--cache-dir", required=True, type=Path)
    add_expect_flags(p_resolve, required_identity=True, require_package_sha256=True)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if args.selftest:
        return run_selftest()
    if not args.cmd:
        parser.print_help(sys.stderr)
        return 2

    as_json = bool(getattr(args, "json", False))
    try:
        if args.cmd == "verify":
            manifest = verify_package(
                args.package,
                expect_commit=args.expect_commit,
                expect_target=args.expect_target,
                expect_build_config=args.expect_build_config,
                expect_package_sha256=args.expect_package_sha256,
            )
            lisp_bin, http_bin = _package_binary_paths(args.package)
            payload: dict[str, object] = {
                "ok": True,
                "status": "verified",
                "package": str(args.package.resolve()),
                "lisp_bin": lisp_bin,
                "http_bin": http_bin,
                "source_commit": manifest["source_commit"],
                "target": manifest["target"],
                "build_config": manifest["build_config"],
                "package_sha256": manifest["package_sha256"],
            }
        elif args.cmd == "ensure":
            payload = ensure_cache(
                args.cache_dir,
                args.source_package,
                expect_commit=args.expect_commit,
                expect_target=args.expect_target,
                expect_build_config=args.expect_build_config,
                expect_package_sha256=args.expect_package_sha256,
            )
        elif args.cmd == "resolve":
            payload = resolve_cache(
                args.cache_dir,
                expect_commit=args.expect_commit,
                expect_target=args.expect_target,
                expect_build_config=args.expect_build_config,
                expect_package_sha256=args.expect_package_sha256,
            )
        else:
            parser.print_help(sys.stderr)
            return 2
    except CacheError as exc:
        if as_json:
            print(json.dumps({"ok": False, "reason": exc.reason, "error": exc.message}))
        else:
            print(f"femacs-lisp-cache ERROR [{exc.reason}]: {exc.message}", file=sys.stderr)
        return 1

    if as_json:
        print(json.dumps(payload))
    else:
        print(
            f"femacs-lisp-cache {payload['status']}: commit={payload['source_commit']} "
            f"target={payload['target']} sha256={payload['package_sha256']}"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
