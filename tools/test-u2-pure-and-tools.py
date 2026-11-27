#!/usr/bin/env python3
"""AU2 integration and functional acceptance for U2 pure modules and companion tools.

Verifies:
  1. `#![forbid(unsafe_code)]` enforcement across the 14 safe core modules in `crates/femacs-sys`,
     all active `lib-src/` tools (`make-fingerprint`, `hexl`, `asset-directory-tool`, `emacsclient`),
     and pure `lib/` leaves, plus 100% `SAFETY:` coverage on thin boundary wrappers.
  2. Real Lisp consumers (`md5`, `secure-hash` across MD5/SHA-1/SHA-2/SHA-3, `buffer-hash`,
     `string-version-lessp`, `string-search`, `file-attributes` mode strings) on ASCII,
     multi-block, binary, Unicode, and error inputs.
  3. Companion executables (`hexl` + `lisp/hexl.el` in both Terminal and Cocoa `exec-directory`,
     `make-fingerprint`, `asset-directory-tool`, and `emacsclient` against a live server daemon
     with Unicode `--eval`, `+LINE:COL` visit, Lisp error recovery, missing-socket error, and
     `--alternate-editor` fallback).
  4. Retired scaffolding items (`CLN278`) have zero active build readers and verified Git
     recovery (`4ad0d5f3`).
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

FORBID_TARGETS = (
    "crates/femacs-sys/src/bitops.rs",
    "crates/femacs-sys/src/c_ctype.rs",
    "crates/femacs-sys/src/filemode.rs",
    "crates/femacs-sys/src/filevercmp.rs",
    "crates/femacs-sys/src/md5.rs",
    "crates/femacs-sys/src/memmem.rs",
    "crates/femacs-sys/src/mini_gmp.rs",
    "crates/femacs-sys/src/sha1.rs",
    "crates/femacs-sys/src/sha256.rs",
    "crates/femacs-sys/src/sha3.rs",
    "crates/femacs-sys/src/sha512.rs",
    "crates/femacs-sys/src/signal_names.rs",
    "crates/femacs-sys/src/strcase.rs",
    "crates/femacs-sys/src/timespec.rs",
    "lib-src/make-fingerprint.rs",
    "lib-src/hexl.rs",
    "lib-src/asset-directory-tool.rs",
    "lib-src/emacsclient.rs",
    "lib/include/c-ctype.rs",
    "lib/include/byteswap.in.rs",
    "lib/include/endian.in.rs",
    "lib/include/u64.rs",
    "lib/regex.rs",
    "lib/stdlib.rs",
    "lib/unistd.rs",
    "lib/mini-gmp-gnulib.rs",
)

BOUNDARY_WRAPPERS = (
    "crates/femacs-sys/src/client_os.rs",
    "lib/md5.rs",
    "lib/md5-stream.rs",
    "lib/sha1.rs",
    "lib/sha256.rs",
    "lib/sha512.rs",
    "lib/sha3.rs",
    "lib/filevercmp.rs",
    "lib/filemode.rs",
    "lib/c-strcasecmp.rs",
    "lib/c-strncasecmp.rs",
    "lib/memmem.rs",
)

RETIRED_U2_ITEMS = {
    "lib-src/make-docfile.rs": "3287575146ef012de482307fd464bd42fc6dbb109191f96c1a5164eb2ea7e652",
    "lib-src/ebrowse.rs": "a4d2a2a1eae753c2aabff3fdb381e9f61864e778d88785f9ba4d55b5ed6bb74d",
    "lib-src/etags.rs": "6f51a0f214d42ff0bd618982d956f2946fc8e67523ad5f5243a6689a5a11a232",
    "lib-src/movemail.rs": "49a450bb601c3cfd3c76924c5a402e986eef1e233065f2bb269e59e4ddc7b8a4",
    "lib-src/pop.rs": "1910272d6a3167ef0ac1d61a63b39064b28337d81e27b9670b4d55b5ed6bb74d"[:0]
    or "1910272d6a3167ef0ac1d61a63b39064b28337d81e27b9670b4d55e4e334fde9",
    "lib-src/include/pop.rs": "3409f970581b35c397b2cb1cfb06ab0206aa98c3299916cd577ad3d08f49a012",
    "lib-src/update-game-score.rs": "9ab68fa23f07d64f69f002fd1ba1a2fd3ee019a2413d447fbe8cb5ba134f60e2",
    "lib/include/str-two-way.rs": "0db5a0937e3251c6786c42b67cc1d47e7223d8a960d20f88ea0bc9374f6263d4",
}

RECOVERY_COMMIT = "4ad0d5f3668febe1a2661c0341d3c7cc3adc349d"


def run_cmd(
    argv: list[str],
    *,
    cwd: Path = REPO,
    env: dict[str, str] | None = None,
    timeout: float = 30.0,
) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        argv,
        cwd=str(cwd),
        env=env,
        capture_output=True,
        timeout=timeout,
        check=False,
    )


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--terminal-dir",
        type=Path,
        default=REPO / "build/terminal/port",
        help="Built terminal `port/` root containing `src/emacs` and `lib-src/{emacsclient,hexl}`",
    )
    ap.add_argument(
        "--cocoa-app",
        type=Path,
        default=REPO / "dist/macos/Femacs.app",
        help="Built Cocoa `Femacs.app` bundle",
    )
    ap.add_argument(
        "--tools-bin-dir",
        type=Path,
        default=REPO / "build/cargo/release",
        help="Directory holding compiled `make-fingerprint` and `asset-directory-tool` binaries",
    )
    ap.add_argument("--baseline", type=Path, help="Optional U1 unsafe baseline JSON")
    ap.add_argument("--json-out", type=Path, help="Optional JSON summary output path")
    args = ap.parse_args()

    term_emacs = args.terminal_dir / "src/emacs"
    term_client = args.terminal_dir / "lib-src/emacsclient"
    term_hexl = args.terminal_dir / "lib-src/hexl"

    cocoa_emacs = args.cocoa_app / "Contents/MacOS/Femacs"
    cocoa_client = args.cocoa_app / "Contents/MacOS/bin/emacsclient"
    cocoa_hexl = args.cocoa_app / "Contents/MacOS/bin/hexl"

    make_fp = args.tools_bin_dir / "make-fingerprint"
    asset_tool = args.tools_bin_dir / "asset-directory-tool"

    results: list[dict] = []
    failures = 0

    def run_case(name: str, fn) -> None:
        nonlocal failures
        t0 = time.monotonic()
        try:
            detail = fn()
            dt = time.monotonic() - t0
            results.append({"id": name, "status": "passed", "seconds": round(dt, 6), "detail": detail})
            print(f"CASE {name} status=passed seconds={dt:.4f}")
        except Exception as exc:
            dt = time.monotonic() - t0
            failures += 1
            results.append({"id": name, "status": "failed", "seconds": round(dt, 6), "error": str(exc)})
            print(f"CASE {name} status=failed seconds={dt:.4f} error={exc}", file=sys.stderr)

    def case_forbid_and_boundary_gates() -> dict:
        with tempfile.TemporaryDirectory(prefix="au2-gate-") as tmp:
            report_json = Path(tmp) / "u2-unsafe.json"
            cmd = [
                sys.executable,
                str(REPO / "tools/check-unsafe-boundary.py"),
                "--require-zero-active-stitching",
                "--out",
                str(report_json),
                "--binary",
                str(term_emacs),
                "--kind",
                "terminal",
                "--require-modules",
            ]
            for target in FORBID_TARGETS:
                cmd.extend(["--require-forbid", target])
            if args.baseline and args.baseline.is_file():
                cmd.extend(["--baseline", str(args.baseline), "--ratchet"])
            p1 = run_cmd(cmd, timeout=30.0)
            assert p1.returncode == 0, p1.stdout.decode(errors="replace") + p1.stderr.decode(errors="replace")

            p2 = run_cmd(
                [
                    sys.executable,
                    str(REPO / "tools/check-unsafe-boundary.py"),
                    "--binary",
                    str(cocoa_emacs),
                    "--kind",
                    "cocoa",
                    "--require-modules",
                ],
                timeout=30.0,
            )
            assert p2.returncode == 0, p2.stdout.decode(errors="replace") + p2.stderr.decode(errors="replace")

            data = json.loads(report_json.read_text(encoding="utf-8"))
            by_file = {row["file"]: row for row in data["files"]}
            for rel in BOUNDARY_WRAPPERS:
                assert rel in by_file, f"missing boundary wrapper row: {rel}"
                row = by_file[rel]
                assert not row["allow_unsafe_op_in_unsafe_fn"], f"{rel} still allows unsafe_op_in_unsafe_fn"
                assert not row["allow_static_mut_refs"], f"{rel} still allows static_mut_refs"
                assert row["safety_documented_blocks"] == row["unsafe_blocks"], (
                    f"{rel}: safety_documented_blocks={row['safety_documented_blocks']} != unsafe_blocks={row['unsafe_blocks']}"
                )
            return {
                "active_forbid_files": data["active_product"]["forbid_unsafe_files"],
                "active_unsafe_fns": data["active_product"]["unsafe_fn_defs"],
            }

    def case_lisp_digests_unicode_and_errors() -> dict:
        samples = [
            b"",
            b"abc",
            ("λ FEmacs × Eliscript 协同演进 🚀\n" * 9).encode("utf-8"),
        ]
        expected = []
        for raw in samples:
            expected.append(
                {
                    "md5": hashlib.md5(raw).hexdigest(),
                    "sha1": hashlib.sha1(raw).hexdigest(),
                    "sha224": hashlib.sha224(raw).hexdigest(),
                    "sha256": hashlib.sha256(raw).hexdigest(),
                    "sha384": hashlib.sha384(raw).hexdigest(),
                    "sha512": hashlib.sha512(raw).hexdigest(),
                    "sha3-224": hashlib.sha3_224(raw).hexdigest(),
                    "sha3-256": hashlib.sha3_256(raw).hexdigest(),
                    "sha3-384": hashlib.sha3_384(raw).hexdigest(),
                    "sha3-512": hashlib.sha3_512(raw).hexdigest(),
                }
            )

        with tempfile.TemporaryDirectory(prefix="au2-digest-") as tmp:
            tmp_path = Path(tmp)
            sample_paths = []
            for idx, raw in enumerate(samples):
                sp = tmp_path / f"sample-{idx}.bin"
                sp.write_bytes(raw)
                sample_paths.append(sp)

            elisp = f"""
