# 退役的配套脚本（丢弃日志）

本文件是**丢弃记录**：凡是从参照树复制进来、而端口自己的构建/验证并不使用的配套脚本，
在这里逐条登记——路径、它原本干什么、为什么丢、由什么取代、丢弃的日期与提交。
判据来自 `PORTING.md` **P-102**（配套脚本政策：能统一就统一成 Rust `xtask` / Python 两种；
确实必需的驱动器与适配器保留；年代早期、对现代机器已过时的产物直接丢弃并记录）。

以下三节保留2026-10-16历史盘点；当时目录前缀与待办不代表展开后的现状。当前变更见第四节。

## 一、语言分类（2026-10-16 清点，端口树内）

| 语言 | 件数 | 位置 | 决定 | 理由 |
| --- | --- | --- | --- | --- |
| Rust | 629 | `femacs/**/*.rs` | **保留（主语言）** | 唯一的构建驱动器 `femacs/xtask` 与全部 1:1 单元 |
| Python | 62（`tools/`+`spikes/`） | 生成物、比对、清单、门 | **保留（唯一的工具语言）** | P-102：新工具一律 Python，不再引入新语言 |
| Shell | 18 | `tools/*.sh`、CI | **保留** | 只用于与参照配方/make 的接口（`make -B -n` 取配方、跑配方） |
| awk | 10 | `femacs/admin/charsets/*.awk` | **保留（适配器）** | 它们是**参照配方本身的输入**（字符集转换的定义），换成别种语言会改变输出的字节，而输出的逐字节一致正是验收项 |
| Perl / JS / Lua / m4 | 28+8+8+1 | 全部在 `femacs/test/**` | **保留（测试夹具）** | 它们是 cperl/js/lua/m4 模式测试的**输入数据**，不是构建脚本；删掉会让 L5 的 ERT 套件失去夹具 |
| Makefile.in / configure.ac | 13（其中 4 在 `test/`） | `admin/*`、`doc/*`、`leim/`、`lisp/`、`test/` | **丢弃 9 / 保留 4** | 见下 |

## 二、丢弃记录

判据：该文件属于参照的 **autotools 构建脚手架**（`docs/port-map.md` 自己就写着
“`Makefile.in` … Cargo and xtask replace it”），端口自己的构建（`femacs/xtask`）与验证
（`tools/`）都不读它；`femacs/test/**` 下的 4 件是测试夹具/测试套件的驱动，按 P-102 保留。

| # | 路径 | 原本用途 | 取代者 | 日期 | 提交 |
| --- | --- | --- | --- | --- | --- |
| 1 | `femacs/lisp/Makefile.in` | 参照的 lisp 目录构建（autoloads/编译/生成物） | `femacs/xtask` + `tools/gen-*.py`（生成物） | 2026-10-16 | 本提交 |
| 2 | `femacs/leim/Makefile.in` | 参照的 leim 构建（TIT 转换、`leim-list.el`） | `tools/gen-leim.py` | 2026-10-16 | 本提交 |
| 3 | `femacs/admin/unidata/Makefile.in` | 参照的 unidata 构建（`uni-*.el`、`charprop.el`） | `tools/gen-unidata.py` | 2026-10-16 | 本提交 |
| 4 | `femacs/admin/charsets/Makefile.in` | 参照的字符集构建（`etc/charsets/*.map`） | `tools/gen-charsets.py` | 2026-10-16 | 本提交 |
| 5 | `femacs/admin/grammars/Makefile.in` | 参照的语法下发构建 | 端口不使用（无对应产物） | 2026-10-16 | 本提交 |
| 6 | `femacs/doc/emacs/Makefile.in` | 参照的 Emacs 手册构建 | 端口自有的文档工具链（未定，见 P-102 ④） | 2026-10-16 | 本提交 |
| 7 | `femacs/doc/lispintro/Makefile.in` | 同上（Lisp Intro） | 同上 | 2026-10-16 | 本提交 |
| 8 | `femacs/doc/lispref/Makefile.in` | 同上（Lisp Reference） | 同上 | 2026-10-16 | 本提交 |
| 9 | `femacs/doc/misc/Makefile.in` | 同上（其它手册） | 同上 | 2026-10-16 | 本提交 |

