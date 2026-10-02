# femacs 一比一复刻执行计划（巨细无遗版）

> 本文是执行手册，不是宣传材料。所有数字可由 `python3 tools/gen-port-map.py` 与
> `python3 tools/extract-abi.py` 复算。可直接粘贴的 goal 见 [goal.md](goal.md)。
>
> 基线：`emacs/` 检出 `fd7afced403`（GNU Emacs 32.0.50），git 忽略，只读参照。
> 对照表：[port-map.md](port-map.md)（生成物，逐文件）。

---

## 0. 结论先行

- **路线**：一次性全量机械移植（Bun 路线），不做增量混合链接，不做"先跑起来再补"。
- **规模**：需要复刻的 C 源 **590 文件 / 678,320 行**；原样复制 **4,656 文件 / 4,606,879 行**（其中 Emacs Lisp 1,596 个 `.el`）；非 C 原生代码 → 新技术（Swift/Kotlin/C++）**53 文件 / 49,759 行**；构建脚手架由 Cargo/xtask 取代。
- **验收**：Emacs 自带测试套件（ERT 621 文件 / 171,791 行）全绿 + 加载阶梯 L5 +
  `.elc`/dump/模块 双向互换 + subr 清单逐项一致 + 生成物逐字节一致。
- **度量**：进度 = 已复刻文件数 / 590；质量 = `extract-abi.py --check` 通过率、ERT 通过率、加载阶梯。

## 1. 方法论来源（同类项目的真实经验，直接借用其机制）

| 项目 | 做法 | 我们借用什么 |
| --- | --- | --- |
| **Bun**（Zig→Rust，2026，535,496 行 Zig，11 天，6,502 次提交，峰值 695 提交/小时） | 一次性全量机械移植；先写 `PORTING.md` + `LIFETIMES.tsv`；先试跑 3 个文件再全量；1 实现 + 2 对抗评审 + 1 应用；`cargo check` 错误当工作队列（约 16,000 个错误，64 个 agent，4 个 worktree）；用原测试套件验收；禁止 `git stash`/`reset`；事后 11 轮安全评审 + 7×24 fuzz | 全套流程骨架：预备文档、试跑、分片并行、评审角色分离、错误队列、测试即验收、git 纪律。**"一次性优于增量"**正是本项目的选择依据 |
| **fish**（C++→Rust，2023-01→2024-01，1,155 文件，+110,247/−88,941 行，7 人主力） | 逐组件移植 + `autocxx` 互操作，全程保持测试可跑；"port，不是 rewrite"；真正的坑在"第二个 90%"（验证期）；教训：`.unwrap()` 直接崩、本地化/格式化字符串、可移植性探测、遗留 C 风格（fd 而不是 File） | 教训清单直接变成本项目的禁令；"验证期比移植期更贵" → 预留验证预算；保留 C 风格（非惯用 Rust）是被接受的 |
| **uutils/coreutils** | 目标是与 GNU 完全一致："与 GNU 有差异即视为 bug"，比对 stdout 与退出码；用 GNU 自己的测试套件持续打分 | 验收口径：**以原实现的行为为准，差异即 bug**；必须有持续通过率指标 |
| **Microsoft typescript-go**（TS→Go，1:1 port） | 专用 staging 仓库；逐特性 parity 表（"与 TS 6.0 完全相同的语法错误/错误位置与消息"） | 每个子系统一张 parity 表；"同名同消息"级别的对齐要求 |
| **DARPA TRACTOR** | 用静态分析 + 动态分析 + LLM 自动把遗留 C 译成 Rust，目标是"达到熟练 Rust 开发者手写的质量"；独立第三方测试评估 | 我们的"机械首过 + 规范化"就是这条路线的工程化版本；必须有独立于实现者的验收 |
| **Prossimo / memorysafety.org**（sudo-rs、ntpd-rs、libz-rs 等） | 用**原项目的 conformance 测试套件**验证 Rust 实现；重点系统做差分测试 | 差分测试与 conformance 优先；先做"能判定对错"的测试基建 |
| **GitHub Blackbird**（代码搜索，Rust 从零实现） | 关键组件用 Rust 重写，性能与正确性同指标 | 参考其对照基线做法的思路（性能与行为同时回归） |
| **Dropbox / Discord / Figma / Cloudflare Pingora / Turbopack** | 全部是**增量 + 互操作**（因为不能停机、有海量用户） | 反面对照：增量路线的前提是"线上有用户"；本项目尚无实现与用户，故一次性是对的 |

