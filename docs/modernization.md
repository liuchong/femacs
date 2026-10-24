# Modernization: stable delivery, retirement, then Lisp runtime work

Current continuation: the **seven-part persistent Goal (2026-10-25)** below. Its first delivery stage closes the remaining Stage IV native gaps. Stages I–IV and failed attempts remain evidence; unchanged acceptance is not rerun merely to refresh a date.

## Persistent Goal: complete the seven-part modernization plan

The user explicitly requests a new native Codex Goal and continuous execution of all previously agreed work. This is a new objective; the closed first-phase CLI goal stays closed. Baseline is `971cf90d7dbe2a17dcd48c78ec639c7bec9289ba`, master, clean when this goal plan was prepared. Primary development and independent paired test/review continue in the existing conversations; private tool/session identities stay outside Git. The native Goal state and this persistent plan must both be checked on continuation; a saved `/goal` paragraph alone is not proof of an active objective.

### Full scope, sequence and finish conditions

| ID | Required outcome | Starting evidence and remaining work | Acceptance |
| --- | --- | --- | --- |
| G1 | macOS terminal and native editor reliably usable | Both products build; classic keys, files and single-window IME have bounded real evidence. Remaining: actual Finder external drop, multi-frame IME and abnormal no-controlling-TTY diagnostic/ownership repair. Do not assume an IME cause from callback counts, input-source ID or Lisp selected-frame alone. | Actual unlocked native workflows, exact PID/product identity, correct file bytes/committed text, cancellation and normal exit; malformed TTY startup fails with the correct message without duplicate cleanup or a crash. |
| G2 | Fresh tracked checkout builds and runs independently | Complete terminal/Cocoa clean-export builds already passed; retain those unchanged results. Ensure later cleanup/runtime changes preserve generated resources, root paths, ignore rules, bundle relocation and documented dependencies without optional `emacs/`, old objects or old Lisp bytecode. | Standard documented build/generation in a new tracked-only export; executable/dump/resource fingerprints, strict signature and actual terminal/native startup. Repeat only when inputs or behavior change. |
| G3 | Explicit, repeatable compatibility baseline | Existing ten-group three-product comparison, VM semantics, selected ERT, keys/files and startup checks are reusable controls. Extend for changed behavior, common config/shipped libraries and boundary failures. Coverage counts are not complete behavioral parity. | Strict case identities, zero skipped requested cases, retained failures, per-case timings and deterministic observations; exact tested version/platform/plugin scope stated. Latest-stable language compatibility requires its own bounded source/version evidence, not a renamed frozen-reference result. |
| G4 | Planned modernization and removal of obsolete infrastructure/device baggage | Six paths already retired and shell policy centralized; platform-support decisions, remaining legacy tools/generators/manual build and Android independent-config gap remain open. Inventory readers/replacements and supported-target obligations before removal; a non-Rust extension alone is no reason to delete or rewrite. | Exact dependency/reader map, supported/unsupported/retired matrix, implemented replacement paths and affected builds/workflows. Every deletion/withdrawal/cache cleanup has a committed `maintenance/cleanup` report, identities, reason, replacement and recovery route, linked from its ledger and progress log. |
| G5 | Actionable real Lisp/GC performance evidence | Seven-round workload and startup data exists but both hosts were non-quiet; no speedup claim. Add attribution for actual allocation/retention/survival, root/mutator and pause/phase costs needed by compiler/collector choices. | Immutable source/config/product inputs; separate training/evaluation data where relevant; all rounds/median/MAD/p95, workload results, load and memory limits. No toy, minimum-only statistic or busy-host run promoted as a release improvement. |
| G6 | Evidence-based interpreter/compiler and native/AOT/JIT work | The rejected shared-tail dispatcher stays rejected unless new evidence addresses its real limitation. Compare actual existing native integration and modern backend options; design runtime ABI, roots/safepoints, unwind, fallback, cache/code lifetime and redefinition before implementation. | A source-bound design and real-editor prototype/implementation with semantic and compatibility tests, startup/compile latency, steady-state speed and memory comparisons. Retain a candidate only for reliable benefit without unresolved correctness/important regression; preserve the working baseline. A rejected candidate is a documented experiment, not a shipped native/JIT feature. |
| G7 | Evidence-based collector modernization, including a generational candidate | Inventory all roots and stores, FFI/interior pointers, conservative roots, pdump objects, weak tables/finalizers and existing string compaction. Use Java/Go designs as technical references, not proof that their collector is directly reusable. | Precise chosen root/barrier/remembered-set/ownership design, real VM candidate and differential/stress/retention/pause tests, measurable tradeoffs and safe baseline fallback. If a candidate is rejected, preserve proof and recovery; do not claim a generational/moving GC shipped. |

G1/G2/G3 form the current **stable-delivery stage**. G4 is the following **cleanup/renovation stage**. G5/G6/G7 form the subsequent **Lisp runtime stage**, with prerequisite measurement/design preceding dependent implementation. These are the user's previously specified stages, not per-file test slices. Each complete stage follows A full design -> B all cases written without running -> C all implementation -> D consolidated integration, tests and independent review. Existing valid implementation/cases/evidence are reused. No new numeric speed target, backend default, platform removal or paid resource budget is invented.

Research items G6/G7 require an actual source-bound prototype and measured adoption/rejection decision; a literature summary alone is incomplete. Useful verified changes are integrated, and unsuccessful candidates are removed with a traceable report. A research decision and an enabled feature are separate statuses. Required editor/build/compatibility outcomes cannot be waived because an optimization fails. Completion of one stage does not complete the entire Goal.

### Current and target code/data flow

```text
current baseline -> required target change / design entry
platform/macos/nsterm.swift        G1: keyDown, window focus and text-input callbacks; external drag
src/term.rs                       G1: init_tty / maybe_fatal / vfatal error and cleanup path
tools/test-macos-ime.py            G1/G3: physical composition and exact native focus evidence
tools/fixtures/macos-{ime,drag}.el  G1/G3: owned buffer/frame/file observations
tools/fixtures/macos-events.rs     G1/G3: exact PID-directed native key events
tools/fixtures/macos-input-source.rs G1/G3: source/status observations, not readiness proof
tools/test-macos-delivery.py       G1/G3: key and file workflows
tools/local-acceptance.py          G1/G2/G3: source/product identities and interactive startup
xtask/src/{standalone,macos,bundle}.rs G2/G4: independent build, resources and app publication
tools/bootstrap-resources.py      G2/G4: tracked-input generation; preserve actual required recipes
tools/test-release-baseline.py    G3: representative strict ERT/observations, not full compatibility
tools/runtime-profile.py          G5: retained samples and isolated process input/measurement
tools/fixtures/runtime-profile.el G5: workload/collector instrumentation
src/{eval,bytecode,comp,alloc,lisp}.rs G6/G7: interpreter, compiler boundary, roots/stores/collector
lisp/emacs-lisp/{bytecomp,byte-opt}.el G6: existing compiler interface; behavior preserved
docs/runtime-modernization.md     G5/G6/G7: exact source-bound runtime designs and experiments
docs/retired-scaffolding.md        G4: retention/replacement policy and current retirement map
maintenance/cleanup/              G4/all: reports, manifests, inert recovery and ledger
docs/modernization.md             all: sole live scope, dependency and progress checklist
```

```mermaid
flowchart LR
  S[Tracked inputs and generated resources] --> B[Terminal and Cocoa build]
  B --> U[Existing keys files startup single-window IME]
  U --> N[Remaining real drag multi-window and TTY acceptance]
  N --> C[Dependency-based cleanup and renovation]
  C --> P[Real workload and collector attribution]
  P --> J[Interpreter compiler native prototype]
  P --> G[Root and barrier design plus collector prototype]
  J --> A[Consolidated semantic native and performance acceptance]
  G --> A
  A --> D[Signed delivery and traceable cleanup]
```

