# PORTING.md — femacs 的 C → Rust 映射指南（v2）

适用基线：`emacs/` 检出 `fd7afced403`。
本文是逐文件移植时的裁决依据。v2 依据 2026-10-02 两名对抗评审的结论重写：
`docs/reviews/2026-10-02-adversarial-review.md` 记录了每条缺陷、证据与处置，
`docs/reviews/2026-10-02-artifacts.sha256` 是评审时的工件哈希。

配套文件：`docs/port-plan.md`（执行手册）、`LIFETIMES.tsv`（生命周期台账，生成物）、
`docs/port-map.md`（逐文件目标与进度）、`docs/baseline/*.tsv`（参照静态基线）、
`tools/cinventory.py`（唯一的 C/Rust 抽取器）、`tools/extract-abi.py`（清单与判定）。

**未知项一律不得自行发挥**：遇到本文未覆盖的构造，先记入 `docs/ports/NAME.md`，
再在本文追加条目（§11），评审确认后统一；同一构造不允许两种写法。

---

## 0. 总则

### 0.1 编译模型（v2 重写：v1 会产生重复符号）

C 与 Rust 的编译模型必须同构，但不能照抄 `#include` 的物理行为：

1. **一个 `.c` = 一个独立编译单元** = `femacs/<同路径>/NAME.rs`，由 `xtask` 逐个调用
   `rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C opt-level=…
   --cfg 'feature="…"' -o build/NAME.o femacs/<dir>/NAME.rs` 编译。
2. **头文件 = 声明镜像**：`emacs/<dir>/NAME.h` → `femacs/<dir>/include/NAME.rs`（`src/`
   为 `femacs/src/include/NAME.rs`）。镜像里**只允许声明**：
   - 类型（`#[repr(C)]` struct/union/enum）、常量、`macro_rules!` 定义；
   - 函数与变量一律写成 `unsafe extern "C" { pub fn NAME(…); pub static mut NAME: T; }`；
   - 不允许出现 `#[no_mangle]` 的定义、`pub static` 的定义、`impl` 块或任何会产出
     符号的项——否则每个包含它的编译单元都会重复定义（评审已实测 `ld: duplicate symbols`）。
   - 参照里**只以指针出现的不完整类型**（`struct buffer *`）在“不含 buffer.h 的编译单元也要
     能编译”的镜像里拼不出来：Rust 侧写 ABI 等价的 `*mut c_void`，并在该声明上方留一行
     `/// C: <参照原型>`，`spikes/lisp-abi-probe.sh` 按这一行核对参照头（P-55）。
   - **例外：定义在单元里的名字，镜像不能再声明**（2026-10-07 定案，status 3.900）。镜像与单元在
     单 crate 布局里是同一个 crate，`unsafe extern "C" { pub static mut X: T; }` 与单元里的
     `#[no_mangle] pub static mut X: T` 并存就是 `E0428`（第六十六个 `xterm` 切片实测两次）。这类缺席
     写进 `docs/ports/excluded-objects.txt`（一行一个对象，格式 `<参照路径>\t<对象>\t<理由>`），
     `tools/extract-abi.py --check` 照读并**打印规模**（`(N file-scope object(s) recorded as not
     carried …)`），读它的单元按 P-72 rule 4 在用法点自己声明；表里点名了一个参照并不声明的对象算失败
     （陈旧行），所以这本表不会腐烂。
3. **单一定义点**：每个函数/全局对象在**唯一一个** `.rs`（其 `.c` 的镜像）里定义，
   并 `#[no_mangle] pub unsafe extern "C"` 导出；其余单元只通过头镜像声明引用。
4. **包含去重**：`xtask` 先展开包含图（`include!` 链）并按文件去重，等价于 C 的
   `#ifndef NAME_H` 包含卫；同一编译单元内同一镜像只包含一次，避免重定义错误。
5. **宏的单一来源**：所有 `macro_rules!` 定义集中在一个 prelude 单元（`#[macro_export]`），
   其它单元 `use`；不得把宏定义逐字复制进多个镜像。
6. **函数指针身份**：凡在 C 里取地址或比较函数指针的位置（例如 `specpdl` 的
   `unwind_ptr.func == xfree`），该函数必须**只有一个定义**（不得依赖 `static inline`
   的每单元副本）；`INLINE` / `EXTERN_INLINE`（见 §10.6）逐个判定。
7. **`cfg` 的来源**：`--cfg` 由 `xtask` 从同一份配置生成（§9.1/§9.2），不得手写。
8. **`Lisp_Object` 与 `Lisp_Word` 是 C 的指针型别名**（见 §2.2），不做 newtype 包装。
9. **头镜像的额外规则（A5 落地时实测）**：镜像被 `include!` 进编译单元，因此
   - 不允许内部属性（`#![…]`）与 crate 相对路径（`use crate::…`），allow 由包含它的
     单元给出；
   - 访问器（`XLI`/`XIL`/`XTYPE`…）不能写成 `const fn`——指针与整数互转在常量求值里非法，
     与 C 一样按普通内联函数处理；
   - 作为 union 成员的 `#[repr(C)]` 结构体必须 `Copy`（C 结构体天然可平凡复制），
     这是 union 成员的硬性要求，不影响布局。（`spikes/layout.rs` 与 `lisp.rs` 均已实测）
10. **`globals.rs` 是 `<globals.h>` 的镜像**：它同时给出两半——`Q*` 内联函数
    （P-17）与**全部 DEFUN 的 `EXFUN` 原型**（`make-docfile` 写的
    `_Noreturn` / `NO_INLINE` / `EXFUN (Ffoo, maxargs)` 前后缀，见
    `lib-src/make-docfile.c` 740-780）。原型放在 `pub mod subrs { … }` 里再
    `use subrs::*;`：Rust 里"同名声明 + 同名定义"是 `E0428`，而**同名定义会遮蔽整体
    导入**（整体导入优先级最低），语义等于 C 的"原型 + 定义"（A6 实测：TU 内定义
    `Fcons` + 调用 `Ffuncall`/`Fvector` 均编译通过，符号表证明 `CALLMANY!` 调用的是
    自己的 `$f`）。
    **推论（评审 F1）**：镜像里**不许**再手写任何 subr 的原型——C 的 subr 原型只来自
    `<globals.h>`，手写一份就等于在 crate 根放了同名项，定义该 subr 的 TU 立刻 `E0428`
    （`lisp.rs` 原先手写的 `Fcons`/`Ffuncall`/`Fsignal` 已按此删除）。
11. **C 的"原型 ↔ 定义"类型检查在 subr 上失效（评审 F2，P-28）**：因为定义遮蔽整体
    导入，生成的 `EXFUN` 原型不会与 `DEFUN!` 定义比对；C 会在编译期拒绝的
    arity/返回类型不一致（例如 noreturn subr 的 `-> !`）在 Rust 里**静默通过**。
    第一个定义 subr 的 TU（`alloc.c`）开工前必须在 `tools/extract-abi.py` 里补上
    机械替代：把 `DEFUN!` 形式的参数个数与返回类型同 `docs/baseline/subrs.tsv`
    对齐，并禁止 `pub mod subrs` 之外的 crate 根 `F*` 声明。
12. **C 的前向声明没有 Rust 对应物**：`static void f (T *);` 之后再定义，在 Rust 是
    `E0428`，而 Rust 的项与顺序无关。删除声明行、把定义留在 C 的位置，并在
    `docs/ports/NAME.md` 的偏离项里记录。
13. **C 的"先声明变量、再在分支里逐字段赋值"没有 Rust 对应物**（`E0381`：Rust 不支持
    部分初始化）。按 C 自己的零值习惯先整体初始化（`T v = T { 空值 };`），后续字段赋值
    逐条保留，并记录偏离。
14. **包含顺序：`config.rs` → `lisp.rs` → 其他镜像**。`lisp.rs` 里定义 `eassert!` /
    `eassume!` / `CHECK_TYPE` 等宏，其余镜像（`character.rs`、`itree.rs`、`buffer.rs`…）
    在函数体里**文本展开**它们，所以 `include!` 顺序与 C 的"先 `lisp.h`"一致；
    `xtask` 的布局探针也按这个顺序拼装（A6 实测：顺序错了会 `cannot find macro eassert`）。
15. **草稿生成器只做机械部分，且从不写入镜像**：`tools/emit-mirror.py` 从 C 头文件机械地
   产出结构体/联合/typedef/字面常量/`extern` 声明（函数指针字段、位域结构体、无法映射的
   类型会被明确跳过并列入"人工工作"清单）。它的输出是**草稿**（`docs/ports/*-draft.rs`），
   必须逐类型补全并通过布局闸门后才允许合并进镜像——实测把未补全的草稿整块贴进镜像时，
   `xtask layout-check` 立刻以 `sizeof handler` 断言失败拦下，这正是期望行为。
16. **镜像自检（每份镜像提交前）**：类型名扫描必须先去注释/字符串（注释里的 `struct foo` 不是声明）；`counts.tsv` 的下限只许上调；位域要写明存储单元宽度（§1.3）；字符串宏按用途选形（`&CStr` 常量 / `macro_rules!`，§2.1）；头内联函数用 `bash spikes/inline-probe.sh` 与 C 对拍（A6 实测抓到 `multibyte_length` 的 `<<` 优先级使 5 字节形态全错，这是 P-30 的第一块，三道静态闸门都看不见）。
17. **布局断言由工具生成，不手写**：`xtask layout-asserts` 读
   `docs/baseline/layout/c-layout-*.tsv`（C 编译器输出的冻结基线），生成 `const _: () =
   assert!(size_of/offset_of …);`；镜像里没有声明过的类型记为 pending，数量随移植递减，
   但不允许把 pending 当成通过。
19. **版本声明只写参照的修订号，参照开头的版本/版权声明整块不抄**（2026-10-03 定案，同日全树落地）：
    每个单元/镜像的首行是 `// femacs: port of <路径> @ <修订号>.`（镜像写 `mirror of`），**不写**
    `(<产品名> x.y.z)` 这一类产品名+版本的括注。理由：一个"移植自某产品"的版本声明会让人以为本仓库
    就是那个产品，有法律风险；修订号（`fd7afced403`）足够定位参照，`docs/baseline` 的冻结基线里也
    已经有全部可复算的版本信息（例如 `darwin-arm64.macros.tsv` 的 `PACKAGE_STRING`，那是实测值，
    必须原样保留）。**参照文件开头的那块版本/版权/许可声明同样不抄**（从首行标题注释到
    `You should have received a copy …` 的整块 `/* … */`）：它正文是一句"本文件是某产品的一部分"，
    放在本仓库里就是一句关于本仓库的错声明，删掉比改写安全。267 个已落地单元/镜像的该块已一并删除，
    其中首行原本就是该块的文件补上了上面那条首行声明（267+1 个文件，-4,932 行；`docs/status.md`
    3.58）。**字符串字面量/docstring 里的产品名是移植语义**，照抄不动（`src/callproc.c` 三条
    `DEFVAR_LISP` 的 docstring 就是例子），门也要求它们与参照逐字一致。**2026-10-05 收窄**：这一句只对
    **参照的** docstring 与参照的实测值成立；**本程序说自己叫什么**的那些字面量（`PACKAGE_NAME`、`RCS_Id`、
    版本横幅、帧名与帧标题、Android 包名、AppUserModelID、快捷方式描述）按第五、第七、第八半改成
    本仓库自己的名字（`femacs`）——**别照抄**。这条规则有机器门：
    `python3 tools/check-declarations.py`（`gate-fast` 的 declaration gate，status 3.73）把
    四类都扫出来——`decl`（复制的所有权/许可块）、`version`（版本行里带产品名的括注）、
    `title`（注释里的产品名，含 `/// C …` 引文）、`literal`（字符串字面量里的产品名）——四类
    都必须在 `docs/baseline/declarations-allowlist.tsv` 里逐条列出并写明理由；前三类**一行都不该有**
    （有就是缺陷），例外只对字面量开放。`/// C …` 的引文照此：`src/emacs.rs` 的 `RCS_Id` 注释把 C
    写的产品名换成 `<PACKAGE_NAME>` 占位并注明原因，引文其余部分不动（status 3.73）。
    这条规则同样管住**代码里的复述**：单元/镜像的注释不再写"参照的许可/版权/版本块已删除/未抄"这类句子
    （那同样是一句关于本仓库的许可声明），决定只记录在本文档与 `docs/status.md`；2026-10-04 的
    `docs/status.md` 3.90 把已落的 11 处复述清掉（`buffer.rs`/`xfaces.rs`/`dispnew.rs`/`emacs.rs` 与 7 个
    `lib/` 单元/镜像）。
    这条规则现在有第二半，且是机器门：**复述本身也是被扫的对象**——`tools/check-declarations.py` 的
    第四类 `recital` 报注释里谈论许可/版权/产品名的句子（带标识符护栏：`emacs_copyright`、`` `COPYRIGHT` ``、
    `canonicalize-lgpl.rs` 这类**移植的名字**不算），扫描面从 `src`/`lib`/`xtask` 扩到**全部已端口树**
    （批次 F 的 `lib-src`/`exec`/`lwlib`/`oldXMenu`/`nt` 与核心同尺）；`femacs/xtask` 是门自己的源码、
    用散文讲这几类，对它只关 `recital` 这一类（status 3.100）。
    这条规则现在有第三半，同样是机器门：**本仓库自己的散文与清单**（根 `README.md`、`PORTING.md`、
    `LICENSE`、`docs/*.md` 与 `docs/i18n/**`、`femacs/README.md`、三个 `Cargo.toml` 与 `Cargo.lock`）
    此前不在任何门的扫描面里，2026-10-05 的复核在这一面量出 **7 处自述句**（两个"某产品的一比一复刻"
    的首段、两处"与本产品同一许可证"的许可句、`goal.md` 的标题、总计划第 3 行、联调判据里的产品名），
    已全部改成中性表述（参照树、参照构建、我们自己的许可证）；`tools/check-declarations.py` 从此把这一面
    读成 `version`/`title` 两类，**一行都不许有**。四个**被豁免的落点**（`docs/status.md`、
    `docs/acceptance.md`、`docs/baseline/`、`docs/ports/`）各自带理由，并在每次运行时**打印规模**——
    那是记录参照，不是本仓库的自我声明（status 3.340）。
    这条规则现在有第四半（2026-10-05，status 3.413）：**写出"身份行"的那些机械口子**。记录参照身份的
    文件是本仓库的工具写出来的，所以它们的头部也在规则之内——`docs/baseline/l{0,1}-reference.txt`
    是 L0/L1 验收金标，`--golden` 把头部的 `# version:` 行写成参照 `--version` 的第一行原文（即
    产品名+版本号）；两个文件是 `.txt`、不在扫描面的后缀表里，而 `docs/baseline/` 按"冻结参照的实测值"
    整棵树被豁免 ⇒ **这一行对每一道门都不可见**（3.413 实测：改回旧写法立刻 `FAIL … title x1`、exit 1）。
    处置：两个验收工具各增 `version_number()`（只取版本号）与 `version_header_lines()`（并写明为什么
    不重复那个名字），金标头部随之重写；`check-declarations.py` 新增 `BASELINE_PROSE`，**按名字**读这两个
    文件的**头部块**（`header_only()`：遇第一条非 `#` 行即停，记录部分仍归豁免），并把它打进覆盖行。
    **一般化**：凡由本仓库工具写出、又落在豁免树里的身份行，要么只写修订号/版本号，要么进
    `BASELINE_PROSE` 这类**按名字的读法**——豁免只覆盖实测数据，不覆盖我们自己写的头。
    这条规则现在有第五半（2026-10-05，status 3.433）：**版本声明不止"文件的头一行"，还有"程序对
    自己叫什么"的声明**。第一到第四半管的是文件头、注释、复述、以及工具写出的身份行；第一半当年把
    **字符串字面量里的产品名**整体留在门外，理由是"C 的 docstring 是移植语义、门要求它与参照逐字一致"——
    那条对 docstring 成立，对**本程序的自我声明**不成立：`PACKAGE_NAME`/`PACKAGE_STRING`/`RCS_Id` 与
    核心、`etags`、`ebrowse` 的版本横幅是**本程序说自己是谁**（`--version` 打印它们、`ident(1)` 从可执行
    文件里读 `RCS_Id`），照抄下来就是一句"本程序是参照产品"。用户判定（2026-10-05）：**这一类也去掉**。
    处置：这类自我声明写本仓库自己的名字（`femacs`）与参照的版本号；**参照的实测值仍留在
    `docs/baseline/config/`**（那是实测数据，不因本仓库的自称而改，第三半的范围不变）。边界按"这句
    话说的是谁"划：**C 的 docstring 照旧不动**（`tools/extract-abi.py --check` 逐字比对）、
    **COPYRIGHT 与许可证文本照旧**（署名与许可义务，且它们不声称本程序是参照产品）、
    **`PACKAGE_BUGREPORT` 与 Haiku 的 bundle 签名 `application/x-vnd.…` 保留**（前者是参照的实测配置值、
    后者是对外注册名，两者都不是"本程序叫什么"；本轮由 `docs/status.md` 3.433 点名，未改）。
    机器门仍是 `tools/check-declarations.py`：本半落地后读数 `14 → 10` 行、`0 forbidden`、`0 failures`。

    这条规则现在有第六半（2026-10-05，status 3.462）：**我们自己写下的工具、探针与 CI 也在规则之内**。
    第一到第五半逐面推到"被移植树、台账、本仓库散文、工具写出的身份行、程序的自我声明"，但 `tools/`
    （45 `.py` + 16 `.sh`）、`spikes/`（22 `.rs` + 6 `.c` + 2 `.h`）、`.github/workflows/` 从第一轮起
    **没有任何门读过**：它们不在 `SCAN_DIRS`（那是被移植树），也不在 `DOC_DIRS`/`PROSE_DIRS`（那是台账与
    计划集），失败的形状与 3.250（`femacs/platform` 只按名字被扫）和 3.413（两个写身份行的工具靠手找）
    相同。处置：`tools/check-declarations.py` 新增第三个扫描面 `TOOL_DIRS`（仍按**后缀**判读，
    `.py`/`.sh`/`.rs`/`.c`/`.h`/`.cc`/`.s`/`.toml`/`.yml`/`.yaml`），判 `decl`/`version`/`title`/`literal`
    **四类、不判 `recital`**（工具要讲它执行的是什么规则、要引用它剥掉的 C 散文，与 `femacs/xtask` 被
    排除在 `recital` 外同一条理由）；`.py`/`.sh`/TOML/YAML 的注释前缀是 `#` 而不是 C 的，扫描器为此新增
    `HASH_LINE_ANYWHERE`（汇编的列首规则不变，`spikes/` 里的 `.c`/`.rs` 照旧走 C 的规则）。
    **一条豁免规则**（按文件+类，且每次运行**打印规模**）：**以识别或剥掉参照许可散文为本职的工具，必须把
    那段散文写下来**——`tools/check-declarations.py` 的模式表与自测表（`literal`）与它要找到的版本括注旧写法
    （`version`）、`tools/cinventory.py` 说明提取器剥掉哪个短语（`literal`）；两处的 `decl`/`title`/`recital`
    照旧生效。读数：工具面 `93` 个文件、**0 forbidden**（全树 `10 declaration sites (0 forbidden, 10 allowed
    literals), 0 failures` 不变），豁免合计 43 处并打印；`--selftest` **46 → 51 例 / 0 失败**；
    `walked past` 现在也报工具树的边界（`spikes/miri/Cargo.lock`、`spikes/shim/README.md`）。
    这条规则现在有第七半（2026-10-05，status 3.503）：**"本程序叫自己什么"不止 `--version` 打印的那几行，
    还包括它在运行时**向外报出的身份**。第五半改了身份常量与版本横幅，把四类留在门外；用户判定
    （"把之前的代码中的这种声明也去掉"）之后逐条复算，改的是**主语是"本程序"的那些**：
    ① **Android 侧的包名与它派生的 JNI 名字**——40 个 Kotlin 目标的 `package` 声明、11 处身份串（extra key、
    provider authority、`package:` URI、`getApplicationInfo`、`RemoteViews`、`loadClass`）、Rust 侧 41 处按名字
    查找的类名/描述符，与 3 个 `#[unsafe(export_name = "Java_org_femacs_EmacsNative_*")]`；C 344 的
    `NATIVE_NAME` 拼的是**参照的**包名，P-83 ①② 随之写清"包名是本程序自己的、类名/字段名/方法名两侧逐字
    一致"，探针拿参照 `.java` 当替身时把替身的 `package` 行一并重挂；② **Windows 的 AppUserModelID**
    （`nt/runemacs.rs` 的 `USER_MODEL_ID`）；③ **三个 `DEFVAR_LISP` 的 docstring**（`src/callproc.rs` 的三条说
    "文件随 <产品> 一起来"，主语是本程序）。**留下的四类照旧点名**：C 的 `DEFUN` docstring（逐字比对）、
    `COPYRIGHT` 与许可文本（署名与许可义务）、`PACKAGE_BUGREPORT`（指向上游，改它要发明一个不存在的联系
    地址）、Haiku 的 `application/x-vnd.…`（`check-carried.py` 把 `.cc` 钉成参照原文）。机器门是
    `tools/check-declarations.py` 的 `PRODUCT` 表新增的参照**其它注册名**（包名、JNI 前缀、`L"…"`、bug 地址、
    bundle 签名）：读数 `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`，每行带理由。
    **同类站点（点名）**：`src/xterm.c` 31017（**已落库**：xterm 切片 109/status 3.906；该片注释照引了 C 的
    那一句、3.911 按第二十六半改成占位符）、`src/w32term.c` 7834、`src/androidterm.c` 6838、
    `src/haikuterm.c` 4536-4547 的窗口标题与 `nt/addpm.c` 266/297 的快捷方式描述，
    以及 `src/android-emacs.c` 的 `app_process` 类名——落库时写本程序自己的名字。
    这条规则现在有第八半（2026-10-05，status 3.599）：**把"本程序默认叫什么"这一族字面量一次量清**。
    第七半点名的五个站点分散在四个 `.c` 里，容易被读成"各文件的局部细节"；判据不变，仍是"这句话说的是谁"，
    凡是**本程序的默认显示名/帧名/帧标题**就是"本程序自称"，落库时写本仓库自己的名字。这一族点名如下：
    `src/xterm.c` 31017（**已落库并已改**：`femacs/src/xterm.rs` 的 `x_term_init` = `c"femacs"`，xterm 切片 109/
    status 3.906；该片的两处注释照引了 C 的那一句，3.911 按第二十六半改成占位符）、`src/w32term.c` 7834、
    `src/androidterm.c` 6838、`src/haikuterm.c` 4536-4547
    （这三处都是 `dpyinfo->default_name`：值是 C 的默认显示名字面量本身，或它 + `" at "` + `system-name`；
    **本文档不重复写出那个字面量**——本文件也在门的散文面里，而产品名在这里是 `title` 类；
    站点与读数记在 `docs/status.md` 3.599）、
    `nt/addpm.c` 266/297（快捷方式描述；**已落库并已改**：`femacs/nt/addpm.rs` 848/891 = `c"femacs"`）、
    `src/xdisp.c` 29565（`decode_mode_spec` 的 `%F` 分支——它就是**帧名**的默认值，与 C `androidfns.c` 921
    的 `fset_name` 是同一个字段）、`src/xdisp.c` 38856（`syms_of_xdisp` 的 `icon_title_name_format`，
    `Vframe_title_format`/`Vicon_title_format` 的模板，TTY 帧标题就长这样）、`src/android-emacs.c` 的
    参照的 Android 包名 `org.gnu.…`（第七半已把本仓库自己的包名换掉）类名（该文件尚未移植，`femacs/src/android-emacs.rs` 只有首行与
    一句"冻结配置不编它"的声明）。已落先例：`femacs/src/androidfns.rs` 1271
    （status 3.490）、`femacs/src/xdisp.rs` 的两处（status 3.599）。
    **对外名称不受这一族影响（照抄）**：`src/w32.rs`/`nt/addpm.rs` 的 `REG_ROOT`（注册表路径）、
    `EMACS_CLASS`/`pgtk_app_name` 的 `Emacs` 资源类、`src/w32select.rs` 的 `CLASSNAME`（窗口类）、
    `src/sound.rs` 交给声音系统的 MCI 设备别名、Haiku 的 bundle 签名 `application/x-vnd.…`、
    `nt/addpm.rs` 的开始菜单目录名与快捷方式文件名——它们是**与外部（用户配置、系统注册表、WM、开始菜单）
    的约定**，改名改的是行为而不是自称；`nt/addpm.rs` 266/297 的"描述改、文件名不改"就是这条边界的先例。
    **这一族现在有第十三半（2026-10-06，status 3.648）**：第八半把两种**片段形态**（注册表路径与设备别名）
    记成"门的表里没有、所以不在站点计数内"的**已知缺口**——后果是**移植单元自己的注释**一直引着 C 的宏文本
    （含那个名字）而门**零命中**：缺陷正好落在"门声称读过这棵树"的盲区里。本半把两种写法（散文里的单反斜杠、
    C/Rust 字面量里的双反斜杠）都加进门，缺口闭合；站点按判据分开处理：⑴ **注释与台账**里引 C 文本的 12 处
    改写成"C 的宏名/标识符 + 中性说明"（门对注释零容忍）；⑵ **字面量 11 处**（注册表路径 2、开始菜单目录名 2、
    快捷方式文件名 2、建组命令 1、MCI 设备别名 3、窗口类 1）逐行登记进 `docs/baseline/declarations-allowlist.tsv`，
    理由写明"这是与外部约定的名字，不是本程序的自称"。
    读数（2026-10-06 现读）：`tools/check-declarations.py` → **`17 declaration sites (0 forbidden, 17 allowed literals),
    0 failures`**（改前 6/0/0）、`--selftest` **53 例 / 0 失败**；三个被改的单元**行数与语句一字未动**（只动注释行），
    所以本半没有编译、没有重链。
    这条规则现在有第九半（2026-10-05，status 3.613）：**"本程序向外报出的身份"里剩下的那一半也改成本仓库自己的名字**。
    第八半把"本程序默认叫什么"（默认显示名/帧名/帧标题）量清之后，字面量豁免表里还剩两类主语是"本程序"的行，本轮一次收口：
    ⑴ **报缺陷的去处**——参照的实测 bug 地址（冻结 `config.h` 的 `PACKAGE_BUGREPORT`）正是本程序对用户说的"有 bug 请报到这里"，
    照抄等于把用户送去参照产品的邮件列表，本身也是一句"本程序是参照产品"的声明；本仓库真实的去处是自己的 issue 页，于是
    **7 个站点**（`src/emacs.rs` 的身份常量、`--help` 里的那一句、`report-emacs-bug-address` 的 `DEFVAR_LISP` docstring、
    `src/android.rs` 的宏、`lib-src/etags.rs` 的 `--help` 行、`lib-src/asset-directory-tool.rs` 的报错句、Kotlin 目标的
    错误提示）都写成 `https://github.com/liuchong/femacs/issues`；参照的**实测值**仍原样留在 `docs/baseline/config/`
    （那是关于参照的实测数据，第三半的范围不变）。⑵ **Haiku 的 bundle 签名**——它是操作系统把**正在运行的程序**登记在哪个名字下的那个值
    （`BApplication` 的构造、写进可执行文件的签名、通知点击交给谁），照抄就是让本程序在 Haiku 上顶着参照的名字注册；四处
    （`platform/haiku/{be_resources,haiku_select,haiku_support}.cc`）改成本仓库自己的 `application/x-vnd.femacs`。
    **这四行是"逐字节携带"这条规则的第一个例外，而且必须是可读的例外**：`docs/baseline/carried-substitutions.tsv` 逐行记下
    被替换的那一行与理由，`tools/check-carried.py` 读它——默认仍是逐字节相同，一行要不同就必须有行、有行就必被读（行错、行过时、
    没有行却是差异，三种都失败）；读数 `11 carried file(s), 11 baseline row(s), 4 sanctioned substitution(s), 0 failure(s)`，
    `--selftest` 9 例（含"把被替换的行改回参照的值"这一例）。**收口后字面量豁免表只剩一类不是自称**：许可方署名与许可正文
    （`COPYRIGHT`、版本横幅里那句 GPL）——把它改成别的名字就是写一句假署名，所以它留着；读数 `check-declarations` 从
    `17 allowed literals` 降到 **6**、`0 forbidden`、`0 failures`。
    **仍然关不掉的一个洞（点名，不冒充"没有"）**：运行期 `M-x emacs-version` 打出的那行文本来自 `lisp/version.el` 的 `format`，
    而 `lisp/` 是**逐字节照抄的六棵树**之一（`tools/check-verbatim.py` 逐字节守，goal 也这么要求）——那个名字因此在**参照的 elisp 正文**里，
    要动它得先动"六棵树逐字节"这条规则；本半不动它，也不把它算作"已清"。
    是不允许的写法）。
    这条规则现在有第十半（2026-10-05，status 3.619）：**照抄树里"程序自己打印/写出的名字"也改**。第九半点名了唯一
    关不掉的那个洞——运行期 `M-x emacs-version` 打出的那行文本来自 `lisp/version.el` 的 `format`——而它的判据与 C 侧
    完全同一条：**这句话说的是谁**。凡**被复制树在运行期打印或写出**、且主语是本程序的字符串，都写本仓库自己的名字：
    版本横幅（`emacs-version` 的 `format`）、plain splash 的第一行（`Welcome to …` / `This is …`）、fancy splash 与
    about 屏里的自称、三处"本程序自带的免责声明"、`rcirc-id-string` 与 ERC 发给频道/服务器的版本串、以及程序
    **生成**的文件里的身份行（`%%Creator:`、iCalendar 的 `PRODID`、命令摘要的 man page 与 texinfo 标题）。
    **要动照抄树得先有可读的例外**，形状与第九半给 `.cc` 的那张表同源：`docs/baseline/verbatim-substitutions.tsv`
    逐行记下被替换的那一行与理由（24 行），`tools/check-verbatim.py` 读它——默认仍是逐字节相同，一行要不同就必须有行、
    有行就必被读（行的文本对不上、行落在文件外、行过时即"已与参照相同"、没有行却是差异，四种都失败），且替换是
    **机械的**：只换产品名那一个 token，同一行其余字符一个都不动。读数 `6 trees, 4604 files, 24 sanctioned line(s),
    0 failures`，`--selftest` 7 例 / 0 失败，两次真树变异读数各 1 处；14 个改动文件用**参照自己的 reader** 逐个读完
    （`read` 24–860 个 form，0 parse 失败）；运行期读数是用 `build/emacs-ref/src/emacs -L femacs/lisp` 载入本仓库的
    `version.el` 得到的 `femacs 32.0.50 (build …)`，对照组（同一二进制载入参照的 `version.el`）打出的是参照的名字。
    **本半留下的三类，点名**：⑴ **裸 `Emacs`**（不含产品名前缀）——照抄树里的 `;;` 注释、docstring、splash 的缓冲区名
    （`*` + 产品名 + `*` 那一种）与引用它的 docstring（1578 个文件的文件头块属此类）；3.226 ⑤ 把这一族留给你裁决，
    本半不擅自扩（**本文档不重复写出那个字面量**，与第八半同一写法）。⑵ **上游链接与它们的标签**
    （splash/about 里指向 gnu.org 的那几条、`Getting New Versions` 一类"怎么拿到参照的新版本"）。⑶ **许可与署名正文**
    （COPYRIGHT、生成器写出的 GPL 头、`etc/`/`doc/`/`test/` 里的说明文字）——改它就是写一句假署名。
    这条规则现在有第十一半（2026-10-06，status 3.632）：**照抄树里"文件自称属于参照产品"的那句话也改**。
    第十半点名的三类里，第一类"文件头块"（1,578 个文件）当时留给你裁决、第三类"许可与署名正文"当时以"改它就是写一句假署名"为由不动；
    你这次的号令（"把之前的代码中的这种声明也去掉"，与 3.226 ① 的读法同源：**不止对新写的文件生效，已写进树的代码也要清**）
    把**其中属于"声明"的那一句**判给了"改"。判据仍是第十半那一条——**这句话说的是谁**：照抄树每个文件开头那句
    "本文件属于<参照产品>"（**本文档不重复写出那个字面量**，与第八、第十半同一写法）的主语是**本文件**，
    在**本仓库**里它就是把参照产品写成这份文件的归属者 ⇒ 在照抄树里也写本仓库自己的名字（`femacs`）。
    字形与第十半的机械替换完全一致：**只换产品名那一个 token，同一行其余字符一个都不动**，所以每个文件除这些行之外
    仍是逐行相同（行数不变，`check-verbatim` 的逐行判读照旧）。
    规模与读数：**1,796 行**（`lisp` 1,610、`etc` 102、`doc` 18、`leim` 3、`admin` 63，分布在 1,762 个文件里）
    逐行记进 `docs/baseline/verbatim-substitutions.tsv`（该表共 1,796 行 = 第十半的 24 + 本半的 1,772）；
    `python3 tools/check-verbatim.py` → **`6 trees, 4604 files, 1796 sanctioned line(s), 0 failures`**，`--selftest` 7 例 / 0 失败。
    其中三个站点是**生成器**（`emacs-lisp/generate-lisp-file.el` 53、`cedet/semantic/wisent/grammar.el` 355、
    `cedet/semantic/bovine/grammar.el` 493 —— 它们把这句话写进自己生成的文件，正是第十半"程序写出的身份行"那一类），
    八个站点是**程序自己的缓冲区名**（`lisp/startup.el` 2238/2273/2316/2378/2437/2715/2721 与 `doc/emacs/entering.texi` 29），
    两者都按第十半的判据写本仓库自己的名字。
    **按判据不动的两处**：`test/`（goal 文本禁止改测试文件 ⇒ 它的 647 个文件仍逐字节相同）与 `ChangeLog*`
    （把这句话当**历史**引用，不是声明）。
    **仍然没动的**（第十半 ⑥⑶ 的理由不变）：版权行（`Copyright (C) … Foundation, Inc.`）、许可正文三句
    （"… is free software …" / "… is distributed in the hope …" / "… along with …"）、指向上游的链接与"怎么拿到新版本"那一类、
    以及手册与 `etc/NEWS` 里作为**叙述**出现的产品名——改它们就是改上游的许可与归属声明。
    这条规则现在有第十二半（2026-10-06，status 3.641）：**第十一半那句话的其它拼法也收口**。
    第十一半的替换是**逐行精确**匹配（整行 `This file is part of <参照产品>.`，1,772 行），于是同一句
    换了拼法或语种的 19 行整片漏在门外：两行少了句点（`lisp/tree-widget.el:9`、`lisp/progmodes/cl-font-lock.el:12`）、
    `etc/org/README:6` 的 `These files are part of <参照产品>.`、教程里的
    `This version of the tutorial is a part of <参照产品>.`（`TUTORIAL`、`.cn`、`.nl`，含 `.nl` 自己的英文原文段）
    与**同一句的译文**（`.cn` 中文两处、`.nl` 荷兰语两处、`.uk` 乌克兰语两处、`.eo` 世界语、`.fa` 波斯语、
    `.sl` 斯洛文尼亚语、`.he` 希伯来语、`.el_GR` 希腊语两处、`.de` 德语）。字形与第十一半**逐字同形**：
    **只换产品名那一个 token，其余字符一个都不动**；19 行逐行记进 `docs/baseline/verbatim-substitutions.tsv`
    （该表 1,796 → **1,815** 行），读数 `6 trees, 4604 files, 1815 sanctioned line(s), 0 failures`。
    **按判据"不是这一句"的一律不动**（逐条点名）：比较句（`This version of the tutorial, like <参照产品>,
    is copyrighted` 一族，12 个语种）、否定句（`This file is not … part of …`）、许可正文三句、`ChangeLog*`
    （历史引用）、`test/`（goal 文本禁止改测试文件），以及**手册成员句**（`@c This is part of the Emacs manual.`
    与 `@c This is part of the <参照产品> Lisp Reference Manual.`）——主语是**手册**这一文档名，不是本程序的
    产品身份；动它要连手册自己的标题与交叉引用一起改名，属第十半 ⑥⑶ 的"叙述"那一类，仍未动（**本文档不重复
    写出那个字面量**，与第八、第十、第十一半同一写法）。
    这条规则现在有第十四半（2026-10-06，status 3.656）：**照抄树这一面此前没有任何"声明"读者**。
    前十三半的入口一直是"哪一句还说自己是参照产品"，逐面推的顺序是文件头 → 注释 → 复述 → 工具写出的身份行 →
    程序的自我声明 → 工具/探针/CI → 程序向外报出的身份 → 默认显示名 → 照抄树里程序打印/写出的名字 →
    照抄树里"文件自称属于参照产品"的那句话 → 那句话的其它拼法 → 移植单元注释里的**片段**形态。这一串都没覆盖一个洞：
    `tools/check-verbatim.py` 只问"逐字节相同吗"，于是"照抄文件里的一句归属声明没被改名"**没有任何门看得见**，
    而且**重新从参照拷一份文件回来会静默通过**（它逐字节相同）并把那句声明带回来。本半先把这一类量清：
    六棵照抄树里"产品名 + 归属谓词（`part of` / `annex of` / `belongs to`）+ 主语是本仓库的某样东西（本文件/这些文件/某文件名）"
    的行共 **818 处**，逐条归类为 ⑴ **653 处**在 `test/`（goal 文本禁止改测试文件）、⑵ **38 处**在 `ChangeLog*`/`NEWS*`/`HISTORY`
    （把这句话当**历史**引用）、⑶ **13 处**是否定句（`is not part of …`——改它就是把真话改成假话）、
    ⑷ **114 处**是**真声明**，逐行登记进新表 `docs/baseline/verbatim-declaration-keeps.tsv`
    （五列：路径/行号/该行原文/类/理由），**0 处未分类**。
    **其中 4 处是这条规则的漏网（本轮实改，按第十半的机械字形只换产品名那一个 token）**：
    `etc/images/README:23` 与 `etc/images/gnus/README:3`（`The following icons are part of …`——主语是"这些图标"，
    不是光秃的 `This file`，第十一半的逐行精确匹配没覆盖）、`etc/schema/README:1`（`locate.rnc, … are part of …`，
    主语是文件名清单）、以及 `lisp/emulation/viper-ex.el:1904`（Viper 的 `ex` 在 Info 文件缺失时**打印**的那句话，
    属第十半"程序自己打印/写出的名字"那一类）。四行逐行记进 `docs/baseline/verbatim-substitutions.tsv`
    （1,815 → **1,819** 行）；`python3 tools/check-verbatim.py` → **`6 trees, 4604 files, 1819 sanctioned line(s), 0 failures`**。
    **门（新增）**：`tools/check-verbatim-declarations.py` 是照抄树这一面第一台**声明**读者——按形状找候选
    （主语 + `is/are … part of/annex of` + 产品名，**裸 `Emacs` 也算**，因为手册成员句用的就是那个拼法），
    逐条判成四类（`forbidden-tree`/`history`/`negation`/`enumerated`）或**失败**；它还看得见字节门看不见的那一种：
    **某一行已被替换表覆盖、却仍然写着产品名**（即"文件被重新拷回来了"——替换表与树此刻互相矛盾）。
    读数 **`818 candidate(s), 114 enumerated keep(s), 818 classified, 0 failure(s)`**、`--selftest` **9 例 / 0 失败**
    （含一例真树端到端读数）；另一侧 `tools/check-declarations.py` 把它加进 `TOOL_EXEMPT`（**只放 `literal` 一类**，
    理由：它的本职工作就是把产品名写进模式表与自测夹具），读数不变（`17 declaration sites (0 forbidden, 17 allowed
    literals), 0 failures`、`--selftest` 53 例 / 0 失败）。
    **按判据"不动"的一类，点名，不冒充已清**：**手册成员句 108 处**（`@c This is part of the Emacs manual.` 54、
    `@c This is part of the <参照产品> Lisp Reference Manual.` 52、`doc/emacs/maintaining.texi:1` 一行里的两句、
    `doc/misc/autotype.texi:2` 的 `annex` 那句）——这句话的主语是**本文件**、宾语是**手册这一文档**，而那本手册
    逐字摆在同一棵树里、它自己的 `@settitle` 写着自己的名字；**只换这句话的 token** 会让文件声称属于一本树里
    没有的书，并把参照的手册说成本仓库的（`改它就是写一句假署名`）。要清这一类，动的是**手册自己的标题**
    （`doc/` 里 934 行带产品名的叙述），属第九、第十半的"叙述"那一类——本轮把它**逐行登记**（上表 `manual-membership` 类，
    108 行，位置与理由都可复算）。同样点名两条**换行句**（`doc/emacs/emacs-xtra.texi:14` 与 `:87`：主语在上一行、
    本行只剩 `are part of the … manual.`）——它们落在本门的**形状界外**（界写在门的模块文档里），不当成已分类。

    这条规则现在有第十五半（2026-10-06，status 3.666）：**照抄树里的声明可以**换行**，而行形状的读者看不见它**。
    第十四半 ⑤ 点名了两条落在"形状界外"的句子（`doc/emacs/emacs-xtra.texi:14` 与 `:87`：主语在上一行、
    本行只剩 `are part of the … manual.`），并把界写在门的模块文档里；本半把那个界推过去：**当一行以动词开头**
    （`is`/`are`/`was`/`were`，前面是文件自己的行首标记）时，它与其上一行**合并成一句**再走同一套形状
    （续行的行首标记先去掉——`@c ` 这种带字母的前缀正是不能用 `\W+` 一刀切的理由），候选按**续行**记名，
    两张表里写的是合并后的那句话。同时量到两条界并写进门里：⑴ **上一行已经带过 `part of` 的不算续行**
    （否则合并只是把两句粘起来、把上一行已经报过的声明再报一遍：实测 `etc/images/README:45-46` 与
    `lisp/minibuffer.el:4249-4250` 就是这种假阳性），⑵ **`part of` 的宾语必须在 8 个词内点到产品名**
    （否则 `... (@pxref{Display Custom,,, emacs, <参照产品> Manual}) … they are part of the window's body too.`
    这种"行里提到产品、宾语却是别的东西"的散文会被误收；8 词是实测宽度，已见最长的宾语是
    `part of the standard distribution of <参照产品>.`）。
    读数：门 **`820 candidate(s) of which 2 wrapped across two lines, 116 enumerated keep(s),
    820 classified, 0 failure(s)`**、`--selftest` **15 例 / 0 失败**；变异实测——把 `doc/misc/modus-themes.org:3879`
    改回参照的写法（等于从参照重拷一行）⇒ **HEAD 的读者一份读数都不提这一行**（它报 `818 candidate(s), 114
    enumerated keep(s), 2 failure(s)`，那 2 条还是本半新加的登记行"陈旧"），本半的读者报
    `femacs/doc/misc/modus-themes.org:3878-3879 a substitution row says this line carries this repository's
    name, but the statement still names the product …`，字节门同时报 `1819 sanctioned line(s), 1 failures`。
    数据：替换表 **1,819 → 1,820** 行（`doc/misc/modus-themes.org:3879`，按第十半的机械字形只换产品名那一个
    token），登记表 **114 → 116** 行（`doc/emacs/emacs-xtra.texi` 的 14 与 87 两处合并句，类 `manual-membership`，
    理由与第十四半那 108 行相同：宾语是同一棵树里逐字复制的那本手册，改 token 会声称一本树里没有的书）。
    **仍未覆盖的形状，量过是空的**：把宾语写在下一行（`… part of` / `<产品名>.`）这一种——六棵树里"上一行以
    `part of` 结尾"的行共 **153 对**，其中合并句是声明的 **0 对**。第十四半点名的另外两类（`test/` 的 653 处、
    手册成员句 108 处）不动，理由不变。
    这条规则现在有第十六半（2026-10-06，status 3.679）：**"台账目录"这一面自己也有后缀界**。第一到第十五半
    分别推到了被移植树、台账、本仓库散文、工具写出的身份行、程序的自我声明、照抄树的每一句话；而
    `tools/check-declarations.py` 的台账走法是 `rglob("*.md")`——**同目录下的表格与代码草稿没有任何读者**：
    `docs/ports/` 的 7 个 `.txt`、4 个 `.tsv` 与 1 个 `.rs`（`tools/emit-mirror.py` 写出的镜像草稿
    `lisp-draft.rs`）既不在台账走法的后缀里，也不在散文/工具走法里，而**覆盖行从不打印这一界的规模**
    （3.250/3.462 的同一形状，只是落在唯一没有自报边界的面上）。处置（只动门与那一行台账）：
    ⑴ 台账面新增 `DOC_SUFFIXES = (".md", ".txt", ".tsv")` 与 `DOC_SRC_SUFFIXES = (".rs",)`：表格走散文
    模型（一格是散文、不是字面量），草稿走源码模型（`literal` 类要进白名单）；⑵ 根 `.gitignore` 与
    `LIFETIMES.tsv` 按名字进 `PROSE_FILES`，`docs/reviews/*.sha256` 由散文后缀表收下；⑶ 台账/散文两面的
    **边界行现在会打印**，且每个后缀集合只在**自己那组目录**里生效（`.tsv` 在 `docs/ports/` 是"读到"、在
    `docs/baseline/` 是"豁免"——全局判后缀会把后者误记成读过）。
    读数：门 `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`（**不变**）、`--selftest`
    **51 → 58 例 / 0 失败**、覆盖行新增 `read 618 ledger-scope file(s) (606 .md 7 .txt 4 .tsv 1 .rs)`、
    `walked past 0 file(s) of an unscanned kind: none`、`left under a sanctioned prefix: 53 file(s)`
    （`docs/baseline/` 的实测值、日志与验收记录，逐条点名而不是静默）。
    这半量出**一处真漏网**（`docs/ports/excluded-macros.txt` 的 `NATIVE_NAME` 行）：它仍写着参照的 JNI 前缀
    （`Java_org_<参照包名>_EmacsNative_##name`），并把本仓库的替换写成"definition site carries the C symbol
    name"——而 `femacs/src/android.rs` 从第五半（3.503）起导出的就是**本仓库自己的类名**
    `Java_org_femacs_EmacsNative_<name>`（`android.rs` 的 1,663/1,690/1,926 三处 `#[unsafe(export_name = …)]`
    为证）。那一行既**陈述错误**（描述的端口写法与树里不符）又**带着产品名**，而它所在的文件对每一道门都
    不可见，所以两件事都没人发现。改法：按树里的实况重写那一行（保留 C 的站点 `src/android.h` 344 与"Java
    文件"这类路径说法），其余字段与机械字形不动；`tools/extract-abi.py --check emacs/src/android.c
    femacs/src/android.rs` 的读数在改前改后**逐字节相同**（那张表只被名字/类别两列驱动）。
    变异实测（判别力）：把那一行改回旧写法 ⇒ **HEAD 的读者报
    `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`**（看不见），本半的读者报
    `18 declaration sites (1 forbidden, 17 allowed literals), 1 failures`；给 `docs/ports/lisp-draft.rs`
    加一行 `const P16: &CStr = c"<参照产品>";` ⇒ HEAD 的读者 `0 failures` 看不见，本半的读者报
    `18 declaration sites (0 forbidden, 18 allowed literals), 1 failures`（`literal` 没有白名单行 ⇒ 必须失败）。
    两处变异还原后，两端读者都回到 `17 / 0 / 17 / 0`。
    这条规则现在有第十七半（2026-10-06，status 3.692）：**唯一"整目录豁免"的那一面里，注释带是本仓库自己写的**。
    第十六半的覆盖行点名了 `left under a sanctioned prefix: 53 file(s)`（`docs/baseline/` 的"实测值"），而那道
    豁免是**整目录**的：那 51 个文件里写着 398 行 `#` 注释（341 行在首块、57 行在首块**之下**），其中 3 行带着
    产品名——而 3.413 给两份验收黄金样例的读法是"读首块、到第一行非 `#` 就停"，把它推广过来正好看不见
    `docs/baseline/verbatim-substitutions.tsv` 的 38/44 两句（3.666"行形状的读者看不见换行声明"的同一形状，
    落在没有读者自报边界的新面上）。处置（只动门与 3 行注释，行/列数据一行不动）：⑴ 新增 `BASELINE_DIR` 这一面：
    `baseline_table_files()`／`baseline_findings_text()` 把 `#` 行**不论位置**当自家散文读、载荷行一律不读，
    `baseline_stats()` 把读数打印成一行；⑵ `PROSE_EXEMPT` 去掉 `docs/baseline/` 那一条（不再整目录豁免）；
    ⑶ 三行改写为占位与指称（**本文档不重复写出那个字面量**，与第八、第十半同一写法）：`verbatim-substitutions.tsv`
    38 的照抄成员句 → `This file is part of <product>.`、44 的括号里那两个缓冲区名（`*` + 登记名 + `*` 及其
    `About` 形）→ `*<product>*` / `*About <product>*`、`config/source-header-macros.tsv` 1 的
    `Macros Emacs' *own source headers* define` → `Macros the reference's *own source headers* define`。
    读数：门 `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`（**不变**；替换表 1,820 行、
    登记表 116 行、字节门也都不变）、`--selftest` **58 → 64 例 / 0 失败**、覆盖行新增
    `read 398 comment line(s) in 51 docs/baseline file(s) (57 of them below the leading block the goldens'
    reader stops at); left as measured values: 125 payload line(s) naming the product in 3 file(s)`，
    `left under a sanctioned prefix` 那一行随那 53 个文件一起退场（文件级空洞归零），三条前缀仍由
    `left out (sanctioned, counted)` 点名：日志 1、验收记录 1、台账 606。
    变异实测（判别力）：把 38/44 两行改回旧写法 ⇒ **HEAD 的读者报 `17 (0 forbidden, 17 allowed), 0 failures`**
    （看不见），本半的读者报 `19 (2 forbidden, 17 allowed), 2 failures` 并逐字点名那两行；还原后两端都回到
    `17 / 0 / 17 / 0`。（HEAD 的读者在仓外 `/tmp/headchk` 跑、白名单同放一份、`--root` 指本仓：先前把它拷进树里
    跑过一次，它把自己也扫了，那条读数作废。）
    这半同时**量死并写下一处界**：门的表只带产品的登记名，**不带宽 `Emacs`**（第十半点名为留给裁决的第一类；
    照抄树那一面由 `check-verbatim-declarations.py` 的裸-`Emacs` 模式表管）。用 `(?<!GNU )\bEmacs\b` 在门自己的
    四个面上量：**1,410 处**——被移植树 1,334（954 在注释半，是 C 自己的注释文字逐字照抄；380 在字符串/代码半，
    是 docstring 字面量与标识符）、本仓库散文 76（台账 42、散文与清单 34：`README`／`plan`／`port-plan`／
    `PORTING`／评审记录，说的都是"我们在重写 Emacs"这类**指称**）。前者改就是改 C 的字节，后者是叙述不是自我
    声明 ⇒ **本轮不扩门的表**，只改新面里那一行（`config/source-header-macros.tsv` 1，那里产品名被写成冻结头
    文件的**归属者**），并把 1,410 处作为读数点名，不冒充已清。
    这条规则现在有第十八半（2026-10-06，status 3.723）：**那张"实测值"读数数的是整行，而一整行里只有引文列是实测/引文**。
    第十七半那条覆盖行写的是 `left as measured values: 126 payload line(s) naming the product in 3 file(s)`；下一层看进去，126 行里
    **109 行命中的是本仓库自己写的理由列**——`docs/baseline/verbatim-declaration-keeps.tsv` 的 111 行里 108 行共用同一条理由串
    （串里写着"934 `<product>` lines in `doc/`"），`docs/baseline/declarations-allowlist.tsv` 的 12 行里 1 行（`SCHEMA_ID` 那行）的理由
    把参照数据文件的路径与两个同族字面量写进了自家散文。处置：⑴ 新增 `BASELINE_OWN_COLUMN`（两张登记表的理由列 = 第 4 列）与
    `baseline_own_findings()`，该列按**散文模型**读（引文列与实测值列不动），`baseline_stats()` 的读数扩成六元组，覆盖行同时打印
    `read the reason column of 2 registration table(s) as prose (0 cell(s) naming the product)` 与 `left as measured values: 70 payload line(s) …`；
    ⑵ 108 行的共用理由串改成"934 product-name lines in `doc/`"；⑶ `SCHEMA_ID` 那行的理由改成指称（`.gschema.xml` 成员 + "the registry,
    clipboard and device-alias rows above"）。读数：改写前 **20 declaration sites (109 forbidden, 18 allowed literals), 2 failures** ⇒ 改写后
    **18 (0 forbidden, 18 allowed literals), 0 failures**；`--selftest` **64 → 68 例 / 0 失败**；载荷行 **126 → 70**，逐列为
    3 个冻结配置实测值 + 12 行移植字面量引文 + 55 行参照成员句引文，**自家列 0**。判别力：把一行理由改回旧写法 ⇒ 本半的读者报
    `19 (1 forbidden, 18 allowed), 1 failures` 并点名，**HEAD 的读者看不见**（两端还原后都是 `18/0/18/0`）。

    这条规则现在有第十九半（2026-10-06，status 3.750）：**两条整文件豁免自称"那里只是引文"，而"引文"是一句关于文件的断言**。
    第十七、十八半把 `docs/baseline/` 从"整目录豁免"变成带读数的读者之后，`PROSE_EXEMPT` 里还剩两条整文件豁免——
    `docs/status.md`（运行日志）与 `docs/acceptance.md`（验收记录），理由写的是"按需要引用用户指令与参照的实测输出"。
    本半把这条理由读出来：新增 `LOG_SCOPE`，逐行读这两个文件，**凡携带产品名的行**都必须落在一条引文里——
    ⑴ 在 ```` ``` ```` 围栏里（参照 `--version` 的实测输出）；⑵ 产品名两侧是**行内代码跨**，其中含日志硬折行造成的
    唯一形状：**上一行未闭合的跨覆盖到本行第一个反引号之前**（读数里单独计为 `carried`）；⑶ 产品名两侧是**引号对**
    （`"…"`、中文引号与角括号引号——日志就是这样引 C、引照抄树、引自己工具的输出的）；⑷ 该行带着**用户指令自己的话**
    （`版本声明不要再说是` / `把之前的代码中的这种声明也去掉`）。四条之外就是本仓库自己的散文 ⇒ `title` 类发现项。
    读数：覆盖行新增 `read 2 sanctioned log file(s) by their citation band (docs/status.md, docs/acceptance.md):
    78 line(s) carry the product's name - 2 in a fenced block, 70 in a code span (1 of them by a span the previous line
    left open), 4 in a quote pair, 26 the instruction's own words, 0 as our own prose`（**本半自己写进 status 的那一条也在这 26 行里**——它按判据就该算指令引文，不是自家散文）；`left out (sanctioned, counted)`
    那一行随之只剩 `docs/ports/`（台账面本就有读者）；门仍是 `18 (0 forbidden, 18 allowed literals), 0 failures`，
    `--selftest` **68 → 75 例 / 0 失败**（新组 7 例：一例对一种引文形状，外加"日志用自己声音说话"与"空行截断 `carried`"）。
    判别力（两次变异实测）：往 `docs/status.md` 追加一行"本程序是参照产品的重写" ⇒ **FAIL docs/status.md: title x1** 并点名该行、
    读数变 `19 (1 forbidden, 18 allowed), 1 failures`，而同一棵树跑 HEAD 的读者 ⇒ `18/0/18/0` 且照样打印那条豁免（**看不见**）；
    往 `docs/acceptance.md` 追加一句同类声明 ⇒ 同样被点名（两个文件都真的被读了）。还原后两端都回到 `18/0/18/0`。
    **本半一处产品名文本都没改**：两个文件本来就只有引文，改动全在门、自测与文档。**未声称**：这是**文本**门，
    "0 处自家散文"不证明日志里没有别种说法——裸 `Emacs` 仍按 3.692 ⑥ 的裁决留在门外的读数里，不冒充已清。

    这条规则现在有第二十半（2026-10-06，status 3.765）：**"已写进树的产出物"里读数最大的一块——六个照抄树里、
    与文件头首句同主语的那三句许可声明**。前十半依次清了移植单元/镜像的头、抄进来的整块声明、复述、本仓库散文、
    工具写出的身份行、程序自称、工具与探针、运行时对外身份、默认帧名与豁免树里的身份行；第十半把照抄树每份
    文件头的**成员句**（`This file is part of <产品>.`）换成本仓库自己的名字，与它同属一个许可块的另外三句没换：
    **授予句**（`<产品> is free software: you can redistribute it and/or modify`）、**担保句**
    （`<产品> is distributed in the hope that it will be useful`）、**收尾句**
    （`along with <产品>.  If not, see <https://www.gnu.org/licenses/>.`）。判据与第十半同一句：这三句的**主语是
    这份许可授予的对象**，放在"本文件是 femacs 的一部分"下面，就是一句关于本仓库的错声明。
    处置：**只换产品名那一个 token**（`femacs`），其余字符一个都不动，逐行记进
    `docs/baseline/verbatim-substitutions.tsv`（登记行 1,820 → 7,188）。判定只看"去掉行首前缀后句子以哪一句
    开头"——前缀有 `;;`、`#`、`##`、`%`、`@c`、roff 的 `.\"` 与无前缀的 `@copying` 正文，均不参与判定；旧式
    变体（`… is free software; you can redistribute` 一类）一并覆盖。**边界**：`Copyright (C) … Free Software
    Foundation` 与 GPL 正文（`under the terms of the GNU General Public License …`，其中不含产品名）是署名与
    许可义务，照旧一字不动；`test/` 按 goal 文本一行不改（那里的同类句**点名计数**，留待裁决）；
    `ChangeLog*`/`NEWS*` 引的是历史、否定句（`… is not a part of …`）与叙述句（手册标题、教程里的
    `The <产品> Manual` 一类）不属这一半，按 3.692 ⑥ 的裁决留在门外并计数。机器门：`tools/check-verbatim.py`
    逐行读这张表（目标行必须与登记逐字相同、参照行必须不同、无登记的差异即失败），读数
    **1,820 → 7,188 sanctioned line(s)、0 failures**；`check-verbatim-declarations.py` 不变（820 候选 / 116 登记 /
    0 失败），它只读成员句那一种形状。

    这条规则现在有第二十一半（2026-10-06，status 3.796）：**第十半 ⑵ 点名却从未量过的那一类第一次量清**——"上游链接与
    它们的标签"，并把"还剩什么"从断言变成四组数字。读数（本轮实测）：`test/` 树 **2,623 行**带那个字面量（其中 1,939 行与
    第十一/二十半所改的是**同一句**），按 goal 第 5 条"`test/` 与 `lisp/` 一字不许改"与验收的 `diff -r` 项保持**逐字节相同**；
    五个非 test 照抄树剩下的**叙述句 1,469 行**；六个树里 `gnu.org/software/emacs` **101 行**，加 `lisp/startup.el` 的
    **18 个 `:link (`** 与 **28 个 `insert-button`** 链接位（其中 **6 行**标签把参照产品写进名字）；`Copyright (C) … Foundation`
    **2,499 行**。四类各自为什么留在门外写在 status 3.796 ③——链接是**引文**（它指向的文档由本树逐字节携带，第十四半登记
    108 处手册成员句的同一条理由），署名是**义务**，`test/` 是 goal 的**保护**，其余是**叙述**；**要判进规则必须先解除对应那一条**。
    本半不改任何行、不新增门，也不把它们记成已清（门只覆盖它自己那四类声明与成员句那一类）。

    这条规则现在有第二十二半（2026-10-06，status 3.803）：**第十九半那句"1,334 处里 954 处在注释半，是 C 自己的
    注释文字逐字照抄"是一个断言，而不是一次读数**。门的表只带产品的**登记名**、不带宽 `Emacs`，所以这 954 行此前
    没有任何读者——"哪些是照抄的 C 注释、哪些是本仓库自己的话"正是没人量过的那件事。本半给它一个读者：
    `tools/check-own-voice.py` 先按每个单元/镜像**首行自己声明的参照路径**取回参照（再沿该文件的 `include!` 图取回
    它所包含的每个镜像的参照，并取回这些参照文件自己 `#include "..."` 进来的头——`src/ccl.c` 里那条字符集注释
    本来就在 `charset.h`），然后对注释里出现的每一处裸名判三件事：⑴ 落在**引文**里（`` `…` `` 跨或 `"…"` 跨，
    含注释硬折行留下的**未闭合**跨）⇒ 那是 C 自己的字节，留；⑵ 该注释件或它的**任何六词窗口**出现在参照正文里
    ⇒ 是照抄的 C 注释，留；⑶ 其余就是**本仓库自己的声音**，必须写 `C` 或 `the reference`。读数（本轮实测）：
    注释面 **848 处**裸名 = **723 处照抄 C** + **78 处在引文里** + **47 处本仓库自己的声音**；47 行逐行改写
    （**只换指称那一个 token 所在的短语，同一行其余字符一个都不动**；两处落在 C 的**标识符**上——`WM_EMACS_*`
    与前缀 `Emacs` 的窗口消息、`EmacsFrame` 组件——改成反引号引文），改后 **801 处 = 723 + 78 + 0**，门读数
    `0 failure(s)`；`--selftest` **6 例 / 0 失败**（照抄、反引号引文、双引号引文、跨行引文、自家声音、引文旁边
    仍是自家声音）。判别力实测：把 `femacs/src/gtkutil.rs` 88 行改回旧写法 ⇒ 门报 `1 failure(s)` 并逐字点名
    该行（`HEAD` 的读者对同一棵树仍然全绿——它没有这一类），还原后回到 `0`。机器门已接进 `gate-fast`
    （`== own-voice gate ==`）。**边界与未声称**：⑴ **首行没有参照声明的 137 个文件**（生成的镜像，以及首行就是
    C 自己注释、而不是被规则替换掉的那块许可声明的单元——`lib/openat-die.rs` 的第一行就是 C 的注释）**读数点名、
    不冒充已读**，它们的注释面不在本半的覆盖里；⑵ 窗口判据只判"这行字是不是 C 的"，不判 C 的注释**被改过几个词**：
    改写过意思的那些行按 ⑶ 处理，所以本半的"自家声音"里既有端口注记，也有改写过的 C 注释；⑶ 台账、散文与工具
    那三面照旧（第十九半的 76 处读数），本半只读**八个被移植树**；⑷ 这是**文本**门——它证明的是"这行字不在参照
    正文里"，不是"C 没这样写过"。

    这条规则现在有第二十三半（2026-10-07，status 3.886）：**第二十二半点名的"137 个文件没有参照声明"里，有 7 个其实在首行点了参照路径**——`reference_of` 的正则要求那一行以 `@ <修订号>.` 结尾，而 `// femacs: port of lib/c-strcasecmp.c @ fd7afced403.  The file is …` 这类行只是把出处声明写成了首行的**前缀**，于是整个文件的注释面被每一道门跳过。本半改两处判据：⑴ 出处声明按**前缀**认（`^// femacs: (?:port|mirror) of (\S+) @ (\S+)\.`），这 7 个文件随即进入注释面；⑵ **没有参照声明的文件不再被跳过**——它没有可比的参照字节，所以它注释里的裸名只可能是本仓库自己写的，门照读并**打印规模**（`N with no reference declaration on the file's own first line (read against an empty reference set …)`），这正是第十九半留下、第二十二半如实点名的那个类。读数（本轮实测，同一批 `.rs` 的走路集 624 个文件不变）：被跳过数 **137 → 130 → 129**（7 个由 ⑴ 进入；`femacs/lib/openat-die.rs` 按 P-86 ① 补上出处行后从 ⑵ 的类里移出）；注释面裸名 **828 → 832 → 830 处** = **751 照抄 C** + **79 引文** + **0 本仓库自己的声音**，中间那 3 处自家声音逐条是 `lib/include/gnulib_config.rs` 5（`Emacs reaches it through src/config.h` ⇒ `the reference reaches it through src/config.h`）、`lib/include/save-cwd.rs` 4（`Emacs's trimmed variant` ⇒ `the reference's trimmed variant`）、`lib/openat-die.rs` 2（C 自己那句 `This should never happen with Emacs.`；补出处行后按"照抄 C"读，不再是无参照文件里的自家声音，改法**只换指称那一个短语，同行其余字符一个都不动**）。`--selftest` **6 → 8 例 / 0 失败**（新增"首行是前缀"与"无参照文件照样读"两例）。**边界与未声称**：⑴ **129 个文件仍没有出处行**（生成的镜像 `include/{config,globals,buildobj,disptab}.rs`、`abi_shim.rs`、专用 SDK 镜像 `include/{gtk,jni,xlib,dynlib}.rs`，以及首行就是 C 自己注释的单元）——它们**已被读**、0 处裸名（本半改掉的那一处除外），但"P-86 ① 的出处行"本身仍是欠账，数量以门的打印值为准；⑵ 本半只动 `tools/check-own-voice.py` 与那三处文本，不动 `check-declarations.py` 的登记名表，也不动六个照抄树；⑶ 门报"0 处自家声音"仍只证明"这行字不在参照正文里"，不证明 C 没这样写过。

    这条规则现在有第二十四半（2026-10-07，status 3.896）：**"被移植树"这句话的走路面从第一轮起就绑在 `*.rs` 上，平台面因此从未被读过**——`tools/check-own-voice.py` 的 `files()` 是 `for tree in TREES: rglob("*.rs")`，而 `femacs/platform` 是 Swift/Kotlin/C++（3.136 点名它、3.250 写下判据：**列在名单里而走路的后缀一条都不匹配的树，就是走路从不读的树**）。本半把走路**按后缀装订**：八个树的 `.rs` 之外，`femacs/platform` 按 `.swift`/`.kt`/`.cc` 走——这 52 个文件**每一个首行都点了参照**（`.kt` 走 `java/org/…/*.java`、`.swift` 走 `src/ns*.m`/`src/macfont.m`、`.cc` 走 `lib-src/be_resources.cc` 与 `src/haiku_*.cc`），所以同一条"这句话说的是谁"的判据直接适用。后缀面的实测：全 `femacs/` 里首行带出处声明的文件是 **600 `.rs` + 40 `.kt` + 7 `.swift` + 5 `.cc` = 652**，`.rs` 之外一个都没有漏在三种后缀之外。读数（两侧都在本修订）：走路集 **624 → 676 个文件**；注释面裸名 **834 + 169（平台面）= 1003 → 1002 = 910 照抄参照正文 + 92 引文 + 0 本仓库自己的声音**（`.rs` 一侧 755 + 79 + 0 不动；平台面 52 个文件贡献 169 处 = 155 照抄 + 13 引文 + **1 处自家声音**）。那 1 处逐条是 `femacs/platform/macos/nsfns.swift` 1626：`/// C 536-549.  Called by Emacs redisplay code; …` ⇒ `Called by the reference's redisplay code; …`——C 536-549 的原文那句（`This function should be called by … redisplay code to set the name…`）是照抄、照旧不动，改的只是这一行**转述句**里指称参照的那一个短语，同行其余字符一个都不动（改后该行折成两行，第一行 91 列）。`--selftest` **8 → 9 例 / 0 失败**（新增"走路按平台后缀装订"一例：`.swift`/`.kt`/`.cc` 被走，同目录的 `.m` 不被走）。**边界与未声称**：⑴ 平台面这一道读者只管**注释散文**——语法、导入面、`@_cdecl` 的导出符号与"能不能编"仍归各自的 `swiftc`/`kotlinc` 探针（3.164 的 `check-swift-units.py` 只比函数名集合）；⑵ 六个照抄树、`docs/` 与门的登记名表本半一字未动；⑶ 门报"0 处自家声音"仍只证明"这行字不在参照正文里"，不证明参照没这样写过。

    这条规则现在有第二十五半（2026-10-07，status 3.905）：**裸名读者第一次走进"本仓库自己的工具、探针与 CI"这一面**。
    前二十四半的走路面一直是"被移植树"（`TREES` 八个 + 第二十四半装订的 `femacs/platform` 后缀），而 `tools/`、`spikes/`、
    `.github/`、`femacs/xtask` 这四处**没有任何读者读过**——声明门从第六半（status 3.462）起就按后缀读它们，理由写得很明白
    （"我们自己写下的工具、探针与 CI 也在规则之内"），裸名那一面却从未跟上。这一面的判据比移植面简单：**这四个树里没有一行是
    照抄的**（出处句、镜像、C 注释都不进来），所以**不需要参照集合**；读的是**散文**——注释、docstring、以及**工具打印出去的
    字符串字面量**（移植面上"字符串是 C 的字节"这条理由在这里不成立；`femacs/xtask` 的 `gate-full` 消息是跨行续写的 `"… \`），
    `` `…` `` 与注释里的 `"…"` 仍是引文，字面量自己的引号不是。三个文件按名字豁免并**打印规模**：
    `tools/check-declarations.py`、`tools/check-own-voice.py`、`tools/check-verbatim-declarations.py`——把产品名写下来是它们的
    本职（模式表与自测夹具），与声明门 `TOOL_EXEMPT` 给它们的 `literal` 类同一条理由。读数（两侧都在本修订）：移植面
    **1003 = 911 照抄 + 92 引文 + 0 自家声音 / 676 个文件**不动；工具面 **23 处 = 0 引文 + 23 自家声音 ⇒ 22 处逐条改写 + 1 处
    点名保留之后为 0**（走路集 **105** 个文件、豁免 **3**、`walked past 2 file(s) of an unscanned kind: spikes/miri/Cargo.lock,
    spikes/shim/README.md`——3.250 那条规则第一次在这一面**打印**出来）；`--selftest` **9 → 16 例 / 0 失败**。22 处的分布是
    `tools/cinventory.py` 10、`tools/gen-cond-macros.py` 5、`tools/gen-port-map.py` 2、`.github/workflows/ci.yml` 1、
    `spikes/va_probe.c` 1、`femacs/xtask/src/main.rs` 1、`tools/acceptance-l1.py` 1、`tools/check-citations.py` 1、
    `tools/check-citation-text.py` 1，改法**只换指称那些词**（`Emacs's K&R style` ⇒ `the reference's K&R style` 一类），
    需要重新折行的段落按原文件自己的宽度重排。**点名保留的 1 处**是 `tools/gen-port-map.py` 16 的 `Emacs Lisp`——**这门语言
    自己的名字**，与"手册成员句"同一条判据（主语不是本程序），动它就是动一个外部专名，本轮不改、只作读数。**本半不做**
    （点名，不冒充已清）：本仓库的散文与台账仍按第十七半（3.692 ⑥）的裁决留在门外（那是**指称**：`README`／`plan`／
    `PORTING`／评审记录说"我们在重写…"），其 76 处读数不变；这一面读的是**散文**，工具的行为、输出与它证明的事一字未动；
    `femacs/xtask` 那条消息的**文本**改了、它所在的 `eprintln!` 没编译过（本片不编译任何东西）。
    这条规则现在有第二十六半（2026-10-07，status 3.911）：**第七/第八半分点的"还没移植的同类站点"里，
    `src/xterm.c` 31017 落库时把 C 的那一句连字面量一起引进了注释**。xterm 切片 109（status 3.906）把
    `x_term_init` 整身落库，**运行期那个值照第八半写成本仓库自己的名字**（`femacs/src/xterm.rs` 的
    `let title: &CStr = c"femacs";`）——问题不在值，在**同一片落的注释**：两处（`x_term_init` 的 doc 注释与
    C 31017 上方的块注释）把 `title[] = …` 那一句**连 C 的字面量一起照引**。门的 `title` 类对注释**一行都不许有**
    （第三半起就明说含 `/// C …` 引文），所以 3.906 落库后的读数是
    `20 declaration sites (2 forbidden, 18 allowed literals), 2 failures`——判据正是第八半自己写下的那句：
    凡是**本程序的默认显示名/帧名/帧标题**就是本程序自称。处置按 `src/emacs.rs` 的 `RCS_Id` 注释先例
    （status 3.73）：**引文里的产品名写成 `<PACKAGE_NAME>` 占位符**，并在这段散文里写明为什么；引文其余部分、
    它旁边的说明、运行期的 `c"femacs"`、语句与行序一字未动，两处注释各多一行
    （`femacs/src/xterm.rs` 31,682 → 31,684 行，md5 `bec354a7…` → `9bdd5776…`）。读数回到
    `18 declaration sites (0 forbidden, 18 allowed literals), 0 failures`，`check-own-voice` 0 failure(s)、
    `check-citations` 676 文件 / 0 drift、`check-port-lines` 0 ratchet row；**本半只动注释行，因此没有编译、没有重链**。
    **未声称**：门报 0 forbidden 只证明这两行字不在本仓库的注释里（引文仍是 C 的句子，只有名字换了占位符），
    不证明参照没这样写过；第八半点名的另外四处（`src/w32term.c` 7834、`src/androidterm.c` 6838、
    `src/haikuterm.c` 4536-4547、`src/android-emacs.c` 的 `app_process` 类名）**仍未落库**，仍欠各自的 lane。
    这条规则现在有第二十七半（2026-10-07，status 3.931）：**照抄树里"程序装进系统之后，系统拿什么名字称呼它"这一面此前从未被读过**。
    前二十六半的入口一直是"哪一句还说自己是参照产品"，逐面推的顺序是文件头 → 注释 → 复述 → 工具写出的身份行 → 程序的自我声明 →
    工具/探针/CI → 程序向外报出的身份 → 默认显示名 → 照抄树里程序打印/写出的名字 → 照抄树里"文件自称属于参照产品"的那句话 →
    那句话的其它拼法 → 移植单元注释里的片段 → 平台面与工具面的裸名。**这些面读的都是"程序运行时打印什么"，没有一个读"程序安装后注册成什么"**：
    AppStream 元数据的 `<name>`、启动器入口的 `Comment=`、Windows 卸载列表里的 `DisplayName`、splash 图像自己的描述——它们是
    安装器/软件中心/文件管理器拿去向用户介绍本程序的那句，主语是本程序（第五、第八、第九半把同一族的 C 侧落点 `PACKAGE_NAME`、
    `AppUserModelID`、快捷方式描述、`PACKAGE_BUGREPORT` 改成过本仓库自己的名字），而它们全在逐字节照抄的六棵树里，离第十半那份 24 行的清单最近。
    字形仍是第十半的**机械替换**：只换产品名那一个 token，其余字符一个都不动，逐行记进 `docs/baseline/verbatim-substitutions.tsv`
    （**7,188 → 7,197 行**，本半新增 9 行）。**9 行 / 7 个文件**：`etc/emacs.metainfo.xml` 6（`<name>`）、11 与 15（组件自己的描述两行）、
    `etc/emacs-mail.desktop` 3 与 `etc/emacsclient-mail.desktop` 3（`Comment=`）、`etc/images/splash.svg` 85（`dc:description`）、
    `lisp/mail/emacsbug.el` 212（`report-emacs-bug` 的 docstring——本程序叫用户把缺陷报给**它自己**）与它在 `lisp/ldefs-boot.el`
    10702 的**生成副本**（两处必须同步）、`admin/nt/dist-build/emacs.nsi` 77（安装程序写进 Windows 卸载项的 `DisplayName`）。
    读数：`python3 tools/check-verbatim.py` **`6 trees, 4604 files, 7196 → 7197 sanctioned line(s), 0 failures`**、`--selftest` **7 例 / 0 失败**；
    **三处真树变异**各 1 处失败（把 metainfo 6 改回参照 ⇒ 报"行是 ` <name>…</name>`，表里写的是 ` <name>femacs</name>`"（**本文档不重复写出那个字面量**，与第八、第十、第十一半同一写法）；把 emacsbug 212
    改成第三种文本 ⇒ 报"行是 …，表里写的是 …"；把行与表同时改成参照的原文 ⇒ 报"该行已与参照相同（行过时）"）；两个 XML 与两个 `.desktop`
    用本机解析器读完（XML well-formed；`[Desktop Entry]` 头在、键数 9 与 15）；两个改动的 elisp 用**参照自己的 reader** 读：
    `emacsbug.el` 30 个 form、`report-emacs-bug` 的 docstring 首行是 `"Report a bug in femacs."`，`ldefs-boot.el` 里那条 `(autoload …)`
    的 docstring 同样是 `"Report a bug in femacs.`（**直接读那一个 form**；全文件 5,424 个 form 的循环读法在这个文件上与它不等价，本半不引用那一条读数）。
    **按判据不动、逐类点名计数**（判据不变：这句话说的是谁）：⑴ 参考卡片与教程（`etc/refcards/*` 77 行、`etc/tutorials/*` 57 行）与手册
    （`doc/` 853 行）里的文档名与正文——句子说的是**那份文档**；⑵ `ChangeLog*`/`NEWS*`/`HISTORY` 一族 288 行——历史引用；
    ⑶ `admin/nt/dist-build/build-zips.sh` 151 与 `admin/quick-install-emacs` 178 各一行——它们是**解析参照 `configure.ac` 的 `AC_INIT` 行**的 sed，改模式等于改行为；
    ⑷ `admin/admin.el` 85/822/925 与 `etc/NEWS`——**发布记录**（公告与 NEWS 标题那一类，第十半 ⑶ 的理由不变）；
    ⑸ `test/` 2,623 行——goal 第 5 条与验收第 5 条的保护（第十四半的 `forbidden-tree` 类）；⑹ **裸 `Emacs`** 这一族（六个照抄树 **23,977 行**）
    仍按第十半 ⑴ 留给用户裁决，本半只作读数、一行不动；⑺ `etc/images/splash.png` 的 tEXt `Description`（与上面那行 SVG 是同一句）与 SVG 不同：
    **二进制资产没有"行"**——`tools/check-verbatim.py` 的行表按 `\n` 切分、按 UTF-8 文本比对，所以这一处无法用行登记收口，本半点名它、不改它。
    这条规则现在有第二十八半（2026-10-07，status 3.956）：**`man` 印在页眉上的那一格、以及软件中心的注册名，此前
    没有任何一半读过**。前二十七半的入口逐面推过"程序运行时打印什么"（文件头、注释、复述、工具写出的身份行、程序的自我声明、
    工具与探针、向外报出的身份、默认显示名、照抄树里打印/写出的名字、文件自称属于谁、那句话的其它拼法、注释里的片段、
    平台面与工具面的裸名、安装后的注册名），而 `.TH` 的**第四栏**——`man` 把程序名与版本印在**每一页页眉**上的那一格——
    从第一轮起就没有读者：它既不是第十四/十五半要的形状（`part of`），也不是第十半读的 elisp `format` 串，
    所以两份照抄树普查都不看它。判据仍是"这句话说的是谁"：第四栏的主语是**本程序**，而它把参照产品名与本程序的
    版本号写在同一个字段里（`emacs.1.in` 的第四栏是 `"<参照产品名> @version@"`，安装时 `@version@` 换成本仓库的版本号）
    ⇒ 写本仓库自己的名字。
    **同一格里认出来、按判据不动的是另一半**：`.TH` 的第一栏是**命令名**（`EMACS`/`EMACSCLIENT`/`EBROWSE`/`ETAGS`，
    用户敲的那个名字）、第五栏是**手册系列**（`"GNU"`，说的是这份文档属于哪个手册集合，与第十四半的 `manual-membership` 同类）、
    `etags.1` 的第四栏是 `"GNU Tools"`（一个来源类别，不含产品名）——三处都保留，逐条点名。
    同一面第二次漏的是 **AppStream 的组件 id**：第二十七半读了 `etc/emacs.metainfo.xml`、改了它的 `<name>` 与两行描述，
    **把第 4 行的 id 留在原地**——那个 id 是软件中心把本程序登记在哪个名字下的值，与第九半的 Haiku bundle 签名、
    第七半的 Android 包名同一判据 ⇒ 写本仓库自己的反向域名注册名（`org.femacs`，与 `femacs/platform/android/kotlin/*.kt`
    的 `package org.femacs` 逐字一致；**本文档不重复写出参照的那个 id**，与第八、第十、第十一、第二十七半同一写法）。
    字形仍是第十半的**机械替换**：只换产品名那一个 token，同一行其余字符一个都不动，逐行记进
    `docs/baseline/verbatim-substitutions.tsv`（**7,197 → 7,202 行**，本半新增 5 行 / 4 个文件）。
    **5 个站点逐条点名**：`doc/man/emacs.1.in` 2（第四栏的版本字段）与 6（`NAME` 段——`whatis`/`apropos` 交给用户的
    那句"这个程序做什么"，与第二十七半改的桌面文件 `Comment=` 是同一句话）、`doc/man/emacsclient.1` 2、
    `doc/man/ebrowse.1` 2（同一栏）、`etc/emacs.metainfo.xml` 4（组件 id）。
    读数：`python3 tools/check-verbatim.py` **`6 trees, 4604 files, 7,197 → 7,202 sanctioned line(s), 0 failures`**
    （逐树 `doc` 85 → **89**、`etc` 459 → **460**，六棵树的文件数逐个不变）、`--selftest` **7 例 / 0 失败**；
    **三处真树变异**各 1 处失败（把 `emacs.1.in` 2 改回参照 ⇒ 报"1 changed"、退出 1；改成第三种文本 ⇒ 同样退出 1；
    把行与表**同时**改成参照的原文 ⇒ 同样退出 1，即"该行已与参照相同、行过时"）；改动前四个手册页与参照 `cmp`
    **逐字节相同**（所以这 5 行是新增登记，不是覆盖旧行）；`check-verbatim-declarations` **820 candidate / 116 keep /
    0 failure** 并把表的规模报成 **7,202 行**（与 `check-verbatim` 同源）；`check-declarations` **18 declaration sites
    (0 forbidden, 18 allowed literals), 0 failures**（`docs/baseline/` 的 reason 列仍 **0 cell(s) naming the product**、
    两份日志仍 **0 as our own prose**）。
    **按判据不动、点名计数**：⑴ `doc/man/*` **正文**里提到参照产品的行（`emacs.1.in` 的 `DESCRIPTION` 段、`etags.1` 305
    引的手册名、各页的 `COPYING` 注释）——句子说的是参照产品或那份文档（第二十七半 ⑴ 的 853 行同源）；
    ⑵ `.SH NAME` 里的**裸名**两处（`emacsclient.1` 6、`etags.1` 9）——第十半 ⑴ 把裸名这一族留给用户裁决，本半只作读数；
    ⑶ `etc/emacs.desktop`/`emacsclient.desktop`/`*-mail.desktop` 的 `Name=`（4 处裸名）与 `etc/emacs.service` 6 的
    `Description=`——同 ⑵；⑷ 同族的**约定名**（`StartupWMClass=Emacs`、`Icon=emacs`、`Exec=`/`ExecStart=` 的命令名、
    `.TH` 第一栏）——第八半的边界（与 WM/桌面/图标/路径的约定）不变。
    **未声称**：没有任何运行证据——本机没有把改后的手册页装进 `man` 的路径树、也没有重建 `whatis`/`apropos` 数据库；
    改的是文本，读数全部来自逐字节门与变异实测。同一批读数与上面的"不动"清单记在 `docs/status.md` 3.956。
    这条规则现在有第三十半（2026-10-08，status 4.008）：**"这棵树自己的手册叫什么"这一面第一次被读**。
    第十四/十五半点名登记、此后历半"按判据不动"的那一类（`manual-membership`，116 行）自己写着出口：
    **要清它，动的是手册自己的标题**——本半就动它，并按第八半"一族字面量一次量清"的手法，把**本仓库自己的两本手册**
    （`doc/emacs` 与 `doc/lispref`）在六棵照抄树里被点名的**每一个**地方当成一个身份一次改名：手册自己的标题
    （`@settitle`/`@title`/`@titlefont`/`@shorttitlepage`、封面/书脊/copying 页）、指向它们的
    `@ref`/`@xref`/`@pxref`/`@cite`（含跨手册的第五栏）、照抄树里对这个名字的散文引用、程序自己名字里带手册名的
    字符串（菜单标签、快速帮助标签表、`:tag`/`:help`/`:help-echo`、splash 的 `:link`）、docstring 与注释、
    文件头标题行、以及那 116 行**成员句**（含两条换行句：`doc/emacs/emacs-xtra.texi` 12-14 与 85-87）。
    字形仍是第十半的**机械替换**（只换产品名那一个 token，`GNU ` 前缀随产品名一起去掉，同一行其余字符一个都不动），
    **963 行 / 207 个文件**逐行记进 `docs/baseline/verbatim-substitutions.tsv`（登记行 **7,211 → 8,174**），
    读数 `python3 tools/check-verbatim.py` → **`6 trees, 4604 files, 7,210 → 8,173 sanctioned line(s), 0 failures`**
    （逐树 `doc` 89 → **953**、`lisp` 6,385 → **6,472**、`etc` 465 → **473**、`admin` 256 → **260**；`test`/`leim`
    与六棵树的文件数不动）。那一类豁免随之**整类撤出** `docs/baseline/verbatim-declaration-keeps.tsv`
    （116 → 0；表从 117 行降到 7 行 = 列头 + 6 行其它类）：行已改名，门不再把它们读成候选，读数
    **`710 candidate(s) of which 0 wrapped across two lines, 6 enumerated keep(s), 710 classified, 0 failure(s)`**
    （上一步 820/116）。
    **按判据不动、逐条点名（11 处）**：句子的主语是**参照方的发行渠道**、不是本程序的手册——`admin/make-manuals` 2/42 与
    `admin/upload-manuals` 3/45（上传到 `gnu.org`）、`doc/man/emacs.1.in` 561、`doc/misc/efaq.texi` 564/569、
    `doc/emacs/help.texi` 835、`lisp/help.el` 731（**从 FSF 买印刷本**那一句）、`doc/misc/efaq-w32.texi` 2361
    （指向 gnu.org 手册集合的链接标签）、`etc/DISTRIB` 30（FSF 的印刷本）；同族的**外部专名与路径**
    （`(emacs)`/`(elisp)` 的 Info 文件名、`info-emacs-manual` 一族命令名/函数名、目录名、`XEmacs` 字样）本半一个都没动：
    改的是"名字"，不是"指向"（第十半"链接指向参照的站点、URL 不动"同一条）。
    **仍未清、点名不冒充**：这两本手册**叙述**里对这个程序的称呼（`doc/` 里仍有 **399 行**带那个拼写的叙述，
    以及 10,980 行里其余的裸名用法）、其余 19 本手册自己的标题（`@settitle` 那一族里**另外 24 行**）、
    `doc/translations/` 的译本里同一批字符串的非英文写法。
    **未声称**：本半没有跑过任何东西——改的是文本，证据只有逐字节门（963 行每行都被读回）与声明门；
    这些手册没有被 `makeinfo` 重建过（交叉引用的第五栏与新标题一致这件事由逐字节门与逐条复核保证）。
    同一批读数记在 `docs/status.md` 4.008。
    这条规则现在有第三十一半（2026-10-08，status 4.020）：**"这棵树自己的手册叫什么"这一面在其余手册上做完，并且第一次读到"换行引用"的另一半**。
    第三十半读的是本仓库自己的**两本**手册（`doc/emacs` 与 `doc/lispref`）；`doc/` 里其余各本手册的书名留在原地，
    每本手册 `@direntry` 块里**条目标签**那一格（第三十半读了同一个块、只改了描述句里的手册名）也没有读者。本半按同一条判据把三处一次改名：
    ⑴ 书名与副题（`@settitle`/`@title`/`@subtitle`/`@shorttitlepage`/`@titlefont`，含 `@center @titlefont{…}`）——
    小型库/模式手册一族、`doc/emacs/emacs-xtra.texi`、`doc/lispref/elisp.texi` 131 的副题、法文译本 `doc/translations/fr/misc/ses-fr.texi`；
    ⑵ 每本手册 `@direntry` 块的**条目标签**与被 `makeinfo` 装进 Info 目录的**描述句**（`doc/emacs/emacs.texi` 44 的标签仍写着参照的名字，
    而同一个文件的 `@settitle` 已是本仓库的名字——同一文件里的**分裂身份**）；⑶ 点到自己手册书名的引用行：`@xref`/`@pxref`/`@ref` 的
    **手册标题栏**、`@cite{…}` 的书名、`@uref` 的标签、`lisp/gnus/mml.el` 的菜单标签与 `:help`。字形仍是第十半的**机械替换**
    （只换产品名那一个 token，`GNU ` 前缀随产品名一起去掉），另有两处**随行而变的第二字形**、逐条写进行的理由里：
    `an Emacs` → `a femacs`（冠词随首音，3 行）与法文 `d'Emacs` → `de femacs`（省音，2 行）。
    **本半第一次读到"换行引用"的另一半**：第三十半的行形读法只覆盖了**手册标题栏的 token 落在续行**的那些换行引用（它把续行改名并登记），
    token 落在**引用首行**的另一半（**136 行**）过得了逐字节门、却没有任何读者：本半用"窗口里看完整引用、只改本行那一段"的读法收掉，
    并且**只改手册标题栏**——节点名（参照产品名 + ` Server` 这类节点标题）与 Info 文件名（`emacs`/`elisp`）一处都没动
    （第十半"改的是名字，不是指向"同一条）。这条读法有一个**行形读者看不见的坑**，照录：把窗口里"引用在**后一行**起头"的匹配
    当成当前行，会把整条引用粘到当前行尾；判据是逐行读回（见 `docs/status.md` 4.020 的结果）。
    规模与读数：**246 行 / 85 个文件**（`ref` 148、`title` 61、`direntry` 35、`label` 2）逐行记进
    `docs/baseline/verbatim-substitutions.tsv`（**8,173 → 8,419** 行）；`python3 tools/check-verbatim.py` →
    **`6 trees, 4604 files, 8,419 sanctioned line(s), 0 failures`**（逐树 `doc` 953 → **1,199**、`lisp` 6,472 → **6,474**，
    `etc`/`leim`/`admin` 与六棵树的文件数逐个不动）；`python3 tools/check-verbatim-declarations.py` →
    **`710 candidate(s), 6 enumerated keep(s), 710 classified, 0 failure(s)`**（候选数与上一半相同）；十门其余不变
    （declarations **18 / 0 / 18 / 0**、own-voice **0**、citations **676 / 0 / 0 / 2,416 / 36 / 0**、mirror-dupes **16/16/0**、
    port-lines **0/0/0/0**、ledger-ticks **238/0**、carried **11/11/4/0**、dead-file-coverage **145/0**）。
    **按判据不动、逐条点名（10 处）**：token 是**语言名**、不是本程序的名字那一族——`doc/lispintro/emacs-lisp-intro.texi` 4/169
    与同一个文件的 copying 页 102/891、`doc/misc/ert.texi` 4/35 与它的 Info 目录描述 15、`doc/misc/cl.texi` 28、
    `doc/misc/eieio.texi` 33、`doc/misc/eshell.texi` 31、`doc/lispintro/emacs-lisp-intro.texi` 89 的标签；判据与第十四半
    "语言名留着、改的是手册名"一致（`doc/misc/calc.texi` 32,132 那一行是先例）。同族的"指向"与署名三处也不动：
    `doc/misc/efaq-w32.texi` 2361（指向 gnu.org 手册集合的链接标签）、`doc/lispref/debugging.texi` 47（指向 ERT 手册的标题栏，
    随那本手册自己的书名不动）、`lisp/help.el` 731（从 FSF 买印刷本那一句）。
    **未声称**：本半没有跑过任何东西——改的是文本，读数只有逐字节门（246 行每行读回，并逐行核对字形：**零处非许可 hunk**，
    即除 token 与那两处第二字形外整行逐字相同）与声明门；这些手册没有被 `makeinfo` 重建过，
    "Info 目录里那一格现在印什么"只是文本读数、不是 `info` 的输出。

    这条规则现在有第三十二半（2026-10-08，status 4.034）：**照抄树里"文件自己开头那一行"这一面第一次被读**。
    前三十一半读的是**句**（成员句、许可句、手册书名、显示标签、注册名），而每个照抄文件**自己那一行标题/摘要**
    （`;;; array.el --- array editing commands for <产品名>`、`% Reference Card for <产品名>`、`#+title: …`）
    既不是成员句、也不是逐字节门看得见的差——它就待在 "This file is part of femacs." 上面三行，**写明这个文件是给谁用的**。
    判据仍是第十四半那一句"这句话说的是谁"：这一行的主语是**这个文件自己**，而文件是本仓库自己的副本 ⇒ 该行里的产品名指的就是本程序
    ⇒ 按第十半的机械字形改名（只换产品名那一个 token，`GNU ` 前缀随产品名一起去掉，同行其余字符一个都不动：
    `% Title:  <产品名> Survival Card` 的双空格、`;;; x.el --- …  -*- lexical-binding: t; -*-` 的两空格与文件变量照旧）。
    **第二形状**：首行是必须居首的指令（`%&tex`/`%&mex`）时标题在第二行——`survival.tex`、`es-survival.tex`、`fr-survival.tex`、
    `pl-refcard.tex` 四张 refcard 按这条读；同理四个 `.svg`（首行 `<?xml …?>`、第二行那行注释写的是自己的 logo）。
    规模与读数：**44 行 / 44 个文件**（`lisp` 21、`etc` 21 = 15 张 refcard + `DEBUG`/`PROBLEMS` 2 + `.svg` 4、`doc` 1、`admin` 1）
    逐行记进 `docs/baseline/verbatim-substitutions.tsv`（**8,419 → 8,463** 行）；`python3 tools/check-verbatim.py` →
    **`6 trees, 4604 files, 8,463 sanctioned line(s), 0 failures`**（逐树 `lisp` 6,474 → **6,495**、`etc` 473 → **494**、
    `doc` 1,197 → **1,198**、`admin` 260 → **261**；`test`/`leim` 与六棵树的文件数逐个不动）。
    **同一半给这一类装上读者**：`tools/check-verbatim-declarations.py` 的候选从"成员句"一个形状扩成**两个**，第二条是
    "文件自己的开头标题行 + **拼写完整**的产品名"（裸名在这一条里**故意不算**：标题里的裸名是**语言名**，第十四/三十一半的边界）。
    `--selftest` **15 → 21 例 / 0 失败**（新增六例：标题行无行无表 ⇒ 1 条失败、标题行有登记行 ⇒ 0、标题行在 `test/` ⇒ `forbidden-tree`、
    `%&tex` 之后的第二行、token 是语言名的那一行有登记行 ⇒ 0、已经是本仓库名字的标题 ⇒ 0 候选）。读数
    **`734 candidate(s) of which 0 wrapped across two lines, 7 enumerated keep(s), 734 classified, 0 failure(s)`**
    （上一步 710/6；逐类 `forbidden-tree` 653 → **659**、`history` 38 → **55**、`negation` 13、`enumerated` 6 → **7**）。
    **判别力实测**（真树变异、复原后回绿）：把 `femacs/lisp/array.el` 第 1 行改回参照 ⇒ 逐字节门报
    `line 1 is '… <产品名> …', the table says '… femacs …'`、声明门报 `a substitution row says this line carries this
    repository's name, but the statement still names the product`，各 1 条失败、复原后两侧都回 `0 failures`。
    **按判据不动、逐条点名（17 + 1 + 6 处）**：⑴ **历史件（17）**——`etc/NEWS` 与 `NEWS.1-17`…`NEWS.31` 十六个文件的首行
    （`<产品名> NEWS -- history of user-visible changes.`）加 `etc/HISTORY`：这些文件**记录的是参照产品的发行史**，改标题就把那些发行
    写成本仓库自己的（`history` 类，与 `ChangeLog*` 同一条）；门为此把**版本区间**的文件名 `NEWS.1-17` 也算进该类——这是本半实测到的
    老判据缺口（原正则只认 `NEWS.18` 这种纯数字后缀，`NEWS.1-17` 会掉成"未归类"）。⑵ **语言名（1）**——
    `femacs/lisp/obsolete/cl-compat.el:1` 把产品名缀在**语言名**上（`… Lisp`），改它就是造一个不存在的语言名
    （第十四/三十一半的边界，`doc/lispintro/emacs-lisp-intro.texi` 4/169 是先例）：逐行登记进
    `docs/baseline/verbatim-declaration-keeps.tsv`（**6 → 7 行**，class `language-name`）。⑶ **`test/`（6）**——目标文本禁止改动的树
    （`INSTALL_BEGIN`、`biditest.el`、`etags.c`、`c-src/emacs/src/lisp.h`、`mod-test.c`、`emacs-module-tests.el`），保持参照的字节。
    **仍未清、点名不冒充**：⑴ 六个照抄树里**首行以外**的标题/题头行本半没有读（`etc/DISTRIB` 2、`etc/MACHINES` 6、`etc/NEXTSTEP` 4、
    `etc/AUTHORS` 2、`admin/notes/spelling` 3、`doc/misc/dired-x.texi` 3，以及 `doc/lispref/internals.texi` 6 那种**节点名**——
    第三十一半已判节点名不动）；⑵ 裸名（`Emacs`）仍按 3.692 的裁决留在表外；⑶ `etc/org.…defaults.gschema.xml` 里的 schema id 与
    `femacs/src/pgtkfns.rs` 的同一字符串是**外部注册名**（第十三半的类：改了就是改行为），一处不动。
    **未声称**：本半没有跑过任何东西——改的是文本，读数只有逐字节门（44 行每行读回）、两个声明门的读者与一次真树变异；
    这些 `.el`/`.tex`/`.svg` 一件也没被加载、渲染或 `makeinfo` 过；`.el` 的首行是 `lm-summary` 运行期读的字符串
    （`finder`/`checkdoc`/`package-vc` 用它），改名后**没有跑过**其中任何一个。
    **顺带一处记账更正**：第三十半的散文把逐树读数里的 `doc` 记成 `953 → 1,199`，而那一版提交的登记表里 `doc` 是 **1,197** 行
    （总数 `8,419` 是对的，逐树那一格高 2）：本半重算时才发现，照实录在这里。
    同一批读数记在 `docs/status.md` 4.034。



18. **镜像里的宏必须先定义后使用**：`macro_rules!` 只在文本顺序之后可见，而三个 spike
    探针把镜像 `include!` 进一个 `mod`（§0.1-9 要求镜像位置无关），镜像内的函数体
    因此不能调用排在它后面的宏——`call0` 按 lisp.h 3462-3471 的顺序挪到了 `calln!`
    之后（P-57）。

| P-86 | **"整身都在假条件下的文件"这一大类此前没有写法与验收约定**：`tools/check-port-lines.py` 量到 **96 个目标的端口小于参照的 10%**、合计 **246,438 C 行**（`android*.c`/`w32*.c`/`haiku*.c`/`pgtk*.c`/`msdos*.c`/`exec/*`/`lwlib/*`/`oldXMenu/*`/`nt/*` 与 `src/{image,sfnt,xfns,w32fns,xgselect,gfilenotify,emacgtkfixed,pgtkim}` 等）；它们的正文整身挂在冻结配置为假的 `#if` 下，P-73 的活行过滤给出**空清单** ⇒ `extract-abi --check` 对它们**空跑通过**，`docs/ports/*.md` 的勾选框也是空的，于是"3 行的头注释"看起来和"落完了"一样 | **规则，立即生效** | ① **照录，不许 stub**：按 C 的顺序逐行转写，**每条 `#if`/`#ifdef` 的两臂都带**，门写成参照自己那个宏的小写 feature（`#[cfg (feature = "have_android")]` 等）；3 行的头注释不是"落完"，是"还没开始"。首行仍是一条 `// femacs: port of <路径> @ <修订号>.`，不带参照的许可块（0.1-19）。② **台账不能用勾选框当覆盖证据**（清单是空的）：改记 **C 行号区间表**（`| C | block | arm |`，格式见 `docs/ports/androidmenu.md` 的 `## C line ranges`），另加 `## Evidence`（本轮读数）与 `## Deviations`（每条 `#if` 的判定依据与臂）；**这张表现在有机器门**：`python3 tools/check-dead-file-coverage.py`（`gate-fast` 已接）要求区间覆盖参照的每一行（块之间的空行除外）、升序不重叠、末行到参照最后一行。完成度读数仍是 `tools/check-port-lines.py` 的棘轮行（行数比）。③ **验收分两档，按本机有没有那个 SDK 定**：⑴ **有 SDK（Android：本机 `~/Library/Android/sdk/ndk/*` + 仓库已有 `spikes/android_inc_probe.rs` 形状）⇒ 必须做"翻臂编译"**：`rustc --cfg 'feature="<该文件的门>"'` 编它的 `build/femacs/units/<stem>/_root.rs`，报 exit 与错误分类，只允许**已登记**类别（别的未落单元、镜像已记名的系统/API 名）；这与 xterm 的 `x11_ext` 臂同一口径。⑵ **没有 SDK（Haiku/Windows/GTK/PGTK 的交叉头）⇒ 如实记 `text-only`**：本轮**无法翻臂编译**，证据只有文本（行数棘轮 + `check-cond-eval` 的 `#if` 判定 + 逐条 C 行号引用），并在台账里**点名缺哪个工具链**——这正是 `docs/goal.md` §9 第 2 条（需要本机不具备的外部资源）的落法，不是"用空跑的门冒充通过"。④ **提交与顺序**：先小后大（先把百行级的落成模板，再上千行）；每件一个签名提交；收尾跑 `tools/check-port-lines.py --write-pending`，棘轮会按"行数越过阈值"逐条退出。 **P-86 的第一件（`src/androidmenu.c`）又量出六条，照实录下**： ⑥ **翻臂编译必须把“文件门 + 每个子门”都设上**：只给 `feature="have_android"` 选中的是 C 829-837 的 3 行 stub 臂，真臂还要 `not_android_stubify`（判据：单 cfg 的跑法会“报完成”而新代码一行没编）。 ⑦ **include 图先坏时，翻臂编译在名字解析阶段中止、根本不检查本单元 ⇒ “本单元 0 错”是空读数**：判据是**变异实测**——往单元文本注入一个类型错误，编译输出**逐字不变**（仍 10 错 + abort）。⇒ 收口要**按文件统计错误**（`_root.rs` 自身 vs 镜像副本），图坏时改跑“臂触达”预检（单元文本 + 能单独编的镜像 + 桩），并**声明那不是同一个读数**。 ⑧ **整身挂门的文件，门要下到 include 图上**：C 里那六个 `#include` 必须带 `have_android`（判据：冻结配置编出 **336 字节空对象**；把 include 放到门外，冻结编译立刻吃 `E0428 Emacs_Cursor` 与 `cannot find macro`）。 ⑨ **在项与项之间插新项后必须重跑“冻结”这一档**：判据——把宏插在 C 70 那行的 `#[cfg]` 之下，属性被吃到宏上（`android_init_emacs_context_menu` 变无门），**冻结预检**报 `cannot find macro ANDROID_DELETE_LOCAL_REF`，而两门都开的预检**照样 0 error、看不见它**。 ⑩ **端口一有 `fn`，`extract-abi --check` 就不再空跑**：3 行注释版打 `OK`（空清单），这个端口打出 2 条 `- ` 行并 `FAIL` ⇒ P-86 的“台账当覆盖证据”必须配“以及机器检查需要的基线行”（本件是 `macro-body-notes.tsv` 的 `goto` 与 `line-drift-tolerance.tsv` 的一行），而**这两本不在移植者的可写白名单里**。 ⑪ **被 `excluded-macros.txt` 排除的宏，前提被推翻后要由消费单元按 C 的名字定义**：那条排除的理由是“`JNINativeInterface` 没有镜像”，而 `include/jni.rs`（3.154，230 项）已有 ⇒ 单元里按 C 的名字定义 `macro_rules! ANDROID_DELETE_LOCAL_REF` 后 body 检查那行消失。 ⑫ **include 边一律两行写**（`#[cfg (feature = "…")]` 一行、`include!(…)` 一行）：`xtask` 的 `gate_states` 只认“属性在前几行、项在下一行”，**同一行**写成 `#[cfg (…)] include!(…)` 的边被判为**无门**。判据：`src/sfntfont-android.c` 的 lane 按同一行写法时，冻结配置编出 **43 条镜像错误**（本该是 336 字节的空对象，P-86 ⑧）；改成两行后冻结档回到空对象，翻臂读数也随之可用。**修 `gate_states` 仍是欠账**（写进 `tools/` 之后本条可撤），在此之前这条写法是硬要求。 **⚠ 前置阻塞（不是本件的缺口，但决定整类能不能验）**：`have_android` 的翻臂编译现在**10 个错误全在镜像里**（`termhooks.rs` 的 `coding_system`×4、`frame.rs` 的 `window`/`WINDOWP`/`XWINDOW`×3、`keyboard.rs` 的 `buffer`×2、`dispextern.rs` 与 `androidgui.rs` 的 `Emacs_Cursor` 重复 `E0428`）⇒ **今天没有任何 `have_android` TU 能编过**，因此**所有 android 的翻臂读数都是空的**；先把那 7 条声明与 cfg 决定补进镜像，这一类才有第一份真证据。 |
| P-87 | **镜像改动的验证集是“所有 include 它的单元”，而这件事此前没有机器在读**：`tools/reconcile-link-ratchets.sh` 只重建**在飞**镜像的 includer，已提交的镜像改动**不会被再验一次**；本会话三个事故都是这条漏的——3.344（`SET_FRAME_GARBAGED` 的强转打死 fresh 的 xdisp）、3.363（镜像写 `*mut window`，而 `bidi`/`coding`/`lread`/`regex-emacs`/`syntax`/`xml` 不 include `window.rs`）、3.375（镜像写 `window_part`，而 `editfns`/`insdel`/`textprop` 不 include `dispextern.rs`，坏了约十个提交才发现） | **规则，立即生效** | ① 改任何 `femacs/{src,lib}/include/*.rs` 之后，**除了**该镜像自己的 `--check` 与 `check-mirror-dupes`，必须跑 `tools/check-all-units.sh`：它按 `docs/baseline/frozen-build-sources.txt` 逐个 `build-tu`，**期望读数是 `83 unit(s), 0 failure(s)`**。② 在 **HEAD 的 worktree** 里跑得到 HEAD 的读数；在主工作树里跑得到的读数**含 lane 在飞改动**，是两个不同的数（3.339 的口径）。③ 一个镜像**不得命名它的 includer 可能没有的类型**：C 的原型只需要 tag（不完整枚举/结构在 C 里合法），端口的对策是**写底层类型**（`window_part` 是 `c_int` 的别名 ⇒ 镜像写 `c_int`，调用点一个都不用改）或按 P-72 rule 3 写 `*mut c_void`；**改完必须用上面那条扫描证明**，不许只编“我碰过的单元”。 ④ **门的读数是它的退出码，不是它打的最后一行**：三处事故的门都会打日志尾巴，而验证常写成 `a && b && c` 链（链子只看整体退出码，日志没人读）——3.541 把 `include/w32common.rs` 的两条记录裸着放进单 crate 探针，`layout-check` 从那时起就 `layout probe failed to compile`（12–14 条 `E0425`），而 3.547 的提交信息照旧写了 "layout-check passed"，**六轮没被发现**（3.548 在 `dca3e1c7` 与 `77c74999` 两个 worktree 里复读才量出来）。规则：这三道门（`layout-check`、`check-all-units.sh`、`gate-fast`）**每轮读它的退出码**，读数是数字（`0 failure(s)`、`passed`）而不是"我跑过了"。 |
| P-88 | **外部库头（参照树里没有对应文件的第三方头）此前没有声明形状**：P-86 要求把整身挂在假条件下的文件照录，而 `<zlib.h>`/`<lcms2.h>` 这类头在参照树里**不存在**（`ls femacs/src/include | grep -iE 'zlib|lcms|jansson'` 为空、全树没有 `/// C: #include <` 段），于是每个这样的文件都会各写各的，或者干脆按 P-08 记成空单元 | **规则，立即生效** | ① **声明写在单元自己里**：在 C 的 `#include` 那一行处起一个段，首行是 `// C <行号>: \`#include <X.h>\``，**不新建 `femacs/src/include/` 下的文件**——那一层是**参照头**的镜像，`check-target-names.py` 按它认参照目标，放一个没有对应参照头的文件进去会造出一个无处安放的 finding。② **每条声明带 `/// C: <头文件里那一行的原文>`**（P-55 的形状），类型写 `#[repr(C)]`，**布局必须用 clang 探针量**（`clang -o /tmp/p /tmp/p.c && /tmp/p` 打印 sizeof/offsetof，读数进台账 `## Evidence`，形状见 `docs/ports/xterm.md` §19 的 xcb 那一段）——不许凭记忆写。③ `#define` 常量照抄数值/表达式，函数声明写 `unsafe extern "C"`（P-26 的镜像规则同样适用）。④ 台账 `## Deviations` 记一行："这一段是外部头的替身，参照树里没有对应文件"。⑤ 若该外部头某天进了参照树（例如被 gnulib 化），把它**搬进 `include/<name>.rs` 并在同一个提交里删掉段内副本**（P-26/P-70 的搬运规则）。 |
| P-89 | **`xtask` 的 `materialize` 对嵌套 `include!` 按**文件**去重，且**不看**到达它的那条边上的 `#[cfg]`**：任何"共享镜像里带门的 include 边"都会把目标文件的**唯一一次包含**挪进死臂。实测两起（3.408 的镜像轮）：`sysdep.rs` 带门的 `wchar.rs` 边与 `layout-check` 的 `lib/msdos.rs` 边——两次都是"镜像自己看不出错、别的单元突然缺失名字" | **已知陷阱，立即生效** | ① 给共享镜像加**带门**的 `include!` 边之前，先确认目标文件还有别的**无门**路径能到达；② 一旦被咬，就在镜像里绕开（把该边写成无门 + 目标文件自带内容门，或把这条 include 下放到消费单元），并在该镜像的 `include-*.md` 里记下事故与绕法；③ **根因已在 3.429 修掉**：`xtask` 的 `materialize` 现在把"带门的 include 边"与"无门路径"分开算——**凡是存在无门路径到达的文件，只在无门那条上展开**（`ungated_reachable` 预扫 + 沿路径传递的 `path_gated` 标志），带门边（无论它自己带门还是嵌套在带门文件的副本里）一律降级成 no-op 注释。判据：`sfnt.c` 的 `have_android` 臂由 `5 条 E0425` 变成 **exit 0 / 4,784 字节对象**，同时 `check-all-units.sh` 两腿仍是 `83/0 + 83/0`。**仍保留 ① ② 两条纪律**：给共享镜像加带门的 include 边前先确认目标还有无门路径；被咬时先按 ② 绕开（工具的这条改动只是让"绕开"不再必需，不是让坏边变好）。 |
| P-90 | **第三方 SDK 表面的声明该放哪：P-88 ① 说"写在单元自己里"，而 X11 有反例**——`femacs/src/include/xlib.rs`（`<X11/Xlib.h>`+`<X11/Xresource.h>`）、`femacs/src/include/jni.rs`（NDK `<jni.h>`）、`femacs/lib/include/haiku_support.rs`（Haiku SDK）都是**专用 SDK 镜像**，首行写 `// femacs: mirror of the <SDK>'s <header> (system header of another target's toolchain, PORTING.md P-26/P-49).`，`tools/check-target-names.py` 认这一行；2026-10-05 四个平台 lane 同时开工（`pgtkterm`/`gtkutil`/`w32proc`/`sfntfont`），同一个表面**第二次**被声明时就必须裁定，否则同一个 TU 里两份同名声明是 `E0428` | **规则，立即生效** | ① **判据是"这个表面有几个消费者"**：只有一个（例如某 `.c` 独有的 `<zlib.h>`）⇒ 按 P-88 ① 在单元里就地声明；**两个及以上**（GTK/GDK/GLib/cairo 表面：`pgtkterm.h`/`gtkutil.h`/`pgtkfns.c`/`pgtkm*`/`emacgtkfixed.c` …）⇒ **一个专用镜像** `femacs/src/include/<sdk>.rs`，首行按上面那行写（`<SDK>` 写工具链的名字），台账 `docs/ports/include-<sdk>.md`（`docs/ports/include-xlib.md` 的形状）；② **一个表面只能有一个镜像**：不许两个镜像各声明一份同名类型（同一 TU 会 `E0428`）；该表面被哪个参照头引用，就在那个镜像的 C `#include` 位置 `include!` 它；③ 单元的段内副本按 **P-88 ⑤ 的搬运规则**搬进 SDK 镜像并**在同一个提交里删掉**（3.408→3.409 的 move-back 形状）；④ SDK 不在本机时，布局按 P-88 ② 能测的 clang 实测、测不了的**点名 `UNVERIFIED`**（不许凭记忆写数字），并在台账写明缺哪个工具链。**本轮不追溯**：已落地的段内副本（`src/pgtkterm.rs` 的 cairo/GLib/GDK/GTK 段）随其下一次切片或下一次镜像轮搬运。 |
    这条规则现在有第七半（2026-10-05，status 3.449）：**"几个消费者"这个判据只对镜像成立，单元不受它约束**。
    镜像是一段被 `include!` 进编译单元的文本，两个镜像各声明一次同名类型就是同一个 TU 里的 `E0428`（这正是第五半要解决的）；
    而**单元是各自独立的 crate**，同一个名字在 `callproc.rs`、`process.rs`、`exec/*.rs` 里各声明一次**没有**这个风险，
    重复只是重复（P-87 ③ 也只约束"镜像不得命名其 includer 可能没有的类型"）。所以：**POSIX/系统宏这类跨单元重复的声明
    （`<sys/wait.h>` 的 `W*` 家族、`exit`/`fork` 这类"镜像还没有"的条目）继续按 P-88 ① 在各单元就地声明**，不因为它们
    有多个消费者就强造一个镜像；反过来，**只要有一个镜像是它们的家，其余镜像就不能再各写一份**（第五半的原意）。
    移动它们的时机是"镜像轮"（3.409 的 move-back 形状），不是"第二个消费者出现"这个信号。
| P-91 | **Rust 没有 `static` 函数原型：C 的"先用后定义"在同一文件里无法照抄**——`sfnt.c` C 6316 `sfnt_init_graphics_state` 调用 C 6308-6310 前向声明的 `sfnt_validate_gs`，而它的**定义在 C 10538**（同一个文件里尚未移植的段）；0.1-12 要求删掉 C 的静态前向声明，而 Rust 里没有"声明"只有"项"，于是只要定义还不在文件里，调用点的**翻臂读数**就必然报"找不到函数"（3.475 实测，`sfnt` 切片 6 因此停在 C 6306） | **规则，立即生效** | ① **移植顺序按依赖闭包，不按 C 的行序**：一个切片要编得过，它调用的每个 `static` 的**定义**必须已经落在同一个 `.rs` 里；缺哪个就先落哪个（它的 C 位置可以远在别处）。② **文本位置仍按 C 的行序**：新切片的内容插在**它自己的 C 位置**——Rust 的项与顺序无关，所以"先落 C 10538 再落 C 6316"不会破坏"同顺序"这条约束，两个项在文件里的先后始终等于 C 里的先后。③ **台账的 `## C line ranges` 保持升序**，`not ported` 行与已落行可以交替（那道门只查覆盖与升序，不要求已落行连续）。④ **翻臂读数新增一类允许项**：「同一个文件里、C 行号已知但尚未移植的 `static` 定义」——每个站点在台账里点名它要等的那条 C 行；这一类随文件填满而递减，**不许**当作"已通过"。 |
| P-92 | **镜像里的 `macro_rules!` 按*文本顺序*可见，C 的宏不是**——把 C 的宏从"镜像里的 `extern` 变量（停止gap）"改回真宏时，凡在该镜像*之前* materialize 的文本（别的镜像的 `fn` 体、或单元自己的早段）展开它都会 `cannot find macro`；C 在**调用点**展开，只要头文件在 TU 里出现过就行。这个类在 `current_buffer`/`specpdl` 迁移里一次量出 **57 个单元失败**（status 3.605，已全部修完） | **规则，立即生效** | ① **宏的家要么在 C 的头位置、要么在能用它的最早位置**：`src/` 镜像不许 include 另一个镜像（0.1-9），所以"谁能看到这个宏"由**单元的 include 顺序**决定——镜像 A 的 `fn`/`macro_rules!` 体用镜像 B 的宏时，A 必须在 B *之后* materialize；② **P-26 的"从属主镜像取名字"照旧**：给 A 加 `include!("B.rs")` 要放在 C 的 `#include` 位置（C 没有那条 include 时，放在 A 第一个用到它的位置之前，并在注释里点名它是 transitively 拿到的）；③ **停止gap可判**：镜像里为"C 的宏"临时声明的 `extern` 存储（`include/lisp.rs` 的 specpdl 三兄弟、`include/buffer.rs` 的 `current_buffer`）在链接期是**未解析符号**，`check-link.py` 的 `unknown` 桶逐条点名——迁移它们按"符号消失"验收，不按"编译过了"验收；④ **转换站点要跳过非代码与绑定位置**：注释/字符串、`$` 前缀的宏形参、`let`/`for`/`fn`/闭包绑定名；**赋值位置**（C 的 `x = v;`）要另外拼（本类用 `set_current_buffer!`），因为 Rust 里"展开成 cast 的宏"既不能当赋值目标也不能取地址；⑤ **一次迁移的验证集 = P-87 两腿 + `check-link.py --executable`**：`83/0` 两腿 + `layout-check passed` + 可执行档未解析名 18 → 14（4 个符号一次消失）。 |
| P-93 | **`memcpy` 家族在端口里写成 `copy_nonoverlapping` 时，第三个参数是 `T` 的*个数*，而 C 写的是*字节数***：3.618（`maybe_resize_hash_table` 的向量迁移）与 3.626（`Fcopy_sequence` 的 bool 向量）各一例，都是"带类型的指针 + 字节数"⇒ 实拷 8×（`Lisp_Object`/`bits_word` 都 8 字节），表现为堆被写穿或用 Rust 的前置条件 panic 炸掉（`copy_nonoverlapping requires ...`） | **规则，立即生效** | 两处都改成"**经 `u8` 指针 + 字节数**"（与 C 的 `memcpy` 等价；3.618 与 3.626 的写法一致）。**已核对**：`fns.rs` 的两处、`dispnew.rs` 1338（本就是 u8）、`buffer.rs` 820（`c_char` 单字节）、`emacs.rs`/`callproc.rs`/`doprnt.rs` 里指针为 `c_void`/`c_char` 的那些。**第三处真缺陷与全量核对（status 3.631，2026-10-06）**：`copy_hash_table`（`fns.rs` 5159-5180，C `fns.c` 4931-4945）的四处 `memcpy` 第三参是`kv_bytes`/`hash_bytes`/`next_bytes`/`index_bytes`（**字节数**），端口把字节数交给带类型的指针 ⇒ `key_and_value` 实拷 **8 倍**、`hash`（`hash_hash_t` = `unsigned int`）与 `next`/`index`（`hash_idx_t` = `int_least32_t`）各 **4 倍**——一次拷贝就把三块刚分配的向量写穿；同一函数的兄弟 `maybe_resize_hash_table` 在 3.618 修过，这四处当时漏了，现已一并改成 `u8` 指针。**其余“形状可疑”的站点逐处对 C 的第三参核完**：`dispnew.rs` 2219/2229/2239、2267/2275/2287（C 1974/1982/1990、2014/2020/2030）、3202/3327（C 2909/3025）、3932/4165（C 3579/3819）、`term.rs` 3651/3679（C 3456/3481）、`keyboard.rs` 2199（C 1958）、`callproc.rs` 1247（C 1034）、`fns.rs` 3760 —— 都已改成“经 `u8` 指针 + 字节数”；`fns.rs` 590/1044/1247/1304/1569/4835 与 `callint.rs:336` **本来就是元素数**（C 写的是 `size * sizeof *dst`，或指针是 `c_void`/`c_char`）。**核对到此没有剩余未核项。** |
| P-94 | **C 的八进制字面量（前导 `0`）抄进 Rust 会变成十进制**：`040` 在 C 是 32、在 Rust 是 40；3.633 量到整张 168 条 opcode 表里 **160 条的值是错的**（`Bcall` = 44 而非 32、`Bconstant` = 300 而非 192、`Bdiscard` = 210 而非 136）⇒ 每次调用都落到为别的 opcode 准备的分支上，字节码里紧随其后的 `Bdiscard` 被当操作数吃掉、每调用一次栈就长一格，直到某次入栈盖掉帧头的 `saved_fp`；另有 `lread.rs`/`tparam.rs`/`xdisp.rs` 的 11 处同类（`037`=37 而 C 是 31、`& 0177`=177 而 C 是 127、`!= 015`=15 而 C 是 CR 13） | **规则，立即生效** | C 那边有前导 `0` 的字面量，Rust 侧一律写 **`0o` + 同样的数字**（`0o40`），**值必须与 C 相等**；机器门 `python3 tools/check-octal-literals.py`（623 个 `.rs`、0 站点，`--selftest` 10 例，注释/字符串/`c"…"`/raw 串都不算）。判据是**值**而不是拼写——`extract-abi` 的常量面此前只比名字与文本，这条门补上值这一面 |
| P-95 | **C 的真 `do { … } while (COND);` 循环转写成 Rust 的 `loop` 时，`break` 的条件必须是 COND 的*取反***：3.635 在 `syntax.rs` 的快扫里量到一例写成了 `if COND { break; }` ⇒ 语法属性一关就立刻跳出快扫，紧随其后*无守卫*的 `update_syntax_table_forward (pos, …)` 去问一个缓存已是 NULL 的位置，撞上 `Error in syntax_table logic for-to-the-end intervals`（夹具加载链停在第 100 个文件） | **规则，立即生效** | `do { B } while (C);` → `loop { B; if !C { break; } }`（`while (0)` 那个*宏惯用法*照旧走 449 行“块表达式”那条，不受本规则影响）；写完把 C 的整句留在注释里，便于逐字复核。**机器门**：`python3 tools/check-do-while.py`（3.636；152 个单元、221 条 C 的 `do/while` 条件、73 条按文本配上端口的 `if … { break; }`、**0 failure**；`--selftest` 5 例）。它按**文本**判——机械转写会保留 C 的 token，所以 `break` 的条件与 C 逐 token 相同 ⇒ 红，写成 `!(C)` ⇒ 绿；*改写*过的条件（如 `*++p != '\0'` 写成 `p.add (1); if *p == 0`）配不上 ⇒ 计为“未检查”并在覆盖行报出（可靠但非完备）。门按单元配对，合法的**内层** `break` 可能与同单元另一个循环的条件同文本 ⇒ 这类逐行进 `docs/baseline/do-while-allowlist.tsv` 复核（现 2 行，写明各自对应的 C 内层语句与该循环真正的取反 trampoline 位置；行不在清单、清单行失效都红）。同盘点（3.636）：82 处 `} while (` 里 73 处是 `while (0)` 宏惯用法、9 处真循环逐个核过——8 处本就正确，1 处是 3.635 修的；门有判别力的证据是拿**修复前**的 `syntax.rs` 喂它 ⇒ 1 candidate，正对 `syntax.c:2187` |
| P-96 | **C 靠无符号回绕实现的算术，在 Rust 里必须显式 `wrapping_*`**：`size_t`/`usize` 的 `-`/`--` 在 C 里静默回绕，Rust（本仓库 `overflow-checks` 开着）直接 panic。3.637 在 dump 路径第一处就撞到：`lib/str-two-way.h` 的镜像里有 **10 个**这种站点——`p = j - max_suffix` / `p = j - max_suffix_rev`（`max_suffix*` 种子是 `SIZE_MAX`，第一次迭代就是要回绕）、四处 `i = suffix - 1;`、四处 `{ i--; }`（`i == 0` 必须回绕成 `SIZE_MAX`，紧邻循环正是 `while i != SIZE_MAX`）——八次 `memmem` 调用全部 panic | **规则，立即生效** | 凡 C 那边**按设计**依赖回绕的点（哨兵 `SIZE_MAX`、`max_*` 种子、`j - max_*`）写成 `x.wrapping_sub (y)`/`wrapping_add`，并把 C 的原句留在注释里；**算法上有界的减法保持受检写法**（越界即 panic，是“逻辑写错”的有用信号）。这一类**不需要静态点名**：回绕点必 panic，加载阶梯本身就是检测器；`overflow-checks` 保持开启，**不许**用关掉检查来“通过”。**3.643 在装载路径上又量到四处**（C 5328/5358 的 `uintptr_t value += emacs_basis ()`、C 5535/5539 的 `pval = <offset> + <base>`）：后两处端口原来还把 `dump_base`/`emacs_basis ()` **窄化成 `dump_off`（int32）再相加**——那不只是回绕，是**截断地址**；四处都按 C 的 64 位无符号运算改写（`as uintptr_t` + `wrapping_add`，再 `as ptrdiff_t` 落回 C 的 `pval`） |
| P-97 | **`DEFVAR_INT`/`DEFVAR_BOOL`/`DEFVAR_LISP` 的载体对象必须与宏解引用它的方式同类型同对齐**：端口的宏把地址强转成 `*mut i64`（Int）/`*mut bool`（Bool）/`*mut Lisp_Object`（Lisp），所以**单元自己持有**的那个 `pub static mut` 一旦写成更小的类型（`c_int`，4 字节对齐），就会被按 8 字节解引用 ⇒ Rust 的 `misaligned pointer dereference` panic（C 那边 `DEFVAR_INT` 指向 `&globals.f_<name>`，天然 8 对齐）。3.638 在 dump 路径上量到 `process.rs` 的 `process_error_pause_time: c_int` 一例（`default_value` → `do_symval_forwarding` 的 Int 支 panic） | **规则，立即生效** | 单元自持的 `DEFVAR_*` 载体写成 C 的那个类型：Int → `EMACS_INT`（与 C 的 `intmax_t` 同宽同对齐）、Bool → `bool`、Lisp → `Lisp_Object`，并去掉为迁就旧类型而加的 `as` 转换。**机器门**：`python3 tools/check-defvar-carriers.py`（3.639）——把每个 `DEFVAR_INT!`/`DEFVAR_BOOL!`/`DEFVAR_LISP!` 的变量名先在本单元、再在 `femacs/src/include/*.rs` 里找声明，再按宏的解引用方式核对类型（Int → `EMACS_INT`/`intmax_t`/`i64`/`isize`/`ptrdiff_t`，Bool → `bool`，Lisp → `Lisp_Object`）。读数：**153 个单元文件 / 651 个站点 / 651 个都取到声明并核对 / 0 failure**，`--selftest` 7 例；判别力证据：拿**修复前**的 `process.rs`（`git show 1cd24844:`）喂它 ⇒ 1 failure，正对 `process_error_pause_time: c_int`。找不到声明（名字可能是宏或字段）计入覆盖行，不算失败 |
| P-98 | **C 的 `(enum E) n` 强转在 Rust 里不能写成 `mem::transmute` 到无字段枚举**：C 的枚举在这种位置只是"整数的拼法"——`ENUM_BF (dump_reloc_type) type : DUMP_RELOC_TYPE_BITS`（C 291-292）是**位域**，赋值只截断、不校验；`transmute::<c_int, E>` 只要 n 不是声明的判别值就构造**非法枚举值**，本仓库（debug + `overflow-checks`）直接 panic。3.640 的 dump 就停在这里：`dump_decode_dump_reloc`（C 3706-3707）的 `transmute::<c_int, dump_reloc_type>`，而这一侧按定义收的是 `RELOC_DUMP_TO_DUMP_LV + <Lisp_Type>`（C 1461）与 `RELOC_DUMP_TO_EMACS_LV + <Lisp_Type>`（C 1510）＝ {6,9,10,11,13} 与 {14,17,18,19,21}，枚举自己的成员只有 0-6 与 14 ⇒ 9/10/11/13/17/18/19/21 全是"非法判别值"。判据是 **C 的行为**（C 不校验，只把数字存进位域），所以端口按 C 的**存储**走：`dump_reloc_set_type` 的参数类型写成 `c_int`（C 的枚举名下流动的本来就是数字；3.642 记为 Deviation），写位域前按 C 的宽度取掩码 `& ((1 << DUMP_RELOC_TYPE_BITS) - 1)`，断言仍是 C 的 `eassert (reloc->type == type)`（C 303）；**不许**用 `transmute`/`as` 构造可能非法的枚举值。修后 dump **跑完**（13,025,304 字节的 `emacs.pdmp`、`Dump complete`）。同族站点（`emacs_reloc_type`/`dump_fixup_type`/`cold_op`/`Lisp_Type` 的 `transmute::<c_int, …>`）当时的取值都是声明的成员，但**没有机器门**钉这一点——**欠账已于 4.393 收口**：`tools/check-enum-transmutes.py` + `docs/baseline/enum-transmutes.tsv` 逐站点记录（覆盖、计数、目标枚举的判别值集合都在门里重新解析 ✓，`why` 列是人的一半 ✓），并接进 `gate-fast`。 |
| P-99 | **C 的 `dump_offsetof (type, member)` 里 member 是**路径**，端口只收一个标识符会让经 union 的站点塌成 union 自己的偏移**：`offsetof (struct Lisp_String, u.s.data)` 是 24、`u.s.intervals` 是 16、`Lisp_Symbol.u.s.val.blv` 是 16、`buffer.own_text.intervals` 是 608+120=728；宏只收一个标识符时这三处只有 `…, u` / `…, own_text`，于是 fixup 被写进 `size`（字符串数据）或别的字段——3.644 量到的形状正是一条 cold 字符串的 `data` 指针落在 `size`、`data` 留 NULL，装载时 `hash_char_array (NULL, len)` 直接崩。修法：宏取**路径**（`$($member:ident).+`），站点逐字写 C 的路径；机器门 `tools/check-dump-offsets.py`（18 个 C 宏站点 + 1 个 C 直写 `offsetof` 站点 = 19 个端口站点，逐对同类型同路径同序；4 例自测）。 |
| P-100 | **装载后的进程只跑 `static` 初始化式，不跑 `syms_of_*`**：C 的 `DEFVAR_LISP/BOOL/INT/KBOARD` 把转发对象写成**静态量**（`static struct Lisp_Fwd const o_fwd = {Lisp_Fwd_Obj, .u.objvar = &globals.f_##vname};`，lisp.h 3495-3531，`DEFVAR_KBOARD` 的 `offsetof` 同样是常量），指针是链接期常量、任何进程里都对；端口原先在**宏体里运行期赋值**（`O_FWD.u.objvar = …`），而 `syms_of_*` 只在 **dump 期**跑过 ⇒ 装载后这些对象的指针**全是 NULL**（3.644 的第二个停点：`do_symval_forwarding` 解引用 `*XOBJVAR` 时 x0 指到的结构第一个字是 `Lisp_Fwd_Obj`、第二个字是 0）。**规则的形状**：凡“装载后的进程也要看见”的静态量（转发对象、静态注册表、任何被 dump 引用的 BSS 对象），初始化必须写进**静态初始化式**，不许只在运行期赋值；机器门 `tools/check-fwd-static-init.py`（**6 个宏**：`include/lisp.rs` 的五个 + `buffer.rs` 的 `DEFVAR_PER_BUFFER`；静态式里指名成员且非空值（`buf` 还要 `offset`/`predicate` 都非零）、宏体内不许有 `*_FWD.u.<member> =`；5 例自测）。**第三例（3.646）**：`DEFVAR_PER_BUFFER`（buffer.c 5016-5031）把 `offsetof (struct buffer, vname##_)` 与 `FWDPRED_##predicate_` 放在静态式里；端口运行期赋值 ⇒ 装载后每个 per-buffer 变量的 fwd `offset` 都是 **0** ⇒ `store_symval_forwarding` 把值写进 **buffer 的第 0 个字（pseudovector 头）**，`set_minibuffer_mode` 的 `setq-local` 把 minibuffer buffer 的头写成变量值（`0xF3F0`），随后 `Flocal_variable_p` 的 `CHECK_BUFFER` 失败——硬件写断点抓到的那一次写，栈是 `store_symval_forwarding ← set_internal ← exec_byte_code ← … ← set_minibuffer_mode`。 |
| P-101 | **构建档位与解释器的栈预算此前没有裁定 —— 端口以 `-C opt-level=0` 编单元，帧比 C 的 `-O2` 大一个数量级，装载器自己的递归因此撞穿 8 MB 栈**：`eval_sub` 的帧 **2992 字节**（端口，反汇编实测）对参照的 **208**（`CFLAGS = -g3 -O2`），而 Lisp 装载器要跑的那段递归（`macroexpand--all-toplevel` 展开 `cus-start.el` 的 41 KB 巨形式，深度到 `max-lisp-eval-depth`）需要 ≈25 MB ⇒ SIGSEGV 处理器把它判成 C 栈溢出、`loadup.el` 就此停下（**112 个文件里只加载 108 个**），`macroexp--pending-eager-loads` 留在正在展开的那个文件上、`temporary-file-directory` 从未算出（`-batch <file>` 于是 exit 255，即 4.377 的症状）。**把 680 个单元整体改到 `-C opt-level=1/2` 现在不是选项**：`2` 时 `styled_format`（xdisp）里的 `memmove` 读未映射地址，`1` 时崩在第一条 `Loading` 之前——端口的 `unsafe` 打洞层（镜像的类型双关与 `&`/`&mut` 别名）**还不具备被优化**，这是点名缺陷，不是档位选择 | **规则，立即生效** | ① **单元仍按 `-C opt-level=0` 编，但把两枚检查显式写出来**（`xtask` 的 `UNIT_CODEGEN_FLAGS`：`-C overflow-checks=on -C debug-assertions=on`）——它们是 O0 的默认值，写出来是为了日后抬档位时不会静默丢掉 P-96 的回绕检查与检查版构建的断言；② **端口的程序按 `-Wl,-stack_size,0x4000000`（64 MB）链接**：Darwin 的主线程栈在 exec 时就定死，C 自己那段运行期 `setrlimit (RLIMIT_STACK, …)`（端口照录、未改）在 macOS 上对它无效（实测：`ulimit -s 65520` 先设好再生效，进程内设则无效），参照的 macOS 应用构建用的是同一手法；③ **优化安全缺陷修掉之后这条链接标志要撤掉**——本条目是那笔债的收据。 |
| P-102 | **配套脚本（构建/生成/辅助）的语言政策此前没有裁定**：参照树里同时有 autotools（`Makefile.in`/`configure.ac`）、awk、perl、shell、python 多种；把它们逐一照抄会把同样的复杂度带进端口，而其中一部分（autotools 脚手架）对端口根本无效——端口自己的构建是 `femacs/xtask`（Rust），生成物由 `tools/gen-*.py`（Python）驱动 | **规则，立即生效** | ① **能统一就统一成两种语言**：构建/链接/闸门一律 Rust（`femacs/xtask`），生成物、清单、比对、门一律 Python（`tools/`）；不新增第三种工具语言。② **确实必需的保留**：驱动器与"与参照配方的适配器"（例：`femacs/admin/charsets/*.awk` —— 它们是参照配方的**输入**，其定义直接决定生成物的字节），以及 `femacs/test/**` 下的测试夹具（`.pl`/`.js`/`.lua`/`.m4` 是模式测试的输入数据，不是构建脚本）。保留必须在 `docs/retired-scaffolding.md` 的分类表里写明理由。③ **过时的直接丢弃**：参照自己的 autotools 脚手架（`Makefile.in`/`configure.ac`）在端口里没有任何读者，按本条丢弃；`docs/port-map.md` 自己也写着 "`Makefile.in` … Cargo and xtask replace it"。④ **丢弃必须留痕**：逐条登记到 `docs/retired-scaffolding.md`（路径 / 原用途 / 取代者 / 日期 / 提交），并在该文件的分类表里给出"保留 / 丢弃"的判据；后续丢弃追加到同一张表，**不许静默删除**。⑤ 本条的第一次应用（2026-10-16）：丢弃 9 件 `Makefile.in`（`admin/{charsets,grammars,unidata}`、`doc/{emacs,lispintro,lispref,misc}`、`leim`、`lisp`），保留 `test/` 下的 4 件与其夹具。 |
| P-103 | **常驻规则此前没有单一落点，容易在上下文压缩或换执行者时漏读**：用户的长期约束（复刻口径、编译与资源纪律、工具语言政策、提交纪律、汇报格式、当前优先级）此前分散在 `docs/goal.md` §2/§3、`PORTING.md` 各 P 条目与 `docs/status.md` 的逐轮散文里，谁读哪一份、以哪一份为准没有裁定 | **规则，立即生效** | ① **仓库根 `AGENTS.md` 是常驻规则的唯一落点**（执行代理每次开工自动读它）：它只写**不可协商的约束 + 指向权威文档的指针**，不复述细节；② **先改权威文档**（`PORTING.md` 的 P 条目 / `docs/goal.md`），再在 `AGENTS.md` 加指针或一行约束，两处**同轮提交**；③ `AGENTS.md` 与权威文档冲突时**以权威文档为准**，并在同一轮修 `AGENTS.md`；④ `AGENTS.md` 的每次修改在 `docs/status.md` 记一轮读数（改了什么、为什么）。 |
| P-104 | **C 的自增语句被丢掉或挪位这一大类此前没有裁定**：`decode_eol` 的 DOS 分支（C 6759-6857）依次做 `del_range_2 (pos, pos_byte, pos + 1, pos_byte + 1, 0); pos++; pos_byte += incr;`，端口把 `pos` 写成不可变绑定、**丢掉 `pos++`**、并**把字节位置多推一次**（`pos_byte += 1; pos_byte += incr;`）——源码读来与 C 逐行同形，ABI/布局/符号/token 各静态门全绿，缺陷只在**行为**上可见 | **规则，立即生效** | ① **C 的每条自增都是语义**：Rust 的不可变绑定不是省略它的理由——该 `let mut` 就 `let mut`，**不许把增量挪到别的变量上补偿**；② **双计数器循环是高风险位点**（`pos`/`pos_byte`、`idx`/`byte_idx`、`p`/`n` 一类）：转写后逐条核对**每条自增的目标变量与次数**；③ **行为缺陷靠差分门拿读数**，不靠重读源码：本条的机器门是 `tools/check-coding-eol.py`（15 例：`insert-file-contents`/`decode-coding-region`/`decode-coding-string` 三条路径 × 声明式（`utf-8-dos`/`iso-latin-1-dos`）/检出式（`undecided`）/无限定（`utf-8`）三种编码，再加两个编码方向 ✓），已接进 `xtask gate-load` 的链接之后（`gate-fast` 不刷新 `build/port/src/temacs`，读数会落在旧二进制上 ✓）；④ **打印文本不可作判据**：`prin1` 写的是裸 CR，终端会**覆盖同一行**，本类读数一律用**字符码列表**（本条首次探测正是这样把两个不同答案读成一致 ✓）。**实测（2026-10-16）**：修前 `(decode-coding-string (string-make-unibyte "a\r\nb\r\n") 'utf-8-dos)` 端口 `(97 13 10 98 10)` 对参照 `(97 10 98 10)`（第一个 CR 未删 ✓），GB2312 的 CRLF 文件两个 CR 全留 ✓；下游后果是 `lisp/language/pinyin.el` 的生成物 **418 行**错（首条丢键、末两条缺失 ✓）；修后 15 例 0 失败 ✓、`pinyin.el` 对**参照程序自己的产物**逐字节加改名记账 = `name-only`（27043 vs 27046 字节 ✓）。 |
| P-105 | **`etc/DOC` 怎么生成此前没有裁定，而 C 的做法依赖"源码文件名"**：C 的 DOC 规则（`src/Makefile.in` 663-668）是 `make-docfile -d <srcdir> <obj 名…> > etc/DOC`，`make-docfile` **按对象名去读 `<srcdir>/<stem>.c`（Objective-C 的读 `.m`）**、**不是读 `.o`**——实测判据：把对象复制到别处、`-d` 不指向源码时会报 `<dir>/alloc.c: No such file or directory` ✓；端口树的源是 `.rs`/`.swift` ✗，`make-docfile` 一个都找不到 ✓。**另有一条硬读数**：端口的 `make-docfile` 移植本身是**忠实**的——**同输入**（参照的 125 个对象名 + `-d emacs/src`）下，端口与参照的 `make-docfile` 输出**逐字节一致**（1,002,892 字节 ✓） | **规则，立即生效** | ① **端口用自己的扫描器**生成 `etc/DOC`（`tools/gen-doc.py` ✓，P-102 的"生成物 = Python 工具"口径 ✓）：扫端口自己的 `.rs`（`DEFUN!` 的 `doc:` ✓）与 `.swift`（P-106 定案的 docstring 载体 ✓）源，**按 C 的格式逐字节产出**（`\037S<对象名>\n` 分节 + `\037F<函数名>\n<docstring>\n` 记录 ✓），对象名与**顺序**照参照 DOC 的分节表（从参照那份 129 节读出 ✓），这样 `etc/DOC` 仍是**可互换件**（DoD-4 ✓），差异只允许来自 0.1-19 的改名 ✓。② **端口 `lib-src/make-docfile` 的移植保留**（它是 1:1 复刻件、且给了上面那条"同输入同输出"的读数 ✓），只是**不参与**端口的 DOC 生成 ✓——这条写进 `PORTING.md` 以免后来者再试一遍 ✓。③ 输入清单的**权威**从**参照的 `etc/DOC`** 分节表 + `make -n` 的对象名并集里取（实测 `make -n` 的解析漏了 4 件：`androidselect.c`/`androidvfs.c`/`sfntfont-android.c`/`sfntfont.c` ✓，而参照 DOC 有 129 节 / `make -n` 组出 125 ✓）。 |
| P-106 | **非 C 单元（`.rs`/`.swift`）的 DEFUN docstring 用什么载体，此前没有裁定**：C 的 docstring 是 `DEFUN` 宏的一个**参数**（字符串字面量 ✓），`make-docfile` 扫源码时按它产出 `\037F<名>` 记录 ✓。端口这边：**Rust 单元已经照做**（`DEFUN!` 的 `doc: "…"` 字符串字面量 ✓，例 `src/alloc.rs` 的 `("make-string" …)` ✓），但 **Swift 单元的 registry 只把 `doc` 写成 `Int` 且当前一律 `0`** ✗（`femacs/platform/macos/nsfns.swift` 88 行 `var doc: Int` ✓、2201 行 `doc: 0` ✓），docstring 只以 `///` 注释存在（该文件 509 行 ✓）——**注释不是可扫的载体** ✗ | **规则，立即生效** | ① **`.rs` 与 `.swift` 一律把 DEFUN 的 docstring 作为 DEFUN 记录里的字符串字面量**（Rust 现状即标准 ✓；Swift 的 registry 增加一个 `docstring: StaticString`/等价字段并写入 C 的原文 ✓），这样 `tools/gen-doc.py`（P-105 ✓）用**一套**扫描器就能覆盖两种语言、且记录内容与 C 逐字一致（只差 0.1-19 的改名 ✓）。② `///` 注释保持为人的文档 ✓、**不作**生成物输入 ✓；`doc: Int` 字段保持与 C 的 `Lisp_Subr.doc` 同形（它指向 DOC 里的偏移 ✓），由生成器填 ✓。③ 判据：扫描器对任一 `.swift` 单元产出的 `\037F` 记录必须与该单元 `.m` 参考文献的 `DEFUN` docstring **逐字相同**（除改名 ✓）——这条在 `tools/gen-doc.py` 的 `--selftest` 里给一例 ✓。 |
| P-107 | **常驻规则文件怎么改、以及“环境会清产物”这条操作事实，此前都没有裁定**：`AGENTS.md`（P-103）是常驻规则的唯一落点，却没写“变更流程”；另一面，**本环境会清掉 `build/femacs/*.o` 与 `build/emacs-ref/lib/libgnu.a`**（多轮实测：每次 relink 前都要重编），而这条事实只活在对话里 ⇒ 上下文一压缩就丢、每轮重付一次约 10 分钟 | **规则，立即生效** | ① **规则变更只改 `AGENTS.md`**，并在本文记一条裁定（P-1xx ✓）、在 `docs/status.md` 记一轮读数（本轮 P-107 / 4.415 即由此而来 ✓）。② `AGENTS.md` 增补**常用调用**块：唯一入口是 `cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- <cmd>` ✓，命令面为 `gen-config / layout-asserts / layout-check / build-tu <c-file> / gate-fast / gate-probes / dead-arms / gate-load / link-program <c-file> / gate-full` ✓；**状态相消对照用无 dump 的 `build/emacs-ref/src/temacs`**（带 dump 的 `emacs` 会额外带 449 个计数器差 ✓，见 4.411）。③ **环境会清产物**：任何 relink 前都要先全量重编 680 单元（约 10 分钟 ✓）；`libgnu.a` 可 `make -C build/emacs-ref/lib libgnu.a` 单独 4 秒重建 ✓；**需要 relink 的探针（插桩、诊断打印）一律折进已经要重建的那一轮**，不为它单开一轮 ✓。 |
| P-108 | **`etc/DOC` 的验收口径（P-105 的“逐字节一致”）此前不可判定 —— 参照 DOC 的 129 节里**有一多半的记录属于冻结配置根本不编译的代码**：DOC 的输入清单是 `SOME_MACHINE_OBJECTS + doc_obj`（`build/emacs-ref/src/Makefile` 477-500 / 470），而 `SOME_MACHINE_OBJECTS` **无条件**列出全部平台对象（`cygw32.o`/`w32notify.o`/`androidselect.c`/`sfntfont.c` …，其中四个还按 `.c` 拼名）⇒ 这正是 4.412 那条“`make -n` 组出 125 而 DOC 有 129 节”的来路 ✓；而冻结配置是**纯终端构建**：`docs/baseline/config/darwin-arm64.macros.tsv` 里 `HAVE_WINDOW_SYSTEM`/`HAVE_NS`/`HAVE_X_WINDOWS`/`HAVE_NATIVE_COMP`/`HAVE_MODULES`/`HAVE_GNUTLS`/`HAVE_TREE_SITTER`/`HAVE_LIBXML2`/`GLYPH_DEBUG` … **全部 undef** ✓ | **规则，立即生效** | ① **活记录必须逐字节一致**：“活” = 该 C 定义所在的整条条件链在冻结配置下为真，判据是那 7,300 条宏表（与 `extract-abi.py` 的 P-08/P-66 同一张表 ✓）；门命令 `python3 tools/gen-doc.py`（默认即分类模式，`live-missing`/`condition-unknown`/`c-origin-unknown` 任一非 0 即 exit 1 ✓），`--strict` 保留“整文件逐字节一致”的更严读法。② **死区记录登记为偏离**（本轮 151 dead-region + 19 dead-file ✓）：端口对被冻结配置排除的 C 区域按 P-81/P-86 的“**记录而不转写**”处理，其 docstring 不在端口源里 ⇒ 这些记录**允许缺席**，但必须由 `--classify` 逐条列出（section / kind / name / C 位置 / 端口位置 / 判定 ✓）**当作偏离账**；**不许**从参照树读 docstring 来补（参照树 git 忽略、生成物不得依赖它 ✓）。③ **Swift 记录不算偏离**（96 条 ✓）：`ns*`/`macfont` 的载体由 P-106 落地，落地后必须变成逐字节一致 ✓。④ **同节同名的文本漂移算缺陷**（本轮 57 条 ✓）：工具给出记录字节数、第一处偏移与两侧上下文（例：`fringe.o` 的 `\\(` 该渲染成 `\(` ✓、`window.o` 的续行缩进被吃掉 ✓、`xdisp.o` 的 `(fn … WHERE-)` ✓）。⑤ **多出来的记录算缺陷**：同节里端口产生了参照没有的记录（首例 `xfaces.o` 的 `face-alternative-font-*-alist` —— C 7625-7628 只有 `= Qnil; staticpro`、**没有** `DEFVAR`，端口那两条宏是凭空发明，已删 ✓）。 |
| P-109 | **docstring 载体的“文本”到底是什么，P-106 ① 的“写入 C 的原文”不够精确**：`make-docfile` 对**注释与字符串字面量都**跑同一套转义处理（`read_c_string_or_comment`：`\n`→换行、`\t`→制表、`\`+换行→丢弃、其他 `\X`→`X` ✓），之后才按“空格/换行挂起、遇非空白才 flush”输出 ⇒ C 源里写 `\(not counting` 的记录，DOC 里是 `(not counting`（`fringe.c` 1773 就是这种写法 ✓） | **规则，立即生效** | ① **端口载体放的是 C 原文经过那层转义之后的文本**：C 的 `\(` 端口写 `(`、C 的 `\n` 端口写真正的换行（Rust 里即 `\n`）✓；Rust 的转义只用来表示**解码后的字节**（`\"`、`\t`、`\n`），不再多留一层反斜杠（这正是 57 条漂移的第一大类 ✓）。② **续行缩进照录**：C 注释里续行的前导空格会进 DOC（挂起的空格在遇到非空白时 flush ✓），端口载体必须带上它们（第二大类 ✓）。③ **usage 行按 C 的参数名出**（`(fn SYMBOL NEWVAL OP WHERE)`），但端口**不必**为它改签名：生成器在出 usage 行前剥掉 `r#` 前缀与**一个尾下划线**（`where_`/`proc_`/`str_`/`macro_` → `WHERE`/`PROC`/`STR`/`MACRO` ✓）。**判据（为什么只许剥这一个）**：参照 DOC 的 1,794 条 usage 行里**没有**任何以 `-` 结尾的参数 token ⇒ C 从不给 DEFUN 参数起尾下划线名 ✓；剥离是否对，由**逐字节比对当场判定**（本轮 9 条 usage 漂移正是这条映射修好的 ✓）——2026-10-16 收窄：原文要求端口改写 `r#where`，实测端口多处用 `proc_`/`str_`/`macro_` 这类拼法，改签名要连带改函数体、风险大于收益，故把映射放进生成器并用字节判据兜住 ✓。④ 工具侧与 C 同形的三条：`MANY`/`UNEVALLED` 不追加 usage（C 把两者都映射成 -1 并改由 docstring 的 `usage:` 关键字提供 ✓）、`&optional` 只在 `minargs == 0 && maxargs > 0` 时打印一次 ✓、`defalt` 打印成 `DEFAULT`、`_` 打印成 `-` ✓。 |
| P-110 | **Swift 载体的具体形状（P-106 ① 只说“加一个 docstring 字段”）此前没定，且参照里 DEFUN 的 docstring 有两种写法**：实测（4.418）⑴ `.swift` 的注册调用一律是 `make_aligned_subr (lname_X, min, max, fn)` 与 `defvar_lisp/bool (&O_FWD_X, "name")`，既没有 Lisp 名（在 `lname_*` 常量里 ✓）、也没有 C 的参数名（usage 行要用 ✓）、更没有 docstring ✓；⑵ 参照的 docstring **两种写法都有**：`doc: /* … */` 与**字符串字面量**（例 `nsselect.m` 812 的 `ns-sent-selection-hooks`，用 `\`+换行续行 ✓）；而且 **DEFUN 的 intspec 本身就是字符串**（`nsfns.m` 1706 的 `0, 1, ""` ✓）⇒ “名字后第一个 `/*` 或 `"`”这种取法会把 intspec 当成 docstring ✗（本轮就是这样漏掉 `ns-popup-color-panel` 的 ✓）| **规则，立即生效** | ① **Swift 载体的形状**：subr 为 `make_aligned_subr (…, docstring: "C 的原文", args: "C 的参数名 以空格分隔")`，变量为 `defvar_* (…, docstring: "C 的原文")`；两个 helper **收下并忽略**这两个形参（C 的宏对它的 docstring 实参也是这个待遇 ✓），`lname_*` 常量给出 Lisp 名 ✓。② **读参照的 docstring 必须按实参位次**：`DEFUN (name, fname, sname, minargs, maxargs, intspec, doc)` 取第 7 个实参、`DEFVAR_* (name, var, doc …)` 取第 3 个 ✓（实现是 `tools/gen-doc.py` 的 `read_c_docstring` ✓，两处辅助工具共用 ✓，见 ③）；两种写法都按 P-109 的转义规则解码，字符串型的续行与拼接（`"…" "…"` ✓）都要吃 ✓。③ **写盘的辅助工具**：`tools/add-swift-doc.py`（把载体写进 Swift 注册 ✓）与 `tools/fix-doc-text.py`（把漂移的字面量改回参照 ✓）都**先渲染、再与参照 DOC 的那条记录逐字节比**，不等就跳过并报告、绝不猜 ✓；两个工具都只改它们要改的那个字面量/形参 ✓。 |
| P-111 | **L5（自带 ERT 套件）的读数口径此前没有定**：套件是*差分*的（`docs/acceptance.md` 为每一级造了可运行的参照程序 ✓），而本轮实测出一种只能靠**重跑**才能分辨的现象：**同一个文件、同一个二进制，一次跑崩、下一次通过** —— `lisp/emacs-lisp/cl-seq-tests.el` 手工连跑 3 次里 1 次通过，而扫器第一次把它记成 `crashed`（背栈落在 `exec_byte_code`），pdumper 那类更是 3 次里 2 次 abort（空指针 UB 检查 ✓，与 ASLR 有关 ✓）⇒ 若不重跑，读数会把"抖动"算成"确定性缺陷"，缺陷表既虚高又难排序 ✗ | **规则，立即生效** | ① **harness 用参照自己的配方**：`tools/check-ert.py` 逐文件跑 `EMACSOPT` + `HOME=/nonexistent` + `-l ert` + 文件 + `ert-run-tests-batch-and-exit t`（参照 `test/Makefile` 62/83/123/183-187 ✓），两个程序各跑**各自那棵树**（端口 `femacs/test`、参照 `emacs/test` ✓），解析 ERT 汇总行比较 ✓。② **判定六类**：`same`（两侧 0 unexpected ✓）/ `port-worse` / `port-better` / `both-fail` /`flaky`（端口崩了、重跑通过 ✓）/ `crashed`（每次都崩、或超时、或没有汇总行 ✓）；**默认重跑一次**（`--retries` ✓）。③ **缺陷表 = flaky ∪ crashed ∪ port-worse** ✓：抖动仍是缺陷（参照从不抖 ✓），只是要单列，好让"修一个根因消掉一片"的进展看得见（pdumper 那条一修，`lisp/emacs-lisp` 切片里 33 个崩溃里的 22 个直接消失 ✓）。④ 汇报必须带重跑口径（例：`68 文件：same=31 / flaky=12 / port-worse=3 / crashed=22` ✓），不许只报一个总数 ✓。⑤ 超时就是缺陷（`gv-tests` 本轮连跑两次都没在 120 秒内出汇总 ✓）；但**跑法要省**：超时文件会把超时预算烧两遍，切片扫要把单文件超时调小或放后台跑 ✓。 |
| P-112 | **崩溃先分类：越界写还是逻辑缺陷，此前没有可判定的判据**：L5 的崩溃五花八门（`xmalloc` 内的 libsystem 断点 ✓、`mem_insert` 的空指针 ✓、`dispnew` 的 panic ✓），背栈只说明**受害者**而不是**加害者** ✓ | **规则，立即生效** | ① **先做最小复现梯度**（`-Q --batch --eval …` 逐级加负载 ✓）：本轮实测 `emacs -Q --batch --eval '(princ "")"` **10/10 干净** ✓、`-l ert` 与 `-l …/ert-tests.el` 各 **5/5 干净** ✓、而 `--eval '(find-file "femacs/lisp/subr.el")'` **5/5 崩** ✓ ⇒ 触发点在**把文件读进 buffer** 这条路上 ✓，`find-file` 是**最省的最小复现器** ✓（比整份 ERT 快两个数量级 ✓），但**不是确定性**的 ✗：同一命令在 18:40 的会话里 **5/5 崩** ✓、在 18:47 之后的会话里 **20/20 过** ✓（dump 与 ASLR 不变，环境与文件系统状态变了）⇒ **复现率必须当场量、并连着写清是哪个会话** ✓，不许把一次 5/5 当成"总能复现" ✗；反过来，一次 0/20 也**不能**证明缺陷消失 ✗（要回到会崩的那个状态或用能固定布局的装置 ✓）。② **用 macOS 的 malloc 调试开关判性质**：崩溃报告里 `exception.type = EXC_BREAKPOINT` 且 PC 落在 **libsystem** 内（本机 `0x1803f29f8` ✓）⇒ 那是 malloc 自己的完整性检查在报错 ⇒ **堆被写坏**（不是栈溢出 ✗）；再加 `MallocGuardEdges=1`（必要时 `MallocScribble=1` / `MallocErrorAbort=1` ✓）若症状消失或改观 ⇒ **布局相关的越界写** ✓（本轮：不加开关 5/5 崩、加上之后 **3/3 过** ✓）。③ 因此"修一个背栈站点"对这类崩溃**无效**：要先用 ASan/守卫分配器抓**写入点** ✓；本环境不能抬 `ulimit -s`（实测 `Operation not permitted` ✓），所以"栈不够"这条假设要用链接期的 `-Wl,-stack_size` 或帧尺寸读数来判，不能靠 ulimit ✗。 |
| P-113 | **`AUTO_*` 栈对象的生命周期：C 是"整个所在块"，端口写成了"宏内的分支块"** —— 这是 L5 那一族"malloc 报堆被写坏"的根因 ✓。C 的宏体是 `make_lisp_ptr ((&(struct Lisp_String) {{…}}), Lisp_String)`（`lisp.h` 5820-5826 ✓）：一个**复合字面量**，其存储期按 C99 6.5.2.5 是**所在块** ✓；`lisp.h` 自己的注释也写着这对象"has the lifetime of the containing block"、且"不该被交给 Lisp 代码" ✓。配置侧两边一致 ✓（`USE_STACK_LISP_OBJECTS` 在 clang 下为 true ✓、`GC_CHECK_STRING_BYTES` undef ✓ ⇒ `USE_STACK_STRING` = true ✓，C 5729 也说明 `-DUSE_STACK_LISP_OBJECTS=0` 只是调试开关 ✓）——**错的不是配置，是生命周期** ✗：端口的每族宏都在 `{{ … }}` / `if` 分支里 `let mut s = …; obj`，于是 `s` 的作用域在**分支块**结束 ⇒ 返回的对象指向**已死的栈** ✗（作者当年用 `black_box (&mut s)` 想"保住"它 ✗——`black_box` 不会延长作用域 ✓）。**ASan 直接点名了**：`stack-use-after-scope`，`READ of size 8`，位置是 `Fstring_equal`（`#1 assoc_ignore_text_properties` ← `Fget_buffer` ← `Fget_buffer_create` ← `get_minibuffer` ← `init_minibuf_once` ✓），受害者变量正是 `get_minibuffer` 帧里的 `s` ✓（`[32,64) 's'` ✓，shadow 是 `f8`）| **规则，立即生效 + 待实施清单** | ① **每一族 `AUTO_*` 的存储都必须声明在调用者的块里** ✓（对标 C 的复合字面量 ✓），即宏展开成**两个** `let`：①`let mut $name = <Lisp_String/Lisp_Cons …>;`（存储 ✓）②`let $name = …`（对象 ✓，**同名遮蔽** ✓）——遮蔽不会缩短第一个绑定的作用域 ✓，而它取过地址 ⇒ 编译器必须把这块栈留到块尾 ✓。② **表达式形宏在 Rust 里无法表达**：`AUTO_STR!`/`AUTO_STR_WITH_LEN!`/`AUTO_CONS_EXPR!`/`STACK_CONS!` 单独出现时，存储只能落在**语句**的块里 ✗ ⇒ 它们的站点一律改写为**语句形**（`AUTO_STRING!`/`AUTO_CONS!`/`AUTO_LIST1..4` ✓）：本轮实测 `callproc.rs` 4 处、`process.rs` 4 处（if/else 两臂 ✓）、`src/include/frame.rs` 1753 的 `AUTO_LIST1! ($name, AUTO_CONS_EXPR! (…))` 内部嵌套 ✓。③ **`AUTO_LIST2..4` 的嵌套是同一缺陷的双份**：C 里每个 `STACK_CONS` 各自是一个复合字面量、都在本块 ✓；端口要在一个绑定里声明**整条链**（`[Lisp_Cons; N]`，先整体零值再逐格赋值、然后把 `cdr` 连成链 ✓，PORTING 0.1-13 的既有写法 ✓），不许嵌套 `STACK_CONS!` ✗。④ **落地读数（2026-10-16 本轮 ✓）**：宏族已按 ①-③ 改写 ✓、8 个表达式形站点已改为语句形 ✓、C 的四个表达式宏名**保留在镜像里**但一律 `compile_error!`（在 Rust 里它们无法正确 ✗，失败要响 ✓），缺的 token 按本项目的机制记进 `docs/ports/macro-body-notes.tsv`（**13 行** ✓）；`extract-abi.py --all` = **590 done / 0 failing** ✓。⑤ **诊断装置（已进 `xtask`）**：`FEMACS_SANITIZER=address` 会让单元编译**与**两处链接都带上 `-Zsanitizer=address`（并自动 `RUSTC_BOOTSTRAP=1` ✓，stable 1.98 即可 ✓，无需切工具链 ✓）；**注意**：被消毒的轮次里 `gate-fast` 的符号门会报"a unit's symbol table differs from its reference object" ✓——那是 ASan 引入符号的**预期副作用** ✓，不是缺陷 ✓。 ⑥ **同族的第六个实例（4.426）**：`sysdep.rs` 的 `init_system_name` —— C 的 `#ifndef/#else` 是**预处理文本、不是块** ✓，所以 C 的 `char hostname_buf[256]` 与 `struct utsname uts` 活在**函数作用域** ✓；端口把它们放进 `#[cfg]` **块**里 ✗ ⇒ 缓冲区作用域提前结束，之后那段仍按指针读写它 ✗（ASan：`stack-use-after-scope`，受害者正是该帧的 `hostname_buf` ✓）。⇒ **规则**：C 在函数作用域声明、端口用 `#[cfg]` 块分臂的存储，必须把存储**提到函数作用域**、块里只留"取值/调用" ✓。 |
| P-114 | **`memset` 家族：C 的第三个参数是*字节数*，而 Rust 的 `std::ptr::write_bytes (p, val, n)` 的 `n` 是 `T` 的*个数*（函数内部再乘 `size_of::<T>()`）** ⇒ 把 C 的字节数交给带类型的指针会**多写 `size_of::<T>()` 倍** ✓。P-93 处理过 `memcpy` 家族的同一件事 ✓，但**只审了 `copy_nonoverlapping` 形状的站点** ✗，`memset` 这一族当时没审 ✗ —— 于是这一族一直在写穿内存 ✓：本轮 ASan 量到 `adjust_glyph_matrix` 的 rows 清零**一次写 1,572,864 字节**（= 24 行 × 256 字节 × 256 ✓），目标正好是那个 6144 字节数组的末尾 ✓，而它就是 L5 那一族"malloc 报堆被写坏、复现率随会话变"的**加害者** ✓ | **规则，立即生效** | ① **写法与 P-93 同形**：目的地指针**经 `*mut u8`** ✓，C 的字节表达式**原样保留**（`std::ptr::write_bytes (dst as *mut u8, 0, <C 的字节数>)` ✓）；反向也成立——若 C 写的是元素数（`size * sizeof *p` 交给 `*mut T`），那就把 `* sizeof` 写全 ✓。② **机器门**：`python3 tools/check-typed-counts.py`（覆盖 `copy_nonoverlapping`/`copy`/`write_bytes` 三族 ✓，判据 = 计数里出现 `size_of`/`size_of_val` 时指针必须是**字节型**（`u8`/`i8`/`c_char`/`c_void`，或显式 `as *mut u8` 等 ✓，`c_void` 在 Rust 里是 1 字节 ✓）✓；`--selftest` 2 反例 3 正例 ✓；**本轮测得 90 站点 / 6 缺陷** ✓，修后 **0** ✓）。③ **本轮修的 6 处**：`dispnew.rs` 703（rows 清零，加害者 ✓）、`dispnew.rs` 5925（`row_entry **` ✓）、`term.rs` 1611（`char_ins_del_vector` ✓）、`term.rs` 3853（C 3645 的 `sizeof (*state)` = **一个结构体** ✓，当元素数用会清掉 344 个 ✗）、`ftcrfont.rs` 855（`font_metrics **` ✓）、`include/w32.rs` 174 的 `FD_ZERO` ✓。 |
| P-115 | **env 守卫的诊断开关此前是"每轮随口发明"**（`FEMACS_GC_TRACE` 在 4.402 提过但从未落地 ✓）：本轮为了抓加害者连加两条（`FEMACS_XPALLOC_TRACE` 在 `alloc.rs` 与 `dispnew.rs` ✓），需要一个统一形状，否则下一次又会换名字、换输出格式、或在修好后忘记它还在 ✗ | **规则，立即生效** | ① 名字一律 `FEMACS_<话题>_TRACE`/`FEMACS_<话题>_CHECK` ✓；② **默认关闭**，判据是 `getenv` 非空（`no_std` 单元里不能用 `std::env` ✗，会引未定义符号 ✓）；③ 只写 **stderr**、每事件**一行**、字段固定为"位置 + 关键操作数" ✓；④ 必须**就地注明它属于哪条裁定**并在 `docs/status.md` 记一次读数 ✓；⑤ 修好后**不删**（它是回归时的现成仪器 ✓），但也不许在没有开关时影响行为或输出 ✓。 |
| P-116 | **L5 里三个"内存失控"文件（`ert-tests`/`package-tests`/`package-vc-tests`）不是三个缺陷，而是 P-101 那笔帧尺寸债务在**保守栈扫描**上的可见后果**：差分探针 `spikes/gc-retention-probe.el` 在两侧各跑 300 轮"造一个 2 万元列表再丢掉、每 100 轮显式 `garbage-collect` 一次"，读数 = 端口 **26,730 → 2,026,727 → 4,026,730** ✓ 对参照 **24,822 → 24,822 → 24,822** ✓ ⇒ 端口**每轮留住一个已丢弃的列表**（+2,000,000/100 轮 ✓），参照一个都不留 ✓。机理：保守扫描本身与 C 同形 ✓，但端口单元按 `-C opt-level=0` 编、解释器帧约为 C 的 **14 倍**（P-101 ✓）⇒ 每轮在不同栈偏移上留下**互不相同的旧指针** ✓，扫描就把整条调用链上残留的对象都标活 ✗；C 的帧小、同一批槽位被下一轮覆盖 ✓，所以只多留一两个 ✓ | **规则，立即生效** | ① **这三个文件按 P-101 的那一项处理，不许当三个独立 bug 逐个追** ✗——它们的"最小复现"就是上面那条探针 ✓（28 秒吃满 2 GB 的 ERT 文件只是这条曲线跑得久 ✓）。② **修法只有一条**：把帧缩回去，即把单元抬到 `-C opt-level>=1` ✓；而 P-101 记录了抬档位当前**被两层 unsafe 缺陷挡住**（`=2` 时 `styled_format` 的 `memmove` 读未映射地址 ✓；`=1` 时崩在第一条 `Loading` 之前 ✓）⇒ **先修那两处，再抬档位** ✓，顺序不许颠倒 ✗。③ 抬档位之前，L5 的内存读数必须**带着这条解释**（"与参照的 GC 保留量差异" ✓），否则会误判成 GC 缺陷 ✗。 ④ **更正（4.430）**：① 抬到 `opt-level=1` 后那条保留曲线**没有变化**（`spikes/gc-retention-probe.el` 仍 +2,000,000 conses / 100 轮 ✓）⇒ "帧尺寸"作为**唯一**解释不成立 ✗；② 端口**在自动 GC**（`gcs-done` 12 次/300 轮 ✓）而 specpdl **平坦**（2784→2944 字节 ✓）⇒ 既不是"从不 GC" ✗ 也不是 specpdl 泄漏 ✗；③ 端口**没有** C 的 `mark_byte_stack`（`bytecode.c` 434 的 `for (fp = bc->fp; fp; fp = fp->saved_fp)` ✓）⇒ 解释器的寄存器窗口只由**保守 C 栈扫描**覆盖 ✓，这条即下一个待验的根 ✓。修正后的规则：**不许把这条归因于帧尺寸就收工** ✗，按 ③ 继续（先按 C 补 `mark_byte_stack` ✓，再用 env 守卫的"收窄扫描"装置判定保留是否消失 ✓）。 ⑤ **第二轮更正与实测（4.431）**：⑴ **C 里根本没有 `mark_byte_stack`** ✗（上一轮那句是错的 ✓）⇒ "端口缺它"这条假设**作废** ✗；⑵ `(internal-stack-stats)` 两侧都**平坦**（端口 1 帧/1 run、参照 5 帧/3 runs ✓，300 轮不变 ✓）⇒ `bc` 链不泄漏 ✓；⑶ `FEMACS_GC_TRACE` 扩到标记栈后读数 = 每次 GC 都 **`mark_sp=0`** ✓（size 稳定 8192 ✓）⇒ 标记栈**没有**残留 ✓；⑷ `FEMACS_GC_SKIP_STACK_SCAN=1`（跳过保守 C 栈扫描 ✓）⇒ 程序**直接失败** ✓ ⇒ 扫描承载**活**数据 ✓（所以它不能当"跳过式"仪器用 ✗，但这条读数本身有价值 ✓）；⑸ **决定性**：把列表造在**被调函数内部且不返回**（`(defun probe-alloc () (make-list 20000 1) nil)` ✓）⇒ 端口**不保留**（650,232 → 648,244 → 668,586 ✓）；把它**返回给 elisp** ⇒ 逐轮保留 ✓ ⇒ 保留发生在**调用者的解释器状态**里 ✓；⑹ 把同一个循环的列表换成 **20 元** ⇒ **不保留**（630,255 → 632,247 → 634,251 ✓）⇒ 这条保留是**尺寸驱动**的 ✓。⇒ 下一条要点名的是"找到那个尺寸阈值，再找持有它的那条根"（阈值本身会指出机制 ✓）。⑺ **附带保真观察**：端口 `internal-stack-stats` 报 **1 run**、参照报 **3 runs** ✓ ⇒ 两侧解释器的"从 C 起跑的嵌套层数"不同 ✓（单列一项，不在本条内 ✓）。 ⑥ **第三轮更正与收窄（4.432）**：⑴ **"尺寸驱动"是错的** ✗——把测量表列全后，`20 元` 的 delta 也正好是 100×20 ✓ ⇒ 那是"**每轮一个对象**"在不同尺寸下的同一现象 ✓（我上一轮把绝对量当成了阈值 ✗）。⑵ **必要条件不是尺寸，是"把值绑到 elisp 局部变量"** ✓：`(setq probe-keep (length (make-list len i)))`（不绑 ✓）⇒ 长度 3…32 **全部不保留**（delta −15…+14 ✓）；`(let ((x (make-list len i))) (setq probe-keep (length x)))`（绑 ✓）⇒ 长度 3…64 **全部每轮留一个**（delta = 200×len ✓，per_iter 2→63 ✓）。⑶ 同轮排除：`handlerlist` **恒定 4** ✓；`mark_c_stack` 的扫描**跨度小而稳定**（3,984→13,856 字节 ✓）⇒ 扫描范围假设作废 ✗；标记栈 `mark_sp=0` ✓、specpdl 平坦 ✓ 不变。⇒ **下一个要查的就是解释器对 `let` 局部的实现**（端口把它放在哪、由谁持有 ✓，与 C 的 `Bbind`/寄存器窗口逐处对 ✓）——它是现在唯一与"绑定即保留"这条判据一致的机制 ✓。 |
| P-117 | **P-116 那条保留的根因：端口自造的 `sa_stack` 池永不释放**（内存失控与"每轮留住一个对象"是同一个原因 ✓）。背景：C 把 `SAFE_ALLOCA` 落在**真正的 C 栈**上（`alloca` ✓），`SAFE_FREE()` 只是 `safe_free (sa_count)` = **按 specpdl 回卷**（`while (specpdl_ptr != specpdl_ref_to_ptr (sa_count))` ✓；alloca 那块无需释放 ✓、`record_xmalloc` 的回退由 unwind 释放 ✓）⇒ **C 里根本没有"池"这个机制** ✓。端口因为 Rust 没有 `alloca`（P-05 ✓）自造了 `sa_stack`（`alloc.rs` 1035-1075 ✓：`sa_stack_alloc` 以 `malloc` 取块、把块头（含 **分配时的 specpdl 深度** ✓）压进 `sa_stack_top` 链 ✓；`sa_stack_release (depth)` 按 `(*top).depth > depth` **严格**弹栈并 `free` ✓）。**实测（4.433）**：`sa_stack_release` **零调用者** ✓（全树 grep ✓）⇒ 链只涨不缩：183 块/93 KB → 873/121 KB → 1,398/319 KB → **255,232 块 / 40.9 MB**（约 1,600 轮循环 ✓）。而 GC 在 `garbage_collect` 里**逐块 `mark_memory` 扫这条链**（`alloc.rs` 6461-6475 ✓，那是端口替 C 的栈扫描补的 ✓）⇒ 块中残留的 `Lisp_Object` **全部成为永久根** ✗ ⇒ `gc-retention-probe` 的"每轮恰好留住一个被绑到局部的对象" ✓ 与三个 ERT 文件的内存失控 ✓ **一次解释清楚** ✓（两条判据：`sa_blocks`/`sa_bytes` 单调增长 ✓、把值绑到 elisp 局部才保留 ✓）。| **规则，立即生效** | ① **修法是给"一对 `SAFE_ALLOCA`/`SAFE_FREE`"配对一道水位线** ✓：`SAFE_ALLOCA` 进入时记下**当前 `sa_stack_top`** ✓、`SAFE_FREE` 把水位线以上的块全部 `free` 并弹链 ✓；**不许**改成 `depth >=` 的宽松比较 ✗ —— 同深度的外层帧可能正持有仍在用的块（`f()` 分配后调 `g()`，`g()` 的 `SAFE_FREE` 会把 `f()` 的块释放掉 ✗），这正是现有那个"严格"注释想避免的 ✓。② 水位线**不必**新增宏参数 ✓：`SAFE_ALLOCA!`/`SAFE_FREE!` 是**词法配对**的 ✓（C 的用法即如此 ✓），用一张 LIFO 的水位线表即可 ✓；`SAFE_ALLOCA!` 的 `record_xmalloc` 回退分支**也要压一个"空水位线"** ✓（否则配对错位 ✗）。③ **验收**：`FEMACS_GC_TRACE` 的 `sa_blocks`/`sa_bytes` 在 `gc-retention-probe` 300 轮后必须**回到个位数块** ✓，且探针读数与参照同形（δ≈0 ✓）；④ 顺带的收益：`gc-retention-probe` 与三个 ERT 文件的内存失控应同时消失 ✓（本条是它们的共同根因 ✓）；`opt-level` 抬档位**不是**本条的前提 ✗（那是另一笔债 ✓）。 ⑤ **第一版修法被证据否证（4.434）**：把 `SAFE_ALLOCA!`/`SAFE_FREE!` 做成"压/弹水位线 + 块计数守卫"（`sa_stack_mark`/`sa_stack_unmark` ✓）后，**端口的 `temacs` 启动即崩**（`check-coding-eol` 的 15 例全 `None` ✓、exit −5 ✓），而 `dump` 仍成功 ✓ ⇒ 说明**释放越界**（把别处仍在用的块还了 ✗）⇒ **这个池的释放不是 LIFO 配对的** ✗（端口 `SAFE_ALLOCA!` 277 处 vs `SAFE_FREE!` 188 + `SAFE_FREE_UNBIND_TO!` 93 处 ⇒ 形态上就不平衡 ✓；块计数守卫只能"少还"、挡不住"弹错一个外部水位线" ✓）。已**回退** ✓，回退后四道门全绿：`gate-fast exit=0`（590 done / 0 failing ✓）、`gate-load exit=0` ✓、`dump exit=0` ✓。⇒ **规则补充**：下一版修法**不许**假设配对 ✗，必须用"**不清空即复用**"或**按函数作用域**的挂钩之一：⑴ 首选 **清空 + 归还到自由链表**（按 size class ✓）：出作用域时把该块的字节清零 ✓（陈旧 `Lisp_Object` 不再成根 ✓）、块进自由链表供下次复用 ✓（内存不再涨 ✓）；**前提是先有一条可靠的"作用域结束"判据**——端口现有的两条（`safe_free` 的深度比较 ✓、水位线配对 ✗）都不够，所以这一步要与 ⑵ 一起做；⑵ 若要做到 C 的"随帧释放"语义，就得给 `USE_SAFE_ALLOCA` 覆盖的那些函数加**不可绕过的收尾**（Rust 里即 `Drop` 守卫 ✓，属 P-05 的已知缺口 ✓），这要单独一条裁定 ✓。⑶ **另一个已实测的门槛**：端口私有的 `#[unsafe(no_mangle)]` 助手会被清单门判为"invented exported symbol" ✗ ⇒ 新增这类名字**必须同时登记**到端口的符号基线（既有的 `sa_stack_alloc`/`sa_stack_release` 就在里面 ✓），否则 `gate-fast` 直接红 ✗。 ⑥ **第二版修法同样被否证、已回退（4.435）**：按"给池加一层按 specpdl 深度分桶的作用域表、在 `USE_SAFE_ALLOCA!` 里登记"实施（新增 `sa_pool_scope_enter`/`sa_pool_scope_leave` ✓）后，**泄漏确实被修掉** ✓（`FEMACS_GC_TRACE` 的 `sa_blocks` 255,232 → **13** ✓、`gate-fast exit=0`（590 done ✓）、`dump exit=0` ✓）**但释放越界** ✗：`gate-load` 的 15 例全 `None` ⇒ "specpdl 深度相同"仍不是可靠的"作用域结束"判据 ✗（它把仍在用的块还了 ✗）。⇒ **规则补充**：⑴ 该池的"作用域结束"判据**只认 specpdl 的 unwind 机制**（`record_unwind_protect`，或 specpdl 里带深度的标记 ✓）——纯栈式压/弹与"按深度分桶"都被否证 ✗；⑵ 下一版**必须双侧验收**：`gc-retention-probe`（`sa_blocks` 个位数、δ≈0 ✓）+ 三个失控文件 + `gate-load`/`check-coding-eol` ✓，**不许**只验 `dump` ✗；⑶ 为实验加的白名单登记，源码回退后就是**孤儿条目** ✗ ⇒ 必须一并回退 ✓（本轮的 `sa_pool_scope_*` 已回退 ✓）。 ⑦ **第三版（4.436）：每次请求挂到 specpdl 上——也被否证，但这次拿到了"为什么所有版本都崩"的机制** ✓。做法：每次 `sa_stack_alloc` 申请 `[大小字][载荷]` 一个块并 `record_unwind_protect_ptr_mark (Some (xfree), chunk, Some (sa_stack_mark))`（C 的 `sort.c` 594 形状 ✓），释放交给 `safe_free` 的走栈、标记交给 `mark_specpdl`，链、深度比较与 GC 里那段逐块扫描全部删除 ✓。**读数**：`gate-fast=0` ✓、`dump=0` ✓，而 **`gate-load=1`**：`check-coding-eol` 的 15 例全 `None`，`temacs` 在启动加载 `custom.el` 时 **SIGABRT**（`___BUG_IN_CLIENT_OF_LIBMALLOC_POINTER_BEING_FREED_WAS_NOT_ALLOCATED`，被 free 的是**栈地址** ✓）。**用新仪器读到的现场**（`FEMACS_SA_TRACE`，P-115 形状，两份文件里各一条打印 ✓）：`safe_free` 的走栈在 `sa_count` 之上撞到一个**活的 `SPECPDL_BACKTRACE` 条目**，把它的 `bt.args`（栈地址 ✓）当 `unwind_ptr.arg` 交给 `xfree` ✗。**机制**：C 的 `safe_free` 是**按深度**走栈的，它要求"从 `USE_SAFE_ALLOCA` 到 `SAFE_FREE` 之间只压过 SAFE_ALLOCA 族的条目"（C 把这条纪律写成 `eassert (func == xfree)` ✓）；而端口树里有**大量"C 里压不压都无所谓"的位置**——C 的 `alloca` 路径压不出条目，所以 C 里 `USE_SAFE_ALLOCA` 之后**没有** `SAFE_FREE` 完全合法 ✓。一旦端口在那些位置压了条目，**别处靠"栈顶就是我的条目"的相对弹栈**（`specpdl_ptr--`、`unbind_to (SPECPDL_INDEX () - arg, …)`）就会弹错对象 ⇒ 真条目被留在栈上 ⇒ 下一次 `safe_free` 走栈把它当 `UNWIND_ARRAY`/`UNWIND_PTR` 释放 ✗（v1/v2 的"释放越界"也属这一类 ✓）。⇒ **两条硬约束（后续任何一版都必须满足）**：⑴ **不许往 specpdl 里加条目** ✗；⑵ **不许把池子的释放建立在 `safe_free` 的走栈上** ✗。⑶ **归因复核**：用 `git show HEAD:` 把两处源码与两个工具的改动全部还原后，同一台机器上 **`gate-load=0`（15 例 0 failure ✓）** ⇒ 崩溃确由本版引入，不是旧缺陷 ✓。 ⑧ **第四版（未实施，约束已定）**：只动释放，**不加 specpdl 条目**、**不靠 `safe_free` 的走栈**。做法：`USE_SAFE_ALLOCA!` 再声明一个**帧内**水位线（`sa_mark`，与 `sa_avail`/`sa_count` 同形 ✓，端口既有的记账偏离 ✓），`SAFE_FREE!`/`SAFE_FREE_UNBIND_TO!` 把池子高度还原到它（`sa_stack_release_to (sa_mark)` ✓：释放水位线**以上**的块 ✓）；标记与 GC 扫描仍沿用链（HEAD 的形状 ✓，它已被 `gate-load=0` 与 L4 证明可用 ✓）。为什么它能行而 v1/v2 不行：v1 的水位线挂在**全局栈**上（非 LIFO ⇒ 弹错别人的水位线 ✗），v2 按 **specpdl 深度**分桶（同深度的兄弟作用域撞桶 ✗）；**帧内变量**既天然 LIFO ✓、又不与别的帧撞 ✓。残余风险只有一条：C 的 `alloca` 活到**帧返回**，本版活到 `SAFE_FREE`（= C 的 `record_xmalloc` 回退路径的寿命 ✓）；4.436 的静态审计（全树 `SAFE_ALLOCA`/`SAFE_FREE` 站点）**没有**找到"`SAFE_FREE` 之后还使用缓冲"的站点 ✓，但仍以双侧验收为准：`gc-retention-probe`（`sa_blocks` 个位数、δ≈0 ✓）+ 三个失控文件 + `gate-load`/`check-coding-eol` ✓。调用点：约 200 处 `USE_SAFE_ALLOCA!` 与 281 处 `SAFE_FREE!`/`SAFE_FREE_UNBIND_TO!` 同步改形 ✓（纯机械 ✓）；§3.2 的措辞随它落地时一并更新 ✓。 ⑨ **第四版（4.437，已落地、四门全绿）：帧内水位线** ✓。做法：`USE_SAFE_ALLOCA!` 声明第三个**帧内**名字 `sa_mark`（= `sa_stack_mark ()` 读到的链高 ✓），`SAFE_FREE!`/`SAFE_FREE_UNBIND_TO!` 把池子还原到它（`sa_stack_release_to (sa_mark)` ✓ 释放水位线**以上**的块 ✓）；`safe_free` 回到 C 的原文（不含池子那行 ✓ ⇒ 约束 ② 满足 ✓）；**不新增任何 specpdl 条目**（约束 ① 满足 ✓）；GC 仍按链扫描（HEAD 的装置 ✓）。附加守卫：块记下所属 Lisp 线程，释放遇到别的线程的块就停（那是挂起不是死亡 ✓）。**读数**：`gate-fast exit=0` ✓（590 done / 0 failing ✓）、`gate-load exit=0` ✓（15 例 0 failure ✓）、`dump exit=0` ✓（13,025,288 字节 ✓）；**判别性探针**（同机 ✓）端口 `26,330 → 26,259 → 26,259` ✓ 对参照 `24,822` 平 ✓（修前 `26,730 → 2,026,727 → 4,026,730` ✗ = +2,000,000 conses/100 轮 ✓）；`FEMACS_GC_TRACE` 的 `sa_blocks`/`sa_bytes` 从 **255,232 块 / 40.9 MB** 降到 **4 块 / 48 字节** ✓。⇒ **P-116/P-117 的"每轮留住一个对象"关闭** ✓。 ⑩ **三个 ERT 文件的内存失控不是这一条（4.437 更正）** ✗：`ert-tests` 仍在 **19 秒吃满 1 GiB** ✗，而那一刻池子是健康的（`gc-h: sa_blocks=63..109 / sa_bytes=1..2.5 KB` ✓）；把它最后跑到的 `ert-test-run-tests-batch` **单独**跑，端口 0.37 秒、峰值 **45 MB** ✓ ⇒ 它的增长是**跨测试累积**的另一种保留 ✗。⇒ 规则："三个 ERT 文件"按**独立缺陷**继续查（下一站：P-116 曾设想的"收窄保守扫描"装置，在 ERT 场景里定位 ✓），**不许**再当作 P-117 的后果 ✗。 ⑪ **改判的改判（4.438）：三个 ERT 文件的"内存失控"也不是缺陷，是本 L5 装置用错了选择器** ✗✗。`tools/check-ert.py` 一直传 `t`（= 参照 `test/Makefile.in` 145 的 `SELECTOR_ALL` ✓）而当默认，于是把各文件里 `:expensive-test`/`:unstable` 的测试也跑了 ✗；`lisp/emacs-lisp/ert-tests.el` 上这会**同时**打爆两侧：参照 **1.26 GiB / 3.4 秒**被上限杀 ✓、端口 **1.1 GiB / 20 秒**同样 ✗ （逐测试读数：两侧都在 `ert-test-run-tests-batch` 之后的那一条上崩，GC 各类计数都很小、`sa_blocks` 64-108 块 ✓ ⇒ 与池子无关 ✓、也非端口独有 ✓）。改用参照自己的 `SELECTOR_DEFAULT`（`test/Makefile.in` 139 ✓）后，同一个文件 **same=1**：两侧各 73 个测试、各 0 unexpected ✓✓。⇒ P-117 ⑩ 的"独立缺陷"这句**作废** ✗；三个文件按新读数分别是：`ert-tests` **same** ✓、`package-tests` **port-worse（3 unexpected vs 0，峰值 70 MiB ✓）**、`package-vc-tests` **crashed（exit 255，进程哨兵里 `Wrong type argument` ✗，峰值 78 MiB ✓）** ⇒ 后两个是**真实、有界**的缺陷 ✓（下一站点名 ✓），与前两个都无关"内存"二字 ✗。 | |
| P-118 | **常驻规则的回填 + 两条操作纪律此前没有裁定**：用户 2026-10-16 的口头约束里有几条只在对话里出现、没落到 `AGENTS.md`（"受控执行器是唯一入口" ✓、"被否证的实验连同白名单一起回退" ✓、"复刻期与 C 严格同形、调整留到验收之后再判断" ✓、"还原文件的唯一姿势" ✓）⇒ 换执行者或上下文压缩后必漏 ✗ | **规则，立即生效** | ① `AGENTS.md` §1.11 / §3 / §4 / §6 四条已按本节口径补齐 ✓（`AGENTS.md` 仍是唯一落点，P-103 ✓）；② 一切编译/链接/测试/解释器/行为未知程序**只经** `controlled-run` 执行器 ✓，超限**缩范围**而不是加额 ✗、不许绕开监控 ✗；③ 实验**先立项后动手** ✓，被否证的版本**连同其白名单/基线/工具改动一并回退** ✓，回退后**四门复绿**才允许推进 ✓；④ 复刻期的结构/命名"顺手改好"一律延后到全部重写完成后的全面验收阶段 ✗。 |
| P-119 | **L5（`tools/check-ert.py`）的读数口径此前有两处硬伤**：⑴ 它传的选择器是 `t`（`SELECTOR_ALL`）而当默认 ✗ ——参照的 `make check` 跑的是 `SELECTOR_DEFAULT = (not (or (tag :expensive-test) (tag :unstable)))`（`test/Makefile.in` 139-145 ✓），于是 `:unstable`/`:expensive-test` 的测试被拉进来跑，`ert-tests` 上把两侧一起打爆 ✗ （引用见 P-117 ⑪ ✓）；⑵ 每侧没有内存上限 ✗，一个重文件会连带把执行器（2 GiB）一起吃掉 ⇒ 读数变成"整轮被杀"而不是"某文件被限" ✗（本机 Darwin **拒绝**下调 `RLIMIT_AS`：`Invalid argument` ✓，所以上限要两条腿：`setrlimit` 尽力而为 + `ps` 看门狗 ✗→✓）。 | **规则，立即生效** | ① 默认选择器 = 参照的 `SELECTOR_DEFAULT` 原文 ✓；另外两个预设按参照给（`expensive` = `(not (tag :unstable))` ✓、`all` = `t` ✓），三者都用 `--selector`/`--selector-preset` 显式可换 ✓；② 每侧必须带内存上限（`--memory-mib` ✓）：`setrlimit (RLIMIT_AS)` **尽力而为**（Linux 有效 ✓、Darwin 会失败 ⇒ 不报错 ✓）+ `ps` 采样看门狗（每 0.5 秒 ✓，越限即 kill ✓）；③ 判定表新增 `both-capped`（两侧都被上限杀 = 文件对装置过重 ✓，不算端口缺陷 ✗）✓；④ 这条只管"默认跑什么" ✓：`SELECTOR_EXPENSIVE`/`SELECTOR_ALL` 属于**压力门**（DoD 的"全量含 `:expensive-test`、0 跳过"那一格 ✓），随时可显式开 ✓。 |
| P-120 | **"原样复制"此前只有口号、没有读数**（规则 7 ✓ 与 goal 的硬约束 ✓，但没人量过到底差多少 ✗）：本轮把六个树（`lisp leim test etc doc admin` ✓）的**追踪文件**逐个比对 ✓，判据是"字节一致 ✓ / 只在本文件声明行的产品名上改写 ✓ / 其他 ✗" ✓；实测 4,595 个追踪文件（除 9 个已登记的 `Makefile.in` 退役件 ✓）里 **2,592 字节一致 ✓ + 1,691 仅声明行改写 ✓ + 312 其他 ✗ + 0 缺失 ✓**。 ⑵ **同轮更正（4.440）**：那"312 其他"是**我临时判据的产物** ✗ —— 本仓库早就有权威装置：`tools/check-verbatim.py` + `docs/baseline/verbatim-substitutions.tsv`（0.1-19 第十/十一半 ✓ **8,427 行**逐行登记 ✓，每行给路径/行号/改写后文本/理由 ✓），它读的是**登记过的行**而不是我的锚点规则 ✗；实测它读数 = `6 trees, 4604 files, 8427 sanctioned line(s), 0 failures` ✓✓（我把临时写的 `tools/check-copied-trees.py` **删掉**了 ✗，因为它与既有门重复且口径更松 ✗）。⑶ **真正的发现**：那 9 个退役件让这个门**红了很久** ✗ —— `lisp` 1 / `doc` 4 / `leim` 1 / `admin` 3 缺失 ✓ + 台账里 9 行"没有树载它" ✗ = 5 failures ✓，而**没有任何调用者** ⇒ 修法三条：① 新增 `docs/baseline/verbatim-absent.txt`（9 行：路径 + 理由 + 指向 `docs/retired-scaffolding.md` ✓）让缺失成为**读数** ✓；② 从台账里**剪掉**那 36 行退役件条目 ✓；③ 把该门**接进 `gate-fast`** ✓（"没人跑的门等于没有门" ✓）。修后 `gate-fast exit=0` ✓ 且其中新步骤打印 `copied trees: every tracked file is byte-identical or sanctioned` ✓。那 312 的方向是**两边都有** ✗：`ChangeLog.*` 的历史条目被**保持原样**（正确 ✓，它们不是"本文件声明" ✓）、而 `etc/DEBUG` 标题、`doc/**` 与 `etc/**` 的散文里"the Emacs manual"之类被**过度改写** ✗；⇒ 旧的那次批量改写既无锚点、也未登记 ✗。 | **规则，立即生效** | ① 判据固定为：**除本文件自己的 GPL 声明块（`This file is part of …` / `is free software …` / `is distributed …` / `along with …` 四句）里的产品名外，一律与参照字节一致** ✓；文件内的历史条目、散文、标题**不改** ✗。② 由 `tools/check-copied-trees.py` 机器判定 ✓（本文件即该门的规格 ✓），它按目录给出三类计数并列出"其他"的样本 ✓；③ 那 312 个按 ① 重新对齐（历史条目恢复参照文本 ✓、本文件声明块按其锚点改写 ✓），完成后该门必须 0 其他 ✓；④ 九个退役件是**唯一**允许缺失的文件 ✓，其记录在 `docs/retired-scaffolding.md` ✓。 |
| P-121 | **`finder-inf.el` 一类"参照构建写进自己源码树、但不入库"的产物，端口此前没有归属** ✗：参照的 `lisp/Makefile.in` 166-170 用自己刚构建的 emacs 跑 `-l finder` + `finder-compile-keywords-make-dist` 写出它 ✓（输入 = `SUBDIRS_FINDER` = lisp 下除 `leim*`/`obsolete`/`term` 的全部目录 ✓ 121-129 行 ✓），而端口的 `lisp/` 里没有它 ✗ ⇒ `package-tests` 三个用例失败 ✗ （`describe-package` 报 `file-missing … finder-inf` ✓、两个 filter 用例报 `user-error "No packages found"` ✓，因为内建包表为空 ✓）。 | **规则，立即生效** | ① 这类产物一律**由 `tools/gen-*.py` 生成** ✓（P-102 的工具语言政策 ✓），并**照参照的配方**跑本仓库自己的程序 ✓：`tools/gen-finder.py` 即此 ✓（生成到 `femacs/lisp/finder-inf.el` ✓，`.gitignore` 已覆盖 ✓，与参照一样**不入库** ✓）。② 判据：与参照自己那份产物**逐行一致** ✓，唯一允许的差异是**已登记的产品名改写**（生成器读的是被改写的复制树 ✓ `docs/baseline/verbatim-substitutions.tsv` ✓）⇒ `--check` 打印"sanctioned rewrite(s) / other" ✓，`other` 必须是 0 ✓。③ 实测（4.441）：端口产物 **538 行**、与参照 **538 行**同长 ✓，**16 处**登记改写 ✓、**0 其他** ✓；生成后 `package-tests` 由 `port-worse（3 unexpected）` ⇒ **same=1** ✓✓（端口与参照各 40 例 / 各 0 unexpected ✓）。④ 未竟：`.elc` 全族、`cus-load.el`、`loaddefs`、unidata/charsets、ja-dic 等同类产物按同一口径逐项落地 ✓。 |
| P-122 | **`package-vc-tests` 的间歇失败定位到"被 advice 包过的进程哨兵"** ✓：`package-vc-tests-upgrade-after-require/test-package-two` 单测在端口上 **8 次里失败 4 次** ✓、参照 **8/8 通过** ✓（4.443 ✓）；通用异步层已排除（裸 `make-process`+`:sentinel` 6/6 一致、0.022-0.028 秒 ✓）。本轮把 `vc-exec-after` 的机制（`(add-function :after (process-sentinel proc) fun)` ✓ `vc-dispatcher.el` 239-320 ✓，`vc-run-delayed` 建在它上面 ✓）单独做成探针 ✓：**端口 20 次里 14 次丢失**（连**原哨兵**都没跑 ✓、且进程 2.5 秒后仍是 `run` ✗），参照 **0/20** ✓✓ ⇒ 缺陷就在"advice 包过哨兵后的进程收割/哨兵派发"这一层 ✓。| **规则，立即生效** | ① 该探针入库为 `spikes/process-sentinel-advice-probe.el` ✓（P-115 的仪器习惯 ✓），读数写在本条与 `docs/status.md` ✓；② 下一版诊断从三层里选一层定案：`add-function` 对**进程哨兵**的包装本身 ✓ / 被包装后哨兵的**调用路径**（`process.c` 的 `status_notify`/`wait_reading_process_output` ✓）/ **收割**路径（SIGCHLD→`process-status` 仍为 `run` ✗）；③ 定位后修 `process.c` 一侧（不许改测试、不许改 `lisp/` 的复制树 ✓），验收 = 探针 **0 丢失** ✓ + `package-vc-tests` 整片 45 例 ✓ + `gate-fast`/`gate-load`/`dump` 复绿 ✓。 ⑵ **更正（4.445）**：上一条把层位写成"被 advice 包过的哨兵" ✗ —— 对照组（**不带** advice 的同形探针 ✓ `N=20`）同样失败（端口 **13/20** ✗、参照 **0/20** ✓）⇒ advice **不是**变量 ✓。真正的变量是**子进程退出与等待的相对时机** ✓：`sh -c "exit 0"`（**等待开始前就已退出**）端口 **14/20** 丢失 ✗、参照 0/20 ✓；`sh -c "sleep 0.05; exit 0"`（在等待**期间**退出）端口 **0/20** ✓（第二轮 N=10 时 **2/10** ✗ ⇒ 并非绝对 ✓）。每次丢失的签名相同：**哨兵从不运行** ✓、且 `accept-process-output` 2.5 秒后 `process-status` 仍答 **`run`** ✗ ⇒ **子进程根本没有被收割** ✓（与哨兵怎么包装无关 ✗）。⇒ 诊断方向改为：端口漏掉了"等待开始前/瞬间退出"的子进程的退出通知 ✓（查 `keyboard.c` 的 SIGCHLD 处理 ✓ + `process.c` 的 `wait_reading_process_output` 播种/轮询那条路径 ✓，与 C 逐句对照 ✓）；探针已按两种形状 + 三个旋钮（`N`/`ADVICE`/`CHILD` ✓）入库 ✓，验收不变（探针 0 丢失 ✓ + 整片 ✓ + 三门口 ✓）。 ⑶ **仪器与读数（4.446）**：给端口加了 `FEMACS_CHILD_TRACE`（P-115 形状 ✓ 默认关 ✓ 只写 stderr 一行 ✓）：`handle_child_signal` 入口 ✓、真正收割到子进程那一支 ✓、`child_signal_notify` 的写端 ✓、`catch_child_signal` 入口 ✓（`process.rs` 共 +15 行 ✓ 逻辑零改动 ✓，`gate-load=0` ✓ 15 例 0 failure ✓）。**快退出子进程探针**（N=4 与 N=6 ✓）读数：missed **2/4** ✗ 与 **5/6** ✗，而四类 trace **一条都没有** ✓✗ ⇒ （a）**SIGCHLD 处理器在整个探针期间从未运行** ✓、（b）**`catch_child_signal` 也从未被调用** ✓ ⇒ 丢唤醒不在"循环的写法"（本轮逐句核对 `wait_reading_process_output`/`status_notify`/`handle_child_signal`，与 C **一致** ✓）而在**信号的安装/屏蔽层** ✓。⇒ 下一步：⑴ 找出端口在哪里安装 SIGCHLD（`emacs_sigaction_init`/`init_signals` ✓）并与 C 对照 ✓；⑵ 查端口的信号屏蔽（`block_child_signal`/`unblock_child_signal` ✓ 与等待前的 `sigprocmask` ✓）——**"SIGCHLD 被永久屏蔽"恰好能解释这份读数** ✓✓；⑶ 定位后修并跑三形状探针（目标 0 丢失 ✓）+ `package-vc-tests` 整片 ✓ + 三门口 ✓。 ⑷ **仪器扩到收割本体（4.447）**：`FEMACS_CHILD_TRACE` 再加一条 `get_child_status`（`sysdep.rs` ✓）—— 它是端口**唯一**的 `waitpid` 包装 ✓（`child_status_changed`/`process_status` 都经它 ✓）⇒ 没有这条打印就说明**根本没发生收割** ✓。**读数**：`slow` 子进程 **3/3 丢失** ✗、`fast` **2/3 丢失** ✗，而 `child-get` 打印 **0 条** ✓✗ ⇒ 探针期间**一次收割都没有** ✓（连"成功"的那一两次也没有 ✗ ⇒ 那几次是 **EOF/EIO** 路径结束的 ✓ 与 `waitpid` 无关 ✓）。**同时如实记下**：miss 率**在轮次间波动**（`slow`：0/20 ✓ → 2/10 ✗ → 3/3 ✗）⇒ 与环境/负载相关 ✓。两个候选取其一在下一轮定案：⑴ **SIGCHLD 未安装或被屏蔽** ✓（打印 `sigaction` 处置与当前屏蔽字 ✓）；⑵ **等待的 fd 集不含子进程管道** ✓（打印 `Atemp`/`max_desc` 的计算 ✓）。两处改动都过 `extract-abi --check` ✓（sysdep OK ✓ process OK ✓）、`gate-load=0` ✓。 ⑸ **最小复现 + "打印改变结果"（4.448）**：**一行 elisp 即可复现** ✓ —— `(let ((p (make-process :name "x" :command '("sh" "-c" "exit 0") :buffer nil :noquery t))) (dotimes (i 40) (accept-process-output nil 0.01)) (process-status p))` ⇒ 端口 **`run`** ✗（子进程永不收割 ✓）、参照 **`exit`** ✓。**同时发现仪器本身的异常** ✗：`wait-enter`/`child-state` 打印**从不出现** ✓，而 `accept-process-output` **确实在等**（40×0.01 秒实测 **0.521 s** ✓ 对照参照 **0.495 s** ✓）；把入口打印改成**无条件**（临时 ✓、已还原 ✓）后，同一条复现**变成 `status=exit`** ✓✓ ⇒ 说明端口这条等待路径**对时序/代码布局敏感（UB 形状）** ✓，与"miss 率在轮次间波动" ✓（P-122 ⑷）一致 ✓。⇒ 下一站锁定 `wait_reading_process_output` 的栈/fd_set/timeout 处理与 C 逐句对照 ✓（P-113 未初始化局部 ✓、P-114 字节数 ✓、P-113 作用域生命周期 ✓ 三条已知家族 ✓）。仪器保持 **默认关** ✓（env 守卫 ✓）、`gate-load=0` ✓（15 例 ✓）、`extract-abi --check` 两个单元 **OK** ✓。 ⑹ **同一二进制上的稳定读数 + 环境变量无关（4.449）**：⑴ 一行复现（P-122 ⑸ ✓）在**同一个二进制**上连跑 5 次：**`exit` 2 / `run` 3** ✗（≈60% 丢失 ✓ 与历史各轮同量级 ✓）；`FEMACS_CHILD_TRACE=42` 与不设**结果完全相同** ✓（2/3 对 2/3 ✓）⇒ **env 变量本身不改变行为** ✓（4.448 那次"无条件打印就变好"是同一批里的好运 ✓，不是变量 ✓）。⑵ **已排除的两个 UB 嫌疑** ✓：`fd_set`/`FD_SETSIZE` **尺寸无问题** ✓（端口用系统头 `<sys/select.h>` 的 `fd_set` 与 `FD_SETSIZE 1024` ✓，C 在 Darwin 上同样 ✓ —— `sysselect.h` 里那套 `typedef int fd_set` 是**死分支** ✓），`thread_select` 的形状也与 C 相同 ✓（都经 `flush_stack_call_func (really_call_select, &sa)` ✓）。⑶ **仍未解释的仪器现象** ✗：四处打印（`accept-apo-enter`/`wait-enter`/`child-state`/`child-get`）在**正在复现的那几次运行里也一条都不出现** ✓，而字符串确在二进制里 ✓、等待确实发生 ✓（0.52 s ✓）。⇒ 下一轮的处理顺序：① 先在**必然执行**的位置（循环体内的原地打印 ✓）验证"打印是否出现"这件事本身 ✓，并把输出同时写文件（`open`+`write` ✓）以排除 stderr 缓冲/重定向的干扰 ✓；② 若确认打印可达，则用"打印是否出现"作探针定位**哪一段没被执行** ✓；③ 之后再动 `wait_reading_process_output` 的栈/fd_set/timeout 处理 ✓。两处仪器 `extract-abi --check` 与 `gate-load=0` 均 ✓（15 例 ✓）。 ⑺ **A/B 定案与一个新缺陷（4.450）**：⑴ **不是 dump 特有** ✓：同一行复现连跑 6 次 —— 端口 **`temacs`**（未 dump）**exit 4 / run 2** ✗、端口 **`emacs`**（已 dump）**exit 3 / run 3** ✗、参照 **6/6 exit** ✓ ⇒ 竞态在两种二进制里都在 ✓（≈1/3 ✓）。⑵ **新发现的独立缺陷 ✓✗：C 层 `getenv` 在端口"已 dump"的二进制里失效** ✗ —— 同一条命令在 **`temacs`** 上四处仪器**全部打印** ✓（`accept-apo-enter` ✓ `wait-enter` ✓ `child-state: handler=… blocked=0` ✓ `child-get` ✓ `child-reap: pid=… status=0 WIFEXITED=1` ✓、`status=exit` ✓），在 **`emacs`** 上**一条也不打印** ✗，而 elisp 的 `(getenv …)` 正常 ✓（⇒ Lisp 侧环境对、C 侧 `environ` 没跟上 ✓）。这也**一次性解释了** 4.444-4.449 所有"仪器静默"现象 ✓✗，并意味着 **P-115 形状的 C 层开关在 dump 后的 `emacs` 上不可用** ✗ ⇒ 规则补充：**仪器验证一律在 `temacs` 上做** ✓（或先用 `egetenv` ✓ 走 Lisp 侧环境 ✓）。⑶ **修的方向（下一轮 ✓）**：对照 C 的 `main`/`init_environ`（`emacs.c`/`sysdep.c` ✓）看端口是否漏了"启动时重新读 `environ`"那一步 ✓（这正是 dump 后 C 层 `getenv` 应该仍然可用的原因 ✓）；同时用 **`temacs` + `FEMACS_CHILD_TRACE=1`** 抓"丢收割"那几次的路径 ✓✓（仪器在那里是可见的 ✓）——**这是本轮最重要的解锁** ✓。 ⑻ **更正 + 八次差分定案（4.451）**：⑴ **更正 4.450 的"C 层 `getenv` 在 dump 后失效"** ✗→✓：真因是 **`build/port/src/emacs` 是上一次 `dump` 留下的旧拷贝** ✓ —— `strings` 实测：`temacs`（23:53 ✓）里四个仪器标记都在 ✓，`emacs`（22:38 ✓）里**一个都没有** ✗ ⇒ 仪器静默与 `getenv` 无关 ✓（4.450 的两条实测面因此收窄：A/B 比较的是**两个不同构建** ✓，结论"两种二进制都有竞态"按 4.451 的 temacs 读数成立 ✓）。⇒ 规则补充：**改完源码要验证时，先 `xtask dump` 刷新 `emacs`，或直接在 `temacs` 上验证** ✓。⑵ **八次差分（同一 `temacs` ✓ 带 `FEMACS_CHILD_TRACE=42` ✓）**：命中时 `apo=1 wait=1 state=1 get=1 reap=1 notify=1` ✓；**丢失时 `apo=1 wait=1 state=1 get=1` 而 `reap=0 notify=0`** ✗ ⇒ **SIGCHLD 处理器跑了 ✓、`waitpid` 被调了 ✓、但没收到任何子进程状态** ✗ ⇒ 处理器那一趟没拿到状态 ⇒ **tick 没变** ⇒ 之后首部那段 `if (update_tick != process_tick)` 的 `status_notify` 永远不会被触发 ✗（这正是 C 注释里"若 SIGCHLD 来晚，timeout 是唯一通知途径"那条纪律的反面 ✓）⇒ 子进程永久停在 `run` ✓✓。⑶ **下一站（点名）**：给 `get_child_status` 那条仪器**加打印返回值与 `errno`** ✓（现在只打 child/options ✓）⇒ 判 `waitpid` 是回 **0（尚未就绪 ✗）** 还是 **-1（ECHILD/EINTR ✗）** ✓，据此定案并修（候选：pid/选项传错 ✓、信号与 `waitpid` 的次序 ✓、或按 C 在超时路径上补一次显式轮询 ✓）。⑷ 仪器默认关 ✓、`gate-load=0` ✓（15 例 ✓）、`extract-abi --check` **process/sysdep 均 OK** ✓。 ⑼ **返回值定案（4.452）**：给 `get_child_status` 加打印**返回值与 errno** 后（同一 `temacs` ✓ 六次 ✓）：命中（4 次 ✓）`child-get: child=30179 options=19` → `child-get-done: pid=30179 errno=20` ✓；**丢失（2 次 ✗）`child-get: child=30252 options=19` → `child-get-done: pid=0`** ✗ ⇒ `waitpid (WNOHANG)` 回 **0 = "尚无状态"** ✓（子进程**当时还没退出** ✓），而那次 `child-signal: sig=20`（Darwin 的 SIGCHLD ✓）、`child-state: … blocked=0` ✓、`child-catch: arming` 仅一次 ✓。⇒ 机制**精确化**：处理器在 SIGCHLD 上跑了 ✓、但唯一 alive 的子进程尚未可收割 ✗ ⇒ `changed=false` ⇒ **tick 不动** ⇒ 管道不写 ⇒ 再无唤醒 ⇒ 永久 `run` ✓。⑵ **本轮读码确认与 C 逐字一致**的六处：`emacs_sigaction_init`（屏蔽字 + `emacs_sigaction_flags` ✓）、`deliver_process_signal`（含主线程转发 ✓）、`get_child_status` ✓、`child_status_changed` ✓、`handle_child_signal` ✓、`status_notify` ✓ 以及 `wait_reading_process_output` 里那段 tick 门 ✓ ⇒ 剩下的差异必在"**谁先收割**"或"**SIGCHLD 被谁消费**"（合并投递 ✓ / 别处已收割 ✓）✓。⇒ 下一步仪器：⑴ 记录**每一次** `child-get`（带时间戳 ✓，含 `deleted_pid_list` 支路 ✓）；⑵ 在 tick 门那段加打印 ✓（证实在丢失运行里它**从不执行** ✓）；⑶ 追 `record_deleted_pid`/`process_status`/`Fcall_process` 等**其它收割路径** ✓，看那个子进程是否在**别处**被收割 ✗。 |
| P-123 | PORTING.md: P-122 clause 10 - two false leads cleared, and the misses come in two shapes  Both "missing line" candidates are present in the port: the loop head's else-if on pending_signals calling process_pending_signals, and Fprocess_status's if (raw_status_new) update_status - each byte-for-byte what the reference has. They looked missing only because a truncated grep (head -10) hid them, so the rule for the next rounds is to count matches rather than trust a head.  The misses themselves come in two shapes on the same one-line reproducer: in one the handler runs before the child is waitable (waitpid answers zero) and nothing retries; in the other the reap succeeds but the status the probe reads is still `run`, so the handler's update landed after the read. Both are the signal's processing being out of step with the wait loop, and the next instrument is a timestamped event order to compare a hit with a miss - handle_child_signal's entry, the tick comparison inside status_notify, and the probe's own status print - against the reference's ordering. |
| P-124 | **`child_status_changed` 的返回类型在镜像与定义之间不一致** ✗（根因，4.455/4.456）：C 的权威原型是 `syswait.h` **60** 行 `extern pid_t child_status_changed (pid_t, int *, int);` ✓（定义在 `sysdep.c` 516 ✓ 也返回 `pid_t` ✓），而端口的镜像把它声明成 **`-> bool`** ✗、定义却是 `-> pid_t` ✓ ⇒ **调用方按 `bool` 读**（Rust 的 `bool` 只保证 0/1 ✓，优化后等价于**只看最低位** ✗）⇒ **偶数 pid 被判成"状态未变"** ✗✓✓ —— Darwin 的 pid 常为 4 的倍数 ✓ ⇒ 丢失几乎总是偶数 pid ✓：实测命中 6 个 pid 全奇 ✓（31671/31687/31707/31719/31733/31749 ✓）、丢失 10 个 pid 全偶 ✗（30252/30272/31806/31834/31882/31922/31944/31968/33092/32970 ✓）⇒ 收割被跳过 ✓、tick 不动 ✓、`status_notify` 永不触发 ✓、进程永久 `run` ✓（P-122 的全链条由此闭合 ✓）。 | **规则，立即生效** | ① 镜像声明与单元定义的**返回类型/参数类型必须逐字一致** ✓（这一条现在有实证代价 ✗）；② C 的"非 bool 返回值放进 `if`"在 Rust 里一律显式写 `!= 0`（或 `> 0` ✓ 按 C 的语义 ✓）✓ —— 本次两处调用点（`deleted_pid_list` 支路 ✓、进程循环 ✓）按此改形 ✓；③ `extract-abi --check` 对 `process`/`sysdep` 均 **OK** ✓（该门只比 token/行数，不查类型 ✗ ⇒ 这条缺陷它看不见 ✗，所以 ① 必须靠人读 ✓）。修后验收：一行复现 **12/12 exit** ✓、三形状探针 **0 丢失**（fast/slow/advice 各 N=8 ✓）、`gate-load=0` ✓、`dump=0` ✓。 |
| P-125 | **诊断仪器自身的两条门约束此前没有被记下**（本轮各踩一次 ✗）：⑴ 仪器**不得引入参照没有的 libc 符号** ✗ —— 我用了 `sigismember`（C 里它是**宏** ✓，参照因此没有该符号 ✓），`gate-fast` 的 partial-link 检查立刻报 `_sigismember … referenced and defined by nothing` ✗ ⇒ 改成直接测位（本目标 `sigset_t` 是单个 32 位字 ✓：`SIGCHLD` = 20 ⇒ bit 19 ✓），不添符号 ✓；⑵ 仪器**不得重复镜像里已有的声明** ✗ —— 我在 `process.rs` 又写了一份 `pthread_sigmask` 声明（`lib/include/signal.rs` 已在包含图里 ✓），mirror-dupes 门报 `already declared … (in the include graph)` ✗ ⇒ 删掉重复声明 ✓。 | **规则，立即生效** | ① 给仪器加新调用前，先在 `emacs/lib` / `emacs/src` 里 `grep` 一次：参照若没有该符号（宏 ✓ 或不在导出表 ✓），就不要调它 ✓（改用手工位运算 ✓ 或既有封装 ✓）；② 声明新名字前，先确认包含图里没有同名声明 ✓（`mirror-dupes` 门会红 ✓）；③ 仪器改动后**必须**跑一次完整 `gate-fast` ✓（本轮因此连跑 4 次 ✓ 才拿到全绿 ✓ —— 代价记在轮次账上 ✓）；④ 若某工具**故意**含产品名（如 `gen-finder.py` 比对参照文本时 ✓），走 `tools/check-declarations.py --write-allowlist` 登记 ✓（本轮新增 1 行 ✓，24 行 ✓ 0 failures ✓）。 |
| P-126 | **P-122 的仪器自己就是 `package-vc-tests` “内存失控”的真凶（四轮误诊的收口 ✓）**：端口在 `handle_child_signal` 里用 `getenv` 查开关 ✓，而 `getenv` **不是 async-signal-safe**（本目标上它取 `os_unfair_lock` ✓）⇒ SIGCHLD 处理器跑起来时，主线程若正停在 `sysdep::get_child_status` 的同型仪器里（同一个 `getenv` ✓，处理器又经 `child_status_changed` 调它 ✓），libplatform 报 “BUG IN CLIENT OF LIBPLATFORM: Trying to recursively lock an os_unfair_lock” 并以 **SIGKILL** 中止整个进程 ✗（macOS 崩溃报告 `emacs-*.ips`：`EXC_BREAKPOINT … signal SIGKILL`，栈 `_os_unfair_lock_recursive_abort ← getenv ← handle_child_signal ← deliver_process_signal ← _sigtramp ← getenv ← get_child_status ← wait_for_termination ← call_process` ✓；lldb 独立抓到同一栈 ✓）。发作率约 50%（取决于信号落在哪条指令 ✓）、RSS 只有 40-70 MB ✓ ⇒ 与内存无关 ✗。而 `tools/check-ert.py` 把“上限内被 SIGKILL 且看门狗没跳”也读成 **out-of-memory** ✗ ⇒ 4.459-4.464 四轮都在追一个不存在的内存失控 ✗。最小复现同时从“整片”收窄到“同一进程里的第二个包安装”（单测 `…install-post-conditions/test-package-one` 通过 ✓、再加第二个测试即崩 ✗ ✓）。 | **规则，立即生效** | ① **信号处理器里不许调用非 async-signal-safe 函数**（`getenv`/`malloc`/`fprintf` 家族 ✓）：诊断开关一律在启动期**采样一次**（本轮：`init_process_emacs` 与 `syms_of_sysdep` 各采一次 ✓）存成普通标志位 ✓，处理器只读标志位 ✓；② 开关语义不变（P-115：默认关 ✓ 一行 stderr ✓），但**必须在启动前设好** ✓；③ `tools/check-ert.py` 的判定：只有**看门狗跳了**才算 `out-of-memory` ✓；`-9` 且看门狗静默按 `crashed` 报并注明 `SIGKILL, watchdog silent` ✓（这条误标是四轮误诊的另一半原因 ✗）；④ 验收（4.465 实测 ✓）：`package-vc-tests` 整片 **same=1** ✓（端口 45 例 / 参照 45 例 / 两侧各 0 unexpected ✓）、两测复现 **4/4 exit 0** ✓（峰值 70 MB ✓）、`gate-load=0` ✓、`dump=0` ✓（13,025,288 B ✓）、`extract-abi --check` 的 `process`/`sysdep` 均 **OK** ✓；⑤ 复核既有结论：P-124 那条证据（6 个奇 pid 命中 / 10 个偶 pid 丢失 ✓）出自**同一批被撞死的运行** ✓ ⇒ 修法保留（镜像声明与定义逐字一致本身是对的 ✓），但“丢失 10 例”的读数应在本次修法之后重取 ✓。 |
| P-127 | **C 的混型 `min (PTRDIFF_MAX, SIZE_MAX)` 被端口降型成了 -1 ⇒ 合法的向量增长被判成“内存耗尽”**：`find-func-tests` 与 `map-ynp-tests` 在端口上 **exit 1 且无输出** ✗，而参照通过 ✓。最小复现收窄到“**同一进程里连续两次 `ert-simulate-keys` + `read-library-name`**”（第二次即死 ✓；单次通过 ✓），死亡信息是 `error "Memory exhausted--use C-x s then exit and restart Emacs"` ✓（`startup.el` 改写过的 `memory-signal-data` ✓）而进程 RSS 只有 **2 MB** ⇒ 不是真的内存不足 ✗。lldb 抓栈定案 ✓：`memory_full ← memory_full_up ← larger_vector ← keyboard::add_command_key ← keyboard::read_char` ✓ —— 第二次 read 把 `this_command_keys` 撑满、调 `larger_vector (vec, 1, -1)` ✓。读码：C 的 `ptrdiff_t C_language_max = min (PTRDIFF_MAX, SIZE_MAX) / sizeof *v->contents;` ✓（`fns.c` 4667）里，`min` 是**三元表达式** ✓ ⇒ 公共类型是 **`size_t`** ✓ ⇒ C 的值是 **PTRDIFF_MAX**（≈1.15e18）✓；端口写成 `min (PTRDIFF_MAX, SIZE_MAX as ptrdiff_t)` ✗ ⇒ SIZE_MAX 先降型成 **-1** ✗ ⇒ `C_language_max = -1/sizeof = 0` ✗ ⇒ `n_max = 0` ⇒ `incr_max = 0 - old_size < incr` ✗ ⇒ `memory_full_up ()` ✗（C 走的才是 `allocate_vector` ✓）。**同一模式全树 8 处** ✓（`fns.rs` `larger_vector` ✗、`term.rs` `set-terminal-…-size` 的 `RANGED_FIXNUMP` 上界 ✗、`ftfont.rs` 特性长度界 ✗、`dispnew.rs` `row_table_max` ✗ 与 “screen size too big” 界 ✗、`buffer.rs` 的 `OVERLAY_COUNT_MAX` 宏 ✗、`callint.rs` ✗、`emacs.rs` 的 `emacs_re_safe_alloca` ✗），其中 dispnew 那两处会给出 **负的**上界 / 对任何正 `area` 都走 `fatal` ✗（批量路径不经过，故此前没暴露 ✓）。 | **规则，立即生效** | ① C 的 `min`/`max` **混型**调用一律先判公共类型：本条是 `size_t` ✓ ⇒ 端口一律在 **usize 域**里比（`min (PTRDIFF_MAX as usize, SIZE_MAX)` ✓），再在 C 的**赋值点**降型 ✓；② **禁止 `SIZE_MAX as isize` / `as ptrdiff_t` 这类降型** ✗（它是本条的成因 ✓；全树 grep 见 ③）；③ 建议下一步加一条 grep 型门（`tools/` 里查 `SIZE_MAX as (isize\|ptrdiff_t)` ✓）并接进 `gate-fast` ✓。④ 验收（4.466 实测 ✓）：最小复现 `v1="org/org" v2=121 END` ✓ exit 0 ✓（修前 exit 255 且报内存耗尽 ✗）、`map-ynp-tests` **same=1** ✓、`find-func-tests` **same=1** ✓（修前两者均 `crashed exit 1` ✗）、`lisp/emacs-lisp` 整片 68 文件 **same 57→62** ✓、**port-worse 8→6** ✓（`crm`/`rmc`/`track-changes` 三个的 unexpected 归零 ✓）、**crashed 3→1** ✓（仅 `edebug-tests` 仍 180 s 超时 ✗）、`gate-load=0` ✓（15 例）、`dump=0` ✓（13,025,288 B ✓）、`extract-abi --check` 七个文件全 **OK** ✓、`gate-fast` **exit=0** ✓。 |
| P-128 | **`sizeof` 家族的第三个成员：`size_of_val` 量的是 `&str` 的字节数，不是 C 的“含 NUL”大小** —— `uuid-tests` 6 例失败 ✗，签名是 `uuid-to-string` 的最后一组十六进制**前 4 位变 0**（`…-9bac-f847db4148a8` ⇒ `…-9bac-0000db4148a8` ✓）。最小复现：`(format "%012x" #x2665a8aea6a2)` 端口答 `0000a8aea6a2` ✗、参照答 `2665a8aea6a2` ✓（同一值的 `%S` 两端一致 ✓ ⇒ 值没错，是**格式化** ✓）。根因：C 的 `editfns.c` 3849 `enum { pMlen = sizeof PRIdMAX - 2 }`（`sizeof` 含 NUL ⇒ `pMlen = len - 1 = 1` ✓），端口写 `std::mem::size_of_val (PRIdMAX) as ptrdiff_t - 2` ✗ —— 对 `&str` 而言 `size_of_val` 是**字节长度**（2）⇒ `pMlen = 0` ✗ ⇒ 规格里丢了 `j` 长度修饰符 ⇒ `%x` 只读 32 位而 `uintmax_t` 传了 64 位 ✗（`doprnt.rs` 179 的同名常量写的是正确形状 `PRIdMAX.len () as isize - 1` ✓，本轮按它统一 ✓）。| **规则，立即生效** | ① `&str`/`&[u8]` 上**不许**用 `size_of_val` 复现 C 的 `sizeof`（C 的字符串常量含 NUL，Rust 的不含）✓；一律写 `len () - 1`（要“去掉最后一个字符”时 ✓）或 `len () + 1`（要含 NUL 时 ✓），并在注释里点出这条换算 ✓。② 验收（4.467 ✓）：`(format "%012x" #x2665a8aea6a2)` 与参照一致 ✓；`uuid-tests` 6 → 0 unexpected ✓（16 例 ✓）。 |
| P-129 | **C 的后自减条件被逐字翻译成“先比后减”，多走一帧** —— `backtrace-tests` **9/9 全挂** ✗ 且 `bytecomp--byte-op-error-backtrace` 挂 ✗（共 10 例），签名统一：端口打出的回溯**少最内层那一帧** ✓（参照 `Test header / backtrace-get-frames() / (setq backtrace-frames …)` ✓，端口直接从 `(setq …)` 开始 ✗）。根因：C 的 `eval.c` 3994 `while (backtrace_p (pdl) && offset-- > 0)` ✓（测试用**旧值** ⇒ `offset == 0` 一帧都不走 ✓），端口写 `while backtrace_p (pdl) && offset >= 0 { pdl = backtrace_next (pdl); offset -= 1; }` ✗ ⇒ `offset == 0` 仍前进一帧 ⇒ `mapbacktrace` 的起点整体上移一帧 ✗。| **规则，立即生效** | ① C 的 `x-- > 0` / `x++ < n` 一律翻成“先取旧值、再自增/自减”的块（本轮用的形状：`while cond && { let o = x; x -= 1; o > 0 }` ✓），**不许**改写成 `x >= 0` 之类的等价直觉 ✗（这条与 P-127 是同一族：都是把 C 的隐式语义“顺手简化” ✗）。② 验收（4.467 ✓）：`backtrace-tests` 9 例全过 ✓（same=1 ✓）、`bytecomp-tests` 106 例全过 ✓（same=1 ✓）。 |
| P-130 | **四份哈希的实现都把 `(char *) ctx->buffer` 的字节偏移写成了字偏移** —— 所有摘要对“长度不是 64 倍数”的输入都是错的 ✗，而空串、64 字节输入却是对的 ✓✓（这条“0 与 64 对、其余错”的指纹指向**残留字节**那段 ✓）。证据：`(secure-hash 'md5 "abc")` 端口 `03c604ef…` ✗ / 参照 `90015098…` ✓、64 个 `x` 两端一致 ✓、100 个 `x` 又错 ✗；`uuid-v5-generator` 因此算出完全不同的 UUID ✗（RFC 9562 A.4 的向量 ✓）。根因：C 的四份源码都写 `memcpy (&((char *) ctx->buffer)[left_over], …)` ✓（**先转 `char *` 再按字节偏移** ✓），端口写成 `(*ctx).buffer.as_mut_ptr ().add (left_over)` ✗ —— `buffer` 是 `uint32_t[32]`/`u64[32]` ⇒ `.add` 走**字**偏移（4× 或 8× 偏）✗。四份文件各 4 处（`finish_ctx` 的填充起点 ✓、`process_bytes` 的首合并 ✓/尾合并 ✓/搬移源 ✓），共 **16 处** ✓ 一并修。注意**不是**全部 `.add` 都要改：C 里按**字**下标的地方（`ctx->buffer[size - 2]`、`&ctx->buffer[16]` ✓）在端口就是字偏移 ✓，保持原样 ✓。| **规则，立即生效** | ① 遇到 C 的 `&((char *) X)[n]`，端口必须先降型成字节指针再 `.add (n)` ✗→✓；同一条表达式里 C 同时出现“字下标”和“字节偏移”是常态 ✓，逐处对照、不要整段统一 ✓。② 验收（4.467 ✓）：`md5`/`sha1`/`sha512` 在 `"abc"` 与 100 个 `x` 上与参照逐字一致 ✓、`uuid-tests` **same=1** ✓（16 例 / 0 unexpected ✓）；**遗留**：`sha256` 仍全长度错 ✗（连 n=0 都错 ⇒ 不在本条，点名在 `sha256_process_block` ✓）。 |
| P-131 | **`SS1` 的第三项是旋转，不是右移** —— `sha256`/`sha224` 的摘要**在任何长度上都错** ✗（连 n=0 都错 ⇒ 不在 P-130 那条“残留字节”里 ✓），而 `md5`/`sha1`/`sha512` 修完 P-130 后与参照逐字一致 ✓，正好把范围锁在 sha256 一份 ✓。根因：C 的 `sha256.c` 323 `#define SS1(x) (rol(x,26)^rol(x,21)^rol(x,7))` ✓，端口 `femacs/lib/sha256.rs` 写成 `rol(x,26) ^ rol(x,21) ^ ((x) >> 7)` ✗ —— 第三项是 `>> 7` 而不是 `rol(x,7)` ✗（同族的 `S0`/`S1`/`SS0` 三个都抄对了 ✓，只有这一个漏了 ✓）。| **规则，立即生效** | ① 逐条核对 C 的 `rol(…,n)` 与 `>> n`：**同一行里两者常同时出现** ✓（`S0` 就是 `rol,rol,>>` ✓），照抄、不要按“看起来重复”合并 ✗。② 验收（4.468 ✓）：`sha256`/`sha224` 在 `"abc"`、n=0/1/55/56/63/64/65/119/120/128/1000 共 13 例上与参照**逐行一致** ✓（`diff` 为空 ✓）。 |
| P-132 | **三份单元把 `FUNCTIONP` 重新定义成了“只看符号”的本地宏** —— `generator-tests` 4 例失败 ✗（`iter-close` 路径上 `(make-finalizer <closure>)` 报 `wrong-type-argument functionp` ✓），而 `(functionp <同一个闭包>)` 两端都答 `t` ✓ ⇒ 差别只在这一份实现上 ✓。最小复现：`(make-finalizer (lambda () x))`（`lexical-binding` 下是闭包 ✓）⇒ 端口 `(wrong-type-argument functionp #[nil (x) ((x . 1))])` ✗ / 参照 `#<finalizer>` ✓。根因：端口的 `alloc.rs`/`keyboard.rs`/`window.rs` 各有一份 `macro_rules! FUNCTIONP { ($x:expr) => { SYMBOLP ($x) && !NILP (Ffboundp ($x)) } }` ✗，注释称“lisp.h 3772 有这样一条宏” ✗ —— **本修订的 lisp.h 里没有**（`FUNCTIONP` 只有 4799 的函数声明 ✓、定义在 `eval.c` 3103 ✓，它接受闭包/子例程/字节码 ✓）。| **规则，立即生效** | ① 本地“简化替身”在**语义上与 C 的实体不同**时不许留 ✗；本轮改为删掉三份宏（共 8 处调用点 ✓）并让它们调用真函数 ✓。**第一版修法被门否掉（本轮实测 ✗）**：先把声明加进 `src/include/lisp.rs` ⇒ `mirror-dupes` 门立刻报 **12 个单元**“already declared … (in the include graph)” ✗ ⇒ 该门要求“**镜像与单元二选一**” ✓；本树既有 12 个单元各自声明它 ✓ ⇒ 按既有形状在 `alloc.rs`/`window.rs` 各补一份本地声明 ✓（`keyboard.rs` 已有 ✓），镜像保持不动 ✓ ⇒ `gate-fast exit=0` ✓。② 注释里引用 C 行号时必须**当场核对**（这条错误的注释引文是当初“简化”的理由 ✓）；发现引文不存在时，连同注释一起改 ✓。③ 验收（4.468 ✓）：`make-finalizer` 探针与参照一致 ✓；`generator-tests` **same=1**（92 例 ✓）；`alloc`/`keyboard`/`window` 的 `extract-abi --check` 仍 **OK** ✓。 |
| P-133 | **生成物语料的“逐字节一致”目前只差 docstring 一层** —— 按 DoD-4 跑 `tools/gen-unidata.py` ✓（先问参照构建会跑什么 ✓、再用**端口自己的**程序跑同一条配方 ✓）：23 个文件全部重新生成 ✓，其中 20 个只差**已登记的产品名改写** ✓，另外若干条的差异集中在**函数 docstring 与 `(fn CHAR VAL TABLE)` 形参行**（如 `uni-decomposition.el` ✓）⇒ 指向端口的 `etc/DOC` 仍缺 170 条死区记录（P-105 的遗留 ✓）而不是生成器本身 ✗。**但**：重生成后 `shortdoc-tests` 由 2 unexpected ⇒ **same=1** ✓（5 例 ✓）⇒ 片内那 2 例正是**陈旧生成物**造成的 ✓。| **规则，立即生效** | ① 生成物族的验收分两步：先“本树自洽”（测试 ✓，本轮做到 ✓），再“与参照逐字节”（待 DOC 补齐后 ✓）；两步的读数要分开写 ✓，不许把前者当后者 ✗。② `uni-*.el` 等生成物**不入库**（与参照一致 ✓，`git status` 不因重生成而变 ✓）。③ 验收（4.468 ✓）：`lisp/emacs-lisp` 片内 **67/68 same** ✓（仅 `edebug-tests` 180 s 超时 ✗）。 |
| P-134 | **C 的 `for (…; …; incr)` 的增量表达式被漏掉 ⇒ 死循环** —— `edebug-tests` 整文件在端口上**挂死** ✗（`sample` 的热栈 1523 个样本全在 `Fcall_interactively` 里 ✓、最深处在 `_platform_memchr` 341 个样本 ✓），参照同文件 **0.3 秒**跑完 ✓。**单测即复现** ✓（`edebug-tests-clear-breakpoint` 单独跑 > 20 秒 ✗ / 修后 1 例 same ✓），所以不是状态相关 ✓。根因：C 的 `callint.c` 416 `for (char const *tem = string; tem < string_end; tem++) { … tem = memchr (tem, '\n', …); if (!tem) break; }` ✓ 里，`tem++` 是**循环头的增量** ✓；端口写成 `while tem < string_end { … tem = memchr (…); if tem.is_null () { break; } }` ✗ **漏掉了 `tem++`** ✗ ⇒ `memchr` 每次都返回**同一个 `\n`** ✓ ⇒ 无限循环 ✓（只有当交互规格里含 `\n` 时才发作 ✓ ⇒ 大多数交互调用照常通过 ✓，这也是它躲过前面几十轮的原因 ✓）。| **规则，立即生效** | ① C 的 `for (init; cond; incr)` 翻成 Rust 时，`incr` **必须**落在循环体末尾（本轮形状：`tem = tem.offset (1);` ✓ 并就地注明它来自 C 的增量表达式 ✓）；本文件另两处同形循环（`i++` 在 451/759 ✓）此前是对的 ✓，对照时**逐头核过** ✓。② 与 P-127/P-129/P-130 同族 ✓：**C 的隐式/位置性语义（类型提升 ✓、后自减 ✓、字节偏移 ✓、循环增量 ✓）是这一类缺陷的公共来源** ✓ ⇒ 每处 `for` 都要问“增量去哪了” ✓。③ 验收（4.470 ✓）：`edebug-tests-clear-breakpoint` **same=1** ✓（>20 秒挂死 → 1 例通过 ✓）、`edebug-tests` 整文件 **same=1** ✓（49 例 / 两侧各 0 unexpected ✓）、`lisp/emacs-lisp` 整片 **68/68 same** ✓、`gate-load=0` ✓（15 例）、`dump=0` ✓（13,025,288 B ✓）、`extract-abi --check` `callint` **OK** ✓。 |
| P-135 | **两条测试失败其实是“参照树自己是 git 仓库、端口树不是”** —— `project-tests` 2 例 ✗（`(file-relative-name FILE (project-root …))` 期望以 `test/` 开头 ✓，端口答 `femacs/test/…` ✗）与 `project-try-vc-invalidates-cache` 的 `file-equal-p "/…/femacs/" "/…/femacs/femacs/"` ✗。读数：`ls -d emacs/.git` **存在** ✓、`git -C emacs rev-parse --show-toplevel` = `…/femacs/emacs` ✓ ⇒ 参照检出**本身就是仓库根** ✓（它的测试因此把项目根算成 `emacs/` ✓）；端口树在仓库里 ✓ ⇒ 项目根是仓库根 ✗ ⇒ 相对名多一层 ✗。| **规则，立即生效** | ① **对比测试的运行环境必须与参照等价**：凡测试读“项目根/仓库根/VCS 后端”的 ✓，两侧的树都必须是各自的项目根 ✓。② 实现：`tools/check-ert.py` 启动时确保 `femacs/.git` 是**指向外层仓库的 gitfile**（`gitdir: ../.git` ✓，被外层 git 忽略 ⇒ 工作树仍干净 ✓，实测 `git status --short` 只有本次有意的改动 ✓）；已实测 `project-tests` **2 → 0**（`same=1` ✓ 15 例 ✓）。③ 反面教训：这类差异**不是**端口的 C→Rust 缺陷 ✓，不许为它改 `femacs/**` 的 Rust 代码 ✗。 |
| P-136 | **C 的混型比较被“先降型再比”反转，且会让取负溢出 panic**（P-127 家族的第 4 个成员 ✓）—— `dbus-tests` 在端口上 **SIGABRT（exit 134）** ✗、参照 ok ✓，自带回溯直接点名：`panic: attempt to negate with overflow` ✓ @ `lread.rs` 的 `string_to_number` ✓（`string_to_number ← read_integer ← read0` ✓）。根因：C `lread.c` 4615 `if (-MOST_NEGATIVE_FIXNUM < n) return make_neg_biguint (n);` ✓ 里 `n` 是 **`uintmax_t`** ✓ ⇒ 比较在**无符号域** ✓（`-MOST_NEGATIVE_FIXNUM` = 2^61 ✓）；端口写 `-(MOST_NEGATIVE_FIXNUM as i64) < n as i64` ✗ ⇒ `n ≥ 2^63` 时降型成**负数** ✗ ⇒ 比较结论反转 ✗ 且随后 `-signed_n` 在 `n = 2^63` 上**溢出 panic** ✗。| **规则，立即生效** | ① C 的 `signed < unsigned` 比较必须**在无符号域**复现（本轮形状：`((-(MOST_NEGATIVE_FIXNUM as i64)) as u64) < n` ✓），**禁止**把无符号侧降型成有符号再比 ✗ —— 这是 P-127（`min` 的公共类型 ✓）的同族。② 验收（4.474 ✓）：最小复现 `(read "-9223372036854775808")` 两侧一致 ✓（修前端口 panic ✓）；`dbus-tests` **crashed → same=1** ✓（18 例 / 两侧各 0 unexpected ✓）；`extract-abi --check lread` **OK** ✓；`gate-load=0` ✓、`dump=0` ✓（13,025,288 B ✓）。 |
| P-137 | **`--glob` 是“相对测试根 + 单层”的，子目录里的测试文件会被静默漏掉** ✗ —— 本轮按目录扫 `lisp/textmodes/*.el` 只跑到 15 个文件 ✓，而该目录的测试文件是 **20** 个 ✓：另外 5 个在 **子目录** `lisp/textmodes/ispell-tests/` 里 ✓（`find` 计数 19 vs 15 是最早的线索 ✓）。这类文件**从未被跑过** ✗，比“跳过某个测试”更隐蔽 ✗。| **规则，立即生效** | ① 每一片的文件清单必须来自**枚举**（`find <root> -name '*tests.el'` ✓ 或 `**` 递归 glob ✓），并**先核对总数**（例如 `lisp/textmodes` = 20 ✓）再跑 ✓；② 目录级汇总要报“已跑文件数 / 枚举文件数” ✓，差额不为 0 就不算跑完 ✓；③ 实测（4.475）：`lisp/textmodes/ispell-tests/*.el` **same=5** ✓（79 例 ✓）⇒ 这一片补齐后 `textmodes` = **20 文件 / same=18 / port-worse=2** ✓。 |
| P-138 | **L5 的分母是参照自己的 `ELFILES` 配方（**564** 个文件），不是“`*tests.el` 的数量”** —— 本轮把两者对齐时发现：`test/Makefile.in` 206-210 的 `ELFILES := $(sort $(shell find ${srcdir} -name manual -prune -o -name data -prune -o -name infra -prune -o -name "*resources" -prune -o … -name "*.el" ! -name ".*" -print))` ✓ 才是权威口径 ✓ ⇒ **564** 个文件 ✓（`lisp/` 510 ✓、**`src/` 49** ✓、`misc/` 4 ✓、`lib-src/` 1 ✓），且 `SLOW_TESTS`（`package-vc-tests`、`tramp-tests` ✓）被排到最前 ✓。此前我按 `*tests.el` 数出来的 438 / “496” 都是**别的口径** ✗，其中 `src/` 的 49 个（测 C 层的 Lisp 测试 ✓）从未跑过 ✗。| **规则，立即生效** | ① 覆盖率的分母**只能**来自这条配方 ✓；`tools/check-ert.py` 的默认 `--glob` 已改为**按同一配方枚举** ✓（新增 `--list` 只打印清单、不启 Emacs ✓），实测 `--list` 与 `find` 的清单**逐行相同** ✓（564 == 564 ✓），运行时打印 `enumerated N file(s)` ✓；② 每次汇报覆盖率必须写“**已跑 / 564**” ✓，并列出分目录的部分覆盖（如 `erc` 24/75 ✗）✓，不许用子集口径冒充分母 ✗。 |
| P-139 | **`forward-comment -1` 在“重叠的 2 字符注释标记”上有行为分歧（最小复现 5 个字符）** —— 端口 `syntax-tests` 3 例失败 ✗（`syntax-comments-c-b6`、`syntax-comments-lisp-n-b105`、`-b107` ✓），本轮把夹具换成自造小文本做差分，拿到**最小复现**：C 风格表（`/`=`. 124b`、`*`=`. 23`、`\n`=`> b` ✓）下文本 `"/*/*/"` ✓，从 `point-max` 调 `(forward-comment -1)` ⇒ 参照 **t** ✓（点回到 1 ✓）、端口 **nil** ✗（不动 ✓）。这正是 C 自己在 `back_comment` 上方点名的那族：“Nasty cases with overlapping 2-char comment markers … `|*|` in C” ✓ ⇒ 分歧在 `back_comment` 的**重叠标记分支**（或它调用的 `char_quoted`/`prev_char_comend_first`/`dec_both`/`UPDATE_SYNTAX_TABLE_BACKWARD` ✓）里 ✓。**注意**：`extract-abi --check` 对 `syntax.c` 读 **OK** ✓ ⇒ 该缺陷**不可能**被 token 级或行数级门抓到 ✗（和 P-124/P-127 一样 ✓）。| **规则，立即生效** | ① 这类“token 全同、语义不同”的缺陷只能靠**行为差分**发现 ✓ ⇒ 最小复现优先用**自造小文本**（而不是上游夹具 ✓，本轮 5 字符就够了 ✓）；② 定位方法：从最小复现出发，逐字符变体（加/减一个字符、换表 ✓）做两侧差分，直到找出触发分支 ✓；③ 下一轮在 `back_comment` 里按该分支补**带两侧读数**的探针（`char_quoted`/`prev_char_comend_first`/`dec_both` 的入口/出口 ✓），必要时要与 C 的**运算符优先级与求值顺序**逐条对齐 ✓。 |
| P-140 | **仪器本身改变了被观察的分支走向 ⇒ 一条读数被作废**（P-119“advice 必须先证可达”的推广 ✓）—— 给 `back_comment` 加的重叠检查打印里，我把 `prev_char_comend_first (from, from_byte)` 直接写进 `fprintf` 的实参 ✗，而它内部会 `UPDATE_SYNTAX_TABLE_BACKWARD (from)`（**有副作用** ✗）⇒ 打印改变了语法表位置 ✓ ⇒ 那次运行“走到 LOSSAGE”✗ 是**仪器制造的** ✓：撤掉该打印后，**同一份源码**在同一条命令下不再进入 lossage ✓（只剩一次 `bc: from=3 …` ⇒ 循环在 `from=2` 处从**重叠检查** break，`comstart_pos` 保持 0 ⇒ 走 `if (comstart_pos == 0) { from = comment_end; }` ⇒ 返回 nil ✓）。⇒ 4.478 ③ 的“分歧在 lossage 恢复段”**作废** ✗。| **规则，立即生效** | ① 打印**只读已算好的局部变量** ✓；要观察某个条件，先把它**赋值给局部**（不改变求值次数/顺序 ✓）再打印 ✓；② **不许**把函数调用写进打印实参 ✗（尤其是 `syntax.c`/`*_table`/`mark_*` 一族 ✓）；③ 加完仪器必须验证两件事：**关掉开关时行为与改动前一致** ✓（`gate-load`/`dump` 复跑 ✓）与**开着开关时缺陷仍复现** ✓；两者不同即说明仪器在扰动 ✗，先修仪器再读数 ✓。 |
| P-141 | **单元构建档位从 `-C opt-level=0` 冻结到 `-O2`；P-101 ② 的 64 MB 栈链接标志一并撤掉（那笔债的收据已兑）** —— P-101 记录过端口“还不具备被优化”（`=1` 崩在第一条 `Loading` 之前 ✗、`=2` 在 `styled_format` 的 `memmove` 读未映射地址 ✗），并因此把单元冻在 `-O0`、程序改挂 `-Wl,-stack_size,0x4000000` ✓。后续那些**真类型/别名缺陷**（P-124 镜像返回类型 ✗、P-127 混型 `min` ✗、P-130 字节/字偏移 ✗、P-132 替身宏 ✗、P-136 无符号比较 ✗）修掉之后重测（4.486）：`-O1` 与 `-O2` **各 83 units / 0 failure** ✓、`gate-load` **15 例 0 failure** ✓、`dump` 成功 ✓，且**在撤掉 64 MB 栈标志的条件下**通过 ✓ ⇒ 帧尺寸债也随之消失（P-116/P-117 的“解释器帧是 C 的 14 倍” ✓）✓。基准（同机 ✓）：解释器 1e7 `dotimes` **5.39 s → 1.26 s** ✓、`string-search` 2 MB **0.0155 → 0.0037** ✓、`sort` **0.0166 → 0.0037**（比参照 0.0047 更快 ✓）、`length` **0.0115 → 0.0026**（比参照 0.0038 更快 ✓）；**正则仍 12.5×** ✗（`a*b*c*d*e` × 20000 个 `a`：6.94 s 对 0.554 s ✓，另见 P-139 的 `regexp-tests-backtrack-optimization` ✗）⇒ 独立缺陷 ✓。| **规则，立即生效** | ① 默认档位是 `-O2` ✓（`UNIT_CODEGEN_FLAGS` ✓），`overflow-checks`/`debug-assertions` 继续显式打开 ✓（P-96 ✓ 与检查版构建 ✓）；`FEMACS_OPT_LEVEL` **保留但降级为诊断覆盖** ✓（供日后二分/压测 ✓，不再代表“端口只能跑 `-O0`” ✓）。② **栈标志已删** ✓：`gate-load` 与 `dump` 都在 8 MB 默认栈上通过 ✓；若日后又在 `loadup` 上见到 C 栈溢出 ✗，先查是不是**新的**帧尺寸回归 ✓，不要条件反射地把它加回来 ✓。③ 连带 **链接棘轮**更新 ✓：`docs/baseline/link-pending.txt` 由 `tools/check-link.py --write-pending` 重写 ⇒ **+`_memset_pattern16`**（优化器生成的常量块填充 ✓，`libc-or-platform` 类 ✓）、**−5 行 stale**（`_fputs`/`_strcat`/`_trunc`/`_fabs`/`__…panic_fmt` 被内联后不再引用 ✓）⇒ 这类“档位改动的连带”必须**在同一笔**里说明 ✓。④ 本轮 `gate-fast` 还抓到一处**我自己**的违规 ✓：`tools/check-ert.py` 的 `--list` 帮助文本里写了产品名 ✗（P-16/0.1-19 ✓）⇒ 工具/探针/CI 散文一律写 “C” 或 “the reference” ✓。⑤ 读出物要跟着改口径 ✓：DoD-7 的基准与 L5 的超时读数都以 `-O2` 为准 ✓（4.486 已见 `ffap-tests` **超时 → same** ✓）。 |
| P-142 | **`on_failure_jump_smart` 把它自己那条 `eassert` 里的表达式当实参传给了 `mutually_exclusive_p` ⇒ 贪心 `*` 永远退化成慢循环（`regexp-tests-backtrack-optimization` 报 “Stack overflow in regexp matcher” ✗，正则基准 12.5× ✗）** —— C 4880-4895 的形状：`p2` 是**循环出口**（= “repeat 之后是什么” ✓，也是 smart 的 jump 目的地 ✓），`p2[-3]` 才是循环尾那条 `jump` ✓；C 的调用是 `mutually_exclusive_p (bufp, p1, p2)` ✓，只有它上面那条 `eassert ((re_opcode_t) p2[-3] == jump && p2 + mcnt == p)` 用 `p2 - 3` ✓。端口把**断言里的表达式**直接抄进了调用（`femacs/src/regex-emacs.rs` 5494：`mutually_exclusive_p (bufp, p1, p2.offset (-3))` ✗）⇒ 分析从**循环尾的 `jump`** 起步 ⇒ 它往回跳到循环体 ⇒ 回调把“跟随其后”认成**循环体自己**（`exactn 'x'` 对 `exactn 'x'`）⇒ 恒“不互斥” ✗ ⇒ 每个字符都 `PUSH_FAILURE_POINT` ✗ ⇒ 失败栈的上限（`emacs_re_max_failures` 40000 × `TYPICAL_FAILURE_SIZE` 20 槽 ÷ 3 槽/点 ≈ 26.6 万个失败点）在第 26.6 万次推送处耗尽 ⇒ `re_match_2` 返回 **-2** ⇒ `search.c` 的 `matcher_overflow` 抛 “Stack overflow in regexp matcher” ✗。修法是一个实参级还原：调用改回 `p2` ✓（`p2 as *const re_char` ✓）。读数：`src/regex-emacs-tests.el` **port-worse 1 unexpected → same ✓**（两侧各 35 例 ✓）；基准 `a*b*c*d*e` × 20000 个 `a`：**6.94 s → 0.925 s**（参照 0.574 s ⇒ **12.5× → 1.61×** ✓）、`x*=*` × 200k：**0.902 ms**（参照 0.567 ms ✓）。 | **规则，立即生效** | ① **C 的 `eassert` 与紧随其后的实参不是一回事** ✓：抄“形如 `p2 - 3`”的表达式之前先读**被调用者的形参语义**（此处 `forall_firstchar (p2 …)` 的起点是“循环出口起的控制流” ✓）；断言里的偏移是**校验形状**，调用里的才是**分析起点** ✓。② **见到失败栈耗尽（返回值 -2 ⇒ “Stack overflow in regexp matcher”）先数“推送次数”，不要先怀疑失败栈本身** ✓：C 里 `-2` 只有“失败点真的太多”一条路 ✓，而“循环退化”是它最常见的成因（本处 1e6 个字符 = 1e6 次推送 ✗）。③ **贪心 `*`/`+` 的编译器形状与匹配期自改字节码**（`on_failure_jump_smart` → `on_failure_keep_string_jump` + 把**循环尾的 jump 目的地**改到循环体之后，好让那个失败点**只推一次** ✓）是一处**必须逐字节对齐的耦合**：改它的任何一半都要跑行为级读数 ✓。④ **性能与正确性在本处是同一笔账** ✓（同一处退化同时给出 12.5× 与 Stack overflow ✗），但与 P-141 的 `-O2` 无关 ✓。⑤ **这类缺陷静态读不出来**：`extract-abi --check`、布局断言、符号表、subr 表全绿却错 ✗ ⇒ **L5 的差分扫是它唯一的有效仪器** ✓（本处的每一条证据都来自 L5 ✓）。⑥ **操作事实（本次踩到 ✗）**：`xtask gate-load`/`build-tu` 重链的是 `build/port/src/temacs`，而 `tools/check-ert.py` 的默认程序是 `build/port/src/emacs`（由 `xtask dump` 用 `cp -f` 刷新 ✓）⇒ **重链后不跑 `dump`，L5 读到的还是旧程序** ✗（本次第一遍读数因此仍是旧的 port-worse ✗）。⇒ L5 读数前先比 `emacs` 与最后一次 `gate-load` 的 mtime ✓。 |
| P-143 | **L5 的读数此前没有声明“两侧加载的 lisp 是哪一形态” ⇒ 每一次 L5 时间/行为比较都可能拿“解释执行的端口”去比“编译执行的参照”** —— 参照树自带 **1,645 个 `.elc`**（`emacs/lisp/**/*.elc` ✓ 参照装载的就是它们 ✓，实测 `symbol-file` 对 `electric-pair--balance-info` 给出 `emacs/lisp/elec-pair.elc` ✓、`cc-mode` 同理 ✓）；端口的 `build/port/lisp` 是 `femacs/lisp` 的软链 ✓，其中只有 **111 个 `.elc`（全在 `lisp/emacs-lisp`）+ 1,459 个 `.el`** ✓ ⇒ 端口在 L5 里几乎全程**解释执行** ✗。判别实验（4.489 ✓）：⑴ 两侧都强制 `(setq load-suffixes (list ".el"))` 后跑 `electric-tests` 的最小复现，**参照 0.002 s 通过** ✓ ⇒ 端口那个挂死**不是**解释执行的账 ✓；⑵ 用端口自己的编译器把 `lisp/progmodes` 编出来（`tools/gen-elc.py lisp/progmodes --skip-matching` ✓，产出 100 件 ✓）之后，端口**照样挂死（≥120 s）** ✓ ⇒ 该缺陷在**编译态**同样成立 ✓、与加载形态无关 ✓。 | **规则，立即生效** | ① **L5 的任何时间读数与“行为差别”读数，先声明两侧加载的 lisp 形态** ✓：写清“参照跑 `.elc`、端口跑 `.el`”（或“两侧都强制 `.el`”✓）以及**这条差异是否影响结论** ✓；**禁止**在未声明的情况下把两者的耗时并列 ✗（这既会把解释开销记成缺陷，也会掩盖真缺陷 ✓）。② **`.elc` 全族不是纯生成物欠账，它是 L5 的前置** ✓（DoD-4 ✓）：读数顺序是 `tools/gen-elc.py <scope> --skip-matching` → 再看 L5 ✓；当前完成度 **111 → 197 / 1,459 = 13.5%** ✓（4.489 增 `lisp/progmodes` 100 件 ✓）。③ `gen-elc` 报的 `DIFFERENT` 已有**具体形状** ✓：端口的编译器把参照以 `#$` **引用**的 docstring（`.elc` 里写作 `("font-lock.elc" . 44273)` ✓）**内联**成字面量 ✗，并使 `#N=` **共享常量编号整体错位**（`#100="else"` 对 `#101="else"` ✗）⇒ 这两条是查端口 `bytecomp` 的起点 ✓（不是 cc-mode 自身的问题 ✗）。④ 探针纪律再加一条（P-140 的续 ✓）：**对“挂死/循环”类缺陷，Lisp 侧探针会改变结论** ✗（本轮同一份探针脚本，仅因为多了一句 `(match-data)` 就能让端口从“挂死”变成“跑完” ✗）⇒ 这类缺陷的下一步证据要在**端口侧**加 env 门的诊断（P-115 形状 ✓），不要把 Lisp 探针的读数当结论 ✓。 |
| P-144 | **`gen-elc` 此前没有镜像参照自己的字节编译开关与编译方法 ⇒ `.elc` 的判定比大面积假红** —— 参照的 `lisp/Makefile.in` 74-78 把三个开关写死在 `BYTE_COMPILE_FLAGS` 里：`load-prefer-newer t`、`byte-compile-warnings 'all`、**`org--inhibit-version-check t`** ✓；最后一个**是承重的** ✓：`org-assert-version`（`org-macs.el` 52）展开式里直接引用 `org--inhibit-version-check` 的**展开期取值** ✓，不设它时每个 `org/**` 文件都会多出一段运行期版本检查 ✗（4.490 实测：`lisp/org` **1 identical / 124 DIFFERENT** ✗）。按参照原样补上两个开关后，同一批（参照自己的方法：一文件一次程序启动 ✓）⇒ **105 identical / 19 DIFFERENT** ✓。 | **规则，立即生效** | ① **字节编译一律逐字带上参照的 `BYTE_COMPILE_FLAGS`** ✓（已落在 `tools/gen-elc.py` 的 `BYTE_COMPILE_FLAGS` ✓，注释里点名 `lisp/Makefile.in` 74-78 ✓）；**禁止**凭“默认值应该一样”省掉它们 ✗（本条的代价是 123 件假红 ✓）。② **判定必须用参照自己的方法** ✓：`DEFAULT_CHUNK = 1`（`$(TARGETS): %.elc: %.el` = 一次程序启动编译一个文件 ✓）；`--chunk N` 只是冒烟用、其判定比**不可比** ✗（工具注释已测：`lisp/emacs-lisp` 分四十批 = 97/1/13，一文件一跑 = 101/2/8 ✓）⇒ 4.490 那批“identical/DIFFERENT 比例”是 chunk-40 的读数、**不采信** ✗（族级“已生成 1,555/1,645”仍成立 ✓，它是“存在性”读数 ✓）；重取必须 chunk=1 ✓。③ **参照的 `.elc` 是两阶段 bootstrap 的混合物** ✓（本轮定位）：残余 19 件里有 **13 件**在“编译时只加载同源 `.el`”（`load-suffixes` 只留 `.el` ✓）的形态下**逐字节一致** ✓，另外 6 件反而相反 ✗ ⇒ 复现该混合物要镜像 bootstrap 两阶段（先源后编 ✓），不能靠单档开关 ✗。④ 残余形状要**显式登记**、不许无声 mask ✗：⑴ code-level 的跳转/共享常量编号错位 ✗（`oc-basic.elc` ✓）；⑵ 懒 docstring 里嵌**编译机的绝对路径** ✗（`("/Users/…/build/emacs-ref/lisp/org/org-element.elc" . 27921)` ✓）——这条本身就是**构建环境产物** ✓，参照树上那些字节只对本机成立 ✓ ⇒ 归一化要作为**新的 rewrite 类别**写明（与 0.1-19 的产品名类别同形 ✓），并在判定里保持“其余字节仍要一致”的前提 ✓。 |
| P-145 | **`.elc` 判定的“差异”此前没有归因步骤 ⇒ 把**构建状态**与**端口缺陷**混为一谈（53 件“真 DIFFERENT”里实际只有 0 件是端口缺陷 ✓）** —— `tools/gen-elc.py` 的 `--cross-check`（用**参照自己的程序**在**端口树里**编同一份源码 ✓）给出决定性归因：`lisp` 全树 97 件不一致里 **96 件是 `state-only`** ✓（"两个程序在这里产出**同一份字节**" ⇒ 差异来自**这棵树的构建状态** ✗，即编译该文件时依赖是 `.elc` 还是 `.el` ✓、由参照自己的 bootstrap 历史决定 ✓），**1 件**报 `DEFECT` ✗ ⇒ 再对该件做**同一程序编两遍**的判定：`lisp/calendar/diary-icalendar.el` 两次产物**差 19 字节** ✗ ⇒ 它是**编译期嵌入时间戳**的（`ical:make-uid` 对 `(format-time-string "%s%N")` 取哈希 ✓，`icalendar.el` 517 ✓）⇒ **本质不可重复** ✗ ⇒ **端口编译器在这 1,644 件产物上是 0 缺陷** ✓。 | **规则，立即生效** | ① **任何“逐字节不一致”结论必须先过两道归因** ✓：⑴ `--cross-check`（参照程序在端口树里编 ✓）分出“状态”与“实现”；⑵ 报 `DEFECT` 的件再做 `--determinism-check`（同一程序编两遍 ✓）分出“实现缺陷”与“产物本身不可重复” ✓。② **不可重复的产物进 `VOLATILE` 白名单** ✓（`tools/gen-elc.py` 的 `VOLATILE`，每条都要写明**为什么**不可重复 ✓），判定里单列 `volatile` 一类 ✓，**不许**把它算成 DIFFERENT ✗，也**不许**为了“看起来一致”去 mask 字节 ✗。③ **“状态差”不是欠账的托辞** ✓：它是**可复现的**（镜像参照的两阶段顺序 + 编译时依赖的形态 ✓）⇒ 记作 DoD-4 的具体工作项 ✓；本轮已量化：**1,560 逐字节一致 / 31 只差产品名 / 96 状态差 / 1 volatile / 1 缺件（生成物 `ja-dic`）** ✓。 |
| P-146 | **`re_match_2` 的返回值可以落在 C 的契约之外 ⇒ “不匹配”与“匹配到 `pos` 之前”被混为一谈（`electric-tests` 挂死的根因类 ✓）** —— C 的契约只有三种：**-2**（失败栈耗尽 ✓）、**-1**（不匹配 ✓）、**≥0**（匹配长度 ✓，`regex-emacs.c` 4102-4110 ✓）。端口在 `lisp/electric-tests` 的最小复现里给出的是 **`i = -11`** ✗（`looking_at` 因此返回 nil ✗，而匹配器**以为成功** ✓）⇒ Lisp 侧的循环 `c-after-change-mark-abnormal-strings` 用 `(goto-char (min (1+ (match-end 0)) (point-max)))` ✓ 永远不前进 ✗ ⇒ **死循环** ✓。读数（端口侧 instrument ✓，`FEMACS_REG_TRACE=1` ✓）：`REGTRACE loop point=2 i=-11 text="(<<>" pat=\(\\(\?…` ✓ —— 即"同一 `point` + 同一 pattern 连续 200 次"的判据抓到的 ✓；`i = d - pos` ⇒ **`d` 在 `pos` 之前 11 字节** ✗ ⇒ `d` 是**非法/陈旧指针** ✓（缓冲只有 5-10 字节 ✓）。已排除：`gc-cons-threshold` 取 20 MB 与 1000 两种极端都照挂 ✓（参照在 tiny-GC 下 0.0005 s 通过 ✓）、`inhibit_shrinking` 镜像在位 ✓（`regex-emacs.rs` 4336/4855 ✓）、`looking-at`/`match-data`/marker/`save-excursion` 的 12 例电池两侧全同 ✓。 | **规则，立即生效** | ① **匹配器的返回值必须按 C 的契约解释** ✓：出现 **-1 与 -2 之外的负值**一律是**缺陷** ✗（不许当成"不匹配"吞掉 ✗）；诊断开关 `FEMACS_REG_TRACE` 已在 `looking_at_1` 里挂上 ✓（`getenv` **只在首次采样** ✓，P-126 的形状 ✓），它按"同一 point + 同一 pattern 连续 200 次"报一行 ✓ ⇒ 死循环类缺陷的定位成本从"整棵 cc-mode"降到一行 ✓。② **`d` 回归的候选点（下一轮的顺序 ✓）**：⑴ `succeed` 指令的 `do_succeed` 分支 ✓；⑵ `restore_best_regs`（`d = match_end` ✓，`match_end` 在 C 里是 `UNINIT` ✓）；⑶ `POP_FAILURE_POINT` 从失败栈恢复的 `str` ✓（P-142 的 smart 循环就在这条路上 ✓）。③ **回归门**：修完后必须同时跑 `regex-emacs-tests`（35 例 ✓）、`electric-tests`（903 例 ✓）、`log-edit-tests`（`re_match_2_internal` 的 UB 中止 ✓）与 `gate-fast` ✓。 |
| P-147 | **`electric-tests` 挂死的机理已落到**数值级**：接受时 `d` 落在 string1 的 `end1 + 1`、而 `dend` 仍是 string2 段 ⇒ `POINTER_TO_OFFSET` 走 string2 分支给出负数 ✗** —— 端口侧 instrument（`FEMACS_REG_TRACE=1` ✓）在最小复现里给出：**`dcnt=-11 d-s1=4 s1-s2=-17 size1=3 pos=1 first=0 succ=1 restore=0`** ✓。算式核对：`d - s2 = 4 - 17 = -13` ✓、`-13 + size1(3) = -10` ✓、`-10 - pos(1) = -11` ✓ ⇒ **macro 本身与 C 逐字一致 ✓ 没错** ✗；错的是 **`d` 越过了 `end1` 一个字节** ✗（`size1 = 3` 而 `d - s1 = 4` ✓），且 `FIRST_STRING_P(d)` 因此为 0 ✓ —— 这正是"段切换该发生却没发生"的形状 ✓。⇒ 下一轮只需找**把 `d` 推过 `end1` 而不经 `PREFETCH` 的那一处** ✓：候选是 `exactn`/`anychar`/`syntaxspec`/`charsetc`/`duplicate` 之外**没有 `PREFETCH` 的前进** ✓，或 `dend` 从 string1 段被改成 string2 段而 `d` 没跟 ✓。 | **规则，立即生效** | ① **匹配器的"段切换"只有两条合法路径** ✓：`PREFETCH`（`d == dend` ⇒ 换段 ✓）与 `PREFETCH_NOLIMIT`（`d == end1` ⇒ 换段 ✓），另有失败路径里的 `d = str` + `dend` 重算 ✓（C 5287-5292 ✓）⇒ **任何 `d` 的前进都必须先经过其中之一** ✓，出现 `d == end1 + 1` 即为缺陷 ✗。② **`POINTER_TO_OFFSET` 的"负数结果"是症状不是病** ✗：它给出负值当且仅当 `d` 已在 string1 的 `end1` 之后且 `dend` 仍指向 string2 段 ✓（本条算式已核 ✓）⇒ 排查要从 `d`/`dend` 的**不变量**入手 ✓，不要先去改 macro ✗。③ 本节的三条诊断（`loop` ✓ / `accept-outside` ✓ / `dcnt<−1` ✓）都留作常驻仪器 ✓（env 门、首次采样、默认关 ✓），下一轮修完必须同时跑 `regex-emacs-tests`(35) ✓ + `electric-tests`(903) ✓ + `log-edit-tests` ✓ + `gate-fast` ✓。 |
| P-148 | **"`d` 落在两串之间"这一状态本身被做成了常驻判据，并据此点名了第一处移动者 = `charset_not`** —— 判据：`end1 < d < string2` ✓（缓冲在 gap 处被切成两段 ✓ ⇒ 夹在两段之间的地址**永远非法** ✓，与 `dend` 的取值无关 ✓）。读数：**`REGTRACE d-in-gap opcode=16 prev=5 pattern-offset=47 d-s1=4 size1=3 dend-s1=20`** ✓ ⇒ 发现者是 `on_failure_jump_loop`（16 ✓）、**留下它的是 `charset_not`（5 ✓）** ✓；同一批里 `prev` 还出现 14（`on_failure_jump` ✓）、7（`stop_memory` ✓）、13（`jump` ✓）⇒ 说明越界状态**先由某次弹栈造成** ✓（`d` 已在 string1 段而 `dend` 已指向 string2 ✓），`charset_not` 只是把它推进 gap ✗。另加一条常驻判据：**"弹栈后 `d` 与 `dend` 跨段"** ✓ —— 但它会在**合法的**弹栈上触发 ✗（`d = str` 之后两行才按 C 5286-5287 重算 `dend` ✓）⇒ 该探针**不得**作为缺陷判据 ✗（本轮据此撤回一处）。 | **规则，立即生效** | ① **gap 判据是硬判据** ✓：`end1 < d < string2` 永远非法 ✓（两段之间是 gap ✓），以此定位移动者 ✓；`d`/`dend` 的"跨段"**不是**判据 ✗（合法的弹栈在重算前就跨段 ✓）。② 下一轮的靶子改为**弹栈**：找那次把 `d` 放回 string1 段却不重算 `dend` 的路径 ✓（候选：`d = str` 的四个 arm ✓ 与 `restore_best_regs` 的 `d = match_end` ✓，后者之后 C 有 `dend = (d >= string1 && d <= end1) ? end_match_1 : end_match_2` ✓ —— 逐字核对 ✓）。③ **行数漂移棘轮**：本轮仪器使 `regex-emacs.rs` 达 **6,245 行 vs C 5,367（16.4%）** ✗ ⇒ 按 `docs/ports/line-drift-tolerance.tsv` 的既有机制登记 **0.18**（理由写清三处诊断的身份与默认关闭 ✓）⇒ `gate-fast` 回到 **590 done / 0 failing** ✓。④ 仪器一律"env 门 + 首次采样 + 默认关" ✓，且**只为定位服务**：一旦缺陷修掉就删掉对应探针 ✓（保留 `loop` 与 `dcnt` 两条通用判据 ✓）。 |
| P-149 | **"`d` 与 `dend` 分段不一致"只有在**操作码边界**上才是缺陷判据（弹栈内部是合法的瞬时态），且必须排除 `size1 == 0`** —— 判据：`size1 != 0 && (dend == end_match_1) != (d <= end1)` ✓，在**每个操作码执行前**检查 ✓。读数：**`REGTRACE crossed prev=2 opcode=0 d-s1=0 size1=1 dend-s1=20`** ✓（另见 `prev=26`、`prev=14` ✓）⇒ 进入不一致状态时 `d` 在 **string1 的起点**（`d-s1=0` ✓）、`dend` 却指向 **string2**（`dend-s1=20` ✓），而**留下它**的操作码是 `exactn`（2 ✓）/ 26 / `on_failure_jump`（14 ✓）。两条**分错极性的探针**本轮各踩一次（照实记 ✓）：⑴ 在弹栈内部检"跨段"会在**合法**路径上触发 ✗（`d = str` 之后两行才按 C 5286-5287 重算 `dend` ✓）；⑵ 不排除 `size1 == 0` 时，单串搜索（`end_match_1` 无意义）会让每个 `d == end1` 都报 ✗ ✓。 | **规则，立即生效** | ① **判据只能挂在操作码边界** ✓：`crossed`（分段不一致 ✓）与 `d-in-gap`（夹在两段之间 ✓）都放在操作码分派**之前** ✓ —— 匹配器内部的瞬时态一律不作判据 ✗。② `size1 == 0` ⇒ 不存在第一段 ⇒ 判据必须先排除 ✓。③ 下一轮的靶子收敛到**两条写 `d` 的路径** ✓：`exactn` 的 `d = dfail` 回滚 ✓（`dfail` 的捕获点与 `dend` 的对应关系 ✓）与弹栈的 `d = str` ✓；修法**必须**保持与 C 同形 ✓（C 5286-5287 只置 `end_match_1` ✓），若确认 C 在同样输入下不会出现该状态，则缺陷必在**捕获/恢复点**而**不是**在重算逻辑 ✓。 |
| P-150 | **弹栈路径的 `dend` 重算是**正确的**（负结果 ✓），不一致只从 `exactn` 出来** —— 在 C 5286-5287 的重算**之后**再检一次分段一致 ✓（`pop-mismatch` 判据 ✓），本轮 **0 次触发** ✓ ⇒ "弹栈把 `d` 放回 string1 段却不重算 `dend`"这条假设**排除** ✓；同时 `d = dfail` 的回滚在端口里与 C 同形（`d = dfail; goto_fail` ✓ ⇒ 随后必经重算 ✓），也被 `pop-mismatch` 覆盖为 0 ✓。于是 `crossed` 的 `prev=2`（`exactn` ✓，两次出现 ✓）成为唯一未解释来源 ✓：进入不一致时 `d` 在 string1 起点、`dend` 在 string2 ✓，说明**某次 `exactn` 的"成功退出"**把 `d` 留在 string1 段而 `dend` 已按上一次 `PREFETCH` 停在 string2 ✓。 | **规则，立即生效** | ① **负结果同样入库** ✓（本条即是）：把一个候选排除掉的读数与把它确认的读数同等重要 ✓，写清"判据 + 0 次触发" ✓。② 下一轮在 `exactn` 上做**入口/出口成对读数** ✓：入口打印 `d`/`dend`/`dfail`（都是已算好的局部 ✓，P-140 ✓）、出口再打印一次 ✓，判定"出口时是否已跨段" ✓ —— 这是 `prev=2` 唯一可能的产生点 ✓。③ `PREFETCH` 切换段的**唯一合法时机**仍是 `d == dend` ✓；`exactn` 的 `d++`（C ✓）与端口的 `d = d.offset(1)` 必须逐字对应 ✓，多字符情形下 C 用 `d++` 而端口若用 `d += buf_charlen` ✗ 会在**多字节**文本上先出问题 ✓ —— 下一轮顺带核对这一处 ✓。 |
| P-151 | **`exactn` 与两串的建立（`p1`/`p2`/`end_match_*`）都已逐字核对为与 C 同形 ⇒ 跨段状态不是"文本比较或前进量"的错** —— 逐字比对结果 ✓：多字节分支 = `PREFETCH (d = dfail)` → `buf_ch = string_char_and_length (d, &buf_charlen)` → `if (TRANSLATE (buf_ch) != pat_ch)` ✓ → `p += pat_charlen` ✓ → **`d += buf_charlen`** ✓（**不是 `d++`** ✗ —— 4.498 的登记把另一分支的 `d++` 误当成多字节分支的形状，此处更正 ✓）→ `mcnt -= pat_charlen` ✓；串的建立同样一致 ✓：`pos >= size1` 走 `d = string2 + (pos - size1)`、`dend = end_match_2 = string2 + (stop - size1)` ✓，否则 `end_match_1 = end1`（或 `string1 + stop` ✓）、`end_match_2` 按 `stop < size1` 分两种 ✓、`d = string1 + pos`、`dend = end_match_1` ✓。⇒ 读到的 `s1 - s2 = -17` 与 C 的 gap 布局一致 ✓（`s2 - s1 = GPT_BYTE + GAP_SIZE - BEGV_BYTE` ✓），`size1 = 3` ✓ ⇒ `d - s1 = 4` 就是 **越过 `end1` 一字节** ✓，产生它的只能是**某次 `PREFETCH` 没有在 `d == dend` 时切换** ✗ 或 **`dend` 被单独改成 `end_match_2`** ✗ —— 第 4.498 轮的 `pop-mismatch` 已把后者排除 ✓。 | **规则，立即生效** | ① **"逐字核对"要按分支核** ✓：同一 `case` 的两个分支前进量不同（`d += buf_charlen` vs `d++` ✓）⇒ 登记时必须写明**哪一个分支** ✓（本条更正 4.498 ③ 的措辞 ✓）。② 下一轮的靶子收窄为**`PREFETCH` 的调用点清单** ✓：逐个核对"调用前 `d` 与该段 `dend` 是否可能不等" ✓（C 的保证是"调用前 `d` 至多在段的末尾" ✓）⇒ 若某处 C 在**多字符前进之后**才调 `PREFETCH` ✓，端口必须同样如此 ✗ 不许提前或延后 ✓。③ 诊断仪器已足够（`crossed` ✓ / `d-in-gap` ✓ / `dcnt` ✓ / `pop-mismatch` ✓）：下一轮**不再加探针** ✓，改为把 `PREFETCH` 的调用点与 C 一一对照 ✓ —— 这是本轮读数指向的**代码级**下一步 ✓。 |
### 0.2 命名与可见性

- 函数、宏、结构体、联合、枚举、枚举成员、全局变量、静态变量、subr：**名字逐字相同**
  （`Fcons`、`Scons`、`Qnil`、`Vload_path`、`GC_MARK_STACK`、`DEFUN`、`AUTO_STRING`…）。
- 与 Rust 关键字冲突时用原生标识符：`r#const`、`r#type`、`r#match`、`r#where`、`r#fn`、
  `r#use`、`r#mod`、`r#final`。参照树中实际冲突（抽样）：`const` 96、`type` 31、
  `match` 9、`where` 5、`fn` 5、`use` 2、`mod` 2、`final` 2。
- **声明 ≠ 定义**：C 的 `extern` 声明 → `unsafe extern "C" { … }`（头镜像内）；C 的定义
  → `#[unsafe(no_mangle)] pub`（唯一单元内；edition 2024 要求 unsafe 属性写法，裸
  `#[no_mangle]` 直接报 "unsafe attribute used without unsafe"）。`static`（内部链接）→
  非 `pub`、不导出符号。
- **`DEFUN` 的文件级记录 `Sfoo`**：C 是 `static union Aligned_Lisp_Subr sname`（内部链接）。
  Rust 侧是同名 `static mut Sfoo`，但 Rust 的静态符号会被 mangle（`_ZN…Sfoo`），
  C 里叫 `Sfoo`（局部符号）。记录为偏离：源码里的名字、`defsubr (&raw mut Sfoo)` 的
  引用与 C 逐字相同，符号表里的名字不同（参照里没有按名字查找 `S*` 的代码）。
- C 的全局变量（尤其 `Lisp_Object` 全局）在 Rust 侧写成 `#[unsafe(no_mangle)] pub static mut NAME: T`
  ——`staticpro` 注册的是它的地址，地址必须稳定（不入 `Box`、不搬移）。

### 0.3 语句与顺序

- 一条 C 语句 = 一条 Rust 语句；定义顺序、`#if` 分支顺序不变。
- 允许并预期 `unsafe`；不得借 `unsafe` 改变语义（例如把 `goto` 清理路径改成提前返回）。
- 禁止 `todo!()`/`unimplemented!()`/空实现/默认返回值/注释掉的代码（硬约束 3）。
- 注释逐字保留，但 C 的注释体里若含 `/*`（如 `w32fns.c:4127`）必须先转义/改写，
  因为 Rust 块注释可嵌套——否则会吞掉后续代码。

### 0.4 参照规模（由 `python3 tools/gen-baseline.py` 生成，可复算）

覆盖 8 个参与链接的 C 目录：`src/`、`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、
`nt/`、`msdos/`。

| 度量 | 数值 | 来源 |
| --- | ---: | --- |
| 文件数 / 行数 | 590 / 678,320 | `docs/baseline/inventory.tsv` |
| 函数定义 | 10,012 | 同上（注释剥离、`#if 0` 跳过） |
| 活跃 `DEFUN`（subr） | **1,841** | `docs/baseline/subrs.tsv`（1,849 处文本出现 − 8 处在 `#if 0` 死区） |
| 唯一 subr C 名 | 1,736 | `docs/baseline/totals.tsv` |
| 宏定义 | 7,156 | `inventory.tsv` |
| 具值常量（对象宏 + 枚举成员） | 3,898 | 同上 |
| 结构体/联合字段 | 5,764 | `LIFETIMES.tsv` |
| 文件级对象 | 1,934 | 同上 |

构造计数（评审复算 + 本轮复核，命令见括号）：

| 构造 | 数值 | 命令 |
| --- | ---: | --- |
| `#if/#ifdef/#ifndef` 行 | 6,147 | `grep -rhoE '^[[:space:]]*#[[:space:]]*(if\|ifdef\|ifndef)\b' *.c *.h \| wc -l` |
| `__atomic_*` 内建 | **24** | `grep -rhoE '__atomic_[a-z_]+ *\(' *.c *.h \| wc -l` |
| `ckd_add` / `ckd_sub` / `ckd_mul` | 154 / 19 / 114 | 同法 |
| 位域成员行（`.c`+`.h`） | 557 | `grep -rhoE ':[[:space:]]*[0-9]+[[:space:]]*;' *.c *.h \| wc -l` |
| `staticpro (&…)` 注册调用 | 273 | `grep -rho 'staticpro *(&' *.c \| wc -l` |
| `mark_*` 调用族 | 281 | `grep -rhoE '\bmark_[a-z_]+ *\(' *.c *.h \| wc -l` |
| `eassert` / `clobbered_eassert` | 1,806 / 6 | 同法 |
| `DEFSYM`（含定义） | 2,311 | 同法（调用 2,310） |
| `INLINE` 标注 | 638 | `grep -rho '\bINLINE\b' *.c *.h \| wc -l` |
| 裸 `alloca (` | 160 | 同法 |
| `SAFE_ALLOCA_LISP` / `SAFE_ALLOCA_STRING` | 30 / 17 | 同法 |
| `setjmp` / `longjmp` 词频 | 34 / 41 | 其中 `sys_setjmp` 18、`sys_longjmp` 11 |
| `va_list` | 49 | 同法 |
| computed goto | 1 处（`bytecode.c:581`） | `grep -rn 'goto \*' *.c` |
| 线程局部 / C11 原子 | 0 / 见 `__atomic_*` 24 | 两者不同：见 §8 |

> 注：v1 中“8,991 函数”“1,850 subr”“位域 30”“原子 0”等数字已废弃；以本表为准。

---

## 1. 类型映射

### 1.1 标量

| C | Rust | 说明 |
| --- | --- | --- |
| `EMACS_INT` / `EMACS_UINT` | `isize` / `usize` | 与 C 同宽 |
| `Lisp_Word` / `Lisp_Object` | `*mut Lisp_X`（见 §1.2） | C 默认构建下是指针型 |
| `ptrdiff_t` / `size_t` | `isize` / `usize` | 用 `libc` 别名 |
| `bool` | `bool` | 1 字节，与 C 一致 |
| `bool_bf` | 位域（§1.3） | 不可用 `bool` |
| `printmax_t` / `intmax_t` / `uintmax_t` | `i64` / `i64` / `u64` | 打印语义 |
| `char`（字节） | `c_char` | `unsigned char *` → `*mut u8` |
| `long double`（9 处） | `c_longdouble` | 不得降级为 `f64` |
| `float` / `double` | `f32` / `f64` | IEEE 一致 |

隐式整型提升与回绕：凡 C 里依赖二进制补码回绕的表达式，必须显式写
`wrapping_add`/`wrapping_mul`/`wrapping_shl` 等，保留原表达式结构。
C23 的 `ckd_add/ckd_sub/ckd_mul`（287 处，`lib/stdckdint` 语义）→ 用
`overflowing_*` + 目标类型转换判定，逐处核对“溢出时不写入目标”的语义。

### 1.2 `Lisp_Object`（v2 更正）

C（`lisp.h:311-323`、`lisp.h:580-600`）：

```c
typedef struct Lisp_X *Lisp_Word;          /* LISP_WORDS_ARE_POINTERS 默认成立 */
typedef Lisp_Word Lisp_Object;             /* 非 CHECK_LISP_OBJECT_TYPE 时 */
#define lisp_h_XLI(o) ((EMACS_INT) (o))    /* 指针/整数互转 */
#define lisp_h_XIL(i) ((Lisp_Object) (i))
```

Rust：

```rust
#[repr(C)] pub struct Lisp_X { _private: [u8; 0] }   // 不完整类型，仅用于指针
pub type Lisp_Word = *mut Lisp_X;
pub type Lisp_Object = Lisp_Word;
#[inline(always)] pub const fn lisp_h_XLI(o: Lisp_Object) -> EMACS_INT { o as isize }
#[inline(always)] pub const fn lisp_h_XIL(i: EMACS_INT) -> Lisp_Object { i as *mut Lisp_X }
```

- **P-01**：`Lisp_Object` 保持 C 默认构建的**指针型别名**；`CHECK_LISP_OBJECT_TYPE`
  与 `LISP_WORDS_ARE_POINTERS=false`（wide-int）**不支持**，在 `xtask` 中直接拒绝
  （记偏离项）。
- **P-02（`USE_LSB_TAG`）**：它不是常量而是由 `ALIGNOF_EMACS_INT`、`HAVE_C_ALIGNASOF`、
  `VAL_MAX/2 < INTPTR_MAX` 推导（`lisp.h:250-258`），并决定 `VALMASK`、`Lisp_Int1`、
  `lisp_h_XTYPE` 是否存在、`make_fixnum` 形态与 dump 格式。移植策略：把
  `USE_LSB_TAG` 提升为**构建期 `cfg` 决策**，只支持 `= true` 的 64 位目标；其余目标
  拒绝构建。`VALMASK`/`GCTYPEBITS`/`INTTYPEBITS`/`Lisp_Type`/`pvec_type` 等逐值一致，
  并由编译期断言锁定。
- `lisp_h_make_fixnum` 的 C 形态依赖 `HAVE_STATEMENT_EXPRESSIONS` + `typeof`（`lisp.h:408-417`）：
  Rust 宏必须先做 `let n_: EMACS_INT = ($n) as EMACS_INT;` 再做溢出断言，且断言按
  `cfg(enable_checking)` 开关——**不得在 C 无断言的构建里新增断言**。

### 1.3 `struct` / `union` / `enum` / 位域

- `struct` → `#[repr(C)] pub struct NAME { … }`，字段顺序、类型、命名一致；嵌套匿名
  `struct`/`union` 必须显式命名嵌入，**不得改变布局**。
- `union` → `#[repr(C)] pub union NAME { … }`；读写一律 `unsafe`。
- `enum` → `#[repr(C)]`（C 的 enum ABI 是 `int`）；`ENUM_BF(T)`（位域里的枚举）见下。
- **位域**（`.c`+`.h` 共 557 行，含 282 处 `bool_bf`）在 Rust 无对应物 → 用等宽整数 +
  显式 `get_*`/`set_*` 辅助，位宽、位序、整体宽度必须逐位一致；与 dump/`memcpy`/`union`
  混用处逐个核对（§6.4）。`ENUM_BF (T) name : 2` 的宽度是 2 位，不是 `T` 的宽度。
  **落地约定**（`Lisp_Marker`、`itree_node` 已用）：同一存储单元里连续的位域折成一个
  整数字段，名字用 `bits`（与后一字段之间需要凑偏移时才加 `_pad`），字段上方用
  `///` 逐条列出 C 的位域声明；**打包是否正确由 C 编译器的偏移/`sizeof` 断言证明**
  （§0.1-16）。
- 变长结构体（`FLEXIBLE_ARRAY_MEMBER` 11 处）→ `data: [u8; 0]` + `offset_of!`。

### 1.4 函数指针与类型双关

- 带 context 的函数指针逐字对应 `unsafe extern "C" fn(*mut c_void) -> …`，字段名与签名
  相同；**不得改成闭包**（会改变布局与调用约定）。
- `*(T *) p`、union 换成员读、`memcpy` 到别类型 → `ptr::read_unaligned`/`write_unaligned`
  或 union 读，保持未对齐访问语义。
- **C 的"位置初始化"（`struct X v = { a, b, … };`）Rust 没有对应物**：写成**同名字段字面量**
  （`X { f1: a, f2: b, … }`），字段名与**顺序**都取镜像记录里的（顺序错了值就落错槽，而布局断言
  看不见值），并逐条核对"C 的初始化项数 = 记录的字段数"——`redisplay_interface` 实测 **28 项 = 28 字段**，
  不足的项按 C 的隐式零写成零值 / `None`，在 ledger 里点名。**函数指针项的类型是*字段的*类型**：
  声明已经是那个类型就写 `Some (f)`；不是（镜像按 P-55 把 C 的 `struct frame *`/`struct window *`
  写成 `*mut c_void`）就**取址再 transmute**：
  `Some (unsafe { core::mem::transmute::<*const (), <字段的 fn 类型> (f as *const ()) })`——
  直接 `as` 是 `E0605`（实测）；同一位置上传的本单元 `static` 助手因此必须定义成
  `unsafe extern "C" fn`（本节第一条）。已落地：`src/pgtkterm.rs` 3756-3785 的
  `pgtk_redisplay_interface`（二十八项：一项数组地址、五项直接、十九项 transmute、三项 `None`）。
- **C 的函数指针参数只有一种 ABI，移植后也只有一种**：库函数（libm 等）在 Rust 侧是
  `unsafe extern "C" fn (…)`，同一参数位置上传的本单元 `static` 助手
  （`floatfns.c` 的 `ceiling2`/`floor2`/`round2`/`truncate2`/`identity`/`rounddiv_q`）
  也必须声明成 `unsafe extern "C" fn`，否则类型不同、`rounding_driver` 无法同时接受两者
  （`port(floatfns.c)` 实测：混用 `unsafe fn` 报 `E0308 expected "C" fn, found "Rust" fn`）。

### 1.5 C 的未初始化局部变量

- C 里"声明后立刻由被调函数写入"的局部量（`double f1, f2;` 配 `frexp` 的 out 参数、
  `int exponent;`、`EMACS_INT value;` 这类）在 Rust 必须有初值才能取得地址：统一写成
  零初始化（`let mut value: EMACS_INT = 0;`）并在 ledger 里点名；代价是
  "value assigned is never read" 一类 warning（`gate-fast` 不看 warning）。
- `void *p; f (&p);` 同理；`let p: *const T;` 只在**每个分支都赋值**时才合法
  （`floatfns.c` 的 `rescale_for_division` 就是这种形状）。

---

## 2. 控制流与表达式

### 2.1 宏 → `macro_rules!`

- 带代码的 `#define` → 同名 `macro_rules!`；`do { … } while (0)` → 块表达式。
- **实参求值次数必须与 C 相同**：多数宏可先 `let` 绑定一次，但 C 里存在**多次求值**的宏
  ——`AVAIL_ALLOCA(size) ((sa_avail -= (size)), alloca (size))`、`SAFE_ALLOCA`（对外层
  `size` 两次）、以及 release 下 `eassert`（**完全不求值**）。这些是“单次求值”规则的
  例外，必须逐条保留原求值次数（P-16）。
- 变参宏（`__VA_ARGS__`）→ `($($args:tt)*)`。
- **C 的字符串宏** → 同名 `macro_rules!` 或 `pub const …: &str`，两者都保持**内容字节**
  （比较时忽略 C 的隐式 NUL：`"t"`、`b"t\\0"`、`c"t"` 视为同一串）。用在 C 的相邻字面量
  拼接处（`"%"pI"d"`）时必须是宏：`concat! ("%", pI! (), "d")` 实测可行（`macro_rules!`
  的调用会在 `concat!` 里展开，而 `const` 不行）；`sizeof uLSQM - 1` 这类用法在使用点
  写成 `.len ()`。
- **宏里需要"实参个数"时不能用求值表达式**：数组长度是常量表达式，C 的 `ARRAYELTS`
  换成 `{ let _ = &$arg; 1 }` 会以 `E0435`（非常量值用于常量）失败，必须用
  `{ let _ = stringify!($arg); 1 }` 计数、由数组初始化真正求值一次（A6 实测）。

### 2.2 `switch` / `goto` / 标签

- `switch` → `match`；`case` 顺序、共享体（fallthrough）、`goto` 落入 case 中部
  （`bytecode.c` 大量使用）必须逐处等价。Rust 无 fallthrough：用显式结构化写法
  （共享体抽成同一分支、或用 `loop { … }` 状态机），语义等价由评审逐函数核对。
- 前向 `goto` 清理路径 → `'block: loop { … break 'block; }` 或状态机，**不得改成提前
  `return`** 而跳过后面的清理。

### 2.3 computed goto（`bytecode.c`，v2 更正）

C 里已有两套实现（`bytecode.c:577-598`）：

```c
#define NEXT goto *(targets[op = FETCH])   /* HAVE_GCC_LABELS */
...
#define NEXT break                          /* 否则走 switch */
```

Rust **没有 label-as-value（`&&label`）**，无法构造 `void *targets[]` 跳转表。决策
**P-03**：移植 C 的 `switch` 实现（即 `HAVE_GCC_LABELS = false` 分支，`CASE`/`NEXT`/
`FIRST` 宏名保留，`insn_*` 标签保留），把“threaded 版本不移植”记为偏离项；
`HAVE_GCC_LABELS` 属于 §9.2 的**结构条件**。

### 2.4 变参函数（`va_list` 49 处，v2 更正）

`va_list` 在 `doprnt.c` 里是**普通参数**传递，且用到 `va_copy`（`doprnt.c:595`）。

- **P-04（2026-10-02 定案）**：本机 stable 1.98.1 定义 C 变参函数报
  `E0658: C-variadic functions are unstable`，而两个 dated nightly
  （`nightly-2026-09-14` = 1.100.0-nightly、`nightly-2026-10-01` = 1.101.0-nightly）
  都能在**没有** `#![feature]` 的情况下定义它（`c_variadic` 自 1.99.0 起 stable，两者
  都在其之后；本段初稿所写“09-14 仍报 unstable”是错的，评审 A 的 F1 与评审 D 的 F6
  各自实测推翻了它）。结论：
  1. `femacs/rust-toolchain.toml` 锁**具体** dated nightly，当前 `nightly-2026-10-01`
     ——选它是因为本轮全部闸门证据都在它上面测，不是因为它比 09-14 更“能编”；
  2. 变参 API 名（`VaList::next_arg`/`clone`）与平台模型在升级时用
     `spikes/va-probe.sh` 复测；
  3. stable ≥ 1.99 发布后切回 stable 并删除该偏离记录；
  4. `#![feature]` 无论如何不能用：镜像单元是被 `include!` 的，内部属性非法。
  映射与证据：`docs/spikes/2026-10-02-variadics.md`；pin 下的 Miri 证据：
  `spikes/miri-probe.sh`（需要 `rustup component add miri rust-src --toolchain <pin>`，
  已对本 pin 安装，`miri-probe: clean (nightly-2026-10-01)`）。
- 变参**调用**（`printf` 家族）不受此限。

### 2.5 断言（v2 更正，评审判定为阻断级）

`eassert`/`eassume` 的 C 语义（`lisp.h:180-196`、`lib/verify.h`）：

| 形态 | C 行为 |
| --- | --- |
| `ENABLE_CHECKING` 且 `suppress_checking == false` | 求值并在失败时 `die` |
| `ENABLE_CHECKING` 且 `suppress_checking == true` | 求值与否按实现，但**不 die**（运行期全局短路） |
| release（`!ENABLE_CHECKING`） | `((void) (false && (cond)))`——**完全不求值** |

Rust 侧三种形态逐一对应，`suppress_checking` 作为同名全局保留；**release 分支不得求值**。
`eassume` → `assume` 语义（`lib/verify.h:389` 的四种平台形态）。另需移植
`clobbered_eassert`（`eval.c:1373` + 5 处）、`verify` 两态、`static_assert`（91 处）、
`eassume`（184 处）。
- `static_assert`（C23 关键字）→ 各单元自己的同名 `macro_rules!`，展开成
  `const _: () = core::assert! (cond);`，调用点写成 `static_assert! (cond);`（A6 实测）。
  **凡"布局/配置断言"（`const _: () = …` 这一族）一律写 `core::assert!`，不写裸 `assert!`**（3.432）：
  C 的 `<assert.h>` 在一个 TU 里只被读一次，冻结配置定义 `NDEBUG` ⇒ 它的 `assert` 是 `((void) 0)`；
  端口在有的单元里按 C 的名字保留了这个空体宏，而**经 `include!` 到达的裸 `assert!` 会解析到它**
  ——轻则 `E0659`，重则**镜像里的布局断言被静默展开成 `()`**（androidvfs 的变异实测量到过）。
  `core::assert!` 是路径限定宏，不受同名 `macro_rules!` 遮蔽，是这一族唯一安全的写法；
  3.432 一次性把 2,138 处（106 个文件）改到位。

### 2.6 数值与内建

| C | Rust |
| --- | --- |
| `ckd_add/sub/mul`（287 处） | `overflowing_*` + 目标类型判定（溢出时不写目标） |
| `__builtin_expect` | 保留为分支提示辅助；**不等价**于块布局提示，记偏离项 |
| `__builtin_bswap*` / `__builtin_inff` | `swap_bytes` / `f32::INFINITY` |
| `__builtin_unreachable` | `unsafe { core::hint::unreachable_unchecked() }` + SAFETY |
| `__builtin_unwind_init` / `frame_address` | 见 §4.3（不是 `black_box`） |

---

## 3. 内存

### 3.1 分配器

- `xmalloc`/`xrealloc`/`xfree`/`xzalloc`/`lisp_malloc`/`lisp_free`/`lisp_align_malloc`/
  `pure_alloc`/`mmap_*` 同名移植；Lisp 对象只经这些路径分配。
- **禁止** `#[global_allocator]`；Rust 侧不得用 `Box`/`Vec`/`String` 分配 Lisp 对象。

### 3.2 `alloca` / `SAFE_ALLOCA`（v2 更正）

C 的预算是**每函数共享 16 KB**（`lisp.h:5616-5631`）：`sa_avail` 动态递减，超出转
`record_xmalloc` + `unwind` 释放。

- 计数：裸 `alloca (` 160 处；`SAFE_ALLOCA_LISP` 30、`SAFE_ALLOCA_STRING` 17、
  `REGEX_USE_SAFE_ALLOCA` 2。
- **P-05（4.437 定稿）**：`sa_avail` 记账逐字保留，超限仍走 `record_xmalloc` 兜底。每次请求是一块
  独立的堆块（`sa_stack_alloc`：16 字节对齐 ✓），挂在链上供 **GC 沿链保守扫描**（那是端口替 C 的
  栈扫描补的 ✓ status 3.626 ✓）。**释放由帧内水位线驱动**：`USE_SAFE_ALLOCA!` 声明第三个**帧内**名字
  `sa_mark`（= `sa_stack_mark ()` 读到的链高 ✓），`SAFE_FREE!`/`SAFE_FREE_UNBIND_TO!` 调
  `sa_stack_release_to (sa_mark)` 释放水位线以上的块 ✓；`safe_free` 回到 C 的原文（不含池子那行 ✓），
  **不新增任何 specpdl 条目** ✓。块记下所属 Lisp 线程，释放遇到别的线程的块就停（那是挂起不是死亡 ✓）。
- **循环内 alloca**（`xmenu.c:1021`、`pgtkmenu.c:303`、`w32menu.c:547` 等）依赖每次迭代地址不同：
  每请求一块独立内存，天然满足 ✓。
- **被否证的三种候选**（记录在此，免得再试 ✗）：⑴ 栈指针内联汇编；⑵ 池 + **按 specpdl 深度**的释放
  （或全局水位线）；⑶ 全走堆 + **每请求一个 specpdl 条目**。理由与读数见 P-117 ⑤/⑥/⑦ ✓。
- **残余偏离（唯一一条）**：字节活到**有一次 `SAFE_FREE`**，而非 C 的"帧返回"——这正是 C 自己的
  `record_xmalloc` 回退路径的寿命（`lisp.h` 5629 ✓），且 C 的 alloca 路径只会更长寿 ✓；
  全树"`SAFE_FREE` 之后仍引用缓冲"的账由 `tools/check-alloca-lifetime.py` 记（4.436：0 处真阳性 ✓）。

### 3.3 拷贝与未对齐访问

`memcpy`/`memmove`/`memset` → 绑定 libc 同名函数，不用 Rust 的
`copy_nonoverlapping` 替代（C 侧可能有重叠/别名假设），逐处核对。

### 3.4 生命周期

每个结构体字段/全局对象的决策写入 `LIFETIMES.tsv`（`owner`、`freed_by` 非空即为已决）。
**Drop 规则**：可能被 `setjmp`/`longjmp` 跳过的路径上不得有带 `Drop` 的局部值。

---

## 4. 错误处理、GC 与栈

### 4.1 非本地跳转（v2 更正）

C 有三种选择（`lisp.h:2271-2287`），必须按条件编译等价移植：

```c
#ifdef HAVE__SETJMP   typedef jmp_buf sys_jmp_buf;   # define sys_setjmp(j) _setjmp (j)
#elif defined HAVE_SIGSETJMP  typedef sigjmp_buf sys_jmp_buf; # define sys_setjmp(j) sigsetjmp (j, 0)
#else                 typedef jmp_buf sys_jmp_buf;   # define sys_setjmp(j) setjmp (j)
```

- 注意 `sigsetjmp` 的第二个参数是 **0（savemask = 0）**，v1 误写为 1。
- macOS/BSD 走 `_setjmp`/`_longjmp` 分支；Windows 的 `__builtin_longjmp`（`w32fns.c`）
  必须与其调用者使用的 `_setjmp` 变体配对。
- `struct handler` 字段与链表语义、`push_handler`/`unwind_to_catch`/`Fsignal` 控制流逐字保留。
- `panic = "abort"`；不用 `Result` 传 C 返回码；不用 `catch_unwind` 模拟 `catch`。

### 4.2 GC 根与标记

- `staticpro (&NAME)`（273 处）逐处保留；`static mut` 地址必须稳定。
- `mark_*` 调用族（281 处）同名移植；specpdl 的 unwind-array 必须被标记与释放。

### 4.3 保守栈扫描（v2 新增，评审判定为阻断级）

C 依赖的机制必须逐个给出 Rust 等价物，`black_box`/空 `asm!` **不能**替代
`__builtin_unwind_init`：

| C 机制 | 位置 | Rust 决策 |
| --- | --- | --- |
| `SET_STACK_TOP_ADDRESS`（两种实现：`__builtin_unwind_init` 或 `test_setjmp()+sys_setjmp`） | `alloc.c:5190-5203` | 采用 `sys_setjmp` 的 jmp_buf 路径（ABI 保证寄存器刷栈） |
| `test_setjmp()` 启动自检（失败 `exit(1)`） | `alloc.c:5087-5170` | 一比一移植，同一失败行为 |
| `GC_SETJMP_WORKS` / `GC_MARK_SECONDARY_STACK` | `configure.ac:6841`、`alloc.c:5257` | 作为构建期常量，取值与 configure 结果一致 |
| `NEAR_STACK_TOP`、栈增长方向判定 | `thread.h:42-46` | 逐行移植 |
| 防尾调用（`__asm__("")` 屏障） | `lisp.h:4466-4471`（bug#65727） | 在同样位置保留屏障；并规定 GC 可见函数的编译单元禁用尾调用消除 |
| `__builtin_frame_address`（`comp.c`） | `comp.c:2164` | `asm!` 取帧地址，逐处 SAFETY |

### 4.4 dump（pdumper）

- **P-09**：偏移由 `src/dmpstruct.awk` 解析 C 源文本生成 `dmpstruct.h`。Rust 侧必须在
  `xtask` 中重新实现等价的 offset/fingerprint 生成器（解析 Rust 结构体镜像），
  否则 dump 校验无输入。可 dump 对象的布局冻结，不得重排。

---

## 5. Lisp 接口

### 5.1 `DEFUN` → `DEFUN!`（1,841 个活跃 subr，v2 更正；宏名与 C 逐字相同，PORTING 0.2）

C：

```c
DEFUN ("cons", Fcons, Scons, 2, 2, 0,
       doc: /* Create a new cons, give it CAR and CDR as components, and return it.  */)
```

Rust（**docstring 必须是字符串字面量**，v1 用注释的写法没有机械保障——评审实测
`stringify!` 得到 `doc : ,`）：

```rust
DEFUN!("cons", Fcons, Scons, 2, 2, 0,
       doc: "Create a new cons, give it CAR and CDR as components, and return it.",
       (car: Lisp_Object, cdr: Lisp_Object),
       {
           return CALLN! (Fcons, car, cdr);
       });
```

（宏调用**不能被紧跟的项**续写，所以函数体是宏的最后一个实参；字段顺序与 C 相同，
`attributes: noreturn` 写在 `doc:` 之后，`maxargs` 决定联合成员 `.a<maxargs>`，
`defsubr (&raw mut Scons)` 在 `syms_of_*` 里注册——C 的 `defsubr` 会写记录的
`header.size`，所以指针是 `*mut`。）

**`attributes: noreturn` 的记录**：C 把 `_Noreturn` 只写在**声明**上（`globals.h` 的
`_Noreturn EXFUN (Ffoo, N)`），定义仍是 `Lisp_Object fnname (…)`，而 `_Noreturn` 不是
类型的一部分，所以 `{.a2 = Fthrow}` 在 C 里没有类型冲突。Rust 的 `-> !` **是**类型的一部分，
`fn(…) -> !` 不能存进 `fn(…) -> Lisp_Object` 的联合成员（`E0308`）。因此：
- 生成的 `subrs` 原型按 C 的 `_Noreturn` 写 `-> !`；
- `DEFUN!` 的定义也写 `-> !`（与原型同型，P-28 才可校验）；
- **记录**里的指针用 `subr_function_pointer!` 打一次形（等价于 C 的联合初值
  `{.a ## maxargs = fnname}`；两侧都是同宽函数指针且函数从不返回，SAFETY 注在宏上）。
  A6 实测：noreturn 记录在 bin/lib 两种 crate 里都能编译，且记录里存的地址与函数地址
  相同（`address_preserved=true`）。

**多行 interactive 规格的坑**：C 的 `\`+换行是**行拼接**，只删掉换行、保留续行的
缩进；Rust 字符串里的 `\`+换行是**行延续**，会把续行的前导空白一起吃掉。所以把 C 的
多行 `intspec` 原样抄进 Rust 会**改变编译后的字符串值**。规则：多行 `intspec` 在 Rust
里写成一行，用 `\t` 与显式空格把 C 拼接后的值原样表达；`--check` 现在按**解码后的
值**逐字节比较，抄布局会直接 FAIL。

**docstring 校验的粒度**：`--check` 比对的是**词级**摘要（C 的注释排版与 Rust 的字面量
排版不同，空白串会被折叠）；`doc:` 字面量与 C 注释**逐字节一致**的保证由 §7.3 的
`etc/DOC` 逐字节比对在阶段 G 兑现。

- 参数上下限、interactive 规格、Lisp 名逐字一致；`--check` 比对 docstring 摘要（
  `docs/baseline/subrs.tsv` 的 `ldoc_digest`）。
- 若某文件的 `make-docfile` 需要从注释抽取，则必须同时满足“注释与字符串字面量逐字一致”，
  并由 `--check` 机械校验（双轨契约）。
- `struct Lisp_Subr` 布局与 `lisp.h` 一致（含函数指针 union、参数上下限、docstring 指针）。

### 5.2 注册与符号

- `syms_of_*`/`keys_of_*` 同名、`#[unsafe(no_mangle)]`，内部调用顺序与 C 完全一致（顺序影响
  `Qnil`/`Qt` 初始化与 dump 内容）；`defsubr` 1,848 处、`DEFSYM` 2,310 调用逐处保留。
- **`defsubr` 的调用形状**：C 的 `defsubr (union Aligned_Lisp_Subr *)` 会写记录的
  `header.size`（`XSETPVECTYPE`），镜像因此声明为 `*mut`，调用点写
  `defsubr (&raw mut Sfoo);`（`foo` 的定义文件里，顺序与 C 的 `syms_of_*` 相同）。
- **`EXFUN` 的前后缀**：`attributes: noreturn`（8 个 subr，`docs/baseline/subrs.tsv` 的
  `flags` 列）→ 声明为 `-> !`（C 写 `_Noreturn Lisp_Object Ffoo (…)` 并加后缀
  `ATTRIBUTE_COLD`；Rust 无法同时表达"返回 Lisp_Object"与"从不返回"，取
  `-> !`，`ATTRIBUTE_COLD` 只影响优化）。同一个 subr 名可以出现在多个配置分支里
  （1,841 条记录 → 1,736 个名字），生成器按名字合并：签名不同即**报错退出**（照
  `make-docfile` 的 error），`flags` 取并集。
  `attributes: const`（15）/`pure`（6）/`noinline`/`ATTRIBUTE_COLD` 在 Rust 的 extern
  声明上无对应物，只影响优化，逐个记录为不表达（P-25）。
- **`DEFUN!` 定义侧同样要遵守 `noreturn`（评审 F8，§5.1）**：`Fsignal`/`Fthrow` 等 8 个
  subr 的定义必须是 `-> !`，否则定义与 `EXFUN` 原型不一致（Rust 不再检查，见 §0.1-11）。

### 5.3 动态模块 ABI

- `src/emacs-module.in.h`（生成出 `emacs-module.h`）与 `src/module-env-{25..32}.h`
  逐字保留：字段顺序、函数指针签名、版本常量，全部由编译期断言锁死大小与偏移。

---

## 6. 线程、信号、原子（v2 更正）

- Emacs 用自己的 `systhread.c` 原语 + 全局锁；Rust 侧**不引入 `std::thread`/`std::sync`
  语义**，移植到 libc pthread，保持信号掩码、栈大小、取消语义一致。
- **`__atomic_*` 有 24 处**（`android.c` 19、`androidterm.c` 3、`keyboard.c:8452`、
  `xgselect.c:43,55`），且与同一对象的**非原子访问混用**（`androidterm.c:6640` vs `6649`、
  `xgselect.c:46` vs `43`）。移植规则：用 `core::sync::atomic` 的 `load`/`store`/`swap`
  配相同的内存序（`__ATOMIC_ACQUIRE/RELEASE/ACQ_REL/SEQ_CST`），保持同一对象；混用处
  必须逐点核对并记录（这类代码的正确性依赖同一变量上的偏序）。
- 线程局部存储：参照树中为 0 处（`_Thread_local`/`__thread` 均无）。

---

## 7. 构建与配置

### 7.1 `config.h` 是输入（v2 更正）

`emacs/src/config.h` 在检出里**不存在**：必须先对每个目标平台运行 `configure` 并把
`config.status` 冻结（记录到 `docs/baseline/config/<target>.json`），再由 `xtask` 生成
`femacs/src/include/config.rs`（同名常量）与 `--cfg` 集合。没有它，§7.2 的判定没有输入。

### 7.2 `#if` 三分法（v2 更正）

| 类别 | 例子 | 翻译 |
| --- | --- | --- |
| 常量条件（两侧都能编译） | `#if HAVE_PDUMPER` 包住语句 | `if HAVE_PDUMPER { … }`（常量折叠） |
| 结构条件（影响类型/字段/函数存在） | `#ifdef HAVE_X_WINDOWS`、`HAVE_GCC_LABELS` | `#[cfg(feature = "…")]` |
| 包含选择 | `# include <sys/…>` 分支 | `xtask` 决定包含哪个镜像 |

每个 `#if` 都要在 `docs/ports/NAME.md` 标记类别；`#elif`/`#else` 的分支同样逐条处理。

### 7.3 DOC 与生成物

- docstring 抽取由移植后的 `make-docfile` 完成，`etc/DOC` 必须与参照逐字节一致。
- `admin/unidata`、`admin/charsets`、`src/lisp.mk`、`ldefs-boot.el` 由 `xtask` 驱动原
  Lisp 脚本生成，产物逐字节比对。

---

## 8. 属性与内建（v2 更正：宏族才是主体）

| C 宏族 / 属性 | 数量 | Rust |
| --- | ---: | --- |
| `GCALIGNED_STRUCT` / `GCALIGNED_UNION_MEMBER` | 31 / 13 | `#[repr(C, align(…))]` / 显式对齐填充 |
| `EXTERNALLY_VISIBLE` | 39 | `#[no_mangle] pub` |
| `ATTRIBUTE_*`（section/align/pure/const/no_sanitize…） | 112 | `#[link_section]`、`#[repr(align)]`、其余记无操作 |
| `NO_INLINE` / `INLINE` / `EXTERN_INLINE` | 30 / 638 | `#[inline(never)]` / `#[inline]`；`INLINE` 的链接语义见 §0.1-6 |
| `cleanup(…)`（`nsterm.h:281`） | 少量 | 作用域守卫（显式调用），逐个核对 |
| `weak_import`（`macfont.h:88`）、`force_align_arg_pointer`（`w32term.h:36`） | 少量 | 目标平台属性/放弃并记偏离 |
| 变量对齐（`android.h:360`） | 少量 | 显式对齐分配 |
| `DEFINE_GDB_SYMBOL*` | 6 个符号 | `#[no_mangle] pub static`，值语义一致 |
| `DEFINE_LISP_SYMBOL` 生成的 `Qxxx` | 依 `DEFSYM` | 同名 `static mut` + 注册 |

`.gdbinit` 还依赖 30+ 类型/枚举名与 `gdb.lookup_symbol("USE_LSB_TAG")`：类型名保持
一致（§0.2），符号用 `#[no_mangle] pub static` 暴露。

内联汇编 1 处 → `core::arch::asm!`，约束与副作用逐条对应并加 SAFETY。

---

## 9. 未决项（每条都有归属与验证方式）

| 编号 | 未决项 | 现状 | 验证 |
| --- | --- | --- | --- |
| P-01 | `Lisp_Object` 用指针型别名；拒绝 wide-int / CHECK_LISP_OBJECT_TYPE | 已定 | 构建期拒绝逻辑 + 断言 |
| P-02 | `USE_LSB_TAG` 作为 `cfg` 决策，仅支持 true 的 64 位目标 | 已定 | 配置生成器 + 断言 |
| P-03 | computed goto 只移植 switch 版 | 已定（偏离已记录） | 基准对比 + ERT |
| P-04 | 变参：`c_variadic`（stable ≥1.99）+ 锁具体 dated nightly | **已定**（2026-10-02，pin `nightly-2026-10-01`） | `doprnt.c` 试跑 + `spikes/va-probe.sh` 6 行 + `docs/ports/va-arg-types.tsv` 的多重集检查（P-45）+ `gate-probes` |
| P-05 | `alloca`：栈池 + `sa_avail` 记账 + 循环点特判 | **落在 `alloc.rs`（2026-10-04，status 3.113）；spike 仍欠** | 已落地：`sa_stack_alloc`/`sa_stack_release` 两个端口自有入口（`AVAIL_ALLOCA!` 记账后向池要字节），释放挂在 `safe_free` 上，按**严格**深度比较（`depth > 被解开的深度`）释放；两条已记录偏离：① 一次请求一个 malloc 块（不是单一 bump 区——释放是子集，bump 区没法在最后一个块之前还回去）；② 同深度起止的作用域要到更浅的释放才还，池比 C 的栈帧占得多。两个符号是唯一的"端口自有导出"类，两个门（`extract-abi.py` 的 invented 规则、`check-symbols.py` 的 invented 表）都按 `PORT_ONLY` 常量显式放行（其它任何自带 `#[unsafe(no_mangle)]` 而没有 C 定义的名字一律是缺陷）。**第三条偏离（2026-10-05，status 3.626，且是修好加载链的那一条）：池里的块必须由 GC 保守标记**。C 的
`alloca` 是**真的 C 栈内存**，`mark_c_stack` 的保守扫描覆盖它，所以 C 可以放心把 `Lisp_Object` 只放在
`alloca` 内存里（`internal_lisp_condition_case` 的 clause 表就是一处，eval.c 1620）；端口的 `alloca` 是
`sa_stack_alloc` 的 **malloc 块**，任何扫描范围都够不着它 ⇒ 只存在池里的 `Lisp_Object` 会被回收到。
处置：`garbage_collect` 在 `mark_threads ()` 之后、`process_mark_stack` 之前，沿 `sa_stack_top` 对每个块做一次
`mark_memory (base, base + used)`（与 C 对同一块内存的栈扫描等价）。判据：未加之前加载链停在 `macroexp.el`（17 个文件、
崩在打印错误对象上），加上之后同一条命令走到 `characters.el`（34 个文件）。仍欠：内存压力对拍（与 `record_xmalloc` 路径的对拍）与循环点用例，尤其是 `depth == sa_count` 的兄弟/嵌套作用域与恒在深度 0 的分配 |
| P-06 | `static mut` 地址稳定 | 已定 | `staticpro` 断言 |
| P-07 | docstring 用字符串字面量，`--check` 校验摘要 | 已定 | `docs/baseline/subrs.tsv` |
| P-08 | `config.h` 需按目标冻结 | **部分完成**：darwin-arm64（minimal）已冻结；GUI/其他目标待冻结 | `docs/baseline/config/*.json` + `xtask gen-config` |
| P-09 | `dmpstruct.awk` 需在 Rust 侧重实现 | 待办 | dump 双向互换 |
| P-10 | setjmp 三分支 + Windows `__builtin_longjmp` 配对 | 待办 | 目标平台 spike |
| P-11 | 断言族（`clobbered_eassert`/`assume`/`verify`/`static_assert`） | 待办 | 逐条映射表 |
| P-12 | `INLINE` 链接语义与函数指针身份 | 待办 | 地址相等断言 |
| P-13 | 属性宏族与 `cleanup`/`weak_import` | 待办 | 逐条映射表 |
| P-14 | `.gdbinit` 的类型/枚举依赖 | 待办 | GDB 冒烟 |
| P-15 | `ckd_*` 语义逐处核对 | 待办 | 溢出用例 |
| P-16 | 多次求值宏与 release 断言 | 待办 | 宏展开对照 |
| P-17 | `Q*` 符号（由生成的 `globals.h` 提供）在 Rust 侧实现为**同名内联函数** | 已定 | `xtask gen-globals` + `docs/baseline/globals/symbols.tsv` |
| P-18 | `flush_stack_call_func` 的寄存器冲刷用 `_setjmp` 屏障替代 `__builtin_unwind_init` | 已定 | `lisp.rs::flush_stack_barrier`（待 `A7-③` 复核） |
| P-19 | `specpdl`/`specpdl_ptr`/`specpdl_end`/`current_buffer` 在 C 是线程局部宏（`thread.h` 131/135/139/147）；镜像仍声明为 `extern` 存储 | **待办（方案已定，2026-10-04 量过规模）** | thread.rs 已落地且 `thread.h` 镜像已带四个 `macro_rules!`（`specpdl!`/`specpdl_end!`/`specpdl_ptr!`/`current_buffer!`），`lisp.rs:1439` 也已按 C 的 lisp.h 2289 `include!("thread.rs")`——所以 `current_thread` 在所有包含 lisp.rs 的单元里都在作用域内 ✓。收口口径：① 删 `include/buffer.rs:27` 与 `include/lisp.rs:3755-3757` 的四个 `extern` 存储声明；② 读点写 `(current_buffer! () as *mut buffer)`（`m_current_buffer` 是 `*mut c_void`，因为 thread.h 不能命名 `struct buffer`，与 C 的不完整类型一致），写点写 `current_buffer! () = b as *mut c_void`（字段是 place，所以赋值合法）；`specpdl*` 三个字段在 thread.rs 里已是有类型的 `*mut specbinding`，不需 cast。③ 规模（2026-10-04 实测）：**876 处 / 49 个文件**（buffer.rs 158、fileio.rs 105、insdel.rs 63、include/buffer.rs 58、editfns.rs 52、indent.rs 43、coding.rs 40、search.rs 32……），必须在两个在飞单元（keyboard.rs 12、xdisp.rs 6）落地后**一次性**做，否则中间态会把它们的构建打断 |
| P-20 | `get_act_rec`/`set_act_rec` 需要 `current_thread`（thread.h） | 已落地 | 2026-10-05（3.351）：thread.rs 落地后这两条 `INLINE` 访问器进了 `include/lisp.rs`（C 5233-5243），eval.rs 的本地副本删除；其余函数早已全部落地 |
| P-21 | `DEFVAR_*` 的变量在 C 里是 `globals.f_V*`（为调试器聚合）；镜像把它们放进 `pub mod variables`，写成 `unsafe extern "C" { pub static mut V…: T; }` 并 `use variables::*;`，由**属主单元**用 `#[unsafe(no_mangle)] pub static mut V…` 定义（定义遮蔽整体导入，与 `DEFUN` 对原型的关系相同，§0.1-10） | 已定（评审 A 的 F4） | `xtask gen-globals` + `docs/baseline/globals/variables.tsv`；实测：定义方不再 `E0428`，符号是真正的 `_Vprintable_chars`。**不许**在 globals.rs 里就地定义：那会给每个 TU 一份 NULL 副本，第一个内联读者（如 `CHAR_PRINTABLE_P` 读 `Vprintable_chars`）就会解引用空指针 |
| P-22 | `FOR_EACH_TAIL*`/`DOHASH*`/`DOOBARRAY` 是循环宏，Rust 形态（迭代器/闭包）尚待决策 | 待办 | `docs/ports/excluded-macros.txt`（kind=deferred），使用这些宏的文件暂不开工 |
| P-23 | GDB 符号技巧（一个名字既是宏又是常量符号）在 Rust 无对应物 | 已定 | `docs/ports/excluded-macros.txt`（kind=debugger）；`xtask` 后续另行产出调试符号 |
| P-24 | 检查器对头镜像的判定规则 | 已定 | 头文件不做行数漂移判定；结构体字段按**成员集合**比对（顺序由布局断言负责）；C 位域跳过；条件分支中的常量与宏别名不比对；**字符串常量按解码后的字节比对**（C 的 `"\xE2\x80\x98"` 与 Rust 的 `"\u{2018}"` 是同一串）；排除/挂账项见 `docs/ports/excluded-macros.txt` |
| P-25 | `EXFUN` 的 `const`/`pure`/`noinline` 前缀无 Rust 对应物 | 已定（偏离已记录） | 只影响优化；`noreturn` 已映射为 `-> !`（§5.2） |
| P-26 | gnulib/系统头（`lib/stdint.in.h`、`lib/limits.in.h`、`lib/intprops.h`、`lib/verify.h`…）的镜像 | **部分完成**：`stdint.in.rs`/`limits.in.rs` 只含宽度一节（WIP） | 逐头文件补全并让 WIP 计数下降（§7.3） |
| P-27 | 检查器只跟随一层 `include!` | 已定 | 名字（函数/宏/静态量/常量/subr）仍按"文件 + 一层镜像"收集；**桩检测看全部被包含文本**（否则可把实现放进被包含文件绕过），5% 行数预算只看文件自身；桩检测跳过 C 文件里没有的名字（镜像内联函数） |
| P-28 | subr 的"原型 ↔ 定义"检查缺失 | **已完成**（`extract-abi.py`：`DEFUN!` 参数个数 vs `maxargs`、`attributes: noreturn` ⟺ C 的 flags、crate 根不许声明 subr 原型） | 合成用例 + `--check` 回归 |
| P-29 | `globals.rs` 的原型集是"全配置超集"（含 304 个仅 GUI/平台文件才有的名字），与 C 按 `GLOBAL_SOURCES` 生成的 `globals.h` 不同 | 已记录（偏离） | C 里调用未配置的 subr 是编译错误，Rust 里会到链接期才暴露；待 `xtask` 冻结每目标的源文件清单后按配置过滤（P-08 完成后） |
| P-28 | subr 的"原型 ↔ 定义"检查缺失 | **已完成**（`extract-abi.py`：注释无关的匹配、`DEFUN!` 参数**类型** vs `DEFUN_ARGS_*`、`attributes: noreturn` ⟺ C 的 flags、crate 根不许声明 subr 原型、`defsubr` 集合逐条对齐、`DEFUN!` 体也做桩检测、`maxargs` 不可解析时报错；`gate-fast` 现在还编译每个**已完成**的 TU） | 评审 A/B 的 40+ 个攻击用例全部按预期通过/拒绝；残余：doc 摘要为词级（字节级靠阶段 G 的 `etc/DOC`） |
| P-31 | 有 5 个 `S…` 记录（`Sfont_drive_otf`、`Sfont_otf_alternates`、`Sdraw_string`、`Sframe_configuration`、`Srestore_frame_configuration`）由 `font.c`/`frame.c` 注册，但定义在 `.m` 一侧；另有 1 个 DEFUN（`minibuf.c` 的 `Fread_function`）参照从不注册 | 已记录 | `.m` 一侧的 Swift 实现要导出这 5 个记录（或由 Swift 侧注册）；`Fread_function` 照参照不注册；`--check` 按参照文件自己的 `defsubr` 集合对齐，两条都自然覆盖 |
| P-33 | `ITREE_FOREACH` / `ITREE_FOREACH_NARROW`：C 在**展开点**声明迭代器，`_NARROW` 再引用它 | **待定（阻断 `buffer.c` 的 17 处；另 4 个文件各 1–2 处）** | 评审 A 已实测两种可行拼法（body 作为最后一个实参 + `while let Some (n) = it.next ()`，`_NARROW` 变成迭代器方法/宏；或适配器 + `for`）。真正待决的是两处语义：(a) `ITREE_##order` 的 token 粘贴在 `macro_rules!` 无对应物（需 4 臂枚举）；(b) C 每个循环**求值 `t` 两次**（头文件自己的 BEWARE），两种 Rust 拼法都只求值一次（当前调用点都是纯表达式，记为偏离）。代价：22 处调用（`buffer.c` 15+2、`xdisp.c` 2、`editfns.c`/`itree.c`/`textprop.c` 各 1）；`casetab.c` 不受阻 |
| P-34 | 布局基线只覆盖手挑的键（现 207 条，A6 起由 `tools/gen-layout-fields.py` 按"镜像结构体的每个非位域字段"生成） | 进行中（有下限闸门） | 每加一个镜像就在 `struct` 列表里加一项并 `bash spikes/run.sh` 重生成（数字永远来自 C 编译器）；`docs/baseline/layout/counts.tsv` 冻结 `asserted/pending` 下限，类型被改名/删除会让断言数下降并**直接失败**（评审 B 的 F1：类型扫描改为去注释/字符串后按 `\s+` 匹配，`inline-probe` 与 `git` 都能复现） |
| P-35 | 位域的**存储单元宽度**不在闸门覆盖内（`offsetof` 不能指名位域；`u8`/`u32`/`u64` 都让邻居偏移与 `sizeof` 保持 80） | 已记录 | 每个镜像在字段注释里写明单元宽度（`itree_node.bits: u8`，clang 记录布局 72:0-72:2）；`lisp.rs` 现存的 `bits: u32 (+_pad)` 在对应 `.c` 落地时逐个核对（评审 A 的 F2） |
| P-36 | 字符串常量：`cinventory` 记录前就折叠了空白，`"a  b"` 与 `"a b"` 会判等；`enum char_bits`（`lisp.h` 3218-3235）还没进 `lisp.rs` | 待办 | 给 `inventory` 增加"原始值"列并在比较时使用；`CHAR_MODIFIER_MASK` 等在 `character.c` 落地前补齐（评审 B 的 F6 / 评审 A 的 F6） |
| P-32 | arity 表在 `lisp.rs` 里出现两次（`DEFUN_ARGS_*` 镜像宏、`DEFUN!` 的 11 个 arity 臂），`xtask` 生成原型时是第三次 | 已记录（待合并） | 三处可归并成一个来源；当前由 P-28 的类型检查与 `DEFUN_ARGS_*` 清单兜底 |
| P-30 | 现有三道闸门**看不见行为错误**（评审 A 的 P0：`XSYMBOL` 少加 `lispsym` 基址，`--check`/`build-tu`/`layout-check` 全绿却会段错误） | **待办（阶段 B 前）** | 用差分对照把核心算法 TU 与参照对拍（评审 A 的 `/tmp/dt` 手法：C 侧 `cc -std=gnu2x` + mock 运行时，Rust 侧 `rustc --crate-type bin`，同一语料逐字节比对 stdout/标记轨迹；语料覆盖 n=1…5000、6 种键型、`values` 两态、`reverse` 两态、中途 signal/longjmp 与 GC 标记） |
| P-37 | C 的宏/函数**种类**必须保持：对象式宏（`BEGV`/`GPT`/`Z`/`MODIFF`/`FETCH_BYTE (n)`…）在 C 里是**左值**（参照有 159 处写入、11 处 `&MODIFF`、1 处 `FETCH_BYTE (pos) =`），写成 `fn` 就再也表达不出这些语句 | 已定（形状见 `docs/ports/include-buffer.md`） | **C 宏 → 同名 `macro_rules!`**（宏体就是 C 的宏体，因此是 place 表达式），**C `INLINE` → `#[inline(always)] fn`**；代价是调用点写 `NAME! ()` 而 C 写 `NAME`，`PT`/`PT_BYTE` 保留 `+ 0` 以保持"不是左值"。`macro_rules!` 无 token 粘贴：`BVAR`/`PER_BUFFER_VAR_OFFSET` 的调用点写完整成员名（C 名 + `_`）；`GET_OVERLAYS_AT` 把宿主函数的 `sa_avail` 作为第一个实参；`FOR_EACH_PER_BUFFER_OBJECT_AT` 把循环体作为最后一个实参（同 `DEFUN!` 的形状）。 证据：`spikes/write-probe.sh`（C/Rust 两侧同一串写入/取址语句，4 行逐字节一致） |
| P-38 | 头/单元镜像的闸门盲区（2026-10-02 三轮对抗评审共实测 16+13+23 个变异体：冻结闸门对 `casetab.rs` 的语句级语义**零覆盖**） | **主要部分已完成（2026-10-02 第三轮）** | `tools/extract-abi.py` 现在比对：① 常量（含一行式 `enum { … };`，并报 `missing constant`）——当场在 `lisp.h` 找出 8 个从未镜像的常量；② **宏体与函数体的 token 包含**（标识符+数字，折叠大小写/整型后缀/字符串字面量/尾随 `_`/`NULL` 惯用法/本文件类型名，排除 C 形参名与短局部名），刻意不同者记 `docs/ports/macro-body-notes.tsv`（支持 glob 与 `*`）；③ 桩检测改成花括号配对（`{ { unimplemented!() } }` 现在报错）；④ docstring 按 **make-docfile 的值**比对（见 5.1）。实测：`--check` 抓得到评审 C 的 MA/MD/MJ（函数体）与 M6/M9/M10（宏体）、M11b/M14（常量）、M7（嵌套块桩）、docstring 词级改动；`gate-fast` 现在还编译 `--cfg enable_checking` 的一份（`eassert!` 不再被静默丢弃）并跑 `fam/shadow/macro` 三个探针。**仍未覆盖**：同一 token 出现多处、只改一处（存在性而非多重性）、字符字面量（`'z'`→`'y'`）、纯顺序调换（两次 `Faset` 对调、`Qt`/`Qnil` 对调）——需要计数/顺序规则，与 P-30 的行为差分互补 |
| P-39 | `lisp.rs` crate 根的 `unsafe extern "C" { pub fn … }`（74 条，约 62 条是 Emacs C 函数：`make_string`/`intern_1`/`defsubr`/`marker_position`…）在**定义它们的单元**（`alloc.c`、`marker.c`…）落地时会 `E0428`——与历史 `Fcons`/`Ffuncall`/`Fsignal` 同型；`--check` 的 P-28 规则只覆盖 subr | 已记录（落地前必须做） | 把这些声明移进一个 `pub mod`（如 `pub mod c_functions { … }`）再 `use c_functions::*;`，让单元里的定义遮蔽整体导入（P-21/P-28 已验证的形状）；同一文件里再加一条闸门规则，禁止 crate 根出现"将由别的单元定义"的 `pub fn` 原型 |
| P-40 | C 的**柔性数组成员**（`Lisp_Vector.contents`、`Lisp_Char_Table.contents`/`extras`、`Lisp_Sub_Char_Table.contents`、`Lisp_Bool_Vector.data`）在镜像里声明为 `[T; 0]`（这样 `sizeof` 与后续字段偏移与 C 一致），但 Rust 的 0 长数组**不能索引**：`arr[i]` 是边界检查，debug/release 都 panic（"the len is 0 but the index is 0"） | 已定（缺陷已修） | `FAM_ELEM! (place, idx)`：按地址取元素，且展开为 **place**，所以 C 的 `x->contents[i]`、`&x->contents[i]`、`x->contents[i] = v` 三种形状都能一一对应；证据 `spikes/fam-probe.sh`（四个 C 名访问器 + 两个 FAM，4 行；负对照：换回旧写法立刻 panic）。**这七个访问器（`AREF`/`ASET`/`aref_addr`/`gc_aset`/`CHAR_TABLE_REF_ASCII`/`set_char_table_extras`/`set_char_table_contents`）此前一直带着这个 P0**——`--check`/`layout-check` 只看文本与布局，`gate-full` 未实现，所以直到 `casetab.c` 成为第一个真正读 `extras` 的单元才暴露 |
| P-41 | `Q*` 的**索引顺序**与 C 不同（P-17 已记录），但没人记录它的后果：`lread.c` 用 `defsym_name[i]` （**make-docfile 生成进 `globals.h` 的表**）给第 i 个内建符号命名。冻结配置里 C 有 1,560 个符号（`iQcase_table` = 339），port 有 2,118 个（`iQcase_table` = 491），集合包含但顺序不同（评审 A 实测：C 的 1,560 全在 port 的集合里，索引逐条一致） | **已记录（`lread.rs` 落地前必须做）** | `lread.rs` 必须按 **port 自己的顺序**重新生成 `defsym_name` 表，否则每个内建符号会被装上别的名字（静默的全局性故障）；`countof (lispsym)` 的变化也会让 `alloc.c` 的内存计数与参照不同；当前 `defsym_name` 在仓库里 0 命中 |
| P-42 | **构建条件文件的离场边界**：`gmalloc.c`/`ralloc.c`/`vm-limit.c` 在冻结配置下不参与链接（`SYSTEM_MALLOC=1`、`REL_ALLOC` undef → `src/Makefile.in` 489-495 的三个 `*_OBJ` 皆空），而 `atimer.c` **在** `base_obj` 里（`src/Makefile.in:461`；此前“清单里没有”的说法是错的） | **范围已定** | 1:1 以**参照树的文件**为准，不以冻结配置的链接清单为准（`docs/port-plan.md` 第 8 行“每一个 `*.c`/`*.h`”）：这三个文件照样移植，验证只到 `--check` + 自身探针；运行时证据要等一个非 `SYSTEM_MALLOC` 的配置被冻结（P-08） |
| P-43 | 同一仓库三种 lint 写法：`doprnt.rs` 允许 `unsafe_op_in_unsafe_fn`，`sort.rs`/`casetab.rs` 不允许 → 同一状态下 438 条 vs 6 条 warning | **已定**：每个 `femacs/src/**/*.rs` 顶部同一组 `#![allow(…)]`，含 `unsafe_op_in_unsafe_fn` | C 的语句本来就“处处 unsafe”，逐句加 `unsafe {}` 只造噪声；`gate-fast` 不看 warning，噪声正是“新 lint 类无人发现”的来源（评审 D F12）。复核方式：三处 allow 列表 `grep` 一致 |
| P-44 | 同义 token 家族此前用“豁免”处理（`macro-body-notes.tsv` 允许 `memcpy`/`mempcpy` 缺席），而 token 检查是**单向包含**：把 `doprnt` 的三处输出拷贝**全删掉**，`--check` 与 `build-tu` 都绿。第二轮评审（B2）又证明只看**出现次数**不够：`overflowing_add (0)` 改成 `(1)` 仍绿 | **已定并实现**：`docs/ports/token-aliases.tsv` + `extract-abi.py` 的 `alias_count_problems`——家族在两侧折叠为同一拼写，并要求 **Rust 侧出现次数 ≥ C 侧** | 评审 D F2。实测：M2（删 1 处）/M3（删 2 处）/M8（删 3 处）/M1（`rpl_mempcpy`→`strcpy`）全部 `FAIL`，阴性对照（原样）`OK` |
| P-45 | `va_arg (ap, T)` 的**读取类型**此前无任何守卫：`%u` 的 `c_uint` 改成 `c_int` 能编译、所有闸门绿（值流进变参 `sprintf`，编译期看不见） | **已定并实现**：`docs/ports/va-arg-types.tsv` + `extract-abi.py` 的 `va_arg_problems`——按**类型多重集**比对，未记录的类型直接报错 | 评审 D F4。实测变异 MV1（`c_uint`→`c_int`）现在 `FAIL`；`doprnt` 两侧各 12 处读取逐项对应 |
| P-46 | `spikes/shim/stdckdint.h` 的 no-op 宏**影子掉**了系统真实实现（本机 `HAVE_STDCKDINT_H=1`），任何按探针命令构建的 C 参照都会静默失去溢出检测（`%99999999999999999999d` 回绕而不是 `error`） | **已定并实现**：shim 只留 `HAVE_*_H` 为 undefined 的头（现仅 `stdbit.h`），且探针一律 `-idirafter spikes/shim` | 评审 A F3；`spikes/shim/README.md` 写明规则与代价 |
| P-47 | 闸门缺“探针可跑性”这一层：`ec20c48` 让三条探针在仓库根跑裸 `rustc`（解析到 stable）而全部闸门仍然绿 | **已定并实现**：`gate-fast` 增加“探针程序的 Rust 侧编译”；新增 `xtask gate-probes` 跑四个 C 差分脚本，`build/emacs-ref` 缺失时**报错**而不是跳过 | 评审 D F1。实测 `gate-probes`：`inline 6081` / `accessor 11` / `write 4` / `va 6` 行逐字节一致 |
| P-48 | **残余**：端口**自造**的文件级常量只被“记录的偏离”覆盖——`doprnt.rs` 的 `TBUF_LEN = SIZE_BOUND_EXTRA + 50` 改成 `+ 500` 仍然全绿（`50` 被 `macro-body-notes.tsv` 豁免，而 `TBUF_LEN` 不是 C 名，没有东西能比它） | **已记录（批次 B 之前做）** | 建“自造常量 ↔ C 表达式”表 + token 包含检查（与 P-44/P-45 同型）；批次 B 起这类常量会变多（`floatfns.c` 的 `signbit`/`emacs_rint`、`bignum` 的 `GMP_NUMB_BITS`），落地时一并做；在那之前 ledger 必须逐条点名自造常量（评审 A F6） |
| P-49 | `<gmp.h>`（= gnulib `lib/gmp.h` → `lib/mini-gmp.h`）没有镜像，而 `bignum.h`、`floatfns.c` 都要 `mpz_t`：`lib/` 属批次 F（`mini-gmp.c` 4,699 行） | **已定（临时位置已记录）** | 该头现已整体镜像为 `femacs/lib/include/mini-gmp.rs`（292 行：类型 + 4 个宏 + 159 条声明，声明放在 `pub mod gmp` 里以便 `bignum.c` 的定义遮蔽它们），由 `spikes/gmp-abi-probe.sh` 从镜像自身反向生成 C harness 做 ABI 类型核对（159 条全过，改错参数宽度立刻失败），每条上面引 C 行；（批次 F 再落 `lib/mini-gmp.c` 的 4,699 行定义。）`mpz_t` 保持 C 的数组 typedef：`pub type mpz_t = [__mpz_struct; 1]`，因此 `sizeof (mpz_t)`=16、`mpz[5]`=80、`Lisp_Bignum`=24、`value` 偏移 8 全部由布局闸门（211 条断言，`spikes/run.sh` 报 `46 shared, 0 mismatch`）钉住——`sizeof mpz` 这条键的根是**对象**不是类型，`xtask layout-asserts` 因此学会从镜像自己的声明取类型（`static mut mpz: [mpz_t; 5]`），评审 B 的 B1 用它证明过 `[mpz_t; 6]` 在此之前全绿；`mpz_t` 形参在 C 里退化为指针，故声明用 `*mut/*const __mpz_struct`，实参写 `…as_mut_ptr ()/…as_ptr ()` |
| P-50 | `<math.h>`/`<limits.h>` 也没有镜像，`floatfns.c` 需要 22 个 libm 函数、`isnan/isfinite/isinf/signbit` 四个宏、`INT_MIN/INT_MAX`、`HUGE_VAL/FP_ILOGB0`，以及 `DBL_MANT_DIG/DBL_MIN_EXP` | **已定（系统镜像已建）** | 新增 `femacs/lib/include/math.rs`（libm 声明 + 四个宏 + 两个常量），`limits.in.rs` 补 `INT_MIN/INT_MAX`，`float.in.rs` 补 `DBL_MANT_DIG=53/DBL_MIN_EXP=-1021`；常量全部取自冻结目标 clang 实测（命令写在文件里）。`signbit` 取**系统语义**（`is_sign_negative`，含 NaN 的符号位），`floatfns.c` 自己的 `#ifndef signbit` 回到 `feature = "no_signbit"` 的死文本——两者对 NaN 结论相反，取系统语义才是参照二进制的行为 |
| P-51 | 死分支的**文本**也要在镜像里（P-08 只决定哪支是活的）：`emacs_rint`/`trunc` 的 `#else`、`double_integer_scale` 的 `#ifdef HAIKU`、`signbit` 的 `#ifndef` | **已定** | 死文本用 `feature = "no_rint"`/`"no_trunc"`/`"have_haiku"`/`"no_signbit"` 包起来（特性名命名**非冻结**的一侧，默认即冻结配置），每条配 `const _: () = assert! (!cfg! (…) || …)` 绑住冻结常量；`--check` 因此能看到死分支的 token，评审也读得到 C 的原文。**第二轮评审（B3/F2）证明“保留文本”不等于“能编译”**（`no_signbit` 那支把 C 的 `!(x)` 写成 f64 的按位 `!`，根本编不过，且没有断言）：新增 `xtask dead-arms`——把 `femacs/` 复制到 `build/dead-arms/`、翻转对应的冻结常量、开对应 feature，逐个编译已完结的单元（现为 4 个），并由 `gate-probes` 调用 |
| P-52 | C 的**非 `static` 定义就是导出符号**（PORTING.md 0.2），而 `floatfns.c` 的 `extract_float`/`fmod_float`/`double_integer_scale` 被移植成了普通 `pub fn`：`--check`/`build-tu`/`layout-check` 全绿，但目标文件里没有 `_extract_float` 等符号，下一个调用它们的单元（`data.c`/`timefns.c`/`indent.c`/`dispnew.c`/`dbusbind.c`）一链接就失败 | **已定并已修** | 三处（以及后来由新规则抓出的 `syms_of_floatfns`、死分支的 `trunc`）改成 `#[unsafe(no_mangle)] pub unsafe extern "C" fn`，并在 `lisp.rs` 按 lisp.h 补了三条声明（P-39 形状）。`extract-abi.py` 新增一条闸门规则：C 里非 `static`、非 `INLINE`、非 subr 的定义，Rust 侧必须在 `fn NAME` 前有 `#[unsafe(no_mangle)]`（头文件与 `DEFUN!` 生成的除外）；评审 A 的 F1 用 `nm` + `clang -r` 部分链接证明，修后 `nm` 能看到全部五个符号 |
| P-53 | 布局闸门看不见**lib 镜像里的类型**：`known_types()`/`declared_object_types()` 扫的是 `../lib/include`，而 port 的 lib 镜像在 `femacs/lib/include`（从 `femacs/` 看是 `lib/include`）。`mini-gmp.rs` 一落地，`__mpz_struct`/`mpz_t` 的 6 条键立刻从 asserted 掉进 pending（205/13，被 ratchet 挡下） | **已修** | 扫描根同时包含 `lib/include` 与 `../lib/include`；另外 `layout-asserts` 现在**先写 pending 列表再查 ratchet**，失败时也能看到是哪几条键回归了。实测：修复后回到 211 asserted / 7 pending |
| P-54 | `lisp.rs` 的 `other_units` 声明**只比名字不比类型**：`extract-abi.py` 看不见类型，`gen-gmp-abi-probe.py` 只看 mini-gmp，于是 `make_string`/`make_unibyte_string`/`intern_1`/`intern_c_string_1`/`fast_c_string_match_internal`/`egetenv_internal` 六条把 `ptrdiff_t` 写成 `usize`，另有 15 条与参照头类型不一致（评审 B 的 B-2） | **已清空基线（68 条受检）** | 新增 `tools/gen-lisp-abi-probe.py` + `spikes/lisp-abi-probe.sh`：把 `other_units` 的声明反向生成 C，用 `-Werror=incompatible-pointer-types` 对着参照 `lisp.h` 编译。第七轮把 15 条存量全部修掉，基线 `docs/ports/lisp-decl-abi-baseline.txt` **清零**：7 条是“Rust 类型名 = C tag”，生成器现在读 `lisp.h` 的 tag/typedef 表并补关键字（`union Aligned_Lisp_Subr *`、`struct Lisp_Fwd const *`、`const union vectorlike_header *`；typedef 优先——lisp.h 589 的 `struct Lisp_Object` 在未启用的 `CHECK_LISP_OBJECT_TYPE` 分支里）；`allocate_pseudovector` 的 `isize` 改成 `int`、`xnmalloc` 的 `usize` 改成 `ptrdiff_t`；`listn` 补回固定的 `Lisp_Object` 形参（`list!` 随之按 C 的两次求值展开）；`calln` 是宏不是函数（删掉发明的声明，`call0` 改用 `calln!`）；`fclose` 移到 `<stdio.h>` 镜像；两条 `struct buffer *` 用 `/// C:`。**覆盖也随之上升**：`allocate_nil_vector`/`allocate_vector`/`staticpro`/`mark_objects`/`record_unwind_protect_array` 原先被过宽的“含 `Lisp_…` 即不可映射”判据当成未检查，现在都在检查（68 受检 / 5 未映射=全是函数指针形参 / 2 跳过，跳过项现在连理由一起打印） |
| P-55 | 参照里**只以指针出现的不完整类型**（`struct buffer *`）在镜像里没有 Rust 对应物：镜像要能出现在不含 `buffer.h` 的编译单元里，而 Rust 的类型定义全局唯一（写 `pub enum buffer {}` 会与 `buffer.rs` 的 `struct buffer` 撞 `E0428`）。用 `*mut c_void` 又丢掉了类型信息 | **已定并实现** | 该声明上方写一行 `/// C: <参照原型>`（如 `/// C: ptrdiff_t buf_charpos_to_bytepos (struct buffer *, ptrdiff_t)`），`gen-lisp-abi-probe.py` 改按这一行生成 C 侧指针并核对 `lisp.h`：注释写错、参照改签名都会编译失败。Rust 侧 `*mut c_void` 与 C 的指针 ABI 相同，调用点（`buffer.rs`）照旧显式转换 |
| P-56 | `lisp.h` 38 行 `#include <stdio.h>`，5316-5318 在非 Android 配置下取 `# define emacs_fclose fclose`；`fclose` 与 `FILE` 此前被塞在 `lisp.rs` 和 `mini-gmp.rs` 里，而 `mini-gmp.rs` 并不拥有 `FILE` | **已定并实现** | 新增 `femacs/lib/include/stdio.rs`（`FILE` + 系统 `fclose`），由各编译单元按 C 的包含顺序 `config.rs → stdio.rs → lisp.rs` 包含（5 个单元 + 6 个 spike 探针 + `xtask` 生成的布局探针）；`mini-gmp.rs` 删掉重复声明（它本来就要求包含者先有 `FILE`）。实测（第八轮评审 B 的 F6 订正）：`REPLACE_FCLOSE` 不在生成的 `lib/stdio.h` 里，而在 `lib/gnulib.mk` 的 `= 0` 处，它让生成的 `lib/stdio.h` 落在字面量 `#if 0`（953 行）——`rpl_fclose` 与 `_GL_CXXALIAS_SYS` 两支都死，系统 `fclose` 来自 50 行的 `#include_next <stdio.h>`；结论不变（镜像的是系统 `fclose`），`lisp.h` 的 `#include <stdio.h>` 在 34 行不是 38 行（`emacs_fclose` 那两行 5315-5319 是对的） |
| P-57 | 镜像内的函数体调用**排在它后面**的宏，在 crate 根能编译、被 `include!` 进 `mod` 时不能：`call0` 用了 `calln!` 而 `macro_rules! calln` 在其后 570 行 | **已修** | `call0` 按 lisp.h 3462-3471 的顺序挪到 `calln!` 定义之后（两处都是 4 行，行数不变）。§0.1 新增第 18 条把“宏必须先于使用”写成规则；`inline`/`write`/`accessor` 三个探针本来就把镜像 放进 `mod`，因此是这条规则的守门人 |
| P-59 | `tools/cinventory.py` 的 `enum_constants` **只认 `enum [NAME] {` 在同一行**的形式，于是 lisp.h 的 `enum Lisp_Closure`（3206）与 `enum char_bits`（3219）——`{` 在下一行——连成员一起完全没进常量清单；切分成员时又只看逗号不看括号，`CHAR_TABLE_STANDARD_SLOTS` 的值被截断，`NLIMBS_LIMIT` 的嵌套 `min (…)` 里还凭空生成假成员 `lim`；函数内的枚举（doprnt.c 218-225 的 `pDlen`/`pIlen`/`pMlen`/`maxmlen`）也被当成文件级常量要清单。触发者是 `port(keymap.h)` 的 `CHARACTERBITS`（编译不过才发现常量根本不在镜子里） | **已修 + 25 条已补齐** | 扫描改成"注释/字符串剥离后的扁平文本 + 括号感知切分 + 只取 `{` 深度 0 的枚举"（函数内枚举的成员由所在函数体的 token 比对覆盖）。lisp.h 随即暴露 25 条：从未镜像的 14 条枚举成员（`CLOSURE_*` 6、`CHAR_ALT/SUPER/HYPER/SHIFT/CTL/META`、`CHAR_MODIFIER_MASK`、`CHARACTERBITS`）、`Cmp_Bit_EQ/LT/GT` 与 `Cmp_EQ/LT/GT`（C 的隐式值 0/1/2）+ `cmp_bits_t`、`header_size`/`bool_header_size`/`word_size` （C 是 enum 常量，port 是 `const fn`；已改成常量并把 8 处调用点的 `()` 去掉，写法与 C 一致）、`USE_STACK_CONS`/`USE_STACK_STRING`（硬编码 `true`，改成 C 的表达式）。**其中两条值原本是错的**：`SUB_CHAR_TABLE_OFFSET` = 2 而 C 是 1（`PSEUDOVECSIZE (struct Lisp_Sub_Char_Table, contents) - 1`，clang 实测 16 = 8 + 1×8 印证），`FIXNUM_BITS` = `INTTYPEBITS + 1` = **3** 而 C 是 `VALBITS + 1` = **62**：前者让 `set_sub_char_table_contents` 写错槽位，后者让 `bignum.c` 的 `bits <= FIXNUM_BITS` 几乎恒假（能装进 fixnum 的值走成 bignum）并让 `SXHASH_REDUCE` 的归约位宽错到 61。三个常量现在都由 C 的表达式推导（`PSEUDOVECSIZE!`），不再手写数字。根因见 P-60。`PSEUDOVECSIZE` 的宏定义按 P-57 提前到 `static_assert!` 旁边（C 在 lisp.h 3155，晚于用它的常量） |
| P-60 | 文件级 `static_assert` 不在任何清单里：lisp.h 有 **15** 条（121/143/829/1446/1579/2007/2213/2254-2258/2789/3171/4149/4294/5888），`lisp.rs` 只有 1 条（4294 的 `FLT_RADIX`）——正是"没有断言"让 P-59 的两条错值活了下来（C 的编译器当场就拒绝） | **部分已补，闸门规则待补** | 已补 4 条：char-table 的三条（lisp.h 2252-2258，把 `header_size`/`CHAR_TABLE_STANDARD_SLOTS`/`SUB_CHAR_TABLE_OFFSET` 钉在布局上）+ `PTRDIFF_MAX <= ULLONG_MAX`（4149）。其余 10 条：7 条是 `GCALIGNED (…)`/`NIL_IS_ZERO`（`GCALIGNED*` 已在 `excluded-macros.txt`，`NIL_IS_ZERO` 待补）、`static_assert (BITS_WORD_MAX >> (BITS_PER_BITS_WORD - 1) == 1)`（143）与 `POWER_OF_2 (FOR_EACH_TAIL_THRESHOLD)`（5888）应在下一轮补；2789 那条 `static_assert (sizeof x <= 2 * sizeof (hash_hash_t))`（在 `reduce_emacs_uint_to_hash_hash` 内）值等价但没镜像。评审 A 的 F5 还指出：原先把这一行框定成「文件级」断言，而 2007/2789/4149 都是函数内的；另外 `alignof Lisp_Float`/`alignof Aligned_Lisp_Subr` 根本不在 `docs/baseline/layout/c-layout-darwin-arm64.tsv` 里（只有 `sizeof` 键），那两条 `GCALIGNED` 断言没有对应的断言键。**待办**：`tools/extract-abi.py` 增加一条"文件级 `static_assert` 计数/条件比对"规则，否则下一次仍会静默漏掉 |
| P-61 | 镜像之间的 `include!` 构成的是**树**还是 DAG：`buffer.rs` 已经嵌套 `include!("../../lib/include/time.in.rs")`，而 `timespec.h`/`systime.h` 的镜像同样要 `struct timespec`——两个镜像拉同一个 lib 镜像会在同一编译单元里重复定义（`include!` 没有 include guard，0.1-4 说的"xtask 展开包含图并按文件去重"还没实现） | **已定规则 + 本轮按规则落地** | 规则：**每个 lib 镜像只能被"拥有者"镜像或 TU（crate root）引入一次**。本轮 `timespec.rs` 拥有 `gnulib_config.rs`，`systime.rs` 拥有 `timespec.rs` 与 `sys_time.rs`，`time.in.rs` 仍只由 `buffer.rs` 拥有，`sys_time.rs` **不**自己引 `time.in.rs`（靠 TU/上游镜像，与 `src/` 镜像不引 `lisp.rs` 同理）。实测：`gate-fast` 的布局探针把全部 `src/include/*.rs` 编进一个 crate 后仍无重复。**已实现（第八轮）**：`xtask` 加了 `include_graph_ordered`（后序 DFS，依赖在先——镜像的嵌套 `include!` 通常在文件头，它带来的宏必须先于该文件正文，0.1-18）与 `materialize`（把根文件的每个 `include!` 行就地展开成「每个可达文件一份副本」，副本内部的 `include!` 行换成同长度注释以保留行号，另生成 `_root.rs`）；`build-tu`、`build_finished_tus`、`dead-arms` 与布局探针都走它，并显式用 `--crate-name <stem>`。旧规则（「每个 lib 镜像只能被拥有者或 TU 引入一次」）随之**废除**：镜像现在可以自由嵌套。实测（负向）：把 `materialize` 短路回原根文件后，`layout-check` 立刻报 `E0428: the name time_t is defined multiple times`（`buffer.rs` 与 `timespec.rs` 都拉 `time.in.rs`，正是 termhooks/window 都要 `dispextern.h` 的形状）；开着去重时 237 条断言全绿。 |
| P-62 | gnulib 的配置宏不在任何参照头文件里：`_GL_CMP`、`_GL_CONFIG_H_INCLUDED`、`_GL_INLINE*`、`_GL_ATTRIBUTE_CONST`、`_GL_ARG_NONNULL` 都是 configure 从 `m4/gnulib-common.m4` 的 AH_VERBATIM 生成进 config 头的，`lib/timespec.h` 等头文件直接使用它们；而 `femacs/src/include/config.rs` 是 `xtask gen-config` 从**手工挑选的值常量 JSON** 生成的，放不下宏形状的东西 | **已定并落地** | 值常量：加进 `docs/baseline/config/darwin-arm64.json`（`_GL_CONFIG_H_INCLUDED = 1`，src/config.h 38），由 `xtask gen-config` 写进 `config.rs`；**宏形状**的：新文件 `femacs/lib/include/gnulib_config.rs`（`_GL_CMP`，C 的 `(((n1) > (n2)) - ((n1) < (n2)))`，PORTING.md 0.1-5 每宏一家）；纯拼写的：`_GL_TIMESPEC_INLINE`/`_GL_INLINE(_HEADER_BEGIN/END)`/`_GL_ATTRIBUTE_CONST`/`_GL_ARG_NONNULL` 记入 `docs/ports/excluded-macros.txt`（kind=syntax）。`lib/timespec.h` 的 `#error "Please include config.h first."` 用 `const _: () = assert! (_GL_CONFIG_H_INCLUDED, …)` 镜像（不是 `static_assert!`——lib 镜像要能被不含 `lisp.rs` 的探针单独编译） |
| P-63 | 第八轮两名对抗评审（Hooke/Volta）在**已提交**的 `d500727`/`e675757`/`133d837` 上找到：`hash_idx_t` 写成 `isize`（C 是 `int_least32_t`）→ `Lisp_Hash_Table` 88 字节而 C 是 72（P0，更早引入、零闸门覆盖）；`make_uninit_sub_char_table` 少一个 `min_char` 形参并把它钉成 0（P1）；新 `enum_constants` 的 `brace_depth` 会被 `extern "C" {`、多行 `#define` 续行、字符字面量里的花括号带偏（8 个参照文件的枚举成员**静默丢失**，其中包括本轮新镜像的 `lib/timespec.h`）；`/// C:` 注解使被注解声明的 Rust 侧**完全**不受检（改宽度/返回类型/加形参都能过）；61 条 `pub static` 与 3 条 `#[link_name]` 声明对探针不可见；`emacs_fclose` 写成包装 `fn`（C 是对象式宏别名，地址语义不同）；`cmp_bits_t` 应是 `unsigned int`；P-56/P-60 的证据与行号有错；`KEYMAPP`/`list!` 的说明与实现细节有误或缺记录 | **已修 + 已记录** | 逐条：`hash_idx_t = i32`；`make_uninit_sub_char_table (depth, min_char)` 并写回 `min_char`；`flat_code()`（去预处理行含续行 + 去字符字面量）+ 认 `extern "C" {` 的 `brace_depth`——实测全树 557 个参照文件**零丢失**、6 个文件**恢复**成员（`xterm.h` 的 FOCUS_*/`X_ERROR_MESSAGE_SIZE`、`macfont.h`、`image.c` 等）；`annotation_disagreement()`（比 arity 与每个形参/返回类型的宽度）——实测把 `charpos: ptrdiff_t → c_int` 的变异打红；`Option<unsafe extern "C" fn …>` 映射成 `void (*)(…)`，**5 条「未映射」变成受检**（68→**73**条、0 未映射、0 基线）；对象声明改为**计数并打印**（61 条）；`emacs_fclose` 改成 `macro_rules!`；`cmp_bits_t = c_uint`；`keymap.rs` 的 typedef 挪到 C 的位置；P-56/P-60 已订正。`SXHASH_REDUCE` 的签名（C 是 `EMACS_UINT → EMACS_UINT`，port 是 `EMACS_INT → EMACS_INT`，值等价）与 `list!` 把 C 的**未定序**两次求值钉成「先数组后实参」都记进 `docs/ports/lisp.md` |
| P-64 | `lisp-abi-probe` 的剩余盲区（评审 B 的 F2/F3 + 评审 A 的 F1） | **已记录，待做** | ①61 条对象声明只被计数打印，未比类型（C 侧可写 `static MAPPED *p = &NAME;` 配 `-Werror=discarded-qualifiers`——`chartab_size` 的 `const` 就漏在这里）；②`/// C:` 的 pointee 拼写只在**宽度**层面比对（`struct buffer *` ↔ `void *` 是同一 8 字节，可命名 pointee 的拼写差异看不出来）；③Rust 的 `pub type` **定义**（把 `Lisp_Word` 改成 `i32` 探针照样 OK）不受任何闸门检查，只靠布局断言兜底；④`Lisp_Hash_Table`/`Lisp_Obarray` 等类型没进 `tools/gen-layout-fields.py` 的 `STRUCTS`，所以 `hash_idx_t` 的 88→72 只能靠 C 探针实测发现（下一步把它们加进去并重生成 `docs/baseline/layout/*.tsv`，同时更新 `counts.tsv` 的 ratchet） |
| P-65 | `tools/gen-layout-fields.py` 的字段过滤用的是"名字出现在结构体正文里"，所以嵌套 `union`/`struct` 的成员会漏到外层类型上：把 `Lisp_Fwd` 加进 `STRUCTS` 时生成了 `offsetof (struct Lisp_Fwd, intvar)`（`intvar` 在 `union u` 里，不是外层成员），C 探针直接编不过 | **已记录，绕过** | 本轮先把 `Lisp_Fwd`/`Lisp_Buffer_Local_Value` 从 `STRUCTS` 拿掉（`Lisp_Hash_Table`/`hash_table_test`/`Lisp_Obarray` 已加入，211→237 条断言），并在 `tools/gen-layout-fields.py` 里写明原因；**待做**：按 `cinventory` 的嵌套路径（`struct_fields` 的 container/嵌套层）只取第 0 层字段，再把这两个类型加回去 | **已修（第十轮）**：`tools/gen-layout-fields.py` 新增 `direct_field()`——按字段名在结构体正文里的**花括号层数**判断（第 1 层才是该结构体自己的成员，嵌套 union/struct 的成员在第 2 层往后），于是 `Lisp_Fwd`/`Lisp_Buffer_Local_Value` 都能进 `STRUCTS`：断言 237 → **245**，pending 仍 7。顺带修掉一个只有它们才会触发的生成器缺陷：断言是**生成的 Rust**，字段名恰好是 Rust 关键字时要写成 `r#`（`Lisp_Buffer_Local_Value.where` → `r#where`），否则布局探针直接编不过——`rust_ident()` 按 PORTING.md 0.2 的关键字表转义。`glyph`/`glyph_slice` 等 dispextern.h 的类型**随镜像一起**加入（缺类型的键算 pending，而 ratchet 只允许 pending 下降） |
| P-66 | `data.c` 链上的头文件（`dispextern.h` 4026 行、`font.h` 1035、`termhooks.h` 994、`window.h` 1268、`frame.h` 1957、`keyboard.h` 536、`coding.h` 791、`composite.h` 363）**大部分正文在平台/图形分支里**（`HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/`HAVE_NTGUI`/`HAVE_NS`/`HAVE_PGTK`/`HAVE_HAIKU`/`HAVE_ANDROID`/`MSDOS`），冻结配置（P-08：仅 darwin-arm64 minimal）只启用很小的一部分。可 `tools/extract-abi.py` 的 `dead_branch_map` **只认字面 `#if 0`**：整段 GUI 声明的"missing" 都会被报出来，等于要求把 batch E/F（显示与平台后端，约 44 万行）在 batch B 之前搬完 | **已记录，方案已定** | ①（**部分已落地**：同一问题的窄版本——C 在**不同分支里重复定义**同名宏时，`extract-abi.py` 取的是**最后一个**定义（Android 分支的 `No_Cursor 0`、`ANDROID_GC_FOREGROUND`），与冻结分支相反；现在这类宏（`macro_duplicates > 1`，`dispextern.h` 里 9 个）的**宏体不比对**、只要求名字在场，并在报告里打印出来）`dead_branch_map` 改成**按冻结配置求值**：读 `docs/baseline/config/*.json` 的宏表（`#ifdef`/`#ifndef`/`#if defined (X)`/`!`/`&&`/`||`/`== 0` 这些形式求值，其余保持"两支都活"的保守行为），落在关闭分支里的 C 声明**不再要求镜像**，而是汇总成一行 "N declarations deferred to the platform target"（可见，不静默）；镜像里若仍写了它们（P-51 的做法）则照常比对。为让求值对未见过的 `HAVE_*` 也正确，需把 JSON 的宏表从"挑选的 30 条"扩成 `build/emacs-ref/src/config.h` 的**完整** `#define`/`#undef` 集（`tools/gen-config-baseline` 重跑，`docs/baseline/config/*.json` 随之更新，由评审复核）。②镜像策略：这些头文件先镜像**冻结配置的活文本**（其余分支以 `// C: #ifdef … (not mirrored on this target — PORTING.md P-66)` 行标出边界，checklist 里逐段列出未镜像部分，WIP 状态照常可见）；batch E/F 各自冻结目标时再补齐。③顺序：`dispextern.h` → `font.h` → `composite.h` → `coding.h` → `termhooks.h` → `window.h` → `frame.h` → `keyboard.h` → `process.h` → `data.c`（`composite.h` 无条件包含 `font.h` + `dispextern.h`，所以它排在这两个之后）。④包含图已就绪：P-61 的去重已实现并负向验证，`termhooks.h`/`window.h` 都要 `dispextern.h` 的菱形不会再撞 `E0428`；⑤**这套抽取器改动不是开工前的必需**：`--all` 本来就把「ledger 里还有未勾选方框」的文件报成 **WIP**（不是 FAIL，四个 lib 镜像就是这么挂着的），所以镜像只覆盖冻结配置的活文本、把平台段留成未勾选项即可开工，代价只是 WIP 计数上升——①仍是把 WIP 变小的精确化，可与 `dispextern.h` 并行做；**第十轮进展**：`tools/gen-cond-macros.py` 落盘了冻结配置的宏表（`clang -dM -E` 全量 dump + config.h 的 `#undef`，**3527 条**，`docs/baseline/config/darwin-arm64.macros.tsv`）；`tools/cinventory.py` 的 `dead_branch_map` 现在会在装载宏表时按 C 预处理器语义求值（`defined`/`!`/`&&`/`||`/比较/算术/`?:`（只求被选中的一侧）/文件内 `#define` 生效且只在活分支生效/**蓝漆规则**：展开中的宏不再展开，`VAL_MAX`↔`USE_LSB_TAG` 的互相引用因此得到 C 的答案）；`tools/check-cond-eval.py` 是它的**差分测试**（把每个分支首行插进 `enum { EMACS_COND_k = 1 };` 标记再预处理，与求值器逐条比对）。**第十二轮进展：求值器已转正，但只对「被差分证明过」的文件生效**。①求值器修到零不一致：宏表扩成 4956 条（`clang -dM -E` 全量 dump，含 `<sys/time.h>` 等系统头，`docs/baseline/config/darwin-arm64.macros.tsv`），求值器补齐 C 条件求值语义（`?:` → `||` → `&&` → 比较 → 移位 → 加减 → 乘除模 → 一元 → primary）、函数式宏展开（如 `GNUC_PREREQ (4,3,2)`）、`true`/`false`（C23 → 1/0）、蓝漆规则 `_COND_EXPANDING`；`tools/check-cond-eval.py` 逐条与 clang 比**分支活性**，实测 **9 个已验证文件、191 条条件、0 不一致**（1 条 unknown）。②`extract-abi.py` 的 `--cond-macros` **默认打开**，但**只对 `docs/baseline/config/cond-eval-verified.txt` 列出的文件生效**（`--cond-macro-paths`）；该表由 `check-cond-eval.py --write-verified` 写出，`xtask gate-fast` 每轮用 `--verified` 重跑差分，**只升不降**（不在表里的文件保持旧的盲态：除字面 `#if 0` 外两支都活）。**第十三轮修正（评审 A 的 F1/F3/F5）：这道「只对表内文件生效」的范围限定第一版是死代码** —— `_BlindConditions` 只把 `CONDITION_MACROS` 置空，而 `dead_branch_map` 读的是 `CONDITION_MACROS_BASE` 并会用 `set_condition_macros` 把它重新装回去，于是宏表实际作用于**每一个**文件；白名单缺失时还是 fail-open（`None` 表示「每个文件都用」），匹配用的是 `path.endswith` 后缀（`test/manual/etags/c-src/emacs/src/lisp.h` 会命中 `src/lisp.h`）。三处都已修：`BASE` 一并保存/清空、白名单缺失直接 `fail`、按 `reference` 根做**绝对路径恒等**匹配；`c_body_text`/`body_is_empty` 也移进同一个盲态作用域（评审 A 的 F2：它们在 `inventory()` 退出后才跑 `dead_branch_map`，`doprnt.c` 的 `exprintf` 体因此被当成「不存在」而不比对）。修好后的真实数字（同一轮实测，全部仍是 WIP、无 FAIL）：`limits.in.h` 1→**24**、`stdint.in.h` 95→**138**、`sys_types.in.h` 18→**17**、`time.in.h` 24→**32**、`dispextern.h` 159→**239**。`docs/ports/include-{limits.in,stdint.in,sys_types.in,time.in}.md` 是**盲态时代**生成的账本，与现在的闸门不是同一套要求，下次动这些文件时用新抽取器重生成。③为什么必须有这道闸：gnulib 的 `*.in.h` 是**模板**，条件里带 `@WINDOWS_STAT_INODES@` 这类 m4 占位符、引用的是**生成后**头文件的状态，冻结宏表根本无法判定——实测 `stdint.in.h` 40/74、`time.in.h` 72/126、`sys_types.in.h` 14/19、`limits.in.h` 5/20、`mini-gmp.h` 2/5 条不一致，故全部留在表外。这些文件的正确判据是 `build/emacs-ref/lib/` 下**生成后**的头文件（未决）。④打开后立刻抓到 `lisp.h` 里 7 处一直在用**错配置文本**的地方并已修正：`NATIVE_COMP_FUNCTIONP`/`NATIVE_COMP_FUNCTION_DYNP` 极性**反了**（把 `#ifdef HAVE_NATIVE_COMP` 的死文本当活的、把 `#else` 的 `return false` 当死的）、`CHILD_SETUP_ERROR_DESC` 取了 `DOS_NT` 分支（应为 5208 行的 `#else`：`"Doing vfork"`）、`EMACS_INT_WIDTH`/`EMACS_UINT_WIDTH`/`BITS_PER_BITS_WORD` 硬编码 64（应为 `LONG_WIDTH`/`ULONG_WIDTH`/`SIZE_WIDTH`；宽度宏按 C 的归属分放——`INT_WIDTH`/`UINT_WIDTH`/`LONG_WIDTH`/`ULONG_WIDTH` 在 `limits.in.h` 112-122，`SIZE_WIDTH`/`PTRDIFF_WIDTH`/`INTMAX_WIDTH`/`UINTMAX_WIDTH` 在 `stdint.in.h` 727-731）、`lisp_h_Qnil` 记入 `docs/ports/excluded-macros.txt`（P-17）。⑤显示链头文件的 checklist 待用新抽取器重生成（`dispextern.h` 已完成）。⑥**枚举的隐式成员过去完全不比对**（评审 B 的 F1/F2）：`tools/cinventory.py` 的 `enum_constants()` 只收带 `=` 的成员，于是 `enum draw_glyphs_face { DRAW_NORMAL_TEXT, DRAW_INVERSE_VIDEO, … }` 的六个成员与 `glyph_row_area` 的 `ANY_AREA = -1` 都不在 C 侧清单里，`dispextern.rs` 把 4 个值写错、漏掉 `DRAW_INVERSE_VIDEO` 与 `ANY_AREA`，而 `--all`/`layout-check` 全绿。现在按 C 的规则展开隐式成员（第一个是 0，之后逐一 +1；前驱不是字面量就不猜、保持沉默），枚举值进了常量闸门；这一改正立刻抓到 `lisp.rs` 缺的 26 个枚举成员（`SYMBOL_TRAPPED_WRITE`、`Test_eql/Test_eq/Test_equal`、`Set_Internal_Bind` 4 个、`handlertype` 6 个、`nonlocal_exit` 2 个、`gc_root_type` 4 个、`hash_table_weakness_t` 5 个），已按 C 的行号与取值补进镜像 |
| P-67 | 显示类头文件里有一类 C 类型是**匿名 struct 的 typedef**（`typedef struct { … } GLYPH;`、`Emacs_Color`、`Emacs_Rectangle`、`Emacs_GC`、`display_pos`、`it_slice`…）：Rust 的 struct 必须有名字，而 `tools/gen-layout-fields.py` 生成键时只认 `struct NAME` 标签（无标签就只打印一行 warning，C 侧其实可以写 `offsetof (Emacs_Color, red)`）；`--check` 的"类型字段集合"比对也按 typedef 名走 | **已定 + 部分落地** | 命名规则：镜像用 **typedef 的名字**声明 `#[repr(C)] pub struct NAME { … }`（ABI 只看布局），并在 checklist 的偏离项里记这一条。布局键的缺口**待做**：让 `gen-layout-fields.py` 支持无标签类型（用 typedef 名生成 `offsetof (NAME, field)`）。本轮实测的第二个教训：位域存储单元要按 **C 的 alignof** 建模——`glyph_slice` 是 8 字节但**只 4 对齐**，写成 `bits: u64` 加 `#[repr(C, align(4))]` 不成立（Rust 的 `align` 只能升不能降），断言当场 `error[E0080]: evaluation panicked: alignof glyph_slice`；改成 `bits: [u32; 2]` 才同时满足 8/4 | |
| P-68 | 第十二～十三轮这两名对抗评审（`791a8a4`/`acbeb3a`/`9f523e4`/`37f1aed`/`feb9955`）留下的**未修项** | **已全部修掉（round 13）** | ①**闸门健壮性（评审 B 的 F3/F4，P1/P2）**：`Lisp_Fwd` 在布局基线里只有 `sizeof`/`alignof`、**零条 offset 键**（它的 `u` 是具名 union 成员，`cinventory` 把 union 成员扁平化到容器后又被 `direct_field()` 按深度拒掉），于是把 `type`/`u` 对调、甚至删掉 `#[repr(C)]`，`layout-check` 与 `--check` 都仍然绿 —— 修法：给生成器加「为聚合成员发键」的能力（`OFF (struct Lisp_Fwd, u)`），并补一条不变量「非位域成员 ≥2 的类型至少要有一条 offset 键」；`counts.tsv` 的 ratchet 只锁计数不锁键集，删一条 `Lisp_Buffer_Local_Value.where` 再补一条可满足的键就能腾挪覆盖（修法：冻一份排序键列表或摘要，要求键集只增）。②**`native_comp` 的 const 断言是装饰（评审 A 的 F6）**：`cfg!(feature = "native_comp")` 在任何真实构建里恒为 `false`（workspace 没有 `[features]`，`xtask dead-arms` 的 `FLIPS` 也没有它），所以那一对死文本从未被编译过；修法：把 `("native_comp", "HAVE_NATIVE_COMP", "true")` 加进 `FLIPS`。③**当轮已修**：`rust_ident()` 换成完整关键字集（含 `box`/`gen`/`try`，并对 `self`/`Self`/`crate`/`super` 直接报错）；基线的「权威文件」改为只认唯一的 `c-layout-*.tsv` 并把文件名打进日志；行号引用已校正。④**评审确认无害、不必重查的部分**：`feb9955` 的 7 处修正与 clang 活文本逐条一致；`glyph_slice` 的 `[u32; 2]` 与 C 的 8/4 及字节模式一致；基线取值可用 `build/emacs-ref` 重放成**逐字节相同**的产物；`GCForeground`/`GCBackground`/`No_Cursor` 的冻结活值被正确选中；`Emacs_Color`/`Emacs_Rectangle`/`Emacs_GC`/`GLYPH` 的布局与 C 逐项相同（但**都还没有布局键**，属覆盖缺口）。 **round 13 的修法（三条都已落地并负向验证）**：①`tools/gen-layout-fields.py` 增 `EXTRA`（`Lisp_Fwd` 补 `offsetof (struct Lisp_Fwd, u)`；C 的 `type` 是位域，`offsetof` 对它非法，所以只能键 `u`，而 `u` 落在 8 就钉住了位域存储单元在前），并加不变量：`STRUCTS` 里每个类型至少一条 offset 键，唯一例外 `glyph_slice`（四个位域）显式记在 `BITFIELD_ONLY` 里带理由 —— 没有这条，一个"本该有键"的类型可以静默变成零键。基线由 `spikes/run.sh` 从 C 编译器重放（255 键，新增 `offsetof Lisp_Fwd.u = 8`），把 `type`/`u` 对调现在报 `error[E0080]: offsetof Lisp_Fwd.u`（已实测，restore 后绿）。②`counts.tsv` 之外新增 `docs/baseline/layout/asserted-keys.txt` 键集棘轮：`xtask layout-check` 要求每一次仍被断言的键都在表里，键集只增；把 `offsetof Lisp_Buffer_Local_Value.where` 从基线删掉（并把计数下限临时放低以隔离两条棘轮）会报 `layout keys disappeared … update asserted-keys.txt only with a reason`（已实测）。③`dead-arms` 的 `FLIPS` 加了 `("native_comp", "HAVE_NATIVE_COMP", "true")`：这是 `lisp.rs` 里 `#[cfg(feature = "native_comp")]` 那段**第一次**过 rustc，立刻抓到两处拼写错误 —— `NATIVE_COMP_FUNCTIONP` 把 C 的 `!NILP (XSUBR (a)->native_comp_u)` 写成 `… != 0`（类型不符），`SUBR_TYPE` 里字段名是 `r#type` 而写成了 `type_`；`DEFUN!` 的 `@record` 分支还要补 C 用聚合规则零填充的四个 native 字段（C 的初始化器停在 `lisp_h_Qnil`）。修完 5 个 TU 在非冻结分支下全部编译通过。⑤**广域差分的边界（评审 A 的实测）**：`emacs/src/*.h` + `emacs/lib/*.h` 共 202 个头里，求值器对 clang 的真错答有 **471 处**，其中 **20 处是「clang 说活、求值器说死」**这种危险方向（`src/conf_post.h:68`、`lib/cdefs.h:44/49/567/669/796` 等）——它们目前都不在已镜像的文件上，白名单修好后不会被用到，但任何新镜像**必须先进 `cond-eval-verified.txt`**（先跑 `--write-verified`），否则闸门会拿错分支当要求。 |
| P-69 | 显示链上的头文件互相引用**不完整类型**（`dispextern.h` 的 `struct glyph`/`glyph_matrix`/`face` 用 `struct frame *`/`struct window *`/`struct font *`，而 C 只靠"首次使用时隐式声明 tag"；`font.h` 的 `struct font_driver` 同样用 `struct frame *`，`composite.h` 用 `void *font` 绕过） | **已定** | C 里 `struct X *` 不需要 `X` 完整，甚至不需要提前声明；Rust 没有不完整类型，所以镜像按**包含图**解决：`X` 的**定义**放在 `X.rs`（`frame.h`→`frame.rs`、`window.h`→`window.rs`、`font.h`→`font.rs`），只以指针用到它的镜像（`dispextern.rs`、`composite.rs`）**不重复声明**，靠 `xtask` 的 TU 包含图把 `X.rs` 排进同一次编译（与 `lisp.rs` 依赖 `buffer.rs`、`buffer.rs` 依赖 `itree.rs` 完全同一条规则；P-61 的去重保证同一类型只有一个声明处）。**顺序推论**：`dispextern.h` 的 `glyph`/`glyph_matrix`/`glyph_row`/`face`/`face_cache`/`it` 与 `font.h` 的 `font_driver` 都排在 `window.h`/`frame.h`/`font.h` 的对应结构**之后**落地（Rust 声明与顺序无关，但类型必须存在于同一 TU）；`composite.h` 因为 C 自己写成 `void *font` 所以不受这条约束，已先落地。 |
| P-70 | `coding.h` 的 `struct coding_system.spec` 里 `struct ccl_spec *ccl` 指向 **ccl.h**（本头不包含它）：C 里这是"未声明 tag 的不完整类型"，Rust 没有不完整类型 | **已定 + 已落地** | 只作指针用途、且自己的头尚未移植的类型，由**首次命名它的镜像**给一个显式替身（本处 `pub struct ccl_spec;`，零大小单元结构；`*mut ccl_spec` 是薄指针，替身自身尺寸永不进入布局），在 ledger 的偏离项里记一行；等 `ccl.rs` 落地时**同一个提交里删掉替身**（否则一个 TU 里两个同名声明会 E0428）。同样适用于将来的 `struct frame`/`struct window`/`struct font`（P-69 的完整定义优先，替身只在还没轮到它们时使用）。 |
| P-71 | `tools/cinventory.py` 的字段扫描器在**嵌套聚合**处出错（本轮实作 `coding.h` 时发现） | **已修 + 已负向验证** | 两个缺陷：①`STRUCT_OPEN_NAME` 写成 `^(struct|union)\s+(NAME)?\s*$`，`\s+` 是必需的，所以裸的 `union`（无名、行上只有关键字）**开不了帧**；②`}` 开头的行只弹栈、**不发字段**，而 `} spec;` 的 `}` 把**外层 struct 也弹掉了**，于是 `coding_system` 的 `spec` 及其后 **17 个成员**（`safe_charsets` 到 `encoder`）对扫描器完全不存在 —— 616 字节的布局只被 `id`/`max_charset_id` 两个键覆盖。修法：分隔符改可选、`} NAME;` 作为**外层容器的字段**发射（`offsetof (struct coding_system, spec)` 合法，C 允许对聚合成员取偏移）。修完 `coding.h` 的键从 37 条增到 66 条，367 条断言全部通过；`gen-layout-fields.py` 的 `EXTRA` 相应去重（扫描器现在自己能报出 `Lisp_Fwd.u`）。**同类风险**：`window.h`/`frame.h` 里大量 `union { … } u;` 形状，落地时必须确认这些聚合成员都进了 `STRUCTS` 的键。 |
| P-72 | 镜像**只能命名它自己声明的类型**（或经它自己的 `include!` 拿到的类型）：Rust 的类型解析不像 C 的"不完整 tag"，一个只在别处定义、而本镜像引用的名字会让**不包含那个头的单元**编译失败 | **已定 + 已落地** | 规则：①类型的**定义**放在定义它的头对应的镜像里；②`typedef struct X Y;` 这类只有 tag 的 typedef 跟着**定义**走（`KBOARD` 从 `lisp.rs` 移到 `keyboard.rs`，虽然 C 写在 lisp.h 3570）；③如果**声明**所在的头的 ledger 要求该声明留在原地（`lisp.h 5081` 的 `extern struct kboard *echo_kboard;`），则指针退化成 `*mut c_void` 并加 `/// C: …` 注释（该约定已有先例：`Lisp_Subr.kbd`、`struct buffer *` 的两个注解），由使用点显式转换；④需要完整类型的单元必须把对应镜像加进自己的 `include!` 列表（`xtask` 的包含图跟随 `include!`）。**触发实例**：落地 `keyboard.rs` 时删掉 `lisp.rs` 的 `kboard` 替身，`src/bignum.rs`（C 的 bignum.c 不需要 kboard）立刻 E0425 —— 这正是这条规则的来源。 |
| P-73 | `tools/cinventory.py` 的字段扫描器**按行**读声明：跨行的成员声明（`void (*set_window_size_hook) (struct frame *f,\n int width, int height);`）会从参数里造出**幽灵成员**（`struct terminal` 因此多出 `width`/`yoff`/`title`）并**丢掉真成员**（那 51 个 hook 里有 4 个没进任何清单/键） | **已修 + 已实测** | 声明现在**拼到分号为止**再解析（与预处理器的规则一致），幽灵成员消失、真成员回来（`terminal` 从 63 个"成员"变成 73 个真实成员），并顺带找回 `composition_status.carryover`（多行数组声明）。**影响面实测**：修复后 `--all` 仍是 17 done / 7 WIP / 0 failing，布局键 325→326（只有那一个），说明此前没有已完成的文件因此被误判；但**任何多行声明的类型**在没有这条修复前都不会被正确键控。 |
| P-74 | `tools/cinventory.py` 的字段扫描器在**收尾声明符换行**时丢成员：`frame.h 653-663` 写成 `union output_data { … }` + 换行 + `output_data;`，P-71 的 `} NAME;` 规则只认同行形式，于是 `struct frame` 少了 `output_data` 这个成员、那 8 字节的联合体在布局基线里**没有任何键** | **已修 + 已实测** | 收尾规则扩展成：若某行只有 `}`（没有声明符），则把**下一行**当作声明符读取并消费掉（`output_data` 因此进入 `struct frame` 的字段表与键表，688 条断言覆盖 568 字节的整个 frame）。同类写法（`}` 与成员名分行）在 `window.h`/`termhooks.h` 里也存在，但那些是匿名聚合、本就无成员名可键。 |
| P-75 | `tools/extract-abi.py` 的**函数体比较**不区分死活分支：`SET_FRAME_ICONIFIED` 在冻结配置下的活文本只有 `eassert`+`f->iconified = i` 两行，但 `c_body_text` 把 `#ifdef HAVE_WINDOW_SYSTEM` 里的 `XSETFRAME`/`FRAME_WINDOW_P`/`gui_consider_frame_title` 也拿去比 token，等于要求端口实现**本目标从不编译**的代码 | **已修 + 已实测** | `c_body_text` 与 `body_is_empty` 现在按该文件自己的宏状态**丢弃死行**（与 P-66 的求值器同一套判定），比较的才是冻结配置真正编译的函数体。**影响面实测**：`--all` 仍是 17 done / 9 WIP / 0 failing，`frame.h` 的差异从 64 降到 48；此前所有含 `#ifdef` 分支的函数体都在要求死文本（例如 lisp.h 的 `NATIVE_COMP_*` 一族），这条修复把这类"幽灵 token"一次性去掉。 |
| P-76 | `extract-abi.py` 的 `defsubr` 抽取不查条件表（`CONDITION_MACROS_PATHS`）：`data.c` 4360-4362 的 `defsubr (&Suser_ptrp)` 在冻结配置（`HAVE_MODULES` undef）下**不该被期望**，但工具仍报 `missing defsubr`；同一文件的 `DEFUN ("user-ptrp", …)` 在把 `src/data.c` 加进 `cond-eval-verified.txt` 之后**正确地**被丢弃了（于是把死文本移植成 `#[cfg(feature = "modules")]` 反而变成 `the reference file has no such DEFUN`）。 | **已修**（2026-10-02，`port(data.c) XIII`） | 两轮评审（A 的 P1、B 的 F1）独立发现：`SUBRS`/`DEFUN` 走条件表，`defsubr` 走原始文本。修法已按本节记录的方式落地：`extract-abi.py` 的 `defsubr` 抽取先在 `ci.blank_comments` 的文本上算 `ci.dead_branch_map`，落在未选中分支里的调用不再被要求，于是 `data.c` 的 `Suser_ptrp`（`HAVE_MODULES` undef）与三条 `HAVE_NATIVE_COMP` 注册都不再报 `missing defsubr`，而 `data.c` 本身第一次通过 `--check`（OK）。ledger 偏离项的临时办法作废。 |
| P-77 | `no_mangle` 规则是**单向**的、且用固定 400 字符回看定位属性：给 C 的 `static` 函数凭空加 `#[unsafe(no_mangle)]`（多一个导出符号）不报错；属性与定义之间插一段长注释就**误报** | **已修**（2026-10-02） | 两轮评审用 `/tmp` 变异体实测（第二轮评审 B 的 M2）。修法：`extract-abi.py` 改为双向规则——C 的非 static 函数必须有 `#[unsafe(no_mangle)]`，Rust 的 `#[unsafe(no_mangle)]` 也必须对应 C 的非 static 定义（否则报 `invented exported symbol`），两者的属性→`fn` 解析共用一个跳过属性/注释/空白的解析器，不再做字符回看。证据：变异体被报错、`--all` 仍 17 done / 10 WIP / 0 failing。 |
| P-78 | **`DEFUN`（subr）的函数体从未参与 token 比对**：`cinventory.function_spans` 有意跳过 `DEFUN (...)`（C 在下标名位置写的是 `Sname`），于是 `c_body_text` 对任何 subr 都返回 None，而比对循环只遍历 `c["functions"]`。实测（2026-10-02）：把 `Feq` 的 `EQ` 改成 `BASE_EQ`、删掉 `Fsetcar` 的一条 `XSETCAR` 语句，**所有闸门全绿**。 | **已修**（2026-10-02） | 新增 `cinventory.defun_spans`（在注释已抹白的文本上做括号/花括号配对，取 `DEFUN` 第二个实参作函数名），`c_body_text`/`body_is_empty` 与比对循环同时遍历 plain function 与 subr；并修掉“以 `#` 开头的整行被丢弃”导致 Rust `#[cfg]` 语句整块丢失（`floatfns.rs` 的 `Fround` 曾比成空集）。覆盖实测：`defun_spans` 找到 150 个 subr 体（data.c 120 / floatfns.c 25 / casetab.c 5），其中 **82 个**真正参与 token 比对（52+25+5；其余是尚未移植的 subr，在 `rust_body is None` 处跳过；data.c 另有 4 个 DEFUN 被条件表判死，本就不期望）；变更后 `--all` 仍 17 done / 10 WIP / 0 failing，首轮扫出的 3 条里 1 条是工具 bug（已修）、2 条是 C 把 `0`/`1` 传给 `bool` 形参（记入 `docs/ports/macro-body-notes.tsv`）。 **残余盲区（评审 B 实测）**：① token 集合看不见"成对常量互换"（`SYMBOL_VARALIAS`↔`SYMBOL_PLAINVAL`、`>`↔`>=`、`idx <= 0`↔`idx < 0`）与 ≤2 字符标识符（`skip_short`，`EQ`↔`BASE_EQ` 因此不可见）——已移植的 52 个 subr 中有 40 个含这类 token，这些点目前只靠人工复核，等 ERT/差分层的"行为探针"（见 docs/port-master-plan.md §7.2）；② `function_spans`（旧路径）与 `cinventory.defuns`（台账路径）的花括号/引号计数仍是字面量盲的，`defun_spans` 已用 `blank_literals` 修好，另外两处按同一理由待修（全树 1,849 个 DEFUN 实测 0 例触发）；③ 给 `bool` 形参传的 `0`/`1`（casetab 两处）走豁免后，`false`↔`true` 互换不可见，见 notes 行的说明；④ **“token 全在、语义变了”这一类完全看不见**——第四轮评审 B 抓到的实例是 `Findirect_variable` 把 `let mut object = object;` 写在 `if` 块内，`XSETSYMBOL!` 写到被遮蔽的绑定而 `return object` 读的是形参（别名链因此静默失效）；第五轮又实测三例：把 `bits | BLV_FOUND` 换成 `| BLV_LOCAL_IF_SET`、把 `& BLV_LOCAL_IF_SET` 换成 `& BLV_FOUND`、整句删掉位写——根因是 notes 机制只能**减**要求，无法要求 Rust 侧的替代拼写（`BLV_FOUND`/`SYMBOL_*_MASK` 在 C 体里根本不存在）。集合式比对无从发现，只能靠人工复核与后面的行为探针。 **第六轮（2026-10-02）部分收口**：④ 类里“整句删掉位写”的一个实例（`make_blv` 的 `blv->local_if_set = 0`）已由 `docs/ports/token-aliases.tsv` 新增的 `local_if_set`/`blv_local_if_set` 家族钉住——家族自带**出现次数**检查，端口必须至少与 C 体同次数地拼写替代 token，删句即报；`set_internal` 的豁免里随之删掉 `local_if_set`，`make_blv` 的豁免行整行删除。`redirect`/`trapped_write`/`found` 仍在逐函数豁免表里，同一类盲区对它们依旧存在；把家族推广到这三个名字是下一项闸门改进。**第七轮（2026-10-02）实测补充（评审 B 的 30+ 变异体，逐条 CLI 确认）**：本轮三条新豁免行本身恰为差集（不多不少），但集合式检查管不住「C 的 token 位置上端口写了什么值」：① `uninit` 豁免后，`UNINIT` 对应的初始化器（`Lisp_Val_Fwd { value: Qnil () }` / `false`）的值、乃至「它是否还存在」都不在任何闸门视野内（改成 `Qunbound ()`/`Qt ()`、整句删掉都全绿；`qnil` 还被 PLAINVAL 臂里的 `valcontents.value = Qnil ()` 顶替）；② `forwarded` 四处赋值的真值（声明处与臂内）；③ 整段删掉 `redirect` 的 packed-bit 写（写里的 `SYMBOL_LOCALIZED` 被 `match` 臂头顶替、`redirect` 被 dispatch 读顶替 —— 所以「家族化 `redirect`」对这类**删除**变体也无效，家族只提供出现次数**下界**，不提供极性/运算符检查）；④ 把 `SYMBOL_REDIRECT_MASK/SHIFT` 换成 `SYMBOL_TRAPPED_WRITE_MASK/SHIFT`（这类 Rust 自造名在 C 里没有 token）；⑤ `| BLV_LOCAL_IF_SET` → `& ! BLV_LOCAL_IF_SET`；⑥ `continue` → `break`（C 是 `goto start`，`goto`/`start` 已豁免；换出的 `break` 反而满足 C 的 switch-arm `break`）；⑦ `CALLN!` → `calln!`、`Fdelq (tem, …)` → `Fdelq (variable, …)`、`if idx > 0` → `idx < 0`、`if ! (*blv).fwd.is_null ()` 丢 `!`、`&&` → `||`、`SET_PER_BUFFER_VALUE_P` 第三实参 `true`/`false`。两个可落地的机制（本段不擅自引入，记为下一项）：`docs/ports/init-values.tsv`（逐声明点记录初始化值，`va-arg-types.tsv` 同形）与 `docs/ports/bitfields.tsv`（C 位域名 → 镜像 MASK/SHIFT 常量对）；此外评审 A 要求把 `Lisp_Symbol`/`Lisp_Buffer_Local_Value` 的位域布局并入可复跑的 `spikes/write_probe.*`（目前只靠第三轮的 clang+gcc 探针与 ABI 推理），这是三者中最廉价的一项。**第八轮（2026-10-02）新增实测**：`variable-binding-locus` 的 `FALLTHROUGH` 形状（空 `SYMBOL_LOCALIZED` 臂 + `break` 落到共享 tail）有 6 个变异体全绿——空臂改成 `return Qnil ()`/`return Qt ()`、删掉/改成 `continue` 那个 `break`、把 tail 搬进臂里再删掉循环后的 tail、给 `SYMBOL_FORWARDED` 臂加 `continue;`；位域 dispatch 与 tail 判定 6 个（掩码/移位互换、丢掉 `>> SHIFT`、tail 的 `== SYMBOL_LOCALIZED` 反转、两个臂标签互换、`KBOARD_OBJFWDP`↔`BUFFER_OBJFWDP`）；别名臂 `continue`→`break`/删除 2 个；`buf`→`current_buffer` 4 处；`BLV_LOCAL_IF_SET` 的 `!= 0`→`== 0`；`Qt`/`Qnil` 互换。两个新机制记为下一步：`docs/ports/required-tokens.tsv`（**正向**要求：C 有 `FALLTHROUGH` ⇒ 端口必须有 `break`；C 有 `goto start` ⇒ 端口必须有 `continue`）与逐 token 计数下界（`rust_count >= c_count`，可抓全部 4 处 `buf` 站点）；`&&` 不是 token、`buffer` 被 Rust 的 `*mut buffer` 类型名顶替，这两条只有探针层能覆盖。另：评审 B 独立复现了**仓库级既有** CI 阻塞项（非本轮引入），已在后续提交里修完：① `cinventory.struct_fields` 的第二个 `<aggregate>` 分支用了 `type` 而不是兄弟分支的 `c_type`（`gen-lifetimes.py` 读的头部域 54 条、`.c`+`.h` 全域 81 条），`gen-lifetimes.py` 与 `extract-abi.py` 的**逐文件 doc 模式**（生成 `docs/ports/*.md` 的那条路径）都会在第一个这样的头文件上 `KeyError: 'c_type'`；`emit-mirror.py` 实测**不受影响**（只在 `struct_body()` 为 None 的回退分支读该键，这些容器到不了那里）—— 键名已统一；② `gen-lifetimes.py` 重写整表时**不搬运决策列**，第一位填写 `owner`/`freed_by` 的移植者会被任何一次再生抹掉（当前表里决策数为 0，所以尚未造成损失）—— 现在按「行的非决策列」为键搬运，行消失的决策会打印并让运行失败；③ `docs/baseline/*.tsv` 相对当前抽取器过期（最后一次写入是 `7ad447f`）：常量 1,834→**3,898**（+2112/−48，其中约 1,546 条是枚举隐式成员，其余是此前被 `flat_code` 挡住的显式枚举成员）、结构体字段 5,441→**5,764**（+324/−1，其中 84 条是 `<aggregate>` 行，另含 P-73 的多行声明修复与旧扫描器从函数指针形参里造出的 33 个伪字段的移除）、subr doc digest 148 行（只动最后一列、行集不变）—— 已再生，并把 master plan 的逐目录表/合计、`docs/goal.md` 与 PORTING.md 的计数表同步更新；两位评审各自复算了这份分解。剩下唯一的 CI 阻塞项是「CI 无 `emacs/` 检出」（见 master plan 403）。**第九轮（2026-10-02）第二个 P-78 类实例（评审 B 的 HIGH）**：`cinventory._defun_heads` 在 Lisp 名之后只跳过 `", \t"`，于是 `DEFUN ("name",\n       Fname, …)` 这种**换行写 C 名**的（Emacs 自己的排版风格：treesit.c 47/58、xwidget.c 25/36、process.c 9/63、w32fns.c 11/60、xfaces.c 8/33…）全被 `re.match` 落空跳过——全树只有 **1,708/1,849** 个 DEFUN 体能进入比对。已把 `\r\n` 计入跳过集合：treesit.c 11→58、data.c 恒为 120、全树 1,849 = 文本出现数；`--all` 不变（17 done / 10 WIP / 0 failing），因为新可达的体都在尚无端口的文件里——不修的话，那些文件移植时会**静默**失去这一层检查。**第十轮（2026-10-02）实测补充（评审 B，`aref`/`aset` + 四个 helper 的 27 个变异体）**：15 个仍全绿，全部属已知的集合式类别——阈值常量（`0x7f`→`0xff`/`0x7e`，且 `0x7f` 被切成 `0`+`x7f` 两个 token）与第二个 `0x7f` 同形、`idxval < 0`→`<= 0`、`! NILP` 极性、`BOOL_VECTOR_P`↔`CHAR_TABLE_P` 与 `VECTORP`↔`CLOSUREP` 臂互换、两处同名 `ASIZE`/`PVSIZE` 只改一处、三条 `error`（字符串字面量不参与比对，删掉其中两条也不报）、`SREF`/`SSET` 的 cast 宽度、`SREF (array, p)` 改成错的下标（`p`/`c` 只有 1 字符，被 `skip_short` 丢掉）。最便宜的收口仍是这里记过的两条：逐 token 计数下界（可关掉 8 个）与 `error` 的字符串字面量比对；同一段里四个 helper 是反例——它们的必需 token 没有一个 C 计数 > 1，逐个都是唯一承重。 |
| P-79 | **能用 safe 就用 safe**（用户 2026-10-02 明确要求）：允许 `unsafe` 不等于应该到处 `unsafe`——只有 C 的指针/位域/联合体/变参/裸内存这些**没有 Rust 安全替身**的地方才用，局部值、算术、切片、迭代、`Option`/`Result`、`match` 一律写 safe Rust | **规则，立即生效** | 口径：①`unsafe` 只能用来镜像 C 的构造（解引用 `*mut`、读写联合体字段、调用 `unsafe fn`、位掩码访问），**不得**用来跳过 Rust 本可静态保证的东西；②能用一个表达式大小的 `unsafe { … }` 就不要包一整块；③新代码里每一处 `unsafe` 都要能指回它所镜像的 C 构造——C 里没有指针运算/联合体/bitfield 的行就不该出现 `unsafe`；④既有代码不做大规模改写（评审基线是按现有形状验过的），但在**下一次触碰**该文件时把无谓的 `unsafe` 收窄；⑤`femacs/xtask/` 与 `tools/`（与 C 无关的宿主侧代码）一律 safe。当前形状的背景：各单元头部的 `#![allow(…, unsafe_op_in_unsafe_fn)]` 是**刻意**的半径决定（否则每个 `*mut` 解引用都要多一层大括号），因此评审按上面五条检查**新写入**的 `unsafe`，把无谓的记为 LOW；本规则不影响已落地代码的既有形状。 |
| P-80 | **位域成员在 Rust 没有字段**，而同一个 C 成员名（`padding_p`、`tab_line_p`、`width`）同时属于多个已镜像结构体：`token-aliases.tsv` 的家族是全局的，会互相串味，所以位域不能走那条路 | **规则，立即生效** | 存储单元化 + **在使用处点名成员**：①每个位域段在镜像里是一个整型存储单元（`glyph.bits: u64`、`glyph_row.bits: [u32;6]` + 有符号的 `right_fringe_offset: i8` + `bools: [u8;4]`、`glyph_matrix.bits: u8`），其位置由真实编译器的 `-fdump-record-layouts` 与逐字节对拍证明（`offsetof`/`sizeof` 看不见位域）；②读取/写入写成**按成员名分派的访问器宏**，成员名作为宏的第一个参数留在移植文本里，于是 `--check` 的宏体 token 比对照样通过：`glyph_bits! (padding_p, g)` / `glyph_bits! (padding_p, g, v)`、`glyph_row_bits! (enabled_p, row[, v])`、`glyph_matrix_bits! (tab_line_p, m[, v])`、`glyph_slice_bits! (width, s[, v])`；③这套形状**按类型分派**，所以同名成员在不同结构体里互不影响，也不存在宏重定义遮蔽；④证明位域位置是本条的硬要求——两个位置都由真 C 编译器的字节串与 Rust 侧的字节串比对（第 55 轮 `glyph`/`glyph_row` 实测逐字节相同）。 |
| P-81 | **gnulib 的 "extern-inline 载体"单元**：`u64.c`/`timespec.c`/`stat-time.c`/`issymlink.c`/`strnul.c`/`memeq.c`/`streq.c`/`c-ctype.c`/`endian.c`/`byteswap.c`/`stdc_*.c`/`mini-gmp-gnulib.c` 这些 `.c` 的正文只有 license + `#define _GL_<X>_INLINE _GL_EXTERN_INLINE` + 几个 include：**函数体在头里**，这个单元在 C99 inline 模型里的职责是"发出外部定义"，参照对象因此导出头的那些函数 | **规则，立即生效** | ①镜像保留头里的体（`#[inline(always)] pub fn`，不发符号）；②**单元**用一个同名模块把头的函数再导出一次：`mod <stem>_unit { use super::*; #[unsafe(export_name = "<C 名>")] pub unsafe extern "C" fn <C 名> (参数) -> 返回 { return <C 名> (参数); } }`（模块内不限定就递归到自己，所以调用写成 `super::` 下的镜像函数，或让模块只包含导出项）；③`tools/extract-abi.py` 的 P-77 双向规则为此放行：**本文件设了 `_GL_*_INLINE` 时，它在包含的头里定义的非 static 函数算本单元的导出集**（否则 `#[unsafe(export_name)]` 被误报成 invented exported symbol；头的声明若带声明符后的属性，`_GL_*_INLINE` 那一行也直接读名字，如 `string.in.h` 的 `_gl_strnul`）；④权威判定仍是 `tools/check-symbols.py`（`nm -g` 与参照对象逐符号对照，棘轮 `docs/baseline/symbol-pending.txt`）。 |
| P-82 | **带门臂（`HAVE_WINDOW_SYSTEM`/`HAVE_NS`/`NS_IMPL_COCOA`）的结构体布局一直没有读数**：3.257 ②/3.264 ⑥ 把它记成"没有任何编译能验"的墙——那堵墙在 **Rust 侧**（端口的带门臂缺平台镜像、编不过）；C 侧可读 | **规则，立即生效** | 用 `spikes/gated-layout-probe.py 'struct frame' frame.h [c|objc]` 读：把冻结构建生成的 `config.h` 当底、把 configure 会发出的门宏放在命令行（臂：`frozen`、`x11`=`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_X_WINDOWS=1`（走本机 Homebrew 的 X11 头）、`ns_cocoa`=`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1 -DNS_IMPL_COCOA=1`，`--define` 再加自定义臂），clang 以 `-Xclang -fdump-record-layouts-complete` 打印记录布局，探针 TU 建在临时目录、**从不写进 `emacs/`**。①臂是**配置**不是单个宏：`HAVE_WINDOW_SYSTEM` 单独给命时 `dispextern.h` 缺 `Emacs_Cursor`/`Display_Info`，不解析（实测）；②必须用 `-complete`：`sizeof` 本身不会让 clang 打印该记录（`struct ns_display_info` 实测），只前置声明、该臂不完整的类型读作 `parse=no`；③读数落 `docs/baseline/<type>-layout.tsv`（工具 `--write` 生成：臂 × 成员 × 偏移/位），端口的带门臂按这些数写，冻结臂仍按冻结配置的读数验。 |
| P-83 | **Kotlin 目标（`java/**` → `femacs/platform/android/kotlin/*.kt`，40 文件 / 15,157 行）此前没有任何写法约定**，而 JNI 是**按名字**找类/字段/方法 | **规则，立即生效** | ① 路径与命名：一个 Java 文件 → 同名 `.kt`，**同包** `org.femacs`、同类名（包名是本程序自己的，3.503）；端口头是 `// femacs: port of java/… @ fd7afced403.`，**不带**参照的许可块（`check-declarations.py` 扫 `.kt`）。② **JNI 可见的名字一个都不许变**：`FindClass("org/femacs/X")`、`GetFieldID(…, "name", "I")`、`FIND_METHOD(…, "m", "()V")` 全按名字+描述符找；JNI 读的**字段**写 `@JvmField`（Java 的 `volatile` → 再加 `@Volatile`），JNI 调的**方法**保持 `fun`（不许换成属性），并且**禁用 `internal`**（Kotlin 会给 JVM 名加后缀）。实测：`EmacsDirectoryEntry` 编出来是 `public int d_type; public java.lang.String d_name;`，正对 `androidvfs.c` 366-369 的 `("d_type","I")`/`("d_name","Ljava/lang/String;")`；`EmacsHandleObject` 的 `handle`（`android.c` 2082 的 `("handle","J")`）与 `destroyHandle`（2076/3099 的 `"()V"`）同样按名保。③ Java 的 package-private **无对应**：Kotlin 没有包内可见、`internal` 又改名 ⇒ 用 `@JvmField` + 默认可见并在文件里记一句。④ `native` 方法 → `external fun`；`synchronized` 块 → `synchronized (this) { }`、方法级 → `@Synchronized`；`throws E` → `@Throws(E::class)`；Java 的 `@SuppressWarnings("deprecation")` 要写成 **`@Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")`**（两个是**不同**的诊断名：只写前者时"本成员覆盖了 deprecated 声明"仍报，实测 `onCreate`/三个 `onPreferenceClick` 剩 4 条 warning，写全归零，类级即可）；Java 的"续行以 `=` 开头"（`method` ⏎ `= foo(...)`）**不是 Kotlin 语法**（`syntax error: Expecting an element`，必须把 `=` 留在首行尾，实测 7 处）；Java 的局部变量被重赋 ⇒ Kotlin `var`（`EmacsPreferencesActivity.onCreate` 的 `listener` 三次赋值，与③的形参承接同族）。⑤ 空安全：Java 无注解的类型写**非空**；参照在该点允许 null（赋值/实参）时才写可空，逐处按参照判。⑥ **编译验证已有**：`spikes/kotlin-compile-probe.sh FILE.kt…`（**每批收口要跑全目录** `…/kotlin/*.kt` 一次：per-file 探针拿参照的 **Java** 当未落库替身，看不到"端口自己的子类被 `final` 挡住"这类错误——实测 `EmacsLauncherPreferencesActivity : EmacsPreferencesActivity()` 报 `this type is final`，只有全 `.kt` 一起编才发现；所以 Java 的非 final 类落成 Kotlin 时按 ① 的 `open` 规则先判一次）（Gradle 9.7.1 自带 `kotlin-compiler-embeddable-2.4.0.jar` + SDK 的 `android.jar`，离线、无插件下载；只把编译器自带三个 jar 放进 `-cp` 会 `NoClassDefFoundError: kotlin/reflect/...`）。源文件相互引用 ⇒ 一次编译要给全依赖，**先叶子后父**；40 个文件的依赖图与拓扑序见 `docs/baseline/kotlin-order.tsv`。 ⑦ **Java 里可被覆盖的方法在 Kotlin 里要显式 `open`**（Kotlin 成员默认 final；实测：`EmacsPixmap.destroyHandle` 覆盖 `EmacsHandleObject.destroyHandle` 时编译器报"'destroyHandle' in 'EmacsHandleObject' is final and cannot be overridden"）。⑧ **Kotlin 的 `Array<T>` 是不变的**：Java 的 `Rect[]`（元素可为 null，如 `real_clip_rects = new Rect[n]`）要写成 `Array<Rect?>`，否则 `real_clip_rects = clip_rects` 编不过（实测）。⑨ Java 在构造体里**晚于**别的工作赋值 `final` 字段时（`EmacsPixmap` 的 `width/height/depth`），Kotlin 的写法是声明不初始化、在 `init` 的同一位置赋值（`@JvmField val` + init 赋值），以保住那个顺序。 ⑩ **Java 的 `protected` 方法也是包内可见的**（Kotlin 的 `protected` 不是、`internal` 又改名）⇒ 同包别类要调的那些写成 public 并在文件里记一句（实测：`EmacsFillRectangle` 调 `EmacsGC.blitOpaqueStipple`）。 ⑪ **JNI 按名字找的 `static` 方法要 `@JvmStatic`**：Kotlin 的 companion 成员在 JVM 上住在 `X$Companion` 上，C 侧的 `FIND_METHOD(…, "makeClipboard", "()Lorg/gnu/emacs/EmacsClipboard;")`（`androidselect.c` 110-112）会找不到（实测：`EmacsClipboard.makeClipboard`）。 ⑫ **Java 里 `final` 的方法在 Kotlin 里写 `final override`**（Kotlin 的覆盖成员默认 open；实测 `EmacsMultitaskActivity.onCreate/getAttachmentToken` 在参照里都是 `final`）。 ⑬ **赋值不是表达式**（P-83 ④ 的续行 `=` 之外更普遍的一种）：`while ((rc = reader.read (buffer, 0, 2048)) != -1)`、`while ((read = stream.read (buffer)) >= 0)`、`if ((scheme = uri.getScheme ()) != null …)` 三处 Kotlin 全不认 ⇒ 读取提前一行、循环写成 `if (rc == -1) break`，`scheme` 的赋值提前一行（值相同、分支结束都 `return`），实测 5 处。 ⑭ 续行以 `=` 开头本轮又踩 5 处（`EmacsOpenActivity` 的 `list`×2/`tem1`，`EmacsActivity` 的 `defaultParams`/`action`）——`=` 必须留在首行尾。 ⑮ **android.jar 的 stub 带可空注解而 Java 源码里看不出来**：`Uri.getPath ()` 是 `String?`、`Window.getInsetsController ()` 是 `WindowInsetsController?`、`Intent.getParcelableExtra (String, Class)` 是 `T?` ⇒ 在**参照第一个解引用处**保留一次 `!!`（位置与实际解引用对齐）。 ⑯ `Long`/`Int` 不能混**不限于 `==`/`!=`**：位运算 `or`/`and`/`inv` 也不行，`LongArray` 的元素与 `int` 常量组合一律 `.toLong ()`。 ⑰ `as` 有时是多余的：`uri = parcelable as Uri` 在 `if (parcelable !is Uri) continue` 之后被编译器判 `no cast needed` ⇒ 直接赋值。 ⑱ **同名遮蔽是安全的，而且是承接“形参不可重赋”的最佳写法**：`var isFocused: Boolean = isFocused`（遮蔽同名形参）、`var window: Window?`（遮蔽同名属性）实测都不报 warning，比另起 `isFocused1` 更贴参照。 ⑲ Kotlin 的 `String` 没有 charset-name 构造：`new String (names, "UTF-8")` 只能写 `String (names, Charsets.UTF_8)`（编译器原文 `actual type is 'String', but 'Charset' was expected`）。 ⑳ Java 的对象 `==` 要写 `===`：`currentActivity == this`、`menu == lastClosedMenu`、`lastFocusedActivity == this` 都是引用比较，Kotlin 的 `==` 是 `equals`（与已入库 `EmacsDialog.kt:299` 的 `context === EmacsService.SERVICE` 同形）。 ㉑ **接口落地才暴露的跨目标 warning**：Kotlin 检查 override 的形参名与父类型是否一致，而 per-file 探针里父类型是**参照的 Java**（Java 不检查名字）⇒ 只有接口本身落成 `.kt`、全目录收口时才会报出来。实测：`EmacsWindowManager.java:90` 写 `attachWindow (EmacsWindow window)`、`EmacsActivity.java:164` 写 `(EmacsWindow child)`——**参照自身就不一致**；接口按前者写 `window`，收口得 `EmacsActivity.kt:164:35` 一条 warning。处置：**保参照名、计入 warning 基线**（改任一侧都会偏离那份 Java）。 ㉒ **⑮ 的新极性**：android.jar 的 stub 把 `android.app.TaskInfo.baseIntent` 标成**非空** ⇒ 参照里跨 API 的 `info.baseIntent != null` 报 `condition is always 'true'`、`?.` 形式报 `unnecessary safe call`（实测 `EmacsWindowManager.kt:125/408`）；同族基线那 5 条是 always-`false`。处置：**保留参照的判空**（该注解对 API 29 之前的 `RecentTaskInfo.baseIntent` 是假话），按基线 warning 记账。 ㉓ **`object` 是 Kotlin 的硬关键字**：`EmacsWindowManager.java:359/431` 的 `Object object;` 临时量连**变量名**都不能照抄 ⇒ 端口把 `getSystemService` 的返回直接 `as ActivityManager`（`=` 留首行尾）。判据：那两处实测编不过、改后收口 exit 0；而 `EmacsDesktopNotification` 的临时量参照自己叫 `tem`，照抄保留。 ㉔ **Java 在循环/分支外声明、内部赋值**（`EmacsWindowManager.java:337/361/362/428` 的 `info`/`taskToken`/`set`）在 Kotlin 里第二圈就是重赋值 ⇒ 声明必须挪到赋值点（`val` 不可重赋；与 ⑧ 的“形参重赋”同族）。 ㉕ **companion 上 `@JvmField` 的初始化位置**：`MANAGER` 的初始化表达式落在**外层类**的 `<clinit>`（照 `EmacsWindowManager.java:83-86` 的 `static { MANAGER = new … }`）⇒ 写 `@JvmField val MANAGER: EmacsWindowManager = EmacsWindowManager()`；`private static long nextActivityToken` 外部无人读 ⇒ companion 私有 `var` 即可（判据：Java 侧 12 处按名字读 `MANAGER`/`ACTIVITY_TOKEN`/`consumers`/`windows`，收口 exit 0）。 ㉖ Java 的 `static` 方法被**经实例**调用时 Kotlin 会报错 ⇒ 必须改成类名限定（实测 `EmacsView.java:695` 的 `public static int eventModifiers`，调用点 `:531/:545` 写 `window.eventModifiers (event)` ⇒ 端口写 `EmacsWindow.eventModifiers (event)`，收口 exit 0）。 ㉗ **Java 子类里可裸名用的"继承来的嵌套类/常量"，Kotlin 不继承** ⇒ 限定或显式 import（实测 `EmacsView.java:283/:414` 的裸 `MeasureSpec`/`GONE`；端口写 `View.MeasureSpec.…`/`View.GONE`，与已入库 `EmacsDialogButtonLayout.kt:7` 的 `import android.view.View.MeasureSpec` 等价——两种都编过，记为风格偏离）。 ㉘ **`Canvas.clipRect(Rect)` 在 stub 里是"非空"侧**（⑮ 只记了可空侧）：`EmacsView.kt:276` 初次收口报 `argument type mismatch: actual type is 'Rect?', but 'Rect' was expected` ⇒ 在该处补 `!!`（与 `EmacsPixmap.kt:83` 同形）。 ㉙ **Kotlin 没有 `|=` 这类复合位赋值**（不只是"赋值不是表达式"）：Java 的 `info.imeOptions |= X` 要写 `= info.imeOptions or X`（实测 `EmacsView.java:894/897` 两处）。 ㉚ **"`var` 属性永不被智能转换"也适用于跨类的字段读**：读 **另一类**的 `var` 字段同样要显式处理可空（实测 `EmacsView.java:719` 读 `EmacsContextMenu.menuItems` ⇒ 端口写 `menu.menuItems!!.isEmpty ()`）。 ㉛ ⑱ 的"同名局部承接"同样适用于 **`override` 的形参被重赋**：`override fun onLayout (changed: Boolean, …)` 里把 `changed` 重赋要写 `var changed: Boolean = changed`（与已入库 `EmacsActivity.kt:389` 同形）。 ㉜ `@Synchronized` 可以与 `override` 共用，Java 的裸 `synchronized` 块对应 `synchronized (this) { }`：实测 `getBitmap`/`getCanvas`/`explicitlyDirtyBitmap`/`onCheckIsTextEditor`/`onDetachedFromWindow` 用注解，`onLayout`/`swapBuffers`/`onAttachedToWindow` 用块（P-83 ④ 的延续）。 ㉝ **属性名与构造形参同名时，"同名前缀"在三个作用域里赢家不同**：属性初始化式 `@JvmField var parent: EmacsWindow? = parent` 的右值解析成**形参**（⑱ 同款）；`init` 块里 `parent` 也解析成形参（val ⇒ 判空后自动 smart-cast，写 `parent!!` 报 `unnecessary non-null assertion`）；而**方法体**里 `parent` 解析成属性（var ⇒ 不 smart-cast，`parent!!` 必须写且不报）——实测 `EmacsWindow.kt` 的 `init` 3 条 warning（240/241/245）与五个方法里同类 `parent!!` 18 处 0 warning。 ㉞ **局部 `var` 刚被赋非空值就用，会被 smart-cast**：`type!!` 报 `unnecessary non-null assertion`（实测 `EmacsWindow.kt:1558/1616`）⇒ 局部量不必预置 `!!`。 ㉟ `StringBuilder.length` 在 Kotlin 是**属性**：`builder.length ()` 报 `expression 'length' of type 'Int' cannot be invoked as a function`，`> 0` 再连报 `operator modifier is required`。 ㊱ `View.findViewById` 是**泛型**，Kotlin 里必须给类型实参：`ctx.findViewById (android.R.id.content)` 报 `cannot infer type for type parameter 'T'` ⇒ `findViewById<View>(…)`；同族：参照的 `(ViewManager) view.getParent ()` 落成 `as ViewManager?`（保"null 进 null 出"）。 ㊲ `MutableList.listIterator ()` 返回 `MutableListIterator`，不是 `java.util.ListIterator`（import 后者会 `assignment type mismatch`）；用 `MutableListIterator<…>`（`kotlin.collections`，免 import）。 ㊳ **已落库的兄弟目标会改变"参照 Java 方法"的可空性，调用点必须按已落库的签名补 `!!`**：`EmacsView.getCanvas` 参照声明 `Canvas` 而 Java 体 `return null`，兄弟 lane 落成 `fun getCanvas (gc): Canvas?`，而**更早**入库的 `EmacsDrawable.lockCanvas` 是非空 `Canvas` ⇒ `EmacsWindow.lockCanvas` 必须 `return view.getCanvas (gc)!!`（诊断原文 `return type mismatch: expected 'Canvas', actual 'Canvas?'`，与 `EmacsPixmap.kt:86` 同形）。**同族工作流提醒**：探针会连带编**别的 lane 的在飞文件**，所以"新报的错"可能来自兄弟目标的声明——收口报告要注明所依据的兄弟文件 revision（实测 `EmacsWindow.kt` 的在飞版本曾让已入库的 `EmacsActivity.kt:533` 变红：`wmName` 落成 `String?` 而那里读进非空局部；3.358 的正解是**两边都按参照的可空性**，见下）。 |
| P-84 | **`Kept` 类（`docs/port-map.md` 的 `exec/loader-*.s` 六个文件，1,774 行）此前没有写法约定**，而它是唯一一类"目标就是参照文件本身"的目标：`check-declarations.py` 按**后缀**取文件，`.s` 不在表里 ⇒ 六个文件各自的 16 行许可块对每一道门都不可见（3.295 实测：门打印 `walked past 7 file(s) of an unscanned kind`，其中六个就是它们） | **规则，立即生效** | ① **正文逐字节**：`Kept` 文件除首行外与参照逐字节相同（许可块后面那个空行是**正文第一行**）；② **首行是出处行**，注释符取该文件自己的：`<leader> femacs: kept from exec/loader-X.s @ fd7afced403.`（`//`=AArch64、`@`=ARM、`#`=x86/MIPS），只留修订号、**不带**参照的许可块；③ 被替换的行必须是**注释行**，且组装器眼里的输入不变——3.295 用 `clang -target {aarch64,armv7,i386}-linux-gnu -c` 对参照与端口各编一次，三个目标的对象 sha256 相同（MIPS 两个文件本机 clang 没有后端，证据是文本的：删掉的全是注释行）；④ `tools/check-carried.py` + `docs/baseline/carried-files.tsv` 钉三件事（3.299 起这个门同时管 `Kept` 与 `C++` 两类，见 P-85）：参照整文件 sha256、被替换行数（由行数差导出，不手写）、首行（并要求它点名参照路径与修订号），行集**必须等于**地图的 `Kept` 节（地图说 Kept 而基线无行、基线有行而地图不说 Kept，都是失败）；⑤ `.s` 的方言注释由 `check-declarations.py` 读：`@` **任意位置**是注释（ARM；`loader-armeabi.s` 实测 188 行），`#` **只在行首**是注释（ARM/AArch64 里 `#` 还是立即数前缀：aarch64 56 行、armeabi 82 行；x86/MIPS 的 `#` 注释在行中 0 行），两条规则各有自测例。 |
| P-85 | **C++ 目标（地图的 5 个 `.cc` / 8,264 行：`src/haiku_{draw,font,select,support}.cc` + `lib-src/be_resources.cc`）没有写法约定**，而它们是唯一一类要 `#include` 另一个目标的工具链头（Haiku SDK：`<View.h>`/`<app/Application.h>`/`<kernel/OS.h>`/`<SupportDefs.h>`…） | **规则，立即生效** | ① 路径：`emacs/src/haiku_X.cc` → `femacs/platform/haiku/haiku_X.cc`、`emacs/lib-src/be_resources.cc` → `femacs/platform/haiku/be_resources.cc`（地图 `C++` 节就是这五行；3.147 修过 `be_resources.cc` 一次错归类）；② **端口就是文本本身**：目标语言＝源语言，没有要改写的语义 ⇒ 去掉参照的 17 行 `/* … */` 声明块（`fd9cf89` 对 268 个 C 单元的同一处置）、首行写 `// femacs: port of <ref> @ <rev>.`、其余**逐字节相同**——这正是它与 3.31 那些空单元+台账的 C 侧 Haiku 单元的区别：那些要从 C 重写成 Rust，本机没有工具链就无法验证，所以留空单元；`.cc` 不需要发明任何东西，丢掉文本才是信息损失；③ **名字一个都不许变**：`haiku_support.h` 的 178 个原型都是 `extern "C"`（镜像 `femacs/src/include/haiku_support.rs` 是名字表，它的台账写着 haiku_support.cc 的端口就是定义单元）⇒ 同名字、同参数、不加 namespace、**不许给未实现入口留空体**；④ **编译证据只能来自 Haiku target**：本机没有 SDK——实测 `clang++ -std=c++23 -fsyntax-only -I build/emacs-ref/src -I emacs/src emacs/src/haiku_draw_support.cc` ⇒ `fatal error: 'View.h' file not found`（exit 1），而 `clang++ -std=c++23` 本身可用（纯 C++ 片段 exit 0）⇒ 现代 C++ 指**语言标准**而不是重写；SDK 头是另一个目标的工具链头（与 NDK 的 `<jni.h>` 同类，3.148），**不许为它发明 stub 头**；⑤ 可验的是文本与名字：`check-declarations.py` 读 `.cc`（3.299 起 `femacs/platform` 的 `.cc` 半边第一次被读）、`check-target-names.py` 认首行、`tools/check-carried.py` + `docs/baseline/carried-files.tsv` 钉住参照整文件 sha256 / 被替换行数 / 首行 / 正文逐字节（`C++` 行写 `port of`、`Kept` 行写 `kept from`，两类各 4 例自测）；⑥ 将来有了 Haiku SDK/工具链，正确动作是把 `clang++` 挂到这批文件上（那时它已经是端口本体），不是重写它们。 |

**P-38/P-24 说明（docstring 的值，2026-10-02 第三轮）**：C 的 `doc: /* … */` 注释不是它的值——
`lib-src/make-docfile.c` 的 `read_c_string_or_comment` 会**丢掉注释首尾空白**，并把 `\X` 读成 X（`\n`/`\t` 等控制字符除外），
所以 C 里的 `\"extra\"` 在 `etc/DOC` 里是 `"extra"`。port 的 `doc:` 字面量必须**解码到同一个值**（评审 A 用本机真实
`etc/DOC` 逐字节验过 casetab.c 的 5 条），而闸门也必须按这个值比对：`cinventory.doc_comment_value` 是唯一实现。
树里有 1,838/1,849 个 DEFUN 注释在 `*/` 前带尾随空白、139 个含 `\`+非 `n/t` 转义，按旧约定写会让几乎每个 docstring 与参照不同。
空白差异仍被 `digest` 的空白折叠吞掉（P-36），词级差异会报 `subr …: docstring differs`。

**探针作者注意（2026-10-02）**：把镜像包进 `mod m { include!(…) }` 时，**带 `#![allow]` 的单元文件（如 `casetab.rs`）会报
"an inner attribute is not permitted in this context"**；探针要么把镜像当 crate root（此时 `lispsym` 之类的外部符号改用嵌套模块定义），
要么去掉那行内部属性。`macro_rules!` 的路径在**调用点**解析，所以探针的调用处必须把 `Fcons`/`c_void`/`list1`… 放进作用域。

**P-17 说明（实测决定）**：参照用 `make-docfile` 生成 `globals.h`（`#define iQfoo N` +
`DEFINE_LISP_SYMBOL (Qfoo)`），使 `Qfoo` 成为**编译期常量表达式**。Rust 无法把符号数组地址
折进常量，因此只有两种选择：`static mut Qfoo`（每个使用点都要 `unsafe`，在 590 个文件的
规模上不可接受）或**同名内联函数** `Qfoo()`（无 unsafe、值相同、名字相同，仅调用形状多一对
括号）。采用后者；`lisp.rs` 像 C 的 `#include <globals.h>` 一样 `include!("globals.rs")`。
符号集合与 Lisp 名冻结在 `docs/baseline/globals/symbols.tsv`（2,118 个，`Qnil` = 0、`Qt` = 1，
其余按确定性顺序；顺序本身是我们的生成器内部细节，dump 保存的是对象而不是索引）。

## 10. 逐文件自检清单

1. `python3 tools/extract-abi.py emacs/<dir>/NAME.c > docs/ports/NAME.md`（生成清单）。
2. 文件头、命名、§1 类型映射、§0.2 声明/定义规则全部遵守。
3. `#if` 逐个标记三类（§7.2）。
4. 位域/union/函数指针/变长结构体/alloca 点按 §1.3/§3.2 处理并记录。
5. 错误路径（`setjmp`/`longjmp`/`goto`）按 §2.2/§4.1 保留。
6. GC 根与分配路径按 §3/§4.2 检查（`staticpro` 是否齐全）。
7. `DEFUN`/docstring/注册顺序按 §5 检查。
8. `python3 tools/extract-abi.py --check emacs/<dir>/NAME.c femacs/<dir>/NAME.rs` 必须 OK。
9. 提交前冻结工件并记录哈希；评审看的是提交后的固定版本。
10. 删除 C 的前向声明（§0.1-12）、把"分支内逐字段赋值"改成先整体零值初始化（§0.1-13），
    并在 `docs/ports/NAME.md` 的偏离项里写明；定义一律 `#[unsafe(no_mangle)]`（§0.2）。
11. **镜像里的每个内联函数都要与 C 的"函数体"逐字比对，不能只看名字**：评审 A 在
    `sort.rs` 上发现 `XSYMBOL` 少了 `XBARE_SYMBOL` 的 `+ lispsym` 基址（返回的是编码
    偏移而不是指针），`--check`/`build-tu`/`layout-check` 三道闸门全绿却会段错误。
    镜像改动用可运行探针验证；行为级差分见 P-30。
12. **gnulib 的 `_GL_*_INLINE` 包装 `.c`（`lib/stdbit.c`、`lib/streq.c`、`lib/strnul.c`
    …）的判据**：这类文件的内容只有 LGPL 头 + `#define _GL_X_INLINE _GL_EXTERN_INLINE`
    + 两个 include，作用是 C99 inline 模型里"由这一个单元发出外部定义"（冻结配置的
    `_GL_EXTERN_INLINE` 就是两个字面关键字 `extern inline`，宏表 2104）。端口的函数是
    各只有一份定义的普通 Rust 项，所以 **端口保留的是宏名和它与 `_GL_EXTERN_INLINE` 的
    关系**：`lib/include/gnulib_config.rs` 里 `_GL_EXTERN_INLINE` 是 marker 常量，
    包装文件里 `pub const _GL_X_INLINE: () = _GL_EXTERN_INLINE;`，函数体本身落在对应的
    `lib/include/<header>.rs` 或 `src/` 镜像里。这类文件**不写 `// femacs:` 文件头**：
    C 自己只有 20 行左右，漂移闸的 ±5% 容不下额外的头注释——出处写在 `docs/ports/<stem>.md`。

13. **gnulib 的"标识符重命名"与"包含式 shim"**：①gnulib 的头会把被替换的函数名重命名
    （`<string.h>` 在 `HAVE_MEMPCPY` 未定义时把 `mempcpy` 变成 `rpl_mempcpy`），所以
    `lib/mempcpy.c` 的定义在 C 目标文件里发出的符号是 `rpl_mempcpy`。端口保留 C 的函数名
    （台账按名字比对）并用 `#[unsafe(export_name = "rpl_mempcpy")]` 发出同一个符号；
    `extract-abi.py` 的"这个函数是否导出"判据同时接受 `no_mangle` 与 `export_name`
    （Rust 禁止两者并用），反向判据（导出 C 没有的符号）不变。②`lib/dtoastr.c`、
    `lib/nstrftime.c`、`lib/strtoll.c` 是**包含式 shim**：`#define LENGTH 2` /
    `#define my_strftime nstrftime` / `#define QUAD 1` 之后 `#include "ftoastr.c"` /
    `"strftime.c"` / `<strtol.c>`，即同一份源码在不同宏绑定下产出不同符号。Rust 没有
    解析前的文本宏替换，所以共享源码要先落成 prelude 里的 `macro_rules!`（`DEFUN!` 的
    同型机制）或按绑定参数化，再由每个 shim 实例化一次；这三个文件因此排在共享源码
    （`ftoastr`/`strftime`/`strtol`）之后落地，台账要逐实例记名。