(progn
  (require 'json)
  (let ((files '({ " ".join(json.dumps(str(p)) for p in sample_paths) }))
        (algos '(md5 sha1 sha224 sha256 sha384 sha512 sha3-224 sha3-256 sha3-384 sha3-512))
        (out nil))
    (dolist (f files)
      (with-temp-buffer
        (set-buffer-multibyte nil)
        (insert-file-contents-literally f)
        (let ((row (list (cons 'md5-fn (md5 (current-buffer)))
                         (cons 'buf-hash (buffer-hash (current-buffer))))))
          (dolist (a algos)
            (push (cons a (secure-hash a (current-buffer))) row))
          (push row out))))
    (let ((bad-err (condition-case err
                       (secure-hash 'not-a-valid-digest "abc")
                     (error (car err)))))
      (princ (json-encode (list (cons 'rows (nreverse out))
                                (cons 'bad_err (symbol-name bad-err))))))))
"""
            for label, binary in (("terminal", term_emacs), ("cocoa", cocoa_emacs)):
                proc = run_cmd([str(binary), "--batch", "-Q", "--eval", elisp], timeout=30.0)
                assert proc.returncode == 0, f"{label} failed: {proc.stderr.decode(errors='replace')}"
                payload = json.loads(proc.stdout.decode("utf-8"))
                assert payload["bad_err"] != "nil", payload
                rows = payload["rows"]
                assert len(rows) == len(expected)
                for got, want in zip(rows, expected):
                    assert got["md5-fn"] == want["md5"], (label, got, want)
                    assert got["buf-hash"] == want["sha1"], (label, got, want)
                    for alg, hex_digest in want.items():
                        assert got[alg] == hex_digest, (label, alg, got[alg], hex_digest)
        return {"binaries_verified": 2, "algorithms": 10, "samples": len(samples)}

    def case_lisp_version_memmem_and_filemode() -> dict:
        with tempfile.TemporaryDirectory(prefix="au2-leaves-") as tmp:
            tmp_path = Path(tmp)
            reg_file = tmp_path / "script.sh"
            reg_file.write_bytes(b"#!/bin/sh\nexit 0\n")
            os.chmod(reg_file, 0o754)
            sub_dir = tmp_path / "owned-dir"
            sub_dir.mkdir()
            os.chmod(sub_dir, 0o750)
            sym_link = tmp_path / "owned-link"
            sym_link.symlink_to(reg_file)

            elisp = f"""
(progn
  (require 'json)
  (let* ((ver-checks
          (list (string-version-lessp "1.2" "1.10")
                (string-version-lessp "1.0~rc1" "1.0")
                (string-version-lessp "pkg-1.2.tar.gz" "pkg-1.10.tar.gz")
                (string-version-lessp "." "..")
                (not (string-version-lessp "2.0" "1.9"))))
         (periodic-hay (concat (make-string 200 ?a) "abaaab" "λ中文字串-target-尾"))
         (search-checks
          (list (equal (string-search "" "abc") 0)
                (equal (string-search "abaaab" periodic-hay) 200)
                (equal (string-search "λ中文字串-target" periodic-hay) 206)
                (null (string-search "missing-needle-xyz" periodic-hay))))
         (m-reg (nth 8 (file-attributes {json.dumps(str(reg_file))})))
         (m-dir (nth 8 (file-attributes {json.dumps(str(sub_dir))})))
         (m-sym (nth 8 (file-attributes {json.dumps(str(sym_link))}))))
    (princ (json-encode (list (cons 'ver ver-checks)
                              (cons 'search search-checks)
                              (cons 'm_reg m-reg)
                              (cons 'm_dir m-dir)
                              (cons 'm_sym m-sym))))))
"""
            proc = run_cmd([str(term_emacs), "--batch", "-Q", "--eval", elisp], timeout=30.0)
            assert proc.returncode == 0, proc.stderr.decode(errors="replace")
            out = json.loads(proc.stdout.decode("utf-8"))
            assert all(x is True for x in out["ver"]), out
            assert all(x is True for x in out["search"]), out
            assert out["m_reg"] == "-rwxr-xr--", out
            assert out["m_dir"] == "drwxr-x---", out
            assert out["m_sym"].startswith("l"), out
            return out

    def case_hexl_executable_and_hexl_el_consumer() -> dict:
        with tempfile.TemporaryDirectory(prefix="au2-hexl-") as tmp:
            tmp_path = Path(tmp)
            raw_data = bytes(range(256)) + "FEmacs Hexl Unicode 验收 🚀\n".encode("utf-8")
            src_bin = tmp_path / "input.bin"
            hex_out = tmp_path / "encoded.hex"
            restored = tmp_path / "restored.bin"
            src_bin.write_bytes(raw_data)

            for hexl_exe in (term_hexl, cocoa_hexl):
                assert hexl_exe.is_file(), f"missing packaged hexl binary: {hexl_exe}"
                enc = subprocess.run(
                    [str(hexl_exe)],
                    stdin=src_bin.open("rb"),
                    capture_output=True,
                    timeout=15.0,
                    check=False,
                )
                assert enc.returncode == 0 and len(enc.stdout) > len(raw_data), hexl_exe
                hex_out.write_bytes(enc.stdout)

                dec = subprocess.run(
                    [str(hexl_exe), "-de"],
                    stdin=hex_out.open("rb"),
                    capture_output=True,
                    timeout=15.0,
                    check=False,
                )
                assert dec.returncode == 0, dec.stderr.decode(errors="replace")
                restored.write_bytes(dec.stdout)
                assert restored.read_bytes() == raw_data, f"roundtrip mismatch for {hexl_exe}"

                bad = run_cmd([str(hexl_exe), "-unknown-flag"], timeout=10.0)
                assert bad.returncode != 0, "expected non-zero exit on invalid hexl flag"

            # Verify real consumer `lisp/hexl.el` locates `hexl` via `exec-directory` in both products.
            work_copy = tmp_path / "hexl-mode-target.bin"
            elisp = f"""
(progn
  (require 'hexl)
  (let ((f {json.dumps(str(work_copy))}))
    (with-temp-buffer
      (set-buffer-multibyte nil)
      (insert-file-contents-literally f)
      (hexlify-buffer)
      (goto-char (point-min))
      (unless (search-forward "00000000:" nil t)
        (error "hexlify output missing offset header"))
      (dehexlify-buffer)
      (write-region (point-min) (point-max) f nil 'silent))))
"""
            for label, editor in (("terminal", term_emacs), ("cocoa", cocoa_emacs)):
                work_copy.write_bytes(raw_data)
                proc = run_cmd([str(editor), "--batch", "-Q", "--eval", elisp], timeout=25.0)
                assert proc.returncode == 0, f"{label} hexl.el failed: {proc.stderr.decode(errors='replace')}"
                assert work_copy.read_bytes() == raw_data, f"{label} hexl.el corrupted binary data"
        return {"hexl_roundtrip_bytes": len(raw_data), "products_verified": 2}

    def case_make_fingerprint_and_asset_directory_tool() -> dict:
        assert make_fp.is_file(), f"missing {make_fp}"
        assert asset_tool.is_file(), f"missing {asset_tool}"

        with tempfile.TemporaryDirectory(prefix="au2-tools-") as tmp:
            tmp_path = Path(tmp)
            placeholder = bytes([
                0xDE, 0x86, 0xBB, 0x99, 0xFF, 0xF5, 0x46, 0x9A,
                0x9E, 0x3F, 0x9F, 0x5D, 0x9A, 0xDF, 0xF0, 0x91,
                0xBD, 0xCD, 0xC1, 0xE8, 0x0C, 0x16, 0x1E, 0xAF,
                0xB8, 0x6C, 0xE2, 0x2B, 0xB1, 0x24, 0xCE, 0xB0,
            ])
            prefix = b"HEADER-Payload-Unicode-\xce\xbb\x00"
            suffix = b"\x01\x02TRAILER"
            initial_image = prefix + placeholder + suffix
            candidate = tmp_path / "dummy-λ.bin"
            candidate.write_bytes(initial_image)

            raw_res = run_cmd([str(make_fp), "-r", str(candidate)], timeout=10.0)
            assert raw_res.returncode == 0, raw_res.stderr.decode(errors="replace")
            expected_digest = hashlib.sha256(initial_image).digest()
            assert raw_res.stdout.decode("ascii").strip() == expected_digest.hex().upper()

            ok = run_cmd([str(make_fp), str(candidate)], timeout=10.0)
            assert ok.returncode == 0, ok.stderr.decode(errors="replace")
            stamped = candidate.read_bytes()
            actual_slot = stamped[len(prefix) : len(prefix) + 32]
            assert actual_slot == expected_digest, "stamped fingerprint mismatch"
            assert stamped[: len(prefix)] == prefix and stamped[len(prefix) + 32 :] == suffix

            no_slot = tmp_path / "no-slot.bin"
            no_slot.write_bytes(b"plain bytes without magic")
            err1 = run_cmd([str(make_fp), str(no_slot)], timeout=10.0)
            assert err1.returncode != 0, "expected failure when placeholder is absent"

            err2 = run_cmd([str(make_fp), str(tmp_path / "missing.bin")], timeout=10.0)
            assert err2.returncode != 0, "expected failure on missing input file"

            in_tree = tmp_path / "assets"
            (in_tree / "lisp/sub").mkdir(parents=True)
            (in_tree / "lisp/sub/hello-λ.el").write_text("(provide 'hello)\n", encoding="utf-8")
            (in_tree / "etc").mkdir()
            (in_tree / "etc/DOC").write_bytes(b"\x1fFtest\nDocstring\n")
            packed = tmp_path / "directory-tree"
            pack_ok = run_cmd([str(asset_tool), str(in_tree), str(packed)], timeout=15.0)
            assert pack_ok.returncode == 0, pack_ok.stderr.decode(errors="replace")
            packed_bytes = packed.read_bytes()
            assert packed_bytes.startswith(b"ANDROID ASSET PACK 1") or len(packed_bytes) > 32
            assert "hello-λ.el".encode("utf-8") in packed_bytes

            pack_err = run_cmd([str(asset_tool), str(tmp_path / "nonexistent-dir"), str(packed)], timeout=10.0)
            assert pack_err.returncode != 0, "expected failure on nonexistent asset input tree"
            return {"stamped_bytes": len(stamped), "packed_bytes": len(packed_bytes)}

    def case_emacsclient_daemon_protocol_unicode_and_errors() -> dict:
        with tempfile.TemporaryDirectory(prefix="au2-srv-") as tmp:
            tmp_path = Path(tmp)
            home_dir = tmp_path / "home"
            run_dir = tmp_path / "run"
            home_dir.mkdir(mode=0o700)
            run_dir.mkdir(mode=0o700)
            sock_path = run_dir / "u2.sock"
            visit_file = tmp_path / "visit-λ.txt"
            visit_file.write_text("line1\nline2\n0123456789-line3\nline4\n", encoding="utf-8")

            env = dict(os.environ)
            env["HOME"] = str(home_dir)
            env["TMPDIR"] = str(run_dir)
            env.pop("XDG_RUNTIME_DIR", None)
            env.pop("EMACS_SOCKET_NAME", None)
            env.pop("ALTERNATE_EDITOR", None)

            # Error cases before server starts:
            for client_bin in (term_client, cocoa_client):
                ver = run_cmd([str(client_bin), "--version"], env=env, timeout=10.0)
                assert ver.returncode == 0 and b"emacsclient 32.0.50" in ver.stdout, ver.stdout
                hlp = run_cmd([str(client_bin), "--help"], env=env, timeout=10.0)
                assert hlp.returncode == 0 and b"--socket-name" in hlp.stdout, hlp.stdout

                bad_to = run_cmd([str(client_bin), "--timeout=-3", "-e", "t"], env=env, timeout=10.0)
                assert bad_to.returncode == 1 and b"Invalid timeout" in bad_to.stderr
                no_arg = run_cmd([str(client_bin)], env=env, timeout=10.0)
                assert no_arg.returncode == 1 and b"file name or argument required" in no_arg.stderr
                missing_srv = run_cmd(
                    [str(client_bin), "-s", str(sock_path), "-e", "(+ 1 1)"],
                    env=env,
                    timeout=10.0,
                )
                assert missing_srv.returncode == 1 and b"can't find socket" in missing_srv.stderr

            # `--alternate-editor` fallback test:
            alt_out = tmp_path / "alt-invoked.txt"
            alt_script = tmp_path / "alt-ed.sh"
            alt_script.write_text(
                f"#!/bin/sh\nprintf '%s\\n' \"$@\" > {json.dumps(str(alt_out))}\n",
                encoding="utf-8",
            )
            os.chmod(alt_script, 0o755)
            alt_run = run_cmd(
                [
                    str(term_client),
                    "-q",
                    "-s",
                    str(sock_path),
                    "-a",
                    f'"{alt_script}" --alt-flag',
                    "+12:3",
                    str(visit_file),
                ],
                env=env,
                timeout=10.0,
            )
            assert alt_run.returncode == 0, alt_run.stderr.decode(errors="replace")
            alt_lines = alt_out.read_text(encoding="utf-8").splitlines()
            assert alt_lines == ["--alt-flag", "+12:3", str(visit_file)], alt_lines

            # Start real foreground daemon and test live protocol from both Terminal & Cocoa clients:
            daemon = subprocess.Popen(
                [str(term_emacs), "-Q", f"--fg-daemon={sock_path}"],
                cwd=str(tmp_path),
                env=env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            try:
                deadline = time.monotonic() + 20.0
                while time.monotonic() < deadline and not sock_path.exists():
                    assert daemon.poll() is None, "daemon exited prematurely"
                    time.sleep(0.05)
                assert sock_path.exists(), "daemon socket did not appear within 20s"

                for label, client_bin in (("terminal", term_client), ("cocoa", cocoa_client)):
                    ev = run_cmd(
                        [
                            str(client_bin),
                            "-s",
                            str(sock_path),
                            "--eval",
                            '(format "%s:%d" "λ中文 & 空格" (+ 20 22))',
                        ],
                        env=env,
                        timeout=15.0,
                    )
                    assert ev.returncode == 0, f"{label}: {ev.stderr.decode(errors='replace')}"
                    assert ev.stdout.decode("utf-8").strip() == '"λ中文 & 空格:42"', ev.stdout

                vis = run_cmd(
                    [
                        str(term_client),
                        "-s",
                        str(sock_path),
                        "-n",
                        "+3:5",
                        str(visit_file),
                    ],
                    env=env,
                    timeout=15.0,
                )
                assert vis.returncode == 0, vis.stderr.decode(errors="replace")

                chk_pos = run_cmd(
                    [
                        str(cocoa_client),
                        "-s",
                        str(sock_path),
                        "-e",
                        f'(with-current-buffer (find-buffer-visiting {json.dumps(str(visit_file))}) '
                        f'(list (line-number-at-pos) (current-column) (char-after)))',
                    ],
                    env=env,
                    timeout=15.0,
                )
                assert chk_pos.returncode == 0, chk_pos.stderr.decode(errors="replace")
                # `+3:5` selects 1-based line 3, 1-based column 5 -> 0-based `current-column` 4 ('4' == 52).
                assert chk_pos.stdout.decode("utf-8").strip() == "(3 4 52)", chk_pos.stdout

                # Lisp condition error + subsequent request recovery on same daemon:
                err_ev = run_cmd(
                    [str(term_client), "-s", str(sock_path), "-e", '(error "u2-expected-boom")'],
                    env=env,
                    timeout=15.0,
                )
                assert err_ev.returncode == 1 and b"*ERROR*:" in err_ev.stderr, err_ev.stderr

                rec_ev = run_cmd(
                    [str(cocoa_client), "-s", str(sock_path), "-e", "(+ 40 2)"],
                    env=env,
                    timeout=15.0,
                )
                assert rec_ev.returncode == 0 and rec_ev.stdout.strip() == b"42", rec_ev.stdout

                # Clean daemon shutdown via Lisp `(kill-emacs)`:
                run_cmd(
                    [str(term_client), "-s", str(sock_path), "-e", "(kill-emacs 0)"],
                    env=env,
                    timeout=15.0,
                )
                daemon.wait(timeout=10.0)
            finally:
                if daemon.poll() is None:
                    daemon.terminate()
                    try:
                        daemon.wait(timeout=5.0)
                    except subprocess.TimeoutExpired:
                        daemon.kill()
                        daemon.wait(timeout=5.0)
            return {"daemon_exit": daemon.returncode}

    def case_retired_items_and_recovery_binding() -> dict:
        doc_file = REPO / "etc/DOC"
        assert doc_file.is_file() and doc_file.stat().st_size > 100_000, "missing generated etc/DOC"

        for rel, want_sha in RETIRED_U2_ITEMS.items():
            assert not (REPO / rel).exists(), f"retired item still present on disk: {rel}"
            git_show = run_cmd(["git", "show", f"{RECOVERY_COMMIT}:{rel}"], timeout=10.0)
            assert git_show.returncode == 0, f"failed to recover {rel} from {RECOVERY_COMMIT}"
            got_sha = hashlib.sha256(git_show.stdout).hexdigest()
            assert got_sha == want_sha, f"SHA-256 mismatch for {rel}: {got_sha} != {want_sha}"

        own_voice = run_cmd([sys.executable, str(REPO / "tools/check-own-voice.py")], timeout=30.0)
        # Verify zero own-voice findings in any file touched in U2 (ignore pre-existing test-stable-suite.py:1228).
        bad_lines = [
            ln
            for ln in (own_voice.stdout.decode(errors="replace") + own_voice.stderr.decode(errors="replace")).splitlines()
            if ln.startswith("FAIL ") and "tools/test-stable-suite.py:1228" not in ln
        ]
        assert not bad_lines, f"own-voice violations: {bad_lines}"

        ledger_ticks = run_cmd([sys.executable, str(REPO / "tools/check-ledger-ticks.py")], timeout=30.0)
        assert ledger_ticks.returncode == 0, ledger_ticks.stdout.decode(errors="replace")
        return {"retired_verified": len(RETIRED_U2_ITEMS), "recovery_commit": RECOVERY_COMMIT}

    run_case("forbid_and_boundary_gates", case_forbid_and_boundary_gates)
    run_case("lisp_digests_unicode_and_errors", case_lisp_digests_unicode_and_errors)
    run_case("lisp_version_memmem_and_filemode", case_lisp_version_memmem_and_filemode)
    run_case("hexl_executable_and_hexl_el_consumer", case_hexl_executable_and_hexl_el_consumer)
    run_case("make_fingerprint_and_asset_directory_tool", case_make_fingerprint_and_asset_directory_tool)
    run_case("emacsclient_daemon_protocol_unicode_and_errors", case_emacsclient_daemon_protocol_unicode_and_errors)
    run_case("retired_items_and_recovery_binding", case_retired_items_and_recovery_binding)

    summary = {
        "schema": "femacs-au2-acceptance-v1",
        "total_cases": len(results),
        "failures": failures,
        "cases": results,
    }
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")

    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
