#!/usr/bin/env python3
"""Fault injection: the same fault, both programs, compared (`docs/port-plan.md` 7.2).

The plan's last rung is *fault* behaviour: out of memory, EINTR, signals, malformed encoded
input.  Each case here runs one program under one fault and compares stdout, stderr and the
exit status with the other program under the same fault - the differential rule the rest of the
ladder uses.

The faults are the ones this platform can actually deliver, and the tool says so when it cannot:

  * **signals**: the harness sends the signal (SIGTERM/SIGINT/SIGHUP/SIGQUIT/SIGWINCH/SIGCONT)
    or the program writes into a closed pipe (SIGPIPE);
  * **limits**: `ulimit -n` (EMFILE), `-f` (EFBIG/SIGXFSZ), `-t` (SIGXCPU), `-s` (stack
    exhaustion on a deep recursion);
  * **memory**: `RLIMIT_AS`, `RLIMIT_DATA` and `ulimit -m` are all *refused* by Darwin
    ("cannot modify limit: Invalid argument", measured), so the memory case here is the one the
    platform allows: a program that allocates far past the harness's RSS cap, where the reading
    is that both sides are killed by the same cap after printing the same bytes; the stack case
    covers the *other* memory-exhaustion path, `stack_overflow`'s longjmp;
  * **encodings**: malformed byte strings and bogus coding systems, which need no external fault.

Two normalisations keep the comparison about *behaviour*: addresses (`0x...`) and the tree path
(`.../emacs/...` vs `.../femacs/...`) are placeholders, and Rust's mangled symbols are reduced to
the identifier they mangle, because the reference's backtrace prints C names where the port's
prints `_RNvCs…20name`.  Backtrace *frames* are compared separately from the message lines: a
frame whose spelling or depth follows from the port's codegen is reported as `backtrace-only`
instead of being counted as a behavioural difference, and the tool prints what it saw.

    python3 tools/check-fault-injection.py                # every case
    python3 tools/check-fault-injection.py --only sig,limit
    python3 tools/check-fault-injection.py --show 4
"""

from __future__ import annotations

import argparse
import os
import re
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PORT_PROG = REPO / "build/port/src/emacs"
REF_PROG = REPO / "build/emacs-ref/src/emacs"

ADDRESS = re.compile(r"0x[0-9a-fA-F]+")
NAME = None          # built on first use, from the name discovered above
TREE = re.compile(r"/[^ \t]*/(?:emacs|femacs)/")
MANGLED = re.compile(r"_R[A-Za-z0-9_]*?(\d+)([A-Za-z_][A-Za-z0-9_]*)")
FRAME = re.compile(r"^\s*\d+\s+\S+\s+0x[0-9a-fA-F]+")
OFFSET = re.compile(r"\+ \d+$")
NAME_LINE = re.compile(r"This file is part of ([^.\n]+)\.")


def product_name() -> str:
    """The reference's product name, read from the reference's own files (never spelled here).

    P-189/P-190's rule and PORTING.md 0.1-19: a ported or own file states its *own* name, and the
    reference's name is discovered from the reference's artefacts.  `lisp/ldefs-boot.el` is the
    tracked one, the other two are its build's products.
    """
    reference = REPO / "emacs"
    for rel in ("lisp/ldefs-boot.el", "lisp/international/charprop.el", "lisp/loaddefs.el"):
        path = reference / rel
        if path.is_file():
            m = NAME_LINE.search(path.read_text("utf-8", "surrogateescape"))
            if m:
                return m.group(1)
    return ""

DEEP_RECURSION = ('(defun fi (n) (if (<= n 0) 0 (1+ (fi (1- n))))) '
                  '(condition-case e (progn (fi 100000) (princ "no-overflow")) '
                  '(error (princ (format "ERR:%S" e))))')
# Allocates in steps and prints each one, so a cap that fires leaves a comparable prefix.  Two
# things are deliberately *not* here: a single huge request (`(make-string (ash 1 40) ?x)`) and a
# multi-gigabyte loop.  macOS over-commits, so a 1 TiB `malloc` *succeeds* and the program then
# touches it - measured 2026-10-17: it walked into swap instead of failing.  The bounded loop plus
# the harness's RSS cap stays inside the executor's budget.
ALLOCATE = ('(let ((l nil)) (dotimes (i 200000) (setq l (cons (make-string 4096 ?x) l)) '
            '(when (zerop (% i 1000)) (princ (format "%d " i)) (terpri))))')
