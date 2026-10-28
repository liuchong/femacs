- CLN-20261028-108：客户端修复复核后的已消费Cargo/四配置生成副本19694文件、694568260逻辑字节；新鲜完整产品、源/对象、失败及验收证据保留。[报告](reports/2026-10-28-108-consumed-client-compiler-copies.md)。

- CLN-20261028-107：第二轮客户端已消费Cargo与四配置生成副本19694文件、694568188逻辑字节；当前构建、原源码/产品/对象和失败证据保留。[报告](reports/2026-10-28-107-consumed-client-compiler-copies.md)。

- CLN-20261028-106：首轮客户端失败构建已消费的Cargo/生成副本5119文件、209942304逻辑字节；源码、程序和失败证据保留，第二轮独立构建继续。[报告](reports/2026-10-28-106-consumed-client-compiler-copies.md)。

# 清理台账

完整字段和记录方法见 [工程档案入口](README.md)，每行链接一份报告。本表不是产品更新日志；同批详细清单/恢复方案由报告维护。日期不含未经记录的精确时刻。前六条于2026-10-24补齐归档；后续批次逐条记录实际动作。

| 编号 | 操作日期 | 类别与结果 | 关联动作/既有证据提交 | 报告 |
| --- | --- | --- | --- | --- |
| CLN-20261016-01 | 2026-10-16 | tracked，九个旧构建脚手架删除 | `245fd341f81c86b6765133b7f333720fb5a52a65` | [依据/9项blob/恢复](reports/2026-10-16-01-autotools.md) |
| CLN-20261022-01 | 2026-10-22 | ignored产物，19个目标范围，10452项；仅有范围级汇总 | `e125bb725faad5737aff97f73366aff70dce147c`，不是Git删除提交 | [旧清单及删后证据](reports/2026-10-22-01-macos-artifacts.md) |
| CLN-20261024-01 | 2026-10-24 | tracked，六项旧工具/实验源码删除 | `2415a335836d0ad81de2564a82301bdceefb8145` | [6项身份/替代/恢复](reports/2026-10-24-01-obsolete-tools.md) |
| CLN-20261024-02 | 2026-10-24 | 零删除检查，0文件/0字节 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`已有摘要 | [范围/未删除事实](reports/2026-10-24-02-export-cache-inspection.md) |
| CLN-20261024-03 | 2026-10-24 | 未提交实验撤回：三文件恢复，一文件删除 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`保存最终状态，原实验无提交 | [4项身份/惰性补丁](reports/2026-10-24-03-dispatch-experiment.md) |
| CLN-20261024-04 | 2026-10-24 | ignored产物，685项；逻辑187264824字节 | `c119748bfa6cb9d5ac39bde602c22702623fa44c`已有摘要，不是Git删除提交 | [逐项清单/重建](reports/2026-10-24-04-generated-artifacts.md) |
| CLN-20261024-05 | 2026-10-24 | 未提交测试收敛：删除重复启动函数/调用，保留统一入口 | 本轮交付；无旧Git blob，前后内容哈希及惰性补丁绑定 | [范围/替代/恢复](reports/2026-10-24-05-startup-harness.md) |
| CLN-20261024-06 | 2026-10-24 | ignored产物，首次失败导出的剩余对象及Cargo缓存 | 本轮交付；逐项删前SHA-256、字节与删后状态见清单 | [范围/重建/未查明原因](reports/2026-10-24-06-build-artifacts.md) |
| CLN-20261025-01 | 2026-10-25 | 未提交诊断候选收敛及导出内生成包替换；保留原始构建证据 | 本轮交付；六份源文件前后身份、生成物身份及惰性恢复补丁 | [范围/替代/恢复](reports/2026-10-25-01-diagnostic-candidate.md) |
| CLN-20261025-02 | 2026-10-25 | 验收断言、TTY制品身份及参照输出保护候选收敛 | 本轮交付；两工具前后身份、独立恢复补丁与操作回执 | [范围/替代/恢复](reports/2026-10-25-02-acceptance-guards.md) |
| CLN-20261025-03 | 2026-10-25 | 探针/L5/linker/文档策略替换，实际D探针及修复后inline验收通过 | 源基线01bfbb7f；逐项隔离Git恢复通过，实现5118ed08已签名推送；非整产品回滚 | [身份/替代/恢复](reports/2026-10-25-03-probe-tooling.md) |
| CLN-20261025-04 | 2026-10-25 | Android构建/设备策略替换，实际正常/checking构建及API36设备验收通过 | 逐项隔离Git恢复通过，实现5118ed08已签名推送；具体范围见报告 | [身份/替代/恢复](reports/2026-10-25-04-android-tooling.md) |
| CLN-20261025-05 | 2026-10-25 | API21以下死分支及一旧类移除，保留现代路径/原始资源 | 隔离Git恢复、七值源码方法矩阵及API36验收通过；5118ed08已签名推送，不称API21设备验收 | [身份/替代/恢复](reports/2026-10-25-05-api21-floor.md) |
| CLN-20261025-06 | 2026-10-25 | 两份历史生成profile删除，源探针/结论保留 | 两份原Git内容隔离恢复/哈希通过，5118ed08已签名推送；不构成性能验收 | [身份/替代/恢复](reports/2026-10-25-06-raw-profiles.md) |
| CLN-20261025-07 | 2026-10-25 | 两份已关闭私有原始协作记录移至项目外持久归档，字节/哈希保留；无源码/记录删除、无同卷空间释放声明 | 两个内容身份及私有本机恢复台账；不入库原始对话/会话定位 | [身份/边界/恢复](reports/2026-10-25-07-private-log-archive.md) |
| CLN-20261025-08 | 2026-10-25 | 五个已停止失败设备尝试的114个可重建AVD文件；保留owner、回执与日志 | 逐项内容哈希/逻辑字节/私有精确路径台账；不声称原瞬态磁盘可逐字恢复或实际卷空间释放量 | [身份/边界/重建](reports/2026-10-25-08-owned-avd-disks.md) |
| CLN-20261025-09 | 2026-10-25 | 后续五个已停止设备尝试的61个可重建AVD文件；保留已通过的batch证据及全部失败日志 | 逐项内容哈希/逻辑字节/私有精确路径台账；仅重建新设备，不伪称恢复瞬态状态或卷空间释放量 | [身份/边界/重建](reports/2026-10-25-09-owned-avd-disks.md) |
| CLN-20261025-10 | 2026-10-25 | 两个已停止GUI失败尝试的可重建AVD文件；保留CheckJNI日志、截图、回执及产品身份 | 逐项内容哈希/字节/私有精确路径台账；只清本次设备，恢复方式为重新创建 | [身份/边界/重建](reports/2026-10-25-10-owned-avd-disks.md) |
| CLN-20261025-11 | 2026-10-25 | 5368个已退出自有构建的缓存/对象、中间APK及设备拉回副本；保留当前制品、旧通过batch所用APK、源码图和所有回执 | 逐项身份/字节/打开句柄检查；保护制品哈希不变；可重编不承诺缓存/旧签名字节复现或实际卷空间释放量 | [身份/边界/重建](reports/2026-10-25-11-idle-build-products.md) |
| CLN-20261025-12 | 2026-10-25 | 四份闲置旧构建图的20030个展开文件移入私有持久压缩归档；原项目源码、当前Android/Mac图和证据不变 | 所有归档成员逐项哈希复验，四份源码实际隔离恢复；保留精确路径/归档身份台账 | [身份/边界/恢复](reports/2026-10-25-12-generated-source-archives.md) |
| CLN-20261025-13 | 2026-10-25 | 后续两个已停止设备尝试的28个可重建设备文件；保留文件创建失败日志及全部回执 | 逐项哈希/字节/私有精确路径；已核对归属和退出，只能重建新设备、不伪称恢复瞬态状态 | [身份/边界/重建](reports/2026-10-25-13-owned-avd-disks.md) |
| CLN-20261025-14 | 2026-10-25 | 三个已停止旧Android源码导出/生成构建副本的44677个普通文件转私有持久归档；原项目源码、新候选与Mac产品保留 | 每成员内容校验及三份实际恢复；既有符号链接保留且不遍历；硬链接按内容校验，旧失败不改写 | [身份/边界/恢复](reports/2026-10-25-14-superseded-source-exports.md) |
| CLN-20261025-15 | 2026-10-25 | 四个旧隔离副本和两个旧生成图的70435个普通文件持久归档；当前源码、候选、G4产品及根试用包保留 | 每成员内容核对及六份实际恢复；保留符号链接及所有外层日志/回执，不承诺逻辑计数等于卷空间变化 | [身份/边界/恢复](reports/2026-10-25-15-prior-build-archives.md) |
| CLN-20261025-16 | 2026-10-25 | 11942个闲置编译缓存、旧展开资源副本与新包中间件；保护当前APK/SO及旧batch包身份不变 | 逐项哈希/字节和打开句柄检查；保留源/对象/日志，按归档配方及输入身份重建 | [身份/边界/重建](reports/2026-10-25-16-idle-build-products.md) |
| CLN-20261025-17 | 2026-10-25 | 四个闲置旧host工具/日志/生成图子树的4714个普通文件持久归档；当前产品与运行资源保留 | 全成员内容核对、四份实际恢复及符号链接保留；日志字节仍可恢复，不以逻辑量冒充卷空间释放 | [身份/边界/恢复](reports/2026-10-25-17-idle-host-archives.md) |
| CLN-20261025-18 | 2026-10-25 | 两份旧Mac工具树与两个旧app备份的7307个普通文件持久归档；当前试用包/G4产品不动 | 全成员核对、四份实际恢复；不把文件内容恢复当成扩展属性/签名或运行验收 | [身份/边界/恢复](reports/2026-10-25-18-old-product-archives.md) |
| CLN-20261025-19 | 2026-10-25 | 已完成新构建中21343个非产品/JSON/日志普通文件归档；APK、普通/checking库保持原路径和哈希 | 全成员核对、代表成员实际恢复、产品保护复验；原项目源码和实际设备输入均保留 | [身份/边界/恢复](reports/2026-10-25-19-current-build-graph.md) |
| CLN-20261025-20 | 2026-10-25 | 仅回收已退出device12的23个可重建AVD文件；保留CCL/JSON实际失败、截图、观察与安装身份 | 全部逐项哈希及归属/退出核对；原客体瞬态状态不承诺恢复 | [身份/边界/重建](reports/2026-10-25-20-owned-avd-disks.md) |
| CLN-20261025-21 | 2026-10-25 | 两个旧tracked-only缓存导出的41733个普通文件持久归档；主仓源码与当前Mac/终端/Android产品保持原路径和哈希 | 每成员核对、两份实际恢复、符号链接保留；原外部参照/SDK和其他任务不动 | [身份/边界/恢复](reports/2026-10-25-21-previous-cache-exports.md) |
| CLN-20261025-22 | 2026-10-25 | 四个旧工具/编译缓存根下16648个普通文件持久归档；主源码、当前产品和现用隔离构建树保留 | 全成员核对、四份实际恢复、保护产品哈希不变；精确恢复路径私有保留 | [身份/边界/恢复](reports/2026-10-25-22-legacy-tool-caches.md) |
| CLN-20261025-23 | 2026-10-25 | 两次已停止的设备13运行下46个AVD临时文件移除；逻辑1345161422字节，删除前分配量1222246400字节 | 保留产品、失败/成功回执、截图、日志及归属根；临时客户机状态仅能按配方重建，不宣称精确恢复或卷释放量 | [身份/边界/恢复](reports/2026-10-25-23-owned-avd-disks.md) |
| CLN-20261025-24 | 2026-10-25 | 四轮已停止的GC诊断设备15–18下92个AVD临时文件移除；逻辑2995327288字节，删除前分配量2780037120字节 | APK、原生库、完整/截断诊断记录、失败回执和截图保留；仅重建临时客体，不假称精确恢复或实际卷释放量 | [身份/边界/重建](reports/2026-10-25-24-owned-avd-disks.md) |
| CLN-20261025-25 | 2026-10-25 | 两个已停止诊断/生产验收设备的46个可重建AVD文件；保留失败回执、日志、截图与产品 | 逐项哈希及私有路径台账；逻辑1402439900字节，删前分配1281687552字节，不等于实际卷释放量 | [身份/边界/重建](reports/2026-10-25-25-owned-avd-disks.md) |
| CLN-20261025-26 | 2026-10-25 | 已停止通过验收设备的23个可重建AVD文件；保留安装身份、11项流程证据及全部截图/日志 | 逐项哈希及私有路径台账；逻辑670614670字节，删前分配611123200字节，不等于实际卷释放量 | [身份/边界/重建](reports/2026-10-25-26-owned-avd-disks.md) |
| CLN-20261025-27 | 2026-10-25 | 两个已停止运行时尝试D6/D8的44626个生成单元/对象/编译缓存成员持久归档，保留原冻结输入、全部日志/JSON与真实程序/dump；D9及当前试用包排除 | 全成员内容核对、每树代表性Rust/对象/归档实际恢复；1510092614逻辑字节、1601650688删前分配字节，归档388178063字节；2868项保护哈希不变，不冒充实际卷释放量 | [身份/边界/恢复](reports/2026-10-25-27-runtime-intermediates.md) |
| CLN-20261025-28 | 2026-10-25 | 已停止D9/D10中33725个生成单元/对象/缓存成员持久归档，保留原冻结输入、日志/JSON、程序/dump、资源及独立工具；另移除两份内容已被替代的准备归档 | 全成员校验与代表性实际恢复；1228855175逻辑字节、1297944576删前分配字节，最终归档321379789字节，2484项保护哈希不变；两份旧准备归档315858197字节均先验证全部内容有替代；不冒充卷空间/硬链接拓扑恢复 | [身份/边界/恢复](reports/2026-10-25-28-runtime-intermediates.md) |
| CLN-20261025-29 | 2026-10-25 | 已停止D11中9883个生成单元/对象/缓存成员持久归档；原冻结输入、两分支链接失败日志、JSON、程序/资源及独立工具保留，活动D12排除 | 全成员校验及代表性实际恢复；437888198逻辑字节、458297344删前分配字节，归档116517819字节，7433项保护哈希不变；不冒充独占卷空间释放或产品修复 | [身份/边界/恢复](reports/2026-10-25-29-runtime-intermediates.md) |
| CLN-20261026-30 | 2026-10-26 | 已停止D12的4717个生成源码副本、对象及编译缓存转持久归档；原始冻结输入、日志、配置、程序/dump、D13及试用包保留 | 全成员哈希、代表成员实际恢复、7295项保护内容复验；不承诺卷空间变化 | [身份/边界/恢复](reports/2026-10-26-30-runtime-intermediates.md) |
| CLN-20261026-31 | 2026-10-26 | 已停止D13的9433个生成单元副本、对象与编译缓存持久归档；原始冻结输入、日志、程序/dump和当前D14保留 | 全成员哈希、代表成员实际恢复、7596项保护内容复验；精确路径台账保留 | [身份/边界/恢复](reports/2026-10-26-31-runtime-intermediates.md) |
| CLN-20261026-32 | 2026-10-26 | 回收295267个旧编译物/重复副本，逻辑14364408836字节 | 逐项哈希/归属/句柄检查，8762份保护内容不变；D14/D18/D20和现用产品排除 | [身份/恢复/边界](reports/2026-10-26-32-unused-artifacts.md) |
| CLN-20261026-33 | 2026-10-26 | 回收48276个被取代的G4试验副本/编译物，逻辑1920342202字节 | 3382份保护内容不变；当前G4候选、恢复产品及运行时矩阵排除 | [身份/恢复/边界](reports/2026-10-26-33-old-g4-copies.md) |
| CLN-20261026-34 | 2026-10-26 | 回收7份D6/D8至D13旧编译缓存归档，逻辑947780933字节 | 原台账哈希不变；撤销这些包的精确恢复承诺，保留源/配置按需重建 | [身份/重建/更正](reports/2026-10-26-34-obsolete-cache-archives.md) |
| CLN-20261026-35 | 2026-10-26 | 回收6327个旧App/字节码/手册/打包残留，逻辑707190095字节 | 逐项哈希与空目录清单，原输入/日志/符号链接/现用候选排除 | [身份/重建/边界](reports/2026-10-26-35-retired-packages.md) |
| CLN-20261026-36 | 2026-10-26 | 回收已退出D14/D18/D20/D21矩阵411156个生成物/重复资源，逻辑23760244161字节 | 全部目标消失、67973项保留内容不变；冻结原输入/日志/现用候选保留，仓库现17G | [身份/恢复/边界](reports/2026-10-26-36-idle-matrices.md) |
| CLN-20261026-37 | 2026-10-26 | 回收停止D22的13379个生成物/重复资源，逻辑956847680字节 | 全部目标消失、14266项保留内容不变；新D23/当前试用包与原始源/记录排除 | [身份/恢复/边界](reports/2026-10-26-37-consumed-d22.md) |

