## 当前接续：Goal active，完整ERT／negative继续（2026-10-28）

原Goal已确认active。回收保护修复8项文件系统集成、2项只读审计和6项保护用例通过，独立复核采用并保留清单证明、历史操作及并发边界的限定；已消费小型夹具见[CLN104](../maintenance/cleanup/reports/2026-10-28-104-consumed-p411-fixtures.md)。

原20配置/61profiles冻结输入核验通过，完整ERT已在新输出中实际继续，其后接原negative验收；重启前不完整结果保留。已通过TLS/dump等不重跑，12726正式样本仍全属busy-host，不能宣称性能收益或回退。整个Goal仍仅G4交付，1/7；稳定语料、剩余功能与真实macOS交付继续推进。

## 当前检查点：测量采集通过，性能待安静复验（2026-10-28）

当前20配置/61profiles的TLS915/915、native格式夹具608及dump保存/恢复48/48已有实际完成回执。搬移60项功能/签名/源路径拒绝断言成功；首轮4项因Spotlight占用回收失败，原报告56/60保留，其后4份副本已核验回收并另存绑定原SHA的完成回执。

七轮正式测量保留12726样本，2727/2727工作负载及441/441启动进程通过，输入/驱动身份不变且状态回收完成；1818组均被既定规则判为busy-host，960项比较无合格项，性能收益待安静环境复验。继续完整ERT/negative、原稳定语料、保护性回归、独立复核、干净构建与真实macOS UI验收；新候选发布仍待这些必要项收口。整个Goal仅G4交付，1/7。下文较早的存储阻碍与running措辞均保留历史边界。


D28已因ENOSPC停止：首个配置两腿完成，随后基线checking写盘失败。
控制器最终报告未保存；已独立核实全部记录身份退出、输出句柄空，并释放
原保护。不得把下方旧running记录当当前状态。
[中断与恢复边界](../maintenance/incidents/2026-10-27-runtime-d28-disk-interruption.md)；
CLN75–76已回收用完副本并退役旧D26制品，源码、参照、当前试用包和原始
证据保留。D26旧products仅作历史记录，不能再直接执行。断点续建尚未完成。

TLS当前源码ABI5复验通过：268项输入绑定、233项C依赖前后不变、99项观测
一致；旧ABI4仅对较早版本有效。D28新冻结源完整构建运行中，尚未取得完整
编辑器/TLS功能验收。用完诊断副本按[CLN74](../maintenance/cleanup/reports/2026-10-27-74-consumed-diagnostics.md)
回收；计划误写的一份生成副本已按原输入精确恢复并核对原始图回执。

D27首个配置normal/checking编译失败，剩余矩阵未启动，没有新产品。
根工作区已按真实TLS头和进程实现修复；接口层99项观测一致、103项函数原型
断言通过，完整编译与TLS功能仍待验收。用完副本按
[CLN73](../maintenance/cleanup/reports/2026-10-27-73-failed-d27-compiler-copies.md)
回收；保留原失败与冻结源码。下方D26和P401初始施工记录各有其版本边界。

D26原构建已自然结束，20配置/61 profile构建与dump通过，尚非语义/兼容/性能通过。
结束后按[CLN68](../maintenance/cleanup/reports/2026-10-27-68-consumed-d26-compiler-copies.md)
回收剩余生成副本及cargo缓存；后续缺失的独立driver已按原身份归档恢复。
下方“继续构建”等措辞保留各自历史上下文，当前ROOT的XML/SQLite/TLS不由D26作证。

D26后续已完成配置的生成副本继续按[CLN71](../maintenance/cleanup/reports/2026-10-27-71-consumed-d26-compiler-copies.md)
回收；各批不重复计数，当前产品与活动配置保留。

D26已完成配置的编译副本按[CLN70](../maintenance/cleanup/reports/2026-10-27-70-consumed-d26-compiler-copies.md)回收；程序/资源/链接输入保留，其他配置继续构建，当前进度以实际process回执为准。

旧G1布局构建1–7的已归档展开源已按[CLN69](../maintenance/cleanup/reports/2026-10-27-69-layout-expansions.md)退役；历史日志保留，复跑须先隔离恢复，不能直接使用旧source路径。当前App、终端和D26冻结源不受影响。
## Fe 标志与 macOS 发行包（2026-10-27，P-395）

