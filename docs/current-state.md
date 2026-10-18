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

- **`xtask gate-probes` 红，只剩 `dead-arms` 的 4 处（4.684 实测）**：本轮修掉的是 `lisp-abi-probe` 的两条（注解规范形 + 生成器按冻结配置求值 `#[cfg]`）
  与 `comp.rs` 的 81 条 `E0425`（按 P-88 补上 `<libgccjit.h>` 的 10 个类型，clang 量得不完整、8 字节指针）。**还剩 4 处，都是"镜像没抄 C 的声明块"**：
  `estimate_mode_line_height` ×2（C 无门声明在 `dispextern.h` 3582）、`window_box_left` ×1（`dispextern.h` 3579）、`haiku_display_info` ×1（`dispextern.h` 159 的 typedef，
  标签定义在 `include/haikuterm.rs`）。端口现在把这些名字分散写在各 unit（`androidterm.rs` 4875、`fringe.rs` 1137、`dispnew.rs` 269 …），C 却是**头里一份**。
  `gate-fast` **不包含** `dead-arms`，所以这条红只在单独跑 `gate-probes` 时现形。
- **声明块的缺口是全局的（4.684 测量，方法见 `docs/status.md` 4.684 ⑤）**：**1,032 条** C 声明不在自己的镜像里，其中 **1,006 条在树内别处声明**（名字口径的门因此看不见）、**26 条全树没有**；
  最多的是 `lisp.h` 423、`dispextern.h` 206、`xterm.h` 126、`font.h` 69、`frame.h` 56、`gtkutil.h` 49、`keyboard.h` 36。**这是测量值、不是判据**（正则口径粗、含死分支名）。
  ⇒ 下一条工作线：按 P-297 把 C 的声明块照抄进对应镜像，并把 unit 里的 use-site 副本按 P-26 收回去。

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
