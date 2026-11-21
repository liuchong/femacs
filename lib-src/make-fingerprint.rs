// femacs: port of lib-src/make-fingerprint.c @ fd7afced403.
#![forbid(unsafe_code)]
//! Safe Rust implementation of `lib-src/make-fingerprint`.

use std::env;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::ExitCode;

const PLACEHOLDER: [u8; 32] = [
    0xDE, 0x86, 0xBB, 0x99, 0xFF, 0xF5, 0x46, 0x9A,
    0x9E, 0x3F, 0x9F, 0x5D, 0x9A, 0xDF, 0xF0, 0x91,
    0xBD, 0xCD, 0xC1, 0xE8, 0x0C, 0x16, 0x1E, 0xAF,
    0xB8, 0x6C, 0xE2, 0x2B, 0xB1, 0x24, 0xCE, 0xB0,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

struct Sha256 {
    state: [u32; 8],
    buf: [u8; 64],
    buflen: usize,
    total_bits: u64,
}

impl Sha256 {
    fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
                0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
            ],
            buf: [0u8; 64],
            buflen: 0,
            total_bits: 0,
        }
    }

    fn compress_block(state: &mut [u32; 8], block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    fn update(&mut self, mut data: &[u8]) {
        self.total_bits = self.total_bits.wrapping_add((data.len() as u64).wrapping_mul(8));
        if self.buflen > 0 {
            let take = (64 - self.buflen).min(data.len());
            self.buf[self.buflen..self.buflen + take].copy_from_slice(&data[..take]);
            self.buflen += take;
            data = &data[take..];
            if self.buflen == 64 {
                Self::compress_block(&mut self.state, &self.buf);
                self.buflen = 0;
            }
        }
        let mut chunks = data.chunks_exact(64);
        for chunk in &mut chunks {
            let mut block = [0u8; 64];
            block.copy_from_slice(chunk);
            Self::compress_block(&mut self.state, &block);
        }
        let rem = chunks.remainder();
        if !rem.is_empty() {
            self.buf[..rem.len()].copy_from_slice(rem);
            self.buflen = rem.len();
        }
    }

    fn finish(mut self) -> [u8; 32] {
        let total_bits = self.total_bits;
        self.buf[self.buflen] = 0x80;
        self.buflen += 1;
        if self.buflen > 56 {
            self.buf[self.buflen..64].fill(0);
            Self::compress_block(&mut self.state, &self.buf);
            self.buflen = 0;
        }
        self.buf[self.buflen..56].fill(0);
        self.buf[56..64].copy_from_slice(&total_bits.to_be_bytes());
        Self::compress_block(&mut self.state, &self.buf);

        let mut out = [0u8; 32];
        for (i, word) in self.state.iter().enumerate() {
            out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }
}

fn sha256_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finish()
}

fn main() -> ExitCode {
    let args: Vec<OsString> = env::args_os().collect();
    let prog = args
        .first()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "make-fingerprint".to_string());

    let mut raw = false;
    let mut idx = 1usize;
    while idx < args.len() {
        let token = args[idx].to_string_lossy();
        if token == "--" {
            idx += 1;
            break;
        }
        let bytes = token.as_bytes();
        if bytes.len() < 2 || bytes[0] != b'-' {
            break;
        }
        for &ch in &bytes[1..] {
            match ch {
                b'r' => raw = true,
                b'h' => {
                    println!("make-fingerprint [-r] FILE: replace or compute a hash");
                    return ExitCode::SUCCESS;
                }
                _ => {
                    let _ = writeln!(io::stderr(), "{prog}: invalid option -- '{}'", char::from(ch));
                    return ExitCode::FAILURE;
                }
            }
        }
        idx += 1;
    }

    if args.len().saturating_sub(idx) != 1 {
        let _ = writeln!(io::stderr(), "{prog}: missing or extra file operand");
        return ExitCode::FAILURE;
    }

    let file_arg = &args[idx];
    let file_disp = file_arg.to_string_lossy();
    let mut file = match OpenOptions::new()
        .read(true)
        .write(!raw)
        .open(Path::new(file_arg))
    {
        Ok(f) => f,
        Err(err) => {
            let _ = writeln!(io::stderr(), "{file_disp}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let meta = match file.metadata() {
        Ok(m) => m,
        Err(err) => {
            let _ = writeln!(io::stderr(), "{file_disp}: {err}");
            return ExitCode::FAILURE;
        }
    };

    if !meta.is_file() {
        let _ = writeln!(io::stderr(), "{prog}: Error: {file_disp} is not a regular file");
        return ExitCode::FAILURE;
    }

    let Ok(expected_len) = usize::try_from(meta.len()) else {
        let _ = writeln!(io::stderr(), "{prog}: {file_disp}: file too big");
        return ExitCode::FAILURE;
    };
    if expected_len >= isize::MAX as usize {
        let _ = writeln!(io::stderr(), "{prog}: {file_disp}: file too big");
        return ExitCode::FAILURE;
    }

    let mut buf = Vec::with_capacity(expected_len);
    if file.read_to_end(&mut buf).is_err() || buf.len() != expected_len {
        let _ = writeln!(io::stderr(), "{prog}: Error: could not read {file_disp}");
        return ExitCode::FAILURE;
    }

    let digest = sha256_bytes(&buf);

    if raw {
        let mut stdout = io::stdout().lock();
        for b in digest {
            if write!(stdout, "{b:02X}").is_err() {
                return ExitCode::FAILURE;
            }
        }
        if stdout.flush().is_err() {
            return ExitCode::FAILURE;
        }
    } else {
        let mut fingered = false;
        let mut pos = 0usize;
        while pos + PLACEHOLDER.len() <= buf.len() {
            if let Some(rel) = buf[pos..]
                .windows(PLACEHOLDER.len())
                .position(|w| w == PLACEHOLDER)
            {
                let match_pos = pos + rel;
                if file.seek(SeekFrom::Start(match_pos as u64)).is_err()
                    || file.write_all(&digest).is_err()
                {
                    let _ = writeln!(io::stderr(), "{file_disp}: write failed");
                    return ExitCode::FAILURE;
                }
                fingered = true;
                pos = match_pos + 1;
            } else {
                break;
            }
        }
        if !fingered {
            let _ = writeln!(io::stderr(), "{prog}: {file_disp}: missing fingerprint");
            return ExitCode::FAILURE;
        }
        if file.flush().is_err() {
            let _ = writeln!(io::stderr(), "{file_disp}: flush failed");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
