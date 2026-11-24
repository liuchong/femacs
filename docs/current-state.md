# 当前状态（2026-11-24）

**本段为当前实际读数（Phase II Stage `C2/A2-R3` RPC Error 非阻塞回传与真实阻塞 pipe 验收收口，响应 `651f05b5` `[FEMACS-C2-A2-R2-REVIEW]`），取代下方 `C2/A2-R2`、`C2/A2-R1`、`C2 & A2` 及更早历史段落。**

- **Phase II Stage `C2/A2-R3`（`[FEMACS-C2-A2-R2-REVIEW]` RPC Error 同步 `stderr` 阻塞去除、阻塞写端恢复与 Scenario D3 实测）已闭合、`dist/lisp` 重建及集成/单测全过、并完成 `CLN293` 清理，待 Codex App `[FEMACS-C2-A2-R3-REVIEW]` 最终核收**：
  - **非阻塞 RPC Error 分支（`crates/http-host/src/lib.rs`）**：
    - 删除 `run_rpc_worker` 中 `RpcResponseFrame::Error` 分支在派发 HTTP 回复前对 `io::stderr().lock()` 的同步写与 flush，避免阻塞的日志接收端卡住 `500` 响应或关停 `worker_handle.join()`；Lisp 侧诊断仍经克隆/继承 stderr 直接输出。
  - **阻塞态恢复、Scenario D3 与保护性单测（`tools/test-c2-a2-lisp-app.py`、`crates/http-host/src/lib.rs`、`maintenance/tooling-inventory.json`）**：
    - 抽提 `fill_pipe_and_restore_blocking(fd)` 在写满共享 stderr pipe 后恢复真实阻塞态（`os.set_blocking(fd, True)`），并新增 `Scenario D3`：满阻塞 stderr pipe + 兄弟 `/bin/sleep 5` 持写端进程下 `/api/boom-quiet` 返回 HTTP `500`、后续 `/api/ping` 保持同 PID 返回 `200`、`SIGTERM` 在 `< 3s` 内退出 0 并回收后端及释放端口；新增单测 `rpc_worker_returns_handler_error_and_continues_on_nonfatal_rpc_error`（`http-host` `9/9`）。
  - **受控构建、集成验收、保护单测与 `CLN293` 读数（`assistant run`）**：
    - `dist/lisp` 重建（`run_id=032f31e33c684afda24a1372698f91c0`，`184.37s`，`exit=0`，`cleanup_complete=true`）。
    - Phase 2 验收（`run_id=02d73762546a4692a7b3184227a54991`，`30.067s`，`exit=0`，`cleanup_complete=true`）：`test-c2-a2-lisp-app.py --selftest`（`PASS`）、`test-c2-a2-lisp-app.py --package dist/lisp`（`PASS`，含 Scenarios A/B/C/D1/D2/D3）、`test-repository-bootstrap.py`（`4/4`）、`check-unsafe-boundary.py`（`442` 活跃文件、`149` `forbid`、`6,327/6,327` `SAFETY:`、`0` 错误）、`check-own-voice.py`（`0` 失败）、`test-tooling-modernization.py`（`33/33`，`1.148s`）。
    - Phase 3 保护性单测（`run_id=1bf23cb6c89640e085d976b8de59103e`，`24.352s`，`exit=0`，`cleanup_complete=true`）：`cargo test -p http-host`（`9/9`）、安全边界与合规扫描 `0` 错误/失败。
    - [CLN293](../maintenance/cleanup/reports/2026-11-24-293-c2-a2-r3-nonblocking-rpc-error.md) 清理 `4` 项临时目录（`3,761` 文件、`1,020` 目录、`8` 符号链接、`822,615,794` 逻辑字节、摘要 `f5d8ee2b6c170143d50e359d3d308b35b487f760b6dbb06af781d6a72d7b313e`）；当前 `dist/lisp`（`210M`）：`bin/femacs-lisp` `5,053,696 B`（`c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6`）、`bin/femacs-lisp.pdmp` `18,325,840 B`（`8b7033d0f887714299cc7a9ed134cff88e62bbf348e26c864be2936e1bdee14f`）、`bin/http-host` `2,145,728 B`（`31f438b7ca7b102dff78545a03a128d6305f13732f7714b2e1ad3035013a5c00`），strict `codesign` 与 `manifest.json` 核验一致；清理后冒烟 `run_id=33e9fc2ff1b14fa0baca25df1aa72661`（`13.07s`，`exit=0`）：`LISP=42`、`HTTP=http-host 0.1.0 (femacs 32.0.50)`、`--package dist/lisp PASS`。保留 `build/terminal`（`45M`）、`dist/macos`（`235M` 与 `/Applications/Fe.app` 同源一致）、`build/generation`（`13M`）与 `build/manuals`（`1.4G`）。

## 历史记录（2026-11-24 Stage C2/A2-R2，已被上方 Stage C2/A2-R3 取代）

- **Phase II Stage `C2/A2-R2`（`651f05b5`）**：
  - **请求体截止与无阻塞关停（`crates/http-host/src/lib.rs`）**：
    - `handle_http` 在读体前即建立 `deadline = Instant::now() + state.timeout`，经 `read_body_bounded`（`tokio::select!` 联立 `watch::Receiver<bool>` 与 `tokio::time::timeout_at(deadline, to_bytes(...))`）有界读取请求体；超限返回 `413`，读体超时返回 `408`（`Connection: close`）且不伤及后端 PID，关停时立即以 `503`/`502`（`Connection: close`）中止等待中的连接。
    - 取消 `spawn_stderr_forwarder` 管道与无期限 `out_fwd`/`err_fwd` `join()`，改为通过 `io::stderr().as_fd().try_clone_to_owned()` 直连子进程 stdout 至 stderr 并沿用 `Stdio::inherit()` stderr；在 `with_graceful_shutdown` 中异步触发 `reap_child_slot`，并对 `axum::serve` drain 施以 `SHUTDOWN_HTTP_DRAIN_GRACE`（`750ms`）硬上限。
  - **集成与单测覆验证（`tools/test-c2-a2-lisp-app.py`、`crates/http-host/src/lib.rs`、`maintenance/tooling-inventory.json`）**：
    - 新增 `Scenario D1`（未读完 `POST` 请求体在 `--timeout-ms 450` 下得 `408`、零副作用且保持同 PID）与 `Scenario D2`（`--timeout-ms 15000` 未读完请求体 + 写满阻塞 stderr 管道 + 外部 `/bin/sleep 5` 持写端条件下 `SIGTERM` `< 3s` 退出并回收后端），以及异步/TCP 保护性单测 `read_body_bounded_enforces_limit_deadline_and_shutdown`（`http-host` `8/8`）。
  - **受控构建、集成验收、保护单测与 `CLN292` 读数（`assistant run`）**：
    - `dist/lisp` 重建（`run_id=6303e1942d5a4ccf9c9e6910451fd829`，`174.563s`，`exit=0`，`cleanup_complete=true`）。
    - Phase 2 验收（`run_id=19a7d42286c04c2f9809d858358cfd5f`，`29.874s`，`exit=0`，`cleanup_complete=true`）：`test-c2-a2-lisp-app.py --selftest`（`PASS`）、`test-c2-a2-lisp-app.py --package dist/lisp`（`PASS`，含 Scenarios A/B/C/D1/D2）、`test-repository-bootstrap.py`（`4/4`）、`check-unsafe-boundary.py`（`442` 活跃文件、`149` `forbid`、`6,327/6,327` `SAFETY:`、`0` 错误）、`check-own-voice.py`（`0` 失败）、`test-tooling-modernization.py`（`33/33`，`1.487s`）。
    - Phase 3 保护性单测（`run_id=dfed7739aeaf4e0685e8a573c98db8d4`，`19.925s`，`exit=0`，`cleanup_complete=true`）：`cargo test -p http-host`（`8/8`）、安全边界与合规扫描 `0` 错误/失败。
    - [CLN292](../maintenance/cleanup/reports/2026-11-24-292-c2-a2-r2-http-body-shutdown.md) 清理 `4` 项临时目录（`4,611` 文件、`1,162` 目录、`8` 符号链接、`895,652,948` 逻辑字节、摘要 `6f5c8f167882402a8cd9f2169bdec6a67321f7fdbb5ed2325db53846e618ffe1`）；当前 `dist/lisp`（`210M`）：`bin/femacs-lisp` `5,053,696 B`（`c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6`）、`bin/femacs-lisp.pdmp` `18,325,840 B`（`39b97e3ee8f7808719db8e846d1a45f4c45f14003e080171b486687d57054071`）、`bin/http-host` `2,147,280 B`（`fb6199f2d01b5b0fa203ec2b84ef48aef535d08c5971970b90767a0a75bacc6f`），strict `codesign` 与 `manifest.json` 核验一致；清理后冒烟 `run_id=89826609e43f4604ab4befba4479c3ed`（`11.059s`，`exit=0`）：`LISP=42`、`HTTP=http-host 0.1.0 (femacs 32.0.50)`、`--package dist/lisp PASS`。保留 `build/terminal`（`45M`）、`dist/macos`（`235M` 与 `/Applications/Fe.app` 同源一致）、`build/generation`（`13M`）与 `build/manuals`（`1.4G`）。

## 历史记录（2026-11-24 Stage C2/A2-R1，已被上方 Stage C2/A2-R2 取代）

- **Phase II Stage `C2/A2-R1`（`8bbe713e`）**：
  - **有界截止与关闭生命周期（`crates/http-host/src/lib.rs`）**：
    - `#![forbid(unsafe_code)]` `run_rpc_worker` 以 `RPC_IO_POLL_SLICE`（`50ms`）切片的 `write_all_with_deadline` 与 `read_bounded_rpc_line_with_deadline` 替代无截止阻塞 I/O，`WorkerJob` 携带绝对 `deadline: Instant`。
    - 出队前淘汰已断连请求（`job.respond.is_closed()`）与已过期排队请求（`Instant::now() >= job.deadline`）；请求超时时立即关闭 socketpair、`kill_child_slot_now` 终止并回收卡死后端进程且 `drain_queued_jobs`，绝不重放；`serve()` 在 `SIGTERM`/`SIGINT` 时置 `shutdown_flag` 并 `Shutdown::Both` 唤醒 worker，随后按先 `reap_child_slot(&child_slot, 500ms)` 再 `worker_handle.join()` 的顺序有界退出，同时保留常规 Lisp `500` 错误后同 PID 续服行为。
  - **忽略规则与生命周期集成/单测（`.gitignore`、`tools/{test-repository-bootstrap.py,test-c2-a2-lisp-app.py}`、`maintenance/tooling-inventory.json`）**：
    - `.gitignore` 新增 `/dist/lisp`，`tools/test-repository-bootstrap.py` 校验 `dist/lisp/manifest.json` 与 `dist/lisp/bin/http-host` 忽略规则；`tools/test-c2-a2-lisp-app.py` 新增 `Scenario B`（断连不执行、`/api/hang` `504`、回收子进程与 `queued-behind-hang` 淘汰）及 `Scenario C`（`--timeout-ms 15000` 忙时 `SIGTERM` `< 3s` 退出、回收子进程、禁止 `queued-at-sigterm` 执行与端口可复用），并保留 `Scenario A`（`500` 后同 PID 恢复）。
  - **受控构建、集成验收、保护单测与 `CLN291` 读数（`assistant run`）**：
    - `dist/lisp` 重建（`run_id=c127f7bd43614fe3914e98193c74d2ae`，`181.142s`，`exit=0`，`cleanup_complete=true`）。
    - Phase 2 验收（`run_id=64d4b408ca8542bcbdd3f776e37dcb01`，`28.208s`，`exit=0`，`cleanup_complete=true`）：`test-c2-a2-lisp-app.py --selftest`（`PASS`）、`test-c2-a2-lisp-app.py --package dist/lisp`（`PASS`）、`test-repository-bootstrap.py`（`4/4`）、`check-unsafe-boundary.py`（`442` 活跃文件、`149` `forbid`、`6,327/6,327` `SAFETY:`、`0` 错误）、`check-own-voice.py`（`0` 失败）、`test-tooling-modernization.py`（`33/33`，`1.074s`）。
    - Phase 3 保护性单测（`run_id=31594183b0ac4307bdb437dffae12af8`，`25.89s`，`exit=0`，`cleanup_complete=true`）：`cargo test -p http-host`（`7/7` 通过），安全边界与合规扫描 `0` 错误/失败。
    - [CLN291](../maintenance/cleanup/reports/2026-11-24-291-c2-a2-r1-http-lifecycle.md) 清理 `4` 项临时目录（`3,761` 文件、`1,020` 目录、`8` 符号链接、`803,855,644` 逻辑字节、摘要 `72075c6ea00064d22d512134f5d960ce5d24ef17aa3d04956eb4036d5f0f16b3`）；当前 `dist/lisp`（`210M`）：`bin/femacs-lisp` `5,053,696 B`（`c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6`）、`bin/femacs-lisp.pdmp` `18,325,840 B`（`42ada548655372605714b99411969e4cba4bb2427b550bab86cfa77aeddd66d0`）、`bin/http-host` `2,089,888 B`（`d91553c1e067e2b563c463bcd02fc8d9de29572bdd1e4e93abf9fd48bc97f39f`），strict `codesign` 与 `manifest.json` 核验一致；清理后冒烟 `run_id=0be5b28c5be4447bae75523a0273a825`（`13.132s`，`exit=0`）：`LISP=42`、`HTTP=http-host 0.1.0 (femacs 32.0.50)`、`--package dist/lisp PASS`。保留 `build/terminal`（`45M`）、`dist/macos`（`235M` 与 `/Applications/Fe.app` 同源一致）、`build/generation`（`12M`）与 `build/manuals`（`1.4G`）。

