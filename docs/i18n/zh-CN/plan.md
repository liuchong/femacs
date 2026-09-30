# 一期：一比一复刻计划

[English](../../plan.md) · **简体中文**

[femacs](../../../README.md) 用复刻的方式重做 GNU Emacs：C 源码逐文件、逐语句译成
Rust，Emacs Lisp 源码原样复制。

- **基线**：`emacs/` 检出，`fd7afced403`（GNU Emacs 32.0.50）。它被 git 忽略，是所有
  复刻工作的唯一参照。
- **配套文档**：[port-map.md](../../port-map.md) 是脚本生成的逐文件对照表——文件名、
  行数、状态。它是数据，不是叙述。
- **一期目标**：参照树中的每一个 C 源文件，都在 Rust 侧存在同名、同结构、同语义的
  文件；随发行一起提供的 Emacs Lisp 树能在复刻结果上运行。

## 1. 规模

| 工作 | 文件数 | 行数 |
| --- | ---: | ---: |
| 需要复刻的 C 源（`.c` + `.h`） | 588 | 678,262 |
| ——可移植核心（`src/`） | 163 | 366,107 |
| ——平台后端（`src/`） | 87 | 200,677 |
| ——gnulib（`lib/`） | 237 | 69,423 |
| ——`lib-src`、`lwlib`、`oldXMenu`、`exec`、`nt` | 101 | 42,055 |
| Objective-C，暂缓（`src/*.m`） | 8 | 26,338 |
| Emacs Lisp，原样复制 | 1,627 | 2,106,892 |
| 测试套件，原样复制 | 1,555 | 464,488 |
| 其他原样复制的树（`etc`、`doc`、`leim`、`admin`、`m4` 等，含 ChangeLog） | 1,750 | 1,937,036 |

三个文件主导工期：`xdisp.c`（39,805 行）、`xterm.c`（33,125）、`sfnt.c`（21,215）。

## 2. 忠实度规则

**R1 —— 文件名。** 每个复刻文件保持原名：`src/NAME.c` 对应 `femacs/src/NAME.rs`；
`src/NAME.h` 对应 `femacs/src/include/NAME.rs`；其他 C 目录在 `femacs/` 下沿用同名
目录。不重命名、不合并、不拆分。

**R2 —— 标识符。** 函数、宏、枚举、结构体、联合体成员、全局变量保持完全相同的拼写：
`Ffuncall`、`Lisp_Object`、`CHECK_STRING`、`Vload_path`、`Qnil`、`struct Lisp_String`、
`GC_MARK_STACK`。带代码的 C `#define` 变成同名 `macro_rules!`；带常量的 C `#define`
变成同名 Rust 常量。

**R3 —— 编译单元与 ABI。** 一个 `.c` 文件对应一个独立编译的 Rust 文件，正如 C 的各
编译单元彼此独立。每个 Rust 文件自行声明所用符号，`src/NAME.h` 按 C 的方式被文本
包含——Rust 的 `include!("include/NAME.rs")` 对应 C 的 `#include "NAME.h"`。导出符号
用 `#[no_mangle] pub unsafe extern "C" fn` 并保留 C 名字，因此 C 与 Rust 目标文件可以
链进同一个二进制，整棵树能逐文件切换。

**R4 —— 内存布局。** `#[repr(C)]` 结构体，成员顺序一致；标记位、GC 位、`USE_LSB_TAG`
以及每个大小与偏移都与 C 完全相同。双方在编译期各自断言：C 侧 `offsetof`/`verify`
对 Rust 侧 `offset_of!` 与 `const` 断言。无法这样表达的布局记为偏离项，而不是重新
设计。

**R5 —— 语义。** 控制流、错误模型、内存行为全部保留：错误走 `sys_setjmp`/
`sys_longjmp` 加 `record_unwind_protect`（不用 Rust 的 `Result`，不用 `Option` 做控制
流，不让 panic 跨越 FFI），标记清扫式 GC 与保守栈扫描照旧，分配器照旧。绝不安装
Rust 全局分配器。

**R6 —— 语句对应。** 文件内部的定义顺序跟随 C 文件，一条 C 语句对应一条 Rust 语句。
行数预期接近，但当 Rust 的写法本身不同（`switch` 用 `match`、函数指针加 context 用
闭包）时，把偏离记进对照表，而不是硬凑行数。

