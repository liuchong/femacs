# CLN-20261027-77 · 恢复构建完成配置的生成副本

基线 `954dba25`。D28独立恢复使用原冻结源码与同哈希驱动，
`terminal-baseline` 的normal/checking构建、链接及dump完成；实际配置进程
退出0、complete=true、无剩余进程组。原首配置按完整产品及链接输入验证后复用。
此时共2/20配置、7/61 profile，不代表完整功能或性能验收。

既有retirement helper只选择本配置两腿 `objects/units` 中的生成副本，
逐项身份与哈希、原构建回执、空句柄检查、持久清单及父目录fsync完成后删除。
共9302份，309409362逻辑字节、
删前分配327954432字节；所选路径复核均不存在。
程序、dump、资源、对象/归档、配置和原始失败证据不在选择范围内。

[逐项身份清单](../manifests/2026-10-27-77-recovered-baseline-copies.jsonl.gz)，
SHA256 `79fd9e5370757c0efbd9dbc38c95eb900a28f7d0980c916de33076b8b373e65a`。
按configuration/leg和相对路径的SHA定位；路径明文与inode等操作证据留在
本机 `build/cleanup/2026-10-27-77` 及原两腿retirement private账本，
避免发布编译器可能编码的机器路径。

恢复时从保留的D28冻结输入、配置和驱动重生成，不保证随机副本名称重现；
不归档另一份全量缓存。恢复前ENOSPC失败仍保留；本批未执行功能测试，
也不据同期系统空间差额宣称独占释放量。

[清理台账](../LOG.md)
