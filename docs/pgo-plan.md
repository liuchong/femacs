# PGO A/B plan（2026-10-21，status 4.1056）

本文件只记录已收敛的方案，不表示 PGO 已实现或默认启用。原 DoD-7 的判据不变：
端口性能不低于参照基线；本计划只是评估 PGO 能否在不改变逻辑的前提下缩小差距。

## 目标与非目标

- 目标：在同一源码、同一 rustc/LLVM 上比较端口 O2 与端口 PGO，并保留原始 C 参照。
- 非目标：不改 C/Rust 逻辑，不改 Elisp/兼容接口，不用更慢的参照配置达标，不自动启用
  PGO 默认构建，不自动升级 LTO/JIT/全 asm。
- 10% 不是新 DoD，也不是丢弃有效改善的门槛；是否追平参照仍按原 DoD 判。

## 训练与评测

两层语料，均预先固定并登记：

1. **真实负载层**：启动/加载、编辑与扫描、正则替换、GC 压力；同一功能/代码路径可以出现在
   训练和评测中，但具体输入、数据、操作序列必须不同。
2. **微基准层**：`tools/bench.sh` 的六项，保持既有 min-of-5 口径。

训练语料不包含评测用例的具体输入/序列；不得把 PGO 说成会记住输入结果——它只改变代码布局/
内联。训练集与评测集在测量前冻结，不中途调整。

## 测量口径

- 同一 commit、同一 rustc/LLVM 的 O2 与 PGO；参照保持原始 C `-g3 -O2` live threaded 构建。
- 预先固定重复轮数（建议 7 轮；每轮内部仍保留 min-of-5），同机交替运行并记录 load。
- 报告每轮值、min、median、MAD/IQR 与离散程度；改善/回退按分布和预注册噪声阈值判断，
  不用单个最好值下结论。
- 预注册关键任务：启动、`loadup`、编辑/扫描、正则、GC、六项 bench；任何超出噪声的关键
  回退都算失败候选。

## 构建接入与已知风险

- 当前端口逐单元 `rustc --crate-type=lib --emit=obj`，由 `xtask`/自定义 no_std driver 链接，
  不是普通 Cargo 全程序构建。
- `-Cprofile-generate` 需要 profiler runtime 与正常退出写 `.profraw`；当前 no_std driver、
  `panic=abort`、`dump` 流程尚未验证能产出/合并 profile。
- PGO 改变可执行文件后必须重新 `dump`；pdmp 指纹绑定可执行文件。
- 逐单元 profile 合并需要 `llvm-profdata` 与两阶段链接 plumbing；这是构建接入工作，
  不等于逻辑改动。
- 以上是**未验证的构建风险**，不是已证明不可行。

## 执行顺序（R39）

先完成全部 PGO 实现与构建接入，再一次性集中构建、训练、测量、正确性验收；不在中途反复
调训练集或局部试跑优化。实现阶段允许修复原授权范围内的构建/验收缺陷并做必要复验；只有
无法在既定范围解决、最终无可靠净收益，或出现未解决的正确性/关键回退时才停止。

## 停止与回退

- PGO 构建不可行，或任一 gate/ERT/smoke 失败且无法在范围内修复；
- 两个固定训练集后仍无超过噪声的净改善；
- 出现未解决的关键任务回退。

满足任一条件：保留 O2，DoD-7 记未达，不自动启用 PGO，不自动升级 LTO/JIT/全 asm。

## 代码核对结论（2026-10-21）

- **A. bench 口径**：`tools/bench.sh` 59 行写 `/tmp/bench-forms.el`，98–103 行写
  `/tmp/bench-port-all.txt` / `/tmp/bench-ref-all.txt`；`fold` 92–96 行每用例取跨轮
  min；`--gate` 136 行只比端口历史毫秒（默认 15% 容差），不是与参照的比值，也没有
  中位数/离散度。PGO 评测不能直接复用它。
- **B. 固定构建目录**：`build_tu` / `build_finished_tus` 写 `build/femacs`；
  `gate_load` 2645 起从固定 `build/femacs` 取对象并写 `build/port/src/temacs`；
  `dump_emacs` 2576 起固定 `build/port/src`，覆盖 `emacs`/`emacs.pdmp`/
  `bootstrap-emacs.pdmp`。PGO 必须使用独立 build root，不能在原目录混档。
- **C. 链接 flags**：`unit_codegen_flags` 48 行只被对象编译调用；`gate_load` 2790 起的
  最终 `rustc` 链接没有经过它。PGO generate 的 runtime/落盘必须同时贯通最终链接。
