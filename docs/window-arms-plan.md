# 窗口系统臂的施工清单（decision C 的队列）— 2026-10-18，status 4.751

`docs/baseline/window-arms-plan.tsv`（由 `python3 tools/gen-window-arms-plan.py` 生成）把
`docs/baseline/android-arms.tsv` 里**属于窗口系统一族**的 32 个名字变成可执行的行：每个名字一条
C 定义位置、它所在的 `#if` 区域（起止行）、将要承载这段臂的端口单元、以及预处理条件。

这一页**不改代码、不做决定**：它只是把"选定 C 之后要干什么"提前准备好，免得决定本身还要再付一次
规划的代价（见 `docs/handoff.md`）。

## 分布

| C 文件 | 名字数 |
|---|---|
| `src/xdisp.c` | 26 |
| `src/xfaces.c` | 3 |
| `src/frame.h` | 2（`IMAGE_OPT_FROM_ID` 与 `fset_parent_frame`，`HAVE_TEXT_CONVERSION` ＋ 窗口系统） |
| `src/frame.c` | 1 |

共 **32** 条；`gen-window-arms-plan.py` 的读数里已经没有任何 "no DEFVAR found"。

## 这 32 条就是 Android 上缺的全部（4.751 量到）

`python3 tools/gen-android-gaps.py` 把每条名字的 C 引用点找出来，并用冻结的 Android 宏表判它是否
被 C 编译（`docs/baseline/android-gaps.tsv`）。设备读数（`check-android-imports.py
--providers-dir`）现在是 **32 条，而且 32 条全部**是这些「C 会编译对它的引用」的臂：

| 判据 | 条数 | 含义 |
|---|---|---|
| **Android 上活** | **32** | C 在该配置下会编译对它的引用（`xdisp.c`、`xfaces.c`、`frame.c`、`frame.h`）⇒ 选 C 就得照写 |

4.749/4.750/4.751 三轮里，另有 **21 条**曾经出现在读数里、但不属于这个决定：它们全都是端口「编了
C 不编的代码」的偏差（Darwin 变体没挂门、`RENAME_EXCL`/`_CS_PATH`/`USABLE_POSIX_SPAWN` 这类
派生或系统头条件没照 C 表达、utmp 循环没挂 gnulib 的条件、以及 8 条把定义留在 Android 不编译的
单元里）⇒ 已按 C 的门逐条修掉，见 `docs/status.md` 4.749–4.751。

## 选定 C 之后的步骤

1. 按 C 文件分组取 `#ifdef HAVE_WINDOW_SYSTEM` 的**整段**（`platform-arms.tsv` 有每条区域的起止与
   C 行数；`xdisp.c` 一族最大）。
2. 在端口对应单元里加 `#[cfg (feature = "have_window_system")]`，把臂**照 C 写出来**（不是空壳：
   P-303/P-311 的反面教材）。
3. 跑 `python3 tools/check-cond-eval.py --verified docs/baseline/config/cond-eval-verified.txt`，
   必须仍是 **0 disagreements**（当前 5,662 条件 ✓）。
4. 重跑 `python3 tools/gen-android-arms.py` 与 `python3 tools/check-android-imports.py`，看棘轮从
   41 往下走、设备读数从 32 归零；每轮如实记账。
5. 归零后跑 `docs/android-app-readiness.md` 的判据 2b（模拟器上的应用层断言）。

## 与「上游缺口」的分界

棘轮里还剩 9 条非窗口名字（`backtrace`、`getifaddrs`、`posix_spawn` 一族、`stdin`/`stdout`/`stderr`
等）：它们是 **API 21 的桩库**没有、而**交付设备自己提供**的名字（设备读数为 0 条非窗口未决）⇒
不会因为写窗口臂而改变，也不需要决定；bionic 在 21 之后补进来的符号按 C 的编排留在棘轮里
（`docs/status.md` 4.749 ③）。
