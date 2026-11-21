#![forbid(unsafe_code)]
//! Pure Safe Rust RFC 1321 MD5 streaming digest (`lib/md5.c` / `lib/md5.h`).

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Md5Ctx {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub total: [u32; 2],
    pub buflen: u32,
    pub buffer: [u8; 128],
}

const _: () = assert!(core::mem::size_of::<Md5Ctx>() == 156);
const _: () = assert!(core::mem::align_of::<Md5Ctx>() == 4);

impl Default for Md5Ctx {
    fn default() -> Self {
        Self::new()
    }
}

impl Md5Ctx {
    #[inline]
    pub const fn new() -> Self {
        Self {
            A: 0x6745_2301,
            B: 0xefcd_ab89,
            C: 0x98ba_dcfe,
            D: 0x1032_5476,
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

        macro_rules! ff {
            ($x:expr, $y:expr, $z:expr) => {
                $z ^ ($x & ($y ^ $z))
            };
        }
        macro_rules! fg {
            ($x:expr, $y:expr, $z:expr) => {
                ff!($z, $x, $y)
            };
        }
        macro_rules! fh {
            ($x:expr, $y:expr, $z:expr) => {
                $x ^ $y ^ $z
            };
        }
        macro_rules! fi {
            ($x:expr, $y:expr, $z:expr) => {
                $y ^ ($x | !$z)
            };
        }
        macro_rules! op1 {
            ($a:ident, $b:expr, $c:expr, $d:expr, $w:expr, $s:expr, $t:expr) => {{
                $a = $a
                    .wrapping_add(ff!($b, $c, $d))
                    .wrapping_add($w)
                    .wrapping_add($t)
                    .rotate_left($s)
                    .wrapping_add($b);
            }};
        }
        macro_rules! op {
            ($f:ident, $a:ident, $b:expr, $c:expr, $d:expr, $w:expr, $s:expr, $t:expr) => {{
                $a = $a
                    .wrapping_add($f!($b, $c, $d))
                    .wrapping_add($w)
                    .wrapping_add($t)
                    .rotate_left($s)
                    .wrapping_add($b);
            }};
        }

        for chunk in bytes[..full_len].chunks_exact(64) {
            let mut w = [0u32; 16];
            for (dst, src) in w.iter_mut().zip(chunk.chunks_exact(4)) {
                *dst = u32::from_le_bytes([src[0], src[1], src[2], src[3]]);
            }

            let a_save = a;
            let b_save = b;
            let c_save = c;
            let d_save = d;

            // Round 1
            op1!(a, b, c, d, w[0], 7, 0xd76aa478);
            op1!(d, a, b, c, w[1], 12, 0xe8c7b756);
            op1!(c, d, a, b, w[2], 17, 0x242070db);
            op1!(b, c, d, a, w[3], 22, 0xc1bdceee);
            op1!(a, b, c, d, w[4], 7, 0xf57c0faf);
            op1!(d, a, b, c, w[5], 12, 0x4787c62a);
            op1!(c, d, a, b, w[6], 17, 0xa8304613);
            op1!(b, c, d, a, w[7], 22, 0xfd469501);
            op1!(a, b, c, d, w[8], 7, 0x698098d8);
            op1!(d, a, b, c, w[9], 12, 0x8b44f7af);
            op1!(c, d, a, b, w[10], 17, 0xffff5bb1);
            op1!(b, c, d, a, w[11], 22, 0x895cd7be);
            op1!(a, b, c, d, w[12], 7, 0x6b901122);
            op1!(d, a, b, c, w[13], 12, 0xfd987193);
            op1!(c, d, a, b, w[14], 17, 0xa679438e);
            op1!(b, c, d, a, w[15], 22, 0x49b40821);

            // Round 2
            op!(fg, a, b, c, d, w[1], 5, 0xf61e2562);
            op!(fg, d, a, b, c, w[6], 9, 0xc040b340);
            op!(fg, c, d, a, b, w[11], 14, 0x265e5a51);
            op!(fg, b, c, d, a, w[0], 20, 0xe9b6c7aa);
            op!(fg, a, b, c, d, w[5], 5, 0xd62f105d);
            op!(fg, d, a, b, c, w[10], 9, 0x02441453);
            op!(fg, c, d, a, b, w[15], 14, 0xd8a1e681);
            op!(fg, b, c, d, a, w[4], 20, 0xe7d3fbc8);
            op!(fg, a, b, c, d, w[9], 5, 0x21e1cde6);
            op!(fg, d, a, b, c, w[14], 9, 0xc33707d6);
            op!(fg, c, d, a, b, w[3], 14, 0xf4d50d87);
            op!(fg, b, c, d, a, w[8], 20, 0x455a14ed);
            op!(fg, a, b, c, d, w[13], 5, 0xa9e3e905);
            op!(fg, d, a, b, c, w[2], 9, 0xfcefa3f8);
            op!(fg, c, d, a, b, w[7], 14, 0x676f02d9);
            op!(fg, b, c, d, a, w[12], 20, 0x8d2a4c8a);

            // Round 3
            op!(fh, a, b, c, d, w[5], 4, 0xfffa3942);
            op!(fh, d, a, b, c, w[8], 11, 0x8771f681);
            op!(fh, c, d, a, b, w[11], 16, 0x6d9d6122);
            op!(fh, b, c, d, a, w[14], 23, 0xfde5380c);
            op!(fh, a, b, c, d, w[1], 4, 0xa4beea44);
            op!(fh, d, a, b, c, w[4], 11, 0x4bdecfa9);
            op!(fh, c, d, a, b, w[7], 16, 0xf6bb4b60);
            op!(fh, b, c, d, a, w[10], 23, 0xbebfbc70);
            op!(fh, a, b, c, d, w[13], 4, 0x289b7ec6);
            op!(fh, d, a, b, c, w[0], 11, 0xeaa127fa);
            op!(fh, c, d, a, b, w[3], 16, 0xd4ef3085);
            op!(fh, b, c, d, a, w[6], 23, 0x04881d05);
            op!(fh, a, b, c, d, w[9], 4, 0xd9d4d039);
            op!(fh, d, a, b, c, w[12], 11, 0xe6db99e5);
            op!(fh, c, d, a, b, w[15], 16, 0x1fa27cf8);
            op!(fh, b, c, d, a, w[2], 23, 0xc4ac5665);

            // Round 4
            op!(fi, a, b, c, d, w[0], 6, 0xf4292244);
            op!(fi, d, a, b, c, w[7], 10, 0x432aff97);
            op!(fi, c, d, a, b, w[14], 15, 0xab9423a7);
            op!(fi, b, c, d, a, w[5], 21, 0xfc93a039);
            op!(fi, a, b, c, d, w[12], 6, 0x655b59c3);
            op!(fi, d, a, b, c, w[3], 10, 0x8f0ccc92);
            op!(fi, c, d, a, b, w[10], 15, 0xffeff47d);
            op!(fi, b, c, d, a, w[1], 21, 0x85845dd1);
            op!(fi, a, b, c, d, w[8], 6, 0x6fa87e4f);
            op!(fi, d, a, b, c, w[15], 10, 0xfe2ce6e0);
            op!(fi, c, d, a, b, w[6], 15, 0xa3014314);
            op!(fi, b, c, d, a, w[13], 21, 0x4e0811a1);
            op!(fi, a, b, c, d, w[4], 6, 0xf7537e82);
            op!(fi, d, a, b, c, w[11], 10, 0xbd3af235);
            op!(fi, c, d, a, b, w[2], 15, 0x2ad7d2bb);
            op!(fi, b, c, d, a, w[9], 21, 0xeb86d391);

            a = a.wrapping_add(a_save);
            b = b.wrapping_add(b_save);
            c = c.wrapping_add(c_save);
            d = d.wrapping_add(d_save);
        }

        self.A = a;
        self.B = b;
        self.C = c;
        self.D = d;
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
    pub fn read_into(&self, out: &mut [u8; 16]) {
        out[0..4].copy_from_slice(&self.A.to_le_bytes());
        out[4..8].copy_from_slice(&self.B.to_le_bytes());
        out[8..12].copy_from_slice(&self.C.to_le_bytes());
        out[12..16].copy_from_slice(&self.D.to_le_bytes());
    }

    pub fn finish_into(&mut self, out: &mut [u8; 16]) {
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

        let bit_lo = total0 << 3;
        let bit_hi = (total1 << 3) | (total0 >> 29);

        self.buffer[bytes] = 0x80;
        if bytes + 1 < pad_total - 8 {
            self.buffer[bytes + 1..pad_total - 8].fill(0);
        }
        self.buffer[pad_total - 8..pad_total - 4].copy_from_slice(&bit_lo.to_le_bytes());
        self.buffer[pad_total - 4..pad_total].copy_from_slice(&bit_hi.to_le_bytes());

        let mut final_blocks = [0u8; 128];
        final_blocks[..pad_total].copy_from_slice(&self.buffer[..pad_total]);
        // Note: `self.total` already counted `bytes` above; `process_block` increments `total`
        // just like gnulib's `md5_finish_ctx` -> `md5_process_block`, which does not affect `A..D`.
        self.process_block(&final_blocks[..pad_total]);
        self.read_into(out);
    }
}

#[inline]
pub fn md5_digest(bytes: &[u8]) -> [u8; 16] {
    let mut ctx = Md5Ctx::new();
    ctx.process_bytes(bytes);
    let mut out = [0u8; 16];
    ctx.finish_into(&mut out);
    out
}
