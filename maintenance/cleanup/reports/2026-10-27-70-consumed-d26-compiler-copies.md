# CLN-20261027-70 · 构建仍在推进时回收已消费编译副本

- source_ref：`7e1c61c7e8a8d4c4ff3be8f8dbecee932da1a38a`。
- 实际执行：2026-10-27T19:46:50+08:00 至 2026-10-27T19:48:14+08:00。
- 范围：`build/acceptance/runtime-d26-current-module-thread-cohort/source/build/runtime` 内，下列12个**已完成配置**的普通/checking两腿 `objects/units/**/*.rs` 生成编译副本。其余配置及cargo缓存不在本批范围。

配置：`cocoa-baseline`, `cocoa-native-collector-observer-diagnostic`, `terminal-baseline`, `terminal-baseline-observer`, `terminal-baseline-observer-diagnostic`, `terminal-collector`, `terminal-collector-observer`, `terminal-native`, `terminal-native-collector`, `terminal-native-collector-observer`, `terminal-native-collector-observer-diagnostic`, `terminal-native-observer`。

## 消费者和保护边界

从每配置实际 `builds/ID/process.json` 读取complete、exit_code=0、未超时及无遗留进程组回收异常；没有把不更新的partial.json当进度。两条腿各自的runtime-build-outcome均为build_complete及artifact_preservation_complete，runtime-product也确认构建完成。两次分别对选定配置做句柄检查，均无打开文件，始终不对仍在构建的整棵D26根宣称闲置。

清理前重新核对D26原owner身份、冻结输入清单SHA、选定目录device/inode及完成回执。流式读取活动保护清单，所选生成Rust副本均不在源、产品、资源、链接或诊断证据保护集合；删除前再次读取活动清单确认未新增消费者。保护清单内已有88190项普通文件在删前删后逐项哈希相同。冻结原源、配置、链接对象/归档、程序/dump、包、Lisp/资源和原始成功/失败记录全部保留；正在编译的配置及在用构建driver不动。

这批回收沿用既有CLN44的“按已完成配置回收”出口，未更改执行器、内存上限、正在运行的构建程序或验收标准。

## 删除、验证和恢复

[逐文件清单](../manifests/2026-10-27-70-generated-rust.jsonl.gz)，SHA256 `dc192283a03650391dbd49ddbdcd990bcf6e5be02e0dbb266d997e5d342564d9`；逐项含路径、角色、SHA、逻辑及分配字节、mode。精确OS身份和句柄、完成快照在本机 `build/cleanup/2026-10-27-70`。冻结输入清单SHA256 `05ef0d660ca992c397f35f342ee9c44a593a3f48d1115356ff240b6514bad2dd`。

再次核对每项device/inode/mtime/mode/大小/SHA，清单及父目录fsync完成后才unlink；实际删除 **112116份、3737290194逻辑字节（约3.48GiB）**，删前分配量3960201216字节，并移除4000个已空生成目录。全部选定文件已不存在，88190项保留内容不变。记录的系统可用空间由5756395520至9672032256字节；同期构建写盘，不能将差额等同于独占释放量或精确APFS共享块回收。

恢复依赖保留的D26冻结源/输入清单、每腿runtime-config/runtime-binding/runtime-inputs、runtime-build-outcome及构建driver身份；在独占恢复输出目录按原runtime-build配方重新生成，不能覆盖当前源码/产品或从已退役units路径直接编译。生成副本为可再生内容，没有保存另一份完整缓存包，也不承诺更换工具链后生成字节恒等。

原有构建正常/checking结果只证明其各自构建及dump，**不代表整个D26、语义、稳定语料或性能验收通过**。本批没有重新运行编译器或产品测试；目标继续active。后续全部构建结束后，CLN68须重新核对剩余消费者再清剩余编译副本/cargo缓存，不能把本批已消费文件重复计数。

[清理台账](../LOG.md) · [清理方法](../README.md)
