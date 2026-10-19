#!/usr/bin/env python3
"""Decode a vector header word the way `src/lisp.h` 924-954 packs it.

The header is a bitfield, and hand-decoding it is how status 4.841 concluded "a zero-length
request" from a `restbytes` that was in fact consistent - the head was 360 bytes, not 248.  The
fields are, from `lisp.h`:

    bit 63          ARRAY_MARK_FLAG
    bit 62          PSEUDOVECTOR_FLAG
    bits 24..31     the `enum pvec_type` code (`PSEUDOVECTOR_AREA_BITS` 24)
    bits 12..23     restsize, in words
    bits 0..11      lispsize, in words

    tools/decode-header.py 4611686018444345344 [...]
    tools/decode-header.py --log run.log        # decode every `push`/`take`/`take2` line
"""
import pathlib
import sys

FLAG = 1 << 62
MARK = 1 << 63


def decode(raw):
    if raw & MARK:
        return "marked"
    if not raw & FLAG:
        return "plain vector, size %d" % raw
    v = raw - FLAG
    code = (v >> 24) & 0xFF
    rest = (v >> 12) & 0xFFF
    lisp = v & 0xFFF
    return ("pvec code %d, restsize %d, lispsize %d -> %d bytes"
            % (code, rest, lisp, 8 + 8 * (rest + lisp)))


def main():
    args = sys.argv[1:]
    if args and args[0] == "--log":
        for line in pathlib.Path(args[1]).read_text().splitlines():
            if not line.startswith("VF "):
                continue
            p = line.split()
            if p[4] in ("push", "take", "take2"):
                print("%-6s addr=%s raw=%s  %s" % (p[4], p[2], p[3], decode(int(p[3]))))
        return 0
    for a in args:
        print("%s  %s" % (a, decode(int(a))))
    return 0


if __name__ == "__main__":
    sys.exit(main())
