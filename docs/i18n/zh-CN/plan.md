# 一期：一比一复刻计划

[English](../../plan.md) · **简体中文**

[femacs](../../../README.md) 用两条线复刻 GNU Emacs：编辑器自有的可移植 C 逐语句译成
Rust；凡是与平台、第三方库、构建相关的部分，一律用当前技术重做，不继承旧技术栈。
Emacs Lisp 源码只复制，不重写。

- **基线**：`emacs/` 检出，`fd7afced403`（GNU Emacs 32.0.50）。它被 git 忽略，是所有
  复刻工作的唯一参照。
- **配套文档**：[port-map.md](../../port-map.md) 是脚本生成的逐文件对照表——模式、文件名、
  行数、状态。它是数据，不是叙述。
- **一期目标**：随发行提供的 Emacs Lisp 树能在复刻结果上运行；参照树的每个源文件都有
  明确归宿：逐语句的 Rust 孪生、现代化重写，或记入台账的淘汰。

## 0. 技术政策

**T1 —— Emacs Lisp 永不重写。** `lisp/`、`leim/` 以及 `admin/` 里的 Lisp 生成器按字节
复制。不翻译、不重排、不现代化，也不为它们花任何 token。

**T2 —— Emacs 自有的可移植 C 一比一变成 Rust。** 同名文件、同名标识符、同语句顺序、
同语义，C 需要 `unsafe` 的地方就用 `unsafe`（见 §2）。这是唯一一条"翻译"而非"重写"的
工作线。

**T3 —— 平台层使用该平台当前的技术，能 Rust 就 Rust。**

| 目标平台 | 参照实现 | femacs |
| --- | --- | --- |
| macOS / iOS | Objective-C（`ns*.m`、`macfont.m`） | **Swift**：AppKit、CoreText、WKWebView、Swift 并发。任何人都不再新增 Objective-C |
| Windows | Win32/GDI/Uniscribe 的 C（`w32*`） | **Rust + windows-rs**：DirectWrite、Direct2D、现行 Win32 |
| Linux / BSD | Xlib/Xt 加 Lucid，或 GTK 3 | **Rust + GTK 4 / libadwaita，走 Wayland** |
| Android | Java + JNI + execve 蹦床 | **Rust（NDK/JNI）+ Kotlin + Gradle** |
| Haiku | C++ 加 BeOS 时代调用 | **现代 C++**（该平台 API 本身就是 C++） |
| tty | terminfo | 不变——terminfo 仍是现行机制 |

后端的头文件即使实现改用 Swift、C++ 或 Kotlin，也仍然是 Rust：它们声明的是与内核共享
的接口和内存布局，内核必须能链接它们（并由它们生成供 Swift 导入的 C 头文件）。

**T4 —— 第三方 C 库换成当前的 Rust crate。**

| 原 | 新 |
| --- | --- |
| GnuTLS | `rustls` |
| libgmp | 纯 Rust 大整数 |
| SQLite 绑定 | `rusqlite` |
| jansson（`json.c`） | `serde_json` |
| libxml2 | `quick-xml` |
| libdbus | `zbus` |
| zlib | `flate2` |
| `dlopen` | `libloading` |
| ImageMagick、lcms2 | `image` crate 与当前编解码器 |
| Emacs 自带 TrueType 栅格化（`sfnt.c`）与 FreeType/HarfBuzz 驱动 | HarfBuzz 整形、FreeType 与各平台文本栈 |
| libgccjit（`comp.c`） | 基于 LLVM 或 Cranelift 的本地编译 |
| WebKit 部件（`xwidget.c`） | WebKitGTK / WKWebView |

**T5 —— 构建与脚手架同样用当前技术。** 用 Cargo workspace 加 Rust 的 `xtask` 驱动
loadup、`DOC`、dump 与各生成表；macOS 侧用 SwiftPM，Android 侧用 Gradle。autoconf、
`m4/`、`Makefile.in`、NMake 工程与 gnulib 一律淘汰：gnulib 覆盖的部分由 Rust std、
`libc`、`nix`、`windows` 承担。

**T6 —— 语义边界。** 凡是 Lisp 可见的行为，保真优先于现代化，哪怕参照实现看起来很
旧：正则语法、字节码格式、dump 格式、模块 ABI、字符集与编码系统、文本属性、键映射、
undo、GC 报告，以及 `lisp/` 树本身。见 §10。

## 1. 规模

