# 当前状态（2026-10-18，status 4.593 时点）

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

| # | 项 | 判定 | 当前证据（命令） |
|---|---|---|---|
| 1 | 590 条 1:1 转写 | ✅ | `python3 tools/extract-abi.py --all` ⇒ **590 done / 0 in progress / 0 failing**（4.671） |
| 2 | 布局/符号/subr/defvar/声明/清单 | ✅ | `xtask gate-fast` ⇒ exit 0（八步） |
| 3 | L0–L5 | ✅（**L5 台账早于 P-195，重跑中**） | 常态 L5 的台账 `docs/baseline/ert-sweep-all.tsv` 是 4.535-4.563 的读数，**早于最后一次 Rust 改动（P-195/4.568）** ⇒ 按 P-222 需重跑（同一轮的 GC 压测门已重跑并清零唯一 `port-worse`，见 P-225）；L0 ⇒ 19/0 ✓；**L1 ⇒ 111 文件 / 0 失败 / 1 接受**（`docs/baseline/l1-accepted.tsv`：参照树自身的 `.elc` 陈旧，只接受 `newer than byte-compiled file` 这一形态）；L5 ⇒ 端口缺陷 0 ✓ |
| 4 | `.elc`/dump/生成物/loadup | ✅ | `check-generated.py` **244 文件**（新增 ja-dic 家族、`uvs.elc` 与 `jisx2131-filter`）⇒ **195 identical / 49 name-only / 0 DIFFERENT**；**`.elc` 集合已满**：参照 1,645 个 `.elc` 全有（`ja-dic.elc` 由 `tools/gen-ja-dic.py` 生成，`uvs.elc` 由端口的程序编译，两者都与参照逐字节相同）；**`src/macuvs.h` 也对上了**：`gen-unidata.py` 的 `run_uvs` 用端口的程序打印出与签约副本逐字节相同的 479,218 B（P-219）；`check-loadup.py` ⇒ 两树逐文件一致；**产物面盘点**：参照构建产物 1,973 个，端口有 **1,923**；缺的 50 个**逐条写明理由**（`docs/baseline/build-products-absent.txt`）：脚手架 23（P-102 退役）+ `info/*` 23 + `doc/*` 3（都要 `makeinfo`）+ make stamp 1；理由为 `unclassified` 的行 = 尚未推进的工作 |
| 5 | 原样树逐字节 | ✅ | `python3 tools/check-verbatim.py` ⇒ 4,604 文件 / 0 failures |
| 6 | 平台（macOS + Android） | ⚠️ 部分（**Android 单元 91/91 全绿；`.so` 69 个 JNI 导出；签名 APK 含 `.so`**，4.670；模拟器冒烟待 E1/E2/E3） | macOS `tools/smoke.sh` **11/11**（2026-10-18 在**重链+dump 后的新二进制**上复验）；Android `xtask gate-android` ⇒ 181,486 B APK / 112 class / 签名通过 |
| 7 | 性能 | ⏸ 暂挂（记未达） | `tools/bench.sh`（同臂读数见 P-193）；`--gate` 回归门默认绿 |
| 8 | 提交签名 + 台账 | ✅ | 全 `-S` 推送；`docs/ports/` 15,011 勾选 / 0 未勾 |

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
# 已授权平台（macOS + Android 结构）一条命令，失败即停：
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
