# Swift 导入 `unclassified` 70 条的裁定 — 2026-10-04

`tools/check-swift-imports.py` 把 `femacs/platform/**/*.swift` 的每条 `@_silgen_name("X")`
导入与它的提供者对比。提供者有四类来源：参照树编出来的对象（`build/emacs-ref/**/*.o`）、
macOS SDK 的 `.tbd`（AppKit/Foundation/CoreGraphics/CoreText/CoreFoundation/QuartzCore/
CoreImage/UniformTypeIdentifiers/CoreServices 与 `libSystem.B`/`libobjc.A`）、参照 C 文本
（`emacs/src|lib|lib-src` 的 `.h/.c/.m`，即 C 自己的宏/INLINE/枚举/DEFSYM），以及 Rust 侧
已经以 C-ABI 名字命名的项（`femacs/src/*.rs`、`femacs/src/include/*.rs`、`femacs/lib/**` 的
`pub fn` 与 `macro_rules!`）。四条来源都不命中的导入是 `unclassified`，本文件逐条给出裁定。

测量与复跑：

```
python3 tools/check-swift-imports.py --report | rg '^REVIEW'   # 本次 70 行
python3 tools/check-swift-imports.py --write-pending           # 重写 docs/baseline/swift-import-pending.txt
```

`--write-pending` 由集成者执行；本文件是那张 ratchet 的 reason 列的依据。测量时工作树里有别的
lane 在改 `femacs/src/frame.rs`、`femacs/src/include/dispextern.rs`、`femacs/src/xdisp.rs`、
`femacs/platform/macos/nsterm.swift`；下表 `femacs/src/frame.rs:*` 的行号是**测量时刻**的行号。

四个裁定类各一句：

1. **核心欠一个 C-ABI 导出** —— 这个名字是某个镜像在 Rust 侧命名的记录成员/标志/访问器构造
   （给出 Rust 侧 `文件:行`），Swift 按 C 名导入它，因此核心必须以该名字导出一个 C-ABI 条目。
2. **别的 Swift 单元提供** —— 实现已经存在于另一个 `.swift`（跨文件的 Objective-C 类方法同此），
   只是该单元还没有把 C-ABI 名字 `@_cdecl` 出去。
3. **平台符号** —— 由 libSystem/libobjc/某个 framework（或 GNUstep）提供，不是本仓库的欠账。
4. **拼错了** —— 参照树与 Rust 树里都不存在这个名字，需要改名或删除。

## 表

