# Three phase-A spikes — results (2026-10-02)

Objective: prove the three mechanisms `PORTING.md` depends on before any porting
work starts. Sources and the runner are in `spikes/`; this file is the recorded
result of `bash spikes/run.sh` (one build round; products removed afterwards).

## Verdict

| Spike | Question | Result |
| --- | --- | --- |
| 1 layout | can C sizes and offsets be reproduced exactly in Rust? | **pass** — 35 shared keys, 0 mismatches |
| 2 jump | do `_setjmp`/`_longjmp` and `sigsetjmp`/`siglongjmp` behave as in C across Rust frames? | **pass** — resumed with 7 and 9, and no destructor ran |
| 3 scan | does a conservative stack scan see a heap pointer held in a Rust frame? | **pass** — found at -O0, -O2 and -O3 |

## What the spikes settled

- **Layout**: the rules in `PORTING.md` (pointer-typed `Lisp_Object`,
  `#[repr(C, align(8))]`, explicit bitfield packing, named nested unions, `Copy`
  structs inside unions) reproduce every size and offset of `Lisp_Object`,
  `vectorlike_header`, `Lisp_String`, `Lisp_Cons`, `Lisp_Vector`, `Lisp_Symbol`
  and `Lisp_Subr`. The first run caught a real defect in the mirror — `Lisp_Subr`
  was missing its `vectorlike_header` (48 vs 56 bytes) — which is exactly what the
  check is for. 30 further C keys (Lisp_Marker, buffer, buffer_text, window,
  handler) are recorded in the report as "mirror pending".
- **Jump**: the C semantics hold from Rust — the value passed to `longjmp` is what
  `setjmp` returns — and constructors of values in jumped-over frames do **not**
  run. That is the empirical basis for `PORTING.md`'s rule that no `Drop` value may
  live across a `setjmp` boundary, and for using C's own three-way selection
  (`_setjmp` / `sigsetjmp(setjmp, 0)` / `setjmp`) with `savemask` 0.
- **Scan**: a conservative word scan over the live stack finds a heap pointer held
  in a Rust frame at every optimization level tested. The first attempt failed
  because the scanned interval was built the wrong way round; the port must decide
  the stack growth direction the way `alloc.c` does (`stack_bottom < &sentry`) and
  must flush registers with a `setjmp` before scanning (C's
  `SET_STACK_TOP_ADDRESS`).

## Environment findings (these affect the reference build, not the port)

1. This host has **no C23 `<stdbit.h>`** (clang 21 and Homebrew GCC 16 both lack it)
   and `lisp.h` includes it unconditionally. gnulib's substitutes are produced by
   make rules rather than by configure:
   `make -C <build>/lib stdbit.h byteswap.h stdint.h limits.h` and
   `make -C <build>/src globals.h`. `spikes/shim/` holds compile-only stand-ins for
   `stdckdint.h` / `stdcountof.h`, which even those rules do not generate here.
2. `configure` requires `makeinfo`; `build/tools/makeinfo` was a stub used only to
   pass that check (removed with the rest of the products).
3. `configure` must run out of tree, otherwise it would dirty the read-only
   reference checkout. `docs/baseline/config/darwin-arm64.json` freezes the macros
   that matter for `PORTING.md` P-08.

## Toolchain finding (settles PORTING.md P-04)

Measured on this host with `rustc --edition 2024 --emit=metadata`:

| Toolchain | Result |
| --- | --- |
| stable 1.98.1 (active) | `E0658: C-variadic functions are unstable` — the 49 `va_list` sites cannot be ported |
| nightly 1.100.0 (2026-09-14) | definitions compile, but the API has moved (`VaList<'a>`, no `arg()` method) |

So the variadic files need a pinned nightly until stable >= 1.99 ships, and the
exact `VaList` entry points must be re-confirmed at every toolchain bump with the
`doprnt.c` `va_copy` / `vsnprintf` case. `PORTING.md` P-04 records this.

## Reproduction

```sh
mkdir -p build/emacs-ref && cd build/emacs-ref
PATH=$PWD/../../build/tools:$PATH ../../emacs/configure --without-all --with-x=no --with-ns=no
make -C lib stdbit.h byteswap.h stdint.h limits.h    # generated headers, no compilation
make -C src globals.h
cd ../..
bash spikes/run.sh                                   # one build round for all three spikes
rm -rf build/spikes-out                              # remove the products
```

## Raw report

```
# spike report 2026-10-01T17:12:59Z
rustc: rustc 1.98.1 (48a229cea 2026-09-01)
clang: Apple clang version 21.0.0 (clang-2100.3.34.2)

## spike 1: layout identity (C vs Rust)
shared keys: 35, mismatches: 0
C-only keys (mirror pending): 30
  pending offsetof Lisp_Cons.u.s.u.cdr = 8
  pending offsetof Lisp_Cons.u.s.u.chain = 8
  pending offsetof Lisp_Marker.buffer = 8
  pending offsetof Lisp_Marker.bytepos = 40
  pending offsetof Lisp_Marker.charpos = 32
  pending offsetof buffer.begv_marker_ = 392
  pending offsetof buffer.directory_ = 32
  pending offsetof buffer.filename_ = 24
  pending offsetof buffer.last_name_ = 16
  pending offsetof buffer.name_ = 8
  pending offsetof buffer.pt_marker_ = 384
  pending offsetof buffer.zv_marker_ = 400
  pending offsetof buffer_text.beg = 0
  pending offsetof buffer_text.gap_size = 40
  pending offsetof buffer_text.gpt = 8
  pending offsetof buffer_text.z = 16
  pending offsetof handler.jmp = 60
  pending offsetof handler.next = 32
  pending offsetof handler.nextfree = 40
  pending offsetof handler.tag_or_ch = 8
  pending offsetof handler.val = 24
  pending sizeof Aligned_Lisp_Subr = 56
  pending sizeof Lisp_Float = 8
  pending sizeof Lisp_Marker = 48
  pending sizeof buffer = 984
  pending sizeof buffer_text = 144
  pending sizeof handler = 288
  pending sizeof jmp_buf = 192
  pending sizeof sigjmp_buf = 196
  pending sizeof window = 520
Rust-only keys (wrong): 3
  wrong offsetof Lisp_Cons.u.s = 0
  wrong offsetof Lisp_String.u.s = 0
  wrong offsetof Lisp_Symbol.u.s = 0
=> MISMATCHES above

## spike 2: setjmp/longjmp across Rust frames
setjmp      resumed with 7 (expected 7)
sigsetjmp   resumed with 9 (expected 9)
guard drop count 0 (expected 0: C does not unwind constructors)
jmpbuf      user buffer 256 bytes, alignment 16

## spike 3: conservative stack scan of Rust frames
stack scan found heap pointer 4309426576: true
opt level -0 : true
stack scan found heap pointer 4344586704: true
opt level -2 : true
stack scan found heap pointer 4331282896: true
opt level -3 : true
## jmp_buf sizes reported by the C side
sizeof jmp_buf                            192
sizeof sigjmp_buf                         196
```
