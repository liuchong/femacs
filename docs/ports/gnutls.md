# emacs/src/gnutls.c (3196 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 7
- [x] gnutls-available-p  (Fgnutls_available_p, 0/0, doc af3baad084da, line 2949)
- [x] gnutls-asynchronous-parameters  (Fgnutls_asynchronous_parameters, 2/2, line 979, `have_gnutls`)
- [x] gnutls-get-initstage  (Fgnutls_get_initstage, 1/1, line 992, `have_gnutls`)
- [x] gnutls-errorp  (Fgnutls_errorp, 1/1, line 1002, `have_gnutls`)
- [x] gnutls-error-fatalp  (Fgnutls_error_fatalp, 1/1, line 1015, `have_gnutls`)
- [x] gnutls-error-string  (Fgnutls_error_string, 1/1, line 1047, `have_gnutls`)
- [x] gnutls-deinit  (Fgnutls_deinit, 1/1, line 1076, `have_gnutls`)

## Functions defined — 20
- [x] syms_of_gnutls  (line 3019)
- [x] gnutls_try_handshake  (line 629, `have_gnutls`)
- [x] emacs_gnutls_write  (line 745, `have_gnutls`)
- [x] emacs_gnutls_read  (line 779, `have_gnutls`)
- [x] emacs_gnutls_record_check_pending  (line 731, `have_gnutls`)
- [x] emacs_gnutls_transport_set_errno  (line 738, `have_gnutls` + `windowsnt`)
- [x] emacs_gnutls_deinit  (line 938, `have_gnutls`)
- [x] check_memory_full  (line 593, `have_gnutls`)
- [x] gnutls_audit_log_function  (line 604, `have_gnutls` + `have_gnutls3`)
- [x] gnutls_log_function  (line 615, `have_gnutls`)
- [x] gnutls_log_function2  (line 622, `have_gnutls`)
- [x] emacs_gnutls_nonblock_errno  (line 670, `have_gnutls` + `not (windowsnt)`)
- [x] emacs_gnutls_handshake  (line 694, `have_gnutls`)
- [x] emacs_gnutls_strerror  (line 804, `have_gnutls`)
- [x] emacs_gnutls_handle_error  (line 815, `have_gnutls`)
- [x] gnutls_make_error  (line 906, `have_gnutls`)
- [x] gnutls_deinit_certificates  (line 925, `have_gnutls`)
- [x] gnutls_hex_string  (line 1084, `have_gnutls`)
- [x] emacs_gnutls_certificate_export_pem  (line 1104, `have_gnutls`)
- [x] init_gnutls_functions  (line 304, `have_gnutls` + `windowsnt`)

