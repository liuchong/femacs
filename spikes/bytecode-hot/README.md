# Bytecode hot-leaf A/B (2026-10-21, rejected)

This directory keeps the aarch64 `asm` hot-loop bridge from the P-324 experiment after it
was detached from the production build.  It is not compiled or linked.

Design: `exec_byte_code` called a leaf asm loop with `top/pc/vectorp/bytestr_data/
const_length/Qt/Qnil/Qunbound/lispsym`; the loop consumed only opcodes whose C bodies
cannot call Rust, allocate, GC, signal or leave the frame, and returned with `pc` rewound
to the first slow opcode.  Slow paths, shared tails, `Breturn`, `sys_setjmp` and the
`maybe_gc -> maybe_quit -> pc` order stayed in the Rust frame.

Measured on the real VM (`build/port/src/emacs`, 3 runs of the six-case corpus, min-of-5
inside each run):

| case | asm off | asm on |
| --- | --- | --- |
| bc-loop | 10.88 ms | 23.45 ms |
| bc-sum | 15.85 ms | 23.26 ms |
| bc-car | 22.38 ms | 53.57 ms |
| regexp-repl | 3.24 ms | 4.60 ms |
| buffer-scan | 0.95 ms | 1.23 ms |
| sort-100k | 3.47 ms | 5.05 ms |

The hot loop was slower on every case (about 1.4–2.4x), including cases that do not stay in
the loop.  The cost is the per-dispatch state handoff and the many slow returns; the
remaining hot opcodes did not form long enough runs to amortize it.  The experiment was
therefore rejected and the default switch VM restored.  No claim is made that a different
asm design cannot work; this one did not.
