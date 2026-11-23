# 退役的配套脚本（丢弃日志）

本文件是**丢弃记录**：凡是从参照树复制进来、而端口自己的构建/验证并不使用的配套脚本，
在这里逐条登记——路径、它原本干什么、为什么丢、由什么取代、丢弃的日期与提交。
判据来自 `PORTING.md` **P-102**（配套脚本政策：能统一就统一成 Rust `xtask` / Python 两种；
确实必需的驱动器与适配器保留；年代早期、对现代机器已过时的产物直接丢弃并记录）。

以下三节保留2026-10-16历史盘点；当时目录前缀与待办不代表展开后的现状。当前变更见第四节。

逐批可追溯档案统一在 [maintenance/cleanup](../maintenance/cleanup/README.md)，具体编号见 [清理台账](../maintenance/cleanup/LOG.md)。本页继续作为丢弃日志入口，只引用详细报告，不混入用户文档或Agent规则。

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
| 1 | `femacs/lisp/Makefile.in` | 参照的 lisp 目录构建（autoloads/编译/生成物） | `femacs/xtask` + `tools/gen-*.py`（生成物） | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 2 | `femacs/leim/Makefile.in` | 参照的 leim 构建（TIT 转换、`leim-list.el`） | `tools/gen-leim.py` | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 3 | `femacs/admin/unidata/Makefile.in` | 参照的 unidata 构建（`uni-*.el`、`charprop.el`） | `tools/gen-unidata.py` | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 4 | `femacs/admin/charsets/Makefile.in` | 参照的字符集构建（`etc/charsets/*.map`） | `tools/gen-charsets.py` | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 5 | `femacs/admin/grammars/Makefile.in` | 参照的语法下发构建 | 端口不使用（无对应产物） | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 6 | `femacs/doc/emacs/Makefile.in` | 参照的 Emacs 手册构建 | 端口自有的文档工具链（未定，见 P-102 ④） | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 7 | `femacs/doc/lispintro/Makefile.in` | 同上（Lisp Intro） | 同上 | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 8 | `femacs/doc/lispref/Makefile.in` | 同上（Lisp Reference） | 同上 | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |
| 9 | `femacs/doc/misc/Makefile.in` | 同上（其它手册） | 同上 | 2026-10-16 | `245fd341f81c86b6765133b7f333720fb5a52a65` |

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

基线 `809d0de3`；删除提交 `2415a335836d0ad81de2564a82301bdceefb8145`；逐项旧内容身份、依据和恢复见 [CLN-20261024-01](../maintenance/cleanup/reports/2026-10-24-01-obsolete-tools.md)。仅移除已无生产调用者、且有明确替代的脚手架；保留历史证据、许可证、数据与所有平台实现。原始文件可从该基线恢复，不在当前工具目录留下无人维护的可执行副本。

| 路径 | 原用途 / 退役依据 | 取代者 / 保留依据 | 日期 / 提交 |
| --- | --- | --- | --- |
| `tools/verify-dod.sh` | 旧路径/参照依赖的一期聚合复验，当前独立交付已有替代 | `tools/local-acceptance.py`；一期结果仍在历史收尾记录 | 2026-10-24 / `2415a335836d0ad81de2564a82301bdceefb8145` |
| `tools/gen-lisp-mk.py` | 仅被旧聚合脚本调用；生成当前 Cargo/xtask 不读取的 Makefile 变量并强依赖可选参照 | `xtask` 独立资源生成/loadup/dump、`test-repository-bootstrap.py` | 同上 |
| `tools/l1-worklist.sh` | 早期单目录编译统计，失败仍退出0，无当前调用者 | 完整终端/Cocoa 自举及严格 ERT 结果 | 同上 |
| `tools/loadup-fixture.sh` | 早期把参照生成物混入 load-path，无当前调用者 | 独立资源生成和自身 Lisp dump | 同上 |
| `tools/reconcile-link-ratchets.sh` | 转写施工期临时 worktree/include 棘轮辅助，无当前调用者 | 完整配置化终端/Cocoa 链接与自举 | 同上 |
| `spikes/bytecode-hot/bytecode_hot.rs` | 809行已断开的失败 asm 热叶实验，生产构建/运行时零读者 | 保留同目录README及P-325负向实测，后续不重复该状态交接设计 | 同上 |