This map is a design/work index, not a claim that every displayed runtime edge has been proved by a graph. The graph service has no femacs index; exact changes use bounded current source reads. Before B/C, complete the file/symbol/variable/signature change table for the current whole stage in this document or the existing runtime design section. Future research does not fabricate an IR, collector policy or source line that has not yet been chosen and justified.

### Immediate source-bound design entries

| Change | Baseline file / symbol | Required work before implementation | Cases / dependency |
| --- | --- | --- | --- |
| GD1 | `src/term.rs:4802 init_tty`, `:5264 vfatal`, `:5277 maybe_fatal` | Trace `delete_terminal_internal` / `delete_tty` ownership and every maybe_fatal caller; replace incorrect variadic forwarding with the exact va-list interface and retain formatting/exit semantics. Specify one cleanup owner for each failure branch; do not patch by skipping cleanup. | Invalid device, unavailable controlling TTY, formatted filename, optional/fatal branch, cleanup exactly once; normal interactive startup remains valid. |
| GD2 | `platform/macos/nsterm.swift:7538` delegate binding, `:8542 interpretKeyEvents`, `:10334/10347/10410` key-window callbacks; `tools/test-macos-ime.py focus_frame/run_scenario` | Observe actual per-window key/first-responder identity and event/source transitions; isolate real behavior from synthetic routing/focus loss. These callback methods and delegate already exist. Do not introduce unconditional discard/commit of pending composition or a firstRect-count readiness rule. Choose the minimal supported repair only after evidence; if desktop is locked, proceed with independent code/design work. | Real two-window independent Chinese commits, pending composition transfer and original input-source restoration; window trace/data must identify the actual native window. |
| GD3 | `platform/macos/nsterm.swift draggingUpdated/performDragOperation`, `tools/fixtures/macos-drag.el femacs-drag-snapshot` | Reuse the implemented optional-event hover cleanup and native callback trace. Execute actual external Finder drag when unlocked; require motion plus drop callback and exact opened content. Source/static calls are not a substitute. | Existing IV-A4 plus file bytes and live target frame/x/y; preserve default and user-selected drag function. |
| GD4 | G2/G3 build and compatibility entries above | Map GD1/GD2 changes to the current terminal/Cocoa products and precise affected checks; preserve prior unchanged pass evidence and failed receipts. Publish only a matching built binary/dump/resources set. | Clean affected build, strict signature, terminal/native input, source/config identities and bounded language/version comparisons. |
| GD5 | G4 retention map and xtask/generator readers | Complete present-day inventory, exact reader/replacement mapping and target policy. Safe no-reader redundant scaffolding may be retired in authorized scope; a support-policy choice affecting retained platforms/devices must be surfaced with concrete options before destructive implementation. | Per-batch report before cleanup; restore/rebuild verification and affected local workflows. |
| GD6 | `docs/runtime-modernization.md`, `src/bytecode.rs:524 exec_byte_code`, `src/eval.rs:2304 eval_sub`, `src/alloc.rs:5482 mark_memory`, `:6191 visit_static_gc_roots`, `:6394 garbage_collect` | Complete measurement and full root/store/call inventory, compare actual backend/collector contracts, then define exact candidate modules/types/ABI and failure/fallback behavior before writing cases/implementation. Source anchors above are verified at this baseline; they are not an exhaustive root/store inventory. | Real semantic/GC/unwind/redefinition/pdump and workload cases; no runtime/backend chosen merely from another language's name or a toy. |

### Construction checklist (writing is not acceptance)

- [x] GC0: persist the complete seven-item scope, baseline, stages, evidence rules and immediate source anchors; bind an active native Goal separately.
- [ ] GC1: finish the stable-delivery stage design/ownership table and all remaining native/TTY cases without running new tests.
- [ ] GC2: complete all stable-delivery repairs and harness wiring; mark implementation pending acceptance, then freeze.
- [ ] GC3: consolidate G1/G2/G3 real acceptance and independent review; fix the collected failures and deliver the stable products.
- [ ] GC4: finish G4 full dependency/support/replacement design and all cleanup/renovation cases, then implement all approved changes.
- [ ] GC5: consolidate G4 affected acceptance, recovery evidence, independent review and signed delivery.
- [ ] GC6: finish G5/G6/G7 source/research/measurement designs and all cases without running the new experiments; retain existing factual measurements.
- [ ] GC7: implement complete instrumentation, chosen compiler/interpreter/native candidate and chosen collector candidate with baseline fallback.
- [ ] GC8: run consolidated runtime-stage compatibility/stress/performance experiments, collect failures and make measured adoption/rejection decisions; integrate accepted outcomes and archive withdrawn work.
- [ ] GC9: complete final local product acceptance, documentation, per-batch cleanup reports, independent review, signed commits and immediate push; audit all seven acceptance items before completing the Goal.

### Acceptance checklist (mark only from actual evidence)

- [ ] GA1 / G1: actual remaining macOS native workflows and TTY failure paths pass, without changing pending-composition semantics merely to satisfy a test.
- [ ] GA2 / G2: final changed products build/start from tracked-only inputs, with matching resources, dumps, signatures and trial instructions.
- [ ] GA3 / G3: exact requested compatibility cases and latest-stable scope evidence pass with preserved identities, timings, failures and explicit limits.
- [ ] GA4 / G4: approved modernization/retirement matrix implemented; required readers/workflows work, every cleanup report and recovery identity committed.
- [ ] GA5 / G5: real attribution/survival/phase/pause data supports the later choices; load, full samples, memory and uncertainty are explicit.
- [ ] GA6 / G6: actual native/compiler/interpreter prototype or implementation has complete semantic and performance evidence and an honest integrate/reject outcome; no unsupported shipped-feature claim.
- [ ] GA7 / G7: actual collector candidate has a complete root/store/barrier argument, stress/compatibility/performance evidence and integrate/reject outcome; rejected work does not become a claimed generational feature.
- [ ] GA8 / all: independent final review, actual usable macOS package, clean tracked worktree, verified signed commits and remote branch receipt; all known failures and remaining unverified scope stated.

Lock/permission/toolchain/policy questions block only dependent work. Continue independent authorized work, keep the same checklist and never call an unfinished native workflow passed. Ask a concise concrete question only when a real user decision is required; do not repeat old authorizations. Goal completion requires all required outcomes and the explicitly research-scoped outcomes above; no budget/blocked status or document-only update substitutes for evidence.

### Durable Goal text

```text
/goal 持续完成 femacs 的七项现代化工作，以 docs/modernization.md 的 Persistent Goal 为唯一当前计划：G1修复并验收macOS终端/图形界面、真实外部拖放、多窗口输入法与异常TTY；G2保证干净拉取后独立生成、构建和运行；G3补齐有明确范围的兼容性与最新稳定版语言行为验证；G4完成旧工具、过时基础设施和设备兼容的规划清理翻新；G5取得真实Lisp/GC性能、分配存活和停顿画像；G6完成解释器/编译器优化及真实native/AOT/JIT候选；G7完成真实分代GC等回收优化候选的设计实现和验证。

起点为971cf90d，macOS双产品已独立构建、30组代表性兼容观测一致；已完成结果复用，未完成项不继承为通过。先稳定交付，再清理翻新，最后运行时优化。每个用户已定义阶段严格执行全部设计→全用例编写但不运行→全部实现→最后集中整合、测试、独立审核与验收。按GC0–GC9施工、GA1–GA8验收维护同一清单；未来候选必须先补齐真实源码绑定的精确设计，不能只做文案或玩具测试。

保留可观察Lisp语义、求值/错误/GC/异常行为、数据及兼容接口；原只读参照不是构建依赖；构建和高风险运行受当前controlled-run保护，禁止远端CI和Fast mode。继续既有结对：我主开发、搭档独立测试审核，双方验证并反驳不成立的说法，不创建子agent。每批删除/撤回/缓存清理先留逐项身份、原因、替代和恢复证据，提交maintenance/cleanup报告并更新日志。完整语义单元验证后签名提交并立即推送。

优化以同机真实工作负载、不可变输入/制品、所有轮次与中位数/波动/停顿/内存证明收益；不擅定性能阈值、不换慢参照、不以busy数据宣称升级。native/GC研究项必须有实际编辑器候选与兼容/性能对照，采纳可靠收益候选；若真实证据否定候选，撤回并保留失败/恢复与结论，只能完成对应研究交付，不得虚称功能已启用或降低必须交付的功能标准。

持续推进所有有权限且可独立完成的事项。锁屏或局部问题只挂起依赖项；遇真实政策/权限选择时转达具体问题、完整选项与影响，继续其他方向。不得因旧goal停止、局部blocked或报告写完而停止整个目标；只有所有必要验收和交付都有真实证据才完成Goal，否则保留准确缺口和下一项，按当前目标接口处理阻塞，不空转、不提前宣告完成。
```

