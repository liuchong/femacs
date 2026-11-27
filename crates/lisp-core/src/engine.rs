#![forbid(unsafe_code)]
//! High-level general-purpose Lisp embedding engine backed by `lisp-vm`.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use lisp_vm::host_runtime::{HeadlessDiagnostics, HeadlessVm, HostInitConfig, VmValue};

use crate::error::Error;
use crate::stdlib_cache::{
    CacheLayout, ensure_unpacked, is_valid_dump_header, resolve_cache_root,
};
use crate::value::Value;

/// Startup mode controlling standard library and portable dump loading in [`Engine`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BootstrapMode {
    /// Automatically extract bundled stdlib into the cache and reuse cached `.pdmp` if valid.
    AutoCache,
    /// Automatically extract bundled stdlib and cold-boot `core-boot.el` without reading `.pdmp`.
    ColdStdlib,
    /// Initialize only core VM primitives without unpacking stdlib or needing external data files.
    Minimal,
    /// Boot from an explicit Lisp directory and/or portable dump file.
    Custom {
        lisp_dir: Option<PathBuf>,
        boot_script: Option<PathBuf>,
        data_dir: Option<PathBuf>,
        dump_file: Option<PathBuf>,
    },
}

/// Configuration for initializing an [`Engine`] instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineConfig {
    pub program_name: String,
    pub bootstrap: BootstrapMode,
    pub cache_dir: Option<PathBuf>,
    pub load_paths: Vec<PathBuf>,
    pub quiet_bootstrap: bool,
    pub replace_load_path: bool,
    pub defer_delayed_custom_init: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            program_name: "lisp-core".to_string(),
            bootstrap: BootstrapMode::AutoCache,
            cache_dir: None,
            load_paths: Vec::new(),
            quiet_bootstrap: true,
            replace_load_path: false,
            defer_delayed_custom_init: false,
        }
    }
}

impl EngineConfig {
    /// Create a minimal primitive-only configuration without bundled stdlib extraction.
    pub fn minimal() -> Self {
        Self {
            bootstrap: BootstrapMode::Minimal,
            ..Self::default()
        }
    }

    /// Create a cold stdlib configuration skipping cached portable dumps.
    pub fn cold_stdlib() -> Self {
        Self {
            bootstrap: BootstrapMode::ColdStdlib,
            ..Self::default()
        }
    }

    pub fn with_program_name(mut self, program_name: impl Into<String>) -> Self {
        self.program_name = program_name.into();
        self
    }

    pub fn with_cache_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = Some(dir.into());
        self
    }

    pub fn with_load_path(mut self, dir: impl Into<PathBuf>) -> Self {
        self.load_paths.push(dir.into());
        self
    }

    pub fn with_bootstrap(mut self, bootstrap: BootstrapMode) -> Self {
        self.bootstrap = bootstrap;
        self
    }

    pub fn with_quiet_bootstrap(mut self, quiet: bool) -> Self {
        self.quiet_bootstrap = quiet;
        self
    }

    pub fn with_replace_load_path(mut self, replace: bool) -> Self {
        self.replace_load_path = replace;
        self
    }

    pub fn with_defer_delayed_custom_init(mut self, defer: bool) -> Self {
        self.defer_delayed_custom_init = defer;
        self
    }
}

/// Safe, single-process owning handle to an initialized general-purpose Lisp VM (`!Send + !Sync`).
pub struct Engine {
    vm: HeadlessVm,
    cache_layout: Option<CacheLayout>,
}

impl Engine {
    /// Initialize the standalone general-purpose Lisp runtime with bundled stdlib & auto cache.
    pub fn new() -> Result<Self, Error> {
        Self::init(EngineConfig::default())
    }

    /// Initialize only core VM primitives without unpacking stdlib or needing `data/` files.
    pub fn minimal() -> Result<Self, Error> {
        Self::init(EngineConfig::minimal())
    }