| 模式 | 文件 | 行数 |
| --- | ---: | ---: |
| `1:1 Rust` —— 逐语句复刻 | 183 | 334,348 |
| `Modern Rust` —— 同名，改用当前 API | 55 | 170,318 |
| `Swift` —— macOS 后端 | 8 | 26,338 |
| `C++` —— Haiku 后端 | 8 | 12,616 |
| `Kotlin` —— Android 应用层 | 40 | 15,157 |
| **手写总量** | **294** | **558,777** |
| `Drop` —— 淘汰并替换 | 580 | 212,023 |
| `Verbatim` —— 原样复制（Emacs Lisp、测试、数据、手册） | 4,616 | 4,400,294 |
| `Generated` —— 由构建产生 | — | — |

`Modern Rust` 细分：Windows（`windows-rs`，17 文件 50,959 行）、Android（`NDK/JNI`，
8 文件 29,754 行）、GTK 4/Wayland（5 文件 15,095 行）、crate 替换（25 文件 74,510 行）。

`Drop` 细分：gnulib（237 文件 69,423 行）、Xlib/Xt 后端（9 文件 54,237 行）、Lucid 部件集
（48 文件 13,529 行）、Windows/DOS 遗留（66 文件 18,890 行）、Android execve 蹦床
（8 文件 4,274 行）、autoconf 一类脚手架（209 文件 41,920 行）、遗留资源（3 文件 9,750 行）。

## 2. 1:1 集合的忠实度规则

**R1 —— 文件名。** `src/NAME.c` 对应 `femacs/src/NAME.rs`；`src/NAME.h` 对应
`femacs/src/include/NAME.rs`；其他 C 目录在 `femacs/` 下沿用同名目录。不重命名、不合并、
不拆分。

**R2 —— 标识符。** 函数、宏、枚举、结构体、联合体成员、全局变量保持完全相同的拼写：
`Ffuncall`、`Lisp_Object`、`CHECK_STRING`、`Vload_path`、`Qnil`、`struct Lisp_String`、
`GC_MARK_STACK`。带代码的 C `#define` 变成同名 `macro_rules!`；带常量的变成同名 Rust 常量。

**R3 —— 编译单元与 ABI。** 一个 `.c` 文件对应一个独立编译的 Rust 文件。每个文件自行
声明所用符号；`src/NAME.h` 按 C 的方式被文本包含——`include!("include/NAME.rs")` 对应
`#include "NAME.h"`。导出符号用 `#[no_mangle] pub unsafe extern "C" fn` 保留 C 名字，
因此 C 与 Rust 目标文件可以链进同一二进制，整棵树能逐文件切换。

**R4 —— 内存布局。** `#[repr(C)]` 结构体、成员顺序一致；标记位、GC 位、`USE_LSB_TAG`
以及每个大小与偏移都与 C 完全相同，并由双方在编译期断言（C 侧 `offsetof`/`verify` 对
Rust 侧 `offset_of!` 与 `const` 断言）。无法这样表达的布局记为偏离项，而不是重新设计。

**R5 —— 语义。** 控制流、错误模型、内存行为全部保留：错误走 `sys_setjmp`/
`sys_longjmp` 加 `record_unwind_protect`（不用 Rust 的 `Result`，不用 `Option` 做控制流，
不让 panic 跨越 FFI），标记清扫式 GC 与保守栈扫描照旧，分配器照旧。绝不安装 Rust 全局
分配器。

**R6 —— 语句对应。** 定义顺序跟随 C 文件，一条 C 语句对应一条 Rust 语句。当 Rust 的
写法本身不同（`switch` 用 `match`、函数指针加 context 用闭包）时，把偏离记进对照表，
而不是硬凑行数。

**R7 —— 生成物仍是生成的。** `src/config.h`、`src/lisp.mk`、`src/emacs-module.h`、
`etc/DOC`、`lisp/loaddefs.el`、`admin/unidata` 与 `admin/charsets` 的表都由构建产生。
逐字节一致是验收目标，不是复刻目标。

**R8 —— 原样复制的树只复制，不修改。** `lisp/`、`test/`、`etc/`、`doc/`、`leim/`、
`admin/`、根目录文档与 ChangeLog 保持原路径与原字节。Emacs Lisp 不重排、不翻译，哪怕
现代风格更短。

## 3. 工作批次

