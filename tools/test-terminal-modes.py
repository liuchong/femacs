#!/usr/bin/env python3
"""Observe macOS PTY restoration in a persistent owning shell session.

The editor is not the session leader. Compare every terminal setting, retain
the raw PENDIN state difference, and exercise canonical input and echo after
normal exit. Run the actual editor through the caller's resource controller.
"""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import pty
import select
import struct
import subprocess
import sys
import termios
import time

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "tools/fixtures/macos-keys.el"


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(262144), b""):
            h.update(block)
    return h.hexdigest()


def identities(binary: Path) -> dict[str, str]:
    return {str(p): digest(p) for p in
            [binary, binary.parent / "emacs.pdmp",
             binary.parent / "libexec/Emacs.pdmp",
             binary.parent.parent / "Resources/libexec/Femacs.pdmp",
             FIXTURE, Path(__file__).resolve(),
             ROOT / "tools/test-macos-delivery.py"]
            if p.is_file()}


def attributes(value: list) -> list:
    return [*value[:6], [x.hex() if isinstance(x, bytes) else x for x in value[6]]]


def supervise(binary: Path, out: Path, master: int, slave: int) -> int:
    spec = importlib.util.spec_from_file_location("delivery", ROOT / "tools/test-macos-delivery.py")
    delivery = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(delivery)
    before = identities(binary)
    home = out / "home"
    home.mkdir()
    snapshot = out / "snapshot.tsv"
    env = {k: v for k, v in os.environ.items()
           if not k.startswith("FEMACS_") and k not in
           {"EMACSLOADPATH", "EMACSDATA", "EMACSDOC", "EMACSPATH", "EMACSNATIVELOADPATH"}}
    env.update(HOME=str(home), TERM="xterm-256color", LC_ALL="en_US.UTF-8",
               FEMACS_KEY_REPORT=str(snapshot), FEMACS_KEY_POINT="7")
    for key, name in [("XDG_CACHE_HOME", "cache"), ("XDG_CONFIG_HOME", "config"),
                      ("XDG_STATE_HOME", "state"), ("TMPDIR", "tmp")]:
        path = out / name
        path.mkdir()
        env[key] = str(path)
    initial = termios.tcgetattr(slave)
    row = {"id": "normal-terminal-mode-restoration", "passed": False,
           "supervisor_pid": os.getpid(), "supervisor_sid": os.getsid(0),
           "pending_state_bit": termios.PENDIN}
    process = None
    started = time.monotonic()

    def pump(log):
        if select.select([master], [], [], .03)[0]:
            chunk = os.read(master, 16384)
            log.write(chunk)
            for query, answer in [(b"\x1b[6n", b"\x1b[1;1R"),
                                  (b"\x1b[>c", b"\x1b[>0;95;0c"),
                                  (b"\x1b[c", b"\x1b[?1;2c")]:
                if query in chunk:
                    os.write(master, answer)

    try:
        with (out / "screen.ansi").open("xb") as log:
            process = subprocess.Popen([str(binary), "-Q", "-nw", "-l", str(FIXTURE)],
                                       stdin=slave, stdout=slave, stderr=slave,
                                       cwd=out, env=env)
            row.update(editor_pid=process.pid, editor_sid=os.getsid(process.pid),
                       foreground_group=os.tcgetpgrp(slave))
            deadline = time.monotonic() + 12
            while (delivery.snapshot(snapshot) is None and process.poll() is None
                   and time.monotonic() < deadline):
                pump(log)
            if delivery.snapshot(snapshot) is None:
                raise RuntimeError("editor fixture did not become ready")
            active = termios.tcgetattr(slave)
            os.write(master, b"\x1b:(kill-emacs 0)\r")
            deadline = time.monotonic() + 5
            while process.poll() is None and time.monotonic() < deadline:
                pump(log)
            if process.poll() is None:
                raise TimeoutError("normal editor quit did not finish")
            restored = termios.tcgetattr(slave)
            stable_initial = list(initial)
            stable_restored = list(restored)
            stable_initial[3] &= ~termios.PENDIN
            stable_restored[3] &= ~termios.PENDIN
            row.update(exit_code=process.returncode,
                       raw_modes_restored=restored == initial,
                       active_modes_differed=active != initial,
                       initial_attributes=attributes(initial),
                       active_attributes=attributes(active),
                       restored_attributes=attributes(restored),
                       settings_without_pending_equal=stable_initial == stable_restored,
                       differing_fields=[i for i in range(7) if initial[i] != restored[i]])
            # Keep the owning shell alive and test the restored device itself.
            os.write(master, b"fe-mode-")
            partial_ready = bool(select.select([slave], [], [], .08)[0])
            after_partial = termios.tcgetattr(slave)
            os.write(master, b"check\n")
            ready = bool(select.select([slave], [], [], 2)[0])
            line = os.read(slave, 4096) if ready else b""
            deadline = time.monotonic() + .1
            while time.monotonic() < deadline:
                pump(log)
            post = termios.tcgetattr(slave)
            log.flush()
            row.update(partial_line_readable=partial_ready,
                       canonical_line_hex=line.hex(),
                       functional_canonical_input=not partial_ready and line == b"fe-mode-check\n",
                       after_partial_attributes=attributes(after_partial),
                       after_input_attributes=attributes(post),
                       after_input_modes_restored=post == initial,
                       echo_observed=b"fe-mode-check" in (out / "screen.ansi").read_bytes())
            row["passed"] = all([process.returncode == 0,
                                 row["active_modes_differed"],
                                 row["settings_without_pending_equal"],
                                 row["functional_canonical_input"],
                                 row["after_input_modes_restored"],
                                 row["echo_observed"]])
    except Exception as error:
        row["error"] = {"type": type(error).__name__, "message": str(error)}
    finally:
        if process is not None and process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=3)
        row.update(seconds=time.monotonic() - started, identities_before=before,
                   identities_after=identities(binary))
        row["passed"] = row["passed"] and row["identities_before"] == row["identities_after"]
        (out / "mode-result.json").write_text(json.dumps(row, indent=2) + "\n")
    return 0 if row["passed"] else 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--supervisor", nargs=2, type=int, help=argparse.SUPPRESS)
    args = parser.parse_args()
    if sys.platform != "darwin":
        parser.error("this acceptance observes macOS termios state")
    binary = args.binary.resolve(strict=True)
    out = args.out.resolve()
    if args.supervisor:
        return supervise(binary, out, *args.supervisor)
    out.mkdir(parents=True)
    master, slave = pty.openpty()
    process = None
    try:
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))

        def attach():
            os.setsid()
            fcntl.ioctl(slave, termios.TIOCSCTTY, 0)

        with (out / "supervisor.log").open("xb") as log:
            process = subprocess.Popen([sys.executable, str(Path(__file__).resolve()),
                                        "--binary", str(binary), "--out", str(out),
                                        "--supervisor", str(master), str(slave)],
                                       pass_fds=(master, slave), preexec_fn=attach,
                                       stdout=log, stderr=subprocess.STDOUT, cwd=out)
            code = process.wait(timeout=25)
    finally:
        if process is not None and process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=3)
        os.close(master)
        os.close(slave)
    row = json.loads((out / "mode-result.json").read_text())
    print(json.dumps({"passed": row["passed"], "exit_code": row.get("exit_code"),
                      "raw_modes_restored": row.get("raw_modes_restored"),
                      "after_input_modes_restored": row.get("after_input_modes_restored"),
                      "seconds": row["seconds"]}))
    return code


if __name__ == "__main__":
    raise SystemExit(main())
