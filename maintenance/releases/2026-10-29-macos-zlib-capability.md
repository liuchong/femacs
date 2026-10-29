# macOS 内置解压功能接线

本项解决两个 macOS 目标未启用既有原生 gzip/zlib 解压实现的问题。
它不改变解压算法，不修改必需原测试，也不是全部兼容性验收的结论。

## 实际缺口

`build/acceptance/runtime-p424-codec-regex-time-original-1/results.json`
记录八个完整原文件在安装参照及四个 P424 制品上的运行：451 通过、39 跳过、
零失败或超时。输入、源、程序、dump 和验收工具身份前后不变。

其中 `test/src/decompress-tests.el` 的 `zlib--decompress` 在参照通过，
在两个前端的普通和检查构建均因缺少解压能力而跳过。这四项是功能缺口；
另外 35 项来自参照和候选共同的 seccomp/bwrap 条件，不能合并称为通过。
该轮控制器 25.136 秒退出，完整回收；严格退出码 1 保留。

## 生产改动

- 两个 `build-support/macos/` 目标开启 `HAVE_ZLIB`，保持 macros/features
  同名定义一致；功能字串加入 `ZLIB`，配置选项加入 `--with-zlib`。
- 两个目标的 `libraries.txt` 加入系统 `-lz`。对象清单已有 `decompress.o`；
  `src/emacs.rs` 的既有条件初始化负责注册两个 Lisp 原语。
- 保留 `src/decompress.rs` 的 gzip/zlib 自动识别、16 KiB 输出步长、
  中断检查、错误回滚和允许部分解压的返回行为，不用外部解压进程替代。

已直接对照固定稳定版提交 `a360712c9d272d950d8d8255ef74570f7e90b7d9`
的原函数控制流；参照检出只读，不是正常构建的输入。

## 实际构建与验收

新冻结工作树快照的四个实际制品和 dump 均构建成功，两个 Cocoa App
严格签名检查通过；构建 1068.373 秒，控制器退出 0 并完整回收。
该快照包含其他既有 WIP，不是一个孤立提交的干净检出验收，也不能把
全部制品差异归因于本项。

SDK 与实际 Rust 类型区间投影的 20 项布局/基本类型观测一致，三个 C
函数指针合同通过；头文件和源前后不变，1.859 秒完整回收。两侧结构体
大小 112、对齐 8，实际 SDK/运行库版本均为 1.2.12。此投影只证明所测
声明，不代替完整单元编译和编辑器运行。

原 coding/decompress/doc 三个完整文件，加上八项原生解压集成，在安装
参照及新四制品共 115 项全部通过，零跳过、失败、超时或未执行。
原解压用例在四候选均真通过；gzip/zlib、40000 字节多输出步、非整区域、
损坏/截断默认回滚、允许部分解压及 multibyte 错误路径均已实际执行。
逐项注册与状态同参照，源、输入、程序、dump、资源与 harness 前后不变；
控制器 27.022 秒退出 0、完整回收。

搭档初写的集成宏漏传 ALLOW-PARTIAL；主开发在首次运行前纠正为显式
参数，默认传 nil，部分解压传 t。没有用未执行的静态预期冒充通过。
固定压缩输入分别为 31、19、129、64 字节，完整/部分输出为 11、40000、
11369 字节，不使用外部 gzip 或改写原测试。

集成通过后，新增一项剩余输入计数的关键保护：损坏 gzip 后追加五字节，
允许部分解压时必须返回 5，并用解出的内容替换完整请求区域、保持正确
前后缀和 point。参照及四候选共 5 项通过，零跳过/失败，身份不变；
控制器 16.073 秒退出 0、完整回收。它覆盖集成中返回零的截断输入之外的
重要分支，不重复重跑已经通过且未变化的整组。

证据分别位于 `build/acceptance/runtime-p425-stable-products-1`、
`build/acceptance/runtime-p425-zlib-sdk-abi-1`、
`build/acceptance/runtime-p425-zlib-functional-1` 和
`build/acceptance/runtime-p425-zlib-protection-1`。

## 制品身份

| 制品 | 程序 SHA-256 | 实际 dump SHA-256 |
| --- | --- | --- |
| cocoa-normal | `047cb02029e737860a0c52ac448f713d44dfbd00a949bec4715a3cc5f903b196` | `9e3b46ed5a9ee29a1c9b979053457cc22aa0b1310c2bd19e88e59fe7762c5846` |
| cocoa-checking | `290273650e1d79305aebc63714e7cd2f1ae6d306a3e4ede229a33d915fa82952` | `a18fbca0749f0053a5d1a99cf4dfb6a065a90015693e778d7e70bc8289e122fd` |
| terminal-normal | `dc3fa5564e06e69eb5ec9ba3b337b4bd7d83362f7c6e206eb126216867b9beef` | `a66f2ec92685c5833d847dde6cbb96ee3dda2faa668e0368df03db7829247277` |
| terminal-checking | `85eae829c04588db094350e5f9564fd1bb7a2e60e13e364d517192da09a89b94` | `b3a6574bb6c8e370f7f04537d2ee8dd6f8de892713ca03c3f7e57c8e7ab68ca2` |

集成夹具 SHA-256 为 `b55ec87cca852b4406aa67bd05d048ea4655b13c3418a126dea04101e2ccfbc9`；
后置保护为 `38ccf49190790101c9ed219400890fbdccfadb6709f545f33b218d99f6ac1f3e`。
后置保护在制品构建后编写，单独绑定其输入身份，不声称已存在于构建快照。

## 剩余边界

同一搭档通过 19 次实际只读核对接受此限定组件，包括漏传参数的纠正、
实际制品、集成和后置保护及其边界；116.836 秒退出并完整回收。
签名提交和推送尚待完成，以实际回执为准。
原远程通知 09 的间歇失败、全部稳定语料和实验 native/GC 仍未闭合；
这次解压接线不解释 Base64 间歇失败，不证明物理 GUI 或整个 Goal 完成。