| 批次 | 内容 | 通过标准 |
| --- | --- | --- |
| 0 | 工作区与构建：Cargo workspace、`xtask`、SwiftPM 包、Gradle 工程，以及把 Rust 与剩余 C 链进同一二进制的逐文件开关 | 每个已复刻文件的两种变体都能编过并链接；开关表由对照表生成 |
| 1A | 解释器内核（`1:1`）：`lisp.h`、`alloc.c`、`gmalloc.c`、`ralloc.c`、`vm-limit.c`、`eval.c`、`lread.c`、`print.c`、`bytecode.c`、`fns.c`、`data.c`、`doc.c`、`doprnt.c`、`floatfns.c`、`sort.c`、`cmds.c`、`macros.c`、`callint.c`、`atimer.c` | `femacs --batch -Q --eval` 在 1A 语料上的求值、打印、报错与退栈与参照逐字节一致 |
| 1B | 文本与缓冲区（`1:1`）：`buffer.c`、`insdel.c`、`marker.c`、`intervals.c`、`textprop.c`、`casefiddle.c`、`casetab.c`、`indent.c`、`syntax.c`、`search.c`、`regex-emacs.c`、`region-cache.c`、`itree.c`、`undo.c`、`editfns.c` | buffer、marker、文本属性、syntax、正则各 ERT 组通过 |
| 1C | 字符与编码（`1:1`）：`character.c`、`charset.c`、`chartab.c`、`category.c`、`ccl.c`、`composite.c`、`coding.c`、`bidi.c` | `coding-tests`、`charset-tests`、`character-tests` 通过；文件读写往返完全一致 |
| 1D | 系统面（`1:1`，其中 `systhread.c`/`sysdep.c`/通知类用现代 Rust）：`emacs.c`、`fileio.c`、`filelock.c`、`dired.c`、`process.c`、`callproc.c`、`thread.c`、`timefns.c`、`profiler.c` | `process-tests`、`fileio-tests`、`thread-tests` 通过，含真实子进程 |
| 1E | 显示与终端（内核 `1:1`，字体栈与本地编译现代化）：`xdisp.c`、`dispnew.c`、`window.c`、`frame.c`、`term.c`、`terminal.c`、`font.c`、`fontset.c`、`xfaces.c`、`fringe.c`、`cm.c`、`scroll.c`、`keyboard.c`、`keymap.c`、`minibuf.c`、`menu.c`、`widget.c`、`textconv.c` | `xdisp-tests`、`window-tests` 通过；终端会话与录制帧一致；文本经 HarfBuzz 渲染 |
| 1F | 平台层现代化：Swift（macOS）、`windows-rs`（Windows）、GTK 4/Wayland（Linux/BSD）、Rust + Kotlin（Android）、现代 C++（Haiku） | 各目标平台 GUI 会话能启动，Lisp 可见行为与参照一致，平台冒烟通过 |
| 1G | Dump、模块与工具（`1:1`）：`pdumper.c`、`emacs-module.c`、`lib-src/`（`emacsclient`、`etags`、`make-docfile`、`movemail`、`hexl` 等） | dump 文件、`.elc`、动态模块与参照双向互换 |

1A–1E 按依赖排序；1E 立住之后 1F 与 1G 并行。1F 按设计就是针对当前 API 的重写，所以它的
通过标准是行为验收，而不是逐行比对。

## 4. 加载阶梯

| 阶梯 | 加载内容 | 目标批次 |
| --- | --- | --- |
| L0 | 手写表达式 | 1A |
| L1 | `lisp/emacs-lisp/*.elc` 全部 | 1B |
| L2 | `subr.elc`、`simple.elc`、`loaddefs.elc` | 1C |
| L3 | 完整的 `loadup.el` 预加载集合 | 1D |
| L4 | `make -C test check` 默认选择器 | 1E |
| L5 | 含 `:expensive-test` 的完整套件 | 一期出口 |

## 5. 验证

1. **ERT 跑任意一个二进制。** `test/Makefile.in` 允许命令行覆盖 `EMACS`，因此
   `make -C test EMACS=…/femacs check` 就能用上游套件验收复刻结果。测试套件就是验收
   标准本身，不是替代品。
2. **差分运行。** 同一批 Lisp 表达式分别由参照与复刻在 `--batch` 下求值，stdout、stderr、
   退出码必须逐字节一致。
3. **布局断言。** 生成的 C/Rust 断言覆盖每一个共享结构体；不一致直接构建失败。
4. **二进制互换。** `.elc`、dump 文件、动态模块必须双向可加载。
5. **生成物一致。** 复刻产出的 `admin/unidata`、`admin/charsets`、`src/lisp.mk`、
   `ldefs-boot.el` 必须与参照输出逐字节一致。
6. **现代化层按行为验收**：每个现代化后端的 Lisp 可见行为与参照对比，并在其平台上冒烟。
7. **性能**在功能对齐之后再量。

## 6. 构建整合

工作区用 Cargo 构建；`xtask` 编排原先由参照 Makefile 负责的步骤（loadup、`DOC`、dump、
unidata 与 charset 表）。一个生成的逐文件开关决定 `src/NAME.c` 还是 `femacs/src/NAME.rs`
参与链接，于是整棵树一次一个文件地迁移，两种版本随时都能构建。Rust 文件按独立单元编译
（`--emit=obj`、`-C panic=abort`、单 codegen unit），与剩余的 C 目标文件链进同一二进制、
共用同一套符号。macOS 后端是一个 SwiftPM 包，把 Rust 内核作为静态库链接，并导入由 Rust
接口定义生成的 C 头文件；Android 应用是围绕 Rust 内核的 Gradle 工程。

