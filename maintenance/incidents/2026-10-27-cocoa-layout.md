# macOS 编辑区错位与标签栏重绘崩溃

本次只修复这处 Cocoa UI。其他开发与原 GC 崩溃调查保持在原检查点；本记录不表示整个图形功能、快捷键、兼容性或现代化目标已完成。

## 原因与修复

- 原生窗口和编辑器内部尺寸不同步：Cocoa 的 `can_set_window_size` 位于第二个标志字第13位，Swift桥接却写第一个字；内部尺寸调整因此跳过原生窗口钩子。`src/abi_shim.rs` 改用现有 UNIT/SHIFT 访问器；标签栏的两项读取和 resized 写入也按相同方式修正，保留其他位及终端布局。
- 第一轮额外窗口回调未解决问题，已撤回。失败候选实际记录 native/view580×508、core492×396；不能把后来的正常重绘当作已修复。
- 修正尺寸后开启标签栏曾实际 SIGSEGV：Cocoa误入终端绘制路径，访问不存在的 frame glyph matrix。`src/xdisp.rs` 恢复原有 Cocoa guard、bar-window matrix 绘制和自动高度/行数调整，不关闭标签栏或忽略无效指针。
- `xtask/src/macos.rs` 的资源复制保留源文件 mtime，避免打包先后次序让未变的 Lisp 源文件误显得比 bytecode 新，并弹出干扰布局的 warnings 窗口。

对照依据为只读参照提交 `fd7afced4036fc1a221567d70705d20a2bd64713` 的相应 frame flag 和 xdisp 分支；参照不参与构建。决策见 `PORTING.md` P-391。

## 本地验证与交付

- 新候选冻结7022项输入，source manifest `b4deb3173ab1cf4582dd09392da624436a78c94641a04f7b10e9e1701cca2c45`；从当前工作树快照完整构建，不冒称仅由本次提交独立构建。较早 runtime WIP 仍单独保留。
- `build/acceptance/g1-layout-cocoa-build-6/results.json`：实际构建、启动镜像和App签名通过；控制器退出0/完整回收，360.108秒，accounted峰值844169216字节。
- `build/acceptance/g1-layout-ui-3/results.json`：真实原生窗口初始65×28、标签栏开启/关闭、85×35、50×20、原生zoom均正常；native/view/core尺寸一致，状态栏及 minibuffer 仍在窗口范围内。原生菜单新建第二个标签页后，主开发实际截图确认两个标题正常绘制；JSON不是标题截图的替代证明。
- 同一实例实测 C-e到point20、左方向键到19、C-a回1。结束时退出0、二进制身份不变；控制器完整回收，333.130秒，accounted峰值187056128字节。
- 独立搭档实际读相关源码、原始C和本轮结果后采用修正，未发现这项有界UI修复的阻断项；未冒称搭档独立操作了原生GUI。
- `tools/test-cocoa-frame-flags.py` 后置保护测试16/16通过：实际生产访问器函数体、两种配置的独立测量位置、邻位保持和读写；这是两字组件测试，完整 frame ABI 由真实编辑器验证，不以组件夹具替代。
- 当前 `dist/macos/Femacs.app` 已替换为同一测试版本；2976项内容、权限、mtime及签名核对一致。程序SHA `ec6e44e3f11818cd5c0148d050d389e93975e860b5af8f9bf3c0d7712e28260b`；dump SHA `d37d66c5f98d886afa770416aaaf911f30c84e4726f85e5aca8b6c531c2967f7`。本机交付回执在 `build/acceptance/g1-layout-delivery-1/results.json`。

## 保留边界

鼠标边角拖动尝试没有产生尺寸变化，不计通过；已通过的是程序改尺寸和原生zoom。中文输入法截获 `C-x t 2` 的前缀问题仍待另项排查；原生菜单 New Tab 已验证。原 GC 越界崩溃仍未解决，本次不把正常UI退出写成GC修复。最早创建/变成 key 之前的瞬时尺寸回调不作为可交互后布局的通过证据。

试用可在仓库根运行 `dist/macos/Femacs.app/Contents/MacOS/Femacs -Q`；本轮不读取或改写用户初始化配置。原始失败、系统崩溃日志和受控结果保留于本机证据目录；删除批次另记清理台账，不把原始机器路径或会话日志提交。

## 发布包普通启动补验

同一程序及dump身份的 `dist/macos/Femacs.app` 在隔离HOME中仅以 `-Q` 启动，没有加载布局验收夹具或个人初始化配置。主开发实际截图确认默认 scratch 编辑区、状态栏及 minibuffer 填满原生窗口；可交互后的 view/content/core 一致为597×508。正常快捷键退出0，程序身份未变，控制器完整回收；136.807秒、accounted峰值142917632字节。本机原始回执见 `build/acceptance/g1-layout-ui-4/results.json`。

三次鼠标拖动未改变尺寸，其中一次落入编辑区并提示未定义的拖动事件；因此鼠标边缘缩放仍未通过，不能以本次截图或此前原生zoom替代。未改产品源码，也未将普通启动验收扩大为个人配置、GC稳定性或完整G1验收。
