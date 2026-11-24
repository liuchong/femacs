#!/usr/bin/env python3
"""Independent C2/A2 standalone Lisp host, http-host, build-lisp, and relocation acceptance gate."""

from __future__ import annotations

import argparse
import hashlib
import http.client
import json
import os
from pathlib import Path
import re
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import threading
import time

REPO = Path(__file__).resolve().parent.parent


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        while True:
            chunk = fh.read(1024 * 1024)
            if not chunk:
                break
            h.update(chunk)
    return h.hexdigest()


def parse_readiness_lines(stdout_text: str) -> tuple[str, int, int]:
    addr: str | None = None
    backend_pid: int | None = None
    extra_lines: list[str] = []
    for raw in stdout_text.splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("LISTENING_ADDR="):
            addr = line.split("=", 1)[1].strip()
        elif line.startswith("BACKEND_PID="):
            backend_pid = int(line.split("=", 1)[1].strip())
        else:
            extra_lines.append(line)
    if extra_lines:
        raise AssertionError(f"http-host stdout polluted with unexpected lines: {extra_lines}")
    if not addr or ":" not in addr:
        raise AssertionError(f"missing LISTENING_ADDR in http-host stdout: {stdout_text!r}")
    if backend_pid is None or backend_pid <= 0:
        raise AssertionError(f"missing valid BACKEND_PID in http-host stdout: {stdout_text!r}")
    host, port_str = addr.rsplit(":", 1)
    return host, int(port_str), backend_pid


def mutate_pdmp_fingerprint(src_pdmp: Path, dst_pdmp: Path) -> None:
    data = bytearray(src_pdmp.read_bytes())
    if len(data) < 64 or data[:4] != b"DUMP":
        raise AssertionError(f"unexpected dump header in {src_pdmp}")
    # Fingerprint lives inside header bytes [16..48]; flip one byte deterministically.
    data[20] ^= 0x5A
    dst_pdmp.write_bytes(bytes(data))


def verify_manifest_relocatability(manifest_path: Path, pkg_root: Path) -> dict:
    data = json.loads(manifest_path.read_text(encoding="utf-8"))
    raw = manifest_path.read_text(encoding="utf-8")
    if "/Users/" in raw or "/home/" in raw:
        raise AssertionError(f"manifest leaks personal host path: {manifest_path}")
    binaries = data.get("binaries", {})
    for name in ("femacs-lisp", "femacs-lisp.pdmp", "http-host"):
        entry = binaries.get(name)
        if not isinstance(entry, dict):
            raise AssertionError(f"manifest missing binary entry `{name}`")
        target = pkg_root / entry["relative_path"]
        if not target.is_file():
            raise AssertionError(f"manifest binary missing on disk: {target}")
        if target.stat().st_size != entry["bytes"]:
            raise AssertionError(f"manifest byte mismatch for {name}")
        if sha256_file(target) != entry["sha256"]:
            raise AssertionError(f"manifest sha256 mismatch for {name}")
    for root, dirs, files in os.walk(pkg_root):
        root_p = Path(root)
        for name in (*dirs, *files):
            p = root_p / name
            if p.is_symlink():
                raise AssertionError(f"packaged runtime contains unexpected symlink: {p}")
    return data


