# 三个问题的结案记录（2026-10-17 提出，2026-10-18 全部有结论）

结论：① 性能 —— 暂搁置（未追平，不写成达标）；② 产品名 —— **不改名**，`lisp/test/etc/doc/leim/admin` 逐字节原样，
兼容接口（函数/变量/路径/Info 节点）不变，已执行；③ 平台 —— macOS 真机 + Android 模拟器，其余搁置。
**新增（2026-10-18）：问题 4 仍待你决定 —— DoD-1「全部复刻」对“活臂/死臂”的口径**（见文末；新证据：Android 侧有 214 条库导入无人提供，其中最大的一块正是被记成死臂的 `HAVE_WINDOW_SYSTEM` 臂）。
**两条外部前置已解决**（2026-10-18，用户授权安装）：`texinfo`/`makeinfo` 已装 ⇒ Android `configure` 跑到底、冻结配置产出
（P-223）；`arm64-v8a` 系统镜像与 AVD 已就绪 ✓。

**已作废（2026-10-18，规则更新）**：受控执行器改为按系统余量**自动扩容**（runtime 4 GiB 起、硬上限 16 GiB），模拟器（~2 GB）在其中可直接运行 ⇒ E1/E2/E3 不再需要用户决定，模拟器冒烟并入 D 阶段的普通验收。原记录保留在下方作历史。

**原待决问题：受控执行器的 runtime 上限装不下 Android 模拟器**（P-224）。实测：连续两次 `reason: memory_limit`，
峰值 ~2.0 GB（上限 1.9 GiB 已用满）；把客人内存 640→512 MiB、关 GPU 后峰值反而 2.035 GB ⇒ 主导项是 QEMU 家族的固定开销，
**改客人大小无效**；同一记账现象在 GC 压测里也量到过（P-222）。

| 选项 | 做法 | 影响 |
|---|---|---|
| **E1（需要你明确授权）** | 为这一件事授权一次更大的运行时额度（或允许我把共享执行器的 runtime 上限提到 ~3 GiB） | 能真的跑起模拟器做冒烟；代价是**改动共享策略**（不是我一个人该拍的板），且我目前**没有权限**自行抬高 |
| **E2** | 本机不跑模拟器，Android 验收停在"结构 + 签名 APK + 原生库" | 0 风险；DoD-6 的"模拟器冒烟"保持**未达**，如实记录 |
| **E3** | 先修共享执行器的记账（求和 → 唯一 RSS 或最大单进程） | 记账更准（长扫与多进程都受益）；同属共享策略改动，需你拍板 |

（我的判断：**E3 是根因**，但它是共享工具的改动；若你只想推进 femacs，**E1 + 事后补 E3** 最省事。）

以下为原始记录（含当时的选项），保留作历史。

实现层与验证层已到底：**转写 590/590**、**平台代码 53/53**、**Android 数据 19/19**、L0–L5 端口缺陷 **0**、生成物 **0 差异**、`gate-fast` 全绿、台账 **15,011 勾选 / 0 未勾**。
完成定义 8 条里 **6 条达成**（4.682 复算：第 2 条 `gate-fast` 已从 HEAD 上的红修回绿 —— 修的是 `window.rs` 的 dispextern 边，改由 `editfns`/`insdel`/`textprop` 三个单元自带，见 `docs/current-state.md`）；剩下 2 条不是"再多写代码就会自动变绿"的，它们各自需要你在下面的选项里选一个。

---

## 问题 1：性能 —— **已决定（2026-10-17）：暂时搁置**

用户决定：本期优先保证 Rust 逻辑与 C 逻辑一模一样；不为性能改变逻辑；**未追平不得写成达标**。第 7 条仍记为**未达**，附同臂读数与本机噪声区间。以下选项保留作历史记录。

### 原问题：性能"不低于参照"怎么判？（完成定义第 7 条）

