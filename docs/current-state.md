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
