# emacs/src/dbusbind.c (2190 lines)

> **P-86's class, with the same two readings sqlite.md records.** `configure.ac` puts `dbusbind.o`
> in the object list only under `AC_DEFINE([HAVE_DBUS])`, and `docs/baseline/frozen-build-sources.txt`
> lists no `dbusbind.o` for this target, so the whole body (C 21-2189) hangs on an `#ifdef` the
> frozen configuration leaves false (`docs/baseline/config/darwin-arm64.macros.tsv` 1259,
> `HAVE_DBUS<TAB>undef`).  `extract-abi.py`'s live checklist for the file is therefore **empty**
> (the five sections are vacuous as coverage evidence, P-86 ②), and this ledger carries the
> substitutes P-86 ②/③ ask for: the **C line-range table**, the `## Evidence` readings and the
> `## Deviations`.  Port: `src/dbusbind.rs` (**3005 lines**, 135,786 bytes), carrying **C
> 1-2190** - the reference's every line - with **both arms of every `#if`**.
>
> What the file implements when the feature is on: the D-Bus bindings - the two file-scope objects
> (`xd_registered_buses`, `xd_registered_fds`), the twenty-five `xd_*` helpers, the six subrs
> (`dbus--init-bus`, `dbus-get-unique-name`, `dbus-message-internal`, `dbus--fd-open`,
> `dbus--fd-close`, `dbus--registered-fds`), `init_dbusbind`, `syms_of_dbusbind_for_pdumper` and
> `syms_of_dbusbind` (with the nine `Vdbus_*` variables it registers).
>
> **`<dbus/dbus.h>` is not on this machine** (Evidence ②) and the reference tree has no copy, so
> its surface is declared **in the unit** at C 24's `#include` line (P-88 ①) and every item is
> **UNVERIFIED** - what *is* measured is the Rust side's own contract, a `clang` restatement of the
> two records and the scalars (Evidence ③).  **Nothing links `libdbus`**, no bus connection was
> opened, and no message was sent or received.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself, C 21's `#ifdef HAVE_DBUS` | `feature = "have_dbus"` - on every item **and on C 22-30's include graph** (P-86 ⑧) | `HAVE_DBUS` is defined |
| C 32-35's `#ifndef O_RDONLY` guard | **none**: the guard is false on this target (`O_RDONLY` is defined, macros.tsv; `check-cond-eval` reports 0 disagreements), so the reference takes no arm; the port takes the value from the `<fcntl.h>` mirror (Deviation 3) | - |
| C 37-39's `#ifndef DBUS_NUM_MESSAGE_TYPES` / `#define … 5` | `feature = "dbus_num_message_types"` (defined) and its negation (undefined); both arms spell 5 | either |
| C 42-47's `#ifdef interface` / `#undef interface` | **none**: Rust has no macro named `interface` to undo (Deviation 6) | - |
| C 104/2173's `#ifdef DBUS_DEBUG` | `feature = "dbus_debug"` | the reference's debug macros are compiled in |
| C 138's `#ifdef HAVE_DBUS_TYPE_IS_VALID` | `feature = "have_dbus_type_is_valid"` | the header has the predicate |
| C 142/195/222/421/707/929/1577/1855/2044/2061's `#ifdef DBUS_TYPE_UNIX_FD` | `feature = "dbus_type_unix_fd"` | the header defines the type code |
| C 321/338/346/354/362's `HAVE_DBUS_VALIDATE_*` | `have_dbus_validate_bus_name` / `_path` / `_interface` / `_member` (the disjunction spelled at each of the four consumers, C 323's shared body once) | the header has the validator |
| C 1051's `HAVE_DBUS_WATCH_GET_UNIX_FD` | `feature = "have_dbus_watch_get_unix_fd"` | the header has `dbus_watch_get_unix_fd` |
| C 1553's `HAVE_DBUS_MESSAGE_SET_ALLOW_INTERACTIVE_AUTHORIZATION` | `feature = "have_dbus_message_set_allow_interactive_authorization"` | the header has the setter |
| C 2083/2093's `DBUS_VERSION_STRING` / `DBUS_VERSION` | `feature = "dbus_version_string"` / `feature = "dbus_version"` (the two stand-in constants are UNVERIFIED) | the header defines each |
| C 1104's `#if 0` (C 1104-1111) | **none**: C's own dead text, recorded and not transcribed (Deviation 12) | - |