## 历史记录（2026-11-24 Stage C2 & A2 初版，已被上方 Stage C2/A2-R1 取代）

- **Phase II Stage `C2`（独立产品与应用）与 `A2`（可独立安装与做应用）初版（`17ef5430`，HTTP 生命周期留待上方 `C2/A2-R1` 收口）**：
  - **独立无头 Lisp 运行时与 `femacs-lisp`（`src/emacs.rs`、`crates/femacs-vm/src/host_runtime.rs`、`crates/femacs-host/src/{lib,lisp_cli,bin/femacs_lisp}.rs`）**：
    - 扩展 `init_headless_vm_body` 与 `HostInitConfig`（`data_dir`、`replace_load_path`、`defer_delayed_custom_init`、`eval_silent`、`prepend_load_path`、`load_library`、`funcall_with_args`、`funcall_two_strings_to_string`、`dump_portable`），拦截 `(kill-emacs [ARG])` 执行 `kill-emacs-hook` 并通过不被 `(ignore-errors ...)` 吞掉的 `VmHostError::ExitRequest(i32)` 返回真实退出码。
    - 非 `ColdMinimal` / 非 `--dump-out` 启动路径按标准启动顺序补全 `set-locale-environment`、编码路径解码、`lisp-directory`/`load-history` 与 `custom-delayed-init-variables`（确保 `temporary-file-directory` 等运行时变量正确初始化，同时 `--dump-out` 保持延迟列表可搬移）；新增 `adopt_inherited_rpc_socket`（校验 `AF_UNIX`/`SOCK_STREAM`、`F_DUPFD_CLOEXEC`、stdio 重定向 `/dev/null`）并在每次宿主调用收尾刷新 C `stdout`/`stderr`。
    - 新增 `#![forbid(unsafe_code)]` `lisp_cli` 模块与独立 `femacs-lisp` 可执行程序（`--eval`、`--batch`、`-Q`/`--no-site-*`、`-L`/`-l`/`-f`/`--script`、`--lisp-dir`/`--data-dir`、`--dump-file`/`--no-dump`/`--dump-out`、多行容错 `--repl` 及 `--rpc http` JSON-RPC 2.0 `http.handle` 循环）。
  - **独立 HTTP 宿主、`xtask build-lisp` 打包与共享 Core 示例（`Cargo.{toml,lock}`、`crates/http-host/`、`xtask/src/{main,cargo_build,runtime,standalone}.rs`、`examples/lisp-web/`、`tools/{unsafe-boundary-manifest.json,test-u4-unit.py,test-c2-a2-lisp-app.py}`、`maintenance/tooling-inventory.json`）**：
    - 新增无任何 `femacs-*` 依赖的 `#![forbid(unsafe_code)]` `crates/http-host`（Axum `0.8` + Tokio `1` + Serde），拉起同目录 `femacs-lisp --rpc http`，经 socketpair 转发请求并把子进程 stdout/stderr 引流至 stderr，强制 `--max-body-bytes`（`413`）、有界队列（`503`）、`--timeout-ms`（`504`），Lisp 异常返回 HTTP `500` 后同一后端继续服务，stdout 仅输出 `LISTENING_ADDR=`/`BACKEND_PID=`，`SIGINT`/`SIGTERM` 优雅清理后端并释放端口。
    - 新增 `xtask build-lisp [--out DIR]` 输出无符号链接、附带严格 `codesign` 与 SHA-256 `manifest.json` 的自含包 `dist/lisp`，新增共享 `items-core.el` 的静态站点生成器 `build-site.el` 与 HTTP 应用 `http-app.el`，扩展 `test-u4-unit.py:689` 识别 `*.dylib`/`*.so` 过程宏依赖，并将新 `#![forbid(unsafe_code)]` 文件及 `test-c2-a2-lisp-app.py` 纳入安全边界清单与工具台账。
  - **洁净构建、全套集成验收与保护性单测读数（`assistant run`）**：
    - 洁净构建（`run_id=0d977ae1517f48b09c3556a698f9ef52`，`684.856s`，`exit=0`，`cleanup_complete=true`）：`build-lisp --out dist/lisp`、`build-terminal`、`link-host-test a1_host_acceptance` 与 `build-macos` 全数重新产出。
    - Phase 2 验收：`run_id=dcdd6dc3e9984f3bb463be747809aed6` 中 `test-c1-a1-host.py`（`ok=true`，`22.97s`）、`test-c2-a2-lisp-app.py`（`--selftest PASS` + 洁净 `--export` 与搬移 HTTP E2E）及 `test-u3-unit.py`（`3/3`）通过，而整批因修复前 `test-u4-unit.py` 的 `serde_derive` 搜索问题以 `exit=1` 结束；修复 `tools/test-u4-unit.py:689` 后的 `run_id=1fcaefbae8fc48978a0e20417e57587d`（`exit=0`，`cleanup_complete=true`）通过 `test-u4-unit.py`（`4/4`，`19.3424s`）、`test-u7-safe-subsystems.py`（`4/4`，`22` 模块、`20` 套 ERT、`U7_GC_STRESS_OK gcs=72`）、`check-unsafe-boundary.py`（`442` 活跃文件、`149` `forbid`、`6,327/6,327` `100.0%` `SAFETY:`、`0` 错误）、`check-own-voice.py`（`0` 失败）、`test-repository-bootstrap.py`（`4/4`）与 `test-tooling-modernization.py`（`33/33`）。
    - Phase 3 保护性单测（`run_id=ca0f9b79bb3a434e963a44f64d6fb5f4` `exit=0` + `run_id=0313057f76704aafa756cd349bf7575d` `24.42s`，`exit=0`）：`http-host`（`4/4`）、`xtask`（`118/118`）、`femacs_host_unit_tests`（`9/9`）、`test-c2-a2-lisp-app.py --package dist/lisp`（`PASS`）、`check-unsafe-boundary.py`（`0` 错误）、`check-own-voice.py`（`0` 失败）。
- **最新可交付制品（经 [CLN290](../maintenance/cleanup/reports/2026-11-24-290-c2-a2-standalone-lisp-app.md) 清理 `24` 项临时范围 / `6,174` 文件 / `1,611,442,831` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `dist/lisp`（`210M`）：`bin/femacs-lisp` `5,053,696 B`（SHA-256 `c7038804618fdb5a37a9448091488ef2fb8a1f1452fccc7921a899dd45eb87b6`），`bin/femacs-lisp.pdmp` `18,326,400 B`（SHA-256 `cf7f316dd9a3ad30e19aba1391e607f5818a796c5bfe8e071ee8755c1e8edda2`），`bin/http-host` `2,066,416 B`（SHA-256 `be41cb78687323d4c4399943e2396a88512bf991d5248167444ea0501020776a`）。
  - `build/terminal`（`45M`，`build/terminal/port`）：`src/emacs` `4,714,176 B`（SHA-256 `3e4349c7fc6dffaac00b125d96c5f75ce8abe0bd231b53f2e4df4eb6539615a3`），`src/emacs.pdmp` `13,101,368 B`（SHA-256 `37ecaf1d2d28b2a4f902e5ca4e76eec09f1e1393cfe013984f8b87c7846349d6`），`lib-src/emacsclient` `539,328 B`（`1baf54e6…`），`lib-src/hexl` `451,488 B`（`15f41ba9…`）。
  - `dist/macos/Femacs.app`（`235M`）及原子重装的 `/Applications/Fe.app`（`235M`，`diff -rq` 空）：`Contents/MacOS/Femacs` `6,247,232 B`（SHA-256 `74e6fa0c5a4864ae1169d402c5f05b24925ae89c1ca7f061d4a066c46fdc851e`），`Contents/Resources/libexec/Femacs.pdmp` `18,600,760 B`（SHA-256 `e0696518a6bf09e272edd64bd13d36b7a98d5f8cc4725f02228783c2b03b6ded`），`emacsclient` `538,784 B`（`ca959192…`），`hexl` `451,120 B`（`2ee11f3c…`）；`dist/lisp` 二进制及双 `.app` `codesign --verify --deep --strict` 全过（`exit 0`），清理后冒烟 `femacs-lisp=42`、`http-host=http-host 0.1.0 (femacs 32.0.50)`、Terminal/Cocoa/`Fe.app`/`femacs`/`fe` 均正常返回 `42` 与 `treesit`/`gnutls`、`feclient` `emacsclient 32.0.50`（`run_id=dafaf76fbeea4132a2b2c2a9e70bcda1`，`7.31s`）。保留 `build/generation`（`12M`）与 `build/manuals`（`1.4G`）。

## 历史记录（2026-11-24 Stage C1-R1，已被上方 Stage C2 & A2 取代）

**本段为历史读数（Phase II Stage C1-R1 公开宿主边界收口，关闭 Codex `[FEMACS-C1-A1-REVIEW]` 阻断项），取代下方初版 `C1/A1`、`U8-R4` 及更早历史段落。**

- **Phase II Stage `C1-R1`（`[FEMACS-C1-A1-REVIEW]` 两项公开边界闭环）已完成、全套集成/单元验收复过，并完成 `CLN289` 与 `/Applications/Fe.app` 原子重装，待 Codex App 确认后进入 Stage `C2`**：
  - **公开宿主边界收口（`crates/femacs-vm/src/host_runtime.rs`、`crates/femacs-host/src/{lib,bin/temacs}.rs`）**：
    - 移除 `host_runtime::with_anchored_context`、公开 `init_headless_host`/`shutdown_headless_host`、`femacs_host::LispHost::with_context` 及 `pub use femacs_vm;`，将原始 `GcContext` 租约收敛至不透明、非 `Copy`、!Send + !Sync 的 `pub struct HeadlessVm`（`Drop` 负责释放并调用 `unbind_vm_lock_owner`），外部安全代码无法绕过 condition 保护或越过宿主生命周期持有 `Rooted`。
    - 新增 `with_host_condition_catch_ctx` 统一为原生主机调用与 `safe_domain`（`HeadlessVm::{exercise_safe_domains, safe_domain_read_eval}`、`LispHost::{exercise_safe_domains, read_eval_safe_domain}`）安装栈锚与外层 `CONDITION_CASE(t)`，确保 `(signal 'error ...)` / `(signal 'quit nil)` / `quit-flag` 可按 `HostError` 恢复续用且仅返回宿主自有值；更新 `temacs.rs` 直接引用 `femacs_vm::editor_main`，并在 `femacs-host` 单测增加 `HeadlessVm` `!Send + !Sync` 编译与运行校验。
  - **真实子进程与外部安全边界复验脚手架（`crates/femacs-host/tests/a1_host_acceptance.rs`、`tools/test-c1-a1-host.py`、`tools/test-u4-unit.py`、`tools/unsafe-boundary-manifest.json`、`maintenance/tooling-inventory.json`）**：
    - `a1_host_acceptance` 将 `safe-domain-with-context` 更新为 `safe-domain-protected`，在真实子进程验证 `exercise_safe_domains` 与 `read_eval_safe_domain` 遇 `error`/`quit`/`quit-flag` 后的完整恢复与 GC 平衡；`test-c1-a1-host.py --selftest`（`4/4`）明文拒绝 `with_anchored_context`、`with_context` 与 `pub use femacs_vm;`。
    - `tools/test-u4-unit.py` Group 4 Part 4a 改为编译 `cargo check -p femacs-host`，实测 `43` 项 `femacs-vm` + `4` 项 `femacs-host` `#![forbid(unsafe_code)]` 外部 `compile-fail` 探针与 `real_pass`/`host_pass` `compile-pass` 检查。
  - **清洁重建与定向复验读数（`assistant run`）**：
    - 清洁重建（`run_id=df83870b1c774796b3992045510f8105`，`792.441s`，`exit=0`，`cleanup_complete=true`）：`xtask` 单测 `116/116`、Terminal、`a1_host_acceptance`、`femacs_host_unit_tests` 与 Cocoa `dist/macos/Femacs.app` 基于最新源码重新产出。
    - 定向复验（`run_id=862bac45632a442ea15f874b5ae65b50`，`91.604s`，`exit=0`，`cleanup_complete=true`）：`femacs_host_unit_tests` `3/3`、`test-c1-a1-host.py --selftest` `4/4`、`test-c1-a1-host.py` `ok=true`（`24.9797s`：Terminal 与 Cocoa `438` 活跃文件 / `145` `forbid` / `6,308/6,308` `100.0%` `SAFETY:` / `0` `static mut` / `nm -gU` 与 `check-libshadows.py` 全过；`13` 组 A1 子进程场景含常态与 `FEMACS_GC_SKIP_STACK_SCAN=1` 下 `safe-domain-protected` 错误/`quit` 恢复；双制品 `(t 42 "alpha gamma\n")`）、`test-u4-unit.py` `4/4`（`21.5199s`）、`test-u3-unit.py` `3/3`（`8/8` 单测）、`test-u7-safe-subsystems.py` `4/4`（`22` 模块、`20` 套核心 ERT、`U7_GC_STRESS_OK gcs=72`）、`check-own-voice.py`（`0` 失败）、`test-repository-bootstrap.py`（`4/4`）、`test-tooling-modernization.py`（`33/33`，`1.233s`）。