**R7 —— 生成物是生成的，不是复刻的。** `src/config.h`、`src/lisp.mk`、
`src/emacs-module.h`、`lisp/loaddefs.el`、`etc/DOC`，以及 `admin/unidata`、
`admin/charsets` 的表，继续由原构建流程生成。逐字节一致是验收目标，不是复刻目标。

**R8 —— 原样复制的树只复制，不修改。** `lisp/`、`test/`、`etc/`、`doc/`、`leim/`、
`admin/`、`m4/`、`build-aux/`、`nextstep/`、`java/`、`msdos/`、`cross/`、`modules/`、
根目录构建文件以及 ChangeLog，路径原样复制。这些树里的任何文件都不修改、不重新
排版、不翻译。

## 3. 工作批次

每个批次都以一个可运行产物和一个 gate 结束。文件数与行数取自
[port-map.md](../../port-map.md)。

| 批次 | 内容 | 通过标准 |
| --- | --- | --- |
| 1A | 解释器内核：`lisp.h`、`alloc.c`、`gmalloc.c`、`ralloc.c`、`vm-limit.c`、`eval.c`、`lread.c`、`print.c`、`bytecode.c`、`fns.c`、`data.c`、`doc.c`、`doprnt.c`、`floatfns.c`、`bignum.c`、`sort.c`、`cmds.c`、`macros.c`、`callint.c`、`atimer.c` | `femacs --batch -Q --eval` 在 1A 语料上的求值、打印、报错与退栈行为与参照逐字节一致 |
| 1B | 文本与缓冲区：`buffer.c`、`insdel.c`、`marker.c`、`intervals.c`、`textprop.c`、`casefiddle.c`、`casetab.c`、`indent.c`、`syntax.c`、`search.c`、`regex-emacs.c`、`region-cache.c`、`itree.c`、`undo.c`、`editfns.c` | buffer/marker/文本属性/syntax/正则各 ERT 组通过 |
| 1C | 字符与编码：`character.c`、`charset.c`、`chartab.c`、`category.c`、`ccl.c`、`composite.c`、`coding.c`、`bidi.c` | `coding-tests`、`charset-tests`、`character-tests` 通过；文件读写往返完全一致 |
| 1D | 系统面：`emacs.c`、`sysdep.c`、`fileio.c`、`filelock.c`、`dired.c`、`process.c`、`callproc.c`、`thread.c`、`systhread.c`、`timefns.c`、`inotify.c`、`kqueue.c`、`gfilenotify.c`、`dynlib.c`、`sound.c`、`json.c`、`sqlite.c`、`gnutls.c`、`xml.c`、`dbusbind.c`、`decompress.c`、`profiler.c` | `process-tests`、`fileio-tests`、`thread-tests` 通过，含真实子进程 |
| 1E | 显示与终端：`xdisp.c`、`dispnew.c`、`window.c`、`frame.c`、`term.c`、`terminal.c`、`termcap.c`、`terminfo.c`、`tparam.c`、`font.c`、`fontset.c`、`ftfont.c`、`ftcrfont.c`、`hbfont.c`、`sfnt.c`、`sfntfont.c`、`xfaces.c`、`fringe.c`、`cm.c`、`scroll.c`、`keyboard.c`、`keymap.c`、`minibuf.c`、`menu.c`、`widget.c`、`image.c`、`textconv.c`、`treesit.c`、`comp.c` | `xdisp-tests`、`window-tests` 通过；终端会话与录制帧一致 |
| 1F | 平台后端：`nsfns.m`、`nsterm.m`、`nsfont.m`、`nsmenu.m`、`nsselect.m`、`nsimage.m`、`nsxwidget.m`、`macfont.m`（Objective-C）、`x*`（X11）、`pgtk*`、`w32*`、`haiku*`、`android*`、`msdos*`、`w16select.c` | 各目标平台 GUI 会话能启动，对应后端测试组通过 |
| 1G | Dump、模块与工具：`pdumper.c`、`emacs-module.c`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`lib/`（gnulib） | dump 文件、`.elc`、动态模块与参照双向互换 |

1A–1E 按依赖排序；1E 立住之后，1F 与 1G 可以并行。

## 4. 加载阶梯

进度用"随发行的 Lisp 树能跑多少"来衡量：

