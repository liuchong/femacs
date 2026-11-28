//! Unified multi-channel release workflow (`vX.Y.Z`):
//! - 5 crates.io crates (`lisp-vm`, `lisp-std`, `femacs`, `lisp`, `elisp`)
//! - Prebuilt release archives + `SHA256SUMS.txt` on GitHub Releases
//! - Zero-dependency npm wrapper (`femacs`, Eliscript `.eli` source)
//! - Homebrew tap (`liuchong/homebrew-tap` / `packaging/homebrew/femacs.rb`)
//! - Shell installer (`docs/install.sh`)
#![forbid(unsafe_code)]

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
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

/// Supported prebuilt binary distribution target mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DistTarget {
    pub npm_key: &'static str,
    pub triple: &'static str,
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

pub(crate) const DIST_TARGETS: &[DistTarget] = &[
    DistTarget {
        npm_key: "darwin-arm64",
        triple: "aarch64-apple-darwin",
    },
    DistTarget {
        npm_key: "darwin-x64",
        triple: "x86_64-apple-darwin",
    },
    DistTarget {
        npm_key: "linux-arm64",
        triple: "aarch64-unknown-linux-gnu",
    },
    DistTarget {
        npm_key: "linux-x64",
        triple: "x86_64-unknown-linux-gnu",
    },
];

const EXPECTED_HOMEPAGE: &str = "https://femacs.com/";
const EXPECTED_REPOSITORY: &str = "https://github.com/liuchong/femacs";
const EXPECTED_README: &str = "README.md";
const MAX_CRATE_TARBALL_BYTES: u64 = 10 * 1024 * 1024;
const TMP_REL_DIR: &str = "build/release-tmp";

const NPM_PACKAGE_DIR: &str = "packaging/npm/femacs";
const NPM_PACKAGE_JSON: &str = "packaging/npm/femacs/package.json";
const NPM_MANIFEST_JSON: &str = "packaging/npm/femacs/manifest.json";
const NPM_README: &str = "packaging/npm/femacs/README.md";
const NPM_LICENSE: &str = "packaging/npm/femacs/LICENSE";
const HOMEBREW_FORMULA: &str = "packaging/homebrew/femacs.rb";
const INSTALL_SH: &str = "docs/install.sh";

