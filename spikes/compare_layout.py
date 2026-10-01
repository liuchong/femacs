#!/usr/bin/env python3
"""Compare the C and Rust layout reports.

Keys present in both reports must have identical values: that is the probe's
result.  Keys only in the C report are recorded numbers whose Rust mirror is not
written yet; keys only in the Rust report are wrong (the mirror invented a field).
"""
import sys
from pathlib import Path


def load(path):
    out = {}
    for line in Path(path).read_text(encoding="utf-8").splitlines():
        if not line.strip():
            continue
        parts = line.split()
        if len(parts) >= 3 and parts[0] in ("sizeof", "alignof", "offsetof"):
            key = " ".join(parts[:-1])
            # C writes `sizeof struct Lisp_String`, Rust writes `sizeof Lisp_String`
            key = key.replace("struct ", "").replace("union ", "")
            out[key] = parts[-1]
    return out


c, r = load(sys.argv[1]), load(sys.argv[2])
shared = sorted(set(c) & set(r))
mismatch = [k for k in shared if c[k] != r[k]]
print(f"shared keys: {len(shared)}, mismatches: {len(mismatch)}")
for k in mismatch:
    print(f"  MISMATCH {k}: C {c[k]} vs Rust {r[k]}")
only_c = sorted(set(c) - set(r))
only_r = sorted(set(r) - set(c))
print(f"C-only keys (mirror pending): {len(only_c)}")
for k in only_c:
    print(f"  pending {k} = {c[k]}")
print(f"Rust-only keys (wrong): {len(only_r)}")
for k in only_r:
    print(f"  wrong {k} = {r[k]}")
sys.exit(1 if (mismatch or only_r) else 0)
