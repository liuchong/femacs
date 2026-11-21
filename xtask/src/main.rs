//! femacs build and gate driver.
//!
//! Orchestrates Cargo workspace builds (`femacs-pure`, `femacs-sys`, `femacs-vm`,
//! `femacs-host`, `femacs-tools`), generates target-local configuration mirrors and
//! layout assertions from the frozen baselines, and runs the mechanical gates.
//!
//! `cargo run -p xtask -- <command>`, with the femacs directory as cwd.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

mod bundle;
mod cargo_build;
mod tls;
mod image;
mod lcms;
mod treesit;
mod check_units;
mod macos;
mod owned_copy;
mod standalone;
mod manuals;
mod probes;
mod product_sources;
mod runtime;
mod android;
pub(crate) mod tooling;
#[cfg(test)]
mod test_support;

const REPO: &str = ".";

/// The default build tree.  `FEMACS_BUILD_ROOT` moves every derived directory
/// (``, `load/`, `port/`, `profiles/`) together, which is what keeps the PGO
/// phases from sharing an object or a dump with the default O2 build.
fn build_root () -> PathBuf
{
  std::env::var_os ("FEMACS_BUILD_ROOT").map (PathBuf::from)
    .unwrap_or_else (|| PathBuf::from ("build"))
}

fn units_dir () -> PathBuf {
  std::env::var_os("FEMACS_UNITS_DIR").map(PathBuf::from)
    .unwrap_or_else(|| build_root().join("femacs"))
}
fn load_dir () -> PathBuf { build_root ().join ("load") }
fn port_dir () -> PathBuf { build_root ().join ("port") }

/// `FEMACS_PGO=off` (default) | `generate:<dir>` | `use:<file>`.
enum PgoMode { Off, Generate (PathBuf), Use (PathBuf) }

fn pgo_mode () -> PgoMode
{
  match std::env::var ("FEMACS_PGO")
    {
      Ok (v) if v.starts_with ("generate:") =>
        PgoMode::Generate (PathBuf::from (&v["generate:".len ()..])),
      Ok (v) if v.starts_with ("use:") =>
        PgoMode::Use (PathBuf::from (&v["use:".len ()..])),
      Ok (v) if v.is_empty () || v == "off" => PgoMode::Off,
      Ok (v) =>
        {
          eprintln! ("error: FEMACS_PGO must be off, generate:<dir> or use:<file>, not {v}");
          exit (2);
        }
      Err (_) => PgoMode::Off,
    }
}

fn pgo_codegen_flags () -> Vec<String>
{
  match pgo_mode ()
    {
      PgoMode::Off => Vec::new (),
      PgoMode::Generate (dir) =>
        vec!["-C".to_string (), format! ("profile-generate={}", dir.display ())],
      PgoMode::Use (file) =>
        vec!["-C".to_string (), format! ("profile-use={}", file.display ())],
    }
}

/// Send a child's runtime profiles to a role-specific directory.  Only `profiles/train`
/// is ever merged; gate and dump runs still execute an instrumented binary and must not
/// contaminate the training set.
fn set_profile_env (cmd: &mut Command, role: &str)
{
  if let PgoMode::Generate (_) = pgo_mode ()
    {
      let dir = build_root ().join ("profiles").join (role);
      let _ = fs::create_dir_all (&dir);
      cmd.env ("LLVM_PROFILE_FILE", dir.join ("%p-%m.profraw"));
    }
}

/// The directories a ported unit can live in.  `docs/port-map.md` pairs every reference file
/// with a target path, and the C tree's non-`src/` directories keep their own names (PORTING.md
/// 0.1: `lib/`, `lib-src/`, `lwlib/`, `oldXMenu/`, `exec/`, `nt/`, `msdos/`), so the unit
/// lookup and the whole-tree walks have to cover all of them.
const UNIT_DIRS: &[&str] = &["src", "lib", "lib-src", "lwlib", "oldXMenu", "exec", "nt", "msdos"];

fn main() {
    std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap()).expect("workspace root");
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let code = match cmd {
        "build-macos" => macos::build_macos(),
        "build-terminal" => standalone::build_terminal(),
        "runtime-build" => runtime::command(&args[1..]),
        "build-manuals" => manuals::command(&args[1..]),
        "audit-probe" => probes::command(&args[1..]),
        "android" => android::command(&args[1..]),
        "gate-platforms" => android::platforms(&args[1..]),
        "generate-resources" => {
            if args.len() != 3 || args[1] != "--program" {
                fail("generate-resources requires --program <explicit-owned-program>")
            } else {
                run_python(&["tools/bootstrap-resources.py", "--family", "all", "--program", &args[2]])
            }
        },
        "link-macos" => macos::link_macos(),
        "package-macos" => macos::package_macos(),
        "gen-config" => gen_config(),
        "gen-globals" => gen_globals(),
        "layout-asserts" => match layout_asserts() {
            Ok(p) => {
                println!("wrote {}", p.display());
                0
            }
            Err(e) => fail(e),
        },
        "layout-check" => layout_check(),
        "check-units" => check_units::run(args.get(1)),
        "probe" => probe_build(&args[1..]),
        "gate-fast" => gate_fast(),
        "gate-probes" => gate_probes(),
        "gate-probe-programs" => match probe_programs(){Ok(())=>0,Err(error)=>fail(error)},
        "gate-load" => cargo_build::link_temacs(args.get(1).map(String::as_str)),
        "link-owned" => cargo_build::link_temacs(Some("--owned")),
        // Android links `libemacs.so` instead of a program (src/Makefile.in 727-755), so the
        // object set needs a *shared* link: same units, no `main`.
        "link-shared" => {
            let manifest=args.iter().position(|a|a=="--units-manifest").and_then(|i|args.get(i+1));
            if args.iter().any(|a|a=="--units-manifest") && manifest.is_none(){fail("--units-manifest needs core-units.txt")}
            else{link_shared(args.get(1).map(String::as_str),manifest.map(Path::new))}
        },
        "link-program" => cargo_build::build_tool_program(args.get(1)),
        "dump" => dump_emacs(),
        "dump-bootstrap" => dump_emacs_mode(true),
        "gate-android" => run_python(&["tools/check-android-apk.py"]),
        "gate-full" => {
            eprintln!(
                "gate-full needs the reference configured, built and its ERT suite \
                 runnable; it is not available until the port reaches the loading ladder \
                 (docs/port-plan.md section 7). Nothing was checked."
            );
            2
        }
        _ => {
            println!(
                "usage: xtask <build-terminal|build-macos|runtime-build --product terminal|cocoa --variant baseline|native|collector|native-collector --out build/runtime/<name> [--observer] [--test-hooks] [--backend-prefix <prefix>]|generate-resources --program <owned-program>|link-owned|gen-config|layout-asserts|layout-check|\
                 check-units [list]|gate-fast|gate-probes|gate-probe-programs|gate-load [--report]|\
                 link-program <tool>|dump|gate-full>"
            );
            0
        }
    };
    exit(code);
}

/// Extra rustc flags for a *diagnostic* build, taken from `FEMACS_SANITIZER=address`.
///
/// L5's crashes are heap corruption inside `malloc`, so the useful instrument is one that
/// names the *writing* site rather than the victim frame (PORTING.md P-112/P-113).  ASan is
/// that instrument: it needs its flag on the unit compile *and* on the link, and it needs
/// `RUSTC_BOOTSTRAP=1` because `-Zsanitizer` is a nightly flag.  A normal build is unchanged -
/// nothing is added unless the variable is set - and the sanitized objects go to the same
/// `build/femacs/` as always, so the round is the same round.
fn sanitizer_flags () -> Vec<String>
{
  match std::env::var ("FEMACS_SANITIZER")
    {
      Ok (v) if ! v.is_empty () => vec! ["-Z".to_string (), format! ("sanitizer={v}")],
      _ => Vec::new (),
    }
}

/// `--cfg feature="…"` items for the frozen configuration named by `FEMACS_CONFIG`.
///
/// One per macro the JSON calls true, lower-cased (`HAVE_ANDROID` -> `have_android`), because that
/// is how the port spells C's `#ifdef`s; plus the `not_*` twins of the negative macros the sources
/// test that way (`ANDROID_STUBIFY` undef -> `not_android_stubify`, which 1,940 sites read).
/// A missing `FEMACS_CONFIG` yields no cfgs at all, which is the darwin default and stays correct
/// for darwin: its minimal configuration defines none of these.
fn config_features () -> Result<Vec<String>,String>
{
  let Some(path)=android::config_source(None)? else{return Ok(Vec::new());};
  let stem=path.display().to_string();
  let text=fs::read_to_string(&path).map_err(|e|e.to_string())?;
  let data:serde_json::Value=serde_json::from_str(&text).map_err(|e|e.to_string())?;
  let mut out: Vec<String> = Vec::new ();
  let mut numbers: std::collections::HashMap<String, u64> = std::collections::HashMap::new ();
  // The names the configuration calls true, for the conditions that combine keys (`aligned_alloc`
  // below is `HAVE_ALIGNED_ALLOC || (!SYSTEM_MALLOC && !DOUG_LEA_MALLOC)`, then
  // `HAVE_POSIX_MEMALIGN`).
  let mut truthy: std::collections::HashSet<String> = std::collections::HashSet::new ();
  let mut values=std::collections::BTreeMap::<String,String>::new();
  fn equivalent_setting(value: &str) -> &str {
    match value {
      "1" | "true" => "1",
      "0" | "false" | "undefined" | "absent" => "0",
      other => other,
    }
  }
  for section in ["macros","features"] {
    if let Some(input)=data.get(section) {
      let items=input.as_object().ok_or_else(||format!("configuration {section} must be an object"))?;
      for (name,value) in items {
        let name=name.to_ascii_uppercase();
        let value=value.as_str().map(str::to_owned).unwrap_or_else(||value.to_string());
        if let Some(previous)=values.insert(name.clone(),value.clone()) {
          if equivalent_setting(&previous)!=equivalent_setting(&value) {
            return Err(format!("conflicting configuration setting: {name}"));
          }
        }
      }
    }
  }
  for (name,value) in values {
    // This optional C string is tested with #ifdef, not #if on its value.
    let truthy_value=matches!(value.as_str(),"1"|"true")
      || (name == "MODULES_SECONDARY_SUFFIX" && c_string_value(&value).is_some());
    let name_lower=name.to_ascii_lowercase();
    if let Ok(n)=value.parse::<u64>(){numbers.insert(name.clone(),n);}
    if truthy_value {truthy.insert(name.clone());}
    out.push(format!("feature=\"{}{}\"",if truthy_value{""}else{"not_"},name_lower));
  }
  // C computes `HAVE_LOAD_EXTENSION` from
  // `HAVE_SQLITE3_LOAD_EXTENSION && SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION`
  // (sqlite.c 39-44).  The header macro is mirrored as a constant inside the
  // sqlite unit (measured 1005), so it is never a configuration key: deriving
  // the second conjunct here keeps one source of truth and avoids a duplicate
  // constant in the generated configuration.
  if truthy.contains("HAVE_SQLITE3_LOAD_EXTENSION") {
    out.push("feature=\"sqlite3_dbconfig_enable_load_extension\"".into());
  }
  // The selected SDK binding supplies the grammar ABI; a generic truthy
  // macro cannot express this comparison from the source dispatch shim.
  if truthy.contains("HAVE_TREE_SITTER") {
    let version = numbers.get("FEMACS_TREE_SITTER_LANGUAGE_VERSION")
      .ok_or("Tree-sitter configuration lacks a bound SDK grammar ABI")?;
    if *version >= 15 { out.push("feature=\"treesit_language_version_ge_15\"".into()); }
  }
  // C's `#if (defined HAVE_ALIGNED_ALLOC || (!defined SYSTEM_MALLOC && !defined DOUG_LEA_MALLOC))`
  // takes the platform's `aligned_alloc`; when that is false and `HAVE_POSIX_MEMALIGN` is set, C's
  // `#elif` defines its own over `posix_memalign` (`alloc.c` 969-988).  bionic at API 21 has no
  // `aligned_alloc` at all, so the second arm is the Android one - derived here because the
  // condition combines keys rather than naming one (status 4.713).
  let platform_aligned_alloc = truthy.contains ("HAVE_ALIGNED_ALLOC")
    || (!truthy.contains ("SYSTEM_MALLOC") && !truthy.contains ("DOUG_LEA_MALLOC"));
  if !platform_aligned_alloc && truthy.contains ("HAVE_POSIX_MEMALIGN")
    {
      out.push ("feature=\"my_aligned_alloc\"".to_string ());
    }
  // `systhread.rs` gates the "alignment room" unions on `alignof_pthread_cond_t_le_…`, which is a
  // *comparison* of two measured numbers, not a macro: the JSON carries
  // `ALIGNOF_PTHREAD_COND_T` and `ALIGNOF_MAX_ALIGN_T` and nothing derived the boolean.  With it
  // always false, Android took the union arm and `thread_state` lost `Clone`/`Copy` and its layout
  // (status 4.616).  Derived here so both targets agree with their own measurement.
  // C's `USABLE_POSIX_SPAWN` (`callproc.c` 37-50) is a *derived* condition rather than a macro:
  // `<spawn.h>`, `posix_spawn`, `posix_spawnattr_setflags`, one of the two `..._addchdir`
  // spellings, `HAVE_DECL_POSIX_SPAWN_SETSID == 1`, and not Haiku.  The Android configuration
  // answers no (bionic's `<spawn.h>` has the first two and none of the rest), and every unit that
  // spelled the arm by hand compiled it anyway (status 4.749).  Both polarities are emitted, the
  // way a macro's are, because the frozen target sets no features at all.
  let usable_posix_spawn = truthy.contains ("HAVE_SPAWN_H")
    && truthy.contains ("HAVE_POSIX_SPAWN")
    && truthy.contains ("HAVE_POSIX_SPAWNATTR_SETFLAGS")
    && (truthy.contains ("HAVE_POSIX_SPAWN_FILE_ACTIONS_ADDCHDIR")
        || truthy.contains ("HAVE_POSIX_SPAWN_FILE_ACTIONS_ADDCHDIR_NP"))
    && truthy.contains ("HAVE_DECL_POSIX_SPAWN_SETSID")
    && !truthy.contains ("HAVE_HAIKU");
  out.push (format! ("feature=\"{}\"", if usable_posix_spawn { "usable_posix_spawn" }
                     else { "not_usable_posix_spawn" }));
  // frame.h's `menu_bar_window` (C 249) and `tool_bar_window`, and dispnew.c's dummy-window
  // creation, are under `HAVE_WINDOW_SYSTEM && !HAVE_EXT_*_BAR`.  The frozen table carries the two
  // macros but nothing derived the combination, so `window_system_no_ext_menu_bar` was never
  // defined: the Android frame never got its menu-bar window while `FRAME_MENU_BAR_LINES` was 1,
  // and `display_menu_bar` dereferenced a nil `menu_bar_window` (status 4.908, tombstone
  // `docs/baseline/android-tombstone-gui.txt`).
  let window_system_no_ext_menu_bar = truthy.contains ("HAVE_WINDOW_SYSTEM")
    && !truthy.contains ("HAVE_EXT_MENU_BAR");
  out.push (format! ("feature=\"{}\"", if window_system_no_ext_menu_bar
                     { "window_system_no_ext_menu_bar" }
                     else { "not_window_system_no_ext_menu_bar" }));
  let window_system_no_ext_tool_bar = truthy.contains ("HAVE_WINDOW_SYSTEM")
    && !truthy.contains ("HAVE_EXT_TOOL_BAR");
  out.push (format! ("feature=\"{}\"", if window_system_no_ext_tool_bar
                     { "window_system_no_ext_tool_bar" }
                     else { "not_window_system_no_ext_tool_bar" }));

  // C defines `USE_ANDROID_ASSETS` inside the units that read it rather than in `config.h`
  // (`lread.c` 100-157, `doc.c` 40-70): `HAVE_ANDROID && !ANDROID_STUBIFY && __ANDROID_API__ >= 9`.
  // The frozen configuration carries no macro of that name, so nothing derived a cfg for it, both
  // units compiled their plain-file arms, and the device asked for `${ANDROID_ASSET_ROOT}/lisp/loadup.el` as a
  // real path: ENOENT, then the fatal exit measured at status 4.786.  Both polarities are emitted,
  // the way a macro's are.
  let use_android_assets = truthy.contains ("HAVE_ANDROID")
    && !truthy.contains ("ANDROID_STUBIFY")
    && truthy.contains ("ANDROID_API_GE_9");
  out.push (format! ("feature=\"{}\"", if use_android_assets { "use_android_assets" }
                     else { "not_use_android_assets" }));
  // gnulib's generated `<stdlib.h>` rewrites the allocator names it cannot trust, and the frozen
  // table carries the answer for this target: `HAVE_REALLOC_POSIX` is 1 while
  // `HAVE_REALLOC_0_NONNULL` is undefined, so `@REPLACE_REALLOC_FOR_REALLOC_POSIX@` is nonzero and
  // `lib/stdlib.in.h` 1461-1482 defines `_GL_INLINE_RPL_REALLOC`, then `#undef realloc` and
  // `#define realloc rpl_realloc`, whose body is `realloc (ptr, size ? size : 1)`.  A configuration
  // that does not state `HAVE_REALLOC_POSIX` at all (the minimal darwin table) keeps the plain
  // binding.  The device measured why the difference matters: bionic's `realloc (p, 0)` on a live
  // block returns NULL, so `shrink_regexp_cache`'s `xrealloc (cp->buf.buffer, cp->buf.used)`
  // reached `memory_full (0)` in the first GC of `loadup.el` (status 4.787).
  let replace_realloc_for_realloc_posix = truthy.contains ("HAVE_REALLOC_POSIX")
    && !truthy.contains ("HAVE_REALLOC_0_NONNULL");
  out.push (format! ("feature=\"{}\"", if replace_realloc_for_realloc_posix
                     { "replace_realloc_for_realloc_posix" }
                     else { "not_replace_realloc_for_realloc_posix" }));
  // The same two facts decide gnulib's own spelling of the condition, which `lib/realloc.rs`
  // reads: `# if !HAVE_REALLOC_0_NONNULL` is what turns a zero request into one byte.
  out.push (format! ("feature=\"{}\"", if replace_realloc_for_realloc_posix
                     { "no_realloc_0_nonnull" } else { "realloc_0_nonnull" }));
  for (small, big, feature) in
    [("ALIGNOF_PTHREAD_MUTEX_T", "ALIGNOF_MAX_ALIGN_T",
      "alignof_pthread_mutex_t_le_alignof_max_align_t"),
     ("ALIGNOF_PTHREAD_COND_T", "ALIGNOF_MAX_ALIGN_T",
      "alignof_pthread_cond_t_le_alignof_max_align_t")]
    {
      if let (Some (&a), Some (&b)) = (numbers.get (small), numbers.get (big))
        {
          if a <= b { out.push (format! ("feature=\"{feature}\"")); }
        }
    }
  // nsgui.h defines NoValue even though its numeric value is zero.
  if truthy.contains("HAVE_NS") {
    out.push("feature=\"no_value\"".to_string());
    out.push("feature=\"window_system\"".to_string());
  }
  out.sort ();
  out.dedup ();
  if !out.is_empty ()
    {
      println! ("config: {} cfg feature(s) from {stem}", out.len ());
    }
  Ok(out)
}

