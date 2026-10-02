# femacs 一比一复刻总计划（巨无霸版）

> **本文件是 femacs 的唯一总计划**：GNU Emacs 的 C 源一比一复刻为 Rust，Emacs Lisp 原样复制。
> 配套文件：
> [port-inventory.md](port-inventory.md)（590 文件逐文件对照表，生成物）、
> [port-map.md](port-map.md)（全树映射，生成物）、
> [port-plan.md](port-plan.md)（执行手册细则：单文件十步循环、并行编排、DoD 原文）、
> [goal.md](goal.md)（可直接粘贴给 Codex 的执行级 goal）、
> [../PORTING.md](../PORTING.md)（C 惯用法 → Rust 写法对照表）、
> [../LIFETIMES.tsv](../LIFETIMES.tsv)（生命周期决策表）。
>
> 参照检出：`emacs/`（GNU Emacs 32.0.50，`fd7afced403`），**git 忽略、只读**。
> 本文件的数字全部可复算；复算命令见 §12。生成物**永不手改**。

---

## 0 一页速览

| 项 | 内容 |
| --- | --- |
| 目标 | 把参照树里 **590 个 `.c`/`.h`（678,320 行）** 一比一复刻为同名同结构的 Rust；Lisp 树原样复制 |
| 路线 | **一次性全量机械移植**（Bun 路线）。不做 C/Rust 混合链接、不做 shim、不做"先跑起来再补" |
| 允不允许 `unsafe` | 允许且必要：目标是复现 C 语义；但**不许发明 C 里没有的行为** |
| 一致性 | 12 个维度 D1–D12（文件/行数/符号/结构/引用/接口/逻辑/内存/编码/产物…），每维都有可执行判据 |
| 语言技术选型 | 能 Rust 就 Rust；平台原生的旧写法一律换新技术（Objective-C → **Swift**，Java → **Kotlin**，Haiku 的 C++ → 现代 C++） |
| 验收 | 上游测试套件（ERT）+ 加载阶梯 L0–L5 + 双向 `.elc`/dump/模块互换 + subr 表逐条一致 + 生成物逐字节一致 |
| 当前进度 | 2026-10-02：**17 done / 10 WIP / 0 failing**（27 个已有目标文件）；进度账 [port-inventory.md](port-inventory.md) |
| 度量 | 进度 = 已复刻文件数 / 590；质量 = `extract-abi.py --all` 差异数 + 闸门 + ERT 通过率 + 阶梯级别 |
| 下一步 | 收尾 `data.c` → 清 `window.h`/`frame.h`/`dispextern.h`/`termhooks.h` 尾巴 → 批次 B 剩余内核 |

---

## 1 规模实测（2026-10-02，命令见 §12.1）

### 1.1 待复刻的 C 源：590 文件 / 678,320 行

| 目录 | 文件 | 行 | 函数 | subr | 宏 | 常量 | 结构体字段 | 全局 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `src/` | 250 | 566,784 | 8,470 | 1,841 | 4,006 | 935 | 4,865 | 1,774 |
| `lib/` | 237 | 69,423 | 974 | 0 | 2,476 | 629 | 329 | 32 |
| `lib-src/` | 14 | 20,960 | 262 | 0 | 150 | 51 | 77 | 81 |
| `lwlib/` | 15 | 8,353 | 217 | 0 | 69 | 44 | 8 | 15 |
| `oldXMenu/` | 33 | 5,176 | 23 | 0 | 62 | 47 | 3 | 8 |
| `exec/` | 8 | 4,274 | 41 | 0 | 36 | 26 | 96 | 16 |
| `nt/` | 31 | 3,292 | 25 | 0 | 351 | 100 | 63 | 8 |
| `msdos/` | 2 | 58 | 0 | 0 | 6 | 2 | 0 | 0 |
| **合计** | **590** | **678,320** | **10,012** | **1,841** | **7,156** | **1,834** | **5,441** | **1,934** |

结构级总量（`docs/baseline/totals.tsv`）：函数 10,012、`DEFUN`/subr 1,841（唯一 C 名 1,736 +
唯一 Lisp 名 1,736，差值是同一实现注册多个名字的情形）、宏 7,156、常量 1,834、结构体字段
5,441、全局对象 1,934。**每一个都必须有同名对应物**（D3–D5）。

最大的 5 个文件（其余见 [port-inventory.md](port-inventory.md) §4）：
`src/xdisp.c` 39,805、`src/keyboard.c` 14,000+、`src/coding.c` 12,312、`src/xterm.c` 11,000+、
`src/emacs.c` 8,000+。**单文件最大 4 万行**，这是决定"按文件分片、绝不并发同一文件"的原因。

### 1.2 参照树里其余部分（不移植，但要做对）

