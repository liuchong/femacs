#![forbid(unsafe_code)]
//! Pure Safe Rust FIPS 180-4 SHA-224 / SHA-256 streaming digests (`lib/sha256.c` / `lib/sha256.h`).

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sha256Ctx {
    pub state: [u32; 8],
    pub total: [u32; 2],
    pub buflen: usize,
    pub buffer: [u8; 128],
}

const _: () = assert!(core::mem::size_of::<Sha256Ctx>() == 176);
const _: () = assert!(core::mem::align_of::<Sha256Ctx>() == core::mem::align_of::<usize>());

const K256: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
    0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
    0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
    0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
    0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
    0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

impl Default for Sha256Ctx {
    fn default() -> Self {
        Self::new_256()
    }
}

impl Sha256Ctx {
    #[inline]
    pub const fn new_256() -> Self {
        Self {
            state: [
                0x6a09_e667,
                0xbb67_ae85,
                0x3c6e_f372,
                0xa54f_f53a,
                0x510e_527f,
                0x9b05_688c,
                0x1f83_d9ab,
                0x5be0_cd19,
            ],
            total: [0, 0],
            buflen: 0,
            buffer: [0u8; 128],
        }
    }

    #[inline]
    pub const fn new_224() -> Self {
        Self {
            state: [
                0xc105_9ed8,
                0x367c_d507,
                0x3070_dd17,
                0xf70e_5939,
                0xffc0_0b31,
                0x6858_1511,
                0x64f9_8fa7,
                0xbefa_4fa4,
            ],
            total: [0, 0],
            buflen: 0,
            buffer: [0u8; 128],
        }
    }