/// `rustc`, with `RUSTC_BOOTSTRAP=1` when the sanitizer is on, so the nightly flag is accepted
/// by the stable toolchain the repository pins (`rust-toolchain.toml`).
fn rustc_cmd () -> std::process::Command
{
  let mut c = std::process::Command::new ("rustc");
  if ! sanitizer_flags ().is_empty ()
    {
      c.env ("RUSTC_BOOTSTRAP", "1");
    }
  c
}

fn run_python(args: &[&str]) -> i32 {
    match std::process::Command::new("python3").current_dir(REPO).args(args).status() {
        Ok(s) if s.success() => 0,
        Ok(_) => 1,
        Err(e) => fail(e),
    }
}

fn fail(msg: impl std::fmt::Display) -> i32 {
    eprintln!("error: {msg}");
    1
}

// --- configuration ---------------------------------------------------------

/// Minimal reader for the frozen configuration files written by
/// tools/gen-config-baseline (a flat JSON object with a "macros" object).
fn read_config(path: &Path) -> BTreeMap<String, String> {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut out = BTreeMap::new();
    let mut in_macros = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("\"macros\"") {
            in_macros = true;
            continue;
        }
        // The `macros` block closes with `},` - a trailing comma - so `t == "}"` never matched and
        // the reader ran on into the `features` section, turning 781 cfg gates into constants.
        // That is the 809-constant `config.rs` behind the 48 `_GL_EXTERN_INLINE` duplicates
        // (status 4.638).
        if in_macros && t.trim_end_matches(',').trim() == "}" {
            break;
        }
        if in_macros {
            if let Some((k, v)) = t.split_once(':') {
                let k = k.trim().trim_matches('"').to_string();
                // A quoted value is a JSON *string*: its escapes have to come off, or the
                // reader hands the rest of the program the JSON-escaped text instead of the
                // macro body.  `SYSTEM_TYPE` is the entry where it showed: the file's body is
                // `"darwin"`, this reader used to deliver `\"darwin\`, and the generated
                // constant was a `&str` holding `"darwin"` *with* its C quotes - not the value
                // `intern_c_string` needs (docs/status.md 3.64).
                let raw = v.trim().trim_end_matches(',');
                let v = if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
                    json_unescape(&raw[1..raw.len() - 1])
                        .unwrap_or_else(|| raw.trim_matches('"').to_string())
                } else {
                    raw.trim().to_string()
                };
                out.insert(k, v);
            }
        }
    }
    out
}

/// The value-bearing macros C reads as C *strings* but the frozen JSON keeps in its `features`
/// object, because that object is the whole `config.h` (status 4.881).
///
/// `config.h` defines `EMACS_CONFIGURATION`, `EMACS_CONFIG_FEATURES` and `EMACS_CONFIG_OPTIONS`
/// per target, and `emacs.c` passes each to `build_string`; `docs/baseline/config/*.json` stores
/// them (with their C quotes) under `features`, where `gen-config` turned them into `cfg` gates -
/// so their *values* never reached the port and `emacs.rs` hard-coded the frozen target's.  The
/// Android build then reported `aarch64-apple-darwin…` in `(emacs-version)` (measured on the
/// device, docs/baseline/android-batch-regex.txt).  This reader returns exactly those three, and
/// only when the JSON carries a quoted string for them.
fn read_feature_strings(path: &Path) -> BTreeMap<String, String> {
    const NAMES: [&str; 3] =
        ["EMACS_CONFIGURATION", "EMACS_CONFIG_FEATURES", "EMACS_CONFIG_OPTIONS"];
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut out = BTreeMap::new();
    let mut in_features = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("\"features\"") { in_features = true; continue; }
        if in_features && t.trim_end_matches(',').trim() == "}" { break; }
        if !in_features { continue; }
        if let Some((k, v)) = t.split_once(':') {
            let k = k.trim().trim_matches('"');
            if !NAMES.contains(&k) { continue; }
            let raw = v.trim().trim_end_matches(',').trim();
            if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2
                && let Some(value) = json_unescape(&raw[1..raw.len() - 1])
                { out.insert(k.to_string(), value); }
        }
    }
    out
}

fn gen_config() -> i32 {
    let dir = Path::new(REPO).join("docs/baseline/config");
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    // The frozen configuration is the *`.json`* in that directory.  Without the filter this
    // took the first name alphabetically, and `docs/baseline/config/cond-eval-verified.txt`
    // sorts before `darwin-arm64.json` - so a regeneration wrote a three-line `config.rs`
    // and silently dropped every constant (caught while adding the `SYSTEM_TYPE` decoding
    // below, docs/status.md 3.64; the file is generated, so the mistake was one command away
    // from being committed).
    entries.retain(|p| p.extension().is_some_and(|e| e == "json"));
    entries.sort();
    // One configuration today (`darwin-arm64`).  The moment a second one lands - an Android or
    // Linux target for the platform rows - "the first file alphabetically" would silently
    // become the port's whole configuration, which is the same class of mistake the `.json`
    // filter above fixed.  So the choice is explicit: `FEMACS_CONFIG=<stem>` names one, and
    // more than one candidate without it is an error rather than a coin flip (P-203).
    let target=match android::config_source(None) {
        Ok(Some(path))=>path,
        Ok(None)=>{if entries.len()!=1{return fail("select one explicit frozen configuration");}entries.remove(0)},
        Err(e)=>return fail(e),
    };
    println!("config: {}", target.display());
    let mut macros = read_config(&target);
    // P-346: native products bind source/config/backend identity into their
    // cache ABI; old non-runtime configurations retain a named legacy value.
    macros.entry("FEMACS_RUNTIME_ABI_ID".into()).or_insert_with(|| "\"legacy\"".into());
    for key in ["FEMACS_NATIVE_SDKROOT", "FEMACS_NATIVE_ASSEMBLER_DIR"] {
        macros.entry(key.into()).or_insert_with(|| "\"\"".into());
    }
    // No inner attributes: this file is textually included by every translation
    // unit, and inner attributes are only legal at the top of a module.
    let mut rs = String::from(
        "// Generated by `xtask gen-config` from the frozen configuration; do not edit.\n\
         // Source: docs/baseline/config/*.json (PORTING.md P-08).\n\n",
    );
    for (k, v) in &macros {
        let line = match v.as_str() {
            "absent" => continue,                   // not in config.h at all
            "undefined" => format!("pub const {k}: bool = false; // #undef in config.h\n"),
            "1" | "true" => format!("pub const {k}: bool = true;\n"),
            "0" | "false" => format!("pub const {k}: bool = false;\n"),
            other => {
                if let Ok(n) = other.parse::<i64>() {
                    format!("pub const {k}: i64 = {n};\n")
                } else if let Some(value) = c_string_value(other) {
                    // A `#define NAME "text"` in config.h is a *C string*, and the C code that
                    // reads it passes it to a `char *`/`const char *` parameter
                    // (`intern_c_string (SYSTEM_TYPE)` in emacs.c 3514).  Emitting the macro
                    // *body* as a Rust `&str` (`"\"darwin\""`, whose value still carries the C
                    // quotes) was the mirror defect print's neighbour found in docs/status.md
                    // 3.64; the constant is a `&CStr` of the decoded value instead, the shape
                    // emacs.rs already uses for its other config.h strings (`PATH_EXEC`).
                    format!("pub const {k}: &core::ffi::CStr = c{}; // C string literal\n",
                            c_literal(&value))
                } else {
                    format!("pub const {k}: &str = {};\n", rust_str(other))
                }
            }
        };
        rs.push_str(&line);
    }
    // The three target strings `emacs.c` reads (see `read_feature_strings`): emitted as `&CStr`
    // exactly like the string macros above, so a unit can hand them to `build_string` unchanged.
    for (k, v) in read_feature_strings (&target) {
        // The JSON stores the macro *body* (`"android"`, with its C quotes), exactly as the
        // `macros` reader above hands it to `c_string_value`.
        let Some (body) = c_string_value (&v) else { continue };
        rs.push_str (&format! ("pub const {k}: &core::ffi::CStr = c{}; // C string literal\n",
                              c_literal (&body)));
    }
    let out = std::env::var_os("FEMACS_CONFIG_RS").map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("src/include/config.rs"));
    fs::create_dir_all(out.parent().unwrap()).ok();
    if let Err(e) = fs::write(&out, rs) {
        return fail(e);
    }
    println!("wrote {} from {}", out.display(), target.display());
    0
}

/// A JSON string's escapes, decoded.  `None` for a form this reader does not spell (then the
/// caller keeps the text as written rather than guessing).
fn json_unescape(s: &str) -> Option<String> {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            '/' => out.push('/'),
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            'b' => out.push('\u{8}'),
            'f' => out.push('\u{c}'),
            _ => return None,
        }
    }
    Some(out)
}

/// The value of a C string literal written as its `#define` body, or None when the text is not
/// one (or uses an escape this generator does not spell - then the body is kept verbatim rather
/// than guessed at).
fn c_string_value(body: &str) -> Option<String> {
    let inner = body.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            '0' => out.push('\0'),
            _ => return None,
        }
    }
    Some(out)
}

/// A Rust `c"..."` literal for a value that has no NUL in it (the C string's terminator is the
/// literal's own), escaping what the literal grammar needs.
fn c_literal(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn rust_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Emit src/include/globals.rs from the frozen symbol table.  In the reference
/// this file comes from make-docfile (`#define iQfoo N`, `DEFINE_LISP_SYMBOL`);
/// the port declares each Q symbol as a static initialised at startup, because
/// Rust cannot fold an array address into a constant (PORTING.md P-17).
fn gen_globals() -> i32 {
    let tsv = Path::new(REPO).join("docs/baseline/globals/symbols.tsv");
    let text = match fs::read_to_string(&tsv) {
        Ok(t) => t,
        Err(e) => return fail(format!("{}: {e}", tsv.display())),
    };
    let mut rows = Vec::new();
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() >= 3 {
            rows.push((f[0].to_string(), f[1].to_string()));
        }
    }
    let mut rs = String::from(
        "// Generated by `xtask gen-globals` from docs/baseline/globals/symbols.tsv;\n\
         // do not edit.  The reference generates globals.h with make-docfile\n\
         // (`#define iQfoo N` and `DEFINE_LISP_SYMBOL (Qfoo)`), which makes every Q\n\
         // symbol a compile-time constant.  Rust cannot fold the symbol array\n\
         // address into a constant, so the port defines an inline function per\n\
         // symbol: same name, same value, call shape `Qnil()` (PORTING.md P-17).\n\n",
    );
    for (index, name) in &rows {
        // The reference writes `#define iQfoo N` next to `DEFINE_LISP_SYMBOL (Qfoo)`;
        // the port keeps the constant so that derivations such as NIL_IS_ZERO stay
        // tied to the generated table (src/lisp.h 1999).
        rs.push_str(&format!("pub const i{name}: usize = {index};\n"));
        rs.push_str(&format!(
            "#[inline(always)]\npub fn {name} () -> Lisp_Object {{ builtin_lisp_symbol ({index}) }}\n"
        ));
    }
    rs.push_str(&format!(
        "\npub const lispsym_count: usize = {};\n\
         unsafe extern \"C\" {{ pub static mut lispsym: [Lisp_Symbol; lispsym_count]; }}\n",
        rows.len()
    ));
    // The other half of the generated globals: the V* variables that the DEFVAR_*
    // forms declare.  The reference keeps them inside struct emacs_globals
    // (`globals.f_Vfoo`) so a debugger can list them; the port declares one static
    // per variable, which keeps the reference's source shape (`Vfoo = x`) and
    // records the difference (PORTING.md P-21).
    let vars = Path::new(REPO).join("docs/baseline/globals/variables.tsv");
    let mut n_vars = 0usize;
    if let Ok(text) = fs::read_to_string(&vars) {
        // The reference keeps them inside struct emacs_globals (`globals.f_Vfoo`) so a
        // debugger can list them; the port declares each as an extern global in a
        // glob-imported module, and the unit that owns it defines it
        // `#[unsafe(no_mangle)] pub static mut` — a definition shadows the glob
        // import, exactly as a DEFUN shadows its generated prototype (PORTING.md
        // 0.1-10/0.1-3, P-21).  Defining them here instead would give every
        // translation unit its own NULL copy, which the first inlined reader of a
        // DEFVAR'd table would then dereference (review A, F4).
        rs.push_str("\n// --- variables declared by DEFVAR_* (generated) ---\n\
                     pub mod variables {\n    use super::*;\n    unsafe extern \"C\" {\n");
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 {
                continue;
            }
            let (name, kind) = (f[0], f[1]);
            let ty = match kind {
                "LISP_OBJECT" => "Lisp_Object",
                "EMACS_INTEGER" => "EMACS_INT",
                "BOOLEAN" => "bool",
                "KBOARD" => "*mut c_void",
                _ => continue,
            };
            rs.push_str(&format!("        pub static mut {name}: {ty}; // {}\n", f[2]));
            n_vars += 1;
        }
        rs.push_str("    }\n}\npub use self::variables::*;\n");
    }

    // The other generated half of the reference's globals.h: make-docfile writes
    // `EXFUN (Ffoo, maxargs)` for every DEFUN (lib-src/make-docfile.c 756), so every
    // translation unit sees a prototype for every Lisp-callable function.  Rust
    // cannot declare and define a function under one name in one module, so the
    // prototypes live in a module that the unit glob-imports: a definition in the
    // unit shadows the glob import, which is C's prototype-plus-definition pair
    // (PORTING.md P-17).  Signatures come from the frozen DEFUN table, so they are
    // the arities the reference compiler saw.
    let subrs = Path::new(REPO).join("docs/baseline/subrs.tsv");
    let mut n_subrs = 0usize;
    if let Ok(text) = fs::read_to_string(&subrs) {
        // One prototype per C name, like globals.h: a name may be defined in more
        // than one configuration branch.  make-docfile refuses two definitions with
        // differing signatures, so the generator refuses them too; the attribute
        // flags are OR'ed (the reference keeps the flags of one record, the port
        // keeps every flag it has seen).
        let mut protos: std::collections::BTreeMap<String, (String, bool, String)> =
            std::collections::BTreeMap::new();
        let mut clashes = 0usize;
        for line in text.lines().skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 7 {
                continue;
            }
            // make-docfile prefixes `_Noreturn` when the DEFUN lists the attribute;
            // Rust's spelling of "never returns" is the return type `!`.
            let noreturn = f[6].split_whitespace().any(|w| w == "noreturn");
            let args = match f[4] {
                "MANY" => "nargs: ptrdiff_t, args: *mut Lisp_Object".to_string(),
                "UNEVALLED" => "args: Lisp_Object".to_string(),
                n => match n.parse::<usize>() {
                    // C's DEFUN_ARGS_n has no parameter names; Rust extern blocks
                    // require them, so the port uses the union member names.
                    Ok(k) => (0..k).map(|i| format!("a{i}: Lisp_Object"))
                        .collect::<Vec<_>>().join(", "),
                    Err(_) => continue,
                },
            };
            match protos.get(f[1]) {
                Some((seen, _, seen_max)) => {
                    if *seen != args || *seen_max != f[4] {
                        eprintln!("subr {}: signature {} vs {}", f[1], seen, args);
                        clashes += 1;
                    }
                }
                None => {
                    protos.insert(f[1].to_string(), (args, noreturn, f[4].to_string()));
                }
            }
            if noreturn {
                if let Some(e) = protos.get_mut(f[1]) {
                    e.1 = true;
                }
            }
        }
        rs.push_str(
            "\n// --- EXFUN prototypes for every DEFUN (generated from subrs.tsv) ---\n\
             // C: globals.h writes `EXFUN (Ffoo, maxargs)`; lisp.h includes <globals.h>,\n\
             // so the prototype is visible in every translation unit.\n\
             pub mod subrs {\n    use super::*;\n    unsafe extern \"C\" {\n",
        );
        for (name, (args, noreturn, _)) in &protos {
            // C's globals.h writes `_Noreturn` before the EXFUN; Rust spells that
            // "never returns" as the return type `!` (PORTING.md 5.2).
            let ret = if *noreturn { "!" } else { "Lisp_Object" };
            rs.push_str(&format!("        pub fn {name} ({args}) -> {ret};\n"));
            n_subrs += 1;
        }
        rs.push_str("    }\n}\npub use self::subrs::*;\n");
        if clashes != 0 {
            eprintln!("{clashes} subr name(s) defined twice with different signatures");
            return 1;
        }
    }

    let out = Path::new("src/include/globals.rs");
    fs::create_dir_all(out.parent().unwrap()).ok();
    if let Err(e) = fs::write(out, rs) {
        return fail(e);
    }
    println!("wrote {} symbols, {} variables and {} subr prototypes to {}", rows.len(), n_vars, n_subrs, out.display());
    0
}

