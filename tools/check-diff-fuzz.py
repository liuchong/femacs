#!/usr/bin/env python3
"""Differential fuzz: one random Lisp program, both programs, byte for byte (docs/port-plan.md 7.2).

The ladder's L5 rung runs the reference's own test files; this rung exists for the defects those
test files cannot see.  `docs/status.md` 4.29x records one: `print_error_message` wrote one byte
too many, and *every* existing gate was blind to it - `extract-abi --check` was OK and
`check-all-units` had 0 failures, while a byte-level comparison of two program outputs caught it
at once.  That is what this tool does: it writes a program neither suite contains, runs it under
both programs with an identical environment, and compares stdout, stderr and the exit status
byte for byte.

The programs are *generated*, not recorded: `--seed` picks a deterministic PRNG, each program is
a sequence of 3-8 steps from a fixed vocabulary of families (`docs/port-plan.md` 7.2's list:
expressions, files, processes, encodings), and every step prints its own value - so a divergence
is a divergence in what a form *returns*, not in what the generator happened to print.  Nothing
in the vocabulary is nondeterministic: no time, no randomness, no hash-table iteration order
(hash keys are sorted before they are printed).  A flaky comparison would be worse than none.

A step that signals is compared as data, not as a crash: `condition-case` prints `ERR:` and the
error, so one bad form does not hide the eight that follow it.

    python3 tools/check-diff-fuzz.py                     # 50 programs from seed 0
    python3 tools/check-diff-fuzz.py --programs 200 --seed 7
    python3 tools/check-diff-fuzz.py --show 3 --dump-dir /tmp/fuzz-diff

Each program runs with its own working directory under `--work-dir`, so file steps cannot see the
repository and cannot collide; the two sides get separate directories with identical *names*.
"""

from __future__ import annotations

import argparse
import random
import shutil
import subprocess
import sys
import threading
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PORT_PROG = REPO / "build/port/src/emacs"
REF_PROG = REPO / "build/emacs-ref/src/emacs"

# -------------------------------------------------------------------------------------------
# the vocabulary.  Every entry must be deterministic on both programs.
# -------------------------------------------------------------------------------------------

WORDS = ["alpha", "Beta", "gamma-DELTA", " x ", "naive", "a-b-c", "", "a;b;c"]
PATTERNS = [r"[a-z]+", r"\([^)]*\)", r"\b\w+\b", r"[^ ]+ [^ ]+", r"x*", r"\S+", r"Delta"]
LISTS = ["'(3 1 2)", "'(1 1 2 3 5 8)", "'()", "'((a . 1) (b . 2))", "'(9 8 7)", "'(a b c a b)"]


def q(s: str) -> str:
    """A Lisp string literal for arbitrary text."""
    out = []
    for ch in s:
        if ch in '"\\':
            out.append("\\" + ch)
        elif ch == "\n":
            out.append("\\n")
        elif ord(ch) < 32 or ord(ch) == 127:
            out.append("\\%o" % ord(ch))
        else:
            out.append(ch)
    return '"' + "".join(out) + '"'


