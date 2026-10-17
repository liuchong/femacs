# 当前状态（2026-10-18，status 4.593 时点）

给下一次会话的**入口快照**：判定口径、读数、复跑命令、未决事项。逐轮记录仍在 `docs/status.md`，
口径与规则在 `PORTING.md`，验收细则在 `docs/acceptance.md`。

## 用户已定的方向（不再询问）

1. **逻辑一致优先**：Rust 与 C 逐函数一致；**不为性能改变逻辑**；性能未追平**不得写成达标**。
2. **性能暂挂**（DoD-7 仍记未达）。
3. **平台范围 = macOS 真机 + Android 模拟器**；Windows/Linux/Haiku 搁置。
4. **产品名不改**：`lisp/test/etc/doc/leim/admin` **逐字节原样复制**；兼容接口（函数/变量/路径/Info 节点）不变。
5. **Java 只用最新 LTS**，不得装旧 JDK 绕过。
6. 内存受控、超限即止；用完的产物及时清理（P-190）。

## 完成定义（8 条等权）— 当前 **6/8 = 75%**

| # | 项 | 判定 | 当前证据（命令） |
|---|---|---|---|
| 1 | 590 条 1:1 转写 | ✅ | `python3 tools/extract-abi.py --all` ⇒ 590/0/0 |
| 2 | 布局/符号/subr/defvar/声明/清单 | ✅ | `xtask gate-fast` ⇒ exit 0（八步） |
| 3 | L0–L5 | ✅（L1 基准待重录） | L0 ⇒ 19/0 ✓；L5 ⇒ 端口缺陷 0 ✓；**L1 ⇒ 111 文件 76 差异，golden 记录于改名后的树，待按新树重录**（P-216） |
| 4 | `.elc`/dump/生成物/loadup | ✅ | `check-generated.py` 240 文件 ⇒ **191 identical / 49 name-only / 0 DIFFERENT**（照抄树恢复逐字节后更接近）；`check-loadup.py` ⇒ 两树逐文件一致 |
| 5 | 原样树逐字节 | ✅ | `python3 tools/check-verbatim.py` ⇒ 4,604 文件 / 0 failures |
| 6 | 平台（macOS + Android） | ⚠️ 部分 | macOS `tools/smoke.sh` 11/11；Android `xtask gate-android` ⇒ 181,486 B APK / 112 class / 签名通过 |
| 7 | 性能 | ⏸ 暂挂（记未达） | `tools/bench.sh`（同臂读数见 P-193）；`--gate` 回归门默认绿 |
| 8 | 提交签名 + 台账 | ✅ | 全 `-S` 推送；`docs/ports/` 15,011 勾选 / 0 未勾 |

## 平台面的两条外部前置（等用户决定，均不涉及旧 JDK）

- **`makeinfo`（texinfo）**：`configure` 必需（`emacs/configure.ac` 14231-14235）。缺它 ⇒ 无法产出
  `docs/baseline/config/android-arm64.json` ⇒ 无法编 `libemacs.so`。
  可选：**(A)** 安装 texinfo；**(B)** 允许在只读参照树放零字节 `emacs/info/emacs` 桩。
- **arm64-v8a 系统镜像**：本机只有 x86 镜像，Apple Silicon 上 `emulator` 报
  `Cannot find AVD system path`。可选：**(C)** 安装 arm64 镜像（`tools/try-android-emulator.sh` 已就绪）。

已解决、**无需旧 JDK** 的 Java 障碍：`configure` 的两处 `-source 1.7` 探测用缓存变量
`emacs_cv_working_javac=yes` / `emacs_cv_android_x_or_later=yes` 跳过（P-207/P-213）。

## 一条命令复跑

```sh
# 已授权平台（macOS + Android 结构）一条命令，失败即停：
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime \
    --memory-mib 1400 -- bash tools/check-authorized-platforms.sh

cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- gate-fast      # 静态/链接/布局/声明/清单
cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- gate-android   # Android 结构验收（~1 GB 峰值）
bash tools/smoke.sh                                                                  # macOS 冒烟
python3 tools/check-verbatim.py                                                      # 原样树逐字节
python3 tools/bench.sh [--gate docs/baseline/bench.tsv]                              # 性能读数/回归门
tools/gen-android-config.sh --check-prereqs                                          # Android 配置前置
```

## 规模

Rust 629 文件 / 757,271 行；平台侧（Swift 8 / Kotlin 40 / C++ 5）60,241 行；照抄树 4,656 文件。
