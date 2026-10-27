# D28磁盘中断及恢复边界

D28首个组合配置normal/checking构建、链接和dump完成，实际配置process退出0；
产生5个profile记录（含参照）。随后 `terminal-baseline` 的checking腿在写入
对象和生成配置时报告 `No space left on device (os error 28)`。
E0133在该日志中是warning，没有发现独立的同码编译error。

原生执行句柄实际返回125，错误码为 `write_report`、errno 28。最终控制器报告没有保存，
原启动记录仍显示running；因此不能据它推断仍在运行，也不能补造原控制器
峰值、stop reason或cleanup字段。已独立核对记录中的guard及全部5个所属
进程身份均不存活，原输出目录lsof为空；这是停止后的OS证据。

经原artifact API对原绝对journal路径释放已停止owner的保护，回执released且
无失败。首次相对路径调用被locator前置检查拒绝，没有修改保护或数据；没有
用批量chflags或修改账本绕过。共享执行器、硬上限和其他对话进程均未修改。

[CLN75](../cleanup/reports/2026-10-27-75-completed-d28-compiler-copies.md)
回收首个已完成配置用完的生成副本；
[CLN76](../cleanup/reports/2026-10-27-76-superseded-d26-runtime-artifacts.md)
在消费者与身份核对后退役旧D26制品。保留当前试用包、参照、冻结源码、实际
原始错误、输入及制品回执。D26旧products清单从此仅作历史身份记录。

恢复必须保持D28原冻结源和原driver身份，逐项验证并复用已完成配置；未完成
配置使用新的独占目录。原生报告缺失与独立停止观察必须分开记录，不能给
旧运行补写一个虚假的fatal/controller报告。新恢复入口须保持原20配置/
61 profile范围、原始失败和逐配置用完清理，再继续功能/兼容/性能验收。
目前并未完成该恢复，也未宣称TLS功能或整个Goal完成。
