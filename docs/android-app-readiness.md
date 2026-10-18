# Android「应用真正跑起来」的判据（2026-10-18，status 4.743）

DoD-6 的"应用层"至今未断言。这一页是它的验收表：每条判据、复跑命令、当前读数、状态。
`tools/check-authorized-platforms.sh` 收尾时会用 `tools/android-readiness.py` 把这三行打出来
（数字从 `docs/baseline/android-imports-pending.txt` 与 `docs/baseline/android-arms.tsv` 现算）。

| # | 判据 | 复跑命令 | 当前读数（4.743） | 状态 |
|---|---|---|---|---|
| 1 | `libemacs.so` 能被 `dlopen`（未决导入归零） | `python3 tools/check-android-imports.py` | **71** 条未决、0 new；链接带 `-z now`（`llvm-readelf -d` 读到 `BIND_NOW`）⇒ 任一未决即不可加载 | ⛔ 未达 |
| 2a | APK 带与 C 同构的 `assets/`（无 dump，Lisp 从这里读） | `bash tools/check-android-kotlin.sh --apk DIR --native libemacs.so --assets` | **4,360** 个成员、APK **48 MB**、含 `assets/directory-tree`（78,642 B）与 `versions`/`build_info` | ✅ 达成 |
| 2b | 启动到 Lisp | `FEMACS_EMULATOR_BUDGET_OK=1 bash tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk /tmp/femacs-apk/emacs.apk` | 断言 `setEmacsParams` 的 `Initializing femacs …`、`EmacsThread` 的 `initEmacs`、无 linker/tombstone | ⛔ 未达（等 1） |
| 3 | 40 条窗口系统臂的取舍 | `docs/decisions-needed.md` 问题 4 | A 维持 P-08 / B 无差别逐行写 / **C 按目标配置决定活/死（建议）** | ⏳ 待你决定 |

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
