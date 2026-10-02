# Adversarial review — `mirror(buffer.h)` (frozen commit `f583f33`)

Two adversarial reviewers worked on independent context windows over the frozen commit
`f583f33` (the first full header mirror with bodies: `femacs/src/include/buffer.rs`,
1,968 lines, 53 macros + 112 accessors, plus the `lisp.rs`/`itree.rs`/`stdint.in.rs`
changes it needed).  They were told the artifact is wrong and asked to prove it.  Neither
edited the tree; both wrote scratch work under `/tmp`.  This file records what they
found, what was fixed in the same round, and what remains open.

Scope: `emacs/src/buffer.h` (`fd7afced403`) vs `femacs/src/include/buffer.rs`, the
`lisp.rs`/`itree.rs`/`stdint.in.rs`/`xtask` touches of the same commit, and the gates that
certify them.  Known-open items were not to be re-reported (P-22, P-30, P-33, P-35, P-19).

## Reviewer A — fidelity and semantics

Method: 112 accessors + 44 macro mirrors read against C statement by statement; machine
checks (setter suffix vs assigned member, macro read target, definition order, struct
field sets); self-built C/Rust probes for `sizeof`/`offsetof`, the six constants, the
itree masks; `--check` on mutated copies.

Conclusions: **no field, constant or comparison direction was falsified** — the C↔Rust
mapping is right in the range the reviewer could see.  The defects were in shape and
evidence, not in the transcribed expressions:

