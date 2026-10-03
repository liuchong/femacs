# femacs 进度与未决项（手工维护，生成物见 port-map.md / port-inventory.md）

本文件是**唯一**手工维护的进度账：每轮开工前先读它，收尾时更新它。逐文件的勾选清单在
`docs/ports/*.md`（由 `tools/extract-abi.py <c>` 生成后手勾），文件/行数总账由
`tools/gen-port-map.py`、`tools/gen-port-inventory.py` 生成，禁止手改。

## 1. 当前数字（每次提交后核对）

- 已竣工单元：`src/` 11 个（data.c 4,400 / macros.c 444 / undo.c 490 / category.c 506 /
  tparam.c 284 / vm-limit.c 220 / itree.c 1,435 / marker.c 874 / json.c 1,851 /
  callint.c 933 / terminfo.c 55），`lib/` 37 个
  （gnulib 包装与替换件，外加实现单元 rawmemchr.c 158 / memrchr.c 158 / malloc.c 56 /
  realloc.c 101 / lstat.c 97 / fstatat.c 150 / readlink.c 114 / symlink.c 57 /
  readlinkat.c 123 / cloexec.c 83 / filemode.c 166 / mini-gmp.c 4,699 / md5.c 394 /
  sha1.c 362 / sha256.c 431 / sha3.c 442 / sha512.c 476 / save-cwd.c 90 /
  copy-file-range.c 99 / md5-stream.c 134 / lchmod.c 97）。
- 镜像（`src/include/` 与 `lib/include/`）：`font.h`、`thread.h`、`comp.h`、`macros.h`、
  `category.h`、`tparam.h`、`sys_resource.h`、`lisp.h`、`frame.h`、`window.h`、`termhooks.h`、
  `dispextern.h`、`buffer.h`、`keyboard.h`、`coding.h`、`composite.h`、`itree.h`、`globals.h`
  等，以及 `lib/include/` 的 `sys_stat.in`、`fcntl.in`、`unistd.in`、`stat-time`、
  `sys_types.in`（含 `struct stat` 需要的六个 Darwin 宽度别名与 `uid_t`）。第 55 轮新增
  **`src/include/termchar.rs`（termchar.h 257 行，`--check` OK）**，并给 `dispextern.rs` 补
  `Mouse_HLInfo`/`SCROLL_INFINITY`/`CHECK_MATRIX`、给 `frame.rs` 补 C 同名读法
  `pub fn output_method`。
- **进度（2026-10-04 第 96 轮，两个口径都实测）**：磁盘上 **259 个单元**（`femacs/src/*.rs` 137 +
  `femacs/lib/*.rs` 122）与 **130 个镜像**（`src/include/` 41 + `lib/include/` 89）。
  **权威口径 `python3 tools/extract-abi.py --all` 本轮真跑过**：
  `checked 362 ported files: 335 done, 25 in progress, 2 failing`——两个 failing 是**正在写的两条通道**：
  `src/pdumper.c` 的单元文件已 6,708 行但台账还没生成（没有清单即"未完成"）、`src/regex-emacs.c`
  的台账被勾满而最后一次正文比对仍有差异（通道仍在收尾）。两者都是**未提交的在飞文件**，不是已提交
  状态的失败；上一次对**已提交**状态的全树复跑是 `357 ported / 333 done / 24 WIP / 0 failing`。
  本轮的落地单元：`src/minibuf.c`、`src/print.c`、`src/search.c`、`src/emacs.c`、`src/sysdep.c`、
  `src/keymap.c`、`src/bidi.c`（各带自己的台账与 §3.58–§3.67 的记录）。
  **整树门 `gate-fast` 上一次跑到底并全绿是在 248 个单元时**（第 95 轮，含 `layout-check` 791 断言 /
  0 pending）；此后落了 7 个单元与 §3.59–§3.67 的工具/镜像修复，**下一批落地后立即复跑**（在飞通道
  写同一棵树时重编译会产生假失败，故按批边界执行）。
  未端口且冻结构建会编译的 `src/*.c` 还有 **14 个**（`tools/live-rank.py` 按活行数排序：`src/font.c`
  4,662 / `src/term.c` 5,021 / `src/frame.c` 5,243 / `src/xfaces.c` 5,489 / `src/buffer.c` 5,721 /
  `src/fileio.c` 6,066 / `src/dispnew.c` 6,549 / `src/alloc.c` 6,731 / `src/fns.c` 6,917 /
  `src/process.c` 8,013 / `src/window.c` 9,600 / `src/coding.c` 11,992 / `src/keyboard.c` 13,086 /
  `src/xdisp.c` 29,836），在读的五条通道分别是 `src/editfns.c`、`src/eval.c`、`src/pdumper.c`、
  `src/regex-emacs.c`、`src/lread.c`。
- **照抄树（Verbatim）已全部落地（2026-10-03 第 79 轮）**：`emacs/{lisp,test,etc,doc,leim,admin}`
  → `femacs/<同名>`，**逐字节照抄**，共 **4,607 文件 / 约 4.6M 行**（lisp 1,627 / test 1,555 /
  etc 887 / doc 257 / leim 28 / admin 253），六个目录各一次签名提交并推送。判定不再是行文说明
  而是机器门：**`python3 tools/check-verbatim.py`**（文件清单 + 逐块 sha256 流式比对，越界即非 0），
  当前输出 `6 trees, 4607 files, 0 failures`。`test/` 与 `lisp/` 一个字节都没动（硬约束 5）。

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

