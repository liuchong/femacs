<p align="center"><img src="../../../packaging/assets/fe-logo.png" width="180" alt="Fe — 铁质 F 与竖向 GNU 牛角"></p>

# Fe

[English](../../../README.md) · **简体中文**

用 Rust 重写并加速 Emacs 的编辑器项目。

[![crates.io femacs](https://img.shields.io/crates/v/femacs?logo=rust&label=femacs)](https://crates.io/crates/femacs)
[![crates.io elisp](https://img.shields.io/crates/v/elisp?logo=rust&label=elisp)](https://crates.io/crates/elisp)
[![crates.io lisp](https://img.shields.io/crates/v/lisp?logo=rust&label=lisp)](https://crates.io/crates/lisp)
[![crates.io lisp-std](https://img.shields.io/crates/v/lisp-std?logo=rust&label=lisp-std)](https://crates.io/crates/lisp-std)
[![crates.io lisp-vm](https://img.shields.io/crates/v/lisp-vm?logo=rust&label=lisp-vm)](https://crates.io/crates/lisp-vm)
[![npm femacs](https://img.shields.io/npm/v/femacs?logo=npm&label=npm)](https://www.npmjs.com/package/femacs)
[![许可证：GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](../../../LICENSE)
[![项目文档](https://img.shields.io/badge/docs-femacs.com-blue)](https://femacs.com/)

## 名字的来历

`Fe` 是铁的元素符号，而 `Rust` 本义就是铁锈——铁锈由铁而生，两者互为表里。
把一把用惯了的编辑器重新回炉，用 Rust 再锻打一遍，正是这个项目想做的事。

## 目标

- 用 Rust 重写编辑器内核，逐步替掉 Emacs 里历史包袱最重的部分
- 保留 Emacs 真正值得保留的东西：Lisp 扩展能力、包生态、用户既有的配置习惯
- 让启动时间、编辑延迟与批量操作提升一个数量级，用真线程而不是绕路方案
- 把文本编辑当成系统问题来做：增量解析、异步 I/O、原生渲染

## 现状

Rust 内核及原生 Swift macOS 界面已有可运行实现；功能验收与性能目标分开，不声称性能已达标。

同时，项目已将零 C 编译依赖、No-GIL 多核并行的独立通用 Lisp 运行时与命令行拆分为四个独立 Crate（`v0.0.1`）发布：[`elisp`](../../../crates/elisp/README.md)、[`lisp`](../../../crates/lisp/README.md)、[`lisp-std`](../../../crates/lisp-std/README.md) 与 [`lisp-vm`](../../../crates/lisp-vm/README.md)。

## 独立通用 Lisp 运行时与命令行（`elisp` / `lisp`）

- **[`elisp`](../../../crates/elisp/README.md)**（`[[bin]]`，`0.0.1`）：独立 CLI、脚本运行器、交互式 TTY REPL（`elisp> ` 提示符）、`init`/`pkg` 项目包管理、字节码编译与 ERT 测试入口（`cargo install elisp`）。
- **[`lisp`](../../../crates/lisp/README.md)**（`[lib]`，`0.0.1`）：面向 Rust 应用的一站式可嵌入库（`default = ["vm", "std"]`，正交 feature `vm` / `std` / `pkg`，支持 `default-features = false`），导出 `lisp::prelude::*`、`lisp::vm` 与 `lisp::stdlib`。
- **[`lisp-std`](../../../crates/lisp-std/README.md)**（`[lib]`，`0.0.1`）：100% 纯 Rust（`#![forbid(unsafe_code)]`，零 `cc` / C `-sys` 依赖）通用标准库电池（22 个 `std/*` 模块：HTTPS/HTTP 客户端与服务端、`redb` ACID KV 与事务 SQL 引擎、JSON/TOML/CSV/YAML/Base64/URL、tar/zip/gzip/zlib、密码学/UUID、时间、`walkdir`/glob 文件、正则、受限子进程、CLI、结构化日志）。
- **[`lisp-vm`](../../../crates/lisp-vm/README.md)**（`[lib]`，`0.0.1`）：无头 Lisp 虚拟机、GC、字节码引擎、`HostExtFn` 回调、`.pdmp` 热启动镜像及可选 `isolate_vm` 多核隔离。

```sh
# 预编译多平台分发（含 elisp / femacs / fe）
brew install liuchong/tap/femacs
npm install -g femacs
curl -fsSL https://femacs.com/install.sh | sh

# 或使用 Cargo 从源码构建
cargo install elisp
elisp eval '(+ 20 22)'
elisp repl
```

平台验证与预编译边界（`v0.0.1`）：Apple Silicon macOS（`aarch64-apple-darwin`）与 Linux `aarch64`（Debian 12 虚拟机实测 + Zig 交叉发布包）已完成运行态验证；`x86_64-apple-darwin`（已完成 `--release` Mach-O 与 `otool -L` 仅系统库检查，本机无 Rosetta/Intel 实机运行）与 `x86_64-unknown-linux-gnu`（已完成 Zig 交叉 `--release` ELF 与 `cargo check`）提供 GitHub Release 预编译压缩包（`SHA256SUMS.txt`）；Windows（`x86_64-pc-windows-msvc` / `x86_64-pc-windows-gnu`）**仅完成交叉编译架构验证，未在物理 Windows 主机上执行运行态测试**，暂不提供预编译包。详见[支持矩阵](../../support-matrix.md)与 [npm 包说明](../../../packaging/npm/femacs/README.md)。

## 构建与试用 Fe 编辑器

在 Apple Silicon macOS 安装 Xcode 命令行工具、Python 3、rustup 和 GNU awk（`brew install gawk`）后，在仓库根目录执行：

```sh
cargo run -p xtask -- build-terminal
build/terminal/port/src/emacs -Q -nw
cargo run -p xtask -- build-macos
dist/macos/Femacs.app/Contents/MacOS/Femacs -Q
```

源码已经提升至根目录。构建从已提交的原始数据和生成器自举，不依赖被忽略的参照目录、旧缓存或已安装的编辑器。详情见[独立构建与验收](../../repository-bootstrap.md)。

### macOS 命令行入口

构建或安装 Cocoa 应用后，在仓库根目录运行 `cargo install --locked --path cli`，同一个包会安装两个命令。

- `femacs notes.org` 将参数原样交给 Fe。用 `femacs -nw notes.org` 进入原生终端模式。
- `fe notes.org` 使用快捷终端预设。`fe -1 a.txt b.txt` 逐个打开文件，`fe -R -nw notes.org` 清掉预设后按原生参数启动。

两条命令都需要 Cocoa 应用。应用若在常用的 Applications 目录之外，用 `FEMACS_APP` 指定应用包路径。详见[命令行入口说明](../../../cli/README.md)。

## 源码布局

`src/`、`lib/`、`lib-src/`、`platform/`、`cli/`、`packaging/` 与 `xtask/` 位于仓库根目录，独立/分层 Crate 位于 `crates/`（`lisp-vm`、`lisp-std`、`lisp`、`elisp`、`femacs-host`、`femacs-tools`、`http-host`）；`lisp/`、`leim/`、`admin/`、`etc/` 保留原始数据与生成器；`docs/` 为项目文档与网站源码。

## 文档

- [项目主页（femacs.com）](https://femacs.com/)（[源码](../../index.html) · [独立 Lisp 专页](https://femacs.com/pages/lisp.html) · [Shell 安装脚本](../../install.sh)）
- [独立 Lisp 运行时架构设计](../../design/lisp-core.md)
- [多渠道统一发布流程（`xtask release`）](../../release-process.md) — 覆盖 5 个 Cargo crate、预编译压缩包、[npm `femacs`](../../../packaging/npm/femacs/README.md)、[Homebrew Tap](../../../packaging/homebrew/femacs.rb) 与 `install.sh`
- [平台支持与验证边界矩阵](../../support-matrix.md)
- [docs/i18n](../../i18n/README.md)（语言索引）：其他语言的译文统一放在 `docs/i18n/<locale>/` 下，路径与英文原文一一对应。

## 许可证

femacs 采用 **GNU General Public License 第 3 版或更新版本**（`GPL-3.0-or-later`）。
完整条款见 [LICENSE](../../../LICENSE)。
