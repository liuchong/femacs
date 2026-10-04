# femacs 计划

[English](../../plan.md) · **简体中文**

[femacs](../../../README.md) 是对参照树的一比一复刻：`emacs/` 里的每一个 C 源文件都变成
同名、同标识符、同结构、同语义的 Rust 源文件，Emacs Lisp 树原样复制。

- **基线**：`emacs/` 检出 `fd7afced403`，被 git 忽略且只读。
- **总计划**：[port-master-plan.md](../../port-master-plan.md)——规模实测、方法论借鉴、12 个
  "完全一致"维度、批次、闸门与风险。
- **执行手册**：[port-plan.md](../../port-plan.md)——单文件循环、并行编排、预备阶段的细则。
- **可直接粘贴的 goal**：[goal.md](../../goal.md)。
- **逐文件数据**：[port-inventory.md](../../port-inventory.md)——590 行，含行数、函数、subr、
  宏、常量、结构体字段、全局、批次与状态，由 `tools/gen-port-inventory.py` 生成；
  [port-map.md](../../port-map.md)——全树映射，由 `tools/gen-port-map.py` 生成。两者都是数据，
  禁止手改。

文档语言约定：本文件与[英文原文](../../plan.md)遵循仓库既有约定（英文为源，
`docs/i18n/<locale>/` 为译文）。`port-plan.md` 与 `goal.md` 是给执行者和 Agent 用的作业
文本，只维护中文。

## 范围

| 工作 | 文件 | 行数 |
| --- | ---: | ---: |
| 一比一复刻的 C 源（`src/`、`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`msdos/`） | 590 | 678,320 |
| Objective-C → Swift（`src/*.m`、`lib-src/be_resources.cc`） | 9 | 26,517 |
| Java → Kotlin（`java/`） | 40 | 15,157 |
| C++ 保持现代写法（`src/haiku_*.cc`） | 4 | 8,085 |
| 汇编原样保留（`exec/loader-*.s`） | 6 | 1,774 |
| 原样复制（Emacs Lisp、测试、数据、手册、ChangeLog） | 4,656 | 4,606,879 |
| 由 Cargo 与 `xtask` 取代的构建脚手架 | 324 | 63,375 |

## 原则

1. **一比一，一次性完成。** 不做 C/Rust 混合的中间态二进制，不做 shim，不做过渡兼容层。
   整棵树复刻完整之后才谈运行。
2. **允许 `unsafe`**，因为目标就是复现 C 的语义；但绝不用它发明 C 里没有的行为。
3. **名字一律保留**——文件、函数、宏、结构体、全局变量、subr、命令行开关，以及它们背后的
   内存布局（`#[repr(C)]`，由双方编译期断言校验）。
4. **错误模型保留**——`sys_setjmp`/`sys_longjmp` 加 `record_unwind_protect`；不用 Rust
   `Result` 做控制流，不让 panic 跨越 FFI。
5. **收集器保留**——Emacs 自己的分配器、保守栈扫描、`staticpro`/`gcpro` 等价物；绝不安装
   Rust 全局分配器。
6. **Lisp 树不可改动**——`lisp/`、`leim/`、`test/`、`etc/`、`doc/` 按字节复制；测试永不跳过
   或删除。
7. **生成物仍是生成的**——`config.h`、`lisp.mk`、`emacs-module.h`、`etc/DOC`、unidata 与
   charset 表。
8. **验收就是上游测试套件**——用 ERT 跑复刻结果（`make -C test EMACS=… check`）、与参照
   差分对拍、subr 表比对、`.elc`/dump/模块 双向互换。

## 方法

方法沿用同类重写项目已经验证的做法：动代码之前先写映射指南与生命周期表（`PORTING.md`、
`LIFETIMES.tsv`），先用 3 个文件试跑校准，然后对每个文件做一遍机械移植，把 `cargo check`
的错误当作工作队列，每个文件 1 名实现者 + 2 名对抗评审，以原测试套件作为验收标准。
[port-plan.md](../../port-plan.md) 把它落成批次、闸门与逐文件清单；
[goal.md](../../goal.md) 是可直接粘贴的形态。
