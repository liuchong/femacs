//! Minimal OS/process FFI boundary for `lisp` (termination signals, process-group /
//! Job-Object supervision, process enumeration, and memory accounting).
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(unix)]
pub const SIGINT: i32 = 2;
pub const SIGKILL: i32 = 9;
pub const SIGTERM: i32 = 15;

static TERMINATION_REQUESTED: AtomicBool = AtomicBool::new(false);

#[cfg(unix)]
extern "C" fn on_termination_signal(_sig: i32) {
    TERMINATION_REQUESTED.store(true, Ordering::Release);
}

#[cfg(windows)]
unsafe extern "system" fn on_console_ctrl(_ctrl_type: u32) -> i32 {
    TERMINATION_REQUESTED.store(true, Ordering::Release);
    1
}

/// Register async-signal-safe termination handlers for clean supervisor teardown.
#[cfg(unix)]
pub fn install_termination_handlers() {
    unsafe extern "C" {
        fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    }
    // SAFETY: `on_termination_signal` performs only an async-signal-safe release atomic store.
    unsafe {
        let _ = signal(SIGINT, on_termination_signal);
        let _ = signal(SIGTERM, on_termination_signal);
    }
}

/// Register a Windows console control handler setting `TERMINATION_REQUESTED`.
#[cfg(windows)]
pub fn install_termination_handlers() {
    use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;

    // SAFETY: `on_console_ctrl` performs only a single atomic release store and returns TRUE (`1`).
    unsafe {
        let _ = SetConsoleCtrlHandler(Some(on_console_ctrl), 1);
    }
}

pub fn termination_requested() -> bool {
    TERMINATION_REQUESTED.load(Ordering::Acquire)
}

/// Return the current process group ID on Unix (or current PID on Windows).
#[cfg(unix)]
pub fn current_pgid() -> u32 {
    unsafe extern "C" {
        fn getpgrp() -> i32;
    }
    // SAFETY: `getpgrp()` takes no arguments and queries the calling process's group ID.
    let pg = unsafe { getpgrp() };
    u32::try_from(pg).unwrap_or(0)
}

#[cfg(windows)]
pub fn current_pgid() -> u32 {
    std::process::id()
}

/// Send `sig` to a single process `pid` (`pid > 1` and never `self`).
#[cfg(unix)]
pub fn signal_process(pid: i32, sig: i32) -> Result<(), i32> {
    let self_pid = i32::try_from(std::process::id()).unwrap_or(0);
    if pid <= 1 || pid == self_pid {
        return Err(22);
    }
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    // SAFETY: `pid > 1` and `pid != self_pid`, so `kill(pid, sig)` targets only that descendant PID.
    let rc = unsafe { kill(pid, sig) };
    if rc == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(1))
    }
}

#[cfg(windows)]
pub fn signal_process(pid: i32, _sig: i32) -> Result<(), i32> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_TERMINATE, TerminateProcess,
    };

    let self_pid = i32::try_from(std::process::id()).unwrap_or(0);
    if pid <= 1 || pid == self_pid {
        return Err(22);
    }
    // SAFETY: `OpenProcess` is called for a validated descendant PID and the handle is closed immediately.
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
        if handle.is_null() {
            return Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(1));
        }
        let ok = TerminateProcess(handle, 1);
        let err = if ok == 0 {
            Some(std::io::Error::last_os_error().raw_os_error().unwrap_or(1))
        } else {
            None
        };
        let _ = CloseHandle(handle);
        match err {
            Some(code) => Err(code),
            None => Ok(()),
        }
    }
}