- **最新可交付制品（经 [CLN289](../maintenance/cleanup/reports/2026-11-24-289-c1-a1-r1-boundary.md) 清理 `23` 项临时范围 / `2,709` 文件 / `901,433,547` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `build/terminal`（`45M`，`build/terminal/port`）：`src/emacs` `4,714,176 B`（SHA-256 `7ed29ec69c0d766a96be0a8a1e9e1ab9088677834b35b1537ae0c8e16c190156`），`src/emacs.pdmp` `13,099,280 B`（SHA-256 `520ae43db3133cce75491147eab293cb07d1dd67321ec7457acf47e316f1acc4`），`lib-src/emacsclient` `539,328 B`（`e004d4be…`），`lib-src/hexl` `451,488 B`（`570f90f2…`）。
  - `dist/macos/Femacs.app`（`235M`）及原子重装的 `/Applications/Fe.app`（`235M`，`diff -rq` 空，严格一致）：`Contents/MacOS/Femacs` `6,247,232 B`（SHA-256 `15814a9bf21d80be7ca8c67b99b205144ac290e82fedbdebd34966094e492823`），`Contents/Resources/libexec/Femacs.pdmp` `18,598,664 B`（SHA-256 `7ad5ff14f03f53075cce2363c7a5d7b1576016e61b73b02a4cd9c57088893c4e`），`emacsclient` `538,784 B`（`6a4d0208…`），`hexl` `451,120 B`（`8758c7f5…`）；双 `.app` `codesign --verify --deep --strict` `exit 0`，直接产物与已安装 app `-Q --batch` 冒烟返回 `(3 t nil nil t t)` / `(3 t nil t t t)`，`femacs`/`fe` 返回 `(3 t t)`，`feclient --version` 返回 `emacsclient 32.0.50`（`run_id=86fbda6fe94343be8826de83c583ad5a`，`1.125s`）。保留 `build/generation`（`8.3M`）与 `build/manuals`（`1.1G`）。

## 历史记录（2026-11-24 Stage C1 & A1 初版，已被上方 Stage C1-R1 取代）

**本段为历史初版读数（Phase II Stage C1 & A1，继 `[FEMACS-AU8-FINAL-REVIEW-R4]` `c187c486` 核收 `AU8` 后）。**

- **Phase II Stage `C1`（宿主边界）与 `A1`（VM 同源且编辑器保真）已完整实现、全套集成/单元验收通过，并完成 `CLN288` 与 `/Applications/Fe.app` 原子重装，可进入 Stage `C2`**：
  - **无头 VM 与 `#![forbid(unsafe_code)]` 宿主边界（`src/emacs.rs`、`crates/femacs-vm/src/{lib,host_runtime}.rs`、`crates/femacs-host/src/{lib,bin/temacs}.rs`、`crates/femacs-host/Cargo.toml`）**：
    - `src/emacs.rs` 提取共享 VM 启动函数（`grow_regex_stack_limit`、`init_vm_once_tables`、`init_vm_core_subsystems`、`syms_of_vm_all`、`init_vm_display_subsystems`）而不改变 `emacs_init_body` 语义，并新增 `init_headless_vm_body` 负责无图形窗口、无控制 TTY 的单进程冷启动或 `.pdmp` 恢复。
    - `crates/femacs-vm/src/host_runtime.rs`（`#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]`）以 `HOST_INIT_STATE` CAS 保证进程单次初始化（`AlreadyInitialized`）、维护只增不缩的嵌套栈锚（`with_host_stack_anchor`）、捕获并分类 Lisp 退出（`VmHostError::{AlreadyInitialized, InitFailed, Read, LispCondition, Quit, Io}`），提供 `read_eval_to_string`、`load_script_file`（绑定 `command-line-args-left`/`argv` 并经 `UnbindGuard` 清理）、`garbage_collect`、`inspect_headless_environment`（`HeadlessDiagnostics`）、`with_anchored_context` 与退出/失败时 `unbind_vm_lock_owner`。
    - `crates/femacs-host/src/lib.rs` 转为 `#![forbid(unsafe_code)]`，导出 `!Send + !Sync` 句柄 `LispHost` 并内置 `#![forbid(unsafe_code)]` `#[cfg(test)]` 保护性单测（`3/3`，含 `LispHost`/`GcContext` `!Send + !Sync` 编译期断言）；`crates/femacs-host/src/bin/temacs.rs` 经 `femacs_host::femacs_vm::editor_main` 入口保持编辑器原启动行为。
  - **统一 Cargo 链接参数传播与集成验证脚手架（`xtask/src/{standalone,cargo_build,main}.rs`、`crates/femacs-host/tests/a1_host_acceptance.rs`、`tools/test-c1-a1-host.py`、`tools/{unsafe-boundary-manifest.json,test-u4-unit.py}`、`maintenance/tooling-inventory.json`）**：
    - `xtask/src/cargo_build.rs` 改由 `CARGO_ENCODED_RUSTFLAGS` 统一传递目标链接参数（同时覆盖前置 `bin temacs`、集成 `--test` 与 `--lib` 单测可执行产物），扩展 `link-host-test lib` 并统一遍历 Cargo `deps/` 与 `release/build/*/out/` 嵌套目录（`cargo test -p xtask` `116/116` 通过）。
    - 将 `crates/femacs-host/src/lib.rs` 移入 `forbid_unsafe_modules`，在 `isolated_unsafe_boundaries` 登记 `crates/femacs-vm/src/host_runtime.rs`，新增 `#![forbid(unsafe_code)]` `harness = false` 实机验收二进制 `a1_host_acceptance` 与综合门禁 `tools/test-c1-a1-host.py`。
  - **最终源码树清洁重建与全套验证（`assistant run`）**：
    - 清洁重建（`run_id=75e175dbb51846a48ef453182e51fbe1`，`571.551s`，`exit=0`，`cleanup_complete=true`）：`xtask` 单测 `116/116`、Terminal、`a1_host_acceptance`、`femacs_host_unit_tests` 与 Cocoa `dist/macos/Femacs.app` 基于同一终稿源码树全部产出。
    - 复验流水线（`run_id=cb1531690af3400fbaff363eff92182e`，`116.46s`，`exit=0`，`cleanup_complete=true`）：
      - `femacs_host_unit_tests`：`3/3` 通过；`tools/test-c1-a1-host.py --selftest`：`4/4` 通过。
      - `tools/test-c1-a1-host.py`（`ok=true`，`37.7158s`）：`static-and-abi-gates`（`34.1777s`，Terminal 与 Cocoa 双制品验证 `438` 活跃文件：`forbid_unsafe_files=145`、`allow_*=0`、`static_mut_items=0`、`stitching_includes=0`、`unsafe_blocks=6,308` 且 `SAFETY:` `6,308/6,308` `100.0%`、`nm -gU` ABI 导出与 `check-libshadows.py` 通过）；`a1-host-lifecycle-and-scripts`（`2.1780s`，`13` 组子运行覆盖 `cold-minimal-lifecycle` / `headless-diagnostics` 常态与 `FEMACS_GC_SKIP_STACK_SCAN=1`、无 GUI/TTY 指标 `noninteractive=t` / `inhibit_window_system=true` / `display_graphic_p=false` / `1` 个 `"t"` frame 与 terminal、`cold-loadup-and-script`、`safe-domain-with-context` 常态与 `.pdmp` `SKIP_STACK_SCAN=1`、`self-dump-create`、`dump-file-roundtrip` 常态与 `SKIP_STACK_SCAN=1`、坏/错指纹 `.pdmp` 拒绝、单次初始化拒绝与 `Drop` 后 `GcContext::current().is_none()`）；`same-source-editor-fidelity`（`1.3599s`，Terminal 与 Cocoa 均输出 `(t 42 "alpha gamma\n")`）。
      - `test-u4-unit.py` `4/4`（`25.8485s`）、`test-u3-unit.py` `3/3`（`8/8` 单测）、`test-u7-safe-subsystems.py` `4/4`（`22` 模块、`20` 套核心 ERT、`U7_GC_STRESS_OK gcs=72`）、`cargo test -p femacs-pure -p femacs-sys --lib`（`10/10` + `3/3`）、`check-own-voice.py`（`0` 失败）、`test-repository-bootstrap.py`（`4/4`）、`test-tooling-modernization.py`（`33/33`，`1.868s`）全过。
- **最终可交付制品（经 [CLN288](../maintenance/cleanup/reports/2026-11-24-288-c1-a1-host-boundary.md) 清理 `22` 项临时范围 / `3,894` 文件 / `1,001,098,146` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `build/terminal`（`45M`，`build/terminal/port`）：`src/emacs` `4,714,176 B`（SHA-256 `9952ddd4710feebaaf572479dc1ebd9f6ab271bde9e81fac794c68a42fb88fa0`），`src/emacs.pdmp` `13,101,368 B`（SHA-256 `96828ce7a7c1db398639af0d40a1c5d803b4facd07a3834c1d30fcc59d084fa9`），`lib-src/emacsclient` `539,328 B`（`1baf54e6…`），`lib-src/hexl` `451,488 B`（`15f41ba9…`）。
  - `dist/macos/Femacs.app`（`235M`）及原子重装的 `/Applications/Fe.app`（`235M`，`diff -rq` 空，严格一致）：`Contents/MacOS/Femacs` `6,247,232 B`（SHA-256 `e007fa58055a1d457afa7574d127e1cecce1f1b3d56eccd72ed75fbc0790eefe`），`Contents/Resources/libexec/Femacs.pdmp` `18,600,760 B`（SHA-256 `816242cf56bc4a4011cfe7dae55b4abdbaed34215baa6dbdb00b027bad6310f5`），`emacsclient` `538,784 B`（`ca959192…`），`hexl` `451,120 B`（`2ee11f3c…`）；双 `.app` `codesign --verify --deep --strict` `exit 0`，直接产物与已安装 app `-Q --batch` 冒烟返回 `(3 t nil nil t)`，`femacs`/`fe` 返回 `(3 t)`，`feclient --version` 返回 `emacsclient 32.0.50`（`run_id=98e08a361bd341fb87966635eba108fb`，`1.865s`）。保留 `build/generation`（`7.6M`）与 `build/manuals`（`971M`）。

## 历史记录（2026-11-24 U8-R4，已被上方 Stage C1 & A1 取代；`c187c486` 获 Codex 终审 R4 核收）