The slash command above is a reusable command surface. Actual creation/status is verified through this conversation's native Goal interface; saving the text does not assert a CLI keystroke was submitted. Goal creation has no user-specified token budget, so none is invented. Official lifecycle reference: [Using Goals in Codex](https://developers.openai.com/cookbook/examples/codex/using_goals_in_codex).

Baseline: `3d24214a9b1f12316f7263bade8faa95651df066`, master, clean at authorization. The user explicitly authorizes implementation in this order: (I) agreed local-delivery repairs, (II) planned removal/renovation of obsolete infrastructure and device support, (III) Lisp compiler/interpreter/GC improvements and measured native/JIT exploration. Codex is primary developer and an independent fixed-model partner owns tests and final review. Private identities/logs remain outside tracked documents. No remote CI is used; delivery commits carry `[skip ci]` to prevent push-triggered runs.

The previous one-to-one transcription is a baseline, not a prohibition on the newly authorized modernization. Observable Lisp evaluation/error behavior, saved data and useful editor workflows remain compatibility constraints. Swift Cocoa and Kotlin platform integration are not obsolete merely because they are not Rust. Do not mechanically rewrite dictionaries, Unicode generators or copied Lisp to satisfy a language count. The optional reference checkout remains read-only and is never a build prerequisite.

## Stage boundaries and status

| Stage | User-visible outcome | Status |
| --- | --- | --- |
| I | Reliable local macOS delivery, honest test outcomes, configurable drag callback, isolated app publication, traceable IME checks | D: implementation and supported local checks passed; real cross-window drag remains unaccepted |
| II | Explicit supported-platform matrix; retired paths with replaced readers; modern Rust tooling where it reduces maintenance; helpers and selected feature delivery | Platform decision pending; inventory/design follows stage I |
| III | Measured Lisp runtime improvements with semantic fallback; evidence-based choice among bytecode, native/AOT/JIT and generational GC | Research/feasibility follows stage II; no blanket adoption or speed claim |

Stages are the user's explicit sequence. Each stage independently follows full design -> all cases written without running -> all implementation -> consolidated local tests and independent review. This is not permission for per-file test/review loops.

## Stage I requirements

R1: ERT comparisons preserve test identities, outcomes, skips and timing. Zero executed cases are untested, not equivalent success; failures/unknowns cannot produce gate success. Collection-only mode is explicit. Retries retain every attempt and still run the reference. A failed retry is not called a passing flaky case.
R2: Normal and forced-GC sweeps are distinct, chunking remains resumable, subprocess failures reach the caller. Enumeration does not require any binary or mutate Git metadata. Default product is `build/terminal/port/src/emacs`; optional reference audit stays separate.
R3: Native drag motion calls the configured `Vns_drag_motion_function` with frame/x/y. Preserve the default and user replacement callback; do not hard-code the default symbol.
R4: Package into a unique sibling staging app, validate contents and signature, then publish. Existing app stays untouched on preparation/sign/verification failure. Deleted source resources disappear from a replacement app. Publication of an existing app uses macOS atomic directory exchange; do not pretend ordinary rename overwrites a nonempty directory. Cleanup only owned staging/old swapped app.
R5: IME diagnostics are opt-in and bounded, default editor behavior unchanged. Separate stable-source composition from source-switch pressure in fresh processes; record actual source/focus/key/Cocoa callback time lines. Commit must equal the configured expected text, not merely contain any Han character. Preserve every failed attempt; no retry-to-green. Unknown cause remains unknown until trace supports a fix.
R6: Provide a local validation entry with explicit current product paths, source/config/product hashes and per-check status/duration. No network/remote runner. Report reused products as reused; command completion alone does not certify functional parity.

## Current and target structure

```text
platform/macos/nsterm.swift       # key/IME callbacks; draggingUpdated omits function argument
xtask/src/macos.rs                # build, copy_tree, package merges into old app
xtask/src/main.rs                 # command dispatch
xtask/src/standalone.rs           # current terminal output
xtask/src/bundle.rs               # NEW isolated staging/publish helper and component cases
 tools/check-ert.py               # summary-only comparator and unconditional successful exit
 tools/l5-sweep.sh                # wrong nonempty STRESS expansion, pipeline masks errors
 tools/test-ert-harness.py        # NEW independent partner-owned unit/integration cases
 tools/local-acceptance.py        # NEW current-artifact local acceptance driver
 tools/verify-dod.sh              # historical audit requires explicit opt-in
 tools/test-macos-ime.py          # existing real keyboard workflow, one mixed scenario
 tools/fixtures/macos-ime.el      # existing Lisp state report
 tools/fixtures/macos-input-source.rs # source selector; ADD status query
 tools/test-macos-stability.py    # NEW diagnostics/source-contract cases
 docs/current-state.md            # current header still uses historical terminal path
 docs/modernization.md            # this sole current roadmap and stage-I execution checklist
```

```mermaid
flowchart LR
 A[old sweep] --> B[summary counts]
 B --> C[always exit zero]
 D[package] --> E[overwrite live app]
 F[IME switched input] --> G[final snapshot only]
```

```mermaid
flowchart LR
 A[normal or GC explicit] --> B[per-test outcomes and all attempts]
 B --> C[comparison plus strict gate or explicit collection]
 D[package] --> E[unique staged app]
 E --> F[contents and codesign verify]
 F --> G[atomic exchange / first publication]
 H[steady and switch scenarios] --> I[source focus key and bounded Cocoa traces]
 I --> J[exact text cancel save and exit assertions]
 K[local acceptance] --> C
 K --> J
 K --> L[real terminal and GUI product identities]
```

## Exact construction map (baseline line anchors)