**来源（本轮实际核对过）**

- Bun：`https://bun.sh/blog/bun-in-rust`（"Rewriting Bun in Rust — Why & how"，2026-07-08），
  以及 `https://bun.sh/blog` 上 "Bun 1.4 rewrites Bun in Rust"（2026-08-20）。
- fish：`https://fishshell.com/blog/rustport/`（"Fish 4.0: The Fish Of Theseus"，2024-12-28）。
- uutils/coreutils：`https://github.com/uutils/coreutils`（README 的目标与验收口径）。
- Microsoft typescript-go：`https://github.com/microsoft/typescript-go`（1:1 原生移植的 parity 表）。
- DARPA TRACTOR：`https://www.darpa.mil/research/programs/translating-all-c-to-rust`。
- 内存安全生态（sudo-rs、ntpd-rs、libz-rs 等）：`https://www.memorysafety.org/`。
- GitHub 的 Rust 实践：`https://github.blog/2023-02-06-the-technology-behind-githubs-new-code-search/`
  （代码搜索引擎 Blackbird 用 Rust 从零实现）。注意：`gh` CLI 本身仍是 Go，本轮未能核实
  "`gh` 重写为 Rust"的说法；若确有该消息，请给出链接，我据此再补一条。
- 增量 + 互操作路线（反面对照）：Dropbox 同步引擎、Discord Read States、Figma 多人引擎、
  Cloudflare Pingora、Vercel Turbopack/Rolldown；它们的前提都是"线上不能停"。

**从这些项目提炼出的 8 条硬机制**（本计划逐条落地）：

1. 先写映射指南（`PORTING.md`）与生命周期决策表（`LIFETIMES.tsv`），再动代码。
2. 先试跑 3 个文件，校准指南，再全量。
3. 一次性全量移植 > 增量混合（本项目无上线包袱）。
4. 每个文件：1 实现 + 2 对抗评审（只给 diff，不给实现者推理）+ 1 应用。
5. 编译错误当工作队列，而不是"想一想哪里错了"。
6. 验收用原实现的测试套件（Emacs 自带的 ERT 就是我们的语言无关验收器）。
7. 严禁 stub/占位/跳测/吞错；发现流程缺陷就改流程，而不是手改单点。
8. git 纪律：单文件提交、禁止 `stash`/`reset`/`rebase`/`checkout --`、禁止改写参考树。

## 2. 复刻规格：12 个"完全一致"维度

每一条都给出**定义**、**判定手段**、**失败处理**。判定手段全部可脚本化。

### D1 代码树结构与文件名（目录级）
- 定义：`emacs/<path>` → `femacs/<path>`，仅扩展名与目录归属变化：`src/NAME.c`→`femacs/src/NAME.rs`；`src/NAME.h`→`femacs/src/include/NAME.rs`；`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`msdos/` 原目录名照搬。
- 判定：`tools/gen-port-map.py` 的 1:1 Rust 表必须 590 条、无空缺；`find femacs -name '*.rs' | wc -l` 与之对齐。
- 失败：缺文件 = 未完成；多文件 = 评审拒绝。

### D2 行与语句（行数级）
- 定义：定义顺序与 C 文件一致；一条 C 语句对应一条 Rust 语句；行数允许因语言差异浮动，但必须记录。
- 判定：`extract-abi.py --check` 报行数偏差；偏差 >10% 或 >20 行需要在 `docs/ports/NAME.md` 备注原因。
- 失败：无法解释的偏差 = 评审拒绝。

### D3 符号：函数
- 定义：C 里定义的每个函数在 Rust 里有同名函数；`static` 保持 `static`（内部链接）；导出符号用 `#[no_mangle] pub unsafe extern "C" fn`。
- 判定：`extract-abi.py` 的 `Functions defined` 集合差异必须为空；链接期 `nm -g` 对比导出符号集。
- 失败：`missing function` / `invented function` 都是失败。

