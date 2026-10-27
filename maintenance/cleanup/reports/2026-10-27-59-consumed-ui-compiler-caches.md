# CLN-20261027-59 · UI 修复完成后的已消费编译缓存

- source_ref：`8c7d2d41`；只清本次已退出的 Cocoa 构建缓存，不修改源码。
- 精确范围：`build/acceptance/g1-layout-cocoa-build-4`、`g1-layout-cocoa-build-5`、`g1-layout-cocoa-build-6` 下的 cargo 目录，以及第4轮普通/checking的 `source/build/runtime/cocoa-layout/objects/units` 生成副本。
- 三次构建进程均已退出0并完整回收，原有制品保全记录为 released。第5轮后续UI运行曾真实崩溃，该失败和系统报告保留，不能用构建通过覆盖运行失败。

实际删除10639份文件，436824308逻辑字节、458670080删前分配字节。26979项冻结源码、输入清单、构建日志/回执、恢复包以及当前试用App和前版回退App内容在删除后逐项SHA不变。新候选、原始崩溃诊断和对象根目录不在删除范围。

两轮逐目录 lsof 均无句柄；逐文件记录路径ID、SHA、大小、权限，持久化并同步清单后，重新核对OS身份与内容再删除。全部所选路径已不存在；遇权限错误则停止，本批未发生权限错误。机器路径/OS身份仅留在 `build/cleanup/2026-10-27-59/private-ledger.json`。

公开[逐项身份清单](../manifests/2026-10-27-59-consumed-ui-compiler-caches.jsonl.gz)使用路径SHA；清单SHA256 `4e9248758a196f387c53af6fdbadb0db2982007310b73d8f6c54474fbd9b7ac7`。恢复时按各轮保留的冻结源、Cargo.lock、配置和原命令重新生成缓存，不承诺随机生成文件名字节相同。保留的源码/归档及构建驱动仍能定位，日志不依赖所删缓存。

本批不重跑产品，不运行远端CI，也不把UI修复解释成原GC问题已解决。

[统一台账](../LOG.md) · [清理方法](../README.md)