| ID | Path / baseline anchor | Target design | Cases |
| --- | --- | --- | --- |
| D1 | tools/check-ert.py:39-69, 123-178, 181-309 | Remove Gitfile creation. Dataclass RunResult(status:str, ran:int, expected:int, unexpected:int, results:dict[str,str], test_seconds:dict[str,float], elapsed_seconds:float, note:str). `parse_result(text:str, returncode:int, elapsed_seconds:float)->RunResult`; parse native ERT result lines and summary, reject duplicates/missing/contradictory detail. `compare_results(port:RunResult, reference:RunResult)->tuple[str,str]` compares names/status before counts; verdicts same, both-fail, port-worse, port-better, different, zero-tests, incomplete, crashed, flaky/both-capped. `gate_exit(verdicts:list[str])->int`: only same and port-better with complete details pass; all unknown/failed/untested/flaky rows fail. `--collect-only` explicitly returns successful collection despite recorded failures. `--results-json` records metadata, each attempt, per-test IDs/status/timing; TSV compatibility columns remain. Every program/fixture path explicit, reference optional only for standalone local delivery, required for differential audits. | T1-T12 |
| D2 | tools/l5-sweep.sh:16-44 | `set -euo pipefail`; argument array for optional GC; default evidence build/acceptance/l5, validate positive chunk; pass explicit program paths/options; collect chunks then aggregate nonzero gate status without discarding failure rows. Never infer total from an error line. | T13-T16 |
| D3 | platform/macos/nsterm.swift:9009 / draggingUpdated | `[Vns_drag_motion_function, frame, make_fixnum(Int(x)), make_fixnum(Int(y))]`. No behavior change elsewhere. | T17 |
| D4 | xtask/src/macos.rs:165-207 / package; NEW bundle.rs | `bundle::publish(destination:&Path, populate:impl FnOnce(&Path)->Result<(),String>, verify:impl FnOnce(&Path)->Result<(),String>)->Result<(),String>` creates unique owned sibling, rejects symlink destination, removes only owned stage on failure, uses `renamex_np(..., RENAME_SWAP)` for existing app, fs::rename for first publication; swapped old app cleanup follows success. macos `package` builds stage with existing copy logic, signs stage and runs strict/deep verification before publishing. | T18-T23 |
| D5 | platform/macos/nsterm.swift:before EmacsView and keyDown 8367, setMarkedText9159, deleteWorkingText9182, insertText9205, firstRect10578 | Add private `ns_input_trace(_ event:String,_ details:String="")` using optional FEMACS_INPUT_TRACE env, FileHandle JSON lines of monotonic uptime/event/details/key-window/marked state at call sites; create mode0600, cap1MiB, default no file/writes. Record callbacks without altering their event order. | T24-T27 |
| D6 | tools/fixtures/macos-input-source.rs:35-72 | Add `status PID` read-only JSON source/frontmost/focused_window using current TIS ID plus Accessibility queries. Preserve select/current/list/restoration. | T28 |
| D7 | tools/test-macos-ime.py:13-74; fixture state report | `--mode steady|switch|all` default all, `--expected` default 你好, each scenario fresh unique out dir/process, reject reused out directory. Split mode selection, verify source+frontmost+focused window before input; record each key time/source status and full trace. Wait on observed states with deadline, no arbitrary extra readiness claim. Store all attempts/scenario outcomes and restore original source even on failure. Exact committed text and normal exit required. | T29-T33 |
| D8 | NEW tools/local-acceptance.py | Arguments --terminal/--app/--out, optional --sender/--sources/--gui/--ime. Persist HEAD, dirty patch digest, selected configs/toolchain/binary/dump hashes; run selected layers sequentially with command/status/elapsed logs. Defaults use fresh owned output paths, never legacy fallback. Existing products explicitly marked reused (build logs are separate provenance). Fail on missing requested dependencies/products. No implicit builds or network, no skipped requested GUI/IME counted as pass. | T34-T37 |
| D9 | AGENTS.md current-phase pointer; docs/current-state.md current header; README build/acceptance; PORTING.md/status.md | Correct current path/header, retain historical records; document local gate, trace/privacy/default-off, exact supported host, new stage order and policy supersession. Commit uses skip-ci token; no remote CI invocation. | T38 |

## Cases and writing ownership

Partner owns NEW `tools/test-ert-harness.py` only: real transcript parser samples, identity swaps, count mismatch, zero tests, skips/expected failures, contradictory/duplicate/incomplete lines, failed and successful retries retaining reference, collect-only vs strict exit, --list without products/Git mutation, normal/stress wrapper argv and pipeline failure. It may use deterministic fake programs and unittest mocks, but must execute real parser/classifier/driver code. Tests record per-case duration via a unittest result adapter. No execution before all implementation finishes.

Primary owns all production code/documents plus bundle component tests (in bundle.rs) and NEW tools/test-macos-stability.py: first publication/replacement/deleted resource, populate/verify failure keeps old app, symlink rejection, and staging cleanup; trace off/on/bounded contract, configured callback argument contract, IME mode/exact-text/out-dir behavior; local acceptance metadata and failure propagation. Existing terminal/native keys/files fixtures are reused; real GUI cases stay a distinct final layer. No edits to copied test/ or data trees.

## Construction checklist

- [x] C0 full stage-I source/design, interfaces and cases specified
- [x] C1 all partner ERT/wrapper and primary bundle/IME/local-driver cases written, not run
- [x] C2 complete ERT parser/comparison/retry/results and strict exit semantics
- [x] C3 complete wrapper modes/path/error propagation
- [x] C4 configured native drag callback and bounded input trace
- [x] C5 isolated/verified app publication and failure recovery
- [x] C6 input-source observation and independent exact-text IME scenarios
- [x] C7 current-artifact local acceptance driver and documentation
- [x] C8 all implementation complete; freeze same snapshot, enter D
- [x] C9 supported local build/UI checks and independent review complete, with A5 explicitly unaccepted
- [x] C10 signed skip-ci commit 809d0de3, signature verified and pushed

## Final acceptance checklist (only in D)

- [x] A1 parser/gate/wrapper component and deterministic process cases, with every case duration and failures/skip states
- [x] A2 Rust publication component cases, including failed signing/verification and unchanged old app
- [x] A3 clean exported source builds terminal and GUI without reference/cache; both final dumps and bundle strict signature valid
- [x] A4 actual terminal and GUI classic keys + Unicode file/clipboard/GC flows; identify exact artifacts
- [ ] A5 real configurable drag callback receives selected frame/x/y (static argument contract alone is not full drag acceptance)
- [x] A6 steady-source and switched-source IME exact commit/cancel/minibuffer/save/exit; separate failures/permissions/lock state, restore user source
- [x] A7 local driver binds metadata to actual paths and fails requested missing/failed layers; normal vs GC argv distinct
- [x] A8 independent review of one frozen snapshot; tracked source/data/reference preservation and signed/pushed delivery with no remote CI

## Future stage design boundary

II begins with a reader/replacement/retirement inventory, supported targets and a retention map. Remove only owned repository code with proven replacement or explicitly retired feature, keep licenses and actionable history, archive useful decisions as searchable docs rather than deleting evidence. Rust tooling consolidation is driven by duplication and dependencies; necessary platform languages/raw-data generators are evaluated individually. User platform response determines retirement scope.

III begins with current evaluator/bytecode/compiler/allocator/root-scanning/GC object graph and real workload measurements. Research native/AOT/JIT backend costs and deoptimization/error/unwind/GC interaction, and generational GC root/write-barrier/pdump/FFI constraints. Java/Go are design references, not drop-in runtime replacements. LLVM reuse must include language semantics, GC safe points and fallback design, not just arithmetic toy results. Prefer an isolated opt-in prototype with unchanged default behavior; promote only with compatibility and real performance evidence. The scope and exact implementation are written after stages I/II establish a stable baseline.

Stage-I B additions: primary bundle component cases and local-driver/exact-text/output-preservation cases are written in xtask/src/bundle.rs and tools/test-macos-stability.py. Configurable-drag real acceptance fixture is tools/fixtures/macos-drag.el. No tests or compiler have run. New driver helpers are run_check(name, argv, out, env=None) -> dict and fingerprints(paths: dict[str, Path]) -> dict; IME helpers commit_matches(state, expected) -> bool and new_output(path) -> Path reject reused evidence directories.

Stage-I C begins after the partner wrote tools/test-ert-harness.py (T01-T16 plus subcases). Primary cases/fixtures are complete. No test, lint, typecheck or product build has run in A/B.

C completion: old verify-dod.sh now requires --historical-audit before any preparation or legacy product access. Its logs remain in owned build evidence. The new local entry never silently falls back to it. All stage-I production/test/documentation changes are now written; enter D once on a frozen snapshot.

D batch 1: publication/local-driver components passed 7+7; independent ERT/wrapper cases were 14/16, exposing the system Bash empty-array nounset behavior. Use the portable guarded array expansion; preserve original failed evidence. IME success now also requires bounded native callback evidence, so an old uninstrumented binary cannot masquerade as diagnostic acceptance. No product-specific IME logic has been changed.

D batch 2: local-1 preserves a failed GUI paste check, while both 26-case key suites, terminal file flow and exact-text steady/switch IME passed. A direct system clipboard probe showed pbcopy exits 0 but drops UTF-8 text under C locale; en_US.UTF-8 preserves it. Fix only the test clipboard CLI boundary, add a component regression, and rerun the affected GUI file flow. The native editor clipboard implementation is unchanged; no product rebuild or repeat of unaffected key/IME suites is warranted.