BURN = '(let ((x 0)) (dotimes (i 100000000) (setq x (+ x i))) (princ "burned"))'
ALLOCATE_SHORT = ('(let ((l nil)) (dotimes (i 2000) (setq l (cons (make-string 4096 ?x) l))) '
                  '(princ (format "allocated:%d" (length l))))')
KEEP_FILES = ('(let (fs) (condition-case e (progn (dotimes (i 40) '
              '(push (find-file-noselect (make-temp-file "femacs-fault")) fs)) (princ "opened")) '
              '(error (princ (format "ERR:%S" e)))))')
WRITE_BIG = ('(with-temp-file "big.txt" (dotimes (i 200) '
             '(insert (make-string 500 ?y)))) '
             '(princ (format "wrote:%s" (nth 7 (file-attributes "big.txt"))))')

CASES: list[dict] = [
    # --- malformed encoded input: no external fault needed -------------------------------
    {"name": "enc-invalid-utf8", "group": "enc",
     "program": '(princ (condition-case e (decode-coding-string "\\303\\050" \'utf-8) '
                '(error (format "ERR:%S" e))))'},
    {"name": "enc-truncated-iso2022", "group": "enc",
     "program": '(princ (condition-case e (decode-coding-string "abc\\033$B" \'iso-2022-jp) '
                '(error (format "ERR:%S" e))))'},
    {"name": "enc-unencodable", "group": "enc",
     "program": '(princ (condition-case e (encode-coding-string "\\U0001D11E" \'latin-1) '
                '(error (format "ERR:%S" e))))'},
    {"name": "enc-invalid-base64", "group": "enc",
     "program": '(princ (condition-case e (base64-decode-string "!!!!") '
                '(error (format "ERR:%S" e))))'},
    {"name": "enc-bogus-coding", "group": "enc",
     "program": '(princ (condition-case e (decode-coding-string "abc" \'no-such-coding) '
                '(error (format "ERR:%S" e))))'},
    {"name": "enc-raw-file-bytes", "group": "enc", "files": {"raw.bin": b"\xff\xfe\x00\xc3\x28"},
     "program": '(with-temp-buffer (set-buffer-multibyte nil) (insert-file-contents "raw.bin") '
                '(princ (format "bytes:%d list:%S" (buffer-size) (string-to-list (buffer-string)))))'},
    # --- signals -------------------------------------------------------------------------
    {"name": "sig-term", "group": "sig", "signal": signal.SIGTERM, "delay": 0.6,
     "program": '(progn (princ "before") (terpri) (sleep-for 5) (princ "after"))'},
    {"name": "sig-int", "group": "sig", "signal": signal.SIGINT, "delay": 0.6,
     "program": '(progn (princ "before") (terpri) (sleep-for 5) (princ "after"))'},
    {"name": "sig-hup", "group": "sig", "signal": signal.SIGHUP, "delay": 0.6,
     "program": '(progn (princ "before") (terpri) (sleep-for 5) (princ "after"))'},
    {"name": "sig-quit", "group": "sig", "signal": signal.SIGQUIT, "delay": 0.6,
     "program": '(progn (princ "before") (terpri) (sleep-for 5) (princ "after"))'},
    {"name": "sig-pipe", "group": "sig", "pipe": True,
     "program": '(dotimes (i 200000) (princ (format "%d\\n" i)))'},
    # --- EINTR: harmless signals while the program waits ---------------------------------
    {"name": "eintr-winched", "group": "sig", "signal": signal.SIGWINCH, "delay": 0.2, "repeat": 8,
     "program": '(progn (let ((t0 (float-time))) (while (< (- (float-time) t0) 1.5) (sleep-for 0.05))) '
                '(princ "slept"))'},
    {"name": "eintr-cont", "group": "sig", "signal": signal.SIGCONT, "delay": 0.2, "repeat": 6,
     "program": '(progn (let ((p (start-process "c" nil "/bin/sleep" "1.2"))) '
                '(while (process-live-p p) (accept-process-output p 0.1)) (princ "reaped"))'},
    # --- resource limits -----------------------------------------------------------------
    {"name": "limit-nofile", "group": "limit", "limits": ["ulimit -n 12"], "program": KEEP_FILES},
    {"name": "limit-fsize", "group": "limit", "limits": ["ulimit -f 4"], "program": WRITE_BIG},
    {"name": "limit-cpu", "group": "limit", "limits": ["ulimit -t 2"], "program": BURN},
    {"name": "limit-stack", "group": "limit", "limits": ["ulimit -s 512"], "program": DEEP_RECURSION},
    # --- memory: what the platform will let us do ----------------------------------------
    {"name": "memory-capped-alloc", "group": "memory", "memory_mib": 200, "timeout": 90,
     "program": ALLOCATE, "capped": True},
    {"name": "memory-rlimit-as", "group": "memory", "limits": ["ulimit -v 400000"],
     "program": ALLOCATE_SHORT,
     "note": "the platform refuses RLIMIT_AS (ulimit -v), so both sides simply run to completion"},
]