- **C2. 退出路径**：`femacs/src/emacs.rs` 2694 起的 `Fkill_emacs` 末尾调用 libc
  `exit(exit_code)`（2760 行附近），不是 `_exit`；最小 no_std 探针证明带 `-lSystem`
  链接、走 `exit()` 时 `.profraw` 能写出。`panic=abort` 不等于正常退出，但
  `kill-emacs` 的正常路径是 libc `exit`，所以“不能写 profile”不能作为普遍结论。
- **D. 派发**：`femacs/src/bytecode.rs` 648–651 行 `NEXT = continue`、`FIRST = match`；
  C 的 `BYTE_CODE_THREADED` 是 label + `goto *targets`。这是已记录的派发形状差异，
  不能仅由它断言全部性能差距根因；真实任务占比仍未知。
- **Fingerprint 缺陷**：`femacs/lib/fingerprint.rs` 是固定 32 字节占位数组；
  `xtask` 没有调用 `lib-src/make-fingerprint` 重写二进制。因此**不能依赖 dump
  fingerprint 拒绝 O2/PGO 混用**。必须用阶段目录 + binary/pdmp 的 SHA-256 清单
  绑定配对，并考虑在链接后运行端口自己的 `make-fingerprint` 让 fingerprint 真正唯一。

## 实施接线（未运行）

1. `xtask` 增加 `FEMACS_BUILD_ROOT`（默认 `build`）与 `PgoMode`：
   `off | generate:<dir> | use:<file>`；`build_tu`/`build_finished_tus`/`gate_load`/
   `dump_emacs` 全部通过根目录派生子路径。
2. 增加 `build-units` 命令：只编译全部完成单元，不跑 gate、不执行程序，避免采样污染。
3. `pgo_flags()` 同时追加到对象编译和最终 `gate_load` 链接；generate 阶段使用专用
   `LLVM_PROFILE_FILE`，gate 产生的 profile 删除后才跑固定训练脚本。
4. 阶段目录：`build/pgo/o2`、`build/pgo/generate`、`build/pgo/use`；每阶段保存
   `temacs`、`emacs`、`emacs.pdmp` 与 `pair.sha256`。训练/评测输入分离，profile merge
   只吃训练目录的明确文件。
5. 评测工具独立于 `bench.sh`：每次运行使用独立临时目录，保留每轮原始样本、min、
   median、MAD/IQR；真实负载 + 六项微基准；参照始终是原始 C `-g3 -O2` live threaded。
6. 默认 O2 不动；PGO 失败时保留 O2 与既有 dump，不自动启用 PGO、不升级 LTO/JIT/全 asm。

## 指纹与配对决定（2026-10-21）

- 事实：`femacs/lib/fingerprint.rs` 是固定 32 字节占位数组；`femacs/xtask/src/main.rs`
  的 `gate_load` 链接后没有调用 `make-fingerprint`。因此不能依赖现指纹拒绝 O2/PGO 混用。
- 参照事实：`emacs/src/Makefile.in` 697-720 在 `temacs.tmp` 链接后执行
  `MAKE_PDUMPER_FINGERPRINT $@.tmp`；端口 `femacs/lib-src/make-fingerprint.rs` 190 行起
  已有“搜索占位数组并改写二进制”的转写。
- 选择 **A**：每个阶段链接 `temacs` 后，先用 `xtask link-program lib-src/make-fingerprint.c`
  构建端口自己的 `make-fingerprint`，再对 `$phase/port/src/temacs` 执行它，最后 `dump`。
  这恢复参照的自校验机制；不混入 C 对象，不改 pdumper 语义，不动默认 O2。
- **B 作为辅助**：每阶段仍生成 `pair.sha256`（temacs/emacs/pdmp 的 SHA-256），在训练/评测
  前核对；但二进制自身的指纹校验是主防线，清单不是唯一防线。
- 风险/未验证：端口 `make-fingerprint` 是否能在最终 Mach-O 中命中占位数组并改写，需在
  集中构建阶段验证；命中失败即停止该阶段，不继续 dump。

## 完整接线设计（2026-10-21，实施前定稿）

### 目录隔离

默认 O2 构建保持原位（`build/`），PGO 各阶段只使用自己的根，互不覆盖：