D evidence: clean tracked-source terminal and Cocoa builds completed without reference/cache (351.257s / 404MiB RSS and 555.451s / 903MiB RSS). ERT/wrapper 16/16, publication 7/7 and final local-driver components 8/8 passed; independent reviewer verified all 20 frozen files and ran the affected cases. Actual terminal/native keys each 26/26, terminal file flow, native Unicode/GC/clipboard flow and separate steady/switch IME passed. IME had 371/390 native callback records, exact 你好, cancel/minibuffer/save/normal exit and source restoration. The first integrated local-1 report remains failed; after correcting only the clipboard test locale, targeted gui-file-2 passed in 15.065s. These results do not certify full ERT parity or all input methods.

A5 remains unaccepted: native argument/configuration contracts are verified, but the manual cross-window drag run did not produce callback evidence. Native UI binding encountered multiple same-bundle paths and reopening rather than a reliable existing-process binding. No direct Lisp invocation is substituted for a real drag. The owned test processes were closed; retain this explicit gap rather than claim complete UI acceptance. Finder/LaunchServices cold start is also not certified. The normal app was atomically republished from the new Cocoa build; the terminal checkout-path build is being refreshed before delivery. Current test host is Apple Silicon macOS 27.0.1; older macOS is not inferred from that result.

## Stage II exact tooling cleanup design

The unanswered platform preference does not authorize removal of any operating system. Retain existing macOS/Android/native glue and every other platform tree until retirement is explicit. This stage removes obsolete bring-up scaffolding and the rejected disconnected asm source, modernizes measurement/differential entry points and records the retention map. Raw Lisp/data, active cross-link scripts and licenses remain. No interpreter/GC behavior changes in stage II.

Current flow: `bench.sh` writes fixed scratch filenames, reports minima, and needs a reference even for current-only measurement; `smoke.sh` can compare empty output from failed processes as success; `differential.sh` accepts equal abnormal exits. `verify-dod.sh -> gen-lisp-mk.py` only generates unused legacy Makefile variables. `l1-worklist.sh`, `loadup-fixture.sh` and `reconcile-link-ratchets.sh` are unused completed-transcription helpers. `spikes/bytecode-hot/bytecode_hot.rs` is not linked and its rejection table stays in README/P-325.

```text
current -> target
 tools/bench.sh                 -> tiny compatibility launcher
 tools/bench.py                 -> NEW candidate/optional-reference measurement
 tools/smoke.sh                 -> tiny compatibility launcher to differential.py --smoke
 tools/differential.sh          -> tiny compatibility launcher
 tools/differential.py          -> NEW strict optional-reference audit driver
 tools/pgo_bench.py             -> retained marker/stat/hash helper, no PGO required by new tool
 tools/pgo/eval.el              -> retained real VM/edit/load/regex/GC corpus
 tools/test-modernization-tools.py -> NEW independent partner-owned cases
 tools/{verify-dod.sh,gen-lisp-mk.py,l1-worklist.sh,loadup-fixture.sh,reconcile-link-ratchets.sh} -> removed
 spikes/bytecode-hot/bytecode_hot.rs -> removed, negative evidence README retained
 tools/check-all-units.sh       -> obsolete removed-tool comment corrected, no platform rewrite
 tools/test-macos-stability.py  -> removed historical guard test replaced by retirement contract
 docs/retired-scaffolding.md, AGENTS.md, README.md, docs/current-state.md, PORTING.md, docs/status.md -> current entries and retirement records
```

```mermaid
flowchart LR
 A[bench.sh compatibility] --> B[bench.py explicit candidate and optional reference]
 B --> C[unique owned run dirs and all samples]
 C --> D[median MAD IQR and fingerprints]
 D --> E[explicit new-format save or strict regression gate]
 F[smoke.sh or differential.sh] --> G[differential.py]
 G --> H[each process status and exact output]
 H --> I[no empty corpus or equal-crash success]
 J[obsolete bring-up scripts] --> K[retirement ledger and Git history]
```

| ID | baseline anchor | precise change / interface | acceptance |
| --- | --- | --- | --- |
| II-D1 | tools/bench.sh:29 PROG through end; tools/pgo_bench.py:121 parse_markers / 145 summarize | New `tools/bench.py`: `parse_samples(text:str)->dict[str,float]` validates six BENCH and four TASK markers; `summarize(rows:list[dict])->list[dict]` uses existing stats helper, preserving every per-process sample. `measure(binary:Path,home:Path,log:Path)->dict` streams stdout/stderr to owned log, rejects nonzero/timeout/missing/duplicate/extra/nonfinite markers, retains failure log. CLI --prog current terminal default, --ref optional, --rounds positive default5, --out new only, --save/--gate exclusive, --tolerance nonnegative default0.15, --force and --max-load-per-core default0.10. Each process runs actual eval.el corpus; startup wall time recorded separately. JSON contains source HEAD/dirty digest, program/dump/corpus hashes, per-round side/order/case/value_ms, stats/elapsed; raw process min5 remains labelled. Baseline TSV columns case/port_ms/ref_ms/date/metric, metric `process-median-of-min5-v1`; reject old minima baseline as incompatible. --save requires quiet load unless explicit force; gate rejects missing/duplicate/invalid/nonpositive baseline cases and fails any median regression beyond tolerance. Existing shell becomes `exec python3 <dynamic tool path> "$@"`; no nested resource executor or additional shell logic. | II-T1-T7 |
| II-D2 | tools/smoke.sh:17 PROG / 37 CHECKS; tools/differential.sh:15 REPO / 50 loop | New `tools/differential.py`: `compare(port:CompletedProcess,ref:CompletedProcess,stderr:bool)->bool` requires both exit0 plus byte-equal stdout and optional stderr; `read_forms(path:Path)->list[str]` ignores comments/blank, rejects empty. CLI --prog current terminal, --ref optional audit default build/emacs-ref/src/emacs, --forms baseline corpus, --smoke selects existing eleven printed forms, --stderr/--keep accepted, --out new only, --timeout positive default60, old --memory-mib accepted as deprecated start hint (no wrapper inside; caller protects whole audit). Unique evidence dirs below build/acceptance; HOME/TMPDIR in owned dir. Always retain JSON, per-form outputs/status/duration, no global scratch and no ref checkout mutation. Nonzero/missing product/timeout/mismatch always nonzero. | II-T8-T12 |
| II-D3 | obsolete five tool entrypoints / spike asm source; current production readers none except verify->gen-lisp | Delete exactly five tools and one detached asm file after recording original purpose/replacement/date/source baseline in retirement ledger. Preserve original historical docs and rejected performance numbers, mark archived recipe source recoverable from Git baseline rather than claim executable replay. Replace test-macos-stability historical guard with exact retirement/replacement presence contract; correct active references in README/AGENTS/current-state/check-all-units. No mass platform deletion. | II-T13 |
| II-D4 | docs/current-state.md current header; AGENTS.md tool policy; docs/retired-scaffolding.md | Record policy: Rust build/link/gate drivers remain xtask, Python measurements/data generators retained where appropriate, minimal shell compatibility launchers do not own policy; Swift/Kotlin native bridges retained. Existing unsupported/untested targets are not called retired. Inventory category counts and reader/replacement map remain searchable; compilation warnings and Android reference-config dependency remain explicit future work. | II-T14 |

- [x] II-C0 design fully specified and discussed
- [x] II-C1 all independent cases written without running
- [x] II-C2 all measurement, differential, compatibility launcher and retirement implementation written
- [x] II-C3 current documentation/readers integrated; freeze and enter D
- [x] II-A1 deterministic parser/stat/baseline/gate/argv/failure isolation cases timed and pass
- [x] II-A2 actual candidate-only and paired measurements complete; 95-form real reference audit passes; deliberately failed fixtures fail honestly
- [x] II-A3 no production reader of deleted asm/helpers; raw/source/data/license/platform trees preserved
- [x] II-A4 independent frozen review; signed skip-ci commit 2415a335 verified and pushed

