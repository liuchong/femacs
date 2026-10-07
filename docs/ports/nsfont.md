# emacs/src/nsfont.m (1765 lines)

> **This file is the GNUstep font backend, and the frozen configuration does not compile
> it** (`docs/baseline/frozen-build-sources.txt`, `docs/status.md` 3.31: darwin-arm64 with no
> window system).  Two independent facts decide what its port is:
>
> 1. `configure.ac` 2816 sets `ns_fontfile=nsfont.o` for the GNUstep configuration and 2762
>    sets `ns_fontfile=macfont.o` for the Cocoa one; src/Makefile.in 480 lists both.  The two
>    are **build alternatives**, each defines its own `font_driver` (`nsfont_driver` 1726,
>    `macfont_driver` in macfont.m), and both are in `docs/port-map.md`'s Swift table.
>    A single Swift module cannot contain the two.
> 2. src/nsfont.m has **no preprocessor conditionals at all** (the only `#`-directive pair in
>    the file is the `#if 0` block at 659-662): it is unconditionally `NS_IMPL_GNUSTEP` code,
>    from its first line ("Font back-end driver for the GNUstep window system") to its
>    `GSFontEnumerator`/`DPSmoveto`/`GSShowGlyphs` calls.  There is no Cocoa arm to take.
>
> The unit `femacs/platform/macos/nsfont.swift` therefore carries **the whole file** - the
> descriptor / entity / charset / script layer (35-830), the driver entry points, the glyph
> and metrics caches, the shaping, the `font_driver` record and `syms_of_nsfont` - and its
> C-ABI boundary is the core's (the lane's rule of `docs/status.md` 3.144: Swift speaks to the
> Rust port only through the C ABI).  **Round 83 lands the two bodies the record kept out**:
> `nsfont_draw` (1151) and `ns_dump_glyphstring` (1711) dereference `struct glyph_string`,
> and this tree still declares that record only as the opaque placeholder `femacs/src/font.rs`
> 739 (P-70, because font.c only ever passes a pointer to it) - so every member the two bodies
> read is a reader entry of the `glyph_string_*` family this unit now declares beside its
> neighbours (`## ABI` (2g)), which is what the `## Deferred` section below records as the
> blocker's resolution.  Everything in the file is ported.  (Round 1 recorded the core's `font.h` mirror as 84 lines of "lands with font.c";
> that is stale: `femacs/src/include/font.rs` (3.158/3.159) and `femacs/src/font.rs` carry
> `font`, `font_metrics`, `font_driver`, `font_bitmap` and `Lisp_Fwd`, and this unit names
> them.)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 36

Ported (36):

- [x] ns_done_font_data  (line 122, static)
- [x] ns_get_font_data  (line 129, static)
- [x] ns_font_descs_match_p  (line 225, static)
- [x] ns_get_family  (line 307, static)
- [x] ns_spec_to_descriptor  (line 325, static)
- [x] ns_descriptor_to_entity  (line 424, static)
- [x] ns_fallback_entity  (line 491, static)
- [x] ns_char_width  (line 499, static)
- [x] ns_ascii_average_width  (line 519, static)
- [x] ns_charset_covers  (line 552, static)
- [x] ns_lang_to_script  (line 582, static)
- [x] ns_otf_to_script  (line 600, static)
- [x] ns_registry_to_script  (line 611, static)
- [x] ns_get_req_script  (line 632, static)
- [x] accumulate_script_ranges  (line 673, static)
- [x] ns_script_to_charset  (line 688, static)
- [x] ns_get_covering_families  (line 728, static)
- [x] ns_findfonts  (line 785, static)
- [x] nsfont_get_cache  (line 848, static)
- [x] nsfont_list  (line 868, static)
- [x] nsfont_match  (line 879, static)
- [x] nsfont_list_family  (line 888, static)
- [x] nsfont_open  (line 912, static)
- [x] nsfont_close  (line 1049, static)
- [x] nsfont_has_char  (line 1077, static)
- [x] nsfont_encode_char  (line 1086, static)
- [x] nsfont_text_extents  (line 1108, static)
- [x] nsfont_draw  (line 1151, static) — landed (round 83) as `fileprivate func nsfont_draw
      (_:_:_:_:_:_:) -> Int32`, the `font_driver.draw` slot (C 1740): every `struct glyph_string`
      member is a reader entry, C's `alloca` is `withUnsafeTemporaryAllocation` (the C 1491/1514
      reading), and C's nil-font arm is `FRAME_FONT`
- [x] ns_font_shape  (line 1214, static)
- [x] nsfont_shape  (line 1464, static)
- [x] ns_uni_to_glyphs_1  (line 1576, static)
- [x] ns_uni_to_glyphs  (line 1622, static)
- [x] ns_glyph_metrics  (line 1664, static)
- [x] ns_dump_glyphstring  (line 1711, exported; declared src/nsterm.h 1118) — landed (round 83)
      as `@_cdecl("ns_dump_glyphstring")`: the same reader family, plus this round's two new
      reads (`glyph_row_overlapping_p`, the indexed `glyph_u_ch`), `fprintf`/`putc`/`stderr`
- [x] syms_of_nsfont  (line 1744) -> `@_cdecl("syms_of_nsfont")`
- [x] syms_of_nsfont_for_pdumper  (line 1762, static)


The list above is the one `tools/extract-abi.py` now produces (36 functions, 0 subrs); its
round-1 blind spot - the six signatures whose return type is split across lines - was fixed in
`tools/cinventory.py` at status 3.160, so the six no longer carry a hand-written note.  The
extractor has no Swift reader yet (status 3.160 item 2), so this ledger is still hand-written
and only the C side of the list is generated.