    #[inline]
    pub fn init_256(&mut self) {
        self.state = [
            0x6a09_e667,
            0xbb67_ae85,
            0x3c6e_f372,
            0xa54f_f53a,
            0x510e_527f,
            0x9b05_688c,
            0x1f83_d9ab,
            0x5be0_cd19,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    #[inline]
    pub fn init_224(&mut self) {
        self.state = [
            0xc105_9ed8,
            0x367c_d507,
            0x3070_dd17,
            0xf70e_5939,
            0xffc0_0b31,
            0x6858_1511,
            0x64f9_8fa7,
            0xbefa_4fa4,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    #[inline]
    fn add_length(&mut self, len: usize) {
        let lolen = len as u32;
        self.total[0] = self.total[0].wrapping_add(lolen);
        let hi_inc = ((len as u64) >> 32) as u32;
        let carry = u32::from(self.total[0] < lolen);
        self.total[1] = self.total[1].wrapping_add(hi_inc).wrapping_add(carry);
    }

    pub fn process_block(&mut self, bytes: &[u8]) {
        let full_len = bytes.len() & !63;
        self.add_length(full_len);

        for chunk in bytes[..full_len].chunks_exact(64) {
            let mut w = [0u32; 64];
            for (dst, src) in w[..16].iter_mut().zip(chunk.chunks_exact(4)) {
                *dst = u32::from_be_bytes([src[0], src[1], src[2], src[3]]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }

            let mut a = self.state[0];
            let mut b = self.state[1];
            let mut c = self.state[2];
            let mut d = self.state[3];
            let mut e = self.state[4];
            let mut f = self.state[5];
            let mut g = self.state[6];
            let mut h = self.state[7];

            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = g ^ (e & (f ^ g));
                let temp1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K256[i])
                    .wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) | (c & (a | b));
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

            self.state[0] = self.state[0].wrapping_add(a);
            self.state[1] = self.state[1].wrapping_add(b);
            self.state[2] = self.state[2].wrapping_add(c);
            self.state[3] = self.state[3].wrapping_add(d);
            self.state[4] = self.state[4].wrapping_add(e);
            self.state[5] = self.state[5].wrapping_add(f);
            self.state[6] = self.state[6].wrapping_add(g);
            self.state[7] = self.state[7].wrapping_add(h);
        }
    }

    pub fn process_bytes(&mut self, mut bytes: &[u8]) {
        if self.buflen != 0 {
            let left_over = self.buflen.min(128);
            let add = (128 - left_over).min(bytes.len());
            self.buffer[left_over..left_over + add].copy_from_slice(&bytes[..add]);
            let new_len = left_over + add;
            self.buflen = new_len;

            if new_len > 64 {
                let block_len = new_len & !63;
                let mut local = [0u8; 128];
                local[..block_len].copy_from_slice(&self.buffer[..block_len]);
                self.process_block(&local[..block_len]);
                let rem = new_len & 63;
                self.buffer.copy_within(block_len..block_len + rem, 0);
                self.buflen = rem;
            }
            bytes = &bytes[add..];
        }

        if bytes.len() >= 64 {
            let full = bytes.len() & !63;
            self.process_block(&bytes[..full]);
            bytes = &bytes[full..];
        }

        if !bytes.is_empty() {
            let mut left_over = self.buflen.min(64);
            self.buffer[left_over..left_over + bytes.len()].copy_from_slice(bytes);
            left_over += bytes.len();
            if left_over >= 64 {
                let mut local = [0u8; 64];
                local.copy_from_slice(&self.buffer[..64]);
                self.process_block(&local);
                left_over -= 64;
                self.buffer.copy_within(64..64 + left_over, 0);
            }
            self.buflen = left_over;
        }
    }

    fn conclude(&mut self) {
        if self.buflen >= 64 {
            let full = self.buflen.min(128) & !63;
            let mut local = [0u8; 128];
            local[..full].copy_from_slice(&self.buffer[..full]);
            self.process_block(&local[..full]);
            let rem = self.buflen.min(128) - full;
            self.buffer.copy_within(full..full + rem, 0);
            self.buflen = rem;
        }

        let bytes = self.buflen;
        let pad_total = if bytes < 56 { 64 } else { 128 };

        let total0 = self.total[0].wrapping_add(bytes as u32);
        let total1 = self.total[1].wrapping_add(u32::from((total0 as usize) < bytes));
        self.total = [total0, total1];

        let bit_hi = (total1 << 3) | (total0 >> 29);
        let bit_lo = total0 << 3;

        self.buffer[bytes] = 0x80;
        if bytes + 1 < pad_total - 8 {
            self.buffer[bytes + 1..pad_total - 8].fill(0);
        }
        self.buffer[pad_total - 8..pad_total - 4].copy_from_slice(&bit_hi.to_be_bytes());
        self.buffer[pad_total - 4..pad_total].copy_from_slice(&bit_lo.to_be_bytes());

        let mut final_blocks = [0u8; 128];
        final_blocks[..pad_total].copy_from_slice(&self.buffer[..pad_total]);
        self.process_block(&final_blocks[..pad_total]);
    }

    #[inline]
    pub fn read_256_into(&self, out: &mut [u8; 32]) {
        for (dst, &word) in out.chunks_exact_mut(4).zip(self.state.iter()) {
            dst.copy_from_slice(&word.to_be_bytes());
        }
    }

    #[inline]
    pub fn read_224_into(&self, out: &mut [u8; 28]) {
        for (dst, &word) in out.chunks_exact_mut(4).zip(self.state[..7].iter()) {
            dst.copy_from_slice(&word.to_be_bytes());
        }
    }

    #[inline]
    pub fn finish_256_into(&mut self, out: &mut [u8; 32]) {
        self.conclude();
        self.read_256_into(out);
    }

    #[inline]
    pub fn finish_224_into(&mut self, out: &mut [u8; 28]) {
        self.conclude();
        self.read_224_into(out);
    }
}

#[inline]
pub fn sha256_digest(bytes: &[u8]) -> [u8; 32] {
    let mut ctx = Sha256Ctx::new_256();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 32];
    ctx.finish_256_into(&mut out);
    out
}

#[inline]
pub fn sha224_digest(bytes: &[u8]) -> [u8; 28] {
    let mut ctx = Sha256Ctx::new_224();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 28];
    ctx.finish_224_into(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::{sha1, sha512};

    fn hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            use core::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
        }
        s
    }

    #[test]
    fn sha1_and_sha2_fips180_vectors_and_streaming_splits() {
        assert_eq!(
            hex(&sha1::sha1_digest(b"abc")),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
        assert_eq!(
            hex(&sha224_digest(b"abc")),
            "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"
        );
        assert_eq!(
            hex(&sha256_digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(&sha512::sha384_digest(b"abc")),
            "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"
        );
        assert_eq!(
            hex(&sha512::sha512_digest(b"abc")),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );

        for len in [55usize, 56, 64, 111, 112, 128, 300] {
            let payload: Vec<u8> = (0..len).map(|i| (i.wrapping_mul(13) ^ 0x5c) as u8).collect();
            let exp1 = sha1::sha1_digest(&payload);
            let exp256 = sha256_digest(&payload);
            let exp512 = sha512::sha512_digest(&payload);

            let mut c1 = sha1::Sha1Ctx::new();
            let mut c256 = Sha256Ctx::new_256();
            let mut c512 = sha512::Sha512Ctx::new_512();
            for chunk in payload.chunks(11) {
                c1.process_bytes(chunk);
                c256.process_bytes(chunk);
                c512.process_bytes(chunk);
            }
            let mut g1 = [0u8; 20];
            let mut g256 = [0u8; 32];
            let mut g512 = [0u8; 64];
            c1.finish_into(&mut g1);
            c256.finish_256_into(&mut g256);
            c512.finish_512_into(&mut g512);
            assert_eq!(g1, exp1);
            assert_eq!(g256, exp256);
            assert_eq!(g512, exp512);
        }
    }
}
