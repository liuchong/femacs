# femacs 进度与未决项（手工维护，生成物见 port-map.md / port-inventory.md）

本文件是**唯一**手工维护的进度账：每轮开工前先读它，收尾时更新它。逐文件的勾选清单在
`docs/ports/*.md`（由 `tools/extract-abi.py <c>` 生成后手勾），文件/行数总账由
`tools/gen-port-map.py`、`tools/gen-port-inventory.py` 生成，禁止手改。

## 1. 当前数字（每次提交后核对）

- 已竣工单元：`src/` 8 个（data.c 4,400 / macros.c 444 / undo.c 490 / category.c 506 /
  tparam.c 284 / vm-limit.c 220 / itree.c 1,435 / marker.c 874），`lib/` 26 个（gnulib 包装与
  替换件，外加两个实现单元 rawmemchr.c 158 / memrchr.c 158）。
- 已竣工镜像：`src/include/` 与 `lib/include/` 共 11 个（font.h、thread.h、comp.h、
  macros.h、category.h、tparam.h、sys_resource.h 及既有若干）。
- `python3 tools/extract-abi.py --all`：**66 ported / 54 done / 12 wip / 0 failing**；
  done/in-progress 的数字以它为准。
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
- **3.6 下一批候选**：依赖已就绪且体量大的 `lib/mini-gmp.c`(4,699)、`lib/regexec.c`(4,306)
  与 `lib/regex_internal.c`(1,718)、`src/json.c`(1,851，0 个预处理条件)、`src/itree.c`(1,435)、
  `src/callint.c`(933)、`src/marker.c`(874)；小件 `lib/getgroups.c`(121)、`lib/group-member.c`(108)、
  `lib/pselect.c`(115)、`lib/rawmemchr` 同族的 `lib/memrchr`(已做)。需要先建镜像的：
  `src/cm.c`(467，需 cm.h/sysstdio.h/termchar.h 三镜像)、`src/decompress.c`(349，需 md5.h)。
  注意 `lib/gettimeofday.c`/`pselect.c`/`pthread_sigmask.c`/`symlink.c`/`readlink.c`/`strtoimax.c`
  在冻结宏表下整支可能为死文本，动它们前先核 `docs/baseline/config/darwin-arm64.macros.tsv`。
