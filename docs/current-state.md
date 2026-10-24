# 当前交付：macOS 独立构建与 Lisp 画像；原生补验仍有缺口（2026-10-24）

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
