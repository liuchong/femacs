# GOAL：femacs 一比一复刻 GNU Emacs（C → Rust）—— 执行级

> 把本文件**整段**作为 goal 粘贴给 Codex。执行到 §10 完成定义全部满足为止；
> **不许中途停下来问**（唯一例外见 §9）。

---

## 0 开工前（按顺序读，别跳）

| # | 文件 | 你要拿到什么 |
| --- | --- | --- |
| 1 | `docs/port-master-plan.md` | **总计划**：规模实测、方法论、12 个一致性维度、批次、闸门、风险 |
| 2 | `docs/port-plan.md` | 执行手册细则：D1–D12 展开、A 阶段交付物、并行编排 |
| 3 | `docs/port-inventory.md` | **590 行逐文件对照表**（行数 + 函数/subr/宏/字段/全局计数 + 批次 + 状态） |
| 4 | `PORTING.md` | C 惯用法 → Rust 写法对照表（宏/位域/union/变参/`goto`/`setjmp`/GC…），以及 P-01…P-75 已知偏离项 |
| 5 | `LIFETIMES.tsv` | 每个复杂生命周期字段的决策（谁释放、是否 GC、是否跨 `setjmp`、是否进 dump） |
| 6 | `docs/ports/<NAME>.md` | 当前文件的**待勾选清单**（由工具生成，逐条勾完才算完成） |
| 7 | `docs/baseline/` | 冻结基线：`inventory.tsv`、`subrs.tsv`、`config/`、`layout/`（含棘轮 `asserted-keys.txt`） |

然后跑这四条状态命令（把输出记进本轮汇报）：

```sh
git status --short && git log --oneline -5
python3 tools/extract-abi.py --all          # 权威判定：当前应显示 17 done / 10 WIP / 0 failing
git -C emacs status --short                 # 必须为空（参照树只读）
python3 tools/gen-port-inventory.py         # 刷新进度账
```

**工作区若有未完成文件（例如 `femacs/src/data.rs`、`femacs/src/include/lisp.rs` 的改动）：
先按 §4 的十步循环把它收尾并提交，再进入新文件。** 不要把半成品留在工作区过夜。

---

## 1 目标与实测规模（2026-10-02）

把参照检出 `emacs/`（`fd7afced403`，**只读、git 忽略**）里的
**590 个 `.c`/`.h`（678,320 行）一比一复刻为 Rust**：同名文件、同名函数/宏/结构体/字段/全局、
同语句顺序、同控制流、同内存布局、同错误模型。允许并鼓励 `unsafe`；**不许发明 C 里没有的行为**。

