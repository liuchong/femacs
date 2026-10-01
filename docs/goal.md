# GOAL：femacs 一比一复刻 GNU Emacs（C → Rust）

把下面的文本整段粘贴给 Codex 即可作为 goal 执行。

---

## 目标

在本仓库 `/Users/liu/projects/src/github.com/liuchong/femacs`（下称 femacs）中，把
`emacs/` 参照检出（GNU Emacs `fd7afced403`，版本 32.0.50，**只读，绝对不许改动**）里的
**全部 C 源码一比一复刻为 Rust**，并让 Emacs 自带的 Lisp 测试套件在复刻结果上全绿。

- 复刻对象：`emacs/**` 下的 `*.c` / `*.h` 共 **590 文件、678,320 行**，目标路径见
  `docs/port-map.md`（`src/NAME.c` → `femacs/src/NAME.rs`；`src/NAME.h` →
  `femacs/src/include/NAME.rs`；`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、
  `nt/`、`msdos/` 原目录名照搬）。
- 非 C 的原生代码用该平台的新技术：`src/*.m`（Objective-C）→ **Swift**；
  `java/**/*.java` → **Kotlin**；`src/haiku_*.cc` → 现代 **C++**。
- Emacs Lisp（`lisp/`、`leim/`、`admin/` 里的 Lisp）与 `test/`、`etc/`、`doc/` **原样复制，
  一个字都不许改、不许重排、不许重写**。
- 构建脚手架（`configure.ac`、`Makefile.in`、`m4/`、`build-aux/`、`cross/`、`nextstep/`）
  不移植：用 Cargo workspace + Rust `xtask` 取代。

执行手册见 `docs/port-plan.md`（**开工前必须先完整读完**），逐文件对照见
`docs/port-map.md`（由 `python3 tools/gen-port-map.py` 生成，禁止手改）。

## 硬约束（违反即失败）

1. **一比一**：同名文件、同名函数、同名宏、同名结构体、同名全局变量、同语句顺序、
   同控制流。Rust 里允许并鼓励 `unsafe`，但**不许发明 C 里没有的行为**。
2. **不混合链接**：不要做"C 与 Rust 混编的中间态二进制"，不要为了"先跑起来"引入兼容层、
   shim、适配层。整棵树一次性复刻完成后才谈运行。
3. **禁止 stub**：`todo!()`、`unimplemented!()`、空函数体、直接返回默认值、注释掉的代码
   一律禁止；对抗评审必须把这类改动打回。
4. **禁止吞错**：不允许把 C 的返回码/错误路径改成"忽略错误"，不允许在没有对应 C 逻辑的
   位置加 `unwrap()`（fish 的教训：`.unwrap()` 就是崩溃）。
5. **禁止改测试**：`test/` 与 `lisp/` 里的任何文件都不许修改、跳过、删除。测试失败必须
   改实现。
6. **禁止改参照**：`git -C emacs status --short` 必须始终为空。
7. **禁止 git 破坏性操作**：`git stash`、`git reset`、`git checkout --`、`git rebase`、
   `git push --force` 全部禁止。提交只提交本次明确处理的文件。
8. **禁止全局分配器**：不得设置 `#[global_allocator]`；Lisp 对象一律走 Emacs 自己的分配器。
9. **禁止 panic 跨 FFI**：`panic = "abort"`；错误传播沿用 `sys_setjmp`/`sys_longjmp` 语义。
10. **布局不许"更 Rust"**：结构体一律 `#[repr(C)]`，偏移/大小/位域/tag 位必须与 C 完全一致；
    无法表达时停下来记录偏离项，不许自行改设计。
11. **提交必须签名**：每个文件一次签名提交（`port(NAME): …`），并推送到 `origin/master`。
12. **不许问、不许等**：遇到不确定处，按 `PORTING.md` 的既有约定处理并记录；只有同一文件
    连续 3 轮评审不通过时才写 `docs/ports/BLOCKED.md` 继续下一个文件。

## 执行顺序

### 阶段 A：预备（不写业务代码）

1. 生成并提交 `PORTING.md`：C 惯用法 → Rust 写法对照表（宏、位域、union、函数指针 +
   context、变参、`alloca`、`goto`、`setjmp/longjmp`、signal、线程局部、tagged pointer、
   GC 宏 `staticpro`/`gcpro`/`GCPRO`、`__attribute__`/对齐、内联汇编、`__builtin_*`、
   computed goto、字符串拼接、条件编译变体）。每条给真实 C 片段 + Rust 片段。
