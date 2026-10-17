# 需要拍板的三件事（2026-10-17，status 4.540 时点）

实现层面的工作已经到底：**转写 590/590**、**L0–L5 端口缺陷 0**、**`.elc` 端口缺陷 0**、**六道门全绿**、**14,775 个清单勾选 0 未勾**。
`docs/acceptance.md` 的逐条审计显示 8 条完成定义里 **5 条达成**，剩下 3 条各自属于**政策 / 资源 / 真实差距**，都需要你选一个方向才能继续。

## 1. 性能（完成定义第 7 条"不低于参照"）

**读数**（同机、两侧 `-O2`、min-of-5）：

| 项 | 端口 | 参照 | 倍数 |
|---|---|---|---|
| 启动 | 13.8 ms | 11.5 ms | 1.20× |
| `regexp-repl` 12 KB | 4.3 ms | 2.5 ms | 1.72× |
| `buffer-scan` 8k 行 | 1.3 ms | 0.9 ms | 1.44× |
| `sort-100k` | 3.4 ms | 1.7 ms | 2.0× |
| 逐字节码操作（7 项） | — | — | **一致 1.22–1.25×** |
| 关掉两枚检查的诊断构建 | — | — | **1.13–1.41×** |

**`overflow-checks` 的代价（P-192，诊断构建实测 3 轮）**：关掉后 `regexp-repl` **1.14–1.15× → 0.97–1.08×**、`buffer-scan` **1.22–1.24× → 0.98–1.01×**（两项追平或超过参照）；`bc-loop`/`bc-sum`/`bc-car`/`sort` 前后区间重叠 ⇒ 噪声。⇒ 检查常开（P-96，以及「能 safe 尽量 safe」的要求）的代价约 **8–16%**，且**不能**解释 VM 的 1.3–1.5×。

**已经排除**：GC 次数（两侧相同）、`sort` 算法（与 C 逐行同形）、`funcall_subr`（同形；`argbuf`/`unwrap` 也改回 C 的形状后无变化）、热访问器（全部 `#[inline(always)]`）、派发状态变量（可预测分支，~0.5%），以及**安全检查**（关掉后只回收 20–35%，P-181 已把 `ub_checks` 关掉 ✓）。
**定量闭环（P-193，2026-10-17）**：把**参照自己**编成 switch 臂（`BYTE_CODE_SAFE=1`）后，`bc-loop` 由 **3.20–3.34 ms 变成 6.80–7.08 ms** ⇒ **派发形状本身值 2.0–2.1×**；端口（switch）4.95–5.45 ms ⇒ **0.70–0.79× 于同臂 C**，`bc-sum` 0.67–0.74×、`bc-car` 0.74×、`sort` 0.82–0.86×；仅 `regexp-repl` 1.04–1.11×、`buffer-scan` 1.14–1.19× 仍慢。

**结论**（P-185）：VM 那一段是**参照的构建与本端走的是同一个 C 文件的不同分支** —— `bytecode.c:37-44` 在 clang/macOS 上定义 `BYTE_CODE_THREADED`（计算跳转 ✓，`ENABLE_CHECKING` 与 `BYTE_CODE_SAFE` 都是关的 ✓、已核实 ✓），端口按 P-03 抄**非线程化**臂；Rust 稳定版没有 label-as-value（`label_break_value` 仍 unstable）⇒ **不用 nightly 关不掉**。其余五项 1.07–1.20× 是真实的小差距。

**选项**
- **A（改口径）**：把第 7 条改成"在**同等安全检查**下的可接受区间"，并把上面这张表作为基线固化。工作量≈0；代价：与参照不是同一安全等级的可比性说明要写清楚。
- **B（重写热点）**：按 C 的原形重写 `exec_byte_code` 的派发（`switch`+标签而不是 `goto` 状态机），预计回收一部分 1.2×。工作量：数天；风险：中（最热函数，需逐操作基准 + L5 全量 + `gate-fast` 三份前后对照）。
- **C（都做）**：先 B 再 A 收尾。

## 2. 产品名（完成定义第 5 条"`diff -r` 无差异"）

**现状**：`check-verbatim.py` = 6 trees / 4,604 文件 / **0 failures**，但其中 **8,427 行**是**登记过的产品名替换**（`docs/baseline/verbatim-substitutions.tsv` 逐行有理由：licence 头 5,341 行、成员声明、手册标题等）。
这条替换是**你明确要求**的（不要以参照产品名对外自述），但它与"逐字节一致"直接冲突；L5 里两个 `name-only` 用例（`calc-tests`、`erc-scenarios-base-send-message`）也是它的直接后果。

**选项**
- **A（保持）**：第 5 条按"除登记替换外逐字节一致"验收（现状）。代价：0 工作量，但 DoD 字面永远不成立。
- **B（回退 `lisp/`）**：恢复逐字节一致。代价：文档/手册里会重新出现参照产品名；需重生成受影响 `.elc`。

## 3. 平台（完成定义第 6 条"五平台冒烟"）

**现状（2026-10-17 更正）**：macOS `tools/smoke.sh` **11 check / 0 failure**；**Android 的 `kotlinc` 其实已在机器上** —— Android Studio 自带完整 CLI（`plugins/Kotlin/kotlinc/bin/kotlinc`）与 JDK 21，SDK 有 android-34/35/36 与 NDK 28.2/29.0，`tools/check-android-kotlin.sh` 可用它们**离线**编译 40 个 Kotlin 文件（该脚本尚未拿到一次完整运行：共享执行器预算被别的项目占满）；**Android 的 Kotlin 壳已零错误编译通过**（4.575：`tools/check-android-kotlin.sh` ⇒ `exit 0`、112 个 class ✓，含 `res/` 18 个资源、生成的 `R` 与 `EmacsConfig` ✓）；**APK 打包链也已跑通**（4.576：`d8` → `aapt2` 资源包 → `zipalign` → `apksigner` ⇒ `emacs.apk` 180 KB、证书校验通过 ✓）；仍缺 **原生库**（等端口的 Android 原生构建）与**真机/模拟器冒烟**。**更正（4.577）**：`rustup target list --installed` 显示 **Android 四个目标与 aarch64 Linux（gnu/musl）都已安装** ✓ —— Android/Linux 缺的不是编译器，而是 `xtask` 的**交叉构建驱动**与设备冒烟；**Windows（mingw）与 Haiku 仍未实测**（见 P-202）。

**选项**
- **A（装工具链继续）**：Android 先走（NDK 已有，只缺 `kotlinc`），其余三平台按可用性排序。需要你允许安装（体积与时间都不小）。
- **B（维持 macOS-only 交付面）**：把第 6 条按"已交付平台"读，其余记为外部资源缺失（现状）。

---

选定之后直接说"A/B/C"即可；三项互相独立，可以分开定。