- **去 `unsafe` 先行阶段 `U8-R4` 真实 `femacs-vm` 公开 API 信任边界收口已闭环，定向实测复验、`CLN287` 与 `/Applications/Fe.app` 原子重装已完成**：
  - **R3 阻断项 #1 收口（VM 全局变量与内部模块私有化）**：
    - `src/thread.rs` 将带 `#[unsafe(no_mangle)]` 的 `current_thread` 与 `all_threads` 缩窄为 `pub(crate) static`（保留外部 Mach-O `D _current_thread` / `D _all_threads` 导出，切断外部安全 Rust 通过 `VmGlobal::set` 改写正在使用线程指针的通道）。
    - `crates/femacs-vm/src/state.rs` 新增安全只读查询 `state::{has_active_thread, current_thread_addr}`；`vm_primitives.rs` 提供 `pub unsafe fn editor_main`（由 `lib.rs` 重导出为 `femacs_vm::editor_main` 并更新 `crates/femacs-host/src/lib.rs`，`lib.rs` 自身保持 `0` 处 `unsafe`）；`crates/femacs-vm/src/lib.rs` 将 `femacs_sys`、`VmGlobal`、`common`、`frame_image_accessors`、`buildobj`、`safe_cmds`..`safe_window` 及全部 `82` 个 C 移植模块（含 `thread`、`emacs`）缩窄为 `pub(crate)`，公开接口仅保留 `femacs_pure`、`nonlocal`、`roots`、`state`、`vm_primitives`、`safe_domain` 与 `editor_main`。
  - **R3 阻断项 #2 收口（堆对象严格收敛到持根句柄或宿主自有值，封堵裸字安全重登记）**：
    - `crates/femacs-vm/src/roots.rs` 新增 `Clone + Copy + Send + Sync` 的立即值类型 `pub struct Immediate(Lisp_Object)`（`nil`/`from_fixnum`/`is_nil`/`as_fixnum`）；将 `LispVal` 缩窄为 `pub(crate) struct LispVal(Lisp_Object)`（移除公开 `Send + Sync`，将内部 `unsafe impl Send` 迁移到 `ThreadRootSlab` 与 `PersistentSlot`），并将 `NoGcToken<'a>` / `UnrootedBorrow<'nogc, T>` 标记为 `!Send + !Sync`。
    - 从 `GcContext` / `Rooted` / `Handle` / `MutableHandle` / `PersistentRoot` 移除公开 `LispVal` 提取与裸字构造（`Rooted::new`/`get`、`PersistentRoot::new`/`get`、`Handle::get`、`MutableHandle::set`），统一由安全句柄/立即值接口（`GcContext::{root_immediate, root_handle}`、`Rooted::{from_immediate, from_handle, handle, handle_mut, is_nil, as_fixnum, as_immediate, same_object, set_immediate, set_from_handle}`、`PersistentRoot::{from_immediate, from_handle, handle, set_immediate, set_from_handle, release}`）与 `pub(crate)` `*_raw` 内核入口承担；将 C-ABI 原始值探针 `femacs_vm_persistent_root_{get,release}` 显式标为 `pub unsafe extern "C" fn`（对齐 `femacs_vm_persistent_root_alloc`，保留原符号与 C ABI）；更新 `crates/femacs-vm/src/vm_primitives.rs`、`safe_domain.rs`（`#![forbid(unsafe_code)]`）、`src/eval.rs::Fprog1` 与 `src/emacs-module.rs`。
  - **真实 `femacs-vm` crate 外部安全边界门禁（`tools/test-u4-unit.py`、`tools/test-u3-unit.py`、`tools/test-u7-safe-subsystems.py`、`maintenance/tooling-inventory.json`）**：
    - `tools/test-u4-unit.py` Group 4 Part 4a 对真实 `cargo check -p femacs-vm` 产出的 `libfemacs_vm-*.rmeta` 直接编译验证 `1` 份真实 `#![forbid(unsafe_code)]` compile-pass 流程与 **`38`** 项 `#![forbid(unsafe_code)]` 外部 `compile-fail` 探针（涵盖 `current_thread.set`/`get`、`all_threads.set`、`femacs_sys`/`VmGlobal`、私有模块、`unsafe editor_main`、私有 `LispVal`、已删 `Rooted`/`PersistentRoot`/`Handle` 裸字入口、全部 5 项 `unsafe extern "C" fn`、`!Send`、`Rooted` 非 `Copy` move 以及 `'static`/drop 后 `Handle` 与跨 GC `UnrootedBorrow` 生命周期门禁），叠加 Part 4b 运行时多 OS 线程 `Mutex` 锁交接与乱序释放；同步更新 `test-u3-unit.py`、`test-u7-safe-subsystems.py --selftest` 与 `maintenance/tooling-inventory.json`：`test-u4-unit.py` `4/4`（`19.7262s`，Group 4 `17.4656s`）、`test-u3-unit.py` `3/3`（`8/8` 测试）、`test-u7-safe-subsystems.py --selftest` `5/5` 全过。
  - **活跃产品闭包（`436` 份文件，Terminal 与 Cocoa 双制品验证通过）**：`#![forbid(unsafe_code)]` **`143`** 份、`allow_*` **`0`** 份、`static mut` **`0`** 项、拼接 `include!` **`0`** 项、`unsafe_blocks` **`6,278`**、`SAFETY:` 契约 **`6,278 / 6,278`（`100.0%`）**、`unsafe_fn_defs` **`4,366`**、`extern_c_fn_defs` **`2,325`**、`extern_c_blocks` **`834`**、`unsafe_impl_traits` **`10`**；双产物 `nm -gU` 确认 `_current_thread`、`_all_threads`、`_Vmain_thread`、`_lispsym`、`_Fall_threads`、`_Fcurrent_thread` 与全部 6 项 `_femacs_vm_*` 导出。
  - **`U8-R4` 重建与定向复验全过（`555` 语料、Cocoa UI 与 `clean-export` 依 Codex R3 意见沿用 `502e87b8` 基线）**：
    - `assistant run --profile build`（`run_id=7c99d5d8614b4d29b18ff901941aa991`，`2084.927s`，`exit=0`，`cleanup_complete=true`）：重建 Terminal、Cocoa 与 `build/runtime/u8-native`（`normal` + `checking`）。
    - `test-u4-gc-roots.py --native-dir build/runtime/u8-native`：`4/4` 组通过（`26.6718s`，`run_id=f2087eb56ed24d3fa08bd9c17d7f71db`）；`test-u3-nonlocal-exit.py`：`4/4` 通过（`passed=4, failed=0`，`19.183s`，`run_id=63149c42fc954953adbf4068dc199038`）；`test-u7-safe-subsystems.py`：`4/4` 通过（`33.166s`，`22` 模块、`20` 套核心 ERT、`U7_GC_STRESS_OK gcs=72`，`run_id=ed556d7571d045e3a113aa795ba42df2`）。
    - `test-u8-au8-integration.py --group performance`（`95.31s` / `95.582s`，`run_id=4154b0cde7c84ec3b7a34f625a36b394`，`exit=0`，`cleanup_complete=true`）：Cocoa 与 Terminal `bench.py --gate`（`10` 项基线，`startup` 单列观测）均为 **`regressions == []`**（Cocoa `startup 24.6953 vs 150.4351 ms`、`gc 30.5989 vs 49.1281 ms`、`edit 7.6048 vs 9.3179 ms`、`regex 2.3210 vs 2.6100 ms`、`regexp-repl 2.9280 vs 2.6922 ms`、`load 1.5869 vs 1.6670 ms`；Terminal `startup 24.3556 vs 149.5969 ms`、`gc 30.4589 vs 49.3710 ms`、`edit 7.5810 vs 9.4881 ms`、`regex 2.2531 vs 2.7440 ms`、`regexp-repl 2.7831 vs 2.8172 ms`、`load 1.5500 vs 1.7309 ms`）；`delivered-terminal-vs-reference-bytecode`（`bytecode 0.1094`、`edit 1.1536`、`gc-churn 0.6484`、`gc-retained 0.6296`、`load 0.2710`、`regex 1.0440`）、`delivered-cocoa-vs-reference-bytecode`（`bytecode 0.1094`、`edit 1.1288`、`gc-churn 0.8948`、`gc-retained 0.7436`、`load 0.5332`、`regex 0.9948`）与 `runtime-modes-comparison`（`passed=true`）全过；注：因宿主负载均值余波，该次带 `quiet=False`（`max_load_per_core=0.465` / `0.427`），参照读数正常且无门禁回退。
    - `cargo test -p femacs-pure -p femacs-sys -p xtask --lib`（`10/10` + `3/3`）、`check-own-voice.py`（`0` 失败）、`test-repository-bootstrap.py`（`4/4`）、`test-tooling-modernization.py`（`33/33`）、`test-u8-au8-integration.py --group selftest`（`12/12`）全过（`run_id=bcf605078e3944f5afe65913f3ffa973`，`50.725s`）。
- **最终可交付制品（经 [CLN287](../maintenance/cleanup/reports/2026-11-24-287-u8-r4-real-vm-api-closure.md) 清理 `15` 项临时范围 / `12,218` 文件 / `2,397,113,488` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `build/terminal`（`45M`，`build/terminal/port`）：`src/emacs` `4,714,976 B`（SHA-256 `1bdc6b3dd72727f52016ae0c03aae4837db856fa6f186985707d86cbf07cf622`），`src/emacs.pdmp` `13,101,368 B`（SHA-256 `58068e6bad108bd51ae2775a63f8d2bfa4024254dfd033736fbe9ed524b11461`），`lib-src/emacsclient` `539,328 B`（`1baf54e6…`），`lib-src/hexl` `451,488 B`（`15f41ba9…`）。
  - `dist/macos/Femacs.app`（`235M`）及原子重装的 `/Applications/Fe.app`（`diff -rq` 空，严格一致）：`Contents/MacOS/Femacs` `6,264,448 B`（SHA-256 `bf922320cfb293252d164f0b355fe371e4097cdee621b4c75011acab34c7586e`），`Contents/Resources/libexec/Femacs.pdmp` `18,600,760 B`（SHA-256 `dea50536212d2eb37c00fbd4f2307dc2746aa51b8b0add21cfc8dfd34b337491`），`emacsclient` `538,784 B`（`ca959192…`），`hexl` `451,120 B`（`2ee11f3c…`）；双 `.app` `codesign --verify --deep --strict` `exit 0`，两份直接产物与已安装 app `-Q --batch` 冒烟返回 `(3 t nil nil t)`，`femacs`/`fe` 返回 `(3 t)`，`feclient --version` 返回 `emacsclient 32.0.50`（`run_id=3986919557084c79b338d53f82dc032f`，`1.333s`）。

## 历史记录（2026-11-24 U8-R3，已被上方 U8-R4 取代；`corpus-555` 与 `macos-ui` 全量读数沿自 `502e87b8`）

- **去 `unsafe` 先行阶段 `U8-R3` 边界收紧已闭环，定向实测复验、`CLN286` 与 `/Applications/Fe.app` 原子重装已完成，待 Codex 独立终审 R3 核收后解锁 `C1–C6`**：
  - **R2 唯一阻塞项收口（VM 内核 `safe/unsafe` API 边界 + C-ABI 裸指针收紧）**：
    - `crates/femacs-vm/src/roots.rs`：`LispVal::from_raw` 收紧为 `pub unsafe fn`；`bind_vm_lock_owner` / `unbind_vm_lock_owner` 收紧为 `pub(crate) unsafe fn`（校验 `prev_tid == 0 || prev_tid == tid`、无活跃 `GcContext`/slab/nonlocal 借用，并将 `TLS_SLAB.vm_lock_key == thread_key` 纳入 `is_vm_lock_owner`）；`mark_thread_u4_slab_roots` / `cleanup_thread_u4_slab` / `mark_persistent_roots` 收紧为 `pub(crate) unsafe fn` 且强制断言当前 OS 线程物理持有 VM 全局锁（slab 接口并断言 `!borrowed`）；`ScopedRawSpan` / `RootedVec` / `rooting_counters()` 收紧为 `pub(crate)`（另提供安全 `GcContext::rooting_counters(&self)`）；`with_narrowed_stack_bottom` 收敛到 `GcContext::with_narrowed_stack_bottom(&mut self, ...)`；裸指针 C-ABI `femacs_vm_persistent_root_alloc` 显式标为 `pub unsafe extern "C" fn`（保留原 `#[unsafe(no_mangle)]` 符号与 C 调用约定）。
    - `crates/femacs-vm/src/{nonlocal,state,vm_primitives,safe_domain}.rs` 与 `src/comp.rs`：同类入口（`ScopedLispRoot`、`cleanup_thread_state`、`mark_thread_explicit_roots`、`raise/resume_vm_nonlocal`、`*_comp_native_exit`、状态观测与遥测写入）统一收敛到 `pub(crate)` / 持锁断言；`VmState` / `ThreadState` 标记 `!Send + !Sync`；`femacs_vm_host_drop_counts` 与 `femacs_vm_u4_root_stats` 收紧为 `pub unsafe extern "C" fn`；更新 `vm_primitives.rs` 12 处 `LispVal::from_raw` 与 `src/comp.rs` 调用点 `SAFETY:` 文档；`safe_domain.rs`（`#![forbid(unsafe_code)]`）迁至 `ctx.with_narrowed_stack_bottom`。
    - 外部安全边界保护回归：`tools/test-u4-unit.py` `4/4` 组通过（含 `12` 项库内测试 + 独立编译 `libfemacs_vm_boundary.rlib` 验证 `12` 项 `#![forbid(unsafe_code)]` 外部 compile-fail + `#![forbid(unsafe_code)]` 外部运行时 `Mutex` 锁交接与乱序 `Rooted` 释放），`tools/test-u3-unit.py` `3/3`（`8/8` 测试 + C-ABI / 可见性结构断言），`tools/test-u7-safe-subsystems.py --selftest` `5/5`。
  - **额外定位并修复的 `FEMACS_GC_SKIP_STACK_SCAN=1` 根保护缺陷**：在 `src/eval.rs::eval_sub` 将 `maybe_gc ()` 移至 `SPECPDL_BACKTRACE` 入栈之后（对齐 `Ffuncall` 与 `bc_call_housekeeping_slow`），避免直接 `readevalloop -> eval_sub`（`.elc` 无 `macroexpand` 路径）新读入的 `form` 在无栈扫描时被入口 GC 回收触发 `SIGBUS`；同时在 `src/lread.rs` 修复 `readevalloop` 的 `val = unbind_to (count1, val)` 及 `readevalloop_eager_expand_eval` 的 `_subforms_span` 根保护，并在 `tools/test-u4-gc-roots.py` 增补 `.elc` 回归用例。
  - **活跃产品闭包（`436` 份文件，Terminal 与 Cocoa 双制品验证通过）**：`#![forbid(unsafe_code)]` **`143`** 份、`allow_*` **`0`** 份、`static mut` **`0`** 项、拼接 `include!` **`0`** 项、`unsafe_blocks` **`6,264`**、`SAFETY:` 契约 **`6,264 / 6,264`（`100.0%`）**、`unsafe_fn_defs` **`4,358`**、`extern_c_fn_defs` **`2,325`**、`extern_c_blocks` **`834`**、`unsafe_impl_traits` **`8`**；双产物 `nm -gU` 确认全部 6 项 `_femacs_vm_*` 导出。
  - **`U8-R3` 定向复验全过（`555` 语料与 Cocoa UI 依 Codex R2 意见沿用 `502e87b8` 基线）**：
    - `test-u4-gc-roots.py --native-dir build/runtime/u8-native`：`4/4` 组通过（`26.5003s`，覆盖解释/字节码/`FEMACS_GC_SKIP_STACK_SCAN=1`、Cocoa `.elc`、原生 `.eln` normal/checking、安全子系统域、`.pdmp` dump/restore）。
    - `test-u3-nonlocal-exit.py`：`4/4` 通过（`passed=4, failed=0`，`18.458s`）。
    - `test-u7-safe-subsystems.py`：`4/4` 通过（`31.714s`，`22` 模块、`20` 套核心 ERT、`U7_GC_STRESS_OK gcs=72`）。
    - `test-u8-au8-integration.py --group performance`（`91.954s`）：Cocoa 与 Terminal 的 `bench.py --gate`（`10` 项基线，`startup` 单列观测）均为 **`regressions == []`**（Cocoa `startup 24.497 vs 150.267 ms`、`gc 30.727 vs 49.971 ms`、`edit 7.699 vs 9.517 ms`、`regex 2.369 vs 2.691 ms`、`load 1.653 vs 1.668 ms`；Terminal `startup 24.328 vs 148.987 ms`、`gc 31.534 vs 50.368 ms`、`edit 7.659 vs 9.330 ms`、`regex 2.196 vs 2.699 ms`、`load 1.593 vs 1.725 ms`）；`delivered-comparison` 与 `runtime-modes-comparison` 全过（`delivered-terminal-vs-reference-bytecode`: `bytecode 0.1094`, `edit 1.1435`, `gc-churn 0.6806`, `gc-retained 0.6511`, `load 0.3030`, `regex 1.0888`）。
    - `cargo test -p femacs-pure -p femacs-sys -p xtask --lib`（`10/10` + `3/3`）、`check-own-voice.py`（`0` 失败）、`test-repository-bootstrap.py`（`4/4`）、`test-tooling-modernization.py`（`33/33`）、`test-u8-au8-integration.py --group selftest`（`12/12`）全过。