`bench.sh`、`smoke.sh`、`differential.sh` 收敛为仅转发参数的兼容入口，测量/严格比对由 Python 工具实现，默认使用当前终端路径，证据在独占的 `build/` 子目录。构建/链接继续由 Rust xtask 负责。保留 Swift Cocoa、Kotlin Android、Linux linker、AWK字符集配方、Python资源生成器及原样数据/测试夹具，语言计数不作为退役理由。

仍待明确：支持平台/最低设备版本的退役矩阵；Android配置再生成还依赖可选参照configure，不能称Android已独立自举；历史配置/声明警告与辅助程序打包尚有欠账。没有把这些工作误报完成，也没有删除对应平台。

`check-all-units.sh` 同样收敛为 Rust `xtask check-units [list]` 入口：复用完整构建的普通/checking编译器、拒绝空/重复/越界输入，物化源目录按次隔离，保留链接消费者的对象位置，不再删除共享单元目录。平台聚合门启用pipefail并使用本仓库build scratch；此为失败传播与工具路径修复，不代表Android重新验收。


## 五、清理报告引用（2026-10-24建档）

[清理台账](../maintenance/cleanup/LOG.md) 已登记旧九项脚手架、macOS历史产物、现代化六项退役、零删除检查、未提交VM实验撤回及685项生成物清理。每批有独立报告、来源/动作提交和恢复说明；必要清单/补丁一并提交。原始记录不足之处明确列缺口，不把回填当作当时已有完整证据。

后续诊断候选收敛见 [CLN-20261025-01](../maintenance/cleanup/reports/2026-10-25-01-diagnostic-candidate.md)：保存六份中间源文件与生成候选身份，以主队列实际响应者观察和真实程序/dump 身份取代有时序/入口歧义的诊断；未删除平台或产品功能，不代表 G4 清理已完成。

最终验收工具收敛见 [CLN-20261025-02](../maintenance/cleanup/reports/2026-10-25-02-acceptance-guards.md)：两份工具以完整诊断、制品指纹及参照输出保护替换较弱中间版本，保留原始回执和独立恢复身份；不新增平台退役。

## 六、G4工具翻新施工（2026-10-25，待D验收）

四份逐项档案：[探针/L5/桥接及文档策略](../maintenance/cleanup/reports/2026-10-25-03-probe-tooling.md)、[Android构建与设备策略](../maintenance/cleanup/reports/2026-10-25-04-android-tooling.md)、[既定API21以下分支/类](../maintenance/cleanup/reports/2026-10-25-05-api21-floor.md)、[两份历史raw profile](../maintenance/cleanup/reports/2026-10-25-06-raw-profiles.md)。已写明旧blob/前后哈希与替代入口；真实恢复、编译/运行及独立审核仍待集中D。没有删除其他平台、原始数据或仍被使用的SDK7/11/23实现。历史限制说明保留其日期，不代表当前候选已验收。