| 类别 | 文件 | 行 | 处理方式 |
| --- | ---: | ---: | --- |
| ObjC（macOS 后端 `src/*.m` + `be_resources.cc`） | 9 | 26,517 | **Swift**（新技术，不用 ObjC） |
| Java（Android 应用层 `java/**`） | 40 | 15,157 | **Kotlin** |
| C++（Haiku 绑定 `src/haiku_*.cc`） | 4 | 8,085 | 现代 C++ |
| 汇编（`exec/loader-*.s`） | 6 | 1,774 | 原样保留 |
| 原样复制（Lisp/测试/数据/文档/ChangeLog） | 4,656 | 4,606,879 | **逐字节复制**，禁改一字 |
| 构建脚手架（`configure.ac`、`Makefile.in`、`m4/`…） | 324 | 63,375 | 由 Cargo workspace + `xtask` 取代 |

原样复制里必须点名的：`lisp/` **1,627 文件 / 2,106,892 行**（其中 1,596 个 `.el`）、
`test/` **1,555 文件 / 464,488 行**（704 个 `.el`，就是我们的验收套件）、
`leim/` 287,373 行、`doc/` 496,405 行、`etc/` 224,869 行、`admin/` 251,091 行。
参照树整体 ≈ **5,629 文件 / 5,400,107 行**。

### 1.3 进度（可复算，2026-10-02）

`python3 tools/extract-abi.py --all` → **checked 27 ported files: 17 done, 10 in progress, 0 failing**。

| 状态 | 文件 |
| --- | --- |
| done（17） | `src/`: lisp.h character.h itree.h buffer.h bignum.h keymap.h systime.h coding.h process.h keyboard.h sort.c casetab.c doprnt.c floatfns.c bignum.c；`lib/`: mini-gmp.h timespec.h |
| WIP（10） | `src/data.c`、`src/dispextern.h`、`src/composite.h`、`src/frame.h`、`src/termhooks.h`、`src/window.h`；`lib/limits.in.h`、`lib/stdint.in.h`、`lib/sys_types.in.h`、`lib/time.in.h` |
| 待开工 | 563 文件 |

---

## 2 方法论：同类项目的一手经验 + 我们借用什么

### 2.1 Bun（Zig → Rust，2026-07-08 起 11 天）——骨架来自它

一手材料：`https://bun.sh/blog/bun-in-rust`（Jarred Sumner，2026-07-08；本轮已实读正文）。
正文里可直接引用的机制：

- **规模与判断**："there is 535,496 lines of Zig… A rewrite in another language would take a small
  team of engineers a full year"，所以"the least risky approach… would be a **mechanical port**
  from Zig to Rust, with the **minimal number of behavioral changes**, using **the exact same test
  suite** we already use"。
- **预备文档**："Before writing any code, I spent about 3 hours talking to Claude about how to map
  patterns from our Zig codebase closely to Rust. Claude serialized this discussion into a
  `PORTING.md`"。另有一个工作流"read every struct field within every single file and trace the
  control flow… propose a lifetime… use **2 adversarial review agents**… serialize into a
  `LIFETIMES.tsv`"，然后对 `PORTING.md` 与 `LIFETIMES.tsv` 再做一轮对抗评审。
- **试跑 3 个文件**："before asking Claude to translate all **1,448** `.zig` files to `.rs` files, I
  started with just **3**"——每个文件"**1 implementer** 写、**2 adversarial reviewers** 检查
  （行为是否与 `.zig` 一致、是否遵守 `PORTING.md`/`LIFETIMES.tsv`）、"after that, **1 fixer**
  applied any suggestions"。
- **评审角色分离**："The reviewer is a second Claude in its own context window: **it gets the diff
  and nothing else** — none of the implementer's reasoning — and is told to **find the way it's
  wrong**"。
- **编译错误当工作队列**：全部代码落地后"fixing every compiler error… crate-by-crate"，
  截图显示 "**≈16,000 errors left**"。
- **git 事故（真实教训）**：并行 2 分钟就出现"one Claude ran `git stash` before committing.
  Another ran `git stash pop`. And then `git reset HEAD --hard`. **They were stepping on each
  other!**"；修法是把纪律写进工作流："never run `git stash` or `git reset` or **any git command
  that doesn't commit a specific file at once**. **No cargo either. No slow commands at all.**"。
  （早前会话记录的 "6,502 次提交 / 峰值 695 提交每小时 / 64 agent / 4 worktree" 细节本轮未复核，
  按未验证处理。）
- **验收与兜底**：原测试套件全绿才是终点；另有 ASAN（他们改了 Zig 编译器来支持）、
  24/7 Fuzzilli fuzz、端到端内存泄漏测试；事后逐类修 bug。

