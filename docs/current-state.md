# 当前状态（2026-10-18，status 4.708 时点）

给下一次会话的**入口快照**：判定口径、读数、复跑命令、未决事项。逐轮记录仍在 `docs/status.md`，
口径与规则在 `PORTING.md`，验收细则在 `docs/acceptance.md`。

## 用户已定的方向（不再询问）

1. **逻辑一致优先**：Rust 与 C 逐函数一致；**不为性能改变逻辑**；性能未追平**不得写成达标**。
2. **性能暂挂**（DoD-7 仍记未达）。
3. **平台范围 = macOS 真机 + Android 模拟器**；Windows/Linux/Haiku 搁置。
4. **产品名不改**：`lisp/test/etc/doc/leim/admin` **逐字节原样复制**；兼容接口（函数/变量/路径/Info 节点）不变。
5. **Java 只用最新 LTS**：官方最新 LTS = **JDK 25**（2025-09；26/27 非 LTS），`openjdk@25` 已装且 Android 链在它下面实测全绿 ⇒ 不需要任何旧 JDK（`docs/baseline/android-jdk.txt`）。
6. 内存受控、超限即止；用完的产物及时清理（P-190）。

## 完成定义（8 条等权）— 当前 **6/8 = 75%**

每行一句可复跑的命令、最后一次读数，以及它在本文件与 `docs/status.md` 里的出处。读这张表
不需要先读 `status.md`：命令跑出来的数与表里的数相同，就是这一条的当前证据。

| # | 项 | 判定 | 证据命令 | 最后一次读数（出处） |
|---|---|---|---|---|
| 1 | 590 条 1:1 转写 | ✅ | `python3 tools/extract-abi.py --all` | **590 done / 0 in progress / 0 failing**（4.671） |
| 2 | 布局/符号/subr/defvar/声明/清单 | ✅ | `cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- gate-fast` | exit 0（八步；**4.682 在本树重跑** 535 s / 峰值 539 MB。红门的起因是 `window.rs` 的 dispextern 边，现改由 `editfns`/`insdel`/`textprop` 三个单元自带，见 status 4.682） |
| 3 | L0–L5 | ✅ | `python3 tools/acceptance-l0.py --check`、`tools/acceptance-l1.py --check`、`python3 tools/check-ert.py --selector-preset default --tsv /tmp/ert.tsv` | L0 **19/0**；L1 **111 文件 / 0 失败 / 1 接受**；L5 默认选择器 **same 552 / both-fail 7 / crashed 5 / port-worse 0**（`docs/baseline/ert-sweep-default.tsv`，4.676）。`all` 档唯一 `port-worse` 是上游 `:unstable` 的 `erc-d-run-linger`（P-293） |
| 4 | `.elc`/dump/生成物/loadup | ✅ | `python3 tools/check-generated.py`、`tools/check-loadup.py`、`tools/check-dump-offsets.py` | **244 文件：195 identical / 49 name-only / 0 DIFFERENT / 0 missing**；两树 loadup 逐文件一致；19 处 dump offset 0 failure（4.600/4.678） |
| 5 | 原样树逐字节 | ✅ | `python3 tools/check-verbatim.py` | **6 棵树 / 4,604 文件 / 0 failures**（4.679） |
| 6 | 平台（macOS + Android） | ⚠️ 部分 | `bash tools/check-authorized-platforms.sh` | macOS smoke **11/0**；`gate-android` **181,486 B / 112 class / 签名**；`libemacs.so` **5,944,816 B / 69 个 JNI 导出**；含 `lib/arm64-v8a/libemacs.so` 的**签名 APK 3.9 MB**；**Android 单元 sweep 91/91 + 0 unit failure**—— 四段 exit 0。**模拟器冒烟已能跑到应用层**（4.705–4.707）：AVD `femacs-arm64-api36` boot ✓、`adb install` Success ✓、`am start -W` Status: ok ✓，并挖出并修掉三个真缺陷（dex 缺 Kotlin 运行库 ✗→✓、`EmacsConfig` 抄错列表 ✗→✓、`-nodefaultlibs` 掉 libc ✗→✓）；**但库仍不可 `dlopen`**：新门 `python3 tools/check-android-imports.py` 读到 **402 导入 / 119 未决**（棘轮 `docs/baseline/android-imports-pending.txt`，构成：`not_found` 66 / `HAVE_WINDOW_SYSTEM` 臂 31 / 其他 20 / 平台名 2）⇒ 应用层结论**未达** |
| 7 | 性能 | ⏸ 暂挂（记未达） | `tools/bench.sh` | 同臂 0.70–0.86×（P-193）；按用户决定不追平、不记为达标 |
| 8 | 提交签名 + 台账 | ✅ | `python3 tools/check-ledger-ticks.py`、`git log --show-signature` | **238 台账绑定 / 0 failure / 0 行无台账**（4.679）；全部提交 `-S` 并推送 |