| CLN-20261026-38 | 2026-10-26 | 回收55配置的164911个闲置搬移副本，逻辑10911717416字节 | 110925个原件内容不变；保留真实失败、现用包和6个不同文件，逐项清单可追溯 | [身份/恢复/边界](reports/2026-10-26-38-relocation-copies.md) |

| CLN-20261026-39 | 2026-10-26 | 回收40编译配置187700个已消费Rust生成副本，逻辑6256833936字节 | 136048个原始源/产品/链接输入及回执内容不变；保留仍有消费者的D23矩阵，补齐逐例/逐轮清理出口 | [身份/重建/边界](reports/2026-10-26-39-generated-rust.md) |

| CLN-20261026-40 | 2026-10-26 | 两个真实baseline搬移用例消费5979个包副本后逐例回收，逻辑323567463字节 | 原件/fixture身份不变、源码拒绝和签名通过，包外逐项清单与实际生命周期回执保留 | [身份/恢复/边界](reports/2026-10-26-40-per-case-relocation.md) |

| CLN-20261026-41 | 2026-10-26 | 两个真实native搬移用例消费6017成员后逐例回收，逻辑424945610字节 | 实际源码/后端拒绝、native编译加载与身份、签名通过，原件/fixture不变 | [身份/恢复/边界](reports/2026-10-26-41-native-relocation.md) |

