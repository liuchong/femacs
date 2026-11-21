#![forbid(unsafe_code)]
//! Pure Safe Rust FIPS 202 SHA-3 (Keccak-f[1600]) 224/256/384/512 streaming digests (`lib/sha3.c`).

pub const SHA3_224_DIGEST_SIZE: usize = 28;
pub const SHA3_256_DIGEST_SIZE: usize = 32;
pub const SHA3_384_DIGEST_SIZE: usize = 48;
pub const SHA3_512_DIGEST_SIZE: usize = 64;

pub const SHA3_224_BLOCK_SIZE: usize = 144;
pub const SHA3_256_BLOCK_SIZE: usize = 136;
pub const SHA3_384_BLOCK_SIZE: usize = 104;
pub const SHA3_512_BLOCK_SIZE: usize = 72;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sha3Ctx {
    pub state: [u64; 25],
    pub buffer: [u8; 144],
    pub buflen: usize,
    pub digestlen: usize,
    pub blocklen: usize,
}

const RC: [u64; 24] = [
    0x00000000_00000001,
    0x00000000_00008082,
    0x80000000_0000808a,
    0x80000000_80008000,
    0x00000000_0000808b,
    0x00000000_80000001,
    0x80000000_80008081,
    0x80000000_00008009,
    0x00000000_0000008a,
    0x00000000_00000088,
    0x00000000_80008009,
    0x00000000_8000000a,
    0x00000000_8000808b,
    0x80000000_0000008b,
    0x80000000_00008089,
    0x80000000_00008003,
    0x80000000_00008002,
    0x80000000_00000080,
    0x00000000_0000800a,
    0x80000000_8000000a,
    0x80000000_80008081,
    0x80000000_00008080,
    0x00000000_80000001,
    0x80000000_80008008,
];