README及App统一使用立体铸铁ε与局部锈蚀图案。根目录 `packaging/` 独立
发行模块已接通本地App、DMG、签名和校验清单；Windows/Linux为后续计划。
当前图形试用包仍在 `dist/macos/Femacs.app`，发行包在 `dist/releases/fe-iron`。
普通Cocoa构建接线及后续新位图打包分别验收；详见[交付记录](../maintenance/releases/2026-10-27-fe-artwork-packaging.md)。

## 多窗口中文输入修复已实测（2026-10-27，P-393/P-394）

真实硬件监视定位到生成显示访问器的接收指针错误；已修正。新Cocoa候选
独立构建、dump及签名通过，用户手动确认双窗中文输入、B→A→B→A切换
全部正常；普通退格错误可恢复，原编辑器自然退出0。保护回归4/4。
见[故障及验收记录](../maintenance/incidents/2026-10-27-multiframe-input.md)。
这是本次输入修复，不是整个G1或Goal完成。

## Finder external drop verified (2026-10-27)

The user-performed Finder text-file drop now has matching native and opened-file evidence: 124 motion callbacks, 1 actual drop, 124 live-frame coordinate observations and exact 57-byte UTF-8 contents. Source/fixture/program/dump identities are unchanged. The editor remains live for pending multi-frame IME, so this is not a normal-exit or whole-G1 claim. The earlier automated no-callback attempts remain failures of delivery observation. See [the bounded acceptance record](../maintenance/releases/2026-10-27-g1-finder-drop.md). Other implementation remains at its prior checkpoints.

## 当前 UI 修复（2026-10-27，P-391/P-392）

按用户最新要求继续只修这处Cocoa UI，其他施工及GC调查保持原检查点。窗口留白/标签栏修复后，用户真实拖动又触发标题格式化SIGSEGV：Swift变参数组误绑定C变参。P-392改为原生Swift标题，保留网格/live分支和旧标题恢复。新App实际记录696条live resize和696次正确标题、6条结束回调、193种内容尺寸；末态native/view/core730×564、正常退出0；随后格式保护6/6。独立源码/旧App反汇编及新证据复核无阻断，并明确采用本次真实失败与修复。

`dist/macos/Femacs.app` 已更新，程序SHA `a1f6f56bad59ee4f582dd2c9c7e2daf1d4fe050be5cdf7766ff841ac7b366292`。这是本次缩放分支的实际修复证据，不是完整G1/G3/快捷键或GC完成。原GC崩溃、中文输入法前缀及其他未验收项目保留。完整原始失败、新证据、限制及试用入口见[UI修复记录](../maintenance/incidents/2026-10-27-cocoa-layout.md)。

## Current storage boundary (2026-10-27, CLN55-57)

The old D24 runtime expansion has been retired after all selected contents were
verified against the retained prepare recovery archive. Its products.json is now
historical identity evidence: do not execute those paths without full isolated
restoration. Keep the 6969 frozen inputs, raw results/failures, recovery mappings
and current Mac trial/G4 Apps. New G3 source requires a fresh build; retirement
does not count as compatibility or performance acceptance. Exact scope and
recovery: maintenance/cleanup/reports/2026-10-27-55-superseded-runtime-expansion.md.
CLN56 also retires 340 archive-covered staging libraries/images/programs; CLN57
retires 2800 consumed manual-output copies. All 20011 retained identities match.
Historical snapshots below retain their original dates and scope.

## 当前施工：G4 已签名交付，继续原生验收及运行时阶段（2026-10-25）

最新交付：实现`5118ed08`已验证良好签名并立即推送，远端master实查一致。独立D22复核和清理恢复元数据已闭合，G4限定范围完成；[交付审计](../maintenance/releases/2026-10-25-g4-modernization.md)记录当前制品指纹、真实验证、旧失败及未完成范围。此前丢失的Mac产物已从新鲜独立源码导出重新构建。终端537.413秒、Cocoa526.680秒，各15/15 CCL回归通过；68本Info读取、搬移后源导出不可用时的Info读取、原包/搬移包严格签名及源码保护通过。重建控制器1073.449秒、峰值871890944字节、回收完成。新候选保留于`dist/candidates/g4-macos-2/Femacs.app`，仍未替换根目录已试用App；复制保留的终端二进制需要原导出资源树。`dist/candidates/`仅新增Git忽略，不删除产物。物理多窗口IME/Finder验收仍未完成，不由batch或构建结果替代。

