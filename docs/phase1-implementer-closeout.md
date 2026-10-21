# 一期实施者结案总结（2026-10-22）

> 本文件由实施者按用户结案要求编写；仅新增本文件，不提交、不清理、不启动新 goal。
> 协调对话负责最终一期文档、合并与签名提交。
> 本稿交接后由协调对话校正模型、时间线、清理和目标生命周期事实；统计边界以一期总报告为准。

## 1. 最终结案口径

- 本期按**功能性口径结案**：转写覆盖已完成，行为/兼容性按现有机器门验收；
  **实施者按当时限定平台口径记 DoD 7/8，性能（第 7 条）未达**，因此不得表述为 8/8 或“性能已追平”。
- 默认产品仍是 **O2 + switch VM** 的终端 darwin 构建；PGO 仅为候选，不默认启用。
- macOS GUI **未集成**：8 个 Swift 单元能单独编译，但没有完整冻结 Cocoa 目标，
  Swift/Rust 接口提供者联编未闭合，未产出或启动 GUI/.app。
- 本期结束，不是因为 DoD 全绿，而是用户明确决定：性能未达如实保留，不再深究；
  GUI 试用请求只完成构建诊断、没有交付；作为本期明确遗留结案，不能事后用排除范围冒充完成。

## 2. 原始目标与范围

- 参照基线：`fd7afced403`，版本 32.0.50 开发版，只读。
- 范围：590 个 C/H 文件一比一移植为 Rust；同名文件/函数/宏/结构体/字段/全局/subr，
  同语句顺序、控制流、错误模型、内存布局。
- `lisp/`、`leim/`、`test/`、`etc/`、`doc/`、`admin/` 原样复制，不重写；
  非 C 原生代码用 Swift/Kotlin/现代 C++；不做 C/Rust 混合链接。
- 验收：清单零差异、布局/符号/subr、L0–L5、原样树、生成物、平台冒烟、性能不低于参照。
- 本期最终：转写清单 590/590；旧限定平台汇总为 7/8。GUI、性能和完整等同性缺口不能由该旧计数消除。

## 3. 设计、架构与来源

- **来源**：逐文件读取参照 C 源码，按 `PORTING.md` 的 P-01…P-325 规则手写 Rust；
  没有使用外部 Lisp VM、解释器库或代码生成器替代原栈机。
- **构建**：Cargo workspace + `femacs/xtask`；逐单元 `rustc --emit=obj`，
  自定义 no_std driver 链接；不移植 autotools/Makefile 脚手架。
- **运行时**：自有 GC、reader、allocator、bytecode 栈机、dump/pdumper 路径；
  默认 switch 派发，PGO 候选独立构建根。
- **工具语言**：构建/链接/闸门 Rust，生成物/比对/门 Python；驱动器/参照配方适配器保留，
  过时脚手架按 `docs/retired-scaffolding.md` 登记丢弃。
- **替代方案调研**：PGO、`opt-level=3`、`target-cpu=native`、`become`+函数指针、
  最小 asm 链、256 项 asm 表、asm 热叶；均按真实 VM 或可复现 spike 评估。
  替代 VM 的资料调查见 `vm-investigation.md`；未完成同条件全面性能/兼容性横评，
  不据此证明原栈机最优，也不将代码生成后端当作完整语言运行时。

## 4. 时间线与关键提交

- 2026-10-02：goal 创建（工具记录 createdAt 2026-10-02T00:49:28+08:00）。
- 2026-10-21：PGO 计划收敛、阶段隔离/指纹/测量工具实现、集中 A/B、L5/GC 复核、
  asm 热叶真实 VM 失败与回退、非法 opcode 默认臂修正、功能性交接。
- 2026-10-21：协调对话 VM 调查与快速核验（`d2f4f774`）。
- 2026-10-22：协调对话 macOS 重编/92 条差分复核（`e125bb72`）及 GUI 构建诊断（`bba195c8`）。
- 关键提交：
  - `a8039971` PGO A/B 方案收敛
  - `2f308ee2` 阶段隔离、指纹、测量工具
  - `b156db51` PGO 集中构建与 7 轮测量
  - `003a6885` 条件门固定系统 clang
  - `32b67939` `verify-dod --full` 证据
  - `11b19a3a` `target-cpu=native` 否证
  - `55150353` PGO+O3 否证
  - `38225c15` 强制 GC ERT 证据
  - `e76daa5d` threaded 路线文档裁定（后被讨论修正）
  - `e8ca1e55` asm 热叶真实 VM 失败、回退与证据边界
  - `413fe74f` 非法 opcode 默认臂修正
  - `017cd8bc` 功能性交接快照
  - `d2f4f774` VM 调查与 bounded closeout 证据
  - `e125bb72` macOS 重编与 92 条差分复核
  - `bba195c8` macOS GUI 构建阻塞与 Swift 对象结果

