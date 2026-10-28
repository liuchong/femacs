# CLN-20261028-102 · 已消费的dump试验图像

实际dump保存/独立进程恢复48/48通过、完整完成回执已核验后，仅回收80份逐例runtime-image.pdmp与dump-native.eln，共836810968逻辑字节。每份SHA严格匹配原成功结果中的image或native_unit_after_restore字段，lsof确认整个输出无打开句柄；本机保留逐项inode/出生时间/mode/大小/SHA与删前同步操作意图。公开身份见[清单](../manifests/2026-10-28-102-consumed-dump-images.json)。

原始dump输入、编辑器、20配置/61profiles资源和7099项冻结源不动；后续搬移/性能调用原产品dump，不读取本批逐例试验图像。所有输入、请求、stdout/stderr、进程结果、完整JSON、索引与完成/清理回执均保留，删前删后逐份元数据SHA相同。恢复需由保留源和dump夹具重新运行，原图像随机内容不保证逐字节复现；新运行不能冒充旧回执。未删除608份原始格式夹具，未删除失败证据；本批不新增或撤销通过结论，整体验收仍未完成。逻辑字节不等于独占APFS释放量。