保留（同样是 `Makefile.in`，但不是构建脚手架）：`femacs/test/Makefile.in`、
`femacs/test/infra/Makefile.in`、`femacs/test/lisp/emacs-lisp/package-vc-resources/Makefile.in`、
`femacs/test/manual/noverlay/Makefile.in`（测试套件驱动与夹具）。

## 三、待办（本政策下已点名、尚未处理）

- `femacs/doc/**` 的手册构建链（`Makefile.in` 之外还有 `*.texi` 的生成规则）：端口需要一条
  自己的文档路线，P-102 ④ 记着；在它落地之前，`doc/` 的**内容**照旧逐字保留，只丢了脚手架。
- 参照树里 `admin/` 下还有 `.sh`/`.pl`/`sed` 组成的辅助脚本（如 `update_autogen`、
  `unidata-gen` 的数据前处理）：它们**不复制进端口**（端口由 `tools/gen-*.py` 直接调用参照的
  只读配方），因此没有"丢弃"动作，只在此点名。


## 四、现代化工具退役（2026-10-24）

基线 `809d0de3`；本节所属提交可由 Git 路径历史定位。仅移除已无生产调用者、且有明确替代的脚手架；保留历史证据、许可证、数据与所有平台实现。原始文件可从该基线恢复，不在当前工具目录留下无人维护的可执行副本。

| 路径 | 原用途 / 退役依据 | 取代者 / 保留依据 | 日期 / 提交 |
| --- | --- | --- | --- |
| `tools/verify-dod.sh` | 旧路径/参照依赖的一期聚合复验，当前独立交付已有替代 | `tools/local-acceptance.py`；一期结果仍在历史收尾记录 | 2026-10-24 / 本节所属提交 |
| `tools/gen-lisp-mk.py` | 仅被旧聚合脚本调用；生成当前 Cargo/xtask 不读取的 Makefile 变量并强依赖可选参照 | `xtask` 独立资源生成/loadup/dump、`test-repository-bootstrap.py` | 同上 |
| `tools/l1-worklist.sh` | 早期单目录编译统计，失败仍退出0，无当前调用者 | 完整终端/Cocoa 自举及严格 ERT 结果 | 同上 |
| `tools/loadup-fixture.sh` | 早期把参照生成物混入 load-path，无当前调用者 | 独立资源生成和自身 Lisp dump | 同上 |
| `tools/reconcile-link-ratchets.sh` | 转写施工期临时 worktree/include 棘轮辅助，无当前调用者 | 完整配置化终端/Cocoa 链接与自举 | 同上 |
| `spikes/bytecode-hot/bytecode_hot.rs` | 809行已断开的失败 asm 热叶实验，生产构建/运行时零读者 | 保留同目录README及P-325负向实测，后续不重复该状态交接设计 | 同上 |

`bench.sh`、`smoke.sh`、`differential.sh` 收敛为仅转发参数的兼容入口，测量/严格比对由 Python 工具实现，默认使用当前终端路径，证据在独占的 `build/` 子目录。构建/链接继续由 Rust xtask 负责。保留 Swift Cocoa、Kotlin Android、Linux linker、AWK字符集配方、Python资源生成器及原样数据/测试夹具，语言计数不作为退役理由。

仍待明确：支持平台/最低设备版本的退役矩阵；Android配置再生成还依赖可选参照configure，不能称Android已独立自举；历史配置/声明警告与辅助程序打包尚有欠账。没有把这些工作误报完成，也没有删除对应平台。

`check-all-units.sh` 同样收敛为 Rust `xtask check-units [list]` 入口：复用完整构建的普通/checking编译器、拒绝空/重复/越界输入，物化源目录按次隔离，保留链接消费者的对象位置，不再删除共享单元目录。平台聚合门启用pipefail并使用本仓库build scratch；此为失败传播与工具路径修复，不代表Android重新验收。