/// Send `sig` to every process in Unix process group `pgid` (`pgid > 1`).
#[cfg(unix)]
pub fn signal_process_group(pgid: i32, sig: i32) -> Result<(), i32> {
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

#[cfg(windows)]
pub fn signal_process_group(pgid: i32, sig: i32) -> Result<(), i32> {
    if pgid <= 1 {
        return Err(22);
    }
    signal_process(pgid, sig)
}

/// Minimal Windows Toolhelp process row (`pid`, `ppid`).
#[cfg(windows)]
#[derive(Clone, Copy, Debug)]
pub struct WinProcessRow {
    pub pid: u32,
    pub ppid: u32,
}

/// Enumerate active Windows processes via `CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS)`.
#[cfg(windows)]
pub fn snapshot_windows_processes() -> Result<Vec<WinProcessRow>, String> {
    use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
        TH32CS_SNAPPROCESS,
    };

    // SAFETY: `CreateToolhelp32Snapshot` returns a valid snapshot handle or `INVALID_HANDLE_VALUE`;
    // `PROCESSENTRY32W` has `dwSize` initialized before calling `Process32FirstW`/`Process32NextW`.
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return Err(format!(
                "CreateToolhelp32Snapshot failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = core::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut rows = Vec::new();
        if Process32FirstW(snap, &raw mut entry) != 0 {
            loop {
                rows.push(WinProcessRow {
                    pid: entry.th32ProcessID,
                    ppid: entry.th32ParentProcessID,
                });
                if Process32NextW(snap, &raw mut entry) == 0 {
                    break;
                }
            }
        }
        let _ = CloseHandle(snap);
        Ok(rows)
    }
}

/// RAII Win32 Job Object configured with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`.
#[cfg(windows)]
pub struct ProcessJob {
    handle: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl ProcessJob {
    /// Create a Job Object configured to kill all members on close and assign `pid` to it.
    pub fn create_for_pid(pid: u32) -> Option<Self> {
        use core::ptr::{null, null_mut};
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        };
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
        };

        // SAFETY: Allocates an anonymous Win32 Job Object handle, configures valid
        // `JOBOBJECT_EXTENDED_LIMIT_INFORMATION` storage, assigns `pid`, and closes temporary handles.
        unsafe {
            let job = CreateJobObjectW(null(), null());
            if job.is_null() {
                return None;
            }
            let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let info_len = core::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32;
            if SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                (&raw const info).cast(),
                info_len,
            ) == 0
            {
                let _ = CloseHandle(job);
                return None;
            }
            let proc_handle = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, 0, pid);
            if proc_handle.is_null() {
                let _ = CloseHandle(job);
                return None;
            }
            let assigned = AssignProcessToJobObject(job, proc_handle);
            let _ = CloseHandle(proc_handle);
            if assigned == 0 {
                let _ = CloseHandle(job);
                return None;
            }
            let _ = null_mut::<core::ffi::c_void>();
            Some(Self { handle: job })
        }
    }

    /// Terminate every process currently assigned to this Job Object.
    pub fn terminate(&self, exit_code: u32) {
        use windows_sys::Win32::System::JobObjects::TerminateJobObject;

        // SAFETY: `self.handle` is an open Win32 Job Object handle owned until `Drop`.
        unsafe {
            let _ = TerminateJobObject(self.handle, exit_code);
        }
    }
}

#[cfg(windows)]
impl Drop for ProcessJob {
    fn drop(&mut self) {
        use windows_sys::Win32::Foundation::CloseHandle;

        // SAFETY: `self.handle` is uniquely owned by this `ProcessJob` and closed once on drop.
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Query `(resident_bytes, phys_footprint_bytes, proc_start_abstime)` for `pid` on macOS.
#[cfg(target_os = "macos")]
pub fn pid_rusage_info(pid: i32) -> Option<(u64, u64, u64)> {
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

#[cfg(target_os = "linux")]
pub fn pid_rusage_info(pid: i32) -> Option<(u64, u64, u64)> {
    if pid <= 0 {
        return None;
    }
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let mut vm_rss_bytes: Option<u64> = None;
    let mut rss_anon_bytes: Option<u64> = None;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kb = rest.split_whitespace().next()?.parse::<u64>().ok()?;
            vm_rss_bytes = Some(kb.saturating_mul(1024));
        } else if let Some(rest) = line.strip_prefix("RssAnon:") {
            let kb = rest.split_whitespace().next()?.parse::<u64>().ok()?;
            rss_anon_bytes = Some(kb.saturating_mul(1024));
        }
    }
    let rss = vm_rss_bytes?;
    let footprint = rss_anon_bytes.unwrap_or(rss);
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let rparen = stat.rfind(')')?;
    let after = stat.get(rparen + 1..)?;
    // Fields after `(comm)` start at field 3 (state = index 0); field 22 (starttime) is index 19.
    let start_ticks = after.split_whitespace().nth(19)?.parse::<u64>().ok()?;
    Some((rss, footprint, start_ticks))
}

#[cfg(windows)]
pub fn pid_rusage_info(pid: i32) -> Option<(u64, u64, u64)> {
    use windows_sys::Win32::Foundation::{CloseHandle, FILETIME};
    use windows_sys::Win32::System::ProcessStatus::{
        K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX,
    };
    use windows_sys::Win32::System::Threading::{
        GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    if pid <= 0 {
        return None;
    }
    // SAFETY: Queries `K32GetProcessMemoryInfo` and `GetProcessTimes` into initialized stack
    // structs for a validated handle, then always closes the handle.
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid as u32);
        if handle.is_null() {
            return None;
        }
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        counters.cb = core::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let mem_ok = K32GetProcessMemoryInfo(
            handle,
            (&raw mut counters).cast::<PROCESS_MEMORY_COUNTERS>(),
            counters.cb,
        );
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();
        let times_ok = GetProcessTimes(
            handle,
            &raw mut creation,
            &raw mut exit,
            &raw mut kernel,
            &raw mut user,
        );
        let _ = CloseHandle(handle);
        if mem_ok == 0 {
            return None;
        }
        let rss = counters.WorkingSetSize as u64;
        let footprint = (counters.PrivateUsage as u64).max(rss);
        let start_ticks = if times_ok != 0 {
            (u64::from(creation.dwHighDateTime) << 32) | u64::from(creation.dwLowDateTime)
        } else {
            0
        };
        Some((rss, footprint, start_ticks))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
pub fn pid_rusage_info(_pid: i32) -> Option<(u64, u64, u64)> {
    None
}

/// Query `(resident_bytes, phys_footprint_bytes)` for `pid`.
pub fn pid_rusage_memory(pid: i32) -> Option<(u64, u64)> {
    pid_rusage_info(pid).map(|(rss, fp, _)| (rss, fp))
}