| CLN-20261026-42 | 2026-10-26 | 退役已消费D23的160845个生成物/重复资源及冗余归档，逻辑17175176376字节 | 43947个保留内容不变，源码/原始回执/三个代表dump保留，明确CLN38/40/41原包恢复改经已验证内容归档 | [身份/恢复/边界](reports/2026-10-26-42-consumed-d23.md) |

| CLN-20261027-44 | 2026-10-27 | 在全量构建继续期间回收12个已完成配置的110532个生成Rust副本，逻辑3672661468字节 | 原配置均已退出并交接；84853个保留内容不变，当前配置和在用出口不动；精确清单先持久化后删除，公开逐项ID与本机精确路径台账对应 | [身份/重建/边界](reports/2026-10-27-44-consumed-d24-compiler-copies.md) |

| CLN-20261026-43 | 2026-10-27 | 新D24全量构建成功退出后自动回收其余77168个生成Rust副本，逻辑2582280788字节；编号前日预留 | 136061个保留内容不变，原包装器核对源/产品/清单身份；与CLN44无重复，公开逐项ID映射至本机精确台账 | [身份/重建/边界](reports/2026-10-26-43-generated-rust.md) |

| CLN-20261027-45 | 2026-10-27 | 退役9个旧快照中的62316份逐字节重复源副本，逻辑3190145253字节 | 6835份恢复源哈希不变，517份独有源及原日志保留；旧快照改为差量、须按原清单补齐后运行，当前D24及试用包不动 | [身份/恢复/边界](reports/2026-10-27-45-duplicate-snapshots.md) |

