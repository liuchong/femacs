#!/usr/bin/env python3
"""Write `src/include/buildobj.rs` from the frozen build's `src/buildobj.h`.

`src/Makefile.in` 673-678 generates that header from `$(ALLOBJS)`, one quoted `name.o`
per object file.  `src/doc.c`'s `Fsnarf_documentation` is the only C code that includes
it, and it does so from inside the function body:

    static char const *const buildobj[] = { #include "buildobj.h" };

so the port needs the same list.  Portable and cheap: the recipe writes the header and
compiles nothing.

    make -C build/emacs-ref/src buildobj.h
    python3 tools/gen-buildobj.py
"""

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
HEADER = REPO / "build/emacs-ref/src/buildobj.h"
OUTPUT = REPO / "src/include/buildobj.rs"

HEAD = """// femacs: generated mirror of the frozen build's `src/buildobj.h` (PORTING.md 0.1, the
// `config.rs` precedent: a generated header lives in `src/include/`).
//
// `src/Makefile.in` 673-678 writes it from `$(ALLOBJS)`: one quoted `name.o` per object
// file, with the directory stripped and `.obj` folded to `.o`.  Regenerate with
//
//     make -C build/emacs-ref/src buildobj.h
//     python3 tools/gen-buildobj.py
//
// `doc.c`'s `Fsnarf_documentation` embeds it as `static char const *const buildobj[]`; the
// port keeps the same array name and order, with `&CStr` elements so `build_string` can take
// `buildobj[i].as_ptr ()` (C's `char const *`).
"""


def main() -> int:
    if not HEADER.is_file():
        print(f"{HEADER} is missing: run `make -C build/emacs-ref/src buildobj.h` first",
              file=sys.stderr)
        return 1
    names = re.findall(r'"([^"]+)"', HEADER.read_text(encoding="utf-8"))
    if not names:
        print(f"{HEADER} has no object names", file=sys.stderr)
        return 1
    body = [
        "#![forbid(unsafe_code)]",
        "",
        f"pub static buildobj: [&core::ffi::CStr; {len(names)}] = [",
    ]
    body += [f'    c"{name}",' for name in names]
    body.append("];")
    OUTPUT.write_text(HEAD + "\n" + "\n".join(body) + "\n", encoding="utf-8")
    print(f"wrote {OUTPUT.relative_to(REPO)} ({len(names)} objects)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
