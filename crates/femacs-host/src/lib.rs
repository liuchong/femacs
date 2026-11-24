#![forbid(unsafe_code)]
//! Safe single-process host interface linking the single VM owner (`femacs-vm`) (`C1`).

use core::marker::PhantomData;
use std::ffi::OsString;
use std::path::Path;

pub use femacs_vm;
pub use femacs_vm::host_runtime::{
    HeadlessDiagnostics, HostBootstrap, HostInitConfig as HostConfig, VmHostError as HostError,
};
use femacs_vm::roots::GcContext;

/// Owning safe handle to a single headless `femacs-vm` instance (`C1`).
///
/// Enforces single-process initialization, `!Send + !Sync` thread affinity matching `GcContext`,
/// explicit GC root and condition-case boundaries across calls, and deterministic shutdown on drop.
pub struct LispHost {
    ctx: Option<GcContext>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl LispHost {
    /// Initialize the process-wide headless `femacs-vm` instance once using `config`.
    pub fn init(config: HostConfig) -> Result<Self, HostError> {
        let ctx = femacs_vm::host_runtime::init_headless_host(&config)?;
        Ok(Self {
            ctx: Some(ctx),
            _not_send_sync: PhantomData,
        })
    }

    #[inline]
    fn ctx_mut(&mut self) -> &mut GcContext {
        self.ctx
            .as_mut()
            .expect("LispHost context present until Drop")
    }

    /// Parse and sequentially evaluate all forms in `source`, returning the final `prin1` result.
    pub fn read_eval(&mut self, source: &str) -> Result<String, HostError> {
        femacs_vm::host_runtime::read_eval_to_string(self.ctx_mut(), source)
    }

    /// Execute a Lisp script file with `command-line-args-left` (`argv`) dynamically bound.
    pub fn load_script(&mut self, path: &Path, argv: &[OsString]) -> Result<(), HostError> {
        femacs_vm::host_runtime::load_script_file(self.ctx_mut(), path, argv)
    }

    /// Run a VM garbage collection cycle inside anchored stack bounds.
    pub fn garbage_collect(&mut self) -> Result<(), HostError> {
        femacs_vm::host_runtime::garbage_collect(self.ctx_mut())
    }

    /// Return headless frame/terminal/pdumper/rooting diagnostics for the active VM.
    pub fn inspect_environment(&mut self) -> Result<HeadlessDiagnostics, HostError> {
        femacs_vm::host_runtime::inspect_headless_environment(self.ctx_mut())
    }

    /// Compose `femacs_vm::safe_domain` or `vm_primitives` directly with anchored stack bounds.
    pub fn with_context<R>(&mut self, f: impl FnOnce(&mut GcContext) -> R) -> R {
        femacs_vm::host_runtime::with_anchored_context(self.ctx_mut(), f)
    }
}

impl Drop for LispHost {
    fn drop(&mut self) {
        if let Some(ctx) = self.ctx.take() {
            femacs_vm::host_runtime::shutdown_headless_host(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    trait NotSendOrSyncFallback {
        const VALUE: bool = false;
    }

    struct SendProbe<T>(PhantomData<T>);
    impl<T: Send> SendProbe<T> {
        const VALUE: bool = true;
    }
    impl<T> NotSendOrSyncFallback for SendProbe<T> {}

    struct SyncProbe<T>(PhantomData<T>);
    impl<T: Sync> SyncProbe<T> {
        const VALUE: bool = true;
    }
    impl<T> NotSendOrSyncFallback for SyncProbe<T> {}

    const _: () = assert!(SendProbe::<String>::VALUE);
    const _: () = assert!(SyncProbe::<String>::VALUE);
    const _: () = assert!(!SendProbe::<LispHost>::VALUE);
    const _: () = assert!(!SyncProbe::<LispHost>::VALUE);
    const _: () = assert!(!SendProbe::<GcContext>::VALUE);
    const _: () = assert!(!SyncProbe::<GcContext>::VALUE);

    #[test]
    fn host_config_builders_set_expected_fields() {
        let minimal = HostConfig::cold_minimal();
        assert_eq!(minimal.program_name, "femacs-host");
        assert_eq!(minimal.bootstrap, HostBootstrap::ColdMinimal);
        assert!(minimal.load_paths.is_empty());
        assert!(minimal.quiet_bootstrap);

        let loadup = HostConfig::cold_loadup("lisp").with_load_path("site-lisp");
        assert_eq!(loadup.bootstrap, HostBootstrap::ColdLoadup);
        assert_eq!(
            loadup.load_paths,
            vec![PathBuf::from("lisp"), PathBuf::from("site-lisp")]
        );

        let dump = HostConfig::with_dump("emacs.pdmp").with_load_path("lisp");
        assert_eq!(
            dump.bootstrap,
            HostBootstrap::DumpFile(PathBuf::from("emacs.pdmp"))
        );
        assert_eq!(dump.load_paths, vec![PathBuf::from("lisp")]);
    }

    #[test]
    fn host_error_display_covers_all_variants() {
        assert_eq!(
            HostError::AlreadyInitialized.to_string(),
            "femacs VM host already initialized"
        );
        assert_eq!(
            HostError::InitFailed("bad pdmp".to_string()).to_string(),
            "femacs VM init failed: bad pdmp"
        );
        assert_eq!(
            HostError::Read {
                symbol: "end-of-file".to_string(),
                message: "unexpected eof".to_string(),
                data: "nil".to_string(),
            }
            .to_string(),
            "Lisp read error (end-of-file): unexpected eof [nil]"
        );
        assert_eq!(
            HostError::LispCondition {
                symbol: "void-variable".to_string(),
                message: "unbound".to_string(),
                data: "(foo)".to_string(),
            }
            .to_string(),
            "Lisp condition (void-variable): unbound [(foo)]"
        );
        assert_eq!(
            HostError::Quit {
                message: "Quit".to_string(),
            }
            .to_string(),
            "Lisp quit: Quit"
        );
        assert_eq!(
            HostError::Io {
                symbol: "file-missing".to_string(),
                message: "No such file".to_string(),
                data: "(\"x.el\")".to_string(),
            }
            .to_string(),
            "Lisp I/O error (file-missing): No such file [(\"x.el\")]"
        );
    }

    #[test]
    fn lisp_host_and_gc_context_remain_not_send_not_sync() {
        assert!(!SendProbe::<LispHost>::VALUE);
        assert!(!SyncProbe::<LispHost>::VALUE);
        assert!(!SendProbe::<GcContext>::VALUE);
        assert!(!SyncProbe::<GcContext>::VALUE);
    }
}
