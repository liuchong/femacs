// femacs: port of lib-src/emacsclient.c @ fd7afced403.
#![forbid(unsafe_code)]

use femacs_sys::client_os::{
    self, LocalConnectError, SUN_PATH_CAPACITY, connect_unix_in_parent, connect_unix_path,
    darwin_user_temp_dir, effective_uid, executable_dir_accessible, init_tty_signals,
    process_grouping, resignal_tty_stop, send_sigttin, send_sigwinch, set_tcp_linger_1s,
    stdout_tty_name, suspend_process_group, take_pending_signals, uid_for_username,
};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::Duration;

const PACKAGE_VERSION: &str = "32.0.50";
const DEFAULT_TIMEOUT_SECS: i64 = 30;
const AUTH_KEY_LENGTH: usize = 64;
const SEND_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Debug)]
struct Options {
    progname: String,
    nowait: bool,
    quiet: bool,
    suppress_output: bool,
    eval: bool,
    create_frame: bool,
    reuse_frame: bool,
    tty: bool,
    display: Option<OsString>,
    alt_display: Option<OsString>,
    parent_id: Option<OsString>,
    alternate_editor: Option<OsString>,
    socket_name: Option<OsString>,
    server_file: Option<OsString>,
    timeout: i64,
    tramp_prefix: Option<OsString>,
    frame_parameters: Option<OsString>,
    operands: Vec<OsString>,
}

impl Options {
    fn new(progname: String) -> Self {
        Self {
            progname,
            nowait: false,
            quiet: false,
            suppress_output: false,
            eval: false,
            create_frame: false,
            reuse_frame: false,
            tty: false,
            display: None,
            alt_display: None,
            parent_id: None,
            alternate_editor: env::var_os("ALTERNATE_EDITOR"),
            socket_name: None,
            server_file: None,
            timeout: -1,
            tramp_prefix: env::var_os("EMACSCLIENT_TRAMP"),
            frame_parameters: None,
            operands: Vec::new(),
        }
    }

    fn effective_timeout_secs(&self) -> Option<u64> {
        if self.timeout == 0 {
            None
        } else if self.timeout < 0 {
            Some(DEFAULT_TIMEOUT_SECS as u64)
        } else {
            Some(self.timeout as u64)
        }
    }
}

#[derive(Clone, Copy)]
struct LongOptSpec {
    name: &'static str,
    has_arg: bool,
    short: u8,
}

const LONG_OPTS: &[LongOptSpec] = &[
    LongOptSpec { name: "no-wait", has_arg: false, short: b'n' },
    LongOptSpec { name: "quiet", has_arg: false, short: b'q' },
    LongOptSpec { name: "suppress-output", has_arg: false, short: b'u' },
    LongOptSpec { name: "eval", has_arg: false, short: b'e' },
    LongOptSpec { name: "help", has_arg: false, short: b'H' },
    LongOptSpec { name: "version", has_arg: false, short: b'V' },
    LongOptSpec { name: "tty", has_arg: false, short: b't' },
    LongOptSpec { name: "nw", has_arg: false, short: b't' },
    LongOptSpec { name: "no-window-system", has_arg: false, short: b't' },
    LongOptSpec { name: "create-frame", has_arg: false, short: b'c' },
    LongOptSpec { name: "reuse-frame", has_arg: false, short: b'r' },
    LongOptSpec { name: "alternate-editor", has_arg: true, short: b'a' },
    LongOptSpec { name: "frame-parameters", has_arg: true, short: b'F' },
    LongOptSpec { name: "socket-name", has_arg: true, short: b's' },
    LongOptSpec { name: "server-file", has_arg: true, short: b'f' },
    LongOptSpec { name: "display", has_arg: true, short: b'd' },
    LongOptSpec { name: "parent-id", has_arg: true, short: b'p' },
    LongOptSpec { name: "timeout", has_arg: true, short: b'w' },
    LongOptSpec { name: "tramp", has_arg: true, short: b'T' },
];

