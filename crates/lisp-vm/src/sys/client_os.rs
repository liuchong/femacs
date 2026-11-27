//! Verified low-level OS, Unix-domain socket, and TTY signal boundary for `emacsclient`.
//!
//! Isolates raw POSIX descriptors, `sockaddr_un`, directory-FD symlink/permission/ACL checks,
//! and async-signal-safe flags behind safe Rust interfaces so `lib-src/emacsclient.rs` runs
//! under `#![forbid(unsafe_code)]`.
#![deny(unsafe_op_in_unsafe_fn)]

use crate::sys_common::*;
use core::sync::atomic::{AtomicBool, Ordering};
use std::ffi::{CString, OsStr};
use std::io;
use std::net::TcpStream;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

const AF_UNIX_C: c_int = 1;
const SOCK_STREAM_C: c_int = 1;
#[cfg(target_os = "macos")]
const SOL_SOCKET_C: c_int = 0xffff;
#[cfg(not(target_os = "macos"))]
const SOL_SOCKET_C: c_int = 1;
#[cfg(target_os = "macos")]
const SO_LINGER_C: c_int = 0x0080;
#[cfg(not(target_os = "macos"))]
const SO_LINGER_C: c_int = 13;
#[cfg(target_os = "macos")]
const SO_ERROR_C: c_int = 0x1007;
#[cfg(not(target_os = "macos"))]
const SO_ERROR_C: c_int = 4;
const CS_DARWIN_USER_TEMP_DIR: c_int = 65537;
#[cfg(target_os = "macos")]
const O_PATH_COMPAT: c_int = O_SEARCH;
#[cfg(target_os = "linux")]
const O_PATH_COMPAT: c_int = 0x200000;
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
const O_PATH_COMPAT: c_int = O_RDONLY;
#[cfg(target_os = "macos")]
pub const SUN_PATH_CAPACITY: usize = 104;
#[cfg(not(target_os = "macos"))]
pub const SUN_PATH_CAPACITY: usize = 108;

type socklen_t = u32;
type SignalHandler = Option<unsafe extern "C" fn(c_int)>;

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy)]
struct Sockaddr {
    sa_len: u8,
    sa_family: u8,
    sa_data: [c_char; 14],
}

#[cfg(not(target_os = "macos"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct Sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Clone, Copy)]
struct SockaddrUn {
    sun_len: u8,
    sun_family: u8,
    sun_path: [c_char; SUN_PATH_CAPACITY],
}

#[cfg(not(target_os = "macos"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct SockaddrUn {
    sun_family: u16,
    sun_path: [c_char; SUN_PATH_CAPACITY],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Linger {
    l_onoff: c_int,
    l_linger: c_int,
}

unsafe extern "C" {
    fn ttyname(fd: c_int) -> *mut c_char;
    fn tcgetpgrp(fd: c_int) -> pid_t;
    fn getpgrp() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn confstr(name: c_int, buf: *mut c_char, len: usize) -> usize;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const Sockaddr, addrlen: socklen_t) -> c_int;
    fn setsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
}

static GOT_SIGCONT: AtomicBool = AtomicBool::new(false);
static GOT_SIGTSTP: AtomicBool = AtomicBool::new(false);
static GOT_SIGTTOU: AtomicBool = AtomicBool::new(false);
static GOT_SIGWINCH: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn handle_sigcont(_sig: c_int) {
    GOT_SIGCONT.store(true, Ordering::Relaxed);
}
unsafe extern "C" fn handle_sigtstp(_sig: c_int) {
    GOT_SIGTSTP.store(true, Ordering::Relaxed);
}
unsafe extern "C" fn handle_sigttou(_sig: c_int) {
    GOT_SIGTTOU.store(true, Ordering::Relaxed);
}
unsafe extern "C" fn handle_sigwinch(_sig: c_int) {
    GOT_SIGWINCH.store(true, Ordering::Relaxed);
}

#[derive(Debug)]
pub enum LocalConnectError {
    NameTooLong,
    UnsafeParentDir,
    TimedOut(u64),
    SocketCreate(io::Error),
    RestoreCwd(io::Error),
    Io(io::Error),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PendingSignals {
    pub sigcont: bool,
    pub sigtstp: bool,
    pub sigttou: bool,
    pub sigwinch: bool,
}

impl PendingSignals {
    #[inline]
    pub fn any(self) -> bool {
        self.sigcont || self.sigtstp || self.sigttou || self.sigwinch
    }
}

#[inline]
pub fn effective_uid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and returns the calling process's effective UID.
    unsafe { geteuid() }
}

