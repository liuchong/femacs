# emacs/src/w32image.c (717 lines) — whole file, C 1-717

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32image.o` is added to
> `W32_OBJ` by `configure.ac` 3051 — the non-Cygwin arm of the Cygwin split inside the `HAVE_W32`
> test (3019-3030, whose 3024 `AC_DEFINE`s `HAVE_NTGUI`) — and `docs/baseline/frozen-build-sources.txt`
> lists no `w32image.o` for this target, so `tools/extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this ledger is that clause's substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `src/w32image.rs` (**1476 lines**,
> md5 `c042f4627985fa246d7ada096bdf6fd1`, sha256
> `10600c22c38cf619550bb7afe09a1b264855d977e09ad022b1514bb0bf9a19a8`), carrying **C 1-717 with both
> arms of every `#if`**.  The reference `emacs/src/w32image.c` is md5
> `7ecfc366f8dad6ee72804923ad1e6e9a` (unchanged; `emacs/` is read-only) and its `docs/port-map.md`
> 576 row pairs `src/w32image.c` 717 with `src/w32image.rs` (the port's own count in that
> table is 13, the inherited stub's — the integrator's regeneration, Owed 7).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w32image.md` (and the
> 13-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object)
> is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32inevt.md`, `docs/ports/w32console.md`, `docs/ports/w32dwrite.md` and
> `docs/ports/w32term.md` record for their own pairs.
>
> **The file gate is `windowsnt`, and the arm set is `windowsnt` + `have_ntgui` +
> `have_window_system`.**  `HAVE_NTGUI` (3024) is on for *both* the Cygwin and the native Windows GUI
> build, while `w32image.o` joins `W32_OBJ` only in the non-Cygwin arm (3051) — the build whose
> `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The units of that same arm already landed with
> `windowsnt` as their file gate (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`,
> `src/w32console.rs`, `src/w32inevt.rs`), so this one does too; `have_ntgui` and
> `have_window_system` are set in the arm reading because the mirrors this unit reaches
> (`w32term.rs`, `w32gui.rs`, `frame.rs`, `dispextern.rs`) spell their platform arms with them.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`gdiplus.h`/`shlwapi.h`), so there is **no
> `<windows.h>` and no GDI+ header to compile or measure against**, no arm flip on a Windows target
> and no layout measurement: every record, value and prototype of the two stand-in sections is
> `UNVERIFIED` (the documented SDK text, not a measurement) and the layout assertions are owed
> (Owed 4).  What the round does carry is the frozen `build-tu` reading, the machine-checked range
> table, the `#if` verdicts, a per-item C citation, the per-file error census of the `windowsnt` +
> `have_ntgui` + `have_window_system` arm, and a **mutation test** over this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3051; the native-Windows `else` of the Cygwin split) | `feature = "windowsnt"` (on every item *and* on C 22-41's include graph) | the native MS Windows GUI build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not the Cygwin GUI build |
| C 26 `#ifdef MINGW_W64` / C 28 `# include <objidl.h>` / C 29 `#endif` | no feature of its own: `objidl.h` has a mirror nowhere in the tree, so the edge carries no file and both arms need the same names (the stand-in section below).  `MINGW_W64` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 3914, so C's `#else` side is the frozen one | both MinGW families |
| C 35 `#ifdef WINDOWSNT` / C 36 `# include "w32.h"` / C 37 `#endif` | the file gate (the arm *is* `WINDOWSNT`; the `src/w32console.rs` reading of its own C 42-44) | the native MS Windows GUI build |
| C 42 `#ifdef WINDOWSNT` … C 207 `#endif /* WINDOWSNT */` (the 28 loaded entry-point objects and `gdiplus_init`) | the file gate | the native MS Windows GUI build |
| C 224 `#ifdef WINDOWSNT` / C 227 `#else` / C 229 `#endif` (inside `w32_gdiplus_startup`) | the file gate; C 227-229 is the arm C's `#else` spells and is carried as C's three lines | the native MS Windows GUI build |

`tools/check-cond-eval.py src/w32image.c` reads **`src/w32image.c: 4 conditions, 0 unknown, 0
disagreements [VERIFIED]`** — C 26, C 35, C 42 and C 224, i.e. exactly the five rows above and
nothing else, so the file's whole conditional text is accounted for.  The frozen table has
`MINGW_W64` and `WINDOWSNT` `undef`, which is why the frozen configuration compiles none of this
text (Evidence ①) and why the arm reading is the native Windows GUI build (Evidence ②).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19); the port's first line is the revision line |
| 20 | the file's own second comment (its one author) | carried as C's words |
| 22-24 | `#include <config.h>`, `"lisp.h"`, `"dispextern.h"` | file gate; `include/{config,lisp,dispextern}.rs` |
| 25 | `#define COBJMACROS` | file gate; the unit-typed `const COBJMACROS` (the `src/w32fns.rs` 95-97 reading) |
| 26-29 | `#ifdef MINGW_W64` / `<objidl.h>` / `#endif` | file gate; no file edge, C's four lines kept; the names stand in the `<objidl.h>` section below |
| 30-32 | `<wtypes.h>`, `<gdiplus.h>`, `<shlwapi.h>` | file gate; the in-place stand-in sections below (P-88 ①, UNVERIFIED) |
| 33-34 | `"w32common.h"`, `"w32term.h"` | file gate; the two mirrors |
| 35-37 | `#ifdef WINDOWSNT` / `"w32.h"` / `#endif` | file gate; `include/w32.rs` (`map_w32_filename`, `filename_to_utf16`) |
| 38-39 | `"frame.h"`, `"coding.h"` | file gate; the two mirrors (`ENCODE_FILE`) |
| 41 | `#include "w32gdiplus.h"` | file gate; `include/w32gdiplus.rs` (its `Gdip*` `_Proc` typedefs, the `fn_*` objects and the 27 `#undef`/`#define` macro pairs) |
| 42-207 | `#ifdef WINDOWSNT`: the 28 loaded entry-point objects (C 43-70) and `gdiplus_init` (C 72-205) | file gate; C 70 repeats C 58's object, written once |
| 209-213 | the five file-scope objects (`gdip_initialized`, `gdiplus_started`, `token`, `input`, `output`) | file gate; `input`/`output` live in `mod gdiplus_state` (Deviations 9) |
| 216-243 | `w32_gdiplus_startup`, with C 224-229's `#ifdef WINDOWSNT`/`#else` pair | file gate; C 225-226 is code, C 227-229 is C's three lines |
| 245-252 | `w32_gdiplus_shutdown` | file gate |
| 254-271 | `w32_can_use_native_image_api` | file gate; C's parameter name `type` is `r#type` |
| 273-282 | `enum PropertyItem_type` | file gate; a `type` plus constants (Deviations 5) |
| 284-315 | `decode_delay` | file gate; C 291-312's `switch` is a `match` (Deviations 6) |
| 317-351 | `w32_frame_delay` | file gate |
| 353-387 | `w32_select_active_frame` | file gate |
| 389-409 | `w32_image_bg_color` | file gate; C 398-400's ternary is the port's `if (if …)` |
| 411-485 | `w32_load_image` | file gate; C 425's local `fn` is named `filename` (Deviations 10) |
| 487-511 | `cached_encoder`, `last_encoder`, `thumb_type_data`, `thumb_types[]` | file gate; C's six `L"…"` literals are `[u16; N]` arrays (Deviations 12) |
| 514-566 | `w32_gdip_get_encoder_clsid` | file gate; C 527/549's `for` loops are `while`s (Deviations 11) |
| 568-641 | `w32_gdip_export_frame` | file gate |
| 643-706 | `Fw32image_create_thumbnail` (the file's one `DEFUN`) | file gate; the `DEFUN!` macro, docstring carried |
| 708-712 | `syms_of_w32image` | file gate |
| 714-717 | `globals_of_w32image` | file gate (empty body in C as well) |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && cargo run -q -p xtask -- build-tu src/w32image.c` → `built src/w32image.rs ->
../build/femacs/w32image.o`, **exit 0**, and `build/femacs/w32image.o` is **336 bytes** (md5
`dbed68837694579185377dbb0821ab62`, the same bytes every run since the first draft) on the final
revision.  That is the expected reading, not a proof of the new text: every item of the port sits
behind `windowsnt`, so under the frozen feature set rustc compiles only the `#![allow]` attribute
and the file's comments — P-86 ⑧'s criterion.  (**Four `build-tu` runs, against a budget of three —
over budget, stated**: the first is the inherited stub's 336-byte baseline, the second failed to
compile (a `*/` inside the C 1-18 comment's own text closed the Rust block comment early —
`error: unknown start of token: \``), the third was the first complete reading, and the fourth is
the final revision's: `configure.ac`'s arm line was corrected in the header note (3051, not 3062)
after run three, which is a comment edit, and the object's bytes did not move.)

**② The arm flip.**  `cargo run -q -p xtask -- probe src/w32image.rs /tmp/w32img-lane/arm4.o
--crate-type lib --emit obj -C panic=abort --cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"'
--cfg 'feature="have_window_system"'` → **exit 1, 35 `error` lines / 33 error headers, 0 of them
anchored in the unit's own text** (`_root.rs`), **29 warnings**, of which 6 are this unit's own
`unused doc comment` lint sites (the `///` C citations that head an `unsafe extern` block; the
mirrors emit the same lint 22 times, and warnings gate nothing).  The reading was taken twice — runs
three and four, before and after the comment-only header correction of ① — with identical numbers,
and the four errors are: 29 are in `include/w32gdiplus.rs` (the 27 `fn_*`
objects and the two functions `w32_gdip_get_encoder_clsid`/`w32_gdip_export_frame`, each declared
there *and* defined here — `E0428`), 3 in `include/w32term.rs` (`E0428` for
`syms_of_w32image`/`globals_of_w32image` at 1347/1348, and `E0425` for `COLORREF` at 1427), and 1 in
`include/frame.rs` (that mirror's deliberate `HAVE_NTGUI` `compile_error!`).  All four rows are
mirror-side and named in Owed 1-3.
**The first draft's reading is the door's own demonstration that it type-checks this unit** (the
second `probe` run, before the fixes): 41 `error` lines / 39 headers, **30 of them anchored in
`_root.rs`** — 27 `cannot find macro Gdip*`/`Gdiplus*`/`SHCreateMemStream` (C 41's `#include
"w32gdiplus.h"` edge was missing from the include graph), 5 `E0530 let bindings cannot shadow
statics` (the `output` static against `let output` in `include/character.rs` 448/474/503 and
`include/buffer.rs` 1919/1947), 3 `E0614 type Emacs_Color cannot be dereferenced` (`(*color).red` on
a record C holds by value) and 2 `E0308 mismatched types` (`core::ptr::null ()` for the `*mut`
out-parameters of `map_w32_filename` and `CreateDIBSection`).  Every one of the 30 was fixed in this
unit; the fixes are what the 0 above measures.

**③ The "0 in the unit" reading is not vacuous, measured.**  The materialized graph
(`build/probes/w32image/`) was copied to `/tmp/w32img-lane/{base,mut}`, and one statement was
injected at the top of `w32_image_bg_color`'s body in the copy: `let _lane_mutation: c_int =
"w32image_arm_probe_no_such_type";`.  Both copies were compiled with the **pinned toolchain**
(`rust-toolchain.toml` → `nightly-2026-10-01-aarch64-apple-darwin`) and the same three
`--cfg`s as ②: **base 34 errors / 29 warnings**, **mutant 35 errors / 29 warnings**, the extra one
`E0308 mismatched types` anchored at the injected line in `_root.rs`, and the identifier
`_lane_mutation` appears exactly once in the mutant's log and zero times in the base's.  The door
therefore resolves *and type-checks* this unit's bodies despite the 31 mirror-side `E0428`s (P-86
⑦'s trap does not bite here), so ②'s "0 in the unit" is a reading and not a silence.

**③a** (**Probe budget, stated**: four `probe` runs against a budget of three — the first draft's
run, the run after the fixes, and the two above.  Each run's materialization and object were deleted
immediately after its reading was recorded.)

**④ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/w32image.c` →
`src/w32image.c: 4 conditions, 0 unknown, 0 disagreements [VERIFIED]` (C 26, 35, 42, 224), which is
why the frozen macro table decides this file's arms and why every arm in `## Gate mapping` is a
verdict rather than a reading of the comments.

**⑤ The text gates.**  `python3 tools/extract-abi.py --check emacs/src/w32image.c
src/w32image.rs` → **two** findings and no other: `- function decode_delay does not mention
break, case, default, switch` (now the `docs/ports/macro-body-notes.tsv` row this round appended,
Deviations 6) and `- line count drifted 105.9%: C 717 vs Rust 1476` (the integrator's tolerance row,
Owed 5).  `python3 tools/check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures`; `python3 tools/check-mirror-dupes.py` → `17 duplicate declaration(s), 17
ratchet row(s), 0 failure(s)`; `python3 tools/check-citations.py` → `676 file(s) scanned, 0 drift, 0
range, 1812 external, 36 allowed, 0 stale`; `python3 tools/check-dead-file-coverage.py` → `0
failure(s)`; `python3 tools/check-ledger-ticks.py` → `0 failure(s)` (this ledger ticks no function:
the file's checklist is the empty P-86 ② one).

**⑥ What the file is.**  C 72-205's `gdiplus_init` loads 27 GDI+/shlwapi entry points by hand (with
`SHCreateMemStream`'s ordinal fallback for pre-Vista Windows); C 216-252's two entry points start and
stop GDI+; C 254-409's four helpers decide whether the native image API applies, decode a frame delay
out of a `PropertyItem`, select a frame and compute the background colour; C 411-485's
`w32_load_image` is the loader `image.c` calls, C 514-641's two functions drive the encoder CLSID
cache and the frame-export path; and C 643-706's one subr creates a thumbnail file.  Nothing of it
was executed anywhere (see `## Not claimed`).

## Deviations

1. **C 1-18's opening ownership/title block is dropped** (PORTING.md 0.1-19); the port's first line
   is the revision line, and C 20's one-author comment is carried as C's own words.
2. **C 26-29's `#ifdef MINGW_W64` around `#include <objidl.h>` carries no file edge.**  `objidl.h`
   is the Windows SDK's, no mirror of it exists in the tree and P-88 ① forbids inventing one under
   `src/include/`, so C's four lines are kept as text and the names the header supplies this
   file (`IStream`, C 432/436/437) stand in the `<objidl.h>` section.  Both arms of the `#ifdef`
   need them: the macro only chooses which SDK header spells them.
3. **C 26-32's four SDK headers are an in-place stand-in section (P-88 ①), and every name in it is
   `UNVERIFIED`.**  The section declares `BYTE`, `WORD`, `WCHAR`, `wchar_t` (the Windows 16-bit
   one), `DWORD`, `LONG`, `ULONG`, `UINT`, `BOOL`, `INT`, `VOID`, `TRUE`/`FALSE`, the handles
   (`HANDLE`, `HWND`, `HDC`, `HBITMAP`, `HGDIOBJ`, `HPALETTE`, `LPVOID`, `LPCSTR`, `ULONG_PTR`),
   `MAX_PATH`, `GUID`/`CLSID`/`IID`/`HRESULT`, `IStream`/`IStreamVtbl`, the GDI+ names
   (`GpStatus` + `Ok`/`GenericError`/`InvalidParameter`, the five handle records, `ARGB`, `PROPID`,
   `PixelFormat`, `InterpolationMode`, `GpUnit`, `RotateFlipType`, `DrawImageAbort`,
   `GetThumbnailImageAbort`, `PropertyTagFrameDelay`, `GdiplusStartupInput`/`GdiplusStartupOutput`,
   `PropertyItem`, `ImageCodecInfo`, `EncoderParameters`), the eleven GDI entry points the export
   path calls, the DIB records (`BITMAPINFOHEADER`, `RGBQUAD`, `BITMAPINFO`) and C 571's `RECT`, the
   four codes (`BI_RGB`, `DIB_PAL_COLORS`, `SRCCOPY`, `BITSPIXEL`) and the two string comparisons
   `stricmp`/`wcscmp`.  Each has the SDK's own text in a `/// C:` line above it, none is measured,
   and the layout assertions are owed (Owed 4).
4. **C 23-24's three declarations the graph's mirrors do not carry stand in place (P-88 ①)**:
   `add_to_log` (C 310/481), `plist_get` (C 392/445, defined by `fns.c`) and `valid_image_p`
   (C 419, defined by `image.c`).  `include/lisp.rs` and `include/dispextern.rs` have no row for
   them, and the `/// C:` lines cite the reference's own header lines (Owed 6).
5. **C 273-282's `enum PropertyItem_type` is a `type` plus its eight constants, not a
   `#[repr(C)] enum`.**  C 287 assigns a GDI+ `WORD` to it and C's own `switch` has a `default`
   arm for the values the enum does not name, so a Rust enum would need a `transmute` that is UB for
   exactly the case C handles; the tree spells an SDK enum whose values are data this way
   (`src/w32select.rs` 412/483's `GpStatus`/`Status`).
6. **C 291-312's `switch` is the port's `match`** (PORTING.md 2.2): each `case` is a pattern,
   C's `default:` is the `_` arm, and the arms' `break`s have no counterpart — the token row this
   round appended to `docs/ports/macro-body-notes.tsv`.
7. **C 437's `IStream_Release` is a `macro_rules!`, because C's text is a macro** (C 25's
   `COBJMACROS` asks for it): `<objidl.h>`'s `#define IStream_Release(This)
   (This)->lpVtbl->Release(This)` is written as C's own body over the `IStream`/`IStreamVtbl`
   records from Deviations 3.  The vtable's first three slots are modelled (`IUnknown`'s
   `QueryInterface`/`AddRef`/`Release`); only `Release` is reached, and the rest of the vtable is
   offsets a reader does not need here.  C 171's `MAKEINTRESOURCEA` is the same shape - C's cast
   `((LPSTR) (ULONG_PTR) ((WORD) (i)))` as a `macro_rules!` body.
8. **C 30's `#define COBJMACROS` is a unit-typed constant** (`pub const COBJMACROS: () = ();`, the
   `src/w32fns.rs` 95-97 reading): the macro selects the COM headers' C spellings and the port has
   no COM mirror to select from.
9. **C 212-213's `input` and `output` live in `mod gdiplus_state` and C's statements read
   `gdiplus_state::input.GdiplusVersion = 1;`.**  A `static` named `output` at this crate root makes
   every `let output` in the mirrors on this graph illegal (measured: `E0530` at
   `include/character.rs` 448/474/503 and `include/buffer.rs` 1919/1947, and Rust has no `#[allow]`
   for it).  C's names are kept; the path is the port's.  The other three objects of C 209-213 stay
   at the crate root.
10. **C 425's local is named `filename`**, not C's `fn`: `fn` is a Rust keyword, and the token
    check's two-character rule skips it (so no notes row is needed).
11. **C 527 and C 549's `for` loops are `while` loops** with C's increment as the body's last
    statement (the `src/w32select.rs` 1034 reading of the same C loop).
12. **C 504-509's six `L"…"` literals are `[u16; N]` arrays plus C's terminator**, because Rust has
    no wide-string syntax (the `src/w32fns.rs` 385-393 reading of `L"DarkMode_Explorer"`), and the
    table's `mime` fields take their addresses.
13. **C 220's `GpStatus status;` is zero-initialised** (`let mut status: GpStatus = 0;`): C's
    declaration has no initialiser and both its paths assign before use (PORTING.md 1.5).
14. **C 42-207's 28 objects and C 708-717's two functions are declared *and* defined** — by this
    unit for the definition and by `include/w32gdiplus.rs` 202-246 and `include/w32term.rs`
    1339-1350 for the mirror-side declaration.  The port keeps C's definitions (they are the symbols
    the other Windows units read through the mirror's macros), so the reading has 31 `E0428`s; the
    fix is the mirrors' and is Owed 1.
15. **C 601-602's `biPlanes`/`biBitCount` widen and narrow explicitly** (`1`, `bits_per_pixel as
    WORD`), and C 470-471's `img->width = width` casts the `UINT` to the record's `int`.
16. **C 549's `j < num` compares `c_int` with `c_uint` as C's own conversion does** (`j < num as
    c_int`), and C 375's `frame >= frameCount` reads `frameCount as c_int`: the port makes C's
    implicit conversions explicit rather than changing the operands.
17. **The three `#ifdef WINDOWSNT` pairs inside the file's own arm (C 224-229, and the C 35/C 42
    include and block edges) need no feature of their own**: the file gate *is* that macro, so the
    `#ifdef` side is code and the `#else` side is carried as C's own lines (C 227-229).
18. **C 26-32's headers also supply names the *mirrors* on this graph spell and the unit does not:**
    an `E0425` for `COLORREF` and the mirrors' own gaps stay (Evidence ②), which is why the unit
    declares `RECT`/`HDC` (its own C 571/573) and not `COLORREF`.

## Owed

1. **The 31 collision rows of Evidence ②.**  29 in `include/w32gdiplus.rs` (the 27 `fn_*` objects
   declared at its 202-246 and the two functions `w32_gdip_get_encoder_clsid`/`w32_gdip_export_frame`
   at its 391-395) and 2 in `include/w32term.rs` 1347/1348 (`syms_of_w32image`/`globals_of_w32image`)
   are names this unit *defines*; each needs the `include/w32gui.rs` 96-116 shape —
   `pub mod <stem>_decls { use super::*; unsafe extern "C" { … } } use <stem>_decls::*;` — which a
   crate-root definition shadows (P-21).  Not this lane's write set.
2. **`include/w32term.rs` 1427's `COLORREF`** (`E0425`): the mirror's `w32_fill_rect` names a
   `<windows.h>` type nothing in the graph declares.  This unit cleared the mirror's
   `RECT`/`HDC` rows of the same class (they are C 571/573's own names here); `COLORREF` is not a
   name `w32image.c` spells and was left to that mirror's owed list (the `docs/ports/w32inevt.md`
   Owed 5 row).
3. **`include/frame.rs`'s deliberate `HAVE_NTGUI` `compile_error!`** is the third mirror-side error
   of Evidence ②; it belongs to that mirror's own owed list (the task names it as a known debt).
4. **The layout assertions and the measured values of the thirteen stand-in records and fourteen
   constants** (Deviations 3), once a Windows or mingw toolchain exists: `xtask layout-asserts` reads
   a C compiler's readings, and there is no `gdiplus.h`/`windows.h` here to read.
5. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32image`, tolerance ≥ **1.06**,
   basis `total`, reason from Deviations 3-14 (the two stand-in sections, the per-item gate and
   `/// C:` citations, the `match` body, the module-qualified records and the widened literals).
   That file is not in this lane's write set, so `extract-abi.py --check` stays `FAIL` with exactly
   the one row Evidence ⑤ records.
6. **The three in-place declarations of Deviations 4**: `add_to_log` (lisp.h 4378), `plist_get`
   (lisp.h 4280) and `valid_image_p` (dispextern.h) have no mirror row; if `include/lisp.rs` or
   `include/dispextern.rs` gains them, this unit's copies retire in the same commit (P-88 ⑤).
7. **`docs/baseline/port-lines-pending.tsv`** (the stale `src/w32image.c` 717/13 row),
   **`docs/port-map.md` 576** (the port's 13-line count), **`docs/baseline/feature-coverage.tsv`**
   and **`docs/status.md`**: the integrator's regeneration and running-log entry for this revision
   (the `docs/ports/w32console.md` Owed 8/9 rows).
8. **`mod gdiplus_state` (Deviations 9) is a workaround for two mirror locals.**  If
   `include/character.rs`'s or `include/buffer.rs`'s `let output` bindings are ever renamed, the two
   records can return to the crate root and the qualification disappears.  Nothing in this lane can
   change those mirrors.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte object, unchanged by every edit), so this is a *text* port: nothing here
  has been executed, and no Windows target, GDI+ runtime, image file, `HGDIOBJ` or thumbnail was
  involved.  There is no behavioural test, no differential run, no ERT and no device evidence.
- **The arm reading is not a Windows compile.**  This machine has no `<windows.h>` and no GDI+
  header, so Evidence ② is a `rustc` reading of the *materialized mirror graph* under three
  `--cfg`s: all 33 of its errors are in mirror copies and "0 in the unit" means only that no error's
  primary span is one of this unit's lines (Evidence ③ shows the reading is not vacuous) — **not**
  that the unit links, and not that the arm is a working Windows configuration.  The 31 `E0428`s and
  the mirror gaps are real work the integrator owes (Owed 1-3).
- **The `<windows.h>`/`<gdiplus.h>` records and values stay unmeasured** (Deviations 3, Owed 4):
  every one is `UNVERIFIED`, with the documented SDK text as its only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 5).  Nothing in this ledger claims
  the file's checklist is non-empty (P-86 ②) or that the port is complete.
- **The `IStreamVtbl` modelling is partial** (Deviations 7): three slots of `<objidl.h>`'s vtable are
  written, which is what C 437 reaches; a Windows target that needs another slot extends the record.