fn short_takes_arg(ch: u8) -> Option<bool> {
    match ch {
        b'n' | b'q' | b'u' | b'e' | b'H' | b'V' | b't' | b'c' => Some(false),
        b'a' | b'F' | b'w' | b's' | b'f' | b'd' | b'T' => Some(true),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseAction {
    Run,
    Help,
    Version,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    TryHelp(String),
    InvalidTimeout(String),
}

fn match_long_opt(prefix: &[u8]) -> Result<Option<LongOptSpec>, ()> {
    let Ok(prefix_str) = core::str::from_utf8(prefix) else {
        return Ok(None);
    };
    let mut matched: Option<LongOptSpec> = None;
    for &spec in LONG_OPTS {
        if spec.name == prefix_str {
            return Ok(Some(spec));
        }
        if spec.name.starts_with(prefix_str) {
            if matched.is_some() {
                return Err(());
            }
            matched = Some(spec);
        }
    }
    Ok(matched)
}

fn apply_option(opts: &mut Options, code: u8, val: Option<OsString>) -> Result<Option<ParseAction>, ParseError> {
    match code {
        b'a' => opts.alternate_editor = val,
        b's' => opts.socket_name = val,
        b'f' => opts.server_file = val,
        b'd' => opts.display = val,
        b'n' => opts.nowait = true,
        b'w' => {
            let raw = val.unwrap_or_default();
            let text = raw.to_string_lossy();
            match (!text.is_empty())
                .then(|| text.parse::<i64>().ok())
                .flatten()
                .filter(|&v| v >= 0)
            {
                Some(secs) => opts.timeout = secs,
                None => return Err(ParseError::InvalidTimeout(text.into_owned())),
            }
        }
        b'e' => opts.eval = true,
        b'q' => opts.quiet = true,
        b'u' => opts.suppress_output = true,
        b'V' => return Ok(Some(ParseAction::Version)),
        b't' => {
            opts.tty = true;
            opts.create_frame = true;
            opts.reuse_frame = false;
        }
        b'c' => opts.create_frame = true,
        b'r' => {
            opts.create_frame = true;
            if !opts.tty {
                opts.reuse_frame = true;
            }
        }
        b'p' => {
            opts.parent_id = val;
            opts.create_frame = true;
        }
        b'H' => return Ok(Some(ParseAction::Help)),
        b'F' => opts.frame_parameters = val,
        b'T' => opts.tramp_prefix = val,
        _ => {
            return Err(ParseError::TryHelp(format!(
                "{}: unrecognized option",
                opts.progname
            )));
        }
    }
    Ok(None)
}

fn parse_args_into(
    opts: &mut Options,
    args: &[OsString],
    posixly_correct: bool,
) -> Result<ParseAction, ParseError> {
    let mut idx = 0usize;
    let mut literal_only = false;

    while idx < args.len() {
        let arg = &args[idx];
        let bytes = arg.as_bytes();
        idx += 1;

        if literal_only || bytes.len() < 2 || bytes[0] != b'-' {
            opts.operands.push(arg.clone());
            if posixly_correct {
                literal_only = true;
            }
            continue;
        }

        if bytes == b"--" {
            literal_only = true;
            continue;
        }

        let is_double_dash = bytes.starts_with(b"--");
        let body = if is_double_dash { &bytes[2..] } else { &bytes[1..] };

        // `getopt_long_only` tries long options for `--foo`, and for `-foo` whenever `foo` is
        // multi-byte, contains `=`, or is not a known single short option character.
        let try_long = is_double_dash
            || body.len() > 1
            || body.contains(&b'=')
            || short_takes_arg(body[0]).is_none();

        if try_long {
            let (name_part, inline_val) = match body.iter().position(|&b| b == b'=') {
                Some(eq) => (&body[..eq], Some(OsString::from_vec(body[eq + 1..].to_vec()))),
                None => (body, None),
            };
            match match_long_opt(name_part) {
                Ok(Some(spec)) => {
                    let arg_val = if spec.has_arg {
                        if let Some(v) = inline_val {
                            Some(v)
                        } else if idx < args.len() {
                            let v = args[idx].clone();
                            idx += 1;
                            Some(v)
                        } else {
                            return Err(ParseError::TryHelp(format!(
                                "{}: option '{}' requires an argument",
                                opts.progname,
                                arg.to_string_lossy()
                            )));
                        }
                    } else if inline_val.is_some() {
                        return Err(ParseError::TryHelp(format!(
                            "{}: option '--{}' doesn't allow an argument",
                            opts.progname, spec.name
                        )));
                    } else {
                        None
                    };
                    if let Some(act) = apply_option(opts, spec.short, arg_val)? {
                        return Ok(act);
                    }
                    continue;
                }
                Err(()) => {
                    return Err(ParseError::TryHelp(format!(
                        "{}: option '{}' is ambiguous",
                        opts.progname,
                        arg.to_string_lossy()
                    )));
                }
                Ok(None) if is_double_dash => {
                    return Err(ParseError::TryHelp(format!(
                        "{}: unrecognized option '{}'",
                        opts.progname,
                        arg.to_string_lossy()
                    )));
                }
                Ok(None) => {}
            }
        }

        // Fall back to short-option cluster (`-nqe`, `-s/tmp/sock`, `-w 5`).
        let mut pos = 0usize;
        while pos < body.len() {
            let ch = body[pos];
            pos += 1;
            let Some(needs_arg) = short_takes_arg(ch) else {
                return Err(ParseError::TryHelp(format!(
                    "{}: invalid option -- '{}'",
                    opts.progname,
                    char::from(ch)
                )));
            };
            let val = if needs_arg {
                if pos < body.len() {
                    let rest = OsString::from_vec(body[pos..].to_vec());
                    pos = body.len();
                    Some(rest)
                } else if idx < args.len() {
                    let next = args[idx].clone();
                    idx += 1;
                    Some(next)
                } else {
                    return Err(ParseError::TryHelp(format!(
                        "{}: option requires an argument -- '{}'",
                        opts.progname,
                        char::from(ch)
                    )));
                }
            } else {
                None
            };
            if let Some(act) = apply_option(opts, ch, val)? {
                return Ok(act);
            }
        }
    }

    if opts.create_frame && !opts.tty && opts.display.is_none() {
        opts.display = env::var_os("DISPLAY");
    }
    if opts.display.is_none() {
        opts.display = opts.alt_display.take();
    }
    if opts.display.as_ref().is_some_and(|d| d.is_empty()) {
        opts.display = None;
    }
    if opts.create_frame && opts.display.is_none() {
        opts.tty = true;
    }

    Ok(ParseAction::Run)
}

fn print_help(progname: &str) {
    let mut out = io::stdout().lock();
    let _ = write!(
        out,
        "Usage: {progname} [OPTIONS] FILE...\n\
Tell the Emacs server to visit the specified files.\n\
Every FILE can be either just a FILENAME or [+LINE[:COLUMN]] FILENAME.\n\n\
The following OPTIONS are accepted:\n\
-V, --version\t\tJust print version info and return\n\
-H, --help    \t\tPrint this usage information message\n\
-nw, -t, --tty, --no-window-system\n\
\t\t\tOpen a new Emacs frame on the current terminal\n\
-c, --create-frame    \tCreate a new frame instead of trying to\n\
\t\t\tuse the current Emacs frame\n\
-r, --reuse-frame\tCreate a new frame if none exists, otherwise\n\
\t\t\tuse the current Emacs frame\n\
-F ALIST, --frame-parameters=ALIST\n\
\t\t\tSet the parameters of a new frame\n\
-e, --eval    \t\tEvaluate the FILE arguments as Elisp expressions\n\
-n, --no-wait\t\tDon't wait for the server to return\n\
-w, --timeout=SECONDS\tSeconds to wait before timing out\n\
-q, --quiet\t\tDon't display messages on success\n\
-u, --suppress-output   Don't display return values from the server\n\
-d DISPLAY, --display=DISPLAY\n\
\t\t\tVisit the file in the given display\n\
--parent-id=ID          Open in parent window ID, via XEmbed\n\
-s SOCKET, --socket-name=SOCKET\n\
\t\t\tSet filename of the UNIX socket for communication\n\
-f SERVER, --server-file=SERVER\n\
\t\t\tSet filename of the TCP authentication file\n\
-a EDITOR, --alternate-editor=EDITOR\n\
\t\t\tEditor to fallback to if the server is not running\n\
\t\t\tIf EDITOR is the empty string, start Emacs in daemon\n\
\t\t\tmode and try connecting again\n\
-T PREFIX, --tramp=PREFIX\n\
                        PREFIX to prepend to filenames sent by emacsclient\n\
                        for locating files remotely via Tramp\n\n\
Report bugs with M-x report-emacs-bug.\n"
    );
    let _ = out.flush();
}

fn quote_argument_bytes(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len() * 2);
    if src.first() == Some(&b'-') {
        out.push(b'&');
    }
    for &c in src {
        match c {
            b' ' => {
                out.push(b'&');
                out.push(b'_');
            }
            b'\n' => {
                out.push(b'&');
                out.push(b'n');
            }
            b'&' => {
                out.push(b'&');
                out.push(b'&');
            }
            other => out.push(other),
        }
    }
    out
}

fn unquote_argument_bytes(src: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(src.len());
    let mut i = 0usize;
    while i < src.len() {
        let c = src[i];
        i += 1;
        if c == b'&' {
            let Some(&next) = src.get(i) else {
                break;
            };
            i += 1;
            match next {
                b'_' => out.push(b' '),
                b'n' => out.push(b'\n'),
                other => out.push(other),
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn is_position_arg(bytes: &[u8]) -> bool {
    bytes.len() >= 2
        && bytes[0] == b'+'
        && bytes[1..]
            .iter()
            .all(|&b| b.is_ascii_digit() || b == b':')
}

fn split_alternate_editor_tokens(raw: &[u8]) -> Vec<OsString> {
    let mut tokens = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        while i < raw.len() && raw[i] == b' ' {
            i += 1;
        }
        if i >= raw.len() {
            break;
        }
        let sep = if raw[i] == b'"' {
            i += 1;
            b'"'
        } else {
            b' '
        };
        let start = i;
        while i < raw.len() && raw[i] != sep {
            i += 1;
        }
        tokens.push(OsString::from_vec(raw[start..i].to_vec()));
        if i < raw.len() {
            i += 1;
        }
    }
    tokens
}

fn fail_or_exec_alternate(opts: &Options) -> ! {
    if let Some(ref alt) = opts.alternate_editor {
        let toks = split_alternate_editor_tokens(alt.as_bytes());
        if let Some((prog, prefix_args)) = toks.split_first() {
            let mut cmd = Command::new(prog);
            cmd.args(prefix_args);
            cmd.args(&opts.operands);
            let err = cmd.exec();
            eprintln!(
                "{}: error executing alternate editor \"{}\": {}",
                opts.progname,
                alt.to_string_lossy(),
                err
            );
        }
    }
    process::exit(1);
}

fn current_dir_preferring_pwd() -> io::Result<PathBuf> {
    if let Some(pwd_os) = env::var_os("PWD") {
        let pwd_bytes = pwd_os.as_bytes();
        if !pwd_bytes.is_empty() && pwd_bytes.len() < 1024 && pwd_bytes[0] == b'/' {
            let pwd_path = PathBuf::from(&pwd_os);
            if let (Ok(pwd_meta), Ok(dot_meta)) = (fs::metadata(&pwd_path), fs::metadata(".")) {
                if pwd_meta.dev() == dot_meta.dev() && pwd_meta.ino() == dot_meta.ino() {
                    return Ok(pwd_path);
                }
            }
        }
    }
    env::current_dir()
}

enum ServerStream {
    Unix(UnixStream),
    Tcp(TcpStream),
}

impl ServerStream {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        match self {
            Self::Unix(s) => s.set_read_timeout(dur),
            Self::Tcp(s) => s.set_read_timeout(dur),
        }
    }

    fn write_raw(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Unix(s) => s.write(buf),
            Self::Tcp(s) => s.write(buf),
        }
    }

    fn read_raw(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Unix(s) => s.read(buf),
            Self::Tcp(s) => s.read(buf),
        }
    }
}

struct Connection {
    stream: ServerStream,
    send_buf: Vec<u8>,
}

impl Connection {
    fn new(stream: ServerStream) -> Self {
        Self {
            stream,
            send_buf: Vec::with_capacity(SEND_BUFFER_SIZE),
        }
    }

    fn send_bytes(&mut self, mut data: &[u8], opts: &Options, emacs_pid: i32) {
        while !data.is_empty() {
            let space = SEND_BUFFER_SIZE - self.send_buf.len();
            let part = data.len().min(space);
            self.send_buf.extend_from_slice(&data[..part]);
            data = &data[part..];

            if self.send_buf.len() == SEND_BUFFER_SIZE
                || self.send_buf.last() == Some(&b'\n')
            {
                let total = self.send_buf.len();
                let sent = loop {
                    match self.stream.write_raw(&self.send_buf) {
                        Ok(n) => break n,
                        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                            let pending = take_pending_signals();
                            if pending.sigwinch {
                                send_sigwinch(emacs_pid);
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "{}: failed to send {} bytes to socket: {}",
                                opts.progname, total, e
                            );
                            fail_or_exec_alternate(opts);
                        }
                    }
                };
                self.send_buf.drain(..sent);
            }
        }
    }

    fn send_str(&mut self, text: &str, opts: &Options, emacs_pid: i32) {
        self.send_bytes(text.as_bytes(), opts, emacs_pid);
    }

    fn quote_and_send(&mut self, bytes: &[u8], opts: &Options, emacs_pid: i32) {
        let quoted = quote_argument_bytes(bytes);
        self.send_bytes(&quoted, opts, emacs_pid);
    }
}

