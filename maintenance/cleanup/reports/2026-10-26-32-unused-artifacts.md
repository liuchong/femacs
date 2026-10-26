# CLN-20261026-32 · 回收已结束运行的编译物与重复副本

- 类型：忽略目录中的生成物与重复测试副本清理。
- 操作与建档日期：2026-10-26；逐次时间见清单。
- source_ref：`227d1ae8bd3b1858d949dfdeedae0cdb6bf3ca1a`；未提交源码另由清单中的三份冻结输入 SHA-256 绑定。
- change_ref：未删除 tracked 文件；报告的签名提交通过本文件 Git 历史定位。

## 范围、依据与保留

用户要求清理本任务用完的构建物和副本。仅处理下表已结束轮次：重复文件必须逐字节哈希匹配保留快照，编译输出必须位于所列旧运行的生成目录。保留当前工作区、独有旧修改、旧失败日志/JSON/回执、原始输入清单、fixture Git 元数据和全部符号链接。没有按整个目录直接删除。

D14 的在用终端产品与原始源快照、D18 的保留 Cocoa 候选、D20 全树及当前恢复必需的 functional-7 源证书全部排除；当前试用包、G4 候选、只读参照检出、安装工具、其他任务均不在删除范围。旧诊断源码或结果不因日期较早就推断无用。

## 逐项证据

[机器清单](../manifests/2026-10-26-32-unused-artifacts.json) 列出全部文件的根索引、相对路径或生成成员角色、内容索引、SHA-256、逻辑/分配字节、mode 和动作。机器路径派生的生成文件名以稳定角色代替；精确路径映射保留在 `build/cleanup/2026-10-26-32/ledger.json`。清单中的内容表同时指定重复文件的保留源；三份保留输入清单另有 SHA-256 固定身份。报告不是只有指向私有目录的引用。

| 旧运行 | 删除文件数 | 逻辑字节 |
| --- | ---: | ---: |
| runtime-d1-frozen | 6910 | 316855913 |
| runtime-d6-products-frozen | 6920 | 316881223 |
| runtime-d8-products-frozen | 13054 | 763376562 |
| runtime-d9-products-frozen | 9994 | 514956652 |
| runtime-d10-products-frozen | 10002 | 535724893 |
| runtime-d11-products-frozen | 6950 | 432970383 |
| runtime-d12-products-frozen | 6952 | 467114926 |
| runtime-d13-products-frozen | 7338 | 554488160 |
| runtime-d15-cocoa-cohort | 11998 | 400187731 |
| runtime-d16-cocoa-cohort | 16640 | 822947222 |
| runtime-d17-cocoa-cohort | 24505 | 1181750945 |
| runtime-d19-cocoa-cohort | 6937 | 338589225 |
| runtime-cocoa-cohort-functional-1 | 1226 | 7341051 |
| runtime-cocoa-cohort-functional-2 | 34052 | 1279836335 |
| runtime-cocoa-cohort-functional-3 | 27742 | 1353346351 |
| runtime-cocoa-cohort-functional-4 | 41613 | 2030080460 |
| runtime-cocoa-cohort-functional-5 | 20809 | 1015262972 |
| runtime-cocoa-cohort-functional-6 | 20815 | 1017107045 |
| runtime-cocoa-cohort-functional-8 | 13872 | 676976630 |
| runtime-terminal-retention-functional-1 | 6938 | 338614157 |

## 执行结果与验证

已删除 **295267** 个普通文件，共 **14364408836** 逻辑字节、删除前 **15056900096** 分配字节。其中 282809 个哈希相同的重复副本占 12480221146 字节；12458 个可重建编译输出占 1884187690 字节。逐根 `lsof +D` 无打开句柄；删除前再核对类型、设备/inode、mtime、大小及哈希，删除后逐项确认不存在。8762 份保护内容哈希及全部被引用的保留原件复核一致。没有为可重建垃圾再创建一份大型压缩副本。

系统可用空间在操作区间增加 13272297472 字节；这是全卷观察，受并发活动/APFS共享块影响，不能当作本批精确物理释放量。清理后仓库 `du -sh .` 为 37G；用户先前观察为 52G，两者同样是目录占用读数。

清理前 D20 所有十组新 Cocoa 构建子进程均已退出，最终一组 exit 0；总控制器 exit 1、已回收，原因是 `product resources changed from build receipt: terminal-package`。该错误在删除前已存在，原 fatal/进程记录仍保留。不能把单组构建成功或本次清理当作完整矩阵、兼容性、GC 或性能验收通过；不在清理任务中偷偷改回执、修复或重建产品。

## 恢复及剩余边界

重复副本：从清单内容表的 `restore_from` 恢复到对应根/相对路径，先核对 SHA-256、字节和 mode；生成成员角色通过私有精确路径台账还原。普通文件内容可逐字节恢复，不承诺原 inode、硬链接拓扑或扩展属性。保留源仍被验收使用，今后删除它们前必须先重新确认恢复依赖。

编译物：使用保留的对应输入清单、独有冻结源、配置、构建日志和工具链身份，在 `build/recovery/CLN-20261026-32/` 隔离恢复源后，按原参数重跑 `cargo run -p xtask -- runtime-build --product ... --variant ... --out ...`；编译/自编译程序必须走当前资源执行器。原 observer/test-hooks/native/collector 参数取旧回执；不把省略号直接当命令执行。缓存只承诺可重建，不承诺与旧机器缓存逐字节相同。此批没有重新编译，也没有重跑产品验收。

[清理台账](../LOG.md) 记录本批。以后每轮隔离运行终止并提取必要证据后立即做清理检查；继续使用中的候选先保留，淘汰后再以新编号清理。