D21保留证据：P-345已恢复按平台选择键表偏移，20项源码编译回归及14项受影响组件检查通过。正常/checking库和签名包已重建，`dist/candidates/g4-android-10`在真实API36 ARM模拟器通过11项设备流程，包括四项快捷键/方向键、精确中文剪贴板、渲染与横竖屏、正常存活，零fatal标记，安装制品哈希相符且进程回收完成。D20及以下失败保留为历史，未改写原回执；随后Mac重建已通过，见上列D22。当前仍未把batch结果当作锁屏未完成的多窗口IME/Finder验收。

产物保留状态更正及D20历史：D17后发现隔离树内已验收的新Mac二进制、Android APK/SO及全部原对象缺失；日志和Root试用App仍在，删除者未知。历史通过不等于当前还有可运行候选，详见[产物丢失事件](../maintenance/incidents/2026-10-25-g4-build-artifact-loss.md)。原生对象和正常包已重新构建保留。P-341经有界诊断定位为按键名缓冲区越界清零相邻fingerprint变量，而非GC算法错误；P-344修复五处结束字节并保护零容量。真实源码边界回归11/11通过，旧源码反向对照确实破坏相邻哨兵。设备20逐条记录确认Ctrl-A/Ctrl-E成功、左方向键失败；该轮未验收渲染/旋转。P-345随后修复并取得上列D21结果，D22又恢复了Mac新产物；旧试用包未覆盖，不停用GC或降低验收要求。

