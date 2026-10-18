# 当前状态（2026-10-18，status 4.682 时点）

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
| 6 | 平台（macOS + Android） | ⚠️ 部分 | `bash tools/check-authorized-platforms.sh` | macOS smoke **11/0**；`gate-android` **181,486 B / 112 class / 签名**；`libemacs.so` **5,944,664 B / 69 个 JNI 导出**；含 `lib/arm64-v8a/libemacs.so` 的**签名 APK 3.2 MB**；**Android 单元 sweep 91/91 + 0 unit failure**（4.682，本树；清单用绝对路径）—— 四段 exit 0、127 s / 峰值 970 MB。**模拟器冒烟未达**：`tools/try-android-emulator.sh --plan` 是它要跑的步骤，等待 E1/E2/E3 |
| 7 | 性能 | ⏸ 暂挂（记未达） | `tools/bench.sh` | 同臂 0.70–0.86×（P-193）；按用户决定不追平、不记为达标 |
| 8 | 提交签名 + 台账 | ✅ | `python3 tools/check-ledger-ticks.py`、`git log --show-signature` | **238 台账绑定 / 0 failure / 0 行无台账**（4.679）；全部提交 `-S` 并推送 |

**计数口径（一句说清）**：8 条里 `✅` 的是 **1、2、3、4、5、8** 六条；第 6 条是 `⚠️ 部分`、
第 7 条是 `⏸ 记未达`，两条都不计入达成 ⇒ **6/8 = 75%**。此前我写过 7/8（87.5%），那是把第 6 条
的"Android 结构面全绿"误当成整条达成——**已更正**，判定不变：第 6 条要等模拟器冒烟（E1/E2/E3），
第 7 条按用户决定暂挂。8/8 的路径因此是"代码已完成 + 用户做出预算决定（E1 或 E3）"。

**第 2 条的历史要记清**：4.681 的账面上把它写成 `✅`，但同一轮的记录里 `gate-fast` 是**红**的（`spikes/haiku_inc_probe.rs`）⇒
按"当前树上的可复跑证据"严格读，那一刻是 **5/8 = 62.5%**；4.682 把红门修回绿（本树 exit 0）⇒ **6/8 = 75%**。计数规则不变：部分达成记 0。

## 已知红（不属完成定义八条，但每轮要知道）

- **`xtask gate-probes` 红，且已连续修掉两条前置原因（4.683 实测）**：`gate-probes` 比 `gate-fast` 多跑
  `spikes/*-probe.sh`（含 `lisp-abi-probe`）与 `xtask dead-arms`。本轮修掉的是：① `spikes/lisp-abi-probe.sh` 的
  `plist_get`/`get_keysym_name` 注解不是生成器要的规范形（已改）；② 生成器不读 `#[cfg]`，于是把冻结配置里**关着的**声明
  （`mark_fringe_data`、`module_finalize_function`）也交给 clang 去解析（已改为按 `docs/baseline/config/darwin-arm64.json`
  过滤，并把跳过的名字逐条打印）；③ 三个 P-05 端口侧名字（`sa_stack_alloc`/`sa_stack_mark`/`sa_stack_release_to`）
  进了 `SKIP` 表，各带理由。`lisp-abi-probe` 现在通过（182 条）。
  **剩下的红是 `dead-arms`，卡在 `src/comp.rs`**：翻转 `have_native_comp` 后，该臂报 **81 个 `E0425`（找不到类型）**
  —— `gcc_jit_type`×30、`gcc_jit_function`×19、`gcc_jit_field`×12、`gcc_jit_rvalue`×8、`gcc_jit_struct`×5、
  `gcc_jit_lvalue`×5、`gcc_jit_context`×1、`gcc_jit_block`×1，即 C 31 `#include <libgccjit.h>` 的类型一个都没声明
  （P-88 要求这种第三方头的声明**写在单元自己里**，`decompress.rs`/`lcms.rs` 是已按此办理的两个先例）。`docs/ports/comp.md`
  已把这条记录为 arm 的 `text-only` 读数（“83 name-resolution errors”），所以它是**已记录、未修**。
  `libgccjit.h` 本机**在**：`/opt/homebrew/include/libgccjit.h`（`brew --prefix libgccjit` = `/opt/homebrew/opt/libgccjit`）
  ⇒ 这条可以照 P-88 用真头量布局后补声明，不需要外部依赖。
- **`docs/ports/*.md` 里有 81 个账本写着 `not transcribed`**（4.683 实测，195 行）：绝大多数是**任何配置都不编译的臂**
  （Windows/Haiku/X11/NS 等），按 P-86 clause 3 记成 `text-only`；**最大的一处是 `src/comp.c`**：`comp.rs` 只到
  **slice 1 = C 25-822**，**slice 2（C 824-5857，约 4,754 行）尚未转写**（`docs/ports/comp.md` 的战表已记），
  `src/xdisp.c` 另有一处 C 15616 的洞（`docs/ports/xdisp.md` 4307 明说“5% 容差因此收不了口”）。
  ⇒ `extract-abi.py --all` 的 `590 done / 0 failing` 是**名字/清单口径**，它覆盖不到“每一行 C 文本都已转写”。
  严格按“全部 C 源码逐行复刻”读，DoD-1 是 **⚠️ 部分**；按 P-86 的容差记录读，DoD-1 是 ✅。**这条口径属于政策选择，留给用户定。**