| name | 导入它的 Swift 文件 | 裁定 | 证据 |
| --- | --- | --- | --- |
| `EmacsImage_allocInitFromFile` | femacs/platform/macos/nsfns.swift | 2 | femacs/platform/macos/nsimage.swift:401 `class EmacsImage` + :419 `class func allocInitFromFile`；该单元需以 `EmacsImage_allocInitFromFile` 导出（无 `@_cdecl`） |
| `GSFontEnumerator_availableFontDescriptors` | femacs/platform/macos/nsfont.swift | 3 | GNUstep 类方法 `-[GSFontEnumerator availableFontDescriptors]`（C 使用点 src/nsfont.m:799）；macOS SDK 无此符号（无 `.tbd`） |
| `GSFontEnumerator_sharedEnumerator` | femacs/platform/macos/nsfont.swift | 3 | GNUstep 类方法 `+[GSFontEnumerator sharedEnumerator]`（C 使用点 src/nsfont.m:790）；macOS SDK 无此符号（无 `.tbd`） |
| `NSColor_colorWithUnsignedLong` | femacs/platform/macos/nsfns.swift | 2 | femacs/platform/macos/nsterm.swift:169 `class func colorWithUnsignedLong`；需以 `NSColor_colorWithUnsignedLong` 导出 |
| `buffer_filename` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/buffer.rs:414 `pub filename_: Lisp_Object`；C `BVAR (XBUFFER (buf), filename)` buffer.h:308 |
| `face_set_background` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/dispextern.rs:1948 `pub background: c_ulong`（`pub struct face` @1943）；C dispextern.h:1819 |
| `frame_explicit_name` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:224 `macro_rules! explicit_name`（FRAME_EXPLICIT_NAME_SHIFT） |
| `frame_icon_name` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:45 `pub icon_name: Lisp_Object` |
| `frame_left_pos` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:106 `pub left_pos: c_int` |
| `frame_menu_bar_items_used` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:69 `pub menu_bar_items_used: c_int` |
| `frame_menu_bar_vector` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:59 `pub menu_bar_vector: Lisp_Object` |
| `frame_n_tool_bar_items` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:90 `pub n_tool_bar_items: c_int` |
| `frame_param_alist` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:54 `pub param_alist: Lisp_Object` |
| `frame_selected_window` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:51 `pub selected_window: Lisp_Object` |
| `frame_set_background_pixel` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:139 `pub background_pixel: c_ulong` |
| `frame_set_border_width` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:110 `pub border_width: c_int` |
| `frame_set_child_frame_border_width` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:111 `pub child_frame_border_width: c_int` |
| `frame_set_double_buffered` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:582 `macro_rules! FRAME_DOUBLE_BUFFERED` + :488 `pub double_buffered: bool`（C nsterm.h:1052） |
| `frame_set_explicit_name` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:224（`explicit_name!` 的写臂） |
| `frame_set_external_menu_bar` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:344 `pub fn FRAME_EXTERNAL_MENU_BAR`（只读桩）；C frame.h:382 `bool_bf external_menu_bar` |
| `frame_set_external_tool_bar` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:354 `pub fn FRAME_EXTERNAL_TOOL_BAR`（只读桩）；C frame.h:367 `bool_bf external_tool_bar` |
| `frame_set_foreground_pixel` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:140 `pub foreground_pixel: c_ulong` |
| `frame_set_internal_border_width` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:112 `pub internal_border_width: c_int` |
| `frame_set_menu_bar_items_used` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:69（写同一字段） |
| `frame_set_menu_bar_lines` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:118 `pub menu_bar_lines: c_int` |
| `frame_set_output_method` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:360 `macro_rules! output_method`（写臂 :362）+ femacs/src/include/frame.rs:167-168 FRAME_OUTPUT_METHOD_SHIFT/MASK |
| `frame_set_tab_bar_height` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:84 `pub tab_bar_height: c_int` |
| `frame_set_tab_bar_lines` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:83 `pub tab_bar_lines: c_int` |
| `frame_set_tab_bar_resized` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:260（`tab_bar_resized!` 的写臂） |
| `frame_set_wants_modeline` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:208 `macro_rules! wants_modeline`（FRAME_WANTS_MODELINE_SHIFT） |
| `frame_tab_bar_redisplayed` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:256 `macro_rules! tab_bar_redisplayed` |
| `frame_tab_bar_resized` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/frame.rs:260 `macro_rules! tab_bar_resized` |
| `frame_tab_bar_window` | femacs/platform/macos/nsfns.swift | 1 | 镜像缺该字段：femacs/src/include/frame.rs:41 的 `struct frame` 无 `tab_bar_window`（C 只在 frame.h:254 的 HAVE_WINDOW_SYSTEM 臂）；使用点 femacs/src/dispnew.rs:1131 在 `#[cfg(feature = "have_window_system")]` 下 |
| `frame_title` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/frame.rs:46 `pub title: Lisp_Object` |
| `frame_tool_bar_items` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:65 `pub tool_bar_items: Lisp_Object` |
| `frame_top_pos` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/frame.rs:107 `pub top_pos: c_int` |
| `image_load_failed_p` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/dispextern.rs:1360（自注 `struct image` 尚未镜像）；C dispextern.h:3348 `bool load_failed_p` |
| `image_pixmap` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/dispextern.rs:1360（无 pixmap 字段）；C dispextern.h:3222 `Emacs_Pixmap pixmap, mask;` |
| `ns_output_background_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:426 `pub background_color` |
| `ns_output_cursor_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:424 `pub cursor_color` |
| `ns_output_display_info` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:478 `pub display_info` |
| `ns_output_explicit_parent` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:455 `pub explicit_parent` |
| `ns_output_foreground_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:425 `pub foreground_color` |
| `ns_output_icon_left` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:465 `pub icon_left` |
| `ns_output_icon_top` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:464 `pub icon_top` |
| `ns_output_in_animation` | femacs/platform/macos/nsfns.swift, femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/nsterm.rs:484 `pub in_animation` |
| `ns_output_miniimage` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:423 `pub miniimage` |
| `ns_output_parent_desc` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:454 `pub parent_desc` |
| `ns_output_set_background_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:426 |
| `ns_output_set_cursor_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:424 |
| `ns_output_set_cursor_kind` | femacs/platform/macos/nsfns.swift | 4 | 独立名 `cursor_kind` 在 `emacs/` 与 `femacs/src` 全无（`rg -n "cursor_kind" emacs/`、`rg -n "cursor_kind" femacs/src` 只命中 `text_cursor_kinds`）；`struct ns_output`（C nsterm.h 935-1023、镜像 `femacs/src/include/nsterm.rs:420`）也无该成员；该 import 无调用点（死码）。疑为 `ns_output_set_text_cursor`（nsfns.swift:308，字段 `femacs/src/include/nsterm.rs:433`）或 `set_frame_cursor_types`（nsfns.swift:121，C nsfns.m:968-971 的 `ns_set_cursor_type` 就是调它） |
| `ns_output_set_current_pointer` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:450 `pub current_pointer` |
| `ns_output_set_explicit_parent` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:455 |
| `ns_output_set_foreground_color` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:425 |
| `ns_output_set_icon_left` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:465 |
| `ns_output_set_icon_top` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:464 |
| `ns_output_set_in_animation` | femacs/platform/macos/nsfns.swift, femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/nsterm.rs:484 |
| `ns_output_set_miniimage` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:423 |
| `ns_output_set_parent_desc` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:454 |
| `ns_output_set_text_cursor` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:433 `pub text_cursor` |
| `ns_output_set_view` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:422 `pub view` |
| `ns_output_set_window_desc` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:453 `pub window_desc` |
| `ns_output_text_cursor` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:433 |
| `ns_output_view` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:422 |
| `ns_output_window_desc` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/nsterm.rs:453 |
| `set_waiting_for_input_flag` | femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/thread.rs:123 `pub m_waiting_for_input: bool` + :162 `macro_rules! waiting_for_input`（C thread.h:185） |
| `terminal_display_info_ns` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/termhooks.rs:167 `pub ns: *mut c_void`（`union display_info`）+ :212 |
| `terminal_id` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/termhooks.rs:201 `pub id: c_int` |
| `window_contents` | femacs/platform/macos/nsfns.swift, femacs/platform/macos/nsmenu.swift | 1 | femacs/src/include/window.rs:51 `pub contents: Lisp_Object` |
| `window_current_matrix` | femacs/platform/macos/nsfns.swift | 1 | femacs/src/include/window.rs:71 `pub current_matrix: *mut c_void` |

## 计数

| 裁定 | 条数 |
| --- | --- |
| 1 核心欠一个 C-ABI 导出 | 65 |
| 2 别的 Swift 单元提供 | 2 |
| 3 平台符号 | 2 |
| 4 拼错了 | 1 |
| **合计** | **70** |

按导入单元看：`femacs/platform/macos/nsfns.swift` 58 条、`femacs/platform/macos/nsmenu.swift`
13 条、`femacs/platform/macos/nsfont.swift` 2 条；三个名字被两个单元同时导入
（`window_contents`、`ns_output_in_animation`、`ns_output_set_in_animation`），一个核心导出可同时解掉两条。

## 附：测量时顺带读到的跨文件不一致

1. `+[NSColor colorWithUnsignedLong:]` 有两种 ABI 拼写，且都没有提供方：
   `femacs/platform/macos/nsfns.swift:470` 的 `NSColor_colorWithUnsignedLong`（类名入名）对
   `femacs/platform/macos/macfont.swift:1325` 的 `colorWithUnsignedLong`（receiver-first）；两者
   指向同一实现 `femacs/platform/macos/nsterm.swift:169`。
2. `unsignedLong` 同名两处声明、签名可空性不同：`femacs/platform/macos/nsfns.swift:472`
   `(NSColor) -> UInt` 对 `femacs/platform/macos/macfont.swift:764` `(NSColor?) -> UInt`。
3. 同一个 GNUstep-only 问题在同一文件里有两种口径：`femacs/platform/macos/nsfont.swift`
   把 `-[NSTextStorage setFont:]` 改写成了公共 API（:1204-1210），而
   `GSFontEnumerator_*`（:1199/:1201）仍留作无人提供的 import，并在 :1333/:1346 被真实调用。

