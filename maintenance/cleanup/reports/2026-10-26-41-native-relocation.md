# CLN-20261026-41 · native搬移验收逐例回收

固定驱动/夹具在原D23 terminal-native-normal及cocoa-native-normal上实际通过源码与原后端路径不可读搬移。两例均验证原始源码/程序/dump及后端库确被拒绝，严格深层签名通过；每例实际执行语言闭包、真实native文件编译/加载/函数身份及native cons读写用例。原包和复制包身份保持不变。不是检查版或全部native/GC配置通过。

每例确认所属子进程退出及精确包根inode、无打开句柄后，在包外保存逐成员身份和回收回执，随即删除该独占副本；两包共6017成员、424945610逻辑字节、删前分配439365632字节，工作包及空case根均已不存在。原始源码/产品/资源/链接输入、运行回执、签名/拒绝访问及用例证据不动；没有继续堆积native整包副本。

[逐项压缩清单](../manifests/2026-10-26-41-native-relocation.jsonl.gz)全部行数/逻辑及分配字节、相对路径已复核，SHA256为`25941228b9751ae0934b0ace73f306bb22e8f7c5fd3a6d7eb90b632ed9c827e4`。私有计划/统计在`build/cleanup/2026-10-26-41`，每例原始回执和清单在`build/acceptance/runtime-d23-fixed-relocation-native-diagnostic-1/actual`。需要时从绑定原包在新独占目录复制并核对身份，不承诺原inode/xattr或已删除旧路径的即时可运行状态。

原控制器0、82.125秒、accounted峰值383352832字节、完整回收，记录保护释放零失败。原产品/fixture身份不变。旧失败仍保留；ROOT collector更正未被这些原产品覆盖，全部61配置、完整兼容/性能及异常清理保护仍待验证。source_ref：`a2f97b2b933a4c56d5879d53d183f1405dd44f3d`；报告签名提交由本文件Git历史定位。[统一台账](../LOG.md)。