// --- layout assertions -----------------------------------------------------

/// C field/type names that Rust spells with a raw identifier (PORTING.md 0.2).  The
/// assertions are generated Rust, so `Lisp_Buffer_Local_Value.where` has to come out as
/// `r#where` — without this the layout probe does not compile (P-65's follow-up).
fn rust_ident(name: &str) -> String {
    // The 2015 strict keywords plus the 2018/2021/2024 reserved ones (review B's F6): the
    // list has to be drawn from the language, not from the names met so far.  `box`, `gen`,
    // `try` and the rest are reserved and cannot be used bare in a generated field access.
    const KEYWORDS: [&str; 51] = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
        "abstract", "become", "box", "do", "final", "gen", "macro", "override", "priv",
        "try", "typeof", "unsized", "virtual",
    ];
    // These four have no raw form (`r#self` is rejected), so a C member spelled that way
    // needs a hand-written assertion rather than a silent mis-generation.
    const NOT_RAW: [&str; 4] = ["self", "Self", "crate", "super"];
    if NOT_RAW.contains(&name) {
        panic!("{name} cannot be a raw identifier in a generated layout assertion");
    }
    if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_string()
    }
}

fn rust_type_expr(kind: &str, key: &str) -> Option<String> {
    let path: Vec<&str> = key.split('.').collect();
    let ty = rust_ident(path[0]);
    match kind {
        "sizeof" => Some(format!("size_of::<{ty}>()")),
        "alignof" => Some(format!("align_of::<{ty}>()")),
        "offsetof" => {
            if path.len() < 2 {
                return None;
            }
            let field = path[1..].iter().map(|f| rust_ident(f))
                .collect::<Vec<_>>().join(".");
            Some(format!("offset_of!({ty}, {field})"))
        }
        _ => None,
    }
}

/// File-scope objects the mirrors declare, as name -> declared type text
/// (`pub static mut mpz: [mpz_t; 5];` -> `mpz` -> `[mpz_t; 5]`).  The layout keys of a
/// C *object* are resolved through this map instead of the type list.
fn declared_object_types() -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    let mut stack = vec![PathBuf::from("src/include"), PathBuf::from("src"),
                         PathBuf::from("lib/include"), PathBuf::from("../lib/include")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.filter_map(Result::ok) {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().map(|x| x != "rs").unwrap_or(true) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else { continue };
            let code = strip_comments_and_strings(&text);
            for line in code.lines() {
                let line = line.trim();
                let rest = match line.strip_prefix("pub static ") {
                    Some(r) => r,
                    None => match line.strip_prefix("static ") {
                        Some(r) => r,
                        None => continue,
                    },
                };
                let rest = rest.strip_prefix("mut ").unwrap_or(rest);
                let Some((name, tail)) = rest.split_once(':') else { continue };
                // strip the terminating `;` first: an array type contains one too
                // (`[mpz_t; 5];`), and splitting on it truncated the type.
                let ty = tail.trim_end().trim_end_matches(';').split('=').next()
                    .unwrap_or("").trim();
                if !name.trim().is_empty() && !ty.is_empty() {
                    map.insert(name.trim().to_string(), ty.to_string());
                }
            }
        }
    }
    map
}

/// Type names the port has already declared, so assertions are only emitted for
/// what exists; the rest are reported as pending (a shrinking number).
///
/// The scan is whitespace- and attribute-tolerant and does not anchor at the line
/// start: an earlier line-prefix scan dropped a type -- and with it every one of its
/// assertions -- when the declaration merely had different whitespace or attributes,
/// which turned the layout gate off silently (review B, F1).
fn known_types() -> std::collections::BTreeSet<String> {
    let mut set = std::collections::BTreeSet::new();
    let mut stack = vec![PathBuf::from("src/include"), PathBuf::from("src"),
                         PathBuf::from("lib/include"), PathBuf::from("../lib/include")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for e in entries.filter_map(Result::ok) {
            let path = e.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().map(|x| x != "rs").unwrap_or(true) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else { continue };
            // Comments and string literals are dropped first: `lisp.rs` mentions
            // `struct itree_node` in a comment, and a mention must not count as a
            // declaration (it would keep a renamed type's assertions alive).
            let code = strip_comments_and_strings(&text);
            for kw in ["struct", "union", "enum", "type"] {
                for name in names_after_keyword(&code, kw) {
                    set.insert(name);
                }
            }
        }
    }
    set
}

/// TEXT without comments and without the contents of string/char literals, so a
/// mention inside either cannot look like a declaration.
fn strip_comments_and_strings(text: &str) -> String {
    let (b, mut out) = (text.as_bytes(), Vec::with_capacity(text.len()));
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if i + 1 < b.len() && b[i + 1] == b'/' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if i + 1 < b.len() && b[i + 1] == b'*' => {
                i += 2;
                let mut depth = 1;
                while i < b.len() && depth > 0 {
                    if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && i + 1 < b.len() && b[i + 1] == b'/' {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                out.push(b' ');
            }
            b'\'' => {
                // A char literal (`'x'`, '\n'), not a lifetime: only a closing quote
                // two or three bytes away makes it one.
                let mut j = i + 1;
                if j < b.len() && b[j] == b'\\' {
                    j += 2;
                } else {
                    j += 1;
                }
                if j < b.len() && b[j] == b'\'' {
                    i = j + 1;
                    out.push(b' ');
                } else {
                    out.push(b'\'');
                    i += 1;
                }
            }
            b'"' => {
                i += 1;
                while i < b.len() {
                    if b[i] == b'\\' {
                        i += 2;
                    } else if b[i] == b'"' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
                out.push(b' ');
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Every identifier that follows KEYWORD as a whole word in TEXT (byte-wise, so the
/// scan is safe on non-ASCII comments).
fn names_after_keyword(text: &str, keyword: &str) -> Vec<String> {
    let (bytes, kw) = (text.as_bytes(), keyword.as_bytes());
    let mut out = Vec::new();
    let mut i = 0;
    while i + kw.len() <= bytes.len() {
        if &bytes[i..i + kw.len()] == kw {
            let before_ok = i == 0
                || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
            if before_ok && bytes.get(i + kw.len()).copied().map(|c| c.is_ascii_whitespace()).unwrap_or(false) {
                let mut j = i + kw.len();
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                let start = j;
                while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                    j += 1;
                }
                if j > start {
                    out.push(text[start..j].to_string());
                }
            }
        }
        i += 1;
    }
    out
}

/// The asserted/pending counts the gate must not fall below: a type that disappears
/// from the scan would drop assertions, and a dropped assertion is invisible in the
/// compiled probe (review B, F1).  Adding mirrors only raises asserted / lowers
/// pending, so this file stays as a floor.
fn check_counts(asserted: usize, pending: usize) -> Result<(), String> {
    let path = Path::new(REPO).join("docs/baseline/layout/counts.tsv");
    // The ratchet must exist and parse: silently returning Ok would make every
    // shrinkage check vacuous (review B's B5).
    let text = fs::read_to_string(&path).map_err(|e| {
        format!("{}: {e} (the layout ratchet must exist; 0.1 records why)", path.display())
    })?;
    let mut want_asserted: Option<usize> = None;
    let mut want_pending: Option<usize> = None;
    for line in text.lines().skip(1) {
        match line.split_once('\t') {
            Some(("asserted", v)) => {
                want_asserted = Some(v.trim().parse().map_err(|e| {
                    format!("{}: asserted: {e}", path.display()) })?)
            }
            Some(("pending", v)) => {
                want_pending = Some(v.trim().parse().map_err(|e| {
                    format!("{}: pending: {e}", path.display()) })?)
            }
            _ => {}
        }
    }
    let (Some(want_asserted), Some(want_pending)) = (want_asserted, want_pending) else {
        return Err(format!("{}: needs both `asserted` and `pending` rows", path.display()));
    };
    if asserted < want_asserted || pending > want_pending {
        return Err(format!(
            "layout gate shrank: {asserted} asserted / {pending} pending, \
             floor is {want_asserted} / {want_pending} ({})",
            path.display()));
    }
    Ok(())
}

/// The *key set* ratchet.  `check_counts` only compares how many keys are asserted, so one
/// assertion could be traded for another: deleting `offsetof Lisp_Buffer_Local_Value.where`
/// and `…defcell` while adding `sizeof Emacs_GC` + `alignof Emacs_GC` kept the count at 247,
/// left the mirror's `where`/`defcell` swapped relative to C, and both gates stayed green
/// (review B's F4).  The recorded list is the set that was asserted when the baseline was
/// last accepted; it may only grow.  Removing a key is a reviewed edit to that file.
fn check_keys(asserted: &[String]) -> Result<(), String> {
    let path = Path::new(REPO).join("docs/baseline/layout/asserted-keys.txt");
    let text = fs::read_to_string(&path).map_err(|e| {
        format!("{}: {e} (the layout key ratchet must exist; PORTING.md P-68 records why)",
                path.display())
    })?;
    let have: std::collections::BTreeSet<&str> = asserted.iter().map(String::as_str).collect();
    let missing: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter(|l| !have.contains(l))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "layout keys disappeared ({} of them): {}{} — a key that stops being asserted \
             stops being checked, and swapping one for another keeps the count the same \
             ({}); update {} only with a reason",
            missing.len(),
            missing.iter().take(6).cloned().collect::<Vec<_>>().join(", "),
            if missing.len() > 6 { ", …" } else { "" },
            path.display(),
            path.display()));
    }
    Ok(())
}

fn layout_asserts() -> Result<PathBuf, String> {
    let dir = Path::new(REPO).join("docs/baseline/layout");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "tsv"))
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy()
                                              .starts_with("c-layout-")))
        .collect();
    files.sort();
    // Exactly one `*.tsv`, and it is named: taking the first entry of the directory meant a
    // stray `README.md` or a second `A-extra.tsv` silently became the baseline, with the
    // real numbers ignored (review B's F5).  The name is logged so the gate's input is
    // visible in its own output.
    let [tsv] = files.as_slice() else {
        return Err(format!("{} needs exactly one c-layout-*.tsv baseline, found {} ({})",
                           dir.display(), files.len(),
                           files.iter().map(|p| p.display().to_string())
                               .collect::<Vec<_>>().join(", ")));
    };
    let tsv = tsv.clone();
    println!("layout baseline: {}", tsv.display());
    let text = fs::read_to_string(&tsv).map_err(|e| e.to_string())?;
    let mut out = String::from(
        "// Generated by `xtask layout-asserts` from the frozen C layout; do not edit.\n\
         // Every line is a number the C compiler printed for the reference headers.\n",
    );
    let known = known_types();
    let objects = declared_object_types();
    if std::env::var("XTASK_DEBUG_TYPES").is_ok() {
        let hits: Vec<&String> = known.iter().filter(|n| n.contains("itree")).collect();
        eprintln!("debug: itree-ish types: {hits:?}");
    }
    let mut n = 0;
    let mut pending: Vec<String> = Vec::new();
    let mut asserted_keys: Vec<String> = Vec::new();
    for line in text.lines().skip(1) {
        let Some((key, value)) = line.split_once('\t') else { continue };
        let mut parts = key.splitn(2, ' ');
        let (kind, rest) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        let root = rest.split('.').next().unwrap_or("");
        // A key's root is usually a type, but C also has file-scope *objects*
        // (`extern mpz_t mpz[5]`): for those the mirror's own declaration supplies the
        // type (`static mut mpz: [mpz_t; 5]`), which is what makes the bound visible to
        // this gate (review B's B1: `[mpz_t; 6]` passed every gate).
        let rest = match objects.get(root) {
            Some(ty) if !known.contains(root) => ty.as_str(),
            _ => rest,
        };
        if !known.contains(root) && !objects.contains_key(root) {
            pending.push(format!("{key} = {value}"));
            continue;
        }
        let Some(expr) = rust_type_expr(kind, rest) else { continue };
        out.push_str(&format!(
            // `::core::assert!`, not `assert!`: the probe includes every mirror in one crate, and
            // a mirror that defines its own `assert` macro shadows the prelude's - android-asset.h
            // does, and the generated message form then failed to expand ("no rules expected `,`",
            // status 3.153).  The path keeps the generated assertions immune to that.
            "const _: () = ::core::assert! ({expr} == {value}, \"{kind} {rest}\");\n"
        ));
        asserted_keys.push(key.to_string());
        n += 1;
    }
    fs::create_dir_all(units_dir()).ok();
    let path = units_dir().join("layout_asserts.rs");
    fs::write(&path, out).map_err(|e| e.to_string())?;
    // The pending list is written before the ratchet check, so a failure still says which
    // keys regressed.
    if !pending.is_empty() {
        let list = units_dir().join("layout_pending.txt");
        fs::write(&list, pending.join("\n") + "\n").ok();
    }
    asserted_keys.sort();
    let asserted_list = units_dir().join("layout_asserted.txt");
    fs::write(&asserted_list, asserted_keys.join("\n") + "\n").ok();
    check_counts(n, pending.len())?;
    check_keys(&asserted_keys)?;
    println!("{n} layout assertions from {}", tsv.display());
    if !pending.is_empty() {
        println!("{} keys pending a mirror: {}", pending.len(),
                 pending.iter().take(8).cloned().collect::<Vec<_>>().join(", "));
    }
    Ok(path)
}