fn act_on_signals(conn: &mut Connection, opts: &Options, emacs_pid: i32) {
    loop {
        let pending = take_pending_signals();
        if !pending.any() {
            break;
        }
        if pending.sigcont {
            let grouping = process_grouping();
            if grouping < 0 {
                if opts.tty {
                    send_sigttin(grouping);
                }
            } else {
                conn.send_str("-resume \n", opts, emacs_pid);
            }
        }
        if pending.sigtstp {
            conn.send_str("-suspend \n", opts, emacs_pid);
            resignal_tty_stop(true);
        }
        if pending.sigttou {
            conn.send_str("-suspend \n", opts, emacs_pid);
            resignal_tty_stop(false);
        }
        if pending.sigwinch && emacs_pid > 0 {
            send_sigwinch(emacs_pid);
        }
    }
}

fn open_tcp_config_file(config_file: &OsStr) -> Option<File> {
    let path = Path::new(config_file);
    if path.is_absolute() {
        return File::open(path).ok();
    }
    let home = env::var_os("HOME").map(PathBuf::from);
    let xdg = env::var_os("XDG_CONFIG_HOME").map(PathBuf::from);

    if let Some(ref h) = home {
        let p = h.join(".emacs.d/server").join(config_file);
        if let Ok(f) = File::open(p) {
            return Some(f);
        }
    }
    if let Some(ref x) = xdg {
        let p = x.join("emacs/server").join(config_file);
        if let Ok(f) = File::open(p) {
            return Some(f);
        }
    } else if let Some(ref h) = home {
        let p = h.join(".config/emacs/server").join(config_file);
        if let Ok(f) = File::open(p) {
            return Some(f);
        }
    }
    None
}