用户最新验收要求：以系统安装的macOS正式稳定版为对照，兼容性必须100%、性能必须实际提升，并比较稳定性、内存及GC延迟；不用从FTP下载原版。Linux需要验收时可用发行版包，本地参照可按需更新，仍不作为产品构建依赖。当前系统参照的实际程序/dump与此前核验31.1时相同；旧48/48有限用例不能冒充100%兼容，旧性能数据也不是这次统一基线的新提速结果。此要求已纳入[唯一总计划](modernization.md#full-scope-sequence-and-finish-conditions)，整个Goal继续active。

[唯一总计划](modernization.md#persistent-goal-complete-the-seven-part-modernization-plan)保持 active；本阶段详见[工具翻新手册](tooling-modernization.md)。G4组件、真实构建/Info/Android设备、冻结独立复核及签名远端交付均已取得上述证据。G4明确交付候选为`dist/candidates/g4-macos-2/Femacs.app`；根目录旧试用App保留至G1原生验收完成。全部清理记录见[台账](../maintenance/cleanup/LOG.md)，CLN03–06已据逐项隔离Git恢复实际证据统一状态，不称整棵产品回滚。外部Finder拖放和多窗口IME仍未通过；G2/G3总体兼容验收仍未闭合；G5–G7尚未实现。

# 新的持续目标：七项现代化工作（2026-10-25）

用户已明确要求建立新的原生 Goal，持续完成[现代化手册的七项总计划](modernization.md#persistent-goal-complete-the-seven-part-modernization-plan)：G1 macOS剩余修复与真实验收、G2独立生成构建运行、G3兼容性、G4历史包袱清理翻新、G5真实Lisp/GC画像、G6编译器/解释器及native/JIT、G7分代GC等回收候选。顺序仍是稳定交付→清理翻新→运行时优化；施工GC0–GC9与验收GA1–GA8分开记录，已通过且未受影响的证据复用。

新的目标已通过当前对话原生目标接口创建并读回 `active`；没有为已结束的一期CLI目标做resume/clear，也没有另建开发进程。完整可粘贴 `/goal` 文案保存在同一手册。GD1–GD4的设计、全部用例和实现已写齐，当前处于D集中验收及修正；锁屏只阻塞依赖真实桌面的验收，不阻塞独立工作。未来native/GC研究仍须真实编辑器候选与语义/性能证据，未启动这些新候选；拒绝候选的研究结论不冒充新功能已启用。

# 本轮验证：TTY 错误路径、窗口观测与稳定版对照（2026-10-25）

修复 `vfatal` 的 va-list 转发和未命名 TTY 打开失败时的释放归属；正常具名终端删除共用明确的链表摘除。原生诊断增加窗口/first-responder身份及回调后的实际状态采样，测试初始窗口在创建前命名，不借诊断自动切换焦点或提交/丢弃组字。

- 完整tracked输入导出、无旧缓存/参照的终端与Cocoa构建均退出0；终端506.029秒、Cocoa439.326秒，受控峰值884621720字节且回收完成。随后仅重建有变更的Swift、链接/dump/打包79.683秒，保留相同Rust/config输入；不能把首次旧诊断候选的构建冒充最终原生验收。
- 新导出终端6/6进程用例通过：无控制TTY的完整格式化诊断、具名无效终端类型及10次可恢复失败后的GC/列表不变性/UTF-8保存。释放准确性另有实际所有权源码审核，进程列表本身不证明分配器free次数。
- 最新稳定版本专项对照已实际核验参照为31.1（安装发行版），端口终端/Cocoa为32.0.50；六组语言边界加十组配置/库/编辑行为，三产品共48/48 ERT、零跳过，确定性观测与制品前后指纹一致。参照使用真实native程序及其dump，拒绝启动脚本身份；不是所有Lisp/插件完整等价，也不把本地发布标签当作已安装发行版的纯上游构建证明。
- 轻量用例：窗口/trace及输出/身份helper10/10（原8项加2项）、更新后的稳定对照6/6、受影响基线组件7/7、既有本地验收组件8/8；独立搭档审核已核实源码/回执并采纳两项真实时序修正。最终复核后补强完整TTY诊断相等性、程序/dump前后指纹与参照目录输出保护，10项helper加6项真实TTY复验16/16，额外JSON输出保护复验10/10；未变范围复用原有证据，重复同版本用例不增加覆盖。
- 根目录正常终端已构建成功512.056秒，使用根目录自己的资源路径；最新app已通过既有原子发布器更新至 `dist/macos/Femacs.app`，真实binary/dump与实测导出相同、严格签名通过，旧完整app备份保留。原独立搭档实际完成根产品TTY6/6、稳定版对照48/48、本地CLI/PTY/batch9/9，含真实终端配置启动、26项按键及文件流程；两次额度错误之间完成的回执另行核实，没有误报为候补运行。候补独立核对源码、结果和当前制品身份后接管审核，未重复这些通过项。
- 搬移副本batch检查退出0：本轮源码导出目录临时不可用时，五个Lisp库以及data/doc/exec路径均实际位于搬移包内，DOC可读、Unicode编辑成功、PID相符，binary/dump前后不变；随后恢复同一源码目录。此为资源搬移的有限证据，不等于Finder启动或原生交互。真实多窗口IME及外部Finder拖放仍待对应验收，桌面当前锁屏，不能用CLI/batch结果代替原生操作。

本轮候选替换有[CLN-20261025-01](../maintenance/cleanup/reports/2026-10-25-01-diagnostic-candidate.md)及[CLN-20261025-02](../maintenance/cleanup/reports/2026-10-25-02-acceptance-guards.md)记录，逐份身份、原始构建/失败/修正与隔离恢复证据保留。G4仅完成234份tools/spikes的有界字面量读者初盘，不是完整依赖证明，未据此批删；G5–G7尚未交付，整个七项目标仍未完成。

# 2026-10-24交付基线：macOS 独立构建与 Lisp 画像（历史读数）

双方同意并实施现代化手册 Stage IV。干净 tracked 源码完整构建终端/Cocoa；新产品与可选参照的十组兼容性观测一致，30/30组、零跳过。真实终端和图形启动读取配置、文件/按键/单窗口输入法已验证；初轮本地总表12/13的C-g测试竞争由明确前缀回执修正，定向复验通过，未把原失败表改成全绿。七轮Lisp/GC采样完成，但quiet=false，不作提速结论。已发布新图形包至 `dist/macos/Femacs.app`；根终端仍是此前交付版本，新干净构建终端保留在独立导出资源树中。

搬移app且原导出暂不可用时，LaunchServices启动/指定文件打开通过；不等于Finder双击实测。外部Finder拖放因锁屏未完成；多窗口自动化仍在第二窗口出现英文直入，根因未定，不宣称通过。另外登记无控制终端的fatal诊断/清理问题，尚未修复。上述缺口使Stage IV原生验收保持打开，详见[本轮证据与下一修复项](modernization.md#stage-iv-d-delivery-evidence-and-open-native-acceptance)。禁止远端CI，旧goal不恢复；后续native编译/分代GC没有提前启用。每批清理见 `maintenance/cleanup/LOG.md`，原始Lisp/数据未更改。

# 当前施工：稳定交付与现代化（2026-10-24）

用户已授权主开发与独立搭档按顺序实施：先本地验收/拖拽/打包/输入法诊断修复，再清理过时基础设施与设备兼容，最后推进 Lisp 编译器、解释器和 GC 优化探索。唯一当前施工入口：[modernization.md](modernization.md)。阶段 I 已通过干净源码构建、终端/原生各 26 项按键、中文文件和独立输入法流程；真实跨窗口拖放仍待验收。完整边界见现代化手册。阶段 II 工具清理已验收：6项旧工具/实验源码退役，4个shell入口收敛，95条实际参照差分通过；平台退役范围未定，所有平台保留。阶段 III 的真实 VM 派发实验没有证明收益，已撤回生产改动；保留语义测试并补齐干净构建的核心 Lisp 编译库字节码生成。相同 VM 隔离对照的指定库加载中位耗时69.520→12.2292毫秒，不能扩展为整体语言提速，新的 JIT/GC 未启用。最终十组语义观测四方一致，现有ERT31项和终端26键通过，独立冻结审核无阻断。旧 goal 保持结案，禁止使用远端 CI。

当前独立终端产物：`build/terminal/port/src/emacs`；Cocoa 产物：`dist/macos/Femacs.app`。历史路径和读数只代表对应时期，不作为当前新候选验收。

---

> Repository layout update: implementation sources now live at the root. Current independent build commands are in `README.md` and `docs/repository-bootstrap.md`. Historical commands and evidence below retain their original context; reference-based comparisons are optional audit tooling, not release build dependencies.

# 当前状态：二期 macOS 图形版可试用（2026-10-23）

原生 Cocoa 图形版已完成构建与实际运行，使用 Rust 核心及 Swift 平台实现。经典 Ctrl/Meta/方向键已修复，性能不作为本期目标。

从仓库根启动：

```sh
open dist/macos/Femacs.app --args -Q
# 已实测的直接启动方式：
dist/macos/Femacs.app/Contents/MacOS/Femacs -Q
# 保留终端版：
build/terminal/port/src/emacs -nw -Q
```

当前交付为 Apple Silicon / macOS 26.6.2 本机版，签名为本地 ad-hoc，不是公证发行版。建议先关闭其他 Femacs 实例，使用 `-Q` 排除个人配置及插件。

- GUI：26/26 原生按键通过，文件保存重开、系统剪贴板、GC 后编辑与正常退出通过；中文、组合字符和彩色 emoji 已实际查看，菜单及原生文件面板已实际操作。
- 终端：680 次普通/checking 编译通过；26 项按键 × 固定参照/端口 = 52/52，包含屏幕光标坐标；两侧中文文件工作流均正常退出。
- 最终打包后另验四项原始故障按键与完整文件流程；实际 dump、数据和 calendar Lisp 都从 app 内加载。严格签名校验通过。
- 590/590 是覆盖检查，不等于完整行为兼容。本机豆包拼音组合/候选提交已验证；其他输入法、所有插件、辅助程序及其他平台未作全面验收；Finder/LaunchServices 冷启动待用户试用，直接可执行文件启动已验证。
- 额外重复声明扫描仍有 15 条配置/作用域相关报告，保留原检查器及基线，不伪称全部门通过。范围与原始失败见施工手册。

输入法补修：候选位置回调把Q符号函数误读为变量的问题已改正，修复提交1baf89d5。解锁后真实豆包拼音切换、marked组字、提交“你好”、取消、M-x输入区组字/取消、保存和正常退出均通过，已恢复原输入源。原26项GUI按键与文件流程回归通过；不能扩展为所有输入法兼容。

[试用方法](macos-tryout.md) · [设计、修复与逐项验收](phase2-macos.md)。一期历史保留，旧 goal 不恢复。

---

## 一期历史：已结案（2026-10-23）

**按用户决定结束本期；macOS 终端版可用，性能未达、GUI 未交付。**
最终入口：[一期收尾报告](phase1-closeout.md)，内含双方工作、设计/问题/经验、模型/会话/goal/运行标识、
token/时间/资源口径、成本未知项与清理记录。
2026-10-23 已按用户决定核对双方会话日志并完成最终归档，不再等待原 CLI 恢复或重新确认；
日志核对范围及较晚计数见收尾报告第 10 节。本次没有重跑产品验收。

## 一期当时可试用的产物

```sh
build/port/src/emacs -nw -Q
```

这是 O2 + switch VM 终端版；PGO 是保留候选，不是默认版本。没有可运行的 Cocoa `.app`。
独立复核记录：680 单元全量重编、链接编码 15/15、dump 成功，真实终端输入/保存/退出 0，92 条差分输入一致。
这不是所有功能或配置已经完整等价的证明。

## 一期当时的口径和遗留

- 590/590 是转写清单覆盖，旧 7/8 是实施者当时限定平台的汇总，不代表完整 GUI 产品已完成。
- 性能本轮默认 bytecode 为参照 1.35–1.59 倍；快速修正候选有功能错误，未采用，不继续深究。
- GUI 只完成八个 Swift 单元编译及 Rust/接口诊断；完整 Cocoa 配置、联编、打包和窗口验收未完成。
- ERT summary/零用例、最新稳定版专项兼容、新候选平台验收及恶意字节码防护仍有证据缺口。
- 原 goal 已由实施者执行 clear。用户原意是 done；
  clear 是协调方传达错误，不能称其已有原生 done 记录。保留旧 ID/统计，不重建、不自动恢复。

## 历史和证据

- [实施者结案总结](phase1-implementer-closeout.md)
- [VM 调查](vm-investigation.md)
- [macOS 独立复核](macos-recheck-20261022.md)
- [macOS GUI 调查](macos-gui-build-20261022.md)
- [旧交接快照](handoff-2026-10-21.md)；`docs/status.md` 保留历史，旧“唯一缺口”不能覆盖上述新发现。

以上为一期历史；二期当前交付与验证边界以本页开头及二期手册为准，不启动新 goal。


G4 D当前读数（2026-10-25，D17）：A/B/C已写齐，旧组合156项证据不冒充一次性全绿。P-340修后本机31.1、全新终端和Cocoa候选各15/15通过；新Mac产品的Info、搬移和严格签名通过，根试用包尚未替换。Android6完整构建及源码保护983.416秒通过，设备13真实输入HELLO、中文剪贴板精确匹配；但快捷键阶段发生原生GC静态根扫描崩溃，GUI/渲染/旋转未验收。独立批处理精确返回`(3 1)`。P-341按实际栈和源码诊断，不禁用GC或跳过根来过关。两次运行均回收完整，CLN23清理自有已停AVD，产品和全部失败证据保留；磁盘空间阻塞已解除，Mac当前仍锁屏。P-339只更正既有off参数说明。最终独立采用、G4签名提交/推送、Mac原生拖放/多窗口IME仍待完成；100%兼容与实际性能提升未据此宣布达成。详见[最新进度](status.md)及[G4清单](tooling-modernization.md)，整个Goal继续。

## D28恢复中的实测检查点（2026-10-27）

P402独立恢复已核实原ENOSPC停止证据，按原哈希复用驱动和首个配置；
新增terminal-baseline的普通/检查两腿构建、链接和dump完成，
共2/20配置、7/61 profile，后续配置仍在编译；没有完整功能或性能验收。
已消费副本见[CLN77](../maintenance/cleanup/reports/2026-10-27-77-recovered-baseline-copies.md)，
旧D26无消费者的手册/冗余保全包见[CLN78](../maintenance/cleanup/reports/2026-10-27-78-superseded-d26-manuals.md)。
总目标仍只完成G4，1/7，剩余6/7；本次不改变计数。

## D28恢复已因TLS配置绑定失败结束（2026-10-28）

实际完成10个终端配置、31个profile含安装参照；首Cocoa两腿因37项TLS
配置在ABI/hash绑定之后加入而被严格比较拒绝，恢复退出1且完整回收。
终端构建成功不证明其配置绑定完整，旧产品不作为P405修复后的最终候选。
P405已在工作区一次写齐早期特征绑定、prepare漂移拒绝和双产品target校验，
独立核查及新统一20配置/61profile重建待执行，未编译/验收此修复。
见[实际失败与修复边界](../maintenance/incidents/2026-10-28-runtime-tls-binding-drift.md)
及[CLN82](../maintenance/cleanup/reports/2026-10-28-82-recovered-terminal-copies.md)。
整个Goal保持1/7，剩余6/7。

## P405/P406修正后统一构建已启动（2026-10-28，D29）

两个回合的独立源码核查确认：早期TLS能力绑定、严格target存在/regular/
无symlink及Cocoa首次准备顺序、header索引刷新、源清单检查已接齐；
当前无新确定缺陷，但静态结论不等于实际构建或功能验收。D29从新统一
冻结源实际启动，按variant交错terminal/cocoa，仍20配置/61profile；
成功配置调用既有逐项账本回收units。retained状态保留为清理待办，不误报完成。
旧D28不复用为新候选，已按[CLN83](../maintenance/cleanup/reports/2026-10-28-83-superseded-d28-runtime-products.md)
退役无消费者制品，源/原失败/日志保留。根试用App未因此替换，整体Goal仍1/7。

D29首对代表实际完成（2026-10-28）：终端/Cocoa联合诊断配置普通和检查腿均构建、dump及保存回执成功；4腿绑定配置与准备后target JSON相等。当前2/20配置、9/61profiles含参照，其他构建仍在运行；这不是TLS功能、全稳定语料、性能或UI验收。首对已消费19130份生成副本按[CLN84](../maintenance/cleanup/reports/2026-10-28-84-new-representative-copies.md)核验汇总，后续配置不重计。

D29 next checkpoint (2026-10-28): terminal-baseline completes, now3/20 configurations and11/61 profiles including the installed reference. The original matrix remains live. CLN85 retires only old D28 registered package resource copies; original frozen inputs and failure evidence remain, with no current-product change or functional/performance acceptance claim. See the [cleanup report](../maintenance/cleanup/reports/2026-10-28-85-superseded-d28-resource-copies.md).

CLN87 storage boundary (2026-10-28):208 copies of already-published cleanup manifests were retired from eight obsolete exports after36 actual Git-blob restorations. Business source bytes and original inventories remain; the old directories now need these metadata inputs restored from pinnedaa707067 before complete original-export verification. Old products retired earlier still require rebuilding. Current D29 is untouched. See the [exact recovery report](../maintenance/cleanup/reports/2026-10-28-87-committed-manifest-export-copies.md).

CLN88 recovery boundary (2026-10-28): ten obsolete source containers now resolve through pinned38f06db8 Git blobs and a private unique-source/evidence store. All13337 needed contents were actually restored and all70104 original regular-source mappings checked. Original path/mode/source inventories and results are also copied into a verified mapping archive. SOURCE-RECOVERY.json in the private store is the recovery entry; these are required historical-source dependencies, not disposable compiler caches. Old programs still require rebuilding. Current D29/root App are unchanged; see the [scope and exact recovery](../maintenance/cleanup/reports/2026-10-28-88-compacted-source-containers.md).

D29监控中断（2026-10-28）：原运行在15/20完整配置、45/61profiles后因session_sample写入ENOSPC退出125，外层已回收。保留实际native回执；原阶段未写出fatal，新增记录明确标注为外部中断观察，并未伪造阶段异常或通过结果。核对原PID退出及原文件身份后释放保全标记，使用同一冻结源/原driver的独立恢复，完成配置须重新核验才能复用。完整功能/兼容/性能/UI尚待验收。见[中断与恢复](../maintenance/incidents/2026-10-28-runtime-d29-monitor-interruption.md)及[CLN89](../maintenance/cleanup/reports/2026-10-28-89-continuation-generated-copies.md)；Goal仍1/7。

D29恢复首个新增配置完成（2026-10-28）：terminal-native-collector-observer普通与检查腿构建、dump和保存回执完成，已到16/20完整配置、49/61profiles；下一配置继续。CLN90回收原已完成15配置30腿用完的src及Cocoa Info暂存12530份/1081451056逻辑字节；终端回执直接引用的Info保留，原日志/回执/制品不变。这是构建与回收检查点，完整功能/兼容/性能/UI仍待验收，Goal1/7。

D29第一次恢复再次因存储中断停止（2026-10-28）：已新增两个完整成功配置，共17/20、51/61profiles；首待补Cocoa配置期间监控session_sample再次ENOSPC，退出125/完整回收。原回执及17配置保留，不能当编译断言失败或全构建通过。旧成功日志按CLN91无损保留并逐份实际解压验证，原路径迁移后需先恢复再读。P407须修复一层来源/固定case路径假设后从17配置继续，不能绕过来源验证、盲重启或重编已完成项。剩余三个Cocoa及全功能/兼容/性能/UI待闭合，Goal1/7。

CLN92恢复空间（2026-10-28）：仅回收两个已中断Cocoa目录的已编译缓存/生成units，14277份、1181584740逻辑字节。17完成配置/51profiles均未引用这两个目录；所选路径全不存在，原7099项冻结输入删前删后完整身份验证通过，配置/原失败及元数据保留。P407来源链实现已写齐，独立静态核查及采用已确认，实际第二次恢复尚待执行；不要把旧中断handle当成运行中。

P407第二次恢复已启动（2026-10-28）：来源链和同名产生者拒绝经独立源码核查与采用，使用原7099项快照/原driver，17配置已全部严格复用核验并开始真实Cocoa续编，完整20配置/61profiles标准不变，尚未全构建通过。CLN93实际恢复4524个Git blob后回收五个旧运行时导出22970份已提交数据副本/758888595逻辑字节；旧源清单不改，须结合CLN87从固定提交恢复对应路径/mode再作完整旧源验收。当前D29、UI/G4试用源及独有源不动，Goal保持1/7。

D29第二次恢复新增Cocoa配置完成（2026-10-28）：cocoa-native-collector普通/检查腿构建、dump及制品保存完成，现18/20完整配置、55/61profiles；余下两个Cocoa继续。CLN94实际核验原程序/68手册/冻结输入及包内Info完整身份后，回收两腿已消费手册暂存910份/94583952逻辑字节；原回执、日志及产品保留，终端Info不动。P407已通过实际来源链及17配置复用路径，仍无全构建/功能/兼容/性能/UI通过结论，Goal1/7。

D29构建检查点19/20（2026-10-28）：第二次恢复的cocoa-native-collector及带observer配置均已实际完成普通/检查两腿，现59/61profiles；最后一个Cocoa基线诊断配置继续。CLN95仅回收与冻结源实际同字节的旧中断半成品资源5056份/179943122逻辑字节；CLN96回收第二个续成Cocoa两腿手册暂存910份/94583952字节。所选路径均不存在，当前候选/冻结源/原失败与回执保留，未宣称全构建、功能/兼容/性能或UI通过，Goal仍1/7。

D29实际编译20/20配置、61/61profiles（2026-10-28）：最后Cocoa基线诊断配置普通/检查腿完成；但最终保全归档时外层退出125/write_report/ENOSPC，原native最终报告缺失，不能据此声称整轮完成或原native回收成功。原guard/owned按创建时间独立核实已退役，输出无句柄，所属保全标记安全释放且零失败。CLN97回收910份已消费手册暂存/94583952逻辑字节；CLN98完整验证138291原文件后仅回收671187835字节未完成归档。实际冻结源、程序、资源、链接输入和原中断结果保留。下一步按显式物理文件保留方式重新严格核验既有20配置，取得新运行的真实完成回执，再作prepare/TLS/完整功能、保护性回归与UI验收；Goal仍1/7。

空间释放后续接（2026-10-28）：P408新运行已严格复用20/20配置、61/61profiles及原host，138291实际原文件核验保留，无重复全量归档；新native退出0并完整回收，原中断不改写。CLN99再清理873份已消费host构建缓存及重复输入/113848544逻辑字节，当前源/制品/链接输入不变，恢复边界见报告。P409自有TLS测试关闭流程修正经独立源码核查，已开始绑定现有61profiles实际验收；此前参照13例中2失败及中断仍保留，未宣称TLS、完整功能/兼容/性能/UI通过，Goal交付仍1/7。

P409实际TLS轮次已显式取消并完整回收（2026-10-28）：原关闭问题在参照程序上通过；共同AEAD用例产生约266MB成功ERT对象，桥接器拒绝读取。仅在7份同类输出后终止己方控制器，实际退出143/cancelled/cleanup完整，不能称全TLS通过或算法断言失败。CLN100完整解析并提取小型状态/耗时/元数据/观测后退役7份冗余diagnostic历史/1864480748字节，原case失败、日志、源/产品及真实取消证据保留。P410已完整写齐流式有界诊断与明确execution-products输入绑定，独立源码核查进行中；原20配置/61profiles保持同一编译源，完整功能/保护/UI验收仍待完成。

CLN101（2026-10-28）：已回收211个退出0且无遗留子进程用例的隔离缓存/591932223逻辑字节，原请求响应与结果哈希不变，失败及608份原始格式夹具保留。此前整体验收没有最终回执，不能视为通过；已实测TLS915/915与夹具608完成仍各限其范围。见[逐项回收与恢复](../maintenance/cleanup/reports/2026-10-28-101-consumed-case-state.md)。

CLN102（2026-10-28）：实际dump保存/恢复48/48通过后，回收80份无后续消费者的试验图像/836810968逻辑字节，原结果SHA及完成回执不变，产品原dump和原始夹具保留。搬移与性能继续使用原产品，整体Goal仍1/7。

CLN103（2026-10-28）：搬移60项功能断言均成功，首轮4项因Spotlight占用回收失败，原报告保持56/60。其后4份副本经全树和无句柄核验退役，回收968654987逻辑字节；收尾字段错误从已存清单修复，未重复删除，原元数据不变。正式测量2727次调用/441次启动及12726样本采集通过，但1818组全为busy-host、960对照无合格项，暂不能判断提速；完整功能/稳定语料/UI仍待闭合，Goal1/7。
