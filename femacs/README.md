# femacs — the Rust replication of GNU Emacs

This directory holds the ported sources and the tooling that builds them.

- `src/` mirrors the reference C tree: `emacs/src/NAME.c` → `femacs/src/NAME.rs`,
  `emacs/src/NAME.h` → `femacs/src/include/NAME.rs`, and the other C directories
  keep their own directory name (`femacs/lib/…`, `femacs/lib-src/…`).
- `xtask/` is the build and gate driver. It replaces autoconf and make: one rustc
  invocation per translation unit, `-C panic=abort`, one codegen unit, `--emit=obj`.
- The reference checkout is read-only and lives at `../emacs` (git-ignored).

## Building

```sh
cd femacs
cargo run -p xtask -- gen-config      # config.rs from the frozen configuration
cargo run -p xtask -- layout-check    # compile the layout probe and run it
cargo run -p xtask -- gate-fast       # inventory + layout + compile checks
cargo run -p xtask -- build-tu src/NAME.c
```

Heavy gates (`gate-full`) need the reference configured and built; they are not
part of the fast tier.

## Artifacts

Everything a build produces goes under `../build/`, which is git-ignored and
removed rather than kept: the port does not check build products in, and the
workstation does not accumulate object files between sessions.