/// Validate strict `MAJOR.MINOR.PATCH` (with optional `-prerelease`) SemVer string.
pub(crate) fn is_valid_semver(version: &str) -> bool {
    let (core, _pre) = match version.split_once('-') {
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
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
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

/// Update top-level `"version"` inside `packaging/npm/femacs/package.json`.
pub(crate) fn bump_npm_package_json(text: &str, new_version: &str) -> Result<String, String> {
    let mut val: Value =
        serde_json::from_str(text).map_err(|e| format!("invalid npm package.json: {e}"))?;
    let obj = val
        .as_object_mut()
        .ok_or_else(|| "npm package.json root must be an object".to_string())?;
    if !obj.contains_key("version") {
        return Err("npm package.json missing `version`".to_string());
    }
    obj.insert("version".to_string(), Value::String(new_version.to_string()));
    let mut out = serde_json::to_string_pretty(&val)
        .map_err(|e| format!("failed to format npm package.json: {e}"))?;
    out.push('\n');
    Ok(out)
}

/// Update `FEMACS_VERSION="${FEMACS_VERSION:-...}"` in `docs/install.sh`.
pub(crate) fn bump_install_sh(text: &str, new_version: &str) -> Result<String, String> {
    let prefix = "FEMACS_VERSION=\"${FEMACS_VERSION:-";
    let mut out = Vec::new();
    let mut replaced = false;
    for line in text.lines() {
        if !replaced && line.starts_with(prefix) && line.ends_with("}\"") {
            out.push(format!("{prefix}{new_version}}}\""));
            replaced = true;
        } else {
            out.push(line.to_string());
        }
    }
    if !replaced {
        return Err("docs/install.sh missing `FEMACS_VERSION=\"${FEMACS_VERSION:-...}\"`".to_string());
    }
    let mut joined = out.join("\n");
    if text.ends_with('\n') {
        joined.push('\n');
    }
    Ok(joined)
}

/// Parse standard `shasum -a 256` output (`<sha256>  <filename>`) into `filename -> sha256`.
pub(crate) fn parse_checksums(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut map = BTreeMap::new();
    for (idx, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let sha = parts
            .next()
            .ok_or_else(|| format!("checksums line {}: missing digest", idx + 1))?;
        let file = parts
            .next()
            .ok_or_else(|| format!("checksums line {}: missing filename", idx + 1))?;
        if parts.next().is_some() {
            return Err(format!("checksums line {}: unexpected extra columns", idx + 1));
        }
        let lower_sha = sha.to_ascii_lowercase();
        if !is_sha256_hex(&lower_sha) {
            return Err(format!(
                "checksums line {}: invalid sha256 `{sha}`",
                idx + 1
            ));
        }
        let base = Path::new(file)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("checksums line {}: invalid filename `{file}`", idx + 1))?
            .to_string();
        map.insert(base, lower_sha);
    }
    Ok(map)
}

/// Deterministically render `packaging/npm/femacs/manifest.json` from `SHA256SUMS.txt`.
pub(crate) fn render_npm_manifest_json(
    version: &str,
    checksums: &BTreeMap<String, String>,
) -> Result<String, String> {
    let mut entries = Vec::new();
    for target in DIST_TARGETS {
        let archive = format!("femacs-v{version}-{}.tar.gz", target.triple);
        if let Some(sha) = checksums.get(&archive) {
            entries.push((target, archive, sha));
        }
    }
    if entries.is_empty() {
        return Err(format!(
            "no matching `femacs-v{version}-<triple>.tar.gz` entries found in checksums"
        ));
    }

    let mut targets_json = String::new();
    for (i, (target, archive, sha)) in entries.iter().enumerate() {
        let comma = if i + 1 < entries.len() { "," } else { "" };
        targets_json.push_str(&format!(
            "    \"{}\": {{\n\
             \x20     \"triple\": \"{}\",\n\
             \x20     \"archive\": \"{}\",\n\
             \x20     \"format\": \"tar.gz\",\n\
             \x20     \"sha256\": \"{}\",\n\
             \x20     \"bins\": {{\n\
             \x20       \"elisp\": \"elisp\",\n\
             \x20       \"femacs\": \"femacs\",\n\
             \x20       \"fe\": \"fe\"\n\
             \x20     }}\n\
             \x20   }}{comma}\n",
            target.npm_key, target.triple, archive, sha
        ));
    }

    Ok(format!(
        "{{\n\
         \x20 \"version\": \"{version}\",\n\
         \x20 \"tag\": \"v{version}\",\n\
         \x20 \"releaseBaseUrl\": \"{EXPECTED_REPOSITORY}/releases/download/v{version}\",\n\
         \x20 \"binaries\": [\"elisp\", \"femacs\", \"fe\"],\n\
         \x20 \"targets\": {{\n\
         {targets_json}\
         \x20 }}\n\
         }}\n"
    ))
}

fn find_target_sha<'a>(
    checksums: &'a BTreeMap<String, String>,
    version: &str,
    triple: &str,
) -> Option<&'a str> {
    let archive = format!("femacs-v{version}-{triple}.tar.gz");
    checksums.get(&archive).map(String::as_str)
}