def step_expressions(rng: random.Random) -> list[str]:
    a, b = rng.randint(-99, 99), rng.randint(1, 99)
    word, other = rng.choice(WORDS), rng.choice(WORDS)
    pat = rng.choice(PATTERNS)
    return [
        f"(list (+ {a} {b}) (- {a} {b}) (* {a} {b}) (/ {a} {b}) (% {a} {b}) (ash {a} {rng.randint(0, 8)}))",
        f"(list (truncate {a} {b}) (float {a}) (abs {a}) (max {a} {b}) (min {a} {b}))",
        f"(list (string-to-number {q(str(a))}) (number-to-string {a}) (number-to-string (float {a})))",
        f"(list (concat {q(word)} {q(other)}) (string {q(word)} {a}) (length {q(word)}))",
        f"(list (substring {q(word)} 0 (min {rng.randint(0, 3)} (length {q(word)}))))",
        f"(list (upcase {q(word)}) (downcase {q(word)}) (capitalize {q(word)}))",
        f"(list (split-string {q(word)} \"-\") (string-replace \"a\" \"A\" {q(word)}))",
        f"(list (string-bytes {q(word)}) (length (string-to-multibyte {q(word)})) "
        f"(multibyte-string-p {q(word)}))",
        f"(list (string= {q(word)} {q(other)}) (equal {q(word)} {q(other)}) "
        f"(compare-strings {q(word)} nil nil {q(other)} nil nil))",
        f"(list (format \"%s|%S|%d|%c\" {q(word)} (list 1 2) {a} {ord(rng.choice('azZ'))}))",
        f"(list (prin1-to-string (list {q(word)} {a})) (read (format \"%S\" (list {q(word)} {a}))))",
        f"(list (string-match \"{pat}\" {q(word)}) (match-beginning 0) (match-end 0) "
        f"(match-string 0 {q(word)}))",
        f"(list (string-match-p \"{pat}\" {q(word)}) (replace-regexp-in-string \"{pat}\" \"#\" {q(word)}))",
        f"(let ((s {q(word)})) (list (string-pad s {rng.randint(0, 12)}) (string-trim s) "
        f"(string-trim-left s) (string-trim-right s)))",
        f"(list (sort (copy-sequence {rng.choice(LISTS)}) (lambda (x y) (if (numberp x) (< x y) "
        f"(string< (format \"%s\" x) (format \"%s\" y))))))",
        f"(list (seq-sort (lambda (x y) (> x y)) (list {a} {b} {rng.randint(-50, 50)})))",
        f"(let ((h (make-hash-table :test 'equal))) (puthash {q(word)} {a} h) "
        f"(puthash {q(other)} {b} h) (list (gethash {q(word)} h) (hash-table-count h) "
        f"(sort (mapcar #'car (hash-table->alist h)) #'string<)))",
        f"(mapcar (lambda (x) (* x x)) '({a} {b} {rng.randint(0, 9)}))",
        f"(list (append (list {a}) {rng.choice(LISTS)}) (reverse (list {a} {b})) "
        f"(delete-dups (list {a} {b} {a})))",
        f"(list (assoc 'b '((a . 1) (b . 2))) (member {a} (list 1 {a} 2)) (nth 1 (list {a} {b})))",
        f"(list (aref [1 2 3] {rng.randint(0, 2)}) (vconcat [1 2] [3]) "
        f"(length (make-vector {rng.randint(0, 5)} 0)))",
        f"(list (char-syntax ?{rng.choice('w._() ')} ) (get-char-code-property ?A 'category) "
        f"(get-char-code-property ?A 'name) (char-from-name \"SNOWFLAKE\") "
        f"(string-to-char \"\\N{{SNOWFLAKE}}\"))",
        f"(list (logand {a} {b}) (logior {a} {b}) (logxor {a} {b}) (lognot {a}) (1+ {a}) (1- {a}))",
        f"(list (floor (sqrt {abs(a) + 1})) (expt 2 {rng.randint(0, 8)}) (exp 0) (log 1))",
    ]


def step_text(rng: random.Random) -> list[str]:
    text, other = rng.choice(WORDS), rng.choice(WORDS)
    return [
        "(with-temp-buffer (insert %s) (goto-char (point-min)) (forward-line 1) (insert %s) "
        "(buffer-substring-no-properties (point-min) (point-max)))" % (q(text), q(other)),
        "(with-temp-buffer (insert %s) (goto-char %d) (delete-region (point-min) (point)) "
        "(buffer-string))" % (q(text), rng.randint(0, 6)),
        "(with-temp-buffer (insert %s) (undo) (buffer-string))" % q(text),
        "(with-temp-buffer (insert %s) (put-text-property 0 (min 2 (length %s)) 'face 'bold) "
        "(list (get-text-property 0 'face) (text-property-any (point-min) (point-max) 'face 'bold) "
        "(next-single-property-change 0 'face)))" % (q(text), q(text)),
        "(with-temp-buffer (insert %s) (list (buffer-size) (point) (point-min) (point-max) "
        "(count-lines (point-min) (point-max)) (char-after (point-min))))" % q(text),
        "(with-temp-buffer (insert %s) (goto-char (point-min)) (re-search-forward \"%s\" nil t) "
        "(list (point) (match-beginning 0) (match-end 0)))" % (q(text), rng.choice(PATTERNS)),
        "(with-temp-buffer (insert %s) (goto-char (point-min)) (list (looking-at \"%s\") (point)))"
        % (q(text), rng.choice(PATTERNS)),
        "(with-temp-buffer (insert %s) (let ((m (set-marker (make-marker) 2))) "
        "(list (marker-position m) (buffer-substring (point-min) m))))" % q(text),
        "(with-temp-buffer (insert %s) (indent-relative) (buffer-string))" % q(text),
        "(with-temp-buffer (insert %s) (let ((ov (make-overlay 1 (max 1 (min 3 (point-max)))))) "
        "(list (overlay-start ov) (overlay-end ov) (length (overlays-at 1)))))" % q(text),
        "(with-temp-buffer (insert %s) (goto-char (point-min)) "
        "(list (parse-partial-sexp (point-min) (point-max)) (syntax-ppss)))" % q(text),
    ]


