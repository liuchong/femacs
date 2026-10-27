# macOS 二期试用

原生图形版位于 `dist/macos/Femacs.app`，使用 Rust 核心与 Swift Cocoa 实现。当前验收主机为 Apple Silicon、macOS 27.0.1；不宣称其他系统或全部扩展包已兼容。历史二期证据见[施工与验收](phase2-macos.md)，最新本地交付范围见[现代化手册](modernization.md)。

当前图形包包含已实测的多窗口输入修复和 Fe 中间版图标；最新 Logo 草图仍在确认。双窗口中文输入及
B→A→B→A切换已有用户实测确认；当前带图标试用版启动、输入和快捷键退出正常。
发行镜像位于 `dist/releases/fe-horn-shoulder/Fe-macos.dmg`。本地包为ad-hoc签名，未公证。
详见[输入修复](../maintenance/incidents/2026-10-27-multiframe-input.md)及
[终端与菜单证据](../maintenance/releases/2026-10-27-terminal-and-native-quit.md)。

从仓库根试用图形版：

```sh
open dist/macos/Femacs.app --args -Q
```

首次使用 `-Q` 排除已有个人配置与插件的影响；不带它会按正常规则读取个人配置。试用前先关闭其他 Femacs 实例。也可以直接运行 `dist/macos/Femacs.app/Contents/MacOS/Femacs -Q`。

从仓库根，构建由当前controlled-run的build保护执行：

```sh
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile build -- env CARGO_TARGET_DIR=build/cargo cargo run -q -p xtask -- build-macos
```

构建中间文件在 `build/macos` 与 `.femacs-cache/macos`，试用交付在 `dist/macos`，均不提交。默认终端版在 `build/terminal/port/src/emacs`，以 `build/terminal/port/src/emacs -nw -Q` 启动。
构建失败会保留独立目标的日志和源输入，不会覆盖默认终端程序或拿旧dump配新程序。

## 键盘

经典控制键应直接识别：C-a/e 行首/尾，C-b/f 字符，C-p/n 行，C-g 取消，C-x C-f 打开、C-x C-s 保存、C-x C-c 退出。
Meta通常对应图形版的Option，由已有 `ns-alternate-modifier` 决定，保持原键图而非另造快捷键。
终端的Meta必须由终端实际发送Escape前缀或对应键盘协议：例如ESC再按x应等效M-x。
不会自动修改用户的WezTerm全局设置；已到达程序的Ctrl/方向键失效属于本期必须修复项。

验收同时检查命令、buffer point、保存结果和屏幕光标；仅C-p/C-n可用不足以判定通过。

历史二期记录已验证原生26项快捷键、文件保存重开、中文/组合字符/彩色emoji、GC后编辑、系统剪贴板、菜单打开/保存/退出及原生文件面板返回。本机豆包拼音的切换、标记组字、候选提交/取消及命令输入区组字已实测；其他输入法尚未完整实测；个人配置、大型插件、打印、辅助程序及其他平台也不在本次通过声明内。没有进行性能优化或作性能达标承诺。


复跑本轮输入验收（从仓库根，每条产品/编译命令由当前 controlled-run 保护）：

```sh
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- python3 tools/test-macos-delivery.py --out build/macos-acceptance/recheck-terminal
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile build -- rustc --edition=2024 tools/fixtures/macos-events.rs -o build/macos-acceptance/macos-events
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- python3 tools/test-macos-delivery.py --gui --binary dist/macos/Femacs.app/Contents/MacOS/Femacs --event-sender build/macos-acceptance/macos-events --out build/macos-acceptance/recheck-gui
```

GUI 按键驱动需要辅助功能授权；没有授权会明确失败。完整文件工作流在同一驱动加 `--file-workflow`，使用独立 `--out` 目录。该流程会临时使用系统纯文本剪贴板并在结束时恢复纯文本内容，无法保存全部富文本格式；运行前先保存需要的剪贴板数据。

输入法补修已修正候选位置查询中的符号接口错误；需关闭旧实例并重新启动同一路径的app。解锁后的真实豆包拼音已完成组字、提交“你好”、取消、命令输入区组字/取消、保存和正常退出；不能扩展为所有输入法兼容。


真实拼音回归入口（需解锁，已安装ABC与拼音源，辅助功能授权有效；结束会恢复原输入源）：

```sh
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile build -- rustc --edition=2024 tools/fixtures/macos-input-source.rs -o build/macos-acceptance/input-source
python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- python3 tools/test-macos-ime.py --binary dist/macos/Femacs.app/Contents/MacOS/Femacs --sender build/macos-acceptance/macos-events --sources build/macos-acceptance/input-source --out build/macos-acceptance/recheck-ime
```

事件发送器沿用上方编译步骤。真实组字测试发送物理字母键，不以Unicode直接注入替代；输入法词典可以影响候选内容，完整结果记录在所选输出目录。


现代化后新增的本地验收入口：`tools/test-release-baseline.py` 覆盖有限的代表性功能组，`tools/runtime-profile.py` 采集真实 Lisp/GC 数据。两者不能代表所有配置或插件兼容。多窗口拼音诊断在上述输入法命令加 `--mode multi`，需重新编译当前事件发送器；旧自动化失败记录保留；2026-10-27用户已在修复版手动确认双窗中文和B→A→B→A全部正常。这不替代未覆盖的候选组字跨窗、取消与所有输入法兼容验收。最新逐项边界见上述输入修复记录。

Stage IV 的原始锁屏/多窗口失败保持历史证据。后续真实Finder拖放已打开正确文件，双窗中文输入已手动通过；当前P393终端候选的六项TTY异常/恢复用例也已通过，诊断和程序/dump身份均保留。原生菜单退出的最新观察及组字跨窗等剩余覆盖继续列为待确认；不能把以上局部通过当成完整G1或100%兼容。

当前 P393 终端候选的26项快捷键、保存重开及退出后终端模式恢复共28项
通过；恢复检查与本机参照使用同一持久PTY方法，保留原始状态位差异，
并实际验证后续输入、回显和全部属性恢复。6项异常TTY入口另有独立回执。
原生菜单退出也已观察完整事件链和自然退出；尚未包含有修改文件的保存
提示或待组字跨窗流程。详见[当前终端与菜单证据](../maintenance/releases/2026-10-27-terminal-and-native-quit.md)。