/// Deterministically render `packaging/homebrew/femacs.rb` from `SHA256SUMS.txt`.
pub(crate) fn render_homebrew_formula(
    version: &str,
    checksums: &BTreeMap<String, String>,
) -> Result<String, String> {
    let base = format!("{EXPECTED_REPOSITORY}/releases/download/v{version}");
    let darwin_arm = find_target_sha(checksums, version, "aarch64-apple-darwin");
    let darwin_x64 = find_target_sha(checksums, version, "x86_64-apple-darwin");
    let linux_arm = find_target_sha(checksums, version, "aarch64-unknown-linux-gnu");
    let linux_x64 = find_target_sha(checksums, version, "x86_64-unknown-linux-gnu");

    if darwin_arm.is_none() && darwin_x64.is_none() && linux_arm.is_none() && linux_x64.is_none() {
        return Err(format!(
            "no supported Unix archives for `v{version}` found in checksums"
        ));
    }

    let mut body = format!(
        "class Femacs < Formula\n\
         \x20 desc \"Standalone No-GIL Elisp-compatible CLI (elisp) and Fe editor CLI launchers\"\n\
         \x20 homepage \"{EXPECTED_HOMEPAGE}\"\n\
         \x20 version \"{version}\"\n\
         \x20 license \"GPL-3.0-or-later\"\n"
    );

    if darwin_arm.is_some() || darwin_x64.is_some() {
        body.push_str("\n  on_macos do\n");
        if let Some(sha) = darwin_arm {
            body.push_str(&format!(
                "    on_arm do\n\
                 \x20     url \"{base}/femacs-v{version}-aarch64-apple-darwin.tar.gz\"\n\
                 \x20     sha256 \"{sha}\"\n\
                 \x20   end\n"
            ));
        }
        if let Some(sha) = darwin_x64 {
            if darwin_arm.is_some() {
                body.push('\n');
            }
            body.push_str(&format!(
                "    on_intel do\n\
                 \x20     url \"{base}/femacs-v{version}-x86_64-apple-darwin.tar.gz\"\n\
                 \x20     sha256 \"{sha}\"\n\
                 \x20   end\n"
            ));
        }
        body.push_str("  end\n");
    }

    if linux_arm.is_some() || linux_x64.is_some() {
        body.push_str("\n  on_linux do\n");
        if let Some(sha) = linux_arm {
            body.push_str(&format!(
                "    on_arm do\n\
                 \x20     url \"{base}/femacs-v{version}-aarch64-unknown-linux-gnu.tar.gz\"\n\
                 \x20     sha256 \"{sha}\"\n\
                 \x20   end\n"
            ));
        }
        if let Some(sha) = linux_x64 {
            if linux_arm.is_some() {
                body.push('\n');
            }
            body.push_str(&format!(
                "    on_intel do\n\
                 \x20     url \"{base}/femacs-v{version}-x86_64-unknown-linux-gnu.tar.gz\"\n\
                 \x20     sha256 \"{sha}\"\n\
                 \x20   end\n"
            ));
        }
        body.push_str("  end\n");
    }

    body.push_str(
        "\n  def install\n\
         \x20   bin.install \"elisp\", \"femacs\", \"fe\"\n\
         \x20 end\n\
         \n  test do\n\
         \x20   assert_match \"elisp #{version}\", shell_output(\"#{bin}/elisp --version\")\n\
         \x20   assert_equal \"42\", shell_output(\"#{bin}/elisp --no-auto-warm eval '(+ 20 22)'\").strip\n\
         \x20 end\n\
         end\n",
    );

    Ok(body)
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

/// Render standardized GitHub Release Markdown covering crates, prebuilt binaries, npm, Homebrew, and shell channels.
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

    let bootstrap_note = if version == "0.0.1" {
        "\n> **Bootstrap note (`v0.0.1`)**: Tag `v0.0.1` remains pinned at its original release commit (`874a28d0`); the `x86_64` prebuilt archives attached to this release were built after adding the non-aarch64 fallback to `NEAR_STACK_TOP!` in `CLN320`.\n"
    } else {
        ""
    };

    format!(
        r##"## FEmacs & Standalone Lisp Runtime `v{version}`

Unified workspace release `v{version}` spanning all five crates (`elisp`, `lisp`, `lisp-std`, `lisp-vm`, `femacs`), prebuilt `elisp` + `femacs` + `fe` archives (`SHA256SUMS.txt`), zero-dependency npm package (`femacs@{version}`), Homebrew tap (`liuchong/tap/femacs`), shell installer ([`install.sh`](https://femacs.com/install.sh)), and project documentation ([femacs.com](https://femacs.com/)).
{bootstrap_note}
### Published Packages (`v{version}`)

| Crate / Channel | Version | Reference / Docs | Role |
| --- | --- | --- | --- |
{rows}| [`femacs` (npm)](https://www.npmjs.com/package/femacs) | `{version}` | Zero runtime dependencies | Prebuilt binary launcher exposing `elisp`, `femacs`, and `fe` |
| [`liuchong/homebrew-tap`](https://github.com/liuchong/homebrew-tap) | `{version}` | `brew install liuchong/tap/femacs` | Homebrew Formula for macOS & Linux (`elisp`, `femacs`, `fe`) |

### Quickstart

```sh
# 1. Homebrew (macOS & Linux)
brew install liuchong/tap/femacs

# 2. npm (zero runtime dependencies)
npm install -g femacs

# 3. Shell installer (macOS & Linux, SHA-256 verified)
curl -fsSL https://femacs.com/install.sh | sh

# 4. Cargo from source (all platforms; macOS launcher CLI needs Fe.app / Femacs.app)
cargo install elisp
cargo install femacs --locked
```

Verify the standalone runtime:

```sh
elisp eval '(+ 20 22)'
elisp --concurrency=no-gil eval '(progn (require (quote std/json)) (std-json-stringify (list (cons "ok" t))))'
elisp repl
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

### Prebuilt Release Archives (`SHA256SUMS.txt`)

Every release archive packs flat `elisp`, `femacs`, `fe`, and `LICENSE` verified against [`SHA256SUMS.txt`](https://github.com/liuchong/femacs/releases/download/v{version}/SHA256SUMS.txt):

| Target Triple | Archive | Notes |
| --- | --- | --- |
| `aarch64-apple-darwin` | `femacs-v{version}-aarch64-apple-darwin.tar.gz` | Apple Silicon macOS (`/usr/lib/libSystem.B.dylib` & `libiconv.2.dylib` only) |
| `x86_64-apple-darwin` | `femacs-v{version}-x86_64-apple-darwin.tar.gz` | Intel macOS (`/usr/lib/libSystem.B.dylib` & `libiconv.2.dylib` only) |
| `aarch64-unknown-linux-gnu` | `femacs-v{version}-aarch64-unknown-linux-gnu.tar.gz` | Linux `aarch64` (`glibc >= 2.31`) |
| `x86_64-unknown-linux-gnu` | `femacs-v{version}-x86_64-unknown-linux-gnu.tar.gz` | Linux `x86_64` (`glibc >= 2.31`) |

### Highlights

- **Four-crate standalone architecture**: Pure `[lib]` `lisp` (`default = ["vm", "std"]`, orthogonal `vm`, `std`, and `pkg` features, `default-features = false` zero-VM reuse), pure `[[bin]]` `elisp` (`default = ["repl-tty", "std"]`, `elisp> ` REPL prompt), pure VM `lisp-vm` (`isolate_vm`), and 100% pure-Rust `#![forbid(unsafe_code)]` `lisp-std`.
- **Zero-C-toolchain build graph**: Builds cleanly without `cc`, `cmake`, `pkg-config`, `bindgen`, OpenSSL, or C SQLite; macOS release binaries link solely `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`.
- **22 Pure-Rust `stdlib/std/*` Modules**: `std/task`, `std/chan`, `std/sync`, HTTPS/HTTP client & embedded HTTP/1.1 server (`std/http`), `redb` ACID KV + transactional SQL engine (`std/kv`, `std/db`), JSON/TOML/CSV/YAML/Base64/URL codecs, `tar`/`zip`/`gzip`/`zlib` (`std/archive`), CSPRNG/SHA/HMAC/UUID (`std/crypto`), timezones (`std/time`), `walkdir`/glob FS, regex, bounded subprocesses (`std/proc`), `.env`, CLI flag parsing, and structured logging.
- **Zero-dependency Eliscript npm launcher**: `packaging/npm/femacs/` compiles 100% `.eli` sources into `dist/*.mjs` during `prepack`, downloads verified archives lazily, and caches binaries locally.

### Platform Support Matrix (`v{version}`)

| Target | Status | Verification Scope |
| --- | --- | --- |
| **macOS (`aarch64-apple-darwin`)** | Verified (real host) | Zero-warning 14-target build/check matrix, full unit & integration tests, TTY REPL, `fe-test` (`9/9` & `39/39`), `otool -L` OS-libs-only linkage, 4-crate `cargo package` rebuild, and real-network npm + `install.sh` smoke tests |
| **macOS (`x86_64-apple-darwin`)** | Cross-compile + Mach-O verified | Clean `--release` cross-compile + `otool -L` system-dylib inspection (`libiconv.2` & `libSystem.B` only); no physical Intel Mac or Rosetta runtime test |
| **Linux (`aarch64-unknown-linux-gnu`)** | Verified (ephemeral Debian 12 VM) + release ELF | Unit & integration suites, release `ldd` (glibc + system libs only), `.pdmp` warm cache, PTY REPL, 4-worker `NoGil` speedup, and Zig-linked `--release` archive (`glibc >= 2.31`) |
| **Linux (`x86_64-unknown-linux-gnu`)** | Cross-compile + release ELF verified | Zig-linked `--release` ELF x86-64 binary build & clean `cargo check` across standalone crates; not executed on physical x86-64 Linux hardware |
| **Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`)** | **Cross-compile architecture verified ONLY** | Clean `cargo check` across default & minimal features (`windows-sys` Job Objects, Toolhelp32 snapshots, TCP loopback `--rpc-addr`, `LOCALAPPDATA` cache root); **no physical Windows host runtime verification has been performed** (no prebuilt Windows binary in `v{version}`) |

### Links

- Project website: https://femacs.com/
- Standalone Lisp page: https://femacs.com/pages/lisp.html
- Release workflow guide: https://github.com/liuchong/femacs/blob/v{version}/docs/release-process.md
- Full workspace documentation: https://github.com/liuchong/femacs/blob/v{version}/README.md
"##
    )
}

fn git_status_clean_in(dir: &Path) -> Result<bool, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("git status failed in {}: {e}", dir.display()))?;
    if !out.status.success() {
        return Err(format!("git status returned non-zero in {}", dir.display()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().is_empty())
}

fn git_status_clean() -> Result<bool, String> {
    git_status_clean_in(Path::new("."))
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

fn run_checked_in(dir: &Path, cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .map_err(|e| format!("failed to run `{cmd}` in {}: {e}", dir.display()))?;
    if !status.success() {
        return Err(format!(
            "`{cmd} {}` in {} exited with status {:?}",
            args.join(" "),
            dir.display(),
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
    if let Ok(pkg_raw) = fs::read_to_string(NPM_PACKAGE_JSON) {
        if let Ok(pkg_val) = serde_json::from_str::<Value>(&pkg_raw) {
            let ver = pkg_val
                .get("version")
                .and_then(Value::as_str)
                .unwrap_or("?");
            println!("  6. {:<9} manifest={}  version={}", "npm", NPM_PACKAGE_JSON, ver);
        }
    }
    if Path::new(HOMEBREW_FORMULA).exists() {
        println!("  7. {:<9} formula={}", "homebrew", HOMEBREW_FORMULA);
    }
    if Path::new(INSTALL_SH).exists() {
        println!("  8. {:<9} script={}", "install", INSTALL_SH);
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

    if Path::new(NPM_PACKAGE_JSON).exists() {
        let old = fs::read_to_string(NPM_PACKAGE_JSON)
            .map_err(|e| format!("{NPM_PACKAGE_JSON}: {e}"))?;
        let updated = bump_npm_package_json(&old, &version)?;
        if dry_run {
            println!("would update {NPM_PACKAGE_JSON} -> {version}");
        } else {
            fs::write(NPM_PACKAGE_JSON, updated)
                .map_err(|e| format!("write {NPM_PACKAGE_JSON}: {e}"))?;
            println!("updated {NPM_PACKAGE_JSON} -> {version}");
        }
    }

    if Path::new(INSTALL_SH).exists() {
        let old = fs::read_to_string(INSTALL_SH).map_err(|e| format!("{INSTALL_SH}: {e}"))?;
        let updated = bump_install_sh(&old, &version)?;
        if dry_run {
            println!("would update {INSTALL_SH} -> {version}");
        } else {
            fs::write(INSTALL_SH, updated).map_err(|e| format!("write {INSTALL_SH}: {e}"))?;
            println!("updated {INSTALL_SH} -> {version}");
        }
    }

    if !dry_run {
        run_checked("cargo", &["update", "--workspace"])?;
    }
    Ok(())
}

fn sync_dist_files(version: &str, checksums_path: &Path) -> Result<(), String> {
    let raw = fs::read_to_string(checksums_path)
        .map_err(|e| format!("{}: {e}", checksums_path.display()))?;
    let checksums = parse_checksums(&raw)?;
    let manifest_text = render_npm_manifest_json(version, &checksums)?;
    let formula_text = render_homebrew_formula(version, &checksums)?;
    fs::write(NPM_MANIFEST_JSON, manifest_text)
        .map_err(|e| format!("write {NPM_MANIFEST_JSON}: {e}"))?;
    fs::write(HOMEBREW_FORMULA, formula_text)
        .map_err(|e| format!("write {HOMEBREW_FORMULA}: {e}"))?;
    println!("synced {NPM_MANIFEST_JSON} and {HOMEBREW_FORMULA} for v{version}");
    Ok(())
}

fn cmd_sync_dist(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let checksums = parse_flag_value(args, "--checksums")?
        .ok_or_else(|| "missing required `--checksums <SHA256SUMS.txt>`".to_string())?;
    sync_dist_files(&version, Path::new(&checksums))
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

/// Validate `packaging/npm/femacs/`, `packaging/homebrew/femacs.rb`, and `docs/install.sh`.
pub(crate) fn verify_distribution_artifacts(version: &str) -> Result<(), String> {
    // 1. npm package.json
    let pkg_raw = fs::read_to_string(NPM_PACKAGE_JSON)
        .map_err(|e| format!("{NPM_PACKAGE_JSON}: {e}"))?;
    let pkg: Value = serde_json::from_str(&pkg_raw)
        .map_err(|e| format!("{NPM_PACKAGE_JSON}: invalid JSON: {e}"))?;
    if pkg.get("name").and_then(Value::as_str) != Some("femacs") {
        return Err(format!("{NPM_PACKAGE_JSON}: expected name `femacs`"));
    }
    if pkg.get("version").and_then(Value::as_str) != Some(version) {
        return Err(format!("{NPM_PACKAGE_JSON}: expected version `{version}`"));
    }
    if let Some(deps) = pkg.get("dependencies") {
        let non_empty = deps.as_object().map(|m| !m.is_empty()).unwrap_or(true);
        if non_empty {
            return Err(format!(
                "{NPM_PACKAGE_JSON}: must have zero runtime `dependencies`"
            ));
        }
    }
    if pkg.get("homepage").and_then(Value::as_str) != Some(EXPECTED_HOMEPAGE) {
        return Err(format!(
            "{NPM_PACKAGE_JSON}: expected homepage `{EXPECTED_HOMEPAGE}`"
        ));
    }
    if pkg.get("license").and_then(Value::as_str) != Some("GPL-3.0-or-later") {
        return Err(format!(
            "{NPM_PACKAGE_JSON}: expected license `GPL-3.0-or-later`"
        ));
    }

    // 2. npm manifest.json
    let man_raw = fs::read_to_string(NPM_MANIFEST_JSON)
        .map_err(|e| format!("{NPM_MANIFEST_JSON}: {e}"))?;
    let man: Value = serde_json::from_str(&man_raw)
        .map_err(|e| format!("{NPM_MANIFEST_JSON}: invalid JSON: {e}"))?;
    if man.get("version").and_then(Value::as_str) != Some(version) {
        return Err(format!("{NPM_MANIFEST_JSON}: expected version `{version}`"));
    }
    if man.get("tag").and_then(Value::as_str) != Some(&format!("v{version}")) {
        return Err(format!("{NPM_MANIFEST_JSON}: expected tag `v{version}`"));
    }
    let targets = man
        .get("targets")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{NPM_MANIFEST_JSON}: missing `targets` object"))?;
    if targets.is_empty() {
        return Err(format!("{NPM_MANIFEST_JSON}: `targets` cannot be empty"));
    }
    for (key, entry) in targets {
        let triple = entry
            .get("triple")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{NPM_MANIFEST_JSON}: target `{key}` missing `triple`"))?;
        let archive = entry
            .get("archive")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{NPM_MANIFEST_JSON}: target `{key}` missing `archive`"))?;
        let expected_archive = format!("femacs-v{version}-{triple}.tar.gz");
        if archive != expected_archive {
            return Err(format!(
                "{NPM_MANIFEST_JSON}: target `{key}` archive `{archive}`, expected `{expected_archive}`"
            ));
        }
        let sha = entry
            .get("sha256")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{NPM_MANIFEST_JSON}: target `{key}` missing `sha256`"))?;
        if !is_sha256_hex(sha) {
            return Err(format!(
                "{NPM_MANIFEST_JSON}: target `{key}` has invalid sha256 `{sha}`"
            ));
        }
    }

    // 3. npm README, LICENSE, and zero tracked .js/.mjs files
    let npm_readme = fs::read_to_string(NPM_README).map_err(|e| format!("{NPM_README}: {e}"))?;
    if npm_readme.trim().is_empty() {
        return Err(format!("{NPM_README} is empty"));
    }
    if npm_readme.contains("](../") {
        return Err(format!("{NPM_README} contains relative `](../` link"));
    }
    if !Path::new(NPM_LICENSE).exists() {
        return Err(format!("missing LICENSE at {NPM_LICENSE}"));
    }
    let tracked = Command::new("git")
        .args(["ls-files", NPM_PACKAGE_DIR])
        .output()
        .map_err(|e| format!("git ls-files {NPM_PACKAGE_DIR}: {e}"))?;
    if !tracked.status.success() {
        return Err(format!("git ls-files {NPM_PACKAGE_DIR} failed"));
    }
    for rel in String::from_utf8_lossy(&tracked.stdout).lines() {
        if rel.ends_with(".js") || rel.ends_with(".mjs") {
            return Err(format!(
                "forbidden tracked JS/MJS file in {NPM_PACKAGE_DIR}: {rel}"
            ));
        }
    }

    // 4. Homebrew formula
    let formula =
        fs::read_to_string(HOMEBREW_FORMULA).map_err(|e| format!("{HOMEBREW_FORMULA}: {e}"))?;
    if !formula.contains(&format!("version \"{version}\"")) {
        return Err(format!(
            "{HOMEBREW_FORMULA}: expected `version \"{version}\"`"
        ));
    }
    if !formula.contains(&format!("femacs-v{version}-")) {
        return Err(format!(
            "{HOMEBREW_FORMULA}: missing archive URL for v{version}"
        ));
    }
    run_checked("ruby", &["-c", HOMEBREW_FORMULA])?;

    // 5. Shell installer
    let install_sh = fs::read_to_string(INSTALL_SH).map_err(|e| format!("{INSTALL_SH}: {e}"))?;
    let expected_default = format!("FEMACS_VERSION=\"${{FEMACS_VERSION:-{version}}}\"");
    if !install_sh.contains(&expected_default) {
        return Err(format!("{INSTALL_SH}: expected `{expected_default}`"));
    }
    run_checked("sh", &["-n", INSTALL_SH])?;

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
    verify_distribution_artifacts(&version)?;

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

fn collect_release_asset_paths(version: &str, assets_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let sums = assets_dir.join("SHA256SUMS.txt");
    if !sums.is_file() {
        return Err(format!("missing checksums file at {}", sums.display()));
    }
    let raw = fs::read_to_string(&sums).map_err(|e| format!("{}: {e}", sums.display()))?;
    let map = parse_checksums(&raw)?;
    let mut files = Vec::new();
    for target in DIST_TARGETS {
        let name = format!("femacs-v{version}-{}.tar.gz", target.triple);
        if map.contains_key(&name) {
            let p = assets_dir.join(&name);
            if !p.is_file() {
                return Err(format!(
                    "archive `{name}` listed in SHA256SUMS.txt is missing under {}",
                    assets_dir.display()
                ));
            }
            files.push(p);
        }
    }
    if files.is_empty() {
        return Err(format!(
            "no archives matching `femacs-v{version}-<triple>.tar.gz` in {}",
            sums.display()
        ));
    }
    files.push(sums);
    Ok(files)
}

fn cmd_publish(args: &[String]) -> Result<(), String> {
    let version = require_version(args)?;
    let execute = has_flag(args, "--execute");
    let skip_already_published = has_flag(args, "--skip-already-published");
    let skip_tag = has_flag(args, "--skip-tag");
    let skip_github = has_flag(args, "--skip-github");
    let with_npm = has_flag(args, "--with-npm");
    let assets_dir = parse_flag_value(args, "--assets-dir")?.map(PathBuf::from);
    let homebrew_tap_dir = parse_flag_value(args, "--homebrew-tap-dir")?.map(PathBuf::from);
    let notes_file_arg = parse_flag_value(args, "--notes-file")?;
    let wait_timeout: u64 = match parse_flag_value(args, "--wait-timeout-secs")? {
        Some(raw) => raw
            .parse::<u64>()
            .map_err(|e| format!("invalid --wait-timeout-secs `{raw}`: {e}"))?,
        None => 180,
    };

    verify_manifests_and_readmes(&version)?;
    verify_distribution_artifacts(&version)?;

    let asset_paths = match assets_dir.as_deref() {
        Some(dir) => Some(collect_release_asset_paths(&version, dir)?),
        None => None,
    };

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
        if let Some(ref paths) = asset_paths {
            println!("  - upload {} release assets (incl. SHA256SUMS.txt)", paths.len());
        }
        if with_npm {
            println!("  - npm publish --access public ({NPM_PACKAGE_DIR})");
        }
        if let Some(ref tap_dir) = homebrew_tap_dir {
            println!(
                "  - sync {HOMEBREW_FORMULA} -> {}/Formula/femacs.rb, git commit -S & push",
                tap_dir.display()
            );
        }
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
            let mut gh_args: Vec<String> = vec![
                "release".to_string(),
                "create".to_string(),
                tag.clone(),
                "--title".to_string(),
                title,
                "--notes-file".to_string(),
                notes_str.to_string(),
                "--verify-tag".to_string(),
            ];
            if let Some(ref paths) = asset_paths {
                for p in paths {
                    gh_args.push(p.to_str().ok_or("non-utf8 asset path")?.to_string());
                }
            }
            let gh_refs: Vec<&str> = gh_args.iter().map(String::as_str).collect();
            run_checked("gh", &gh_refs)?;
            run_checked(
                "gh",
                &["release", "view", &tag, "--json", "url,tagName,name"],
            )?;
        }

        if with_npm {
            let npm_dir = Path::new(NPM_PACKAGE_DIR);
            run_checked_in(
                npm_dir,
                "npm",
                &["install", "--ignore-scripts", "--no-audit", "--no-fund"],
            )?;
            run_checked_in(npm_dir, "npm", &["test"])?;
            run_checked_in(npm_dir, "npm", &["publish", "--access", "public"])?;
        }

        if let Some(ref tap_dir) = homebrew_tap_dir {
            if !git_status_clean_in(tap_dir)? {
                return Err(format!(
                    "homebrew tap directory {} is dirty",
                    tap_dir.display()
                ));
            }
            let dest_formula = tap_dir.join("Formula/femacs.rb");
            if let Some(parent) = dest_formula.parent() {
                fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
            }
            fs::copy(HOMEBREW_FORMULA, &dest_formula)
                .map_err(|e| format!("copy formula to {}: {e}", dest_formula.display()))?;
            run_checked_in(tap_dir, "git", &["add", "Formula/femacs.rb"])?;
            if !git_status_clean_in(tap_dir)? {
                let commit_msg = format!("feat(femacs): bump formula to v{version}\n");
                let msg_path = Path::new(TMP_REL_DIR).join("tap-commit.txt");
                fs::write(&msg_path, commit_msg).map_err(|e| format!("write tap msg: {e}"))?;
                let abs_msg = fs::canonicalize(&msg_path)
                    .map_err(|e| format!("canonicalize tap msg: {e}"))?;
                let msg_str = abs_msg.to_str().ok_or("non-utf8 path")?;
                run_checked_in(tap_dir, "git", &["commit", "-S", "-F", msg_str])?;
                run_checked_in(tap_dir, "git", &["push", "origin", "master"])?;
            }
        }
        Ok(())
    })();
    let _ = fs::remove_dir_all(TMP_REL_DIR);
    res
}