fn parse_tcp_server_config<R: Read>(progname: &str, reader: R) -> (SocketAddrV4, [u8; AUTH_KEY_LENGTH]) {
    let mut reader = BufReader::new(reader);
    let mut first_line = String::new();
    let n = reader.read_line(&mut first_line).unwrap_or(0);
    let trimmed = first_line.trim_end_matches(&['\r', '\n'][..]);
    let Some((ip_str, port_str)) = (n > 0).then(|| trimmed.split_once(':')).flatten() else {
        eprintln!("{progname}: invalid configuration info");
        process::exit(1);
    };
    let ip: Ipv4Addr = ip_str.parse().unwrap_or(Ipv4Addr::UNSPECIFIED);
    let port: u16 = port_str
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    let mut auth = [0u8; AUTH_KEY_LENGTH];
    if reader.read_exact(&mut auth).is_err() {
        eprintln!("{progname}: cannot read authentication info");
        process::exit(1);
    }
    (SocketAddrV4::new(ip, port), auth)
}

fn try_connect_tcp(opts: &Options, server_file: &OsStr) -> Option<Connection> {
    let file = open_tcp_config_file(server_file)?;
    let (addr, auth) = parse_tcp_server_config(&opts.progname, file);
    if *addr.ip() != Ipv4Addr::LOCALHOST && !opts.quiet {
        println!("{}: connected to remote socket at {}", opts.progname, addr.ip());
    }

    let sock_addr = SocketAddr::V4(addr);
    let stream_res = match opts.effective_timeout_secs() {
        Some(secs) => TcpStream::connect_timeout(&sock_addr, Duration::from_secs(secs)),
        None => TcpStream::connect(&sock_addr),
    };
    let stream = match stream_res {
        Ok(s) => s,
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
            let secs = opts.effective_timeout_secs().unwrap_or(DEFAULT_TIMEOUT_SECS as u64);
            let unit = if secs == 1 { "second" } else { "seconds" };
            eprintln!("{}: Connection timed out after {secs} {unit}", opts.progname);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("{}: connect: {e}", opts.progname);
            return None;
        }
    };

    if let Err(e) = set_tcp_linger_1s(&stream) {
        eprintln!("{}: setsockopt: {e}", opts.progname);
    }

    let mut conn = Connection::new(ServerStream::Tcp(stream));
    conn.send_str("-auth ", opts, 0);
    conn.send_bytes(&auth, opts, 0);
    conn.send_str(" ", opts, 0);
    Some(conn)
}

