# CLN-20261024-01 · 六项旧工具和断开实验源码退役

报告建立日期：2026-10-24（历史回填）；本次只建档，不再次执行原删除。

- 实际操作日期：2026-10-24，精确删除命令时间未记录。
- `source_ref`：`809d0de38b83b2f779c8576bc109788da75241a9`。
- `change_ref`：`2415a335836d0ad81de2564a82301bdceefb8145`；六项 tracked 删除与替代工具/测试/文档同批。
- [逐项旧内容清单](../manifests/CLN-20261024-01.tsv)：6 路径、Git blob、SHA-256、字节与 source ref。

## 每项依据和替代者

| 路径 | 原用途与退役原因 | 替代者/保留证据 |
| --- | --- | --- |
| tools/verify-dod.sh | 一期旧路径/参照依赖聚合门，当前独立交付已有替代 | tools/local-acceptance.py；一期历史结论保留 |
| tools/gen-lisp-mk.py | 仅供旧聚合门生成当前构建不消费的 Makefile 变量 | xtask 独立资源生成及 tools/test-repository-bootstrap.py |
| tools/l1-worklist.sh | 早期编译统计，失败退出语义不可靠且无当前调用者 | 完整自举及严格 ERT 结果 |
| tools/loadup-fixture.sh | 曾把参照生成物混入 load-path，无当前调用者 | 独立资源生成、自身 dump |
| tools/reconcile-link-ratchets.sh | 已完成转写期的临时 worktree/include 棘轮助手，无当前调用者 | 配置化终端/Cocoa 链接与自举 |
| spikes/bytecode-hot/bytecode_hot.rs | 809 行未链接的旧失败 asm 热叶实现 | 同目录 README/P-325 负向结果保留；未重新声称所有 VM 优化不可行 |

四个 shell 入口的转发收敛和对象命名修复属于同批**修改**，不是上述六条删除；可用 `git show --stat 2415a335836d0ad81de2564a82301bdceefb8145` 区分。平台/设备支持未明确退役，Swift/Kotlin、AWK 原始配方、Python 生成器、原始数据和测试夹具保留。

## 验证与风险

同批严格测量/差分驱动组件14/14、新增边界1/1、本地驱动8/8、Rust 单元驱动4/4；真实普通/checking与目标后缀对象检查通过。95条参照差分通过；首次无效输入和接口交叉失败仍保留。这里只引用当时证据，不重新跑产品，也不由此证明全 ERT、所有平台或 native/JIT/GC 完成。详见 [阶段 II 记录](../../../docs/modernization.md#stage-ii-exact-tooling-cleanup-design) 和 [进度日志](../../../docs/status.md)。

## 恢复和排障

例如 `git show 809d0de38b83b2f779c8576bc109788da75241a9:tools/verify-dod.sh > build/recovery/CLN-20261024-01/verify-dod.sh`（先建目录），然后按清单核对 SHA-256。按旧路径逐项恢复，不把已失效的脚手架自动放回活动入口；先对照替代者、配置和失败传播语义。Git 可精确恢复旧源码，不代表旧方案现在可用。源码大小不是磁盘净释放量。
