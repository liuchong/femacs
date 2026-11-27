#![forbid(unsafe_code)]
//! Pure-Rust cryptography & ID generation (`std/crypto`).

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;

use crate::StdHandler;
use crate::value::{StdValue, expect_bytes, expect_str, expect_u64};

const MAX_RANDOM_BYTES: u64 = 64 * 1024 * 1024;

type HmacSha256 = Hmac<Sha256>;

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        use std::fmt::Write as _;
        let _ = write!(out, "{b:02x}");
    }
    out
}

pub fn hex_decode(raw: &str) -> Result<Vec<u8>, String> {
    let s = raw.trim();
    if s.len() % 2 != 0 {
        return Err("hex string must have an even length".to_string());
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let hi = decode_nibble(chunk[0])?;
        let lo = decode_nibble(chunk[1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn decode_nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(format!("invalid hex character '{}'", char::from(b))),
    }
}

pub fn sha256_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn sha256_hex(data: &[u8]) -> String {
    hex_encode(&sha256_bytes(data))
}

pub fn sha512_hex(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hex_encode(&hasher.finalize())
}

pub fn hmac_sha256_hex(key: &[u8], data: &[u8]) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| format!("hmac-sha256 init error: {e}"))?;
    mac.update(data);
    Ok(hex_encode(&mac.finalize().into_bytes()))
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let max_len = a.len().max(b.len());
    let mut diff = a.len() ^ b.len();
    for i in 0..max_len {
        let ba = a.get(i).copied().unwrap_or(0);
        let bb = b.get(i).copied().unwrap_or(0);
        diff |= usize::from(ba ^ bb);
    }
    diff == 0
}

pub(crate) fn register(mut reg: impl FnMut(&str, StdHandler)) {
    reg(
        "%std-crypto-random-bytes",
        std::sync::Arc::new(|args| {
            let n = expect_u64(args.first(), "length")?;
            if n > MAX_RANDOM_BYTES {
                return Err(format!("random byte length exceeds limit ({MAX_RANDOM_BYTES})"));
            }
            let mut buf = vec![0u8; n as usize];
            getrandom::getrandom(&mut buf).map_err(|e| format!("CSPRNG error: {e}"))?;
            Ok(StdValue::Bytes(buf))
        }),
    );

    reg(
        "%std-crypto-sha256",
        std::sync::Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            Ok(StdValue::String(sha256_hex(data)))
        }),
    );

    reg(
        "%std-crypto-sha256-bytes",
        std::sync::Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            Ok(StdValue::Bytes(sha256_bytes(data)))
        }),
    );

    reg(
        "%std-crypto-sha512",
        std::sync::Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            Ok(StdValue::String(sha512_hex(data)))
        }),
    );

    reg(
        "%std-crypto-hmac-sha256",
        std::sync::Arc::new(|args| {
            let key = expect_bytes(args.first(), "key")?;
            let data = expect_bytes(args.get(1), "data")?;
            Ok(StdValue::String(hmac_sha256_hex(key, data)?))
        }),
    );

    reg(
        "%std-crypto-uuid-v4",
        std::sync::Arc::new(|_| Ok(StdValue::String(Uuid::new_v4().to_string()))),
    );

    reg(
        "%std-crypto-uuid-v7",
        std::sync::Arc::new(|_| Ok(StdValue::String(Uuid::now_v7().to_string()))),
    );

    reg(
        "%std-crypto-hex-encode",
        std::sync::Arc::new(|args| {
            let data = expect_bytes(args.first(), "data")?;
            Ok(StdValue::String(hex_encode(data)))
        }),
    );

    reg(
        "%std-crypto-hex-decode",
        std::sync::Arc::new(|args| {
            let raw = expect_str(args.first(), "hex")?;
            Ok(StdValue::Bytes(hex_decode(raw)?))
        }),
    );

    reg(
        "%std-crypto-constant-time-eq",
        std::sync::Arc::new(|args| {
            let a = expect_bytes(args.first(), "a")?;
            let b = expect_bytes(args.get(1), "b")?;
            Ok(StdValue::Bool(constant_time_eq(a, b)))
        }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc4231_hmac_sha256_and_hex_roundtrip() {
        // RFC 4231 Test Case 2: key = "Jefe", data = "what do ya want for nothing?"
        let mac = hmac_sha256_hex(b"Jefe", b"what do ya want for nothing?").expect("hmac");
        assert_eq!(
            mac,
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
        let decoded = hex_decode(&mac).expect("decode hex");
        assert_eq!(hex_encode(&decoded), mac);
        assert!(hex_decode("0").is_err());
        assert!(hex_decode("zz").is_err());
    }

    #[test]
    fn constant_time_compare_handles_equal_diff_and_lengths() {
        assert!(constant_time_eq(b"secret", b"secret"));
        assert!(!constant_time_eq(b"secret", b"secrex"));
        assert!(!constant_time_eq(b"short", b"shorter"));
    }
}