`have_dbus` is the tree's (`docs/baseline/feature-coverage.tsv` 138, first used by `keyboard.rs`
4768).  **Every other name above is new** and is Owed 1.

## C line ranges

The rows tile C 1-2190 **contiguously** (each opens on the line after the previous row's end), so
the union is the reference's every line including its blanks, and the last row reaches C 2190 -
what `tools/check-dead-file-coverage.py` reads.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the reference's opening licence/declaration block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-20 | C 19's `#include <config.h>` and C 20's blank | ungated (every unit includes it) |
| 21-25 | C 21's `#ifdef HAVE_DBUS`, C 22-24's `<stdio.h>`/`<stdlib.h>`/`<dbus/dbus.h>`, C 25's blank | **the file gate**; the `<dbus/dbus.h>` stand-in (P-88 ①, every item UNVERIFIED) |
| 26-31 | C 26-30's `lisp.h`, `termhooks.h`, `keyboard.h`, `pdumper.h`, `process.h` and C 31's blank | file gate; `coding.rs`/`string.rs` are included with them because C reaches `ENCODE_FILE`/`strlen` through `lisp.h` (P-72 rule 4) |
| 32-39 | C 32-35's `#ifndef O_RDONLY` + `<fcntl.h>` guard, C 36's blank, C 37-39's `#ifndef DBUS_NUM_MESSAGE_TYPES` pair | file gate; the fcntl mirror (Deviation 3); the two `dbus_num_message_types` arms |
| 40-47 | C 40-41's blanks and C 42-47's `interface` note + `#ifdef interface`/`#undef`/`#endif` | file gate; recorded, no arm (Deviation 6) |
| 48-59 | C 48-49's blanks, C 50-55's alist note, C 56's `xd_registered_buses`, C 57's blank, C 58's note, C 59's `xd_in_read_queued_messages` | file gate; C's two tentative definitions, zeroed (Deviation 7) |
| 60-89 | C 60-66's prefix note and blanks, C 67-73/75-81/83-89's `XD_SIGNAL1/2/3` | file gate |
| 90-133 | C 90-91's blank, C 92-98's `XD_ERROR`, C 99-103's blank + debug note, C 104-133's `#ifdef DBUS_DEBUG` pair | file gate; `dbus_debug`'s two arms; the explicit `__func__` argument (Deviation 8) |
| 134-172 | C 134-137's blank + `XD_BASIC_DBUS_TYPE` note, C 138-172's three arms | `have_dbus_type_is_valid` × `dbus_type_unix_fd` |
| 173-204 | C 173-178's blanks + the two notes, C 179-204's `xd_symbol_to_dbus_type` | file gate; C 195-197 gated by `dbus_type_unix_fd` (Deviation 5) |
| 205-231 | C 205-206's note + `static Lisp_Object`, C 207-231's `xd_dbus_type_to_symbol` | same (C 222-224) |
| 232-259 | `XD_KEYWORDP`, `XD_DBUS_TYPE_P`, the `XD_OBJECT_TO_DBUS_TYPE` note + macro, `XD_NEXT_VALUE` | file gate |
| 260-281 | C 260-274's `XD_MESSAGE_TYPE_TO_STRING` (note included), C 275-281's note + `XD_OBJECT_TO_STRING` | file gate |
| 282-336 | C 282-283's blank, C 284-319's `XD_DBUS_VALIDATE_BUS_ADDRESS`, C 320-336's `TODO` + shared `XD_DBUS_VALIDATE_OBJECT` | file gate; the disjunction of the four `HAVE_DBUS_VALIDATE_*` names |
| 337-368 | C 338-368's four `XD_DBUS_VALIDATE_*` pairs (both arms of each) | the four features, each arm gated |
| 369-380 | C 369-371's blank + note, C 372-380's `xd_signature_cat` | file gate |
| 381-575 | C 381-390's note + `xd_signature`'s declarator, C 391-575's body | file gate; C 421-423 gated by `dbus_type_unix_fd` (Deviation 5) |
| 576-603 | C 576-578's note + declarator, C 579-603's `xd_extract_signed` | file gate |
| 604-634 | C 604-606's note + declarator, C 607-634's `xd_extract_unsigned` | file gate |
| 635-872 | C 635-638's note + declarator, C 639-872's `xd_append_arg` | file gate; C 707-709 gated by `dbus_type_unix_fd` |
| 873-1002 | C 873-875's note + declarator, C 876-1002's `xd_retrieve_arg` | file gate; C 929-931 gated by `dbus_type_unix_fd` |
| 1003-1016 | C 1003-1005's note + declarator, C 1006-1016's `xd_get_connection_references` | file gate; C's two reinterpreting assignments (Deviation 11) |
| 1017-1021 | C 1017-1019's note + declarator, C 1020-1021's `xd_lisp_dbus_to_dbus` | file gate |
| 1022-1047 | C 1022-1028's note + declarator, C 1029-1047's `xd_get_connection_address` | file gate |
| 1048-1063 | C 1048-1050's note + `TODO`, C 1051-1060's `xd_find_watch_fd` with its `#if HAVE_DBUS_WATCH_GET_UNIX_FD` pair, C 1061-1063's blank + `/* Prototype.  */` and the forward declaration | `have_dbus_watch_get_unix_fd`; the prototype has no Rust counterpart (Deviation 4) |
| 1064-1090 | C 1064-1067's note + declarator, C 1068-1090's `xd_add_watch` | file gate; `unsafe extern "C" fn` (its address is taken, Deviation 10) |
| 1091-1118 | C 1091-1093's note + declarator, C 1094-1118's `xd_remove_watch` with C 1104-1111's `#if 0` | file gate; the `#if 0` recorded, not transcribed (Deviation 12) |
| 1119-1128 | C 1119-1121's note + declarator, C 1122-1128's `xd_toggle_watch` | file gate |
| 1129-1167 | C 1129-1131's note + declarator, C 1132-1167's `xd_close_bus` | file gate |
| 1168-1293 | C 1168's blank and the `DEFUN ("dbus--init-bus", …)` (C 1169-1293) | file gate (the `DEFUN!` invocation carries it, or `DEFUN!` would be out of scope in the frozen configuration) |
| 1294-1316 | C 1294's blank and the `DEFUN ("dbus-get-unique-name", …)` | file gate |
| 1317-1666 | C 1317's blank and the `DEFUN ("dbus-message-internal", …)` (4, MANY) | file gate; C 1553-1558 gated by `have_dbus_message_set_allow_interactive_authorization`, C 1577-1581 by `dbus_type_unix_fd` |
| 1667-1693 | C 1667's blank, C 1668-1669's fd-alist note + `xd_registered_fds`, C 1670-1693's `DEFUN ("dbus--fd-open", …)` | file gate; the object zeroed (Deviation 7) |
| 1694-1714 | C 1694's blank and the `DEFUN ("dbus--fd-close", …)` | file gate; `assoc_no_quit` declared in-unit (Owed 2) |
| 1715-1739 | C 1715's blank and the `DEFUN ("dbus--registered-fds", …)` (0, 0) | file gate |
| 1740-1760 | C 1740's blank and `xd_store_event` (C 1741-1760) | file gate; the `input_event_bits!` macro (PORTING.md 1.3) and `DBUS_EVENT` (a `termhooks.h` 249 member the frozen mirror does not carry, Owed 3) |
| 1761-1951 | C 1761-1763's note + declarator, C 1764-1951's `xd_read_message_1` | file gate; C 1855-1869 gated by `dbus_type_unix_fd`; C 1846/1852/1880/1943's `goto`s are the two labelled blocks (Deviation 1) |
| 1952-1967 | C 1952-1954's note + declarator, C 1955-1967's `xd_read_message` | file gate; `unsafe extern "C" fn` (passed to `internal_catch`, Deviation 10) |
| 1968-1995 | C 1968-1970's note + declarator, C 1971-1995's `xd_read_queued_messages` | file gate; `unsafe extern "C" fn` (passed to `add_read_fd`/`add_write_fd`) |
| 1996-2009 | C 1996-1997's blank + `void`, C 1998-2002's `init_dbusbind`, C 2003-2009's `syms_of_dbusbind_for_pdumper` | file gate; `xputenv` declared in-unit (Owed 2); the pdumper hook is `unsafe extern "C" fn` |
| 2010-2188 | C 2010's blank and `syms_of_dbusbind` (C 2011-2188), plus the nine `Vdbus_*` definitions the unit owns | file gate; C 2044-2046/2061-2063 gated by `dbus_type_unix_fd`, C 2083-2087 by `dbus_version_string`, C 2093-2100 by `dbus_version`, C 2173-2180 by `dbus_debug` |
| 2189-2190 | C 2189's blank and C 2190's `#endif /* HAVE_DBUS */` | the gate closes here, as in C |

## Evidence

**① The frozen configuration (`build-tu`), twice.**
`cd femacs && CARGO_TARGET_DIR=/tmp/dbus-lane/cargo-target cargo run -q -p xtask -- build-tu
src/dbusbind.c` → `built src/dbusbind.rs -> ../build/femacs/dbusbind.o`, **exit 0**, and
`build/femacs/dbusbind.o` is **336 bytes** (md5 `dbed68837694579185377dbb0821ab62` on the final
revision) - the P-86 ⑧ empty object, i.e. every item of the port sits behind `have_dbus` and
rustc compiles only `include/config.rs`.  That is the expected reading, not evidence about the
body.

**② `<dbus/dbus.h>` is not on this machine.**  `ls -d /opt/homebrew/include/dbus*
/usr/local/include/dbus* /usr/include/dbus*` is empty, `find /opt/homebrew /usr/local
/Library/Developer -maxdepth 6 -name 'dbus.h' -o -name 'dbus-types.h'` prints nothing, and the
cargo registry has no `dbus*` source directory (`ls -d ~/.cargo/registry/src/*/dbus*` is empty).
`which -a clang` is `/usr/bin/clang` (Apple clang 21.0.0.0.0), which is what measured ③.

**③ The stand-in's own contract, measured with `clang` (P-88 ②).**
`clang -o /tmp/dbus-lane/restate /tmp/dbus-lane/restate.c && /tmp/dbus-lane/restate` on a
restatement of the declarations the unit carries prints:
`dbus_bool_t 4 dbus_int16_t 2 dbus_uint16_t 2 dbus_int32_t 4`;
`dbus_uint32_t 4 dbus_int64_t 8 dbus_uint64_t 8`;
`DBusMessageIter sizeof 72 align 8 dummy1 0 dummy2 8 dummy3 16 dummy4 20 dummy11 48 pad1 52 pad2 56
pad3 64`; `DBusError sizeof 32 align 8 name 0 message 8 padding1 24`; and the two type codes
`'y' == 121`, `'h' == 104`.  Every number the unit's comments state is that output; it proves the
`#[repr (C)]` records match the shape *stated in the unit* and nothing about the real header.
`PRIdMAX`/`PRIuMAX` are the same class of measurement: `printf "#include <inttypes.h>…" | clang -x
c - -o /tmp/prisel && /tmp/prisel` prints `jd ju`.  The harness lived in the lane's own
`/tmp/dbus-lane`, which the brief has the lane remove at its close (`rm -rf`), so the restatement
is not kept: it is the four typedef blocks (`dbus_bool_t`/`dbus_int16_t`/`dbus_uint16_t`/
`dbus_int32_t`/`dbus_uint32_t`/`dbus_int64_t`/`dbus_uint64_t`, `DBusMessageIter`, `DBusError`)
exactly as the unit's comments spell them, plus the two `printf`s quoted above.

**④ The arm flip (P-86 ③⑴, the file gate plus every sub-gate).**
`cargo run -q -p xtask -- probe src/dbusbind.rs /tmp/dbus-lane/x.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_dbus"' …` with **all thirteen sub-features set**
(`dbus_debug`, `have_dbus_type_is_valid`, `dbus_type_unix_fd`, the four
`have_dbus_validate_*`, `have_dbus_watch_get_unix_fd`,
`have_dbus_message_set_allow_interactive_authorization`, `dbus_version_string`, `dbus_version`,
`dbus_num_message_types`) → **exit 1, 10 errors, all 10 in mirror copies**: 8 × `E0425` in
`include/buffer.rs` (`STRING_CHAR` ×2 at 1498/1512, `UNIBYTE_TO_CHAR` ×2 at 1524/1593,
`string_char_and_length` ×2 at 1921/1947, `BYTES_BY_CHAR_HEAD` at 1965, `raw_prev_char_len` at
1985 - all names `include/character.rs` owns and `buffer.rs` does not include, P-72 rule 4) and
2 × `E0425` for `Emacs_Color` at `include/termhooks.rs` 248/251.  **0 are in the unit's own
text.**  `include/buffer.rs` is reached through C 28's own `keyboard.h` mirror (`keyboard.rs` 32),
so the ten are inside the reference's own five headers' mirrors.  Per P-86 ⑦ that raw count is not
by itself proof that the unit's text was reached, so:

**⑤ The mutation reading (P-86 ⑦/⑧ - the flip is sensitive).**
Copying the materialized `build/probes/dbusbind/_root.rs` and injecting a deliberate type error
into the unit's own text (`let _mutation: c_int = signature;`, inside `xd_signature_cat`) changes
the reading: with `rustc` (the `rust-toolchain.toml` nightly, `--edition 2024 -O
--crate-type=lib -C panic=abort` and the same thirteen `--cfg`s) the unmutated copy reports
`10 × E0425` and the mutated one reports the **same 10 plus 1 × E0308 pointing at the injected
line**.  The unit's own text is therefore reached and type-checked, not silently skipped.
(Reading the same root at the *repository root* - where `rustc` is stable 1.98.1 rather than the
pinned nightly - adds one mirror-side `E0658` for `include/lisp.rs` 3761's `core::ffi::VaList`;
the difference is the toolchain the proxy picks, and it is in the mirror.)

**⑥ `extract-abi.py --check` reads OK, and why that is not coverage evidence.**
`python3 tools/extract-abi.py --check emacs/src/dbusbind.c src/dbusbind.rs` →
`OK emacs/src/dbusbind.c -> src/dbusbind.rs`, with **no `- ` row at all**.  The checklist
is empty (P-86 ②), and the line-count rule passes through the existing tolerance row
(`docs/ports/line-drift-tolerance.tsv` 78, `0.99`); the token comparison is **vacuous for this
file**, which a mutation proves: renaming `make_mint_ptr` in the port and re-running the check
prints the *same* output plus only the line drift of the temporary path.  `check-dead-file-coverage.py`
is the gate that reads the coverage instead (Evidence ⑦).

**⑦ The other gates (repo root, final revision).**
`python3 tools/check-cond-eval.py src/dbusbind.c` →
`src/dbusbind.c: 27 conditions, 0 unknown, 0 disagreements [VERIFIED]` —
`python3 tools/check-dead-file-coverage.py` → (see Owed 5's line: the table above is the reading) —
`check-declarations.py`, `check-citations.py`, `check-port-lines.py`,
`check-ledger-ticks.py` and `check-dead-file-coverage.py` read clean for this file, and
`check-mirror-dupes.py` reports nothing for it: the exact numbers and the two readings that are
*not* this file's are Owed 5 and Owed 6.

## Deviations

1. **`goto cleanup` / `goto monitor` are two labelled blocks** (C 1846, 1852, 1880, 1943;
   PORTING.md 2.2).  `break 'cleanup` is `goto cleanup`, `break 'first` is `goto monitor`, and the
   `'first` block's end is C's fall-through into `monitor:`; C's labels are kept as comments
   (`/* monitor:  */`, `/* cleanup:  */`) and no `return` is introduced.  The `goto` token itself
   has no Rust counterpart, and this file's token comparison is vacuous (Evidence ⑥), so no
   `docs/ports/macro-body-notes.tsv` row is owed - the row would claim the check needs an
   exemption it does not ask for.
2. **C's `switch`es are `match`es** with C's case order; `DBUS_TYPE_BYTE | DBUS_TYPE_UINT16` and
   `DBUS_TYPE_STRUCT | DBUS_TYPE_DICT_ENTRY` keep their shared bodies, and the two `switch`es
   that have no `default` in C get an empty `_ =>` arm (Deviation 10's reading).
3. **C 32-35's `#ifndef O_RDONLY` guard is dead on this target** and carries no gate: `O_RDONLY`
   is defined (`darwin-arm64.macros.tsv`; `check-cond-eval` reports 0 disagreements over the file's
   27 conditions), C reaches it through `config.h`'s system headers, and no mirror in this unit's
   include graph declares it - so the port takes the value from the `<fcntl.h>` mirror
   (`include!("../lib/include/fcntl.in.rs")`), the same thing `src/dired.rs` 60 does at its C 31.
   The `#ifndef` is recorded as this deviation (the `filelock.rs` `ELOOP` shape).
4. **C 1062-1063's forward declaration is dropped** (PORTING.md 0.1-12): `xd_read_queued_messages`
   is defined at its C position (C 1971) and Rust's items are order-independent, so C's `/*
   Prototype.  */` note stays and the declaration line does not.
5. **A `#[cfg]` cannot sit on one branch of an or-pattern**, so C's shared case groups that have a
   gated member (`case DBUS_TYPE_UINT32: case DBUS_TYPE_UINT64: #ifdef DBUS_TYPE_UNIX_FD case
   DBUS_TYPE_UNIX_FD: #endif case DBUS_TYPE_INT32: …`, and the same shape in `xd_append_arg` and
   `xd_retrieve_arg`) are one arm with C's body plus a `#[cfg (feature = "dbus_type_unix_fd")]` arm
   with the *same* body.  The two `else if` chains with a gated member
   (`xd_symbol_to_dbus_type`, `xd_dbus_type_to_symbol`) are one `if … return` per member, in C's
   order and with C's first-match semantics, because no attribute can sit on an `else if` arm.
6. **C 42-47's `#ifdef interface` / `#undef interface`** guards a macro some platform headers
   define; Rust has no macro named `interface` to undo, so the pair is recorded as a comment and
   carries no arm.
7. **C's tentative definitions are zero objects.**  `xd_registered_buses` (C 56),
   `xd_registered_fds` (C 1668) and the nine `Vdbus_*` are C's all-zero objects (`globals.h`'s
   `Lisp_Object` fields), so the port's initialisers are `lisp_h_Qnil ()` for the Lisp objects and
   `false` for `xd_in_read_queued_messages` (C 59) - the value `syms_of_dbusbind_for_pdumper`
   (C 2006) stores anyway.  C's `static` keeps internal linkage (`PORTING.md` 0.2), so those items
   are not `pub` and carry no `#[unsafe(no_mangle)]`.
8. **`__func__` becomes an explicit macro argument** (Deviation tied to C 105-132): a Rust
   `macro_rules!` body cannot see a local of its expansion site (macro hygiene), so
   `XD_DEBUG_MESSAGE`/`XD_DEBUG_VALID_LISP_OBJECT_P` take the function name as their first
   parameter and every call site passes the `__func__` its function defines; C's implicit
   preprocessor capture is the one thing this file cannot spell.  The same reason makes the two
   bodies that C's validate macro reassign (`bus` in `Fdbus_get_unique_name`, since the `DEFUN!`
   expansion's binding is not `mut`) rebind their parameter with `let mut bus = bus;`.
   `__LINE__` is `line! ()`.
9. **Uninitialised locals carry PORTING.md 1.5's zero initialiser** (named here, not discovered by
   the compiler): `DBusError derror` (C 1175, 1310), `DBusMessageIter iter` (C 1329, 1775, and the
   two `subiter`s at C 639/908), `char signature[DBUS_MAXIMUM_SIGNATURE_LENGTH]` (C 639, 1334) and
   `char x[…]` inside `xd_signature` (C 393/443).
10. **The functions C takes the address of are `unsafe extern "C" fn`**: `xd_add_watch`,
    `xd_remove_watch`, `xd_toggle_watch` (passed to `dbus_connection_set_watch_functions`),
    `xd_read_queued_messages` (passed to `add_read_fd`/`add_write_fd`), `xd_read_message` (passed
    to `internal_catch`) and `syms_of_dbusbind_for_pdumper` (the pdumper hook).  A Rust `fn` item
    is the Rust ABI and does not unify with `Option<unsafe extern "C" fn …>` (the `androidmenu.rs`
    / `callproc.rs` shape); `PORTING.md` 0.1-6 also wants each one defined once, which it is.
11. **`xd_get_connection_references` keeps C's two reinterpreting assignments** (C 1012-1014) -
    C's own note says the library's record is not public, so the port spells C's casts
    (`&raw mut connection as *mut ptrdiff_t`, then `*refcount as *mut ptrdiff_t`) instead of
    inventing a layout.
12. **C 1104-1111's `#if 0` is recorded, not transcribed** (the `alloc.rs`/`android.rs` reading):
    it is C's own dead text (`unsetenv` in the unset path), so its `unsetenv` call and the
    `XSYMBOL` comparisons after it exist in no configuration and add nothing to the port.
13. **`%"pD"d` is written `%td`** (lisp.h 150's `pD` is `"t"`, the `intervals.rs`/`region-cache.rs`
    precedent), and `PRIdMAX`/`PRIuMAX` are the unit's own `macro_rules!` resolving to the measured
    `"jd"`/`"ju"` so the format text is `concat!`-built exactly where C concatenates.
14. **The nine `Vdbus_*` objects are defined in this unit** (the `buffer.rs` 4919 shape,
    `docs/status.md` 3.29): C names a field of the single `globals` object, whose declaration the
    port mirrors in `include/globals.rs`'s `variables` module; the unit whose `syms_of_dbusbind`
    registers them is where the definitions live, and their addresses must be stable for
    `staticref`/the forwarding records.

## Owed (nothing here is claimed as done)

1. **Twelve feature names are new** (`docs/baseline/feature-coverage.tsv` 138 has `have_dbus`
   alone): `dbus_debug`, `have_dbus_type_is_valid`, `dbus_type_unix_fd`,
   `have_dbus_validate_bus_name`, `have_dbus_validate_path`, `have_dbus_validate_interface`,
   `have_dbus_validate_member`, `have_dbus_watch_get_unix_fd`,
   `have_dbus_message_set_allow_interactive_authorization`, `dbus_version_string`, `dbus_version`,
   `dbus_num_message_types`.  Until they are registered the arm flip needs each `--cfg` by hand
   (Evidence ④), and `feature-coverage.py --write` will list them as enabled by no configuration.
   **NOT DONE (outside this lane's write set).**
2. **Mirror gaps the unit declares at the use site** (P-72 rule 4): `message` (lisp.h 4389),
   `internal_catch` (lisp.h 4803), `assoc_no_quit` (lisp.h 4274), `memq_no_quit` (lisp.h 4286),
   `FUNCTIONP` (lisp.h 4799), `make_formatted_string` (lisp.h 4542) and `xputenv` (lisp.h 5579) -
   each with C's own prototype in a `/// C:` comment.  `internal_catch`'s and `xputenv`'s
   *definitions* are in `eval.rs` and `alloc.rs`; the other five are not defined anywhere in the
   port yet.  **NOT DONE (a mirror round).**
3. **`termhooks.rs` does not carry `DBUS_EVENT`** (termhooks.h 249, the member `HAVE_DBUS` adds to
   `enum event_kind`), so the unit declares it (`pub const DBUS_EVENT: event_kind = 27`) with the
   value the `have_dbus`/no-`THREADS_ENABLED` configuration gives.  The mirror's numbering stops at
   the frozen configuration's members, so a `have_dbus` build also needs the mirror's own values
   after `SAVE_SESSION_EVENT` shifted by one - a `have_dbus` build of `keyboard.rs` would read
   `CONFIG_CHANGED_EVENT` and `DBUS_EVENT` as the same number until that is done.  **NOT DONE.**
4. **The ten mirror errors the arm flip reports** (Evidence ④: `include/buffer.rs`'s five
   `character.rs` names, `include/termhooks.rs`'s `Emacs_Color`) are **named, not fixed** here:
   they are in mirrors this lane does not own and were reported by the flip exactly as P-86 ⑦
   requires.  **NOT DONE (mirror round).**
5. **The other gates, on the final revision** (repo root): `check-dead-file-coverage.py` →
   `info docs/ports/dbusbind.md: 43 range row(s) cover src/dbusbind.c (2190 lines; 0 blank line(s)
   between blocks need no row)`, **94 ledger(s) bound, 0 failure(s)**;
   `check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`
   (the allowed literals are the reference's own docstrings, PORTING.md 0.1-19's `literal` class);
   `check-citations.py` → `676 file(s) scanned, 5 drift, 1 range, 1189 external, 30 allowed, 5
   stale`, **none of them in `dbusbind.rs`/`dbusbind.md`** (the one drift this file had - a
   citation line whose first backticked snippet sat three lines from the cited `termhooks.h` 249 -
   was fixed in the unit, not waived); `check-ledger-ticks.py` → `232 ledger(s) bound …, 0
   failure(s)`; `check-port-lines.py` → `33 file(s) under ten percent, 50,069 reference line(s); 37
   ratchet row(s), 0 new, 4 stale`, with **`stale src/dbusbind.c: no longer under the threshold`**
   - this file's exit from the P-86 ratchet, which the ratchet file (`docs/baseline/port-lines-pending.tsv`,
   outside this lane's write set) should record with `--write-pending`.  **PARTLY DONE: the three
   gate readings are clean for this file; the ratchet row and the tolerance row's reason text are
   the integrator's.**  The tolerance row's *text* (`line-drift-tolerance.tsv` 78's "an empty
   unit", `0.99`) still describes the 12-line port: the drift is now 37.2% (C 2190 vs Rust 3005),
   which the existing 0.99 tolerance carries, so no new row is *required*, but the row's reason
   should be re-derived.  **NOT DONE (tolerance and ratchet files are outside this lane's write
   set).**
6. **`check-mirror-dupes.py` does not pass, and none of it is this file.**  Its final reading is
   `34 duplicate declaration(s), 17 ratchet row(s), 17 failure(s)`; the 17 are in
   `src/keyboard.rs` (7), `src/coding.rs` (4), `src/tparam.rs` (2),
   `src/xfont.rs` (2), `src/doprnt.rs` (1) and `src/ftfont.rs` (1) - in-unit
   declarations (`memcpy`/`strcpy`/`strncmp`/`memmove`/`memset`/`stpcpy`/`strcmp`, `font_add_log`,
   `x_mark_frame_dirty`) that their include graphs now also reach through
   `lib/include/string.rs`, `include/font.rs` and `include/xterm.rs`.  **`dbusbind.rs` has
   no finding**, and those files are untouched here (they are not in this lane's working-tree
   diff); the working tree also carries another lane's in-flight `src/include/lisp.rs` edit,
   so the number is that lane's/HEAD's reading rather than this file's.  **NOT DONE (not this
   lane's files).**
7. **`docs/ports/dbusbind.md`'s own numbers are not measured by any gate**: the `## C line ranges`
   table and Evidence ⑤ are what `check-dead-file-coverage.py` and the flip read.

## Not claimed

- **Zero run evidence.**  `x.rlib` was never produced: the arm flip *failed to compile* (exit 1),
  so the reading is a name-and-type reading of the unit's text plus the mirror set (P-86 ⑦'s
  weaker reading).  No bus was connected, no message sent, received, appended or retrieved, and no
  Lisp call made.
- **Not a `libdbus` build.**  The flip compiles against this unit's own `<dbus/dbus.h>` stand-in
  (every item UNVERIFIED, Evidence ③), not against the real header, and nothing links the library.
- **The frozen object's reading is the gate's, not the body's.**  336 bytes is byte-identical to
  the empty-unit port's object, which is the point (the whole body is inside the gate) - it says
  nothing about the body.
- **`extract-abi --check` does not compare this file's bodies** (Evidence ⑥: the live checklist is
  empty).  The port's token-level fidelity is therefore *not* machine-checked here; what is
  checked is the flip (④/⑤), the coverage table (⑦) and the other gates.
- **`have_dbus` is not a configuration this tree can build.**  Even with the twelve features set,
  the mirrors' own gaps (Owed 4) stop the flip; no claim is made that the reference's D-Bus
  bindings would work.
- **The `<dbus/dbus.h>` surface is a stand-in**: its constants, function signatures and the two
  records are written from the library's published interface; only the Rust side's own shape is
  measured (③).  A machine with `libdbus` should re-measure and retire the section into
  `src/include/dbus.rs` (P-26/P-70's move-back).