All workloads that start the compiled editor run under the external controlled executor once; wrappers do not nest it. A new measurement baseline does not retroactively prove a speedup or native/JIT feasibility. Stage III uses these complete samples to select a bounded real-runtime optimization, after stage II delivery.

II final testable interfaces: bench `load_baseline(path:Path)->dict[str,float]` validates exact ten-case set/metric/finite positive values; `gate_rows(summary:list[dict],baseline:dict[str,float],tolerance:float)->list[str]` returns regressing case names from candidate medians only. `main(argv:list[str]|None=None)->int` and `new_output(path:Path)->Path` are public on both tools; fresh output is exclusive and never wipes evidence. `summarize(rows)` accepts case/side/round/value_ms and returns case/side/n/min/median/mad/iqr; side names candidate/reference. `measure(binary,home,log)` returns samples as a list of case/value_ms objects plus wall_ms/corpus_wall_ms/exit_code; it checks both raw log names before any process, runs the corpus first, retains failed logs, then measures a separate fresh-process startup. The CLI samples include that explicitly labelled startup wall case; no cold-cache latency claim is made.

Root-path delivery refreshed: terminal complete bootstrap/dump succeeded in 420.553s (400.2MiB peak RSS); real terminal file workflow then passed. The published root app passed strict/deep signature verification and batch Unicode/calendar load. Both normal trial paths now contain this build; no generated binaries are committed.

II interface messages crossed during B; final D resolution uses candidate/reference side labels and a samples list consistently in tests, implementation and design. Startup is explicitly labelled process wall time, distinct from ten in-process min5 cases. Summary aggregation accepts partial case sets for component reuse, while CLI completeness and baseline validation require all ten eval cases. Output homes use the same owned pathname reset between differential sides to avoid environment-path false mismatches and shared state. B contains 14 case groups, not 14 individual assertions; no execution occurred in A/B. Stage II C now begins at baseline 809d0de3.

II C complete: six obsolete paths removed, active wrappers replaced, measurement/differential failure handling and current documentation fully integrated. Retention ledger names remaining platform/config/packaging gaps. All source edits are now complete; freeze before D. No stage-II tests or compiled editor runs occurred in A/B/C.

II D consolidated fixes: first independent run was 11/14 with three interface-crossing errors, not hidden as passing. Final contract now consistently uses candidate/reference and list samples; logs are checked before either child starts, per-round order and total elapsed time are explicit, products are rehashed after measurement. The full self-comparison corpus initially failed two authored fixtures: temporary-buffer undo was disabled and a reader input used invalid text-property syntax. Repair valid undo/vector inputs and add separate explicit expected-error probes plus valid text-properties; no cases skipped, no editor behavior changed. A self-comparison is a harness control, not independent compatibility proof.

D also exposed the still-broken historical unit wrapper: old nested-directory assumption, shared scratch deletion and lost pipeline failures. Add `xtask/src/check_units.rs::parse(text:&str)->Result<Vec<String>,String>` for owned C/Rust lists (reject empty/missing/duplicate/path escapes) and `run(list:Option<&String>)->i32`; reuse the existing normal/checking compiler via `build_selected_units(units:Vec<String>,materialization:&Path)->Result<(),String>`. Keep object locations for link consumers, isolate materialization per invocation, remove no other run's tree. `tools/check-all-units.sh` becomes an argument-forwarding Rust launcher. Three timed Rust component cases cover mapping/invalid lists/ownership; one real unit verifies both compiler legs without rebuilding unchanged product code. The platform aggregate explicitly uses Bash pipefail and owned build scratch so child failure reaches its caller; this is not Android runtime acceptance or retirement. The corresponding defaults/retention map are updated before one consolidated re-freeze and affected verification.

D cross-target review caught a genuine output-path regression: reusing the full-unit compiler must retain target-suffixed normal/checking object names consumed by the shared linker. Both legs now use one object_path helper, the same nonempty target, Linux linker setting and Cocoa ABI cfg. Add a fourth timed component contract and verify one explicit host-target compile alongside a preserved unsuffixed sentinel; Android itself is not rerun. A first Rust component run (2/3) exposed test-CWD-dependent source resolution; parse now anchors to the manifest parent and maps exact owned unit paths, then passed 3/3 before the target-naming case was added. Original failures remain recorded.

II final local results: independent light suite 14/14 (21.053s), affected gate edge case 1/1 (1.305s), retained local driver8/8, Rust unit-list/object-name cases4/4 (2.617s), actual normal/checking one-unit compile5.287s and explicit-target compile5.037s with host sentinel unchanged. Actual existing reference audit95/95 in12.484s; this is the bounded authored corpus, not full ERT or stable-version compatibility. Paired5-round110-sample measurement13.923s has binary/dump/corpus hashes and all rows retained. Baseline was explicitly forced at load/core1.00; it is development evidence, not a quiet-host speedup or release performance claim. New source/corpus schemas do not rewrite old reports. Shell inventory19→15: four completed bring-up shells removed, four retained current shells reduced to launchers, l5 wrapper retained, ten platform-related scripts held with their reader/independence gaps recorded. No operating system was silently retired.


## Stage III bounded dispatch experiment

Exact current architecture, primary-source research, differences and semantic/performance cases are in `runtime-modernization.md`, bound to 2415a335. The user-authorized first implementation explores shared-tail dispatch within the actual VM, default state vs experimental direct build. It does not enable Lisp native compilation or change GC. Larger backend/collector choices remain researched routes. Platforms are preserved while retirement scope is unspecified; that does not prevent this platform-independent VM experiment.

- [x] III-C0 actual shared-tail/frame/root source and exact state/direct design written; independent design discussion in progress
- [x] III-C1 all independent semantic/observation cases and primary mode cases written without running
- [x] III-C2 all shared-tail macro/label/configuration implementation and documentation complete
- [x] III-C3 frozen snapshot, isolated state/direct build recipe and baseline identity complete; enter D
- [x] III-A1 compiler-mode tests and both VM unit configurations compile/check
- [x] III-A2 real state/direct/reference semantic cases and deterministic observations agree; full authored differential and requested ERT cases complete
- [x] III-A3 no VM candidate retained; original root terminal keys/files rechecked, unchanged app signature verified, stage-I GUI/IME evidence retained without a new UI claim
- [x] III-A4 actual generated-code and alternating workload comparison, confidence/limitations stated, experimental/default decision justified
- [x] III-A5 independent frozen code review and exact cleanup complete; signed skip-ci delivery receipt is the Git commit/signature and upstream state

III B completed without execution: independent fixture has10 ERT groups with operand/dispatch observations and a strict multi-artifact driver/self-tests; primary mode parser cases are written. Phase A had zero remaining design blockers after the five explicit control-flow/cache challenges were resolved. C starts now; no test/compiler/editor has run for stage III.

III all implementation is written and source/label/configuration items cross-checked against the manual. Freeze before D: both modes share handlers, call/handler arms merged, state-only immediate transfers guarded. Original delivered terminal quartet copied with hashes as an additional control. No stage-III test/editor/compiler has run in A/B/C.


### Stage III release compiler resource gap

- [x] III-RC0 exact clean/delivered missing-library evidence and generator change design written
- [x] III-RC1 component cases written, all implementation completed before execution
- [x] III-RA1 component cases and actual clean-export compiler resource generation pass
- [x] III-RA2 same-VM source/compiled observations and alternating before/after loading evidence
- [x] III-RA3 final independent code review complete; signed delivery receipt is recorded in Git


Stage-III final D outcome: both experimental VMs and the C reference passed the
corrected ten-group semantic/critical-observation suite; the delivered root VM also
passed all ten with identical observations, eval/alloc 27+4 ERT, 26 terminal keys
and the Unicode save/reopen flow. The rejected VM source/switch/build flag is fully
removed; current source and trial products retain the original VM. No new native
UI claim is made. Compiler generation and scoped source/bytecode load comparison
are accepted as recorded in runtime-modernization.md; native/JIT/new GC remain
future designs, not delivered features. Existing A5 real cross-window drag and
unaccepted platform retirement/support decisions remain explicit gaps.

