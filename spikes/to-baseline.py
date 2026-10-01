#!/usr/bin/env python3
"""Freeze the C layout report into docs/baseline/layout/.

The baseline must come from the C compiler, never from the Rust mirror: it is the
authority the mirror is checked against (`xtask layout-check`).
"""
import re
import sys
from pathlib import Path

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
rows = {}
for line in src.read_text(encoding="utf-8").splitlines():
    m = re.match(r"^(sizeof|alignof|offsetof)\s+(.+?)\s+(\d+)$", line.strip())
    if m:
        key = f"{m.group(1)} {m.group(2).replace('struct ', '').replace('union ', '')}"
        if key in rows and rows[key] != m.group(3):
            raise SystemExit(f"duplicate key with different values: {key}")
        rows[key] = m.group(3)
dst.parent.mkdir(parents=True, exist_ok=True)
dst.write_text("key\tvalue\n" + "\n".join(f"{k}\t{v}" for k, v in sorted(rows.items())) + "\n",
               encoding="utf-8")
print(f"wrote {dst}: {len(rows)} C layout keys")
