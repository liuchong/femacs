# macOS GUI 构建尝试（2026-10-22）

**未产出可运行的 GUI，未进行图形窗口验收。** 基线 `e125bb72`，工作树起始干净。
本次用户要求编译重写后的 macOS 图形界面。所有编译在单个受控进程树内串行进行；
未恢复旧 goal，没有改生产源、照抄树或终端程序，也没有用另一个编辑器的 GUI 代替。

## 实际结果

- `femacs/platform/macos/` 的八个 Swift 单元分别生成对象成功、退出均为 0：
  macfont、nsfns、nsfont、nsimage、nsmenu、nsselect、nsterm、nsxwidget。
  这是 Foundation/AppKit/CoreText 等 SDK 能解析、源码单元可编译的证据，**不是完整产品可链接的证据**。
  编译仍有警告，例如 nsxwidget 的 raw pointer 到 NSView 条件转换被报为 always fails，不能把退出 0 当成功能正确。
- 现有 `docs/baseline/config/darwin-arm64.json` 明确由关闭 NS 的终端配置生成；
  配置目录只有该目标与 Android 目标，现有 xtask 没有独立 Cocoa 应用打包入口。
- 在独立构建目录为 `frame.c` 的 Rust 单元启用 `have_ns/have_window_system/ns_impl_cocoa`，
  两个编译 pass 均失败：每个 pass 八处 `geometry` 不在作用域（`frame.rs` 的
  `Fx_parse_geometry`）。其声明还依赖 `no_value` 配置关联；这是最小开关诊断，
  **不能代替一个完整冻结的 Cocoa 配置，也不能据此断言补齐配置后仍是这八个错误**。
- 对 Swift 显式 `@_silgen_name` 导入做符号核对：共 1437 个名字，当前终端 Rust 对象加本次
  Swift 对象未提供其中 1000 个。它们包含系统/SDK 和未启用目标符号，**不是 1000 个最终链接错误**。
  其中 544 个还在既有 pending 清单中被明确记为 core-export 欠账，例如 `AREF`、`make_fixnum`、
  `FRAME_NS_VIEW`。需逐项确认其实际 GUI 配置、ABI 与提供者，不能用允许未定义符号绕过。
- 本轮未启动完整 GUI 链接，因为尚无完整 Cocoa 配置和可用 Rust GUI 对象集。
  没有生成 `.app`，没有声称窗口、菜单、输入法、剪贴板或保存可用。

## 结论与后续所需工作

之前“590/590”是转写覆盖计数，既有可用 macOS 产物是终端版；这个计数不覆盖 GUI 产品集成。
要交付真实 GUI，仍需：完整 Cocoa 配置及对应布局/全局表 → Rust 图形条件分支全量构建 →
Swift/Rust 接口提供者与 ABI 对齐 → 完整链接、指纹/dump、应用资源打包 → 真窗口输入/保存/菜单/退出验证。
不能靠把终端程序放进 `.app` 就冒充完成。性能工作保持停止。

## 可复现证据

本轮只读源码和构建诊断；源图未索引，因此结论来自直接源码、编译器和符号表。
[原始结果、逐单元日志及接口差集](baseline/macos-gui-build-20261022/)。对象与完整日志保存在本机
`build/macos-gui-20261022/`；默认终端程序未被覆盖，哈希见 `result.json`。

单元编译配方（逐文件，并在 controlled-run 的 build profile 内执行）：

```sh
xcrun swiftc -parse-as-library -module-name femacs_nsterm -c \
  femacs/platform/macos/nsterm.swift -o build/macos-gui-20261022/nsterm.o
```

八个单元按相同配方使用各自模块名与输出名。Rust 最小 NS flags 诊断通过现有 xtask 的
`build-tu src/frame.c` 执行，构建根和 Cargo 目标目录均隔离；不把该诊断配置登记成可交付目标。