pub fn uid_for_username(name: &OsStr) -> Option<u32> {
    let c_name = CString::new(name.as_bytes()).ok()?;
    // SAFETY: `c_name` is a valid NUL-terminated string; single-threaded CLI reads `pw_uid` immediately.
    unsafe {
        let pw = getpwnam(c_name.as_ptr());
        if pw.is_null() {
            None
        } else {
            Some((*pw).pw_uid)
        }
    }
}

pub fn darwin_user_temp_dir() -> Option<PathBuf> {
    if !cfg!(target_os = "macos") {
        return None;
    }
    let mut buf = [0u8; SUN_PATH_CAPACITY];
    // SAFETY: `buf` provides `SUN_PATH_CAPACITY` writable bytes for `confstr`.
    let n = unsafe {
        confstr(
            CS_DARWIN_USER_TEMP_DIR,
            buf.as_mut_ptr().cast::<c_char>(),
            buf.len(),
        )
    };
    if n == 0 || n == usize::MAX {
        return None;
    }
    let len = (n - 1).min(buf.len());
    let end = buf[..len].iter().position(|&b| b == 0).unwrap_or(len);
    if end == 0 {
        return None;
    }
    Some(PathBuf::from(OsStr::from_bytes(&buf[..end])))
}

pub fn executable_dir_accessible(path: &Path) -> bool {
    let Ok(c_path) = CString::new(path.as_os_str().as_bytes()) else {
        return false;
    };
    // SAFETY: `c_path` is a valid NUL-terminated path passed to `faccessat`.
    unsafe { faccessat(AT_FDCWD, c_path.as_ptr(), X_OK, AT_EACCESS) == 0 }
}

pub fn stdout_tty_name() -> Option<String> {
    // SAFETY: `ttyname` takes an integer file descriptor; single-threaded CLI copies any returned CStr immediately.
    unsafe {
        let ptr = ttyname(STDOUT_FILENO);
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }
}

pub fn process_grouping() -> i32 {
    // SAFETY: `tcgetpgrp` and `getpgrp` have no memory safety preconditions.
    unsafe {
        let tcpgrp = tcgetpgrp(STDOUT_FILENO);
        if tcpgrp >= 0 {
            let pgrp = getpgrp();
            if tcpgrp == pgrp { pgrp } else { -pgrp }
        } else {
            0
        }
    }
}