## Stage IV: macOS release baseline and real Lisp profile

Baseline: `3ab55087a57d14c5ded30938351c8dcc9983ffc3`, master, clean. The user approved the next plan and continued pairing. This delivery covers the proposed next-round items 1/2/3/5: native delivery gaps, independent builds, broader bounded compatibility, and runtime measurement. Remaining retirement, native compiler and collector implementation stay later in the authorized roadmap. No new goal or remote CI; no VM/compiler/collector redesign in this batch.

### Current architecture and exact gaps

```text
xtask/src/{standalone,macos,bundle}.rs  target builds, generated resources, signed publication
tools/bootstrap-resources.py          self-contained tracked-input generation
tools/local-acceptance.py             product identity and existing native/terminal/IME checks
tools/test-macos-delivery.py           26 real input cases and Unicode file/clipboard flow
tools/test-macos-ime.py               marked/commit/cancel/minibuffer/source-switch acceptance
tools/check-ert.py                    strict identities, exit semantics and per-test durations
tools/bench.py + tools/pgo/eval.el     min-of-five throughput; not pause/heap attribution
tools/fixtures/macos-drag.el          native motion callback; no complete drop/ready evidence yet
platform/macos/nsterm.swift           draggingUpdated -> safe_funcall(function,frame,x,y)
```

```mermaid
flowchart LR
 S[tracked inputs] --> B[standalone terminal and Cocoa builds]
 B --> P[signed app and dumps]
 P --> K[existing keys files IME]
 P --> D[drag callback fixture: incomplete native evidence]
 P --> E[strict bounded ERT]
 P --> M[min5 timing: no pause or heap samples]
```

Graph service has no femacs index; relevant sources are read directly. Existing drag code calls the configured function; do not fabricate native acceptance by directly invoking it from Lisp. Existing ERT/differential success is bounded, not all upstream versions/plugins. There is no provenance for a post-compiler-library full Cocoa build yet. Native launch and drag require an unlocked usable desktop. Do not change system privacy permissions or interfere with user editor instances. LaunchServices must identify the newly launched exact executable/PID, not reuse another same-bundle app.

### Requirements and file-level changes

| ID | file / baseline anchor | precise change | acceptance |
| --- | --- | --- | --- |
| IV-D1 | NEW `tools/runtime-profile.py` | CLI `--binary Path --rounds int(default7) --out Path --max-load-per-core float(default0.10)`. `parse_profile(text:str)->list[dict]` requires exactly one schema marker and six unique complete cases; finite nonnegative timings, GC counts, pause samples, heap/allocation observations and completed checksum required. `summarize(rows:list[dict])->list[dict]` records all samples, median/MAD/p95; never minima as headline. New-only owned output, isolated HOME, binary/dump/fixture hashes before/after, per-process RSS samples via psutil and fresh-process startup wall latency. Report busy load as non-quiet, not a speedup or trustworthy promotion baseline; do not silently force a quiet flag. Streaming logs, bounded parsing, timeout cleanup. | IV-A3 |
| IV-D2 | NEW `tools/fixtures/runtime-profile.el` | Six cases `load/edit/regex/bytecode/gc-churn/gc-retained`. Default GC threshold remains active except explicit allocation workloads, locally bound to 100000 bytes. Each records wall_ms, gc_count, gc_ms, individual post-gc elapsed deltas, `memory-use-counts` delta, `garbage-collect-heapsize` before/after and deterministic result. Counters and heap describe this workload, not object age, precise total memory or full GC phase timing. Retained workload explicitly holds a list through a collection, checks its contents, then releases it. Emit one `FEMACS-PROFILE` JSON marker with schema `femacs-runtime-profile-v1`; errors cause nonzero process exit. | IV-A3 |
| IV-D3 | `tools/fixtures/macos-drag.el:3–11` | Extend acceptance-only fixture with a named native target frame and an external Finder source, explicit geometry, ready JSON, native callback records and periodic drop/opened-file state. Environment `FEMACS_DRAG_REPORT`, `FEMACS_DRAG_READY`, `FEMACS_DRAG_FILE`; callback records live frame, numeric x/y and frame name. No rebinding native drag to a simulated Lisp call. | IV-A4 |
| IV-D4 | NEW independent `tools/test-release-baseline.py`, `tools/fixtures/release-baseline.el` | Partner designs/writes bounded compatibility suite for user-style config, representative shipped packages, Unicode coding/save, search/undo/markers, errors/unwind and GC live roots. Run unchanged fixture on terminal and native batch products via strict ERT parser; require exact expected IDs, zero skips/failures, per-case timings and artifact fingerprints. Component tests cover profile schema, invalid markers/missing cases and output preservation. Final interfaces agreed in discussion before writing. | IV-A2/3 |
| IV-D5 | `docs/current-state.md:1`, this section, `docs/runtime-modernization.md` and `docs/macos-tryout.md` | Add actual current outcomes, commands and exact scope, including open failures and resource/profile limitations. Preserve old failed evidence. No obsolete-code deletion is needed; any actual artifact cleanup gets its own maintenance report/manifest before removal. | IV-A5 |

Target adds the two profile files, two independent test files and extends the acceptance fixture. Build/runtime production files stay unchanged unless final D exposes a confirmed defect; such fixes are collected into one batch and recorded here before affected revalidation.

```mermaid
flowchart LR
 S[tracked frozen source export: no reference/cache] --> B[complete terminal and Cocoa build]
 B --> F[hashes dumps signed trial app]
 F --> C[new bounded ERT and existing keys files IME]
 F --> D[actual native drag and cold launch: exact PID receipt]
 F --> R[all seven rounds and real GC/heap/RSS observations]
 C --> O[honest local release report]
 D --> O
 R --> O
```

### Sequence, ownership and acceptance

- [x] IV-C0 complete source-bound design and resolve partner interface feedback (A).
- [x] IV-C1 write all component/compatibility fixtures and native acceptance steps; no execution (B).
- [x] IV-C2 implement all profile/drag/documentation changes; freeze the complete batch (C).
- [x] IV-A1 build terminal and Cocoa from a fresh tracked snapshot with no ignored reference, old resources or object cache; bind export manifest, full build logs, final dump hashes and signature (D).
- [ ] IV-A2 run exact bounded new ERT IDs on both products, existing authored differential/VM coverage as appropriate, native and terminal keys/file flows plus independently requested IME; no full-compatibility claim.
- [x] IV-A3 strict parser/component cases pass; all profile rounds complete with every raw sample, median/MAD/p95, allocation/GC/heap/RSS data, source and artifact identity. A busy host remains a disclosed environment limitation.
- [ ] IV-A4 actual configurable drag callback sees live target frame and numeric coordinates; file drop opens the expected owned file; cold-launch identifies the correct app and reaches a live native frame. Distinguish real event delivery from fixture assertions and document any unavailable layer.
- [ ] IV-A5 independent frozen review, necessary consolidated fixes, user trial instructions, signed `[skip ci]` commit and immediate push; exact cleanup report if anything is removed.

Primary owns profile tooling/fixture, native fixture, design/integration/docs and Git. Partner owns the two new test files once interfaces are accepted, then final read-only review and independent allowed tests. Neither runs tests or compilers before the entire batch is written. Native desktop tests are serialized. Heavy processes are controlled with the stable owning session; no cross-session budget or process cleanup. Do not auto-enable JIT, PGO, new GC, or delete platform trees.


IV A discussion resolution: the partner explicitly agrees to the seven-item order and current 1/2/3/5 delivery. Accept its real-drop, exact-PID relocation and multi-frame IME additions. Keep the two agreed test filenames; do not create a second profile framework. Primary writes the profile implementation, partner writes independent contracts and compatibility. The `currentEvent!` concern is a real optional-unwrapping risk; `mouseExited` also clears the hover state, so simply skipping the call on nil would lose behavior. Planned production fix: `clearMouseFace(event: NSEvent?)` shares the existing body, writes timestamp zero for no event, and always clears the existing mouse face. AppKit `mouseExited(with:)` forwards its nonoptional event; `draggingUpdated` forwards `window?.currentEvent`. Add a `performDragOperation` trace to distinguish motion from actual native drop.