fn try_connect_local(opts: &Options, server_name: &OsStr) -> Option<Connection> {
    let uid = effective_uid();
    let timeout_secs = opts.effective_timeout_secs();
    let name_bytes = server_name.as_bytes();

    let report_fatal = |err: &LocalConnectError, display_path: &Path| -> ! {
        match err {
            LocalConnectError::NameTooLong => {
                eprintln!(
                    "{}: socket-name {}... too long",
                    opts.progname,
                    display_path.to_string_lossy()
                );
                fail_or_exec_alternate(opts);
            }
            LocalConnectError::TimedOut(secs) => {
                let unit = if *secs == 1 { "second" } else { "seconds" };
                eprintln!("{}: Connection timed out after {secs} {unit}", opts.progname);
                process::exit(1);
            }
            LocalConnectError::SocketCreate(e) => {
                eprintln!("{}: can't create socket: {e}", opts.progname);
                fail_or_exec_alternate(opts);
            }
            LocalConnectError::RestoreCwd(e) => {
                eprintln!("{}: .: {e}", opts.progname);
                process::exit(1);
            }
            _ => unreachable!(),
        }
    };

    if name_bytes.contains(&b'/') {
        let p = Path::new(server_name);
        match connect_unix_path(p, timeout_secs) {
            Ok(s) => return Some(Connection::new(ServerStream::Unix(s))),
            Err(
                ref e @ (LocalConnectError::NameTooLong
                | LocalConnectError::TimedOut(_)
                | LocalConnectError::SocketCreate(_)
                | LocalConnectError::RestoreCwd(_)),
            ) => report_fatal(e, p),
            Err(LocalConnectError::UnsafeParentDir) => {
                eprintln!(
                    "{}: Invalid permissions on parent directory of socket: {}",
                    opts.progname,
                    p.to_string_lossy()
                );
                return None;
            }
            Err(LocalConnectError::Io(e)) => {
                if e.kind() == io::ErrorKind::NotFound {
                    if !opts.quiet || opts.alternate_editor.is_none() {
                        eprintln!(
                            "{}: can't find socket; have you started the server?\n{}: To start the server in Emacs, type \"M-x server-start\".",
                            opts.progname, opts.progname
                        );
                    }
                } else {
                    eprintln!("{}: can't connect to {}: {e}", opts.progname, p.to_string_lossy());
                }
                return None;
            }
        }
    }

    if let Some(xdg_runtime) = env::var_os("XDG_RUNTIME_DIR") {
        let full = PathBuf::from(xdg_runtime).join("emacs").join(server_name);
        match connect_unix_path(&full, timeout_secs) {
            Ok(s) => return Some(Connection::new(ServerStream::Unix(s))),
            Err(
                ref e @ (LocalConnectError::NameTooLong
                | LocalConnectError::TimedOut(_)
                | LocalConnectError::SocketCreate(_)
                | LocalConnectError::RestoreCwd(_)),
            ) => report_fatal(e, &full),
            Err(LocalConnectError::UnsafeParentDir) => {
                eprintln!(
                    "{}: Invalid permissions on parent directory of socket: {}",
                    opts.progname,
                    full.to_string_lossy()
                );
                return None;
            }
            Err(LocalConnectError::Io(e)) => {
                if e.kind() == io::ErrorKind::NotFound {
                    if !opts.quiet || opts.alternate_editor.is_none() {
                        eprintln!(
                            "{}: can't find socket; have you started the server?\n{}: To start the server in Emacs, type \"M-x server-start\".",
                            opts.progname, opts.progname
                        );
                    }
                } else {
                    eprintln!(
                        "{}: can't connect to {}: {e}",
                        opts.progname,
                        full.to_string_lossy()
                    );
                }
                return None;
            }
        }
    }

    let tmp_base = env::var_os("TMPDIR")
        .map(PathBuf::from)
        .or_else(darwin_user_temp_dir)
        .unwrap_or_else(|| PathBuf::from("/tmp"));

    let make_parent = |owner_uid: u32| tmp_base.join(format!("emacs{owner_uid}"));
    let parent = make_parent(uid);
    let display_sock = parent.join(server_name);
    if tmp_base.as_os_str().as_bytes().len() >= SUN_PATH_CAPACITY {
        eprintln!(
            "{}: socket-name {}... too long",
            opts.progname,
            tmp_base.to_string_lossy()
        );
        fail_or_exec_alternate(opts);
    }

    let mut first_err = match connect_unix_in_parent(&parent, server_name, uid, timeout_secs) {
        Ok(s) => return Some(Connection::new(ServerStream::Unix(s))),
        Err(
            ref e @ (LocalConnectError::NameTooLong
            | LocalConnectError::TimedOut(_)
            | LocalConnectError::SocketCreate(_)
            | LocalConnectError::RestoreCwd(_)),
        ) => report_fatal(e, &display_sock),
        Err(other) => other,
    };

    if let Some(user_name) = env::var_os("LOGNAME").or_else(|| env::var_os("USER")) {
        if let Some(pw_uid) = uid_for_username(&user_name) {
            if pw_uid != uid {
                let alt_parent = make_parent(pw_uid);
                let alt_display_sock = alt_parent.join(server_name);
                match connect_unix_in_parent(&alt_parent, server_name, pw_uid, timeout_secs) {
                    Ok(s) => return Some(Connection::new(ServerStream::Unix(s))),
                    Err(LocalConnectError::NameTooLong) => {
                        eprintln!(
                            "{}: socket-name {}... too long",
                            opts.progname,
                            alt_display_sock.to_string_lossy()
                        );
                        process::exit(1);
                    }
                    Err(
                        ref e @ (LocalConnectError::TimedOut(_)
                        | LocalConnectError::SocketCreate(_)
                        | LocalConnectError::RestoreCwd(_)),
                    ) => report_fatal(e, &alt_display_sock),
                    Err(other) => first_err = other,
                }
            }
        }
    }

    match first_err {
        LocalConnectError::UnsafeParentDir => {
            eprintln!(
                "{}: Invalid permissions on parent directory of socket: {}",
                opts.progname,
                display_sock.to_string_lossy()
            );
        }
        LocalConnectError::Io(e) if e.kind() == io::ErrorKind::NotFound => {
            let run_user = PathBuf::from(format!("/run/user/{uid}"));
            if executable_dir_accessible(&run_user) {
                eprintln!(
                    "{}: Should XDG_RUNTIME_DIR='{}' be in the environment?\n{}: (Be careful: XDG_RUNTIME_DIR is security-related.)",
                    opts.progname,
                    run_user.display(),
                    opts.progname
                );
            }
            if !opts.quiet || opts.alternate_editor.is_none() {
                eprintln!(
                    "{}: can't find socket; have you started the server?\n{}: To start the server in Emacs, type \"M-x server-start\".",
                    opts.progname, opts.progname
                );
            }
        }
        LocalConnectError::Io(e) => {
            eprintln!(
                "{}: can't connect to {}: {e}",
                opts.progname,
                display_sock.to_string_lossy()
            );
        }
        _ => {}
    }

    None
}