| # | Finding | Disposition |
| --- | --- | --- |
| A1 (P1) | The 45 object-like macros had become `fn`, and C *writes* to 14 of them (159 statement sites, 11 `&MODIFF`, one `FETCH_BYTE (pos) =`) in 8 files; the ledger recorded only "l-value becomes a function" and the file header claimed "the same statements".  A `macro_rules!` whose body is a place expression *is* assignable and addressable (the reviewer's `place2.rs` proves it), so this was a shape defect, not a Rust limit. | fixed: C macro → `macro_rules!` of C's kind, body = C's body; `spikes/write-probe.sh` is the new evidence; P-37 rewritten |
| A2 (P1) | `--check -> OK` (quoted as the round's main evidence) compares no bodies, no constants and no bit masks for a header; six hand-injected semantic errors (swapped branches, dropped `- BEG_BYTE`, wrong member, `min` fold removed, …) still passed. | evidence claims corrected; the gap is P-38 with the mutation matrix |
| A3 (P1) | `GET_OVERLAYS_AT` → `SAFE_NALLOCA`'s `size_of_val (&*$buf)` evaluates the pointer; both real call sites (`xdisp.c` NULL, `xfaces.c` uninitialized) abort in a debug build, where C's `sizeof *(buf)` is unevaluated. | fixed: `pointee_size ($buf)` in `lisp.rs` |
| A4 (P2) | The six `bool_bf` members had no names and no recorded bit numbers (clang: 960:0-960:3, 136:0-136:1); the gate has zero coverage of masks. | six mask constants; `spikes/write-probe.sh` compares the raw bytes (15/3) |
| A5 (P2) | The only two C comments missing from the mirror were the two describing `buffer_text`'s bitfields. | restored on those fields |
| A6 (P2) | The ledger's "99 / 19 offsets" mixed an `offsetof` count with a key count (19 = 17 offsets + `sizeof` + `alignof`). | corrected |
| A7 (P2) | `eassert!` dropped the condition text and hardcoded `lisp.rs` as the file. | fixed with `concat! (stringify! ($cond), "\0")` / `concat! (file! (), "\0")` |
| A8 (P2) | `FOR_EACH_PER_BUFFER_OBJECT_AT` was filed under the P-22 loop-macro decision although it is a numeric `for` loop; its real difficulty is assigning to a macro argument, i.e. A1's decision. | written (body as last argument); only `FOR_EACH_LIVE_BUFFER` stays deferred |
| A9 (P2) | `#[cfg (feature = "tree_sitter")]` was not tied to the frozen configuration, so enabling it would grow fields with no layout keys. | `const _: () = assert! (!cfg! (…) \|\| HAVE_TREE_SITTER);` |
| A10 | Include-guard comment contradicted its constant; `buf_charpos_to_bytepos`'s parameter was named `buffer` where `marker.c` says `b`; the four `struct buffer` bitfield comments sat on the next field's doc. | all three corrected |

## Reviewer B — gates and Rust soundness

Method: baseline reproduction, then **16 semantic mutants** run against all three gates
(`--check`, `layout-check`, `gate-fast`); ABI and constant probes taken from the C
compiler rather than from the ledger; a 67-line C/Rust differential of the accessors
(C side: `#undef current_buffer` + a stub thread state; Rust side: an `export_name =
"current_buffer"` static standing in for P-19).

Conclusions: "all gates green" and "the code is wrong" hold simultaneously today —
16/16 mutants passed all three gates.  What it confirmed as correct: `XSETPVECTYPESIZE`
now matches `lisp.h` 1358-1366 term by term (six constants compared against the C
compiler), `record_unwind_protect`'s `Option<unsafe extern "C" fn (Lisp_Object)>` is the
right ABI for `void (*)(Lisp_Object)`, no `static mut` is turned into a reference, the
`offset_from` provenance matches C's same-allocation requirement, and the layout gate is
hard for non-bitfield fields.

| # | Finding | Disposition |
| --- | --- | --- |
| B1 (P0) | `--check`'s macro-body digest comparison is dead code: only a *blank* body is reported, so macro-level mutations (M5/M6/M9/M11b) pass. | P-38 |
| B2 (P0) | Function bodies are never compared, and the stub check has three exemptions (names that are not C functions are skipped — which covers the 105 macros-as-`fn`; `R_FN_BODY`'s `[^{}]*` gives up on any body with a brace).  `ZV () { unimplemented! () }` passed every gate. | P-38; `spikes/accessor-probe.sh` is partial compensation |
| B3 (P0) | `cinventory` extracts **0** constants from `buffer.h`: its one-line `enum { … };` forms are never read, so `BEG`/`GAP_BYTES_DFL`/`UNKNOWN_MODTIME_NSECS` are not even checked for existence (M10/M11/M11b/M14 passed). | P-38 (the tool fix is its own task) |
| B4 (P1) | The itree/`buffer` bit masks had no mechanical evidence (M12 passed). | `spikes/write-probe.sh`; numbers recorded |
| B5 (P1) | The five new crate-root `extern` declarations in `lisp.rs` are an `E0428` landmine for the units that define them (`marker.rs`, `alloc.rs`), and ~62 more pre-existing declarations are the same shape; P-28's rule only covers subrs. | P-39 |
| B6 (P1) | The commit quoted `spikes/inline-probe.sh` (6081 lines) as evidence although it covers only `character.h`; the constant differential it also quoted constrains five values, not the expressions. | evidence claims corrected in the ledger; P-30 still open |
| B7 (P2) | One new `- [ ]` line in the ledger turns a hard `--check` `FAIL` into `inventory: no differences`/exit 0, and `excluded-macros.txt` is a self-service escape hatch with no check that the reason is true. | recorded (gate policy, P-38) |
| B8 (P2) | The layout probe's `#![allow(…)]` list was widened to match the translation units (`static_mut_refs` etc.).  Measured today: nothing real is masked (0 hits with the allow removed), but no policy says which lints may be turned off globally. | recorded as a policy gap in P-38's note |
| B9 (P2) | `#[repr(C, align(8))]` on the four structs is an attribute C does not have; harmless on this target, wrong on a 32-bit one, and unrecorded. | recorded in the ledger |
| B10 (P2) | No behavioural evidence for the accessors in-repo. | `spikes/accessor-probe.sh` (11 lines, with a negative control) |

## Where the artifact stands after this round

- Not falsified: the transcribed expressions, the layouts, the constants (five of them),
  the masks, and the two conversions the reviewer probed.
- Fixed: A1/A3/A5/A7/A8/A9/A10 (code), A2/A6/B6 (claims), B4/B10 (new probes).
- Recorded as open: P-38 (what the gates cannot see for a header mirror, with both
  reviewers' mutation matrices as the measurement) and P-39 (the `E0428` landmine, whose
  fix is a `lisp.rs`-wide refactor and gets its own change).  P-30 remains the umbrella
  for behavioural differentials of the accessors no probe reaches yet.