**计数口径（一句说清）**：8 条里 `✅` 的是 **1、2、3、4、5、8** 六条；第 6 条是 `⚠️ 部分`、
第 7 条是 `⏸ 记未达`，两条都不计入达成 ⇒ **6/8 = 75%**。此前我写过 7/8（87.5%），那是把第 6 条
的"Android 结构面全绿"误当成整条达成——**已更正**，判定不变：第 6 条要等模拟器冒烟（E1/E2/E3），
第 7 条按用户决定暂挂。8/8 的路径因此是"代码已完成 + 用户做出预算决定（E1 或 E3）"。

**第 2 条的历史要记清**：4.681 的账面上把它写成 `✅`，但同一轮的记录里 `gate-fast` 是**红**的（`spikes/haiku_inc_probe.rs`）⇒
按"当前树上的可复跑证据"严格读，那一刻是 **5/8 = 62.5%**；4.682 把红门修回绿（本树 exit 0）⇒ **6/8 = 75%**。计数规则不变：部分达成记 0。

## 已知红（不属完成定义八条，但每轮要知道）

- **`xtask gate-probes` 已全绿（4.685 实测）**：`dead-arms` **340/340、0 failed**（6 条翻转：no_rint/no_trunc/no_signbit/have_native_comp/HAVE_LOG2/glyph_debug），
  `lisp-abi-probe` 与 7 个 C 差分脚本也都通过 ⇒ exit 0。修的过程中定了三件事：`dead-arms` 改成**逐个编、集中报**（原来第一处红就收工，把 `abi_shim` 之后的读数全遮住了）；
  Haiku/窗口系统那两条翻转**移出**（理由：`dispextern.rs` 750 的 `Display_Info = haiku_display_info` 需要一个 Rust 表达不了的"不完整标签"，记录在反向包含 dispextern.rs 的 `haikuterm.rs` 里；
  NS 那条臂同一形状、当年就没进表；Android 的记录在 `termhooks.rs` 所以能编 —— 读数：那两条开着时 340 个 unit 失败 62 个）；并修掉两处真缺陷（`lread.rs` 的不可变循环变量、`w32fns.rs` 裸 `c_int`）。

- **声明块的缺口是全局的（4.684 测量，方法见 `docs/status.md` 4.684 ⑤）**：**1,032 条** C 声明不在自己的镜像里，其中 **1,006 条在树内别处声明**（名字口径的门因此看不见）、**26 条全树没有**；
  最多的是 `lisp.h` 423、`dispextern.h` 206、`xterm.h` 126、`font.h` 69、`frame.h` 56、`gtkutil.h` 49、`keyboard.h` 36。**这是测量值、不是判据**（正则口径粗、含死分支名）。
  ⇒ 下一条工作线：按 P-297 把 C 的声明块照抄进对应镜像，并把 unit 里的 use-site 副本按 P-26 收回去。

- **`tools/check-authorized-platforms.sh` 的静默失败已修**（4.682）：它在子 shell 里 `cd femacs` 之后用相对路径调 `tools/check-all-units.sh`，
  这一段一直失败而 `tail -1` 把它咽掉；改用 `"$REPO/tools/..."` 之后四段真的都跑了（exit 0）。同类脚本若 `cd` 之后仍用相对路径，都要按这条检查。

## 平台面的现状（前置全解决；一条口径问题等用户决定）

- **texinfo/makeinfo：已装（7.3）** ⇒ Android `configure` 跑到底 ✓（P-223）；**arm64-v8a 镜像 + AVD
  `femacs-arm64-api36` 已就绪** ✓；受控执行器按 R38 自动扩容（实测 4 → 6 GiB ✓、硬上限 16 GiB ✓）⇒
  原 **E1/E2/E3 已作废**（`docs/decisions-needed.md` 保留作历史）。