Primary B additions (written, not run): optional two-frame mode in `tools/fixtures/macos-ime.el` adds selected frame name and second buffer text; `tools/fixtures/macos-input-source.rs::focus_window(pid,title)` binds the exact app/title through AX; `tools/test-macos-ime.py --mode multi` checks commits in A/B and pending-composition focus transfer without inter-buffer corruption. Focus-loss commit/cancel policy is observed explicitly, not invented as a universal IME rule. Existing `--mode all` retains its historical steady/switch semantics. Drag source is an external Finder window; do not create a fake source event or require an unused editor source frame. Native fixture records frame/paths and actual opened content. These are acceptance-only changes; source Lisp/data stay untouched.


IV B receipt: independent `release-baseline.el` has rb01..rb10 groups (custom/config, shipped libraries/modes/tooling, Unicode I/O, search/narrowing, undo/atomic groups, markers/properties/registers, unwind, GC roots). Its rb01 manually loads isolated early/init/custom files in batch; it does **not** prove startup automatically discovers them. Primary `tools/local-acceptance.py::user_init_workflow(binary,work,gui=False)` additionally uses a fresh interactive PTY/native startup with `--init-directory`, no `-Q`, batch flag or fixture load: requires early-init/init/custom/startup-hook order, sentinel731, exact PID, native/terminal state and normal exit. Added before any execution. Native helper and profile runner are acceptance harnesses; no tests were run while writing them. Full new test ownership returned by the partner; production implementation is only the P-333 Swift helper/drop trace and current documentation.


IV D first batch: existing stability components8/8. New component CLI initially failed7/7 because its custom per-test loop bypassed setUpClass; a standard-suite diagnosis passed7/7, then the repaired actual CLI passed7/7. Independent frozen review confirmed this root cause and the duplicate startup framework; one consolidated repair keeps the shared actual terminal/GUI startup workflow and the rb01 isolated custom/keymap test. Uncommitted-code recovery is archived as CLN-20261024-05. Independent bootstrap3/3 and ERT parser16/16 also passed. Original failures remain separate.

The first clean export completed324 compiler legs, then failed with ENOENT before linking; the driver executable and earlier objects were no longer present while later objects remained. The actor/cause is unconfirmed, so this is a failed build, not a compiler/runtime pass or a source defect conclusion. Retry uses a new isolated persistent cache export, unchanged production sources and the same resource limits; no foreign cleanup task or process is altered.

### Stage IV D delivery evidence and open native acceptance

The second tracked-only export, initially without reference/build/cache inputs, completed both full terminal and Cocoa builds, generation, final dumps and app signing in 934.926 seconds. Peak accounted/RSS was 809009152 bytes, footprint 474138736 bytes; the controller exited zero and cleaned its tree. The export uses `.femacs-cache/stage4-source-2`; generated inputs are produced by the standard build, not copied from the optional reference. The app published at `dist/macos/Femacs.app` has the same executable and dump hashes as that accepted candidate. The previous root app is retained privately for rollback; no binaries enter Git. The normal root terminal remains the prior delivered executable; the new clean-export terminal is separately bound to these tests and its own resource tree, rather than moving a binary with embedded paths.

Independent frozen review found no blocker in the accepted profile/startup/compatibility changes. The same corrected ten-group ERT fixture passed on new terminal, new Cocoa and the existing optional C reference: 30/30 groups, zero skips, equal deterministic observations. This is representative compatibility, not all upstream ERT, arbitrary configurations or plugin certification. The reference executable is only an audit comparator. Component results: bootstrap3/3, strict ERT parser16/16, stability8/8, new profile/release contracts7/7.

The initial new-product local aggregate is **12/13**, not an all-green run: terminal26/26 keys, Unicode save/reopen, native25/26 keys, native file/clipboard/GC flow, signatures, batch checks, and real interactive terminal/Cocoa config discovery all passed. Config discovery observes early-init/init/custom/startup-hook order, sentinel731, exact PID and normal exit; the corrected PTY owns a controlling terminal. The remaining synthetic C-g recovery case exposed a prefix-cancel race. The harness now observes pending `C-x`, sends C-g and observes an empty prefix before C-a; the targeted rerun passed with point1, `move-beginning-of-line` and normal exit. Original failures remain intact. No runtime key behavior was changed to fit a test.

Single-window physical-key IME steady/source-switch runs passed Chinese composition, commit, cancellation, minibuffer composition/cancellation, save and normal exit; original input source restored. Multi-frame `--mode multi` remains **unaccepted**: frame A succeeds, native window cycling selects B, but the automated B run inserts ASCII without a marked composition. A separate manual probe observed B marked composition, which is narrower than the complete required workflow. AXMain/Raise and a global-event attempt did not settle the cause and were withdrawn; do not call this a proven product defect or a proven harness-only defect. Existing `--mode all` continues to mean the established steady/switch pair, not multi-frame acceptance.

The actual external Finder drag attempt was interrupted by the desktop being locked; the owned test process was closed. There is no successful motion/drop/opened-file receipt, so IV-A4 remains open. P-333 removes the nil optional unwrap in code and is present in the fully built product, but source review and build success do not prove external drag behavior.

The relocated, strict-signature-verified app passed LaunchServices `open -n` with its original source export temporarily unavailable: exact new executable/PID, native frame, expected document bytes and all active resource directories inside the relocated app. The source export was restored and owned process closed. This proves that bounded startup/file-open case, **not Finder double-click or unlocked visual interaction**. LaunchServices reparents the app outside the controller tree: this already measured, bounded lightweight `-Q` invocation used exact PID/birth/path cleanup; no whole-tree footprint measurement is claimed for this one case.

The seven-round Lisp/GC results and environment limits are recorded in [runtime-modernization.md](runtime-modernization.md). Busy-host data is not a speedup or quiet-host promotion baseline. No native backend, JIT, PGO default or new GC was enabled.

The independent reviewer also identified an existing untested no-controlling-TTY fatal diagnostic path in `src/term.rs`: `vfatal` argument forwarding and terminal cleanup ownership require a separate source-bound fix and negative-path test. Normal PTY/native startup passed; neither that fact nor this report claims the abnormal path is fixed. Keep it in the next repair list with multi-frame input and external drag.

Cleanup records: [CLN-20261024-05](../maintenance/cleanup/reports/2026-10-24-05-startup-harness.md) archives duplicate startup/test-helper withdrawals and fixture repairs, with isolated recovery4/4 hashes; [CLN-20261024-06](../maintenance/cleanup/reports/2026-10-24-06-build-artifacts.md) removes192 idle compiled/cache files from the first failed export, logical5336495 bytes, allocated5828608 bytes. Raw logs, failures, source/data, active candidate resources and trial products are preserved. Neither cleanup proves a system-wide free-space delta.

Private raw evidence: `build/acceptance/stage4-final-baseline`, `stage4-profile-final`, `stage4-local-1`, `stage4-gui-cancel-2`, `stage4-ime-multi-6`, `stage4-ime-multi-7`, `stage4-drag-1`, `stage4-cold-1` and the two `stage4-*-build-2.log` files. They retain process/host details and are not committed. Stage IV's native acceptance stays open until the missing real workflows pass; the delivered tools/build/profile results must not be described as completing all Stage IV acceptance.

Final path/prose review removed three CLI help wording violations and replaced a literal profiling cwd with its isolated owned HOME. Affected profile rerun retained42+7 samples, stable13 artifact identities and cleanup success; quiet=false at load/core1.0180, recorded separately in `runtime-modernization.md`. No GUI/ERT rerun was implied by those text/cwd changes. Prose gate then reports zero failures; final component CLI7/7, cleanup identities/links and recovery4/4 pass. Staged path candidates were semantically reviewed (dynamic historical placeholders, prose/API notation, relative joins and interpreter directives), with private machine/session identifiers excluded.
