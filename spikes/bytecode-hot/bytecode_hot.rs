// Hot-loop bridge for the bytecode interpreter.  This is build scaffolding, not a
// ported C file: the aarch64 asm loop keeps the interpreter's locals in registers
// and executes only opcodes that cannot call Rust, allocate, GC, signal, or leave
// the current frame.  Every other opcode returns to `exec_byte_code` with `pc`
// rewound to the opcode start; the original Rust match then owns it.
//
// State layout is duplicated in the asm below; keep the offsets in sync.

#[repr(C)]
pub struct BcHotState {
  pub top: *mut Lisp_Object,
  pub pc: *const c_uchar,
  pub vectorp: *mut Lisp_Object,
  pub bytestr_data: *const c_uchar,
  pub bytestr_length: ptrdiff_t,
  pub const_length: ptrdiff_t,
  pub quitcounter: c_uchar,
  pub _pad: [c_uchar; 7],
  pub qt: Lisp_Object,
  pub qnil: Lisp_Object,
  pub qunbound: Lisp_Object,
  pub lispsym: *mut Lisp_Symbol,
}

// The asm duplicates these numbers; keep them tied to the Rust definitions.
const _: () = {
  assert!(core::mem::offset_of!(BcHotState, top) == 0);
  assert!(core::mem::offset_of!(BcHotState, pc) == 8);
  assert!(core::mem::offset_of!(BcHotState, vectorp) == 16);
  assert!(core::mem::offset_of!(BcHotState, bytestr_data) == 24);
  assert!(core::mem::offset_of!(BcHotState, bytestr_length) == 32);
  assert!(core::mem::offset_of!(BcHotState, const_length) == 40);
  assert!(core::mem::offset_of!(BcHotState, quitcounter) == 48);
  assert!(core::mem::offset_of!(BcHotState, qt) == 56);
  assert!(core::mem::offset_of!(BcHotState, qnil) == 64);
  assert!(core::mem::offset_of!(BcHotState, qunbound) == 72);
  assert!(core::mem::offset_of!(BcHotState, lispsym) == 80);
  assert!(core::mem::offset_of!(Lisp_Symbol_s, val) == 16);
  assert!(GCTYPEBITS == 3 && INTTYPEBITS == 2);
  assert!(Lisp_Type::Lisp_Int0 as u32 == 2 && Lisp_Type::Lisp_Int1 as u32 == 6);
  assert!(Bstack_ref == 0 && Bstack_ref1 == 1 && Bstack_ref7 == 7);
  assert!(Bvarref == 8 && Bvarref7 == 15 && Bvarset == 16 && Bvarset7 == 23);
  assert!(Beq == 61 && Badd1 == 84 && Bsub1 == 83);
  assert!(Bdiff == 90 && Bplus == 92 && Bnegate == 91 && Bmult == 95);
  assert!(Beqlsign == 85 && Bgtr == 86 && Blss == 87 && Bleq == 88 && Bgeq == 89);
  assert!(Bgoto == 130 && Bgotoifnil == 131 && Bgotoifnonnil == 132);
  assert!(Bdiscard == 136 && Bdup == 137);
  assert!(Bstack_set == 178 && Bstack_set2 == 179 && BdiscardN == 182);
  assert!(Bconstant == 192);
};

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
core::arch::global_asm!(r###"// femacs bytecode hot loop (aarch64).  Dispatch-only helper; all non-trivial
// opcodes return to exec_byte_code with pc rewound to the opcode start.
// HotState offsets: top 0, pc 8, vectorp 16, bytestr_data 24,
// bytestr_length 32, const_length 40, quitcounter 48, pad 56,
// qt 56, qnil 64, qunbound 72.
.text
.p2align 2
.globl _femacs_bc_hot
_femacs_bc_hot:
  stp x19, x20, [sp, #-96]!
  stp x21, x22, [sp, #16]
  stp x23, x24, [sp, #32]
  stp x25, x26, [sp, #48]
  stp x27, x28, [sp, #64]
  str x29, [sp, #80]
  str x30, [sp, #88]
  mov x19, x0
  ldr x20, [x19, #8]       // pc
  ldr x21, [x19, #0]       // top
  ldr x22, [x19, #16]      // vectorp
  ldr x23, [x19, #24]      // bytestr_data
  ldr x27, [x19, #40]      // const_length
  ldr x24, [x19, #56]      // Qt
  ldr x25, [x19, #64]      // Qnil
  ldr x26, [x19, #72]      // Qunbound
  adr x28, Lbc_targets
Lloop:
  mov x11, x20             // opcode start, for the slow rewind
  ldrb w9, [x20], #1
  ldr w10, [x28, x9, lsl #2]
  add x10, x28, w10, sxtw
  br x10
Lslow:
  mov x20, x11
  str x20, [x19, #8]
  str x21, [x19, #0]
  mov w0, #0
  b Lret
Lgc:
  str x20, [x19, #8]
  str x21, [x19, #0]
  strb w30, [x19, #48]
  mov w0, #2
  b Lret
Lret:
  ldp x27, x28, [sp, #64]
  ldp x25, x26, [sp, #48]
  ldp x23, x24, [sp, #32]
  ldp x21, x22, [sp, #16]
  ldp x19, x20, [sp], #96
  ldr x30, [sp, #-8]
  ret

// stack-ref 1..7: push top[-n]
Lref1:
  sub x10, x21, #8
  b Lref_load
Lref2:
  sub x10, x21, #16
  b Lref_load
Lref3:
  sub x10, x21, #24
  b Lref_load
Lref4:
  sub x10, x21, #32
  b Lref_load
Lref5:
  sub x10, x21, #40
  b Lref_load
Lref_load:
  ldr x9, [x10]
  add x21, x21, #8
  str x9, [x21]
  b Lloop
Lref6:
  ldrb w9, [x20], #1
  sub x10, x21, x9, lsl #3
  ldr x9, [x10]
  add x21, x21, #8
  str x9, [x21]
  b Lloop
Lref7:
  ldrb w9, [x20], #1
  ldrb w10, [x20], #1
  orr w9, w9, w10, lsl #8
  sub x10, x21, x9, lsl #3
  ldr x9, [x10]
  add x21, x21, #8
  str x9, [x21]
  b Lloop

// stack-set, stack-set2
Lstack_set:
  ldrb w9, [x20], #1
  sub x10, x21, x9, lsl #3
  ldr x9, [x21]
  sub x21, x21, #8
  str x9, [x10]
  b Lloop
Lstack_set2:
  ldrb w9, [x20], #1
  ldrb w10, [x20], #1
  orr w9, w9, w10, lsl #8
  sub x10, x21, x9, lsl #3
  ldr x9, [x21]
  sub x21, x21, #8
  str x9, [x10]
  b Lloop

Ldup:
  ldr x9, [x21]
  add x21, x21, #8
  str x9, [x21]
  b Lloop
Ldiscard:
  sub x21, x21, #8
  b Lloop
LdiscardN:
  ldrb w9, [x20], #1
  tst w9, #0x80
  b.eq 1f
  and w9, w9, #0x7f
  ldr x10, [x21]
  sub x12, x21, x9, lsl #3
  str x10, [x12]
1:
  sub x21, x21, x9, lsl #3
  b Lloop

// constant: opcode in [192, 192+const_length)
Lconstant:
  sub w12, w9, #0xc0
  cmp w12, w27
  b.hs Lslow
  ldr x9, [x22, x12, lsl #3]
  add x21, x21, #8
  str x9, [x21]
  b Lloop

// eq: tagged-pointer equality, no allocation
Leq:
  ldr x9, [x21]
  sub x21, x21, #8
  ldr x10, [x21]
  cmp x9, x10
  csel x9, x24, x25, eq
  str x9, [x21]
  b Lloop

// fixnum unary/binary fast paths; any non-fixnum or 61-bit range failure is slow
Ladd1:
  ldr x9, [x21]
  tst x9, #3
  b.ne Lslow
  asr x10, x9, #2
  adds x10, x10, #1
  b.vs Lslow
  lsl x12, x10, #3
  asr x12, x12, #3
  cmp x12, x10
  b.ne Lslow
  lsl x10, x10, #2
  orr x10, x10, #2
  str x10, [x21]
  b Lloop
Lsub1:
  ldr x9, [x21]
  tst x9, #3
  b.ne Lslow
  asr x10, x9, #2
  subs x10, x10, #1
  b.vs Lslow
  lsl x12, x10, #3
  asr x12, x12, #3
  cmp x12, x10
  b.ne Lslow
  lsl x10, x10, #2
  orr x10, x10, #2
  str x10, [x21]
  b Lloop
Lplus:
  ldr x14, [x21]
  ldr x15, [x21, #-8]
  tst x14, #3
  b.ne Lslow
  tst x15, #3
  b.ne Lslow
  asr x14, x14, #2
  asr x15, x15, #2
  add x10, x14, x15
  lsl x12, x10, #3
  asr x12, x12, #3
  cmp x12, x10
  b.ne Lslow
  lsl x10, x10, #2
  orr x10, x10, #2
  sub x21, x21, #8
  str x10, [x21]
  b Lloop
Lminus:
  ldr x14, [x21]
  ldr x15, [x21, #-8]
  tst x14, #3
  b.ne Lslow
  tst x15, #3
  b.ne Lslow
  asr x14, x14, #2
  asr x15, x15, #2
  sub x10, x15, x14
  lsl x12, x10, #3
  asr x12, x12, #3
  cmp x12, x10
  b.ne Lslow
  lsl x10, x10, #2
  orr x10, x10, #2
  sub x21, x21, #8
  str x10, [x21]
  b Lloop
Lmult:
  ldr x14, [x21]
  ldr x15, [x21, #-8]
  tst x14, #3
  b.ne Lslow
  tst x15, #3
  b.ne Lslow
  asr x14, x14, #2
  asr x15, x15, #2
  mul x10, x14, x15
  smulh x12, x14, x15
  cmp x12, x10, asr #63
  b.ne Lslow
  lsl x12, x10, #3
  asr x12, x12, #3
  cmp x12, x10
  b.ne Lslow
  lsl x10, x10, #2
  orr x10, x10, #2
  sub x21, x21, #8
  str x10, [x21]
  b Lloop

// fixnum comparisons: tagged words compare in the same order
Leqlsign:
  ldr x9, [x21]
  ldr x10, [x21, #-8]
  tst x9, #3
  b.ne Lslow
  tst x10, #3
  b.ne Lslow
  cmp x9, x10
  csel x9, x24, x25, eq
  sub x21, x21, #8
  str x9, [x21]
  b Lloop
Lgtr:
  ldr x9, [x21]
  ldr x10, [x21, #-8]
  tst x9, #3
  b.ne Lslow
  tst x10, #3
  b.ne Lslow
  cmp x9, x10
  csel x9, x24, x25, gt
  sub x21, x21, #8
  str x9, [x21]
  b Lloop
Llss:
  ldr x9, [x21]
  ldr x10, [x21, #-8]
  tst x9, #3
  b.ne Lslow
  tst x10, #3
  b.ne Lslow
  cmp x9, x10
  csel x9, x24, x25, lt
  sub x21, x21, #8
  str x9, [x21]
  b Lloop
Lleq:
  ldr x9, [x21]
  ldr x10, [x21, #-8]
  tst x9, #3
  b.ne Lslow
  tst x10, #3
  b.ne Lslow
  cmp x9, x10
  csel x9, x24, x25, le
  sub x21, x21, #8
  str x9, [x21]
  b Lloop
Lgeq:
  ldr x9, [x21]
  ldr x10, [x21, #-8]
  tst x9, #3
  b.ne Lslow
  tst x10, #3
  b.ne Lslow
  cmp x9, x10
  csel x9, x24, x25, ge
  sub x21, x21, #8
  str x9, [x21]
  b Lloop

// varref: fast path only for a plain symbol bound to a non-Qunbound value
Lvarref1:
  mov w12, #1
  b Lvarref
Lvarref2:
  mov w12, #2
  b Lvarref
Lvarref3:
  mov w12, #3
  b Lvarref
Lvarref4:
  mov w12, #4
  b Lvarref
Lvarref5:
  mov w12, #5
  b Lvarref
Lvarref0:
  mov w12, #0
  b Lvarref
Lvarref6:
  ldrb w12, [x20], #1
  b Lvarref
Lvarref7:
  ldrb w12, [x20], #1
  ldrb w13, [x20], #1
  orr w12, w12, w13, lsl #8
Lvarref:
  ldr x10, [x22, x12, lsl #3]
  and w13, w10, #7
  cbnz w13, Lslow
  ldr x9, [x19, #80]
  add x10, x9, x10
  ldr w13, [x10]
  ubfx w13, w13, #1, #2
  cbnz w13, Lslow
  ldr x13, [x10, #16]
  cmp x13, x26
  b.eq Lslow
  add x21, x21, #8
  str x13, [x21]
  b Lloop

// varset: fast path only for a plain, untrapped symbol
Lvarset1:
  mov w12, #1
  b Lvarset
Lvarset2:
  mov w12, #2
  b Lvarset
Lvarset3:
  mov w12, #3
  b Lvarset
Lvarset4:
  mov w12, #4
  b Lvarset
Lvarset5:
  mov w12, #5
  b Lvarset
Lvarset0:
  mov w12, #0
  b Lvarset
Lvarset6:
  ldrb w12, [x20], #1
  b Lvarset
Lvarset7:
  ldrb w12, [x20], #1
  ldrb w13, [x20], #1
  orr w12, w12, w13, lsl #8
Lvarset:
  ldr x10, [x22, x12, lsl #3]
  and w13, w10, #7
  cbnz w13, Lslow
  ldr x9, [x19, #80]
  add x10, x9, x10
  ldr w13, [x10]
  ubfx w14, w13, #1, #2
  cbnz w14, Lslow
  ubfx w14, w13, #3, #2
  cbnz w14, Lslow
  ldr x15, [x21]
  str x15, [x10, #16]
  sub x21, x21, #8
  b Lloop

// branches: forward only; backward returns to OP_BRANCH for quitcounter/GC/quit
Lgoto:
  ldrb w12, [x20], #1
  ldrb w13, [x20], #1
  orr w12, w12, w13, lsl #8
  add x12, x23, x12
  cmp x12, x20
  b.hs 1f
  add w30, w30, #1
  cbnz w30, 2f
  mov w30, #1
  mov x20, x12
  b Lgc
2:
  mov x20, x12
  b Lloop
1:
  mov x20, x12
  b Lloop
Lgotoifnil:
  ldrb w12, [x20], #1
  ldrb w13, [x20], #1
  orr w12, w12, w13, lsl #8
  ldr x9, [x21]
  cmp x9, x25
  b.ne 1f
  add x12, x23, x12
  cmp x12, x20
  b.hs 2f
  add w30, w30, #1
  cbnz w30, 3f
  mov w30, #1
  mov x20, x12
  sub x21, x21, #8
  b Lgc
3:
  mov x20, x12
  sub x21, x21, #8
  b Lloop
2:
  mov x20, x12
1:
  sub x21, x21, #8
  b Lloop
Lgotoifnonnil:
  ldrb w12, [x20], #1
  ldrb w13, [x20], #1
  orr w12, w12, w13, lsl #8
  ldr x9, [x21]
  cmp x9, x25
  b.eq 1f
  add x12, x23, x12
  cmp x12, x20
  b.hs 2f
  add w30, w30, #1
  cbnz w30, 3f
  mov w30, #1
  mov x20, x12
  sub x21, x21, #8
  b Lgc
3:
  mov x20, x12
  sub x21, x21, #8
  b Lloop
2:
  mov x20, x12
1:
  sub x21, x21, #8
  b Lloop

.p2align 2
Lbc_targets:
  .long Lslow - Lbc_targets
  .long Lref1 - Lbc_targets
  .long Lref2 - Lbc_targets
  .long Lref3 - Lbc_targets
  .long Lref4 - Lbc_targets
  .long Lref5 - Lbc_targets
  .long Lref6 - Lbc_targets
  .long Lref7 - Lbc_targets
  .long Lvarref0 - Lbc_targets
  .long Lvarref1 - Lbc_targets
  .long Lvarref2 - Lbc_targets
  .long Lvarref3 - Lbc_targets
  .long Lvarref4 - Lbc_targets
  .long Lvarref5 - Lbc_targets
  .long Lvarref6 - Lbc_targets
  .long Lvarref7 - Lbc_targets
  .long Lvarset0 - Lbc_targets
  .long Lvarset1 - Lbc_targets
  .long Lvarset2 - Lbc_targets
  .long Lvarset3 - Lbc_targets
  .long Lvarset4 - Lbc_targets
  .long Lvarset5 - Lbc_targets
  .long Lvarset6 - Lbc_targets
  .long Lvarset7 - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Leq - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lsub1 - Lbc_targets
  .long Ladd1 - Lbc_targets
  .long Leqlsign - Lbc_targets
  .long Lgtr - Lbc_targets
  .long Llss - Lbc_targets
  .long Lleq - Lbc_targets
  .long Lgeq - Lbc_targets
  .long Lminus - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lplus - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lmult - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lgoto - Lbc_targets
  .long Lgotoifnil - Lbc_targets
  .long Lgotoifnonnil - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Ldiscard - Lbc_targets
  .long Ldup - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lstack_set - Lbc_targets
  .long Lstack_set2 - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long LdiscardN - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lconstant - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
  .long Lslow - Lbc_targets
"###);

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
unsafe extern "C" {
  fn femacs_bc_hot (state: *mut BcHotState) -> c_int;
}

/// Run the hot loop once.  Returns true when the loop is compiled and enabled.
/// `FEMACS_BC_HOT=0` keeps it off for A/B; any other value enables it.
pub unsafe fn bc_hot_run (state: &mut BcHotState) -> c_int
{
  #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
      if bc_hot_enabled ()
        {
          return femacs_bc_hot (state as *mut BcHotState);
        }
    }
  let _ = state;
  0
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
fn bc_hot_enabled () -> bool
{
  use core::sync::atomic::{AtomicU8, Ordering};
  unsafe extern "C" { fn getenv (name: *const c_char) -> *mut c_char; }
  static ON: AtomicU8 = AtomicU8::new (2);
  match ON.load (Ordering::Relaxed)
    {
      2 =>
        {
          let on = unsafe {
            let p = getenv (c"FEMACS_BC_HOT".as_ptr ());
            p.is_null () || *p != b'0' as c_char || *p.add (1) != 0
          };
          ON.store (on as u8, Ordering::Relaxed);
          on
        }
      v => v != 0,
    }
}
