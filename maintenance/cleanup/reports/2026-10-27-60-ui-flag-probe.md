# CLN-20261027-60 · 已消费的 UI 标志保护测试程序

基线提交 `d63502c7`。本批仅退役 `build/acceptance/g1-frame-flag-components-1/probe`，原16/16保护测试已正常结束并回收；它不被当前试用App引用。删除1份普通文件，511576逻辑字节、512000删前分配字节。

删前两次精确文件lsof无句柄；持久化并同步清单后重核设备/inode/mtime/大小/模式/SHA再删除。8项测试源码、原编译/运行日志和结果、本轮独立字位置核对结果及在用试用程序内容在删后SHA不变。窗口验证进程仍由原控制器保护，未管理或结束它。

[逐项身份清单](../manifests/2026-10-27-60-ui-flag-probe.jsonl.gz)仅含路径ID、内容SHA、大小和模式；清单SHA `1c32c27de8e143b2917c6cca193fbdcc10e27a7026e729b83e075bc0f6fbd5bb`。本机精确操作台账在 `build/cleanup/2026-10-27-60/private-ledger.json`。遇权限错误停止，本批无错误。恢复可按保留的probe.rs和原编译参数经controlled-run重新生成程序，不承诺编译器产物逐字节相同。

附带源码核对：实际桥接剩余8个直接字访问在终端/Cocoa两组共16个字位置比对均与独立布局表一致，且对应输入与已测试冻结源一致；这是静态字选择核对，不是全frame ABI或鼠标缩放运行验收。完整记录留本机 `build/acceptance/g1-frame-bridge-audit-1/results.json`。

[统一台账](../LOG.md)