| CLN-20261027-46 | 2026-10-27 | 回收被后继准备归档完整覆盖的旧D24内容归档，逻辑590461454字节 | 旧10770个成员均在保留14015成员包内，所有成员流式验证；原回执不变，恢复路径明确转交后继包 | [身份/恢复/边界](reports/2026-10-27-46-superseded-archive.md) |

| CLN-20261027-47 | 2026-10-27 | 回收41216份成功进程临时文件及一份完整被覆盖的中途快照，逻辑1479334370字节 | 6110个case内容逐项比较，6120份保留结果/输入身份不变，4个失败现场保留；没有重跑产品 | [身份/恢复/边界](reports/2026-10-27-47-consumed-runtime-state.md) |

| CLN-20261027-48 | 2026-10-27 | 回收40次已消费手册导出的15240份重复源文件，逻辑999739600字节 | 对应381份冻结原输入，8744份保留内容哈希不变；Info、回执、日志及产品资源均保留 | [身份/恢复/边界](reports/2026-10-27-48-manual-input-copies.md) |

| CLN-20261027-49 | 2026-10-27 | Reclaimed3072 consumed pilot input copies and1 generated cache;42028000 logical bytes | Original1536 inputs and69 raw evidence files unchanged; live full followup untouched | [Identity/recovery/scope](reports/2026-10-27-49-consumed-pilot-inputs.md) |

