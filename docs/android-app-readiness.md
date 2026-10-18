# Android「应用真正跑起来」的判据（2026-10-18，status 4.738）

DoD-6 的"应用层"至今未断言。这一页把"要满足什么才算断言"写成三条可复跑的判据，
并给出每条当前的读数与缺口。前两条属于代码/打包，第三条是待决的政策问题。

## 1. `libemacs.so` 能被 `dlopen`

**判据**：`python3 tools/check-android-imports.py` 的未决数 **归零**。

**为什么是归零而不是"更少"**：这个链接带 `-z now`（NDK 驱动的默认，实测 `llvm-readelf -d`
读到 `BIND_NOW`），任何一条无人提供的导入都会让 `dlopen` 直接失败；工具会打印
`binding eager (-z now): the library will not load`。

**当前读数**：`359 导入 / 71 未决 / 0 new`。构成（`docs/decisions-needed.md` 附录）：
40 条窗口系统臂（受问题 4 约束）、31 条上游/平台缺口（C 自己的构建里同样没有这些名字）。

## 2. 应用能启动到 Lisp

**判据**：logcat 里出现 `setEmacsParams` 的 `Initializing femacs …` 行、`EmacsThread` 调
`initEmacs` 的行，且没有 `UnsatisfiedLinkError`/`tombstone`（`tools/try-android-emulator.sh`
已按这三条断言，退出码即结论）。

**当前缺口（新发现，4.738）**：APK 里 **没有 `assets/`** ✗。C 的 `java/Makefile.in` 200-297
把 `etc`/`lisp`/`info`/`bitmaps` 装进 `install_temp/assets/`，并对大文件做 gzip/大文件分包，
因为 Android 上的 Emacs 从 `assets/` 读 Lisp（没有 dump 文件）。`unzip -l
/tmp/femacs-apk/emacs.apk` 现在是 18 个成员、0 个 `assets/` ⇒ 即使 `.so` 能加载，也找不到
Lisp，启动会在 `loadup` 之前失败。

**要做的事（不需要政策决定）**：

1. 打包 `assets/{etc,lisp,info,bitmaps}`，源就是仓库里逐字节一致的 `femacs/lisp`、`femacs/etc`、
   `femacs/info`、`femacs/etc/images`（C 的 `install_temp` 规则 206-235 起）。
2. 复刻 C 的压缩与大文件方案（236-297）：`aapt2` 不压缩已压缩资源，C 对 ≥1536 字节的 `.el`
   做 `gzip -9n`、把超大文件与 `.gz` 打包进 `assets_raw/`、`assets_gz/`，由
   `lib-src/asset-directory-tool` 生成 `directory-tree`。
3. 打包后重跑第 2 条断言。

## 3. 第 1 条里那 40 条窗口系统臂

受 `docs/decisions-needed.md` 的问题 4 约束（A 维持 P-08 / B 无差别逐行写 / C 按目标配置
决定活/死，建议 C）。这一条不解决，第 1 条不可能归零。

## 进度（4.738）

- 判据 1：未决 **71** 条，未达。
- 判据 2 的第一半（`assets/` 存在且与 C 同构）：**达成** ✓（4.741）——
  `tools/gen-android-assets.py` 复刻了 C 的 `install_temp` 全套：复制四棵树 + 垃圾过滤、
  `gzip -9n` 1,557 个文件、`version`/`build_info`、以及用**端口自己的**
  `build/port/lib-src/asset-directory-tool` 生成 `assets/directory-tree`（78,642 B）。
  `tools/check-android-kotlin.sh --assets` 打进 APK ⇒ **4,360 个 `assets/` 成员、47 MB**。
  C 的 `--api-8` 大文件分包按它的条件（`ANDROID_SDK_8_OR_EARLIER`）不适用，minSdk 21。
- 判据 3：待问题 4。
