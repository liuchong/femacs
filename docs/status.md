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
  readlinkat.c 123 / cloexec.c 83 / filemode.c 166）。
- 已竣工镜像：`src/include/` 与 `lib/include/` 共 15 个（font.h、thread.h、comp.h、
  macros.h、category.h、tparam.h、sys_resource.h 及既有若干）。本轮新增 WIP 镜像
  `lib/include/{sys_stat.in,fcntl.in,unistd.in,stat-time}.rs`，并给 `sys_types.in.rs` 补上
  `struct stat` 需要的六个 Darwin 宽度别名（mode_t/nlink_t/blksize_t/blkcnt_t/ino_t/dev_t）与
  `uid_t`。
- `python3 tools/extract-abi.py --all`：**72 done / 21 wip / 0 failing**（93 个已有目标；
  数字随在飞工作变化，以它为准）。
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