## Macros defined — 3
- [x] INVALID_GLYPH  (line 296, body ec4cc6079e5c) -> `fileprivate let INVALID_GLYPH: UInt32`
- [x] NSFONT_TRACE  (line 47, body b6589fc6ab0d) -> `fileprivate let NSFONT_TRACE: Int32 = 0`; the
      `if (NSFONT_TRACE)` bodies are dead text that still compiles, as in C
- [x] RIGHT_TO_LEFT_P  (line 1304, body 005fcb61ed11) -> `#define RIGHT_TO_LEFT_P permutation`,
      a textual alias for the local pointer of `ns_font_shape`; the Swift body keeps the
      `permutation != nil` expressions the alias produced, and the macro is recorded here

## Types and their fields — 2
- [x] gs_font_data: `int specified, enum gs_font_slant slant, enum gs_font_weight weight,
      enum gs_font_width width, bool monospace_p, char* family_name` (lines 109-116) ->
      `struct gs_font_data`.  This is the file's own type and never crosses the ABI; Swift's
      raw-value enums are **1 byte** here, so its layout differs from C's (measured in
      `## Evidence`) and the mirror keeps Swift's types on purpose
- [x] ns_glyph_layout: `NSRange comp_range, NSUInteger string_index, CGFloat advance_delta,
      CGFloat baseline_delta, CGFloat advance, NSGlyph glyph_id` (lines 51-71) ->
      `struct ns_glyph_layout`; the mirror's `sizeof` 52 has a **56-byte stride**, which is
      what the `alloca` arrays of `ns_font_shape` index by, exactly as C's does

## File-scope objects — 1
- [x] static NSString ascii_printable  (line 516, static) -> `fileprivate var ascii_printable:
      NSString?`, created once by `ns_ascii_average_width` exactly as C does

Plus (defined by this file, not a "type or field" the extractor lists): `nsfont_driver`
(1726) -> `fileprivate let nsfont_driver: UnsafeMutablePointer<font_driver>`, and
`Vns_reg_to_script` (1746-1748's `DEFVAR_LISP`) -> `fileprivate var Vns_reg_to_script`.  Both
are C data objects Swift cannot emit as symbols; see `## ABI` (1) and (3).

## Decisions

**The file's provenance and target.**  This is the GNUstep backend (the header comment says
so, `configure.ac` 2762/2816 chooses between it and `macfont.m`, and the file has no
conditionals).  The lane's rule - "the lane's target is the Cocoa/AppKit build; arms for other
targets are recorded, not transcribed" - cannot apply to a file that has no Cocoa arm, so the
port follows the *workable* reading: transcribe everything a Swift translation unit can carry
against the macOS SDK, and name the rest.  Two consequences are for the lane, not this file:

1. **`nsfont.swift` and `macfont.swift` are alternatives, not siblings.**  They define the
   same kind of global driver record; only one may be linked, and the "9 files are one module"
   plan cannot hold for this pair.
2. **Some callees have no provider in this repository.**  `GSFontEnumerator`
   (`[GSFontEnumerator sharedEnumerator]`, line 790), `[NSTextStorage setFont:]` and
   `DPSmoveto`/`GSShowGlyphs` (1180-1181) are GNUstep-only: Apple's SDK has none of them, so
   the definitions live in the GNUstep libraries and the lane reaches them as C-ABI entries of
   their own names (`## ABI` (2e)).  `DPSmoveto`/`GSShowGlyphs` are the two the deferred
   `nsfont_draw` would need.

**Mirrors of the records this file dereferences.**  A Swift mirror of a C record is measured
against the *windowed* declarations this file's target implies (`clang -std=gnu23
-DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1 -DNS_IMPL_GNUSTEP=1`, `## Evidence`), and each mirror
records the C declaration, the owner mirror in this tree when one exists, and the measured
numbers.  The lane rule that a mirror uses **raw integers, never Swift raw-value enums** is
what keeps these layouts C's (`gs_font_data`, the file's own type, is the exception that
proves it - it never crosses the ABI and keeps its enums).