- **Android 侧现在的真实缺口是可加载性，不是结构** ✓：4.705 修的三个缺陷（dex 缺 Kotlin 运行库 ✓、
  `EmacsConfig` 列表语义 ✓、`-nodefaultlibs` 掉 libc ✓）之后，4.708 又把未决导入从 **214 → 119** ✓：
  `lib/` 单元进了 Android 链接集（`docs/baseline/lib-link-sources-android.txt` ✓，−59 的 `mpz_*`）、
  panic 入口按 rlib 里读到的名字编出并断言 ✓、`strmode` 的条件写反 ✓、`boot-time.c` 的目标谓词/`clock_gettime`/
  `<sys/sysinfo.h>` 镜像补齐 ✓（Android 现在 `src` 91/91 ＋ `lib` 87/87 全绿 ✓）。读数与构成见
  `docs/status.md` 4.708 ✓；门仍是 `python3 tools/check-android-imports.py` ✓。
- **新问题（等用户决定，`docs/decisions-needed.md` 问题 4）**：那 119 条里有一块（31 条）是端口的
  `xdisp.rs` 等处**按 P-08 记成"dead on this target"的 `HAVE_WINDOW_SYSTEM` 臂** ✓ —— 对冻结的
  **darwin** 配置它们确实是死臂 ✓，对用户授权的 **Android** 却是活臂 ✓（`HAVE_ANDROID` 打开 ✓）。
  口径 A（死臂只记录）下 macOS 侧成立、Android 侧永远不可加载；口径 C（按目标配置决定活/死）下
  这些臂要写成条件编译。**注意**：这 31 条不再是全部 —— 剩下 88 条是两类与死臂无关的活儿
  （66 条 `not_found`：端口侧还没有定义或还没接入的单元/全局；20 条 `other` 与 2 条平台名：
  目标接口镜像，如 `__error`、`CLOCK_REALTIME`、`utmpxname`），它们**不需要任何政策决定**，
  我会继续推进。

已解决、**无需旧 JDK** 的 Java 障碍（P-213/P-217）：① `configure` 的两处 `-source 1.7` 探测改用预置缓存变量
`emacs_cv_working_javac=yes` / `emacs_cv_android_x_or_later=yes`；② 在 **JDK 25** 下跑整条 Android 链时唯一的真实错误是
`d8` 拒绝 `javac` 默认 target 的类文件（`Unsupported class file major version 69`）⇒ 给生成的 `R` 类钉 `--release 17`
（Android 构建的常规做法）即绿：181,486 B 签名 APK、112 class；与 JBR 21 的 `classes.dex`/`resources.arsc` 哈希相同 ✓。
默认 JDK 解析：`openjdk@25` → Studio 的 JBR（`JAVA_HOME` 可覆盖）。

## 一条命令复跑

```sh
# 已授权平台（macOS + Android 结构 + 原生库 + 带 .so 的签名 APK）一条命令，失败即停：
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime \
    -- bash tools/check-authorized-platforms.sh

# 模拟器冒烟（应用层断言：进程、native 入口、失败形态；R38 自动扩容）：
ANDROID_SDK_ROOT=$HOME/Library/Android/sdk FEMACS_EMULATOR_BUDGET_OK=1 \
  python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- \
  bash tools/try-android-emulator.sh --avd femacs-arm64-api36 --apk /tmp/femacs-apk/emacs.apk

# Android 可加载面（导入 vs NDK stub 库，棘轮）：
python3 tools/check-android-imports.py

cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- gate-fast      # 静态/链接/布局/声明/清单
cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- gate-android   # Android 结构验收（~1 GB 峰值）
bash tools/smoke.sh                                                                  # macOS 冒烟
python3 tools/check-verbatim.py                                                      # 原样树逐字节
python3 tools/gen-ja-dic.py && python3 tools/check-generated.py                       # ja-dic 生成 + 生成物 242 文件
python3 tools/bench.sh [--gate docs/baseline/bench.tsv]                              # 性能读数/回归门
tools/gen-android-config.sh --check-prereqs                                          # Android 配置前置
```

## 规模

Rust 629 文件 / 757,271 行；平台侧（Swift 8 / Kotlin 40 / C++ 5）60,241 行；照抄树 4,656 文件。