fn keccak_f1600(a: &mut [u64; 25]) {
    for &round_const in &RC {
        let c0 = a[0] ^ a[5] ^ a[10] ^ a[15] ^ a[20];
        let c1 = a[1] ^ a[6] ^ a[11] ^ a[16] ^ a[21];
        let c2 = a[2] ^ a[7] ^ a[12] ^ a[17] ^ a[22];
        let c3 = a[3] ^ a[8] ^ a[13] ^ a[18] ^ a[23];
        let c4 = a[4] ^ a[9] ^ a[14] ^ a[19] ^ a[24];

        let d0 = c4 ^ c1.rotate_left(1);
        let d1 = c0 ^ c2.rotate_left(1);
        let d2 = c1 ^ c3.rotate_left(1);
        let d3 = c2 ^ c4.rotate_left(1);
        let d4 = c3 ^ c0.rotate_left(1);

        a[0] ^= d0;
        a[5] ^= d0;
        a[10] ^= d0;
        a[15] ^= d0;
        a[20] ^= d0;
        a[1] ^= d1;
        a[6] ^= d1;
        a[11] ^= d1;
        a[16] ^= d1;
        a[21] ^= d1;
        a[2] ^= d2;
        a[7] ^= d2;
        a[12] ^= d2;
        a[17] ^= d2;
        a[22] ^= d2;
        a[3] ^= d3;
        a[8] ^= d3;
        a[13] ^= d3;
        a[18] ^= d3;
        a[23] ^= d3;
        a[4] ^= d4;
        a[9] ^= d4;
        a[14] ^= d4;
        a[19] ^= d4;
        a[24] ^= d4;

        // Rho and Pi
        let mut t1 = a[1];
        let mut t2 = t1.rotate_left(1);
        t1 = a[10];
        a[10] = t2;
        t2 = t1.rotate_left(3);
        t1 = a[7];
        a[7] = t2;
        t2 = t1.rotate_left(6);
        t1 = a[11];
        a[11] = t2;
        t2 = t1.rotate_left(10);
        t1 = a[17];
        a[17] = t2;
        t2 = t1.rotate_left(15);
        t1 = a[18];
        a[18] = t2;
        t2 = t1.rotate_left(21);
        t1 = a[3];
        a[3] = t2;
        t2 = t1.rotate_left(28);
        t1 = a[5];
        a[5] = t2;
        t2 = t1.rotate_left(36);
        t1 = a[16];
        a[16] = t2;
        t2 = t1.rotate_left(45);
        t1 = a[8];
        a[8] = t2;
        t2 = t1.rotate_left(55);
        t1 = a[21];
        a[21] = t2;
        t2 = t1.rotate_left(2);
        t1 = a[24];
        a[24] = t2;
        t2 = t1.rotate_left(14);
        t1 = a[4];
        a[4] = t2;
        t2 = t1.rotate_left(27);
        t1 = a[15];
        a[15] = t2;
        t2 = t1.rotate_left(41);
        t1 = a[23];
        a[23] = t2;
        t2 = t1.rotate_left(56);
        t1 = a[19];
        a[19] = t2;
        t2 = t1.rotate_left(8);
        t1 = a[13];
        a[13] = t2;
        t2 = t1.rotate_left(25);
        t1 = a[12];
        a[12] = t2;
        t2 = t1.rotate_left(43);
        t1 = a[2];
        a[2] = t2;
        t2 = t1.rotate_left(62);
        t1 = a[20];
        a[20] = t2;
        t2 = t1.rotate_left(18);
        t1 = a[14];
        a[14] = t2;
        t2 = t1.rotate_left(39);
        t1 = a[22];
        a[22] = t2;
        t2 = t1.rotate_left(61);
        t1 = a[9];
        a[9] = t2;
        t2 = t1.rotate_left(20);
        t1 = a[6];
        a[6] = t2;
        t2 = t1.rotate_left(44);
        a[1] = t2;

        // Chi
        let mut j = 0usize;
        while j < 25 {
            let s0 = a[j];
            let s1 = a[j + 1];
            a[j] ^= (!a[j + 1]) & a[j + 2];
            a[j + 1] ^= (!a[j + 2]) & a[j + 3];
            a[j + 2] ^= (!a[j + 3]) & a[j + 4];
            a[j + 3] ^= (!a[j + 4]) & s0;
            a[j + 4] ^= (!s0) & s1;
            j += 5;
        }

        // Iota
        a[0] ^= round_const;
    }
}

impl Sha3Ctx {
    #[inline]
    pub const fn new(digestlen: usize, blocklen: usize) -> Self {
        Self {
            state: [0u64; 25],
            buffer: [0u8; 144],
            buflen: 0,
            digestlen,
            blocklen,
        }
    }

    #[inline]
    fn init(&mut self, digestlen: usize, blocklen: usize) -> bool {
        self.state = [0u64; 25];
        self.buflen = 0;
        self.digestlen = digestlen;
        self.blocklen = blocklen;
        true
    }

    #[inline]
    pub fn init_224(&mut self) -> bool {
        self.init(SHA3_224_DIGEST_SIZE, SHA3_224_BLOCK_SIZE)
    }

    #[inline]
    pub fn init_256(&mut self) -> bool {
        self.init(SHA3_256_DIGEST_SIZE, SHA3_256_BLOCK_SIZE)
    }

    #[inline]
    pub fn init_384(&mut self) -> bool {
        self.init(SHA3_384_DIGEST_SIZE, SHA3_384_BLOCK_SIZE)
    }

    #[inline]
    pub fn init_512(&mut self) -> bool {
        self.init(SHA3_512_DIGEST_SIZE, SHA3_512_BLOCK_SIZE)
    }

    pub fn process_block(&mut self, bytes: &[u8]) -> bool {
        let blocklen = self.blocklen;
        if blocklen == 0 || blocklen > 144 || blocklen % 8 != 0 {
            return false;
        }
        let lanes = blocklen / 8;
        for chunk in bytes.chunks_exact(blocklen) {
            for (lane, src) in self.state[..lanes].iter_mut().zip(chunk.chunks_exact(8)) {
                *lane ^= u64::from_le_bytes([
                    src[0], src[1], src[2], src[3], src[4], src[5], src[6], src[7],
                ]);
            }
            keccak_f1600(&mut self.state);
        }
        true
    }