**背景读数（都可复跑）**
- 参照的 macOS 构建走 `BYTE_CODE_THREADED`（计算跳转），端口按 P-03 只转录了同一个 `bytecode.c` 的 `switch` 臂。
- **把参照自己编成 switch 臂**（`BYTE_CODE_SAFE=1`）后：`bc-loop` 由 **3.2 ms 变成 7.0 ms** ⇒ **仅派发形状就值 2.0–2.1×**；此时端口 **4.95–5.45 ms**，即 **0.70–0.79× 于同臂 C**。
- 同臂下其余项：`bc-sum` 0.67–0.74×、`bc-car` 0.74×、`sort` 0.82–0.86×、`buffer-scan` 1.09–1.16×（已由 P-195 修）、`regexp-repl` 1.13–1.20×（剖析形状与参照重合，差额在 `exec_byte_code`）。
- 诊断构建把 `overflow-checks` 关掉：`regexp-repl` 0.97–1.08×、`buffer-scan` 0.98–1.01×，VM 与 `sort` 在噪声内 ⇒ 安全检查的代价约 8–16%。

**选项**

| 选项 | 做法 | 影响 |
|---|---|---|
| **A. 改为"同等构建分支下的可接受区间"** | 验收口径写成"同一派发臂内不低于参照；跨臂差异记录在案"，把上面的表固化为基线 | 0 工作量，立即满足第 7 条；代价是**口径变了**，外界无法再说"逐项不低于原版" |
| **B. 上 nightly 搬 threaded 臂** | `label-as-value`/`asm goto` 复刻计算跳转 | 能吃掉 VM 那 2×，但**引入 nightly 工具链**，且与"稳定版 Rust"这条项目约束冲突；需要重跑全部门与 L5 |
| **C. 先 B 后 A** | 先做 B 测出真实收益，再决定口径 | 时间最长，但结论最硬 |

> 我的建议：**A**（证据已足）；B/C 只在你不接受改口径时才有意义。
> 注意：**关掉 `overflow-checks` 不作为选项**（P-96 + 你"能 safe 尽量 safe"的要求），它只用来给差距定价。

---

## 问题 2：`lisp/` 等照抄树 —— **已决定（2026-10-18）：不改产品名、逐字节原样**

用户决定：产品名本阶段不改（不是优先目标）；`lisp` 与文档原样复制；兼容接口不变。已执行：
`tools/restore-verbatim.py` 回拷 2,003 个文件、登记表清空 ⇒ `check-verbatim.py` = 4,604 文件 / 0 failures ✓（DoD-5 达成）。
许可正文与版权声明保留；"8,427" 是当时**被登记的替换行数**的口径（非函数数、非改动行数），现已不在适用。

### 到底改了什么（真实路径 + 前/后）
1. **版权/许可头（注释）** — `femacs/lisp/simple.el` 第 9 行
   - 前：`;; This file is part of <the reference's product>.`
   - 后：`;; This file is part of femacs.`（**已按你的决定撤销，现为逐字节原样**）
   （第 11 行 `;; GNU Emacs is free software: …` 同理；**许可证正文与 GPL 条款一字未动**，只改"本文件属于哪个程序"这一句。）
2. **提示/文档文案（字符串或注释）** — `femacs/etc/tutorials/TUTORIAL` 第 885 行
   - 前：`applications call "windows".  The Emacs manual contains a Glossary of`
   - 后：`… The femacs manual contains a Glossary of`
3. **可执行 Lisp 里的 docstring/`:tag`/`:help-echo` 文本（字符串字面量，不是标识符）** — `femacs/lisp/cus-edit.el`
   - 前：`:tag "Emacs manual"` / `"Read the node on Easy Customization in the Emacs manual."`
   - 后：`:tag "femacs manual"` / `… in the femacs manual.`
   - 同时可见**未改**的兼容接口：`info-emacs-manual`、`info-emacs-bug` 等**函数名**，以及 Info 节点名 `"(emacs)Bugs"`、`"emacs"` 这些**路径/节点标识**都保持原样。

### "8,427" 是什么口径
`tools/check-verbatim.py` 报告的是**被登记的替换行数**（一行里只要出现产品名并被替换就计 1），跨 6 棵树 4,604 个文件；按树分布（本次实测的**差异行**，含 `.elc` 二进制）：`lisp` 13,517、`doc` 1,182、`etc` 494、`admin` 249、`leim` 11、`test` 1。它不是"改了多少个函数"，也不是"删改了多少行代码"。

