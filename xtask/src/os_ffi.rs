//! Minimal OS/process FFI boundary for `xtask` (atomic macOS directory swap and
//! serialized test env setup).
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::OsStr;
use std::path::Path;

#[cfg(target_os = "macos")]
pub(crate) fn exchange_dirs(left: &Path, right: &Path) -> Result<(), String> {
    use std::{ffi::CString, os::unix::ffi::OsStrExt};
    unsafe extern "C" {
        fn renamex_np(
            from: *const std::ffi::c_char,
            to: *const std::ffi::c_char,
            flags: u32,
        ) -> i32;
    }
    let from = CString::new(left.as_os_str().as_bytes()).map_err(|_| "NUL in staging path")?;
    let to = CString::new(right.as_os_str().as_bytes()).map_err(|_| "NUL in destination path")?;
    // SAFETY: `from` and `to` are NUL-terminated `CString` paths passed to macOS `renamex_np(RENAME_SWAP)`.
    let rc = unsafe { renamex_np(from.as_ptr(), to.as_ptr(), 0x0000_0002) };
    if rc != 0 {
        return Err(format!(
            "atomic app exchange failed; old app retained: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn exchange_dirs(_left: &Path, _right: &Path) -> Result<(), String> {
    Err("application exchange requires macOS".into())
}

/// Mutate a process environment variable while serialized on the owning CLI thread or under `env_lock()`.
pub(crate) fn set_process_env(key: &str, value: impl AsRef<OsStr>) {
    // SAFETY: Called only by single-threaded `xtask` CLI setup or while holding the test `env_lock()` mutex.
    unsafe {
        std::env::set_var(key, value);
    }
}

#[cfg(test)]
pub(crate) fn remove_process_env(key: &str) {
    // SAFETY: Called only while holding the serialized test `env_lock()` mutex.
    unsafe {
        std::env::remove_var(key);
    }
}