fn establish_connection(opts: &mut Options, no_exit_if_error: bool) -> Option<Connection> {
    if opts.socket_name.is_none() {
        opts.socket_name = env::var_os("EMACS_SOCKET_NAME");
    }
    if let Some(sock_name) = opts.socket_name.clone() {
        let conn = try_connect_local(opts, &sock_name);
        if conn.is_some() || no_exit_if_error {
            return conn;
        }
        eprintln!(
            "{}: error accessing socket \"{}\"",
            opts.progname,
            sock_name.to_string_lossy()
        );
        process::exit(1);
    }

    let local_server_file = opts
        .server_file
        .clone()
        .or_else(|| env::var_os("EMACS_SERVER_FILE"));
    if let Some(sf) = local_server_file {
        let conn = try_connect_tcp(opts, &sf);
        if conn.is_some() || no_exit_if_error {
            return conn;
        }
        eprintln!(
            "{}: error accessing server file \"{}\"",
            opts.progname,
            sf.to_string_lossy()
        );
        process::exit(1);
    }

    let default_name = OsStr::new("server");
    if let Some(conn) = try_connect_local(opts, default_name) {
        return Some(conn);
    }
    let tcp_conn = try_connect_tcp(opts, default_name);
    if tcp_conn.is_some() || no_exit_if_error {
        return tcp_conn;
    }

    eprintln!(
        "{}: No socket or alternate editor.  Please use:\n\n\
\t--socket-name\n\
\t--server-file      (or environment variable EMACS_SERVER_FILE)\n\
\t--alternate-editor (or environment variable ALTERNATE_EDITOR)",
        opts.progname
    );
    process::exit(1);
}

fn start_daemon_and_retry(opts: &mut Options) -> Connection {
    let mut cmd = Command::new("emacs");
    if let Some(ref sock) = opts.socket_name {
        let mut arg = OsString::from("--daemon=");
        arg.push(sock);
        cmd.arg(arg);
    } else {
        cmd.arg("--daemon");
    }
    match cmd.status() {
        Ok(status) if status.success() => {
            if !opts.quiet {
                eprintln!("Emacs daemon should have started, trying to connect again");
            }
        }
        Ok(_) => {
            eprintln!("Error: Could not start the Emacs daemon");
            process::exit(1);
        }
        Err(_) => {
            eprintln!("{}: error starting emacs daemon", opts.progname);
            process::exit(1);
        }
    }

    if let Some(conn) = establish_connection(opts, true) {
        conn
    } else {
        eprintln!("Error: Cannot connect even after starting the Emacs daemon");
        process::exit(1);
    }
}

fn find_tty(opts: &Options, noabort: bool) -> Option<(String, OsString)> {
    let Some(tty_name) = stdout_tty_name() else {
        if noabort {
            return None;
        }
        eprintln!("{}: could not get terminal name", opts.progname);
        fail_or_exec_alternate(opts);
    };
    let Some(tty_type) = env::var_os("TERM") else {
        if noabort {
            return None;
        }
        eprintln!(
            "{}: please set the TERM variable to your terminal type",
            opts.progname
        );
        fail_or_exec_alternate(opts);
    };
    if let Some(inside) = env::var_os("INSIDE_EMACS") {
        let inside_lossy = inside.to_string_lossy();
        let term_lossy = tty_type.to_string_lossy();
        if inside_lossy.contains(",term:") && term_lossy.starts_with("eterm") {
            if noabort {
                return None;
            }
            eprintln!(
                "{}: opening a frame in an Emacs term buffer is not supported",
                opts.progname
            );
            fail_or_exec_alternate(opts);
        }
    }
    Some((tty_name, tty_type))
}

