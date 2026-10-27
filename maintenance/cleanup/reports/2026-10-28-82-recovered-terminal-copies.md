# CLN-20261028-82 · 恢复构建终端配置副本清理汇总

基线 `9eae9f85`。D28恢复的十个终端配置normal/checking均实际退出0、
complete=true、无剩余进程组；本批汇总其中尚未发布的八个配置两腿。
首个native-collector-observer-diagnostic已在CLN75、baseline已在CLN77
记录，均不重复计数。本次发布工具没有再次执行删除。

原恢复协调器在各配置完整构建、链接、dump及产品保护交接后，分别以
实际process回执调用既有retirement helper。只选择 `objects/units` 的
生成Rust副本；原逐项身份/哈希、空句柄检查、清单与父目录fsync完成后
才unlink。共74416份、2478845236逻辑字节，删前分配
2628767744字节。发布时重核各腿构建回执、原清单SHA、
私有路径映射和选定路径不存在；不把当前Cocoa范围纳入已完成清理。

[逐项内容/路径身份与scope](../manifests/2026-10-28-82-recovered-terminal-copies.jsonl.gz)，
SHA256 `e4c65a5a31227afa9da71f1fcda3bc2624c5ef6d5c3adc04e1f9e1ee657c50c1`。
冻结输入清单 `73c1df696064499d215aabb7a76decfa3a272dcee25ac996ffc220980b199884`。
原路径与inode等操作证据留在各腿原retirement private账本，本机汇总
`build/cleanup/2026-10-28-82` 绑定16个原构建/清理回执和产品回执SHA。
公开清单使用configuration/leg及相对路径SHA，避免编译器编码的机器路径。

源、配置、对象/归档、产品、资源和原始失败日志未列入本批副本删除范围；
按保留源/配置/驱动可重生成，不保留另一份全量缓存，不承诺随机名称相同。
不根据系统同时发生的空闲变化推算本批独占释放量。

随后首个Cocoa配置因P405所述TLS后置配置差异失败，整个恢复退出1。
十个终端构建完成不能证明配置绑定正确、完整功能/兼容/性能通过，
也不作为修复后新统一矩阵的验收候选；详见
[实际配置失败](../../incidents/2026-10-28-runtime-tls-binding-drift.md)。
本批没有执行语义测试，没有改变整个Goal计数。

[清理台账](../LOG.md)