## 5. 遇到的问题、解决与未解决项

### 已解决

- PGO 阶段混用：用 `FEMACS_BUILD_ROOT`/`FEMACS_PGO` 隔离；工具（`make-fingerprint`）
  固定普通 O2 对象根；gate/dump profile 不 merge。
- 指纹假占位：链接后运行端口自己的 `make-fingerprint`，再 `codesign -s - -f`，
  校验占位消失/摘要一致；每阶段写 `pair.sha256`。
- `ar` 版本漂移：执行器 PATH 可能先命中 GNU `ar`，Darwin 链接器拒绝其 archive；
  使用系统 SDK 随附的 `ar`。
- `llvm-profdata` 版本漂移：固定项目 nightly 工具链的 llvm-tools，避免 Xcode/系统版本
  的 raw profile 版本不匹配。
- asm 热叶调试：修复 `;` 被当注释导致分支丢失、`lispsym` 偏移误当直接指针、
  LR 恢复顺序等问题；但整体仍无性能收益。
- 非法 opcode：`op < Bconstant` 现在按 live threaded 的 `insn_default` 报
  `Invalid byte opcode`，不再负偏移索引 `vectorp`。
- 条件求值门：固定系统 Xcode `clang`，避免执行器 PATH 改变预处理器答案。

### 未解决 / 未证明

- **DoD-7 性能**：默认字节码仍为参照 **1.35–1.59×**；PGO `load` 为 **2.0439×ref**；
  没有热点采样/因果隔离，不能把剩余差距全部归因 bytecode 派发。
- **asm 热叶**：真实 VM 六项全部变慢（`bc-loop 10.88→23.45 ms`、
  `bc-car 22.38→53.57`，约 1.4–2.4×）；已回退，不能据此宣称所有 asm 方案都不可能。
- **GUI**：8 个 Swift 单元生成 `.o` 退出 0；但没有完整冻结 Cocoa 目标，
  `frame.c` 诊断在 `have_ns`/`have_window_system`/`ns_impl_cocoa` 下两个 pass 各 8 处
  geometry 作用域错误，并依赖 `no_value`；Swift/Rust 接口提供者未闭合；未产出/启动
  GUI 或 `.app`。`590/590` 只是转写覆盖，不等于 GUI 可用。
- **ERT 证据**：`check-ert.py` 只解析 summary 的 ran/expected/unexpected；TSV 的
  `same` 是文件级计数，不是逐用例身份/结果一致；564 份中 63 份 `same` 两侧 ran=0。
  失败身份不同仍 both-fail、运行数量不同仍 same、零次仍 same 的限制已被 AST 证实。
- **新鲜完整验收**：`verify-dod --full` 不重跑 L5；Android 行只检查旧图片存在；
  新候选的完整 L5/Android 未重跑。
- **最新稳定版兼容**：固定参照是 32.0.50 开发版，没有最新稳定版专门对拍。
- **恶意/畸形字节码**：只修了 `op < Bconstant` 默认臂；整体防护未证明。
- **清理历史**：实施阶段删除 O3 产物曾遇审批服务错误；协调对话随后已清理
  `build/pgo/use3` 等失败产物，共 564,571,593 逻辑字节，见 `macos-recheck-20261022.md`。
  本次一期最终清理另见总报告，不能继续把这条旧残留写成当前事实。

## 6. 真实验收与缺口

- `verify-dod --full`：所有实际运行的 row `ok`；DoD-7 未运行、记未达；
  1,219 s / 峰值 1.10 GB。
- L0 **19/0**；L1 **111/0+1**；L5 全量 564：**552 same / 7 both-fail /
  4 crashed（全部两侧）/ 1 port-better / 0 port-worse**；端口 9,014 ran /
  110 unexpected，参照 111。
- 强制 GC 压测同型：**552/7/4/1/0**，端口 110 / 参照 111。
- 生成物、原样树、平台结构门、签名提交门、`gate-fast`、`gate-load`、`dump`、
  terminal smoke 均绿。
- 用户侧 macOS 复核：680 单元重编、链接/dump、真实终端输入/保存/退出 0、
  92 条差分输入一致；同机三轮默认字节码仍为参照 **1.35–1.59×**。
- **缺口**：以上不能扩展为逐用例 ERT 全等、最新稳定版兼容、完整 GUI 交付、
  恶意字节码防护或性能追平。

## 7. 经验教训

