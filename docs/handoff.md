# 交接页（快照 2026-10-21 / status 4.1005）

给下一次会话的**一页纸**。逐轮细节在 `docs/status.md`（两万三千行），规则在 `PORTING.md`，
判定表在 `docs/current-state.md`。

**先跑这一条**（load≈6 时约 30 分钟）：

```sh
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- bash tools/verify-dod.sh --full
```

它跑第 **1 / 2 / 4 / 5 / 6 / 8** 行和两道文档门，第 7 行只打印“按用户决定暂挂、记未达”。
**第 3 行（L0–L5）不在这个脚本里**：L5 与 GC 压测各用 `tools/l5-sweep.sh [--gc-stress]`
（564 文件、30 一个块、分块可续跑），跑完**先合并**成 `docs/baseline/ert-l5-sweep.tsv` /
`ert-gc-stress-sweep.tsv`，**再**删分块。

## 判定

**7/8 = 87.5%**（8 条等权、整条达成才计 1、部分达成记 0；第 6 条按用户授权以 macOS 验收，
另加 Android 设备读数）。逐行读数以 `docs/current-state.md` 为准，本页是 4.1005 的快照。

| # | 项 | 状态 | 一条命令 | 最后读数 |
|---|---|---|---|---|
| 1 | 590 条 1:1 转写 | ✅ | `python3 tools/extract-abi.py --all` | 590 done / 0 failing（4.998） |
| 2 | 布局/符号/subr/声明/清单 | ✅ | `xtask gate-fast`、`xtask gate-probes` | gate-fast exit 0（4.1000，约 570 s / 469 MB）；gate-probes 340/340（4.998，135 s / 372 MB） |
| 3 | L0–L5 | ✅ | `acceptance-l0.py`、`acceptance-l1.py`、`tools/l5-sweep.sh` | L0 19/0、L1 111/0+1（4.744）；L5 552 same / 7 both-fail / 4 crashed（全两侧）/ 1 port-worse（tramp，孤立 3 次判两侧抖动）/ 0 port-better（4.1000） |
| 4 | 生成物/loadup/dump ＋ 压力 | ✅ | `check-generated.py`、`check-loadup.py`、`check-dump-offsets.py`、`l5-sweep.sh --gc-stress` | 244 对 0 DIFFERENT、loadup 一致、dump 0；GC 压测 552 / 7 / 4（全两侧）/ 1 port-better / 0 port-worse（4.1001） |
| 5 | 原样树逐字节 | ✅ | `check-verbatim.py` | 4,604 文件 0 failures（4.1000 内跑） |
| 6 | 平台（macOS + Android） | ✅ | `bash tools/check-authorized-platforms.sh` | exit 0（4.1000）：macOS smoke 11/0、differential 92/0；Android APK 922,862 B / 114 class、`libemacs.so` 5.4 MB / 69 JNI、设备三元组（4.962）、设备口径导入 304/0（4.966） |
| 7 | 性能 | ⏸ 暂挂（记未达） | `tools/bench.sh --rounds 5` | load≈6 两次：`bc-loop` 1.43 / 1.53×、`bc-sum` 1.29 / 1.39×、`bc-car` 1.26 / 1.35×、`regexp-repl` 1.08 / 1.09×、`buffer-scan` 0.99 / 0.99×、`sort` 1.14 / 1.15×（4.999）；不录基线 |
| 8 | 签名 + 台账 | ✅ | `check-ledger-ticks.py`、`git verify-commit` | 238 台账绑定 / 0 failure / 0 行无台账（4.1005 复跑）；提交全部签名并推送 |

## 交付面

- **macOS 是终端版**：冻结配置里 `HAVE_NS` / `HAVE_WINDOW_SYSTEM` 都是 `undefined`（参照构建同样），
  `build/port/src/emacs` 不链 AppKit、无 `ns_term_init`；Cocoa 的 Swift 代码按 P-51 是承载文本，
  `have_ns` 翻牌要等 AppKit 镜像（3.262）。
- **Android 是 GUI 面**：签名 APK 能装、能渲染、能打字、能转屏，同 `pid` 且 `Fatal signal` 0。

## 还剩什么

1. **第 7 条**：字节码派发比参照的 threaded 臂慢 1.3–1.5×；`Rust` 稳定版无法复刻计算跳转，
   关 `overflow-checks` 与上 nightly 都被项目约束排除 ⇒ 按用户决定**暂挂、记未达**。
2. **声明缺口 598 条**已收口成三类：**类型墙 325**（要 `struct frame`/`struct window` 在镜像 scope 有名字，
   见 P-323）、**死平台 177**（`xterm.h` 按其镜像末尾的记录有意不搬，`gtkutil`/`w32*` 同属不交付平台）、
   **C 自身陈旧声明 21**（参照里也只有声明没有定义，或挂在 `#undef` 的宏下）。活配置下已无可直接搬的。
3. 若要把 macOS 从终端版推进到 Cocoa app：先做 AppKit 镜像，再翻 `have_ns`，属独立大工程。

## 未决计数

窗口臂 0 / 上游缺口 5 / 其它 0（`docs/decisions-needed.md` 附录的 A/B/C 三类；那 5 条是
API 21 桩读数里缺、交付设备自己提供的名字，见 `docs/baseline/android-gaps.tsv`）。

## 已知红（不是回归）

`tools/check-verbatim-declarations.py` 退出 1：1,897 条全部在照抄的 `femacs/lisp/*.el`，
就是用户要求逐字节保留的声明行 ⇒ 判据与决定冲突，**不修、不判红**；照抄树以 `check-verbatim` 为准。

## 五条施工纪律（血泪换的，别再踩）

1. **生成物**：`femacs/src/include/config.rs` 是生成物；`git add -A` 前先看 `git status`（P-319）。
2. **门报红先分三类**：代码错 / 表过期 / 口径错（P-320）；表过期就改表，口径错**不放宽判据**。
3. **cfg 词表**：只写 `xtask` 能发出的键；`have_*` 看配置有没有正向键，`not_have_*` 看 `"undefined"`（P-310/P-312）。
4. **一个 C 单元 include 多个 `.c`** 时，符号比对要取并集（`ALIASES` 是 stem 列表，4.736）。
5. **`link-program` 的工具**（`lib-src/*`）是 host 工具，不要按 `have_android` 门住（4.741）。

## 最近提交

`fa41571e`（macOS 交付口径）· `6269e9b2`（声明缺口收口）· `79ed87b5`（红门定性）
· `0eb45123`（清理 469 MB）· `b94f9ffe`（GC 压测）· `4ba8fd35`（L5 重跑）
· `4c41e228`（`lisp.h` 254 条原型）