    /// Initialize the runtime with an explicit [`EngineConfig`].
    pub fn init(config: EngineConfig) -> Result<Self, Error> {
        let (host_cfg, cache_layout, extra_load_paths) = match &config.bootstrap {
            BootstrapMode::Minimal => {
                let mut hc = HostInitConfig::cold_minimal()
                    .with_program_name(config.program_name.clone())
                    .with_replace_load_path(config.replace_load_path)
                    .with_defer_delayed_custom_init(config.defer_delayed_custom_init);
                hc.quiet_bootstrap = config.quiet_bootstrap;
                for dir in &config.load_paths {
                    hc = hc.with_load_path(dir);
                }
                (hc, None, Vec::new())
            }
            BootstrapMode::ColdStdlib => {
                let cache_root = resolve_cache_root(config.cache_dir.as_deref());
                let layout = ensure_unpacked(&cache_root)?;
                let mut hc =
                    HostInitConfig::cold_boot_script(&layout.stdlib_dir, &layout.boot_script)
                        .with_program_name(config.program_name.clone())
                        .with_data_dir(&layout.data_dir)
                        .with_replace_load_path(config.replace_load_path)
                        .with_defer_delayed_custom_init(config.defer_delayed_custom_init);
                hc.quiet_bootstrap = config.quiet_bootstrap;
                hc = hc
                    .with_load_path(layout.stdlib_dir.join("emacs-lisp"))
                    .with_load_path(layout.stdlib_dir.join("international"));
                (hc, Some(layout), config.load_paths.clone())
            }
            BootstrapMode::AutoCache => {
                let cache_root = resolve_cache_root(config.cache_dir.as_deref());
                let layout = ensure_unpacked(&cache_root)?;
                let use_dump = layout.pdmp_path.is_file() && is_valid_dump_header(&layout.pdmp_path);
                let mut hc = if use_dump {
                    HostInitConfig::dump_or_boot_script(
                        &layout.stdlib_dir,
                        &layout.pdmp_path,
                        &layout.boot_script,
                    )
                    .with_replace_load_path(true)
                } else {
                    HostInitConfig::cold_boot_script(&layout.stdlib_dir, &layout.boot_script)
                        .with_replace_load_path(config.replace_load_path)
                };
                hc = hc
                    .with_program_name(config.program_name.clone())
                    .with_data_dir(&layout.data_dir)
                    .with_defer_delayed_custom_init(config.defer_delayed_custom_init)
                    .with_load_path(layout.stdlib_dir.join("emacs-lisp"))
                    .with_load_path(layout.stdlib_dir.join("international"));
                hc.quiet_bootstrap = config.quiet_bootstrap;
                (hc, Some(layout), config.load_paths.clone())
            }
            BootstrapMode::Custom {
                lisp_dir,
                boot_script,
                data_dir,
                dump_file,
            } => {
                let mut hc = match (dump_file, boot_script, lisp_dir) {
                    (Some(dump), Some(boot), Some(dir)) => {
                        HostInitConfig::dump_or_boot_script(dir, dump, boot)
                    }
                    (Some(dump), _, _) => HostInitConfig::with_dump(dump),
                    (None, Some(boot), Some(dir)) => HostInitConfig::cold_boot_script(dir, boot),
                    (None, None, Some(dir)) => {
                        let candidate = dir.join("core-boot.el");
                        let mut cfg = if candidate.is_file() {
                            HostInitConfig::cold_boot_script(&dir, candidate)
                        } else {
                            HostInitConfig::cold_loadup(&dir)
                        };
                        for sub in ["emacs-lisp", "international"] {
                            let sub_dir = dir.join(sub);
                            if sub_dir.is_dir() {
                                cfg = cfg.with_load_path(sub_dir);
                            }
                        }
                        cfg
                    }
                    (None, Some(boot), None) => HostInitConfig::cold_boot_script(
                        boot.parent().unwrap_or_else(|| Path::new(".")),
                        boot,
                    ),
                    (None, None, None) => HostInitConfig::cold_minimal(),
                };
                hc = hc
                    .with_program_name(config.program_name.clone())
                    .with_replace_load_path(config.replace_load_path)
                    .with_defer_delayed_custom_init(config.defer_delayed_custom_init);
                hc.quiet_bootstrap = config.quiet_bootstrap;
                if let Some(d) = data_dir {
                    hc = hc.with_data_dir(d);
                }
                if let Some(l) = lisp_dir {
                    if !matches!(config.bootstrap, BootstrapMode::Custom { dump_file: Some(_), .. }) {
                        // `cold_*` constructors already include `lisp_dir`, whereas `with_dump` does not.
                    } else {
                        hc = hc.with_load_path(l);
                    }
                }
                for dir in &config.load_paths {
                    hc = hc.with_load_path(dir);
                }
                (hc, None, Vec::new())
            }
        };

        let mut vm = HeadlessVm::init(&host_cfg)?;
        for dir in extra_load_paths.iter().rev() {
            vm.prepend_load_path(dir)?;
        }
        Ok(Self { vm, cache_layout })
    }

    /// Return the materialized stdlib cache layout if initialized with bundled stdlib.
    pub fn cache_layout(&self) -> Option<&CacheLayout> {
        self.cache_layout.as_ref()
    }