| CLN-20261027-50 | 2026-10-27 | Retired179958 consumed relocation members over60 profiles,11962942294 logical bytes | Every work copy absent, raw ledger SHA verified; originals retained. Executed old fsync gap disclosed; later P385 repair separate | [Identity/recovery/scope](reports/2026-10-27-50-d25-relocation.md) |

| CLN-20261027-51 | 2026-10-27 | P385 filesystem integration removes3 owned fixture members,648 logical bytes | External sentinel unchanged; real sync-error/open-FD controls retained. Four integration and2 subsequent unit cases pass | [Identity/recovery/scope](reports/2026-10-27-51-p385-fixture.md) |

| CLN-20261027-52 | 2026-10-27 | Retired6 superseded Android APK/library files,263789634 logical bytes | Current version10 and Mac programs/old identity receipts unchanged; no new product test | [Identity/recovery/scope](reports/2026-10-27-52-superseded-android.md) |

| CLN-20261027-53 | 2026-10-27 | Retired11963 consumed diagnostic/relocation App files,615402920 logical bytes | Original logs/receipts and current trial/release Apps preserved; explicit rebuild/copy restoration | [Identity/recovery/scope](reports/2026-10-27-53-consumed-app-copies.md) |

| CLN-20261027-54 | 2026-10-27 | 回收11632份已消费dump/native镜像与负面夹具副本，1230580584逻辑字节 | 7144项保留内容哈希不变，原始准备夹具和当前候选不动；恢复须先生成独占工作制品 | [身份/恢复/边界](reports/2026-10-27-54-consumed-case-artifacts.md) |

| CLN-20261027-55 | 2026-10-27 | 回收127070份已被新源码取代的旧候选产品/资源/链接展开文件，11435235140逻辑字节 | 14015成员恢复包逐内容核对，20011项保留内容不变；旧D24路径退役，新G3须重建 | [身份/恢复/边界](reports/2026-10-27-55-superseded-runtime-expansion.md) |

| CLN-20261027-56 | 2026-10-27 | 回收340份同恢复包内容相等的旧暂存库/dump/程序，2024827592逻辑字节 | 独立记录原保护清单未列出的暂存路径/mode；试用包和原证据保留 | [身份/恢复/边界](reports/2026-10-27-56-duplicate-runtime-expansion.md) |

| CLN-20261027-57 | 2026-10-27 | 回收2800份已消费的手册输出中间副本，841532040逻辑字节 | 所选内容与保留包成员逐项相等，文档原输入/配方和结果保留 | [身份/恢复/边界](reports/2026-10-27-57-consumed-manual-outputs.md) |

每份报告本次归档提交通过 `git log --follow -- maintenance/cleanup/reports/文件名.md` 定位；这是报告的Git溯源，不把较早的源码/摘要提交误写为本次归档提交。未来新增批次先取得新编号并追加本表，不复用或覆盖旧编号。

非计划删除另记[2026-10-25 G4产物丢失事件](../incidents/2026-10-25-g4-build-artifact-loss.md)：删除者未知，不算作已授权清理批次，也不虚构恢复或仍保有产物。


| CLN-20261027-58 | 2026-10-27 | 两次失败Cocoa构建后回收19880份编译缓存，722231668逻辑字节 | 原源码、失败和回执14056项哈希不变；当前构建/试用包保留 | [范围与恢复](reports/2026-10-27-58-failed-cocoa-compiler-caches.md) |

