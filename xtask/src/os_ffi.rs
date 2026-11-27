//! Minimal OS/process FFI boundary for `xtask` (atomic macOS directory swap, serialized env
//! setup, process-group signaling, and macOS process memory inspection).
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::OsStr;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const SIGINT: i32 = 2;
pub(crate) const SIGKILL: i32 = 9;
pub(crate) const SIGTERM: i32 = 15;

static TERMINATION_REQUESTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_termination_signal(_sig: i32) {
    TERMINATION_REQUESTED.store(true, Ordering::Release);
}

/// Register async-signal-safe `SIGINT`/`SIGTERM` handlers for clean supervisor teardown.
pub(crate) fn install_termination_handlers() {
    unsafe extern "C" {
        fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    }
    // SAFETY: `on_termination_signal` performs only an async-signal-safe relaxed/release atomic store.
    unsafe {
        let _ = signal(SIGINT, on_termination_signal);
        let _ = signal(SIGTERM, on_termination_signal);
    }
}

pub(crate) fn termination_requested() -> bool {
    TERMINATION_REQUESTED.load(Ordering::Acquire)
}

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

/// Return the current process group ID.
pub(crate) fn current_pgid() -> u32 {
    unsafe extern "C" {
        fn getpgrp() -> i32;
    }
    // SAFETY: `getpgrp()` takes no arguments and queries the calling process's group ID.
    let pg = unsafe { getpgrp() };
    u32::try_from(pg).unwrap_or(0)
}

/// Send `sig` to a single process `pid` (`pid > 1` and never `self`).
pub(crate) fn signal_process(pid: i32, sig: i32) -> Result<(), i32> {
    let self_pid = i32::try_from(std::process::id()).unwrap_or(0);
    if pid <= 1 || pid == self_pid {
        return Err(22);
    }
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    // SAFETY: `pid > 1` and `pid != self_pid`, so `kill(pid, sig)` targets only that tracked descendant PID.
    let rc = unsafe { kill(pid, sig) };
    if rc == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(1))
    }
}

/// Send `sig` to every process in Unix process group `pgid` (`pgid > 1`).
pub(crate) fn signal_process_group(pgid: i32, sig: i32) -> Result<(), i32> {
    if pgid <= 1 {
        return Err(22);
    }
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    // SAFETY: `pgid > 1`, so `-pgid < -1` targets only the child's isolated process group.
    let rc = unsafe { kill(-pgid, sig) };
    if rc == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(1))
    }
}

/// Query `(resident_bytes, phys_footprint_bytes, proc_start_abstime)` for `pid` on macOS.
#[cfg(target_os = "macos")]
pub(crate) fn pid_rusage_info(pid: i32) -> Option<(u64, u64, u64)> {
    if pid <= 0 {
        return None;
    }
    const RUSAGE_INFO_V2: i32 = 2;
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct RusageInfoV2 {
        ri_uuid: [u8; 16],
        ri_user_time: u64,
        ri_system_time: u64,
        ri_pkg_idle_wkups: u64,
        ri_interrupt_wkups: u64,
        ri_pageins: u64,
        ri_wired_size: u64,
        ri_resident_size: u64,
        ri_phys_footprint: u64,
        ri_proc_start_abstime: u64,
        ri_proc_exit_abstime: u64,
        ri_child_user_time: u64,
        ri_child_system_time: u64,
        ri_child_pkg_idle_wkups: u64,
        ri_child_interrupt_wkups: u64,
        ri_child_pageins: u64,
        ri_child_elapsed_abstime: u64,
        ri_diskio_bytesread: u64,
        ri_diskio_byteswritten: u64,
    }
    unsafe extern "C" {
        fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut core::ffi::c_void) -> i32;
    }
    let mut info = RusageInfoV2 {
        ri_uuid: [0; 16],
        ri_user_time: 0,
        ri_system_time: 0,
        ri_pkg_idle_wkups: 0,
        ri_interrupt_wkups: 0,
        ri_pageins: 0,
        ri_wired_size: 0,
        ri_resident_size: 0,
        ri_phys_footprint: 0,
        ri_proc_start_abstime: 0,
        ri_proc_exit_abstime: 0,
        ri_child_user_time: 0,
        ri_child_system_time: 0,
        ri_child_pkg_idle_wkups: 0,
        ri_child_interrupt_wkups: 0,
        ri_child_pageins: 0,
        ri_child_elapsed_abstime: 0,
        ri_diskio_bytesread: 0,
        ri_diskio_byteswritten: 0,
    };
    let buf_ptr: *mut core::ffi::c_void = (&raw mut info).cast();
    // SAFETY: `buf_ptr` points to valid writable `RusageInfoV2` storage matching `RUSAGE_INFO_V2`.
    let rc = unsafe { proc_pid_rusage(pid, RUSAGE_INFO_V2, buf_ptr) };
    if rc == 0 {
        Some((
            info.ri_resident_size,
            info.ri_phys_footprint,
            info.ri_proc_start_abstime,
        ))
    } else {
        None
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn pid_rusage_info(_pid: i32) -> Option<(u64, u64, u64)> {
    None
}

/// Query `(resident_bytes, phys_footprint_bytes)` for `pid` on macOS via `proc_pid_rusage`.
pub(crate) fn pid_rusage_memory(pid: i32) -> Option<(u64, u64)> {
    pid_rusage_info(pid).map(|(rss, fp, _)| (rss, fp))
}