def run_selftest() -> int:
    host, port, pid = parse_readiness_lines("LISTENING_ADDR=127.0.0.1:43210\nBACKEND_PID=98765\n")
    assert (host, port, pid) == ("127.0.0.1", 43210, 98765)
    try:
        parse_readiness_lines("noise line\nLISTENING_ADDR=127.0.0.1:80\nBACKEND_PID=1\n")
        raise AssertionError("expected noisy stdout rejection")
    except AssertionError:
        pass

    with tempfile.TemporaryDirectory(prefix="femacs-c2-selftest-") as tmp:
        root = Path(tmp)
        bin_dir = root / "bin"
        bin_dir.mkdir()
        pdmp = bin_dir / "femacs-lisp.pdmp"
        pdmp.write_bytes(b"DUMP\x00\x00\x00\x00" + b"01234567" * 8)
        lisp_bin = bin_dir / "femacs-lisp"
        http_bin = bin_dir / "http-host"
        lisp_bin.write_bytes(b"bin")
        http_bin.write_bytes(b"http")
        mutated = root / "mutated.pdmp"
        mutate_pdmp_fingerprint(pdmp, mutated)
        assert mutated.read_bytes() != pdmp.read_bytes()
        assert mutated.stat().st_size == pdmp.stat().st_size

        manifest = root / "manifest.json"
        bins = {}
        for p in (lisp_bin, pdmp, http_bin):
            bins[p.name] = {
                "relative_path": f"bin/{p.name}",
                "bytes": p.stat().st_size,
                "sha256": sha256_file(p),
            }
        manifest.write_text(json.dumps({"schema": 1, "binaries": bins}), encoding="utf-8")
        verify_manifest_relocatability(manifest, root)
        (root / "bad-link").symlink_to(lisp_bin)
        try:
            verify_manifest_relocatability(manifest, root)
            raise AssertionError("expected symlink detection to fail")
        except AssertionError:
            pass
    print("test-c2-a2-lisp-app --selftest PASS")
    return 0


def run_cmd(
    argv: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    input_text: str | None = None,
    timeout: int = 120,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=str(cwd) if cwd else None,
        env=env,
        input=input_text,
        capture_output=True,
        text=True,
        timeout=timeout,
    )


def verify_relocated_hygiene(pkg: Path, hidden_forbidden_prefixes: list[str]) -> None:
    for name in ("femacs-lisp", "http-host"):
        bin_path = pkg / "bin" / name
        res = run_cmd(["codesign", "--verify", "--strict", str(bin_path)])
        if res.returncode != 0:
            raise AssertionError(f"codesign --verify --strict failed for {bin_path}: {res.stderr}")
        ot = run_cmd(["otool", "-L", str(bin_path)])
        if ot.returncode != 0:
            raise AssertionError(f"otool -L failed for {bin_path}: {ot.stderr}")
        for line in ot.stdout.splitlines()[1:]:
            dep = line.strip().split(" ", 1)[0]
            if dep.startswith(("/opt/homebrew/", "/usr/local/")):
                raise AssertionError(f"{name} depends on non-relocatable host path {dep}")
            for bad in hidden_forbidden_prefixes:
                if bad and bad in dep:
                    raise AssertionError(f"{name} depends on build checkout path {dep}")
            if name == "http-host" and any(
                tag in dep for tag in ("libgnutls", "libtree-sitter", "liblcms2", "libsqlite3", "femacs")
            ):
                raise AssertionError(f"http-host must not link VM/native libs, found {dep}")


def http_request(
    host: str,
    port: int,
    method: str,
    path: str,
    body: bytes | None = None,
    headers: dict[str, str] | None = None,
    timeout: float = 15.0,
) -> tuple[int, dict[str, str], bytes]:
    conn = http.client.HTTPConnection(host, port, timeout=timeout)
    try:
        conn.request(method, path, body=body, headers=headers or {})
        resp = conn.getresponse()
        data = resp.read()
        hdrs = {k.lower(): v for k, v in resp.getheaders()}
        return resp.status, hdrs, data
    finally:
        conn.close()


def wait_for_readiness(proc: subprocess.Popen[str], timeout_sec: float = 20.0) -> tuple[str, int, int]:
    deadline = time.monotonic() + timeout_sec
    collected: list[str] = []
    assert proc.stdout is not None
    while len(collected) < 2 and time.monotonic() < deadline:
        line = proc.stdout.readline()
        if line == "":
            if proc.poll() is not None:
                break
            time.sleep(0.02)
            continue
        if line.strip():
            collected.append(line)
    return parse_readiness_lines("".join(collected))


def pid_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def assert_port_reusable(host: str, port: int) -> None:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        sock.bind((host, port))
        sock.listen(1)