fn run_client() -> i32 {
    let raw_args: Vec<OsString> = env::args_os().collect();
    let progname = raw_args
        .first()
        .map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "emacsclient".to_string());

    let mut opts = Options::new(progname);
    let rest = if raw_args.len() > 1 { &raw_args[1..] } else { &[][..] };
    let posixly_correct = env::var_os("POSIXLY_CORRECT").is_some();

    match parse_args_into(&mut opts, rest, posixly_correct) {
        Ok(ParseAction::Help) => {
            print_help(&opts.progname);
            return 0;
        }
        Ok(ParseAction::Version) => {
            println!("emacsclient {PACKAGE_VERSION}");
            return 0;
        }
        Ok(ParseAction::Run) => {}
        Err(ParseError::InvalidTimeout(bad)) => {
            eprintln!("Invalid timeout: \"{bad}\"");
            return 1;
        }
        Err(ParseError::TryHelp(msg)) => {
            eprintln!("{msg}");
            eprintln!("Try '{} --help' for more information", opts.progname);
            return 1;
        }
    }

    if opts.operands.is_empty() && !opts.eval && !opts.create_frame {
        eprintln!(
            "{}: file name or argument required\nTry '{} --help' for more information",
            opts.progname, opts.progname
        );
        return 1;
    }

    if opts.tty {
        let grouping = process_grouping();
        if grouping < 0 {
            send_sigttin(grouping);
        }
    }

    let start_daemon_if_needed = opts
        .alternate_editor
        .as_ref()
        .is_some_and(|s| s.is_empty());

    let no_exit_if_error = opts.alternate_editor.is_some() || start_daemon_if_needed;
    let mut conn = match establish_connection(&mut opts, no_exit_if_error) {
        Some(c) => c,
        None => {
            if !start_daemon_if_needed {
                fail_or_exec_alternate(&opts);
            }
            start_daemon_and_retry(&mut opts)
        }
    };

    let cwd = match current_dir_preferring_pwd() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("{}: Cannot get current working directory", opts.progname);
            fail_or_exec_alternate(&opts);
        }
    };

    let mut emacs_pid: i32 = 0;

    if opts.create_frame {
        for (k, v) in env::vars_os() {
            let mut entry = k.into_vec();
            entry.push(b'=');
            entry.extend_from_slice(v.as_bytes());
            conn.send_str("-env ", &opts, emacs_pid);
            conn.quote_and_send(&entry, &opts, emacs_pid);
            conn.send_str(" ", &opts, emacs_pid);
        }
    }

    conn.send_str("-dir ", &opts, emacs_pid);
    if let Some(ref tramp) = opts.tramp_prefix {
        conn.quote_and_send(tramp.as_bytes(), &opts, emacs_pid);
    }
    conn.quote_and_send(cwd.as_os_str().as_bytes(), &opts, emacs_pid);
    conn.send_str("/ ", &opts, emacs_pid);

    let mut skiplf = true;
    let mut exit_status: i32 = 0;
    let mut last_read_failed = false;

    'retry: loop {
        if opts.nowait {
            conn.send_str("-nowait ", &opts, emacs_pid);
        }
        if !opts.create_frame || opts.reuse_frame {
            conn.send_str("-current-frame ", &opts, emacs_pid);
        }
        if let Some(ref disp) = opts.display {
            conn.send_str("-display ", &opts, emacs_pid);
            conn.quote_and_send(disp.as_bytes(), &opts, emacs_pid);
            conn.send_str(" ", &opts, emacs_pid);
        }
        if let Some(ref pid_str) = opts.parent_id {
            conn.send_str("-parent-id ", &opts, emacs_pid);
            conn.quote_and_send(pid_str.as_bytes(), &opts, emacs_pid);
            conn.send_str(" ", &opts, emacs_pid);
        }
        if let Some(ref fparams) = opts.frame_parameters {
            if opts.create_frame {
                conn.send_str("-frame-parameters ", &opts, emacs_pid);
                conn.quote_and_send(fparams.as_bytes(), &opts, emacs_pid);
                conn.send_str(" ", &opts, emacs_pid);
            }
        }

        if opts.create_frame || !opts.eval {
            if let Some((tty_name, tty_type)) = find_tty(&opts, !opts.tty) {
                init_tty_signals();
                conn.send_str("-tty ", &opts, emacs_pid);
                conn.quote_and_send(tty_name.as_bytes(), &opts, emacs_pid);
                conn.send_str(" ", &opts, emacs_pid);
                conn.quote_and_send(tty_type.as_bytes(), &opts, emacs_pid);
                conn.send_str(" ", &opts, emacs_pid);
            }
        }

        if opts.create_frame && !opts.tty {
            conn.send_str("-window-system ", &opts, emacs_pid);
        }

        if !opts.operands.is_empty() {
            for op in &opts.operands {
                let raw = op.as_bytes();
                if opts.eval {
                    conn.send_str("-eval ", &opts, emacs_pid);
                    conn.quote_and_send(raw, &opts, emacs_pid);
                    conn.send_str(" ", &opts, emacs_pid);
                } else if is_position_arg(raw) {
                    conn.send_str("-position ", &opts, emacs_pid);
                    conn.quote_and_send(raw, &opts, emacs_pid);
                    conn.send_str(" ", &opts, emacs_pid);
                } else {
                    conn.send_str("-file ", &opts, emacs_pid);
                    if let Some(ref tramp) = opts.tramp_prefix {
                        if raw.first() == Some(&b'/') {
                            conn.quote_and_send(tramp.as_bytes(), &opts, emacs_pid);
                        }
                    }
                    conn.quote_and_send(raw, &opts, emacs_pid);
                    conn.send_str(" ", &opts, emacs_pid);
                }
            }
        } else if opts.eval {
            let stdin = io::stdin();
            let mut reader = stdin.lock();
            let mut line = Vec::new();
            while reader.read_until(b'\n', &mut line).unwrap_or(0) > 0 {
                conn.send_str("-eval ", &opts, emacs_pid);
                conn.quote_and_send(&line, &opts, emacs_pid);
                line.clear();
            }
            conn.send_str(" ", &opts, emacs_pid);
        }

        conn.send_str("\n", &opts, emacs_pid);

        if !opts.eval && !opts.tty && !opts.nowait && !opts.quiet && process_grouping() >= 0 {
            print!("Waiting for Emacs...");
            skiplf = false;
        }
        let _ = io::stdout().flush();

        if let Some(secs) = opts.effective_timeout_secs() {
            let _ = conn.stream.set_read_timeout(Some(Duration::from_secs(secs)));
        } else {
            let _ = conn.stream.set_read_timeout(None);
        }

        let mut saw_response = false;
        let mut recv_buf: Vec<u8> = Vec::new();
        let mut chunk = [0u8; 4096];

        while exit_status == 0 {
            let mut msg_showed = opts.quiet;
            let read_n = loop {
                act_on_signals(&mut conn, &opts, emacs_pid);
                match conn.stream.read_raw(&mut chunk) {
                    Ok(n) => break Ok(n),
                    Err(e) => {
                        let is_timeout = matches!(
                            e.kind(),
                            io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                        );
                        let mut retry = is_timeout;
                        if retry && !saw_response {
                            if opts.timeout > 0 {
                                eprint!(
                                    "\nServer not responding; timed out after {} seconds",
                                    opts.timeout
                                );
                                let _ = io::stderr().flush();
                                retry = false;
                            } else if !msg_showed {
                                msg_showed = true;
                                eprint!("\nServer not responding; use Ctrl+C to break");
                                let _ = io::stderr().flush();
                            }
                        }
                        if e.kind() == io::ErrorKind::Interrupted || retry {
                            continue;
                        }
                        break Err(e);
                    }
                }
            };

            let n = match read_n {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => {
                    last_read_failed = true;
                    break;
                }
            };

            recv_buf.extend_from_slice(&chunk[..n]);
            saw_response = true;

            let mut cursor = 0usize;
            while let Some(rel_nl) = recv_buf[cursor..].iter().position(|&b| b == b'\n') {
                let line = &recv_buf[cursor..cursor + rel_nl];
                cursor += rel_nl + 1;

                if let Some(rest) = line.strip_prefix(b"-emacs-pid ") {
                    let pid_text = core::str::from_utf8(rest).unwrap_or("").trim();
                    if let Ok(p) = pid_text.parse::<u64>() {
                        emacs_pid = p as i32;
                    }
                } else if line.starts_with(b"-window-system-unsupported ") {
                    if let Some(alt) = opts.alt_display.take() {
                        opts.display = Some(alt);
                    } else {
                        opts.nowait = false;
                        opts.tty = true;
                    }
                    continue 'retry;
                } else if let Some(rest) = line.strip_prefix(b"-print ") {
                    if !opts.suppress_output {
                        let unq = unquote_argument_bytes(rest);
                        let mut out = io::stdout().lock();
                        if !skiplf {
                            let _ = out.write_all(b"\n");
                        }
                        let _ = out.write_all(&unq);
                        let _ = out.flush();
                        if let Some(&last) = unq.last() {
                            skiplf = last == b'\n';
                        }
                    }
                } else if let Some(rest) = line.strip_prefix(b"-print-nonl ") {
                    if !opts.suppress_output {
                        let unq = unquote_argument_bytes(rest);
                        let mut out = io::stdout().lock();
                        let _ = out.write_all(&unq);
                        let _ = out.flush();
                        if let Some(&last) = unq.last() {
                            skiplf = last == b'\n';
                        }
                    }
                } else if let Some(rest) = line.strip_prefix(b"-error ") {
                    let unq = unquote_argument_bytes(rest);
                    if !skiplf {
                        println!();
                    }
                    let mut err = io::stderr().lock();
                    let _ = err.write_all(b"*ERROR*: ");
                    let _ = err.write_all(&unq);
                    let _ = err.flush();
                    if let Some(&last) = unq.last() {
                        skiplf = last == b'\n';
                    }
                    exit_status = 1;
                } else if line.starts_with(b"-suspend ") {
                    if !skiplf {
                        println!();
                    }
                    skiplf = true;
                    suspend_process_group();
                } else {
                    let mut out = io::stdout().lock();
                    if !skiplf {
                        let _ = out.write_all(b"\n");
                    }
                    let _ = out.write_all(b"*ERROR*: Unknown message: ");
                    let _ = out.write_all(line);
                    let _ = out.write_all(b"\n");
                    let _ = out.flush();
                    skiplf = true;
                }
            }

            if cursor > 0 {
                recv_buf.drain(..cursor);
            }
        }

        break;
    }

    if !skiplf && process_grouping() >= 0 {
        println!();
    }
    if last_read_failed {
        exit_status = 1;
    }

    let _ = client_os::take_pending_signals();
    exit_status
}