| mirror | C declaration | owner mirror in this tree | Swift | C |
| --- | --- | --- | --- | --- |
| `Lisp_Char_Table` (+`contents64`) | lisp.h 2053-2082 | - (this file's own need) | defalt 8, extras 552, size 560 | defalt 8, extras 552, sizeof 552 |
| `XCharStruct` | nsgui.h 46-53 | - | sizeof 20 | sizeof 20 |
| `ns_glyph_layout` | this file 51-71 | - | sizeof 52, stride 56 | stride 56 |
| `gs_font_data` (enums kept) | this file 109-116 | - | sizeof 16, family_name 8 | sizeof 32, family_name 24 |
| `font` | font.h 275-404 (windowed) | `femacs/src/include/font.rs` (frozen: 184) | sizeof 216, props 8, driver 208 | sizeof 216, props 8, driver 208 |
| `font_metrics` | font.h 414-417 | `femacs/src/font.rs` 196 | sizeof 10, 0/2/4/6/8 | sizeof 10, 0/2/4/6/8 |
| `font_bitmap` | font.h 419-430 | `femacs/src/font.rs` 208 | size 36, stride 40, buffer 16 | sizeof 40, buffer 16 |
| `font_driver` | font.h 589-821 (windowed) | `femacs/src/font.rs` 266 (frozen: 208, no `draw`) | sizeof 232, all 30 offsets | sizeof 232, all 30 offsets |
| `Lisp_Fwd`/`Lisp_Fwd_u` | lisp.h 3118-3133 | - (the `nsselect` unit's type, same spelling) | sizeof 16, u 8 | sizeof 16, u 8 |
| `nsfont_info` | nsterm.h 824-849 (`NS_IMPL_GNUSTEP`) | - (platform record) | sizeof 296, name 216, glyphs 280, metrics 288 | same |
| `ns_display_info` (prefix) | nsterm.h 853-928 | - | prefix sizeof 24, name_list_element 16 | full record 280, name_list_element 16 |

`font_bitmap`'s Swift `size` is 36 while C's `sizeof` is 40: C rounds the trailing 4 bytes up
to the record's 8-byte alignment, and Swift's `size` omits what its `stride` (40) includes -
the same relation `Lisp_Char_Table` has for the opposite reason.  Every *offset* is the C
compiler's, which is the ABI fact; only `ns_display_info`'s mirror stops short of C's record
because the tail needs the Objective-C header's types and this file reads only
`name_list_element`.

**`@convention(c)` cannot name a Swift struct's pointer, so the driver record's slots are
raw.**  `font_driver`'s members are C function pointers, and a `@convention(c)` function type
in a file that imports AppKit requires *Objective-C-representable* parameter types; a pointer
to a Swift struct is not one, and the compiler rejects
`@convention(c) (UnsafeMutablePointer<font>?) -> Void` outright (Evidence).  The record
therefore spells every `struct font *`/`struct font_metrics *`/`struct font_bitmap *`/
`struct face *`/`struct glyph_string *` slot as `UnsafeMutableRawPointer?` with C's prototype
in the comment, and the initialiser reinterprets the pointer at the three slots this file
fills (`close_font`, `encode_char`, `text_extents`).  The functions themselves keep their
typed signatures, so `nsfont_open`/`nsfont_close`/`nsfont_encode_char`/`nsfont_text_extents`
read C's own members by name.

**C constructs whose Swift spelling is not C's text, each measured/documented:**

- `DEFSYM (Qfoo, "foo")` is `#define DEFSYM(sym, name) /* empty */` (src/lisp.h 2394): the
  seven calls in `syms_of_nsfont` carry no statement, and the comment says so (the marker is
  for make-docfile).  The symbols come from the generated globals.
- `DEFVAR_LISP ("ns-reg-to-script", Vns_reg_to_script, doc)` (1748-1749) builds a
  function-local `static struct Lisp_Fwd` (lisp.h 3495-3500) and hands it to `defvar_lisp`.
  Swift has no function-local statics, so the record is a fileprivate `Lisp_Fwd` global and
  `syms_of_nsfont` fills it and calls `defvar_lisp` - the `docs/ports/nsselect.md` shape.
- `VECSIZE (struct nsfont_info)` is `(sizeof (type) - header_size + word_size - 1) /
  word_size` (lisp.h 2015-2016); the core mirror spells the macro as a `const fn` over the
  type's size (`femacs/src/include/lisp.rs` 2925), and this lane declares that entry rather
  than re-deriving C's arithmetic.
- `pdumper_do_now_and_after_load (hook)` (pdumper.h 92-99) is `INLINE` with a
  `HAVE_PDUMPER` body; it is a core C-ABI entry here.
- The sixteen `LGSTRING_*`/`LGLYPH_*` macros of `nsfont_shape` (composite.h 252-325) are
  core entries too.  Their signatures are C's own expressions: `LGSTRING_GLYPH_LEN`'s return
  is the `ptrdiff_t` its callers compare against, `LGLYPH_CHAR` returns `EMACS_INT`
  (`XFIXNUM`), `LGLYPH_SET_CODE`'s value is `unsigned long` (`INT_TO_INTEGER`), the index
  arguments are `LGLYPH_IX_*`'s `int`, and `LGLYPH_SET_CHAR`'s value is the `UTF32Char` of its
  only call site (1520).
- `alloca (len * sizeof *mb_buf)` (1491) and `alloca (sizeof (struct ns_glyph_layout) *
  glyph_len)` (1514) are `withUnsafeTemporaryAllocation` - the scoped, stack-first temporary
  storage.  Capacities are `max (_, 1)` because C's `alloca (0)` never yields NULL while a
  zero-capacity Swift buffer has no `baseAddress` to hand the SDK; C's `alloca (0)` argument
  is the only case that differs, and it is a case C never reaches with a non-empty string.
- `memset (metrics, 0, sizeof (struct font_metrics))` is an all-zero `font_metrics` value.
- `lrint` takes a `double` in C and promotes its `float`/`int` arguments; the port writes the
  promotion (`Double (...)`) at the five sites where the argument is a `float` field.
- `font->props[FONT_NAME_INDEX] = …` indexes C's fixed 17-slot array; the port re-reads
  `props` as `Lisp_Object *` (`withUnsafeMutablePointer` + `withMemoryRebound`) and keeps C's
  index expression instead of hard-coding slot numbers.
- `[nsfont screenFont]` is obsoleted in Swift 3 and renamed `screen`; `NSBoldFontMask` /
  `NSItalicFontMask` are `NSFontTraitMask.boldFontMask` / `.italicFontMask` (their values are
  the SDK's 2 and 1; the C build's are GNUstep's, and this is the one place the two SDKs could
  differ - the C text is a `char` field either way).  `screen` was verified to be the same
  property (the object answers `screenFont`; Evidence).
- `font = (struct font *) font_info` (952) is not a conversion: `struct nsfont_info`'s first
  member *is* the `struct font`, so the port writes the `font->…` members as
  `font_info.pointee.font.…` and drops the redundant alias.
- Two dead assignments in the reference are kept as the calls they are: `tem = AREF
  (font_entity, FONT_ADSTYLE_INDEX)` (930) and the `family`/`userFixedPitchFontOfSize:`
  fallback (931-933) are written and never read by `nsfont_open`.  The port keeps the calls
  (they are the reference's), notes that nothing reads the values, and does not invent a use.
- `goto gout` in `ns_font_descs_match_p` (298-304) is a labelled `do` block with `break
  gout`: every branch leaves through the same two `ns_done_font_data` calls and the same
  `return match_p` (PORTING.md 2.2 forbids an early `return` that would skip the cleanup).
- `ns_uni_to_glyphs_1` (1576-1620) wraps `glyphAtIndex:` in `@try`/`@catch`; Swift cannot
  catch an Objective-C exception, so the call is plain and that guard is a recorded deviation.
- `ns_font_shape`'s `if (!(textStorage && layoutManager && textContainer)) emacs_abort ()`
  (1243-1244) has no Swift spelling (the SDK's initialisers do not return nil), as does
  `nsfont_shape`'s `if (!string) return Qnil` (1497-1498); both are recorded, not transcribed.
- Mechanical spellings: `max`/`min` -> Swift's (C's macros are plain comparisons);
  `NSString`'s `isEqualToString:` -> `isEqual(to:)`; `NSFontAttributeName` ->
  `NSAttributedString.Key.font`; the trait dictionary's keys keep their SDK names
  (`NSFontDescriptor.AttributeKey.family`, `…TraitKey.slant/weight/width`);
  `[NSNumber numberWithFloat:]`/`numberWithBool:` -> `NSNumber(value:)`;
  `[NSCharacterSet characterSetWithRange: NSMakeRange (0, 127)]` -> `NSMutableCharacterSet` +
  `addCharacters(in: NSRange(location: 0, length: 127))` (a range, not a string set); the
  `%s`-with-a-stack-buffer in `ns_ascii_average_width` goes through
  `withUnsafeBufferPointer` so the pointer lives for the call; `intern ([family UTF8String])`
  goes through `withCString` (C's transient C string); C's stdio calls are declared with
  `@_silgen_name` (`printf`/`fprintf`/`fputs` are "unavailable: Variadic function is
  unavailable" as imported by Swift); `pD` (src/lisp.h 150, `"t"`) is the core's entry and the
  `"%"pD"d"` formats are composed at runtime from the same pieces.

**The driver record's export: a getter, not a data symbol.**  Swift cannot emit a C data
symbol, so `nsfont_driver` (1726) is a Swift global whose address is handed out by
`@_cdecl("nsfont_driver") func nsfont_driver_getter () -> UnsafeMutableRawPointer?` - the
route the lane adopted (rc 3.166, decision 2) and the shape `docs/ports/nsselect.md`'s
`## ABI` (3) item 3 uses for PORTING.md P-31's records.  C's `&nsfont_driver` becomes a call
to that entry on the Rust side; inside this unit the Swift global is still read directly.

**The `#if 0` at 659-662 and `#define RIGHT_TO_LEFT_P`** are recorded: the first is dead text
(the `iso10646_1` rewrite in `ns_get_req_script`), the second is a textual alias whose only
assignment is `permutation = NULL` (1303), so `RIGHT_TO_LEFT_P` is constant-false and every
branch behind it is dead text the port still writes (`permutation != nil`).

## Deferred - **closed by round 83**

Two bodies, one blocker.  The list the integrator asked for, in the requested shape:

| record | C definition | which functions need it | how long missing |
| --- | --- | --- | --- |
| `struct glyph_string` | dispextern.h **1362-1507**, inside `#ifdef HAVE_WINDOW_SYSTEM`; `femacs/src/font.rs` 739 declares it **opaque** (P-70: "font.c only ever passes a pointer to it") | `nsfont_draw` (1151) reads `s->first_glyph->type` (through `struct glyph`, mirrored at `femacs/src/include/dispextern.rs` 289), `s->char2b`, `s->font`, `s->f`, `s->face`, `s->width`, `s->hl`; `ns_dump_glyphstring` (1711) reads `s->nchars`, `s->x`, `s->y`, `s->left_overhang`, `s->right_overhang`, `s->row->overlapping_p`, `s->background_filled_p`, `s->first_glyph[i].u.ch` | since round 1 (2026-10-04) |

Two secondary records the same two bodies would then need, both of which have owner mirrors
for the **frozen (non-windowed)** configuration only, with the `HAVE_WINDOW_SYSTEM` members
absent (so a Swift mirror here would have to be measured for the NS form, as `struct font`'s
was): `struct face` (`femacs/src/font.rs` 464; C's windowed form adds `gc`, `stipple`, `extra`)
and `struct glyph_row`'s `overlapping_p` bit (`femacs/src/include/dispextern.rs` 767, whose own
comment says `HAVE_WINDOW_SYSTEM`'s `clip` is behind its feature).

`nsfont_draw`'s driver slot (`font_driver.draw`) was written `nil` in `nsfont_driver` for this
reason - it was the **one** member of the record whose C value the port could not yet supply,
and the ledger said so rather than pointing it at a stand-in.  **Round 83 closes both halves
of the hole**: the slot now holds C 1740's own value (`draw: nsfont_draw`), and the record is
read through entries instead of mirrored - `glyph_string_*` for the record's members,
`glyph_row_overlapping_p` for the row bit, the indexed `glyph_u_ch` for `first_glyph[i].u.ch`
(the index lives in that entry because `struct glyph`'s stride is the core mirror's), and
`NS_FACE_BACKGROUND`/`NS_FACE_FOREGROUND`/`FRAME_*_COLOR` for the two *secondary* records
(`struct face`, and the frame's colour slots) the paragraph above lists - so none of the three
records needs a windowed Swift mirror after all.

## ABI

### (1) C symbols this file defines and exports

| C name | C prototype | where | how |
| --- | --- | --- | --- |
| `syms_of_nsfont` | `void (void)` | 1744, declared src/font.h 986 | `@_cdecl("syms_of_nsfont")` - `nm -g` shows `T _syms_of_nsfont` |
| `ns_dump_glyphstring` | `void (struct glyph_string *)` | 1711, declared src/nsterm.h 1118 | `@_cdecl("ns_dump_glyphstring")` (round 83) - `nm -g` shows `T _ns_dump_glyphstring` |
| `nsfont_driver` | `struct font_driver const` | 1726, declared src/font.h 985 | a C data symbol Swift cannot emit - **exported as a getter of the same C name**: `@_cdecl("nsfont_driver") func nsfont_driver_getter() -> UnsafeMutableRawPointer?` (see (3) item 1) |

`nm -g` on the object shows two non-mangled exports, `T _syms_of_nsfont` and
`T _nsfont_driver` (the latter is the getter that carries the data symbol's C name).

### (2) C symbols this half calls

**a. Symbols the reference defines out of line** (the ported core defines them): `xfree`
(lisp.h 5543), `xstrdup` (5551), `xlispstrdup` (5553), `xzalloc` (5537), `xmalloc` (5535),
`font_make_entity` (font.h 842), `font_make_object` (846), `Ffont_xlfd_name` (globals.h 4765),
`get_frame_param` (lisp.h 5120), `register_font_driver` (font.h 896), `defvar_lisp` (lisp.h
3472), `intern` (4757), `debug_print` (4702), `assq_no_quit` (4272),
`Fcons`/`Fmemq`/`Fnreverse` (globals.h 4560/5101/5161), `list1` (4486), `map_char_table`
(declared lisp.h 4689, defined chartab.c 842; it takes a function pointer), `list_length`
(4243), `memory_full_up` (4433, `AVOID` -> `Never`), `emacs_abort` (5302, `AVOID` -> `Never`),
`block_input`/`unblock_input` (`INLINE`, src/blockinput.h), `strcmp`/`strncmp`/`lrint` (libc).

**b. Header-only constructs** (macros, `INLINE` functions, enumerators, generated-globals
entries; the core exports each C-ABI entry):

| C name | reference form | where |
| --- | --- | --- |
| `NILP`, `CONSP`, `FIXNUMP` | `#define NILP(x) lisp_h_NILP (x)` and the two like it | src/lisp.h 450, 446, 449 |
| `EQ`, `SYMBOLP` | INLINE functions | src/lisp.h 1315, 1075 |
| `XCAR`, `XCDR` | `lisp_h_XCAR`/`lisp_h_XCDR` macros | src/lisp.h 455, 456 |
| `XSETCDR` | `INLINE void XSETCDR (Lisp_Object, Lisp_Object)` | src/lisp.h 1515 |
| `AREF`, `ASET`, `XFIXNUM`, `XFIXNAT`, `SYMBOL_NAME`, `SSDATA`, `SBYTES`, `make_fixnum`, `intern` | INLINE functions / macros | src/lisp.h 1964, 1978, 1268, 3246, 2350, 1658, 1697, 1211, 4757 |
| `VECSIZE`, `build_unibyte_string` | macros / INLINE of src/lisp.h 2015-2016, 4556 | - |
| `XFONT_OBJECT`, `CHECK_FONT_GET_OBJECT` | INLINE functions | src/font.h 539, 553 |
| `XCHAR_TABLE` | `INLINE struct Lisp_Char_Table *XCHAR_TABLE (Lisp_Object)` | src/lisp.h 2090 |
| `FONT_TYPE_INDEX` … `FONT_OBJLIST_INDEX` (13), `FONT_NAME_INDEX`, `FONT_FULLNAME_INDEX` | `enum font_property_index` members | src/font.h 70-190 |
| `FONT_SPACING_PROPORTIONAL`, `FONT_SPACING_MONO` | enumerators | src/font.h 408-410 |
| `FONT_WEIGHT_SYMBOLIC`, `FONT_SLANT_SYMBOLIC`, `FONT_WIDTH_SYMBOLIC`, `FONT_INVALID_CODE` | macros over `AREF` / a `#define` | src/font.h 205-213, 585 |
| `FONT_SET_STYLE` | `#define FONT_SET_STYLE(font, prop, val) ASET (…)` | src/font.h 236 |
| `Lisp_Fwd_Obj` | `enum Lisp_Fwd_Type` member | src/lisp.h 532 |
| `pdumper_do_now_and_after_load` | INLINE with a `HAVE_PDUMPER` body | src/pdumper.h 92-99 |
| `LGSTRING_HEADER`/`_FONT`/`_GLYPH_LEN`/`_GLYPH`, `LGSTRING_SET_GLYPH`, `LGLYPH_NEW`, `LGLYPH_CHAR`, `LGLYPH_SET_FROM`/`_TO`/`_CHAR`/`_CODE`/`_WIDTH`/`_LBEARING`/`_RBEARING`/`_ASCENT`/`_DESCENT` | macros | src/composite.h 252, 258, 272-273, 274, 299, 302, 313-325 |
| `pD` | `#define pD "t"` | src/lisp.h 150 |
| `Qnil`, `Qns`, `Qitalic`, `Qoblique`, `Qreverse_italic`, `Qreverse_oblique`, `Qnormal`, `Qbold`, `Qlight`, `Qcondensed`, `Qexpanded`, `Qiso10646_1`, `QCscript`, `QClang`, `QCotf`, `QL2R`, `QR2L`, `Qfontsize` | generated globals.h `#define Qfoo builtin_lisp_symbol (iQfoo)` | `docs/baseline/globals/symbols.tsv` |
| `printf`, `fprintf`, `fputs` | libc, variadic (Swift imports them as unavailable) | - |

**c. Core variables this half reads** (C: `globals.f_*`, owned by the unit that `DEFVAR`s
them, docs/status.md 3.29; the core's Rust units export them as C data symbols, so the lane
imports them): `Vchar_script_table` (character.c), `Votf_script_alist` (fontset.c).

**d. This file's own variables**: `Vns_reg_to_script` is `DEFVAR_LISP`'d here (1746-1748), so
this unit owns its storage - a Swift global with C's initial `Qnil`, because Swift cannot emit
C data symbols; `syms_of_nsfont` registers it through `defvar_lisp` with the fileprivate
`Lisp_Fwd`.  `nsfont_driver` is the same situation (see (3)).

**e. This lane's own header constructs** (declared here, defined by the unit named):

| C name | prototype / form | declared | defined by |
| --- | --- | --- | --- |
| `FRAME_DISPLAY_INFO` | `#define FRAME_DISPLAY_INFO(f) ((struct ns_display_info *) (f)->output_data.ns->display_info)` | src/nsterm.h 1032 | the port of src/nsterm.m |
| `stringWithLispString` | `+[NSString stringWithLispString: (Lisp_Object)]` | src/nsterm.h 367 | the port of src/nsfns.m (4104) |
| `setFont` | `-[NSMutableAttributedString setFont: (NSFont *)]` | GNUstep's class | the GNUstep libraries |

**f. GNUstep-only entries** (no provider in this repository; Apple's SDK has none of them):
`GSFontEnumerator_sharedEnumerator`, `GSFontEnumerator_availableFontDescriptors` (the class
and its two messages, 790/799), `setFont` (1254), and `DPSmoveto`/`GSShowGlyphs` (C
1205-1206's two calls, which round 83's `nsfont_draw` is the first body to need - they are
declared now).  A Swift port can *declare* them; only the GNUstep libraries can define them.

**g. The `struct glyph_string` reader family round 83 adds** (the record is the tree's opaque
placeholder, so each read is its own entry - the shape `nsterm.swift` 3578-5016 spells; this
unit declares them because it compiles alone):

| entry | C member | C line | state |
| --- | --- | --- | --- |
| `glyph_string_first_glyph`, `glyph_type` | `s->first_glyph->type` | 1161 | **open** (the core's) |
| `glyph_string_font`, `glyph_string_f`, `glyph_string_face` | `s->font`, `s->f`, `s->face` | 1165-1169 | **open** |
| `glyph_string_width`, `glyph_string_hl`, `glyph_string_char2b` | `s->width`, `s->hl`, `s->char2b` | 1176-1193 | **open** |
| `glyph_string_nchars`, `glyph_string_x`, `glyph_string_y` | `s->nchars`, `s->x`, `s->y` | 1716-1717 | **open** |
| `glyph_string_left_overhang`, `glyph_string_right_overhang` | `s->left_overhang`, `s->right_overhang` | 1717 | **open** |
| `glyph_string_row`, `glyph_row_overlapping_p` | `s->row`, `s->row->overlapping_p` | 1718 | **open** (the row read is new here) |
| `glyph_string_background_filled_p` | `s->background_filled_p` | 1718 | **open** |
| `glyph_u_ch` | `s->first_glyph[i].u.ch` | 1720 | **open** (new; the index is in the entry) |
| `COMPOSITE_GLYPH`, `DRAW_CURSOR` | the two enumerators | 1161/1183 | **open** |
| `FONT_HEIGHT`, `FONT_BASE` | `#define FONT_HEIGHT/FONT_BASE` (src/font.h 244-245) | 1173-1182 | **provided** (the core's) |
| `NS_FACE_BACKGROUND`, `NS_FACE_FOREGROUND`, `FRAME_BACKGROUND_COLOR`, `FRAME_CURSOR_COLOR`, `FRAME_FOREGROUND_COLOR`, `FRAME_FONT` | the face/frame reads | 1167-1198 | **provided** (the port of src/nsterm.m) |
| `colorWithUnsignedLong` | `+[NSColor colorWithUnsignedLong:]` (src/nsterm.h 361) | 1185/1197 | **provided** |
| `NSRectFill` | AppKit's `NSGraphics.h` | 1189 | **provided** (the SDK) |
| `putc` | `stdio.h` | 1720-1721 | **provided** (libSystem) |

### (3) What the lane still has to settle (flagged for the integrator)

1. **`nsfont_driver` is a C data symbol Swift cannot emit** (`@_cdecl` refuses `var`;
   `@_silgen_name` on a `var` imports) - **settled at rc 3.166, decision 2**: the lane takes the
   *getter* route, so this unit exports `@_cdecl("nsfont_driver") func nsfont_driver_getter ()
   -> UnsafeMutableRawPointer?`, returning the process-stable address of the Swift global that
   holds the record (nothing frees it, as C's `static` storage is never freed).  The Rust side
   writes C's `register_font_driver (&nsfont_driver, f)` and src/nsfns.m 1316/3001's two
   `&nsfont_driver` sites as calls to this entry - the same device
   `docs/ports/nsselect.md`'s `## ABI` (3) item 3 records for PORTING.md P-31's five `S…`
   records.  **Deviation, named here as the decision requires**: C is a *data* symbol and the
   port is a *function* of the same name returning its address; the macOS target's frozen
   configuration is what makes the substitution necessary (Swift has no data-symbol export).
2. **`struct glyph_string`** is what keeps `nsfont_draw` and `ns_dump_glyphstring` unwritten
   (see `## Deferred`): this tree's owner declares it opaque, and this lane does not invent a
   substitute.
3. **The three slots of `font_driver` that name a Swift struct's pointer** (`close_font`,
   `encode_char`, `text_extents`) are the only places the port converts a pointer type, because
   `@convention(c)` cannot spell `UnsafeMutablePointer<font>` (see Decisions).  If the lane
   would rather those three functions take the raw pointer at their boundary, that is a
   lane-level convention to fix once for all nine Swift files, not a change to any other line.

## Evidence

- `swiftc --version` -> `swift-driver version: 1.168.6 Apple Swift version 6.4
  (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`, `Target: arm64-apple-macosx26.0`.
- `cd /Users/liu/projects/src/github.com/liuchong/femacs && swiftc -parse-as-library -c -o
  /tmp/nsfont.o femacs/platform/macos/nsfont.swift` -> **exit 0**, no warnings, **no
  `-swift-version` flag** (swiftc 6.4's default language mode is 5).  With
  `-swift-version 6` the same file reports **7 `[#MutableGlobalVariable]` errors** - the
  nonisolated global mutable state C's globals are: `Vchar_script_table` (538),
  `Votf_script_alist` (539), `Vns_reg_to_script` (545), `ascii_printable` (564),
  `scriptToFamilies` (1151), `nsfont_driver` (2208, "non-`Sendable` type
  `UnsafeMutablePointer<font_driver>`") and `O_FWD_Vns_reg_to_script` (2253).  A faithful
  transcription cannot make those `let` or `@MainActor`, so the gate is the default mode, as
  the round's instruction allows.
- `nm -g /tmp/nsfont.o` -> two non-mangled defined symbols, `T _syms_of_nsfont` and
  `T _nsfont_driver` (the getter); every other `T`/`D`/`S` row is Swift runtime
  (`___swift_*`, `_symbolic …`, `_associated conformance …`) or `$s`-mangled, and `nm -u`
  lists 450 undefined entries (the `@_silgen_name` imports plus Swift's own).
- The C side of every mirrored layout, measured with the windowed declarations this file's
  target implies: `/opt/homebrew/opt/llvm/bin/clang -std=gnu23 -DHAVE_WINDOW_SYSTEM=1
  -DHAVE_NS=1 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib`
  (Homebrew clang 23.1.1; the two `nsterm.h` records add `-DNS_IMPL_GNUSTEP=1`) on probes
  that include `<config.h>`, `"lisp.h"`, `"dispextern.h"`, `"nsgui.h"`, `"nsterm.h"`:
  - `struct font_metrics` -> `sizeof` 10, align 2, `lbearing` 0, `rbearing` 2, `width` 4,
    `ascent` 6, `descent` 8
  - `struct font_bitmap` -> `sizeof` 40, align 8, `bits_per_pixel` 0, `rows` 4, `width` 8,
    `pitch` 12, `buffer` 16, `left` 24, `top` 28, `advance` 32
  - `struct font_driver` -> `sizeof` 232, align 8, `type` 0, `case_sensitive` 8, `get_cache`
    16, `list` 24, `match` 32, `list_family` 40, `free_entity` 48, `open_font` 56,
    `close_font` 64, `prepare_face` 72, `done_face` 80, `has_char` 88, `encode_char` 96,
    `text_extents` 104, `draw` 112, `get_bitmap` 120, `free_bitmap` 128, `anchor_point` 136,
    `otf_capability` 144, `otf_drive` 152, `start_for_frame` 160, `end_for_frame` 168,
    `shape` 176, `check` 184, `get_variation_glyphs` 192, `filter_properties` 200,
    `cached_font_ok` 208, `combining_capability` 216, `drop_xrender_surfaces` 224
    (`HAVE_HARFBUZZ` unset, so its two members are absent)
  - `struct Lisp_Fwd` -> `sizeof` 16, align 8, `type` 0 (an 8-bit bitfield), `u` 8
  - `struct nsfont_info` -> `sizeof` 296, align 8, `name` 216, `width` 224, `height` 228,
    `underpos` 232, `underwidth` 236, `size` 240, `nsfont` 248, `bold` 256, `ital` 257,
    `synthItal` 258, `max_bounds` 260, `glyphs` 280, `metrics` 288; `XCharStruct` 20
  - `struct ns_display_info` -> `sizeof` 280, `next` 0, `terminal` 8, `name_list_element` 16
- The same two records, Swift side: `swiftc -o … femacs/platform/macos/nsfont.swift
  <a main that prints MemoryLayout> <a C stub defining the two imported variables>
  -Xlinker -undefined -Xlinker dynamic_lookup`, then the program:
  - `font_metrics` sizeof 10, stride 10, 0/2/4/6/8 **= C**
  - `font_bitmap` size 36, **stride 40**, 0/4/8/12/16/24/28/32 **= C** (C's `sizeof` is the
    stride, i.e. its tail padding)
  - `font_driver` sizeof 232, stride 232, all 30 offsets **= C**
  - `Lisp_Fwd` sizeof 16, type 0, u 8 **= C**
  - `nsfont_info` sizeof 296, name 216, width 224, glyphs 280, metrics 288 **= C**;
    `XCharStruct` 20
  - `ns_display_info` (prefix) sizeof 24, next 0, terminal 8, name_list_element 16 **= C**
  - `font` sizeof 216, props 8, driver 208 **= C**; `Lisp_Char_Table` defalt 8, extras 552,
    size 560 (C's `sizeof` 552 + the one `extras` element the port reads); `gs_font_data`
    sizeof 16 with `family_name` 8 versus C's `sizeof` 32 with `family_name` 24 (Swift's
    raw-value enums are 1 byte - the lane rule's worked example); `ns_glyph_layout` sizeof 52,
    stride 56
- `@convention(c)` and Swift structs: `swiftc -c` on
  `typealias F1 = @convention(c) (UnsafeMutablePointer<S>?) -> Void` (a two-`Int32` `S`) ->
  `error: '(UnsafeMutablePointer<S>?) -> Void' is not representable in Objective-C, so it
  cannot be used with '@convention(c)'`; the same file's `UnsafeMutableRawPointer?` and
  `UnsafeMutablePointer<Int32>?` spellings compile (exit 0).  This is why `font_driver`'s
  slots are raw.
- `[NSFont screenFont]`: a program that asks an `NSFont` for `responds(to: Selector
  ("screenFont"))` and for `.screen` prints `screen == screenFont selector? true`, `same
  object? true` - Swift's `screen` is the same property.
- `python3 tools/extract-abi.py emacs/src/nsfont.m` -> 0 subrs, **36** functions (the six
  line-wrapped signatures are in the list since `tools/cinventory.py`'s 3.160 fix), 3 macros,
  2 types, 1 file-scope object.
- `python3 tools/extract-abi.py --check emacs/src/nsfont.m femacs/platform/macos/nsfont.swift`
  -> `FAIL`, with **36** `missing function` rows plus the macros/types/object: the extractor
  has no Swift reader (status 3.160 item 2), so a `.swift` target cannot be compared and no
  gate can be made green by anything in this file.  This unit's evidence is therefore the
  three-part set of 3.144/3.150 - the `swiftc -parse-as-library -c` exit, the `nm -g` export
  reading, and this ledger - as the round's instruction records.
- The file's conditional map: `grep -nE "^\s*#\s*(if|ifdef|ifndef|else|elif|endif)"
  emacs/src/nsfont.m` -> only `659:#if 0` and `662:#endif`;
  `python3 tools/check-cond-eval.py src/nsfont.m` -> `1 conditions, 0 unknown, 0
  disagreements [VERIFIED]`.
- `grep -n "nsfont\|macfont" emacs/src/Makefile.in emacs/configure.ac` ->
  `src/Makefile.in:480  nsterm.o nsfns.o nsmenu.o nsselect.o nsimage.o nsfont.o macfont.o …`;
  `configure.ac:2762 ns_fontfile=macfont.o`, `configure.ac:2816 ns_fontfile=nsfont.o`.
- `register_font_driver (&nsfont_driver, …)` has three call sites outside this file:
  src/font.h 985 (`extern`), src/nsfns.m 1316 and 3001; `syms_of_nsfont ()` is called from
  src/nsterm.m 11806.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0
  failures`; `python3 tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a
  definition in their own unit`; `python3 tools/check-mirror-dupes.py` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)` (the row is `femacs/src/insdel.rs`, another
  lane's, not this file's); `python3 tools/check-target-names.py` -> `577 .rs files, 593 map
  targets, 0 finding(s)`; `python3 tools/check-symbols.py` -> `86 units compared, 2 mismatched`
  (`lib/mempcpy`, `lib/regex`, pre-existing).
- `emacs/` is untouched by this round (`git -C emacs status --short` is empty), and no
  `/tmp` product of the probes remains.

- **Round 83 lands the file's last two bodies** (`nsfont_draw`, C 1151-1212, and
  `ns_dump_glyphstring`, C 1710-1722) and closes the hole the `## Deferred` section named:
  `swiftc -parse-as-library -c femacs/platform/macos/nsfont.swift` -> **exit 0**, **0 errors**,
  **0 warnings** (the file's baseline is 0 warnings, and the two the first run raised -
  "result of call to 'putc' is unused" - were repaired in place by writing C's own discard
  (`_ = putc (...)`), not by naming them).  Object `/tmp/nsfont83.o`, **409,936 bytes**, md5
  `57b4593546d766070432c084a4379120`.  The unit is **2,521 lines**.
- **The two repairs the first compiles asked for**: `GSShowGlyphs (context, c, len)` needed
  `Int (len)` (the entry's count is the GNUstep `NSInteger`, not C's `int`), and the two `putc`
  discards above.  C's `NSGlyph` turned out to need no spelling decision at all: the SDK
  declares it (`NSFont.h` 147, `typedef unsigned int NSGlyph`), the same width as `char2b`'s
  `unsigned` (src/dispextern.h 1407), so C's element copy is exact.
- **`python3 tools/check-swift-units.py`**: `nsfont.swift: 36 C definition(s), **0** unported,
  0 row(s); **143** C symbol imports` (the round's 29 new entries), and the terminal reading
  **`0 new, 0 stale, 7 Swift unit(s) landed`** - the two manifest rows
  (`docs/baseline/swift-units.tsv`: `ns_dump_glyphstring`, `nsfont_draw`) were removed by name
  in the same commit, the reading the earlier rounds got when a row landed.  The lane's other
  units still read `nsfns.swift: 1 unported` (its recorded permanent dead arm) and
  `nsterm.swift: 6 unported` (the six recorded dead arms), so with this round **every Swift
  unit of the lane is landed or recorded**.
- **Not claimed**: no runtime evidence - no glyph string was drawn and no dumper ran (this
  lane does not link, and the entries are the core's to define); the reading is the compile
  gate, the export/import gates and this ledger.
