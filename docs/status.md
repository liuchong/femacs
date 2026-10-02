# femacs 进度与未决项（手工维护，生成物见 port-map.md / port-inventory.md）

本文件是**唯一**手工维护的进度账：每轮开工前先读它，收尾时更新它。逐文件的勾选清单在
`docs/ports/*.md`（由 `tools/extract-abi.py <c>` 生成后手勾），文件/行数总账由
`tools/gen-port-map.py`、`tools/gen-port-inventory.py` 生成，禁止手改。

## 1. 当前数字（每次提交后核对）

- 已竣工单元：`src/` 6 个（data.c 4,400 / macros.c 444 / undo.c 490 / category.c 506 /
  tparam.c 284〔见 §3.1〕/ vm-limit.c 220），`lib/` 24 个（gnulib 包装与替换件）。
- 已竣工镜像：`src/include/` 与 `lib/include/` 共 11 个（font.h、thread.h、comp.h、
  macros.h、category.h、tparam.h、sys_resource.h 及既有若干）。
- `python3 tools/extract-abi.py --all`：**0 failing**；done/in-progress 的数字以它为准。
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

- **3.1 `tparam.c` 的行数漂移（唯一未清的 `--check` 差异）**：C 284 行 vs 端口 383 行，
  因三处 label + 四处 fallthrough 必须写成"共享语句一份 + 入口阶段变量"。台账
  `docs/ports/tparam.md` 保留一个未勾选框，故 `--all` 记 WIP 而非 FAIL。修法二选一：
  压缩端口排版，或让漂移规则认识"结构化展开"（P-76 的同类工具修正）。
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
- **3.6 下一批候选**（依赖已就绪、体量可控）：`src/cm.c`(467，需 cm.h/termchar.h/
  sysstdio.h 三镜像)、`src/decompress.c`(349，先核 `HAVE_ZLIB`/`HAVE_NATIVE_COMP`)、
  `lib/allocator.c`(22)、`lib/futimens` 家族完成后的 `lib/utimens.c`(690)。