    pub fn process_bytes(&mut self, mut bytes: &[u8]) -> bool {
        let blocklen = self.blocklen;
        if blocklen == 0 || blocklen > 144 {
            return false;
        }

        if self.buflen > 0 {
            let buflen = self.buflen.min(blocklen);
            let left = blocklen - buflen;
            if bytes.len() < left {
                self.buffer[buflen..buflen + bytes.len()].copy_from_slice(bytes);
                self.buflen = buflen + bytes.len();
                return true;
            }
            self.buffer[buflen..blocklen].copy_from_slice(&bytes[..left]);
            bytes = &bytes[left..];
            let mut local = [0u8; 144];
            local[..blocklen].copy_from_slice(&self.buffer[..blocklen]);
            self.process_block(&local[..blocklen]);
            self.buflen = 0;
        }

        let full_len = bytes.len() - (bytes.len() % blocklen);
        if full_len > 0 {
            self.process_block(&bytes[..full_len]);
            bytes = &bytes[full_len..];
        }

        self.buffer[..bytes.len()].copy_from_slice(bytes);
        self.buflen = bytes.len();
        true
    }

    pub fn read_into(&self, out: &mut [u8]) {
        let n = self.digestlen.min(out.len()).min(200);
        let words = n / 8;
        let rem = n % 8;
        for (dst, &lane) in out[..words * 8]
            .chunks_exact_mut(8)
            .zip(self.state[..words].iter())
        {
            dst.copy_from_slice(&lane.to_le_bytes());
        }
        if rem != 0 {
            let tail = self.state[words].to_le_bytes();
            out[words * 8..n].copy_from_slice(&tail[..rem]);
        }
    }

    pub fn finish_into(&mut self, out: &mut [u8]) {
        let blocklen = self.blocklen.min(144);
        if blocklen == 0 {
            return;
        }
        let buflen = self.buflen.min(blocklen - 1);
        self.buffer[buflen] = 0x06;
        if buflen + 1 < blocklen {
            self.buffer[buflen + 1..blocklen].fill(0);
        }
        self.buffer[blocklen - 1] |= 0x80;
        let mut local = [0u8; 144];
        local[..blocklen].copy_from_slice(&self.buffer[..blocklen]);
        self.process_block(&local[..blocklen]);
        self.read_into(out);
    }
}

#[inline]
pub fn sha3_224_digest(bytes: &[u8]) -> [u8; 28] {
    let mut ctx = Sha3Ctx::new(SHA3_224_DIGEST_SIZE, SHA3_224_BLOCK_SIZE);
    ctx.process_bytes(bytes);
    let mut out = [0u8; 28];
    ctx.finish_into(&mut out);
    out
}

#[inline]
pub fn sha3_256_digest(bytes: &[u8]) -> [u8; 32] {
    let mut ctx = Sha3Ctx::new(SHA3_256_DIGEST_SIZE, SHA3_256_BLOCK_SIZE);
    ctx.process_bytes(bytes);
    let mut out = [0u8; 32];
    ctx.finish_into(&mut out);
    out
}

#[inline]
pub fn sha3_384_digest(bytes: &[u8]) -> [u8; 48] {
    let mut ctx = Sha3Ctx::new(SHA3_384_DIGEST_SIZE, SHA3_384_BLOCK_SIZE);
    ctx.process_bytes(bytes);
    let mut out = [0u8; 48];
    ctx.finish_into(&mut out);
    out
}

#[inline]
pub fn sha3_512_digest(bytes: &[u8]) -> [u8; 64] {
    let mut ctx = Sha3Ctx::new(SHA3_512_DIGEST_SIZE, SHA3_512_BLOCK_SIZE);
    ctx.process_bytes(bytes);
    let mut out = [0u8; 64];
    ctx.finish_into(&mut out);
    out
}
