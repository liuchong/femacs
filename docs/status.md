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
- ~~3.4 `lib/allocator.c` 需要 `allocator.h` 镜像~~ **已解决（2026-10-04，3.193）**：`femacs/lib/include/allocator.rs` 其实早已写好且有 `lib/allocator.c` 这个消费者，只是台账框没勾而被记成 WIP；本轮补勾 + 补 `build-tu` 证据，`lib/allocator.c` 已可编译。
- **3.11 `lib/include/*.rs` 镜像没有全树编译器**（**已由 3.217 收口**：可自成一 crate root 的 29 个进 probe 种子，其余 102 个「需要 includer」的按数目打印出来）：`xtask layout-check` 从 `femacs/src/include/*.rs` 加两个
  foundation 出发、只编 `include!` 图**够得到**的文件（`femacs/xtask/src/main.rs` 795-812），所以 `lib/include` 的镜像
  一般**不在这道门里**——实测 `grep -c 'lib/include/stdlib.in.rs' build/femacs/layout_probe.rs` → **0**
  （`stdbit.in.rs` 在，因为 `src/include/lisp.rs` 够得到它）。**待办**：把 `lib/include/*.rs` 加进 probe 的种子，
  代价是可能暴露若干「离开 unit 上下文编不过」的镜像（它们现在的证据是 `build-tu lib/<h>` 的 crate-root 形状，
  或 `/tmp` 的 unit 形状探针）；**要在在飞的 lib 镜像（`stdio.in.rs`）落定后再做**，否则会把共享门弄红。
- **3.10 `lib/stdcountof.in.h` 的模板条件判不了（P-66 的承载问题）**（**已由 3.215 收口**：那些「判不了」的是模板的 `@NAME@` 占位符，它们的值在冻结配置的替换表里，可测）：`check-cond-eval` 报 `10 conditions, 4 unknown, 5 disagreements [NOT VERIFIED]`（`_GL_GNUC_PREREQ (3, 1) && ! defined `__clang__`` 这类它判不了），所以抽取器的清单是**各臂并集**：C++ 那两段模板（`_gl_verify_type`、`GNULIB_defined_struct__gl_verify_type`）也在清单里，`countof (...)`（`sizeof (A) / sizeof (A)[0] + 0 * _gl_verify_is_array (A)`）是活的。**落它之前要先定两件事**：① 未知条件的承载方式（P-51/P-66；同 `assert.in.h`/`stdckdint.in.h`）；② `countof` 在 5 个已端口单元（`alloc.rs`/`data.rs`/`chartab.rs`/`androidvfs.rs`/`emacs.rs`）里已有各自的拼法，镜像落地时一并决定 re-point。
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

- **3.194 gnulib 五个小头落地（`warn-on-use` / `ftoastr` / `unlocked-io` / `openat` / `stddef.in`），每个都能当自己的 crate root 编（2026-10-04）**：
  ① **本轮实产**（`35dadca`，签名 `verified=true`，已推 `origin/master`）：5 个镜像 + 5 个台账，合计
  `--check` **5×OK/0 行**、`build-tu lib/<h>` **5×built**（`warn-on-use.rs` 35 行、`ftoastr.rs` 160、
  `unlocked-io.rs` 127、`openat.rs` 58、`stddef.in.rs` 131）。**没有一条共享行**。
  ② **判据工作**（这五项的价值都在这里）：`warn-on-use.h` 在本 target **整份是死文本**
  （`check-cond-eval` 12 conditions / 0 unknown / 0 disagreements），镜像就是一条注记（`arg-nonnull.rs` 同形）；
  `ftoastr.h` 的 `FLT_RADIX == 10` 臂死（实测 `FLT_RADIX` 2），实值仍在 `lib/ftoastr.rs`，镜像只落名字；
  `unlocked-io.h` **不在 verified 表**（16 conditions / 0 unknown / **9 disagreements**：macOS 自己的 `<stdio.h>`
  把 8 个 `*_unlocked` 定义成宏，正是每个条件的第二个析取项），所以抽取器盲读、要求全部 32 个名字 ⇒
  活/死两支都带（14 条死臂各挂 `#[cfg (feature = …)]`）；`openat.h` 的 `!HAVE_OPENAT` 声明支是活臂；
  `stddef.in.h` 不在 verified 表（34 conditions / 7 unknown / 10 disagreements），活臂由**生成头**
  `build/emacs-ref/lib/stddef.h` 定（`gl_unreachable` → `core::hint::unreachable_unchecked`、`_GL_STDDEF_H`）。
  ③ **一条方法**：5 个镜像都遵守"镜像不写 `use`"的仓库约定（否则 `E0252` 会撞包含它的 unit 的 import），
  标量一律写绝对路径 `core::ffi::*` —— 这正好让 `build-tu lib/<h>` 能把镜像当 crate root 编，编译证据就是
  台账 Evidence 的末行。（`layout-check` 从 `src/include/*.rs` 出发，覆盖不到 `lib/include/` 的大部分镜像；
  这是 §3 里挂着的"把 lib 镜像纳入 foundation"那条的另一面。）
  ④ **裁决**：`_GL_FLOAT_STRLEN_BOUND_L` 那条"可选 excluded 行"**不采纳**——现在放宽的写法保住了 C 的
  `_gl`/`prec_bound`/`min_10_exp`/`max_10_exp` token，门因此不需要额外行；"调用点给拼好的名字"的先例只在
  有真实消费者、且列在 notes 行里时才用。三条交接点（`lib/ftoastr.rs` 的重复常量、`lib/utimensat.rs` 66-70 与
  `lib/openat-die.rs` 10-13 的手抄原型、`openat-die.rs` 15 行那句过时注释）判定为**由我下一轮接线**。
  ⑤ 读数：`gen-port-inventory` **590 files、done=560、wip=17、todo=13** 门全绿
  （`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-target-names`
  612/593/0、`layout-check` 87 of 98 镜像编译 + 11 平台独占跳过）。

- **3.195 四个 `src/*.h` 平台头落地（`pgtkgui` / `textconv` / `w32gdiplus` / `xwidget`），并记下两条工具级发现（2026-10-04）**：
  ① **实产**（`9e15167`，签名 `verified=true`，已推 `origin/master`）：四个镜像 + 四个台账，`--check` **4×OK**；
  我按 lane 的报告写了 `macro-body-notes.tsv` **7 行**（`xwidget.h` 233-239 的单行 INLINE 定义——抽取器的 span 是整行，
  于是 body 检查向 Rust 函数体索要函数自己的名字，与 `dispextern.rs` 113 的 `GLYPH_CHAR` 同类）并把
  `have_xinput2`/`have_xinput2_4` 登记进 `[features]`（3.187 的表）。
  ② **判据的亮点**：`pgtkgui.h` **自身没有任何 `#if`**——`#ifdef HAVE_PGTK` 在它的*包含者*里（`dispextern.h` 151-157、
  `pgtkterm.h` 29），先包含 dispextern.h 再包含它会得到 typedef 重定义（`struct _GdkCursor *` vs `void *`）与
  `NativeRectangle` 宏重定义，所以 GTK 半边挂 `have_pgtk`、冻结可见的记录（`XCharStruct` 20/4、`XRectangle` 16/4、
  `XChar2b` 2/2、`Color`、`XrmDatabase`）带实测断言；`textconv.h` 的 `HAVE_ANDROID` 块是 14 条各自带 cfg 的声明
  （就是我 3.192 修的形状）；`w32gdiplus.h` **整份没有 include guard**（C 1 就是 `#ifdef WINDOWSNT`）⇒ 全挂
  `windowsnt`、照 C 写 Windows 名字、**不写布局断言**（本 target 没有 `<windows.h>` 可量）；`xwidget.h` 两个记录挂
  `have_xwidgets`，`__OBJC__` 半边只记录不镜像。
  ③ **它们的编译证据是 `layout-check` 而不是 `build-tu`**：头镜像当 crate root 编时看不到 `c_int`/`HAVE_PDUMPER`
  （前者在 `lisp.rs` 的 `use std::ffi::…` 里、后者在单位的 `config.rs` 里），已入库的 `widget.h`/`w32font.h`/
  `w32gui.h`/`nsxwidget.h`/`charset.h` 同样失败——这是那条命令的边界，不是这批文件的缺陷。
  ④ **两条工具级发现**（都来自这条 lane，都不是它自己的文件）：**`layout-excluded-mirrors.txt` 的续行被当成文件名**
  （`dispextern.rs` 因此被静默排除，见 3.196）；`xdisp.rs:1112` 用 `unreachable! ()` 表达 C 的"不可能到达"
  ——检查器的 STUB_RE 会把它当桩，建议该 lane 改成 `emacs_abort ()`（`include/frame.rs` 240 的形状）。
  ⑤ 读数：`gen-port-inventory` **590 files、done=560、wip=18、todo=12**；门全绿。

- **3.196 修掉 `layout-check` 的静默跳过：一个真的镜像被一个假名字顶掉（2026-10-04）**：
  ① **现象**（`2aad9aa`，签名 `verified=true`，已推 `origin/master`）：3.195 的 lane 发现跳过列表里出现
  **`dispextern.rs`**，而排除文件**并没有**这一行。根因：排除文件允许"理由"续行（续行以空白开头），
  而解析器对**每一行**都取"第一个空白分隔 token"——续行里的首个词于是变成了排除项：实测多出 `mirs`、`shadowed`、
  `compile`、`live`、`` `error[E0428]: ` ``，以及 **`dispextern.rs`**（它在 `nsgui.rs` 的*理由正文*里被点名）。
  **后果**：`dispextern.rs` 的布局断言**当轮根本没跑**——它是全树断言最多的镜像之一。
  ② **修法**（两条件缺一不可）：排除项必须是**从第 1 列开始**的行，且首个 token **以 `.rs` 结尾**；续行因此归属上一行
  （正是文件头写的规则）。修后实测：`layout-check: 89 of 99 mirror(s) compiled, 10 platform-exclusive skipped`
  （修前 88/99、11 skipped）且 **passed** ⇒ `dispextern.rs` 的断言现在真的在跑并且成立。
  ③ **教训**（与 3.153/3.185 同源）：**"把散文当数据"** 这类解析错误会让门**安静地少测一块**；凡是"台账/清单 → 代码"
  的解析，都要对**形状**做约束（列位、后缀、类型），并在输出里**打印被排除/被跳过的集合**（这次正是那条打印救了它）。

- **3.197 `src/conf_post.h` 落地；三个 gnulib 单元的邻居指向真镜像（2026-10-04）**：
  ① **实产**（`815277e`，签名 `verified=true`，已推 `origin/master`）：`src/conf_post.h`（445 行 / 66 宏）
  → `femacs/src/include/conf_post.rs`（366 行；40 `macro_rules!` + 16 `const`，其中 **25 条活臂 + 31 条死臂**
  挂 `#[cfg (feature = …)]`）。**判活靠实测**：`clang -E -dD` 的预处理输出 + `check-cond-eval`（41 conditions /
  4 unknown / 3 disagreements [NOT VERIFIED]，三处 disagreement 全是 clang 内建谓词），所以抽取器在这里是盲态、
  66 个名字都必须有落点。`HAVE_LSTAT`/`_REGEX_AVOID_UCHAR_H` 虽在 MSDOS 臂里，但冻结 `config.h` 同名同值也定义
  ⇒ 按**活值**承载。
  ② **我写的 10 条 `excluded-macros.txt` 行**：`HAS_ATTRIBUTE`/`HAS_FEATURE`（预处理器内建谓词 + 标识符粘贴，
  Rust 两样都没有）、其死回退表 `HAS_ATTR_*`（5 条）、`UNINIT`（声明后缀）、`likely`（唯一拼法在 `lisp.rs` 704）、
  `vfork`（死重命名）。写入后本件 `--check` **OK/0 行**。**裁决**：这几条**维持 excluded 行**而不改
  `#[cfg (feature = …)]` 承载（3.187 的不变量：feature 名必须有 `--cfg` 驱动方，`address_sanitizer` 一类在本 target
  永远没人驱动）；死臂目前**没有任何门在编**是已知的 FLIPS 缺口（§3.2/§3.9），属门自身的改动、不在 lane 范围内。
  ③ **邻居指向真镜像**（这一批 gnulib 镜像落地的自然收尾，我做集成）/本轮先做三件：`lib/ftoastr.rs` 的五个
  `FTOASTR_*` 改从 `include/ftoastr.rs` 取、`lib/utimensat.rs` 的 `openat_save_fail`/`openat_restore_fail` 改从
  `include/openat.rs` 取（只留 `openat-priv.h` 那条）、`lib/openat-die.rs` 为 C 4 的 `#include "openat.h"` 加上该镜像
  ——**而且它顺手纠正了一句陈旧的判断**："`openat.h` 是系统头"是错的（实测 `!HAVE_OPENAT` 的声明支才是活臂）。
  三件的 `--check` 结论**不变**（都 OK）、三件都 built；`line-drift-tolerance.tsv` 的两行随它们的行数变化更新
  （`ftoastr` 0.15→0.20、`openat-die` 3.35→3.70）。**取证更正**：我一度写下"这两行在改之前就已超窗"，
  但按工具自己的公式重算 HEAD 版（`abs(130-152)/152 = 14.5% < 15%`、`abs(30-7)/7 = 328.6% < 335%`）
  **是落在窗内的**，所以那句不成立、已改掉。真正可复现的是：改完之后它们**超出**了原窗口（16.4% / 357.1%），
  两行随之更新后 `--check` 恢复 OK。**留下的方法教训**：漂移闸只在"参考文件行数 × (1±tolerance)"这个窗口内
  说话，改一个文件的行数（哪怕只是把重复声明换成 `include!`）都要**先重算窗口再看门**；而容忍表是
  **append-only**（新行覆盖旧行），所以更新应当**追加**而不是原地改（我这次是原地改，因为没有更早的行可保留）。
  ④ 读数：`gen-port-inventory` **590 files、done=562、wip=17、todo=11**；门全绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/1/0、`layout-check` 89 of 99 + 10 平台独占跳过）。

- **3.198 `lib/readutmp.h` 落地；`src/process.c` 被"散文当清单"错记成 WIP（已修）；`boot-time.rs` 接线（2026-10-04）**：
  ① **实产**（`2e30fb2` + `418142d`，签名 `verified=true`，已推 `origin/master`）：`lib/readutmp.h`（352 行）
  → `femacs/lib/include/readutmp.rs`，台账 34 框全勾、`--check` **OK/0 行**。判据全部实测：布局探针打印
  `sizeof 72 / align 8` 与九个偏移（`ut_user` 0 … `ut_exit` 60），镜像把它们写成
  `const _: () = assert! (…)`；活臂是 `UTMP_STRUCT_NAME utmpx` + `setutxent`/`getutxent`/`endutxent`/`utmpxname`，
  其余（`utmp_h`、`utxdb_active`、`android`、`glibc_time_bits_64`、六条 `no_*` 路径/常量臂）走 `#[cfg]` 承载。
  ② **`lib/boot-time.rs` 接线**：该单元原来自己声明五个名字（`UTMP_FILE` + 四个 entry 宏），现在全部改从镜像取；
  **只留 `UTMP_STRUCT_NAME` 的类型别名**——C 把它写成"用在类型位置的宏"，端口那里必须是类型别名，而类型与宏不同命名空间、
  与镜像的宏不冲突（镜像的注释也这么说）。`--check` 结论不变（OK）、`build-tu lib/boot-time.c` built。
  ③ **一个真的账目 bug（已修）**：`src/process.c`（**9,122 行 / 125 函数 / 63 subr**）**其实早就完工**——
  `--check` OK/0 行、台账 235 框全勾、文件已入库、`build-tu src/process.c` built——但 `gen-port-inventory` 一直把它记成
  WIP。根因是状态判据用了**裸子串** `"- [ ]" in text`，而该台账的 Evidence 里引着自己的命令
  `grep -c '^- [ ]' docs/ports/process.md -> 0`，于是**散文里的那五个字符被当成未勾项**。改成"**以 `- [ ]` 开头的行**"
  （`^\s*- \[ \]`）后，全树只有这一份台账的两种读法不同，**`src/process.c` 记 `done`**——这是本树**最大**的一个单元。
  这与 3.196（排除文件的续行被当文件名）**同一类**：台账/清单是散文，读者必须对**形状**下约束。
  ④ 同轮 lane 侧：**xdisp 第 13 轮**（C 11647-12525、879 行；`missing function` 174→164、`missing subr` 10→7、
  xdisp.rs 13,611 行），它要的 note 行累计 **5 条**，仍压在闭合时由我写；**readutmp lane** 已收到"`intprops` 一对放行 +
  先查 `intprops-internal.h` 的 C 18 守卫 disagreement"的派工。
  ⑤ 读数：`gen-port-inventory` **590 files、done=563、wip=17、todo=10**；门全绿
  （`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/1/0、`layout-check` 89 of 99 + 10 跳过）。

- **3.199 `src/nsterm.h` 落地；三份台账的陈旧勾选清掉；把"WIP 复核"做成常驻工具（2026-10-04）**：
  ① **实产**（`8b21bb3`，签名 `verified=true`，已推 `origin/master`）：`src/nsterm.h`（1,395 行：140 宏 +
  21 常量 + 102 字段 + 8 全局）→ `femacs/src/include/nsterm.rs`（1,237 行），`--check` **OK**、台账框全勾
  ——这是 macOS 两个终端单元（`nsfns.m`/`nsterm.m`）要用到的头。
  ② **三份台账的陈旧勾选**（与 3.198 的 `process.c` 同一类）：`--check` 都 OK 但框没勾，逐名核对后确认
  **镜像确实带这些名字**，于是补勾 + 补 Evidence：`lib/execinfo.in.h` 的那一条是**已登记的 excluded 名**
  （`_GL_EXECINFO_INLINE`）；`lib/acl.h` 的五条是 split guard + C 活臂 `#else` 定义的两个 `aclinfo` 宏 +
  `aclinfo_scontext_free` + `aclinfo` 记录本身；`lib/stdlib.in.h` 的十条就是我 3.193 写进 `excluded-macros.txt`
  的九行加 `MB_CUR_MAX` 的 notes 行——它的 `random_data` 一条**故意不勾**（死臂 + 本 target 无该类型），
  所以 extractor 打的是 `missing type deferred` 而不是失败。
  ③ **新增常驻工具 `tools/audit-wip.py`**：把"WIP 复核"从一次性脚本变成可重跑的门。它读 `port-inventory.md`
  的 `wip` 行，对每一行跑 `--check` 并数台账未勾框，分成三组——**A** 检查失败且有余框（真的没做完，不用管）；
  **B** 检查 OK 但有余框（去读该台账的 `## Decisions`：**有意的 deferral** 还是**陈旧框**）；
  **C** 检查 OK 且框全勾（**inventory 错了，这单元其实完工**，`exit 1` 让包装脚本能抓）。**当前读数**：
  A=8（`lib/getopt-pfx-ext.h` 11 行、`lib/getopt.in.h` 3、`lib/stdio.in.h` 6（在飞）、`src/dispextern.h` 142、
  `src/font.h` 49、`src/frame.h` 47、`src/w32common.h` 2、`src/xdisp.c` 229（在飞））、**C=0**（不再有账目错误）、
  B=6 且**全部**是台账自己写明的有意 deferral（`c-ctype.h` 的 19 条是它 Decisions 里写着的 WIP；
  `stdlib.in.h` 的 `random_data`；`emacsgtkfixed.h` 2、`w32.h` 1、`w32gui.h` 1、`widgetprv.h` 3）。
  **A 组里没有 lane 的**（`font.h`/`frame.h`/`getopt` 一对/`w32common.h`）就是下一批要派的活。
  ④ 读数：`gen-port-inventory` **590 files、done=566、wip=14、todo=10**；门全绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-verbatim` 4607/0、`layout-check passed`）。

- **3.200 getopt 前缀层闭合（`lib/getopt.in.h` + `lib/getopt-pfx-ext.h`）；A 组从 8 降到 6（2026-10-04）**：
  ① **实产**（`347b2ef`，签名 `verified=true`，已推 `origin/master`）：这两件是 `tools/audit-wip.py` 的 A 组里
  **没有 lane 认领**的两行，现在都 **`--check` OK/0 行**。**Rust 能承载的就承载**：`getopt-pfx-ext.rs` 加上守卫
  （`_GETOPT_PFX_EXT_H`）与 C 60-63 的两个值（`__getopt_argv_const` 作空宏标记、`__getopt_argv_const_is_empty` = 1）
  ——后者正是 `getopt-ext.rs` 的原型收 `char **argv`、`lib/getopt1.rs` 的 `ARGV_CAST` 是恒等的原因。
  ② **承载不了的按类登记**（9 条 `excluded-macros.txt`）：三个粘贴宏（`__GETOPT_CONCAT`/`__GETOPT_XCONCAT`/
  `__GETOPT_ID`，`syntax`——标识符粘贴在 Rust 无拼法，端口在每个使用点写出结果）；三个重命名（`getopt_long`/
  `getopt_long_only`/`_getopt_internal`，**`linkage`**——端口的表达是声明处的 `#[link_name = "rpl_getopt_long"]`
  与定义处的 `#[unsafe(export_name = …)]`，实测参照对象 `nm build/emacs-ref/lib/getopt1.o` 定义 `_rpl_getopt_long`）；
  `getopt.in.h` 的三个瞬时标记（`_GL_SYSTEM_GETOPT`、`__need_system_stdlib_h` 是 `config`；`option` 是
  include 期的重命名，`syntax`）。两份台账补齐 Decisions + Evidence。
  ③ **编译证据**：include 链是真的（`lib/getopt1.rs` → `getopt.in.rs` → 两个 pfx 镜像），所以
  `build-tu lib/getopt1.c` built。`getopt-pfx-core.h` 早在之前就已 OK（台账 0 未勾）。
  ④ **审计读数**：`tools/audit-wip.py` 的 **A 组 8 → 6**（剩下的 6 条是两件在飞单元 + `src/font.h` 49 行、
  `src/frame.h` 47 行、`src/w32common.h` 2 行），**C 组仍为 0**。`src/w32common.h` 的两条是 `sysinfo_cache`/
  `osinfo_cache` 两个 Windows 对象（本 target 无可测类型），做法与 `w32gdiplus.rs` 同型：挂
  `#[cfg (feature = "windowsnt")]`、照 C 写 C 的名字 ⇒ 留给下一批（连同 `font.h`/`frame.h`，它们没有 lane）。
  ⑤ 读数：`gen-port-inventory` **590 files、done=568、wip=12、todo=10**；门全绿（`check-declarations` 13/0、
  `check-globals` 283/0、`check-mirror-dupes` 1/1/0、`layout-check` 90 of 100 + 10 平台独占跳过）。

- **3.201 四个单元交出 `conf_post` 的名字；两处陈旧注释改正（2026-10-04）**：
  ① **实产**（`dbe36bb`，签名 `verified=true`，已推 `origin/master`）：`lib/time_rz.rs`、`src/sysdep.rs`、
  `src/alloc.rs`、`src/emacs.rs` 各自 `include!` 新的 `conf_post.rs` 镜像，并把它们自己声明的副本删掉
  （`getenv_TZ`/`setenv_TZ`、`ADDRESS_SANITIZER`、`VFORK`、`EXTERN_INLINE`/`INLINE`）。`emacs.c` 那一处**先核语义再动**：
  C 21 的 `#define INLINE EXTERN_INLINE` 与 `conf_post.h` 358 的**逐字相同**，且 emacs.c 写在 `<config.h>` 之前，
  差别只是"两份同义文本哪一份生效"，而端口里**没有任何 `INLINE! (…)` 展开**（lisp.h 的 INLINE 函数一律
  `#[inline(always)] fn`，P-12）⇒ 无可观察差别，可以只留一个落地点（P-26）。
  ② **两处陈旧陈述改正**：`window.rs` 3346 说 `UNINIT` 在本 target 是 `/* empty */`，实测 `GCC_LINT` 是 1 ⇒
  C 369 的 `= {0,}` 才是活臂（其台账的 deviation 条目同改）；`alloc.rs` 288-292 说 `ADDRESS_SANITIZER` 未定义，
  实测它定义为 0。**该编辑必须零行差**：引证允许表按行号索引 `femacs/src/window.rs:5452`，多加一行就会把该 finding
  推出范围（实测 `13 drift …, 1 stale`）；改成同尺寸的一行后门恢复 `12 drift / 3 range / 36 allowed / 0 stale`。
  ⇒ 给所有 lane 的教训：**改 `.rs` 的行数会移动其下方所有引证 finding**，要么零行差改，要么先改共享表。
  ③ 五个单元的 `--check` 结论不变（都 OK），四个被改的单元都 built。

- **3.202 条件求值器的 marker 对含连字符的文件名无效，7 条"已验证"是空洞的（已修，2026-10-04）**：
  ① **实产**（`6b5a238`，签名 `verified=true`，已推 `origin/master`）：`tools/check-cond-eval.py` 原来把分支 marker 写成
  `EMACS_COND_TAKEN_{stem}_{line}`；**stem 含连字符或点**时那不是合法 C 标识符，于是 clang 把
  `#define EMACS_COND_TAKEN_intprops-internal_17 1` 读成一个名叫 `EMACS_COND_TAKEN_intprops`、body 是
  `-internal_17 1` 的宏（并给 `ISO C99 requires whitespace after the macro name` 警告），工具的
  `"#define <marker> " in out` 永远不匹配 ⇒ **clang 对"冻结构建实际走的分支"一律被记成 dead**。marker 现已消毒。
  ② **后果（实测）**：用修好的工具重跑 `cond-eval-verified.txt`，**7 个文件**的"每个条件都与 clang 一致"是那个 bug 的
  产物：`lib/alloca.in.h`、`lib/arg-nonnull.h`、`lib/c++defs.h`、`lib/getopt-cdefs.in.h`、`lib/getopt-core.h`、
  `lib/getopt-pfx-core.h`、`lib/warn-on-use.h`（全部是含连字符/点的 stem）。清单按工具重生成（436 → **424**），
  这 7 个从此**盲读**（P-66 的保守政策：每个 `#if` 的两支都要）而不再咨询一张没人替它们验证过的表。
  ③ **补洞**：三个镜像补上它们新清单要的守卫（`_GETOPT_CDEFS_H`、`_GETOPT_CORE_H`、`_GETOPT_PFX_CORE_H`），
  38 个 Rust 无法承载的名字写进 `excluded-macros.txt`（`alloca` ⇒ 栈池 P-05；getopt 六个重命名 ⇒ `linkage`
  （声明处 `#[link_name]`、定义处 `#[export_name]`）；`__BEGIN_DECLS`/`__END_DECLS`/`__THROW`/`__GNUC_PREREQ`
  四个声明拼法；四个 `_GL_WARN_ON_USE*` 属性；`c++defs.h` 的 22 个 C++ 兼容宏）。六份台账按抽取器**重新生成**
  （旧清单正是在那次空洞验证下产出的），六件 `--check` 全部恢复 **OK**。
  ④ **`tools/audit-wip.py` 增加 `--done`**：查的另一半问题——inventory 记 `done` 而 `--check` 其实失败的行
  （3.202 这一类的可见化；WIP 审计看不见它们）。**本轮读数：568 个 done 行逐个 `--check`，失败 0 个。**
  ⑤ 这条是 **conf_post lane 从探针里那句 clang 警告读出来的**（它把读数贴进了 `include-readutmp.md`），
  与 3.196/3.198 同源：**门的读数反了，比没有门更糟**。

- **3.203 `lib/stdio.in.h` 落地并闭合；`src/w32common.h` 闭合；A 组只剩 4（2026-10-04）**：
  ① **实产**（`3d00656` + `5376f55` + `d87717b`，签名 `verified=true`，已推 `origin/master`）：
  `lib/stdio.in.h`（2,224 行 / 85 项）→ `femacs/lib/include/stdio.in.rs`（307 行：32 条声明进
  `stdio_in_decls` 模块 + 12 `macro_rules!` + 11 `pub const`）。**判据实测**：`getline` 的 `@GNULIB_GETLINE@` →
  生成头 1684 `#if 1`、`@REPLACE_GETLINE@` → 1691 `# if 0`（所以 RPL 臂全死，`HAVE_DECL_GETLINE` = 1 连 gnulib 自己的
  `_GL_FUNCDECL_SYS` 都跳过 ⇒ **`getline` 就是系统的**）；14 个活宏里**没有任何 `#define X rpl_X`**，所以本件按规则
  保留 C 源名、不需要 export_name 单元。我写 **6 条 excluded 行**（`getline` 的函数项、`fcloseall`、`fwrite_unlocked`、
  `obstack_printf`、`obstack_vprintf`、`rpl_getdelim`）⇒ **`--check` OK/0 行**、7 个框勾上、单元离开 WIP。
  **lane 的探针当场抓到一个真 `E0428`**（它原把模块叫 `stdio_decls`，而 `stdio.rs:23` 已拥有该名）⇒ 改名 `stdio_in_decls`。
  ② **`__has_feature` 行的理由更正**：那行原说"冻结配置既没有该 builtin 也没有该宏"，实测 clang **有** `__has_feature`
  （`printf '#ifdef __has_feature…' | clang -E` 打印 `HAS_FEATURE`）⇒ 跳过的是模板的 fallback，活臂是 builtin 本身；
  结论不变、理由改了。③ `vfork` 行的措辞按 lane 的核查收紧为"没有**已声明**的 `[features]` 名字"（台账词汇把那个
  feature 叫 `address_sanitizer`，但树里没有该声明）。
  ④ **`src/w32common.h` 闭合**：两条缺项是两个缓存对象，按 `w32gdiplus.rs` 同型挂 `#[cfg (feature = "windowsnt")]`
  并照 C 写 C 的类型名（该 target 的 `<windows.h>` 供给它们；feature 关闭时在名字解析前被剥离 ⇒ `layout-check` 编过它
  就是证据）。三个 DLL 装载宏的行**早已存在**（`excluded-macros.txt` 149-151，`kind=deferred`：粘贴 + `__VA_ARGS__`
  + 从外层函数 `return`，`macro_rules!` 一样都表达不了，由 Windows target 首次展开时定形状）。
  ⑤ **读数**：`gen-port-inventory` **590 files、done=571、wip=10、todo=9**；`tools/audit-wip.py` 的 **A 组降到 4**：
  两件在飞单元（`src/dispextern.h`、`src/xdisp.c`）+ **`src/frame.h`（47 行）**、**`src/font.h`（49 行）**——
  这两件**没有 lane**，已派给刚收工的 stdio lane（并发纪律写进简报：先重读文件、只追加、绝不回退别人的项、
  `frame.h` 做完先报一次）。
  ⑥ 门：`check-declarations` 13/0、`check-globals` 283/0、`check-mirror-dupes` 1/1/0、`check-verbatim` 4607/0、
  `layout-check` 89 of 99 + 10 平台独占（本轮它编过 `w32common.rs`）；`check-citations` 12 drift/3 range/0 stale。

- **3.204 `lib/intprops-internal.h` 与 `lib/libc-config.h` 落地（2026-10-04）**：
  ① **`lib/intprops-internal.h`**（`c1161a8`）：镜像 437 行 / **29 宏**（0 类型、0 常量——抽取器说的"3 类型"
  实测是三个**常量**，dup 2/4 让常量规则直接跳过、一条都不要求），台账 29/29、`--check` OK/0 行、**0 条共享行**。
  **按盲读政策落**（该件不在 `cond-eval-verified.txt`、也**不该**在：它的 7 条 disagreement + 1 条 unknown 是本文件自己的
  `__typeof__` 探针与 `__has_builtin` 表达式，不是配置宏，任何镜像改动都换不来 verified）⇒ 每个 `#if` **两支都带**、
  死臂各挂一个命名"非冻结一侧"的 feature。判据不是注释而是**直接实测**：`-dM` 带/不带本头对拍 + `comm -13`
  （`_GL__GENERIC_BOGUS` 在 clang 是 1 ⇒ `_Generic` 臂死、C 274 的 `sizeof` 链活；`_GL_TYPE_SIGNED` 是
  `(! ((t) 0 < (t) -1))`；`_GL_HAS_BUILTIN_OVERFLOW_P` = `__has_builtin (__builtin_mul_overflow_p)` 而
  Apple clang 答 0 ⇒ `_GL_INT_NEGATE_OVERFLOW` 走 range 臂；`LLONG_MAX`/`LLONG_MIN` 的 C 93-96 臂死因为
  `<limits.h>` 已定义）。两条 Rust 词法规则也记进台账：**C 的反斜杠续行不能带进 Rust**（116 处会报
  `unknown start of token: \`），`?:`/`sizeof`/`__typeof__` **保持 C 文本**（这些宏在端口没有展开点）。
  ② **`lib/libc-config.h`**（`fd68038`）：这是那五个已端口单元（`tempname`/`canonicalize-lgpl`/`strftime`/`mktime`/
  `timegm`）在 C 里的**第一个** include，此前**没有镜像**。现在有：`_FEATURES_H`=1、`__GNULIB_CDEFS`/`__inline` 作标记、
  `__set_errno` 取 fallback 宏（本 target 的 `<errno.h>` 没有它）、`__GNUC_PREREQ`/`__glibc_clang_prereq` 取 0 体、
  `SHLIB_COMPAT`=0、七个空载 glibc 标记（`attribute_hidden` + 四个 `libc_hidden_*` + `strong_alias`/`weak_alias`）作
  no-op；`compat_symbol`/`versioned_symbol`（C body 是 `extern int dummy`，glibc 兼容符号机制用的声明）记进
  `excluded-macros.txt`。全部用绝对 `core::ffi::*`、不取 includer 任何名字 ⇒ **它自成一编译单元**
  （`build-tu lib/libc-config.h` built），`--check` OK/0 行。**跟随项**：那 7 处本地副本
  （5 个 `__set_errno!` + `regcomp`/`regexec` 的 `weak_alias!`/`libc_hidden_def!`）现在与镜像**同名**，
  同 crate 两份定义会报错 ⇒ 已派给 conf_post lane，**一次一个文件**（每个都 `--check` 不变 + `build-tu`）。
  ③ 读数：`gen-port-inventory` **590 files、done=572、wip=10、todo=8**；`tools/audit-wip.py` 的 A 组仍是 4
  （`dispextern.h`/`xdisp.c` 在飞 + `frame.h`/`font.h` 已派给 stdio lane）；C 组 0。

- **3.205 `mempcpy` 导出 C 自己的符号，三个用点停用 `rpl_mempcpy`（2026-10-04）**：
  ① **3.112 的结论是错的**。那一条读的是生成头 `build/emacs-ref/lib/string.h` 的 `# undef mempcpy` /
  `# define mempcpy rpl_mempcpy`（它引的是该头 1027-1028 行），据此把定义端写成
  `#[unsafe(export_name = "rpl_mempcpy")]`，并让 `src/doprnt.rs`（3 处）、`lib/canonicalize-lgpl.rs`（宏体 + 声明）、
  `lib-src/etags.rs`（声明 + 5 处调用）去调这个**发明出来的**名字。实测：那两行躺在**第 1025 行的 `# if 0` 里**
  （3.112 漏读了外层条件），`nm -g build/emacs-ref/lib/mempcpy.o` 打 **`T _mempcpy`**——C 的标识符根本没被改名。
  ② 修正（`dc302aa`）：定义端回到 `#[unsafe(no_mangle)]`，三个用点回到 `mempcpy`。`export_name` 真正的用场是
  **真的替换了 libc 函数**的那些单元——实测 `T _rpl_mktime`（参照 `lib/mktime.o`）、`T _rpl_re_search`（参照
  `src/regex-emacs.o`）——`tools/extract-abi.py` 里拿 mempcpy 当例子的那段注释改成这两件。
  ③ 读数：`check-symbols.py --dir lib --pending …` **86 units / 0 mismatched / 1 pending**（唯一 pending 仍是
  `lib/regex` 的聚合拆分，`symbol-pending.txt` 已记录；`--dir src` 73 units / 0 mismatched；`check-linkage` 3 units / 0 missing）；
  `nm -g build/femacs/{mempcpy,doprnt,canonicalize-lgpl,etags}.o` → `T _mempcpy` + 三个 `U _mempcpy`；
  `check-undef.py` 的 `_rpl_mempcpy` **消失**，NEW 只剩 `src/xdisp.c` 的 6 个待落函数（它们由 xdisp 自己后段定义）。
  ④ 教训：**读生成头（生成文件）的一行，必须连它的外层 `#if` 一起读**——与 3.196（排除清单的续行被当文件名）、
  3.198（台账散文里的 `- [ ]` 被当未勾项）、3.202（非法 marker 让 7 个文件的"已验证"变空洞）是同一类"把片段当事实"。
  另外 `build/` 的 `.checking.o` 会过期：3.112 留下的旧对象仍报 `_rpl_mempcpy`，判据前先按 §四 的命令重编。

- **3.206 端口树自己的 README 不再把产品名写进标题（2026-10-04）**：
  ① 2026-10-03 的两道指令扫的是**被复制来的代码**（版本行的产品括注 473 件、复制来的声明块与首行标题 268 件，
  `2ee1d1d` / `4810bb0` / `fd9cf89`）。`femacs/README.md` 的首行
  `# femacs — the Rust replication of GNU Emacs` **是我们自己写的句子**，而 `check-declarations.py` 只扫被移植的树
  （`femacs/{src,lib,lib-src,exec,lwlib,oldXMenu,nt,platform,xtask}`），所以它扫不到这一件，那一轮也就没动它。
  ② 现在改成 `# femacs — the Rust rewrite`（`30ff6c2`）——文件下面本来就写着 "the ported sources and the tooling that
  builds them"、"`src/` mirrors the reference C tree"、"The reference checkout is read-only"，标题不需要点名。
  ③ 复核：`rg -c 'GNU Emacs' .`（扣掉六棵逐字节树 `femacs/{lisp,test,etc,doc,leim,admin}`、`femacs/target` 与参照 `emacs/`）
  只剩——门自己的模式表（`tools/check-declarations.py` 10）、实测值与台账行（`docs/baseline` 10）、描述参照或许可的散文
  （`docs` 13 处 + 根 `README.md` 1 处许可句）、以及三个被移植单元里 **C 自己的字符串字面量**（`src/emacs.rs` 3、
  `src/callproc.rs` 3、`lib-src/etags.rs` 2；全部在 `declarations-allowlist.tsv` 里——docstring 门要与参照逐字对比，
  改不得）。`check-declarations` **13 sites / 0 forbidden / 13 allowed literals / 0 failures**。

- **3.207 `lib/sys_time.in.h` 落地：模板镜像的活臂由**生成头**决定，而不是 evaluator（2026-10-04）**：
  ① 判据：`--check emacs/lib/sys_time.in.h femacs/lib/include/sys_time.in.rs` 要的是**所有臂的并集**——
  **21 宏 + 常量 `GNULIB_defined_struct_timeval` + 类型 `timeval`**（模板里的 `@NAME@` 条件工具读不了，P-66），
  而 `check-cond-eval.py lib/sys_time.in.h` 给 `31 conditions, 1 unknown, 2 disagreements [NOT VERIFIED]`
  ⇒ 这一件是**盲读**（P-51）：每个 `#if` 两支都带，死臂挂命名"非冻结一侧"的 feature。
  ② 活臂全部来自**生成头**（实测，不是表）：`diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_time.in.h)
  build/emacs-ref/lib/sys/time.h` 给出每个替换值，其中三条决定形态——`@HAVE_STRUCT_TIMEVAL@`=1（生成头 627 读
  `#if !1 || 0`）⇒ **C 87-92 自带的 `struct timeval` 与 C 83 的 `timeval rpl_timeval` 都是死的**，端口的活
  `timeval` 是系统半边 `sys_time.rs`（C 40/45 的 `#include_next`，P-26/P-50）；`@GNULIB_GETTIMEOFDAY@`=1 配
  `@REPLACE_GETTIMEOFDAY@`=0（生成头 648/649）⇒ **SYS 臂活、`gettimeofday` 不改名**；`@HAVE_WINSOCK2_H@`=0
  ⇒ winsock2 块（C 64-68）与 18 个 `*_used_without_including_*` 改名（C 142-231）**全死**，挂 `windowsnt`。
  `_GL_CONFIG_H_INCLUDED`（C 30）与 C 39 的系统头 guard 就是那两条 disagreement 的来源，两者都记进台账表格。
  ③ **"这一件没有消费者"也是实测的**：`cargo run -p xtask -- build-tu lib/sys_time.in.h` → 3 条 E0425
  （`timeval`/`c_void`/`c_int`）；同类模板镜像 `build-tu lib/stdlib.in.h` → E0425 `c_int`/`c_char`。原因是
  `.in.h` 镜像的标量来自 includer（PORTING.md 0.1-9），而本 target 的消费者直接 include **系统半边**
  （`src/include/systime.rs` 40、`lib/boot-time.rs` 51、`lib/include/boot-time-aux.rs` 34）——这条写进台账
  `docs/ports/include-sys_time.in.md` 的 Decisions（与 `include-stdlib.in.md` 258 的"无 build-tu 消费者"同形）。
  ④ 读数：`gen-port-inventory` **590 files、done=575、wip=10、todo=5**（由 573/10/7 推进一格）；
  **剩下的 5 件 todo**：`lib/cdefs.h`(900)、`lib/signal.in.h`(536)、`lib/stdcountof.in.h`(124)、
  `lib/sys_select.in.h`(342)、`src/xterm.h`(2,045)（`src/w32term.h` 已被 Poincare lane 落盘、转 wip）。
  门：`check-declarations` 13/0、`check-mirror-dupes` 1 duplicate/1 ratchet/0、`check-target-names` 621/593/0。

- **3.208 `lib/sys_select.in.h` 落地：`pselect` 的 SYS 臂活，而 `select` 根本不在这次构建里（2026-10-04）**：
  ① 同一套判据（见 3.207）：**生成头**决定每一支——`build/emacs-ref/lib/sys/select.h`（36,552 字节），
  `diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/sys_select.in.h) build/emacs-ref/lib/sys/select.h` 逐条给出替换值；
  `check-cond-eval.py lib/sys_select.in.h` 给 `40 conditions, 18 unknown, 29 disagreements [NOT VERIFIED]`
  ⇒ 盲读（P-51/P-66），每个 `#if` 两支都带。
  ② 三条结构性读数（生成头行号）：`@HAVE_SYS_SELECT_H@`=1（33/42/77）配 `@HAVE_WINSOCK2_H@`=0（661、696 两个 `#if 0`）
  ⇒ C 154-174 的 `suseconds_t` typedef、**`rpl_fd_isset` 函数**（body 逐 token 转写）与 `FD_ISSET` 覆盖、
  以及 C 176-254 的 18 个 `*_used_without_including_*` 改名全死（挂 `windowsnt`）；`@GNULIB_PSELECT@`=1 /
  `@REPLACE_PSELECT@`=0 / `@HAVE_PSELECT@`=1（777/778/790）⇒ **SYS 臂活、`pselect` 不改名**；
  **`@GNULIB_SELECT@`=0（813 读 `#if 0`）**⇒ `select` 模块**不在这棵构建里**，C 292-317 的每一支都是死的
  （`rpl_select` 改名、SYS 别名、`select_used_without_requesting_gnulib_module_select`），名字只好从死臂承载。
  ③ 系统半边仍是 `sys_select.rs`（C 39/51/60/88 的 `#include_next`），另加 `sys_types.in.rs`（C 75）与
  C 103-105 的 `<signal.h>`（**活的**：实测 `!((__GLIBC__ || __CYGWIN__ || __KLIBC__) && !__UCLIBC__)` 为真）。
  ④ 读数：`gen-port-inventory` **590 files、done=576、wip=10、todo=4**（由 575/10/5 推进一格；
  剩 `lib/cdefs.h`(900)、`lib/signal.in.h`(536)、`lib/stdcountof.in.h`(124)、`src/xterm.h`(2,045)）；
  门：`check-declarations` 13/0、`check-mirror-dupes` 1 duplicate/1 ratchet/0、`check-target-names` 622/593/0。

- **3.209 `lib/signal.in.h` 落地：这棵构建里 gnulib 一个名字都不用改（2026-10-04）**：
  ① 同一套判据（3.207/3.208）的第三件：生成头 `build/emacs-ref/lib/signal.h`（41,625 字节），
  `check-cond-eval.py lib/signal.in.h` 给 `87 conditions, 34 unknown, 44 disagreements [NOT VERIFIED]`
  （3.136 已把这一件列进"需要 P-51/P-66 承载"的名单）⇒ 盲读；镜像头部那张表是把模板与生成头**同步走**
  读出来的字面替换表。
  ② **活的一半**（也就是镜像的真文本）：`sighandler_t`（`@HAVE_SIGHANDLER_T@` **0**，生成头 663 读 `#if !0`
  ——SDK 没有这个类型，所以 C 自己的 typedef 就是移植文本）、`SIG2STR_MAX`、`sig2str` 与 `str2sig`
  （`@GNULIB_SIG2STR@` 1 配 `@HAVE_SIG2STR@`/`@HAVE_STR2SIG@` **0** ⇒ `_GL_FUNCDECL_SYS` 是活声明）、
  `pthread_sigmask`（`@REPLACE_PTHREAD_SIGMASK@` 0 ⇒ SYS 臂）、C 519-523 的 `SA_NODEFER` fallback，
  以及 C 78-86 的 `<pthread.h>` 臂（它的 `__APPLE__ && __MACH__` 操作数为真）。
  ③ **死的一半**（都要带，因为 gnulib 正是"自己定义这些类型"的那一侧）：`@GNULIB_SIGPROCMASK@` 0
  （整段 C 249-441：`SIG_BLOCK`/`SIG_SETMASK`/`SIG_UNBLOCK`、`GNULIB_defined_signal_blocking`、
  `verify_NSIG_constraint` 断言 typedef、`_gl_function_taking_int_returning_void_t`、`signal` 改名）、
  `@GNULIB_SIGACTION@` 0（C 444-515 的 `union sigval`、`struct siginfo_t`、带匿名 `_sa_func` 的
  `struct sigaction`、`sa_handler`/`sa_sigaction`、`SA_RESETHAND`/`SA_RESTART`）、
  `@GNULIB_SIGNAL_H_SIGPIPE@` 0、`@GNULIB_RAISE@` 0、`@HAVE_SIGSET_T@` 1、`@HAVE_TYPE_VOLATILE_SIG_ATOMIC_T@` 1。
  ④ **两半的名字写进台账**：C 41/57 的 `#include_next` 是冻结的 `<signal.h>`，端口那一半的镜像是
  `signal.rs`（信号号、`sigset_t`、Darwin 的 `union __sigaction_u` + 带实测布局断言的 `struct sigaction`、
  `SIGPIPE`=13）；本文件是 gnulib 包装层，上面那些读数就是"它在这棵构建里什么都不改"的证明。
  ⑤ 读数：`gen-port-inventory` **590 files、done=577、wip=10、todo=3**（由 576/10/4 推进一格；
  剩 `lib/cdefs.h`(900)、`lib/stdcountof.in.h`(124)、`src/xterm.h`(2,045)）；门：`check-declarations` 13/0、
  `check-mirror-dupes` 1 duplicate/1 ratchet/0（cfg 掉的类型定义没有新增重复）、`check-target-names` 622/593/0。

- **3.210 `lib/intprops.h` 入库（lane 交付，集成者复核）＋两条待裁决留在报告里（2026-10-04）**：
  ① `femacs/lib/include/intprops.rs` + `docs/ports/include-intprops.md` 入库（`3083d23`）。镜像头部写着判据：
  `check-cond-eval.py lib/intprops.h` → `4 conditions, 0 unknown, 2 disagreements [NOT VERIFIED]`，两条都在
  同一条 `__STDC_VERSION__ < 201112 || (defined _MSC_VER && _MSC_VER < 1944) || …`（C 28 与 C 66），
  而 `_GL__GENERIC_BOGUS` 在冻结 clang 上是 1（存活的定义是 `include/intprops-internal.rs` 的 C 34-38 臂）
  ⇒ 本文件走**非 `_Generic`** 一侧，所有臂按 P-51 都带；6 个多定义宏（`TYPE_IS_INTEGER`/`TYPE_MAXIMUM`/
  `TYPE_MINIMUM`/`_GL_ADD_OVERFLOW`/`_GL_MULTIPLY_OVERFLOW`/`_GL_SUBTRACT_OVERFLOW`）门只要求在场。
  ② **我在这个 HEAD 重测的证据**：`--check` → **OK**、台账 **0 个未勾**；`check-mirror-dupes` **1 duplicate /
  1 ratchet row / 0 failures**——某条 lane 中途读到的 **18 条 `INTERNAL_BORDER_*` 失败是在飞的 `frame.rs` 对**
  （`src/frame.rs` / `src/keyboard.rs` vs `src/include/frame.rs`），现在已消失，**共享门是绿的**；
  `check-declarations` 13/0；`check-target-names` 622/593/0；`build-tu lib/lchmod.c`（该头最小的包含者）→ built。
  ③ 读数：`gen-port-inventory` **590 files、done=577、wip=10、todo=3**（剩 `lib/cdefs.h`(900)、
  `lib/stdcountof.in.h`(124)、`src/xterm.h`(2,045)）。
  ④ **两条留给下一轮的裁决（来自 frame.h lane 的报告，我没有代它决定）**：
  (a) `src/frame.h` 镜像要的 **2 条共享行**：`excluded-macros.txt` 的 `WINDOW_SYSTEM_RETURN`（kind=syntax，
  C 1471-1475 的 `_Noreturn ATTRIBUTE_COLD`，端口写 `-> !`）与 `macro-body-notes.tsv` 的 `FRAME_MOUSE_UPDATE`
  的 `false`（C 的 `do { … } while (false)`）；
  (b) **12 项 ownership**：10 个 `INTERNAL_BORDER_*` + `internal_border_part` 已被 `src/frame.rs`（351-362）
  与在飞的 `src/keyboard.rs` 定义，`FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL` 已被
  `src/dispnew.rs`/`src/term.rs`/`src/xfaces.rs` 定义且那三处是**带类型**的更好端口；镜像里再放一份就是
  `E0428`（lane 实测 `build-tu src/frame.c` 10 条 E0428 + 1 条 `E0659`），所以它**只记录不动**，迁移按
  3.181 的 `stdbit.in.rs` 规则逐项裁决——其中能"被镜像修好"的唯一 unit 是在飞的 `keyboard.rs`，要等它落地。
  同一份报告里还留了一条观察：`src/font.rs` 689 把 `decode_window_system_frame` 声明成返回 `*mut frame`，
  而本 target 的 `WINDOW_SYSTEM_RETURN` 是 `_Noreturn`，那里应为 `-> !`（未改，只报）。

- **3.211 `src/frame.h` 的 ownership 裁决：镜像收下 enum，三件 inline 留在各自单元（2026-10-04）**：
  ① **裁决的依据与做法**：这一件有 14 条 `--check` 行。`enum internal_border_part` 与九个枚举值此前**两边都有**
  （`src/frame.rs` 351-362、`src/keyboard.rs` 6155-6166，两处注释自己写着"镜像没有它"），镜像再放一份就是 `E0428`
  ⇒ 按 3.181 的 `stdbit.in.rs` 规则**镜像当 owner、单元副本退休**，本轮就把两份副本删掉（类型与九个常量随
  `include/frame.rs` 进入每个包含者）。**实测**：`build-tu src/frame.c` → built、`build-tu src/keyboard.c` → built，
  间接读这个枚举的三个 unit（`src/term.c`/`src/dispnew.c`/`src/xfaces.c`）也都 built。
  ② **lane 要的两条共享行由我加**（本来就是集成者的活）：`docs/ports/excluded-macros.txt` 的 `WINDOW_SYSTEM_RETURN`
  （kind=syntax，C 1471 的 `_Noreturn ATTRIBUTE_COLD struct frame *`，`AVOID` 先例，端口写 `-> !`）与
  `docs/ports/macro-body-notes.tsv` 的 `FRAME_MOUSE_UPDATE` 的 `false`（C 1411-1423 的 `do { … } while (false)`，
  `FD_SET`/`FD_CLR` 同类）；`FOR_EACH_FRAME` 的框也顺手勾上（它早已满足，是旧框）。加完这两条，`--check` **14 → 3**。
  ③ **剩下三条不是漏，是裁决**：`FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL` 都是 C 会**粘贴进每个
  包含者**的文本，所以"inline 定义"的 1:1 形式就是**每个单元一份**——实测 `macro_rules! FRAME_FACE_CACHE` 在
  `src/alloc.rs` 3467、`src/font.rs` 545、`src/xfaces.rs` 445、`src/xdisp.rs` 1101、`src/window.rs` 920，`fn` 形式在
  `src/term.rs` 364-377 等处；镜像里再放一份就是那些单元的 `E0659`/`E0428`（lane 的实测仍在：`build-tu src/frame.c`
  10 条 `E0428` + `E0659: WINDOW_FRAME is ambiguous`）。**闭合的前置条件**写进台账
  `docs/ports/include-frame.md`：先要有一个共享的家放 `struct face_cache`/`struct face`——它们是 `dispextern.h` 的，
  而那件正是 A 组在飞单元；它带上这两个类型之后，这里的 `frame.face_cache` 才能定型、镜像才能按 C 的 body 收下这三件、
  单元的副本才能退休（与本轮处理 enum 是同一个动作）。台账同时记了一条工具事实：`cinventory` 把 enum 成员折进类型视图，
  而 `--check` 按常量数，所以这九个名字在清单里**没有行**，裁决段就是它们的记录。
  ④ **本轮换来的一条实测教训**：**单元文件的注释里不要写 include 指令的字面拼法**——`xtask` 的 dedupe 会把它当指令、
  整行替换掉，第一版注释因此让 `build-tu src/frame.c` 报 `error: prefix \`mirror\` is unknown`
  （生成物 `build/femacs/units/frame/_root.rs` 352）。注释现在只写镜像的路径。
  ⑤ 读数：`gen-port-inventory` **590 files、done=577、wip=10、todo=3**（`frame.h` 仍是 WIP，但只剩三条**已记录**的行）；
  门：`check-mirror-dupes` 1 duplicate/1 ratchet/0、`check-declarations` 13/0。

- **3.212 `src/w32term.h` 入库（lane 交付，集成者复核）＋A 组里 `frame.h` 降到 3 行（2026-10-04）**：
  ① `src/w32term.h`（929 行）→ `femacs/src/include/w32term.rs` + 台账 `docs/ports/include-w32term.md` 入库
  （`7f03341`）。判据：`check-cond-eval.py src/w32term.h` → `17 conditions, 0 unknown, 1 disagreement
  [NOT VERIFIED]`（镜像按工具那条保守读法走并记录）；清单满：**119 宏 / 10 类型与字段 / 25 文件级对象 / 0 函数**，
  `--check` → **OK**、台账 **0 未勾**。
  ② **它的编译证据是共享门**：这一件**在 layout 种子里**（`docs/baseline/layout-excluded-mirrors.txt` 里没有它），
  本轮在这个 HEAD 重跑 `cargo run -p xtask -- layout-check` → `layout probe ok` + **`layout-check passed`**——
  这一次同时覆盖了 3.211 对 `src/include/frame.rs` 的改动。
  ③ `tools/audit-wip.py` 的 A 组现在是：`dispextern.h`（142 行，在飞）、`font.h`（49）、**`frame.h`（3，全部是本轮
  记录在案的 includer-owned 行）**、`xdisp.c`（176，在飞）。读数：`gen-port-inventory` **590 files、done=577、
  wip=10、todo=3**；门：`check-mirror-dupes` 1/1/0、`check-declarations` 13/0。

- **3.213 两份镜像台账停止复述 3.202 之前的读数（2026-10-04）**：
  3.202 修掉判据 marker 之后，`check-cond-eval.py` 对这两件的读数变了，而台账还在引旧数字。本轮实测：
  `lib/unlocked-io.h` → **`16 conditions, 0 unknown, 0 disagreements`**（原来的"9 条不一致"是修正前的假象；
  系统头那个事实——真 `<stdio.h>` 定义 `clearerr_unlocked` 等——照旧保留，`--write-verified` 重生成后可以进
  verified 清单）；`lib/stddef.in.h` → **`34 conditions, 7 unknown, 9 disagreements`**，九条全是模板的
  `@PLACEHOLDER@` 与 guard 条件，且**根因可测**：模板文本里还带 `@` 时 clang 根本判不了探针
  （`error: missing ')' after 'defined'`），所以那一件的判据是**生成头**而不是对拍；修正前的第十条是
  C 25 的 `#if __GNUC__ >= 3`，正是 3.202 修掉的那一行。两份都是 intprops lane 交回的更正，集成者重测后入库（`2c22229`）。

- **3.214 `lib/cdefs.h` 落地：转录交给一个入库的工具，活臂按实测决定（2026-10-04）**：
  ① **形状**：900 行里 125 个 `#define`、**83 个名字带不止一支**（每个编译器一族），文本本身没有判断可抄，
  所以转录是机械的、可重跑的：新增 `tools/gen-cdefs-mirror.py`（`--reference emacs/lib/cdefs.h --dM <clang -dM 输出>`），
  规则写进它的 docstring 也抄进镜像头部：续行合并、块注释丢弃、C 的参数名改成 Rust 元变量、`...` → `$($va:tt)*`、
  `__VA_ARGS__` → `$($va)*`、**C 的整数后缀剥掉**（Rust 没有 `1ULL`）、**`##` 写成两个带空格的 `#`**
  （Rust 既禁 multi-hash token，也禁"标识符紧跟 `#`"——实测 `__nldbl_##name` 报
  `error: prefix \`__nldbl_\` is unknown`），**clang 活的那一支不带 `#[cfg]`**、其余每支挂
  `#[cfg (feature = "<按它自己的条件生成的 slug>")]` 并把条件原文写在条目上方（Rust 会解析被 cfg 掉的宏臂，
  所以整头都在文件里、每个名字只编一支——P-51 的形状）。
  ② **活臂是实测的**：本 target 没有 `<features.h>`（实测 `fatal error: 'features.h' file not found`），
  cdefs.h 的到达路径是 `lib/libc-config.h` C 181 的 `#include <cdefs.h>`，所以探针是带 config.h 与
  libc-config.h 的 `clang -dM -E`（exit 0，1467 条 `#define`），每一支与"它之下存活的那个定义"对拍。
  `check-cond-eval.py lib/cdefs.h` 给 `85 conditions, 9 unknown, 30 disagreements [NOT VERIFIED]`——
  不一致全是 `__has_attribute`/`__has_builtin`/`__GNUC_PREREQ` 这一族（配置表不建模的编译器能力），
  所以臂是**带**而不是**判**出来的。
  ③ **门**：`--check` → **OK**（122 宏 + 常量 `_SYS_CDEFS_H`/`__LDBL_COMPAT`；80 个名字多支、门只要求在场，
  另外 42 个单定义的 body 逐 token 比）。唯一需要的一条 body note 是 `__fortify_clang_bos_static_lt_impl` 的 `ull`
  （C 的 `-1ULL`，已进 `docs/ports/macro-body-notes.tsv`）。**编译证据最强的一件**：`build-tu lib/cdefs.h` →
  `built lib/include/cdefs.rs`（镜像自成一编译单元；它目前没有消费者——`rg -l cdefs emacs/` 只有
  `lib/libc-config.h`，其 Rust 侧自己声明要用的名字）。门：`check-mirror-dupes` 1/1/0、`check-declarations` 13/0。
  ④ 读数：`gen-port-inventory` **590 files、done=578、wip=10、todo=2**（由 577/10/3 推进一格；
  只剩 `lib/stdcountof.in.h`(124) 与 `src/xterm.h`(2,045)，后者在 Poincare lane 手里）。

- **3.215 `lib/stdcountof.in.h` 落地：占位符的值可测，3.10 的延迟因此收口（2026-10-04）**：
  ① **3.10 的前提太强**：那一条以「模板条件判不了（P-66）」为由延迟，读数 `10 conditions, 4 unknown,
  5 disagreements`；本 HEAD 复测是 `10 conditions, 4 unknown, 4 disagreements`（少的那条正是 3.202 修掉的 marker 假象），
  而四个 unknown **全是模板的 `@NAME@` 占位符**——占位符的值不是求值器的事，它在冻结配置自己的替换表里：
  `grep -n 'STDCOUNTOF' build/emacs-ref/config.status` → `GL_GENERATE_STDCOUNTOF_H_CONDITION=""`（**所以
  `build/emacs-ref/lib/stdcountof.h` 根本不存在**，`ls` 证实）、`HAVE_STDCOUNTOF_H="1"`、`CXX_HAVE_STDCOUNTOF_H="1"`、
  `NEXT_STDCOUNTOF_H="<stdcountof.h>"`。
  ② **由此每一支都是死的**：`HAVE_*` 与 `CXX_HAVE_*` 都是 1 ⇒ C 26 的
  `#if (defined __cplusplus ? @CXX_HAVE_STDCOUNTOF_H@ : @HAVE_STDCOUNTOF_H@)` 在语言测试两侧都为真 ⇒ **活文本是
  C 27 的 `# @INCLUDE_NEXT@ @NEXT_STDCOUNTOF_H@`（系统头）**，整个 body（C 30-122）在本 target 死。门要的三宏一常量一类型
  （`countof`/`_gl_verify_is_array`/`_gl_verify_type`/`GNULIB_defined_struct__gl_verify_type`）因此**全部来自死臂**，
  按 P-51 挂在 `no_have_stdcountof_h`/`cplusplus`/`gnu_c_without_clang` 下——这是本批第三种「死臂承载」形状
  （前两种是 3.207 的 `struct timeval`、3.209 的 `struct sigaction`）。
  ③ **唯一转写不了的**是 C 63-66 的 `template <int w>` 与它「负位宽即报错」的机关：该类型只在 C++ 臂（本 target 死），
  端口带上 C 声明的那个成员并把这个理由记下来（P-08 读死臂的事实）。另加一条 body note：`countof` 的 `__va_args`
  （C 的变参 token 写成 Rust 元变量），在 `docs/ports/macro-body-notes.tsv`。
  ④ **证据**：`--check` → **OK**、台账框全勾；`build-tu lib/stdcountof.in.h` → `built lib/include/stdcountof.in.rs`；
  `check-mirror-dupes` 1/1/0。读数：`gen-port-inventory` **590 files、done=579、wip=10、todo=1**——
  **剩下的唯一一件 todo 是 `src/xterm.h`（2,045 行），在 xterm lane 手里**。

- **3.216 七个单元不再各带一份 libc-config 的名字，改成 include 镜像；外加 `ALIGN_STACK` 的裁决（2026-10-04）**：
  ① **背景**：3.204 才做出 `femacs/lib/include/libc-config.rs`（`lib/libc-config.h` 的镜像——那七个 unit 在 C 里的**第一个** include，却一直没有镜像）。按 3.181/3.211 的 ownership 规则，名字归镜像，于是七个单元把各自长出来的本地副本撤掉：
  `tempname`/`mktime`/`canonicalize-lgpl`/`strtol`/`euidaccess` 的 `__set_errno!`，`regcomp`/`regexec` 的 `weak_alias!` 与 `libc_hidden_def!`（`05843b6`）。
  ② **证据（本轮重测）**：七个 `--check` 全 **OK**；`build-tu lib/{mktime,strtol,tempname}.c` 三件都 **built**；`check-mirror-dupes` 1/1/0、`check-declarations` 13/0；读数 **590 files、done=579、wip=10、todo=1**。
  ③ **两处形态事实照记**：七个调用点**一个都没改**（镜像形参名不同不算语义，`*errno () = $val` 与 `*errno () = ($val)` 是同一个赋值）；**只有三个 C 文件真的写了 `#include <libc-config.h>`**——`strtol.c`/`euidaccess.c` 走自己的 `#ifndef` 兜底，`regcomp.c`/`regexec.c` 是被 `regex.c` 文本包含的 shim（那行在 `regex.c` 24），所以端口里每个 include 落在它 C 源给出那个名字的位置，出处写在注释里。
  ④ **裁决（w32term lane 交回的那一处，C 33-40）**：`#if USE_STACK_LISP_OBJECTS && !defined _WIN64 && !defined __x86_64__ && __GNUC__ + (__GNUC_MINOR__ > 1) >= 5`——clang 判 True、求值器判 False，因为 `USE_STACK_LISP_OBJECTS` **不是 config.h 的宏，是 `lisp.h` 的**（C 5721-5736），配置表看不到它。**裁决：保持现状（两支都带）**，理由是这不取决于哪支"活"：属性的那一支（`__attribute__((force_align_arg_pointer))`）Rust 表达不了、按既有做法只记录，端口的文本本来就是空臂——两种读法下镜像的文字都一样，所以没有必要为它改文件；把"clang 的 True 来自 lisp.h"这条写进台账，保守读法就有了出处。

- **3.217 §3.11 收口：`lib/include` 里能自成一编译单元的镜像进了 layout probe 的种子（2026-10-04）**：
  ① **缺口**：probe 只从 `femacs/src/include/*.rs` 出发、顺着 `include!` 图走，所以 `lib/include` 的镜像只有在被 src 镜像够到时才被编（`stdbit.in.rs` 被 `lisp.rs` 够到；`stdlib.in.rs` 谁都不够到）。
  ② **不能整目录塞，理由可测**：镜像就是头文件，绝大多数要从 includer 拿标量（`c_int`/`size_t`/`time_t`）与 lisp.h 的宏（PORTING.md 0.1-9）。一条命令把目录分好类：
  `cd femacs && for f in lib/include/*.rs; do rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit metadata -Awarnings --crate-name probe_$(basename $f .rs) -o /tmp/probe.rmeta "$f"; done`
  ⇒ **29 个能自成一 crate root、102 个不能**。那 29 个写进 `docs/baseline/layout-standalone-mirrors.txt`（带命令与增删规则），probe 只种它们，**补集由门打印出来**（P-66：两类都不静默）。
  ③ **证据（本轮实测）**：`cargo run -p xtask -- layout-check` → `+ 29 of 131 lib/include mirror(s) seeded as their own crate root (102 need an includer, PORTING.md 0.1-9)`、`92 of 102 src mirror(s) compiled, 10 platform-exclusive skipped (…)`、**`layout-check passed`**。3.214 的 `cdefs.rs` 与 3.215 的 `stdcountof.in.rs` 都在种子里 ⇒ 它们现在由**共享门**编，而不只是各跑一次 `build-tu`。

- **3.218 门的一处"假 done"被修掉，且 590 个目标全部存在（`src/xterm.h` 落地）（2026-10-04）**：
  ① **发现方式**：我在盘 `todo` 时读到 `src/xterm.h` 那一行是 **`done`**，而 `--check` 同时报 7 条问题。
  根因是 `ledger_for` 的**回退**（3.102 起就有的 looseness：没有自己台账的头读它 unit 的台账，`src/indent.h` 读 `indent.md`）被 `--all` 的状态机当成了"这个文件自己的清单"——那份 `xterm.md` 是全勾的，于是半个镜像读成了"完成"。**这正是本项目反复在修的那类错误：拿一个不是它的东西当它的判据。**
  ② **修法（`b03d084`）**：`extract-abi.py` 新增 `own_ledger()`——只有台账首行点名**这个**参照文件时才认它；`--all` 的判定与 WIP 判断都用它，于是"没有自己清单"的文件按**完整条目集**受判，不通过就报 `FAIL … (no checklist of its own)`；`gen-port-inventory.py` 同规则（回退行读 `wip`）并把这一类**打印出来**（P-66：不许静默）。全树只有两个文件吃回退：`src/indent.h`（`--check` 本来就 OK ⇒ 本轮给它补了 `docs/ports/include-indent.md`，勾满 + 读数）与 `src/xterm.h`（它的 lane 自己已经写出 `include-xterm.md`）⇒ **回退类现在是空的**，规则因此只花一个文件的账。
  ③ **读数（修后实测）**：`python3 tools/extract-abi.py --all` → **`checked 590 ported files: 586 done, 4 in progress, 0 failing`**，四个 WIP 都有名有数（`src/dispextern.h` 142 差异、`src/font.h` 16、`src/frame.h` 3、`src/xdisp.c` 176——都由它们自己的台账认领）；`gen-port-inventory` → **590 files、done=580、wip=10、todo=0**，且没有回退行。
  ④ **同轮的第二件事：`src/xterm.h`（2,045 行）落地**（`b03d883`）：镜像 76,101 字节、台账 **0 未勾**、`--check` → **OK**，它在 layout 种子里且共享门在这个 HEAD 复跑过（`92 of 102 src mirror(s) compiled` + `+ 29 of 131 lib/include mirror(s) seeded` + **`layout-check passed`**）。**`todo` 由此归零：590 个参照文件每一个都有镜像**；剩下的是 A 组的 4 件 WIP（`dispextern.h`/`font.h`/`frame.h`/`xdisp.c`，各有 lane）与两条非 C 赛道的在飞产物（macfont/nsfont 的 Swift、`nsmenu.m` 的 Swift、xdisp 第 15 轮）。

- **3.219 `src/xterm.h` lane 的两条解析教训与十个 feature 名入库；外加一条共享门的瞬时红（2026-10-04）**：
  ① **十个 feature 名**（集成者的活，`femacs/xtask/Cargo.toml` 的 `[features]` 表按字母序插入）：`have_gtk3`、`have_x11r6_xim`、`have_xdbe`、`have_xfixes`、`have_xinput2_1/_2/_3`、`have_xrender`、`have_xsync`、`have_xsynctriggerfence`——每个都是它代表的 C 宏的小写拼法。**为什么必须有行**：`gate-probes`/`dead-arms` 就是把这些 feature 打开、把已完成单元的死臂编一遍；表里没有这一行，那个翻转就根本不会发生。证据：`cargo metadata --no-deps` 解析 OK，十个名字各命中一次。
  ② **两条 Rust 解析规则**（xterm lane 实测，`layout-check` 抓的）：`rustc` **会先解析再 cfg 掉**，所以惰性臂也必须是合法 Rust——C 的 `int use;` 要写 `pub r#use`（`error: expected identifier, found keyword \`use\``）；C 的 `(x >> n) as T << m` 必须写成 `((x >> n) as T) << m`（Rust 的 cast 比 `<<` 结得紧，否则报 `'<<' is interpreted as a start of generic arguments for 'c_ulong', not a shift`）。两条都记进本段，凡是 X/GTK/cairo 类的镜像都会再遇到。
  ③ **xterm lane 交回的读数**（我复核过 `--check`/台账/layout-check）：`check-cond-eval.py src/xterm.h` → **`112 conditions, 0 unknown, 0 disagreements [VERIFIED]`**（没有需要裁决的不一致，112 支全按表走）；台账 89/89；`--check` OK。
  ④ **一条共享门的瞬时红（归 font lane，已通知它收敛）**：`check-mirror-dupes` → `FAIL femacs/src/composite.rs: FONT_INVALID_CODE is already declared by femacs/src/include/font.rs (in the include graph)`（1 failure）。这是 `font.rs` 在飞改动带出来的：镜像按 `--check` 要求收下了 `FONT_INVALID_CODE`，`composite.rs` 里那份本地副本就成了重复；按 3.181/3.211 的 ownership 规则应由 unit 退休副本（xterm lane 把 `xterm.rs` 临时移开验证过这条与它无关）。我已把这条连同两条解析规则发给 font lane，并要它附 `check-mirror-dupes` 末行 / `composite.c` 的 `--check` / `build-tu` 三件证据。

- **3.220 三件 lane 交付入库；`face`/`face_cache` 的 ownership 裁决；以及一次被我撞出来的七个红单元（2026-10-04）**：
  ① **入库三件**：①`femacs/src/composite.rs` 退休它那份 `FONT_INVALID_CODE`（font 镜像现在 owns；`check-mirror-dupes` 回到 `1 duplicate / 1 ratchet / 0 failures`、`--check composite.c` OK、`build-tu src/composite.c` built）；②`femacs/src/include/dispextern.rs` 收下 `struct face`（C 1787-1937，sizeof **280**）+ `struct face_cache`（C 1991-2009，sizeof **40**）与 **21 条实测布局断言**——这正是 3.211 给 frame.h 定的前置条件；③`macfont.swift` 10,266 → **10,596** 行（③a：`macfont_open`/`macfont_close`/cache helpers）、`macfont.md` 756 行、`swift-units.tsv` 22 → 18 行（`swiftc -parse-as-library -c` exit 0）。
  ② **被我撞出来的七个红单元**：那两个记录一落进镜像，**七个 unit 同时 `E0428`**（`xfaces`/`xdisp`/`window`/`font`/`term`/`dispnew`/`alloc`——它们各自都有一份 `face`/`face_cache`，而且都 include 这个镜像；实测 `build-tu src/dispnew.c` → "the name `face` is defined multiple times"）。
  ③ **裁决：镜像那份挂 `dispextern_face_records`（== `dead-arms`/`gate-probes` 会翻的那支），unit 的带类型副本保持活的**（`9f07de1`）。理由是量出来的：unit 的副本是**带类型**的（`*mut face_cache`/`*mut font`/`*mut frame`），退休它们（3.181 的 `stdbit.in.rs` 动作）在小单元上也要改 3 处（实测 `alloc.rs` 删掉两条记录后剩 1 条 E0308 + 2 条 `no field header on type c_void`），总量约 40-60 处，而且要碰**两条 lane 的在飞文件**（`xdisp.rs`、`font.rs`）并把它们的类型化访问器降级成 cast。裁定取"保留 C 文本在场、不动任何人的活代码"这一支：镜像那份成为翻转门编的那一支（feature 行写进 `femacs/xtask/Cargo.toml` 的字母序表，带原因），**备选方案、代价与理由都记进镜像注释与 `docs/ports/include-dispextern.md`**，留给后续轮次主动决定。
  ④ **证据**：`build-tu src/{dispnew,term,window,xfaces,alloc}.c` **五件全 built**（改前全红）；`--check dispextern.h` **仍是 142 行**（不变）；`layout-check passed`。
  ⑤ **顺带发现的既有红（不是本轮引入）**：`xtask dead-arms` 在 `src/alloc.rs` 上报 `cannot find function mark_haiku_display`——Haiku 侧的死臂调了一个只有 `haikuterm.rs` 定义的名字，这就是 3.2/3.9 记的那类 FLIPS 缺口（死臂也要能编）。

- **3.221 `dead-arms` 的两个单元修好；一个容差行按实测更新；一条前一段里的错读数更正（2026-10-04）**：
  ① **`xtask dead-arms` 是"把死臂也编一遍"的门**，本轮我把它从"红在第一个单元"推到了只剩 lane 在飞的那一件。修的四处全是**真编译才看得见**的缺陷（真实构建永远不会走到这些臂）：
  - `src/alloc.rs`：`have_haiku` 臂调用 `mark_haiku_display ()` 却没有声明（C 从 haikuterm.h 323 拿到原型），`have_native_comp` 臂把带类型的 `*mut Lisp_Native_Comp_Unit` 传给 `unload_comp_unit`，而 `include/comp.rs` 的声明是**未类型化**指针（P-72 ③，那份镜像自己的注释）⇒ 现在在臂里声明 haiku 名（`dynlib_handle_ptr` 形状）并在那个把类型带回作用域的调用点转换指针。
  - `src/keyboard.rs`：三处死臂把 C 的 `event->kind` 位域**当成 Rust 字段**写（`(*event).kind == SELECTION_CLEAR_EVENT as u16`）——端口把 C 的 `kind : 16`/`part : 16` 放在一个 `bits` 单元里（PORTING.md 1.3），活臂用的是 `input_event_bits! (kind, …)`；三处都改成它，两处 `have_haiku` 调用按同样办法声明 `haiku_handle_selection_clear`（haikuterm.h 370）。
  - 证据：两件 `--check` **OK**（修法保住 C 的 token：`kind`/`ie`/`SELECTION_CLEAR_EVENT` 都在）、`build-tu src/alloc.c` 与 `src/keyboard.c` **built**；`dead-arms` 现在越过这两件、报在 `src/font.rs`（font lane 在飞的那一件）。
  ② **容差行按实测更新（`alloc` 0.08 → 0.10）**：`docs/ports/line-drift-tolerance.tsv` 的 `alloc` 行原本记 C 7639 vs 8169（6.9%），我那两个臂的修法把它推到 **8258 行（8.1%）**，于是 `--check` 只剩那一条漂移行 ⇒ 按 3.220 的规则（实测 + 0.01）改写该行并把新数字与原因写进去。
  ③ **更正（我自己的错读数）**：`3a819d0` 的提交信息里写了 `--check emacs/src/alloc.c …` → OK，**那是错的**：当时它报的正是上面那条 8.1% 漂移行（我的编辑把它推过了 0.08 的容差）。修完容差行之后这两件才是真的 OK（本轮实测）。这条按项目惯例留在这里更正，不重写历史。

- **3.222 静态门全跑了一遍（结果分派到 owner）；`dead-arms` 又推过一件；五条引证按实测更正（2026-10-04）**：
  ① **静态门读数**：`check-globals` 283 units / 0、`check-declarations` 13/0、`check-mirror-dupes` 1 duplicate / 1 ratchet / 0 failures、`layout-check passed`、`extract-abi --all` **586 done / 4 in progress / 0 failing** ✅。
  ② **三类非绿，全部追到 owner**：`check-citations` 由 14 drift/3 range/35 allowed 变成 **10 drift / 0 range / 37 allowed / 1 stale**——我改了 5 处（三处就地改数：`coding.h 800→689` 的 `code_convert_string`、`termopts.h/termchar.h 425→dispextern.h 4012` 的 `is_tty_frame`（dispnew.c 3490 定义）、`dispextern.h 3410-3430→3407`；`nsimage.swift` 把 `make_float` 的声明从 `floatfns.c 4652` 改成实测的 `lisp.h 4652`；两条"引证正确但门只能猜片段"的按规矩进 allowlist，带理由）；**剩下的 10 drift 与 1 stale 全在 lane 在飞文件里**（Harvey 的 `dispextern.rs`、Mill 的 `macfont/nsfont.swift`、Boyle 的 `nsmenu.swift`）。`check-undef` 的 **12 条 NEW 全是 `referenced by xdisp.o`**（xdisp 单元后段才定义的函数，在飞期的预期状态，闭合时重取棘轮）。`check-swift-units` 的 **140 new** 全在 Swin lane 的在飞文件里（func 落地但 manifest 行还没写）。
  ③ **`dead-arms` 又推过一件（`keyboard.rs`）**：三处翻转臂缺陷 + 我自己引入的一处**活代码回归**。
  - `have_haiku` 臂调 `haiku_handle_selection_clear` 没声明（C 从 haikuterm.h 370 拿）⇒ 在臂里声明，且调用改 `&raw mut copy`（C 的 `&copy` 在非 const 局部上就是可变指针）；
  - 同一臂把 C 的 `event->ie.kind` 当 Rust 字段读（端口把 `kind : 16`/`part : 16` 放在一个 `bits` 单元里）⇒ 改用 `input_event_bits! (kind, &raw mut (*event).ie)`；
  - `kbd_buffer[0] = kbd_buffer[KBD_BUFFER_SIZE - 1]` 要求元素 `Copy`，而 C 的 `union buffered_input_event` 是可平凡复制的、端口那份镜像漏了 derive ⇒ `include/keyboard.rs` 补 `#[derive(Clone, Copy)]`（树里其它 C union 镜像的形状）。
  - **回归与教训**：C 的 `event->kind`（`union buffered_input_event *` 上）读的是**联合体自己的 `kind` 成员**（端口镜像成 `u16`），不是 `ie` 的位域——我把两处**活**站点改成了位域宏，`build-tu src/keyboard.c` 立刻报 `no field bits on type buffered_input_event`。两处已还原成 C 的写法（`(*event).kind == … as u16`）。**教训**：翻转门只看死臂，活代码要另有一个门兜住——这次是 `build-tu`。
  ④ 证据：`--check keyboard.c` **OK**；`build-tu src/{keyboard,alloc,lread,cm,term}.c` **五件全 built**；`dead-arms` 现在越过 `alloc.rs` 与 `keyboard.rs`、报在 **`src/lread.rs`**（下一件，仍是同类小修）。

- **3.223 `xtask dead-arms` 转绿：337 个单元在非冻结臂下全部编过（2026-10-04）**：
  ① **这是全会话里被推过的最长一条门队列**：`dead-arms` 把每个已完成单元**开着 C 的死臂**编一遍，本轮之前它一直红（3.2/3.9 记的"死臂没有门在编"那个缺口）。本轮走完的：`alloc.rs`、`keyboard.rs`（前两个提交）→ **`lread.rs`** → **`print.rs`**，外加一处只有翻转才看得见的**端口自身缺口**：
  - **`lread.rs` 要 `NATIVE_ELISP_SUFFIX`**，C 是从 `config.h` 拿的（`configure.ac` 5289 的 `AC_DEFINE_UNQUOTED`，冻结 `build/emacs-ref/src/config.h` 2150）。这是**配置值**，所以按端口的规矩走"curation"这一步：值进 `docs/baseline/config/darwin-arm64.json` 的 31 条 curated 宏，`gen-config` 重新生成 `config.rs`（新增恰好一行：`pub const NATIVE_ELISP_SUFFIX: &core::ffi::CStr = c".eln";`）。
  - **`print.rs` 要 `XNATIVE_COMP_UNIT`**，而这一条是**端口自己的缺口**：C 在 `comp.h` 55-67 定义它（在 C 52 起的 `#ifdef HAVE_NATIVE_COMP` 内），所以**镜像 owns 它**——它此前是 `pdumper.rs` 的本地副本（3.153 那次只把**记录**搬进了镜像）。现在 `NATIVE_COMP_UNITP` 与 `XNATIVE_COMP_UNIT` 都在 `include/comp.rs` 的 feature 下，**按 C 的路径到达每个单元**（`lisp.h` 2168 的 `#include "comp.h"`，由 `include/lisp.rs` 镜像），`pdumper.rs` 的副本退休。
  - **`include/lisp.rs` 里 comp.rs 的 include 必须挪到 `eassert!`/`XUNTAG!` 之后**：`macro_rules!` 是文本作用域，而 C 的 lisp.h 在 181-189 定义 `eassert`、750 定义 `XUNTAG`，都在它 2168 的 `#include "comp.h"` **之前**；放在前面时翻转构建报 `cannot find macro eassert`（实测在 `alloc.rs`）。现在 include 落在端口第 508 行，注释里写着 C 的顺序。
  ② **一行位移的连带处理**：这次挪动让它下面所有内容移动一行，于是四条 allowlist 行（`lisp.rs:2748/3837/3966/4094`）失效——按项目既有做法把 `docs/baseline/citations-allowlist.tsv` 的 key 整体 +5 重锚，`check-citations` 回到 **10 drift / 0 range / 37 allowed / 1 stale**（剩下的都在 lane 在飞文件里）。
  ③ **证据**：`dead-arms` → **`337 translation unit(s) compiled with the non-frozen arms`（绿）**；`--check` 对 `src/{alloc,print,pdumper,lread}.c` 全 **OK**；`build-tu src/{alloc,lread,pdumper,print}.c` 全 built；`layout-check passed`；`check-mirror-dupes` 1/1/0；`extract-abi --all` **586 done / 4 in progress / 0 failing**；`gen-port-inventory` **590 files、done=580、wip=10、todo=0**。

- **3.224 三件 lane 交付入库；两条共享行；`font.h` 六个记录的实测数字与裁决（2026-10-04）**：
  ① **`dispextern.h` 的自由切片落地（142 → 104 行）**：29 项由镜像 own 且不与任何 unit 撞（`xstrcasecmp` 落在**不 glob-import 的模块** `dispextern_externs`——`src/font.rs` 561 也定义同名函数且 include 本镜像，这是 `bignum.rs`/`cm.rs` 的既成形；另有 `IF_DEBUG`、`TEXT_POS_EQUAL_P`、三个 `DESIRED_*_LINE_HEIGHT`、`VCENTER_BASELINE_OFFSET`、`FONT_TOO_HIGH`、`FACE_UNIBYTE_P`、`FACE_EXTENSIBLE_P`、`MAX_FRINGE_BITMAPS`、`ITERATOR_AT_END_P`、`DEFAULT_TAB_BAR_*`/`DEFAULT_TOOL_BAR_*`/`TOOL_BAR_IMAGE_*` 各族）。剩下的 104 行 = 103 个名字（**102 个由 unit 声明**，落进镜像就是那些 unit 的 `E0428`）+ 1 条 token 行；逐名 owner 表在台账 `## Evidence`。
  ② **新增两条共享行**（lane 报、我写）：`NativeRectangle` 进 `excluded-macros.txt`（kind=config：取值归窗口系统，五个平台镜像各自持 `macro_rules!`，同 crate 第二份会撞宏命名空间，`pub type` 会替所有单元钉死本 target 的 `int`）；`IF_DEBUG` 的 `0` 进 `macro-body-notes.tsv`（`GLYPH_DEBUG` undef ⇒ 体是 `((void) 0)`，端口 no-op 没有可点名的 0）。
  ③ **一条门行为（普遍规则，值得记）**：`extract-abi.py --check` 会把**台账还有未勾框**的文件的缺失项**降级成 stderr 的 `deferred:` 提示**——所以"先落被延期的项、再勾框"是硬顺序，否则一勾框那些项立刻变硬失败。
  ④ **`nsmenu` 入库（Swift）**：`nsmenu.swift`（90,744 字节）+ `docs/ports/nsmenu.md`；台账记着决定它形态的事实——**`src/nsmenu.m` 不在冻结配置的编译清单里**（`docs/baseline/frozen-build-sources.txt`），所以端口是"承载该 target 的文本"而不是提供可编译单元。`swiftc -parse-as-library -c` exit 0；`check-swift-units` 对它 0 行（该门报的 **140 条 new 全在 `nsterm.swift`**，即 nsterm lane 的在飞剩余量）。
  ⑤ **`font.h` 的裁决与实测数字**：lane 发现"6 个类型被延期"（正是 ③ 的机制），并把探针卡在 flags 上；我把探针跑通并把数字交给它（`clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src -I emacs/src -I emacs/lib`，探针 `/tmp/font_records_probe.c`）：`font_spec` **112**/align 8（header@0、props@8，`FONT_SPEC_MAX` 13）、`font_entity` **120**/8（props@8，`FONT_ENTITY_MAX` 14）、`font_metrics` **10**/2（lbearing@0、rbearing@2、width@4、ascent@6、descent@8）、`font_bitmap` **40**/8（bpp@0、rows@4、width@8、pitch@12、buffer@16、left@24、top@28、advance@32）、`font_driver_list` **24**/8（on@0、driver@8、next@16）、`font_driver` **208**/8（type@0、get_cache@16、list@24、match@32、close_font@64、text_extents@104、otf_capability@120、shape@152、check@160、cached_font_ok@184、combining_capability@192；它的 `draw`/`get_bitmap`/`free_bitmap` 在 `#ifdef HAVE_WINDOW_SYSTEM` 里、本 target 不存在）。**裁决**：按"记录先落（带实测断言）、函数后落、最后一次性勾框"推进，`font_driver` 单独一段。`font_driver` 的 208/`close_font`@64 与树里 `alloc.rs` 的局部探针一致，可互证。
  ⑥ 读数：`--check dispextern.h` 104 行、`--check font.h` 16 行（台账 39/55）；`build-tu src/{font,bidi}.c` built；`layout-check passed`；`check-mirror-dupes` 1/1/0；`dead-arms` **337 units 全绿**（3.223）；`extract-abi --all` 586 done / 4 in progress / 0 failing。

- **3.226 产品名声明这条长期指令的当轮复算，门的覆盖推到台账；`gate-fast` 第三次重跑的唯一红行（undef）按棘轮结清（2026-10-04）**：
  ① **指令的读法不变**：当轮两句话（`版本声明不要再说是gnu emacs了，我们这样说可能会有法律风险` + 38 秒后的
  `把之前的代码中的这种声明也去掉`）是**同一条指令的前后两半**（3.134/3.186 的读法）：不只对新写的文件生效，
  **已写进树的代码**也要清。三段处置一处没退：`2ee1d1d`/`4810bb0` 把 231 个首行的版本括注压到只留修订号、
  `fd9cf89` 不携带 268 个版权/许可块、标题注释与"复述许可块"的句子进 `title`/`recital` 两类。
  ② **当轮实测**（HEAD `2fd724e` 的工作树，含 3.186 之后落的单元与在飞文件：`src/xdisp.rs` 与
  `platform/macos/{nsfns,nsfont,nsterm,macfont,nsmenu}.swift`）：`python3 tools/check-declarations.py` →
  **`13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`**；定向扫
  （`part of GNU|is free software|Free Software Foundation|Copyright \(C\)|under the terms of the GNU|GNU Emacs [0-9]|@ <rev> \(`）
  在十棵被移植树上**只命中 13 处**，逐处都在 `docs/baseline/declarations-allowlist.tsv` 上
  （`src/emacs.rs` 5、`src/callproc.rs` 3、`lib-src/etags.rs` 3、`lib-src/ebrowse.rs` 2）——**3.186 之后落的单元零命中**；
  `docs/ports/*.md` 的 **600 份台账零命中**；catch-all 扫 `*.toml`/`*.sh`/`*.json`/`Makefile`（扣掉六棵逐字树与参照）
  **零命中**（只剩 `gnu_linux`/`gnulib_*` 这类 cfg 名，是标识符不是声明）。
  ③ **门的覆盖推到台账（本轮新绑）**：`docs/ports/*.md` 进 `tools/check-declarations.py` 的扫描面，判
  **`decl`/`version`/`title` 三类**，**不判 `recital`**——台账正是 3.90/3.100 指定的"许可注解的家"。理由是台账最像
  3.100 的 `lib-src/hexl.rs`：它按定义引 C 的原文，能把参照的开场块或去名前的版本行带回来，而任何被移植单元都不携带它。
  **覆盖不是空跑**：临时放 `docs/ports/__probe.md`（三段各一行）→ 门报
  `16 declaration sites (3 forbidden, 13 allowed literals), 3 failures`（`decl x1`/`title x1`/`version x1`），
  删掉后回到 `13 sites / 0 failures`；`--selftest` **22 → 28 例**（新增 6 例走 Markdown 形状：引声明块 `decl+title`、
  引去名前的版本行 `version`、引 C 的 docstring `title`、`(gnulib)` 出处记不算、台账自己的许可注解不算、只写修订号不算）。
  ④ **边界规则（写在这里，后续口径照此）**：**我们自己写的散文可以"提到"参照**（`docs/*.md` 的基线行、
  "femacs 一比一复刻参照检出 `fd7afced403`"、根 `README.md` 的许可句——它们说明我们在移植什么，是 3.90/3.100 允许的
  决定记录），**但任何文件都不许"声明自己是该产品"**——四个类就是这条边界在被移植树上的判据。边界外只剩四处，
  各有"为什么必须在这儿"的理由：门自己的模式表（`tools/check-declarations.py`）、允许清单的行（它们就是记账）、
  `docs/baseline` 的**实测值**（`PACKAGE_STRING`/`COPYRIGHT` 是 clang 冻结值，改了与参照不符）、
  `docs/status.md` 与 `docs/reviews` 的决策叙述。
  ⑤ **残下的 13 处为什么不动——本轮拿到新的实测理由**：运行时那行 `GNU Emacs 32.0.50 (build …)` 是**逐字树**里的
  `femacs/lisp/version.el:111` 拼出来的（`(format "GNU Emacs %s (build %s, %s%s%s%s)%s" …)`），逐字树按用户指令
  一个字不改、`check-verbatim` 钉着它 ⇒ **改 C 侧那几条字面量不会让程序少说一次产品名**，只会同时破坏 1:1 与 docstring 门
  （其中 4 条是 `callproc.c`/`emacs.c` 自己的字符串，门要与参照逐字比对）。所以这 13 条继续以"被移植语义 + 逐条理由"
  留在允许清单上；**要不要在产品身份层面改口径是全面验收期的判决**（用户长期指令：前期复刻期间与 C 尽量一致，
  全面验收时再根据需要判断是否调整）。
  ⑥ **`gate-fast` 第三次重跑：唯一的红行是 undef，按棘轮结清**（原输出 `/tmp/gf3.log`，5.7 MB）：**674 个 translation
  unit 编译完成**、inventory 与 globals 两段过、`check-symbols: 86 units compared, 0 mismatched, 1 still pending`；
  红在 `check-undef`——`8 ratchet entries are stale (now satisfied)` + `12 NEW undefined reference(s)`，12 条**全部
  `referenced by xdisp.o`**。读清单后处置：**8 条 satisfied 是已落地的欠账**（`_valid_font_driver` 随 3.225 的 font 镜像
  定义，`_die`/`_suppress_checking`/`_in_display_vector_p`/`_partial_line_height`/`___gl_dynarray_resize`/
  `_Vlast_prefix_arg`/`_Vprefix_arg` 随各自单元落地）；**12 条 NEW 全是 xdisp 自己的后段定义**（实测行号：
  `redisplay_internal` 17351、`try_window` 21684、`restore_selected_window` 14521、`restore_frame_selected_window` 14563、
  `Vmessages_buffer_name` 38602、`inhibit_message` 38430、`Vmax_mini_window_height` 39042、`Vresize_mini_windows` 39050、
  `message_truncate_lines` 39113、`Vdebug_on_message` 39406、`Vset_message_function` 39410、
  `redisplay_adhoc_scroll_in_resize_mini_windows` 39497）——即 lane 的**未到达区间**，不是拼写或发明名字。处置 =
  **让棘轮记账，而不是把它静音**：`cp` 备份 → `python3 tools/check-undef.py --write-pending` → `diff` 恰好
  **8 删 / 12 增**（111 → 115 行，别无其他）→ 复跑 **`check-undef: 695 unit object(s), 161 reference object(s)/archive(s):
  110 pending, 0 new`（exit 0）**。**附条件**：这 12 条在 xdisp 落进 C 13995+ 与 `syms_of_xdisp` 之后会变 stale，
  那时必须从棘轮上删掉。
  ⑦ **一条门的事实（免得再误记）**：`gate-fast` 的段落是 inventory / globals / 编译（两种配置）/ 符号 / undef，
  **不含 `layout-check`**；`layout-check` 本轮单独跑过 → **`layout-check passed`**
  （`CARGO_TARGET_DIR=/tmp/femacs-lc-226`，用完即删）。
  ⑧ **两条 lane 交付本轮收下，下轮入库存档**：nsterm lane 第 1 轮（`nsterm.swift` 404 行 + `docs/ports/nsterm.md` 739 行，
  它的发现入册：**`swiftc -parse` 不能当 Swift lane 的编译证据**——同一份文本 `-parse` 无输出、`-c` 报 12 个 error，
  `-c` 才是闸门）；nsfns lane（`nsfns.swift` + `docs/ports/nsfns.md` 383 行 + 21 条 `swift-units.tsv` 行，
  manifest 由 lane 按 writer 的形状写入并 diff 验证）。入库时要补的是 `nsterm.swift` 那 **140 条 manifest 行**
  （`--write-manifest` 会同时加它自己那 140 条与 nsfns 的 21 条，只取 nsterm 的部分）。

- **3.227 两条 Swift lane 入库（`nsfns` 与 `nsterm` 第 1 轮），`keymap.rs` 的 `parse_solitary_modifier` 声明修正，以及"引文正文从来没人读"这条实测缺口（2026-10-04）**：
  ① **`nsfns` 入库**（`bcd3e83`）：`femacs/platform/macos/nsfns.swift`（1,482 行）+ `docs/ports/nsfns.md`（383 行）
  + 21 条 `swift-units.tsv` 行。判据：`swiftc -parse-as-library -c -o /tmp/s226/nsfns.o` **exit 0 无诊断**；
  `check-swift-units` 对本文件 **56 C definition(s) / 21 unported / 21 row(s)；216 C symbol import(s)**。
  清单**不是** `--write-manifest` 刷出来的：生成器会同时删 `macfont.swift` 的 3 行（Mill 在飞的切片已在工作树里落成那三个函数，
  但没进仓库）并加 `nsterm.swift` 的 140 行，直接再生就会把本提交不携带的文件写成事实——所以按"只让该消失的行消失"的纪律
  手插 21 条（与生成器对该文件的输出逐字节一致，顺序仍是文件自己的 (path, name)）。唯一带 reason 的行是
  `interpret_services_menu`（`#ifndef NS_IMPL_COCOA` 文本，本 target 定义了 `NS_IMPL_COCOA` ⇒ 永久死文本）。
  ② **`nsterm` 第 1 轮入库**：`nsterm.swift`（404 行，C 60-390 的切片）+ `docs/ports/nsterm.md`（738 行）+ 140 条 manifest 行
  （142 定义 − 2 已落）。**lane 的发现入册**：上一轮那份文本用 `swiftc -parse` 看是"绿"的，
  `swiftc -parse-as-library -c` 却报 **12 个 error** ⇒ **`-parse` 不做名字解析，不能当 Swift lane 的编译证据，`-c` 才是闸门**
  （与 `nsselect` 先例一致）。其余按 C 行号记：`#if NSTRACE_ENABLED`（C 96-149，`NSTRACE_ENABLED`=0）整块死文本；
  GNUstep 的 include/import 序言被 `import Foundation/AppKit` 取代；`dockMenu`/`mainMenu` 跨单元（`nsmenu.swift`）走 P-55 的
  `UnsafeMutableRawPointer?` + 接收者在前的 C-ABI 条目；`NSAutoreleasePool` 在 ARC 下不可用 ⇒ 保存/恢复用
  `objc_autoreleasePoolPush/Pop`；26 个 `static` → `fileprivate`。**欠账（记在这里，别让它消失）**：
  `ns_use_srgb_colorspace` 这个数据符号**核心侧还没有导出**（`rg` 只有 Swift 侧的两处导入），它在 `syms_of_nsterm` 的
  `DEFVAR_BOOL`（C 11716）那一段落地时补。
  ③ **`femacs/src/keymap.rs` 的一处真缺陷（由 lane 的报告带出来的）**：`/// C: keyboard.h 465: … parse_solitary_modifier … -> Lisp_Object`
  —— 实测 C 是 keyboard.h **463** 的 `extern int parse_solitary_modifier (Lisp_Object symbol);`、定义在 keyboard.c 7920 也是 `int`，
  而调用点 `keymap.rs:2609` 用 `as isize as c_int` 把这个错类型糊过去（"名字/类型错了要等链接才现形"，正是 `check-undef.py` 的存在理由）。
  同块另外三条引证也错：`parse_modifiers` 463→**461**、`lucid_event_type_list_p` 417→**497**、`menu_item_eval_property` 440→**516**，
  而 `message_with_string` **不在 keyboard.h**（在 `lisp.h` 4397）。全部就地改正（**文件行数不变**：改前改后都是 3,456 行，
  所以 `citations-allowlist.tsv` 一个 key 都没移动），`build-tu src/keymap.c` → **built**；台账 `docs/ports/keymap.md` 记了决定。
  ④ **新测出来的系统性缺口：`/// C:` 引证的"引文正文"没有任何门在读**。我（集成者）写了个一次性审计：抽 `femacs/src/**/*.rs` 里
  **单行**且带反引号引文的 `/// C: <header> <line>: \`…\`` 引证，与参照该行逐条比对，读数
  **checked 618 / mismatched 215 / ranges 185 / unresolved 4**（抽查确认是真错：`buffer.rs:105` 引 `lisp.h 4420`，而
  `extern void mark_object (Lisp_Object);` 在 **4436**；`alloc.rs:3472` 引 `emacs.c 818`，那一行是 `{`，`emacs_pipe` 不在 emacs.c 里）。
  `tools/check-citations.py` 只校验**类型名类**引证（`\`struct glyph *\` is at dispextern.h:754`），所以这批从没被读到——
  而引证是这套复刻的**可追溯性本体**（"每个数字都要有一条能重跑的命令撑着"）。**已派 lane（Feynman）**：写 `tools/check-citation-text.py`
  把这条量化和可复跑，逐条就地改正（行数不变、排除在飞文件 `xdisp.rs`/`font.rs`/`include/font.rs`/`include/dispextern.rs`/`platform/**`/`keymap.rs`
  与全部共享表），带回逐条 old→new 表与未动清单；**树级修正由它分级推进，不要静默批量改**。
  ⑤ **读数（本轮末，工作树）**：`check-swift-units` **0 new / 3 stale / 7 Swift unit(s) landed**（3 条 stale 是 `macfont.swift` 的
  `mac_font_create_line_with_string_and_font`/`mac_font_shape`/`macfont_shape`——Mill 的 ③b 在工作树里落了这三个函数但没入库，
  **那一段入库时这 3 行必须从 manifest 上删掉**）；`check-declarations` 13/0（`--selftest` 28/0）；`check-verbatim` 6 trees / 4607 files / 0；
  `check-citations` 628 files / 10 drift / 0 range / 1 stale（stale 行是 `femacs/src/include/dispextern.rs:539`，dispextern lane 的在飞件）；
  `layout-check passed`。
  ⑥ **生成物的一条纪律（本轮实测后没有提交）**：`gen-port-map.py`/`gen-port-inventory.py` 现在跑出来的读数**描述的是工作树**，
  其中 `macfont.swift 11,069`（Mill 在飞）、`alloc.rs 8,268`（历史漂移）、`font.rs`/`include/font.rs`（Ohm 在飞）都不是本提交携带的状态 ⇒
  我按 HEAD 还原了这两个文件，**等这些 lane 入库后的那一轮再一起再生**（生成物不许超前于它所描述的文件）。

- **3.228 `dispextern.h` 的镜像状态复核、那条发现被位移的 allowlist key 重锚，以及 lane 报告的重复项甄别（2026-10-04）**：
  ① **lane 的 `142 → 104` 切片已经在 `4deb60f`（3.224）入库**，本轮我按"判据要在当前树上取"重新量了一遍，确认状态与台账一致：
  `--check emacs/src/dispextern.h femacs/src/include/dispextern.rs` 的 `- ` = **104（全部 `missing`，0 条 token 行）**
  ——对比入库前的 **142（140 missing + `GLYPH_CHAR`/`GLYPH_FACE` 两条 token 行）**。也就是说 lane 报告里那个 106 是它中途的读数，
  当前树上的权威数字是 104；`femacs/src/include/dispextern.rs` 与 `docs/ports/include-dispextern.md` 在工作树里**没有任何未提交改动**
  （`git diff` 为空，最后一次改动是 `4deb60f`）。
  ② **这 104 行是什么**：4 functions + 23 macros + 65 constants + 13 objects，103 个名字里 **102 个由 unit 声明**
  （逐名 owner 表在台账 `## Evidence`：xdisp.rs 63、dispnew.rs 30、indent.rs 25、window.rs 25、term.rs 25、keyboard.rs 21、
  xfaces.rs 16、frame.rs 5、bidi.rs 2、eval.rs 2、font.rs 1），**1 个 free：`NativeRectangle`**（共享行 3.224 已写）；
  抽取器仍 defer 的 5 个类型（`bidi_it`/`composition_it`/`draw_fringe_bitmap_params`/`it`/`redisplay_interface`）**也全是 unit-owned**
  （`bidi.rs:86`、`dispnew.rs:6828/6855/3569`、`indent.rs:180`、`minibuf.rs:666`）。它们与那 102 个名字是同一件事，归 **3.220 的裁决**
  （镜像那份挂 `dispextern_face_records` 之类的 flip feature，或落**不 glob-import 的模块**）：本单元唯一已落地的形态决定是
  `xstrcasecmp` 落在 `dispextern_externs`（`src/font.rs:561` 自己定义同名函数且 include 本镜像 ⇒ 顶层是 `E0428`，实测点名 `font.rs:561`；
  `bignum.rs`/`cm.rs` 的既成形）。**下一轮按同一张 owner 表推进这 102 + 5**。
  ③ **lane 请我加的两条共享行是重复项，本轮不追加**：`NativeRectangle` → `docs/ports/excluded-macros.txt`、`IF_DEBUG` 的 `0` →
  `docs/ports/macro-body-notes.tsv` 两条**在 3.224 就已经写好**（`4deb60f` 的提交信息里也点了这两条）；它的报告写在看到那一轮之前。
  同一个名字写两行会被门当成重复 key，所以这里只记"它要求的两条已经在了"，不重复写。
  ④ **本轮真正的改动是那条被位移的引证 key**：这次切片让它下方内容移动 5 行，`docs/baseline/citations-allowlist.tsv` 里唯一一条
  `femacs/src/include/dispextern.rs:539`（理由：`dispextern.h 648-661` 是对的、`struct glyph *` 是参数类型）失效 ⇒ 重锚到 **:544**
  （引证本体在文件里就是从 539 挪到 544 的那一条）。复跑：`check-citations` → **628 files / 9 drift / 0 range / 284 external / 38 allowed / 0 stale**
  （drift 10→9、stale 1→0 一起动，因为那条 finding 现在被 allowlist 正确覆盖）。
  ⑤ **lane 报的"`src/keyboard.c` 现在红"我复测为不成立**：`build-tu src/keyboard.c` → **built**（exit 0）。它当时看到的
  `E0609 no field bits on type buffered_input_event` 是 `build/` 里过期对象/前一轮镜像态造成的；**判据要在当前产物上取**。
  ⑥ 读数（本轮末）：`extract-abi --all` → **checked 590 ported files: 587 done, 3 in progress, 0 failing**（3 个 in progress = 台账未勾满的
  `include-dispextern` 53、`include-font` 16、`xdisp` 157）；`build-tu src/{font,dispnew,xfaces,bidi,character}.c` **五件全 built**；
  `layout-check passed`；`check-mirror-dupes` 1 duplicate / 1 ratchet row / 0；`check-globals` 283 units / 0；`check-declarations` 13/0；
  `check-target-names` 626 .rs / 591 targets / 0。

- **3.229 `macfont` ③b 入库、`nsfont` 补入库，以及"生成文件在 `git add` 与 `commit` 之间被改掉"这条竞态（2026-10-04）**：
  ① **`macfont` ③b 入库（`061fac1`）**：`macfont.swift` 10,596 → **11,069** 行（C 3094-3241、3775-3836、3839-3988），
  函数 **64/74 → 67/74**；`swiftc -parse-as-library -c` exit 0 无诊断；`check-swift-units` 对本文件
  **74 C definition(s) / 7 unported / 7 row(s)；127 C symbol import(s)**。形态决定按 C 记：三处 `alloca` → 嵌套
  `withUnsafeTemporaryAllocation`（容量 `max(_,1)`，C 的 `alloca(0)` 不会给 NULL）；`RIGHT_TO_LEFT_P` 的 `permutation`
  路径是活代码（`xmalloc`/`xfree`），C 的 `permutation != nil` 判定全留；`mac_font_shape` 的 `dir` 参数在 C 里没人读 ⇒
  按声明保留、不引用。**台账按参照改正了三处**：`OTF_TAG_STR`（1875-1882）在 macfont.m 里**没有任何读者** ⇒ 不写（写了就是死代码），
  `OTF_SYM_TAG` 的范围是 1869-1873，另外四处是本 lane 上一轮自己写错的行号（`macfont_shape` 3094-3241、
  `mac_font_create_line_with_string_and_font` 3775-3836、`mac_font_shape` 3839-3988、第二处 `RIGHT_TO_LEFT_P` 3870）。
  21 条新 `@_silgen_name` 全都先 grep 过全树（`LGSTRING_*`/`LGLYPH_*` 在 `include/composite.rs` 已有整套 `pub fn`，
  `CHECK_FONT_GET_OBJECT` 在 `font.rs`/`include/font.rs`，`QL2R`/`QR2L` 在 `symbols.tsv`）⇒ **只写声明，没有第二份定义**。
  ② **竞态（本轮实测，值得记）**：`docs/baseline/swift-units.tsv` 我在 3.227 是按 "HEAD 的 12 行 + 21 nsfns + 140 nsterm = 173 行"
  写好的，`cp` 之后隔了十来分钟才 `git commit`；**这中间 lane 跑了一次 `--write-manifest`**（它把已落的 3 条 `macfont.swift` 行删掉、
  保留 140 条 nsterm 行），于是 `0389861` 里实际进去的是**那份 170 行的再生版本**。后果实测出来了：那个提交里
  `femacs/platform/macos/macfont.swift` 仍是 10,596 行的旧版（10 unported）而 manifest 只写 7 行 ⇒ **在干净检出上会报 3 条 missing**。
  本轮的 ① 把文件补上之后两边重新一致（7 unported / 7 row(s) / 0 new），`git diff` 为空。**纪律（写在这里）**：
  生成物在 `git add` 与 `git commit` 之间必须**重读一次**（`git diff --cached --stat` + 与它配套的门读数），
  不能拿"我写它的时候是对的"当判据——与 3.196/3.202 同一类"把片段当事实"。
  ③ **`nsfont` 补入库**：`femacs/platform/macos/nsfont.swift`（2,320 行）+ `docs/ports/nsfont.md`（455 行）。
  这是一个**孤儿交付**：它的 2 条 manifest 行在 `805f713`（3.220 那一轮）就进了仓库，而**文件从来没 add 过**——反过来的同一种不一致
  （干净检出上那 2 行会是 stale）。现在补上，`swiftc -parse-as-library -c` **exit 0**，门读数
  **`nsfont.swift: 36 C definition(s), 2 unported, 2 row(s); 111 C symbol import(s)`**，与它台账的 `Functions 36（34 已落、
  2 deferred）`、2 个未勾框逐条对上。它为什么长这样（台账首段）：**`src/nsfont.m` 是 GNUstep 的字体后端，冻结配置不编译它**
  （`docs/baseline/frozen-build-sources.txt`；`configure.ac` 2816 与 2762 让 `nsfont.o`/`macfont.o` 二选一），
  整文件没有预处理条件（唯一的 `#if 0` 在 659-662），所以端口承载**整份文本**而不是提供可编译单元；两个 deferred 体
  （`nsfont_draw` 1151、`ns_dump_glyphstring` 1711）都需要 `struct glyph_string` 的字段，而本树只把它声明成不透明占位
  （`femacs/src/font.rs` 739，P-70）。
  ④ 读数（本轮末）：`check-swift-units` **0 new / 4 stale / 7 Swift unit(s) landed**（4 条 stale 全在 `nsterm.swift`：
  `append2`/`hold_event`/`ns_finish_events`/`ns_init_events`——Poincare 第 2 轮已在工作树里落了这几个，
  **那 4 行在它入库时删掉**）；`check-declarations` 13/0；`check-verbatim` 6 trees/4607 files/0；
  `check-citations` 9 drift / 0 range / 38 allowed / 0 stale；`layout-check passed`；`extract-abi --all` 587 done / 3 in progress / 0 failing。

- **3.230 `src/frame.h` 收口：三个访问器落进 `include/frame.rs` 的模块臂，`--check frame.h` 转 OK（2026-10-04）**：
  ① **实产**：`FRAME_FACE_CACHE`（C 1379 的 `#define`）、`FACE_FROM_ID`（C 1646-1650 的 `INLINE`）、
  `FACE_FROM_ID_OR_NULL`（C 1656-1661）三个名字落进 `femacs/src/include/frame.rs` 的
  **`#[cfg (feature = "dispextern_face_records")] pub mod frame_face_accessors`** —— 3.211 的裁决说"等共享镜像先承载
  `struct face`/`struct face_cache`"，3.220 把它们落在 `include/dispextern.rs` 的同一 feature 下，本轮据此收口。
  **判据**：`extract-abi.py --check emacs/src/frame.h femacs/src/include/frame.rs` → **`OK`**（此前 3 条 `missing`：
  两个 function + 一个 macro；中途还抓到一条 token 行——C 1659 的 `eassume (0 <= used);` 必须照写，写成 `eassume!` 后才过）。
  ② **为什么是"模块 + feature"而不是写在镜像顶层**：37 个单元 `include!` 这份镜像（实测：这 37 个**全部**也 include
  `include/dispextern.rs`，所以模块依赖 `face`/`face_cache` 是安全的），而 INLINE/`#define` 的活形态在端口里是"每个 include 者一份"
  （`macro_rules! FRAME_FACE_CACHE` 在 alloc.rs 3467 / font.rs 545 / xfaces.rs 445 / xdisp.rs 1101 / window.rs 920，
  `fn` 形态在 term.rs 364-377）⇒ 顶层副本是那些单元的 `E0428`（本轮复测过）。放进模块后两边并存：
  `tools/check-mirror-dupes.py` 的规则是"**带体的声明是定义、从不作为重复报**"，而"镜像不 glob-import 的模块不在单元作用域里"
  ——复跑仍是 **1 duplicate / 1 ratchet row / 0 failure**，没有新增行。
  ③ **编译证据（普通臂）**：`build-tu src/{frame,term,alloc,xfaces,category,cm,data}.c` **七件全 built**；
  `layout-check passed`；`check-declarations` 13/0；`check-citations` 9 drift / 1 range / 38 allowed / 0 stale
  （那条 range 是 `femacs/platform/macos/macfont.swift:11287` 引 `src/macfont.m 4248-4260` 而参照只有 4,257 行，是 Mill 在飞的新文本，
  下一轮找它改）。
  ④ **翻转臂的实测状态（写清楚，别让它变成假声明）**：`rustc --cfg 'feature="dispextern_face_records"' build/femacs/units/<stem>/_root.rs`
  → **frame / category / cm / data / doc / eval / ccl 全 exit 0**（我的新模块在这些单元里编得过，这是本轮真正的验证），
  而 **term / alloc / xfaces 仍是 `E0428: face/face_cache defined multiple times`**——那是 3.220 就存在的欠账（这些单元自带
  无条件的带类型副本），不是本轮的回归。**收口路径已写进 `docs/ports/frame.md` 的决定 7 与 `include/frame.rs` 的注释**：
  每个自带副本加一行 `#[cfg (not (feature = "dispextern_face_records"))]`，等 `xdisp.rs`/`font.rs` 两个在飞单元入库后一次做完，
  那时翻转臂才能真正全绿。另外核了一遍：`femacs/xtask/Cargo.toml:61` 有这条 feature 行，但 `dead-arms` 不 flip 它
  （它 flip 的是配置侧的臂）⇒ 这条 feature 目前**没有门在编**，这才是上面那条欠账一直没暴露的原因。

- **3.231 `macfont` ④ 收口入库、`nsterm` 第 2 轮入库，以及两条裁决（`ns_relocate` 的自包含臂、Swift 导入不进 Rust 棘轮）（2026-10-04）**：
  ① **`macfont` ④ 收口（`a40bce4`）**：`macfont.swift` 11,069 → **11,313** 行，`swiftc -parse-as-library -c` exit 0；
  门读数 **`74 C definition(s), 3 unported, 3 row(s); 131 C symbol import(s)`**，`0 new / 0 stale` —— 剩下 3 条正是**永久死臂**
  （`mac_ctfont_get_glyph_for_cid` 在未定义的 `USE_CT_GLYPH_INFO` 下 + 两个 GNUstep-only 体），不是欠账。
  **3.220 那条附条件验收成立**：`nm -u` 里 **`_macfont_driver` 消失**（同文件 `@_silgen_name` 自引用由定义落地解析），
  留下的导入都是核心提供的（`_VECSIZE`/`_font_build_object`/`_QC*`）；`nm -g` 的导出是 `_macfont_driver`、
  `_mac_register_font_driver`、`_macfont_get_nsctfont`、`_macfont_update_antialias_threshold`、`_syms_of_macfont`。
  ② **`nsterm` 第 2 轮入库**：`nsterm.swift` 404 → **613 行**（C 391-521：`EV_*` 宏块、`hold_event`/`append2`、
  `ns_init_events`/`ns_finish_events` 等），`docs/ports/nsterm.md` 846 行；`swiftc -parse-as-library -c` **exit 0**；
  门读数 **`142 C definition(s), 136 unported, 136 row(s); 15 C symbol import(s)`**。台账里两条**有意留空的盒子**记着原因：
  `EV_TRAILER`/`EV_TRAILER2` 的体调用本文件自己的 `hold_event`(C 498) 与 `ns_send_appdefined`(C 5013)，Swift 没有前向声明、
  先 `@_silgen_name` 后定义同名函数是 `invalid redeclaration` ⇒ 按 C 的顺序挂到 C 5013（首次展开在 C 6412，之前无使用者）。
  形态决定按 C 记：`EV_UDMODIFIERS` 保留十项 OR（不折成集合判断）、`EV_MODIFIERS2` 的隐式窄化一律 `truncatingIfNeeded`、
  `struct input_event` 的 Swift 视图按 `include/termhooks.rs` 的字段名 + clang 实测布局（sizeof 64、逐字段偏移）。
  ③ **裁决：`ns_relocate`（C 522）取 `NS_SELF_CONTAINED` 臂为活**。实测依据：`configure.ac` **689-690 的默认值是 yes**
  （`EN_NS_SELF_CONTAINED=$enableval` / 缺省 `yes`），2885-2886 据此 `AC_DEFINE([NS_SELF_CONTAINED])` ⇒ 自包含臂是缺省构建的活文本，
  `--without-ns-self-contained` 那一支按 P-51 记死文本（行号进台账）；`nsterm.m` 本来就在 `check-cond-eval` 的 NOT VERIFIED 类
  （盲读，P-51/P-66），所以这条由 configure.ac + 行号记录定，不靠 evaluator。
  ④ **裁决：Swift 侧的 `@_silgen_name` 导入进不了 `undef-pending.txt`**。lane 请求把 `_QCmouse`/`_up_modifier`/…/`_list1`
  这 16 个"核心还没导出 C ABI 入口"的导入记进 Rust 棘轮——**不加**：`check-undef.py` 的 universe 是 `build/femacs/**/*.o`
  里**被移植的 Rust 单元对象**，Swift 单元的对象不在其中，加进去就是一行永不匹配的 stale（门会红）。
  **同时把这条记成缺口**：Swift lane 的"核心必须导出 X"目前**没有任何门在核**（`check-swift-units.py` 只数导入条数），
  `ns_use_srgb_colorspace`（等 `syms_of_nsterm` 的 `DEFVAR_BOOL`）就是这类欠账的实例。下一轮把它做成一条能跑的门
  （例如从 Swift 的 `@_silgen_name`/`@_cdecl` 抽出符号集，与 Rust 单元的导出集求差）。
  ⑤ 读数（本轮末）：`check-swift-units` **0 new / 0 stale / 7 Swift unit(s) landed**（4 个文件的行数全部与树一致）；
  `check-declarations` 13/0；`check-verbatim` 6 trees/4607 files/0；`layout-check passed`；
  `extract-abi --all` 587 done / 3 in progress / 0 failing。

- **3.231 补记（同轮修）**：上一条 ② 说"`swift-units.tsv` 少了本轮的四条 nsterm 行"——**这条在 `d6e3d3f` 里没落地**：
  我算了 tsvA/tsvB 两个版本，却只把 tsvA 写回工作树（tsvB 只存在于内存里做断言），于是 `d6e3d3f` 只提了 3 个文件，
  仓库里那 4 行仍在（干净检出上会报 4 条 `stale`）。**本补记的提交把它修掉**：删掉
  `append2`/`hold_event`/`ns_finish_events`/`ns_init_events` 四行，`check-swift-units` 回到 **0 new / 0 stale / 7 units**。
  教训与 3.229 的竞态同源：**"我算出来的版本"不等于"工作树里的版本"**——凡是对生成文件做"减法"，写完必须
  在同一分钟里用门（或 `rg -c`）复核工作树，再 `git add`。

- **3.232 `xdisp.rs` 第 16 轮入库（15,185 行，前沿 C 13994），并把它欠的 9 条 token 行写进共享表（2026-10-04）**：
  ① **实产**：`femacs/src/xdisp.rs`（未跟踪 → 入库，**15,185 行**，`--check` 的 frontier 到 **C 13994
  `format_mode_line_unwind_data` 的闭括号**）+ `docs/ports/xdisp.md`（1,799 行）。本轮区间 C 13256-13994（739 行 C），
  新增 **21 个函数**（`resize_echo_area_exactly`/`resize_mini_window`/`current_message`/`push_message`/`restore_message`/
  `pop_message_unwind`/`check_message_stack`/`clear_message_stack`/`truncate_echo_area`/`clear_message`/`clear_garbaged_frames`/
  `echo_area_display`/…）、`MODE_LINE_NOPROP_LEN` 与 9 个文件级对象；SCAFFOLD 烧 4 加 1（15 → 12 条）。
  `build-tu src/xdisp.rs` → **built**（4 轮修错后）。台账勾选：Functions **169/302**、Macros 26/38、File-scope 58/60。
  ② **本单元欠的 9 条 `macro-body-notes.tsv` 行已写**（lane 逐条给了理由与 C 行号，我只做录入；
  两条 `SAVE_IT` 早就在表里）：`redisplay_trace`/`move_trace`（`va_list,va_start,va_end` → `VaList`）、
  `dsafe_calln`（`__VA_ARGS__` → `$($args:expr),*`）、`move_it_in_display_line_to`（六个 `goto` → labelled `break`；
  `UNINIT` → `0`）、`move_it_to`（`switch`/`case`/`default`/`goto out` → `match`/`break`）、`move_it_vertically_backward`
  （`move_further_back:` → labelled `loop`）、`add_to_log`（`va_*` → `VaList`）、`message_dolog`（`INT_STRLEN_BOUND` 实测 30、
  `PRIdMAX` 实测 `"jd"`）、`ensure_echo_area_buffers`（`INT_STRLEN_BOUND` 实测 27）。
  **判据**：`--check emacs/src/xdisp.c femacs/src/xdisp.rs` 的 `does not mention` 行 **6 → 0**，`- ` 行 176 → **170**
  （剩下全是 `missing …` 的未转写余量，含 7 subr + 13 defsubr + 123 function）。`extract-abi.py --selftest` 9 cases / 0。
  ③ **这条单元的在飞性质要写清**：它是**增量入库**（39,805 行的最大单元，本轮前沿 C 13994），所以 `--check` 仍是 FAIL，
  台账 157 个未勾框；`extract-abi --all` 因此把它算在 **3 个 in progress** 里（另两个是 `include-dispextern` 53、`include-font` 16）。
  另据 lane 实测：它引证的 `cond-eval-verified.txt` 文件数从 431 变 424 是别的 lane 改的，不是本单元。
  ④ 读数（本轮末）：`check-swift-units` 0 new / 0 stale / 7 units；`check-declarations` 13/0；`check-verbatim` 6 trees/4607 files/0；
  `check-citations` 9 drift / 1 range（macfont 在飞文本，已转给该 lane）/ 38 allowed / 0 stale；`layout-check passed`；
  `extract-abi --all` 587 done / 3 in progress / 0 failing。

- **3.233 引文正文的批量纠错：23 个文件、212 处 `/// C:` 行号按参照改正，并留下可复跑的审计脚本（2026-10-04）**：
  ① **问题**（3.227 量化）：`femacs/src` 下 618 条单行带引文引证里 215 条在所指行上找不到引文，`tools/check-citations.py`
  只校验"类型名类"引证，**引文正文从来没有任何门在读**——而引证是这套复刻的可追溯性本体。
  ② **本轮实产**（Feynman lane）：新增 `tools/check-citation-text.py`（可复跑审计：按 `emacs/<file>`→`emacs/src/`→
  `emacs/lib/`→`emacs/lib-src/` 解析、空白折叠、引文为参照行子串即命中；单行与区间引证分开统计；占位行号 `4xxx` 不参与匹配），
  并**逐条改正 23 个文件里的 212 处引证**——全部行内替换，`git diff --numstat` 合计 **+212 / −212**（逐文件增删相等，
  所以 `citations-allowlist.tsv` 的 key 一个都没移动）。读数：可动范围 `checked 265 → 467 / mismatched 226 → 24`；
  全量 `femacs/src`+`femacs/lib` `checked 417 → 619 / mismatched 235 → 33`（余额见④）。
  ③ **编译证据**：19 个受影响 `.c` TU 全部 `built`（lane 报告逐条贴了末行）；`check-citations` 对照会话开始
  **drift 10 → 9、stale 1 → 0、range 0 → 1**（range 那条是 `macfont.swift:11287` 引 `macfont.m 4248-4260` 越界，属平台 lane 在飞文本，已转给该 lane）。
  ④ **未动清单（有理由，不是遗漏）**：18 条**跨行原型/跨行定义**（行号已指向声明首行，正文是整段拼接；要彻底对齐得改写成区间引证，
  而区间引证不在本轮范围）；7 条**正文在参照树里不存在**（生成头 `globals.h`/`stdcountof.in.h`、`#ifdef HAVE_NATIVE_COMP` 里的同名声明、
  只以宏存在的 `waiting_for_input`）；1 条正文大小写不符（`enum Set_Internal_Bind`，与相邻行不一致，留备查）。
  **这 33 条 mismatched 是复核队列，不是补丁清单**。
  ⑤ 附带发现（已有解释，不是回归）：`build-tu src/{lisp,thread,indent,intervals}.h` 四个**头镜像** TU 报 `E0425`
  （`HAVE_PDUMPER`/`HAVE_NATIVE_COMP`）；HEAD 版本同样引用这两个名字 ⇒ 镜像本来不是独立 TU（与 handoff §五 的
  "`lib/include/*` 种子不从 unit 拿标量"同类），不由本轮引入。

- **3.234 新的门 `tools/check-swift-imports.py`：Swift 的 539 条 C-ABI 导入里有 427 条核心还没有导出（2026-10-04）**：
  ① **缺口的由来**（3.231 ④）：`check-swift-units.py` 只**数**Swift 单元的导入条数，不问有没有人提供；
  `check-undef.py` 的 universe 是**被移植的 Rust 单元对象**，Swift 的 `.o` 不在其中（所以 nsterm lane 请求把 16 条 Swift 导入写进那条棘轮被我否掉）。
  结果就是 `ns_use_srgb_colorspace` 这类"核心必须导出 X"的声明**没有任何门在核**。
  ② **本轮的测量**：新门扫 `femacs/platform/**/*.swift` 的 `@_silgen_name("X")`，与"核心的导出集"（`nm -g build/femacs/**/*.o`）
  加"Swift 自己 `@_cdecl` 的名字"求差，并把每条未提供的名字分类：
  `reference` 53（参照对象 define/reference 它）、`reference-source` 298（参照的 C 文本里就是它：头文件宏/INLINE/枚举，或 .c/.m 的 `DEFSYM`）、
  `system` 6（SDK 的 `.tbd` 提供）、`rust-side` 0、`unclassified` 70（需要人工裁定）。总读数：
  **539 import(s) / 7 Swift 文件 / 5525 core export(s) / 427 unprovided**。
  ③ **棘轮**：`docs/baseline/swift-import-pending.txt`（427 行，逐条带分类理由，由 `--write-pending` 生成）。
  门**绿**（0 failure），规则与其它棘轮一致：**新名字**（不在棘轮里的未提供项）FAIL；**棘轮里已消失的** STALE FAIL；
  已记账但分类不出来的 70 条按 **REVIEW** 列出（`--report`），不红——它们要么是"核心还欠一个只有 lane 知道的 C-ABI 名字"，
  要么是别的 Swift 单元（ObjC 类方法等）提供的符号，逐条裁定写进棘轮的 reason 列。
  ④ **这条读数的意义**：它是**链接阶段的义务清单**。Swift lane 的规则是"宏/INLINE/枚举/记录成员访问器由核心以 C 自己的名字导出成 C-ABI 条目"
  （status 3.144），而这 427 条里绝大多数是这类构造 ⇒ **现在没有任何 Rust 单元导出它们**，链接阶段一旦开工就会全线失败。
  这不是 lane 的缺陷（Swift 侧的声明与它们的台账一致），是**核心侧还没实现的 shim**：下一轮起把"按名字导出"做成一个真实的活儿
  （逐镜像 `#[no_mangle] extern "C"` 包装，或一个专门的 shim 单元），并在每批落地后让那 427 行按"消失即删行"的棘轮规则收缩。

- **3.235 三条 lane 入库：`font.h` 与 `dispextern.h` 的镜像都转 `OK`，`nsterm` 第 3 轮进来；`extract-abi --all` 只剩 `xdisp.c` 一个 WIP（2026-10-04）**：
  ① **`font.h` 收口（`25115c1`）**：`struct font_driver`（C 589-821，sizeof 208/align 8，`close_font`@64、`text_extents`@104、
  `cached_font_ok`@184…）与 `struct font_driver_list`（C 824-837，24/8，`on`@0/`driver`@8/`next`@16）从 unit 搬进 `include/font.rs`，
  带 13 条实测断言；`draw`/`get_bitmap`/`free_bitmap`（C 682-698）挂 `have_window_system`；unit 侧退休 `font.rs` 两条记录与
  `alloc.rs` 的 9 行 stand-in；台账 **55/55 全勾**，两条 `deferred` 消失 ⇒ `--check emacs/src/font.h …` = **OK**。
  ② **同一提交里的回归与修法（值得记）**：那批新成员把 `prepare_face`/`done_face` 写成了 `*mut face`，而 `struct face` 只在
  `include/dispextern.rs` 的 `dispextern_face_records` 臂里 —— 28 个 include 这份镜像的单元里有 7 个不含那条臂，
  `build-tu src/keyboard.c` 直接 `E0425: cannot find type face`（rustc 的 note 还写着 `found an item that was configured out`）。
  修法照镜像自己的先例（P-72 ③：`*mut c_void` + `/// C:` 原文行），unit 两处调用点加转换。**教训**：`layout-check` 只编布局探针、
  `dead-arms` 只翻配置臂，**镜像改动可以打断它的消费者单元而没有任何门会响** —— 337 单元的那次 `dead-arms` 是全树唯一的批量编译，
  但它不是每次改动都跑。判据在最终产物上重取：`build-tu src/{keyboard,font,alloc,xfaces,dispnew}.c` 五件 built、`layout-check passed`、
  `check-mirror-dupes` 1/1/0。
  ③ **`dispextern.h`（`e14a852`）**：87 个 unit-owned 名字落进**不 glob-import 的模块 `dispextern_unit_names`**（C 顺序、`/// C:` 原文；
  宏/枚举常量/对象声明三类；`FACE_FOR_CHAR` 因为要读 `struct face` 成员而落在同一条 records 臂里）——这是 `xstrcasecmp`（3.224）的形状，
  不需要新 feature 行，而且模块文本被每个包含它的单元与 `layout-check` 真编译。**一处更正**：那条提交实际携带的比它的信息里说的多
  —— 我 stage 的那一刻工作树里已经有该 lane 落下的**下一批 15 个名字**，所以 `e14a852` 的真实状态是 `--check` **3 条 token 行**（不是 15 条 missing）；
  本轮的 3 条 `DEC_TEXT_POS`/`INC_TEXT_POS`/`PRODUCE_GLYPHS` 的 `false` 行（同 FD_SET 那一类）进 `macro-body-notes.tsv` 之后，
  **`--check emacs/src/dispextern.h femacs/src/include/dispextern.rs` = `OK`**。台账未勾 53 → 14（剩下的 5 个记录 + 若干 stale 框）。
  ④ **`nsterm` 第 3 轮入库**：`nsterm.swift` 613 → **757 行**（C 522-641：`ns_relocate`/`ns_init_pool`/`ns_init_locale`/`ns_release_object`/
  `ns_retain_object`/两个池入口），`docs/ports/nsterm.md` 918 行；`swiftc -parse-as-library -c` **exit 0**；
  门读数 `142 C definition(s), 129 unported, 129 row(s); 18 C symbol import(s)`，**0 new / 0 stale**（第 2 轮那 7 个函数对应的 manifest 行已删）。
  本轮的裁决与形状：`ns_relocate` 取 `NS_SELF_CONTAINED` 臂为活（`configure.ac` 689-690 默认 yes、2885-2886 `AC_DEFINE`），
  池三件套走 `objc_autoreleasePoolPush/Pop`（ARC 没有 `NSAutoreleasePool`，第 1 轮 `outerpool` 的同一条），
  `retain`/`release` 用 `Unmanaged` 的引用计数入口（真实 RC 操作，不是 stub），autoreleased 指针用 `passRetained(...).autorelease()`
  保住 C 的生命周期（物证：`nm -u` 里有 `_objc_autorelease`）。
  ⑤ **里程碑读数**：`extract-abi --all` → **checked 590 ported files: 589 done, 1 in progress, 0 failing**。
  **唯一的 WIP 是 `src/xdisp.c`**（`WIP src/xdisp.c -> femacs/src/xdisp.rs (160 differences, docs/ports/xdisp.md has unchecked boxes)`）——
  也就是说 `frame.h`、`font.h`、`dispextern.h` 三个曾经 WIP 的头镜像现在 `--check` 全 `OK`，其余 586 个文件也全部 0 differences。
  其余门：`check-declarations` 13/0、`check-undef` **108 pending / 0 new**、`check-citations` 9 drift/1 range/38 allowed/0 stale、
  `check-swift-units` 0 new/0 stale/7 units、`check-mirror-dupes` 1/1/0。

- **3.236 `xdisp` 第 17 轮入库（15,825 行，前沿 C 15586），并把"还剩多少"第一次量成一张可复算的清单（2026-10-04）**：
  ① **实产**：`femacs/src/xdisp.rs` 15,185 → **15,825 行**，函数 169 → **179/302**、宏 26 → 27、合计 258 → **269/415**；
  `build-tu src/xdisp.rs` 末行 `built`；`--check emacs/src/xdisp.c femacs/src/xdisp.rs` 从 176/178 → **159 条 `- `**
  （`missing function` 123 → 113，正好 10 个；**`does not mention` = 0**）。**前沿：C 15586 `tty_get_tab_bar_item`**
  （lane 自证：C 14785-15566 那整片 `HAVE_WINDOW_SYSTEM` tab-bar 文本是死臂，只记录不转写）。本轮记死臂的段落：
  `gui_consider_frame_title` C 14124-14227、`prepare_menu_bars` 的三段、`update_menu_bar` 的 `HAVE_EXT_MENU_BAR` 臂
  （活臂是它的 `#else`）、`update_tab_bar` C 14588-14594、`display_tab_bar` 的 `HAVE_NTGUI`/toolkit/`HAVE_NS`/X 链
  （活的是末尾 TTY `else`）。SCAFFOLD 烧 2 加 3 → 13 条；新增 7 个 use-site extern（Integrator 37）。
  ② **一条共享行已录**（lane 报、我写）：`restore_selected_window` / `uninit` / `xdisp.c 14541` 的 `Lisp_Object frame UNINIT;`
  （冻结配置里 `UNINIT` = `= {0,}`）在端口初始化为 `Qnil ()`（PORTING.md 1.5）。**值得记的对照**：她这一轮自己也消掉了一条
  自造的 token 行（`restore_frame_selected_window does not mention frame_selected_window`——靠给 `FRAME_SELECTED_WINDOW!` 补
  l-value 臂复现 C 的 token），**没有**往共享表塞一行；这正是"先让端口更忠实、再记账"的正确顺序。
  ③ **剩下的工作量第一次被量出来（可复算）**：把 `docs/port-inventory.md` 的映射与工作树的真实行数对齐，
  **97 个单元的目标文件不到 C 的一半，欠账合计 ≈ 305,000 行 C**。其中**冻结构建**（`docs/baseline/frozen-build-sources.txt`，
  83 个源文件，即最终 Lisp 测试会真正跑到的那些）里还欠：`xdisp.c` 23,980、`comp.c` 5,819、`treesit.c` 5,586、
  `gnutls.c` 3,081、`frame.c` 2,644、`xfaces.c` 2,329、`lib-src/pop.c` 1,673、`sound.c` 1,527、`fileio.c` 1,137、
  `buffer.c` 924、`sqlite.c` 857、`emacs.c` 708、`lcms.c` 592、`menu.c` 524、`window.c` 506、`movemail.c` 498、
  `systhread.c` 447、`decompress.c` 337、`xml.c` 268。**这个算术是上界**：有几个文件是"按设计就这么薄"
  （例：`decompress.c` 的整个函数体在 `HAVE_ZLIB` 里，冻结配置未定义 ⇒ 端口就是空单元，台账里已记），
  所以真正的缺口要逐文件读条件才能定。非冻结构建的平台后端是剩下的主体：`xterm.c` 33,125、`w32*.c` 合计 ≈ 34,000、
  `sfnt.c` 21,215、`image.c` 13,805、`android*.c` ≈ 29,000、`pgtk*.c` ≈ 11,700、`gtkutil.c` 6,753 —— 这些不在冻结构建里，
  但仍属"一比一复刻"的范围内（平台部分按用户指令用新技术重写）。
  ④ **lane 分派（按上面这张表）**：xdisp 第 18 轮（C 15586+）→ 原 lane；`src/comp.c` → font lane（它做完 `font.h` 收口了）；
  dispextern 的 5 个记录 → 原 lane（之后转 `src/xfaces.c`）；nsterm C 641+ → 平台 lane；**`src/treesit.c` 排在新 lane 的第一位**
  ——本轮 spawn 撞到 agent 线程上限（6），下一轮有槽就开。另：`swift-import-pending.txt` 那 70 条待裁定已由一条 lane 逐条取证
  （**65 条属"核心必须以该名字导出"**，每条给了 Rust 侧的字段/宏位置与 C 行号；其余是别的 Swift 单元提供/平台符号），
  我下一轮把它们写进棘轮的 reason 列（表只许集成者写）。
  ⑤ 读数（本轮末）：`check-declarations` 13/0、`check-undef` 108 pending/0 new、`check-citations` 9 drift/1 range/38 allowed/0 stale、
  `check-swift-units` 0 new/0 stale/7 units、`check-mirror-dupes` 1/1/0、`layout-check passed`、
  `extract-abi --all` 589 done / 1 in progress（`src/xdisp.c`）/ 0 failing。

- **3.237 "还剩多少"的口径更正：台账框数才是真实进度（13519 勾 / 604 未勾），`comp.c` 这类"C 行数大"的单元是按设计收口的空形状（2026-10-04）**：
  ① **上一轮那张"C 行数欠账表"是上界，不是欠账**（我在 3.236 ③ 里写了这个保留，这一轮把它量实）。font lane 把 `src/comp.c`
  量穿：`python3 tools/check-cond-eval.py src/comp.c` → **60 conditions / 0 unknown / 0 disagreements [VERIFIED]**；
  `HAVE_NATIVE_COMP` 在冻结配置里是 **undef**（`docs/baseline/config/darwin-arm64.macros.tsv` 1473）⇒ C 25-5578 的整段
  （`comp` 记录与访问器、Limple 操作码/枚举、GIMPLE→Limple 遍、编译器接口）与 `syms_of_comp` 内的同宏区段全是**死文本**，
  按 P-08 / 3.31（空形状）/ 3.12（暂缓）记录不转写；活的只有 C 21-23 的两个 include、`Fnative_comp_available_p`
  与一个 `defsubr`，`comp.rs` 的 38 行已覆盖。**判据**：`--check emacs/src/comp.c femacs/src/comp.rs` = **OK**、
  `build-tu src/comp.c` = built、`docs/ports/comp.md` 的框已勾。**裁决（写在这里免得再摇摆）**：维持现状，
  **不把整片死区当文本承载**（那是政策变更，会与 P-08/3.31 冲突）。
  ② **同一类事实在冻结配置里是普遍现象**（我逐条量了宏表）：`HAVE_GNUTLS`/`HAVE_SOUND`/`HAVE_SQLITE3`/`HAVE_LCMS2`/
  `HAVE_LIBXML2`/`HAVE_ZLIB`/`HAVE_TREE_SITTER`/`HAVE_MODULES`/`HAVE_RSVG`/`HAVE_IMAGEMAGICK`/`HAVE_PNG`/`HAVE_JPEG`/`HAVE_GIF`/
  `HAVE_TIFF`/`HAVE_WEBP`/`HAVE_NATIVE_COMP`/`HAVE_X11`/`HAVE_X_WINDOWS` **全是 undef**，而 `HAVE_PDUMPER` = 1。
  ⇒ `treesit.c`/`gnutls.c`/`sound.c`/`sqlite.c`/`lcms.c`/`xml.c`/`decompress.c`/`comp.c` 这些"行数很大"的文件在本 target 上
  的活文本都极少；上一条状态里把它们按 C 行数排进欠账表是**误读**，本条目更正。
  ③ **真实进度口径：全树台账的框数**（每个单元的 `docs/ports/*.md` 逐条列 C 的 subr/函数/宏/类型/对象）。
  本轮实测：**已勾 13519 / 未勾 604**；未勾的分布（前 13 条，占 604 的 99%）：`nsterm` **345**、`xdisp` **143**、`nsfns` **80**、
  `include-c-ctype` **19**、`include-frame` 3、`include-widgetprv` 3、`macfont` 3、`include-emacsgtkfixed` 2、`nsfont` 2、
  `include-dispextern` 1、`include-stdlib.in` 1、`include-w32` 1、`include-w32gui` 1。
  对照：`extract-abi --all` → **589 done / 1 in progress（`src/xdisp.c`，157 differences）/ 0 failing**；全树
  **没有任何 stub 标记**（`rg -c 'todo!\(\)|unimplemented!\(\)|unreachable!\(\)' femacs/{src,lib,platform}` **零命中**）。
  ⇒ 声明/文本层面的复刻已经到"只剩 xdisp 一个文件有条目"，剩下的是**逐条转写的尾巴**（604 项）与**运行期的两件大事**。
  ④ **到"能跑起来"为止还差什么（写清路线，供后续排期）**：**(a)** 604 条台账项（nsterm/xdisp/nsfns 占 94%），
  **(b)** **链接期的 C-ABI shim**：`tools/check-swift-imports.py` 量出 Swift 的 539 条导入里 **427 条核心还没有导出**
  （其中一条 lane 已逐条裁定：70 条待分类里 **65 条属"核心必须以该名字导出"**），
  **(c)** 加载阶梯（`docs/port-plan.md` §7.1 的 L0–L5）与最终 Lisp 测试套件 —— 这三件做完才算终局。
  ⑤ **lane 分派（按③的框数）**：`nsterm` 第 4 轮（C 641+）→ 原 lane；`xdisp` 第 18 轮（C 15586+）→ 原 lane；
  `lib/include/c-ctype`(19) + `stdlib.in`(1) + 五条小镜像尾巴（frame 3 / widgetprv 3 / emacsgtkfixed 2 / w32 1 / w32gui 1）
  → font lane（它刚把 `comp.c` 量穿）；`include-dispextern` 最后 1 条 → 原 lane；**`nsfns` 的 80 条暂时无人**
  ——它原 lane 已被关闭，而 agent 线程上限（6）挡住了 resume/spawn，等有槽我再把它接上（或由我本人在集成间隙推）。

- **3.238 一条回归的完整闭环、`dispextern.h` 收口、以及 Swift 导入棘轮的 70 条裁定落地（2026-10-04）**：
  ① **回归与修法（`6e975c2`）**：`25115c1` 把 `struct font_driver` 搬进 `include/font.rs` 时，12 个成员写成 `*mut frame`，
  而 `struct frame` 在 `include/frame.rs` 里——**正常构建**下就有 4 个单元编不过（`callint`/`coding`/`marker`/`json`，C 的这四个 .c
  也都不 include frame.h），`dead-arms` 也在 `callint` 上红。修法按 P-72 ③（C 只需要不完整类型：指针不带类型 + `/// C:` 原文行），
  12 个成员改 `*mut c_void`、`font.rs` 15 处调用点加转换、`composite.rs` 为 `driver->encode_char` 自带的那份 byte-padded
  `font_driver` 前缀退休（镜像 owns 了，第二份是 `E0428`）。**验收**：**28 个 include font 镜像的单元全部 built**（不是抽查五个）、
  `layout-check passed`、`dead-arms` 回到 **exit 0 / `337 translation unit(s) compiled with the non-frozen arms`**。
  **教训（比上一轮更锋利）**："镜像改动必须能让每个消费者编过"的验证动作是**枚举全部 include 者逐个编**，
  而不是编 lane 顺手点名的几个——这次是 lane 在 `dead-arms` 上撞见才暴露的，我上一轮的验证太窄。
  ② **`dispextern.h` 收口（`a295cb5`）**：最后 15 个名字进同一模块；5 个记录挂 `dispextern_face_records`（3.220 的同一裁决，
  unit 的带类型副本保持活的）：`bidi_it` 2328/8（32 偏移）、`draw_fringe_bitmap_params` 64/8（13）、`composition_it` 88/8（14）、
  `iterator_stack_entry` 304/8（20，C 把它定义在 `it` 内部、Rust 不能嵌套定义记录，所以提到 `it` 之前）、`it` **4968/8（94 偏移）**
  与匿名联合的三条臂 —— 合计 **180 条实测偏移断言**。判据：`--check dispextern.h` **15 → 0（OK）**、台账未勾 **53 → 1**
  （只剩 `NativeRectangle`，它的 deferral 行 3.224 就写在 `excluded-macros.txt`）、`--all` 不再把它算 in progress；
  臂本身被手工编过（按 `xtask` 的展开规则从 `femacs/` 目录展开 + `--cfg feature="dispextern_face_records"` → exit 0、
  `records probe ok`、180 条断言在场；同一展开从仓库根编会因别的镜像的 `c_variadic` 失败——这是探针 cwd 的问题，已记台账）。
  **仍然开着的缺口（我的文件）**：`xtask dead_arms()` 只翻它那 6 行 `FLIPS`，所以**这条 feature 臂没有任何门在编**；
  要给它一条真 config 常量的 `FLIPS` 行（或让循环跳过空 key）——下一轮做。
  ③ **Swift 导入棘轮的 70 条裁定落地**：那条 lane 把逐条取证写成了 `docs/reviews/2026-10-04-swift-import-rulings.md`
  （70 行表格；裁定计数 **1 → 65**（核心必须以该名字导出）、**2 → 2**（别的 Swift 单元提供）、**3 → 2**（平台符号）、
  **4 → 1**（Swift 侧拼错）），我据此把 `docs/baseline/swift-import-pending.txt` 的 reason 列**逐条改写**，
  并补上 nsterm 第 4 轮在飞带出的 5 条新名字（`FRAME_PARENT_FRAME`、`WINDOW_TO_FRAME_PIXEL_Y`、`ns_auto_hide_menu_bar`、
  `glyph_row_visible_height`、`glyph_row_y`）⇒ `check-swift-imports.py` → **545 imports / 5531 core exports /
  432 unprovided / 432 ratchet rows / 0 failures**。
  ④ 读数（本轮末）：全树台账 **13553 勾 / 570 未勾**（未勾集中在 `nsterm` 333、`xdisp` 143、`nsfns` 80，其余是 3 条以下的小尾巴）；
  `extract-abi --all` 589 done / 1 in progress（`src/xdisp.c`）/ 0 failing；`dead-arms` 337 单元 exit 0；
  `check-declarations` 13/0、`check-undef` 108 pending/0 new、`check-verbatim` 6/4607/0、`check-mirror-dupes` 1/1/0。

- **3.239 两条平台/显示 lane 入库（`xdisp` 第 18 轮 → 16,194 行；`nsterm` 第 4 轮 → 1,084 行），并给 `nsterm` 那条"Swift 声明顺序墙"定了裁决（2026-10-04）**：
  ① **`xdisp` 第 18 轮（`48edb0f`）**：C 15584-16929（1,346 行 C）——活码 342 行（`tty_get_tab_bar_item`、
  `hscroll_window_tree`、`hscroll_windows` + 一个 use-site extern），**死臂 955 行**（Tool-bars 整段在 `HAVE_WINDOW_SYSTEM` 内，
  按 P-08/P-51 记录，三个 subr 按死臂规则留作 `missing subr`），另有 40 行是**刻意缺口**（C 15615-15654
  `tty_handle_tab_bar_click`：`include/termchar.rs:274` 已声明同名 `extern "C"`，这里再定义就是 `E0428`）。
  读数：`xdisp.rs` 15,825 → **16,194 行**、函数 179 → **182/302**、`--check` 157 → **156** 行、
  `does not mention` 归 0（本轮那条 `hscroll_window_tree` / `double` 的共享行已录）。**下一轮起点：C 17006
  `text_outside_line_unchanged_p`**（lane 自证 C 16940-16997 是 `GLYPH_DEBUG` 死块、不是定义）。
  ② **`nsterm` 第 4 轮（本条目同批）**：`nsterm.swift` 757 → **1,084 行**（C 641-1000 段的 `ns_menu_bar_should_be_hidden`、
  `struct EmacsMargins`（clang 实测 32/8，top 0/bottom 8/left 16/right 24）、`ns_screen_margins`、`ns_parent_window_rect`、
  `ns_row_rect`、`ns_frame_scale_factor`、`constrain_frame_rect` 等），台账 1,003 行；`swiftc -parse-as-library -c` **exit 0**；
  门读数 `142 C definition(s), 121 unported, 129 row(s); 25 C symbol import(s)`，删除 8 条既落行的 manifest 后
  **0 new / 0 stale**。形状决定：`ns_auto_hide_menu_bar` 是 **`DEFVAR_LISP`**（C 11697）不是 `DEFVAR_BOOL` ⇒ 按 `Lisp_Object` 导入；
  `row->y`/`row->visible_height` 走 `nsmenu.swift` 311-328 的"逐成员一条 C-ABI 条目"规则（`glyph_row_y`/`glyph_row_visible_height`，
  上一轮已进棘轮）；`ns_parent_window_rect`/`ns_frame_scale_factor` 只落活臂（SDK 26 的 `convertRectToScreen:`/`backingScaleFactor`，
  GNUstep 与 `respondsToSelector:` 回退臂按 P-51 记死）；C 962-985 的 `#if 0` 段不落文本（P-08）。
  ③ **裁决：Swift 的"声明顺序墙"走"不同 Swift 名 + 同一 C 符号"的自引用条目**（`nsterm` lane 在 C 1001 撞上：
  `ns_constrain_all_frames` 要 `[view isFullscreen]`，而 `EmacsView` 在 8,000 行之后；C 1001-6000 里约 71 处同类调用点）。
  规则：调用点前写 `@_silgen_name("<selector>") func <selector>_entry(...)`（接收者在前的 C-ABI 条目），
  类落地处写 `@_cdecl("<selector>") func <selector>(...)`；**Swift 名不同、C 符号相同** ⇒ 不触发同文件 `invalid redeclaration`
  （lane 实测同名会触发），链接期 entry 解析到本文件自己的定义（**`macfont_driver` 的先例**：3.220 批准、3.231 验收 `nm -u` 该名字消失）。
  反对把 `EmacsApp`/`EmacsView`/`EmacsWindow`（C 6225-10610）提前落：那会放弃 C 行序，而类体又反过来调那些自由函数 ⇒
  每个边界都变判断题。**记账要求**：台账 `## ABI` 增"entry 名 / 首个调用点 C 行 / 待定义的类（C 行）/ 状态"表，
  每条在定义落地前都是 `## Deferred` 的一行，最终验收 = `nm -u` 里这些 entry 名全部消失；别的单元已经在用的四个条目
  （`setMiniwindowImage`/`fullscreenState`/`setFSValue`/`beginDrag`）同表。

- **3.240 台账清扫：22 个"陈旧框"清掉、8 个"真欠账"被识别并保持未勾，附带一条可复用的操作规则（2026-10-04）**：
  ① **实产**（ledger-only，本条目同批提交的 7 个 `docs/ports/include-*.md`）：`include-c-ctype` 未勾 **19 → 0**（12 个 `c_*` 函数、
  `C_CTYPE_INLINE`、4 个 `_C_CTYPE_*` 宏都早已在镜像里；另 2 个由 `excluded-macros.txt` 的既有行满足）、`include-frame` 未勾 **3 → 0**
  （`FRAME_FACE_CACHE`/`FACE_FROM_ID`/`FACE_FROM_ID_OR_NULL`——3.230 收口后它们才真正满足）。全树框数：**13567 勾 / 560 未勾**（前 13553/570）。
  ② **8 个"真欠账"没有被勾掉**（勾上就等于把它们变成硬失败＝假绿）：`lib/stdlib.in.h` 的 `struct random_data`（1）、
  `src/w32.h` 的 `_child_process`（1）、`src/w32gui.h` 的 `XImage`（1）、`src/emacsgtkfixed.h` 的 `_EmacsFixed`/`_EmacsFixedClass`（2）、
  `src/widgetprv.h` 的三个 X11 widget 记录（3）。这 5 条各自在台账里写了 `## Decisions (stale-box sweep: **not stale**)`，
  判据与 `missing type` 原文都贴在里面；后四条是**死平台文本**（本 target 无窗口系统/非 Windows/非 GTK），落地时按 P-51/3.9 挂 feature。
  ③ **这条操作规则值得记（台账清扫的可复跑做法）**：**"带延期时 `--check` 干净是二义的"** —— 框未勾时
  `--check` 只在 stderr 打 `missing type deferred: <file> <name> (the ledger has unchecked boxes)`、`- ` 计数是 0；
  **勾框后同一命令会把它变成硬行**。所以清扫的动作是"**勾框 → 复跑 → 若出现硬行就回退**"，本轮 7 条正是这么分开的
  （复跑读数：7 条全部 `- :0`；`build-tu lib/c-ctype.c` built、`build-tu src/frame.c` built、`layout-check passed`）。
  ④ 下一段（该 lane 自己排的序）：先 `lib/stdlib.in.h` 的 `random_data`（**先看生成头 `build/emacs-ref/lib/stdlib.h` 定活臂**，
  P-66/3.207 的 `sys_time.in.h` 先例），再 w32 → w32gui → emacsgtkfixed → widgetprv（含一个匿名成员，按 P-72 ③ 命名）。

- **3.240 补记（同轮）**：`check-undef` 报两条棘轮行已满足（`_restore_frame_selected_window`、`_restore_selected_window`——`xdisp` 第 17/18 轮把它们落成了定义），
  按"行不再匹配即删"的规则从 `docs/baseline/undef-pending.txt` 删掉（110 行 → **108 行**）：复跑 `check-undef` →
  **`108 pending, 0 new`，且不再打印任何 stale 行**。（工具把"仍对应孤儿的行"计作 pending、把"已满足的行"单列 stale，所以 110→108 时 pending 仍是 108 ✅ 两个读数一起读才对。）

- **3.241 两条平台 lane 入库：`nsterm` 第 5 轮（1,300 行）与 `nsfns` 批②（2,394 行），`swift-units` 与 `swift-imports` 两门同时转绿（2026-10-04）**：
  ① **`nsterm` 第 5 轮**：`nsterm.swift` 1,084 → **1,300 行**（C 1001-1211：`ns_constrain_all_frames`、`ns_update_auto_hide_menu_bar`、
  `ns_update_begin`、整个 Accessibility Zoom 块：6 个 static + 2 个区间常量 + `ns_ua_zoom_enabled_p` + `ns_cg_rect_flip_y` + `ns_UAZoomChangeFocus`），
  台账 1,085 行（**95 勾 / 131 未勾**）；`swiftc -parse-as-library -c` **exit 0**（1 条 warning：C 自己 C 1085 的 `[view lockFocus]` 在 macOS 10.14 弃用；
  量级参照：已入库的 `nsimage.swift` 有 4 条）。**3.239 的裁决形状落地**：`@_silgen_name("isFullscreen") func isFullscreen_entry (…)` 在调用点、
  台账 `## ABI` 新表列出 **5 条待定条目**（`isFullscreen` ← C 1001 首个调用点、等 `EmacsView -isFullscreen` C 8957；另四条是别的单元已在用的
  `setMiniwindowImage`/`fullscreenState`/`setFSValue`/`beginDrag`，各自的类行号都记了），**验收 = `nm -u` 里这些名字消失**。
  形态决定：`FOR_EACH_FRAME` 照宏自己拼（不导入）、`UAZoomEnabled`/`UAZoomChangeFocus` 要 `import ApplicationServices` 且形参类型要显式转换、
  `NSApplicationPresentationDefault` 在 Swift 里是 `[]`、`cgRectJiggle` 要 `var`（`&` 传参）；她还**手工补了生成器漏掉的 4 个盒子**
  （两个 `static inline`/跨行 `static` 的函数 + 两个区间常量，并记了"重生成会丢"）。
  ② **`nsfns` 批②**：`nsfns.swift` 1,482 → **2,394 行**（+915，C 1188-1884），台账 383 → 487 行（未勾 **71**）；
  `swiftc -parse-as-library -c` **exit 0 / 0 error / 0 warning**（那一批唯一的 warning 是 `main_window` 未变，已改 `let`）；
  新增 **7 个 `@_cdecl` 导出**：`Fx_create_frame`（DEFUN 1/1，C 1192 那个 418 行的大函数）、`Fns_frame_list_z_order`、`Fns_frame_restack`、
  `Fx_select_font`、`Fns_popup_color_panel`、`ns_run_file_dialog`、`Fns_read_file_name`（6 个 `S…` 记录照 C 的 file-static 用 `fileprivate`）。
  形态决定：`f->output_data.ns = xzalloc (…)` 走 core 的 `frame_alloc_ns_output (f)`（记录归 core、分配是 core 的动作）；
  15 个 `Emacs_Cursor` 槽 + 2 个滚动条 cursor 走**逐成员访问器**，并**删掉了批① 那条拼错的 `ns_output_set_cursor_kind` 字符串派发**；
  6 个 DEFUN 对用本批新增的 `make_aligned_subr`（含 `PVEC_SUBR << PSEUDOVECTOR_AREA_BITS` 头字）；`record_unwind_protect` 传非捕获闭包；
  `MODAL_OK_RESPONSE` = `NSApplication.ModalResponse.OK`；`id panel` → `NSSavePanel? as? NSOpenPanel`；`[NSSavePanel savePanel]` → `NSSavePanel()`；
  `[[NSApp orderedWindows] reverseObjectEnumerator]` → `NSApp.orderedWindows.reversed()`；`EmacsFileDelegate_new ()` 等下一批。
  ③ **两条棘轮/清册的同步**：`swift-units.tsv` 删 **6** 行（Poincare 4 + Boyle 2），`swift-import-pending.txt` 本轮 **+102 行**（在飞 lane 新增的
  "core 还没导出"导入，按四类写 reason）并删 **3 条 stale**（`Vframe_list`、`isFullscreen`、`ns_output_set_cursor_kind`——最后一条正是 Boyle 删掉的错拼法）。
  **读数的口径提醒**：`check-swift-imports.py` 的"provider 集"取自 `build/femacs/**/*.o` 的导出 + Swift 的 `@_cdecl`，所以 `build/` 里若存在临时探针
  （例如 shim 设计期的 `abi_shim.rs`）会瞬时"提供"某些名字；判据要在**落库后的那次构建**上重取。
  ④ 读数（本轮末）：`check-swift-units` **0 new / 0 stale / 7 units**；`check-swift-imports` **662 imports / 5532 core exports /
  533 unprovided / 533 ratchet rows / 0 failures**；全树台账 **13596 勾 / 531 未勾**（未勾：`nsterm` 323、`xdisp` 131、`nsfns` 71、`macfont` 3、`nsfont` 2）；
  `check-declarations` 13/0、`check-mirror-dupes` 1/1/0、`check-verbatim` 6/4607/0、`layout-check passed`。

- **3.242 `xdisp` 第 19 轮入库（16,570 行，前沿 C 17351 `redisplay_internal`），并给连接期 shim 的落位定了案（2026-10-04）**：
  ① **实产**：`femacs/src/xdisp.rs` 16,194 → **16,570 行**（C 17005-17348，**344 行全是活码**；唯一死臂是 `overlay_arrow_at_row` 内的
  C 17233-17240 七行，按 P-08 记录）。新增 **10 个函数**（`text_outside_line_unchanged_p`、`redisplay`（非 static ⇒
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn`）、`overlay_arrow_string_or_property`、`overlay_arrow_in_current_buffer_p`、
  `overlay_arrows_changed_p`、`update_overlay_arrows`、`overlay_arrow_at_row`、`check_point_in_composition`、`reconsider_clip_changes`、
  `propagate_buffer_redisplay`）+ **2 个宏**（`STOP_POLLING`/`RESUME_POLLING`）+ 3 个 use-site extern；Scaffold 不烧不加（`redisplay_internal`
  按 C 自己的前置声明（C 1149）必须留到 C 17351 落地，实测块内 `fn` 行仍 13）。台账：Functions **182 → 192/302**、Macros **27 → 29/38**、
  合计 **272 → 284/415**。**判据**：`build-tu src/xdisp.rs` → built（对象 mtime 22:16:56 晚于源 22:16:51，6 warnings 与上轮完全同）；
  `--check emacs/src/xdisp.c femacs/src/xdisp.rs`：157/156 → **149**，本轮的 5 条共享行录入后 → **144**，
  **`does not mention` = 0**、`stub or empty body` = 0。**下一轮起点：C 17351 `redisplay_internal`**（891 行的最大函数，之后是
  `unwind_redisplay_preserve_echo_area` C 18238）；唯一早于边界的 `missing function` 是 C 15616 那个**故意的 E0428 缺口**（item 38，不是未到达）。
  ② **5 条共享行已录**（lane 逐条给了 C 行号与 deviation 号，我只做录入）：`overlay_arrow_in_current_buffer_p`/`overlay_arrows_changed_p`/
  `update_overlay_arrows`/`overlay_arrow_at_row` 的 `continue`（C 的 `for (…; COND; INC)` 带 `continue` ⇒ 端口用 `'body:` 标签块把
  C 的 increment 放在体末，保证每条路径都执行；deviation 133）与 `STOP_POLLING` 的 `false`（`do { … } while (false)` 在 Rust 无对应物；
  deviation 140；`RESUME_POLLING` 的体仍提到 `false`，不需要行）。
  ③ **连接期 shim 的落位定案（写给后续）**：**候选 A 批准** —— 新建 `femacs/src/abi_shim.rs`（+ 台账 `docs/ports/abi_shim.md`），
  `include!` 所需镜像，只放 `#[unsafe(no_mangle)] pub unsafe extern "C" fn` 导出。**理由是硬的**：镜像是被 `include!` 进**每个**单元的，
  在镜像里写定义＝每单元一份定义（`E0428`）；而"没有对应 C 源的新产物"在本仓库有 `xtask` 的先例，门也放行（`check-target-names`
  实测 `627 .rs / 591 targets / 0 finding(s)`）——**文件首行保持 `// femacs: generated mirror of the frozen build's C-ABI exports …`
  即命中既有 `DECLARED` 模式（该工具第 6 条），不需要改工具**。三条语义裁定同时定下：`frame_tab_bar_window` 导出 `Qnil` 并在
  `/// C:` 行注明"该成员在 `#ifdef HAVE_WINDOW_SYSTEM` 内、本 target 无它"（先例：comp.rs 的 `Fnative_comp_available_p` 冻结臂）；
  `frame_set_external_menu_bar`/`frame_set_external_tool_bar` 导出成 **no-op** 并引 frame.h 1082/1097 的 `#define … false`；
  其余按 lane 表里的签名/体落，每批 8-10 条、逐字对照 `nm -g` 与 Swift 的 `@_silgen_name("…")`，参数 ABI 不一致时**以 Swift 侧为准**。

- **3.243 8 个真欠账落地（5 个镜像的 8 条记录），并更正 3.240 那轮的一次按位回退失误（2026-10-04）**：
  ① **实产**（5 个镜像 + 5 个台账）：
  - `lib/include/stdlib.in.rs` 的 `struct random_data`（模板 88-102，7 成员）：**死臂**——生成头 `build/emacs-ref/lib/stdlib.h` 第 88 行是
    `# if !1`，即冻结 `<stdlib.h>` 自带该类型 ⇒ gnulib 臂是死文本，挂 `no_struct_random_data`（P-66/3.207 的"按生成头定活臂"先例）。
  - `src/include/w32.rs` 的 `_child_process` + `child_process` 别名（C 74-113）、`src/include/w32gui.rs` 的 `_XImage` + `XImage`（C 38-44）：
    **死平台**（本 target 无 `_WIN32`）⇒ 挂 `windows`。
  - `src/include/emacsgtkfixed.rs` 的 `_EmacsFixed`（C 39-44）、`_EmacsFixedClass`（C 46-50）：无 GTK/PGTK ⇒ 用该头自己的 `HAVE_PGTK` 名；
    C 的成员 `priv` 在 Rust 是 `r#priv`。
  - `src/include/widgetprv.rs` 的 `EmacsFramePart`（C 27 的匿名 `typedef`，取别名名，P-72 ③）、`EmacsFrameRec`、`EmacsFrameClassRec`：
    无窗口系统（`HAVE_X11` undef）⇒ 挂 `have_x11`。成员名与顺序一律照 C（字段成员检查比的就是名字）；死臂内的成员类型保 C 拼写
    （`HANDLE`/`BITMAPINFO`/`GtkFixed`/`Boolean`/`Pixel`/`CorePart`…），因为冻结构建不编它们。
  **判据**：五条 `--check` 全部 `OK`、**stderr 里再无 `missing type deferred`**（`- :0`、`deferred:0`）；`layout-check passed`
  （它编全树 `src/include` 镜像与 include 图）。一条口径说明：`lib/stdlib.in.rs` **今天没有任何单元 include 它**（实测
  `grep -rln 'include/stdlib.in.rs' femacs/lib femacs/src` 为空），所以"最小包含者 `build-tu`"对它不成立（头形式需要 includer 提供标量，
  这一点在我动手前就如此）；该镜像新增内容全在 `#[cfg (feature = "no_struct_random_data")]` 里，故最强证据是 `layout-check` + cfg 门。
  ② **更正上一轮的一次失误**：3.240 那条"回退假勾"是**按位置**（前 n 个 `- [ ]`）做的、不是按名字，于是勾/取消勾错位：
  该回退的 8 个**类型框**被留成已勾，而 `_gl_MB_CUR_MAX`/`CHILD_ACTIVE`/`AllValues`/`EMACSGTKFIXED_H`/`_EmacsFrameP_h`/`<anon@61>`
  这些**已满足**的框被误取消——`0570d2b` 提交的就是那个状态。本轮把 8 个类型落地后，**按名字**把 5 份台账重新核对并勾满
  （5 份都 0 未勾），并各自补了 `## Decisions`/`## Evidence`（判据 + 读数 + 前后框数）。**教训**（与 3.240 的规则同族）：
  **框位纠正必须按名字做，且每次改完要复跑 `--check`**——按位置批量回退会把两类错误互换成一个更难发现的错误。
  ③ 读数：全树台账 **13596 勾 / 531 未勾**（未勾仍是 `nsterm` 323、`xdisp` 131、`nsfns` 71、`macfont` 3（永久死臂）、`nsfont` 2
  （两条 deferred 体，等 `glyph_string` 的字段））。**该 lane 的 `include-*.md` 尾巴全部清零**。

- **3.244 第一次量"链起来还缺什么"：C 自己的对象集（程序 76 + `libgnu.a` 86 = 162 个）在 `ld -r` 下报 **427 个未定义符号**；方法与读数写进 `docs/ports/link.md`（2026-10-04）**：
  ① **为什么量这个**：全树的门都停在 `.o` 或文本层——`build-tu` 只编单个单元、`layout-check` 编探针、`check-undef.py` 看每个单元自己的
  undefined 列表、`check-swift-imports.py` 只管 Swift 侧的 C-ABI 缝——**没有任何东西量过"整个程序"**。这就是 L0 之前那一级的欠账。
  ② **对象集必须照 C 的配方取**（`build/emacs-ref/src/Makefile`：`temacs: $(ALLOBJS) $(LIBEGNU_ARCHIVE) …`、`LIBEGNU_ARCHIVE = $(lib)/libgnu.a`）：
  `docs/baseline/frozen-build-sources.txt` 的 83 个源映射到 `build/femacs/<stem>.o`，**减去**定义了 `_main` 的那些（那张清单里混着 7 个独立程序：
  `emacsclient`/`ebrowse`/`etags`/`hexl`/`make-docfile`/`make-fingerprint`/`movemail`），**加上** `libgnu.a` 的成员。
  **实测教训**：① 用 `nm` 的 ` T _main` **前缀**匹配会把 `src/thread.c`（它的 `_main_thread_p` 以 `_main` 开头）误删，正确判据是 `^\S+ [Tt] _main$`；
  ② `build/femacs/*.checking.o`（`enable_checking` 那份）必须排除——第一版没排，`ld -r` 报 **4,978 个重复符号**；
  ③ 只取"冻结源"（不含 libgnu）时报 **459** 个未定义，取全 C 的对象集（162 个）时报 **427**。
  ③ **读数与分类**（对照 `nm -g build/emacs-ref/**/*.o` 的定义/未定义两个集合）：`rust-runtime` 20（`_RNv…4core…panic*`，链接期由 Rust 运行时提供）、
  `libc-or-platform` 269（参照对象也引用它们 ⇒ libc/SDK 提供）、**`port-owed` 59（参照定义了、我们没有任何单元定义）**、
  `unknown` 79（主要是 C `globals.h` 生成的 `V…` 载体符号——那是端口**已记录**的偏离（`check-symbols.py` 记 467 个"折叠成 Rust 静态量"），
  加上 `src/xdisp.c` 还没移植的余量 `calc_pixel_width_or_height`/`automatic_hscrolling`/`composition_break_at_point`/`handle_line_prefix`/
  `maybe_produce_line_number` 等，以及少数 libc 名）。
  ④ **两条已测的保留**（写进 `docs/ports/link.md`，不让它变成假结论）：`libgnu.a` 成员 → 单元对象的**映射要精确**——`mini-gmp.c` 的成员名与单元 stem
  不同，所以那 59 条 `port-owed` 里绝大多数是 `mpz_*`，而 `femacs/lib/mini-gmp.rs` **是定义了它们的**（`nm -g build/femacs/mini-gmp.o` 打 `T _mpz_add`）；
  `V…` 那一类必须由链接器决定：(a) 照样导出、(b) 记进棘轮、还是 (c) 证明没有单元按那个名字引用它——本文件只记录问题，不给答案。
  ⑤ **下一步**：写 `xtask link`（新子命令）——按上面的配方组对象集、跑部分链接、打印未定义清单、与棘轮 `docs/baseline/link-pending.txt` 比对
  （**新名字 FAIL、消失即删**，形状照 `check-undef.py`/`check-swift-imports.py`）；它的第一件活是把④的两条保留解决掉，第二件是把这份清单与
  `docs/baseline/swift-import-pending.txt`（533 条 Swift 侧导入）**求交集**——那个交集就是"能跑的 macOS 构建"最先需要的那批 C-ABI 条目，
  它们落进 `femacs/src/abi_shim.rs`。

- **3.245 `nsterm` 第 6 轮入库（1,538 行）——本文件的**第一个类**落地，且它证明 3.239 的裁决形状有"不需要用"的一侧（2026-10-04）**：
  ① **实产**：`nsterm.swift` 1,300 → **1,538 行**（C 1212-1492，281 行 C）：`ns_update_end`(1212)、`ns_focus`(1236，**又一个生成器漏项**——
  `static void` 跨行导致 reader 跳过，手工补盒并记"重生成会丢")、`ns_unfocus`(1282)、可见铃的叙述注释(1313-1330)、
  **第一个类 `EmacsBell`**(1333-1430：`@interface` + 四个方法)、`bell_view`(1433)、`ns_ring_bell`(1436-1469)；
  `hide_bell`(1471-1491) **判死**（守卫是 `#if !defined (NS_IMPL_COCOA) || MAC_OS_X_VERSION_MIN_REQUIRED < 101400`，
  本 lane 目标两半都不成立，唯一调用点 C 9388 也在同一守卫内 ⇒ 按 P-08 记录不落文本）。台账 95 → **103 勾 / 124 未勾**；
  `swiftc -parse-as-library -c` **exit 0，8 条 warning**（4× `lockFocus` + 4× `unlockFocus`，全是 C 自己的调用，
  C 1085/1219/1220/1249/1300/1302 一带；量级参照：已入库的 `nsimage.swift` 同命令 4 条）。
  ② **裁决形状的第一次"不需要用"**：`EmacsBell` 是第一个类，但它的方法调用点都在类之后（类在 C 1345 落地、首个外部调用在 C 1436+）
  ⇒ **完全不需要 entry**，台账 (4) 待定条目表本轮 **+0**（`isFullscreen` 等跨单元条目仍按 deferred 留着，`nm -u` 里 `_isFullscreen` 未消失是**正确**状态）。
  这补齐了 3.239 那条裁决的另一半：entry 只在"调用点早于定义点"时才需要。
  ③ **本轮新面**（按既有规则）：`MOUSE_HL_INFO (f)->mouse_face_defer = 0` 是该单元第一次写"没有宏命名的核心记录成员"，而它是
  `bool_bf : 1` **位域**（Swift 视图写不了）⇒ 走**成员条目** `mouse_highlight_set_mouse_face_defer`（命名照 `nsmenu.swift` 311-328 的
  `ns_output_set_*` 族）；`NSRectClip`/`NSRectClipList` 是 NSGraphics.h 210-211 的 `APPKIT_EXTERN` C 函数、Swift 标 unavailable ⇒ 按名导入
  （`nsfont.swift` 对 `fprintf` 的同款读法）；`visible_bell`（`DEFVAR_BOOL` 数据符号）与 `SELECTED_FRAME`（宏条目）。
  棘轮 `swift-import-pending.txt` **+2 行**（`visible_bell`、`mouse_highlight_set_mouse_face_defer`，reason 按 lane 建议）；`swift-units.tsv` **−3 行**（本 lane）。
  ④ 三门读数：`check-swift-units` **0 new / 8 stale**（那 8 条是 `nsfns` 批③ 在飞的，不是本件）；`check-declarations` 13/0；
  `check-citations` 本 lane **1 条 `external`**（`nsterm.swift:1312` 引 `NSGraphics.h 210-211`＝AppKit 系统头，按规则"counted, never a failure"）。
  ⑤ 下一段起点：**C 1493 `ns_get_focus_frame`** → `ns_focus_frame`/`ns_raise_frame`/`ns_lower_frame`/`ns_frame_raise_lower`/`ns_frame_rehighlight`/
  `ns_make_frame_visible`…（frame 管理段，正是 (4) 待定条目表开始长行的地方）。

- **3.245 补记（同轮）：`docs/ports/link.md` 的保留①被量成了结构性的**——`libgnu.a` 的成员是 `mini-gmp-gnulib.o`，而 `nm -g build/femacs/mini-gmp-gnulib.o`
  **没有任何 `mpz_*`**（它们在这个端口里是**独立单元** `mini-gmp.o`，`grep -c ' T _mpz_'` = 60，因为 `lib/mini-gmp.c` 是单独的移植文件）⇒
  链接集不是"把归档成员按 stem 映射"那么简单：**还要加上那些被 amalgamating 成员 `#include` 的单元**（gnulib 用 `#include` 合并源码、而本端口一个文件一个单元）。
  这条写进了 `docs/ports/link.md` 的保留①：在那条规则写下来之前，`port-owed` 是线索不是判决。

- **3.246 链接那一级的门落地：`tools/check-link.py`（部分链接 + 棘轮）与第二条读数——**`port-owed` 归零**（2026-10-04）**：
  ① **新工具**：`tools/check-link.py` 组 C 自己的对象集（`docs/baseline/frozen-build-sources.txt` 的 83 源映射到 `build/femacs/<stem>.o`，
  减去**精确**匹配 `^\S+ [Tt] _main$` 的那些＝7 个独立程序，再加 `libgnu.a` 的成员），跑 `ld -r`（部分链接：不需要运行时库、不需要可执行成功），
  `nm -u` 出"程序引用、没有任何成员定义"的名字，按参照对象（定义/引用两个集合）与 SDK `.tbd` 分类，
  与棘轮 `docs/baseline/link-pending.txt` 比对（**新名字 FAIL、消失即删**）。未构建的对象存在时**拒绝比对**（会虚增清单），
  提示先跑 `gate-fast`/`build-tu`。
  ② **本轮修正的那条规则**（上一轮的保留①）：**gnulib 用 `#include` 合并源码、本端口一个文件一个单元** ⇒ 归档成员 `X-gnulib.o` 还要
  加上同名的 `X.o`（`mini-gmp-gnulib.o` 里**没有一个 `mpz_*`**，`build/femacs/mini-gmp.o` 里有 **127** 个）——加上这条之后，
  未定义符号从 **427 → 369**（`libc-or-platform` 270、`rust-runtime` 20、`unknown` 79），而 **`port-owed` 从 59 → 0**：
  **参照的单元定义的每个符号，本端口的单元也都定义了**。这是一条实质结论，改了"链接还欠什么"的判断。
  ③ **那 79 条 `unknown` 的实测分解**：**19** 条 `V…` 载体（端口**已记录**的偏离，`check-symbols.py` 记 467 个"折叠成 Rust 静态量"）、
  **7** 条是 `src/xdisp.c` 还没落的名字（随它落地消失）、**53** 条尾巴——其中大多数是**参照-based 分类器看不到的 libc/BSD 名**
  （C 经内联/宏路径到达它们：`clearerr`/`environ`/`fabs`/`fdopen`/`feof`/`ferror`/`fsync`/`getc`/`getchar`/`htons`/`lrint`/`ntohs`/`pathconf`/`pipe2`/`pselect`/`putc`/`putchar`/`realpath`/`sqrt`/`sync`/`umask`/`sigaddset`/`sigemptyset`…），
  其余是端口**有意的结构偏离**（`current_buffer`、`specpdl`/`specpdl_ptr`/`specpdl_end` 在本端口是 Rust 结构）与 `src/xwidget.c` 未移植的 `syms_of_xwidget`。
  ④ **因此链接这一级的真实状态**：**对象集自闭合**；剩下的是**一个决定**（`V…`/`specpdl`/`current_buffer` 这些偏离要不要还是给 C-ABI 载体）、
  `xdisp` 的余量、以及真正的链接驱动那次带 `-lSystem` 与各 framework 的 `cc` 调用。读数、方法与两条教训（`_main` 前缀误伤 `thread.o`、
  `.checking.o` 必须排除）都在 `docs/ports/link.md`；棘轮 369 行、门绿。

- **3.247 两条平台 lane 入库（`nsterm` r7 → 1,804 行；`nsfns` b3 → 3,195 行），导入门把"平台提供的名字"与"核心欠的名字"分开（2026-10-04）**：
  ① **`nsterm` 第 7 轮**：1,538 → **1,804 行**（C 1493-1743）：焦点钩（`ns_get_focus_frame`/`ns_focus_frame`）、raise/lower 对与派发、
  `ns_frame_rehighlight`、可见性钩（`ns_make_frame_visible`/`ns_make_frame_invisible`/`ns_make_frame_visible_invisible`/`ns_iconify_frame`，
  其中三个是 `@_cdecl` 导出）；台账 1,189 行（113 勾 / 114 未勾）；`swiftc -c` **exit 0 / 8 warnings**（仍是 C 自己的 `lockFocus`/`unlockFocus` 那批）。
  **裁决形状第一次扩展到"函数"**：`ns_set_offset`（调用 C 1691 / 定义 C 1854）与 `ns_set_frame_alpha`（调用 C 1616 / 定义 C 2366）是本单元自己的函数、
  调用点早于定义点，而 **C 自己在 C 1622 就写了前向声明**——参考侧对同一情形的证据；故同样 entry/定义分离，定义处 `@_cdecl("<C 名>")`，
  台账 (4) 表两行标 `deferred (function)`。成员条目按既有命名族（`ns_display_info[_set]_<member>`、`frame_<member>`），宏/INLINE 照 C 自己的名字。
  另外 `check-citations` 抓了他一条 drift（`EmacsView` 与 `src/nsterm.h 1226` 写在同一行）⇒ 已改。
  ② **`nsfns` 批③**：2,394 → **3,195 行**（+801），台账 571 行（未勾 37）；`swiftc -c` **exit 0 / 0 warning**；
  新增 11 个函数 + 23 个 DEFUN。形态决定（都实测）：两个返回 `const char *` 的函数**不能**把 `@_cdecl` 的返回类型写成 `UnsafePointer<T>?`
  （Swift 6.4 报 "its result type cannot be represented in Objective-C"）⇒ 用 mutable 指针拼法；`ns_get_defaults_value` 返回的是
  autoreleased NSString 内部的指针，Swift 桥接临时量一返回就释放 ⇒ 端口用**单槽缓存**保住最后一条字符串；`ns_init_colors` 的
  `@try/@catch (NSException *)` Swift 抓不住 ⇒ 那次被保护的 `writeToURL:` 走端口命名入口 `ns_color_list_write_or_log (cl)`
  （失败返回 C 的 `false` 并照 `@catch` 打日志）；`NSBestDepth` 用 `@_silgen_name` 直接绑 SDK 符号；AppleScript 常量来自 Carbon 伞头 ⇒ 文件新增
  `import Carbon`；`struct input_event ev` 只有地址跨进 nsterm.m 的两个钩子 ⇒ 按实测 `sizeof` 分配 64 字节堆缓冲；
  死臂（`MIN_REQUIRED < 101300` 的 backing-store 两支、`ns-hide-emacs` 的 GNUSTEP 臂、`ns-list-services` 的 `#else`、颜色回退等）记而不载；
  他还删掉了一条自己多建的 `EmacsScroller_scrollerWidth` 入口（避免白占棘轮一行）。
  ③ **导入门的口径改了（本轮）**：`tools/check-swift-imports.py` 现在把**SDK framework 的 `.tbd` 导出的名字当作平台提供**（不再算 finding）——
  棘轮因此**恰好等于"核心还欠的 C-ABI 条目"**。连带：6 条 system 行退出棘轮（4 个 CoreGraphics + 2 个 objc），41 条新名字按类进表；
  **"瞬时提供者"的处理**：那 10 条 `frame_*` 曾因 Harvey **未提交**的 shim 对象而显示 stale ⇒ 我删掉 `build/femacs/abi_shim.o`，
  让棘轮反映**已提交的树**（他下一次 `build-tu` 会重建，届时随 shim 入库一并删行）。这条与 3.241 记的口径同源：**判据要在已落库的状态上取**。
  ④ 读数（本轮末）：`check-swift-units` **0 new / 0 stale / 7 units**；`check-swift-imports` **724 imports / 5557 core exports /
  567 unprovided / 567 ratchet rows / 0 failures**；全树台账 **13678 勾 / 885 未勾**——未勾的跳升是**新的 `docs/ports/xterm.md`**（413 个框，
  那是新战线的完整工作清单，不是回归）；其余仍是 `nsterm` 299、`xdisp` 130、`nsfns` 37、`macfont` 3（永久死臂）、`nsfont` 2。

- **3.248 连接期 shim 批① 入库（`abi_shim.rs` 34 个 C-ABI 导出），导入门的 STALE 一分为二，并记下一条"台账散文"的坑（2026-10-04）**：
  ① **实产**：新单元 `femacs/src/abi_shim.rs`（122 行）+ 台账 `docs/ports/abi_shim.md`。选址按 3.242 的裁决（候选 A）：镜像是被 `include!` 进每个单元的，
  在里面写定义＝每单元一份定义（`E0428`）；没有 C 配对的新产物有 `xtask` 先例，门也放行（`check-target-names` 实测 627 .rs / 591 targets / **0 finding**，
  首行 `// femacs: generated mirror of the frozen build's C-ABI exports …` 命中既有 `DECLARED` 模式，**不需要改工具**）。本批落 **10 个 `frame_*` 取值器**
  （`frame_left_pos`/`frame_top_pos`/`frame_title`/`frame_icon_name`/`frame_param_alist`/`frame_selected_window`/`frame_menu_bar_vector`/`frame_tool_bar_items`/
  `frame_menu_bar_items_used`/`frame_n_tool_bar_items`），**签名与 Swift 的 `@_silgen_name` 逐字一致**（`UnsafeMutableRawPointer?`↔`*mut c_void`、`Int32`↔`c_int`、
  `Lisp_Object`↔端口 8 字节 `Lisp_Object`），体是 `include/frame.rs` 的 `pub` 成员直读 + `/// C:` 行带 C 行号（frame.h 604/162/158/209/1204/234/283/307/573）。
  **判据**：`build-tu abi_shim` → `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`（无 C 源的 stem 兜底已记台账）；`nm -g` = **34 个 `T _`**；
  导入门那 10 条从"未提供"变成 **`STALE (satisfied)`** ⇒ 行删掉（棘轮 −10，**557 行**）。
  ② **导入门的 STALE 一分为二**（本轮改，回答 `abi_shim.md` 提出的措辞坑）：`STALE (satisfied)` = 名字**已被提供**（删行即可）vs
  `STALE (unimported)` = **没有任何 Swift 单元再导入它**（要去查调用点——行可能为某个从未用过的拼法加的）。判据是"名字是否还在 `imp` 集合里"，
  与 `missing`/`rows` 的差集一起用。这条同时解释了上一轮"10 条 `frame_*` 瞬时 stale"的歧义（`tools/check-swift-imports.py` 的分支现在会直接说 `satisfied`）。
  ③ **一条新的坑（值得所有人记）**：台账散文里**不能写"未勾选框"的字面拼法**。shim 台账第一版在 `## Decisions` 里引用那个两字符记号，结果 `xtask` 的
  `finished_units()` 把它当成"还有未勾框"⇒ **该单元不进流水线**（`dead-arms` 只有 337 个单元、没有 `src/abi_shim.rs`）；改成散文措辞后
  `dead-arms` 编它了：**`338 translation unit(s) compiled with the non-frozen arms`**。这是 3.198/3.202 那一族"把散文当数据"的又一实例。
  ④ 其余门读数（batch ① 收口时）：`check-globals` **284 units / 0**（shim 算一个单元）、`check-mirror-dupes` 1/1/0、`check-verbatim` 4607/0、
  `layout-check passed`、`check-undef` 0 new（shim 的导入全在 `build/femacs` 里被满足）、`check-symbols` 86 units / 1 mismatched（`lib/regex`，非本件）。
  ⑤ **一条瞬态说明**：`check-swift-imports` 此刻报 28 条 failure——那是在飞的 Swift 轮次（`nsterm` r8 / `nsfns` b4）新引入的导入，按惯例在它们入库时录入棘轮；
  同理 `ns_set_offset`/`ns_free_frame_resources` 这两条已显 `STALE (satisfied)` 是因为 `nsterm.swift` 的**未提交** r8 已在工作树里定义了它们——
  **判据在已落库的状态上取**（3.241/3.247 的同一条）。

- **3.249 `xterm.c` 这条战线开篇（设计 + 第一片）：非冻结平台文件的"活侧"裁定，与 443 行剩余清单（2026-10-04）**：
  ① **量清的实况**（lane 实测）：`src/xterm.c` **33,125 行**、代码正文起 **C 625**；`check-cond-eval src/xterm.c` → **852 conditions /
  0 unknown / 0 disagreements [VERIFIED]**（在 `cond-eval-verified.txt` 426 行 ⇒ 冻结宏表**读得准**这个文件）；`HAVE_X_WINDOWS` **全文只出现 1 次**，
  且在一个 `#elif`（C 26143）——**这个文件不是被它包起来的**，"本 target 是否编它"由 Makefile 决定（`frozen-build-sources.txt` 里没有 `src/xterm.c`）；
  条件组地图：`USE_GTK` 204 组/4,223 行、`USE_X_TOOLKIT` 77/1,676、`HAVE_XINPUT2` 75/5,309、`USE_CAIRO` 48/2,009、`USE_TOOLKIT_SCROLL_BARS` 45/2,077、
  `USE_XCB` 42/840、`HAVE_X_I18N` 34/845、`USE_MOTIF` 25/1,123…；按冻结表：**活 19,809 / 死 13,317 行**（可归属的死行：`HAVE_XINPUT2` 1,442、`USE_GTK` 1,231、
  `#else` 臂 1,172、`USE_CAIRO` 850、`USE_X_TOOLKIT` 828、`USE_MOTIF` 807…）。
  ② **裁决（三条路线里选）**：lane 给出路线 A（另立 X11/GTK3 表：667 个必须名、26,914 活行；**要新机制**——今天是一 target 一张表 + 文件白名单，没有 per-file 表，
  且仍漏 Lucid/Motif/Xaw/Xt 那 59 个名字）与路线 B（**盲读**：721 个必须名、33,006 活行；零新机制，一行共享文件——把 `src/xterm.c` 从 `cond-eval-verified.txt` 去掉），
  并建议**两者都不选**。**我采纳他的建议**：**清单保持冻结读数，但按 P-51 把各臂的文本都承载**（两臂都写、非冻结一侧挂 feature 并在台账记名）——
  理由：① 同一子系统（`src/include/xterm.rs` 头镜像 b03d883 已经落成这个形状：全身挂 `have_x_windows`、清单取冻结读数、清单外的臂在台账记名）不该有两种读数；
  ② 路线 A 要为一个**冻结 target 从不编译**的文件发明新机制；③ 范围是 P-42 的 1:1 文本，臂无论选哪条读数都要写，读数只决定"**必须**"集合。
  **若将来真要配一个 X11 target**，正确且最便宜的开关是路线 B（一行）。
  ③ **第一片（C 625-990）**：`femacs/src/xterm.rs` 616 → **944 行**、台账 `docs/ports/xterm.md` 658 行（413 个未勾框＝这条战线的完整工作清单）；
  落了 19 个名字 + 2 个 typedef + 2 个 record（宏 `XtNpickTop`/`XtNinitialState`/`USE_CAIRO_XCB_SURFACE`/`HAVE_XCB_SHAPE_INPUT_RECTS`/`X_XI*`/`X_NEXT_KBD_EVENT`/`ATOM_REFS_INIT`；
  对象 `xg_pending_quit_event`/`any_help_event_p`/`x_display_list`/`Xt_app_con`/`Xt_default_resources`/`toolkit_scroll_bar_interaction`/`emacs_class`/…；
  函数 `x_defer_selection_requests`/`x_release_selection_requests`/`x_release_selection_requests_and_flush`；记录 `x_selection_request_event`/`x_atom_ref`；
  typedef `Emacs_XErrorHandler`/`Emacs_XIOErrorHandler`）。**记录在案的降级**：`Display`/`XErrorEvent`/`XtAppContext` 用不透明指针 + `/// C:`（P-72 ③）、
  `EMACS_CLASS` 展成 `[c_char; 6]`（frame.h 1840 的 `"Emacs"`）、`#ifndef XtNpickTop` 那类是"X11 头自己是否已定义"的守卫（端口看不到那些头 ⇒ 按外层 feature 承载并记名）、
  `buffered_input_event.sie`（C 917/920）与 `x_handle_pending_selection_requests`（xterm.h 1857）两个名字在镜像里**没有**（前者只在 `include/keyboard.rs` 的注释里、
  后者只在 `include/xterm.rs` 的宏体里）⇒ 已记台账。
  ④ 读数：`build-tu src/xterm.c` → **built**；`--check emacs/src/xterm.c femacs/src/xterm.rs` = **443 条 `- `**（本片之后）；
  链接棘轮重生成：**387 undefined / 387 行 / 0 failures**——`unknown` 类从 79 涨到 **97**，多出来的正是这一片承载的 **X11 专属符号**
  （它们在 X11 构建里由 X11 库解析；冻结 target 从不链接它们），这条已写进 `docs/ports/link.md` 的口径里。

- **3.250 产品名声明这条长期指令的复算，与门的一个真洞：`femacs/platform` 只按"名字"被扫过（2026-10-04）**：
  ① **指令的读法不变**（3.226 ①②）：`版本声明不要再说是gnu emacs了` 与 38 秒之后的 `把之前的代码中的这种声明也去掉` 是**同一条指令的前后两半**，
  不只对新写的文件生效，**已写进树的代码**也要清；三段处置（231 个首行版本括注压到只剩修订号、268 个版权/许可块不抄、标题注释与"复述许可块"的句子进 `title`/`recital` 两类）
  一处没退。本轮在 HEAD `2241507` 的工作树上复算，且把这个复算的**扫描面**从"门读的那些树"扩到全仓我们自己写的文件。
  ② **当轮实测**：`python3 tools/check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`；
  `--selftest` **28 → 33 例 / 0 失败**（新增 5 例走的是**别的语言**的形状：Swift 注释里的声明块 → `decl`+`title`、Swift 里引 C 的 docstring 字面量 → `literal`、
  Kotlin 的出处行只留修订号 → 无、C++ 标题注释 → `title`、Swift 注释里的许可复述 → `recital`）；`docs/ports/*.md` 台账零命中（600 份）；
  逐字树 `check-verbatim.py` → `6 trees, 4607 files, 0 failures`。定向扫描面**扩到 1,373 个文件**（全仓减去参照 `emacs/`、六棵逐字树、`build/` 与 `femacs/target/`，
  后缀含 `rs/swift/py/sh/md/toml/json/yml/c`）后，被移植树上的命中仍然**只有那 13 处字面量**，其余命中全在四处"许可注解的家"里
  （`PORTING.md` 的规则正文、`docs/baseline/**` 的实测值与允许清单、`docs/status.md`+`docs/reviews` 的决策叙述、`tools/check-declarations.py` 自己的模式表）——**3.226 之后落的单元零命中**。
  ③ **门的一个真洞（本轮要改的就是它）**：`findings()` 走的是 `(root / base).rglob("*.rs")`，而 `SCAN_DIRS` 里的 `femacs/platform` **一个 `.rs` 都没有**——
  那里是 7 个 `.swift`（`src/*.m` 的 Swift 目标）。也就是说：docstring 写着"平台树也被扫"，实际**一个文件都没读**；而 `src/*.m` 每个文件开头恰好有 4 处产品名
  （`nsterm.m` 的 6 / 8 / 13 / 19 行就是那个 FSF 声明块），平台单元按定义又要把 C 的**注释**转写进 Swift ⇒ 这条用户指令**在最该被绑的树上没有守卫**。
  **修法**：读什么由**后缀**决定（`SRC_SUFFIXES` = `rs/swift/kt/kts/java/cpp/cc/cxx/m/mm/h/hpp/c`），并每次打印一条覆盖读数——
  `read 630 files (623 .rs 7 .swift)` 与 `walked past 1 file(s) of an unscanned kind: femacs/xtask/Cargo.toml`——**让"某个被点名的树一个文件都没读"变成读数，而不是沉默**。
  `//` 与 `/* */` 在各语言里同形，扫描器本来就是按语法走的（raw 串、`c"` 串都已建模），所以四类判据原样搬过去，没有新机制。
  ④ **正对照（洞是真的，且已合上）**：把两行 FSF 声明块写进 `femacs/platform/macos/__probe.swift`，**HEAD 的门报 `13 sites, 0 failures`——探针完全不可见**；
  本门的门报 `FAIL …: decl x1` 与 `FAIL …: title x1`。把探针换成五类全带（版本括注 + 声明块 + 标题注释 + 复述 + 字面量）→
  `19 declaration sites (5 forbidden, 14 allowed literals), 6 failures`（`version`/`decl`/`title` x2/`recital`/`literal` 各一条），删掉探针回到 `13 / 0 / 0`。
  **这次改动的实际效果 = 平台的 7 个 Swift 单元第一次被读**；读数是**零新增行**（它们今天是干净的），门只是从此看得见——所以允许清单一行没动。
  ⑤ **边界与残余（沿用 3.226 ④⑤，本轮拿到更硬的一条证据）**：我们自己写的散文可以**提到**参照（基线行、"femacs 一比一复刻参照检出 `fd7afced403`"、根 `README.md` 的许可句），
  **但任何文件都不许声明自己是该产品**。13 处字面量继续留在允许清单：运行时那行产品名是**逐字树**的 `femacs/lisp/version.el:111`
  （`(format "GNU Emacs %s (build %s, %s%s%s%s)%s" …)`）拼出来的，本轮实测该文件与参照 `emacs/lisp/version.el` **`cmp` 逐字节相同**、`check-verbatim` 钉着它 ⇒
  改 C 侧那几条字面量既**减不掉**程序里出现的产品名，又会同时破坏 1:1 与 docstring 门。产品身份层面要不要改口径，是**全面验收期**的判决（用户长期指令：前期复刻期间与 C 尽量一致）。
  ⑥ 读数：门 `13 / 0 / 0`、`read 630 files (623 .rs 7 .swift)`、`--selftest 33 例 0 失败`；`check-verbatim` 6/4607/0；
  改动面 `tools/check-declarations.py` 一个文件（`SCAN_DIRS` 的走法改成按 `SRC_SUFFIXES` 取文件、每次打印覆盖读数、5 条跨语言自测例），**允许清单零改动**。

- **3.251 第一次真链接：C 自己的库行、四类"被发明出来的外部符号"，与 84 个剩余（2026-10-04）**：
  ① **配方取自 C 自己**：`build/emacs-ref/src/Makefile` 的 `temacs` 规则，`make -n temacs` 展开到第 457 行——
  `gcc -std=gnu23 -o temacs.tmp <ALLOBJS> ../lib/libgnu.a -lncurses`，之后 `make-fingerprint` 与 `codesign -s - -f`。
  冻结配置里 `LIBXMENU`/`LIBX_BASE`/`PGTK_LIBS`/`LIBS_GNUSTEP`/`LIBS_SYSTEM`/`EMACSRES` **全空** ⇒ 整个程序的外部库只有 **`-lncurses`**。
  ② **读数（163 个对象，与 `check-link.py` 同一集合）**：`cc -o /tmp/femacs-link <objs>` + 隐式 `-lSystem` ⇒ **118 undefined**；加 C 自己的 `-lncurses`
  ⇒ **106**；再加 Rust 运行时的 `libcore`/`libpanic_abort`/`libcompiler_builtins`（nightly-2026-10-01 的 `target-libdir`）⇒ **88**；
  修掉下面两条 ⇒ **84**。这一级之所以要有，是因为此前每个门都停在对象或文本上（`build-tu` 一个对象、`layout-check` 探针、`check-undef` 每单元的未定义表、
  `check-swift-imports` 只管 Swift 侧）。
  ③ **链接级门抓到的四类"发明的符号"（文本门在结构上看不见）**：`_fflush_unlocked`/`_fputc_unlocked`/`_fwrite_unlocked` —— `src/sysdep.c` 2914-2943 写的是这三个名字，
  但本 target 上它们是 `lib/unlocked-io.h` 的宏（映射到 `fflush`/`fputc`/`fwrite`；镜像 `lib/include/unlocked-io.rs` 早有实测活侧臂），
  而本 target 的 `<stdio.h>` 一个都不声明（实测：把 `<stdio.h>` 包进去再调用它们**编译不过**；`nm -u build/emacs-ref/src/sysdep.o` 报的是 `_fflush`/`_fputc`/`_fwrite`）；
  `sysdep.rs` 当年把宏写成了 `extern` 声明，于是**文本与 C 逐字相同**、没有文本门能看出差别。修法：`include/sysstdio.rs` 补上 C 29 的 `#include <unlocked-io.h>`
  （batch L 的镜像已存在，此前 mirror 注释还写着"尚无镜像"），四个调用点改用镜像的宏，删掉三条声明。
  ④ 第二个：`_VFORK` —— `src/callproc.c` 1534 的 `VFORK ()` 是 `conf_post.h` 280-291 的宏，`DARWIN_OS` 活侧体是 `vfork ()`（`nm -u build/emacs-ref/src/callproc.o` 报 `_vfork`），
  而 `callproc.rs` 把它声明成了函数 `VFORK`。修法：调用点 `VFORK! ()`（`include/conf_post.rs` 的宏）+ `<unistd.h>` 自己的 `vfork` 声明（照 `sysdep.rs` 的既成形状），删掉发明的声明。
  验证：`build-tu src/{sysdep,callproc}.c` 均 built、`extract-abi --check` 两单元均 OK、`check-declarations` 仍 13/0/0、`check-mirror-dupes` 仍 1 dup/1 棘轮/0 failure，链接的未定义表少了这四个名字。
  ⑤ **剩下 84 的构成**：约 62 是 `src/xdisp.c` 的余量（正在复刻，`docs/ports/xdisp.md`）；**其中多数被分类器记成 `unknown` 而不是 `port-owed`，原因是一条要修的测量**——
  `build/emacs-ref/src/xdisp.o` **从未建过**，参照定义集合里根本没有 xdisp；19 个 `V…` 载体 + `_current_buffer`/`_specpdl`/`_specpdl_ptr`/`_specpdl_end` + `_syms_of_xwidget`
  是已记录的偏离与未复刻单元（决定，不是新发现）；最后 2 个 `__RNvCs8kYXCe6bXIp_7___rustc17rust_begin_unwind` 与 `_rust_eh_personality` 是**驱动者的账**：
  `--emit obj` 的单元 crate 没有 `#[panic_handler]`，最终二进制由谁来驱动（rustc 当驱动器，或一个持有 panic handler 的根 crate）是这一级留下的最后一个问题——
  它也是 `xdisp` 之外，`emacs` 可执行文件出现前的最后一步。读数与命令都写进 `docs/ports/link.md` 的 reading three。
  ⑥ **同轮把⑤里点名的测量补上了**：`make xdisp.o`（在 `build/emacs-ref/src`，几秒钟）建出参照的 xdisp 对象，于是同一张 84 的清单分成 **`port-owed` 15 / `unknown` 67 / `rust-runtime` 2**
  ——15 个是 xdisp 的**导出**定义（`disp_char_vector`/`init_xdisp`/`count_lines`/`mark_window_display_accurate`/`note_mouse_highlight`…），67 个里约 47 个是 xdisp 的 **`static`**（`nm -g` 看不见；全符号一趟读作 28 个 `port-owed`），
  余下是 C 的**生成全局量**（`hscroll_margin`/`inhibit_message`/`automatic_hscrolling`/`V…` 族/`current_buffer`/`specpdl*`，分别被我们的 `xdisp.o`/`indent.o`/`term.o`/`alloc.o`/`bidi.o`/`window.o`/`dispnew.o` 引用）与未复刻的 `syms_of_xwidget`。
  这一步还量出一条**结构性**事实：C **根本没有** `_hscroll_margin` 这个符号——`build/emacs-ref/src/globals.h` 893 是 `#define hscroll_margin globals.f_hscroll_margin`，载体是 `src/alloc.c` 278 的 `struct emacs_globals globals;`；
  而本端口的镜像把它们**摊平**成 `pub static mut hscroll_margin: EMACS_INT;`（`src/include/globals.rs` 7015），`check-symbols.py` 记的 467 个"折算进 Rust statics"的名字因此**没有任何定义点**，跨单元引用必然未定义。
  两条出路（都只做一次决定，不是 467 个小决定）：复刻 C 自己的载体（`struct emacs_globals globals;` + 生成字段表，正是 C 的文本，且字段布局可被 layout 门检查），或保留摊平名字、在**单一**位置给它们存储（`femacs/src/abi_shim.rs` 已经就是"C-ABI 里没人定义的名字"的落点，3.248）。验证：把参照 `xdisp.o` 移开再放回，`check-symbols`（86 units / 1 mismatched，即既有的 `lib/regex` 行）与 `check-undef` 读数**不变**。

- **3.252 第一条"死 text 拥有、活 text 读取"的生成全局量：`Vcomp_eln_to_el_h` 有了存储，链接那一级也变成门（2026-10-04）**：
  ① **把 3.251 剩下的那一家族量清了**：真链接余下 84 里那一族**不是**结构缺口。逐一核过——**44 个属于 `src/xdisp.c`**（DEFVAR 是活 text，随该单元落地时由属主定义，`callint.rs`/`casetab.rs` 的 3.29 形状）；
  **`current_buffer`/`specpdl`/`specpdl_ptr`/`specpdl_end`** 是 C 的**线程宏**（`thread.h` 131/139/147，例如 `#define specpdl (current_thread->m_specpdl)`），镜像把存储当**临时**措施声明并在原地说清了，落点是 `thread.rs`；
  **真正缺的只有一个** `Vcomp_eln_to_el_h`，而且缺的是**存活性**而不是结构：C 5766 的 `DEFVAR_LISP` 在 `HAVE_NATIVE_COMP`（本 target undef）里是死 text，但 C 的**存储**是生成结构体的字段、**在每种配置里都存在**，
  而 `src/lread.c` 1019（`compute_found_effective`，用预处理器的括号栈实测**外层没有任何 `#if`**）读它。修法：`femacs/src/comp.rs` 保留这条存储
  （`#[unsafe(no_mangle)] pub static mut Vcomp_eln_to_el_h: Lisp_Object = lisp_h_Qnil ();`），DEFVAR 行继续按死 text 记录在 `docs/ports/comp.md`——即"**存储跟着生成结构体走（无条件）、只有注册跟着冻结构建的话 text 走**"。
  验证：`build-tu src/comp.c` built；`--check comp.c` OK；`nm -g build/femacs/comp.o` 出现 `_Vcomp_eln_to_el_h`；`check-globals` 仍 284 units/0 problems；`check-symbols` 仍 86/1（既有的 `lib/regex`）；**可执行链接 84 → 83，该名字消失**。
  ② **链接那一级升格为门**：`tools/check-link.py` 挂进 `xtask gate-fast`（在符号门与 linkage 门之后），理由是它找到的那一类——"被引用、C 有定义、本端口无人定义"——在每一级下层门里都不可见（`build-tu` 停在一个对象、`check-undef.py` 只读单元自己的未定义表、`check-linkage.py` 问的是相反方向）。
  棘轮按两处修正重生成：**383 rows / 0 failures**；`cargo build -p xtask` exit 0。`docs/ports/link.md` 里 3.251 那段"结构性缺口/一次决定"的说法已就地更正（新增 "Reading three, corrected" 一节，指明被更正的两句与理由）。

- **3.253 可执行那一级落地：Rust 运行时缝合上、C 自己的库行由 makefile 读出、余量只剩 71 个名字（2026-10-04）**：
  ① **`tools/check-link.py` 加了 `--executable`**：它不合并对象，而是**真的链接出程序**——部分链接那 163 个对象，加两样没有任何单元拥有的东西，两样都从"权威处"取而不是写死在工具里：
  第一是 **Rust 运行时缝**（单元是 `--crate-type lib --emit obj` 编的，所以没有单元持有 `#[panic_handler]`，链接欠 `__rust_begin_unwind` 与 `_rust_eh_personality`）；工具把一段四行 `#![no_std]` crate 落到 `build/femacs/rust-runtime-root.rs`，
  **在 `femacs/` 下**调 `rustc`（这样 `rust-toolchain.toml` 决定用哪个 rustc，P-04 的钉子），再带上该 toolchain `target-libdir` 的 `libcore`/`compiler_builtins`。
  第二是 **C 自己的库行**：`make -n temacs`（在 `build/emacs-ref/src`）打印 C 的配方，工具读它的 `-L`/`-l`/`-framework`——冻结配置里 `LIBXMENU`/`LIBX_BASE`/`PGTK_LIBS`/`LIBS_GNUSTEP`/`LIBS_SYSTEM`/`EMACSRES` 全空，所以集合就是十六个 Homebrew `-L` 目录加 `-lncurses`。
  ② **读数（reading four）**：`--executable` → **71 个未定义**（`port-owed` 10 / `unknown` 61），棘轮 `docs/baseline/link-exec-pending.txt` **71 行 / 0 failures**；两类就是 3.251/3.252 那两类减去两个运行时名字：`src/xdisp.c` 的余量（活的 `DEFVAR_*` 与它的 `static`，随该单元落地）、
  `current_buffer`/`specpdl*`（已记录的临时偏离，落点 `thread.rs`）、`_syms_of_xwidget`（未复刻单元）。**即"一个 emacs 可执行文件"是差一个单元的距离，而不是差基础设施**——链接驱动器、运行时缝、C 的库行都就位并量过了。
  ③ **两条口径（都实测过）**：两队棘轮都在**含在飞单元对象**的工作树上取的，所以净 HEAD 构建可能出现"棘轮缺的新名字"（补行）或"不再需要的旧行"（删行）——这就是棘轮常规churn，落点应是 HEAD 的 `git worktree`（也是跑 `gate-fast` 的地方）；
  `--executable` **暂不挂进 `gate-fast`**（部分链接那一级已挂，见 3.252），理由是它应当先在净 HEAD 构建上量一次，免得棘轮变成每轮churn。`check-link.py` 的两处工具 bug 也是本轮实测改掉的：C 库行的续行要**先拼接再取 token**（`replace("\\\n", " ")`），以及运行时 rlibs 当初漏进命令行（`libs` 变量名被复用的笔误）。
  ④ **棘轮口径（同轮实测后定）**：棘轮行"不再成立"= 棘轮在**收缩**，而在飞单元的对象恰好造成这种收缩——`src/xdisp.rs` 在一小时内又落了十个定义，
  于是当轮读数是 `373 undefined / 383 行棘轮 / 1 failure / 11 stale`（那一个 failure 是同一 lane 在飞切片里的新引用 `_redisplay_window`）。因此 `check-link.py` 两种模式都**只对"新名字"失败**，
  stale 行只在 `--strict` 下算失败；`--strict` 是**净 HEAD worktree** 里对账用的模式（那里文件必须精确），`gate-fast` 用默认模式，于是五条 lane 不停重建对象时快速门不会红。

- **3.254 两队棘轮都改成"在 HEAD 对账"，工具修正与 `--strict` 后置条件（2026-10-04）**：
  ① **为什么要有这一步**：两队棘轮都是 `build/femacs/*.o` 的 `nm` 读数，而仓里**永远有五条 lane 的对象在飞**——正在复刻的单元会先引用、后定义。在工作树上重生成棘轮就等于把 lane 的半成品写进一个**之后每个门都要比**的文件；
  能算数的读数是 **HEAD 上**的。新脚本 `tools/reconcile-link-ratchets.sh` 就是那个测量：临时 worktree 挂 HEAD、把 `emacs/` 与 `build/emacs-ref/`（两者都不被 git 跟踪，worktree 里没有）符号链接进去、抄进当前对象、**只**把工作树在飞的那些单元按 **HEAD 自己的源码**重建，然后写两队棘轮并用 `--strict` **复检**（后置条件：文件对 HEAD 的对象必须**精确**）。
  ② **读数五（HEAD `ee92417`，对账后）**：部分链接 **365 个名字**、可执行链接 **63 个名字**（`port-owed` 15 / `unknown` 48）。同一时刻工作树上的两个门读作 373 与 71，其中 **14 个名字只在在飞的 `src/xdisp.rs` 里被引用**（`_display_line`/`_redisplay_window`/`_set_cursor_from_row`/`_hscroll_margin`/`_Voverlay_arrow_*` 等，都是 xdisp 自己后续切片要定义的名字）——两个读数之差就是 lane 的进度，没有别的。
  ③ **工具修正（同一轮实测抓到）**：`write_exec_pending` 的写文件循环被一次按行替换误删，于是 `--executable --write-pending` **只写了注释头、没有行**，而汇总行仍报 "wrote N row(s)"——`ee92417` 里落库的那份 `link-exec-pending.txt` 因此是**空棘轮**（10 行注释、0 行数据），在净 HEAD 上表现为"63 个名字全是新的"。修正已入本轮提交；教训写进脚本的注释：**写后必须用 `--strict` 复检**，否则"写成功"这句话本身没有证据。
  ④ 现状：`tools/check-link.py`（部分链接那级）仍挂在 `gate-fast`，`--executable` 仍不挂——现在两队棘轮都精确了，等 `xdisp` 落地后把可执行那级也接上就是一个两行改动。
  ⑤ **为什么可执行那级不挂进 `gate-fast`（本轮把理由量清）**：可执行链接剩下的未定义名字是部分链接那一级的**子集**（部分链接里没人定义的名字，除非被库或运行时提供，否则在可执行级仍无人定义），所以"名字类"的回归由已挂进 `gate-fast` 的部分链接门覆盖；
  可执行那级的价值在别处——它是**收官时的验收命令**（`--executable` 绿即"程序存在"），以及每轮读一次就能拿到"离可执行还有多远"的**进度读数**（本轮 63）。脚本已端到端自测：在本轮 HEAD 上重跑 `tools/reconcile-link-ratchets.sh` → 两级 `--strict` 全绿、两队棘轮**零 diff**、worktree 自动清除。

- **3.255 两条 lane 交付入库；Swift 缝的两条裁决落定（宏条目 B→A、`frame_set_double_buffered` 带门导出），并把 Swift 两队棘轮一起纳入 HEAD 对账（2026-10-04）**：
  ① **入库**：shim 批③④（10 条 `frame_*`，shim 共 **28** 条、`nm -g build/femacs/abi_shim.o` 共 52 个导出）与 `nsterm` 第 9 片（`nsterm.swift` 2,109 行；台账 119 勾 / 108 未勾，起点 C 1959）。共享表按门的读数结清：`swift-import-pending.txt` 删 20 行（shim 18 + nsterm 2，正是门报的 `STALE (satisfied)` 全名单），`swift-units.tsv` 删 6 行；删后 `check-swift-units` = `0 new, 0 stale, 7 Swift unit(s) landed`。
  ② **裁决一（Swift 缝上的"宏条目"）**：两条 lane 都来问同一个问题——Swift 侧把 C 的**宏名**当 C-ABI 条目 `@_silgen_name` 导进来（`NILP`/`FRAME_NS_VIEW`/`XSETFRAME`…），是"核心导出同名 shim"（A）还是"宏必须展开"（B）？**裁定：先 B 后 A，判据是"展开式在 Swift 侧能不能照抄"**。
     * **B（首选）**：宏体在 Swift 里能写成算式/成员读的（纯 AppKit 表达式、算术、布局已被 Swift 声明过的记录的成员读）**必须展开在用点**——不给冻结构建造一个 C 里不存在的符号。本轮 `nsterm` 的 `FRAME_NS_TITLEBAR_HEIGHT`/`FRAME_TOOLBAR_HEIGHT` 是样本（展开后 `nm -u` 里那两个名字归零）；3.251 的 `fflush_unlocked` 是同一条规则的同语言形态。
     * **A（例外，且是必要的）**：展开式需要**只有核心知道的事实**时（`Lisp_Object` 的谓词/存取器、`globals.h` 的生成条目、枚举器、跨记录的 INLINE），核心**导出同名 C-ABI 条目**，落点 `femacs/src/abi_shim.rs`，理由进 `docs/ports/abi_shim.md`，棘轮行留在 `swift-import-pending.txt` 直到导出为止。根据是语言边界：C 的 `nsterm.m` 能直接用宏是因为它就在 C 里；Swift 要过 C-ABI 缝，而 `NILP(x)` 的语义（`Qnil` 取什么值）属于核心。
     * 两条不是口味之争：**能展开不展开 = 给冻结记录造符号（P-08）**；**不能展开硬展开 = 让 Swift 复写核心的内部表示（P-72）**。宏观读数：本条 lane 现有的宏名条目在参照对象里**真符号数为 0**（lane 用 `nm -g build/emacs-ref/{src,lib}/*.o` 逐个查过 24 个名字），所以它们全部属于 A 类，是"核心欠的条目"而不是缺陷。
  ③ **裁决二（`frame_set_double_buffered` 扣下的那一条）**：该成员只在 `NS_IMPL_COCOA && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`（nsterm.h 1052）下存在，冻结配置里两个宏都 undef（macros.tsv 1481/2996/2997）。**裁定：带同一个门导出**（`#[cfg (feature = "ns_impl_cocoa")]`），棘轮行保留、理由写成"只有 Cocoa 构建能兑现"——与 `macfont`/`nsfont` 那几条永久死臂行同类。拒绝另两条：不带门导出＝给冻结记录里不存在的成员造宿主（P-08）；挂到 NS 构建那一轮＝把一条**已经量清**的账推给将来，而 Swift 侧本轮已把 Cocoa 臂当目标文本承载，符号与成员在同一个门上才是自洽形状。
  ④ **对账脚本扩到四队棘轮**：`tools/reconcile-link-ratchets.sh` 现在除了两队链接棘轮，还在同一个 HEAD worktree 里重生成 `swift-units.tsv`（`--write-manifest`）与 `swift-import-pending.txt`（`--write-pending`）——理由与链接那两队相同：Swift 棘轮读的是**被跟踪的 Swift 文本 + 核心导出表**，而后者来自同一批对象，在飞单元一样会把它染偏。本轮读数（HEAD）：`link-pending` **365**、`link-exec-pending` **63**、`swift-import-pending` **549 行**、`swift-units.tsv` 116 行；工作树上同一时刻 `check-swift-units` 报 6 条 stale（`nsfns` 在飞批⑤落的定义）、`check-swift-imports` 的多余行同样来自在飞 Swift，属已知 churn。
  ⑤ **交接状态**：`nsterm` lane 连做 9 轮后自述上下文耗尽 ⇒ 需换新会话（起点 C 1959 `ns_set_undecorated`，其后 2043 / 2120-2168 / 2169 / 2191，并在 **C 2366** 收口第二条函数型待定条目 `ns_set_frame_alpha`）；shim lane 已完结（批⑤的清单变成"棘轮里 A 类的行"）。

- **3.256 验收基线建成：参照程序 `emacs` + `emacs.pdmp` 真的跑起来了；Lisp 那一半的两条弯路与"参照树保持原样"的实测（2026-10-04）**：
  ① **结果**：`make -C build/emacs-ref -j3`（顶层驱动器）把 `src → lisp → dump` 走完，得到 `build/emacs-ref/src/emacs` 与 **13 MB 的 `emacs.pdmp`**；参照侧 L0 读数：`emacs --version` → `GNU Emacs 32.0.50 / Development version fd7afced4036 on master branch; build date 2026-10-04`（版本串里就是冻结修订本身），
  `emacs -Q --batch --eval '(princ (+ 1 2))'` → **3**，`(princ (mapcar (lambda (x) (* x x)) (list 1 2 3)))` → **(1 4 9)**。冻结配置是 `--without-all --with-x=no --with-ns=no`，这解释了 3.251 量到的"外部库只有 `-lncurses`、不编译任何 Objective-C"。
  ② **`temacs` 单跑不够，两次弯路都量清了**：没有 dump 的 Lisp 时，连 `temacs -batch --eval '(princ (+ 1 2))'` 都报 `Symbol's function definition is void: debug-early--handler`；直接驱动 `make -C build/emacs-ref/lisp EMACS=../src/bootstrap-emacs`（temacs 的副本，Emacs 自己 bootstrap 的做法）推进到 `main-first`/`semantic` 也失败——它算出的 load-path（`.../lisp/lisp`）是**顶层驱动器**才设对的，而生成 `.el` 的规则要的是**已 dump 的程序**。
  ③ **顶层驱动器要重跑 `configure`**：`./config.status --recheck` 会停在 *"source tree does not seem to have pre-built manuals in the 'info' directory. Please install a suitable version of makeinfo."*——`emacs/configure` 14231-14235 用 `$srcdir/info/emacs` 区分"发行 tarball"与"仓库检出"，本检出无 `info/`；**零字节的 `emacs/info/emacs` 就能让它过关**，随后构建自己找到配置里的 `MAKEINFO`（`makeinfo (GNU texinfo) 7.1`）。
  ④ **参照树保持原样的实测（本轮最值得记的一条）**：构建把 **git 忽略**的产物写进**参照树**（实测 1,878 个文件：1,646 个 `.elc`、`info/*.info`、生成的语法 `.el`、stamp）。`tools/check-verbatim.py` 比的是**文件集合**，所以清理过宽会被它抓住：`git -C emacs clean -fX` 除这些产物外还删掉了**构建之前就存在**、而端口的逐字副本镜像着的忽略文件（`etc/refcards/emacsver.tex`、`doc/emacs/emacsver.texi`、`doc/man/emacs.1`），门从 0 变成 3 failures；按门报的路径**逐条按正确方向回拷**（`lisp/leim/ja-dic/ja-dic.el{c,}` 是反方向：由参照拷进端口）后回到 **`6 trees, 4609 files, 0 failures`**。
  ⑤ **同一清理也删掉了 `emacs/configure` 与 `emacs/aclocal.m4`**（同样是忽略的生成文件）。**按设计不再补**：端口测量路径不需要它们（冻结构建基线是 `docs/baseline/config/darwin-arm64.macros.tsv` + `build/emacs-ref/**`），而用另一个 `autoconf` 重新生成会有碰到**被跟踪**文件（`src/config.in`）的风险。经过这一切 `git -C emacs status --short` 始终为空。
  ⑥ 这份配方与两条弯路写进新文件 **`docs/acceptance.md`**（含"构建后只清构建产物、或按门报路径与端口副本重新同步"的教训）。它不在当日关键路径上，但它是 L0-L5 的前置：现在**参照侧已经可以逐条量**，端口侧还差链接（reading five：部分链接 365 个名字、可执行 63 个）。


- **3.257 shim 批⑤ 入库（15 条、11 条棘轮行自动清掉），四队棘轮再对账一次（2026-10-04）**：
  ① **入库**：`frame_name`/`frame_set_terminal`/`frame_set_no_split`/`frame_set_undecorated`/`frame_set_can_set_window_size`/`frame_set_was_invisible`/`window_contents`/`window_current_matrix`/`buffer_filename`/`terminal_id`/`terminal_display_info_ns` 十一条**无门** C-ABI 条目，
  加 `frame_alloc_ns_output`/`frame_set_fontset`/`frame_set_display_info`/`frame_set_double_buffered` 四条**带门**（`have_ns`，末条另加 `ns_impl_cocoa`，即 3.255 的裁决形状）。`femacs/src/abi_shim.rs` 现共 **43 条**条目，对象 **63 个导出**。
  位段写者用的是镜像自己的位移常量（`FRAME_NO_SPLIT_SHIFT` 14、`FRAME_UNDECORATED_SHIFT` 25、`FRAME_CAN_SET_WINDOW_SIZE_SHIFT` 28、`FRAME_WAS_INVISIBLE_SHIFT` 5），与 `frame.rs` 的读宏**同一颗位** ⇒ 读写不可能悄悄错位。
  ② **扣下一条（要 `frame.rs` 先长一个字段）**：`frame_set_ns_transparent_titlebar` 的成员 `bool_bf ns_transparent_titlebar`（frame.h 735-739，`#ifdef NS_IMPL_COCOA`）在本端口的 `struct frame` 镜像里**没有宿主**——无门导出＝造宿主（P-08），带门导出＝空体；记在台账，落成后与其余七条同形。
  ③ **lane 自报的限度（诚实且正确）**：那四条带门函数体在**任何配置下都没被编译过**——`xtask dead-arms` 的 FLIPS 只有六项、不含 `HAVE_NS`/`NS_IMPL_COCOA` ⇒ 它们是**被记录的文本**而非被验证的文本（P-51），已写进台账而不是算作成果。
  ④ **四队棘轮再对账（`tools/reconcile-link-ratchets.sh`，在同一 HEAD worktree 里）**：`swift-import-pending.txt` **549 → 538 行**（11 条 satisfied 行被自动清掉，正是 lane 要删的那 11 条），`link-pending` **365**、`link-exec-pending` **63**、`swift-units.tsv` 120 行 —— 不再需要人手改共享表，落地后跑一次脚本即可。
  ⑤ **留给下一步的三条**：⑴ `check-swift-imports.py` 里那 14 条信息量最低的理由（机器兜底桶 `not found in the reference objects, the SDK or the reference's C and Rust text - check the spelling`）应当按裁定书 class 1 归类为"核心欠的 C-ABI 条目（3.255 A）"——判据是同族名字在 `nm -g build/emacs-ref/{src,lib}/*.o` 里真符号数为 0（lane 逐名验过），而不是拼写；⑵ B 类清单（`FRAME_NS_VIEW`/`FRAME_DISPLAY_INFO`/`MOUSE_HL_INFO`/`WINDOW_TO_FRAME_PIXEL_Y`）与同形的 `FRAME_NS_WINDOW`/`FRAME_CURSOR_COLOR`/`FRAME_POINTER_TYPE`/`FRAME_FONT`：**B 的前提是导入方 Swift 文件声明该记录的布局前缀**（`nsfns`/`nsfont`/`macfont` 已有 `ns_display_info` 前缀的先例），`ns_output` 的前缀 `nsterm.swift` 还没有 ⇒ 归 Swift lane 的一个声明决定，而 `nsterm.swift` 2009-2031 那两个已展开的函数正调用 `FRAME_NS_VIEW(f)`，要一起改；⑶ 加 `ns_output_*`/`ns_output_set_*` 一族（约 20 条）与尾部 `set_waiting_for_input_flag`/`face_set_background`，`glyph_row_visible_height`/`glyph_row_y` 按 A 类（`struct glyph_row` 的布局属核心，每次访问就是一条 C-ABI 条目）。

- **3.258 棘轮理由的措辞：`unclassified` 桶按实测分成两半，并把"查拼写"这句误导话去掉（2026-10-04）**：
  ① **触发**：3.257 ⑤ 记的 lane 反馈——`check-swift-imports.py` 的兜底理由 `not found in the reference objects, the SDK or the reference's C and Rust text - check the spelling` 挂在一批**拼写完全正确**的名字上（它们是 lane 给记录成员/宏起的 C-ABI 拼法）。
  ② **实测这一桶里到底是什么**（在 538 行棘轮上逐名跑 `classify`）：`reference-source` 347、`unclassified` **82**、`reference` 105、`rust-side` 4。这 82 行再分两半：**18 行是成员存取器**（名字的尾部是参照 C 文本里的标识符，如 `frame_set_size_hint_flags`→`size_hint_flags`、`glyph_row_y`→`y`… 由新加的 `tail_evidence()` 查证，理由是**被查到的那条事实**"`<tail>` 是 `<file>` 里的标识符"）；**其余是 Objective-C 面**（`EmacsFileDelegate_new`、`EmacsImage_allocInitFromFile`、`EmacsView_initFrameFromEmacs`、`GSFontEnumerator_*`、`NSColor_colorWithUnsignedLong`）——**另一个 Swift 单元自有**的类/选择子，而本工具的四个论域（参照对象、SDK、参照 C 文本、Rust item 名）按构造就看不见它们。
  ③ **改动**：`reason()` 的兜底分成两句，第一句给查到的证据（尾标识符 + 文件），并写明两种出路（3.255 的 A：核心导出同名条目 / B：该 Swift 单元在用点展开），第二句才说"要么是只有 lane 知道名字的 C-ABI 拼法、要么是要修的拼写"——**不再暗示拼写错误**；`main()` 的 review 注释把两半写清（核债务 / ObjC 名字归属 / 拼写）。**类别与计数没动**（`classify` 不加新论域，避免每次跑都多扫一遍 C 文本），语义只在**棘轮行里的理由**。
  ④ **一个过程教训（本轮第二次踩）**：对账脚本是在 **HEAD 的 worktree** 里重生成棘轮的，而工具的改动**还没提交**时，worktree 里跑的是旧工具 ⇒ 第一次重生成写出的理由仍是旧文本（文件零 diff 即证据）。正确顺序是**先提交工具、再对账**——与 3.254 ⑥ 记的 `write_exec_pending` 那次是同一类错误（"写成功"必须用后置条件验）。

- **3.259 nsterm 第 10 片入库（C 1959-2398，15 个定义，第二条函数型待定条目收口）；四队棘轮按新 HEAD 重生成（2026-10-04/05）**：
  ① **入库**：`nsterm.swift` **2,109 → 2,703 行**、台账 1,229 → 1,455（函数框 51/145 → **66/149**、宏 16/23 → **17/23**）；`swiftc -parse-as-library -c` exit 0（新增 3 条 warning 是 C 自己的 `scanFloat` 弃用，ns_get_color 的 C 2257-2259）。
  **第二条函数型待定条目 `ns_set_frame_alpha` 收口**：`nm -g` 有 ` T _ns_set_frame_alpha`、`nm -u` 里计数为 0（台账 `## ABI` (4) 表据此标 landed）；本片新增 11 个 C 导出。
  ② **lane 自报并量清的两件事**：⑴ `tools/cinventory.py` 的 `function_spans` **8 行 lookahead 窗口**让 4 个函数（`ns_set_undecorated`/`ns_set_parent_frame`/`ns_set_no_accept_focus`/`ns_set_z_group`，文档注释 ≥8 行）**永久不进出清单** —— 台账用 4 个手补框承载，**manifest 不得为它们加行**（重生成会把它们丢掉）；这是**工具缺陷**，记进我的队列（与 `emacs.c` 的 `main` 那次同类，修法是放宽窗口并对全参照树量差异）。⑵ `Emacs_Color` 布局实测：C `sizeof 16 align 8 pixel 0 red 8 green 10 blue 12`，Swift `size 14 stride 16 align 8` 且四个 offset 相同（`stride` = C 的 `sizeof`）。
  ③ **四队棘轮按新 HEAD 重生成**（`tools/reconcile-link-ratchets.sh`）：`swift-units.tsv` **−11 行**（r10 落地的定义离开清单，且**没有**为 4 个 generator-gap 名字加行），`swift-import-pending.txt` **549 行**（195 行变动 = 3.258 的新理由 + 本片新导入），`link-pending` **365**、`link-exec-pending` **63** 不变。
  ④ **交接**：本片欠 shim lane **21 个 A 类条目**（6 个 `frame_set_*` 成员写 + `frame_alpha` + 枚举器/全局量等，名单已交给 Dirac）；`nsfns` 批⑤ 与 `xdisp`/`xterm` 仍各自在飞。

- **3.260 `tools/cinventory.py` 的声明体前瞻：真正的问题是"注释"，不是窗口大小——修好后全树读数不变，但清单多出 8 个名字（其中 1 个从未被列过）（2026-10-05）**：
  ① **触发**：3.259 ② 转来的 lane 发现——4 个 r10 函数（`ns_set_undecorated`/`ns_set_parent_frame`/`ns_set_no_accept_focus`/`ns_set_z_group`）**永久不进出清单**。lane 的判断是"8 行 lookahead 窗口"，实测**原因不同**：`function_spans` 在参数表结束行之后逐行找 `{`，遇到**注释行就 break**，于是 Emacs 那种
  `static void` / `name (args)` / `/* 多行文档 */` / `{` 的写法（文档注释写在**声明体之前、参数表之后**）整条被丢掉；那个 8 行**窗口只是让它看起来像窗口问题**（注释越长越容易超窗）。
  ② **修法**：前瞻里**跳过注释行**（`/*`、`*`、`*/`、`//`），并把安全上限从 8 放宽到 **64 行**——**规则**是"遇到既非死行、又非空行、又非预处理行、又非注释行的那一行就停，`{` 在那行即为定义"，窗口只是界。
  ③ **实测（全树，两步都量）**：`emacs/src/nsterm.m` 的函数 **142 → 150**（r10 那 4 个现在在列 ✓）；**`tools/extract-abi.py --all` 全树读数不变**：`checked 590 ported files: 588 done, 2 in progress, 0 failing`——即这 8 个名字**早就被端口写出来了**，盲区在**清单**而不在端口（这正是"清单完整性"这一级证据原本的漏洞）。
  ④ **残留一个真缺口**：修好后 `check-swift-units.py` 报 **1 new**：`ns_get_shifted_character`（nsterm.m，**既没有 Swift func 也没有 manifest 行**）——它是盲区里唯一"从未被列过、也确实没落地"的定义；下一次对账会把它写进 `swift-units.tsv`，此后它就是 nsterm lane 的正常清单项。
  ⑤ **口径**：这是**工具正确性**修正（不是口径变化），经全树读数前后一致确认无副作用；`--all` 的两次读数（修前 588/2/0、修后 588/2/0）就是这条断言的证据。

- **3.261 L0 的验收机械落地：19 条自检表达式的 golden + 一条命令的对拍（2026-10-05）**：
  ① **`tools/acceptance-l0.py`**：`--golden <参照二进制>` 记录 golden、`--check <二进制>` 对拍；每条表达式以 `<binary> -Q --batch --eval <prog>` 跑，比较的三元组是 **(退出码, stdout, stderr)**，逐字节——正是 `docs/port-plan.md` 7.1 的 L0 口径。19 条覆盖：算术（含 bignum 路径）、`prin1`/`princ`/`format`、闭包与 `mapcar`、`condition-case`、`unwind-protect`、字符串改写与 `string-bytes`、`sort`/`append`、哈希表、`read`/`prin1` 往返、`char-to-string`、`with-temp-buffer` 往返、走向 stderr 的 `message`、`error-message-string`，以及 `(kill-emacs 3)`（**退出码本身进比较**）。
  ② **golden**：`docs/baseline/l0-reference.txt` 记着参照二进制的路径、`--version` 首行与前 16 位 sha256（参照移动过就看得见，而不是静默），每条流的字节夹在显式的 `--- stdout`/`--- end stdout` 标记之间（不必猜最后一个换行属谁）。
  ③ **读数**：参照全过——`19 expression(s), 0 failure(s)`（这是对拍机械**自己**的自检：它对自己的 golden）；**负对照**是没有 dump 的 `build/emacs-ref/src/temacs` → `19/19 failure(s)`（它打印的正是 acceptance.md 记过的那条 "Lisp directory … No such file or directory"），说明对拍真的能分辨。端口侧还没有二进制（reading five），所以 check 侧暂无对象；**端口一链接出来，L0 就是一条命令**。
  ④ 表达式**刻意选成不依赖路径**：任何会打印 build/data 目录的东西属于后面的 rung（那里端口与参照本来就该不同，比较要先归一化）。

- **3.262 把 `have_ns` 臂真的翻过来编译一次：翻牌抓到 11 处裸 `c_float`，剩下的 9 处是"平台目标解析"的账；翻牌本身等 AppKit 镜像（2026-10-05）**：
  ① **动机**：3.257 ③ 记的 lane 自述——`#[cfg (feature = "have_ns")]` 的那些体（`include/nsterm.rs` 的 NS 区、`abi_shim.rs` 的四个带门导出）**任何配置都没编译过**，`xtask dead-arms` 的 FLIPS 里没有 `HAVE_NS`/`NS_IMPL_COCOA`（P-51 的"承载文本 ≠ 编译它"）。
  ② **实验**：给 FLIPS 加两条——`("have_ns", "HAVE_NS", "true")` 与 `("ns_impl_cocoa", "", "true")`（空 key = 只翻 feature，没有配置常量可翻，循环里加一个 `continue`；数组长度 6 → 8）。跑 `dead-arms`：**在第 28 个单元 `src/abi_shim.rs` 处失败**，14 条 E0425 全部是"找不到类型"。
  ③ **第一类是可修的、且是本轮真抓到的缺陷**：**11 处裸 `c_float`**（`include/nsterm.rs` 76、288-292、885-886、953 等）——`c_float` 属 `core::ffi`，正常构建里靠单元 prelude 的 `use` 才可见，**镜子不能依赖包含者的 prelude**（3.91 ② 的成文规则）；翻牌是唯一能看见它的配置。已全部限定为 `core::ffi::c_float`，重跑后这 5 条错误消失（11 处里 5 处落在被编译的臂上）。
  ④ **第二类是"设计上就要靠平台目标"的**：剩 9 条 E0425 点名 `CGFloat`、`NSRect`、`NSString`、`NSArray`×2、`EmacsMenu`、`Window`×3——正是 `include/nsterm.rs` 50-52 自己写的"**不降级成 `*mut c_void`，因为整段在这里是惰性的：C 自己的名字照抄，平台 target 去解析它们**"。端口**没有 AppKit/GNUstep 的镜像**，所以这 9 个名字今天在任何 Rust 配置下都无法解析。因此：**翻牌回退**（数组长度回 6），并在 FLIPS 的注释里写下这次实验、那个缺陷、以及重新加回来的条件（两条 + 数组长度，等 AppKit 镜像落地）。
  ⑤ **回退后的读数**：`xtask dead-arms` → **`337 translation unit(s) compiled with the non-frozen arms`、exit 0**（门仍是绿的，且这次实验的所有事实都留在代码注释与这里）。
  ⑥ **命名下来的任务**：**一个 AppKit/GNUstep 镜像（NS 平台分片）**——它是"让这 9 个名字可解析、并让翻牌重新成立"的前置；在那之前 NS 区的文本状态是"**被承载、且经实测无法编译**"，而不是 3.257 ③ 的"未验证"。

- **3.263 承载面有多大、以及第一次把一个非平台臂真的翻绿：`glyph_debug` 现在被编译（附 5 处 "一处声明" 收敛）（2026-10-05）**：
  ① **先把承载面量出来**：新工具 `tools/feature-coverage.py`（+ 基线表 `docs/baseline/feature-coverage.tsv`）数出**树里提到 736 个 feature，其中 729 个没有任何配置打开**（列：feature / 出现点 / 文件数 / 谁打开 / 翻牌要一并移动的 `config.rs` 常量）。前几名：`windowsnt` 371 点、`dispextern_face_records` 244、`have_x_windows` 234、`have_ns` 204、`have_android` 123、`have_window_system` 107。**这就是 P-51"承载文本"的真实规模**：只有 6 个 feature 会被任何构建打开（5 个在 FLIPS，加上 `ns_impl_cocoa` 那类 feature-only 条目）。
  ② **`glyph_debug` 试探成功、并保留在 FLIPS 里**（34 个直接点，且每个单元的 GLYPH_DEBUG 臂都进编译）：过程是三步实测——加翻牌 → `src/dispnew.rs` 报 `cannot find function snprintf`（该单元的 prelude 只带 `sprintf`）→ 把 `snprintf` 落到**它正确的家** `lib/include/stdio.rs`（<stdio.h> 的系统条目镜，gnulib 的 `stdio.in.rs` 1646 只在该系统缺它时把它重定向成 `rpl_snprintf`，本 target 不缺）→ 立刻在 `src/pdumper.rs` 报 **E0659 名字歧义**（它自己在模块里又声明了一次）→ 顺着把**五处重复声明收敛**：`src/{filelock,font,pdumper,sysdep,process}.rs` 不再各自声明 `snprintf`，改为注释指向镜子（0.1-3 的"一处声明点"，`font.rs` 的原注释甚至写着"镜像有 `sprintf` 但没有 `snprintf`"）。
  ③ **读数**：`xtask dead-arms` → **`337 translation unit(s) compiled with the non-frozen arms`、exit 0**（这次是**带着** `glyph_debug` 的非冻结构建）；`check-mirror-dupes` 回到 1 dup/1 棘轮/0 failure；`extract-abi --check` 五个单元全 OK；`check-declarations` 仍 13/0/0。
  ④ **另外三个试探的结论（都回退，并记下条件）**：`dispextern_face_records` 撞到**已记录的延后项**——`struct face`/`face_cache` 被定义两次（`include/dispextern.rs` 1941 的 gated 副本 vs 单元的本地副本），而互补门正是 3.220 的裁决与 `docs/ports/frame.md` 358 记的"**等 xdisp.rs 与 xterm.rs 落地后再加**"；`have_tree_sitter` 缺 `Lisp_TS_Parser`/`Lisp_TS_Query` 的声明（树外模块的类型没有任何 Rust 侧声明）；`have_ns` 就是 3.262 的 AppKit 镜像那条。三者的失败点、报错与"重新加回来的条件"都写进 FLIPS 的注释。
  ⑤ **命名下来的新工作流**：**一个 feature 一轮**——加翻牌（feature + 其配置常量，或 feature-only）→ 跑一次 `dead-arms`（约 40 秒）→ 若失败，先判断是不是**声明/镜像**那一级的小缺口（能在一个文件里修完就修）否则回退 + 报错原文；成功的翻牌**永久保留**，于是那段"承载文本"从此进 rustc。

- **3.264 shim 批⑥⑦ 入库（55 条，文件 98 条、对象 51 个 C-ABI 名）；两条形状裁决落定；棘轮把"已导出但带门"与"欠核心"分开（2026-10-05）**：
  ① **入库**：批⑥是 `ns_output_*` **全族 42 条**（14 读 + 28 写）+ 两个 `glyph_row_*` + `set_waiting_for_input_flag` + `face_set_background`；批⑦ 9 条（`frame_set_no_accept_focus`、`frame_alpha`、模块形状的 6 条 `z_group_*`/`FLOATP`/`XFLOAT_DATA`、最后一条可落的 class 1 `frame_set_menu_bar_items_used`）。**前任"约 20 条"的估计被实测纠正**（该族棘轮是 42 行），lane 选择整族一次落完（门/棘轮处理是**一个**决定）。位段写者用镜像自己的位移（`FRAME_NO_ACCEPT_FOCUS_SHIFT`＝26），与 `frame.rs` 的读宏**同一颗位**。反事实实测：去掉那行带门 `include!("include/nsterm.rs")` → **39 × `cannot find type ns_output` + 36 × `cannot find macro FRAME_OUTPUT_DATA`** ⇒ 那行是 P-72 的必要条件而非装饰。
  ② **裁决（模块形状）**：名字**已被镜像用 Rust 拼过**时（crate 根再定义同名是 `E0428`，lane 实测），C-ABI 拼法放进模块（`c_abi_of_mirror_named_items`），体走 `super::` 调**镜像自己的项**（值的唯一来源；glob 导入会被同名项遮蔽，模块内 `super::` 是刻意为之）。**这就是 3.255 的 A 在"名字已被占"时的标准解**，先例是 `thread.rs` 的 `thread_objs` 与 `frame.rs` 的 `frame_face_accessors`；验收＝`nm -g` 出现该 C 名。
  ③ **裁决（`Q*` 族）**：**A 类、调用形状**。C 的 `Q*` 在参照对象里是**数据**符号，但端口按 P-17 把 Lisp 符号拼成内联函数（符号表地址不是 Rust 常量），而 Swift 缝需要一个 C-ABI 条目 ⇒ shim 在那个模块里导出 `QNAME()`（体调 `include/lisp.rs` 自己的项），Swift 侧按**函数**导入。**整个 `Lisp_Object` 存取器族（`NILP`/`CONSP`/`EQ`/`STRINGP`…）同一条裁决**。
  ④ **棘轮的口径修正（lane 的反馈，实测改掉）**：43 条带门体落地后，工具的 `rust_items()` 只看见 `pub fn`、看不见它上面的门，于是把它们归成 `rust-side`（"核心必须导出它"）——**那是已经做完的活**。`rust_items()` 现在分两个集合（无门名 / 带门名→其 feature，取声明上方最近的 `#[cfg (feature = "…")]`），`classify()` 多一个 `rust-side-gated`，`reason()` 说的是真话："核心**确实**导出了它，但只在 `feature = "X"` 之后；本仓任何配置都不编那条臂，所以只有带 X 的构建能清掉这一行——活已做完，这一行是记录。" 实测：**47 行**迁到该类，理由里点名 feature（`ns_output_view`→`have_ns`、`face_set_background`→`dispextern_face_records`、`frame_set_double_buffered`→`ns_impl_cocoa`）。
  ⑤ **四队棘轮按新 HEAD 重生成**：`swift-import-pending.txt` **537 行**（12 条 satisfied 行自动清掉 + 47 条理由改写），`link-pending` 365、`link-exec-pending` 63、`swift-units.tsv` 不变。
  ⑥ **命名下来的任务（它们不在 shim 的 lane 里）**：⑴ `include/frame.rs` 需要在**各自的门下**承载 `HAVE_WINDOW_SYSTEM`/`NS_IMPL_COCOA` 的成员与位移（`no_focus_on_map`/`z_group`/`ns_appearance` 枚举与成员/`ns_transparent_titlebar`）——冻结布局不受影响（门是关的），但**带门布局没有任何编译能验**（与 3.262 同一堵墙）；这 7 条棘轮行等它；⑵ `Vframe_alpha_lower_limit`（frame.c 7421 的 `DEFVAR_LISP`，**实测外层是 `#ifdef HAVE_WINDOW_SYSTEM`**）的存储按 3.29/3.252 的模型归**宿主单元 `src/frame.rs`**（`comp.rs` 的 `Vcomp_eln_to_el_h` 先例）——冻结构建没有任何东西读它，所以它属于**窗口系统/NS 分片**那一轮，不是今天的缺口；⑶ `setAppearance`/`EmacsWindow_initWithEmacsFrame` 是 class 2（Swift 单元自己的 `@_cdecl`）。

- **3.265 L1 的编译半边落地：110 个文件的对拍机械 + 一条 ladder 级发现（编译这一步不能独立跑）（2026-10-05）**：
  ① **工具** `tools/acceptance-l1.py`：把 `lisp/emacs-lisp`（110 个文件）复制到固定的 `build/lisp-l1`，对每个 `.el` 跑 `<binary> -Q --batch -L <scratch> -L <lisp 树的每个子目录> -f batch-byte-compile <file>`，逐文件比较 `(退出码, 诊断)` 与 golden；`--golden` 记参照、`--check` 对拍（golden 里记着参照二进制的身份）。
  ② **实测读数**：参照对自己的 golden **逐字节相同**（`110 file(s), 0 failure(s)` ⇒ 机械本身验过）；而**110 个编译全部失败**，错误是 `Cannot open load file … "srecode/loaddefs"`。
  ③ **这不是任何一棵树的缺陷，而是一条 ladder 级发现**：`*loaddefs.el` 是**构建产物**（`build/emacs-ref/lisp/Makefile` 69-71 用 `find` 找它们、`gen-lisp` 生成它们），所以参照检出与端口的逐字副本**都没有**它们（也不该有），于是**独立**编译 `emacs-lisp/` 无法解析它 `require` 的闭包。两条结论写进 `docs/acceptance.md`：⑴ 编译属于**构建轮**（`make -C lisp`：loaddefs 先、编译后），这也正是计划把 `loaddefs.elc` 放在 L2、把完整 `loadup` 集合放在 L3 的原因；⑵ 因此这个工具的比较是**差分**的而不是绝对的——端口必须在同一输入上复现参照的行为，而当它能自己跑那条构建轮时，把同一工具指向它自己生成的 `.elc` 才是 L1 的"加载"半边要的东西。（**3.268 更正**：这一条的*前提*是错的——"独立编译不可能成功"是 harness 的 `-L` 表在遮蔽，不是 ladder 级发现，见 3.268 ②③；"闭包来自构建过的树、端口需要自己的构建轮"这半条仍然成立。）
  ④ **踩过的两个坑（都记在工具里）**：scratch 目录**固定**（`build/lisp-l1`），这样诊断里出现的路径两侧完全一致；`-L` 必须是**每个 lisp 子目录**各一条（C 自己的 `make -C lisp` 就是每条一条；dumped 二进制的 load-path 只点到树根，所以第一次尝试在 `compile.el`（在 `lisp/progmodes`）上失败）。

- **3.266 参照树的"生成物"是我上一轮清理时删掉的：它现在无法 require 任何非预加载的库——修法与两个障碍都量清了（2026-10-05）**：
  ① **事实（我的回归，先说清）**：3.256 那次清理用 `git -C emacs clean -fX` 删掉参照树里**被 git 忽略**的构建产物，其中包含**生成的 Lisp 文件**——每目录的 `subdirs.el`、各 `*loaddefs.el`、以及 `.elc` 文件。它们不是装饰：`subdirs.el` 是把子目录放进 `load-path` 的东西，`loaddefs` 是 `require` 一个未被 dump 预加载的库时被找的东西。**实测三种调用**：
  `-L <tree>` → `Cannot open load file "bytecomp"`（`bytecomp.el` 在 `emacs-lisp/`，而 dump 的 load-path 只点到树根）；`-L <tree> -L <tree>/emacs-lisp` → `… "compile"`（`compile.el` 在 `progmodes/`）；`-L <每个子目录>` → `… "srecode/loaddefs"`（一个**生成**文件）。
  ② **影响面**：**被跟踪**的参照树没动（`git -C emacs status --short` 为空、`check-verbatim` 仍 `6 trees, 4609 files, 0 failures`），**端口不受影响**；受影响的是**验收侧**——L1 的编译半边、以及任何要加载 dump 之外 Lisp 的检查，在那些文件重建之前跑不了（L0 仍可跑，因为它只需要 dump）。
  ③ **修法与两个障碍（都实测）**：修法就是再跑一次当初生成基线的顶层驱动器，而它的第一步 `./config.status --recheck` 要重新跑 `../../emacs/configure`。障碍一：**`configure` 也是同一次清理删掉的忽略文件**，而单独 `autoconf` 造不出来——实测 `configure.ac:2396: error: undefined or overquoted macro: m4_default` 与 `configure.ac:6405: … AM_LANGINFO_CODESET`（需要带 `m4/` 的完整 `autoreconf`）；`autoreconf` 会重写**被跟踪**的 `src/config.in` 与各 `Makefile.in`，所以修完必须查 `git -C emacs status` 并把被改的还原。障碍二：**手工驱动 `lisp/Makefile` 不等价**——`make -C build/emacs-ref/lisp all EMACS=../src/emacs` 停在 `semantic`（`cedet/semantic/bovine/make-by.el` Error 255），因为 grammar `.el` 的生成是顶层驱动器用 `temacs`/`bootstrap-emacs` 排出的 bootstrap 步骤。
  ④ **安全序列（写进 `docs/acceptance.md`）**：参照树里 `autoreconf` → 还原它改过的被跟踪文件 → 放回零字节 `emacs/info/emacs`（configure 区分 tarball/检出的那个测试）→ `touch build/emacs-ref/config.status`（让 recheck 不触发）→ `make -C build/emacs-ref -j3`（就是建出基线的那条驱动器）。
  ⑤ **顺带量清的 L1-L3 构建轮五步**（记在验收文档里而不是代码里——**跑不起来的工具不是证据**，所以这一轮我写了一个驱动原型、量完就把原型删了）：`update-subdirs` **逐目录**（它把 `subdirs.el` 写进**当前目录**，必须以该目录为 cwd 运行）、`COMPILE_FIRST`（macroexp/cconv/byte-opt/bytecomp/loaddefs-gen/radix-tree）、`MAIN_FIRST`、autoloads（`-l emacs-lisp/loaddefs-gen.elc -f loaddefs-generate--emacs-batch <subdirs>`）、以及"编译其余文件、重复整轮直到不再有进展"。

- **3.268 L1 的编译半边立住了：111 个文件零失败，而上一轮那四个"文件缺陷"是 harness 的 load-path 遮蔽（2026-10-05）**：
  ① **重录 golden（参照树修好之后）**：`111 file(s), 0 non-zero exit(s), 0 file(s) with diagnostics`；`--check` 连跑两次都 `111 file(s), 0 failure(s)`（同一读数两遍 = 机械可重复）；负对照：`cp -Rp` 出一份 `emacs-lisp` 树、只在 `advice.el` 末尾追加一个不闭合的 `(defun l1-neg-control (x` ⇒ `111 file(s), 1 failure(s)`，只有那一件 FAIL。（第一版负对照用 `cp -r`，拷贝的 mtime 比树里的 `.elc` 新，于是**每一件**都多出 `Source file … newer than byte-compiled file` 诊断——时间戳也是输入的一部分。）
  ② **根因：不是文件，是 `-L` 表在遮蔽**。3.265 那份 golden 里的 4 个非零（`bytecomp.el` 的 `failed to provide feature 'compile'`，`checkdoc.el`/`eieio-opt.el`/`eieio-speedbar.el` 的 `Recursive 'require'`）来自 harness：它的 `-L` 是 scratch + 树的**子目录**，**没有树根**，而且按字典序。实测 `locate-library` 三条：
     `compile` → `emacs/lisp/cedet/srecode/compile.elc`（那个文件提供的是 `srecode-compile`）、`speedbar` → `emacs/lisp/cedet/ede/speedbar.elc`、`dired` → `emacs/lisp/cedet/ede/dired.elc`；而正确解析是 `progmodes/compile.elc`、`speedbar.elc`、`dired.elc`（全在树根）。
  ③ **两条修法（都进了工具）**：⑴ **树根必须进 `-L`**（紧跟 scratch）；⑵ 子目录必须按**被测二进制自己的 dumped load-path 顺序**，因为 **`-L` 目录是按命令行顺序搜索的**（实测 `-L A -L B` ⇒ `load-path` 头 `(A B …)`，排在 dump 自带的路径**之前**），而参照 dump 的顺序是 `vc use-package url textmodes progmodes play org nxml net mh-e mail leim language international image gnus eshell erc emulation emacs-lisp cedet calendar calc obsolete`（`make -C lisp` 的 `SUBDIRS` 被逐个前插后的倒序）——**`progmodes` 排在 `cedet` 之前，就是 `bytecomp.el` 能 `require` 到 `compile` 的全部原因**。工具新增 `dump_lisp_dirs(binary)`（`(prin1 load-path)`，取 `lisp` 根之下、保持顺序的相对路径），`load_flags()` = scratch + 树根 + 那些子目录，顺序即搜索顺序。
  ④ **文档就地更正**：`docs/acceptance.md` 的 L1 节标题与正文改成"编译半边 + harness 的两个 load-path 缺陷"：旧结论"独立编译不可能成功/110 全失败"标为**第一版读数**并写明它测的是 harness；两条 ladder 结论保留但换了理由——闭包来自**构建过的**树（`subdirs.el`/`*loaddefs.el`/`.elc` 是 C 的驱动器写的），所以参照侧靠基线构建、端口侧要靠它自己的构建轮（呼应 L2 的 `loaddefs.elc` 与 L3 的 `loadup`）。参照树的修复过程（`autoreconf -fi`、`/tmp/fakebin/makeinfo` 这个**关于工具的谎**、零字节 `emacs/info/emacs`、`touch config.status`、`make -j3 bootstrap` ⇒ exit 0、1645 个 `.elc`、`git -C emacs status` 为空）也补进同一文档，并写明 **`make all`（`info/`、`leim`、`admin`）故意没跑**：没有真 `makeinfo` 时它只会造出空手册，而 ladder 现在不需要。
  ⑤ **教训**：**"某文件编译失败"先量它 `require` 到了哪个文件**——一行 `locate-library` 就能把"树的缺陷"变成"harness 的遮蔽"。这与 3.196/3.198/3.202/3.206 是同一类错误（把片段当事实），这次的片段是 load-path 的顺序。

- **3.269 `check-declarations` 在 fe06dbc 上的复测，以及"还没动的"到底是什么（2026-10-05）**：
  ① **复测读数**：`13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`（读 630 个文件：623 `.rs` + 7 `.swift`）。四类声明（`decl` 归属块 / `version` 版本括注 / `title` 其它注释里的产品名 / `recital` 复述许可处理）**全部 0 禁止项**；13 条允许项**全是 C 自己的字符串**（`src/emacs.rs` 6、`src/callproc.rs` 3、`lib-src/etags.rs` 3、`lib-src/ebrowse.rs` 1），逐条理由在 `docs/baseline/declarations-allowlist.tsv`。历史处理：231 个版本行（`2ee1d1d`/`4810bb0`）、268 个复制来的声明块（`fd9cf89`）、端口树自己的 README 标题（`30ff6c2`）。
  ② **把"产品名"缩到裸 `Emacs` 之后还剩什么（两个实测数）**：端口树注释里 **472 行**提到它——逐行看全是 C 自己的解释性散文（"Emacs ignores SIGPIPE"、"Store into *ACTION a signal action suitable for Emacs"、"Open FILE as a stream for Emacs use."），**没有一条是声明**；其中"标题式"（短行、以 `for/of/in Emacs.` 收尾）**54 行 / 约 19 个文件**，两个文件（`src/xterm.rs`、`platform/macos/nsterm.swift`）正是 lane 在飞的文件。另外 `lib-src/emacsclient.rs:2223` 的 `"Waiting for Emacs..."` 是 C 的**字面量**（语义，docstring 门对比）。
  ③ **为什么这一轮不动**：⑴ 用户指令"前期复刻期间跟 C 尽量一致"，改写这些散文是**偏离参照**，而它们不主张身份、不给出版本；⑵ 真正主张身份的三处（`PACKAGE_NAME`、`RCS_Id`、`etags` 的 banner）是 C 的语义，改它们会让端口与参照的 `--version`/`emacs-version` 行为分叉（L0/L5 的判据）；⑶ 4 条 lane 正在改其中两个文件，全树注释清扫会与在飞工作冲突。
  ④ **留给用户的裁决（不阻塞当前工作）**：是否把 `title` 类从"`GNU Emacs`"扩到裸 `Emacs`（代价：~54 行 C 散文要改写，或进 allowlist 逐条记理由）；`docs/` 里 14 处描述性句子（"a one-to-one replication of GNU Emacs"、许可句）同理。

- **3.270 四队交付一次入库，四本棘轮按新 HEAD 重生成（2026-10-05）**：
  ① **入的四片**（各一个签名提交，只含该 lane 的单元与台账）：`772e460` xterm 第 3 片（C 1763-2173，13 个 static 函数：Motif 副作用映射、targets 表读写、拖拽窗口与两个错误处理器）；`0eb2928` nsfns 第 5 批（C 3229-3800，9 个 DEFUN + 5 个函数：tooltip 机器与 geometry 组，含 `x-show-tip`/`x-hide-tip`、tip frame 的建与撤、monitor attribute list）；`34c9598` nsterm 第 11 片（C 2401-2621，Mouse handling 整节 4 个定义）；`801da98` xdisp 本轮（redisplay 尾段四个偏差节）。
  ② **入库前复检（集成者自己跑的，不引用 lane 的读数）**：`build-tu src/xterm.c`、`build-tu src/xdisp.c` 各 exit 0；`swiftc -parse-as-library -c` 对 `nsterm.swift`/`nsfns.swift` 都 exit 0（nsterm 的 7 条 warning 与上一片同一批、无新增）；`check-declarations` 13 sites/0 forbidden、`check-globals` 284/0、`check-mirror-dupes` 1/0、`check-verbatim` `6 trees, 4604 files, 0 failures`。
  ③ **四本棘轮重生成**（`tools/reconcile-link-ratchets.sh`，2m35s）：`link-pending` **365 → 373**、`link-exec-pending` **63 → 83**、`swift-import-pending` **537 → 616**（nsfns 交来的 78 条 + nsterm 的新导入）、`swift-units.tsv` 少 10 行（6 条 stale 定义在内）。复检：`check-link` 373/373/0、`check-link --executable` 83/83/0、`check-swift-units` `0 new, 0 stale, 7 Swift unit(s) landed`、`check-swift-imports` `877 import(s) in 7 Swift file(s), 5656 core export(s), 616 unprovided, 0 failure(s)`。**顺序是硬要求**：这台工具在 **HEAD 的 worktree** 里造读数，所以四片必须先各自提交、再对账（这次 4 提交 → 1 对账走通；工作树在对账时是干净的，工具那条"在飞单元在 HEAD 重建"的分支为空）。
  ④ **命名下来的任务（本轮不做，各台账都写了）**：⑴ xdisp 的 **item 50**——`current_buffer`/`specpdl`/`specpdl_end`/`specpdl_ptr` 在 `include/buffer.rs:27` 与 `include/lisp.rs:3783-3785` 被声明成 `extern "C"` static，而 `include/thread.rs:156/158` 已有正确的宏形态，端口**没有任何单元定义**这四个名字（参照对象里也没有）⇒ 最终链接会缺 `_current_buffer`/`_specpdl`/`_specpdl_end`/`_specpdl_ptr`；⑵ **item 51**——xdisp 的 `putc`（C 12809/12813）在本目标是 stdio 宏，参照对象引的是 `putc_unlocked`；⑶ xterm lane 要的裁决——X11 臂里的 `None`/`Success` 与 Rust 的 `Option::None` 冲突（要么一份 Xlib 镜像定这些名字，要么单元内 `pub const None: Window = 0;`）；⑷ nsfns 的最后一批从 C 3802 起（`ns-request-user-attention` → `syms_of_nsfns`），含唯一一个 `EmacsFileDelegate`。

- **3.271 带门臂的布局第一次有了读数：那堵墙在 Rust 侧，C 侧本来就读得出来（2026-10-05）**：
  ① **问题**：3.257 ②/3.264 ⑥ 把 `HAVE_WINDOW_SYSTEM`/`HAVE_NS`/`NS_IMPL_COCOA` 臂的结构体布局记成"没有任何编译能验"。复核后拆成两半：**Rust 侧**确实编不过（端口的带门臂缺平台镜像，3.262 的 14 条 E0425 是同一件事），**C 侧**只是没人读——冻结构建生成的 `config.h` 加上 configure 会发出的门宏，头文件照样解析。
  ② **工具** `spikes/gated-layout-probe.py`（P-82，立即生效）：`spikes/gated-layout-probe.py 'struct frame' frame.h [c|objc]`——探针 TU 建在临时目录、**从不写进 `emacs/`**，clang 以 `-Xclang -fdump-record-layouts-complete` 打印记录布局，臂为 `frozen` 与 `ns_cocoa`（`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_NS=1 -DNS_IMPL_COCOA=1`），`--define` 加第三臂，`--write` 落 TSV。
  ③ **两条实测口径**：⑴ **臂是配置不是单个宏**——只给 `HAVE_WINDOW_SYSTEM=1`（不给窗口系统头）时 `dispextern.h` 缺 `Emacs_Cursor`/`Emacs_Pix_Container`/`Display_Info`，整臂不解析（这正是 3.262 那堵墙的同一形状）；⑵ **必须 `-complete`**——`sizeof` 本身不会让 clang 打印该记录（`struct ns_display_info` 只取 `sizeof` 时 dump 里一条都没有），而 `-fdump-record-layouts-complete` 打 1912 条、含它。只前置声明、该臂不完整的类型读作 `parse=no` 而不是静默丢失。
  ④ **读数**（落 `docs/baseline/`）：`struct frame` **frozen 568/8、ns_cocoa 608/8**；ns_cocoa 独有的 17 个成员及其精确位置——`tab_bar_window` 160、`desired_tab_bar_string` 168、`current_tab_bar_string` 176、`image_cache` 216、`minimize_tab_bar_window_p` 272:5、`external_tool_bar` 272:6、`external_menu_bar` 273:2、`tooltip` 275:6、`want_fullscreen` 276:0-3、`vertical_scroll_bar_type` 276:4-5、`horizontal_scroll_bars` 276:6、`override_redirect` 276:7、`skip_taskbar` 277:0、`no_focus_on_map` 277:1、`z_group` 277:2-3、`ns_appearance` 592、`ns_transparent_titlebar` 596。`struct ns_display_info` ns_cocoa **280/8**（frozen 侧 `parse=no`，它只在 NS 臂里完整）。两个臂的成员表各 295 / 52 行存进 `docs/baseline/frame-layout.tsv`、`docs/baseline/ns_display_info-layout.tsv`（工具生成，可重跑复检）。
  ⑤ **改变什么**：`include/frame.rs` 的带门臂从"按 C 的声明手推"变成"按读数写"（17 个成员 + 位段在 NS 臂里的位移），剩下的差距是**端口的带门臂仍编不过**（需要平台镜像），那一条在 3.262 已量过、现在少了"没有数"这一半；同一手法对 nsterm/nsfns 需要的每个带门结构体都成立，lane 可以自己读。

- **3.272 `struct frame` 的带门成员落地：25 个成员 + 两个声明链按 C 的门进镜像，位序只写冻结臂并显式反过来挡住（2026-10-05）**：
  ① **落地**（`femacs/src/include/frame.rs`）：C 那 29 条带门成员声明里，端口此前缺 25 条，现在全部按 **C 自己的门**补上，位置照 C 的声明顺序——`menu_bar_window`（`HAVE_WINDOW_SYSTEM && !HAVE_EXT_MENU_BAR`）、`tab_bar_window`/`desired_tab_bar_string`/`current_tab_bar_string`/`image_cache`（`HAVE_WINDOW_SYSTEM`）、`tool_bar_window`/`desired_tool_bar_string`/`current_tool_bar_string`/`last_tool_bar_item`（`HAVE_WINDOW_SYSTEM && !HAVE_EXT_TOOL_BAR`）、`font_data`（`HAVE_XFT||HAVE_FREETYPE`）、`namebuf`（`USE_X_TOOLKIT||HAVE_NTGUI`）、`shell_position`（`USE_X_TOOLKIT`）、`wait_event_type`（`HAVE_X_WINDOWS||HAVE_ANDROID`）、`ns_appearance`/`ns_transparent_titlebar`（`NS_IMPL_COCOA`）、`conversion`（`HAVE_TEXT_CONVERSION`）；连同 C 为它们声明的 `enum ns_appearance_type` 与 `HAVE_TEXT_CONVERSION` 那一段（`enum text_conversion_operation` + `struct text_conversion_action` + `struct text_conversion_state`——`src/alloc.rs` 的带门代码早就在点这两个名字，此前镜像里根本没有）。
  ② **新增四个门**（`femacs/xtask/Cargo.toml`，按字母序插进表里）：`have_ext_tool_bar`、`have_ext_menu_bar`、`have_xft`、`have_freetype`。**实测关系**：`HAVE_EXT_TOOL_BAR`/`HAVE_EXT_MENU_BAR` 在 C 里是**窗口系统蕴含**的（`lisp.h:3933/3938` 为 NS 定义它们），所以探针的 ns 臂有这两个门、x11 臂没有——翻 `have_ns` 时必须一起翻。
  ③ **两处只有 C 说得清的形状**：⑴ `ns_transparent_titlebar` 是**非位域**的 `bool_bf`，而 C 的 `bool_bf` 在本构建里是 `typedef unsigned int`（`conf_post.h:59`），读数也对（596，`z_order` 在 600）⇒ 端口那里写 `c_uint` 并注明，别的 `bool_bf` 仍是位域拼法；⑵ 位序**逐配置不同**（C 把窗口系统的位插进那一串，且放不下的 `ENUM_BF` 会被推到下一个存储单元——实测 `want_fullscreen` 在 X11 臂是 `316:0-3`，比 `tooltip` 的 `315:6` 过一位）。
  ④ **位序这一轮只写冻结臂，并且反过来挡住**：那 37 个 `FRAME_*_SHIFT` 常量与 frame.h 1240-1271 的 9 个 `#else` 形访问器（`FRAME_OVERRIDE_REDIRECT`/`FRAME_SKIP_TASKBAR`/`FRAME_NO_FOCUS_ON_MAP`/`FRAME_NO_ACCEPT_FOCUS`/`FRAME_TOOLTIP_P` 与 z-group 四件）现在都带 `not (have_window_system)`。**理由**：那两个窗口系统臂的编号已经在 `docs/baseline/frame-layout.tsv` 里量到了（三臂：frozen 568 / x11 648 / ns_cocoa 608），但还没抄成常量；宁可让窗口系统构建在第一个访问器处**编不过**，也不要它静悄悄读到错的位。下一步就是把量到的编号写出来（或按 C 的声明顺序推导并用该表逐条复检），同时补那 9 个访问器的 `#if` 体。
  ⑤ **读数**（本修订复跑）：`build-tu src/frame.c`、`src/window.c`、`src/buffer.c` 全 exit 0；`layout-check passed`；`check-declarations` 13 sites/0 forbidden；`check-globals` 284 units/0；`check-mirror-dupes` 0 failure(s)；`check-verbatim` 6 trees/4604 files/0 failures。

- **3.273 位序的下一层：不只是 shift，还有"落在哪个 32 位单元"——5 个成员跨臂换单元（2026-10-05）**：
  ① **新读数形式**：`spikes/gated-layout-probe.py --bits` 把每个臂的位段导成镜像真正需要的形式 `(unit, shift, width)`（C 把标志打进 4 字节单元），写入 `docs/baseline/frame-bits.tsv`（frozen 37 行、x11 48 行、ns_cocoa 48 行）。
  ② **导出式与镜像对得上**：frozen 那 37 行与 `femacs/src/include/frame.rs` 里已验过的字面常量**逐条相等（0 处不符）**——这正是另外两个臂的行可以拿来用的前提。
  ③ **两个决定下一步的数**：⑴ 50 个位成员里 **32 个**在 frozen 与两个窗口系统臂之间 `(unit, shift)` 不同（`visible` frozen u1+8、两个窗口系统臂都是 u1+11）；⑵ **5 个成员连"单元"都换了**：`no_special_glyphs`（u1+27 → u2+12）、`can_set_window_size`（u1+28 → u2+13）、`after_make_frame`（u1+29 → u2+14）、`tab_bar_redisplayed`（u1+30 → u2+15）、`tab_bar_resized`（u1+31 → u2+16）——C 把窗口系统的标志插在它们前面，这一串跨过了 4 字节边界。
  ④ **因此冻结编号是"带门"而不是"留着字面量"**：镜像的读取端是**手工**选单元的（`frame_bit1 (f, …)` 读 unit 1、`set_frame_flag2!` 写 unit 2），所以窗口系统配置需要把**单元**也当数据；要么每臂一对 `(unit, shift)` 常量（表已生成），要么读取端按单元常量分派。两条都是模型级改动，留给后续一轮；两条现在都有读数可依。

- **3.274 frame.h 另外四个"一门两体"的宏也按同一口径挡住（2026-10-05）**：`FRAME_EXTERNAL_MENU_BAR`/`FRAME_EXTERNAL_TOOL_BAR`（frame.h 1078-1098，一个臂读成员、另一个臂是 `false`）、竖滚动条四件（1215-1230）、`FRAME_HAS_HORIZONTAL_SCROLL_BARS`（1276-1280，门是 `USE_HORIZONTAL_SCROLL_BARS`）此前都只带了 C 的 `#else` 体且**没有门**——现在按各自的门带上（`not (have_ext_menu_bar)`、`not (have_ext_tool_bar)`、`not (have_window_system)`、`not (use_horizontal_scroll_bars)`；最后一个 feature 是新增的，冻结 `config.h` 里 `USE_HORIZONTAL_SCROLL_BARS` 未定义）：窗口系统配置在它们处**编不过**，而不是拿冻结臂的常数回答；它们的 `#if` 体要读 `docs/baseline/frame-bits.tsv` 的编号，属同一个下一步。读数：`build-tu src/frame.c`、`src/term.c` exit 0；`layout-check passed`；`check-declarations` 13/0；`check-globals` 284/0；`check-mirror-dupes` 0 failure(s)。

- **3.275 `struct window` 的四个窗口系统成员落地；"镜像漏掉带门成员"这一类量出了范围（2026-10-05）**：
  ① **落地**：`phys_cursor_type`/`phys_cursor_width`/`phys_cursor_ascent`/`phys_cursor_height`（window.h 339-351，`#ifdef HAVE_WINDOW_SYSTEM`）此前被字段扫描器的活行过滤（P-73 读冻结 `config.h`）挡在镜像外，现在按 `feature = "have_window_system"` 带上；位置不是猜的——`docs/baseline/window-layout.tsv`（三臂：frozen 520 / x11 536 / ns_cocoa 536，两个窗口系统臂**只多这四个**，452/456/460/464）加上 `docs/baseline/window-bits.tsv`（19 个位成员，三臂 `(unit, shift)` **完全相同**）⇒ 这里**不需要**门住任何编号。
  ② **这一类有多大：量了**（`spikes/gated-layout-probe.py` 三臂 × 逐镜像比对 `pub <name>:`）——`struct terminal`（termhooks.h）4 个带门成员**镜像全缺**（`query_colors`、`free_pixmap`、`any_grab_hook`、`activate_menubar_hook`，都是函数指针，且类型链是 `Emacs_Pixmap`/`Display_Info`/`Emacs_Color` 那一批 ⇒ 和窗口系统镜像链一起做）；`struct image`（dispextern.h 3215）**整个类型都在 `#ifdef HAVE_WINDOW_SYSTEM` 里**——冻结臂里它根本不是一个完整类型、镜像里也没有它，所以那一类不该记成"26 个成员缺"（**3.278 更正**：这是"整个类型带门"，不是"成员漏抄"）；`struct face` 1 个缺（`stipple`，dispextern.h 1810，在 `#ifdef HAVE_WINDOW_SYSTEM` 块内）；`struct ns_output` 40 个带门成员**一个不缺**（nsterm lane 的镜像已经带了）；`struct glyph`/`glyph_row`/`glyph_matrix`/`tty_output`/`font`/`x_output` 都是 0。
  ③ 读数：`build-tu src/window.c` exit 0；`layout-check passed`；`struct window` 对探针已无缺项。

- **3.276 `struct face` 的窗口系统两件也落地（2026-10-05）**：C 在 `#ifdef HAVE_WINDOW_SYSTEM` 下给 `struct face` 声明了画字用的 GC 与 stipple id（dispextern.h 1797-1811），其中 GC 自己还有三臂（X11 的 `GC`、Android 的 `struct android_gc *`、其余 `Emacs_GC *`）。现在按这三条门带上（X11 那条沿用 `struct glyph_string` 已有的 P-72 rule 3 无类型指针写法）。读数：`docs/baseline/face-layout.tsv`——frozen 280/8、x11 296/8、ns_cocoa 296/8，`gc` 在 168、`stipple` 在 176（两臂相同），这十六字节正是这一对，其后位串不动 ⇒ 本结构不需要门任何编号。读数复检：`build-tu src/xfaces.c`、`src/dispnew.c` exit 0；`layout-check passed`。

- **3.277 `struct terminal` 的四个带门 hook 落地（2026-10-05）**：与 `struct window` 同一类——`query_colors`（termhooks.h 615-621，门是 `HAVE_X_WINDOWS||HAVE_NTGUI||HAVE_PGTK||HAVE_ANDROID`）、`activate_menubar_hook`（719-722，`HAVE_EXT_MENU_BAR`）、image 对 `free_pixmap`（849-853）与 `any_grab_hook`（878-885，`HAVE_WINDOW_SYSTEM`）都被 P-73 的活行过滤挡在镜像外，现在各按 C 自己的门带上。其中两件的参数类型本树没有镜像（`Emacs_Pixmap` 是 xterm.h 的 `typedef XImage *`、`Display_Info` 是 xterm.h 的 display 记录）⇒ 按 P-72 rule 3 留无类型指针并写明理由（与 `struct glyph_string` 的 GC 同一裁决）。读数：`docs/baseline/terminal-layout.tsv`——frozen 520/8、x11 544/8、ns_cocoa 544/8。复检：`build-tu src/term.c`、`src/keyboard.c` exit 0；`layout-check passed`；字段集对上一修订是 **+query_colors +activate_menubar_hook +free_pixmap +any_grab_hook、无丢失**（第一次补丁误把 `menu_show_hook`/`delete_frame_hook`/`toolkit_position_hook` 顶掉，是 `layout-check` 的 offset 断言抓出来的——这正是那道门存在的意义）；`check-declarations` 13/0、`check-globals` 284/0、`check-mirror-dupes` 0 failure(s)。

- **3.278 "整个类型带门"这一类第一次量清：探针的 `incomplete` 就是它的读数（2026-10-05）**：
  ① **口径**：`spikes/gated-layout-probe.py <type> <header>` 对某个配置说 `incomplete type` 时，意思是**该配置根本没有定义这个类型**（不是解析失败）——这是一等读数，它区分了两类活：**成员漏抄**（类型在、某个成员在别的臂里才有）与**整个类型带门**（类型本身只在某些门下存在）。
  ② **实测（12 个候选类型 × 三臂 × 镜像是否存在）**：真正的缺口只有两个——`struct image`（dispextern.h 3215，门 `HAVE_WINDOW_SYSTEM`：frozen 不完整、x11/ns 完整，**镜像里整个类型都没有**）与 `struct pgtk_output`（pgtkterm.h，门 `HAVE_PGTK`：三臂都不完整，镜像也没有）。其余都对得上：`struct ns_output`/`struct ns_display_info` 只在 ns 臂完整而镜像**已有**（nsterm lane 的功劳）；`struct xwidget` 三臂都不完整而镜像已有（门是 `HAVE_XWIDGETS`）；`x_output`/`w32_output`/`haiku_output`/`android_output`/`tty_output`/`glyph_pool`/`x_display_info` 在 frozen 臂就完整、镜像也都有。
  ③ **因此 3.275 ② 的那句话是错的并已就地更正**：`struct image` 不是"34 个带门成员、26 个缺"，而是**整个类型带门**；把它带进来意味着镜像里给这个结构加 `#[cfg]`（连同它自己那 34 个窗口系统形状的成员——其中平台句柄是各臂的差异，其余两臂一致）。`pgtk_output` 同理。
  ④ 下一步（记在本轮台账里，不在本轮做）：按这个口径把"整个类型带门"的清单扫完（本轮只扫了 12 个候选），然后逐个补 gated 镜像；`struct image` 优先，因为 xterm lane 的图像路径要用它。

- **3.281 xdisp 第 22 轮与 nsfns 收口入库，四棘轮按新 HEAD 重生成；对账脚本两处真 bug 修好（2026-10-05）**：
  ① **入库**：`f47e69f` xdisp 第 22 轮（`set_cursor_from_row`，C 18630-19255 共 626 行；台账 Functions 208/302 → **209/302**，`- ` 仍 126，新增 1 条 `does not mention goto`——那是标签块 `'compute_x` 的写法，不是缺体）；`1881348` nsfns 第 6 批（C 3802-4348）——**`docs/ports/nsfns.md` 0 未勾，该单元收口**，同轮还按 P-82 把该文件唯一的带门镜像 `struct ns_display_info` 与探针逐成员核对（15 个成员、偏移 0/8/…/92、`sizeof 280`，**零改动**）。
  ② **对账脚本两处真 bug**（第一次跑就暴露）：⑴ 它把**在飞的镜像** `src/include/xterm.rs` 当翻译单元去 `build-tu` ⇒ 446 条错误；现在改成"镜像 → 包含它的单元"（`grep -l "include/<mirror>" femacs/src/*.rs`）。⑵ 上次被杀的运行留下 worktree 注册、目录已不存在 ⇒ `worktree add` 拒绝；现在先 `git worktree prune`（脚本可重跑）。
  ③ **四棘轮重生成**（2m18s）：`link-pending` 373、`link-exec-pending` 83（**都 0 failure**——此前那个 `_cursor_row_p` FAIL 随 xdisp 片入库而消失）、`swift-import-pending` 616 → **621**、`swift-units.tsv` 少 4 行。复检：`check-link` 373/373/0、`--executable` 83/83/0、`check-swift-units` `0 new, 10 stale, 7 landed`（stale 全是在飞文件）、`check-swift-imports` 907 imports / 5661 core exports / 621 rows / 17 failure（17 条来自在飞 `nsterm.swift`，HEAD 侧一致）。
  ④ **item 50 的输入现在齐了**（xdisp 的 40 处 `BVAR!` + 70 处裸 `current_buffer`（含 4 处赋值）+ 18 处 `SPECPDL_INDEX` 调用点清单），但该 lane 会接着写第 23 轮 ⇒ P-19 那次一次性改写仍要等这个文件静下来，或先把不碰 xdisp 的部分做完。

- **3.282 镜像覆盖率的第一次全树读数：冻结臂 **0 缺**，带门臂 115 条待补；探针一处"把平台头喂给冻结臂"的假象修好（2026-10-05）**：
  ① **新工具** `spikes/mirror-struct-coverage.py`：把 `femacs/src/include/*.rs` 里**每个 `pub struct`** 拿去问 `gated-layout-probe.py`（按镜像 stem 猜 C 头，`nsterm`/`x_fns` 一类有覆盖表），逐臂比对"读数里有、镜像里没有 `pub <成员>:`（含 `r#` 写法）"的成员。两个读数分开报：**冻结臂缺口 = 缺陷**，**窗口系统臂缺口 = 待补工作清单**。
  ② **冻结臂：0 缺**（282 个结构体 / 846 个臂尝试 / 239 个臂读不了——读不了=该配置不定义这个类型，或平台头不在）。这条把"字段扫描器 P-73 会不会把冻结臂的成员也滤掉"这个担心**否定**了：至少在这 282 个结构体上，冻结配置的成员一个不缺。
  ③ **带门臂待补 115 条 / 5 个结构体**：`x_display_info`（x11 臂 80 条：`Xatom_*` 家族、`wm_type`、`x_dnd_atoms*`、`motif_drag_atom*`…）、`redisplay_interface`（两臂各 15 条：`draw_fringe_bitmap`/`define_fringe_bitmap`/`destroy_fringe_bitmap`/`compute_glyph_string_overhangs`…）、`x_output`（2 条：`black_relief`/`white_relief`）、`handler`（1 条：`x_error_handler_depth`）、`image`（1 条 `type`——该类型整体还没进镜像，归 3.278 那条"整个类型带门"）。这就是带门面的剩余清单（Boyle 正在做 `struct image`）。
  ④ **两个假象，都是我自己的工具造成的（记下来）**：⑴ 探针**无条件**把 `-I /opt/homebrew/include` 加进每个臂 ⇒ 冻结臂也能解析 `xterm.h`，于是报出 `struct x_display_info` 的"207 个成员、镜像缺 141"——而那个结构在冻结配置里根本不参与编译（镜像里 `#[cfg (feature = "have_x_windows")]` 的理由写在注释里）。修法：平台头**跟着平台臂走**（`-I X11` 只给定义 `HAVE_X_WINDOWS` 的臂，`-I nextstep` 只给 objc 臂）；修完复跑七张已发布的 `docs/baseline/*-layout.tsv`/`*-bits.tsv`，**逐字节不变** ⇒ 已发布的读数没有被污染。⑵ 覆盖率工具的字段正则只认小写开头（`[a-z_0-9]+`），于是 C 的大写开头成员（`Wcm`、`TS_ins_line`、`Voverriding_terminal_local_map_`、`IP`/`SP`）**全被报成缺**——第一次跑出 76 条"冻结臂缺口"，逐条查完全部是这两类假象；改成 `[A-Za-z_0-9]+` 后归 0。教训与 3.196/3.198/3.202/3.268 同族：**读数的第一件事是拿已知正确的例子验它**（这次两处都验出来了）。

- **3.283 `struct handler` 的 `x_error_handler_depth` 落地，覆盖率读数 115 → 114（2026-10-05）**：lisp.h 3868-3870 的 `#ifdef HAVE_X_WINDOWS` 成员（`eval.c` 1442 的 `x_unwind_errors_to (catch->x_error_handler_depth)` 与 1862 的存点）此前不在镜像里，现按门带上；读数：`struct handler` frozen 288 / x11 **296**（该成员在 288）/ ns 288（该臂没有 HAVE_X_WINDOWS）。复检：`build-tu src/eval.c` exit 0、`layout-check passed`、`check-declarations` 13/0、`check-mirror-dupes` 0 failure，且**覆盖率工具的读数自己动了 115 → 114 并把 `struct handler` 从清单里去掉**——这是这种"全树读数"能当证据用的样子（改动与读数一一对应）。剩下的带门缺口全部落在别人在飞的镜像里：`x_display_info`（x11，80 条）与 `x_output`（2 条）在 `include/xterm.rs`（Ohm），`redisplay_interface`（两臂各 15 条）与 `struct image`（整类型）在 `include/dispextern.rs`（Boyle）——已通报给两位。

- **3.284 非 C 平台目标的现状第一次量清：53 个目标里 52 个还没落地；先落 6 个"Kept"汇编文件（2026-10-05）**：
  ① **读数**（按 `docs/port-map.md` 的目标路径逐条查文件系统）：**Swift 7/8**（缺 `femacs/platform/macos/nsxwidget.swift`）、**Kotlin 0/40**（`femacs/platform/android/kotlin/*.kt` 一个都没有）、**C++ 0/5**（`femacs/platform/haiku/*.cc`）、**Kept 0/6**（`femacs/exec/loader-*.s`）⇒ **53 个非 C 目标里 52 个不存在**。计划里这批是**批次 F**"平台与工具"（`java/`→Kotlin、`*.m`→Swift、`*.cc`→C++，53 文件 / 49,759 行），但 `docs/ports/` 的台账机制**没有给它们生成勾选清单**（`android-emacs.md`/`haiku.md` 是 `src/*.c` 的台账，不含 `java/`、`*.cc`）⇒ 它们此前只在 `port-map.md` 里被计数，没有工作台账盯着。
  ② **先落最便宜的一类**：`port-map.md` 把 6 个 `exec/loader-*.s` 归为 **Kept**（生成器原文："assembly and data files that are used as they are"，目标路径原样保留）⇒ 逐字节复制 `emacs/exec/` → `femacs/exec/` 并 `cmp` 验过：`loader-aarch64.s` 6301 B（sha256 `79df36c4537c7026…`）、`loader-armeabi.s` 6644 B（`acda1c4be755f57f…`）、`loader-mips64el.s` 10378 B（`650670e5049d39ae…`）、`loader-mipsel.s` 10619 B（`c1f4aeb69922baa8…`）、`loader-x86.s` 8320 B（`7bff61c4a606be5a…`）、`loader-x86_64.s` 7487 B（`330c4aba65e40ebe…`），六个全 `identical`。
  ③ **下一步（自由工作，不在任何 lane 的写集里）**：Kotlin 40 个文件（15,157 Java 行）与 Haiku C++ 5 个（8,264 行）是**没人做**的大块；最后那个 Swift（`nsxwidget.swift`）归 NS lane。规矩上要先在 `PORTING.md` 立 Kotlin/C++ 的写法节（否则 15k 行会各写各的），再按文件推进；另外给 Kept 类加一道"路径 + sha256 基线"的小门，才是让它们一直逐字节一致的办法（本轮只做了复制与 `cmp` 证据）。

- **3.285 本轮最重要的读数：98 个目标文件只有参考的 10% 以下，共 **279,028 行 C**——而现有每一个门都看不见它们（2026-10-05）**：
  ① **机制（为什么一直没看见）**：清单门（`extract-abi.py --check`）比的是**冻结配置的活行**（P-73 的活行过滤）。一个**整段都包在冻结 `config.h` 为假的条件里**的文件——`android*.c`、`haiku*.c`、`w32*.c`、`pgtk*.c`、`msdos*.c`、`lwlib*`、`oldXMenu`、`exec/exec.c`、`src/sfnt.c`、`src/image.c`…——其清单是**空的**，于是 `--check` 空过、台账 0 未勾，而端口目标是"11 行头注释"。**上一轮那个"702 未勾"的进度读数是这个机制的下界，不是全部**：未勾框只覆盖"有活行的文件"。
  ② **新门** `tools/check-port-lines.py` + `docs/baseline/port-lines-pending.tsv`：按 `docs/port-map.md` 的 1:1 表逐条比"参考行数 vs 目标行数"，**低于 10% 且参考 ≥200 行**记一行（首行为阈值 10%）。新出现的缺口 = 失败，修好的行 = stale（信息性），与其它棘轮同口径。读数：**98 行 / 280,076 行参考**（其中 `.c` 96 个 / 279,028 行）；按顶层目录：`src` 75、`lwlib` 6、`lib-src` 5、`exec` 3、`nt` 3、`oldXMenu` 3、`lib` 1。最大的几个：`src/xterm.c` 33,125 → 2,403（**这是 Ohm 在飞的那片**）、`src/sfnt.c` 21,215 → 11、`src/image.c` 13,805 → 11、`src/w32fns.c` 12,488 → 13、`src/w32.c` 11,362 → 15、`src/xfns.c` 10,662 → 11、`src/w32term.c` 8,414 → 11、`src/androidvfs.c` 7,950 → 11。另有 30 个 10–50% 的"薄"文件（61,205 行）不计入这个门（信息性）。
  ③ **它改变了"下一步"的判断**：端口的真正大头**不是** xterm/nsterm/xdisp 剩下的那些勾选框（702 个），而是这批被门忽略的平台/工具文件——`plan` 的批次 F 早就写了"约 24 万行"，但**每个文件级的门都看不见它**，所以进度汇报里从来没出现。行数口径上，590 个目标里 462 个 ≥50%、98 个 <10%，真正的完成度**按行不足六成**。
  ④ **下一步（自由工作，没有 lane 在做）**：按批次 F 的顺序开平台树（`w32*`/`x*`/`pgtk*`/`android*`/`haiku*`/`msdos*`/`lwlib`/`oldXMenu`/`exec`/`sfnt`/`image`），并先把 3.284 说的 Kotlin/C++ 写法节立起来；这批每落一个文件，本门就会把它从清单里移出一行（或降级为 stale），进度**可数**。

- **3.286 平台目标开了头：Kotlin 写法节 P-83 + 头三个叶子文件（编译已验证）+ 40 文件依赖序（2026-10-05）**：
  ① **P-83（Kotlin 约定，PORTING.md）**：JNI 是**按名字**找类/字段/方法（`FindClass("org/gnu/emacs/X")`、`GetFieldID(…,"name","I")`、`FIND_METHOD(…,"m","()V")`）⇒ JNI 读的字段写 `@JvmField`（Java `volatile` 再加 `@Volatile`）、JNI 调的方法保持 `fun`（不许换属性）、**禁用 `internal`**（Kotlin 会给 JVM 名加后缀）；Java 的 package-private 无对应（用默认可见 + 在文件里记一句）；`native`→`external fun`、`synchronized`→`synchronized(this){}`/`@Synchronized`、`throws E`→`@Throws(E::class)`；无注解的 Java 类型写非空、按参照逐处判可空；端口头是 `// femacs: port of java/… @ fd7afced403.`（不带参照的许可块，`check-declarations` 会扫 `.kt`）。
  ② **头三个文件**（叶子，`femacs/platform/android/kotlin/`）：`EmacsHolder.kt`、`EmacsDirectoryEntry.kt`、`EmacsHandleObject.kt`（0/40 → **3/40**）。
  ③ **编译验证工具** `spikes/kotlin-compile-probe.sh`：本机没有 `kotlinc`，但 Gradle 9.7.1 自带 `kotlin-compiler-embeddable-2.4.0.jar`、SDK 有 `platforms/android-36/android.jar` ⇒ 用 `java -cp "<gradle lib>/*" org.jetbrains.kotlin.cli.jvm.K2JVMCompiler` **离线**编译（实测：只把编译器自带三个 jar 放进 `-cp` 会 `NoClassDefFoundError: kotlin/reflect/...`，整个 lib 目录就够）。三个文件**编译通过**——这是本项目里第一个由编译器验证过的平台目标。
  ④ **JNI 名字的实测证据**：`javap -p` 显示 `EmacsDirectoryEntry` 编出 `public int d_type; public java.lang.String d_name;`，正对 `androidvfs.c` 366-369 的 `("d_type","I")` / `("d_name","Ljava/lang/String;")`；`EmacsHandleObject` 的 `handle` 对 `android.c` 2082 的 `("handle","J")`、`destroyHandle` 对 2076/3099 的 `"()V"`。
  ⑤ **顺序已备**：40 个 Kotlin 目标的依赖图 + 拓扑序落 `docs/baseline/kotlin-order.tsv`（叶子 5 个：`EmacsDialogButtonLayout`/`EmacsDirectoryEntry`/`EmacsDocumentsProvider`/`EmacsHandleObject`/`EmacsHolder`；已落 3，剩 `EmacsDialogButtonLayout`(152 行) 与 `EmacsDocumentsProvider`(578 行)）。
  ⑥ **顺带修一条门**：`check-citations` 的 1 条 stale 是我 3.277 在 `termhooks.rs` 插入后行号漂走的 allowlist 行（`--fix` 早在 3.166 被移除，理由是"引用是一句话、不是一个字段"）⇒ 手工改到 365，现 `0 stale / 38 allowed`；剩 30 drift 全在 lane 在飞的 Swift 里。下一步：按 `kotlin-order.tsv` 推完 37 个 Kotlin（先两个叶子），再立 Haiku C++ 的节（8,264 行，5 文件）。

- **3.288 Kotlin 批的第 5 个、也是最后一个叶子：`EmacsDocumentsProvider`（578 行 Java → 476 行 Kotlin），并给探针补上 `R`（2026-10-05）**：
  ① **文件**：`femacs/platform/android/kotlin/EmacsDocumentsProvider.kt`（`DocumentsProvider` 的 SAF provider：`queryRoots`/`queryDocument`/`queryChildDocuments`/`openDocument`/`createDocument`/`deleteDocument`/`removeDocument`/`renameDocument`/`moveDocument`/`isChildDocument`/`getDocumentType` + 三个私有帮手）；与已有四个一起**编译通过**（5/40）。
  ② **两类 Java 形状在 Kotlin 里没有对应**（都记在文件头）：`flags |= X` → `flags = flags or X`；`while ((length = stream.read (buffer)) > 0)` → `while (true)` + 读一行 + `break`（与 C 侧处理 C 的赋值表达式同形）。另外 Java 的形参在 Kotlin 里不可变，所以 `projection` 覆盖写改成局部 `columns`。
  ③ **空安全是实测出来的**：本机 SDK 的 Kotlin 签名里 `ContentProvider.getContext()` 是 `Context?`（Java 参照所用的签名是非空），于是 5 处按"参照在同点解引用"写 `!!`——与 Java 在该情形的 NPE 等价（P-83 ⑤）。另有两处编译器 warning（`file.parent` 是 `String?` 而 `notifyChangeByName` 收非空），那是**参照自己的**未判空调用，端口照抄并记录，不"修好"它。
  ④ **`R` 的来源**：`R` 是 aapt 的产物，而 `java/res/**`（18 个文件）与 `java/` 下的构建文件**不在 `docs/port-map.md` 的任何表里**（map 只有 40 个 `.java`）⇒ 新工具 `spikes/android-r-from-res.py` 从**参照自己的 `res/` 树**生成 `R.kt`（文件名与值文件里的 `<tag name=…>`/`<item type=… name=…>` 真解析，只有*值*是 0——真值要 aapt），并把它接进 `spikes/kotlin-compile-probe.sh`（现在会把 `R.kt` 一起编）。**这也顺带记下 map 的一个缺口**：`java/res/**`、`AndroidManifest.xml.in`、`gradle`/`proguard` 等构建文件在 map 里没有分类。
  ⑤ 读数：`spikes/kotlin-target-order.py --check` → **40 目标 / 5 已落 / 35 待做**；五个文件全部由 `spikes/kotlin-compile-probe.sh` 编译通过。下一步按该序推进（叶子已尽，接下来是 `EmacsActivity` 710 行一类依赖它们的文件）。

- **3.289 Kotlin 第二批：图形簇四件（`EmacsDrawable`/`EmacsGC`/`EmacsPixmap`/`EmacsTileObject`，499 Java 行）——9/40，全部编译通过（2026-10-05）**：
  ① **为什么是这四个**：叶子已尽，按 `docs/baseline/kotlin-order.tsv` 的依赖图算"闭包"最小的可编译簇（脚本实测：`EmacsDrawable`/`EmacsGC`/`EmacsPixmap`/`EmacsTileObject` 的闭包 = 499 行，是当时最小的一组）。
  ② **落地**：`EmacsDrawable.kt`（接口，33 行）、`EmacsGC.kt`（198→约 190 行，含 `markDirty`/`prepareStipple`/`blitOpaqueStipple` 与 `clip_serial` 计数器）、`EmacsPixmap.kt`（170→约 150，`depth/width/height` 的 `final` 与 `destroyHandle` 的 `recycle`）、`EmacsTileObject.kt`（98→约 70）。JNI 读的字段（`android.c` 3306-3341 的 `foreground`/`background`/`function`/`fill_style`/`clip_rects`/`stipple`）全部 `@JvmField` 保名保描述符。
  ③ **P-83 又添三条实测口径**：⑦ Java 里可被覆盖的方法在 Kotlin 里要 `open`（实测 `EmacsPixmap.destroyHandle` 覆盖父类被编译器拒绝）；⑧ Kotlin 的 `Array<T>` 不变，Java 的 `Rect[]`（元素可 null）要写 `Array<Rect?>`（实测 `real_clip_rects = clip_rects` 编不过）；⑨ Java 在构造体里晚于别的工作赋值 `final` 字段时（`EmacsPixmap.width/height/depth`），Kotlin 用"声明不初始化 + `init` 同位置赋值"保住顺序。
  ④ **读数**：`spikes/kotlin-target-order.py --check` → **40 目标 / 9 已落 / 31 待做**；九个文件由 `spikes/kotlin-compile-probe.sh` 编译通过（只剩 `EmacsDocumentsProvider` 那两条"参照自己不判空"的 warning，见 3.288 ③）。下一步仍按依赖闭包挑最小簇（下一组候选已在同一条命令里算得）。

- **3.289（续）Kotlin 再落两件：`EmacsFillPolygon`（86 行）、`EmacsFillRectangle`（91 行）——11/40（2026-10-05）**：两件都是 `static perform(...)` 的绘图入口（Kotlin 用 companion member；JNI 不找它们，所以不加 `@JvmStatic`）。`Math.floor/ceil` → `kotlin.math`；Java 的 `static { … }` 初始化块 → companion 属性初始化 + `floatArrayOf`。**P-83 再添一条实测口径 ⑩**：Java 的 `protected` 方法在 Kotlin 里既不包内可见、`internal` 又会改名 ⇒ 同包别类要调的就写 public 并记一句（实测：`EmacsFillRectangle` 调 `EmacsGC.blitOpaqueStipple`，编译器先拒绝 protected，改 public 后过）。读数：**40 目标 / 11 已落 / 29 待做**，十一个文件全部编译通过。

- **3.290 Kotlin 再落两件绘图入口：`EmacsDrawRectangle`（134 行）、`EmacsDrawLine`（160 行）——13/40（2026-10-05）**：
  ① `EmacsDrawRectangle`：`static final Xfermode` → companion 属性；clip-mask 分支里的临时 bitmap/canvas 与 `setIntersect` 早退照抄；`gc.clip_mask.bitmap` 在端口是可空的 ⇒ 按参照的解引用点 `!!`（P-83 ⑤）。
  ② `EmacsDrawLine`：`Math.hypot/abs/min/max` → `kotlin.math`；`polyDashPattern` 的 `while (offset >= gc.dashes[i])` 与 dash 相位切换照抄。
  ③ **又一处"Java 形参被重赋"**（第三例，与 `EmacsDocumentsProvider.projection`、`EmacsGC` 的数组同族）：`polyDashPattern` 在循环里重赋 `x1/y1`，Kotlin 形参不可变 ⇒ 在循环前 `var px1/var py1` 两个局部承接（语义不变，已在文件里记一句）。
  ④ 读数：**40 目标 / 13 已落 / 27 待做**；十三个文件全部由 `spikes/kotlin-compile-probe.sh` 编译通过（只剩 `EmacsDocumentsProvider` 那两条参照自身不判空的 warning）。

- **3.291 Kotlin 剪贴板簇 + 光标：`EmacsCursor`/`EmacsClipboard`/`EmacsSdk8Clipboard`/`EmacsSdk11Clipboard`（461 行）——18/40（2026-10-05）**：
  ① 四件落地并编译通过：`EmacsCursor`（47，构造器早退改成单表达式）、`EmacsClipboard`（48，抽象类 + companion 的 `makeClipboard`）、`EmacsSdk8Clipboard`（133，`manager` 的 `getSystemService` 强转）、`EmacsSdk11Clipboard`（233，`@Synchronized` 三个方法、`ClipData`/`ClipDescription` 的读取、两个 catch 返回 null）。
  ② **P-83 又添一条 ⑪（实测）**：**JNI 按名字找的 `static` 方法必须 `@JvmStatic`**——Kotlin 的 companion 成员在 JVM 上住在 `X$Companion` 上，而 `androidselect.c` 110-112 是在 `EmacsClipboard` 类上找 `makeClipboard`（`()Lorg/gnu/emacs/EmacsClipboard;`）⇒ 不加就找不到。
  ③ **探针升级（这批的关键使能）**：不再要求"依赖先落库才能编"——`spikes/kotlin-compile-probe.sh` 现在把**参照自己的 `java/**`（去掉已被 Kotlin 覆盖的那些名字）**一起交给编译器，作为未落库依赖的替身（就是端口将来要复现的那套接口）⇒ **每个 Kotlin 文件一落地就能真编译**。它证明的是被移植文件的语法与类型对参照接口成立；"与端口自己后续文件的接口一致性"仍由 `docs/baseline/kotlin-order.tsv` 的簇闭包在落库时核（这条限制已写进探针注释）。
  ④ 读数：**40 目标 / 18 已落 / 22 待做**，十八个文件全部编译通过（只剩 `EmacsDocumentsProvider` 那两条参照自身不判空的 warning）。

- **3.292 Kotlin 又三件（`EmacsLauncherPreferencesActivity` 31、`EmacsMultitaskActivity` 57、`EmacsThread` 70）——21/40（2026-10-05）**：三件都编译通过。`EmacsThread` 里保留了参照的 `System.arraycopy` 与 `Arrays.toString`（日志文本是运行期对拍会看的东西，P-83 ①）；`EmacsMultitaskActivity` 的两个覆盖在参照里是 `final` ⇒ 端口写 `final override`——**P-83 新增 ⑫**（Kotlin 覆盖成员默认 open，不写 `final` 就与参照的封闭性不符）。
  读数：**40 目标 / 21 已落 / 19 待做**（剩 12,600 余 Java 行，集中在 UI/服务那几个大件）；二十一个文件全部由探针编译通过。

- **3.294 两条交付入库 + 四棘轮重生成；镜像覆盖率整树归零（2026-10-05）**：
  ① **入库**：`887f689` xterm lane 补完 `include/xterm.rs` 的 x11 臂（1,439 → 1,570 行，82 条成员：`x_display_info` 的 WMTYPE 枚举、EWMH/SM/XDND atom 族、`x_dnd_atoms` 三元、motif 拖拽 atom、`modmap`、屏幕尺寸、failable-request 计数，外加 5 条 `HAVE_XKB` 虚拟修饰键 atom；`x_output` 的 `black_relief`/`white_relief` 与 C 定义在 `x_output` 体内、端口提升到同级的 `struct relief`）；`c124faa` nsterm 第 13 片（C 60-2960，四个函数框、`ns_clear_frame` 导出、`copyRect:to:` 记作该 lane 首次遇到的多段选择器）。
  ② **镜像覆盖率整树归零**：`spikes/mirror-struct-coverage.py` 由 3.282 的 **114 条缺口** → **0 条**（282 个结构体、846 个臂尝试、241 个臂"读不了"=该配置不定义该类型或平台头不在）。另外两条口径由 lane 写进台账：端口在 `x_display_info`/`x_output` 上的 `have_x_windows` 门是**按类型可得性收窄**（C 那两个结构无门），以及该工具的 x11 臂**不带扩展宏**（`HAVE_XFIXES`/`HAVE_XSYNC`/`HAVE_XRANDR`/`HAVE_X_I18N`/`HAVE_XINPUT2`）⇒ 这些门下的字段不在读数里（记为工具待办）。
  ③ **棘轮**（3m14s，脚本内 `--strict` 已过 ⇒ 棘轮文件是 HEAD 一致的）：`swift-units.tsv` **-10 行**（正是 nsterm 13 片列的 10 条）且 `check-swift-units` 读 **0 new / 0 stale / 7 landed**；`swift-import-pending.txt` 621 → **636** 行（nsterm 交来的 16 条）；`link-pending` 373、`link-exec-pending` 83。**工作树当前的 FAIL/NEW 是四条 lane 在飞造成的**（`check-link` 的 `_Vmake_cursor_line_fully_visible` 属 xdisp 第 23 轮、`check-swift-imports` 的 26 条同理），HEAD 侧一致。
  ④ **一条留给下一轮的核对**：对账脚本把"改动过的镜像 → 包含它的单元"是靠 `grep -l "include/<mirror>"` 找的；若某镜像**被另一个镜像 include**（`include!` 链），它的单元不会被找到、对象可能带在飞状态。本轮脚本的 `--strict` 是绿的，所以这次没有咬到，但值得专门验一次（记在案）。
- **3.295 产品名声明这条长期指令的复算，抓到一个真洞：`Kept` 的六个汇编文件对每一道门都不可见（2026-10-05）**：
  ① **指令的读法不变**（3.226/3.250）：`版本声明不要再说是gnu emacs了` 与 38 秒之后的 `把之前的代码中的这种声明也去掉` 是**同一条指令的前后两半**，
  不只对新写的文件生效、**已写进树的代码**也要清；三段处置（231 个首行版本括注压到修订号、268 个版权/许可块不抄、标题注释与"复述许可块"进 `title`/`recital`）一处没退。
  本轮在 HEAD `ef5b099` 的工作树上复算，逐类按**后缀**扫全仓。
  ② **复算的读数**：`check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`、`read 652 files (623 .rs 22 .kt 7 .swift)`——
  **但门自己打印的下一行就是洞**：`walked past 7 file(s) of an unscanned kind: femacs/exec/loader-{aarch64,armeabi,mips64el,mipsel,x86,x86_64}.s, femacs/xtask/Cargo.toml`。
  六个 `.s` 是 `docs/port-map.md` 的 **`Kept`** 类（3.284 逐字节复制、`cmp` 验过），每个文件的**头 16 行就是参照的许可块**
  （`Copyright (C) 2023-2026 Free Software Foundation, Inc.`、`This file is part of GNU Emacs.`、`GNU Emacs is free software`、`along with GNU Emacs.`），
  而 `.s` 不在 `SRC_SUFFIXES` 里 ⇒ **36 行声明对每一道门都不可见**。这是 3.250 那个洞的**第二个实例**：那次是 `femacs/platform` 的 7 个 `.swift` 只按"名字"被扫，
  这次是 `femacs/exec` 的 6 个 `.s`——同一类失败模式（门声称读了、其实一个文件都没读），同一种抓法（门打印它走了过去什么）。
  ③ **门的覆盖按后缀扩到汇编，且有正面证据**：`SRC_SUFFIXES` 加 `.s`/`.S`，并按**方言**建模行注释——`@` **任意位置**是注释（ARM；`loader-armeabi.s` 实测 188 行），
  `#` **只在行首**是注释（ARM/AArch64 的 `#` 同时是立即数前缀：aarch64 56 行、armeabi 82 行；而 x86/MIPS 的 `#` 注释**行中 0** 行，所以"只在行首"既不漏也不误）。
  `--selftest` **22 → 37 例 / 0 失败**，新增 5 例：3 例是汇编形状（`#` 方言的许可块、`@` 方言的许可块、两块的 `decl`+`title` 读数），
  2 例是**切分规则本身**（`\tmov\tx0, #1\t\t// x0 = 1` 里 `#` 之后仍是代码、缩进后的 `#` 是注释——这两例在"`#` 任意位置都是注释"和"只认行首 `#`"下会分别失败）。
  加上后缀之后门第一次读这六个文件，**当场报 54 处 `decl`/`title`**（每文件 9 处：**7 `decl` + 2 `title`**，六个文件都一样）——这就是"覆盖不是空跑"的证据。
  ④ **清理**：六个文件的 16 行注释块换成一行出处行 `<该文件自己的注释符> femacs: kept from exec/loader-X.s @ fd7afced403.`
  （`//` aarch64、`@` armeabi、`#` 其余四个；块后面那个空行保留为正文第一行），`git diff --stat` = **6 insertions(+), 96 deletions(-)**。
  **语义不变的证据是对象级的**：`clang -target {aarch64,armv7,i386}-linux-gnu -c` 对参照与端口各编一次，得到
  `loader-aarch64` `28eb92a2e452a7d5…`、`loader-armeabi` `0704694397973891…`、`loader-x86` `579eb745e079ba4c…`，**三个对象逐字节相同**；
  正文本身六对逐字节相同（`diff <(tail -n +17 emacs/…s) <(tail -n +2 femacs/…s)`）；`loader-mips64el`/`loader-mipsel` 本机 clang **没有 MIPS 后端**
  （`error: unknown target triple`）⇒ 这两个的证据是文本的（被删的 16 行全是该文件自己注释符开头的注释行）。
  ⑤ **3.284 ③ 命名的那道门做出来了：`tools/check-kept.py` + `docs/baseline/kept-files.tsv`**。行集由 `docs/port-map.md` 的 `Kept` 节决定；
  每行钉三件事——参照整文件 sha256、被替换行数（`removed_lines`，由行数差**导出**而非手写）、首行（并按 `PATH_RULE` 要求它点名参照路径与修订号），
  之外还要求**正文逐字节**。读数 `6 kept file(s), 6 baseline row(s), 0 failure(s)`；`--selftest` **3 例 / 0 失败**（干净文件 0、正文翻一个 bit 报 1、删掉首行报 **2**——
  首行与正文两个读数是独立的）。绑定是双向的：地图说 `Kept` 而基线没有行、基线有行而地图不说 `Kept`，都算失败；`--write` 会拒绝记录不是出处行的首行。
  ⑥ **残下的 13 处与边界不变**（3.226 ④⑤ / 3.250 ⑤）：13 处仍是允许清单上的 C 字面量（横幅要打印、docstring 门要逐个比对，改它就与参照分叉）；
  六棵逐字节树按用户指令原样（`check-verbatim` 钉着），运行时那行产品名由 `femacs/lisp/version.el:111` 的 `format` 拼出、该文件与参照 `cmp` 相同。
  我们自己写的散文仍可"**提到**"参照（基线行、决定记录），但任何文件都不许"**声明自己是该产品**"。**新口径**：`Kept` 类也在这条规则之内——
  它现在有 `PORTING.md` 的 **P-84** 与两道门（声明门按后缀读 `.s`、`check-kept` 钉正文）。
  ⑦ **顺手量到的生成物读数（本轮不改，已命名）**：`Kept` 行在地图里的"目标行数"列仍是 `-`（地图在 3.284 复制之前生成），
  而 `gen-port-map.py` 现在会数出来（214/249/343/342/282/254）；在 HEAD 工作树上重生成，`docs/port-map.md` 与提交版差 **56 行**（diff 输出 170 行），
  其中 50 行是 1:1 Rust 侧的目标行数（`src/comp.rs` 38→48、`lib/stdlib.in.rs` 274→300 这类，且这两个文件在工作树里是**干净的**）⇒
  地图在 HEAD 上**确实是陈的**，值得一次"在 HEAD 里重生成生成物"的专门提交（协议 §1 的新鲜度链那一步）。
- **3.296 生成物在 HEAD 上重生成：地图 52 行、inventory 11 行都不再是陈的（2026-10-05）**：
  ① **方法**（协议 §1 的新鲜度链那一步，`Kept` 行的目标行数正是被 3.295 改动的读数，所以这一步不能不做）：一次性 worktree 在 **`7f47065`** 上
  （`git worktree add --detach HEAD` + 把未跟踪的只读参照 `emacs/` 软链进去；参照与 `build/` 都不在版本控制里，所以 worktree 里必须软链），在**那里**跑
  `tools/gen-port-map.py`、`tools/gen-port-inventory.py`，把结果拷回。**不拿脏工作树生成**是硬要求：主工作树里有五条 lane 的在飞文件（xdisp 19,197 行、xterm、abi_shim、dispextern、nsterm.swift），
  用脏树生成会把在飞状态写进生成物。
  ② **读数**：`docs/port-map.md` **52 行变了**（行数 787 不变、**0 个增/删 hunk**，全是"目标行数"列：`src/comp.rs` 38→48、`lib/stdlib.in.rs` 274→300、`src/alloc.rs` 8,258→8,259 …），
  其中 `Kept` 六行的"目标行数"从 `-`（地图在 3.284 复制之前生成）变成 **214/249/343/342/282/254**（= 参照行数 − 16 行块 + 1 行出处行）；
  `docs/port-inventory.md` **11 行变了**（行数 682 不变、**0 个增/删 hunk**）：汇总 `done / wip` **580/10 → 587/3**，状态列方向是**混合的**
  （`lib/c-ctype.h`、`lib/stdlib.in.h`、`src/emacsgtkfixed.h`、`src/font.h`、`src/frame.h`、`src/w32.h` 等 wip→done；`src/xterm.c` done→wip——HEAD 的台账还有未勾项）
  ⇒ **提交的那一对不是在任何一个干净 HEAD 上生成的**（3.284 之后主工作树就一直在飞），这就是"生成物 vs 工作树"双向对账才能看到的东西（3.202 同族）。
  ③ **未测的两位**：`gen-baseline.py`（写 `docs/baseline/` 一整目录）与 `gen-lifetimes.py`（`LIFETIMES.tsv`）本轮**没跑**——它们与本次改动（汇编注释、门的后缀表）无关联，
  且重生成面大；命名给下一轮或专门的"新鲜度链"提交。读数到此为止：这条链本轮覆盖 `gen-port-map` + `gen-port-inventory` 两位。
  ④ **复检**：拷回后 `git diff --stat` = 2 文件 / 63 insertions / 63 deletions；`check-declarations` 仍 `13 / 0 / 0`（`read 661 files`，含 6 个 `.s`）、
  `check-kept` 仍 `6 / 0`；worktree 用完即删（`git worktree remove --force`）。
- **3.297 Kotlin 又四件：字体驱动三件 + `EmacsApplication`，25/40；JNI 的 28 个字段名与 9 个方法名逐个对过（2026-10-05）**：
  ① **入库四件**（`EmacsFontDriver` 162、`EmacsSdk7FontDriver` 440、`EmacsSdk23FontDriver` 98、`EmacsApplication` 119；参照 Java 180/503/120/159 行）：
  `EmacsFontDriver` 是抽象基类 + 四个嵌套记录（`FontSpec`/`FontMetrics`/`FontEntity`/`FontObject`，其中 `FontObject` 在 Java 里不是 static ⇒ 端口写 `inner class`，
  JVM 名仍是 `EmacsFontDriver$FontObject`），八个入口保持 `fun`、`createFontDriver` 在 companion 上带 `@JvmStatic`（P-83 ②⑪）；两个 driver 是它的具体实现。
  **一处真缺陷已修**：`EmacsApplication.findDumpFile` 的 `val apk: File` 编译不过（`'val' cannot be reassigned`）——参照的 `File apk;` 声明在**方法作用域**、而在 `for` 循环里**每次迭代**赋值（`apk = new File (apkFileName);`），
  Kotlin 的 `val` 只能赋值一次 ⇒ 改成 `var apk: File`（声明顺序、赋值点、语句形状与参照一致，这是 P-83 ⑨ 那条规则的"多次赋值"情形）。
  ② **验证（我自己跑的，不引用 lane 的读数）**：`spikes/kotlin-compile-probe.sh` 对四个 `.kt` + 参照 `java/**` 替身 → **exit 0、零诊断**（`EmacsNative.kt` 还没落，探针按设计用参照的同名 `.java` 当替身，所以这一条的通过面是"端口的语法与类型对着**参照的接口**"）。
  **JNI 名字逐个对**（`emacs/src/androidfont.c` 是唯一按名字找这些类的 C 文件）：`FIND_METHOD` 的八个名字 `list`/`match`/`listFamilies`/`openFont`/`hasChar`/`textExtents`/`encodeChar`/`draw` 与 `GetStaticMethodID("createFontDriver")` **全在**；
  `FIND_FIELD` 的 **28 个字段名字符串**（`foundry`/`family`/`adstyle`/`registry`/`width`/`weight`/`slant`/`size`/`spacing`/`avgwidth`/`dpi`、`lbearing`/`rbearing`/`ascent`/`descent`、
  `minWidth`/`maxWidth`/`pixelSize`/`height`/`spaceWidth`/`averageWidth`/`underlineThickness`/`underlinePosition`/`baselineOffset`/`relativeCompose`/`defaultAscent`/`encodingCharset`/`repertoryCharset`）**一个不缺**；
  字段集与 Java **相等**（Java 的 6 个 `Integer` 装箱字段 → Kotlin `Int?` + `@JvmField`，其余按基本类型）。
  ③ **读数**：`spikes/kotlin-target-order.py` 重生成 `docs/baseline/kotlin-order.tsv` → `40 target(s), 25 ported, 15 to go`、`--check` 绿；
  Kotlin 侧剩 15 个目标（`EmacsView`/`EmacsWindow`/`EmacsWindowManager`/`EmacsService`/`EmacsNative`/`EmacsActivity`/`EmacsSurfaceView` 等）。
  ④ **本轮没有 Rust 编译**（只跑了一次 Kotlin 探针），探针的输出目录是它自己的 `mktemp -d` + `trap` 清理，工作树里没有新产物。
- **3.298 abi shim 第⑩批 26 个导出入库（台账收口），四棘轮按新 HEAD 重生成：swift-import 636 → 610（2026-10-05）**：
  ① **入库**（`6b24abc`，commit message 见下 ⑤ 的教训）：`femacs/src/abi_shim.rs` +274 行 / 0 删除、`docs/ports/abi_shim.md` 台账 +153 / -12，**26 个新导出**——
  window 侧（`window_set_left_col`/`window_set_top_line`/`window_set_pixel_left`/`window_set_pixel_top`/`window_set_total_cols`/`window_set_total_lines`/`window_set_pixel_width`/`window_set_pixel_height`/
  `window_total_cols`/`window_total_lines`/`window_pixel_width`/`window_pixel_height`/`window_phys_cursor_x`/`window_phys_cursor_y`/`window_desired_matrix`）、
  frame 侧（`frame_set_total_cols`/`frame_set_pixel_width`/`frame_set_pixel_height`/`frame_border_width`/`frame_size_hint_flags`/`frame_set_size_hint_flags`/`frame_set_config_scroll_bar_width`/`_cols`/`_height`/`_lines`）、
  `set_hash_value_slot`。台账 `docs/ports/abi_shim.md` 的**未勾框归 0**（文件开头写着理由：还有未勾项的文件会被 `xtask` 的 `finished_units()` 跳过，于是 shim 不会被 `gate-fast`/`dead-arms` 构建、它已经提供的导出反而读成"缺"）。
  ② **入库前我自己复检（不引用 lane 的读数）**：`build-tu src/abi_shim.c` **exit 0**（`grep -c '^error'` = 0；输出里那行 `For more information about this error, try rustc --explain E0133` 是 E0133 警告组的收尾句，不是错误）；
  `check-declarations` **13 / 0 / 0**、`check-kept` **6 / 0**、`check-mirror-dupes` **1 dup / 1 棘轮行 / 0 failure**。**`extract-abi.py --check` 对这件不适用**——它不是参照文件的复刻（C 里没有这些名字），首行是 `check-target-names.py` 接受的 DECLARED 形式
  （`gen-port-map`/`gen-port-inventory` 重跑确认它不进任何一行；台账的 `## Evidence` 记着这条）。
  ③ **四棘轮重生成**（`tools/reconcile-link-ratchets.sh`，**3m02s**，在它自己造的 HEAD worktree 里跑）：`docs/baseline/swift-import-pending.txt` **636 → 610 行**（**−26，正好是本批的 26 个名字**——台账里写着"batch 10 的 26 行等集成者的下一次 `--write-pending`"，本轮兑现）；
  `link-pending.txt`、`link-exec-pending.txt`、`swift-units.tsv` **逐字节不变**（本批的名字全是 Swift 导入行，不是链接行）。`git diff --numstat` = **1 文件 / 0 insertions / 26 deletions**。
  ④ **重生成后的读数（工作树，即带着四条 lane 的在飞文件）**：`check-link` `373 undefined / 373 行 / 1 failure / 1 stale(informational)`，那 1 条 failure 是 `_Vmake_cursor_line_fully_visible (unknown): referenced and defined by nothing`
  （xdisp 在飞：`DEFVAR` 的 Lisp 符号在它的工作副本里被引用、还没到定义处）；`check-link --executable` `83 / 83 行 / 1 failure`；`check-swift-units` **0 new / 3 stale / 7 landed**（3 条 stale 全点名 `femacs/platform/macos/nsterm.swift`，是 lane 已落、棘轮行还停在 HEAD 的那几个名字）；
  `check-swift-imports` `931 import(s) in 7 Swift file(s), 5,687 core export(s), 632 unprovided, 610 ratchet row(s), **22 failure(s)**`——**22 条全部点名 `femacs/platform/macos/nsterm.swift`**（`CHAR_GLYPH`/`COMPOSITE_GLYPH`/`composition_lbearing` 这类它在飞开始导入的名字），
  即**没有一条 failure 落在 HEAD 上**（HEAD 侧的门与棘轮一致，这正是"先在 HEAD 里造读数、再对账"那条顺序的意义）。
  ⑤ **教训（工具层，写在这里以免重犯）**：`3440f71`（Kotlin 四件）的 commit message 里两处反引号被 **shell 命令替换**吃掉了（`\`val apk: File\`` 变成空、`apk` 一词变成 `cat` 的输出错误提示）；
  该提交**已推送**，而 `git push --force` 是硬禁止 ⇒ **不重写历史**（内容与证据以本条与 3.297 为准）。从此提交信息一律写进文件、用 `git commit -S -F <file>`（`6b24abc` 已是这么做的）。
  ⑥ **清理**：本轮我自己造的两个临时目录已删（`/tmp/femacs-verify` 的 cargo target、`/tmp/{bt-abishim,kotlin-probe}.txt`）；`build/femacs/` 里的对象是工程正常状态、按 3.284 的口径保留。
- **3.299 五个 C++ 目标一次性落地（地图的 `C++` 节 8,264 行），载文门从 `Kept` 扩到两类：`check-kept` → `check-carried`（2026-10-05）**：
  ① **落地**：`femacs/platform/haiku/{haiku_support.cc 5,661 行, haiku_font_support.cc 1,193, haiku_select.cc 647, haiku_draw_support.cc 520, be_resources.cc 163}` —— 参照是 5,677/1,209/663/536/179（**共 8,264 行 → 端口 8,184，每件 −16**：17 行 `/* … */` 声明块换成 1 行出处行），
  首行逐个是 `// femacs: port of <参照路径> @ fd7afced403.`，**正文五对逐字节相同**（`diff <(tail -n +18 emacs/…) <(tail -n +2 femacs/…)` 全过）。
  ② **为什么是"承载文本"而不是 3.31 的空单元**（P-85 ②⑥）：目标语言＝源语言——`docs/port-plan.md` §4 batch F 对这批的写法是 "`*.cc`→C++"，也就是**保留 C++** ⇒ 端口就是文本本身，没有要发明的语义；C 侧的 Haiku 单元（`femacs/src/haikufns.rs` 等 11 行 + 台账）仍按 3.31 留空单元，因为那要把 C 重写成 Rust、而本机没有工具链可验证。丢掉这 8,264 行才是信息损失。
  ③ **门跟着扩**：`tools/check-kept.py` → **`tools/check-carried.py`**、`docs/baseline/kept-files.tsv` → **`docs/baseline/carried-files.tsv`**，行集＝地图 `Kept` 节 ∪ `C++` 节（**11 行 = 6 + 5**）。每行四读：参照整文件 sha256、目标首行（并按**类别**要求短语——`Kept` 写 `kept from`、`C++` 写 `port of`，因为 `C++` 的路径要从 `src/`/`lib-src/` 移到 `femacs/platform/haiku/`）、正文与参照 `removed_lines` 之后逐字节相同。
  读数 **`11 carried file(s), 11 baseline row(s), 0 failure(s)`**；`--selftest` **8 例 / 0 失败**（每类 4 例：干净过、正文翻一个 bit 报 1、删首行报 2、**换成别类的短语报 1**——最后这条是类别规则的正面证据）。
  ④ **声明门的覆盖第三次扩张**（3.250 的 `.swift`、3.295 的 `.s` 之后）：`check-declarations.py` 现在读 **666 文件（623 .rs + 25 .kt + 7 .swift + 6 .s + 5 .cc）**，`femacs/platform` 的 **`.cc` 半边第一次进扫描面**，读数仍是 **13 sites / 0 forbidden / 0 failures**（新落地的 5 件零命中：块已去）。
  ⑤ **P-84 ④ 的工具名/基线名更新**到 `check-carried`/`carried-files.tsv`；**P-85 立规**（C++ 目标的路径、载文、名字不许变、SDK 头是"另一个目标的工具链头"、编译证据只能来自 Haiku target）。实测记录：本机 `clang++ -std=c++23` 可用（纯 C++ 片段 exit 0），但 `-fsyntax-only` 到这五个文件立刻 `fatal error: 'View.h' file not found`（exit 1）——所以 P-85 ④ 把"编译"记成 Haiku target 的事，而不是发明 SDK stub 头。
  ⑥ **清理**：本轮临时脚本与临时 inventory 读数（`/tmp/land-cpp.py`、`/tmp/patch-porting.py`、`/tmp/inv-now.md` 等）已删；没有跑 cargo/rustc。
  **⑦ 一条就地更正（3.301）**：`6674342` 里 `git mv` 只把**改名**入了索引，两个文件的新**内容**（新门 + 新基线）当时还在工作树里没 `git add` ⇒ 那个提交上的 `tools/check-carried.py` 仍指向已改名的旧基线（该 HEAD 上这个门跑不起来）。内容已随 `3.301` 补齐并复核（下面那条）；教训与 3.298 ⑤ 同族：**`git mv` 之后必须再 `git add` 内容**，改名与内容是两个动作。
- **3.300 地图在落地之后重生成：9 个"目标行数"格子（5 个 C++ + 4 个 Kotlin）（2026-10-05）**：
  ① **方法同 3.296**：一次性 worktree 在 `6674342` 上（`git worktree add --detach` + 软链未跟踪的参照 `emacs/`），在那里跑 `tools/gen-port-map.py`，拷回；`docs/port-inventory.md` **零行变化**（它迭代的是 590 个 C 文件，`C++`/`Kotlin` 目标不在它的行里——跟踪它们的是地图的 `C++` 节与 `docs/baseline/carried-files.tsv`）。
  ② **读数**：地图 **9 行变、0 个增/删 hunk** —— `C++` 五行的目标行数从 `-` 变成 **163/520/1,193/647/5,661**（= 8,264 − 5×16 + 5），
  Kotlin 四行（`3440f71` 落的三件字体驱动 + `EmacsApplication`）从 `-` 变成 **119/162/98/440**。
  ③ 复检：`check-declarations` 13/0/0（`read 666 files`）、`check-carried` `11 / 11 / 0`；worktree 用完即删。
- **3.302 dispextern 覆盖率通道入库：`redisplay_interface` 的 15 个窗口系统成员 + `image.type` → `r#type`（Boyle 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/include/dispextern.rs` 3,565 → **3,607 行**（`struct redisplay_interface` 的 `HAVE_WINDOW_SYSTEM` 臂 15 个成员，C 3131-3195：`draw_fringe_bitmap`/`define_fringe_bitmap`/`destroy_fringe_bitmap`/`compute_glyph_string_overhangs`/`draw_glyph_string`/`define_frame_cursor`/`clear_frame_area`/`clear_under_internal_border`/`draw_window_cursor`/`draw_vertical_window_border`/`draw_window_divider`/`shift_glyphs_for_insert`/`show_hourglass`/`hide_hourglass`/`default_font_parameter`，每条各自 `#[cfg (feature = "have_window_system")]`）；
  `struct image.type` 从 `type_`（回避关键字）改回 **`r#type`**——本文件其它 `type` 成员（`bidi_saved_info` 1811、`bidi_it` 2608）就是这个拼法，覆盖率工具也认 `pub (r#)?<name>:`；台账 `docs/ports/include-dispextern.md` 944 → 978 行（新增那条 Decisions 与 Evidence 段），外加它的 `docs/baseline/image-layout.tsv`（87 行，上一轮 `--write` 的读数）。
  ② **入库前我自己复检**（不引用 lane 读数）：`extract-abi --check emacs/src/dispextern.h …` → **OK**（`1 C macros have a recorded Rust counterpart: NativeRectangle`）；`xtask layout-check` → **`layout-check passed`**；`build-tu src/xfaces.c`、`build-tu src/window.c` 均 **built**；
  `spikes/mirror-struct-coverage.py` → **`282 mirrored struct(s): 846 arm(s) attempted, 241 unreadable …, 0 gap(s)`**（与 3.294 的整树读数**逐字相同**：0 gap / 241 unreadable，所以这批既没引入缺口也没把臂变成"读不了"）；`check-mirror-dupes` 1 dup / 1 棘轮行 / 0 failure；`check-carried` 11 / 11 / 0；`check-declarations` 13 / 0 / 0。
  ③ **lane 自己写明、我照抄进台账的边界**（不许当成"已验证"）：这 15 个成员**在本 target 的任何树内门里都不存在**（frozen 不定义；`have_window_system` 一打开，整树按 P-82 在第一个受门存取器处编不过），所以证据只有覆盖率读数 + lane 的"剪段探针"（把镜像里这段原文剪出、给 6 个被引用记录打桩，`rustup run nightly-2026-10-01 rustc --cfg 'feature="have_window_system"' …` exit 0，并带 `size_of::<redisplay_interface>() == 224` 通过）；
  **它们没有布局读数**（全是 8 字节函数指针，带门臂没有可测的探针臂），**函数签名也没有与任何真实调用点对过**（本 target 没有 window-system 单元）。这三条与 3.262 ⑥ 的"承载文本、编译未验证"同一形状，等一个窗口系统 target 才能翻牌。
  ④ 读数：改动面 3 个文件（+42 行镜像、+34 行台账、台账基线 87 行）；`layout-check`/`build-tu` 各一次；无 cargo 之外的产物（lane 的 `/tmp/imgcov`、`/tmp/femacs-target-img3` 它自己已删）。
- **3.303 xterm 第 4+5 片入库：2,403 → 2,844 行（Ohm 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/xterm.rs` **+964 / −23**（2,403 → **2,844 行**，含 lane 的片 4 与片 5 两轮）、`docs/ports/xterm.md` 965 → **1,023 行**（**306 个未勾框**＝这条战线剩下的清单）。
  ② **入库前我自己复检**：`build-tu src/xterm.c` → **built**；`extract-abi --check emacs/src/xterm.c femacs/src/xterm.rs` → **313 条 `- `**（`missing function XTcondemn_scroll_bars`/`XTflash` …）加 **`line count drifted 91.4%: C 33,125 vs Rust 2,844`**——
  这是**中段单元的既有状态**（3.249 片 1 之后是 443 条；台账的未勾框数就是它的进度），不是本片的缺陷；镜像覆盖率仍是 **0 gap**（3.302 ② 的同一读数；x11 臂的 `x_display_info` 与 `x_output` 两条缺口正是这条战线的镜像工作，已归零）。
  ③ **下一片 lane 自己命名**：C 3,013 `x_dnd_compute_toplevels`（含 `#ifndef USE_XCB` 与 `HAVE_XCB_SHAPE`/`HAVE_XCB_SHAPE_INPUT_RECTS` 两臂），之后是 3,060+ 的 XDND 路径。
  ④ **它提的那个待裁决已判**（见 3.304）：`uint*_t` 一族进 `MACRO_TOKEN_SKIP`，于是本单元里那 12 处"每处写一行"的登记一起消失（313 → **301**）。lane 未 commit/add/push、未动 `docs/baseline/**`，它的 `/tmp` 缓存已自己删。
- **3.304 共享门的一次定向放宽：C 的定宽整数名进 `MACRO_TOKEN_SKIP`（2026-10-05）**：
  ① **它是什么**：`tools/extract-abi.py` 的宏体比对把两侧 token 去掉一批"两种语言拼法不同的类型名"（`int`/`unsigned`/`size_t`/`u8`/`i64`/`ptrdiff_t`/`c_uint` …）后再比多重集；**C 的定宽拼法**（`int8_t`…`uint64_t`）此前不在表里，于是 xterm lane 遇到一处就得写一行登记（3.303 ④）。
  ② **判据**：这与表里已有的条目是**同一类**（`uint32_t` 与 `u32` 指同一个类型，一侧已跳过、另一侧没跳过只是不对称）；而且跳过更多**类型** token 只会**减少**差异、不可能新增差异——比对的目的（3.243 的 F2：删掉的操作不许被笔记放过）针对的是**操作名**，不是类型名。
  ③ **实测**：`extract-abi --check` 的 `- ` 条数 `src/xterm.c` **313 → 301**（正是那 12 处）；对照三个已知干净单元不变——`src/casetab.c` **0**、`src/doprnt.c` **0**、`src/sort.c` **0**（放宽后仍 0 ⇒ 无回归）。
  ④ **口径**：这是一次**定向**放宽（C 的 8 个定宽整数名，成组写在宏体豁免表里并附理由），不是给宏体比对开新口子；`docs/ports/macro-body-notes.tsv`（逐处笔记）与 `token-aliases.tsv`（身份别名）仍是各自的既定家。
- **3.305 落地 3.302/3.303 之后重跑四棘轮：逐字节不变（读数如此，不是"没跑"）（2026-10-05）**：
  ① **跑了**：`tools/reconcile-link-ratchets.sh`（**2m15s**，在它自己的 HEAD worktree 里），四点全部重生成 ——
  `git diff --numstat docs/baseline/` **空**：`link-pending.txt`、`link-exec-pending.txt`、`swift-units.tsv`、`swift-import-pending.txt`（仍 **610 行**）**逐字节不变**。
  ② **为什么不变是对的**：3.302 只往**镜像**里加了 15 个带门函数指针成员与一个字段改名（镜像不导出任何符号）；3.303 的 xterm 片 4/5 新增的是 **X11 专属名字**，它们在冻结 target 上本来就在 `unknown` 类里被计数（3.249 ③ 的口径），
  链接级未定义表因此没有新行，Swift 侧一个 import 也没变。
  ③ **重生成后的读数（工作树）**：`check-link` **373 undefined / 373 行 / 1 failure / 1 stale(informational)**（那条 failure 仍是 xdisp 在飞的 `_Vmake_cursor_line_fully_visible`）；`--executable` **83 / 83 / 1 failure**；
  `check-swift-units` **0 new / 3 stale / 7 landed**（3 条 stale 全点名在飞的 `nsterm.swift`）。**没有一条落在 HEAD 上。**
- **3.306 xdisp 第 23 轮入库：C 19256-19862（607 行 C → 640 行 Rust），台账 302 → 308/415、缺口 113（Hypatia 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/xdisp.rs` **+654 / −4**（19,197 行）、`docs/ports/xdisp.md` **+139 / −55**（未勾框 **107**，缺口分类 126 → **113**）。
  本轮四个函数 + 两个宏 + 两组匿名枚举常量：`run_window_scroll_functions`(C 19263)、`cursor_row_fully_visible_p`(C 19304)、`try_scrolling`(C 19401，330 行)、`compute_window_start_on_continuation_line`(C 19744)；
  `SCROLL_LIMIT`(C 19391 → `const`)、`BASE_LINE_NUMBER_VALID_P`(C 19396 → `macro_rules!`，C 的逗号表达式落成语句块)；`SCROLLING_{SUCCESS,FAILED,NEED_LARGER_MATRICES}`(C 19380-19385)、`CURSOR_MOVEMENT_{SUCCESS,CANNOT_BE_USED,MUST_SCROLL,NEED_LARGER_MATRICES}`(C 19856-19862)；
  跨单元 extern：`window_scroll_margin`(window.h 1226，按 P-72 rule 3 带 `/// C:` 就地声明)、`safe_run_hooks_2`(keyboard.c)；删 scaffold 里 `cursor_row_fully_visible_p` 的前向声明（块内 `fn` 15 → 14）。分类读数（lane 逐类对 base 与现文件 diff 过）：`missing function` 83 → **79**、`missing macro` 8 → **6**、`missing constant` 8 → **0**、`does not mention` 2 → **3**（新增的正是 `try_scrolling`/`goto`）、`stub or empty body` 0 → 0。
  ② **入库前我自己复检**：`build-tu src/xdisp.c` → **`built src/xdisp.rs -> ../build/femacs/xdisp.o`**（exit 0）；`extract-abi --check emacs/src/xdisp.c femacs/src/xdisp.rs` → **113 条**分类 + **`line count drifted 51.8%: C 39,805 vs Rust 19,197`**（中段单元的既有状态，不是本片缺陷）；
  它的四条边界自证我抽验过两条（台账四个函数框开工时全 `[ ]`、`--check` 最早的 `missing function` 是 C 15616 `tty_handle_tab_bar_click`＝item 38 的刻意缺口，其后第一个 ≥ C 20475，都在本片区间之外）。
  ③ **集成者动作**：`docs/ports/macro-body-notes.tsv` 补一行 `try_scrolling\tgoto\t…`（C 19725 的 `goto too_near_end` 是端口的 `continue too_near_end` + 跨函数其余部分的标签 loop，deviations 164-165）——加完**重跑 `--check`，那条 `function try_scrolling does not mention goto` 消失**（剩下两条 `does not mention` 是 3.305 之前就在的，不进这张表）；`include/window.rs` 的 `window_scroll_margin` 条目与 `include/dispextern.rs` 的 `pub struct it` 归属（在 `dispextern_face_records` 臂里、对本单元不可见 ⇒ item 39 仍成立）按 lane 的说明留给它自己的下一轮或对应 lane。
  ④ 未声称：本片只有 `build-tu` + `--check` 分类 + 台账边界三类证据；**没有**在冻结 target 上运行任何 xdisp 代码路径（P-51 的"承载文本"），本片新增的四个 static 函数一个都没有跨单元调用点。
- **3.307 nsterm 第 14 轮入库：三个 C 定义落地，`nsterm.swift` 3,728 行；共享表两项交集成者（Ramanujan 交付，2026-10-05）**：
  ① **入的两件**：`femacs/platform/macos/nsterm.swift` **+227 / −2**（3,728 行）、`docs/ports/nsterm.md` **+116 / −9**（未勾框 **265**）。本轮落地三个 C 定义（正是 manifest 里那三行）：
  `ns_after_update_window_line`、`ns_shift_glyphs_for_insert`、`ns_compute_glyph_string_overhangs`；台账记的累计读数：**150 个 C 定义里 83 已落 / 71 未落**、**186 个 C 符号导入**（`check-swift-units`/`check-swift-imports` 的同一读数）。
  ② **入库前我自己复检**：`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift` → **exit 0**（deprecation 警告与上一片同一批）；`check-swift-units` → **`0 new, 3 stale, 7 Swift unit(s) landed`**，三条 stale **正好点名本片的三个定义**（"manifest row no longer matches (it landed?)"）——这就是"它们真的落地了"的机器证据；
  `check-declarations` **13 / 0 / 0**、`check-carried` **11 / 11 / 0**。
  ③ **本片的两条实测机制发现**（lane 写进台账，我照抄）：⑴ **未被调用的 `@_silgen_name` 声明会被整个丢掉**——不进 `nm -u`、也进不了导入棘轮；所以本片只声明本片真正调用的名字（两个 overhang 只声明 `_set_` 半边，读条目等读到它们的那一片再声明）。这条解释了"为什么棘轮的行数不能当作'声明数'"。
  ⑵ 本片**第一次使用 3.271 的带门布局探针**（只读、未 `--write`）：`struct font_metrics` 在 frozen/x11/ns_cocoa 三臂一律 `sizeof=10, align=2`、成员 0/2/4/6/8 ⇒ Swift 侧五个 `Int16` 与 C 一致，**不需要带门臂**。
  ④ **交给集成者的共享表两项**（都由下一轮的棘轮重生成完成，见 3.308）：`docs/baseline/swift-units.tsv` 删 3 行（本片那三个名字）、`docs/baseline/swift-import-pending.txt` 加 **22 行**（4 个 `reference-source`：`font_driver`/`glyph_type`/`CHAR_GLYPH`/`COMPOSITE_GLYPH`；18 个 `unclassified`：`window_pseudo_window_p`、三个 `glyph_row_*`、十个 `glyph_string_*`、三个 `composition_*`、`font_driver_text_extents`）；
  其中 `windows_or_buffers_changed` 与两个 `composition_gstring_*` **已由核心提供**，不用行。`docs/baseline/**` 本片未动，探针未 `--write`。
- **3.308 落地 3.306/3.307 后重跑四棘轮：三个门第一次同时全绿（0 failure / 0 stale）（2026-10-05）**：
  ① **跑了**：`tools/reconcile-link-ratchets.sh`（**2m26s**），四本读数如下（`git diff --numstat`）：
  `swift-import-pending.txt` **+22**（610 → **632**，正是 3.307 ④ 那份清单：4 `reference-source` + 18 `unclassified`）；`swift-units.tsv` **−3**（`nsterm.swift` 的 `ns_after_update_window_line`/`ns_compute_glyph_string_overhangs`/`ns_shift_glyphs_for_insert`，即 3.307 本片那三个定义）；
  `link-pending.txt` 与 `link-exec-pending.txt` **各 1 行换名**：`_cursor_row_fully_visible_p` → `_Vmake_cursor_line_fully_visible`。
  ② **门读数（工作树）**：`check-swift-units` **`0 new, 0 stale, 7 Swift unit(s) landed`**（第一次全清）；`check-link` **`373 undefined / 373 行 / 0 failure`**（此前那条 `_Vmake_cursor_line_fully_visible` failure 随 3.306 落地消失）；
  `check-link --executable` **`83 / 83 / 0 failure`**。⇒ **三本棘轮门与 Swift 单元门同时绿**，而这批没有把任何在飞文件留在树上。
  ③ **两处换名各自的原因**（写清以免下次误读）：`_cursor_row_fully_visible_p` 那行的消失＝`cursor_row_fully_visible_p` 已经作为函数落地（3.306），不再是"引用而无人定义"；
  `_Vmake_cursor_line_fully_visible` 出现＝xdisp 里那个 `V...` 载体的名字进入"待定性"行（文案与其它 `V...` 载体同一句，属已记录的偏差类）。
- **3.309 带门探针补上 X11 扩展臂 `x11_ext`：臂读数 846 → 1,128，第一个新缺口立刻出现（集成者，2026-10-05）**：
  ① **做了什么**（3.294 ④ 命名的那件事）：`spikes/gated-layout-probe.py` 的 `ARMS` 增一臂 **`x11_ext`** = `x11` 的定义 + **`HAVE_XSYNC`** + **`HAVE_X_I18N`**。
  为什么只有这两个扩展宏：本机（**实测**）`X11/Xlib.h`、`X11/Xlocale.h`、`X11/extensions/{Xext.h,sync.h}` **在**，而 `X11/extensions/{Xfixes.h,Xrandr.h,XInput2.h,Xinerama.h}` **不在** ⇒ `HAVE_XFIXES`/`HAVE_XRANDR`/`HAVE_XINPUT2`/`HAVE_XINERAMA` 在这台机器上**打不开**（这是机器的限制、不是取舍；`xterm.h` 第 23 行是无条件的 `#include <X11/Xlib.h>`，所以冻结臂连这个头都进不去——那是 P-51 的"冻结配置不编译 xterm.h"）。四个打不开的宏写在臂的注释里，换一台有头的机器（或做头镜像）时是**往元组里加宏**，不是发明第二套机制。
  ② **顺带纠正一处口径**：`--define` 造的是"**冻结配置** + 这些宏"的 `custom` 臂，**不是**"x11 + 这些宏"——所以它到不了 X11 扩展臂（本轮实测：`--define HAVE_XSYNC=1 --define HAVE_X_I18N=1` 的 `custom` 臂跟 frozen 一样死在 `'X11/Xlib.h' file not found`）。这一句写进了模块 docstring。
  ③ **读数（`spikes/mirror-struct-coverage.py`，整树）**：`282 mirrored struct(s): **1,128 arm(s) attempted**（原 **846**）, **318 unreadable**（原 241）, **5 gap(s)**（原 0）`——
  多出来的 282 次尝试＝每个镜像结构体多读一臂；**5 条缺口全部来自新臂，全部是 `struct x_display_info` 的 `HAVE_XSYNC` 成员**：`xsync_supported_p`、`xsync_major`、`xsync_minor`、`server_time_monotonic_p`、`server_time_offset`。
  实测该镜像里这 5 个名字 **0 次**（而 XIM 那 4 个 `use_xim`/`xim`/`xim_coding`/`preferred_xim_style` 都在）⇒ 新臂**当场把"未知面"变成了工作清单**（这是加臂的目的，不是回归：旧臂的 0 gap 读数一个字没变）。
  ④ **交接**：这 5 个名字归 xterm lane 的镜像 `femacs/src/include/xterm.rs`（它正在做 C 3,013 起的下一片；已把清单发过去）。**未**重生成任何已发布的 `docs/baseline/*-layout.tsv`：那五个类型（frame/window/face/terminal/ns_display_info）不依赖 XSYNC/X_I18N，加一列只会是 `x11` 的复制品（覆盖率工具是**动态读臂**，不需要基线）。
- **3.310 行数债那 280,076 行到底是什么：第一次按内容逐条判类（集成者实测，2026-10-05）**：
  ① **口径**：`tools/check-port-lines.py` / `docs/baseline/port-lines-pending.tsv` 列的是"端口行数低于参照 10%"的 98 个目标、**280,076 行参照行**。本轮把那 98 个目标**逐个读内容**判类（不看行数、看它携带的注释），判据是两类既有政策的措辞：
  **政策 A（3.31）**"the file is not compiled by the frozen configuration"（`build/emacs-ref/src/Makefile` 的 `WINDOW_SYSTEM_OBJ=`/`HAIKU_OBJ = `/`MSDOS_OBJ =` 是空，权威表 `docs/baseline/frozen-build-sources.txt`）；**政策 B（3.5）**"has no live text in the frozen configuration"／它的同族措辞"all but N lines are dead text"（整部正文在某一个未定义的 `#ifdef` 里，注里点名宏与 `docs/baseline/config/darwin-arm64.macros.tsv` 的行号）。
  ② **读数**：**98 个目标里 97 个是政策注（A 或 B），剩下 1 个是在飞的前线 `src/xterm.c`（3,516 / 33,125 = 10.6%）**；`docs/ports/` 里这 98 个目标**每一个都有台账**（0 个缺台账）。
  抽查的 5 个"看着像半成品"的（`src/comp.rs` 48、`src/treesit.rs` 48、`src/gnutls.rs` 115、`src/sqlite.rs` 76、`lib/file-has-acl.rs` 95）全部是政策 B 的"all but N lines are dead text"写法；`lib/include/c++defs.rs`（23）是"C++-only 臂全死"的写法（并附 `check-cond-eval` 的 27 条件 0 disagreement 读数）。
  ⇒ **这 280,076 行不是"没人做的活"，而是"冻结配置不构建 / 整部正文在某宏里"的记录在案延后项**；判类的意义在于把"债"和"未开工"分开（3.202 同族的"生成物 vs 工作树"双向对账）。
  ③ **按名字前缀的构成**（指示性的，不是权威分类；同一文件只入第一个匹配的桶）：X11/GTK/pgtk 面 **18 目标 / 80,628 行**、Windows（w32/w16/nt）**18 / 52,609**、窗口系统门内（`image`/`sfnt*`）**4 / 40,063**、Android **7 / 29,640**、其它（`comp`/`emacs-module`/`fontset`/`fringe`/`ftcrfont` …）**13 / 24,873**、`lib-src`+`exec`+`lwlib`+`oldXMenu` **17 / 17,326**、可选库（gnutls/lcms/dbus/treesit/sqlite/sound/zlib）**10 / 15,936**、Haiku **7 / 12,500**、msdos **2 / 5,148**、gnulib `lib/` **2 / 1,353**（合计 280,076）。
  ④ **由此命名的一条工单（给下一个空出来的 lane）**：**X11 面是本机唯一有实际牵引力的延后块**——本机有 X11 头（`spikes/gated-layout-probe.py` 的 `x11`/`x11_ext` 两条臂就是用它读的）、`src/xterm.c` 已经在这条路上（Ohm 的 lane，3,516 行），而 `src/xfns.c`（**10,662 行**）与 `src/xmenu.c`（2,932）、`src/xrdb.c`（513）、`src/xsmfns.c`（582）、`src/xgselect.c`（296）**还没有 lane**。
  建议顺序（按"本机能否读到它的门与头"排）：`src/xfns.c` → `src/xmenu.c` → `src/xselect.c` → `src/xrdb.c` → `src/xsmfns.c`（`xsmfns` 要 libSM：本机没有 ⇒ 落成政策 B 注 + 台账，或按 xterm 的载文形状落、编译留给 X11 target）。政策与 xterm 同一条：**P-51 载文 + 带门臂 + 读数来自 `x11`/`x11_ext` 探针**，不是空单元（3.299 的 P-85 对 C++ 已经这么定了；C 侧的 X11 文件同理，因为它们的头在本机可读）。
  ⑤ 未做（明确记下）：本轮**没有**改任何 `*_pending.tsv` / 台账 / 端口文件——这是一次量测与工单命名，不是移植。
- **3.311 "整个类型带门"的清单第一次扫完：13 个真候选 → 本文件落 3 个、交 frame.rs 1 个、按事实推迟 7 个、假候选 2 个（Boyle 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/include/dispextern.rs` **+64 / −7**（3,607 → **3,664 行**）、`docs/ports/include-dispextern.md` **+95**（978 → **1,073 行**，含完整候选表与 Evidence）。落地的三条全挂 `have_window_system`：
  `struct gui_box`（dispextern.h 205-212；4 个 `int` 于 0/4/8/12，**16 / align 4**）、`struct image_cache`（dispextern.h 3368-3385；6 成员于 0/8/16/24/32/40，**48 / align 8**）、`IMAGE_CACHE_BUCKETS_SIZE`（dispextern.h 3387，`= 1009`，同门）。
  ② **候选表其余各行的判定**（每个都带三臂读数，写在台账里）：**`struct MonitorInfo`（frame.h 1928-1935）** 可落地但不属本文件 ⇒ 交 `frame.rs`（读数：`geom` 0、`work` 16、`mm_width` 32、`mm_height` 36、`name` 40，48/8；消费方是 frame.c 的两个入口与 NS lane）；
  **7 个按事实推迟**（5 个 `pgtk*` + `Lisp_TS_Parser`/`Lisp_TS_Node`）：本机三臂**都读不了**（`HAVE_PGTK`/`HAVE_TREE_SITTER` 的头不在），且**没有消费者**（`src/pgtkterm.c` 不在冻结构建内、无已端口单元命名这些类型）⇒ 推迟到"该 target 冻结"时镜像/pointee/臂一起到（`include-pgtkterm.md`/`include-treesit.md` 已按 dead-by-construction 记着）；
  **2 个假候选**：`lib/dynarray.h` 的 `int_array`/`str_array` 在**一段 `/* … */` 文档注释里**，根本不是定义（探针把它读成 incomplete 就是这个原因——**扫描器不剥注释**这条限制值得记下来），`__time_t_must_be_integral` 已由 `lib/include/time.in.rs` 按标记承载（`Lisp_TS_Query` 由单元 `pdumper.rs:2527` 声明，镜像不欠）。
  ③ **入库前我自己复检**：`extract-abi --check emacs/src/dispextern.h …` → **OK**；`xtask layout-check` → **`layout-check passed`**；`build-tu src/xfaces.c` → **built**；
  `spikes/mirror-struct-coverage.py` → **`284 mirrored struct(s): 1,136 arm(s) attempted, 320 unreadable …, 0 gap(s)`**（结构体 282 → 284＝两条新记录进了被检查行）。
  **这行 0 要读准**：它是**工作树**读数——3.309 新臂读出的那 5 条 `x_display_info` XSYNC 缺口已由 **xterm lane 在飞的 `include/xterm.rs`** 补上（5 个名字各命中 1 次，实测）；在 HEAD 上仍是 5 条，等那一片入库。
  ④ **集成者动作（共享表，已做）**：`docs/baseline/citations-allowlist.tsv` 第 37 行 `femacs/src/include/dispextern.rs:544` → **`:567`**（`gui_box` 的插入把该引用行下移 23 行）。
  改前 `check-citations` 报同一条 finding 两次（`drift …:567` + `stale …:544`），改后**两条同时消失**：`662 file(s) scanned, 32 drift, 1 range, 306 external, 38 allowed, **0 stale**`。
  lane 另报的三条既有漂移（`dispextern.rs:1970`、`abi_shim.rs:1051/1061/1100`、两个 Swift 文件各一条）**先不动**（都在本批之前就存在，属"归谁"待定）。
- **3.312 `struct MonitorInfo` 按门落地；**臂之间不一致**这件事第一次被这条记录量出来（集成者，2026-10-05）**：
  ① **落地**（Boyle 的候选表交给 `frame.rs` 的那一条）：`femacs/src/include/frame.rs` **1,195 → 1,231 行（+36）**、台账新增一节。内容：
  `#[cfg (feature = "have_window_system")] #[repr(C)] pub struct MonitorInfo { geom, work: Emacs_Rectangle; mm_width, mm_height: c_int; name: *mut c_char; #[cfg (feature = "have_pgtk")] scale_factor: c_double }` + 两个入口的 `unsafe extern "C"` 声明（`free_monitors`、`make_monitor_attribute_list`，frame.h 1937-1943 声明、frame.c 定义）。
  ② **这条记录量出的关键事实：带门臂之间不一致，而且分歧在 typedef 不在记录**（`spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h`，我本轮自己跑的原文）：

  | 臂 | `geom`/`work` | 偏移 | sizeof / align |
  |---|---|---:|---|
  | frozen | — | — | `incomplete type`（门未开） |
  | **x11** | `XRectangle`（4 个 short，8 字节；dispextern.h **44** 的 `typedef XRectangle Emacs_Rectangle`） | `mm_width` 16、`mm_height` 20、`name` 24 | **32 / 8** |
  | **ns_cocoa** | 非 X 的 typedef（4 个 int，16 字节；dispextern.h **67-70**） | `mm_width` 32、`mm_height` 36、`name` 40 | **48 / 8** |

  ③ **因此把 Boyle 交来的读数读准**：它的"48/8、0/16/32/36/40"是 **ns_cocoa 臂**的数；**x11 臂是 32/8**。我第一版注释写成了"x11 与 ns_cocoa 一致"——**已就地改正**（镜子里的错注释比缺注释更坏），台账里两种臂的读数与逐成员偏移都记全。
  镜像承载的是**非 X 的 typedef**（冻结配置的臂，也是树内每个单元编译时用的那个）；x11 的读数**记在台账里而不复现**，因为复现它要给 `Emacs_Rectangle` 加臂分裂，而那需要 `XRectangle`——**一份 Xlib 镜像，目前没有任何单元有**。
  ④ **由此命名一条新任务（给 X11/xterm lane，与 3.270 ④ 的 `None`/`Success` 同一份 Xlib 镜像）**：`Emacs_Rectangle` 的臂分裂（`have_x_windows` → `XRectangle` = 4 shorts），受影响的不止这条记录（`androidgui.rs` 的 `NativeRectangle = Emacs_Rectangle`、以及任何 X11 侧记录）。
  ⑤ **入库前我自己复检**：`extract-abi --check emacs/src/frame.h femacs/src/include/frame.rs` → **OK**；`build-tu src/frame.c` → **built**（冻结臂不含这条记录，读数不变）；
  带门臂用剪段探针编过：记录原文 + 六个 `assert!`（sizeof 48、align 8、偏移 0/16/32/36/40；对应**非 X** 的 `Emacs_Rectangle`），`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib -C panic=abort --cfg 'feature="have_window_system"'` → **exit 0（0 error）**。
- **3.313 Kotlin 又三件（`EmacsSurfaceView`/`EmacsPreferencesActivity`/`EmacsNoninteractive`）→ 28/40；P-83 按实测补三条（Mill 交付，2026-10-05）**：
  ① **入的四件**：三个 `.kt`（参照 Java 223/182/227 行 → 端口 **192/151/202**）+ 重生成的 `docs/baseline/kotlin-order.tsv`（**40 target(s), 28 ported, 12 to go**）。
  ② **入库前我自己复检**（不引用 lane 读数）：`spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/*.kt`（**全目录 28 个一起**）→ **exit 0、0 error、13 warning**（13 条全是既有文件的，本批三件 0 条）；
  **按名字找的消费者我自己逐条查过**：`emacs/java/AndroidManifest.xml.in:294` 的 `android:name=".EmacsPreferencesActivity"`（按名字启动的入口）⇒ 端口类名与 `open class` 都对（`EmacsLauncherPreferencesActivity : EmacsPreferencesActivity()` 要能继承）；
  `emacs/src/android-emacs.c:28/82` 的 `org.gnu.emacs.EmacsNoninteractive`（`app_process` 命令行）⇒ 类名对；参照自己在 `EmacsNoninteractive.java:215` **反射查 `"main1"`** ⇒ 端口把 `main1` 保成 companion 上的 `@JvmStatic @Throws(Exception::class) fun main1 (args: Array<String>, context: Context)`（外层类上是真 static，P-83 ④+⑪）；
  `EmacsSurfaceView` 在 `emacs/src/*.c` 与 manifest 里 **0 命中** ⇒ 它**没有 JNI 名字义务**（只在 Java/Kotlin 层被 `EmacsView` 用）。
  ③ **P-83 按实测补三条**（PORTING.md 已改）：④ 增 `@Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")`（两个诊断名，只写前者仍报 4 条 warning）、Java 的续行 `=` 不是 Kotlin 语法（7 处）、局部变量重赋 ⇒ `var`；⑥ 增 **每批收口必须跑全目录探针**（per-file 探针拿参照 **Java** 当替身，看不到"端口自己的子类被 `final` 挡住"——实测 `EmacsLauncherPreferencesActivity` 那条 `this type is final`，只有全 `.kt` 一起编才暴露）。
  ④ **顺带的下一件预量（`EmacsNative`）**：`grep` 对过参照 `EmacsNative.java` 的 **69 个 `public static native`** 与 `emacs/src/` 的 **69 条 `NATIVE_NAME (...)`**，**集合完全相同**；`javap -p -s` 实测 companion 成员在外层类上生成**真 static native**（`()Ljava/lang/String;` 等）⇒ 那 69 个都能按 `@JvmStatic external fun` 落，一次编译探针即可验。
- **3.314 nsterm 第 15 轮入库：三个 fringe 定义落地，文件级对象 49/49 收口（Ramanujan 交付，2026-10-05）**：
  ① **入的两件**：`femacs/platform/macos/nsterm.swift` **+260 / −2**（**3,986 行**）、`docs/ports/nsterm.md` **+90 / −9**（未勾框 **261**）。本轮三个 C 定义（正是 manifest 里那三行）：`ns_define_fringe_bitmap`、`ns_destroy_fringe_bitmap`、`ns_draw_fringe_bitmap`，外加 `fringe_bmp` 对象框；
  台账读数：函数 **83 → 86 / 150**、**文件级对象 48 → 49 / 49（该段收口）**、宏 17/23、类型 1/1。
  ② **入库前我自己复检**：`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift` → **exit 0、0 error**；`check-swift-units` 的 **3 条 stale 正好点名本片那三个定义**（"manifest row no longer matches (it landed?)"），其余 0 stale。
  ③ **本片两条值得留档的读数**：⑴ **一个 C 拼写对应一个条目**——`WINDOW_FRAME`（宏）与第 13 片的 `window_frame`（成员）、`face_foreground`/`face_background`（bare 成员读）与 `NS_FACE_BACKGROUND`（宏）**并存不合并**，理由是 JNI/Swift 按名字导入的是**各自的拼写**；
  ⑵ **两个名字来自冻结配置不编译的 `src/fringe.c`**（`femacs/src/fringe.rs` 是 3.31 记录的空单元，其注释引用的正是构建里空的 `WINDOW_SYSTEM_OBJ=`）⇒ `gui_define_fringe_bitmap`（fringe.c 1853）与 `max_used_fringe_bitmap`（fringe.c 497）**必须带 `fringe.c` 所属配置的门导出**，是"带门欠账"而不是普通缺名。
  ④ **共享表两项交给棘轮重生成**（下一轮，见 3.315）：`swift-units.tsv` 删 3 行（本片三个定义）、`swift-import-pending.txt` 加 **18 行**（5 `reference-source`：`WINDOW_FRAME`/`face_foreground`/`face_background`/`gui_define_fringe_bitmap`/`max_used_fringe_bitmap`；13 `unclassified`：`draw_fringe_bitmap_params_*` 的成员条目）。
  ⑤ **给 shim lane 的工单**（已转给 Turing，作为批⑫的输入）：那 18 个名字，其中 `gui_define_fringe_bitmap` 要带门、`max_used_fringe_bitmap` 是无门数据符号、13 个 params 成员含两个 `bool_bf` 位域、`face_foreground`/`face_background` 是 `struct face` 的 bare 成员条目（与既有宏条目并存）。
- **3.315 落地 3.313/3.314 后重跑四棘轮：swift-import 632 → 650、swift-units −3；两门剩下的 failure 全在四条在飞 lane 上（2026-10-05）**：
  ① **跑了**：`tools/reconcile-link-ratchets.sh`（**2m41s**），`git diff --numstat`：`swift-import-pending.txt` **+18**（632 → **650**，正是 3.314 ④ 那份清单：5 `reference-source` + 13 `unclassified`）、`swift-units.tsv` **−3**（`nsterm.swift` 的 `ns_define_fringe_bitmap`/`ns_destroy_fringe_bitmap`/`ns_draw_fringe_bitmap`）；
  `link-pending.txt`/`link-exec-pending.txt` **内容零变化**（这一片没有新导出，nsterm 自己也报"本片无新导出"）。
  ② **门读数**：`check-swift-units` **`0 new, 0 stale, 7 Swift unit(s) landed`**（clean）；`check-link` **`374 undefined / 373 行 / 1 failure`**、`--executable` **`84 / 83 / 1`**——**那 1 条 failure 是在飞 lane 的，不是 HEAD 的**：`_Fcurrent_bidi_paragraph_direction (port-owed): referenced and defined by nothing`，我按"读数归谁"实测过——`nm -u build/femacs/*.o` 里它只出现在 **`build/femacs/xdisp.o`**，
  而 `femacs/src/xdisp.rs` 现在是**第 24 轮的脏文件**（lane 自己 `build-tu` 出来的对象），所以这是 lane 在飞的引用、随它那一片入库消失（3.294/3.308 的同一口径）。
  ③ `check-swift-imports` **`949 import(s) in 7 Swift file(s), 5,705 core export(s), 636 unprovided, 650 ratchet row(s), 14 failure(s)`**，并且**两次相隔几分钟的运行读数在动**（`unprovided` 638 → 636、failures 12 → 14、core exports 5,703 → 5,705）——正是四条在飞 lane（xdisp/xterm/nsterm/abi_shim）在改树上的证据；**650 行棘轮是 HEAD 的读数**（脚本在它自己的 HEAD worktree 里造），工作树侧多出来的 failure 不进棘轮（这正是"先在 HEAD 造读数再对账"那条顺序的分工）。
  ④ 处置不变：不把在飞状态写进任何基线；等四条 lane 交活、入库后重跑同一脚本，读数自然归零（3.308 已经演示过一次：三本门同时 0 failure / 0 stale）。
- **3.316 3.310 ④ 那条工单开工前的预量：X11 面第一个"可以真的开"的文件是 `src/xrdb.c`，前置条件都已满足（集成者，2026-10-05）**：
  ① **为什么不是 `xfns.c` 开头**：3.310 ④ 把 `src/xfns.c`（10,662 行）列为 X11 面的第一件，但它是一个"开着做几轮的 10k 行单元"；**要先把一个 500 行的整文件走通**，`src/xrdb.c`（**513 行**，X11 面最小的目标）才是正确的第一件——它的成功会把"3.31 空单元 → 真端口"这条路径在 X11 面上验一次。
  ② **它的清单已经量出来了**（`tools/cinventory.py` 直接读，绕开 `extract-abi.py` 对空单元形状的抑制）：**11 个函数**（`x_get_customization_string`、`magic_db`、`search_magic_path`、`get_system_app`、`get_fallback`、`get_user_app`、`get_user_db`、`get_environ_db`、`x_load_resources`、`x_get_resource`、`x_get_string_resource`）+ **1 个宏**（`XrmStringType`，`cinventory` 同时记为常量）+ **2 个文件级对象**（`x_customization_string`、`xdefaults`）＝ **15 项**（正是 `not_built_on_target: 15`）。
  ③ **"开单元"在工具里是被支持的，量到两处机制**：`tools/extract-abi.py` 1352-1366 的豁免判据是**空单元形状**（`not re.search(r"\bfn\s+\w+", rs)`）——"A unit that *did* port the text … keeps the full comparison: the exemption is for the empty shape, not for the file" ⇒ 一旦 `xrdb.rs` 里有真 `fn`，`--check` 就按 15 项**全比对**（xterm lane 的 `--check` 313 条 `- ` 就是这么来的）。**因此半开的单元会污染 `--all`**——这是"要么开、要么不开"的原因，也是本轮**没有**动手开它的理由（把半个单元留在树上比保持空单元更坏）。
  ④ **前置条件逐条核过**：⑴ `alloca` —— P-05 的机制**已经选定**（裸 `alloca` 走栈池 `sa_stack_alloc (n)`，既有写法见 status 3358-3359 的实例，`SAFE_ALLOCA` 族走 `AVAIL_ALLOCA!` + specpdl），所以 P-05 末句"选定前不得批量移植含 alloca 的文件"**不阻塞**这一个文件；⑵ X11 头 **全在**本机：`X11/{Xlib.h,Xatom.h,X.h,Xutil.h,Xresource.h}`（实测 `ls` 通过）；⑶ 名字拼法已有先例：`XrmDatabase` 在 `femacs/src/include/{xterm.rs:370,pgtkgui.rs:122}`（`pub type XrmDatabase = *mut c_void`）已落，`xfree`/`xmalloc`/`xpalloc`/`xstrdup` 在 `include/lisp.rs` 的 `extern "C"` 里有声明；⑷ 单元的臂形状照 `xterm.rs`：整身 `#[cfg (feature = "have_x_windows")]` + 内层条件按小写 feature 名。
  ⑤ **交接**：这件归下一个空出来的 lane（或集成者在管线安静的回合自己开）；开的时候按 xterm 的流程走（先 `cinventory` 出清单 → 抄文本 → `build-tu` → `--check` 的 `- ` 条数与最早 missing 逐条自证 → 台账勾选 → 我入库）。
- **3.317 abi shim 第⑫批 18 个导出入库（nsterm 第 15 轮要的那 18 个名字），台账 179/179（Turing 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/abi_shim.rs` **+537 / −0**（真条目 **161 → 179**，台账勾选框 **179 / 179、0 未勾**）、`docs/ports/abi_shim.md` **+316 / −40**。本批 18 条的分布：
  **模块 `c_abi_of_mirror_named_items` 2 条**——`WINDOW_FRAME`（名字已被镜像拼成**函数**（`include/frame.rs` 939），crate 根再定义是 `E0428` ⇒ 模块 + `super::`；与第 13 片的成员条目 `window_frame` **并存**，PORTING.md 0.2 的"逐名不合并"）、
  `max_used_fringe_bitmap`（镜像是 **`extern` 声明**（dispextern.rs 2510），本批给**定义** `pub static mut … : c_int = 25`——25 来自 `fringe.c` 460-489 的 `countof (standard_bitmaps)`，注释里写明出处）；
  **带门 15 条 `dispextern_face_records`**：13 个 `draw_fringe_bitmap_params_*` 成员（含两个 `bool_bf` 位域：`cursor_p` = 字节 52 位 0、`overlay_p` 位 1，实测）+ `face_foreground`/`face_background`（`struct face` 的 `unsigned long` bare 成员读，dispextern.h 1816/1819）；
  **带门 1 条 `have_window_system`**：`gui_define_fringe_bitmap`（`fringe.c` 1853 定义，冻结配置不编译该文件 ⇒ 带门欠账）。
  ② **入库前我自己复检**：`build-tu src/abi_shim.c` → **exit 0、0 条 `^error`**（末行 `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`）；台账 **179 勾 / 0 未勾**；
  我自己的 `nm -g build/femacs/abi_shim.o` 读数：**`T _WINDOW_FRAME`**、**`D _max_used_fringe_bitmap`**（数据符号）在冻结对象里，而 **`_face_foreground`/`_face_background` 不在**（正是"带门才对"的证据——它们只在翻牌下出现）；`check-declarations` 13/0/0、`check-carried` 11/11/0、`check-mirror-dupes` 1 dup/1 棘轮行/0。
  ③ **lane 的翻牌读数（照抄，我未重跑）**：`dispextern_face_records` 翻牌 → `xtask probe … --cfg 'feature="dispextern_face_records"'` **exit 0、15/15 条带门符号进对象**；
  `have_window_system` 翻牌 → **exit 1、48 条 error，但没有一条点名本批**（与批⑪同一次读数：那堵墙是 `frame.rs` 的带门存取器缺平台类型，不是本文件的问题）。
  ④ **下一步**：棘轮重生成应把这 18 个名字**从待办里去掉**（`swift-import-pending` 650 → 632）——见 3.318。
- **3.318 X11 两个记录的臂表入库（4 张 TSV），并更正 3.309 一处读数口径（Boyle 交付，2026-10-05）**：
  ① **入的四张**（都是新文件，生成命令写在各自表头）：`docs/baseline/x_display_info-layout.tsv` **495 行**（4 臂：`sizeof` **4776/8**（x11）与 **4856/8**（x11_ext）；frozen/ns_cocoa 各一行 `parse=no`）、
  `docs/baseline/x_output-layout.tsv` **181 行**（**448/8** vs **560/8**）、`docs/baseline/x_display_info-bits.tsv` **12 行**、`docs/baseline/x_output-bits.tsv` **23 行**。我自己的读法复核：行数、两份表头里的生成命令、`frozen`/`ns_cocoa` 的 `parse=no` 行位置（7 与 246/82）都对得上。
  ② **一条更正（我 3.309 的话说窄了）**：x11 与 x11_ext 在 `x_display_info` 上的差集是 **11 个顶层名**，不是 9——**第三道门** `defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME` 另外贡献了 `server_time_monotonic_p`/`server_time_offset`（本臂同时满足它：macros.tsv 1248 的 `HAVE_CLOCK_GETTIME 1`、3861 的 `USE_GTK undef`）。
  所以 3.309 里"5 条缺口＝XSYNC 成员"的账要这么读：那 5 条是 **XSYNC 的 3 个 + 第三道门的 2 个**（`xsync_supported_p`/`xsync_major`/`xsync_minor`/`server_time_monotonic_p`/`server_time_offset`），与 xterm lane 已经补进镜像的 5 个名字**逐个吻合**。
  ③ **一处计数口径分歧（记下来，避免下次误引）**：3.302/3.309 我引的"x11 181 / x11_ext 190"是**我自己的名字正则**在臂输出上的计数；Boyle 的表用**clang dump 的缩进规则**（深度 0 = 顶层）给出 **207 / 218 顶层 + 29 嵌套**。两者相差 26，**表是权威口径**（它可复算、且是发布物）；教训与 3.182/3.196/3.198/3.202 同族：**引数字要连口径一起引**。
  ④ **三条对镜像作者要紧的读数**（lane 写进表头/报告，我照抄）：⑴ **共享成员在带门臂里会整体平移**（`x_display_info` 207 个共享名里 113 个偏移不变、85 个 +48、4 个 +60、3 个 +64、2 个 +80；`x_output` 59 个里 43 不变、6 个 +88、4 个 +116）⇒ **手推偏移会错，必须读臂表**；
  ⑵ **位成员不跨臂移动**（`x_output` 的 6 个旧位成员两臂都是 unit 1 / shift 0-5，x11_ext 新增 5 个在 unit 19 / shift 0-4；`x_display_info` 的 3 个两臂都是 unit 1 / shift 0-2）；
  ⑶ **一个读法陷阱**：`x_display_info-bits.tsv` 的那 3 行其实是**嵌套 `Mouse_HLInfo` 的位串**（296:0-2），谁照它给 `x_display_info` 本体造位镜像都会错——**命名一条改进**（本轮不做，因为会改已发布位表的列形状）：让 `--bits` 把**嵌套深度**一起记下来。
  ⑤ **集成者的裁决（Boyle 问的那条）**：`parse=no` 的语义**不改表格式**——两张表的表头本来就写着"marks an arm this machine cannot read, with the reason in the run"，而探针把原因打在 `unavailable: N error(s)` 块里（本处是 `fatal error: 'X11/Xlib.h' file not found`，因为 `-I <X11>` 只给定义了 `HAVE_X_WINDOWS` 的臂）。
  我把这条读法写进了 `spikes/gated-layout-probe.py` 的 docstring：**`ns_cocoa` 的 `parse=no` 绝不能被读成"NS 没有 `x_display_info`"**——NS 的那个记录在 `nsterm.h`、镜像在 `femacs/src/include/nsterm.rs`，两者是**同名不同源**的两个类型。
- **3.319 落地 3.317 后重跑四棘轮（650 → 636），并修掉对账脚本一个"静默死亡"（2026-10-05）**：
  ① **先撞上一个真 bug（工具层的）**：第一次跑 `tools/reconcile-link-ratchets.sh` **1.6 秒退出、一个字都没打印**。`bash -x` 追到根因：脚本把"改动的镜像 → 包含它的单元"用 `grep -l "include/$m" femacs/src/*.rs` 找，而 xterm lane **刚落的 `src/include/xlib.rs` 还没有任何单元 include 它** ⇒ `grep -l` 退出 1，`set -e` + `set -o pipefail` 把它变成**静默死亡**（命令替换里的非零状态不打印任何东西）。
  修法两处（都写进脚本注释、留在代码里）：⑴ 那条 `grep` 之后加 `|| true`——"还没有单元包含它"是**一个读数**，不是失败；⑵ `git worktree add` 用 `if ! … ; then` 包起来并打印原因 + 处置（原来 `--quiet` 让"残留 worktree 挡路"也变成静默退出）。这个失败模式与 3.196/3.198/3.202/3.268 同族：**工具静默失败比工具报错更坏**，而它是靠"跑一次、看它什么都不说"才被抓住的。
  ② **重跑读数**（3m12s，脚本自己的 HEAD worktree）：`swift-import-pending.txt` **+23 / −37**（**650 → 636**）——**−37 行**是 3.317 那 18 个（`WINDOW_FRAME`、`max_used_fringe_bitmap`、15 个 `draw_fringe_bitmap_params_*`/`face_foreground`/`face_background`、`gui_define_fringe_bitmap`）+ 别的 lane 刚落的导入；**+23 行**是 nsterm 在飞第 16 轮等新导入（净 −14）。
  `link-pending.txt`/`link-exec-pending.txt`/`swift-units.tsv` **内容零变化**。
  ③ **门读数（工作树）**：`check-swift-units` `0 new / 3 stale / 7 landed`（3 条 stale 是在飞 `nsterm.swift` 第 16 轮的定义）；`check-link` `374 undefined / 373 行 / 1 failure`（仍是在飞 xdisp 的对象）。
- **3.320 `src/xrdb.c` 的开门前置条件量清了：它要的 19 个 X 名字里 **17 个全树都不存在**，所以 X11 面的第一件文件**必须等 Xlib 镜像**（集成者，2026-10-05）**：
  ① **我本轮本来要开 `src/xrdb.c`**（3.316 的工单），读完全文前先把它的外部依赖逐名查了一遍——结论是**不能先开**，理由是可复算的：`xrdb.c` 用到的 X 名字里，**只有 `XrmDatabase` 在树里有拼法**（`femacs/src/include/{xterm.rs:370,pgtkgui.rs:122}` → `pub type XrmDatabase = *mut c_void`），**其余 17 个在 `femacs/**`（扣掉六棵逐字树）里一次都没出现**：
  `DefaultScreenOfDisplay`、`XResourceManagerString`、`XScreenResourceString`、`XrmClass`、`XrmGetFileDatabase`、`XrmGetStringDatabase`、`XrmInitialize`、`XrmMergeDatabases`、`XrmName`、`XrmPutLineResource`、`XrmQGetResource`、`XrmRepresentation`、`XrmStringToClassList`、`XrmStringToNameList`、`XrmStringToQuark`、`XrmStringType`、`XrmValue`。
  ② **为什么不就地声明**：若在 `xrdb.rs` 里自带一套 `extern "C"`，`XrmDatabase`/`XrmValue`/`XrmStringType` 就会有**两个家**（本文件 + xterm lane 正在建的 `src/include/xlib.rs`）——那正是 PORTING.md 0.1-3"一处声明点"要防的漂移。**顺序只能是"先镜像、后单元"**。
  ③ **因此这条同时是给镜像的验收清单**（xterm lane 的 `include/xlib.rs` 已在建，见 3.312 ④ 的 `XRectangle`/`None`/`Success`）：上面 17 个名字 + `XrmDatabase` 的**唯一拼法**，是 X11 面第一个消费者的实测需求；镜像落地后，`xrdb.c` 才能按 3.316 ⑤ 的流程开（11 函数 + 1 宏 + 2 全局，`cinventory` 已量）。
  ④ **未做**：本轮没有改任何 `femacs/src/{xrdb.rs,xlib.rs}`（前者仍是空单元、后者是在飞文件），没有再动 `docs/baseline/**`；这条是"量清了顺序"，不是"推迟"——X11 面的第一件文件的开工条件第一次变成**可复算的清单**。
- **3.321 Kotlin 又两件（`EmacsNative` + `EmacsContextMenu`）→ 30/40；并修掉探针缺的第二个"生成类"（Mill 交付 + 集成者修工具，2026-10-05）**：
  ① **入的两件**：`EmacsNative.kt`（参照 Java **362 → 320**）、`EmacsContextMenu.kt`（**404 → 356**）+ 重生成的 `docs/baseline/kotlin-order.tsv`（**40 target(s), 30 ported, 10 to go**）。
  `EmacsNative` 是整批里**名字义务最重**的一个：`src/android.h:344` 的 `#define NATIVE_NAME(name) Java_org_gnu_emacs_EmacsNative_##name` 是**命名约定**而不是查表，69 条实现分布在 `android.c` 42 + `androidterm.c` 22 + `androidvfs.c` 5；
  lane 给了 69 对"Java 声明行 ↔ C 实现行"的逐行对照（如 `getFingerprint` 41↔1462、`initEmacs` 91↔2087、`takeSnapshot` 283↔6359），空安全按 C 体判（8 个有显式 `return NULL` 的写成可空返回：`getProcName`/`getSelectedText`/`getTextAfterCursor`/`getTextBeforeCursor`/`getSelection`/`getExtractedText`/`takeSnapshot`/`displayNameHash`，其余 61 个非空）。
  `EmacsContextMenu` 走的是**查表**（`androidmenu.c` 103-115）：七个成员逐条对过（`createContextMenu`/`addItem`/`addSubmenu`/`addPane`/`parent`/`display`/`dismiss`），外加**三个静态字段**（`itemAlreadySelected` `Z`、`wasSubmenuSelected` `J`、`lastMenuEventSerial` `I`）——`EmacsActivity.java` 508-529 是按**字段**读的，端口用 companion `@JvmField`（`EmacsApplication.dumpFileName` 的先例）。
  ② **入库前我自己复检（三项，都是我自己跑的）**：⑴ 探针（**全目录 30 个 `.kt`**）**exit 0 / 0 error / 13 warning**（13 条全是既有文件的）；⑵ **JNI 名字集合双向零差**：Java 的 `public static native` **69** ↔ `emacs/src/` 的 `NATIVE_NAME (...)` **69** ↔ 端口的 `external fun` **69**，`java-only` 与 `C-only` 都是**空**；
  ⑶ `spikes/kotlin-target-order.py --check` → `40 target(s), 30 ported, 10 to go`。
  ③ **一个真缺陷（工具层，我修的）**：第一次跑全目录探针 **exit 1**，唯一错误是 `EmacsNative.kt:292: unresolved reference 'EmacsConfig'`。根因不是端口写错，而是**参照树里没有这个类**——它由 Android 构建生成（`emacs/java/Makefile.in` 409-427 的 `cf-stamp-1`：`package org.gnu.emacs; public class EmacsConfig { public static final String[] EMACS_SHARED_LIBRARIES = {$(ALL_DEPENDENCIES)}; }`），所以探针的"拿参照 `.java` 当替身"机制**给不出它**。
  修法与 `R.kt` 同一条既有约定（**真名字、探针自己的值**）：`spikes/kotlin-compile-probe.sh` 现在也写一份 `$out/EmacsConfig.java`（同一个 `package`/类名/唯一成员，值是空数组，注释里点名 Makefile 的 409-427 行），并把它加进编译命令行；重跑 **exit 0**。
  **这条同时是 P-83 ⑥"每批收口跑全目录"的正面证据**：per-file 探针（用 Java 替身）看不到它，只有 30 个 `.kt` 一起编才暴露——正是 Mill 上一轮自己补进 P-83 的那条规矩。
- **3.322 nsterm 第 16 轮入库：三个"光标/边框/分隔条"定义，函数 89/150；该 lane 第一次碰到"头文件里带的门"（Ramanujan 交付，2026-10-05）**：
  ① **入的两件**：`femacs/platform/macos/nsterm.swift` **+290 / −2**（**4,274 行**）、`docs/ports/nsterm.md` **+100 / −5**（未勾框 **258**）。本轮三个定义：`ns_draw_window_cursor`、`ns_draw_vertical_window_border`、`ns_draw_window_divider`；台账读数：函数 **86 → 89 / 150**、覆盖范围 C 60-3508、宏 17/23、文件级对象 49/49、类型 1/1。
  ② **入库前我自己复检**：`swiftc -parse-as-library -c femacs/platform/macos/nsterm.swift` → **exit 0、0 error**；`check-swift-units` 的 **3 条 stale 正好点名本片那三个定义**（其余 0 stale、0 new）。
  ③ **本片三条值得留档的读数**：⑴ **该 lane 第一次需要"头文件里带的门成员"**——`struct window` 的 `HAVE_WINDOW_SYSTEM` 字段块（window.h 340-350，5 个成员：`phys_cursor_type`/`phys_cursor_on_p`/`phys_cursor_width`/`phys_cursor_hpos` 一族），按 3.262 的形状记"未验证 + 重新成立的条件＝定义 `HAVE_WINDOW_SYSTEM` 的构建（即这条 lane 一直写的 Cocoa 目标）"；
  ⑵ **三个带门/死文件函数**（`get_phys_cursor_geometry`/`draw_phys_cursor_glyph` 来自 xdisp.c 的 `#ifdef HAVE_WINDOW_SYSTEM`、`draw_fringe_bitmap` 来自**没有任何构建编译的 `fringe.c`**）与 `get_phys_cursor_glyph`（由 `window.rs` 提供）形成对照；
  ⑶ **14 条 `STALE (satisfied)`** 全是第 14/15 片的名字（`CHAR_GLYPH`/`COMPOSITE_GLYPH`/`WINDOW_FRAME`/3 个 `composition_*`/`font_driver`/`font_driver_text_extents`/3 个 `glyph_row_*`/`glyph_type`/`max_used_fringe_bitmap`/`window_pseudo_window_p`）⇒ **shim lane 的批⑪/⑫ 已经把它们补齐**，这些行由下一次棘轮重生成删掉（见 3.323）。
  ④ **共享表两项交给棘轮**：`swift-units.tsv` 删 3 行、`swift-import-pending.txt` **加 27 行**（13 `reference-source`：`WINDOW_XFRAME`/`FRAME_CURSOR_WIDTH`/`FRAME_FOREGROUND_PIXEL`/`DEFAULT_CURSOR`/`NO_CURSOR`/`FILLED_BOX_CURSOR`/`HOLLOW_BOX_CURSOR`/`BAR_CURSOR`/`HBAR_CURSOR`/`VERTICAL_BORDER_FACE_ID`/`WINDOW_DIVIDER_FACE_ID`/`WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID`/`WINDOW_DIVIDER_LAST_PIXEL_FACE_ID` + 3 个带门函数；14 `unclassified`：5 个 `window_*phys_cursor*` 成员、4 个 `glyph_row_*`、2 个 `glyph_*` 等）。
  ⑤ **给 shim lane 的批⑬**（已转给 Turing）：那 27 个名字，**带门的三类必须按同门导出**——① 两条 xdisp.c 的 `HAVE_WINDOW_SYSTEM` 函数；② `draw_fringe_bitmap`（`fringe.c` 所属的那套构建门）；③ 5 个 `struct window` 的 `HAVE_WINDOW_SYSTEM` 字段块成员；其余是无门成员/枚举器/宏条目。
- **3.323 落地 3.321/3.322 后重跑四棘轮：swift-import 636 → 663（+27），swift-units −3（2026-10-05）**：
  ① **跑了**（3m23s）：`swift-import-pending.txt` **+27 / −0**（**636 → 663**，正是 nsterm 第 16 轮那份清单）、`swift-units.tsv` **−3**（该片三个定义：`ns_draw_window_cursor`/`ns_draw_vertical_window_border`/`ns_draw_window_divider`）；`link-pending.txt`/`link-exec-pending.txt` **内容零变化**。
  （该轮 14 条 `STALE (satisfied)` 不是这次删的——3.319 那次重生成已经删掉了，所以这次的净增正好是 +27。）
  ② **门读数（工作树）**：`check-swift-units` **`0 new, 0 stale, 7 Swift unit(s) landed`**（clean）；`check-link` **`374 undefined / 373 行 / 1 failure`**、`--executable` **`84 / 83 / 1`**——那条 failure 仍是在飞 xdisp 对象里的 `_Fcurrent_bidi_paragraph_direction`（3.315 ② 的实测归属），不是 HEAD 的。
  ③ **没有混进来的东西**：工作树里 `docs/baseline/{frame,window,x_display_info,x_output}-bits.tsv` 与 `spikes/gated-layout-probe.py` 的改动是 **Boyle 的在飞工作**（`--bits` 的 `depth` 列，见 3.318 ④ 那条命名改进）——本次提交**只含两本棘轮**，等它交付时连同探针一起入库，避免把"生成物 vs 生成器"的半途状态写成基线。
- **3.324 `--bits` 的 `depth` 列落地、四张位表重生成：`x_display_info` 的 6 行现在读作 depth 1（Boyle 交付 + 我复核，2026-10-05）**：
  ① **入的四张 + 探针**：`docs/baseline/{frame,window,x_display_info,x_output}-bits.tsv` 各加一列 **`depth`**（表头原文：`DEPTH is the member's nesting depth: 0 = this struct's own member, >0 = a member of a record nested inside it and NOT this struct's bit`），
  `spikes/gated-layout-probe.py` 的 `--bits` 从 clang dump 的缩进层级取 depth。diff：frame **187/139**、window **82/63**、x_display_info **12/12**、x_output **23/23**。
  ② **入库前我自己复核（两项）**：⑴ 四张表**逐张与"用当前探针重新 `--write` 的结果"逐字节相同**（生成器↔生成物一致，不是半途状态）；⑵ 逐表点 depth：`x_display_info` 的 **6 行全部 depth 1**（正是 3.318 ④ 那条陷阱——它们是**嵌套 `Mouse_HLInfo`** 的位串，现在数据本身就说得明明白白），
  而 **`frame`（181 行）与 `window`（76 行）里 depth>0 的行是 0**——这回答了 lane 问的第 3 条：两张老位表**从来没有**带这个坑。`x_output` 的 17 行全 depth 0。
  ③ **格式变更是有意的**（3.318 ⑤ 已记口径）：这三张表**只有探针自己读**（实测），所以加列不破坏任何消费者；位表"一行一个位成员"的语义现在没有歧义。
- **3.325 xdisp 第 24 轮入库（19,771 行、缺口 107），并修掉一处"影子宏"语义偏差（Hypatia 交付 + 集成者纠错，2026-10-05）**：
  ① **入的两件**：`femacs/src/xdisp.rs` **19,197 → 19,771 行**、`docs/ports/xdisp.md`（+108/−66，未勾框 **107 → 102**）。本轮 4 个函数 + 1 个 subr（结束边界 **C 20422**）：分类 112 → **107**（`missing function` −4：`try_cursor_movement`/`set_vertical_scroll_bar`/`set_horizontal_scroll_bar`/`window_start_acceptable_p`；`missing subr` 7 → 6：`Flong_line_optimizations_p`）；台账 **308 → 313/415**（Subrs 3 → **4**/13、Functions defined 213 → **217**/302）。
  ② **入库前我自己复检**：`build-tu src/xdisp.c` → **exit 0、0 条 `^error`**；`extract-abi --check` → **107 条**（与 lane 同数）；台账 102 未勾；lane 的六项自查（括号平衡、逐 C 行标识符覆盖）我在改完后重跑过。
  ③ **我抓到并修掉的一处真偏差（"影子宏"类）**：本片在 `xdisp.rs` 里新造了两个宏
  `macro_rules! BUF_BEGV { ($buf:expr) => { (*$buf).begv }; }` / `BUF_ZV`，注释写的是"buffer.h 1399-1400: `#define BUF_BEGV(buf) ((buf)->begv)`"。**这个引用是错的**：参照里 `BUF_BEGV`/`BUF_ZV` **只有** `buffer.h` 917/951 的 **INLINE 函数**（带 `current_buffer` 快路径与 `begv_marker`/`zv_marker` 分支），没有任何 `#define`；
  而**镜像早就有它们**（`include/buffer.rs:923/957` 的 `#[inline(always)] pub fn`）。Rust 的**宏与函数是两个命名空间**，所以这个宏不报重名、却让 `BUF_ZV! (buf)`/`BUF_BEGV! (buf)` 的 4 个调用点（xdisp.c 20285-20287 的三条语句）**只读裸字段**、丢掉快路径与 marker 分支——这正是硬约束 1 禁止的"发明 C 里没有的行为"。
  处置：删掉两个宏与其注释，4 个调用点改用**镜像的 inline 函数**（同一文件里另有 7 处早就是这么写的，本身就自相矛盾）；改后 `build-tu` 仍 exit 0、`--check` 仍 107、`BUF_BEGV!`/`BUF_ZV!` 全树 **0 次**。
  ④ **因此命名一条门改进（给下一个空出来的 lane）**：现在的重名门只看**声明**、且跨不过宏/函数两个命名空间，抓不到这一类。要加的是："**一个单元声明的名字（宏**或**函数）与它所 `include!` 的镜像里已声明的同名项**"——即 0.1-3"一处声明点"的机械化版本（本轮这条它就是漏网的）。判据用现成的 `check-mirror-dupes.py` 扩一档，或新写一个（先量清两者边界再定）。
  ⑤ **lane 交接的三条**：**item 56 关闭**（它说"镜像缺 `BUF_BEGV`/`BUF_ZV`"——实测**镜像有**，所以正确做法不是"补镜像"而是"**删单元里那个影子宏**"，已按 ③ 做）；**item 55 记成镜像完备性任务**（`window.h` 的**函数** extern 一整族——`window_list` 1187、`window_scroll_margin` 1226 等——`include/window.rs` 只带文件级对象 1100-1206、不带函数，各单元按 P-72 rule 3 就地声明；搬进镜像归窗口镜像那条线）；
  **item 39 结论不变**（`include/dispextern.rs` 的 `pub struct it` 整块在 `dispextern_face_records` 臂里、对本单元不可见，单元内的实测副本继续留）。
- **3.326 重名门补上"宏命名空间"：一次扫描把 110 处"单元重声明了镜像已经有的名字"变成有序工作清单（集成者，2026-10-05）**：
  ① **动机是 3.325 ③ 那处真偏差**：`xdisp.rs` 自造的 `macro_rules! BUF_BEGV` 与镜像的 `pub fn BUF_BEGV` **跨命名空间**，所以重名门看不见——Rust 把宏与函数放在两个命名空间里，单元里的宏**不报重名**却让调用点拿到不同语义。
  ② **改法**（`tools/check-mirror-dupes.py`，+32/−8）：`scan()` 现在**也收集 `macro_rules! NAME`**（顶层与模块内，`#[cfg]` 门下的照旧不算——`gated` 那条逻辑已经在了），`visible_declarations()` 把**镜像顶层**的宏算进"包含它的单元可见的名字"（宏是**文本作用域**，`include!` 点之后就在单元里；镜像模块内的宏不算，本树没有 `#[macro_use]`），`findings()` 除了同名同类，还比**跨命名空间**的两组：单元的宏 × 镜像的项、单元的项 × 镜像的宏。
  ③ **一次扫描的量**：hit 从 **1 → 111 行**（`mirror-dupes.tsv` +110 行），按**体比较**（去注释/去空白后逐字比）分成四类：**54 条"体完全相同"**（单元那份是纯冗余，删掉即可）、**40 条"体不同"**（**这才是危险类**：Rust 的遮蔽让单元那份生效，必须逐条评审——`BUF_BEGV` 就是这一类，已修）、**16 条跨命名空间**（10 宏×项 + 5 项×宏 + 1 项×项；`euidaccess.rs` 的 `stat`/`access`/`weak_alias` 是 gnulib 的 `#define stat rpl_stat` 重定向写法，属"要写理由"而不是"要删"）。
  集中的文件：**`xdisp.rs` 44、`font.rs` 22、`xfaces.rs` 9、`euidaccess.rs` 7**、`frame.rs` 4、`window.rs` 4、`dispnew.rs` 3、`keyboard.rs` 3 …
  ④ **棘轮行带"机器量出来的理由"**：每行 `(unit, mirror, name)` 后面写的是**具体结论**——"体相同，删单元那份（P-26）"（带两边的 `file:line`）、或"**体不同**——遮蔽使单元那份生效，属 `BUF_BEGV` 类，先评审"、或"跨命名空间，一个构造两种拼法"。**唯一一条人要写理由的旧行（`insdel.rs`/`bset_redisplay`）原文保留**（它是 P-72 rule 3 的合法偏离，不是冗余）。
  ⑤ **读数**：`check-mirror-dupes` **111 dup / 111 棘轮行 / 0 failure**；`--selftest` **9 例 0 失败**（新增的宏收集没有破坏既有判据）。
  ⑥ **由此命名的工作清单（给 lane）**：先做 **54 条"体相同"**（逐单元：删单元那份 → `build-tu` → `extract-abi --check` → 重跑本门；这是机械活、可批量），再逐条评审 **40 条"体不同"**（每条的结论要么进棘轮理由、要么修单元那份）；16 条跨命名空间按"一个构造两种拼法"逐条定：能收敛的收敛，不能的写理由。
- **3.327 layout 表的 `depth` 列落地（8 张，不是 7 张）：同偏移不同记录的歧义消掉了，并且顺带补上了一个整臂（Boyle 交付 + 我复核，2026-10-05）**：
  ① **入的九件**：`docs/baseline/{frame,window,face,terminal,ns_display_info,x_display_info,x_output,image}-layout.tsv` + `spikes/gated-layout-probe.py`。列形状 **3 → 4**（`arm / member / depth / offset-or-bits`）；**`parse` 与 `sizeof`/`align` 行补 `-`** 是**有意的**（3 字段的 `parse` 行会被按位置读成 `depth="no"`——正是这轮要消掉的陷阱），表头写明 `depth` 的定义（0 = 本记录自己的成员，>0 = 内嵌记录的成员，`-` = 不是成员）；位表**一个都没动**（`git status` 可证）。
  ② **入库前我自己复核（两项）**：⑴ 8 张表**逐张 4 列**、行数与 lane 报的一致；⑵ **"旧行逐条保留"我自己独立验过**：把 HEAD 版的 `(arm, member, value)` 与新版（按 4 列映射）各取集合，**8 张表 old-rows-missing-from-new 全为 0**。
  ③ **歧义确实消掉了**（lane 给的实例我逐行核过）：`x_display_info` 的 `x11 | Visual * visual | 0 | 72`、`x11 | XVisualInfo visual_info | 0 | 80`、**`x11 | Visual * visual | 1 | 80`**（内嵌 `XVisualInfo` 的成员）——旧表里后两行**同成员名同偏移**、读者无从分辨。另两族实例：`frame`（`struct ns_output *` 内嵌 union 的成员，4 臂各 8 行）与 `window`（内嵌 `struct` 的成员，13/臂）。
  ④ **一条顺带量出来的真读数**：`frame` **455 → 615 行**、`window` 332 → 444、`face` 119 → 159、`terminal` 234 → 313、`ns_display_info` 52 → 54、`image` 81 → 122——**这些表自 3.309 加 `x11_ext` 臂之后就没重生成过**，所以**整条 `x11_ext` 臂的读数此前根本不在已发布的表里**；这一轮把它们补上了（`x_display_info`/`x_output` 两行数不变，因为它们是 3.318 生成的、本来就带那一臂）。位表那边 `frame`/`window` **depth>0 为 0**（3.324），layout 这边是 **32 / 52**——**两边的差异本身就是"为什么 layout 也必须加这一列"的证据**。
  ⑤ **两条裁决**（Boyle 问的）：⑴ **`image-layout.tsv` 保留**——它是已发布表且有同一歧义，我给的清单说"7 张"只是举例，判据是"全部已发布的 layout 表"，它多做的一张是对的；⑵ **形状变化记在这里**（见 ①）。
- **3.328 110 处重命中的性质第一次量清：40 条"体不同"里 7 条只是注释/格式、33 条真不同，其中 4 条的错在**镜像**（C 拼法留在 Rust 宏体里）——已修（集成者，2026-10-05）**：
  ① **判据与读数**（对棘轮那 111 行逐行取两边宏体、去注释空白后比）：`BODIES DIFFER` 40 条中 **7 条是一侧的行集合包含于另一侧**（注释/格式差别，删哪份都要先对语义）、**33 条结构不同**；再按"C 拼法"过滤（`->`/`NULL`/`(*p)(`）：
  **镜像侧 4 条**（全在 `femacs/src/include/font.rs:221-224`：`FONT_HEIGHT`/`FONT_BASE`/`FONT_DESCENT`/`FONT_WIDTH` 的体写成 `(($f)->height)` 这样的 C 文本），**单元侧 0 条**。
  ② **修**：那 4 条按 Rust 拼法落到镜像（`(*$f).height` 等，与 `femacs/src/font.rs:170-173` 单元那份逐字一致），注释里写明**为什么它们能一直不被发现**——`macro_rules!` 的体**只在展开时解析**，而这 4 个名字在每个用到它们的单元里都被单元自己的同名宏**遮蔽**（Rust 宏可重定义、后者生效），所以那段 C 拼法的体从未被任何构建碰到。
  读数：`build-tu src/font.c` → **built**；**镜像侧 C 拼法的宏体现在 0 条**。
  ③ **因此 3.326 ③ 的措辞要收紧一档**：那 40 条"体不同"**不能一概说成"单元那份可疑"**——错了也可能是**镜像**（`font.rs` 这 4 条就是），逐条判据必须"以参照 C 为准 + 看哪份是可展开的 Rust"。lane 的清理指令已按这条细化（先修镜像、再删单元那份）。
  ④ **棘轮现在读 67 dup / 111 行 / 44 failure**——那 44 条**不是回归**，是 **Hypatia 正在按 3.326 ⑥ 删 `xdisp.rs` 那 44 处**（它的在飞写作集），行数对得上；等它这一轮入库后我重跑 `--write-ratchet` 让棘轮与树一致（**现在不重写**：会把半成品状态写进基线）。
- **3.329 abi shim 第⑬批 27 条入库（台账 206/206），并收下它的"门对门"审计报告（Turing 交付，2026-10-05）**：
  ① **入的两件**：`femacs/src/abi_shim.rs` **+385 / −0**（真条目 **179 → 206**）、`docs/ports/abi_shim.md` **+373 / −79**（台账 **206 / 206、0 未勾**）。27 条里 **21 条无门**（8 个光标枚举器 `DEFAULT_CURSOR`/`NO_CURSOR`/`FILLED_BOX_CURSOR`/`HOLLOW_BOX_CURSOR`/`BAR_CURSOR`/`HBAR_CURSOR`… + 4 个 `*_FACE_ID` + `WINDOW_XFRAME` + `FRAME_CURSOR_WIDTH`/`FRAME_FOREGROUND_PIXEL` + 5 个 `glyph_*`/`glyph_row_*` + 2 个 `window_phys_cursor_*`），**6 条带 `have_window_system` 门**（`window_set_phys_cursor_type`/`window_phys_cursor_width`/`window_set_phys_cursor_width`/`get_phys_cursor_geometry`/`draw_phys_cursor_glyph`/`draw_fringe_bitmap`）。
  ② **入库前我自己复检（四项）**：`build-tu src/abi_shim.c` → **exit 0、0 条 `^error`**；台账 **206 勾 / 0 未勾**；我自己的 `nm -g build/femacs/abi_shim.o` 抽验——`BAR_CURSOR`/`WINDOW_XFRAME`/`glyph_row_used`/`window_phys_cursor_hpos` **四个都在**，而 `get_phys_cursor_geometry`/`draw_fringe_bitmap` **两个都不在**（正是"该带门"的证据）；
  `check-declarations` 13/0/0、`check-carried` 11/11/0、`check-globals` 284 units / 0、`check-target-names` 628 .rs / 591 targets / 0。
  ③ **审计（同一份报告里的另一件，已收）**：它的脚本把每个带门条目的门与**它所引用标识符在镜像里的最近 `#[cfg]`** 逐个比（同一家族多臂时判据是"条目的门蕴含至少一份"），产出逐条表；结论是**本批 6 条带门与 73 条既有的门没有一处与镜像冲突**（脚本用法与核心逻辑写在报告里，**没进 `tools/`**，符合写作集）。
  ④ **两处必须记住的限度**（lane 在三处点名，我照抄）：⑴ **`draw_fringe_bitmap` 的体一条语句都没承载、`draw_phys_cursor_glyph` 只到 `draw_glyphs` 调用前**——本批交付的是**符号 + 台账行**，不是行为等价；补全要等 `fringe.c`/`xdisp.c` 对应单元（`STANDARD_BITMAP_HEIGHT`/`get_logical_cursor_bitmap`/`draw_fringe_bitmap_1`/`draw_glyphs`）。
  ⑵ `get_phys_cursor_geometry` 的体完整，但 `x_stretch_cursor_p` 的**存储**今天没有任何单元定义（镜像只有 `extern` 声明）⇒ 冻结构建不受影响（`nm -u` 不变），而 `have_window_system` 构建会多一条未定义引用；要可链接需 xdisp 单元落地那个 `DEFVAR_BOOL`。
  ⑤ **共享表两项交给棘轮**（下一轮）：`swift-import-pending.txt` **删 21 行**（本批无门的那 21 个名字，工具已读成 `STALE (satisfied)`）、**6 行的类别改为 `rust-side-gated`**（工具自己会改，理由写成"核心确实导出了，但只在 `have_window_system` 之后；本仓任何配置都不编那条臂 ⇒ 活已做完，这行是记录"）。
- **3.330 落地 3.329 后重跑四棘轮：swift-import 663 → 642（−21），link-pending +1 行且两本链接门回到 0 failure（2026-10-05）**：
  ① **跑了**（3m20s）：`swift-import-pending.txt` **+3 / −24**（663 → **642**，正是 Turing 报的"删 21 行 + 改类 6 行"）：删掉的 21 行是批⑬那批**无门**名字（`BAR_CURSOR`/`WINDOW_XFRAME`/`glyph_row_used`… 工具读成 `STALE (satisfied)`），新增 3 行是 `window_phys_cursor_width`/`window_set_phys_cursor_type`/`window_set_phys_cursor_width` **改类为 `rust-side-gated`**，理由原文："the core does export it, but only behind `feature = \"have_window_system\"`: no configuration this repository builds"；
  **它要求"不删"的那 3 个带门名字我都核过仍在表里**（`get_phys_cursor_geometry`/`draw_phys_cursor_glyph`/`draw_fringe_bitmap` 各 1 行、类别仍是 `reference-source`——那是分类器的判断，记录没丢：它们"带门导出"的事实写在 `docs/ports/abi_shim.md` 的 206 行台账里）。
  ② `link-pending.txt` **+1 / −0**（373 → **374 行**）：新行是 **`_Fcurrent_bidi_paragraph_direction`**，理由原文 "the reference defines it and no unit of this port does - the definition is owed"——即 xdisp 那条战线上"参照有定义、端口还没落"的名字**第一次进了棘轮**（此前它只作为工作树的 failure 出现在读数里）。
  ③ **门读数**：`check-link` **`374 undefined / 374 行 / 0 failure`**（**回到全绿**）；`check-swift-units` `0 new / **7 stale** / 7 Swift unit(s) landed`——7 条 stale 是 Ramanujan 在飞的第 17 轮的定义（预期）；`check-link --executable` 84/84 行。
- **3.331 重名清理（一）：`xdisp.rs` 的 44 处全删，符号表逐字节不变（Hypatia 交付 + 我复核，2026-10-05）**：
  ① **读数**：`femacs/src/xdisp.rs` **19,771 → 19,518 行**（44 个 `macro_rules!` 项 = 243 行，另把三条因此变孤儿的注释块合并成一条 8 行说明）；`docs/ports/xdisp.md` 同步。
  ② **入库前我自己复检**：`build-tu src/xdisp.c` → **built**；`extract-abi --check emacs/src/xdisp.c femacs/src/xdisp.rs` → **107 条**（与删之前**完全一致**）。
  ③ **最强的一条证据是 lane 自己跑的**：删除前后各构一次 `nm -g build/femacs/xdisp.o | sort`，**474 行、`diff` 为空** ⇒ 44 个宏的删除对导出符号**零影响**（宏不产出符号，但这条读数把"确实只是删冗余"钉死了）。
  ④ **逐条判据（两条都过：参照 C 怎么写 / 哪一份是可展开的 Rust）**：44 条里 **体相同** 的 36 条直接删；**体不同** 的 7 条逐条评审后**镜像胜**——3 条只差宏臂末尾一个 `;`（去空白去尾 `;` 后逐 token 比，差异归零；C 依据 dispextern.h 1520/1529/1538）、3 条镜像写的是 **C 的字面拼法**（`XFRAME ((*($w)).frame)`，C 依据 1575/1616/1650）、1 条 `WINDOW_BOTTOM_DIVIDER_WIDTH` 镜像保留了 C 的 token（`WINDOW_PSEUDO_P` 与单元的 `window_bits! (pseudo_window_p)` 是**同一位**：`WINDOW_PSEUDO_WINDOW_P_SHIFT`=11）。**待集成者修镜像的行：0 条**。
  ⑤ **保留不动的**：`window_bits`/`window_bits_shift`（镜像只有 `window_bit` 函数与位移常量，没有这两个宏名 ⇒ 不是重名）、`FRAME_OUTPUT_DATA`，以及 use-site 那一块**函数**声明（镜像确实没有那些原型——属 3.325 ⑤ 的 item 55 类）。
- **3.332 重名清理（二）：非 `xdisp` 的 33 处删掉，但**量出 16 处"错在镜像"**（Boyle 交付 + 我复核，2026-10-05）**：
  ① **删掉的 33 处**（8 个单元，`build-tu` 8/8 built、`--check` 8/8 **OK**，我逐个复跑过）：`dispnew.rs` 1、`font.rs` 15、`frame.rs` 2、`keyboard.rs` 3、`lread.rs` 1、`term.rs` 1、`window.rs` 3、`xfaces.rs` 7。
  ② **16 行判为"单元那份是对的、镜像要改"——而且 lane 用编译器把理由量出来了**（每一行都附**真实的编译错误**与 C 依据）：`SET_FRAME_GARBAGED`（`include/frame.rs:947`，5 个单元命中；`E0308: fset_redisplay (f)` 期望 `*mut c_void` 得到 `*mut frame`；C frame.h 1173-1174）、`SET_FRAME_VISIBLE`（1020；`E0425: redisplay_other_windows` 无此名 + `do_switch_frame (…, Qnil)` 处要 `Qnil ()`；C 1536-1552）、`AUTO_FRAME_ARG`（1009；**"no rules expected 'expr' metavariable"**——镜像绑 `$name:expr` 而 `AUTO_LIST1!` 收 `ident`；C 1425-1426）、`FONT_{WEIGHT,SLANT,WIDTH}_NUMERIC`（`include/font.rs` 186/189/192；体是 **C 的三元 `? :`**，Rust 无此语法；C font.h 193-201）、`FONT_SET_STYLE`（216；`ASET` 索引要 `isize`、`FONT_*_INDEX` 是 `c_int`，单元写 `as isize`；C 236-237）、`XSETFONT`（238；体是 **C 的函数式** `XSETPSEUDOVECTOR (a, b, PVEC_FONT)`；C 552）、`FONT_ADD_LOG`/`FONT_DEFERRED_LOG`（249/254；体是 **C 的 `do { … } while (false)`**；C 1004-1013）、`CHAR_CHARSET_P`（`include/charset.rs` 589；**`expected bool, found integer`**——镜像把布尔用法的 `CHARSET_FAST_MAP_REF!` 与 `0` 比；C charset.h 519-529 是直接用 `&&`）。
  ⇒ 这 16 条与 3.328 那 4 条**同一类**（镜像宏体不可展开，靠单元同名宏遮蔽"活着"），**归我修镜像**（写作集 `femacs/src/include/{frame,font,charset}.rs`），下一轮做。
  ③ **18 行判为合法保留**（不是 P-26 的重复）：16 条**跨命名空间**且 **C 自己那么写**——`euidaccess.rs` 7（`access`/`getuid`/… 是 euidaccess.c 57-69 的 `#ifdef _LIBC` 改名块，`weak_alias` 是 glibc 的 `#define`）、`faccessat.rs` 1（faccessat.c 46-51 的 `#undef access / #define access euidaccess`，**live**）、`ftoastr.rs` 3（数组长度处**必须**是 const，宏在 Rust 里当不了长度）、`mktime.rs` 1、`time_rz.rs` 2（生成的 `<time.h>` 的 `#define mktime rpl_mktime`）、`charset.rs` 1（`# define emacs_fclose fclose`，单元用 `#[link_name = "fclose"]`）、`sysdep.rs` 1（unistd.in.h 1830 的 `pipe2 rpl_pipe2`）；另 2 条：`floatfns.rs signbit` 是 `#ifndef signbit` 的**死臂文本**（与镜像的活臂是同一 C 条件的两个臂）、`insdel.rs bset_redisplay` 是棘轮里那条 `kept on purpose`（P-72 rule 3）。
  ④ **棘轮**：门现在读 **34 dup / 111 行 / 77 failure**（`77 = 33 + 44`，正是两家已删的行）；下一轮重写棘轮（删 77 行、给 16+2 行写机器量出来的理由）。
- **3.333 16 处"错在镜像"的宏体全部按单元那份修好，单元副本一并删除；重名门从 111 行收到 18 行（集成者，2026-10-05）**：
  ① **修的 11 个镜像宏体**（`include/frame.rs`：`SET_FRAME_GARBAGED`/`SET_FRAME_VISIBLE`/`AUTO_FRAME_ARG`；`include/font.rs`：`FONT_{WEIGHT,SLANT,WIDTH}_NUMERIC`、`FONT_SET_STYLE`、`XSETFONT`、`FONT_ADD_LOG`、`FONT_DEFERRED_LOG`；`include/charset.rs`：`CHAR_CHARSET_P`）——**体取单元那份**（Boyle 用编译器量过哪份对，见 3.332 ②），并在同一轮**删掉所有单元副本**（这样镜像的新体第一次被真正展开，`build-tu` 就是它的验收）。
  ② **两处"体取哪一份"的裁决值得记**：⑴ `SET_FRAME_GARBAGED` 有**两种**单元写法（4 个单元写 `(*$f).bits1 = (*$f).bits1 | (1u32 << FRAME_GARBAGED_SHIFT)`，`term.rs` 写 `frame_bits_set! (garbaged, $f, true)`）；我先取 `term.rs` 那份，**编译立刻失败**：`cannot find macro frame_bits_set`——因为那是 **`term.rs` 私有的辅助宏**（它内部才用 `FRAME_GARBAGED_SHIFT`），镜像的体**只能用镜像自己提供的名字**。改用 `bits1 + 位移常量` 那版后 5 个单元全绿。**这条是"镜像体的名字必须自足"的实测判据**。
  ⑵ `AUTO_FRAME_ARG` 两份只差形参名与一个 `;`，取 `xfaces.rs` 那份（形参名 `$parameter` 与 C frame.h 1425-1426 的 `parameter` 一致）。
  ③ **删掉的单元副本**：本轮的脚本删 **96 行**（11 个名字在 7 个单元里的副本），另加 `font.rs` 遗留的那份 `XSETFONT`（3 行）与 `coding.rs` 里两个**函数内嵌套**的 `CHAR_CHARSET_P` 变通（46 行）。
  **两处"镜子坏了"的旧注释同时改正**（`font.rs` 的"reported to the mirror's owner, not changed there"、`coding.rs` 的"the mirror is not edited"）——镜像已经修好，留着那两句就是错的记录。
  ④ **我自己复核**：受影响的 7 个单元 `build-tu` **7/7 built**、`extract-abi --check` **7/7 OK**；`check-declarations` 13/0/0、`check-carried` 11/11/0。
  ⑤ **棘轮收口**：`docs/baseline/mirror-dupes.tsv` **111 → 18 行**，18 行**每条都带 C 依据**、全是**合法的两种拼法**——gnulib/glibc 的 `#define access euidaccess`（`euidaccess.rs` 7 条）、`faccessat.c` 的 live `#undef access`、`ftoastr.h` 的数组长度常量（Rust 宏当不了长度，`ftoastr.rs` 3 条）、`mktime.c` 的 `tzset`、生成的 `<time.h>` 的 `mktime`/`timegm`、`lisp.h` 的 `emacs_fclose fclose`、`unistd.in.h` 的 `pipe2 rpl_pipe2`、`floatfns.rs signbit`（`#ifndef signbit` 的死臂，P-51）、`insdel.rs bset_redisplay`（原有那条 `kept on purpose`）。
  门读数：**18 dup / 18 棘轮行 / 0 failure**，`--selftest` 9 例 0 失败。⇒ 从 3.326 那次发现（110 处、其中 40 处体不同）到现在，**93 条已消、剩 18 条记录在案**。
- **3.334 重名清理之后的引文对账：5 条过期允许行删掉、1 条行号搬移；并量清 `check-citations` 现在的红在哪（集成者，2026-10-05）**：
  ① **删掉的 5 条**（`docs/baseline/citations-allowlist.tsv`）：`dispnew.rs:3644`、`window.rs:152`、`window.rs:5452`、`frame.rs:124`、`keyboard.rs:14049`——3.332/3.333 删掉的重复宏**连带删掉了它们上面那些 `/// C:` 引文注释**，所以这 5 条允许行的 finding **已经不存在**（gate 报 `stale … the row no longer matches a finding`）。删后 **stale 归 0**。
  ② **搬移的 1 条**：`include/dispextern.rs:567 → :587`（Ohm 的 20 行"`Emacs_Rectangle` 两臂"插入把它解释的那条 finding 下移；它报的 `drift …:587` + `stale …:567` 一对都消掉）。
  ③ **剩下 44 条 drift 的分布**（我在**工作树**与**HEAD worktree** 各跑一次：工作树 44、**HEAD 43**——即这条门在 HEAD 上本来就已经红，不是本轮新引入的唯一问题）：`abi_shim.rs` **20**（Turing 在飞文件自己的引文行号）、Swift 三件 17（`nsterm.swift` 8 / `nsfont.swift` 5 / `macfont.swift` 4，属更早的 Swift 轮次）、我们刚动过的 6 个文件各 1（`window` 2 条）。
  ⇒ **命名一条工单**：这 44 条要逐条要么**改准引文行号**、要么**进允许表并写理由**（`check-citations` 的语义是：drift = 引文引的参照行号与实际不符）。分派：`abi_shim.rs` 那 20 条归 Turing（它下一批就在这个文件里）、Swift 那 17 条归 Ramanujan/nsterm 线，其余零头归集成者。
- **3.335 Kotlin 又三件（`EmacsDialog`/`EmacsInputConnection`/`EmacsSafThread`）→ 33/40，并修掉 `EmacsNative` 里 3 个已入库的真 bug（Mill 交付 + 我复核，2026-10-05）**：
  ① **入的四件**：三个 `.kt`（参照 Java **419 → 364**、**713 → 586**、**1708 → 1537**）+ 重生成的 `docs/baseline/kotlin-order.tsv`（**40 target(s), 33 ported, 7 to go**：余 `EmacsActivity`/`EmacsDesktopNotification`/`EmacsOpenActivity`/`EmacsService`/`EmacsView`/`EmacsWindow`/`EmacsWindowManager`）。
  ② **同一个提交里修掉 3 个已入库的 `EmacsNative` 缺陷**（`3757f58` 那批留下的，lane 对本轮文件时用编译器发现）：⑴ `getSelection` 原写 `String?`，而参照是 `public static native int[] getSelection (long)`、C 侧 `androidterm.c:5651` 的 `NewIntArray (env, 2)` + 5685 `SetIntArrayRegion`，消费点 `EmacsInputConnection.java:197/458` 直接索引 `selection[0]/[1]` ⇒ **语义错**，改为 `IntArray?`；
  ⑵ `getSurroundingText` 参照可返 NULL（`androidterm.c:6376/6392 return NULL`，Java 侧 590 显式判空）⇒ 改为 `SurroundingText?`；⑶ `replaceText` 的 `attributes` 平台 stub 就是可空（编译器原文："'replaceText' overrides nothing. Potential signatures … TextAttribute?"）⇒ 改为 `TextAttribute?`。
  ③ **我自己复核**：三处 Kotlin 声明与参照 Java 的返回/参数类型逐条对上（`getSelection (window: Long): IntArray?`、`getSurroundingText (…): SurroundingText?`、`replaceText (…, attributes: TextAttribute?)`）；**全目录 33 个 `.kt` 探针 exit 0 / 0 error / 13 warning**（13 条与开工前逐条同形，新三件零诊断）。
  ④ **名字义务**：`EmacsDialog` 有 JNI 义务（`androidmenu.c` 618-619 `FindClass "org/gnu/emacs/EmacsDialog"`、644-647 的 `createDialog`/`addButton`/`display`），`javap -p -s` 逐条对过（`@JvmStatic` 使静态成员落在外层类上）；`EmacsInputConnection` 与 `EmacsSafThread` **没有 JNI 名字义务**（前者只被 Java 侧 `EmacsView.java:906` 实例化，后者经门面 `EmacsService`——但门面方法名/描述符 `android.c` 1791-1807 全部一字不改地保留、`javap` 复核过）。
- **3.336 `include/xlib.rs` 落地（249 行）：X11 面第一个真正的"系统头镜像"，`xrdb.c` 的 17 个外部依赖全在其中；连带确认 `MonitorInfo` 的 x11 臂回到 32/8（Ohm 交付 + 集成者裁决，2026-10-05）**：
  ① **入的两件**：`femacs/src/include/xlib.rs`（新文件，**249 行**，首行是 `check-target-names.py` 认的 DECLARED 形式）+ `docs/ports/xterm.md`。内容是 `<X11/Xlib.h>`/`<X11/Xresource.h>` 里"端口真正要用的那一层"：协议常量（21 个）、以及**两个按值携带的记录** `XRectangle`（`[sizeof=8, align=2]`，`const` 断言在声明处）与 `XrmValue`（`16/8`，同样断言），其余按 P-72 rule 3 退化并留 `/// C:` 原文。
   **实测意义**：`include/dispextern.rs` 的 `Emacs_Rectangle` 现在按臂分裂（`have_x_windows` → `XRectangle`），于是 3.312 那条"臂之间不一致"的**x11 读数从"记录"变成"复现"**——`spikes/gated-layout-probe.py 'struct MonitorInfo' frame.h` 的 x11 臂这次读出 **`[sizeof=32, align=8]`**（4 个 `short` 的 `geom`/`work`）✓。
  ② **我做的两件事**（裁决与对账）：⑴ **`XrmDatabase` 的家判给 `xlib.rs`**（P-72 rule ①：声明它的头是 `<X11/Xresource.h>`，就是本文件那一节）；`include/pgtkgui.rs:122` 那份**全树没有任何 `include!` 能到达**（实测 `rg -n 'include!\("pgtkgui.rs"\)' femacs | wc -l` = 0），所以它是"不可达的第二份"，归 PGTK 那条线以后搬，不在本轮造第三份；⑵ 引文允许表的 `include/dispextern.rs:567 → :587` 搬移（见 3.334）。
  ③ **lane 的读数（照抄）**：`extract-abi --check src/xterm.c` → `300` 条 `- `（"镜像是镜像，不是单元"）；`build-tu src/xterm.c` → built；`layout-check passed`；**`have_x_windows` 探针 432 → 411 条 error**（`xlib.rs` 贡献 **0** 条；余 400 条 E0425 是 47 个名字的既有缺口：`Atom` 161 / `Window` 64 / `Cursor` 19 / `XEvent` 10 / 本 lane 后续片的 `x_*` helper 36）；覆盖率 **284 结构体 / 0 gap**；`check-target-names` 628 .rs / 0 finding、`check-declarations` 13/0/0。
  ④ **一条它顺手修的（我认可）**：`xlib.rs` 的嵌套 `mod` 看不见包含单元的 import ⇒ 该模块顶部补了一行 `use core::ffi::{c_int, c_long, c_ulong};`（21 个常量否则全是 `E0425`，把 X11 臂真正的缺口埋在噪声里）。PORTING.md 0.1-9 禁的是 `use crate::…`，`core::ffi` 是绝对路径，不在此列。
  ⑤ **`src/xrdb.c` 的开工条件现在全齐**（3.316/3.320 的清单逐条对上：17 个外部名字 + `XrmDatabase` 的唯一拼法 + `XrmStringType` 的形状**按 `&CStr` 落**——lane 在仓库外做过小样实测：`extract-abi` **按字节比常量**，而 C 的字面量带结尾 NUL 且 `src/xrdb.c:386` 把它交给 C 函数，所以取 `src/emacs.rs` 的"config.h 字符串交给 C"形状，不是只在 Rust 侧用的 `&str`）。
- **3.337 把 3.251 那条"真链接"配方变成一道门：`tools/link-probe.py` + 74 行棘轮——L0 的距离第一次可复算（集成者，2026-10-05）**：
  ① **为什么要有它**：其他每道门都停在链接之前——`build-tu` 一个对象、`check-undef` 每单元未定义表、`check-link` 是**按对象**数未定义符号；只有**链接器**知道 libc/`-lncurses`/Rust 运行时到底解决了哪些。3.251 那次真链接正是这样抓到**四类"被发明出来的符号"**（把 libc 头的宏写成 `extern` 声明之类）。
  ② **工具**（`tools/link-probe.py`，与 `check-link.py` 共用对象集合——**直接 import 它的 `program_objects()`/`lib_objects()`/`LIBGNU`**，两者不会对"程序的对象"有分歧）：按 C 自己的配方 `cc -o <out> <ALLOBS> ../lib/libgnu.a -lncurses` + pinned nightly（channel 从 `femacs/rust-toolchain.toml` 读）的三支运行时 rlib；解析链接器 stderr 的 `"name", referenced from:` 块；棘轮 `docs/baseline/link-undefined.txt`（`<symbol>\t<reason>`）——**新名字 = 失败**（劣化）、**棘轮里有而链接器已解决 = stale**（进步，正是写下来的目的）。
  ③ **读数**：**74 undefined / 74 棘轮行 / 0 failure**，`--selftest` 75 条 finding（74 新名 + 1 stale）= 预期 ✓。构成：**25 个 `_V…` 载体**（`V` + C 变量名：`display_line_numbers`/`hscroll_step`/`inhibit_redisplay`/`glyphless_char_display`… ⇒ 那些单元的 `DEFVAR` 还没落）+ **1 个 `_F…` 载体**（`Fcurrent_bidi_paragraph_direction`）+ **48 个"其它"**（`_calc_pixel_width_or_height`/`_count_lines`/`_current_buffer`/`_cursor_row_p`/`_automatic_hscrolling`…＝**尚未移植的单元/尚未落地的片**的直接定义名；另有 1 个 Rust 运行时垫片 `__RNvCs…rust_begin_unwind`，属这条临时链接线**少一支库**、不是端口欠账，理由写进棘轮行）。
  ④ **与 3.251 的对比**：那次（对象集合 163、无 Rust 运行时）读 **84**；现在对象多了很多、读 **74** ⇒ 距离"第一次真链接"**缩短了 10 个符号**，而且这 74 个**第一次写成了带理由的清单**（此前只有一段叙述）。**注意口径**：这 74 与 `check-link` 的 374 **不是同一件事**（后者按对象累计未定义，前者是链接器解析后的余量），两行数不许互引。
  ⑤ **下一步（这条门自己指出来的）**：26 个载体随各单元剩余片的 `DEFVAR_*`/`DEFUN` 落地自然消失；48 个直接名随对应单元落地消失；等这两块都空，`cc` 这一步就该 **exit 0**——那就是 L0 的"第一次真链接"里程碑，届时把它写进 `docs/port-plan.md` §7.1 的阶梯记录里。
- **3.338 引文门第一次全绿：17+6 条漂移逐条改准，`check-citations` `0 drift / 0 stale`（Boyle 交付 + 我复核，2026-10-05）**：
  ① **读数**：`python3 tools/check-citations.py` → **`669 file(s) scanned, 0 drift, 0 range, 344 external, 33 allowed, 0 stale`、exit 0**——本会话里这条门第一次全绿（3.334 量过它在 **HEAD 上是 43 drift / 5 stale**）。
  ② **逐条处置的方向值得记**：这 17 条漂移**不是行号位移**，而是"**引文引的参照行号与该行后面第一个像代码的名字不符**"——所以修法是**改准参照行号**或**把真正落在该范围内的名字提到前面**，**一条允许行都没加**。举例：`frame.rs` 的 `frame.h 1790-1823` → **`1790-1800`**（两个转换宏在那儿，`adjust_frame_size` 在 333）；`include/frame.rs` 的 `735-743` → **`735-747`**（`z_order` 在 747）；`window.rs` 的 `frame.h 1163-1226` → **`331-441`**（`bool_bf` 标志串的位置）；`macfont.swift` 的 `src/macfont.m 4248-4260` → **`4248-4256`**（该文件只有 4257 行，原 range 越界，是唯一一条 `range` 类）；`nsfont.swift` 的 `src/lisp.h 532` → **`527-532`**（`Lisp_Fwd_Obj` 在 531）。
  ③ **顺手一条事实更正**：`window.rs` 的注释里引过一个**参照里根本不存在**的标识符 `SELECT_MINI_WINDOW_FLAG`（`rg` 全树 0 命中；成员名是 `select_mini_window_flag`，frame.h 536），已从注释里删掉。
  ④ **入库前我自己复核**：`check-citations` exit 0 / 0 drift / 0 stale（见 ①）；`build-tu src/{dispnew,frame,keyboard,window}.c` **4/4 built**；`swiftc -parse-as-library -c` 对 `nsfont.swift`/`macfont.swift` **0 error**。
  ⑤ **一条提交时的取舍**：`femacs/src/include/dispextern.rs` 的工作树里**混着 xterm lane 在飞的 +20 行**（`Emacs_Rectangle` 两臂拆分 + `xlib.rs` 引入），所以这个提交**只挑走 Boyle 那一处引文改动**（`git apply --cached` 单 hunk，2 行），Ohm 的在飞行**原样留在工作树**——这是"只提交本次明确处理的文件"在**同一文件内**的落法。
- **3.339 引文门的两个口径分开记：**工作树** 0 drift / 0 stale，**HEAD** 28 drift / 1 stale——差额全部是**未入库的在飞改动**（2026-10-05）**：
  ① **HEAD worktree 读数**（同 3.334 的方法）：`668 file(s) scanned, **28 drift**, 0 range, 402 external, 32 allowed, **1 stale**`。差额逐条对得上：**`abi_shim.rs` 的 20 条**（Turing 的在飞文件，它已修好但未入库）+ **`nsterm.swift` 的 8 条**（Ramanujan 的在飞文件）= 28；那 **1 条 stale** 是本轮搬到 `include/dispextern.rs:587` 的允许行——**HEAD 上那个 587 还不存在**（Ohm 的 +20 行在飞），所以它在 HEAD 上匹配不到 finding。
  ② **结论**：3.338 的"全绿"是**工作树**读数；要 HEAD 也全绿，需要**Turing 的 `abi_shim` 批与 Ohm 的 `Emacs_Rectangle` 两臂**入库（这两件入库后这条门应读 0/0/0，届时我复跑并把 HEAD 读数写进那一条的 status）。
  ③ 这条与 3.315②/3.323② 是同一类口径（工作树 vs HEAD）：**在飞 lane 造成的红不算 HEAD 的红，但也绝不能把工作树的绿当成 HEAD 的绿**——两个数都要写下来。

- **3.340 版本声明这条长期指令的第三半：本仓库自己的散文与清单进扫描面，7 处自述句改成中性表述（2026-10-05）**：
  ① **指令的读法不变**（3.90/3.134/3.226/3.250/3.269/3.318）：`版本声明不要再说是gnu emacs了` 与 38 秒之后的
     `把之前的代码中的这种声明也去掉` 是同一条指令的前后两半。前两半在**被移植的树**里早已落地（231 个版本括注
     压到修订号、268 个复制的声明块不抄、`recital` 类清掉 11 处），本轮复算读数仍是 **13 declaration sites
     (0 forbidden, 13 allowed literals), 0 failures**——13 条允许项全是 C 自己的字符串（`src/emacs.rs` 的
     `PACKAGE_NAME`/`RCS_Id`/无担保横幅、`src/callproc.rs` 三条 docstring、`lib-src/etags.rs` 与
     `lib-src/ebrowse.rs` 的 banner），改它们会让端口的 `--version` 与参照分叉（L0 判据），所以留着并逐条记理由。
  ② **本轮把门每次打印的"走过的空洞"收成 0**：`read 677 files (624 .rs 35 .kt 7 .swift 6 .s 5 .cc)` 之外，
     门现在读 **43 own prose/manifest file(s)**（38 `.md` + 3 `.toml` + `LICENSE` + `Cargo.lock`）——
     根 `README.md`、`PORTING.md`、`LICENSE`、`docs/*.md` 与 `docs/i18n/**`、`femacs/README.md`、
     三个 `Cargo.toml`、`Cargo.lock` 此前不在任何门的视野里；这一面只关 `version`（`@ <rev> (<产品名> x.y.z)`
     括注）与 `title`（产品名）两类：**我们自己许可证的名称不是 finding**，不许做的是"文档声明本仓库就是那个产品"。
     覆盖行因此第一次读 `walked past 0 file(s) of an unscanned kind: none (1 of them read above)`。
  ③ **7 处自述句逐处改成中性表述**（每处 `git diff` 一行）：`README.md` 与 `docs/i18n/zh-CN/README.md` 的许可句
     （"与本产品同一许可证"→"采用 **GNU General Public License 第 3 版或更新版本** / GPL-3.0-or-later"）、
     `docs/plan.md` 与 `docs/i18n/zh-CN/plan.md` 的首段（"某产品的一比一复刻"→"参照树的一比一复刻：`emacs/` 里的
     每一个 C 源文件"）、`docs/goal.md` 的标题（保留"参照树的 C 源一比一复刻为 Rust"）、`docs/port-master-plan.md`
     第 3 行、`docs/port-plan.md` D12 的联调判据（"某产品产出的 `.elc`/`.pdmp`"→"参照构建产出的 `.elc`/`.pdmp`"）。
  ④ **四个被豁免的落点带规模打印**（豁免是读数而不是沉默）：`docs/status.md` 1 file（运行日志，逐字引用用户指令
     与参照的输出）、`docs/acceptance.md` 1 file（记录**参照程序自己**的 `--version` 输出，是证据）、
     `docs/baseline/` 0 file（实测值，本面无 `.md`/`.toml`/`.lock`）、`docs/ports/` **602 file**（各单元台账，
     本来就是 `DOC_DIRS` 的扫描面）。这一行与 3.250/3.295 的两次"门声称读了、其实没读"是同一类防护。
  ⑤ **自证与负对照**：`--selftest` **41 例 / 0 失败**（新增 4 例：散文里的产品名、散文里的版本括注、我们自己的
     许可证句不算 finding、只写修订号的标题不算 finding）；**负对照**：`python3 tools/check-declarations.py
     --root /tmp/prose-neg` 在一棵只有两行违规的假树上当场报 `FAIL README.md: title x1` + `FAIL docs/plan.md:
     version x1`、**exit 1**；本树读数 `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`、
     `read 43 own prose/manifest file(s)`、`walked past 0 ... none (1 of them read above)`、**exit 0**。
  ⑥ **未声称**：C 自己的 13 条字面量仍在（同①，那是参照语义、不是我们的声明）；`docs/status.md` 与
     `docs/acceptance.md` 逐字记录参照的输出，故仍含产品名——两者是**测量/日志**；六棵逐字节树按用户指令原样
     （`check-verbatim` 钉着）；裸 `Emacs` 在注释散文里的 ~54 行"标题式"用词按 3.269③ 不动（那是 C 的解释性
     散文、不主张身份）；本轮改的都是本仓库自己的句子，未触碰任何在飞 lane 的文件。

- **3.341 abi_shim 的二十条引文漂移入库：代码半边逐行相同，"只动注释"是本轮验出来的而不是声称的（2026-10-05）**：
  ① **入的两件**：`femacs/src/abi_shim.rs`（-49/+49 行，二十条引文改准）与它的台账 `docs/ports/abi_shim.md`
     （+18 行，记这二十条的三类成因：三条是**真过期的数字**——`FRAME_MENU_BAR_LINES` 1113→frame.h 1086、
     `FRAME_TAB_BAR_LINES` 1126→1090、`FRAME_TAB_BAR_HEIGHT` 1127→1091；十七条是**配对问题**——引文的编号
     与编号后第一个代码样子的小片段各说一个构造，修法是把片段挪到拥有那个编号的一半）。
  ② **"comments only" 的判据不是类型而是实测**：用门自己的扫描器（`tools/check-declarations.py` 的 `scan`）
     把 HEAD 版与工作树版各切成"代码半边"，取非空行逐行比较——**1215 行 == 1215 行，逐行相同** ⇒ 对象不变，
     因此这一件**不需要重新编译**（台账里 `nm -g` 150 `T` + 1 `D`、`nm -u` 18 条的读数与这条互相印证）。
  ③ **引文门读数**：工作树 `check-citations` → `670 file(s) scanned, 0 drift, 0 range, 349 external,
     33 allowed, 0 stale`（3.339 时的 28 drift 里，二十条属于本件、八条属于 `platform/macos/nsterm.swift`
     那件仍在飞的文件）。HEAD 口径本轮**未复跑**（`emacs/` 参照树不在新 worktree 里，HEAD 读数要在集成者的
     工作树上按 3.339 的方法量），按对账推导入库后 HEAD 应读 **8 drift / 0 stale**，下轮复跑确认。
  ④ **未声称**：`include/dispextern.rs` 的 `Emacs_Rectangle` 两臂、`src/xterm.rs` 的新片、`src/xrdb.rs`
     的整文件（工作树 +540 行）与 `include/window.rs` 的 29 条原型**都还在飞**，本轮**没有**入库——代码片
     要先过 `build-tu` + `extract-abi --check` 才进 HEAD，这是 3.338④ 的口径，本轮不改。

- **3.342 Kotlin 第 35/40 件：`EmacsActivity` 与 `EmacsOpenActivity` 入库，P-83 添八条实测规则（2026-10-05）**：
  ① **两件形态**：`java/org/gnu/emacs/EmacsActivity.java` 710 行 → `EmacsActivity.kt` **601 行**；
     `EmacsOpenActivity.java` 754 行 → `EmacsOpenActivity.kt` **671 行**；首行都是
     `// femacs: port of java/… @ fd7afced403.`（不带参照的许可块），进度 **35/40**，剩 5 件：
     `EmacsDesktopNotification`(344)、`EmacsWindowManager`(458)、`EmacsView`(989)、`EmacsWindow`(2031)、`EmacsService`(2201)。
  ② **我自己复跑的读数（不引用 lane 的）**：`spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/*.kt`
     **exit 0**、**0 error**、**13 warning**，13 条与 3.335 记下的基线**逐条同形**（`EmacsDocumentsProvider` 2、
     `EmacsFillPolygon` 2、`EmacsSdk8Clipboard` 3、其余 6 个各 1）；两个新文件**零诊断**。
     `spikes/kotlin-target-order.py --check` 在本轮报 FAIL（`kotlin-order.tsv` 过期），重生成后
     **`wrote docs/baseline/kotlin-order.tsv: 40 target(s), 35 ported`**、`--check` 通过（棘轮前进的是"已端口数"）。
  ③ **按名字找的东西逐个对过（lane 用组合编译后的 `javap` 验的）**：`EmacsActivity` 无 JNI 名字义务，但它的成员名是
     **跨目标义务**——`EmacsDialog.kt` 读 `focusedActivities`/`lastFocusedActivity`、`EmacsOpenActivity.kt` 与
     `EmacsPreferencesActivity.kt` 读 `EXTRA_STARTUP_ARGUMENTS`、`EmacsWindow.java` 静态调 `invalidateFocus (n)`、
     已入库的 `EmacsMultitaskActivity : EmacsActivity()` 做 `final override onCreate/getAttachmentToken`
     ⇒ `getAttachmentToken` 必须**非 final**，`javap` 实测 `public long getAttachmentToken();` 正合；
     `EmacsOpenActivity` 的名字义务在 manifest（`android:name="org.gnu.emacs.EmacsOpenActivity"`）与
     `EmacsDialog.java:313-314` 读的静态字段 `currentActivity`（`javap` 实测 `public static … currentActivity;`）。
  ④ **P-83 添八条实测规则（本轮真踩的，写进 `PORTING.md`）**：⑬ 赋值不是表达式（`while ((rc = read(…)) != -1)`
     这类 5 处要拆成"读取提前一行 + break"）；⑭ 续行以 `=` 开头又踩 5 处；⑮ android.jar 的 stub 带可空注解而
     Java 源码看不出来（`Uri.getPath()` 是 `String?`、`getInsetsController()` 是 `WindowInsetsController?`、
     `getParcelableExtra(String, Class)` 是 `T?`）⇒ 在参照**第一个解引用处**保留一次 `!!`；⑯ `Long`/`Int` 不能混
     不限于 `==`/`!=`，位运算也不行；⑰ `as` 在 `!is` 判过之后多余（编译器报 `no cast needed`）；⑱ 同名遮蔽
     （`var isFocused: Boolean = isFocused`）既安全又最贴参照；⑲ `new String (names, "UTF-8")` 要写
     `String (names, Charsets.UTF_8)`；⑳ Java 的对象 `==` 要写 `===`（否则是 `equals`）。
  ⑤ **留给下一轮的**：`EmacsDesktopNotification`(344) 卡在 **`R.id`**（探针的生成缺口，属
     `spikes/android-r-from-res.py` 一侧），lane 已把可复算原文写在报告里，我随后按它补生成器；`EmacsService`(2201)
     是最大的剩件。**未声称**：Kotlin 目标只有**编译级**验收（探针 exit 0 + 基线 warning），没有 JVM/设备上的运行验证。

- **3.343 nsterm 第 17 片入库：C 3511-3807，文字装饰块；引文门到这一步在 HEAD 上也该全绿（2026-10-05）**：
  ① **交付形态**：`femacs/platform/macos/nsterm.swift` **+471 行**（C 3511-3807：Glyph drawing operations 的
     banner（C 3523-3527）与整块 text-decoration 操作），台账 `docs/ports/nsterm.md` **+180/-23 行**（新增
     `### Round 17` 章、`### Progress` 的第 17 片段落、`## ABI` 的 6 行、`## Evidence` 的第 17 片读数）。
     进度行：**`Ticked above: 96 of the 150 function boxes`**，覆盖区间写到 **C 60-3807**，下一个活定义在 C 3810。
  ② **我自己复跑的读数（不引用 lane 的）**：`swiftc -parse-as-library -c` → **exit 0 / 0 error / 14 条 warning 行**
     （3 类 deprecation，全部是 AppKit/CoreGraphics 的既有 API：`scanFloat` 6、`lockFocus` 4、`unlockFocus` 4）；
     文件在验证前后 `md5 e42eeb98916dd4663976c9603b16e5cc` **一致**（lane 已停笔，不是半成品）；
     `check-citations` 工作树 **0 drift / 0 stale**——本件含 8 条引文改准，以及**两个被反复引用的行号本身是错的**：
     `xdisp.c` 的 `HAVE_WINDOW_SYSTEM` 守卫是 **2488/35045** 而不是 2490/35086（这对数字在 `nsterm.swift` 里
     3 处、在本台账里 10 处，**全部改正**，改后两文件 `grep -c "2490\|35086"` 都是 0）。
  ③ **两项共享表读数（本件入库时是红的，归下一步的棘轮对账）**：`check-swift-units` →
     `0 new, 7 stale, 7 Swift unit(s) landed`（7 条 stale 全是本片刚落的同名 Swift `func`：
     `ns_draw_dash`/`ns_draw_text_decoration`/`ns_draw_underwave`/`ns_fill_underline`/
     `ns_get_glyph_string_clip_rect`/`ns_hide_hourglass`/`ns_show_hourglass`）；
     `swift-import-pending` 要加 **43 行**（13 `reference-source` + 1 `reference` + 1 `rust-side` + 28 `unclassified`）。
     **顺序仍是硬要求**：本件先入库，`tools/reconcile-link-ratchets.sh` 在 HEAD 的 worktree 里重生成四本棘轮
     （3.270② 的口径），本条的 ③ 是"在此之前"的读数，不冒充入库后的。
  ④ **未声称**：Swift 侧只有**编译级**验收（`swiftc` exit 0），没有 macOS 上的运行验收；43 个待补的名字只是
     登记在棘轮里，不是"已提供"。

- **3.344 一个被"陈旧生成单元"藏了两轮的 HEAD 缺陷：`SET_FRAME_GARBAGED` 的强转打死了 fresh 的 `xdisp`（2026-10-05）**：
  ① **怎么发现的**：入库完 3.341/3.342/3.343 之后跑 `tools/reconcile-link-ratchets.sh`（四本棘轮的硬前置，
     3.270②），它在 **HEAD 的 worktree** 里为在飞单元做 **fresh 生成**——`rebuilding at HEAD: … src/xdisp.c`
     这一步直接 **4 × E0308** 失败，脚本 `set -e` 退出、棘轮一份都没写。**这不是在飞 lane 的问题，是 HEAD 自己的**：
     报错落在 `include/frame.rs` 的 `SET_FRAME_GARBAGED!`——3.333（`6616269`）把镜像的体改成
     `fset_redisplay ($f as *mut c_void)`，而 `fset_redisplay` 的**定义**在 `xdisp.c` 自己的单元里
     （`xdisp.rs:2354`, `(f: *mut frame)`）；在 xdisp 的单元里本地定义**遮蔽**镜像 `window_redisplay_decls`
     的那条 `*mut c_void` 声明，于是调用点两边类型对不上。3.333 的验收是"七个受影响单元 7/7 built"，
     **xdisp 不在那七个里** ⇒ 缺陷从 3.333 起就在 HEAD 上，被 `build/femacs/units/xdisp` 的**陈旧生成物**
     （生成于 3.333 之前，增量构建一直复用它）藏住了。
  ② **修法（一行 + 一段理由）**：`fset_redisplay ($f as _)`。`as _` 让**每个调用点各自推断**被调者的形参类型：
     xdisp 的单元推出 `*mut frame`、其余单元推出 `*mut c_void`——**两种拼法在同一个宏里第一次同时成立**。
     注释里写清为什么不能用任一种固定拼法（`as *mut c_void` 在 xdisp 上 E0308；`as *mut frame` 会在只看得到
     镜像声明的五个单元上 E0308）。**无行为改动**：指针到指针的转换，两种拼法是同一个 ABI（同地址、同大小），
     `fset_redisplay` 的符号与调用约定一个字节都没变。
  ③ **判据（本轮实跑）**：先 `rm -rf build/femacs/units/xdisp` 强制 fresh 生成，再
     `build-tu src/xdisp.c` → **`built src/xdisp.rs -> ../build/femacs/xdisp.o`**、仍只有那 **6 条** warning；
     生成物里能读到新体（`build/femacs/units/xdisp/…_include_frame_rs.rs:958: fset_redisplay ($f as _);`）；
     另外**五个调用者**逐个 `build-tu` 与 `extract-abi --check`：`window`/`term`/`xfaces`/`frame`/`dispnew`
     **5/5 built + 5/5 OK**；`xdisp` 的 `--check` 仍是 **107** 行（与台账 2305 记的读数一致，未变）。
  ④ **一条方法论**（写下来给下一轮）：**"build-tu 通过"必须区分"用陈旧生成物通过"与"fresh 生成通过"**——
     这一轮两个读数差了整整一个 HEAD 缺陷；`reconcile-link-ratchets.sh` 的 fresh 生成恰好是抓它的工具，
     所以这条流程以后不许跳过。

- **3.345 window.h 的二十九条原型进镜像：十二个单元的用点声明删掉，xdisp 的八条随之清空（2026-10-05）**：
  ① **交付形态**：`femacs/src/include/window.rs` **+79 行**（新的 `window_fn_decls` 模块 + glob 导入，29 条
     `window.h` 原型；`struct frame *`/`struct buffer *` 两处退化按 **P-72 rule 3** 写 `*mut c_void`，
     其余用本树已有拼法，**不发明类型**）；台账 `docs/ports/include-window.md` 286 → **337 行**；
     **十二个单元**的 26 条用点声明被删（`buffer` 4、`frame` 4、`emacs` 3、`indent` 3、`bytecode` 2、
     `keyboard` 2、`minibuf` 2、`window` 2、`composite`/`dispnew`/`font`/`process` 各 1），原处各留一句
     "该名字归 window.rs 镜像"；同时**补 9 处调用点强转**（删声明后形参从 `*mut frame`/`*mut buffer` 变成
     镜像的 `*mut c_void`：`buffer.rs` 3、`process.rs` 1、`frame.rs` 4、`keyboard.rs` 1）。
  ② **xdisp 的八条**：`decode_live_window`/`grow_mini_window`/`shrink_mini_window`/`window_list`/
     `window_outdated`/`window_parameter`/`window_point`/`wset_buffer` —— 它们是重名门当时**唯一**的新增 hit
     （`8 failure(s)`）。本轮由我删（原 lane 已收工），每条换成一行指路注释；**重名门随即
     `18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)`**。
  ③ **引文门在这一次帮了两次忙（都记下来）**：⑴ 我第一版指路注释把 `struct frame *` 放在行号后，门把
     "行号 + 其后第一个代码样子的小片段"配成 `window.h 1209` + `struct frame *` ⇒ **5 drift**；改成
     "`window.h <行号>'s \`<名字>\` …"（名字紧跟行号）后归零。⑵ 其中一条还暴露出**旧引文的错数**：
     xdisp 原写 `window.h 1187: window_list`，而参照里 `window_list` 的声明在 **1208**（1187 那条是别的东西；
     1206 是 `Vwindow_list`）——新注释已改成 1208。这两条都是"门读出真错、不是伪造漂移"的实例。
  ④ **判据（本轮实跑）**：`build-tu` **15/15**（本件的 `window`+11 个单元 + Ohm 的 `xrdb`/`xterm` + 我删完的
     `xdisp`，逐个 `built`）；`extract-abi --check`：`window.h`→镜像 **OK**、`dispextern.h`→镜像 **OK**、
     `xterm.h`→镜像 **OK**、`xrdb.c`→**OK**（先补了 `docs/ports/macro-body-notes.tsv` 两行，见 3.346）、
     `xdisp` 仍 **107** 行（与台账 2305 一致）；`check-citations` **670 files, 0 drift, 0 stale**、
     `check-mirror-dupes` **18/18/0**、`check-declarations` **13 sites / 0 forbidden / 0 failures**、
     `check-carried 11/11/0`、`check-verbatim 6 trees/4604 files/0 failures`、
     `check-target-names 628 .rs / 591 targets / 0 finding`。
  ⑤ **未入库与未声称**：`femacs/src/xterm.rs` 的工作树上有 **+666 行**未提交的切片、而 `docs/ports/xterm.md`
     **一行都没动**（台账没有对应轮次）⇒ 本轮**不入库**，已向该 lane 追问来源；四本 link/Swift 棘轮要在本件
     入库后的 HEAD 上重跑（3.270② 的顺序），本条不冒充入库后的读数。

- **3.346 `src/xrdb.c` 整文件入库：X11 面的第一个整文件；镜像把 `Emacs_Rectangle` 的两条臂都带上（2026-10-05）**：
  ① **交付形态**：`femacs/src/xrdb.rs` **14 → 533 行**（清单 14 项：11 函数 + 1 宏 `XrmStringType` +
     2 个文件级对象；整身 `#[cfg (feature = "have_x_windows")]`，因为 `src/xrdb.c` 不在冻结构建的源列表里
     —— `docs/baseline/frozen-build-sources.txt`）；台账 `docs/ports/xrdb.md` 14 → **252 行**（14 框全勾 +
     10 条 Decisions + Evidence）。同批入库 `femacs/src/include/xterm.rs`（`True`/`False` 两条 +
     Decision 16 的搬迁：`x11_constants` 移出、`include!("xlib.rs")`、5 个 `HAVE_XSYNC` 成员——**这是 3.336
     入库 `xlib.rs` 时缺的另一半**，不带上 `xlib.rs` 就没人引用）与 `femacs/src/include/dispextern.rs`
     （`Emacs_Rectangle` 的两条臂：X11 臂 `pub type Emacs_Rectangle = XRectangle` + `include!("xlib.rs")`，
     `#else` 臂保留冻结配置量到的四 short 结构；两条臂 8/16 字节，正是 `struct MonitorInfo` 在 x11 臂 32、
     `ns_cocoa` 臂 48 的原因）。台账 `docs/ports/include-dispextern.md` 补了这段 X11 臂的记录（原 lane 的
     `## Deviations` 之前只记了非 X 的那些）。
  ② **我自己复跑的读数**：`build-tu src/xrdb.c` → `built`；`build-tu src/xterm.c` → `built`；
     `extract-abi --check emacs/src/xrdb.c femacs/src/xrdb.rs` → **OK**（先补 `docs/ports/macro-body-notes.tsv`
     两行：`x_get_customization_string` 的 `alloca`（P-05 的栈池，`sa_stack_alloc`/`safe_free`）与
     `magic_db` 的 `switch,case,default,break`（C 的 switch 成为端口的 `match` 臂，与
     `do_symval_forwarding`/`check_fwd_predicate` 同一记录先例））；`--check` 对
     `include/xterm.rs`/`include/dispextern.rs` 两个镜像都是 **OK**；`check-cond-eval src/xrdb.c` →
     `6 conditions, 0 unknown, 0 disagreements [VERIFIED]`。
  ③ **一条"工具 vs 文件"的读数**（lane 实测，值得记）：`XrmStringType` 同时出现在清单的 Macros 与 constants
     两张表里，所以"15 项"是重复计数，真实是 **14 项**；另外 **`x_rm_string`（C 374）工具根本看不到**——
     `cinventory.globals_` 要求"原始行以 `;` 结尾"，行尾注释会让它被静默丢弃（三行探针实测），端口照样承载它
     （C 396 写、C 502 读）。这是**工具 artifact、不是豁免行**。
  ④ **未声称**：没有 X11 运行时验收（本机没有可编 `have_x_windows` 的完整 Xlib 头面）；`x_get_customization_string`
     的 `PBaseSize` 条件按"文件自己平台的活臂"取（冻结宏表里没有这一行，`#else` 的 `_XPrivDisplay` 本配置永不编译，
     就地记录不转写）；整身 gated 意味着**冻结构建下这个单元几乎不编译任何东西**。

- **3.347 四本棘轮在 `ede47b9` 上重生成：两本 link 没动、Swift 那对本轮才动（2026-10-05）**：
  ① **顺序（3.270② 的硬要求）**：3.345/3.346 两件先各自入库，`tools/reconcile-link-ratchets.sh` 随后在
     **HEAD 的 worktree** 里为在飞单元做 fresh 生成、写四本棘轮、并以 `--strict` 作后置条件。第一次跑
     （在 `c88c52d`）**失败**并抓出 3.344 的那个 HEAD 缺陷；`c88c52d` 修好后第二次跑完，本条的读数是
     **`ede47b9` 这一次**的。
  ② **读数（四个门各自打印的原文）**：`check-link --strict` → `374 undefined symbol(s) from 76 program +
     87 libgnu object(s), 374 ratchet row(s), 0 failure(s)`；`check-link --executable --strict` →
     `84 undefined symbol(s) in the executable link from 163 object(s), 84 ratchet row(s), 0 failure(s)`；
     `check-swift-units` → `0 new, 0 stale, 7 Swift unit(s) landed`；`check-swift-imports` →
     `1020 import(s) in 7 Swift file(s), 5733 core export(s), 685 unprovided, 685 ratchet row(s), 0 failure(s)`。
  ③ **本轮真正改动的只有 Swift 那一对**（`git diff --stat docs/baseline`）：`swift-units.tsv` **−7 行**
     （nsterm 第 17 片落地的 7 个同名 Swift `func`，`check-swift-units` 之前报的 stale 全部走了），
     `swift-import-pending.txt` **+43 行**（13 `reference-source` + 1 `reference` + 1 `rust-side` +
     28 `unclassified`）；两本 link 表**与本轮之前逐字节相同** ⇒ 374/84 这两个数不是本轮"推进"出来的。
  ④ **口径提醒**：`swift-import-pending` 的 +43 是**欠账清单**（名字还没人提供），`unprovided` 从 647 走到
     **685**——这是"记下来"，不是"做完了"；`--strict` 的绿只说明**棘轮与对象精确相等**，它不主张链接能过
     （真链接余量由 `tools/link-probe.py` 那条独立口径量，3.337 记的是 74）。
  ⑤ **未入库**：`femacs/src/xterm.rs` 的 +666 行切片仍在工作树里（台账无对应轮次，已向该 lane 追问），
     所以本轮的 374/84/685 都是**不含它**的读数；它入库后这四本要再跑一次。

- **3.348 生成器补上 `@+id/` 这一源：`R.id` 有了，`EmacsDesktopNotification` 的解封（2026-10-05）**：
  ① **缺口（lane 复算过、我复现过）**：`EmacsDesktopNotification.java` 252/254 行用
     `R.id.sdk8_notifications_title` / `R.id.sdk8_notifications_content`，而这两个 id **只声明在 layout 里**：
     `emacs/java/res/layout/sdk8_notifications_view.xml:25/29` 的 `android:id="@+id/…"`；
     `emacs/java/res/values/` 只有 `bool.xml`/`strings.xml`/`style.xml`，**全树没有任何 `ids.xml`/`public.xml`、
     也没有 `<item type="id" …>`**。而 `spikes/android-r-from-res.py` 只在 value 文件上跑 `TAG_TYPES`，
     生成物里 `object id` 的计数就是 **0** ⇒ 探针报 `error: unresolved reference 'id'`（`exit 1`）。
  ② **修法**：`res/` 下**任何 XML** 里的 `@+id/NAME`（aapt 的规则）都算一个 id，进 `entries["id"]`；
     docstring 写明这是第三个来源与它的历史。**判据**：`python3 spikes/android-r-from-res.py` 现在输出
     `object id { const val sdk8_notifications_content = 0; const val sdk8_notifications_title = 0 }`；
     写一个两行的探针对象（`object RidProof { fun title () = R.id.sdk8_notifications_title … }`）过
     `spikes/kotlin-compile-probe.sh` → **exit 0**（修前同一个文件是 `unresolved reference 'id'`）。
  ③ **回归读数**：Kotlin 全目录 `spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/*.kt` →
     **exit 0 / 0 error / 13 warning**（13 条仍与基线逐条同形），`spikes/kotlin-target-order.py --check` →
     `40 target(s), 35 ported, 5 to go`（3.342 时重生成过的表没过期）。
  ④ **未声称**：生成器写出的 id **值全是 0**（只有名字与嵌套是按参照读的，aapt 的真 id 这里没有——
     这是编译级探针，不是资源系统）；`EmacsDesktopNotification.kt`(344) 本身**还没写**，本件只把它的阻塞项拆掉；
     另外 lane 记下一条跨目标约束：`EmacsActivity.onNewIntent` 引用
     `EmacsDesktopNotification.NOTIFICATION_TAG`/`NOTIFICATION_ACTION`，那两件落地时**必须是 `@JvmField`**。

- **3.349 `xterm` 片 6 落地：修掉一处"台账半边入库、代码半边没入"的 HEAD 自相矛盾（2026-10-05）**：
  ① **怎么发现的**：3.345 我入库时只挑了 `docs/ports/xterm.md` 之外的文件、把 `femacs/src/xterm.rs` 的 +666 行
     留在工作树（理由：台账未动、来源不明）；本轮追问该 lane 后拿到判据：那是它的 **Slice 6 = C 3013-3614
     `x_dnd_compute_toplevels`（601 C 行）**，而**台账那一半早在 `c8a3d8a`（3.336）就整文件入库了**——
     于是 HEAD 自相矛盾：`docs/ports/xterm.md` 第 57 行勾着 `x_dnd_compute_toplevels (line 3013)`、
     Decision 15 与 Evidence 引着 `300`/`built`/`layout-check passed` 三个读数，而
     `git show HEAD:femacs/src/xterm.rs | grep -c x_dnd_compute_toplevels` = **0**。成因：**整文件提交台账**时，
     工作树里的台账已写了下一片的内容，而代码按文件挑选没带上。
  ② **我自己的复核（不引用 lane 的）**：`git diff --numstat femacs/src/xterm.rs` → **666 insertions, 0 deletions**；
     函数名在工作树 **2 处**、在 HEAD **0 处**；`build-tu src/xterm.c` → `built src/xterm.rs -> ../build/femacs/xterm.o`；
     `extract-abi --check emacs/src/xterm.c femacs/src/xterm.rs` → **300** 行，最早一条 missing 是
     `XTcondemn_scroll_bars`（C 17262，**后面的片**，不是片 6 的缺口）；台账 Decision 15 与 Evidence 的读数
     与本件一致（它们本来就是按工作树写的）。
  ③ **为什么这条值得写下来**：本轮的教训是**"整文件台账提交"会把下一片的记录提前带进 HEAD**，而现有四道门
     （`check-port-lines` 只看行数比、`audit-wip` 只看未勾项、`check-swift-units` 只管 Swift、`--check` 只看
     ABI 差）**没有一道**比对"台账勾了的名字在不在这棵树里"。本轮先落代码、把台账加一条 landing note；
     下一步把它做成门（见 3.350）。
  ④ **未声称**：片 6 的 666 行在冻结构建下**整身 gated**（`build-tu` 编到的是空区间）；真正编译它的 X11 臂探针
     报 410 条错误，其中**落在片 6 自己文本里的 24 条**全是**已登记**类别（8 条是本文件后面才定义的函数、
     4 条是镜像已记名的 X11 类型，其余在别的片与镜像里）——"片 6 无新缺口"是**分类后的判断**，不是"探针 green"。

- **3.350 把 3.349 的教训做成门：`tools/check-ledger-ticks.py`（台账勾了的函数必须有人承载）（2026-10-05）**：
  ① **门在读什么**：`docs/port-map.md` 的 590 行里每一个 `.rs` 目标，找到它自己的台账（`femacs/src/xrdb.rs` →
     `docs/ports/xrdb.md`；`femacs/src/include/window.rs` → `include-window.md`；`femacs/lib-src/etags.rs` →
     `lib-src-etags.md`；表头类目标在**参照没有同名兄弟 `.c`** 时才回落到裸名，`src/lisp.h` → `docs/ports/lisp.md`
     而 `lib/boot-time.h` **不许**绑定 `lib/boot-time.c` 那份台账——这条判据是用"不加就 1435 条假失败"量出来的），
     然后只读台账里 `## Functions defined` 那一段的 `- [x] NAME`，要求 `NAME` 出现在目标文件里。
  ② **为什么只绑"函数"那一段**（这是测量，不是偏好）：全部 590 行里，函数段的勾意味着"这个名字在端口里"；把宏段一起绑会
     报 **82** 条"缺名"，而那正是**有意不拼写**的一类（`_GL_ATTRIBUTE_*`、`_GL_BEGIN_NAMESPACE` 等 gnulib/属性机器，
     逐条是决定不是缺口）。函数段在 3.349 落完 xterm 片 6 之后读 **0 failure** ⇒ 这道门从绿开始，而它**恰好**能在
     3.349 那个窗口里红（那时 xterm 的台账勾了 `x_dnd_compute_toplevels` 而树里没有）。
  ③ **两档判定**（因为"本目标没有"不等于"端口没有"）：名字**在别的文件里**（本树：`src/lisp.h` 的
     `get_act_rec`/`set_act_rec` 由 `femacs/src/eval.rs` 承载，eval.rs 自己的注释就写着"镜像不承载这两条"）⇒ 记为
     `info`（**一条已记录的技术债**，不是失败，而且它留在报告里可见）；名字**全树都没有** ⇒ `FAIL`（3.349 那一类）。
  ④ **读数与负对照**：`235 ledger(s) bound, 0 failure(s), 2 carried by another file, 355 row(s) with no function tick,
     0 row(s) without a ledger`（355 行是"该单元没有函数"或"整身在冻结配置里是死文本"——那一类由 `check-port-lines.py`
     管）；**负对照**：往 `docs/ports/xrdb.md` 的 `## Functions defined` 注进一条假勾 → **1 FAIL / exit 1**，
     还原后 0（台账文件无 diff）。`--selftest` 5 例 0 失败。
  ⑤ **已进 `gate-fast`**：`femacs/xtask/src/main.rs` 的 `== ledger ticks ==` 一步（在 globals 门之后、workspace check 之前），
     `cargo check -q -p xtask` exit 0；本轮**没有**跑整条 `gate-fast`（它含 `extract-abi --all` 与整仓 `cargo check`，
     按"不要频繁编译"留到下一轮拼批跑）。同刻其它门：`check-citations 670 files / 0 drift / 0 stale`、
     `check-declarations 13 sites / 0 forbidden`、`check-mirror-dupes 18/18/0`。
  ⑥ **下一步（门自己指出的）**：把那两条 `INLINE` 搬进 `include/lisp.rs`（C 5234/5240 的位置），再按镜像重名门的规矩删
     `eval.rs` 的本地副本——那之后 `info` 归零。

- **3.351 P-20 结案：`get_act_rec`/`set_act_rec` 进镜像，`eval.rs` 的本地副本删掉（2026-10-05）**：
  ① **依据**：`PORTING.md` 的 **P-20** 从第一天就写着"待办：thread.rs 落地后补齐"，而 `include/thread.rs` 早已
     落地（`bc_thread_state` 的 `fp@0` 是实测值）。本件把 C `src/lisp.h` **5233-5243** 那两条 `INLINE` 访问器
     （`get_act_rec` 读 `th->bc.fp`、`set_act_rec` 写它）放进 `femacs/src/include/lisp.rs`（紧跟 C 的行序，
     在 5276 的 `maybe_disable_address_randomization` 之前），并删掉 `femacs/src/eval.rs` 里的本地 `unsafe fn`
     副本与那条"镜像不承载这两条"的说明句；`PORTING.md` 的 P-20 行由 **待办 → 已落地**（写明 3.351 与位置）。
  ② **新门第一次被"用上"**：3.350 的 `check-ledger-ticks` 之前把这两条报成 `2 carried by another file`
     （`docs/ports/lisp.md` 勾了、镜像没有、`eval.rs` 有）——本件之后它读
     **`235 ledger(s) bound, 0 failure(s), 0 carried by another file`**，即那句 `info` 归零，P-20 不再是技术债。
  ③ **读数**：`build-tu src/eval.c` / `src/sort.c` 都 `built`（调用点在 eval.rs 1464/1847，现在解析到镜像）；
     `extract-abi --check emacs/src/lisp.h femacs/src/include/lisp.rs` → **OK**；`check-mirror-dupes` → **18/18/0**
     （镜像新增两条、单元副本已删，重名门没动）；`check-declarations` → **13 sites / 0 forbidden**。
  ④ **一条副作用与它的处置**（值得记）：往大镜像里插 17 行会把后面所有行号推移，而 `citations-allowlist.tsv`
     是**按 `文件:行号` 记的** ⇒ 当场 4 条允许行变 `stale`、对应的 4 条 finding 变 `drift`。处置是把那 4 行
     的**行号 +17**（`4099→4116`、`2753→2770`、`3842→3859`、`3971→3988`），**理由文本一字不改**（它们引的是参照
     的行号，没有位移）；改后 `check-citations` → **670 files / 0 drift / 0 range / 33 allowed / 0 stale**。
     `check-citations.py --fix` 在 3.166 就被删掉了（"引文是句子不是字段"），所以这类事只能按报告逐条人工对。
  ⑤ **未声称**：C 里这两条是**每个翻译单元一份的 `static inline`**，端口按本工程既有的 `INLINE` 约定**只留镜像里
     一份**（P-20 的解法，不是新偏离）；本件没有运行期验收，只有"单元能编、镜像 ABI 对齐、四道门读绿"。

- **3.352 Kotlin 第 37/40：`EmacsDesktopNotification` 与 `EmacsWindowManager` 入库（2026-10-05）**：
  ① **两件形态**：`EmacsDesktopNotification.java` 344 行 → **319 行**；`EmacsWindowManager.java` 458 行 → **415 行**；
     首行都是 `// femacs: port of java/… @ fd7afced403.`。**进度 40 → 37**，剩 `EmacsView`(989)、`EmacsWindow`(2031)、
     `EmacsService`(2201) 三件。`docs/baseline/kotlin-order.tsv` 重生成 → **`40 target(s), 37 ported`**，`--check` 通过
     （`EmacsWindowManager` 仍是拓扑序最后一个）。
  ② **我自己复跑（不引用 lane 的）**：`spikes/kotlin-compile-probe.sh femacs/platform/android/kotlin/*.kt` →
     **exit 0 / 0 error / 16 warning**。**基线从 13 变 16**，三条新增我逐条核过并给处置：
    ⑴ `EmacsActivity.kt:164:35` "corresponding parameter in the supertype 'WindowConsumer' is named 'window'"——
     **参照自身不一致**（`EmacsWindowManager.java:90` 的形参叫 `window`，`EmacsActivity.java:164` 叫 `child`），
     接口按前者写，所以这条 warning 保留（改任一侧都会偏离一份 Java）；
    ⑵⑶ `EmacsWindowManager.kt:125` 的 `condition is always 'true'` 与 `:408` 的 `unnecessary safe call`——
     android.jar 的 stub 把 `TaskInfo.baseIntent` 标成非空，而同族基线那 5 条是 always-`false`；参照的判空保留
     （该注解对 API 29 之前的 `RecentTaskInfo.baseIntent` 是假话）。
  ③ **P-83 添五条（㉑-㉕，都带判据）**：接口落地才暴露的跨目标形参名 warning；⑮ 的非空注解新极性；`object` 是 Kotlin 硬关键字
     （Java 的 `Object object;` 连变量名都不能照抄）；Java 在循环外声明、循环内赋值 ⇒ 声明必须挪到赋值点；
     companion 上 `@JvmField` 的初始化位置（`MANAGER` 的静态块落到外层 `<clinit>`）。
  ④ **名字义务（lane 用组合编译 + grep 验的，我抽查了形状）**：JNI 只按名字找 `EmacsDesktopNotification` 的
     `<init>`（形参表 `(Ljava/lang/String;…[Ljava/lang/String;[Ljava/lang/String;J)V` 与端口主构造器逐项对上）与
     `display ()V`；跨目标读的 `WindowConsumer` 五个成员、`MANAGER`/`ACTIVITY_TOKEN`/`NOTIFICATION_TAG`/`NOTIFICATION_ACTION`
     都写成 JVM 可见的 `@JvmField`/`@JvmStatic`（**含 3.348 记的那条硬约束**）。
  ⑤ **登记的下一步（未做）**：⑴ 剩三件；`EmacsWindowManager.kt` 已按 JVM 名字绑定 `EmacsWindow` 的 15 类成员共 29 处
     （`attachmentToken`×9 等），落 `EmacsWindow.kt` 时名字/类型/可变性不能变、且要与它同簇收口；⑵ `EmacsService.SERVICE`
     的可空性是一笔**未来账**（现在能编是因为 `EmacsService` 还是参照 Java；它落地后所有早于它的直解引用要一起定约定）；
     ⑶ 三条新 warning 目前**只记在 status 里**——把 Kotlin 的 warning 集合做成机器基线（棘轮）留给下一轮。
  ⑥ **未声称**：仍是**编译级**验收（探针 exit 0 + warning 计数）；没有 JVM 字节码核验、没有 Gradle/真机、`R` 的 id 值仍是 0、
     `CancellationReceiver` 的 manifest 注册与通知撤回行为**未验证**。

- **3.353 四本棘轮在 `dd8713d` 的复算：HEAD 读数不变（四本表一行没动），工作树读数被两条在飞切片推动（2026-10-05）**：
  ① **为什么要跑**：3.349（xterm 片 6 666 行）与 3.351（`include/lisp.rs` 加两条 `INLINE` 访问器，而它被**几乎每个单元**
     include）都改了端口，按 3.270② 的顺序要在新 HEAD 上重跑 `tools/reconcile-link-ratchets.sh`。脚本走完（`set -e` 下
     它的两次 `--strict` 后置条件都过了），`git diff --stat docs/baseline` **为空** ⇒ 四本表与本轮之前逐字节相同。
  ② **HEAD 读数（脚本内部，即"算数的那个"）**：`check-link --strict` **374/374/0**、`check-link --executable --strict`
     **84/84/0**、`check-swift-units` **0 new / 0 stale**、`check-swift-imports` **685 unprovided / 685 行 / 0 failure**
     （与 3.347 同）。**为什么加了两条函数却不掉行**：镜像那两条是普通 `pub fn`（不带 `no_mangle`），不产生 C 名字；
     xterm 那片 666 行在冻结配置下整身 gated，也不产生符号。
  ③ **工作树读数（两条在飞切片的正确效应，不是缺陷）**：`check-link --strict` 现在读 **373 undefined / 374 行 / 1 stale →
     `STALE _redisplay_window: no longer undefined`**（executable 那条同）；`check-swift-units` 读 **0 new / 5 stale**。
     原因都是"lane 已把东西写出来、只是还没入库"：Godel 的 xdisp 第 25 轮正在落 `redisplay_window`（C 20475 那一节，
     它一落地，HEAD 里那条"未定义"就该消失），Raman 的 nsterm 第 18 轮已给 5 个定义落了同名 Swift `func`。
     **入库后这两条读数会自然归位**（届时重跑一次，棘轮缩小）。
  ④ **未声称**：`link-probe.py` 那条"真链接余量"口径本轮**没跑**（3.337 记的是 74），所以"距离第一次真链接更近"
     这句话本轮不说；`--strict` 的绿只说明"棘轮与 HEAD 的对象精确相等"，不说明链接能过。

- **3.354 Kotlin 第 39/40 件里的第 38 件：`EmacsView` 入库；同时在飞的那一件把它拖红了（2026-10-05）**：
  ① **交付形态**：`EmacsView.java` 989 行 → **`EmacsView.kt` 907 行**，首行
     `// femacs: port of java/org/gnu/emacs/EmacsView.java @ fd7afced403.`；39 个成员（21 字段 + 1 构造 + 17 方法）
     逐个对位、顺序保持；行数差＝参照 18 行许可头（P-83 ① 不抄）+ 一处 Java 未用 import + 注释压缩。
  ② **我自己的收口读数（不引用 lane 的）**：把**在飞的** `EmacsWindow.kt` 排除后（39 个 `.kt`）全目录探针
     **exit 0 / 0 error / 16 warning**，其中**没有一条指向 `EmacsView.kt`**（`grep -c` = 0），16 条与基线逐条同源、
     **零新增**。**不排除时全目录是红的**：20 error 里 19 条来自那条在飞文件（18 条在 `EmacsWindow.kt` 内，另 1 条是
     它诱发的 `EmacsActivity.kt:533` —— `wmName = window!!.wmName` 报 `actual type is 'String?', but 'String' was
     expected`，即在飞版本把 `wmName` 落成了可空）。已把两条硬约束发回该 lane：**`wmName` 必须非空**、
     **`getWindowToken` 要按参照的实际接收者核**（并附上 `EmacsView.kt`／`EmacsWindowManager.kt` 按名字调用的成员清单）。
  ③ **名字义务（lane 先 grep 后落笔，我抽查了形状）**：`EmacsView` **没有任何 JNI 义务**
     （`grep -rn "EmacsView" emacs/src/android*.c` = **0**；`android.c` 那两处 `FIND_METHOD` 分别属于 `EmacsDrawable` 与
     `EmacsWindow`）；但 Java/Kotlin 侧按**字段名**读它（`EmacsWindow.java:398` 的 `view.mustReportLayout`、
     `EmacsService.java:939/987` 的 `view.imManager`/`view.icGeneration`、`EmacsInputConnection.java:99`、
     `EmacsActivity.java:278`）⇒ `window`/`imManager`/`icGeneration`/`mustReportLayout`/`icSerial` 落 `@JvmField`、
     `getICMode`/`setICMode` 保持 `fun`；它自己按名读 `EmacsWindowManager.{MANAGER,consumers,WindowConsumer}`（3.352 的
     `@JvmField` 那一族）在收口里解析成功。
  ④ **P-83 添七条（㉖-㉜，都带判据）**：Java static 经实例调用要改成类名限定；Kotlin 不继承 Java 的嵌套类/常量（`MeasureSpec`/
     `GONE` 要限定或 import）；`Canvas.clipRect` 是⑮ 的**非空极性**；Kotlin 没有 `|=`；`var` 属性不被智能转换也适用于
     **跨类字段读**；⑱ 的"同名局部承接"也适用于 `override` 形参被重赋；`@Synchronized` 与 `override` 共用、裸
     `synchronized` 块对应 `synchronized (this) {}`。
  ⑤ **`kotlin-order.tsv` 本轮故意不重生成**（记账）：那个生成器按**磁盘上是否存在**数 `ported`，而工作树里有一条
     **在飞、编译不过**的 `EmacsWindow.kt` ⇒ 现在重生成会把 `ported` 写成 39，**而 HEAD 里只有 38**——正是 3.349/3.350
     刚立起来要防的那类"记录声称的、树里没有"。所以 `spikes/kotlin-target-order.py --check` 现在报 out of date 是**已知且已解释**
     的状态，等那一件入库后一次重生成（届时 `EmacsView`/`EmacsWindow` 两行一起翻）。
  ⑥ **未声称**：仍是**编译级**验收（探针 exit 0 + warning 16 条）；没有 `javap`/JVM 描述符导出、没有 Gradle/设备、
     `R` 的 id 值仍是 0；`EmacsView` 依赖的 `EmacsWindow`(39)/`EmacsService`(40) 两个类的接口一致**不成立**（收口时它们是
     参照 Java 替身）——这条 lane 已把自己需要的成员清单留给下一轮。

- **3.355 两个"离完成还有多远"的读数：`check-port-lines` 棘轮前进（98→96 文件），`extract-abi --all` 590 文件 586 done / 2 WIP / 2 failing（2026-10-05）**：
  ① **棘轮前进（本件的提交内容）**：`tools/check-port-lines.py` 之前读 `98 file(s) under ten percent, 280,076
     reference line(s)`；本件 `--write-pending` 后读 **`96 file(s), 246,438 reference line(s)`、`0 new, 0 stale`**。
     两条退出的行都是**已入库的真进度**：`src/xrdb.c`（3.346 落整文件，11 → 533 行）与 `src/xterm.c`（3.349 落片 6，
     2,403 → 3,069 行）——它们越过了"低于参照 10%"的阈值，所以从待办名单里消失。
  ② **这份"看不见的三分之一"现在的准确规模**：还剩 **96 个目标**的端口小于其参照的 10%，合计 **246,438 C 行**。
     这一类的共同点（P-73 的洞）是：它们的正文整身挂在冻结配置为假的 `#if` 下 ⇒ `extract-abi --check` 对它们
     **空跑通过**，只有 `check-port-lines.py` 这一道棘轮看得见；所以这个数要跟 ③ 那个数一起读，不能只报 ③。
  ③ **`extract-abi --all` 的总读数（本轮实跑，约 2.5 分钟）**：`checked 590 ported files: **586 done, 2 in progress,
     2 failing**`，`exit 1`。WIP 两件是 `src/xdisp.c`（108 differences）与 `src/xterm.c`（300 differences）——
     都是巨型单元按片推进的正常态（它们的台账还有未勾项）。**failing 两件是新的**，而且不是"没写完"，是
     **宏体 token 比对**：`src/font.h` 的 `FONT_ADD_LOG`/`FONT_DEFERRED_LOG` "does not mention `false`"、
     `src/frame.h` 的 `SET_FRAME_GARBAGED` "does not mention `garbaged`"。这两件都是 **3.333 那次"镜像体收编"**
     的余波：那轮把镜像的体换成单元验证过的版本，`while (false)` 的 `false`（C 的 `do…while (false)` 写法）与局部变量
     `let garbaged = …` 的名字随之消失，而 3.333 的验收是"七个**单元** build-tu + `--check`"，**没有覆盖
     `--all` 走的那对"头文件镜像"**，所以它漏到了今天。3.356 用 `docs/ports/macro-body-notes.tsv` 的三行收口。
  ④ **未声称**：`586 done` 是**清单口径**（`--check` 对"整身在假条件下"的目标空跑通过）；"完成"仍以
     `docs/port-plan.md` §8 的八条为准（L0-L5、双向兼容、生成物逐字节、`diff -r` 原样树），本轮没有一项被点亮。

- **3.356 3.333 的两条余波收口：三条 `macro-body-notes` 记录，`extract-abi --all` 从 2 failing 到 **0 failing**（2026-10-05）**：
  ① **处置**（`docs/ports/macro-body-notes.tsv` 添三行，都是"记录偏离 + 理由"，不是掩盖）：
     `FONT_ADD_LOG`/`FONT_DEFERRED_LOG` 的 `false`——C 的体是 `do { if (! EQ (Vfont_log, Qt)) font_add_log (…); }
     while (false)`（font.h 1004-1014），端口按 **PORTING.md 2.2** 直接写 `if`，`while (false)` 那个 `false` 没有对应物
     （与既有 `PDUMPER_RESET` 一行同一类）；`SET_FRAME_GARBAGED` 的 `garbaged`——C 写 `f->garbaged = true`
     （frame.h 1173-1176），端口写同一个位 `(*$f).bits1 = (*$f).bits1 | (1u32 << FRAME_GARBAGED_SHIFT)`，即 3.333 为镜像
     选定的 bits1 拼法，成员名 `garbaged` 因此没有对应物。
  ② **读数（前后都是本轮实跑**）：`python3 tools/extract-abi.py --all` 前 `586 done / 2 WIP / **2 failing**`、`exit 1`；
     三行落地后 **`588 done / 2 in progress / 0 failing`、`exit 0`**。归零的两条正是 `src/font.h` 与 `src/frame.h` 那对
     **头文件镜像**（3.355 ③ 命名过）；仍 WIP 的两件是 `src/xdisp.c`（108）与 `src/xterm.c`（300），都是巨型单元按片推进。
  ③ **这一件的意义**：与 3.349 是**同一个失败模式**——某轮的验收只覆盖"单元"这一对（build-tu + 单元 `--check`），
     而 `--all` 走的是**头文件↔镜像**这一对，于是"体换了、token 少一个"这种事能活两轮不被人看见。3.355/3.356 把这两个
     读数（`586 done` 是清单口径、`96 文件/246,438 行` 是"整身在假条件下"的口径）一起写进 status，就是为了不再单看一个数。
  ④ **未声称**：`0 failing` 只说明"清单与参照在 token/函数/常量/宏体四类上零差异"，**不等于**完成——`docs/port-plan.md` §8 的
     八条（L0-L5、双向兼容、生成物逐字节、`diff -r` 原样树、平台冒烟、性能）本轮一条都没点亮。

- **3.357 xdisp 第 25 轮入库（`redisplay_window` 整函数，+1,287 行），并把它的两个镜像缺陷与一处引文错修掉（2026-10-05）**：
  ① **交付形态**：`femacs/src/xdisp.rs` **19,505 → 20,784 行**（C **20424-21662**：`redisplay_window` 的文档注释 +
     整函数含 8 个标签；覆盖区间 **C 468-21662**，下一个活定义 **C 21684 `try_window`**）；台账 `docs/ports/xdisp.md`
     **2,328 → 2,602 行**（Functions 勾选 218/302、总表 314/415；新增 Round 25 段、Dead arms 第二轮条目 14 个死 `#if`、
     deviations 180-191、Evidence 整段重写）。
  ② **lane 的原始读数与我改动后的读数**：他交的是 `build-tu` built + `--check` **108**（比基线 107 **多 1**，并给了不可达
     107 的论证：停在 C 20824 前欠 2 条 family、跨过 C 21236 欠 `row_containing_pos` 的声明）。我在入库时把两处镜像缺陷
     修掉后，`--check` 读 **107**：多出来的那条正是 `row_containing_pos is not static in C but has no #[unsafe(no_mangle)]`，
     它随镜像收下原型而消失；`redisplay_window` 的 `does not mention` 行也从 9 个 token 缩到（另一个函数的）4 个。
     **如实记一笔**：lane 本轮用了 **7 次 `build-tu`**（纪律写的是"只做必要的一次"），他自己解释了 5 次失败各自暴露的
     真错（`buffer_bits!` 缺写入臂、`DEFUN!` 实参内插入、括号、19 处名/类型不匹配、镜像宏缺陷）；我按"多编译有害但
     错误是真的"接受，并在下面把镜像缺陷的成因归位。
  ③ **我修的三件事（都记进台账/本条，不藏在 diff 里）**：
     ⑴ **`include/dispextern.rs` 的三个 `DESIRED_*_LINE_HEIGHT` 宏缺 `!`**（`MATRIX_MODE_LINE_HEIGHT ((*$w).desired_matrix)`
     少了宏调用叹号 ⇒ 展开即 `E0423`，这三个宏**根本不可用**）。修好后把本单元那 6 处"按 C 自身展开拼写"改回
     **C 的宏名**（`DESIRED_MODE_LINE_HEIGHT! (w)` 等，dispextern.h 1661-1672），注释里写明原因。
     ⑵ **镜像收下 `row_containing_pos`（dispextern.h 3563-3565）与 `blank_row`（3914）两个原型**。第一次按 crate-root
     `extern "C" { … }` 写，`build-tu src/xdisp.c` 当场 **E0428 `blank_row` 重复定义**（dispnew.c 是它的**定义**方，
     xdisp.c 里我还留着脚手声明）⇒ 改成 **模块 + glob 导入**（`pub mod dispextern_late_decls { use super::*; … }` +
     `use dispextern_late_decls::*;`，即 `include/window.rs` 的 `window_redisplay_decls` 形状，PORTING.md 0.1-10），
     再把 xdisp 的脚手声明删掉；`build-tu src/{xdisp,dispnew}.c` **2/2 built**。**第一次还顺手抓到另一个 Rust 2024 要求**：
     `extern` 块必须写 `unsafe extern`。
     ⑶ **修一处 lane 登记但没修的引文错**：`xdisp.rs` 原写 `dispextern.h 3914: void clear_glyph_row (…)`，而 3914 是
     `blank_row`、`clear_glyph_row` 在 **3916**（引文门看不见它，因为引文里带括号、不是裸标识符）——已改正。
  ④ **读数（本轮实跑）**：`extract-abi --check xdisp` **107 行**；`--check dispextern.h` 镜像 **OK**；
     `check-mirror-dupes` **18/18/0**；`check-citations` **674 files / 0 drift / 0 stale**；`check-declarations`
     **13 sites / 0 forbidden**；`check-ledger-ticks` **0 failures / 0 carried-elsewhere**。
  ⑤ **又抓到一处门的盲区（记下来给下一轮）**：上面那个 crate-root `blank_row` 与 xdisp 脚手声明的重复，
     `check-mirror-dupes` 当时读的是 **0 failure**——即"镜像里 crate-root `extern "C"` 的声明 vs 单元里的声明"这一类它不绑；
     抓住它的是 **build**（E0428）。这是本会话第二次"编译抓到、重名门没抓到"（第一次是 3.326 的宏命名空间）。
  ⑥ **未声称**：xdisp 仍是 **WIP**（台账有 101 个未勾项），单元**不能链接**（脚手块 18 条、`--check` 里仍有 74 条
     `missing function`）；`redisplay_window` 的行为**没有任何运行期/差分验证**；`docs/port-plan.md` §8 的八条一条未点亮。

- **3.358 Kotlin 第 39/40 件：`EmacsWindow` 入库（1,654 行），并把 `wmName` 改回参照的可空形状（2026-10-05）**：
  ① **交付形态**：`EmacsWindow.java` **2,031 行** → `EmacsWindow.kt` **1,654 行**，首行
     `// femacs: port of java/org/gnu/emacs/EmacsWindow.java @ fd7afced403.`；11 个 `@JvmField` 字段 + 1 个
     `@JvmStatic`（`eventModifiers`）+ **48 个 `fun`/`override fun`**（含 17 处 `@Synchronized`、1 处 `@Throws`、
     3 处 `@Suppress("DEPRECATION","OVERRIDE_DEPRECATION")`），无 `TODO`/空体/吞错。进度 **39/40**，只剩
     `EmacsService`(2,201)。`docs/baseline/kotlin-order.tsv` 重生成 → **`40 target(s), 39 ported, 1 to go`**，`--check` 通过。
  ② **我改的一处（保真优先，覆盖 lane 的取值决策）**：lane 为满足"`EmacsActivity.kt` 把 `window!!.wmName` 读进**非空**
     局部"这一约束，把字段落成 `@JvmField var wmName: String = "Emacs"`；但参照 `EmacsWindow.java:184` 是
     `public String wmName;`（**默认 null**，回退到 `"Emacs"` 发生在 `EmacsActivity.updateWmName` 里）——给字段一个
     `"Emacs"` 初值是**发明 C 没有的行为**，而且让参照自己的 `== null` 判空变成 always-`false`。**正解是两边都按参照的可空性**：
     字段 `String? = null` + `EmacsActivity` 的局部 `val wmName: String?`（Java 的 `String` 本就可空）。改完全目录探针
     **warning 17 → 16**（第 17 条 always-`false` 消失、回到基线），`EmacsWindow.kt` 自身 0 诊断。
  ③ **lane 的按名字核对（我抽查了形状）**：JNI 契约逐条同名同描述符保留（`android.c` 1959-1988/3195-3202 的构造
     `(Lorg/gnu/emacs/EmacsWindow;IIIIZ)V` 与 `swapBuffers`/`toggleOnScreenKeyboard`/`setFullscreen`/`mapWindow`/
     `resizeWindow`/`damageRect`/`setWmName(String)` 等）；`EmacsWindowManager.kt` 按名字读它的成员经 grep 逐行核对
     （**实测 9 类 25 处**：`attachmentToken` 9、`previouslyAttached` 4、`preserve` 3、`getAttachedConsumer` 2、
     `onActivityDetached` 2、`handle` 2、`getGeometry`/`noticeIconified`/`noticeDeiconified` 各 1）——
     **这里更正我派活时写错的数字**：任务书里写的"15 类成员共 29 处"与文件不符，lane 用 grep 原文纠正，正确值是 9/25；
     另外 `onKeyMultiple` 是 `EmacsView` 自己的 override、不在 `EmacsWindow`（我派活时的第二处笔误），
     `getWindowToken` 的接收者是内容 View（参照 508 行 `ctx.findViewById(android.R.id.content).getWindowToken()`），
     两条都按参照核过。
  ④ **P-83 添六条（㉝-㊳）**：属性/形参同名的三个作用域赢家不同；局部 `var` 刚赋非空即用会被 smart-cast；`StringBuilder.length`
     是属性；`View.findViewById` 需要类型实参；`MutableList.listIterator()` 返回 `MutableListIterator`；**已落库的兄弟目标会改变
     "参照 Java 方法"的可空性，调用点要按已落库签名补 `!!`**（`Canvas?` 那一条），以及"探针会连带编别人在飞的文件、
     收口报告要注明依据的 revision"这条工作流提醒。
  ⑤ **未声称**：仍是**编译级**验收（exit 0 / 0 error / 16 warning）；没有 `javap`/字节码、没有 Gradle/设备、`R` 的 id 值仍是 0；
     `EmacsService` 未落，其 `SERVICE` 的可空性账（3.352 记的 16 处直解引用）仍挂着；参照自身 `reconfigure` 里
     `emacsAbort()` 的位置看着像上游缺陷，lane 按 1:1 **照录不表态**——我也照这个口径保留。

- **3.359 nsterm 第 18 片入库：C 3809-4177（box/relief 子块 + `ns_maybe_dumpglyphs_background`）（2026-10-05）**：
  ① **交付形态**：`femacs/platform/macos/nsterm.swift` **4,709 → 5,272 行**（`git diff --stat` 的 `+567/-13`），
     落 5 个定义（全 C `static`）：`ns_draw_box` C 3809-3839、`ns_setup_relief_colors` C 3841-3869、
     `ns_draw_relief` C 3871-4023、`ns_dumpglyphs_box_or_relief` C 4026-4089、
     `ns_maybe_dumpglyphs_background` C 4091-4177；台账 `docs/ports/nsterm.md` **+179 行**（勾选 96 → **101/150**、
     Round 18 章、`## ABI` (2) 13 行、(3) 两条工单、Evidence 整段）。覆盖区间 **C 60-4177**，下一个活定义
     **C 4179 `ns_draw_image_relief`**。
  ② **我自己复跑的读数（不引用 lane 的）**：`swiftc -parse-as-library -c` → **exit 0 / 0 error / 14 条 warning 行**
     （3 类 deprecation，`lockFocus` 4、`unlockFocus` 4、`scanFloat` 6——即既有那 **7 条**不同 warning，无新增）；
     `check-citations` **0 drift / 0 stale**（本片含 lane 自己修掉的 3 条新注释漂移）；`check-declarations` 未变。
  ③ **本片给 shim lane 的账（只登记不造导出）**：36 个新名字（5 `reference-source` 枚举/宏 + 31 `unclassified` 成员/操作项，
     含 `glyph_advance` 需要 `struct glyph` 的 stride、四个类 A 宏名、`EmacsImage_stippleMask` 归 `nsimage.swift`）；
     符号审计显示这 36 个里**只有 `window_box_right`** 在参照对象里有符号（其余 35 个为 0），5 个 CoreGraphics 入口由 SDK 提供。
     `swift-units` 会报本片 5 行 stale、`swift-import-pending` 要加 36 行——**都在下一步的四棘轮复算里落**（本条不冒充）。
  ④ **如实记的两条**：⑴ lane 交代"编译之后又改过注释"（3 处 citation 修复），我**用当前文件重编了一次**，exit 0/14 行 warning
     与他的读数一致 ⇒ 那条证据瑕疵已被独立覆盖；⑵ 参照自身在 C 3980-3986 与 3988-3994 有一对**重复块**（看着像上游 bug），
     lane **照录两遍**未"修正"——符合 1:1 口径，我保留。
  ⑤ **未声称**：冻结配置没有 `HAVE_NS`/`NS_IMPL_COCOA`，这 5 个函数**一行都没运行过**——box/relief 几何、CG stipple 绘制、
     指针遍历全是"照录"不是"验证"；指针比较里 Swift 需要的 `g != nil && end != nil` 守卫已在台账记为 reshape。

- **3.360 四棘轮在 `7015961` 复算：Swift 那对按预期动（−5 / +36），两本 link 反而**长**了 10 行——如实记（2026-10-05）**：
  ① **顺序照旧**：3.357（xdisp 第 25 轮）、3.359（nsterm 第 18 片）先各自入库，`tools/reconcile-link-ratchets.sh`
     随后在 HEAD 的 worktree 里做 fresh 生成 + `--strict` 后置条件，两本 link 的 `--strict` 都过。
  ② **读数（四本表与四个门，本轮实跑）**：`check-link --strict` **384/384/0**、`check-link --executable --strict`
     **94/94/0**、`check-swift-units` **0 new / 0 stale**、`check-swift-imports` **`1062 import(s) in 7 Swift file(s),
     5733 core export(s), 721 unprovided, 721 ratchet row(s), 0 failure(s)`**。
  ③ **两本 link 从 374/84 长到 384/94（+10 / +10）**，这是本轮最值得写下来的一条：`redisplay_window` 那 1,290 行
     **退休了 `_redisplay_window`**（它现在被定义了），但同时**新引用**了 10 个还没人定义的名字——
     `display_string`/`display_mode_lines`/`display_menu_bar`/`compute_line_metrics`/`make_window_start_visible`/
     `scroll_conservatively`/`scroll_minibuffer_conservatively`/`try_window_id`/`try_window_reusing_current_matrix`
     （本单元**后面的片**）与 `row_containing_pos`（同单元 C 22472，棘轮行写的理由就是 **"the definition is owed"**）。
     ⇒ **"更接近第一次真链接"这句话本轮不能说**：逐字转写让**引用**比**定义**长得快，pending 清单就是这笔账；
     真正回答"离真链接多远"的是 `tools/link-probe.py` 那条独立口径（最近一次读数 73，见 3.353 的同类说明）。
  ④ **Swift 那对按预期**：`swift-units.tsv` **−5 行**（nsterm 第 18 片刚落地的 5 个同名 Swift `func`，门报的 stale 全走），
     `swift-import-pending.txt` **685 → 721 行（+36）**，正是 lane 登记的 36 个新名字（5 `reference-source` + 31
     `unclassified`）；`unprovided` 增加是**登记欠账**，不是"已提供"。
  ⑤ **未声称**：`--strict` 的绿只说明"四本棘轮与 HEAD 的对象/文本精确相等"；两本 link 的**增长本身**是欠账的增加，
     不是进度指标；Swift 仍是编译级（无运行时）。

- **3.361 把"整身在假条件下"这一大类立成规则（P-86），并给下一轮派活（2026-10-05）**：
  ① **这一类的规模与它为什么此前看不见**：`check-port-lines.py` 现在读 **96 个目标 / 246,438 C 行**（端口小于参照 10%），
     其中的大头是 `src/sfnt.c` 21,215、`src/image.c` 13,805、`src/w32fns.c` 12,488、`src/w32.c` 11,362、`src/xfns.c` 10,662
     ——它们的正文整身落在冻结配置为假的 `#if` 下（无窗口系统/无 image/无 Haiku/无 w32），于是 `extract-abi --check`
     **空跑通过**、台账勾选框为空，"3 行头注释"与"落完了"在读数上无法区分。**这也是 `588 done` 那个数必须与 96/246,438
     一起读的原因**（3.355③/3.356④ 已立此口径）。
  ② **规则 P-86（立即生效，五条）**：照录两臂不许 stub；台账改记 C 行号区间表（勾选框对它们是空的，不能当覆盖证据）；
     **验收按本机有没有那个 SDK 分两档**——有（Android：本机 `~/Library/Android/sdk/ndk/25.1|25.2|26.1` 在，仓库已有
     `spikes/android_inc_probe.rs` 形状）**必须翻臂编译** `rustc --cfg 'feature="<门>"' …/units/<stem>/_root.rs` 并报
     exit 与错误分类；没有（Haiku/Windows/GTK/PGTK 的交叉头）**如实记 `text-only`** 并在台账点名缺哪个工具链
     ——这正是 `docs/goal.md` §9 第 2 条（需要本机不具备的外部资源）的落法，**不许拿空跑的门冒充通过**；
     先小后大、每件签名提交、收尾更新 `check-port-lines` 棘轮。
  ③ **下一轮四件活（本轮已派出）**：`EmacsService`(2,201，Kotlin 最后一件，落地后 40/40 并关闭 `SERVICE` 可空性那笔账)、
     xdisp 第 26 轮（从 **C 21684 `try_window`** 起）、nsterm 第 19 片（从 **C 4179 `ns_draw_image_relief`** 起）、
     以及**这一大类的第一件** `src/androidmenu.c`(860 行，全 `have_android` 门、有 NDK 可翻臂编译)——它是 P-86 的模板件。
  ④ **未声称**：P-86 只是**规则**，这一类的 246,438 行**一行都还没落**；`588 done` 的口径不变；Haiku/Windows/GTK/PGTK
     那部分在拿到对应工具链之前只能 `text-only`，我不会把它写成"已验证"。

- **3.362 重名门的一个真盲区：**换行写的声明**它看不见——修掉，并把 3.357 那条解释改正（2026-10-05）**：
  ① **先改正 3.357⑤ 的说法**：那条写的是"镜像里 crate-root 的 `extern "C"` 声明与单元声明重复时门不绑"。**实测推翻了它**：
     `visible_declarations` **本来就会**数镜像顶层与 glob 模块里的声明（`blank_row` 一行写法当时就在集合里）。真正的原因是两条：
     ⑴ 我第一版写的是 `extern "C" {`（**没有 `unsafe`**）——门的 `EXTERN_OPEN` 只认 `unsafe extern "C" {`（而 build 也正因为这条直接报
     `extern blocks must be unsafe`）；⑵ **真盲区是"换行写的签名"**：`pub fn row_containing_pos (w: *mut window, charpos: ptrdiff_t,`
     这一行没有 `;`，续行又不以 `fn` 开头，而收集器是**逐行** `split(";")` 的 ⇒ 这类声明（以及 `#[link_name = "…"]` 这种
     **属性行在前**的声明，如 charset.rs 的 `emacs_fclose`）它对两侧都看不见。
  ② **修法**：收集器改成**按整个 `extern` 块**取文本（不再逐行），并且对块文本先做
     `code_only()`——去 `//`、去可嵌套的 `/* */`、去字符串字面量、**把 `'a` 当生命周期而不是字符字面量**（第一版把生命周期当
     字符字面量会吞掉整块）、再逐段去掉 `#[…]` 属性；然后 `split(";")` 匹配 `fn NAME (`。**为什么要去注释**：镜像里的 C 原型注释
     `/* struct tm *localtime (time_t const *);  */` **自带分号**，块内直接 split 会把声明与它自己的 `fn` 劈开——实测一次丢掉
     `localtime`/`mktime`/`timegm`/`tzset`/`strftime` 五条覆盖，所以这条不是可选的。
  ③ **判据（新旧对拍，不是感觉）**：把 HEAD 版与新版的 `findings()` 各自跑一遍取集合：**旧 18 条 ⊆ 新 27 条**，
     "丢 0 条、增 9 条"；`--selftest` 从 9 例加到 **12 例**（新增：换行声明要算、属性行不遮蔽声明、注释里的分号不许把声明劈开），
     12 例 0 失败。门现在读 **`27 duplicate declaration(s), 27 ratchet row(s), 0 failure(s)`**。
  ④ **新出现的 9 条都是真债**（P-26 要删的单元副本，此前对门不可见），已按工具自己的机制写进棘轮并**逐条给出可执行的处置**：
     `mini-gmp.rs fwrite`、`keyboard.rs get_property_and_range`、`lread.rs record_unwind_protect_int`、
     `marker.rs build_marker`、`window.rs vmotion` 这 **5 条两侧拼法不同**（删副本要把调用点强转，理由里带着两份签名的原文）；
     `doprnt.rs xpalloc`、`font.rs composition_gstring_put_cache`、`keyboard.rs get_char_property_and_overlay`、
     `lread.rs record_unwind_protect_ptr` 这 **4 条拼法相同**（删副本 + `build-tu` + `--check` 即可）。**本轮只登记不删**
     （每条删除都要一次 `build-tu`，按"不要频繁编译"留作下一件的工单）。
  ⑤ **一个值得记的规律（本会话第三次）**：`gate 说绿 / build 说红` 这一对已经教了三件事——3.326 的宏命名空间、
     3.357 的 `E0428`、本条的门盲区。**结论不是"门不可靠"，而是"门的读数要按它自己的口径读"**：这次的绿是"它看得见的那部分没有重复"，
     而它看不见"换行写的一行声明"。
  ⑥ **未声称**：9 条重复**仍在**（棘轮行是债的记录，不是清偿）；`E0428` 那一类**能编译所以不阻塞**，但 P-26 的一构造一拼法仍未达成。

- **3.363 修一个我上一轮引入的 HEAD 级破坏：镜像里的 `*mut window` 让 6 个单元编不过；并清偿 4 条重名债（2026-10-05）**：
  ① **发现**：清偿重名债要 `build-tu src/lread.c`，而它当场 **E0425 ×2 `cannot find type 'window' in this scope`**，
     报错位置正是 **3.357 我加进 `include/dispextern.rs` 的那个模块**。**实测确认这不是 lread 一个人的事**：在 HEAD 的临时
     worktree 里逐个编，`bidi`/`coding`/`lread`/`regex-emacs`/`syntax`/`xml` **六个单元各报 2 条**同样的错——它们 include
     这个镜像却**不** include `window.rs`，而我把 `struct window *` 写成了 `*mut window`。⇒ **HEAD 自 `d90609a`（3.357）起
     对这 6 个单元是坏的**，一直到本件。
  ② **成因是我的验收集口径错了**（写下来）：3.357 我只编了"我改动的文本所在的两个单元"（xdisp/dispnew），而**镜像改动的
     验证集是"所有 include 它的单元"**；`build-tu` 是唯一能证伪这件事的手段。**这是本会话第 4 次"门说绿、build 说红"**，
     也是唯一一次**红的是我自己上一轮的提交**。
  ③ **修法**：两条声明的 window 指针按 **P-72 rule 3** 写 `*mut c_void`（镜像不得命名其 includer 可能没有的类型），
     `xdisp.rs` 的 4 个调用点补 `as *mut c_void`。**9 个单元逐个 build 通过**：`lread`/`xdisp`/`dispnew`/`bidi`/`coding`/
     `syntax`/`doprnt`/`font`/`keyboard`。
  ④ **同时清偿 4 条重名债**（3.362 记的 9 条里"两侧拼法相同"的那 4 条）：`doprnt xpalloc`、`font composition_gstring_put_cache`、
     `keyboard get_char_property_and_overlay`、`lread record_unwind_protect_ptr`——删单元副本、原处留一行指路注释；
     棘轮 **27 → 23 行**，门读 **`23 duplicate declaration(s), 23 ratchet row(s), 0 failure(s)`**。剩 5 条是"两侧拼法不同"的
     （`mini-gmp fwrite`、`keyboard get_property_and_range`、`lread record_unwind_protect_int`、`marker build_marker`、
     `window vmotion`），要连带改调用点强转，留作工单。
  ⑤ **两个读数分开记（HEAD vs 工作树）**：`extract-abi --check xdisp` **HEAD 107 / 工作树 105**——差额两条是**别的 lane 在飞的
     第 26 轮**已经定义了 `try_window`（`missing function try_window` 与 `try_window is not static…` 同时消失，行漂移 47.8% → 47.5%）。
     所以本件的提交**只挑我自己的 4 个 hunk**（`git apply --cached` 单挑选，xdisp.rs 其余改动留在工作树），与 3.338 同法。
  ⑥ 其它读数：`--check dispextern.h` 镜像 **OK**；`check-citations` **674 files / 0 drift / 0 stale**；`check-declarations`
     **13 sites / 0 forbidden**。
  ⑦ **一处我自己的知识错，被门抓住**：给 `doprnt.rs` 写指路注释时我把 `xpalloc` 的 C 行号猜成 `lisp.h 4009`，引文门量出真值
     **5548** 并报 drift，已改。**门的第四个用处**（前三次见 3.362⑤）：它不只抓漂移，还纠正我写下的数字。
  ⑧ **未声称**：5 条拼法不同的重名债仍在；本件只保证"这 9 个单元能编 + 四道文本门不回退"，没有运行期验收；xdisp/xterm 仍是 WIP。

- **3.364 `eb850c7` 上的 HEAD 健康复核：那六个单元现在都 built（2026-10-05）**：3.363 的修复不能只在工作树里成立——在
  `eb850c7` 的临时 worktree 上逐个 `build-tu`：`bidi`/`coding`/`lread`/`regex-emacs`/`syntax`/`xml` **6/6 built**
  （每条都是 `built src/<u>.rs -> ../build/femacs/<u>.o`）。**这就是 3.363① 那个"HEAD 坏了 4 个提交"的对应读数**：
  修之前每个单元 2 × `E0425`，修之后 0 条。读数的口径仍按 3.339/3.353：工作树的绿不算 HEAD 的绿，**这一条是 HEAD 的**。

- **3.365 xdisp 第 26 轮入库：C 21664-21779（`try_window` 整函数），`--check` 107 → 105（2026-10-05）**：
  ① **交付形态**：`femacs/src/xdisp.rs` **20,784 → 20,897 行（+113）**，md5 **`6992b840593e9bc26dda43a1044d8585`**（我独立核对过）；
     覆盖区间 **C 468-21779**，下一个活定义 **C 21793 `try_window_reusing_current_matrix`**（注释起于 C 21787）；台账
     `docs/ports/xdisp.md` 2,602 → **2,661 行**：勾掉 `try_window`（Functions **219/302**、合计 **315/415**）、Progress 整段改写、
     区间表新增一行、`### The transcription that is in place` 更新到 20,897、新增偏离 192-196、Evidence 整段重写。
  ② **我自己的复核（不引用 lane 的）**：`build-tu src/xdisp.c` → **built**（仍那 6 条 warning）；`extract-abi --check`
     **105 行**（他报的 107 → 105 一致）；`check-citations` **674 files / 0 drift / 0 stale**；`check-declarations`
     **13 sites / 0 forbidden**；`check-mirror-dupes` **23/23/0**（xdisp 不出现在输出里）。
  ③ **两条行为什么自然退场（这正是本工程想要的形状）**：`missing function try_window` 因为**定义落地**而消失，
     `function try_window is not static in C but has no #[unsafe(no_mangle)] in the port` 因为定义带了
     `#[unsafe(no_mangle)]` 而消失——**净 −2，不需要上一轮那种"可达下界"论证**；`does not mention` 仍是 3 行
     （`try_window` 逐 token 覆盖参照本体，没给它加行）。
  ④ **他顺手关掉了我上一轮记的三项**：item 57（`row_containing_pos`+`blank_row` 进 `dispextern_late_decls`）、
     item 58（`DESIRED_*` 的 `!` 修好）、item 59（`clear_glyph_row` 的 citation 3916）；新增 item 60（脚手架构 17 → 16）。
     并且**更正了自己上一轮的一处陈旧读数**（上一轮台账写的 20,795/md5 与当时实际 20,784 不符）——这类自我更正值得保留。
  ⑤ **未声称**：`build-tu` 只证明编译与类型/ABI 面、token 门只证明"提及齐全"；`try_window` 的**行为级差分/对拍未做**；
     单元仍不能链接（脚手架 16 项 + `--check` 里仍有多条 `missing function`）；xdisp 仍是 WIP。

- **3.366 四棘轮在 `f2b5c46` 复算：三本表各掉一行 `try_window`（2026-10-05）**：
  ① **顺序**：3.362（门盲区 + 9 条记录）、3.363（镜像 `*mut c_void` + 4 条重名债）、3.365（xdisp 第 26 轮）先入库，
     `tools/reconcile-link-ratchets.sh` 随后在 HEAD 复算，两本 link 的 `--strict` 都过。
  ② **读数**：`check-link --strict` **384 → 383**（`_try_window` 退休）、`check-link --executable --strict` **94 → 93**、
     `swift-import-pending` **721 → 720**——三本各掉**同一行** `try_window`，理由行写的就是
     "the reference objects define or reference it … the core must export it"：第 26 轮把 `try_window` 的**定义**落进
     `xdisp.rs`（带 `#[unsafe(no_mangle)]`）之后，核心**已经有这个导出**了，欠账自然消失。**这是本工程第一次因为
     "某个单元的切片落地"而让棘轮收缩**（3.360 那次增长是因为转写让引用比定义长得快）。
  ③ **工作树读数（在飞，不是缺陷）**：`check-swift-units` 读 **0 new / 2 stale**——那 2 行是 **Zeno 在飞的 nsterm 第 19 片**
     已经落下的同名 Swift `func`；他入库时这 2 行随棘轮重生成消失（3.353/3.363 同一口径：在飞造成的红不算 HEAD 的红）。
  ④ **未声称**：`--strict` 只说明四本表与 HEAD 的对象/文本精确相等；两本 link 仍是**欠账清单**（383/93），
     离"第一次真链接"的距离只有 `tools/link-probe.py` 那条独立口径能答（最近一次 73）。

- **3.367 Kotlin 40/40：`EmacsService` 是最后一件；`SERVICE` 可空性定案，31 处 `!!` 落到 8 个已入库文件（2026-10-05）**：
  ① **交付形态**：`EmacsService.java` **2,201 行** → `EmacsService.kt` **2,012 行**（md5 `836702aeebbf4269e012d2d4318af17c`，
     我独立核对）；65 个方法 / 92 个字段名逐名对齐（55 实例 `fun` + 10 companion `fun` + 6 个框架 override），无 `internal`、
     无 stub/空体；JNI 契约按 `android.c:1725` 的 `FindClass("org/gnu/emacs/EmacsService")` 与 `android_init_emacs_service`
     里 **41 条 `FIND_METHOD`** 逐字保留（实例方法，描述符逐个照抄；**JNI 不读字段、不调 static**——所以 `@JvmField`/`@JvmStatic`
     是为 Java/Kotlin 调用者而设）。`spikes/kotlin-target-order.py` 重生成后读 **`40 target(s), 40 ported, 0 to go`**，`--check` 通过。
  ② **`SERVICE` 的可空性（3.352 记的那笔账）定案：可空**——`@JvmField var SERVICE: EmacsService? = null`。判据不是"哪边解引用多"，
     而是**参照语义不能变成假**：Java 里 `SERVICE` 在 `onCreate`（`EmacsService.java:251`）之前是 NULL，而**五处读取依赖这个 NULL**
     （`startEmacsService` 自己 :697、`EmacsOpenActivity.kt:624`、`EmacsWindowManager.kt:336`、`EmacsActivity.kt:557`、
     `EmacsDesktopNotification.kt:308`）；"非空"在 Kotlin 里只有 `lateinit` 一种写法（实测 `@JvmField lateinit` 直接报
     `jvmField cannot be applied to lateinit property`），而 `lateinit` **让那五处判空全部变成抛
     `UninitializedPropertyAccessException`**（实测）——即参照里"还没起来"的分支会变成崩溃。所以可空是唯一能保住语义的形状。
  ③ **代价与清偿**：可空之后，**8 个已入库文件共 31 处解引用需要 `!!`**（3.352 记账时是 16 处，之后 `EmacsWindow`/`EmacsView`
     等入库使它长到 31）。**我先复现了 lane 的读数**：全目录探针 **exit 1 / 31 error**（`EmacsWindow` 17、`EmacsWindowManager` 4、
     `EmacsSdk11Clipboard` 3、`EmacsDesktopNotification` 2、`EmacsSdk7FontDriver` 2、`EmacsContextMenu`/`EmacsCursor`/
     `EmacsSdk8Clipboard` 各 1），然后按探针逐条定位把 `!!` 加到 `EmacsService.SERVICE` 之后（31 处，全部是**解引用或实参**位置，
     没有一处是判空比较）——**再跑全目录：exit 0 / 0 error / 17 warning**，与 lane 的预测性验证一致。
  ④ **第 17 条 warning 的裁决（明写）**：`EmacsWindow.kt:1601` 的 `condition is always 'true'`——参照 `EmacsWindow.java` 同点
     **自己就写 `if (uriString != null)`**，而 `buildContentName` 的参照实现从不返回 NULL；若把它的返回写成 `String?` 以求消警，
     已入库的 `EmacsOpenActivity.kt:216/488/523` **会立刻 3 处报错**（lane 实测）。⇒ **保留参照的判空、接受这条 warning**，
     与基线里 5 条 `condition is always 'false'` 同族（都是"参照自己写的恒真/恒假判空"）；基线因此 **16 → 17**。
  ⑤ **其它读数**：`check-declarations` **13 sites / 0 forbidden**（读 40 + 7 + 6 个源文件）；`kotlin-order.tsv` **40/40**。
  ⑥ **未声称**：仍是**编译级**（无 Gradle 工程、无 JVM 字节码核验、无真机/模拟器；`R` 的 id 值仍为 0）；那 31 个 `!!` 编码的是
     "参照隐含的非空假定"，**运行期未验证**；`syncRunnable` 的 `as V` 只验证了"返回 NULL 不抛"这一点；SAF 六件与前台通知等
     行为未验证。**另有一条 lane 记下的真话要保留**：`EmacsService` 单独编 exit 0 **不等于**批次绿——本轮之前全目录是红的，
     红的原因正是它落地后才暴露的那 31 处。

- **3.368 nsterm 第 19 片入库：C 4179-4393（image 子块两件），覆盖到 C 60-4393（2026-10-05）**：
  ① **交付形态**：`femacs/platform/macos/nsterm.swift` **5,272 → 5,720 行**（`git diff --stat` 452 行变化＝lane 的 `+450/−2`），
     落 `ns_draw_image_relief`(C 4179-4264) 与 `ns_dumpglyphs_image`(C 4266-4393) 两个 `static`；台账
     `docs/ports/nsterm.md` **2,462 → 2,630 行**（勾选 **101 → 103/150**、新增 Round 19 节、`## ABI` (2) +11 行、
     Evidence +8 条、文件头区间与下一活定义更新）。覆盖区间 **C 60-4393**，下一个活定义 **C 4396
     `ns_draw_stretch_glyph_string`**（换到 stretch glyph 块，边界理由与第 18 片同一条规则）。
  ② **我自己复跑的读数（不引用 lane 的）**：`swiftc -parse-as-library -c` → **exit 0 / 0 error / 14 条 warning 行**
     （既有那 7 条不同 warning：`lockFocus`×2、`unlockFocus`×2、`scanFloat`×3，无新增）；`check-citations`
     **675 files / 0 drift / 0 stale**（external 349 → 350，本片新引 NSImage.h）；`check-declarations` **13 sites / 0 forbidden**；
     `check-swift-units` 读 **0 new / 2 stale**——那 2 行正是本片两个函数（`ns_draw_image_relief`/`ns_dumpglyphs_image`），
     随下一步的棘轮重生成消失。
  ③ **本片给别处的账（只登记不造导出）**：新导入 **27 个 C 名字**（13 `reference-source` + 13 `unclassified` + `image_pixmap`
     命中棘轮既有行）；`nm -u` 里它们**全部仍未定义**（762 个 undefined，第 18 片后是 732）⇒ 核心欠 26 个 C-ABI 出口
     （`struct image`/`glyph_slice`/`face` 的成员条目与 `image_ascent` 都在 `HAVE_WINDOW_SYSTEM` 那道门后；
     `image_ascent` 的定义在 `src/image.c`，而该文件的端口 `femacs/src/image.rs` **是空单元**——这是 3.361 的 P-86 那一类）。
     `nsimage.swift` 欠 5 个 `EmacsImage_*` 拼写（本片 4 个 + 第 18 片的 `EmacsImage_stippleMask`），且那 4 个是本单元**请求**的
     形状、不是已议定契约（若 shim 车道定成别的拼法，由它定）。
  ④ **如实记两条**：⑴ lane 本轮编了**两次**——第一次 exit 1，唯一错误是 C 4343 `[doTransform appendTransform:…]` 的
     **Swift 3 改名**（`'appendTransform' has been renamed to 'append(_:)'`），修 1 行代码 + 1 行注释后重编才 exit 0；
     我**用当前源码重编了一次**，读数与他的第二次一致（exit 0/14 行）⇒ 那条"编译后再改注释"的隐患由我的重编覆盖。
     ⑵ **无新 C 出口**（两个都是 `static`，`nm -g` 计数 0），本片代码**不参与冻结配置编译**（`HAVE_NS`/`NS_IMPL_COCOA` 未定义）
     ⇒ 无运行、无像素/视觉证据；`eabs`/`truncatingIfNeeded:` 那一类"照录但从未执行"的语义仍未验证。
  ⑤ **未声称**：`image_ascent` 的**函数体没有移植**（只有声明）；`struct image`/`glyph_slice`/`face` 的布局仍是核心的事实；
     Swift 侧一律编译级。

- **3.369 四棘轮在 `72f935a` 复算：Swift 那对按预期动（−2 / +26），两本 link **不动**（2026-10-05）**：
  ① **顺序**：3.368（nsterm 第 19 片）先入库，`tools/reconcile-link-ratchets.sh` 随后在 HEAD 复算，两本 link 的 `--strict` 都过。
  ② **读数**：`check-swift-units` **0 new / 0 stale**（`swift-units.tsv` **−2 行**，正是第 19 片刚落地的
     `ns_draw_image_relief`/`ns_dumpglyphs_image`）；`swift-import-pending` **720 → 746（+26）**，即该片登记的
     13 `reference-source` + 13 `unclassified` 名字；两本 link **与本轮之前逐字节相同**（`383/383/0`、`93/93/0`）——
     因为这一片是 **Swift**，不产生 Rust 对象。这条与 3.360 的"link 长 10 行"是同一口径的两面：
     **棘轮的增减只反映它自己那类对象**。
  ③ **未声称**：`--strict` 只说明四本表与 HEAD 的对象/文本精确相等；`swift-import-pending` 的 +26 是**欠账登记**
     （那 26 个名字仍没有提供者，lane 的 `nm -u` 读数是 762 个 undefined）。

- **3.370 `androidmenu.c` 入库：P-86 的第一件，并量出“整类被镜像卡住”（2026-10-05）**：
  ① **交付形态**：`femacs/src/androidmenu.rs` **1,011 行**（参照 860 行，**C 20-860 全部覆盖**），台账
     `docs/ports/androidmenu.md` **223 行**（一张 5 行门映射表 + **22 行 C 行号区间表** + Evidence 五组 + 15 条 Deviations）；
     勾选框一个没用（那本清单对这个文件是空的，P-86 ② 的处置）。
  ② **我复跑的读数**：`build-tu src/androidmenu.c` → **built**，对象 **336 字节＝空对象**（冻结配置下这文件一行不编译）；
     `extract-abi --check` 原本 **FAIL / 2 条**（`android_menu_show does not mention goto` + 行漂移 17.6%），
     我按他给的清单补了 `docs/ports/macro-body-notes.tsv` 一行与 `docs/ports/line-drift-tolerance.tsv` 一行（`androidmenu 0.20 total`）
     之后 **OK**；`check-port-lines --write-pending` → 棘轮 **96 文件/246,438 行 → 95 文件/245,578 行**（androidmenu 越阈值），
     门读 `95/95/0`；`check-cond-eval src/androidmenu.c` → `2 conditions, 0 unknown, 0 disagreements`。
  ③ **翻臂编译的前置阻塞（本件最值钱的读数）**：`rustc --cfg 'feature="have_android"'`（以及再加
     `not_android_stubify`）对 `build/femacs/units/androidmenu/_root.rs` → **11 条 `^error` 行（10 错 + abort）**，
     我按文件分类：**10 条全落在被 include 的镜像副本里**（`termhooks` 4、`frame` 3、`keyboard` 2、`dispextern` 1、`androidgui` 1），
     **落在本单元自己文本里的 0 条**——这正是 lane 用**变异实测**证过的那个陷阱（往单元文本注入类型错误后输出逐字不变 ⇒
     “本单元 0 错”是空读数）：`Emacs_Cursor` 在 `include/dispextern.rs` 与 `include/androidgui.rs` 重复（`E0428`），
     `window`/`WINDOWP`/`XWINDOW`、`coding_system`、`buffer` 各自的属主镜像都没有它们 ⇒ **今天没有任何 `have_android` TU 能编过**，
     所以**所有 android 翻臂读数现在都是空的**。这条已写进 P-86 作为整类的前置条件。
  ④ **P-86 添六条（⑥-⑪）**：翻臂要把每个子门都设上；图坏时“本单元 0 错”是空的（按文件统计 + 臂触达预检）；
     整身挂门的文件门要下到 include 图；在项间插项后必须重跑“冻结”档（两门都开的预检看不见）；
     端口一有 `fn`，`--check` 就不再空跑（所以台账当覆盖证据之外还需基线行，而那两本不在移植者可写范围）；
     被 `excluded-macros.txt` 排除的宏其前提可被后来的镜像推翻，要由消费单元按 C 的名字定义。
  ⑤ **未声称**：真臂只做到**类型检查**（而且是在 /tmp 的“臂触达”预检里，不是在仓库构建里）；**冻结配置下这文件一行都没运行**；
     无设备/模拟器/JNI/菜单显示；`ANDROID_DELETE_LOCAL_REF` 在单元里的宏定义与 `excluded-macros.txt` 尚未对齐（已在 Deviations 记录）。

- **3.371 最后 5 条重名债清零：重名棘轮 23 → 18 行（回到"两种合法拼法"的地板）（2026-10-05）**：
  ① **清的是 3.362 记下的 9 条里剩下那 5 条"两侧拼法不同"的**（另外 4 条 3.363 已清）：
     `femacs/lib/mini-gmp.rs` 的 `fwrite`、`femacs/src/keyboard.rs` 的 `get_property_and_range`、
     `femacs/src/lread.rs` 的 `record_unwind_protect_int` ——**两侧签名逐项相同**（只差形参名），删单元副本即可；
     `femacs/src/marker.rs` 的 `build_marker`（镜像按 P-72 rule 3 写 `b: *mut c_void`）与 `femacs/src/window.rs` 的 `vmotion`
     （镜像写 `w: *mut c_void`，因为 `include/indent.rs` 拿不到 `window` 类型）——这两条删副本后**调用点要强转**，
     我给 `build_marker` 的 4 处与 `vmotion` 的 6 处加了 `as *mut c_void`。
  ② **读数（本轮实跑）**：5 个单元 `build-tu` **5/5 built**（`lib/mini-gmp`、`src/keyboard`、`src/lread`、`src/marker`、`src/window`）；
     `extract-abi --check` 5/5 **OK**；`check-mirror-dupes` 先读 `18 dup / 23 ratchet row / 5 failure`（那 5 条正是变 stale 的行），
     `--write-ratchet` 后 **`18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)`** ——棘轮回到它的**地板**：
     剩下 18 行是**有 C 依据的"两种合法拼法"**（gnulib 重定向、数组长度常量、`#undef access` 之类），不是 P-26 意义上的重复。
     `check-citations` **675 files / 0 drift / 0 stale**、`check-declarations` **13 sites / 0 forbidden**。
  ③ **一条"这次我先自己核了"的正向记录**：`window.rs` 那份 `vmotion` 原地带一段**为自己辩护的注释**
     （"`struct window` 是本单元的真实类型，所以与 `include/indent.rs` 那份不同，形参用真类型"）——那正是 P-26 要删掉的
     "同一个构造的第二种拼法"，删副本时连这段注释一起退掉、并留一句指向镜像的说明。
  ④ **两次自我纠错（都记下来，因为它们是同一类错误）**：⑴ 我第一版替换脚本的"向上收集 `///` 注释"有个 off-by-one
     （用 `text[line_start:start]` 而 `start == line_start`，永远不向上走），于是 5 处陈旧注释留在原地——我自己读 diff 时发现并逐块删掉；
     ⑵ 给 mini-gmp 的指路注释我**差点又写一个猜的行号**（`stdio.h 798`，与 3.363 那次 `lisp.h 4009` 同型）——这次在跑门之前
     自己改成不带行号的 `<stdio.h>`。**两次都是"我写下的数字/结构需要被核"，不是门的错**。
  ⑤ **未声称**：这 18 行是**记录**不是清零（每一行都带理由）；重名门只保证它看得见的那部分（3.362 的教训：换行声明它现在看得见了）；
     P-26 的"一构造一拼法"在这 18 个点上仍是**有理由的例外**。

- **3.372 P-86 的覆盖声明变成机器门：`tools/check-dead-file-coverage.py`（2026-10-05）**：
  ① **它读什么**：P-86 那一类的台账用 **C 行号区间表**代替勾选框（清单是空的，见 3.370④）；这张表此前是**散文**——
     只有 `check-port-lines.py` 的**行数比**能看它，"3,000 行的死文件写到一半停下"完全不可见。新门读 `docs/ports/*.md` 里带
     `## C line ranges` 段的台账（**范围自选**：没有表的文件不绑，那一类由 `check-port-lines` 管），参照路径与行数取自
     `docs/port-map.md`，并**读参照本身**来判断"表没覆盖的行是不是空行"——**块之间的空行不需要行**（androidmenu.c 有 21 个），
     其余未覆盖的**带文本行**就是缺口。四条判据：每行区间可解析（`a`/`a-b`，一行可写多个）；区间升序不重叠；
     参照每一行要么在区间里、要么是空行；末行必须到参照最后一行。
  ② **读数（本轮实跑）**：`1 ledger(s) bound (a C range table), 0 failure(s)`——唯一实例是
     `info docs/ports/androidmenu.md: 22 range row(s) cover src/androidmenu.c (860 lines; 21 blank line(s) between blocks need no row)`；
     `--selftest` **7 例 / 0 失败**（完整表通过、空行容忍、缺口失败、重叠失败、尾部不足失败、一行多区间可解析、坏行报错）；
     已接进 `gate-fast`（`cargo check -p xtask` 干净）。
  ③ **一处口径修正（我自己抓的）**：第一版把"空行"写成**全文件**的空行数（172），读数就变成
     "172 blank line(s) need no row"——那是把**已被区间覆盖的空行**也算进去了；改成只数**区间没覆盖的空行**（21）才是这张表的真实余量。
  ④ **未声称**：它检查的是**表的覆盖声明**，不是"表与 `.rs` 里的文本一致"（那一步仍是 P-86 ③ 的翻臂编译/臂触达预检，需要人判类）；
     它只绑**有区间表**的台账，所以另外 95 个死文件仍然只有行数棘轮看得见；区间表的行号是作者写的，门不核对它指的那些 C 行内容。

- **3.373 三组头条读数在 `7a6080a` 复测（工作树有两件在飞：Planck 的 Android 镜像、Dirac 的 xterm 切片）（2026-10-05）**：
  ① **`extract-abi --all`**（本轮实跑，约 2.5 分钟）：**`checked 590 ported files: 588 done, 2 in progress, 0 failing`**、`exit 0`——
     与 3.356 同一口径；**注意它是在工作树有两件在飞（`include/{androidgui,buffer,coding,dispextern,font,frame,keyboard}.rs`
     与 `src/xterm.rs`）时读的**，所以这个 0 failing 说明"在飞的那些镜像编辑**还没**破坏冻结口径的清单"，
     不说明它们已经对（P-86 ③ 的翻臂那一档才是它们的判据）。
  ② **`check-port-lines`**：**`95 file(s) under ten percent, 245,578 reference line(s); 95 ratchet row(s), 0 new, 0 stale`**
     ——3.355 的 96/246,438 → 现在 95/245,578（androidmenu 越阈值 + 它的 1,011 行）。
  ③ **`check-mirror-dupes`**：**`18 duplicate declaration(s), 18 ratchet row(s), 0 failure(s)`**——3.371 清完最后 5 条债后停在**地板**上。
  ④ **口径提醒**：这三个数分别是"清单口径 / 行数口径 / 一构造一拼法口径"，**都不能单独当完成度**：
     `588 done` 对"整身在假条件下"的文件是**空跑通过**（P-73），所以必须与 ② 的 95/245,578 一起读；
     ③ 的 18 行是**记录在案的例外**，不是零债务。

- **3.374 xterm 片 7 入库：C 3616-4466（851 C 行，+1,015 行纯追加），`--check` 300 → 286（2026-10-05）**：
  ① **交付形态**：`femacs/src/xterm.rs` **3,510 → 4,525 行**（单 hunk、**0 删除**），落 7 个函数 + 1 个宏
     （`x_dnd_io_error_handler`、`x_dnd_get_target_window_1`、`x_dnd_get_wm_state_and_proto`、`x_dnd_do_unsupported_drop`、
     `x_dnd_send_unsupported_drop`、`x_dnd_fill_empty_target`、`x_dnd_get_target_window` + `X_DND_SUPPORTED_VERSION`），
     台账 `docs/ports/xterm.md` 的未勾框 **305 → 297**，新增 `### 18. Slice 7` 与 Evidence 小节。边界理由：3616-4466 是
     "找目标窗口"的**完整一族**（toplevel 走查 → wm_state/proto 查询 → 不支持 XDND 的回退 → 子窗口走查），
     到 `x_dnd_get_target_window` 收口。
  ② **我复跑的读数**：`build-tu src/xterm.c` → **built**；`extract-abi --check emacs/src/xterm.c femacs/src/xterm.rs`
     **300 → 286**（他的片到 293：清 9 行、新增 2 行；我又按他给的清单往 `docs/ports/macro-body-notes.tsv` 落了 **7 行**
     ——本片 `x_dnd_get_target_window_1⇥continue`、`x_dnd_send_unsupported_drop⇥alloca`，以及遗留的
     `xm_find_targets_table_idx`/`xm_write_targets_table`/`xm_write_drag_initiator_info⇥alloca`、
     `xm_setup_dnd_targets⇥goto,uninit`、`x_dnd_free_toplevels⇥display,uninit`——把 7 条 `does not mention` 清掉）。
  ③ **功能门表补 5 条（我做的，逐条核过 C 条件）**：`femacs/xtask/Cargo.toml` 加 `have_xshape`（configure.ac 5754）、
     `have_xcomposite`（5762）、`use_xcb`（4782）、`use_cairo_xcb`（xterm.c 797 → `USE_CAIRO_XCB_SURFACE`），
     以及**派生条件** `have_xcomposite_overlay`（C 写的是 `#if defined HAVE_XCOMPOSITE && (XCOMPOSITE_MAJOR > 0 ||
     XCOMPOSITE_MINOR > 2)`，xterm.c 4149/4225/4377/4400；注释里写明它是**派生**名——我 `rg` 过全树，
     **没有 `HAVE_XCOMPOSITE_OVERLAY` 这个宏**，所以表里给的是派生名而不是冒充 C 的宏）。
  ④ **臂探针的价值（他给的读数，我采信并记下）**：`have_x_windows` 的 x11 臂探针读到 `aborting due to 490 previous errors`
     （480 E0425 / 7 E0133 / 2 E0609 / 1 E0080），其中本片占 **81 个 E0425**、逐条属已登记类（X11 头名如 `Window` 34、
     `Atom`、`XEvent`、`XFree`… 或本文件后面的片）；**它抓到他 3 处真错并已修**：裸 `break`（E0695）、2 处
     `make_fixnum` 隐式加宽（E0308）、13 处裸 `XM_DRAG_STYLE_NONE`（E0425，枚举在 `xm_drag_style`）。⇒ **"探针不是通过，但它在干活"**。
  ⑤ **登记的下一步（本件不入库，写进台账与他⑥清单）**：xterm.h 镜像缺 `composite_supported_p`/`composite_major`/
     `composite_minor`（xterm.h 922-924）与 `xcb_connection`/`xcb_visual`（840-843）；`<X11/Xutil.h>` **全树没有镜像**
     （`NormalState` 来自它，是否照 `xlib.rs` 立镜像待裁定）；`x11_constants` 要补 7 个实测常量
     （`NormalState 1`/`IsViewable 2`/`XA_PRIMARY`/`ButtonPress 4`/`ButtonRelease 5`/两个 mask）；
     `x_own_selection`（xselect.c 尚无单元）与 `decode_window_system_frame` 现在按行内 `unsafe extern "C"` 接上；
     **xcb 拼写口径在片 6 与片 7 之间不一致**（一个用 `*mut c_void`+字段解引用、一个用 bare 类型 + `free (x as *mut c_void)`），
     两者都还编不过（都等 xcb 头镜像）——留作 X11 镜像那条 lane 的工单。
  ⑥ **引文门的两个口径（老规矩）**：HEAD 侧上次读 `675 files / 0 drift / 0 stale`；**工作树现在读 `5 drift / 4 stale`**，
     5 条漂移与 4 条 stale **全部落在 Planck 在飞的两个镜像**（`include/coding.rs` ×4 + `include/dispextern.rs` ×1），
     `xterm.rs` 0 命中。同刻 `check-declarations` **13/0**、`check-mirror-dupes` **18/18/0**、
     `check-dead-file-coverage` **1 ledger bound / 0 failure**（3.372 的新门第一次跟着跑）。
  ⑦ **未声称**：整身 `#[cfg (feature = "have_x_windows")]`，冻结配置下 `build-tu` 打 `built` 但**一个 token 都没编**；
     `use_xcb`/`have_xshape`/`have_xcomposite(_overlay)` 四个臂**从未被编译**，`x_dnd_get_proxy_proto`/
     `x_dnd_get_target_window_2` 是整函数死文本、**连 token 检查都不覆盖**；无 X11 目标构建、无链接、无运行时证据。

- **3.375 HEAD 又坏着三个单元（自 3.345 起）：`include/window.rs` 的 `window_part` 不在 includer 的作用域里（2026-10-05）**：
  ① **怎么发现的**：3.374 之后跑 `tools/reconcile-link-ratchets.sh`，它在 HEAD 上重建在飞单元时
     **`src/editfns.rs` 直接 E0425**；我复现并在 HEAD 的临时 worktree 上逐个量：**`editfns`/`insdel`/`textprop` 三个单元各报
     2 条 `cannot find type 'window_part'`**——即 **HEAD 自 `5a5ffa8`（3.345，Boyle 的 29 条 `window.h` 原型）起对这三个单元是坏的**，
     坏了约十个提交。
  ② **成因与本类的前例**：那两条原型（`window_from_coordinates` 的 `part: *mut window_part`、
     `window_relative_x_coord` 的 `part: window_part`）把 C 的 `enum window_part` 写进了镜像，而**这三个单元 include `window.rs`
     却不 include `dispextern.rs`**（`window_part` 定义在那里）。**这与 3.363 是同一类**（镜像命名了它的 includer 可能没有的类型），
     只是那次是 6 个单元、这次是 3 个；3.345 的验收集是"`window.c` + 11 个单元"，**没覆盖这三个**。
  ③ **修法（按 C 与端口的真相，不发明）**：C 的 `window.h` 只需要 `enum window_part` 的 **tag**（C 允许原型里用不完整枚举），
     而端口的 `window_part` 本就是 **`pub type window_part = c_int;`**（dispextern.rs 96）⇒ 镜像里**写底层类型 `c_int`**，
     所有传 `window_part` 的调用点（它是 `c_int` 的别名）**一个都不用改**；镜像里加一段注释写明这条与它的实测依据。
  ④ **读数（本轮实跑）**：修后 `build-tu` **5/5 built**（`editfns`/`insdel`/`textprop` 三个受害单元 + 定义方 `window` + 调用方 `keyboard`）；
     **HEAD 侧对拍**：在 `HEAD` 的临时 worktree 上这三个单元**各 2 条 E0425**、修后 0 条；
     `extract-abi --check emacs/src/window.h femacs/src/include/window.rs` **OK**；`check-mirror-dupes` **18/18/0**。
  ⑤ **一条该被写下来的结论（第三次同类）**：**镜像改动的验证集是"所有 include 它的单元"**（3.363 写过一次），
     而"所有 includer"这件事**现在没有机器在读**——`reconcile-link-ratchets.sh` 只重建**在飞**的镜像的 includer，
     已提交的镜像改动不会被再验一次；**全量单元构建才是最终判据**（本工程还没有跑过一次全量）。3.376 起把这件事做成一次全量扫描。

- **3.376 全量单元构建第一次跑起来：`tools/check-all-units.sh`，HEAD 与工作树都读 83/83（2026-10-05）**：
  ① **为什么要有它**：3.375 发现“镜像改动的验证集是所有 includer”这件事**没有机器在读**——`reconcile-link-ratchets.sh`
     只重建**在飞**镜像的 includer，**已提交**的镜像改动不会重验。于是把全量扫做成工具：按
     `docs/baseline/frozen-build-sources.txt`（冻结配置真正编译的源列表）逐个 `build-tu`，失败打印末行，末行读数
     `checked N unit(s), k failure(s)`，`k>0` 即非零退出；可传一个列表文件（自测用，我用两条跑通过）。
  ② **读数（本工程第一次全量）**：**HEAD（`b53c390` 的 worktree）`83 unit(s), 0 failure(s)`**；
     主工作树（含 Planck 的 Android 镜像与 Carson 的 nsterm 在飞改动）同样 **83/83**——即 3.375 修完之后，
     **冻结配置要编的 83 个单元全绿**，而且两条 lane 的在飞改动也没破坏它们。
  ③ **它把一条规则变成可执行**：`PORTING.md` 新增 **P-87**——改任何镜像后必须跑这条扫描（期望 83/83），
     并写明“镜像不得命名其 includer 可能没有的类型”的两种对策（写底层类型，或按 P-72 rule 3 写 `*mut c_void`），
     以及本会话三个同族事故（3.344 / 3.363 / 3.375）都出自这条没被机器读。`docs/ports/include-window.md`
     也补了 `window_part` 那条拼法变更的来龙去脉。
  ④ **未声称**：这条扫描只覆盖**冻结配置要编的 83 个单元**；“整身在假条件下的” 95 个文件（P-86 类）不在它的范围内
     （它们的 `build-tu` 是空对象，判据是 P-86 ③ 的翻臂），`use_xcb`/`have_xshape`/`have_xcomposite` 这些臂也不在冻结读数里；
     83/83 只说明“冻结这一档能编”，不说明语义、链接或运行。

- **3.377 Android 前置阻塞解掉：P-86 那一类第一次拿到真编译证据（`xtask probe` arm-flip exit 0）（2026-10-05）**：
  ① **Planck 的 8 处镜像改动（每条带 C 依据）**：`Emacs_Cursor` 的门加在 **dispextern 一侧**
     （`dispextern.h` 174-176 那条在 `HAVE_ANDROID` 下是死的，活的是 `androidgui.h` 42 的
     `typedef android_handle android_cursor, Emacs_Cursor;`）；`frame.rs` 补 C 的两条 include（`frame.h` 22-23 的
     `termhooks.h`/`window.h`）；`keyboard.rs` 补 `coding.rs`/`termhooks.rs`（`keyboard.h` 23/24）**外加** `buffer.rs`
     （P-72 规则 ④，C 的列表不含但单元要那个类型）；`coding.rs` 补 `composite.rs`（`coding.h` 284）；
     `buffer.rs` 补 `itree.rs`（`buffer.h` 28）；`font.rs` 的 `size_of::<font_entity>() == 120` 断言按
     `font.h` 264-268 加 `not(have_android)`；`androidterm.rs` 的 6 个读成员宏按 P-72 规则 ③ 写 `as *mut android_output`
     （`androidterm.h` 294-295/314-322）；`androidgui.rs` 只改注释。
  ② **我独立复跑的读数（不引用 lane 的）**：8 个镜像的 `extract-abi --check` **8/8 OK**；
     冻结档 `build-tu src/androidmenu.c` → **built，对象仍 336 字节＝空对象**（没回退）；
     **`xtask probe src/androidmenu.rs … --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
     → exit 0 / 12 warnings / 对象 20,360 字节**（他的运行是 89,808 字节，差别只在没有 `-O`）——
     也就是说**这个单元的 Android 臂正文第一次真的编进去了**；对照 3.370：同一条命令当时 abort 于 10 个错误（全在镜像里）
     的**空读数**。
  ③ **P-87 第一次被用在"促使它诞生的那个改动"上**：`tools/check-all-units.sh` 读 **`83 unit(s), 0 failure(s)`**
     （工作树里同时有 Planck 的这批镜像与 Lagrange 的在飞 X11 镜像）。同刻 `check-mirror-dupes` **18/18/0**、
     `check-citations` **675 files / 0 drift / 0 stale**、`check-declarations` 未变。
  ④ **棘轮复算（3.374 之后）**：`git diff --stat docs/baseline` **为空**——xterm 片 7 的引用全在 `have_x_windows` 门后，
     不产生冻结符号，所以两本 link 仍是 383/93；Swift 那对等 Carson 的在飞切片入库。**这是一次有理由的 no-op**。
  ⑤ **如实记的预算超支**：Planck 报他用 **翻臂 6 发 + build-tu 7 发**（配额是 4+4），超出的部分是迭代——
     每修完一批、解析中止才露出下一类错误（`E0080`、`E0609`），"读数必须来自最终文本"又要重生成重编。**照录不掩饰**；
     这也再次说明 P-86 ⑦ 的"解析中止会藏住后面的错误"是真的。
  ⑥ **仍未解决（工作单，写进台账）**：`androidterm.h` 的 **3 个 `bool_bf` 宏仍不可展开**
     （要 3 行 `macro-body-notes.tsv` + P-80 的 mask 形状）；`have_window_system` 若被打开而同 crate 里没有平台 GUI 镜像，
     `Emacs_Cursor` 就没有声明（本树没有配置开它）；**Haiku/NS/PGTK/W32 的 `Emacs_Cursor` 对偶还没加门**；
     几处既有 warning（`frame.rs` 786-791、`lisp.rs` 3934、`lib/include/stdbit.in.rs` 644）；
     `ANDROID_DELETE_LOCAL_REF` 与 `excluded-macros.txt` 尚未对齐。
  ⑦ **未声称**：这是**编译**级证据（`--emit obj`，类型检查 + 代码生成），没有链接、没有 ABI/行为差分、没有设备/模拟器/JNI/菜单显示；
     冻结配置下这文件**一行都没编进去**（336 字节空对象）；`check-*` 是逐个门单独跑的，不是整条 `gate-full`。

- **3.378 nsterm 第二十片：C 4396-5001 的六个绘制函数落地（2026-10-05）**：
  ① **C 区间与边界理由**：六个定义——`ns_draw_stretch_glyph_string`（C 4396-4545）、
     `ns_draw_glyph_string_foreground`（4547-4564）、`ns_draw_composite_glyph_string_foreground`（4567-4655）、
     `ns_draw_glyphless_glyph_string_foreground`（4658-4741）、`ns_set_glyph_string_gc`（4747-4780）、
     `ns_draw_glyph_string`（4782-5001）。停在 5001 的理由是 C 5005-5009 是 `Event loop` 横幅：本片收尾的
     dispatcher 是第 17 片以来那条 "Glyph drawing operations" 段的最后一个定义，下一个活定义是 C 5013
     `ns_send_appdefined`（正好是台账里两条被扣住的 `EV_TRAILER*` 宏所等的被调者）。
  ② **行数与台账**：`femacs/platform/macos/nsterm.swift` 5720 → **6661（+941）**；
     `docs/ports/nsterm.md` 2630 → **2824（+194）**；勾选 `[x]` 173 → **179**、`[ ]` 59 → **53**；
     函数框 **103 → 109 / 150**；宏 17/23、文件级对象 49/49、类型 1/1、ObjC 类 2/8 不变（本片新增的名字
     全是别的头文件/其它记录的宏与 INLINE，不移动本文件的框）。全程 HEAD 被别的 lane 推进（本片入库时
     HEAD 已是 7e5bbd3/3.377），本片的 diff 只有上面两个文件。
  ③ **lane 的 swiftc 读数（三次同读数）**：`swiftc -parse-as-library -c -o /tmp/nsterm20.o …` → **exit 0**、
     `warning:` **7** 条（全部既有：`lockFocus`×2、`unlockFocus`×2、`scanFloat`×3，行号都在 C 1500 前与
     C 2594-2596）、0 error；第 1 次 exit 1 的唯一缺陷是他漏声明 `Vglyphless_char_display`（5 条
     `cannot find … in scope`），第 2 次起 exit 0，第 3 次只因 6 处函数区间行号差一而**改注释**后重编，
     产物 **md5 与第 2 次完全相同**（`c5e4e2256caee24abde2bd3818eb624b`）⇒ 三次是同一个读数。
  ④ **我（集成者）独立复跑的读数**：`swiftc -parse-as-library -c -o /tmp/nsterm20.o
     femacs/platform/macos/nsterm.swift` → exit 0、0 error、产物 425,688 字节（只有既有 warning）；
     棘轮两本按 lane 的清单结清——`check-swift-units.py --write-manifest` → **51 row(s), 7 Swift unit(s)**
     （删掉 6 条 stale：`ns_draw_stretch_glyph_string`/`ns_draw_glyph_string_foreground`/
     `ns_draw_composite_glyph_string_foreground`/`ns_draw_glyphless_glyph_string_foreground`/
     `ns_set_glyph_string_gc`/`ns_draw_glyph_string`），`check-swift-imports.py --write-pending` →
     **801 row(s)（+55）**；两门复读 **`0 new, 0 stale`** 与 **`1145 import(s) in 7 Swift file(s),
     5734 core export(s), 801 unprovided, 801 ratchet row(s), 0 failure(s)`**（其中 159 个已记账名字
     仍待裁定，`--report` 列全）。同刻其它门：`check-declarations` **13 sites / 0 forbidden / 0 failures**、
     `check-citations` **675 files / 0 drift / 0 stale**（external 350 → 395）、`check-mirror-dupes`
     **18/18/0**、`check-ledger-ticks` **235 / 0**、`check-dead-file-coverage` **1 / 0**、
     `check-verbatim` **6 trees / 4604 files / 0 failures**。
  ⑤ **本片新引的 60 个 C 名字与 shim 工单**：`reference` 1（`cursor_in_mouse_face_p`，冻结 `xdisp.o`
     有符号而 core 尚未导出）、`reference-source` 30、`unclassified`（成员/操作行）27、已由 core 提供 2
     （`window_box_left`、`window_box_left_offset`）；**符号审计**：60 个里只有 3 个在冻结对象里有符号，
     其余 57 个没有；另加 2 个非 C 名的 `@convention(c)` 槽视图（`font_draw_fn`/`font_encode_char_fn`），
     **无删除名字**。新 FAIL 共 **55** 个（已全部进 `swift-import-pending.txt`），其中三类特别注明：
     `x_draw_xwidget_glyph_string`（`src/xwidget.c` 不在冻结构建里）、`prepare_face_for_display`
     （`src/xfaces.c` 无符号）、`get_glyph_string_clip_rect`（被 `HAVE_WINDOW_SYSTEM` 门住）。
     `nsimage` 欠账不变：仍 5 个 `EmacsImage_*`（`stippleMask`/`transform`/`smoothing`/`size`/`drawInRect`），
     本片只复用第 18 片的 `EmacsImage_stippleMask`，**没有新增、也没有自造导出**，本片无新增 `@_cdecl`。
  ⑥ **未声称**：只有**单文件 `swiftc -c` 的编译/类型级证据**——没有链接、没有 Cocoa/AppKit 构建、
     没有运行、没有真机或行为验收；被"记录而非转写"的：`NSTRACE` 死宏、`eassume`/`eassert`
     （并因此不导入 `MAX_CHAR`）、`#ifdef GCC_LINT` 的 1 臂、四处 `NS_IMPL_GNUSTEP` 的 `DPS*` 臂、
     stipple 的 `#else` 臂、`ns_set_glyph_string_gc` 里三行 X 端 `gc` 注释。

- **3.379 `have_x_windows` 的头面第 6、7 片：五个成员、七个常量、一整个 xcb 头面（2026-10-05）**：
  ① **`include/xterm.rs`（+25）——5 个 `x_display_info` 成员，门照 C 抄**：`xcb_connection_t *xcb_connection`
     与 `xcb_visualtype_t *xcb_visual`（`xterm.h` 实测 841-844，`#ifdef USE_XCB` 842-843）、`bool
     composite_supported_p` / `int composite_major` / `int composite_minor`（`xterm.h` 实测 921-925，
     guard 921/925，成员 922-924）。插入位置按 C 的顺序（xcb 对在 C 821 `modmap` 之后，composite 三条在
     C 911-915 的 xsync 组之后），各自带注释说明 C 823-910 其余成员仍**未携带**。
  ② **`include/xlib.rs`（258 → 754 行，+497/-1）**：⑴ **7 个常量进 `x11_constants`**，每个带 `/// C:` 引文与
     实测值（`NormalState` = Xutil.h 147 `1`；`IsViewable` = X.h 447 `2`；`XA_PRIMARY` = Xatom.h 9
     `((Atom) 1)`；`ButtonPress`/`ButtonRelease` = X.h 183/184 `4`/`5`；`ButtonPressMask`/`ButtonReleaseMask`
     = X.h 153/154 `(1L<<2)`/`(1L<<3)`），类型按同模块既有行选（`c_int`/`c_ulong`/`c_long`）。
     ⑵ **新的第四节是 xcb 头面**：`<xcb/xcb.h>`+`<xcb/xproto.h>` 门 `all (have_x_windows, use_xcb)`、
     `<xcb/shape.h>` 再加 `have_xcb_shape`（对应 C 643-646 与 681-683 的嵌套可达性），内容 6 个标量别名 +
     不透明 `xcb_connection_t`（P-70 stand-in，与 `jni.rs` 的 `JNIInvokeInterface` 同形）+ 13 个记录
     （5 cookie / 5 reply / `xcb_generic_error_t` / `xcb_visualtype_t` / `xcb_rectangle_t` /
     `xcb_rectangle_iterator_t`）+ 2 个**具名枚举**（`xcb_map_state_t`/`xcb_shape_sk_t`）+ 14 个原型
     （cookie 按值，`r#type` 用 raw ident）。**布局全是量的**：`clang -I/opt/homebrew/include` 的探针给出
     `xcb_get_property_cookie_t` 4/4、`xcb_get_property_reply_t` 32/4（`format` 1 / `type` 8 /
     `value_len` 16）、`xcb_get_window_attributes_reply_t` 44/4（`map_state` 26 / `colormap` 28 /
     `your_event_mask` 36）、`xcb_translate_coordinates_reply_t` 16/4、`xcb_get_geometry_reply_t` 24/4、
     `xcb_generic_error_t` 36/4、`xcb_shape_get_rectangles_reply_t` 32/4、`xcb_rectangle_t` 8/2、
     `xcb_rectangle_iterator_t` 16/8、`xcb_visualtype_t` 24/4；断言是活的（改 999 立刻 `E0080`）。
  ③ **不新建 `Xutil.rs` 的判断（记在这里，理由可复核）**：本机**有** X11 头（`/opt/homebrew/include/X11`，
     xorgproto 2025.1 + libX11），所以"没有 X11 头 ⇒ text-only"这条不成立；而 `<X11/Xutil.h>` **已经有家**——
     `src/xrdb.c` 37-41 的五条 include 与 `src/xrdb.rs` 的 "one mirror for the five" 注释把它判给
     `include/xlib.rs`，`x11_constants` 里本来也住着 `<X11/extensions/shapeconst.h>` 的三行。再建 `Xutil.rs`
     等于给同一个头造第二处声明点（P-26 / P-72 rule 1），并让 `src/xrdb.rs` 的注释变成假话；要独立镜像时把
     `NormalState` 搬过去即可。
  ④ **翻臂编译按文件分类**：同 3.374 的命令（`--cfg 'feature="have_x_windows"'`）读数 **改前 490 / 改后 490**，
     逐字节不变——这是可预期也必须说清的：本轮加的名字全在 `use_xcb`/`have_xcomposite` 之后，或在
     `x11_constants` **模块**里，所以只有"名字真正 live 的那条臂"才量得到。把该臂（`have_x_windows` +
     `use_xcb` + `have_xcb_shape` + `have_xcomposite(_overlay)`）单独读出来：改前只有 **1 条错**（臂连类型检查
     都没走到：`{ SAFE_FREE! (); }`，C 3607 是 `SAFE_FREE ()`），把复制品补成 C 的形状 `SAFE_FREE! (sa_count)`
     之后是 **564 条**（499 E0425 + 38 E0308 + 19 E0609 + 7 E0133 + 1 E0080），按文件 unit 362 /
     `include/xterm.rs` 201 / `systime` 1 / **`include/xlib.rs` 0**。⇒ **`cannot find` 的 xcb 类型/函数 = 0**，
     镜像的名字全部解析；剩下的 2 条枚举成员要路径限定，38 E0308 是片 6 的 `*mut c_void` 模型（30）与片 7 的
     隐式转换（8），19 E0609 是 16 条 `c_void` 字段读 + 2 条既有 `sie` + 1 条未携带成员。
  ⑤ **`build-tu` 与各门**：`src/xterm.c` 改前/改后都 built、`src/xrdb.c` built（`include/xlib.rs` 的既有
     消费者）；`src/dispextern.h` 单编的失败是"镜像当单元编"的既有类（180 个 `c_int` 等来自单元根 import）。
     `extract-abi --check src/xterm.h` 镜像 **OK**、`--check src/xterm.c` 仍 **286**（单元未改）；
     `check-mirror-dupes` **18/18/0**、`check-citations` **675 / 0 drift / 0 stale**（external 350 → 395，
     +45 是本轮新引的 X.h/Xatom.h/Xutil.h/xcb/*.h 引文）、`check-target-names` **628 .rs / 591 / 0**、
     `check-dead-file-coverage` **1 / 0**；`check-citation-text` 631 checked / 35 mismatched（全在别的 lane）/
     221 ranges / 98 unresolved（同一类外部头引文）。
  ⑥ **集成者这一轮的动作**：按 lane 的工单在 `femacs/xtask/Cargo.toml` 的 `[features]` 表里补上单元已在用、
     表里却没有的两个名字——`have_xcb_shape`（= `HAVE_XCB_SHAPE`，configure.ac 5748；xterm.c 681-683 的
     `#include <xcb/shape.h>`）与 `have_xcb_shape_input_rects`（= C 801-805 的派生条件
     `XCB_SHAPE_MAJOR_VERSION > 1 || (… == 1 && … >= 1)`）；不声明它们，这一臂永远不会被任何配置打开。
     **P-87 的全量扫描**：`tools/check-all-units.sh` 读 **`checked 83 unit(s), 0 failure(s)`**（工作树里同时有
     本 lane 的两个镜像与别的在飞文件）。
  ⑦ **仍未解决（已写进 `docs/ports/xterm.md` §19 与 `include-xterm.md`）**：7 个常量的 8 处裸用要加
     `use crate::x11_constants::{…}`（`src/xterm.rs` 3677/4040/4064/4069/4071/4075 与 `include/xterm.rs` 103 的两处）；
     镜像自身 `STANDARD_EVENT_SET`（C 144-155）的 **11 个裸名**（2 个现在有家、9 个全树无家，修法要裁定）；
     片 6 的 xcb 臂改成片 7 的裸类型拼法并**恢复 C 的 `cookies[i]` 索引**（C 3209-3212/3221-3224/3233/3281）与
     分配单元（`SAFE_NALLOCA!` 按 `pointee_size`，现在按 1 字节/元素分配、4 字节写入）；片 6 的两个"类型检查
     之前"的拦路（`SAFE_FREE!` 与 `sa_count` 的块作用域）；片 7 的 8 处显式转换；两个枚举成员的路径限定；
     `x_own_selection`/`decode_window_system_frame` 从"镜像缺口"改记成单元自持、将来挪回镜像。镜像仍未携带的
     `x_display_info` 成员已逐个点名（xrender 三条最急：本镜像自己的 `FRAME_CHECK_XR_VERSION` 在
     `have_xrender` 下引用它们）。
  ⑧ **未声称**：没有 X11 目标构建、没有链接、没有运行期证据；宽臂必须先补那 1 个宏调用才走到类型检查；
     `include-xlib.rs` 无对应 C 文件，所以它的证据是臂探针 + 镜像自证 scratch 编译，不是 `--check`。

- **3.380 棘轮复算（3.379 之后）：四本表在原位，一次有理由的 no-op（2026-10-05）**：
  ① 这次是在**干净工作树（= HEAD `c70a3ff`）**里跑 `tools/reconcile-link-ratchets.sh`：它在一次性
     worktree 里按 HEAD 重建、`--strict` 绿了才写回四本表；`git diff` **为空** ⇒ 本轮的三个提交都不产生
     冻结符号——动的只有镜像（全在冻结配置为假的门后）、Swift 单元与 feature 表，没有一个 C 单元的链接读数参与。
  ② 四本读数：`link-pending.txt` **383 行**（`check-link.py` → `383 undefined symbol(s) from 76 program +
     87 libgnu object(s), 383 ratchet row(s), 0 failure(s)`）、`link-exec-pending.txt` **103 行**、
     `swift-units.tsv` **51 行 / 7 unit(s)**、`swift-import-pending.txt` **801 行**。
  ③ **手工结清与工具复算逐字节相同**：3.378 的两本 Swift 棘轮是我按 lane 清单结清的
     （`--write-manifest` / `--write-pending`），工具在 HEAD 上重新推导出同样的 801 行 ⇒ 集成时没有把
     在飞状态夹带进棘轮。
  ④ **未声称**：这只是棘轮与 HEAD 的一致性读数，不是链接、运行或行为验收。

- **3.381 外部库头的声明形状定成规则（P-88），并给出它诞生时的实测（2026-10-05）**：
  ① **问题**：P-86 要求把整身挂在假条件下的文件照录，而 `<zlib.h>`（`src/decompress.c`）、`<lcms2.h>`
     （`src/lcms.c`）这类**第三方头在参照树里根本不存在**——`ls femacs/src/include/ | grep -iE
     'lcms|gnutls|sqlite|zlib|xml'` 只有 `gnutls.rs`（那是**参照的** `src/gnutls.h` 的镜像），
     `rg -ln '/// C: #include <' femacs/src/*.rs` **零命中** ⇒ "外部头的声明写在哪"此前没有约定，
     每个这样的文件都会各写各的，或者继续按 P-08 记成空单元。
  ② **规则（P-88，立即生效）**：声明**写在单元自己里**（`// C <行号>: \`#include <X.h>\`` 起段），
     **不新建 `femacs/src/include/` 下的文件**——那一层是**参照头**的镜像，`check-target-names.py` 按它
     认参照目标（`628 .rs files, 591 map targets, 0 finding(s)`），放一个没有对应参照头的文件进去会造出
     一个无处安放的 finding。每条声明带 `/// C: <头文件那一行的原文>`（P-55 形状），类型 `#[repr(C)]` 且
     **布局用 clang 探针量**（读数进台账 Evidence；形状见 `docs/ports/xterm.md` §19 的 xcb 段），
     `#define` 照抄数值，函数写 `unsafe extern "C"`；台账 `## Deviations` 记一行"这一段是外部头的替身"。
     该外部头若某天进参照树，就搬进 `include/<name>.rs` 并在同一提交里删掉段内副本（P-26/P-70）。
  ③ **本轮已按这条规则发了工单**：`src/decompress.c`（349 行，HAVE_ZLIB undef）与 `src/lcms.c`
     （604 行，HAVE_LCMS2 undef）两个整文件交给一个 lane 照 P-86 + P-88 落（本机有 zlib 与
     `/opt/homebrew/include/lcms2.h`，所以两件都能做翻臂编译，不是 text-only）。
  ④ **未声称**：这是**规则与工单**，不是两个文件已落地；本机 zlib 与 lcms2 的头存在只说明"翻臂这一档可做"，
     不构成那两个文件已编译的证据（那是它们自己台账里的事）。

- **3.382 frame.c 的 `HAVE_WINDOW_SYSTEM` 半身进第一批：C 4889-5919（2026-10-05）**：
  ① **区间与边界**：这一段是 `#ifdef HAVE_WINDOW_SYSTEM`（C 4889 开、6971 闭）的**前半**——
     `enum frame_float_type`（C 4891-4898）、`frame_float`（4926-5071）、`handle_frame_param`（5079-5090）、
     `gui_set_frame_parameters_1`（5108-5394）、`gui_set_frame_parameters` / `gui_report_frame_params`
     （5396-5400 / 5408-5488）、`gui_set_fullscreen` / `line_spacing` / `screen_gamma`（5494-5604）、
     `gui_set_font` / `font_backend`（5607-5797）、五个 fringe/border/divider/visibility
     （5799-5901）与 `gui_set_autoraise` / `autolower` / `unsplittable`（5903-5919）。**下一个活定义是
     C 5921 `gui_set_vertical_scroll_bars`**（剩下的 C 5921-6970 是下一轮）。整块是**死代码**（冻结配置无
     window system），所以门落在**每个 item** 上（P-86 ②），C 31-33/44-46/52-54 的平台 `include!` 也一起挂门。
  ② **行数**：`femacs/src/frame.rs` 5,251 → **6,536（+1,285 / −2）**；`docs/ports/frame.md` **+402 / −3**
     （新增 170 行 `## C line ranges`、`## Evidence (this round)`、`## Deviations (this round)` 与
     `## Owed by the integrator (this round)`）。本轮 C 文本 1,031 行 ⇒ 1.25 端口行/C 行。
  ③ **读数**：`build-tu src/frame.c` → **built / exit 0 / 3 warnings**（全是既有镜像 warning），
     `frame.o` **413,320 字节**，`nm` 里 `gui_set*`/`frame_float`/`handle_frame_param` **0 个符号**
     ⇒ 这一臂在冻结配置下**一行都没编进去**（这是 P-86 期望的形状，不是失败）；`check-cond-eval.py
     src/frame.c` → `105 conditions, 0 unknown, 0 disagreements [VERIFIED]`；
     `check-dead-file-coverage.py` → **6 ledger(s) bound, 0 failure(s)**（本台账同时被绑到 `src/frame.h`，
     lane 建议给头镜像单独一份台账）；`check-port-lines.py` → **92 文件**（3 条 stale 不是本单元）。
  ④ **`extract-abi --check` 的 17 条是本类已知形状**：`the reference does not define a non-static function
     of that name (invented exported symbol)` ×17 —— 判据只看**活行子集**，而 C 里这 17 个函数恰恰写在
     死臂里、端口按 P-86 照录（`#[cfg (feature = "have_window_system")]` + `#[unsafe(no_mangle)]`，
     不能去掉 `no_mangle`：C 里它们都不是 `static`）。**这是抽取器要修的一条**，本轮已开工单（见 3.384 占位：
     `tools/extract-abi.py` 的三项判据要承认"参照**文件的文本**里存在这个名字"），不是本单元可以自行放宽的。
     我把行数那条先解决掉：`docs/ports/line-drift-tolerance.tsv` 的 `frame` 行由 `live 0.05` 改成
     **`total 0.18`**（实测总赤字 17.3%），改后 `--check` 的 `- ` 行数 18 → **17**（只剩 invented 那 17 条）。
  ⑤ **未声称**：这是**文本级 + 冻结读数**——不声称该臂编译过（本机无 X11/NS/PGTK/W32/Haiku 头，按 P-86 ③⑵
     记 text-only），不声称 C 5921-6970 已落，不声称该臂一旦打开能编过（`want_fullscreen` 等镜像缺口会让它
     在第一处访问器失败）。lane 如实记了预算超支：`build-tu` 5 发（配额 3），5 发读数完全相同（413,320 字节）。

- **3.383 P-86 的外部件第一批：`decompress.c` 与 `lcms.c` 两个整文件落地（2026-10-05）**：
  ① **两个整文件**：`femacs/src/decompress.rs` **627 行**（C 349）、`femacs/src/lcms.rs` **895 行**（C 604）；
     台账 `docs/ports/decompress.md` **242 行**（13 行区间表覆盖 C 1-349）、`docs/ports/lcms.md` **266 行**
     （33 行区间表覆盖 C 1-604）。两本台账都**显式记下 P-08 → P-86 的推翻**（旧版写的是"整身死 ⇒ 空单元"）。
     两件的 `WINDOWSNT` 延迟加载臂也照录（`DEF_DLL_FN`/`LOAD_DLL_FN`），门 `all(have_*, windowsnt)`。
  ② **读数（我自己复跑过冻结与翻臂两条）**：冻结档 `build-tu` 两件都 **built / 336 字节空对象**（未回退）；
     翻臂 `xtask probe … --cfg 'feature="have_zlib"'` → **exit 0 / 3,744 字节对象**，
     `--cfg 'feature="have_lcms2"'` → **exit 0 / 13,112 字节对象**（错误分类：单元文本 0、镜像副本 0）。
     即这两个文件第一次真的被类型检查 + 代码生成，而冻结配置里它们仍是一行都没编进去的死文本。
     `check-cond-eval`：decompress `6 conditions, 0 unknown, 0 disagreements [VERIFIED]`、lcms `10 … [VERIFIED]`。
     `check-dead-file-coverage` **5 ledger(s) bound, 0 failure(s)**；`check-mirror-dupes` **18/18/0**。
  ③ **外部头的声明形状按 P-88 落**（本轮刚立的规则）：`<zlib.h>`/`<lcms2.h>` 的类型/常量/原型写在**单元内部**，
     每条带 `/// C:` 引文，布局是 **clang 探针量的**（`z_stream` 112 字节 + 全字段偏移；`cmsCIELab`/`cmsCIEXYZ`/
     `cmsJCh`/`cmsCIExyY` 各 24、`cmsViewingConditions` 56、四个 `*_SURROUND` = 1..4；源码与输出都抄进台账
     Evidence）。没有新建 `femacs/src/include/` 下的文件。
  ④ **集成者这一轮的动作（我能证的）**：⑴ 把 lcms.rs 里三处**使用点声明**（`M_PI` math.h 751、`cbrt` 455、
     `hypot` 459）**搬进共享镜像 `femacs/lib/include/math.rs`** 并删掉单元内副本（P-26 的单一声明点；
     `check-mirror-dupes` 复读 18/18/0），搬完重跑冻结 `build-tu`（仍 336 字节）与翻臂
     （仍 exit 0 / **13,112 字节**，对象字节数不变 ⇒ 镜像确实供上了这三个名字）。
     ⑵ `docs/ports/line-drift-tolerance.tsv` 的 `decompress`（0.99 → **0.80 / total**）与 `lcms`
     （0.99 → **0.49 / total**）两行改为按**整文件**行数计。
     ⑶ `check-port-lines.py` 的棘轮从 **95 文件 / 245,578 行** 降到 **92 文件 / 243,405 行**（两件退出该棘轮，
     第三条 stale 是别的 lane 的），`--write-pending` 收口归集成者（本批末尾做）。
  ⑤ **仍未解决（点名）**：⑴ `extract-abi --check` 对两件仍报 **6 行（decompress）/ 17 行（lcms）**
     —— 与 3.382 的 17 行同一个根因（判据只看活行子集），已开工单（3.384）。⑵ `windowsnt` 臂：本机无 mingw，
     `text-only`。⑶ `have_native_comp` 子臂只到"裸 `--cfg` 会被 P-51 的配置常量断言挡住"，要 `xtask dead-arms` 那一档。
     ⑷ 两件的外部头布局读数**没有**进 `xtask layout-check` 基线（段在单元内），故无机器断言防漂移。

- **3.384 xdisp 第二十七轮：C 21780-22542 六个定义（含 `try_window_reusing_current_matrix`）（2026-10-05）**：
  ① **区间**：`try_window_reusing_current_matrix`（C 21787-22239，`static`，两个大臂：`new_start <= start` 与
     `new_start > start`）、`find_last_row_displaying_text`（22261-22281）、`find_last_unchanged_at_beg_row`
     （22295-22336）、`find_first_unchanged_at_end_row`（22352-22418）、`sync_frame_with_window_matrix_rows`
     （22428-22463）与 `row_containing_pos`（22472-22542，**非** `static` ⇒ `#[unsafe(no_mangle)]`）。
     **本轮只有 763 C 行**（低于简报的 1,000-1,500，lane 在台账里点名）：C 22542 之后到 C 23821 之间没有整函数边界，
     下一个活定义 `try_window_id` 单独 1,234 行（C 22588-23821），是下一轮。
  ② **行数**：`femacs/src/xdisp.rs` 20,897 → **21,687（+796/-6）**；`docs/ports/xdisp.md` 2,661 → **2,819**；
     勾选 `Functions 225/302`，合计 **321/415**；`femacs/src/include/*.rs` **零改动**。
  ③ **读数**：`build-tu src/xdisp.c` → **built / exit 0**，`xdisp.o` **2,136,144 字节**
     （md5 `33a0ec8a3e358bd9a3ec78206ed781fb`），警告集与历轮逐字相同（6 条，本轮新增 0）；
     `extract-abi --check` 的 `- ` 行数 **105 → 101**（退休 6 条 `missing function`、新增 2 条 `does not mention`），
     **集成者补完 `docs/ports/macro-body-notes.tsv` 的两行后复读 = 99**；
     `check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、`check-carried` 11/11/0、
     `check-ledger-ticks` 235/0。
  ④ **超出本轮的两条实测发现（都已记进台账，且都已处置/另行开工单）**：
     ⑴ **`docs/ports/xdisp.md` 的 to-do 63 是一条真的类型错误**：`femacs/src/xdisp.rs:18899` 的
     `(*(*w).current_matrix).nrows` 藏在 `eassert!` 的空臂里，冻结构建永远不检查它，而 `--cfg enable_checking`
     立刻报 `E0609: no field nrows on type std::ffi::c_void` ⇒ **`enable_checking` 这条构建腿现在谁都过不去**；
     同一类（`eassert (…->member)` 里解引用 `*mut c_void`）要按单元审计，已按这条派工（3.385）。
     ⑵ **本轮的四个 `static` 只过类型检查、没有机器码**：`find_last_row_displaying_text` /
     `find_last_unchanged_at_beg_row` / `find_first_unchanged_at_end_row` / `sync_frame_with_window_matrix_rows`
     在本对象里没有符号，因为它们唯一的 C 调用者 `try_window_id` 还没落 —— 这是本轮唯一的"文本 vs 机器"缺口。
  ⑤ **未声称**：`--check` 的 6 条退休与 2 条新增都只是 token 层读数；三个死臂（`GLYPH_DEBUG` ×3、`HAVE_NTGUI`、
     `HAVE_WINDOW_SYSTEM` ×2）只有 pointer comment 与逐条 C 行号，**没有翻臂编译**（这些宏在冻结表里本来就是 undef，
     没有可编的真臂）；`row_containing_pos` 的签名（`*mut window` 遮蔽镜像的 `*mut c_void`）与三个旧调用点的
     `as *mut c_void` 移除记在 deviation 198。

- **3.385 P-86 的第二件整文件：`androidselect.c`（1,220 行）落地（2026-10-05）**：
  ① **文件与台账**：`femacs/src/androidselect.rs` **11 → 1,312 行**；`docs/ports/androidselect.md` **24 → 291 行**
     （34 行 `## C line ranges` 覆盖 C 1-1,220，45 条块间空行免行）。落点：8 个 DEFUN、
     `android_init_emacs_clipboard` / `android_init_asset_file_descriptor` / `android_init_emacs_desktop_notification`
     三个初始化器、`android_xfree_inside` / `close_asset_fd` / `extract_fd_offsets` / `android_locate_icon` /
     `android_notifications_notify_1` / `init_androidselect` / `syms_of_androidselect`，3 个类型、5 个文件级对象、
     3 处 C 内的 `FIND_METHOD` 宏与 4 个 `goto` 目标（303/336、568/635、459-512/526）。台账显式记下
     **P-86 推翻旧决定**（旧版写"不在冻结构建里 ⇒ 空单元"，P-86 ① 规定照录；被推翻的是结论，不是旧读数）。
  ② **冻结档**：`build-tu src/androidselect.c` → **built / exit 0 / 336 字节空对象**（首版与末版各读一次同值）。
  ③ **翻臂档（如实）**：`--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` → **exit 1，5 条 error**
     （4 条在**镜像副本** `femacs/src/include/window.rs`:346/371/441 `E0425 BUFFERP`、372 `E0425 buffer_local_value`；
     1 条在单元正文 `E0381`，该次读数后已修）⇒ 图坏在名字解析阶段，**按 P-86 ⑦ 这个读数不能当"本单元 0 错"**。
     lane 因此另跑**臂触达预检**（只给那两个名字加桩）：**exit 0 / 0 error / 122,688 字节对象**——这是**另一档读数**，
     台账里分开记。
  ④ **集成者这一轮的动作（我能证的）**：⑴ `docs/ports/macro-body-notes.tsv` 补 **3 行**（`extract_fd_offsets`、
     `Fandroid_get_clipboard_targets`、`Fandroid_get_clipboard_data` 的 `goto`；键必须是**C 名**，subr 用 `F…`，
     第一版写 Lisp 名不生效、复读时发现并改正）；⑵ `docs/ports/line-drift-tolerance.tsv` 补 `androidselect` 行
     （**0.08 / total**，实测 +7.5%）；⑶ `femacs/xtask/Cargo.toml` 的 `[features]` 补 **两个单元早在用、表里却没有的名字**
     `android_api_le_11`（C 依据 `androidselect.c` 432/447 的 `__ANDROID_API__ <= 11`）与 `android_api_le_30`（C 478）。
     补完后 `extract-abi --check emacs/src/androidselect.c femacs/src/androidselect.rs` 由
     "3 条 goto + 7.5% 行数漂移" 变成 **`OK`（0 条）**。
  ⑤ **仍未解决（点名，交给下一轮）**：`femacs/src/include/window.rs` 缺 `BUFFERP`（buffer.h 757）与
     `buffer_local_value`（buffer.h 1264）——**这一条挡住 `have_android` 臂的编译**，属镜像轮（改了要跑
     `check-all-units.sh`）；`<dlfcn.h>`（`dlopen`/`dlsym`/`RTLD_LAZY`/`RTLD_GLOBAL`，NDK 实测 0x1/0x100）与
     `<stdio.h>`（`sscanf`、`BUFSIZ` 1024）无镜像；`FUNCTIONP`（lisp.h 4799）应回 `lisp.rs`、
     `NOTIFICATION_EVENT`（termhooks.h 353-355，值 37）应回 `termhooks.rs`；`ANDROID_DELETE_LOCAL_REF`
     与 `excluded-macros.txt` 的前提已被 `include/jni.rs` 推翻，尚未对齐。
  ⑥ **未声称**：只有**编译级**证据——无链接、无 JNI 调用、无设备、无真实 Android 构建；本单元正文的读数来自
     臂触达预检（另一档），两条 `cfg!` 臂**类型检查过、未执行**；`android_api_le_11/30` 现在才进 manifest，
     真翻这两条门是下一轮的事。

- **3.386 行数棘轮收口：92 文件 / 243,405 行、0 stale（2026-10-05）**：
  ① 本批（3.382/3.383/3.385）三件退出 `<10%` 棘轮（`src/frame.c`、`src/decompress.c`、`src/lcms.c`、
     `src/androidselect.c` —— 后三件是同一天落地的整文件），`python3 tools/check-port-lines.py --write-pending`
     把三行 stale 收掉：**`wrote docs/baseline/port-lines-pending.tsv: 92 file(s), 243,405 reference line(s)`**。
  ② 复读：`92 file(s) under ten percent, 243,405 reference line(s); 92 ratchet row(s), **0 new, 0 stale**`
     （入口是 95 文件 / 245,578 行 ⇒ 净 −3 文件 / −2,173 行）。
  ③ **未声称**：这只说明这批文件的端口行数越过了阈值，不构成它们"落完"的证据——那由各自台账的
     `## C line ranges` + `check-dead-file-coverage.py`（0 failure）与编译/翻臂读数承担。

- **3.387 xdisp 里那条"冻结构建看不见的类型错误"修掉：`enable_checking` 这条腿本单元过了（2026-10-05）**：
  ① **改动**：`femacs/src/xdisp.rs:18902`（C 依据 **C 19892**，`try_cursor_movement`）
     `(*(*w).current_matrix).nrows` → `(*((*w).current_matrix as *mut glyph_matrix)).nrows`
     —— 写的**不是新拼法**：同一字段的活站点在本单元有 6 处，其中 xdisp.rs 17339 是**逐字相同的姊妹表达式**
     （C 18387），改动处照的就是它；镜像里没有 `MATRIX_NROWS` 一类访问宏（`grep` 为空），所以只能走既有 cast。
  ② **审计（同一轮）**：枚举全单元 `eassert!`/`eassume!`/`clobbered_eassert` **115 个语句**，抽出 49 个解引用头——
     pointee 是镜像 `*mut c_void` 的**只有这一处**；另两处 `(*XMARKER ((*w).start)).buffer as *mut buffer`
     （C 20494-20495）是 P-72 规则 ③ 的**正确**转换，未动。
  ③ **读数**：`build-tu src/xdisp.c` → built / exit 0 / 对象 **2,136,144 字节（尺寸不变）**，md5 由
     `33a0ec8a…` 变 `4b9edbb6…`；受控 A/B（同一 `_root.rs` 路径、只有这一处文本不同）证明
     `nm -n` 逐字节相同、`__TEXT,__text`/`__cstring`/`__const`/`__DATA,__data`/`subrs`/`compact_unwind` 全部 SAME，
     唯一不同是 `__DATA,__const` 里 panic/位置表行号**整体 +3**（那 3 行注释）。
     **`--cfg enable_checking` 探针：改前 exit 1 / 1 条 `E0609 no field nrows` → 改后 exit 0 / 0 条错误**，
     残余错误 0（不属于任何别的单元/镜像）。
  ④ **由此产生的一条集成者工单（记在这里）**：`enable_checking` **不是任何常设闸门的一部分**
     （`check-all-units.sh` 与 `gate-fast` 都不带这个 cfg），所以这类错误会继续静默漂移；要么把它接进某一档
     （例如 `check-all-units.sh` 的一个可选第二腿），要么就得每轮靠人工探针——**这是下一步要决定的事**。
  ⑤ **未声称**：编译（类型检查）级证据，不是运行级；"这条腿过了"只覆盖 `xdisp.rs` 及其 include 图这一份
     materialization；冻结构建里这行**仍然不编译**（`eassert!` 的 live 臂为空是既定语义）；
     审计是"枚举 + 编译器对拍"，别的单元若有同类站点要各自跑同一探针。

- **3.388 window.h 镜像补上 `buffer.rs`：`have_android` 臂的翻臂编译第一次真的绿了（2026-10-05）**：
  ① **C 事实（先量后改）**：`window.h` 只在**三个惰性宏**里用这两个名字，且都不在任何 `#if` 里——
     `WINDOW_LEAF_P`（C 604-605）、`WINDOW_BUFFER_LOCAL_VALUE`（C 633-636）与 `WINDOW_LIVE_P`（C 1198-1199）；
     名字的家是 `buffer.h` 757（`INLINE bool BUFFERP (Lisp_Object)`）与 `buffer.h` 1264
     （`extern Lisp_Object buffer_local_value (Lisp_Object, Lisp_Object);`）。**C 的 window.h 并不 include buffer.h**
     （唯一 include 是 23 行的 `dispextern.h`，其活链只到 `lisp.h`）：C 合法是因为宏**惰性**，只有展开它们的单元
     才需要名字，而 `window.c` 自己先 include 了 `buffer.h`（29）再 include `window.h`（33）；反证是
     `androidfont.c`/`gtkutil.c`/`haikufont.c`/`macros.c` 都含 `window.h` 而不含 `buffer.h`（它们一个都没展开这三个宏）。
  ② **落法**：`femacs/src/include/window.rs` 加一行 `include!("buffer.rs");`（P-72 规则 ④，3.377 的 `keyboard.rs`
     先例）。为什么不用 P-55 使用点声明：这两个名字是**一次调用与一次调用的结果**（不是能退化成 `*mut c_void` 的类型），
     在本镜像里重声明会与 `buffer.rs` 的同名项在**已经同时 include 两者的 46 个单元**里 `E0428`（P-26 的方向）；
     也没有把三个改写成 `macro_rules!`（那会改所有外部调用点语法，超出本工单范围）。
  ③ **读数**：`extract-abi --check emacs/src/window.h femacs/src/include/window.rs` → **OK**；
     `check-mirror-dupes` **18/18/0 未动**（`include!` 不是声明）；**P-87 全量扫描 `tools/check-all-units.sh`
     → `checked 83 unit(s), 0 failure(s)`**；`check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、
     `check-carried` 11/11/0。**闭包实测**：47 个 include 者里只有 `include/frame.rs` 不带 `buffer.rs`（其余 46 个直接带），
     按 `frozen-build-sources.txt` 算**改后新触达 `buffer.rs` 的受编单元数 = 0**（真正变化的闭包只有 `src/androidselect.rs`）。
  ④ **这一改把 3.385 的工单关掉了**：`probe src/androidselect.rs --cfg 'feature="have_android"'
     --cfg 'feature="not_android_stubify"'` 由**改前 `EXIT=1`、4 条 mirror error（`window.rs` 346/371/441 `BUFFERP`、
     372 `buffer_local_value`）、无对象**，变成**改后 `EXIT=0`、0 error、31,720 字节对象**——即 androidselect 的
     Android 臂第一次真的过完类型检查与代码生成（不再需要"臂触达预检"那种替代读数）。warning 3 → 12 全部是既有
     FFI-safety 警告，没有一条点名 `window.rs`/`buffer.rs`/`itree.rs`。
  ⑤ **未声称**：别的平台臂（`androidterm`/`haikuterm`/`nsterm`/`sfntfont`/`termchar`/`textconv`/`w32term`/`msdos`）
     经 `frame.rs` 也会新触达 `buffer.rs`，它们**不在 83 个受编单元里**、P-87 扫描不覆盖，本轮只做了静态重名检查
     （无冲突），**不声称它们绿**；没跑 `gate-fast`/`layout-check`（`window.rs` 作单 crate 根时现在会连带 `buffer.rs`）、
     没做 P-86 ⑦ 的变异实测、Haiku/NS/sfnt/w32/msdos 各臂无工具链未编。

- **3.389 抽取器认下 P-86 那一类：`--all` 变成 588 done / 2 WIP / **0 failing**（2026-10-05）**：
  ① **问题**：`extract-abi --check` 的三项判据（DEFUN 是否存在、`defsubr` 是否被参照注册、非 static 定义是否
     "凭空发明"）只看**活行子集**；该文件在 `cond-eval-verified.txt` 里时死分支被剪掉，于是按 P-86 照录的死臂
     被读成"发明"（实测：decompress 6 行、lcms 17 行、frame 17 行——3.382/3.383/3.385 三笔一直带着这些 FAIL）。
  ② **改法（只改"名字在参照里到底存不存在"这一问）**：新增 `_without_dead_branches(fn)`（临时把
     `ci.dead_branch_map` 换成"每行都活"**并同时置 `ci.BLINDED = True`**，`finally` 还原）、
     `whole_file_defun_names` / `whole_file_exported_names` / `whole_file_defsubr_names` 与 `compare()` 内的惰性
     `whole_reference(which)`（正常文件零开销）；三处判据各把整文件集合减掉。**活行侧一个检查都没动**：
     DEFUN 参数/`noreturn`/docstring、`subr_defs` 一一对应、missing-* 系列、逐函数 token/body、常量/结构体值、
     行数漂移全部原样。`ci.BLINDED` 那一半是必须的：frame.c 82 行是死臂里的 `static` 原型、5409 才是定义，
     只抹平 liveness 会让 `function_spans` 把它判成内部链接（新增的自测用例钉住这点）。
  ③ **读数**：三条 `--check` **6 → 0 / 17 → 0 / 17 → 0**（末行都是 `OK`，行数漂移行也随 `total` 三行一起消失）；
     `--selftest` **12 cases, 0 failures**（新增 3 例）；**`--all`：`585 done / 2 in progress / 3 failing`（exit 1）
     → `588 done / 2 in progress / 0 failing`（exit 0）**，diff 的全部内容就是那三个文件从 FAIL 变 done
     （decompress 删 6 行、lcms 删 17 行、frame 删 17 行），**其余 587 个 pair 逐字节相同**，
     `xdisp`（99 行）/`xterm`（286 行）两个 WIP 的差异条数一字未变。这次 `--all` 的前后两次运行用
     `git rev-parse HEAD`（`ac6599c`）+ 全树 mtime/size 指纹锁住同一棵树（第一次跑时别的 lane 连提 5 个 commit，那次读数作废重做）。
  ④ **负对照（三条，全在 `/tmp` 副本上实测）**：把 `Flcms_cam02_ucs` 改成参照里不存在的
     `Flcms_cam02_ucs_zzz_nonexistent` → **仍报 FAIL**；改成只在**函数体里被调用**的 `Fcons` → **仍报 FAIL**；
     改成只出现在**注释里**的 `chromaticity` → **仍报 FAIL**。即"定义位置"这条约束没丢。
  ⑤ **未声称的边界（lane 照实记的）**：整文件集合是**该文件文本的全部**，所以 `#if 0` 臂里的定义/注册也算数——
     退化情形（某个 port-only 的名字恰好在参照的 `#if 0` 臂里被顺带写成一个定义）现在不会被报成 invented；
     收紧它需要区分"哪个臂在冻结配置下真为假"，属 P-86 的另一半工作，本轮明确不覆盖。
     另外它没跑 `gate-fast`/`cargo test`/`check-all-units.sh`，也没动 `docs/baseline/**` 与 `docs/ports/**`。

- **3.390 §8 第 5 条（原样复制）量清楚了：已跟踪内容逐字节一致，字面 `diff -r` 的差全是被参照
  `.gitignore` 忽略的构建产物（2026-10-05）**：
  ① 背景：参照检出**在本地被构建过**（冻结基线就在 `emacs/` 里 `make` 出来），所以它的 `lisp/*.elc`、
     `etc/charsets/*.map`、`doc/{emacs/emacsver.texi,man/emacs.1}`、`admin/**` 的 `*.stamp`/`*.elc` 留在树里。
  ② 实测：`diff -r` 只有 `test`/`leim` 无差异，其余四棵树的差异**全是"参照侧多出的文件"**（lisp 1727、etc 132、
     doc 4、admin 4），逐条问参照自己的 `.gitignore`——`*.elc`（:180）、`etc/charsets/*.map`（:265）、
     `doc/emacs/emacsver.texi`（:314）、`*.stamp`（:171）、`doc/man/emacs.1`（:315）**全部命中**。
  ③ 真判据（**已绿**）：六棵树**已跟踪文件数逐个相等**（lisp 1627/1627、test 1555/1555、etc 886/886、
     doc 255/255、leim 28/28、admin 253/253），`tools/check-verbatim.py` → **`6 trees, 4604 files, 0 failures`**。
     这一条与判据一起写进 `docs/acceptance.md`（新增一节），免得后来者对着字面 `diff -r` 反复排查。
  ④ 顺带记录一个事实：**`emacs/` 不在本仓库索引里**（`git ls-files emacs/lisp` = 0，它是外部只读检出），
     所以这条判据依赖本机的参照检出，CI 上无法复核（`docs/port-plan.md` §10 已记）。
  ⑤ **未声称**：§8 的其余七条一条都没点亮；本轮只把第 5 条变成"有判据、有读数"。

- **3.391 xdisp 第二十八轮：`try_window_id` 整函数落完（C 22545-23381，794 行）（2026-10-05）**：
  ① **边界校正**：简报说的"C 22588-23821"不是边界——C 23821 是**下一个**定义 `get_overlay_arrow_glyph_row`
     的所在行；`try_window_id` 自己在 **C 23381** 收尾（`23381-22588+1 = 794`），中间的 C 23388-23809 是整块
     **死的 `#ifdef GLYPH_DEBUG`**（`dump-*`/`trace-*` 七个 subr），本轮未转录、只记入 dead-arm 清单（deviation 209）。
     函数内 26 个活 `GIVE_UP`（+1 个死）、两条整流复用短路、`display_line` 循环、`dy && run.height` 的终端滚动臂
     （`set_terminal_window`/`ins_del_lines`）、`rotate_matrix`+`clear_glyph_matrix_rows`+`shift_glyph_matrix` 与四路
     `adjust_window_ends`/`window_end_*` 收尾全部按 C 顺序落下。
  ② **行数**：`femacs/src/xdisp.rs` 21,690 → **22,573（+885/-2）**；`docs/ports/xdisp.md` 2,830 → **2,979**；
     勾选 **322/415**（Functions 226/302）。
  ③ **读数**：`build-tu src/xdisp.c` → **built / exit 0**，对象 **2,230,704 字节**（上一轮 2,136,144；
     变大正是那四个 helper 第一次真的被 codegen 保留——上一轮它们只因 `try_window_id` 没落而没有机器码），
     警告仍同样六条、未新增；`extract-abi --check` 的 `^- ` **99 → 99（退 2 进 2）**，其中新增的
     `does not mention`（`try_window_id`：7 条活 `IF_DEBUG` 的 8 个 token）**集成者补
     `docs/ports/macro-body-notes.tsv` 一行后复读 = 98**；`--cfg enable_checking` 探针 **exit 0 / 0 error**
     （对象 2,785,240 字节 —— 本轮新加的 11 条 `eassert!/eassume!` 只有这条腿看得见，正是 3.387 的同类防线）；
     `check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、`check-carried` 11/11/0、
     `check-ledger-ticks` 235/0、`check-dead-file-coverage` 7/0。
  ④ **两点方法论（记下来）**：⑴ lane 首跑 `enable_checking` 用了**默认 stable** 工具链，得 16 个与本单元无关的
     `E0658`——**这条腿必须用仓库钉住的 `nightly-2026-10-01`**（`femacs/rust-toolchain.toml`），否则读数是噪声；
     ⑵ `build-tu` 只跑一发 rustc，**不编 `#[cfg (enable_checking)]` 臂**，所以这一类的门只能在探针里读（见 3.389 的工单）。
  ⑤ **未声称**：无行为验证（没有任何测试/差分证明"矩阵复用"运行时正确）；`eassert!/eassume!` 只在探针里过类型检查、
     从未执行；6 个 use-site 名（`update_begin`/`update_end`/`set_terminal_window`/`ins_del_lines`/
     `clear_glyph_matrix_rows`/`shift_glyph_matrix`，家在 `dispextern.rs` 镜像缺口）与脚手架里的
     `gui_clear_window_mouse_face` 本轮没有任何机器码；`HAVE_WINDOW_SYSTEM`/`HAVE_NTGUI`/`GLYPH_DEBUG` 各块
     只有文本记录；`build-tu` 4 发（超预算 1 发，如实记）。

- **3.392 P-86 第三件整文件：`androidfont.c`（1,102 行）落地（2026-10-05）**：
  ① **文件与台账**：`femacs/src/androidfont.rs` **15 → 1,391 行**；`docs/ports/androidfont.md` **27 → 295 行**
     （46 行 `## C line ranges` 覆盖 C 1-1,102，47 条块间空行免行）。落点：7 个 struct、7 个 file-scope static、
     **24 个函数**（五个 `android_init_*`、`androidfont_*` 十五个、`syms_of_androidfont_for_pdumper`/
     `syms_of_androidfont`/`init_androidfont`/`android_finalize_font_entity`）、导出全局 `androidfont_driver`（C 1038-1052）；
     12 处 C 内宏定义（`FIND_METHOD`×2、`FIND_FIELD`×3、`DO_SYMBOL_FIELD`×2、`DO_CARDINAL_FIELD`×4）按 C 落成函数内
     `macro_rules!`。**Subr 0 个**。C 的 `#ifndef ANDROID_STUBIFY` **没有 `#else` 臂**——stub 侧就是"什么都不定义"，
     与 androidmenu 的 3 行 stub 形态不同，台账已对比写明。
  ② **冻结档**：`build-tu src/androidfont.c` → **built / exit 0 / 336 字节空对象**（两次读数同值）。
  ③ **翻臂档（三档读数，分得很清）**：⑴ 2×`--cfg` 首跑 **exit 1 / 26 行 error**（5 `E0605`、1 `E0560 draw`、
     7 `E0609 font 的 window-system 成员`、2 `E0617` 等）；⑵ 修完单元侧后 2×`--cfg` → **exit 1 / 9 错，全在单元文本**，
     且**全是 `HAVE_WINDOW_SYSTEM` 类名字**；⑶ 加第三个 cfg（`have_window_system`，Android 真实配置）→ **exit 1 / 28 错，
     全在镜像副本**：`include/frame.rs` 19（11 个 `FRAME_*_SHIFT` + 3 个 scroll-bar 谓词，它的 window-system 区
     **只用不声明**）、`include/font.rs` 8（`E0080` 布局断言按冻结布局量）、`include/androidterm.rs` 1（`need_buffer_flip`，
     镜像自己登记过的不可编点）⇒ **单元文本 0 错**。⑷ 臂触达预检（三类补丁具名写在台账里）：**exit 0 / 0 error /
     225,624 字节对象**，并用**变异实测**证明它不是空读数（注入一处类型错误立刻 `E0308`）。
  ④ **集成者这一轮的动作**：补 `docs/ports/line-drift-tolerance.tsv` 的 `androidfont` 行（**0.30 / total**，实测 +26.2%）
     ⇒ `extract-abi --check emacs/src/androidfont.c femacs/src/androidfont.rs` 由"1 条行数漂移"变成 **`OK`（0 条）**。
     其它读数：`check-dead-file-coverage` **7 ledger(s) bound / 0 failure**、`check-port-lines` 报本件 **stale**（退出棘轮）、
     `check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、`check-carried` 11/11/0、
     `check-ledger-ticks` 235/0、`check-mirror-dupes` 18/0。NDK 常量是**实测**的（NDK 29 的 clang `--target=aarch64-linux-android24`
     编过 `_Static_assert (ANDROID_LOG_WARN == 5)` 与 `(ANDROID_LOG_FATAL == 7)`）。
  ⑤ **仍未解决（点名，交给下一轮）**：⑴ 三个**镜像缺口**要一轮 mirror round + `check-all-units.sh`：`frame.rs` 的
     window-system 区（11 个 shift 常量 + 3 个谓词）、`font.rs` 那 8 条按冻结布局量的断言（窗口系统布局 2026 年未量）、
     `androidterm.rs` 的 `FRAME_ANDROID_DRAWABLE` P-80 掩码；⑵ `macro-body-notes.tsv` 还欠 3 条 P-80 行
     （`need_buffer_flip`/`complete`/`need_cursor_updates`）；⑶ `check-port-lines --write-pending` 与 `gen-port-map.py`
     的数字要重生成（集成者在批末做）；⑷ lane 的 probe 用了 **4 发**（配额 3），如实记录。
  ⑥ **未声称**：冻结配置下这 1,102 行**一行都没编进去**（336 字节空对象）；所有读数都是**编译级**——没有 JNI 调用、
     没有字体、没有 frame、没有设备；⑷ 那一档是**替代读数**（补丁具名），不等于"无补丁也能编"；
     font 的布局断言被注掉、frame 的常量是 stand-in、掩码是手写 ⇒ 不声称 Android 窗口系统布局或掩码位序正确。

- **3.393 `enable_checking` 变成 `tools/check-all-units.sh` 的第二腿；83/83 全绿（2026-10-05）**：
  ① **先纠正一个前提**：`enable_checking` **已经在 `gate-fast` 里**（`build_finished_tus()`，自 3.184 起对同一
     `_root.rs` 再编一遍），3.387 记的"不属任何闸门"不准确。真正的缺口是**覆盖面**：那一半的单元集合来自
     `finished_units()`（**台账没有 `- [ ]`**），实测 81/83，差集正是 **`src/process.c` 与 `src/xdisp.c`**
     ——而 3.387 那条错误就藏在 `xdisp` 里（它台账未完成 ⇒ `gate-fast` 连 `build-tu` 都跳过，
     `check-all-units.sh` 又不带这个 cfg，于是这个单元此前**没有任何一条腿跑过 checking**）。
  ② **改动**：`tools/check-all-units.sh` 加第二腿——`build-tu` 已经把 `build/femacs/units/<stem>/_root.rs` 物化好，
     紧跟着用钉住的工具链对**同一个** `_root.rs` 跑 `--cfg enable_checking`（`--emit=metadata -o <scratch>`，
     收工 `trap` 清理），读数印成 `checking N unit(s), M unit failure(s)`；失败行按 P-86 ⑦ 的名口
     **区分"单元文本"与"镜像副本"**。冒烟：3 单元列表 → `checked 3 unit(s), 0 failure(s)` +
     `checking 3 unit(s), 0 unit failure(s)`（3.7 s；按实测 ≈0.6 s/单元，83 单元整轮约 +50 s）。
  ③ **实测读数（lane 报的，方法可复算）**：在 `305669b` 的 `git archive` 快照上（源文件 sha256 指纹前后一致）
     **83 个单元 × checking 腿全绿（exit 0，0 error；unit-text 0 / mirror-copy 0）**；工作树里 `frame`/`xdisp`
     （别的 lane 在飞）单独补测也绿 ⇒ **这一类现在是空的**。**阳性对照**（证明这条腿不是空读数）：把物化副本里
     `_root.rs:18902` 临时退回 3.387 之前的写法 → 立刻 `E0609`；还原后 0 错。
  ④ **两条方法论（都写进脚本注释）**：⑴ **工具链必须钉**——`rust-toolchain.toml` 在 `femacs/` 下，从仓库根用裸 `rustc`
     会落到 stable 1.98.1，**75/83 假红、134 个 `E0658`**，而且这些 feature-gate 错误**会把真错误挡在后面**
     （`character` stable 1 错 → nightly 0 错；`xdisp` stable 16 错 → nightly 0 错）；脚本里的 `cd "$root/femacs"` 就是钉它的那一步。
     ⑵ **不给 `--emit` 会把产物甩在当前目录**：探针那一轮在仓库根留下 **878 MB 的 `lib*.rlib`**（已删），所以第二腿
     显式写 metadata 到 scratch。
  ⑤ **未声称**：⑴ 这条腿只到**类型检查**（`--crate-type=lib`、`--emit=metadata`、不链接、不跑测试），不代表运行时行为，
     也不代表 ABI/layout/symbol/link 任何一层；⑵ 覆盖面只到 `frozen-build-sources.txt` 的 83 个单元与
     `#[cfg (enable_checking)]` 这**一个轴**——`#[cfg (feature = …)]` 臂（含 `all (feature, enable_checking)` 交集）
     归 `xtask dead-arms`（`gate-probes`）管，本轮未碰；⑶ **脚本的红路径没有端到端走过**：绿路径是 3 单元冒烟，
     红路径只在"命令级"被阳性对照证明过（脚本自身的失败分支靠代码审查，未构造真实红灯）；⑷ 工作树读数含在飞编辑（3.339 的两读数规则）。

- **3.394 frame.c 的 `HAVE_WINDOW_SYSTEM` 半身补齐：C 5921-6970 落地（2026-10-05）**：
  ① **区间**：21 个定义——`gui_set_vertical_scroll_bars`（C 5921-5951）到 `gui_figure_window_size`（6708-6967），
     中间含 `gui_set_alpha`/`alpha_background`、`validate_x_resource_name`、`gui_display_get_resource`、
     `x-get-resource`、`x_get_string_resource`、`gui_display_get_arg`、`gui_frame_get_arg`（static）、
     `gui_frame_get_and_record_arg`、`gui_default_parameter`、`XParseGeometry`（static）、`x-parse-geometry`。
     **整段 C 4889-6970 现在都带了**（上一轮 4889-5919）。
  ② **行数**：C 5921-6970 = 1,050 行；`femacs/src/frame.rs` 6,536 → **7,857（+1,321，1.26 端口行/C 行）**；
     对 C 全文件 7,907 行**只差 0.63%**（上一轮 17.3%），落在已改好的 `frame 0.18 total` 容差内。
  ③ **读数**：`build-tu src/frame.c` → **built / exit 0 / 3 warnings（全既有）**，对象 **413,320 字节**
     （与上一轮逐字节同形状：`nm` 1,523 行，`gui_*`/`XParseGeometry`/`Fx_*` 被门挡住的符号 **0 个**）
     —— 即这一臂在冻结配置下**一行都没编进去**（P-86 期望形状）。**`extract-abi --check emacs/src/frame.c
     femacs/src/frame.rs` → `OK`（0 条 `- `）**：本轮新增 19 个非 static 定义，由 3.389 的 `whole_reference`
     减法正确覆盖。`check-dead-file-coverage` **7 ledger(s) bound / 0 failure**（本台账 170 行覆盖 frame.c 7,907 与
     frame.h 1,956）；`check-port-lines` **90 文件 / 229,815 行**（2 stale）；`check-citations` 675/0 drift/0 stale；
     `check-declarations` 13/0/0；`check-carried` 11/11/0；`check-cond-eval src/frame.c` 105 conditions / 0 unknown /
     0 disagreements [VERIFIED]。旁证 `check-linkage` 2 units / 0 missing、`check-undef` 的 22 条 NEW 全部
     "referenced by xdisp.o"（别人的在飞单元）。
  ④ **集成者这一轮的动作**：把 lane 新引入的 feature 名 **`no_value`** 补进 `femacs/xtask/Cargo.toml` 的
     `[features]` 表（C 依据 `frame.c` 6538 的 `!defined (HAVE_X_WINDOWS) && defined (NoValue)`；
     `NoValue` 是**非 X** 的 `*gui.h` 常量——`androidgui.h` 236 / `haikugui.h` 77 / `nsgui.h` 116 实测都是
     `0x0000`）。不声明它，这一臂永远不会被任何配置打开。
  ⑤ **仍未解决（点名）**：⑴ 镜像工程——window-system 的 `want_fullscreen` 位段/flag run（上一轮遗留）＋本轮的
     `horizontal_scroll_bars`/`vertical_scroll_bar_type` 与六个 scroll-bar 槽位；⑵ 本轮 24 条 use-site 声明
     （六份 `Display_Info` typedef、`enum resource_types`、`check_x_display_info`、`note_mouse_highlight`、
     `x_get_string_resource`、`esprintf`、`EMACS_CLASS`、11 个被门掉的 `FRAME_*` 宏、五个非 X gui 头的
     gravity/position/`NoValue` 家族）在进各自镜像后要**同一提交删除单元副本**（`search.rs` 先例）；
     ⑶ C 7877-7894 那段死的 `#ifdef HAVE_WINDOW_SYSTEM`（`defsubr` 三个 DEFUN + `Qmove_toolbar`）仍在范围外、
     仍 recorded；⑷ `docs/ports/frame.md` 一份台账同时绑 frame.c 与 frame.h 的问题仍在。
  ⑥ **未声称**：这 1,050 行是**死代码**——只有文本级证据（C 行按块覆盖、逐 `#if` 判定、24 条引用经 citations 校核）
     与冻结读数（413,320 字节 / 1,523 符号 / 0 个该臂符号）；本机无 X11/NS/PGTK/W32/Haiku 头，**未做翻臂编译**，
     故不声称该臂编译过；`include/frame.rs` 的自述已写明 GUI 配置会在第一个 accessor 处编不过（翻臂会报错而不是静默错位）。
     另一条本轮实测：同一源两次编译的 `.o` 逐字节相同，但源码快照变化时 md5 会动（`1509205b…` → `93043164…`）
     而大小与 `nm` 不变 ⇒ **形状读数是大小＋`nm`，md5 只钉源码快照**。

- **3.395 最大的死文件开工：`w32fns.c` 第一片 C 1-884（12,488 行里的第一刀）（2026-10-05）**：
  ① **边界**：900 落在 `add_system_logical_colors_to_map`（C 887-932）**未完成的调用表达式**里（C 897-900），
     最近的整函数边界是 C 884（`w32_color_map_lookup` 的 `}`），下一个是 C 932 ⇒ 取 C 884。
     落点：`w32_window_to_frame`、`w32_real_positions`、`w32_fullscreen_rect`、`DEFUN w32-define-rgb-color`、
     `colormap_t` + `w32_color_map[240]`、`w32_default_color_map`、`w32_color_map_lookup` 等
     **6 个函数定义、29 个 `#define`、53 个文件级对象**（机械对账，全部在端口里）。
  ② **行数**：`femacs/src/w32fns.rs` **13 → 1,561 行**；`docs/ports/w32fns.md` **24 → 339 行**
     （63 行 `## C line ranges` 覆盖 C 1-12,488，末行 `885-12488` 明记"本片之外"以便机器门通过）。
     **本文件已退出 `<10%` 棘轮**（阈值 1,248，端口 1,561）——与派单时的预期相反，据实记。
  ③ **读数**：`build-tu src/w32fns.c` → **built / exit 0 / 336 字节空对象**（冻结配置一行不编）；
     `check-dead-file-coverage` **8 ledger(s) bound / 0 failure**；`check-port-lines` 报本件 **stale**（退出棘轮）；
     `check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、`check-carried` 11/11/0；
     **`check-cond-eval src/w32fns.c` → `117 conditions, 3 unknown, 3 disagreements [NOT VERIFIED]`**
     （C 167/8286/8300）⇒ **这个文件不在 `cond-eval-verified.txt` 里**，于是 `--check` 的清单是**整份文件**、
     不是活行子集（详见 ⑤）。
  ④ **`text-only`（点名缺的工具链）**：本机无 `x86_64-w64-mingw32-gcc`/`i686-w64-mingw32-gcc`、无 Homebrew
     `mingw-w64`、全盘无 `windows.h`。仍做了两档**文本级**附加读数并**都做了对照**：⑴ 翻臂
     （`--cfg have_ntgui,windowsnt,win32_windows_ge_0410`）**exit 1 / 161 error、本单元正文 0 条**，
     并用**变异实测**证明这是空读数（注入类型错误后输出逐字不变）；⑵ 臂触达预检（`/tmp` 复制物化图 +
     补镜像缺口 + 46 条桩）**本单元正文 0 错**、只剩 1 个镜像缺陷，且**正对照**（注入错误立刻报出）
     证明它真的在类型检查本单元正文。
  ⑤ **集成者这一轮的动作（我能证的）**：⑴ `femacs/xtask/Cargo.toml` 补两个单元已在用、表里却没有的 feature：
     `mingw_w64`（C 依据 `w32fns.c` 38/63）与 `win32_windows_ge_0410`（C 167 的 `_WIN32_WINDOWS >= 0x0410`）；
     ⑵ `docs/ports/macro-body-notes.tsv` 补**两行**——`w32_fullscreen_rect`（C 517 的 `switch (fsmode)` 四臂
     落成 `match`，`switch/case/default/break` 四个 token 无对应物）与 `SYSTEM_COLOR_PREFIX_LEN`
     （C 127 的 `sizeof (SYSTEM_COLOR_PREFIX) - 1` 落成 `.len ()`，`1` 与 `sizeof` 都无对应物，值同为 6）；
     补完后 `--check` 的 `does not mention` 行全部消失，读数 = **256 行、全是本片之外余段的 `missing *`**
     （与 xdisp 的 WIP 读数同类）。⑶ **刻意不加** `line-drift-tolerance.tsv` 行：xdisp 同为 WIP 也没有行，
     而半成品的"87.5% 漂移"是**进度标记**、不是结构膨胀；这一行应在文件落完时按最终密度推导（记在这里，
     不留给下一个人猜）。
  ⑥ **仍未解决（点名，交给下一轮）**：⑴ 五处镜像缺陷（`FRAME_DISPLAY_INFO` 宏体 `(void)($f)` 非 Rust、
     `w32_kbdhook_active` 把宏写成 `pub const`、`FRAME_W32_P` 缺 `#ifdef HAVE_NTGUI` 对臂、`enum fullscreen_type`
     不在 `frame.rs`、`dispextern.rs` 缺 `check_x_display_info` 原型）；⑵ **10 条声明↔定义冲突（E0428）**
     （`w32_fullscreen_rect`/`w32_window_to_frame`/`w32_real_positions`/`menubar_in_use`/`hinst`/`syspage_mask`/
     `w32_*_version`/`w32_build_number`/`os_subtype`）要把镜像声明搬进 `*_decls`/`*_objs` 模块（window.rs 先例）；
     ⑶ `SYSTEM_INFO`/`OSVERSIONINFO` 两个 C 记录**无头文件可测**（按 `w32common.rs` 先例 defer）；
     ⑷ 结构性的：**w32fns.c 不在 cond-eval-verified**（3 个 unknown），因此在那之前 `--check` 永远要求整份文件
     ——要不要把这三个条件办进验证表，是下一步的一个独立决定。
  ⑦ **未声称**：`text-only` —— 没有编译过任何 Windows 目标、**没有任何 `<windows.h>` 的尺寸/偏移测量**
     （`RECT`/`MSG`/`TITLEBARINFO` 等布局 **UNVERIFIED**，断言属 owed）、无链接无运行、预检的 46 条桩是 `/tmp` 脚手架。

- **3.396 本批收口：两腿全量扫描 `83/0` + `83/0`，棘轮 90 文件 / 229,815 行，port-map 55.5%（2026-10-05）**：
  ① **全量扫描（`tools/check-all-units.sh`，工作树 = 落完的 HEAD `3239234`）**：
     `checked 83 unit(s), 0 failure(s)`（冻结腿）+ **`checking 83 unit(s), 0 unit failure(s)`**（3.393 新加的
     `enable_checking` 第二腿），整轮 **1 分 37 秒**（其中 checking 腿约 50 s）。
  ② **第二腿的第一次全量跑就抓到东西——抓到的是脚本自己的缺陷**：4 个单元红在
     `error: invalid character '-' in crate name: make-fingerprint`——`--crate-name` 不能用连字符，而列表里有
     `lib-src/make-fingerprint.c`、`src/regex-emacs.c`、`src/region-cache.c`。修法：把 stem 里的 `-` 换成 `_`
     再传给 `--crate-name`（注释里记了为什么）。**这条红的好处是：3.393 里"脚本的红路径没有端到端走过"
     这个限度现在**不成立了**——红路径是真实跑出来的（FAIL 行 + 非零计数），修完复读才是 `83/0 + 83/0`。
  ③ **行数棘轮收口**：`--write-pending` → **90 文件 / 229,815 行（0 new, 0 stale）**（本批入口是 92/243,405，
     ⇒ **净 −2 文件 / −13,590 参照行**：`androidfont` 与 `w32fns` 退出该棘轮）。
  ④ **进度表重生成**：`python3 tools/gen-port-map.py` → 1:1 Rust 590 条的**目标行数 376,657 / 678,320 = 55.5%**
     （本批入口 371,529 / 54.8%；上一批收口 359,810 / 53.0%）。
  ⑤ **本批落地的四件（3.391-3.395）**：xdisp 第二十八轮（`try_window_id` 整函数，+885 行）、`androidfont.c` 整文件
     （+1,376 行）、frame.c 的 `HAVE_WINDOW_SYSTEM` 半身补齐（C 5921-6970，+1,321 行）、`w32fns.c` 第一片
     （C 1-884，+1,548 行）——合计约 **+5,300 行端口文本**。
  ⑥ **未声称**：⑴ 两腿都只是**类型检查级**（不链接、不跑测试、不代表运行时行为）；⑵ 棘轮量的是**参照行数**，
     不是完成度；⑶ port-map 的 55.5% 是**行数比**，不等于 590 条里有 55.5% "落完"（清单/布局/符号/subr 与
     L0-L5 仍未点亮）；⑷ §8 里只有第 5 条（原样复制）有判据与读数（3.390）。

- **3.397 链接棘轮复算（3.396 之后）：382 行 / 0 failure，三退两进（2026-10-05）**：
  ① **跑法**：`tools/reconcile-link-ratchets.sh`（一次性 worktree 在 **HEAD = `8666ae1`** 上重建，`--strict` 绿了才写回四本表；
     在飞单元按"工作树里脏的单元"识别、用 HEAD 自己的源码重建），**4 分 09 秒**，exit 0。
  ② **读数**：`check-link.py` → **`382 undefined symbol(s) from 76 program + 87 libgnu object(s), 382 ratchet row(s),
     0 failure(s)`**（上一批 383）；`check-linkage.py` → **`2 unit object(s) compared …, 0 with 0 missing symbol(s)`**；
     Swift 两本（`swift-units.tsv` 51 行、`swift-import-pending.txt` 801 行）**未动**（本批没碰 Swift）。
  ③ **表里的进出（diff 全文）**：**退三条**——`_try_window_id`、`_try_window_reusing_current_matrix`、`_row_containing_pos`
     （本批 xdisp 把这三个定义落了，于是它们从"未定义引用"名单里消失）；**进两条**——`_gui_clear_window_mouse_face`
     （xdisp 里等 C 37705 定义的非 static 名，台账 to-do item 67 记着）与 `_row_for_charpos_p`（同族的 interim 名）。
     净 **−1 行**。⇒ 这正是 `check-link` 该有的行为：**它量的是"参照定义、端口还没有"的差集**，落一个函数就退一行。
  ④ **未声称**：链接棘轮只说明"未定义引用名单与 HEAD 的 83 个单元对象一致"，**不是链接成功**（没有任何可执行文件被链出来）；
     Swift 两本没动只说明本批没改 Swift，不是它们的读数变了。

- **3.398 xdisp 第二十九轮：C 23817-25075 十一个定义（2026-10-05）**：
  ① **区间与清单**：`get_overlay_arrow_glyph_row`（C 23821）、`insert_left_trunc_glyphs`（23893）、
     `row_hash`（24048，**非 static**）、`compute_line_metrics`（24082，收掉一个 scaffold）、`clear_position`（24157）、
     `append_space_for_newline`（24176）、`extend_face_to_end_of_line`（24384）、`trailing_whitespace_p`（24869）、
     `highlight_trailing_whitespace`（24891）、`row_for_charpos_p`（24975，收 scaffold）、`cursor_row_p`（25062，收 scaffold）。
     区间上方 C 23385-23816（"More debugging support" 横幅 + 整块死 `#ifdef GLYPH_DEBUG`）上一轮已记进 dead-arm 清单，
     本轮只做边界对账、不转录。**下一个活定义是 C 25077 `push_prefix_prop`**。
  ② **行数**：`femacs/src/xdisp.rs` 22,573 → **23,569（+996）**，覆盖到 C 25075；台账 2,979 → **3,090**；
     勾选 `Functions 237/302`、合计 **333/415**；scaffold 17 → 14。
  ③ **读数**：`build-tu src/xdisp.c` → built / exit 0，对象 **2,281,096 字节**（上轮 2,230,704），警告同样六条未增；
     **`extract-abi --check` 的 `^- ` 98 → 87**（退掉本轮 11 条 `missing function`，**无新增**，
     所以**不欠 `macro-body-notes` 行**）；`--cfg enable_checking` 探针 **exit 0 / 0 error**（对象 2,845,464 字节，
     本轮两条 `eassert!` 只在这一档被检查）；`nm` 新增 4 个符号（`T _row_hash` + 三个 static）。
     `check-citations`/`check-declarations`/`check-carried`/`check-ledger-ticks` 全绿；报告里另外两条红
     （citations 2 drift、dead-file-coverage 2 failure）**全部落在别的 lane 的在飞文件**（`androidterm.rs`/`image.rs`/
     `docs/ports/androidterm.md`），本单元零 finding。
  ④ **未声称**：本轮 7 个 static **只有类型检查、没有机器码**（C 调用点还在未转写区，`nm` 无符号，记 item 72）；
     五块 `HAVE_WINDOW_SYSTEM` 死臂按 P-08 记入台账不转录；无运行时证据（无链接、无测试）；
     `it`/`face`/`glyph` 仍是本单元自己的量测副本（`struct face.background` 的偏移 176 有 `face-layout.tsv` 与
     `xfaces.rs` 双证），**未改任何镜像文件**（两个 extern 与 3 个成员/访问器记进 to-do item 71）。

- **3.399 P-86 第四件：`androidterm.c` 第一片 C 1-829（2026-10-05）**：
  ① **区间**：收在 C 829（`android_handle_ime_event` 的 `}`）——C 836-1892 是**单个 1,056 行的
     `handle_one_android_event`**，取它之后的边界会到 C 1892（远超 1,000），829 之前又没有别的整件边界。
     **下一个活定义 = `handle_one_android_event`（C 836-1892）**，其前置声明在 C 834。
     落点：3 个文件级对象（`x_display_list` 非 static、`any_help_event_p`、`wheel_event_x/y`）、
     **28 个函数**（7 个按 C 取地址 → `unsafe extern "C" fn`）、5 个单元内 `macro_rules!` 与 3 个 `ANDROID_EVENT_*` 枚举。
  ② **行数**：`femacs/src/androidterm.rs` **11 → 1,055 行**；台账 24 → **302 行**（41 行区间表连续覆盖 C 1-7,117）。
  ③ **读数**：冻结档 `build-tu` → built / exit 0 / **336 字节空对象**；**翻臂 `probe`（2 cfg）→ exit 1，17 条 error，
     按文件分类 镜像 11 / 单元文本 6**——镜像 11 全是 `include/androidterm.rs` 那**三个 `bool_bf` 宏体**（441/452/462），
     单元文本 6 是 C 332 `MOUSE_HL_INFO` 的 `mouse_face_defer`、C 707-708 `need_cursor_updates`、
     C 458 `android_frame_rehighlight`（定义在 C 2127）与 C 795-796 需要 `have_text_conversion`（cfg 集没设它，
     非移植缺陷）。**名字解析没有中止**（与 androidmenu 首读不同），所以这 17 条是该 cfg 集下的完整错误表。
     `check-dead-file-coverage` **10 ledger(s) bound / 0 failure**；`check-port-lines` → **87 文件 / 203,995 行**
     （3 stale，含本件）；`check-citations`/`check-declarations`/`check-carried` 全绿。
  ④ **集成者这一轮的动作**：`macro-body-notes.tsv` 补 **4 行**，每条的 C 依据我都实读核对过——
     `android_toggle_visible_pointer`（C 288 `goto set_invisibility` / C 300 标签）、`android_focus_changed`
     （C 484-485 的两处 `= 0` → `null_mut ()`）、`android_detect_focus_change`（C 505 `switch (event->type)`，臂 507/508、`default:` 513）、
     `android_handle_ime_event`（C 793 的 `switch (event->ime.operation)` 七臂 + C 718 `Lisp_Object text UNINIT;` 的零值化）。
     补完 `--check` 由 **116 → 112**（余下全是余量 C 830-7,117 的 `missing *` 与行数漂移）。
     **刻意不加** `line-drift-tolerance.tsv` 行：与 xdisp 同口径（WIP 的漂移是进度标记，不是结构膨胀），
     该行应在文件落完时按最终密度推导。
  ⑤ **仍未解决（点名）**：⑴ 三个 `bool_bf` 宏（`FRAME_ANDROID_NEED_BUFFER_FLIP`/`DRAWABLE`/`COMPLETE_P`）要 **P-80 掩码形状**
     + 三行 notes（这是挡住翻臂的镜像侧主因）；⑵ 镜像轮：`include/frame.rs` 的 window-system 编号与 `MOUSE_HL_INFO` 的
     Android 臂（现在只有冻结的 TTY 臂）、`include/dispextern.rs` 的 window-system 半；⑶ 8 条 use-site 声明归位
     （`popup_activated`/`gui_mouse_grabbed`/`buffer_flipping_blocked_p`/`remember_mouse_glyph`/`note_mouse_highlight`/
     `setup_coding_system`/`detect_input_pending`/`android_create_gc`）+ `struct redisplay_interface` 与 `textconv_decls` 的 glob；
     ⑷ **本台账同时绑 `androidterm.c` 与 `androidterm.h`**（与 `frame.md` 同族问题），修法是给 header 单独一份台账或改解析规则；
     ⑸ 预算超支如实记（build-tu 5 发 vs 3、probe 4 发 vs 3，原因逐条在 Evidence）。
  ⑥ **未声称**：冻结配置下这 829 行**一行都没编进去**；翻臂只是**编译级/类型检查级**读数，没有 JNI 调用、没有 frame、
     没有光标/铃声/IME 事件被执行；**C 830-7,117 只是登记、没有移植**；`have_window_system` 一路没进 probe，
     所以"Android 真配置下的正确性"本片给不出读数。

- **3.400 P-86 第五件：`haikuterm.c` 第一片 C 1-1044（2026-10-05）**：
  ① **区间**：收在 C 1044（`haiku_draw_text_decoration` 的 `}`，距 1000 只差 44 行；上一项边界 C 885）。
     **下一个活定义是 `haiku_draw_string_box`（C 1046-1106）**。落点 **37 个定义 + 10 个文件级对象/宏**，
     台账 52 行区间表连续覆盖 C 1-4,898（`haiku_fill_underline` C 846-886 含 `switch` + DOTS→DASHES 的 `FALLTHROUGH`，
     `haiku_draw_text_decoration` C 887-1044）。
  ② **行数**：`femacs/src/haikuterm.rs` **11 → 1,555 行**；台账 24 → **316 行**。
  ③ **读数**：冻结档 `build-tu` → built / exit 0 / **336 字节空对象**（2 发，末版用同一 materialized root 复现）；
     `check-cond-eval src/haikuterm.c` → **5 conditions, 0 unknown, 0 disagreements [VERIFIED]**；
     `check-dead-file-coverage` **12 ledger(s) bound / 0 failure**；`check-port-lines` **87 文件 / 203,995 行**
     （3 stale，含本件）；`check-citations`/`check-declarations`/`check-carried` 全绿。
     **无一档 Haiku 目标编译**（`which` 四种 haiku-gcc 全空、`rustup target list` 无 haiku、brew 无 haiku、
     全盘无 `be_interface.h`）⇒ 按 P-86 ③⑵ **`text-only`**；lane 另在 `/tmp` 的 harness 上做了只读/写读数
     （2 cfg 下发 `exit 1 / 78 error`，其中单元文本 6 / 镜像 72）+ **变异对照**（注入后 79 行），并自己标明
     "那不是 Haiku-target 编译"。
  ④ **集成者这一轮的动作**：⑴ `macro-body-notes.tsv` 补 **5 行**，C 依据逐条实读并核过端口拼法——
     `haiku_focus_frame`（C 547 `haiku_frame_raise_lower (frame, 1)`，端口写 `true`）、`haiku_new_focus_frame`
     （C 560/565 的 `0`/`1` → `false`/`true`）、`haiku_implicitly_set_name`（C 575 `haiku_set_name (f, arg, 0)` → `false`）、
     `haiku_draw_underwave`（C 764 `float ax, ay, bx, by;` → `f32`）、`haiku_fill_underline`（C 846-886 的 `switch` +
     C 874 的 `FALLTHROUGH` → `match`）；补完 `--check` **76 → 71**（余下全是 C 1045-4,898 余量的 `missing *` 与行数漂移）。
     ⑵ **本轮含一处镜像改动**（`femacs/src/include/haikuterm.rs` +10/-3：`FRAME_COMPLETE_P` 加 C 的写臂；
     `haiku_output_bit_p/_set` 的形参改 `*mut c_void` 内部一次 cast——因为 `FRAME_OUTPUT_DATA` 给的是 P-72 ③
     的 `*mut c_void`，原签名让每个 `MAKE_FRAME_DIRTY` 展开成 `E0308`），**按 P-87 跑了全量扫描**（读数见下一条）。
  ⑤ **仍未解决（点名）**：⑴ 镜像轮：`include/frame.rs` 缺 `tooltip` 成员与**全部 window-system 标志编号**
     （`FRAME_*_SHIFT` 全在 `not(have_window_system)` 下）、`include/dispextern.rs` 缺 `#include "fontset.h"`、
     19 条 `eassert`/`XUNTAG` 的 include 自足性、9 条 window-system 臂的 `E0080` 布局断言；
     ⑵ **`include/window.rs` 399 的 `WINDOW_BUFFER_LOCAL_VALUE (w, v)` 形参顺序与 C 的 `(V, W)` 相反**
     （单元侧已交换实参并注明，镜像该改）；⑶ 15 条 use-site 声明归位 + `struct face`/21 个 `FACE_*` 掩码的镜像副本；
     ⑷ **刻意不加** `line-drift-tolerance` 行（WIP 口径同 xdisp），该行应在文件落完时按最终密度推导。
  ⑥ **未声称**：**无 Haiku 构建、无链接、无运行**；③ 的 harness 读数不是 Haiku-target 编译；
     `build-tu` 的 336 字节空对象只证明"门在、冻结档编不到"，不是新文本的证明；`femacs/platform/haiku/*.cc` 那一侧未触及。

- **3.401 最大的死文件第一片：`image.c` C 1-1095（2026-10-05）**：
  ① **区间**：收在 C 1095（`image_create_bitmap_from_file` 的 `}`）；**下一个活定义 = `free_bitmap_record`（C 1099-1119）**。
     41 个块，覆盖 include 图与平台宏/类型段（X / CAIRO / NS / PGTK+IMAGEMAGICK / NTGUI **两臂** / HAIKU / ANDROID / `ANDROID_STUBIFY` /
     `WORDS_BIGENDIAN` 全录）、CAIRO 六个像素容器助手、NS 两个 shim、位图访问器、PGTK pattern 构造、
     `image_create_bitmap_from_data`（C 595-735）与 `image_create_bitmap_from_file`（C 755-1095）。
  ② **行数**：`femacs/src/image.rs` **11 → 1,714 行**（密度 1.57）；台账 **24 → 401 行**（P-08 的"空单元"整篇改写成 P-86 三节 + 41 行区间表）；
     6 个导出定义全部 `#[unsafe(no_mangle)]`。
  ③ **读数**：冻结档 `build-tu` → built / exit 0 / **336 字节空对象**（末版 md5 与直接 rustc 复现一致）；
     **翻臂两发**：NS 配置 `exit 1 / 55 error 行（127 位点）、0 个在单元文本`（全在镜像：frame 45 / dispextern 36 / nsterm 32 /
     font 8 的 `E0080` / window 3 / lisp 1 / stdbit 1）；X+cairo 配置 `exit 1 / 233 error 行`，单元文本 3 条是 Xlib/Xutil 的 SDK 名
     + `E0428 x_bitmap_stipple`（与 `include/xterm.rs` 190-192 的声明撞名）。**臂触达预检 + 变异实测**：NS 配置下 harness
     **exit 0 / 0 error / 31,520 字节对象**，注入一处类型错误立刻 `E0308`（非空读数）。
     `check-dead-file-coverage` **13 ledger(s) bound / 0 failure**；`check-citations` 675/0 drift/0 stale；
     `check-port-lines` 报本件 **stale**（退出棘轮）。
  ④ **本片的一个方法论收益（值得记）**：lane 顺手跑了**九配置扫描**（frozen/NS/haiku/android+stub/android/ntgui/pgtk/x11/x11+cairo），
     除了给出每个配置"单元文本多少错"的表，**它抓到了三处翻臂抓不到的缺陷**——Haiku/Android 臂的 `!(STRINGP (…) != 0)`、
     `android_create_bitmap_from_data` 的两个 `c_uint` 形参、四处 `f->foreground_pixel & 0xffffffff` 写进 `uint32_t`
     字段需要显式截断。也就是说：**多配置只读扫描是 P-86 这一类里比"挑一个配置翻臂"更强的读数**，值得成为后续轮次的常规动作。
  ⑤ **集成者这一轮的动作**：`femacs/xtask/Cargo.toml` 补 **四个**单元已在用、表里没有的 feature——
     `have_png`（C 30）、`have_gif`（C 244）、`have_webp`（C 244）、`have_imagemagick`（C 104，
     `#if defined HAVE_PGTK && defined HAVE_IMAGEMAGICK`）。`--check` 无新增 `does not mention`，故不欠 notes 行；
     行数容差行按 WIP 口径**刻意不加**（同 xdisp/haikuterm）。
  ⑥ **仍未解决（点名）**：⑴ 五条臂是 **text-only**（无 mingw/windows.h、无 Haiku SDK、无 GTK/PGTK；X11 臂因 `include/xlib.rs`
     缺名不可编）；⑵ **本机没有任何 window-system TU 能编过**，所以"单元 0 错"只能靠 harness + 变异测试成立；
     ⑶ 镜像缺口一串：`include/pgtkterm.rs` 是无 `include!` 链的占位、`frame.rs` 的 window-system 区、`dispextern.rs` 的
     `Emacs_Cursor` 未分臂、`nsterm.rs` 缺 `Window`/`NSArray`/`NSString`/`NSRect`/`EmacsMenu` 且不引 `nsgui.h`、
     `font.rs` 8 条按冻结布局的断言、`xterm.rs` 缺 Xlib 记录/过程、`w32gui.rs` 202 把 `XImage` 门在 `windows`、
     `xterm.rs` 190-192 的 `x_bitmap_stipple` 声明与单元定义撞名（要 `*_decls` 模块化）；树里**没有 cairo 镜像**；
     ⑷ 预算：`build-tu` **8 发**（超 5），原因（4 发失败迭代）记在台账 Deviations。
  ⑦ **未声称**：无运行/行为证据；NS 的 `exit 0` 是 **harness 读数**（打了三类补丁，不是 `xtask probe` 读数）；
     Windows/Haiku/PGTK/Android 臂**不声称能编或布局正确**；**C 1,096-13,805 只是登记、没有移植**。

- **3.402 本批收口：棘轮 87 文件 / 203,995 行（0 stale），port-map 56.3%，P-87 两腿 83/0（2026-10-05）**：
  ① **P-87 全量扫描**（3.400 里那处 haiku 镜像改动触发）：`checked 83 unit(s), 0 failure(s)` +
     `checking 83 unit(s), 0 unit failure(s)` —— 两腿全绿，即这个镜像改动对 83 个受编单元是 no-op（Haiku 不在其中），
     而这是"改镜像必须证明所有 includer 都能编"这条规则的落法。
  ② **行数棘轮收口**：`--write-pending` → **87 文件 / 203,995 行（0 new, 0 stale）**；本批入口是 90/229,815
     ⇒ **净 −3 文件 / −25,820 参照行**（`androidterm.c`/`haikuterm.c`/`image.c` 三件退出 `<10%` 类）。
  ③ **进度表重生成**：`gen-port-map.py` → **381,951 / 678,320 = 56.3%**（本批入口 55.5%、上一批收口 53.0%）。
  ④ **门的一行读数**：`check-dead-file-coverage` **13 ledger(s) bound / 0 failure**、`check-mirror-dupes` **18/18/0**、
     `check-citations` **675 / 0 drift / 0 stale**、`check-declarations` **13 sites / 0 forbidden**。
  ⑤ **本批落地的四件（3.398-3.401）**：xdisp 第 29 轮（C 23817-25075，11 个定义，+996 行）、`androidterm.c` 第一片
     （C 1-829，+1,044 行）、`haikuterm.c` 第一片（C 1-1044，+1,544 行）、`image.c` 第一片（C 1-1095，+1,703 行）
     ——合计约 **+5,300 行端口文本**；另 3.397 的链接棘轮复算（382 行 / 0 failure）。
  ⑥ **一条方法论（image.c 片带来）**：**多配置只读扫描**比"挑一个配置翻臂"更强——九配置扫描抓到三处翻臂看不见的缺陷
     （Haiku/Android 臂的 `!(STRINGP (…) != 0)`、`android_create_bitmap_from_data` 的 `c_uint` 形参、
     四处 `& 0xffffffff` 写 `uint32_t` 缺显式截断）。后续 P-86 轮次应把它当常规动作。
  ⑦ **未声称**：两腿都只是类型检查级；棘轮量参照行数、port-map 的 56.3% 是行数比而非完成度
     （清单/布局/符号/subr 与 L0-L5 仍未点亮）；§8 八条里仍只有第 5 条有判据与读数。

- **3.403 两条为下一轮定方向的读数：验证表覆盖率与"签名+推送"这一条（2026-10-05）**：
  ① **`cond-eval-verified.txt` 的覆盖率**（决定 P-86 那一类文件能不能让 `extract-abi --check` 变绿）：
     表里 **424 个文件**；P-86 待办 **87 个文件**里 **73 个在表内**、**14 个不在**（合计 **52,934 参照行**）。
     不在表内的按大小是：`src/w32.c`（11,362）、**`src/xfns.c`（10,662）**、**`src/w32term.c`（8,414，正是本轮在派的那件）**、
     `src/androidvfs.c`（7,950）、`src/gtkutil.c`（6,753）、`lwlib/lwlib-Xm.c`（1,964）、`src/w32menu.c`（1,850）、
     `lwlib/lwlib.c`（1,330）、`src/sfntfont-android.c`（821）、`lib-src/update-game-score.c`（506）、
     `lib-src/seccomp-filter.c`（409）、`lib/c++defs.h`（367）等。
     **为什么这条重要**（3.395 的 w32fns.c 已经撞过）：不在验证表里的文件，抽取器**不剪死分支**、`--check` 的清单是**整份文件**，
     于是哪怕照 P-86 逐行照录，也永远只能读到一长串 `missing *`（w32fns 现在是 256 行）而不可能 `OK`。
     ⇒ **"把这 14 个办进验证表"是那些文件证据链的前置条件**，而不是可选项；办法是逐个跑
     `python3 tools/check-cond-eval.py src/NAME.c` 看 unknown/disagreement 属于哪一类（求值器缺口 vs 真的判不了），再决定补宏表/补求值器/记豁免。
  ② **§8 第 8 条的前半（"全部提交签名并推送"）核过**：`git log --format='%G?' --all` → **0 条未签名**；最近 30 条全 `G`；
     共 **820 个提交**；工作树与 `origin/master` 同部（`git rev-list --count origin/master..HEAD` = 0）。
     后半（`docs/ports/*.md` 清单全部勾选）**远未达成**（WIP 文件按定义还有未勾项），本条**只声称签名与推送这一半**。
  ③ **未声称**：① 只是覆盖率读数，不代表那 14 个文件的移植难度或证据质量；② 不把 §8 第 8 条整体说成已点亮。

- **3.404 验证表与求值器已经不同步；"办进验证表"这件事需要先定一个前提（2026-10-05）**：
  ① **实测**：对 3.403 点名的几个大文件跑 `python3 tools/check-cond-eval.py NAME.c`——
     `src/w32term.c` **41 conditions, 2 unknown, 0 disagreements → 末行打 `[VERIFIED]`**；`src/xfns.c` **231 / 0 / 0 → `[VERIFIED]`**；
     `src/gtkutil.c` 212 / 1 / 1；`src/w32.c` 35 / 5 / 4；`src/androidvfs.c` 14 / 0 / 5。
     即：**`cond-eval-verified.txt` 里没有 w32term.c/xfns.c，但求值器今天的判据已经给它们 `[VERIFIED]`（0 disagreements）**
     —— 表是**陈旧的**（它由某次 `--write-verified` 写出，之后没人重写）。
  ② **为什么不能顺手写进去**：写表的口径是"零 disagreements 的文件"（`--write-verified PATH`，且必须带
     `--verified <list>` 才会把已知文件一起保留，否则会把 424 条写没）——这一点本身可控（只增不减）。
     但还有一个**前提没定**：`src/w32term.c` 这类文件**整份不在冻结构建里**（P-73 的源列表排除），
     而求值器对它的 `#if` 是逐个判的，`_WIN32_WINNT < 0x0500 && !defined (MINGW_W64)` 在冻结目标上按 C 规则是**真**
     （未定义标识符按 0），于是"活行"里会留下老 Windows 的臂 —— 也就是说，**"进验证表"会让清单从'整份文件'变成'一些本不该活的行'**，
     而这两种都不是 P-86 想要的那个答案（这一整类文件应当"整份照录、证据是文本级"）。
  ③ **决定（本轮不擅自改表）**：把"源列表排除的文件在 `--check` 里该怎么算"与"验证表覆盖"**绑在一起定**，
     由集成者在下一轮出一版方案（候选：给抽取器一条"本文件不在冻结构建源列表 ⇒ 清单为空、由 C 行号区间表承担覆盖"的规则，
     与 3.389 给"整身死"文件加整文件集合的做法同族）；在那之前**不动** `cond-eval-verified.txt`，
     也不把"表陈旧"写成"求值器错了"。
  ④ **未声称**：① 的读数只覆盖列出的五个文件；② 不给"这 14 个文件何时能 OK"的时间表。

- **3.405 xdisp 第三十轮：C 25077-25863 十一个定义（2026-10-05）**：
  ① **区间与清单**：`push_prefix_prop`（25077）、`get_it_property`（25175）、`get_line_prefix_it_property`（25196）、
     `handle_line_prefix`（25217）、`unproduce_glyphs`（25258）、`find_row_edges`（25278）、
     `display_count_lines_logically`（25431）、`display_count_lines_visually`（25450）、
     `maybe_produce_line_number`（25499，本轮最大）、`should_produce_line_number`（25800）、`row_text_area_empty`（25835）——
     全部 `static`。**下一个活定义 = C 25865 `display_line`（C 25865-26935，1,072 行 = 下一轮整轮）**。
     本轮 **787 个 C 行**（低于 1,000-1,400 的目标，理由见台账 ①：这一组的自然边界就是 25863，
     再往上要么整吞 `display_line`、要么把它切成半截 stub，两者都不许）。
  ② **行数**：`femacs/src/xdisp.rs` 23,569 → **24,435（净 +866）**；台账 3,100 → **3,220**；
     勾选 `Functions 248/302`、合计 **344/415**；脚手架 13 → 12。
  ③ **读数**：`build-tu src/xdisp.c` → built / exit 0，对象 **2,358,272 字节**（上轮 2,281,096），警告同样六条未增；
     **`--check` 的 `^- ` 87 → 78**（退 11 条 `missing function`，新增 2 条：一条 `does not mention` 由集成者补
     `macro-body-notes` 后复读 **77**，另一条是脚手架声明的 `no_mangle` 暂态行）；
     `--cfg enable_checking` 探针 **exit 0 / 0 error**（对象 2,930,272 字节，本轮 10 处 `eassert!` 只由它读过）；
     `nm` 新增 8 个 `t` 符号；`check-citations` 675/0 drift/0 stale、`check-declarations` 13/0/0、
     `check-carried` 11/11/0、`check-dead-file-coverage` 13/0（上一轮 androidterm 的两条 failure 已消失）。
  ④ **集成者这一轮的动作**：补 `docs/ports/macro-body-notes.tsv` 一行 `maybe_produce_line_number`（`alloca`/`int_strlen_bound`，
     C 依据 **C 25642** 的 `char *lnum_buf = alloca (max (it->lnum_width, INT_STRLEN_BOUND (ptrdiff_t)) + 2 + 1)`——
     我实读核过行号；端口用 SAFE_ALLOCA 族、`INT_STRLEN_BOUND (ptrdiff_t)` 在冻结目标量得 20）。
  ⑤ **未解决（点名）**：⑴ `display_count_lines` 的 `no_mangle` 暂态行（C 29914 落定即消）；⑵ text-vs-machine 缺口——
     `find_row_edges`/`unproduce_glyphs`/`row_text_area_empty` 目前**无机器符号**（调用者 `display_line` 还没进来）；
     ⑶ 5 个 use-site extern（`save_restriction_save`/`_restore`、`labeled_restrictions_remove_in_current_buffer`、
     `disable_line_numbers_overlay_at_eob`、`strcat`）与 `struct it.eol_pos`（本轮把 `_pad_f` 转正）该进镜像。
  ⑥ **未声称**：不声称 `src/xdisp.c` 可编译/链接（`--check` 仍 FAIL、行数漂移 38.6%、77 条未闭合）；
     token 包含检查通过 ≠ 语义等价；`xtask layout-check` 本轮未跑（如实记未跑，不当作通过）。

- **3.406 P-86 第六件：`w32term.c` 第一片 C 1-1002（2026-10-05）**：
  ① **区间**：收在 C 1002（`w32_define_fringe_bitmap` 的 `}`；C 1000 落在该函数内，最近整项边界就是 1002）。
     **下一个活定义 = `w32_destroy_fringe_bitmap`（C 1005-1013）**。落点 **28 个定义 + 10 个宏/常量族**，
     含 include 图、fringe/caret/scroll/codepage 文件级对象、13 个后备缓冲/裁剪/波形/矩形/清窗/透明度/像素尺寸函数、
     `w32_update_begin`…`w32_flip_buffers_if_dirty` 的 update 段与 `w32_after_update_window_line`/`w32_draw_fringe_bitmap`。
     59 行区间表**同时满足 `w32term.c` 与 `w32term.h` 两行映射**（理由写在段首）。
  ② **行数**：`femacs/src/w32term.rs` **11 → 1,592 行**（密度 1.59）；台账 24 → **389 行**（旧"空单元"结论按 P-86 显式推翻）。
  ③ **读数**：冻结 `build-tu` **6 发全 exit 0 / 336 字节**（超预算，如实记）；**11 配置只读扫描表**（`have_ntgui` 单档 42 条落单元正文，
     加 `windowsnt` 起 0 条——但 lane **自证那是空读数**：镜像的 E0428/E0425 在名字解析阶段就中止 crate）；
     **臂触达预检 + 两个变异实测**：注入**类型**错误 → 输出**逐字节不变**（142 block）⇒ **不声称"单元无类型错"**，
     只有解析层的那 1 条（`w32_clip_to_row`，定义在 C 1739 属下一片）是实测；
     `check-cond-eval src/w32term.c` **41 conditions, 2 unknown, 0 disagreements [VERIFIED]**；
     `check-dead-file-coverage` **15 ledger(s) bound / 0 failure**；`--check` **134 行**（133 行是 C 1003-8414 余量）。
  ④ **集成者这一轮的动作**：`femacs/xtask/Cargo.toml` 补两个 feature——`win32_winnt_lt_0500`
     （C 依据 `w32term.c` 98 的 `_WIN32_WINNT < 0x0500 && !defined (MINGW_W64)`）与 `win32_winnt_lt_0601`（C 129，实读核过行号）。
  ⑤ **仍未解决（点名，且这是本轮最大的发现）**：⑴ `include/w32term.rs` 有 **23 个声明与本单元定义重名**（7 函数 + 16 对象）
     ⇒ 必须走 `*_decls`/`*_objs` 模块化，**在此之前没有任何 `have_ntgui` TU 能编这个文件**；
     ⑵ `w32_output` 缺 `paint_buffer_dirty`/`want_paint_buffer` 访问器；`BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT`/`enter_crit!`/`leave_crit!`
     展开到镜像没有的名字；⑶ `dispextern.rs` 缺 `draw_fringe_bitmap_params` 的 `cursor_p`/`overlay_p`、`Mouse_HLInfo.mouse_face_defer`
     与 5 个原型；⑷ `frame.rs` 的 `FRAME_W32_P` 只有冻结臂、`FRAME_MOUSE_UPDATE` 宏体不可编；⑸ `w32gui.rs` 的矩形转换宏宽度不匹配；
     ⑹ `w32.rs` 与 lib 镜像的 `FD_SETSIZE`/`fd_set` 等重名；⑺ **Windows 臂的镜像图从来没被编过**（预检 141 条镜像错点名
     `w32gdiplus.rs` 24 / `w32term.rs` 15 / `frame.rs` 11 个"没人声明"的 SDK 名）——这与 3.313 的 xcb 类同属"镜像从没进过编译面"。
  ⑥ **未声称**：`text-only`（无 mingw/Windows SDK，无 Windows 目标编译）；**无 `<windows.h>` 的任何布局测量**
     （`POINT`/`RECT`/`_TOUCHINPUT`/`W32Msg` 未验证）；无链接、无运行；除扫描表那 11 档 cfg 外没有别的臂被编过；
     C 1003-8,414 只是登记。

- **3.407 最大死文件第二片：`image.c` C 1096-2211（2026-10-05）**：
  ① **区间**：39 条新行替换 `1096-13805`，余量改为 `2212-13805`；收在 C 2211（`image_alloc_image_color` 的 `}`），
     **下一个活定义 = `make_image_cache`（C 2224）**。落点含 `struct image_type`/`IMAGE_TYPE_INIT`、`parse_image_spec`、
     三个 DEFUN（`image-size`/`image-mask-p`/`image-metadata`）、`make_image`/`free_image`/`check_image_size`/
     `prepare_image_for_display`、`image_ascent`/`four_corners_best`、`image_background*`、`CLEAR_IMAGE_*` 与
     `image_clear_image_1`/`_clear_image`/`image_alloc_image_color`。
  ② **行数**：`femacs/src/image.rs` 1,714 → **3,202（+1,488）**；台账 401 → **716**；冻结对象仍 **336 字节 / 同 md5**。
  ③ **读数**：`build-tu` 4 发（超预算 1 发，如实记）；**13 档多配置扫描**（含 `have_ns`/`have_x_windows`/`use_cairo`/
     `have_haiku`/`have_android(±stub)`/`have_ntgui+windowsnt`/`have_pgtk`），逐档报"单元文本 / 镜像"错数；
     `+have_ns+dispextern_face_records` 与 `+have_ntgui+windowsnt` **单元文本 0 条**；`probe`（NS+face_records）
     exit 1 / 8 条**全在镜像**（4 E0428 与镜像 lane 的新块相撞 + 4 条 nsterm.rs 缺名）；**臂触达预检 + 三个变异**
     （类型错 → E0308 命中；改名 → E0425 两处；**`&&`→`||` 的语义变异编译器看不见**——这条限度照实写进报告）。
     `check-dead-file-coverage` **15 ledger(s) bound / 0 failure**；`--check` 的 `- ` 行 **174 → 139**，
     集成者补 8 行 `macro-body-notes` 后复读 **131**；`check-citations`/`check-declarations`/`check-carried` 全绿。
  ④ **本片的一处真收益**：多配置扫描**抓出并修掉 5 处缺陷**，其中 3 处在**上一片**的宏体里
     （三组 `*16_FROM_ULONG` 沿用 C 的 `RED_FROM_ULONG (x)` 少了 `!`——只有本片的 `image_background` 才会展开它，
     上一片三档都看不见）与 1 处 `(*c).buckets[...]` 的原始指针下标。**"多配置扫描"连续两片抓到翻臂看不见的缺陷**，
     这条经验现在是这个项目里可复用的动作（3.401/3.402 已记，本轮再加一证）。
  ⑤ **集成者这一轮的动作**：`macro-body-notes.tsv` 补 **8 行**（`image_error` 的 `va_list/va_start/va_end`＝P-04；
     `parse_image_spec` 的 `switch/case/default/goto/maybe_done`；`image_spec_value` 的 `0,1`；`check_image_size` 的 `1`；
     `four_corners_best` 的 `uninit`（C 1962 实读）；`image_background`/`_transparent`/`image_clear_image_1` 的
     `*_valid` 家族）；`Cargo.toml` 补 **4 个 feature**：`have_rsvg`/`have_tiff`/`have_jpeg`/`have_xpm`
     （C 依据 `image.c` 1360-1361 的条件与 3097/5913/5921 的单独站点）。
  ⑥ **仍未解决（点名）**：⑴ `--check` 余 131 行（61 函数/42 常量/10 defsubr/7 宏/7 subr/2 对象 + 10 行 token/声明 + 行数）；
     ⑵ **4 处 E0428 必须与镜像 lane 的新块同提交删除本单元的 use-site 拷贝**（P-26 的搬运规则；本单元 165-171 保 pgtk、
     186-198+217-220+233+236、245、230）；⑶ `dispextern_face_records` 要进本单元的翻臂 cfg 集；
     ⑷ `include/pgtkterm.rs` 仍是占位、`nsterm.rs` 缺 `NSArray`/`NSString`/`EmacsMenu`；
     ⑸ `frame.rs` 的 `FRAME_IMAGE_CACHE!` 返回 `*mut c_void`、消费方要 `*mut image_cache`。
  ⑦ **未声称**：Windows/Haiku/GTK/Android 臂**仍 text-only**；预检不是行为验证（loader/DEFUN/`free_image` 从未运行）；
     `--check`/扫描/预检都不是语义证明；probe 的 NS 读数**停在名字解析阶段**、没到本单元类型检查。

- **3.408 镜像轮：把四件 P-86 文件翻臂卡住的那批缺口修掉（2026-10-05）**：
  ① **改动（6 个镜像 + 7 份 `include-*.md` 台账，1350 insertions）**：
     ⑴ **`frame.rs`**：`struct frame` 的 flag run 现在按**实测的四臂**落（frozen / A=`not ext_tool_bar && not ext_menu_bar`（x11/pgtk/android/w32）/
     B=`not ext_tool_bar && ext_menu_bar`（haiku、USE_X_TOOLKIT）/ C=`ext_tool_bar`（NS/GTK）），每个 flag 同时给 `FRAME_*_UNIT` 与 `FRAME_*_SHIFT`，
     并新增 `frame_word`/`frame_flag`/`frame_set_flag`/`frame_field` 四个读取器（旧的 `frame_bit1` 只读 `bits1`，在 B 臂会把 `no_special_glyphs`
     读成 `iconified`，已删）；另落 `enum fullscreen_type` + 5 成员、`FRAME_W32_P` 的 `HAVE_NTGUI` 对臂、`MOUSE_HL_INFO` 三臂（含 Android 臂）。
     **编号不是推的**：`spikes/gated-layout-probe.py --bits 'struct frame' frame.h` 新读了 haiku（sizeof 632）与 android（648）两臂，
     android 的 48 行 flag 与 x11 **逐位相同**（生成时逐位断言）。**`have_ntgui` 是拒编**（frame.h 471 的 `#ifndef HAVE_NTGUI` 把 `override_redirect` 从 run 里拿掉，
     本机无 `<windows.h>` 量不了）⇒ A 臂带 `not (feature = "have_ntgui")` + 一条 `compile_error!` 说明该读哪把探针。
     ⑵ **`dispextern.rs`**：window-system 半（三条平台 `gui.h` 边 gated、六份 `Display_Info` 落 5 份、`USE_CAIRO` 的匿名容器带 **clang 实测**
     sizeof 24 / data 8 / bytes_per_line 16 / bits_per_pixel 20 四条断言、NS/haiku/android 三份 `Emacs_Pix_Container`、默认 `Emacs_Pix_Context`）。
     ⑶ **`xterm.rs`**：`x_bitmap_stipple` 声明搬进 `xterm_bitmap_decls` 模块（glob 导入被 crate-root 定义遮蔽 ⇒ x+cairo 臂的 `E0428` 消失，且不需要 ratchet 行）。
     ⑷ **`window.rs`**：`WINDOW_BUFFER_LOCAL_VALUE` 形参序改回 C 的 `(V, W)`（C window.h 631-635 实读）。
     ⑸ **`w32term.rs`**：`FRAME_DISPLAY_INFO` 宏体从 `{ (void) ($f); … }`（**不是合法 Rust**，消费者全挂）改成 `{ let _ = ($f); … }`；
     `w32_kbdhook_active` 从 `pub const` 改成**零元 `macro_rules!`**（C 是 `#define`）。
     ⑹ **`w32gui.rs`**：`XImage` 的门由 `windows` 改成 `have_ntgui`（C 依据：该头无 `#if`，两个 includer 都在 `HAVE_NTGUI` 里）。
     ⑺ **cairo 的判断**：`<cairo.h>` 是参照树里没有的外部头 ⇒ 按 **P-88** 声明留在**单元内**，**不新建** `include/cairo.h`（判断与依据写进 `include-xterm.md`）。
     ⑻ **`pgtkterm.rs` 本轮不能修**（实测：本机无 GTK 头、`pkg-config --exists gtk+-3.0` 为 absent ⇒ PGTK 臂没有任何一臂能被 clang 读到），
     5 个记录留给 PGTK round。
  ② **读数**：`check-mirror-dupes` **18/18/0 未动**；7 个镜像的 `extract-abi --check` **7/7 OK**；**`xtask layout-check` → `layout-check passed`**
     （它就是抓出 `w32gui.rs` 那条边的门）；`check-citations` 675/0 drift/0 stale（`citations-allowlist.tsv` 的 `dispextern.rs:587` 行号刻意未动）；
     `check-declarations` 13/0/0；**P-87 全量扫描两腿 `83/0` + `83/0`**（中途红过一次 `src/sysdep.c`，是 lane 自己引入并修掉的，见 ③）。
  ③ **翻臂前后对比（都是"名字解析级"读数，不是类型检查级）**：`image.rs`+NS：**55 → 11**（`frame.rs` 45 → 0、18 条 `Emacs_Cursor` 消失）；
     `image.rs`+x+cairo：233 → 230（`x_bitmap_stipple` 的 E0428 消失，余为 `xlib.rs` 缺口）；`image.rs`+haiku：29 → 6；
     `image.rs`+android：28 → 6；`image.rs`+pgtk：35 → 12；`haikuterm.rs` 臂触达：**78 → 42**（台账点名的 `no field tooltip`/`FRAME_TOOLTIP_P`/
     `FRAME_AUTO_*_SHIFT`/`FRAME_HAS_VERTICAL_SCROLL_BARS*`/`fontset_from_font` **全部归零**）；`androidterm.rs` 那条带 `have_android` 而不带
     `have_window_system`，故不受本轮编号影响（17 未动），其 blocker 在 `include/androidterm.rs` 的 `bits` 折叠。
     **口径必须一起读**：后四个配置现在**在名字解析阶段就停**（`E0428`），所以 `font.rs` 的 8 条 `E0080` 与 haiku/android 扫描里那些**单元文本**缺陷
     （`!(STRINGP(…) != 0)`、`c_uint` 形参、`foreground_pixel & 0xffffffff`）是**被挡在后面，不是被修好**。
  ④ **新增规则 P-89（来自本轮两次事故）**：`xtask` 的 `materialize` 对嵌套 `include!` **按文件去重、且不看边上的 `#[cfg]`** ⇒
     任何"共享镜像里带门的 include 边"都会把目标文件**唯一的一次包含**挪进死臂（实测两起：`sysdep.rs`×`wchar.rs`、`layout-check`×`lib/msdos.rs`）。
     规则写进 PORTING.md：加带门 include 边前先确认还有无门路径；被咬了就在镜像里绕开并记账；根因在 xtask，是独立工作。
  ⑤ **交接给集成者（下一条提交）**：**单元侧副本必须与镜像同批删除**（P-26 的搬运规则；本提交因 lane 不许碰单元而欠着）——
     `src/image.rs` 155-180（六 `Display_Info` 臂）+ 211-245（容器/上下文对）、`src/frame.rs` 4755-4765 与 `src/w32fns.rs` 1184-1192
     （`fullscreen_type` + `FULLSCREEN_*`）；另 `src/haikuterm.rs` 1423-1441 三处 `WINDOW_BUFFER_LOCAL_VALUE` 调用序与 476 的 `(*f).tooltip`
     要按新镜像对齐。`check_x_display_info` **镜像未落**（加了会让 `check-mirror-dupes` 报与 `frame.rs` 的重复声明，而 ratchet 行在 `docs/baseline/**`），
     故 `src/frame.rs` 6155-6156 的副本**保留**。

- **3.409 3.408 欠的那笔搬运：单元侧副本删掉、调用点对齐（2026-10-05）**：
  ① **背景**：3.408 的镜像轮落了 `dispextern.rs` 的 window-system 半与 `frame.rs` 的 `enum fullscreen_type`，
     但 lane 不许碰单元，于是"同提交删除单元副本"（P-26 的搬运规则）**欠了一笔**——那期间这些平台臂的翻臂
     会停在 `E0428`。这一笔把它补上。
  ② **改动**：⑴ `femacs/src/image.rs` 删掉镜像已拥有的三块——五份 `Display_Info` 臂（x11/w32/NS/haiku/android）、
     `USE_CAIRO` 的 `emacs_pix_container` 记录 + 四条断言、NS/haiku/android 三份 `Emacs_Pix_Container` 与
     默认 `Emacs_Pix_Context` + NS 的 `Emacs_Pixmap`；**保留两份镜像故意不带的**：pgtk 的 `Display_Info`
     （镜像缺）与 w32 的 `Emacs_Pix_Container`/`Emacs_Pix_Context` 对（C 137-139 要未测的 Windows 类型）；
     并把该段开头的注释改成"现在来自 `dispextern.rs`"。
     ⑵ `femacs/src/frame.rs` 与 `femacs/src/w32fns.rs` 各自那一段 `fullscreen_type`/`FULLSCREEN_*` 删掉
     （两处注释本来就写着"镜像拿到 enum 后必须删，否则 E0428"）。
     ⑶ `femacs/src/haikuterm.rs` 三处 `WINDOW_BUFFER_LOCAL_VALUE` 调用点改回 **C 的 `(V, W)` 实参序**（3.408 修了镜像），
     并把 `if (*f).tooltip` 改成 `FRAME_TOOLTIP_P (f)`（C 的 `tooltip` 是 flag run 里的一位，镜像给的是读取器）。
  ③ **读数**：四个单元冻结档 `build-tu` 全部 **built / exit 0**（`frame`/`w32fns`/`haikuterm` 各 336 字节空对象，
     `image` 同样 336 字节）；本提交只删单元侧副本与对齐调用点，**不动任何镜像**，所以 P-87 的全量扫描在批末统一跑。
  ④ **未声称**：这些平台臂的翻臂读数仍是**名字解析级**（3.408 已记：`font.rs` 的 `E0080` 与 haiku/android 的单元文本缺陷
     在被挡住的后面）；本笔**不改**那些，只把 3.408 欠的搬运补齐。`check_x_display_info` 的镜像条目仍未落
     （会让 `check-mirror-dupes` 报与 `frame.rs` 的重复），故 `frame.rs` 的副本按 3.408 的说明**保留**。

- **3.410 本批收口：棘轮 86 文件 / 195,581 行（0 stale）、port-map 57.0%、P-87 两腿 83/0（2026-10-05）**：
  ① **P-87 全量扫描**（覆盖 3.408 的六处镜像改动 + 3.409 的四处单元改动）：`checked 83 unit(s), 0 failure(s)`
     + `checking 83 unit(s), 0 unit failure(s)`。
  ② **棘轮**：`--write-pending` → **86 文件 / 195,581 行（0 new, 0 stale）**；本批入口 87/203,995 ⇒ **净 −1 文件 / −8,414 参照行**
     （`w32term.c` 退出 `<10%` 类）。
  ③ **进度表**：`gen-port-map.py` → **386,588 / 678,320 = 57.0%**（本批入口 56.3%、上批收口 55.5%）。
  ④ **门**：`check-mirror-dupes` **18/18/0**、`check-declarations` **13/0/0**、`check-citations` **675/0 drift/0 stale**、
     `check-dead-file-coverage` **15 ledger(s) / 0 failure**。
  ⑤ **本批五件（3.405-3.409）**：xdisp 第 30 轮（C 25077-25863，11 定义，+866 行）、`w32term.c` 第一片（C 1-1002，+1,581 行）、
     `image.c` 第二片（C 1096-2211，+1,488 行）、**镜像轮**（6 个镜像 + 7 份台账，把四件 P-86 文件翻臂卡住的缺口修掉：
     `frame.rs` 的四臂 flag run（实测编号）+ `dispextern.rs` 的 window-system 半 + `xterm.rs` 的 `x_bitmap_stipple` 模块化 +
     `window.rs` 的形参序 + `w32term.rs` 的两处拼写 + `w32gui.rs` 的门）、以及**补齐 3.408 欠的搬运**（单元侧副本删除 + 调用点对齐）
     ——合计约 **+2,800 行端口文本**，另加约 1,400 行镜像与台账。
  ⑥ **本批两条规则/经验**：⑴ **P-89**——`xtask` 的 `materialize` 按文件去重嵌套 `include!` 且不看边上的 `#[cfg]`，
     带门 include 边会把目标文件的唯一一次包含挪进死臂（实测两起）；⑵ **多配置只读扫描**连续三片抓到翻臂看不见的缺陷
     （3.401 三处、3.402 一处、3.407 三处在**上一片**的宏体里），已是这个项目里可复用的动作。
  ⑦ **未声称**：两腿都是类型检查级；棘轮量参照行数、port-map 的 57.0% 是行数比；§8 八条里仍只有第 5 条有判据与读数；
     镜像轮的翻臂对比是**名字解析级**（`font.rs` 的 `E0080` 与 haiku/android 的单元文本缺陷在被挡住的后面）。

- **3.411 xdisp 第三十一轮：`display_line` 整函数（C 25864-26935，1,072 行）（2026-10-05）**：
  ① **区间**：整函数一次落完，C 内**无任何 `#if`**（唯一预处理行是 C 26934 的 `#undef RECORD_MAX_MIN_POS`、无 Rust 位点）；
     随函数落地 `RECORD_MAX_MIN_POS`（C 26105-26134，函数内 `do{…}while(false)` 语句宏 → 函数作用域 `macro_rules!`）、
     `back_to_wrap:`/`at_end_of_line:` 两个标号与三个 `goto`。覆盖区间现为 **C 468-26935**；
     **下一个活定义 = C 26937 `current-bidi-paragraph-direction`（DEFUN）**。
  ② **行数**：`femacs/src/xdisp.rs` 24,435 → **25,692（+1,257）**；台账 3,220 → **3,364**；
     勾选 `Functions 249/302`、合计 **345/415**；scaffold 12 → 11。
  ③ **读数**：`build-tu` 3 发（第 3 发绿；如实记）→ 对象 **2,661,656 字节**（上轮 2,358,272），警告仍同样六条；
     **`--check` `^- ` 77 → 77（退 `display_line` 与 `RECORD_MAX_MIN_POS` 两条 missing，进两条 `does not mention`）**，
     集成者补 **2 行** `macro-body-notes` 后复读 **75**；**`--cfg enable_checking` 探针 exit 0 / 0 error**（本轮 5 条 `eassert!` 真的被编到）；
     `nm` 新增 4 个 `t` 符号（`display_line` + 因此可达的 `find_row_edges`/`unproduce_glyphs`/`row_text_area_empty`）与 1 条
     `U _prepare_desired_row`；`check-citations` 675/0/0、`check-declarations` 13/0/0、`check-carried` 11/11/0、
     `check-ledger-ticks` 235/0。报告里的两条红（`check-dead-file-coverage` 2 failure）**落在别人在飞台账 `docs/ports/android.md`**。
  ④ **集成者动作**：补 `macro-body-notes.tsv` 两行——`display_line`（`goto,uninit`：C 26331/26368 的两个 `goto` + C 26415 标号
     → 旗标 + `break 'end_of_continued_line`；C 25870-25883 的 8 个 `UNINIT` → 零值，PORTING 1.5/2.2）与
     `RECORD_MAX_MIN_POS`（`false`：`do{…}while(false)` 的收尾条件在 `macro_rules!` 里无对应物）。
  ⑤ **未解决（点名）**：`prepare_desired_row`（dispextern.h 3917）应进镜像链（本单元按 P-72 规则 3 自持）；
     台账待办 78 记下一条**烧掉的 scaffold 条目引用写错**（写的是 `dispextern.h 3595`，C 自己的前向声明是 **C 1167**）；
     待办 79 记下 `dispextern.rs` 里 `dispextern_face_records` 门后的 `struct it` 与单元自有 `[u8; 12]` 臂**一旦开该门要一起重裁**。
  ⑥ **未声称**：只声称"文本 1:1 + 冻结配置编过 + `enable_checking` 臂编过"；**不声称任何运行/显示正确性**
     （没跑过重显示、没生成过 glyph matrix、`eassert!` 只被编译未求值）；两个 `goto` 的改写只声称"按读检查的语义等价"。

- **3.412 P-86 第七件：`android.c` 第一片 C 1-1037（2026-10-05）**：
  ① **区间**：收在 `android_select` 的闭括号；**下一个活定义 = C 1041 `android_run_debug_thread`（C 1041-1078）**。
     落点 **13 个函数**（三个 thread-events 初始化/终结、`android_get_poll_thread`、`android_set_task_name`、
     `android_run_select_thread`、`android_handle_poll_signal`、`android_init_events`、`android_pending`、
     `android_wait_event`/`android_next_event`/`android_check_if_event`/`android_write_event`、`android_select`）、
     **9 个 struct**（7 个 class 记录 + 三个事件队列记录）、约 40 个文件级对象与 `SELECT_SIGNAL`/`INT_STRLEN_BOUND` 等宏；
     台账 28 行区间表连续铺满 C 1-1,037 + 余量行。
  ② **行数**：`femacs/src/android.rs` **11 → 1,311 行**；台账 24 → **345 行**。
  ③ **读数（四档，分得很清）**：⑴ 冻结 `build-tu` 3 发都 exit 0 / **336 字节空对象**；⑵ **翻臂**（`have_android`+`not_android_stubify`）
     **exit 1，只有 1 条错，且在镜像副本**（`lib/include/signal.rs` 的 `sigaction` 是冻结 Darwin 形、缺 bionic 的 `sa_sigaction`，报在单元 C 749）——
     **单元文本 0 条**；⑶ 追加子门 `+ android_api_lt_21`（`<` 那一臂不碰 `sa_sigaction`）→ **exit 0 / 5,232 字节，未打任何补丁**；
     ⑷ **臂触达预检**（只补 `signal.rs` 的 `sa_sigaction` 一类）→ 六配置全 exit 0（23,888…37,968 字节）+ **变异实测**证明非空读数。
     **多配置扫描**：base/lt21/thr/lt21+thr/aarch64/i386 六档，只有上述那一处镜像缺口。
     NDK 量测全部实测（NDK 29.0.14206865 / aarch64-linux-android24）：mutex 40、cond 48、`sem_t` 16、`sigaction` 32（`sa_flags`@0/`sa_mask`@16）、
     `siginfo_t` 128、`sigset_t` 8、`SA_SIGINFO 0x4`、`SIGUSR1 10`、`SIGIO 29`、`ANDROID_LOG_INFO/WARN/FATAL 4/5/7` 等。
  ④ **集成者这一轮的动作**：⑴ `Cargo.toml` 补 feature **`android_api_lt_21`**（我实读核过行号：C **972** 是 `#if __ANDROID_API__ >= 21`，
     所以这个名字是**另一臂**的端口名，注释按实读改写）；⑵ `macro-body-notes.tsv` 补 **1 行**——`android_write_event` 的
     `switch/case/default/break`（C 868-909 的 switch 落成 `match`）。**刻意不为 `android_set_task_name` 加行**（见 ⑤）。
  ⑤ **本轮抓到一条工具缺陷（照实记，不用豁免掩盖）**：`extract-abi.py` 把 C 470-471 的字面量 `"/proc/self/task//comm"` 里的 `//`
     当成注释起点，`c_body_text` 于是把该行后段挖空、引号不闭合，产出假阳性
     `function android_set_task_name does not mention comm, failed, proc, self, set, task`（lane 已实测复现）。
     ⇒ **那一行留给工具修**（修在 `extract-abi.py` 的扫描器里），不是加 notes 行；本条与 3.404 的"验证表陈旧"是两条独立的工具工作单。
  ⑥ **未解决（点名）**：`lib/include/signal.rs` 的 bionic `sa_sigaction`（镜像轮，需 `check-all-units.sh`）；
     本单元内那批外部头声明（`<pthread.h>`/`<semaphore.h>`/`gettid`/`pipe2`/`kill`/`sig*`/`__android_log_print` 等，P-88 就地声明）日后搬进镜像；
     `line-drift-tolerance` 的 `android` 行留到末片。
  ⑦ **未声称**：本片每一行都挂在冻结配置为假的门下——④ 是**打了补丁的替代读数**，②③ 都是**编译级**，
     **没有执行过任何东西**（无线程、无信号量、无 JNI、无事件、无 select）；C 1038-7,594 未移植。

- **3.413 版本声明这条长期指令的第四半：写出身份行的两个工具，和它们落进的豁免树（2026-10-05）**：
  ① **指令的读法不变**（3.90/3.134/3.226/3.250/3.269/3.295/3.318/3.340）：`版本声明不要再说是gnu emacs了`
     与 38 秒之后的 `把之前的代码中的这种声明也去掉` 是同一条指令的前后两半——不只对新写的文件生效，
     **已写进树的产出物也要清**。本轮不改口径，改按**产出物**（而不是按后缀）再复算一遍：
     端口树（`femacs/` 的 `src`/`lib`/`lib-src`/`exec`/`lwlib`/`oldXMenu`/`nt`/`msdos`/`platform`/`xtask`）
     与 `docs/ports/**` 的 **682 + 602 个文件，`decl`/`version`/`title`/`recital` 四类 0 命中**；
     许可短语（`is part of …`/`is free software`/`along with …`/`Free Software Foundation`）在全仓
     （除逐字节复制树）只出现在**门自己的自测例**与 `docs/status.md` 的引文里。
  ② **本轮量的真洞：两处金标头部，写在"整棵树被豁免"的文件里**。`docs/baseline/l{0,1}-reference.txt`
     的 `# version:` 行是 `--golden` 用参照 `--version` 的第一行**原文**写的（3.261/3.265 记的就是
     `GNU Emacs 32.0.50`）；这两个文件是 `.txt`、不在 `PROSE_SUFFIXES` 里，而 `docs/baseline/` 按
     "冻结参照的实测值"整棵豁免 ⇒ **这一行对每一道门都不可见**，重新记录就会把产品名原样写回来。
     这与 3.250（`.swift` 只按名字被扫）、3.295（`.s` 的 36 行声明）是**同一类失败模式**：门声称读了，
     其实一个文件都没读。
  ③ **处置（工具 + 数据 + 门，一处不漏）**：⑴ `tools/acceptance-l{0,1}.py` 增 `version_number()`
     （取 `--version` 末段=版本号）与 `version_header_lines()`（在头部写明为什么不重复那个名字），
     `--golden`/`--check` 的打印同步换成版本号；⑵ 两个金标头部重写——L0 用 `--golden` **重新生成**，
     L1 手工改后用**同一函数**回算比对（逐字节相同）；⑶ `check-declarations.py` 新增 `BASELINE_PROSE`，
     **按名字**读这两个文件的**头部块**（`header_only()`：遇第一条非 `#` 行即停，记录部分仍归豁免），
     并把这条读法打进覆盖行。
  ④ **读数**：`--selftest` **46 例 / 0 失败**（新增 3 例：旧写法=`title`、新写法=无、**记录里出现产品名
     不算我们的散文**）；全量扫描 **13 declaration sites（0 forbidden / 13 allowed literals），0 failures**，
     新覆盖行 `read 2 baseline header(s) by name …` 照打。**变异实测**：把 `# version: 32.0.50` 改回
     `GNU Emacs 32.0.50` ⇒ `FAIL docs/baseline/l0-reference.txt: title x1`、**exit 1**、读数变
     `14 declaration sites (1 forbidden …), 1 failures`；还原后回到 0 ⇒ 这条规则**写不回旧写法了**。
  ⑤ **顺带量到的一条（照实记，不掩盖）**：L0 金标记的 `sha256[0:16]` 是 `ea7514dca1b463f5`，而当前
     `build/emacs-ref/src/emacs` 是 `a959f1d7a8bb4e11`（= L1 在 3.265 记的值）⇒ **L0 金标是对更早一次构建
     记录的**。`--golden` 重跑后 `diff` **只有头部 3 行**：19 条表达式记录（exit/stdout/stderr）**逐字节未变**，
     两条读数同时成立——记录值现在有效，而身份行此前指的不是当前这个二进制；现在一致了。
  ⑥ **仍未解决（点名，且这是留给用户的裁决）**：`docs/baseline/config/darwin-arm64.macros.tsv` 的
     `PACKAGE_NAME "GNU Emacs"`/`PACKAGE_STRING` 是**实测值**（PORTING.md 0.1-19 明确"必须原样保留"）；
     `docs/acceptance.md`、`docs/status.md` 里是**参照自己的输出与用户指令的引文**；端口单元里 **13 条字面量**
     是 C 的 `PACKAGE_NAME`、`RCS_Id`、etags/ebrowse 版本横幅与 4 条 docstring ⇒ **移植语义**，docstring 门
     与 L0 对拍要求它们与参照逐字一致，改它们等于让端口与参照在 `--version` 上分叉。
     ⇒ 是否在**最终验收阶段**把端口的 `--version`/横幅改成不主张身份（代价：L0/L5 与 docstring 门要改成
     "除身份行外一致"），需要用户一句话；本轮**不擅自改语义**。
  ⑦ **未声称**：本轮只扫"我们写下的东西"并修掉其中的身份行，**没有**动逐字节复制树（`lisp/`/`test/`/`doc/`/
     `etc/`/`admin/`/`leim/`——它们就是参照的原文，产品名在原文里）；`docs/baseline/` 其余实测表未动；
     ⑥ 里的字面量与那两处引文**仍在原处**。

- **3.414 集成者这一轮：mirror-dupes 那道红门、以及 `blank_comments` 的字符串洞（2026-10-05）**：
  ① **范围**：本轮做的是 3.412 台账里点名"欠集成者"的几项，加上当时正红的一道门。
  ② **红门（`check-mirror-dupes` 1 FAIL）的处置，以及为什么不按 P-26 删声明**：`femacs/src/android.rs`
     的 P-88 段声明了 bionic 的 `pipe2`，而 `femacs/lib/include/unistd.in.rs` 里也有这个名字（冻结配置的
     `# undef pipe2 / # define pipe2 rpl_pipe2` 落成 `macro_rules! pipe2`）。**删掉单元侧声明会把 Android 臂
     悄悄改成"调 gnulib 的替换实现"**——C 的 android.c 在 Android 配置下没有替换（`@REPLACE_PIPE2@` 只对
     冻结的 Darwin 为 1），那是"发明 C 里没有的行为"（硬约束 1）。证据是实测的：bionic 的 `<unistd.h>`
     299 行 `int pipe2 (int[2], int)`（NDK 25.1/26.1/27.0/28.2 四个版本一致）。⇒ 按树里既有的同类判据
     （`sysdep.rs` 那行）**加棘轮行**，理由写清"这是两个配置各自的臂，不是一份声明被抄了两遍"。
     **读数：`19 duplicate declaration(s), 19 ratchet row(s), 0 failure(s)`**（此前 `19/18/1`）。
  ③ **真洞：`tools/cinventory.py` 的 `blank_comments` 不认识字符串字面量**。它在 code 态只看 `/*`/`//`，
     于是 C 里 `"/proc/self/task//comm"`（android.c 470-471）的 `//` 被当成注释起点，**该行后半被挖空**，
     `android_set_task_name` 的参照体因此丢掉 `comm/failed/proc/self/set/task` 六个 token，`extract-abi --check`
     报出假阳性（3.412 ⑤ 已实测复现，当时留着"给工具修"）。本轮修在扫描器里：**字符串与字符字面量都建模**
     （字符串按闭合引号走，转义成对吃掉，反斜杠续行的字符串不跨行断；字符字面量按 `strip_char_literals`
     既有的"几个字符就结束"上限，免得一个落单的撇号吞掉整行）。**假阳性消失**，取代它的是真finding
     `- function android_set_task_name does not mention goto`。
  ④ **同一条洞的第二个实例（本轮新发现，影响另一个文件）**：`emacs/src/w32fns.c` 4127 的
     `strchr ("0123456789/*-+.,", code_unit)` 里那个 `/*` 让老扫描器**进入了块注释态**，把 **C 4127-4139
     全部挖空**——那是 `deliver_wm_chars` 的收尾和 `#ifdef DBG_WM_CHARS` 两臂的 `FPRINTF_WM_CHARS` 定义。
     修好后 `w32fns` 的清单多出 `FPRINTF_WM_CHARS` 这条宏、并恢复若干函数（`deliver_wm_chars` 等）的可见性。
  ⑤ **完整性读数（谁被这条洞影响，一次量清）**：`emacs/` 下 **634 个 `.c/.h`** 逐个比对新旧 blanking，
     **12 个文件**的读数变了，**最大 13 行**（w32fns.c）——没有"某个状态泄漏、吞掉几百行"的情形：
     `w32fns.c`13、`androidvfs.c`6、`sfnt.c`5、`image.c`3、`exec/trace.c`3、`android.c`2、
     `lib-src/make-docfile.c`2、`xterm.c`/`gtkutil.c`/`fileio.c`/`emacs.c`/`comp.c` 各 1。
  ⑥ **`extract-abi --all` 前后对拍（同一天两次全量，2 分钟一次）**：`checked 590 ported files:
     582 done, 2 in progress, 6 failing` **两侧一致**；**差异只有两处**：`android.c` 的假阳性变成真 `goto`、
     `w32fns.c` 的函数/宏清单按 ④ 变多。⇒ 扫描器修复**没有**在其他 588 个文件上改变任何读数。
     ⑥ 里那 6 个 failing 是**切片中的 P-86 单元**（android 11/7,594、androidterm 1,055/7,117、
     haikuterm 1,554/4,898、image 3,134/13,805〔在飞〕、w32fns 1,548/12,488、w32term 1,592/8,414），
     它们报的是"还没移植的函数"，不是缺陷；2 个 WIP 是 xdisp/xterm 的未勾选清单。**验收口径不变**：
     完成定义要求的是全部落完后 `--all` 零差异。
  ⑦ **本轮落地的行**：`docs/ports/macro-body-notes.tsv` 补 `android_set_task_name → goto`
     （C 481/486 的两处 `goto failure` 与 C 491 的 `failure:` 标号，端口是 `'failure:` 标号块 +
     `break 'failure`，PORTING.md 2.2），并把"为什么这行 3.413 之前量不出来"写进该行；
     `docs/ports/android.md` 的"Owed by the integrator"逐条标状态（1/2/5 已做、4 有意延到批次收口、
     `pipe2` 一项按 ② 结案）；`docs/baseline/mirror-dupes.tsv` 加 ② 那行。
  ⑧ **新欠（点名，给 w32fns 的下一片）**：④ 恢复的 **C 4127-4139** 需要进 w32fns 的对照面——
     `FPRINTF_WM_CHARS` 两臂与 `deliver_wm_chars` 的收尾；`docs/ports/w32fns.md` 的区间表要按恢复后的文本
     重核（`check-dead-file-coverage.py` 现在 `17 ledger(s) bound, 0 failure(s)`，尚无该文件的表）。
  ⑨ **未声称**：本轮**没有编译**（只跑文本门：`check-mirror-dupes`、`extract-abi --check/--all`、
     `check-declarations`、`check-dead-file-coverage`）；Android 臂的两处配置**没有跑过**；②的棘轮行是一条
     **带证据的判断**（bionic 头实测 + android.c 两处调用点 + 冻结配置的替换值），不是"语义已验"。

- **3.416 `image.c` 第三片（C 2212-3550）：图像缓存、`postprocess_image`、缩放与变换（2026-10-05）**：
  ① **区间与块**：C 2212-3550（收在 C 3550 的 `#endif /* HAVE_IMAGEMAGICK || HAVE_NATIVE_TRANSFORMS */`；
     **下一个活定义 = `lookup_image`（C 3556）**）。块：`make_image_cache`/`search_image_cache`/`filter_image_spec`/
     `uncache_image`/`free_image_cache`/`clear_image_cache`/`clear_image_caches`/`DEFUN clear-image-cache`/
     `image_size_in_bytes`/`image_frame_cache_size`/`DEFUN image-flush`/`postprocess_image`/`scale_image_size`/
     `image_get_dimension`/`image_compute_scale`/`compute_image_size`/`matrix3x3`（三处助手 + 四个 Android 矩阵助手）/
     `image_set_transform`。
  ② **行数**：`femacs/src/image.rs` 3,134 → **4,881**（+1,754/−7；本片 Rust 文本 1,719 行对应 C 1,337 行）；
     `docs/ports/image.md` 717 → **1,044**。
  ③ **读数（lane 的命令原文）**：⑴ 冻结 `build-tu src/image.c` **3 发全 exit 0**、对象 336 字节、
     md5 `dbed68837694579185377dbb0821ab62`（与前两片逐字节相同）；⑵ **24 个 cfg 组合**的只读扫描：
     **本片自身文本 0 个单元文本错**，残留全在第 1/2 片或镜像缺口（`+have_ns` 14 行/28 站点/单元 3/镜像 25、
     `+have_x_windows` 47/81/29/52、`+have_android+not_android_stubify` 10/18/5/13、`+have_xrender` 50/84/31/53）；
     ⑶ **arm-reach 预检**（`have_window_system+have_ns+have_native_transforms+dispextern_face_records`，
     只补 4 个镜像替身）**exit 0 / 0 error / 96,584 字节**；⑷ **变异实测 3 条**（注入类型错→exit 1 且错在变异行；
     改本片 `DBL_MAX` 名→exit 1 两处；`postprocess_image` 的 `&&`→`||` **exit 0、编译器看不见，如实记**）；
     ⑸ `extract-abi --check` **139 → 121** 行，其中 2 条是本片定义产生的 token 行 ⇒ **集成者补
     `macro-body-notes.tsv` 两行后 119**（`clear_image_cache` 的 `inhibit_clear_image_cache,max`——位域访问器与
     `f64` 没有 `Ord` 而写三目；`image_compute_scale` 的 `goto,invalid_value`——标号块）。
  ④ **本片抓到并修掉 8 处真缺陷**：4 处在自写文本（`c_float` 不在 prelude、`inhibit_clear_image_cache` 要走 P-80
     位段、`std::cmp::max` 对 `f64` 非法、`scale_image_size` 少 `as c_int`、少一层括号的解析错、未初始化绑定
     `&raw mut` 的 `E0381`、`FOR_EACH_FRAME!` 不能写不可变形参、Android 四个矩阵助手少外层门），
     4 处在**既有文本**——其中 `free_image` 的 `FRAME_IMAGE_CACHE!` 缺 `as *mut image_cache` 强转**每个窗口系统
     配置都报 `E0308`**（属第 1/2 片遗留）。
  ⑤ **镜像缺口点名**：`include/frame.rs` 的 `image_cache` 成员仍是 `*mut c_void`（按 P-70 应为 `*mut image_cache`）、
     `FRAME_WINDOW_P` 的 body 恒 false（语义缺口）、`lisp.rs` 的 `sxhash`/`plist_get`、`float.in.rs` 的 `DBL_MAX`、
     `math.rs` 的 `sinf`/`cosf`、`nsterm.rs` 的 `NSArray`/`NSString`/`EmacsMenu`、`xlib.rs` 的 XRender 面。
  ⑥ **欠（点名）**：`have_native_transforms` 与 `have_rsvg` **还没进 `femacs/xtask/Cargo.toml`**（本片两条门只能
     由 `--cfg` 直接给）；`docs/ports/include-xlib.md` 仍无页（xlib 的偏离记在邻页）。
  ⑦ **未声称**：只声称"文本 1:1 + 该冻结配置下编过 + 四个臂的预检编过"；**没有任何运行/显示证据**——
     没跑过重显示、没生成过 glyph matrix、没做过视觉/差分对拍；C 3551-13,805 未移植。

- **3.417 镜像轮（第二批）：w32term 的 27 个重名归位，xlib 的 Xlib 面，dispextern/frame 的缺口（2026-10-05）**：
  ① **落地内容**：`include/w32term.rs` 的 **27 个重名**搬进 `pub mod w32term_fn_decls` / `w32term_objs`
     （`use super::*;` + `use …::*;`，即 `window.rs` 302-367 / `androidterm.rs` 513-537 的形状，P-21），
     并连带修掉 `w32fns` 需要的 4 条；`include/xlib.rs` **+475 行**补 Xlib 的记录与过程；`include/dispextern.rs`
     落 2 个原型 + 位域访问器；`include/frame.rs` 的 `FRAME_MOUSE_UPDATE` 修好；`w32gui`/`w32`/`w32common`
     三处搬家；台账 7 份同步（`include-{w32term,w32,w32common,w32gui,dispextern,frame,xterm}.md`）。
  ② **翻臂读数（lane 的原文命令，写进台账）**：
     `xtask probe src/w32term.rs … --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'` →
     **13 行 error / 26 个 `E0428` 之前，12 行 / 0 个 `E0428` 之后**；
     `xtask probe src/w32fns.rs` 同配置 → **15 行 / 12 个 → 7 行 / 0 个**；
     `cargo run -p xtask -- layout-check` → `layout probe ok` / `layout-check passed`。
  ③ **集成者本轮复跑（独立读数）**：`tools/check-all-units.sh` → **`checked 83 unit(s), 0 failure(s)` +
     `checking 83 unit(s), 0 unit failure(s)`**（P-87 要求的那条；两腿都是 0 失败）；文本门全绿——
     `check-mirror-dupes` **19/19/0**、`check-declarations` **13 sites / 0 forbidden / 0 failures**、
     `check-citations` **675 文件 / 0 drift / 0 stale**、`check-ledger-ticks` **235 / 0**、
     `check-dead-file-coverage` **17 ledger / 0 failure**、`check-carried` **11/11/0**、
     `check-target-names` **628 .rs / 591 目标 / 0 finding**、`check-swift-units` **0 new / 0 stale**、
     `check-port-lines` **85 文件 / 187,987 行 / 85 棘轮行 / 0 new / 0 stale**。
  ④ **仍未解决（点名）**：`include/dispextern.rs` 的 **4 个原型**与 `check_x_display_info`（加了会让
     `check-mirror-dupes` 与 `frame.rs` 重复，ratchet 行不在本 lane 的白名单里）；
     `include/xlib.rs` 的 **`XEvent` 的 33 个成员**；`include/frame.rs` 的 `FRAME_RES_*` 的 WS 臂与
     `HAVE_NTGUI` 旗标段的 `compile_error!`（两种写法都量过，这是树的既有决定）；
     **`docs/ports/include-xlib.md` 仍无页**（xlib 的偏离记在邻页：`include-dispextern.md` 201/1174、
     `include-xterm.md` 188/196、`image.md` 187/279/1032）。
  ⑤ **未声称**：② 是 lane 的读数（命令原文在台账里，我复跑的是 ③ 的 83/0 两腿与文本门）；本轮
     **没有任何运行证据**——两腿都是编译级。

- **3.418 本批收口：地图重生成、棘轮清行、以及一次磁盘满造成的假读数（2026-10-05）**：
  ① **`gen-port-map.py` 重跑（8 行变化）**：`src/dispextern.h` 3,854 → 3,934、`src/frame.h` 1,855 → 1,885、
     `src/xlib.h`…（镜像轮）与 `src/image.c` 端的 `femacs/src/image.rs` 3,134 → **4,881**、`src/android.c`
     11 → **1,311**、`src/xdisp.c` 24,435 → **25,692**（3.415 已修的那两行这次是合并后的最终读数）。
  ② **`check-port-lines.py --write-pending`**：`src/android.c` 的棘轮行（切片后已不再 <10%）退出 ⇒
     **85 文件 / 187,987 行 / 85 棘轮行 / 0 new / 0 stale**。
  ③ **一次磁盘满，如实记（不是代码读数）**：本批第一次 `tools/check-all-units.sh` 有 **30 个单元**
     报 `No space left on device (os error 28)` —— 卷当时 100% 满（可用 648 MiB）。清理**本项目自己的**
     编译产物后（`build/femacs/*.o` 697 个 + `build/femacs/units/` + `build/probes/` + `build/dead-arms/`，
     另加本轮 /tmp 的 6 个本项目目录与两个临时 worktree），重跑得到真读数
     **83/0（冻结）+ 83/0（checking）**。卷上其余占用是**别的项目的 /tmp 缓存**（约 23 GB），按"只清本项目"
     的规矩未动；当前可用约 1.3 GiB。⇒ 下一次大批编译前先看 `df`。
  ④ **本批门读数汇总（集成者复跑，全部原文）**：`check-declarations` **13 sites / 0 forbidden / 0 failures**；
     `check-mirror-dupes` **19 duplicate / 19 ratchet / 0 failure**；`check-ledger-ticks` **235 / 0**；
     `check-dead-file-coverage` **17 / 0**；`check-carried` **11 carried / 11 row / 0**；`check-verbatim`
     **6 树 / 4,604 文件 / 0 failure**；`check-citations` **675 / 0 drift / 0 stale**；`check-target-names`
     **591 目标 / 0 finding**；`check-swift-units` **0 new / 0 stale**；`extract-abi --all`
     **`582 done, 2 in progress, 6 failing`**（那 6 个是切片中的 P-86 单元，验收要求的是全部落完后零差异）。
  ⑤ **下一步（点名）**：`image.c` 第四片 = `lookup_image`（C 3556）；`w32fns.c` 的 **C 4127-4139**（3.414 恢复
     的文本）要重核进它的对照面与区间表；`android.c` 第二片（C 1038 起）；`include/signal.rs` 的 bionic
     `sa_sigaction`（镜像轮）；`docs/ports/include-xlib.md` 建页。
  ⑥ **未声称**：本批只点亮"文本 1:1 + 冻结/检查两腿编过 + 文本门全绿"；加载阶梯（L0-L5）、双向兼容、
     生成物逐字节、原样树 `diff -r`、平台冒烟与性能**一条都还没点亮**。

- **3.419 集成侧欠账：`have_native_transforms` 入表、xdisp 的三条 `does not mention` 收口、xlib 台账建页（2026-10-05）**：
  ① **范围**：3.416/3.417 两份台账点名"欠集成者"的条目，加上一份**没人点过**的欠账（②）。
  ② **`femacs/xtask/Cargo.toml` 补 `have_native_transforms`**（3.416 的 Owed 10）：image.c 2671/2777 的
     `#if defined (HAVE_IMAGEMAGICK) || defined (HAVE_NATIVE_TRANSFORMS)` 两臂与它们门住的
     `compute_image_size`/`image_set_transform` 段此前只能用 `--cfg` 直接给。**读数**：
     `cargo metadata --no-deps` → xtask **590 个 feature**、`have_native_transforms` 在表内；
     `tools/feature-coverage.py` 照常跑完（`enabled:` 行未变）。
  ③ **xdisp 的三条 `does not mention`（本轮新查出的欠账，此前无人点名）**：
     `redisplay_internal`（`end_of_redisplay,goto,modiff,save_modiff`）、`set_cursor_from_row`（`goto`）、
     `redisplay_window`（`case,debug_method_add,default,goto,if_debug,switch`）。逐条读 C 与端口后写成
     `macro-body-notes.tsv` 三行，理由全部有 C 行号：C 17792 的 `goto end_of_redisplay` 与 C 18221 的标号是
     端口的 `break 'retry`；C 17530 的 `(SAVE_MODIFF < MODIFF)` 写成两个宏的展开
     `BUF_SAVE_MODIFF!`/`BUF_MODIFF!`（P-15）；C 19077 的 `goto compute_x` 与 C 19123 的标号是
     `'compute_x:` 标号块；C 20954/C 21136 的两个 `switch` 是端口的两个 `match`、六个 `goto` 是标号块；
     `if_debug`/`debug_method_add` 来自 C 21077 的 `IF_DEBUG (debug_method_add (w, "1"))`（调试专用语句，
     文本对 token 门是活的，端口按 P-08 记 GLYPH_DEBUG 臂为死）。**读数**：
     `extract-abi --check src/xdisp.c src/xdisp.rs` 的 `^- ` 行 **77 → 72**、`does not mention` **0 条**
     （另外 2 条 `display_line`/`RECORD_MAX_MIN_POS` 的行此前已在该文件里）。
  ④ **`docs/ports/include-xlib.md` 建页**：3.417 的 xlib 镜像（+475 行）此前无页，而 `xlib.rs` 自己的头注释
     引用了"本台账的 Evidence"。**先说清一条前提**：无页**不是**违反约定——`femacs/{src,lib}/include` 有
     **234 个镜像**、`docs/ports/include-*.md` **251 页**，最大的几个镜像（`lisp.rs`/`config.rs`/`globals.rs`/
     `jni.rs`）都没有页；建页只是让那句引用成立并给测量一个家。**读数（本轮用 clang 重新量的，不是抄的）**：
     `XRectangle` 8/2、`XExtData` 32/8、`Visual` 56/8、`Depth` 16/8、`Screen` 128/8、`XImage` 136/8
     （`data`@16、`f`@88）、`XColor` 16/8（`pixel`@0）、`XModifierKeymap` 16/8、`XWMHints` 56/8、
     `XVisualInfo` 64/8（`visualid`@8）、`XEvent` 192/8、`XrmValue` 16/8；`ZPixmap`=2、`BitmapSuccess`=0、
     `StaticGray`=0、`TrueColor`=4（命令原文进页）。页里同时点名两条欠账：**`XEvent` 的 33 个成员**、
     **XRender 面**（`have_x_windows+have_xrender` 的 text-only 类）。
  ⑤ **一条本轮的裁决（写下来，避免下一轮重复讨论）**：`lib/include/signal.rs` 的 bionic `sa_sigaction`
     **不加**。理由：镜像承载的是**冻结配置的形状**，android 台账自己把 Darwin 与 bionic 的差异逐条记档
     （`sigset_t` 4/8、`sigaction` 16/32、`SIGUSR1`/`SIGIO` 值、`fd_set`/`FD_*`、`errno`），因此平台臂的读数是
     **打了补丁的预检**（P-86 ⑦ 的既有口径）；把 bionic 成员塞进冻结镜像会同时推翻它的
     `const _` 冻结布局断言，并让"镜像=冻结形状"这条判据失效。要让平台臂免补丁，正确做法是给该平台一个
     **自己的形状镜像**（例如 `signal_android.rs`）或按 P-88 在单元内就地声明——那是一件独立工作。
  ⑥ **在飞（本轮派出，尚无读数）**：`android.c` 切片 2（自 C 1041 `android_run_debug_thread`）、
     `w32term.c` 切片 2（自 C 1005）、`xfns.c` 切片 1（C 1 起，新建 `docs/ports/xfns.md`）。
  ⑦ **未声称**：本轮**没有编译**（只跑文本门 + `cargo metadata`）；③ 是**读 + 加行**，`does not mention`
     归零只说明"行与 finding 对应"，不说明那些 C 语句的语义已被差分验证。

- **3.420 xdisp 第三十二轮：`current-bidi-paragraph-direction`（C 26937-27028，一个 DEFUN）（2026-10-05）**：
  ① **区间**：C 26937-27028（`Fcurrent_bidi_paragraph_direction`/`Scurrent_bidi_paragraph_direction`，0/1）。
     覆盖区间从 C 468-26935 推到 **C 468-27028**；**下一个活定义 = C 27030 的
     `bidi-find-overridden-directionality`**（DEFUN，Subrs 表第 9 个未勾项）。C 26929 的 `#undef
     RECORD_MAX_MIN_POS` 与 DEFUN 之间没有别的活定义。
  ② **行数**：`femacs/src/xdisp.rs` 25,692 → **25,779**（+87 / −0，md5
     `25a1231b6aa5e72965c02096b8c3ce02` → `7d0539d6438f7806f23a006d2c994cc4`）；台账 `docs/ports/xdisp.md`
     3,364 → 3,405 行。
  ③ **读数（命令原文进台账的 Gate evidence）**：⑴ 冻结 `xtask build-tu src/xdisp.c` **exit 0、1 发**，
     对象 **2,674,432 字节**（HEAD 2,661,656）、md5 `5dd3b2cc6f03149295d5d8083afcc034`，警告仍是原有的 6 条；
     `nm` 读到 `T _Fcurrent_bidi_paragraph_direction` 与该 subr 的 `Scurrent_bidi_paragraph_direction`
     静态符号；⑵ `extract-abi --check` **72 → 71 个 `- ` 行**——退掉 `missing subr Fcurrent_bidi_paragraph_direction`
     与 `missing defsubr (&raw const Scurrent_bidi_paragraph_direction)`，新增的
     `subr … does not mention case, default, switch` 由本轮新加的 `macro-body-notes.tsv` 一行答掉；
     `does not mention` 归零；⑶ `check-ledger-ticks` **235 / 0**（新 subr 的 tick 被承载）。
  ④ **本轮 5 条偏离（246-250，写进台账）**：C 27016-27027 的 `switch`（两个 `case` 各 `return` + 一个
     `default: emacs_abort ()`）→ 端口的 `match`；C 未初始化的 `struct bidi_it itb` → `core::mem::zeroed ()`（1.5）；
     C 的 `while ((c = FETCH_BYTE (bytepos)) == '\n' || …)` → 块表达式（`trailing_whitespace_p` 同款），
     `'\f'` 写 `'\x0c'`；`AUTO_STRING (trailing_white_space, "[\f\t ]*\n")` →
     `AUTO_STRING! (…, c"[\x0c\t ]*\n".as_ptr ())`；`BVAR (buf, x)` → `BVAR! (buf, x_)`，
     `itb.string.from_disp_str/unibyte = false` 走本单元的 `bidi_string_bits!` 位串臂。
  ⑤ **本轮欠账清点**：新加 1 条 use-site 声明（`fast_looking_at`，lisp.h 5031-5032，台账第 17 项）与 1 条
     `macro-body-notes.tsv` 行（上面那条）；**没有**动任何镜像、`docs/baseline/**` 与别人的文件。
  ⑥ **在飞（本轮派出，尚无读数）**：`android.c` 切片 2、`w32term.c` 切片 2、`xfns.c` 切片 1、`sfnt.c` 切片 1、
     `w32.c` 切片 1——5 条 lane，各自独占文件。
  ⑦ **未声称**：本轮没有运行任何东西；`--cfg enable_checking` 那条腿**本轮没跑**（上一轮的读数仍在台账里，
     不代表本轮新增的 `eassert!` 语句）；臂读数不适用（xdisp.c 在冻结构建里，`build-tu` 就是真编）。

- **3.421 同一轮的第二个 DEFUN：`bidi-find-overridden-directionality`（C 27030-27179）（2026-10-05）**：
  ① **区间**：C 27030-27179（`Fbidi_find_overridden_directionality`/`Sbidi_find_overridden_directionality`，3/4）。
     覆盖区间 **C 468-27179**；**下一个活定义 = C 27181 的 `move-point-visually`**（DEFUN，Subrs 表第 10 个未勾项）。
  ② **行数**：`femacs/src/xdisp.rs` 25,779 → **25,909**（+130）。
  ③ **读数**：⑴ 冻结 `xtask build-tu src/xdisp.c` **exit 0、1 发**，对象 **2,684,688 字节**
     （前一件 2,674,432）、md5 `7aaf0b43ac76352217bee6bbdd799c9e`，警告仍是原有 6 条；`nm` 读到
     `T _Fbidi_find_overridden_directionality` 与该 subr 的静态符号；⑵ `extract-abi --check` **71 → 70 个 `- ` 行**
     （整轮相对 HEAD 的 72），`does not mention` **0 条**、行数漂移 34.9%（HEAD 35.2%）；⇒ 本件**不欠**
     `macro-body-notes.tsv` 行。
  ④ **本轮新增 4 条偏离（251-254，写进台账）**：C 的 `do { … } while (…)` → 端口在末尾取反的 `loop`，
     C 的空体 `while ((found = …) < from_pos) ;` → 赋值块表达式 + 空 `{ }`；C 的 `ptrdiff_t from_pos,
     to_pos, from_bpos; void *itb_data;` 与后置的 `ptrdiff_t found;` 按 1.5 **零初始化**（两个分支通过指针
     参数写 `from_pos`/`to_pos`，Rust 看不见那些写）；DEFUN 的形参被 `validate_region` 取址改写 ⇒ 端口用
     `let mut from = from;`/`let mut to = to;` 承接（`composite.rs` 1989 同款）；`EQ (…) ? R2L : L2R` 与末尾的
     三目都写成 `if/else`。
  ⑤ **台账**：`## Subrs` 两个复选框勾上；Progress 的本轮 bullet 改成两个 DEFUN；use-site 声明加第 18 项
     （`bidi_find_first_overridden` dispextern.h 3558、`validate_subarray` lisp.h 4261-4262，并写明
     `CHECK_STRING`/`validate_region`/`decode_live_window`/`find_newline_no_quit` 早已可见的原因）；
     Gate evidence 追加本件读数。
  ⑥ **未声称**：同上一条——没有运行；`--cfg enable_checking` 那条腿本轮没跑；没有动镜像/`docs/baseline/**`/别人的文件。

- **3.422 上一笔的自己咬到自己：`validate_subarray` 的单元内声明是重复声明（2026-10-05）**：
  ① **发现**：3.421 提交后复跑门，`check-mirror-dupes` 报
     `FAIL femacs/src/xdisp.rs: validate_subarray is already declared by femacs/src/include/lisp.rs
     (in the include graph)`（读数 `20 duplicate / 19 ratchet / 1 failure`）。这是我自己的错：加声明前只查了
     **单元内**有没有这个名字，没查**镜像链**——`include/lisp.rs` 3719-3721 早就带着同一条原型
     （`(array, from, to, size, ifrom, ito)`）。
  ② **处置**：删掉单元内那份声明，原位留一行注释写明理由（P-26 的"一个名字一个家"），**不加棘轮行**。
  ③ **读数**：`check-mirror-dupes` 回到 **19 / 19 / 0**；重编 `build-tu src/xdisp.c` **exit 0**、警告仍是原有 6 条、
     对象 md5 `e8f95d661dc6f5033c3543544f3bbe8b`（删声明前后的 `--check` 都是 **70 个 `- ` 行**、`does not mention` 0 条）。
  ④ **教训（写下来）**：加 use-site 声明前必须同时查 `femacs/src/**` 与 `femacs/{src,lib}/include/**`；本轮台账的
     use-site 第 18 项已按此改写（`validate_subarray` 从"本轮新增"改成"镜像已有，不声明"）。
  ⑤ **未声称**：只是把这一个 finding 修平；`src/xfns.c` 的棘轮行已 stale（xfns lane 的在飞切片越过 10%），
     留给批次收口时 `--write-pending`。

- **3.423 android 切片 2（C 1038-1869）：线程/事件之后的文件系统与 JNI 入口（2026-10-05）**：
  ① **区间**：C 1038-1869（收在 `android_init_emacs_pixmap` 的 `}`）；**下一个活定义 = C 1871
     `android_init_graphics_point`**。块：`android_run_debug_thread`、`android_user_full_name`（C 1083-1090 的
     `HAVE_STRUCT_PASSWD_PW_GECOS` 对，死臂按 P-51 照录）、`android_is_special_directory`、
     **`#if 0` 区（C 1124-1360 两个函数）只照录为注释**、`android_get_home_directory`、`android_proc_name`、
     `android_create_lib_link`、三个 `NATIVE_NAME` JNI 入口（`getFingerprint`/`setEmacsParams`/`getProcName`）、
     `android_init_emacs_service`（函数内 `FIND_METHOD` 宏 + 41 次调用）、`android_init_emacs_pixmap`。
  ② **行数**：`femacs/src/android.rs` 1,311 → **2,103**（+798/−6）；`docs/ports/android.md` 345 → **465**；
     C 覆盖面 1,037 → **1,869 行**。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0**、对象 **336 字节空对象**（4 次同值）；⑵ 翻臂 ②a（真臂，无 API 门）
     **exit 1**，唯一错误是**已登记的镜像缺口** `E0609 no field sa_sigaction`（C 749，切片 1 就记过），
     **切片 2 自身 0 错**；⑶ 翻臂 ②b（`+ android_api_lt_21`）**exit 0 / 0 error / 17,376 字节对象**；
     ⑷ `extract-abi --check` **135 → 131 个 `- ` 行**（退掉本片 8 个函数 + `FIND_METHOD`，新增 5 条见台账），
     漂移 82.7% → **72.3%**；⑸ `check-dead-file-coverage` 50 行区间覆盖 C 7,594 行、**0 failure**。
  ④ **真缺陷（翻臂编译抓出来的，证明该读数非空）**：生成 `FIND_METHOD!` 调用清单时把两个 `android_init_*`
     的边界吃掉，`android_init_emacs_pixmap` 的序言与它自己的宏体被删（42 错）⇒ 修好后才有 ③ 的读数。
     C 自身的两处笔误（C 1564/1578 检查了相邻变量）**原样保留**，记进 Deviations。
  ⑤ **集成者本轮替它落的账**：`macro-body-notes.tsv` 两行（`android_is_special_directory` → `case,switch`；
     `android_create_lib_link` → `goto`）——补后 `--check` **131 → 129**；
     `docs/baseline/declarations-allowlist.tsv` 一行 `literal`（`macro_rules! PACKAGE_STRING`，`<config.h>` 2205
     的值被 C 1511-1513 拼进初始化日志的 format，属既有 6 行同一类；**lane 明确拒绝**为过门而拆字符串或改名，
     这是对的）⇒ `check-declarations` 回到 **14 sites / 0 forbidden / 0 failures**。
  ⑥ **仍未解决（点名）**：`tools/cinventory.py` 的函数扫描看不见 `NATIVE_NAME (x) (…)` 定义 ⇒ `extract-abi`
     对 3 个 JNI 入口报"invented exported symbol"（工具工作单，不是移植缺陷）；`lib/include/signal.rs` 的 bionic
     `sa_sigaction`（本次 ②a 的唯一错）；六处就地声明日后搬进镜像；编译预算**两项都超**（`build-tu` 5 发、
     `probe` 8 发，lane 如实报）。
  ⑦ **未声称**：无运行证据（无真机/NDK/JNI 调用/线程）；②b 的对象**从未运行**；②a 的基础配置**没有对象**。

- **3.424 xfns 切片 1（C 1-802）：位置解析、颜色与 alpha（2026-10-05）**：
  ① **区间**：C 1-802（收在 `x_set_alpha_background` 的 `}`）；**下一个活定义 = C 805
     `x_set_tool_bar_position`**。块：许可/声明块（drop）、三组 include、`HAVE_X_I18N`/`USE_XCB`/
     `HAVE_XRANDR`+`Xinerama`/`USE_GTK`/`HAVE_XDBE`/`HAVE_XINPUT2` 各门、`USE_X_TOOLKIT` 大块、
     `PropMotifWmHints`、`check_x_display_info`、`x_real_pos_and_offsets`（C 206-588，四个查询/翻译段）、
     `x_real_positions`、`x_relative_mouse_position`、`gamma_correct`、`x_defined_color`、`x_decode_color`、
     `x_set_wait_for_wm`、`x_set_alpha_background`。
  ② **行数**：`femacs/src/xfns.rs` **11 → 1,126**（lane 报 1,120，集成者补 `USG` 宏后 +6）；台账**新建**
     `docs/ports/xfns.md` **365 行**（28 行区间表）。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0**、对象 **336 字节空对象**（整身挂门；lane 用了 4 发，**超预算 1**，
     首发是 `E0758` 未终止块注释）；⑵ **四次翻臂**（`have_x_windows`；再加 `use_xcb/have_x_i18n/glyph_debug/
     have_xrender/have_xdbe/have_xinput2`；再加 `use_gtk/have_gtk3`；全部门并集）：**本单元正文分别 2 / 3 / 2 / 2
     条**，镜像侧 21 / 50 / 67 / 74 条（无镜像 SDK 名 + 镜像 INLINE 的 `E0133`），**没有一次是 0 error**——GTK
     臂只能 `text-only`（本机无 GTK 树）；⑶ `extract-abi --check` **230 → 227 个 `- ` 行**（集成者补 2 行 notes +
     `USG` 宏后；lane 原始读数是"227 条 missing"），漂移 **89.4%**；⑷ `check-dead-file-coverage` 28 行/0 failure。
  ④ **真缺陷**：`E0381` 顶层声明零初始化（1.5）；**Window↔`xcb_window_t`/Atom↔`xcb_atom_t` seam** 与 xcb 回复的
     窄整型回读共 18 个 `E0308` + 3 个参数类型错（C 的隐式转换，端口必须写 `as`）；`E0758`（注释里照抄
     `Xm/*.h`，Rust 嵌套块注释把它当打开 `/*`，PORTING.md 0.3 明写的陷阱）；`1L` 后缀、镜像宏要 `!`、
     `bw` 的窄化、`long *fe`、两处 `Emacs_Color`/`XColor` 别名转换；**5 处引用行号按装好的头改正**（不编造）。
  ⑤ **集成者本轮替它落的账**：2 行 `macro-body-notes.tsv`（`check_x_display_info` → `0`；`x_real_pos_and_offsets`
     → `uninit`）；在 `xfns.rs` 里按 C 87-95 定义 **`USG` 空标记宏**（C 自己就是 `#define USG` 空定义，
     先例是 alloc.rs 的 `__MALLOC_HOOK_VOLATILE`/`GC_SETJMP_WORKS`）⇒ 三条 finding 退掉、重编 exit 0 / 336 字节；
     `femacs/xtask/Cargo.toml` 补 **4 个 feature**：`use_motif`、`have_xaw3d`、`have_xrandr`、`have_xinerama`
     （`cargo metadata` 读回 **594** 个 feature）。
  ⑥ **一条裁决（写下来，免得下一轮重复讨论）**：lane 请求给 `xfns.c` 加一条
     `docs/ports/line-drift-tolerance.tsv` 行（"每个切片都欠，直到末片落地"）。**不加**——实测该表现有 10+ 行
     **全部是已完成的文件**、且理由都是**结构性**涨落（`fileio` 的 docstring 折叠、`realloc` 的开头块不抄、
     `terminfo` 的短文件吸收标准头……），**没有任何部分移植的文件在表里**；`line count drifted 89.4%` 是
     "文件只落了 10.6%"的**如实读数**，不是需要豁免的结构性偏差。⇒ 该行（以及 `image` 的同名欠账）在**各自末片
     落地时**才写；这一条同时更正 `docs/ports/image.md` 的 Owed 措辞。
  ⑦ **未声称**：无运行证据（没跑过 X server/XCB/GTK）；四次翻臂都是编译读数且镜像侧都有错，**不声称任何配置可工作**；
     本单元正文仍留 2 条已登记的欠账（`x_display_info_for_name`、镜像缺 `wait_for_wm` 存储）。

- **3.425 sfnt 切片 1（C 1-1112）：表目录、cmap 的七个格式与交换助手（2026-10-05）**：
  ① **区间**：C 1-1112（收在 `sfnt_read_cmap_table` 的 `}`）；**下一个活定义 = C 1117 `sfnt_lookup_glyph_0`**。
     23 个函数定义 + `sfnt_table_names` + 4 个宏 + 3 处预处理名；`#ifdef TEST` 两臂、七个分配器、
     `sfnt_table_names`（18 个槽全部指定、无隐式零）、四个 `sfnt_swap*`（`#ifndef WORDS_BIGENDIAN` 另一臂是 C 的空体）、
     `sfnt_read_table_directory`/`sfnt_find_table`/`sfnt_read_cmap_format_{0,2,4,6,8,12,14}`/`sfnt_read_24`/
     `sfnt_read_cmap_table{_1,}`。
  ② **行数**：`femacs/src/sfnt.rs` **11 → 1,298**（+16.7% 对 C 1,112 行）；台账**新建** `docs/ports/sfnt.md` **263 行**
     （26 行区间表，同时覆盖 `src/sfnt.c` 21,215 行与 `src/sfnt.h`）。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0**、对象 **336 字节空对象**（3 发用满；首发暴露端口自身语法错
     `unclosed delimiter`，修掉）；⑵ 翻臂预检（`have_android + not_android_stubify`）**exit 1 / 8 个 error，
     `端口自身正文 0 个`**——8 条全在镜像：`include/sfnt.rs` 2200/2206 的两条 `E0428`、
     `lib/include/alloca.in.rs` 49 的 `expected item after doc comment`、经 lisp.rs 链到达的 5 条
     `E0425 time_t/timespec`；**对照读数**（修之前的同一 harness）确实是**本单元正文**的错
     （36×`E0608`、21×`E0600`、4×`E0308`、3×`E0381`、1×`E0530`）⇒ ② 的"正文 0 错"不是空话；
     ⑶ `extract-abi --check` **368 → 367 个 `- ` 行**（集成者补 1 行 notes 后）、漂移 93.9%；
     ⑷ `check-dead-file-coverage` 20 → **21 ledger / 0 failure**。
  ④ **本片抓到 6 条真缺陷，其中 4 条是"已提交文件里的潜伏缺陷"（值钱的一条）**：
     ⑴ `lib/include/alloca.in.rs` 尾部是 doc 注释且其后无 item ⇒ 真编时报 `expected item after doc comment`
     （本单元是该镜像在全树的**第一个使用者**，此前没人编到它）；⑵ `include/sfnt.rs` 把两个共享读者声明放在
     **顶层** ⇒ 任何定义它们的单元必然 `E0428`，需 `pub mod sfnt_decls { … }`（`include/android.rs` 的形状）；
     ⑶ `static mut` + 初始化块内 `let` 同名绑定 = `E0530`（**`keyboard.rs` 5607-5610 是同形代码**，在
     `have_android` 下同样编不过）；⑷ `(-1 as *mut T)` 这类写法**不可编译**（`E0600`），而 `doc.rs` 83、
     `lread.rs` 2092/2106 都是这个写法——三处都在 `use_android_assets` 门内、从未被编译过；⑸ `extract-abi.py`
     对本文件仍打印"not compiled by the frozen configuration … Nothing in this file is required here"
     （工具文本没跟上 P-86 ① 的裁定）；⑹ 台账自己会被 `check-declarations.py` 扫（初稿照抄了 C 1-18 的版权行）。
  ⑤ **集成者本轮替它落的账**：`macro-body-notes.tsv` 一行（`sfnt_read_cmap_table_1` → `case,default,switch`，
     C 990-1010 的 `switch (format)` 与 C 992/996/1000/1004 四个 `case`、C 1008 的 `default`）。
  ⑥ **本次新立的工作单（点名，下一轮可做）**：⑴ 上面 ⑵⑶⑷ 三处**潜伏缺陷**要在各自单元/镜像里改，并给
     `token-aliases.tsv` 加两行（`(T *) -1` → `usize::MAX as *mut T`；原生指针 `p[i]` → `*(p.offset (i as isize))`）；
     ⑵ `include/sfnt.rs` 的 `sfnt_decls` 模块（镜像轮，需 `check-all-units.sh`）；⑶ `TEST_STATIC` 的 Rust 形态
     （`excluded-macros.txt` 或 PORTING.md §11）；⑷ Android 臂还缺的镜像（alloca 注释、`time_t`/`timespec`、
     mman/setjmp/X11 三类）。
  ⑦ **未声称**：本片**没有任何运行证据**（没打开过字体、没读 table directory、没解析过一次 cmap）；
     ② 连目标文件都没产出（错全在镜像），翻臂读数取自与最终修订仅差注释的修订；C 1113-21,215 **未移植**。

- **3.426 w32term 切片 2（C 1004-1949）：GC 选择、各种 glyph string 的绘制与 relief（2026-10-05）**：
  ① **区间**：C 1004-1949（946 行，收在 `w32_draw_glyph_string_box` 的 `}`）；**下一个活定义 = C 1952
     `w32_image_rotations_p`**。22 个函数 + 1 个宏（`HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`）；本片自身无 `#if`
     （只有两处 `#if 0` 死文本，按 C 原文搬进注释）；台账区间表 59 → **84 行**。
  ② **行数**：`femacs/src/w32term.rs` 1,592 → **2,941**（+1,349）；`docs/ports/w32term.md` 389 → **614**。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0 / 336 字节空对象**（3 发用满）；⑵ `extract-abi --check` **134 → 110 个 `- ` 行**
     （102 缺函数 + 1 缺宏 `LOAD_PROC` + 6 缺文件级对象 + 行数漂移 65.0%），**`does not mention` 0 行**；
     ⑶ 臂触达：frozen（无 cfg）**0 error**；base 集（`have_ntgui+windowsnt+have_window_system+
     dispextern_face_records`）首轮 **70 error / 单元 44 / 本片 26**，修好后 **42 / 单元 16 / 本片 0**；
     `have_ntgui` 单门 108 error（与切片 1 同类，锚点 110）；⑷ `check-dead-file-coverage` 84 行覆盖 .c/.h、0 failure。
  ④ **真缺陷（本片抓到 26 个自己的错，冻结门完全看不到——臂读数为什么值钱）**：Rust 裸指针字段访问不自动解引用
     ⇒ C 的 `s->f->output_data.w32->cursor_gc->foreground` 每跳要 `(*…)`、`as *mut w32_output` 必须自带括号
     （9 处 `E0606/E0609`）；4 处 `glyph_u_cmp_bits!` 传值而宏内解引用（`E0614`）；`RGB!` 体内 `u8|u16`
     （`E0277`，4 次展开）；`composition_gstring_width` 的 `ptrdiff_t` 形参收到 `c_int`（`E0308`）；
     `FillRect` 与切片 1 的 `<windows.h>` 段**重复声明**（`E0428`，删本片那份并点名）；并**顺带修掉切片 1 的 C 970 行**
     （同样缺一对括号，2 个错）——这是本批唯一改动的切片 1 文本，已记明。
  ⑤ **仍未解决（点名）**：`include/w32term.rs` 340 行处声明了 `w32_query_frame_background_color` 而未做 owed 4 的
     `*_decls` 模块化 ⇒ 单元定义 `E0428`（实测）；`include/lisp.rs` 缺 `min`/`max`；`include/dispextern.rs` 缺
     一批访问器/宏/入口点；`<windows.h>` 整体无镜像（`POINT`/`RECT`/`W32Msg` 的布局仍 UNVERIFIED）；
     base 臂仍有 24 个 frozen-only `E0080` 布局断言、`include/frame.rs` 329 的 `have_ntgui compile_error!`、
     `include/w32common.rs` 111 的 `SYSTEM_INFO`；文件余下 **C 1950-8,414** 未移植。
  ⑥ **未声称**：**零运行证据**（无 Windows、无 `windows.h`、无 mingw、无布局测量、无对拍/ERT/设备）；
     "本片 0 错"只等于"该臂下没有锚在本片文本上的名字解析/类型错误"，**不等于该单元可编译、更不等于 Windows 配置可用**；
     `--check` 依旧 FAIL（110 行里 109 是余下部分 + 行数行），本文件按 P-86 ② 用区间表 + 读数代替清单通过。

- **3.427 sfnt lane 点名的两处潜伏缺陷已修：`sfnt_decls` 模块与 `alloca.in.rs` 的注释（2026-10-05）**：
  ① **缘由**：3.425 的 sfnt 臂读数 `exit 1 / 8 error` 里有 3 条**不在本单元正文**，而是两处**已提交文件里的
     潜伏缺陷**——⑵ `include/sfnt.rs` 把两个共享读者声明放在顶层（任何定义它们的单元必然 `E0428`）；
     ⑴ `lib/include/alloca.in.rs` 结尾 26 行 `/// C: …` 是**没有 item 承接的 doc 注释**（该镜像在全树的
     第一个使用者就是 sfnt）。两处都只在门后的臂里才被编到，所以此前十几次提交都没暴露。
  ② **处置**：⑴ `alloca.in.rs` 的 26 行 `///` 改成 `//`（它们本来就是**引文**，不是文档）；⑵ `include/sfnt.rs`
     C 1469-2483 的**声明段整段**搬进 `pub mod sfnt_decls { use super::*; … }` 并在其后 `use sfnt_decls::*;`
     ——crate 根项**遮蔽** glob 导入，所以定义这些名字的单元不再 `E0428`（`window.rs` 302-367 /
     `include/android.rs` 的形状，P-21）。
  ③ **读数**：⑴ 冻结 `build-tu src/sfnt.c` **exit 0**、对象仍 336 字节；⑵ **臂触达 `8 → 5 error`**，
     剩下的 5 条**全是** `E0425 time_t`/`timespec`（经 lisp.rs 链到达 `sys_time.rs`/`sys_select.rs`/`thread.rs`，
     就是 3.425 ⑥ 那组的下一项）；⑶ **P-87 两腿**：`tools/check-all-units.sh` → `checked 83 unit(s),
     0 failure(s)` + `checking 83 unit(s), 0 unit failure(s)`（两处镜像改动没有碰坏任何一个冻结单元）。
  ④ **台账**：`docs/ports/sfnt.md` 的 Owed 第 1 项标 ✅ Done、第 5 项标"部分完成"，并追加一段本轮实测。
  ⑤ **仍未解决（点名，继承 3.425）**：`time_t`/`timespec` 镜像；`TEST_STATIC` 的 Rust 形态；
     `(T *) -1`/原生指针下标两条 `token-aliases.tsv` 行；`<sys/mman.h>`/`<setjmp.h>`/TEST 臂的 X11 头无镜像；
     `keyboard.rs` 5607-5610 的 `static mut` + `let` 遮蔽（`E0530`）与 `doc.rs` 83 / `lread.rs` 2092/2106 的
     `(-1 as *mut T)`（`E0600`）——同一类潜伏缺陷，下一轮按同一办法修。

- **3.428 同一类潜伏缺陷的第二批：`E0530`（keyboard）与 `E0600`（doc/lread）已修（2026-10-05）**：
  ① **缘由与实测**：3.425 的 sfnt 臂把这一类点了名——**只在门后的臂里才被编到**的代码从未被编译过。本轮先用一个
     最小实验坐实根因：`fn f (p: *mut c_void) -> bool { p != (-1 as *mut c_void) }` 在钉住的 nightly 上**真的报
     `E0600 cannot apply unary operator '-' to type 'usize'`**（该上下文把 `-1` 推成 `usize`，并提示 `usize::MAX`），
     所以 `doc.rs` 83 与 `lread.rs` 2092/2106 不是"看似有先例"而是**确实编不过**。
  ② **处置**：⑴ `keyboard.rs` 5606-5699 的 `#[cfg (feature = "have_android")] static mut lispy_function_keys`：
     初始化块里 `let mut lispy_function_keys` **同名遮蔽了 static**（Rust 的 `E0530`）⇒ 块内那个数组改名 `keys`
     （**87 处使用点同步改名**，C 的每条 index/value 一字不动，并留注释说明为什么名字不同）；
     ⑵ `doc.rs` 1 处、`lread.rs` 2 处的 `(-1 as *mut c_void)` → **`(usize::MAX as *mut c_void)`**
     （C 的 `(void *) -1` 哨兵的字面表达）。
  ③ **读数**：⑴ 冻结 `build-tu` 三件（keyboard/doc/lread）**都 exit 0**；⑵ 臂探针（同一命令，改后）：
     `keyboard.rs + have_android` → **3 error（2 `E0425` + 1 `E0428`），`E0530` 归零**；
     `doc.rs + use_android_assets` → **6 error（4 `E0425` + 2 `E0308`），`E0600` 归零**；
     `lread.rs + use_android_assets` → **13 error（6 `E0425` + 5 `E0610` + 1 `E0422` + 1 `E0308`），`E0600` 归零**。
     剩下的**全部是臂里既有的镜像/类型缺口**，不是本轮的改动物（例：lread 的 5 条 `E0610` 是
     `use_android_assets` 臂里 `fd` 仍是 `i32`、而臂内代码写 `fd.fd`/`fd.asset` 这类字段）。
  ④ **一条实测裁决**：3.425 建议的两条 `docs/ports/token-aliases.tsv` 行（`(T *) -1` → `usize::MAX as *mut T`；
     原生指针 `p[i]` → `*(p.offset (i as isize))`）**现在不需要**——`sfnt`/`keyboard`/`doc`/`lread` 四对 `--check`
     里**没有任何**非 `missing …`/`is not static`/`line count drifted`/`does not mention` 的 finding，
     而该表也没有"未使用行"校验；⇒ **不加空行**（加了就是无据可验的文档），等真出现 occurrence 检查时再补。
  ⑤ **仍未解决（点名）**：上头三个臂剩下的缺口（`E0425` 的 `time_t`/`timespec` 与各臂各自的 SDK 名、
     lread 的 `E0610`/`E0422`、keyboard 的 `E0428`）；`TEST_STATIC` 的 Rust 形态；
     `include/sfnt.rs` 之外的镜像轮欠账照旧。

- **3.429 P-89 的根因修掉：materialize 的展开不再让"带门的 include 边"抢走唯一一次包含（2026-10-05）**：
  ① **症状与真因**：sfnt 的 `have_android` 臂一直报 **5 条 `E0425 time_t`/`timespec`**，3.425 把它记成"镜像缺名"。
     实测走查证明**名字不缺**：`lib/include/time.in.rs` 36/43 行就声明着它们；坏的是 **展开顺序**——
     sfnt.rs 的 `<time.h>`/`<timespec.h>`（C 52-53）在 C 里位于 `#ifdef TEST` 内，端口照录成
     `#[cfg (… feature = "test")] include!(…)`；`materialize` 的 `seen` 是**按文件、不看边上的 `#[cfg]`**
     （P-89 的原文判据），于是这条**带门的边**占掉了 `time.in.rs` 的唯一一次展开，而**无门**那条
     （经 `lisp.rs` 链 → `buffer.rs` 16 → `time.in.rs`）被降级成 `// [xtask dedupe] already included`
     ⇒ 类型从未出现在被编译的文本里。
  ② **修法**（`femacs/xtask/src/main.rs`）：新增 `include_edges()`（逐行报告每条 `include!` **是否带门**，属性行与
     include 之间允许空行）与 `ungated_reachable()`（只沿**无门**边做可达性预扫）；`render` 增加沿路径传递的
     `path_gated`，规则一句话：**凡存在无门路径到达的文件，只在无门那条上展开**，任何带门路径（无论是它自己带门，
     还是嵌套在带门文件的副本里——第一版只判了前者，实测不够）上的到达一律降级成
     `// [xtask dedupe] … expands at its ungated reach`。没有无门路径的文件照旧由第一条到达展开 ✓。
  ③ **读数**：⑴ `cargo check -p xtask` 通过；⑵ **sfnt 的 `have_android + not_android_stubify` 臂：`exit 1 / 5 error`
     → `exit 0`，产出 4,784 字节对象**（这是该臂第一次编出对象）；⑶ **P-87 两腿 `checked 83 unit(s), 0 failure(s)` +
     `checking 83 unit(s), 0 unit failure(s)`**——每个单元都要重新 materialize，所以这条读数是这次改动的主要判据；
     ⑷ `docs/ports/sfnt.md` 的 Owed 5 按此更正（"镜像缺名"→"展开顺序"）。
  ④ **文档同步**：`PORTING.md` P-89 的第 ③ 条（"根因在 xtask，修它是一项独立工作"，3.408 的工作单）改成"已在 3.429 修掉"，
     并保留 ① ② 两条纪律（加带门边前先确认目标有无门路径；被咬先按 ② 绕开）。
  ⑤ **未声称**：本改动只让"带门边不再抢位"，**不声称**任何配置可运行；P-87 两腿是冻结构建，它不覆盖被门挡住的文本；
     上面那个 4,784 字节对象**从未运行**。

- **3.430 w32 切片 1（C 1-1197）：SDK 装载、安全描述符与系统时间（2026-10-05）**：
  ① **区间**：C 1-1197（收在 `get_system_times` 的 `}`）；**下一个活定义 = C 1199 `create_symbolic_link`**。
     文件门是 **`feature = "windowsnt"`（不是 `have_ntgui`）**：`configure.ac` 3051 只在非 Cygwin 臂把 `w32.o`
     放进 `W32_OBJ`，而 `nt/inc/ms-w32.h` 正是 `#define WINDOWSNT` 的地方；门下到整条 include 图（P-86 ⑧）。
     56 行区间表连续覆盖 C 1-11,362；本片含 44 个 `g_b_init_*`、约 100 个 SDK 入口 typedef、
     `w32_get_internal_run_time`（唯一导出定义）与 18 个 security-descriptor helper。
  ② **行数**：`femacs/src/w32.rs` **15 → 1,846**；台账 `docs/ports/w32.md` 24 → **350**。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0 / 336 字节空对象**（4 发，**超预算 1**；首发是注释里
     `include/*.rs` 打开 Rust 嵌套块注释的 `E0758`——真缺陷，已修）；⑵ **`windowsnt` 臂**：修正本单元那条
     `ltime` 引用后 **111 个错误站点全部在镜像**（socket 70 / ms-w32 17 / signal 14 / errno 13 / fcntl 6 /
     dirent 5 / dispextern 4 / time 4 …），**本单元与 `include/w32.rs` 各 0 条**；⑶ 打桩预检 harness
     **exit 0 / 0 error / 15,224 字节对象** + **2 条变异测试**（`== 0`→`== "x"`、`LoadLibrary (1)` 都 exit 1）；
     ⑷ `extract-abi --check` **277 → 271 行**（集成者补 2 行 token-aliases + 2 行 excluded-macros 后；
     余下 4 条 `missing macro` 是 remainder 里的 `DIR_FIND_DATA_A/W`、`DRIVE_INDEX`、`FILETIME_TO_U64`）；
     漂移 83.8%；⑸ `check-dead-file-coverage` **23 ledger / 0 failure**（覆盖 `w32.c` 11,362 行与 `w32.h` 273 行）。
  ④ **真缺陷（7 条）**：注释里的 `include/*.rs` 触发 `E0758`；缺 `LPFILETIME`/`LPMEMORYSTATUS`（10 处 E0425）；
     漏 `get_process_times_fn`（C 383）；`c_longlong` 不在 lisp.rs 的 use 名单（改 `i64`）；三处 C 隐式转换与
     `(-1 as c_int) as DWORD`（`-1 as DWORD` 直接 `E0600`）；`REPARSE_DATA_BUFFER.DUMMYUNIONNAME` 要按 C 拼写；
     归属注释里的"不是 licence 块"触发 `recital` 类（改中性表述）。
  ⑤ **集成者本轮替它落的账**：`femacs/xtask/Cargo.toml` 补 **4 个门名**（`unicode`、`win32_winnt_lt_0501`、
     `mingw32_w32api_ge_5_2`、`mingw32_w32api_lt_3_15`）；`docs/ports/excluded-macros.txt` 两行
     （`DEFER_MS_W32_H`、`ORIG_WIN32_WINNT`，皆 kind=config）；`docs/ports/token-aliases.tsv` 两族
     （`wchar_t`→`c_ushort`；`NULL`→`None`/`is_none ()`/`is_some ()`/`null_mut`/`is_null`）；
     **`include/w32.rs` 的 4 项声明模块化**（`w32_objs`/`w32_fn_decls` + glob 导入，`window.rs` 302-367 形状）
     ⇒ 臂里那 4 条 `E0428` 消失。
  ⑥ **未解决（点名）**：上面 111 条镜像错各自的镜像轮（socket 那 70 条尤其大，且各头自己的台账要写）；
     9 条 SDK 记录的布局断言需 mingw 工具链；`SYSTEM_INFO`/`OSVERSIONINFO` 与 `w32common.rs`/`w32fns.rs` 的
     推迟决定要调和 + PORTING.md §11 条目；`check-cond-eval` 对该文件仍 `[NOT VERIFIED]`（35 条件 / 5 unknown）。
  ⑦ **未声称**：**零运行证据**（没有 `LoadLibrary`/`GetVersionEx`/注册表/套接字/文件调用，没有 Windows 目标编译、
     没有布局实测）；打桩 harness 的对象**从未运行**，变异测试只证明 harness 真的在编本单元正文；
     C 1198-11,362 **未移植**（271 行读数与 83.8% 漂移是那条边界）。

- **3.431 androidvfs 切片 1（C 1-995）：vnode/vdir 骨架、名字规范化与 Unix 半边（2026-10-05）**：
  ① **区间**：C 1-995（收在 `android_unix_opendir` 的 `}`）；**下一个活定义 = C 1016 `android_extract_long`**
     （其间还有 C 999-1002 的注释块与 C 1005/1008/1011 三个文件级对象）；40 行区间表连续覆盖 C 1-7,950。
     块：`android_vops`(13 成员)/`android_vnode`/`android_special_vnode`、`static_assert (NIL_IS_ZERO)`、
     9 个 enum 成员、6 个文件级对象、三个 `android_init_*_class`（各带函数内 `FIND_METHOD`）、
     `EMACS_PATH_MAX`、`android_vfs_canonicalize_name`、`root_vnode` + 11 条前向声明、`unix_vfs_ops`、
     `android_unix_name` 与 16 个 Unix vnode 操作。
  ② **行数**：`femacs/src/androidvfs.rs` **11 → 1,429**；台账 `docs/ports/androidvfs.md` 24 → **303**。
  ③ **读数**：⑴ 冻结 `build-tu` **exit 0 / 336 字节空对象**；⑵ **两条臂都 exit 0**：
     ②a（文件门 only，C 的 `#else` 臂）**0 error / 4,272 字节对象**、②b（`+ android_api_ge_9`，C 的 `>= 9` 臂）
     **0 error / 4,216 字节对象**；5 条 warning 全来自被包含的镜像，**本单元自己 0 error 0 warning**；
     ⑶ `extract-abi --check` **181 → 179 行**（集成者补 2 行 notes 后；178 条 `missing…` 逐条核对**全部落在
     C 995 之后**）；漂移 82.0%；⑷ `check-dead-file-coverage` 40 行/0 failure、`check-declarations` 14/0/0、
     `check-mirror-dupes` 19/19/0、`check-citations` 675/0。**预算如实**：`build-tu` 11 发（限 3）、`probe` 5 发（限 3）。
  ④ **真缺陷（4 条，前两条是系统性的）**：⑴ **`include/android-asset.rs` 的 `macro_rules! assert` 会污染兄弟镜像**——
     经 `include!` 到达的同名宏属"macro-expanded"，任何兄弟镜像里的 `assert!` 都变 **E0659**（两臂 60+ 条）；
     代价用**变异实测**量过（把 `dispextern.rs` 的一条布局断言改成 `== 999` 仍编过 ⇒ 该 TU 里镜像的断言被展开成 `()`），
     本单元自己的断言因此一律写 `core::assert!`；⑵ **P-89 的坑在镜像内部又来一次**（`android-asset.rs` 自己那条
     带门的 `include!("jni.rs")` 会吞掉 `jni.rs` 的唯一一次包含）⇒ 把本单元自己的 `jni.rs` 边放到 C 51 边之前；
     ⑶ `struct android_vdir` 与 `include/android.rs` 的 P-70 替身**不能共存**（E0428；用替身则布局 0 字节而 C 是 24）
     ⇒ 本单元不 include `android.h` 镜像、就地声明那一个名字；⑷ Rust 必补写法（`p[i]`→`*p.offset(i as isize)`、
     enum→`as c_short`、变参 `open` 的 `mode`、`errno`→`*errno ()` 等）+ 16 处零初始化（1.5）。
  ⑤ **集成者本轮替它落的账**：`femacs/xtask/Cargo.toml` 补 **`android_api_ge_9`**（C 46 的 `__ANDROID_API__ >= 9`）；
     `docs/ports/macro-body-notes.tsv` 两行（`android_vfs_canonicalize_name` → `case,default,goto,switch`；
     `android_unix_name` → `goto`）。
  ⑥ **本轮的 `gate-fast` 读数（另一件事，记下来免得误读）**：它在**清单步**停下并打印
     `checked 590 ported files: 577 done, 2 in progress, 11 failing` + `error: inventory differences`
     ——那 11 个就是**正在切片的单元**（含 4 条新 lane 刚写下的部分文件），是批量推进中的正常态；
     它没有走到 layout/build-tu 两腿，所以**不能**当作全树编译读数。
  ⑦ **未声称**：**零运行证据**（没有 JNI 调用、没有 `opendir/readdir/fstatat/rename/symlink`、没有创建/命名/打开过任何
     vnode、没有 Android 设备或模拟器）；②a/②b 的对象**从未运行**；C 996-7,950 未移植；C 1005 的 `directory_tree`
     被 include 图"提前回答"，**切片 2 不得当成已移植**（台账已点名）。

- **3.432 布局断言族的静默失效修掉：`const _: () = …` 一律写 `core::assert!`（2,138 处 / 106 文件）（2026-10-05）**：
  ① **危害（3.431 的 lane 实测到的那条）**：C 的 `<assert.h>` 在**一个 TU 里只被读一次**，冻结配置定义
     `NDEBUG` ⇒ 它的 `assert (cond)` 展开成 `((void) 0)`、**操作数不求值**。端口在有的单元里按 C 的名字与空体
     保留了这个宏（`androidvfs.rs` 的 C 23、`include/android-asset.rs` 自己读到的那次），而**经 `include!` 到达的
     裸 `assert!` 会解析到它**：轻则 `E0659`，重则**镜像里的布局断言被静默展开成 `()`**——lane 的变异实测量过：
     把 `dispextern.rs` 的 `size_of::<Emacs_Rectangle> () == 16` 改成 `== 999`，当年那个 TU **照样编过**。
     也就是说：**"布局断言真的在断言"这件事，在一个包含 C `assert` 宏的 TU 里是假的**。
  ② **处置**：**凡"布局/配置断言"（`const _: () = …` 这一族）一律写 `core::assert!`**——路径限定宏不受同名
     `macro_rules!` 遮蔽。**一次性改写 2,138 处、106 个文件**（含各单元自己的 `macro_rules! static_assert`
     展开体）；`PORTING.md` 的 `static_assert` 条目同步改写并写上理由。
  ③ **读数**：⑴ **P-87 两腿 `checked 83 unit(s), 0 failure(s)` + `checking 83 unit(s), 0 unit failure(s)`**
     （改动扫到 106 个文件，这是主判据）；⑵ **变异实测（证明断言真的活了）**：基线 = androidvfs 的 ②b 臂
     （`have_android + not_android_stubify + android_api_ge_9`）**exit 0 / 0 error / 4,216 字节对象**；
     把 `include/androidterm.rs` 的第一条布局断言从 `== 48` 改成 `== 999` → **`error[E0080]: evaluation panicked:
     assertion failed: std::mem::size_of::<android_bitmap_record>() == 999`**（改之前同样的变异会静默通过），
     随后已还原；⑶ `git add` 只收**非在飞**文件 104 个 + `PORTING.md`，4 条 lane 正在写的
     `xdisp/pgtkterm/comp/treesit` 里那部分改名**随各自落地一起进**（保证任何单次提交里的 TU 自洽）。
  ④ **未声称**：这是一次**拼写级**的机械改写——它不保证任何一条断言为真，只保证它们**真的被求值**；
     4 条在飞文件里剩余的约 150 处随它们落地；`assert` 宏到底该由**镜像**还是**单元**定义（C 的 `<assert.h>`
     在一个 TU 里只有一次读）仍按 3.431 的现场处置留着，未改。

- **3.433 版本声明的第五半：程序"说自己叫什么"的那几处声明也改成本仓库自己的名字（2026-10-05）**：
  ① **范围与判据**：0.1-19 的前四半管到文件头、注释、复述与工具写出的身份行，但**字符串字面量整类**
     被留在门外；其中 C 的 docstring 确实是移植语义（`tools/extract-abi.py --check` 逐字比对），而
     `PACKAGE_NAME`/`PACKAGE_STRING`/`RCS_Id` 与核心、`etags`、`ebrowse` 的版本横幅是**本程序对自己
     身份的声明**（`--version` 打印它们、`ident(1)` 从可执行文件里读 `RCS_Id`），照抄下来就是一句
     "本程序是参照产品" ⇒ 按用户判定（2026-10-05："版本声明不要再说是 GNU Emacs 了……把之前的代码中的
     这种声明也去掉"），**这一类也去掉**。判据一句话：**这句话说的是谁**——说"本程序叫什么"的改，
     说"参照叫什么"或"对外注册名"的留。
  ② **改动 6 处 / 4 个单元**：`src/emacs.rs` 的 `PACKAGE_NAME`（`--version` 打印、`emacs-version` 携带）
     与 `RCS_Id`；`lib-src/etags.rs` 的 `PACKAGE_NAME` 与由它拼出的横幅；`lib-src/ebrowse.rs` 横幅里
     "本程序按参照产品同样的条款分发"那一句（改写成本仓库自己的条款 `GPL-3.0-or-later`）；
     `src/android.rs` 的 `PACKAGE_STRING` 宏。一律写本仓库自己的名字 `femacs` + 参照的版本号 `32.0.50`。
  ③ **保留（点名，不藏着）**：C 的 docstring、`COPYRIGHT` 与许可证文本（署名/许可义务，且它们不声称
     本程序是参照产品）、`PACKAGE_BUGREPORT`（参照的实测配置值）与 Haiku 的 `application/x-vnd.…`
     bundle 签名（对外注册名）；参照的**实测值**仍留在 `docs/baseline/config/`。这四处若也要改，是下一句话的事。
  ④ **读数**：⑴ `check-declarations` **14 → 10 行**、`0 forbidden`、`0 failures`（豁免表随之重写：4 行随
     改写消失、2 行随横幅文本更新，其余 10 行理由不变）；⑵ `extract-abi --check`：`emacs.c`/`etags.c`/
     `ebrowse.c` 三个单元仍是 **OK**（改的不是 docstring），`android.c` 仍是**同样的两条 FAIL**
     （`setEmacsParams` 的 `#[unsafe(no_mangle)]` 与 72.3% 漂移；把 HEAD 版取到 `/tmp` 逐条比过，两条一字不差，
     漂移里的 Rust 行数 2,103 → 2,106 正是本条加的三行注释）；⑶ 冻结档 `build-tu` **四个单元全部 exit 0**
     （`emacs.o` 177,536 / `etags.o` 693,000 / `ebrowse.o` 293,216 / `android.o` 336 字节空对象）；
     ⑷ **目标字符串在目标文件里实测到了**：`strings emacs.o` → `$Id: femacs 32.0.50 (aarch64-apple-darwin25.6.0 PDUMPER) $`、
     `etags.o` → `etags (femacs 32.0.50)`、`ebrowse.o` → `… under the terms of the GNU General Public License,
     version 3 or later.`，`nm -g emacs.o` 里 `_RCS_Id` 仍在；⑸ `check-ledger-ticks` **234 行 / 0 failure**（在飞
     台账会让这个数浮动）、`check-dead-file-coverage` 29 ledger / 0 failure、`check-mirror-dupes` 19/19/0 不动。
  ⑤ **未声称**：**没有任何运行证据**——四个单元只编到对象，**没有链接、没有启动、没有跑过 `--version`**；
     `src/android.rs` 的那一处还在冻结配置为假的文件门里，只有文本级证据。与参照 `--version` 的**逐字差异
     是有意的**：L0/L1 金标不含版本横幅（3.413 已把两个金标的头部改成只写版本号），二者都不受影响；但差分
     阶梯里凡打印 `emacs-version`/版本横幅的地方，从此**与参照不同**——这是本条规则明码标价的代价，记在
     这里而不是藏在代码里。`build-tu` **4 发（限 3，超 1，如实报）**。
  ⑥ **顺带**：3.432 的 status 段此前没有随它的提交落地，本次一并入库（该段文本未改）。

- **3.434 treesit 切片 1（C 29-946）：动态加载、语言缓存与 `treesit_load_language`（2026-10-05）**：
  ① **区间**：C 29-946（收在 `treesit_load_language` 的 `}`）；**下一个活定义 = C 947**。31 行区间表覆盖
     C 1-5,634（逐块标"本轮转写／仍欠"）。块：文件门 `#if HAVE_TREE_SITTER`、60 条 `DEF_DLL_FN` +
     `init_treesit_functions`、60 条 `#define` 改名长跑（记账）、文件自带的 Commentary、19 个
     `Vtreesit_str_*`、三个缓存对象、`treesit_lang_cache_get/_put`、`resolve_language_symbol`、
     `treesit_load_language` 与两个 `treesit_loaded_lang`/`treesit_lang_cache_entry` 布局断言（clang 实测 16/8、24/8）。
  ② **行数**：`femacs/src/treesit.rs` **48 → 1,216**；台账 `docs/ports/treesit.md` 37 → **279**。
  ③ **读数**：冻结 `build-tu` **exit 0**，对象 **2,656 字节**，`nm -g` 只有 `_Ftreesit_available_p` 与
     `_syms_of_treesit`（`ts_`/`TREESIT` 符号 0 ⇒ 门内一字未漏进冻结配置）；`have_tree_sitter` 臂**只有
     文本级证据**——本机**没有 libtree-sitter**（头、库、`pkg-config` 全无），且裸 `--cfg` 会撞
     `include/buffer.rs` 1222 的"特性↔配置"断言（忠实的翻臂是 `xtask dead-arms`，本期未跑）；臂错误
     从首读 39 条降到 **34 条**（25 条在本单元正文，9 条在 buffer.rs 副本）；`extract-abi --check`
     **70 → 53 行**（`- ` 行即剩余工作）；`check-dead-file-coverage` 31 行/0 failure、`check-declarations`
     10/0/0（本提交后）、`check-port-lines` 把 `src/treesit.c` 从棘轮里退出（`stale`）。
  ④ **真缺陷（两条）**：⑴ **参照自己的一处**：`emacs/src/treesit.c` 864-941 用了 `dynlib_handle_ptr`
     与四个 `dynlib_*`，而它 include 的五个头**没有任何一条声明它们**（冻结配置永不编译该区，故快照容忍）；
     端口按 P-26 从属主镜像取，台账 `## Findings` 记名。⑵ 本轮自造并被翻臂读数量出的三条已修
     （`BUF_Z` 是宏 ⇒ `BUF_Z!`、`xpalloc` 的 size 要 `as ptrdiff_t`、`treesit.h` 镜像边会与单元定义的
     `syms_of_treesit` 撞 `E0428` ⇒ 撤边）。
  ⑤ **集成者本轮替它落的账**：`femacs/xtask/Cargo.toml` 登记 **`treesit_language_version_ge_15`** 与
     **`have_ts_tree_cursor_goto_previous_sibling`**（随 3.437 一起入库）。**欠账（点名）**：本文件改名
     60 个入口点，按 P-22 后续每处调用点要一对 `#[cfg]` 或每名一个 wrapper——**需 PORTING.md §11 裁定**；
     `docs/ports/line-drift-tolerance.tsv` 145 的 `treesit` 行仍是 P-08 时期的 `live` 基准（本文件现已
     **照录**死区，基准应在收尾时改为 `total`；按现行裁决"只给已完成文件写行"，本轮不动）。
  ⑥ **未声称**：**零运行证据**——tree-sitter 臂**一次都没编过**（无头、无库），本机也没有任何 tree-sitter
     运行时；C 947-5,634（4 个 DEFUN、linecol/parsing/query/navigation 各段与 `syms_of_treesit` 的门内区）
     **未移植**；`build-tu` 3 发（限 3）。

- **3.435 xdisp 第三十三轮：`move-point-visually`（C 27181-27636，456 行）（2026-10-05）**：
  ① **区间/块**：一个 DEFUN 一次落完（`Fmove_point_visually`/`Smove_point_visually`，doc `3a8d3658283b`）。
     块：函数内宏 `ROW_GLYPH_NEWLINE_P`（C 27196-27202 → 函数作用域 `macro_rules!`，参数 `$row`/`$glyph`）、
     字形路径的三臂（`BUFFERP`、overlay-string、`ROW_GLYPH_NEWLINE_P`）+ 两个方向的字形扫描、
     **标号 `simulate_display:`**（C 27373，由 2 处前向 `goto` 与 `if` 落空到达）里的 `struct it` 竞技场、
     **标号 `reseat:`**（C 27421，1 处后向 `goto`）、`PRODUCE_GLYPHS` 取宽、overshoot/bidi 修正、
     TTY `WORD_WRAP` 的 `SAVE_IT`/`RESTORE_IT` 探测、GUI 单字素行走、`SET_PT_BOTH`/`bidi_shelve_cache`。
  ② **行数**：`femacs/src/xdisp.rs` **25,908 → 26,406**（md5 `e8d54bdc…`）；台账 3,467 → **3,611**；
     覆盖区间 **C 468-27179 → C 468-27636**，**下一个活定义 = C 27637 `bidi-resolved-levels`**。
  ③ **读数**：冻结 `build-tu` **exit 0**（**4 发，限 3，超 1 如实报**：一发是 56 处 `E0614`、一发因
     3.432 的 `core::assert!` 扫过本单元而重跑、一发因注释行号修正而重跑，后三发对象逐字节相同），对象
     **2,738,320 字节**（HEAD 2,684,688），`nm` 见 `_Fmove_point_visually` 且**无新未定义符号**；
     `extract-abi --check` **70 → 69 行**（退掉 `missing subr Fmove_point_visually` 与
     `missing macro ROW_GLYPH_NEWLINE_P`，新增唯一一条 `subr Fmove_point_visually does not mention goto`
     ——**集成者本轮补了 `macro-body-notes.tsv` 的那一行**，补后 `--check` 落到 68 行、`does not mention` 归零）；
     `check-ledger-ticks`/`check-dead-file-coverage`/`check-declarations`/`check-mirror-dupes`/`check-carried`/
     `check-citations` 全部 0 failure；行数漂移 33.7% 是**既有**读数，不是本轮新错。
  ④ **真缺陷**：移植文本本身**未发现**（控制流、语句顺序、名字与错误模型逐条对齐）；lane 自己修掉 4 处
     注释里的 C 行号抄错，以及一处台账措辞——后者曾让 `check-dead-file-coverage` 把 `docs/ports/xdisp.md`
     当成 P-86 区间表并要求行（改措辞后回到 0 failure）。
  ⑤ **点名（留给镜像轮）**：`include/window.rs` 的 `struct window.current_matrix` 仍是 `*mut c_void`
     （本单元被迫三处 `as *mut glyph_matrix` 强转，deviation 263；`set_cursor_from_row`/`display_line`
     是同一个 workaround）；`include/dispextern.rs` 的 `ITERATOR_AT_END_OF_LINE_P` 仍未镜像（端口就地定义）。
  ⑥ **未声称**：**零运行证据**（没有 `move-point-visually` 的光标视觉移动、`simulate_display` 回退、
     bidi 段向翻转、GUI 字素行走、`WORD_WRAP` 换行探测的任何验证）；`missing defsubr (&raw const
     Smove_point_visually)` 与另外 12 条要到 `syms_of_xdisp`（C 38469 起）落笔才退；`probe` 0/3。

- **3.436 comp 切片 1（C 25-822）：DLL 加载表、`comp_t` 与 `hash_native_abi`（2026-10-05）**：
  ① **区间**：C 25-822（收在 `hash_native_abi` 的 `}`）；**边界停在 C 824**——`freloc_check_fill` 是全文
     第一处需要 `FOR_EACH_TAIL`（`excluded-macros.txt` 的 `kind=deferred`，P-22 不许开工）。台账把旧结论
     "dead text, recorded, not transcribed"（P-08）按 P-86 推翻为**照录、两臂都带**。
  ② **行数**：`femacs/src/comp.rs` **48 → 1,194**（145 条 `#[cfg]`、9 个 feature 名）；台账 62 → **277**
     （31 行区间表覆盖 `comp.c` 5,857 行与 `comp.h` 106 行）。
  ③ **读数**：冻结 `build-tu` **exit 0**，对象 **2,784 字节 / sha256 `39e24211…`，与改前逐字节相同**
     （照录没往冻结配置漏进一行），2 条 warning 全在镜像；`check-cond-eval` **60 条件 / 0 unknown /
     0 disagreements [VERIFIED]**；`have_native_comp` 臂 **84 错**：83×`E0425 cannot find type`
     （libgccjit 的 8 个不透明类型与 10 个按值枚举）+ 1×`E0080`（`include/lisp.rs` 2853 的"特性↔配置"断言：
     裸 `--cfg` 按设计编不动该臂，忠实的翻臂是 `xtask dead-arms`，本期未跑）；`extract-abi --check` 唯一
     一条 `- ` 是 `line count drifted 2553.3%`；`check-dead-file-coverage`/`check-declarations`/
     `check-ledger-ticks`/`check-globals` 均 0 failure；`check-port-lines` 把 `src/comp.c` 退出棘轮。
  ④ **真缺陷（7 条，已修）**：12 条原型缺 `unsafe extern "C"` 块；`format_string` 是 C 变参 ⇒ 必须
     `unsafe extern "C" fn` + `let ap: core::ffi::VaList<'_> = args;`；`emacs_fclose` 是镜像里的宏 ⇒ `emacs_fclose!`；
     `comp_hash_source_file` 的 `int res` 在 C 里声明在 `#ifdef` 臂内而作用域是外层块（Rust `let` 不跨臂）；
     include 图缺 `character.rs`（buffer.h 的 `CHAR_STRING` 家族）；`load_gccjit_if_necessary` 在 C 里是
     `static`，首版写成 `#[unsafe(no_mangle)]` 被 `--check` 的 "invented exported symbol" 抓住；
     `set_internal`/`vsnprintf` 是按 P-72 规则 4 在使用点声明的镜像缺口。
  ⑤ **点名**：**本机有 libgccjit 16.2.0**（`/opt/homebrew/include/libgccjit.h` 与 `libgccjit.dylib`）——
     任务前提被实测推翻：缺的不是 SDK，而是本单元要自带的那段**声明段**（P-88 的 `<zlib.h>` 路径）。
     libgccjit 的 8 个不透明类型 + 10 个枚举 + 4 个 `LIBGCCJIT_HAVE_*` **故意不发明**（P-88 要求 clang 量布局，
     本机没有可量它们的头）；`comp_t` 的两个形状用 clang 实测（2680/8 与 736/8）并各配 `const _` 断言。
     **集成者本轮替它落的账**：`femacs/xtask/Cargo.toml` 登记 `have_zlib`、`ns_self_contained`、
     `lisp_object_is_struct`（`have__setjmp` 本切片只在注释里出现，**未登记**——不给没人用的名字占位）。
  ⑥ **未声称**：**零运行证据**（没有链接、没有启动、没有执行任何 Lisp/C 路径；冻结档只证明"产物与改前逐字节
     相同"）；臂读数只到**名字解析**阶段且撞镜像断言，**不构成两臂可编**；C 824-5,857（`FOR_EACH_TAIL`、
     emit/gimple 段、`load_comp_unit`、`syms_of_comp` 的门内区）**未照录**；**预算如实报**：`build-tu` 5 发、
     `probe` 5 发（限各 3）。

- **3.437 pgtkterm 切片 1（C 1-1078）：设备回调族、`pgtk_free_frame_resources` 与帧几何（2026-10-05）**：
  ① **区间**：C 1-1078（收在 `pgtk_set_no_accept_focus` 的 `}`）；**下一个活定义 = C 1080 `pgtk_set_z_group`**。
     34 个函数全部落成（与 C 同名同序）、8 个文件级对象、31 个 `macro_rules!`；33 行区间表同时覆盖
     `pgtkterm.c` 7,780 行与 `pgtkterm.h` 681 行。
  ② **行数**：`femacs/src/pgtkterm.rs` **11 → 2,044**；台账 `docs/ports/pgtkterm.md` 24 → **333**。
  ③ **读数**：冻结 `build-tu` **exit 0 / 336 字节空对象**（每项都在 `have_pgtk` 之后）；`check-cond-eval`
     **31 条件 / 0 unknown / 0 disagreements [VERIFIED]**；PGTK 臂 **text-only**：`have_pgtk + have_window_system`
     读 155-157 条错，**全部可分类**——143 条是 `include/pgtkterm.rs` 占位镜像欠的三条记录
     （`pgtk_output` 122 / `pgtk_display_info` 14 / `pgtk_device_t` 7）、4 条是定义在 C 1080 之后的调用、
     8 条是 `include/font.rs` **首次**被 window-system 臂编到的布局断言、`lisp.rs` 的 `VaList` 与 `xwidget.rs`
     的 unsafe 各 1 条；**本切片自身的语句级错误 0 条**；`extract-abi --check` 195 条 `- `（192 条是未落
     remainder 的覆盖陈述，真正要基线的只有 2 条 token + 1 条 drift）；`check-dead-file-coverage` 29/0、
     `check-declarations` 10/0/0、`check-port-lines` 把 `src/pgtkterm.c` 退出棘轮。
  ④ **真缺陷**：⑴ `pgtk_new_font` 在 C 875-876 是 **static**，首版写成 `#[unsafe(no_mangle)] pub`，
     被 `--check` 的 "invented exported symbol" 抓住（已改）；⑵ `FRAME_OUTPUT_DATA` 漏译（错写成
     `FRAME_X_OUTPUT`），被 token 规则抓住；⑶ 三条**镜像侧**缺陷首次被编到（`include/font.rs` 的 8 条布局
     断言、`include/lisp.rs` 3756 的 `VaList` 需要 unstable `c_variadic`、`include/xwidget.rs` 203 在 unsafe
     块外解引用裸指针）——是**发现**，归各自镜像轮，已入台账 `## Owed`；⑷ 自伤一处已修：块注释里引 C 的
     `/*` 触发 `E0758`（与 `docs/ports/w32.md` 记录的首发缺陷同类）。
  ⑤ **集成者本轮替它落的账**：`docs/ports/macro-body-notes.tsv` 两行（`CLEAR_IF_EQ` → `0,false`；
     `pgtk_free_frame_resources` → `false`），`femacs/xtask/Cargo.toml` 登记 **`gdk_windowing_wayland`**。
     **本提交同时把四个 landing 用到的 feature 名一次登记完**（`have_ts_tree_cursor_goto_previous_sibling`、
     `have_zlib`、`libgccjit_have_{gcc_jit_context_new_bitcast,gcc_jit_global_set_initializer,gcc_jit_version,reflection}`、
     `lisp_object_is_struct`、`ns_self_contained`、`treesit_language_version_ge_15`、
     `gdk_windowing_wayland`，共十个）——Cargo.toml 是一个共享文件，按文件落而不是按单元落。
     **欠账（点名）**：`include/pgtkterm.rs`/`include/gtkutil.rs` 的占位记录要搬回；`GtkAllocation` 布局、
     `GdkInputSource` 宽度、`GTK_STYLE_PROVIDER_PRIORITY_USER` 的值需 GTK3 主机的 clang 实测（本机**没有 GTK 树**
     且 `pkg-config` 不存在；cairo 1.18.4 与 GLib 2.90.0 在，`GList` 是 clang 实测 24/8）。
  ⑥ **未声称**：**零运行证据**——本机没有 GTK 树也没有 PGTK 目标，冻结读数是 336 字节空对象（不编任何新文本），
     臂读数只是 rustc 在**名字解析/类型**阶段停下的错误清单，"本切片自身 0 错"是很弱的读数（P-86 ⑦ 的变异
     实验正是说明这类读数不能当编译通过）；C 1079-7,780（`pgtk_set_z_group` 起）**未移植**；**预算如实报**：
     `build-tu` 6 发（限 3）、`probe` 3 发（限内）。

- **3.438 本批收口：四个 landing 一起验，棘轮退 6 行，进度 59.6%（2026-10-05）**：
  ① **本批入库**：3.433 版本声明第五半（6 处 / 4 单元）、3.434 `treesit.rs` 切片 1（C 29-946）、
     3.435 `xdisp.rs` 第三十三轮（C 27181-27636）、3.436 `comp.rs` 切片 1（C 25-822）、
     3.437 `pgtkterm.rs` 切片 1（C 1-1078）。四个 landing 的共用项按文件落：`femacs/xtask/Cargo.toml`
     一次登记十个 feature 名，`docs/ports/macro-body-notes.tsv` 三行（xdisp 1、pgtkterm 2）。
  ② **P-87 全树扫描（本批的主判据，含四个 landing 与版本声明改写）**：
     `CARGO_TARGET_DIR=/tmp/femacs-sweep tools/check-all-units.sh` → **`checked 83 unit(s), 0 failure(s)` +
     `checking 83 unit(s), 0 unit failure(s)`**（两次腿都跑完，不是停在清单步的那种读数）。
  ③ **其余门**：`check-mirror-dupes` 19 dup / 19 ratchet / 0 failure、`check-declarations` **10 行 / 0 forbidden /
     0 failures**（3.433 之后的新基线）、`check-carried` 11/11/0、`check-ledger-ticks` **234 行 / 0 failure**、
     `check-dead-file-coverage` **29 ledger / 0 failure**、`check-target-names` 628 .rs / 591 targets / 0 finding、
     `check-verbatim` 6 trees / 4,604 files / 0 failures、`check-citations` 675 / 0 drift / 0 stale。
  ④ **进度与棘轮**：`tools/gen-port-map.py` → **404,070 / 678,320 = 59.6%**（3.415 时是 391,172 = 57.7%）；
     `tools/check-port-lines.py --write-pending` 把棘轮从 **85 → 79 行**、**0 new / 0 stale**（退掉的 6 行是
     本批与前三批落地的 `src/{comp,treesit,pgtkterm,w32,xfns,androidvfs}.c`）；`tools/gen-port-inventory.py`
     → **590 files, done 587 / wip 3 / todo 0**；`tools/extract-abi.py --all` → **574 done / 2 in progress /
     14 failing**（那 14 个的 `- ` 行就是各自的剩余工作清单，是中途态而不是新错）。
  ⑤ **未声称**：本批**没有任何运行证据**——83 个单元只到**编译成对象**这一档（没有链接、没有启动一个程序、
     没有跑过一条 Lisp）；59.6% 是**行数口径**的进度，不是"功能完成度"；`docs/goal.md` §8 的加载阶梯
     L0-L5、双向兼容、生成物逐字节、平台冒烟、性能**一条都未点亮**；四个 landing 各自的 `build-tu`/`probe`
     超支已在各自段落如实报（本批合计 `build-tu` 4+3+5+6 = 18 发，`probe` 0+0+5+3 = 8 发，限各 3 是**每轮**的额度）。

- **3.439 P-90 定案：第三方 SDK 表面的声明只有一个家（2026-10-05）**：
  ① **触发**：`pgtkterm`/`gtkutil`/`w32proc`/`sfntfont` 四条平台 lane 同时开工，GTK/GDK/GLib/cairo 表面第一次
     出现**第二个消费者**——三个先例（`femacs/src/include/xlib.rs` 的 X11、`femacs/src/include/jni.rs` 的 NDK、
     `femacs/lib/include/haiku_support.rs` 的 Haiku SDK）都是**专用 SDK 镜像**（首行声明 + 自己的台账），
     而 P-88 ① 说"写在单元自己里"；两份同名声明进同一个 TU 就是 `E0428`，所以必须裁一次。
  ② **规则**：判据是"这个表面有几个消费者"——只有一个 ⇒ 单元里就地声明（P-88 ① 的原意）；**两个及以上 ⇒
     一个专用镜像 `femacs/src/include/<sdk>.rs`**（首行按 xlib 那一行写，台账 `docs/ports/include-<sdk>.md`）；
     一个表面只许一个镜像；单元的段内副本按 P-88 ⑤ 搬进去并**在同一提交里删掉**（3.408→3.409 的 move-back 形状）；
     SDK 不在本机的部分点名 `UNVERIFIED` 并写明缺哪个工具链（不许凭记忆写数字）。
  ③ **本轮不追溯**：`src/pgtkterm.rs` 的 cairo/GLib/GDK/GTK 段是先落地的段内副本，搬运欠给"建
     `include/gtk.rs`"的那一轮——已写进 `docs/ports/pgtkterm.md`、`docs/ports/include-pgtkterm.md`、
     `docs/ports/include-gtkutil.md` 的 `## Owed`；在飞的四条 lane 已按"就地声明 + `## Owed` 点名"继续，
     不会造出第二份镜像。
  ④ **读数**：本段**不改任何代码**，只改 `PORTING.md` 与三个台账 ⇒ `check-declarations` **10 行 / 0 forbidden /
     0 failures**、`check-ledger-ticks` **234 行 / 0 failure**、`check-dead-file-coverage` **29 ledger / 0 failure**，
     与 3.438 同（这是"改动没碰到被这些门读的东西"的证据，不是新的功能读数）。
  ⑤ **未声称**：这是一条**规则**，不是实现——`femacs/src/include/gtk.rs` 与那次 move-back **尚未落地**；
     本机没有 GTK 头（`/opt/homebrew/include/gtk-3.0` 不存在、`pkg-config` 不存在），所以 GTK 侧布局在搬运
     那一轮仍是 `UNVERIFIED`，PGTK 臂在此之前也编不动（`include/pgtkterm.rs` 欠的记录还在欠着）。