```
build/                                   # O2：默认构建，保持可用
  femacs/                                # O2 单元对象；也是 make-fingerprint 的普通 O2 工具对象根
  port/src/{temacs,emacs,emacs.pdmp,bootstrap-emacs.pdmp}
  pgo/
    generate/                            # FEMACS_BUILD_ROOT
      femacs/                            # 插桩对象
      load/                              # driver/rsp/临时 archive
      port/src/{temacs,emacs,emacs.pdmp,bootstrap-emacs.pdmp,pair.sha256}
      profiles/gate/                     # gate 运行产生，永不 merge，训练前删除
      profiles/dump/                     # dump 运行产生，永不 merge，dump 后删除
      profiles/train/                    # 唯一允许 merge 的目录
      merged.profdata
    use/                                 # 结构同 generate，无 profiles
    results/{raw.tsv,summary.tsv,rounds.tsv}
```

- `xtask` 新增 `FEMACS_BUILD_ROOT`（默认 `../build`），`femacs/`、`load/`、`port/`
  全部由它派生；默认值下所有既有路径逐字节不变。
- 工具对象根独立：`FEMACS_TOOL_UNITS`（默认 `../build/femacs`）。`link-program`
  只从这里取对象、只用 `unit_codegen_flags()` 编译，**不读取也不追加 `FEMACS_PGO`**。
  因此 `make-fingerprint` 永远是普通 O2 工具，插桩阶段的 profile 不会混入工具或建 dump 过程。
- 每个阶段结束保留二进制/dump/`pair.sha256`；`profiles/gate`、`profiles/dump`、
  `load/` 下的响应文件与临时 archive 在阶段收尾清理（P-190）。

### 对象与最终链接 flags

- `PgoMode = off | generate:<dir> | use:<file>`，来自 `FEMACS_PGO`；默认 `off`。
- 对象编译（`build-tu`、`build-units`，含 `--cfg enable_checking` 的检查对象）：
  `unit_codegen_flags()` 之后追加 `-Cprofile-generate=<绝对路径>` 或
  `-Cprofile-use=<绝对路径>`。
- `gate-load` 的最终 `rustc` 链接也追加同一组 flags（generate 阶段由此把 profiler
  runtime 链入 no_std driver；既有 `-lSystem` 保留）。`use` 阶段同一函数调用，保证
  对象与链接档位不会一半一半。
- `link-program`（make-docfile、make-fingerprint 等 lib-src 工具）**不经过** PGO flags。
- 默认 O2 不设置 `FEMACS_PGO`，flags 与现状完全一致；不自动启用 PGO/LTO/JIT/全 asm。

### 训练输入与采样筛选

- 训练语料固定为 `tools/pgo/train.el`；评测语料固定为 `tools/pgo/eval.el`。两者使用
  不同的文本数据、规模与操作序列，但可覆盖相同代码路径（PGO 只改布局/内联，不记忆输入）。
- 每个子进程按角色显式设置 `LLVM_PROFILE_FILE`：
  - gate：`<root>/profiles/gate/%p-%m.profraw` → 训练前删除，永不 merge；
  - dump：`<root>/profiles/dump/%p-%m.profraw` → dump 后删除，永不 merge；
  - train：`<root>/profiles/train/%p-%m.profraw` → 唯一输入。
- `llvm-profdata merge --failure-mode=all -o merged.profdata <显式 train 文件列表>`；
  文件列表在训练结束后收集，任一文件为空/不可读、列表为空、merge 非零即失败。
- 不扫全盘、不按目录通配其他阶段的 profile；训练期间不跑 gate/bench。

### 指纹、签名与 dump 顺序

参照 `emacs/src/Makefile.in` 713-720：链接 `temacs.tmp` → `make-fingerprint` → 条件
`codesign -s - -f` → `mv`。端口在 macOS 上无条件执行等价步骤，因为改写 `__DATA`
会作废 lld 生成的 ad-hoc 签名，不重签的 arm64 二进制会被系统终止：

1. `gate-load` 链接到 `<root>/port/src/temacs`；
2. 用普通 O2 工具 `make-fingerprint` 改写该文件（每次链接只能执行一次；占位数组第二次
   已不存在，工具会报 `missing fingerprint`）；
3. `codesign -s - -f <temacs>`，随后 `codesign --verify`；
4. 才允许 `dump`：复制为 `emacs`，运行 `temacs -batch -l loadup --temacs=pdump`；
5. `dump` 结束写 `pair.sha256`（`shasum -a 256` 的 `temacs`、`emacs`、`emacs.pdmp`、
   `bootstrap-emacs.pdmp` 四行）。

