// femacs: port of src/coding.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): the type layer.  `coding.h` includes `lisp.h` (line 29) and
// `composite.h` (line 284), so `MAX_COMPOSITION_COMPONENTS` comes from composite.rs; the two
// inlines and the 62 `CODING_*` macros in docs/ports/include-coding.md arrive with coding.c
// (batch C).  `tools/check-cond-eval.py` reports zero disagreements for this header, so the
// ledger's branch set is the frozen configuration's.

// coding.h 27: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_CODING_H: bool = true;

/* `struct ccl_spec` is defined in ccl.h 69-71, which coding.h does not include: C names it
   only as a pointer inside `coding_system.spec`, and an undeclared tag is an incomplete type
   there.  Rust has no incomplete types, so the port declares the stand-in here and deletes it
   when ccl.rs lands (PORTING.md P-70; recorded in the ledger's deviations).  Only the pointer
   is ever used, so the stand-in's own size never enters a layout.  */
pub struct ccl_spec;

// coding.h 252-259: `enum coding_result_code`.
pub type coding_result_code = c_int;
pub const CODING_RESULT_SUCCESS: coding_result_code = 0;
pub const CODING_RESULT_INSUFFICIENT_SRC: coding_result_code = 1;
pub const CODING_RESULT_INSUFFICIENT_DST: coding_result_code = 2;
pub const CODING_RESULT_INVALID_SRC: coding_result_code = 3;
pub const CODING_RESULT_INTERRUPT: coding_result_code = 4;

// coding.h 286-294: `enum composition_state`.
pub type composition_state = c_int;
pub const COMPOSING_NO: composition_state = 0;
pub const COMPOSING_CHAR: composition_state = 1;
pub const COMPOSING_RULE: composition_state = 2;
pub const COMPOSING_COMPONENT_CHAR: composition_state = 3;
pub const COMPOSING_COMPONENT_RULE: composition_state = 4;

// coding.h 364-369 / 371-375.
pub type utf_bom_type = c_int;
pub const utf_detect_bom: utf_bom_type = 0;
pub const utf_without_bom: utf_bom_type = 1;
pub const utf_with_bom: utf_bom_type = 2;

pub type utf_16_endian_type = c_int;
pub const utf_16_big_endian: utf_16_endian_type = 0;
pub const utf_16_little_endian: utf_16_endian_type = 1;

// coding.h 296-312.  Twelve bytes of counters and then `carryover`, whose bound C spells as
// `4 + MAX_COMPOSITION_COMPONENTS * 3 - 2 + 2 + MAX_COMPOSITION_COMPONENTS` = 68 (272 bytes
// at offset 24; sizeof 296, align 4 — measured).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition_status
{
  pub state: composition_state,                        // coding.h 298
  pub method: composition_method,                      // 299
  pub old_form: bool,                                  // 300
  pub length: c_int,                                   // 301
  pub nchars: c_int,                                   // 302
  pub ncomps: c_int,                                   // 303
  pub carryover: [c_int; (4 + MAX_COMPOSITION_COMPONENTS as usize * 3 - 2 + 2
                           + MAX_COMPOSITION_COMPONENTS as usize)],   // 306-310
}

// coding.h 316-344.  `flags`, the two register arrays and the segment length are four-byte
// members (0/4/12/28); the three `bool_bf : 1` members share a 4-byte unit at 32, and
// `cmp_status` follows at 36 (sizeof 332, align 4 — measured).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iso_2022_spec
{
  pub flags: c_uint,                                   // coding.h 319
  pub current_invocation: [c_int; 2],                  // 322
  pub current_designation: [c_int; 4],                 // 327
  pub ctext_extended_segment_len: c_int,               // 331
  pub bits: u32,                                       // single_shifting:1 | bol:1 | embedded_utf_8:1
  pub cmp_status: composition_status,                  // 343
}

// coding.h 346-349.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_mule_spec
{
  pub cmp_status: composition_status,                  // coding.h 348
}

