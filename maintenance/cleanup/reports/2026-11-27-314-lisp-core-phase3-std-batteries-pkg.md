# CLN314 — Standalone `lisp-core` Phase 3: 100% Pure-Rust `crates/lisp-std` Batteries & `init`/`pkg` Workflow (2026-11-27)

## Summary

Completed Phase 3 of the standalone general-purpose Lisp runtime roadmap across `crates/lisp-std` and `crates/lisp-core`: created the 100% pure-Rust (`#![forbid(unsafe_code)]`, zero `cc` or C `-sys` dependencies) `crates/lisp-std` crate, wired `std-batteries` into `Engine` and `NoGil` worker isolates while keeping editor builds isolated (`default-features = false`), added 19 `stdlib/std/*.el` library modules plus `core-prelude.el` autoloads, implemented `lisp-core init` and `lisp-core pkg` (`lisp.toml`, atomic `lisp.lock`, `.lisp-packages/`), verified unit, integration, `fe-test`, `cargo package`, and all 6 static boundary gates cleanly, and retired temporary build/packaging/test scopes (`20,582` files, `4,041` dirs, `2` symlinks, `7,939,996,649` logical bytes).

## Delivered & Verified Evidence

- **Pure-Rust `crates/lisp-std` (`v0.0.1`, all `#![forbid(unsafe_code)]`, `0` C/`-sys` deps)**:
  - `src/{value,lib}.rs`: `StdValue`, JSON conversions, shared `StdRuntime` with clean embedded-server `Drop` teardown, `register_all`, and pure-Rust TLS provider installation (`rustls-rustcrypto`).
  - `src/http.rs` (`std/http`): HTTPS/HTTP client (`ureq` + `rustls` + `rustls-rustcrypto` + `webpki-roots` + `gzip`) and embedded multi-threaded HTTP/1.1 `TcpListener` server (`std-http-server-*`, `std-http-serve`).
  - `src/db.rs` (`std/kv`, `std/db`): `redb` embedded ACID KV store and pure-Rust transactional SQL engine (`CREATE/DROP TABLE`, `INSERT`, `SELECT` + `WHERE`/`ORDER BY`/`LIMIT`/`OFFSET`/`COUNT(*)`, `UPDATE`, `DELETE`, `BEGIN`/`COMMIT`/`ROLLBACK`, `:memory:` and file DBs).
  - `src/formats.rs` (`std/{json,toml,csv,yaml,url,base64}`), `src/archive.rs` (`std/archive` gzip/zlib/tar/tar.gz/zip with path-traversal and symlink guard), `src/crypto.rs` (`std/crypto` `getrandom`/SHA-256/512/HMAC-SHA256/UUID v4&v7/hex/constant-time-eq), `src/time.rs` (`std/time` `jiff` UTC/RFC-3339/timezones/monotonic), `src/fs.rs` (`std/{fs,path}` `walkdir`/`globset`/bytes/tempdir/paths), and `src/system.rs` (`std/{regex,proc,env}` `regex`, timeout-bounded subprocesses, `.env`).
- **Engine Integration, `stdlib/std/*`, and `init`/`pkg` (`crates/lisp-core`)**:
  - `crates/lisp-core/Cargo.toml` enables `std-batteries = ["vm", "dep:lisp-std"]` by default; `femacs-host`, `femacs-tools`, and `xtask` keep `default-features = false` (editor isolation verified).
  - `crates/lisp-core/src/engine.rs` attaches shared `StdRuntime` to both root `Engine` and `ConcurrencyHub` `NoGil` workers.
  - Added 19 `crates/lisp-core/stdlib/std/*.el` modules (incl. `std/cli` and `std/log`) and autoload entries in `stdlib/core-prelude.el`.
  - Added `crates/lisp-core/src/pkg.rs` (`#![forbid(unsafe_code)]`) and CLI subcommands `init`, `pkg <add|install|list|update|remove>`, zero-arg project `run`/`test`, and automatic `src/` + `.lisp-packages/*/src` `load-path` discovery.