**我们借用**：预备文档（`PORTING.md` + `LIFETIMES.tsv`）、3 文件试跑、1+2+1 角色、只给 diff 的
对抗评审、编译错误队列、测试即验收、**git 纪律**——本项目把"不 commit 具体文件之外的 git 命令"
进一步收紧为明确禁令（§7）。

### 2.2 其他同类项目与借用点

| 项目 | 做法 | 我们借用 |
| --- | --- | --- |
| **fish**（C++→Rust，2023-01→2024-01） | 逐组件移植 + `autocxx` 互操作，全程保持测试可跑；"port, not rewrite"；真正的成本在"第二个 90%"（验证期）；教训：`.unwrap()` 直接崩、遗留 C 风格（fd 而非 `File`）是可接受的 | 禁令第 4 条（禁吞错/禁 `unwrap()` 兜底）、验证期预算、保留 C 风格 |
| **uutils/coreutils** | 与 GNU 行为差异即 bug，比对 stdout/退出码，用 GNU 自己的测试套件持续打分 | 验收口径：**以参照行为为准，差异即 bug**；持续通过率指标 |
| **Microsoft typescript-go**（TS→Go，1:1） | 专用 staging 仓库 + 逐特性 parity 表（连错误消息、错误位置都要一样） | 每个子系统一张 parity 表；"同名同消息"级别的对齐 |
| **DARPA TRACTOR / c2rust** | 自动 C→Rust 转译：产物**忠实但布满 `unsafe`**，先保证"编译通过 + 语义等价 + 过原测试"，再谈人味重构 | 我们的"机械首过 + 闸门"就是这条路线的工程化版本：`unsafe` 忠实优先，惯用化留到最后且不得改语义 |
| **Prossimo / memorysafety.org**（sudo-rs、ntpd-rs、libz-rs） | 用**原项目的 conformance/测试套件**验证 Rust 实现；关键系统做差分测试 | 差分对拍优先；先做"能判定对错"的基建 |
| **gitoxide** | 纯 Rust 实现 Git；用 Git 自己的测试套件判定"与 C 版本一致" | "过原测试套件"是唯一硬标准 |
| **GitHub Blackbird**（代码搜索后端，Rust 从零实现） | 关键组件 Rust 重写，性能与正确性同指标 | 性能与行为同时回归 |
| **GitHub `gh` CLI** | **实测核实（2026-10-02）：仍是 Go**，不是 Rust | 不采用；见下方证据 |
| Dropbox / Discord / Figma / Cloudflare Pingora / Turbopack | 全部是**增量 + 互操作** | 反面对照：增量路线的前提是"线上有用户不能停机"；本项目尚无实现与用户，所以一次性是对的 |

`gh` 的核实命令与输出（本轮实跑）：

```sh
$ gh api repos/cli/cli --jq '{lang:.language,pushed:.pushed_at}'
{"lang":"Go","pushed":"2026-10-01T16:57:11Z"}
$ gh api repos/oven-sh/bun --jq '{lang:.language,pushed:.pushed_at}'
{"lang":"Rust","pushed":"2026-10-02T09:37:52Z"}
```

即：**Bun 仓库现在的主语言确为 Rust**；**`gh` 仍是 Go**，网传"gh 重写为 Rust"不成立。
GitHub 侧真正的 Rust 重写实践是代码搜索后端 Blackbird
（`https://github.blog/2023-02-06-the-technology-behind-githubs-new-code-search/`）。

### 2.3 从这些项目收敛出的 9 条机制（本项目已全部落地）

1. 动手前先写 `PORTING.md` + `LIFETIMES.tsv`，并对两者做对抗评审。
2. 先试跑 3 个形态不同的文件，据此修订文档（本项目：`doprnt.c` 纯算法、`sort.c` 数据结构 +
   DEFUN、`casetab.c` 表 + 结构体）。
3. 每文件：**1 实现 + 2 对抗评审（只给 diff）+ 1 应用**。
4. **编译器/清单报错当工作队列**：不"通读代码找问题"，让工具把差异吐出来。
5. **测试即验收**：上游套件（ERT）全绿 + 加载阶梯；差分对拍判定行为等价。
6. **git 纪律**：单文件提交、禁 `stash`/`reset`/`checkout --`/`rebase`/`push --force`。
7. 机械保真优先于"更 Rust"：`unsafe` 是允许的，**发明行为是禁止的**。
8. 预备"第二个 90%"：验证期（阶梯、对拍、fuzz、GC 压测）单独排期。
9. 布局/ABI 这类"看不见的一致性"要用**双向断言**（C 侧 `offsetof`/`sizeof` ↔ Rust 侧 `const` 断言）。

---

## 3 代码树与映射（树级 + 文件级）

### 3.1 全树映射