| CLN-20261027-59 | 2026-10-27 | UI修复完成后回收10639份编译缓存，436824308逻辑字节 | 26979项源/证据及当前、回退App哈希不变；第5轮真实运行崩溃保留 | [范围与恢复](reports/2026-10-27-59-consumed-ui-compiler-caches.md) |

| CLN-20261027-60 | 2026-10-27 | 回收已消费UI标志组件程序1份，511576逻辑字节 | 8项源码/结果/当前程序身份不变，在用窗口不动；额外直接字访问静态核对16项一致 | [范围与恢复](reports/2026-10-27-60-ui-flag-probe.md) |

| CLN-20261027-61 | 2026-10-27 | 缩放修复交付后回收14955份编译副本，527335335逻辑字节 | 27063项源/失败/新证据/当前与回退App身份不变；冻结链接/诊断输入保留 | [范围与恢复](reports/2026-10-27-61-live-resize-compiler-caches.md) |

| CLN-20261027-62 | 2026-10-27 | 回收5177份已消费多窗口编译副本，202100121逻辑字节 | 13066项冻结源/程序/原失败/新诊断身份不变；真实输入缺口保留 | [范围与恢复](reports/2026-10-27-62-consumed-multiframe-build-caches.md) |

| CLN-20261027-63 | 2026-10-27 | Retired8153 consumed receiver-build/cache and failed-copy files,357814435 logical bytes | All16015 retained input/product/evidence identities unchanged; current native/release apps preserved | [Identity/recovery/scope](reports/2026-10-27-63-consumed-multiframe-build-caches.md) |

| CLN-20261027-64 | 2026-10-27 | Retired2 rejected uncommitted SVG masters plus23363 consumed generated/cache/intermediate files,1584540183 cache logical bytes | Exact backups and per-file manifest; fsync gap on initial SVG retirement disclosed; all16013 retained identities unchanged | [Identity/recovery/scope](reports/2026-10-27-64-artwork-and-consumed-release-caches.md) |

| CLN-20261027-65 | 2026-10-27 | Retired6812 consumed packaging/test/cache/superseded release files, 508068131 logical bytes | Durable identity manifest; all5980 retained identities unchanged; current F/epsilon delivery preserved | [Scope and recovery](reports/2026-10-27-65-consumed-f-artwork-outputs.md) |

| CLN-20261027-66 | 2026-10-27 | Retired6885 consumed output files, 486132014 logical bytes | Durable identity manifest; all5982 retained identities unchanged at cleanup; current products/evidence retained | [Scope and recovery](reports/2026-10-27-66-selected-horn-artwork-outputs.md) |

| CLN-20261027-67 | 2026-10-27 | Retired4058 consumed output files, 289754448 logical bytes | Durable identity manifest; all9002 retained identities unchanged at cleanup; current products/evidence retained | [Scope and recovery](reports/2026-10-27-67-consumed-contour-preview-cache.md) |

| CLN-20261027-69 | 2026-10-27 | 净回收55470份旧布局构建展开副本，3712209331逻辑字节 | 七份既有归档逐内容验证，20874项保留身份不变；34份build配置/文字额外恢复，错误与恢复如实记录 | [范围与恢复](reports/2026-10-27-69-layout-expansions.md) |

| CLN-20261027-70 | 2026-10-27 | 12个已完成配置回收112116份生成Rust副本，3737290194逻辑字节 | 88190项冻结源/产品/资源/链接证据身份不变；其他配置继续构建 | [范围与恢复](reports/2026-10-27-70-consumed-d26-compiler-copies.md) |

| CLN-20261027-71 | 2026-10-27 | 后续5个已完成配置回收48890份Rust副本，1640451148逻辑字节 | 120230项保护内容不变；在跑配置与当前产品不动，旧批不重复计数 | [范围与恢复](reports/2026-10-27-71-consumed-d26-compiler-copies.md) |

| CLN-20261027-68 | 2026-10-27 | D26结束后29770份编译副本/缓存，1039083114逻辑字节 | 实际晚于70/71；136269项删前后不变；后续缺失驱动按原归档恢复，原因未知 | [范围与恢复](reports/2026-10-27-68-consumed-d26-compiler-copies.md) |

| CLN-20261027-73 | 2026-10-27 | 首轮TLS编译失败后回收9686份生成副本/缓存、364128325逻辑字节 | 8260项保留内容哈希不变，原失败/冻结源/接口证据保留 | [范围与恢复](reports/2026-10-27-73-failed-d27-compiler-copies.md) |

| CLN-20261027-74 | 2026-10-27 | 回收656份已结束诊断副本/失败链接输入、42832592逻辑字节 | 16份最小编译根/配置身份不变；计划误写副本已精确恢复并记录；D28不动 | [范围与恢复](reports/2026-10-27-74-consumed-diagnostics.md) |