fn layout_check() -> i32 {
    let asserts = match layout_asserts() {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    fs::create_dir_all(units_dir()).ok();
    let probe = units_dir().join("layout_probe.rs");
    let mut src = String::new();
    src.push_str("#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]\n");
    src.push_str("use std::mem::{align_of, size_of, offset_of};\n");
    // Every header mirror, so the assertions run for all of them; globals.rs is
    // included by lisp.rs (mirroring lisp.h including <globals.h>).
    let mut mirrors: Vec<PathBuf> = fs::read_dir("src/include")
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    mirrors.retain(|p| p.extension().map(|x| x == "rs").unwrap_or(false));

    // Platform-exclusive mirrors are skipped here and compiled by their own platform probes: the
    // frozen target never compiles their headers, and their X-compatibility names collide with
    // each other and with dispextern.rs by construction (status 3.147 - 36 E0428s across
    // androidgui.rs/haikugui.rs/androidterm.rs).  The list is data
    // (docs/baseline/layout-excluded-mirrors.txt), one file name per line, and the skip count is
    // printed so nothing is excluded silently (P-66).  It has to happen *before* the include-graph
    // dedupe below: `character.rs` is included only by androidterm.rs and haikuterm.rs, so
    // dropping those two first is what keeps `character.rs` in the probe (its `MAX_CHAR` is what
    // `dispextern.rs` names).
    // A reason may wrap onto continuation lines that start with whitespace, and one of those
    // lines' first word happens to be `dispextern.rs` (it is *named* in `nsgui.rs`'s reason) -
    // taking every line's first token excluded a mirror that is not on the list and silently
    // stopped its assertions from running (measured: the skip list grew by `mirs`, `shadowed`,
    // `compile`, `live`, `` `error[E0428]: ` `` and `dispextern.rs`).  An entry is a line that
    // starts in column 1 and whose first token is a mirror name, so both filters are required.
    let excluded: Vec<String> = fs::read_to_string("docs/baseline/layout-excluded-mirrors.txt")
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with(char::is_whitespace))
        .map(|l| l.split('#').next().unwrap_or("").trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
        .filter(|l| l.ends_with(".rs"))
        .collect();
    let total = mirrors.len();
    let skipped: Vec<String> = mirrors.iter()
        .filter(|p| p.file_name().map(|n| excluded.iter().any(|e| e == &n.to_string_lossy()))
                    .unwrap_or(false))
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    let skipped = { let mut v = skipped; v.sort(); v };
    mirrors.retain(|p| !p.file_name()
        .map(|n| excluded.iter().any(|e| e == &n.to_string_lossy())).unwrap_or(false));
    // The lib/include mirrors that compile as their own crate root are seeded too, so the shared
    // gate really compiles them (status 3.11, closed 3.217).  A mirror is a header: most of
    // lib/include legitimately needs the including unit's scalars, so the seed is the measured
    // class in docs/baseline/layout-standalone-mirrors.txt (one file name per line) and the
    // complement is printed, not hidden.
    let standalone: Vec<String> =
        fs::read_to_string("docs/baseline/layout-standalone-mirrors.txt")
            .unwrap_or_default()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
            .filter(|l| l.ends_with(".rs"))
            .collect();
    let mut lib_mirrors: Vec<PathBuf> = fs::read_dir("lib/include")
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    lib_mirrors.retain(|p| p.extension().map(|x| x == "rs").unwrap_or(false));
    let lib_total = lib_mirrors.len();
    lib_mirrors.retain(|p| p.file_name()
        .map(|n| standalone.iter().any(|e| e == &n.to_string_lossy())).unwrap_or(false));
    let lib_seeded: Vec<String> = lib_mirrors.iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string())).collect();
    if lib_total > 0 {
        println!("layout-check: + {} of {} lib/include mirror(s) seeded as their own crate root \
({} need an includer, PORTING.md 0.1-9): {}",
                 lib_seeded.len(), lib_total, lib_total - lib_seeded.len(), lib_seeded.join(", "));
    }
    mirrors.extend(lib_mirrors);

    println!("layout-check: {} of {} src mirror(s) compiled, {} platform-exclusive skipped ({})",
             total - skipped.len(), total, skipped.len(), skipped.join(", "));
    // PORTING.md 0.1-4: expand the include! graph and dedupe by file, so a mirror that
    // (like lib/inttypes.in.rs will) includes another mirror is not listed twice.
    let mut included: std::collections::BTreeSet<PathBuf> = std::collections::BTreeSet::new();
    for p in &mirrors {
        collect_includes(p, &mut included);
    }
    mirrors.retain(|p| !included.contains(&canonical(p)));
    // The foundation mirrors, in C's header order, ahead of everything else.  Two reasons:
    // `macro_rules!` are textually scoped, so the mirror that defines `eassert!`/`eassume!`
    // (lisp.rs) has to be expanded before any mirror that uses them; and the mirrors C reaches
    // through the translation unit rather than through their own `#include` (PORTING.md 0.1-9 -
    // lisp.rs is the documented case, and character.h/systime.h/stdio.h are the same in
    // practice) name definitions that live in these files.  `Expand.seen` dedupes a file included
    // twice, so forcing them to the front cannot duplicate items; being first is what counts even
    // when the list-dedupe above already dropped them, because only the *first* occurrence's
    // position matters (status 3.147: character.rs was reachable only through
    // androidterm.rs/haikuterm.rs - both platform-excluded now - and lisp.rs was dropped by the
    // dedupe, so `dispextern.rs`'s `MAX_CHAR` and `character.rs`'s own `eassert!` were unresolved).
    // `lib/include` rather than `../lib/include`: the port's lib mirrors live inside
    // (the same mistake P-53 records for the type scan).
    let mut foundation: Vec<PathBuf> = vec![PathBuf::from("lib/include/stdio.rs"),
                                            PathBuf::from("lib/include/mini-gmp.rs"),
                                            PathBuf::from("src/include/lisp.rs"),
                                            PathBuf::from("src/include/character.rs"),
                                            PathBuf::from("src/include/systime.rs")];
    foundation.retain(|p| p.is_file());
    foundation.reverse();
    for f in foundation {
        if let Some(i) = mirrors.iter().position(|p| canonical(p) == canonical(&f)) {
            mirrors.remove(i);
        }
        mirrors.insert(0, f);
    }
    mirrors.push(asserts);
    for path in mirrors {
        let full = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        src.push_str(&format!("include!({:?});\n", full.display().to_string()));
    }
    src.push_str("fn main() { println!(\"layout probe ok\"); }\n");
    if let Err(e) = fs::write(&probe, src) {
        return fail(e);
    }
    let dir = units_dir().join("layout_probe_src");
    let root = match materialize(&probe, &dir) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let bin = units_dir().join("layout_probe");
    let status = rustc_cmd ()
        .args(["--edition", "2024", "--crate-name", "layout_probe", "-C", "panic=abort", "-o"])
        .arg(&bin)
        .arg(&root)
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => return fail(format!("layout probe failed to compile ({s})")),
        Err(e) => return fail(e),
    }
    match Command::new(&bin).status() {
        Ok(s) if s.success() => {
            println!("layout-check passed (see build/femacs/layout_asserts.rs)");
            0
        }
        Ok(s) => fail(format!("layout probe exited with {s}")),
        Err(e) => fail(e),
    }
}

// --- spike probes ----------------------------------------------------------

/// `xtask probe <rs-file> <out> [<extra rustc args>…]`: build one spike probe.
///
/// The shell probes under `spikes/` reach their mirrors through their own `include!` lists,
/// which have no include guard, so a file reached by two paths (`lisp.rs` ->
/// `thread.rs` -> `sys_select.rs` -> `time.in.rs` and `buffer.rs` -> `time.in.rs`) was
/// defined twice and `rustc` stopped with E0428.  They are materialized exactly as a unit is
/// (PORTING.md 0.1-4), and the extras after the output path are passed to `rustc` unchanged
/// (`macro-probe.sh` needs `--crate-type lib --emit metadata`).
fn probe_build(args: &[String]) -> i32 {
    let (Some(src), Some(out)) = (args.first(), args.get(1)) else {
        return fail("probe needs <rs-file> <out> [<extra rustc args>…]");
    };
    let repo=Path::new(REPO);
    let output=match tooling::guard_scratch(repo,Path::new(out)){Ok(p)=>p,Err(e)=>return fail(e)};
    if output.exists(){return fail("probe output already exists");}
    if let Some(parent)=output.parent(){if let Err(e)=fs::create_dir_all(parent){return fail(e);}}
    let src=Path::new(src);
    let scratch=match tooling::owned_new_output(repo,None,"probe-graph"){Ok(run)=>run,Err(e)=>return fail(e)};
    let root=match materialize(src,&scratch.path.join("graph")){Ok(path)=>path,Err(e)=>return fail(e)};
    let identities=output.with_extension("inputs.json");
    if identities.exists(){return fail("probe input receipt already exists");}
    let recorded=(|| -> Result<(),String> {
        tooling::write_json(&identities,&serde_json::json!({"schema":"femacs-materialized-probe-inputs-v1",
            "root_source":src,"root_source_sha256":tooling::sha256_file(src)?,
            "rust_toolchain":probes::pin(repo)?,"graph":scratch.path.join("graph"),
            "files":tooling::graph_input_hashes(&scratch.path.join("graph"))?}))
    })();
    if let Err(error)=recorded{return fail(error);}
    let status = rustc_cmd ()
        .args(["--edition", "2024", "-O"])
        .arg(&root)
        .arg("-o")
        .arg(&output)
        .args(&args[2..])
        .status();
    match status {
        Ok(s) if s.success() => 0,
        Ok(s) => fail(format!("rustc failed for {} (exit status: {s})", src.display())),
        Err(e) => fail(format!("probe {}: {e}", src.display())),
    }
}

// --- gates -----------------------------------------------------------------