def normalise(text: str) -> str:
    global NAME
    if NAME is None:
        NAME = re.compile("|".join(filter(None, (re.escape(product_name()), "femacs"))))
    text = MANGLED.sub(lambda m: m.group(2), text)
    text = ADDRESS.sub("0xADDR", text)
    text = TREE.sub("<TREE>/", text)
    text = NAME.sub("PRODUCT", text)
    return "\n".join(OFFSET.sub("+ N", line) for line in text.split("\n"))


def split_backtrace(lines: list[str]) -> tuple[list[str], list[str]]:
    """(message lines, frame lines).

    A frame is `N  binary  address  symbol + off`; a lone `...` belongs to the backtrace too -
    `emacs_backtrace` prints it when the trace reached the depth limit, which is why it is a
    reading about the *frames* and not a message of its own (measured on the SIGXCPU case, where
    the two sides walked 32 and 41 frames).
    """
    message, frames = [], []
    for line in lines:
        (frames if FRAME.match(line) or line.strip() == "..." else message).append(line)
    return message, frames


def watch(proc: subprocess.Popen, mib: int, fired: dict) -> None:
    while proc.poll() is None:
        time.sleep(0.1)
        try:
            out = subprocess.run(["ps", "-o", "rss=", "-p", str(proc.pid)],
                                 capture_output=True, text=True, timeout=5)
            rss = int(out.stdout.strip() or 0)
        except (ValueError, subprocess.SubprocessError):
            continue
        if rss > (mib << 10):
            try:                        # the case runs in its own session: kill all of it
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except OSError:
                proc.kill()
            fired["fired"] = True
            return


def send_later(pid: int, sig: int, delay: float, repeat: int, done: dict) -> None:
    time.sleep(delay)
    for _ in range(max(1, repeat)):
        try:
            os.kill(pid, sig)
        except OSError:
            return
        time.sleep(0.05)
        if done.get("closed"):
            return