| 模式 | 文件 | 行 | 目标 |
| --- | ---: | ---: | --- |
| 1:1 Rust | 590 | 678,320 | `femacs/<same path>.rs`（`src/*.h` → `femacs/src/include/*.rs`） |
| Swift | 9 | 26,517 | `femacs/platform/macos/NAME.swift` |
| Kotlin | 40 | 15,157 | `femacs/platform/android/kotlin/NAME.kt` |
| C++ | 4 | 8,085 | `femacs/platform/haiku/NAME.cc` |
| Kept | 6 | 1,774 | 路径不变 |
| Verbatim | 4,656 | 4,606,879 | 路径不变 |
| Replaced | 324 | 63,375 | —（Cargo + `xtask`） |

### 3.2 目标路径规则（唯一实现：`tools/gen-port-map.py`）

- `src/NAME.c` → `femacs/src/NAME.rs`
- `src/NAME.h` → `femacs/src/include/NAME.rs`（头文件镜像）
- `DIR/NAME.c` → `femacs/DIR/NAME.rs`；`DIR/NAME.h` → `femacs/DIR/include/NAME.rs`
  （`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`msdos/` 原目录名照搬）
- `src/*.m` → `femacs/platform/macos/NAME.swift`；`java/**` → `femacs/platform/android/kotlin/NAME.kt`；
  `src/haiku_*.cc` → `femacs/platform/haiku/NAME.cc`

判定（D1）：`gen-port-map.py` 的 1:1 表必须 **590 条、无空缺**；`find femacs -name '*.rs'`
的 1:1 部分与之一致。缺文件 = 未完成；多文件 = 评审拒绝。**规则只写在生成器里，文档不复制规则。**

### 3.3 文件级对照数据（巨细无遗那一层）

[port-inventory.md](port-inventory.md) 是本文档的数据附录，**590 行逐文件表**，每行 11 列：

| 列 | 含义 |
| --- | --- |
| 参照源 / 行 | 参照树路径 + 行数（行数级） |
| 函数 / subr / 宏 / 常量 / 字段 / 全局 | 从 `docs/baseline/inventory.tsv` 抽取的**结构级清单计数**，逐项必须在 Rust 侧有同名对应物 |
| 批次 | 本文档 §6 的分片编号（B/C/D/E/F/G/L/H） |
| 目标 | 目标文件路径（树级/文件级） |
| 状态 | `todo` / `wip` / `done`（按 `docs/ports/*.md` 勾选 + 目标存在性） |

批次聚合（同表 §3）：B 29 文件 51,139 行、C 36/70,340、D 34/61,745、E 50/172,942、
F 192/244,303、G 12/8,428、L 237/69,423。**F 是 192 文件的大桶**，落地时按平台再切
F1 X11 / F2 w32 / F3 NS+Haiku / F4 lib-src 与其余工具程序四片。

---

## 4 "完全一致"的 12 个维度（D1–D12）

每一维都是可执行判据；**判定不通过即未完成**。以下为速查表，展开说明与命令见
[port-plan.md](port-plan.md) §2（同一套编号，勿另立编号）。

| # | 维度 | 一句话定义 | 主要判据 |
| --- | --- | --- | --- |
| D1 | 文件与树 | 同名文件、同路径结构、无缺无多 | `gen-port-map.py` + `find` 计数 |
| D2 | 行与语句 | 定义顺序与 C 一致；语句一一对应；行数偏差要解释 | `extract-abi.py --check` 的行数偏差报告 |
| D3 | 函数 | 同名、同 `static`/外部链接、同签名 | 函数名集合 diff 为空；`nm -g` 对比导出符号 |
| D4 | subr（DEFUN） | 名称/参数上下限/interactive 规格/docstring 一致 | subr 表比对（名称、min/max、doc 摘要） |
| D5 | 宏、常量、枚举 | 同名；带代码的宏 → 同名 `macro_rules!` | 宏集合 diff 为空 + `const` 断言抽样 |
| D6 | 数据结构与布局 | `#[repr(C)]`、同序同宽、位域/tag/对齐逐位一致 | C `offsetof`/`sizeof` ↔ Rust `offset_of!`/`size_of` 双向断言 |
| D7 | 引用关系与调用图 | callee 集合一致；不新增跨层调用、不消除中间层 | 两侧"本文件调用的外部符号"集合 diff，差异须在 `docs/ports/NAME.md` 说明 |
| D8 | 接口（头镜像/ABI/模块/命令行） | 头文件声明集合、模块 ABI、命令行开关一致 | 声明集合 diff；模块结构体大小/偏移断言；`--help` 对拍 |
| D9 | 逻辑结构（控制流与错误模型） | `switch`→`match`；`goto` 用显式状态模拟；`setjmp/longjmp` 用 libc 同名调用 | 评审逐函数核对 + 故障注入对拍 |
| D10 | 内存与 GC 语义 | 走 Emacs 自己的分配器；保守栈扫描可见；`staticpro`/`gcpro` 等价物齐全 | `(garbage-collect)` 计数一致；`gc-cons-threshold=1` 压测 |
| D11 | 字符串、编码与字符 | 内部编码、charset、coding system 表逐项一致 | 往返读写对拍 + `char-charset`/`charset-plist` 输出对拍 |
| D12 | 产物与兼容点 | `.elc`、dump、动态模块、`DOC`、生成表双向可用 | 双向加载/互换 + 生成物逐字节一致 |

**禁令与 D 维度的关系**：禁止 stub（`todo!()`/`unimplemented!()`/空壳）、禁止吞错、
禁止把 C 的错误码改成 `Result` 传播、禁止改 `lisp/` 与 `test/`、禁止改参照树、
禁止 `#[global_allocator]`、禁止 panic 跨 FFI（`panic = "abort"`）。

---

## 5 结构 / 引用 / 接口 / 调用的对齐细则

### 5.1 命名与链接
- 文件名、函数名、宏名、结构体名、字段名、全局名、subr 名**一律照抄**（含前缀与下划线风格）。
- C 的 `static` 函数保持内部链接；导出符号用 `#[no_mangle] pub unsafe extern "C" fn`。
- Rust 关键字冲突（`type`、`box`、`gen`、`try`…）用 `r#type` 或工具提供的 `rust_ident()`
  规则处理，**不改名**；`self`/`Self`/`crate`/`super` 直接报错人工处理。

### 5.2 类型与布局
- 一律 `#[repr(C)]`；位域用显式位运算 + `bitsN` 存储单元复现（本项目 `struct frame` 的 37 个
  标志位落在两个 4 字节单元：全置 1 时字节序列 `ff ff ff ff ff 00 00 00`）；`union` 用
  `#[repr(C)] union` 或等宽 `[u8; N]` + 访问器。
- 布局键由 `tools/gen-layout-fields.py` 从 C 编译器重放生成（`docs/baseline/layout/c-layout-*.tsv`，
  694 键；Rust 侧当前 688 条断言 / 6 pending），并有**键集棘轮** `asserted-keys.txt`（键集只增）。
- 镜像**只能命名自己声明或经自己 `include!` 拿到的类型**（P-72）；跨头引用走
  `include!` 图，按 P-69/P-70 的替身规则处理平台指针类型。

### 5.3 宏体系
- object-like 宏若在 C 里是 **l-value 位置**（可赋值/可取址），Rust 侧必须仍是可赋值位置——
  这是"布局/ABI 闸门合格但语义已变"的经典陷阱（本项目实测 16 个语义变异体全部躲过冻结闸门）。
- 函数宏 → 同名 `macro_rules!`；`DEFUN` → `DEFUN!` 宏，注册项进 subr 表。
- 宏体也参与比对：`extract-abi.py` 比对宏体的 token 包含关系；刻意不同者记
  `docs/ports/macro-body-notes.tsv`。

### 5.4 控制流与错误模型
- `switch` → `match`；`goto` 清理路径 → 显式标签 + loop/state 模拟，**不许**改成 `?` 传播；
- `sys_setjmp`/`sys_longjmp` 沿用 libc 同名调用；跨 `setjmp` 路径**禁止持有 `Drop` 类型**；
- `record_unwind_protect` 语义保留；`panic` 不得跨 FFI。

### 5.5 调用图与接口
- D7 判据：两侧各抽"本文件调用的外部符号"集合做 diff。**允许**调用形态不同（宏展开、闭包、
  内联），**禁止**悄悄换实现路径（例如绕过 `redisplay` 直接写屏）。
- D8 判据：头文件声明集合 diff 为空 + `emacs-module.h` 一族 ABI 结构逐字一致 + 命令行开关一致。
- 每个文件的移植记录里必须有一行"调用关系差异"：无差异写"无"，有差异写清单与理由。

### 5.6 GC 与生命周期
- `LIFETIMES.tsv`（5,562 行）对每个复杂生命周期字段给出：谁分配、谁释放、是否 GC 管理、
  是否跨 `setjmp`、是否进 dump、是否需要 `staticpro`。
- 保守栈扫描必须能看见 Rust 栈帧上的对象（spike 已在 -O0/2/3 通过）；
  **禁止 `#[global_allocator]`**。

---

## 6 批次与依赖阶梯（文件级排序）

### 6.1 批次

批次只是**分片与排序**，不是增量交付；任何批次都不产生半成品二进制。

| 批次 | 范围 | 文件 | 行 | 出口判据 |
| --- | --- | ---: | ---: | --- |
| A | 预备（无 C 文件） | — | — | `PORTING.md`、`LIFETIMES.tsv`、3 spike、工具链、3 文件试跑（**已完成**） |
| B | 内核：`lisp.h` 族 + `alloc/gmalloc/ralloc/vm-limit/eval/lread/print/bytecode/fns/data/doc/doprnt/floatfns/sort/bignum/cmds/macros/callint/atimer` | 29 | 51,139 | 清单/布局/符号三项全绿（此时仍不可运行） |
| C | 文本与字符：`buffer/insdel/marker/intervals/textprop/casefiddle/casetab/indent/syntax/search/regex-emacs/region-cache/itree/undo/editfns/character/charset/chartab/category/ccl/composite/coding/bidi` | 36 | 70,340 | 编码/字符表对拍通过 |
| D | 系统面：`emacs/sysdep/fileio/filelock/dired/process/callproc/thread/systhread/timefns/inotify/kqueue/gfilenotify/dynlib/sound/json/sqlite/gnutls/xml/dbusbind/decompress/lcms/image/profiler` | 34 | 61,745 | 子进程/文件/线程/编码差分对拍 |
| E | 显示与终端：`xdisp/dispnew/window/frame/term/terminal/termcap/terminfo/tparam/font/fontset/ftfont/hbfont/sfnt/xfaces/fringe/cm/scroll/keyboard/keymap/minibuf/menu/widget/textconv/treesit/comp` | 50 | 172,942 | 抓帧对拍 + 终端矩阵 + dump 前置 |
| F | 平台后端与工具程序（X11 / w32 / NS / Haiku / Android / pgtk / MS-DOS + `lib-src`/`lwlib`/`oldXMenu`/`exec`/`nt`） | 192 | 244,303 | 平台冒烟；切 F1–F4 四片 |
| G | dump 与模块：`pdumper`、`emacs-module` ABI | 12 | 8,428 | dump 双向可用 + 模块双向加载 |
| L | `lib/` 系统层（gnulib 替代） | 237 | 69,423 | 随消费者批次插入，逐镜像 `--check` |

A 阶段的实际状态（2026-10-02）：`PORTING.md` 完成、`LIFETIMES.tsv` 完成、三 spike 完成、
`femacs/` workspace + `xtask` + CI 两档完成、`lisp.h` 镜像第一片完成；3 文件试跑已完成
（`doprnt.rs`、`sort.rs`、`casetab.rs` 均 `--check` OK 并各有对抗评审记录）。

### 6.2 依赖阶梯（实测，勿重复推导）

`data.c` 的前置镜像链（每个文件一条清单、一次签名提交）：

```
systime.h ✓ → composite.h → coding.h ✓ → dispextern.h → termhooks.h
  → window.h → frame.h → keyboard.h ✓ → process.h ✓ → data.c
```

约束：被声明的 `struct` 必须**字段完整**；落地时把类型加进 `tools/gen-layout-fields.py`
的 `STRUCTS` 并重生成布局基线（`sizeof window` 已在 pending，声明即断言 520）；
`GCALIGNED` 类 `static_assert` 按 P-60 处理。

### 6.3 加载阶梯（每级都是闸门）

| 阶梯 | 内容 | 判定 |
| --- | --- | --- |
| L0 | 自检表达式（`(+ 1 2)`、`prin1`、`condition-case`、`unwind-protect`） | 与参照逐字节一致 |
| L1 | `lisp/emacs-lisp/*.elc` 全部加载 | 无错误、无警告差异 |
| L2 | `subr.elc` + `simple.elc` + `loaddefs.elc` | 同上 |
| L3 | 完整 `loadup.el` 预加载集合 | dump 成功 |
| L4 | `make -C test check` 默认选择器 | 通过率 = 参照 |
| L5 | 含 `:expensive-test` 全量 + `--batch` 对拍脚本 | 100% 通过，0 跳过 |

---

## 7 单文件十步循环（缺一不算完成）

1. `python3 tools/extract-abi.py emacs/src/NAME.c > docs/ports/NAME.md` 生成待勾选清单。
2. 按 `PORTING.md`/`LIFETIMES.tsv` 逐函数移植到 `femacs/src/NAME.rs`（或 `include/`）。
3. 补齐 `#[repr(C)]` 布局断言与 `const` 断言（D6）。
4. 逐条勾选清单：函数、DEFUN、宏、类型、extern、全局（D3/D4/D5）。
5. 记录调用关系差异（D7），无差异写"无"。
6. `cargo check` 到该文件零错误；禁止 stub。
7. `python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs` 必须 OK。
8. **2 名对抗评审**（独立上下文，只给 diff，任务是"找出它为什么是错的"）。
9. **1 名应用者**按评审意见修补，重跑第 7 步。
10. 重跑 `python3 tools/gen-port-map.py`（+ 收尾时 `gen-port-inventory.py`），补
    `docs/reviews/` 记录，**签名提交** `port(NAME): …` 并推送。

**并行编排**：按批次分片，每片一个 worktree；片内 4–16 个 agent 并行**不同**文件，
同一文件绝不并发。角色：0 实现者 / 2 对抗评审 / 1 应用者 / 1 verifier（批次末跑闸门）。
**禁令**：`git stash`、`git reset`、`git checkout --`、`git rebase`、`git push --force`、
改写 `emacs/`、跨文件"顺手重构"、改 `lisp/` 与 `test/`。

**失败升级**：同一文件 3 轮评审未过 → 记入 `docs/ports/BLOCKED.md` 并附证据，转下一个文件。

---

## 8 工具链、冻结基线与闸门

### 8.1 工具

| 工具 | 作用 |
| --- | --- |
| `tools/extract-abi.py` | 唯一 C 抽取器：`<c>` 出清单、`--check <c> <rs>` 单文件判定、`--all` 全量判定、`--cond-macros/--cond-macro-paths` |
| `tools/gen-port-map.py` | 全树映射（含目标路径规则） |
| `tools/gen-port-inventory.py` | 590 行逐文件对照表（本文档数据附录） |
| `tools/gen-baseline.py` / `gen-globals-baseline.py` / `gen-lifetimes.py` / `gen-layout-fields.py` | 冻结基线生成 |
| `tools/check-cond-eval.py` | 条件求值器 vs clang 差分；新镜像必须先 `--write-verified` 进白名单 |
| `femacs/xtask` | `gen-config` / `gen-globals` / `layout-asserts` / `layout-check` / `build-tu` / `gate-fast` / `gate-probes` / `dead-arms`（`gate-full` 仍是 stub） |

### 8.2 冻结基线（提交进仓库，可复算）

- `docs/baseline/inventory.tsv`（590 行）、`subrs.tsv`（1,841）、`totals.tsv`、`globals/`
- `docs/baseline/config/darwin-arm64.{json,macros.tsv}`（4,956 条宏）+ 白名单
  `cond-eval-verified.txt`（17 个文件）
- `docs/baseline/layout/c-layout-darwin-arm64.tsv`（694 键）+ `counts.tsv`（688/6）+
  键集棘轮 `asserted-keys.txt`

### 8.3 闸门命令

```sh
# 快闸：条件求值器 + 清单 + cargo check + 单元构建 + 探针 + fam/shadow/macro + 布局断言
(cd femacs && cargo run -q -p xtask -- gate-fast)
# 探针闸：inline/accessor/write/va/syshdrs/gmp-abi/lisp-abi + dead-arms 翻转
(cd femacs && cargo run -q -p xtask -- gate-probes)
# 全量清单判定
python3 tools/extract-abi.py --all
# 生成物新鲜度（CI 校验 port-map.md，不校验含状态列的 port-inventory.md）
python3 tools/gen-port-map.py && git diff --exit-code -- docs/port-map.md
# 参照树必须干净
git -C emacs status --short
```

冻结配置从零重建（新环境或清过 `build/` 后）：

```sh
mkdir -p build/tools && printf '#!/bin/sh\necho "makeinfo (GNU texinfo) 7.1"\n' > build/tools/makeinfo && chmod +x build/tools/makeinfo
mkdir -p build/emacs-ref && cd build/emacs-ref && PATH=$PWD/../../build/tools:$PATH ../../emacs/configure --without-all --with-x=no --with-ns=no
make -C lib stdbit.h byteswap.h stdint.h limits.h && make -C src globals.h
# 健康线：config.h 342 行含 #define
```

### 8.4 闸门盲区（诚实清单，别把绿灯当证明）

- `gate-fast` **不看 warning**；
- `--check` 只比 token 集合/出现次数与调用内数字：同一 token 出现多处只改一处、
  字符字面量（`'z'`→`'y'`）、纯顺序调换**抓不到**（与 P-30 的行为差分互补）；
- 镜像的 `pub type` 定义、61 条对象声明目前只计数（P-64）；
- 尚无 `src/emacs` 二进制 → **没有 ERT 级证据**；`gate-full` 仍是 stub；
- 布局/ABI 门全绿**不等于**语义正确（实测 16 个语义变异体曾全部通过冻结闸门）。
- **CI 现状（2026-10-02 实测）**：`gh run list` 显示每次 push 都是 `failure`，失败在第一步
  "Check the reference checkout is present and clean"——CI 的 checkout 里没有 `emacs/`
  （参照树被 git 忽略，且工作流用 `submodules: false`），因此快档的**清单 / 新鲜度 / 布局三步
  从未真正执行过**。要让 CI 恢复保护力，必须先给 CI 一份参照检出（submodule、显式拉取，
  或把依赖 `emacs/` 的步骤拆到带参照的 job）；在那之前 CI 绿灯是假象、红灯也不是 regresion 信号。
- **`docs/port-map.md` 的 `Target lines` 列会漂移**：工作区里有在飞文件时重新生成，会把未提交
  的行数写进产物（2026-10-02 实测提交里的版本已过期 14 行）。规则：**再生该文件时必须让工作区
  等于提交树**（在飞文件先落地，或临时移出），否则参照齐备后的新鲜度校验会红。

---

## 9 进度账与棘轮

- 进度账：`docs/port-inventory.md`（590 行，状态列）+ `docs/ports/*.md`（逐文件勾选清单）。
- 质量账：`extract-abi.py --all`（17/10/0）、`docs/baseline/layout/counts.tsv`（688 断言/6 pending）、
  `asserted-keys.txt`（键集只增）、`cond-eval-verified.txt`（白名单只增）。
- 棘轮规则：**只增不减**；任何"减少覆盖"的改动都要在 `docs/ports/NAME.md` 里写明理由，
  否则评审拒绝。
- 每轮收尾：跑闸门 → 清 `build/`、`femacs/target` → 签名提交 → 推送 →
  `gh api repos/liuchong/femacs/commits/HEAD --jq '.sha[0:7] + " verified=" + (.commit.verification.verified|tostring)'`。

---

## 10 风险登记册与已知偏离项

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| 宏的 l-value/求值语义漂移 | 静默 bug（闸门看不见） | §5.3 规则 + 行为差分 + 变异体测试 |
| `bytecode.c` computed goto | 解释器结构与性能 | 专项：`match` 或内联汇编，均需基准对拍 |
| `setjmp/longjmp` 跨 Rust 帧 | 资源泄漏/语义错 | spike 已过；规则：跨 `setjmp` 禁持 `Drop` |
| 保守栈扫描看不见 Rust 栈 | 对象被误回收 | spike 已过；必要时显式 root/影子栈 |
| 布局不可表达（位域/union/对齐） | dump 与模块 ABI 断裂 | D6 双向断言；不可表达即阻断该文件并记偏离 |
| 大规模并行互相踩踏 | 仓库损坏 | 单文件提交 + 禁令（Bun 的真实教训） |
| LLM"看似合理"的错误 | 静默 bug | 只给 diff 的对抗评审 + 后期 fuzz/差分对拍 |
| 验证期被低估 | 发布质量 | fish 的教训：验证期单独排期，不压缩 |

**已知偏离项索引**：`PORTING.md` §9 的 `P-01 … P-75`（例如 P-66 白名单 + 条件求值器、
P-69 指针类型包含图、P-70 替身规则、P-72 镜像只命名自己声明的类型、P-75 函数体比较丢死行、
P-64 `lisp-abi-probe` 盲区、P-60 还缺 10 条 `static_assert`）。**勿重复上报**，新增偏离记在
`PORTING.md`，不要写进本文档。

---

## 11 完成定义（DoD）

1. `port-map.md` 的 1:1 Rust 590 条全部 `ported`，`extract-abi.py --all` 零差异。
2. 布局断言、符号表、subr 表三项全绿（含键集棘轮）。
3. L0–L5 全绿：ERT 通过率与参照一致，**0 跳过、0 删除**。
4. `.elc`/dump/动态模块双向互换；`admin/unidata`、`admin/charsets`、`ldefs-boot.el`、
   `src/lisp.mk` 生成物逐字节一致。
5. `diff -r emacs/lisp femacs/lisp`、`test/`、`etc/`、`doc/` 无差异。
6. 已交付平台（macOS/Linux/Windows/Android/Haiku）冒烟通过。
7. 性能不低于参照基线（启动时间、`benchmark-run`、批量文本操作）。
8. 全部提交签名并推送；`docs/ports/*.md` 清单全部勾选。

---

## 12 命令附录

### 12.1 复算本文档的数字

```sh
python3 tools/gen-port-map.py            # 全树映射（590/678,320 等）；再生前先让工作区=提交树
python3 tools/gen-port-inventory.py      # 590 行逐文件表 + 批次聚合
python3 tools/extract-abi.py --all       # 17 done / 10 WIP / 0 failing
git -C emacs status --short              # 必须为空
git -C emacs log -1 --format='%H %ci'    # fd7afced403
```

### 12.2 单文件

```sh
python3 tools/extract-abi.py emacs/src/NAME.c > docs/ports/NAME.md
python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs
(cd femacs && cargo run -q -p xtask -- build-tu src/NAME.c)
```

### 12.3 每轮收尾

```sh
(cd femacs && cargo run -q -p xtask -- gate-fast)
(cd femacs && cargo run -q -p xtask -- gate-probes)
rm -rf build femacs/target
git add -A <本次文件> && git commit -S -m 'port(NAME): …' && git push origin master
gh api repos/liuchong/femacs/commits/HEAD --jq '.sha[0:7] + " verified=" + (.commit.verification.verified|tostring)'
```
