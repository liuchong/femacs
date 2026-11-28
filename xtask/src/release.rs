//! Unified 5-crate programmatic release workflow (`lisp-vm`, `lisp-std`, `femacs`, `lisp`, `elisp`).
#![forbid(unsafe_code)]

use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[cfg(test)]
#[path = "release_tests.rs"]
mod tests;

/// Publishable workspace crate metadata in topological dependency order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PublishCrate {
    pub name: &'static str,
    pub manifest: &'static str,
    pub readme: &'static str,
    pub license: &'static str,
    pub role: &'static str,
    pub has_docs_rs: bool,
    pub internal_deps: &'static [&'static str],
}

/// Topological 3-tier publish order:
/// Tier 1: `lisp-vm`, `lisp-std`, `femacs` (no workspace crate dependencies)
/// Tier 2: `lisp` (depends on `lisp-vm` and `lisp-std`)
/// Tier 3: `elisp` (depends on `lisp`)
pub(crate) const PUBLISH_CRATES: &[PublishCrate] = &[
    PublishCrate {
        name: "lisp-vm",
        manifest: "crates/lisp-vm/Cargo.toml",
        readme: "crates/lisp-vm/README.md",
        license: "crates/lisp-vm/LICENSE",
        role: "Standalone Elisp-compatible VM, GC, bytecode engine & opt-in isolate_vm",
        has_docs_rs: true,
        internal_deps: &[],
    },
    PublishCrate {
        name: "lisp-std",
        manifest: "crates/lisp-std/Cargo.toml",
        readme: "crates/lisp-std/README.md",
        license: "crates/lisp-std/LICENSE",
        role: "100% pure-Rust (#![forbid(unsafe_code)]) standard library batteries",
        has_docs_rs: true,
        internal_deps: &[],
    },
    PublishCrate {
        name: "femacs",
        manifest: "cli/Cargo.toml",
        readme: "cli/README.md",
        license: "cli/LICENSE",
        role: "macOS editor command-line launchers (`femacs` and `fe`)",
        has_docs_rs: false,
        internal_deps: &[],
    },
    PublishCrate {
        name: "lisp",
        manifest: "crates/lisp/Cargo.toml",
        readme: "crates/lisp/README.md",
        license: "crates/lisp/LICENSE",
        role: "Pure-Rust No-GIL embeddable Lisp runtime & standard library API",
        has_docs_rs: true,
        internal_deps: &["lisp-vm", "lisp-std"],
    },
    PublishCrate {
        name: "elisp",
        manifest: "crates/elisp/Cargo.toml",
        readme: "crates/elisp/README.md",
        license: "crates/elisp/LICENSE",
        role: "Standalone general-purpose CLI, script/package manager & TTY REPL",
        has_docs_rs: false,
        internal_deps: &["lisp"],
    },
];

const EXPECTED_HOMEPAGE: &str = "https://femacs.com/";
const EXPECTED_REPOSITORY: &str = "https://github.com/liuchong/femacs";
const EXPECTED_README: &str = "README.md";
const MAX_CRATE_TARBALL_BYTES: u64 = 10 * 1024 * 1024;
const TMP_REL_DIR: &str = "build/release-tmp";

/// Validate strict `MAJOR.MINOR.PATCH` (with optional `-prerelease`) SemVer string.
pub(crate) fn is_valid_semver(version: &str) -> bool {
    let (core, pre) = match version.split_once('-') {
        Some((c, p)) => {
            if p.is_empty()
                || !p
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
            {
                return false;
            }
            (c, Some(p))
        }
        None => (version, None),
    };
    let _ = pre;
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

/// Compute the crates.io sparse index URL for a crate name.
pub(crate) fn sparse_index_url(crate_name: &str) -> String {
    let lower = crate_name.to_ascii_lowercase();
    let len = lower.len();
    match len {
        1 => format!("https://index.crates.io/1/{lower}"),
        2 => format!("https://index.crates.io/2/{lower}"),
        3 => {
            let first = &lower[0..1];
            format!("https://index.crates.io/3/{first}/{lower}")
        }
        _ => {
            let a = &lower[0..2];
            let b = &lower[2..4];
            format!("https://index.crates.io/{a}/{b}/{lower}")
        }
    }
}

/// Extract a string field value from the `[package]` table of a `Cargo.toml`.
pub(crate) fn read_package_field(text: &str, field: &str) -> Option<String> {
    let mut in_package = false;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package || line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, rest)) = line.split_once('=') {
            if k.trim() == field {
                let val = rest.trim();
                if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
                    return Some(val[1..val.len() - 1].to_string());
                }
            }
        }
    }
    None
}