def exercise_relocated_package(
    pkg: Path,
    app_src: Path,
    extra_terminal_dump: Path | None,
    forbidden_prefixes: list[str],
) -> None:
    empty_home = app_src.parent / "empty-home"
    empty_home.mkdir(exist_ok=True)
    clean_env = {
        "PATH": "/usr/bin:/bin",
        "HOME": str(empty_home),
        "LC_ALL": "C",
    }

    verify_relocated_hygiene(pkg, forbidden_prefixes)

    lisp_bin = pkg / "bin" / "femacs-lisp"
    pdmp_path = pkg / "bin" / "femacs-lisp.pdmp"
    http_bin = pkg / "bin" / "http-host"
    assert lisp_bin.is_file() and pdmp_path.is_file() and http_bin.is_file()

    # Check relocated runtime directories point strictly inside pkg.
    pkg_real = str(pkg.resolve())
    r = run_cmd(
        [
            str(lisp_bin),
            "--batch",
            "--eval",
            '(princ (json-serialize (list (cons (quote data) (expand-file-name data-directory)) '
            '(cons (quote exec) (expand-file-name exec-directory)) '
            '(cons (quote load0) (expand-file-name (car load-path))))))',
        ],
        env=clean_env,
    )
    if r.returncode != 0:
        raise AssertionError(f"relocated path probe failed: {r.stderr}\nstdout={r.stdout}")
    paths_json = json.loads(r.stdout.strip())
    for key in ("data", "exec", "load0"):
        if not paths_json[key].startswith(pkg_real):
            raise AssertionError(f"relocated `{key}` leaked outside {pkg_real}: {paths_json[key]}")

    # Check dump corruption & fingerprint mismatch rejection.
    bad_dir = app_src.parent / "bad-dumps"
    bad_dir.mkdir(exist_ok=True)
    corrupt_pdmp = bad_dir / "corrupt.pdmp"
    corrupt_pdmp.write_bytes(b"NOT_A_VALID_PDMP_HEADER")
    r_corrupt = run_cmd(
        [str(lisp_bin), "--dump-file", str(corrupt_pdmp), "--eval", "(+ 1 2)"],
        env=clean_env,
    )
    if r_corrupt.returncode == 0:
        raise AssertionError("corrupt.pdmp should have failed to load")

    mutated_pdmp = bad_dir / "mutated-fingerprint.pdmp"
    mutate_pdmp_fingerprint(pdmp_path, mutated_pdmp)
    r_mutated = run_cmd(
        [str(lisp_bin), "--dump-file", str(mutated_pdmp), "--eval", "(+ 1 2)"],
        env=clean_env,
    )
    if r_mutated.returncode == 0:
        raise AssertionError("mutated-fingerprint.pdmp should have failed fingerprint check")

    if extra_terminal_dump is not None and extra_terminal_dump.is_file():
        r_wrong = run_cmd(
            [str(lisp_bin), "--dump-file", str(extra_terminal_dump), "--eval", "(+ 1 2)"],
            env=clean_env,
        )
        if r_wrong.returncode == 0:
            raise AssertionError(f"wrong dump {extra_terminal_dump} should have failed")

    # Cold --no-dump + dumped buffer/GC + error + invalid CLI + kill-emacs exit codes.
    buf_form = (
        '(let ((b (get-buffer-create "*c2-reloc*"))) '
        '(with-current-buffer b (insert "ok") (garbage-collect) '
        '(prog1 (buffer-string) (kill-buffer b))))'
    )
    r_cold = run_cmd([str(lisp_bin), "--no-dump", "--eval", buf_form], env=clean_env, timeout=180)
    if r_cold.returncode != 0 or r_cold.stdout.strip() != '"ok"':
        raise AssertionError(f"--no-dump buffer/GC check failed: rc={r_cold.returncode}, out={r_cold.stdout!r}, err={r_cold.stderr!r}")

    r_dump = run_cmd([str(lisp_bin), "--eval", buf_form], env=clean_env)
    if r_dump.returncode != 0 or r_dump.stdout.strip() != '"ok"':
        raise AssertionError(f"dumped buffer/GC check failed: rc={r_dump.returncode}, out={r_dump.stdout!r}, err={r_dump.stderr!r}")

    r_err = run_cmd([str(lisp_bin), "--eval", '(error "c2-expected-boom")'], env=clean_env)
    if r_err.returncode != 1 or not r_err.stderr.strip():
        raise AssertionError(f"error exit expected 1 with stderr: rc={r_err.returncode}, err={r_err.stderr!r}")

    r_usage = run_cmd([str(lisp_bin), "--no-such-flag-c2"], env=clean_env)
    if r_usage.returncode != 2:
        raise AssertionError(f"usage exit expected 2: rc={r_usage.returncode}, err={r_usage.stderr!r}")

    r_kill7 = run_cmd([str(lisp_bin), "--eval", "(kill-emacs 7)"], env=clean_env)
    if r_kill7.returncode != 7:
        raise AssertionError(f"(kill-emacs 7) expected exit 7, got {r_kill7.returncode}: err={r_kill7.stderr!r}")

    r_kill_hook = run_cmd(
        [
            str(lisp_bin),
            "--eval",
            '(progn (setq kill-emacs-hook (list (lambda () (princ "hook-ok\\n")))) (ignore-errors (kill-emacs 9)))',
        ],
        env=clean_env,
    )
    if r_kill_hook.returncode != 9 or "hook-ok" not in r_kill_hook.stdout:
        raise AssertionError(f"kill-emacs hook/ignore-errors failed: rc={r_kill_hook.returncode}, out={r_kill_hook.stdout!r}")

    # Batch --script argument forwarding check.
    probe_script = app_src.parent / "script-probe.el"
    probe_script.write_text(
        '(princ (format "args=%S\\n" command-line-args-left))\n',
        encoding="utf-8",
    )
    r_script = run_cmd(
        [str(lisp_bin), "--batch", "-Q", "--script", str(probe_script), "--", "alpha", "beta"],
        env=clean_env,
    )
    if r_script.returncode != 0 or 'args=("alpha" "beta")' not in r_script.stdout:
        raise AssertionError(f"--script arg forwarding failed: rc={r_script.returncode}, out={r_script.stdout!r}, err={r_script.stderr!r}")

    # REPL recovery across Lisp error.
    r_repl = run_cmd(
        [str(lisp_bin), "--repl"],
        env=clean_env,
        input_text='(+ 1 2)\n(error "repl-boom")\n(* 6 7)\n',
    )
    if r_repl.returncode != 0 or "3\n42\n" not in r_repl.stdout or "repl-boom" not in r_repl.stderr:
        raise AssertionError(f"REPL recovery failed: rc={r_repl.returncode}, out={r_repl.stdout!r}, err={r_repl.stderr!r}")

    # Offline static site generator using shared items-core.el.
    site_out = app_src.parent / "site-out"
    r_site = run_cmd(
        [str(lisp_bin), "--script", str(app_src / "build-site.el"), "--", "--out", str(site_out)],
        env=clean_env,
    )
    if r_site.returncode != 0:
        raise AssertionError(f"build-site.el failed: {r_site.stderr}")
    static_html = (site_out / "index.html").read_bytes()
    static_json = json.loads((site_out / "items.json").read_text(encoding="utf-8"))
    assert static_json["ok"] is True and static_json["count"] == 3

    # Live HTTP host end-to-end checks.
    stderr_log = app_src.parent / "http-host.stderr.log"
    with stderr_log.open("w+", encoding="utf-8") as err_fh:
        proc = subprocess.Popen(
            [
                str(http_bin),
                "--listen",
                "127.0.0.1:0",
                "--handler",
                str(app_src / "http-app.el"),
                "--max-body-bytes",
                "128",
                "--max-queue",
                "8",
                "--timeout-ms",
                "10000",
            ],
            env=clean_env,
            stdout=subprocess.PIPE,
            stderr=err_fh,
            text=True,
        )
        try:
            host, port, backend_pid = wait_for_readiness(proc)
            assert pid_alive(backend_pid), "backend femacs-lisp pid is not alive"

            st_html, hdr_html, body_html = http_request(host, port, "GET", "/")
            assert st_html == 200 and "text/html" in hdr_html.get("content-type", ""), f"GET / bad: {st_html}"
            assert body_html == static_html, "live GET / differs from offline static index.html"

            st_items, hdr_items, body_items = http_request(host, port, "GET", "/api/items")
            assert st_items == 200 and "application/json" in hdr_items.get("content-type", "")
            assert json.loads(body_items.decode("utf-8")) == static_json, "live /api/items differs from static items.json"

            echo_payload = b'{"msg":"hello femacs"}'
            st_echo, _, body_echo = http_request(
                host, port, "POST", "/api/echo?mode=reloc", body=echo_payload, headers={"Content-Type": "application/json"}
            )
            assert st_echo == 200
            echoed = json.loads(body_echo.decode("utf-8"))
            assert echoed["method"] == "POST" and echoed["query"] == "mode=reloc" and echoed["bytes"] == len(echo_payload)

            st_404, _, _ = http_request(host, port, "GET", "/missing-route")
            assert st_404 == 404, f"expected 404, got {st_404}"

            st_413, _, _ = http_request(host, port, "POST", "/api/echo", body=b"x" * 1024)
            assert st_413 == 413, f"expected 413, got {st_413}"

            st_500, _, _ = http_request(host, port, "GET", "/api/boom-with-logs")
            assert st_500 == 500, f"expected 500, got {st_500}"
            time.sleep(0.05)
            err_fh.flush()
            stderr_text = stderr_log.read_text(encoding="utf-8")
            for expected_log in (
                "femacs-web log line 1",
                "femacs-web log line 2",
                "femacs-web diagnostic stderr log before intentional error",
            ):
                if expected_log not in stderr_text:
                    raise AssertionError(f"missing expected log line `{expected_log}` in http-host stderr:\n{stderr_text}")

            # Ensure same healthy backend PID served next request cleanly after 500 error.
            assert pid_alive(backend_pid), "backend process died after /api/boom-with-logs"
            st_after, _, body_after = http_request(host, port, "GET", "/api/items")
            assert st_after == 200 and json.loads(body_after.decode("utf-8")) == static_json

            # Ensure inherited RPC socketpair fd has FD_CLOEXEC across child /bin/sh spawn.
            st_clo, _, body_clo = http_request(host, port, "GET", "/api/check-cloexec")
            assert st_clo == 200
            clo = json.loads(body_clo.decode("utf-8"))
            assert clo["clean"] is True and clo["leakedFds"] == "", f"RPC fd leaked into child sh: {clo}"

            proc.send_signal(signal.SIGTERM)
            rc = proc.wait(timeout=10)
            assert rc == 0, f"expected clean SIGTERM exit 0, got {rc}"
            # Verify no orphan backend PID and listening port can be bound again immediately.
            for _ in range(40):
                if not pid_alive(backend_pid):
                    break
                time.sleep(0.05)
            assert not pid_alive(backend_pid), f"backend pid {backend_pid} survived http-host shutdown"
            assert_port_reusable(host, port)
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait(timeout=5)
            if proc.stdout:
                leftover = proc.stdout.read()
                if leftover.strip():
                    raise AssertionError(f"http-host emitted unexpected late stdout: {leftover!r}")

    exercise_http_lifecycle_edge_cases(http_bin, app_src.parent, clean_env)


