# 交接页（2026-10-19，status 4.829）

给下一次会话的**一页纸**。逐轮细节在 `docs/status.md`（两万行），规则在 `PORTING.md`，
验收表在 `docs/current-state.md` 与 `docs/android-app-readiness.md`。

## 判定

**6/8 = 75%**（口径：8 条等权重、**整条达成才计 1**、部分达成记 0）。

| # | 项 | 状态 | 一条命令 | 最后读数 |
|---|---|---|---|---|
| 1 | 590 条 1:1 转写 | ✅ | `python3 tools/extract-abi.py --all` | 590 done / 0 failing（4.732） |
| 2 | 布局/符号/subr/声明/清单 | ✅ | `xtask gate-fast`（八步）＋ `xtask gate-probes` | 两个都 exit 0；`dead-arms` 340 单元 0 failed（4.730/4.733） |
| 3 | L0–L5 | ✅ | `acceptance-l0.py --check build/port/src/emacs`、`acceptance-l1.py --check build/port/src/emacs` | L0 19/0、L1 111/0+1 accepted（4.744）；L5 以 4.676 实测引用 |
| 4 | 生成物/loadup/dump | ✅ | `check-generated.py`、`check-loadup.py`、`check-dump-offsets.py` | 244 文件 0 DIFFERENT；loadup 一致；dump 0（4.744） |
| 5 | 原样树逐字节 | ✅ | `check-verbatim.py` | 4,604 文件 0 failures（4.734） |
| 6 | 平台（macOS + Android） | ⚠️ 部分 | `bash tools/check-authorized-platforms.sh` | exit 0；macOS 11/0、112 class、69 JNI、assets 4,360、48 MB 签名 APK；**应用层未断言**（4.742） |
| 7 | 性能 | ⏸ 暂挂 | `tools/bench.sh` | 同臂 0.70–0.86×（P-193），按用户决定不追平 |
| 8 | 签名 + 台账 | ✅ | `check-ledger-ticks.py`、`git verify-commit` | 238 绑定 0 失败；近期提交全部签名并推送（4.744） |

## 第 6 条还差什么

`python3 tools/android-readiness.py` 会打印三行（平台脚本收尾也会打）：

1. **`libemacs.so` 可 `dlopen`**：`check-android-imports.py` ⇒ **9 条未决 / 0 new**（API 21 桩；**设备读数 0 —— 全部窗口系统臂写完**），链接 eager
   binding（`BIND_NOW`）⇒ 必须**归零**。
2. **assets**：✅ 已达成（4,360 个成员、`assets/directory-tree` 78,642 B、`version`/`build_info`）。
3. **40 条窗口系统臂**：待问题 4。

构成可复跑：`python3 tools/gen-android-arms.py`（名字 → 类 → 定义位置 → 预处理条件）、
`tools/gen-android-gaps.py`（每条名字的 C 引用点与「C 自己编不编」的判据），
清单见 `docs/decisions-needed.md` 附录（**A 窗口臂 0 / B 上游缺口 9 / C 其它 0**）——那 9 条是 API 21 的桩
读数缺、**交付设备自己提供的名字**（`backtrace`、`getifaddrs`、`posix_spawn` 一族等）；逐条的 C 侧判据见
`docs/baseline/android-gaps.tsv`。

## 设备上的崩溃：现在知道它在**分配路径**里（4.829）

干净安装 + **带 `gettid()`** 的落盘探针（四个工具见 `tools/add-*-probe.py`）给出一致读数：
最后几条是 `FA m 0 1` → `FA c1 160 591` → **`AL mv0 1 0`** ⇒ 进 `make_vector (1, elt)` 之后、
`allocate_clear_vector` 返回之前；`AL gc0`/`AL gce` = **27/27** ⇒ GC 每次都跑完；
两次运行的停点不同（`CB csi 0 0` 与 `AL mv0`），且**探针自身会挪动停点**（写入量 658,687 行）
⇒ 与**堆被写坏**一致，不是某条语句。

下一步（都要新设备轮）：① `libc.debug.malloc.options "guard backtrace"`（4.830：`setprop` 已设但未生效，需要重启 zygote/`stop`+`start`）；② `ENABLE_CHECKING` 的 Android 构建（本仓有 `*.checking.o` 通路）。

两条会当场弄坏 `.so` 的教训：探针**必须插在 `#[unsafe(no_mangle)]` 之上**（否则符号消失、`dlopen` 报
`cannot locate symbol ...`）；日志必须带线程号（两个线程往同一份 `probe.log` 写）。

## 提交阻塞已解除（4.828）

`git commit -S` 现在 exit 0（`pinentry-mac` + agent 缓存），4.7xx 起 25+ 次「弹窗取消」不再复现：
101 个在飞文件已分三批签名入树并推送（`30906074`、`1ef04c27`、`46f13960`）。

## 唯一待决问题（问题 4）

- **A 维持 P-08**（死臂只记录）：macOS 侧成立、零工作量；Android 那 40 条永远无人提供 ⇒ `.so`
  永不可加载，DoD-6 停在结构面。
- **B 无差别逐行写**（含 darwin 死臂）：口径最满；全树 `HAVE_WINDOW_SYSTEM` 区域约 14,946 行，
  darwin 上永不执行。
- **C（建议）按目标配置决定活/死**：活臂照写不降标准、macOS 读数不变；工作队列即
  `docs/baseline/platform-arms.tsv` 里 Android 相关的区域（`HAVE_ANDROID` 9,597 行、
  `HAVE_WINDOW_SYSTEM` 14,946 行、`HAVE_MODULES` 191 行，见 4.711）。

选定 **C** 后的落地顺序：`platform-arms.tsv` 取 `#ifdef HAVE_WINDOW_SYSTEM` 的区域 → 按
`android-arms.tsv` 的 40 个名字对齐 → 每处加 `#[cfg (feature = "have_window_system")]` 并把臂写出
→ `check-cond-eval --verified` 保持 0 disagreements → 棘轮逐条下降 → 归零后跑判据 2b。

## 五条施工纪律（血泪换的，别再踩）

1. **生成物**：`femacs/src/include/config.rs` 是生成物；`git add -A` 前先看 `git status`（P-319）。
2. **门报红先分三类**：代码错 / 表过期 / 口径错（P-320）；表过期就改表，口径错**不放宽判据**。
3. **cfg 词表**：只写 `xtask` 能发出的键；`have_*` 看配置有没有正向键，`not_have_*` 看
   `"undefined"`（P-310/P-312）。
4. **一个 C 单元 include 多个 `.c`** 时，符号比对要取并集（`ALIASES` 是 stem 列表，4.736）。
5. **`link-program` 的工具**（`lib-src/*`）是 host 工具，不要按 `have_android` 门住（4.741）。

## 最近提交

`05c6a3dd`（入口快照对齐）· `dba15328`（L0/L1/生成物复跑）· `dbcd0726`（验收表 + readiness 输出）
· `a9962bb5`（平台门带 assets）· `829b54dd`（`directory-tree` 通）· `4c277543`（cond-eval 全量）
· `688d3d93`（`check-symbols` 别名）
