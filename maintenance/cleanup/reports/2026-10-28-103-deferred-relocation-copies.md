# CLN-20261028-103 · 搬移副本延后回收

原搬移验收60项的代码签名、源路径拒绝、编辑/闭包及适用的真实native编译/加载断言全部成功，原程序与搬移包完整身份前后一致；原回执仍为56通过、4失败，失败均为Spotlight读取副本时回收器保留文件。原占用输出及最终退出1不改写。

受控测量树结束并完整回收后，本批重新核验4份归属明确的副本与原产品全树路径/类型/字节/mode/链接一致，核对无打开句柄，按新核验的根身份持久化逐项清单后回收12132个成员/968654987逻辑字节。未终止Spotlight或其他系统进程。原时刻的根设备/inode未另存，故不冒充原根元数据。逐项公开记录见[清单](../manifests/2026-10-28-103-deferred-relocation-copies.jsonl.gz)。

删除和summary已实际完成后，收尾代码因copies字段重名而抛出TypeError，仅最终汇总回执未写出。修复该字段为copy_receipts后，从已保存的4份lifecycle/manifest重新核验原结果哈希、清单SHA和所选路径均不存在，补齐独立回执；没有重复执行删除。原错误和修复边界分别留在本机build/cleanup/2026-10-28-103/receipt-finalization-error.json与deferred-cleanup-completion.json。

原results、completion、case-index、inputs、state-retirement及4份原case result哈希均不变；新的延后回收回执绑定原结果和完成回执SHA。可据此独立复核“功能断言60项成功、后续回收4项完成”，不能将原失败改成首轮全绿，也不新增整体兼容、性能、UI或Goal通过结论。恢复副本直接复制仍保留的原产品，不保证inode/xattr相同。当前原产品、冻结源、原始格式夹具及失败证据保留；逻辑字节不等于独占APFS释放量。