def exercise_http_lifecycle_edge_cases(
    http_bin: Path,
    work_dir: Path,
    clean_env: dict[str, str],
) -> None:
    lifecycle_el = work_dir / "lifecycle-app.el"
    lifecycle_el.write_text(
        """;;; lifecycle-app.el --- HTTP timeout/disconnect/SIGTERM test handler -*- lexical-binding: t; -*-
(require 'subr-x)

(defun femacs-http-lifecycle-json (status obj)
  (list (cons 'status status)
        (cons 'headers '(("content-type" . "application/json; charset=utf-8")))
        (cons 'body (json-serialize obj))))

(defun femacs-http-handle (req)
  (let* ((path (or (alist-get 'path req) "/"))
         (query (or (alist-get 'query req) "")))
    (cond
     ((string= path "/api/ping")
      (femacs-http-lifecycle-json 200 (list (cons 'ok t))))
     ((string= path "/api/sleep")
      (let ((ms (if (string-match "ms=\\\\([0-9]+\\\\)" query)
                    (string-to-number (match-string 1 query))
                  100)))
        (sleep-for (/ (float ms) 1000.0))
        (femacs-http-lifecycle-json 200 (list (cons 'sleptMs ms)))))
     ((string= path "/api/hang")
      (while t
        (sleep-for 1)))
     ((string= path "/api/side-effect")
      (let ((tag (if (string-match "tag=\\\\([^&]+\\\\)" query)
                     (match-string 1 query)
                   "default"))
            (marker (getenv "FEMACS_TEST_MARKER_FILE")))
        (when marker
          (write-region (concat tag "\\n") nil marker 'append 'silent))
        (femacs-http-lifecycle-json 200 (list (cons 'tag tag)))))
     (t
      (femacs-http-lifecycle-json 404 (list (cons 'path path)))))))

(provide 'femacs-http-lifecycle-test)
""",
        encoding="utf-8",
    )

    def spawn_req_thread(
        host: str,
        port: int,
        path: str,
        out: dict[str, object],
        timeout: float = 5.0,
    ) -> threading.Thread:
        def _run() -> None:
            try:
                status, _, body = http_request(host, port, "GET", path, timeout=timeout)
                out["status"] = status
                out["body"] = body
            except BaseException as exc:
                out["error"] = repr(exc)

        t = threading.Thread(target=_run, daemon=True)
        t.start()
        return t

    def marker_lines(marker_file: Path) -> list[str]:
        if not marker_file.is_file():
            return []
        return [ln for ln in marker_file.read_text(encoding="utf-8").splitlines() if ln]

    # Scenario B: disconnect eviction + hung 504 + queued eviction + child reap.
    marker_b = work_dir / "lifecycle-b.marker"
    env_b = dict(clean_env)
    env_b["FEMACS_TEST_MARKER_FILE"] = str(marker_b)
    err_b = work_dir / "http-lifecycle-b.stderr.log"
    with err_b.open("w+", encoding="utf-8") as err_fh:
        proc = subprocess.Popen(
            [
                str(http_bin),
                "--listen",
                "127.0.0.1:0",
                "--handler",
                str(lifecycle_el),
                "--max-body-bytes",
                "256",
                "--max-queue",
                "8",
                "--timeout-ms",
                "450",
            ],
            env=env_b,
            stdout=subprocess.PIPE,
            stderr=err_fh,
            text=True,
        )
        try:
            host, port, backend_pid = wait_for_readiness(proc)
            assert pid_alive(backend_pid)
            st_ping, _, _ = http_request(host, port, "GET", "/api/ping", timeout=5.0)
            assert st_ping == 200, f"Scenario B initial ping failed: {st_ping}"
            st_base, _, _ = http_request(host, port, "GET", "/api/side-effect?tag=baseline", timeout=5.0)
            assert st_base == 200
            assert "baseline" in marker_lines(marker_b)

            # Hold backend busy for ~260ms, enqueue a client that disconnects immediately.
            sleep_res: dict[str, object] = {}
            t_sleep = spawn_req_thread(host, port, "/api/sleep?ms=260", sleep_res, timeout=5.0)
            time.sleep(0.05)
            with socket.create_connection((host, port), timeout=2.0) as raw_sock:
                raw_sock.sendall(
                    b"GET /api/side-effect?tag=disconnected HTTP/1.1\r\n"
                    b"Host: 127.0.0.1\r\n"
                    b"Connection: close\r\n\r\n"
                )
                time.sleep(0.04)
                try:
                    raw_sock.shutdown(socket.SHUT_RDWR)
                except OSError:
                    pass
            t_sleep.join(timeout=5.0)
            assert not t_sleep.is_alive(), "Scenario B sleep thread hung"
            assert sleep_res.get("status") == 200, f"Scenario B sleep failed: {sleep_res}"

            st_ping2, _, _ = http_request(host, port, "GET", "/api/ping", timeout=5.0)
            assert st_ping2 == 200 and pid_alive(backend_pid), "backend died during sleep/disconnect test"
            assert "disconnected" not in marker_lines(marker_b), (
                f"disconnected client job was executed: {marker_lines(marker_b)}"
            )

            # Hung request should get 504, reap wedged child, and evict queued side-effect without execution.
            hang_res: dict[str, object] = {}
            queued_res: dict[str, object] = {}
            t_hang = spawn_req_thread(host, port, "/api/hang", hang_res, timeout=5.0)
            time.sleep(0.06)
            t_queued = spawn_req_thread(host, port, "/api/side-effect?tag=queued-behind-hang", queued_res, timeout=5.0)
            t_hang.join(timeout=4.0)
            t_queued.join(timeout=4.0)
            assert not t_hang.is_alive() and not t_queued.is_alive(), (
                f"Scenario B hung/queued threads did not settle: hang={hang_res}, queued={queued_res}"
            )
            assert hang_res.get("status") == 504, f"expected 504 on /api/hang, got {hang_res}"
            assert queued_res.get("status") in (502, 504), (
                f"expected 502/504 for queued request behind hang, got {queued_res}"
            )

            deadline_reap = time.monotonic() + 1.0
            while pid_alive(backend_pid) and time.monotonic() < deadline_reap:
                time.sleep(0.02)
            assert not pid_alive(backend_pid), f"wedged backend pid {backend_pid} survived 504 timeout"
            time.sleep(0.2)
            assert "queued-behind-hang" not in marker_lines(marker_b), (
                f"expired/evicted queued request executed: {marker_lines(marker_b)}"
            )

            proc.send_signal(signal.SIGTERM)
            rc = proc.wait(timeout=3.0)
            assert rc == 0, f"expected clean SIGTERM exit 0 after timeout, got {rc}"
            assert_port_reusable(host, port)
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait(timeout=5)

    # Scenario C: SIGTERM during busy hang with long --timeout-ms 15000 must exit & reap < 3s.
    marker_c = work_dir / "lifecycle-c.marker"
    env_c = dict(clean_env)
    env_c["FEMACS_TEST_MARKER_FILE"] = str(marker_c)
    err_c = work_dir / "http-lifecycle-c.stderr.log"
    with err_c.open("w+", encoding="utf-8") as err_fh:
        proc = subprocess.Popen(
            [
                str(http_bin),
                "--listen",
                "127.0.0.1:0",
                "--handler",
                str(lifecycle_el),
                "--max-body-bytes",
                "256",
                "--max-queue",
                "8",
                "--timeout-ms",
                "15000",
            ],
            env=env_c,
            stdout=subprocess.PIPE,
            stderr=err_fh,
            text=True,
        )
        try:
            host, port, backend_pid = wait_for_readiness(proc)
            assert pid_alive(backend_pid)
            st_ping, _, _ = http_request(host, port, "GET", "/api/ping", timeout=5.0)
            assert st_ping == 200, f"Scenario C initial ping failed: {st_ping}"

            hang_res_c: dict[str, object] = {}
            queued_res_c: dict[str, object] = {}
            t_hang_c = spawn_req_thread(host, port, "/api/hang", hang_res_c, timeout=5.0)
            time.sleep(0.08)
            t_queued_c = spawn_req_thread(
                host, port, "/api/side-effect?tag=queued-at-sigterm", queued_res_c, timeout=5.0
            )
            time.sleep(0.08)

            t0 = time.monotonic()
            proc.send_signal(signal.SIGTERM)
            rc = proc.wait(timeout=3.0)
            elapsed = time.monotonic() - t0
            assert rc == 0, f"expected clean busy SIGTERM exit 0, got {rc} in {elapsed:.3f}s"
            assert elapsed < 3.0, f"busy SIGTERM shutdown exceeded 3s: {elapsed:.3f}s"

            t_hang_c.join(timeout=3.0)
            t_queued_c.join(timeout=3.0)
            assert hang_res_c.get("status") != 200
            assert queued_res_c.get("status") != 200

            deadline_reap = time.monotonic() + 1.0
            while pid_alive(backend_pid) and time.monotonic() < deadline_reap:
                time.sleep(0.02)
            assert not pid_alive(backend_pid), f"wedged backend pid {backend_pid} survived busy SIGTERM"
            time.sleep(0.15)
            assert "queued-at-sigterm" not in marker_lines(marker_c), (
                f"queued request executed during shutdown: {marker_lines(marker_c)}"
            )
            assert_port_reusable(host, port)
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait(timeout=5)