## 7. unsafe 政策

`unsafe` 是预期之内且被允许的，因为目标就是复现 C 的语义。但它有边界，不是随手用：

- 本质上就不安全的点——标记指针、对象布局、收集器、`setjmp`/`longjmp`、裸缓冲区、FFI
  ——可以自由使用 `unsafe`。
- 每个 `unsafe` 块都带 `SAFETY:` 注释，说明它复现的是哪条 C 不变式。
- 不允许 panic 跨越 FFI 边界（`panic = abort`，不用 `Result` 传递错误）。
- 绝不用 `unsafe` 发明 C 源码里没有的行为。

## 8. 单文件工作流

1. 确定该文件的模式（`1:1 Rust`、`Modern Rust`、`Swift`、`C++`、`Kotlin`、`Drop`）并记入
   对照表。
2. 在 `femacs/…` 以目标语言复刻或重写。
3. 断言：为其声明的结构体补上布局断言。
4. 对 `1:1` 文件做双向构建：开关打开与关闭都必须能链接。
5. 测试：差分运行 + 受影响的 ERT 组。
6. 记录：`python3 tools/gen-port-map.py` 刷新 `docs/port-map.md` 的模式、目标与行数。
7. 提交：签名提交，消息形如 `port(NAME): …`、`modern(NAME): …`、`drop(NAME): …`，并推送。

## 9. 现代化与淘汰台账

| 淘汰对象 | 文件 | 行数 | 替代 |
| --- | ---: | ---: | --- |
| gnulib（`lib/`） | 237 | 69,423 | Rust std、`libc`、`nix`、`windows` |
| Xlib/Xt 后端（`xterm.c`、`xfns.c`、`xselect.c` 等） | 9 | 54,237 | Rust + GTK 4（Wayland） |
| Lucid 部件集（`lwlib/`、`oldXMenu/`） | 48 | 13,529 | GTK 4 |
| NT/MS-DOS 遗留（`nt/`、`msdos/`、`dosfns.c`、`cygw32.c`、`w16select.c`、`w32heap.c`） | 66 | 18,890 | cargo 目标、现行 Win32 |
| Android execve 蹦床（`exec/`） | 8 | 4,274 | 普通 NDK/JNI |
| autoconf 脚手架（`configure.ac`、`m4/`、`build-aux/`、`Makefile.in`、`cross/`、`nextstep/`） | 209 | 41,920 | Cargo、`xtask`、SwiftPM |
| 遗留资源（`macuvs.h`、`emacs-icon.h`、`be_resources.cc`） | 3 | 9,750 | CoreText、资源目录、SwiftPM 资源 |

## 10. 保真优先于现代化

下面这些"看起来旧"是故意的，按原样复刻，因为 Lisp 看得见它们：

| 按参照保留 | 原因 |
| --- | --- |
| `regex-emacs.c` | Emacs 正则语法与回溯语义属于语言的一部分 |
| `bytecode.c` | `.elc` 与 `byte-code` 对象必须保持兼容（computed `goto` 改用 `match` 或内联汇编） |
| `charset.c`、`coding.c`、字符表与 category 表 | 编码系统与字符集决定文件怎么读写 |
| `intervals.c`、`textprop.c` | 文本属性行为对 Lisp 可见 |
| 键映射、undo、marker、overlay | 用户可见语义 |
| `alloc.c` 与收集器、`pdumper.c` | dump 格式与 `garbage-collect` 报告 |
| `emacs-module.c` 与 `module-env-*.h` | 动态模块 ABI 是兼容契约 |
| `term.c`、`termcap.c`、`terminfo.c` | terminfo 仍是 tty 的现行机制 |

## 11. 待定决策

1. Swift ↔ Rust 桥：Swift 侧用手写 C 头文件，还是由 Rust 布局定义生成（当前计划：生成）。
2. GTK 4 后端是否也通过 XWayland 服务 X11 会话，还是只面向 Wayland。
3. gnulib 是否在所有要交付的平台上真的可以淘汰（当前计划：淘汰，由 Rust std 与
   `nix`/`windows` 承担）。
4. `comp.c` 重建在 LLVM 还是 Cranelift 上。
5. 渲染差异（字体整形与栅格化改由平台文本栈负责）是否可接受，即验收按行为而非像素。
6. 1F 已覆盖 macOS、Windows、Linux/BSD、Android、Haiku，一期实际交付哪些平台后端。
