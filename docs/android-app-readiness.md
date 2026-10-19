# Android「应用真正跑起来」的判据（2026-10-19，status 4.777）

DoD-6 的"应用层"至今未断言。这一页是它的验收表：每条判据、复跑命令、当前读数、状态。
`tools/check-authorized-platforms.sh` 收尾时会用 `tools/android-readiness.py` 把这三行打出来
（数字从 `docs/baseline/android-imports-pending.txt` 与 `docs/baseline/android-arms.tsv` 现算）。

| # | 判据 | 复跑命令 | 当前读数（4.768） | 状态 |
|---|---|---|---|---|
| 1 | `libemacs.so` 能被 `dlopen` | `python3 tools/check-android-imports.py --providers-dir /tmp/femacs-device-libs`（设备库由 `try-android-emulator.sh` 拉取） | **设备读数 0 条未决**：窗口系统臂全部写完（4.777），`libemacs.so` 的导入首次全部解析（`-z now` ⇒ 任一未决即不可加载）：这就是 decision 4 的对象（`docs/window-arms-plan.md` 的 0 行队列；4.752–4.777 已照 C 写出 36 条（`frame.h` 的两条内联函数、`share_image_cache`、两个矩形 helper、三条 hourglass、`make_monitor_attribute_list`、`prepare_face_for_display`、`get_phys_cursor_geometry`、两个裁剪矩形 helper、`remember_mouse_glyph`、`font_for_underline_metrics`、`gui_get_glyph_overhangs`、`gui_write_glyphs`、`gui_insert_glyphs`、`gui_clear_end_of_line`、`gui_fix_overlapping_area`、`draw_phys_cursor_glyph`、`display_and_set_cursor`、`gui_update_cursor`、`gui_clear_cursor`、`find_hot_spot`、`gui_draw_vertical_border`、`redraw_image_glyphs`、`get_tab_bar_item_kbd`、`handle_tab_bar_click`、`handle_tool_bar_click`、`face_for_font`、`expose_frame`（连同 `expose_window_tree`/`expose_window`/`expose_line`/`expose_area`/`expose_overlaps`/`phys_cursor_in_rect_p` 与两个窗口分隔条绘制函数）、`gui_produce_glyphs`（连同 `append_glyph`/`append_composite_glyph`/`append_stretch_glyph`/`append_glyphless_glyph`/`produce_glyphless_glyph`/`produce_image_glyph`/`calc_line_height_property`/`IT_APPLY_FACE_BOX`/`IT_EXPAND_MATRIX_WIDTH`/`take_vertical_position_into_account`）））。**非窗口与窗口旁的端口偏差已清零**（4.749–4.751 共 21 条：Darwin 变体的门、`RENAME_EXCL`/`_CS_PATH`/`USABLE_POSIX_SPAWN` 这类头宏与派生条件、utmp 循环、`sysctl`，以及 8 条把定义留在 Android 不编译单元的引用）。逐条见 `docs/baseline/android-gaps.tsv`；API 21 桩读数 `9` 见下表 | ⛔ 未达 |
| 2a | APK 带与 C 同构的 `assets/`（无 dump，Lisp 从这里读） | `bash tools/check-android-kotlin.sh --apk DIR --native libemacs.so --assets` | **4,360** 个成员、APK **48 MB**、含 `assets/directory-tree`（78,642 B）与 `versions`/`build_info` | ✅ 达成 |
| 2b | 启动到 Lisp | `FEMACS_EMULATOR_BUDGET_OK=1 python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- bash tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk /tmp/femacs-apk/emacs.apk` | 断言：**20 s 窗口内进程始终在**（单次 `pidof` 不算断言）、读**应用自己的** logcat、`setEmacsParams` 的 `Initializing femacs …`、`EmacsThread` 的 `initEmacs`、无 linker/tombstone、收尾时设备仍 booted；另外先 `uninstall` 再装并**比对设备上 `.so` 的 md5 与本树的**（4.749 之前那次跑的是上一次会话装进去的旧包，日志里的 `cannot locate symbol "malloc"` 就是它） | ⛔ 未达（等 1） |
| 3 | 窗口系统臂的取舍 | `docs/decisions-needed.md` 问题 4 | A 维持 P-08 / B 无差别逐行写 / **C 按目标配置决定活/死（建议）**。队列只剩 **0 条**，全部在 Android 上是活的 | ⏳ 待你决定 |

## 两种读数：API 桩 vs 设备自己

`--api N` 读的是 NDK 为第 N 级 API 提供的**桩库**，回答"最老的设备能不能加载"；`--providers-dir`
读的是从**交付设备**上拉下来的真实 `.so`，回答"我们要跑的这台机器能不能加载"。bionic 在 21 之
后补进来的名字（`posix_spawn` 28、`backtrace` 33、`getifaddrs` 24）在前者读作未决、在后者不是：

| 读数 | 命令 | 未决 |
|---|---|---|
| API 21 桩 | `python3 tools/check-android-imports.py --api 21` | 9 |
| API 35 桩（NDK 29 的最高一级） | `python3 tools/check-android-imports.py --api 35` | 53 |
| 设备（模拟器 android-36，`/system/lib64` + `/apex/.../bionic`） | `… --providers-dir /tmp/femacs-device-libs` | **0**（窗口系统臂已全部写完） |

所以此前写"31 条上游缺口，C 自己的构建也没有"是**错的**：那 31 条里有 18 条设备提供、C 也照
样导入（parity，不是缺口，只是桩读数的产物），其余 13 条是**我们的偏差**，按 C 的门修即可，
与问题 4 的决策无关（`docs/baseline/android-gaps.tsv` 逐条给了 C 的引用点与判据）。

## 为什么第 1 条必须是"归零"

这个链接是 eager binding（`-z now`，NDK 驱动的默认）。懒绑定下少几个符号还能加载、只在调用时
炸；eager 下任何一条无人提供的导入都会让 `dlopen` 直接失败 —— `check-android-imports.py` 会打印
`binding eager (-z now): the library will not load`。

## assets 那一段复刻了 C 的什么

`java/Makefile.in` 200-297：复制 `lisp`/`etc`/`bitmaps`、去掉 `.gitignore`/`ChangeLog*` 一类垃圾、
把 `.elc` ≥1536 字节对应的 `.el` 做 `gzip -9n`（1,557 个）、写 `assets/version` 与
`assets/build_info`、用 `asset-directory-tool` 生成 `assets/directory-tree`。`--api-8` 的
大文件分包只在 `ANDROID_SDK_8_OR_EARLIER` 时执行，本端口 minSdk 21 ⇒ 按 C 的条件不适用。
`info/` 是 makeinfo 生成物，未建时跳过并打印说明。