### 保留 / 还原的实际影响
- **保留（现状）**：`check-verbatim` 0 failures（"除登记替换外逐字节一致"）；但 `diff -r` 字面不成立；两个 ERT 用例因文案不同呈 `name-only`。
- **还原（逐字节）**：`diff -r` 字面成立；代价是**所有 licence 头与手册/教程文案重新出现原产品名**（与你"避免法律风险"的意图相反），且需重生成受影响 `.elc`。
- **许可角度**：改名**不是**规避许可义务——GPL 正文、版权声明、修改声明都保留；改动只在"本文件属于哪个程序"这类身份句。是否保留由你定。

### 原问题：`lisp/` 等照抄树要不要做到"逐字节一致"？（完成定义第 5 条）

**现状**：`check-verbatim.py` = 6 棵树 / **4,604 文件 / 0 failures**，但其中 **8,427 行**是你要求的**产品名替换**（licence 头、成员声明、手册标题等；逐行登记在 `docs/baseline/verbatim-substitutions.tsv`）。

**选项**

| 选项 | 做法 | 影响 |
|---|---|---|
| **A. 保持现状** | 第 5 条按"除登记替换外逐字节一致"验收 | 0 工作量；但 `diff -r` 字面永远不成立，两个 ERT 用例（`calc-tests`、`erc-scenarios-base-send-message`）的 `name-only` 也源于此 |
| **B. 回退 `lisp/` 到逐字节一致** | 恢复原文件名/原字样 | 第 5 条字面成立；但**文档与 licence 头会重新出现参照产品名**（与你"不要有法律风险"的要求相反），且需重生成受影响 `.elc` |

> 我的建议：**A**（政策优先）；若你要求字面一致，选 B，但请确认接受产品名回归。

---

## 问题 3：平台 —— **已决定（2026-10-17）：macOS + Android**

用户决定：先验收 macOS（现成真机）与 Android（可用虚拟机/模拟器）；其余平台暂时搁置；无需再问范围。
**JDK 的旧结论已作废**：Android 原生库不需要 JDK ≤ 19 —— 用最新 LTS（JDK 25）+ 预置缓存变量即可（P-207/P-213/P-217）。
以下选项保留作历史记录。

### 原问题：平台交付面做到哪几层？（完成定义第 6 条）

**现状**
- macOS：`tools/smoke.sh` **11/11**（dump 版与 temacs 版都过）。
- Android：**Kotlin 41 文件零错误（112 class）**、`res/` 18 个资源 + 清单、`R` 与 `EmacsConfig` 生成、**d8 → zipalign → apksigner 产签名的 `emacs.apk`（180 KB）**；缺**原生库**（等一次 Android `configure`，已连续被机器上别的任务占满共享内存预算）与**真机/模拟器冒烟**。
- Linux：`aarch64-unknown-linux-{gnu,musl}` 目标已装，缺**按-target 的构建驱动**。
- **实测边界（4.583，只读核查）**：`zig` 在 ✓、`qemu-system-aarch64` 在 ✓，但**无容器运行时、无 Linux 镜像** ✗、mingw ✗、Haiku 工具链 ✗ ⇒ **Linux 可构建但本机不能运行**；**Windows/Haiku 连构建都缺**。
- **Android 原生库的精确卡点（4.582 实测）**：参照的 `configure` 用 `javac -source 1.7` 判定编译器可用，而本机只有 **JDK 21 / 26**，两者都拒绝 1.7 ⇒ 需要一个 **JDK ≤ 19**（17 或 11），属于安装动作，需你同意。

**选项**

| 选项 | 做法 | 影响 |
|---|---|---|
| **A. 交付面 = macOS + Android（到 APK，无设备）** | 把第 6 条读作"已交付平台的冒烟；Android 到可安装产物" | 可以在机器空闲后推进到"有原生库的 APK"；**没有设备就不算真机冒烟**，这一条仍留一个明确缺口 |
| **B. 再加 Linux（headless 冒烟）** | 用已装的 Linux 目标写交叉构建驱动 + 在容器/远端跑冒烟 | 需要一台 Linux 环境（本机没有）或允许装容器运行时 |
| **C. 五平台全做** | 追加 Windows（mingw）与 Haiku 工具链 | 安装体积与时间最大；Haiku 还需模拟器 |
| **D. 维持 macOS-only** | 第 6 条按"已交付平台"读 | 0 工作量；Android/Linux 的现有成果记为超出验收范围 |

