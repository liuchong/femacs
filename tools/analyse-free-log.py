#!/usr/bin/env python3
"""Cross-check `tools/add-vector-free-probe.py`'s log for the allocator's own invariants.

Three invariants, each a line of output when it breaks:

  * a `take`/`take2` whose header is 0 while the entry was linked with a size - the vector was
    zeroed while it sat on the list;
  * an `al-clear` whose `[p, p + nbytes)` covers a vector that is still linked - the caller is
    zeroing a free object (the mechanism the 4.836-4.842 rounds were chasing);
  * a `split-push` whose `restbytes` is outside `0 .. VECTOR_BLOCK_BYTES` - the bin's head did not
    match its bin.

    tools/analyse-free-log.py run.log
"""
import pathlib
import sys

FLAG = 1 << 62
VECTOR_BLOCK_BYTES = 4088


def size_of(raw):
    if raw <= 0 or not (raw & FLAG):
        return None
    v = raw - FLAG
    return 8 + 8 * (((v >> 12) & 0xFFF) + (v & 0xFFF))


def main():
    if len(sys.argv) < 2:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    events = [(int(p[2]), int(p[3]), p[4])
              for p in (l.split() for l in pathlib.Path(sys.argv[1]).read_text().splitlines()
                        if l.startswith("VF "))]
    linked = {}
    findings = 0
    for i, (a, b, tag) in enumerate(events):
        if tag == "push":
            linked[a] = size_of(b)
        elif tag in ("take", "take2"):
            want = linked.pop(a, None)
            if want and size_of(b) == 0:
                findings += 1
                print("event %d: take of %d reads 0, linked with %d bytes" % (i, a, want))
        elif tag == "al-clear":
            p, n = a, b
            for addr, size in list(linked.items()):
                if size and p < addr + size and addr < p + n:
                    findings += 1
                    print("event %d: al-clear [%d,%d) covers linked vector %d (%d bytes)"
                          % (i, p, p + n, addr, size))
        elif tag == "split-push":
            if not (0 <= b <= VECTOR_BLOCK_BYTES):
                findings += 1
                print("event %d: split-push restbytes %d is outside 0..%d"
                      % (i, b, VECTOR_BLOCK_BYTES))
    print("tools/analyse-free-log.py: %d event(s), %d finding(s), %d entr(ies) still linked"
          % (len(events), findings, len(linked)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