### D4 符号：subr（DEFUN）与 Lisp 接口
- 定义：每个 `DEFUN ("name", Fname, Sname, min, max, n, doc)` 生成同名 Rust 注册项，参数上下限、interactive 规格、docstring 逐字一致。
- 判定：启动后导出完整 subr 表（名称、参数上下限、docstring 摘要）与参照二进制逐项 diff；`extract-abi.py` 的 `Subrs` 集合必须一致。
- 失败：任何名称/docstring 差异 = bug。

### D5 符号：宏、常量、枚举
- 定义：C `#define` 带代码 → 同名 `macro_rules!`；带常量 → 同名 Rust 常量；枚举成员名逐字一致（含前缀）。
- 判定：`extract-abi.py` 的 `Macros defined` 集合一致；常量值抽样比对（编译期 `const` 断言）。
- 失败：改名/换值 = 失败。

### D6 数据结构与内存布局
- 定义：`struct`/`union`/`enum`/位域/数组/对齐全部 `#[repr(C)]` 同序同宽；tagged pointer、GC 位、`USE_LSB_TAG`、填充与偏移逐位一致。
- 判定：C 侧 `offsetof`+`sizeof`（由脚本生成的 `layout_assert.c` 打印）与 Rust 侧 `offset_of!`+`size_of` 编译期断言双向对比，CI 全绿。
- 失败：任何偏移/大小差异 = 阻断，不允许"改成更 Rust 的表达"。

### D7 引用关系与调用图
- 定义：函数的 callee 集合与 C 一致（允许调用形态不同：宏展开、闭包、内联）；不新增跨层调用；不消除中间层。
- 判定：用 `ctags`/`extract-abi.py` 为两侧各自抽取"本文件调用的外部符号"集合，做集合 diff；差异需在 `docs/ports/NAME.md` 说明。
- 失败：悄悄换实现路径（例如绕过 `redisplay` 直接写屏）= 评审拒绝。

### D8 接口（头文件镜像、ABI、模块、命令行）
- 定义：`src/NAME.h` 的声明集合（类型、宏、extern、inline 函数）在 `femacs/src/include/NAME.rs` 里有同名对应；`emacs-module.h`/`module-env-*.h` 的 ABI 结构逐字一致；命令行开关集合与参照一致。
- 判定：头文件声明的符号集合 diff 为空；模块 ABI 结构大小/偏移断言；`--help`/`--version` 行为对拍。
- 失败：接口缺失 = 阻断。

### D9 逻辑结构（控制流与错误模型）
- 定义：`switch`→`match`；`goto` 清理路径→显式标签模拟（loop+match state，禁止改成 `?`）；`sigsetjmp/siglongjmp` → libc 同名调用；`record_unwind_protect` 语义保留；`panic` 不得跨 FFI。
- 判定：评审逐函数核对控制流等价；用故障注入（内存不足、EINTR、信号）对拍错误路径。
- 失败：把 C 的错误返回码改成 `Result` 传播 = 评审拒绝。

### D10 内存与 GC 语义
- 定义：所有 Lisp 对象走 Emacs 自己的分配器；保守栈扫描必须能扫到 Rust 侧栈上的 `Lisp_Object`；`staticpro`、`gcpro`、`GCPRO`、`mark_object` 等价物齐全；不安装 Rust 全局分配器（`#[global_allocator]` 禁用）。
- 判定：`(garbage-collect)` 前后对象计数与参照一致；强制 GC 压测（`(setq gc-cons-threshold 1)` 跑测试套件）；内存增长曲线对拍。
- 失败：泄漏/OOM/对象被错误回收 = 阻断。

### D11 字符串、编码与字符
- 定义：内部编码、char table、charset、coding system 表全部与参照逐项一致。
- 判定：文件读写往返对拍（含 CRLF/UTF-8/UTF-16/ISO-2022/日语编码样本）；`char-charset`、`charset-plist` 输出对拍。
- 失败：任何编码表差异 = bug。