    /// Evaluate one or more Lisp expressions and convert the final result into [`Value`].
    pub fn eval(&mut self, source: &str) -> Result<Value, Error> {
        Ok(Value::from(self.vm.read_eval_value(source)?))
    }

    /// Evaluate Lisp expressions and format the final result with `prin1-to-string`.
    pub fn eval_printed(&mut self, source: &str) -> Result<String, Error> {
        Ok(self.vm.read_eval_to_string(source)?)
    }

    /// Evaluate Lisp expressions silently without capturing a printed representation.
    pub fn eval_silent(&mut self, source: &str) -> Result<(), Error> {
        Ok(self.vm.eval_silent(source)?)
    }

    /// Evaluate Lisp expressions with `command-line-args-left` bound, returning optional printed
    /// result and unconsumed trailing arguments.
    pub fn eval_with_args(
        &mut self,
        source: &str,
        print_result: bool,
        args_left: &[OsString],
    ) -> Result<(Option<String>, Vec<OsString>), Error> {
        Ok(self.vm.eval_with_args(source, print_result, args_left)?)
    }

    /// Call a named Lisp function with safe typed arguments and convert its return value.
    pub fn call(&mut self, func_name: &str, args: &[Value]) -> Result<Value, Error> {
        let vm_args: Vec<VmValue> = args.iter().map(VmValue::from).collect();
        Ok(Value::from(self.vm.call_value(func_name, &vm_args)?))
    }

    /// Prepend `dir` to runtime `load-path`.
    pub fn prepend_load_path(&mut self, dir: &Path) -> Result<(), Error> {
        Ok(self.vm.prepend_load_path(dir)?)
    }

    /// Load a Lisp library or source file via `load`.
    pub fn load_library(&mut self, file: &str) -> Result<(), Error> {
        Ok(self.vm.load_library(file)?)
    }

    /// Load a Lisp library while temporarily exposing `args_left` as `command-line-args-left`.
    pub fn load_library_with_args(
        &mut self,
        file: &str,
        args_left: &[OsString],
    ) -> Result<Vec<OsString>, Error> {
        Ok(self.vm.load_library_with_args(file, args_left)?)
    }

    /// Call a zero-argument Lisp function by name while passing `args_left` in `command-line-args-left`.
    pub fn funcall_with_args_left(
        &mut self,
        func_name: &str,
        args_left: &[OsString],
    ) -> Result<Vec<OsString>, Error> {
        Ok(self.vm.funcall_with_args_left(func_name, args_left)?)
    }

    /// Call a two-string-argument Lisp function returning a string.
    pub fn funcall_two_strings_to_string(
        &mut self,
        func_name: &str,
        arg1: &str,
        arg2: &str,
    ) -> Result<String, Error> {
        Ok(self.vm.funcall_two_strings_to_string(func_name, arg1, arg2)?)
    }

    /// Execute a Lisp script file with shebang stripping and script `argv`/`command-line-args-left`.
    pub fn run_script(&mut self, path: &Path, args: &[OsString]) -> Result<(), Error> {
        Ok(self.vm.load_script_file(path, args)?)
    }

    /// Byte-compile source files, failing on any compile warning when `strict` is `true`.
    pub fn compile_files(&mut self, strict: bool, files: &[PathBuf]) -> Result<(), Error> {
        if files.is_empty() {
            return Err(Error::InitFailed(
                "compile requires at least one source file".to_string(),
            ));
        }
        let mut args = Vec::with_capacity(files.len() + 1);
        args.push(Value::Bool(strict));
        for path in files {
            args.push(Value::String(path.to_string_lossy().into_owned()));
        }
        self.call("lisp-core--compile-files", &args)?;
        Ok(())
    }

    /// Load ERT test files and run registered tests in batch mode.
    pub fn run_ert_files(&mut self, files: &[PathBuf]) -> Result<(), Error> {
        let args: Vec<Value> = files
            .iter()
            .map(|p| Value::String(p.to_string_lossy().into_owned()))
            .collect();
        self.call("lisp-core--run-ert-files", &args)?;
        Ok(())
    }

    /// Write a portable dump snapshot to `out_path`.
    pub fn dump_image(&mut self, out_path: &Path) -> Result<(), Error> {
        Ok(self.vm.dump_portable(out_path)?)
    }

    /// Run garbage collection in the underlying VM.
    pub fn gc(&mut self) -> Result<(), Error> {
        Ok(self.vm.garbage_collect()?)
    }

    /// Inspect runtime diagnostics (noninteractive flags, frame/terminal counts, rooting stats).
    pub fn inspect_environment(&mut self) -> Result<HeadlessDiagnostics, Error> {
        Ok(self.vm.inspect_environment()?)
    }
}
