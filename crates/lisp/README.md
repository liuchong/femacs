# lisp

Pure-Rust, No-GIL, Elisp-compatible embeddable Lisp runtime and standard library (`0.0.1`).

> Looking for the standalone CLI and interactive REPL (`cargo install elisp`)? See [`elisp`](../elisp/README.md).
> Architecture overview: `lisp` combines the low-level VM ([`lisp-vm`](../lisp-vm/README.md)), bundled core standard library, No-GIL multi-core runtime, and pure-Rust standard library batteries ([`lisp-std`](../lisp-std/README.md)) behind a `#![forbid(unsafe_code)]` Rust API.

## Highlights

- **Pure-Rust Build Graph (Zero C Compilers Needed)**: Compiles without `cc`, `cmake`, `pkg-config`, `bindgen`, OpenSSL, or C SQLite. Release binaries link solely against standard OS runtime libraries.
- **No-GIL Multi-Core Concurrency (`NoGil` Default)**: Each `Engine` owns an isolated VM heap/globals context (`lisp-vm/isolate_vm`), making `Engine: Send` across OS threads. Lisp programs get `std/task`, `std/chan`, and `std/sync`, or can select `ConcurrencyMode::Gil` (`LISP_CONCURRENCY=gil`) for legacy single-isolate semantics.
- **Full Pure-Rust Standard Library (`std`)**: Default builds bundle core Lisp modules plus `22` `std/*` modules (HTTPS/HTTP client & server, `redb` ACID KV + transactional SQL engine, JSON/TOML/CSV/YAML/Base64/URL, tar/zip/gzip/zlib, CSPRNG/SHA/HMAC/UUID, `jiff` time, walkdir/glob FS, regex, bounded proc, CLI, and structured logging).
- **Silent Cold Boot & Atomic `.pdmp` Warm Cache**: Silent stderr by default (or enable `EngineConfig::with_verbose(true)` / `LISP_VERBOSE=1`) with content-fingerprinted cache under `LISP_CACHE_DIR` / platform cache dirs (`~/.cache/lisp/...`).
- **Host Extensions & Zero-Copy Byte Buffers**: Register safe Rust closures via `Engine::register_fn` and pass binary payloads through `Value::Bytes(Vec<u8>)`.

## Feature Flags (Clean Layering)

| Configuration | Included Capabilities | Best For |
| --- | --- | --- |
| `default = ["vm", "std"]` (`cargo add lisp`) | `Engine`/`Value`/`Error`, `prelude`, bundled stdlib, No-GIL VM (`vm`), and all pure-Rust `std/*` batteries (`stdlib`) | General Rust applications embedding full-featured programmable Lisp |
| `default-features = false, features = ["vm"]` | `Engine`/`Value`/`Error`, bundled core stdlib, and `std/{task,chan,sync}` without HTTP/TLS/`redb`/archive dependencies | Lightweight or network/DB-free sandboxes and rule engines |
| `features = ["pkg"]` | Adds `lisp.toml` / atomic `lisp.lock` / `.lisp-packages/` dependency management (`lisp::pkg`) | Embedders driving project-scoped package resolution |
| `default-features = false` | Zero-VM `Value`, `Error`, `exec` process supervisor, cross-platform JSON-RPC `rpc`, and cache path utilities | Build/test tooling (`xtask`) sharing process supervision without linking the VM |

`lisp` re-exports `lisp::prelude::*`, `lisp::vm` (`lisp_vm`), and `#[cfg(feature = "std")] lisp::stdlib` (`lisp_std`).

## Rust Embedding Quickstart

```toml
[dependencies]
lisp = "0.0.1"
```

```rust
use lisp::prelude::*;

fn main() -> Result<(), Error> {
    let mut engine = Engine::init(
        EngineConfig::cold_stdlib()
            .with_concurrency(ConcurrencyMode::NoGil)
            .with_verbose(false),
    )?;

    engine.register_fn("rust-bytes-len", |args| {
        let bytes = args
            .first()
            .and_then(|v| v.as_bytes())
            .ok_or_else(|| "expected string or bytes".to_string())?;
        Ok(Value::Int(bytes.len() as i64))
    })?;

    let len = engine.eval(r#"(internal-host-call "rust-bytes-len" "hello")"#)?;
    assert_eq!(len.as_int(), Some(5));

    let json = engine.eval(
        r#"(progn (require 'std/json) (std-json-stringify (list (cons "ok" t))))"#,
    )?;
    assert_eq!(json.as_str(), Some("[[\"ok\",true]]"));
    Ok(())
}
```

Key public types in `lisp`:
- `Engine` (`new`, `minimal`, `init`, `eval`, `eval_printed`, `eval_silent`, `call`, `register_fn`, `complete_symbols`, `run_script`, `compile_files`, `run_ert_files`, `dump_image`, `gc`, `booted_from_dump`, `concurrency`)
- `EngineConfig` (`minimal`, `cold_stdlib`, `with_program_name`, `with_cache_dir`, `with_load_path`, `with_bootstrap`, `with_quiet_bootstrap`, `with_verbose`, `with_concurrency`)
- `Value` (`Nil`, `Bool`, `Int`, `Float`, `Symbol`, `String`, `Bytes`, `List`, `Vector`, `Map`, `Opaque`), `Error`, `BootstrapMode`, `ConcurrencyMode`

## Environment Variables

- `LISP_CACHE_DIR`: Override stdlib / `.pdmp` cache root directory
- `LISP_CONCURRENCY`: `no-gil` (default) or `gil`
- `LISP_VERBOSE`: Set to `1` to print bootstrap load diagnostics
- `LISP_NO_AUTO_WARM`: Set to `1` to disable automatic background `.pdmp` warming

## Platform Support Matrix

| Target | Architecture | Status | Verification Scope |
| --- | --- | --- | --- |
| macOS (`aarch64-apple-darwin`) | Real host | Verified | Full `cargo test` (`--all-features`, `--no-default-features --features vm`, `--no-default-features`), `engine_integration`, `std_batteries_integration`, `isolate_vm`, release `otool -L`, `cargo package` isolated rebuild |
| Linux (`aarch64-unknown-linux-gnu`) | Ephemeral Debian 12 VM | Verified | Unit, integration, release `ldd` (glibc + OS libs only), `.pdmp` warm cache, `NoGil` 4-worker speedup |
| Linux (`x86_64-unknown-linux-gnu`) | Cross-compile | Verified | Clean `cargo check` |
| Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) | Cross-compile ONLY | Architecture Verified (No Windows Host Runtime) | Pure-Rust `windows-sys` Job Objects, Toolhelp32 process snapshots, memory accounting, `RpcTransport` TCP loopback, and `LOCALAPPDATA` cache root pass `cargo check` across all targets with 0 warnings; **no physical Windows host runtime verification has been performed** |

## License

Licensed under `GPL-3.0-or-later`. See [LICENSE](LICENSE).