### D12 产物与兼容点
- 定义：`.elc`（字节码版本与编码 cookie）、dump 文件格式、动态模块（`.so`/`.dylib`）、`DOC`（docstring 索引）、`etc/DOC*` 必须双向可加载/可用。
- 判定：GNU Emacs 产出的 `.elc`/`.pdmp` 在 femacs 可用，反之亦然；用参照编译的模块能被 femacs 加载；`admin/unidata`、`admin/charsets` 生成物逐字节一致。
- 失败：任一方向不通用 = 阻断。

## 3. 预备阶段（批次 A：写代码之前，先花时间把风险卖掉）

| 任务 | 交付物 | 完成判据 |
| --- | --- | --- |
| A1 映射指南 | `PORTING.md`：C 惯用法 → Rust 写法对照表（宏、位域、union、函数指针+context、变参、`alloca`、`goto`、`setjmp/longjmp`、signal、线程局部、tagged pointer、GC 宏、属性/对齐、内联汇编、`__builtin_*`、computed goto、字符串字面量拼接、K&R 老风格函数定义、条件编译变体） | 每条给出一个真实 C 片段与对应 Rust 片段；评审（2 名对抗评审）通过 |
| A2 生命周期/所有权表 | `LIFETIMES.tsv`：每个"复杂生命周期"字段/变量一行（对象、谁释放、是否 GC 管理、是否跨 setjmp、是否进 dump、是否需要 `staticpro`） | 覆盖 `src/*.h` 全部结构体字段与 `src/*.c` 中所有 `static`/全局对象 |
| A3 清单工具 | `tools/extract-abi.py`（已有）：逐文件输出 函数/DEFUN/宏/类型/extern/全局 + `--check` 对比 Rust 侧 | 在 3 个样本文件上无假阳性、无假阴性 |
| A4 布局工具 | 生成 `layout_assert.c`（C 侧 offsetof/sizeof 打印）与 Rust 侧 `const` 断言；CI 双向跑 | 双方输出一致 |
| A5 符号工具 | 导出参照二进制的 `nm -g` 与 subr 表，作为长期基线快照提交到 `docs/baseline/` | 可复算 |
| A6 试跑 3 个文件 | `src/doprnt.c`（纯算法）、`src/sort.c`（数据结构 + DEFUN）、`src/casetab.c`（表 + 结构体）三种形态的完整闭环 | 3 个文件全部通过 D1–D9 判定，并据此修订 `PORTING.md` |
| A7 三个专项 spike | ① 布局：`Lisp_Object`/`Lisp_String`/GC 头/`struct buffer` 逐位一致；② 长跳：`setjmp`↔`longjmp` 跨 Rust 帧无 `Drop` 泄漏；③ 保守栈扫描能标记 Rust 栈帧上的对象 | 三项各有可复现的判定脚本与结论记录 |
| A8 构建骨架 | Cargo workspace + `xtask`（loadup/DOC/dump/生成表）+ CI 两档（快：`cargo check` + 清单/布局断言；重：全量测试） | 空壳二进制可构建，CI 跑通 |

**A 阶段不许写业务移植代码**（除 A6 的 3 个文件）。

### 阶段 A 进度（2026-10-02）

| 项 | 状态 | 证据 |
| --- | --- | --- |
| A1 `PORTING.md` | 完成（v2，含评审修正） | `PORTING.md` |
| A2 `LIFETIMES.tsv` | 完成（5,562 行，0 损坏） | `LIFETIMES.tsv` |
| A3 对抗评审 | 完成并已修 P0 | `docs/reviews/2026-10-02-adversarial-review.md` |
| A4 三个 spike | 完成（布局 35/35 一致；长跳语义与 `Drop` 禁令有据；栈扫描 -O0/2/3 通过） | `docs/spikes/2026-10-02-three-spikes.md` |
| A5 构建骨架 | 完成：`femacs/` workspace + `xtask`（gen-config、layout-asserts、layout-check、build-tu、gate-fast、gate-full）+ CI 快/重两档 | `femacs/xtask/src/main.rs`、`.github/workflows/ci.yml`；`xtask gate-fast` 全绿 |
| A6 三文件试跑 | 进行中：`femacs/src/include/lisp.rs` 第一片（10 个类型，43 条布局断言通过，22 条待镜像）；`doprnt.c`/`sort.c`/`casetab.c` 未开始 | `docs/ports/lisp.md`（559 个待勾选条目） |

