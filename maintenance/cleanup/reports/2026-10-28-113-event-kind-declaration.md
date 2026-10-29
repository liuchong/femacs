# CLN-20261028-113 · 文件通知事件号的旧局部声明

移除kqueue单元内冻结为28的局部FILE_NOTIFY_EVENT声明，改读共享termhooks
的同名按配置计算成员。启用线程及Cocoa时枚举编号会改变，局部旧值既会
冲突，也不能保证回调被正确派发；没有删除文件或通知功能。

[清单](../manifests/2026-10-28-113-event-kind-declaration.json)绑定旧Git提交/blob、
修改前后散列及被移除块散列。替代者为既有共享声明；恢复仅取旧blob中的
声明块并同步其旧配置，不能覆盖其他修改或把硬编码恢复到新配置。

两种macOS配方同时接入后端及对象。当前只完成正式接线，ABI、真实文件
操作、原版用例及新产品验收待执行；不能把此前单宏翻转编译计作后端通过。

The same uncommitted declaration-consolidation unit also removes the local
O_RDONLY/O_NONBLOCK/O_NOFOLLOW definitions exposed as duplicates by the first
Cocoa integration. Both frontends explicitly use `lib/include/fcntl.in.rs`;
Darwin-only O_EVTONLY/O_SYMLINK remain local. Each removed block hash is in the
manifest, recoverable from the recorded old Git blob. SDK/full verification
remains required after this change.


2026-10-29 source-delivery update: the historical construction readings above
remain historical. The declaration changes are included in the complete
experimental runtime foundation group, whose exact proposed tree built four
macOS baseline products and passed the limited integration and protection
checks in [the foundation report](../../releases/2026-10-29-runtime-foundation.md).
This records source consolidation; it is not physical deletion, full compatibility
or native/generational-GC performance acceptance.