fn main() {
    let code = run_client();
    process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_and_unquote_roundtrip_special_bytes_and_utf8() {
        let cases: &[&[u8]] = &[
            b"-eval-looking-arg",
            b"hello world\nsecond & line",
            "λ Unicode 文件名 & 空格\n".as_bytes(),
            b"",
            b"&_literal",
        ];
        for &input in cases {
            let quoted = quote_argument_bytes(input);
            assert!(!quoted.contains(&b' '));
            assert!(!quoted.contains(&b'\n'));
            assert_ne!(quoted.first(), Some(&b'-'));
            let restored = unquote_argument_bytes(&quoted);
            assert_eq!(restored, input);
        }
    }

    #[test]
    fn parse_options_supports_long_only_clusters_positions_and_double_dash() {
        let mut opts = Options::new("emacsclient".into());
        let args: Vec<OsString> = [
            "-nw",
            "-q",
            "--socket-name=/tmp/test.sock",
            "-w",
            "5",
            "+42:7",
            "file.el",
            "--",
            "-literal-dash.el",
        ]
        .into_iter()
        .map(OsString::from)
        .collect();
        let act = parse_args_into(&mut opts, &args, false).expect("valid options");
        assert_eq!(act, ParseAction::Run);
        assert!(opts.tty);
        assert!(opts.create_frame);
        assert!(opts.quiet);
        assert_eq!(opts.timeout, 5);
        assert_eq!(opts.socket_name, Some(OsString::from("/tmp/test.sock")));
        assert_eq!(
            opts.operands,
            vec![
                OsString::from("+42:7"),
                OsString::from("file.el"),
                OsString::from("-literal-dash.el"),
            ]
        );
        assert!(is_position_arg(b"+42:7"));
        assert!(is_position_arg(b"+10"));
        assert!(!is_position_arg(b"+notpos"));
    }

    #[test]
    fn alternate_editor_tokenizer_handles_quotes_and_spaces() {
        let toks = split_alternate_editor_tokens(b"  \"/Applications/My Editor/bin/ed\" --flag  plain ");
        assert_eq!(
            toks,
            vec![
                OsString::from("/Applications/My Editor/bin/ed"),
                OsString::from("--flag"),
                OsString::from("plain"),
            ]
        );
    }
}