fn print_usage() {
    println!(
        "usage: xtask release <plan|bump|sync-dist|check|notes|publish> --version <X.Y.Z> [options]\n\
         \n\
         Subcommands:\n\
         \x20 plan      --version <X.Y.Z>                       Show crate & package versions + crates.io status\n\
         \x20 bump      --version <X.Y.Z> [--dry-run]           Bump 5 Cargo manifests + npm package.json + docs/install.sh\n\
         \x20 sync-dist --version <X.Y.Z> --checksums <FILE>    Regenerate npm manifest.json & Homebrew femacs.rb\n\
         \x20 check     --version <X.Y.Z> [--allow-dirty] [--skip-package]\n\
         \x20 notes     --version <X.Y.Z> [--out <FILE>] [--already-published a,b]\n\
         \x20 publish   --version <X.Y.Z> [--execute] [--skip-already-published] [--skip-tag] [--skip-github] [--assets-dir <DIR>] [--with-npm] [--homebrew-tap-dir <DIR>] [--notes-file <FILE>] [--wait-timeout-secs N]"
    );
}

/// Dispatch `cargo run -p xtask -- release ...`.
pub fn command(args: &[String]) -> i32 {
    let sub = args.first().map(String::as_str).unwrap_or("help");
    let rest = if args.len() > 1 { &args[1..] } else { &[] };
    let res = match sub {
        "plan" => cmd_plan(rest),
        "bump" => cmd_bump(rest),
        "sync-dist" => cmd_sync_dist(rest),
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