pub fn set_tcp_linger_1s(stream: &TcpStream) -> io::Result<()> {
    let l_arg = Linger {
        l_onoff: 1,
        l_linger: 1,
    };
    // SAFETY: `stream` owns a valid socket fd and `l_arg` is a properly sized `struct linger`.
    let rc = unsafe {
        setsockopt(
            stream.as_raw_fd(),
            SOL_SOCKET_C,
            SO_LINGER_C,
            (&raw const l_arg).cast::<c_void>(),
            core::mem::size_of::<Linger>() as socklen_t,
        )
    };
    if rc < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn open_cloexec_unix_socket() -> Result<OwnedFd, LocalConnectError> {
    // SAFETY: Creates a fresh `AF_UNIX` stream socket and sets `FD_CLOEXEC` before wrapping in `OwnedFd`.
    unsafe {
        let fd = socket(AF_UNIX_C, SOCK_STREAM_C, 0);
        if fd < 0 {
            return Err(LocalConnectError::SocketCreate(io::Error::last_os_error()));
        }
        fcntl(fd, F_SETFD, FD_CLOEXEC);
        Ok(OwnedFd::from_raw_fd(fd))
    }
}

fn connect_raw_with_timeout(
    sockfd: c_int,
    addr: &SockaddrUn,
    timeout_secs: Option<u64>,
) -> Result<(), LocalConnectError> {
    let addr_ptr = (addr as *const SockaddrUn).cast::<Sockaddr>();
    let addr_len = core::mem::size_of::<SockaddrUn>() as socklen_t;

    let Some(limit_secs) = timeout_secs else {
        // SAFETY: `sockfd` is valid and `addr_ptr` points to a full `SockaddrUn`.
        let rc = unsafe { connect(sockfd, addr_ptr, addr_len) };
        return if rc == 0 {
            Ok(())
        } else {
            Err(LocalConnectError::Io(io::Error::last_os_error()))
        };
    };

    // SAFETY: `sockfd` is owned by caller; temporarily toggle `O_NONBLOCK` and poll via `pselect`.
    unsafe {
        let flags = fcntl(sockfd, F_GETFL);
        if flags == -1 || fcntl(sockfd, F_SETFL, flags | O_NONBLOCK) == -1 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }

        let interval = make_timespec(0, (TIMESPEC_HZ / 20) as c_long);
        let end = timespec_add(current_timespec(), make_timespec(limit_secs as time_t, 0));
        let mut outcome: Result<(), LocalConnectError> = Ok(());
        let mut wait_writable = false;

        loop {
            if connect(sockfd, addr_ptr, addr_len) == 0 {
                break;
            }
            let err = *errno();
            if err == EAGAIN || (EWOULDBLOCK != EAGAIN && err == EWOULDBLOCK) {
                if timespec_cmp(end, current_timespec()) <= 0 {
                    return Err(LocalConnectError::TimedOut(limit_secs));
                }
                pselect(
                    0,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                    &raw const interval,
                    core::ptr::null(),
                );
            } else if err == EINPROGRESS {
                wait_writable = true;
                break;
            } else {
                outcome = Err(LocalConnectError::Io(io::Error::from_raw_os_error(err)));
                break;
            }
        }

        if wait_writable {
            let mut fdset: fd_set = core::mem::zeroed();
            let rc = loop {
                FD_ZERO(&raw mut fdset);
                FD_SET(sockfd, &raw mut fdset);
                let now = current_timespec();
                if timespec_cmp(end, now) <= 0 {
                    return Err(LocalConnectError::TimedOut(limit_secs));
                }
                let remaining = timespec_sub(end, now);
                let r = pselect(
                    sockfd + 1,
                    core::ptr::null_mut(),
                    &raw mut fdset,
                    core::ptr::null_mut(),
                    &raw const remaining,
                    core::ptr::null(),
                );
                if r != -1 || *errno() != EINTR {
                    break r;
                }
            };
            if rc == 0 {
                return Err(LocalConnectError::TimedOut(limit_secs));
            } else if rc < 0 {
                outcome = Err(LocalConnectError::Io(io::Error::last_os_error()));
            } else {
                let mut xerrno: c_int = 0;
                let mut xlen = core::mem::size_of::<c_int>() as socklen_t;
                if getsockopt(
                    sockfd,
                    SOL_SOCKET_C,
                    SO_ERROR_C,
                    (&raw mut xerrno).cast::<c_void>(),
                    &raw mut xlen,
                ) == -1
                {
                    outcome = Err(LocalConnectError::Io(io::Error::last_os_error()));
                } else if xerrno != 0 {
                    outcome = Err(LocalConnectError::Io(io::Error::from_raw_os_error(xerrno)));
                }
            }
        }

        if fcntl(sockfd, F_SETFL, flags) == -1 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }
        outcome
    }
}

fn build_sockaddr_un(raw_name: &[u8]) -> Result<SockaddrUn, LocalConnectError> {
    if raw_name.is_empty() || raw_name.len() >= SUN_PATH_CAPACITY || raw_name.contains(&0) {
        return Err(LocalConnectError::NameTooLong);
    }
    #[cfg(target_os = "macos")]
    let mut un = SockaddrUn {
        sun_len: 0,
        sun_family: AF_UNIX_C as u8,
        sun_path: [0; SUN_PATH_CAPACITY],
    };
    #[cfg(not(target_os = "macos"))]
    let mut un = SockaddrUn {
        sun_family: AF_UNIX_C as u16,
        sun_path: [0; SUN_PATH_CAPACITY],
    };
    for (dst, &b) in un.sun_path[..raw_name.len()].iter_mut().zip(raw_name.iter()) {
        *dst = b as c_char;
    }
    Ok(un)
}

pub fn connect_unix_path(
    path: &Path,
    timeout_secs: Option<u64>,
) -> Result<UnixStream, LocalConnectError> {
    let sock = open_cloexec_unix_socket()?;
    let un = build_sockaddr_un(path.as_os_str().as_bytes())?;
    connect_raw_with_timeout(sock.as_raw_fd(), &un, timeout_secs)?;
    Ok(UnixStream::from(sock))
}

