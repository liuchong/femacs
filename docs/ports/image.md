# emacs/src/image.c (13805 lines)

> **The largest file in P-86's class.  Six slices are ported: C 1-1095 (the first batch),
> C 1096-2211 (the second), C 2212-3550 (the third), C 3552-4623 (the fourth),
> C 4620-6163 (the fifth) and C 6166-7967 (this one).**
> `configure.ac` 7521 fills `WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` only under
> `HAVE_WINDOW_SYSTEM`, that macro is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`
> (1678), and `docs/baseline/frozen-build-sources.txt` lists no `image.o`, so the frozen
> configuration compiles nothing from this file.  `src/image.c` **is** in
> `docs/baseline/config/cond-eval-verified.txt` (306), so `tools/extract-abi.py` *does* read its
> `#if`s - but the file's own text is mostly **ungated** (the file itself is the gate), so the
> tool's checklist is the whole 13,805-line text and P-73's live-line filter grants no exemption:
> `--check` is *not* vacuous here, and its reading (Evidence 6/11) counts out what the next slices
> still owe.  P-86 1 applies to the port: **transcription, both arms of every `#if`**, each item
> behind the file gate (`have_window_system`) plus C's own condition lower-cased, and this ledger
> records P-86 2's **C line-range table** instead of ticked boxes.  Port: `femacs/src/image.rs`
> (**10,976 lines** after this batch, measured with `grep -c ''`; the sixth slice added 2,555).
>
> **The first slice is C 1-1095**, ending at the `}` of `image_create_bitmap_from_file` (C 1095).
> It holds: the include graph and the platform macro/type sections (C 20-222), the
> forward-declaration block (C 224-246), the six `USE_CAIRO` pixel-container helpers (C 250-368),
> the two `HAVE_NS` pixel shims (C 374-386), the six bitmap accessors (C 403-496), the
> `HAVE_PGTK` Cairo-pattern builder (C 505-591), `image_create_bitmap_from_data` (C 597-735) and
> `image_create_bitmap_from_file` (C 757-1095).
>
> **This batch is the second slice: C 1096-2211**, ending at the `}` of `image_alloc_image_color`
> (C 2211).  It holds `free_bitmap_record` (C 1099-1153), `image_destroy_bitmap` (C 1157-1173),
> `image_destroy_all_bitmaps` (C 1177-1188), the cross-platform forward declarations (C 1190-1231),
> the X11-only `x_create_bitmap_mask` (C 1234-1335), `struct image_type` (C 1338-1372),
> `add_image_type`/`valid_image_p` (C 1380-1427), the four error helpers (C 1429-1458),
> `enum image_value_type`/`struct image_keyword` (C 1461-1502), `parse_image_spec` (C 1505-1644),
> `image_spec_value` (C 1646-1671), the three `DEFUN`s (`image-size` C 1673-1708, `image-mask-p`
> C 1710-1731, `image-metadata` C 1733-1757), `MAX_IMAGE_SIZE` and `make_image`/`free_image`
> (C 1758-1827), `check_image_size` (C 1829-1857), `prepare_image_for_display` (C 1859-1896),
> `image_ascent` (C 1911-1949), `four_corners_best` (C 1957-2004), `make_color_name`
> (C 2006-2010), `image_background`/`image_background_transparent` (C 2017-2100), the
> `CLEAR_IMAGE_*` masks (C 2098-2100), `image_clear_image_1`/`image_clear_image` (C 2102-2174) and
> `image_alloc_image_color` (C 2182-2211).
>
> **This batch is the third slice: C 2212-3550**, ending at C 3550's
> `#endif /* HAVE_IMAGEMAGICK || HAVE_NATIVE_TRANSFORMS */`.  It holds the "Image Cache" banner
> (C 2214-2218), C 2219's `static void cache_image (…)` prototype (dropped, PORTING.md 0.1-12),
> `make_image_cache` (C 2225-2237), `search_image_cache` (C 2241-2279), `filter_image_spec`
> (C 2284-2320), `uncache_image` (C 2324-2343), `free_image_cache` (C 2347-2368),
> `clear_image_cache` (C 2374-2453), `clear_image_caches` (C 2455-2465), `DEFUN
> ("clear-image-cache")` (C 2467-2506), `image_size_in_bytes` (C 2508-2554),
> `image_frame_cache_size` (C 2556-2569), `DEFUN ("image-flush")` (C 2571-2604),
> `postprocess_image` (C 2606-2670), the `HAVE_IMAGEMAGICK || HAVE_NATIVE_TRANSFORMS` gate's
> `scale_image_size` (C 2676-2698), `image_get_dimension` (C 2699-2720) and `compute_image_size`
> (C 2784-2851), `image_compute_scale` (C 2727-2776), the affine-transform banner and
> `typedef double matrix3x3[3][3]` (C 2853-2931), `matrix3x3_mult` (C 2933-2946),
> `compute_image_rotation` (C 2948-2968), the `HAVE_ANDROID` matrix helpers (C 2970-3056) and
> `image_set_transform` (C 3058-3548).
>
> **This batch is the fourth slice: C 3552-4623**, ending at the `}` of `slurp_image` (C 4617) -
> the end of the file's "File Handling" section.  It holds `lookup_image` (C 3552-3688),
> `cache_image` (C 3690-3727), the `HAVE_WEBP || HAVE_GIF` animation-cache block (C 3730-3860:
> the three records, the `anim_cache` object and `anim_create_cache` /
> `anim_prune_animation_cache` / `anim_get_animation_cache`), `mark_image` (C 3862-3871),
> `mark_image_cache` (C 3873-3890), the "X / NS / W32 support code" banner and the
> `HAVE_X_WINDOWS || HAVE_ANDROID` half of its section (C 3894-4114: `x_check_image_size`,
> `x_create_x_image_and_pixmap`, `x_destroy_x_image` and the `!USE_CAIRO && HAVE_XRENDER`
> `x_create_xrender_picture`), `image_check_image_size` (C 4064-4074),
> `image_create_x_image_and_pixmap_1` (C 4086-4241, the cairo / X-Android / Haiku / Windows / NS
> arms), `image_destroy_x_image` (C 4246-4267), `gui_put_x_image` (C 4274-4291),
> `image_create_x_image_and_pixmap` (C 4296-4310), `image_put_x_image` (C 4318-4338),
> `image_sync_to_pixmaps` (C 4344-4359), the `HAVE_NTGUI` pair `image_get_x_image_or_dc` /
> `image_unget_x_image_or_dc` (C 4367-4386) and its `#else`'s `image_get_x_image` /
> `image_unget_x_image` (C 4391-4442), the "File Handling" banner (C 4446-4448),
> `image_find_image_fd` (C 4456-4501), `image_find_image_file` (C 4507-4511), the Android
> `close_android_fd` (C 4515-4519), `slurp_file` (C 4528-4595) and `slurp_image` (C 4601-4617).
> **This batch is the fifth slice: C 4620-6163**, ending at C 6161's `}` of `xpm_free_colors`
> (C 6163's `#endif` closes the `ALLOC_XPM_COLORS` block the slice opened, so it is the slice's
> last C line).  It holds the **XBM section** (C 4620-5451: C 4624's dropped prototype, the
> `xbm_keyword_index`/`xbm_token` enums with the `xbm_format` table, `xbm_image_p`, `xbm_scan`,
> the `HAVE_NTGUI` pair `w32_create_pixmap_from_bitmap_data`/`convert_mono_to_color_image`,
> `Create_Pixmap_From_Bitmap_Data`, `xbm_read_bitmap_data`, `xbm_load_image`, `xbm_file_p` and
> `xbm_load`), the **whole Canvas section** (C 5453-5906: `canvas_keyword_index`/`canvas_format`,
> the `canvas_map`/`canvas_list` objects, `canvas_parse`, `canvas_image_p`, `canvas_free_unused`,
> `canvas_apply_data`, `canvas_get`, `canvas_load`, `canvas_prepare_for_display`, `canvas_data`
> and `DEFUN ("canvas-refresh")`) and the **XPM section's head** (C 5909-6161: the banner,
> C 5913-5919's dropped prototype pair, C 5921-5942's `<X11/xpm.h>` - this slice's one P-88
> declaration section, the only place in this file that takes an external header's *layout* -
> C 5944-5984's `xpm_keyword_index`/`xpm_format`, C 5986-5996's `ALLOC_XPM_COLORS` derivation and
> C 5998-6163's colour cache).
> **The next live definition is `xpm_valid_color_symbols_p` (C 6213)**, after C 6166's
> `#ifdef WINDOWSNT` DLL block and `xpm_load`'s two arms (C 6307, C 7087); C 6000-6001's
> prototype and C 5913-5919's pair were dropped in place (PORTING.md 0.1-12), and the kept
> declarations of the three functions this slice *defines* - `xbm_read_bitmap_data`
> (C 751-752, the second slice's block), `canvas_prepare_for_display` and `canvas_free_unused`
> (C 126-127, the first/second slices' blocks) - are gone with it.
>
> **The second slice is `text-only` for five of the six platform arms** (P-86 3-2).  The machine has no
> Windows toolchain and no Haiku/GTK SDK (`which x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc` is
> empty, and `/opt/homebrew/include` has no `gtk`/`pango` tree), so the Windows, Haiku and PGTK
> arms get no arm flip - what this ledger has for them is the frozen reading (Evidence 1), the
> machine-checked sweep (Evidence 8) and the per-item C citation.  X11 and cairo *do* have headers
> here (`/opt/homebrew/include`: xorgproto 2025.1, libX11, cairo 1.18.4), so the X+cairo
> configuration is flipped in the sweep; the **NS** configuration is the one this machine's
> platform actually is, and it is the one carried through the **arm-reach pre-flight** to `exit 0`
> (Evidence 9).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`WINDOW_SYSTEM_OBJ`, configure.ac 7521) | `feature = "have_window_system"` (on every item and on C 22-65's include graph) | a window-system configuration |
| C 30 `#if defined HAVE_PNG` ... C 32 | `all (have_window_system, have_png)` | a PNG configuration |
| C 55 `#ifdef HAVE_SYS_STAT_H`, C 59 `#ifdef HAVE_SYS_TYPES_H` | **no feature**: both macros are `1` in the frozen table (1632/1637), so the includes are the live arm (the empty `#else` side carries no text) | every configuration |
| C 63 `#ifdef HAVE_WINDOW_SYSTEM` ... C 65 (`TERM_HEADER`) | one `include!` per platform, under the platform's own feature | a window-system configuration |
| C 244-246 `#if defined (HAVE_WEBP) \|\| defined (HAVE_GIF)` (the `anim_prune_animation_cache` prototype, now with its call site C 2490) | `any (have_webp, have_gif)` | a WebP or GIF configuration |
| C 2488-2491 `#if defined (HAVE_WEBP) \|\| defined (HAVE_GIF)` (inside the `clear-image-cache` subr) | `any (have_webp, have_gif)` | a WebP or GIF configuration |
| C 2513-2552 `image_size_in_bytes`'s `#if`/`#elif` chain (`USE_CAIRO`, `X_WINDOWS \|\| ANDROID`, `HAVE_NS`, `HAVE_NTGUI`, `HAVE_HAIKU`) | one arm per condition, each excluding the ones before it (the `#elif` chain) | a window-system configuration that defines one of them |
| C 2671 and C 2777 `#if defined (HAVE_IMAGEMAGICK) \|\| defined (HAVE_NATIVE_TRANSFORMS)` | `any (have_imagemagick, have_native_transforms)`; **`have_native_transforms` is not in `femacs/xtask/Cargo.toml` yet** (Owed 10) | an ImageMagick or native-transforms configuration |
| C 3082-3087 `# if (defined HAVE_IMAGEMAGICK && !defined DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE)` | `have_imagemagick`, `not (have_pgtk)` (the second term is C 104-111's constant, so `!defined` is `not (all (have_pgtk, have_imagemagick))`) | an ImageMagick configuration that is not PGTK |
| C 3089-3092 `# if !defined USE_CAIRO && defined HAVE_XRENDER` | `not (use_cairo)`, `have_xrender` | an XRender configuration without cairo |
| C 3095-3103 `#ifdef HAVE_RSVG` | `have_rsvg` (**not in the manifest yet**, Owed 7) | an SVG configuration |
| C 3105-3115 `# if defined USE_CAIRO \|\| defined HAVE_XRENDER \|\| defined HAVE_NS \|\| defined HAVE_HAIKU \|\| defined HAVE_ANDROID \|\| defined HAVE_NTGUI` | that `any (…)` | any of those six |
| C 3126-3141 the scale matrix's `# if`/`# elif`/`# else` chain (cairo/xrender/android, ntgui/ns/haiku, else) | three arms, each excluding the earlier conditions | as above |
| C 3163-3330 `#ifndef HAVE_ANDROID`'s arm and C 3165's five-way `# if` | `not (have_android)`; `any (use_cairo, have_xrender, have_ntgui, have_ns, have_haiku)` | a non-Android window-system configuration |
| C 3227-3287 the rotation block's `#  if`/`#  else` (cairo/xrender/android vs the rest) | both arms | as above |
| C 3331-3492 `#else` (the `HAVE_ANDROID` inverse transform) | `have_android` | an Android configuration |
| C 3494-3548 the store-the-transform `# if`/`# elif` chain (`HAVE_NS`, `USE_CAIRO`, `HAVE_XRENDER`, `HAVE_NTGUI`, `HAVE_HAIKU`, `HAVE_ANDROID`) | six arms, each excluding the ones before it | a window-system configuration that defines one of them |
| C 67 `#ifdef HAVE_X_WINDOWS` ... C 77 (C 69 `#ifndef USE_CAIRO` nested) | `have_x_windows`; the nested pair `all (have_x_windows, not (use_cairo))` | an X11 configuration |
| C 79 `#if defined (USE_CAIRO) || defined (HAVE_NS)` ... C 90 (C 81 `#ifndef HAVE_NS`) | `any (use_cairo, have_ns)`; C 82's `ARGB_TO_ULONG` is `all (..., not (have_ns))` | a cairo or NS configuration |
| C 92 `#ifdef USE_CAIRO` ... C 102 | `use_cairo` | a cairo configuration |
| C 104 `#if defined HAVE_PGTK && defined HAVE_IMAGEMAGICK` ... C 111 | `all (have_pgtk, have_imagemagick)` | a PGTK + ImageMagick configuration |
| C 129 `#ifdef HAVE_NTGUI` ... C 151 (C 147 `#else`) | `have_ntgui`; **C 149's `#else` arm is `not (have_ntgui)`** (`XBM_BIT_SHUFFLE`) | a Windows GUI configuration |
| C 153 `#ifdef HAVE_NS` ... C 163 | `have_ns` | the NS configuration |
| C 165 `#ifdef HAVE_PGTK` ... C 167 | `have_pgtk` | a PGTK configuration |
| C 169 `#if (defined HAVE_X_WINDOWS && ! (defined HAVE_NTGUI || defined USE_CAIRO || defined HAVE_NS))` ... C 173 | `all (have_x_windows, not (any (have_ntgui, use_cairo, have_ns)))`, spelled where C writes `#ifdef COLOR_TABLE_SUPPORT` | a plain-X11 configuration |
| C 175 `#ifdef HAVE_HAIKU` ... C 194 | `have_haiku` | the Haiku configuration |
| C 196 `#ifdef HAVE_ANDROID` ... C 222 | `have_android` | an Android configuration |
| C 238 `#ifdef HAVE_NTGUI` ... C 242 | `have_ntgui` | a Windows GUI configuration |
| C 244 `#if defined (HAVE_WEBP) || defined (HAVE_GIF)` ... C 246 | `any (have_webp, have_gif)` | a WebP or GIF configuration |
| C 248 `#ifdef USE_CAIRO` ... C 370 | `use_cairo` | a cairo configuration |
| C 372 `#ifdef HAVE_NS` ... C 387 | `have_ns` | the NS configuration |
| C 402 `#ifdef HAVE_X_WINDOWS` ... C 449 (C 415 `#ifdef USE_CAIRO` nested) | `have_x_windows`; `use_cairo` for `x_bitmap_stipple` | an X11 configuration |
| C 451 `#if defined (HAVE_X_WINDOWS) || defined (HAVE_NTGUI) || defined (HAVE_ANDROID)` ... C 459 | `any (have_x_windows, have_ntgui, have_android)` | any of the three |
| C 461 `#ifdef HAVE_X_WINDOWS` ... C 467 | `have_x_windows` | an X11 configuration |
| C 498 `#ifdef HAVE_PGTK` ... C 593 (C 512/559 `#ifdef WORDS_BIGENDIAN`, C 570 `#else`) | `have_pgtk` / `words_bigendian` | a PGTK configuration |
| C 604/613/625/654/660/666/691/696/701/716/722/727 (`HAVE_X_WINDOWS`, `HAVE_ANDROID`+`ANDROID_STUBIFY`, `HAVE_NTGUI`, `HAVE_NS`, `HAVE_PGTK`, `HAVE_HAIKU`, `USE_CAIRO`) | the same features, nested in the file gate | the arm's own configuration |
| C 737 `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` ... C 745 (C 743 `#else`) | `all (have_android, not_android_stubify)` / its negation | Android, stubs on or off |
| C 747 `#if defined HAVE_HAIKU || defined HAVE_NS || defined HAVE_PGTK || defined HAVE_ANDROID || defined HAVE_NTGUI` ... C 753 | `any (have_haiku, have_ns, have_pgtk, have_android, have_ntgui)` | any of the five |
| C 762-1092 (`HAVE_NTGUI`, `HAVE_NS`, `HAVE_PGTK`, `HAVE_X_WINDOWS`, `HAVE_HAIKU`, `HAVE_ANDROID`+`ANDROID_STUBIFY`) | the same features, nested in the file gate | the arm's own configuration |

| C 1102 `#ifdef HAVE_X_WINDOWS` ... C 1118 (C 1114 `#ifdef USE_CAIRO` nested) | `have_x_windows` / `all (have_x_windows, use_cairo)` | an X11 configuration |
| C 1120 `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` ... C 1125 | `all (have_android, not_android_stubify)` | Android, stubs off |
| C 1127 `#ifdef HAVE_NTGUI` ... C 1130 | `have_ntgui` | a Windows GUI configuration |
| C 1132 `#ifdef HAVE_NS` ... C 1134 | `have_ns` | the NS configuration |
| C 1136 `#ifdef HAVE_PGTK` ... C 1139 | `have_pgtk` | a PGTK configuration |
| C 1141 `#ifdef HAVE_HAIKU` ... C 1146 | `have_haiku` | the Haiku configuration |
| C 1190 `#ifndef HAVE_XRENDER` ... C 1193 | `not (have_xrender)`; the `#ifdef`-less arm is Xrender.h's `Picture` | the two Xrender arms |
| C 1200 `#ifdef HAVE_NTGUI` ... C 1213 (C 1205 `#else`) | `have_ntgui` / `not (have_ntgui)` | the X-image accessor pair, **both arms** |
| C 1215 `#if defined HAVE_X_WINDOWS \|\| defined HAVE_ANDROID` ... C 1329 (C 1217 `#ifndef USE_CAIRO`, C 1229 `#if defined HAVE_X_WINDOWS`, C 1327/C 1329 the two `#endif`s) | `all (any (have_x_windows, have_android), not (use_cairo))` for `image_sync_to_pixmaps`; `any (have_x_windows, have_android)` for the two prototypes; `have_x_windows` for `x_create_bitmap_mask` | an X11 or Android configuration |
| C 1345 `#ifdef WINDOWSNT` ... C 1347 | `windowsnt` | the `struct image_type.init` member |
| C 1352 `#if defined HAVE_RSVG \|\| … \|\| defined HAVE_ANDROID` ... C 1361 (C 1355 `#ifdef WINDOWSNT` / C 1357 `#else`) | the eleven features; `/ windowsnt` | the `IMAGE_TYPE_INIT` macro's two arms |
| C 1808 `#if !defined USE_CAIRO && defined HAVE_XRENDER` ... C 1817 | `all (not (use_cairo), have_xrender)` | an Xrender, non-cairo configuration |
| C 1818 `#ifdef HAVE_X_WINDOWS` ... C 1820 | `have_x_windows` | an X11 configuration |
| C 1877 `#ifdef USE_CAIRO` ... C 1889 / C 1890 `#elif defined HAVE_X_WINDOWS \|\| defined HAVE_ANDROID` ... C 1896 | `use_cairo` / `all (not (use_cairo), any (have_x_windows, have_android))` | `prepare_image_for_display`'s two refresh arms |
| C 1924 `#ifdef HAVE_NTGUI` ... C 1934 (C 1929 `#else`) | `have_ntgui` / its negation | `image_ascent`'s two centring expressions |
| C 1965 `#ifdef USE_CAIRO` ... C 1970 | `use_cairo` | a cairo configuration |
| C 2021 `#ifdef HAVE_NTGUI` ... C 2023; C 2029 `#ifdef USE_CAIRO` ... C 2033 | `have_ntgui`; `use_cairo` | `image_background`'s two arms |
| C 2064 `#ifdef HAVE_NTGUI` ... C 2066 | `have_ntgui` | `image_background_transparent` |
| C 2114 / C 2132 `#if (defined HAVE_X_WINDOWS \|\| defined HAVE_ANDROID) && !defined USE_CAIRO` ... C 2121 / C 2139 | `all (any (have_x_windows, have_android), not (use_cairo))` | `image_clear_image_1`'s X-image arms |
| C 2145 `#if defined HAVE_X_WINDOWS && !defined USE_CAIRO` ... C 2147 | `all (have_x_windows, not (use_cairo))` | the colour-table arm |
| C 2153 `#ifdef USE_CAIRO` ... C 2159 | `use_cairo` | a cairo configuration |

The frozen values behind those arms (`docs/baseline/config/darwin-arm64.macros.tsv`):
`HAVE_WINDOW_SYSTEM` `undef` (1678), `HAVE_X_WINDOWS` `undef` (1720), `USE_CAIRO` `undef` (3858),
`HAVE_NS` `undef` (1481), `HAVE_NTGUI` `undef` (1482), `HAVE_PGTK` `undef` (1489), `HAVE_HAIKU`
`undef` (1388), `HAVE_ANDROID` `undef` (1240), `HAVE_PNG` `undef` (1491), `HAVE_WEBP` `undef`
(1677), `HAVE_GIF` `undef` (1377), `HAVE_IMAGEMAGICK` `undef` (1393), `WORDS_BIGENDIAN` `undef`.
`src/image.c` is one of the 424 files in `cond-eval-verified.txt`, which is what makes the table
above checkable rather than asserted.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 22-26 | `errno.h`, `fcntl.h`, `math.h`, `unistd.h`, `stdlib.h` | file gate |
| 28-29 | the comment "Include this before including `<setjmp.h>` to work around bugs with older libpng" | file gate (C's own comment, kept) |
| 30-32 | `#if defined HAVE_PNG` / `# include <png.h>` | file gate / `have_png`; **no declaration of this slice's is taken from `<png.h>`**, so the P-88 section is owed with the PNG slice (C 7xxx) |
| 34 | `#include <setjmp.h>` | file gate; its names are `lisp.h`'s mirror's (`include/lisp.rs` 1060/2147) - this slice reads none |
| 36-39 | `math.h`, `stdint.h`, `c-ctype.h`, `flexmember.h` | file gate |
| 41-53 | `lisp.h`, `frame.h`, `process.h`, `window.h`, `buffer.h`, `dispextern.h`, `blockinput.h`, `sysstdio.h`, `systime.h`, `epaths.h`, `coding.h`, `termhooks.h`, `font.h` | file gate; `<epaths.h>` is generated and this slice reads no `PATH_*` value of it |
| 55-57 | `#ifdef HAVE_SYS_STAT_H` / `#include <sys/stat.h>` | the macro is `1` (macros.tsv 1632), so this is the live arm (`no_sys_stat_h` has no text) |
| 59-61 | `#ifdef HAVE_SYS_TYPES_H` / `#include <sys/types.h>` | the macro is `1` (macros.tsv 1637) - the live arm |
| 63-65 | `#ifdef HAVE_WINDOW_SYSTEM` / `#include TERM_HEADER` | one gated `include!` per platform (`have_x_windows`/`have_ns`/`have_pgtk`/`have_ntgui`/`have_haiku`/`have_android`), the `frame.rs` 82-98 shape |
| 67-77 | `#ifdef HAVE_X_WINDOWS`: C 68 `Bitmap_Record`, C 69-76's `#ifndef USE_CAIRO` `GET_PIXEL`/`PUT_PIXEL`/`NO_PIXMAP`/`PIX_MASK_*` | `have_x_windows` (the inner pair `all (have_x_windows, not (use_cairo))`) |
| 79-90 | `#if defined (USE_CAIRO) || defined (HAVE_NS)`: C 80-89's nine colour macros (C 81-83 the nested `#ifndef HAVE_NS`) | `any (use_cairo, have_ns)` / `all (…, not (have_ns))` |
| 92-102 | `#ifdef USE_CAIRO`: C 93-98 the five macros, C 100-101 the `image_alloc_image_color` prototype | `use_cairo`; the prototype's definition is C 2318's (outside this slice) so it is kept as an `extern` declaration |
| 104-111 | `#if defined HAVE_PGTK && defined HAVE_IMAGEMAGICK` / `DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE` | `all (have_pgtk, have_imagemagick)` |
| 113-124 | `struct canvas` | file gate |
| 126-127 | `canvas_prepare_for_display`, `canvas_free_unused` (both `static`) | dropped: prototype + definition in one unit (PORTING.md 0.1-12); the definitions are C 2438/2472, outside this slice |
| 129-151 | `#ifdef HAVE_NTGUI` … `#else` … `#endif`: C 131-135 the nested `#ifdef WINDOWSNT` include pair, C 137 `Bitmap_Record`, C 138-145 the six macros, C 147-151's `#else` arm | `have_ntgui`; **both arms carried** (C 145 `XBM_BIT_SHUFFLE(b) (~(b))` / C 149 `XBM_BIT_SHUFFLE(b) (b)`) |
| 153-163 | `#ifdef HAVE_NS`: C 154 `Bitmap_Record`, C 156-161 the five macros | `have_ns` |
| 165-167 | `#ifdef HAVE_PGTK`: C 166 `Bitmap_Record` | `have_pgtk` |
| 169-173 | the derived `#if (defined HAVE_X_WINDOWS && ! (defined HAVE_NTGUI || defined USE_CAIRO || defined HAVE_NS)) # define COLOR_TABLE_SUPPORT 1` | `all (have_x_windows, not (any (have_ntgui, use_cairo, have_ns)))`, spelled at each `#ifdef COLOR_TABLE_SUPPORT` |
| 175-194 | `#ifdef HAVE_HAIKU`: C 176 the include, C 177 `Bitmap_Record`, C 179-192 the macros | `have_haiku` |
| 196-222 | `#ifdef HAVE_ANDROID`: C 197 the include, C 199-202 the three typedefs, C 204-217 the macros, C 219-221 `#define n_planes n_image_planes` | `have_android` |
| 224-236 | C 224-229 `image_disable_image`/`image_edge_detection`/`image_compute_scale`, C 231-232 `init_color_table`/`lookup_rgb_color`, C 233-236's `#ifdef COLOR_TABLE_SUPPORT` pair (`free_color_table`, `colors_in_color_table`) | dropped: `static` prototype + definition in one unit (0.1-12); all six definitions are C 2275-2413's, outside this slice |
| 238-242 | `#ifdef HAVE_NTGUI`'s `w32_create_pixmap_from_bitmap_data` (C 239) and `XPutPixel` (C 240) prototypes | `have_ntgui`; **not declared**: the arm is text-only, and the declarations would need `<windows.h>`'s `HBITMAP`/`COLORREF`/`XImage`, which this tree does not mirror (`include/w32gui.rs` 202 gates its `XImage` on `windows`, and `src/w32fns.rs` 250 declares `COLORREF` privately) - see the Deviations |
| 244-246 | `#if defined (HAVE_WEBP) || defined (HAVE_GIF)`'s `anim_prune_animation_cache` prototype | `any (have_webp, have_gif)`; dropped (its definition is C 9184's, and nothing here calls it) |
| 248-370 | `#ifdef USE_CAIRO`: C 250-267 `image_create_pix_container`, C 269-277 `image_pix_container_put_pixel`, C 279-286 `image_pix_context_get_pixel`, C 288-306 `image_pix_container_create_from_bitmap_data`, C 308-344 `cr_create_surface_from_pix_containers`, C 346-368 `cr_put_image_to_cr_data` | `use_cairo`; the definitions are inside this slice |
| 372-387 | `#ifdef HAVE_NS`: C 374-378 `XGetPixel`, C 382-386 `XPutPixel` | `have_ns`; both are this file's own `static` functions and are inside this slice |
| 389-398 | the "Code to deal with bitmaps" comment block | kept as C's comment (file gate) |
| 400 | `/* Functions to access the contents of a bitmap, given an id.  */` | kept as C's comment |
| 402-449 | `#ifdef HAVE_X_WINDOWS`: C 403-407 `x_bitmap_height`, C 409-413 `x_bitmap_width`, C 415-448 `#ifdef USE_CAIRO`'s `x_bitmap_stipple` | `have_x_windows`, the third under `use_cairo` too; `x_bitmap_stipple` is non-`static` in C, so it is `#[unsafe(no_mangle)]` |
| 451-459 | `#if defined (HAVE_X_WINDOWS) || defined (HAVE_NTGUI) || defined (HAVE_ANDROID)`'s `image_bitmap_pixmap` | `any (have_x_windows, have_ntgui, have_android)`; non-`static` |
| 461-467 | `#ifdef HAVE_X_WINDOWS`'s `x_bitmap_mask` | `have_x_windows`; non-`static` |
| 469-489 | `image_allocate_bitmap_record` | file gate (`static`) |
| 490-496 | `image_reference_bitmap` | file gate; non-`static` |
| 498-593 | `#ifdef HAVE_PGTK`'s `image_bitmap_to_cr_pattern` (C 512-547 the `#ifdef WORDS_BIGENDIAN` table, C 559-580 the two copy loops) | `have_pgtk` / `words_bigendian` for the pair |
| 595-735 | `image_create_bitmap_from_data` (C 604-611 `HAVE_X_WINDOWS`, C 613-623 `HAVE_ANDROID`/`ANDROID_STUBIFY`, C 625-652 `HAVE_NTGUI`, C 654-658 `HAVE_NS`, C 660-664 `HAVE_PGTK`, C 666-687 `HAVE_HAIKU`, C 691-709 and C 716-732 the per-platform record writes) | file gate (non-`static`) + one arm per platform; **both arms of C 613-623 carried** (`emacs_abort ()`'s stubify arm) |
| 737-745 | C 737-742's `image_fd` typedef pair | `all (have_android, not_android_stubify)` / its negation |
| 747-753 | C 749-752's three `static` prototypes (`slurp_file`, `image_find_image_fd`, `xbm_read_bitmap_data`) | `any (have_haiku, have_ns, have_pgtk, have_android, have_ntgui)`; **kept as declarations** (their definitions are C 4457/4529/5080's, outside this slice) |
| 755-1095 | `image_create_bitmap_from_file` (C 762-807 `HAVE_NTGUI`, C 809-852 `HAVE_NS`, C 854-887 `HAVE_PGTK`, C 889-937 `HAVE_X_WINDOWS`, C 939-1020 `HAVE_HAIKU`, C 1022-1094 `HAVE_ANDROID`/`ANDROID_STUBIFY`) | file gate (non-`static`) + one arm per platform; **the slice ends at C 1095, the function's `}`** |
| 1096-1098 | the `/* Free bitmap B.  */` comment | file gate (C's own comment, kept) |
| 1099-1153 | `free_bitmap_record` (C 1102-1118 `HAVE_X_WINDOWS`, C 1114-1117 the nested `USE_CAIRO`, C 1120-1125 `HAVE_ANDROID`+`ANDROID_STUBIFY`, C 1127-1130 `HAVE_NTGUI`, C 1132-1134 `HAVE_NS`, C 1136-1139 `HAVE_PGTK`, C 1141-1146 `HAVE_HAIKU`, C 1148-1152 the shared tail) | file gate (`static`) + one arm per platform; **every arm carried** |
| 1155-1173 | `image_destroy_bitmap` | file gate; non-`static` |
| 1175-1188 | `image_destroy_all_bitmaps` | file gate; non-`static` |
| 1190-1193 | `#ifndef HAVE_XRENDER`'s `typedef void Picture;` | `not (have_xrender)`; the `HAVE_XRENDER` arm is Xrender.h's `Picture` (`XID`, an `unsigned long`) - `include/xlib.rs` carries neither |
| 1195-1198 | C 1195-1197 `image_create_x_image_and_pixmap_1` and C 1198 `image_destroy_x_image`, both `static` | file gate; **kept as declarations** (their definitions are outside this slice; the C 749-752 shape) |
| 1200-1213 | C 1201-1204 the `HAVE_NTGUI` accessor pair, C 1206-1208 the other pair's declarations, C 1209-1210 the two `image_*_or_dc` macros | `have_ntgui` / `not (have_ntgui)`; **both arms carried** - the non-Windows arm's wrappers are `macro_rules!`, the call sites keep one spelling |
| 1215-1227 | C 1218 `image_sync_to_pixmaps` (inside C 1217's `#ifndef USE_CAIRO`), C 1221-1223 C's comment, C 1225-1227 the two X-image prototypes | `all (any (have_x_windows, have_android), not (use_cairo))` / `any (have_x_windows, have_android)`; kept as declarations |
| 1229-1329 | `#if defined HAVE_X_WINDOWS`'s `x_create_bitmap_mask` | `have_x_windows`; non-`static` (so `#[unsafe(no_mangle)]`); its Xlib names stay C's tokens (the ledger's Deviations) |
| 1331-1372 | the "Image types" banner (C 1331-1333), the record's comment (C 1335-1336) and `struct image_type` (C 1345-1347 the `WINDOWSNT` member, C 1352-1361 the `IMAGE_TYPE_INIT` pair) | file gate; the member and both macro arms are `windowsnt`-gated |
| 1374-1378 | C 1374 `lookup_image_type`, C 1375 `image_laplace`, C 1376 `image_emboss`, C 1377-1378 `image_build_heuristic_mask` (`static` prototypes) | file gate; **kept as declarations** (outside this slice) |
| 1380-1392 | `add_image_type` | file gate (`static`) |
| 1394-1427 | `valid_image_p` | file gate; non-`static` |
| 1429-1459 | the "Image specifications" banner (C 1457-1459) and C 1429-1436 `image_error` (variadic), C 1438-1439 `image_invalid_data_error`, C 1444-1445 `image_not_found_error`, C 1450-1451 `image_size_error` | file gate; all four `static` |
| 1461-1477 | `enum image_value_type` | file gate; the port's `c_int` type + one constant per member |
| 1479-1503 | `struct image_keyword` and C 1500-1503's comment tail | file gate |
| 1505-1644 | `parse_image_spec` (C 1539's `goto maybe_done`, C 1541-1635 the switch) | file gate (`static`); the `goto` is the tree's flag shape, the `switch` a `match` |
| 1646-1671 | `image_spec_value` | file gate; non-`static` |
| 1673-1708 | `DEFUN ("image-size", Fimage_size, Simage_size, …)` | file gate (`DEFUN!`) |
| 1710-1731 | `DEFUN ("image-mask-p", Fimage_mask_p, Simage_mask_p, …)` | file gate (`DEFUN!`) |
| 1733-1757 | `DEFUN ("image-metadata", Fimage_metadata, Simage_metadata, …)` | file gate (`DEFUN!`) |
| 1758-1781 | C 1758 `MAX_IMAGE_SIZE` and C 1762-1781 `make_image` | file gate (`static`) |
| 1783-1827 | `free_image` (C 1808-1817 the `!USE_CAIRO && HAVE_XRENDER` arm, C 1818-1820 the `HAVE_X_WINDOWS` guard on C 1822's statement) | file gate (`static`) + those two conditions |
| 1829-1857 | `check_image_size` | file gate (`static`) |
| 1859-1905 | `prepare_image_for_display` (C 1877-1889 `USE_CAIRO`, C 1890-1896 the `#elif`, C 1899-1904 the canvas-refresh tail) | file gate; non-`static` + the two refresh arms |
| 1908-1949 | `image_ascent` (with C 1908-1910's comment) (C 1924-1934 the `HAVE_NTGUI` / `#else` pair) | file gate; non-`static`; **both arms carried** |
| 1951-2004 | C 1951-1955's "Image background colors" comment and `four_corners_best` (C 1965-1970 the `USE_CAIRO` arm) | file gate (`static`) |
| 2006-2010 | `make_color_name` | file gate (`static`) |
| 2012-2053 | `image_background` (with C 2012-2016's comment) (C 2021-2023 `HAVE_NTGUI`, C 2029-2033 `USE_CAIRO`) | file gate; non-`static` |
| 2055-2085 | `image_background_transparent` (C 2064-2066 `HAVE_NTGUI`) | file gate; non-`static` |
| 2087-2100 | the "Helper functions for X image types" banner, the "Clear X resources" comment and C 2098-2100's three `CLEAR_IMAGE_*` masks | file gate |
| 2102-2160 | `image_clear_image_1` (C 2114-2121 and C 2132-2139 the `(X \|\| ANDROID) && !USE_CAIRO` arms, C 2145-2147 the colour-table arm, C 2153-2159 the `USE_CAIRO` arm) | file gate (`static`) + those three conditions |
| 2162-2174 | `image_clear_image` | file gate (`static`) |
| 2177-2211 | `image_alloc_image_color` | file gate (`static`); **this slice's last item** |
| 2212-2218 | the "Image Cache" banner | file gate (C's own banner text, kept) |
| 2219 | `static void cache_image (struct frame *, struct image *);` | dropped: prototype + definition in one unit (PORTING.md 0.1-12); the definition is C 3694's |
| 2221-2237 | `make_image_cache` (C 2221-2224's comment) | file gate; non-`static` |
| 2238-2279 | `search_image_cache` (C 2239-2240's comment) | file gate (`static`) |
| 2280-2320 | `filter_image_spec` (C 2281-2283's comment) | file gate (`static`) |
| 2321-2343 | `uncache_image` (C 2322-2323's comment) | file gate (`static`) |
| 2344-2368 | `free_image_cache` (C 2345-2346's comment) | file gate; non-`static` |
| 2369-2453 | `clear_image_cache` (C 2370-2373's comment) | file gate; non-`static` |
| 2454-2465 | `clear_image_caches` | file gate; non-`static` |
| 2467-2506 | `DEFUN ("clear-image-cache", Fclear_image_cache, Sclear_image_cache, 0, 2, 0, …)` (C 2488-2491 the WebP/GIF arm) | file gate (`DEFUN!`) + `any (have_webp, have_gif)` |
| 2507-2554 | `image_size_in_bytes` (C 2523-2531's X/Android arm, C 2528-2531 the two `XImage` field reads) | file gate (`static`) + the five platform conditions |
| 2555-2569 | `image_frame_cache_size` | file gate (`static`) |
| 2571-2604 | `DEFUN ("image-flush", Fimage_flush, Simage_flush, 1, 2, 0, …)` | file gate (`DEFUN!`) |
| 2605-2670 | `postprocess_image` (C 2604-2605's comment) | file gate (`static`) |
| 2671-2721 | C 2671 `#if defined (HAVE_IMAGEMAGICK) \|\| defined (HAVE_NATIVE_TRANSFORMS)`, C 2672-2675's comment, `scale_image_size` (C 2676-2698), C 2700-2720's comment and `image_get_dimension` (C 2699-2720), C 2721 `#endif` | `any (have_imagemagick, have_native_transforms)`; both `static` |
| 2723-2776 | `image_compute_scale` (C 2723-2726's comment; C 2733's `invalid_value:` label) | file gate (`static`); the `goto` is the tree's loop shape |
| 2777-3550 | C 2777's second `#if defined HAVE_IMAGEMAGICK \|\| defined HAVE_NATIVE_TRANSFORMS` … C 3550 `#endif`, holding C 2779-2783's comment and `compute_image_size` (C 2784-2851), C 2853-2931's affine-transform banner and `typedef double matrix3x3[3][3]`, `matrix3x3_mult` (C 2933-2946), `compute_image_rotation` (C 2948-2968), C 2970-3056's `#ifdef HAVE_ANDROID` matrix helpers and `image_set_transform` (C 3058-3548) | `any (have_imagemagick, have_native_transforms)` plus, inside: `have_android` (the four helpers) and `image_set_transform`'s own chains (`have_rsvg`, `use_cairo`/`have_xrender`/`have_ns`/`have_haiku`/`have_android`/`have_ntgui`, the scale-matrix arms and the six store-the-transform arms) - every `#if`'s both arms carried |
| 3552-3688 | `lookup_image` (C 3552-3554 C's comment, C 3555-3556 the signature, C 3558-3688 the body) | file gate; **non-`static`, so `#[unsafe(no_mangle)]`** - and the declaration the third slice kept (dispextern.h 3757) is dropped here, since this *is* the definition; the `have_native_transforms` arm of C 3645-3647 |
| 3691-3727 | `cache_image` (C 3691's comment) | file gate (`static`); C 2219's prototype was already dropped by the third slice |
| 3730-3860 | `#if defined (HAVE_WEBP) \|\| defined (HAVE_GIF)`: C 3732-3738 `struct gif_anim_handle`, C 3740-3751 `struct webp_anim_handle`, C 3756-3788 `struct anim_cache` (C 3764-3772's nested `union anim_handle`), C 3790 the `anim_cache` object, C 3795-3805 `anim_create_cache`, C 3811-3833 `anim_prune_animation_cache`, C 3835-3858 `anim_get_animation_cache`, C 3860's `#endif` | `any (have_webp, have_gif)` plus the nested `have_gif`/`have_webp`; C 244-246's prototype of `anim_prune_animation_cache` is dropped here (its definition is C 3812) |
| 3862-3871 | `mark_image` (C 3862's comment) | file gate (`static`) |
| 3873-3890 | `mark_image_cache` (C 3873-3874's comment; C 3886-3889's `HAVE_WEBP \|\| HAVE_GIF` loop) | file gate; **non-`static`** |
| 3894-3900 | the "X / NS / W32 support code" banner (C 3894-3898) and C 3900's `#if defined HAVE_X_WINDOWS \|\| defined HAVE_ANDROID` (closed by C 4114, whose `#endif` C mislabels `/* HAVE_X_WINDOWS */`) | file gate + that pair of conditions |
| 3902-3933 | `x_check_image_size` (C 3905-3907's comment, C 3908-3912's local `enum`, C 3917-3921's `HAVE_ANDROID` pair) | the gate above; both arms of each |
| 3935-3993 | `x_create_x_image_and_pixmap` (C 3939-3942's two locals, C 3948-3955's pair, C 3975-3983's pair incl. `ANDROID_STUBIFY`, C 4004-4008's pair) | the gate above; both arms |
| 3995-4009 | `x_destroy_x_image` (C 4004-4008's pair) | the gate above |
| 4011-4057 | C 4011's `# if !defined USE_CAIRO && defined HAVE_XRENDER`, its comment, `x_create_xrender_picture` (C 4013-4055), C 4056-4057's two `#endif`s | the gate above + `not (use_cairo)`, `have_xrender`; the Xrender surface (`XRenderFindStandardFormat`, `XRenderPictureAttributes`, `PictStandard*`, `CPRepeat`, `RepeatPad`, `XRenderCreatePicture`) is the SDK-name class this machine's headers have but no mirror carries |
| 4059-4074 | the doc comment (C 4059-4062) and `image_check_image_size` (C 4067-4073) | file gate; both arms |
| 4076-4241 | the doc comment (C 4076-4084) and `image_create_x_image_and_pixmap_1` (C 4091-4104 `USE_CAIRO`, C 4105-4114 `#elif X \|\| ANDROID` incl. C 4108-4111's nested `HAVE_XRENDER`, C 4116-4138 `HAVE_HAIKU`, C 4140-4228 `HAVE_NTGUI` incl. C 4191-4201's `GNUC_PREREQ` pragma pair, C 4230-4240 `HAVE_NS`) | file gate + those five conditions; the Windows and Haiku arms are `text-only` on this machine |
| 4244-4267 | the comment (C 4244) and `image_destroy_x_image` (C 4249-4250's gate, C 4251-4266's `#else`; C 4255-4259's block is C's comment alone) | file gate; both arms |
| 4270-4291 | the comment (C 4270-4272) and `gui_put_x_image` (C 4278-4279, C 4280-4287, C 4288-4289) | file gate + the three-arm `#if`/`#elif`/`#elif` |
| 4293-4310 | the comment (C 4293-4294) and `image_create_x_image_and_pixmap` (C 4304-4306's nested pair) | file gate + `have_xrender` |
| 4312-4338 | the comment (C 4312-4316) and `image_put_x_image` (C 4322-4332, C 4333-4336) | file gate + both arms |
| 4340-4360 | C 4340's `#if (X \|\| ANDROID) && !USE_CAIRO`, the comment, `image_sync_to_pixmaps` (C 4344-4359), C 4360's `#endif` | file gate + that pair; C 1218's prototype is dropped here |
| 4362-4386 | C 4362's `#ifdef HAVE_NTGUI`, the comment and `image_get_x_image_or_dc` (C 4367-4378) / `image_unget_x_image_or_dc` (C 4380-4386) | file gate + `have_ntgui` (`text-only`); C 1201-1204's prototypes drop here |
| 4387-4443 | C 4387's `#else  /* !HAVE_NTGUI */`, the comment and `image_get_x_image` (C 4394-4420: C 4394/4396/4415's chain, C 4401/4406's pair) / `image_unget_x_image` (C 4426/4427/4439's chain), C 4443's `#endif` | file gate + `not (have_ntgui)`; C 1206-1208's prototypes drop here |
| 4446-4454 | the "File Handling" banner (C 4446-4448) and C 4450-4454's comment | file gate |
| 4456-4501 | `image_find_image_fd` (C 4463-4464, 4469, 4476-4479 and 4483-4484's comments, C 4488-4499's pair) | file gate; both arms; C 750's prototype drops here |
| 4503-4511 | the comment (C 4503-4505) and `image_find_image_file` (C 4507-4508) | file gate; **non-`static`** |
| 4513-4521 | C 4513's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`, `close_android_fd` (C 4515-4519), C 4521's `#endif` | file gate + that pair; `text-only` |
| 4523-4595 | the comment (C 4523-4526) and `slurp_file` (C 4531-4559's first arm, C 4560-4592's `#else`) | file gate; both arms; C 749's prototype drops here |
| 4597-4617 | the comment (C 4597-4599) and `slurp_image` (C 4601-4602) | file gate (`static`); **this slice's last item** |
| 4620-4624 | the "XBM images" banner (C 4620-4622) and C 4624's `static bool xbm_file_p (Lisp_Object);` | file gate; the prototype is dropped (PORTING.md 0.1-12) and its definition is C 5301's, inside this slice |
| 4627-4680 | C 4627's comment, C 4629-4647's `enum xbm_keyword_index`, C 4649-4669's `xbm_format` table and C 4671-4680's `enum xbm_token` | file gate; each enumerator is a `c_int` constant (`image_value_type`'s C 1461 shape), and C's three-member initialisers spell their two implicit zeros out |
| 4682-4797 | C 4682-4709's comment and `xbm_image_p` (C 4710-4797) | file gate (`static`); C's `memcpy` of the table and the four data shapes |
| 4800-4934 | C 4800-4804's comment and `xbm_scan` (C 4805-4934) | file gate (`static`); C 4810's `loop:` label plus C 4924's backward `goto loop` are the port's labelled loop, and the three `ckd_*` calls are the `overflowing_*` pair (P-15) |
| 4936-5003 | C 4936's `#ifdef HAVE_NTGUI`, C 4938-4969's `w32_create_pixmap_from_bitmap_data` (with C 4941-4945's `swap_nibble` table and C 4953's `alloca`), C 4971-5001's `convert_mono_to_color_image` and C 5003's `#endif` | `have_ntgui`; `text-only` on this machine (no Windows toolchain), so `<windows.h>`'s names stay C's tokens |
| 5005-5066 | `Create_Pixmap_From_Bitmap_Data` (C 5011-5016 `USE_CAIRO`, C 5017-5030 `HAVE_X_WINDOWS` incl. C 5027-5030's nested `HAVE_XRENDER`, C 5031-5040 `HAVE_ANDROID` incl. C 5033-5039's `ANDROID_STUBIFY` pair, C 5041-5047 `HAVE_NTGUI`, C 5048-5049 `HAVE_NS`, C 5050-5063 `HAVE_HAIKU`) | file gate (`static`) + those six conditions, each arm carried |
| 5068-5216 | C 5068-5075's comment and `xbm_read_bitmap_data` (C 5093-5111's three function-local macros, C 5148/C 5149's two `goto`s, C 5210-5212's three `#undef`s) | file gate (`static`); the three macros are expanded at each use site (see the Deviations), the two `goto`s are two labelled blocks |
| 5218-5295 | C 5218-5236's comment and `xbm_load_image` (C 5238-5295) | file gate (`static`) |
| 5297-5308 | C 5297-5299's comment and `xbm_file_p` (C 5300-5308) | file gate (`static`) |
| 5310-5451 | C 5310-5312's comment and `xbm_load` (C 5313-5451, with C 5416-5427's `HAVE_NTGUI` block and C 5433's `USE_SAFE_ALLOCA`) | file gate (`static`) + `have_ntgui` |
| 5453-5456 | the "Canvas" banner (C 5453-5456) | file gate (C's own banner text, kept) |
| 5457-5487 | C 5457's comment, C 5460-5471's `enum canvas_keyword_index` and C 5473-5487's `canvas_format` | file gate |
| 5489-5501 | C 5489-5495's comment, C 5497's `canvas_map`, C 5499's comment and C 5501's `canvas_list` | file gate; the two file-scope objects (`--check`'s two `missing file-scope object` rows) |
| 5503-5520 | C 5503's comment and `canvas_parse` (C 5505-5520) | file gate (`static`) |
| 5522-5529 | C 5522-5524's comment and `canvas_image_p` (C 5525-5529) | file gate (`static`) |
| 5531-5558 | C 5531-5532's comment and `canvas_free_unused` (C 5533-5558) | file gate (`static`); the `DOHASH` walk and the list rewrite |
| 5560-5659 | C 5560-5565's comment and `canvas_apply_data` (C 5576-5596's first `WORDS_BIGENDIAN` pair, C 5598-5638's vector arm, C 5640-5659's file arm with its own pair) | file gate (`static`) + `words_bigendian` at each of the two pairs |
| 5661-5710 | C 5661's comment and `canvas_get` (C 5662-5710) | file gate (`static`) |
| 5712-5737 | C 5712's comment and `canvas_load` (C 5713-5737) | file gate (`static`) |
| 5739-5849 | C 5739's comment and `canvas_prepare_for_display` (C 5760-5826 `USE_CAIRO`, C 5827-5845 `HAVE_X_WINDOWS`, C 5846-5848 `HAVE_ANDROID`, C 5850-5855 `HAVE_NS`, C 5856-5875's `#else`) | file gate (`static`) + those five conditions; the `#else` arm is the port's `not (any (…))` |
| 5851-5864 | C 5851-5852's comment and `canvas_data` (C 5853-5864) | file gate; **non-`static`, so `#[unsafe(no_mangle)]`** |
| 5866-5906 | `DEFUN ("canvas-refresh", Fcanvas_refresh, Scanvas_refresh, 1, 2, 0, …)` (C 5894-5903's `HAVE_WINDOW_SYSTEM` block) | file gate (`DEFUN!`) |
| 5909-5919 | the "XPM images" banner (C 5909-5911) and C 5913-5919's prototype pair under `#if defined (HAVE_XPM) || defined (HAVE_NS) || defined (HAVE_PGTK) || defined (HAVE_ANDROID)` | those four conditions; the pair is dropped (PORTING.md 0.1-12: both definitions are outside this slice, and nothing here calls them) |
| 5921-5942 | C 5921's `#ifdef HAVE_XPM`, C 5922-5936's `HAVE_NTGUI` arm (its `noX/xpm.h` and the three `xpm_*` renames) and C 5938-5942's `#include "X11/xpm.h"` pair | `have_xpm`; the X11 arm's declarations are the port's P-88 section (the Deviations) |
| 5944-5984 | C 5944's `#if defined HAVE_XPM \|\| …`, C 5948-5962's `xpm_keyword_index`, C 5964-5984's inner `xpm_format` table and C 5984's `#endif` | those five conditions, plus the table's own inner one |
| 5986-5996 | C 5986's `#if defined HAVE_X_WINDOWS && !defined USE_CAIRO`, its comment, C 5993-5995's `ALLOC_XPM_COLORS` derivation and C 5996's `#endif` | `all (have_x_windows, not (use_cairo))`; the derived macro is the feature `alloc_xpm_colors` (P-51) |
| 5998-6163 | C 5998's `#ifdef ALLOC_XPM_COLORS`, C 6000-6001's prototype (dropped), the `xpm_cached_color` record (C 6012-6018), `XPM_COLOR_CACHE_BUCKETS` (C 6023), C 6024's `xpm_color_cache`, `xpm_init_color_cache` (C 6028-6051), `xpm_free_color_cache` (C 6053-6071), `xpm_color_bucket` (C 6074-6081), `xpm_cache_color` (C 6086-6102), `xpm_lookup_color` (C 6108-6137), `xpm_alloc_color` (C 6143-6150), `xpm_free_colors` (C 6154-6161) and C 6163's `#endif` | `all (have_x_windows, not (use_cairo), alloc_xpm_colors)`; **this slice's last item** |
| 6166-6204 | `#ifdef WINDOWSNT`'s XPM DLL block: C 6168's comment, C 6170-6177's four `DEF_DLL_FN`, `init_xpm_functions` (C 6179-6192), C 6194-6197's four `#undef`s and C 6199-6202's four `#define X<name> fn_X<name>` renames | `windowsnt`; the block's names (`DEF_DLL_FN`/`LOAD_DLL_FN`/`get_proc_addr`/`w32_delayed_load`/`VOIDFNPTR`) reach the TU through C 131-135's `have_ntgui && windowsnt` include, so the arm is `text-only` (P-86 ③-⑵: no Windows toolchain) |
| 6206-6245 | `#if defined HAVE_XPM \|\| HAVE_NS \|\| HAVE_HAIKU \|\| HAVE_PGTK \|\| HAVE_ANDROID`'s `xpm_valid_color_symbols_p` (C 6213) and `xpm_image_p` (C 6231) | that five-way condition |
| 6247-6299 | `#if defined HAVE_XPM && defined HAVE_X_WINDOWS && !defined USE_GTK`'s `x_create_bitmap_from_xpm_data` (C 6249, non-`static` so `#[unsafe(no_mangle)]`), with C 6264-6270's `ALLOC_XPM_COLORS` block | that condition, and `alloc_xpm_colors` inside it |
| 6301-6613 | `#if defined HAVE_XPM && !defined USE_CAIRO`'s `xpm_load` (C 6307), with C 6314-6317's `HAVE_NTGUI` declarations, C 6323-6330's `#ifndef HAVE_NTGUI` visual block, C 6332-6349's `ALLOC_XPM_COLORS` pair, C 6351-6399's `:color-symbols` block, C 6409-6472's file/`data` arms (C 6432-6437 the nested `WINDOWSNT` re-encode, C 6461-6471's `XpmCreateImageFromBuffer` pair), C 6474-6510's X11 pixmap/mask block and C 6512-6604's two success/error halves | `have_xpm` + `not (use_cairo)`, plus C's own nested conditions (`have_ntgui` for the shim's arm, `have_x_windows`, `have_xrender`, `alloc_xpm_colors`) |
| 6615-7122 | the libxpm-free XPM implementation: C 6625-6630's four dropped prototypes, `enum xpm_token` (C 6634-6639), `xpm_scan` (C 6648-6705), the two colour-table pairs (C 6714-6779), `enum xpm_color_key` (C 6781-6787), `xpm_color_key_strings` (C 6789), `xpm_str_to_color_key` (C 6791-6798), `xpm_str_to_int` (C 6800-6818), `xpm_load_image` (C 6820-7084, with C 6841-6858's three function-local macros) and `xpm_load`'s second definition (C 7086-7120) | C 6615-6619's five-way condition (`use_cairo && have_xpm`, or one of `have_ns`/`have_haiku`/`have_pgtk`/`have_android` without `have_xpm`), plus `have_ns`/`have_android`/`use_cairo` inside the bodies |
| 7126-7396 | the "Color table" section: `struct ct_color` (C 7134-7141), `CT_SIZE` (C 7145), `ct_hash_rgb` (C 7149-7153), `ct_table`/`ct_colors_allocated`/`ct_colors_allocated_max` (C 7157-7167), `init_color_table` (C 7171-7177), `free_color_table` (C 7182-7197), `lookup_rgb_color` (C 7205-7290), `lookup_pixel_color` (C 7296-7345), `colors_in_color_table` (C 7351-7374) and C 7376-7396's `#else` arm (`lookup_rgb_color` C 7378-7390, `init_color_table` C 7392-7395) | C 7130's `#ifdef COLOR_TABLE_SUPPORT` / its `#else`, both under the file gate (the condition is the first slice's C 169-173 one) |
| 7398-7967 | the "Algorithms" section: C 7406/7413's two matrices, `COLOR_INTENSITY` (C 7423), `image_to_emacs_colors` (C 7432-7483), `XPutPixel` (C 7491-7527, the `HAVE_NTGUI` arm), `image_from_emacs_colors` (C 7535-7569), `image_detect_edges` (C 7580-7645, with C 7592's `COLOR`), `image_emboss` (C 7651-7655), `image_laplace` (C 7662-7666), `image_edge_detection` (C 7680-7705), `image_pixmap_draw_cross` (C 7711-7759), `image_disable_image` (C 7765-7850, with C 7805-7809's two macros) and `image_build_heuristic_mask` (C 7860-7967) | the file gate; `have_ntgui`, `have_ns`, `use_cairo`, `have_haiku` and `have_android` inside the bodies |
| 7970-8351 | C 7970-7972's "PBM (mono, gray, color)" banner, `enum pbm_keyword_index` (C 7976-7990), `pbm_format` (C 7995-8008), `pbm_image_p` (C 8012-8028, whose `return 0` is the port's `return false`), `pbm_next_char` (C 8030-8052, the comment-skipping reader), `pbm_scan_number` (C 8054-8076), `pbm_scan_index` (C 8078-8095, the one- and two-byte big-endian reader) and `pbm_load` (C 8097-8351: the magic check and the `switch (*p++)`, the width/height/max-colour reads, the mono and gray/colour pixel loops with `PUT_PIXEL`, the `#ifdef USE_CAIRO` colour pair, the `#ifdef COLOR_TABLE_SUPPORT` tail and the `free_lab`-style `error:` cleanup) | the file gate (`have_window_system`), plus C's own `use_cairo` and `COLOR_TABLE_SUPPORT` arms inside the body - **transcribed** (the seventh slice) |
| 8352-13300 | **the recorded remainder**: the `HAVE_NATIVE_IMAGE_API` arm (C 8358-8449, with `image_can_use_native_api` C 8360), the `USE_CAIRO` `xpm_load` arms, and the eight format loaders - "PNG" (C 8456-9111), "JPEG" (C 9119-9677), "TIFF" (C 9685-10112), "GIF" (C 10120-10985), "WEBP" (C 10986-11328), "ImageMagick" (C 11331-12150), "SVG" (C 12158-...), and the `HAVE_NATIVE_TRANSFORMS`/`WINDOWSNT` helpers they carry | every loader hangs on an `#if` the frozen configuration makes false (`HAVE_PNG`/`HAVE_JPEG`/`HAVE_TIFF`/`HAVE_GIF`/`HAVE_WEBP`/`HAVE_IMAGEMAGICK`/`HAVE_RSVG` are undef in `docs/baseline/config/darwin-arm64.macros.tsv`, and the condition evaluator agrees: this file's live text is 4264 lines of 13805), so the table **records** the range instead of transcribing it (P-08's shape, the comp.c/treesit.c class) and `--check` does not ask for it - the tool's inventory drops it too |
| 13301-13805 | the fourth batch's live tail: the "Tests"/"Initialization" banners, `DEFUN ("imagep")` (C 13306-13311), the `GLYPH_DEBUG`-gated `DEFUN ("lookup-image")` (C 13313-13331), `DEFUN ("image-transforms-p")` with its `HAVE_NATIVE_TRANSFORMS` three-way `#elif` chain (C 13335-13364), `DEFUN ("image-cache-size")` (C 13367-13388), `DEFUN ("init-image-library")` (C 13391-13401), `initialize_image_type` (C 13403-13432), the `image_types[]` table (C 13433-13475) with its per-format entries, `native_image_type` (C 13477-13480), `lookup_image_type` (C 13483-13503), `image_prune_animation_caches` (C 13504-13515) and `syms_of_image` (C 13516-13805, the five `DEFVAR_*` invocations, the 43 `DEFSYM`s, the `add_image_type` calls, `staticpro` and the eleven `defsubr`s) | the file gate, plus C's own `GLYPH_DEBUG`, `HAVE_NATIVE_TRANSFORMS`, `HAVE_GHOSTSCRIPT`, `WINDOWSNT`, the format conditions, `HAVE_RSVG`, `HAVE_NS`, `NS_IMPL_COCOA`, `HAVE_NATIVE_IMAGE_API` and `HAVE_IMAGEMAGICK` arms - **transcribed** (the eighth slice) |

## Evidence

**1. The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/imageport/cargo cargo run -q -p xtask -- build-tu src/image.c`
-> `built src/image.rs -> ../build/femacs/image.o`, **exit 0**, and `build/femacs/image.o` is
**336 bytes** - an empty object.  That is the expected reading, not a proof of the new text: every
item sits behind `have_window_system`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  (The budget note below records the three
*failing* `build-tu` iterations that preceded it.)

**2. The arm flip (P-86 3-1, the NS configuration).**
```
CARGO_TARGET_DIR=/tmp/imageport/cargo cargo run -q -p xtask -- probe src/image.rs \
  /tmp/imageport/probe-ns.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"'
```
-> **exit 1, 55 `error` lines** (54 rustc errors + xtask's own `rustc failed for src/image.rs`
summary) = **127 error sites, 0 of them in the unit's own text**.  By file (the `-->` anchors):
`include/frame.rs` 45, `include/dispextern.rs` 36, `include/nsterm.rs` 32, `include/font.rs` 8,
`include/window.rs` 3, `include/lisp.rs` 1, `lib/include/stdbit.in.rs` 1.  By class: 18 x
`cannot find type Emacs_Cursor` (the name `dispextern.h` 175 defines only under
`#ifndef HAVE_WINDOW_SYSTEM`, which `include/dispextern.rs` carries unconditionally - the
interaction `docs/ports/include-androidterm.md` already records), 13 x the `FRAME_*_SHIFT`
constants and 6 x `FRAME_HAS_VERTICAL_SCROLL_BARS`/`..._ON_LEFT`/`..._ON_RIGHT` in
`include/frame.rs` (the class the androidfont lane's 2c named), 8 x `E0080` layout assertions in
`include/font.rs` (measured for the frozen layout), and the NS records
`Window`/`NSArray`/`NSString`/`NSRect`/`EmacsMenu` in `include/nsterm.rs`.  **These are mirror
gaps, not this unit's** - Evidence 4 is what turns "0 errors in the unit" into a reading (P-86 7).

**3. The X+cairo flip (P-86 3-1, the second configuration).**
```
... probe src/image.rs /tmp/imageport/probe-xcairo.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_window_system"' --cfg 'feature="have_x_windows"' --cfg 'feature="use_cairo"'
```
-> **exit 1, 233 `error` lines / 302 sites; 6 sites in the unit's own text, of which 3 are errors**
(the other 3 are the `unused doc comment` warnings P-88's citation blocks carry - the same warning
class the mirrors have).  The three are the **Xlib/Xutil SDK names this arm calls and no mirror
declares** - `XCreateBitmapFromData` (C 606), `XReadBitmapFile` (C 917) and `BitmapSuccess`
(C 921) - plus the `E0428` collision `x_bitmap_stipple` (this unit's C 416 definition against
`include/xterm.rs` 190-192's declaration; owed below).  The mirrors hold the rest:
`include/xterm.rs` 202 (124 x `Atom`, 19 x `Cursor`, 13 x `Window`, 6 x `GC`, and
`XVisualInfo`/`XColor`/`Display`/`Screen`/`Drawable`/`Colormap`/`XEvent`/`KeySym`/`Visual`/
`XWMHints`/`XModifierKeymap`/`cairo_t` and the twelve `*Mask`/`TrueColor`/`StaticGray` values -
`include/xlib.rs` mirrors the surface `src/xrdb.c` needed and stops there), `include/frame.rs` 45,
`lib/include/fcntl.in.rs` 24, `include/window.rs` 13, `include/font.rs` 9 (+1 `E0133`),
`include/systime.rs` 1 (its `have_x_windows` constant assertion), `include/lisp.rs` and
`lib/include/stdbit.in.rs` 1 each.

**4. The arm-reach pre-flight, and the mutation test (P-86 7) - a *different* reading.**  The
`build-tu` materialization of the final revision (the unit's text line for line; the only
differences are xtask's `include!` path rewrites) with three patch classes and nothing else:
(1) 20 stand-ins prepended to three mirror copies - `FRAME_*_SHIFT` (as `u32`, the type
`frame_bit1` takes), the three `FRAME_HAS_VERTICAL_SCROLL_BARS*` predicates (as `c_int`, the type
their call sites compare with `!= 0`), `Emacs_Cursor` in the `dispextern.rs` copy and
`Window`/`NSArray`/`NSString`/`NSRect`/`EmacsMenu` in the `nsterm.rs` copy; (2) `include/font.rs`'s
eight layout assertions commented out (they were measured for the frozen layout); (3) the copies'
`include!` paths repointed at the patched copies:
```
rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"' units/image-arm/_root.rs \
  -o /tmp/imageport/arm.o
-> exit 0, 0 errors, 11 warnings, 31,520-byte object
```
**The configuration sweep (a *free* reading: `materialize` + direct `rustc`, no `xtask probe`).**
The two probe configurations cannot reach ten of the file's `#if` arms, so the same harness was run
over each platform configuration and the *unit's own* error sites were read out - which is what
found the defects the two flips could not:

| configuration | error lines | sites | sites in the unit's own text |
| --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 (the 336-byte object) |
| `have_window_system` + `have_ns` | 54 | 127 | 0 |
| + `have_haiku` | 29 | 56 | 0 |
| + `have_android` (stubs) | 28 | 55 | 0 |
| + `have_android` + `not_android_stubify` | 28 | 56 | 0 |
| + `have_ntgui` + `windowsnt` | 125 | 149 | 3 (`CreateBitmap`, and the two `w32_create_pixmap_from_bitmap_data` calls of the undeclared C 239) |
| + `have_pgtk` + `have_imagemagick` + `use_cairo` | 35 | 65 | 8 (all of them `FRAME_DISPLAY_INFO`/`pgtk_display_info`/`pgtk_bitmap_record`, the `include/pgtkterm.rs` placeholder gap) |
| + `have_x_windows` | 230 | 301 | 9 (the Xlib SDK names, and citation-location notes) |
| + `have_x_windows` + `use_cairo` | 232 | 302 | 6 (3 errors + 3 warnings) |

This sweep is what caught, before the probes, the two defects the NS and X+cairo configurations
cannot see: `!(STRINGP (…) != 0)` in the Haiku and Android arms (C 964/1061 - the port's `STRINGP`
is a `bool`) and `android_create_bitmap_from_data`'s two `c_uint` parameters (C 616/1080), plus the
four `f->foreground_pixel & 0xffffffff` assignments whose C truncation to the records' `uint32_t`
fields (C 705-708, 999-1002) Rust needs written out.

**Mutation test (the same harness).**  Inserting `let _mutant: c_int = c"mutant".as_ptr ();` into
`image_create_bitmap_from_file`'s body turns the output into **exit 1, 1 error**,
`error[E0308]: mismatched types ... units/image-arm/_root.rs:1300:24` - the output changes and the
error is in the unit's own text, so 4 is not an empty reading.  The harness and the mutation were
reverted (the copies are byte-identical to the pristine materialization again).

**5. The P-88 / clang readings (measured, not remembered).**  This machine has the toolkit headers:
```
$ clang -I/opt/homebrew/include -o probe probe.c && ./probe
CAIRO_FORMAT_ARGB32=0 RGB24=1 A8=2 A1=3
CAIRO_STATUS_SUCCESS=0 CAIRO_EXTEND_REPEAT=1
sizeof(Pixmap)=8 sizeof(XImage*)=8 sizeof(Display*)=8 sizeof(cairo_surface_t*)=8
BitmapSuccess=0 BitmapOpenFailed=1 BitmapFileInvalid=2 BitmapNoMemory=3
sizeof(cairo_matrix_t)=48 alignof=8
offsetof(cairo_matrix_t,xx)=0 xy=16 x0=32 yx=8 yy=24 y0=40
sizeof(cairo_user_data_key_t)=4
```
(`cairo_matrix_t` in this cairo has **no `y1` member** - the first draft of the probe asked for one
and clang refused it; the record is `xx, yx, xy, yy, x0, y0`.)
```
$ clang -o pcprobe pcprobe.c && ./pcprobe     # dispextern.h 122-131's record, as C declares it
sizeof(pointee)=24 alignof=8
offsetof width=0 height=4 data=8 bytes_per_line=16 bits_per_pixel=20
sizeof(Emacs_Pix_Container)=8
```
The unit carries all of these: the six cairo constants and the two records with their `/// C:`
lines (`cairo.h`), plus the four `const _: () = assert! ( ... )` rows on `emacs_pix_container`.

**6. The inventory gates (repo root, final revision).**
```
python3 tools/check-dead-file-coverage.py
  -> info docs/ports/image.md: 41 range row(s) cover src/image.c (13805 lines; ... blank line(s)
     between blocks need no row)
     ... 13 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  -> stale src/image.c: no longer under the threshold
     ... 87 file(s) under ten percent, 203,995 reference line(s); 90 ratchet row(s), 0 new, 3 stale
python3 tools/check-citations.py
  -> 675 file(s) scanned, 0 drift, 0 range, 491 external, 33 allowed, 0 stale
python3 tools/check-declarations.py
  -> 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
python3 tools/check-carried.py
  -> 11 carried file(s), 11 baseline row(s), 0 failure(s)
python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs | tail -3
  -> - missing file-scope object canvas_list
     - line count drifted 87.6%: C 13805 vs Rust 1714
     FAIL emacs/src/image.c -> femacs/src/image.rs
```
`--check` is **not vacuous** for this file (P-86 10): its reading is 174 `- ` rows - 84 missing
functions, 54 constants, 13 macros, 10 subrs, 10 defsubr records, 2 file-scope objects
(`canvas_map`, `canvas_list`) and the line-count row.  **Every one of them is work in the
C 1096-13805 remainder**: the nearest is C 1201 (`image_get_x_image_or_dc`), the two objects are
C 5497/5501's, and the eleven names the slice *mentions* without defining
(`image_alloc_image_color` C 100, the two `canvas_*` C 126-127, the three `image_*` C 224-228, the
`COLOR_TABLE_SUPPORT` pair C 231-236 and `slurp_file`/`image_find_image_fd`/`xbm_read_bitmap_data`
C 749-752) are exactly the prototypes the table marks `dropped`/`kept`.  Nothing this slice defines
is missing, invented, or token-short.

**7. The frozen configuration, the second slice (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/image2-lane/cargo cargo run -q -p xtask -- build-tu src/image.c`
-> `built src/image.rs -> ../build/femacs/image.o`, **exit 0**, and the object is **336 bytes**,
md5 **dbed68837694579185377dbb0821ab62** - *byte-identical* to the first slice's object, because
the file gate keeps every item out of the frozen configuration.  Four runs this round: the first
revision, the revision after the sweep's fixes, and two re-verifications (one of them a no-op
re-run of a revision whose scripted edit had failed, i.e. of identical text); every one of them
produced that md5.  That is **one over the lane's <=3 budget**, reported rather than hidden (the
first slice's precedent).  The sweep's materialization is the same one `build-tu` writes
(`build/femacs/units/image/`), and the `xtask probe` runs below materialize into
`build/probes/image/`.

**8. The multi-config sweep (a *free* reading: `materialize` + direct `rustc`).**  One row per
configuration, `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj
-C panic=abort --crate-name image <cfgfs> build/femacs/units/image/_root.rs`.  "unit-text" counts
the errors whose **primary** `-->` span is `_root.rs` (this unit's own text); "mirror" counts the
rest (the copies' own spans).  The first slice's two flips cannot reach ten of the file's `#if`
arms, so the sweep is what reads them:

| configuration | error lines | unit-text | mirror |
| --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 (the 336-byte object) |
| `have_window_system` (no arm at all - not a configuration C can build) | 38 | 35 | 2 |
| `+ have_ns` | 10 | 1 | 8 |
| `+ have_ns` + `dispextern_face_records` | 9 | 0 | 8 |
| `+ have_x_windows` | 234 | 41 | 192 |
| `+ have_x_windows` + `dispextern_face_records` | 235 | 40 | 194 |
| `+ have_x_windows` + `use_cairo` | 229 | 33 | 195 |
| `+ have_x_windows` + `use_cairo` + `dispextern_face_records` | 230 | 32 | 197 |
| `+ have_haiku` | 5 | 1 | 3 |
| `+ have_android` (stubs) | 5 | 1 | 3 |
| `+ have_android` + `not_android_stubify` | 5 | 1 | 3 |
| `+ have_ntgui` + `windowsnt` | 7 | 0 | 6 |
| `+ have_pgtk` + `use_cairo` | 11 | 8 | 2 |
| `+ have_pgtk` + `use_cairo` + `have_imagemagick` | 11 | 8 | 2 |

Every unit-text row is one of four named classes, and the sweep found **five defects in this
unit's text** - four of them in this slice's own new lines:

* the **Xlib names** the X arms keep as C's tokens (`XImage`, `GC`, `ZPixmap`, `BitmapSuccess`,
  `XCreateBitmapFromData`, `XReadBitmapFile`, `XFreePixmap`, `XGetImage`, `XGetPixel`,
  `XPutPixel`, `XPutImage`, `XCreateGC`, `XFreeGC`, `XDestroyImage`) - the first slice's rule and
  its ledger item, now with `x_create_bitmap_mask`'s twenty-odd more sites;
* `struct face`, which `include/dispextern.rs` carries only under **`dispextern_face_records`**
  (`image_ascent`'s prototype names it) - the one unit-text row the NS configuration has, and it
  is **0** with that feature on;
* the **pgtkterm placeholder** (`FRAME_DISPLAY_INFO`, `pgtk_display_info`, `pgtk_bitmap_record`) -
  the mirror gap the first slice's item 5 named;
* and the defects: `parse_image_spec`'s `keywords[i].count == 0` (the port's `count` is a `bool`),
  `image_background`'s `RED16_FROM_ULONG (bg)` (the tree's macro calls need the `!`; **all three
  of the first slice's `*16_FROM_ULONG` families carried C's text and only surface when they are
  expanded - this slice's `image_background` is the first expansion**), and `free_image`'s
  `(*c).buckets[…]`/`(*c).images[…]` (C's array indexing on a raw pointer needs
  `.offset (…)`).  The last two are the class the frozen and NS readings cannot see: they are
  downstream of the name-resolution errors, which is what Evidence 9 is for.

**9. The arm-reach pre-flight (P-86 7) and the mutation test.**  The sweep's NS row stops at name
resolution (eight mirror sites), so it never type-checks this unit; the pre-flight is the same
materialization with **only the obstacles patched in the copies** - the four declarations the
mirror lane's this-batch `include/dispextern.rs` block added that collide with this unit's own
copies (see the Deviations), `Emacs_GC` and `NSArray`/`NSString`/`EmacsMenu` as stand-ins, and
`font.rs`'s eight plus `dispextern.rs`'s fifteen frozen-layout assertions commented out (all
measured for the frozen layout, the first slice's reading), and `frame.rs`'s `FRAME_IMAGE_CACHE!`
copy given the `as *mut image_cache` cast its consumers need (see the Deviations):

```
rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"' \
  --cfg 'feature="dispextern_face_records"' /tmp/…/arm/_root.rs -o arm.o
-> exit 0, 0 errors, 17 warnings, 65,968-byte object
```

Three mutations of the final text, run through the same harness (`reroot` + direct `rustc`, the
same two steps `xtask probe` runs):

| mutation | exit | errors | where |
| --- | --- | --- | --- |
| none (the clean text) | 0 | 0 | - |
| `DEFAULT_IMAGE_ASCENT` assigned through `as *mut c_void` (a type error in `make_image`) | 1 | 1 | `_root.rs:2704` (the mutated line) |
| `DEFAULT_IMAGE_ASCENT`'s **definition** renamed | 1 | 2 | `_root.rs:2235` (the definition) and `_root.rs:2704` (the use) |
| `&&` → `||` in `prepare_image_for_display`'s `load_failed_p` test (semantic) | 0 | 0 | **invisible to the compiler** |

The first two are the proof that the harness reads this unit's own text (a mutation the frozen
and NS readings cannot see becomes an error); the third is the honest boundary - **the compiler
is not a semantic check**, and neither this sweep nor the pre-flight is a behavioural test (the
file's loaders and the `DEFUN`s are not run anywhere yet).

**10. The inventory gates (repo root, this revision).**
```
python3 tools/check-dead-file-coverage.py
  -> info docs/ports/image.md: 75 range row(s) cover src/image.c (13805 lines; 70 blank line(s)
     between blocks need no row)
     ... 15 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  -> 86 file(s) under ten percent, 195,581 reference line(s); 87 ratchet row(s), 0 new, 1 stale
     (src/w32term.c; src/image.c left the class with the first slice and its row is gone)
python3 tools/check-citations.py
  -> 675 file(s) scanned, 0 drift, 0 range, 494 external, 33 allowed, 0 stale
python3 tools/check-declarations.py
  -> 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
python3 tools/check-carried.py
  -> 11 carried file(s), 11 baseline row(s), 0 failure(s)
```

**11. What `--check` still owes.**  `python3 tools/extract-abi.py --check emacs/src/image.c
femacs/src/image.rs | tail -3` reads **139 `- ` rows** (the first slice's reading was 174, so
this batch cleared 35): 61 missing functions, 42 missing constants, 10 missing `defsubr` records,
7 missing macros, 7 missing subrs, 2 missing file-scope objects (`canvas_map`, `canvas_list`),
**9 token/declaration rows this slice's own definitions produced**, and the line-count row
(`line count drifted 76.8%: C 13805 vs Rust 3202`).  Every one is work in the C 2212-13805
remainder - the nearest is `cache_image`/`make_image_cache` (C 2219/2224) - except the ten rows
the `## Owed by the integrator` list names: the nine token rows are the tree's
`docs/ports/macro-body-notes.tsv` class (`image_error`'s `va_list`/`va_start`/`va_end` under
P-04; `parse_image_spec`'s `switch`/`case`/`default`/`goto`/`maybe_done` under PORTING.md 2.2;
`image_spec_value`'s `0`/`1`, `check_image_size`'s `1` and `four_corners_best`'s `uninit` under
1.5; `image_background`'s `background_valid` and
`image_background_transparent`/`image_clear_image_1`'s `background_valid`/
`background_transparent`/`background_transparent_valid` under 1.3/P-80) plus the
`lookup_image` row, which disappears with the definition's slice.

**12. The frozen configuration, the third slice (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/img3-lane/cargo cargo run -q -p xtask -- build-tu src/image.c`
-> `built src/image.rs -> ../build/femacs/image.o`, **exit 0**, and the object is **336 bytes**,
md5 **dbed68837694579185377dbb0821ab62** - *byte-identical* to the two earlier slices' object,
because the file gate keeps every item out of the frozen configuration (`have_window_system` is
`undef` there).  Three runs this round, one after each edit round (the first append; the
type-check fixes the sweep found; the last fixes), all exit 0 and all that md5 - the lane's
`build-tu` budget is **3, exactly used**.

**13. The multi-config sweep (a *free* reading: `materialize` + direct `rustc`).**  One row per
configuration, `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj
-C panic=abort --crate-name image <cfgfs> build/femacs/units/image/_root.rs`.  "unit-text"
counts the errors whose **primary** `-->` span is `_root.rs` (this unit's own text); "mirror"
counts the rest.  This round's sweep adds the four configurations the earlier tables could not
reach (`+ have_xrender`, `+ have_imagemagick`, `+ have_native_transforms`, `+ have_imagemagick +
have_rsvg`) plus the gate combinations that exercise `image_set_transform`'s arms:

| configuration | error lines | sites | unit-text | mirror |
| --- | --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 | 0 (the 336-byte object) |
| `have_window_system` (no arm at all - not a configuration C can build) | 44 | 43 | 34 | 9 |
| `+ have_ns` | 14 | 28 | 3 | 25 |
| `+ have_ns` + `dispextern_face_records` | 27 | 40 | 2 | 38 |
| `+ have_x_windows` | 47 | 81 | 29 | 52 |
| `+ have_x_windows` + `dispextern_face_records` | 61 | 98 | 28 | 70 |
| `+ have_x_windows` + `use_cairo` | 44 | 76 | 30 | 46 |
| `+ have_x_windows` + `use_cairo` + `dispextern_face_records` | 58 | 93 | 29 | 64 |
| `+ have_x_windows` + `have_xrender` | 50 | 84 | 31 | 53 |
| `+ have_haiku` | 11 | 18 | 3 | 15 |
| `+ have_android` (stubs) | 10 | 17 | 4 | 13 |
| `+ have_android` + `not_android_stubify` | 10 | 18 | 5 | 13 |
| `+ have_ntgui` + `windowsnt` | 29 | 53 | 6 | 47 |
| `+ have_pgtk` + `use_cairo` | 23 | 46 | 22 | 24 |
| `+ have_pgtk` + `use_cairo` + `have_imagemagick` | 23 | 46 | 22 | 24 |
| `+ have_imagemagick` (reaches `image_set_transform`, but is not a configuration C can build) | 44 | 158 | 83 | 75 |
| `+ have_native_transforms` (the other arm of the same gate) | 44 | 158 | 83 | 75 |
| `+ have_ns` + `have_native_transforms` | 14 | 28 | 3 | 25 |
| `+ have_ns` + `have_native_transforms` + `dispextern_face_records` | 27 | 40 | 2 | 38 |
| `+ have_imagemagick` + `have_rsvg` | 44 | 158 | 83 | 75 |
| `+ have_x_windows` + `have_imagemagick` | 47 | 81 | 29 | 52 |
| `+ have_x_windows` + `have_native_transforms` | 47 | 81 | 29 | 52 |
| `+ have_android` + `not_android_stubify` + `have_native_transforms` | 10 | 18 | 5 | 13 |
| `+ have_haiku` + `have_native_transforms` | 11 | 18 | 3 | 15 |

Note the `have_window_system`-only row: with no platform arm selected, the platform-exclusive
mirror names (`Display_Info`, `Bitmap_Record`, `Emacs_Pix_Container`, `Emacs_Pix_Context`, the
`GET_PIXEL!`/`PUT_PIXEL!`/`NO_PIXMAP!`/`PIX_MASK_*` macro arms) are configured out, so that row's
34 unit-text rows are the earlier slices' lines - C cannot build that configuration either
(configure.ac 7521 picks a platform with the same test that fills `WINDOW_SYSTEM_OBJ`).
**No unit-text row is in the third slice's own lines in any of the 24.**  Every unit-text row
belongs to one of four named classes, and three of them are the *earlier* slices':

* the X arm's Xlib surface (`XGetPixel`/`XPutPixel`/`XPutImage`/`XGetImage`/`ZPixmap`/
  `BitmapSuccess`/`GC`/`XImage` in slice 1's `x_create_bitmap_mask` and `image_create_bitmap_from_data`
  - the first slice's ledger item; the mirror lane's in-flight `include/xlib.rs`/`xterm.rs` work
  resolves part of it, which is why the X rows above are smaller than the earlier table's);
* `struct face` without `dispextern_face_records` (`image_ascent`'s prototype, slice 2) - 0 with
  the feature on;
* slice 1's `image_create_bitmap_from_data` Windows arm's `nbytes` arithmetic (the `CHAR_BIT`
  mixed-width row) and slice 2's `x_create_bitmap_mask` reads;
* the mirror gaps the arms keep as C's tokens (the pgtkterm placeholder, `XRender*`/`Filter*`/
  `XTransform`/`XDoubleToFixed`, `XFORM`'s fields, the NS SDK records) - the `text-only` classes
  below.

**The mirror lane's in-flight edits move the mirror column.**  `femacs/src/{include/dispextern,
frame,xlib,xterm,w32gui,w32term}.rs` were being written *while* this sweep ran (`git status` at
the time: all six modified, 08:26), so the mirror-side counts are a reading of that state, not of
`HEAD`; the **unit-text** column is the one this slice answers for, and re-running the sweep
against a settled mirror will move only the mirror column (P-87 2's rule: an in-flight worktree
and a HEAD worktree are two different readings).  The first sweep of this round, on the same unit
text with the earlier mirror, read `+ have_ns` 21/37/7, `+ have_x_windows` 256/352/86, `+
have_android` + `not_android_stubify` 54/98/63 - the same conclusion, larger mirror numbers.

**14. The arm-reach pre-flight (P-86 7) and the mutation test - a *different* reading.**  The
sweep's rows stop at name resolution in the platform mirrors, so they never type-check the whole
of `image_set_transform`; the pre-flight is the same materialization with **only the obstacles
patched in the copies** - four declarations the mirror has not landed (`NSArray`, `NSString`,
`EmacsMenu`, `Emacs_GC` in the `nsterm.rs` copy), the copies' `const _` layout assertions
commented out (they were measured for the frozen layout), and a `reroot` (the materialization's
`include!` paths rewritten to the copy directory):

```
rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"' \
  --cfg 'feature="have_native_transforms"' --cfg 'feature="dispextern_face_records"' \
  /tmp/img3-lane/arm/_root.rs -o arm/out.o
-> exit 0, 0 errors, 12 warnings, 96,584-byte object
```

Three mutations of that text, run through the same harness (a copy per mutation):

| mutation | exit | errors | where |
| --- | --- | --- | --- |
| none (the clean text) | 0 | 0 | - |
| `make_image_cache`'s `(*c).size = 50;` -> `= core::ptr::null_mut ();` (a type error in the cache section) | 1 | 1 | `_root.rs:3205` (the mutated line) |
| this slice's `DBL_MAX` **definition** renamed | 1 | 2 | `_root.rs:4381` and `_root.rs:4387` (its two uses in `image_set_transform`'s matrix arms) |
| `&&` -> `||` in `postprocess_image`'s `else if NILP (mask) && found_p && !(*img).mask.is_null ()` (semantic) | 0 | 0 | **invisible to the compiler** |

The first two are the proof that the harness reads this slice's own text in both halves (the
cache section and the transform section); the third is the honest boundary - **the compiler is
not a semantic check**, and neither the sweep nor the pre-flight is a behavioural test (the
cache, the two `DEFUN`s and `image_set_transform` are not run anywhere yet).

**15. The P-88 / clang readings (measured, not remembered).**  This round's new third-party names
are the cairo filter constants and `DBL_MAX`:
```
$ clang -I/opt/homebrew/include -o probe probe.c && ./probe
CAIRO_FILTER_FAST=0 GOOD=1 BEST=2 NEAREST=3 BILINEAR=4 GAUSSIAN=5
DBL_MAX=1.7976931348623157e+308
```
(gcc's `<float.h>` spelling of the target's `DBL_MAX`; the four `CAIRO_FILTER_*` the unit reads
are the first four, `cairo.h` 259-266.)

**16. The inventory gates (repo root, this revision).**
```
python3 tools/check-dead-file-coverage.py
  -> info docs/ports/image.md: 92 range row(s) cover src/image.c (13805 lines; 75 blank line(s)
     between blocks need no row)
     ... 17 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  -> stale src/android.c: no longer under the threshold
     85 file(s) under ten percent, 187,987 reference line(s); 86 ratchet row(s), 0 new, 1 stale
     (src/android.c - another lane's file, in flight)
python3 tools/check-citations.py
  -> 675 file(s) scanned, 0 drift, 0 range, 620 external, 33 allowed, 0 stale
python3 tools/check-declarations.py
  -> 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
python3 tools/check-carried.py
  -> 11 carried file(s), 11 baseline row(s), 0 failure(s)
```
`python3 tools/check-port-lines.py --write-pending` is **not** run: it rewrites
`docs/baseline/port-lines-pending.tsv`, which is outside this lane's writable set (the read-only
form reads 0 new rows, so nothing is pending for `src/image.c`).

**17. What `--check` still owes.**
`python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs` reads **121 `- `
rows** (the second slice's reading was 139, so this batch cleared **18**): 50 missing functions,
42 missing constants, 10 missing `defsubr` records, 7 missing macros, 5 missing subrs, 2 missing
file-scope objects (`canvas_map`, `canvas_list`), the line-count row (`line count drifted 64.6%:
C 13805 vs Rust 4881`), the two rows this slice's own definitions produced
(`clear_image_cache does not mention inhibit_clear_image_cache, max` and
`image_compute_scale does not mention goto, invalid_value` - the tree's
`docs/ports/macro-body-notes.tsv` class, Owed 12), and the two kept declarations
(`lookup_image` and `image_prune_animation_caches`, each "not static in C but has no
`#[unsafe(no_mangle)] in the port" - no C symbol to link, the slice-2 row's class).  Every row
except those four is work in the C 3552-13805 remainder (the nearest is `lookup_image`, C 3556).

**18. The fourth slice's two pre-flight gates, and its frozen reading.**
```
$ python3 tools/check-cond-eval.py src/image.c
src/image.c: 435 conditions, 0 unknown, 0 disagreements [VERIFIED]
$ python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs     # before this batch
119 `- ` rows (50 missing functions, 42 missing constants, 10 missing defsubr records, 7 missing
macros, 5 missing subrs, 2 missing file-scope objects, the line-count row and the two kept
declarations' rows) -> FAIL
$ cd femacs && CARGO_TARGET_DIR=/tmp/image-lane/target cargo run -q -p xtask -- build-tu src/image.c
built src/image.rs -> ../build/femacs/image.o     (exit 0; **336 bytes** - the empty object,
Evidence 1's reading, because every item sits behind `have_window_system`)
```
The `C line ranges` table this round replaces had said "`3552-13805` the remainder": the rows
above turn **C 3552-4623** (1,072 C lines) into real rows and leave `4620-13805`.

**19. The arm flips (P-86 3), this round.**  Two configurations were compiled, both with
`--cfg 'feature="dispextern_face_records"'` (the condition `include/dispextern.rs` gates
`struct face`/`struct face_cache` on, and Owed 4's item).

*The NS configuration* (`have_window_system` + `have_ns` - this machine's own platform, the one
Evidence 2/9 carry):
```
$ rustup run nightly-2026-10-01 rustc --edition 2024 -O build/probes/image/_root.rs \
    -o /tmp/image-lane/arm-ns.o --crate-type lib --emit obj -C panic=abort \
    --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"' \
    --cfg 'feature="dispextern_face_records"'
-> exit 0, **0 errors**, 19 warnings
```
`build/probes/image/_root.rs` is what `xtask probe` materializes (the unit's text with xtask's
`include!` paths and dedupes; the first reading of this round was the `xtask probe` form itself -
**exit 1, 12 errors, all 12 in this slice's new text**: `XLIB_BYTES_MAX`'s `<` parsed as a generic
argument, `eabs` called without its `!`, `builtin_lisp_symbol`'s `usize`, the three `font *` reads,
`share_image_cache` undeclared, `file_found`'s `E0381`, `face_id` not `mut`, and the two cfg'd
assignments - all fixed before the reading above).

*The X+cairo configuration* (`have_window_system` + `have_x_windows` + `use_cairo`):
```
-> exit 1, **69 `error` diagnostics** (the 70th line is rustc's "aborting due to" summary): **62
anchored in the unit's text, 7 in mirror copies**; of the 62, **2 are this slice's** and **60 are
pre-existing** (they are below this slice's first line, `_root.rs` 4854's banner)
```
The two in this slice are the **Xlib names no mirror declares** - `XCreateImage` (C 3949) and
`XCreatePixmap` (C 3976) - i.e. Evidence 3's `XCreateBitmapFromData`/`XReadBitmapFile` class.  The
60 pre-existing ones are the same configuration's known state: 41 are `no field
bitmaps`/`bitmaps_last`/`bitmaps_size`/`display`/`root_window` on
`dispextern_unit_names::x_display_info` (the mirror's X-arm `Display_Info` alias is the *opaque*
tag while this unit reads the record's members - Owed 16) spread over the **first** slice's
`image_allocate_bitmap_record` family (C 885-1812) and the **third** slice's rows (C 2750/2937),
and 19 are `E0308`s in the first slice's `x_create_bitmap_mask` (C 1903/1923/1925's `XGetImage`/
`XGetPixel` argument types) and the third slice's C 2750/2751/2937.  **Nothing in this slice's own
text fails to type-check in either flipped configuration except those two SDK names.**

*Not flipped* (P-86 3-2): the **Windows** (`have_ntgui`), **Haiku** (`have_haiku`), **PGTK**
(`have_pgtk`) and **Android** (`have_android`) arms, and the **plain-X11** (`have_x_windows`
without `use_cairo`) configuration.  `which x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc` is empty
and `/opt/homebrew/include` has no `gtk`/`pango`/Haiku tree; the plain-X11 configuration is the
one where the unit's and the mirror's `Emacs_Pix_Container` X-arm declarations collide (`E0428`,
Owed 16).  Those arms' evidence is the frozen reading above, the `check-cond-eval` verdicts and
the per-item C citation, and the ledger says `text-only` for each.

**20. What `--check` still owes, after this batch.**
`python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs` reads **105 `- ` rows**
(119 before, so this batch cleared **16**): 34 missing functions, 42 missing constants, 10 missing
`defsubr` records, 7 missing macros, 5 missing subrs, 2 missing file-scope objects
(`canvas_map`, `canvas_list`), the line-count row (`line count drifted 55.2%: C 13805 vs Rust
6182`), the one kept declaration (`image_prune_animation_caches`, Owed 13), and **three rows this
slice's own definitions produced**: `lookup_image does not mention background_valid`,
`image_check_image_size does not mention 1` and `image_find_image_file does not mention 0` (the
tree's `docs/ports/macro-body-notes.tsv` class - Owed 14).  The thirty-four missing functions are
the XBM / XPM / canvas / PNG / JPEG / TIFF / GIF / WebP / SVG / ImageMagick readers from C 4624
onward; the nearest is C 4637's `xbm_image_p` region (whose prototype line C 4624 is).

**21. The XPM surface's layout (P-88 ②), measured with this machine's clang.**
`<X11/xpm.h>` has no file in the reference tree and none on this machine (`ls
/opt/homebrew/include/X11/xpm.h` is empty), so the upstream header was fetched *for the
measurement only* - the unit declares the surface itself and needs no header at build time:
```
$ curl -sSL -o /tmp/xpm.h https://gitlab.freedesktop.org/xorg/lib/libxpm/-/raw/master/include/X11/xpm.h
$ sha256sum /tmp/xpm.h
652b9aa90b3c5a516b367c1a4bed0658920505290657ea0d9362152a3e7c6388
$ clang -I/opt/homebrew/include -o p p.c && ./p     # sizeof/offsetof of every member
```
-> `XpmColorSymbol` **24** (name 0, value 8, pixel 16); `XpmAttributes` **224**, align **8**
(valuemask 0, visual 8, colormap 16, depth 24, width 28, height 32, x_hotspot 36, y_hotspot 40,
cpp 44, pixels 48, npixels 56, colorsymbols 64, numsymbols 72, rgb_fname 80, nextensions 88,
extensions 96, ncolors 104, colorTable 112, hints_cmt 120, colors_cmt 128, pixels_cmt 136,
mask_pixel 144, exactColors 148, closeness 152, red_closeness 156, green_closeness 160,
blue_closeness 164, color_key 168, alloc_pixels 176, nalloc_pixels 184, alloc_close_colors 188,
bitmap_format 192, alloc_color 200, free_colors 208, color_closure 216); `Pixel` 8 bytes,
`Bool` 4; `XpmSuccess` 0, `XpmOpenFailed` -1, `XpmFileInvalid` -2, `XpmNoMemory` -3,
`XpmColorFailed` -4; `XpmColorSymbols` 64, `XpmAllocColor` 524288, `XpmFreeColors` 1048576,
`XpmColorClosure` 2097152.  The unit's `#[repr(C)]` records repeat every one of those numbers as
a `const _: () = core::assert! (…)`, so the declaration is machine-checked rather than asserted.

**22. The frozen configuration (`build-tu`), and what it actually checks.**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/img-lane cargo run -q -p xtask -- build-tu src/image.c
built src/image.rs -> ../build/femacs/image.o     (exit 0; `build/femacs/image.o` is **336 bytes**
- the empty object, Evidence 1's reading: every item sits behind `have_window_system`)
```
`build-tu` passes no `--cfg` at all (`xtask` 1031-1037), so it *compiles* nothing of this file -
but it does **parse** the whole text, and that is what this round's first two iterations caught:
7 syntax sites, all of them the `as T <` generic-argument ambiguity (`bool_vector_size (data) /
height as EMACS_INT <…`, `while i as c_uint < (*attrs).numsymbols`), plus 2 x `E0277` for the two
`image_keyword` tables (a `static`'s type must be `Sync`, and a raw pointer is not; fixed with
`unsafe impl Sync for image_keyword` - `src/font.rs` 529's `table_entry` shape).  A direct
`rustc --edition 2024 --crate-type lib --emit=metadata` of the unit text with that same feature
set reads **exit 0, 0 errors, 0 warnings**.

**23. The required probe reading (the file gate).**
```
$ CARGO_TARGET_DIR=/tmp/img-lane cargo run -q -p xtask -- probe src/image.rs /tmp/img-lane/x.rlib \
    --crate-type=lib -C panic=abort --cfg 'feature="have_window_system"'
-> exit 1, 60 `error` lines, 86 error sites: **10 in this slice's text**, 76 in the text of the
four earlier slices and the mirrors
```
All ten of this slice's sites are **one class** - `NO_PIXMAP` (5), `Emacs_Pixmap` (2),
`Emacs_Pix_Container` (2) and `PUT_PIXEL` (1): the platform typedefs and macros C defines inside
its `#ifdef HAVE_X_WINDOWS`/`HAVE_NTGUI`/`HAVE_NS`/`HAVE_HAIKU`/`HAVE_ANDROID`/`USE_CAIRO` arms
(C 67-222).  `configure.ac` 7521 fills `WINDOW_SYSTEM_OBJ` only under `HAVE_WINDOW_SYSTEM`, which
configure sets *from* those platform macros, and C 64's `TERM_HEADER` has no arm for a
window-system build without one - so **this feature set is not a configuration C builds**, and
the 76 pre-existing sites are the same class (56 in `_root.rs` before this slice's first line, 20
in `include/dispextern.rs`/`frame.rs`/`termhooks.rs`/`lisp.rs`/`stdbit.in.rs`).  The same reading
is what makes this slice's two `NO_PIXMAP`-comparison `E0308`s (C 5271's and C 5443's arms)
recovery noise rather than type errors.

**24. The arm flips (P-86 3), this round.**  One `xtask probe` configuration and a twelve-row
sweep over the platform arms, all on this machine:
```
$ CARGO_TARGET_DIR=/tmp/img-lane cargo run -q -p xtask -- probe src/image.rs /tmp/img-lane/ns.rlib \
    --crate-type=lib -C panic=abort --cfg 'feature="have_window_system"' --cfg 'feature="have_ns"' \
    --cfg 'feature="dispextern_face_records"'
-> exit 0, **0 errors** (11 warnings, all of them the mirrors' `unused doc comment` class)
```
The sweep is the ledger's Evidence 14 shape (`build/probes/image/_root.rs` + a direct `rustc`;
the second column counts the `error` lines, the third the sites *in this slice's own text* - the
materialized root's line 6185 is where this slice starts):

| configuration | `error` lines | sites in this slice's text |
| --- | --- | --- |
| `have_window_system` + `have_ns` + `dispextern_face_records` | 0 (exit 0) | 0 |
| + `have_haiku` + `dispextern_face_records` | 3 | 0 |
| + `have_android` + `dispextern_face_records` | 9 | 0 |
| + `have_android` + `not_android_stubify` + `dispextern_face_records` | 2 | 0 |
| + `have_ntgui` + `windowsnt` + `dispextern_face_records` | 51 | 0 |
| + `have_x_windows` + `dispextern_face_records` | 2 | 0 |
| + `have_x_windows` + `have_xpm` + `alloc_xpm_colors` + `dispextern_face_records` | 2 | 0 |
| + `have_x_windows` + `have_xrender` + `dispextern_face_records` | 15 | 0 |
| + `have_x_windows` + `use_cairo` + `dispextern_face_records` | 68 | 0 |
| + `have_pgtk` + `use_cairo` + `dispextern_face_records` | 9 | 0 |
| + `use_cairo` + `dispextern_face_records` (no platform) | 23 | 1 (the degenerate class of Evidence 23: C's `terminal.query_colors` is gated on `have_x_windows`/`have_ntgui`/`have_pgtk`/`have_android` in `termhooks.h` 615-621, so C would not build this either) |
| `have_window_system` + `have_xpm` (no platform) | 59 | 10 (Evidence 23's class) |

The first four iterations of the sweep **did** find four sites in this slice's text, all fixed
before the table above: `let value` assigned twice in `xbm_load_image` (now `mut value`), the
`SAFE_NALLOCA!` buffer `bits` read before assignment (`lisp.rs`'s own note says the call site must
initialize it; now C's zero), the Android canvas arm's `PUT_PIXEL` pixel (`uint32_t` ->
`unsigned long`, written out) and the Haiku arm's `data += bytes_per_line` (the parameter is
`mut data`).  Two more came from the same runs: `lookup_rgb_color`'s use-site declaration had to
carry the *union* of its call sites (including `canvas_prepare_for_display`'s platform-independent
`#else` arm) and not just the three named platforms.  The remaining third-column cells are the
earlier slices' registered classes (Evidence 16/17's `Emacs_Pix_Container` `E0428`,
`x_display_info`'s opaque `Display_Info`, the pgtk placeholder, the text-only `<windows.h>`
names), and two of the previous rounds' own classes are **gone**: `XCreateImage`/`XCreatePixmap`
(Evidence 19's two errors - this slice declares them) and the Xrender surface of Evidence 15's
`x_render` row (`XRenderFreePicture`/`RepeatPad` are still missing, the row is 15 lines, 0 in this
slice's text).

**25. What `--check` reads now.**
`python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs` reads **54 `- ` rows**
(102 before, so this batch cleared **48**): 19 missing functions (the PNG/JPEG/TIFF/GIF/WebP/SVG/
ImageMagick/ImageMagick readers and `syms_of_image` and the colour table's five), 12 missing
constants (all `PBM_*`, the Algorithms section's), 7 missing macros (`COLOR`, `COLOR_INTENSITY`,
`CrossForeground`, `MaskForeground` - the Algorithms section's - and **`match`/`expect`/
`expect_ident`, the three function-local macros of `xbm_read_bitmap_data`**, owed below), 4 missing
subrs, 10 missing `defsubr` records (`syms_of_image` is C 13xxx's), the line-count row
(`C 13805 vs Rust 8420`), and the one pre-existing kept-declaration row
(`image_prune_animation_caches`).  Five rows this slice's own definitions first produced are
**cleared by this round's `docs/ports/macro-body-notes.tsv` append** (that file is in this
slice's writable set): `xbm_scan` - `goto,uninit`; `xbm_read_bitmap_data` -
`expect,expect_ident,ident,match,token,goto`; `xbm_load_image` - `background_valid`;
`xbm_file_p` - `1`; `canvas_load` - `background_valid,background_transparent_valid`.

## Deviations

- **The two object-like `#define` families are written as `macro_rules!`, one per arm, with C's
  use-site arity.**  C 71/93/94/139/157/180 are object-like (`#define PUT_PIXEL XPutPixel`,
  `#define GET_PIXEL image_pix_context_get_pixel`) while C 138/204/205 are function-like; an
  object-like `#define` of an identifier has no Rust counterpart (the `lcms.rs` 82-88 rule), and
  the ported bodies must keep C's tokens for the body gate.  Each arm's macro therefore takes the
  four (or three) parameters its call sites pass, which is C's *preprocessed* call site - so
  `PUT_PIXEL!`/`GET_PIXEL!` stand in the ported bodies exactly where C's do.
- **`#define n_planes n_image_planes` (C 221) renames a *field***, which no Rust spelling reaches:
  the Android arm's text is written the way C's preprocessor leaves it (the `lcms.rs` rule), and
  this slice writes no `n_planes` at all (the read is C 11219's).
- **`# define COLOR_TABLE_SUPPORT 1` (C 172) and `# define
  DONT_CREATE_TRANSFORMED_IMAGEMAGICK_IMAGE` (C 110) have no body.**  The port spells their
  *definedness* as a `pub const ...: bool = true` (the `XTERM_H` / `include/lisp.rs` 2913
  `HAVE_EXT_MENU_BAR` shape), and C 169-173's derived condition is written out at each
  `#ifdef COLOR_TABLE_SUPPORT` site.
- **Static prototype + definition pairs in one translation unit** (PORTING.md 0.1-12): C 126-127,
  224-229, 231-232, 233-236 and 244-246 are dropped, and their definitions keep C's place in the
  slice that reaches them.  **Kept as declarations** are the four whose definitions are *outside*
  this slice while the slice calls or needs them: `image_alloc_image_color` (C 100/2318),
  `slurp_file` (C 749/4529), `image_find_image_fd` (C 750/4457) and `xbm_read_bitmap_data`
  (C 751-752/5080).  C 239-240's `w32_create_pixmap_from_bitmap_data`/`XPutPixel` are **not**
  declared: the Windows arm is text-only on this machine, and declaring them would need
  `<windows.h>`'s `HBITMAP`/`COLORREF`/`XImage`, which no mirror carries (`include/w32gui.rs` 202
  gates its `XImage` alias on `windows` rather than on the arm's feature, and `src/w32fns.rs` 250
  declares `COLORREF` inside that unit) - Evidence 4's NTGUI column is where that shows.
- **The three `lisp.h` declarations the mirror does not carry** (`memory_full` 4432, `xlispstrdup`
  5553, `openp` 4743) stand at the use site with C's own prototypes; `src/fns.rs` 156 and
  `src/lread.rs` 1888 declare two of the same three.  They belong in `include/lisp.rs` (P-72 rule
  4), and this unit drops its copies in the commit that adds them.
- **Uninitialised locals are zeroed** (PORTING.md 1.5/0.1-13): `fd`/`size`/`width`/`height`/`data`/
  `contents`/`bitmap`/`found`/`tmp`/`x`/`y`/`bytes_per_line`/`rc`/`xhot`/`yhot`/`result`/
  `filename`/`id` in `image_create_bitmap_from_file` (C 763-767, 810-813, 855-857, 890-895,
  940-944, 1029-1034 - each is written through `&x` or assigned before its first read) and `matrix`
  in `cr_put_image_to_cr_data` (C 357, filled by `cairo_pattern_get_matrix` before it is read).
- **C's implicit conversions are written out** where Rust would not infer them: `make_fixnum (R_OK)`
  becomes `make_fixnum (R_OK as EMACS_INT)` (C 911/960/1052/1057, the four `openp` calls);
  `CHAR_BIT` mixed with the `unsigned int` counters (C 295/636/790) and with `size_t` arithmetic;
  `height`/`width` (`unsigned int`) into `int` record fields (C 712-713, 799-800, 880-881, 930-931,
  996-997, 1089-1090); `pixel as uint32_t`/`as uint8_t` (C 274/276); the `*mut c_void` <->
  `Emacs_Pix_Container` casts the mirror's degraded `pixmap`/`mask`/`cr_data` members need
  (C 350-351, 355/361/362, 367); `bitmap`/`stipple` into the Android record (`android_pixmap` is
  `intptr_t`); and `XBM_BIT_SHUFFLE`'s C `~` as Rust's `!` on the same `char`.
- **C's predicates are the port's `bool`** (`STRINGP`, `NILP`): C 769/815/859/964/1061 write
  `!STRINGP (...)`/`!NILP (...)`, and the port writes the same text without the `!= 0` C needs;
  C's `rc = xbm_read_bitmap_data (...)` (a `bool` in C) is `if ... { 1 } else { 0 }` because the
  port's declaration returns `bool` while C's `rc` is an `int`.
- **`while (height--)`** (C 562, 574) is the tree's `while { height -= 1; height >= 0 }`
  (`xfaces.rs` 1398's shape): same test, same final value.
- **C's own quirks are kept as written**: C 682's `bits[8]` in the Haiku arm (a fixed 8, not
  `x / 8`; noted at the line), and C's two swapped assignments - `height = width; width = height;`
  (C 799-800, Windows) and `height = ns_image_width (bitmap); width = ns_image_height (bitmap);`
  (C 846-847, NS) - in C's order.
- **`image_create_bitmap_from_file` has no statement after its last platform arm** (C 755-1095).
  With none of the six compiled it falls off its end, which C reports as a warning and Rust as
  `E0308`; the port gives the no-platform arm a documented `unreachable! ()` (`lread.rs` 719's
  shape).  A window-system configuration always selects one of the six (configure.ac 7521 fills
  `WINDOW_SYSTEM_OBJ` from the same test that picks the platform, and `dispextern.h` defines
  `Display_Info` only per platform), so that arm is unreachable in every configuration that exists.
- **Three more C conversions/degrades the sweep found, and their spelling**: C 705-708/999-1002's
  `f->foreground_pixel & 0xffffffff` assigned to the records' `uint32_t` fields is
  `((*f).foreground_pixel & 0xffffffff) as uint32_t` (C's `&` on an `unsigned long` keeps the low
  word and the store truncates); `android_create_bitmap_from_data (bits, width, height)`
  (C 616/1080) is `(…, width as c_uint, height as c_uint)`, the mirror's own parameter type; and
  C 738's `#include "android.h"` **is** an `include!("include/android.rs")` under
  `all (have_window_system, have_android, not_android_stubify)` - `image_fd`'s `struct
  android_fd_or_asset` is that header's and `include/androidterm.rs`'s chain does not reach it.
  C 137-138's Windows pair (`Emacs_Pix_Container`, `Emacs_Pix_Context`) is the untyped pointer for
  the same reason: `include/w32gui.rs` 202 gates its `XImage` on `windows`, and `HDC` has no
  mirror at all.
- **Four per-platform names the mirrors lack are declared at the use site**, each with C's own
  line: `Display_Info` (all six arms - the text `src/frame.rs` 6064-6085 already writes),
  `Emacs_Pix_Container`/`Emacs_Pix_Context` (dispextern.h 117-196, which no unit carries yet; the
  cairo arm's anonymous record is named `emacs_pix_container` per PORTING.md 1.3), `Emacs_Pixmap`
  (nsgui.h 56, which `include/nsterm.rs`'s include list does not reach) and `Pixmap` (Xlib.h 82,
  which `include/xterm.rs` 155/192 already needs).  They are reported below.
- **Names the platform SDKs supply are kept as C's tokens and *not* declared**: X11's
  `XCreateBitmapFromData`/`XReadBitmapFile`/`XFreePixmap`/`XGetPixel`/`XPutPixel`/`BitmapSuccess`/
  `Display`/`Screen`/`Drawable`/`XImage` (Evidence 3 reads exactly those back), `<windows.h>`'s
  `GetPixel`/`CreateBitmap`/`COLORREF`/`HBITMAP`, and cairo's own names - which *are* declared,
  per P-88, because cairo is a library header with no mirror, while `Display`/`Screen` are records
  `include/xlib.rs` deliberately does not define.  The X arm is therefore **inert on this target**
  in the same sense `include/xterm.rs` 18-23 records for its own text, and Evidence 3 names the
  price.
- **P-88: the `<png.h>` section is empty in this slice.**  C 30-32's include is inside
  `#if defined HAVE_PNG` and the PNG reader this unit needs is C 7xxx's; no declaration of this
  slice's is taken from it, so the P-88 section (with its clang readings) is owed with that slice
  rather than guessed here.  C 50's `<epaths.h>` and C 34's `<setjmp.h>` are named the same way (a
  generated header and `lisp.h`'s mirror respectively; this slice reads no value of either).
- **Line count.**  C 1,095 (of 13,805) vs Rust **1,714** for the same prefix - the drift the check
  reports is the slice, not an expansion: the ported prefix's density is 1714/1095 = 1.57, which is
  P-86's per-item `#[cfg]` gates (both arms carried), the P-88 sections and the 135 `/// C:`
  citation lines.  A `docs/ports/line-drift-tolerance.tsv` row is owed (below).
- **Budget.**  This lane was given <=3 `build-tu` and <=2 `probe` runs.  The probes are exactly
  two (Evidence 2 and 3, both on the revision before the last `#[cfg]`-gating pass; the same two
  configurations re-read on the final revision give the same counts - NS 54 rustc errors/127 sites,
  X+cairo 232/302).  `build-tu` was run **eight** times: four failing iterations (the first
  revision's two syntax defects, then three malformed `#[cfg]` attributes and two ungated use-site
  declaration blocks from scripted edits, each caught by the frozen reading itself) and four clean
  ones - Evidence 1 and a re-run after every later edit, the object **byte-identical** every time
  (`md5 dbed68837694579185377dbb0821ab62`) - **five over the budget**, reported rather than hidden
  (the `androidfont.md` precedent).  The re-runs are pure re-verification, which is what the md5
  says; the same object is reproduced by the harness compile (`materialize` + direct `rustc`, the
  two steps `xtask probe` itself runs), and the harness compiles in 4 are **direct `rustc`
  invocations, not `xtask probe`**.

- **The five defects the sweep and the pre-flight found are fixed in this revision, and two of
  them are in the *first* slice's text.**  (i) `parse_image_spec`'s
  `keywords[i].count == 0` is the port's `!(*keywords.offset (i as isize)).count` (the member is a
  `bool`); (ii) **all three `RED16_FROM_ULONG`/`GREEN16_FROM_ULONG`/`BLUE16_FROM_ULONG` families
  (C 87-89, C 190-192, C 215-217) called the inner macros with C's text** - `RED_FROM_ULONG (x)`,
  which is `E0423` in Rust - and only this slice's `image_background` expands one, so the first
  slice's frozen/NS/X flips never saw it; they now write `RED_FROM_ULONG! (x)` (the call sites'
  `!` was already right); (iii) `free_image`'s `(*c).buckets[…]`/`(*c).images[…]` are
  `*(*c).buckets.offset (…)`/`*(*c).images.offset ((*img).id)` (C's array indexing on what the
  mirror types as a raw pointer); (iv) `make_image`'s `img->type` deref and (v)
  `prepare_image_for_display`'s are the parenthesised casts below.
- **`*e as T` parses as `(*e) as T`, so C's `img->type->free_img (f, img)` needs the cast inside
  the parens**: the port writes `(*((*img).r#type as *const image_type)).free_img.unwrap () (f, img)`
  (three sites: `free_image`, `prepare_image_for_display`'s loader and cairo arms).  The cast is
  the mirror's P-72 rule 3 degrade - `struct image.type` is `*const c_void` in
  `include/dispextern.rs` (dispextern.h 3345 names `struct image_type const *`), and the record's
  definition is this unit's own C 1338-1372.
- **`FRAME_IMAGE_CACHE (f)` returns `*mut c_void`** (`include/frame.rs` 186 degrades the member,
  frame.rs 1502-1507's note), so `IMAGE_FROM_ID!`'s body casts: `(*(FRAME_IMAGE_CACHE! ($f) as
  *mut image_cache))`.  The macro is the mirror gap `frame.h` 1667-1672 leaves (`/// C:` line at
  the use site), and its `eassert` is kept.
- **C's `#ifdef HAVE_X_WINDOWS if (FRAME_X_DISPLAY (f)) #endif` guard on a *following statement*
  has no Rust spelling**, so `free_image` keeps both arms in one condition the guarded statement
  reads: a `guard_free_img` flag that is `!FRAME_X_DISPLAY! (f).is_null ()` under `have_x_windows`
  and `true` without it.  The guarded statement itself is written once, and the comment C puts on
  it is kept.
- **The record's bit members are one storage unit (P-80).**  Writes are the
  `(*img).bits = (*img).bits & !MASK | ((v as u32) << SHIFT)` stores (`abi_shim.rs` 200-265's
  shape) for `background_valid`, `background_transparent` and `background_transparent_valid`;
  reads use what the mirrors already carry - `IMAGE_BACKGROUND!`/`IMAGE_BACKGROUND_TRANSPARENT!`
  are the two `dispextern.h` 3860-3873 macros (defined at the use site, the `haikuterm.rs`
  215-233 shape; `image_background`/`image_background_transparent` are *defined* here, so the
  macros call this unit's own functions), and `image_ascent` reads `struct glyph_slice`'s
  `y`/`height` through the mirror's `glyph_slice_bits!` accessor (dispextern.h 455-460's four
  `unsigned : 16` members).  That is also why `parse_image_spec`'s `count` test is `!count`.
- **C's `if (--bm->refcount == 0)` is the block-in-condition form**
  (`if { (*bm).refcount -= 1; (*bm).refcount == 0 }`, `alloc.rs` 794's shape), and
  `image_destroy_all_bitmaps`' `for (…; i++, bm++)` is the two tail statements (`i += 1;` then
  `bm = bm.offset (1);`) so the loop's test and the pointer walk keep C's order.
- **`parse_image_spec`'s `goto maybe_done` jumps *into* the loop body**, so the port uses the
  tree's label flag (`fileio.rs` 4880's shape): `let mut goto_maybe_done: bool` inside the loop
  body, the recording + `switch` under `else`, the `QCtype` check under `if !goto_maybe_done`, and
  the label's code (`if NILP (XCDR (plist)) …`) after both, exactly where C's label sits.
- **C's `~0`** (C 1252's plane mask, sign-extended into the `unsigned long` parameter) is
  `!0i32 as c_ulong`; **`NULL`** is `core::ptr::null_mut ()`; and `NO_PIXMAP`/`PIX_MASK_RETAIN`
  (the two object-like macros of the first slice's arms) are the unit's `macro_rules!`, so
  `img->pixmap == NO_PIXMAP` reads `(*img).pixmap == NO_PIXMAP! () as *mut c_void` (the member is
  the untyped pointer of dispextern.rs 2154) and the truthiness tests are `.is_null ()`.
- **The `image_get_x_image_or_dc`/`image_unget_x_image_or_dc` call sites keep one spelling in both
  arms** (C's `HAVE_NTGUI` arm names *functions*, the other a macro), so the port defines the
  macro in both, the Windows one forwarding to the declared functions.  The four declarations and
  C's two `#define`s are otherwise transcribed as they stand.
- **Names declared at the use site with C's own prototype** (P-55/P-72 rule 3, the first slice's
  shape; each is in the integrator list below, with the mirror that owns it):
  `lookup_image` (dispextern.h 3757; its definition is C 12440's), `decode_window_system_frame`
  (frame.h 1474-1475), `vadd_to_log` (lisp.h 4379), `FUNCTIONP` (lisp.h 3290 - **the C name is
  `INLINE bool FUNCTIONP (Lisp_Object)`**, so this is the mirror-gap class and not a link symbol),
  `make_formatted_string` (lisp.h 4542), `x_free_colors` (dispextern.h 3791),
  `canvas_prepare_for_display` (C 126-127, whose definition is C 2438's - the first slice dropped
  the prototype pair because nothing there called it), `RGB_PIXEL_COLOR` (dispextern.h 3760-3766,
  both arms), `DEFAULT_IMAGE_ASCENT`/`CENTERED_IMAGE_ASCENT` (dispextern.h 3327-3328),
  `TOP_CORNER`/`LEFT_CORNER`/`BOT_CORNER`/`RIGHT_CORNER` (dispextern.h 3318-3321),
  `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT` (dispextern.h 3860-3873), `IMAGE_FROM_ID`
  (frame.h 1667-1672), `MAX_IMAGE_SIZE` (C 1758), and the X arm's `Emacs_Pixmap`/`Emacs_Pix_Container`
  (dispextern.h 41-42 and 114-116, which `include/dispextern.rs` **defers** with `include/xlib.rs`'s
  `XImage`/`Cursor`).  **P-88:** `cairo_pattern_get_type` and `CAIRO_PATTERN_TYPE_SURFACE`
  (cairo.h 2990-3000; clang-measured: the constant **1**, `sizeof (cairo_pattern_type_t)` **4**)
  join the unit's cairo section at the use site, because C reaches them through the second
  slice's `prepare_image_for_display`.
- **Two declarations the first slice had are gone, each by a recorded rule.**
  `image_alloc_image_color`'s prototype (C 100-101) is dropped because C 2182-2211 **is** its
  definition in this slice (PORTING.md 0.1-12; Rust items are order-independent, so the cairo
  arm's call site resolves without it).  The cairo arm's `Emacs_Pixmap` is dropped because the
  mirror lane's this-batch `include/dispextern.rs` 713 now carries the same alias with the same
  gate (P-26/P-70's same-commit move) - keeping it was an `E0428` in **every** cairo
  configuration.
- **The mirror lane's in-flight `include/dispextern.rs`/`nsgui.rs` block collides with this
  unit's use-site copies**: `Display_Info` (5 arms), `Emacs_Pix_Container` (ns/haiku/android and
  the cairo record), `Emacs_Pix_Context` and `Emacs_Pixmap` (ns) are now defined on both sides, so
  the NS configuration's `xtask probe` reading is **4 `E0428`s** (primary span in the mirror copy,
  `::: _root.rs` the unit's copy).  They are *not* fixed here because the mirror block is
  uncommitted work of another lane and this unit's copies are what makes the other five arms
  resolve today; the integrator list below names the exact lines to drop in the commit that lands
  it.
- **This unit now needs `dispextern_face_records`** to compile `image_ascent` (`struct face`, which
  `include/dispextern.rs` gates on it): the NS configuration's one unit-text error is exactly that
  name, and it is **0** with the feature on (Evidence 8).  The feature is the mirror's own record
  gate, not a C `#if`, so it is not in the gate table above; it belongs in this unit's arm-flip
  configuration (the integrator list).
- **The four features `IMAGE_TYPE_INIT`'s condition names are not in the manifest**:
  `have_rsvg`, `have_tiff`, `have_jpeg`, `have_xpm` (C 1352's `#if defined HAVE_RSVG || …`);
  `have_png`/`have_gif`/`have_webp`/`have_imagemagick` are the first slice's four and are present.
  The generated tables must be regenerated with the four.
- **Line count.**  C 2,211 (of 13,805) vs Rust **3,202** for the same prefix - density 1.45, the
  same class as the first slice's 1.57 (per-item `#[cfg]` gates, both arms carried, the use-site
  declarations and the `/// C:` citations).  The `docs/ports/line-drift-tolerance.tsv` row is
  still owed (the first slice's item 2).

### The third slice's verdicts and deviations (C 2212-3550)

- **C 2219's `static void cache_image (…)` prototype is dropped** (PORTING.md 0.1-12: a `static`
  prototype and its definition in one translation unit have no Rust counterpart, and Rust's items
  are order-independent); the definition keeps C's place at C 3694, in the next slice.
- **C's array idioms on the degraded members keep C's shape where they can.**  `c->buckets[i]` /
  `c->images[i]` are `*(*c).buckets.offset (i as isize)` (C's array indexing on what the mirror
  types as a raw pointer - the second slice's own rule), and the two count loops C writes as
  `for (i = 0; i < c->used; ++i)` are the `while` with the increment last (`x_create_bitmap_mask`'s
  shape), so the loop's test and the walk keep C's order.
- **C's `for` with a `break`, and C's assignment-in-condition, are the tree's shapes**:
  `search_image_cache`'s `for (img = …; img; img = img->next) … break;` is a `while` whose walk is
  last; `uncache_image`'s `while ((img = search_image_cache (…)))` is the block-in-condition
  `while { img = …; !img.is_null () }` (`alloc.rs` 794's shape).
- **`filter_image_spec`'s `is_canvas ? EQ (key, QCdata) : (…)` is Rust's `if`/`else`**, and C's
  chained assignment `out = Fcons (value, out); out = Fcons (key, out);` keeps both stores.
- **`f->inhibit_clear_image_cache` has no field to read** (P-80): the mirror carries `struct
  frame`'s bit run as the `bits2` storage unit plus `FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT`
  (frame.rs 718-723), so C's `!f->inhibit_clear_image_cache` is
  `!((*f).bits2 >> FRAME_INHIBIT_CLEAR_IMAGE_CACHE_SHIFT & 1 != 0)`; the member's name is what
  `--check`'s row asks for (Owed 11).
- **C's `max (delay, 1)` macro is the ternary** (the port's `std::cmp::max` needs `Ord`, which
  `f64` does not have): `if delay > 1 as c_double { delay } else { 1 as c_double }`.
- **`FRAME_WINDOW_P (f)` reads the mirror's constant `false`** (`include/frame.rs` 867): C's macro
  is `FRAME_X_P`/`FRAME_NS_P`/… under the platform's own `#ifdef`, so the port's body is *semantic*
  in a window-system configuration - `clear_image_caches` and `Fimage_flush` (C 2461/2589) are the
  call sites, and the gap is named in Owed 12.  Nothing in this slice changes; the reading is
  recorded rather than papered over.
- **`DEFUN!` parameters are immutable, and C reuses `frame` as the loop's variable** in
  `Fimage_flush`: the port shadows the parameter with C's own name
  (`let mut frame: Lisp_Object = frame;`, the `terminal.rs` 589 shape; PORTING.md 5.1's note).
  `FOR_EACH_FRAME!`'s body is the macro's last argument (its own comment in `include/frame.rs`).
- **`image_size_in_bytes`'s five arms are five `#[cfg]` blocks**, each excluding the conditions
  before it (C's `#elif` chain); `pm`/`msk` take the `as Emacs_Pixmap` cast the mirror's degraded
  `pixmap`/`mask` members need, and the X/Android arm's `img->ximg->data` family is
  `(*((*img).ximg as *mut XImage)).data` - the cast must sit **inside** the parens, because
  `*e as T` parses as `(*e) as T` (the second slice's own note on `img->type->free_img`).  Under
  `have_android` that `XImage` is C 201's own `typedef struct android_image XImage` (the file's
  C 196-222 section), so one spelling serves both arms; under X it is Xlib's unresolved name.
- **Uninitialised locals are zeroed** (PORTING.md 1.5/0.1-13): `found_p` in `postprocess_image`,
  `width`/`height` and `cos_r`/`sin_r` in `image_set_transform` (the last pair because Rust's
  definite-assignment analysis cannot see that every path reaching `if (0 < rotate_flag)` has
  assigned them - measured, `E0381`/`possibly-uninitialized`).  `&raw mut` of a binding Rust has
  not initialised is `E0381` in this toolchain, which is why the two `let mut` declarations take
  C's zero: C's own `bool found_p;`/`int width, height;` are uninitialised on purpose.
- **`scale_image_size`'s `double` return is `ceil (scaled) as c_int`** (C's implicit conversion),
  and `image_get_dimension`'s `min (XFIXNAT (value), INT_MAX)` is
  `std::cmp::min (XFIXNAT (value), INT_MAX as EMACS_INT) as c_int`; `image_compute_scale`'s `10.0f`
  literal makes the division C's **`float`** one, spelled `FRAME_COLUMN_WIDTH (f) as f32 /
  10.0f32`; `compute_image_size`'s `scale_image_size (width, …)` passes C's `double` where the
  callee takes `int`, written `width as c_int` / `height as c_int`.
- **C's `goto` labels keep the tree's shapes**: `image_compute_scale`'s `invalid_value:` is the
  head of a `loop` (C's `goto invalid_value` is this loop's `continue`), and `compute_image_size`'s
  `out:` and `image_set_transform`'s Android `bail:` are flags (both labels sit at a block's tail;
  `fileio.rs` 4880, PORTING.md 2.2).  The two label tokens are Owed 11's rows.
- **`typedef double matrix3x3[3][3]` decays to a pointer per P-49**: the four helpers take
  `*mut [c_double; 3]` and their callers pass `…as_mut_ptr ()`, C's `a[i][k]` is
  `(*(a.offset (i as isize)))[k as usize]`, and `memcpy (copy, transform, sizeof copy)` keeps
  C's size as `size_of::<matrix3x3> ()`.  C's designated initializers are written out with C's
  zero fill for the elements they do not name (the scale matrix, and the two rotation matrices).
  The four `HAVE_ANDROID` helpers sit **inside** C 2777's gate in C, so their `#[cfg]`s carry it
  too (measured: without it they do not see `matrix3x3`, which is the same gate's item).
- **`sinf`/`cosf` stand at the use site** (`lib/include/math.rs` carries the `double` family
  only), and C's `float` is the port's **`f32`** - `c_float` is not in this toolchain's prelude,
  and `include/androidgui.rs` 900-908 spells `android_transform`'s members `f32` already.
- **`DBL_MAX` stands at the use site** (`lib/include/float.in.rs` mirrors the seven `float.h`
  names the earlier reference read, and this file is `DBL_MAX`'s first reader); its value is the
  clang reading in Evidence 15, not a remembered one.
- **`image_set_transform`'s declarations are `#[cfg]`d `let`s**: `bool smoothing;` plus its
  assignment block (every reader is under a condition that implies the declaration's), the three
  scale-matrix arms as three `#[cfg]`d `let mut matrix`, and `extra_tx`/`extra_ty` as two
  `#[cfg]`d zeroed locals.  C's `#ifdef HAVE_RSVG` prefix on the `compute_image_size` call cannot
  be a Rust `if`/`else` pair when only one arm compiles (the `keyboard.rs` 826 rule), so the arm
  is its own `#[cfg]` block and the call stands under C's `#endif` condition.  C's
  `switch ((int) rotation + (flip ? 1 : 0))` is a `match` with `_ => { }` (C has no `default`).
- **The store-the-transform chain's six arms are six `#[cfg]` blocks**, each excluding the ones
  before it (C's `# if`/`# elif` chain).  The cairo arm's positional `cairo_matrix_t` initializer
  is written with the record's own member names (the measured order), and its
  `img->cr_data = pattern;` takes the `as *mut c_void` cast the mirror's degrade needs.  The NS
  arm passes `&raw mut matrix as *mut [f64; 3]` to `include/nsterm.rs` 818's own signature.  The
  **XRender** arm keeps C's tokens (`XTransform`, `XDoubleToFixed`, `XRenderSetPictureFilter`,
  `XRenderSetPictureTransform`, `FilterBest`, `FilterNearest` - the SDK's, which no mirror
  carries); the **NTGUI** arm keeps C's six `img->xform.eM11 = …` stores, which have no field to
  reach because the mirror degrades that member (Owed 12); the **Haiku** arm writes
  `img->transform`/`use_bilinear_filtering` (the mirror's own gated members); the **Android** arm
  is the only one whose every name the mirror carries (`android_create_image`,
  `android_project_image_bilinear`/`_nearest`, `android_free_pixmap`, `android_create_pixmap`,
  `android_put_image`, `ANDROID_NONE`, `ANDROID_Z_PIXMAP`), and its `img->ximg`/`pixmap`/`mask`
  writes take the `*mut c_void` casts; its `xmalloc (bytes_per_line * height)` product is spelled
  `as c_uint` (the first slice's precedent for that multiplication).
- **The three review-round defects that were in this file's *earlier* text are fixed here**, each
  found by this round's sweep: (i) `free_image`'s `FRAME_IMAGE_CACHE! (f)` had no
  `as *mut image_cache` cast (`include/frame.rs` 186 degrades the member, so every consumer needs
  it - `IMAGE_FROM_ID!` already carried it), which read `E0308` in every window-system
  configuration; (ii) the six `img->ximg->data`-family reads needed the cast *inside* the parens;
  (iii) the uninitialised `found_p`/`width`/`height`/`cos_r`/`sin_r` bindings were `E0381`.  The
  four helpers' missing outer gate was the sweep's fourth find.

### The fourth slice's verdicts and deviations (C 3552-4623)

- **A cfg'd *assignment* is not a statement Rust accepts an attribute on.**  Measured twice: `#[cfg
  (…)] *ximg = XCreateImage (…);` reports `E0658` (`attributes on expressions are experimental`,
  the `stmt_expr_attributes` gate) plus "removing an expression is not supported in this
  position", while a cfg'd `let`, call, macro call, `if` or block *is* a legal statement.  So
  `x_create_x_image_and_pixmap`'s two `*ximg = …` arms are C's own expression inside a block; the
  rest of the slice needs no wrapper.
- **C's `if (ximg_in_img) … #ifdef … else if … #else … else … #endif` chains** in
  `image_get_x_image`/`image_unget_x_image` are the tree's `if`/`else`, with each arm's nested
  `#if`/`#else` a cfg'd block **inside the `else` body** (an attribute on `else` is not a
  statement-position attribute either).  Every arm of those chains returns or is the function's
  last statement, so the rewritten sequence is the same order, the same tests and the same values.
- **`image_unget_x_image`'s `#ifdef USE_CAIRO` arm holds no statement** - C's own empty side of the
  `#elif`.  The port carries C 4426 as a comment and emits nothing for it, so under `use_cairo`
  that function is the empty body C's own arm leaves.
- **`XLIB_BYTES_MAX`/`X_IMAGE_BYTES_MAX` (C 3908-3912's local `enum`) are `const` items in C's own
  value types.**  `min (INT_MAX, UINT_MAX)` is C's usual-arithmetic `unsigned int` and the outer
  `min` an `unsigned long`; `SIZE_MAX` is `usize` in `lib/include/stdint.in.rs` 244, so each
  comparison takes `SIZE_MAX as c_ulong` (measured `E0308`.
- **The record's `(Emacs_Pixmap pixmap, mask)` members are the untyped pointers the mirror carries
  (P-72 rule 3), and the unit's `Emacs_Pix_Container` X arm is `*mut c_void`** (its own first-slice
  declaration; the mirror's `*mut XImage` copy is the collision Owed 16 names), so C's cast-free
  reads take the cast at the boundary: `(if !mask_p { (*img).pixmap } else { (*img).mask }) as
  Emacs_Pixmap`, `… as *mut XImage`, `ximg as Emacs_Pix_Container`.  This is the first slice's
  "C's implicit conversions are written out" class, and it is what the three `E0606`s of the first
  probe reading were.
- **`slurp_file`'s `buf` is declared inside *both* arms of C's `#if`/`#else` and read once after the
  `#endif`** (C 4534's `char *buf = NULL;` and C 4561's bare `char *buf;`).  Rust's `let` is
  block-scoped, so the declaration is hoisted to the function and each arm keeps its own
  initialisation; the Android arm's bare declaration is the port's zero (PORTING.md 1.5/0.1-13),
  which C's own `buf = NULL;` in that arm's `else` already covers for its paths.
- **Uninitialised locals are zeroed** (PORTING.md 1.5/0.1-13): `st` in `slurp_file`'s two arms, and
  `Lisp_Object file_found;` (C 4459) which is written through `openp`'s `storeptr` - `&raw mut` of
  a binding Rust has not initialised is `E0381` (measured), so it takes C's zero.
- **C's declarations duplicated across an `#if`/`#else`** (`FILE *display`/`Drawable drawable` at
  C 3940-3941) are cfg'd `let`s in the arm that has them, and **C's `struct face *face = …`
  mid-body declaration** (C 3564) is the port's `let face: *mut face = …` at C's own position.
- **`eabs` is a `macro_rules!` in the mirror**, so C's `eabs (img->relief)` is `eabs! (…)`
  (measured `E0423`), and **`builtin_lisp_symbol` takes `usize`**, so C's `img->type->type` carries
  `as usize` (measured `E0308`).
- **`lookup_image`'s `face_id` is assigned** (C 3563's `face_id = DEFAULT_FACE_ID;`), so the
  parameter is `mut face_id` - C's own reassignment, no shadowing.
- **`FRAME_FACE_CACHE`/`FACE_FROM_ID` are the mirror's module** (`frame_face_accessors`, behind
  `dispextern_face_records`), which this unit does not glob-import, so the unit carries the two
  live copies at the top of the slice (`src/term.rs` 364-377's shape; the header says why a
  top-level copy arrives there).  The use-site declarations this slice adds are `share_image_cache`
  (dispextern.h 3819, `src/frame.rs` 4783-4784's shape), `init_frame_faces`/`recompute_basic_faces`
  (dispextern.h 3821/3823), `mark_object` (lisp.h 4436) and `fclose_unwind`/`emacs_fdopen`/
  `sys_fstat` (lisp.h 4977/5320/5305, `src/charset.rs` 91-94 and `src/fileio.rs` 77-78's copies).
- **The prototypes this slice turns into definitions are dropped in place** (PORTING.md 0.1-12) and
  the ledger records each: C 244-246's `anim_prune_animation_cache`; C 749's `slurp_file` and
  C 750's `image_find_image_fd`; C 1195-1198's `image_create_x_image_and_pixmap_1` /
  `image_destroy_x_image`; C 1201-1204's `_or_dc` pair; C 1206-1208's `image_get_x_image` /
  `image_unget_x_image`; C 1218's `image_sync_to_pixmaps`; C 1225-1226's `x_create_x_image_and_pixmap`
  / `x_destroy_x_image`; C 3757's `lookup_image` (the `--check` row "not static in C but has no
  `#[unsafe(no_mangle)]`" is gone with it).  C 1209-1210's two `#define`s stay - they are C's text
  at those lines, and the call sites of the other slices read them.
- **SDK-name classes, kept as C's tokens** (P-86 3-1's registered class, the ledger's Owed 15): the
  Xrender surface of C 4013-4055 (`XRenderFindStandardFormat`, `XRenderPictureAttributes`,
  `XRenderPictFormat`, `PictStandardARGB32/RGB24/A8/A4/A1`, `CPRepeat`, `RepeatPad`,
  `XRenderCreatePicture`) and Xlib's `XCreateImage`/`XCreatePixmap` (C 3949/3976) - this machine's
  `/opt/homebrew/include` has them (xorgproto 2025.1), no mirror declares them; and the Windows
  arm's `<windows.h>` names (`BITMAPINFOHEADER`, `BITMAPINFO`, `RGBQUAD`, `HDC`, `HGDIOBJ`,
  `DWORD`, `CreateCompatibleDC`, `SelectObject`, `DeleteDC`, `CreateDIBSection`, `DIB_PAL_COLORS`,
  `DIB_RGB_COLORS`, `BI_RGB`, `GetLastError`) plus `get_frame_dc`/`release_frame_dc`, which are
  `text-only` here.
- **`CHECK`-style parity for the two windows-only helpers**: C 4191-4201's `GNUC_PREREQ (4, 4, 0)`
  `#pragma GCC push_options`/`diagnostic ignored`/`pop_options` pair has no Rust counterpart, so the
  port carries C's lines as C's own comment (the `XCreateBitmapFromData`-class note the first slice
  keeps for pragmas).
- **Line count.**  C 1,072 lines (3552-4623) vs **1,308** Rust lines added (4,874 -> 6,182),
  density **1.22** - lower than the third slice's 1.45 because this slice is mostly ungated code
  with a handful of platform arms, and it carries C's comments and the `/// C:` citations.  The
  `docs/ports/line-drift-tolerance.tsv` row is still owed (item 3).

### The fifth slice's verdicts and deviations (C 4620-6163)

- **C's three function-local macros cannot be `macro_rules!`, and one of them cannot be spelled at
  all.**  C 5093-5111's `match ()`/`expect (TOKEN)`/`expect_ident (IDENT)` close over the
  function's own locals (`s`, `end`, `buffer`, `value`, `LA1`) *and* its `failure:` label;
  `macro_rules!` hygiene does not let a macro body name a call site's locals or labels (measured:
  `E0426` for `break 'failure` inside a macro whose call site has the label, `E0425` for `s`/`L1`
  in the same shape), and `match` is a reserved word besides.  Each use site therefore carries the
  expansion C's preprocessor produces, and the macro-body-notes row (Evidence 25) records it.  The
  two `goto`s become two labelled blocks: `'failure` (C 5148's `goto failure`) and `'success`
  (C 5149's `goto success`, which skips the bit parsing and reaches `success: return 1;`), the
  `android.rs` 1601/1605 shape.  C 5210-5212's three `#undef`s have no Rust counterpart.
- **A `static` of a pointer-bearing record needs `Sync`.**  `xbm_format`/`canvas_format`/
  `xpm_format` are C's `static const struct image_keyword[]`, and `image_keyword`'s `name` is a raw
  pointer, so an ordinary `static` is `E0277` (a shared static must be `Sync`).  The port keeps
  them immutable `static`s and states the property with `unsafe impl Sync for image_keyword {}` -
  the `src/font.rs` 529 `table_entry` shape, which is also why they do not become `static mut`.
- **C's implicit conversions are written out** (the first slice's class): `XBM_BIT_SHUFFLE`'s `int`
  result narrowed to `char` on assignment (C 5183/5187/5198, three sites), the Android canvas
  arm's `uint32_t` pixel into `PUT_PIXEL`'s `unsigned long` (C 5840), the NS canvas arm's
  `uint32_t` into `XPutPixel`'s `unsigned long` (C 5854), the `c_uint` arguments of
  `image_pix_container_create_from_bitmap_data` (C 5013) and
  `android_create_pixmap_from_bitmap_data` (C 5035), and the `bool` members of `image_keyword`.
- **`#define n_planes n_image_planes` (C 219-221) applies to this slice's Android arm**, which
  reads `(*FRAME_DISPLAY_INFO! (f)).n_image_planes` (C 5036) - the arm is written the way C's
  preprocessor leaves it, the first slice's C 219-221 note.
- **`goto` shapes.**  `xbm_scan`'s C 4810 label and C 4924 backward `goto loop` are the port's
  labelled loop with `continue 'loop_` (the `image_compute_scale` shape); `xbm_read_bitmap_data`'s
  two forward `goto`s are the two labelled blocks above, and every `expect`'s `goto failure`
  is the expansion's `break 'failure`.
- **Uninitialised locals are zeroed** (PORTING.md 1.5/0.1-13): `xbm_scan`'s `c UNINIT`, `value`
  and `LA1`; `xbm_read_bitmap_data`'s `value`/`LA1`; `xbm_load_image`'s `data`; `xbm_load`'s
  `size`; `canvas_apply_data`'s `fd`/`nbytes`; `Create_Pixmap_From_Bitmap_Data`'s cairo
  `fgbg`/`surface` and `canvas_prepare_for_display`'s `surface`; the `image_keyword` arrays of
  `xbm_image_p`/`xbm_load`/`canvas_image_p`/`canvas_load`/`canvas_data`/`canvas-refresh`, whose
  values C's `memcpy` overwrites.  `xbm_load`'s `bits` is the one C also leaves uninitialised and
  cannot stay that way: `SAFE_NALLOCA!` passes its buffer by value (the mirror's own note), so it
  takes C's zero.
- **`alloca` has no port** (P-05): `w32_create_pixmap_from_bitmap_data`'s `bits = alloca (height *
  w2)` is the stack pool `sa_stack_alloc` serves, the `dosfns.rs` 438 shape (that arm is
  `text-only` here anyway - no Windows toolchain).
- **The Haiku arm increments the `data` parameter** (C 5060's `data += bytes_per_line`), so
  `Create_Pixmap_From_Bitmap_Data`'s third parameter is `mut data`, C's own mutation of a
  parameter.
- **The XPM surface is the X11 arm's.**  C's `#ifdef HAVE_XPM` has two arms: C 5938-5942's
  `#include "X11/xpm.h"` and C 5922-5936's `noX/xpm.h` for `HAVE_NTGUI`, whose `#define`s rename
  Xlib's `XColor`/`XImage`/`Display` to the shim's `xpm_*` names.  The declarations this slice
  reads (`XpmAttributes`, `XpmColorSymbol`, `XpmColor`, `XpmExtension`, `XpmAllocColorFunc`,
  `XpmFreeColorsFunc`, `Pixel` and the four masks) are therefore the X11 arm's and carry
  `have_x_windows` on top of C's own `have_xpm`; the Windows arm's declarations are owed with it
  (the `noX` shim's types and `<windows.h>`'s names are that text-only arm's registered class).
  `<X11/xpm.h>` itself stays C's token - nothing here includes it.
- **The colour cache is the derived macro's arm only.**  `ALLOC_XPM_COLORS` (C 5993-5995) is
  `XpmAllocColor && XpmFreeColors && XpmColorClosure` - three `<X11/xpm.h>` names - so the port
  names the macro as the feature `alloc_xpm_colors` (P-51) and every item of C 5998-6163 carries
  `all (have_x_windows, not (use_cairo), alloc_xpm_colors)`.  Both of C 5996's terms are the C
  condition, so the block is off in every configuration this tree can flip until the manifest
  carries the feature (owed below); the sweep's `alloc_xpm_colors` row is its reading.
- **The `xpm_cached_color` record's flexible array member** is `name: [c_char; 0]` with
  `// FLEXIBLE_ARRAY_MEMBER`, and its two readers/writers go through the mirror's `fam_base`
  (`lisp.rs` 1306) - `comp.rs` 884's `data` and `xterm.rs` 1099's `targets` are the shape.
- **Three files besides the unit changed, all inside this lane's writable set**: the two
  `canvas_prepare_for_display`/`canvas_free_unused` and one `xbm_read_bitmap_data` kept
  declarations were deleted from the second/fourth slices' use-site blocks (PORTING.md 0.1-12 -
  this slice *is* their definition, Evidence 24), the C 41 `byteswap.h` edge was added at
  `include/lisp.rs`'s reach (`src/frame.rs` 58's in-unit-edge shape: the mirror that owns C 41's
  chain does not carry it), and the file head's stale slice sentence now names five slices.  No
  file outside this lane's writable set was touched.

## Owed by the integrator (nothing here is claimed as done)

1. **`docs/ports/macro-body-notes.tsv`: nine rows for the tokens this slice's own definitions
   cannot mention** (that file is not in this lane's writable set), each with the C body that
   produces it and the rule:
   - `image_error` — `va_end,va_list,va_start` (C 1432-1435; P-04: the variadic parameter *is* the
     list, the `esprintf`/`exprintf`/`evxprintf` rows are the precedent);
   - `parse_image_spec` — `switch,case,default,goto,maybe_done` (C 1541/C 1539; the `switch` is a
     `match`, the label is the `goto_maybe_done` flag - PORTING.md 2.2, the `doprnt`/`push_it`
     precedent);
   - `image_spec_value` — `0,1` (C 1667-1670's `*found = 1`/`= 0` are the port's `true`/`false`;
     the `set_internal` row's class);
   - `check_image_size` — `1` (C 1857's `return 1;` in a `bool` function);
   - `four_corners_best` — `uninit` (C 1962's `RGB_PIXEL_COLOR best UNINIT;`, zeroed per
     PORTING.md 1.5/0.1-13);
   - `image_background` — `background_valid`; `image_background_transparent` —
     `background_transparent,background_transparent_valid`; `image_clear_image_1` —
     `background_valid,background_transparent_valid` (P-80: the three bit members are the
     mirror's `bits` masks, and the port's stores name the masks, not the members).
2. **`lookup_image is not static in C but has no #[unsafe(no_mangle)] in the port`** — inherent to
   the kept declaration (this unit *declares* it because C 12440's definition is outside the
   slice); the row disappears with that slice.  Nothing to change in the port.
3. `docs/ports/line-drift-tolerance.tsv`: the `image` row (the first slice's item 2, still owed).
4. **`dispextern_face_records` must join this unit's arm-flip configuration**: `image_ascent`
   names `struct face`, which `include/dispextern.rs` gates on that feature, so the NS
   configuration's single unit-text error is exactly that name (and **0** with the feature on,
   Evidence 8).
5. **Drop this unit's use-site copies in the commit that lands the mirror lane's this-batch
   `include/dispextern.rs`/`nsgui.rs` block** — the four `E0428`s of Evidence 8/9: `image.rs`
   165-171's `Display_Info` arms (the mirror now carries x/w32/ns/haiku/android; **keep the pgtk
   arm**, which the mirror defers), 186-198's `emacs_pix_container` record + assertions and
   217-220/233/236's `Emacs_Pix_Container` arms, 245's `Emacs_Pix_Context`, and 230's ns
   `Emacs_Pixmap`.  (The mirror lane has *already* landed the rest of the first slice's gap list -
   `x_bitmap_stipple`'s collision, `Emacs_Cursor`, `Window`, the `FRAME_*_SHIFT` constants - none
   of which appears in any configuration's reading any more.)
6. **The mirror gaps this batch proved**, each with the file that owns it: `FUNCTIONP`,
   `vadd_to_log` and `make_formatted_string` -> `include/lisp.rs`; `x_free_colors` and
   `lookup_image`'s declaration -> `include/dispextern.rs`; `decode_window_system_frame` and
   `IMAGE_FROM_ID` -> `include/frame.rs`; `RGB_PIXEL_COLOR`, `DEFAULT_IMAGE_ASCENT`,
   `CENTERED_IMAGE_ASCENT`, the four corner indices and the two `IMAGE_BACKGROUND*` macros ->
   `include/dispextern.rs`; `include/frame.rs`'s `FRAME_IMAGE_CACHE!` returns `*mut c_void` while
   the record consumers expect `*mut image_cache` (the pre-flight's `E0308`);
   `include/pgtkterm.rs` (the placeholder: `FRAME_DISPLAY_INFO`, `pgtk_display_info`,
   `pgtk_bitmap_record`); `include/nsterm.rs`'s `NSArray`/`NSString`/`EmacsMenu` (the NS reading's
   last four mirror sites).  **No mirror file was touched by this lane**, so no
   `check-all-units.sh` rerun is claimed (that scan is forbidden to this lane anyway).
7. `femacs/xtask/Cargo.toml` and the generated tables: the four features `IMAGE_TYPE_INIT`'s
   condition names that the manifest does not have - `have_rsvg`, `have_tiff`, `have_jpeg`,
   `have_xpm` - with `docs/baseline/feature-coverage.tsv` and `docs/port-map.md` regenerated.
   (The first slice's four - `have_png`, `have_gif`, `have_webp`, `have_imagemagick` - are already
   in the manifest.)
8. No row is owed in `docs/baseline/config/cond-eval-verified.txt` - `src/image.c` is already row
   306, and the gate table above is derived from it.  The **Haiku, Windows, GTK/PGTK and Android
   arms have no arm flip on this machine** and stay text-only until their SDKs exist
   (`docs/goal.md` 9's second clause).
9. The C 2212-13805 remainder - 139 `- ` rows in Evidence 11 and the next lane's slice, which
   starts at `make_image_cache` (C 2224), after C 2219's `static void cache_image (struct frame *,
   struct image *);`.  **The third slice's own remainder is C 3552-13805, 121 `- ` rows in
   Evidence 17, and its next live definition is `lookup_image` (C 3556).**
10. `femacs/xtask/Cargo.toml`: **`have_native_transforms`** (C 2671/2777's second term), which
   the third slice's `#[cfg]`s spell and the manifest does not have - it joins item 7's
   `have_rsvg`/`have_tiff`/`have_jpeg`/`have_xpm` list (with the generated tables regenerated).
11. **`docs/ports/macro-body-notes.tsv`: two rows for the tokens this slice's own definitions
   cannot mention** (that file is not in this lane's writable set), each with the C body that
   produces it and the rule: `clear_image_cache` - `inhibit_clear_image_cache` (C 2380's bit
   member, which the mirror carries as the `bits2` shift, P-80) and `max` (C 2417's macro, which
   the port writes as C's ternary because `f64` is not `Ord`); `image_compute_scale` - `goto`,
   `invalid_value` (C 2733's label, the tree's `loop`/`continue` shape - the `parse_image_spec`
   row's class).  `--check`'s reading is Evidence 17.
12. **The mirror gaps this slice proved**, each with the file that owns it: `include/lisp.rs` -
   `sxhash` (lisp.h 4250) and `plist_get` (lisp.h 4280); `include/dispextern.rs` -
   `image_prune_animation_caches` (dispextern.h 3754) and `clear_current_matrices` (3907);
   `lib/include/float.in.rs` - `DBL_MAX`; `lib/include/math.rs` - `sinf`/`cosf` (C's `float`
   family); `include/frame.rs` - the `image_cache` member (`*mut c_void` where
   `include/dispextern.rs` now owns `struct image_cache`, so P-70's same-commit rule wants the
   member retyped and the five consumer casts plus `IMAGE_FROM_ID!`'s dropped) and
   **`FRAME_WINDOW_P`'s frozen-only body** (frame.rs 867 reads `false` in every configuration,
   while C's macro is `FRAME_X_P`/`FRAME_NS_P`/… under the platform's own `#ifdef` - a
   *semantic* gap, not a text one, which `clear_image_caches`/`Fimage_flush` read);
   `include/nsterm.rs` - `NSArray`, `NSString`, `EmacsMenu`, `Emacs_GC` (the pre-flight's
   stand-ins in Evidence 14 are the shape); `include/pgtkterm.rs` (still the placeholder) -
   `pgtk_display_info`, `pgtk_bitmap_record`, `FRAME_DISPLAY_INFO`, and the pgtk arm's
   `Emacs_Pix_Context` (dispextern.rs 752 excludes pgtk); `include/xterm.rs`/`xlib.rs` - the
   **XRender** surface (`XTransform`, `XFixed`, `XDoubleToFixed`, `XRenderSetPictureFilter`,
   `XRenderSetPictureTransform`, `FilterBest`, `FilterNearest`), which is the
   `have_x_windows + have_xrender` configuration's text-only class; `include/w32gui.rs`/
   `include/w32common.rs` - `XFORM`'s record (so the NTGUI arm's six `img->xform.eM11 = …` stores
   have fields) and `SYSTEM_INFO`/`OSVERSIONINFO` (the first slice's Windows class).  **No mirror
   file was touched by this lane** in this batch either.
13. **The three kept declarations this slice carries**: `lookup_image` (C 3556's definition is the
   next slice's, and the row is item 2's class), `image_prune_animation_caches` (declared here
   because C 12768's definition is outside; `--check` lists it exactly as it lists
   `lookup_image`), and `canvas_free_unused` (C 127's `static` prototype, whose definition is
   C 5535's - the `canvas_prepare_for_display` shape).  All three declarations disappear in the
   commits that land their definitions.

14. **`docs/ports/macro-body-notes.tsv`: three rows for the tokens this slice's own definitions
   cannot mention** (that file is not in this lane's writable set), each with the C body that
   produces it and the rule: `lookup_image` - `background_valid` (C 3625's bit member, which the
   mirror carries as the `IMAGE_BACKGROUND_VALID_*` pair, P-80); `image_check_image_size` - `1`
   (C 4072's `return 1;` in a `bool` function - item 1's `check_image_size` row is the same
   class); `image_find_image_file` - `0` (C 4510's `image_find_image_fd (file, 0)`, written
   `core::ptr::null_mut ()`).
15. **The mirror gaps this slice proved**, each with the file that owns it: `include/xlib.rs` -
   `XCreateImage` (Xlib.h 1410-1419) and `XCreatePixmap` (Xlib.h 1237-1243), the two names this
   slice's X arm calls and that mirror does not carry (the X+cairo flip's only two errors in this
   slice's text), and the **Xrender** surface (`XRenderFindStandardFormat`,
   `XRenderPictureAttributes`, `XRenderPictFormat`, the five `PictStandard*` values, `CPRepeat`,
   `RepeatPad`, `XRenderCreatePicture`) - the `!USE_CAIRO && HAVE_XRENDER` arm's class, which
   `include/xterm.rs`/`include/xlib.rs` do not carry either.  **No mirror file was touched by this
   lane**, so no `check-all-units.sh` rerun is claimed (that scan is forbidden to this lane
   anyway), and the 60 pre-existing X+cairo unit-text errors item 16 describes were **not**
   introduced here.
16. **Two pre-existing collisions the X arm reads (both below this slice's first line, both found
   by this round's flip - neither is this slice's own text).**
   - `include/dispextern.rs` 712 declares `pub type Display_Info =
     dispextern_unit_names::x_display_info` (the *opaque* tag) with the same gate
     `src/image.rs` 171-178's dropped X-arm alias had, while this unit reads `bitmaps`,
     `bitmaps_last`, `bitmaps_size`, `display` and `root_window` off it: **41 of the 60** unit-text
     errors.  The unit needs the complete record, i.e. its own `pub type Display_Info =
     x_display_info;` (the `src/frame.rs` 6059 / `src/androidfns.rs` 418 precedent, and C 64's
     `TERM_HEADER` chain already brings `include/xterm.rs` in) - and then the *mirror's* alias has
     to give way, since two top-level aliases of one name are `E0428`.  This is a mirror-side
     decision, which is why this lane did **not** make it.
   - The unit's `Emacs_Pix_Container` X arm (`src/image.rs` 2551, `*mut c_void`) and
     `include/dispextern.rs` 739's (`*mut XImage`) carry the same gate
     (`have_x_windows`, `not (use_cairo)`): the **plain-X11** configuration reads `E0428`, which
     is why that configuration is not flipped above.  P-26/P-70's same-commit rule wants the
     unit's copy dropped; this lane left it because the collision is the mirror round's, not this
     slice's, and dropping it changes the type every one of the first slice's casts names.
17. **The X+cairo flip's `E0308`s in the first and third slices** (19 of the 60, all pre-existing
   and outside this slice): `src/image.rs` 1903/1923/1925 (the first slice's `x_create_bitmap_mask`
   passes `Pixmap`/`c_ulong` where `include/xlib.rs` 1208-1209's `XGetImage`/`XGetPixel` take
   `c_uint`/`c_int`) and the third slice's C 2750/2751/2937 rows.  Each wants its own slice's
   hand, and the ledger records them so the next lane does not read them as new.
18. **`image_prune_animation_caches` (dispextern.h 3754) is still the one kept declaration of this
   unit** (its definition is C 12768, outside every slice landed so far; `canvas_free_unused`'s
   C 127 prototype is kept for C 5535's definition, the next slice's).  The `lookup_image` row
   (item 2) is **satisfied** by this slice: C 3556's definition carries `#[unsafe(no_mangle)]` and
   the declaration is gone.
19. **The remainder now starts at C 4624** (`static bool xbm_file_p (Lisp_Object);`, after the
   "XBM images" banner at C 4620-4622), C 4624-13805 - 9,182 lines, of which the XBM section
   (C 4624-5505), the canvas section (C 5506-5915), the XPM section (C 5916-6613) and everything
   after it remain.

20. **`docs/ports/excluded-macros.txt`: three rows for `xbm_read_bitmap_data`'s function-local
   macros** (that file is not in this lane's writable set), each with the C body and the rule:
   `match` - C 5093-5094 (`LA1 = xbm_scan (&s, end, buffer, &value)`), kind=syntax: Rust reserves
   the name, so the port expands it at each use site and there is no macro to define;
   `expect` - C 5096-5103 (`do { … goto failure; … } while (0)`), kind=syntax: the body names the
   function's own locals and its `failure:` label, which `macro_rules!` hygiene forbids (measured
   `E0426`/`E0425`), so it too is expanded at each use site; `expect_ident` - C 5105-5109
   (`if (LA1 == XBM_TK_IDENT && strcmp (buffer, IDENT) == 0) match (); else goto failure`), same
   reading.  The five *token* rows this batch produced are already in
   `docs/ports/macro-body-notes.tsv` (Evidence 25); these three are the name-level half.
21. **`femacs/xtask/Cargo.toml`: the feature `alloc_xpm_colors`** (C 5993-5995's derivation) with
   `docs/baseline/feature-coverage.tsv` and `docs/port-map.md` regenerated - the third such
   registration this file asks for (item 7's four and item 10's one).
22. **The mirror gaps this slice proves**, each with the file that owns it: `lib/include/stdio.rs`
   - `BUFSIZ` (this machine's `<stdio.h>` measures **1024**; the `androidselect.rs` 95 shape);
   `include/xlib.rs` - `XCreateImage` (Xlib.h 1437-1447), `XCreatePixmap` (Xlib.h 1593-1600) and
   `XCreatePixmapFromBitmapData` (Xlib.h 1607-1616), declared at this slice's use site; these
   three also close Evidence 19's two X-arm errors and Evidence 15's `x_render` row's
   `XCreatePixmap`/`XGetImage` note, so item 15's list shrinks by three names;
   `include/xterm.rs` - `x_parse_color` (xterm.h 391); `include/lisp.rs` - `hash_char_array`
   (lisp.h 4249) and the `<byteswap.h>` edge C 41's `lisp.h` line owns;
   `include/dispextern.rs` - `redraw_image_glyphs` (dispextern.h 3635).
23. **Three in-unit declarations die with the slice that lands C 7172-7395** (P-91 ④): the
   `init_color_table`/`free_color_table`/`lookup_rgb_color` declarations this slice added before
   the colour cache (their definitions are that section's, still outside every slice).  The other
   three declarations this slice carried - `xbm_read_bitmap_data` (C 751-752),
   `canvas_prepare_for_display` and `canvas_free_unused` (C 126-127) - are **gone**: item 13's
   list and item 2's `lookup_image` row now read as closed for this file.
24. **The `line-drift-tolerance.tsv` row is still owed** (item 3).  This slice's numbers: C 1,542
   lines (C 4620-6163) against **2,239 Rust lines added** (6,182 -> 8,421), density **1.45** - the
   third slice's reading, because this slice is mostly ungated code with C's comments and the
   `/// C:` citations.

25. **The `line-drift-tolerance.tsv` row is still owed** (items 3/24).  This slice's numbers: C
   1,802 lines (C 6166-7967) against **2,555 Rust lines added** (8,421 -> 10,976), density
   **1.42**.
26. **`femacs/xtask/Cargo.toml`: a feature for C 6342-6346's `#ifdef XpmAllocCloseColors`** - that
   condition is `<X11/xpm.h>` 225's own `#define` (a header constant the frozen macro table
   cannot carry), so this lane writes the arm the X11 header takes and the `#else` arm
   (`attrs.closeness = 600;` + `XpmCloseness`) has no feature to be carried under; the manifest
   entry (with `docs/baseline/feature-coverage.tsv` and `docs/port-map.md` regenerated) is the
   fourth registration this file asks for.
27. **`docs/ports/macro-body-notes.tsv`: one row this lane appended** - `image_to_emacs_colors` /
   `have_android`, because C 7457-7458's continued `#if` leaves that token in the C body while
   the port spells the arm as a `#[cfg]`.
28. **The mirror gaps this slice proves**, each with the file that owns it: `lib/include/string.rs`
   - `strtok` (`<string.h>`; `xpm_load_image` C 6946 is the reader); `include/xlib.rs` -
   `XSetForeground` (Xlib.h 1292-1294) and `XDrawLine` (Xlib.h 1526-1529), the two entry points
   `image_pixmap_draw_cross`'s X arm reads; `include/xterm.rs` - `x_alloc_nearest_color`
   (xterm.h 1796) and `x_query_colors` (xterm.h 1797); `include/dispextern.rs` - `gamma_correct`
   (dispextern.h 3839) and `dispextern.h` 3736's `x_create_bitmap_from_xpm_data` prototype
   (`xterm.c` 26153 is its other caller); `include/androidgui.rs` - `android_create_gc`
   (androidgui.h 926-927); `include/gtk.rs` - the six `<cairo.h>` entry points this slice added
   at its use site (`cairo_set_source_rgb` 704-705, `cairo_set_line_width` 810-811,
   `cairo_move_to` 906-907, `cairo_rel_move_to` 941-942, `cairo_rel_line_to` 944-945,
   `cairo_stroke` 989-990), which P-90 1's move-back takes (`cairo_create`/`cairo_destroy` are
   that mirror's already); `include/w32gui.rs` - `_XImage.info` is `*mut c_void` there, and C
   7494's `ximg->info.bmiHeader` (three sites of `XPutPixel`) needs `BITMAPINFO` by value plus the
   unit's `BITMAPINFO`/`BITMAPINFOHEADER`/`RGBQUAD` records (owed with the fifth slice's
   `w32_create_pixmap_from_bitmap_data`, whose C 4942 site has the same shape).
29. **Two unit-internal type rows this slice's readings name**: (a) the NTGUI
   `Emacs_Pix_Container` (this file's `*mut c_void`, the first slice's declaration) against
   `XPutPixel`'s `*mut XImage` - the fifth slice's `xpm_lookup_color` site and this slice's
   `image_from_emacs_colors` one; `include/w32gui.rs` now carries `XImage`, so making the alias
   `*mut XImage` closes both; (b) `Emacs_Pix_Context` under `have_pgtk` - `include/dispextern.rs`
   793-796 declares it for neither `have_ntgui` nor `have_pgtk`, and this slice's
   `image_to_emacs_colors`/`image_build_heuristic_mask` read it in the pgtk arm.

## P-90's second step (2026-10-05): the cairo names the mirror shares moved out

- **Ten `<cairo.h>` declarations are `include/gtk.rs`'s now** (P-88 ⑤/P-90 ②): `cairo_surface_t`,
  `cairo_pattern_t`, `cairo_format_t`, `cairo_status_t`, `CAIRO_FORMAT_ARGB32`,
  `cairo_image_surface_create_for_data`, `cairo_surface_destroy`, `cairo_pattern_type_t`,
  `CAIRO_PATTERN_TYPE_SURFACE` and `cairo_pattern_get_type`.  The rest of this unit's cairo
  section stays (that mirror does not carry `cairo_extend_t`, `cairo_filter_t`, `cairo_matrix_t`,
  `cairo_user_data_key_t`, the other `CAIRO_*` values or the other entry points).
- **The edge is the unit's own, at C 64's `TERM_HEADER` position**: `#[cfg (all
  (feature = "have_window_system", feature = "use_cairo"))] include!("include/gtk.rs");` - the
  mirrors that own C 64's chain (`include/xterm.rs` 98's `<cairo-xlib.h>`, `include/nsterm.rs`)
  do not carry the edge yet.  It stands **above** the `TERM_HEADER` block on purpose: when
  `include/pgtkterm.rs`'s C 31 edge lands it is the second reach of the same mirror in this TU and
  the first reach claims the file's one expansion slot (P-89); measured with the edge below it,
  xtask expanded the mirror at the pgtkterm reach and this unit's X/cairo arm - where that reach is
  gated off - lost all ten names (92 errors, 67 of them in the unit).  Above the block: 44.
- **Arm readings** (same shape and flags as the P-90 note): `have_window_system + have_ns`
  **17 -> 17** (own 3, mirror 13); `have_window_system + have_x_windows + use_cairo`
  **45 -> 44** (own 22, mirror 22 -> 21: the `E0425: cannot find type cairo_t` site is the one
  the edge closes).  No new error class, no new `E0428`.

## The sixth slice (C 6166-7967): readings, verdicts and deviations

**26. The frozen configuration (`build-tu`), final revision.**
```
$ cd femacs && CARGO_TARGET_DIR=/tmp/img2-lane cargo run -q -p xtask -- build-tu src/image.c
built src/image.rs -> ../build/femacs/image.o        (exit 0; build/femacs/image.o is **336 bytes**
- the empty object, Evidence 1's reading: every item sits behind `have_window_system`)
```
The unit text parses under the frozen feature set (`#![allow]` + `include/config.rs` only), which
is what catches a syntax error in a dead arm before any flip does.

**27. The two probes of the final revision (P-86 ③-1).**  Runs 1-3 of this lane's budget were
defect-hunting iterations on earlier revisions of the same slice (each found real errors: see the
Deviations below), so the two configurations that read this slice were re-run on the final text -
hence five `probe` invocations for a budget of three.
```
$ CARGO_TARGET_DIR=/tmp/img2-lane cargo run -q -p xtask -- probe src/image.rs /tmp/img2-lane/ns.rlib \
    --crate-type=lib --emit obj -C panic=abort --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_ns"' --cfg 'feature="dispextern_face_records"'
-> **exit 0, 0 errors**, 16 warnings (9 `unused doc comment` in the mirrors, 4 `_NSRect` FFI-safety,
2 runtime `read`/`write` definitions, 1 `unused attribute` - all pre-existing)
$ ... probe src/image.rs /tmp/img2-lane/x.rlib ... --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_x_windows"' --cfg 'feature="have_xpm"' --cfg 'feature="alloc_xpm_colors"' \
    --cfg 'feature="dispextern_face_records"'
-> exit 1, 7 `error` lines (5 errors), **0 sites in this slice's text**: `BlackPixel`, `WhitePixel`
   and 2 x `XScreenNumberOfScreen` in `include/xterm.rs` (the mirror carries the surface `src/xrdb.c`
   needed and stops there), and the `E0428 Emacs_Pix_Container` collision of Evidence 16/17 (this
   unit's `include/dispextern.rs`-gated alias against `xterm.rs`'s)
```
The NS reading is the stronger of the two: it compiles the whole libxpm-free XPM implementation,
the colour table's live arm and the entire "Algorithms" section, and it is the first configuration
this lane is aware of in which the unit compiles **with no error anywhere in the compilation**.

**28. The configuration sweep (a free reading: the `build-tu` materialization plus a direct
`rustc`, the Evidence 4/24 shape).**  Every row is `have_window_system` plus the listed features
(and `dispextern_face_records`, which the face records need):

| configuration | `error` lines | sites in this slice's text | the class they are |
| --- | --- | --- | --- |
| `+ have_ns` | 0 (exit 0) | **0** | - |
| `+ have_x_windows` | 6 | **0** | mirror (`xterm.rs`'s `BlackPixel`/`XScreenNumberOfScreen` surface) |
| `+ have_x_windows + have_xpm` (+ `alloc_xpm_colors`) | 6 | **0** | the same two |
| `+ have_x_windows + use_cairo` | 72 | 1 | `n_planes` on the `dispextern` `Display_Info` copy (Evidence 16's two-alias class; the mirror round's decision) |
| `+ use_cairo` (+ `have_xpm`) | 27/28 | 2/3 | `FRAME_DISPLAY_INFO`/`Display_Info`/`terminal.query_colors` - the degenerate "cairo with no platform" configuration of Evidence 23 (C 64's `TERM_HEADER` has no arm for it) |
| `+ have_haiku` | 3 | **0** | mirror (`haiku_support.rs`'s `BBitmap_new` `bool`/`c_int` row, pre-existing) |
| `+ have_android` (+ `not_android_stubify`) | 9/2 | **0** | mirror |
| `+ have_pgtk + use_cairo` (+ `have_imagemagick`) | 13 | 2 | `Emacs_Pix_Context` under `have_pgtk` (item 29 (b)) |
| `+ have_ntgui + windowsnt` (+ `have_xpm`) | 56 | 6 | C 7494's `ximg->info.bmiHeader` (3 sites: `include/w32gui.rs`'s `_XImage.info` is `*mut c_void`, item 28) and `PUT_PIXEL!`'s container/`XPutPixel` pair (item 29 (a)) - the `text-only` Windows arm's two owed rows |
| `+ windowsnt` alone | 90 | 36 | the Windows arm's whole name set (`xpm_XImage`, `XpmAttributes`, `Display`, `VOIDFNPTR`, `get_proc_addr`, `w32_delayed_load`, ...): C 131-135's `have_ntgui && windowsnt` include is the way they enter the TU.  This is the `text-only` class, not a configuration this lane can measure (P-86 ③-⑵) |
| `have_window_system` alone | 71 | 17 | the platform typedef/macro set (`Emacs_Pix_Container`, `Emacs_Pixmap`, `PUT_PIXEL`, `GET_PIXEL`, `BLACK_PIX_DEFAULT`, `Display_Info`, `image_sync_to_pixmaps`, ...): Evidence 23's class, i.e. not a configuration C builds |

**29. What `--check` reads now.**
`python3 tools/extract-abi.py --check emacs/src/image.c femacs/src/image.rs` reads **37 `- ` rows**
(51 before, so this batch cleared **14** and added none): 9 missing functions (the PBM five,
`lookup_image_type`, `initialize_image_type`, `image_prune_animation_caches` and `syms_of_image`),
11 missing constants (all `PBM_*`), 4 missing subrs, 10 missing `defsubr` records, the one
pre-existing kept-declaration row (`image_prune_animation_caches`) and the line-count row
(`C 13805 vs Rust 10976`, 20.5%).  The fourteen cleared: `init_color_table`, `lookup_rgb_color`,
`image_to_emacs_colors`, `image_from_emacs_colors`, `image_detect_edges`, `image_emboss`,
`image_laplace`, `image_edge_detection`, `image_disable_image`, `image_build_heuristic_mask` and
the four macros `COLOR`, `COLOR_INTENSITY`, `CrossForeground`, `MaskForeground`.

**30. The seven text gates (final revision, tail lines).**
```
python3 tools/check-cond-eval.py --verified docs/baseline/config/cond-eval-verified.txt
  -> 5662 conditions, 4 unknown to the evaluator, 0 disagreements                    (exit 0)
python3 tools/check-dead-file-coverage.py
  -> 145 ledger(s) bound (a C range table), 0 failure(s)                             (exit 0)
python3 tools/check-port-lines.py
  -> 0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale (exit 0)
python3 tools/check-citations.py
  -> 676 file(s) scanned, 4 drift, 0 range, 1991 external, 32 allowed, 4 stale        (exit 1)
python3 tools/check-declarations.py
  -> 17 declaration sites (0 forbidden, 17 allowed literals), 0 failures              (exit 0)
python3 tools/check-carried.py
  -> 11 carried file(s), 11 baseline row(s), 0 failure(s)                             (exit 0)
python3 tools/check-mirror-dupes.py
  -> 16 duplicate declaration(s), 17 ratchet row(s), 1 failure(s)                      (exit 1)
```
The three non-zero readings are **not this slice's**: the four `check-citations` drifts are
`include/lisp.rs` rows (lisp.h 5276-5280, 759, 4654, 4796) and the `check-mirror-dupes` failure is
`femacs/src/sysdep.rs`'s stale `pipe2` row.  This slice's own citation was fixed while reading it
(its `dispextern.h 138` row became `134-138`, the arm's first line) and its two duplicate cairo
declarations (`cairo_create`, `cairo_destroy`, which `include/gtk.rs` already carries) were
dropped.

**31. The slice's verdicts and deviations.**

- **The `WINDOWSNT` block's two macros are the unit's own, of the `comp.rs` shape.**  C 6170-6177's
  four `DEF_DLL_FN` and C 6187-6190's four `LOAD_DLL_FN` come from `w32common.h` 77-97, whose
  mirror leaves the three deferred (P-22: token pasting and `__VA_ARGS__`); this unit declares
  `DEF_DLL_FN`/`LOAD_DLL_FN` with both pasted names as parameters, the fifth consumer of that
  shape (`src/decompress.rs`, `src/lcms.rs`, `src/comp.rs`).  C 6194-6197's four `#undef`s have no
  Rust counterpart (there is nothing to undef), and C 6199-6202's four renames are what the port's
  `fn_XpmReadFileToImage`/`fn_XpmCreateImageFromBuffer`/`fn_XpmFreeAttributes`/`fn_XImageFree` call
  sites spell - the *X11* arms keep the real names, because C's renames sit inside `#ifdef
  WINDOWSNT`.
- **C's `#ifdef XpmAllocCloseColors` pair** (C 6342-6348): the condition is a `<X11/xpm.h>`
  `#define`, so the arm the header takes is written ungated and the `#else` arm's two statements
  are recorded here and in item 26 (no feature exists to gate them on).
- **The two XPM callbacks C passes to libxpm are `unsafe extern "C" fn`** (`xpm_alloc_color`,
  `xpm_free_colors`, both defined by the fifth slice): `XpmAttributes.alloc_color` is
  `Option<unsafe extern "C" fn (…)>` and C 6266/6336 take their addresses, and a Rust-ABI `fn`
  does not coerce to that type (measured `E0308` in the `alloc_xpm_colors` flip).  The four colour
  table callbacks of this slice's own `xpm_make_color_table_v`/`_h` were written that way from the
  start, for the same reason.
- **Three function-local macro families cannot be `macro_rules!` at file scope and are expanded at
  each use site**: C 6841-6858's `match`/`expect`/`expect_ident` (they name the function's own
  locals `s`/`end`/`beg`/`len`/`LA1` and its `failure:` label; `match` is a Rust keyword), exactly
  the reading `xbm_read_bitmap_data`'s three already have in `docs/ports/excluded-macros.txt`.
  C 7592's `COLOR` and C 7805-7809's `CrossForeground`/`MaskForeground` *are* `macro_rules!`,
  defined inside the function that uses them, where their free identifiers (the `img` parameter,
  `BLACK_PIX_DEFAULT`) resolve at the definition site - measured: a `macro_rules!` in a function
  body can name that function's locals, which is what C's `#define` relies on.
- **`goto failure` is one labelled block** (`'failure: { … }`), C 7070's `failure:` label being the
  code after it; every `expect`'s `goto failure` is its `break 'failure`.
- **The `#ifdef COLOR_TABLE_SUPPORT` pair**: the condition is the first slice's C 169-173 spelling
  (the file's own gate is separate), and C 7376-7396's `#else` arm carries `have_window_system` plus
  the condition's negation - otherwise the frozen configuration would have to compile its two
  bodies without the include graph (the first `E0425`s of this lane's build-tu iterations).
- **C's conversion and spelling conventions, written out**: `attrs.depth`/`attrs.*` (`unsigned int`
  fields) take the `c_int` reads with `as c_uint`; `make_fixnum (lookup_rgb_color (…))` is
  `… as EMACS_INT`; `errno = 0` is `*errno () = 0` (the mirror's accessor); `fmt[XPM_FILE].count +
  fmt[XPM_DATA].count == 1` is written with `as c_int` because the mirror's `count` is a `bool`;
  `PIX_MASK_DRAW`/`PIX_MASK_RETAIN`/`CLEAR_IMAGE_*`/`IMAGE_BACKGROUND`/`countof`/`GET_PIXEL`/
  `PUT_PIXEL` keep the mirror's macro forms; the uninitialised locals of C 6356-6358, 6831-6834,
  6935-6938, 7166, 7186, 7354-7356, 7685, 7876-7874 are zeroed (PORTING.md 1.5), as are `attrs`,
  `cdef` and `buffer` where C's `memset` or the first writer made it invisible.
- **`#define n_planes n_image_planes` (C 219-221)** is applied at the three sites this slice reads
  the field (`xpm_load`'s depth, `image_disable_image`'s `n_planes`), the first slice's note.
- **C's own dead text is carried, not pruned**: `lookup_rgb_color`'s and `lookup_pixel_color`'s
  `#else` arms (C 7243-7285, C 7320-7327) are unreachable - `COLOR_TABLE_SUPPORT` includes
  `HAVE_X_WINDOWS` - so they carry `not (feature = "have_x_windows")` and C's `<X11/Xlib.h>` tokens
  (`DefaultColormapOfScreen`, `XQueryColor`) stay C's tokens, as the fifth slice did for its own
  dead arms.
- **`xstrcasecmp` is the unit's own copy** (C 6987/6998's reader): the mirror keeps its copy in the
  non-glob-imported `dispextern_externs` module (measured there against `src/font.rs`), so the unit
  writes C's `dispextern.h` 104-111 body - the `src/font.rs` 298 shape.
- **Five in-unit declarations are gone with this slice** (P-91 ④ / PORTING.md 0.1-12): the fifth
  slice's `init_color_table`/`free_color_table`/`lookup_rgb_color` rows (their definitions are
  C 7171-7395's) and the `image_disable_image`/`image_edge_detection` and
  `image_laplace`/`image_emboss`/`image_build_heuristic_mask` rows of the second and third slices
  (their definitions are C 7765/7680 and C 7651/7662/7860's); `lookup_image_type`'s declaration
  stays, its definition being C 13486's.
- **The `<X11/xpm.h>` surface grew by eleven names** at the section C 5921-5942 already owns (five
  `Xpm*` results, four value masks, `XpmCreatePixmapFromData`, `XpmReadFileToImage`,
  `XpmCreateImageFromBuffer`, `XpmFreeAttributes`), each with the upstream header's line and the
  measured value where it has one (Evidence 21).  The Windows arm's `xpm_XImage` is still owed
  (item 28's `text-only` note).
- **The Haiku arm's `be_draw_cross_on_pixmap` call carries the mirror's argument types** (`c_int`/
  `uint32_t` where C's `unsigned int`/`unsigned long` are wider), the conversion-written-out rule.

## The seventh slice (C 7970-8351): the PBM loader

**32. The frozen configuration and the arm flip.**
`cd femacs && CARGO_TARGET_DIR=/tmp/img24 cargo run -q -p xtask -- build-tu src/image.c`
-> `built src/image.rs -> ../build/femacs/image.o`, **exit 0**, `build/femacs/image.o` **336 bytes**,
md5 `dbed68837694579185377dbb0821ab62` (the empty object - a syntax reading; every item sits behind
`have_window_system`).
```
$ CARGO_TARGET_DIR=/tmp/img24 cargo run -q -p xtask -- probe src/image.rs /tmp/img24/ns.o \
    --crate-type=lib --emit obj -C panic=abort --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_ns"' --cfg 'feature="dispextern_face_records"'
-> exit 0, **0 errors**, **16 warnings** (the same classes Evidence 27 records: 9 `unused doc
   comment` in the mirrors, 4 `_NSRect` FFI-safety, 2 runtime `read`/`write`, 1 `unused attribute`),
   `/tmp/img24/ns.o` **36,448 bytes**
```
**This is the first slice of this unit with a type-checking reading**, which is the point of the
NS configuration: C 7970-8351 hangs on the *file* gate alone (no other arm of the file's `#if`
ladder covers it), so the same probe that compiles the whole unit compiles this slice's text.  The
first two runs of the slice read **4 error lines (2 errors), both in the new text** - `expected
{ , found ;` for C 8061/8065's two `while (…) ;` loops (the port had written C's empty statement
instead of a Rust `{ }` body) and `E0381 used binding ximg isn't initialized` at C 8171's
`&ximg` - and both were retired before the reading above; C's `Emacs_Pix_Container ximg;` is
zeroed (PORTING.md 1.5), the `xnmalloc`/`xbm_load_image` 7697 shape this file already uses.

**33. `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/image.c
femacs/src/image.rs` -> **`FAIL` with 25 rows before the slice, 19 after**: the slice retires
**five `missing function`** (`pbm_image_p`, `pbm_next_char`, `pbm_scan_number`, `pbm_scan_index`,
`pbm_load`) and **all twelve `missing constant`** rows (`PBM_TYPE` … `PBM_LAST`), and the row the
token check then demanded was paid for where the tree's rule says: two rows in
`docs/ports/macro-body-notes.tsv` (`pbm_image_p	0` - C 8019's `return 0` in a `bool` function is the
port's `return false`, the `image_check_image_size` row's class; `pbm_load
background_valid,case,default,goto,switch` - C's `switch` is the `match` of PORTING.md 2.2, C 8113's
`error:` label with its eight backward `goto error`s is one labelled block, and the bit member is
the P-80 mask pair), plus the `ckd_mul` family's argument check, which is why C 8275-8277's two
`ckd_mul`s are written as the `overflowing_mul` pair (the `sfnt.rs` slice-16 / `xnmalloc` 381
shape) - the family's checker reads the numbers *inside* the call, and C's multiplicand
`(two_byte ? 2 : 1) * (type == PBM_COLOR ? 3 : 1)` is exactly there.  The 19 rows that remain are
the remainder's: four `missing function` (`lookup_image_type`, `initialize_image_type`,
`image_prune_animation_caches`, `syms_of_image`), four `missing subr` and ten `missing defsubr`
rows, the `no_mangle` note for `image_prune_animation_caches`, and `line count drifted 16.7%: C
13805 vs Rust 11497` (the drift falls from 20.5%).

**34. The seven text gates.**
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)** (the range table's last row now
splits at C 8351 and still tiles C 1-13805); `check-port-lines.py` **0 file(s) under ten percent, 0
reference line(s); 0 ratchet row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s)
bound, 0 failure(s)**; `check-cond-eval.py src/image.c` **435 conditions, 0 unknown, 0
disagreements**; and `check-mirror-dupes.py`'s ratchet is untouched (no mirror changed).

**35. Verdicts and deviations.**

- **C's `error:` label is reached backward, so the port hoists its body.**  C 8113's label sits
  *inside* C 8109's first `if`, and the eight later `goto error` sites (C 8123, C 8156, C 8168,
  C 8171, C 8232, C 8253, C 8297, C 8308) jump back to it; a Rust `break` can only leave a block
  forward, so the block `'error: { … }` opens at the first statement that can reach the label and
  closes after the last jump site, and C's three-statement label body (`xfree (contents); img->
  pixmap = NO_PIXMAP; return 0;`) stands after the block, where every `break 'error` lands.  The
  success path returns inside the block, exactly as C's fall-through reaches its own `xfree
  (contents); return 1;`, so the statements are not duplicated and no flag is needed (the
  `xbm_read_bitmap_data` `'failure`/`'success` pair is the same shape, and `parse_image_spec`'s
  *forward* `goto maybe_done` keeps its flag because its label's code runs in both paths).
- **C's four post-increment reads are the load-then-step pair** (Deviation 26): C 8109's
  `*p++ != 'P'` (evaluated only when `end - p < 2` is false, which is why the port builds the
  condition as the magic test plus the step), C 8126's `switch (*p++)`, C 8034/8043's
  `*(*s)++` in `pbm_next_char`, and C 8081/8085's `*p++` in `pbm_scan_index`.
- **C's chained assignment `r = g = b = …;` is one statement per assignment in C's right-to-left
  order** (the `dispnew.md` 312 (b) rule), so C 8294/8296's single read of `pbm_scan_index` /
  `pbm_scan_number` happens once, into `b`, and `g`/`r` follow.
- **C's `switch (*p++)` is the port's `match` on the byte** (`b'1'` … `b'6'`, `_` for `default`):
  the `case` keywords cannot be spelled in Rust, which is the notes row's subject.
- **Two conversions written out**: C 8306's `(double) r * 65535 / max_color_idx` is
  `(r as c_double * 65535.0 / max_color_idx as c_double) as c_int` (the `gtkutil.rs` 1215 shape),
  and C 8283's `(two_byte ? 2 : 1) * (type == PBM_COLOR ? 3 : 1)` is written with the two `if`
  expressions plus the `ptrdiff_t` cast the `ckd_mul` family needs.
- **C's uninitialised locals are zeroed** (PORTING.md 1.5): C 8101's `ptrdiff_t size`, C 8106's
  `Emacs_Pix_Container ximg` (measured `E0381`), C 8103's `end`/`p` are written before any read, and
  C 8100's `bool raw_p` is assigned by every `case` arm but zeroed at its declaration.
- **The `<time.h>`-class use-site names**: none - this slice's text names only the unit's own
  helpers, the two P-88 mirror surfaces it already had (`FRAME_TERMINAL!`/`query_colors` under
  `use_cairo`) and the `PIX_CONTAINER_TO_CONTEXT!`/`IMAGE_BACKGROUND!`/`PUT_PIXEL!`/`NO_PIXMAP!`
  macros the earlier slices established, so no mirror changed with this slice.
- **What is *not* claimed**: the PBM reader has never run - no PBM file was opened, no header
  scanned, no pixel stored, and `init_color_table`/`lookup_rgb_color` were not called by anything
  (their only callers are the loaders, and nothing invokes `pbm_load`); the arm probe is a
  *compile* reading of the `have_ns` configuration, not a behaviour test; the format loaders
  between C 8352 and C 13805 are recorded above rather than transcribed because the frozen
  configuration makes their `#if`s false, so the unit's `--check` still reads `FAIL` and this
  slice is not the file's close.

## The eighth slice (C 13301-13805): the tests, the image-type registry and `syms_of_image`

**36. The arm flip.**  The same NS configuration as Evidence 27 (the stronger of the two readings
this lane uses):
```
$ CARGO_TARGET_DIR=/tmp/img24 cargo run -q -p xtask -- probe src/image.rs /tmp/img24/ns7.o \
    --crate-type=lib --emit obj -C panic=abort --cfg 'feature="have_window_system"' \
    --cfg 'feature="have_ns"' --cfg 'feature="dispextern_face_records"'
-> exit 0, **0 errors**, **20 warnings** (the 16 of Evidence 27 plus four `dead_code` ones for the
   arms this configuration leaves out), `/tmp/img24/ns7.o` **62,840 bytes**
```
The slice's own defect iterations are recorded rather than hidden: the first run read **28 error
lines**, of which 12 were `expected one of `,`, `:`, or `}`, found `!`` for `IMAGE_TYPE_INIT!
(…)` written as a struct-literal *field* (a macro cannot supply the field's name, so each of the
seven entries with an `init` initialiser is written twice, under `windowsnt` and under its
negation), and the rest were: the slice literal missing its `&` (`static image_types: &[image_type]
= [`), `image_frame_cache_size`'s `intptr_t` widening to `intmax_t`, and the two names this slice's
new text needs at its use site (`decode_live_frame`, lisp.h 1476; `decode_env_path`, lisp.h 5135)
plus `image_can_use_native_api`, whose C definition is C 8360's inside the range the table now
records, so the GNUstep/`HAVE_NATIVE_IMAGE_API` arms that call it are text-only until that region
lands.

**37. `extract-abi.py --check`.**  **`OK`** - the unit closes: the four `missing function` rows
(`lookup_image_type`, `initialize_image_type`, `image_prune_animation_caches`, `syms_of_image`),
the four `missing subr` rows (`Fimagep`, `Fimage_transforms_p`, `Fimage_cache_size`,
`Finit_image_library`), the ten `missing defsubr` rows and the `no_mangle` note are all retired.
Two rows were paid for where the tree's rule puts them (`docs/ports/macro-body-notes.tsv`):
`lookup_image_type	countof` (C 13493's `countof (image_types)` is the table's length, and the
port's `&[image_type]` slice carries it as `image_types.len ()` because the array's length is
configuration-dependent) and `syms_of_image	doc` (the three `DEFVAR_*` families take C's
docstring positionally - the `syms_of_data` row's class).  The line-count row is answered by this
unit's `docs/ports/line-drift-tolerance.tsv` row (`image	0.13`, basis `total`): 12,104 Rust lines
against C's 13,805 is **-12.3%**, and the reason is the recorded range above - the port is the
*shorter* side because ~4,900 C lines of loaders are recorded rather than transcribed, while it
*is* longer than C's live 4,264 lines, which is what the `total` basis exists for.

**38. The seven text gates.**
`check-declarations.py` **17 declaration sites (0 forbidden, 17 allowed literals), 0 failures**;
`check-citations.py` **676 file(s) scanned, 0 drift, 0 range, 2029 external, 36 allowed, 0 stale**;
`check-dead-file-coverage.py` **145 ledger(s) bound, 0 failure(s)** (153 rows now cover
src/image.c); `check-port-lines.py` **0 file(s) under ten percent, 0 reference line(s); 0 ratchet
row(s), 0 new, 0 stale**; `check-ledger-ticks.py` **238 ledger(s) bound, 0 failure(s)**;
`check-cond-eval.py src/image.c` **435 conditions, 0 unknown, 0 disagreements**; `build-tu
src/image.c` is exit 0 / **336 bytes** / md5 `dbed68837694579185377dbb0821ab62` (the empty object).
No mirror changed.

**39. Verdicts and deviations.**

- **Nine functions gained the C ABI.**  The `image_types[]` entries store the loaders' addresses in
  `struct image_type`'s `Option<unsafe extern "C" fn (…)>` members, and a Rust-ABI `fn` does not
  coerce to them (the ledger's own `E0308` reading for the two XPM callbacks), so `xbm_image_p`,
  `xbm_load`, `xpm_image_p`, both `xpm_load`s, `canvas_image_p`, `canvas_load`, `pbm_image_p`,
  `pbm_load` and `image_clear_image` are declared `unsafe extern "C" fn` - the `sfntfont_get_glyph`
  shape, and the same reason.
- **C's `image_types[]` is a slice in the port.**  Its length is configuration-dependent (one entry
  per configured format), which a Rust array's *type* cannot spell without enumerating the arm
  combinations; the entries keep C's order and CFG arms, and `countof (image_types)` reads
  `image_types.len ()` (the notes row above).  The seven entries that carry C's
  `IMAGE_TYPE_INIT (…)` initialiser are written twice, under `feature = "windowsnt"` (with
  `init: Some (…)`) and under its negation (without the member, which C's `#ifdef WINDOWSNT` keeps
  out of the record), because a macro cannot supply a struct field's *name*.
- **`syms_of_image`'s 43 `DEFSYM`s are the mirror's no-op macro** (P-23: `globals.rs` provides the
  Q symbols directly), so the invocations carry C's names and strings and compile to nothing; the
  five `DEFVAR_LISP`/`DEFVAR_BOOL`/`DEFVAR_INT` invocations write C's docstrings in the mirror's
  positional parameter, and the three symbol values C assigns (`Vimage_types = Qnil`,
  `Vmax_image_size = make_float (MAX_IMAGE_SIZE)`, `cross_disabled_images = 0`,
  `Vimage_cache_eviction_delay = make_fixnum (300)`, `Vimage_scaling_factor = Qauto`) are written
  out in C's order - including `decode_env_path (0, PATH_BITMAPS, 0)`, whose `epaths.h` macro the
  port carries at the use site with the frozen build's measured value (`""`, the `PATH_DOC` shape).
- **C's `#if` arms inside the bodies are `#[cfg]` blocks**, one arm per `#elif` link with each
  excluding the ones before it (the file's gate-mapping rule): `image-transforms-p`'s three-way
  chain, `image-cache-size`'s `HAVE_WEBP || HAVE_GIF` block, `initialize_image_type`'s `WINDOWSNT`
  body, `syms_of_image`'s eleven gated groups (including the NS `HEIC` pair and the
  `HAVE_NATIVE_IMAGE_API` `native-image`/`bmp`/`gdiplus` group), and the two `defsubr` rows C gates
  (`Simagemagick_types` under `HAVE_IMAGEMAGICK`, `Slookup_image` under `GLYPH_DEBUG`).
- **What is *not* claimed**: no image was created, no `defsubr` ran, no `DEFSYM` was evaluated (the
  macro is empty), `lookup_image_type`/`initialize_image_type` have never looked anything up, and
  `syms_of_image` has never been called - it is reached from `emacs.c`'s `syms_of_*` chain, which
  no gate links here.  The arm probe is a **compile** reading of the NS configuration: it proves the
  new text type-checks there, not that any of it behaves.