def step_coding(rng: random.Random) -> list[str]:
    text = rng.choice([w for w in WORDS if w])
    coding = rng.choice(["utf-8", "latin-1", "utf-16le", "raw-text", "utf-8-unix"])
    return [
        f"(let ((b (encode-coding-string {q(text)} '{coding}))) "
        f"(list (string-bytes b) (decode-coding-string b '{coding}) (coding-system-p '{coding})))",
        f"(list (base64-encode-string (encode-coding-string {q(text)} 'utf-8)) "
        f"(base64-decode-string (base64-encode-string (encode-coding-string {q(text)} 'utf-8))))",
        f"(list (string-to-list {q(text)}) (apply #'string (string-to-list {q(text)})) "
        f"(length (string-to-list {q(text)})))",
        f"(list (secure-hash 'sha1 {q(text)}) (secure-hash 'md5 {q(text)}) "
        f"(length (secure-hash 'sha256 {q(text)})))",
        f"(list (string-as-unibyte {q(text)}) nil)",
    ]


def step_io(rng: random.Random) -> list[str]:
    text, n = rng.choice(WORDS), rng.randint(1, 5)
    return [
        "(progn (with-temp-file \"f.txt\" (insert %s)) "
        "(list (file-exists-p \"f.txt\") (nth 7 (file-attributes \"f.txt\")) (nth 0 (file-attributes \"f.txt\"))))"
        % q(text),
        "(progn (with-temp-file \"g.txt\" (dotimes (i %d) (insert (format \"line %%d\\n\" i)))) "
        "(with-temp-buffer (insert-file-contents \"g.txt\") (list (buffer-size) (buffer-string))))" % n,
        "(list (file-name-directory \"/tmp/a/b.txt\") (file-name-nondirectory \"/tmp/a/b.txt\") "
        "(file-name-extension \"a.tar.gz\") (expand-file-name \"x\" \"/tmp\"))",
        "(list (directory-file-name \"/tmp/a/\") (file-name-as-directory \"/tmp/a\") "
        "(file-relative-name \"/tmp/a/b\" \"/tmp\"))",
    ]


def step_process(rng: random.Random) -> list[str]:
    word = rng.choice([w for w in WORDS if w and w.isascii() and w.strip()]) or "hello"
    return [
        f"(with-temp-buffer (list (call-process \"/bin/echo\" nil t nil {q(word)}) (buffer-string)))",
        "(with-temp-buffer (list (call-process \"/bin/sh\" nil t nil \"-c\" \"exit 3\") (buffer-string)))",
        "(list (process-lines \"/bin/echo\" \"one\" \"two\") (numberp (string-to-number \"3\")) (zerop 0))",
    ]


FAMILIES = [step_expressions, step_text, step_coding, step_io, step_process]


def make_program(rng: random.Random) -> str:
    steps: list[str] = []
    for _ in range(rng.randint(3, 8)):
        steps.extend(rng.choice(FAMILIES)(rng))
    body = []
    for form in steps:
        body.append(f"  (condition-case err (prin1 {form}) (error (princ \"ERR: \") (prin1 err)))")
        body.append("  (terpri)")
    return ";;; -*- lexical-binding: t -*-\n(progn\n" + "\n".join(body) + "\n  nil)\n"


# -------------------------------------------------------------------------------------------
# the runner
# -------------------------------------------------------------------------------------------


