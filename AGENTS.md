# AGENTS.md —— femacs 常驻规则

> 本文件是**常驻规则的唯一落点**，开工前先读（含上下文被压缩或换执行者之后）。
> 它只写**不可协商的约束 + 指向权威文档的指针**，细节不复述；冲突时**以权威文档为准**，
> 并在同一轮修回本文件（裁定见 `PORTING.md` P-103）。
> 规则变更：只改本文件，并在 `PORTING.md` 记一条裁定（P-1xx）、在 `docs/status.md` 记一轮读数。

## 0 权威文档与开工自检

| 文件 | 管什么 |
| --- | --- |
| `docs/goal.md` | 可粘贴的 goal：目标、硬约束、DoD（§10）、停止条件（§9） |
| `PORTING.md` | C → Rust 映射的**裁决依据**，P-01…P-103 已知偏离项与裁定（§9 未决项表、§0 近期裁定表） |
| `docs/port-master-plan.md` / `docs/port-plan.md` | 总计划（规模、方法论、批次、闸门、风险）；执行手册（D1–D12、逐文件循环） |
| `docs/status.md` | 唯一的手工进度账：逐轮读数、装置配方、下一轮点名 |
| `docs/port-inventory.md` / `docs/port-map.md` / `LIFETIMES.tsv` | 生成物台账（**禁止手改**） |
| `docs/retired-scaffolding.md` | 丢弃日志（P-102 ④） |
| `docs/acceptance.md` / `docs/port-plan.md` §7.1 | 验收阶梯 L0–L5 |

开工先跑这三条，把读数记进本轮汇报：

```sh
git status --short && git log --oneline -5
python3 tools/extract-abi.py --all          # 覆盖清单：必须 0 failing
git -C emacs status --short                 # 参照树只读：必须为空
```

常用调用（唯一入口；`<cmd>` ∈ `gen-config | layout-asserts | layout-check | build-tu <c-file> | gate-fast | gate-probes | dead-arms | gate-load | link-program <c-file> | gate-full`）：

```sh
cd femacs && CARGO_TARGET_DIR=/tmp/xt-cargo cargo run -q -p xtask -- <cmd>
```

参照程序：状态相消对照用**无 dump** 的 `build/emacs-ref/src/temacs`；带 dump 的是 `build/emacs-ref/src/emacs`。

参照基线：`emacs/` 检出 `fd7afced403`，**只读**、git 忽略。

## 1 复刻口径（不可协商）

1. **一比一**：同名文件/函数/宏/结构体/字段/全局/subr、同语句顺序、同控制流、同错误模型、同内存布局。
   目标路径规则**唯一实现在** `tools/gen-port-map.py`（不要照抄文档里的规则）。
2. **C 是唯一的行为依据**：不许发明 C 里没有的行为，不许"更 Rust"地改设计。
3. **`unsafe` 允许且必要；能 safe 就 safe**：safe 的地方不要滥用 `unsafe`，但安全抽象**不得**改变 ABI、布局、求值顺序或语义。
4. **布局与内建**：一律 `#[repr(C)]`，偏移/大小/位域/tag 位与 C 完全一致；`overflow-checks` **保持开启**（P-96）；`panic = "abort"`；**禁** `#[global_allocator]`。
5. **禁 stub / 禁吞错**：`todo!()`、`unimplemented!()`、空函数体、返回默认值、注释掉的代码；不许忽略 C 的返回码与错误路径；没有对应 C 逻辑就不许加 `unwrap()`。
6. **参照树只读**：`git -C emacs status --short` 必须始终为空；**任何写盘工具必须先断言输出不在 `emacs/` 下**。
7. **原样复制，一个字不改**：`lisp/`、`leim/`、`test/`、`etc/`、`doc/`、`admin/` 的数据。**不要花 token 重写 Lisp**；测试失败改实现，**不许**改测试、不许跳过或删除。
8. **非 C 原生代码用新技术**：`src/*.m` → **Swift**（不用 Objective-C）、`java/**` → Kotlin、`src/haiku_*.cc` → 现代 C++、汇编照旧；**能用 Rust 就用 Rust**。
9. **不混合链接**：不做 C/Rust 混编的中间态二进制，不做 shim/适配层；构建脚手架（autotools、`Makefile.in`、`configure.ac`）不移植，由 Cargo workspace + `femacs/xtask` 取代。
10. **产品名与版本声明**：端口自己的文档、注释、提交信息里**不写参照的产品名**（法律风险），一律写"参照 / the reference"；这道门是 `tools/check-own-voice.py`。
11. **实验纪律**：任何"修法实验"先在 `PORTING.md` 立项（P-1xx）再动手；被否证的版本**连同它带来的白名单/基线/工具改动一并回退**（还原见 §4），回退后**四门复绿**才允许继续推进（P-118）。

## 2 工具与语言政策（P-102）

- 只允许两种工具语言：构建/链接/闸门 = **Rust**（`femacs/xtask`）；生成物、清单、比对、门 = **Python**（`tools/`）。**不新增第三种**。
- **确实必需的保留**：驱动器、"与参照配方的适配器"（例 `femacs/admin/charsets/*.awk`，它们是参照配方的输入，换语言会改变生成物字节）、`femacs/test/**` 下的测试夹具。
- **过时的丢弃**：参照自己的构建脚手架在端口里没有读者，直接丢弃——但**必须逐条登记**到 `docs/retired-scaffolding.md`（路径 / 原用途 / 取代者 / 日期 / 提交），**不许静默删除**。

## 3 编译与资源纪律

