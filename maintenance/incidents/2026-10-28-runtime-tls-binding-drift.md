# D28恢复在首个Cocoa配置的TLS绑定差异

D28原磁盘中断后的独立恢复实际完成10个终端配置、30个终端profile及
1个安装参照profile。进入 `cocoa-native-collector-observer-diagnostic`
时，normal/checking均报 `Cocoa continuation configuration changed`，
该配置进程退出1、完整回收；外层恢复也退出1，剩余Cocoa矩阵未执行。
这是配置绑定错误，不是新的磁盘或内存中断，不能报告整个构建通过。

原始对比显示：两腿 `runtime-config.json` 与 `config/darwin-cocoa.json`
各相差37个TLS能力/版本字段。真实调用顺序为 prepare_leg先绑定头/库
身份、计算配置和native ABI哈希，再由tls::prepare补这些字段。
终端使用同一后置解析，但旧校验未比较其target副本，因此终端完成
也不等于当前矩阵的配置身份完整。保留旧成功构建和错误作为历史证据，
不修改旧回执、补假通过或直接复用为修复后的最终候选。

P405一次修复：在bind_configuration统一解析真实头的全部能力/版本字段，
先于两个产品的配置/native ABI哈希；prepare重解析并拒绝绑定后漂移；
terminal/cocoa均校验其真实目标副本，保留19头/库身份验证和严格hash。
实现已写入当前工作区，尚未编译或完成集成验收，独立搭档正在只读核查。
因两产品hash/ABI输入均受影响，后续从新统一冻结源重建20配置/61profile。
功能套件已写齐；完整功能通过后才补保护性单测，不降低兼容或性能标准。

本次恢复的外层原生报告：elapsed4144.992秒，exit1，reason=exited，
cleanup_complete=true；整树RSS峰值521338880、footprint493013464、
按较大值计量峰值521338880字节，无扩容。该报告属于本次恢复，
不能替代原ENOSPC运行缺失的最终报告或补造它的峰值。

原始source/product/config/输入、逐进程日志和完整控制器报告只在本机
验收证据目录保留，原始会话/进程身份不发布。相关生成副本操作见
[CLN82](../cleanup/reports/2026-10-28-82-recovered-terminal-copies.md)。
整个Goal仍只交付G4，1/7，余6/7；当前失败不停止其余有权限工作。