// coding.h 351-361.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct undecided_spec
{
  pub inhibit_nbd: c_int,                              // coding.h 354
  pub inhibit_ied: c_int,                              // 357
  pub prefer_utf_8: bool,                              // 360
}

// coding.h 377-382.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct utf_16_spec
{
  pub bom: utf_bom_type,                               // coding.h 379
  pub endian: utf_16_endian_type,                      // 380
  pub surrogate: c_int,                                // 381
}

// coding.h 444-453: C's anonymous union.  Its size is 336 (the widest member,
// `iso_2022_spec`'s 332, rounded up to the union's 8-byte alignment, which the `ccl` pointer
// sets) and it sits at offset 16 — measured.
#[repr(C)]
pub union coding_system_spec
{
  pub iso_2022: iso_2022_spec,                         // coding.h 446
  pub ccl: *mut ccl_spec,                              // 447 (`struct ccl_spec *`)
  pub utf_16: utf_16_spec,                             // 448
  pub utf_8_bom: utf_bom_type,                         // 449
  pub emacs_mule: emacs_mule_spec,                     // 450
  pub undecided: undecided_spec,                       // 451
}

// coding.h 396-521.  The flag word after `id` packs ten members into 31 bits of one 4-byte
// unit (offset 8): `common_flags:14`, `mode:5`, then six `bool_bf : 1`, `eol_seen:3` and
// `ENUM_BF (coding_result_code) result:3`.  Measured with all six bools set, `mode` 7,
// `common_flags` 5, `eol_seen` 3 and `result` 4:
//
//     bytes 8..11 : 05 c0 f9 47      (= 0x47f9c005)
//
// so the port stores the unit as `bits: u32` and keeps the field reads/writes in the
// accessors (PORTING.md 1.3: the storage unit is what the gate can assert).
#[repr(C)]
pub struct coding_system
{
  pub id: ptrdiff_t,                                   // coding.h 404
  pub bits: u32,                                       // 408-440
  pub max_charset_id: c_int,                           // 442
  pub spec: coding_system_spec,                        // 453
  pub safe_charsets: *mut u8,                          // 455
  pub head_ascii: ptrdiff_t,                           // 462
  pub detected_utf8_bytes: ptrdiff_t,                  // 466
  pub detected_utf8_chars: ptrdiff_t,
  pub produced: ptrdiff_t,                             // 469
  pub produced_char: ptrdiff_t,
  pub consumed: ptrdiff_t,
  pub consumed_char: ptrdiff_t,
  pub src_pos: ptrdiff_t,                              // 471
  pub src_pos_byte: ptrdiff_t,
  pub src_chars: ptrdiff_t,
  pub src_bytes: ptrdiff_t,
  pub src_object: Lisp_Object,                         // 472
  pub source: *const u8,                               // 473 (`const unsigned char *`)
  pub dst_pos: ptrdiff_t,                              // 475
  pub dst_pos_byte: ptrdiff_t,
  pub dst_bytes: ptrdiff_t,
  pub dst_object: Lisp_Object,                         // 476
  pub destination: *mut u8,                            // 477
  pub charbuf: *mut c_int,                             // 490
  pub charbuf_size: c_int,                             // 491
  pub charbuf_used: c_int,
  pub carryover: [u8; 64],                             // 493
  pub carryover_bytes: c_int,                          // 494
  pub default_char: c_int,                             // 496
  pub detector: Option<unsafe extern "C" fn (*mut coding_system,
                                              *mut coding_detection_info) -> bool>,  // 498
  pub decoder: Option<unsafe extern "C" fn (*mut coding_system)>,                    // 499
  pub encoder: Option<unsafe extern "C" fn (*mut coding_system) -> bool>,            // 500
}

// coding.h 384-392: the detection bookkeeping `detector` fills in.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct coding_detection_info
{
  pub checked: c_int,                                  // coding.h 388
  pub found: c_int,                                    // 390
  pub rejected: c_int,                                 // 392
}

// C: coding.h 30-250's `CODING_*` constants and inline accessors, and 523-790's prototypes
// and macros — (not mirrored yet; they land with coding.c, batch C).