def prepare_export_treesit(export_dir: Path) -> str:
    candidate = os.environ.get("FEMACS_TREE_SITTER_PREFIX")
    if candidate:
        vr = run_cmd([sys.executable, "tools/toolchain-pins.py", "verify", "--prefix", candidate], cwd=REPO)
        if vr.returncode == 0:
            return candidate
    repo_prefix = REPO / "build" / "treesit-0.27.1"
    if repo_prefix.is_dir():
        vr = run_cmd([sys.executable, "tools/toolchain-pins.py", "verify", "--prefix", str(repo_prefix)], cwd=REPO)
        if vr.returncode == 0:
            return str(repo_prefix)
    export_prefix = export_dir / "build" / "treesit-0.27.1"
    ir = run_cmd(
        [sys.executable, "tools/toolchain-pins.py", "install", "--prefix", str(export_prefix)],
        cwd=export_dir,
        timeout=600,
    )
    if ir.returncode != 0:
        raise AssertionError(f"toolchain-pins install in export failed: {ir.stderr}\n{ir.stdout}")
    return str(export_prefix)


def run_acceptance(args: argparse.Namespace) -> int:
    with tempfile.TemporaryDirectory(prefix="femacs-c2-a2-") as tmp_root_str:
        tmp_root = Path(tmp_root_str)
        pkg_copy = tmp_root / "relocated-app-runtime"
        app_src = tmp_root / "example-app"
        shutil.copytree(REPO / "examples" / "lisp-web", app_src, symlinks=False)

        forbidden: list[str] = [str(REPO.resolve())]
        if args.package:
            pkg_src = args.package.resolve()
            manifest_data = verify_manifest_relocatability(pkg_src / "manifest.json", pkg_src)
            assert manifest_data.get("product") == "femacs-lisp"
            shutil.copytree(pkg_src, pkg_copy, symlinks=False)
            exercise_relocated_package(pkg_copy, app_src, args.terminal_dump, forbidden)
            print("test-c2-a2-lisp-app --package smoke PASS")
            return 0

        export_dir = tmp_root / "clean-export"
        hidden_dir = tmp_root / "clean-export-hidden"
        exp = run_cmd(
            [sys.executable, "tools/test-repository-bootstrap.py", "--export", str(export_dir)],
            cwd=REPO,
            timeout=180,
        )
        if exp.returncode != 0:
            raise AssertionError(f"test-repository-bootstrap --export failed:\n{exp.stdout}\n{exp.stderr}")

        ts_prefix = prepare_export_treesit(export_dir)
        build_env = dict(os.environ)
        build_env["FEMACS_TREE_SITTER_PREFIX"] = ts_prefix
        bld = run_cmd(
            ["cargo", "run", "-p", "xtask", "--", "build-lisp", "--out", "dist/lisp"],
            cwd=export_dir,
            env=build_env,
            timeout=1800,
        )
        if bld.returncode != 0:
            raise AssertionError(f"clean-export xtask build-lisp failed:\n{bld.stdout}\n{bld.stderr}")

        pkg_src = export_dir / "dist" / "lisp"
        manifest_data = verify_manifest_relocatability(pkg_src / "manifest.json", pkg_src)
        assert manifest_data.get("product") == "femacs-lisp"
        shutil.copytree(pkg_src, pkg_copy, symlinks=False)
        forbidden.append(str(export_dir.resolve()))

        os.rename(export_dir, hidden_dir)
        try:
            exercise_relocated_package(pkg_copy, app_src, args.terminal_dump, forbidden)
        finally:
            if hidden_dir.exists() and not export_dir.exists():
                os.rename(hidden_dir, export_dir)

    print("test-c2-a2-lisp-app PASS (clean export + build-lisp + isolated relocation + HTTP E2E)")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--selftest", action="store_true", help="Run fast pure-Python unit selftests")
    ap.add_argument("--package", type=Path, help="Verify an already-built dist/lisp package in isolation")
    ap.add_argument(
        "--terminal-dump",
        type=Path,
        default=REPO / "build" / "terminal" / "port" / "src" / "emacs.pdmp",
        help="Optional non-matching editor dump path to confirm fingerprint rejection",
    )
    args = ap.parse_args()
    if args.selftest:
        return run_selftest()
    return run_acceptance(args)


if __name__ == "__main__":
    raise SystemExit(main())