- **`tools/check-authorized-platforms.sh` 的静默失败已修**（4.682）：它在子 shell 里 `cd femacs` 之后用相对路径调 `tools/check-all-units.sh`，
  这一段一直失败而 `tail -1` 把它咽掉；改用 `"$REPO/tools/..."` 之后四段真的都跑了（exit 0）。同类脚本若 `cd` 之后仍用相对路径，都要按这条检查。

## 平台面的现状（两条前置皆已解决；一条新问题等用户决定）

- **texinfo/makeinfo：已装（7.3）** ⇒ Android `configure` 第一次跑到底 ✓：
  `docs/baseline/config/android-arm64.json`（32 宏，与 darwin 只差 3 处）✓。四个坑与规则见 P-223。
- **arm64-v8a 系统镜像：已装**（`system-images;android-36;default;arm64-v8a`）+ 已建 AVD
  `femacs-arm64-api36` ✓；`tools/try-android-emulator.sh` 已可调客人内存/GPU ✓。
- **新问题（等用户决定，P-224）**：受控执行器的 **runtime 档上限 2 GiB**，而它量的是"树内 RSS 之和"
  ⇒ 模拟器（QEMU 家族，该机固定开销 ~2.0 GB）**连跑两次都被上限杀掉** ✗，缩小客人内存无效 ✗。
  可选项见 `docs/decisions-needed.md`：**(E1)** 用户明确授权一次模拟器运行的内存额度（需要改共享策略，
  我没权限自行改）✓；**(E2)** 接受"本机不跑模拟器"，Android 验收停在"结构 + 签名 APK + 原生库"✓；
  **(E3)** 先修共享执行器的记账（把"和"改成唯一 RSS/最大单进程 ✓，属共享策略改动，需用户拍板）✓。
- **Android 原生库（不依赖模拟器）**：交叉目标已装 **到钉住的 nightly 上**（P-04 的 `nightly-2026-10-01`）✓、冻结配置已产出 ✓、编译集已冻结（**91 个 `src/*.c`**，P-226 ✓）、**按-target 构建已全绿：android 91/91、darwin 83/83，0 failure** ✓✓（P-228：4 个缺陷全修 —— `signed char` 必须是 `i8` 而不是平台相关的 `c_char` ✓、`HAVE_SIGNBIT` 改为按目标测量 ✓）。**`.so` 已能链**（`xtask link-shared` ⇒ `libemacs.so` 755,048 B、AArch64 ✓），但**导出符号几乎为空** ✗ —— 配置宏 → `--cfg feature` 的映射只覆盖 darwin 的 32 个键、`HAVE_ANDROID` 不在其内 ⇒ Android 的门（`have_android` 2,316 处 ✓）全是关的 ✗（P-229）。***映射已建成** ✓（`--cfg` 按目标发出 ✓，874 键 / 334 真 ✓；并从**内层** `config.h.android` 取值 ✓ 以免记成 stubify ✗）⇒ **打开门后 `src/android.c` 报三类真缺陷** ✓（`Copy` ✗、`Clone` ✗、`thread_state` 的尺寸/偏移断言 ✗）⇒ 下一步是按这三类清单修 Android 臂 ✓ → 重编 → 断言 JNI 导出 ✓ → `android-emacs.rs`（目前空单元 ✗）→ 打包含 `.so` 的 APK ✓；并按 P-222 在这一版上**再跑一次常态 L5** ✓（正在跑的那次是改动前的 ✓）。

已解决、**无需旧 JDK** 的 Java 障碍（P-213/P-217）：① `configure` 的两处 `-source 1.7` 探测改用预置缓存变量
`emacs_cv_working_javac=yes` / `emacs_cv_android_x_or_later=yes`；② 在 **JDK 25** 下跑整条 Android 链时唯一的真实错误是
`d8` 拒绝 `javac` 默认 target 的类文件（`Unsupported class file major version 69`）⇒ 给生成的 `R` 类钉 `--release 17`
（Android 构建的常规做法）即绿：181,486 B 签名 APK、112 class；与 JBR 21 的 `classes.dex`/`resources.arsc` 哈希相同 ✓。
默认 JDK 解析：`openjdk@25` → Studio 的 JBR（`JAVA_HOME` 可覆盖）。

## 一条命令复跑

```sh
# 已授权平台（macOS + Android 结构 + 原生库 + 带 .so 的签名 APK）一条命令，失败即停：
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime \
    --memory-mib 1400 -- bash tools/check-authorized-platforms.sh

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