pub fn connect_unix_in_parent(
    parent_dir: &Path,
    server_name: &OsStr,
    expected_uid: u32,
    timeout_secs: Option<u64>,
) -> Result<UnixStream, LocalConnectError> {
    let full = parent_dir.join(server_name);
    if full.as_os_str().as_bytes().len() >= SUN_PATH_CAPACITY {
        return Err(LocalConnectError::NameTooLong);
    }
    let un = build_sockaddr_un(server_name.as_bytes())?;
    let c_parent = CString::new(parent_dir.as_os_str().as_bytes())
        .map_err(|_| LocalConnectError::NameTooLong)?;

    let sock = open_cloexec_unix_socket()?;

    // SAFETY: Opens `parent_dir` with `O_NOFOLLOW | O_DIRECTORY`, verifies owner/mode/ACL on the
    // open directory descriptor, connects relative to it, and restores the saved working directory.
    unsafe {
        let dirfd = open(
            c_parent.as_ptr(),
            O_PATH_COMPAT | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC,
        );
        if dirfd < 0 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }
        let dir_guard = OwnedFd::from_raw_fd(dirfd);

        let mut st: stat = core::mem::zeroed();
        if fstat(dir_guard.as_raw_fd(), &raw mut st) != 0 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }
        if st.st_uid != expected_uid || (st.st_mode & (S_IWGRP | S_IWOTH)) != 0 {
            return Err(LocalConnectError::UnsafeParentDir);
        }

        let wdfd = open(c".".as_ptr(), O_PATH_COMPAT | O_CLOEXEC);
        if wdfd < 0 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }
        let wd_guard = OwnedFd::from_raw_fd(wdfd);

        if fchdir(dir_guard.as_raw_fd()) != 0 {
            return Err(LocalConnectError::Io(io::Error::last_os_error()));
        }

        let has_acl = file_has_acl(c".".as_ptr(), &raw const st);
        let connect_res = if has_acl > 0 {
            Err(LocalConnectError::UnsafeParentDir)
        } else if has_acl < 0 {
            Err(LocalConnectError::Io(io::Error::last_os_error()))
        } else {
            connect_raw_with_timeout(sock.as_raw_fd(), &un, timeout_secs)
        };

        if fchdir(wd_guard.as_raw_fd()) != 0 {
            return Err(LocalConnectError::RestoreCwd(io::Error::last_os_error()));
        }

        connect_res?;
    }

    Ok(UnixStream::from(sock))
}

unsafe fn install_handler_respecting_ignore(sig: c_int, handler: SignalHandler, check_ign: bool) {
    // SAFETY: Queries and installs async-signal-safe `sigaction` handlers with empty `sa_mask`.
    unsafe {
        if check_ign {
            let mut oact: sigaction = core::mem::zeroed();
            if sigaction(sig, core::ptr::null(), &raw mut oact) == 0
                && oact.__sigaction_u.__sa_handler.map(|f| f as usize) == Some(SIG_IGN as usize)
            {
                return;
            }
        }
        let mut act: sigaction = core::mem::zeroed();
        act.__sigaction_u.__sa_handler = handler;
        sigemptyset(&raw mut act.sa_mask);
        sigaction(sig, &raw const act, core::ptr::null_mut());
    }
}

pub fn init_tty_signals() {
    // SAFETY: Registers async-signal-safe `AtomicBool` handlers unless currently `SIG_IGN`.
    unsafe {
        install_handler_respecting_ignore(SIGCONT, Some(handle_sigcont), true);
        install_handler_respecting_ignore(SIGTSTP, Some(handle_sigtstp), true);
        install_handler_respecting_ignore(SIGTTOU, Some(handle_sigttou), true);
        install_handler_respecting_ignore(SIGWINCH, Some(handle_sigwinch), true);
    }
}

pub fn take_pending_signals() -> PendingSignals {
    PendingSignals {
        sigcont: GOT_SIGCONT.swap(false, Ordering::Relaxed),
        sigtstp: GOT_SIGTSTP.swap(false, Ordering::Relaxed),
        sigttou: GOT_SIGTTOU.swap(false, Ordering::Relaxed),
        sigwinch: GOT_SIGWINCH.swap(false, Ordering::Relaxed),
    }
}

pub fn send_sigttin(pgrp: i32) {
    // SAFETY: Sends `SIGTTIN` to the specified process group ID (`pgrp < 0`).
    unsafe {
        let _ = kill(pgrp, SIGTTIN);
    }
}

pub fn send_sigwinch(pid: i32) {
    if pid > 0 {
        // SAFETY: Forwards `SIGWINCH` to the known server PID.
        unsafe {
            let _ = kill(pid, SIGWINCH);
        }
    }
}

pub fn suspend_process_group() {
    // SAFETY: Sends `SIGSTOP` to the current process group (`0`) in response to `-suspend`.
    unsafe {
        let _ = kill(0, SIGSTOP);
    }
}

pub fn resignal_tty_stop(is_tstp: bool) {
    let (sig, handler): (c_int, SignalHandler) = if is_tstp {
        (SIGTSTP, Some(handle_sigtstp))
    } else {
        (SIGTTOU, Some(handle_sigttou))
    };
    // SAFETY: Temporarily restores `SIG_DFL`, raises `sig`, and reinstalls the handler upon resume.
    unsafe {
        install_handler_respecting_ignore(sig, None, false);
        let _ = raise(sig);
        install_handler_respecting_ignore(sig, handler, false);
    }
}