- **最终可交付制品（经 [CLN286](../maintenance/cleanup/reports/2026-11-24-286-u8-r3-vm-boundary-tightening.md) 清理 `15` 项临时范围 / `12,122` 文件 / `1,923,285,065` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `build/terminal`（`45M`，`build/terminal/port`）：`src/emacs` `4,732,480 B`（SHA-256 `9464839edbe417be4006fef84a0093a1f9c52b056a39b11649cc3cf77e85c1cc`），`src/emacs.pdmp` `13,101,368 B`（SHA-256 `66f1d81d06b4864d8a920ae3e9aa9a65d468c3397eb166eba7f4b65c570a33a8`），`lib-src/emacsclient` `539,328 B`（`1baf54e6…`），`lib-src/hexl` `451,488 B`（`15f41ba9…`）。
  - `dist/macos/Femacs.app`（`235M`）及原子重装的 `/Applications/Fe.app`（`diff -rq` 空，严格一致）：`Contents/MacOS/Femacs` `6,282,528 B`（SHA-256 `54d884c1a3c41c8b2889460cde2d500923cfe00accf2f8a25863bab61c839283`），`Contents/Resources/libexec/Femacs.pdmp` `18,600,760 B`（SHA-256 `92db122b012034a7e301323d239ca43cb778ca6ff62912b09e6767e53ac88530`），`emacsclient` `538,784 B`（`ca959192…`），`hexl` `451,120 B`（`2ee11f3c…`）；双 `.app` `codesign --verify --deep --strict` `exit 0`，两份直接产物与已安装 app `-Q --batch` 冒烟返回 `(3 t nil nil t)`，`femacs`/`fe` 返回 `(3 t)`，`feclient --version` 返回 `emacsclient 32.0.50`。

## 历史记录（2026-11-24 R2，已被上方 U8-R3 取代；`corpus-555` 与 `macos-ui` 全量读数沿自 `502e87b8`）

- **去 `unsafe` 先行阶段 `U8-R2` 复核阻断已全闭环（`P-488`），`AU8` 四组严格重验通过、`CLN285` 与 `/Applications/Fe.app` 原子重装已完成，待独立复核确认后进入 `C1–C6`**：
  - 修复三项阻断与性能 `--gate` 缺口：`GcContext::current()` VM 锁单租约与线程绑定及 `Rooted` 代际空闲链表独立槽位释放（阻塞 #1）；依授权按方案 A 对齐协同计划 `§0.1`、边界 `VmResult<T>` / `.eln` / 动态模块 / `run_thread` 契约与 `test-u3-unit.py`（阻塞 #2）；取消按名盲豁免、`adjudicate_555_runs` 全部 76 个差异/未完成入口接入 12 份外部对照包并经 `stable-classification` `_accept_rule`/`assess` 零未解释阻断/零结构失败闭合（阻塞 #3）；落地 `docs/baseline/bench-{cocoa,terminal}-ga8.tsv` 并强制 `--gate` `regressions == []`。
  - **活跃产品闭包（`436` 份文件）**：`#![forbid(unsafe_code)]` **`143`** 份、`allow_*` **`0`** 份、`static mut` **`0`** 项、拼接 `include!` **`0`** 项、`unsafe_blocks` **`6,245`**、`SAFETY:` 契约 **`6,245 / 6,245`（`100.0%`）**、`unsafe_fn_defs` **`4,338`**。
  - **`AU8-R2` 四组全过**：`clean-export`（`930.176s`，`7,722` 项跟踪导出，搬移 GUI/batch 全过）、`macos-ui`（`137.027s`，14 项 GUI/IME、`447` 多窗回调、`26/31` Finder 拖放、`16` 项异常 TTY）、`corpus-555`（`5625.912s`，555 入口 `8,685` 用例与 76 严格入口 `1,031` 用例 `0` 未解释阻断、`u8-native-normal`/`checking` 各 `11/11` / `603` 用例 `0` 阻断、`10` 项 ERT 与 `runtime-execution.py` `prepare_negative=1`/`young=120`/`major=44` 全过）、`performance`（`103.607s`，Cocoa 与终端 `bench.py --gate` 零回退）。
- **最终可交付制品（经 [CLN285](../maintenance/cleanup/reports/2026-11-24-285-u8-r2-review-closure-and-au8-retirement.md) 清理 `14` 项临时范围 / `48,469` 文件 / `2,741,223,009` 逻辑字节并原子重装 `/Applications/Fe.app` 后复验通过）**：
  - `build/terminal/port`（`45M`）：`src/emacs` `4,733,952 B`（SHA-256 `f1450fadfce4674d17bde284c51b1f0bcd313f48eb93dbd6067d973a67ec40c6`），`src/emacs.pdmp` `13,101,368 B`（SHA-256 `087e5609b9e8559321b6df296135a63c5866b57a720113fd957aa95831b5eac2`），`lib-src/emacsclient` `539,328 B`（`1baf54e6…`），`lib-src/hexl` `451,488 B`（`15f41ba9…`）。
  - `dist/macos/Femacs.app`（`235M`）及已安装 `/Applications/Fe.app`：主程序 `6,267,632 B`（SHA-256 `c1481fe564794624663c17b8cc9b9ff5dfbbd2e3509af8feb08d8fedcc89992a`），`Femacs.pdmp` `18,601,672 B`（SHA-256 `65d5e30c9b991dc98f128846da3c5ee2d4a848bf0d08e63264e8f6458557e3ac`）；工作区与已安装 bundle 均通过 `codesign --verify --deep --strict` `exit 0`、双产物 `-Q --batch` `(+ 1 2)` + `treesit` 冒烟 `(3 t t nil t)` 及 `femacs`/`fe`/`feclient` 验证。
- **关键实测对比摘要（`AU8-R2` 同机 vs 已安装参照 `32.0.50`）**：
  - 7 轮基准中位数（`--gate` 0 回退）：冷启动 `20.852 / 19.276 ms` vs `181.294 / 184.283 ms`（提速 `8.69x / 9.56x`），`bc-loop` `0.0041 vs 1.896–1.907 ms`，`bc-sum` `0.342–0.344 vs 4.274–4.344 ms`，`bc-car` `0.0041 vs 1.776–1.868 ms`，`sort-100k` `0.354–0.355 vs 1.985–1.995 ms`，`edit` `7.616–7.826 vs 9.543–9.634 ms`，`regex` `2.227–2.297 vs 2.746–2.762 ms`，`gc` `31.021–32.708 vs 50.465–51.851 ms`，`load` `1.603–1.750 vs 1.698–1.846 ms`。
  - 已交付对比参照字节码：`delivered-terminal-vs-reference-bytecode` 比值 `bytecode 0.1086`、`edit 1.1607`、`gc-churn 0.7059`、`gc-retained 0.6816`、`load 0.2796`、`regex 1.0460`；`delivered-cocoa-vs-reference-bytecode` 比值 `bytecode 0.1086`、`edit 1.1511`、`gc-churn 0.9130`、`gc-retained 0.8258`、`load 0.5885`、`regex 1.0819`。

## 历史记录（2026-11-23，已被上方 2026-11-24 R2 复验取代）

**本段保留为前一轮记录。**

- **去 `unsafe` 先行阶段（`U1–U8` / `AU1–AU8`）已完成实施与四组全量终验（`P-480`–`P-487`），待独立终审确认后进入 `C1–C6`**：
  - 活跃产品闭包（`436` 份文件）已收口为 `#![forbid(unsafe_code)]` **`143`** 份、`allow(unsafe_op_in_unsafe_fn)` **`0`** 份、`allow(static_mut_refs)` **`0`** 份、`static mut` **`0`** 项（较 `U1` 基线 `-2,602`）、拼接 `include!` **`0`** 项（较 `U1` `-2,232`）、`unsafe_blocks` **`6,243`**、`SAFETY:` 契约覆盖 **`6,243 / 6,243`（`100.0%`）**、`unsafe_fn_defs` **`4,337`**（较 `U1` `-884`）。
  - `U8` 在 `src/{alloc,eval}.rs`、`src/include/lisp.rs` 修复 `rtn15`（为 `sa_stack_block` 增加 `frame_depth`/`SA_FRAME_DEPTH`/`sa_stack_frame_enter` 清零嵌套退出帧自由块，并用 `[Qnil (); 8]`、`_cc_sa_guard` 与 `#[inline(never)]` 慢路径消除 `eval.rs` C 栈残留槽位），并经 `AU8` 四组正式验收全部通过：`clean-export`（`834.215s`，`7,719` 项纯跟踪重建与搬移通过）、`macos-ui`（`142.073s`，GUI/IME `456` 回调、多窗、Finder 拖放 `23/28` 与异常 TTY `16` 项全过）、`corpus-555`（555 入口 `8,685` 用例与 72 未完严格复跑 `781` 用例均为 `0` 阻断；`u8-native-normal`/`checking` 各 `11/11` / `603` 用例 `0` 阻断；写屏障 ERT 与 `runtime-execution.py` 负例/`young 120`/`major 44` 全过）、`performance`（`98.264s`，7 轮与四组对比零重要回退）以及全部 `12` 项三期保护门禁。
- **最终可交付制品（经 [CLN284](../maintenance/cleanup/reports/2026-11-23-284-u8-final-integration-and-au8-retirement.md) 清理 `45` 项临时范围 / `1,486,480,381` 逻辑字节后复验通过）**：
  - `build/terminal/port`（`45 MB`）：`src/emacs` `4,475,248 B`（SHA-256 `bf497bb5e380caf633325890127a757e3e6037a88d049a02ab19a922160a60cd`），`src/emacs.pdmp` `13,101,352 B`（SHA-256 `3e56fbf0bf454da5d83ebb348d9f2c6435fce62d48d7245a984c23c85985feab`），`lib-src/emacsclient` `564,976 B`（`383644de…`），`lib-src/hexl` `475,072 B`（`d44f1c44…`）。
  - `dist/macos/Femacs.app`（`235 MB`）：主程序 `5,998,256 B`（SHA-256 `e5e702bf00b3ecd08f0abe46ec2d2b70dde78340b26ca040eda54685d13863c2`），`Femacs.pdmp` `18,601,728 B`（SHA-256 `67a2ebadcc6709e799062833bff5c86be132f60b3fbb3f8394ddf21a2c4008d0`），`codesign --verify --deep --strict` `exit 0`（`spctl` 本地未公证 `exit 3` 边界），双产物 `-Q --batch` `(+ 1 2)`、`(treesit-available-p)` 与 `(require 'treesit)` 冒烟均返回 `(3 t t nil t)`、`exit 0`。