| CLN-20261027-75 | 2026-10-27 | D28首个完成配置回收9302份Rust副本、310060430逻辑字节 | 13723项保留内容不变；中断后独立核实退出并释放原保护再继续同一账本 | [范围与恢复](reports/2026-10-27-75-completed-d28-compiler-copies.md) |

| CLN-20261027-76 | 2026-10-27 | 退役旧D26的126950份无消费者制品、11457749357逻辑字节 | 45381项保留内容不变；冻结源、原始日志与回执保留，无新全量缓存归档 | [范围与恢复](reports/2026-10-27-76-superseded-d26-runtime-artifacts.md) |

| CLN-20261027-77 | 2026-10-27 | 恢复后的baseline两腿回收9302份副本、309409362逻辑字节 | 完整退出回执；产品与链接输入保留；2/20配置 | [范围与恢复](reports/2026-10-27-77-recovered-baseline-copies.md) |

| CLN-20261027-78 | 2026-10-27 | 旧D26生成手册与冗余保全包23761份、2540186628逻辑字节 | 无消费者；全部冻结源与原始回执不变，旧archive路径退役 | [范围与恢复](reports/2026-10-27-78-superseded-d26-manuals.md) |

| CLN-20261027-79 | 2026-10-27 | 旧D26残余编译制品3900份、2097411420逻辑字节 | 无消费者；全部冻结源及原始回执不变；当前D28不动 | [范围与恢复](reports/2026-10-27-79-superseded-d26-residual-products.md) |

| CLN-20261028-80 | 2026-10-28 | 旧D24/D27残余编译缓存9581份、475034633逻辑字节 | 两棵冻结源/元数据不变；四棵无法完整验证的旧树整棵排除；当前D28不动 | [范围与恢复](reports/2026-10-28-80-obsolete-matrix-caches.md) |

| CLN-20261028-81 | 2026-10-28 | 166份旧验收JSON完整保留为154内容成员，退役原路径；1份无效压缩试样清理 | 解压SHA/字节全核验；约1.73GiB分配量回收；冻结源及原始文本日志保留，P403拒绝/P404采用 | [范围与恢复](reports/2026-10-28-81-retained-evidence-storage.md) |

| CLN-20261028-82 | 2026-10-28 | 8个恢复终端配置两腿74416份生成副本、2478845236逻辑字节 | 原逐项fsynced操作账本汇总；首配置/基线已75/77不重计；后续Cocoa配置失败不冒充验收通过 | [范围与恢复](reports/2026-10-28-82-recovered-terminal-copies.md) |

| CLN-20261028-83 | 2026-10-28 | 退役配置绑定失效的D28旧制品/生成手册22985份、5864944479逻辑字节 | 冻结源及原有元数据逐项不变；原失败和日志保留；新D29、根App不动 | [范围与恢复](reports/2026-10-28-83-superseded-d28-runtime-products.md) |

| CLN-20261028-84 | 2026-10-28 | D29首对代表四腿已消费副本19130份、640352977逻辑字节 | 原逐项操作账本核验，制品/源/配置保留；未运行功能验收 | [范围与恢复](reports/2026-10-28-84-new-representative-copies.md) |

| CLN-20261028-85 | 2026-10-28 | Retire59172 superseded D28 package resource copies,2641604359 logical bytes | Released ownership and original hashes verified; frozen inputs/logs/current D29 retained; no exclusive-space claim | [Scope and recovery](reports/2026-10-28-85-superseded-d28-resource-copies.md) |

| CLN-20261028-86 | 2026-10-28 | Retire3 redundant old UI source archives,606773535 logical bytes | All21132 original source/metadata identities retained; incomplete old scopes skipped; current D29 untouched | [Scope and recovery](reports/2026-10-28-86-redundant-ui-source-archives.md) |

| CLN-20261028-87 | 2026-10-28 | Retire208 committed manifest export copies/712090173 logical bytes, plus36 consumed restoration probes | Real Git restoration verified; business inputs preserved; old exports require metadata restoration before full source verification | [Scope and recovery](reports/2026-10-28-87-committed-manifest-export-copies.md) |

| CLN-20261028-88 | 2026-10-28 | Compact10 old source containers/2822825012 bytes into reachable Git plus77045704-byte unique-content and24089514-byte mapping stores | All13337 contents actually restored;70104 source mappings checked; identified compiler caches omitted; raw journals/results retained | [Scope and recovery](reports/2026-10-28-88-compacted-source-containers.md) |

| CLN-20261028-89 | 2026-10-28 | D29后续13配置26腿已消费副本124082份、4143469900逻辑字节 | 原操作账本逐项核验；不重计CLN84，当前产品及源保留，发布器无新删除 | [范围与恢复](reports/2026-10-28-89-continuation-generated-copies.md) |

| CLN-20261028-90 | 2026-10-28 | D29完成配置30腿手册暂存12530份、1081451056逻辑字节 | 实际包内Info同字节核验；终端直接引用Info、原回执/日志和当前候选保留 | [范围与恢复](reports/2026-10-28-90-consumed-manual-staging.md) |