- 结构级总量：函数 10,012、`DEFUN`/subr 1,841、宏 7,156、常量 3,898、结构体字段 5,764、全局 1,934。
- 目标路径：`src/NAME.c` → `femacs/src/NAME.rs`；`src/NAME.h` → `femacs/src/include/NAME.rs`；
  `lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`msdos/` 原目录名照搬
  （规则唯一实现在 `tools/gen-port-map.py`，**不要照抄文档里的规则**）。
- 非 C 原生代码用新技术：`src/*.m` → **Swift**；`java/**` → **Kotlin**；`src/haiku_*.cc` → 现代 C++。
- Emacs Lisp（`lisp/` 1,627 文件 / 2,106,892 行）与 `test/`、`leim/`、`etc/`、`doc/`、`admin/`
  **原样复制，一个字都不许改、不许重排、不许重写**（不要花 token 重写 Lisp）。
- 构建脚手架（`configure.ac`、`Makefile.in`、`m4/` 等）不移植：用 Cargo workspace + `xtask` 取代。

---

## 2 硬约束（违反即失败）

1. **一比一**：同名文件/函数/宏/结构体/全局/语句顺序/控制流。
2. **不混合链接**：不做 C 与 Rust 混编的中间态二进制，不做 shim/适配层。
3. **禁止 stub**：`todo!()`、`unimplemented!()`、空函数体、返回默认值、注释掉的代码。
4. **禁止吞错**：不许忽略 C 的返回码/错误路径；没有对应 C 逻辑就不许加 `unwrap()`。
5. **禁止改测试**：`test/` 与 `lisp/` 一字不许改；测试失败必须改实现。
6. **禁止改参照**：`git -C emacs status --short` 必须始终为空。
7. **禁止破坏性 git**：`git stash`、`git reset`、`git checkout --`、`git rebase`、`git push --force`。
8. **禁止全局分配器**：不得设置 `#[global_allocator]`；Lisp 对象一律走 Emacs 自己的分配器。
9. **禁止 panic 跨 FFI**：`panic = "abort"`；错误传播沿用 `sys_setjmp`/`sys_longjmp`。
10. **布局不许"更 Rust"**：一律 `#[repr(C)]`，偏移/大小/位域/tag 位与 C 完全一致；
    表达不了就记偏离项，不许自行改设计。
11. **提交必须签名**：每个文件一次签名提交（`port(NAME): …`）并推送到 `origin/master`。
12. **不许问、不许等**：按 `PORTING.md` 既有约定处理并记录；只有同一文件连续 3 轮评审不通过时
    写 `docs/ports/BLOCKED.md`，然后继续下一个文件。

---

## 3 工作区纪律（用户明确要求）

- **不要频繁编译**：只在必要时编译——单文件用 `xtask build-tu src/NAME.c`，
  批次末用 `xtask gate-fast` / `gate-probes`；**不要**反复跑全量 `cargo build`/`make`。
  编译产物用独立目录（`CARGO_TARGET_DIR=/tmp/...` 或仓库内 `build/`），不要污染别处。
- **及时清理编译产物**：每轮收尾 `rm -rf build femacs/target`（它们是生成物，可重建）。
- 工具链已 pin：`femacs/rust-toolchain.toml` = `nightly-2026-10-01`；
  **所有 `rustc`/`cargo` 命令都要在 `femacs/` 目录里跑**（仓库根的裸 `rustc` 会解析到别的版本）。
- 只改本次明确处理的文件；不做"顺手重构"；不新增与复刻无关的文件。

---

## 4 单文件十步循环（缺一不算完成）

1. `python3 tools/extract-abi.py emacs/src/NAME.c > docs/ports/NAME.md` 生成待勾选清单。
2. 按 `PORTING.md`/`LIFETIMES.tsv` 把 C 逐函数移植为 `femacs/src/NAME.rs`（头文件 → `femacs/src/include/NAME.rs`）。
3. 补齐 `#[repr(C)]` 布局断言与 `const` 断言；新类型要进 `tools/gen-layout-fields.py` 并重生成布局基线。
4. 逐条勾选清单：函数、DEFUN（subr）、宏、类型、extern、全局。
5. 记录调用关系差异（D7）：无差异写"无"，有差异写清单与理由。
6. `cargo check` 到该文件零错误；禁止 stub。
7. `python3 tools/extract-abi.py --check emacs/src/NAME.c femacs/src/NAME.rs` 必须 OK。
8. **2 名对抗评审**：独立上下文，**只给 diff 不给实现者推理**，任务是"找出它为什么是错的"。
9. **1 名应用者**按评审意见修补，重跑第 7 步；评审记录写入 `docs/reviews/`。
10. 重跑 `python3 tools/gen-port-map.py`（收尾时再跑 `gen-port-inventory.py`），
    **签名提交** `port(NAME): …` 并推送到 `origin/master`。

并行规则：一个 worktree 内 4–16 个 agent 并行**不同**文件；**同一文件绝不并发**。

---

## 5 每轮固定动作（RL 循环）

1. 读 §0 的文件与状态命令；确认上一轮遗留（`git status --short` 必须干净或只有本轮文件）。
2. 挑**当前批次里依赖已满足**的文件（顺序见 §6），走 §4 十步循环，逐个提交。
3. 每完成 1 个文件：`--check` OK + 签名提交 + 推送 + 记录进度。
4. 每完成一批：`xtask gate-fast` → `xtask gate-probes` → `python3 tools/extract-abi.py --all`。
5. 收尾：`rm -rf build femacs/target` → 刷新 `docs/port-inventory.md` → 汇报（§8）。

---

## 6 闸门（命令；新环境/清过 `build/` 后先重建冻结配置）

```sh
# 冻结配置重建（每轮从零重建；健康线：config.h 有 342 行含 #define）
mkdir -p build/tools && printf '#!/bin/sh\necho "makeinfo (GNU texinfo) 7.1"\n' > build/tools/makeinfo && chmod +x build/tools/makeinfo
mkdir -p build/emacs-ref && cd build/emacs-ref && PATH=$PWD/../../build/tools:$PATH ../../emacs/configure --without-all --with-x=no --with-ns=no
make -C lib stdbit.h byteswap.h stdint.h limits.h && make -C src globals.h

# 闸门
python3 tools/extract-abi.py --all                      # 清单：必须 0 failing
(cd femacs && cargo run -q -p xtask -- gate-fast)       # 条件求值 + 清单 + cargo check + 单元 + 探针 + 布局断言
(cd femacs && cargo run -q -p xtask -- gate-probes)     # inline/accessor/write/va/syshdrs/gmp-abi/lisp-abi + dead-arms
python3 tools/gen-port-map.py && git diff --exit-code -- docs/port-map.md   # 生成物新鲜度
git -C emacs status --short                             # 必须为空
```

**新镜像必须先过条件求值白名单**：`python3 tools/check-cond-eval.py <hdr> --write-verified …`，
否则闸门会拿错分支当要求（P-66）。布局/符号门全绿**不等于**语义正确——宏的 l-value/求值语义
变异曾 16 次全部躲过冻结闸门，所以 D7/D9 的对抗评审与行为差分不能省。

---

## 7 批次顺序与下一步（2026-10-02 的现场）

批次表见总计划 §6。**当前状态：A 阶段完成；B/C/E/L 正在并行清尾巴。**

1. **先收尾在飞的文件**：`src/data.c`（WIP，120 个 subr 全未写；自然批次：
   `add_to_function_history`/`defalias`/`kboard_for_bindings`/`do_symval_forwarding`/
   `store_symval_forwarding`/`swap_in_global_binding`/`swap_in_symval_forwarding`/
   `find_symbol_value`/`set_internal`/`default_value`/`make_blv` → 120 个 `DEFUN!` →
   算术驱动 `arithcompare`/`arith_driver`/`float_arith_driver`/`bignum_arith_driver`/
   `minmax_driver`/`expt_integer`/布尔向量 → `syms_of_data`）。
2. **清头镜像尾巴**（依赖顺序）：`window.h`（剩 55 宏）→ `frame.h`（剩 48 项全是显示面）→
   `termhooks.h`（9 个 `FRAME_*`）→ `dispextern.h`（矩阵/face/image 段，逐批把 `*mut c_void`
   注释升级成真类型）→ `termchar.h`。
3. **再推批次 B 内核剩余**：`alloc.c`、`eval.c`、`lread.c`、`print.c`、`fns.c`…；
   同时按需推进 L0–L5 加载阶梯的基建（`xtask` 的 loadup/DOC/dump）。

一个文件的三轮评审都过不了 → 记 `docs/ports/BLOCKED.md`（附证据与尝试过的修法）→ 继续下一个文件。

---

## 8 进度台账与汇报格式

每完成一批（或每 10 个文件，先到者为准）向用户汇报，格式固定：

```
本轮：port(NAME) ×N（提交 <sha7> …，全部 verified=true）
进度：已复刻 X/590 文件、Y/678,320 行；--all = a done / b WIP / c failing
闸门：gate-fast <绿/红>；gate-probes <绿/红>；布局断言 <n>/<pending>
阶梯：当前 L<k>（下一级差什么）
阻塞：<无 / BLOCKED.md 条目>
下一轮：<具体文件清单>
```

台账：`docs/port-inventory.md`（进度）+ `docs/ports/*.md`（勾选）+ `docs/reviews/`（评审）。

---

## 9 停止条件（只有这几种才停）

1. 同一文件连续 3 轮评审不通过（写 `BLOCKED.md` 后**继续下一个文件**，不算停下）。
2. 需要本机不具备的外部资源（例如 Android NDK、Windows 工具链、磁盘/网络不可用）。
3. 需要用户裁决的**规格冲突**（例如 C 行为自相矛盾），且 `PORTING.md` 里没有既有约定。

**除此之外不许停下来征求同意**：按本文档与总计划推进到 §10 全部满足为止。

---

## 10 完成定义（DoD）

1. `port-map.md` 的 1:1 Rust 590 条全部 `ported`，`extract-abi.py --all` 零差异。
2. 布局断言、符号表、subr 表三项全绿（含键集棘轮只增）。
3. L0–L5 全绿：ERT 通过率与参照一致，**0 跳过、0 删除**。
4. `.elc`/dump/动态模块双向互换；`admin/unidata`、`admin/charsets`、`ldefs-boot.el`、
   `src/lisp.mk` 生成物逐字节一致。
5. `diff -r emacs/lisp femacs/lisp`、`test/`、`etc/`、`doc/` 无差异。
6. 已交付平台（macOS/Linux/Windows/Android/Haiku）冒烟通过。
7. 性能不低于参照基线（启动时间、`benchmark-run`、批量文本操作）。
8. 全部提交签名并推送；`docs/ports/*.md` 清单全部勾选。

---

## 附：别重复踩的坑（精选，完整清单在 `PORTING.md` §9）

- **P-66**：新镜像必须先 `check-cond-eval.py --write-verified` 进白名单，否则用错分支。
- **P-72**：头镜像只能命名自己声明或经自己 `include!` 拿到的类型。
- **P-69/P-70**：跨头指针类型走包含图；平台只作指针用途的类型用替身，平台头落地时再删。
- **P-74/P-75**：字段扫描器与函数体比较要按该文件的宏状态丢弃死行（`#ifdef` 包围的成员/语句）。
- **P-60**：`GCALIGNED` 类 `static_assert` 还没补全（缺 10 条）。
- **P-64**：`lisp-abi-probe` 对 `pub type` 定义与 61 条对象声明只计数。
- `xtask gen-config` 必须在**仓库根**跑（在 `femacs/` 下会把 `config.rs` 写残）。
- 布局生成器只键**一层**成员（union 内层成员不可键）。
- `sizeof`/`alignof` 已是 token family（要求出现次数一致），改一处会红。