## 4. 移植阶段（批次 B–F）

批次只是**分片与排序**，不是"先跑起来"的增量交付；任何批次都不产生"半成品二进制"要求。

| 批次 | 分片 | 文件/行数 | 出口 |
| --- | --- | --- | --- |
| B 内核 | `src/` 解释器与内存：`lisp.h`、`alloc.c`、`gmalloc.c`、`ralloc.c`、`vm-limit.c`、`eval.c`、`lread.c`、`print.c`、`bytecode.c`、`fns.c`、`data.c`、`doc.c`、`doprnt.c`、`floatfns.c`、`sort.c`、`cmds.c`、`macros.c`、`callint.c`、`atimer.c` | 约 5.2 万行 | 清单/布局/符号三项全绿（此时仍不可运行） |
| C 文本与字符 | `buffer.c`、`insdel.c`、`marker.c`、`intervals.c`、`textprop.c`、`casefiddle.c`、`casetab.c`、`indent.c`、`syntax.c`、`search.c`、`regex-emacs.c`、`region-cache.c`、`itree.c`、`undo.c`、`editfns.c`、`character.c`、`charset.c`、`chartab.c`、`category.c`、`ccl.c`、`composite.c`、`coding.c`、`bidi.c` | 约 10 万行 | 编码/字符表对拍通过 |
| D 系统面 | `emacs.c`、`sysdep.c`、`fileio.c`、`filelock.c`、`dired.c`、`process.c`、`callproc.c`、`thread.c`、`systhread.c`、`timefns.c`、`inotify.c`、`kqueue.c`、`gfilenotify.c`、`dynlib.c`、`sound.c`、`json.c`、`sqlite.c`、`gnutls.c`、`xml.c`、`dbusbind.c`、`decompress.c`、`lcms.c`、`image.c`、`profiler.c`、`gnutls`/`tree-sitter` 绑定 | 约 12 万行 | 子进程/文件/线程测试通过 |
| E 显示与终端（最大单件） | `xdisp.c`、`dispnew.c`、`window.c`、`frame.c`、`term.c`、`terminal.c`、`termcap.c`、`terminfo.c`、`tparam.c`、`font.c`、`fontset.c`、`ftfont.c`、`ftcrfont.c`、`hbfont.c`、`sfnt.c`、`sfntfont.c`、`xfaces.c`、`fringe.c`、`cm.c`、`scroll.c`、`keyboard.c`、`keymap.c`、`minibuf.c`、`menu.c`、`widget.c`、`textconv.c`、`treesit.c`、`comp.c` | 约 20 万行 | 抓帧对拍 + GUI 冒烟 |
| F 平台与工具 | 平台后端全部 C 文件（`w32*`、`x*`、`pgtk*`、`android*`、`haiku*`、`msdos*`、`ns*` 的 C 部分）+ `lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`lib/`（gnulib）+ `java/`→Kotlin、`*.m`→Swift、`*.cc`→C++ | 约 24 万行 | 各平台冒烟 + 工具命令对拍 |
| G 收尾 | `pdumper.c`、`emacs-module.c`、`.elc` 兼容、模块 ABI、精简 unsafe、去重、性能 | 约 1 万行 | D12 全部互换通过 |

### 4.1 下一个单元 `src/data.c` 的前置镜像链（2026-10-02 实测，勿重复推导）

`data.c` 的 `#include` 是 `lisp.h`(✓) / `bignum.h`(✓) / `character.h`(✓) / `buffer.h`(✓) /
`keymap.h`(✓ 本轮) / `process.h`(✗) / `frame.h`(✗) / `keyboard.h`(✗)。按冻结配置展开活分支后，
真正被 `data.c` 的**代码**用到的是：`KBOARD`（keyboard.h 的 `struct kboard`）、
`FRAME_KBOARD`（frame.h 919，展开成 `(f)->terminal->kboard`）、`SELECTED_FRAME`（frame.h 1526，
用到 `FRAME_LIVE_P`/`XFRAME`/`emacs_abort`）、`KBOARD_OBJFWDP`（lisp.h 3155 ✓）。因此需要按下面
顺序把镜像补齐（每个文件一条一份清单、一次签名提交，走 §5 的十步）：

