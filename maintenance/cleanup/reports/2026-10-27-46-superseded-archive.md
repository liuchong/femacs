# CLN-20261027-46 · 后继准备归档覆盖后的旧构建归档回收

- 类型：已消费的重复内容归档清理，不删除源码或产品。
- source_ref：`a6e11ed44e8dafb8b158751ad1fbd7668c963a3d`；报告提交由本文件Git历史定位。
- 实际操作：2026-10-27T02:08:14+08:00至2026-10-27T02:08:15+08:00。
- 删除对象：`build/acceptance/runtime-d24-repaired-cohort/artifact-content.tar.gz`，逻辑590461454字节、删前分配590462976字节，SHA256 `bfb575d9fca4fa15f593288dcc52972414178a30f9966bd3b8c421c09caaf0a3`。
- 保留替代：`build/acceptance/runtime-d24-prepare-1/artifact-content.tar.gz`，603244378字节，SHA256 `3db7107d6bbc16be607b7ea62fd64aa62f8d0887a3290a18471dca643be0705b`。

后继prepare已经真实完成32/32项夹具准备，控制器退出0/完整回收。它不证明功能或性能通过，但已产生覆盖原产品内容及新增native夹具的后继恢复包。两个原始保护回执均released、release_failures为空；两个归档均按原回执SHA和大小验证，全部成员流式核对内容SHA/大小，无重复成员、无非普通成员，symlinks清单均为空。旧10770个内容成员全部被新14015个内容成员覆盖，未重新压缩历史缓存。

两次lsof均无活动句柄；逐项清单先file fsync保存，核对旧文件inode/mtime/大小及保留包哈希后只unlink这一文件，删除后保留包哈希及两份原始回执哈希均不变。全部旧产品、原路径映射、权限映射和原始执行日志保留。系统全卷空闲变化不当作本批独占释放量。

[逐项身份与替代清单](../manifests/2026-10-27-46-superseded-archive.json)，SHA256 `a85e6163727dcf3e721554ac4038a7b36a4c51282444b8c46e643fa4eb6cbfb2`。本机执行/验证记录在`build/cleanup/2026-10-27-46`。

**恢复边界更正：** D24构建目录原`artifact-content.tar.gz`已被后继包替代；原保护回执保持历史原样，不能再据其旧路径宣称归档仍存在。恢复原成员时仍用原`artifact-preservation.json`中的路径/mode映射，按内容SHA从保留prepare归档取出，在独占目录验证；保留包有真实恢复消费者，下一次退役必须先完成交接。旧构建的成功、完整性及签名结果不因此被扩写为功能验收。当前直接产品和冻结输入仍在原处，本次不要求重编译或重跑编辑器测试。

[统一台账](../LOG.md)。