| CLN-20261028-91 | 2026-10-28 | 旧成功构建138份原日志734439595字节无损保留为62032435字节gzip | 每份实际解压恢复核SHA，原元数据与失败/当前日志保留；旧路径须按映射恢复 | [范围与恢复](reports/2026-10-28-91-retained-build-logs.md) |

| CLN-20261028-92 | 2026-10-28 | 两个中断Cocoa case已无消费者的编译缓存14277份、1181584740逻辑字节 | 原冻结源前后完整验证；17完成配置、配置/失败日志/元数据保留，旧半成品需重建 | [范围与恢复](reports/2026-10-28-92-interrupted-cocoa-caches.md) |

| CLN-20261028-93 | 2026-10-28 | 五个旧运行时导出已提交数据副本22970份、758888595逻辑字节 | 4524不同Git blob实际恢复，固定232f1651及原mode映射；旧完整源须结合87恢复，新D29/UI试用源不动 | [范围与恢复](reports/2026-10-28-93-committed-data-export-copies.md) |

| CLN-20261028-94 | 2026-10-28 | 首个续成Cocoa两腿已消费手册暂存910份、94583952逻辑字节 | 68/68、真程序/冻结输入及包内Info完整身份已核；原回执/日志保留，终端Info不动 | [范围与恢复](reports/2026-10-28-94-consumed-cocoa-manual-staging.md) |

| CLN-20261028-95 | 2026-10-28 | 两个旧中断Cocoa半成品重复资源5056份、179943122逻辑字节 | 每份与当前冻结源实际同字节；2494不同替代内容已核，原配置/失败/元数据保留 | [范围与恢复](reports/2026-10-28-95-interrupted-resource-copies.md) |

| CLN-20261028-96 | 2026-10-28 | 第二个续成Cocoa两腿已消费手册暂存910份、94583952逻辑字节 | 程序/68手册/包内Info完整身份核验，原回执/日志/产品保留，终端Info不动 | [范围与恢复](reports/2026-10-28-96-consumed-cocoa-manual-staging.md) |

| CLN-20261028-97 | 2026-10-28 | 最后续成Cocoa配置两腿已消费手册暂存910份、94583952逻辑字节 | 程序、68手册及包内Info身份已核；原日志、回执及产品保留 | [范围与恢复](reports/2026-10-28-97-consumed-cocoa-manual-staging.md) |

| CLN-20261028-98 | 2026-10-28 | 最终化中断后未完成保全归档671187835字节 | 全部138291原文件实际核验后才退役重复输出；原native最终回执缺失，不虚构通过或指标 | [范围与恢复](reports/2026-10-28-98-incomplete-preservation-archive.md) |

| CLN-20261028-99 | 2026-10-28 | 两份已消费host Cargo缓存及一份重复工具输入，873文件、113848544逻辑字节 | 原host与保全包逐份实际核验，当前138291文件账本未引用所选项，7099冻结源前后完整验证 | [范围与恢复](reports/2026-10-28-99-consumed-host-caches.md) |

| CLN-20261028-100 | 2026-10-28 | 已取消TLS轮次中7份冗余成功断言历史、1864480748逻辑字节 | 全量JSON解析后保留除大diagnostic外全部字段、原始SHA及真实取消回执；原超限拒绝与取消状态不改写为通过 | [范围与恢复边界](reports/2026-10-28-100-verbose-passed-responses.md) |

| CLN-20261028-101 | 2026-10-28 | 211个已消费成功用例隔离状态、23302份/591932223逻辑字节 | 逐例进程/句柄/删前身份核对，原结果哈希不变，失败与原始夹具保留；缓存可重建 | [范围与恢复](reports/2026-10-28-101-consumed-case-state.md) |

| CLN-20261028-102 | 2026-10-28 | dump48/48完成后80份试验图像/836810968逻辑字节 | 原结果SHA匹配、输出无句柄，原产品/夹具及所有回执保留；再生不承诺随机图像字节一致 | [范围与恢复](reports/2026-10-28-102-consumed-dump-images.md) |

| CLN-20261028-103 | 2026-10-28 | 4份已消费搬移副本/968654987逻辑字节 | Spotlight自然释放后全树比对及无句柄核验；独立延后回执绑定原失败SHA，修复收尾字段错误但未重复删除 | [范围与恢复](reports/2026-10-28-103-deferred-relocation-copies.md) |

| CLN-20261028-104 | 2026-10-28 | 独立复核消费后的9份小型夹具/26文件与链接/258逻辑字节 | 原结果与证明保留；逐项身份、无句柄及持久意图后回收，不计额外测量缓存释放 | [范围与恢复](reports/2026-10-28-104-consumed-p411-fixtures.md) |

| CLN-20261028-105 | 2026-10-28 | 完整ERT后的自动缓存回收41216文件/1043534381逻辑字节；negative零选中 | 10943份逐例回执与清单/私有路径映射逐项核对；无重复删除，原结果和产品保留 | [身份与恢复](reports/2026-10-28-105-runtime-functional-state.md) |
