# femacs 进度与未决项（手工维护，生成物见 port-map.md / port-inventory.md）

本文件是**唯一**手工维护的进度账：每轮开工前先读它，收尾时更新它。逐文件的勾选清单在
`docs/ports/*.md`（由 `tools/extract-abi.py <c>` 生成后手勾），文件/行数总账由
`tools/gen-port-map.py`、`tools/gen-port-inventory.py` 生成，禁止手改。

## 1. 当前数字（每次提交后核对）

- 已竣工单元与镜像：**权威清单在 `docs/port-inventory.md`（590 个参照 `.c/.h` 的状态列）与
  `python3 tools/extract-abi.py --all` 的输出**，本节只记总数。截至 `a0caa61`：**已提交单元 320 个**
  （`femacs/src/*.rs` 149 + `femacs/lib/*.rs` 122 + `femacs/lib-src/*.rs` 5 + `femacs/exec/*.rs` 5 +
  `femacs/lwlib/*.rs` 6 + `femacs/oldXMenu/*.rs` 29 + `femacs/nt/*.rs` 4）、**已提交镜像 143 个**
  （`femacs/src/include/*.rs` 49 + `femacs/lib/include/*.rs` 94）；其中 48 个是本轮按 3.96 落的**未编译空单元**。
  `docs/port-inventory.md` 复算：**590 files, done=402, wip=32, todo=156**（本轮 3.96 之前是 done=344/todo=216）。；未提交的在飞单元见下面「第三批」一条（`window.rs`/`coding.rs`/`keyboard.rs`）。
  镜像这一批（2026-10-04，第三批后续）：`src/include/` 的 `getpagesize.rs`（整文件死）、`sysstdio.rs`
  （27 个单元 include 它）、`sysselect.rs`、`syswait.rs`、`gnutls.rs`、`systty.rs`，以及系统镜像
  `lib/include/termios.rs`（新）与 `lib/include/sys_select.rs` 补的三个访问器；每条各自 `--check` OK、
  `layout-check` 通过，commit 见 `fbc5dbe`/`e9e3c1f`/`76be712`/`5535266`/`a09f8c9`/`8c06329`/`79b6db5`/`a0caa61`。
- 镜像（`src/include/` 与 `lib/include/`）：`font.h`、`thread.h`、`comp.h`、`macros.h`、
  `category.h`、`tparam.h`、`sys_resource.h`、`lisp.h`、`frame.h`、`window.h`、`termhooks.h`、
  `dispextern.h`、`buffer.h`、`keyboard.h`、`coding.h`、`composite.h`、`itree.h`、`globals.h`
  等，以及 `lib/include/` 的 `sys_stat.in`、`fcntl.in`、`unistd.in`、`stat-time`、
  `sys_types.in`（含 `struct stat` 需要的六个 Darwin 宽度别名与 `uid_t`）。第 55 轮新增
  **`src/include/termchar.rs`（termchar.h 257 行，`--check` OK）**，并给 `dispextern.rs` 补
  `Mouse_HLInfo`/`SCROLL_INFINITY`/`CHECK_MATRIX`、给 `frame.rs` 补 C 同名读法
  `pub fn output_method`。
- **进度（2026-10-04 第 98 轮第二批）**：已提交 **268 个单元**（`femacs/src/*.rs` 146 +
  `femacs/lib/*.rs` 122）与 **131 个镜像**。本轮第二批入树：**`src/fileio.rs`（f170ddc，5,987 行）**、
  **`src/alloc.rs`（5665819，8,186 行）**；`src/dispnew.rs`（8,570 行）已竣工待入树、`src/fns.rs` 在飞。
  **整树门（`11be128`）**：条件求值器一致、`inventory: no differences`、globals **268 单元 / 0 缺定义**、
  `cargo check: ok`、**534 个单元两种配置全部 built**、符号门 lib **78/0（1 pending）**；
  未定义引用门报 **8 NEW**（全是未端口单元的依赖：coding.c/fns.c/xdisp.c/dispnew.c/keyboard.c），
  于是**按已提交状态重取了棘轮**（`123 pending`，11 条 stale 因 fileio/alloc 定义了自己的 `DEFVAR_*` 载体
  而消失），重取后单跑该门是 `534 unit object(s), 123 pending, 0 new`；门的其余步骤（符号门 src
  **0 mismatched / 0 pending**——已提交状态 67 个单元、主树含在飞的 `dispnew.rs` 为 69 个；连接门 0 缺失、
  declaration gate 0、mirror-dupes gate 0、`layout-check` passed）也已逐条跑过。**总进度**：590 个参照 `.c/.h` 里已端口 268 个单元 + 131 个镜像。
- **进度（2026-10-04 第 99 轮）**：`src/coding.rs` 收口入树、`lib/include/boot-time-aux.rs`（3.104）成为镜像后
  **已提交单元 323 个**（`femacs/src/*.rs` 150 + `femacs/lib/*.rs` 122 + `femacs/lib-src/*.rs` 7 +
  `femacs/exec/*.rs` 5 + `femacs/lwlib/*.rs` 6 + `femacs/oldXMenu/*.rs` 29 + `femacs/nt/*.rs` 4）、
  **已提交镜像 145 个**（`femacs/src/include/*.rs` 49 + `femacs/lib/include/*.rs` 95 +
  `femacs/lib-src/include/*.rs` 1）。`docs/port-inventory.md` 最近一次复算（3.104 之后）：
  **590 files, done=415, wip=32, todo=143**（该文件的状态列按工作树取，所以里面也含**未提交**的在飞项：
  `keyboard.rs`/`xdisp.rs` 与 `femacs/nt/inc/sys/` 的 8 个镜像）。**在飞（未提交）**：
  `femacs/src/keyboard.rs`、`femacs/src/xdisp.rs`、`femacs/nt/inc/`（16 个 `nt/inc` 镜像的前 8 个已写好、
  台账名冲突待裁决，见 3.100 之后的待办）。
- **照抄树（Verbatim）已全部落地（2026-10-03 第 79 轮）**：`emacs/{lisp,test,etc,doc,leim,admin}`
  → `femacs/<同名>`，**逐字节照抄**，共 **4,607 文件 / 约 4.6M 行**（lisp 1,627 / test 1,555 /
  etc 887 / doc 257 / leim 28 / admin 253），六个目录各一次签名提交并推送。判定不再是行文说明
  而是机器门：**`python3 tools/check-verbatim.py`**（文件清单 + 逐块 sha256 流式比对，越界即非 0），
  当前输出 `6 trees, 4607 files, 0 failures`。`test/` 与 `lisp/` 一个字节都没动（硬约束 5）。

- **进度（2026-10-04 第 98 轮第三批）**：`ab943c4` 时**已提交单元 270 个 / 镜像 135 个**。本轮三批之后
  入树的有：`src/fns.rs`（7,307 行）、`src/dispnew.rs`（8,542 行）、`src/emacs.rs` 与四个 `lib/` 单元的注释
  清理（`ff55b60`，3.90）、**`src/include/macuvs.rs`（9,238 行，唯一的 73,655 字节 IVD 表，`const` 而非
  `pub static`，与 clang 逐字节比对通过）**，以及台账里同类的复述清理（`ab943c4`）。
  **整树门（`af01731`，`EXIT=0`）**：`checked 374 ported files: 351 done, 23 in progress, 0 failing`、
  `inventory: no differences`、`cargo check: ok`、符号门 lib `78 units compared, 0 mismatched, 1 still pending`
  与 src `69 units compared, 0 mismatched, 1 still pending`、未定义引用门 `538 unit object(s) … 112 pending, 0 new`、
  连接门 `0 with 0 missing symbol(s)`、`8 declaration sites (0 forbidden, 8 allowed literals), 0 failures`、
  `layout-check passed`。**对 `ab943c4` 的整树门正在跑**（`/tmp/gate-ab943c4.txt`，worktree `/tmp/fv8`）。
  **在飞（未提交）**：`window.rs` 192,720 B（C 1–4905 / 9,740，尚不能编译，Peirce 接着做）、
  `coding.rs` 141,582 B（C 1–4,576 / 12,312，能 `built`，`--check` 还剩 142 条 missing，Noether 接着做）、
  `keyboard.rs` 7,357 B（14,682 行，Hume 刚开工）。这三个单元收口前**不提交**——它们的台账还有未勾项、`--check` 未满，提交等于把未完成的清单当成品。
- 每次提交的验收证据（缺一不可）：`build-tu`、`gate-fast`、`gate-probes`、
  `extract-abi.py --check`、手工 `rustc --cfg enable_checking`、`--all`、新鲜度链
  （`gen-port-map`/`gen-baseline`/`gen-lifetimes` 后 `git diff --exit-code`）。

## 2. 每轮固定流程

1. 读本文件 §3，挑一个**依赖已就绪**的目标；
2. 读 C 源码 → 写 `<unit>.rs`（同名、同序、同语句；`PORTING.md` 为准）；
3. `cargo run -p xtask -- build-tu <C>`（不频繁编译：一轮一次，除非编译器抓错）；
4. `python3 tools/extract-abi.py --check <C> <rs>` 到 **OK**（差异按 §3 的既有类别处理：
   宏/关键字豁免进 `docs/ports/macro-body-notes.tsv`，死宏进 `excluded-macros.txt`，
   重命名进 `token-aliases.tsv`）；
5. 生成并勾选 `docs/ports/<stem>.md` 与镜像的 `include-<stem>.md`，写 Decisions；
6. 冻结配置（若 `build/` 被清理）→ `gate-fast`、`gate-probes`、`rustc --cfg enable_checking`；
7. 更新本文件 §1/§3 → 再生生成物 → **签名提交并推送** → 核对 `verified=true` → 清理
   `build/`、`/tmp/femacs-target`、`femacs/target`。

## 3. 未决项（定案前不要碰相关文件）

- ~~3.1 `tparam.c` 的行数漂移~~ **已解决（2026-10-03）**：新增
  `docs/ports/line-drift-tolerance.tsv`，逐文件声明**结构化展开**造成的、超过 5% 的行数
  容差（`tparam` 0.40、`itree` 0.10，各附理由），`tools/extract-abi.py` 按该表判定；
  逐函数 token 比对不受影响，它才是真正的守卫。tparam 的唯一未勾选框已清，`--all` 记 done。
- **3.2 `vm-limit.c` 的两条死臂未被编译过**：`gate-probes` 的 `FLIPS` 没有列出
  `windowsnt`/`msdos`；臂内已各自声明所需平台入口，补 flip 即可编译核对（注意 C 的
  `#if/#elif` 是互斥链，三条臂已用 cfg 守卫成"同时只有一条活"）。
- **3.3 gnulib 包含式 shim**（`dtoastr.c`/`nstrftime.c`/`strtoll.c`）：
  见 PORTING.md 10.13②，须先落共享源码（`ftoastr`/`strftime`/`strtol`）并把共享体写成
  prelude `macro_rules!` 或参数化，再实例化；在此之前不碰这三个文件。
- **3.4 `lib/allocator.c` 需要 `allocator.h` 镜像**（`struct allocator`），尚未建。
- **3.5 `lib/binary-io.c` 整文件在 `#if defined __DJGPP__ || defined __EMX__` 之内**：
  这两个宏不在冻结宏表里，求值器保守地要求两支都活，且 `FLIPS` 无对应开关——移植它需要
  先定"未知条件"的既有政策（P-66 已记录该保守行为），否则会产出永不被编译的死文本。
- **3.7 `P-33` 的 `ITREE_FOREACH` 形状已定案（2026-10-03）**：在**使用点**展开 C 宏
  所代表的代码（`if (!t) {} else` 守卫 + 就地声明的 `itree_iterator` + 由
  `itree_iterator_next` 驱动的循环），`ITREE_FOREACH_NARROW` 直接调用
  `itree_iterator_narrow`。首个使用者是 `src/itree.c`；其余四个（buffer.c、editfns.c、
  textprop.c、xdisp.c）照此办理。`excluded-macros.txt` 两行已从 deferred 改为 syntax。
- **3.8 带早退/捕获调用者局部变量的语句宏形状已定案（2026-10-03）**：`marker.c` 的两个
  `CONSIDER`（C 100-136、277-315）实测证明 `macro_rules!` **不能引用调用者的局部变量**
  （卫生性报 `cannot find value ... not accessible due to macro hygiene`）。定案：该语句序列
  写成**一个 helper 函数**——急切求值的操作数当值传入，**惰性求值的操作数当 `FnOnce`**（保住
  C 注释里"BYTEPOS 只在需要时求值"的语义），宏捕获的游标变量走 `&mut`，宏里的 `return` 变成
  `Option` 并由使用点的 `if let Some (value) = … { return value; }` 返回；宏名进
  `excluded-macros.txt`，token 进 `token-aliases.tsv` 的 family（带**出现次数**校验，丢掉一次
  调用仍会失败）。同类宏（`CALLN` 之外的早退宏）照此办理。
- **3.9 gnulib 死臂的保留方式已定案（2026-10-03）**：`lib/*.c` 里的 `#if !HAVE_X` 死臂**照 P-51
  保留文本**——用命名"非冻结一侧"的 feature 包起来（`no_malloc_0_nonnull`、`no_malloc_ptrdiff`、
  `no_malloc_posix`、`need_sanitized_realloc`、`no_realloc_0_nonnull`、`no_realloc_posix`），
  这样 `--check` 能看到死臂 token（含 `ckd_add` family 的出现次数）而台账保留原文；它们不进
  `xtask dead-arms`，因为 FLIPS 只能翻转 `config.rs` 里的键，而这些 gnulib 宏不在其中（与
  `marker_debug` 同类）。`<errno.h>` 的镜像见 `femacs/lib/include/errno.rs`（`errno` 是
  `(*__error ())` 的访问器、`ENOMEM` = 12，取自冻结目标 clang）。
- **3.10 抽取器两处修正（2026-10-03，json.c 暴露）**：① `function_spans` 的**花括号计数**此前
  用只去注释的文本，C 的 `case '{':`（json.c 1565）被当成真的花括号，导致 `json_parse_value`
  的 span 一路吃到文件尾、其后 8 个函数（含 `syms_of_json`）从函数表里消失，并连带产生
  "invented exported symbol" 误报；现在计数改用 `blank_literals` 之后的视图（长度/行号不变）。
  ② `static` 判定原为"上一行以 static 开头"，C 802 的 `NO_INLINE static void` 因此被判成非
  static；现在用 `\bstatic\b` 在该行任意位置匹配。两处都改了 `docs/baseline` 的 C 侧清单
  （inventory 14 行、totals、LIFETIMES），已随生成物一起提交。
- **3.11 `src/kqueue.c`(550)的 libc 面已实测（2026-10-03，尚未建镜像）**：`struct kevent` 在
  冻结目标上 size=32，字段偏移 ident@0 / filter@8 / flags@10 / fflags@12 / data@16 / udata@24
  （用 clang 编一段量偏移的 C 得到）；文件只用到 `EV_SET` 与 11 个常量：`EVFILT_VNODE`=-4、
  `EV_ADD`=0x0001、`EV_ENABLE`=0x0004、`EV_CLEAR`=0x0020、`NOTE_DELETE`=1、`NOTE_WRITE`=2、
  `NOTE_EXTEND`=4、`NOTE_ATTRIB`=8、`NOTE_LINK`=0x10、`NOTE_RENAME`=0x20、`NOTE_REVOKE`=0x40。
  台账：3 个 subr + 6 个函数 + 2 个 static 对象；实现方式与本轮 `signal.rs` 同（新镜像
  `femacs/lib/include/sys_event.rs` 记常量与结构体，unit 里声明 `kqueue ()`/`kevent ()`）。
- **3.12 `src/kqueue.c` 暂缓（2026-10-03 结论）**：`HAVE_KQUEUE` 在冻结配置里 **undef**
  （macros.tsv 807），`src/Makefile.in` 185 行只在 `HAVE_KQUEUE` 时编 `kqueue.o`，所以整个
  unit 在冻结构建里根本不参与编译，它依赖的 `FILE_NOTIFY_EVENT`（termhooks.h 291，也在
  `#ifdef USE_FILE_NOTIFY` 之内，而 `USE_FILE_NOTIFY` 是 undef）无法按现有镜像体系表达——
  照 P-51 硬写只会得到永不被编译、也无法验证的文本。**决定：暂缓**；前置工作已留在树里
  （lisp.h 的 `list5`/`report_file_error`/`report_file_notify_error`/`emacs_open`/`emacs_close`、
  keyboard.h 的三个声明、`sys_resource.h` 的 `RLIMIT_NOFILE`=8、sys_stat/fcntl 镜像），
  等出现一个 `HAVE_KQUEUE` 为真的目标配置时再落地。以下细节留给那次：
所需声明已进镜像——lisp.h 的 `list5`、
  `report_file_error`、`report_file_notify_error`、`emacs_open`、`emacs_close`（`/// C:` 行由
  lisp-abi-probe 校验，现为 **143** 条）,keyboard.h 的 `add_read_fd`/`delete_read_fd`/
  `kbd_buffer_store_event`。unit 侧还要处理:`input_event.kind` 在端口是 `bits`（kind:16|part:16），
  所以 `event.kind = FILE_NOTIFY_EVENT` 要按掩码写（掩码名取自 termhooks.rs）；`EVENT_INIT` 的
  签名是 `(*mut input_event)` ✓；`O_NONBLOCK`=4、`O_EVTONLY`=0x8000、`O_SYMLINK`=0x200000、
  `O_RDONLY`=0、`O_NOFOLLOW`=0x100、`RLIMIT_NOFILE`=8 均已实测；`directory_files_internal` 仍需
  一条声明；`kqueue_compare_dir_list` 里的 `goto the_end` 用 Rust 的带标签块 `'the_end: { … }`
  表达；`struct kevent`/`EV_SET`/11 个常量随 unit 一起写（不另建镜像，便于随 unit 一次验证）。
- **3.6 下一批候选**：依赖已就绪且体量大的 `lib/mini-gmp.c`(4,699)、`lib/regexec.c`(4,306)
  与 `lib/regex_internal.c`(1,718)、`src/json.c`(1,851，0 个预处理条件)、`src/itree.c`(1,435)、
  `src/callint.c`(933)、`src/marker.c`(874)；小件 `lib/getgroups.c`(121)、`lib/group-member.c`(108)、
  `lib/pselect.c`(115)、`lib/rawmemchr` 同族的 `lib/memrchr`(已做)。需要先建镜像的：
  `src/cm.c`(467，需 cm.h/sysstdio.h/termchar.h 三镜像)、`src/decompress.c`(349，需 md5.h)。
  注意 `lib/gettimeofday.c`/`pselect.c`/`pthread_sigmask.c`/`symlink.c`/`readlink.c`/`strtoimax.c`
  在冻结宏表下整支可能为死文本，动它们前先核 `docs/baseline/config/darwin-arm64.macros.tsv`。

- **3.13 sys/stat 一族镜像已落地（2026-10-03）**：新增 WIP 镜像
  `lib/include/sys_stat.in.rs`（实测 `struct stat` 144 字节与 17 个字段偏移、`S_IF*`/`S_IS*`、
  `UTIME_*`、stat/lstat/fstat/fstatat/chmod/fchmod/fchmodat/faccessat/mkdir(at)/mknod(at)/
  futimens/utimensat/umask 的声明）、`lib/include/fcntl.in.rs`（`O_*`、`F_*`、`AT_*`，
  实测值；`AT_EMPTY_PATH` 该目标不定义，所以 fstatat.c 自己 `#define` 成 0）、
  `lib/include/unistd.in.rs`（`ssize_t`=long 与 fd/文件族声明）、`lib/include/stat-time.rs`
  （stat-time.h 的九个 `_GL_STAT_TIME_INLINE` 函数，darwin 活体是 `st_*timespec` 成员，
  `stat_time_normalize` 用 `overflowing_add` 表达 `ckd_add`，P-15）。
  **`chmod` 只声明一次**（在 sys_stat.in.rs）：C 在 <unistd.h> 与 <sys/stat.h> 都声明它，
  Rust 一个模块只有一个命名空间，重复声明是 E0428——这是镜像层的"单一定义点"（0.1-3）。
  由此落地七个 unit：`lib/lstat.c`、`lib/fstatat.c`、`lib/readlink.c`、`lib/symlink.c`、
  `lib/readlinkat.c`、`lib/cloexec.c`（含 `lib/include/cloexec.rs` 镜像）、`lib/filemode.c`
  （含 `lib/include/filemode.rs` 镜像；HAVE_DECL_STRMODE=1，其 `ftypelet`/`strmode` 是 P-51 死文本，
  活体 `filemodestring` 调系统 `strmode`）。这些文件因把 P-51 死臂写全而声明了行数容差，
  见 line-drift-tolerance.tsv。
  **同一条经验**：C 的同一函数名在两个 `#if` 臂里各定义一次时（cloexec.c 的 `set_cloexec_flag`），
  死臂要写成**同一个函数内的 `#[cfg]` 块**（malloc.c 的形状），不能另起一个函数名——否则
  清账会报"函数体没提到 dup2/EBADF/errno"并且多出一个 invented exported symbol。
- **3.14 工具修正（2026-10-03）**：`xtask build-tu` 原先把任何参数都映射成 `src/<stem>.rs`，
  `lib/` 的 unit 因此无法单编（只能靠 gate-fast 的 `finished_units`）；现在按 `finished_units`
  的同一规则在 `src/`/`lib/` 里找目标，`build-tu lib/fstatat.c` 可用。
- **3.15 在飞工作的工作区纪律**：`extract-abi.py --all` 把"目标 .rs 已存在但没有清单"判为
  **failing**（只有清单里还有 `- [ ]` 才是 wip）。因此一个文件一落盘就要有 `docs/ports/<stem>.md`
  （头镜像用 `include-<stem>.md`），哪怕先只生成未勾选的占位清单。

- **3.16 常量比较接受整型后缀（2026-10-03，工具修正）**：`tools/extract-abi.py` 的
  `norm_const` 原来对**十进制**字面量做精确文本比较，于是 `regex.h` 76 的
  `# define RE_BACKSLASH_ESCAPE_IN_LISTS 1ul` 只有把 Rust 写成 `1ul` 才能过——而 rustc 直接报
  `invalid suffix 'ul'`，这个宏在端口里**不可能**满足；十六进制那批（`0xffffffffUL` 等）因为
  不匹配字面量正则而落到"表达式 token"分支，只能逐条进 `macro-body-notes.tsv`。现在
  `norm_const` 统一剥掉 `[uUlL]+` 并把字面量归一成十进制值（`1ul` → `1`，`0xffffffffUL` →
  `4294967295`），语义就是比较想表达的东西，同时让那批十六进制 notes 行变成冗余。
  影响：`lib/regex.h` 镜像转 OK（`--all` 的 done 计数 +1），sha*/mini-gmp 的检查不受影响。

- **3.17 下一批（2026-10-03 收尾时的状态）**：
  - **依赖已就绪、体量最大**：`lib/regex_internal.c`(1,718) / `regcomp.c`(3,814) /
    `regexec.c`(4,306)。它们的头镜像 `lib/include/regex_internal.rs` 已完整落地并通过
    `--check`（90 个 macro_rules!/const、三个枚举、regex 内部全部结构体、12 个
    static-inline 定义），`lib/include/regex.rs` 也已转 OK；**注意**这三个 `.c` 不在
    `cond-eval-verified.txt` 里，所以抽取器对它们用盲条件：`#ifdef _LIBC` 的 glibc 内部代码
    （regcomp.c 369 行、regexec.c 3,195 行等）全被判为活，需要按既有政策决定用
    P-51 cfg 保留文本还是进 notes（这是动手前必须先量的一件事，量法：
    `python3 tools/extract-abi.py --check <c> <空 rs>` 看缺项）。
  - **接着做**：`lib/strftime.c`(2,446) + `lib/mktime.c`(582)（time.in.rs/sys_time.rs 已有，
    缺的声明往镜像补）。
  - **`at-func.c` 模板**（fchmodat.c/faccessat.c 与已落地的 fstatat.c/readlinkat.c 共用）：
    现有政策是"死臂写成 cfg 的 macro_rules + 模板体进 macro-body-notes.tsv"；若将来要移植
    GNU/Linux 目标，需要把它改成 prelude `macro_rules!` 或参数化函数（PORTING.md 10.13②
    的"包含式 shim"条款）。
  - **`lib/fsync.c` 不要在冻结配置下动**：它的非 Windows 臂是 `#error`，整个 unit 在
    darwin-arm64 上没有活体文本。

- **3.18 镜像的 `use` 约定暴露的两个真问题（2026-10-03 修）**：
  ① 镜像按 0.1-9 不能写 `use`，于是它用到的 `c_int`/`c_char`/`c_void` 全靠**包含它的 unit**
  提供；只要镜像新增一个用到新别名的项，老 unit 就可能编不过（本轮 strftime 的
  `timezone_t = *mut c_void` 让 `futimens.rs`/`getline.rs` 红了，stdio 镜像新加的 `fread`
  让 `getline.rs` 红了）。处置：凡**闭包文本里出现某别名而 unit 的 `use std::ffi::{…}` 没写**
  的，一律补齐（本轮补了 futimens/getline/lstat/gettimeofday/mempcpy）；同时把新加的
  stdio 项写成 `core::ffi::c_void` 绝对路径，并在 `time-internal.rs` 里也这么做。
  ② 多个镜像各自开 `pub mod decls` 会撞名（`md5.rs` 与 `stdio.rs` 同在一个 unit 时 E0428），
  所以每个镜像的整导入模块必须**唯一命名**（`stdio_decls`/`save_cwd_decls`/`filemode_decls`/
  `cloexec_decls`）。
- **3.19 常量家族（token-aliases）的两条边界（2026-10-03 修，regex 簇暴露）**：
  ① 家族成员必须把**端口的拼法**也列进去（regex 侧的 `type` 家族要含 `type_`/`set_type_`，
  否则 lisp.h 的 `type` 形参在端口写成 `type_` 后计数为 0 而报红）。
  ② 家族计数**不看**被 `*` 备注全豁免的条目：`macro-body-notes.tsv` 里 `*` 行意味着该宏体
  在端口没有对应拼法（lisp.h 的 `XUNTAG` 一族），此时"出现次数下限"没有可数的对象，
  `tools/extract-abi.py` 的宏循环现在与函数循环一致地跳过（否则 grep 不到 `type` 的宏体会红）。

- **3.20 `#if` 求值器补全 + 已核验文件从 18 扩到 43（2026-10-03，工具修正，本轮最大杠杆）**：
  `tools/cinventory.py` 的求值器补齐三处——①一元 `+`/`-`（C 把负常量写成 `(-1)`，如
  strftime.c 229 的 `# define FAILURE (-1)`）；②条件运算符 `?:`（`eval_condition` 原来一见 `?`
  就答"未知"，而 `HAVE_NATIVE_TIME_Z` 的宏值就是三元式）；③宏值里的 `?:` 展开不再提前 bail；
  ④**死分支里的条件一律记 False**（`regexec.c` 225 的 `SHLIB_COMPAT (…)` 位于 `#ifdef _LIBC`
  之内，clang 判死而旧求值器答"未知"）。
  随后用 `tools/check-cond-eval.py` 逐条对拍 clang，`cond-eval-verified.txt` 由 18 个文件扩到
  **42 个**（新增 `lib/strftime.c`(115 条件)、`lib/mktime.c`、`lib/time_rz.c`、`lib/nstrftime.c`、
  `lib/regcomp.c`、`lib/regexec.c`、`lib/regex_internal.c`、`lib/utimens.c`、`lib/utimensat.c`、
  `lib/fchmodat.c`、`lib/faccessat.c`、`lib/getopt.c`、`lib/fcntl.c`、`lib/open.c`、
  `lib/fsusage.c`、`lib/acl-internal.c`、`lib/set-permissions.c`、`lib/file-has-acl.c`、
  `lib/pipe2.c`、`lib/getrandom.c`、`lib/careadlinkat.c`、`lib/filemode.c`、`lib/filevercmp.c`、
  `lib/save-cwd.c`）。
  **收益**：这些文件里的死臂不再要求 token——`strftime.c` 的 ~900 行 glibc/NetBSD locale 表、
  `regcomp.c`/`regexec.c` 的 `_LIBC` 内部代码（数千行）、4 个 at-func.c 消费者的模板展开，
  全部不用再写。仍未核验的 `lib/tempname.c`/`lib/getopt1.c`/`lib/getloadavg.c`/`lib/nproc.c`
  分歧原因已量出（表里没有系统头宏，如 `TMP_MAX`/`HW_NCPU`/`__getopt_argv_const_is_empty`），
  它们照旧按盲条件移植。

- **3.21 `SSIZE_MAX` 的镜像归属待收口（2026-10-03）**：`<limits.h>` 的 `SSIZE_MAX`（本目标 =
  LONG_MAX，实测 9223372036854775807）目前被 `lib/getdelim.rs`、`lib/regex_internal.rs`、
  `lib/careadlinkat.rs` 各自在单元内声明一处（都是"镜像还没有时先本地声明"的产物）。
  我试过把它放进 `lib/include/limits.in.rs`，结果三个单元同时 E0428——所以现在 limits 镜像里
  只留一条 NOTE，不改这三个单元。收口方案：一次性把三处本地声明删掉、统一 include 镜像；
  这需要同时改三个已交付单元（其中 regex_internal.rs 还在并行开发中），故留作独立小任务。

- **3.22 已核验文件扩到 421（2026-10-03，本轮第二大杠杆）**：把 `tools/check-cond-eval.py`
  对**全部 552 个参照 .c/.h** 跑了一遍（逐条对拍 clang），结果 **421 VERIFIED / 131 NOT
  VERIFIED**（11,432 个条件，1,911 处分歧集中在 131 个仍不合格的文件里）。清单已写入
  `docs/baseline/config/cond-eval-verified.txt`。**后果**：剩下 450 个待开工文件里绝大多数
  （含 `src/` 的大件）现在都由冻结配置判活/判死，死臂不再要求 token——这是后续几万行的直接
  减负。代价：`gate-fast` 的第一步（`--verified`）现在要跑 ~6 分钟（它逐文件对拍 clang）。
  扩清单同时暴露并修掉了一批"旧端口按盲条件写下、现在不再需要/不再成立"的构造：
  - `tools/cinventory.py`：函数定义的花括号前允许**预处理行**（gnulib 在签名与 `{` 之间写
    `# undef nanosleep`，nanosleep.c 44），否则那个函数整个消失，端口的 `nanosleep` 反被
    判成 invented exported symbol（同类还有 McClintock 的属性行回看修复）。
  - 6 个死臂函数去掉 `#[unsafe(no_mangle)]`（getgroups 的 rpl_getgroups、group-member/
    tparam 的 `main`、readlink/symlink 的死臂同名函数、doprnt 的 `exprintf`）；
    反过来 utimens/lutimens/rpl_utimensat 三个**活体**函数补上 `#[no_mangle]`（原来漏了）。
  - sha1/sha256/sha512 的 `SWAP`：C 现在只剩一个活定义，宏体比对生效，活臂必须写 C 的
    `bswap_32 (n)` / `u64bswap (n)`（前者在单元里按 md5.rs 先例声明），并把活臂放在**第一个**
    `macro_rules!`（`macro_body_of` 取首次出现）。
  - 镜像补齐：`filename.rs` 的 `FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE`、`errno.rs` 的
    `EISDIR`、`fcntl.in.rs` 的 `PROMOTED_MODE_T`；`_Restrict_`/`lock_fini`/`lock_unlock`/
    `DEFINE_SHA3_*` 四个无法拼写的构造进 `macro-body-notes.tsv`。

- **3.23 ACL 族的落法（2026-10-03）**：`USE_ACL` 在冻结配置里是 **0**、`HAVE_SYS_ACL_H` 也是
  undef，所以 `acl-internal.h` 的 `#if USE_ACL` 平台臂整片判死，ACL 六个 unit 的活体都不大：
  `acl_entries.c` 只剩"计数骨架"（`acl_get_entry` 两个循环判死），`qcopy-acl.c`/
  `get-permissions.c`/`set-permissions.c`/`file-has-acl.c`/`acl-internal.c` 的活体分别是
  "无 ACL 支持"的 `#else` 分支。已落地：实测的 `sys_acl` 镜像（四个句柄类型 `void *`、四个
  标量类型 32 位、常量族、函数声明）与 `acl` 镜像（`struct aclinfo` 184 字节/union@28、
  两个 flag 常量、声明、三个 xattr 宏的 `#else` 形态）。**`acl-internal.h` 的镜像暂缓**：
  它 42 个宏里绝大多数是判死的平台臂，真正的第一个重消费者是 `qcopy-acl.c`/
  `get-permissions.c`（需要 `struct permission_context`），镜像随它们一起落。
- **3.24 新增镜像条目时的固定检查（2026-10-03，连续三轮踩到）**：往**共享镜像**里补一个系统
  条目（`strcpy`/`strncmp`/`EOF`/`sprintf`/`ssize_t` 这类）时，必须先在整树 grep 同名声明：
  任何 unit 里的本地同名声明都会立刻变成 E0428。已修的三处：`canonicalize-lgpl.rs` 的
  `strcpy`、`getopt.rs` 的 `strncmp`、`getdelim.rs` 的 `EOF`。规则：**系统条目归镜像，单元
  只保留镜像没有的**（0.1-3 单一定义点）。

- **3.25 "整文件死" 的移植政策（2026-10-03，本轮定案并落地 9 个文件）**：冻结配置把整批
  `src/` 文件关掉（`HAVE_ZLIB`/`HAVE_GLIB`/`HAVE_LCMS2`/`HAVE_SOUND`/`HAVE_DBUS`/`HAVE_X_SM`/
  `MSDOS` 全 undef），这些文件的**整个函数体**都在 `#ifdef <特性>` 之内，C 侧除两个 include 外
  什么都不编译。政策：这种文件移植为**空单元**——保留 C 自己的 license 头 + 一段 `// femacs:`
  说明（点名 gate 宏、config 行号、开启时需要哪些未具备的头文件），不携带死文本；对应 P-51
  "臂体需要本目标没有的头"那一条，只是粒度提到整文件。**已落地 9 个**：`xgselect.c`(296)、
  `decompress.c`(349)、`lcms.c`(604)、`sound.c`(1,554)、`dbusbind.c`(2,190)、`xsmfns.c`(581)、
  `w16select.c`(692)、`dosfns.c`(798)、`msdos.c`(4,350)——合计 11,414 行 C，在冻结配置下零产出，
  每个都有 0.99 的行数容差行（原因写明）。注意 `msdos.c` 含非 UTF-8 字节，读参照时按
  `errors="replace"` 处理，避免污染对照文档。

- **3.26 条件宏表扩到 7,222 条 + "死臂不得发符号"这一类（2026-10-03）**：
  ① `tools/gen-cond-macros.py` 的探针原先只 include 十几组头，导致 `TMP_MAX`/`HW_NCPU`/
  `__getopt_argv_const_is_empty` 这类**系统头宏**在表里缺席；本轮把探针扩到 22 组头
  （stdio/locale/langinfo/wchar/wctype/iconv/pwd/grp/dirent/utime/math/pthread/sys_param/
  sys_mount/sys_sysctl/sys_select/sys_wait/sys_socket/netinet_in/sched 等），表由 4,956 条
  涨到 **7,222 条**，全树核验数由 **421 → 427**（`cond-eval-verified.txt` 已换新）。
  ② 表变准后 `src/floatfns.c` 当场被打红：`HAVE_TRUNC` 现在是"已定义 ⇒ `#ifndef HAVE_TRUNC`
  死"，于是端口里 `#[cfg(feature = "no_trunc")]` 的 `trunc` **不该带 `#[unsafe(no_mangle)]`**
  （参照在这个配置下不定义 `trunc` 符号）。这与上一轮 `filemode.c` 的 `strmode` 是**同一类**：
  **凡是被配置判死的臂，其定义一律不发符号**，即使 `dead-arms` 翻转构建会编译它（那种构建只
  验证"能编译"，不要求导出）。同类处置已写进 `docs/ports/*.md` 的 Decisions。

- **3.27 `termchar.h` 镜像已落地，显示链的下一块是 `glyph`/`glyph_pool`/`glyph_matrix`/`glyph_row`（2026-10-03 第 55 轮）**：
  ①`src/include/termchar.rs` 是 termchar.h 的完整镜像（`--check OK`）：`tty_output`、
  `TERMCAP_BUFFER_SIZE`、`tty_display_info`（三段 `bool_bf` 连续位域各用一个 `bits_*` 字节 +
  `TTY_*` mask，与 `itree_node.bits`/`frame.bits1` 同形）、`tty_list`、`FRAME_TTY`/`CURTTY`
  两个宏（`macro_rules!`，在使用点展开）、`tty_handle_tab_bar_click`/`tty_setup_colors`。
  ②**`scroll.c` 不是"只依赖现有镜像"的文件**：它读 `current_matrix->nrows`、`MATRIX_*` 一族与
  `FRAME_SCROLL_REGION_OK/COST`（termhooks.h 920-921，正文里指向 `FRAME_TTY`），因此它的前置是
  `dispextern.h` 的**显示核心结构**：`glyph`、`glyph_pool`、`glyph_matrix`、`glyph_row`
  （dispextern.h 472-1160）+ `enum glyph_type`/`enum glyph_row_area`（431/881）+ 1163-1340 的
  `MATRIX_*` 宏。这些是 batch E（显示与终端，50 文件 / 172,942 行）的共同前置，**优先于**
  `scroll.c` 本身落地；`frame.rs`/`window.rs` 现在把它们退化成 `*mut c_void`（P-72 规则③），
  落地时把 `glyph`/`glyph_pool`/`glyph_matrix`/`glyph_row`/`Mouse_HLInfo` 加进
  `tools/gen-layout-fields.py` 的 `STRUCTS`，由布局探针按真 C 编译器的 `offsetof`/`sizeof` 断言。
  ③`FRAME_TTY` 里的 `(f)->terminal->display_info.tty` 在 termhooks.rs 是 `*mut c_void`
  （termhooks.h 533-542 的 union 指向未镜像类型），宏里**显式转回** `*mut tty_display_info`，
  这是 P-55 退化而不是新指针类型。④同日发现并修掉一处并行冲突的实证：两个 worker 同时往
  `lisp.rs` 的 `other_units` 加同名 `string_overflow` → E0428，整个 TU 编译失败；**往共享镜像
  加声明前必须先 grep 同名**（已写进 worker 指令）。

- **3.28 两个工具盲区（2026-10-03 第 55 轮实测记录，尚未修）**：
  ① **跨行 C 字符串让 token 门变瞎**：`tools/extract-abi.py` 的 `body_token_seq` 用正则匹配 C 字符串，
  它跨不过字符串里 `\`+换行的续行。实测 `src/doc.c` 296-298 的 `error ("\` + 换行 处，
  该字面量的词（`invalid`/`data`/`documentation`/`followed`/`code`/`03`）**漏进 token 比对**，
  且它后面的真实代码被当成字符串吞掉——这段区域的变异对门不可见。当前处置：Rust 侧写成同样的
  `\`+换行 让两侧状态一致（字节也一致），但**门本身仍瞎**；修法是正则按 C 词法处理续行，并加回归用例。
  ② **`tools/cinventory.py` 的 `norm_const` 把 C 八进制字面量当十进制**：`040` 被归一成 `40`，
  而 C 的值是 32。第一个受害者是 `dispextern.h` 的 `SPACEGLYPH`（`037`/`040` 一族）与
  `commands.h` 的 `Ctl`（`037`）。当前处置：按 C 真值写 Rust、把该行记进对应 ledger 的 Decisions；
  **常量没有 notes 机制**，所以修工具前这类差异只能靠手工登记。修法：按 C 规则解析 `0` 前缀的
  八进制/十六进制字面量后再比较，并加回归用例。

- **3.29 `DEFVAR_*` 的存储体：单元的 `V*` 对象必须由该单元定义（2026-10-03 第 55 轮发现，闸门看不见）**：
  C 里 `Vfoo` 不是独立对象，而是**一个 `globals` 结构体的字段**：`lisp.h` 3495-3500 的
  `DEFVAR_LISP` 取 `&globals.f_##vname`，`struct emacs_globals globals;` 定义在 `src/alloc.c:278`。
  端口的现形状是 `include/globals.rs` 的 `pub mod variables` 里
  `unsafe extern "C" { pub static mut Vfoo: Lisp_Object; }`（**只有声明**），因此：
  ① **谁在 C 里拥有它，谁就要在端口里定义它**（`#[unsafe(no_mangle)] pub static mut Vfoo:
  Lisp_Object = lisp_h_Qnil ();`）——已按此形状的有 `src/casetab.rs`（四个）与
  `src/character.rs`（`Vchar_unify_table`），第 55 轮补上 `src/cmds.rs` 的
  `Vpost_self_insert_hook`；
  ② **第 55 轮已补齐 34 个**（`tools/check-globals.py` 从"调用点减去定义点"机械求得，8 个单元：
  callint 6、character 8、doc 5、undo 5、data 3、macros 3、casefiddle 2、category 2），
  补齐后该工具报 **0**；`xtask gate-fast` 新增 "globals gate" 一步，回归立刻红。原先列出的 20 个是：
  `character.rs`（Vauto_fill_chars/Vchar_width_table/Vambiguous_width_chars/Vprintable_chars/
  Vtranslation_table_vector/Vchar_script_table/Vunicode_category_table）、`doc.rs`
  （Vdoc_file_name/Vbuild_files）、`callint.rs`（Vcommand_history/Vcommand_debug_status/
  Vcurrent_prefix_arg/Vlast_prefix_arg/Vmark_even_if_inactive/Vmouse_leave_buffer_hook）、
  `macros.rs`（Vexecuting_kbd_macro/Vkbd_macro_termination_hook/Vlast_kbd_macro_）、
  `data.rs`（Vmost_positive_fixnum/Vmost_negative_fixnum）、`category.rs`
  （Vword_separating_categories）；
  ③ **为什么原来闸门看不见**：`extract-abi.py` 的 "File-scope objects" 只扫 C 文件里
  **自己写的**对象定义，`DEFVAR_LISP` 的存储它看不到——`src/cmds.c` 的清单因此报
  "File-scope objects — 0"，而该文件在 C 里确实定义了一个对象。**已补的符号级门**：
  `tools/check-globals.py`（每个 `DEFVAR_LISP|LISP_NOPRO|BOOL|INT` 调用点的第二个参数都必须是
  纯标识符，且同单元里要有带初始值的 `static mut` 定义；`DEFVAR_KBOARD` 是 `struct kboard` 的
  字段偏移，不在其列），并已挂进 `xtask gate-fast`；
  ④ 长期口径（等 `alloc.c` 落地时定）：把 `struct emacs_globals` 镜像进 `globals.h` 的镜像、
  由 `alloc.rs` 定义唯一的 `globals` 对象，`V*` 变成**字段访问**（C 的存储模型）。在此之前
  按 ① 的形状办，偏离记在各单元 ledger 的 Decisions 里。

- **3.29 `#undef` 的死分支判定 + `#define` 的函数式名字（2026-10-03 第 56 轮，工具修正；
  `getloadavg.c` 因此进入核验表）**：`tools/cinventory.py` 两处修正——
  ①`#undef` 原来**不检查死分支**（`#define` 检查），于是 `lib/getloadavg.c` 104-110 躺在
  `#ifdef NeXT` / `#if defined __gnu_hurd__ || defined NeXT`（皆假）里的
  `#undef BSD`/`#undef FSCALE` 被执行，把该文件自己 `<sys/param.h>` 的两个宏抹掉，
  整个 nlist 链被判死：82 条条件里 **20 条与 clang 分歧**；
  ②函数式 `#define NAME (参数)` 记录的名字是 `NAME (参数)`，`defined (NAME)` 因此恒假，
  `LDAV_CVT`（C 190）永远不满足 C 267 的 `#ifndef LDAV_CVT`（最后一处分歧）。两处修好后
  `check-cond-eval.py lib/getloadavg.c` = **82 条件 / 0 分歧**，`lib/getloadavg.c` 已加入
  `cond-eval-verified.txt`（现 **428** 文件）；把整张表重跑一遍（**5,601 条条件 / 0 分歧**）
  确认无回归。**对后续的影响**：凡是「死分支里有 `#undef`」的文件，之前可能把活文本判死
  （端口少写、门也看不见），现在会按 clang 的真值要求活文本；`--all` 未见新增 failing。

- **3.30 符号表门：替换件的 `rpl_` 改名方向 + "头文件载体"单元（2026-10-03 第 55 轮定案并落地工具）**：
  ① **改名方向的权威是参照对象**：gnulib 的生成头在 `REPLACE_*` 为真时做 `#define foo rpl_foo`，
  而 `.c` 有时在声明符与函数体之间 `# undef foo`（`nanosleep.c 41`、`pipe2.c 49`）、有时不写
  （`mempcpy.c`）；因此**定义落在改名的哪一侧，只看端口源码是判不出来的**。本轮的实测：端口写错
  了 5 个——`nanosleep`（无限自递归 + 导出 `_nanosleep`，应为 `_rpl_nanosleep` + 调用系统入口）、
  `pipe2`/`futimens`（漏改名）、`mempcpy`/`canonicalize-lgpl`（多改了：参照导出 `_mempcpy`/
  `_rpl_realpath`）。已全部按参照对象的 `nm -g` 改对（`export_name` ↔ `no_mangle`，并给
  `time.in.rs` 补上系统 `nanosleep` 的模块限定声明——`pthread_sigmask.rs` 的既有形状）。
  ② **新门 `tools/check-symbols.py`**：逐个单元比对 `build/femacs/<stem>.o` 与
  `build/emacs-ref/lib/<stem>.o` 的 `nm -g` 导出集（过滤 Rust 自己的 monomorphization 噪声
  `__R*`/`_ZN*`）；`--pending docs/baseline/symbol-pending.txt` 是**棘轮**（清单里的单元变干净了
  也算失败，只许收缩），并已挂进 `xtask gate-fast`。
  ③ **未收口的 18 个是同一类："头文件载体"单元**：`byteswap`、`c-ctype`、`endian`、`fingerprint`、
  `issymlink`、`memeq`、`mini-gmp-gnulib`、`openat-die`、`stat-time`、`stdc_*`（5 个）、`streq`、
  `strnul`、`timespec`、`u64`——它们的 `.c` 只是给头的 extern-inline 函数一个落点，参照对象因此
  导出 `T _c_isalpha`/`T _stat_time_normalize`/整片 `_mpz_*`（`mini-gmp-gnulib` 约 200 个），而端口
  把定义放在镜像里当 `#[inline(always)]`，一个符号都不发。收口方式：让这些单元自己发
  `#[unsafe(no_mangle)]` 定义。清单已冻结成棘轮，下一轮从这里接着做。

- **3.31 "构建系统判死"的平台文件（2026-10-03 第 58 轮发现，政策与工具待落）**：
  ① **现象**：`src/w32reg.c`(165)、`w32xfns.c`(447)、`w32cygwinx.c`(134)、`cygw32.c`(139)、
  `xrdb.c`(513)、`widget.c`(514)、`pgtkim.c`(311)、`emacsgtkfixed.c`(210)、`haikuimage.c`(116)、
  `haiku_io.c`(217)、`haiku.c`(286)、`gfilenotify.c`(336) 这些文件**没有顶层 `#if` 守卫**
  （§3.25 处理的是有守卫的那批），它们不在本目标的构建里靠的是**构建系统的源列表**：
  `build/emacs-ref/src/Makefile` 的 `WINDOW_SYSTEM_OBJ=`（304 行，空）、`HAIKU_OBJ = `（378 行，空）、
  `MSDOS_OBJ =`（307 行，空）。而 `tools/extract-abi.py` 只按**预处理条件**判定死活，于是把这些文件里的
  函数/子例程当成活文本——实测 `w32reg.c` 的 ledger 要求 5 项（Windows 注册表）、`xrdb.c` 13 项（X11
  resource 数据库）、`haikuimage.c` 3 项、`gfilenotify.c` 8 项。照做就要实现本目标既没有头也没有库的
  平台代码。
  ② **判据不能是 `.o`**：`build/emacs-ref/src/*.o` 一个都没有（参照构建只完成到 `lib/` 的对象），
  所以"有对象才算本目标"这条不可用；权威是 Makefile 的那三个变量与 `$(obj)` 的拼装。
  ③ **已实现（同日）**：判据落成 `docs/baseline/frozen-build-sources.txt`——用
  `(cd build/emacs-ref/src && make -n) | grep -oE 'emacs/src/[a-zA-Z0-9_-]+\.c' | sort -u`
  生成（dry run，5 秒，不编译）：**152 个 `src/*.c` 里冻结构建只编 75 个，其余 77 个是平台后端**。
  `tools/extract-abi.py` 现在对"在 `src/` 里但不在该清单里、且端口是**空形状**（全文没有 `fn`）"的文件
  不再要求实现：ledger 打印一行可见的
  "This file is not compiled by the frozen configuration … N declaration(s) are deferred"，
  条目数为 0。**豁免只给空形状**：`vm-limit.rs`/`tparam.rs` 这类在该规则之前就已完整移植的文件
  仍走全量比对（实测 `--check` 仍 OK、15 条条目仍在），所以这条改动不会削弱既有验证。
  已落地的头 6 个：`w32reg`(165)、`w32cygwinx`(134)、`cygw32`(139)、`xrdb`(513)、`widget`(515)、
  `gfilenotify`(338)——每个都是"C 自己的 license 头 + 点名排除依据的说明"的空单元（30-36 行），
  `build-tu` 全 built、`--check` 全 OK、ledger 0 条目。剩余 71 个同族文件照此批量落地。
  ④ 注意 `src/xml.c` **两类都沾**：它有真的 `#ifdef HAVE_LIBXML2`（按 ① 之外的既有判据办，已落地：
  活文本只有 `libxml-available-p` 的 `#else` 臂与 `syms_of_xml`），但文件本身也不在 X11/Windows 源列表里。

- **3.32 `profiler.c` 与 `filelock.c` 的前置镜像（2026-10-03 第 66 轮记录，尚未落）**：
  已构建但未移植的最小一批里，先要补的镜像：
  ① `src/syssignal.h`（88 行）→ 缺 `femacs/src/include/syssignal.rs`：7 个信号的 block/unblock 原型
  （`init_signals`/`block_child_signal`/`unblock_child_signal`/`block_interrupt_signal`/
  `restore_signal_mask`/`block_tty_out_signal`/`unblock_tty_out_signal`）、`empty_mask`、
  `signal_handler_t`（`typedef void (*)(int)` → `Option<unsafe extern "C" fn (c_int)>`，P-27）、
  `emacs_sigaction_init`/`safe_strsignal`/`deliver_process_signal`，以及 `NSIG_MINIMUM`/`SA_SIGINFO`/
  `emacs_raise`/`strsignal` 这几个宏。`HAVE_PTHREAD` 在本目标是 1
  （`docs/baseline/config/darwin-arm64.macros.tsv` 1471 行），所以 `FORWARD_SIGNAL_TO_MAIN_THREAD` 是活文本。
  **注意**：`struct sigaction` 在本目标没有镜像（`femacs/lib/include/signal.rs` 只有 `sigset_t`/`NSIG`），
  而 `emacs_sigaction_init` 只把它当指针用，所以按 P-70/P-72③ 落一个不完整替身 + `/// C:` 注明即可。
  ② `src/pdumper.h`（281 行）→ 缺 `femacs/src/include/pdumper.rs`：`pdumper.c` 的公开 API
  （`pdumper_load`/`pdumper_dump`/`dump_fingerprint` 一族 + 几个结构体）。
  补完这两个，`profiler.c`(733) 与 `filelock.c`(840) 即可开工（两者都不在 §3.31 的排除集里，是**真需要实现**的单元）。

- **3.33 §3.31 空单元的两个实测坑（2026-10-03 第 67 轮，从 62 个文件的批量落地中总结）**：
  ① **"license 块"要停在第一个非注释行，而不是第一个 `#include`**：`src/w32.c`/`w32notify.c`/`w32proc.c`
  在第一个 `#include` **之前**有一行真代码（`#define DEFER_MS_W32_H` 一族），按"复制到第一个 `#include` 为止"
  的写法会把它当注释文本抄进 `.rs`，rustc 直接报 `expected one of '!' or '['`（这 3 个文件初次 `build-tu` 真失败过）。
  正确口径：从文件开头复制到**第一个既非空行也非注释的行**为止。已提交的 3 个单元现在都不含 `#define`
  （`grep -c '^#define'` 为 0，且 `build-tu` 通过），6 个样板按新口径重生成后**逐字节不变**。
  ② **`clear_unbuilt_inventory` 曾漏 `struct_fields`**：ledger 的 "Types and their fields" 一节就是用它的，
  于是 62 份里有 35 份一边写"Nothing in this file is required here"、一边列出顺延的结构体字段
  （`w32.c` 那 8 个还是 `<anon@185>` 这类垃圾容器名）。已修（`b706b46`）并重生成全部受影响台账
  （`83eb42e`/`3748fca`/`b17e87e`）；全批顺延声明合计 **5,199** 条（`w32.c` 一个文件 388 条）。
  ③ **这一类已清零**：`emacs/src/*.c` = 152 = 冻结构建编译 **75** + 排除 **77**；77 个排除项全部有端口
  （真实单元 3 个：`dynlib`/`tparam`/`vm-limit`；空单元 74 个），62/62 逐文件 `build-tu` built 且
  `extract-abi.py --check` OK。

- **3.34 常量的八进制/二进制拼法（2026-10-03 第 70 轮：一半已修，一半待修）**：
  ① **已修**：`extract-abi.py` 的 `norm_const` 原先把 C 的八进制当十进制（`0200` → 200 而不是 128），
  于是端口按 C 语义写的 `0o400`/`0o200` 被报成"缺常量"。现在 `0x`/`0b`/前导 `0`（八进制）/十进制四种
  进制都按 C 的值归一（`0200`→128、`0400`→256、`0b101`→5、`017`→15），后缀 `u`/`l` 照旧折叠。
  实测：`src/syntax.c` 的差异从 55 条降到 **53 条**，而已通过的单文件（`terminal`/`cmds`/`regcomp`）仍 OK。
  ② **已修（2026-10-03 第 77 轮，`243df0d`）**：`body_token_seq`/`family_call_numbers` 原来按**拼法**
  比较整数字面量，C 的 `0400`（一个 token）与端口的 `0o400`（被切成 `0`、`o400` 两个）因此对不上
  （`syntax.c` 的 `skip_chars` 报 "does not mention 0200, 0400" 就是这个原因）。现在两处都用同一份
  `INT_LITERAL` 正则切分并按**值**归一（`0x`/`0b`/`0o`/前导 `0` 八进制/十进制），顺带修掉 `0x80`
  被切成 `0`+`x80` 导致与 `128` 比较不等的问题。回归：11 个此前 OK 的单元仍 OK，`syntax.c` 从 55 条
  差异降到 0。

- **3.35 源码头文件定义的宏：`source-header-macros.tsv`（2026-10-03 第 77 轮定案并落地）**：
  `PROFILER_CPU_SUPPORT` 由 **src/syssignal.h 53-54** 定义（`#if defined SIGPROF && !defined PROFILING
  && (defined HAVE_SETITIMER || defined HAVE_ITIMERSPEC)`），**config.h 里没有它**，所以冻结宏表不认识它。
  后果不是"门报错"而是"门答错"：求值器把 `src/profiler.c` 的 C 343-524（整个 CPU profiler 区）判成死文本，
  于是端口照 C 写的四个 `defsubr (&raw mut Sprofiler_cpu_*)` 被报成
  "unexpected defsubr (...): the reference file does not register it"——而这一类**没有任何豁免机制**，
  只能把表修对。定案：
  ① 新增 `docs/baseline/config/source-header-macros.tsv`，一行一个 `<header>\t<macro>`，首行
  `syssignal.h\tPROFILER_CPU_SUPPORT`，文件头写明"为什么 config.h 决定不了、真实构建确实定义了它"；
  ② `tools/gen-cond-macros.py` 用 clang（与 config.h dump 同一套 `-I`）**实测**每行的值再合并进表——
  行过期会打印 warning 记成 `undef`，值错了会在 `check-cond-eval.py` 里表现为 disagreement，
  不会静默冻结一个错分支（P-66 的同一条原则）；③ 表因此只多一行（7223 宏 / 6782 defined）；
  ④ `src/profiler.c` 加进 `cond-eval-verified.txt`（429 个文件），它的 13 个条件全部与 clang 一致，
  整表重跑 0 disagreement，其余 428 个文件无回归。
  **同类风险**：任何"由源码头文件而非 config.h 决定"的 `#ifdef` 都会重演这一幕（`src/*.h` 里再出现
  时照此加一行，不要临时在 unit 里迁就门）。

- **3.36 版本声明去产品名（2026-10-03 用户指令，当天全树落地）**：所有单元/镜像首行的
  `@ fd7afced403 (GNU Emacs 32.0.50)` 括注一律去掉，只留修订号（**231 个文件**一次性机械改写；
  连同 `PORTING.md`/`docs/plan.md`/`docs/port-plan.md`/`docs/port-master-plan.md`/`docs/goal.md`/
  `docs/i18n/zh-CN/plan.md` 的基线行）。规则写进 `PORTING.md` 0.1-19，后续新文件照此。
  **不在改写范围**：参照文件自己的版权/许可声明和首行标题注释（例如 `/* Cursor motion
  subroutines for GNU Emacs.  */`）——那是被复制文本的一部分，GPL 要求保留；
  也不动 `docs/baseline/config/darwin-arm64.macros.tsv` 的 `PACKAGE_STRING "GNU Emacs 32.0.50"`
  （它是 clang 实测的冻结值，改了就与参照不符）。

- **3.37 照抄树落地与机器门（2026-10-03 第 79 轮）**：目标里 `lisp/`、`test/`、`etc/`、`doc/`、
  `leim/`、`admin/` 是"原样复制，一个字都不许改"，`docs/port-map.md` 把它们标成 `Verbatim`
  （目标 `femacs/<top>/**`），但此前仓库里**一个都没落**（`git ls-files` 在这六个前缀下为 0）。
  本轮用 `cp -R -p` 复制并逐个 `diff -r` 核对（6 个目录全部 0 行差异），然后补上**可复算的机器门**
  `tools/check-verbatim.py`：按 `docs/port-map.md` 的 `VERBATIM_DIRS` 比文件清单（缺文件与改文件
  同等算失败）再流式比内容（1 MiB 块 sha256，4.6M 行不进内存），非 0 退出。以后任何一轮改动
  `femacs/lisp` 或 `femacs/test` 都会被它当场抓住——这正是硬约束 5（不许改测试）此前缺的那道闸。
  注意 `docs/baseline` 的 `Verbatim 58 entries` 里还有 49 个**根目录文件**（`INSTALL`、`CONTRIBUTE`
  等）尚未复制，它们不在本轮六棵树内，等根目录入库策略定了再一起处理（`port-plan.md` 只把它们
  记为 "repository root"，没有 gate）。

- **3.38 `DEFVAR_*!` 的名称参数必须写 `c"…"`（2026-10-03 第 80 轮发现并修）**：
  `DEFVAR_LISP!`/`DEFVAR_INT!`/`DEFVAR_BOOL!`/`DEFVAR_KBOARD!` 的宏体是
  `defvar_lisp (&raw const O_FWD, $lname.as_ptr () as *const c_char)`——即把 `$lname` 的
  **字节指针**交给 C。传 `"name"`（`&str`）时 `as_ptr()` 指向的字符串**没有 NUL 结尾**，
  `defvar_lisp` 会越界读到栈上的下一个 0；传 `c"name"` 才是 C 的 NUL 结尾字面量。
  实测 **16 处**写成了 `"…"`（`callint.rs` 8 处、`character.rs` 8 处，都是早期轮次落的），
  已全部改成 `c"…"`；两个文件重建 `built`、`--check` 仍 OK。**这类缺陷三道静态门都看不见**
  （token 比对把字符串字面量折叠掉，只有运行期才会读错名字），所以记在这里当检查项：
  今后凡是 `DEFVAR_*!`/`DEFSYM!` 之外的 C 字符串字面量参数，一律 `c"…"`。

- **3.39 行数门新增 `live` 基准（2026-10-03 第 80 轮，工具改动）**：`src/comp.c`(5,857)、
  `src/treesit.c`(5,634)、`src/gnutls.c`(3,196) 三件**整文件基本是死文本**（HAVE_NATIVE_COMP /
  HAVE_TREE_SITTER / HAVE_GNUTLS 皆 undef，活的分别只有 45 / 53 / 124 行），照 P-08 只记台账
  不抄死区。可它们的行数门拿**全文件行数**当分母，于是忠实的 59 行端口报 99% 漂移，只能填一个
  0.99 的容差——那等于把这道门关掉。定案：`docs/ports/line-drift-tolerance.tsv` 增加**第四列
  `basis`**（`total` 默认 / `live`），`live` 行的分母由 `tools/extract-abi.py` 的
  `c_live_lines()` 用**与声明比对同一套分支求值器**算出的"本配置真正编译的行数"给出；只有明确
  不再抄死区的文件才写 `live`。**没有**改成全局 live 基准是有实测理由的：P-51 那批 gnulib 文件
  （readlink/getgroups/dup2/filemode/acl-internal…）**故意**把死臂保留成
  `#[cfg (feature = …)]` 文本，端口比 C 的"活文本"长得多，全局改基准会让 **119 个文件**当场不通过；
  现在 `live` 只对 5 个文件（comp/treesit/gnutls/systhread/sqlite）生效，其余照旧。
  顺带把 systhread/sqlite 两行从 0.80/0.90 收紧到 0.30，容差表头写清两种基准的判据。
- **3.41 选点用 `tools/live-rank.py`（2026-10-03 第 80 轮落地）**：把
  `docs/baseline/frozen-build-sources.txt` 里还没有端口的文件按**活行数**排序（`cinventory`
  的分支求值器逐文件判定），`--ported` 则按 `|活行数 - 端口行数| / 活行数` 排，用来一眼看出
  "端口比该有的活文本短得多的文件"（掉内容的头号嫌疑）。当前 `--top 12` 的头两件
  `src/dired.c`(活 1,067) 与 `src/menu.c`(活 1,079) 就是本轮派给两个 worker 的目标，
  40 件未端口文件里活行数最大的是 xdisp/alloc 那一档；每轮开工先跑它，不要再靠目录列表猜。
- **3.40 第 80 轮批量落地（2026-10-03）**：本轮新增 5 个单元——`src/systhread.c`、`src/sqlite.c`、
  `src/comp.c`、`src/treesit.c`、`src/gnutls.c`（合计 **14,174 行 C → 约 470 行 Rust**，全部
  `built` + `--check` OK + 台账 0 未勾），加一个镜像 `src/include/systhread.rs`。选点方法是把
  `docs/baseline/frozen-build-sources.txt` 里还没有端口、且**活行数最小**的文件排出来
  （`cinventory.dead_branch_map` 逐文件算活行），这把"整文件死文本"的 5 件一次挑了出来；
  剩下的 46 件里活行数最少的是 dired(1,067)/menu(1,079)/thread(1,255)。

- **3.42 `lib/` 的冻结对象只剩 1 件（2026-10-03 第 81 轮）**：冻结构建要的 lib 对象共 **78** 个
  （`build/emacs-ref/lib/*.o`），本轮把最后 4 件里的 3 件落地：`lib/binary-io.c`（P-81 载体：
  C 17 把 `BINARY_IO_INLINE` 定义成 `_GL_EXTERN_INLINE`，头里两个体就是这个对象的对外定义；
  `nm -g` 与参照对象逐符号一致；C 21-38 的 `__DJGPP__`/`__EMX__` 区由求值器判死，**这回答了
  §3.5 的悬案**——不需要 DJGPP 代码）、`lib/regex.c`（**聚合 TU**：C 57-59 include 了
  regex_internal/regcomp/regexec，参照只有一个 `regex.o`；端口按 0.1 的"一个 .c 一个 .rs"保留
  三个兄弟单元，本对象的符号表**按构造为空**，因此进 `symbol-pending.txt` 并写明理由）、
  `lib/dtoastr.c`（`ftoastr.c` 模板的 `LENGTH 2` 实例化，**待共享体 `femacs/lib/ftoastr.rs`**，
  同样进棘轮并写明）。剩 `lib/boot-time.c`(306) 在飞。
  **棘轮口径**：`python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt`
  现为 `63 units compared, 0 mismatched, 2 still pending`——棘轮只能缩小，两个待项各有一个
  明确的关闭条件（落 `ftoastr`；或决定把三个对象链成一个）。

- **3.43 `lib/` 的冻结对象全部落地（2026-10-03 第 82 轮收口）**：冻结构建要的 **78** 个 lib 对象
  现在**全部有端口**（`lib/boot-time.c` 是最后一件，378 行 + 新系统镜像 `lib/include/utmpx.rs`
  93 行，`struct utmpx` 实测 size 640 与九个字段偏移，九条 `const _` 钉住）。本轮 lib 侧共落
  7 件：`binary-io`、`regex`、`dtoastr`、`boot-time`、`ftoastr`，加先前已在飞的两件。
  **符号棘轮已缩到 1 项**：`python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt`
  → `64 units compared, 0 mismatched, 1 still pending`，唯一剩项 `lib/regex` 是聚合 TU 的构造性
  结果（台账写明：三个兄弟对象合起来就是参照 `regex.o` 的符号集）。
  `lib/ftoastr.c` 按 10.13② 的**方案一**落地：`femacs/lib/ftoastr.rs` 用
  `macro_rules! ftoastr_body { ($length, $name, $export, $float, $promoted, $dig, $min, $prec_bound, $strtof) }`
  携带整份活体、自身不实例化，`dtoastr.rs` 是唯一实例（`LENGTH 2` 那支），于是
  `nm -g build/femacs/dtoastr.o` 与参照一样定义 `_dtoastr`，棘轮里的 `lib/dtoastr` 条目随之删除。
- **3.44 本轮 src 侧新落 2 件（2026-10-03）**：`src/dired.c`（活 1,067 / 全 1,213 → 1,162 行）与
  `src/menu.c`（活 1,072 / 全 1,638 → 1,209 行）。两件的死区都按 P-08 记台账不抄写；
  menu 的落地顺带暴露并修掉了求值器的一个真缺陷（见 §3.45）。
  未端口的冻结构建 src 文件从 40 降到 **38**（最便宜的下一批：thread 1,255 / chartab 1,387 /
  bytecode 1,773；`tools/live-rank.py` 每轮开工先跑）。
- **3.45 求值器漏掉预处理行尾注释（2026-10-03 第 82 轮修，`c03b048`）**：`#ifdef HAVE_NTGUI
  /* W32 specific because … */` 这种**行尾块注释**没有在求值前去掉，tokenizer 撞上 `/` 就答
  `unknown`，分支于是保守留活（而 clang 判死）——`src/menu.c` 正是因此一直进不了
  `cond-eval-verified.txt`，`--check` 会要求整份窗口系统代码。修法按 C 的翻译阶段 3（指令读到前
  注释已删除）截掉该逻辑行未闭合的注释；**独立复验**：`src/menu.c` 变为
  `21 conditions, 0 unknown, 0 disagreements [VERIFIED]`，全表重跑 429 个文件
  `5696 conditions, 4 unknown, 0 disagreements`（零回归）。这是"门答错"这一类的最新一例。

- **3.46 `src/thread.c` 落地 + `thread.h` 镜像收口（2026-10-03 第 83 轮）**：`THREADS_ENABLED` 虽
  是 undef，但 thread.c **不是整文件死文本**——十几个小 `#if` 把它切成若干死臂，活文本 1,255/1,314
  （21 个 subr 全部注册但因 C 937-941 的活体 `error ("Concurrency is not supported…")` + `if (0)`
  守卫而**不生效**，与 C 一致）。端口 1,319 行，容差行用 `live` 基准。镜像 320 行收口（含
  `#[repr(C, align(4))]` 包住的 `sys_jmp_buf`：C 是 `int[48]`/align 4，lisp.rs 的 `jmp_buf` 是
  `[u8;192]`/align 1，而 `thread_state` 内嵌它）；`struct handler` 在镜像里声明后，布局基线里
  `handler` 的 6 个 pending 键变成 asserted（**785→791 asserted / 6→0 pending**）。
- **3.47 全树 `gate-fast` 真跑一次抓出四件事（2026-10-03 第 83 轮，均已修并提交）**：
  ① `lib/allocator.rs`、`lib/sigdescr_np.rs` **根本编译不过**（它们的 `use std::ffi::{…}` 早于共享
  镜像后来加进的 `strtoul`/`c_uint`/`c_void`）；② `lib/canonicalize-lgpl.rs` 自己定义了 `F_OK` 又
  include 了定义它的 `unistd.in.rs`（E0428）；③ `src/chartab.rs` 在 `eassert!` 里**先用了后定义**的
  `CHARSET_ATTR_ID!`——这一条只有 `--cfg enable_checking` 才看得见（断言平时是死文本），是 P-57 的
  又一例；④ 13 个 `DEFVAR_*` 变量没有在**自己的单元**里定义（见 §3.29），上一轮已修。
  结论写进文档：**单文件三道门看不见"整棵树才有的"缺陷**（缺 import、重复定义、宏顺序、
  DEFVAR 归属），所以 `gate-fast` 不能长期省。
- **3.48 `materialize` 改为按 C 的文本顺序展开（2026-10-03 第 83 轮，`456bbd6`）**：旧实现"依赖
  先行拍平"会把每个镜像排在**包含它的文件之前**；而 C 里 `thread.h` 是**在 lisp.h 内部**被
  include 的（lisp.h 2289），所以它看得见 lisp.h 181-196 定义的 `eassert` 与 750-752 的 `XUNTAG`。
  拍平逼着 thread 镜像写偏离 C 的形状。现在按"首次到达它的那条 `include!` 的位置"就地展开
  （再次到达即成去重注释，等价于 include guard），既保住 1:1，也正是单元里那些宏一直需要的行为。
  **连带修的探针**：`spikes/` 下 7 个 `.rs` 探针与 8 个 shell 探针各自手写 `include!` 列表、没有
  include guard，镜像图长大到 `time.in.rs` 同时能被 `lisp.rs`（经 thread.rs→sys_select.rs）与
  `buffer.rs` 到达，于是 E0428；现在它们与单元走**同一套 materialize**（新增
  `xtask probe <rs> <out> [<extra rustc args>…]` 取代 shell 里裸调 `rustc`）。
  这一轮的收口证据是**整条 `gate-fast` 跑到底**：`condition evaluator` →
  `inventory: no differences` → `checked 338 ported files: 312 done, 26 in progress, 0 failing` →
  `globals: every DEFVAR_* variable has a definition` → 七个 `probe …: the Rust side compiles` →
  `symbols: every built unit matches its reference` → `fam-probe: 4 lines ok` /
  `shadow-probe: shadow-probe true 0 true` / `macro-probe: every expansion compiles` →
  `layout-check passed`。

- **3.49 求值器判不了的 src/ 文件：照 P-51 抄下两支，而不是记台账（2026-10-03 第 84 轮定案，
  `src/timefns.c` 为案例）**：`timefns.c` 的 `#if` 里用了 **function-like 宏**
  `FIXNUM_OVERFLOW_P (…)`（lisp.h 1178）与 `TRILLION <= ULONG_MAX`（`ULONG_MAX` 在表里是
  `(__LONG_MAX__ *2UL+1UL)`，求值器解不了），另有 `#ifdef NEED_ZTRILLION_INIT` 与 clang 不一致。
  该文件不在 `cond-eval-verified.txt`，于是 `blind_conditions` 丢表、**两支都留活**，生成的台账
  同时要求 `timespec_hz`/`trillion`/`ztrillion` 的 `#if` 与 `#else` 两半、以及 clang 判死的
  `syms_of_timefns_for_pdumper`——照 P-08"只记台账"永远拿不到 `--check` OK，照台账抄又违反 src/
  的死臂政策。**定案：这类文件按 P-51 抄下两支**——冻结构建真正编译的那支是普通文本，clang 判死的
  那支放进 `#[cfg (feature = "…")]`（feature 按 C 的条件命名，如 `no_faster_timefns`；
  它们不进 `xtask dead-arms`，因为是编译器/平台谓词而非 config.rs 键，§3.9 同例）。
  先例已在树里：`lib/boot-time.rs` 正是这个理由抄下 utmp/sysctl 两支，`lib/binary-io.rs` 同。
  **抄下更多 C 文本是更 1:1，不是更少**；`line-drift-tolerance.tsv` 用默认 `total` 基准。
  这条不改变其他文件：能进 `cond-eval-verified.txt` 的文件仍按 P-08 只记死区。
  另注：`timefns.c` 的行数口径以 `count("\n")` = **2144** 为准（编辑器报 2145）。

- **3.50 第 85 轮：`src/bytecode.c` 与 `src/timefns.c`（2026-10-03）**：两件都是大盘，
  且各自验证了一条既有政策。
  ① **`timefns.c` 是 §3.49 新政策的第一个实践者**：`#if FIXNUM_OVERFLOW_P (…)`（function-like
  宏）与 `TRILLION <= ULONG_MAX` 求值器答 `unknown`，该文件不在 `cond-eval-verified.txt`；
  按定案**抄下两支**（活支普通文本、clang 判死支放 `#[cfg (feature = "no_fixnum_timespec_hz")]`
  等），因此台账可满足、`--check` 得 `OK`，行数门用 `total` 基准（+1.3%）。它同时把
  `include/systime.rs` 的十条声明移进 `systime_decls` + glob 导入（timefns 是其中七条的定义
  单元，crate 根声明+定义会 E0428）。
  ② **`bytecode.c` 是 P-03 的第一个完整案例**：冻结配置**确实定义** `BYTE_CODE_THREADED`
  （`__GNUC__` 4、非 `__STRICT_ANSI__`），C 的活解释器是 computed-goto 那支；Rust 没有 `&&label`，
  P-03 定 `BYTE_CODE_THREADED` 是**结构性条件**，故端口抄**switch 形状的那支**，并把
  `CASE`/`NEXT`/`FIRST`/`CASE_DEFAULT`/`CASE_ABORT` 保留为同名 `macro_rules!`（C 的调用形状）。
  这是**语义选择**，台账写明：switch 支送去 `default` 的 opcode，在 threaded 支里走
  `insn_default`（即 `CASE_ABORT` 错误支），而 `BYTE_CODE_SAFE` 正是用来抓这个差异的。
  168 个 opcode 名与值与 166 条臂顺序已机械化比对，`struct bc_frame` 偏移用 clang 实测并钉六条断言，
  单元在 `--cfg enable_checking` 下也编译（P-57）。
  ③ 顺带修一个镜像缺口：`lisp.rs` 的 `record_in_backtrace` 丢掉了 C 的返回值
  （lisp.h 3759 `INLINE specpdl_ref record_in_backtrace (…)`），bytecode 单元因此写了偏离并用
  `SPECPDL_INDEX ()` 补；现在镜像 `return count;`，bytecode.rs 按 C 795-796 的原点取值。

- **3.51 `TIOCSCTTY` 与"已验证清单 vs 抄下两支"的相互作用（2026-10-03 第 86 轮，`df23855`）**：
  ① `src/callproc.c` 的 `#ifdef TIOCSCTTY` 是 §3.35 那一类缺口——`TIOCSCTTY` 来自
  `<sys/ioctl.h>` 而不是 `config.h`，冻结表看不见它，求值器答 False 而 clang 答 True。
  已修：`source-header-macros.tsv` 加 `sys/ioctl.h\tTIOCSCTTY` 行，`gen-cond-macros.py` 用 clang
  实测（`_IO('t', 97)`），表因此多一行；`check-cond-eval.py src/callproc.c` 变为
  `75 conditions, 0 unknown, 0 disagreements [VERIFIED]`，全表重跑 `5792 conditions, 4 unknown,
  0 disagreements`（零回归）。
  ② **但 `src/callproc.c` 故意不进 `cond-eval-verified.txt`**：进了清单的文件，检查器只要求**活支**，
  于是 `USABLE_POSIX_SPAWN` 的 `#define … 1` 会去和端口的 `bool = true` 拼法较劲；而该端口按 §3.49
  **两支都抄**，不进清单反而比对更严（两支都要），也正是端口满足的那一种。这条相互作用写进
  `docs/ports/callproc.md` 的 Integrator note。
  ③ 顺带记录一条待办：`coding_system` 的**结构体赋值**在端口用 `std::ptr::read` 顶（该镜像没有
  `Copy`），台账写明镜像缺口留给 `coding.c` 自己落位，不要在这一轮改别人的镜像。

- **3.52 第 87 轮：`src/insdel.c` 与两条待补的镜像缺口（2026-10-03）**：insdel 是**已验证**文件
  （`24 conditions, 0 unknown, 0 disagreements`），故按 P-08 记死区而非抄两支（`HAVE_TREE_SITTER`
  的十二处、`MARKER_DEBUG`、`USE_MMAP_FOR_BUFFERS`/`REL_ALLOC`/`DOUG_LEA_MALLOC`、八处
  `BYTE_COMBINING_DEBUG`），容差行用 `live` 基准（2,280 活行、9.0%）。它的两个位域写
  （`suspend_auto_hscroll`、`inhibit_shrinking`）经镜像掩码，并新增两个 token-alias family
  （带出现次数校验）；C 2260-2282 那五个**捕获调用者局部量**的语句宏按 3.8 落成同名 `unsafe fn`。
  **两条 integrator 待办**（worker 报回、未越界改别人的镜像，单元在使用点声明）：
  `bset_redisplay`（window.h 1169）与 `record_first_change`（lisp.h 5253）应补进各自镜像。
  权威口径复跑：`checked 342 ported files: 317 done, 25 in progress, 0 failing`。

- **3.53 第 88 轮：`src/composite.c` 与两处镜像补全（2026-10-03）**：composite 是**已验证**文件
  （`1 conditions, 0 unknown, 0 disagreements`），唯一的 `#ifdef HAVE_WINDOW_SYSTEM`（C 959-970）
  按 P-08 记台账不抄写，容差行 `live` 基准（2.98%）。同一提交把 **`composite.h` 镜像做完**
  （121 → 469 行：访问器、`LGLYPH_IX_*`、五个 INLINE、十二条原型；本单元定义的九条原型与两个对象
  移进 `composite_decls`/`composite_objs` + glob 导入）。它的台账列出 14 条使用点声明与五条记录
  （`font`/`font_driver` 的 clang 实测偏移 176/96、`font_metrics`、`composition_it`、部分 `it`）
  留给各自的拥有者。
  **integrator 侧另补两处镜像缺口**（`ccf0686`）：`lisp.rs` 加 `record_first_change`（lisp.h 5254），
  `window.rs` 加 `wset_redisplay`/`fset_redisplay`/`bset_redisplay`（window.h 1167-1169，放进
  `window_redisplay_decls` + glob 导入）与 `enum window_body_unit`（1217-1222）。window.h 镜像仍是
  WIP（下一个未覆盖项为 `MARGIN_IN_PIXELS`），但三个常量与渲染入口已不再缺失。

- **3.54 第 89 轮：整树门又抓到一个镜像回归（2026-10-03，`0ad0c42`）**：`composite.h` 镜像里
  `composition_reseat_it` 的第四参写成了 `*mut window`，而镜像**只允许命名自己声明或自己
  `include!` 链能到达的类型**（P-66/P-72）——`casefiddle.c` 包含 composite.h 却不含 window.h，
  于是 `gate-fast` 的编译步报 `rustc failed for src/casefiddle.rs: cannot find type 'window'`。
  同一行上两个邻居（`struct composition_it`、`struct face`）本来就是 `*mut c_void`，`window` 归队
  并写明理由。**这类缺陷单文件门看不见**（worker 只验证了自己那几个消费者：character/insdel 恰好
  都含 window.h），再次印证 §3.47 的结论。
  修后整条 `gate-fast` 复跑全绿：`symbols: every built unit matches its reference`、七个探针程序、
  三个探针脚本（`fam-probe: 4 lines ok` / `shadow-probe: shadow-probe true 0 true` /
  `macro-probe: every expansion compiles`）、`layout-check passed`；库存口径
  `checked 343 ported files: 319 done, 24 in progress, 0 failing`。

- **3.55 第 90 轮：`src/ccl.c` 落地（2026-10-03，`cebf9a4`）**：2,394 行 C → **3,217 行 Rust**
  （`ccl` 容差行 **`total` 基准 0.36**：解释器分发本身要花行数）。该文件**不在**已验证清单，
  故按 §3.49 抄两支（`CCl_DEBUG` 的回溯机器包在 `#[cfg (feature = "ccl_debug")]`、`#if 0` 只记台账）。
  `ccl_driver` 的五个 label 收成一个 `ccl_goto` 枚举 + 单个 `'ccl_repeat: loop`；九个语句宏与
  `GET_CCL_RANGE`/`GET_CCL_CODE` 按 P-57 定义在函数内部（所捕获局部量之后）；两个 `bool_bf : 1`
  合成 `bits: u8` 并配 token-alias 家族；三个结构的布局 clang 实测 + `const _` 钉死。
  **`ccl.h` 镜像同提交落地**，并执行 P-70 的"一个类型只能有一个镜像"：删掉 `coding.rs` 里的
  `struct ccl_spec` 替身、把该字段降为 `*mut c_void` + `/// C:` 注（4 行，PORTING.md 与该镜像
  自己的注释都要求；两份 ccl 台账都记了）。**报回三条镜像缺口**（未越界改）：`charset.h` 无镜像、
  `lisp.rs` 缺 `larger_vector`/`make_specified_string`、`<intprops.h>` 无镜像（P-26）。

- **3.56 第 91 轮：`src/intervals.c` + `intervals.h` 镜像收口（2026-10-03，`f3406a9`）**：
  2,416 行 C（活 2,410）→ 2,567 行 Rust（`live` 基准 6.5%）；该文件在已验证清单里，唯一的
  `#if 0`（C 1643-1648）按 P-08 记台账。**同一提交做完 `intervals.h` 镜像**（24 条原型进
  `intervals_decls` + glob 导入；六个 `bool_bf` 合成 `bits: u8` 配 `interval_bits!` 与五个新的
  出现次数家族；布局 clang 实测 sizeof 56 / `bits`@40 / `plist`@48）。顺带修掉三个镜像缺陷：
  `LEAF_INTERVAL_P`/`BOTH_KIDS_P` 改回 C 自己的字段比较，`TEXT_PROP_MEANS_INVISIBLE` 改为 `c_int`
  （它两臂原为 `bool`/`c_int`，而 C 拿它与 `2` 比较），`INTERVAL_PARENT_OR_NULL`/`RESET_INTERVAL`
  补上 setter 需要的 cast。**`buffer.rs` 的六条 `buffer.h` 取点声明按 P-70 同提交迁进
  `buffer_point_decls` + glob 导入**（本单元定义它们；28 个受影响的单元仍全部 `built`）。
  该 worker 自己跑过整树 `gate-fast`：`346 ported files: 323 done, 23 in progress, 0 failing`、
  490 个单元编译通过、符号门与布局门全绿。

- **3.57 第 92 轮：`src/charset.c` 与此前**完全没有镜像**的 `charset.h`（2026-10-03，`3972225`）**：
  2,464 行 C → 2,791 行 Rust（`total` 基准 13.3%）。**`charset.h` 原先一个镜像都没有**——三个已端口
  单元（chartab.rs、ccl.rs…）只能在使用点声明它的名字；本轮一次性把它做完整（三个 enum、
  `struct charset`/`charset_table`、63 个宏、两个 INLINE、19 个对象、7 条原型），本单元定义的
  五条原型与 19 个对象进 `charset_decls`/`charset_objs` + glob 导入。六个 `bool_bf : 1` 合成
  `bits: u8` 配 `charset_bits!` 与六个出现次数家族；20 条 `const _` 钉住 clang 实测布局
  （sizeof 320/align 8、`bits`@80、`code_offset`@316）。唯一结构性偏离已记台账且不影响偏移：
  `struct charset_map_entries` 的首成员是**匿名结构体数组**，Rust 必须有名字，故元素命名
  `charset_map_entry` 并加三个零尺寸标记（`entry`@0、`next`@0xC0000、sizeof 0xC0008 全断言）。
  两个"门看不见"的发现：`emacs_fclose` 在 lisp.h 里 `#define` 成 `fclose`，故使用点声明带
  `#[link_name = "fclose"]`（靠读 `nm -u` 发现，**没有任何门会看未定义引用**）；`lisp.rs` 仍缺
  `larger_vector`/`make_specified_string`（留给拥有者）。批边界 `gate-fast` 复跑全绿。

- **3.58 参照开头的版本/版权/许可声明整块删除（2026-10-03 用户指令，同日全树落地）**：
  3.36 只去掉了自己那行版本声明里的产品名括注，并明确"参照文件自己的版权/许可声明与首行标题注释
  照抄"；本轮用户指令把这条反过来：**那一整块也不要**——它正文就是一句"本文件是某产品的一部分"，
  写在本仓库里就是一句关于本仓库的错声明，法律风险正在它身上（`PORTING.md` 0.1-19 已改写）。
  实测：`femacs/src` 与 `femacs/lib` 下 268 个 `.rs` 里 267 个带这种 `/* … */` 块（每个文件恰一块），
  加上 `src/include/lisp.rs` 用 `//` 写的那段，一次机械删除（行首定位，块内含
  "Free Software Foundation"/"part of …"/"General Public License" 任一即认），**-4,932 行**。
  ① **111 个文件的首行原本就是该块**（101 个 lib + 10 个 src），删完就没有任何出处声明了，故按
  0.1-19 补 `// femacs: port of <参照路径> @ fd7afced403.`（补 73 行——其中 17 个文件的出处声明本在
  下面几行，补的那行是重复的，已回退）；现在每个端口文件的出处声明**恰好一条**。全树 378 个
  `.rs` 里只有 10 个没有这条声明：3 个是生成物（`src/include/` 的 `buildobj.rs`、`config.rs`、
  `globals.rs`，生成器写自己的头），7 个是手工写的系统/gnulib 镜像（`lib/cloexec.rs`、
  `lib/openat-die.rs`、`lib/include/` 的 `a_out.rs`、`errno.rs`、`gnulib_config.rs`、`signal.rs`、
  `stdio-impl.rs`）——它们本来就没有这个块，不在本轮范围。
  ② **产品名在代码里的残留只剩三处**，都是 `src/callproc.rs` 的 `DEFVAR_LISP` **docstring 字面量**
  （`data-directory` 等三条）——那是移植语义，P-38 的门要求它与参照逐字一致，不能动；注释里的产品名
  只剩 `src/xterm.rs`/`src/xmenu.rs` 两处描述性标题，三行改写掉（"for <产品名>" 删除）。
  ③ **行数门被这一刀打穿**：`line-drift-tolerance.tsv` 默认以**总行数**为基准，删掉 18–30 行让 44 个
  文件越线（最短的 gnulib 包装件 C 19–33 行，端口只剩 note + 声明，漂移 55%–90%；`realloc`/`sha3`/
  `profiler`/`category` 这类大文件只有 5.6%–29.7%）。44 行全部按"实测漂移 + 0.01"重写/新增
  （25 行改写、19 行新增），理由列写明是这一刀，基准不变；16 个 `live` 基准的行**无**越线。
  ④ **顺手补掉一个 3.31 留下的门缺陷**（本轮的 `--all` 抓出来的）：`compare` 的豁免原来读
  `not_built_on_target` 的**计数**，而"整文件都是死文本"的单元（`dbusbind.c` 的 `#ifdef HAVE_DBUS`、
  `msdos.c` 的 `#ifdef MSDOS`）清出来是 **0 项**，0 是假值——豁免于是失效，空端口被报成 99.5% 漂移。
  现在漂移检查读的是与清账同一条件 `empty_unbuilt = not_built_on_target (c) and 端口无 fn`。
  实测 `src/dbusbind.c`/`msdos.c`/`dosfns.c`/`xsmfns.c` 从 FAIL 回到 OK。**`--all` 复跑：0 failing**
  （351 个端口文件 = 328 done / 23 WIP / 0 FAIL）。
  ⑤ 生成物同步：`docs/port-map.md`（270 行目标行数变化，CI 的新鲜度门会查）、`docs/port-inventory.md`。
  版本/版权信息的**唯一**落点从此是 `docs/baseline` 的冻结基线（`PACKAGE_STRING` 等实测值）与
  0.1-19 本身。

- **3.59 第 94 轮：未定义引用闸门（2026-10-03）**：3.57 记下的那个洞——**没有任何门看未定义引用**——
  本轮补上。此前四道门各看一半：`extract-abi.py` 比**文本**、`layout-check` 比**布局**、
  `check-symbols.py` 比单元的**导出**符号表、`check-globals.py` 比单元**拥有**的声明；而单元
  **引用**了哪些符号没人看，错名字要到加载阶梯的链接阶段才暴露——而链接阶段目前一道门都没有。
  `emacs_fclose` 就是标本：`lisp.h` 把它 `#define` 成 `fclose`，C 的目标文件引用 `_fclose`，
  而一个写成 `_emacs_fclose` 的端口**文本忠实、导出符号全对、根本链不上**。
  新工具 `tools/check-undef.py`，量法是差分而不是手写 libc 白名单：
  **论域** = ① 已构建单元定义的全部符号（`build/femacs/**/*.o`，两种配置都在）∪ ② 冻结参照目标文件
  **定义或引用**的全部符号（`build/emacs-ref/**/*.o` + `lib/libgnu.a`；参照对象引用而 SDK 不导出的
  名字，例如 termcap 的 `_nc_*`，由参照构建本身证明可解析）∪ ③ SDK `usr/lib/libSystem.B.tbd` 导出的
  名字（**宽读**：`.tbd` 里的 re-export 也收，误收只会放宽论域）。三者都不含的 `nm -u` 条目就是
  **孤儿**：本程序的任何配置都无法满足的名字。
  ① 未端口单元拥有的 subr 在移植期间本来就会是未定义的（落地时 **526 个孤儿，几乎全是 `_F*`**），
  所以 `docs/baseline/undef-pending.txt` 是**棘轮**（与 `symbol-pending.txt` 同一件仪器，
  `--write-pending` 重写）：工具只对**新增**的孤儿报错（拼错、发明的 helper、漏写的 `#[link_name]`），
  并把**已经满足**的棘轮条目报成 stale，让它随移植缩小。
  ② 反向验证（不是"跑绿了"）：往 `build/femacs/` 里放一个引用 `emacs_fclose` 的 clang 目标文件，
  闸门**恰好**报一条 `_emacs_fclose` 并以 1 退出；删掉探针后回到 `526 pending, 0 new`。
  ③ 接线：`xtask gate-fast` 在 **symbol gate 之后**加一段 `== undefined-reference gate ==`（必须在
  单元构建之后才有目标文件可比）。CI 的快档不接它——那档只 `cargo check`，不产出 `.o`；这条门属于
  `gate-fast` 这一档。

- **3.60 第 94–95 轮：五个单元并行、`DEFVAR` 载体符号的定案、两个"门看不见"的缺陷（2026-10-03）**：
  本轮同时开五条实现通道（`src/minibuf.c`、`src/print.c`、`src/emacs.c`、`src/sysdep.c`、
  `src/search.c`，后期加 `src/keymap.c`），落地三件：**minibuf 2,759→2,779 行**、
  **print 3,069→3,297 行**、**search 3,514→3,846 行**（合计 9,342 行 C → 9,922 行 Rust，各自
  `--check` OK、台账 0 未勾、`--cfg enable_checking` exit 0）。单元数 **248 → 251**。
  ① **`DEFVAR_*` 载体符号定案（`build/emacs-ref/src/print.o` 逼出来的）**：C 把 `DEFVAR_*` 的存储
  放进唯一的 `globals` 对象（`globals.f_<name>`，lisp.h 3495-3530），所以参照目标文件导出 `globals`
  而**没有** `_Vprint_level` 这类符号；print 通道据此把 18 个载体写成私有 `static mut`。
  **这是错的**：`include/globals.rs` 把名字镜像成 `unsafe extern "C"` 里的**外部声明**，而
  `keyboard.c`（未端口）读 `Vprint_level`——载体私有化会让 `_Vprint_level` 在链接期无处可寻
  （这正是 3.59 那道门要防的类，只是当下没有对象引用它，门看不见）。定案：**§3.29 不变**
  （载体 `#[unsafe(no_mangle)] pub static mut`，由本单元定义），并把这条**系统性符号偏离**装进
  `tools/check-symbols.py`：它从参照源码解析本单元的 `DEFVAR_LISP/BOOL/INT/LISP_NOPRO` 名字，
  把对应的 `_*` 从 invented 里扣除并**计数报告**。实测 `--dir src` →
  `1 units compared, 0 mismatched; 18 DEFVAR carrier symbol(s) are the port's documented deviation`；
  `--dir lib` 仍是 `78 units compared, 0 mismatched`。
  **顺带打开一扇窗**：此前 src 单元一个参照目标文件都没有，`--dir src` 的比较是**空的**；现在
  `make -C build/emacs-ref/src <stem>.o` 能逐个把 src 的符号门变成真的（print 是第一个）。
  ② **`cinventory.py` 的真缺陷（emacs.c 通道发现，已修）**：`function_spans` 的前瞻碰到"声明符与
  `{` 之间夹着一条死 `#if` 行"就停（emacs.c 的 `main` 正是这个形状），于是 `main` 根本不是该文件的
  函数——端口的 `#[unsafe(no_mangle)]` 定义被报成"发明的导出符号"，正文也没有 token 检查。
  修好后**独立复算**：8 个 PORTED_DIRS 的 **590 个参照文件**用新旧两个模块各做一次完整清单，
  **只有 `emacs/src/emacs.c` 变化**（`('main', 1317)` 出现）。
  ③ **两个"门看不见"的缺陷已记录、待修**：(a) `tools/extract-abi.py` 的**字符串扫描先于字符字面量
  剥离**——`'"'` 这个字符字面量会开启一个"字符串"，Rust 侧因此吞掉 `print_bool_vector` 里其后全部
  `printchar` token 并让函数体越过 `}`，C 侧则把字符串内容（`data`、`table`、`04`）漏进 token 集合
  （print 通道发现，行内记为 work-around 而非改门）；(b) **非 static 文件级对象的连接检查缺失**——
  `re_match_object` 少了 `#[unsafe(no_mangle)]`，单元发出的是 `__RNv…_6search15re_match_object` 而不是
  `_re_match_object`，三道文本门全绿（对象规则只看名字）。search 通道手工发现并修好
  （`nm -g` 复核 `S _re_match_object`）；**待办**：把"每个非 static 文件级对象必须发出同名 C 符号"
  加进 3.59 那道门。
  ④ **镜像缺口台账（由本轮三条通道汇总）**：`regex-emacs.h` 镜像仍缺 `struct re_pattern_buffer`、
  `re_*` API 与 `print_compiled_pattern`（search 在使用点自带一份，**镜像补齐时必须同一提交删除**，
  否则 E0428）、`region-cache.h` 完全没有镜像、`lisp.rs` 缺 `make_specified_string`/`substring_both`/
  `copy_text`、`window.rs` 缺 `set_window_buffer`/`resize_mini_window`/`restore_window_configuration`/
  `WINDOW_FRAME`/位设置器、`keyboard.rs` 缺四个 keyboard 入口、`<string.h>` 缺 `memrchr`、
  `<limits.h>` 缺宽度族、`systty.h`/`<ieee754.h>` 无镜像（后者是 gnulib 模板，P-26）。
  ⑤ 批边界实测：`extract-abi.py --all` → **357 个端口文件 / 332 done / 25 WIP / 0 failing**；
  `tools/check-undef.py` 棘轮重写为 **558 pending / 0 new**（新单元带来的未定义名全部是未端口单元
  拥有的依赖，附引用者）；`gen-port-map.py`、`gen-port-inventory.py` 已重跑（done 324 / wip 33 /
  todo 233）。`gate-fast` 待本批最后一个单元落地后跑（五条通道同时在写，整树重编译没有意义）。

- **3.61 第 95 轮：连接闸门（`tools/check-linkage.py`）与 `src` 侧符号闸门接线（2026-10-03）**：
  3.60 记下的第二个洞——"非 static 文件级对象忘了 `#[unsafe(no_mangle)]` 没有任何门能看见"——
  本轮补上。新工具 `tools/check-linkage.py` 只查一个方向：**`cinventory` 在 C 源里找到、且 C
  没有标 `static` 的每个文件级对象，都必须在端口目标文件里是一个导出符号**（另一个方向由
  `check-symbols.py` 负责）。
  ① **三个过滤器都是实测出来的，不是猜的**（工具的第一版报了 43 个单元 / 189 个"缺符号"，全是噪声）：
  (a) **`extern` 声明不是定义**——`menu.c` 的 `extern AppendMenuW_Proc unicode_append_menu;`、
  `emacs.c` 的 `extern unsigned char etext asm ("etext");`、`callproc.c` 的 `environ` 都属于这一类；
  (b) **死臂不定义任何东西**——`emacs.c` 的 `Vlibrary_cache`(WINDOWSNT)、`display_arg`
  (HAVE_X_WINDOWS/HAVE_PGTK/HAVE_NS 全未定义)、`w32_daemon_event`、`treesit` 的常量，用冻结条件求值器
  逐行判定（文件在已验证清单里时）；(c) **冻结构建不编译的 `src/*.c` 不在链接里**——`tparam.c` 在 C 里
  `char *BC; char *UP;` 是暂定定义，但该文件**不在** `frozen-build-sources.txt`，冻结构建的这两个对象
  来自 `terminfo.c`，端口也确实定义在 `terminfo.rs`。
  ② **两处放宽/分桶**：配置期改名（`#define optarg rpl_optarg` 在生成的 config 里，源扫描看不见）
  会让 `_rpl_re_syntax_options` 被误报成缺 `re_syntax_options`，故**后缀匹配即算存在**（名字够长，
  不会掩盖真缺失）；文件不在 `cond-eval-verified.txt` 时条件未知，这类发现进"需人工判定"桶而**不**失败
  （`ccl.c` 的两个对象在 `#ifdef CCL_DEBUG` 里，台账已记）。
  ③ 实测：**20 个单元的冻结对象全部导出齐备，0 缺失**，1 个待判定（ccl，已知死臂）；`gate-fast` 已接线
  （紧跟未定义引用闸门之后）。同一轮还把 `check-symbols.py --dir src` 接进 `gate-fast`——此前它只跑
  `--dir lib`，而 src 侧现在**不再空转**（print.o 是第一个 src 参照对象，`--dir src` 报
  `1 units compared, 0 mismatched` + 18 个 DEFVAR 载体计数）。

- **3.62 第 96 轮：字符字面量/字符串的扫描顺序——一个"门把自己蒙住"的缺陷（2026-10-03）**：
  print 通道在台账里记下的那条（`docs/ports/print.md`：`print_bool_vector` 里其后的 `printchar`
  token 全被吞掉，C 侧则把字符串内容 `data`/`table`/`04` 漏进 token 集合）本轮查实并修掉。
  根因在 `tools/extract-abi.py` 的 `body_token_seq`：**先**用正则把字符串字面量抹成 `""`，**后**才
  剥字符字面量；于是 `'"'`（Rust `b'"'`、C `'"'`）里的那个引号被当成字符串的开引号，一直到**下一个
  真正的字符串**之间的全部文本被抹掉——C 侧和 Rust 侧**同时**被抹，所以 token 比对看不见差异，门报绿。
  ① **最小复现**（修前）：`let c = b'"'; if foo (bar) { baz ("q"); }` 的 token 序列是
  `['let','c','b','q']`——`foo`/`bar`/`baz` 全没了；C 侧 `if (c == '"') { foo (always, present, "q"); }`
  的 `always`/`present` 同样消失。
  ② **修法**：把两步对调（先剥字符字面量）。`strip_char_literals` 是**有界**的（最多 12 个字符、
  不跨行），所以不会吞掉 lifetime（`'a`、`'static`）或注释里的撇号——修后四条用例逐条复核：目标 token
  回来了、字符串内容仍然被抹、lifetime 仍然完好。
  ③ **门因此变严了，而且立刻抓到东西**：`--check` 对全树重跑后，`src/print.c` 与 `src/json.c` 从绿翻红，
  差异正是被那个畸形扫描藏住的两处——`print_object` 少了 `default,pridmax`（C 的 `default:` 与
  `. #%" PRIdMAX ")`，端口写 `_ =>` 与拼接常量 `"jd"`），`json_out_string` 少了 `switch`（C 的
  `switch (c)` 与端口的 `match`）。两条都属**已记录偏离**类，故 `docs/ports/macro-body-notes.tsv` 的
  对应两行按机制**加长 token 列表**并写明这是扫描顺序修复后新可见的 token（**不是**放宽门，是让门
  能看见）。
  ④ 全树复核：`extract-abi.py --all` → **357 个端口文件 / 333 done / 24 WIP / 0 failing**。
  这条记录的意义是：一个"吞 token"的扫描缺陷既能让门误报，也能让门**漏报真差异**；修好后是后者被
  消灭，不是前者被引入。

- **3.63 第 96 轮：条件文本里的尾随注释——clang 探针被自己写坏（2026-10-03）**：sysdep 通道报的
  那 2 个 `check-cond-eval` disagreement（C 1135 `#ifdef INLCR /* … */`、C 1279
  `#ifndef HAIKU /* … */`）根因查明并修掉。`tools/cinventory.py` 的 `CONDITION_LOG` 按设计保存
  **参照行自己的文本**（注释还在），而 `tools/check-cond-eval.py` 拿它直接拼 clang 探针——C 在
  **翻译阶段 3** 就去掉注释（早于读指令），于是探针里那句 `#ifdef INLCR /* …` 是**语法坏的**
  （`unterminated /* comment`，随后 `missing ')' after 'defined'`），clang 于是对一条冻结构建**会**
  编译的分支回答"dead"，被记成 disagreement。修法：探针侧在建探针前把 `/* …` 去掉（求值器侧本来就
  已经这么做了，只有探针漏了）。实测 `src/sysdep.c` 从 **2 个 disagreement 变为 `283 conditions,
  0 unknown, 0 disagreements`**；全表 `--verified` 复跑 **5717 conditions, 4 unknown,
  0 disagreements**（无回归）。注意：**不**因此把 `sysdep.c` 写进已验证清单——它按 §3.49 抄两支，
  而 §3.51 明确"已验证清单"与"抄两支"互斥（`--write-verified` 会因为 0 disagreement 把它收进去，
  所以这一步不能顺手跑）。

- **3.64 第 96 轮：`config.rs` 的 `SYSTEM_TYPE` 与它背后的生成器缺陷（2026-10-03）**：emacs 通道记下
  的那个镜像缺陷（`config.rs` 存的是 config.h 宏**体**而不是 C 字符串值，于是使用点只能写
  `c"darwin"`，并把 `SYSTEM_TYPE` 记进 `macro-body-notes` 当豁免）本轮修掉，连带挖出**两个更深的
  生成器缺陷**：
  ① **`xtask gen-config` 选错了输入**：它按名字排序取 `docs/baseline/config/` 的**第一个**文件，
  而 `cond-eval-verified.txt` 排在 `darwin-arm64.json` 前面——我第一次重跑就把 `config.rs` 从 29 行
  写成 3 行（26 个常量全丢；因为它是生成物，这个错误离被提交只差一条命令）。现在按 `.json` **过滤**，
  找不到就报错。
  ② **`read_config` 是行式 JSON 读器，从不反转义**：`"SYSTEM_TYPE": "\"darwin\""` 交给程序的
  值是 `\"darwin\``（JSON 转义原样保留），所以取值端连"它是不是 C 字符串字面量"都判断不出来。
  现在引号值经 `json_unescape` 解码，其余条目行为不变（布尔/数字/`undefined` 那些字符串解出来还是
  原样）。
  ③ 修好之后：`gen-config` 把 C 字符串字面量解码成 **`&CStr`**（`pub const SYSTEM_TYPE:
  &core::ffi::CStr = c"darwin";`），与 `emacs.rs` 里 `PATH_EXEC` 等 config.h 字符串同形；`emacs.rs`
  的使用点改回 C 的表达式 `intern_c_string (SYSTEM_TYPE.as_ptr ())`（C 写
  `intern_c_string (SYSTEM_TYPE)`），`macro-body-notes.tsv` 的 `syms_of_emacs` 行**退回**只允许
  `0,doc`。实测：`config.rs` 相对 HEAD 只有 1 行差异；`emacs.rs` `build-tu` built、`--check` OK；
  另取三个消费者（`indent.rs`、`minibuf.rs`、`lib/timespec.rs`）重建，全部 built。
  **教训写在这里**：一个"生成的镜像"出问题时，先问生成器读的是不是它声称的那个文件——这次是
  "读错文件"和"不反转义"两层叠在一起，只看 `config.rs` 的错值永远查不到第二层。

- **3.65 更正：`44fa275`（keymap）的提交信息有三处与实测不符（2026-10-03）**：该提交的信息是集成者
  写的，其中三处说法**没有对照台账**，与 keymap 通道的实测报告冲突——① 它说 `src/keymap.c` 有
  `HAIKU`/`WINDOWSNT`/`HAVE_X_WINDOWS` 死臂：**该文件一条预处理器条件都没有**
  （`check-cond-eval.py src/keymap.c` → `0 conditions, 0 unknown, 0 disagreements [VERIFIED]`，
  P-08 无事可记，§3.49 不适用）；② 它写"36 个函数、9 个文件级对象"，台账实测是 **37 个函数、
  10 个对象**（79 行清单全勾，`docs/ports/keymap.md`）；③ 它只列了 5 个镜像消费者，实际重建的是
  **11 个**（`casefiddle`/`category`/`cmds`/`data`/`doc`/`emacs`/`intervals`/`macros`/`menu`/
  `minibuf`/`undo`）。历史提交不改写（禁止 force-push），以台账与 `docs/ports/include-keymap.md`
  的 Integrator note 为准。
  **纪律（写进来以示后）**：提交信息里的**每一个数字**和**每一条 dead arm 的断言**，都必须能指出
  当轮命令输出或台账里的出处；指不出就不写。这次是集成者按"这类文件通常有平台死臂"的**先验**
  写了断言，而 keymap.c 恰好是一条条件都没有的那一种——正是本轮 `extract-abi`/`check-cond-eval`
  反复强调的"门的结论要来自测量"在文档层的同一课。

- **3.66 第 96 轮：`lisp.rs` 的 `maybe_disable_address_randomization` 签名偏离修掉（2026-10-03）**：
  emacs 通道记的最后一条镜像偏离——`lisp.h` 5274-5280 的活支是
  `INLINE int maybe_disable_address_randomization (int argc, char **argv) { return argc; }`
  （`HAVE_PERSONALITY_ADDR_NO_RANDOMIZE` 在冻结配置里是 `undef`，所以 `sysdep.c` 里那个双参数**定义**
  是死文本，`sysdep.rs` 按 §3.49 照抄在 cfg 里），而 `lisp.rs` 只留了它读的那个参数，于是 `emacs.rs`
  的调用点写成单参数。现在镜像保留 C 自己的两参数（第二个参数 C 也不用），调用点写回
  `maybe_disable_address_randomization (argc, argv)`，与 C 的形状一致。
  实测：`emacs.rs` `build-tu` built、`--check` OK；另一消费者 `sysdep.rs` rebuild built；
  `spikes/` 里没有调用点（不必改探针）。

- **3.67 第 96 轮：src 侧符号门从"空转"变成真的，并当场抓到 4 个真缺陷（2026-10-03/04）**：
  3.61 留下的那句话——"src 侧符号门对没有参照目标文件的单元是**空转**的"——本轮把它变成真的：
  `make -C build/emacs-ref/src <stem>.o` 逐个构建参照目标文件（`minibuf`/`print`/`search`/`emacs`/
  `sysdep`/`keymap` 六个），`check-symbols.py --dir src` 于是第一次真的比符号表，**当场报出 4 个真缺陷**：
  `emacs.rs` 的 `syms_of_emacs`、`sysdep.rs` 的 `set_random_seed`（三个 cfg 臂）与 `renameat_noreplace`
  ——C 里这四个都是**内部链接**（`static`），端口却给了它们**外部名字**；参照目标文件是权威证据
  （`nm -g build/emacs-ref/src/*.o` 四个名字一个都没有）。四个都已改成 C 的链接形状。
  ① **根因在扫描器，不在端口**：`tools/cinventory.py` 判定 `static` 只看**声明符上一行**，于是两种真实
  形状全丢：`#ifdef _LIBC / static / #endif / int / try_tempname_len (…)`（sysdep.c 的
  `#ifndef HAVE_ANDROID` 同形）与"先 `static` 声明、后无关键字定义"（emacs.c 268 的
  `static void syms_of_emacs (void);` 管着 3477 的定义，C 6.2.2p4）。修法三段：
  (a) **说明符窗口**：从**声明符那一行本身**起向上走（单行定义 `static void f (…) { … }` 的
  `static` 就在这一行），跨过属性行、说明符行与预处理行；
  (b) **名字规则只在可信处生效**：新增 `BLINDED` 标志，条件表被丢掉的未验证文件不再套用"先 static
  声明"规则——`lib/tempname.c` 的 `#if _LIBC` 原型是**死**的（它的定义在 C 里是外部符号，参照目标文件
  导出 `_try_tempname_len`），而盲区里它看着是活的；
  (c) **预处理决定链接时，文本一律不下结论**：窗口里跨过 `#` 行（或命中名字规则）就标记
  `conditional_static`，`extract-abi` 的**两个方向**都让开——这一类交给目标文件级的两道门
  （`check-symbols.py` / `check-linkage.py`）。**实测**：全部 634 个参照文件里 `static` 判定变化的
  只有 **12 个函数**，逐个用参照目标文件核对过方向（emacs/sysdep 的 3 个在 C 里是内部，tempname 的
  2 个在 C 里是外部——正是"文本判不了、对象判得了"的那一类）。
  ② **P-17 类计入而非报错**：C 通过 `globals.h` 把每个 Lisp 符号做成**编译期常量**，参照目标文件因此
  导出 **约 2,100 个 `_Q*`**；Rust 无法把符号表地址折进常量，端口把它们写成内联函数（`Qnil ()`），
  一个都不导出。`check-symbols.py` 现在从 `docs/baseline/globals/symbols.tsv` 与
  `DEFINE_GDB_SYMBOL_BEGIN` 推出这一类并**计数报告**（与 3.60 的 DEFVAR 载体同一机制）。
  **实测**：`--dir src` → `6 units compared, 0 mismatched`（72 个 DEFVAR 载体 + **1,567** 个 P-17 折叠
  符号被计数）；`--dir lib` 仍是 `78 units compared, 0 mismatched`。
  ③ **原则写明白**：**文本能判的用文本，文本判不了的用目标文件**。这一轮之前只有前半句，于是
  "先 static 声明"和"预处理决定链接"两类被判错，而错误的判定会**要求**错误的一比一（端口被迫发出
  C 没有的符号）。

- **3.68 第 96 轮：gnulib 的 `rpl_` 改名与一次"差点丢文件"的教训（2026-10-04）**：3.67 让 src 侧符号门
  变成真的之后，给 20 个已端口 src 单元补了参照目标文件（`build/emacs-ref/src/*.o` 从 7 个增到 27 个），
  第一次比对就报出 `src/regex-emacs` 的**符号名不一致**：冻结的 `src/config.h` 3642-3663 把 11 个
  `re_*` 标识符 `#define` 成 `rpl_re_*`（gnulib 的替换件方案），所以 C 的目标文件导出的是
  `_rpl_re_search` 等，而端口导出的还是 `_re_search`——**这是真缺陷**（本项目自己的 `lib/` 侧早就用
  改名后的拼法，例如 `regcomp.o` 的 `_rpl_re_syntax_options`）。修法与 3.57 的 `emacs_fclose` 同一形状：
  5 个**定义**写 `#[unsafe(export_name = "rpl_…")]`（`re_compile_pattern`/`re_search`/`re_search_2`/
  `re_match_2`/`re_set_registers`），镜像里 5 条**声明**写 `#[link_name = "rpl_…"]`；调用方（search.rs、
  syntax.rs 等）继续用 Rust 名，无需改动。实测：`nm -g build/femacs/regex-emacs.o` 与参照目标文件
  导出**同名同型**的 5 个符号；三个消费者重建 built；单元与镜像的 `--check` 都 OK；
  `check-symbols.py --dir src` 现在只剩在飞的 `src/lread` 一项。
  **代价与教训（写下来）**：我第一次改镜像的脚本在**循环里对"重新读回的文件"反复写**，
  一次写入把 `femacs/src/include/regex-emacs.rs` 从 255 行截成 216 行——那是**未提交**的他人成果
  （regex-emacs 通道刚完成、尚未合入）。恢复路径是本项目自己的机制：`xtask materialize` 会把每个
  `include!` 的文件**复制**进 `build/femacs/units/<unit>/`，其中两份 255 行的副本正是完成版；
  `cp` 回来后 `--check emacs/src/regex-emacs.h` **OK** 即为"恢复的是正确内容"的证据，随后改为
  **一次性行级改写 + 前后行数断言**（255→255、+10 行后 266）。两条纪律：**机械改写必须断言前后规模**；
  **materialize 的副本是免费备份**（任何镜像被写坏时的第一恢复源）。

- **3.69 第 96 轮：`safe_calln!` 的镜像缺陷（行为级，6 个调用点）——由 eval 通道发现并修好（2026-10-04）**：
  C 的 `safe_calln` 是 `CALLMANY (safe_funcall, ((Lisp_Object []) {__VA_ARGS__}))`（lisp.h 4845-4847），
  即"走 `safe_funcall`（会捕获错误、处理 `inhibit-redisplay`）"；而 `lisp.rs` 的 `safe_calln!`
  展开成 **`calln!`（`Ffuncall`）再加一个 C 根本没有的 `maybe_quit ()`**——**六个调用点**
  （composite/keymap/print/syntax/terminal/eval）因此丢掉了错误捕获与 inhibit-redisplay 处理。
  这不是符号问题而是**行为**问题，所以它没有触发任何静态门（宏展开后调用的是另一个函数，两次调用
  的形式参数都合法）。eval 通道没有将就它，而是在 `safe_eval` 里照 C 展开、并把缺陷报回镜像 owner。
  修法：镜像正文改成 C 的形状 `CALLMANY! (safe_funcall, arr)`（`arr` 用 `CALLN!` 同款
  `stringify!` 计数，参数只求值一次，与 C 的复合字面量一致），并**新增 `lisp_safe_decls` 模块 +
  glob 导入**声明 `safe_funcall`（C 在 lisp.h 4844 声明它，`eval.rs` 定义它；同作用域声明会 E0428，
  故按 `keymap_decls` 形状放进模块）。这里有一个 Rust 细节值得记：**`macro_rules!` 正文里的裸路径在
  *定义处* 解析**（mixed-site hygiene），所以宏要能命名 `safe_funcall`，declaration 就必须在
  `lisp.rs` 里，而不是在六个调用点各写一遍。
  实测：六个调用点单元全部重建 `built`；六个单元的 `--check` 全部 `OK`；`lisp.h` 镜像 `--check` 仍
  `OK`；全树没有第二处 `fn safe_funcall`（否则会重复定义）。

- **3.70 第 96 轮：`lisp.rs` 缺 `<stdio.h>`——一个只有隔离跑整树门才暴露的真缺陷（2026-10-04）**：
  本轮改用**在 `git worktree`（detached，HEAD）里跑 `gate-fast`**来独立验证"已提交状态"，把在飞通道写进
  工作树的 WIP 文件排除在外（否则 `--all`/`gate-fast` 会把没有台账的半成品单元报成 failing——本轮
  `src/process.c` 刚开工时就是这样）。隔离跑立刻暴露一个真缺陷：`src/atimer.rs` 在 `--cfg enable_checking`
  下**编译失败**（`cannot find type FILE`），而默认配置下它是绿的——因为出错的是
  `regex-emacs.h` 199 的 `extern void print_compiled_pattern (FILE *dest, …)`，它只在 `ENABLE_CHECKING`
  下活，而 C 的 `regex-emacs.h` 只 include `<stddef.h>` 与 `lisp.h`：**`FILE` 是从 `lisp.h` 第 34 行的
  `#include <stdio.h>` 来的**，所以 C 的每个翻译单元都能看见它。
  ① 端口的 `lisp.rs` **没有**镜像那一行，反而让每个"需要 `FILE`"的单元自己 include
  `lib/include/stdio.rs`（emacs.rs 131、search.rs 62、syntax.rs 32 —— syntax.rs 的注释甚至写着
  "lisp.h 34: FILE"）。于是 `atimer.rs` 这种"include 了 lisp.rs 与 regex-emacs.rs、但不需要 FILE"的单元
  在检查模式下没有 `FILE`，而**默认构建永远看不到**（那条声明被 cfg 掉）。
  ② 修法是补齐 C 的形状：`lisp.rs` 在自己的 include 链里加上
  `include!("../../lib/include/stdio.rs");`（注释写明 C 的 lisp.h 34 与它为什么必须在这里）。
  `xtask materialize` 按**文件路径**去重，所以自己已经 include stdio 的单元仍然只有一份副本 ✓（实测：
  search/syntax/emacs/eval/thread 五个消费者全部重建 `built`；`--check emacs/src/lisp.h` 仍 OK；
  `atimer` 的 `--cfg enable_checking` 编译 **0 error**）。
  ③ **方法记下来**：共享工作树里同时在飞多条通道时，整树门的红灯既可能来自在飞文件，也可能来自已提交
  状态——**用 `git worktree` 在 HEAD 上隔离跑一次**就能分清这两件事。这一次正是隔离跑才把一个被噪声盖住的
  真缺陷（P-57 那一类：只有检查模式才编译的声明）挖出来。
  ④ 过程里我自己的一个失误也记下来：第一版补丁只写了注释、**漏了 `include!` 那一行**，靠"检查材质化
  目录里有没有 stdio 副本"这个**效果检查**才发现；二次补丁加完 include 后不仅查了行是否在文件里，也重跑
  了那个失败的编译模式。**机械改写的验证要落在效果上，不能只看补丁文本。**

- **3.71 第 96 轮：`modiff_incr` 丢掉返回值（镜像缺陷，buffer 通道发现并修好）（2026-10-04）**：
  C 的 `modiff_incr`（lisp.h 4142-4154）把计数器加完**返回加之前的值**（`return a0;`）；端口的
  `lisp.rs` 写成了 `-> ()`，只更新 `*a` 不返回值。这是**门看不见的**那一类：token 比对里 `return` 属于
  被折叠的词，正文其余 token 完全一致，所以 `--check` 一直绿；buffer 通道在移植
  `Frestore_buffer_modified_p` 时发现，并**没有**顺手把它绕过去写坏，而是照 C 写成"先读 `MODIFF`"
  的等价形状，同时把缺陷报回镜像 owner。
  修法：`modiff_incr` 返回 `i64`（`a0`），与 C 一致；实测 `--check emacs/src/lisp.h` 仍 `OK`，
  五个消费者（insdel/textprop/editfns/intervals/buffer）重建全部 `built`（把返回值当语句丢弃的调用点
  在 Rust 里天然合法，所以不必改调用点）。buffer 通道那处"先读 MODIFF"的等价写法保留——它与 C 的
  语义相同，且属于在飞文件，不在本轮改动内。

- **3.72 第 97 轮：参照对象从 29 个扩到 63 个，又抓到两个真缺陷（2026-10-04）**：把"给已端口 src 单元
  构建参照目标文件"这件事从 29 个做到 **63 个**（`build/emacs-ref/src/*.o`），`check-symbols.py --dir src`
  于是真的比了 **63 个单元**，又报出两处（都属 3.67 那一类——只有目标文件能判的符号问题）：
  ① **`character.c` 的 `hexdigit` 少了 `#[unsafe(no_mangle)]`**：C 里它是非 `static` 的文件级对象
  （`signed char const hexdigit[UCHAR_MAX + 1]`），C 的目标文件导出 `_hexdigit`，而端口发出的是被 mangle
  的 `__RNvCs52…_9character8hexdigit`——与 3.60 的 `re_match_object` 同一类。修好后
  `nm -g build/femacs/character.o` 是 `S _hexdigit`。
  ② **`terminfo.c` 多定义了三个对象**：端口的 `terminfo.rs` 定义了 `UP`/`BC`/`PC`，而 C 的目标文件
  **一个都不导出**——因为 `#ifndef TERMINFO_DEFINES_BC`（C 29-31）在冻结配置里是**死臂**：
  `build/emacs-ref/src/config.h` **2358 行**就是 `#define TERMINFO_DEFINES_BC 1`（冻结宏表 3740 行同样
  是 `1`，`check-cond-eval.py src/terminfo.c` → `1 conditions, 0 unknown, 0 disagreements`），这三个
  对象由 terminfo 库提供。端口的老注释写的是"该臂是活文本"，与 config.h 相反——**它是照着"表里没有
  这个宏"的旧印象写的，而不是照着 config.h**。按 P-08（该文件在已验证清单里）改成"记录不抄"：三个
  定义删除，死臂写在台账里；`cm.rs` 的 `UP = 0; BC = 0;` 与 C 的 `cm.c` 一样，赋值给库里的对象。
  ③ 实测：`check-symbols.py --dir src` → **63 units compared, 1 mismatched**，唯一不符的是**在飞的**
  `src/process`（WIP）；`terminfo.o` 现在导出 `_BC/_UP/_PC` **0 个**；两个单元的 `--check` 都 OK；
  `cm.rs` 重建 built。**结论**：这一轮之前 src 侧的符号门在多数单元上是空转的；把参照对象逐个建起来
  之后，它连着抓出 `rpl_re_*`（3.68）、`hexdigit` 与 terminfo 三个对象（3.72）——**文本门看不见的东西，
  目标文件看得见**。

- **3.73 第 97 轮：产品名只剩在字面量里——一条规则变成一道门（2026-10-04）**：两条用户指令
  （2026-10-03）已经把"声明"从树里清过两遍：`2ee1d1d`/`4810bb0` 把 231 个单元/镜像首行的
  `@ <修订号> (<产品名> x.y.z)` 括注删到只剩修订号，`fd9cf89` 又删掉 268 个文件开头整块照抄的
  版本/版权/许可声明（-4,932 行）。两者都靠一次性机械改写完成，**没有任何门**——而这之后落的每一个
  单元都要照抄它那份 C 的注释（`0.3` 的"注释逐字保留"），于是**复发的路是敞开的**：
  `src/emacs.c`（`79a41a7`，比 `fd9cf89` 晚 1 小时 22 分）就把产品名的版本/版权声明带回了
  `src/emacs.rs`。本轮的交付是把这条规则做成门，并把残留清到零：
  ① `tools/check-declarations.py`：把 393 个 `.rs`（`femacs/src` 180 + `femacs/lib` 211 +
  `femacs/xtask` 1，193,877 行）按"注释/字面量"分开扫（可嵌套 `/* */`、`r#"…"#`、`c"…"`、
  生命周期 `'a` 与字符字面量 `'a'` 逐一分清，否则规则要么漏掉整块声明、要么在每个 docstring 上误报），
  分成四类：`decl`（复制的所有权/许可块，268 块的形状）、`version`（版本行里带产品名的括注）、
  `title`（注释里其余的产品名，含 `/// C …` 引文）、`literal`（字符串字面量里的产品名）。
  ② **实测残留只剩 8 处，全是 `literal`**：`src/callproc.rs` 三条 `DEFVAR_LISP` docstring
  （C 2120/2124/2135）与 `src/emacs.rs` 五处（`PACKAGE_NAME`、`COPYRIGHT`、`RCS_Id`、
  版本横幅 printf、`Vemacs_bugreport` 的 docstring）——都是 C 自己的字面量，docstring 门逐字对拍，
  改了就与参照不符。它们逐条写进 `docs/baseline/declarations-allowlist.tsv`（含理由），
  是**计数偏离**不是隐藏。
  ③ **唯一一条注释里的产品名**是 `src/emacs.rs` 的 `/// C 163-167:` 引文（它引用 C 的 `RCS_Id`
  定义），按 0.1-19 改成 `<PACKAGE_NAME>` 占位并就地注明原因，引文其余部分逐字不动；台账
  `docs/ports/emacs.md` 记了这条偏离。改完重扫：`decl`/`version`/`title` **各 0 处**。
  ④ 门的两条自证：`--selftest` 13 例（复制块报 `decl`+`title`、`c"…"` 里的版权行按位置算 `literal`、
  `'a` 生命周期不误报、`r#"…"#`、嵌套块注释、`@ <rev> (gnulib)`/`(PORTING.md 0.1-2)` 这类**出处注**
  不误报为 `version`——第一版正则正是被这两行打回重写的，`@ … (` 太宽）；以及负向探针：
  拿一个合成根 `--root`（四类各一条）跑，**4 sites / 4 forbidden / exit 1**，而真实树
  `8 declaration sites (0 forbidden, 8 allowed literals), 0 failures`，耗时 1.3 s。
  ⑤ 已接进 `gate-fast`（`== declaration gate ==`，紧接连接门之后）。仍**不在**范围内的是两处实测值：
  `docs/baseline/config/darwin-arm64.macros.tsv` 的 `PACKAGE_STRING`，以及六棵逐字节照抄树
  （`lisp`/`test`/`etc`/`doc`/`leim`/`admin`）里参照自己的版权头——前者是 clang 实测的冻结值，
  后者是"原样复制"本身（`tools/check-verbatim.py` 要求逐字节相等）。

- **3.74 第 98 轮：`region-cache.h` 的镜像落地——P-26 的"第二个 includer 就该建镜像"终于在第五个 includer 上兑现（2026-10-04）**：
  这棵树里**五处**写着同一句话"`region-cache.h` has no mirror"（`bidi.rs`/`insdel.rs`/`search.rs`/`indent.rs`
  各自的 use-site 声明区，加上 `region-cache.rs` 自己的 include 说明），每个单元都把那六条原型抄了一遍、
  `struct region_cache *` 各按 `*mut c_void` 拼一次——正是 P-26 说的"同一构造不允许两种写法"的灰色地带。
  ① 新增 `femacs/src/include/region-cache.rs`（70 行）：`EMACS_REGION_CACHE_H` 宏 + `region_cache_decls`
  模块里的六条声明 + glob import（0.1-10），`--check` 与参照头一致。
  ② **两种指针分别按各自规则拼**：`struct region_cache` 在头里是**不完整类型**（记录是 `region-cache.c` 的），
  按 0.1-2/P-72③ 拼成 `*mut c_void`——与 `include/buffer.rs` 里 `newline_cache`/`width_run_cache`/
  `bidi_paragraph_cache` 三个字段早就用的拼法一致；`struct buffer *BUF` 则**保持真类型**：C 59 只是
  前置声明，`include/buffer.rs` owns 那条记录，镜像 `include!("buffer.rs")` 后就能写 `*mut buffer`
  （`intervals.rs` 的 `offset_intervals` 先例），也顺带让 layout 探针的顺序无关（0.1-4 的包含图保证
  buffer.rs 排在本文件之前）。
  ③ **一次真实的试错，值得记下来**：第一版让镜像自己带 `pub type region_cache = c_void;` 并让定义单元
  `region-cache.rs` 也 include 它 → `build-tu src/region-cache.c` 立刻 **E0428**（`include!` 是**文本展开**，
  镜像里的别名与单元自己的 `pub struct region_cache` 同层同名；0.1-10 的"定义遮蔽整体导入"只对
  **glob import** 成立，对文本展开的同层重名不成立）。定案：镜像只用 `*mut c_void`，别名留在
  `search.rs`（它的六处局部声明要写 C 的类型名，P-72③ 允许），定义单元 include 镜像后它的定义遮蔽镜像
  的**声明**（这些在模块里 + glob import，合法）。
  ④ 五个单元改成 include 镜像、删掉各自副本：`bidi.rs`(C 247)、`insdel.rs`(C 30)、`search.rs`(C 32)、
  `indent.rs`(C 33)、`region-cache.rs`(C 27)，净 -31 行；五份台账里"no mirror"的陈述一并改成终态。
  ⑤ 闸门实测：镜像 `--check` **OK**；五个单元 `build-tu` 全部 **built**、`--check` 全部 **OK**；
  `check-globals` 0 缺定义；`check-undef` 512 单元/120 pending/**0 new**；`check-linkage` 0 缺失；
  `layout-check` **passed**（791 条断言，新镜像进了探针 crate）。镜像计数 **130 → 131**（`src/include/` 41 → 42）。

- **3.75 第 98 轮：镜像补齐后的去重，与一份"重复声明"审计（2026-10-04）**：`region-cache.h` 的镜像落地
  （3.74）把"单元自己声明、镜像其实也声明"这种现象暴露出来——它不止 region-cache 一处。
  ① 本轮先修可确证的一处：`src/editfns.rs` 的 `text_property_stickiness` 副本。它的 `intervals.h` 镜像
  （`include/intervals.rs` 的 `intervals_decls`，C 307-308）**已经**带着这条声明，而 editfns.rs 也
  `include!` 了该镜像，所以那份 use-site 副本是纯冗余（glob 导入被本地项遮蔽，一直没报错）；删掉它，
  `build-tu` 仍 `built`、`--check` 仍 `OK`。
  ② 顺手做了一份**全树审计**（不是门，方法写下来给下一轮做棘轮用）：把每个单元 `unsafe extern "C" { … }`
  里"只有声明、没有函数体"的名字抽出来，与 131 个镜像里同样形态的名字求交——**66 个单元**存在"镜像已有、
  单元又声明了一遍"的名字，例如 `tparam.rs` 的 `memcpy`/`strlen`（`string.rs`/`lisp.rs` 镜像有）、
  `process.rs` 的 `sprintf`/`list3`/`xfree`、`lib/readlinkat.rs` 的 `strlen`/`fstatat`。
  **注意这份名单要逐条判**：只有当该镜像是**这个单元的包含图之内**时才是冗余（P-26 的"同一构造一种写法"），
  不在包含图里的声明是合法的 P-72③ use-site 声明。门里已经能看见这一类的**一半**症状：
  `gate-fast` 的 unit 编译会报 `clashing_extern_declarations` 警告（`gate-wt3.txt` 里就有），
  那是"同一个名字两种签名"，与"同一签名重复一遍"是同一根因的两面。
  下一轮的做法建议：新增 `tools/check-mirror-dupes.py`（只报"镜像在该单元 include 图之内"的命中），
  用 `docs/baseline/mirror-dupes.tsv` 做棘轮把 66 条挂上去，然后按单元逐个删副本——
  每条删除都只需 `build-tu <unit>` + `--check <unit>` 两步验证。

- **3.76 第 98 轮：求值器补上"文件自身默认"这一课——`FONT_DEBUG` 一行换来 8 个 subr 可达（2026-10-04）**：
  font lane 报回一个**单点可修**的偏离：`src/font.h` 997-999 是 `#ifndef FONT_DEBUG` / `#define FONT_DEBUG`，
  所以该宏由**源码头文件**决定、`config.h` 说不出它，`check-cond-eval.py src/font.c` 报 `2 disagreements`
  （C 5240/5938 两条 `#ifdef FONT_DEBUG`，clang 判活）。按 §3.35 的机制补一行
  `font.h<TAB>FONT_DEBUG`（用与 `gen-cond-macros.py` 相同的 clang 调用实测）之后：
  `src/font.c: 45 conditions, 0 unknown, 0 disagreements [VERIFIED]`。
  ① **语义回报**：`tools/extract-abi.py` 的 `c_defsubr_live` 是**故意**不走 `blind_conditions` 的，
  表一旦知道 `FONT_DEBUG`，它就要求 C 5938-5949 的**八条 `defsubr`**（`open-font`、`close-font`、
  `query-font`、`font-get-glyphs`、`font-has-char-p`、`font-match-p`、`font-at`、`frame-font-cache`）——
  照抄进 `syms_of_font` 后这 8 个 Lisp 函数**重新可达**（此前只能记台账：抄了会被判 "unexpected defsubr"，
  不抄则 Lisp 里根本没有它们）。font.c 本身仍**不进** `cond-eval-verified.txt`：它照 P-51 两支都抄，
  而 verified 文件的检查器只要求活支（§3.51 的 callproc 注）。
  ② **这一行反而暴露了求值器的模型缺口**：源码头文件行的值是该头文件**自身 define 的净效果**，
  于是 `font.h:997` 的 `#ifndef FONT_DEBUG` 被答成 False，而 clang 答 True（那条 define 在 6 行之后）。
  `tools/cinventory.py` 因此加了**第二遍扫描**：只对"该文件自己拥有 `source-header-macros.tsv` 行"的宏，
  在扫到 `#ifndef X` 时先把 X 置为 undef，让守卫答出 clang 的答案、再由文件自己的 `#define` 供值。
  两个坑都实测过：① 规则一开始过宽（任何 `#ifndef`）→ lisp.h 立刻 45 处分歧，收窄到"本文件拥有的行"才干净；
  ② 第二遍**必须从表的新副本开始**——第一遍已经把文件自己的 define 写进了那张表，沿用会让半个头文件的
  守卫全部提前生效（实测 font.h 从 1 处分歧变 4 处）。终态：`src/font.h` 1→**0**、`src/font.c` 0、
  `--verified docs/baseline/config/cond-eval-verified.txt` → **5717 conditions, 4 unknown, 0 disagreements**
  （432 个文件，exit 0）。
  ③ 同一天 process lane 也踩了同一根柱子的另一面：它加的 `sys/ioctl.h<TAB>TIOCNOTTY` 是**全局**行，
  但该宏的可得性是**逐文件**的——`src/term.c` 含 `<termios.h>` 而不含 `<sys/ioctl.h>`，
  于是 `term.c:4443` 反被判活，`--verified` 直接失败；lane 自己把那行撤了（process.c 照旧是 blind 文件，
  不需要它）。**教训**：源码头文件行只对"真正由该文件定义"的宏成立，别拿它去表达"某个 TU 里看得到"。

- **3.77 第 98 轮：重复声明变成一道门，16 处副本清掉（2026-10-04）**：§3.75 的那份审计（66 个单元里
  "镜像已经声明过、单元又声明一遍"）本轮落成门：`tools/check-mirror-dupes.py`。
  ① **它按"在不在作用域里"判**：从单元出发展开 `include!` 图，镜像侧的声明只有两种算数——
  镜像顶层的（`include!` 是文本展开，直接落在包含它的 crate 根）或镜像 glob 导入的模块里的
  （`use mod::*;`）；单元自己模块内的声明是另一个作用域，不算重复；带函数体的是**定义**不是副本
  （定义遮蔽 glob 导入正是 0.1-10 要的形态）。6 条 `--selftest` 用例把这几条边界钉住（含"同层文本重复"
  与"模块局部不算"），并做过负向验证：往一棵真树里注入一条副本，门立刻失败。
  ② **16 处副本删除**（每处都 `build-tu` + `--check` 双验）：`lib/lchmod.rs` 的 `sprintf`、
  `callproc.rs` 的 `del_range`、`doprnt.rs` 的 `sprintf`、`insdel.rs` 的 `record_first_change`、
  `tparam.rs` 的 `strlen`、`term.rs` 的 `xstrdup`/`xzalloc`、`lread.rs` 的七条、`font.rs` 的两条
  （`composition_gstring_p`/`composition_gstring_adjust_zero_width`）。其中两条**不只是副本**：
  `lread.rs` 把 `emacs_read_quit` 写成 `-> isize`（C 与镜像都是 `ptrdiff_t`，同一类型的两种拼法），
  而 `Fnative_elisp_load` 干脆**没有返回类型**——`globals.rs` 里由生成器写的 `EXFUN` 原型才是对的
  （`Lisp_Object`；调用点本来就丢弃返回值）。
  ③ 棘轮 `docs/baseline/mirror-dupes.tsv` 现在只剩**一行**：`insdel.rs` 的 `bset_redisplay`
  （镜像把参数拼成 `*mut c_void`，调用点传 `*mut buffer`；要合流得让 `window.rs` include `buffer.rs`，
  那是镜像属主的决定）——行里写着理由，不是隐藏。门已接进 `gate-fast`（`== mirror-dupes gate ==`）。

- **3.78 第 98 轮：process.c 收口时，只有对象级与预处理级证据才看得见的四处缺陷（2026-10-04）**：
  这一轮的教训集中在一句话上——**文本门看不见符号级的错**。四条都已修：
  ① **`TIOCNOTTY` 是活文本**（`src/process.c` C 71 含 `<sys/ioctl.h>`，`#ifdef TIOCNOTTY`
  C 2171-2181 因此编译）：此前被当死臂 carry，现在按活文本落地并补上实测常量 `536900721`。
  顺带把"给它加 `source-header-macros.tsv` 行"的做法**否掉**：行是全局的，而该宏的可得性是逐文件的
  ——`src/term.c`（在 verified 名单里）从不 include `<sys/ioctl.h>`，加了行它的 `#ifdef TIOCNOTTY`
  立刻与 clang 相反、`--verified` 失败（与 §3.76③ 同一条教训的另一面）。
  ② **`inherit_process_coding_system` 的属主是 `src/coding.c`**：port 在 process.rs 里定义它，
  `check-symbols.py --dir src`（该单元已有参照对象）判它 **invented**（C 的存储是 coding.c 的
  `DEFVAR_BOOL`，落在单一 `globals` 对象里）。删掉后 `67 units compared, 0 mismatched, 0 pending`；
  代价是它变成一条 pending 未定义引用（`callproc.o` 一直都在引用它，只是被这份"错主的定义"掩盖）——
  已由 §3.76 的棘轮重取吸收（`126 pending, 0 new`）。
  ③ **三条拼写缺陷**（集成者修）：`bind_`/`connect_`/`insert_1_both_` 各多一个下划线。C 调的是
  `bind`/`connect`（`<sys/socket.h>`）与 `insert_1_both`（lisp.h 4331，insdel.c 定义），**参照
  `process.o` 里根本没有那三个符号**，所以端口引用了三个永远无人提供的名字；改名（含调用点）后
  `check-undef` 是 `516 unit object(s), 124 pending, 0 new`。`--check` 也跟着更准（C 的 token 才对得上）。
  ④ **一个镜像归属错误**：`add_read_fd`/`delete_read_fd` 是 **process.h 298-305** 的，不是 keyboard.h
  （§3.5 与 `keyboard.rs` 的表头都写成了 keyboard.h）。本轮把该处的**五条** fd 入口
  （`add_read_fd`/`add_non_keyboard_read_fd`/`delete_read_fd`/`add_write_fd`/`delete_write_fd`）移进
  `include/process.rs` 的 `process_fd_decls`（模块 + glob，P-21 形状），`keyboard.rs` 只留 keyboard.h 498
  的 `kbd_buffer_store_event`；五个消费者重建通过。
  ⑤ **两条工具/约定定案**：
  - `tools/cinventory.py` 的 `dead_branch_map` 返回的是**逐物理行**的布尔表，而 `logical_lines` 是
    **逐逻辑行**的；把两者 `zip` 起来做"活文本视图"会**静默错位**（xfaces lane 的视图尾部错了 13 行，
    它在自查时发现）。任何新的"活区判定"脚本都要盯这一点。
  - `docs/ports/line-drift-tolerance.tsv` 是**追加式日志**，同一 stem 可以有多行：工具原来取**第一条**
    匹配（旧行永远生效，与 lane 们"新行覆盖旧行"的预期相反），本轮改成**取最后一条**；用 `buffer`
    （0.20 临时行 → 0.16 终态行）与 `process`（0.10 临时行 → 0.05 终态行）两处验证 `--check` 仍 OK。

- **3.79 第 98 轮：`<stdio.h>` 的九个名字收敛进镜像——19 个单元各自抄了一两年（2026-10-04）**：
  P-26 的"同一构造一种写法"在 stdio 上一直是灰的：`fprintf` 被 **13 个**单元各自声明、`putc` 9 个、
  `fflush` 6 个、`fileno` 4 个、`clearerr` 3 个、`getc` 4 个，`stdout`/`stdin`/`stderr` 三个流对象
  被 **24 个**单元各自声明（每处都带 Darwin 的 `__stderrp`/`__stdinp`/`__stdoutp` link_name）。
  ① 镜像补齐：六个函数进 `include/stdio.rs` 的 `stdio_decls`（模块 + glob，0.1-10 的遮蔽机制让
  任何单元保留自己的副本也不会 `E0428`），三个流对象进新的 `stdio_objs` 模块（同样带 link_name）。
  ② 19 个单元（`getdelim`/`getopt`/`mini-gmp`/`bidi`/`buffer`/`bytecode`/`charset`/`cm`/`emacs`/
  `frame`/`lread`/`minibuf`/`pdumper`/`print`/`regex-emacs`/`region-cache`/`search`/`sysdep`/`term`）
  删掉各自的副本，全部 `build-tu` + `--check` **OK**；`enable_checking` 抽查 emacs/bidi/term 三个
  单元 exit 0；`check-globals` 269/0、`check-linkage` 0 缺失、`check-declarations` 0、
  `check-mirror-dupes` 仍只有棘轮那一行。在飞的 fileio/alloc/dispnew/fns 四条 lane 的副本会在它们
  收口时被同一个门点出来（我给它们的清单里就有这一步）。
  ③ **顺手抓到一处真缺陷**：`src/lread.rs` 把 `clearerr` 声明成 `-> c_int`，而 C 与镜像都是 `void`
  ——这正是"同一个名字两种签名"的 `clashing_extern_declarations` 类；它的副本随本轮删除，声明归镜像。
  ④ 过程中的一个教训（写下来备查）：用"注释后紧跟的下一行像不像条目"来清理悬挂 `///` 时，
  `macro_rules!` 一类不以关键字开头的条目会被误判——第一版脚本就这样删掉了 16 条**合法**文档注释，
  随后按 `HEAD` 复原、只重跑定向移除才干净。**清理脚本的判据要保守，且必须逐文件编译验证。**

- **3.80 第 98 轮：errno 码同样收进镜像（2026-10-04）**：与 3.79 同一条纪律的另一半——`EAGAIN`/
  `EWOULDBLOCK`/`EMFILE`/`EINPROGRESS`/`EISCONN`/`EMSGSIZE`/`EPIPE`/`EDOM`/`ECHILD` 九个码分别被
  `src/process.c`（7 个）、`src/sysdep.c`（5 个）、`src/dired.c`（1 个）、`lib/dup2.c`（1 个）
  就地声明。值用 `clang -E -dM -include errno.h` 实测（`EWOULDBLOCK` 在 C 里就是 `#define EWOULDBLOCK
  EAGAIN`，端口两个名字同值）；四个单元的副本删除，各自 `build-tu` + `--check` OK，
  `check-declarations`/`check-mirror-dupes` 不变。**注意 `errno.rs` 的常量是顶层 `pub const`**
  （不像 `<stdio.h>` 的函数那样有模块 + glob 兜底）：所以"往镜像加码"与"删掉单元的副本"**必须在同一个
  提交里**，否则先加的那一步就是 `E0428`。`EXDEV`（18）随 fileio 单元一起搬。

- **3.81 第 98 轮：`USE_LSB_TAG` 进源码头文件行表——alloc.c 的 4 处分歧归零（2026-10-04）**：
  alloc lane 报回 `check-cond-eval.py src/alloc.c` = `138 conditions, 4 disagreements`，四条都是
  `#if ! USE_LSB_TAG`（求值器答 True、clang 答 False）。`USE_LSB_TAG` **不在** `config.h` 里：
  `lisp.h` 250-257 自己算（`#if (ALIGNOF_EMACS_INT < IDEAL_GCALIGNMENT && !HAVE_C_ALIGNASOF &&
  !defined alignas && !defined HAVE_STRUCT_ATTRIBUTE_ALIGNED)` → `false`，`#else` →
  `(VAL_MAX / 2 < INTPTR_MAX)`），正是 §3.35/§3.76 那一类"由源码头文件决定、config.h 说不出"的宏。
  按同一机制补一行 `lisp.h<TAB>USE_LSB_TAG` 并用与 `gen-cond-macros.py` 相同的 clang 调用实测
  （探针**必须**先 `#include <config.h>` 再 `#include "lisp.h"`——我第一次只包含 lisp.h，量到的是
  `false`，而以 config.h 打头时活支是 `#else` 的表达式）。结果：`src/alloc.c: 138 conditions,
  0 unknown, 0 disagreements [VERIFIED]`，整表 `--verified` 仍 **5717 conditions, 4 unknown,
  0 disagreements**（432 个文件，exit 0）。
  **alloc.c 依旧不进 `cond-eval-verified.txt`**：它照 P-51 两支都抄，而 verified 文件的检查器只要求
  活支（§3.49/§3.51 的既有定案）；这一行改变的是**求值器的答案**，不是端口的写法。

- **3.82 第 98 轮：两个"由别的单元定义"的原型从 crate 根搬进模块——alloc.rs 的两个包装模块消失（2026-10-04）**：
  alloc lane 报回它的两个定义（`make_interval`、`build_overlay`）**被迫**放进 `alloc_defs`/
  `alloc_defs_overlay` 模块再 glob 导入，原因是 `include/intervals.rs` 与 `include/buffer.rs` 把这两条
  原型（连同同一块里的 `recenter_overlay_lists`/`fix_overlays_before`/`mmap_set_vars`）放在**crate 根**，
  而同名定义在 crate 根就是 `E0428`。这违反了镜像的既有形状：凡是"由别的单元定义"的原型都该在
  `*_decls` 模块 + glob 里（0.1-10/P-21 的遮蔽机制），`intervals.rs` 自己的注释就是这么写的，只是这一条
  漏在外面。本轮把 `make_interval` 挪进 `intervals_decls`，把 buffer.h 那一块的四条挪进新的
  `buffer_other_decls`，删掉 alloc.rs 的两个包装模块（定义回到 crate 根），并**顺手纠正一处镜像注释的
  属主错误**：`build_overlay` 是 `alloc.c` 3768 定义的，旧注释写成 insdel.c。
  ① 重建与验证：alloc 的 `build-tu`/`--check` OK，六个镜像消费者（buffer/insdel/search/textprop/editfns/
  callproc）`build-tu` 全 built、`--check` 全 OK；`check-symbols --dir src` 仍 **68/0/0**、
  `check-globals` 270/0、`check-declarations` 0、`check-mirror-dupes` 仍只有棘轮那一行。
  ② **一处两门相反的定案**：`refill_memory_reserve` 在 C 里是 `static`（C 392 的前置声明），对象级的
  `check-symbols` 因此判端口的 `#[unsafe(no_mangle)] pub` 是 **invented**；而文本门看不见那条前置声明
  （blind 文件下 `static_prior` 规则被关掉，§3.67），反过来要求必须有 `#[unsafe(no_mangle)]`。按 lread 的
  `oblookup_considering_shorthand` 先例，把名字记进 `docs/ports/excluded-macros.txt` 的 `linkage` 类并写明
  两边的实测证据，文本门据此让开——**"两门相反时以对象级为准 + 记豁免"** 是这类构造的定式。

- **3.83 第 98 轮：`check-mirror-dupes` 补上常量类——以及"两门相反以对象级为准"的第二例（2026-10-04）**：
  §3.80 的 errno 收敛漏了 `EDOM`（`filelock.rs`/`timefns.rs` 也各自声明了它），是**整树门**在
  `src/filelock.rs` 上以 `error[E0428]: the name \`EDOM\` is defined multiple times` 抓出来的——
  原因是做扫描时用的名字表是**第一次测量**的那张（`EAGAIN`…`EIO`），不是**要加入**的那张（含 `EDOM`/
  `ECHILD`）。两处副本已删（`11be128`）；**教训**：往镜像加名字的扫描必须用同一张名字表，而**常量这一类
  当时没有任何门**（`check-mirror-dupes.py` 只扫 `fn` 声明），所以只能靠整树编译兜住。
  ① 本轮把工具补成两类都扫，并且**两类的判据不同**（实测定案）：镜像里 glob 导入模块中的常量被单元自己的
  常量遮蔽是**合法**且是既定的载体形态（`lib/acl-internal.rs` 定义 `ACL_INTERNAL_INLINE`、
  `lib/set-permissions.rs` 自定义 `HAVE_FCHMOD`，两者都照常编译），只有**顶层**常量才会与单元的同名常量
  在同一个作用域里撞成 `E0428`（`errno.rs` 的码就是顶层，这正是 `EDOM` 咬人的原因）；另外 `#[cfg (…)]`/
  `#[cfg_attr (…)]` 门控的条目属于**另一个配置**的携带臂（`acl-internal.rs` 的
  `#[cfg (feature = "no_fchmod")] pub const HAVE_FCHMOD` 就是），不参与判定；`const _: () = assert!(…)`
  这种布局断言按匿名处理，不算同名。9 条 `--selftest` 用例把这三条边界钉住（含 3 条常量专例）。
  实测：已提交树上 **只剩棘轮里那一行**（`insdel`/`bset_redisplay`），也就是说新覆盖的常量类当前是干净的，
  而它**本来就能抓住 `EDOM`**。
  ② 同一天第二处"两门相反"：`refill_memory_reserve`（§3.82）——对象级把它判成 invented、文本门反过来
  要求 `#[unsafe(no_mangle)]`，定式是**以对象级为准 + 在 `docs/ports/excluded-macros.txt` 记 `linkage`
  免责行**。

- **3.84 第 98 轮：两处镜像宏缺陷修好，dispnew 的两个影子宏删除（2026-10-04）**：dispnew lane 在用点
  用"照 C 原文 + 一处显式转换"的影子宏盖住了两处镜像缺陷（Rust 的 `macro_rules!` 是文本作用域，后定义的
  从该点起生效），并在台账里点名让集成者在修镜像的**同一个提交**里删掉它们。本轮照做：
  ① `dispextern.rs` 的 `SET_CHAR_GLYPH`（C `(GLYPH).u.ch = (CODE);` 是把 `int` 赋给 `unsigned` 成员，
  即隐式取模转换；镜像原来直写 `($code)`，于是 `SET_CHAR_GLYPH_FROM_GLYPH` 编不过 E0308）改成
  `($code) as c_uint` 并写明理由；`MATRIX_ROW_VPOS`（C `((ROW) - (MATRIX)->rows)` 是**指针相减**，镜像
  写成 `(*($row)).offset_from (…)`，先解引用再相减，是另一个表达式）改成 `($row).offset_from (…)`。
  ② `dispnew.rs` 顶部的两个影子宏连同注释一起删除，台账里那段"是影子宏盖住"的叙述与 integrator to-do
  一并改成终态（Done in status 3.84）。
  ③ 验证：dispnew 重建 `built`、`--check` OK；另外七个 `dispextern.rs` 消费者（frame/xfaces/font/term/
  bidi/indent/composite）全部重建通过；frame/xfaces/term/font 的 `--check` 也都 OK；
  `check-symbols --dir src` **70/0/0**、`check-declarations` 0、`check-mirror-dupes` 仍只有棘轮一行。
  **注意**：`check-symbols` 在 lane 正在重建对象的瞬间可能读到半新半旧的一轮（本轮出现过一次
  "1 mismatched"、重跑即 0）——判读这类门要避开在飞 lane 的构建窗口。

- **3.85 第 98 轮：三个系统头镜像落地——`passwd`/`group`/`utsname` 各从 3–4 份副本收成一份（2026-10-04）**：
  `<pwd.h>`/`<grp.h>`/`<sys/utsname.h>` 一直没有镜像，于是 `dired.rs`/`editfns.rs`/`fileio.rs`/`sysdep.rs`
  各自把 `struct passwd`（clang 实测 72/`pw_name`@0/`pw_uid`@16/`pw_dir`@48/`pw_shell`@56/`pw_expire`@64）、
  和其中三个单元各自把 `struct group`（32）与 `struct utsname`（1280）连 `const _` 断言一起抄了一遍。
  ① 新增 `lib/include/pwd.rs`（`passwd` + `getpwuid`/`getpwnam`/`getpwent`）、`grp.rs`（`group` + `getgrgid`）、
  `sys_utsname.rs`（`utsname` + `uname`），布局与断言随定义一起搬，四个单元的副本删除、按 C 的顺序
  `include!`（`time_t`/`uid_t`/`gid_t` 由包含单元提供，所以插在 `systime.rs` 之后）。
  ② 验证：四个单元全部 `build-tu` built、`--check` OK；`check-globals` 272/0、`check-declarations` 0、
  `check-mirror-dupes` 仍只有棘轮一行、`check-linkage` 0 缺失。
  ③ 过程教训（写下来备查）：**按"注释往回吞"的通用清理脚本很危险**——第一版把类型上面的 `/* … */`
  块的收尾 `*/` 一起吞掉，`editfns.rs` 立刻报 `unterminated block comment`（`/*` 与 `*/` 计数 289/288 是
  最快的自检）；改成"按内容定位的整段删除"就干净了。另外 `sysdep.rs` 用的是**单行** extern 块
  （`unsafe extern "C" { pub fn uname (…) -> c_int; }`），行首锚定的匹配器抓不到它，靠编译期的 `E0428`
  才发现，已单独删掉。

- **3.86 第 98 轮：`langinfo.rs` 的 `CODESET` 是错的（14 是 glibc 的值）——fns lane 用冻结预处理器抓到（2026-10-04）**：
  fns lane 在写 `Flocale_info` 时把镜像与冻结目标对拍：`clang -E -dM -include langinfo.h` 给出
  **`CODESET 0`**（`DAY_1 7` … `DAY_7 13`、`MON_1 21` … `MON_12 32`），而 `lib/include/langinfo.rs` 写的是
  **14**（镜像注释自己就写着 "glibc's `_NL_CTYPE_CODESET_NAME` = 14"——值是抄 glibc 的，不是这台机器的）。
  这不是"两条写法"而是**数值错误**：`Flocale_info` 的 `codeset` 支会拿错 item 去问 `nl_langinfo`。
  ① 修正 `CODESET` 为实测的 **0**，并把 `DAY_*`/`MON_*` 十九条按实测值补进镜像（19 条 + `CODESET` 都在同一份
  `const` 顶层作用域，所以**改镜像与删单元副本必须同一个提交**，§3.80 的教训）。
  ② `fns.rs` 顶部那 19 条就地声明删除，改用镜像（唯一消费者就是它；`build-tu` + `--check` 都过）。
  **教训**：系统头镜像里的每个数值都该有"这台机器实测"的出处；注释里出现别的 libc 的宏名就是警报。

- **3.87 第 98 轮：`lisp.rs` 的三条 crate 根声明进模块——fns 的 `fns_root_shadowed` 消失（2026-10-04）**：
  fns lane 报回：`lisp.rs` 把 `hash_find`/`hash_put`/`string_make_unibyte` 三条（lisp.h 4254-4279，各写成一个
  单行 `unsafe extern "C" { … }`）放在**crate 根**，而 `src/fns.c` 正是这三条的定义方——同作用域定义就是
  `E0428`，于是它把三个定义塞进 `pub mod fns_root_shadowed`（P-21 从另一侧的用法），代价是单元内部的调用
  变成走**外部符号**再回到自己的定义（能链上，但不是 C 的"直接调本地定义"）。
  ① 修镜像：三条声明进 `pub mod lisp_hash_decls` + glob 导入（P-21 的遮蔽形状，与 `intervals_decls`/
  `buffer_point_*` 同类），镜像注释写明"这是 fns.c 的定义方，声明必须可被遮蔽"。
  ② `fns.rs` 的包装模块删除，三个定义回到 crate 根（调用恢复成直接调本地定义）。
  ③ 验证：fns 重建 `built`、`--check` OK；另外四个 `lisp.rs` 重消费者（eval/alloc/dispnew/print）全部重建通过；
  `check-globals` 272/0、`check-mirror-dupes` 仍只有棘轮一行（这三条是**定义**不是副本，工具按定义/声明区分，
  所以不会误报）。

- **3.88 第 98 轮：七个 lib 镜像的 `decls` 模块改名——以及探针揭示的下一类冲突（2026-10-04）**：
  fns lane 报回：`lib/include/{filevercmp,md5,sha1,sha256,sha512,sha3,utimens}.rs` 都把 extern 块放进一个
  **叫 `decls` 的模块**，而 `xtask` 的 materializer 会把这一层 `mod` 拍平，于是**同一单元 include 两个**
  就是 `E0428`（它需要其中六个）。本轮按文件名改名：`<stem>_decls`（`filevercmp_decls`、`md5_decls`、
  `sha1_decls`、`sha256_decls`、`sha3_decls`、`sha512_decls`、`utimens_decls`），七处 `use decls::*;`
  同步改；十个消费者（`lib/filevercmp`/`md5`/`md5-stream`/`sha1`/`sha256`/`sha3`/`sha512`/`utimens`/
  `utimensat` + `src/fns`）全部重建通过。
  ① **负向探针**（在 `fns.rs` 里临时多加 `md5.rs`/`sha256.rs` 两个 include 再 `build-tu`，随后立刻按备份
  还原、确认文件回到原状）：`E0428` **不再**来自模块名，而是
  `the name MD5_DIGEST_SIZE is defined multiple times` / `SHA224_DIGEST_SIZE` / `SHA256_DIGEST_SIZE`
  ——即**镜像顶层的常量**与 `fns.rs` 自己的同名副本撞车（它当初因为这些镜像不能共存而在使用点声明了
  十个 `*_DIGEST_SIZE`）。这正是 §3.83 给 mirror-dupes 门加常量类的同一根因，只是**镜像不在该单元的
  include 图里**，所以门还看不见；要收掉它得"给 fns.rs 加这些 include + 同时删掉它自己的十个副本"，
  记在这里作为下一步（同类还牵扯若干 `*_buffer` 函数声明）。

- **3.89 第 98 轮：fns 的六个摘要镜像进 include 列表，十四个用点声明收进镜像（2026-10-04）**：
  接 §3.88：模块改名之后，`fns.rs` 把 `md5.rs`/`sha256.rs`/`sha512.rs`/`sha3.rs`/`filevercmp.rs`
  连同原有的 `sha1.rs` 一起 include，删掉它自带的 `filenvercmp`、`md5_buffer`、`sha224/256/384/512_buffer`、
  `sha3_{224,256,384,512}_buffer` 十四个用点声明与九个 `*_DIGEST_SIZE` 常量（`SHA1_DIGEST_SIZE` 本来就在
  镜像里、它没抄）。验证：`build-tu` built、`--check` OK、`check-globals` 272/0，而且**新覆盖的
  `check-mirror-dupes` 报 0 duplicates**——工具从加常量类那天起就看得见这些常量，只是镜像原先不在该单元的
  include 图里；这次收敛等于把 §3.83 的判据用在真身上。台账里"镜像不能共存、只能 include sha1"的叙述与
  `CODESET` 的旧值都改成终态。

- **3.90 第 98 轮：代码里不再复述"参照的许可/版本声明"（2026-10-04 用户指令）**：
  用户指令：版本声明不写产品名之后，"把之前的代码中的这种声明也去掉"。3.36/3.58 已经把**声明本体**处理干净
  （版本行只留修订号、268 个复制的声明块不抄，`check-declarations.py` 现在 0 forbidden），但**我们自己的注释在
  复述那件事**：`buffer.rs` 的 "the reference's own license block is deleted at its head"、`xfaces.rs`/`dispnew.rs`
  的 "C's version/copyright declaration … is not carried"、`emacs.rs` 的 `RCS_Id` 注释里 "keeps the product name out
  of every comment"、5 个 gnulib 包装单元的 "its C text is the license"、`lib/include/nproc.rs` 的 "starts with the
  license comment"、`lib/include/sig2str.rs` 的 "The reference header is the license"。复述本身仍是一句关于**本仓库**
  的许可声明，与 0.1-19 要躲的正是同一类风险，所以一律删除或改成中性表述：
  ① 删：`buffer.rs` 两行 + 一个空注释行、`dispnew.rs` 末句、`xfaces.rs` 尾括注；
  ② 改：5 个 `lib/` 包装 → "its C text carries no code — only …"、`nproc.rs` → "a comment block"、`sig2str.rs` →
  "holds the include guard and one constant"、`emacs.rs` → "leaves the name `PACKAGE_NAME` spells out as a placeholder"。
  验证：11 个文件 `build-tu` 全部 `built`（src 4 个 + lib 5 个 + `layout-check` 探针覆盖 2 个镜像，探针
  `layout probe ok`）；`extract-abi.py --check` 9 个单元全 `OK`；`check-declarations.py` 仍
  `8 declaration sites (0 forbidden, 8 allowed literals), 0 failures`；`check-mirror-dupes.py` 仍是既有那 1 行棘轮、
  0 failure；`check-verbatim.py` `6 trees, 4607 files, 0 failures`。规则补进 `PORTING.md` 0.1-19 末尾。
  **未动**：`docs/ports/line-drift-tolerance.tsv` 里 10 行用中性措辞说明同一件事（台账口径/漂移理由，不是代码），
  以及 `docs/status.md`/`PORTING.md` 自身的政策叙述——这些是决定的记录，删掉就没法复算。

- **3.91 镜像的三条书写纪律（2026-10-04 第 98 轮第三批，`src/include/sysstdio.rs` 暴露）**：
  写 `src/sysstdio.h`（47 行，27 个单元 include 它）的镜像时，`xtask layout-check` 连拦三次，
  三条都由工具实测钉下，写在这里供后续镜像照做：
  ① 镜像里的 **`*_decls` 模块必须自带 `use super::*;`**：`c_int`/`c_void`/`ptrdiff_t` 是**包含它的
  单元**给的（lisp.rs 的 crate 根 `use std::ffi::{…}` 与 `pub type ptrdiff_t`），模块内的名字不自动
  可见——首版漏了这行，探针报 `E0425: cannot find type c_int`（`atimer_decls` 是现成样板）。
  ② 字符串字面量宏在镜像里是 **`const`**（提取器按常量比），而且写成 **`&core::ffi::CStr`** 全限定
  形——镜像不能自己 `use std::ffi::CStr`，那会和单元的 prelude 撞 `E0252`（`character.rs`/`config.rs`/
  `buildobj.rs` 先例）；需要 `concat!` 里当字面量的用途（`src/xfaces.rs` 572）保留就地 `macro_rules!`，
  作为用点偏离记录。
  ③ 探针里 **lisp.rs 的“排第一”是死代码**：lisp.rs 被别的镜像嵌套 include，去重后它不在顶层列表里，
  于是那段 `mirrors.insert(0, lisp)` 找不到它。**本轮实测它没有造成失败**（顶层第一个 reach lisp.rs 的
  镜像恰好排在前面），所以**没有改 xtask**——`/tmp` 试验里把那个 reach 者提前也不改变结果；这条只作
  观察记录，等真出现顺序失败再修工具（不许为假想问题改闸门）。

- **3.92 C 的 UB 域里，Rust 用 panic 代替（2026-10-04 第 98 轮第三批，`sysselect.h` 的差分探针发现）**：
  `src/include/sysselect.rs` 与 `lib/include/sys_select.rs` 的三个新访问器（`FD_SET`/`FD_CLR`/`FD_ZERO`）
  用一条确定性序列做了 C↔Rust 差分：两边 dump 与最终 128 字节 `fd_set` 完全相同
  （`sha256 0c832346…`）。**同一支探针顺手测出**：Darwin 的
  `__darwin_check_fd_set_overflow`（`sys/_types/_fd_def.h` 58-79）**不按 `FD_SETSIZE` 设界**——
  实测 `check(1023)=1 check(1024)=1 check(1100)=1`，于是 C 的 `FD_ISSET (1024, &f)` 会读
  `fds_bits[32]`（32 元素数组的下一格，**未定义行为**，栈上恰好读到 0）；Rust 镜像的切片带边界检查，
  同样输入**panic**。定案：这是语言层面的必然偏离，**记录不掩盖**——树内调用点全在范围内
  （`sysselect.h` 的三个包装函数用 `eassume` 写明契约），真跑测试套件时若出现越界 fd 会以 panic 暴露，
  而不是静默读脏数据。判据与探针配方写在 `docs/ports/include-sysselect.md` 与
  `docs/ports/include-sys_select.md`。

- **3.93 镜像落地后的收敛：本地测量副本必须退位（2026-10-04 第 98 轮第四批）**：
  本批新增的 `src/include/{systty,sysstdio,sysselect,syswait}.rs` 与系统镜像 `lib/include/termios.rs`
  落地后，**同一批**把两个单元的本地副本收掉——`femacs/src/sysdep.rs` 删掉 `<termios.h>` 四 typedef +
  `NCCS` + `struct termios`（8 条 `const _` 断言）、本地 `struct emacs_tty` 与本地 `CDISABLE`；
  `femacs/src/minibuf.rs` 删掉它的**不可逐字段读**的 `struct termios { _storage: [u8; 72] }` 假体、
  `emacs_tty` 与三条用点声明。规则：0.1-3 的"每个类型一个定义点"——镜像一落地，本地"测量副本"就是重复定义，
  必须在同一批退位（`check-mirror-dupes.py` 现在 0 failure，说明这一批只减不增）。
  **两条实现上的定式**：① 系统常量的类型照 C 的类型走（`limits.in.rs` 的 `CHAR_BIT: c_int` 先例），
  于是镜像的 `NCCS: c_int` 让 `memcmp (…, NCCS)` 需要 C 的隐式转换显式化 → 写 `NCCS as usize`
  （P-15 家族的"显式 `as`"偏离）；② 镜像里的数组长度也带上 C 的 token（`c_cc: [cc_t; NCCS as usize]`，
  不要写死 20）。
  证据：两个单元 `build-tu` = built、`extract-abi --check` = OK、`rustc --cfg enable_checking` = exit 0、
  `check-globals` 273 单元 / 0 缺定义、`check-declarations` 0。剩余一份在 `femacs/src/process.rs` 7390-7430
  （那条 lane 在飞），收敛方案与理由写在 `docs/ports/include-termios.md`。

- **3.94 `xtask` 认得全部单元目录（2026-10-04 第 98 轮第四批）**：`build-tu`、整树 walks 与 ledger→单元
  的映射此前把目录硬编码成 `["src", "lib"]`，于是**批次 F 的五个目录（`lib-src/`、`lwlib/`、`oldXMenu/`、
  `exec/`、`nt/`、`msdos/`）连自己的文件都找不到**——`build-tu lib-src/etags.c` 会去 `src/etags.rs`
  找。现在五处共用一条 `UNIT_DIRS`（八目录，镜像目录按 `<dir>/include/<stem>.rs` 推），`dead-arms`
  对尚未打开的目录直接跳过。**今天行为中性**：那五个目录里还没有 `.rs`，所以门的读数不动；实测
  `build-tu src/emacs.c` 仍 `built`，`build-tu lib-src/make-fingerprint.c` 用下一条提交的新单元
  走通了这条路（批次 F 的第一个目录由此打开）。

- **3.95 批次 F 开张：第一个 `lib-src/` 单元（2026-10-04 第 98 轮第四批）**：`3.94` 让 `xtask` 认全八个目录之后，
  **`lib-src/make-fingerprint.c`（178 行，`main` + 一个 `static char *buf`）** 是第一个落地的非 `src/`/`lib/` 单元：
  `build-tu lib-src/make-fingerprint.c` → `built lib-src/make-fingerprint.rs -> ../build/femacs/make-fingerprint.o`，
  `extract-abi --check` = OK，`--cfg enable_checking` = exit 0。它是**活文件**（冻结构建 `make -n` 会编它，
  参照目录里有二进制），台账 `docs/ports/make-fingerprint.md` 记了 4 条偏离：C 51-61 的 WINDOWSNT 臂与
  C 45-47 的 `#ifndef SSIZE_MAX` 是死文本；`"r" FOPEN_BINARY`（拼接不能点名常量）与 `TYPE_MAXIMUM (off_t)`
  两条 token 走 `macro-body-notes.tsv`（**行必须按函数名 `main` 建键**，按文件名建键工具不认——本轮踩过）；
  `SEEK_SET`/`fseeko`/`ptrdiff_t` 是 P-72③ 的用点声明。行数门加 `make-fingerprint 0.45`（实测 178 vs 251）。
  **顺带把三处镜像补全并收敛**（同一批，因为它们与新单元互相触发）：`stdio.rs` 补 `printf`/`perror`、
  `stdlib.rs` 补 `EXIT_SUCCESS`/`EXIT_FAILURE` → `src/emacs.rs`、`src/sysdep.rs`、`lib/group-member.rs` 删掉各自的副本
  （`check-mirror-dupes` 从 7 failures 回到 0）；`string.rs` 补 `memmem`；`lib/include/min-max.rs` 新建。
  **教训**：往共享镜像里加常量前先查它是否已属于另一个镜像或已被多个单元本地定义——`fseeko`/`SEEK_*` 就是
  这样被回退成用点声明的（`SEEK_*` 归 `unistd.in.rs`，`fseeko` 放镜像会让不含 `off_t` 的消费者编不过）。

- **3.96 批次 F 的"未编译文件"基线（2026-10-04 第 98 轮第五批）**：`docs/baseline/frozen-build-sources.txt`
  过去只有 `src/`（由 `make -n` 生成），于是 `lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、`nt/`、`msdos/`
  的文件**永远得不到 3.31 的"未编译 → 空单元 + 说明"处置**。本轮：①给基线补一段批次 F 派生段落——
  `lib-src/` 按参照 `lib-src/Makefile` 的 `INSTALLABLES`/`UTILITIES`/`DONT_INSTALL` 三个变量判定
  （编 8 个：ebrowse/emacsclient/etags/hexl/make-docfile/make-fingerprint/movemail/pop），`exec/` 无任何规则
  （参照目录里根本没有 `exec/`，HAVE_PDUMPER=1），`lwlib`/`oldXMenu` 是 X11 部件库、`nt`/`msdos` 是平台移植；
  ②`tools/extract-abi.py` 里两处 `src/` 硬编码（`not_built_on_target` 的成员判定、清清单时的单元路径查找）
  换成与 `xtask` 同一份 `UNIT_DIRS`——**这是让 3.31 对非 `src/` 目录真正生效的唯一开关**（改前实测：
  `lib-src/update-game-score.c` 的输出仍是 24 条待办，改后才变成"Nothing in this file is required here"）。
  ③按新基线落 **4 个 lib-src 空单元**：`update-game-score.c`(506)、`seccomp-filter.c`(409)、`ntlib.c`(601)、
  `asset-directory-tool.c`(338)，每个 `--check` = OK、`build-tu` = built，台账用 `extract-abi` 生成并写明
  "deferred declarations"（`docs/ports/{update-game-score,seccomp-filter,ntlib,asset-directory-tool}.md`）。
  **下一步可复用**：`lwlib/`(15)、`oldXMenu/`(33)、`exec/`(8)、`nt/`(31)、`msdos/`(2) 共 89 个 `.c` 现在都满足 3.31，
  可以按同样方式一次性落空单元（域内 `lib-src` 只剩 `.h` 两个需要真镜像）。

- **3.97 `src/window.c` 收口 + 进 verified 清单（2026-10-04 第 98 轮第六批）**：window lane 交回**完整**单元
  （C 1–9740 全覆盖、`femacs/src/window.rs` 9,588 行、台账 251/251、行数漂移 −1.6%、`build-tu` built、
  `rustc --cfg enable_checking` exit 0、`check-symbols` 里 window 既不 missing 也不 invented），提交 `d08a521`。
  它同时纠正了文件头的一个假前提：**`src/window.c` 不在 `cond-eval-verified.txt` 里**（只有 `window.h`），
  尽管 `tools/check-cond-eval.py src/window.c` 实测 `17 conditions, 0 unknown, 0 disagreements [VERIFIED]`——
  于是整树 25 条 token 差异里 22 条的根因是**基线漏了一行**，不是端口错。集成者本轮补上那一行（全表复检
  `5,734 conditions, 4 unknown, 0 disagreements`），并**按实测裁剪 notes**：删掉两行（`replace_window…`
  的 22 token 行、`set_window_scroll_bars` 行）、把 `Frecenter` 行缩到只剩 `uninit`，`--check` 仍 OK。
  **教训**：死文本差异要先问"提取器凭什么要这些 token"，再决定是补文本还是补基线——两者的成本差两个数量级。

- **整树门（`fcbf0bc`，`EXIT=0`，2026-10-04 第 98 轮第六批）**：`checked 381 ported files: 358 done, 23 in progress,
  0 failing`、`inventory: no differences`、`cargo check: ok`、
  符号门 lib `78 units compared, 0 mismatched, 1 still pending` 与 src `69 units compared, 0 mismatched, 1 still pending`、
  未定义引用门 `538 unit object(s) … 111 pending, 0 new`（另 `1 ratchet entry stale (now satisfied): _record_auto_save`）、
  连接门 `0 with 0 missing symbol(s)`、declaration gate `8 sites (0 forbidden, 8 allowed literals), 0 failures`、
  `layout-check passed`。这一轮它是**带完整日志**跑的（`/tmp/gate-fcbf0bc.txt`，82,382 行），所以读数可复算。
  它之后落地的有：window 收口、`src/window.c` 进 verified 清单、44 个未编译空单元、第一个 lib-src 单元——
  这些都已由各自的逐文件门与 `--check` 覆盖；**下一次整树门会对新的 HEAD 重跑**。

- **3.98 第二个真 `lib-src` 单元 + stdio 镜像补全（2026-10-04 第 98 轮第七批）**：
  **`lib-src/hexl.c`（221 行）** 落地——`build-tu lib-src/hexl.c` = `built`、`--check` = OK、`--cfg enable_checking`
  = exit 0、台账 4/4；台账记了 6 条偏离（`PRIxMAX` 实测 `"jx"` 走 notes 行、`uintmax_t` 由单元提供、
  `<unlocked-io.h>` 无镜像按用点处理、`exit` 用点声明、`i` 零初始化与 `c + 0u` 的显式无符号提升、
  `<binary-io.h>` 带 `O_BINARY`）。为它补全 `stdio.rs`（`getchar`/`putchar`/`puts`）并让 `src/minibuf.rs`、
  `src/dispnew.rs` 退位；行数门加 `hexl 0.55`（实测 C 221 vs 331+1）。**剩一个 `check-mirror-dupes` 失败
  属在飞的 `keyboard.rs`**（`getchar`），已通知该 lane 收敛。

- **3.99 第三个真 `lib-src` 单元（movemail）+ 两处集成裁决（2026-10-04 第 98 轮第八批）**：
  **`lib-src/movemail.c`** 落地——只移植**活文本**（`check-cond-eval` 实测 `37 conditions, 0 disagreements
  [VERIFIED]`，所以 POP 半区、lock-file 机制、maillock、platform 分支全部按 P-08 记账），
  `build-tu` = `built`、`--check` = OK（8/8 + 2 条 notes 行）、`--cfg enable_checking` = exit 0、
  行数门用 **`live` 基准**（C 871 行里约三分之一是死文本）。**token 门这一轮抓到了真 bug**：我漏抄了
  `main` 两处 `fatal (…, strerror (errno), …)` 里的 `strerror`（写成了把 errno 强转成指针 ✗，能编过），
  门报 `function main does not mention strerror` 才发现——这正是它存在的理由。
  同时把 `strerror` 补进 `lib/include/string.rs` 并让 `callproc.rs`/`charset.rs`/`emacs.rs`/`sysdep.rs`
  四份用点声明退位（`check-mirror-dupes` 保持棘轮一行）。
  **两条裁决**：① keyboard lane 报的"行数预算"问题：它实测本单元端口密度是每行 C ≈1.1 行 Rust（前段 +4.0%、
  后段 +11.9%），默认 5% 容差对剩余 10,089 行 C 不够——**裁决按成品实测补 tolerance 行**（等收口时测，
  现在不加 provisional 行）；② `lib-src/hexl.c` 与 `lib-src/movemail.c` 的 verified 清单状态：
  movemail 已在清单里（本轮复核 5,734 conditions / 0 disagreements），hexl 是 0 条件文件，无需登记。
- **3.100 复述规则合上机器门，扫描面扩到批次 F（2026-10-04 第 99 轮）**：3.90 清掉 11 处复述之后，
  **规则本身没有门**，两轮内就回来了：3.98 落的 `lib-src/hexl.rs` 头注释里有一段四行辩解
  （"C's own head comment … carried here because it is the file's text, not a claim the port makes
  about itself"），而 `check-declarations.py` 的 `SCAN_DIRS` 只有 `femacs/src`、`femacs/lib`、
  `femacs/xtask`——**`lib-src` 根本不在扫描面里**，所以没有任何门看得见它；同一批的
  `lib/stdbit.rs` 第 1 行也还写着 "C's LGPL header"。本轮做三件事：
  ① **扫描面**：`SCAN_DIRS` 扩到**全部已端口树**（`src`、`lib`、`lib-src`、`exec`、`lwlib`、
  `oldXMenu`、`nt`、`msdos`、`xtask`），批次 F 的每个程序/库从此与核心单元同一把尺；
  ② **第四类 `recital`**：注释里谈论许可/版权/产品名即报（`licence|license|licensing|copyright|
  copyleft`、裸词 `LGPL`/`GPL`、`product name`、`产品名|许可|版权`），大小写敏感并带标识符护栏——
  `emacs_copyright`/`emacs-copyright`/`` `COPYRIGHT` ``/`canonicalize-lgpl.rs` 是**移植的名字**、
  `GPLv3` 是版本号，三者都不许触发（否则这条规则只会被它自己的 allowlist 埋掉）；`RECITAL_TREES`
  把 `femacs/xtask` 排除在这第四类之外（它是门自己的源码，用散文讲这几类）；
  ③ **清场**：`hexl.rs` 删掉那四行（C 8 的出处句只在台账记账），`stdbit.rs` 第 1 行去掉
  "C's LGPL header"。
  验证：`--selftest` **22 cases, 0 failures**（新增 6 例：许可复述、中文复述、无产品名的复述、
  移植标识符、连字符标识符、`canonicalize-lgpl.rs`）；扩面+新类在清场前报
  `11 declaration sites (3 forbidden, 8 allowed literals), 3 failures`（**3 条全是真缺陷**），
  清场后 `8 declaration sites (0 forbidden, 8 allowed literals), 0 failures`；`hexl.rs` 332 → **328 行**、
  `stdbit.rs` 8 行不变，两文件 `build-tu` = `built`、`extract-abi.py --check` = `OK`、
  `rustc --cfg enable_checking` = exit 0；`check-mirror-dupes` 仍 `1 duplicate declaration(s),
  1 ratchet row(s), 0 failure(s)`、`check-verbatim` 仍 `6 trees, 4607 files, 0 failures`。
  **未动**：`docs/ports/*.tsv` 与各台账里同样意思的中性措辞（那是决定记录，不是代码），以及本文件与
  `PORTING.md` 自身的政策叙述。规则补进 `PORTING.md` 0.1-19。
- **3.101 `src/coding.c` 收口入树（2026-10-04 第 99 轮第二批）**：coding lane 交回**完整**单元
  **`femacs/src/coding.rs`（13,435 行 / C 12,312 行；`--check` 的 `missing` 86 → 0、`does not mention` 0、
  无 problem 行）**，台账 `docs/ports/coding.md`（593 行，`- [ ]` 54 → 0）同提交。
  集成侧**在本次提交的工作树上重跑**的复核（不是复述 lane 的数字）：`build-tu src/coding.c` = `built`、
  `extract-abi.py --check` = `OK`、`check-globals` = `274 units, 0 DEFVAR_* variables without a definition in
  their own unit`、`rustc --cfg enable_checking` = exit 0（9 warnings，全是既有的 function-pointer 比较家族）、
  `check-mirror-dupes` = `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`、
  `check-symbols --dir src` = `73 units compared, 1 mismatched`（唯一的 mismatched 是**在飞**的
  `src/keyboard`；coding 的行已从报告里消失 = `missing=[] invented=[]`）、`check-verbatim` =
  `6 trees, 4607 files, 0 failures`。DEFVAR 载体 367 → **387**（+20 是本单元 `syms_of_coding` 的注册项，
  属 status 3.60 已记录的偏离）；行数门按成品实测追加 `coding 0.10`（实测 9.1%，supersede 之前的
  provisional 0.63 行）。**本单元的三条已知偏离**（台账已记）：① 5 个 `*_def` 模块——`decode_coding_object`
  等在 `coding.h` 镜像是 crate-root 声明，定义因此放进 glob 导入模块；② 29 个 DEFVAR 载体的存储写在
  文件头、`syms_of_coding` 只注册（C 的顺序偏离，沿用第一窗口先例）；③ `detect_coding_charset` 的两处
  `sizeof ("…") - 1` 写成 `size_of_val`（sizeof family 的数字门要求）。**未决（交给下一轮）**：
  `docs/baseline/undef-pending.txt` 里被本单元满足的 stale 行（`_Vcoding_system_for_read`/`_write`、
  `_Vdefault_process_coding_system`、`_Vlast_coding_system_used`、`_Vlocale_coding_system`、
  `_Vselect_safe_coding_system_function`、`_Vtranslation_table_for_input` 等）需在**已提交态**的隔离
  worktree 里用 `check-undef.py --write-pending` 重取（棘轮不在工作树上取，见第 7 条纪律）；两个镜像缺陷
  （`src/include/charset.rs` 的 `CHAR_CHARSET_P` 一处 `E0308`、`coding.h` 镜像的 5 个 crate-root 声明）
  仍开，只报告未修。**没有运行证据**：本单元尚未被链接进可运行的 Emacs，`--check`/`build-tu`/
  `check-symbols` 只证明文本、对象与符号表一致。
- **3.102 `lisp-abi-probe` 又能跑了：182 条声明通过 C 类型检查（2026-10-04 第 99 轮第三批）**：
  `xtask gate-probes` 在本轮之前是**红的**——`spikes/lisp-abi-probe.sh` 连 harness 都生成不出来
  （`error: current_column: 1 parameters in the \`/// C:\` prototype, 0 in the Rust declaration`），
  也就是说 `gate-probes`（因而 `gate-full`）在修掉它之前根本走不到 C 差分那一步。三处修复各自都是真 bug：
  ① **`(void)` 是零个参数**：生成器把 `ptrdiff_t current_column (void)`（lisp.h 5110）参数表里的 `void`
  当成一个参数，与 Rust 侧 0 个不符；`tools/gen-lisp-abi-probe.py` 现在把参数表恰好是 `void` 的情形
  当空表；
  ② **函数的 `/// C:` 必须写裸原型**：`record_first_change`（lisp.rs 4182）把它写成散文（``lisp.h 5254:
  `extern void record_first_change (void);` …``），生成器按文档要求拒绝解析——改成裸原型，散文移到前面的
  `/* … */` 注（同文件 4185 行的既有写法）；
  ③ **`to_c` 漏了指针**：只有指向两个 C 基本名与 tag 的指针有规则，`validate_subarray` 的 `*mut ptrdiff_t`
  于是原样进了 harness（clang 报 "a type specifier is required"）；现在 RULES 之后统一把
  `*mut T`/`*const T` 映射成 `T *`。
  修完实测：**`lisp-abi-probe: 182 declarations type-check against lisp.h`（0 unmapped, 0 baselined，
  只有 `die` 与 `sa_stack_alloc` 两条既有的 skipped）**；`extract-abi.py --check emacs/src/lisp.h
  femacs/src/include/lisp.rs` 仍 `OK`，`check-declarations` 仍 0 failures。
  **教训**：probe 类门失败时先看它自己能不能跑（生成器/解析器），再怀疑被检查的对象——这三个缺陷都在门身上，
  门红着就没人知道那 182 条声明是好是坏。**仍未过**：`gate-probes` 的 dead-arms 段在 `lib/boot-time.rs` 上红
  （非冻结臂要 `get_haiku_boot_time`，E0425），记在 3.103 的收尾清单里。
- **3.103 批次 F 的 8 个 `nt/inc/sys` 镜像入树 + 台账名消歧 + 批次 F 镜像的第一个编译器（2026-10-04 第 99 轮第四批）**：
  ① **台账名规则**：`nt/inc/sys/acl.h` 与 `lib/acl.h` 的 stem 都是 `acl`，而 `docs/ports/include-acl.md`
  已被后者占用——`tools/extract-abi.py` 与 `tools/gen-port-inventory.py` 现在把候选名扩成三个
  （`include-<stem>.md`、`<stem>.md`、`include-<参照相对路径用 - 连起来>.md`），并且**只认注行点名本文件的
  台账**（全树 445 个台账里只有 3 个注行不是 `emacs/<file>` 形式，判据因此可靠）；候选全都无主时仍回落到
  第一个存在的（`src/indent.h` 一直读的是 `indent.md`，这条旧宽松保持不动），于是 `nt/inc/sys/acl.h` 的
  台账是 **`docs/ports/include-nt-inc-sys-acl.md`**；
  ② **8 个镜像**（`param`/`time`/`file`/`ioctl`/`acl`/`mman`/`resource`/`wait`）各自 `extract-abi.py --check`
  = `OK`，台账勾满（`resource.h` 的 `RLIMIT_*` 自引用 `#define` 另记 `macro-body-notes.tsv` 三行）；
  ③ **`spikes/f_inc_probe.rs`：批次 F 镜像的第一个编译器**。`xtask layout-check` 只编 `src/include` 与
  `lib/include` 的镜像，`nt/inc/**` 的镜像此前**只有文本门**，于是三个真缺陷一直没人看见：`acl.rs` 把
  `<sys/types.h>` 的 include 写成 `sys_types.rs.in`（真名 `sys_types.in.rs`，**这份镜像根本编不过**）、
  `time.rs` 去 include 冻结目标的 `<sys/time.h>` 镜像（MinGW 的同名头与它冲突：`ITIMER_PROF` 1 vs 2，
  两边都定义 `itimerval`/`setitimer`，E0428 四次且值也是错的）、`acl.rs` 向 `<sys/types.h>` 镜像要
  `ssize_t`（那份镜像没有它，端口从 `<unistd.h>` 镜像取）。三条都已就地修好并写进各自台账；探针已注册进
  `xtask probe_programs()`，所以 `gate-fast` 也会编它。
  实测：`xtask probe ../spikes/f_inc_probe.rs …` 加材质化后的 `rustc` = exit 0（8 个镜像全部通过类型检查）；
  8 个 `--check` 全 `OK`；`check-declarations` = `8 declaration sites (0 forbidden, 8 allowed literals),
  0 failures`（扫描面 3.100 起已含 `femacs/nt`）；`gen-port-inventory.py` 复算 **590 files, done=414,
  wip=32, todo=144**。
  **仍未过（下一轮）**：`gate-probes` 的 dead-arms 段在 `lib/boot-time.rs` 上红——非冻结臂调
  `get_haiku_boot_time`，本单元没有它的定义（E0425）；要么把 Haiku 臂补齐，要么按 P-51 把该臂的依赖记账。
- **3.104 `boot-time-aux.h` 成为镜像，`gate-probes` 转绿；顺带修掉三个真缺陷（2026-10-04 第 99 轮第五批）**：
  起点是 `xtask gate-probes` 在 HEAD 上**红**：dead-arms 段报 `lib/boot-time.rs` 里 `haiku` 臂
  （C 239-244）调 `get_haiku_boot_time`，而该函数住在 `lib/boot-time-aux.h` 里——那个头**没有镜像**，
  单元按"名字写在用点"的老约定只抄了自己编的那一个函数。补法和终局一致：**把 `lib/boot-time-aux.h`
  做成镜像**（`femacs/lib/include/boot-time-aux.rs`，488 行 / C 426，行数门新行 0.15），单元改成
  `include!("include/boot-time-aux.rs")`（337 行 / C 306，旧行 0.25 仍生效），`CTL_KERN`/`KERN_BOOTTIME`
  随活体函数搬进镜像；六个平台臂按单元已在用的 feature 名保留（`linux_or_android`/`linux_no_android`/
  `android`/`openbsd`/`haiku`/`os_h`/`cygwin_win32`+`no_cygwin`）外加四个新名
  （`no_glibc_boottime`/`no_glibc_realtime`/`have_decl_sysinfo`/`win32_vista`），台账
  `docs/ports/include-boot-time-aux.md` 勾满并记决策。
  追 gate 的过程中又挖出**三个 HEAD 上真实存在的破损**（上一次整树 `gate-fast` 停在 `fcbf0bc`，
  批次 F 的提交之后没人再编译过这些单元）：
  ① **`src/fns.rs` 编不过**（E0428 `memmem`）：批次 F 给 `lib/include/string.rs` 加 `memmem` 时，
  fns.rs 自己在用点也声明了一个——而那个声明**没有改名**，于是 `build/femacs/fns.o` 打的是
  `U _memmem`，参照却是 `U _rpl_memmem`（`build/emacs-ref/lib/string.h` 991 行
  `#   define memmem rpl_memmem`）：文本忠实、导出齐全、**调错函数**。现在声明只留在
  `lib/include/string.rs`，写成 `memmem` + `#[link_name = "rpl_memmem"]`（`emacs_fclose` 形状，status 3.57），
  放进 `memmem_decl` 模块 + glob 导入（P-70），好让 **`lib/memmem.rs`**（同一个符号的定义方）以显式项
  遮蔽它而不是撞名；
  ② **`lib/memmem.rs` 编不过**（同一处 E0428）——现在 `build-tu` = `built`，`nm build/femacs/memmem.o`
  含 `T _rpl_memmem`，与参照逐符号一致；
  ③ **`no_trunc` 翻转下 `trunc` 找不到**（`src/editfns.rs` C 4008、`src/timefns.rs` 780 是**活代码**）：
  `lib/include/math.rs` 原来只在"系统有 trunc"时声明它，而 C 的 gnulib `<math.h>` 两种配置都声明
  （定义在 `floatfns.c`）。现在 `math.rs` 的整块原型进 `math_decls` 模块 + glob，`trunc` 无条件声明，
  `src/floatfns.rs` 的定义照 P-70 遮蔽它。（先用"用点声明"试过一次，**`check-mirror-dupes` 判它重复**
  ——门在这里是对的：声明只该有一个落点。）
  验证：**`xtask gate-probes` = EXIT 0**（7 个 C 差分脚本全 `identical`、`lisp-abi-probe` 182 条、
  `gmp-abi-probe` 159 条、`dead arms` **322 translation unit(s) compiled with the non-frozen arms**、
  `layout-check passed`）；`--check` 对 `lib/boot-time.c`/`lib/boot-time-aux.h`/`src/fns.c`/`lib/memmem.c`
  全 `OK`；`build-tu` = `built` 覆盖从 `lib/boot-time.c`、`lib/memmem.c`、`lib-src/make-fingerprint.c`
  到 `src/{fns,editfns,timefns,floatfns}.c`；`check-symbols --pending …` = `78 units compared, 0 mismatched,
  1 still pending`；`check-mirror-dupes` 0 failure；`check-globals` 274/0；`check-declarations` 0 failures。
  **仍未绿（不是本批引入）**：`check-undef` 的 39 条 NEW 全部来自在飞的 `keyboard.o`/`xdisp.o`，
  12 条 stale 棘轮行是 coding 落地后的既得项（要在**已提交态**重取）。
- **3.106 第一个链接门：`xtask gate-load`（2026-10-04 第 99 轮第六批）**：加载阶梯 L0（`docs/port-plan.md`
  §7.1）此前**没有任何基础设施**，而它需要的第一个东西是"能不能把已落的单元链起来"。本轮把它做成门：
  `gate-load` 用 `make -C ../build/emacs-ref/src -n temacs` 问**参照构建自己的链接命令**（不手抄 `LIBES`：
  Makefile 怎么写就怎么用，含 `-lncurses` 与那串 `-L` 路径），把其中的参照目标文件逐个换成
  `build/femacs/<同名>.o`、把 `../lib/libgnu.a` 换成**端口自己的 `lib/` 单元对象**（端口没有归档，
  混进参照的 `libgnu.a` 就是硬约束 2 禁止的 C/Rust 混链），然后真的链一次；链不上的符号就是 L0 的工单。
  `gate-load`（严格）非零退出，`--report` 打印后退出 0 并写 `build/load/undefined.txt`（`符号\t引用它的对象`）。
  **第一次实测**（`--report`）：**230 个未定义符号，来自 44 个对象；零 `duplicate symbol`**——后者是个
  好消息（每个 `.c` 一个 crate 的模型没有在链接层撞名），前者就是 L0 的工作量清单。构成上：
  ① 在飞单元未收口的部分（`keyboard.o` 引 `_Fdiscard_input` 一族、`xdisp.o`/`window.o` 引
  `_MOUSE_HL_INFO` 等）；② **镜像缺口**被链接层准确点出来：`MOUSE_HL_INFO` 在 C 里是 `dispextern.h` 的
  `INLINE` 函数（参照对象里**没有**这个符号，clang 全内联），端口的 `window.rs` 却在用点把它当外部符号调，
  说明 `include/dispextern.rs` 还缺这个（以及 xdisp lane 列出的 `struct it`、`IT_*`、`PRODUCE_GLYPHS` 等）
  内联镜像；③ 未移植单元的 DEFVAR 载体与函数。
  **门的位置**：`gate-load` **不**并入 `gate-fast`/`gate-probes`（它要参照构建树、而且要等阶梯前置工作落地），
  按批次单独跑；`xtask` 的 usage 与 `gate-full` 的说明都指向它。
  **顺带核实**（lanes 报的两处镜像缺陷）：`charset.rs` 的 `CHAR_CHARSET_P` `E0308` **不复现**——
  `build-tu src/coding.c` 与 `build-tu src/xdisp.c` 都 `built`，两单元的 `rustc --cfg enable_checking` 都 exit 0；
  `coding.h` 镜像那 5 条 crate-root 声明仍是待办（待 coding 镜像的下一次整理）。
- **3.107 `gate-load` 第二版：链接交给 rustc，工单从 230 收敛到 165（2026-10-04 第 99 轮第七批）**：
  第一版用 clang 直接链、报 230 个未定义——**其中约 210 个是 Rust 运行时自己的符号**
  （`core::panicking::*`、`rust_begin_unwind`、`compiler_builtins`），因为每个单元都是 `--crate-type lib`
  的目标文件，C 链接器不认识 Rust 运行时。第二版改成**由 rustc 驱动链接**：一个 `#![no_std] #![no_main]`
  的 bin crate 当 driver（`#![no_main]` 让入口仍是端口自己的 `#[unsafe(no_mangle)] main`），driver 里放
  端口自己的 `#[panic_handler]`（abort，PORTING.md 424）和一个 no-op `rust_eh_personality`
  （`libcompiler_builtins` 的预编译 rlib 是给工具链自己的 std 配置编的、带 EH landing pad；panic=abort
  下没人触发它）——rustc 于是自动把 `libcore`/`libcompiler_builtins` 交给 clang，另补 `-lSystem`
  （bin crate 自带 `-nodefaultlibs`）。driver 由 xtask 写进 `build/load/driver.rs`（构建脚手架，不是待移植
  文件），对象与库走 `build/load/link-args.rsp`。
  实测：**165 个未定义符号，全部是端口自己的缺口**，最集中的是 `keyboard.o`（114 条引用）、`window.o`（68）、
  `dispnew.o`（41）、`keymap.o`（38）。构成与三个发现：
  ① **在飞单元的缺口**：keyboard.c 的 `_Fread_char`/`_read_key_sequence`/`_init_keyboard`/`_syms_of_keyboard`
  与它的 `V*`、xdisp.c 的 `_init_iterator`/`_move_it_*`/`_message*`/`_syms_of_xdisp` 一族（两个 lane 在做）；
  ② **P-19 的替身没有定义方**：`_current_buffer`（dispnew.o/xdisp.o 引）与 `_specpdl`/`_specpdl_end`/
  `_specpdl_ptr`（dispnew.o/alloc.o/eval.o 引）在端口里**没有任何单元定义**——C 里它们分别是宏和 `globals`
  成员，链接层第一次把这件事量化；
  ③ **镜像/单元缺口**：`_MOUSE_HL_INFO`（C 的 `INLINE` 函数，参照对象里**没有**这个符号；端口的 `window.rs`
  却在用点当外部符号调 → `include/dispextern.rs` 缺这组内联，与 xdisp lane 的 to-do 一致）、`_rpl_mempcpy`
  （`lib/mempcpy.c` 未移植）、`_sa_stack_alloc`（P-05 栈池未落地）、`_nlist`、`_cursorX`、`_row_hash`。
  `--report` 写 `build/load/undefined.txt`（`符号<TAB>引用它的对象`），这就是 L0 的工单。
- **3.108 批次 F 的 lwlib / oldXMenu 13 个镜像入树；P-19 的收口口径量清（2026-10-04 第 99 轮第八批）**：
  ① **`lwlib/`（9）+ `oldXMenu/`（4）镜像**由 lane 产出、集成侧逐个复核：`lwlib.h`、`lwlib-Xaw/Xlw/Xm`、
  `lwlib-int`、`lwlib-utils`、`lwlib-widget`、`xlwmenu`、`xlwmenuP`、`X10`、`XMenu`、`XMenuInt`、`copyright`
  ——**13 个 `extract-abi.py --check` 全 `OK`**，13 个台账 `docs/ports/include-*.md` 勾满（0 未勾项），
  决策/偏离/证据齐（X11 名字 `Widget`/`XmString`/`Xaw...`/`Display`/`XID` 按 P-26/P-72(3) 保留 C 拼写、
  留给使用方；`oldXMenu/copyright.h` 是**许可正文本身**，按 0.1-19 只留出处行 + 说明的"空镜像"形状）。
  这些头不被任何 probe 编译（本目标无窗口系统，`USE_X_TOOLKIT`/`HAVE_X11`/`HAVE_X_WINDOWS` 皆 undef），
  文本门就是全部验收。`check-declarations` 仍 0 failures、`check-mirror-dupes` 仍 0 failures。
  ② **P-19 的收口口径量清并写进 `PORTING.md` 的 P-19 行**：`specpdl`/`specpdl_end`/`specpdl_ptr`/`current_buffer`
  在 C 是 `thread.h` 131/135/139/147 的宏，镜像仍声明为 `extern` 存储（这正是 `gate-load` 里 `_specpdl*`/
  `_current_buffer` 四个未定义符号的来源）。前置条件已齐：`thread.rs` 落地、它的四个 `macro_rules!` 在、
  `lisp.rs:1439` 已按 C 的 `lisp.h` 2289 `include!("thread.rs")`，所以 `current_thread` 对所有包含 lisp.rs 的单元可见。
  收口三步（删四个 extern 声明 → 读点 `(current_buffer! () as *mut buffer)`、写点 `current_buffer! () = b as *mut c_void`
  → `specpdl*` 直接换字段，字段已是有类型的 `*mut specbinding`），**规模实测 876 处 / 49 个文件**
  （buffer.rs 158、fileio.rs 105、insdel.rs 63、include/buffer.rs 58、editfns.rs 52……），且必须等两个在飞单元
  （keyboard.rs 12、xdisp.rs 6）落地后**一次做完**，否则中间态会打断它们的构建。
  ③ **lane 进度**：`nt/inc` 的 19 个里已落 15 个（剩 `stdint.h`、`sys/stat.h`、`sys/socket.h`、`unistd.h` 四个大件）。
- **3.109 `MOUSE_HL_INFO` 从"外部符号"改回宏：gate-load 少一个不存在的符号（2026-10-04 第 99 轮第九批）**：
  `gate-load` 报的 `_MOUSE_HL_INFO` 是个**参照里根本不存在的符号**——C 的 `MOUSE_HL_INFO` 是
  `frame.h` 1023-1039 的宏（三条臂，冻结配置无窗口系统所以走 TTY 那条：
  `(&(F)->output_data.tty->display_info->mouse_highlight)`），clang 把它内联掉了，参照对象里没有它；
  而端口的 `src/window.rs` 在**用点**把它声明成 `unsafe extern "C" fn`（注释还把它记成
  `dispextern.h 4020` 的声明——dispextern.h 里没有这条），`src/dispnew.rs` 也照着调。
  修法：把宏放回它的 C 老家 `include/frame.rs`（`macro_rules! MOUSE_HL_INFO`，body 里
  `tty_output`/`tty_display_info`/`mouse_highlight` 由**使用方**的 `termchar.rs` 提供——`frame.rs` 不能
  自己 include termchar.rs，那是环，C 的宏同样是不卫生的），`window.rs` 与 `dispnew.rs` 的调用点改成
  `MOUSE_HL_INFO! (f)`。
  实测：`build-tu src/window.c`/`src/dispnew.c` = `built`；两文件 `--check` 仍 `OK`（C 的拼写是调用形状，
  Rust 侧是宏调用，token 门按名字比对 ✓）；`check-mirror-dupes` 仍是 0 failure；`gate-load --report`
  **165 → 164 个未定义符号，`_MOUSE_HL_INFO` 消失**。`docs/ports/include-frame.md` 的第 178 项由此勾上
  （frame.h 的镜像还剩 47 个未勾项，是它的 display/face 结构族，见该台账的 blocked 段）。
- **3.110 `nt/inc` 19 个镜像入树（27 个全 OK）；lisp.rs 两个条件常量修回 `false`；keyboard lane 第三窗口（2026-10-04 第 99 轮第十批）**：
  ① **`nt/inc` 批次完成**：lane 落 19 个镜像（`arpa/inet`、`dirent`、`grp`、`inttypes`、`langinfo`、`mingw_time`、
  `ms-w32`、`netdb`、`netinet/{in,tcp}`、`nl_types`、`pwd`、`signal`、`stdalign`、`stdbool`、`stdint`、
  `sys/{socket,stat}`、`unistd`），集成侧逐个复核：`nt/inc` 下**27 个 `--check` 全 `OK`**、19 个台账勾满（0 未勾项）。
  集成时抓到一处**门需要的共享行没落地**：`nt/inc/langinfo.h` 的 22 条自引用 `#define X X`（`CODESET`、
  `DAY_*`、`MON_*`、`_NL_PAPER_*`）在门里要"宏体的 token 出现在 Rust 文本里"，而镜像只按 1.3 的枚举规则写成常量
  ——补 **5 行 glob 化的 `docs/ports/macro-body-notes.tsv`**（`CODESET`、`DAY_*`、`MON_*`、两条 `_NL_PAPER_*`）
  后 `OK`。这些镜像**不被任何 probe 编译**（无 `nt/` 单元被编），文本门是全部验收；`nt/inc/sys/**` 那 8 个
  另有 `spikes/f_inc_probe.rs` 的编译证据。
  ② **`include/lisp.rs` 的两个条件常量修回 `false`**（keyboard lane 报的，是"会静默走错分支"的一类）：
  `HAVE_EXT_MENU_BAR`/`HAVE_EXT_TOOL_BAR` 是 lisp.h 3930-3936 的 `#if defined (USE_X_TOOLKIT) || …` 条件
  `#define`，冻结表里两个键都**不存在** ⇒ C 里未定义、值为 false，而镜像原写 `true`。今天没有单元按值读它们
  （都写成 `#[cfg]`），但第一次有人写成 `if HAVE_EXT_*` 就会走错臂。修后 `--check emacs/src/lisp.h` 仍 `OK`。
  **审计口径**：镜像里的 `pub const NAME: bool = true` 应与 `docs/baseline/config/darwin-arm64.macros.tsv`
  对账，条件 `#define` 缺席的一律 `false`。
  ③ **keyboard lane 第三窗口**：C 4594-5698 / 5713-5784 / 6093-6219（+1,304 C 行）落地，`keyboard.rs` 4,938 →
  **6,304 行**；台账 `[ ]` 176 → 143，`--check` missing 212 → 173，D7 行数漂移 66.4% → 57.1%；它的一条
  `timer_check_2 continue` 说明行已由集成者补进 `macro-body-notes.tsv`。**裁决**：`keyboard` 的 tolerance 行
  **现在不加**（3.99 的裁决仍然有效：等单元收口按成品实测再补；WIP 单元的漂移不会让 `--all` FAIL——它按台账
  未勾项先归 WIP）。它报的镜像缺口（`include/window.rs` 的 `WINDOW_*` 几何宏是 `make_lispy_position` 的前置）
  已记入 `docs/ports/keyboard.md`，是下一个镜像任务。它遇到的 `gate-fast` 在 `--all` 汇总前中止（无 traceback、
  约 20 分钟）与集成侧同轮成功跑完的 `checked 447 ported files: 422 done, 25 in progress, 0 failing` 不一致，
  记录为待复查的环境现象。
- **3.112 链接工单 164 → 163：`rpl_mempcpy` 的定义端正名 + gate-load 的棘轮 + `0o` 常量归一（2026-10-04 第 99 轮第十一批）**：
  ① **`mempcpy` 的符号**：**生成的** `<string.h>`（`build/emacs-ref/lib/string.h` 1027-1028
  `# undef mempcpy` / `# define mempcpy rpl_mempcpy`）把每个编译单元里的标识符改名，所以 C 的 `mempcpy.c`
  定义的是 `rpl_mempcpy`、调用它的 `src/doprnt.c` 284 也编译成它。端口这边却是"定义端 `no_mangle` 出
  `_mempcpy`、调用端声明 `rpl_mempcpy`"——两边各说各话，`gate-load` 因此报 `_rpl_mempcpy` 未定义。定义端
  改成 `#[unsafe(export_name = "rpl_mempcpy")]`、`canonicalize-lgpl.rs` 的用点声明与 `__mempcpy!` 宏体改叫
  `rpl_mempcpy`（它原来声明 `mempcpy`，所以该目标文件要的是 `_mempcpy`）后：`nm build/femacs/mempcpy.o`
  打 `T _rpl_mempcpy`，**链接工单 164 → 163**。
  ② **`gate-load` 加棘轮**：`docs/baseline/link-pending.txt`（`符号<TAB>理由`，与 `undef-pending`/
  `symbol-pending` 同制），只登记"冻结配置**证明**提供不了的符号"——目前 **1 行**：`_nlist` 来自
  `lib/getloadavg.c` 的 nlist 仿真（C 226 的 `LOAD_AVE_TYPE` 在端口读法是活的），而那个文件在本目标上
  **根本编不过**（`make -C build/emacs-ref/lib getloadavg.o` → `fatal error: 'a.out.h' file not found`），
  参照因此也没有它的目标文件，macOS 也不导出 `nlist`（C 探针同样链不上）。门现在把未定义分成
  "棘轮内 / 新增两堆"，只在有新增时非零退出 ✓。同时撤掉上一版按 `libgnu.a` 成员表过滤对象文件的尝试
  （那会把端口自己的 GMP 替身 `mini-gmp.o` 排除掉，`_mpz_*` 立刻变成 200+ 条）。
  ③ **`norm_const` 认 Rust 的 `0o`**：`nt/inc/sys/include/socket.rs` 的 `O_NONBLOCK` 原本为了绕过工具缺口写成
  `0x800`（lane 报告），现在 `tools/extract-abi.py` 的正则收了 `0[oO][0-7]+`，镜像改回 C 的忠实拼写
  **`0o4000`** 且 `--check` 仍 `OK`；抽样复核 `src/syntax.c`/`src/charset.c` 等使用 `0o` 的单元不受影响
  （归一化只会更宽松）。
- **3.113 P-05 的栈池落地：`_sa_stack_alloc` 不再是未定义符号（2026-10-04 第 99 轮第十二批）**：
  `AVAIL_ALLOCA!` 一直向一个从未定义的 `sa_stack_alloc` 要字节（`dispnew.o`/`frame.o`/`scroll.o` 等
  三处引用），这是 `gate-load` 工单里唯一"设计未定"的项（P-05 标着"待 spike"）。本轮按 PORTING.md §3.2 的口径落地：
  ① `femacs/src/alloc.rs` 增**栈池**：`sa_stack_alloc`（对齐到 16 字节、记账 `sa_avail` 之外再记 specpdl 深度）与
  `sa_stack_release`（`safe_free` 调用），块头串成链表，**严格**按 `depth > 被解开的深度` 释放；
  ② `femacs/src/include/lisp.rs` 的 `safe_free` 末尾调 `sa_stack_release (unwrap_specpdl_ref (sa_count))`（C 的
  `SAFE_FREE` 不负责 alloca 字节——那是栈帧的事，这行是端口机制，注释里写明）；
  ③ **两条已记录偏离**：一次请求一个 malloc 块（P-05 写的是"单一栈缓冲池"，但释放是子集，bump 区在最后一个块还回去之前
  不能释放），以及同深度起止的作用域要到更浅的释放才还（保守，不会释放仍然活着的块；C 的栈帧语义更早回收）。
  **关键裁决**：这两个名字是**唯一的"端口自有导出"类**——两个门（`extract-abi.py` 的 invented-exported-symbol 规则、
  `check-symbols.py` 的 invented 表）原先一律拒绝自带 `#[unsafe(no_mangle)]` 而 C 没有定义的名字（这是对的：
  `emacs_fclose`/P-52 类），现在各自加一个显式的 `PORT_ONLY` 常量放行这两个名字并注释指向 P-05；别的一律仍然拒绝。
  实测：`build-tu src/alloc.c` 与七个调用方（dispnew/frame/scroll/editfns/font/eval/pdumper）全 `built`；
  `--check emacs/src/alloc.c` = `OK`（此前报 "invented exported symbol"）；`check-symbols --pending` 里 `src/alloc`
  不再出现（只剩在飞的 `src/keyboard`）；**`gate-load` 162 total / 1 ratcheted / 161 new，`sa_stack_*` 从工单里消失**。
  **仍欠**：P-05 的 spike（与 `record_xmalloc` 路径的内存/性能对拍、`depth == sa_count` 的兄弟与嵌套作用域、恒在深度 0 的分配）。
- **3.114 `cursorX` 也是宏（`MOUSE_HL_INFO` 同类）：工单 162 → 161（2026-10-04 第 99 轮第十三批）**：
  `gate-load` 的 `_cursorX`（引用方 `sysdep.o`）又是一处"把 C 的宏写成外部符号"：`termhooks.h` 979-984
  只在 `WINDOWSNT` 下声明 `extern int cursorX (struct tty_display_info *)`，其余情况是
  `#define cursorX(t) curX(t)`（cm.h 的 place 宏），所以冻结配置里 C 的 `sysdep.c` 1526 编成 `curX (tty_out)`，
  参照对象**没有** `_cursorX`。端口把 `cursorX` 写在用点当 `unsafe extern "C" fn`，调用点又按函数调它
  （`term.rs` 那边其实已经是 `cursorX!` 宏了）。修法：删掉该 extern 声明（`cursorX` 的宏在
  `include/termhooks.rs`、`curX!` 在 `include/cm.rs`，`sysdep.rs` 已按 C 的 `#include "cm.h"` 引了它），
  调用点改 `cursorX! (tty_out)`。实测：`build-tu src/sysdep.c` = `built`、`--check emacs/src/sysdep.c` = `OK`、
  `nm -u build/femacs/sysdep.o` 里 cursorX 归零、**`gate-load` 161 total / 1 ratcheted / 160 new**。
  顺带核实：同批工单里的 `_count_lines` **不是**缺陷——C 的 `count_lines` 定义在 `src/xdisp.c:29897`，
  属于在飞 xdisp lane 的余量（`fns.o`/`lread.o` 引用它是忠实的）。

- **3.115 `lib-src/make-docfile.c` 入树：1232 行 C、21 个函数、2 个结构体（2026-10-04 第 100 轮）**：
  lane 交回完整单元 **`femacs/lib-src/make-docfile.rs`（1,686 行 / C 1,232 行）**与台账
  **`docs/ports/make-docfile.md`**（`- [ ]` 29 → 0）。集成侧要做的是它无权做的两笔共享行：
  ① **`docs/ports/macro-body-notes.tsv` 追加 5 行**——`error`/`fatal` + `va_end`（C 的 `va_end (ap);` 与既有的
  va_list/va_start 同类，P-04）、`write_c_args` + `uninit`（`UNINIT` 在冻结配置为空）、`write_globals` +
  `break,case,default,switch`（C 的 `switch` 变 `match`，2.2）、`scan_c_stream` + `goto`（C 的 `eof:` 标签与
  前向 `goto` 变 `'eof` 标签循环，2.2）；② **`docs/ports/line-drift-tolerance.tsv` 追加 `make-docfile 0.38`**——
  实测漂移 **36.85%**（C 的紧凑风格：一行 `if (…) continue;`、`case …: …; break;`、
  `do c = getc (infile); while (…)`，在端口里各要一个花括号块，纯粹的花括号行 **504 对 C 的 201**），
  按"实测 + 0.01"取 0.38（hexl 0.55 / make-fingerprint 0.45 同类）。
  **死臂**：C 53-58（`#undef fopen` + `<direct.h>`）与 C 60-67（`#undef chdir`）在冻结配置里是**纯预处理器行、
  没有语句**，按 P-08 记账不转写；证据 `python3 tools/check-cond-eval.py lib-src/make-docfile.c` →
  `2 conditions, 0 unknown, 0 disagreements [VERIFIED]`。
  实测（集成者在本次提交的工作树上重跑，不是复述 lane 的数字）：`build-tu lib-src/make-docfile.c` = `built`；
  `extract-abi.py --check emacs/lib-src/make-docfile.c femacs/lib-src/make-docfile.rs` = **`OK`**
  （此前 5 条 "does not mention" + 1 条行数；共享行落地后全消）；手工 `rustc --crate-type=lib --emit=metadata`
  = exit 0；`check-globals` = 274 units / 0 缺定义；`check-declarations` = `8 declaration sites
  (0 forbidden, 8 allowed literals), 0 failures`；`check-verbatim` = `6 trees, 4607 files, 0 failures`；
  `check-mirror-dupes` = `4 duplicate declaration(s), 1 ratchet row(s), 3 failure(s)`——**3 条全是别的 lane
  在飞的配对**（`femacs/src/xdisp.rs` 对 `femacs/src/include/window.rs` 的 `window_wants_mode_line`/
  `window_wants_tab_line` 等，Sartre/Lagrange 的窗口 lane），没有一条涉及本单元。
  `docs/port-inventory.md` 复算：**590 files, done=454, wip=32, todo=104**（上一批 done=453/todo=105）。
  **未跑**：`extract-abi.py --all`（20 分钟级，留给整树门那一轮）与 `gate-load`（本次没有改链接相关的代码）。
  同一笔提交里还带上 lane 并发写进 `macro-body-notes.tsv` 的一行 `WINDOW_BOTTOM_DIVIDER_WIDTH`
  （在飞的窗口 lane 的注记，纯追加，不影响本单元）。

- **3.116 `lib-src/pop.c` 入树：整文件死单元（0 定义），以及 `--all` 的复查（2026-10-04 第 100 轮第二批）**：
  `lib-src/pop.c`（1,703 行）**整文件体在 `#ifdef MAIL_USE_POP` 内**，而冻结表里 `MAIL_USE_POP` 是
  `undef`（`docs/baseline/config/darwin-arm64.macros.tsv` 2691），所以它是一个按 **3.25/3.49** 政策产出的
  **空单元**（`femacs/lib-src/pop.rs`，30 行说明、0 定义、0 用点声明——`<pop.h>` 只被死区内的 `#include` 用到）。
  这不是 stub：**参照对象同样零产出**，实测逐字相同——
  `nm build/emacs-ref/lib-src/pop.o` 与 `nm build/femacs/pop.o` 都只打 `0000000000000000 t ltmp0`，
  `size` 两个对象都是 `0 0 0 0 0 0`。（同族先例：`decompress.rs`/`dbusbind.rs`/`dosfns.rs`；`movemail.rs`
  对同一个门也是"recorded not transcribed"。）
  集成侧补一笔共享行：`docs/ports/line-drift-tolerance.tsv` 的 `pop 0.99`（端口 30 行 vs C 1,703 行，
  实测漂移 98.24%，与 `decompress` 同值），补后 `--check emacs/lib-src/pop.c femacs/lib-src/pop.rs` = **`OK`**；
  `build-tu lib-src/pop.c` = `built`；手工 `rustc --emit=metadata` = exit 0；`check-declarations` 0 forbidden、
  `check-globals` 274/0、`check-mirror-dupes` **1 ratchet row / 0 failure**（`xdisp.rs` 那 3 条重复声明已被
  接手的 lane 收敛掉）、`check-verbatim` 6 trees/4607 files/0。
  **本轮 lane 布局**（都写各自单元、集成者提交）：malloc 的 `scratch_buffer` 簇与 `dynarray` 簇各一条、
  `lib-src/emacsclient.rs` 一条、`lib-src/ebrowse.rs` 待开工，外加**接手的 xdisp lane**（前一条 lane 的
  handle 已失效，`femacs/src/xdisp.rs` 是孤儿在飞文件，现由新 lane 从台账边界继续；它已先按 0.1-3 把
  `window_wants_mode_line`/`window_wants_tab_line` 等重复声明收敛掉）。
  **`--all` 复查（进行时）**：3.101/3.99 两处 lane 报"`gate-fast` 的 inventory 步骤在打印汇总前中止、无
  traceback"。本轮实测复跑：`python3 tools/extract-abi.py --all` 的 Python 进程 **RSS 41.9 MB、99% CPU、
  3 分钟时输出 82 行**（远未到 20 分钟），说明它不是一启动就死；结论待跑完后再补。

- **3.117 `--all` 从"20+ 分钟不返回"变成 106 秒：每函数重扫整份 C 文件的四条 memo（2026-10-04 第 100 轮第三批）**：
  两条 lane 报过"`gate-fast` 的 inventory 步骤在打印 `checked N ported files` 汇总前结束、无 traceback"，
  本轮把它当成主验收闸门不可用的问题查到底（证据都是实测，不是推断）：
  **① 根因（cProfile 3,000 行切片 + 逐级计时）**：`tools/extract-abi.py` 的 `c_body_text`/`body_is_empty`
  **按每个 C 函数**重新扫整份参照文件——`blank_comments` 逐字符重写（341 次/6,000 行）、`blank_literals`
  重扫、`function_spans`/`defun_spans` 重解析、`dead_branch_map` 重算每个 `#if`；`body_is_empty` 还每次
  重新 `read_text`，**新字符串对象把按对象的 memo 一起打掉**（memo 只按 `id(text)` 命中）。另有
  `note_tokens` 的 **71,400 次 `fnmatchcase`**（fnmatch 自带 256 项缓存，被 `macro-body-notes.tsv` 的
  ~1,100 条 pattern 击穿，profile 里 69,900 次 `translate`，占 12 s 中的 4.1 s）。
  实测缩放：3,000 行 8.6 s / 6,000 行 26.7 s / 12,000 行 74.1 s，整份 `xdisp.c`（39,805 行）**十分钟不结束、
  RSS 450 MB**——这就是"静默中止"和用户先前看到的"Python 吃内存"的同一条路径。
  **② 修法（行为不变）**：`_lines_for`/`_rows_for`/`_spans_for`/`_text_for` 四条按对象的 memo（都带
  `hit[0] is text` 的身份校验并保强引用，上限 8 项，避免 `id` 复用答错文件；参照树整轮只读，所以文本
  不会过期），`_note_pattern` 预编译 `fnmatch.translate`（`re.compile(...).match` 就是 `fnmatchcase` 本体）。
  **③ 验证**：四个文件的 `--check` stdout **逐字节不变**（`src/dispnew.c`、`lib-src/hexl.c`、`src/window.c`、
  `lib/getopt.c`）；6,000 行 **26.7 s → 1 s**、12,000 行 **74.1 s → 2 s**、整份 `src/xdisp.c` **>600 s（450 MB）
  → 6 s**；`python3 tools/extract-abi.py --all` **106 s 跑完并打出汇总**：
  `checked 500 ported files: 472 done, 27 in progress, 1 failing`——**唯一 FAIL 是 `lib-src/emacsclient.c`
  还欠的 9 行共享豁免行**（该单元的框已勾，属集成侧待补），其余 27 条全是台账仍有未勾项的 WIP。

- **3.121 三个门补上递归：`lib/malloc/` 这样的嵌套参照目录第一次有了符号/全局/镜像重复覆盖（2026-10-04 第 100 轮第四批）**：
  `lib/malloc/` 是本树第一个**参照自身带子目录**的端口树（单元在 `femacs/lib/malloc/*.rs`、镜像在
  `femacs/lib/malloc/include/*.rs`），而 `tools/check-globals.py` 的 `units()`、`tools/check-symbols.py` 的
  单元循环、`tools/check-mirror-dupes.py` 的两侧都只做**单层** `glob("*.rs")`——三层门都看不见这批文件
  （两条 lane 独立报过）。修法：
  ① `check-globals.py`：`units()` 改成对 `femacs/{src,lib}` 递归，并排除路径里带 `include` 的组件（镜像是
  "被 include 的"，不是编译单元）；单位数 **274 → 283**（正好 +9 = malloc 的 9 个单元），仍 `0` 缺定义；
  ② `check-symbols.py`：单元按参照路径**递归**发现（同样排除 `include`），并且**参照对象按参照自己的目录**
  解析（`build/emacs-ref/lib/malloc/x.o`，而不是扁平化后的 `build/emacs-ref/lib/x.o`）——不修这一条，
  递归只会把新单元"发现"出来然后因为参照对象路径不存在而跳过；`--dir lib` 的比对单位数 **78 → 86**，
  仍只有既有的 `lib/mempcpy` 一条 mismatch（1 pending）；
  ③ `check-mirror-dupes.py`：镜像改成"任一已端口树里路径含 `include` 的 `.rs`"（因此
  `lib/malloc/include/*.rs` 进了扫描面），单元同规则排除 `include`；读数仍是 `1 duplicate declaration(s),
  1 ratchet row(s), 0 failure(s)`（既有的 `insdel.rs` 棘轮行），说明新面里没有隐藏重复。
  验证：`xtask gate-fast` **整条链第一次跑完**（287 s；此前卡在 inventory 步骤，见 3.117）——它现在把
  501 个端口文件报成 `476 done / 24 in progress / 1 failing`，**唯一那条 FAIL 是 `lib-src/etags.rs`**：
  那条 lane 刚落文件、台账 `docs/ports/etags.md` 还没建（"有端口文件、没有台账"按设计就是 FAIL），已通知
  该 lane 下一个停顿点先落台账；其余 24 条都是有未勾项的 WIP。

- **3.122 `src/comp.h` 镜像收口：`#else` 分支的两个空体 inline，加上三件死文本的记账（2026-10-04 第 100 轮第五批）**：
  冻结配置里 `HAVE_NATIVE_COMP` 是 `undef`（macros.tsv 1473），所以 comp.h 走 `#else` 分支，那里只有三样：
  `maybe_defer_native_compilation`（C 92-95）与 `unload_comp_unit`（C 97-98）两个 **`static inline` 空体**（空体是
  参照自己的，不是端口 stub），以及 `extern void syms_of_comp (void)`（C 100，归 comp.c 自己的单元）。镜像原先只有
  前者，`--check` 因此报唯一一条 `missing function unload_comp_unit`。本轮补上后者，并把台账里另外三个未勾项
  记成 **P-08 死文本**：`NATIVE_COMP_UNITP`（C 57）、`XNATIVE_COMP_UNIT`（C 63）与带字段的
  `struct Lisp_Native_Comp_Unit`（C 30-44）都在 `#ifdef HAVE_NATIVE_COMP` 里，本配置不编译（参照对象里也没有
  这些名字），只记账不转写。
  **一处签名偏离（已记台账）**：C 的 `#else` 分支从不定义那个结构体，只在空体 inline 的形参里点它的名——
  C 允许不完整类型，Rust 不行，于是端口写 `*mut core::ffi::c_void`（PORTING.md P-72③）；带字段的定义在
  `femacs/src/pdumper.rs`（`native_comp` feature 下），因为拥有它的 `comp.c` 还没移植（batch G）。
  实测：`extract-abi.py --check emacs/src/comp.h femacs/src/include/comp.rs` = **`OK`**；四个经 `lisp.h` 2168
  到达本镜像的消费者 —— `src/emacs.c`/`src/alloc.c`/`src/print.c`/`src/pdumper.c` —— 全部 `built`；手工
  `rustc --emit=metadata`（emacs 单元）= exit 0；`check-mirror-dupes` `1 ratchet row / 0 failure`、
  `check-declarations` `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、`check-verbatim` 0、
  `check-globals` `283 units / 0`。`port-inventory` 复算 **590 files, done=470, wip=32, todo=88**。
  **同一批（未单独提交，随各自单元入树）**：补上 `docs/ports/ebrowse.md`（Arendt 已落 116 KB 的
  `femacs/lib-src/ebrowse.rs` 但还没建台账——按门的规则"有端口文件、没有台账 = FAIL"，那是当时全树唯一的
  1 failing；补后台账让它按 WIP 计数，套件回到 **0 failing**）；把两个新程序（`etags.rs`/`ebrowse.rs`）的
  **5 条 C 自有字面量**登记进 `docs/baseline/declarations-allowlist.tsv`（版本 banner/`COPYRIGHT`/
  `PACKAGE_NAME`），并修掉 `ebrowse.rs` 里一段**复述版权散文的注释**（0.1-19 的 `decl` 类）。
  另：keyboard lane 在补完 `make_lispy_position`（C 5785-6099）后交回（6,304 → 6,701 行、两处 P-51 死臂改 `NAME!`、
  加 7 条用点 extern），已另起 lane 从 `make_lispy_event`（C 6230-7309）继续，并要求它先把台账更新到与产出
  一致；etags lane 交回 C 143-1659（1,787 行）后暴露出结构性前置——`lang_names` 表要取 31 个语言解析器的地址，
  而它们定义在 C 3456-7000，Rust 没有前向声明（0.1-12），所以**该单元在解析器落地前不可能编译**，新 lane 已按
  "先移植 31 个解析器"的顺序派出。

- **3.123 `undef-pending.txt` 棘轮按"只减不增"收缩 15 条（2026-10-04 第 100 轮第六批）**：
  棘轮的完整重取要求**已提交态的隔离 worktree**（见 3.112 的口径），而当前树里有 5 条在飞 lane，所以本轮
  只做**严格收缩**那一步：把已经是"陈旧（now satisfied）"的 15 条删掉，一条也不新增——
  `_Vcoding_system_for_read`/`_Vcoding_system_for_write`/`_Vdefault_process_coding_system`/
  `_Vlast_coding_system_used`/`_Vlocale_coding_system`/`_Vminibuf_scroll_window`/
  `_Vselect_safe_coding_system_function`/`_Vtranslation_table_for_input`/`_coding_system_require_warning`/
  `_inherit_process_coding_system`/`_mode_line_in_non_selected_windows`/`_record_auto_save`（coding.rs 落地后
  它们已有定义）、`_cursorX`（3.114 的宏化）、`_rpl_mempcpy`（3.112 的正名）、`_sa_stack_alloc`（3.113 的栈池）。
  做法是用 `check-undef.py --write-pending` 生成一份"现状"副本，再**只把旧文件里出现在删除集合里的行删掉**
  （新增的 43 条全是 keyboard/window 等在飞单元的未定义引用，属于**不能**在脏树上烘进棘轮的部分）。
  实测：`check-undef` 从 `15 stale + 43 NEW` 变成 **`0 stale + 43 NEW`**；`undef-pending.txt` 117 → 102 条数据行。

- **3.124 `src/disptab.h` 镜像收口：glyph-table 家族落地，term.rs/dispnew.rs 的 12 处副本同批退位（2026-10-04 第 100 轮第七批）**：
  该镜像的 WIP 注释一直写着"等 dispextern.h 的 `GLYPH_*` 访问器镜像落地再补"——现在那个前提已经满足，于是本轮把
  disptab.h 72-122 整段补上：`GLYPH_TABLE_LENGTH`/`GLYPH_TABLE_BASE`、`GLYPH_SIMPLE_P`/`GLYPH_ALIAS_P`/
  `GLYPH_FOLLOW_ALIASES`（C 的 `do { … } while (false)` 写成块，`SET_GLYPH_FROM_GLYPH_CODE` 同形）、
  `GLYPH_LENGTH`/`GLYPH_STRING`、`SPACEGLYPH`（C 的八进制 040 = 32）、`NULL_GLYPH`（0）与
  `SET_GLYPH_FROM_CHAR`；每个宏都是 `macro_rules!`，因为 C 的就是（P-33）。
  **同批收敛（0.1-3）**：`femacs/src/term.rs` 原来自带其中 7 个宏 + 两个常量、`femacs/src/dispnew.rs` 自带 5 个 +
  `SET_GLYPH_FROM_CHAR`（各自都写着"disptab 镜像到这里为止"），两个单元都 include 本镜像，所以家族一落地它们
  就是重复定义——共删 12 处（term.rs −40 行、dispnew.rs −27 行）。其余 7 个消费者（doc/indent/xfaces/print/
  window/character + 在飞的 xdisp/keyboard）一行都不用改：它们从来自带副本。
  一笔共享行：`GLYPH_FOLLOW_ALIASES` 的 `false`（C 的循环测试，Rust 块里没有落点）进
  `docs/ports/macro-body-notes.tsv`。
  实测：`extract-abi.py --check emacs/src/disptab.h femacs/src/include/disptab.rs` = **`OK`**；十个消费者
  （`term`/`dispnew`/`doc`/`indent`/`xfaces`/`print`/`window`/`character`/`xdisp`/`keyboard`）全部 `built`；
  `check-mirror-dupes` `1 ratchet row / 0 failure`、`check-declarations` `13 sites (0 forbidden, 13 allowed
  literals) / 0 failures`、`check-verbatim` 0、`check-globals` `283 units / 0`；`port-inventory` 复算
  **590 files, done=472, wip=30, todo=88**。
  顺带清掉一个**跑出来的杂散产物**：仓库根出现 `BROWSE`（54 KB，ebrowse 对 `emacs/src/xdisp.c` 生成的标签文件，
  某条 lane 的探针留在工作树里），已删除并提醒该 lane 把这类探针产物放到 `/tmp`。

- **3.125 三个小镜像收口：`issymlink.h` / `issymlinkat.h` / `openat-priv.h`（2026-10-04 第 100 轮第八批）**：
  三个文件各自只差一项，收口后 `--check` 全 `OK`：
  ① `lib/issymlink.h` 与 ② `lib/issymlinkat.h` 的 `_GL_ISSYMLINK_INLINE` / `_GL_ISSYMLINKAT_INLINE`
  （C 31-33 的 `#ifndef … / # define … _GL_INLINE`）——**不能在镜像里落成 const**：两个单元
  （`femacs/lib/issymlink.rs` / `issymlinkat.rs`）按 C 的做法在 include 之前自己定义它
  （`#define … _GL_EXTERN_INLINE`），镜像再定义一次就是 `E0428`（实测踩到）。于是改用既有的
  `docs/ports/excluded-macros.txt` 机制记两行（`kind=syntax`，与 `_GL_TIMESPEC_INLINE`/`_GL_U64_INLINE`/
  `_GL_UTIMENS_INLINE` 同族），并在镜像注释里写明为什么这里不落名字；
  ③ `lib/openat-priv.h` 20-21 的 include guard `_GL_HEADER_OPENAT_PRIV` 落成
  `pub const _GL_HEADER_OPENAT_PRIV: bool = true;`（其它 guard 的既有写法）。
  实测：三个文件的 `--check` 全 `OK`；消费者 `build-tu lib/issymlink.c` / `lib/issymlinkat.c` /
  `lib/lchmod.c` 全 `built`；`check-declarations` `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、
  `check-verbatim` 0、`check-globals` `283 units / 0`；`port-inventory` 复算 **590 files, done=475,
  wip=27, todo=88**。
  **两条在飞 lane 的红灯（已反馈给对应 lane，不是本批引入）**：`check-mirror-dupes` 报
  `femacs/lib/rawmemchr.rs: UCHAR_WIDTH is already declared by femacs/lib/include/limits.in.rs`（镜像 lane 正在
  重写 limits.in.rs，需同批收敛 rawmemchr.rs 的副本）；`build-tu lib/openat-proc.c` 报
  `cannot find type c_longlong`（同一镜像的新文本用了 `c_longlong`/`c_ulonglong`，但包含它的单元未必 import
  这两个名字——建议镜像里写全限定 `core::ffi::c_longlong`，或按单元 prelude 的既有约定处理）。

- **3.126 `lib-src/ebrowse.c` 入树：4,414 行、150 项全勾，并附本项目第一份成规模的**行为差分**证据（2026-10-04 第 100 轮第九批）**：
  单元 `femacs/lib-src/ebrowse.rs`（**4,414 行** / C 3,926）落地，`--check` 从不 OK 到 **`OK`** 靠的正是集成者该补的
  15 条共享行：`macro-body-notes.tsv` 14 条（`switch`→`match` 与 `goto`→标签的必然缺 token，含 `FALLTHROUGH` 合并臂；
  `version` 的 `PACKAGE_VERSION`/`COPYRIGHT` 被折进一个字面量）+ `line-drift-tolerance.tsv` 的 `ebrowse 0.13`
  （实测 +12.4%）；台账 150 个框按 `--check` = OK 的读数勾满。
  **行为差分（本批最有分量的证据）**：lane 把物化单元在 `/tmp` 编成可执行文件（副本改名 `main`，仓库文件未改），
  与用参照 `ebrowse.c` 编出的二进制并排跑，比对 **BROWSE 输出 / stdout / stderr / 退出码**：
  参照树全部 661 个 C/C++ 文件（`-x`）、`src|lib|lib-src` 488 文件 × 3 组选项（1,464 次）、26 个 C++ 文件、
  `--position-info=N`/`-p N` 扫描、550 个生成输入、47 个手写边界用例 × 7 组选项（329 次），以及命令行路径
  （多文件/stdin/`--files=`/`-a`/`-I`/`--version`/`--help`/`--`/两次 `-o`/非法选项/打不开的文件）——**零差异**。
  差分抓出并修掉两个真 bug：`yyival` 的 C 无符号回绕（原写法在 debug 断言下直接 abort）、
  `{"position-info", required_argument}` 被抄成 `no_argument`。这是树里第一次有单元拿到"运行期逐字节对拍"级证据
  （此前多数单元只有文本/对象层证据；status 3.63 记过"门不是语义证明"的教训）。
  两条**有意偏离**（都记进台账）：① `is_ident/is_digit/is_white` 声明为 `[c_char; 256]` 而非 C 的 `[255]`
  （C 的 `IDENTP(0xFF)` 读越界一字节，Rust 会 panic；差分已覆盖 `0xFF` 输入）；② 参照自带的
  `case 'R'` 首字母吞噬 bug **1:1 复刻、不修**。另外 lane 报告了两个工具建议，值得后续处理：
  `tools/extract-abi.py` 的 `R_STATIC`/`R_CONST_NAME` 应像 `R_FIELD` 一样接受 `(?:r#)?`（否则 C 的 `static char *in;`
  在 Rust 只能是 `r#in`，台账误报缺对象）；**`strip_comments` 先判 `"` 后判字符字面量**，`'"'` 这种字面量会把其后的
  注释当代码——本树 8 个 `.rs` 有该字面量，门因此**少报** token（lane 用自写的准确剥离器复跑，确认实际缺 token 集合
  就是它delivered的 14 对；工具修不修都能通过，但修了才不会被欠报蒙住）。
  实测：`--check` = `OK`；`build-tu lib-src/ebrowse.c` = `built`；手工 `rustc --emit=metadata` = exit 0；
  `check-mirror-dupes` `1 ratchet row / 0 failure`、`check-declarations` `13 sites (0 forbidden, 13 allowed literals)
  / 0 failures`、`check-verbatim` 0、`check-globals` `283 units / 0`；`port-inventory` 复算
  **590 files, done=476, wip=26, todo=88**。

- **3.127 抽取器的两处缺陷修掉：字符字面量先于字符串，`r#` 标识符进 `R_STATIC`/`R_CONST_NAME`（2026-10-04 第 100 轮第十批）**：
  两处都是 **ebrowse lane 在做行为差分时自己写准确剥离器、与工具对拍发现**的（3.126 记了它的报告）：
  ① **`strip_comments` 先判 `"` 后判字符字面量**——遇到 `'"'` 会把它当成"字符串开始"，一路吞到下一个 `"`，
  于是其后的**注释被当成代码**。后果有两面：可能藏住定义，也会往 Rust 侧灌进假 token，让"does not mention"
  门**少报**。修法：在判 `"` 之前先按 check-declarations 的同款启发式读字符字面量（`\` 转义、窗口 12 字符内
  找收尾 `'`；生命周期 `<'a>` 找不到收尾就按普通代码落回）。
  **修完立刻暴露出 4 个真实欠报**（此前这 4 个文件因为该缺陷而"绿"）：`src/lread.c` 3 条、`src/font.c` 4 条、
  `src/syntax.c` 2 条、`lib-src/make-docfile.c` 1 条——**全部是既有的"Rust 里不可能出现该 token"家族**
  （指定初始化器 `.ieee_nan = { .exponent … }`、C 的数组上界惯用式 `MAX (INT_STRLEN_BOUND …, 1 + DBL_MAX_10_EXP + 1)`
  被端口写成实测常量、`goto`/标签（2.2）、`UNINIT`（P-04/1.5）、`switch`/`case`）。逐条核实后补了 **10 行**
  `macro-body-notes.tsv`，四个文件重新回到 `--check` = `OK`，全树 `--all` = **484 done / 18 in progress / 0 failing**。
  ② **`R_STATIC`/`R_CONST_NAME` 不收 `r#` 前缀**（`R_FIELD` 一直收）：C 的 `static char *in;` 在 Rust 只能是
  `r#in`，工具会把该对象误判成"缺定义"。修法：两个正则都加 `(?:r#)?`（非捕获组，名字本身不带前缀，与 `R_FIELD` 同形）。
  验证：`python3 tools/extract-abi.py --all` exit 0、`484 done / 18 in progress / 0 failing`；
  四个受影响文件 `--check` 全 `OK`；`check-mirror-dupes` / `check-declarations` / `check-verbatim` / `check-globals` 与
  修前一致（`1 ratchet row / 0 failure`、`13 sites (0 forbidden) / 0 failures`、`0`、`283 units / 0`）。
  **教训（写进这里供后来者）**：门绿不等于门在**看**——`strip_comments` 的这一个顺序错误，让 4 个已竣工单元
  在一个家族上长期"免检"；这类"扫描器自身的盲点"只能靠**独立实现对拍**（lane 的自写剥离器）发现。

- **3.128 抽取器有了自己的回归用例：`extract-abi.py --selftest`（2026-10-04 第 100 轮第十一批）**：
  3.127 的教训是"门绿不等于门在**看**"——`strip_comments` 的一个判断顺序错误，让 4 个已竣工单元在一个
  token 家族上长期免检。于是给它补上自己的用例（照 `check-declarations.py --selftest` 的形制）：
  **9 条**，覆盖 `'"'` 字符字面量（就是那个 bug）、转义引号字符字面量、生命周期 `<'a>` 不被当成字面量、
  原始串 `r#"…"#`、字节串 `b"…"`、嵌套块注释、字符串里含未闭合注释、块注释藏定义、以及"字面量里的斜杠 + 转义
  引号的字符串"；比较时把空白折叠，只比语义文本。**回归有效性是实测的**：把 5e1e033 的旧 `strip_comments`
  单独抽出来跑同一批用例，**`'"'` 那条失败**（1/9 命中），其余 8 条通过——即这条用例真能咬住那个缺陷，
  而不会只是复述现在的行为。用法：`python3 tools/extract-abi.py --selftest`（9 cases, 0 failures）。
  验证：`--all` 读数与修前一致（**484 done / 18 in progress / 0 failing**，说明这次只加了用例、没有动判定逻辑）。

- **3.129 四个 gnulib 头镜像收口（`verify.h`/`limits.in.h`/`attribute.h`/`stat-time.h`）+ `process.c` 的 `E0428` 修掉（2026-10-04 第 100 轮第十二批）**：
  ① **`lib/verify.h`**（186 行镜像，26 条差异清 0）：`_GL_VERIFY_*`/`assume`/`static_assert` 家族落地。**跨切面收敛**：
  `femacs/src/include/lisp.rs` 原本自带 `assume`/`static_assert` 的 `macro_rules!`，现在按 C 的 lisp.h 44 位置
  `include!` 这份镜像（0.1-3 一个名字一个落点——改前全树有两处定义，charset 链一旦接上就是 `E0428`）。
  ② **`lib/limits.in.h`**（132 行）：C23 的 11 个宽度、`BOOL_*`、`SSIZE_MAX`/`SHRT_WIDTH`/`UCHAR_WIDTH` 等按冻结
  clang 实测值落地；`MB_LEN_MAX` 走**排除表**而不是 `pub const`——模板自己的兜底是 16，而冻结目标经系统
  `<limits.h>`（模板 C 40 的 `#include_next`）实测是 **6**，两个 `DEBUG_ASSERT` 与多字节缓冲区尺寸都按 6 走，
  所以只有"宏形状 + `excluded-macros.txt` 的 dead-text 行"能同时保住冻结值（集成者补行，实测即 `OK`）。
  ③ **`lib/attribute.h`**（300 行，32 条清 0）：`ATTRIBUTE_*` 家族两层次写法，私有 `_GL_ATTRIBUTE_*` 空展开、
  公开名转发，既有 `FALLTHROUGH!()` 用点（ebrowse.rs 3 处）照旧展开为空。
  ④ **`lib/stat-time.h`**（236 行）：`STAT_TIMESPEC!`/`STAT_TIMESPEC_OFFSETOF!` 按 `NUMNAME` 形状枚举 4/3 个 ident
  分支，死支用 port 局部 feature 名（`stat_time_*`）包起来并各配 `const _: () = assert! (!cfg! (…))`；
  `_GL_STAT_TIME_INLINE` 走排除表（单元 `lib/stat-time.c` 自己定义它，镜像落名会 `E0428`），
  `get_stat_birthtime` 的两条 `#if` 条件文本走 `macro-body-notes.tsv`。
  **收敛的消费者 10 个**（全是删用点副本/改走镜像）：`lib/include/regex_internal.rs`、`lib/regex_internal.rs`、
  `lib/regcomp.rs`、`lib/rawmemchr.rs`、`lib/getdelim.rs`、`lib/careadlinkat.rs`、`src/fileio.rs`、`src/indent.rs`、
  `lib-src/make-fingerprint.rs`、`src/include/lisp.rs`；代表消费者 `build-tu` 全 `built`
  （`lib/regex_internal.c`、`src/bignum.c`、`lib/stat-time.c`、`lib/mktime.c`、`lib/rawmemchr.c`、`lib/regcomp.c`、
  `lib/careadlinkat.c`、`lib-src/make-fingerprint.c`）。
  ⑤ **顺带修掉一个已提交态的 `E0428`**：`femacs/src/process.rs` 仍在 crate 根自带 `FD_SET`/`FD_CLR`/`FD_ZERO`
  的用点副本（注释还写着"镜像只有 `FD_ISSET`"——那已经过期，`lib/include/sys_select.rs` 现在三个都有），
  `build-tu src/process.c` 因此报三条 `E0428`，并会挡住 `gate-fast` 的建单元步骤。按 0.1-3 删掉三份副本后
  `built src/process.rs`。**这是 lane 报告里点名、由集成者修掉的跨单元缺陷**（`docs/ports/process.md` 未勾项为 0，
  所以门不会自己发现它——记录在案）。另已提醒 etags lane：它单元里的 `pub const CHAR_MAX` 与收口后的
  `limits.in.rs` 重复，须删除。
  实测：四个镜像 `--check` 全 `OK`；`check-mirror-dupes` `1 ratchet row / 0 failure`、`check-declarations`
  `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、`check-verbatim` 0、`check-globals` `283 units / 0`；
  `port-inventory` 复算 **590 files, done=480, wip=22, todo=88**。

- **3.130 `lib-src/etags.c` 整文件入树（9,068 行 / 126 函数全落地），以及 `cinventory` 的"跨行字面量"根因修复（2026-10-04 第 100 轮第十三批）**：
  ① **单元**：`femacs/lib-src/etags.rs`（1,787 → **9,068 行**，C 8,280）——`--check` = **`OK`**，台账
  **126/126 函数 + 29/29 宏 + 7 文件级对象** 全勾。31 个语言解析器全部在 **C 原位**落地（此前那条"必须前移到
  `lang_names` 之前"的偏离判断被证伪：Rust item 与顺序无关，0.1-12 只要求丢掉 C 的前向声明），`build-tu
  lib-src/etags.c` = `built`、手工 rustc = exit 0。
  ② **共享行 41 条**（集成者）：35 条 token 行（`switch`/`case`/`default`/`goto`/`FALLTHROUGH` 家族、
  `PRIdMAX`/`UNINIT`/`fopen_binary`/`UCHAR_MAX`/字面量下标 0/1/`assert`）+ 4 条**消息字面量函数**用 `*` 行
  （`print_help`/`print_language_names`/`print_version`/`main`——工具对 `*` 行的既有约定是"整个 body 的 token
  都是字面量自己的文本，家族计数按构造跳过"）+ 1 条 `found` 家族说明 + `line-drift-tolerance.tsv` 的
  `etags 0.11`（实测 +9.5%）。**顺带发现一处工具口径**：家族下限（`found` 3 次 vs 端口 0 次）是**token 行
  挡不住**的——只有 `*` 行会让函数循环 `continue`，这条写进注释供后来者照做。
  ③ **根因修复（`tools/cinventory.py`）**：`globals_` 逐行用 `strip_strings`（一条只认"本行内完整字符串"的
  正则）剥字面量，于是 **C 的反斜杠续行字面量**（翻译阶段 2 的拼接）从第二行起就"看不见开头引号"，
  `Forth_help` 那句里的 `code/create/defer/value/variable` 被当成 5 个文件级对象。改成读 `Lines.scan_rows`
  ——`Lines` 早就为花括号扫描建好的、**跨行**的字面量消隐视图。修完 `--check` 里那 5 条 `missing file-scope
  object` 直接消失，C 本来就没声明这些名字。**这是"用排除表记账"与"修工具"之间选了后者**。
  ④ **证据边界（如实记录）**：etags **没有**行为差分（不像 ebrowse 那样有运行期对拍）；lane 做到的是逐处对读
  + 编绿 + 文本/结构门，唯一由对读抓出的真 bug 是 `hash` 的 `default:`/`case 2:` fallthrough 顺序（原先让字面量
  分支不可达）。若要与 ebrowse 同级，需另跑一轮 `etags` 输出对拍。
  实测：`--all` **489 done / 13 in progress / 0 failing**；`check-mirror-dupes` `1 ratchet row / 0 failure`、
  `check-declarations` `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、`check-verbatim` 0、
  `check-globals` `283 units / 0`；`port-inventory` 复算 **590 files, done=482, wip=20, todo=88**。

- **3.131 七个 gnulib 头镜像收口（sys_types/time/fcntl/gettext + unistd/sys_stat/stdio-impl），以及只有"编译"能发现的三个 `E0428`（2026-10-04 第 100 轮第十四批）**：
  ① **`sys_types.in.h`**（135 行镜像，16→0）、**`time.in.h`**（233，20→0）、**`fcntl.in.h`**（350，68→0）、
  **`gettext.h`**（351，45→0）四个按冻结生成头（`build/emacs-ref/lib/{…}`）落地：模板自身定义的与系统头提供的两半
  都写全，死臂按 P-51 以 `#[cfg (feature = "…")]` 命名 C 条件承载，取值全部用冻结 clang 实测并写进台账。
  形状上有一处**依赖"first-wins"的写法**需要记录：`fcntl.in.h`/`time.in.h`/`gettext.h` 里"系统已定义、模板的
  `#ifndef` 臂是死文本"的名字，写成**同名两条臂**（C 的臂在前、`#[cfg]` 门控；冻结值在后、不门控）——
  抽取器读"第一条定义"因而比对的是 C 的臂，rustc 只编译未门控那条，运行期是冻结值；`acl-internal.rs` 的
  `MODE_INSIDE_ACL` 是同一先例。若将来评审不接受这个形状，替代是 17 行 `excluded-macros.txt` + 约 20 个
  调用点改 `NAME!()`。
  ② **`unistd.in.h`**（330，68→0）、**`sys_stat.in.h`**（290，40→0）、**`stdio-impl.h`**（251，40→0）同理；
  unistd 的两个 `_GL_*_INLINE` 走排除表（两个单元自己 define 限定名，镜像落名会 `E0428`）。
  ③ **三个 `E0428` 全部是"编译"发现的、不是静态门发现的**（这条教训由 unistd lane 明确写进台账）：
  镜像新带的名字与**已竣工单元**的用点副本重复——`src/callproc.rs`（`*_FILENO`/`environ`）、
  `lib-src/emacsclient.rs`（`STDOUT_FILENO`/`environ`）、`lib/faccessat.rs`（`_GL_INCLUDING_UNISTD_H`），
  外加本批由集成者修掉的 `lib-src/etags.rs` 的 `pub const STDOUT_FILENO`。**`--all` 与四个 check 门都不编译**，
  所以这类缺陷只有 `xtask gate-fast`（建单元段）或"逐消费者 build-tu"能看见——本轮先把 `gate-fast` 从
  "卡在 inventory"推到"能跑到建单元"（3.117/3.129 的收益），它当场就抓出了 emacsclient 与 etags 两条。
  ④ **keyboard lane 的一处语义修正（值得记）**：前一 lane 把 C 4108-4110 那段
  `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` 里的 `gobble_input ();` 当成死文本（写成
  `#[cfg (feature = "usable_sigio_or_usable_sigpoll")]`），而冻结表里 **`USABLE_SIGIO 1`**
  （macros.tsv 3829，clang 实测该条件为真）——那会让一次**活调用被静默丢掉**。本 lane 改成活文本并在台账注明，
  同一条件在 C 8419-8445、8515-8519 两处也一律按活文本处理。另：keyboard 覆盖 **C 96-8574**（8,030 → 9,355 行，
  台账 137 → 103 未勾），scaffold 14 → 9；`--check` 的 "does not mention" 已由集成者补 3 行清零。
  验证：七个镜像 `--check` 全 `OK`；代表消费者 `build-tu` 全 `built`（etags/emacsclient/keyboard/xdisp/callproc/
  sysdep/fcntl/timefns/regcomp/faccessat/term）；`check-mirror-dupes` `1 ratchet row / 0 failure`、
  `check-declarations` `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、`check-verbatim` 0、
  `check-globals` `283 units / 0`；`port-inventory` 复算 **590 files, done=489, wip=13, todo=88**。

- **3.132 `lib/stdint.in.h` 镜像收口（136 个宏名 + 9 个常量），`INTMAX_WIDTH`/`UINTMAX_WIDTH` 归位（2026-10-04 第 100 轮第十五批）**：
  ① **镜像**（`femacs/lib/include/stdint.in.rs`，**343 行**）：135 条差异清零。做法与上一批同族——gnulib 模板的
  两半都写全（模板自身定义 + 系统 `<stdint.h>` 经 `#include_next` 提供的那半），**每个取值/类型宽度用冻结 clang
  实测**（`_Generic` 类型 + `__INT_FAST16_TYPE__` 一族），实测命令与结果写在台账 Evidence；镜像的探针把 88 个
  实测值/宽度/类型写成 `const _: () = assert!(…)` 并 exit 0。
  ② **31 条 `excluded-macros.txt` 行**（集成者补）：**26 条类型重命名**（`int8_t`…`uintptr_t`、`int_least*`、
  `int_fast*`——C 的 `# define intN_t gl_intN_t` 在冻结目标是死文本，系统头供类型，端口写 `pub type … = core::ffi::…`，
  门名的 item 扫描读不到 `pub type`）+ **5 条 dead-text 值**（`INT_FAST16_MIN/MAX`、`UINT_FAST16_MAX`、
  `SIG_ATOMIC_MAX`、`WCHAR_MAX`——模板的整套 limit 段在 `#if !@HAVE_C99_STDINT_H@` 下、冻结目标为假，值是系统头的
  实测值；后两条的体还是 `@PLACEHOLDER@` 文本，Rust 无拼法）。**裁决记录**：26 个类型名选"`pub type` + 排除行"
  而非 P-51 双分支（P-66 对 gnulib 模板的规定就是"镜像冻结文本 + 台账逐段列未镜像部分"，`MB_LEN_MAX` 是同型先例）；
  `INT_FAST16_MIN/MAX` 类的"模板死臂 `INT_FAST32_*` vs 冻结 `short`"也按冻结值写常量。
  ③ **归属修正**：C 把 `INTMAX_WIDTH`/`UINTMAX_WIDTH` 定义在 `lib/stdint.in.h` 725-726，`limits.in.rs` 只是
  无家可归时的临时携带者——本轮把两条常量移进 `stdint.in.rs`、从 `limits.in.rs` 删除（两个都仍 `--check` = `OK`；
  用到它们的 `src/bignum.rs`/`src/print.rs` 本来就包含 stdint 镜像），并把 limits 台账里那句"本镜像携带这两个名字"
  的 Evidence 改成"已移出"的说明。同批收敛 `src/alloc.rs`（`UINTPTR_MAX` 副本）与 `src/pdumper.rs`
  （`UINT_FAST32_WIDTH`/`INTPTR_MAX`/`int_least32_t`/`INT_LEAST32_*` 副本）；`limits.in.rs` 的
  `_GL_INTEGER_WIDTH` 签名**未改**（仍是 `(isize, isize) -> usize`，因为 `src/sort.rs`/`src/buffer.rs` 按 `usize` 用它）。
  ④ **跨镜像隐患（记录不修）**：`nt/inc/include/stdint.rs` 也定义了 `INT32_MAX`/`UINT64_MAX`/`SIZE_MAX`/`intptr_t`
  等一批名字；今天没有任何单元同时 include 两者，但将来某个 `nt/` 单元这样做会 `E0428`
  （`docs/ports/nt-inc-stdint.md` 的 Decisions 已留待办）。
  实测：两个镜像 `--check` 全 `OK`；消费者 `build-tu` 全 `built`（`lib/byteswap.c`、`lib/regcomp.c`、`src/bignum.c`、
  `src/fileio.c`、`src/pdumper.c`、`src/alloc.c`）；`check-mirror-dupes` `1 ratchet row / 0 failure`、
  `check-declarations` `13 sites (0 forbidden, 13 allowed literals) / 0 failures`、`check-verbatim` 0、
  `check-globals` `283 units / 0`；`port-inventory` 复算 **590 files, done=488, wip=14, todo=88**。

- **3.133 共享表被 lane 的"还原快照"铲掉两行：`excluded-macros.txt` 的 unistd 两行补回（2026-10-04 第 100 轮第十六批）**：
  3.131 里我（集成者）往 `docs/ports/excluded-macros.txt` 追加了 unistd 的两条 `*_INLINE` 行，随后 stdint lane 在
  收尾时按"共享文件属于集成者"的规则把自己动过的版本**还原成它开工前的快照**（sha256 与它开工时一致），于是这两行
  **被静默铲掉**——`--check emacs/lib/unistd.in.h` 重新报 `missing macro _GL_UNISTD_INLINE` /
  `_GL_GETPAGESIZE_INLINE`，而 unistd 台账的框已全勾，`--all` 因此从 `0 failing` 变成 **`1 failing`**
  （d43c5b3 的提交态里就是缺这两行）。本轮把两行按原文补回并复测 `OK`；`--all` 回到 0 failing。
  **教训（写进这里，供后续 lane 照做）**：① 共享 tsv（`excluded-macros.txt`/`macro-body-notes.tsv`/
  `line-drift-tolerance.tsv`/`*pending*.txt`）**只许追加**，任何"还原成我的快照"的动作都会把别人并发写入的行一起铲掉；
  ② 集成者在每个 lane 交回后要**重跑一次受影响文件的 `--check`**（而不是只看 lane 报告里的读数）——本次就是靠
  `--all` 的 FAIL 才发现的。

- **3.134 版本声明的全树复核：任何已写单元里都不再声明产品名（2026-10-04 用户指令，本轮逐项复算）**：
  用户当轮的两句话（`版本声明不要再说是gnu emacs了，我们这样说可能会有法律风险` + 38 秒后的
  `把之前的代码中的这种声明也去掉`）是**同一条指令的前后两半**：不要只对新写的文件生效，**已写进树的代码**
  也要清。当前树的三段处置全部落在这条指令上，本轮把每一段都重新验了一遍：
  ① **版本行的括注**（`@ fd7afced403 (GNU Emacs 32.0.50)`）**231 个文件**一次性去名，只留修订号
  （`2ee1d1d` + 收尾空格的 `4810bb0`，3.36 的机械改写）；
  ② **参照文件自带的版本/版权声明块**（C 文件开头那段"本文件属于某产品"的整块）**268 块不随复刻携带**
  （`fd9cf89`）——它是对**本仓库**的虚假陈述，不是被复制的语义；
  ③ **标题注释里的产品名**与**复述"许可块怎么处理"的句子**并入 `tools/check-declarations.py` 的
  `title`/`recital` 两类，随 `decl`/`version` 一起在**每轮**判。
  本轮实测（都在 `ef75d4d` 工作树上）：`check-declarations.py` → **13 declaration sites (0 forbidden,
  13 allowed literals), 0 failures**；`--selftest` → **22 cases, 0 failures**；对 `femacs/src`、`femacs/lib`、
  `femacs/lib-src`、`femacs/exec`、`femacs/lwlib`、`femacs/oldXMenu`、`femacs/nt`、`femacs/msdos`、`femacs/xtask`
  的定向扫描（`part of GNU Emacs` / `GNU Emacs is free software` / `GNU Emacs is distributed` /
  `along with GNU Emacs` / `Free Software Foundation` / `Copyright (C)` / `@ <rev> (` 产品括注）**零命中**。
  **余下的 13 处全是 C 自己的字符串字面量**（`emacs.rs` 的 `PACKAGE_NAME`/`COPYRIGHT`/`RCS_Id`、
  `etags.rs`/`ebrowse.rs` 的 `COPYRIGHT` 与版本横幅、四条 `callproc.c`/`emacs.c` 的 docstring），它们是被
  移植的**语义**（横幅要打印、docstring 门要逐字比对），逐条理由在 `docs/baseline/declarations-allowlist.tsv`；
  照抄树（`lisp`/`test`/`etc`/`doc`/`leim`/`admin`，`check-verbatim` 6 trees/4607 files）与 `emacs/` 参照树
  不在范围内（前者是逐字节复制，后者是只读参照）；`docs/` 的中性历史叙述是决定记录，按 3.90/3.100 的定义留下。

- **3.135 `lib/arg-nonnull.h` 的镜像落地（2026-10-04，本轮集成者自己收的小单元）**：这是"待开工 88 → 87"里
  最干净的一个——整个头只有一条宏，而且不可决定的条件数为 0。
  ① **宏 `_GL_ARG_NONNULL` 走既有排除行，不在台账里列名**：`docs/ports/excluded-macros.txt` 45 行早就记着
  `_GL_ARG_NONNULL  syntax  attribute (lib/arg-nonnull.h); no Rust counterpart`——属性是拼写不是语义
  （PORTING.md 0.3）。冻结目标的 clang 让 C 21 的 `defined __clang__` 项为真，因此活的是 C 22 的
  `__attribute__ ((__nonnull__ params))`、C 24 的空臂是死文本；**但两臂都产不出 Rust 声明**，所以既不需要
  P-51 的 `#[cfg]` 载体，也不存在"选错臂"的风险。`tools/extract-abi.py lib/arg-nonnull.h` 生成的台账因此是
  **0 函数 / 0 宏 / 0 类型 / 0 全局**——这正是"被排除的宏由清单门计数、不由台账计数"的既有口径。
  ② **C 20 的 `#ifndef _GL_ARG_NONNULL` 是整文件名字守卫**，Rust 无对应物（`xtask` 的包含图去重做同样的事，
  PORTING.md 0.1-4）。
  ③ **镜像刻意不被任何单元 include**：参照里也没有——`arg-nonnull.h` 是 gnulib 的 snippet 模块
  （`emacs/lib/gnulib.mk.in` 3159-3171 安装），`src/`、`lib/` 里搜不到 `#include "arg-nonnull.h"`，宏是靠
  `system.h` 的聚合文本到达使用者的；端口按 0.1-4/P-61 由聚合体直接取名，所以这个镜像就是一条"处置记录"。
  证据：`--check emacs/lib/arg-nonnull.h femacs/lib/include/arg-nonnull.rs` → **OK**（无 problem 行）；
  `check-cond-eval.py lib/arg-nonnull.h` → `2 conditions, 0 unknown to the evaluator, 0 disagreements`；
  消费者侧 `build-tu lib/getdelim.c` → **built**（`getdelim.rs` 正是自己拼这个被丢掉的属性的两个单元之一）；
  `check-globals` 283 units / 0、`check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0 failures、
  `check-mirror-dupes` 0 failures、`check-verbatim` 6 trees/4607 files/0；`port-inventory` 复算
  **503 个已有目标 = 489 done / 14 wip，待开工 87**（本轮由 488/14/88 推进一格）。

- **3.136 声明门扩到 batch F 的非 C 平台树 `femacs/platform`，以及"剩余工作"的实测口径（2026-10-04 本轮）**：
  ① **门先于文件**：3.100 的教训是"没有任何门扫描的树里，`lib-src/hexl.rs` 照样复述了参照的开头块"。
  `docs/port-map.md` 613-663 行给 batch F 的非 C 目标定的落点是 `femacs/platform/macos/*.swift`、
  `femacs/platform/android/kotlin/*.kt`、`femacs/platform/haiku/*.cc`，而**这棵树今天还不存在**
  （`find femacs -name '*.swift'` = 0、`*.kt` = 0、`femacs/platform` 无此目录）。本轮把
  `tools/check-declarations.py` 的 `SCAN_DIRS` 加进 `femacs/platform`（并写进它的模块文档与注释），
  这样**写进那里的第一个文件**就受 decl/version/title/recital 四类约束，而不是等第一轮评审才发现。
  实测：`--selftest` 22 cases / 0 failures；扫描读数不变（`femacs/platform` 为空 ⇒ 仍是
  13 sites (0 forbidden, 13 allowed literals) / 0 failures）。
  ② **剩余工作的实测口径**（本轮用工具现场测，不引用旧读数）：
  - C 侧 590 文件的账：`port-inventory` 复算 **503 个已有目标 = 489 done / 14 wip，待开工 87**；
    87 个**全部是头文件镜像**（31,374 行 C、219 个函数），其中 62 个是平台/图形头
    （`src/android*`、`src/haiku*`、`src/w32*`、`src/pgtk*`、`src/gtkutil.h`、`src/xwidget.h`…），
    这批的冻结分支可判性已逐条实测：`androidgui.h`/`haiku_support.h`/`pgtkterm.h`/`androidterm.h`/
    `haikuterm.h`/`gtkutil.h`/`xwidget.h`/`treesit.h`/`android.h`/`emacs-icon.h`/`lib/diffseq.h` 全部
    `0 unknown, 0 disagreements`（可开工），而 `lib/cdefs.h`(30 条不一致)、`lib/signal.in.h`(45)、
    `lib/dirent.in.h`(39)、`src/conf_post.h`(3+4 unknown)、`lib/intprops.h`(2)、`src/w32term.h`(1)、
    `src/w32.h`(1) 需要 P-51/P-66 的承载或先修判据，**不是一轮能收的**。
  - **非 C 平台树 53 文件 / 49,759 行，落地数为 0**：`src/*.m`→Swift 9 文件 / 26,517 行
    （`nsterm.m` 单件 11,811 行，与 `xdisp.c` 同级的另一个长杆）、`java/`→Kotlin 40 / 15,157、
    `haiku_*.cc`→C++ 4 / 8,085。本轮据此开了第一条 Swift lane（先做 `src/nsimage.m` 658 行，
    要求它交回 ABI 两半清单：本文件**导出**的 C 符号与**调用**的 C 符号），并要求它把
    "Swift 侧用声明式 extern（`@_silgen_name`）对接 Rust 提供的 C ABI、`@_cdecl` 保持导出名不变"
    的结论写进 `docs/ports/nsimage.md` 的 `## ABI` 一节——**这条规矩定了，剩下 8 个 Swift 文件才分片**。
  ③ **在飞 lane（本轮结束时）**：keyboard（Pascal，从 C 8587 起）、xdisp（Beauvoir，从 C 4667 起）、
  mirror A（Dewey：`haikugui.h`/`haiku_support.h`/`androidgui.h`/`android.h`）、
  mirror B（Tesla：`androidterm.h`/`android-asset.h`/`pgtkterm.h`/`gtkutil.h`）、Swift（Mill：`nsimage.m`）。

- **3.137 `src/treesit.h` 镜像落地：整段死文本 + 一条活声明的用点清理（2026-10-04 本轮）**：
  ① **可判性先测**：`HAVE_TREE_SITTER` 在冻结表里是 `undef`（`docs/baseline/config/darwin-arm64.macros.tsv`
  1656），`src/treesit.h` 又在 `cond-eval-verified.txt`（392 行），`check-cond-eval.py` 报
  `2 conditions, 0 unknown, 0 disagreements` ⇒ C 25-253 的 `#ifdef HAVE_TREE_SITTER` 整段是死文本
  （三个伪向量记录 `Lisp_TS_Parser`/`Lisp_TS_Node`/`Lisp_TS_Query` 连它们的 `TSParser`/`TSTree`/`TSInput`/
  `TSNode`/`TSQuery`/`TSQueryCursor` 成员与那串 `bool` 标志、九个 inline 存取器/检查器、`ts_linecol` 的
  四处使用、`TREESIT_EMPTY_LINECOL`、十个 `extern`），按 P-08 **记账不转写**，台账 `docs/ports/include-treesit.md`；
  它与 `docs/ports/treesit.md`（单元侧）对同一区间的边界必须一致，这正是两次 `--check` 量出来的东西。
  ② **留下的那一条是活的**：C 255 的 `extern void syms_of_treesit (void);` 在守卫之外，因为
  `src/emacs.c` 147-149 是**无守卫**包含（注释原文："We don't guard this with HAVE_TREE_SITTER because
  treesit.o is always compiled (to provide treesit-available-p)"），C 2347 也就无条件调用它。镜像因此只声明这个名字。
  ③ **用点清理（0.1-4）**：`emacs.rs` 把"headers with no mirror at all"块里的 `syms_of_treesit` 副本删掉
  （该块现在只剩 `syms_of_xwidget`），改成 C 149 位置的 `include!("include/treesit.rs")`；
  `src/pdumper.c` 47 也是无守卫包含 ⇒ `pdumper.rs` 同位置加 include。三处"`treesit.h` has no mirror"
  的注释（`pdumper.rs` 两处、`casefiddle.rs` 563）改成与实际一致。
  ④ **顺带纠正三个写错的 C 行号**（`pdumper.rs` 的"未镜像"清单）：`<sys/mman.h>` 27（原写 35）、
  `<stdarg.h>` 24（原写 32）、`<sys/param.h>` 28（原写 39）、`treesit.h` 47（原写 53）——都用
  `rg -n '#include …' emacs/src/pdumper.c` 现测，`emacs.rs` 的 97/103/151/152 四个同类行号同时复核为**正确**，
  说明偏差只在这一处，不是系统性口径问题。
  ⑤ 证据：`--check emacs/src/treesit.h femacs/src/include/treesit.rs` → **OK**；
  `build-tu src/emacs.c` 与 `build-tu src/pdumper.c` 都 **built**（它们是仅有的两个无守卫包含者）；
  `check-globals` 283 units / 0、`check-declarations` 13 sites (0 forbidden) / 0、`check-mirror-dupes` 0、
  `check-verbatim` 6 trees/4607 files/0；`port-inventory` 复算 **504 = 490 done / 14 wip，待开工 86**
  （由 489/14/87 推进一格）；`extract-abi.py --all`（本轮）→ **504 ported files: 497 done, 7 in progress,
  0 failing**。

- **3.138 xdisp 本轮（C 4667-4980）由集成者复核通过，以及"两个巨型在飞单元不进 git"这条纪律的书面化（2026-10-04）**：
  ① **独立复核**（不采信 lane 报告）：`--check emacs/src/xdisp.c femacs/src/xdisp.rs` 输出里 312 条 problem
  **没有一条**落在本轮新增的四个名字上（`handle_fontified_prop`/`face_at_pos`/`handle_face_prop`/
  `underlying_face_id` 在输出里 **0 命中**，实测用 `rg -c` 数出来），全部是 C 4980 之后的"未到达"条目
  （234 个函数、20 个宏、12 个常量、17 个文件级对象、13 个 defsubr、11 个 subr，加上仍未定义的 5 个跨单元入口
  `reseat_at_previous_visible_line_start`/`move_it_to`/`move_it_by_lines`/`add_to_log`/`update_redisplay_ticks`）
  与一行 86.2% 的漂移（未完成单元的预期结论）；`build-tu src/xdisp.c` → **built**；台账 `docs/ports/xdisp.md`
  的 Progress 段记 **C 468-4980**、131/415 勾、`SCAFFOLD` 24 → 22。下一条活定义 = **C 4989
  `face_before_or_after_it_pos`**（签名行 C 4988，文档注释 C 4983-4986）。
  ② **lane 自己交回的两条交接提醒**（下一轮必须照做）：`it_props[]` 表把 handler 的地址交出去，所以表里余下三个
  （`handle_display_prop`、`handle_invisible_prop`、`handle_composition_prop`）的定义必须是
  `unsafe extern "C" fn`（PORTING 0.2/P-52 的"地址被交出"分支）；`handle_overlay_change` 与
  `face_before_or_after_it_pos` 虽不在表里，scaffold 已按 C ABI 声明，定义时必须删掉对应声明否则
  `E0428`/`E0308`。
  ③ **纪律（唯一一处"工作树领先于 git"）**：两个巨型在飞单元 `femacs/src/xdisp.rs`（5,493 行）与
  `femacs/src/keyboard.rs`（9,355 行）以及它们的台账**不进 git**，`git status` 里恒为 `??`。理由是仓库既有
  实践——单元**闭合时**一次 `port(NAME): …` 提交（先例：`femacs/src/window.rs` 的 `d08a521`
  "window: the unit closes - C 1-9740 covered…"，此后只有修复型提交），而这两个单元的 `--check` 在 C 侧还没搬完
  之前**必然**报 FAIL（本轮实测 312 条 / 36.3% 漂移），所以它们的提交点就是闭合那天。这不是遗漏：本节即是
  该状态的书面记录，任何人看到 `git status` 的 `??` 都应在这里找到解释。
  ④ 本轮 lane 面：mirror A/B 已经开始落盘（`femacs/src/include/androidterm.rs` 已出现，尚未收口），
  Swift lane 在 `src/nsimage.m` 的 ABI 清单上，keyboard lane 仍在原边界内推进。

- **3.139 对账抓到的唯一一处"文件名不在 map 上"：`getopt-cdefs` 少了个 `.in`，已 `git mv` 回 map 的名字（2026-10-04）**：
  ① **发现方式（双向对账，不是抽样）**：把 `docs/port-inventory.md` 的 target 列（593 行有目标）与工作树里全部
  `femacs/**/*.rs`（541 个）做**两个方向**的集合差。反向（有文件、没有行指向它）命中 36 个，其中 **35 个是系统头
  镜像**——`<stdio.h>`/`<stdlib.h>`/`<string.h>`/`<ctype.h>`/`<dirent.h>`/`<errno.h>`/`<signal.h>`/`<math.h>`/
  `<pwd.h>`/`<grp.h>`/`<termios.h>`/`<langinfo.h>`/`<sys/time.h>`/`<sys/select.h>`/`<sys/resource.h>`/
  `<sys/mount.h>`/`<sys/utsname.h>`/`<sys/sysctl.h>`/`<sys/acl.h>`/`<stdbit.h>`/`<stdcountof.h>`/`<float.h>`/
  `<a.out.h>` 等，它们是 P-26/P-50 的"冻结目标系统头"镜像，参照里没有对应的文件，**本来就不该有行**；
  **只有 1 个是真错**：`femacs/lib/include/getopt-cdefs.rs` 与它的台账 `docs/ports/include-getopt-cdefs.md`。
  ② **为什么它必须修**：`docs/port-map.md` 给 `lib/getopt-cdefs.in.h` 定的目标是
  `femacs/lib/include/getopt-cdefs.in.rs`（与 `getopt.in.rs`/`limits.in.rs`/`stdint.in.rs`/`sys_types.in.rs`/
  `time.in.rs`/`fcntl.in.rs`/`unistd.in.rs`/`sys_stat.in.rs` 同一形状），而台账名同规则是
  `docs/ports/include-<stem>.md`。少了 `.in` 的后果不是编译错，而是**清单永远读 `todo`**——文件早在树里、`--check`
  也是 OK 的，DoD 第 1 条（590 条全 `ported`）却永远不可能满足。这正是"生成物 vs 工作树"双向对账才能看到的缺陷：
  前 100 多轮只单向看 `todo` 行，就把它读成了"还没开工"。
  ③ **修法**：`git mv` 两个文件到 map 的名字（保留历史：`R` 状态），同步三处引用——`include/getopt.in.rs` 18 的
  `include!("getopt-cdefs.in.rs")`、`include/getopt-core.rs` 4 的注释、`docs/ports/getopt.md` 71 的引用；台账的
  Decisions 记下改名与理由，"WIP mirror" 的措辞改成"处置记录、无条目"（与 `arg-nonnull.rs` 同形：四个拼写宏
  `__BEGIN_DECLS`/`__END_DECLS`/`__GNUC_PREREQ`/`__THROW` 无 Rust 条目，`HAVE_SYS_CDEFS_H` 未定义使
  `#include <sys/cdefs.h>` 是死文本）。
  ④ 证据：`--check emacs/lib/getopt-cdefs.in.h femacs/lib/include/getopt-cdefs.in.rs` → **OK**；
  消费者两侧 `build-tu lib/getopt.c` 与 `build-tu lib-src/make-fingerprint.c` 都 **built**；
  `check-declarations` 13 sites (0 forbidden) / 0、`check-mirror-dupes` 0 failures、`check-globals` 283 units / 0、
  `check-verbatim` 6 trees/4607 files/0；`port-inventory` 复算 **508 = 494 done / 14 wip，待开工 82**
  （该行由 `todo` 直接跳到 `done`；总数同时含两条镜像 lane 已落盘的在飞文件）。
  ⑤ **留给后续的规矩**：每轮收尾时把 target 列与工作树做一次双向对账（本次是 541 个文件里唯一一处），
  比只盯 `todo` 行可靠。

- **3.140 把 3.139 的对账做成常驻门：`tools/check-target-names.py`（2026-10-04，第五个 check 门）**：
  ① **两条规则**：**R1** 每个 `femacs/**/*.rs` 必须要么是 `docs/port-inventory.md` 某行的 target，要么在首行**自我声明**
  不是参照文件的复刻——系统头镜像（`<stdio.h>`、`<errno.h>`、`<sys/select.h>`… 26 个）、三个生成物
  （`config.rs`/`globals.rs`/`buildobj.rs`）、`gnulib_config.rs`、照抄树里的 4 个 `.rs`、以及 `xtask`；
  **R2** 首行写了 `// femacs: port|mirror of X @ rev` 的文件，map 必须把 X 送到**这条路径**——这正是
  3.139 手改的那一类（`getopt-cdefs.rs` 声明自己是 `lib/getopt-cdefs.in.h` 的镜像，map 却要
  `getopt-cdefs.in.rs`）。
  ② **为什么值得常驻**：这类缺陷对编译、`--check`、四个既有门**全部隐形**——文件在树里、`--check` 是 OK 的，
  只有清单那一行永远读 `todo`，而 DoD 第 1 条（590 条全 `ported`）就卡在那里。R1+R2 把"文件说自己是复刻的谁"
  与"map 认为它该叫什么"直接对上。
  ③ 实测：`--selftest` **8 cases / 0 failures**（含 `getopt-cdefs` 的真实首行 → `R1`、以及 `alloc.rs` 报错参照
  → `R2` 两个负例）；在全树上跑 **544 个 `.rs`、593 个 map target、0 finding**。
  ④ **每轮的 check 门因此由四个变五个**：`check-globals` / `check-declarations` / `check-mirror-dupes` /
  `check-verbatim` / **`check-target-names`**。

- **3.141 mirror lane A/B 的前四个镜像单元入库：`androidterm.h`、`gtkutil.h`、`pgtkterm.h`、`haikugui.h`（2026-10-04）**：
  ① **复核（集成者自己跑，不采信 lane 报告）**：四对 `--check` 全 **OK**——`emacs/src/androidterm.h` +
  `include/androidterm.rs`（26,017 字节的镜像，496 行参照）、`gtkutil.h`、`pgtkterm.h`（681 行参照、1 个函数、
  46 个宏、169 个字段）、`haikugui.h`（205 行、35 个宏、19 个常量、4 个字段）；四个台账都是 **0 未勾**、
  `## Decisions` 与 `## Evidence` 齐备（含 lane 自己的编译证明与"最接近消费者"的 `build-tu` 冒烟）。
  ② **五个门全绿**：`check-globals` 283 units / 0、`check-declarations` 13 sites (0 forbidden, 13 allowed
  literals) / 0、`check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row / 0 failure、
  `check-verbatim` 6 trees / 4607 files / 0、**`check-target-names` 544 个 `.rs` / 593 个 target / 0 finding**。
  ③ **提交态自洽性的一处说明**：`androidterm`/`gtkutil`/`pgtkterm` 三行在上一批 `docs/port-inventory.md`
  的提交（`105c2b6`）里就已经被计成 `done`——生成器按**工作区**统计（含尚未提交的在飞文件，这是它能当进度账
  的原因），所以本轮把它们真正入库后，提交态与清单才对得上；本次唯一新翻的一行是 `src/haikugui.h`
  （`todo` → `done`）。复算 **509 = 495 done / 14 wip，待开工 81**。
  ④ lane 在 `haikugui` 台账里记下的处置值得留档：`XCharStruct` 一类的名字与 `dispextern.rs` 的别名冲突时，
  选择"照参照逐声明携带 + 在台账写明"，而不是发明 `pub mod` 包装（`macro_rules!` 是文本作用域，包装会把宏名
  藏起来，比冲突更糟）。

- **3.142 `src/emacs-icon.h` 镜像落地：死的一半（XPM）记账、活的一半（XBM）逐字节携带（2026-10-04）**：
  ① **唯一条件可判且为假**：`HAVE_XPM`/`HAVE_X_WINDOWS`/`USE_GTK` 在冻结表里全 `undef`（1708/1720/3861），
  `check-cond-eval.py src/emacs-icon.h` 报 `1 conditions, 0 unknown, 0 disagreements`，所以 C 27-318
  （`gnu_xpm_bits` 的 XPM 数组连 255 色表）是死文本，记账不转写（P-08）。参照自己在 C 19-20 的注释就是这段
  守卫的来由：GTK 端口用 gdk 画图标，所以"能编出 XPM 支持"并不是它的前提。
  ② **活的是守卫之下那一半**：C 321-322 的两个尺寸宏（body 都是 `50`，哈希同 `e1822db470e6`）与
  `gnu_xbm_bits`。C 把它写成 `static unsigned char[]`——内部链接、每个包含者一份——所以端口保持同一拼法、
  长度按 C 自己的初始化列表量成 **348**，并且**不加 `#[no_mangle]`**：编译证明里的 `nm -g` 打出的是
  mangled 名字（`D __RNvCs…gnu_xbm_bits`），正是 C 文件级 `static` 的样子。
  ③ **这个头没有 include guard**（C 从开头注释直接进第一个 `#if`）——照参照"没有"，不发明一个。
  ④ **没有能编译的消费者**：它唯一的包含者是 `src/xterm.c`（X11 后端，在本目标是空壳单元，status 3.31），
  所以编译证据用独立 crate 的 `rustc --edition 2024 --crate-type=lib --emit=obj`（exit 0）。
  ⑤ 证据：`--check emacs/src/emacs-icon.h femacs/src/include/emacs-icon.rs` → **OK**；
  `check-declarations` 13 sites (0 forbidden) / 0、`check-mirror-dupes` 1 duplicate / 1 ratchet / 0 failure、
  `check-globals` 283 units / 0、`check-verbatim` 6 trees/4607 files/0、**`check-target-names` 545 `.rs` / 593
  targets / 0 finding**；清单复算 **510 = 496 done / 14 wip，待开工 80**。

- **3.143 抽取器的 `globals_` 盲点（多词类型说明符）修掉，它当场暴露 5 个真的镜像缺口（2026-10-04）**：
  ① **bug 与最小复现**（mirror lane B 在对读 `src/androidterm.h` 时发现，`current_menu_serial` 那个
  `extern unsigned int` 对象在台账里根本不存在）：`GLOBAL_LINE` 的类型组是惰性的，
  `extern unsigned int a_one;` 被切成 `type="extern unsigned"` / `decls="int a_one"`，`DECLARATOR` 随即拒绝，
  于是对象**被静默丢弃**——所有 `unsigned int`/`unsigned long`/`long long`/`unsigned short` 形状的文件级对象
  都不在门比较的 C 侧里。修法是"把说明符一个词一个词挪过边界，直到每个 declarator 都能解析"；
  另加一条实测出来的抑制：C 把属性写在 declarator **之后**（`void *lisp_malloc_loser EXTERNALLY_VISIBLE;`），
  扫描 590 个参照文件里所有"名字长得像属性"的对象共 17 个，只有 `EXTERNALLY_VISIBLE` 是属性而不是对象，
  所以它进 `TRAILING_ATTRIBUTES`（不猜、用实测集合）。
  ② **回归用例**：`tools/cinventory.py` 新增 `--selftest`（5 例：多词说明符、尾属性、普通形状不回归、
  函数声明不是对象、`Lisp_Object const *staticvec[NSTATICS]`），实测 **5 cases / 0 failures**。
  ③ **它暴露的 5 个缺口全是真的**（`--all` 由 0 failing 变 2 failing）：`src/keyboard.h` 的 `timers_run`(471)、
  `DEV_TTY`(531)，`src/lisp.h` 的 `staticvec`(3922)、`hashtest_eq`(4260)、`emacs_wd`(5131)——
  对照各处定义点（keyboard.c 340/92-95、alloc.c 503、fns.c 5019、emacs.c 262），端口一直靠**用点副本**撑着。
  本轮把 5 条声明补进两个镜像（`include/keyboard.rs`、`include/lisp.rs`，位置按 C 行序），
  并**退役 5 处用点副本**（`process.rs`、`emacs.rs`、`pdumper.rs`、`xfaces.rs`、`buffer.rs`），
  5 个单元重建全 **built**；两个头 `--check` 均 **OK**；台账 `docs/ports/lisp.md` 与
  `docs/ports/include-keyboard.md` 各补上对应行（C 行号照抄抽取器的新读数）。
  ④ **教训（写给后来者）**：工具的盲点会让"全绿"变成假象——这次是"某类对象从不被比较"，上一次是
  "反斜杠续行字面量被当成 5 个对象"（3.130）。因此 gate 修好后必须**重跑 `--all` 看新增的 FAIL**，
  而不是只跑改动文件；也要把新发现的对象挨个回查定义点，别让"镜像缺声明 + 单元有副本"这种半影子状态留下来。
  ⑤ 本轮实测：`--all` **514 ported files: 507 done, 7 in progress, 0 failing**；`cinventory --selftest` 5/0；
  `check-declarations` 13 sites (0 forbidden) / 0；`check-mirror-dupes` 0 failure；
  `check-globals` 283/0；`check-verbatim` 6 trees/4607 files/0；`check-target-names` 549 `.rs`/593 targets/0。
  另：`docs/ports/include-c++defs.md` 的一处**错误论断**同批改正——它原先按
  `rg "c++defs\.h"` 的输出列了四个"包含者"，而那个模式不是合法正则；用
  `grep -F` 复测的真相是**没有任何源文件包含它**（只有 `gnulib.mk.in` 3180/3182 的构建机制），
  台账已按实测重写（并把"为什么当时的输出不可当证据"写进去）。

- **3.144 第一条非 C lane 的第一个单元入库：`src/nsimage.m` → `femacs/platform/macos/nsimage.swift`，并定下 Swift↔Rust 的 ABI 规矩（2026-10-04）**：
  ① **单元**：`emacs/src/nsimage.m`（658 行）→ `femacs/platform/macos/nsimage.swift`（753 行），台账
  `docs/ports/nsimage.md`（393 行，34 个勾选框全勾）。**集成者独立复验**（不采信 lane 报告）：
  `swiftc -parse-as-library -c -o /tmp/nsimage-check.o` → **exit 0**（只有 warning：`EmacsImage` 非 final
  在 Swift 6 语言模式下不满足 `Sendable`）；`nm -g` 数出 **15 个 `T _ns_*`**，与 lane 的"15/15 可见"一致。
  ② **ABI 规矩（本条是给剩下 8 个 Swift 文件定的）**：Swift 侧**只经 C ABI 与 Rust 复刻对接**，
  不引入任何 C 头、不写 shim、不复写解释器内部——
  - 参照里有真实符号的（`valid_image_p`(image.c 1395)、`image_find_image_file`、`add_to_log`(xdisp.c)、
    `plist_get`(fns.c)、`Fcons`(alloc.c)、`make_float`）按 C 原型声明直接用；
  - **参照里没有符号的构造（宏 / `INLINE` / `globals.h` 的字段）不在 Swift 里复写，而由 core 侧按 C 构造的
    名字导出 C-ABI 入口**：`EQ`、`STRINGP`、`FIXNUMP`、`XFIXNAT`、`XCDR`、`SSDATA`、`SBYTES`、`make_fixnum`
    与 12 个 `Q*`（索引取 `docs/baseline/globals/symbols.tsv`）等约 20 个；这也让 Swift 侧的调用形态与
    Rust 镜像已有的 `pub fn Qfoo ()`（P-17）一致。
  - ③ **编译模型**：**单文件 `swiftc -c` 作为每文件门**（今天唯一能逐文件跑的编译证据）；将来 9 个文件
    作为一个 Swift 模块编译时，共享的 ABI 声明必须抽成单一来源（PORTING.md 0.1-5）——记为**开口项**，
    到链接阶段前定，不在本单元内发明。
  ④ **对读发现（原样保留，写进台账）**：`getPixelAtX:Y:` 的 `colorAtX:y:` 分支在 `int` 里拼 ARGB 后按
  `unsigned long` 返回会符号扩展；`allocInitFromFile:` 在 `[image isValid]` 之前就置 `bmRep = nil`；
  `setFrame:` 的 `index < 0` 对无符号恒假；`stippleMask` 的 `CGImageRetain` 不平账。四条都是参照自身的形状，
  按"一比一"不动。
  ⑤ **一条待更正的映射**：`docs/port-map.md` 把 `lib-src/be_resources.cc`（Haiku 构建用的资源工具）映成
  `femacs/platform/macos/be_resources.swift`，这是生成器"按 `*.cc` 归到平台新技术"的归类结果，落该文件前
  先改 `tools/gen-port-map.py` 的规则（或加显式例外）。

- **3.145 mirror lane 的六个镜像单元入库：`haiku_support.h`、`androidgui.h`、`android.h` 新增，`androidterm.h`/`pgtkterm.h`/`haikugui.h` 收尾（2026-10-04）**：
  ① **集成者逐对复跑 `--check`**（不采信 lane 报告）：`haiku_support.h`(784)、`androidgui.h`(930)、`android.h`(363)、
  `androidterm.h`(496)、`pgtkterm.h`(681) **五对全 OK**，五个台账 **0 未勾**；`check-mirror-dupes` 1 duplicate /
  1 ratchet / 0 failure、`check-declarations` 13 sites (0 forbidden) / 0、`check-target-names` 0 finding。
  ② **lane A 的三条判定值得留档**：`androidgui.h` 在冻结目标**没有死区**（`ANDROID_STUBIFY` undef ⇒ C 256-928
  的 image 段全活，逐声明携带）；`haiku_support.h` 的活文本是 13 个 enum（125 常量）、29 个结构、3 个对象、
  175 条原型，死文本（`__cplusplus` 头尾、`HAVE_FREETYPE` 两个 include、`HAVE_NATIVE_IMAGE_API` 3 条、
  `USE_BE_CAIRO` 4 条）按 P-08 记账不抄——**并点名"批 F 的 Haiku C++ 消费者（4 文件/8,085 行）将来要用的正是
  那一段"，到时要给它落点**；`android.h` 的 `<jni.h>`/`<android/bitmap.h>` 按 P-49/P-70 在镜像里按 C 分支形状
  声明（`JNIEnv = *const JNINativeInterface` 等，不透明）。
  ③ **两条 `excluded-macros.txt` 新增行**（lane 写、我复核并补了"替代物"一句，符合该文件表头"a mirror of
  another header appends its own rows here"的邀请）：`NATIVE_NAME`（android.h 344 的
  `Java_org_gnu_emacs_EmacsNative_##name` 把实参拼进标识符，Rust 无 identifier pasting ⇒ 定义点写
  `#[unsafe(export_name = "…<name>")]`）与 `ANDROID_DELETE_LOCAL_REF`（android.h 340-342 读 `<jni.h>` 的
  vtable 项，NDK 的 `JNINativeInterface` 约 230 项无镜像 ⇒ 不透明结构读不到字段）。
  ④ **一条已记录的平台冲突**：`xtask layout-check` 把整个 `src/include` 编进一个 crate，平台互斥镜像因此报
  **35 条 `E0428`**（`haikugui.rs` 与 `dispextern.rs` 的 `Emacs_Cursor`；`androidgui.rs` 与 `haikugui.rs` 的
  `XCharStruct`/`Emacs_Pixmap`/`Emacs_Window`/`Drawable`/`Emacs_Cursor` 与 29 个 X-compat 常量）。裁决：**镜像
  不加 `#[cfg]`、不包 `pub mod`**（前者让被 cfged 的项失去编译器，后者会藏起 `macro_rules!` 的名字），改用
  **独立平台探针 crate** 提供编译证据（`femacs/nt/inc` 的先例），lane A 本轮补探针；`layout-check` 自身是否
  学会跳过平台互斥镜像由集成者决定（记入下一轮）。

- **3.146 又两个同类的抽取器盲点：`typedef struct X {…} Y;` 看不见，以及"注释里的结构体把真结构体的字段表覆盖成空"（2026-10-04）**：
  ① **盲点一（lane A 报的）**：`struct_fields` 的 `STRUCT_OPEN_INLINE`/`STRUCT_OPEN_NAME` 只认行首的
  `struct|union`，于是 `typedef struct haiku_char_struct { … } XCharStruct;` 整段**不可见**——
  `src/haikugui.h` 的 `XCharStruct` 五个字段一直被镜像携带着，却**从不参与比较**（3.143 的同类）。
  修法：两条正则各加可选前缀 `typedef\s+`（`} Y;` 里的别名不是字段，不收）；`struct_fields` 的
  `--selftest` 用例由 5 条变 **6 条**（新增 `typedef'd struct and union bodies are not invisible`）。
  ② **盲点二（修完①立刻暴露的）**：`rust_inventory` 的 Rust 侧结构体扫描跑在**原始文本**上，而镜像的文档
  注释里会照抄 C 原型（PORTING.md 0.1-2/P-55 要求的那一行）——`lwlib-widget.rs` 的
  `/// lwlib-widget.h 43-103: \`typedef struct _widget_value { ... } widget_value;\`` 正好被那条正则匹配，
  并且**用注释里的空字段表覆盖了真结构体的 18 个字段**（同名后写覆盖）。②①合起来让 6 个单元立刻 FAIL：
  `lib-src/etags.c`（`struct stack_entry`）、`lwlib/lwlib-int.h`、`lwlib/lwlib-widget.h`、`lwlib/xlwmenuP.h`、
  `oldXMenu/X10.h`、`oldXMenu/XMenu.h`，报的都是"整条字段表全缺"。
  修法：扫描跑在 `strip_comments` 之后的视图上（`include!` 进来的文件同样剥注释），并且**后到的空匹配不再
  覆盖已有的非空表**。修完 6 对 `--check` 全 **OK**，`--all` 回到 **514 ported files: 507 done / 7 in progress /
  0 failing**。
  ③ **记录（不静默）**：`docs/ports/*.md` 的 "Types and their fields" 段是 3.146 之前生成的，因此**不含**
  typedef 形态的结构体行；这些台账在该文件下次被动到时用新抽取器重生成（生成会覆盖手写的
  `## Decisions`/`## Evidence`，所以由集成者在重生成后补回）。本轮只动工具与状态，不改任何台账。
  ④ 证据：`cinventory --selftest` **6 cases / 0 failures**；六个单元 `--check` OK；`--all` 0 failing；
  `check-declarations` 13 sites (0 forbidden)/0；`check-mirror-dupes` 0 failure；`check-target-names` 549 `.rs`/
  593 targets/0 finding。

- **3.147 `layout-check` 恢复绿灯，`be_resources.cc` 的归类改正（2026-10-04）**：
  ① **`layout-check` 之前是红的**（把整个 `src/include` 编进一个 crate）：平台互斥镜像的 X 兼容名互相撞
  （`androidgui.rs` 34 条 + `haikugui.rs`/`androidterm.rs` 各 1 条 = **36 条 `E0428`**），而且这个撞墙**掩护了
  另外两类错误**：`dispextern.rs` 的 `MAX_CHAR`、`termhooks.rs` 的 `Time`、`buffer.rs` 的 `STRING_CHAR` 未解析
  （E0425），以及 `character.rs` 自己的 `eassert!`/`eassume!` 未定义（宏是**文本作用域**，而 `lisp.rs` 被列表去重
  掉、`character.rs` 只被两条已排除的平台镜像 include）。
  ② **两处修法**（都在 `femacs/xtask/src/main.rs`）：**(a) 平台互斥镜像按数据文件排除**——
  新增 `docs/baseline/layout-excluded-mirrors.txt`（一行一个文件名 + 理由：冻结目标根本不编译这些头），
  `layout-check` 读它、**打印跳过数量与名单**（`51 of 59 mirror(s) compiled, 8 platform-exclusive skipped
  (android.rs, androidgui.rs, androidterm.rs, gtkutil.rs, haiku_support.rs, haikugui.rs, haikuterm.rs,
  pgtkterm.rs)`），每个平台的布局断言由该平台自己的探针 crate 负责（lane A 本轮补；见 3.145 ④）。
  **排除必须发生在"包含图去重"之前**——`character.rs` 只被 `androidterm.rs`/`haikuterm.rs` include，先排掉那两条
  才能让它留在探针里。**(b) 基础镜像按 C 头顺序置顶**：`stdio.rs` → `mini-gmp.rs` → `lisp.rs` → `character.rs`
  → `systime.rs`，理由是"宏文本作用域"与"PORTING.md 0.1-9 交给 TU 的名字"；`Expand.seen` 会对重复 include 去重，
  所以置顶不会产生重复项，而**只有首次展开的位置算数**。
  ③ **实测**：`cargo run -p xtask -- layout-check` → **exit 0、0 行 error、`layout-check passed`**，并打印上面那行
  可见的跳过清单；顺带 `build-tu lib/getdelim.c` 仍 **built**、`layout-asserts` exit 0（公共路径未被改坏）。
  ④ **`be_resources.cc` 归类改正**：`tools/gen-port-map.py` 原先有一个 `SWIFT_EXTRA_STEMS = {"be_resources"}`
  （注释写"C++ file of the macOS bundle"），把 **`lib-src/be_resources.cc` 映成 macOS 的 `be_resources.swift`**。
  该文件第一行自述 "Haiku window system support"、include 的是 `<SupportDefs.h>`/`<AppFileInfo.h>`，
  `lib-src/Makefile.in` 450 行只在 Haiku 上构建它（`be-resources`）——所以它属于 Haiku 的 C++ 那一组。
  删掉那个 set、把 `be_resources` 加进 `CPP_STEMS`，重生成后它的目标是
  **`femacs/platform/haiku/be_resources.cc`**，C++ 表由 4 条变 **5 条 / 8,264 行**。

- **3.148 `check-target-names` 接纳"别的工具链的系统头"镜像，并定下 `android-asset.h` 的做法（2026-10-04）**：
  ① mirror lane B 在收口时把 `android-asset.h` 停下来问（它自身 0 个 `#if`，9 个 JNI 函数按"无条件文本必须携带"
  要逐声明写；但它只被 `androidvfs.c` 的 `#else /* __ANDROID_API__ < 9 */` 包含，而 `__ANDROID_API__` 在冻结表里
  根本不存在）。**裁决：走方案 A（它是 590 里的一行，`--check` 必须能给 OK，所以镜像要真写）**，前提与做法：
  - 先落一份 `femacs/src/include/jni.rs`——**别的工具链的系统头镜像**（NDK 的 `<jni.h>`，本机
    `~/Library/Android/sdk/ndk/*/…/sysroot/usr/include/jni.h` 可实测），首行按新形式自我声明；
    `JNINativeInterface` 的函数表偏移用 `offsetof` 逐项实测并配 `const _: () = assert!(…)`。
  - 该镜像**不在 590 之内**（参照树里没有 `jni.h`），属于 P-26/P-49/P-70 的"系统头替身"家族，因此它不需要
    map 行；但 `check-target-names` 的 R1 会要求它**自我声明**，所以本轮把该门的 `DECLARED` 增一条
    「`// femacs: mirror of the <谁的> <头>`（任意 possessor）」，并补 selftest 用例（**9 cases / 0 failures**；
    全树 550 `.rs` / 593 targets / 0 finding）。`jni.h` 只是**第一个**这类镜像，Haiku SDK 的 `<kernel/OS.h>`
    （mirror lane A 已在 `haiku_support.rs` 里就地声明了几个类型）是同类——将来若也要独立镜像，照这条走。
  ② `android-asset.rs` 之后可以与 `jni.rs` 同批（或随后）落地：2 个 struct、1 个 `static const char *`
  目录树、2 条原型、9 个函数体（含 `AASSET_MODE_*` 匿名枚举）与那两个 `excluded-macros` 之外的排他项。

- **3.149 平台镜像的编译证据入册：两条探针进 `xtask` 的 PROBES 名单（2026-10-04）**：
  ① lane A 交回 `spikes/haiku_inc_probe.rs`（`haikugui.rs`+`haiku_support.rs`+`haikuterm.rs`）与
  `spikes/android_inc_probe.rs`（`androidgui.rs`+`android.rs`）——它们就是 3.147 里"平台互斥镜像的编译证据由
  各自平台探针负责"那条裁决的落地物，文件头都写明了为什么 `layout-check` 装不下它们。
  ② **我把它们注册进 `femacs/xtask/src/main.rs` 的 `probe_programs()`**（`PROBES` 8 → 10，附一行注释说明
  与 3.147 的关系），因为那才是本项目既有的"探针编译门"（`gate-fast` 会逐个 `materialize` 去重再编译）；
  实测 `cargo run -p xtask -- probe ../spikes/haiku_inc_probe.rs …` 与 `… android_inc_probe.rs …` **都 exit 0**。
  我先写的临时 `spikes/inc-probe.sh` **已删除**——探针的编译路径只留一条（否则就是"同一构造两种写法"）；
  直接用 `rustc` 编这两个探针会因 `include!` 无包含卫而报几百条 `E0428`，`xtask probe` 的物化去重正是为此存在。
  ③ lane A 同时把两条 `excluded-macros` 行补成带 `Replacement:` 的完整形式（`NATIVE_NAME` →
  定义点 `#[unsafe(export_name = "…<name>")]`；`ANDROID_DELETE_LOCAL_REF` → core 侧按 C 构造名导出 C-ABI 入口），
  并给四个平台镜像的台账补了决策段（都是 additive）。
  ④ **`haikuterm.rs`（19,671 字节）本轮不提交**：它已落盘、`--check` 也 **OK**，但台账还有 **1 个未勾**
  （`struct haikufont_info` 的字段行），说明还在 lane 手上。顺带记一条**门的边界**：`--check` 的字段比较只遍历
  *Rust 侧*已有的容器，所以"C 里有一整个 struct、镜像完全没有"这种情况**不会**被门报出来
  （`haikufont_info` 正是这一例，靠台账未勾框才看得见）——把"缺类型"也纳入比较是下一步可做的工具改进去处。

- **3.150 两个单元入库：`lib/diffseq.h` 镜像与 batch F 的第三个 Swift 文件 `nsselect.swift`（2026-10-04）**：
  ① **`lib/diffseq.h`（578 行）→ `femacs/lib/include/diffseq.rs`**（mirror lane A）：`--check` **OK**、台账
  `docs/ports/include-diffseq.md` **0 未勾**。
  ② **`src/nsselect.m`（825 行）→ `femacs/platform/macos/nsselect.swift`（982 行）**（Swift lane），台账
  `docs/ports/nsselect.md`（384 行、30 勾、0 未勾）。**集成者独立复验**（不采信 lane 报告）：
  `swiftc -parse-as-library -c` **exit 0、0 行 error**；`nm -g` 数出 **3 个 `T _ns_*`**——
  `ns_get_local_selection`、`ns_string_from_pasteboard`、`ns_string_to_pasteboard`，正是这个单元对外的那一面
  （其余是 ObjC 方法，不导出 C 符号）。
  ③ **`femacs/src/include/jni.rs`（51,840 字节）暂不提交**：它首行已按 3.148 定下的形式自我声明
  （`// femacs: mirror of the Android NDK's <jni.h> …`），`check-target-names` 全树 **0 finding**、552 个 `.rs`；
  但它与 `android-asset.rs` 是一对（后者要对前者取 `JNIEnv` 的函数表），**等后者落地一起验、一起提交**，
  避免只进一半造成"半影子"状态。
  ④ **`haikuterm.rs` 仍不提交**：台账那 1 个未勾（`struct haikufont_info` 的字段行）还在，属 lane 手上。

- **3.151 量了"缺类型"检测的噪音：先别做朴素版，先把 tag↔typedef 别名的对应补上（2026-10-04）**：
  3.149 记下了一条门的边界——`--check` 的字段比较只遍历 *Rust 侧*已有的容器，所以"C 有、镜像完全没有"的 struct
  不会被报（`haikufont_info` 是活例子）。本轮**先量了朴素版会报多少**，方法是把 514 对（C 文件 ↔ 它的镜像）里
  两边都存在、且 C 侧有容器的 **192 对**逐对比：C 的容器名（`cinventory.struct_fields`）是否有同名的
  Rust `pub struct/union/enum/type`。
  **结果：344 个容器"没有同名 Rust 类型"**——如果直接拿它当门，就是 344 条噪音。抽样分类（证据在下面）：
  - **主因是命名约定，不是缺口**：C 的 `typedef struct X { … } Y;` 在工具里容器名是 **tag** `X`，而端口按
    typedef 名写 Rust（`lib-src/etags.c` 275 的 `typedef struct node_st { … } node;` → 镜像是
    `pub struct node`；`lib-src/etags.c` 249 的匿名 `typedef struct { … } compressor;` → 镜像
    `pub struct compressor`，而 C 侧容器名是 `<anon@245>`）。这两类在朴素检查里必然"缺"。
  - **少数是真的缺**（要逐个回查才会知道）：例如 `lib-src/asset-directory-tool.c` 60 的
    `struct directory_tree`，镜像 `femacs/lib-src/asset-directory-tool.rs` 里**搜不到**这个名字。
  **设计（下一步照这个做）**：① `cinventory.struct_fields` 在遇到 `} NAME;` 时把那个 typedef 别名也记进
  记录（tag 与别名并存）；② 比较时"C 的 tag 或别名"命中"Rust 的 struct 名或 `pub type` 别名"即算在场；
  ③ 再跑一次全表，把残留按"真缺 / P-55 的 `*mut c_void` + `/// C:` 记账 / P-70 替身"逐个归类，
  真缺的才开成 lane 的工作项。**不先做这一步就不加门**——3.143/3.146 的教训是"纳入了就要能修完"，
  而 344 条里绝大多数是约定而非缺陷。

- **3.152 `struct_fields` 记下 typedef 别名（tag 与别名并存），并把"缺类型"检测的残差一路量到 21（2026-10-04）**：
  ① **改动**（`tools/cinventory.py`）：两条 open 正则各多一个 `(?P<td>typedef\s+)?` 捕获；被 `typedef` 引入的聚合
  在闭合行 `} NAME;` 处把 `NAME` 记成该容器的 **alias**（字段记录上同时带 `container` 与 `alias`）。
  **关键区分**：非 typedef 聚合里的同一个 `} spec;` 形状是**成员**，代码路径不动——实测四例：
  `typedef struct X {…} Y;` → `(X, alias Y)`；匿名 `typedef struct {…} Y;` → `(<anon@N>, alias Y)`；
  `struct Z { struct { … } spec; };` → `spec` 仍是 Z 的成员、无 alias；`typedef struct node_st {…} node;` →
  `(node_st, alias node)`（`etags.c` 实测：`<anon@245>→compressor`、`node_st→node`、`stack_entry→stkentry`）。
  `--selftest` 由 6 例变 **7 例**（新增"typedef 聚合记别名、成员不记"）。
  ② **残差一路量下去**（每一步都实测，不是估）：
  - 朴素版（C 容器名 vs 镜像同名类型）：**344**；
  - 引入 tag-或-alias 匹配后：**253**（alias 只解释掉 90 条左右）；
  - 只统计 `docs/baseline/frozen-build-sources.txt` 里的 TU（83 条；其余 515 对跳过）：**21**。
  这 21 条**看着全是"我的临时脚本绕过了工具的条件上下文"造成的假阳性**——`ci.Lines`/`ci.struct_fields` 直接调用
  时没有装载冻结宏表（工具的入口 `inventory()` 是用 `_BlindConditions` 包住的），于是 `#ifdef HAVE_TREE_SITTER`
  （undef）里的 `treesit_lang_cache_entry`、`HAVE_ALSA` 里的 `alsa_params` 等**死分支结构体**都被当成活文本。
  ③ **结论（仍未开门，但设计收敛了）**：这条检查要成立必须同时满足——(a) **只对冻结 TU 列表里的文件**比；
  (b) **用工具自己的条件上下文**（`inventory()` 那条路径）而不是裸调 `struct_fields`；
  (c) 跳过"匿名且无 alias"的容器（它们没有可比的契约名）。下一步是在工具里按这三条实现、再量一次真残差，
  真缺的才开成工作项。**本轮只落了①（有自检的净改进）与②的实测链**，没有把 253/21 里的任何一条当成缺陷去改。
  ④ **当前门读数**：`--all` → **517 ported files: 509 done, 7 in progress, 1 failing**；唯一 FAIL 是 lane 正在做的
  **未提交**文件 `femacs/src/include/android-asset.rs`，三条是标准 token 类（`AAssetManager_open`/`android_asset_read_internal`
  的 `goto`、`AAsset_seek` 的 `case, default`），按既有惯例要由**集成者**在 `docs/ports/macro-body-notes.tsv`
  补行（`switch,case,default` 与 `goto` 的先例已有多条）——等 lane 收口报告带上它的准确定位后再补，避免改两次。

- **3.153 "缺类型"检查落地：它当场抓到 2 个真缺口，并暴露两处**写反的**决定记录（2026-10-04）**：
  ① **实现**（按 3.152 量出的三条口径）：`tools/extract-abi.py` 的 `compare()` 新增一段——对"冻结目标的 TU
  （`FROZEN_BUILD_SOURCES`）或任何 `.h`"里的每个 **有字段的** C 聚合，若它的 **tag 与 typedef 别名都不在**
  Rust 侧的 `struct/union/enum` 名与 `pub type` 名里，就报 `missing type <名>`；匿名且无别名的容器跳过
  （没有可比的契约名）。Rust 侧同时开始收集 `pub type` 别名（`rust_inventory` 的 `aliases`）。
  ② **它第一次跑就报了 4 条**：`src/comp.h` 的 `Lisp_Native_Comp_Unit`、`src/lisp.h` 的 `gc_root_visitor`、
  `src/timefns.c` 的 `timespec`/`timeval`。其中后两条是**解析器假象**：C 把返回类型单独写一行
  （`struct timeval\nmake_timeval (struct timespec t)\n{`）时，扫描器把函数的 `{` 当成结构体体，于是凭空造出
  `timeval`/`timespec` 容器、字段还是函数的局部量。修法：`pending` 的 `struct X` **下一行必须是 `{`**，否则
  清空 `pending`（`tools/cinventory.py`）——实测该形状产出 0 个容器，而真正的 `struct X\n{` 照旧。
  ③ **两条真缺口都修了**，而且发现**台账早就把这两行勾了**（`docs/ports/lisp.md` 509、`docs/ports/include-comp.md` 15）
  ——说明台账领先于镜像，缺的就是这道执行：
  - `gc_root_visitor`（lisp.h 4504-4508）：加进 `include/lisp.rs`（两个字段，函数指针按本文件既有的
    `Option<unsafe extern "C" fn …>` 拼法），并**退役两处用点副本**（`alloc.rs` 6043、`pdumper.rs` 175，
    它们的注释原本写着"mirror 是缺口"）；`comp.c`/`pdumper.c`/`alloc.c`/`data.c` 四个单元重建全 **built**。
  - `Lisp_Native_Comp_Unit`（comp.h **25-51**）：加进 `include/comp.rs`（字段表照 `pdumper.rs` 里那份**已验证过**
    的副本；`GCALIGNED_STRUCT` → `#[repr(C, align(8))]`），`comp.rs` 同时补上 C 23 的 `#include <dynlib.h>`
    （`dynlib_handle_ptr` 的属主）；`pdumper.rs` 里那份 `#[cfg (feature = "native_comp")]` 副本**删掉**
    （它与镜像的记录在 native_comp 构建里会 `E0428`，镜像经 lisp.h→comp.h 的链条到达该单元）。
  ④ **纠正两处写反的陈述**（这是本轮最有价值的部分）：`include/comp.rs` 的头注与
  `docs/ports/include-comp.md` 的 Decisions 都说"字段承载的 `Lisp_Native_Comp_Unit`（C 30-44）在
  `#ifdef HAVE_NATIVE_COMP` 里、是死文本"——**照参照，结构体是 C 25-51、在 `#ifdef`(C 52) 之前，即无条件活文本**；
  两处都已按实测重写（并把"曾把守卫用在*使用点*上这件事当成定义"记进去）。
  ⑤ 顺带两处基建：`xtask layout_asserts` 生成的断言改用 **`::core::assert!`**（镜像里若有"复刻 C 的
  `<assert.h>` 宏"的文件，例如 lane 正在写的 `android-asset.rs`，会把 prelude 的 `assert!` 变歧义），
  并把 `android-asset.rs` 加进 `docs/baseline/layout-excluded-mirrors.txt`（Android 平台头，编译证据归 android
  平台探针）；`layout-check` 恢复 **exit 0 / passed**。
  ⑥ 证据：`--check` 的 `lisp.h`/`comp.h` 均 **OK**；`build-tu` 的 comp/pdumper/alloc/data 全 **built**；
  `--all` → **517 ported files: 509 done, 8 in progress, 0 failing**；`cinventory --selftest` 7/0。

- **3.154 三条 lane 交付入库（jni / android-asset / haikuterm），并给缺类型检查加了一条台账豁免（2026-10-04）**：
  ① **`jni.rs`**（NDK 的 `<jni.h>` 镜像，53KB）——它是"别的工具链的系统头"，没有 C 侧可配对，所以它的门是**差分探针**：
  `spikes/jni-probe.sh` 实测 "233 entries, jni.h order matches the mirror"、"the NDK's clang confirms all
  234 assertions (aarch64-linux-android24)"、"the mirror's offset_of!/size_of assertions hold (pinned toolchain,
  exit 0)"。我把该探针**注册进 `gate-probes`**（它是 jni.rs 唯一的门），实测 gate-probes **exit 0**。
  ② **`android-asset.rs`**（22.7KB）+ 它的平台探针 `spikes/android_asset_probe.rs`（已注册进 `probe_programs()`
  的 PROBES，10 → 11）：`--check` **OK**、台账 0 未勾。
  ③ **`haikuterm.rs`**：`--check` **OK**，但台账还有 1 个未勾（`struct haikufont_info`）——**这条暴露一个真依赖**：
  该结构体第一个字段是 `struct font`（**按值**，不是指针），而 `include/font.rs` 是 WIP 切片、还没有
  `pub struct font`（它现在只以**用点副本**存在于 `font.rs` 256 / `window.rs` 893 / `xfaces.rs` 337）。
  也就是说这条记录要等 font.h 的镜像长出 `struct font` 才写得出来。
  **为了既不造 stub 也不静默**，给缺类型检查加了一条**台账豁免**：当文件自己的台账还有未勾框时，报
  `missing type deferred: <file> <name>（the ledger has unchecked boxes）` 到 stderr 而不计入 problem——
  本轮 `--all` 上共 **18 条**这样的可见延后（每一条都能顺藤查到某个 WIP 依赖）。
  ④ **一串同源用点副本的退役**（都由我这个 `include/comp.rs` → `include/dynlib.rs` 的 include 引出）：
  `pdumper.rs` 里 `dynlib_open_for_eln`/`dynlib_sym`/`dynlib_error` 三处副本由 `check-mirror-dupes` 抓到，
  **第四处 `dynlib_handle_ptr` 只有 `dead-arms` 门抓到**（它在 `native_comp` 构建里才活）——又一次"只有编译
  才算数"的实证；四处都退役并留了指向镜像的注释。
  ⑤ 证据：`--all` → **517 ported files: 510 done, 7 in progress, 0 failing**；`layout-check` **passed**；
  `gate-probes` **exit 0**（含 jni 探针与 336 个 TU 的非冻结臂编译）；`check-declarations` 13 sites (0 forbidden)/0；
  `check-target-names` 553 `.rs` / 593 targets / 0 finding；`check-mirror-dupes` 0 failure。

- **3.155 两条巨型 lane 的本轮交付复核通过，并修掉我自己在 3.143 留的一个漏检（2026-10-04）**：
  ① **xdisp（C 4983-5647）**：`face_before_or_after_it_pos` → `handle_invisible_prop`，`xdisp.rs` 5,094 → **6,313** 行；
  我复核的 `--check` **313 条 problem 全部在 C 5647 之后**（无一条落在本轮新增函数上），`build-tu src/xdisp.c` **built**，
  台账 Progress 段已推进。下一条活定义 **C 5654 `setup_for_ellipsis`**；已按它自己交回的三条 ABI 提醒重新派工
  （`it_props[]` 余下两个 handler 必须 `unsafe extern "C" fn`；`move_it_vertically_backward`/`move_it_in_display_line`
  定义要 `#[unsafe(no_mangle)]`；`reseat_1`/`next_overlay_string`/`get_overlay_strings` 的 scaffold 声明定义时删）。
  ② **keyboard（C 8577-10070）**：菜单栏/标签栏/工具条整族（1,496 C 行、19 函数、9 对象），`keyboard.rs` 9,355 →
  **10,990** 行；`--check` 的 **108 条 problem 全部在 C 10070 之后**；台账未勾 103 → **74**，SCAFFOLD 仍 9。
  下一条活定义 **C 10096 `read_char_x_menu_prompt`**，已重新派工。
  ③ **复核抓到一处我自己的漏检（本轮修）**：3.143 我把 `timers_run`(C 471)/`DEV_TTY`(C 531) 按"镜像拥有名字"加进
  `include/keyboard.rs` 的 **crate 根**，而这两个对象的**定义**在 `keyboard.rs` 的 **crate 根** → 同 crate 同作用域
  两个项 ⇒ `build-tu src/keyboard.c` 报 `E0428: the name timers_run is defined multiple times`。
  **根因是我的验证集漏了它**：3.143 那轮我编了 process/emacs/pdumper/xfaces/buffer 五个单元，**唯独没编 keyboard**
  （它当时是 WIP 单元、不在我的验证集合里）。修法：把两条声明**移进 `pub mod keyboard_objs` + 既有 glob 导入**
  （PORTING.md 0.1-10/P-21 的 `variables` 形状——定义方在 crate 根、声明方走整体导入，定义遮蔽导入）；修完
  keyboard/process/emacs **三个单元全 built**。
  **教训（写进这里）**：往镜像加声明后，**凡其定义位于某个单元 crate 根的，都要编那个单元**；"定义在 crate 根"
  与"定义在模块里"是**两种相反的布置**——前者要求镜像的声明走 glob，后者相反（`item_properties` 就是后者）。
  ④ 证据：`--all` → **517 ported files: 510 done, 7 in progress, 0 failing**；`check-mirror-dupes` 1 duplicate /
  1 ratchet / **0 failure**；`check-declarations` 13 sites (0 forbidden) / 0；`check-target-names` 553 `.rs` /
  593 targets / 0 finding。

- **3.156 `lib/root-uid.h` 镜像落地：一个宏、两个臂，并纠正一处写错的用点副本注释（2026-10-04）**：
  ① **单元**：`emacs/lib/root-uid.h`（30 行）→ `femacs/lib/include/root-uid.rs`，台账
  `docs/ports/include-root-uid.md`。整头只贡献一个宏：C 20-21 是 include guard、C 23 是注释、
  C 24-28 是 `#ifdef __TANDEM / # define ROOT_UID 65535 / #else / # define ROOT_UID 0 / #endif`。
  ② **两臂都带，C 27 的 `0` 在前**：`__TANDEM` 是 NonStop 编译器自己的宏，冻结表看不见它，
  `check-cond-eval.py lib/root-uid.h` 如实报 `2 conditions, 0 unknown, 1 disagreement`。C 27 的 `0` 既是冻结配置
  选择的那条、也是抽取器记录的"最后定义"（C 的 `#else` 在后），所以写在前；C 25 的 `65535` 走
  `#[cfg (feature = "tandem")]`（P-51）。这类"同名多定义"抽取器**按 presence 比、不计 body**——`--check` 的原话就有
  `1 C macros have several definitions (configuration branches), body not compared: ROOT_UID`——这正是两臂能并存的原因。
  ③ **命名规则留档**：特性名去掉前导下划线（`HAVE_ANDROID`→`have_android` 全树 96 处、`WINDOWSNT`→`windowsnt` 79 处），
  所以 `__TANDEM` → `"tandem"`。`ROOT_UID` 拼成 `uid_t`（C 宏无类型；唯一用户 `euidaccess.c` 拿它比
  `geteuid ()`，类型由该单元的 include 提供，PORTING.md 0.1-9）。
  ④ **纠正一处写错的用点注释**（与 3.153 的 comp.h 同类）：`femacs/lib/euidaccess.rs` 原本写着
  "root-uid.h is two lines: `#include <sys/types.h>` and that define"——照参照，这个头**没有任何 `#include`**、
  共 30 行（guard + 注释 + `__TANDEM` 两臂）。副本的退役与该注释的删除同批完成，单元改为在 C 32-34 的位置
  `include!("include/root-uid.rs")`。
  ⑤ 证据：`--check` **OK**（含上面的 branch-macro 注记）；`build-tu lib/euidaccess.c` → **built**；
  `check-declarations` 13 sites (0 forbidden)/0、`check-mirror-dupes` 0 failure、`check-globals` 283/0、
  `check-verbatim` 6 trees/4607 files/0、`check-target-names` 554 `.rs`/593 targets/0 finding；
  `--all` → **518 ported files: 511 done, 7 in progress, 0 failing**；`port-inventory` 复算
  **518 = 503 done / 15 wip，待开工 72**（本行由 todo → done）。
  ⑥ **调度记录**：本机 agent 线程 6 个位子已满（xdisp / keyboard / font / mirror lane A / Swift），所以
  gnulib-template 的下一批（`lib/errno.in.h`、`lib/dirent.in.h`）**本轮没能开 lane**。它的方法与先例写在这里备下一轮：
  判据是**生成后的头**（`build/emacs-ref/lib/<name>.h`），不可判的臂按 P-51 以 `#[cfg (feature = "…")]` 承载，
  取值/宽度用冻结 clang 实测；先例见 status 3.131/3.132 与 `docs/ports/include-{stdint.in,fcntl.in,unistd.in}.md`。

- **3.157 `lib/assert.in.h` 镜像落地：一个"什么条目都没有"的头，以及它的那条判不了的条件（2026-10-04）**：
  ① **实测的"零条目"**：`extract-abi.py lib/assert.in.h` 报 0 subrs / 0 functions / 0 macros / 0 types /
  0 file-scope objects——这个模板的每一行都是预处理文本（C 25 的 `#include_next <assert.h>`、C 20-22 的
  `#pragma GCC system_header`、C 23 的 `@PRAGMA_COLUMNS@`、两条注释），所以镜像就是一份**处置记录**
  （`arg-nonnull.rs`/`c++defs.rs` 同形）：`--check` 因此 **OK**。
  ② **它唯一的那条条件判不了**：`check-cond-eval.py lib/assert.in.h` 报 `1 conditions, 0 unknown, 1 disagreement`
  （clang 在这个配置下认为 `__GNUC__` 未定义，求值器的宏表不这么认为）。但**两臂承载的是同一条 `#pragma`**，
  而 pragma 不是声明 ⇒ 既不需要 P-51 的 `#[cfg]` 臂、也没有值可测；这条 disagreement **如实写进台账**，
  不是"绕过"。
  ③ **这个模板没有 include guard，且是它自己说的**：C 17-18 解释"不能加保护，因为 `<assert.h>` 每次包含都要定义
  `assert`" ⇒ 端口也没有 guard 名可带（与 `root-uid.rs` 的 `ROOT_UID_H_` 正好相反，台账里点了名）。
  ④ **需要 C 的 `assert` 的单元各自镜像那个宏**：`lib-src/ebrowse.rs` 与 `include/android-asset.rs` 各有
  `macro_rules! assert` + 冻结 `NDEBUG` 体——两处是**同一构造的同一拼法**，本模板什么都不交给它们；
  台账把这条"归属"写明，免得后来者以为该由本镜像提供。
  ⑤ **没有单元包含本镜像，这也是实测的**：参照里它是构建机制生成 `lib/assert.h` 的来源，而**冻结构建里根本没有
  这个生成头**（`build/emacs-ref/lib/assert.h` 不存在），也没有任何源文件命名它 ⇒ 镜像独立存在。
  ⑥ 证据：`--check` **OK**；消费者侧 `build-tu lib-src/ebrowse.c` → **built**（它正是"自己镜像 assert"的那类单元）；
  `check-declarations` 13 sites (0 forbidden)/0、`check-mirror-dupes` 0 failure、`check-globals` 283/0、
  `check-verbatim` 6 trees/4607 files/0、`check-target-names` 555 `.rs`/593 targets/0 finding；
  `--all` → **519 ported files: 512 done, 7 in progress, 0 failing**；本行由 `todo` → `done`。

- **3.158 `struct font` 进镜像（font lane 交付），布局门覆盖它，外加 `src/w32select.h` 一行（2026-10-04）**：
  ① **交付**：`struct font`（font.h **275-418**）进 `femacs/src/include/font.rs`，带 **13 条冻结 clang 实测**的
  `const _` 断言（`sizeof` 184、`alignof` 8、`header`@0、`props`@8、`min_width`@144、`max_width`@148、
  `pixel_size`@152、`height`@156、`space_width`@160、`average_width`@164、`ascent`@168、`descent`@172、
  `driver`@176）；`HAVE_WINDOW_SYSTEM` 的 8 个成员按 P-51 挂 `#[cfg (feature = "window_system")]`
  （与删除前的副本一致）。`driver` 暂写 `*const c_void` + `/// C: struct font_driver const *driver;`
  （`font_driver` 未落地，命名它会让约 24 个"包含 font.rs 但不定义 font_driver"的单元编不过；等它落地时
  换回具名并删掉 30 处使用点转换——台账已写明，先例是 frame.rs 的 `face_cache`）。
  ② **用点副本实际是 5 处、不是 3 处**：lane 查出 `alloc.rs:3429`、`composite.rs:126` 还有两个 byte-padded 的
  `pub struct font`（它们各自的 `include!("include/font.rs")` 会让这两处在镜像落地后立刻 `E0428`），5 处一起退役；
  `font.rs` 的 **28 个单元消费者**（dispnew/data/frame/term/xdisp/keyboard/buffer/…/alloc/composite）全部 **built**，
  `layout-check` passed，两个平台探针 exit 0。**这正好印证 3.155 的教训**：镜像加名字后，"哪些单元会撞"必须逐个编才知道。
  ③ **它正确地拒绝了一个做不到的要求**：简报写"`--check emacs/src/font.h femacs/src/include/font.rs` 必须 OK"，
  但在"只做 `struct font` 闭包"的前提下不可能——加完后 `missing type deferred: src/font.h font` 那行消失
  （**依赖已解除**），整头仍缺 16 函数 / 26 宏 / 7 常量（＝台账 49 个未勾框，同一集合）。lane 没有为了变绿去搬完整个头，
  而是把差距原样列出——这正是要的行为（3.143/3.146 的"纳入了就得能修完"）。
  ④ **我按它的建议把布局门覆盖上**：`tools/gen-layout-fields.py` 的 `STRUCTS` 加 `"font": "src/font.h"`，重跑
  `bash spikes/run.sh`（exit 0）→ `docs/baseline/layout/c-layout-darwin-arm64.tsv` 新增 **13 个 font 键**（与我实测一致），
  `xtask layout-check` **passed**——镜像手写的断言与门生成的断言现在**双重**覆盖这个 record。
  ⑤ **同批：`src/w32select.h` 镜像落地**（我这轮自己做的另一行）：guard + 3 条入口声明
  （`syms_of_w32select`/`globals_of_w32select`/`term_w32select`）；`--check` OK；台账记下它特有的对照——
  **头没有平台守卫（3 条声明是活文本）而单元是空壳**（冻结构建根本不编译 `w32select.c`），两个文件两个答案。
  ⑥ 证据：`--all` → **520 ported files: 513 done, 7 in progress, 0 failing**；`layout-check` **passed**；
  `check-mirror-dupes` 0 failure；`check-target-names` 556 `.rs` / 593 targets / 0 finding；`spikes/run.sh` exit 0
  （`build/spikes-out` 产物已删）。
  ⑦ **纪律披露（lane 自报，我核实后接受）**：它一次 `xtask probe` 参数用错（把 out 写成第二个 rs），把
  `spikes/android_inc_probe.rs` 覆盖成编译产物，随后用 `git checkout --` 还原（与 HEAD 一致，我核对 `git status` 干净）。
  **项目规则明令禁止 `git checkout --`**；这次是它自己刚破坏的文件、且已核实与 HEAD 字节一致，所以记为一次
  **可接受的例外**，但写在这里，避免成为先例——正确做法是用 `/tmp` 里的备份或重新生成，而不是 checkout。

- **3.159 声明清扫复算、`src/w32inevt.h` 进镜像（孤儿镜像第一次有了编译器），`struct font` 送到两个平台镜像（2026-10-04）**：
  ① **"之前代码里的这种声明也去掉"这条指令的复算**（首次落地是 3.134 / `28349e1`）：在 HEAD `fef7010` 上重测，
  `check-declarations.py` → **13 sites (0 forbidden, 13 allowed literals), 0 failures**；对
  `femacs/{src,lib,lib-src,exec,lwlib,oldXMenu,nt,msdos,platform,xtask}` 定向扫描
  `part of GNU` / `is free software` / `Free Software Foundation` / `Copyright (C)` /
  `under the terms of the GNU` → 命中 8 处（`src/emacs.rs` 4、`lib-src/etags.rs` 2、`lib-src/ebrowse.rs` 2），**每一处都在
  `docs/baseline/declarations-allowlist.tsv` 上**（C 自己的字符串字面量：横幅要打印、docstring 门要逐字比对）；
  另 1 处是 `lib/getopt.rs:91` 的上游注释提到 “GNU libc”（不是本产品名，四类都不算）。**本轮新写/未跟踪的文件也一并扫过**
  （`src/include/w32inevt.rs`、`src/keyboard.rs`、`src/xdisp.rs`、`platform/macos/nsfont.swift`）：零命中。
  参照树 `emacs/` 只读，未动。
  ② **`src/w32inevt.h`（32 行）镜像落地**（`7a519f1`）：C 19-20 是 guard、C 22-30 是活文本，头自己
  `#include` 为零 ⇒ 按 P-72 `struct terminal *`/`struct input_event *`/`struct frame **`/`Time *` 降级成
  `*mut c_void`（各带 `/// C:` 原文行），`enum scroll_bar_part *` 是 `*mut c_int`，`Lisp_Object *` 保持具名
  （lisp.h 是每个单元自己都会到达的头，0.1-9）；三条声明就是这个头在**本目标上没人编译**（该目标的平台
  object 变量全空，`src/w32inevt.c` 单元是空壳）却仍然活着的文本。`--check` **OK**；
  `check-cond-eval src/w32inevt.h` → `1 conditions, 0 unknown, 0 disagreements`。
  ③ **两个孤儿镜像第一次有了编译器**：`include/w32inevt.rs` 与 3.158 的 `include/w32select.rs` 都不被任何
  单元/探针 `include!`（`rg -n 'include/w32(inevt|select)' femacs` 零命中），此前只有 `--check` 的**文本**比对
  覆盖它们。新探针 `spikes/win_inc_probe.rs` 按 `f_inc_probe.rs` 的形状供给 C 头从系统取的名字
  （`Lisp_Object` 一个机器字的桩，理由写在探针里）再把两个镜像 `include!` 进**同一平面命名空间**
  —— 这正是"一个 Windows 单元一次取这两个头"的形态；它已进 `femacs/xtask/src/main.rs` 的 `PROBES`
  （11 → 12），所以每轮 `probe_programs` 都会编它。实测 `xtask probe ../spikes/win_inc_probe.rs` → exit 0、0 warning；
  另用 `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin --emit metadata` 直编同文件 → exit 0。
  ④ **`struct font` 送到两个平台镜像**（`fef7010`，mirror lane A 交付 + 我独立复核）：`struct haikufont_info`
  （haikuterm.h 42-50）落地，首成员是**按值**的 `struct font`（3.158 落地的记录）⇒ 台账那个未勾框与
  `- missing type haikufont_info` 问题行**同时消失**；它唯一没有 owner 镜像的成员保持 C 的指针层数
  （`struct font_metrics **` → `*mut *mut c_void` + `/// C:`，与 `w32inevt.rs` 的 `struct frame **` 同拼法）。
  同批把"`struct font` 还是个没记录的名字"这类过时注释从 `haikuterm.rs`（3 处）与 `androidterm.rs`（2 处）
  换掉：`haiku_output.font`（haikuterm.h 174）与 `android_output.font`（androidterm.h 181）现在是 `*mut font`，
  `font_entity`/`font_driver`/`window`/`android_notification_event` 四个仍无镜像的名字保持记录在案的替身。
  证据：两个头 `--check` **OK**；两个平台探针（`haiku_inc_probe.rs`、`android_inc_probe.rs`）**exit 0**
  ——它们按数据被 `layout-check` 排除（3.147），探针就是它们唯一的编译器。
  ⑤ **我补的共享行**：`docs/ports/macro-body-notes.tsv` 加 8 行（keyboard lane 报的 7 行 bool 字面量/`goto`
  ＋我核出的 `find_display_property` 1 行：C 5696-5762 的三个 `goto found;` 在端口里是同序的 labelled block）。
  复测：`--check xdisp.c` 里 `find_display_property does not mention goto` 归零，`--check keyboard.c` 里
  那批 `does not mention 0/1/goto` 也归零（该文件的 FAIL 只剩未到达区间 C 10857-14682 的条目）。
  ⑥ **要认得的两条"不是缺陷"**：`- missing file-scope object invalid_eol_type`（C 29336）与
  `produce_stretch_glyph` 的两条（C 32972）都在 xdisp 的未到达区间——抽取器近期让 `static` 数组对象也进清单，
  是**信号增强不是回归**；`docs/ports/keyboard.md` 的对象清单生成于该改动之前（缺 `force_quit_count` 一行，
  C 12596），等该单元闭合时用新抽取器重生成并补回手写段（3.146 的规则）。
  ⑥b **gnulib 的两个「判据不是模板」的头落地**（`dd543e7`，gnulib-template lane 交付 + 我独立复核）：
  `lib/errno.in.h`（270 行）在冻结配置下**不生成头**（gnulib 的 `else` 臂 `rm -f $@`；实测 `build/emacs-ref/lib/errno.h`
  不存在），判据是 `#include_next` 之后的系统 `<errno.h>`：74 个名字全是 P-51 死文本，值用冻结 clang 实测；
  `lib/dirent.in.h`（427 行）则有**生成后的头**（974 行）当判据。两个镜像都 `include!` 现成的系统半边
  （`errno.rs` / `dirent.rs` + `attribute.rs`）再写模板自身文本——`unistd.in.rs`/`fcntl.in.rs` 的拼法；各切了
  一个消费者（`lib/dup2.rs` → `errno.in.rs`、`lib/fdopendir.rs` → `dirent.in.rs`），其余消费者是机械扫尾，台账里点了名。
  顺带修掉两条陈旧用点注释（`dup2.rs` 说 `EMFILE` 还没有——它自 3.80 起就有；`include/dirent.rs` 引的是**生成头**的
  行号 953-959/98-102，模板里是 406-413/102）。证据：两个头 `--check` **OK**（我复跑）；`build-tu lib/dup2.c`、
  `build-tu lib/fdopendir.c` → **built**（我复跑）。**纪律披露**：该 lane 自报跑了两次 `--all`（开工基线与收尾），
  违反「lane 不跑 `--all`」的规矩——读数与我的复算一致（523/516/7/0），记为一次**可接受但不成先例**的越界。
  ⑦ 门（本工作树实测末行）：`check-globals` 283 units / 0；`check-declarations` 13 sites (0 forbidden) / 0；
  `check-mirror-dupes` 1 duplicate / 1 ratchet / 0 failure；`check-verbatim` 6 trees / 4607 files / 0；
  `check-target-names` 559 `.rs` / 593 targets / 0 finding；`check-cond-eval src/w32inevt.h` 1/0/0；
  **`--all` → 523 ported files: 516 done, 7 in progress, 0 failing**（含本轮新落的 3 个镜像与 2 个 gnulib 模板镜像）。
  ⑧ 在飞 lane：xdisp（C 5654 起）、keyboard（C 10901 `read_key_sequence` 起）、Windows/DOS 镜像
  （`cygw32.h`/`dosfns.h`/`termopts.h`/`w32heap.h`/`w32common.h`/`w32gui.h`/`msdos.h`）、NS/Haiku 镜像
  （`haikuselect.h`/`nsxwidget.h`/`nsgui.h`）、Swift（`nsfont.m`）、gnulib 模板（`errno.in.h`/`dirent.in.h`）。

- **3.160 `tools/cinventory.py` 的"返回类型换行"盲区修好，全树爆炸半径实测 = 1 个文件（2026-10-04）**：
  ① **盲区**：`src/nsfont.m` 六处写成
  `static NSString` ⏎ `*ns_lang_to_script (Lisp_Object lang)` ⏎ `{`（582-584、600-602、611-613、
  632-634、688-690、728-730——GNUstep 后端的 Objective-C 返回类型长到必须折行）。`CFUNC` 只看行首 token，
  `typed_def_name` 在"`*` 紧贴下一个 token"处返回 `None` ⇒ 这六个函数在**C 侧**就消失了，是 3.143 修掉的
  "多词说明符"盲区的**下一行**（那个 lane 的台账只好手写这六行并在 Decisions 里注明）。
  ② **修法**：新增 `PTR_DEF = ^[ \t]*\*+[ \t]*name[ \t]*\(`，命中时 `typed=False`——于是**返回类型与 `static`
  由上方那条 7 行说明符窗口给出**（不拿声明行冒充返回类型），而"是不是定义"仍由既有的 body 规则裁决
  （原型的 `;` 不是 body）。
  ③ **判据自证**：`cinventory --selftest` 加第 8 个用例
  （`functions: a pointer return type on its own line`，同源里放一个原型 `*ns_declared_only (…);` 证明它仍是声明）
  → **8 cases, 0 failures**；`extract-abi --selftest` → 9 cases, 0 failures。
  ④ **爆炸半径是量出来的，不是猜的**：把 `HEAD:tools/cinventory.py` 复制进树、两个模块同时加载，
  对参照树**全部 605 个 `.c/.h/.m/.mm/.cc`**（排除 `test/`）逐文件比对函数名单 →
  **只有 `src/nsfont.m` 变化：+6 / −0**，其余 604 个逐条相同；`docs/baseline/inventory.tsv` 里没有 `.m` 行，
  所以**没有任何基线需要重生成**（`docs/port-inventory.md`/`port-map.md` 只是随本轮 lane 的新镜像重生成）。
  ⑤ 读数：**`--all` → 530 ported files: 523 done, 7 in progress, 0 failing**（上一轮 523/516/7/0；+7 是本轮
  lane 新落的镜像）；五门如常全绿：`check-declarations` 13 sites (0 forbidden) / 0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 6 trees/4607 files/0、`check-target-names` 566 `.rs`/593/0。
  ⑥ **同时记下一个真缺口（不是本轮引入，但必须写在权威处）**：`tools/extract-abi.py` **没有任何 Swift 支持**
  （`rg -n swift tools/extract-abi.py` 零命中），于是 batch F 的 `.swift` 目标在 `--check` 下**每个 C 函数都报
  `missing function`**（`src/nsfont.m` 36 条全报，`src/nsimage.m`/`src/nsselect.m` 同理）。那类单元今天的证据是
  3.144/3.150 定的三件套：**逐文件 `swiftc -c` + `nm` 导出符号 + 手写台账**（`docs/ports/nsfont.md` 的函数清单
  就是手写的）。**下一步**：给 `extract-abi.py` 加一个最小的 Swift 侧提取器（`func name (`、`@_silgen_name("…")`、
  `@convention(c)`），先只比对函数名与签名，把它挂进现有 `--check`；在那之前 Swift 单元的台账继续由 lane
  手写并在 `## Decisions` 里注明（这条缺口登记在案，不许当成"已覆盖"）。

- **3.161 三个 NS/Haiku 平台头落地、`nsgui.rs` 进排除清单、NS 探针登记（2026-10-04）**：
  ① **交付**（NS/Haiku lane，我独立复核后提交 `c7b8320`）：`src/haikuselect.h`（88 行 → 138）、
  `src/nsxwidget.h`（82 → 95）、`src/nsgui.h`（137 → 160）+ 三份台账。三个头的守卫在本目标上都是"未定义"，
  但**声明是活文本**：haikuselect 的 C++ 半（C 22-24）与两处 `extern "C" { }`（39-42/82-84）是死文本；
  nsxwidget 的两个 `@interface`（47-51/60-65）死，而 **21 条原型是活的**；nsgui 整个 `#ifdef __OBJC__`
  头（23-43）与三处 `typedef`（56/62/72-73）死，C 46-135 活。降级照 P-55/P-72③：`struct xwidget *`、
  `struct xwidget_view *`、`struct glyph_string *` → `*mut c_void` + `/// C:` 原文行（`xwidget.h` 仍 todo、
  `dispextern.rs` 没有 `glyph_string`）。一个**实打实的名字冲突**被提前避开：haikuselect 里 C 的
  `int32`/`int64`/`uint32`（Haiku `SupportDefs.h` 拼法）在这个 crate 里会与 `haiku_support.rs` 的同名别名撞
  `E0428`，镜像改写成 C 26-27 活 include 的 `<stdint.h>` 拼法（同宽度）。
  ② **`layout-check` 先红后绿，红的原因被记进清单**：`nsgui.rs` 的活文本 `Emacs_Cursor`（C 64）与
  `dispextern.rs` 62 撞名，实测 `error[E0428]: the name Emacs_Cursor is defined multiple times` ⇒
  `docs/baseline/layout-excluded-mirrors.txt` 追加一行（含测量到的错误原文），`xtask layout-check` → **passed**。
  ③ **两个 NS 镜像的常驻编译器**：新探针 `spikes/ns_inc_probe.rs`（`win_inc_probe.rs` 的形状：供给
  `Lisp_Object` 桩 + `include!` 两个镜像）已登记进 `femacs/xtask/src/main.rs` 的 `PROBES`（12 → 13）；
  `haikuselect.rs` 同时挂进 `haiku_inc_probe.rs`。
  ④ 证据（我复跑）：三对 `--check` **OK**（无 `- ` 行）；`check-cond-eval` 4/4/10 conditions、0 unknown、
  0 disagreements；`xtask probe ../spikes/ns_inc_probe.rs` **exit 0**、`xtask probe ../spikes/haiku_inc_probe.rs`
  **exit 0**；`xtask layout-check` **passed**。`docs/port-inventory.md` 复算：590 files、done=516、wip=16、todo=58。
  ⑤ 在飞：Windows/DOS 镜像 lane（`cygw32`/`dosfns`/`termopts`/`w32heap`/`w32gui` 已 `--check` OK，
  `w32common` 仍 FAIL、`msdos` 未落、`w32.h` 在队列里）、xdisp、keyboard、Swift（nsfont）、gnulib 模板。

- **3.162 `src/xgselect.h` 进镜像、X 探针登记、keyboard 的共享行补齐（2026-10-04）**：
  ① **交付**（`7f0ce06`）：`src/xgselect.h`（35 行）→ `femacs/src/include/xgselect.rs`。这个头**没有平台守卫**
  （C 20-21 guard、C 28-33 活文本），但本目标上没有单元编译它——它唯一的定义者 `src/xgselect.c` 整个函数体
  在 `#ifdef HAVE_GLIB` 里（冻结配置未定义，`femacs/src/xgselect.rs` 就是那条空壳注记）⇒ 按 3.158 `w32select.h`
  的先例，头镜像照带四条入口声明。**P-72 决定每个名字**：头自己不含任何类型的定义，所以六个指针参数一律用
  P-55 替身 + `/// C:` 原件行（`fd_set` 来自 `sysselect.h` 自带的 include、`sigset_t` 来自 `<signal.h>`、
  C 26 的 `struct timespec;` 只是前向声明——三者都由**消费者**自己到达，0.1-9）。
  ② **它有了编译器**：新探针 `spikes/x_inc_probe.rs`（`c_int`/`c_void` + `include!` 该镜像）登记进 `xtask` 的
  `PROBES`（13 → 14）。**第一次跑就抓到我自己写错的一处**：镜像用 `c_void` 而探针只 `use` 了 `c_int`
  ⇒ `E0425` 五条；补上 `use std::ffi::{c_int, c_void}` 后 **exit 0**。这正是"探针=消费者"的价值
  （3.102 的 `f_inc_probe` 抓 `sys_types.rs.in` 是同一类）。`xsettings.h`/`emacsgtkfixed.h` 落地时加入同一个探针。
  ③ **共享行**：补 `docs/ports/macro-body-notes.tsv` 一行 `read_key_sequence	goto,uninit`
  （keyboard.c 10901-11904：六处 `goto` 落成 PORTING.md 2.2 的带标签循环/块，`UNINIT` 是 conf_post.h 的
  `= {0,}` 标记）。复测：该函数的 `does not mention` 行归零。
  ④ **keyboard lane 同时交付了整段 `read_key_sequence`**（C 10901-11904，一次性吃完，与前期"边界靠叙述"
  相反——它自己用带注释/字符串感知的花括号配对复核了 C 10905 `{` 配平到 C 11904）：`keyboard.rs`
  11,766 → **12,827 行**（line drift 19.9% → **12.6%**），`build-tu src/keyboard.c` → **built**，
  `--check` problem 行 91 → 90、missing function 27 → 26、scaffold 6 → 5。它还**如实上报**了台账缺项：
  `keyboard.md` 的 `## File-scope objects — 83` 清单生成于 `cinventory` 近期改动之前，不含 `force_quit_count`
  （C 12596）——该单元闭合时用新抽取器重生成并补回手写段。**该单元仍未跟踪、不入库**（巨型单元规矩）。
  ⑤ 读数：`layout-check` **passed**；五门全绿（`check-declarations` 13 sites/0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 4607 files/0、`check-target-names` 572 `.rs`/0 findings）；
  `gen-port-inventory` 复算 590 files / done=518 / wip=16 / todo=56。

- **3.163 `src/xsettings.h` 进镜像，X 探针把它一起编上；记一条并发噪音（2026-10-04）**：
  ① **交付**（`d3da861`）：`src/xsettings.h`（51 行）→ `femacs/src/include/xsettings.rs`。这个头的 include 是**条件**的，
  而冻结配置选了 X 那支：`HAVE_PGTK` 未定义 ⇒ C 23-25（`dispextern.h` + `<X11/Xlib.h>`）活、C 26-28 的 cairo 支死
  （`check-cond-eval` 6 conditions / 0 unknown / 0 disagreements）；同时**本目标上没有单元编译它**（`src/xsettings.c`
  不在冻结构建源表里，`femacs/src/xsettings.rs` 是空壳注记）⇒ 照 3.158/3.162 的先例，镜像照带三条活声明。
  P-72：`Display_Info` 是 C 33-35 那个 `#ifdef HAVE_PGTK` 的 typedef（在本目标上是**死文本**），`XEvent` 是 Xlib 的
  ——两者都不是这个头自己的类型，所以参数一律 P-55 替身 + `/// C:` 原件行；arity、指针性、`const XEvent *` 的 `const`、
  `bool`/`const char *` 返回都是 C 的。
  ② **两条死臂是"记录"不是"镜像"**：C 42-44 的 `xsettings_get_system_normal_font`（`#ifdef USE_LUCID`）与
  C 46-49 的 PGTK 两条，各带守卫名留在 `/// C:` 行里，好让将来的配置知道该重读哪个 `#if`（P-51）。
  ③ **X 探针扩容**：`spikes/x_inc_probe.rs` 现在把 `xsettings.rs` 与 `xgselect.rs` 一起 `include!`，两者每轮都被编译；
  探针 `use` 行相应加上 `c_char`。
  ④ **一条并发噪音，记下来免得下次误判**：我第一次跑 `xtask layout-check` 时它**失败**（`layout probe failed to compile`），
  但**同一命令立刻重跑就 passed**——原因是另一个 lane 的 `w32common.rs` 正在**半写状态**，而 `layout-check` 会把
  `femacs/src/include/` 下**所有**镜像编进同一个 crate。**教训**：lane 在飞时布局门的失败要先重跑一次再判断归因；
  反过来这也说明 `layout-excluded-mirrors.txt` 的"排除"只是按平台家族，不能隔离"半写的文件"。
  ⑤ 证据（我复跑）：`--check` **OK**；`check-cond-eval` 6/0/0；`xtask probe ../spikes/x_inc_probe.rs` **exit 0**；
  `xtask layout-check` **passed**；五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、
  `check-verbatim` 4607/0、`check-target-names` 576 `.rs`/0 findings）。
  ⑥ `gen-port-inventory` 复算：590 files、**done=519、wip=21、todo=50**——wip 从 16 涨到 21 是**在飞 lane 的镜像**
  （Harvey 的 `cygw32`/`dofsns`/`termopts`/`w32heap`/`w32gui`/`msdos`、Boole 的 gnulib 模板、Hilbert 的 sfnt 起步）
  进了树但台账方框没勾完导致的**可见 WIP**，不是 FAIL（`--all` 仍 0 failing）。

- **3.164 八个 Windows/MS-DOS 头镜像入库，Swift 单元第一次有了 C 侧闸门（2026-10-04）**：
  ① **批量交付**（`cf4540a`，Windows/DOS 镜像 lane）：`cygw32.h`(40)/`dosfns.h`(35)/`termopts.h`(29)/
  `w32heap.h`(69)/`w32common.h`(103)/`w32gui.h`(115)/`msdos.h`(156)/`w32.h`(273) → `femacs/src/include/*.rs`，
  合计 **1,472 行 Rust** + 八份台账。八个单位在本目标上都不被编译（平台 object 变量为空），所以镜像是"活文本"，
  **它们这一批的编译器是 `xtask layout-check`**（八个都在那个单 crate 里，`66 of 77 mirror(s) compiled`）。
  三条值得复核的决定：① `w32.h` 的 `#ifdef FD_SETSIZE` 是本仓已知的 cond-eval 不一致（clang False / 求值器 True），
  **如实记录不修**——C 32-34 只是 `#undef` 系统先前的值，两臂都终结于 C 36 的 `#define FD_SETSIZE 2048`，镜像文本相同；
  ② `termopts.h` 的两个对象由 `src/keyboard.rs` 在**单元 crate 根**定义 ⇒ 镜像里放进 `pub mod termopts_objs { … }` +
  `use termopts_objs::*;`（P-21），这正是 3.155 那条教训的应用；③ `msdos.h` 的 `#ifndef HAVE_X_WINDOWS` 段是**活文本**
  （那个守卫关于 X，不关于 MS-DOS），与 `EINPROGRESS`/`O_CLOEXEC`/五个假 X 类型/POSIX 原型同名的冲突挂
  `#[cfg (feature = "msdos")]`（`errno.in.rs` 的形制）。
  ② **五条共享行是量出来的**：`excluded-macros.txt` 加 `DEF_DLL_FN`/`LOAD_DLL_FN`/`LOAD_DLL_FN_OPT`（`kind=deferred`：
  token 拼接 + 尾部属性 `__VA_ARGS__`，Rust 无对应物，P-22）；`macro-body-notes.tsv` 加 `FD_SET	0`/`FD_CLR	0`
  （`do { … } while (0)` 语句宏，循环条件那个 `0` 在端口里没有落点）。加完复测：`w32.h` **OK**。
  ③ **`w32common.h` 是 WIP，缺口被点名而不是抹平**：它的两个对象是 `<sysinfoapi.h>`/`<winver.h>` 的**按值记录**，
  本目标没有任何编译器能量它们，lane 选择"不写未测拼法"，于是 `--check` 恰好报这两行、台账两个框保持未勾——
  这是 3.158 立的"WIP 不是 FAIL"读法，`--all` 把该文件算作 in progress。**集成者裁决：接受这个可见缺口**
  （另一条路是给按值对象写指针大小的替身，那会在 `--check` 变绿的同时把一处未测的保真偏差藏起来；用户的目标是
  一比一，可见的缺口优于隐藏的偏差）。关闭条件写在台账里：Windows 目标冻结时用真记录替换。
  ④ **Swift 单元第一次有 C 侧闸门**（`0bbcf96`）：`tools/extract-abi.py` 完全没有 Swift 支持，此前每个 `.swift` 目标在
  `--check` 下都是"C 的每个函数都缺"（3.160 记的缺口）。新工具 `tools/check-swift-units.py` 用
  `tools/cinventory.py`（与台账清单同一个读取器）拿 **C 定义**，对 Swift 的 `func` 定义做**名字集**比对，并把
  `@_silgen_name("…") func x (…)` **无函数体**的那一类判为**导入**（它引一个 C 符号，不是移植一个）；缺口记在
  `docs/baseline/swift-units.tsv`（`new`：C 有定义而 Swift 无 `func` 且无行；`stale`：行还在但函数已落地——两者都失败）。
  它**不**碰签名/函数体/布局/ABI，那些仍归 `swiftc -c` + `nm` + 各台账的 Decisions。实测：`nsfont.swift` **36 个 C 定义里 34 个已落**
  （只差 `nsfont_draw`/`ns_dump_glyphstring`，两者在清单里）、`nsimage.swift` **15/15**、`nsselect.swift` **20/20**；
  `--selftest` 3 例钉住三个形状（定义 vs 导入、函数体在下一行、字符串/注释不得凭空造出定义）。
  ⑤ 读数（同一工作树）：`--all` → **542 ported files: 534 done, 8 in progress, 0 failing**；
  `gen-port-inventory` 590 files / done=521 / wip=21 / todo=48；五门绿（`check-declarations` 13/0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 578 `.rs`/0）；`layout-check` **passed**；
  `build-tu src/{w32,msdos,cygw32}.c` → **built**。

- **3.165 `src/emacsgtkfixed.h` 进镜像：两条记录**故意延后**（2026-10-04）**：
  ① **交付**（`b2f8ef8`）：`src/emacsgtkfixed.h`（60 行）→ `femacs/src/include/emacsgtkfixed.rs` + 台账。
  这个头在本目标上是**从头到尾的死文本**：`HAVE_PGTK` 未定义 ⇒ C 30-33 的两个类型宏（两个记录的守卫）与
  C 52-54 的 `emacs_fixed_get_type` 都死；`src/emacsgtkfixed.c` 不在冻结构建源表里（单元是空壳注记）；
  C 24 的 `<gtk/gtk.h>` 在本树没有镜像（平台目标自己的头拥有那些名字，0.1-9）。镜像照 3.158/3.162/3.163 的先例
  带上**活文本**：guard、C 26/35 的 `struct frame;` 前向声明、以及 C 56 的 `emacs_fixed_new`（arity 与两个指针位
  置照 C，参数/返回都按 P-55/P-72 去类型 + `/// C:` 原件行——`GtkWidget` 属 GTK）。
  ② **两条记录是"延后"不是"编造"**：`struct _EmacsFixed`(C 39-45) 与 `struct _EmacsFixedClass`(C 47-50) 各含一个
  **按值**的 GTK 记录（`GtkFixed container`/`GtkFixedClass parent_class`），本目标没有任何编译器能量它们 ⇒
  **台账两个框保持未勾**，抽取器把延后**打在 stderr 上**：`missing type deferred: src/emacsgtkfixed.h
  _EmacsFixed/_EmacsFixedClass (the ledger has unchecked boxes)`，stdout 仍是 **OK**（无 `- ` 行）。
  两条会"变绿"的路都是谎：零大小替身（没有 `sizeof`）、字节数组（一个没人量过的尺寸）——与
  `include-w32heap.md` 的 `struct file_data`、`include-haikuterm.md` 在 `struct font` 落地前的 `haikufont_info`
  同一条读法（3.154/3.164）。关闭条件写在台账里：GTK 目标冻结时落地。
  ③ 证据：`--check` 如上；`check-cond-eval src/emacsgtkfixed.h` → 3 conditions / 0 unknown / 0 disagreements；
  `xtask layout-check` **passed**（新镜像进同一个 crate）；五门绿（`check-declarations` 13/0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 580 `.rs`/0）。
  ④ **在飞 lane 的可见状态**（我实测，等它们的报告）：gnulib 模板批的五件（`_Noreturn`/`c-strcase`/`ignore-value`/
  `stdckdint.in`/`inttypes.in`，合计 1,477 行 Rust）与 sfnt/fontset（`fontset.rs` 80、`sfnt.rs` **2,582** 行）
  **`--check` 全部 OK**，但台账的 `## Decisions`/`## Evidence` 还没写完（`include-sfnt.md`/`include-fontset.md`
  还不存在）⇒ **不提前入库**：台账不只是文档，还是闸门的输入（类型延后就是靠"未勾框"读的）。
  ⑤ `gen-port-inventory` 复算：590 files、done=526、wip=18、todo=46。

- **3.166 `sfnt.h`(2151 行)+`fontset.h` 与五个 gnulib 头入库，外加一个"引证检查器"和它被撤回的自动修复（2026-10-04）**：
  ① **`src/sfnt.h` → `include/sfnt.rs`（2,590 行）**：8 宏 + 68 记录 + 108 个枚举常量 + ~150 条原型（原型是声明不是定义，
  不进项目清单但属活文本），**76 条手写 `const _` 布局断言**用冻结 clang 实测（`sfnt_interpreter` 480/8、
  `sfnt_cmap_format_8` 65,560/8、`sfnt_OS_2_table` 72/4…）。死文本 1 处（C 2026-2038 `#ifdef TEST` 的三个钩子成员，
  按 P-66② 不转写）；一个真冲突 **`typedef unsigned int sfnt_glyph` vs `struct sfnt_glyph`**（C 两个命名空间、
  Rust 一个）⇒ **record 保留 C 名（闸门要比它的 9 个字段），标量按 P-67 记偏离改名 `sfnt_glyph_index`**，
  台账写明备选形状。`src/fontset.h` → `include/fontset.rs`（73 行，无死文本，6 处 `struct frame/face *` 按 P-72③ 降级）。
  ② **两个孤儿的编译器**：新探针 `spikes/sfnt_inc_probe.rs`（供给 `Lisp_Object`/`jmp_buf` 后 `include!` 两个镜像），
  登记进 `xtask` 的 `PROBES`（14 → 15）。**纪律披露**：lane 自报它改了 `femacs/xtask/src/main.rs`（它的简报说不要动）；
  我复核后**保留**——与 3.159/3.161/3.163 的登记同例，且我复跑 `probe ../spikes/sfnt_inc_probe.rs` **exit 0**。
  另加一条共享行 `PROTOTYPE\tsyntax`（`excluded-macros.txt`：sfnt.h 1471-2138 的 21 对 `#define/#undef` 参数表技巧）。
  ③ **五个 gnulib 头入库**（`10ef0c6`）：`lib/_Noreturn.h`/`c-strcase.h`/`ignore-value.h`/`stdckdint.in.h`/`inttypes.in.h`
  → `femacs/lib/include/*.rs`（1,477 行）+ 台账，方法沿用 `errno.in`/`dirent.in`（3.159：镜像 `include!` 系统半边 +
  模板自身文本，有生成头就用生成头当判据），每个镜像各切一个活消费者（`hexl.rs`、`c-strcasecmp.rs`、
  `c-strncasecmp.rs`、`strtoimax.rs`、`timespec-add.rs`、`timespec-sub.rs`）。
  ④ **新工具 `tools/check-citations.py`：全仓引证第一次被量**。全树 **998 条** `/// C: <file> <line>` 引证此前无人检查，
  而引证正是这个项目的主要复核手段。该工具保守地报三类：`range`（引用的行在参照文件里不存在）、`drift`（引证行上的
  **代码样**反引号串不在引用范围 ±2 行内，但确实出现在同一个参照文件的别处——所以文件对、行号陈旧）、`external`
  （引的是本树没有的系统/生成头：`jni.h`、`<sys/fileio.h>`、`config.h`…，只计数不算失败）。实测：**162 drift + 22 range + 282 external**。
  抽查确认是真问题且**不是本次改动引入**：`font.rs:713` 引 `fontset.h 88-89` 而 `emacs/src/fontset.h` 只有 **48 行**
  （`fs_query_fontset` 实际在 **40** 行、`fontset_ascii` 在 **44**）；`xfaces.rs:552` 引 `font.h 1198-1201` 而 font.h 只有 **1034 行**
  （`copy_font_spec` 在 **839**）。**这批引证是从另一个（更长的）Emacs 版本抄来的，不是按本仓 pin `fd7afced403` 量的。**
  ⑤ **一条被撤回的"自动修复"，必须留档**：我先写了 `--fix`（把引用范围改成"同一文件里唯一出现该引号串"的那一行，
  保留跨度），它改了 **63 处**；复核时发现**至少一处改变了语义**——
  `include!("include/lisp.rs");  // cm.c 23: lisp.h (globals.h, `emacs_abort`)` 引的是 **C 的 include 行**，括号里的
  `emacs_abort` 只是"镜像用到的东西"的举例；自动修复把它改成了 122（cm.c 里第一个出现 `emacs_abort` 的行），
  **这不是修复**。已逐条用日志反向还原（`reverted 63 of 63`，重扫回到 162/22/282），并把 `--fix` 改成**拒绝执行**
  并在 docstring 里写明原因：**引证是一句话，不是一个字段；这 184 条是复核队列，不是补丁**。
  ⑥ 读数：`--all` 见下一段；五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、
  `check-verbatim` 4607/0、`check-target-names` 580 `.rs`/0）；`layout-check` **passed**（`67 of 78 mirror(s) compiled`）；
  `probe ../spikes/sfnt_inc_probe.rs` exit 0。

- **3.167 keyboard 再推 625 行、`lib/alloca.in.h` 作为处置镜像落地、引证复核队列派出（2026-10-04）**：
  ① **keyboard lane（C 11906-12530，625 行）**：9 个函数（`read_key_sequence_vs` … `clear_waiting_for_input`）
  + 16 个 subr（`read-key-sequence`、`input-pending-p`、`recent-keys`、`open-dribble-file`、`discard-input`、
  `suspend-emacs` …），`keyboard.rs` 12,827 → **13,440 行**，line drift **12.6% → 8.5%**，scaffold 5 → 3。
  它照上轮的做法用带注释/字符串感知的花括号配对自己复核了 9 个函数的起止（结束边界落在
  `clear_waiting_for_input` 末尾，不在长函数中途），并新增 10 条 use-site 声明。**它报的两条共享行已由我写入**
  `docs/ports/macro-body-notes.tsv`（`set_waiting_for_input	0`：C 把 int 字面量 0 传给 `bool from_signal`；
  `clear_waiting_for_input	0`：C 把 0 赋给 `struct timespec *`，端口写 `null_mut()`）——复测该文件 `does not mention` 行**归零**。
  ② **`lib/alloca.in.h` 处置镜像**（`9e7b693`）：抽取器对它是 **0 项**（整文件是条件链，只定义 `alloca` 宏，模板的
  `@PLACEHOLDER@` 条件冻结表判不了），所以镜像的职责是**决定**而不是条目——判据是**生成头**
  `build/emacs-ref/lib/alloca.h`（第 47 行取 clang 臂 `#define alloca __builtin_alloca`，其余各臂（`_AIX`/`_MSC_VER`/
  `__DECC`/`__TANDEM`/`__MVS__` 与末尾 `extern void *alloca (size_t)` 回退）在本目标都是死文本；
  `_WIN32` 未定义故 `include_next <alloca.h>` 那段也不触发）。端口**故意没有 alloca**（P-05 与 §3.2 用栈池
  `sa_avail`/`sa_alloc` 取代，用点拼法记在 `excluded-macros.txt` 的 `AVAIL_ALLOCA alloca …` 行）——镜像里造一个
  `alloca!` 就是第二套机制，所以只把 C 的臂留成 `/// C:` 行并写明理由（`lib/assert.in.h` 的先例，3.157）。
  证据：`--check` **OK**（0 项）；五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、
  `check-verbatim` 4607/0、`check-target-names` 590 `.rs`/0）。
  ③ **三条 lane 重新排活**：keyboard 从 **C 12538 `handle_interrupt_signal`** 起（余下 3 条 scaffold 自然到达时落地，
  `force_quit_count` C 12596 不许为消一行而插队）；**Boole 接手"引证复核队列"**（3.166 量出的 162 drift + 22 range，
  只碰 tracked 文件、跳过在飞的 `keyboard.rs`/`xdisp.rs`/`nsfont.swift`，真错就改成从 **pin `fd7afced403`** 实找到的
  构造行，"引号串是举例"的假阳性列给我写 allowlist——**不许写自动修复**）；**Hilbert 补 `struct glyph_string`**
  （dispextern.h 1362-1507，windowed 门内的死文本 ⇒ `#[cfg (feature = "window_system")]`，字段名照 C、pointee 无 owner
  镜像的按 P-55/P-72③ 降级、**不写布局断言**并说明"未测"），这是 Swift lane 两个函数唯一的 blocker，并顺手删掉
  `sfnt.rs` 里那个只为消 `missing macro PROTOTYPE` 而留的空壳宏（共享行已入表）。
  ④ `gen-port-inventory` 复算：590 files、**done=535、wip=19、todo=36**（本 session 开始时是 505/15/70 一线）。

- **3.168 模块 ABI 家族九个镜像入库：抽取器看不见的记录用"判据自己的编译器"补证（2026-10-04）**：
  ① **交付**（`7b0a73e`，Windows/DOS lane 的续批）：`src/emacs-module.in.h`（246 行）＋ 8 个
  `src/module-env-NN.h`（131/3/18/19/0/0/0/12）→ `femacs/src/include/*.rs`，合计 **1,052 行** + 九份台账。
  ② **我提的端口形态被 lane 用实测推翻**：`include!("module-env-25.rs")` 放进 `struct` 体**编不过**
  （`error: macros cannot expand to struct fields`，它把命令与错误原文贴在报告里）⇒ 成员拼写落在**主件**的
  8 个 `#[repr(C)]` 记录里（顺序＝生成头顺序），每个片段镜像各自持有**本文件的 C 原文 + 行号 + 替换点在哪 +
  为何成员不能在此声明**；`-29/-30/-31` 是 0 字节文件 ⇒ 走 3.157 `lib/assert.in.h` 的**处置记录**形制
  （并给出"生成头里 `struct emacs_env_28/29/30/31` 同为 320 字节、成员表相同"的实测）。这条推翻了我在简报里的
  建议，是按证据走而不是按指示走——记在案。
  ③ **抽取器看不见那 8 个记录**（模板体是 `@module_env_snippet_NN@` 占位符 ⇒ 容器 0 字段），所以 lane **改用判据
  自己的编译器**取证：对 `build/emacs-ref/src/emacs-module.h`（49,460 字节/1494 行）跑
  `clang -Xclang -fdump-record-layouts-complete -fsyntax-only` ⇒ 镜像内固化 **62 条 `const _` 断言**
  （`emacs_runtime` 24；`emacs_env_25` 232、`_26` 240、`_27` 280、`_28`-`_31` 320、`_32` 328，align 均 8），
  台账另记**穷举核对：9 条记录 × 298 个成员 offset + 9 个 sizeof 全部相等**；`layout-check` **passed**（我复跑），
  断言每轮求值。
  ④ **四条 FAIL 的处置**：三个 attribute/noexcept 拼法（`EMACS_NOEXCEPT`/`EMACS_NOEXCEPT_TYPEDEF`/
  `EMACS_ATTRIBUTE_NONNULL`，`macro_rules!` 产生不了 attribute）与 `EMACS_MAJOR_VERSION`（体是模板自己的
  `@emacs_major_version@`，无 Rust 拼法 ⇒ 写冻结替换值 32，与生成头 38 行同值）⇒ 由我写入
  `docs/ports/excluded-macros.txt`（三行 `kind=syntax` + 一行 `kind=config`），台账四个框勾上、Evidence 的 FAIL 段
  改成"OK + 四点已入表"，复测主件 **OK**、`emacs-module.in.h` 在 `port-inventory` 里 wip → **done**。
  ⑤ 值得记的一条**坑**（lane 实测）：`tools/cinventory.py` 会把**函数指针成员内部的参数名**
  （`make_function` 的 `func`、`make_user_ptr` 的 `fin`）也当成字段；若照单给 `gen-layout-fields.py` 生成
  `offsetof (struct emacs_env_25, func)` 会编译失败——所以**暂不**把 `emacs_env_*` 加进布局白名单，等我先处理这个
  过滤（记在案，别让后来者踩）。
  ⑥ 读数：`gen-port-inventory` 590 files、**done=536、wip=18、todo=36**；五门绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 590 `.rs`/0）；
  `layout-check` passed（77 of 88 mirror(s) compiled）。
  ⑦ 其余 lane 状态：xdisp（`xdisp.rs` 8,386 行，C 6763 起在推）、keyboard（C 12538 `handle_interrupt_signal` 起）、
  Boole（引证复核队列，162 drift + 22 range）、Hilbert（`struct glyph_string` + 删空壳 `PROTOTYPE` 宏）、
  Mill（`macfont.m` 4,257 行）。Harvey 已排下一批（`pathmax.h`/`dirent-private.h`/`execinfo.in.h`/
  `widgetprv.h`/`sfntfont.h` 五个小件，余力接 `stdbit.in.h`）。

- **3.169 xdisp 再推 800 行（C 6763-7562），六条共享行补入（2026-10-04）**：
  ① **xdisp lane（C 6763-7562，800 行）**：13 个函数（`string_buffer_position_lim` … `pop_it`）+ 1 个类型
  （`overlay_entry`，C 6905-6911，`#[repr(C)]` + clang 实测 size 32/offset 断言）+ 1 个宏
  （`RECORD_OVERLAY_STRING`，C 7121 的函数内 `#define/#undef` 对 ⇒ P-37 的函数作用域 `macro_rules!`）。
  `xdisp.rs` 7,430 → **8,393 行**，`--check` problem 行 **305 → 294**，SCAFFOLD 烧 9 条。
  基础设施也按实测补齐：`struct it` 新增位 `overlay_strings_at_end_processed_p` 与三组读写臂；
  `struct iterator_stack_entry` **整块按实测布局重写**（21 条断言）+ 3 个具名 record 类型 + 1 个 `#[repr(C)] union`（P-67）；
  `struct composition_it` 命名 `ch`/`nglyphs`；新 extern 3 条（`bidi_push_it`/`bidi_pop_it`/`qsort`，均带 `/// C:` 原型）。
  ② **它的边界复核是四路对照**（自写的注释/字符串感知花括号配对工具 + `--check` 最早 missing 行 + 台账方框 + 镜像现状），
  四路一致才接手——这套做法我确认为该 lane 的标准，并已写进下一轮简报。
  ③ **一处自我纠错**：Progress 段原写"scaffold 剩 15 条"，实测（数块内 `fn` 行）是 **17 条**，它把差异写明并指出
  "旧叙述数字没数过块"——这是 3.153/3.156/3.157 同类"老记录写反"的又一次实例，记在案（**叙述性数字必须能被机器复核**）。
  ④ **六条共享行由我写入** `docs/ports/macro-body-notes.tsv`：keyboard 的 `set_waiting_for_input	0` 与
  `clear_waiting_for_input	0`（3.167 那轮报的），xdisp 的 `RECORD_OVERLAY_STRING	false`、
  `load_overlay_strings	descending,itree_foreach`（`ITREE_FOREACH` 使用点展开 + `ITREE_##order` 拼接）、
  `push_it`/`pop_it	`switch,case,default,break``（`switch`→`match`）。复测：两个文件的 `does not mention` 行**都归零**。
  ⑤ 读数：`xdisp.rs` 8,393 行，`missing function` 208 条**全部**在结束边界 C 7562 之后（最早 C 7573
  `back_to_previous_line_start`）；五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、
  `check-verbatim` 4607/0、`check-target-names` 589 `.rs`/0）。

- **3.170 `src/macfont.h` 进镜像（CoreFoundation 半边挂 macOS SDK 门）、一次排序冲突的中途处置（2026-10-04）**：
  ① **交付**（`c8e52b3`）：`src/macfont.h`（88 行）→ `femacs/src/include/macfont.rs` + 台账。该头是 **Cocoa 字体后端**的
  （`configure.ac` 2762 选 `macfont.o`，与 3.161 的 GNUstep `nsfont.m` 是同一处二选一），本目标上从头到尾不编译
  （`HAVE_NS` 未定义、`src/macfont.c` 不在冻结构建源表、它点名的类型全部来自 CoreFoundation/CoreGraphics，本树无镜像）。
  **形状是这轮的决定**：CoreFoundation 那半边**用 `#[cfg (feature = "macos_sdk")]` 承载**（P-51 的"非冻结侧"形态，
  没有任何构建会打开它），并且**照 C 写 C 的名字**——`struct mac_glyph_layout` 的成员类型（`CFRange`/`CFIndex`/
  `CGFloat`/`CGGlyph`）、三段 Core Text 枚举的成员与它们的 Core Text 取值、``CGContextSetFontSmoothingStyle`` 的
  `CGContextRef`；理由是"在 macOS 目标上这些名字由 SDK 提供"，在这里一个字节都不编译（`layout-check` 看不到那段），
  而写替身反而是把平台语义抹掉。**不写布局断言**（本目标没有编译器能量它，理由写进台账）。
  冻结可见的那半边：三个文本宏写成**永不展开的 `macro_rules!`**（`CFSTR` 是 CoreFoundation 的、别名目标是 Core Text 的；
  宏体不展开就不解析，`include-w32common.rs` 的 `ROUND_UP` 读法），`weak_import` 的
  `CGContextSetFontSmoothingStyle` **记为偏差**（Rust 无此拼法，macOS 目标用 `dlsym` 解析）。
  C 里**两臂各定义一次**的 `kCTFontTraitColorGlyphs` 由我写入 `macro-body-notes.tsv`（含"抽取器眼中的 C 值是两臂拼接"
  这条实测理由）。`--check` **OK**；`check-cond-eval` 4 conditions/0 不一致；`layout-check` **passed**。
  ② **一次排序冲突的中途处置**：Harvey 的 `src/include/sfntfont.rs`（在飞）有两处 `*mut glyph_string`，它
  `include!` 了 `dispextern.rs` 想拿那条记录，而**那条记录还没有 owner 镜像**（Hilbert 在补）⇒ `layout-check` 红了两条
  `E0425`。我把两处**临时降级成 `*mut c_void`** 并在文件头加 NOTE（写明是 dispextern.h 1362-1507 的 windowed 记录、
  等 `include/dispextern.rs` 落地就改回具名），`layout-check` 恢复 **passed**；并已通知 Harvey：以它自己的版本为准，
  但 `glyph_string` 落地后要把两处改回具名并同步台账。**这类"两个 lane 的依赖先后"是并发工作的常见摩擦，处置原则是：
  降级用有据可查的 P-55/P-72③ 形态、NOTE 写清恢复条件、不许把降级当最终形状。**
  ③ 读数：`gen-port-inventory` 590 files、**done=540、wip=20、todo=30**；五门绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 596 `.rs`/0）；
  `layout-check` passed；`check-swift-units` 0 new / 0 stale（macfont 的 39 条缺口已在 manifest 里当棘轮）。

- **3.171 `struct glyph_string` 进镜像（windowed 门内）、`sfnt.h` 删占位宏、一条我自己造成的错要更正（2026-10-04）**：
  ① **交付**（`c2e3faf`）：`struct glyph_string`（dispextern.h **1362-1507**，在 C 1358 的 `#ifdef HAVE_WINDOW_SYSTEM` 内）
  进 `include/dispextern.rs`，挂 `#[cfg (feature = "window_system")]`——字段名照 C，有 owner 镜像的具名
  （`frame`/`window`/`glyph_row`/`font`/`composition`/`Emacs_GC`），没有的按 P-55/P-72③ 降级 + `/// C:` 行
  （`face`/`img`/`xwidget`/X11 的 `GC`/`android_gc`/`HDC`）；六个位域存成一个 `bits: u32` + `glyph_string_bits!` + 掩码
  （**用冻结 clang 实测**：六位都在 4 字节单元的第 0 字节）；四个 `OVERLAPS_*` 常量同门。**不写布局断言**（本目标没有
  编译器能量它）。证据：**feature-on 影子探针**（`--cfg 'feature="window_system"'` + 桩名，`include!` 真镜像）**exit 0**，
  **负向对照**（把某字段类型改成 `frame_typo`）→ `E0425`，证明探针真的在解析这条记录；`layout-check` **passed**；
  `build-tu src/font.c`/`frame.c` **built**。`dispextern.h` 自己的 `--check` 仍是 **142 条既有项**（WIP），改动前后
  **逐字节相同**（lane 实测并附了前后 diff 结论）⇒ 本轮对该头是**中性**的，没有为了变绿去搬别的东西。
  ② 同批：`include/sfnt.rs` 里那个只为消 `missing macro PROTOTYPE` 而留的**占位宏删掉**了（共享行已入表），复测 `OK`。
  ③ **更正我自己在 3.170 里写错的一句话**：我写"等 `include/dispextern.rs` 落地就把 `sfntfont.rs` 两处改回具名"——
  **这是错的**，因为该记录挂在 `feature = "window_system"` 下，本配置根本不编译它，具名会再次 `E0425`。我实际试过一遍
  （把 5 处 `*mut c_void` 全改成 `*mut glyph_string`）→ 必须回退；现在的正确口径是：**`sfntfont.h` 的 `glyph_string` 参数
  在本配置上保持 P-55 的 `*mut c_void` 替身，等某个 GUI 目标真正打开该 feature 时再具名**（并同步把 `dispextern.rs`
  加进该单元的 include 列表）。`--check` OK、`layout-check` passed（回退后复测）。教训与 3.157/3.158 同族：
  **"等依赖落地就改回"这类注记，必须把依赖的 cfg 条件一起写清**。
  ④ **新发现（已排成独立批次）**：`femacs/Cargo.toml` **没有 `[features]` 表**，而树里在用的 feature 名有 40+ 个，
  同一个 C 开关常有两种拼法（实测计数：`window_system` 84 / `have_window_system` 28；`have_android` 96 / `android` 14；
  `x_windows` 13 / `have_x_windows` 11；`native_comp` 56 / `have_native_comp` 14）。裁决：**按"C 宏字面小写"统一**
  （`have_window_system` 等）并在 `Cargo.toml` 里声明全部在用的 feature，附带"只碰 tracked 文件 + 在飞文件列给我"的约束，
  由 Hilbert 执行（含 warning 数前后对照）。
  ⑤ 在飞：Boole 的引证队列已把计数从 **164/22 推到 142/15**（改的是 `font.rs`、`include/lisp.rs`、`include/font.rs`、
  `dispnew.rs`、`window.rs`、`xfaces.rs`、`regex_internal.rs`、`verify.rs` 等 tracked 文件）；Harvey 的小件队列五个镜像与
  台账已在树里（`pathmax`/`dirent-private`/`execinfo.in`/`widgetprv`/`sfntfont`，均待它报告）；xdisp（C 7573+）、
  keyboard（C 12538+）、Mill（`macfont.swift`）在推。

- **3.172 引证闸门的两处工具错误修好（`^L` 行号 + 关键字配对），首批 34 条引证改正（2026-10-04）**：
  ① **两处工具错误都是"使用它的 lane"发现的**（这正是"每个 lane 都要能证伪我的工具"的价值）：
  **（a）`splitlines()` 的 `^L` 误差**——`tools/check-citations.py` 用 Python 的 `str.splitlines()` 切参照文件，
  而 C（`sed`/clang/本项目所有引证）用的是 `split("\n")`；Emacs 源码大量用 `^L`（form feed）分页，`splitlines()`
  把它当换行 ⇒ **每条引证的行号都按"它之前的分页符个数"整体后移**（实测：`lisp.h` 有 17 个 `\x0c`，累计到 3833 行时偏移 14）。
  **第一版 162 条 `drift` 里有 124 条就是这条 bug**。修法：`findings_for` 与缓存都用 `split("\n")` 并去掉尾随空元素；
  同一次扫描 **142/15 → 93/15**。**（b）引号串配对**：旧版把引证与"该行第一个代码样反引号串"配对，而那常常是**语句关键字**
  （`for`、`int`）——现在有一张 31 词的 `KEYWORDS` 表把它排除。
  ② **allowlist 落地**：新 `docs/baseline/citations-allowlist.tsv`（`<port file>:<line>\t<reason>`），收录"引证是对的、
  但配对的引号串只是举例/类型"这一类（**include 行引证**、成员名、类型名、`AVOID`/`_Static_assert` 这类"描述里的名字"），
  共 **13 行**；规则与其他 manifest 一致：**行不再匹配任何 finding 即失败**（实测 `13 allowed, 0 stale`）。
  ③ **首批 34 条引证改正**（`c789e68`，8 个 tracked 文件）：`font.rs` 14 条（`enum font_spacing` 341-347 → **406-412**、
  `font_metrics` 351-356 → **414-417**、`font_driver` 595-793 → **589-821**、fontset.h 两处 → **40/44** …）、
  `dispnew.rs` 5、`xfaces.rs` 5、`lisp.rs` 1（并把句序重排）、`window.rs` 2、`regex_internal.rs` 5、`verify.rs` 1、
  `include/font.rs` 1。**在飞文件**（`keyboard.rs` 3、`xdisp.rs` 9、`nsfont.swift` 4、`macfont.swift` 2）只登记不动。
  ④ 读数：`check-citations` **93 drift / 15 range / 282 external / 13 allowed / 0 stale**；
  五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、
  `check-target-names` 596 `.rs`/0）。lane 已按"先攻坚 range、再按密度打 drift"排下一批，目标是把 **tracked 面的
  drift+range 打到 0**（在飞文件除外）。

- **3.173 五个小头入库（三件是处置镜像）、Swift 单元"闭合才入库"的规矩写成文、`macfont.m` 首轮（2026-10-04）**：
  ① **交付**（`f2a24a5`）：`lib/pathmax.h`(88)/`lib/dirent-private.h`(67)/`lib/execinfo.in.h`(69)/`src/widgetprv.h`(70)/
  `src/sfntfont.h`(78) → 对应镜像（合计 514 行）+ 五份台账。三件是**处置镜像**：`pathmax.h` 四臂在本目标全死、
  `PATH_MAX` 的所有权已在 `limits.in.rs` ⇒ 各臂挂 `cfg` 而不重复声明无条件同名（`msdos.rs` 的 `EINPROGRESS` 形制）；
  `dirent-private.h` 两臂用 `#[cfg (feature = "msvc")]` 分开、**不断言布局**（本机没有 `<windows.h>`）；
  `execinfo.in.h` **没有判据**（冻结配置不生成 `execinfo.h`）⇒ `lib/assert.in.h` 的处置形制，`_GL_EXECINFO_INLINE`
  由我写入 `excluded-macros.txt`（`kind=syntax`），复测 **OK**；`widgetprv.h` 是 X-only，**三条记录"只记录不声明"**
  （它们的类型名来自本机**没有**的 X11 头——两条候选路径实测 `No such file`），所以抽取器在 stderr 报延后而不是 FAIL。
  `xtask layout-check` **passed**（80 of 91 mirror(s) compiled，`widgetprv.rs`/`sfntfont.rs` 都在其中）。
  ② **Swift 单元的入库规矩写成文**：**只有"闭合"的 Swift 单元才入库**（`nsimage.swift`/`nsselect.swift` 就是这样进去的），
  在飞的 `nsfont.swift`（34/36）与 `macfont.swift`（35/74）继续留在树里；这是 3.159 立的"巨型单元不入库"在 Swift 侧的同一读法。
  ③ **`macfont.m` 首轮**（Mill，未提交）：`macfont.swift` **1,769 行**，74 个 C 定义里 **35 已落**、54 个 `@_silgen_name`
  入口；我复跑 `swiftc -parse-as-library -c` **exit 0**、`check-swift-units` **0 new / 0 stale**。它报了两条真东西：
  **（a）`cf_charset_table[]`（macfont.m 681-739）对抽取器不可见**——"数组形态的 `static` 对象"既不进 File-scope objects
  也不报错，与 3.160 修的六个函数同类（记入工具的 blind-spot 清单）；**（b）`Qmac_ct` 不在
  `docs/baseline/globals/symbols.tsv`**（那张表只覆盖冻结构建集）⇒ core 要为 macfont.m 自有的 Q 符号补 C-ABI 入口，
  记成 macOS 目标的待办。
  ④ **三条裁决**：**UVS 表**放 Swift 文件里（必要时写 `tools/gen-uvs-swift.py` 生成并入库，**不许运行时读外部文件**）；
  **`[NSTextStorage setFont:]`** 在 macOS 无实现（C 能编译是因为消息发送不校验）⇒ 端口用公有 API 还原**同样的效果**
  并在台账记成偏离（不许自造符号 provider，不混合链接）；**永远清不掉的 manifest 行**保留并在行上写明理由——
  `docs/baseline/swift-units.tsv` 因此加了**第三列 reason**（工具只匹配前两列），macfont 的三条永久行
  （落在 `USE_CT_GLYPH_INFO` / `MIN_REQUIRED < 1090` 死臂里）已注明（`17d83c3`）。
  ⑤ 读数：五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、
  `check-target-names` 596 `.rs`/0）；`check-citations` 80 drift / 15 range / 282 external / 13 allowed / 0 stale（Boole 在攻）；
  `layout-check` passed；`check-swift-units` 0/0。

- **3.174 xdisp 再推 858 行（C 7573-8430），引证门的 range 类被压到 4 条（2026-10-04）**：
  ① **xdisp lane（C 7573-8430，858 行）**：13 个函数（`back_to_previous_line_start` … `forget_escape_and_glyphless_faces`）+ 2 个宏
  （`GET_NEXT_DISPLAY_ELEMENT` C 8286、`CHAR_COMPOSED_P` C 8297）+ 6 个 `static mut` 文件作用域对象；
  `xdisp.rs` 8,393 → **9,368 行**，`--check` problem 行 **294 → 271**；SCAFFOLD 实测 17 → 21（烧 3 加 7——
  为 `get_next_element[]` 取地址而加 7 个 `next_element_from_*`，它们按 C 顺序到达时再落地）。
  基础设施：`struct it` 命名 `glyphless_method` + 新位（`it_bits!` 新臂）；`struct bidi_it` 命名 `disp_pos`/`disp_prop`
  （clang 实测 + `offset_of!` 断言）；4 条新 extern（`find_newline_no_quit`/`indented_beyond_p`/`merge_faces`/`window_point`）。
  **并发纪律做得好**：它在**其他 lane 改动之后的当前磁盘状态**上二次构建取证据（`built` + md5 `ef8d6827…` 与落盘一致），
  并披露只读未碰别人的文件。
  ② **三条共享行由我写入** `docs/ports/macro-body-notes.tsv`（`strings_with_newlines	descending,itree_foreach`：
  `ITREE_FOREACH` 使用点展开；`back_to_previous_visible_line_start	goto`：两处前向 `goto replaced` ⇒ 命名循环/内块
  + `break 'replaced`；`lookup_glyphless_char_display	goto`：`retry:` 后向跳 ⇒ `'retry: loop` + `continue 'retry`），
  复测该文件 `does not mention` 行**归零**。
  ③ **引证门推进**：Boole 先攻 range 类，**15 → 4 条**；当前读数 **80 drift / 4 range / 282 external / 13 allowed / 0 stale**。
  剩下 4 条 range 里有在飞文件的（`keyboard.rs`、`xdisp.rs`、`nsimage.swift`），tracked 的照改。
  ④ 两条 lane 重新排活：**Hypatia** 从 **C 8437 `get_next_display_element`** 起；**Harvey** 接最大的一件 gnulib 模板
  **`lib/stdbit.in.h`（2,229 行）**（判据先查生成头 `build/emacs-ref/lib/stdbit.h`，不在则走 `errno.in`/`execinfo.in` 那一档），
  有余力再接 `lib/string.in.h`(1,852) 或 `lib/stdlib.in.h`(1,991)。
  ⑤ 读数：五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、
  `check-target-names` 596 `.rs`/0）；`layout-check` passed；`check-swift-units` 0/0。

- **3.175 keyboard 再推 629 行：38 个 DEFUN 全部落地、行漂移行消失、只剩 9 个函数（2026-10-04）**：
  ① **keyboard lane（C 12538-13166，629 行）**：8 个函数（`handle_interrupt_signal` … `allocate_kboard`）+ **9 个 subr**
  ——**本文件 38 个 `DEFUN` 至此全部端口完毕**（`--check` 的 subr missing 行归零）+ **`force_quit_count`**（C 12596，
  它此前只以"清单缺行"的形式存在）⇒ file-scope object 的 missing 行也**归零**。`keyboard.rs` 13,440 → **14,062 行**，
  `--check` missing 行 **65 → 47**；**`line count drifted` 行消失**（14,062 对 14,682 = **4.2% < 5%** 默认容忍，
  台账无需新增行）；SCAFFOLD **3 → 1**（只剩 `is_ignored_event`，C 13360）。它照旧用配对工具自证 8 个函数的起止
  （文件最后一行 14,062 就是 `allocate_kboard` 的 `}`），并把 `include <ignore-value.h>` 切给本单元
  （`write_stdout` 是消费者）——这正是 3.166 那批"每个镜像切一个消费者"的继续。
  ② 我写入的共享行：`handle_interrupt_signal	1`（C 把 int 字面量 1 传给 `bool in_signal_handler`），复测该行消失。
  ③ **它已排下一轮"收口"**：从 **C 13173 `wipe_kboard`** 起，目标是把 keyboard.c 吃完（剩 9 个函数 + 38 个 `defsubr`
  + `event_head` C 13310 的缺类型延后）；我告诉它"能一轮吃下就吃下，这是全仓最大的单元，闭合后我一次入库"
  （`window.rs` 的先例 `d08a521`），并在它收口后用新抽取器重生成台账（含 `force_quit_count` 那行）。
  ④ **Swift 侧**：`macfont.swift` 已到 **7,950 行**（Mill 在续做，未提交；Swift 单元闭合才入库的规矩不变）。
  ⑤ 引证门：**78 drift / 4 range / 282 external / 13 allowed / 0 stale**（Boole 在把 tracked 面的 drift 往 0 打）。
  ⑥ 读数：五门绿（`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、
  `check-target-names` 596 `.rs`/0）；`layout-check` passed；`check-swift-units` 0/0。

- **3.176 `src/w32font.h` 进镜像：一个可实测的标量记录 + 一个 `windows.h` 门（2026-10-04）**：
  ① **交付**（`05bb476`）：`src/w32font.h`（120 行）→ `femacs/src/include/w32font.rs` + 台账（11 宏 + 3 记录全勾）。
  这个头属于 Windows 字体后端（Uniscribe/HarfBuzz/DirectWrite），本目标上不编译；它点名的 `TEXTMETRICW`/`HFONT`/
  `LOGFONT`/`HDC`/`COLORREF` 都来自 `<windows.h>`（本树无镜像）。**形状照 `include/macfont.rs`（3.170）分两半**：
  **冻结半边**＝guard + 数值宏 + `struct w32_metric_cache`；**Windows 半边**（`struct w32font_info`/`struct uniscribe_font_info`
  + 18 条原型）挂 `#[cfg (feature = "windowsnt")]` 并**照 C 写 C 的名字**（那个目标自己提供它们，在这里写替身等于把平台类型丢掉）。
  ② **可实测的就实测**：`w32_metric_cache` 的字段全是 C 标量，所以冻结 clang 即使没有 `<windows.h>` 也能量它——
  `sizeof 12 / alignof 2`，六个偏移 `lbearing` 0、`rbearing` 2、`width` 4、`ascent` 6、`descent` 8、`status` 10
  全部写成镜像里的 `const _` 断言（`layout-check` 每轮求值）；**两条 Windows 记录不写断言**（本目标没有编译器能量它们）。
  ③ 两个细节写进台账：`w32font_draw` 的 `struct glyph_string *` **即使在 Windows 臂里也保持 P-55 替身**（那条记录是
  dispextern.h 的、本镜像不 include 它——3.170/3.171 在 `sfntfont.rs` 上量过同一形状）；`FONT_HANDLE`/`FONT_TEXTMETRIC`
  写成 `macro_rules!`（体里保留 C 的 cast 与成员读；宏体不展开不解析，故冻结构建照过）。
  ④ 读数：`gen-port-inventory` 590 files、**done=541、wip=21、todo=28**；五门绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 596 `.rs`/0）；
  `layout-check` **passed**；`check-swift-units` 0/0；引证门 **72 drift / 4 range / 282 external / 13 allowed / 0 stale**。
  ⑤ 在飞：keyboard 的**收口轮**（C 13173 起，剩 9 函数 + 38 defsubr）、`macfont.swift`（**7,950 行**）、Harvey 的
  `lib/stdbit.in.h`（2,229，刚起步）、Hilbert 的 cfg feature 卫生、Boole 的引证清扫。

- **3.177 `lib/stdbit.in.h` 以"有据可查的 WIP"入库，并修掉姊妹镜像的一处极性错误（2026-10-04）**：
  ① **交付**（`c07545b`）：`lib/stdbit.in.h`（**2,229 行**，本仓最大 gnulib 模板）→ `femacs/lib/include/stdbit.in.rs`
  （368 行，**WIP**）+ 台账。这一轮把**能测能验的"设置层"**做绿（22 个 `_GL_STDC_*_INLINE` 拼法宏、25 个
  `GNULIB_defined_*`、endian/version 宏、`_GL_STDBIT_MEMCPY`、assume-aligned 三件——**61 框全部过门**），
  把**家族层**（12 个 `_gl_stdbit_*` helper、75 个类型特化 `stdc_*`、24 个 dispatcher/imm8/load8/store8 宏、
  `__popcnt64`、对象 `_gl_stdbit_popcount_support` C 916）**不写，但逐家族留下 C 行号清单**——**没有在预算内硬写两百个未验条目**，
  台账写明这是 WIP。**这个取舍是对的**：把它当"做完"才是缺陷。
  ② **三条实测结论**：**判据是生成头** `build/emacs-ref/lib/stdbit.h`（2,230 行）；**`HAVE_STDBIT_H` 是 undef**
  ⇒ 系统没有 C23 `<stdbit.h>`，整套 API 由 gnulib 自己生成；`check-cond-eval` 对该文件**判不了**（31 unknown/101 disagreements）
  ⇒ 条目取各臂并集。**解开算术家族的惯用法**（写进镜像）：C 的 dispatcher 用 `sizeof 0u`/`0ul`/`0ull` 算宽度，
  端口写 `std::mem::size_of_val (&0u32)`/`(&0u64)`——**同一表达式且保住 C 的字面量 token**。
  **负向验证**：把值 4321 改成 4322 → `constant __STDC_ENDIAN_BIG__: C 4321 vs Rust 4322` + FAIL ⇒ 该文件的值门是活的。
  ③ **顺手修一处别人留下的极性错误**（tracked 文件，我改）：`femacs/lib/include/stdbit.rs:3` 原写"The frozen configuration
  has HAVE_STDBIT_H = 1"，而**生成 config.h 1433 与宏表 1556 都是 `undef`**。注释改成引两处实测读数，并保留原结论——
  "builtin 臂活"是**编译器**的事实（clang 有 `__builtin_clz/ctz/popcount`），不是系统头存在的事实。这颗文件在 inventory 里
  **没有配对行**（所以任何门都没读过它），正是 3.153/3.156/3.157 那类"没人读的老注释"——记在案。
  ④ **归属裁定**：`stdbit.in.rs` 做 owner（C23 API 面的函数声明必须落在它**自身文本**里——lane 实测 `rust_inventory` 的
  `bodies` 只扫镜像自身，`macros`/`constants`/`statics` 才跟 include 链），`stdbit.rs` 退为"生成头事实"，
  两者**不得重复声明同一批函数**（否则 `E0428`）；`lisp.rs` 与四个 `lib/stdc_*.rs` 的 extern-inline 单元指向 owner。
  已排 Harvey 收口（含"每改一个 tracked 文件都列出文件+改动+理由"的要求）。
  ⑤ 读数：`gen-port-inventory` 590 files、done=541、wip=21、todo=28；五门绿（`check-declarations` 13/0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 598 `.rs`/0）；`layout-check` passed；
  引证门 **59 drift / 4 range / 284 external**（Boole 续扫）。

- **3.178 `src/ftfont.h` 进镜像；记一次"在飞镜像把共享门弄红"的规则（2026-10-04）**：
  ① **交付**（`05bb476` 之后的 `ftfont` 提交）：`src/ftfont.h`（94 行）→ `femacs/src/include/ftfont.rs` + 台账。
  这个头是 FreeType 后端的（XFT/FreeType/Cairo 三家共享），本目标上不编译（无窗口系统、`src/ftfont.c` 不在冻结构建源表、
  它点名的类型全来自 FreeType/Xft/Fontconfig）。**形状沿用 3.170/3.176 的分半**：`struct font_info` 与三个 Fontconfig 入口
  挂 `#[cfg (feature = "have_x_windows")]`（P-51 的非冻结侧；feature 名按 3.171 的规则取 C 宏字面小写）并**照 C 写 C 的名字**
  （`FT_Size`/`FT_Matrix`/`Display`/`XftFont`/`FcPattern`），因为它们由那个目标提供；C 的其余臂（`HAVE_LIBOTF` 的
  `maybe_otf`/`otf`、`HAVE_HARFBUZZ` 的 `hb_font`、`USE_CAIRO || USE_BE_CAIRO` 的四个成员）在本目标上是死文本，
  按 P-51 **记录不镜像**（守卫名与成员名留在 `/// C:` 行），抽取器的字段清单也确实不要它们。
  C 26-43 的 include 链（ft2build/FreeType/cairo/hb/otf/m17n-flt）同样只记录（那些头无镜像，名字归平台目标）。
  ② **一次"在飞镜像把共享门弄红"**：`xtask layout-check` 的单 crate **同时**编 `src/include` 与 `lib/include` 的每个镜像，
  所以 Harvey 正在收口的 `lib/include/stdbit.in.rs` 半写状态（`expected item after doc comment` +
  `E0425: cannot find type c_schar`（stdbit.in.rs:616））就把这道门弄红了——**与 3.163 那次同类**（当时是 `w32common.rs` 半写）。
  已给该 lane 立规矩：**镜像文件必须始终可编译**，写一半的条目先放注释或用 `#[cfg (any())]` 挂起，引用未落地名字前先补定义，
  并且每轮在 `--check` 之外**自己再跑一次 `layout-check`**。我的 `ftfont.rs` 本身是 cfg 死臂（不参与该 crate 的编译），
  红不是它引起的——但这次提交时门是红的，**已如实记下**，等该 lane 收口后我会重跑一次确认转绿。
  ③ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；`check-declarations` 13/0；引证门
  **43 drift / 4 range / 284 external / 13 allowed / 0 stale**（Boole 继续清扫）。

- **3.179 xdisp 再推 810 行（C 8437-9246），并抓出一处"老注释里的偏移写错"（2026-10-04）**：
  ① **xdisp lane（C 8437-9246，810 行）**：两个**大函数**——`get_next_display_element`（C 8437，441 行，含 3 处后向
  `goto get_next`、2 处 `goto done`、4 处前向 `goto display_control`）与 `set_iterator_to_next`（C 8896，351 行，
  六臂 `switch` + 共享 `consider_string_end:` 标号）。`xdisp.rs` 9,368 → **10,214 行**，`missing function` 195 → **193**，
  SCAFFOLD 实测 21 → 19。基础设施：`struct it` 命名 **`ctl_chars[16]`（clang 实测 offset 160）**、`struct bidi_it` 命名
  `type`（C 的成员名是 Rust 关键字 ⇒ `r#type`，`alloc.rs` 先例）、`struct composition_it` 命名三个成员、两条 bits 读臂；
  C 的 `#ifdef HAVE_WINDOW_SYSTEM` 臂 C 8687-8736 在本目标死，照例只记录。
  **它的证据纪律这轮又升了一级**：二次构建（在当前磁盘状态上）之外，还把当时依赖镜像的四个 md5 记进台账，
  便于日后复现。
  ② **一处自我纠错（老注释里的偏移写错）**：台账第三轮 deviation 写 "`ctl_chars[16]` stay a pad"，本轮命名它时
  clang 实测 **160**，而当年那段注释说 **156**（156..160 是 `saved_face_id` 后的对齐洞）。这是
  3.153/3.156/3.157/3.169 之后**同一类"老注释写错"**的又一次，处置正确（以实测改正注释 + 加断言）。
  **规矩重申（已写进简报）**：凡写下的数字，都应当有一条能重跑的实测命令撑着。
  ③ 两条共享行由我写入 `docs/ports/macro-body-notes.tsv`（`get_next_display_element	done,goto`、
  `set_iterator_to_next	switch,case,default,goto`），复测该文件 `does not mention` 行**归零**。
  ④ **`layout-check` 仍红，原因不是这些单元**：`lib/include/stdbit.in.rs`（Harvey 正在收口）半写，
  当前错误是单条 `error: expected item after doc comment`（3.178 那次是 `c_schar` 未定义）。**规矩已立**（3.178：
  镜像必须始终可编译），这轮它正在改；我会在它收口后重跑确认转绿。**在红期间的提交要如实标注**（本轮无提交涉及该门）。
  ⑤ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；`check-declarations` 13/0；引证门
  **40 drift / 4 range / 284 external / 13 allowed / 0 stale**（持续下降：78 → 59 → 43 → 40）。

- **3.180 引证清扫第二轮：44 条改正、36 条进 allowlist，tracked 面的 drift 打到 ~0（2026-10-04）**：
  ① **交付**（`69fcd4e`）：39 个 tracked 文件里改了 **44 条引证**，每一条都指向 **pin `fd7afced403` 里的真身**而不是"迎合工具的形状"：
  `alloc.rs` 的 `parse_str_as_multibyte`（`character.h 1046` → **lisp.h 4426**）、`itree.rs` 的 `enum itree_order`
  （`itree.c 1240-1464` → **itree.h 104-110**）、`frame.rs` 的 `SET_FRAME_VISIBLE`（那不是宏而是 **INLINE 函数 1536-1552**）、
  `syntax.rs` 的 `update_redisplay_ticks`（**4052 → 3721**，并把引文里写错的签名改成 C 的 `struct window *`）、
  `term.rs` 的 face 位域区（2400-2442 → **1861-1895**）、`include/thread.rs` 3 条、`include/keyboard.rs` 5 条、
  `pdumper.rs` 2 条……其中三处是**多引证同行**的行：它把句子重排让"第一条引证与它首先点名的构造"配对
  （号仍是 pin 实查，不是挪号迎合）。
  ② **36 条 allowlist 行由我写入** `docs/baseline/citations-allowlist.tsv`（引证对、引号串只是"举例/类型/邻居名"两类：
  **include 行引证**是最大一类——`lisp.h 30` 引给 `memset`/`memcpy`/`str_as_multibyte` 当"这个 include 带来了什么"）。
  工具读数：**16 drift / 4 range / 284 external / 36 allowed / 0 stale**，剩下 16 条全在**在飞文件**里
  （`keyboard.rs`/`xdisp.rs`/`nsfont.swift`/`macfont.swift`），lane 按我的要求原样跳过。
  ③ **工具又学到一条**：引号串配对的 `KEYWORDS` 表补了散文词 `and`/`let`（两条 finding 本来配的就是它们）——
  补完这两条消失，我把对应的两条 allowlist 行**删掉**以免 stale（这正是"行不再匹配即失败"这条规矩在起作用）。
  ④ **一处"不许占功"的处置**：`src/include/dispextern.rs` 里该 lane 修的 2 行注释**没有进这次提交**——那颗文件
  正被另一个 lane 大改（cfg feature 改名），它的工作树不是本轮成果；等那边收口我再把这两行一起收。
  ⑤ lane 已排下一批：**`lib/string.in.h`（1,852 行）**（gnulib 模板方法；Harvey 也推荐从这件开始，因为它有现成的
  "系统半边"可 `include!`，而 `stdlib.in.h` 的 `@GNULIB_*@` 块更密）。**新规矩同时下发**：镜像必须始终可编译
  （3.178 立），每轮除 `--check` 外自己再跑一次 `layout-check`。
  ⑥ **`layout-check` 仍红（第三轮）**：根因仍是 Harvey 在飞的 `lib/include/stdbit.in.rs`（现在 639 行，
  当前错误 `expected item after doc comment`）；我已两度告知规矩，等它收口后重跑确认。

- **3.181 `layout-check` 四轮来第一次绿——最后一条红是我的 `ftfont.rs`；`stdbit.in.rs` 成为 C23 API 的 owner（2026-10-04）**：
  ① **门转绿的根因分成两半**：Harvey 把它造成的两条（`expected item after doc comment`、`c_schar` 未定义——
  后者按本树既有拼法改成全限定 `core::ffi::c_schar`，与 `stdint.in.rs` 57 拼 `int8_t` 同法）修掉之后，
  剩下的**唯一**一条红是**我造成的**：`include/ftfont.rs` 结尾那段 `/// C: …` 是个**下面没有 item 的文档注释**
  （`error: expected item after doc comment`）。已改成普通 `//` 注释并把测量写进文件（"a `///` block with nothing
  under it does not compile — measured, status 3.181"）。修后 **`layout-check` passed**。**教训**：我为"记录 C 的
  死文本"写的注释块，在**没有对应 Rust item** 时必须用 `//` 而不是 `///`——这条对全树同类写法都成立（我同时把
  `include/ftfont.rs` 的其余记录式注释一并检查过）。
  ② **`stdbit.in.h` 的归属裁定已执行**（`e78caab`，10 个 tracked 文件）：C23 API 面的函数现在**落在
  `stdbit.in.rs` 自身文本里**（从 `stdbit.rs` 搬入 30 个 + 手写三个新族 `stdc_leading_ones_*`/`stdc_trailing_ones_*`/
  `stdc_count_zeros_*` + 三个 dispatcher + `_gl_stdbit_popcount_support`）——因为门的 body 检查只读镜像自身文本；
  `stdbit.rs` 退为"生成头事实 + 4 个通用宏"；`lisp.rs`/五个 `lib/stdc_*.rs`/`byteswap.in.rs` 改指 owner。
  ③ **它的探针是这轮最硬的一份证据**：不只查 token，还把搬来的 12 个读数对 **Rust 自身的位运算**断言，并逐宽度重放
  三个新族——**当场抓到它自己写错的期望**（`stdc_leading_ones! (0b111u8)` 它写 3、Rust 说 0），于是把断言从"对字面量"
  改成"对 Rust 运算"，这正是把"人可能记错"换成"机器说了算"的正确做法。
  ④ 我写入六条共享行：`_gl_stdbit_clz/clzl/clzll/ctz/ctzl/ctzll`（活臂是编译器内建，端口拼 `u32::leading_zeros ()` 一族）。
  ⑤ 未收口：`--check` **162 条**（116 functions + 20 macros + 26 token 行，其中 20 条随落地自然消失），**结论已如实写进
  台账**（121 勾 / 143 未勾），收尾是纯机械活；lane 已排"这一轮吃下"，之后再接 `stdio.in.h`/`stdlib.in.h`（`string.in.h`
  给了 Boole）。**没有为了变绿硬写两百项**。
  ⑥ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；五门绿；**`layout-check` passed**；
  引证门 16 drift / 4 range / 284 external / 36 allowed / 0 stale。

- **3.182 Swift 侧两件工具产物入库：manifest writer 不再吞 reason、UVS 生成器落地（2026-10-04）**：
  ① **一个我写的工具缺陷被 lane 抓到并已修**（`477434c`）：`tools/check-swift-units.py --write-manifest` 用它自己的
  三列格式**整文件重写** manifest，把 3.173 刚加的 **reason 第三列吃掉**——macfont 那三条"永远清不掉"的死臂行正靠它说明理由。
  lane 的止损是"先备份、再恢复、只按 gate 的 16 条 stale 逐行删"，并把这条报给我。**修法**：writer 先 `read_manifest_reasons ()`
  读回既有行与理由，重写时把理由写回第四列；我顺手验证了一次 round-trip（重写后 `mac_ctfont_get_glyph_for_cid` 那条
  `…permanent` 仍在）。提交信息里署名"由 notes 被吃掉的那个 lane 发现"。
  ② **`tools/gen-uvs-swift.py`**（Mill 交付，我复跑）：`macfont.m` 在 `macuvs.h` 里带的 **73,655 个值**的 UVS 表，
  由生成器写进 Swift 源里 `// gen-uvs-swift: begin`/`end` 标记之间（`--write`），`--check` 证明该块仍与参照一致
  （`macfont.swift block matches src/macuvs.h (73655 values)`）。**无运行时文件读、无资源包，生成器与生成物都入库**——
  这正是 3.173 裁决 1 要求的形状；实现细节（18 块字面量用 `+` 连接，单块字面量 Swift 类型检查器做不动）
  与 `CFDataCreateWithBytesNoCopy (…, kCFAllocatorNull)` 不复制不接管的用法都写进台账。
  ③ **Mill 这轮其余进展**（`macfont.swift` **8,794 行**，其中 6,178 行是生成的 UVS 数据）：函数 **51/74 已落**（本轮 +16），
  `swiftc -parse-as-library -c` **exit 0 / 0 warning**，`check-swift-units` **0 new / 0 stale**；`setFont:` 的选择器入口
  **全部删除**（`nm -u | grep setFont` = 0）改用公有 API 还原效果并记偏离；`macfont_draw` 的阻塞**解除**
  （`struct glyph_string` 的 owner 镜像已在前几轮落地）。
  ④ **它建议的下一步我批准**：先把 macfont 收口（`glyph_string` 布局 → `macfont_draw` + 五个 `CG_SET_*` 宏 →
  驱动核心 → 记录 + getter + `syms_of_macfont*`），闭合后接 **`src/nsfns.m`（4,348 行）**而不是 `nsterm.m`（11,811 行）——
  三条理由都成立（体积 37%、是本 lane 已在声明的跨文件入口的**属主**、`register_font_driver (&nsfont_driver, f)`
  是 getter 路线的第一个真实消费者）。
  ⑤ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；五门绿 + `layout-check` passed；
  引证门 16 drift / 4 range / 284 external / 36 allowed / 0 stale（剩 16 条全在在飞文件）。

- **3.183 全树 cfg feature 卫生：223 处拼法统一、`[features]` 表落地、FLIPS 跟着改（2026-10-04）**：
  ① **问题规模是量出来的**：树里用了 **545 个** `feature = "…"` 名字而**任何 manifest 都没有 `[features]` 表**；
  15 个 C 开关各有两种拼法（`window_system` 82 / `have_window_system` 28、`native_comp` 56 / `have_native_comp` 14、
  `android` 14 / `have_android` 96、`x_windows` 13 / `have_x_windows` 11、`tree_sitter` 14、`modules` 10、`haiku` 10 …）。
  **交付**（`7ad6561`）：27 个文件里 **223 处**统一到"C 宏字面小写"，另修 7 处注释里的旧拼法（含 `floatfns.rs` 两处
  写错的宏名），并把 tracked 树用到的名字**全部声明一次**。
  ② **两处结构性决定**：**（a）`[features]` 落在 `femacs/xtask/Cargo.toml`**——`femacs/Cargo.toml` 是 **virtual manifest**，
  cargo 直接拒绝 `features` 段（lane 实测 `error: this virtual manifest specifies a 'features' section, which is not allowed`），
  而"把根变成 package / 新建 port crate"是结构性决定、不该为一张表仓促做；表头写明"这是**移植树**的命名空间、
  等移植树有 package 时整段搬走"。**（b）`xtask` 的 `FLIPS` 两个 feature 串跟着改**：不改就会**静默掉死臂覆盖**
  （`dead-arms` 按 feature 名翻常量，名字一改翻转就不再与 cfg 相配，断言退化恒真）。
  ③ **证据**：`--check-cfg` 过一遍最大的消费者单元，`unexpected cfg condition value` **228 → 0**（34 个名字 → 0）；
  `layout-check` 与 18 个 `build-tu` 在副本上先跑过（当时真树忙），**真树上同样 18 个全部 `built`**；
  `cargo metadata --no-deps`、`cargo build -p xtask` exit 0；新探针 `spikes/window_system_probe.rs`
  （把 `--cfg` 命令写在文件头、保留负向对照）。
  ④ **四条裁决**：`[features]` 落点与 FLIPS 都**接受**（理由如上）；**`enable_checking` 的轴裁定为"裸 `--cfg`"**
  （`regex-emacs.rs` 大量用裸形式、xtask 的 check 通过程显式传它），因此树里那 **3 处 `feature = "enable_checking"` 是错的**
  （那个 feature 永不开、等于死文本）⇒ 已排下轮修；**16 个缩写名**（`bdffont`/`be_cairo`/`freetype`/`harfbuzz`/`xft`…）
  与 **`windows`(81)/`windowsnt`(100)** **保持不动**——规则定为"只有同一 C 开关出现两种拼法时才统一到 C 字面小写"，
  而这两个的 C 条件本就不同（gnulib 的 `_WIN32`/`_CYGWIN` 臂 vs `WINDOWSNT`），lane 的实测判断我采纳。
  ⑤ **prose 的边界**：`docs/ports/*.md` 的 229 处旧拼法**做**（tracked ledger，逐文件列改动）；`docs/status.md`(18)/
  `docs/reviews/*`(6)/`docs/spikes/*`(1) **不做**——历史条目里的旧拼法是对当时状态的记录，我在新条目里说明更名即可；
  `PORTING.md` 的 5 处（P-51/P-68/P-76）**由我自己改**。
  ⑥ 在飞文件的 **36 个**未声明 feature 名**先记账**，等那些单元闭合入库后再并进表。
  ⑦ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；五门绿；引证门 17 drift / 4 range / 284 external /
  36 allowed / 0 stale。**`layout-check` 目前红**，根因是 Harvey 在飞的 `lib/include/stdbit.in.rs`
  （`cannot find macro _gl_stdbit_popcount*` 三条——正写到一半），不涉及本轮提交。

- **3.184 macfont 的诚实"半轮"、windowed `glyph_string` 实测就位、共享门第三次红的干预（2026-10-04）**：
  ① **Mill 这轮只推进 1 个函数（52/74）并如实说明为什么**：它的可支配预算被两件事吃掉——（a）**它自己造成的台账事故**：
  一个脚本的锚点 `## Deferred` 命中了正文里对**同一标题的引用**，把台账中段（Functions/Macros/Types/Objects/Decisions）截掉，
  它重建并逐节校验（现 488 行、节序完整、82 勾/24 未勾）；（b）新代码的编译迭代（`cf_charset_table` 37 行 + `macfont_get_charset`，
  含 C 的转义循环、`alloca` → `withUnsafeTemporaryAllocation`、新入口 `fast_c_string_match_ignore_case`）。
  剩下 19 条活代码约 **1,700 行 C**（驱动核心 ~500、绘制/整形 ~370、OpenType 层 ~400、CoreText helper ~330、记录与收尾 ~80），
  它判断"继续硬写只会产出没走查的代码"，**在单元仍可编译、台账自洽的状态停下**——这个取舍我认可。
  ② **它顺手做了下一轮要用的测量**（写进 Evidence）：windowed `struct glyph_string` = **sizeof 272 / align 8**，
  27 个成员偏移全部量出（`x`@0、`y`@4、`width`@12、`f`@32、`w`@40、`row`@48、`char2b`@64、`face`@80、`font`@88、
  `cmp`@96、`first_glyph`@128、`img`@136、`xwidget`@144、`clip_head`@160、`clip_tail`@168…），位域在尾部同一分配单元
  （下轮按 `macfont_info` 的办法处理）。**`macfont_draw` 的阻塞至此只剩"把镜像写出来"。**
  ③ **它验证了我的工具修复**：用**新 writer** 删掉 `macfont_get_charset` 那一行 manifest，先备份再 diff——
  **只少那一行、reason 列完好** ⇒ 3.182 的修复有效（这是"修好的工具由使用者复验"的闭环）。
  ④ **我批准它下一轮的执行序**（逐段编译、段段 `swiftc` exit 0）：`glyph_string` Swift 镜像 → `macfont_draw` + 五个 `CG_SET_*` 宏
  → OpenType 层（`get_open_type_spec`/`create_attributes_with_spec`/`supports_charset_and_languages_p`）→ `list`/`match`
  → `open`/`close`/`text_extents`/`shape` → 记录 + `@_cdecl("macfont_driver")` getter + `mac_register_font_driver` +
  `syms_of_macfont*` + `macfont_get_nsctfont`，CoreText helper 四个最后。收口后接 `src/nsfns.m`。
  ⑤ **共享门第三次红的干预（升级为硬性要求）**：`layout-check` 已因 `lib/include/stdbit.in.rs` **连续五轮红**
  （`cannot find macro _gl_stdbit_popcount{,l,ll}`：`stdc_count_ones_*` 的函数体已入文件、它调的宏还没写）。
  已发第三次请求：**先把文件恢复成可编译状态**（补宏定义，或把函数体挪进注释/`#[cfg (any())]`）再继续。
  **理由写在请求里**：这是全树镜像**唯一**的编译闸门，红着的时候没有任何人能验证任何镜像可编译。同时把它提为**该 lane 的硬性要求**：
  每次把新块写进文件前，先确保它引用的名字已在同一文件里。
  ⑥ 读数：`gen-port-inventory` 590 files、done=542、wip=21、todo=27；四门绿（`check-declarations` 13/0、`check-globals` 283/0、
  `check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 600/0）；引证门 17 drift / 4 range / 284 external /
  36 allowed / 0 stale；`check-swift-units` 0 new / 0 stale（macfont 22 行、nsfont 2 行）。

- **3.185 共享门由我兜底修好（第五次红的收尾），并定下"脚本改台账"的规矩（2026-10-04）**：
  ① **兜底修复**（`7f74cf9`）：`layout-check` 连续五轮红的根因是 `lib/include/stdbit.in.rs` 里三条
  `stdc_count_ones_*` 调用 `_gl_stdbit_popcount{,l,ll}` **当宏用**，而该文件自己的 `_gl_stdbit_clz` 一族是**函数**
  ——全树镜像唯一的编译闸门因此对所有人失效。我做了**最小修复**：把三个 helper 定义成函数
  （`n.count_ones () as c_int`，与同文件 `_gl_stdbit_clz` 的形状一致；C 的活臂 `__builtin_popcount*` 返回 `int`），
  三处调用点改成函数调用，并在文件里留 **NOTE** 说明"这是集成者的解封，拥有该文件的 lane 回到这一族时请换成本意定义"。
  **`layout-check` 恢复 passed**；该镜像的 `--check` 未完成项 **162 → 136**。
  ② **规矩：脚本改台账必须锚定"第一次出现"或按行号，改完必须 diff**。来源是 Mill 那轮的事故——它的脚本锚点
  `## Deferred` 命中了正文里对**同一标题的引用**，把关账中段（Functions/Macros/Types/Objects/Decisions）截掉；
  它重建并逐节校验（488 行、节序完整）后如实上报。这条与 3.153/3.156/3.157/3.169 那几轮"叙述性文本被当成事实"
  同源：**结构锚点只能锚结构**。
  ③ **两条 lane 重新排活**：Harvey 先看我的 NOTE 决定 helper 的本意形态（宏或函数，**只要文件仍可编译**），
  再继续落 136 条家族层（含 13 个 dispatcher 宏 + `stdc_imm8*`/`load8`/`store8`/`memreverse8`），
  并**重申硬性要求**（写进文件的引用必须先有定义；每段落完自己跑 `layout-check`，绿了才继续）；
  Mill 按 3.184 批准的执行序推进 macfont（`glyph_string` 镜像 → `macfont_draw` + 五个 `CG_SET_*` 宏 → OpenType 层
  → `list`/`match` → `open`/`close`/`text_extents`/`shape` → 记录 + getter + `syms_of_*`），
  并提醒它 **cfg 名已统一为 `have_window_system`**（3.183）。
  ④ 读数：`gen-port-inventory` 590 files、done=542、wip=22、todo=26；四门绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/0、`check-verbatim` 4607/0、`check-target-names` 600/0）；
  **`layout-check` passed**；引证门 17 drift / 4 range / 284 external / 36 allowed / 0 stale。

- **3.186 `src/widget.h` 进镜像（Xaw 半挂 `use_x_toolkit`），并复算"产品名声明"这条长期指令在这一版 HEAD 上的状态（2026-10-04）**：
  ① **本轮实产**（`f2217ec`，签名 `verified=true`，已推 `origin/master`）：`src/widget.h`（102 行，**36 宏 + 1 对象**）
  → `femacs/src/include/widget.rs` + `docs/ports/include-widget.md`（台账 37 个勾选框全满、0 未勾）。
  35 个资源名字符串（`XtNminibuffer`/`XtCMinibuffer`/`XtCForeground`…）是 C 自己的字面量，冻结可见，落成
  `pub const … : &CStr`（`c"…"` 形式，Rust 侧逐字）；`display_context` 是 `*mut c_void` 加 `/// C:` 原文行；
  `_EmacsFrameRec`/`_EmacsFrameClassRec` 是 **Xaw 的类型**（本目标没有它的镜像），两个 typedef 随之为 `*mut c_void`；
  五个声明（`emacsFrameClass`/`EmacsFrameSetCharSize`/`widget_store_internal_border`/
  `widget_update_wm_size_hints`）与所有 Xaw 类型一起挂 `#[cfg (feature = "use_x_toolkit")]`，**照 C 写 C 的名字**
  （P-51：非冻结侧的存在理由是该目标自己供给这些名字；`USE_X_TOOLKIT`/`USE_LUCID` 在 `docs/baseline/config/` 里未定义，
  `src/widget.c` 也不在冻结源码清单里）。六个 `#ifndef` 守卫的宏照发并记理由（它们是"未定义才定义"的守卫，不是普通 `#define`）。
  ② **长期指令的复算，不是重做**：`把之前的代码中的这种声明也去掉`（与 10-03 的 `版本声明不要再说是gnu emacs了` 同 turn，
  见 3.134）的处置没有回退——`2ee1d1d`/`4810bb0` 把 231 个首行压到只留修订号（`@ fd7afced403`）、`fd9cf89` 不携带
  268 个版权/许可块。本轮实测：`check-declarations` → **13 declaration sites (0 forbidden, 13 allowed literals), 0 failures**；
  定向扫 `part of GNU|is free software|Free Software Foundation|Copyright (C)|under the terms of the GNU|GNU Emacs [0-9]`
  命中 8 处，**每一处都是允许清单 `docs/baseline/declarations-allowlist.tsv` 上的 C 字符串字面量**
  （`src/emacs.rs` 4、`lib-src/etags.rs` 2、`lib-src/ebrowse.rs` 2——横幅要打印、docstring 门要逐字比对，属被移植的语义），
  另 1 处是 `lib/getopt.rs:91` 的上游注释提到 "GNU libc"（不是本产品名）；本轮新落的 `include/widget.rs` **零命中**。
  ③ 读数：`gen-port-inventory` **590 files、done=543、wip=22、todo=25**；门全绿——`check-declarations` 13/0、
  `check-globals` **283 单元 / 0 缺定义**、`check-mirror-dupes` 1 重复声明 / 1 棘轮行 / 0 失败、`check-verbatim`
  **6 trees / 4607 files / 0 failures**、`check-target-names` **601 .rs / 593 targets / 0**、`check-swift-units`
  **0 new / 0 stale / 4 Swift 单元已入库**、`check-citations` 600 files / 18 drift / 4 range / 284 external / 36 allowed / 0 stale；
  **`layout-check` passed**（我自己重跑，`CARGO_TARGET_DIR=/tmp/femacs-lc-187`，用完即删）。
  ④ **三条 lane 本轮交活，下一轮由我入库**（3.187/3.188）：**keyboard 闭合**（接手 C 13173 `wipe_kboard` → C 14682
  `mark_kboards` 的收尾括号 = 全文件；`keyboard.rs` 15,360 行，`--check` 的 missing 行归 0，scaffold 3→0，
  78 个 `DEFVAR_*` 持有者落地使 `check-undef` 的 21 条棘轮行 stale 而 `check-globals` 从 78 条缺定义降到 0）；
  **macfont 第 ① 段**（`glyph_string` Swift 镜像按实测 sizeof 272/align 8 与 27 个偏移、`macfont_draw`、
  五个 `CG_SET_*` 宏的 Swift 形态，`macfont.swift` 9,240 行、53/74 函数）；**cfg 卫生第二批**
  （`alloc.rs` 三处 `#[cfg (feature = "enable_checking")]` 改成裸 `#[cfg (enable_checking)]`，一改立刻暴露
  `AVAIL_ALLOCA!` 单参调用与 `terminate_due_to_signal` 未声明两个从未被任何编译器读过的缺陷；66 处台账散文旧拼法；
  `[features]` 表删掉"声明了但没有 `--cfg` 驱动方"的 `enable_checking`）。

- **3.187 cfg 卫生第二批：`enable_checking` 改裸 cfg，两个从未被任何编译器读过的缺陷当场现形（2026-10-04）**：
  ① **本轮的实产**（`9317473`，签名 `verified=true`，已推 `origin/master`）：`femacs/src/alloc.rs` 的三处
  `#ifdef ENABLE_CHECKING` 文本（C 7342-7356 的 `suppress_checking`/`die`、C 7358-7378 的 `verify_alloca`）
  原来写 `#[cfg (feature = "enable_checking")]`，改成**裸** `#[cfg (enable_checking)]`——本树其它 checking-only
  文本都是这个拼法，`xtask` 的 finished-unit 编译正是用 `--cfg enable_checking` 编译它们的。
  ② **为什么四道门都看不见**：`extract-abi.py --check` 的 token 比对在两侧都先剥掉 `#[…]` 属性（cfg 的名字不进被比对的
  身体）；`build-tu` 与所有门**不传任何** `--cfg feature=…` ⇒ feature 门控文本被剥掉，"名字写错"与"名字对但没打开"
  完全同形；检查 pass 找的是裸名字；**没有任何门问"这个 feature 名有没有驱动方"**。更要紧的是 `[features]` 表会让
  `unexpected_cfgs` 闭嘴，于是声明一个没人驱动的名字**更安静而不是更响**。不变量因此写成：**一个 feature 名只有在某个
  `--cfg` 产出方（FLIPS 行 / 探针 / 检查 pass）能打开它时才有存在理由**；`enable_checking` 那条已从表里删除
  （545 行 = 545 个真用到的名字，0 unused）。
  ③ **轴一改，那段文本立刻报出两个缺陷**（都是"从未被任何编译器读过"的同一类）：`verify_alloca` 写的是
  `AVAIL_ALLOCA! (i as isize)`，而 port 的宏取两个参数（P-37）→ C 这里是不动 `sa_avail` 的普通 `alloca`，
  改成直接调栈池 `sa_stack_alloc (i as usize)`（本树其它裸 `alloca` 处的写法，P-05）；`die` 调的
  `terminate_due_to_signal` 在本单元 include 图里没有声明（C 在 lisp.h 5137，lisp.h 镜像不带它）⇒ 本单元为自己的调用者
  带一份同 gate 的声明。证据：`build-tu src/alloc.c` built；同一 materialized root 用**pin 的 toolchain**（`rustup run
  nightly-2026-10-01 rustc`）+ `--cfg enable_checking` → exit 0，`nm -g` 里 `T _die`/`S _suppress_checking` 都在，
  冻结构建的 `build/femacs/alloc.o` 里两者计数 0（负向对照）；`--check src/alloc.c` OK。
  ④ 后半是散文：24 个 tracked 台账里 **66 处**旧拼法（我为集成者复算过：`+109 -65`，逐文件对称）；`docs/status.md`
  的历史条目按裁决**不动**。**数字更正**：我上一轮交代的"229 处"是把 `docs/` 全部 `feature = "…"` 提及（含正确拼法）
  加总的粗糙统计，真旧拼法只有 66。

- **3.188 `src/keyboard.c` 闭合：14,682 行全文件，并抓出横向传参的真缺陷（2026-10-04）**：
  ① **本轮的实产**（`9467e60`，签名 `verified=true`，已推 `origin/master`）：第十二轮把 C 13173 `wipe_kboard`
  一路做到 **C 14682 = `emacs/src/keyboard.c` 的最后一行**（`mark_kboards` 的收尾括号）。`keyboard.rs` **15,360 行**
  （vs C 14,682 = +4.6%，仍在 5% 默认容忍内、无 tolerance 行），`--check` 的 **missing 行归 0**（函数/subr/defsubr/
  file-scope object/宏/常量六类全 0），最后一块 scaffold 连同说明注释删除（3→0），`event_head` 结构体按冻结 clang
  实测落地（sizeof 4 / alignof 2 / var@0 / kind@2 + 4 条 `const _` 断言），78 个 `DEFVAR_*` 持有者一行一个。
  ② **我做掉的集成项**：`macro-body-notes.tsv` 两行（`syms_of_keyboard` 的位置化 docstring、`is_ignored_event` 的
  `switch`→`match`，这是 `--check` 仅剩的两条 note 行）；`[features]` 三个名字（`no_interrupt_input`、
  `no_usable_sigio_or_sigpoll`、`use_lucid`）；台账补上漏掉的 `force_quit_count`（C 12596）并把闭合文件遗留的
  6 个 subr 框勾上（**329 框全满 → 单元离开 WIP**，inventory 从 543/22 变 545/20）；`undef-pending.txt` 重取
  （78 个持有者让 21 条棘轮行 stale 消失，30 条新条目**逐条读过**，全是 xdisp/window/regex 这些在飞或未端口单元的依赖）。
  ③ **抓出一个真缺陷并修**：`src/callint.rs` 的两处 `DEFVAR_KBOARD!` 传的是 C 名字对应的**变量**，而 C 的宏存的是
  `offsetof (KBOARD, vname##_)`（lisp.h 3525-3530）⇒ 记进 `kbdoffset` 的是指针值，运行时 `prefix-arg` 会读到垃圾
  （keyboard lane 在按同形改写自己的 8 处时发现，按纪律没动别人的文件）。两处改成
  `std::mem::offset_of! (kboard, Vprefix_arg_)`，`build-tu src/callint.c` built。**教训**：跨单元宏的实参语义要按 C 的
  宏体核对，不能按名字猜。
  ④ 证据：`build-tu src/keyboard.c` built（11 warnings、0 error）；`--check emacs/src/keyboard.c` 0 missing；
  `check-cond-eval src/keyboard.c` 216 conditions / 0 unknown / 0 disagreements；check-globals 283/0、
  check-declarations 13/0、check-mirror-dupes 1/1/0、check-verbatim 4607/0、check-target-names 601/593/0 全绿。

- **3.189 `lib/string.in.h` 落地，两个模板自带函数体搬回 owner 镜像（2026-10-04）**：
  ① **本轮实产**（`c4ed03e`，签名 `verified=true`，已推 `origin/master`）：`lib/string.in.h`（1,852 行、111 个
  `@NAME@` 占位符）→ `femacs/lib/include/string.in.rs`（391 行），台账 47/47 全勾，`--check` **OK 且无 `- ` 行**。
  实测：108 个占位符在生成规则里，解析后 **36 个为 `1`**；77 个 `GNULIB_*`/`REPLACE_*` 选择子里 **16 个活**
  （`MDA_MEMCCPY`…`REPLACE_FREE`），另 61 个是死文本（P-51）；抽取器判不了 `@PLACEHOLDER@`，清单是各臂并集（P-66，台账写明）。
  ② **归属裁定**：模板自带的两个函数体 `memeq`（C 425-431）/`streq`（C 817-822）从 `string.rs` 搬进本镜像
  （3.181 对 `stdbit.in.rs` 的同一裁定：body 检查只读镜像自身文本，且一个 crate 里两份同名定义是 `E0428`）；
  保留 C 自己的形参名 `__s1`/`__s2`/`__n`（body token 门要比）。三个 `_GL_*_INLINE` 写成 `macro_rules!` 而非 `const`
  （消费者保留同名 `pub const`，第二个 `const` 是 `E0428`，宏与 const 不同命名空间）；`strnul`（C 1302-1305）
  **故意不写**（`macro_rules!` 按文本顺序遮蔽，会打断 `lib/time_rz.rs:198` 用的那个）。9 个 tracked 消费者
  `include!` 改指新 owner。
  ③ **三条负向验证**（改→查→立刻还原）写进台账：活选择子的常量、`streq` 的 body token、以及**死臂**的
  `rpl_memchr`——第三条证明死的 `#[cfg]` 臂也在被比。顺带纠正两条流传的错引证：`GL_GENERATE_STRING_H_CONDITION`
  **不存在**（`grep -c '^GL_GENERATE_STRING_H' build/emacs-ref/lib/gnulib.mk` → 0，真因是 `gnulib.mk:3838` 的无条件规则），
  活选择子是 16 不是 15（漏了 `REPLACE_FREE`）。
  ④ 新增待裁定项：`lib/include/string.in.rs` 与 `string.rs` **不在 `layout-check` 的 foundation 图里**（没有
  `src/include/*.rs` include 它们），要让共享门也覆盖它们，需把 `lib/include/string.in.rs` 加进 xtask 的 foundation 列表
  （待定，见 §3）。

- **3.190 `lib/stdbit.in.h` 闭合：家族层落地、八条共享行就位、popcount 回归"宏"的身份（2026-10-04）**：
  ① **本轮实产**（`150864c`，签名 `verified=true`，已推 `origin/master`）：`femacs/lib/include/stdbit.in.rs` **1,890 行**
  （242 个 item：110 函数 + 11 宏 + 常量/类型别名）覆盖整份模板，**`--check` OK 且 0 行**；台账 264 框全勾。
  ② **我做的集成**（lane 只读共享表，行由我写）：`excluded-macros.txt` 3 行（`__popcnt64`、
  `_gl_stdbit_popcount_supported`、`_gl_stdbit_popcount_wide`——都在 C 836-838 的 `#else` 死臂 / `#ifdef _MSC_VER` 内，
  `__cpuid`/`__popcnt` 没有 Rust 拼法）；`macro-body-notes.tsv` 5 行（两条 `ull` 后缀：tokeniser 只折叠
  `u/l/ul/ll`，Rust 只能写 `1u64`；三条 `stdc_memreverse8u16/32/64`：活臂是 `__builtin_bswap*` ⇒ 端口拼
  `swap_bytes ()`，死臂是 mask 表）。**写入后本件 `--check` 从"恰好 8 行"变 OK/0 行**。
  另把台账里 5 个未勾框补齐（三个死臂宏 + `_`/`_Bool` 两条 recorded-name 行，按 `include-errno.in.md`/
  `include-limits.in.md` 对 `_` 的先例），单元因此离开 WIP。
  ③ **修掉我 3.185 的兜底形态**：我当初把 `_gl_stdbit_popcount{,l,ll}` 写成**函数**解封共享门；lane 用**实测**证明
  那样会让 `rust_inventory` 的 body 检查去读 MSC **死臂**、多出三条 `does not mention __popcnt /
  _gl_stdbit_popcount_supported / _gl_stdbit_popcount_wide`。C 839-841 的活臂本来就把它定义成
  `__builtin_popcount*` 的**宏** ⇒ 改回宏（`macro_rules!`）后门不再索要死臂 token，三个调用点改回 `!`。
  **教训**：集成者兜底修镜像时，形态也要按 C 的身份选（宏 vs 函数），否则会把门引到死臂上。
  ④ **新增 tracked 探针**：`spikes/stdbit_family_probe.rs`（212 行）登记进 xtask `PROBES`（15 → **16**）；
  断言全以标准库同名运算为期望（`is_power_of_two`/`rotate_left|right` 含跨掩码 0/32/33/100/0xFFFF_FFFF/
  `from_le_bytes`/`to_le_bytes`/`leading_ones`…），覆盖 5 个宽度，**544 checks passed**。三个负向对照里有一条
  说明**门的边界**：把 `bit_floor_ui` 的 `else { 0 }` 改成 `else { 1 }`，门仍打印 OK（`0` 还留在 `n != 0` 里），
  而探针 panic `bit_floor_ui 0 left: 1 right: 0` —— 这类"常量值被同文件其它出现掩蔽"的缺陷只有语义探针能抓。
  ⑤ 三处**不需要共享行**的拼法（都写进台账 Decisions）：`_GL_STDBIT_UINT_FASTN` 是类型宏 ⇒ 另立
  `pub type`（取值用冻结 clang 的 AST 实测 `unsigned short/int/long long`）；`_GL_STDBIT_MEMCPY` 两侧都过
  `c_uchar`（C 的 `memcpy` 收 `void *`，`copy_nonoverlapping` 要求同一元素类型）；`_GL_STDBIT_ASSUME_ALIGNED`
  拼 C 自己的 `#else` 臂 `(ptr)`（活臂 `__builtin_assume_aligned` 原文留在 doc comment，P-55，不造符号）。
  ⑥ 读数：`gen-port-inventory` **590 files、done=546、wip=19、todo=25**；门全绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-target-names` 601/593/0、`check-swift-units`
  0 new/0 stale）；**`layout-check` passed**（83 of 94 镜像编译、11 平台独占跳过）。
  ⑦ 同轮 lane 侧：**xdisp 第 11 轮**落 C 10063-10877（`IT_POS_VALID_AFTER_MOVE_P` +
  `move_it_in_display_line_to` 含其体内两个函数内宏；`missing function` 181→180、problem 行 254→251、
  `xdisp.rs` 11,915 行），它要的那条 `macro-body-notes` 行（`move_it_in_display_line_to` / `goto, uninit`）
  我**压在闭合时再写**——共享表的行必须能被仓库内的文件验证；同时给它留了两条下一轮的活。

- **3.191 `lib/ieee754.in.h` 落地，两个消费者交出各自私有的 union（2026-10-04）**：
  ① **本轮实产**（`23d162b`，签名 `verified=true`，已推 `origin/master`）：镜像
  `femacs/lib/include/ieee754.in.rs`（224 行）覆盖 8 宏 + 三个位域 union。**判据与实测都来自冻结 clang**：
  生成头 `build/emacs-ref/lib/ieee754.h` 的第 20 行把 glibc 臂写成 `#if 0` ⇒ 活臂是 gnulib 的四个 order 旋钮，
  它们命名 `<endian.h>` 的名字，而本树里那些名字的 owner 是 `lib/include/endian.in.rs`（1234/4321，
  实测 `BYTE_ORDER` 1234、`BIG_ENDIAN` 4321、`FLOAT_WORD_ORDER` 1234）；三个 union 实测 4/4、8/8、16/8，
  成员结构体 4/4、8/4、12/4；每个位的位置都由**字节 dump** 固定（探针与输出原文进了台账 Evidence）。
  ② **两条可复用的形状**：**include guard 的 C body 是 `1`** ⇒ 必须写 `core::ffi::c_int = 1`（`bool = true`
  会让 body 检查报 `_IEEE754_H does not mention 1`，先例 `byteswap.in.rs` 23）；**位域访问器宏的第一个参数必须是
  C 自己的成员路径**（`ieee754_double_bits! (ieee_nan.mantissa0, u)`）——这不是风格问题：C 的函数体 token 集合里
  `ieee_nan` 与 `mantissa0` 是**两个 token**，把变体名放进宏名（`ieee754_double_ieee_nan_bits!`）会丢掉
  `ieee_nan`，门当场报 `- function float_to_string does not mention ieee_nan`（实测后改回）。
  另：`long double` 在本 target 是 8 字节 double（`editfns.rs` 197 的 `c_longdouble = c_double` 同一实测），
  所以第三个 union 的 16 字节 = `ceil (12, 8)`；FFI 类型一律全限定 `core::ffi::*`（裸名在单 crate 布局里是
  `E0425`，`stdio.rs` 644-653 的既有实测）。
  ③ **顺手消掉两处"模板还没有镜像"的私有副本**：`src/print.rs`（C 1200-1208 的 NaN 臂）与 `src/lread.rs`
  （C 4572）各自声明过一份 `union ieee754_double`（P-26/P-62），现在都改成 `include!` 本镜像，本地块删除，
  它们当年写的测量搬进本台账。两件的 `--check` 结论**不变**（都是 OK），`build-tu src/print.c` 与
  `build-tu src/lread.c` 都 built。
  ④ 读数：`gen-port-inventory` **590 files、done=547、wip=21、todo=22**；门全绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-target-names` 604 .rs/593/0）。

- **3.192 共享门第四次红的干预：`textconv.rs` 的嵌套 extern 块（集成者最小修复）（2026-10-04）**：
  ① **现象**：`xtask layout-check` 红，`error: extern block is not supported in `extern` blocks`
  指向在飞镜像 `femacs/src/include/textconv.rs:174`。根因是 `pub mod textconv_decls` 的外层
  `unsafe extern "C" { … }` 里又嵌了一个 `#[cfg (feature = "have_android")] unsafe extern "C" { … }`
  ——**Rust 不允许 extern 块嵌套**（C 侧那 14 个 android 声明在 `#ifdef HAVE_ANDROID` 里，形状上"成组"，
  但 Rust 只能把 cfg 落在每个 item 上）。`layout-check` 是全树镜像**唯一**的编译闸门，它红着的时候
  所有 lane 都无法验证任何镜像，所以由我兜底。
  ② **最小修复**（lane 的文件、未入库，内容仍是它的）：删掉内层 `unsafe extern "C" {`/`}` 包装，
  把 `#[cfg (feature = "have_android")]` 移到那 **14 个** android 声明各自头上
  （`start_batch_edit` … `get_conversion_field`），并把缩进并回 mod 的 4 空格级。修完：
  **`layout-check` passed**、`--check emacs/src/textconv.h` 仍 **OK**（改动不碰任何被比对的 token）。
  ③ **写给所有 lane 的规则**：需要按 cfg 分组**声明组**时，用「每个 item 前一行 `#[cfg (…)]`」，
  或把整组收进一个 `#[cfg]` 的 `mod` 再 `pub use`（后者会改调用路径，不推荐）；**不要嵌套 extern 块**。
  这条与 3.185 的"任何时刻镜像必须可编译"同源：3.184/3.185/3.192 三次红都不是新发明的问题，
  而是"写完一段没就地编译"。
  ④ 读数（门修复后）：`gen-port-inventory` **590 files、done=547、wip=21、todo=22**；
  `check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/1/0、
  `check-target-names` 604 .rs/593/0、**`layout-check` passed**（83 of 94 镜像编译、11 平台独占跳过）。

- **3.193 `lib/stdlib.in.h` 落地，并收口四个"其实已经干净、只是台账没勾"的镜像（2026-10-04）**：
  ① **本轮实产**（`6daa2f2`，签名 `verified=true`，已推 `origin/master`）：`lib/stdlib.in.h`（1,991 行 / 78 项）
  → `femacs/lib/include/stdlib.in.rs`（274 行：36 `pub fn` + 6 `macro_rules!` + 15 `pub const`），
  **`--check` OK 且 0 行**；台账 256 行（67 勾 + 1 个**有意保留**的未勾框）。判据是生成头
  `build/emacs-ref/lib/stdlib.h`（94,510 字节）：34 条活声明的签名逐条来自**判据编译器自己的** `clang -Xclang
  -ast-dump`（`wchar_t` → `c_int`、`long double` → `c_double`，都实测）；`MB_CUR_MAX` 的模板臂是死的
  （生成头 1348 行 `#if 0`），活的是系统的 `__mb_cur_max`（镜像声明它并拼成宏）；12 个 `GNULIB_defined_*`
  见证常量按 `sys_types.in.rs` 先例携带 C 那条臂设的值。
  ② **我写的共享行**（lane 只读共享表）：`excluded-macros.txt` **9 条**（`reallocarray`、`initstate_r`/
  `random_r`/`srandom_r`/`setstate_r`、`print_stack_trace`、`GNULIB_defined_ptsname_r`、`_gl_MB_CUR_MAX`、
  `rpl_realloc`，全是死臂/本 target 无此函数，`kind=config`）+ `macro-body-notes.tsv` **1 条**（`MB_CUR_MAX`
  的记录体是死臂的 `_gl_MB_CUR_MAX ()`）。写入后本件从"恰好 10 行"变 OK/0 行。
  ③ **三条裁决**（写进 lane 简报）：**死名字维持 excluded 行**而不是 `#[cfg (feature = "gnulib_x")]` 载体
  （3.187 的不变量：feature 名必须有 `--cfg` 驱动方，这些名字在本 target 永远没人驱动；且 excluded 表正是这个
  登记处）；**include 站点迁到 `stdlib.in.rs` 批准但分开做**（一次一个单元、各自 `build-tu`，`stdlib.rs` 作为
  系统半边保留）；**`struct random_data` 维持台账 deferral**（死臂 + 本 target 无该类型，extractor 打
  `missing type deferred` 而不是失败——与 `widgetprv`/`emacsgtkfixed` 同类：**未勾框是有意的**）。
  ④ **收口批次**：`lib/allocator.h`、`lib/careadlinkat.h`、`lib/fseterr.h`、`lib/minmax.h` 四个镜像在树里
  **本来就是 `--check` OK**，只因台账框没勾而挂在 WIP。本轮逐个补勾 + 补 `## Evidence`（`--check` 末行 +
  消费者的 `build-tu` 末行：`lib/allocator.c`、`lib/careadlinkat.c`、`lib/fseterr.c`；`minmax.h` 没有自己的
  单元，证据是 `careadlinkat.rs`/`getrandom.rs`/`nproc.rs` 对它的 `include!`）。其中 **`allocator.h` 关掉了
  §3.4**（`lib/allocator.c` 需要的 `struct allocator` 镜像一直在，只是没勾）。
  **两个故意留在 WIP 的**：`src/widgetprv.h`、`src/emacsgtkfixed.h`——它们的 Decisions 写明有记录因缺 X11/GTK
  头而**deferred**，不是漏做（`widgetprv.rs`/`emacsgtkfixed.rs` 本身在 layout 探针里、编译通过）。
  ⑤ 读数：`gen-port-inventory` **590 files、done=558、wip=18、todo=14**（本轮 547/21/22）；门全绿
  （`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-target-names`
  612 .rs/593/0、**`layout-check` passed**、86 of 97 镜像编译 + 11 平台独占跳过）。