def watch(proc: subprocess.Popen, mib: int, fired: dict) -> None:
    """Kill `proc` once its RSS passes `mib` (the reading check-ert.py also uses)."""
    while proc.poll() is None:
        time.sleep(0.5)
        try:
            out = subprocess.run(["ps", "-o", "rss=", "-p", str(proc.pid)],
                                 capture_output=True, text=True, timeout=5)
            rss = int(out.stdout.strip() or 0)
        except (ValueError, subprocess.SubprocessError):
            continue
        if rss > (mib << 10):
            proc.kill()
            fired["fired"] = True
            return


def run_one(prog: Path, source: Path, cwd: Path, timeout: int,
            memory_mib: int) -> tuple[bytes, bytes, int, bool]:
    proc = subprocess.Popen([str(prog), "-Q", "--batch", "-l", str(source)],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            env={"HOME": "/nonexistent", "PATH": "/usr/bin:/bin", "LC_ALL": "C"},
                            cwd=str(cwd))
    fired = {"fired": False}
    t = (threading.Thread(target=watch, args=(proc, memory_mib, fired), daemon=True)
         if memory_mib else None)
    if t:
        t.start()
    try:
        out, err = proc.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        proc.kill()
        out, err = proc.communicate()
        return out or b"", (err or b"") + b"<timeout>", -1, fired["fired"]
    if t:
        t.join(timeout=5)
    return out or b"", err or b"", proc.returncode, fired["fired"]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--programs", type=int, default=50)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--start-at", type=int, default=0, help="skip the first N programs")
    ap.add_argument("--timeout", type=int, default=30)
    ap.add_argument("--memory-mib", type=int, default=600, help="0 disables the RSS watchdog")
    ap.add_argument("--show", type=int, default=5, help="differing programs to detail")
    ap.add_argument("--dump-dir", type=Path, default=Path("/tmp/femacs-fuzz-diff"))
    ap.add_argument("--port-prog", type=Path, default=PORT_PROG)
    ap.add_argument("--ref-prog", type=Path, default=REF_PROG)
    ap.add_argument("--work-dir", type=Path, default=Path("/tmp/femacs-fuzz-work"))
    a = ap.parse_args()

    for p in (a.port_prog, a.ref_prog):
        if not p.is_file():
            print(f"missing program: {p}", file=sys.stderr)
            return 2
    a.work_dir.mkdir(parents=True, exist_ok=True)
    same = differ = 0
    started = time.time()
    for i in range(a.start_at, a.start_at + a.programs):
        rng = random.Random(f"{a.seed}:{i}")
        src = a.work_dir / f"program-{i}.el"
        src.write_text(make_program(rng), encoding="utf-8")
        dirs = []
        for side in ("port", "ref"):
            d = a.work_dir / f"run-{i}-{side}"
            if d.exists():
                shutil.rmtree(d)
            d.mkdir(parents=True)
            dirs.append(d)
        po, pe, pc, pf = run_one(a.port_prog, src, dirs[0], a.timeout, a.memory_mib)
        ro, re_, rc, rf = run_one(a.ref_prog, src, dirs[1], a.timeout, a.memory_mib)
        if (po, pe, pc) == (ro, re_, rc):
            same += 1
        else:
            differ += 1
            a.dump_dir.mkdir(parents=True, exist_ok=True)
            kept = a.dump_dir / f"diff-{a.seed}-{i}.el"
            shutil.copyfile(src, kept)
            if differ <= a.show:
                print(f"DIFFER #{i} ({kept}): exit port={pc}{' (rss-capped)' if pf else ''} "
                      f"ref={rc}{' (rss-capped)' if rf else ''}")
                for name, x, y in (("stdout", po, ro), ("stderr", pe, re_)):
                    if x != y:
                        print(f"  {name}: port {x[:200]!r}")
                        print(f"  {name}: ref  {y[:200]!r}")
        if (i - a.start_at) % 10 == 9:
            print(f"  … {i - a.start_at + 1}/{a.programs} programs ({time.time() - started:.0f}s)",
                  file=sys.stderr)
    print(f"programs: {a.programs} (seed {a.seed}, from {a.start_at}); same={same}, differ={differ}")
    if differ:
        print(f"the differing programs are kept in {a.dump_dir}")
    return 1 if differ else 0


if __name__ == "__main__":
    sys.exit(main())