- **不要频繁编译**：单文件用 `xtask build-tu <c-file>`；批次末才 `xtask gate-fast` / `gate-probes`；不要反复跑全量 `cargo build` / `make`。
- 产物用独立目录（`CARGO_TARGET_DIR=/tmp/...` 或仓库内 `build/`），**及时清理**（`build/`、`femacs/target/` 是生成物，可重建）。
- **不要乱扫文件、不要全盘扫描**：只读与当前任务相关的目录/文件。
- **不要起大内存任务**：统计/比对脚本必须流式、分文件、有界；历史上一个 Python 脚本吃光内存导致工作进程被中断。
- **受控执行器是唯一入口**：一切编译、链接、测试、解释器与行为未知的程序都经
  `python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile build|runtime -- sh -c '…'`；
  超限就**缩小范围**，**不许加额、不许绕开监控**（P-118）。
- 规模口径固定：`cloc --quiet femacs`（**不要**量整仓库，`build/` 下生成的 `.rs` 会让数字乱跳）。
- **环境会清产物**：`build/femacs/*.o` 与 `build/emacs-ref/lib/libgnu.a` 常被环境清空 ⇒ 任何 relink 前都要先全量重编 680 单元（约 10 分钟）；`libgnu.a` 可 `make -C build/emacs-ref/lib libgnu.a` 单独 4 秒重建。**需要 relink 的探针**（插桩、诊断打印等）**要折进已经要重建的那一轮**，不为它单开一轮。

## 4 git 纪律

- **提交必须签名**（`git commit -S`；提交信息写进文件，不用 heredoc/引号）。无法签名就**停下报告**，不许绕过。
- **每个文件一次提交**（`port(NAME): …` / `tools: …` / `docs(status): …`），并**及时推送** `origin/master`。
- **禁止破坏性 git**：`stash`、`reset`、`checkout --`、`rebase`、`push --force`、`amend`。
- **还原文件的唯一姿势**：`git show HEAD:<路径> > <路径>`；`git checkout --` 在禁止列表里（历史上犯过一次）。
- 提交内容里**不许出现绝对路径**、不许提交 secrets；保留无关 WIP 与真实作者。

## 5 汇报纪律

- 中文、**简洁**、读数为王：数字 + 结论 + 缺口，不写长文。
- **频繁阶段性汇报**（每批或每 10 个文件，先到者为准）；被问总进度时给**百分比 + 剩余工作量 + 剩余比例**。
- 进度三口径：`python3 tools/extract-abi.py --all`（覆盖）、`cloc --quiet femacs`（规模）、`docs/status.md`（本轮读数）。

## 6 当前阶段与优先级（入口：`docs/current-state.md`）

**先读 `docs/current-state.md`** —— 那一页是用户已定方向、8 条完成定义判定与复跑命令的唯一快照；本段只保留长期规则，具体读数以快照与 `docs/status.md` 末轮为准。

### 长期规则（2026-10-16 起）

- 转写清单 **590/590 完成**（`extract-abi.py --all` = 590 done / 0 WIP / 0 failing）；当前处**行为级验收**相，DoD 见 `docs/goal.md` §10。
- **优先推进覆盖与代码量**：先把覆盖率推到 100% 阶段完成，之后才集中做问题修复、整合构建与全面验收；**不要做无谓评审、不要过早精修**。
- **复刻期与 C 严格同形**：文件名/结构/命名/行序都照参照；要不要"更 Rust"地调整，留到全部重写完成后的**全面验收阶段**再判断，之前**不许顺手改好**（P-118）。
- 验收以**测试与机器门的实际读数**为准（`gate-fast`、`gate-load`、L0–L5、ERT、逐字节比对），不以人工审查代替。
- 剩余欠账与最近读数：读 `docs/current-state.md` 的**欠账清单**（逐条「现状 / 证据 / 复跑命令」，2026-10-20 复核过一轮）与 `docs/status.md` 末轮记录。

- **用完即清（P-190）**：`xtask dump` 每轮新增一对 `build/port/src/emacs-32.0.50.N{,.pdmp}`（~16 MB），加上 `build/*.dbg*`、`build/probes`、探针目录与 `/tmp` 一次性临时树，**用完即删**；保留在用项 = `build/port/src/{temacs,emacs,emacs.pdmp,bootstrap-emacs.pdmp}`、`build/femacs/*.o` 与 `units/`、`build/emacs-ref/**`、`build/lisp-l1`。每轮收尾 `du -sh build/* | sort -h` 看一眼；删前 `grep -rl <名> tools femacs/xtask docs` 确认没人读；删后复验 dump + smoke，并把释放量记进 `docs/status.md`。

## 7 速查

- **收尾复验 = 一条命令**：`python3 ~/.assistant/skills/controlled-run/scripts/run.py --profile runtime -- bash tools/verify-dod.sh --full`
  —— 按 `docs/current-state.md` 的八行逐项打印 `ok`/`FAIL`（4.981 实测全绿，1884 s / 峰值 1.12 GB），
  缺 `libgnu.a` / `asset-directory-tool` 时自愈；第 7 行不运行且会打印"按用户决定暂挂、记未达"。
- **两条长扫**：`tools/l5-sweep.sh [--gc-stress] [--start-at N] [--chunk N]` —— L5 与强制 GC 压测共用，
  分块可续跑、每块独立 TSV；跑完把分块合成一张总表后删分块（P-190）。
- **禁止启用 Codex Fast mode**：不得 `/fast on`、不得 `service_tier = "fast"`、不得 `[features].fast_mode = true`（无例外硬规则）。
- 本项目是**个人项目**：本仓库任何文件（代码、注释、文档、提交信息、PR/changelog）里**不得**出现公司项目的名称、路径、链接或可推断的暗示。