1. ~~`src/systime.h`~~（94 行）+ `lib/timespec.h`：**已完成**（2026-10-02 第八轮）；顺带落地 `femacs/lib/include/{timespec.rs, sys_time.rs, gnulib_config.rs}`、`config.rs` 增 `_GL_CONFIG_H_INCLUDED`、P-61/P-62。
2. `src/composite.h`（363）→ 3. `src/coding.h`（791，keyboard.h 为 `ENCODE_UTF_8`/`ENCODE_SYSTEM` 包含它）。
4. `src/dispextern.h`（4026；`termhooks.h`/`window.h` 的 `glyph_matrix`/`glyph_pool`/`image_cache`/
   `face_cache`/`text_conversion_state`）→ 5. `src/termhooks.h`（994；`struct input_event` 被
   keyboard.h 的 `union buffered_input_event` **按值**包含，必须完整）→ 6. `src/window.h`（1268）
   → 7. `src/frame.h`（1957）→ 8. `src/keyboard.h`（535）→ 9. `src/process.h`（320，只为 include
   图；`HAVE_GNUTLS` 未定义，故不含 gnutls.h）→ 10. `src/data.c`（4400，120 个 subr）。

约束：任何被声明的 `struct` 必须**字段完整**（`struct frame` 150 字段、`struct terminal` 71、
`struct window` 90、`struct kboard` 26），并在落地时把类型加进 `tools/gen-layout-fields.py` 的
`STRUCTS` 重新生成 `docs/baseline/layout/c-layout-*.tsv`（`sizeof window` 已在 pending 里，
声明它就会立刻断言 520）；`GCALIGNED` 类的 `static_assert` 按 P-60 处理。

## 5. 单文件工作流（每个文件都必须走完，缺一即未完成）

1. `python3 tools/extract-abi.py src/NAME.c > docs/ports/NAME.md`，生成待勾选清单。
2. 按 `PORTING.md`/`LIFETIMES.tsv` 逐函数移植到 `femacs/src/NAME.rs`（或 `include/`）。
3. 补齐 `#[repr(C)]` 布局与 `const` 断言（D6）。
4. 逐条勾选清单：函数、DEFUN、宏、类型、extern、全局（D3/D4/D5）。
5. 记录调用关系差异（D7），无差异则写"无"。
6. `cargo check` 直到该文件零错误；禁止 `todo!()`/`unimplemented!()`/空壳函数。
7. `python3 tools/extract-abi.py --check src/NAME.c femacs/src/NAME.rs` 必须 OK。
8. 两名对抗评审（只给 diff，不给推理）提出反驳；实现者不改口径，评审只找 bug。
9. 1 名应用者按评审意见修补；再次 `--check`。
10. 更新 `docs/port-map.md`（重跑生成器）与 `docs/ports/NAME.md` 勾选状态，签名提交
    `port(NAME): …` 并推送。

## 6. 并行编排

- **分片**：按批次 B–F 分片，每片一个 git worktree；每 worktree 内 4–16 个 agent 并行不同文件（同文件不允许并发）。
- **角色**：`implementer`（1）、`adversarial reviewers`（≥2，独立上下文，只有 diff）、`applier`（1）、`verifier`（每批次结束跑闸门，独立于实现者）。
- **禁止**：`git stash`、`git reset`、`git checkout --`、`git rebase`、改写 `emacs/`、批量 `cargo` 全局命令阻塞时长超限、跨文件"顺手重构"、改 `lisp/` 与 `test/`。
- **提交**：只提交本次明确处理的文件；消息 `port(NAME):`/`layout(NAME):`/`fix(NAME):`。
- **失败升级**：同一文件 3 轮评审未过 → 记入 `docs/ports/BLOCKED.md` 并附证据，不硬改。

## 7. 语义对齐与验收（批次 D 之后逐级点亮）

### 7.1 加载阶梯（每一级都是闸门）

