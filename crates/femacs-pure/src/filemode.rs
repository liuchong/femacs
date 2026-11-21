#![forbid(unsafe_code)]
//! Pure Safe Rust POSIX/BSD `ls`-style file mode formatting (`lib/filemode.c`).

const S_IFMT: u32 = 0o170000;
const S_IFIFO: u32 = 0o010000;
const S_IFCHR: u32 = 0o020000;
const S_IFDIR: u32 = 0o040000;
const S_IFBLK: u32 = 0o060000;
const S_IFREG: u32 = 0o100000;
const S_IFLNK: u32 = 0o120000;
const S_IFSOCK: u32 = 0o140000;
const S_IFWHT: u32 = 0o160000;

const S_ISUID: u32 = 0o4000;
const S_ISGID: u32 = 0o2000;
const S_ISVTX: u32 = 0o1000;

const S_IRUSR: u32 = 0o0400;
const S_IWUSR: u32 = 0o0200;
const S_IXUSR: u32 = 0o0100;
const S_IRGRP: u32 = 0o0040;
const S_IWGRP: u32 = 0o0020;
const S_IXGRP: u32 = 0o0010;
const S_IROTH: u32 = 0o0004;
const S_IWOTH: u32 = 0o0002;
const S_IXOTH: u32 = 0o0001;

#[inline]
pub const fn ftypelet(bits: u32) -> u8 {
    match bits & S_IFMT {
        S_IFREG => b'-',
        S_IFDIR => b'd',
        S_IFBLK => b'b',
        S_IFCHR => b'c',
        S_IFLNK => b'l',
        S_IFIFO => b'p',
        S_IFSOCK => b's',
        S_IFWHT => b'w',
        _ => b'?',
    }
}

#[inline]
pub const fn strmode_bytes(mode: u32) -> [u8; 12] {
    let u_exec = if mode & S_ISUID != 0 {
        if mode & S_IXUSR != 0 { b's' } else { b'S' }
    } else if mode & S_IXUSR != 0 {
        b'x'
    } else {
        b'-'
    };

    let g_exec = if mode & S_ISGID != 0 {
        if mode & S_IXGRP != 0 { b's' } else { b'S' }
    } else if mode & S_IXGRP != 0 {
        b'x'
    } else {
        b'-'
    };

    let o_exec = if mode & S_ISVTX != 0 {
        if mode & S_IXOTH != 0 { b't' } else { b'T' }
    } else if mode & S_IXOTH != 0 {
        b'x'
    } else {
        b'-'
    };

    [
        ftypelet(mode),
        if mode & S_IRUSR != 0 { b'r' } else { b'-' },
        if mode & S_IWUSR != 0 { b'w' } else { b'-' },
        u_exec,
        if mode & S_IRGRP != 0 { b'r' } else { b'-' },
        if mode & S_IWGRP != 0 { b'w' } else { b'-' },
        g_exec,
        if mode & S_IROTH != 0 { b'r' } else { b'-' },
        if mode & S_IWOTH != 0 { b'w' } else { b'-' },
        o_exec,
        b' ',
        0,
    ]
}