- **关键实测对比摘要（`AU8` 同机同模式 vs 已安装参照 `32.0.50`）**：
  - 7 轮基准中位数：冷启动 `23.918 / 24.259 ms` vs `146.635 / 150.667 ms`（提速 `6.13x / 6.21x`），`bc-loop` `0.0091 vs 1.885 ms`，`bc-sum` `0.330 vs 4.274–4.333 ms`，`bc-car` `0.0091 vs 1.776–1.876 ms`，`sort-100k` `0.412–0.413 vs 1.949 ms`，`edit` `7.886–8.075 vs 9.447–9.457 ms`，`regex` `2.188–2.399 vs 2.665–2.683 ms`，`gc` `33.706–35.761 vs 49.157–49.849 ms`，`load` `1.656–1.740 vs 1.700–1.716 ms`。
  - 已交付对比参照字节码：`delivered-terminal-vs-reference-bytecode` 比值为 `bytecode 0.116`、`edit 1.101`、`gc-churn 0.679`、`gc-retained 0.647`、`load 0.284`、`regex 0.955`；`delivered-cocoa-vs-reference-bytecode` 比值为 `bytecode 0.116`、`edit 1.109`、`gc-churn 0.946`、`gc-retained 0.830`、`load 0.625`、`regex 0.985`。
  - 既有现代化基线档案保留：`GA1–GA3` [Stage I 收口回执](../maintenance/releases/2026-11-20-stage1-ga1-ga2-ga3-closure.md)、`GA4`、`GA5–GA8` [Stage III 收口回执](../maintenance/releases/2026-11-20-stage3-ga5-ga6-ga7-ga8-closure.md) 与 [CLN273](../maintenance/cleanup/reports/2026-11-20-273-final-acceptance-runtime-and-cache-retirement.md)。

---

## 历史记录（2026-11-16）

**测试分层已定稿：** 功能验证用官方同一批样例、对本次交付的构建跑一次（分钟级），等价审计才用变体矩阵；有意偏离必须"登记 + 标记 + 自有测试"三件套。见 [testing-strategy.md](testing-strategy.md)。

**2026-11-17 教训已落档：** 555×48 变体矩阵被否，按"功能验证=官方量级、等价审计才用矩阵"分界；完整事实、实测成本与规则见 [incidents/2026-11-17-matrix-test-cost.md](../maintenance/incidents/2026-11-17-matrix-test-cost.md)。

**Goal 已按用户通过原生 Goal 接口的指令暂停（2026-11-16）。** 暂停后不新增开发、测试/批次、提交或自动接续；恢复入口与精确未完位置记在 continuation（`~/AI/docs/femacs/modernization-continuation.json` 的 `paused`/`g3_formal_acceptance`）。

- 唯一活动候选矩阵：`build/acceptance/current-runtime-build-6`，同源 61 profiles / 20 configurations 自然通过（源清单 `a72dd63a…`，控制器 exit0，11,685s，峰值 913,966,304B，零增长，回收完成）。冻结的 `current-runtime-build-5` 注册表未被改写；本批已为该 cohort 重新做参照 probe 与 `current-reference-selection-6`。
- GC 修复证据（受 young 路径门控）：新 cohort 上用正式 captured harness + 原 ERC 夹具重跑原失败的 `cocoa-collector-checking-major`/`-young`（另含 installed-reference）四个 ERC 场景文件，`12/12` 行 complete 且 passed，含此前 SIGABRT 的 `erc-scenarios-keep-place-indicator-trunc`（真实 Cocoa 显示，`window-system=ns`、`noninteractive=false`、exit0）。`build/acceptance/p466-erc-checking-6b/results.json`（最终驱动 `8d31e48c…`，SHA256 `3bbf4a34…`；同范围早先一次 `ce3adf7f…` 使用前一驱动修订）；该补丁位于 `MODE == 2`/`young_owner()` 分支，`checking-major` 不执行它，因此这里只确认“真实缺陷修复 + 新 cohort 指定回归通过”，历史 major 崩溃因果仍未证实，需由完整 checking 语义继续覆盖；原 abort raw 记录保留不变。
- 默认路径回归：单 profile 使用默认 12 文件列表、batch 显示，结构完整，`uses_default_files=true`、`declared_corpus_entrypoints=555`；`build/acceptance/p466-default-batch-6/results.json`（SHA256 `fa26753f…`）。batch 下的真实失败（如 `erc-fill-tests` 6 failed）仍是 raw 失败，未分类、未放行。
- eieio：`removed-upstream-producer` 分类规则已在源码闸门与运行期闸门实现，绑定废弃断言、缺失的生产者形式、上游提交 `4c6554413d3…`、batch 跳过与交互终端失败两种真实表现，以及同文件通过用例；源码展开 575 规则/27,600 定义/48 profile；分类集成 42/42（含 9 项新流程），`build/acceptance/p466-classification-it-1/results.json`（SHA256 `c84ea052…`）。
- 两轮驱动：每个 per-case 截止有界，最多两次精简重跑；阻塞用例保留 timeout，所有注册 ID 保留，合并文件恒为 `complete=false`/`passed=false`，未运行者显式 `not-run`。真实终端 `server-tests`：3/7 用例在同一阻塞点耗尽 300s 截止（`emacsclient/create-frame`、`server-force-stop/keeps-frames`、`server-start/stop-prompt-with-client`），4/7 通过；24 profile 受影响集合保持阻断，13 个有真实行、11 个显式未执行而非重烧同一挂点。索引 `build/acceptance/p466-terminal-driver-index-1/index.json`（SHA256 `c5194763…`）。工程控制取消的 `p466-terminal-twopass-6c` 已按实际 owner 状态释放 journal（`state=released`、`release_failures=[]`），partial 与 controller 记录保留，未重启。
- 工具链：SDK 可由 `build-support/toolchain-pins.json` 重建；本轮实际 dylib（`742a50cb…`）与各产品的 bundled 依赖身份见 `build/acceptance/p466-toolchain-binding-1/toolchain-binding.json`（SHA256 `d4119b48…`）。dylib 字节不作跨构建 pin。
- 清理：CLN269 退役被取代的单产品候选与两份规则检查输出（删除时 `du -sh` 977M/41M/41M），保留全部在用证据。

完整 48-profile current-version 覆盖、fresh 555/48 语料分类、G5/G6/G7、兼容/性能与最终 Goal 验收仍未完成，不因本段读数预支。


**已闭合清理记录（CLN255）：** 按用户明确指令，已删除整棵 `build/`（本次实测约18GiB）；该次清理终点目录物理缺席、仓库5.6GiB。126份已有WIP内容、HEAD与Git index在文档更新前核对未变，源码、参照检出、安装及 `dist/` 保留。最后一次恢复已退出1且完成回收，恢复子进程未启动；该次清理未再启动构建。原中断矩阵、18项候选检查点及两次失败恢复输出均已删除，不再作为当前输入或复用donor。后续必须按维护源与声明依赖新建输出，完整20/61、555/48、语义、性能及最终验收仍未完成。[清理报告](../maintenance/cleanup/reports/2026-11-14-255-explicit-build-cleanup.md)。

P478恢复入口代码仍为未完成真实集成验收的WIP。首次恢复因SDK元数据查询失败，第二次在准入时因原输出占用状态无法确认而失败，不把任何一次改标为成功。CLN253–254及下方活动路径描述均为历史记录，以本段当前物理状态为准。

P478占用诊断现保留超时/返回码/字节数与脱敏占用者摘要；真实空闲、占用、查询缺失3项集成通过（0.656秒），未编译或启动编辑器，build仍缺席。完整恢复、原语料和性能不因此通过。

P479已补上现有进程等待期间的父进程生命周期检查：失去原启动父进程后停止子树、释放捕获文件保护并退出125，不继续后续构建。真实文件/进程集成5/5、后置保护2/2通过；未编译或启动编辑器，不替代完整兼容/性能验收。[结果](../maintenance/releases/2026-11-14-runtime-owner-lifetime.md)。

**当前实际验收工作集：** 旧build/缓存删除后未恢复，含P479父生命周期修复的当前源码已新建唯一 `current-runtime-build-4` 冻结矩阵：7,717文件、源清单 `31c6077f...`，release构建器实际编译通过。现已自然构建通过20/20配置、61 profiles、41物理产品；40腿编译输入已canonical退场，共6,504,468,694逻辑字节。控制器退出0、回收完成，保护标志全部释放；完整运行验收仍未接受。该工作集只供剩余完整语义、555/48、性能及交付消费者；仅verified-files保全，无重复归档。CLN258已清掉消费后的语料获取及Cargo引导缓存等76,539,942逻辑字节，冻结源不变；独立构建器完成用途后已由CLN259退场3,635,536逻辑字节。宿主SDK只在该命令环境显式选择Xcode macOS27.0，未改全局配置或安装产品。

当前参照准入7项后置检查通过；当前受管Python harness的32个native profile各19项准备（608项）全部通过，产品、输入和保护释放核验完成。工具功能8项与实际占用查询3项、相关回归20项及关键保护24项通过；两次进程回收失败和4项保护夹具字段错误均保留失败，不升格。CLN260已立即删除消费后的245节点、4,074,354逻辑字节。完整语义阶段已启动（ert/negative/dump/relocation），沿用本批61产品及当前参照；完整语义、555/48、性能及交付均不预支接受。

单项DNS环境限制已获用户批准继续推进：本机代理必须保留，原 `non-existent-lookup-failure` 的失败状态与普通通过计数不改。2026-11-14经现有代理执行四次远端HTTPS解析，Google和Cloudflare均能解析 `example.com`，且均对 `emacs.invalid` 返回NXDOMAIN（Status3）；本机系统解析仍给出代理假地址。该对照只确认解析环境差异，不替代候选在正常系统DNS下的原ERT实跑。后续仅对该精确原用例绑定新观测单列；环境变化后重测，其他未解释网络差异仍须修复。Dot路线已取消，未发送任务。

以下CLN249–251和P476/P477为此前已闭合记录；其中build缺席读数描述各次清理终点。

**当前磁盘与执行状态（CLN249）：** 按用户本轮明确要求，已停止并回收在途49产品诊断（退出143，42项部分记录，不接受为完成），删除整棵 `build/` 及仓库外182个本任务测试容器。当前 `build/` 物理缺席，仓库5.6GiB；源码、原未提交实现、Git index、安装产品与 `dist/` 保留。此前矩阵/语义/首对语料结论仅为历史读数，已删原始路径不能继续核收或作为复用donor。后续真实运行需要按维护源和声明输入新建输出；本轮不立即重建。完整555/48、性能、工作流、安装及独立终审仍未完成。[清理报告](../maintenance/cleanup/reports/2026-11-14-249-explicit-build-cleanup.md)。

以下历次阶段记录中的保留路径与“当前矩阵”描述均以本段清理后的实际状态为准。

P476已关闭默认重复归档入口，并使远程诊断取消后及时回收、明确拒绝部分结果。真实文件/进程集成11/11和后置保护2/2通过，未运行编辑器矩阵；CLN250立即清除全部夹具输出和日志，`build/`仍缺席。[结果与范围](../maintenance/releases/2026-11-14-physical-file-retention.md)。

P477进一步修正mock shell在启动后恢复作业控制的初始化缺口：真实PTY2/2通过；显式单参照诊断完成，8项信号/3项监听生命周期正确，原信号即时断言与参照监听文件名失败仍保留。6项边界保护及3项相关回归通过，CLN251已即时清除全部消费输出和旧闭合日志，`build/`仍缺席；该诊断不进入矩阵分类。[结果与范围](../maintenance/releases/2026-11-14-owned-shell-initialization.md)。

CLN245进一步删除仓库外已消费旧日志、审阅包及诊断副本：1,218文件、86目录，412,750,738逻辑字节；私有review工作区由397MiB降至不足1MiB，仅保留当前输入/日志与两份最小待替代记录。活动矩阵不受影响。[清理报告](../maintenance/cleanup/reports/2026-11-14-245-consumed-diagnostic-workset.md)。

CLN246在完整语义自然闭合后立即删除消费完的测试dump和case临时输出：418文件、11,847目录，831,059,469逻辑字节。94项源码/WIP、Git index、16份正式动作元数据及12,135份case结果复算未变。当前矩阵产品及实际后续输入保留至消费结束。同一final cohort首对现已自然闭合：555文件/1,110文件侧完整、9,466相同注册ID，8,993普通通过/473非普通；结构错误0、61产品及七角色身份不变。原退出1保留，5项尚未覆盖结果待集中定位/控制，47候选、完整分类及独立终审仍待完成。[清理报告](../maintenance/cleanup/reports/2026-11-14-246-consumed-semantic-state.md)。