## Macros defined — 4 (the Windows arm's loader pair + the two log macros)
- [x] DEF_DLL_FN  (w32common.h 77-79's expansion, written in long form; `have_gnutls` + `windowsnt`)
- [x] LOAD_DLL_FN  (w32common.h 82-89's expansion; `have_gnutls` + `windowsnt`)
- [x] GNUTLS_LOG  (src/gnutls.h 64-68, `have_gnutls`)
- [x] GNUTLS_LOG2  (src/gnutls.h 70-74, `have_gnutls`)

## Types and their fields — 1 + the third-party surface
- [x] gnutls_initstage_t  (src/gnutls.h 37-54's enum; `have_gnutls`)
- [x] extra_peer_verification / CERTIFICATE_NOT_MATCHING  (C 99-102)
- [x] the `<gnutls/gnutls.h>` / `<gnutls/x509.h>` / `<gnutls/crypto.h>` surface this slice names
  (P-88 ①: declared in the unit; `docs/ports/gnutls.md`'s segment note and `## Evidence` ⑤)

## File-scope objects — 2
- [x] gnutls_global_initialized  (C 90, `have_gnutls`)
- [x] gnutls_free_func  (C 302, `have_gnutls` + `windowsnt`)
- [x] Vlibgnutls_version  (defined by this unit; the `DEFVAR_LISP`-owned variable)

> **One of P-86's class — and this ledger replaces the P-08 reading it carried until now.**
> The frozen configuration leaves `HAVE_GNUTLS` `undef`
> (`docs/baseline/config/darwin-arm64.macros.tsv` 1381), so C's body C 30-2336 is dead text and
> `tools/extract-abi.py --check` has no live item for it (P-86 ②/⑩).  Until this revision the
> ledger said "dead text recorded here, not transcribed (P-08)" and the unit carried only the live
> 124 lines.  **P-86 ① replaces that**: this revision transcribes C 30-1130 with **both arms of
> every `#if`**, each item under the gate that names C's macro, and the frozen object is *still*
> byte-for-byte the one the live text produces (Evidence ①) because every new item sits under the
> file gate.  The P-08 measurements above are kept (they are what the previous revision measured);
> only its conclusion — "not transcribed" — is withdrawn.
>
> **The frozen object is not the 336-byte empty one.**  This file's whole body is *not* dead: C
> 2965-3016's `Fgnutls_available_p` and C 3018-3196's `syms_of_gnutls` are live, so
> `build/femacs/gnutls.o` is **4272 bytes** (`__TEXT` 895, `__DATA` 248; the reference's own
> object is 7144 bytes with `_Fgnutls_available_p`, `_syms_of_gnutls`, `_Vlibgnutls_version` and
> `_Sgnutls_available_p`).  P-86 ⑧'s 336-byte criterion therefore does not apply to this file, and
> neither does its "gate the include graph" clause: C's six includes (C 23-28) sit *before*
> C 30's `#ifdef HAVE_GNUTLS`, so C itself does not gate them, and the live tail needs `lisp.h`
> and `src/gnutls.h` whatever the gate says.  That is why `process.h` alone is gated in the unit
> (its only reader is the gated text, the `atimer.rs` shape) and the other three are recorded.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself, C 30's `#ifdef HAVE_GNUTLS` | `feature = "have_gnutls"` — already registered (`feature-coverage.tsv` 59) | `HAVE_GNUTLS` is defined |
| C 82-86 and C 104-589 (`WINDOWSNT`: `<windows.h>`, `w32common.h`, `w32.h`, the 102 `DEF_DLL_FN`/`LOAD_DLL_FN` pairs, `init_gnutls_functions`, the 102 renames) | `feature = "windowsnt"` (registered, `feature-coverage.tsv` 8) | a Windows build |
| C 704/713 (`#ifdef WINDOWSNT` / `#else` inside `emacs_gnutls_handshake`), C 737-743 | `windowsnt` / `not (windowsnt)` | each side |
| C 669-692 (`# ifndef WINDOWSNT`) | `not (feature = "windowsnt")` | every non-Windows build |
| C 677 (`#  ifdef _AIX`) | `feature = "aix"` (registered, `feature-coverage.tsv` 146; `_AIX` is `undef`, macros.tsv 4036) | AIX |
| C 603-612, 154, 242-300, 343-344, 395-441, 535-581, 838-841, 878-882 (`HAVE_GNUTLS3`) | `feature = "have_gnutls3"` | the header defines `HAVE_GNUTLS3` (`src/gnutls.h` 26-29, i.e. GnuTLS 3.x) |
| C 871-876 (`EMSGSIZE`) | `feature = "emsgsize"` | `EMSGSIZE` is defined — **the frozen table defines it (40, macros.tsv 870)**, so this arm is taken once the file gate is on |
| C 878-882 (`ECONNRESET`) | `feature = "econnreset"` | `ECONNRESET` is defined — **the frozen table defines it (54, macros.tsv 834)** |
| the ten version conditions C 32-80 and the eleven `HAVE_GNUTLS_*` they `#define`, plus each `#ifdef HAVE_GNUTLS_X` site (C 129, 134, 234, 244, 248, 253, 258, 270, 282, 297, 327, 330, 390, 397, 401, 406, 411, 419, 425, 438, 471, 495, 537, 541, 546, 551, 559, 565, 578) | one feature per macro: `have_gnutls_x509_system_trust`, `have_gnutls_certificate_set_x509_key_file2`, `have_gnutls_cipher_get_iv_size`, `have_gnutls_cipher_get_tag_size`, `have_gnutls_digest_list`, `have_gnutls_mac_get_nonce_size`, `have_gnutls_aead`, `have_gnutls_etm_status`, `have_gnutls_ext_get_name`, `have_gnutls_compression_get` | a build whose `GNUTLS_VERSION_NUMBER` passes the matching C 32-80 condition |
| C 1608-1611's `# if 0`, C 2454-2464's `#if 0` (inside C 1131-2946, not in this slice) | no gate — dead in every configuration | never |

`windowsnt` and `aix`, `emsgsize`, `econnreset` and the ten `have_gnutls_*` names: the four
already-registered ones are named above; the other thirteen are Owed 3 (the feature table is a
generated file, `tools/feature-coverage.py --write`).

## Decisions

- **The slice is C 30-1130, and its boundary is a function end.**  C 1130 closes
  `emacs_gnutls_certificate_export_pem`; the next definition in C order is C 1132's
  `emacs_gnutls_certificate_details` (**not ported**).  The next *live* item of the frozen
  configuration is C 2949's `Fgnutls_available_p`, already landed at this unit's end.  The choice
  of boundary is the dependency closure of P-91: every `static` the slice calls is defined inside
  it (`check_memory_full`, the two log functions, `emacs_gnutls_strerror`,
  `emacs_gnutls_handle_error`, `gnutls_try_handshake`, `gnutls_deinit_certificates`,
  `gnutls_hex_string`, `emacs_gnutls_nonblock_errno`), so no site waits on a C line outside it.
- **The gate is measured, not inferred.**  `python3 tools/check-cond-eval.py src/gnutls.c` ->
  `104 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  `HAVE_GNUTLS` is `undef`
  (macros.tsv 1381) and `tools/cinventory.py`'s branch map makes C 30-1130 **entirely dead** in
  the frozen configuration (the live set is C 1-29 plus the two live functions); the two
  sub-gates the frozen table *does* define are `EMSGSIZE` 40 and `ECONNRESET` 54.
- **A `#define` has no Rust counterpart, so the ten version conditions are carried as the port's
  comment table and every site carries the feature named after the macro it tests** (P-86 ①).  The
  unit's head comment spells the same table; the `#if` conditions themselves are clang's.
- **`<gnutls/gnutls.h>`, `<gnutls/x509.h>` and `<gnutls/crypto.h>` (`src/gnutls.h` 23-24, 28)
  have no file in the reference tree, so their declarations stand in the unit** (P-88 ①: not
  under `femacs/src/include/`, which mirrors the *reference* headers).  The enumerated surface is
  exactly what C 30-1130 names: 31 type names, 15 constants/function declarations on the
  non-Windows arm, and byte-for-byte header lines in `/// C:` notes.
- **`src/gnutls.h`'s own `#ifdef HAVE_GNUTLS` arm is still the P-08 stub in its mirror**
  (`femacs/src/include/gnutls.rs`, 26 lines, unchanged this round — it is not in this lane's write
  scope), so the eleven names this slice needs from it (`gnutls_initstage_t` and its members,
  `GNUTLS_INITSTAGE`, `GNUTLS_LOG`, `GNUTLS_LOG2`, the two `GNUTLS_EMACS_ERROR_*` codes,
  `GNUTLS_EMACS_HANDSHAKES_LIMIT`) are declared at this use site under the same gate — the
  P-72 rule-4 shape `femacs/src/font.rs` 380-381 uses for `nconc2`, and Owed 4 moves them into
  the mirror (P-88 ⑤/P-26's single-declaration-point rule).
- **`GNUTLS_LOG`/`GNUTLS_LOG2` keep C's adjacent-literal concatenation** via `concat!`, so their
  `$string` is a `literal`; every call site inside C 30-1130 passes one (`GNUTLS_LOG` at C 312,
  952, 959; `GNUTLS_LOG2` at C 453, 843, 853, 859, 895).  `GNUTLS_LOG2i` is not called in the
  slice and stays owed with the rest of the mirror's arm.
- **The Windows arm's 102 `DEF_DLL_FN`/`LOAD_DLL_FN` pairs are written in long form**, the shape
  `femacs/src/decompress.rs` 212-271 (first consumer) and `femacs/src/lcms.rs` 204-290 (second)
  established: Rust has no token pasting, so `W32_PFN_<func>` and `fn_<func>` are parameters.  C's
  `#define <func> fn_<func>` renames have no counterpart either, so every call site of a renamed
  entry point is spelled twice (`fn_<func> (...)` under `windowsnt`, the plain name otherwise) —
  C 646-649, 657, 762, 786, 815, 843, 860-861, 888, 953, 960, 966.
- **The two `#if 0`-style dead conditions in this slice's neighbourhood (C 2340-2356, 2454-2464)
  are outside it**; the slice's own conditions are the 54 the evaluator lists.
- **Line-count basis (decided 2026-10-06, status 3.678)**: that table's newest `gnutls` row is
  `gnutls 0.41 total` (the measured shortfall against the file's total, the two owed ranges named);
  the `live` row this note called "still the old" is superseded.  The note as it stood: the old
  `live`/0.10 reading (124 live vs 122).  With the P-86 transcription the row must move to the
  `total` basis (C 3196 vs the port's 1897 = 40.7% drift, so tolerance 0.41) — Owed 1, and
  `extract-abi --check` cannot pass until it does (P-86 ⑩: the baseline files are outside this
  lane's write scope).

## C line ranges

The rows are **contiguous** on purpose, in ascending order with no overlap:
`docs/port-map.md` binds this ledger to `src/gnutls.c` (3196 lines) *and* to `src/gnutls.h`
(100 lines), and `tools/check-dead-file-coverage.py` reads the same table against **both**
references, so the union runs C 1-3196 and covers the header's 100 lines with it.  Rows marked
**landed** are in `femacs/src/gnutls.rs`; rows marked **not ported** are the text later slices
owe.  The one slice this revision lands is C 30-1130 (rows 5-35 below).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block and its blank | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, 0.1-19) |
| 19-22 | `<config.h>`, `<errno.h>`, `<stdio.h>` and the blank | ungated in C too; **landed** (`config.rs`, `errno.in.rs`, `stdio.rs`) |
| 23-29 | the six includes (`lisp.h`, `process.h`, `gnutls.h`, `coding.h`, `buffer.h`, `pdumper.h`) and the blank | outside C 30's gate in C; **landed** for `lisp.h`/`gnutls.h` (ungated) and `process.rs` (under `have_gnutls`); `coding.h`/`buffer.h`/`pdumper.h` carry no reader in C 30-1130 and stay in their mirrors (their text is C 1131-3196) |
| 30-31 | C 30's `#ifdef HAVE_GNUTLS` and the blank | the file gate `have_gnutls` (both arms carried: the live tail is C 2948-3196) |
| 32-80 | the ten `GNUTLS_VERSION_NUMBER` conditions, the eleven `HAVE_GNUTLS_*` `#define`s and C's prose | carried as the port's comment table; the per-macro features are the sites' (see Gate mapping) |
| 81-86 | the blank and C 82-86's `WINDOWSNT` include arm (`<windows.h>`, `w32common.h`, `w32.h`) | `have_gnutls` + `windowsnt`; **landed** (the two mirrors; `<windows.h>` has no file in the tree) |
| 87-102 | the blank, the four static declarations, C 94-96's `HAVE_GNUTLS3` audit-log declaration, the `enum extra_peer_verification` and the blank | `have_gnutls` (+ `have_gnutls3`); **landed**: the three forward declarations are dropped (0.1-12), `gnutls_global_initialized` and the enum are the port's |
| 103-105 | the blank, C 104's `# ifdef WINDOWSNT` and the blank | the Windows arm's head (see the next row) |
| 106-300 | the 102 `DEF_DLL_FN` uses, with C 129/134/154/234/242/244/248/253/258/270/282/297's nested `#ifdef`s | `have_gnutls` + `windowsnt` + the nested sub-gates; **landed** as 102 attribute-plus-invocation pairs (text-only: no Windows toolchain on this machine) |
| 301-303 | the blank, `static gnutls_free_function *gnutls_free_func;` and the blank | `have_gnutls` + `windowsnt`; **landed** |
| 304-458 | `init_gnutls_functions` (the 102 `LOAD_DLL_FN`s, C 445's `GetProcAddress` for `gnutls_free`, C 450-455's log) | `have_gnutls` + `windowsnt` (+ the same nested sub-gates on the loads); **landed** |
| 459-590 | the 102 `#define <func> fn_<func>` renames, C 583-587's `gnutls_free` pair, C 589's `# endif /* WINDOWSNT */` and the blanks | carried as the port's comment block; the call sites spell the two arms (**Deviation 3**) |
| 591-600 | the page break, C 592's comment and `check_memory_full` (C 593-601) | `have_gnutls`; **landed** |
| 601-612 | C 603-612's `# ifdef HAVE_GNUTLS3` arm: the comment and `gnutls_audit_log_function` (C 604-611) | `have_gnutls` + `have_gnutls3`; **landed** |
| 613-618 | `gnutls_log_function` (C 614-618) and its comment | `have_gnutls`; **landed** |
| 619-627 | the blank, C 620's comment and `gnutls_log_function2` (C 622-627) | `have_gnutls`; **landed** |
| 628-668 | the blank, C 628's comment and `gnutls_try_handshake` (C 629-667) | `have_gnutls`; **landed** (the `while`-with-assignment as a loop, PORTING.md 1.5) |
| 669-692 | C 669-692's `# ifndef WINDOWSNT` arm: `emacs_gnutls_nonblock_errno` (C 670-691), with C 677's `_AIX` sub-arm | `have_gnutls` + `not (windowsnt)`; `aix` for C 677; **landed** |
| 693-729 | the blank, C 694's `static int` and `emacs_gnutls_handshake` (C 694-729), with C 704-723's two arms | `have_gnutls`; C 704-712 `windowsnt`, C 713-723 `not (windowsnt)`; **landed** |
| 730-736 | the blank and `emacs_gnutls_record_check_pending` (C 731-735) | `have_gnutls`; **landed** |
| 737-744 | C 737-743's `# ifdef WINDOWSNT` arm: `emacs_gnutls_transport_set_errno` (C 738-742) and the blank | `have_gnutls` + `windowsnt`; **landed** |
| 745-778 | `emacs_gnutls_write` (C 745-777) and the blank | `have_gnutls`; **landed** (the `do … while` as a loop) |
| 779-803 | `emacs_gnutls_read` (C 779-802) and the blank | `have_gnutls`; **landed** |
| 804-810 | `emacs_gnutls_strerror` (C 804-809) and the blank | `have_gnutls`; **landed** |
| 811-901 | C 811-814's comment, `emacs_gnutls_handle_error` (C 815-900) and the blank | `have_gnutls`; the two `switch`es are the port's `match`es, C 838-840's arm is `have_gnutls3`, C 871-876 `emsgsize`, C 878-882 `have_gnutls3` + `econnreset`; **landed** |
| 902-924 | C 902-905's comment, `gnutls_make_error` (C 906-923) and the blank | `have_gnutls`; **landed** |
| 925-937 | `gnutls_deinit_certificates` (C 925-936) and the blank | `have_gnutls`; **landed** (C 931-932's pointer subscript is the port's `.add`, **Deviation 4**) |
| 938-978 | `emacs_gnutls_deinit` (C 938-977) and the blank | `have_gnutls`; **landed** |
| 979-991 | the DEFUN `gnutls-asynchronous-parameters` (C 979-990) and the blank | `have_gnutls`; **landed** |
| 992-1001 | the DEFUN `gnutls-get-initstage` (C 992-1000) and the blank | `have_gnutls`; **landed** |
| 1002-1014 | the DEFUN `gnutls-errorp` (C 1002-1013) and the blank | `have_gnutls`; **landed** |
| 1015-1046 | the DEFUN `gnutls-error-fatalp` (C 1015-1045) and the blank | `have_gnutls`; **landed** |
| 1047-1075 | the DEFUN `gnutls-error-string` (C 1047-1074) and the blank | `have_gnutls`; **landed** |
| 1076-1083 | the DEFUN `gnutls-deinit` (C 1076-1082) and the blank | `have_gnutls`; **landed** |
| 1084-1103 | `gnutls_hex_string` (C 1084-1102) and the blank | `have_gnutls`; **landed** (C 1089-1090's `ckd_mul`/`ckd_add` as the P-15 pair) |
| 1104-1130 | `emacs_gnutls_certificate_export_pem` (C 1104-1130) | `have_gnutls`; **landed** — this is the slice's last row (C 1130 closes the function; the next definition is C 1132) |
| 1131-2336 | **not ported**: the rest of the `HAVE_GNUTLS` body — `emacs_gnutls_certificate_details` (C 1132), the DEFUNs `gnutls-peer-status-warning-describe` (C 1325), `gnutls-peer-status` (C 1385), `gnutls-format-certificate` (C 1635), `gnutls-boot` (C 1915) and `gnutls-bye` (C 2305), and the helpers `emacs_gnutls_global_init`, `gnutls_ip_address_p`, `emacs_gnutls_global_deinit`, `boot_error`, `gnutls_verify_boot`, `key_file2_aux` — plus C 2336's `#endif` | `have_gnutls`; a later slice |
| 2337-2947 | **not ported**: the `HAVE_GNUTLS3` block (C 2338-2947 — the `gnutls_cipher_get_iv_size`/`gnutls_cipher_get_tag_size`/`gnutls_digest_list` fallbacks and the seven DEFUNs `gnutls-ciphers` 2358, `gnutls-symmetric-encrypt` 2645, `gnutls-symmetric-decrypt` 2671, `gnutls-macs` 2697, `gnutls-digests` 2736, `gnutls-hash-mac` 2766, `gnutls-hash-digest` 2865, with C 2454-2464's `#if 0`) and its `#endif` | `have_gnutls3`; a later slice |
| 2948-2964 | the blank, `Fgnutls_available_p`'s DEFUN head (C 2949-2960), `(void)`, the body's first lines and C 2965's `#ifdef HAVE_GNUTLS` | **landed** by this unit's live half (the DEFUN record and the docstring) |
| 2965-3013 | the dead `#ifdef HAVE_GNUTLS` arm inside `Fgnutls_available_p` and C 3013's `#endif` | **not transcribed**: the unit carries the previous revision's note that this arm is dead text; P-86 ① says it is transcribed when this function's arms are taken up (Owed 5) |
| 3014-3017 | `return capabilities;`, the closing brace and the blank | **landed** (the live half) |
| 3018-3196 | `syms_of_gnutls` (C 3018-3196), including C 3027-3031's and C 3036-3142's dead `HAVE_GNUTLS` arms | **landed** for the live part (`DEFVAR_LISP`, `make_fixnum (-1)`, `defsubr`, the 49 `DEFSYM`s); the two dead arms are recorded as the previous revision's notes (Owed 5) |

## Evidence

**① The frozen configuration (`build-tu`).**  `cd femacs && CARGO_TARGET_DIR=/tmp/gnutls-lane/cargo
cargo run -q -p xtask -- build-tu src/gnutls.c` -> `built src/gnutls.rs -> ../build/femacs/gnutls.o`,
**exit 0**, **2 warnings** (both `unused doc comment` in `include/stdbit.in.rs` and
`include/lisp.rs` — mirror-side and pre-existing), and the object is **4272 bytes**
(`size`: `__TEXT 895, __DATA 248`; `nm` shows `_Fgnutls_available_p`, `_Vlibgnutls_version`,
`_Sgnutls_available_p` and `syms_of_gnutls`'s local forwarder).  The reference's own object,
`build/emacs-ref/src/gnutls.o`, is **7144 bytes** with the same two exported symbols.  Re-run after
every later edit to the unit (P-86 ⑨): the same `exit 0` and the same 4272 bytes.  **This is the
measurement that refutes the task's "336-byte empty object" expectation**: the file has live text.

**② The arm flip** (P-86 ⑥: the file gate *and* every sub-gate a GnuTLS 3.8.13 build would take —
this machine's header is `/opt/homebrew/include/gnutls/gnutls.h`, `GNUTLS_VERSION_NUMBER`
**0x03080d**):

    cd femacs && CARGO_TARGET_DIR=/tmp/gnutls-lane/cargo cargo run -q -p xtask -- probe \
      src/gnutls.rs /tmp/gnutls-lane/gnutls.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_gnutls"' --cfg 'feature="have_gnutls3"' \
      --cfg 'feature="have_gnutls_certificate_set_x509_key_file2"' \
      --cfg 'feature="have_gnutls_x509_system_trust"' --cfg 'feature="have_gnutls_cipher_get_iv_size"' \
      --cfg 'feature="have_gnutls_cipher_get_tag_size"' --cfg 'feature="have_gnutls_digest_list"' \
      --cfg 'feature="have_gnutls_mac_get_nonce_size"' --cfg 'feature="have_gnutls_aead"' \
      --cfg 'feature="have_gnutls_etm_status"' --cfg 'feature="have_gnutls_ext_get_name"' \
      --cfg 'feature="emsgsize"' --cfg 'feature="econnreset"'

-> **exit 1**, **34 errors**, **26 warnings**, no rlib.  Every one of the 34 is the *same* error:
`error[E0609]: no field <name> on type Lisp_Process`, and the ten names are exactly the member
block `process.h` 194-216 guards with `#ifdef HAVE_GNUTLS`:

| field | sites | | field | sites |
| --- | --- | --- | --- | --- |
| `gnutls_initstage` | 9 | | `gnutls_p` | 3 |
| `gnutls_state` | 7 | | `gnutls_log_level` | 1 |
| `gnutls_certificates` | 5 | | `gnutls_complete_negotiation_p` | 1 |
| `gnutls_x509_cred` | 3 | | `gnutls_certificates_length` | 1 |
| `gnutls_anon_cred` | 3 | | `gnutls_boot_parameters` | 1 |

So the reading is: **the unit's own text has 0 errors of any class it owns; all 34 are one named
mirror gap** (Owed 4).  The 26 warnings are all `unused doc comment` and all in the mirrors (the
`///`-before-`extern "C"` shape `include/lisp.rs` 3934 and `include/stdbit.in.rs` 644 carry).

The two edits that followed the first flip — `is_non_blocking_client` read through the mirror's
mask, and the pointer subscript in `gnutls_deinit_certificates` — each removed one error class;
the reading above is the one this revision's text produces (`probe` was run five times in this
lane: the four earlier ones are the ones this section's classification is derived from, plus the
frozen re-runs P-86 ⑨ requires).

**③ The reading is not vacuous** (P-86 ⑦'s mutation test).  Same materialisation, same flags, with
one injected type error (`GNUTLS_E_SUCCESS`'s `0` replaced by a `&CStr`) compiled directly with
rustc: the output changes from 34 to **36 errors** — the injected `E0308`
(`expected i32, found &CStr`, at the unit's own line) plus the one mirror-side `E0658`
(`include/lisp.rs` 3756's `verror`, `c_variadic`, which the *probe* run does not report because it
runs under the repository's nightly toolchain while a direct `rustc` from a non-repository
directory uses the default toolchain).  The unit's text is therefore read by the compile, and the
"0 errors of its own" classification is a real reading rather than an empty one.

**④ The mirror gap, closed with a stub, is the *only* blocker.**  With `include/process.rs`'s copy
in the materialisation extended by the ten fields (three of them as plain `bool`s where C has
`bool_bf`, i.e. a stub that satisfies names, *not* a layout claim) and nothing else changed, the
same rustc invocation gives **exit 0, 0 errors, an 8,467,400-byte rlib, 43 warnings** — 26 the
mirrors' pre-existing `unused doc comment`, **15 this unit's**
`extern block uses type <opaque struct>, which is not FFI-safe` (the P-88 ① opaque-handle
declarations, named rather than silenced), and 2 `unistd.in.rs`'s `read`/`write`.  This is the second half of the classification: after
the mirror lands, the port compiles.

**⑤ The third-party surface is measured, not remembered.**  The declarations in the unit's
`<gnutls/gnutls.h>` segment are clang readings against this machine's header
(`/opt/homebrew/include/gnutls/gnutls.h`, gnutls **3.8.13**): `sizeof (gnutls_datum_t)` **16**
(`data` 0, `size` 8), every `*_t` enum **4**, `gnutls_session_t` / `gnutls_x509_crt_t` /
`gnutls_certificate_credentials_t` / `gnutls_anon_client_credentials_t` / `gnutls_transport_ptr_t`
/ `gnutls_cipher_hd_t` / `gnutls_aead_cipher_hd_t` / `gnutls_hash_hd_t` / `gnutls_hmac_hd_t`
**8** (opaque pointers), the four function-pointer typedefs **8**, and the twelve `GNUTLS_E_*`
codes plus `GNUTLS_X509_FMT_PEM`:

    GNUTLS_E_SUCCESS 0      GNUTLS_E_AGAIN -28        GNUTLS_E_INTERRUPTED -52
    GNUTLS_E_INVALID_SESSION -10    GNUTLS_E_UNEXPECTED_PACKET_LENGTH -9
    GNUTLS_E_MEMORY_ERROR -25       GNUTLS_E_WARNING_ALERT_RECEIVED -16
    GNUTLS_E_FATAL_ALERT_RECEIVED -12  GNUTLS_E_PREMATURE_TERMINATION -110
    GNUTLS_E_LARGE_PACKET -7        GNUTLS_E_PUSH_ERROR -53
    GNUTLS_E_SHORT_MEMORY_BUFFER -51   GNUTLS_E_APPLICATION_ERROR_MIN -65500
    GNUTLS_X509_FMT_PEM 1   GNUTLS_X509_FMT_DER 0

The probe source is `#include <gnutls/gnutls.h>` + `printf` of those `sizeof`/values, compiled
`clang -I/opt/homebrew/include -o /tmp/gnutls-lane/probe /tmp/gnutls-lane/probe.c` and run.  The
values are the *host header's*; a build against a different GnuTLS release is **UNVERIFIED**
(these are `#define`s and enums the header owns, and the port carries them as its own constants).

**⑥ The parallel gates.**

| command | reading |
| --- | --- |
| `python3 tools/check-cond-eval.py src/gnutls.c` | `104 conditions, 0 unknown, 0 disagreements [VERIFIED]` |
| `python3 tools/check-citations.py` | `676 file(s) scanned, **0 drift**, 0 range, 730 external, 35 allowed, 0 stale`, exit 0 |
| `python3 tools/check-declarations.py` | `10 declaration sites (0 forbidden, 10 allowed literals), 0 failures` |
| `python3 tools/check-dead-file-coverage.py` | the table above (`src/gnutls.c` + `src/gnutls.h`) |
| `python3 tools/check-port-lines.py` | `58 file(s) under ten percent … 0 new, 3 stale`, with `stale src/gnutls.c: no longer under the threshold` — the file has left the P-86 class (1897 / 3196 = 59.4%) |
| `python3 tools/extract-abi.py --check emacs/src/gnutls.c femacs/src/gnutls.rs` | **FAIL** — `- line count drifted 1429.0%: C 124 live (of 3196) vs Rust 1897`.  This is P-86 ⑩ exactly: the check is no longer vacuous (the port has `fn`s now) and its line-count half compares the port against the C *live* baseline; the fix is the tolerance row (Owed 1), which is outside this lane's write scope.  No other problem is reported (no missing/extra item, no macro-body, no DEFUN/docstring finding). |

## Deviations

1. **The three C forward declarations (C 88, C 92-93, C 94-96) are dropped** and the definitions
   sit at their own C positions (PORTING.md 0.1-12/P-91).
2. **`gnutls_global_initialized` and `gnutls_free_func` are initialised** (`false`, null) where C
   left tentative definitions (PORTING.md 0.1-13).
3. **The `#define`s of C 460-587 are carried as comments**; every call site of a renamed entry
   point is spelled by its arm (`fn_<func> (...)` / the plain name).  Same for C 32-80's `#define`s
   (the features carry them) and C 582-587's `#undef`/`#define gnutls_free` pair.
4. **C's array subscript on a pointer** (C 931-932 `p->gnutls_certificates[i]`) is the port's
   `*((*p).gnutls_certificates.add (i as usize))`; C's `(void *) (intptr_t) proc->infd` is
   `(*proc_).infd as isize as *mut c_void`; C's `&emacs_gnutls_push` (a function address) is
   `Some (emacs_gnutls_push)` against the `Option<fn>` pointer type; C's `buf += rtnval` is
   `buf_.add (rtnval as usize)`.
5. **Bitfields are read through the mirror's masks** (PORTING.md 1.3): C's
   `proc->is_non_blocking_client` is `((*proc_).bits2 >> PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT) &
   1 != 0` at both sites (C 634, 720); the mirror models the whole unit as `bits1`/`bits2`.  The
   two gnutls `bool_bf` members (`gnutls_p`, `gnutls_complete_negotiation_p`) have no bit to name
   yet — they are part of Owed 4.
6. **`ckd_mul`/`ckd_add` (C 1089-1090) are the P-15 `overflowing_*` pair with the write-back**,
   in C's own short-circuit order (`femacs/src/casefiddle.rs` 315-320's shape).
7. **`emacs_gnutls_nonblock_errno` (C 670-691) is `unsafe extern "C" fn`** — C passes its address
   to `gnutls_transport_set_errno_function`, whose parameter type is a C function pointer
   (measured by the flip's first `E0308`).
8. **The third-party header's citations are spelled `<gnutls/gnutls.h> <line>`**, not
   `gnutls.h <line>`: `check-citations.py` resolves a bare basename against its `ROOTS`, where
   `emacs/src/gnutls.h` (the *reference* header, 100 lines) would capture every such cite and
   report 50 `range` findings against the wrong file.  The first pass did exactly that (51
   findings); the angle-bracket form is outside the cite pattern, so the gate counts them neither
   as external nor as checked — an entry in `## Owed` would be the alternative, but a *silent*
   wrong file is worse.  `crypto.h <line>` and `x509.h <line>` **are** external (no such file in
   the tree) and are counted as such.
9. **`errno.in.rs`, not `errno.rs`, is this unit's `<errno.h>`**: the two names the gated text
   needs beyond the system half (`ENOTCONN` 57, `ECONNRESET` 54) live in the template mirror's
   ungated fallback block, and `femacs/exec/mipsfpu.rs` 27 includes it for the same C line.  With
   `errno.rs` alone the flip reports `E0408` (`ENOTCONN` unbound) and `E0425` (`ECONNRESET`) —
   both measured before the change.
10. **`message` (lisp.h 4389), `memory_full` (lisp.h 4432) and `nconc2` (lisp.h 4271) are declared
    at this use site** (P-72 rule 4) because `include/lisp.rs` does not carry them yet;
    `femacs/src/process.rs` 1609, `femacs/src/image.rs` 203 and `femacs/src/font.rs` 381 declare
    them the same way.  Owed 4 moves them to the mirror.
11. **`GNUTLS_LOG`/`GNUTLS_LOG2` take a `literal` `$string`** (`concat!` does C's adjacent-literal
    concatenation at compile time); `GNUTLS_LOG2i` is not carried because the slice does not call
    it.
12. **The `allow` list gained `unused_mut` and the `clippy::all` group** for the new text; the
    frozen build's warning count went from 3 to 2 because the live `let mut capabilities` warning
    that the list now covers was already there.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. `docs/ports/line-drift-tolerance.tsv`'s `gnutls` row: rewrite from
   `0.1 … live` to the `total` basis with tolerance `0.41` (C 3196 vs the port's 1897), the
   P-86 ⑩ step.  **DONE (status 3.488, integrator):** the `total`/`0.41` row is appended
   (C 3196 total vs the port's **1901** lines; the old `live` row stays - append-only, newest
   governs), and `extract-abi --check emacs/src/gnutls.c femacs/src/gnutls.rs` is now
   **OK with zero `-` rows**.
2. `docs/baseline/port-lines-pending.tsv`: regenerate with
   `tools/check-port-lines.py --write-pending` — the `src/gnutls.c` row is reported `stale`
   ("no longer under the threshold").  **DONE (status 3.487/3.488, integrator):** the
   `--write-pending` run took the row off (61 rows with 3 stale -> **58 rows, 0 stale, 58 files
   under ten percent / 74,423 reference lines**), and the file now carries no `src/gnutls.c` row.
3. `docs/baseline/feature-coverage.tsv`: regenerate with `tools/feature-coverage.py --write`.
   The thirteen new feature names are `have_gnutls3`, `have_gnutls_x509_system_trust`,
   `have_gnutls_certificate_set_x509_key_file2`, `have_gnutls_cipher_get_iv_size`,
   `have_gnutls_cipher_get_tag_size`, `have_gnutls_digest_list`, `have_gnutls_mac_get_nonce_size`,
   `have_gnutls_aead`, `have_gnutls_etm_status`, `have_gnutls_ext_get_name`,
   `have_gnutls_compression_get`, `emsgsize`, `econnreset`.  `femacs/xtask/Cargo.toml`'s feature
   list (the file holds `have_gnutls`) is the same file the probe's `--cfg`s came from by hand.
   **DONE (status 3.488, integrator):** all thirteen names are registered in
   `femacs/xtask/Cargo.toml`'s `[features]` table (each row cites its `src/gnutls.c` line), and
   `tools/feature-coverage.py --write` has been re-run.
4. `femacs/src/include/process.rs`: carry `process.h` 194-216's `#ifdef HAVE_GNUTLS` member block —
   `gnutls_initstage` (`gnutls_initstage_t`), `gnutls_state`, `gnutls_x509_cred`
   (`gnutls_certificate_client_credentials`), `gnutls_anon_cred` (`gnutls_anon_client_credentials_t`),
   `gnutls_certificates` (`gnutls_x509_crt_t *`), `gnutls_certificates_length`,
   `gnutls_peer_verification`, `gnutls_extra_peer_verification`, `gnutls_log_level`,
   `gnutls_handshakes_tried`, the two `bool_bf` bits (`gnutls_p` 1, `gnutls_complete_negotiation_p`
   1) and the Lisp-side pair `gnutls_cred_type`/`gnutls_boot_parameters` (process.h 118-120) — plus
   the two bit shifts the unit's mask reads take.  This is the **whole** of Evidence ②'s 34 errors;
   the type names it needs (`gnutls_initstage_t`, `gnutls_certificate_client_credentials`) are
   declared by this unit under `have_gnutls`, so the mirror will need the same two aliases (P-90 ②:
   one surface, one home — the alias `gnutls_certificate_client_credentials` is `src/gnutls.h`'s
   neighbour, not the third-party header's, and belongs in `include/gnutls.rs`).
   `docs/ports/include-process.md` needs the matching rows.  **NOT DONE.**
5. `femacs/src/include/gnutls.rs` (the `src/gnutls.h` mirror, still the 26-line P-08 stub): carry
   `src/gnutls.h` 22-94's `#ifdef HAVE_GNUTLS` arm under `have_gnutls` — the three includes
   (`<gnutls/gnutls.h>`, `<gnutls/x509.h>`, `<gnutls/crypto.h>`) or a pointer to the unit's
   segment, `GNUTLS_EMACS_HANDSHAKES_LIMIT` (6000), the `gnutls_initstage_t` enum,
   `GNUTLS_EMACS_ERROR_NOT_LOADED`/`INVALID_TYPE`, `GNUTLS_INITSTAGE`, `GNUTLS_LOG`,
   `GNUTLS_LOG2`, `GNUTLS_LOG2i` and the five prototypes (C 82-94) — and then **delete the copies
   this unit declares at its use site** (P-88 ⑤/P-26's move-back, the `3.408→3.409` shape).
   `docs/ports/include-gnutls.md` needs the rows.  **NOT DONE.**
6. P-86 ① for the rest of the file: C 1131-2336, C 2337-2947 and the dead arms inside the two live
   functions (C 2965-3013, C 3027-3031, C 3036-3142) are still "recorded, not transcribed"; they
   are the next slices' text.  **NOT DONE.**
7. `docs/ports/macro-body-notes.tsv`: nothing new is needed *today* (`extract-abi --check` reports
   no macro-body problem for this pair); if a later slice calls `GNUTLS_LOG`/`GNUTLS_LOG2` with a
   non-literal string, the `literal` parameter is the thing to revisit.  **NOT NEEDED (recorded).**

## Not claimed

- **No run evidence.**  Nothing was linked, started or exercised: the flip produced no rlib (the
  mirror gap aborts it), and the stub reading (Evidence ④) is a *compile* of a modified copy in
  `/tmp`, not of the tree.
- **Not a GnuTLS build.**  The arm flip compiles against this unit's own declaration segment; the
  host header's numbers (Evidence ⑤) are what that segment is measured from, but no GnuTLS symbol
  was resolved and no `gnutls_*` call was made.  `gnutls_free`'s and the DLL loader's Windows
  behaviour is **text-only** (no Windows cross toolchain on this machine, P-86 ③⑵).
- **The Windows arm is not compiled.**  Its 102 `DEF_DLL_FN`/`LOAD_DLL_FN` pairs, the
  `init_gnutls_functions` body and the 102 renames have no compile reading at all.
- **The 34 E0609s are the mirror's, not a pass.**  The unit's own text is error-free *as measured
  by masking the one gap with a stub*; until Owed 4 lands, the arm flip cannot pass and the
  336-byte criterion of P-86 ⑧ stays inapplicable (the file has live text — Evidence ①).
- **Not the whole file.**  C 1131-3196 is not ported; `extract-abi --check` FAILs on the
  line-count baseline until Owed 1 lands; the ratchet files are owed (2, 3).
