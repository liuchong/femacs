# CLN-20261027-53 · 已消费的签名恢复与搬移测试副本

- 类型：已确认归属、无消费者的生成物清理。
- 实际操作：2026-10-26T21:41:30.122129+00:00 至 2026-10-26T21:41:32.588430+00:00。
- 基线source_ref：`a3e1c93ca15a5ac10031f1165adf33ba6ced203c`；忽略产物不产生tracked删除提交。报告提交由本文件Git历史定位，不代表运行时WIP已交付。

## 范围和依据

回收build/recovery/runtime-d20-exact-resource-1中三份signed-reconstruction-N/Femacs.app，以及g4-macos-products-restored-2/relocated/Femacs.app。它们是已经结束的诊断/搬移工作副本；当前Mac试用App、G4交付候选、原始结果、签名日志与恢复回执全部保留。D18/D20旧生成原产品已由CLN36退役，不应无限保留其恢复实验App。

## 实际结果

删除11963个普通文件，逻辑615402920字节、删前分配644128768字节；系统空闲空间变化未作独占归因。全部选定文件/工作包已消失。执行前逐项哈希并完成两次无句柄核对；清单及其目录在删除前fsync，删除后同步父目录。归属路径无tracked文件、无符号链接跳转；未删除其他目录。

[删前逐项身份与恢复边界](../manifests/2026-10-27-53-consumed-app-copies.jsonl.gz)，SHA256 `15579e95a78d7077fcbed20b667995dc3975521a366fdefde8b78d29aee100dd`。实际本机操作/占用/删后记录在`build/cleanup/2026-10-27-53`。

15项保护文件哈希一致，覆盖原恢复JSON、原codesign日志、G4完整通过回执、当前试用/交付程序和Android当前制品。三份原恢复回执均记录exact_program_match与full_bundle_matches_original_receipt；G4原回执passed。它们是历史执行证据，不意味着退役副本现在仍存在。

## 恢复与验收边界

搬移副本2983个文件逐项SHA等同原source-export/dist/macos/Femacs.app，清单restore_from给出相对恢复源；复跑时先在新隔离目录重建副本。三份旧恢复App仅承诺按原冻结输入/配置重新构建，不能保证旧签名及二进制字节完全一致；按CLN36与CLN45的映射补齐已退役生成物/重复输入后重建，不盲跑旧owner。

本批未运行新编译、产品测试或远端CI，没有将历史失败改写成通过。当前D24/D25的产品/输入/实际运行不在删除范围，真实界面、完整兼容性与性能验收状态不变。

每轮结束按消费者及时回收，不把目录名或历史保留清单当永久豁免；相关规则与本轮其他报告见[清理台账](../LOG.md)。
