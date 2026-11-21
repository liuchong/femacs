#![forbid(unsafe_code)]
//! Pure Safe Rust FIPS 180-4 SHA-384 / SHA-512 streaming digests (`lib/sha512.c` / `lib/sha512.h`).

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sha512Ctx {
    pub state: [u64; 8],
    pub total: [u64; 2],
    pub buflen: usize,
    pub buffer: [u8; 256],
}

const _: () = assert!(core::mem::size_of::<Sha512Ctx>() == 344);
const _: () = assert!(core::mem::align_of::<Sha512Ctx>() == 8);

const K512: [u64; 80] = [
    0x428a2f98_d728ae22,
    0x71374491_23ef65cd,
    0xb5c0fbcf_ec4d3b2f,
    0xe9b5dba5_8189dbbc,
    0x3956c25b_f348b538,
    0x59f111f1_b605d019,
    0x923f82a4_af194f9b,
    0xab1c5ed5_da6d8118,
    0xd807aa98_a3030242,
    0x12835b01_45706fbe,
    0x243185be_4ee4b28c,
    0x550c7dc3_d5ffb4e2,
    0x72be5d74_f27b896f,
    0x80deb1fe_3b1696b1,
    0x9bdc06a7_25c71235,
    0xc19bf174_cf692694,
    0xe49b69c1_9ef14ad2,
    0xefbe4786_384f25e3,
    0x0fc19dc6_8b8cd5b5,
    0x240ca1cc_77ac9c65,
    0x2de92c6f_592b0275,
    0x4a7484aa_6ea6e483,
    0x5cb0a9dc_bd41fbd4,
    0x76f988da_831153b5,
    0x983e5152_ee66dfab,
    0xa831c66d_2db43210,
    0xb00327c8_98fb213f,
    0xbf597fc7_beef0ee4,
    0xc6e00bf3_3da88fc2,
    0xd5a79147_930aa725,
    0x06ca6351_e003826f,
    0x14292967_0a0e6e70,
    0x27b70a85_46d22ffc,
    0x2e1b2138_5c26c926,
    0x4d2c6dfc_5ac42aed,
    0x53380d13_9d95b3df,
    0x650a7354_8baf63de,
    0x766a0abb_3c77b2a8,
    0x81c2c92e_47edaee6,
    0x92722c85_1482353b,
    0xa2bfe8a1_4cf10364,
    0xa81a664b_bc423001,
    0xc24b8b70_d0f89791,
    0xc76c51a3_0654be30,
    0xd192e819_d6ef5218,
    0xd6990624_5565a910,
    0xf40e3585_5771202a,
    0x106aa070_32bbd1b8,
    0x19a4c116_b8d2d0c8,
    0x1e376c08_5141ab53,
    0x2748774c_df8eeb99,
    0x34b0bcb5_e19b48a8,
    0x391c0cb3_c5c95a63,
    0x4ed8aa4a_e3418acb,
    0x5b9cca4f_7763e373,
    0x682e6ff3_d6b2b8a3,
    0x748f82ee_5defb2fc,
    0x78a5636f_43172f60,
    0x84c87814_a1f0ab72,
    0x8cc70208_1a6439ec,
    0x90befffa_23631e28,
    0xa4506ceb_de82bde9,
    0xbef9a3f7_b2c67915,
    0xc67178f2_e372532b,
    0xca273ece_ea26619c,
    0xd186b8c7_21c0c207,
    0xeada7dd6_cde0eb1e,
    0xf57d4f7f_ee6ed178,
    0x06f067aa_72176fba,
    0x0a637dc5_a2c898a6,
    0x113f9804_bef90dae,
    0x1b710b35_131c471b,
    0x28db77f5_23047d84,
    0x32caab7b_40c72493,
    0x3c9ebe0a_15c9bebc,
    0x431d67c4_9c100d4c,
    0x4cc5d4be_cb3e42b6,
    0x597f299c_fc657e2a,
    0x5fcb6fab_3ad6faec,
    0x6c44198c_4a475817,
];

