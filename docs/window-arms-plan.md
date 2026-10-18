# 窗口系统臂的施工清单（decision C 的队列）— 2026-10-18，status 4.747

`docs/baseline/window-arms-plan.tsv`（由 `python3 tools/gen-window-arms-plan.py` 生成）把
`docs/baseline/android-arms.tsv` 里**属于窗口系统一族**的 40 个名字变成可执行的行：每个名字一条
C 定义位置、它所在的 `#if` 区域（起止行）、将要承载这段臂的端口单元、以及预处理条件。

这一页**不改代码、不做决定**：它只是把"选定 C 之后要干什么"提前准备好，免得决定本身还要再付一次
规划的代价（见 `docs/handoff.md`）。

## 分布

| C 文件 | 行数（名字数） |
|---|---|
| `src/xdisp.c` | 27 |
| `src/xfaces.c` | 3 |
| `src/frame.c` | 3（`Vx_resource_class`、`Vx_resource_name` 是 `DEFVAR_LISP`） |
| `src/androidterm.c` | 2（`x_underline_*` 两条 `DEFVAR_BOOL`） |
| `src/androidfns.c` | 2（`Vx_*_pointer_shape`） |
| `src/msdos.c` | 1（`tty_draw_row_with_mouse_face`） |
| `src/frame.h` | 1（`IMAGE_OPT_FROM_ID`，`HAVE_TEXT_CONVERSION` ＋ 窗口系统） |

共 **40** 条；`gen-window-arms-plan.py` 的读数里已经没有任何 "no DEFVAR found"。

## 选定 C 之后的步骤

1. 按 C 文件分组取 `#ifdef HAVE_WINDOW_SYSTEM` 的**整段**（`platform-arms.tsv` 有每条区域的起止与
   C 行数；`xdisp.c` 一族最大）。
2. 在端口对应单元里加 `#[cfg (feature = "have_window_system")]`，把臂**照 C 写出来**（不是空壳：
   P-303/P-311 的反面教材）。
3. 跑 `python3 tools/check-cond-eval.py --verified docs/baseline/config/cond-eval-verified.txt`，
   必须仍是 **0 disagreements**（当前 5,662 条件 ✓）。
4. 重跑 `python3 tools/gen-android-arms.py` 与 `python3 tools/check-android-imports.py`，看棘轮从
   71 往下走；每轮如实记账。
5. 归零后跑 `docs/android-app-readiness.md` 的判据 2b（模拟器上的应用层断言）。

## 与上游缺口（31 条）的分界

那 31 条**不在**本清单：它们要么是 bionic 在 API 21 没有的符号，要么是 macOS 专有接口，C 自己的
构建里同样不存在（见 `docs/status.md` 4.715/4.725）。它们不会因为写窗口臂而消失，所以选 C 之后
`dlopen` 的判据要按"40 条归零 + 31 条按 C 编排"来读，而不是"71 条归零"。