指纹后置条件（任一失败即停止该阶段，不 dump）：
- 改写前全文件恰好出现 1 次 32 字节占位数组；
- `make-fingerprint -r` 输出 64 位十六进制摘要；
- 改写后占位数组出现 0 次，且原偏移处 32 字节与摘要逐字节相等；
- `codesign --verify` 成功。

`pair.sha256` 是辅助防线：训练/每轮评测前重算并比对，防止目录被旧 dump 污染；主防线仍是
pdumper 自身按指纹拒绝错配。两者都不改变 pdumper 语义，也不混入参照 C 对象。

### 原始样本与错误传播

- `tools/pgo_bench.py` 独立于 `tools/bench.sh`：每次运行在 `tempfile.mkdtemp()` 的独立
  目录内，`HOME` 指向该目录，不使用共享 `/tmp` 固定文件名；结束后只保留结果行与 stderr
  摘要，临时目录删除。
- 预注册 7 轮；每轮每侧一个进程，进程内六项微基准各自 min-of-5（保留既有口径），真实
  任务另测。同轮顺序轮换（O2→PGO→REF，下一轮 REF→PGO→O2），记录每分钟 load。
- 每行原始样本写 `raw.tsv`：`case side round value_ms wall_ms exit`；汇总写 `summary.tsv`：
  `n min median MAD IQR` 与相对参照/O2 的比值，不用单个最好值下结论。
- 任一次运行超时（300 s）、非零退出、缺少预期 `BENCH`/`TASK` 标记、stderr 出现
  fatal/abort，整轮测量失败并以非零退出；不吞错、不用部分样本生成结论。
- 正确性接线：`pgo-pipeline.py verify` 在每轮前核对 `pair.sha256`、指纹占位已消失、
  `codesign --verify`，并运行 batch smoke（启动、加载 Lisp、字节码执行、退出码 0）；
  PGO 阶段自身的 `gate-load` 已含链接检查与 coding-eol 对照，`dump` 成功是硬条件。
- 判定：只有全部正确性项通过、且相对 O2 的净改善超过测量离散度、无关键任务回退时，
  PGO 才作为候选记录；否则保留 O2、DoD-7 如实记未达。10% 不是门槛。

## 集中验证结果（2026-10-21，7 轮）

- O2 / generate / use 三阶段均完成：`gate-load` 0 undefined、coding-eol 15/15、dump 成功；
  每个阶段的 `pair.sha256`、指纹占位消失、`codesign --verify` 与 batch smoke 全部通过。
- 训练只用 `build/pgo/generate/profiles/train/` 的 1 个 `.profraw`（738,496 B）；gate/dump
  的 profile 从未 merge。匹配工具链的 `llvm-profdata`（nightly-2026-10-01 llvm-tools）
  合并出 861,776 B 的 `merged.profdata`；use 阶段以它构建并 dump。
- 原始样本/汇总保存在 `docs/baseline/pgo-20261021/{raw-pgo,summary-pgo,rounds}.tsv`（7 轮，
  每轮内 min-of-5；median/MAD/IQR 见 summary）。
- 相对 O2 的 median 比值：`bc-loop 0.790`、`bc-sum 0.903`、`bc-car 0.802`、
  `regexp-repl 0.852`、`buffer-scan 0.868`、`sort-100k 0.847`、`startup 0.969`、
  `load 0.888`、`edit 0.757`、`regex 0.835`、`gc 0.843`；11 项全部不慢于 O2。
- 相对参照仍为：bytecode 1.27–1.47×、load 2.04×、其余 0.79–1.05×；因此 DoD-7 仍未追平，
  PGO 作为可靠候选记录，但**不自动启用默认构建**，默认 O2 保持。10% 不是门槛，本轮也不据此
  宣称 DoD-7 达标。

## `target-cpu=native` A/B（2026-10-21，已否证）

在 PGO 之上仅增加 `-C target-cpu=native`（纯代码生成，不改逻辑）重建 `use` 阶段并测 7 轮：
`bc-loop 2.595 ms`（PGO 2.600）、`bc-sum 5.674`（5.610）、`bc-car 2.590`（2.601），其余项
差异同样在离散度内；相对参照仍 `bc-loop 1.46×`、`bc-sum 1.29×`、`bc-car 1.46×`。
⇒ **无可复现净收益，丢弃该候选**，默认 O2 与 PGO 候选均不变。证据：
`docs/baseline/pgo-20261021/{summary-native,raw-native,rounds-native}.tsv`。