/// Extract the pinned version from an inline table sibling dependency line `<dep> = { ... }`.
pub(crate) fn read_dep_version(text: &str, dep_name: &str) -> Option<String> {
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if let Some((lhs, rhs)) = line.split_once('=') {
            if lhs.trim() == dep_name {
                let rhs = rhs.trim();
                if rhs.starts_with('{') && rhs.contains("path =") {
                    if let Some(idx) = rhs.find("version = \"") {
                        let after = &rhs[idx + "version = \"".len()..];
                        if let Some(end) = after.find('"') {
                            return Some(after[..end].to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Update `[package] version = "..."` and workspace sibling inline `version = "..."` pins.
pub(crate) fn bump_manifest_text(
    text: &str,
    new_version: &str,
    sibling_deps: &[&str],
) -> Result<String, String> {
    let mut out = Vec::new();
    let mut in_package = false;
    let mut bumped_pkg = false;
    let mut bumped_deps: BTreeSet<&str> = BTreeSet::new();

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            out.push(raw_line.to_string());
            continue;
        }
        if in_package && !bumped_pkg && !trimmed.starts_with('#') {
            if let Some((k, _)) = trimmed.split_once('=') {
                if k.trim() == "version" {
                    out.push(format!("version = \"{new_version}\""));
                    bumped_pkg = true;
                    continue;
                }
            }
        }
        if let Some((lhs, rhs)) = raw_line.split_once('=') {
            let dep_key = lhs.trim();
            if let Some(&matched) = sibling_deps.iter().find(|&&d| d == dep_key) {
                if rhs.contains("path =") {
                    if let Some(idx) = raw_line.find("version = \"") {
                        let start = idx + "version = \"".len();
                        if let Some(rel_end) = raw_line[start..].find('"') {
                            let mut updated = String::new();
                            updated.push_str(&raw_line[..start]);
                            updated.push_str(new_version);
                            updated.push_str(&raw_line[start + rel_end..]);
                            out.push(updated);
                            bumped_deps.insert(matched);
                            continue;
                        }
                    }
                    return Err(format!(
                        "sibling dependency `{dep_key}` uses path but has no `version = \"...\"` pin"
                    ));
                }
            }
        }
        out.push(raw_line.to_string());
    }

    if !bumped_pkg {
        return Err("missing `[package] version = \"...\"` field".to_string());
    }
    for &dep in sibling_deps {
        if !bumped_deps.contains(dep) {
            return Err(format!("missing sibling dependency pin for `{dep}`"));
        }
    }

    let mut joined = out.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    Ok(joined)
}

/// Read the `[package]` version string of a manifest path.
pub(crate) fn read_manifest_version(path: &Path) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    read_package_field(&text, "version")
        .ok_or_else(|| format!("{}: missing [package] version", path.display()))
}

/// Query crates.io sparse index to check whether `<crate_name>` at `<version>` is published and unyanked.
pub(crate) fn crates_io_has_version(crate_name: &str, version: &str) -> Result<bool, String> {
    let url = sparse_index_url(crate_name);
    let output = Command::new("curl")
        .args(["-sS", "-w", "\n%{http_code}", &url])
        .output()
        .map_err(|e| format!("failed to run curl for {url}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "curl failed for {url}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let body = String::from_utf8_lossy(&output.stdout);
    let trimmed = body.trim_end_matches(&['\r', '\n'][..]);
    let (content, code) = match trimmed.rsplit_once('\n') {
        Some((c, s)) => (c, s.trim()),
        None => ("", trimmed.trim()),
    };
    if code == "404" {
        return Ok(false);
    }
    if code != "200" {
        return Err(format!("crates.io sparse index {url} returned HTTP {code}"));
    }
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let val: Value = serde_json::from_str(line)
            .map_err(|e| format!("invalid NDJSON in sparse index {url}: {e}"))?;
        let matches_ver = val.get("vers").and_then(Value::as_str) == Some(version);
        let yanked = val.get("yanked").and_then(Value::as_bool) == Some(true);
        if matches_ver && !yanked {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Render standardized GitHub Release Markdown covering all 5 unified crates and platform scope.
pub(crate) fn render_release_notes(version: &str, already_published: &[String]) -> String {
    let mut rows = String::new();
    for c in PUBLISH_CRATES {
        let docs_col = if c.has_docs_rs {
            format!("[docs.rs/{0}](https://docs.rs/{0}/{version})", c.name)
        } else {
            "— (binary CLI)".to_string()
        };
        let status_note = if already_published.iter().any(|s| s == c.name) {
            format!("{0} (published earlier; unified into `v{version}`)", c.role)
        } else {
            c.role.to_string()
        };
        rows.push_str(&format!(
            "| [`{0}`](https://crates.io/crates/{0}/{version}) | `{version}` | {docs_col} | {status_note} |\n",
            c.name
        ));
    }

    format!(
        r##"## FEmacs & Standalone Lisp Runtime `v{version}`

Unified workspace release `v{version}` spanning all five publishable crates (`elisp`, `lisp`, `lisp-std`, `lisp-vm`, and `femacs`), zero external C build dependencies, No-GIL multi-core execution, and updated project documentation ([femacs.com](https://femacs.com/)).

### Published Crates (`v{version}`)

| Crate | Version | API Docs | Role |
| --- | --- | --- | --- |
{rows}
### Quickstart

```sh
# Standalone general-purpose Elisp-compatible CLI & interactive TTY REPL
cargo install elisp
elisp eval '(+ 20 22)'
elisp repl

# macOS Fe editor CLI launcher (requires built/installed Fe.app / Femacs.app)
cargo install femacs --locked
```

Embed in Rust (`Cargo.toml`):

```toml
[dependencies]
lisp = "{version}"
```

```rust
use lisp::prelude::*;

fn main() -> Result<(), Error> {{
    let mut engine = Engine::init(EngineConfig::cold_stdlib())?;
    let val = engine.eval(r#"(progn (require 'std/json) (std-json-stringify '((ok . t))))"#)?;
    assert_eq!(val.as_str(), Some("[[\"ok\",true]]"));
    Ok(())
}}
```

### Highlights

- **Four-crate standalone architecture**: Pure `[lib]` `lisp` (`default = ["vm", "std"]`, orthogonal `vm`, `std`, and `pkg` features, `default-features = false` zero-VM reuse), pure `[[bin]]` `elisp` (`default = ["repl-tty", "std"]`, `elisp> ` REPL prompt), pure VM `lisp-vm` (`isolate_vm`), and 100% pure-Rust `#![forbid(unsafe_code)]` `lisp-std`.
- **Zero-C-toolchain build graph**: Builds cleanly without `cc`, `cmake`, `pkg-config`, `bindgen`, OpenSSL, or C SQLite; macOS release binaries link solely `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`.
- **22 Pure-Rust `stdlib/std/*` Modules**: `std/task`, `std/chan`, `std/sync`, HTTPS/HTTP client & embedded HTTP/1.1 server (`std/http`), `redb` ACID KV + transactional SQL engine (`std/kv`, `std/db`), JSON/TOML/CSV/YAML/Base64/URL codecs, `tar`/`zip`/`gzip`/`zlib` (`std/archive`), CSPRNG/SHA/HMAC/UUID (`std/crypto`), timezones (`std/time`), `walkdir`/glob FS, regex, bounded subprocesses (`std/proc`), `.env`, CLI flag parsing, and structured logging.
- **Automated multi-crate release tooling & docs**: Synchronized 5-crate `cargo run -p xtask -- release` workflow (`plan`, `bump`, `check`, `notes`, `publish`) and updated project site at [femacs.com](https://femacs.com/).

### Platform Support Matrix (`v{version}`)

| Target | Status | Verification Scope |
| --- | --- | --- |
| **macOS (`aarch64-apple-darwin`)** | Verified (real host) | Zero-warning 14-target build/check matrix, full unit & integration tests across all feature combinations, interactive TTY REPL, `fe-test` (`--suite self` `9/9`, `--suite all` `39/39`), `otool -L` OS-libs-only linkage, and isolated 4-crate `cargo package` rebuild |
| **Linux (`aarch64-unknown-linux-gnu`)** | Verified (ephemeral Debian 12 VM) | Unit & integration suites, release `ldd` (glibc + system libs only), `.pdmp` warm cache, PTY REPL, 4-worker `NoGil` parallel speedup, and `init`/`pkg` workflow (verified prior to the `lisp` / `elisp` split) |
| **Linux (`x86_64-unknown-linux-gnu`)** | Cross-compile verified | Clean `cargo check` across workspace standalone crates |
| **Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`)** | **Cross-compile architecture verified ONLY** | Clean `cargo check` across default & minimal features (`windows-sys` Job Objects, Toolhelp32 snapshots, TCP loopback `--rpc-addr`, `LOCALAPPDATA` cache root); **no physical Windows host runtime verification has been performed** |

### Links

- Project website: https://femacs.com/
- Standalone Lisp page: https://femacs.com/pages/lisp.html
- Release workflow guide: https://github.com/liuchong/femacs/blob/v{version}/docs/release-process.md
- Full workspace documentation: https://github.com/liuchong/femacs/blob/v{version}/README.md
"##
    )
}

fn git_status_clean() -> Result<bool, String> {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("git status failed: {e}"))?;
    if !out.status.success() {
        return Err("git status returned non-zero".to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

fn find_js_under_docs(dir: &Path, hits: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("read_dir: {e}"))?;
        let p = entry.path();
        if p.is_dir() {
            find_js_under_docs(&p, hits)?;
        } else if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("mjs") {
                hits.push(p);
            }
        }
    }
    Ok(())
}

fn run_checked(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("failed to run `{cmd}`: {e}"))?;
    if !status.success() {
        return Err(format!(
            "`{cmd} {}` exited with status {:?}",
            args.join(" "),
            status.code()
        ));
    }
    Ok(())
}

fn parse_flag_value(args: &[String], flag: &str) -> Result<Option<String>, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            let val = args
                .get(i + 1)
                .ok_or_else(|| format!("{flag} requires a value"))?;
            return Ok(Some(val.clone()));
        }
        if let Some(rest) = args[i].strip_prefix(&format!("{flag}=")) {
            if rest.is_empty() {
                return Err(format!("{flag}= requires a non-empty value"));
            }
            return Ok(Some(rest.to_string()));
        }
        i += 1;
    }
    Ok(None)
}

fn require_version(args: &[String]) -> Result<String, String> {
    let ver = parse_flag_value(args, "--version")?
        .ok_or_else(|| "missing required `--version <X.Y.Z>`".to_string())?;
    if !is_valid_semver(&ver) {
        return Err(format!("invalid SemVer version: `{ver}`"));
    }
    Ok(ver)
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn local_tag_points_to_head(tag: &str) -> Result<bool, String> {
    let tag_ref = format!("refs/tags/{tag}");
    let out = Command::new("git")
        .args(["rev-parse", "--verify", &format!("{tag_ref}^{{commit}}")])
        .output()
        .map_err(|e| format!("git rev-parse tag: {e}"))?;
    if !out.status.success() {
        return Ok(false);
    }
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|e| format!("git rev-parse HEAD: {e}"))?;
    if !head.status.success() {
        return Err("git rev-parse HEAD failed".to_string());
    }
    let tag_sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let head_sha = String::from_utf8_lossy(&head.stdout).trim().to_string();
    if tag_sha != head_sha {
        return Err(format!(
            "tag `{tag}` already exists at {tag_sha}, not HEAD ({head_sha})"
        ));
    }
    Ok(true)
}

fn cmd_plan(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let clean = git_status_clean()?;
    let tag = format!("v{version}");
    let tag_at_head = match local_tag_points_to_head(&tag) {
        Ok(b) => if b { "points-to-HEAD" } else { "absent" }.to_string(),
        Err(e) => format!("conflict ({e})"),
    };
    println!("release plan for v{version}");
    println!("  git_clean: {clean}");
    println!("  local_tag ({tag}): {tag_at_head}");
    for (idx, c) in PUBLISH_CRATES.iter().enumerate() {
        let local_ver = read_manifest_version(Path::new(c.manifest))?;
        let live = crates_io_has_version(c.name, &version)?;
        let status = if live {
            "LIVE on crates.io"
        } else {
            "pending publish"
        };
        println!(
            "  {}. {:<9} manifest={}  version={}  crates.io={}",
            idx + 1,
            c.name,
            c.manifest,
            local_ver,
            status
        );
    }
    Ok(())
}

fn cmd_bump(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let dry_run = has_flag(args, "--dry-run");
    for c in PUBLISH_CRATES {
        let path = Path::new(c.manifest);
        let old = fs::read_to_string(path).map_err(|e| format!("{}: {e}", c.manifest))?;
        let updated = bump_manifest_text(&old, &version, c.internal_deps)?;
        if dry_run {
            println!("would update {} -> {version}", c.manifest);
        } else {
            fs::write(path, updated).map_err(|e| format!("write {}: {e}", c.manifest))?;
            println!("updated {} -> {version}", c.manifest);
        }
    }
    if !dry_run {
        run_checked("cargo", &["update", "--workspace"])?;
    }
    Ok(())
}

fn verify_manifests_and_readmes(version: &str) -> Result<(), String> {
    for c in PUBLISH_CRATES {
        let manifest_path = Path::new(c.manifest);
        let text = fs::read_to_string(manifest_path).map_err(|e| format!("{}: {e}", c.manifest))?;
        let actual_ver = read_package_field(&text, "version")
            .ok_or_else(|| format!("{}: missing [package] version", c.manifest))?;
        if actual_ver != version {
            return Err(format!(
                "{}: expected version {version}, found {actual_ver}",
                c.manifest
            ));
        }
        for (field, expected) in [
            ("homepage", EXPECTED_HOMEPAGE),
            ("repository", EXPECTED_REPOSITORY),
            ("readme", EXPECTED_README),
        ] {
            let val = read_package_field(&text, field)
                .ok_or_else(|| format!("{}: missing `{field}`", c.manifest))?;
            if val != expected {
                return Err(format!(
                    "{}: `{field}` is `{val}`, expected `{expected}`",
                    c.manifest
                ));
            }
        }
        for &dep in c.internal_deps {
            let pinned = read_dep_version(&text, dep)
                .ok_or_else(|| format!("{}: missing version pin for sibling `{dep}`", c.manifest))?;
            if pinned != version {
                return Err(format!(
                    "{}: sibling dep `{dep}` pinned at `{pinned}`, expected `{version}`",
                    c.manifest
                ));
            }
        }
        let readme_path = Path::new(c.readme);
        let readme = fs::read_to_string(readme_path).map_err(|e| format!("{}: {e}", c.readme))?;
        if readme.trim().is_empty() {
            return Err(format!("{} is empty", c.readme));
        }
        if readme.contains("](../") {
            return Err(format!(
                "{} contains relative `](../` link (broken on crates.io)",
                c.readme
            ));
        }
        if !Path::new(c.license).exists() {
            return Err(format!("missing LICENSE file at {}", c.license));
        }
    }
    Ok(())
}

fn cmd_check(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let allow_dirty = has_flag(args, "--allow-dirty");
    let skip_package = has_flag(args, "--skip-package");

    if !allow_dirty && !git_status_clean()? {
        return Err("working tree is dirty; commit changes first or pass --allow-dirty".to_string());
    }
    verify_manifests_and_readmes(&version)?;

    let mut js_hits = Vec::new();
    find_js_under_docs(Path::new("docs"), &mut js_hits)?;
    if !js_hits.is_empty() {
        return Err(format!(
            "forbidden generated JS files under docs/: {:?}",
            js_hits
        ));
    }

    run_checked("python3", &["tools/check-own-voice.py"])?;
    run_checked("python3", &["tools/check-crate-boundaries.py"])?;

    if !skip_package {
        for leaf in ["lisp-vm", "lisp-std", "femacs"] {
            let mut pkg_args = vec!["package", "-p", leaf];
            if allow_dirty {
                pkg_args.push("--allow-dirty");
            }
            run_checked("cargo", &pkg_args)?;
        }
        let mut multi_args = vec![
            "package",
            "-p",
            "lisp-vm",
            "-p",
            "lisp-std",
            "-p",
            "femacs",
            "-p",
            "lisp",
            "-p",
            "elisp",
            "--no-verify",
        ];
        if allow_dirty {
            multi_args.push("--allow-dirty");
        }
        run_checked("cargo", &multi_args)?;

        for c in PUBLISH_CRATES {
            let archive = PathBuf::from(format!("target/package/{}-{version}.crate", c.name));
            let meta = fs::metadata(&archive)
                .map_err(|e| format!("missing {}: {e}", archive.display()))?;
            if meta.len() >= MAX_CRATE_TARBALL_BYTES {
                return Err(format!(
                    "{} size {} exceeds crates.io 10 MiB limit",
                    archive.display(),
                    meta.len()
                ));
            }
            println!(
                "packaged {} ({} bytes, {:.2} MiB)",
                archive.display(),
                meta.len(),
                meta.len() as f64 / (1024.0 * 1024.0)
            );
        }
    }
    println!("release check v{version}: OK");
    Ok(())
}

fn cmd_notes(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let out = parse_flag_value(args, "--out")?;
    let already_raw = parse_flag_value(args, "--already-published")?.unwrap_or_default();
    let already: Vec<String> = already_raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    let notes = render_release_notes(&version, &already);
    match out {
        Some(path) => {
            if let Some(parent) = Path::new(&path).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
                }
            }
            fs::write(&path, notes).map_err(|e| format!("write {path}: {e}"))?;
            println!("wrote release notes to {path}");
        }
        None => print!("{notes}"),
    }
    Ok(())
}

fn wait_for_sparse_index(crate_name: &str, version: &str, timeout_secs: u64) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        if crates_io_has_version(crate_name, version)? {
            println!("indexed {crate_name} {version} on crates.io");
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "timed out after {timeout_secs}s waiting for {crate_name} {version} on crates.io sparse index"
            ));
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn cmd_publish(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let execute = has_flag(args, "--execute");
    let skip_already_published = has_flag(args, "--skip-already-published");
    let skip_tag = has_flag(args, "--skip-tag");
    let skip_github = has_flag(args, "--skip-github");
    let notes_file_arg = parse_flag_value(args, "--notes-file")?;
    let wait_timeout: u64 = match parse_flag_value(args, "--wait-timeout-secs")? {
        Some(raw) => raw
            .parse::<u64>()
            .map_err(|e| format!("invalid --wait-timeout-secs `{raw}`: {e}"))?,
        None => 180,
    };

    verify_manifests_and_readmes(&version)?;

    if !execute {
        println!("release publish --version {version} (DRY-RUN preview; pass --execute to publish)");
        for c in PUBLISH_CRATES {
            let live = crates_io_has_version(c.name, &version)?;
            let action = if live && skip_already_published {
                "SKIP (already live on crates.io)"
            } else if live {
                "CONFLICT (already live; pass --skip-already-published)"
            } else {
                "PUBLISH via `cargo publish`"
            };
            println!("  - {:<9} v{version} -> {action}", c.name);
        }
        println!("  - git tag -s v{version} && git push origin v{version}");
        println!("  - gh release create v{version} --verify-tag");
        return Ok(());
    }

    if !git_status_clean()? {
        return Err("refusing `release publish --execute` on a dirty working tree".to_string());
    }

    let mut already_published: Vec<String> = Vec::new();
    for c in PUBLISH_CRATES {
        let live = crates_io_has_version(c.name, &version)?;
        if live {
            if skip_already_published {
                println!(
                    "SKIP {} {version} (already published on crates.io)",
                    c.name
                );
                already_published.push(c.name.to_string());
                continue;
            }
            return Err(format!(
                "{} {version} is already published on crates.io (pass --skip-already-published to unify into this release)",
                c.name
            ));
        }
        println!("PUBLISH {} {version} ...", c.name);
        run_checked("cargo", &["publish", "-p", c.name])?;
        wait_for_sparse_index(c.name, &version, wait_timeout)?;
    }

    let tag = format!("v{version}");
    fs::create_dir_all(TMP_REL_DIR).map_err(|e| format!("{TMP_REL_DIR}: {e}"))?;
    let res = (|| -> Result<(), String> {
        if !skip_tag {
            let tag_at_head = local_tag_points_to_head(&tag)?;
            if !tag_at_head {
                let tag_msg_path = Path::new(TMP_REL_DIR).join(format!("tag-{tag}.txt"));
                let tag_msg = format!(
                    "Release {tag} (elisp, lisp, lisp-std, lisp-vm, femacs) [skip ci]\n"
                );
                fs::write(&tag_msg_path, tag_msg)
                    .map_err(|e| format!("write {}: {e}", tag_msg_path.display()))?;
                let msg_str = tag_msg_path.to_str().ok_or("non-utf8 path")?;
                run_checked("git", &["tag", "-s", &tag, "-F", msg_str])?;
            }
            run_checked("git", &["tag", "-v", &tag])?;
            run_checked("git", &["push", "origin", &tag])?;
        }

        if !skip_github {
            let notes_path = match notes_file_arg {
                Some(ref p) => PathBuf::from(p),
                None => {
                    let p = Path::new(TMP_REL_DIR).join(format!("notes-{tag}.md"));
                    let content = render_release_notes(&version, &already_published);
                    fs::write(&p, content).map_err(|e| format!("write {}: {e}", p.display()))?;
                    p
                }
            };
            let title = format!(
                "{tag} — FEmacs & Standalone Lisp Runtime (elisp, lisp, lisp-std, lisp-vm, femacs)"
            );
            let notes_str = notes_path.to_str().ok_or("non-utf8 path")?;
            run_checked(
                "gh",
                &[
                    "release",
                    "create",
                    &tag,
                    "--title",
                    &title,
                    "--notes-file",
                    notes_str,
                    "--verify-tag",
                ],
            )?;
            run_checked(
                "gh",
                &["release", "view", &tag, "--json", "url,tagName,name"],
            )?;
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(TMP_REL_DIR);
    res
}

fn print_usage() {
    println!(
        "usage: xtask release <plan|bump|check|notes|publish> --version <X.Y.Z> [options]\n\
         \n\
         Subcommands:\n\
         \x20 plan    --version <X.Y.Z>                       Show crate versions & crates.io status\n\
         \x20 bump    --version <X.Y.Z> [--dry-run]           Bump all 5 manifests + sibling pins\n\
         \x20 check   --version <X.Y.Z> [--allow-dirty] [--skip-package]\n\
         \x20 notes   --version <X.Y.Z> [--out <FILE>] [--already-published a,b]\n\
         \x20 publish --version <X.Y.Z> [--execute] [--skip-already-published] [--skip-tag] [--skip-github] [--notes-file <FILE>] [--wait-timeout-secs N]"
    );
}

/// Dispatch `cargo run -p xtask -- release ...`.
pub fn command(args: &[String]) -> i32 {
    let sub = args.first().map(String::as_str).unwrap_or("help");
    let rest = if args.len() > 1 { &args[1..] } else { &[] };
    let res = match sub {
        "plan" => cmd_plan(rest),
        "bump" => cmd_bump(rest),
        "check" => cmd_check(rest),
        "notes" => cmd_notes(rest),
        "publish" => cmd_publish(rest),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        other => Err(format!("unknown release subcommand `{other}`")),
    };
    match res {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e}");
            if sub == "help" || e.starts_with("unknown release subcommand") {
                print_usage();
            }
            2
        }
    }
}