| 阶梯 | 内容 | 判定 |
| --- | --- | --- |
| L0 | 自检表达式（`(+ 1 2)`、`prin1`、`condition-case`、`unwind-protect`） | 与参照逐字节一致 |
| L1 | `lisp/emacs-lisp/*.elc` 全部加载 | 无错误、无警告差异 |
| L2 | `subr.elc`+`simple.elc`+`loaddefs.elc` | 同上 |
| L3 | 完整 `loadup.el` 预加载集合 | dump 成功 |
| L4 | `make -C test check` 默认选择器 | 通过率 = 参照 |
| L5 | 含 `:expensive-test` 全量 + `--batch` 对拍脚本 | 100% 通过，0 跳过 |

### 7.2 语言无关验收（借用 uutils/Prossimo 口径）

- **以参照行为为准，差异即 bug**（不是"能跑就行"）。
- **差分对拍**：随机 Lisp 程序生成器（表达式 + 文件读写 + 进程 + 编码）双跑比对 stdout/stderr/退出码；
  参照与 femacs 各自运行，逐字节比对。
- **subr 表比对**：名称、参数上下限、interactive 规格、docstring 摘要。
- **二进制互换**：`.elc`、dump、模块双向。
- **生成物一致**：`admin/unidata`、`admin/charsets`、`ldefs-boot.el`、`src/lisp.mk` 逐字节。
- **强制 GC 压测**：`gc-cons-threshold=1` 下跑完整测试套件。
- **故障注入**：内存不足、EINTR、信号、编码异常输入。
- **性能**：启动时间、`benchmark-run` 基线、批量文本操作；性能回退超过阈值视为 bug（借用 uutils 口径）。

## 8. 完成定义（DoD）

1. `port-map.md` 中 1:1 Rust 590 条全部为 `ported`，且 `extract-abi.py --all` 零差异。
2. 布局断言、符号表、subr 表三项全绿。
3. L0–L5 全绿，ERT 通过率与参照一致，0 跳过、0 删除。
4. `.elc`/dump/模块 双向互换通过；生成物逐字节一致。
5. `lisp/` 与 `test/` 与参照逐字节一致（`diff -r` 无差异）。
6. 平台分片（macOS/Linux/Windows/Android/Haiku 中已交付者）冒烟通过。
7. 性能指标不低于参照基线（阈值见 7.2）。
8. 全部提交签名并推送；`docs/ports/*.md` 清单全部勾选。

## 9. 风险与对策

| 风险 | 影响 | 对策 |
| --- | --- | --- |
| `bytecode.c` computed goto 无 Rust 对应物 | 解释器性能/结构差异 | 专项：`match` 或内联汇编，二者都要与参照对比基准 |
| `setjmp/longjmp` 跨 Rust 帧 | 语义/资源泄漏 | A7-② spike；规则：跨 `setjmp` 路径禁止持有 `Drop` 类型 |
| 保守栈扫描看不到 Rust 栈 | 对象被误回收 | A7-③ spike；必要时显式 root/影子栈 |
| 宏体系（`DEFUN`/`gcpro`/`FOR_EACH_*`） | 大面积语义漂移 | `PORTING.md` 逐条映射 + `macro_rules!` 同名；清单工具强制 |
| 布局不可表达（位域/union/对齐） | dump 与模块 ABI 断裂 | D6 双向断言；不可表达者记偏离并阻断该文件 |
| 大规模并行导致的互相踩踏 | 仓库损坏 | git 纪律 + 单文件提交 + 禁 stash/reset（Bun 的真实教训） |
| LLM 产出的"看似合理"错误 | 静默 bug | 对抗评审（只给 diff）+ 后期 fuzz 对拍（Bun 的 bug 全是编译通过型） |
| 验证期被低估 | 发布质量 | fish 的教训：预留"第二个 90%"预算，验证期不压缩 |

## 10. 命令附录

```sh
# 规模与对照表
python3 tools/gen-port-map.py

# 单文件清单（待勾选）
python3 tools/extract-abi.py emacs/src/NAME.c

# 单文件一致性判定
python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs

# 全量一致性判定（已复刻的文件）
python3 tools/extract-abi.py --all

# 参考树完整性（不允许被改动）
git -C emacs status --short          # 必须为空
diff -r emacs/lisp femacs/lisp       # 必须无差异（原样复制校验）
```
