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

## 鼠标缩放的输入路径复核

独立搭档实际对照固定参照和当前 `EmacsWindow` 初始化、应用 `sendEvent`、`windowWillResize`、`resize(withOldSuperviewSize:)` 与结束回调：有界范围内未发现禁用缩放的具体代码缺陷。窗口仍有原生 resizable 样式、事件转交 AppKit、view宽高自动调整；静态一致不证明真实边缘命中通过。

发布包另以 `-Q -geometry 65x28+350+80` 在隔离HOME启动；native/view/core起始为492×396，窗口含标题栏为492×428。按记录窗口位置拖动右边缘、分别用截图内坐标和屏幕坐标解释拖动标题栏，采集到的原生窗口位置及尺寸均未变化。原有“使用了全局坐标就能修正拖动”的推断未获实际证实；两种坐标控制均无移动，不能据此认定或排除产品边缘缩放缺陷。

本轮不改产品源码，不凭自动拖动失败增加窗口回调。已向用户询问真实手动边缘操作，暂留受控独占窗口等待观察，尚无该项回复；本机中间回执在 `build/acceptance/g1-layout-ui-5/results.json`，并明确没有正常退出或缩放通过的结论。搭档已确认本轮UI-only边界，后续操作证据对其坐标推断的纠正待自然边界回传/确认，不写成已采用。其余开发、GC和输入法前缀排查仍在原检查点。

## 人工缩放实测崩溃与 P-392 修正

用户明确反馈手动拖动右边缘立即崩溃。原独占实例实际SIGSEGV退出，程序SHA仍为 `ec6e44e3f11818cd5c0148d050d389e93975e860b5af8f9bf3c0d7712e28260b`；系统栈为 `EmacsView.windowWillResize` → `esprintf` → `doprnt`，实际AppKit手动缩放入口在栈中。原控制器退出245、完整回收，714.623秒、accounted峰值149143552字节；原 `exit.json`、系统崩溃报告、输出和 `failure-summary.json` 保留于 `build/acceptance/g1-layout-ui-5`。这推翻了仅以自动拖动未成功解释问题的判断；此前静态复核漏掉了活跃缩放时才进入的标题格式化分支。

根因是用 `@_silgen_name` 将Swift的 `CVarArg...` 数组声明绑定到C/Rust的C变参 `esprintf`，两个调用约定不相同。`platform/macos/nsterm.swift` 改由真实生产辅助函数 `ns_window_resize_title` 用原UTF-8标题和Int32十进制尺寸构造完全相同的标题；ARC管理临时字符串，移除不安全导入和中间字符缓冲。原旧标题捕获/恢复、live条件、文字网格以及原生缩放功能保留，没有跳过分支或关闭尺寸标题。此修复不改Rust格式化器，也不处理原GC崩溃。

- `build/acceptance/g1-layout-cocoa-build-7/results.json`：从新冻结工作树完成真实Cocoa构建、运行镜像和签名；控制器退出0/完整回收，346.641秒、accounted峰值873955328字节。仍明确含先前保留WIP，不冒称当前签名HEAD单独完成干净构建。
- `build/acceptance/g1-layout-ui-6/results.json`：新根目录App以隔离HOME、`-Q`运行，未加载布局夹具/个人配置。原生记录696条 `windowWillResize live=true`、696次UTF-8标题逐字节预期匹配、6条live结束回调、193种内容尺寸；最后native/view/core为730×564且中文原始标题恢复。主开发只在此前操作Raise及Control-g，未直接调用delegate或强置live标记；后续真实外部原生缩放事件不冒称为自动拖动成功，也不推断操作者身份。
- 主开发实际截图核对缩放后的编辑区、状态栏和minibuffer填满窗口；随后正常退出0，程序身份不变。原控制器退出0/完整回收，145.962秒、accounted峰值175898624字节。
- 功能通过后，`tools/test-macos-resize-title.py` 提取真实生产辅助函数编译Swift保护测试；ASCII、中文/emoji、组合字符和引号字节保留、Int32边界、空标题与长UTF-8六项均通过。原源码身份不变；控制器退出0/完整回收，2.276秒、accounted峰值347471872字节。结果在 `build/acceptance/g1-resize-title-components-1/results.json`；不以它代替真实原生验收。
- 当前 `dist/macos/Femacs.app` 为新候选，2976项内容、模式、mtime和签名一致。程序SHA `a1f6f56bad59ee4f582dd2c9c7e2daf1d4fe050be5cdf7766ff841ac7b366292`，dump SHA `c6a13f551e94831a55c5aa4201495a61e8d207889b45fb3331c586b79c710ea8`；交付回执在 `build/acceptance/g1-layout-delivery-2/results.json`，原崩溃App保留作身份/诊断回退。

独立搭档实际读取源码、原C、旧App反汇编和新结果，明确采用人工崩溃证据并撤回仅输入问题的判断，未发现本次修复阻断项。旧App指令显示把Swift变参数组放入x2后调用C变参esprintf，原标题/行列没有按C变参传递；因此根因得到机器码交叉确认。搭档独立重计696/696、6条结束及193种尺寸，未冒称它独立操作了GUI。交付回执已由待验更新为本次原生缩放通过，初始待验快照另存。后续原GC问题、中文输入法前缀、其他变参接口及完整G1/现代化验收仍分别保留，不能把本次窗口缩放修复算成全目标完成。其他施工继续保持原检查点。