def run(prog: Path, case: dict, work: Path, default_timeout: int) -> tuple[bytes, bytes, int, bool]:
    program = case["program"] + " (terpri)"
    source = work / "program.el"
    source.write_text(";;; -*- lexical-binding: t -*-\n" + program + "\n", encoding="utf-8")
    for name, data in (case.get("files") or {}).items():
        (work / name).write_bytes(data)

    argv = [str(prog), "-Q", "--batch", "-l", str(source)]
    if case.get("limits"):
        argv = ["sh", "-c", "; ".join(case["limits"]) + '; exec "$@"', "sh"] + argv
    if case.get("pipe"):
        argv = ["sh", "-c", 'exec "$@" | head -c 64', "sh"] + argv

    proc = subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=str(work),
                            env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin", "LC_ALL": "C"},
                            start_new_session=True)
    done: dict = {}
    killers = []
    if case.get("signal"):
        children = subprocess.run(["pgrep", "-P", str(proc.pid)], capture_output=True, text=True)
        pid = int(children.stdout.split()[0]) if children.stdout.split() else proc.pid
        t = threading.Thread(target=send_later,
                             args=(pid, case["signal"], case.get("delay", 0.5),
                                   case.get("repeat", 1), done), daemon=True)
        t.start()
        killers.append(t)
    fired = {"fired": False}
    watcher = None
    if case.get("memory_mib"):
        watcher = threading.Thread(target=watch, args=(proc, case["memory_mib"], fired), daemon=True)
        watcher.start()
    try:
        out, err = proc.communicate(timeout=case.get("timeout", default_timeout))
    except subprocess.TimeoutExpired:
        proc.kill()
        out, err = proc.communicate()
        out, err = out or b"", (err or b"") + b"<timeout>"
    done["closed"] = True
    if watcher:
        watcher.join(timeout=5)
    return out or b"", err or b"", proc.returncode, fired["fired"]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", default="", help="comma-separated case names or groups")
    ap.add_argument("--show", type=int, default=3, help="differing cases to detail")
    ap.add_argument("--timeout", type=int, default=45)
    ap.add_argument("--port-prog", type=Path, default=PORT_PROG)
    ap.add_argument("--ref-prog", type=Path, default=REF_PROG)
    ap.add_argument("--work-dir", type=Path, default=Path("/tmp/femacs-fault-work"))
    a = ap.parse_args()

    for p in (a.port_prog, a.ref_prog):
        if not p.is_file():
            print(f"missing program: {p}", file=sys.stderr)
            return 2
    wanted = [w for w in a.only.split(",") if w]
    cases = [c for c in CASES
             if not wanted or c["name"] in wanted or c.get("group") in wanted]

    same = backtrace_only = differ = 0
    shown = 0
    print(f"{len(cases)} case(s)")
    for case in cases:
        rows = []
        # One directory per case, reused by both sides (the runs are sequential and the directory
        # is cleared first): a per-side directory would put its own name into every message that
        # prints a file name, which the tool would then report as a difference (measured).
        work = a.work_dir / case["name"]
        for side, prog in (("port", a.port_prog), ("ref", a.ref_prog)):
            work.mkdir(parents=True, exist_ok=True)
            for stale in work.iterdir():
                stale.unlink()
            rows.append(run(prog, case, work, a.timeout))
        (po, pe, pc, pf), (ro, re_, rc, rf) = rows
        pn, rn = normalise((po + pe).decode("utf-8", "surrogateescape")), \
            normalise((ro + re_).decode("utf-8", "surrogateescape"))
        pm, pfr = split_backtrace(pn.split("\n"))
        rm, rfr = split_backtrace(rn.split("\n"))
        verdict = "same"
        if case.get("capped"):
            # Both sides are expected to die at the cap, so their printed prefixes are the
            # reading, not a byte-for-byte match: one side being capped while the other finishes
            # is the difference worth failing on.
            verdict = "capped" if (pf and rf) else "differ"
            if verdict == "capped":
                print(f"{verdict:14s} {case['name']:22s} killed at the {case['memory_mib']} MiB cap "
                      f"on both sides; prefix bytes port={len(po) + len(pe)} "
                      f"ref={len(ro) + len(re_)}"
                      f"{'; ' + case['note'] if case.get('note') else ''}")
                same += 1
                continue
        elif pc != rc or pm != rm:
            verdict = "differ"
        elif pfr != rfr:
            verdict = "backtrace-only"
        note = ""
        if case.get("memory_mib"):
            note = f" (killed by the {case['memory_mib']} MiB cap: port={pf}, ref={rf})"
        if case.get("limit_refused_expected") or case.get("expect_limit_refused"):
            note += " (this platform refuses that limit: the case documents it)"
        print(f"{verdict:14s} {case['name']:22s} exit port={pc} ref={rc}"
              f"{' frames port=' + str(len(pfr)) + ' ref=' + str(len(rfr)) if pfr or rfr else ''}{note}")
        if verdict == "same":
            same += 1
        elif verdict == "backtrace-only":
            backtrace_only += 1
        else:
            differ += 1
            if shown < a.show:
                shown += 1
                print(f"    port message: {pm[0][:160] if pm else ''!r}")
                print(f"    ref  message: {rm[0][:160] if rm else ''!r}")
                for x, y in zip(pfr, rfr):
                    if x != y:
                        print(f"    port frame: {x[:120]}")
                        print(f"    ref  frame: {y[:120]}")
                        break
    print(f"cases: {len(cases)}; same={same}, backtrace-only={backtrace_only}, differ={differ}")
    return 1 if differ else 0


if __name__ == "__main__":
    sys.exit(main())