fn gate_fast() -> i32 {
    // The inventory gate below reads `#if` conditions through the frozen macro table, so the
    // table itself is gated first: every file allowed to consult it must still agree with
    // clang on every branch it decides (PORTING.md P-66).  Without this step the table's
    // accuracy would be a claim in a help string instead of a measurement.
    println!("== condition evaluator ==");
    if !Path::new("build/emacs-ref/src/config.h").is_file() {
        return fail("build/emacs-ref is missing: the `#if` differential needs the frozen \
configuration (see PORTING.md 2.1's setup steps)");
    }
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-cond-eval.py", "--verified",
               "docs/baseline/config/cond-eval-verified.txt"])
        .status()
    {
        Ok(s) if s.success() => println!("condition evaluator: verified list agrees with clang"),
        Ok(_) => return fail("the `#if` evaluator disagrees with clang on a verified header"),
        Err(e) => return fail(e),
    }
    // The frozen configuration is the other half of that table: it decides every
    // `--cfg feature="..."` and every `src/include/config.rs` constant.  Reading its header line by
    // line recorded a `#define` inside an arm the compiler never takes as if it were live, and
    // Android's `WORDS_BIGENDIAN` went with it: the port's `sfnt_swap16_1`/`sfnt_swap32_1` became
    // empty bodies, every font table came back byte-swapped, and no font ever registered (status
    // 4.806).  The preprocessor that reads the same header is the authority here, so the table is
    // compared with it before anything reads the table.
    println!("== frozen configuration ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-config-json.py"])
        .status()
    {
        Ok(s) if s.success() => println!("frozen configuration: every key agrees with its preprocessor"),
        Ok(_) => return fail("a frozen configuration disagrees with the preprocessor that reads its header"),
        Err(e) => return fail(e),
    }
    println!("== inventory gate ==");
    let inv = Command::new("python3")
        .current_dir(REPO)
        .args(["tools/extract-abi.py", "--all", "--reference", "emacs", "--root", "."])
        .status();
    match inv {
        Ok(s) if s.success() => println!("inventory: no differences"),
        Ok(_) => return fail("inventory differences (nothing is ported yet, so any entry is a bug)"),
        Err(e) => return fail(e),
    }
    // `DEFVAR_*`'s storage is not a C object definition, so the inventory gate above cannot
    // see it: a unit that owns a variable (C's `globals.f_<vname>`) may reference it through
    // `globals.rs`'s declaration and never define it, and only the eventual link would
    // notice.  The names are checked mechanically (status §3.29).
    println!("== globals gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-globals.py"])
        .status()
    {
        Ok(s) if s.success() => println!("globals: every DEFVAR_* variable has a definition"),
        Ok(_) => return fail("a DEFVAR_* variable has no definition in its own unit"),
        Err(e) => return fail(e),
    }
    // A ledger's own claim is a fifth reading of the tree (status 3.349/3.350): a ledger that
    // ticks a function no ported file carries is a contradiction the four gates around this one
    // cannot see, because each of them measures something else (line counts, unticked boxes,
    // the Swift units, the ABI difference).
    println!("== ledger ticks ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-ledger-ticks.py"])
        .status()
    {
        Ok(s) if s.success() => println!("ledger ticks: every ticked function is carried"),
        Ok(_) => return fail("a ledger ticks a function no ported file carries"),
        Err(e) => return fail(e),
    }
    // The numbers the hand-off documents quote are claims like any other, and the tables under
    // `docs/baseline/` are what they are derived from: three stale readings (a queue with 14 rows
    // for names the ratchet had already closed, a "31 upstream gaps" that the delivery device
    // provides, a symbol ratchet entry that had gone clean) survived until a session read them
    // against the tables by hand (status 4.748/4.749, PORTING.md P-320).
    println!("== doc numbers ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-doc-numbers.py"])
        .status()
    {
        Ok(s) if s.success() => println!("doc numbers: the documents agree with the tables"),
        Ok(_) => return fail("a document quotes a number the baseline tables contradict"),
        Err(e) => return fail(e),
    }
    // A P-86 file's ledger records C line ranges instead of ticked boxes (its checklist is empty),
    // and the table is prose unless something reads it (status 3.372).
    println!("== dead-file coverage ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-dead-file-coverage.py"])
        .status()
    {
        Ok(s) if s.success() => println!("dead-file coverage: every range table accounts for its reference"),
        Ok(_) => return fail("a P-86 ledger's C range table has a gap or a malformed row"),
        Err(e) => return fail(e),
    }
    // P-98's own debt note: "the same family's sites currently receive only declared members, but
    // nothing pins that".  A payload-less Rust enum has no legal value outside its discriminants,
    // so every integer-to-enum transmute is recorded in docs/baseline/enum-transmutes.tsv, and the
    // gate re-parses each target enum (a renumbering must not pass silently).
    println!("== enum transmute gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-enum-transmutes.py"])
        .status()
    {
        Ok(s) if s.success() => println!("enum transmutes: every site recorded, every enum unchanged"),
        Ok(_) => return fail("an integer-to-enum transmute is unrecorded, or its enum moved"),
        Err(e) => return fail(e),
    }
    println!("== workspace check ==");
    if let Err(e) = cargo_build::check_workspace_crates() {
        return fail(e);
    }
    println!("== unsafe boundary gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-unsafe-boundary.py"])
        .status()
    {
        Ok(s) if s.success() => println!("unsafe boundary: pure crates carry forbid(unsafe_code)"),
        Ok(_) => return fail("unsafe boundary check failed"),
        Err(e) => return fail(e),
    }
    // The link above takes the reference archive's own member set, which is what keeps the
    // ported `lib/` units that define a *system* function's name (`open`, `fcntl`, ...) out of
    // the program.  That rule is invisible in the link command itself, so the thirteen units
    // are audited one by one here (docs/status.md 4.393, PORTING.md's P-86 family).
    println!("== lib shadow gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-libshadows.py"])
        .status()
    {
        Ok(s) if s.success() => println!("lib shadows: no ported lib/ object shadows a system symbol"),
        Ok(_) => return fail("a ported lib/ object is linked, or shadows a system symbol"),
        Err(e) => return fail(e),
    }
    // C's `memcpy`/`memset` count *bytes*; `copy_nonoverlapping`/`write_bytes` count `T`s and
    // multiply by `size_of::<T> ()` themselves, so a ported call that keeps C's byte count
    // writes `size_of` times too much.  P-93 found it three times in the `memcpy` family and
    // audited only `copy_nonoverlapping`-shaped sites; the `memset` family went unread until the
    // dump died writing 1,572,864 bytes into a 6144-byte array (P-114, status 4.426).  The gate
    // needs no type inference: a count that mentions `size_of` must go through a byte pointer.
    println!("== typed-count gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-typed-counts.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("typed counts: every byte count goes through a byte pointer (P-93/P-114)")
        }
        Ok(_) => return fail("a byte count is handed to a non-byte pointer (P-93/P-114)"),
        Err(e) => return fail(e),
    }
    // The four classes of declaration 0.1-19 keeps out of the tree - the copied
    // ownership block, the product name in a comment, the removed
    // `@ <rev> (<product> x.y.z)` parenthetical, and the comment that recites any of
    // the three instead of carrying one (status 3.100) - are prose, and prose comes
    // back: every unit ported after `fd9cf89` copies its C file's comments, and
    // `src/emacs.c`'s own literals are the worked example of the class that *stays*.
    // This gate reads the tree the way the rule does (comments and string literals
    // separated, `'a` lifetimes versus `'a'` chars) and compares with
    // `docs/baseline/declarations-allowlist.tsv`, so the literals that must stay are
    // counted with a reason rather than being invisible to every other gate (3.73).
    // DoD-2's third leg: the subr table itself.  The symbol gate covers linkage names; this
    // asks both programs which subrs they register, with what arity and interactive form.
    println!("== subr table gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-subrs.py"])
        .status()
    {
        Ok(s) if s.success() => println!("subr table: the port registers the reference's subrs"),
        Ok(_) => return fail("the port's subr table differs from the reference's (DoD-2)"),
        Err(e) => return fail(e),
    }
    // P-178: a `DEFVAR_LISP` whose C initialiser line was dropped still compiles, still
    // registers, and reads nil only when a consumer asks.  This step compares the *values* of
    // every such variable between the two programs, which is why the class cannot hide here.
    println!("== defvar value gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-defvar-values.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("defvar values: every DEFVAR the port declares holds the reference's value")
        }
        Ok(_) => return fail("a DEFVAR holds a different value than the reference (P-178)"),
        Err(e) => return fail(e),
    }
    // P-197: the non-C platform rows of the port map (Swift/Kotlin/C++) had no gate, so a
    // missing target printed as `-` and passed.  This step verifies every mapped target
    // exists.  P-198 closed the one gap it found (`src/nsxwidget.m`'s Swift lane), so the
    // `--warn` that kept `gate-fast` green while that file was missing is gone.
    println!("== platform inventory gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-platform-inventory.py"])
        .status()
    {
        Ok(s) if s.success() => println!("platform inventory: every mapped platform target exists"),
        Ok(_) => return fail("a mapped platform target is missing (P-197)"),
        Err(e) => return fail(e),
    }
    println!("== declaration gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-declarations.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("declarations: no product-name declaration in the port")
        }
        Ok(_) => {
            return fail("a ported file carries a product-name declaration (PORTING.md 0.1-19)")
        }
        Err(e) => return fail(e),
    }
    // The declaration gate above reads the name the rule names; the bare word is what nobody
    // read.  A ported unit copies its C file's comments, and those call the program they belong
    // to by name - so a comment line carrying the bare name is either the reference's own bytes
    // (a copy, or a citation inside backticks or quotes) or this repository's own voice, which
    // calls the reference `C` (PORTING.md 0.1-19, its twenty-second half).
    println!("== own-voice gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-own-voice.py"])
        .status()
    {
        Ok(s) if s.success() => {
            println!("own voice: the port calls the reference C, not by name")
        }
        Ok(_) => {
            return fail("a ported comment in this repository's own voice names the reference \
                         (PORTING.md 0.1-19)")
        }
        Err(e) => return fail(e),
    }
    // A unit that repeats a declaration a mirror *in its own include graph* already carries
    // compiles, passes every text gate, and becomes `clashing_extern_declarations` the moment
    // the two spellings disagree - the warning gate-fast's unit builds already print.  The
    // ratchet holds the rows that are deliberate (status 3.76/3.77).
    // A string handed to C through `.as_ptr ()` has to be NUL-terminated, and a Rust `"..."` is
    // not: rustc packs literals back to back in rodata, so C reads on past them.  Two defects of
    // this shape were found by hand (lib/strftime.rs's L_! in 4.398, src/region-cache.rs's two
    // fprintf format strings in 4.399) and both were text, so the gate reads text - the tree's
    // 11,683 `c"..."` sites are the accepted spelling, `b"...\\0"` the other, and a plain
    // literal with no terminator is the defect.
    println!("== c string literal gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-c-string-literals.py"])
        .status()
    {
        Ok(s) if s.success() => println!("c strings: every site carries its terminator"),
        Ok(_) => return fail("a string literal reaches C without a NUL terminator"),
        Err(e) => return fail(e),
    }
    // The six copied trees are a hard constraint of the goal ("copied verbatim, one word
    // unchanged"), and the gate that reads them is only a claim unless it runs somewhere: the
    // retirement of the reference's Makefile.in scaffolding (PORTING.md P-102) left it red for
    // days because nothing called it (measured 2026-10-16: 5 failures, all of them the retired
    // files' absences and their rows in the substitution table).
    println!("== copied-trees gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-verbatim.py"])
        .status()
    {
        Ok(s) if s.success() => println!("copied trees: every tracked file is byte-identical or sanctioned"),
        Ok(_) => return fail("a copied tree's file list or text moved without a record"),
        Err(e) => return fail(e),
    }
    println!("== mirror-dupes gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-mirror-dupes.py"])
        .status()
    {
        Ok(s) if s.success() => println!("mirror duplicates: none beyond the ratchet"),
        Ok(_) => {
            return fail("a unit repeats a declaration a mirror in its include graph carries")
        }
        Err(e) => return fail(e),
    }
    // The Android (bionic) record layouts the port asserts, against the NDK's own answers: three
    // of them were found on the device the hard way (status 4.785-4.792), so the comparison is a
    // gate step rather than a note.  A missing NDK skips it with a line rather than failing, so a
    // host without the Android toolchain can still run the rest of the gate.
    println!("== android layout gate ==");
    match Command::new("python3")
        .current_dir(REPO)
        .args(["tools/check-android-layout.py"])
        .status()
    {
        Ok(s) if s.success() => println!("android layouts: every asserted record matches the NDK"),
        Ok(s) if s.code() == Some(2) => {
            println!("android layouts: skipped (no NDK toolchain on this host)")
        }
        Ok(_) => return fail("an asserted Android record no longer matches the NDK's layout"),
        Err(e) => return fail(e),
    }
    println!("== probe programs ==");
    if let Err(e) = probe_programs() {
        return fail(e);
    }
    println!("== layout gate ==");
    for name in ["fam","shadow","macro"] {
        println!("== {name} ==");
        if let Err(e)=probes::run_probe(Path::new(REPO),name,&probes::ProbeOptions::default()){return fail(e);}
    }
    layout_check()
}

/// Compile every probe *program* the way its script does — from ``, so
/// `rust-toolchain.toml` decides the toolchain — without running the C side.
///
/// `gate-fast` was blind to the probes, and `ec20c48` broke three of them: the scripts
/// ran a bare `rustc` from the repository root, which has no `rust-toolchain.toml`, so
/// the *stable* toolchain resolved and the mirrors' `VaList` parameter became E0658.
/// The layout gate stayed green (review D's F1).  A probe that cannot even compile is
/// not evidence, so this is part of the fast gate now; the C-differential half is
/// `gate-probes`.
fn probe_programs() -> Result<(), String> {
    // `f_inc_probe.rs` is the newest and the only one without a C side: it compiles the batch-F
    // header mirrors of `nt/inc/`, which no configuration reaches - `layout-check` covers
    // the `src/include` and `lib/include` mirrors only, and two real defects lived in that gap
    // until the probe existed (docs/status.md 3.102).
    // `haiku_inc_probe.rs` and `android_inc_probe.rs` are the platform-exclusive mirrors'
    // compiler: `layout-check` excludes those mirrors by data
    // (docs/baseline/layout-excluded-mirrors.txt, status 3.147 - the X-compatibility names
    // collide by construction), so this is the only place their assertions and declarations are
    // compiled.  Same shape as `f_inc_probe.rs`: the probe names its mirrors and the graph's
    // de-duplication does the rest.
    // `win_inc_probe.rs` is the same shape again, for the two `src/include` mirrors whose unit is
    // an empty shape (`src/w32inevt.h`, `src/w32select.h`): no configuration compiles a `src/w32*`
    // unit on this target, so those mirrors had no consumer at all until this probe (status 3.159).
    // `ns_inc_probe.rs` is the same shape once more, for the two NS mirrors whose headers are
    // dead on this target (`src/nsgui.h`, `src/nsxwidget.h`): `layout-check` excludes nsgui.rs
    // by data (the `Emacs_Cursor` collision above), and the pair has no consumer at all
    // otherwise (status 3.161).
    // `sfnt_inc_probe.rs` is the same shape for the two platform-independent orphans whose
    // *unit* is the empty shape (`src/sfnt.h`, `src/fontset.h`): the frozen configuration
    // builds neither `src/sfnt.c`/`src/sfntfont.c` nor `src/fontset.c`, and the header-form
    // `build-tu src/sfnt.h` cannot compile any mirror that names `c_int` (the crate root is the
    // mirror, which carries no `use`), so the probe is what compiles the pair with the
    // `lisp.h`/`<setjmp.h>` names they take from their translation unit.
    const PROBES: [&str; 16] = ["spikes/inline_probe.rs", "spikes/accessor_probe.rs",
                                "spikes/write_probe.rs", "spikes/va_probe.rs",
                                "spikes/layout.rs", "spikes/jump.rs", "spikes/scan.rs",
                                "spikes/f_inc_probe.rs", "spikes/win_inc_probe.rs",
                                "spikes/haiku_inc_probe.rs", "spikes/android_inc_probe.rs",
                                "spikes/android_asset_probe.rs", "spikes/ns_inc_probe.rs",
                                "spikes/x_inc_probe.rs", "spikes/sfnt_inc_probe.rs",
                                "spikes/stdbit_family_probe.rs"];
    let repo=fs::canonicalize(REPO).map_err(|e|e.to_string())?;
    let mut roots=serde_json::Map::new();
    for probe in PROBES {
        roots.insert(probe.into(),serde_json::json!(tooling::sha256_file(&repo.join(probe))?));
    }
    let run=tooling::owned_new_output(&repo,None,"probe-programs")?;
    let mut graphs=serde_json::Map::new();let mut steps=Vec::new();
    let result=(|| -> Result<(),String> {
        for probe in PROBES {
            let stem=Path::new(probe).file_stem().ok_or("probe stem missing")?.to_string_lossy().to_string();
            let directory=run.path.join(format!("graph-{stem}"));
            let root=materialize(&repo.join(probe),&directory)?;
            graphs.insert(probe.into(),serde_json::json!(tooling::graph_input_hashes(&directory)?));
            let plan=tooling::StepPlan{id:stem.clone(),program:"rustc".into(),
                args:vec!["--edition".into(),"2024".into(),"--crate-type".into(),"bin".into(),"--emit".into(),"metadata".into(),root.into_os_string(),"-o".into(),run.path.join(format!("{stem}.rmeta")).into_os_string()],
                cwd:repo.clone(),env:vec![("RUSTUP_TOOLCHAIN".into(),probes::pin(&repo)?.into())]};
            let receipt=tooling::execute_step(&plan,&run)?;
            let checked=tooling::require_success(&receipt);steps.push(receipt);checked?;
            println!("probe {stem}: metadata compiler passed");
        }
        Ok(())
    })();
    tooling::write_json(&run.path.join("report.json"),&serde_json::json!({"schema":"femacs-metadata-probes-v1",
        "status":if result.is_ok(){"passed"}else{"failed"},"root_sources":roots,"materialized":graphs,
        "rust_toolchain":probes::pin(&repo)?,"steps":steps,"error":result.as_ref().err()}))?;
    println!("probe metadata identities: {}",run.path.join("report.json").display());
    result
}

/// `gate-load`: the loading ladder's first rung (docs/port-plan.md section 7.1).
///
/// The frozen reference build knows its own `temacs` link command; this asks it with
/// `make -n temacs`, substitutes the port's objects for the reference ones, and links.  Every
/// symbol the linker cannot resolve is a unit that is not ported yet or a name the port spells
/// wrong - the worklist L0 needs, which no other gate can produce (the text gates compare
/// files, the symbol gate compares exports, the undefined-reference gate works per object
/// against the SDK's own export list).  `--report` prints the list and exits 0; without it the
/// command is a gate.
///
/// **The link is driven by `rustc`, not by the C compiler**, and that is the whole point of the
/// driver below.  A unit is a `--crate-type lib` object (PORTING.md 0.1), so it expects the
/// Rust runtime's own symbols - `core`'s panic machinery, `compiler_builtins`' intrinsics, and
/// the `rust_begin_unwind` lang item - which a plain `clang` link leaves undefined (measured:
/// 210 of the first 230 undefined symbols were `core::panicking::*`).  The driver is an empty
/// `#![no_std] #![no_main]` bin crate with the port's own `#[panic_handler]` (PORTING.md 424:
/// `panic = "abort"`, and a panic is a defect, so the handler aborts like C's `emacs_abort`),
/// so rustc hands clang the runtime rlibs and the program's entry stays the port's own `main`
/// (emacs.rs's `#[unsafe(no_mangle)]` one).  `-nodefaultlibs` comes with the bin crate, hence
/// the explicit `-lSystem` (libc is what the ported C code calls).  The objects, the port's
/// `lib/` objects in place of the reference `libgnu.a` (the port has no archive, and mixing the
/// reference library in is the C/Rust hybrid goal constraint 2 forbids), the frozen `-L` paths
/// and `-lncurses` all go through a response file.
/// The members of the frozen `libgnu.a`, in the order `ar` prints them.
///
/// This is C's own `lib/` object set: the frozen configuration's `lib/Makefile` lists exactly
/// these, and C's `temacs` rule links them through `LIBEGNU_ARCHIVE`.  The port has no archive of
/// its own (goal constraint 2), so its objects for these members stand in - and *only* these:
/// see the call site for what linking a member C never builds costs.
/// The driver of a shared library: no `main`, but the panic handler C's build gets from libc.
/// The Android link's `-nodefaultlibs` is the reason this driver is compiled separately: the
/// unit objects are Rust objects, `core` is linked from its rlib, and nothing then supplies the
/// symbol `core`'s precompiled panic paths call - `rust_begin_unwind`, under the crate
/// disambiguator rustc minted when *core* was built.  A separately compiled crate cannot
/// reproduce that spelling (measured: a crate-local `#[panic_handler]` is exported as
/// `_RNvCs1njKG4L9aB3_7___rustc17rust_begin_unwind`), so the driver exports exactly the name the
/// objects import - read out of them by `unwind_symbol` below - and routes the lang item to it.
/// The handler aborts, which is what C does and what PORTING.md's constraint 9 (`panic = "abort"`,
/// never unwind into C) requires.
const SHARED_DRIVER: &str = "#![no_std]\n\n\
/* The lang item `core`'s precompiled code calls.  Its symbol carries the crate disambiguator\
   rustc minted for `__rustc` in the toolchain that built `core`, which is why this driver is\
   compiled with the *same* toolchain (`rustc_cmd`, the repository's pinned nightly) and why\
   `link-shared` verifies the definition afterwards instead of trusting it.  A panic aborts -\
   never unwinds into C (PORTING.md constraint 9).  */\n\
struct PanicBuf { b: [u8; 320], n: usize }\n\
impl core::fmt::Write for PanicBuf\n\
{\n\
  fn write_str (&mut self, s: &str) -> core::fmt::Result\n\
  {\n\
    let bytes = s.as_bytes ();\n\
    if bytes.len () > self.b.len () - self.n { return Err (core::fmt::Error) }\n\
    self.b[self.n..self.n + bytes.len ()].copy_from_slice (bytes);\n\
    self.n += bytes.len ();\n\
    Ok (())\n\
  }\n\
}\n\
/* A panic is a defect, and its message has to reach the device log: the default std hook never\n\
   runs here (this file supplies the lang item), which is why the statuses up to 4.864 had only\n\
   the backtrace and no source line (status 4.865).  */\n\
#[panic_handler]\n\
fn rust_begin_unwind (info: &core::panic::PanicInfo) -> !\n\
{\n\
  use core::fmt::Write;\n\
  let mut pb = PanicBuf { b: [0u8; 320], n: 0 };\n\
  /* Rust's precompiled panics do not all carry a `&str`/`String` payload: the formatted ones\n\
     (`index out of bounds: …`, `attempt to add with overflow`) carry a `PanicMessage`, which the\n\
     payload downcast could not name and reported as `?` (status 4.895).  `PanicInfo::message`\n\
     renders every payload, so that is what is printed.  */\n\
  let msg = info.message ();\n\
  let _ = write! (&mut pb, \"PANIC {}:{} {}\",\n\
                  info.location ().map (|l| l.file ()).unwrap_or (\"?\"),\n\
                  info.location ().map (|l| l.line ()).unwrap_or (0),\n\
                  msg);\n\
  pb.b[pb.n] = b'\\n'; pb.n += 1;\n\
  unsafe { write (2, pb.b.as_ptr (), pb.n as usize); abort () }\n\
}\n\
\n\
/* Rust's unwind entry point.  With `panic = abort` nothing unwinds, and C's build has no such\n\
   symbol at all; the name is a requirement of the Rust runtime objects that `-nodefaultlibs`\n\
   links here, so the driver provides it with `_Unwind`'s own signature and aborts if it is ever\n\
   reached (status 4.726).  */\n\
#[unsafe(export_name = \"rust_eh_personality\")]\n\
pub extern \"C\" fn rust_eh_personality (_version: i32, _actions: i32, _class: u64,\n\
                                      _object: *mut core::ffi::c_void,\n\
                                      _context: *mut core::ffi::c_void) -> i32\n\
{ unsafe { abort () } }\n\n\
unsafe extern \"C\" { fn abort () -> !; fn write (fd: i32, buf: *const u8, n: usize) -> isize; }\n\
\n\
/* `core`'s own panics carry a `&str`; `panic! (\"{x}\")` in precompiled code can carry a\n\
   `String`, which the first version reported as `?` (status 4.895).  The type is named through\n\
   `alloc`, whose rlib the link already passes.  */\n\
extern crate alloc as alloc_crate;\n\
use alloc_crate::string::String as alloc_string;\n";

