#![forbid(unsafe_code)]
//! Pure Safe Rust FIPS 180-4 SHA-1 streaming digest (`lib/sha1.c` / `lib/sha1.h`).

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sha1Ctx {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub E: u32,
    pub total: [u32; 2],
    pub buflen: u32,
    pub buffer: [u8; 128],
}

const _: () = assert!(core::mem::size_of::<Sha1Ctx>() == 160);
const _: () = assert!(core::mem::align_of::<Sha1Ctx>() == 4);

const K1: u32 = 0x5a82_7999;
const K2: u32 = 0x6ed9_eba1;
const K3: u32 = 0x8f1b_bcdc;
const K4: u32 = 0xca62_c1d6;

impl Default for Sha1Ctx {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha1Ctx {
    #[inline]
    pub const fn new() -> Self {
        Self {
            A: 0x6745_2301,
            B: 0xefcd_ab89,
            C: 0x98ba_dcfe,
            D: 0x1032_5476,
            E: 0xc3d2_e1f0,
            total: [0, 0],
            buflen: 0,
            buffer: [0u8; 128],
        }
    }

    #[inline]
    pub fn init(&mut self) {
        self.A = 0x6745_2301;
        self.B = 0xefcd_ab89;
        self.C = 0x98ba_dcfe;
        self.D = 0x1032_5476;
        self.E = 0xc3d2_e1f0;
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

        let mut a = self.A;
        let mut b = self.B;
        let mut c = self.C;
        let mut d = self.D;
        let mut e = self.E;

        for chunk in bytes[..full_len].chunks_exact(64) {
            let mut w = [0u32; 80];
            for (dst, src) in w[..16].iter_mut().zip(chunk.chunks_exact(4)) {
                *dst = u32::from_be_bytes([src[0], src[1], src[2], src[3]]);
            }
            for i in 16..80 {
                w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
            }

            let mut aa = a;
            let mut bb = b;
            let mut cc = c;
            let mut dd = d;
            let mut ee = e;

            for (i, &wi) in w.iter().enumerate() {
                let (f, k) = match i {
                    0..=19 => (dd ^ (bb & (cc ^ dd)), K1),
                    20..=39 => (bb ^ cc ^ dd, K2),
                    40..=59 => ((bb & cc) | (dd & (bb | cc)), K3),
                    _ => (bb ^ cc ^ dd, K4),
                };
                let temp = aa
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(ee)
                    .wrapping_add(k)
                    .wrapping_add(wi);
                ee = dd;
                dd = cc;
                cc = bb.rotate_left(30);
                bb = aa;
                aa = temp;
            }

            a = a.wrapping_add(aa);
            b = b.wrapping_add(bb);
            c = c.wrapping_add(cc);
            d = d.wrapping_add(dd);
            e = e.wrapping_add(ee);
        }

        self.A = a;
        self.B = b;
        self.C = c;
        self.D = d;
        self.E = e;
    }

    pub fn process_bytes(&mut self, mut bytes: &[u8]) {
        if self.buflen != 0 {
            let left_over = (self.buflen as usize).min(128);
            let add = (128 - left_over).min(bytes.len());
            self.buffer[left_over..left_over + add].copy_from_slice(&bytes[..add]);
            let new_len = left_over + add;
            self.buflen = new_len as u32;

            if new_len > 64 {
                let block_len = new_len & !63;
                let mut local = [0u8; 128];
                local[..block_len].copy_from_slice(&self.buffer[..block_len]);
                self.process_block(&local[..block_len]);
                let rem = new_len & 63;
                self.buffer.copy_within(block_len..block_len + rem, 0);
                self.buflen = rem as u32;
            }
            bytes = &bytes[add..];
        }

        if bytes.len() >= 64 {
            let full = bytes.len() & !63;
            self.process_block(&bytes[..full]);
            bytes = &bytes[full..];
        }

        if !bytes.is_empty() {
            let mut left_over = (self.buflen as usize).min(64);
            self.buffer[left_over..left_over + bytes.len()].copy_from_slice(bytes);
            left_over += bytes.len();
            if left_over >= 64 {
                let mut local = [0u8; 64];
                local.copy_from_slice(&self.buffer[..64]);
                self.process_block(&local);
                left_over -= 64;
                self.buffer.copy_within(64..64 + left_over, 0);
            }
            self.buflen = left_over as u32;
        }
    }

    #[inline]
    pub fn read_into(&self, out: &mut [u8; 20]) {
        out[0..4].copy_from_slice(&self.A.to_be_bytes());
        out[4..8].copy_from_slice(&self.B.to_be_bytes());
        out[8..12].copy_from_slice(&self.C.to_be_bytes());
        out[12..16].copy_from_slice(&self.D.to_be_bytes());
        out[16..20].copy_from_slice(&self.E.to_be_bytes());
    }

    pub fn finish_into(&mut self, out: &mut [u8; 20]) {
        if self.buflen >= 64 {
            let full = (self.buflen as usize).min(128) & !63;
            let mut local = [0u8; 128];
            local[..full].copy_from_slice(&self.buffer[..full]);
            self.process_block(&local[..full]);
            let rem = (self.buflen as usize).min(128) - full;
            self.buffer.copy_within(full..full + rem, 0);
            self.buflen = rem as u32;
        }

        let bytes = self.buflen as usize;
        let pad_total = if bytes < 56 { 64 } else { 128 };

        let total0 = self.total[0].wrapping_add(bytes as u32);
        let total1 = self.total[1].wrapping_add(u32::from(total0 < bytes as u32));
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
        self.read_into(out);
    }
}

#[inline]
pub fn sha1_digest(bytes: &[u8]) -> [u8; 20] {
    let mut ctx = Sha1Ctx::new();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 20];
    ctx.finish_into(&mut out);
    out
}