impl Default for Sha512Ctx {
    fn default() -> Self {
        Self::new_512()
    }
}

impl Sha512Ctx {
    #[inline]
    pub const fn new_512() -> Self {
        Self {
            state: [
                0x6a09e667_f3bcc908,
                0xbb67ae85_84caa73b,
                0x3c6ef372_fe94f82b,
                0xa54ff53a_5f1d36f1,
                0x510e527f_ade682d1,
                0x9b05688c_2b3e6c1f,
                0x1f83d9ab_fb41bd6b,
                0x5be0cd19_137e2179,
            ],
            total: [0, 0],
            buflen: 0,
            buffer: [0u8; 256],
        }
    }

    #[inline]
    pub const fn new_384() -> Self {
        Self {
            state: [
                0xcbbb9d5d_c1059ed8,
                0x629a292a_367cd507,
                0x9159015a_3070dd17,
                0x152fecd8_f70e5939,
                0x67332667_ffc00b31,
                0x8eb44a87_68581511,
                0xdb0c2e0d_64f98fa7,
                0x47b5481d_befa4fa4,
            ],
            total: [0, 0],
            buflen: 0,
            buffer: [0u8; 256],
        }
    }

    #[inline]
    pub fn init_512(&mut self) {
        self.state = [
            0x6a09e667_f3bcc908,
            0xbb67ae85_84caa73b,
            0x3c6ef372_fe94f82b,
            0xa54ff53a_5f1d36f1,
            0x510e527f_ade682d1,
            0x9b05688c_2b3e6c1f,
            0x1f83d9ab_fb41bd6b,
            0x5be0cd19_137e2179,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    #[inline]
    pub fn init_384(&mut self) {
        self.state = [
            0xcbbb9d5d_c1059ed8,
            0x629a292a_367cd507,
            0x9159015a_3070dd17,
            0x152fecd8_f70e5939,
            0x67332667_ffc00b31,
            0x8eb44a87_68581511,
            0xdb0c2e0d_64f98fa7,
            0x47b5481d_befa4fa4,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    #[inline]
    fn add_length(&mut self, len: usize) {
        let lolen = len as u64;
        self.total[0] = self.total[0].wrapping_add(lolen);
        let carry = u64::from(self.total[0] < lolen);
        self.total[1] = self.total[1].wrapping_add(carry);
    }

    pub fn process_block(&mut self, bytes: &[u8]) {
        let full_len = bytes.len() & !127;
        self.add_length(full_len);

        for chunk in bytes[..full_len].chunks_exact(128) {
            let mut w = [0u64; 80];
            for (dst, src) in w[..16].iter_mut().zip(chunk.chunks_exact(8)) {
                *dst = u64::from_be_bytes([
                    src[0], src[1], src[2], src[3], src[4], src[5], src[6], src[7],
                ]);
            }
            for i in 16..80 {
                let s0 = w[i - 15].rotate_left(63) ^ w[i - 15].rotate_left(56) ^ (w[i - 15] >> 7);
                let s1 = w[i - 2].rotate_left(45) ^ w[i - 2].rotate_left(3) ^ (w[i - 2] >> 6);
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

            for i in 0..80 {
                let ss1 = e.rotate_left(50) ^ e.rotate_left(46) ^ e.rotate_left(23);
                let ch = g ^ (e & (f ^ g));
                let t1 = h
                    .wrapping_add(ss1)
                    .wrapping_add(ch)
                    .wrapping_add(K512[i])
                    .wrapping_add(w[i]);
                let ss0 = a.rotate_left(36) ^ a.rotate_left(30) ^ a.rotate_left(25);
                let maj = (a & b) | (c & (a | b));
                let t0 = ss0.wrapping_add(maj);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t0.wrapping_add(t1);
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
            let left_over = self.buflen.min(256);
            let add = (256 - left_over).min(bytes.len());
            self.buffer[left_over..left_over + add].copy_from_slice(&bytes[..add]);
            let new_len = left_over + add;
            self.buflen = new_len;

            if new_len > 128 {
                let block_len = new_len & !127;
                let mut local = [0u8; 256];
                local[..block_len].copy_from_slice(&self.buffer[..block_len]);
                self.process_block(&local[..block_len]);
                let rem = new_len & 127;
                self.buffer.copy_within(block_len..block_len + rem, 0);
                self.buflen = rem;
            }
            bytes = &bytes[add..];
        }

        if bytes.len() >= 128 {
            let full = bytes.len() & !127;
            self.process_block(&bytes[..full]);
            bytes = &bytes[full..];
        }

        if !bytes.is_empty() {
            let mut left_over = self.buflen.min(128);
            self.buffer[left_over..left_over + bytes.len()].copy_from_slice(bytes);
            left_over += bytes.len();
            if left_over >= 128 {
                let mut local = [0u8; 128];
                local.copy_from_slice(&self.buffer[..128]);
                self.process_block(&local);
                left_over -= 128;
                self.buffer.copy_within(128..128 + left_over, 0);
            }
            self.buflen = left_over;
        }
    }

    fn conclude(&mut self) {
        if self.buflen >= 128 {
            let full = self.buflen.min(256) & !127;
            let mut local = [0u8; 256];
            local[..full].copy_from_slice(&self.buffer[..full]);
            self.process_block(&local[..full]);
            let rem = self.buflen.min(256) - full;
            self.buffer.copy_within(full..full + rem, 0);
            self.buflen = rem;
        }

        let bytes = self.buflen;
        let pad_total = if bytes < 112 { 128 } else { 256 };

        let total0 = self.total[0].wrapping_add(bytes as u64);
        let total1 = self.total[1].wrapping_add(u64::from(total0 < bytes as u64));
        self.total = [total0, total1];

        let bit_hi = (total1 << 3) | (total0 >> 61);
        let bit_lo = total0 << 3;

        self.buffer[bytes] = 0x80;
        if bytes + 1 < pad_total - 16 {
            self.buffer[bytes + 1..pad_total - 16].fill(0);
        }
        self.buffer[pad_total - 16..pad_total - 8].copy_from_slice(&bit_hi.to_be_bytes());
        self.buffer[pad_total - 8..pad_total].copy_from_slice(&bit_lo.to_be_bytes());

        let mut final_blocks = [0u8; 256];
        final_blocks[..pad_total].copy_from_slice(&self.buffer[..pad_total]);
        self.process_block(&final_blocks[..pad_total]);
    }

    #[inline]
    pub fn read_512_into(&self, out: &mut [u8; 64]) {
        for (dst, &word) in out.chunks_exact_mut(8).zip(self.state.iter()) {
            dst.copy_from_slice(&word.to_be_bytes());
        }
    }

    #[inline]
    pub fn read_384_into(&self, out: &mut [u8; 48]) {
        for (dst, &word) in out.chunks_exact_mut(8).zip(self.state[..6].iter()) {
            dst.copy_from_slice(&word.to_be_bytes());
        }
    }

    #[inline]
    pub fn finish_512_into(&mut self, out: &mut [u8; 64]) {
        self.conclude();
        self.read_512_into(out);
    }

    #[inline]
    pub fn finish_384_into(&mut self, out: &mut [u8; 48]) {
        self.conclude();
        self.read_384_into(out);
    }
}

#[inline]
pub fn sha512_digest(bytes: &[u8]) -> [u8; 64] {
    let mut ctx = Sha512Ctx::new_512();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 64];
    ctx.finish_512_into(&mut out);
    out
}

#[inline]
pub fn sha384_digest(bytes: &[u8]) -> [u8; 48] {
    let mut ctx = Sha512Ctx::new_384();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 48];
    ctx.finish_384_into(&mut out);
    out
}