/// `xtask link-shared <target>` - link the port's objects for TARGET into `libemacs.so`.
///
/// `src/Makefile.in` 727-755 builds Android's `libemacs.so` with `-shared` over `$(ALLOBJS)`;
/// the port's equivalent is its per-target unit objects (`build/femacs/<unit>.<target>.o`, written
/// by `build-tu` under `FEMACS_TARGET`), linked as a Rust `cdylib` so the language runtime's own
/// symbols come along.  No `libgnu.a` is passed: a port unit carries the gnulib mirrors it
/// includes, which is why the first link needed none - it left three undefined symbols, all of
/// them bionic/libc++ (`__cxa_atexit`, `__cxa_finalize`, `rust_eh_personality`), measured with
/// `llvm-nm -D --undefined-only` (status 4.614).
///
/// The linker is the NDK's cross compiler, named by `FEMACS_ANDROID_CC` or found under
/// `$ANDROID_HOME`/`$HOME/Library/Android/sdk` the way `tools/gen-android-config.sh` finds it.
fn link_shared (target: Option<&str>, manifest: Option<&Path>) -> i32
{
  let Some(target) = target else
    {
      return fail ("link-shared needs a target, e.g. aarch64-linux-android");
    };
  let target = target.to_string ();
  let build = units_dir ();
  if manifest.is_some_and(|p|p.file_name().is_none_or(|s|s!="core-units.txt") || !p.is_file()) {
      return fail("--units-manifest must name the existing owned core-units.txt beside lib-units.txt");
  }
  let checking=std::env::var("FEMACS_LINK_CHECKING").is_ok_and(|v|v=="1");
  let support=manifest.and_then(Path::parent).map(Path::to_owned)
    .unwrap_or_else(||android::support_directory(android::BuildTarget::AndroidArm64));
  let units=match android::load_units(&support){Ok(v)=>v,Err(e)=>return fail(e)};
  let objects=match android::expected_objects(&units,&build,checking){Ok(v)=>v,Err(e)=>return fail(e)};
  let Some (linker) = android_linker (&target)
    else
      {
        return fail ("no Android cross compiler: set FEMACS_ANDROID_CC, or install the NDK\
                      under $ANDROID_HOME / $HOME/Library/Android/sdk");
      };
  let dir = port_dir ().join (&target);
  if let Err (e) = fs::create_dir_all (&dir)
    {
      return fail (format! ("creating {}: {e}", dir.display ()));
    }
  if let Err(e)=fs::create_dir_all(load_dir()){return fail(e);}
  let mut rsp: Vec<String> = Vec::new ();
  for object in &objects
    {
      rsp.push (fs::canonicalize (object).map (|p| p.display ().to_string ())
                  .unwrap_or_else (|_| object.display ().to_string ()));
    }
  if let Err (e) = fs::write (load_dir ().join ("link-shared-args.rsp"), rsp.join ("\n") + "\n")
    {
      return fail (format! ("writing the response file: {e}"));
    }
  let out_path = dir.join ("libemacs.so");
  println! ("== link-shared: {target} ==");
  println! ("   {} object(s) -> {}", objects.len (), out_path.display ());
  // The link goes through the NDK's own clang, the way C's Android build does, and **not**
  // through `rustc --crate-type cdylib`: a cdylib's version script lists only the driver
  // crate's own items, so every symbol the 91 objects define stayed invisible
  // (`--dyn-syms` had nothing but the table header, status 4.669).  clang keeps every
  // `#[no_mangle]` definition visible by default, which is exactly C's export set - the 69
  // `Java_org_femacs_EmacsNative_*` entry points included.
  let sysroot = match rustc_cmd ().arg ("--print").arg ("sysroot").output ()
    {
      Ok (o) if o.status.success () =>
        PathBuf::from (String::from_utf8_lossy (&o.stdout).trim ()),
      _ => return fail ("rustc --print sysroot failed: the target's core rlib is needed"),
    };
  let rlib_dir = sysroot.join ("lib/rustlib").join (&target).join ("lib");
  let pick = |prefix: &str| -> Option<PathBuf>
    {
      let mut hits: Vec<PathBuf> = fs::read_dir (&rlib_dir).ok ()?.flatten ()
        .map (|e| e.path ())
        .filter (|p| p.file_name ().and_then (|n| n.to_str ())
                 .map (|n| n.starts_with (prefix) && n.ends_with (".rlib")).unwrap_or (false))
        .collect ();
      hits.sort ();
      hits.pop ()
    };
  let (Some (core), Some (builtins)) = (pick ("libcore-"), pick ("libcompiler_builtins-"))
    else { return fail (format! ("no core/compiler_builtins rlib under {}", rlib_dir.display ())) };
  // `alloc` is the third runtime rlib the objects need: the port's own `Vec`/`Box`/`String`
  // code names generic instances (`<String as Clone>::clone`, `CString::new`) whose bodies only
  // exist in `alloc`'s precompiled objects, exactly as `core`'s panic path only exists in
  // `core`'s.  Linking the rlib resolves them the way rustc would; `alloc`'s own calls back
  // into the global allocator land on the shim `alloc_shim_source` writes (status 4.866).
  let alloc = pick ("liballoc-");
  // The panic entry point.  `core`'s own code references the lang item `rust_begin_unwind` under
  // the crate disambiguator rustc minted for `core` - a different spelling per toolchain
  // (measured: `_RNvCs1njKG4L9aB3_...` from stable 1.98.1, `_RNvCs8kYXCe6bXIp_...` from the pinned
  // nightly) - and nothing in the link defines it, so the driver is compiled for the target and
  // exports exactly the name the inputs import, read out of them rather than guessed.
  let mut driver: Vec<PathBuf> = Vec::new ();
  let mut inputs: Vec<PathBuf> = objects.clone ();
  inputs.push (core.clone ());
  inputs.push (builtins.clone ());
  if let Some (alloc) = &alloc { inputs.push (alloc.clone ()); }
  match ndk_bin (&linker).and_then (|bin| unwind_symbol (&bin, &inputs))
    {
      Some (name) =>
        {
          // `-nodefaultlibs` also leaves `alloc`'s global-allocator shim undefined: the port's
          // own `Vec`/`Box` code (emacs.rs) calls `__rust_alloc`/`__rust_dealloc`, whose bodies
          // `std` normally supplies.  The shim symbols are read out of the objects and generated
          // here on top of `malloc`/`free` - the same libc allocation C's `xmalloc` uses, so the
          // port's heap accounting and `malloc` hooks stay the reference's (status 4.866).
          let mut source = String::from (SHARED_DRIVER);
          source.push_str (&alloc_shim_source (&ndk_bin (&linker).unwrap (), &inputs));
          if let Err (e) = fs::write (load_dir ().join ("shared-driver.rs"), &source)
            {
              return fail (format! ("writing the shared driver: {e}"));
            }
          let object = load_dir ().join (format! ("shared-driver.{target}.o"));
          let mut cmd = rustc_cmd ();
          cmd.arg ("--target").arg (&target).arg ("-O").arg ("--crate-type").arg ("lib")
             .arg ("--emit=obj").arg ("-o").arg (&object)
             .arg (load_dir ().join ("shared-driver.rs"));
          match cmd.output ()
            {
              Ok (o) if o.status.success () => driver.push (object),
              Ok (o) => return fail (format! ("compiling the shared driver: {}",
                                              String::from_utf8_lossy (&o.stderr).trim ())),
              Err (e) => return fail (format! ("rustc for the shared driver: {e}")),
            }
          // The point of the whole exercise is that this exact spelling exists, so it is checked
          // here rather than trusted: a definition of `name` in the driver's object, or the link
          // goes on and the device refuses the `dlopen` (P-300).
          let Some (bin) = ndk_bin (&linker) else { return fail ("no NDK bin directory") };
          match defines (&bin, &driver[0], &name)
            {
              Ok (true) => println! ("   panic driver: {name}"),
              Ok (false) => return fail (format! ("the driver does not define {name}")),
              Err (e) => return fail (format! ("llvm-nm on the driver: {e}")),
            }
          for (shim, _) in alloc_shims (&bin, &inputs)
            {
              match defines (&bin, &driver[0], &shim)
                {
                  Ok (true) => println! ("   alloc shim: {shim}"),
                  Ok (false) => return fail (format! ("the driver does not define {shim}")),
                  Err (e) => return fail (format! ("llvm-nm on the driver: {e}")),
                }
            }
        }
      None => println! ("   no rust_begin_unwind import: no panic driver needed"),
    }
  let mut cmd = Command::new (&linker);
  cmd.arg ("-shared").arg ("-o").arg (&out_path)
     // `-z relro,-z now` stays: the NDK driver defaults to `-z now` anyway (measured with
     // `llvm-readelf -d` on a scratch link), so it is the target's binding either way, and
     // `check-android-imports.py` reports which binding the built library has.
     .args (["-Wl,--gc-sections", "-Wl,-z,relro,-z,now", "-nodefaultlibs"])
     // `-nodefaultlibs` (below) drops the driver's own `-lc`, and bionic resolves a `dlopen`ed
     // library's undefined symbols through that library's `DT_NEEDED` list alone - libc is not
     // implicit.  Without `-lc` here the library has no `malloc` at all, and the device reports
     // `dlopen failed: cannot locate symbol "malloc" referenced by ... libemacs.so` (measured
     // 4.707; the C build's `libemacs.so` carries libc.so through the driver's defaults).
     // `-llog -ljnigraphics` (and `-landroid`, which `configure.ac` 2634-2635 adds when
     // `android_get_device_api_level` exists) are the reference's `ANDROID_LIBS`; `-lc` and
     // `-ldl -lm` are what the driver would have added if `-nodefaultlibs` were absent.
     // `-lz` is C's `LIBZ` (`src/Makefile.in` 266, from `configure.ac` 4986's
     // `emacs_cv_libz`); the Android configuration has `HAVE_ZLIB`, so `decompress.c`'s
     // `inflate*` calls are compiled and the stub library provides them (measured on
     // `sysroot/.../21/libz.so`, status 4.717).
     .args (["-ldl", "-lm", "-llog", "-ljnigraphics", "-landroid", "-lz", "-lc"]);
  for object in &objects { cmd.arg (object); }
  for object in &driver { cmd.arg (object); }
  cmd.arg (&core).arg (&builtins);
  if let Some (alloc) = &alloc { cmd.arg (alloc); }
  let out = match cmd.output ()
  {
    Ok (o) => o,
    Err (e) => return fail (format! ("clang link: {e}")),
  };
  if !out.status.success () || !out_path.is_file ()
    {
      return fail (format! ("link-shared failed: {}",
                            String::from_utf8_lossy (&out.stderr).trim ()));
    }
  println! ("linked {} ({} bytes)", out_path.display (),
            fs::metadata (&out_path).map (|m| m.len ()).unwrap_or (0));
  0
}

/// The tool directory the NDK cross compiler lives in, where its `llvm-nm`/`llvm-readelf` are.
fn ndk_bin (linker: &str) -> Option<PathBuf>
{
  Path::new (linker).parent ().map (|p| p.to_path_buf ())
}

/// The `rust_begin_unwind` spelling the objects import, or `None` when they import none.
fn unwind_symbol (bin: &Path, objects: &[PathBuf]) -> Option<String>
{
  let mut cmd = Command::new (bin.join ("llvm-nm"));
  cmd.arg ("-u");
  for object in objects { cmd.arg (object); }
  let out = cmd.output ().ok ()?;
  let text = String::from_utf8_lossy (&out.stdout).into_owned ();
  text.lines ()
      .map (|line| line.rsplit (' ').next ().unwrap_or ("").trim ().to_string ())
      .find (|name| name.contains ("17rust_begin_unwind"))
}