2. 生成并提交 `LIFETIMES.tsv`：`src/*.h` 全部结构体字段与 `src/*.c` 全部全局/静态对象的
   生命周期决策（谁分配、谁释放、是否 GC 管理、是否进 dump、是否跨 `setjmp`）。
3. 用两个对抗评审 agent 交叉复核 `PORTING.md` 与 `LIFETIMES.tsv`，修订后定稿。
4. 完成三个 spike 并记录结论：① `Lisp_Object`/`Lisp_String`/GC 头/`struct buffer`
   逐位布局一致；② `setjmp`↔`longjmp` 跨 Rust 帧无 `Drop` 泄漏；③ 保守栈扫描能看到
   Rust 栈帧上的对象。每个 spike 附可复现脚本。
5. 搭好 Cargo workspace + `xtask`（loadup、DOC、dump、unidata/charsets 生成）+ CI 两档
   （快：`cargo check` + 清单/布局断言；重：全量测试）。
6. **试跑 3 个文件**：`src/doprnt.c`、`src/sort.c`、`src/casetab.c`，走完整"单文件工作流"，
   据此修订 `PORTING.md`。

### 阶段 B–G：全量移植

按 `docs/port-plan.md` §4 的批次（B 内核 / C 文本与字符 / D 系统面 / E 显示与终端 /
F 平台与工具 / G dump 与模块）分片并行。每片一个 git worktree，每 worktree 内多个 agent
并行**不同**文件（同一文件绝不并发）。

**每个文件的固定循环（10 步，缺一不算完成）：**

1. `python3 tools/extract-abi.py emacs/src/NAME.c > docs/ports/NAME.md` 生成待勾选清单。
2. 按 `PORTING.md`/`LIFETIMES.tsv` 把 C 逐函数移植为 `femacs/src/NAME.rs`。
3. 补 `#[repr(C)]` 布局断言与 `const` 断言。
4. 勾清单：函数、DEFUN（subr）、宏、类型、extern、全局，逐条对齐。
5. 记录调用关系差异（无差异写"无"）。
6. `cargo check` 到该文件零错误。
7. `python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs` 必须 OK。
8. **1 名实现者 + 2 名对抗评审**（评审在独立上下文，只拿到 diff，不给实现者推理，任务
   是找出"它为什么是错的"）。
9. 1 名应用者按评审意见修补，重跑第 7 步。
10. 重跑 `python3 tools/gen-port-map.py`，记录进度，签名提交并推送。

### 验收闸门（全部必须点亮）

- 清单：`python3 tools/extract-abi.py --all` 零差异；`port-map.md` 中 1:1 Rust 590 条全
  `ported`。
- 布局/符号/subr 表：与参照双向断言一致。
- 加载阶梯 L0→L5：手写表达式 → `lisp/emacs-lisp/*.elc` → `subr/simple/loaddefs` →
  完整 `loadup.el`（dump 成功）→ `make -C test check` 默认选择器 → 全量含
  `:expensive-test`，**0 跳过、0 删除**。
- 兼容：GNU Emacs 产出的 `.elc`/dump 文件/动态模块能在 femacs 用，反之亦然。
- 生成物：`admin/unidata`、`admin/charsets`、`src/lisp.mk`、`ldefs-boot.el` 逐字节一致。
- 原样复制：`diff -r emacs/lisp femacs/lisp` 与 `test/`、`etc/`、`doc/` 全部无差异。
- 压力：`gc-cons-threshold=1` 下跑完整测试套件；随机 Lisp 程序差分对拍（stdout/stderr/
  退出码逐字节一致）。

## 进度与汇报

- 进度台账：`docs/port-map.md`（生成物）+ `docs/ports/*.md`（逐文件勾选清单）。
- 每完成一批，向用户报告：已复刻文件数/行数、`--check` 差异数、加载阶梯当前级别、
  测试通过率、阻塞项。
- **持续执行**：不要在阶段之间停下来征求同意；按本文档推进到"完成定义"全部满足为止。
  只有以下情况才停止并汇报：同一文件 3 轮评审不通过、需要用户提供本机不具备的平台
  （例如 Android NDK 或 Windows 工具链）、或磁盘/网络等外部资源不可用。

## 完成定义

`docs/port-plan.md` §8 的 8 条全部满足：590 个文件全部复刻且清单零差异、布局/符号/subr
三项全绿、L0–L5 全绿、双向兼容通过、生成物逐字节一致、原样树 `diff -r` 无差异、已交付
平台冒烟通过、性能不低于参照基线；全部提交已签名并推送。