CLN247随即清退4处不可复用状态：139文件、81目录、4 FIFO，共5,202,873逻辑字节；12份原结果/owner/事件及96项源码/WIP未变，不补造闭合时证明。这4侧后续强制真实重跑。[清理记录](../maintenance/cleanup/reports/2026-11-14-247-consumed-nonreusable-case-state.md)。

P474当前工具与行为对照已验证：单一owner的两对并发、计划顺序和取消回收通过真实Python进程验收，6项关键保护通过；七角色源字节/mode仍与闭合seed一致。实际49产品dframe采样中48候选正确、参照仍有已知缺陷；双端各8项Tramp就绪后信号控制通过，但原正式5项未覆盖结果仍未分类，诊断重跑不回写原失败。CLN248即时清退两轮已消费失败输出及日志，共76,478,435逻辑字节。[结果与边界](../maintenance/releases/2026-11-14-runtime-control-dispatch.md)。

P473规范资源根修复已由无旧字节码的干净源码导出与release xtask重建验证：最小Cocoa native normal/checking产品通过，两候选四项真实缓冲区合同通过，无注入覆盖。完整原侧栏文件三端执行，候选四项普通通过，参照保留一项原失败；七项后置检查全真。整批工具功能9/9及关键保护17/17通过，568条源码定义展开为27,264条，尚未绑定正式全量运行。CLN244已立即清退消费后的整棵build和缓存，不以旧资源阶段失败冒充新构建结果，也不以局部成功冒充完整555/48、性能、安装或独立终审。

清理暴露的手工安装输入依赖已修齐维护入口：`runtime-reference-probe.py capture-inputs`从显式当前安装生成局部诊断描述，两读者共用完整身份重验；文件/进程集成8/8、保护26/26通过。实际参照进程已读取新采集输入，隐藏缓冲区缺陷仍被检出，不改标为正确行为。小型输出用完即清，无候选VM重建；正式矩阵仍须完整reference selection。

唯一施工与完成定义：[七项现代化计划](modernization.md)。用户已明确恢复后续施工与收尾；原 Goal 保持 active，仍按 GA1–GA8 验收，不能以本轮局部交付结束整个 Goal。

## 当前整批施工

2026-11-13用户明确恢复独立推进，停止联系搭档；本轮由主开发独立执行，不声称新的独立终审。当前固定矩阵和参照继续沿用。完整语义第二次运行因受管快照漏带XML/SQLite夹具而失败，负控608项、dump48项及搬移60项通过，61产品身份与回执未变。现按实际fixture组捕获完整依赖；修正接线的5项功能、分类链路19项和243条源码规则展开已集中通过（3/3组、9.198秒、峰值101,597,184字节）。第三次正式语义运行在9,509/10,335进程、进程失败0时发现外层资源控制器已缺席；按失监控中断处理，已回收原进程及逃逸动作子进程并通过正式owner入口释放冻结标志。该次未完成、不接受。第四次完整语义现已通过：10,335个发现/ERT进程、608负控、48 dump及60搬移产品全部通过，61产品身份与回执未变；控制器退出0、回收完成，7,832.026秒、峰值3,562,700,992字节。此前失败与中断不升格。

稳定分类先前444条仓库内定义、1,267段LF锚点及闭合控制自动绑定，保持原始状态与48候选范围；新增100条关闭native配置的正向编译检查及自举事件/闭合状态逐端门禁已完成工具功能验收：干净候选源码树的分类集成29/29、自举文件/CLI/读者集成5/5，444条定义展开为21,312条。原始555/48观测尚未绑定这些规则，所有未解释失败仍阻断。先前343条/702锚点的干净候选源码树中的新增分类集成24/24、参照采样入口集成7/7已通过；首次展开因缺13份忽略覆层失败，新增显式固定语料获取后343条定义展开为16,464条通过。未从旧诊断或原工作树复制缺失输入，正式安装采样已通过实际查询和7项后置检查；新增门禁的后置保护仍待集中验收。完整555/48、性能、安装及收尾均未接受。用完的40份编译账本镜像已canonical退场，41行辅助清单与原私有流对齐，主公开流和原账本未变。

按用户最新要求，一次写齐剩余业务与集成驱动，再冻结单一快照统一构建和集中验收；集中运行暴露的真实缺陷成批修复，只复验受影响集合。P466 字符串 interning 修复的标准双产品构建及三产品功能已通过：15 份闭合进程记录（6 次新执行、9 份未变控制绑定）；272 项日历注册分叉消除，当前版本原测试通过，3 项历史断言失败原样保留。保护代码已交回，尚未执行，不单独终审或提交该小单元。

当前集中编码包括：严格同执行器闭合文件复用、占用快照与两层有界清退、原生双自举显式预算、ERC/Tramp 无干预运行态记录、逐项分类 catalog 的真实事件绑定装配、同一参照 epoch 的新矩阵准入与最终证据汇总。完整 555/48、G5/G6/G7 实测及 GA8 仍待统一验收；ERC/Tramp 的未解释差异仍阻断。旧矩阵的接受范围不变。

统一工具功能首批 7/7 组通过（20.288 秒，峰值 118,784,240 字节），绑定代码未变；随后标准终端与 Cocoa 从空输出重建通过（527.150 秒，峰值 886,210,560 字节），源输入未变。三处先前标准输出及旧审查镜像现已在CLN233中退场，当前产品未变。以下为这次统一重建候选，不把构建通过当成完整运行、稳定语料或性能通过。

随后统一矩阵自然构建通过：20 配置、61 profiles、41 物理产品，源清单 `b6eaf19f...`，控制器退出 0、回收完成；实际编译的是当前未提交实现，Git 基线仍为 `63b1e531`。固定官方参照在同 epoch 重新采样绑定，61 产品回执、安装和图标配置未变。当前矩阵的完整 authored 语义已由第四次运行验证；555/48 稳定语料与性能仍待验，以下历史 `9e257276` 运行不能认证新 VM。

新矩阵集中诊断的 14 次原文件进程完整结束，8 项后置保护全真。修齐自有 fixture 的 Perl 资源、xref 产品库相对路径及 uniquify 目录/项目根：Perl 原14项、xref 原9项双端转为通过；uniquify 后续四次全文件均7/7（28/28）通过。候选原生编译文件完整执行186项，185通过、原双自举 `cmp` 失败保留，旧未运行的168项本轮全部执行通过。两端双自举的代码/数据节及其余字节相同，差异仅限库名、UUID和签名元数据；该证据待逐项分类审核。Tramp 原候选base64/PTY两项失败在本轮普通/观测运行未复现，双端其余失败和所有跳过仍保留，不预支完整稳定语料接受。

后续整批代码已补齐独立project Git fixture、受控ERC终端错误交付、真实短版本/显示元数据及按源码门禁核验的适用性控制。两轮helper源码切片已机械核收，旧远端监控两项运行分支仍未知。分类适用性工具16项功能集成已通过，真实显示/完整语料尚待验收。首次新矩阵authored语义的进程部分已自然闭合：10,335个发现/ERT进程正常通过，608负控、48 dump与60搬移产品全部通过，产品身份和61回执不变；但`cocoa-baseline-observer-normal`的`rtl08`在survival/all两模式出现弱项保守存活计数1而参照为0，原用例允许0或1，但汇总器整串比较失败。首次原阶段`passed=false`保留；仅该观察值的合法0/1结构契约已修正，并由第四次完整语义运行验证通过，不宣称已证明特定保守根或观察器归因。CLN229已发布新矩阵40份既有编译退场账本，新增删除0，独立终审待完成。

完整首对正式运行现已闭合：555入口、1,110文件侧全部完成，注册集合一致，共9,466对用例（8,990普通通过、476失败或跳过），61产品与冻结源不变，控制器5,118.274秒、峰值3,470,065,664字节、退出1并完整回收。原始阶段未接受。该次489条源码规则的干净工具集成30/30及23,472条展开通过；其中44项已绑定本轮真实原事件（含自举容器闭合证明），结构失败0，余下432对仍阻断，剩余47候选尚待执行。[首对原始状态和绑定边界](../maintenance/releases/2026-11-13-aligned-stable-seed.md)。

当前源码规则为567条（495条可选/适用性、12条无效或非有限自动化原入口、60条版本差异），展开27,216条。新增20条的干净候选工具集成31/31及关键保护30/30通过；此处合成描述符验收不代表真实原语料已分类接受。新增55项已绑定真实原事件及同产品当前版本控制：该子集原8,990/476不变、55分类/421阻断、结构失败0，六项后置检查全真；汇总脚本字段名错误及外层退出1保留，未升级为完整分类通过。控制与旧展开等已消费工作材料由CLN235清退525,515,112逻辑字节，14,470行三流对账通过。P471缩短自有mock路径后的原Tramp修复目标、库路径及严格文件复用集中功能通过，四项关键保护通过；种子2随后已闭合；P472进一步修正原ERC逐项终端隔离与既有worker约束传递，旧种子和控制均不满足变更后七角色门禁。

当前精确harness的新555文件种子已自然闭合：1,110文件侧完整、9,466对用例、8,988普通通过/478非普通，结构失败0，61产品及保全复验通过；原退出1和未接受状态保持。大型owner读取的协调器修正已通过三组集中功能及37项保护，不改变七角色或编译产品。11文件/55项目标当前版本控制已通过并由CLN239清退已消费工作集。P472五文件双端14项中12普通通过，nickbar双端原失败保持阻断；12个终端子进程实采JIT=true/jobs=1。分类工具32项集成和干净来源生成通过；当前版本ERC对照在途，新harness种子、适用显示控制及实际分类尚待完成。

## 已交付与本轮互审

- 已核收 G1、G2、G4：3/7。既有证据边界不变；G3、G5、G6、G7 及 GA8 最终收尾尚未完成。
- 已审查 `a404c000..d22daf1e` 八个签名提交的代码与验收范围。发现并修复简单词法闭包创建绕过 advice / 重定义 hook 的真实回归。
- P453 已签名推送 `87fe18eb`；保留 JNI 调用表和客户端回执版本保护已签名推送 `f6bef801`。独立搭档已完成两个阶段的签名、源码、制品身份与真实结果交叉核收；整体 Goal 余项不升格。
- 两种标准产品均从空输出重建，控制器退出 0、完整回收；新 Cocoa 包严格签名通过。构建明确使用独立固定的 Tree-sitter 0.27.1 SDK；本机 0.27.2 保留，版本门禁未放宽。
- 既有签名源 `59d4215c` 的全矩阵构建及负控准备已闭合：20 配置、61 profile、41 个物理产品、32 个 native 负控 profile，控制器均退出 0、保全验证及释放通过。完整稳定语料与性能验收尚未完成；[构建和准备记录](../maintenance/releases/2026-11-10-current-runtime-matrix-build.md)。
- 随后 61-profile authored 语义执行已完成：9,432 项 ERT、608 项负控、48 个 dump/恢复、124 项源不可读条件下的搬移功能检查全部通过。14 项副本清理因真实索引器占用失败，原回执保持失败；占用解除后独立绑定清退通过，原件不变。[语义与清理边界](../maintenance/releases/2026-11-10-current-runtime-semantics.md)。完整稳定语料与性能仍待验。
- 第一完整稳定对照完成 336/555 个原文件、5,596 观测后，控制器进程身份采样返回 EPERM，退出 125、完整回收；峰值 2,450,883,304 字节，未达到内存限额。原 259 项原始失败及未完成状态保留；独立保全释放后置检查确认全部 61 个产品与冻结源码未变。P458 已修复真实字节码间接调用帧回归：标准双产品从空输出重建，三产品功能与后置保护均通过；[修复与当前制品](../maintenance/releases/2026-11-10-bytecode-indirect-call-frames.md)。独立终审接受该修复单元，旧矩阵不认证新 VM。
- P458 与 CLN222 已签名推送 `1a8f68e4`。其余 20 条已闭合状态差异完成静态分类：9 条库/Unicode 版本演进、9 条特性或布局差异、2 条参照侧运行失败/超时；未证明新增 VM 缺陷，原失败不豁免，实际加载路径仍待验证。
- P459 补齐 primary/Full 回退调度、启动退出计时及内存身份、停顿/物理内存摘要和完整矩阵报告边界。工具集成及 86 项后置保护通过，独立终审接受，零确认缺陷；[工具证据与范围](../maintenance/releases/2026-11-10-runtime-measurement-contracts.md)。合成矩阵不代表编辑器性能通过。
- 当前签名源 `9e257276` 的新矩阵已自然构建成功并独立核收：20 配置、61 profile、41 物理产品、32 native profile；7,591 个冻结输入，单一源清单 `6eeeadb2...`。32 个 native 负控准备随后通过；完整 authored 语义阶段已自然完成并独立核收：10,335 ERT/发现进程、608 负控、48 dump 产品及 60 搬移产品全部通过，制品未变。该回执仍绑定历史 31.1 参照。
- 五产品独立库/能力诊断已自然成功并独立核收：确认参照 31.1 与候选 32.0.50 的 ERC/Unicode 差异，以及 native、图像和 Info 目录能力边界。该诊断使用产品默认上下文，不复现正式稳定文件上下文，不作兼容验收。
- P460 修复自有稳定测试桥接丢失物理产品库路径的问题：原 help-fns 完整 22 项 × 五产品共 110/110 通过，随后路径保护 30/30 通过；产品、原语料和冻结源码不变。CLN223 已清退本轮消费的复制输入；[修复证据](../maintenance/releases/2026-11-10-stable-source-context-paths.md)。独立终审通过，四项其他上游 unstable/FIXME 原失败不豁免。
- P464 显式绑定已登记产品 Lisp 根并捕获六文件 Python harness，修复官方参照布局引起的启动前阻断。四项文件/进程集成通过；五产品原 help-fns 共110/110、参照与 Cocoa native 原 module 共78/78 通过；27 项后置保护和五组回归通过。CLN226 已退场消费副本，全部61产品及冻结输入复验通过；[结果与边界](../maintenance/releases/2026-11-10-stable-product-libraries.md)。独立终审接受、零确认缺陷，完整555文件/48候选仍待运行。