/// The allocator shim symbols the inputs import, paired with the shim body each one needs.
///
/// `alloc`'s precompiled half expects the global allocator to be reachable under these names,
/// exactly as `core`'s panic path expects `rust_begin_unwind`; the spelling carries the
/// toolchain's `__rustc` crate disambiguator, so it is read out of the objects, never guessed.
/// The suffixes are the stable tails of the three symbols rustc emits for `#[global_allocator]`.
fn alloc_shims (bin: &Path, objects: &[PathBuf]) -> Vec<(String, &'static str)>
{
  let mut cmd = Command::new (bin.join ("llvm-nm"));
  cmd.arg ("-u");
  for object in objects { cmd.arg (object); }
  let Ok (out) = cmd.output () else { return Vec::new () };
  let text = String::from_utf8_lossy (&out.stdout).into_owned ();
  let mut names: Vec<String> = Vec::new ();
  for line in text.lines ()
    {
      let name = line.rsplit (' ').next ().unwrap_or ("").trim ();
      if ! names.iter ().any (|n| n == name) { names.push (name.to_string ()); }
    }
  let mut shims: Vec<(String, &'static str)> = Vec::new ();
  for (suffix, kind) in [("35___rust_no_alloc_shim_is_unstable_v2", "unstable"),
                         ("12___rust_alloc", "alloc"),
                         ("14___rust_dealloc", "dealloc"),
                         ("15___rust_realloc", "realloc"),
                         ("20___rust_alloc_zeroed", "alloc_zeroed")]
    {
      if let Some (name) = names.iter ().find (|n| n.ends_with (suffix))
        {
          shims.push ((name.clone (), kind));
        }
    }
  shims
}

/// Rust source for the allocator shims above, appended to `SHARED_DRIVER`.
///
/// Every body lands on libc `malloc`/`free`/`realloc`: the reference's `xmalloc` is a wrapper
/// over the same calls, so an allocation the port makes through `Vec` is accounted for, and
/// freed, exactly where C's is.  Over-aligned requests (align > 16, bionic malloc's own
/// guarantee) go through `posix_memalign`, which `free` accepts.
fn alloc_shim_source (bin: &Path, objects: &[PathBuf]) -> String
{
  let shims = alloc_shims (bin, objects);
  if shims.is_empty () { return String::new (); }
  let mut src = String::from ("\nunsafe extern \"C\" {\n  fn malloc (n: usize) -> *mut core::ffi::c_void;\n  fn free (p: *mut core::ffi::c_void);\n  fn realloc (p: *mut core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;\n  fn posix_memalign (p: *mut *mut core::ffi::c_void, align: usize, size: usize) -> i32;\n}\n");
  src.push_str ("unsafe fn femacs_alloc_impl (size: usize, align: usize) -> *mut u8\n{\n  if align <= 16 { return malloc (size) as *mut u8 }\n  let mut p: *mut core::ffi::c_void = core::ptr::null_mut ();\n  if posix_memalign (&mut p, align, size) != 0 { return core::ptr::null_mut () }\n  p as *mut u8\n}\n");
  src.push_str ("unsafe fn femacs_dealloc_impl (p: *mut u8) { free (p as *mut core::ffi::c_void) }\n");
  src.push_str ("unsafe fn femacs_realloc_impl (p: *mut u8, old: usize, new: usize, align: usize) -> *mut u8\n{\n  if p.is_null () { return femacs_alloc_impl (new, align) }\n  if align <= 16 { return realloc (p as *mut core::ffi::c_void, new) as *mut u8 }\n  let q = femacs_alloc_impl (new, align);\n  if ! q.is_null ()\n    {\n      core::ptr::copy_nonoverlapping (p, q, core::cmp::min (old, new));\n      femacs_dealloc_impl (p);\n    }\n  q\n}\n");
  src.push_str ("unsafe fn femacs_alloc_zeroed_impl (size: usize, align: usize) -> *mut u8\n{\n  let p = femacs_alloc_impl (size, align);\n  if ! p.is_null () { core::ptr::write_bytes (p, 0, size) }\n  p\n}\n");
  for (name, kind) in shims
    {
      let text = match kind
        {
          "unstable" => format! ("#[unsafe(export_name = \"{name}\")]\npub static FEMACS_ALLOC_SHIM_V2: u8 = 0;\n"),
          "alloc" => format! ("#[unsafe(export_name = \"{name}\")]\npub unsafe fn femacs_alloc (size: usize, align: usize) -> *mut u8\n{{ femacs_alloc_impl (size, align) }}\n"),
          "dealloc" => format! ("#[unsafe(export_name = \"{name}\")]\npub unsafe fn femacs_dealloc (p: *mut u8, _size: usize, _align: usize)\n{{ femacs_dealloc_impl (p) }}\n"),
          "realloc" => format! ("#[unsafe(export_name = \"{name}\")]\npub unsafe fn femacs_realloc (p: *mut u8, old: usize, new: usize, align: usize) -> *mut u8\n{{ femacs_realloc_impl (p, old, new, align) }}\n"),
          _ => format! ("#[unsafe(export_name = \"{name}\")]\npub unsafe fn femacs_alloc_zeroed (size: usize, align: usize) -> *mut u8\n{{ femacs_alloc_zeroed_impl (size, align) }}\n"),
        };
      src.push_str (&text);
    }
  src
}

/// Whether OBJECT defines SYMBOL (an `nm` type letter other than `U`).
fn defines (bin: &Path, object: &Path, symbol: &str) -> Result<bool, String>
{
  let out = Command::new (bin.join ("llvm-nm")).arg (object).output ()
    .map_err (|e| e.to_string ())?;
  let text = String::from_utf8_lossy (&out.stdout).into_owned ();
  Ok (text.lines ().any (|line|
    {
      let mut fields = line.split_whitespace ();
      match (fields.next (), fields.next (), fields.next ())
        {
          (Some (_addr), Some (kind), Some (name)) => kind != "U" && kind != "w" && name == symbol,
          (Some (kind), Some (name), None) => kind != "U" && kind != "w" && name == symbol,
          _ => false,
        }
    }))
}

/// The NDK cross compiler for TARGET: `FEMACS_ANDROID_CC`, else the newest NDK under the SDK.
fn android_linker (target: &str) -> Option<String>
{
  if let Ok (cc) = std::env::var ("FEMACS_ANDROID_CC")
    {
      if !cc.is_empty () { return Some (cc); }
    }
  let api = std::env::var ("ANDROID_NDK_API").unwrap_or_else (|_| "21".to_string ());
  let sdk = std::env::var ("ANDROID_HOME").ok ().map (PathBuf::from).or_else (||
    std::env::var ("HOME").ok ().map (|h| Path::new ("/").join (h).join ("Library/Android/sdk")))
    ?;
  let triple = match target
    {
      "aarch64-linux-android" => format! ("aarch64-linux-android{api}-clang"),
      "x86_64-linux-android" => format! ("x86_64-linux-android{api}-clang"),
      "armv7-linux-androideabi" => format! ("armv7a-linux-androideabi{api}-clang"),
      _ => return None,
    };
  let mut found: Vec<PathBuf> = Vec::new ();
  for entry in fs::read_dir (sdk.join ("ndk")).ok ()?
    {
      let entry = entry.ok ()?;
      let prebuilt = entry.path ().join ("toolchains/llvm/prebuilt");
      for host in fs::read_dir (&prebuilt).ok ()?.flatten ()
        {
          let candidate = host.path ().join ("bin").join (&triple);
          if candidate.is_file () { found.push (candidate); }
        }
    }
  found.sort ();
  found.pop ().map (|p| p.display ().to_string ())
}

fn target_manifest(name: &str) -> Result<Vec<String>, String> {
    let target=android::build_target()?;
    let path=android::support_directory(target).join(name);
    let entries = read_manifest(&path)?;
    if name == "objects.txt" { runtime::product_objects(Path::new(REPO), entries) }
    else { Ok(entries) }
}

fn read_manifest(path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(text.split_whitespace().map(str::to_string).collect())
}

/// The port's build tree: `src/temacs` plus the trees the port owns under their own names.
///
/// C's build tree and its source tree are one tree (`lib-src/`, `etc/`, `lisp/` beside `src/`),
/// and the port's trees are the verbatim copies under `` (`lisp/`, `etc/`, `leim/`,
/// `doc/`, `test/`, `admin/`).  Links keep `build/` a build artifact rather than a second copy
/// (`build/` is git-ignored), and `lib-src/` is a real directory because that is where the
/// program's own outputs go (`emacs.pdmp` for a dumped build).
fn port_tree(root: &Path) -> std::io::Result<()> {
    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("lib-src"))?;
    for name in ["lisp", "leim", "etc", "doc", "test", "admin"] {
        // The *target string* is relative to the link's own directory (`build/port/`, hence two
        // levels up), while the existence test has to be about the real tree the port owns - the
        // relative string resolves against this process's cwd and would skip every link.
        if !Path::new(REPO).join(name).exists() {
            continue;                       // a tree the port does not own is not linked
        }
        // Absolute target: the port tree is no longer always `build/port` (PGO phases live
        // under `build/pgo/<phase>/port`), so a fixed `../..` would point at the phase root.
        let target = fs::canonicalize(Path::new(REPO).join(name))
            .unwrap_or_else(|_| Path::new(REPO).join(name));
        let link = root.join(name);
        // `symlink` is Unix-only, which is all this driver needs (`std::os::unix::fs`).
        let _ = fs::remove_file(&link);
        match std::os::unix::fs::symlink(&target, &link) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// The 32-byte placeholder in `lib/fingerprint.rs`.  `make-fingerprint` replaces
/// exactly this array once per linked binary; the pdumper compares the value stored in the
/// dump with the loading binary's array, so leaving the placeholder in place would make every
/// dump look compatible with every binary.
const FINGERPRINT_PLACEHOLDER: [u8; 32] =
  [
   0xDE, 0x86, 0xBB, 0x99, 0xFF, 0xF5, 0x46, 0x9A,
   0x9E, 0x3F, 0x9F, 0x5D, 0x9A, 0xDF, 0xF0, 0x91,
   0xBD, 0xCD, 0xC1, 0xE8, 0x0C, 0x16, 0x1E, 0xAF,
   0xB8, 0x6C, 0xE2, 0x2B, 0xB1, 0x24, 0xCE, 0xB0,
  ];

fn count_in_file (path: &Path, needle: &[u8]) -> std::io::Result<usize>
{
  let mut f = fs::File::open (path)?;
  let mut buf = vec![0u8; 1 << 16];
  let mut carry: Vec<u8> = Vec::new ();
  let mut count = 0usize;
  loop
    {
      let n = f.read (&mut buf)?;
      if n == 0 { break; }
      let mut data = Vec::with_capacity (carry.len () + n);
      data.extend_from_slice (&carry);
      data.extend_from_slice (&buf[..n]);
      let mut pos = 0usize;
      while pos + needle.len () <= data.len ()
        {
          if &data[pos..pos + needle.len ()] == needle
            {
              count += 1;
              pos += needle.len ();
            }
          else { pos += 1; }
        }
      let keep = (needle.len () - 1).min (data.len ());
      carry = data[data.len () - keep..].to_vec ();
    }
  Ok (count)
}

fn find_in_file (path: &Path, needle: &[u8]) -> std::io::Result<Option<u64>>
{
  let mut f = fs::File::open (path)?;
  let mut buf = vec![0u8; 1 << 16];
  let mut carry: Vec<u8> = Vec::new ();
  let mut consumed = 0u64;
  loop
    {
      let n = f.read (&mut buf)?;
      if n == 0 { break; }
      let start = consumed - carry.len () as u64;
      let mut data = Vec::with_capacity (carry.len () + n);
      data.extend_from_slice (&carry);
      data.extend_from_slice (&buf[..n]);
      if let Some (pos) = data.windows (needle.len ()).position (|w| w == needle)
        {
          return Ok (Some (start + pos as u64));
        }
      consumed += n as u64;
      let keep = (needle.len () - 1).min (data.len ());
      carry = data[data.len () - keep..].to_vec ();
    }
  Ok (None)
}

fn hex_digest (text: &str) -> Option<[u8; 32]>
{
  if text.len () != 64 { return None; }
  let mut out = [0u8; 32];
  for i in 0..32
    {
      out[i] = u8::from_str_radix (&text[i * 2..i * 2 + 2], 16).ok ()?;
    }
  Some (out)
}

/// Link-time post step: rewrite the fingerprint, then re-sign the Mach-O.
///
/// Reference order (`emacs/src/Makefile.in` 713-720): link to `temacs.tmp`,
/// `MAKE_PDUMPER_FINGERPRINT $@.tmp`, `codesign -s - -f $@.tmp`.  The rewrite changes
/// bytes inside the signed Mach-O, so re-signing is mandatory before the binary may run.
/// The tool itself comes from the plain-O2 object root (`link_program`), never from the
/// instrumented phase, so building or running it cannot pollute the training profiles.
fn finalize_binary (path: &Path) -> Result<(), String>
{
  let before = count_in_file (path, &FINGERPRINT_PLACEHOLDER)
    .map_err (|e| format! ("{}: reading fingerprint placeholder: {e}", path.display ()))?;
  if before != 1
    {
      return Err (format! ("{}: expected exactly one fingerprint placeholder, found {before}",
                           path.display ()));
    }
    let tool_rel = "lib-src/make-fingerprint.c".to_string ();
    if cargo_build::build_tool_program (Some (&tool_rel)) != 0
      {
        return Err ("building make-fingerprint failed".to_string ());
      }
  let tool = port_dir ().join ("lib-src").join ("make-fingerprint");
  if ! tool.is_file ()
    {
      return Err (format! ("make-fingerprint was not linked at {}", tool.display ()));
    }
  let raw = Command::new (&tool).arg ("-r").arg (path).output ()
    .map_err (|e| format! ("running make-fingerprint -r: {e}"))?;
  if ! raw.status.success ()
    {
      return Err (format! ("make-fingerprint -r failed: {}",
                           String::from_utf8_lossy (&raw.stderr).trim ()));
    }
  let digest_hex = String::from_utf8_lossy (&raw.stdout).trim ().to_string ();
  let digest = hex_digest (&digest_hex)
    .ok_or_else (|| format! ("make-fingerprint -r printed a malformed digest: {digest_hex}"))?;
  let offset = find_in_file (path, &FINGERPRINT_PLACEHOLDER)
    .map_err (|e| format! ("{}: locating the placeholder: {e}", path.display ()))?
    .ok_or_else (|| format! ("{}: placeholder disappeared before the rewrite", path.display ()))?;
  let status = Command::new (&tool).arg (path).status ()
    .map_err (|e| format! ("running make-fingerprint: {e}"))?;
  if ! status.success ()
    {
      return Err (format! ("make-fingerprint failed for {}", path.display ()));
    }
  let after = count_in_file (path, &FINGERPRINT_PLACEHOLDER)
    .map_err (|e| format! ("{}: re-reading the placeholder: {e}", path.display ()))?;
  if after != 0
    {
      return Err (format! ("{}: fingerprint placeholder survived the rewrite", path.display ()));
    }
  let mut f = fs::File::open (path)
    .map_err (|e| format! ("{}: {e}", path.display ()))?;
  f.seek (SeekFrom::Start (offset)).map_err (|e| format! ("{}: seek: {e}", path.display ()))?;
  let mut got = [0u8; 32];
  f.read_exact (&mut got).map_err (|e| format! ("{}: read digest: {e}", path.display ()))?;
  if got != digest
    {
      return Err (format! ("{}: stored fingerprint does not match make-fingerprint -r",
                           path.display ()));
    }
  if cfg! (target_os = "macos")
    {
      let signed = Command::new ("codesign").args (["-s", "-", "-f"]).arg (path).status ()
        .map_err (|e| format! ("running codesign: {e}"))?;
      if ! signed.success ()
        {
          return Err (format! ("codesign -s - -f failed for {}", path.display ()));
        }
      let verified = Command::new ("codesign").arg ("--verify").arg (path).status ()
        .map_err (|e| format! ("running codesign --verify: {e}"))?;
      if ! verified.success ()
        {
          return Err (format! ("codesign --verify failed for {}", path.display ()));
        }
    }
  println! ("fingerprint: {} -> {digest_hex}", path.display ());
  Ok (())
}

/// The reference's dump chain, in `src/Makefile` order.
///
/// `emacs$(EXEEXT)` (Makefile 630-636) is a copy of the linked program - `rm -f $@ && cp -f
/// temacs$(EXEEXT) $@` - and `$(pdmp)` (651-655) then runs that program from the `src`
/// directory: `LC_ALL=C $(RUN_TEMACS) -batch $(BUILD_DETAILS) -l loadup --temacs=pdump
/// --bin-dest '$(BIN_DESTDIR)' --eln-dest '$(ELN_DESTDIR)'`, followed by `cp -f $@
/// $(bootstrap_pdmp)` where `bootstrap_pdmp` is `bootstrap-emacs.pdmp` for `DUMPING=pdumper`
/// (404).  The frozen configuration's `BIN_DESTDIR`/`ELN_DESTDIR` are `not-set` and
/// `BUILD_DETAILS` is empty, so the command line below is the reference's own, verbatim.
///
/// Without the copy the dump cannot finish: loadup's last step names the dumped program
/// `emacs-<version>.<build-number>` with `add-name-to-file`, which needs the file the
/// Makefile rule creates (measured 2026-10-16: the dump itself completed and the naming step
/// failed with `file-missing`).
fn dump_emacs() -> i32 { dump_emacs_mode(false) }

fn dump_emacs_mode(bootstrap: bool) -> i32 {
    let src = port_dir().join("src");
    let temacs = src.join("temacs");
    if !temacs.is_file() {
        return fail("build/port/src/temacs is missing; `xtask gate-load` links it");
    }
    let emacs = src.join("emacs");
    let _ = fs::remove_file(&emacs);
    if let Err(e) = fs::copy(&temacs, &emacs) {
        return fail(format!("cp -f temacs emacs: {e}"));
    }
    let mut dump = Command::new("./temacs");
    dump.args(["-batch", "-l", "loadup", if bootstrap { "--temacs=pbootstrap" } else { "--temacs=pdump" },
               "--bin-dest", "not-set", "--eln-dest", "not-set"])
        .current_dir(&src)
        .env("LC_ALL", "C");
    // The dump run executes the instrumented temacs in the generate phase.  Its profile
    // belongs to the build machinery, not to the training corpus: keep it in its own
    // directory so only profiles/train can ever be merged.
    set_profile_env(&mut dump, "dump");
    let status = dump.status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => return fail(format!("the dump program exited with {s}")),
        Err(e) => return fail(format!("running ./temacs: {e}")),
    }
    let pdmp = src.join("emacs.pdmp");
    if bootstrap {
        if let Err(e) = fs::copy(src.join("bootstrap-emacs.pdmp"), &pdmp) { return fail(e); }
    }
    let bootstrap = src.join("bootstrap-emacs.pdmp");
    if let Err(e) = fs::copy(&pdmp, &bootstrap) {
        return fail(format!("cp -f emacs.pdmp bootstrap-emacs.pdmp: {e}"));
    }
    match fs::metadata(&pdmp) {
        Ok(m) => println!("dumped {} ({} bytes)", pdmp.display(), m.len()),
        Err(e) => return fail(format!("emacs.pdmp: {e}")),
    }
    if let Err(e) = write_pair_manifest(&src) {
        return fail(e);
    }
    0
}

/// Record the binary/dump quartet so the measurement tool can refuse a stale pair.
/// The fingerprint is the primary check; this manifest also catches an accidental
/// copy of one phase's dump into another phase's directory.
fn write_pair_manifest(src: &Path) -> Result<(), String> {
    let names = ["temacs", "emacs", "emacs.pdmp", "bootstrap-emacs.pdmp"];
    let files: Vec<PathBuf> = names.iter().map(|n| src.join(n)).collect();
    for f in &files {
        if !f.is_file() {
            return Err(format!("cannot hash missing {}", f.display()));
        }
    }
    let mut cmd = Command::new("shasum");
    cmd.args(["-a", "256"]);
    for f in &files {
        cmd.arg(f);
    }
    let out = cmd.output().map_err(|e| format!("shasum: {e}"))?;
    if !out.status.success() {
        return Err(format!("shasum failed: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut rows: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(2, "  ");
        let (Some(digest), Some(path)) = (parts.next(), parts.next()) else {
            return Err(format!("malformed shasum row: {line}"));
        };
        let name = Path::new(path).file_name().unwrap_or_default().to_string_lossy().to_string();
        rows.push((name, digest.to_string()));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    let mut manifest = String::new();
    for (name, digest) in rows {
        manifest.push_str(&format!("{digest}  {name}\n"));
    }
    fs::write(src.join("pair.sha256"), manifest)
        .map_err(|e| format!("writing pair.sha256: {e}"))
}

/// The C-differential tier: every probe that compares the C reference with the mirrors.
fn gate_probes() -> i32 {
    for name in ["inline","accessor","write","va","syshdrs","gmp-abi","lisp-abi","jni"] {
        println!("== {name} ==");
        if let Err(e)=probes::run_probe(Path::new(REPO),name,&probes::ProbeOptions::default()){return fail(e);}
    }
    0
}

fn canonical(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}


/// The `include!("…")` target of a line, if the line is such a call.
fn include_target(line: &str) -> Option<&str> {
    let pos = line.find("include!(")?;
    let rest = &line[pos + "include!(".len()..];
    let end = rest.find(')')?;
    rest[..end].trim().strip_prefix('"').and_then(|s| s.strip_suffix('"'))
}

/// Materialize ROOT's include graph under OUT and return the generated crate root.
///
/// PORTING.md 0.1-4: the tool expands the graph and de-duplicates it, because `include!` has no
/// include guard.  A file reached by two paths (`buffer.rs`'s `<time.h>` and `systime.rs`'s
/// `<timespec.h>` both reach `time.in.rs`) would otherwise define its items twice and rustc
/// would reject the unit.  Every reachable file is therefore written out once, at the position
/// of the `include!` that first reaches it in C's textual order - so a mirror included by
/// `lisp.h`'s mirror is expanded *inside* it and sees the macros `lisp.h` defines above its
/// `#include` line (`eassert!`/`XUNTAG!`), which is what C does and what the old
/// "dependencies first" flattening could not express (see docs/ports/include-thread.md's
/// "Changes outside the unit").  A later reach of an already-expanded file becomes a
/// dedupe comment, i.e. the include guard.  Line numbers inside a copy still point into the
/// real mirror, so diagnostics name a file and line a reader can look at.
fn materialize(root: &Path, out: &Path) -> Result<PathBuf, String> {
    let _ = fs::remove_dir_all(out);
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    // Absolute from here on: a copy names another copy with `include!`, which resolves
    // relative to the file holding the call.
    let out = canonical(out);
    let root = canonical(root);
    let mut expand = Expand { out: out.clone(), root: root.clone(), seen: vec![root.clone()],
                              ungated: ungated_reachable(&root) };
    let text = expand.render(&root, false)?;
    let root_file = out.join("_root.rs");
    fs::write(&root_file, text).map_err(|e| format!("{}: {e}", root_file.display()))?;
    // Absolute: the callers that run rustc with a different working directory (dead-arms)
    // hand it straight to `rustc`.
    Ok(canonical(&root_file))
}

/// The in-place expansion state: the file the walk started from (whose text is the crate
/// root) and the files already expanded, in the order the preprocessor would see them.
/// Walk TRIM from bracket depth DEPTH, ignoring brackets inside string literals, and return
/// the depth when the line ends if the attribute closed, or `None` when it is still open (the
/// next line continues it).  The `attr_open`/`attr_depth` pair this serves is `render`'s gate
/// bookkeeping (see the note there).
fn scan_brackets(trim: &str, depth: i32) -> (i32, bool) {
    let mut depth = depth;
    let mut in_string = false;
    let mut escaped = false;
    // `depth == 0` before the first `[` means "the attribute has not opened yet", not "it
    // closed": returning closed on the `#` of `#[cfg …` is what made every attribute read as
    // an empty one.
    let mut opened = depth > 0;
    for ch in trim.chars() {
        if in_string {
            if escaped { escaped = false; }
            else if ch == '\\' { escaped = true; }
            else if ch == '"' { in_string = false; }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '[' | '(' | '{' => { depth += 1; opened = true; }
            ']' | ')' | '}' => depth -= 1,
            _ => {}
        }
        if opened && depth == 0 { return (0, true); }
    }
    (depth, opened && depth == 0)
}

struct Expand {
    out: PathBuf,
    root: PathBuf,
    seen: Vec<PathBuf>,
    /// The files an *ungated* path reaches from the root (`ungated_reachable`).  A `#[cfg]`-gated
    /// `include!` must not claim the one expansion slot of one of these: the ungated reach is live
    /// in every configuration, so it - not the gated one - is the reach that has to expand the
    /// file.  See the note on `render`.
    ungated: std::collections::BTreeSet<PathBuf>,
}

impl Expand {
    /// The copy that carries PATH's own text.  Derived from the absolute path alone, so a
    /// file may name a copy that is written later in the walk.
    fn copy_path(&self, path: &Path) -> PathBuf {
        let name = path.to_string_lossy().replace(['/', '\\', ':', '.'], "_");
        self.out.join(format!("{name}.rs"))
    }

    /// PATH's text with every `include!` line resolved the way C's `#include` would be: the
    /// first file the walk reaches at that point is expanded recursively, later reaches are
    /// include-guard no-ops.  ROOT's text is returned (it becomes `_root.rs`); every other
    /// file is written to its copy first.
    fn render(&mut self, path: &Path, path_gated: bool) -> Result<String, String> {
        let input = if path.ends_with("src/include/config.rs") {
            std::env::var_os("FEMACS_CONFIG_RS").map(PathBuf::from).unwrap_or_else(|| path.to_path_buf())
        } else if path.ends_with("src/include/globals.rs") {
            std::env::var_os("FEMACS_GLOBALS_RS").map(PathBuf::from).unwrap_or_else(|| path.to_path_buf())
        } else if path.ends_with("src/include/epaths.rs") {
            std::env::var_os("FEMACS_EPATHS_RS").map(PathBuf::from).unwrap_or_else(|| path.to_path_buf())
        } else { path.to_path_buf() };
        let text = fs::read_to_string(&input).map_err(|e| format!("{}: {e}", input.display()))?;
        let dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut out = String::with_capacity(text.len() + 64);
        // Whether the `include!` on each line is under an attribute (`gate_states`), so this
        // walk and `ungated_reachable` cannot disagree about which edge is gated.
        let gates = gate_states(&text);
        for (line_no, line) in text.split_inclusive('\n').enumerate() {
            let (body, newline) = match line.strip_suffix('\n') {
                Some(b) => (b, "\n"),
                None => (line, ""),
            };
            // Whether *this* line's own edge is gated; `path_gated` carries the same fact for
            // every edge above it on the path from the root.
            let gated_here = gates[line_no];
            match include_target(line).map(|name| canonical(&dir.join(name))) {
                Some(target) if target.is_file() => {
                    // `FEMACS_DEBUG_EXPAND=1` prints each edge's decision.  It exists because the
                    // `_GL_EXTERN_INLINE` duplicate (48 Android units, P-249) is invisible in the
                    // output: the copy is included once per *including* mirror, and this is the
                    // only place that knows which branch each edge took.
                    if std::env::var("FEMACS_DEBUG_EXPAND").is_ok_and(|v| !v.is_empty()) {
                        eprintln!("expand: {} gated={gated_here} path_gated={path_gated} seen={} ungated={} <- {}",
                                  target.file_name().unwrap_or_default().to_string_lossy(),
                                  self.seen.contains(&target),
                                  self.ungated.contains(&target),
                                  dir.display());
                    }
                    if self.seen.contains(&target) {
                        out.push_str(&format!("// [xtask dedupe] {} already included{newline}",
                                              target.display()));
                        // A comment is not an item, so an attribute that was written for this
                        // `include!` would attach to whatever comes next - with the gate off,
                        // that is the file the ungated reach put after it (measured on the
                        // zz_mlgate fixture: the ungated edge's items disappeared).  The
                        // anonymous const consumes the attribute and is dropped with it.
                        out.push_str(&format!("const _: () = ();{newline}"));
                    } else if (gated_here || path_gated)
                        && target != self.root
                        && self.ungated.contains(&target)
                    {
                        // P-89's root fix (status 3.429).  A gated `include!` does *not* set C's
                        // include guard when the gate is off, so it must not consume the file's
                        // one expansion slot either: an *ungated* reach is live in every
                        // configuration, so a file that has one expands there and every
                        // gated-path reach of it (this one, or one nested inside a gated file's
                        // copy) stays a no-op.  The reverse order is what broke `sfnt.rs`'s
                        // `#ifdef TEST` `<time.h>`: it suppressed `sys_select.rs`'s own ungated
                        // one, and five `E0425`s (`time_t`/`timespec`) were the arm's reading.
                        out.push_str(&format!(
                            "// [xtask dedupe] {} expands at its ungated reach (this include is under a #[cfg]){newline}",
                            target.display()));
                        // Same reason as the branch above: the attribute written for this edge
                        // must be consumed by an item, not by a comment.
                        out.push_str(&format!("const _: () = ();{newline}"));
                    } else {
                        self.seen.push(target.clone());
                        let copy = self.copy_path(&target);
                        let body_text = self.render(&target, path_gated || gated_here)?;
                        fs::write(&copy, body_text)
                            .map_err(|e| format!("{}: {e}", copy.display()))?;
                        out.push_str(&format!("include!({:?});{newline}",
                                              copy.display().to_string()));
                    }
                }
                // An `include!` whose target does not exist relative to the file that writes it:
                // C's mirrors reach `../src/include/config.rs` from a directory where that path
                // does not resolve, and the file it names is included by the *unit* instead.  In
                // a copy the line must be commented out, because the copy lives elsewhere and
                // the relative path would point at a file in the build tree (the old
                // `neutralize` step did this for every copy); the *root*'s own lines are kept
                // verbatim, so a typo in a unit still fails the compile with rustc's message.
                _ => {
                    if path != self.root && include_target(line).is_some() {
                        out.push_str(&format!("// [xtask dedupe] {body}{newline}"));
                    } else {
                        out.push_str(&format!("{body}{newline}"));
                    }
                }
            }
        }
        Ok(out)
    }
}

/// `(gated, target)` for every `include!` line of a file's TEXT, where `gated` is true when the
/// line is preceded (blank lines skipped) by a `#[cfg…]`/`#[cfg_attr…]` attribute.  A mirror's
/// comment describing an include is not one: only a real `include!(` line counts.
fn include_edges(text: &str, dir: &Path) -> Vec<(bool, PathBuf)> {
    let gates = gate_states(text);
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if let Some(name) = include_target(line) {
            out.push((gates[i], canonical(&dir.join(name))));
        }
    }
    out
}

/// Whether an `include!` written on each line of TEXT sits under an attribute.  The rule is
/// "the attribute run that reaches this line", where a run is opened by `#[…]`/`#![…]`, may
/// span several lines (a multi-line `#[cfg (all (…, not (…)))]`), is not interrupted by a
/// blank line or a `//` comment, and is consumed by the first real item after it.
///
/// Reading only the previous line was a real defect with a real cost: a multi-line gate read as
/// no gate at all, so the gated edge claimed the file's single expansion slot away from the
/// reach that is live in every configuration, and a frozen build that must produce the empty
/// object produced code instead (measured on gmalloc's first draft: 1240 bytes holding
/// `flush_stack_barrier`/`cons_to_integer`, docs/status.md 3.553; the SetAEQ-class units write
/// their gates as two or three `#[cfg]` lines).  `render` and `ungated_reachable` both read this
/// one function now, so the two readings cannot disagree again.
fn gate_states(text: &str) -> Vec<bool> {
    let mut out = Vec::new();
    let mut attr_run = false;
    let mut attr_open = false;
    let mut depth = 0i32;
    for line in text.lines() {
        out.push(attr_run);
        let trimmed = line.trim_start();
        if attr_open {
            let (d, closed) = scan_brackets(trimmed, depth);
            depth = d;
            if closed {
                attr_open = false;
                attr_run = true;
            }
        } else if trimmed.starts_with("#[") || trimmed.starts_with("#![") {
            let (d, closed) = scan_brackets(trimmed, 0);
            depth = d;
            attr_open = !closed;
            attr_run = true;
        } else if trimmed.is_empty() || trimmed.starts_with("//") {
            // A blank line or a line comment does not end the run: the attribute still
            // belongs to the next item.
        } else {
            // A real item: whatever run reached this line applied to it and is consumed.
            attr_run = false;
        }
    }
    out
}

/// The files an *ungated* `include!` path reaches from ROOT: the ones every configuration
/// expands.  `Expand::render` needs this set to keep a `#[cfg]`-gated edge from taking a file's
/// single expansion slot away from its always-live reach (P-89's root fix, status 3.429).
fn ungated_reachable(root: &Path) -> std::collections::BTreeSet<PathBuf> {
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let dir = path.parent().unwrap_or(Path::new("."));
        for (gated, target) in include_edges(&text, dir) {
            if gated || !target.is_file() {
                continue;
            }
            if seen.insert(target.clone()) {
                stack.push(target);
            }
        }
    }
    seen
}

/// Every file the `include!("…")` chain of ROOT reaches, resolved the way rustc does
/// (relative to the file that contains the macro call) and added to SEEN.
fn collect_includes(root: &Path, seen: &mut std::collections::BTreeSet<PathBuf>) {
    let Ok(text) = fs::read_to_string(root) else { return };
    let dir = root.parent().unwrap_or(Path::new("."));
    let mut rest = text.as_str();
    while let Some(pos) = rest.find("include!(") {
        rest = &rest[pos + "include!(".len()..];
        let Some(end) = rest.find(")") else { break };
        let quoted = rest[..end].trim();
        if let Some(name) = quoted.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            let inc = canonical(&dir.join(name));
            if seen.insert(inc.clone()) && inc.is_file() {
                collect_includes(&inc, seen);
            }
        }
        rest = &rest[end..];
    }
}
