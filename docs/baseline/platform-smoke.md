# 平台冒烟矩阵（DoD-6 的记账）

本文件记录 **本机实际具备的工具链**、**每个平台能跑到哪一级**、以及**缺什么**。
它是 DoD-6 的读数入口；每轮冒烟后只更新这里的"读数"列，不另开文件。

## 1 本机工具链（实测，2026-10-17）

| 工具 | 路径 / 结果 |
| --- | --- |
| `cc` / `clang` | `/usr/bin/cc`、`/usr/bin/clang` ✓ |
| `rustc` / `cargo` | `~/.cargo/bin/rustc`、`~/.cargo/bin/cargo` ✓ |
| `swiftc` | `/usr/bin/swiftc` ✓ |
| `java` / `javac` | `/usr/bin/java`、`/usr/bin/javac` ✓ |
| `kotlinc` | **缺失** ✗ |
| Android NDK | `~/Library/Android/sdk/ndk/{25.1.8937393,25.2.9519653,26.1.10909125}` ✓ |
| `x86_64-w64-mingw32-gcc`（Windows） | **缺失** ✗ |
| `x86_64-linux-gnu-gcc` / `aarch64-linux-gnu-gcc` | **缺失** ✗ |
| Haiku 交叉工具链 | **缺失** ✗ |

## 2 平台 × 级别（L0–L4 = 见 `docs/goal.md`；本节只记"能跑到哪"）

| 平台 | 现在能做的 | 缺什么 | 读数 |
| --- | --- | --- | --- |
| macOS（本机） | 全部：`gate-fast`、`gate-load`、`dump`、L0–L5、性能基准 | — | ✓ 已在本机持续跑（`docs/status.md` 各轮） |
| Android | NDK 编 `aarch64` 的 `temacs`（`.so`/可执行 ✓）；Kotlin 侧需 `kotlinc` 或 Gradle 分发包 | `kotlinc` ✗ | 未开始 |
| Windows | 文本级：`nt/`、`w32*.rs` 的镜像与翻臂编译 | mingw 交叉工具链 ✗ | 未开始（按 goal §9 记"外部资源"） |
| Linux | 文本级：`exec/`、`lib-src/` 的镜像 | linux 交叉 `gcc` ✗ | 未开始（同上） |
| Haiku | 文本级：`haiku*.rs` 的镜像 | Haiku 交叉工具链 ✗ | 未开始（同上） |

## 3 DoD-6 的推进顺序（按本机能做的先做）

1. **macOS 冒烟脚本化**：把 L0–L4 阶梯 + 一条 Lisp 冒烟收进一个命令（`tools/smoke.sh macos`），
   让"平台冒烟"变成可重复的一行读数，而不是散在 status 里。
2. **Android**：装 `kotlinc`（或 Gradle wrapper）后，用 NDK 26 编 `aarch64` 的 `temacs`，
   跑同一条 Lisp 冒烟；Kotlin 单元用 `kotlinc` 编 + `java` 跑单元级冒烟。
3. **Windows / Linux / Haiku**：工具链在本机不存在 ⇒ 按 `docs/goal.md` §9 第 2 条**如实记为
   "需要本机不具备的外部资源"**，并在本节写明缺哪个包；不得用"文本级翻臂编译"冒充平台冒烟。