G4 Android historical diagnostic stages: `back`, kernel-wchan-only `threads`, temporary-build `probes`, root/debuggerd `bt` and injected `signal` are explicitly retired; old key/rotate/tap/insert-rotate screenshot stages are superseded by strict current editing observations. Focus/log inspection and plan/dump-only/memory/tail remain through the Python owner. Exact before identities/recovery and capability mapping: [CLN-20261025-04](../maintenance/cleanup/reports/2026-10-25-04-android-tooling.md), [current CLI migration](repository-bootstrap.md#android-device-command-migration). No additional native platform is retired.

## 七、Stage II（`GA4`）复核与历史本地清单收口（2026-11-20）

对照 `5118ed08..6065cf1b` 的全部新增工具与脚本复核：上述 6 项退役脚手架继续保持缺席且无回潮引用；`tools/*.sh` 全部 15 个入口维持薄转发器或交叉链接适配器（均不超过 25 行，零嵌套资源执行器）；所有未宣布退役的目标平台（Linux、Windows、Haiku、Android 各 ABI）及原样 Lisp/数据/AWK 配方保持保留。`57101ad7` 本地保留的 `47` 份历史运行级清理清单（`CLN227–CLN269`）与已取代中间产物的逐项 SHA-256、原因、替代与恢复边界统一收口于 [CLN269](../maintenance/cleanup/reports/2026-11-16-269-superseded-single-product-gc-candidate.md) 及 [CLN271](../maintenance/cleanup/reports/2026-11-20-271-historical-manifest-disposition-and-g4-reaudit.md)。

## 八、U1 逐单元对象拼接与归档驱动退役（2026-11-22）

按 P-480 与去 `unsafe` 第一阶段（U1/AU1），主产品与工具构建统一收敛到 Cargo workspace，退役 `xtask` 内部旧逐单元 `.o` 编译与归档驱动（详见 [CLN277](../maintenance/cleanup/reports/2026-11-22-277-u1-cargo-graph-and-au1-retirement.md)）：

| 路径 / 入口 | 原用途 / 退役依据 | 取代者 / 恢复依据 | 日期 |
| --- | --- | --- | --- |
| `xtask` `build-tu` / `build-units` / `unit_codegen_flags` / `LINK_DRIVER` / `libgnu-port.a` / `libgnu-program.a` | 逐翻译单元调用 `rustc --emit=obj`、拼接临时镜像图并手工打包静态库；已被 Cargo 模块图完全取代，无生产读者 | `xtask/src/cargo_build.rs` + `crates/femacs-{pure,sys,vm,host,tools}`；旧实现可从基线 `c9bc7544` 恢复 | 2026-11-22 |
| `xtask/src/macos.rs::make_objects` | 通过参照 Makefile 变量展开查询对象列表的遗留辅助函数，当前仓库零调用者 | `xtask/src/product_sources.rs` + `build-support/macos/*/objects.txt`；可从 `c9bc7544` 恢复 | 2026-11-22 |
| `lib-src/make-docfile.rs` | 历史 C 转写文档与全局表生成器（`46,215` B，SHA-256 `3287575146ef012de482307fd464bd42fc6dbb109191f96c1a5164eb2ea7e652`），当前仓库零调用者 | `tools/gen-doc.py` + `xtask gen-globals`；可由 `git show 4ad0d5f3:lib-src/make-docfile.rs` 恢复 | 2026-11-22 |
| `lib-src/ebrowse.rs` | 历史 C++ 类树离线索引工具转写（`130,178` B，SHA-256 `a4d2a2a1eae753c2aabff3fdb381e9f61864e778d88785f9ba4d55b5ed6bb74d`），未随终端或 Cocoa 打包且零运行消费者 | 编辑器内建 Tree-sitter/xref/eglot；可由 `git show 4ad0d5f3:lib-src/ebrowse.rs` 恢复 | 2026-11-22 |
| `lib-src/etags.rs` | 历史离线 CLI 标签生成器转写（`290,553` B，SHA-256 `6f51a0f214d42ff0bd618982d956f2946fc8e67523ad5f5243a6689a5a11a232`），未随产品打包，`etags-tests.el` 使用已提交 `ETAGS.good_*` 夹具 | 外部 `ctags -e` 或内建 xref/Tree-sitter；可由 `git show 4ad0d5f3:lib-src/etags.rs` 恢复 | 2026-11-22 |
| `lib-src/movemail.rs` | 历史外部邮件池搬运工具转写（`13,254` B，SHA-256 `49a450bb601c3cfd3c76924c5a402e986eef1e233065f2bb269e59e4ddc7b8a4`），未随产品打包且零消费者 | Lisp 层 `lisp/net/pop3.el` 与 `lisp/gnus/nnmail.el`；可由 `git show 4ad0d5f3:lib-src/movemail.rs` 恢复 | 2026-11-22 |
| `lib-src/pop.rs` | `movemail` 的未启用 `MAIL_USE_POP` 后端（`84,117` B，SHA-256 `1910272d6a3167ef0ac1d61a63b39064b28337d81e27b9670b4d55e4e334fde9`），零读者 | `lisp/net/pop3.el`；可由 `git show 4ad0d5f3:lib-src/pop.rs` 恢复 | 2026-11-22 |
| `lib-src/include/pop.rs` | `lib-src/pop.rs` 配套声明头（`5,578` B，SHA-256 `3409f970581b35c397b2cb1cfb06ab0206aa98c3299916cd577ad3d08f49a012`），零读者 | `lisp/net/pop3.el`；可由 `git show 4ad0d5f3:lib-src/include/pop.rs` 恢复 | 2026-11-22 |
| `lib-src/update-game-score.rs` | 共享目录计分辅助程序转写（`28,349` B，SHA-256 `9ab68fa23f07d64f69f002fd1ba1a2fd3ee019a2413d447fbe8cb5ba134f60e2`），产品配置 `PATH_GAME = null` | `lisp/play/gamegrid.el` 直接写入 `user-emacs-directory`；可由 `git show 4ad0d5f3:lib-src/update-game-score.rs` 恢复 | 2026-11-22 |
| `lib/include/str-two-way.rs` | C 宏展开式双路子串搜索模板头（`18,830` B，SHA-256 `0db5a0937e3251c6786c42b67cc1d47e7223d8a960d20f88ea0bc9374f6263d4`），原唯一读者为 `lib/memmem.rs` | `crates/femacs-pure/src/memmem.rs`（`#![forbid(unsafe_code)]`）+ `lib/memmem.rs` 薄桥；可由 `git show 4ad0d5f3:lib/include/str-two-way.rs` 恢复 | 2026-11-22 |
| `tools/check-libshadows.py`（旧 `build-tu` / `link-args.rsp` 实现体） | Cargo 统一前逐单元调用 `xtask build-tu lib/<unit>.c` 并读取 `build/femacs/<unit>.o` 与 `build/load/link-args.rsp` 的系统符号遮蔽门（`4,741` B，SHA-256 `b1f8b1bf551fdf3d580f6d832d7ef52c6e628234f2ff35bbe427237197c56b6f`） | 同路径 Cargo 模块图排除门 + `nm -g` 导出核验实现（含 `--selftest`）；旧体可由 `git show bf09d0fc:tools/check-libshadows.py` 恢复 | 2026-11-22 |
| `tools/pgo_pipeline.py`（旧 `xtask build-units` 与 `/tmp/xt-cargo` 接线） | Cargo 统一前在 `o2`/`generate`/`use` 阶段先调用已退役 `xtask build-units` 并写 `/tmp/xt-cargo` 的旧 PGO 驱动体（`8,386` B，SHA-256 `1120174dca974890dff08fe8f5d8d4c7622c72e9d7e976869fef73b4ae902f40`） | 同路径直接走 `xtask gate-load`（`cargo_build::link_temacs` 内置 `FEMACS_PGO`）+ 受管 `xt-cargo` 目录；旧体可由 `git show bf09d0fc:tools/pgo_pipeline.py` 恢复 | 2026-11-22 |
| `tools/test-u5-au5-integration.py` | `U5` 扩展为八阶段（`U1–U8`）前编写的早期集中整合脚本（`58,666` B，SHA-256 `b5fa636478b746e46ff156b1c4c05aba0fbeba65d88395d8f0472415c9799c08`），硬编码 `build/runtime/u4-native` 且未纳入 `U6–U8` 零 `static mut`/`143` `forbid` 闭合门 | `tools/test-u8-au8-integration.py`（显式 `--native-runtime-root`、`--require-zero-static-mut`、`--min-forbid-files 143`、`--u1-baseline` 棘轮与隔离 `TMPDIR`）；可由 `git show ea54f868:tools/test-u5-au5-integration.py` 恢复 | 2026-11-22 |