> 我的建议：**A**；若你能提供设备/Linux 环境，可升到 B。

---

选定后请直接说"A/B/C"（三个问题互相独立，可分开定）。我会把选中结论写进 `docs/acceptance.md` 的判定列、更新本文件为结案记录，并把对应证据重跑一遍。

---

## 问题 4（新增，2026-10-18）：DoD-1「全部复刻」对**活臂**的口径 —— Android 侧已经等价于“必须写”

**背景（本次模拟器冒烟把断言提到应用层后量到的）**
- 用户授权的两个平台里，**Android 是窗口系统**（`HAVE_ANDROID` 打开 ✓），而端口的 `xdisp.rs` 等处把 C 的
  `#ifdef HAVE_WINDOW_SYSTEM` 段落**逐处写成了“dead on this target (P-08)”注释**（例：`femacs/src/xdisp.rs`
  30417/30421/30426）。这在冻结的 **darwin** 配置下是对的，在 **Android** 上却把活代码记成了死代码。
- 读数（可复跑）：`python3 tools/check-android-imports.py` ⇒ `libemacs.so` **402 个导入 / 119 个无人提供**
  （棘轮 `docs/baseline/android-imports-pending.txt`；`docs/status.md` 4.708 记了从 214 降到 119 的六步）。
  逐条落位（`tools/gen-android-arms.py` ⇒ `docs/baseline/android-arms.tsv`）后：**`HAVE_WINDOW_SYSTEM` 臂 31 条**
  （`xdisp.c` 26、`xfaces.c` 3 等）、`not_found` 66 条、`other` 20 条、平台名 2 条。
- 结论：只要这些名字无人提供，`libemacs.so` 就**不能被 `dlopen`**，Android 侧只能停在“结构 + 可链接 + 签名 APK”。

**选项**

| 选项 | 做法 | 影响 |
|---|---|---|
| **A. 维持现状规则（P-08）** | 死臂只记录，不转写 | macOS 侧 DoD-1 成立 ✓、零额外工作量；但 Android 侧那 31 条臂**永远无人提供** ⇒ “Android 能跑”不可达 ✗，DoD-6 只能记“结构面达成、应用层未达”；新门会长期报 31 条未决（另 88 条与死臂无关，A 口径下照常修）|
| **B. 无差别逐行写（含 darwin 死臂）** | 把 `HAVE_WINDOW_SYSTEM` 等臂在**所有**目标上都写出来 | 最“满”的口径；工作量最大（`xdisp.c` 一处此前量到 62 个函数），且 darwin 上这些代码永不执行、只增加维护面 |
| **C. 按目标配置决定活/死（建议）** | 给镜像加 `have_window_system`/`have_android` 之类的 cfg 门，把臂写成**条件编译**；活臂必写、死臂仍只记录 | 规则从“人工裁剪平台”变成“配置决定” ✓；macOS 侧读数不变 ✓；Android 侧的工作队列就是那份 214 条的棘轮 ✓（其中 3 条是另一类小井：darwin 头镜像名 2 条、panic handler 1 条，与臂无关，各自独立修） |

> 我的建议：**C**。它不降低验收标准（活臂照样逐行写），也不为 darwin 增加永不执行的代码。

**另需你知晓（不需要决定，属实现细节）**：`y`/`n` 之外的第三方依赖（例如 Android 上 `BC`/`UP`/`PC`/`cost` 在 darwin 由
系统的 ncurses 提供、Android 必须由树内自己的 `termcap.c` 提供）会在 C 侧一并落进同一批工作，按 C 的配置走即可。

选定后我会把结论写进 `docs/acceptance.md` 的判定列，并按口径重跑平台与 DoD-1 的读数。
