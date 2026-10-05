# emacs/src/w32dwrite.c (1399 lines) — whole file, C 1-1399

> **One of P-86's class: the frozen configuration compiles nothing here.**  The target's platform
> object variables are empty (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`,
> `docs/baseline/frozen-build-sources.txt`), so `w32dwrite.o` is in no source list and
> `extract-abi.py`'s checklist for the file is empty (P-86 (2)) — this ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `femacs/src/w32dwrite.rs` (**2415 lines**, md5 `a831345a7968db78edd9bcfcc459b576`, sha256
> `e3a19d5efe4cf3b97d1f6024e0a784e53ea3c500419aa3931999d7d5bfb5cae9`), carrying **C 1-1399 with both
> arms of every `#if`**.  The reference `emacs/src/w32dwrite.c` is md5
> `11001339948e3d4939aee0a262cf5688` (unchanged; `emacs/` is read-only).
>
> **The arm set is `have_ntgui` + `windowsnt` + `have_window_system` (+ `harfbuzz` for the
> `w32_use_direct_write` half)**, the set the landed siblings `src/w32font.rs` and
> `src/w32uniscribe.rs` register.  P-86 (1)'s reading applies: transcribe the text, both arms, no
> stub; what the earlier note measured (not in the frozen source list, empty checklist, 336-byte
> `build-tu` object) is unchanged and still true — it is the *conclusion* that is withdrawn.
>
> **Acquisition: this piece is `text-only` (P-86 (3)(ii)).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty and `find`
> finds no `windows.h`/`dwrite.h`/`dwrite_3.h`/`d2d1.h`), so there is **no `<windows.h>` and no
> `<dwrite_3.h>`/`<d2d1.h>` to compile or measure against**: every record and value of the three
> stand-in sections is `UNVERIFIED` (documented, not measured) and the layout assertions are owed.
> What the round does carry: the frozen `build-tu` reading, the machine-checked coverage table, the
> `#if` verdicts, a per-block C citation, the `probe` reading of the
> `have_ntgui`+`windowsnt`+`have_window_system`+`harfbuzz` arm (0 errors anchored in this unit), a
> **free multi-configuration sweep** of the same materialized text, and a **mutation test** over
> this unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (a platform backend; `docs/baseline/frozen-build-sources.txt` lists no `w32dwrite.o`) | `feature = "have_ntgui"` (on every item *and* on C 36-50's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 40-42 `#if !defined MINGW_W64 && !defined CYGWIN` / `# define INITGUID` | `all (not (feature = "mingw_w64"), not (feature = "cygwin"))` inside the file gate | the non-MinGW (not mingw-w64) and non-Cygwin headers |
| C 52 `#ifndef MINGW_W64` … C 548 `#endif` (the file's own `<dwrite_3.h>` stand-in, C 53-544) | `not (feature = "mingw_w64")`; the `#else` arm (C 546-547, the two SDK includes) is `feature = "mingw_w64"` | the two MinGW families |
| C 1136 `#ifdef MINGW_W64` / C 1141 `#else` / C 1146 `#endif` (the three IUnknown vtable slots) | `feature = "mingw_w64"` / `not (feature = "mingw_w64")` | both arms carried |
| C 1332 `#ifdef HAVE_HARFBUZZ` / C 1337 `#else` / C 1339 `#endif` in `w32_use_direct_write` | `feature = "harfbuzz"` / its negation | the HarfBuzz configurations |

`tools/check-cond-eval.py src/w32dwrite.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (the file is on `docs/baseline/config/cond-eval-verified.txt` 391): C 40
(`#if !defined MINGW_W64 && !defined CYGWIN`), C 52 (`#ifndef MINGW_W64`), C 1136
(`#ifdef MINGW_W64`) and C 1332 (`#ifdef HAVE_HARFBUZZ`) — the four rows above plus their `#else`
arms, and nothing else.  The frozen table has all three macros `undef`, so the extractor reads the
`MINGW_W64`-free and HarfBuzz-free arms as C's live program; the port carries both arms of each.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19); the port's first line is the revision line |
| 20-34 | the file's own design note ("This requires the HarfBuzz font backend to be available. …") | file gate; carried as C's words |
| 36 | `#include <config.h>` | file gate; `include/config.rs` |
| 37 | `#include <math.h>` | file gate; `lib/include/math.rs` + the in-place `round` declaration (Owed 1) and `FLT_MAX` |
| 38 | `#include <windows.h>` | file gate; the stand-in section below (C 38's names: scalars, handles, `SUCCEEDED`/`FAILED`, `S_OK`/`E_NOINTERFACE`/`E_INVALIDARG`, `_HRESULT_TYPEDEF_`, `RGB`, `SIZE`/`RECT`/`LOGFONT`/`LOGFONTW`, the six GDI/loader entry points) |
| 40-42 | `#if !defined MINGW_W64 && !defined CYGWIN` / `# define INITGUID` / `#endif` | both arms carried (`mingw_w64`/`cygwin`); the port's answer is the unit-typed `const INITGUID` |
| 43-45 | `<initguid.h>`, `<ole2.h>`, `<unknwn.h>` | file gate; the stand-in section (`GUID`/`IID`/`REFIID`, `IUnknown`, `IUnknownVtbl`) |
| 47-50 | `frame.h`, `w32font.h`, `w32common.h`, `w32term.h` | file gate; the four mirrors |
| 52-59 | `#ifndef MINGW_W64` … `# define EMACS_DWRITE_UNUSED (name) …` | `not (mingw_w64)`; macro-body-notes row (the SFINAE-free `stdmethodcalltype`, Deviations 4) |
| 61 | `#define DWRITE_E_NOCOLOR _HRESULT_TYPEDEF_(0x8898500CL)` | `not (mingw_w64)` |
| 63-93 | the five DWrite enums (`DWRITE_PIXEL_GEOMETRY` … `DWRITE_FACTORY_TYPE`) | `not (mingw_w64)`; `#[repr(C)] pub enum` with C's values |
| 95-116 | `DWRITE_FONT_METRICS`, `DWRITE_GLYPH_METRICS` | `not (mingw_w64)` |
| 118-152 | `D2D1_POINT_2F`, `D2D1_BEZIER_SEGMENT` and the four `D2D1_*` enums | `not (mingw_w64)` |
| 154-163 | the ten `typedef interface X X;` forward declarations | `not (mingw_w64)`; C's lines kept as comments (Rust has no tag declaration) |
| 165-170 | the three `DEFINE_GUID` definitions | `not (mingw_w64)`; `#[unsafe(no_mangle)] pub static` records |
| 172-204 | `DWRITE_GLYPH_OFFSET`, `DWRITE_GLYPH_RUN`, `_D3DCOLORVALUE`/`DWRITE_COLOR_F`, `DWRITE_COLOR_GLYPH_RUN` | `not (mingw_w64)` |
| 206-256 | `IDWriteFontFaceVtbl` | `not (mingw_w64)` |
| 258-260 | `interface IDWriteFontFace` | `not (mingw_w64)` |
| 262-279 | `IDWriteRenderingParamsVtbl` | `not (mingw_w64)` |
| 281-283 | `interface IDWriteRenderingParams` | `not (mingw_w64)` |
| 285-311 | `IDWriteFontVtbl` | `not (mingw_w64)` |
| 313-315 | `interface IDWriteFont` | `not (mingw_w64)` |
| 317-347 | `IDWriteBitmapRenderTargetVtbl` | `not (mingw_w64)` |
| 349-351 | `interface IDWriteBitmapRenderTarget` | `not (mingw_w64)` |
| 353-375 | `IDWriteBitmapRenderTarget1Vtbl` | `not (mingw_w64)` |
| 377-379 | `interface IDWriteBitmapRenderTarget1` | `not (mingw_w64)` |
| 381-402 | `IDWriteGdiInteropVtbl` | `not (mingw_w64)` |
| 404-406 | `interface IDWriteGdiInterop` | `not (mingw_w64)` |
| 408-443 | `IDWriteFactoryVtbl` | `not (mingw_w64)` |
| 445 | `interface IDWriteFactory` | `not (mingw_w64)` |
| 447-462 | `IDWriteColorGlyphRunEnumeratorVtbl` | `not (mingw_w64)` |
| 464-466 | `interface IDWriteColorGlyphRunEnumerator` | `not (mingw_w64)` |
| 468-515 | `IDWriteFactory2Vtbl` | `not (mingw_w64)` |
| 517-519 | `interface IDWriteFactory2` | `not (mingw_w64)` |
| 521-537 | `ID2D1SimplifiedGeometrySinkVtbl` | `not (mingw_w64)` |
| 539-541 | `interface ID2D1SimplifiedGeometrySink` | `not (mingw_w64)` |
| 543 | `typedef ID2D1SimplifiedGeometrySink IDWriteGeometrySink;` | `not (mingw_w64)` |
| 545-548 | `#else /* MINGW_W64 */`, the two SDK includes, `#endif` | `mingw_w64`; C's lines kept as comments (no mirror exists) |
| 550-555 | the three `config_*` variables | file gate |
| 557-560 | `MEASURING_MODE`/`RENDERING_MODE`/`ANTIALIAS_MODE` | file gate; three `macro_rules!` |
| 562-572 | `release_com` and `RELEASE_COM` | file gate; the two `as`/`unwrap` chains of C's arrow chain (Deviations 6) |
| 574-594 | the geometry-sink design note and `struct geometry_sink` | file gate |
| 596-613 | `geometry_sink_QueryInterface` / `AddRef` / `Release` | file gate |
| 615-635 | `geometry_sink_AddBeziers` | file gate |
| 637-649 | `geometry_sink_AddLines` | file gate |
| 651-662 | `geometry_sink_BeginFigure` | file gate |
| 664-667 | `geometry_sink_EndFigure` | file gate; C's body is empty and stays empty |
| 669-673 | `geometry_sink_Close` | file gate |
| 675-679 | `geometry_sink_SetFillMode` | file gate; empty body |
| 681-685 | `geometry_sink_SetSegmentFlags` | file gate; empty body |
| 687-694 | the seven file-scope objects (`direct_write_available`, the four COM pointers, the sink and its vtable) | file gate; `core::mem::zeroed ()` for the two aggregates |
| 696-706 | `verify_hr` | file gate |
| 708-789 | `get_font_face` (with its design note) | file gate; C's `void *` readings of `dwrite_cache` are the port's `as` casts (Deviations 3) |
| 791-796 | `w32_dwrite_free_cached_face` | file gate; `#[unsafe(no_mangle)]` |
| 798-802 | `convert_metrics_sz` | file gate |
| 805-916 | `text_extents_internal` (with the note at C 805-807) | file gate; the two `SAFE_ALLOCA`s and four exits |
| 918-938 | `w32_dwrite_encode_char` | file gate; `#[unsafe(no_mangle)]` |
| 940-962 | `w32_dwrite_text_extents` | file gate; `#[unsafe(no_mangle)]` |
| 964-1011 | `get_bitmap_render_target` (with the note at C 964-965) | file gate; C's two function `static`s |
| 1013-1158 | `w32_initialize_direct_write` | file gate; C 1137-1140 the `mingw_w64` arm (comments), C 1142-1145 the `#else` arm |
| 1160-1321 | `w32_dwrite_draw` | file gate; C's `for (;;)` is the `loop { … break; }` shape (PORTING.md 2.2) |
| 1323-1340 | `w32_use_direct_write` (with the note at C 1323-1328) | file gate; C 1333-1336 `harfbuzz`, C 1338 the negation |
| 1342-1348 | `DEFUN ("w32-dwrite-available", …)` | file gate |
| 1350-1385 | `DEFUN ("w32-dwrite-reinit", …)` | file gate |
| 1387-1399 | `syms_of_w32dwrite` | file gate; the `DEFVAR_BOOL` storage is defined at this unit (Deviation 8) |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32dw-lane/cargo cargo run -q -p xtask -- build-tu src/w32dwrite.c`
→ the first run exited 1 with one syntax error in this unit's own text (`error: `<` is interpreted
as a start of generic arguments for `c_float`, not a comparison` at `_root.rs:1747`, i.e. C 875's
`metrics->lbearing > lbearing` needed parentheses around the `as`), then:
`built src/w32dwrite.rs -> ../build/femacs/w32dwrite.o`, **exit 0**, and
`build/femacs/w32dwrite.o` is **336 bytes** — the P-86 empty object, which is what every item being
`#[cfg (feature = "have_ntgui")]` produces.  `build-tu` spent **3**; its third run carries the
LOGFONT and `mut cache` fixes.  The *final* revision was re-read with the same `rustc` invocation
`build-tu` uses (`--edition 2024 --crate-type lib --emit obj --crate-name w32dwrite
-C panic=abort -C codegen-units=1 -C opt-level=0`, no `--cfg`) on a root the materializer had been
re-applied to: **exit 0, 336 bytes**.

**② The arm reading (`xtask probe`), on the final revision.**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32dw-lane/cargo cargo run -q -p xtask -- probe
src/w32dwrite.rs /tmp/w32dw-lane/base.rlib --crate-type=lib -C panic=abort
--cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="have_window_system"'
--cfg 'feature="harfbuzz"'`
→ **exit 1, 1 error line, and 0 of them anchored in this unit** (`_root.rs`: 0 — P-86 (7)'s by-file
reading; 19 warnings, all in mirror copies).  The one error is a **mirror's own refusal**:
`build/probes/w32dwrite/_Users_liu_projects_..._femacs_src_include_frame_rs.rs:329:1`,
`src/include/frame.rs`'s deliberate `compile_error!` ("the HAVE_NTGUI arm of `struct frame`'s flag
run is unmeasured on this machine … a Windows/mingw clang probe has to measure it") — the same row
`docs/ports/w32font.md` Owed 5 and `docs/ports/w32uniscribe.md` Owed 4 record, and not this lane's
to fix.  The 19 warnings are the mirrors' own (`unused doc comment` on `extern` blocks in
`lib/include/stdbit.in.rs` 644, `include/lisp.rs` 3939, `include/w32common.rs` 176/195/234,
`include/w32term.rs` 87/126/281/292/543/1024/1047/1076/1101/1116/1147/1255/1328, and one
`unused attribute` at `include/frame.rs` 1355/1360).  **No warning is anchored in `_root.rs`.**
Budget: `probe` spent **2** — the second run is the *final* revision's (the md5 Evidence ④ names),
and both read the same: exit 1, one error, that anchor, 19 warnings.

**③ The free multi-configuration sweep (direct `rustc` on a materialized root; not a Windows-target
compile).**  `xtask`'s materializer was re-applied by a local script
(`/tmp/w32dw-lane/render2.py`, deleted with the scratch dir) that reproduces its shape (the unit's
own text is `_root.rs`, 2416 lines against `xtask`'s 2415 — one trailing line; every gated mirror
becomes its own copy), then the root was compiled once per configuration with the pinned toolchain
(`rustup`'s `nightly-2026-10-01`, `femacs/rust-toolchain.toml`) and
`--edition 2024 --crate-type lib --emit obj -C panic=abort -C opt-level=0`:

| configuration | exit | typed `error[…]` | plain `error: …` | anchored in `_root.rs` |
| --- | --- | --- | --- | --- |
| `have_ntgui` | 1 | 0 | 1 | **0** |
| `+ windowsnt` | 1 | 0 | 1 | **0** |
| `+ have_window_system` | 1 | 0 | 1 | **0** |
| `+ windowsnt + have_window_system` | 1 | 0 | 1 | **0** |
| `+ have_window_system + harfbuzz` | 1 | 0 | 1 | **0** |
| `+ windowsnt + have_window_system + harfbuzz` (base) | 1 | 0 | 1 | **0** |
| none (frozen) | **0** | 0 | 0 | 0 |

So *every* arm of the file's four conditions compiles with no error anchored in this unit, and the
frozen row is Evidence ①.  **The one plain error in each arm row is the mirror's
`include/frame.rs` 329 `compile_error!`** (Evidence ②).  **Reading limit, stated because it matters
for the narrower rows**: `rustc` reports a root-level `compile_error!` and can stop before the
type-checking of the rest of the crate (measured locally: a `compile_error!` at the crate root
suppresses a later `E0412`, while the same text inside a `cfg`'d module does not), so the
*no-windowsnt* rows are **not** a reading of "the `windowsnt`-gated mirror items resolve" — that
question is answered by Evidence ④'s mutation test on the base arm, where the unit's own text *is*
type-checked.

**④ The mutation test (P-86 (7)): the reading is not vacuous.**  One type error injected into this
unit's own text — `w32_dwrite_free_cached_face`'s body got
`let _mutation: *mut u8 = cache;` before its `if` — and the base arm re-compiled: exit 1,
**2** errors against ③'s 1, the new one `error[E0308]: mismatched types` anchored in the unit's own
text (`_root.rs:1668:28`, and that materialized line *is* the injected line — the root carries the
unit's own text).  The mutation was reverted and the unit verified: md5
`a831345a7968db78edd9bcfcc459b576` before and after, `grep -c _mutation` is 0.  So the base arm's
"0 in this unit" certifies that the unit's own lines type-check under that arm, not merely that they
resolve.

**⑤ `python3 tools/extract-abi.py --check emacs/src/w32dwrite.c femacs/src/w32dwrite.rs`**: before
this round's `docs/ports/macro-body-notes.tsv` rows it was `FAIL` with four findings (three
`does not mention …` rows plus the line-drift row); the three note rows **are written** (that file is
in this lane's write set, appended at its end: `EMACS_DWRITE_UNUSED`/`stdmethodcalltype`,
`w32_initialize_direct_write`/`winapi`, `syms_of_w32dwrite`/`doc`), so the re-run leaves exactly
**one** finding: `FAIL line count drifted 72.6%: C 1399 vs Rust 2415` —
`docs/ports/line-drift-tolerance.tsv`'s row (not in this lane's write set; Owed 3).

**⑥ `python3 tools/check-citations.py`**: `676 file(s) scanned, 0 drift, 0 range, 1604 external,
36 allowed, 0 stale` — the file's `/// C: …` citations (every item's, and the per-block ranges) are
all inside their cited windows.

**⑦ `python3 tools/check-declarations.py`**: exit 0, no `decl`/`version`/`title`/`recital`/`literal`
row for `femacs/src/w32dwrite.rs` (the file's comments quote the SDK and C's own design notes and
recite nothing about the reference's opening block — that block is dropped, Deviation 1).

**⑧ `python3 tools/check-mirror-dupes.py`**: `17 duplicate declaration(s), 17 ratchet row(s),
0 failure(s)` — the shared working tree's pre-existing rows, none of them this unit's.

**⑨ `python3 tools/check-ledger-ticks.py`** and **⑩ `python3 tools/check-dead-file-coverage.py`**:
this ledger carries no function tick (P-86 (2)'s class uses the range table instead), so the tick
gate leaves it alone; the coverage gate reads the `## C line ranges` table above and reports
`0 failure(s)` for `src/w32dwrite.c` (every non-blank line of C 1-1399 is inside one row, in
ascending order without overlap).

**⑪ The shared working tree.**  Evidence ②③④ ran against the tree as it stood on 2026-10-05,
whose `femacs/src/include/frame.rs` also carries **another lane's uncommitted** `FRAME_*_P`
predicate edit (`git diff` rows from line 841 on) and `femacs/src/w32console.rs` carries another
lane's edit as well — neither is this lane's file and both were left untouched.  The readings' only `include/frame.rs` row is still the `compile_error!` at its 329, so
the other lane's work neither added nor removed an error in this unit's graph; the by-file split
above is what it is at that revision, not at a committed one.

**⑫ The toolchain**: `femacs/rust-toolchain.toml` pins `nightly-2026-10-01`
(`rustc 1.101.0-nightly (21b707e3f 2026-09-30)`); Evidence ②③④ were taken with it.  With the
machine's *default* stable toolchain the same base arm adds one mirror row
(`include/lisp.rs` 3762's `E0658 use of unstable library feature c_variadic`, the pin's reason).

## Deviations

1. **The opening ownership/title block (C 1-18) is dropped** and the port's first line is
   `// femacs: port of src/w32dwrite.c @ fd7afced403.` (PORTING.md 0.1-19); C 20-34's design note is
   carried as C's own words, because it is the file's program documentation and recites nothing.
2. **The `mingw_w64` arm's two SDK headers (C 545-548) and its three `Base.`-qualified vtable slots
   (C 1136-1140) have no Rust item.**  With `MINGW_W64` the types come from `<dwrite_3.h>`/`<d2d1.h>`
   and `dwrite_geometry_sink_vtbl.Base` is that header's embedded `IUnknownVtbl`; no file in this
   tree mirrors either header, so those lines stand as C's text (comments) and the port's own
   stand-in record (the `not (feature = "mingw_w64")` arm, C 1142-1145) is the arm the port can
   spell.  Both arms are carried; the `mingw_w64` arm is unbuildable here (Owed 4).
3. **C's implicit `void *` ⇄ typed-pointer conversions are explicit `as` casts**: C 733/786's
   `*face = uniscribe_font->dwrite_cache;` / `uniscribe_font->dwrite_cache = *face;`,
   C 1002/1045/1055's `(void **)`/`(IUnknown **)` casts, C 929's `(UINT32 *) &c` (written
   `&raw mut c as *const UINT32`, the field's parameter being `const UINT32 *`), and C 1026-1036's
   `HMODULE`/`VOIDFNPTR` readings.  Rust has no implicit qualification/void conversions, so each
   site carries one `as`; the value is C's.
4. **`EMACS_DWRITE_UNUSED` cannot be called at its own sites.**  C's expansion is a *struct field*
   (`void (STDMETHODCALLTYPE *name) (void)`), and a Rust `macro_rules!` cannot expand to one
   (measured locally: `macros cannot expand to struct fields`), so the port writes each of the ~50
   fields out as `Option<unsafe extern "system" fn ()>` and keeps the macro as the C body's answer;
   the one token the Rust body cannot name (`stdmethodcalltype`) is registered in
   `docs/ports/macro-body-notes.tsv`.  The same file registers `w32_initialize_direct_write`'s
   `winapi` (C 1030-1031's function-local typedef) and `syms_of_w32dwrite`'s `doc` (C 1390's
   `DEFVAR_BOOL` label, which the mirror's macro takes positionally).
5. **`STDMETHODCALLTYPE`/`WINAPI`/`CALLBACK` are `extern "system"`** and `VOID` is `()` for a
   function-pointer result (C's `<windll>` attribute and `typedef void VOID` have no Rust item);
   the stand-in sections name them only in `/// C:` lines.
6. **C's `->lpVtbl->Method` chains are the port's two dereferences of the same arrows plus the
   `Option<…>` the tree's convention gives a C function pointer**: `(*(*obj).lpVtbl).Method.unwrap ()
   (obj, …)`; `release_com`'s `(*i)->lpVtbl->Release (*i)` is
   `(*(**i).lpVtbl).Release.unwrap () (*i)`, and `RELEASE_COM (i)`'s `(IUnknown **) &i` is
   `&raw mut $i as *mut *mut IUnknown` (Rust cannot cast a `&mut` to a differently-typed raw
   pointer).  For the three IUnknown slots C 1142-1145 cast a function pointer through `void *`; a
   direct `as` between two function-pointer types is `E0605`, so the port uses
   `core::mem::transmute` at exactly those three sites (the seven other slot assignments, whose
   signatures match, are plain `Some (…)`).
7. **C's narrowing/widening conversions are explicit `as` casts**: `float`→`short` and
   `double`→`int`→`short` at C 870-878/910-911, `int`→`unsigned short` at C 823/825/1231/1233,
   `c_int`→`UINT32`/`FLOAT` at the DirectWrite call sites, `f64`→`c_float` in `Fw32_dwrite_reinit`
   (C 1363-1377), and `int`→`DWRITE_PIXEL_GEOMETRY` for C 1116's `def->lpVtbl->GetPixelGeometry
   (def)` (C's implicit enum conversion, which Rust spells `core::mem::transmute` — the field's own
   type stays C's `int`).  Rust's `as` saturates where C's conversion is out of range/undefined.
8. **`w32_inhibit_dwrite`'s storage is defined in this unit.**  C's `DEFVAR_BOOL` takes
   `&globals.f_w32_inhibit_dwrite`, i.e. the field of the single `struct emacs_globals globals`
   (`src/alloc.c`), so C's text has no definition line; the port keeps the variable in its own unit
   (`#[unsafe(no_mangle)] pub static mut w32_inhibit_dwrite: bool = false`, the
   `tools/check-globals.py` rule and the `src/buffer.rs` 4949 shape) and the mirror's
   `include/globals.rs` 7156 declaration is the name.
9. **C 875's `metrics->lbearing > lbearing` is written `((*metrics).lbearing as c_float) > lbearing`**
   (and C 877's pair likewise): Rust parses `x as T < y` as generic arguments, so the cast needs
   parentheses — the one syntax the frozen reading caught (Evidence ①).
10. **`include/w32font.rs` is included directly although C 47-50 does not name it**: C's
   `w32dwrite.c` reaches `struct uniscribe_font_info`/`FONT_HANDLE` through `w32font.h`, and
   `include/w32font.rs` is that mirror (it includes `font.rs` itself, C 23's `#include "font.h"`);
   `include/lisp.rs` and `include/conf_post.rs` are included for the chain every one of the four
   headers needs (`Lisp_Object`, `eassert`, `SAFE_ALLOCA`, `defsubr`, `DebPrint`) — the
   `src/w32font.rs` 694-708 reading.
11. **The ten `typedef interface X X;` lines (C 154-163) have no Rust item**: `interface` is
   `<objbase.h>`'s `struct`, so each is C's `typedef struct X X;` — a tag declaration, and Rust's
   one item per name is the definition below (no forward declarations exist).  C's lines are kept
   as comments so the order reads.
12. **The line count**: C 1399 → Rust 2415 (**72.6%**).  The reasons are the three stand-in
   sections (C 38-45's `<windows.h>`/COM names and the ~250-line expansion of the file's own
   490-line DWrite/D2D block into per-item `#[cfg]`s and `/// C:` citations), the both-arms `#[cfg]`s,
   and the per-block `/// C:` citations; the tolerance row is Owed 3.

## Owed

1. **`lib/include/math.rs` needs a `round` row** (`math.h`'s `double round (double)`): this unit
   declares it in place (P-88 (1)) because the mirror carries none.  One declaration.
2. **`include/font.rs` needs the `harfbuzz_font_driver` row** (`font.h` 980's
   `extern struct font_driver harfbuzz_font_driver;`, defined in `src/w32uniscribe.rs`): declared in
   place here for the same reason.  `docs/ports/w32font.md`'s Owed list names the same mirror gap
   (`register_font_driver`, `font_make_object`).
3. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32dwrite`, tolerance ≥ **0.726**,
   basis `total`, reason from Deviation 12.  That file is not in this lane's write set, so
   `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑤ records.
4. **The Windows round that flips this arm** must register this file's arm set as `have_ntgui` +
   `windowsnt` + `have_window_system` **+ `harfbuzz`**, and it must decide the two unbuildable arms:
   the `mingw_w64` half (whose types C takes from `<dwrite_3.h>`/`<d2d1.h>`, Deviation 2) and the
   `<windows.h>`/`<dwrite_3.h>`/`<d2d1.h>` stand-in records (Deviation 5's UNVERIFIED values).
5. **The layout assertions for the stand-in records** (`GUID`, `LOGFONT`/`LOGFONTW`, `SIZE`, `RECT`,
   `DWRITE_FONT_METRICS`/`DWRITE_GLYPH_METRICS`/`DWRITE_GLYPH_RUN`/`DWRITE_COLOR_GLYPH_RUN`,
   `D2D1_POINT_2F`/`D2D1_BEZIER_SEGMENT`, the ten `*Vtbl`/`*` vtable records and the four enums'
   widths) and the measured values of `S_OK`/`E_NOINTERFACE`/`E_INVALIDARG`/`SRCCOPY`/`FLT_MAX`,
   once a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's readings;
   `x86_64-w64-mingw32-gcc` is absent here).
6. **The mirror gates in `include/w32font.rs` and `include/w32term.rs`** (their Windows halves are
   `#[cfg (feature = "windowsnt")]` at 93/111/157/190 and at 1057, whose block declares
   `get_frame_dc` at 1061): this file is compiled by the
   Cygwin GUI build as well (`HAVE_NTGUI` without `WINDOWSNT`), and in that configuration those
   mirrors supply nothing, so the unit's `struct uniscribe_font_info`/`struct w32font_info`/
   `get_frame_dc` references have no declarer.  The same holds for the landed `src/w32font.rs`,
   `src/w32uniscribe.rs`, `src/w32term.rs`, so it is a mirror-level decision rather than this
   lane's: named here because this unit depends on it.
7. **`docs/baseline/port-lines-pending.tsv`, `docs/baseline/feature-coverage.tsv` and
   `docs/status.md`**: the integrator's regeneration and running-log entry for this revision (the
   `docs/ports/w32uniscribe.md` Owed 7/8 rows); `docs/port-map.md`'s `src/w32dwrite.c` row still
   reads 27 lines until `tools/gen-port-map.py` is re-run.
8. **`docs/ports/macro-body-notes.tsv`'s three new rows** are this lane's only write outside the
   two files it owns (appended, never rewritten): they are the machine-checkable half of
   Deviation 4 and Evidence ⑤.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, GUI frame, `HDC`, DirectWrite factory, geometry sink or redisplay loop was
  involved.  There is no behavioural test, no differential run, no ERT and no device evidence.
- **The arm reading is not a Windows compile.**  This machine has no `windows.h`/`dwrite_3.h`/
  `d2d1.h` and no mingw toolchain, so Evidence ②③④ are `rustc` readings of the *materialized
  mirror graph* under four `--cfg`s; one error remains in it, anchored in a mirror copy, and
  "0 in the unit" means that no error's primary anchor is one of this unit's lines — not that the
  unit *links*, and not that the arm is a working Windows configuration.  The narrower arms'
  readings carry the `compile_error!`-stops-typechecking limit Evidence ③ states.
- **The stand-in records and values stay unmeasured** (Owed 4/5): every one is `UNVERIFIED`, with
  the documented SDK text and the landed sibling sections as their only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 3).  Nothing in this ledger claims
  the file's checklist is non-empty (P-86 (2)) or that the port is complete.
