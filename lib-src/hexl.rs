// femacs: port of lib-src/hexl.c @ fd7afced403.
#![forbid(unsafe_code)]
//! Safe Rust implementation of `lib-src/hexl` (hex dump and undump utility).

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process::ExitCode;

fn hex_nibble(byte: u8) -> i32 {
    let b = i32::from(byte);
    if (i32::from(b'0')..=i32::from(b'9')).contains(&b) {
        b - i32::from(b'0')
    } else {
        b.wrapping_sub(i32::from(b'a') - 10)
    }
}

fn write_error(progname: &str) -> ExitCode {
    let _ = writeln!(io::stderr(), "{progname}: write error");
    ExitCode::FAILURE
}

fn process_decode<R: Read, W: Write>(reader: R, out: &mut W, group_by: usize) -> Result<bool, ()> {
    let mut bytes = reader.bytes();
    loop {
        let b = match bytes.next() {
            Some(Ok(v)) => v,
            Some(Err(_)) => return Ok(true),
            None => break,
        };
        if b != b' ' {
            continue;
        }
        let mut hit_eof = false;
        let mut last_byte = b;
        for i in 0..16 {
            let hi = match bytes.next() {
                Some(Ok(v)) => v,
                Some(Err(_)) => return Ok(true),
                None => {
                    hit_eof = true;
                    break;
                }
            };
            last_byte = hi;
            if hi == b' ' {
                break;
            }
            let lo = match bytes.next() {
                Some(Ok(v)) => v,
                Some(Err(_)) => return Ok(true),
                None => {
                    hit_eof = true;
                    break;
                }
            };
            last_byte = lo;
            let value = (hex_nibble(hi).wrapping_mul(0x10).wrapping_add(hex_nibble(lo))) as u8;
            out.write_all(&[value]).map_err(|_| ())?;
            if (i & group_by) == group_by {
                match bytes.next() {
                    Some(Ok(sep)) => last_byte = sep,
                    Some(Err(_)) => return Ok(true),
                    None => {
                        hit_eof = true;
                        break;
                    }
                }
            }
        }
        if hit_eof {
            break;
        }
        while last_byte != b'\n' {
            match bytes.next() {
                Some(Ok(v)) => last_byte = v,
                Some(Err(_)) => return Ok(true),
                None => {
                    hit_eof = true;
                    break;
                }
            }
        }
        if hit_eof {
            break;
        }
    }
    Ok(false)
}

fn process_encode<R: Read, W: Write>(
    reader: R,
    out: &mut W,
    group_by: usize,
    iso_flag: bool,
) -> Result<bool, ()> {
    let mut bytes = reader.bytes();
    let mut address: u64 = 0;
    let mut eof = false;
    while !eof {
        let mut ascii = [b' '; 17];
        let mut row_len = 0usize;
        for i in 0..16 {
            if !eof {
                match bytes.next() {
                    Some(Ok(b)) => {
                        if i == 0 {
                            write!(out, "{address:08x}: ").map_err(|_| ())?;
                        }
                        ascii[i + 1] = if b < 0x20 || (b >= 0x7f && (!iso_flag || b < 0xa0)) {
                            b'.'
                        } else {
                            b
                        };
                        write!(out, "{b:02x}").map_err(|_| ())?;
                        row_len = i + 1;
                    }
                    Some(Err(_)) => return Ok(true),
                    None => {
                        eof = true;
                        if i == 0 {
                            break;
                        }
                        out.write_all(b"  ").map_err(|_| ())?;
                    }
                }
            } else {
                out.write_all(b"  ").map_err(|_| ())?;
            }
            if (i & group_by) == group_by {
                out.write_all(b" ").map_err(|_| ())?;
            }
        }
        if row_len > 0 {
            out.write_all(&ascii[..=row_len]).map_err(|_| ())?;
            out.write_all(b"\n").map_err(|_| ())?;
        }
        address = address.wrapping_add(0x10);
    }
    Ok(false)
}

fn main() -> ExitCode {
    let mut args: Vec<OsString> = env::args_os().collect();
    let progname = args
        .first()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "hexl".to_string());
    if !args.is_empty() {
        args.remove(0);
    }

    let mut group_by: usize = 0x01;
    let mut un_flag = false;
    let mut iso_flag = false;
    let mut idx = 0usize;

    while idx < args.len() {
        let s = args[idx].to_string_lossy();
        let bytes = s.as_bytes();
        if bytes.len() < 2 || bytes[0] != b'-' {
            break;
        }
        match s.as_ref() {
            "--" => {
                idx += 1;
                break;
            }
            "-un" | "-de" => un_flag = true,
            "-hex" => {}
            "-iso" => iso_flag = true,
            "-group-by-8-bits" => group_by = 0x00,
            "-group-by-16-bits" => group_by = 0x01,
            "-group-by-32-bits" => group_by = 0x03,
            "-group-by-64-bits" => group_by = 0x07,
            other => {
                let _ = writeln!(io::stderr(), "{progname}: invalid switch: \"{other}\".");
                let _ = writeln!(io::stderr(), "usage: {progname} [-de] [-iso]");
                return ExitCode::FAILURE;
            }
        }
        idx += 1;
    }

    let files: Vec<OsString> = if idx < args.len() {
        args[idx..].to_vec()
    } else {
        vec![OsString::from("-")]
    };

    let mut status = ExitCode::SUCCESS;
    let stdout_handle = io::stdout();
    let mut out = BufWriter::new(stdout_handle.lock());

    for file_arg in files {
        let disp = file_arg.to_string_lossy();
        let read_trouble = if disp == "-" {
            let stdin_handle = io::stdin();
            let reader = BufReader::new(stdin_handle.lock());
            let res = if un_flag {
                process_decode(reader, &mut out, group_by)
            } else {
                process_encode(reader, &mut out, group_by, iso_flag)
            };
            match res {
                Ok(trouble) => trouble,
                Err(()) => return write_error(&progname),
            }
        } else {
            let file = match File::open(Path::new(&file_arg)) {
                Ok(f) => f,
                Err(err) => {
                    let _ = writeln!(io::stderr(), "{disp}: {err}");
                    status = ExitCode::FAILURE;
                    continue;
                }
            };
            let reader = BufReader::new(file);
            let res = if un_flag {
                process_decode(reader, &mut out, group_by)
            } else {
                process_encode(reader, &mut out, group_by, iso_flag)
            };
            match res {
                Ok(trouble) => trouble,
                Err(()) => return write_error(&progname),
            }
        };

        if read_trouble {
            let _ = writeln!(io::stderr(), "{progname}: read error");
            status = ExitCode::FAILURE;
        }
    }

    if out.flush().is_err() {
        return write_error(&progname);
    }
    status
}