[闭包修复与制品身份](../maintenance/releases/2026-11-10-interpreted-closure-hook.md) · [保留接口合同](../maintenance/releases/2026-11-10-retained-interface-contracts.md)

## 当前可试用制品与安装边界

| 对象 | 当前状态 |
| --- | --- |
| `build/terminal/port/src/emacs` | 统一重建候选（含整批 G3 源码修复）；SHA256 `dd0a18496566b18a26ddc8de44fa2a96c711e8efa5b831f9909a52cf3059886c` |
| 终端 `emacs.pdmp` | SHA256 `d028fe86c45fb10b2df2df845c5ddbde82aefbec64ce0abb39dffa1ffb7b81fb` |
| `dist/macos/Femacs.app` | 统一重建候选（含整批 G3 源码修复）；程序 `c7f49df0d89de7f015369e7049534b212ec85746c52aad251953d5dfdce07358` |
| Cocoa `Femacs.pdmp` | SHA256 `39ddf767ba56821168ba3dea6641e2bba32f7e9ca3f5e454471eb08590ecc33d` |
| P458历史镜像（已退场） | CLN233已清退消费副本；历史SHA256 `d0293c6f4cff5e6a79a3197b622512812eca67f7db9d05ce12da074edbbd2fbf` |
| 已安装 Fe | 旧 App 仍在使用，已请用户保存退出后替换；尚不宣称安装已更新 |

本地包为 ad-hoc 签名，未公证。P458历史镜像的身份仅保留在记录中，副本已退场，不认证当前P466候选；当前候选尚待整批验收和最终打包。完整运行态与签名证据不替代物理 GUI/IME 或全语料验收。

## 当前验收读数

- P458：7 项自有真实调用帧用例 × 3 产品全部通过；原 nadvice 全文件三端均 11 普通通过、2 原预期失败、零 unexpected。随后 10 项热路径保护与 10 项 bytecode 保护 × 3 产品共 60 项通过，9 项本地检查和 6 项新终端 TTY 恢复通过。以下 P453 既有结果保留其原制品范围。
- 6 条闭包功能路径 × 3 产品，18 次观测一致：advice / 替换 hook 各调用 1 次，正常捕获、局部声明、错误传播和禁用 hook 语义保持参照行为。
- 8 个完整稳定 31.1 原文件 × 3 产品，共 24 次完成运行、1,248 观测：1,224 普通通过、15 原预期失败、9 原跳过，零 unexpected，状态映射一致。不把预期失败/跳过计作普通通过。
- 后置热路径保护 8/8、原 bytecode 保护 10/10，各在三产品通过，共 54 项。
- 本地验收、16 例代表性兼容 × 3 产品（48 观测）、稳定夹具/运行阶段/测量报告/保留合同、9 项真实 OS 保全、7 项客户端回执保护，以及 own-voice / declaration 门通过。
- 当前标准终端的 6 项真实 TTY 异常/恢复路径刷新通过，制品前后身份不变；不替代 GUI/IME 或 61-profile 运行时语义验收。
- 首次自有夹具把局部声明误期望为 22，三产品共同否定；正确参照值为 12。一次误用开发版 cl-macs 文件使 31.1 参照触及 4 GiB 硬限，旧 aggregate 保持中断/137；单独采用 manifest 固定的稳定原文件后，三端 77 项均完成零 unexpected。漏传两个工具的必选输出参数产生的 exit 2 同样保留，仅修正相应调用。
- 当前读数仅关闭对应修复和有限文件范围；`stable-compat.py` 不是完整 555 文件覆盖，转来的“343”也不是全兼容验收。

## 尚需完成的工作

主体设计与实现已经完成（GC6、GC7已勾选），已核收的G1/G2/G4和各修复、工具单元继续关闭。本轮只剩三个收尾批次：完整兼容范围收口；共用的一批性能/native/GC实测与采纳裁定；最终安装、必要受影响检查、复核、签名推送和清理。不能将七项目标的最终勾选数量换算为剩余开发工作量百分比；先前“剩57%”表述作废。已核收且未受影响的结果直接复用，不新增或重开已关闭单元。

| 验收项 | 必需的剩余工作 |
| --- | --- |
| GA3 / G3 | 完整 555 原文件、48 候选对照；P463 工具集成、关键保护和独立终审已通过，待实际完整运行及逐项 catalog 核验；原状态保留，必需功能/端口回归/未解释差异仍必须修复 |
| GA5 / G5 | 安静主机、完整模式/轮次的分配、存活、phase 22/23/24、停顿与进程内存证据 |
| GA6 / G6 | 实现已完成，当前31c6077f源20配置/61-profile构建、32-native准备均通过；本轮10,335项发现/ERT进程已执行且进程失败0，正在状态清退，完整语义阶段尚未闭合。随后与GA5共用性能实测，作实际采纳/拒绝裁定 |
| GA7 / G7 | 实现已完成，与GA3/GA5/GA6共用当前产品的正确性、稳定语料和性能数据，完成实际采纳/拒绝裁定；不另开一轮开发或重复已通过的检查 |
| GA8 / 收尾 | 汇总收益与稳定性结论、当前安装及必要受影响检查、最终复核、干净工作树、签名推送和清理；已核收且源码等价的GUI/TTY等证据直接复用 |

历史9e257276矩阵的 authored 语义已独立核收；历史b6eaf19f矩阵的完整语义由主开发验证通过，均保持已核收范围，不作废其既有结论。当前31c6077f产品的受影响检查和尚未完成的全量范围分开记录，不用旧源码结论冒充新运行；新的独立终审未执行。完整稳定语料和安静主机性能仍待验，components只补本轮实际受影响项。P461 已签名推送 63e729da，显式 runner 接线的功能/后置保护与独立终审通过；保持编译源 9e257276、冻结 writer 和制品身份。P463 分类工具、集成和保护均通过，独立终审接受、零确认缺陷；实际完整语料和逐项证据仍待验。按用户要求，官方参照已安装为 32.0.50，安装回执固定上游 fd7afced4036fc1a221567d70705d20a2bd64713；现有 modern-sexy-v2 图标配置保留，应用/客户端签名与实际功能查询通过。运行时 Git 变量为 null，按实记录，源码提交由固定配方和实际安装回执证明。P462 新参照选择已通过五项文件/子进程集成、23 项后置保护和真实 61-profile 制品准入，独立终审接受、零确认缺陷；新旧回执分开绑定，不预支完整稳定语料/性能验收。App 安装单独等待用户保存退出；不重启已失效的旧冻结源恢复事务。

## 清理与历史

CLN238已结束被新harness替代的旧首对用途，清退旧输出及191保留容器，共31,210节点、700,630,273逻辑字节，26保护身份未变；历史8,990/476及未接受状态保留，当前种子没有引用选区。[报告](../maintenance/cleanup/reports/2026-11-13-238-replaced-first-stable-seed.md)。

CLN236已清退当前前置门禁工具的16个消费范围：4,139节点、140,849,418逻辑字节，21项保护身份未变；当前564条源码定义、正式展开及最小功能结果供实际装配使用。干净源码和获取副本已退场。[报告](../maintenance/cleanup/reports/2026-11-13-236-consumed-prerequisite-work.md)。

CLN234已清退本批工具源码/获取工作副本、旧定义与功能输出及绑定中间产物：1,959文件、313目录，67,316,734逻辑字节，2,272条公开/私有流完整对账；13项保护文件身份未变。当前原始事件和新定义保留至消费者结束。[报告](../maintenance/cleanup/reports/2026-11-13-234-consumed-source-gate-work.md)。

CLN233已清退失败旧语义/弱引用诊断、干净源码与获取工作集、旧定义和标准构建/App备份等消费材料：143,786文件、28,900目录、14链接，4,608,005,859逻辑字节，172,700公开/私有行对齐；13项受保护文件身份未变，当前矩阵、SDK与新分类定义仍供后续验收使用。[报告](../maintenance/cleanup/reports/2026-11-13-233-consumed-unified-working-set.md)。

CLN219 已清退 13,460 普通文件、13 链接、500 目录：684,553,921 逻辑 / 715,595,776 先前逐路径分配字节。13,973 项公开清单与私有身份/操作逐条一致，选中项全不存在，当前产品、新包、源及安装中 App 不变；不作整卷因果回收声明。

CLN220 记录本轮构建中已自动清退的编译副本：195,460 文件、6,800 目录、6,503,836,334 逻辑字节；202,260 项公开投影绑定全部 40 份 canonical 账本。发布记录不产生新删除，冻结输入、产品及原运行观测保留。

CLN221 记录已消费语义状态、60 份搬移副本和空父目录：310,964 普通文件、30 链接、74,602 目录，17,254,112,173 逻辑字节；385,596 项公开清单完整对账。当前源、制品及原始观测保留。

CLN222 已清退 P453 三处旧构建/App 备份：9,760 文件、13 链接、373 目录，548,866,166 逻辑字节；10,146 项公开清单与实际操作逐一一致，当前产品与两份已审镜像不变。

CLN223 已清退 P460 消费后的三处原语料/上下文复制目录：1,838 文件、30 链接、266 目录，48,913,383 逻辑字节；2,134 项公开/私有清单与操作逐一一致，60 项受保护源/产品/证据不变。

CLN224 已发布新 9e257276 矩阵在构建中自动清退的 40 份编译复制输入账本：195,460 文件、6,800 目录、6,503,533,814 逻辑字节，202,260 行独立流式对账一致；随后清退 41 份临时元数据副本及 2 个自有目录，25,227,474 逻辑字节。原源码、产品、观测、原账本和公开清单保留不变，不作整卷因果回收声明。

CLN225 已清退本次官方参照构建的消费缓存与临时状态：5,664 文件、3 链接、446 目录，1,909,455,588 逻辑字节；6,113 项公开身份记录与私有操作/缺席核验一致，安装/配置/源及原失败不变，独立终审接受。

CLN226 已清退四处生成语料/上下文复制根：3,374 文件、30 链接、517 目录，69,910,355 逻辑字节；3,921 行与实际操作逐一一致。另清退319,929字节的冗余发布副本与一个自有父目录，独立两行记账；210项受保护路径及全部61产品复验不变。

当前构建制品和固定 SDK 仍有下一轮消费者，保留到用途结束。历史 App 备份已被当前制品替代；不再按目录名永久保留。

[CLN219](../maintenance/cleanup/reports/2026-11-10-219-superseded-build-app-backups.md) · [CLN220](../maintenance/cleanup/reports/2026-11-10-220-consumed-runtime-compiler-copies.md) · [历史检查点](../maintenance/history/2026-11-10-current-state-checkpoints.md) · [手工进度账](status.md)

CLN239已清退被P472替代的版本控制工作集：2,536节点、105,823,913逻辑字节，19项保护和三流缺席核验通过。见[报告](../maintenance/cleanup/reports/2026-11-13-239-consumed-version-controls.md)。

## 本轮即时清退（CLN241）

旧第二种子、5组ERC对照、3组窗口诊断、207棵闭合状态和4处已用完编译缓存已退场；用完的审计工作副本随即回收。共61,184节点、2,106,253,796逻辑字节，主操作86项保护身份未变。[报告](../maintenance/cleanup/reports/2026-11-13-241-consumed-stable-and-diagnostic-workset.md)。当前61制品、源码、SDK及WIP保留。后续只消费当前维护源与新结果，旧路径不可作为输入。

P473正确行为合同及撤销流程的前驱8组功能/15项保护已有结果；后继资源路径修复和普通通过分支合同门禁已通过6项生命周期及27次生成器调用，新整批功能/保护和完整产品、原文件验收待续。官方恢复正确行为时撤销相应规则；候选保持同错不能通过。