| 阶梯 | 加载内容 | 目标批次 |
| --- | --- | --- |
| L0 | 手写表达式 | 1A |
| L1 | `lisp/emacs-lisp/*.elc` 全部 | 1B |
| L2 | `subr.elc`、`simple.elc`、`loaddefs.elc` | 1C |
| L3 | 完整的 `loadup.el` 预加载集合 | 1D |
| L4 | `make -C test check` 默认选择器 | 1E |
| L5 | 含 `:expensive-test` 的完整测试套件 | 一期出口 |

## 5. 验证

1. **ERT 跑任意一个二进制。** `test/Makefile.in` 允许命令行覆盖 `EMACS`，因此
   `make -C test EMACS=…/femacs check` 就能用上游测试套件验收复刻结果。测试套件是
   验收标准本身，不是替代品。
2. **差分运行。** 同一批 Lisp 表达式分别由参照与复刻在 `--batch` 下求值，stdout、
   stderr、退出码必须逐字节一致。
3. **布局断言。** 生成的 C/Rust 断言覆盖每一个共享结构体；不一致直接构建失败。
4. **二进制互换。** `.elc`（字节码与编码 cookie）、dump 文件、动态模块必须双向可加载。
5. **生成物一致。** 复刻产出的 `admin/unidata`、`admin/charsets`、`src/lisp.mk`、
   `ldefs-boot.el` 必须与参照输出逐字节一致。
6. **性能。** 功能对齐之后再量：启动、编辑延迟、批量操作，与参照构建对比。

## 6. 构建整合

参照构建系统继续主导：它已经知道 `loadup`、`DOC`、`leim`、unidata 表和 dump 怎么
处理。用一个逐文件开关决定 `src/NAME.c` 还是 `femacs/src/NAME.rs` 参与 `temacs` 的
链接，于是整棵树可以一次一个文件地迁移，两种版本随时都能构建。Rust 文件按独立单元
编译（`--emit=obj`、`-C panic=abort`、单 codegen unit），与仍是 C 的目标文件链进同一
个二进制、共用同一套符号。开关表由对照表生成。

## 7. unsafe 政策

`unsafe` 是预期之内且被允许的，因为目标就是复现 C 的语义。但它有边界，不是随手用：

- 本质上就不安全的点——标记指针、对象布局、收集器、`setjmp`/`longjmp`、裸缓冲区、
  FFI——可以自由使用 `unsafe`。
- 每个 `unsafe` 块都带 `SAFETY:` 注释，说明它复现的是哪条 C 不变式。
- 不允许 panic 跨越 FFI 边界（`panic = abort`，不用 `Result` 传递错误）。
- 绝不用 `unsafe` 发明 C 源码里没有的行为。

## 8. 单文件工作流

1. 复刻：按 R1–R6 把 C 文件译成 `femacs/src/NAME.rs`。
2. 断言：为其结构体补上布局断言。
3. 双向构建：开关打开与关闭都能编过并链接。
4. 测试：差分运行，加上受影响的 ERT 组。
5. 记录：`python3 tools/gen-port-map.py` 刷新 `docs/port-map.md` 的状态与行数。
6. 提交：签名提交，消息形如 `port(NAME): …`，并推送。

## 9. 已知偏离（台账起点）

| 偏离 | 原因 | 处理 |
| --- | --- | --- |
| `bytecode.c` 使用 computed `goto` | Rust 没有 computed `goto` | 用 `match` 或内联汇编；记入对照表，并与参照对比测量 |
| `src/*.m`（Objective-C） | 不是 C | Rust 加 `objc_msgSend` FFI，或保留一层 Objective-C 外壳；批次 1F |
| `src/haiku_*.cc`（C++） | 不是 C | 批次 1F |
| `gnulib`（`lib/`） | 第三方可移植垫片 | 在批次 1G 一比一复刻 |
| `comp.c` 需要 `libgccjit` | 外部库 | 用 FFI 调 `libgccjit`，本身在批次 1E 一比一复刻 |

## 10. 待定决策

1. Objective-C 后端是译成 Rust，还是保留一层薄 Objective-C 外壳。
2. gnulib 是留在 C 后面走 FFI，还是一期就复刻（当前计划放在批次 1G）。
3. 一期实际交付的平台后端（`x*`、`pgtk*`、`w32*`、`haiku*`、`android*`、`msdos*`
   目前都在上面的范围内）。