- **Test Suites, Release Binary & Packaging Verification**:
  - `cargo test -p lisp-std`: `8/8` unit tests (`0` warnings).
  - `cargo test -p lisp-core` (`run_id=a1a16407fa684a898f5ba888229c63e2` -> full run `414d759551384be8bd410ddedd6b945e`): `27/27` unit tests (`0` warnings), `13/13` `engine_integration` cases, and all `5` suites in `std_batteries_integration` (`http_client_and_server`, `kv_and_sql_persistence`, `formats_archive_crypto_time_fs_regex_proc_cli_log`, `nogil_parallel_batteries`, and `init & pkg CLI workflow` incl. `--path` + localhost `--url`).
  - Workspace & feature builds (`run_id=414d759551384be8bd410ddedd6b945e` & `5c114813b96247299c3f28077fa74ad7`): `cargo check -p lisp-core --no-default-features --features vm` (`0` warnings), `cargo check --workspace`, `cargo check -p femacs-host --features tty_terminfo`, `cargo build --release -p lisp-core` (`otool -L` links only `/usr/lib/libiconv.2.dylib` & `/usr/lib/libSystem.B.dylib`; `--version` prints `lisp-core 0.0.1`), `lisp-vm --lib` (`12/12`), `lisp-vm --features isolate_vm --lib` (`16/16`), `xtask --bin xtask` (`119/119`), `http-host` (`9/9`).
  - `fe-test --suite self`: `9/9` (`6.659s`); `fe-test --suite all --no-cache`: `39/39` (`wall=12.463s`, `caseSum=25.636s`, `concurrency=2.06x`, `peakMem=219.00 MiB`, `run_id=5c114813b96247299c3f28077fa74ad7`).
  - `cargo package -p lisp-vm -p lisp-core -p lisp-std --allow-dirty --no-verify`: `lisp-vm-0.0.1.crate` (`4,024,140` B, `398` entries, `0` escapes), `lisp-core-0.0.1.crate` (`1,148,117` B, `131` entries, `0` escapes), `lisp-std-0.0.1.crate` (`45,091` B, `15` entries, `0` escapes).

## Controlled Post-Cleanup Verification

- Cleaned `target/`, `build/*`, and `tools/__pycache__`; `build/` is empty, `target/` and `tools/__pycache__` are absent; `dist/macos/Femacs.app` and `dist/lisp` remain intact with empty `diff -rq dist/macos/Femacs.app /Applications/Fe.app`.
- All `6` static gates + `git diff --check` passed post-cleanup (`run_id=9fc3c846e3d240fab10db603ab8580ba`, `23.030s`, `exit=0`, `cleanup_complete=true`): `tools/check-own-voice.py` (`0` failures across `334` ported + `375` own files), `tools/test-repository-bootstrap.py` (`4/4`), `tools/test-tooling-modernization.py` (`33/33`), `tools/check-ledger-ticks.py` (`58` bound / `0` failures), `tools/check-unsafe-boundary.py` (`active_product`: `471` files, `175` `forbid_unsafe_files`, `6,423/6,423` `SAFETY:` blocks, `0` `static_mut`, `0` errors), and `tools/check-crate-boundaries.py` (`ok`, `standalone=4`, editor isolation verified).

## Retired Temporary Scopes

Retired `3` owned temporary build, packaging, and test-framework scopes totaling `20,582` files, `4,041` dirs, `2` symlinks, and `7,939,996,649` logical bytes (`run_id=9fc3c846e3d240fab10db603ab8580ba`, `23.030s`):

| Removed scope | Files | Dirs | Symlinks | Logical bytes | Normalized digest | Replacement / Recovery |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| `target/package` | `13` | `5` | `0` | `15,661,336` | `4700af91d942521f06ac18bbf70bbe53227fc705b3b750bf17972b2dd43a0dd7` | Recreated via `cargo package -p lisp-vm -p lisp-core -p lisp-std --allow-dirty --no-verify` |
| `target` | `20,544` | `3,934` | `0` | `7,924,316,864` | `abf00f88bb32bb6c7c2241c2e311dc730d71f3e749c2185b203a19bf3ece6fda` | Rebuild via `cargo build --release -p lisp-core` |
| `build/*` + `tools/__pycache__` | `25` | `102` | `2` | `18,449` | `8f4ec49109baa3b5bfd1bcad403b920fdbadea65c34df77a3f81dea95e8faf32` | Recreated on demand by `fe-test` |