- “转写覆盖 590/590”与“产品可用”是两种口径，文档和汇报必须分开。
- 性能必须用真实 VM、原始样本、轮次/离散度和固定参照；toy 只能否证具体实现，
  不能外推为所有方案的上限。
- 构建工具也会因 PATH/工具链漂移改变产物或 gate 结果；`clang`、`ar`、
  `llvm-profdata`、`codesign` 等应固定可核验来源。
- 指纹/签名/配对是构建正确性的一部分：改了二进制必须重算指纹、重签、重 dump。
- 汇总型 ERT 计数不足以证明逐用例一致；需要身份/结果级证据。
- GUI 需要完整冻结配置和接口提供者闭合，单独编译 Swift 单元不等于 GUI 可用。

## 8. 工具、运行标识、时间与资源

### 工具与客户端

- 客户端：Codex CLI；构建/运行经 `controlled-run` 执行器，普通 Git/搜索/文件操作直跑。
- 构建/验证工具：`cargo`、`rustc`、`xtask`、`clang`、`ar`、`codesign`、`shasum`、
  `llvm-profdata`、`lldb`、`otool`、Python 工具链、`git`。
- 模型：原会话元数据实际出现 `deepseek-flash`、`deepseek/deepseek-v4.1-flash`、
  `deepseek-v4.1-flash`；当前界面为 DeepSeek V4.1 Flash high。初始元数据 provider 为
  `deepseek-intl`，当前实际命令使用 `opencode-go` profile；中间 commandcode 阶段为
  人类已确认信息，无法逐请求核对其账单。完整日期与统计见一期总报告；不记录任何 key。

### 标识

- 会话/线程 ID：`01a0f2cb-af89-70b0-b13a-9c2e91cfdc90`
- 原 goal ID：`c9201722-3803-4aef-a3c0-97afbefae0d6`；threadId 与 goalId 不相同。
  结案前曾置 blocked，后来由实施者亲自执行原生 `/goal clear`。用户原意是 done，
  clear 是协调方理解错误；没有保留下来的原生 done 记录。
- 可用运行标识：`CARGO_TARGET_DIR=${TMPDIR}/xt-cargo`、构建根 `build/`、
  PGO 根 `build/pgo/{generate,use}`、结果目录 `build/pgo/results`。
- 代表性 controlled-run `run_id`：`93d704a469ef45909111a50c1c8f16c0`、
  `5234c82d6ce94171834e268e709934ce`、`ca038f657f484d20ac4a21190909c66a`、
  `8d511ce30f534efbbda62501d4c6732d`、`e8bf1d5b5390470dbd55da09a678de77`。

### 时间

- goal 创建：2026-10-02T00:49:28+08:00。
- 会话最早元数据：2026-09-30T22:50:37.500+08:00；本稿重点记录后期整合，
  不能把 10 月 21–22 日写成全部施工周期。
- 工具记录的 pause/交接附近 updatedAt：2026-10-21T23:31:06+08:00。
- 工具记录 goal `timeUsedSeconds`：1,192,918 s（约 331.4 h）；该值是 goal 口径的
  累计时间，可能包含空闲/等待，**不等于活跃计算时长**。
- 本文件写入：2026-10-22（Asia/Shanghai）。

### Token 与成本

- `get_goal`/`update_goal` 记录的 goal `tokensUsed`：**383,412,661**。
- 这是清除前保存的 goal 口径，**不是全会话/账单成本**。协调对话另提取了原会话
  客户端累计 token 快照及缓存字段，见一期总报告；账单金额仍未知，没有据此编造费用。

### 资源

- `verify-dod --full`：1,219 s / 峰值 1.10 GB。
- 默认 ERT 与 GC 压测：各约 7.8 min，峰值约 9.9 GB（受控执行器动态扩容，硬上限
  16 GB，结束回收）。
- 全量单元构建：峰值约 0.45–0.56 GB；PGO/asm 构建同量级。
- 以上为受控执行器报告值；不是整机总量或其他会话占用。

## 9. 最终状态

- 转写：**590/590**。
- 旧限定平台汇总：7/8 = 87.5%，性能未达；本期最终还明确保留 GUI 未交付等缺口，不再称只有性能问题。
- 默认构建：O2 + switch VM；PGO 仅候选；GUI 未集成。
- 工作树在本文件写入前为干净状态；本文件未提交、未清理，交由协调对话统一处理。
- 用户要保留目标并标记 done。协调对话误解成 clear 并引导执行；实施者已亲自用
  原生 `/goal clear` 清除，界面显示 `Goal cleared`，协调方只读核对原目标行已不存在。
  不补造 done、不新建替代目标。随后最终文字回复遇到上游 503，用户要求的重试亦失败；
  此错误不撤销已经发生的清除，也不影响已写出的总结文件。
