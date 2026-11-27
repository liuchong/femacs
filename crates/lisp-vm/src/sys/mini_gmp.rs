#![forbid(unsafe_code)]
//! Pure Safe Rust arbitrary-precision integer (`mpn` / `mpz`) arithmetic matching `mini-gmp`.

pub type Limb = u64;

pub const GMP_LIMB_BITS: usize = 64;
pub const GMP_LIMB_MAX: Limb = u64::MAX;
pub const GMP_LIMB_HIGHBIT: Limb = 1u64 << 63;
pub const GMP_DBL_MANT_BITS: i32 = 53;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DivRoundMode {
    Floor,
    Ceil,
    Trunc,
}

#[derive(Clone, Copy, Debug)]
pub struct MpzRef<'a> {
    pub neg: bool,
    pub limbs: &'a [Limb],
}

impl<'a> MpzRef<'a> {
    #[inline]
    pub fn normalized(neg: bool, limbs: &'a [Limb]) -> Self {
        let len = mpn_normalized_size(limbs);
        let slice = &limbs[..len];
        Self {
            neg: neg && !slice.is_empty(),
            limbs: slice,
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    #[inline]
    pub fn sgn(&self) -> i32 {
        if self.limbs.is_empty() {
            0
        } else if self.neg {
            -1
        } else {
            1
        }
    }

    #[inline]
    pub fn to_owned_mpz(&self) -> Mpz {
        Mpz {
            neg: self.neg && !self.limbs.is_empty(),
            limbs: self.limbs.to_vec(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mpz {
    pub neg: bool,
    pub limbs: Vec<Limb>,
}

impl Mpz {
    #[inline]
    pub fn zero() -> Self {
        Self {
            neg: false,
            limbs: Vec::new(),
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    #[inline]
    pub fn as_ref(&self) -> MpzRef<'_> {
        MpzRef {
            neg: self.neg,
            limbs: &self.limbs,
        }
    }

    #[inline]
    pub fn normalize(&mut self) {
        let n = mpn_normalized_size(&self.limbs);
        self.limbs.truncate(n);
        if self.limbs.is_empty() {
            self.neg = false;
        }
    }

    #[inline]
    pub fn from_limbs(neg: bool, mut limbs: Vec<Limb>) -> Self {
        let n = mpn_normalized_size(&limbs);
        limbs.truncate(n);
        Self {
            neg: neg && !limbs.is_empty(),
            limbs,
        }
    }

    #[inline]
    pub fn from_ui(x: u64) -> Self {
        if x == 0 {
            Self::zero()
        } else {
            Self {
                neg: false,
                limbs: vec![x],
            }
        }
    }

    #[inline]
    pub fn from_si(x: i64) -> Self {
        if x >= 0 {
            Self::from_ui(x as u64)
        } else {
            Self {
                neg: true,
                limbs: vec![x.unsigned_abs()],
            }
        }
    }

    #[inline]
    pub fn is_odd(&self) -> bool {
        self.limbs.first().is_some_and(|l| (l & 1) != 0)
    }
}

#[inline]
pub fn mpn_normalized_size(xp: &[Limb]) -> usize {
    let mut n = xp.len();
    while n > 0 && xp[n - 1] == 0 {
        n -= 1;
    }
    n
}

#[inline]
pub fn mpn_copyi(d: &mut [Limb], s: &[Limb]) {
    let n = d.len().min(s.len());
    d[..n].copy_from_slice(&s[..n]);
}

#[inline]
pub fn mpn_copyd(d: &mut [Limb], s: &[Limb]) {
    let n = d.len().min(s.len());
    for i in (0..n).rev() {
        d[i] = s[i];
    }
}

#[inline]
pub fn mpn_zero(rp: &mut [Limb]) {
    rp.fill(0);
}

#[inline]
pub fn mpn_zero_p(rp: &[Limb]) -> i32 {
    i32::from(mpn_normalized_size(rp) == 0)
}

#[inline]
pub fn mpn_cmp(ap: &[Limb], bp: &[Limb]) -> i32 {
    assert_eq!(ap.len(), bp.len());
    for i in (0..ap.len()).rev() {
        if ap[i] != bp[i] {
            return if ap[i] > bp[i] { 1 } else { -1 };
        }
    }
    0
}

#[inline]
pub fn mpn_cmp4(ap: &[Limb], bp: &[Limb]) -> i32 {
    let an = mpn_normalized_size(ap);
    let bn = mpn_normalized_size(bp);
    if an != bn {
        if an < bn { -1 } else { 1 }
    } else {
        mpn_cmp(&ap[..an], &bp[..bn])
    }
}

#[inline]
pub fn mpn_add_1(rp: &mut [Limb], ap: &[Limb], mut b: Limb) -> Limb {
    assert!(!ap.is_empty());
    assert!(rp.len() >= ap.len());
    for i in 0..ap.len() {
        let (r, cy) = ap[i].overflowing_add(b);
        rp[i] = r;
        b = Limb::from(cy);
    }
    b
}

#[inline]
pub fn mpn_add_1_in_place(rp: &mut [Limb], mut b: Limb) -> Limb {
    assert!(!rp.is_empty());
    for slot in rp.iter_mut() {
        let (r, cy) = (*slot).overflowing_add(b);
        *slot = r;
        b = Limb::from(cy);
        if b == 0 {
            break;
        }
    }
    b
}

#[inline]
pub fn mpn_add_n(rp: &mut [Limb], ap: &[Limb], bp: &[Limb]) -> Limb {
    let n = ap.len();
    assert_eq!(n, bp.len());
    assert!(rp.len() >= n);
    let mut cy: Limb = 0;
    for i in 0..n {
        let sum = u128::from(ap[i]) + u128::from(bp[i]) + u128::from(cy);
        rp[i] = sum as Limb;
        cy = (sum >> 64) as Limb;
    }
    cy
}

#[inline]
pub fn mpn_add_n_in_place(rp: &mut [Limb], bp: &[Limb]) -> Limb {
    let n = bp.len();
    assert!(rp.len() >= n);
    let mut cy: Limb = 0;
    for i in 0..n {
        let sum = u128::from(rp[i]) + u128::from(bp[i]) + u128::from(cy);
        rp[i] = sum as Limb;
        cy = (sum >> 64) as Limb;
    }
    cy
}

#[inline]
pub fn mpn_add(rp: &mut [Limb], ap: &[Limb], bp: &[Limb]) -> Limb {
    let an = ap.len();
    let bn = bp.len();
    assert!(an >= bn);
    assert!(rp.len() >= an);
    let mut cy = mpn_add_n(&mut rp[..bn], &ap[..bn], bp);
    if an > bn {
        cy = mpn_add_1(&mut rp[bn..an], &ap[bn..an], cy);
    }
    cy
}

#[inline]
pub fn mpn_sub_1(rp: &mut [Limb], ap: &[Limb], mut b: Limb) -> Limb {
    assert!(!ap.is_empty());
    assert!(rp.len() >= ap.len());
    for i in 0..ap.len() {
        let (r, borrow) = ap[i].overflowing_sub(b);
        rp[i] = r;
        b = Limb::from(borrow);
    }
    b
}

#[inline]
pub fn mpn_sub_1_in_place(rp: &mut [Limb], mut b: Limb) -> Limb {
    assert!(!rp.is_empty());
    for slot in rp.iter_mut() {
        let (r, borrow) = (*slot).overflowing_sub(b);
        *slot = r;
        b = Limb::from(borrow);
        if b == 0 {
            break;
        }
    }
    b
}

#[inline]
pub fn mpn_sub_n(rp: &mut [Limb], ap: &[Limb], bp: &[Limb]) -> Limb {
    let n = ap.len();
    assert_eq!(n, bp.len());
    assert!(rp.len() >= n);
    let mut cy: Limb = 0;
    for i in 0..n {
        let a = u128::from(ap[i]);
        let b = u128::from(bp[i]) + u128::from(cy);
        rp[i] = (a.wrapping_sub(b)) as Limb;
        cy = Limb::from(a < b);
    }
    cy
}

#[inline]
pub fn mpn_sub_in_place(rp: &mut [Limb], bp: &[Limb]) -> Limb {
    assert!(rp.len() >= bp.len());
    let mut cy: Limb = 0;
    for i in 0..bp.len() {
        let a = u128::from(rp[i]);
        let b = u128::from(bp[i]) + u128::from(cy);
        rp[i] = (a.wrapping_sub(b)) as Limb;
        cy = Limb::from(a < b);
    }
    if rp.len() > bp.len() && cy != 0 {
        cy = mpn_sub_1_in_place(&mut rp[bp.len()..], cy);
    }
    cy
}

#[inline]
pub fn mpn_sub(rp: &mut [Limb], ap: &[Limb], bp: &[Limb]) -> Limb {
    let an = ap.len();
    let bn = bp.len();
    assert!(an >= bn);
    assert!(rp.len() >= an);
    let mut cy = mpn_sub_n(&mut rp[..bn], &ap[..bn], bp);
    if an > bn {
        cy = mpn_sub_1(&mut rp[bn..an], &ap[bn..an], cy);
    }
    cy
}

#[inline]
pub fn mpn_mul_1(rp: &mut [Limb], up: &[Limb], vl: Limb) -> Limb {
    assert!(!up.is_empty());
    assert!(rp.len() >= up.len());
    let mut cl: Limb = 0;
    for i in 0..up.len() {
        let prod = u128::from(up[i]) * u128::from(vl) + u128::from(cl);
        rp[i] = prod as Limb;
        cl = (prod >> 64) as Limb;
    }
    cl
}

#[inline]
pub fn mpn_mul_1_in_place(rp: &mut [Limb], vl: Limb) -> Limb {
    assert!(!rp.is_empty());
    let mut cl: Limb = 0;
    for slot in rp.iter_mut() {
        let prod = u128::from(*slot) * u128::from(vl) + u128::from(cl);
        *slot = prod as Limb;
        cl = (prod >> 64) as Limb;
    }
    cl
}

#[inline]
pub fn mpn_addmul_1(rp: &mut [Limb], up: &[Limb], vl: Limb) -> Limb {
    assert!(!up.is_empty());
    assert!(rp.len() >= up.len());
    let mut cl: Limb = 0;
    for i in 0..up.len() {
        let prod = u128::from(up[i]) * u128::from(vl) + u128::from(rp[i]) + u128::from(cl);
        rp[i] = prod as Limb;
        cl = (prod >> 64) as Limb;
    }
    cl
}

#[inline]
pub fn mpn_submul_1(rp: &mut [Limb], up: &[Limb], vl: Limb) -> Limb {
    assert!(!up.is_empty());
    assert!(rp.len() >= up.len());
    let mut cl: Limb = 0;
    for i in 0..up.len() {
        let prod = u128::from(up[i]) * u128::from(vl) + u128::from(cl);
        let prod_lo = prod as Limb;
        let mut prod_hi = (prod >> 64) as Limb;
        let rl = rp[i];
        let diff = rl.wrapping_sub(prod_lo);
        if diff > rl {
            prod_hi = prod_hi.wrapping_add(1);
        }
        rp[i] = diff;
        cl = prod_hi;
    }
    cl
}

pub fn mpn_mul(rp: &mut [Limb], up: &[Limb], vp: &[Limb]) -> Limb {
    let un = up.len();
    let vn = vp.len();
    assert!(un >= vn && vn >= 1);
    assert!(rp.len() >= un + vn);
    rp[un] = mpn_mul_1(&mut rp[..un], up, vp[0]);
    for j in 1..vn {
        rp[j + un] = mpn_addmul_1(&mut rp[j..j + un], up, vp[j]);
    }
    rp[un + vn - 1]
}

#[inline]
pub fn mpn_mul_n(rp: &mut [Limb], ap: &[Limb], bp: &[Limb]) {
    assert_eq!(ap.len(), bp.len());
    mpn_mul(rp, ap, bp);
}

#[inline]
pub fn mpn_sqr(rp: &mut [Limb], ap: &[Limb]) {
    mpn_mul(rp, ap, ap);
}

#[inline]
pub fn mpn_lshift(rp: &mut [Limb], up: &[Limb], cnt: u32) -> Limb {
    let n = up.len();
    assert!(n >= 1 && cnt >= 1 && (cnt as usize) < GMP_LIMB_BITS);
    assert!(rp.len() >= n);
    let tnc = GMP_LIMB_BITS as u32 - cnt;
    let mut low_limb = up[n - 1];
    let retval = low_limb >> tnc;
    let mut high_limb = low_limb << cnt;
    for i in (0..n - 1).rev() {
        low_limb = up[i];
        rp[i + 1] = high_limb | (low_limb >> tnc);
        high_limb = low_limb << cnt;
    }
    rp[0] = high_limb;
    retval
}

#[inline]
pub fn mpn_lshift_in_place(rp: &mut [Limb], cnt: u32) -> Limb {
    let n = rp.len();
    assert!(n >= 1 && cnt >= 1 && (cnt as usize) < GMP_LIMB_BITS);
    let tnc = GMP_LIMB_BITS as u32 - cnt;
    let mut low_limb = rp[n - 1];
    let retval = low_limb >> tnc;
    let mut high_limb = low_limb << cnt;
    for i in (0..n - 1).rev() {
        low_limb = rp[i];
        rp[i + 1] = high_limb | (low_limb >> tnc);
        high_limb = low_limb << cnt;
    }
    rp[0] = high_limb;
    retval
}

#[inline]
pub fn mpn_rshift(rp: &mut [Limb], up: &[Limb], cnt: u32) -> Limb {
    let n = up.len();
    assert!(n >= 1 && cnt >= 1 && (cnt as usize) < GMP_LIMB_BITS);
    assert!(rp.len() >= n);
    let tnc = GMP_LIMB_BITS as u32 - cnt;
    let mut high_limb = up[0];
    let retval = high_limb << tnc;
    let mut low_limb = high_limb >> cnt;
    for i in 1..n {
        high_limb = up[i];
        rp[i - 1] = low_limb | (high_limb << tnc);
        low_limb = high_limb >> cnt;
    }
    rp[n - 1] = low_limb;
    retval
}

#[inline]
pub fn mpn_rshift_in_place(rp: &mut [Limb], cnt: u32) -> Limb {
    let n = rp.len();
    assert!(n >= 1 && cnt >= 1 && (cnt as usize) < GMP_LIMB_BITS);
    let tnc = GMP_LIMB_BITS as u32 - cnt;
    let mut high_limb = rp[0];
    let retval = high_limb << tnc;
    let mut low_limb = high_limb >> cnt;
    for i in 1..n {
        high_limb = rp[i];
        rp[i - 1] = low_limb | (high_limb << tnc);
        low_limb = high_limb >> cnt;
    }
    rp[n - 1] = low_limb;
    retval
}

#[inline]
fn mpn_common_scan(mut limb: Limb, mut i: usize, up: &[Limb], ux: Limb) -> u64 {
    let un = up.len();
    while limb == 0 {
        i += 1;
        if i >= un {
            return if ux == 0 {
                u64::MAX
            } else {
                (un * GMP_LIMB_BITS) as u64
            };
        }
        limb = ux ^ up[i];
    }
    (i * GMP_LIMB_BITS) as u64 + u64::from(limb.trailing_zeros())
}

#[inline]
pub fn mpn_scan1(up: &[Limb], bit: u64) -> u64 {
    let i = (bit / GMP_LIMB_BITS as u64) as usize;
    if i >= up.len() {
        return u64::MAX;
    }
    let limb = up[i] & (GMP_LIMB_MAX << (bit % GMP_LIMB_BITS as u64));
    mpn_common_scan(limb, i, up, 0)
}

#[inline]
pub fn mpn_scan0(up: &[Limb], bit: u64) -> u64 {
    let i = (bit / GMP_LIMB_BITS as u64) as usize;
    if i >= up.len() {
        return bit;
    }
    let limb = (!up[i]) & (GMP_LIMB_MAX << (bit % GMP_LIMB_BITS as u64));
    mpn_common_scan(limb, i, up, GMP_LIMB_MAX)
}

#[inline]
pub fn mpn_com(rp: &mut [Limb], up: &[Limb]) {
    assert!(rp.len() >= up.len());
    for i in 0..up.len() {
        rp[i] = !up[i];
    }
}

#[inline]
pub fn mpn_com_in_place(rp: &mut [Limb]) {
    for slot in rp.iter_mut() {
        *slot = !*slot;
    }
}

#[inline]
pub fn mpn_neg(rp: &mut [Limb], up: &[Limb]) -> Limb {
    let n = up.len();
    assert!(rp.len() >= n);
    let mut idx = 0;
    while idx < n && up[idx] == 0 {
        rp[idx] = 0;
        idx += 1;
    }
    if idx == n {
        return 0;
    }
    rp[idx] = up[idx].wrapping_neg();
    for j in (idx + 1)..n {
        rp[j] = !up[j];
    }
    1
}

#[inline]
pub fn mpn_neg_in_place(rp: &mut [Limb]) -> Limb {
    let n = rp.len();
    let mut idx = 0;
    while idx < n && rp[idx] == 0 {
        idx += 1;
    }
    if idx == n {
        return 0;
    }
    rp[idx] = rp[idx].wrapping_neg();
    mpn_com_in_place(&mut rp[idx + 1..]);
    1
}

#[inline]
pub fn mpn_popcount(p: &[Limb]) -> u64 {
    p.iter().map(|x| u64::from(x.count_ones())).sum()
}

pub fn mpn_invert_3by2(u1: Limb, u0: Limb) -> Limb {
    const HALF: usize = 32;
    const LLIMB_MASK: Limb = (1u64 << HALF) - 1;
    let ul = (u1 & LLIMB_MASK) as u32;
    let uh = (u1 >> HALF) as u32;
    let mut qh = ((u1 ^ GMP_LIMB_MAX) / Limb::from(uh)) as u32;
    let mut r = ((!u1).wrapping_sub(Limb::from(qh).wrapping_mul(Limb::from(uh))) << HALF)
        | LLIMB_MASK;
    let p = Limb::from(qh).wrapping_mul(Limb::from(ul));
    if r < p {
        qh = qh.wrapping_sub(1);
        r = r.wrapping_add(u1);
        if r >= u1 && r < p {
            qh = qh.wrapping_sub(1);
            r = r.wrapping_add(u1);
        }
    }
    r = r.wrapping_sub(p);

    let p2 = (r >> HALF).wrapping_mul(Limb::from(qh)).wrapping_add(r);
    let mut ql = (p2 >> HALF).wrapping_add(1);
    r = (r << HALF)
        .wrapping_add(LLIMB_MASK)
        .wrapping_sub(ql.wrapping_mul(u1));
    if r >= (GMP_LIMB_MAX & (p2 << HALF)) {
        ql = ql.wrapping_sub(1);
        r = r.wrapping_add(u1);
    }
    let mut m = (Limb::from(qh) << HALF).wrapping_add(ql);
    if r >= u1 {
        m = m.wrapping_add(1);
        r = r.wrapping_sub(u1);
    }

    if u0 > 0 {
        r = (!r).wrapping_add(u0);
        if r < u0 {
            m = m.wrapping_sub(1);
            if r >= u1 {
                m = m.wrapping_sub(1);
                r = r.wrapping_sub(u1);
            }
            r = r.wrapping_sub(u1);
        }
        let prod = u128::from(u0) * u128::from(m);
        let th = (prod >> 64) as Limb;
        let tl = prod as Limb;
        r = r.wrapping_add(th);
        if r < th {
            m = m.wrapping_sub(1);
            m = m.wrapping_sub(Limb::from((r > u1) || (r == u1 && tl > u0)));
        }
    }
    m
}

#[derive(Clone, Copy, Debug, Default)]
struct DivInverse {
    shift: u32,
    d1: Limb,
    d0: Limb,
    di: Limb,
}

#[inline]
fn udiv_qrnnd_preinv(nh: Limb, nl: Limb, d: Limb, di: Limb) -> (Limb, Limb) {
    let prod = u128::from(nh) * u128::from(di);
    // Note: nh + 1 may wrap when nh == u64::MAX, matching gmp_add_ssaaaa:
    // _ql = _ql_prod + nl; _qh = _qh_prod + (nh + 1) + carry(_ql).
    let ql_prod = prod as Limb;
    let qh_prod = (prod >> 64) as Limb;
    let (ql, cy) = ql_prod.overflowing_add(nl);
    let mut qh = qh_prod
        .wrapping_add(nh.wrapping_add(1))
        .wrapping_add(Limb::from(cy));
    let mut r = nl.wrapping_sub(qh.wrapping_mul(d));
    let mask = Limb::from(r > ql).wrapping_neg();
    qh = qh.wrapping_add(mask);
    r = r.wrapping_add(mask & d);
    if r >= d {
        r = r.wrapping_sub(d);
        qh = qh.wrapping_add(1);
    }
    (qh, r)
}

#[inline]
fn add_ssaaaa(ah: Limb, al: Limb, bh: Limb, bl: Limb) -> (Limb, Limb) {
    let (sl, cy) = al.overflowing_add(bl);
    let sh = ah.wrapping_add(bh).wrapping_add(Limb::from(cy));
    (sh, sl)
}

#[inline]
fn sub_ddmmss(ah: Limb, al: Limb, bh: Limb, bl: Limb) -> (Limb, Limb) {
    let (sl, borrow) = al.overflowing_sub(bl);
    let sh = ah.wrapping_sub(bh).wrapping_sub(Limb::from(borrow));
    (sh, sl)
}

#[inline]
fn udiv_qr_3by2(n2: Limb, n1: Limb, n0: Limb, d1: Limb, d0: Limb, dinv: Limb) -> (Limb, Limb, Limb) {
    let prod = u128::from(n2) * u128::from(dinv);
    let (mut q, q0) = add_ssaaaa((prod >> 64) as Limb, prod as Limb, n2, n1);

    let mut r1 = n1.wrapping_sub(d1.wrapping_mul(q));
    let mut r0;
    (r1, r0) = sub_ddmmss(r1, n0, d1, d0);
    let t = u128::from(d0) * u128::from(q);
    (r1, r0) = sub_ddmmss(r1, r0, (t >> 64) as Limb, t as Limb);
    q = q.wrapping_add(1);

    let mask = Limb::from(r1 >= q0).wrapping_neg();
    q = q.wrapping_add(mask);
    (r1, r0) = add_ssaaaa(r1, r0, mask & d1, mask & d0);
    if r1 >= d1 && (r1 > d1 || r0 >= d0) {
        q = q.wrapping_add(1);
        (r1, r0) = sub_ddmmss(r1, r0, d1, d0);
    }
    (q, r1, r0)
}

fn div_qr_1_invert(d: Limb) -> DivInverse {
    assert!(d > 0);
    let shift = d.leading_zeros();
    let d1 = d << shift;
    DivInverse {
        shift,
        d1,
        d0: 0,
        di: mpn_invert_3by2(d1, 0),
    }
}

fn div_qr_2_invert(mut d1: Limb, mut d0: Limb) -> DivInverse {
    assert!(d1 > 0);
    let shift = d1.leading_zeros();
    if shift > 0 {
        d1 = (d1 << shift) | (d0 >> (GMP_LIMB_BITS as u32 - shift));
        d0 <<= shift;
    }
    DivInverse {
        shift,
        d1,
        d0,
        di: mpn_invert_3by2(d1, d0),
    }
}

fn div_qr_invert(dp: &[Limb]) -> DivInverse {
    let dn = dp.len();
    assert!(dn > 0);
    if dn == 1 {
        div_qr_1_invert(dp[0])
    } else if dn == 2 {
        div_qr_2_invert(dp[1], dp[0])
    } else {
        let mut d1 = dp[dn - 1];
        let mut d0 = dp[dn - 2];
        assert!(d1 > 0);
        let shift = d1.leading_zeros();
        if shift > 0 {
            d1 = (d1 << shift) | (d0 >> (GMP_LIMB_BITS as u32 - shift));
            d0 = (d0 << shift) | (dp[dn - 3] >> (GMP_LIMB_BITS as u32 - shift));
        }
        DivInverse {
            shift,
            d1,
            d0,
            di: mpn_invert_3by2(d1, d0),
        }
    }
}

fn div_qr_1_preinv(mut qp: Option<&mut [Limb]>, np: &[Limb], inv: &DivInverse) -> Limb {
    let nn = np.len();
    let mut r: Limb;
    let mut shifted_buf: Vec<Limb>;
    let src: &[Limb] = if inv.shift > 0 {
        shifted_buf = vec![0; nn];
        r = mpn_lshift(&mut shifted_buf, np, inv.shift);
        &shifted_buf
    } else {
        r = 0;
        np
    };

    for i in (0..nn).rev() {
        let (q, rem) = udiv_qrnnd_preinv(r, src[i], inv.d1, inv.di);
        r = rem;
        if let Some(qslice) = qp.as_deref_mut() {
            qslice[i] = q;
        }
    }
    r >> inv.shift
}

fn div_qr_1_preinv_in_place(np: &mut [Limb], inv: &DivInverse) -> Limb {
    let nn = np.len();
    let mut r = if inv.shift > 0 {
        mpn_lshift_in_place(np, inv.shift)
    } else {
        0
    };
    for i in (0..nn).rev() {
        let (q, rem) = udiv_qrnnd_preinv(r, np[i], inv.d1, inv.di);
        r = rem;
        np[i] = q;
    }
    r >> inv.shift
}

fn div_qr_2_preinv(mut qp: Option<&mut [Limb]>, np: &mut [Limb], inv: &DivInverse) {
    let nn = np.len();
    assert!(nn >= 2);
    let mut r1 = if inv.shift > 0 {
        mpn_lshift_in_place(np, inv.shift)
    } else {
        0
    };
    let mut r0 = np[nn - 1];
    for i in (0..=nn - 2).rev() {
        let n0 = np[i];
        let (q, nr1, nr0) = udiv_qr_3by2(r1, r0, n0, inv.d1, inv.d0, inv.di);
        r1 = nr1;
        r0 = nr0;
        if let Some(qslice) = qp.as_deref_mut() {
            qslice[i] = q;
        }
    }
    if inv.shift > 0 {
        r0 = (r0 >> inv.shift) | (r1 << (GMP_LIMB_BITS as u32 - inv.shift));
        r1 >>= inv.shift;
    }
    np[1] = r1;
    np[0] = r0;
}

fn div_qr_pi1(
    mut qp: Option<&mut [Limb]>,
    np: &mut [Limb],
    mut n1: Limb,
    dp: &[Limb],
    dinv: Limb,
) {
    let nn = np.len();
    let dn = dp.len();
    assert!(dn > 2 && nn >= dn);
    let d1 = dp[dn - 1];
    let d0 = dp[dn - 2];
    assert!((d1 & GMP_LIMB_HIGHBIT) != 0);

    for i in (0..=nn - dn).rev() {
        let mut n0 = np[dn - 1 + i];
        let mut q: Limb;
        if n1 == d1 && n0 == d0 {
            q = GMP_LIMB_MAX;
            mpn_submul_1(&mut np[i..i + dn], dp, q);
            n1 = np[dn - 1 + i];
        } else {
            let (qq, nn1, nn0) = udiv_qr_3by2(n1, n0, np[dn - 2 + i], d1, d0, dinv);
            q = qq;
            n1 = nn1;
            n0 = nn0;
            let cy = mpn_submul_1(&mut np[i..i + dn - 2], &dp[..dn - 2], q);
            let cy1 = Limb::from(n0 < cy);
            n0 = n0.wrapping_sub(cy);
            let cy_top = Limb::from(n1 < cy1);
            n1 = n1.wrapping_sub(cy1);
            np[dn - 2 + i] = n0;
            if cy_top != 0 {
                let add_cy = mpn_add_n_in_place(&mut np[i..i + dn - 1], &dp[..dn - 1]);
                n1 = n1.wrapping_add(d1.wrapping_add(add_cy));
                q = q.wrapping_sub(1);
            }
        }
        if let Some(qslice) = qp.as_deref_mut() {
            qslice[i] = q;
        }
    }
    np[dn - 1] = n1;
}

fn div_qr_preinv(qp: Option<&mut [Limb]>, np: &mut [Limb], dp: &[Limb], inv: &DivInverse) {
    let nn = np.len();
    let dn = dp.len();
    assert!(dn > 0 && nn >= dn);
    if dn == 1 {
        np[0] = div_qr_1_preinv(qp, np, inv);
    } else if dn == 2 {
        div_qr_2_preinv(qp, np, inv);
    } else {
        let nh = if inv.shift > 0 {
            mpn_lshift_in_place(np, inv.shift)
        } else {
            0
        };
        div_qr_pi1(qp, np, nh, dp, inv.di);
        if inv.shift > 0 {
            let cy = mpn_rshift_in_place(&mut np[..dn], inv.shift);
            assert_eq!(cy, 0);
        }
    }
}

pub fn mpn_div_qr(qp: Option<&mut [Limb]>, np: &mut [Limb], dp: &[Limb]) {
    let nn = np.len();
    let dn = dp.len();
    assert!(dn > 0 && nn >= dn);
    let inv = div_qr_invert(dp);
    if dn > 2 && inv.shift > 0 {
        let mut shifted_d = vec![0; dn];
        let cy = mpn_lshift(&mut shifted_d, dp, inv.shift);
        assert_eq!(cy, 0);
        div_qr_preinv(qp, np, &shifted_d, &inv);
    } else {
        div_qr_preinv(qp, np, dp, &inv);
    }
}

#[inline]
fn base_power_of_two_p(b: u32) -> u32 {
    match b {
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        128 => 7,
        256 => 8,
        _ => 0,
    }
}

#[derive(Clone, Copy)]
struct BaseInfo {
    exp: u32,
    bb: Limb,
}

#[inline]
fn get_base_info(b: Limb) -> BaseInfo {
    let m = GMP_LIMB_MAX / b;
    let mut exp = 1u32;
    let mut p = b;
    while p <= m {
        p = p.wrapping_mul(b);
        exp += 1;
    }
    BaseInfo { exp, bb: p }
}

#[inline]
fn limb_size_in_base_2(u: Limb) -> usize {
    assert!(u > 0);
    GMP_LIMB_BITS - u.leading_zeros() as usize
}

fn get_str_bits(bits: u32, up: &[Limb]) -> Vec<u8> {
    let un = up.len();
    let sn = ((un - 1) * GMP_LIMB_BITS + limb_size_in_base_2(up[un - 1]) + bits as usize - 1)
        / bits as usize;
    let mask = ((1u32 << bits) - 1) as u8;
    let mut out = vec![0u8; sn];
    let mut i = 0usize;
    let mut j = sn;
    let mut shift = 0u32;
    while j > 0 {
        j -= 1;
        let mut digit = (up[i] >> shift) as u8;
        shift += bits;
        if shift as usize >= GMP_LIMB_BITS {
            i += 1;
            if i < un {
                shift -= GMP_LIMB_BITS as u32;
                digit |= (up[i] << (bits - shift)) as u8;
            }
        }
        out[j] = digit & mask;
    }
    out
}

fn limb_get_str(out: &mut Vec<u8>, mut w: Limb, binv: &DivInverse) -> usize {
    let start = out.len();
    while w > 0 {
        let h = if binv.shift == 0 {
            0
        } else {
            w >> (GMP_LIMB_BITS as u32 - binv.shift)
        };
        let l = w << binv.shift;
        let (nw, mut r) = udiv_qrnnd_preinv(h, l, binv.d1, binv.di);
        w = nw;
        r >>= binv.shift;
        out.push(r as u8);
    }
    out.len() - start
}

fn get_str_other(base: i32, info: &BaseInfo, up_in: &[Limb]) -> Vec<u8> {
    let mut up = up_in.to_vec();
    let mut un = up.len();
    let binv = div_qr_1_invert(base as Limb);
    let mut out = Vec::new();
    if un > 1 {
        let bbinv = div_qr_1_invert(info.bb);
        loop {
            let w = div_qr_1_preinv_in_place(&mut up[..un], &bbinv);
            if up[un - 1] == 0 {
                un -= 1;
            }
            let mut done = limb_get_str(&mut out, w, &binv);
            while done < info.exp as usize {
                out.push(0);
                done += 1;
            }
            if un <= 1 {
                break;
            }
        }
    }
    limb_get_str(&mut out, up[0], &binv);
    out.reverse();
    out
}

pub fn mpn_get_str(base: i32, up: &[Limb]) -> Vec<u8> {
    assert!(!up.is_empty() && *up.last().unwrap() > 0);
    let bits = base_power_of_two_p(base as u32);
    if bits != 0 {
        get_str_bits(bits, up)
    } else {
        let info = get_base_info(base as Limb);
        get_str_other(base, &info, up)
    }
}

fn set_str_bits(sp: &[u8], bits: u32) -> Vec<Limb> {
    let mut sn = sp.len();
    let mut rp = Vec::with_capacity((sn * bits as usize + GMP_LIMB_BITS - 1) / GMP_LIMB_BITS);
    let mut limb: Limb = 0;
    let mut shift = 0u32;
    while sn > 0 {
        sn -= 1;
        limb |= Limb::from(sp[sn]) << shift;
        shift += bits;
        if shift as usize >= GMP_LIMB_BITS {
            shift -= GMP_LIMB_BITS as u32;
            rp.push(limb);
            limb = Limb::from(u32::from(sp[sn]) >> (bits - shift));
        }
    }
    if limb != 0 {
        rp.push(limb);
    } else {
        let n = mpn_normalized_size(&rp);
        rp.truncate(n);
    }
    rp
}

fn set_str_other(sp: &[u8], b: Limb, info: &BaseInfo) -> Vec<Limb> {
    let sn = sp.len();
    assert!(sn > 0);
    let mut k = (1 + (sn - 1) % info.exp as usize) as u32;
    let mut j = 0usize;
    let mut w = Limb::from(sp[j]);
    j += 1;
    while {
        k -= 1;
        k != 0
    } {
        w = w.wrapping_mul(b).wrapping_add(Limb::from(sp[j]));
        j += 1;
    }
    let mut rp = vec![w];
    while j < sn {
        w = Limb::from(sp[j]);
        j += 1;
        for _ in 1..info.exp {
            w = w.wrapping_mul(b).wrapping_add(Limb::from(sp[j]));
            j += 1;
        }
        let mut cy = mpn_mul_1_in_place(&mut rp, info.bb);
        cy = cy.wrapping_add(mpn_add_1_in_place(&mut rp, w));
        if cy > 0 {
            rp.push(cy);
        }
    }
    assert_eq!(j, sn);
    rp
}

pub fn mpn_set_str(sp: &[u8], base: i32) -> Vec<Limb> {
    if sp.is_empty() {
        return Vec::new();
    }
    let bits = base_power_of_two_p(base as u32);
    if bits != 0 {
        set_str_bits(sp, bits)
    } else {
        let info = get_base_info(base as Limb);
        set_str_other(sp, base as Limb, &info)
    }
}

#[inline]
fn gcd_11(mut u: Limb, mut v: Limb) -> Limb {
    assert!((u | v) > 0);
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    if (u & 1) == 0 {
        core::mem::swap(&mut u, &mut v);
    }
    v >>= v.trailing_zeros();
    while u != v {
        if u > v {
            u -= v;
            u >>= u.trailing_zeros();
        } else {
            v -= u;
            v >>= v.trailing_zeros();
        }
    }
    u << shift
}

pub fn mpn_gcd(up_in: &[Limb], vp_in: &[Limb]) -> Vec<Limb> {
    let mut u = up_in.to_vec();
    let mut v = vp_in.to_vec();
    let mut un = u.len();
    let mut vn = v.len();
    assert!(un >= vn && vn > 0);
    assert!(v[vn - 1] > 0 && ((u[0] | v[0]) & 1) != 0);

    if un > vn {
        mpn_div_qr(None, &mut u[..un], &v[..vn]);
    }
    un = mpn_normalized_size(&u[..vn]);
    u.truncate(un);
    if un == 0 {
        return v;
    }
    if (v[0] & 1) == 0 {
        core::mem::swap(&mut u, &mut v);
        core::mem::swap(&mut un, &mut vn);
    }
    while un > 1 || vn > 1 {
        let mut zero_limbs = 0;
        while zero_limbs < un && u[zero_limbs] == 0 {
            zero_limbs += 1;
        }
        if zero_limbs > 0 {
            u.drain(..zero_limbs);
            un -= zero_limbs;
        }
        let shift = u[0].trailing_zeros();
        if shift > 0 {
            let cy = mpn_rshift_in_place(&mut u[..un], shift);
            assert_eq!(cy, 0);
            if u[un - 1] == 0 {
                un -= 1;
                u.truncate(un);
            }
        }
        if un < vn {
            core::mem::swap(&mut u, &mut v);
            core::mem::swap(&mut un, &mut vn);
        } else if un == vn {
            let c = mpn_cmp(&u[..un], &v[..vn]);
            if c == 0 {
                u.truncate(un);
                return u;
            } else if c < 0 {
                core::mem::swap(&mut u, &mut v);
            }
        }
        let cy = mpn_sub_in_place(&mut u[..un], &v[..vn]);
        assert_eq!(cy, 0);
        un = mpn_normalized_size(&u[..un]);
        u.truncate(un);
    }
    vec![gcd_11(u[0], v[0])]
}

pub fn mpn_sqrtrem(p: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
    assert!(!p.is_empty() && *p.last().unwrap() != 0);
    let u = MpzRef {
        neg: false,
        limbs: p,
    };
    let (s, r) = mpz_rootrem(u, 2);
    assert_eq!(s.limbs.len(), (p.len() + 1) / 2);
    (s.limbs, r.limbs)
}

pub fn mpn_perfect_square_p(p: &[Limb]) -> i32 {
    assert!(!p.is_empty() && *p.last().unwrap() != 0);
    let u = MpzRef {
        neg: false,
        limbs: p,
    };
    mpz_perfect_square_p(u)
}

// ---- MPZ conversions & comparisons ----

#[inline]
pub fn mpz_get_ui(u: MpzRef<'_>) -> u64 {
    u.limbs.first().copied().unwrap_or(0)
}

#[inline]
pub fn mpz_get_si(u: MpzRef<'_>) -> i64 {
    let r = mpz_get_ui(u);
    let c = 1u64; // (-i64::MAX - i64::MIN) as u64 == 1
    if u.neg {
        (-1i64).wrapping_sub(((r.wrapping_sub(c)) & (i64::MAX as u64)) as i64)
    } else {
        (r & (i64::MAX as u64)) as i64
    }
}

#[inline]
pub fn mpz_size(u: MpzRef<'_>) -> usize {
    u.limbs.len()
}

#[inline]
pub fn mpz_getlimbn(u: MpzRef<'_>, n: isize) -> Limb {
    if n >= 0 && (n as usize) < u.limbs.len() {
        u.limbs[n as usize]
    } else {
        0
    }
}

#[inline]
pub fn mpz_cmpabs_ui(u: MpzRef<'_>, v: u64) -> i32 {
    if u.limbs.len() > 1 {
        1
    } else {
        let uu = mpz_get_ui(u);
        (uu > v) as i32 - (uu < v) as i32
    }
}

#[inline]
pub fn mpz_cmp_ui(u: MpzRef<'_>, v: u64) -> i32 {
    if u.neg {
        -1
    } else {
        mpz_cmpabs_ui(u, v)
    }
}

#[inline]
pub fn mpz_cmp_si(u: MpzRef<'_>, v: i64) -> i32 {
    if v >= 0 {
        mpz_cmp_ui(u, v as u64)
    } else if !u.neg {
        1
    } else {
        -mpz_cmpabs_ui(u, v.unsigned_abs())
    }
}

#[inline]
pub fn mpz_cmpabs(u: MpzRef<'_>, v: MpzRef<'_>) -> i32 {
    mpn_cmp4(u.limbs, v.limbs)
}

#[inline]
pub fn mpz_cmp(a: MpzRef<'_>, b: MpzRef<'_>) -> i32 {
    let asize = if a.neg {
        -(a.limbs.len() as isize)
    } else {
        a.limbs.len() as isize
    };
    let bsize = if b.neg {
        -(b.limbs.len() as isize)
    } else {
        b.limbs.len() as isize
    };
    if asize != bsize {
        if asize < bsize { -1 } else { 1 }
    } else if asize >= 0 {
        mpn_cmp(a.limbs, b.limbs)
    } else {
        mpn_cmp(b.limbs, a.limbs)
    }
}

#[inline]
pub fn mpz_fits_ulong_p(u: MpzRef<'_>) -> i32 {
    i32::from(!u.neg && u.limbs.len() <= 1)
}

#[inline]
pub fn mpz_fits_slong_p(u: MpzRef<'_>) -> i32 {
    i32::from(mpz_cmp_si(u, i64::MAX) <= 0 && mpz_cmp_si(u, i64::MIN) >= 0)
}

#[inline]
pub fn mpz_fits_sint_p(u: MpzRef<'_>) -> i32 {
    i32::from(mpz_cmp_si(u, i64::from(i32::MAX)) <= 0 && mpz_cmp_si(u, i64::from(i32::MIN)) >= 0)
}

#[inline]
pub fn mpz_fits_uint_p(u: MpzRef<'_>) -> i32 {
    i32::from(!u.neg && mpz_cmpabs_ui(u, u64::from(u32::MAX)) <= 0)
}

#[inline]
pub fn mpz_fits_sshort_p(u: MpzRef<'_>) -> i32 {
    i32::from(mpz_cmp_si(u, i64::from(i16::MAX)) <= 0 && mpz_cmp_si(u, i64::from(i16::MIN)) >= 0)
}

#[inline]
pub fn mpz_fits_ushort_p(u: MpzRef<'_>) -> i32 {
    i32::from(!u.neg && mpz_cmpabs_ui(u, u64::from(u16::MAX)) <= 0)
}

pub fn mpz_from_d(mut x: f64) -> Mpz {
    if x != x || x == x * 0.5 {
        return Mpz::zero();
    }
    let neg = x < 0.0;
    if neg {
        x = -x;
    }
    if x < 1.0 {
        return Mpz::zero();
    }
    let b = 4.0 * ((GMP_LIMB_HIGHBIT >> 1) as f64);
    let bi = 1.0 / b;
    let mut rn = 1usize;
    while x >= b {
        x *= bi;
        rn += 1;
    }
    let mut limbs = vec![0u64; rn];
    let mut f = x as Limb;
    x -= f as f64;
    limbs[rn - 1] = f;
    for i in (0..rn - 1).rev() {
        x *= b;
        f = x as Limb;
        x -= f as f64;
        limbs[i] = f;
    }
    Mpz::from_limbs(neg, limbs)
}

pub fn mpz_get_d(u: MpzRef<'_>) -> f64 {
    let mut un = u.limbs.len();
    if un == 0 {
        return 0.0;
    }
    let b = 4.0 * ((GMP_LIMB_HIGHBIT >> 1) as f64);
    un -= 1;
    let mut l = u.limbs[un];
    let clz = l.leading_zeros() as i32;
    let mut m = clz + GMP_DBL_MANT_BITS - GMP_LIMB_BITS as i32;
    if m < 0 {
        l &= GMP_LIMB_MAX << (-m as usize);
    }
    let mut x = l as f64;
    while un > 0 {
        un -= 1;
        x *= b;
        if m > 0 {
            l = u.limbs[un];
            m -= GMP_LIMB_BITS as i32;
            if m < 0 {
                l &= GMP_LIMB_MAX << (-m as usize);
            }
            x += l as f64;
        }
    }
    if u.neg { -x } else { x }
}

pub fn mpz_cmpabs_d(x: MpzRef<'_>, mut d: f64) -> i32 {
    let xn = x.limbs.len();
    if d < 0.0 {
        d = -d;
    }
    if xn != 0 {
        let b = 4.0 * ((GMP_LIMB_HIGHBIT >> 1) as f64);
        let bi = 1.0 / b;
        for _ in 1..xn {
            d *= bi;
        }
        if d >= b {
            return -1;
        }
        for i in (0..xn).rev() {
            let f = d as Limb;
            let xl = x.limbs[i];
            if xl > f {
                return 1;
            } else if xl < f {
                return -1;
            }
            d = b * (d - f as f64);
        }
    }
    -i32::from(d > 0.0)
}

pub fn mpz_cmp_d(x: MpzRef<'_>, d: f64) -> i32 {
    if x.neg {
        if d >= 0.0 {
            -1
        } else {
            -mpz_cmpabs_d(x, d)
        }
    } else if d < 0.0 {
        1
    } else {
        mpz_cmpabs_d(x, d)
    }
}

// ---- MPZ arithmetic ----

#[inline]
pub fn mpz_abs(u: MpzRef<'_>) -> Mpz {
    Mpz {
        neg: false,
        limbs: u.limbs.to_vec(),
    }
}

#[inline]
pub fn mpz_neg(u: MpzRef<'_>) -> Mpz {
    Mpz {
        neg: !u.limbs.is_empty() && !u.neg,
        limbs: u.limbs.to_vec(),
    }
}

fn abs_add_slices(a: &[Limb], b: &[Limb]) -> Vec<Limb> {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    if long.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u64; long.len() + 1];
    let cy = if short.is_empty() {
        out[..long.len()].copy_from_slice(long);
        0
    } else {
        mpn_add(&mut out[..long.len()], long, short)
    };
    if cy != 0 {
        out[long.len()] = cy;
    } else {
        out.pop();
    }
    out
}

fn abs_sub_slices(a: &[Limb], b: &[Limb]) -> (bool, Vec<Limb>) {
    let cmp = mpn_cmp4(a, b);
    if cmp > 0 {
        let mut out = vec![0u64; a.len()];
        let cy = if b.is_empty() {
            out.copy_from_slice(a);
            0
        } else {
            mpn_sub(&mut out, a, b)
        };
        assert_eq!(cy, 0);
        let n = mpn_normalized_size(&out);
        out.truncate(n);
        (false, out)
    } else if cmp < 0 {
        let mut out = vec![0u64; b.len()];
        let cy = if a.is_empty() {
            out.copy_from_slice(b);
            0
        } else {
            mpn_sub(&mut out, b, a)
        };
        assert_eq!(cy, 0);
        let n = mpn_normalized_size(&out);
        out.truncate(n);
        (true, out)
    } else {
        (false, Vec::new())
    }
}

pub fn mpz_add(a: MpzRef<'_>, b: MpzRef<'_>) -> Mpz {
    if a.neg == b.neg {
        let limbs = abs_add_slices(a.limbs, b.limbs);
        Mpz::from_limbs(a.neg, limbs)
    } else {
        let (sub_neg, limbs) = abs_sub_slices(a.limbs, b.limbs);
        Mpz::from_limbs(a.neg ^ sub_neg, limbs)
    }
}

pub fn mpz_sub(a: MpzRef<'_>, b: MpzRef<'_>) -> Mpz {
    if a.neg == b.neg {
        let (sub_neg, limbs) = abs_sub_slices(a.limbs, b.limbs);
        Mpz::from_limbs(a.neg ^ sub_neg, limbs)
    } else {
        let limbs = abs_add_slices(a.limbs, b.limbs);
        Mpz::from_limbs(a.neg, limbs)
    }
}

#[inline]
pub fn mpz_add_ui(a: MpzRef<'_>, b: u64) -> Mpz {
    let bb = Mpz::from_ui(b);
    mpz_add(a, bb.as_ref())
}

#[inline]
pub fn mpz_sub_ui(a: MpzRef<'_>, b: u64) -> Mpz {
    let bb = Mpz::from_ui(b);
    mpz_sub(a, bb.as_ref())
}

#[inline]
pub fn mpz_ui_sub(a: u64, b: MpzRef<'_>) -> Mpz {
    let aa = Mpz::from_ui(a);
    mpz_sub(aa.as_ref(), b)
}

pub fn mpz_mul(u: MpzRef<'_>, v: MpzRef<'_>) -> Mpz {
    if u.is_zero() || v.is_zero() {
        return Mpz::zero();
    }
    let mut out = vec![0u64; u.limbs.len() + v.limbs.len()];
    if u.limbs.len() >= v.limbs.len() {
        mpn_mul(&mut out, u.limbs, v.limbs);
    } else {
        mpn_mul(&mut out, v.limbs, u.limbs);
    }
    Mpz::from_limbs(u.neg ^ v.neg, out)
}

#[inline]
pub fn mpz_mul_ui(u: MpzRef<'_>, v: u64) -> Mpz {
    let vv = Mpz::from_ui(v);
    mpz_mul(u, vv.as_ref())
}

#[inline]
pub fn mpz_mul_si(u: MpzRef<'_>, v: i64) -> Mpz {
    let vv = Mpz::from_si(v);
    mpz_mul(u, vv.as_ref())
}

pub fn mpz_mul_2exp(u: MpzRef<'_>, bits: u64) -> Mpz {
    if u.is_zero() {
        return Mpz::zero();
    }
    let limb_shift = (bits / GMP_LIMB_BITS as u64) as usize;
    let bit_shift = (bits % GMP_LIMB_BITS as u64) as u32;
    let un = u.limbs.len();
    let rn = un + limb_shift + usize::from(bit_shift > 0);
    let mut out = vec![0u64; rn];
    if bit_shift > 0 {
        let cy = mpn_lshift(&mut out[limb_shift..limb_shift + un], u.limbs, bit_shift);
        out[rn - 1] = cy;
    } else {
        out[limb_shift..limb_shift + un].copy_from_slice(u.limbs);
    }
    Mpz::from_limbs(u.neg, out)
}

#[inline]
pub fn mpz_addmul(r: MpzRef<'_>, u: MpzRef<'_>, v: MpzRef<'_>) -> Mpz {
    let prod = mpz_mul(u, v);
    mpz_add(r, prod.as_ref())
}

#[inline]
pub fn mpz_addmul_ui(r: MpzRef<'_>, u: MpzRef<'_>, v: u64) -> Mpz {
    let prod = mpz_mul_ui(u, v);
    mpz_add(r, prod.as_ref())
}

#[inline]
pub fn mpz_submul(r: MpzRef<'_>, u: MpzRef<'_>, v: MpzRef<'_>) -> Mpz {
    let prod = mpz_mul(u, v);
    mpz_sub(r, prod.as_ref())
}

#[inline]
pub fn mpz_submul_ui(r: MpzRef<'_>, u: MpzRef<'_>, v: u64) -> Mpz {
    let prod = mpz_mul_ui(u, v);
    mpz_sub(r, prod.as_ref())
}

// ---- MPZ division ----

pub fn mpz_div_qr(
    n: MpzRef<'_>,
    d: MpzRef<'_>,
    mode: DivRoundMode,
    want_q: bool,
    want_r: bool,
) -> (Option<Mpz>, Option<Mpz>, bool) {
    assert!(!d.is_zero(), "mpz_div_qr: Divide by zero.");
    if n.is_zero() {
        return (
            want_q.then(Mpz::zero),
            want_r.then(Mpz::zero),
            false,
        );
    }
    let nn = n.limbs.len();
    let dn = d.limbs.len();
    let qs_neg = n.neg ^ d.neg;

    if nn < dn {
        if mode == DivRoundMode::Ceil && !qs_neg {
            let r = want_r.then(|| mpz_sub(n, d));
            let q = want_q.then(|| Mpz::from_ui(1));
            (q, r, true)
        } else if mode == DivRoundMode::Floor && qs_neg {
            let r = want_r.then(|| mpz_add(n, d));
            let q = want_q.then(|| Mpz::from_si(-1));
            (q, r, true)
        } else {
            let r = want_r.then(|| n.to_owned_mpz());
            let q = want_q.then(Mpz::zero);
            (q, r, true)
        }
    } else {
        let mut np = n.limbs.to_vec();
        let qn = nn - dn + 1;
        let mut q_buf = if want_q { vec![0u64; qn] } else { Vec::new() };
        if want_q {
            mpn_div_qr(Some(&mut q_buf), &mut np, d.limbs);
        } else {
            mpn_div_qr(None, &mut np, d.limbs);
        }
        np.truncate(dn);
        let mut tr = Mpz::from_limbs(n.neg, np);
        let rem_nonzero = !tr.is_zero();
        let mut tq = if want_q {
            Some(Mpz::from_limbs(qs_neg, q_buf))
        } else {
            None
        };

        if mode == DivRoundMode::Floor && qs_neg && rem_nonzero {
            if let Some(q) = &mut tq {
                *q = mpz_sub_ui(q.as_ref(), 1);
            }
            if want_r {
                tr = mpz_add(tr.as_ref(), d);
            }
        } else if mode == DivRoundMode::Ceil && !qs_neg && rem_nonzero {
            if let Some(q) = &mut tq {
                *q = mpz_add_ui(q.as_ref(), 1);
            }
            if want_r {
                tr = mpz_sub(tr.as_ref(), d);
            }
        }
        (tq, want_r.then_some(tr), rem_nonzero)
    }
}

pub fn mpz_div_q_2exp(u: MpzRef<'_>, bit_index: u64, mode: DivRoundMode) -> Mpz {
    if u.is_zero() {
        return Mpz::zero();
    }
    let un = u.limbs.len();
    let limb_cnt = (bit_index / GMP_LIMB_BITS as u64) as usize;
    let rem_bits = (bit_index % GMP_LIMB_BITS as u64) as u32;
    let target_mode = if !u.neg {
        DivRoundMode::Ceil
    } else {
        DivRoundMode::Floor
    };
    let adjust = if mode == target_mode {
        limb_cnt >= un
            || mpn_zero_p(&u.limbs[..limb_cnt]) == 0
            || (rem_bits > 0 && (u.limbs[limb_cnt] & ((1u64 << rem_bits) - 1)) != 0)
    } else {
        false
    };

    let mut q = if limb_cnt >= un {
        Mpz::zero()
    } else {
        let qn = un - limb_cnt;
        let mut qp = vec![0u64; qn];
        if rem_bits != 0 {
            mpn_rshift(&mut qp, &u.limbs[limb_cnt..], rem_bits);
        } else {
            qp.copy_from_slice(&u.limbs[limb_cnt..]);
        }
        Mpz::from_limbs(false, qp)
    };

    if adjust {
        q = mpz_add_ui(q.as_ref(), 1);
    }
    if u.neg && !q.is_zero() {
        q.neg = true;
    }
    q
}

pub fn mpz_div_r_2exp(u: MpzRef<'_>, bit_index: u64, mode: DivRoundMode) -> Mpz {
    if u.is_zero() || bit_index == 0 {
        return Mpz::zero();
    }
    let rn = ((bit_index + GMP_LIMB_BITS as u64 - 1) / GMP_LIMB_BITS as u64) as usize;
    let un = u.limbs.len();
    let shift_top = rn * GMP_LIMB_BITS - bit_index as usize;
    let mask = GMP_LIMB_MAX >> shift_top;
    let target_mode = if !u.neg {
        DivRoundMode::Ceil
    } else {
        DivRoundMode::Floor
    };

    let mut rp = vec![0u64; rn];
    let mut res_neg = u.neg;

    if rn > un {
        if mode == target_mode {
            let cy = mpn_neg(&mut rp[..un], u.limbs);
            assert_eq!(cy, 1);
            for slot in &mut rp[un..rn - 1] {
                *slot = GMP_LIMB_MAX;
            }
            rp[rn - 1] = mask;
            res_neg = !res_neg;
        } else {
            return u.to_owned_mpz();
        }
    } else {
        rp[..rn - 1].copy_from_slice(&u.limbs[..rn - 1]);
        rp[rn - 1] = u.limbs[rn - 1] & mask;
        if mode == target_mode {
            mpn_neg_in_place(&mut rp);
            rp[rn - 1] &= mask;
            res_neg = !res_neg;
        }
    }
    Mpz::from_limbs(res_neg, rp)
}

#[inline]
pub fn mpz_divexact(n: MpzRef<'_>, d: MpzRef<'_>) -> Mpz {
    let (q, _, rem) = mpz_div_qr(n, d, DivRoundMode::Trunc, true, false);
    assert!(!rem);
    q.unwrap()
}

#[inline]
pub fn mpz_divisible_p(n: MpzRef<'_>, d: MpzRef<'_>) -> i32 {
    let (_, _, rem) = mpz_div_qr(n, d, DivRoundMode::Trunc, false, false);
    i32::from(!rem)
}

#[inline]
pub fn mpz_congruent_p(a: MpzRef<'_>, b: MpzRef<'_>, m: MpzRef<'_>) -> i32 {
    if m.is_zero() {
        return i32::from(mpz_cmp(a, b) == 0);
    }
    let diff = mpz_sub(a, b);
    mpz_divisible_p(diff.as_ref(), m)
}

#[inline]
pub fn mpz_div_qr_ui(
    n: MpzRef<'_>,
    d: u64,
    mode: DivRoundMode,
    want_q: bool,
    want_r: bool,
) -> (Option<Mpz>, Option<Mpz>, u64) {
    let dd = Mpz::from_ui(d);
    let (q, r_opt, _) = mpz_div_qr(n, dd.as_ref(), mode, want_q, true);
    let rr = r_opt.unwrap();
    let ret = mpz_get_ui(rr.as_ref());
    (q, want_r.then_some(rr), ret)
}

// ---- GCD, LCM, Modular Inverse, Powers, Roots, Combinatorics, Primality ----

fn make_odd(r: &mut Mpz) -> u64 {
    assert!(!r.is_zero() && !r.neg);
    let shift = mpn_scan1(&r.limbs, 0);
    if shift > 0 {
        *r = mpz_div_q_2exp(r.as_ref(), shift, DivRoundMode::Trunc);
    }
    shift
}

pub fn mpz_gcd(u: MpzRef<'_>, v: MpzRef<'_>) -> Mpz {
    if u.is_zero() {
        return mpz_abs(v);
    }
    if v.is_zero() {
        return mpz_abs(u);
    }
    let mut tu = mpz_abs(u);
    let uz = make_odd(&mut tu);
    let mut tv = mpz_abs(v);
    let vz = make_odd(&mut tv);
    let gz = uz.min(vz);
    if tu.limbs.len() < tv.limbs.len() {
        core::mem::swap(&mut tu, &mut tv);
    }
    let g_limbs = mpn_gcd(&tu.limbs, &tv.limbs);
    let odd_g = Mpz::from_limbs(false, g_limbs);
    mpz_mul_2exp(odd_g.as_ref(), gz)
}

pub fn mpz_gcd_ui(u: MpzRef<'_>, v: u64) -> (Mpz, u64) {
    let vv = Mpz::from_ui(v);
    let g = mpz_gcd(u, vv.as_ref());
    let ret = if v > 0 { mpz_get_ui(g.as_ref()) } else { 0 };
    (g, ret)
}

pub fn mpz_gcdext(u_in: MpzRef<'_>, v_in: MpzRef<'_>) -> (Mpz, Mpz, Mpz) {
    if u_in.is_zero() {
        return (mpz_abs(v_in), Mpz::zero(), Mpz::from_si(i64::from(v_in.sgn())));
    }
    if v_in.is_zero() {
        return (mpz_abs(u_in), Mpz::from_si(i64::from(u_in.sgn())), Mpz::zero());
    }

    let mut u = u_in;
    let mut v = v_in;
    let mut swapped = false;

    let mut tu = mpz_abs(u);
    let mut uz = make_odd(&mut tu);
    let mut tv = mpz_abs(v);
    let mut vz = make_odd(&mut tv);
    let gz = uz.min(vz);
    uz -= gz;
    vz -= gz;

    if tu.limbs.len() < tv.limbs.len() {
        core::mem::swap(&mut tu, &mut tv);
        core::mem::swap(&mut u, &mut v);
        core::mem::swap(&mut uz, &mut vz);
        swapped = true;
    }

    let (q_opt, r_opt, _) = mpz_div_qr(tu.as_ref(), tv.as_ref(), DivRoundMode::Trunc, true, true);
    tu = r_opt.unwrap();
    let mut t1 = mpz_mul_2exp(q_opt.unwrap().as_ref(), uz);
    let mut s0 = Mpz::zero();
    let mut s1 = Mpz::zero();
    mpz_setbit(&mut s1, vz);
    let mut t0 = Mpz::zero();
    let mut power = uz + vz;

    if !tu.is_zero() {
        let mut shift = make_odd(&mut tu);
        mpz_setbit(&mut t0, uz + shift);
        power += shift;
        loop {
            let c = mpz_cmp(tu.as_ref(), tv.as_ref());
            if c == 0 {
                break;
            }
            if c < 0 {
                tv = mpz_sub(tv.as_ref(), tu.as_ref());
                t0 = mpz_add(t0.as_ref(), t1.as_ref());
                s0 = mpz_add(s0.as_ref(), s1.as_ref());
                shift = make_odd(&mut tv);
                t1 = mpz_mul_2exp(t1.as_ref(), shift);
                s1 = mpz_mul_2exp(s1.as_ref(), shift);
            } else {
                tu = mpz_sub(tu.as_ref(), tv.as_ref());
                t1 = mpz_add(t0.as_ref(), t1.as_ref());
                s1 = mpz_add(s0.as_ref(), s1.as_ref());
                shift = make_odd(&mut tu);
                t0 = mpz_mul_2exp(t0.as_ref(), shift);
                s0 = mpz_mul_2exp(s0.as_ref(), shift);
            }
            power += shift;
        }
    } else {
        mpz_setbit(&mut t0, uz);
    }

    tv = mpz_mul_2exp(tv.as_ref(), gz);
    s0 = mpz_neg(s0.as_ref());

    s1 = mpz_abs(mpz_divexact(v, tv.as_ref()).as_ref());
    t1 = mpz_abs(mpz_divexact(u, tv.as_ref()).as_ref());

    while power > 0 {
        power -= 1;
        if s0.is_odd() || t0.is_odd() {
            s0 = mpz_sub(s0.as_ref(), s1.as_ref());
            t0 = mpz_add(t0.as_ref(), t1.as_ref());
        }
        s0 = mpz_div_q_2exp(s0.as_ref(), 1, DivRoundMode::Trunc);
        t0 = mpz_div_q_2exp(t0.as_ref(), 1, DivRoundMode::Trunc);
    }

    s1 = mpz_add(s0.as_ref(), s1.as_ref());
    let cmp = mpz_cmpabs(s0.as_ref(), s1.as_ref());
    if cmp >= 0 {
        t1 = mpz_sub(t0.as_ref(), t1.as_ref());
        if cmp > 0 || mpz_cmpabs(t0.as_ref(), t1.as_ref()) > 0 {
            core::mem::swap(&mut s0, &mut s1);
            core::mem::swap(&mut t0, &mut t1);
        }
    }

    if u.neg {
        s0 = mpz_neg(s0.as_ref());
    }
    if v.neg {
        t0 = mpz_neg(t0.as_ref());
    }

    if swapped {
        (tv, t0, s0)
    } else {
        (tv, s0, t0)
    }
}

pub fn mpz_lcm(u: MpzRef<'_>, v: MpzRef<'_>) -> Mpz {
    if u.is_zero() || v.is_zero() {
        return Mpz::zero();
    }
    let g = mpz_gcd(u, v);
    let q = mpz_divexact(u, g.as_ref());
    mpz_abs(mpz_mul(q.as_ref(), v).as_ref())
}

pub fn mpz_lcm_ui(u: MpzRef<'_>, v: u64) -> Mpz {
    if v == 0 || u.is_zero() {
        return Mpz::zero();
    }
    let (_, g) = mpz_gcd_ui(u, v);
    mpz_abs(mpz_mul_ui(u, v / g).as_ref())
}

pub fn mpz_invert(u: MpzRef<'_>, m: MpzRef<'_>) -> Option<Mpz> {
    if u.is_zero() || mpz_cmpabs_ui(m, 1) <= 0 {
        return None;
    }
    let (g, mut tr, _) = mpz_gcdext(u, m);
    if mpz_cmp_ui(g.as_ref(), 1) != 0 {
        return None;
    }
    if tr.neg {
        if !m.neg {
            tr = mpz_add(tr.as_ref(), m);
        } else {
            tr = mpz_sub(tr.as_ref(), m);
        }
    }
    Some(tr)
}

pub fn mpz_pow_ui(b: MpzRef<'_>, e: u64) -> Mpz {
    let mut tr = Mpz::from_ui(1);
    if e == 0 {
        return tr;
    }
    let mut bit = 1u64 << (63 - e.leading_zeros());
    while bit > 0 {
        tr = mpz_mul(tr.as_ref(), tr.as_ref());
        if (e & bit) != 0 {
            tr = mpz_mul(tr.as_ref(), b);
        }
        bit >>= 1;
    }
    tr
}

#[inline]
pub fn mpz_ui_pow_ui(blimb: u64, e: u64) -> Mpz {
    let b = Mpz::from_ui(blimb);
    mpz_pow_ui(b.as_ref(), e)
}

pub fn mpz_powm(b: MpzRef<'_>, e: MpzRef<'_>, m: MpzRef<'_>) -> Mpz {
    assert!(!m.is_zero(), "mpz_powm: Zero modulo.");
    if e.is_zero() {
        return Mpz::from_ui(u64::from(mpz_cmpabs_ui(m, 1) != 0));
    }
    let abs_m = mpz_abs(m);
    let base = if e.neg {
        mpz_invert(b, abs_m.as_ref())
            .expect("mpz_powm: Negative exponent and non-invertible base.")
    } else {
        let (_, r, _) = mpz_div_qr(b, abs_m.as_ref(), DivRoundMode::Floor, false, true);
        r.unwrap()
    };

    let mut tr = Mpz::from_ui(1);
    for &w in e.limbs.iter().rev() {
        let mut bit = GMP_LIMB_HIGHBIT;
        while bit > 0 {
            tr = mpz_mul(tr.as_ref(), tr.as_ref());
            if (w & bit) != 0 {
                tr = mpz_mul(tr.as_ref(), base.as_ref());
            }
            let (_, r, _) = mpz_div_qr(tr.as_ref(), abs_m.as_ref(), DivRoundMode::Trunc, false, true);
            tr = r.unwrap();
            bit >>= 1;
        }
    }
    let (_, r, _) = mpz_div_qr(tr.as_ref(), abs_m.as_ref(), DivRoundMode::Trunc, false, true);
    r.unwrap()
}

#[inline]
pub fn mpz_powm_ui(b: MpzRef<'_>, elimb: u64, m: MpzRef<'_>) -> Mpz {
    let e = Mpz::from_ui(elimb);
    mpz_powm(b, e.as_ref(), m)
}

pub fn mpz_rootrem(y: MpzRef<'_>, z: u64) -> (Mpz, Mpz) {
    assert!(!(y.neg && (z & 1) == 0), "mpz_rootrem: Negative argument, with even root.");
    assert!(z != 0, "mpz_rootrem: Zeroth root.");

    if mpz_cmpabs_ui(y, 1) <= 0 {
        return (y.to_owned_mpz(), Mpz::zero());
    }

    let bc = (mpz_sizeinbase(y, 2) as u64 - 1) / z + 1;
    let mut t = Mpz::zero();
    mpz_setbit(&mut t, bc);
    let mut u;

    if z == 2 {
        loop {
            u = t;
            let (q, _, _) = mpz_div_qr(y, u.as_ref(), DivRoundMode::Trunc, true, false);
            t = mpz_add(q.unwrap().as_ref(), u.as_ref());
            t = mpz_div_q_2exp(t.as_ref(), 1, DivRoundMode::Trunc);
            if mpz_cmpabs(t.as_ref(), u.as_ref()) >= 0 {
                break;
            }
        }
    } else {
        if y.neg {
            t = mpz_neg(t.as_ref());
        }
        loop {
            u = t;
            let u_pow = mpz_pow_ui(u.as_ref(), z - 1);
            let (q, _, _) = mpz_div_qr(y, u_pow.as_ref(), DivRoundMode::Trunc, true, false);
            let v = mpz_mul_ui(u.as_ref(), z - 1);
            let sum = mpz_add(q.unwrap().as_ref(), v.as_ref());
            let (next_x, _, _) = mpz_div_qr_ui(sum.as_ref(), z, DivRoundMode::Trunc, true, false);
            t = next_x.unwrap();
            if mpz_cmpabs(t.as_ref(), u.as_ref()) >= 0 {
                break;
            }
        }
    }

    let u_pow = mpz_pow_ui(u.as_ref(), z);
    let rem = mpz_sub(y, u_pow.as_ref());
    (u, rem)
}

#[inline]
pub fn mpz_root(y: MpzRef<'_>, z: u64) -> (Mpz, i32) {
    let (x, r) = mpz_rootrem(y, z);
    let exact = i32::from(r.is_zero());
    (x, exact)
}

#[inline]
pub fn mpz_sqrtrem(u: MpzRef<'_>) -> (Mpz, Mpz) {
    mpz_rootrem(u, 2)
}

#[inline]
pub fn mpz_sqrt(u: MpzRef<'_>) -> Mpz {
    mpz_rootrem(u, 2).0
}

#[inline]
pub fn mpz_perfect_square_root(u: MpzRef<'_>) -> (Mpz, i32) {
    if u.neg {
        (Mpz::zero(), 0)
    } else if u.is_zero() {
        (Mpz::zero(), 1)
    } else {
        mpz_root(u, 2)
    }
}

#[inline]
pub fn mpz_perfect_square_p(u: MpzRef<'_>) -> i32 {
    mpz_perfect_square_root(u).1
}

pub fn mpz_mfac_uiui(mut n: u64, m: u64) -> Mpz {
    let mut x = Mpz::from_ui(n + u64::from(n == 0));
    if m.wrapping_add(1) < 2 {
        return x;
    }
    while n > m + 1 {
        n -= m;
        x = mpz_mul_ui(x.as_ref(), n);
    }
    x
}

#[inline]
pub fn mpz_2fac_ui(n: u64) -> Mpz {
    mpz_mfac_uiui(n, 2)
}

#[inline]
pub fn mpz_fac_ui(n: u64) -> Mpz {
    mpz_mfac_uiui(n, 1)
}

pub fn mpz_bin_uiui(mut n: u64, mut k: u64) -> Mpz {
    let mut r = Mpz::from_ui(u64::from(k <= n));
    if k > (n >> 1) {
        k = if k <= n { n - k } else { 0 };
    }
    let t = mpz_fac_ui(k);
    while k > 0 {
        r = mpz_mul_ui(r.as_ref(), n);
        n -= 1;
        k -= 1;
    }
    mpz_divexact(r.as_ref(), t.as_ref())
}

fn jacobi_coprime(mut a: Limb, mut b: Limb) -> i32 {
    assert!((b & 1) != 0 && a != 0);
    let mut bit: i32 = 0;
    b >>= 1;
    let mut c = a.trailing_zeros();
    a >>= 1;
    loop {
        a >>= c;
        bit ^= (c & ((b ^ (b >> 1)) as u32)) as i32;
        if a < b {
            if a == 0 {
                return if (bit & 1) != 0 { -1 } else { 1 };
            }
            bit ^= (a & b) as i32;
            a = b - a;
            b -= a;
        } else {
            a = a.wrapping_sub(b);
            assert!(a != 0);
        }
        c = a.trailing_zeros() + 1;
    }
}

fn lucas_step_k_2k(v: &mut Mpz, qk: &mut Mpz, n: MpzRef<'_>) {
    let (_, qk_mod, _) = mpz_div_qr(qk.as_ref(), n, DivRoundMode::Floor, false, true);
    *qk = qk_mod.unwrap();
    let v_sq = mpz_mul(v.as_ref(), v.as_ref());
    let sub = mpz_submul_ui(v_sq.as_ref(), qk.as_ref(), 2);
    let (_, v_rem, _) = mpz_div_qr(sub.as_ref(), n, DivRoundMode::Trunc, false, true);
    *v = v_rem.unwrap();
    *qk = mpz_mul(qk.as_ref(), qk.as_ref());
}

fn lucas_mod(v: &mut Mpz, qk: &mut Mpz, q_param: i64, b0: u64, n: MpzRef<'_>) -> bool {
    let mut u = Mpz::from_ui(1);
    *v = Mpz::from_ui(1);
    *qk = Mpz::from_si(q_param);
    let mut bs = mpz_sizeinbase(n, 2) as u64 - 1;
    while bs > b0 {
        bs -= 1;
        u = mpz_mul(u.as_ref(), v.as_ref());
        lucas_step_k_2k(v, qk, n);
        if b0 == bs || mpz_tstbit(n, bs) != 0 {
            *qk = mpz_mul_si(qk.as_ref(), q_param);
            core::mem::swap(&mut u, v);
            u = mpz_add(u.as_ref(), v.as_ref());
            if u.is_odd() {
                u = mpz_add(u.as_ref(), n);
            }
            u = mpz_div_q_2exp(u.as_ref(), 1, DivRoundMode::Trunc);
            *v = mpz_mul_si(v.as_ref(), -2 * q_param);
            *v = mpz_add(u.as_ref(), v.as_ref());
            let (_, vr, _) = mpz_div_qr(v.as_ref(), n, DivRoundMode::Trunc, false, true);
            *v = vr.unwrap();
        }
        let (_, ur, _) = mpz_div_qr(u.as_ref(), n, DivRoundMode::Trunc, false, true);
        u = ur.unwrap();
    }
    u.is_zero()
}

fn stronglucas(x: MpzRef<'_>) -> i32 {
    let n = MpzRef {
        neg: false,
        limbs: x.limbs,
    };
    let (mut qk, exact_sq) = mpz_root(n, 2);
    if exact_sq != 0 {
        return 0;
    }
    let max_d = if qk.limbs.len() == 1 {
        qk.limbs[0] - 1
    } else {
        GMP_LIMB_MAX
    };
    let mut d: Limb = 3;
    loop {
        if d >= max_d {
            return 1 + i32::from(d != GMP_LIMB_MAX);
        }
        d += 2;
        let (_, _, tl) = mpz_div_qr_ui(n, d, DivRoundMode::Trunc, false, false);
        if tl == 0 {
            return 0;
        }
        if jacobi_coprime(tl, d) != 1 {
            break;
        }
    }
    let mut v = Mpz::zero();
    let mut b0 = mpn_scan0(n.limbs, 0);
    let q_param = if (d & 2) != 0 {
        ((d >> 2) as i64) + 1
    } else {
        -((d >> 2) as i64)
    };
    if !lucas_mod(&mut v, &mut qk, q_param, b0, n) {
        while !v.is_zero() && {
            b0 -= 1;
            b0 != 0
        } {
            lucas_step_k_2k(&mut v, &mut qk, n);
        }
    }
    i32::from(b0 != 0)
}

fn millerrabin(n: MpzRef<'_>, nm1: MpzRef<'_>, base: MpzRef<'_>, q: MpzRef<'_>, mut k: u64) -> bool {
    let mut y = mpz_powm(base, q, n);
    if mpz_cmp_ui(y.as_ref(), 1) == 0 || mpz_cmp(y.as_ref(), nm1) == 0 {
        return true;
    }
    while {
        k -= 1;
        k > 0
    } {
        y = mpz_powm_ui(y.as_ref(), 2, n);
        if mpz_cmp(y.as_ref(), nm1) == 0 {
            return true;
        }
    }
    false
}

pub fn mpz_probab_prime_p(n_in: MpzRef<'_>, mut reps: i32) -> i32 {
    const GMP_PRIME_PRODUCT: u64 = 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23 * 29;
    const GMP_PRIME_MASK: u64 = 0xc96996dc;

    let n = MpzRef {
        neg: false,
        limbs: n_in.limbs,
    };
    if n.limbs.first().is_none_or(|l| (l & 1) == 0) {
        return if mpz_cmpabs_ui(n, 2) == 0 { 2 } else { 0 };
    }
    if mpz_cmpabs_ui(n, 64) < 0 {
        return ((GMP_PRIME_MASK >> (n.limbs[0] >> 1)) & 2) as i32;
    }
    if mpz_gcd_ui(n, GMP_PRIME_PRODUCT).1 != 1 {
        return 0;
    }
    if mpz_cmpabs_ui(n, 31 * 31) < 0 {
        return 2;
    }

    let mut nm1 = n.to_owned_mpz();
    nm1.limbs[0] -= 1;
    let k = mpn_scan1(&nm1.limbs, 0);
    let q = mpz_div_q_2exp(nm1.as_ref(), k, DivRoundMode::Trunc);

    let two = Mpz::from_ui(2);
    let mut is_prime = i32::from(
        millerrabin(n, nm1.as_ref(), two.as_ref(), q.as_ref(), k) && stronglucas(n) != 0,
    );
    reps -= 24;
    let mut j: i32 = 0;
    while is_prime != 0 && j < reps {
        let base_val = (i64::from(j) * i64::from(j) + i64::from(j) + 41) as u64;
        let y = Mpz::from_ui(base_val);
        if mpz_cmp(y.as_ref(), nm1.as_ref()) >= 0 {
            break;
        }
        is_prime = i32::from(millerrabin(n, nm1.as_ref(), y.as_ref(), q.as_ref(), k));
        j += 1;
    }
    is_prime
}

// ---- Bitwise & Logical operations ----

pub fn mpz_tstbit(d: MpzRef<'_>, bit_index: u64) -> i32 {
    let dn = d.limbs.len();
    let limb_index = (bit_index / GMP_LIMB_BITS as u64) as usize;
    if limb_index >= dn {
        return i32::from(d.neg);
    }
    let shift = (bit_index % GMP_LIMB_BITS as u64) as u32;
    let w = d.limbs[limb_index];
    let bit = ((w >> shift) & 1) as i32;
    if d.neg {
        if shift > 0 && (w << (GMP_LIMB_BITS as u32 - shift)) > 0 {
            return bit ^ 1;
        }
        if d.limbs[..limb_index].iter().any(|&x| x > 0) {
            return bit ^ 1;
        }
    }
    bit
}

fn abs_add_bit(d: &mut Mpz, bit_index: u64) {
    let limb_index = (bit_index / GMP_LIMB_BITS as u64) as usize;
    let bit = 1u64 << (bit_index % GMP_LIMB_BITS as u64);
    if limb_index >= d.limbs.len() {
        d.limbs.resize(limb_index + 1, 0);
        d.limbs[limb_index] = bit;
    } else {
        let cy = mpn_add_1_in_place(&mut d.limbs[limb_index..], bit);
        if cy > 0 {
            d.limbs.push(cy);
        }
    }
    d.normalize();
}

fn abs_sub_bit(d: &mut Mpz, bit_index: u64) {
    let limb_index = (bit_index / GMP_LIMB_BITS as u64) as usize;
    let bit = 1u64 << (bit_index % GMP_LIMB_BITS as u64);
    assert!(limb_index < d.limbs.len());
    let cy = mpn_sub_1_in_place(&mut d.limbs[limb_index..], bit);
    assert_eq!(cy, 0);
    d.normalize();
}

pub fn mpz_setbit(d: &mut Mpz, bit_index: u64) {
    if mpz_tstbit(d.as_ref(), bit_index) == 0 {
        if !d.neg {
            abs_add_bit(d, bit_index);
        } else {
            abs_sub_bit(d, bit_index);
        }
    }
}

pub fn mpz_clrbit(d: &mut Mpz, bit_index: u64) {
    if mpz_tstbit(d.as_ref(), bit_index) != 0 {
        if !d.neg {
            abs_sub_bit(d, bit_index);
        } else {
            abs_add_bit(d, bit_index);
        }
    }
}

pub fn mpz_combit(d: &mut Mpz, bit_index: u64) {
    if (mpz_tstbit(d.as_ref(), bit_index) != 0) ^ d.neg {
        abs_sub_bit(d, bit_index);
    } else {
        abs_add_bit(d, bit_index);
    }
}

#[inline]
pub fn mpz_com(u: MpzRef<'_>) -> Mpz {
    let p1 = mpz_add_ui(u, 1);
    mpz_neg(p1.as_ref())
}

pub fn mpz_and<'a>(mut u: MpzRef<'a>, mut v: MpzRef<'a>) -> Mpz {
    if u.limbs.len() < v.limbs.len() {
        core::mem::swap(&mut u, &mut v);
    }
    let un = u.limbs.len();
    let vn = v.limbs.len();
    if vn == 0 {
        return Mpz::zero();
    }
    let mut uc = Limb::from(u.neg);
    let mut vc = Limb::from(v.neg);
    let mut rc = uc & vc;
    let ux = uc.wrapping_neg();
    let vx = vc.wrapping_neg();
    let rx = rc.wrapping_neg();

    let rn = if vx != 0 { un } else { vn };
    let mut rp = Vec::with_capacity(rn + rc as usize);
    for i in 0..vn {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let vl = (v.limbs[i] ^ vx).wrapping_add(vc);
        vc = Limb::from(vl < vc);
        let rl = ((ul & vl) ^ rx).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    for i in vn..rn {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let rl = ((ul & vx) ^ rx).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    if rc != 0 {
        rp.push(rc);
    }
    Mpz::from_limbs(rx != 0, rp)
}

pub fn mpz_ior<'a>(mut u: MpzRef<'a>, mut v: MpzRef<'a>) -> Mpz {
    if u.limbs.len() < v.limbs.len() {
        core::mem::swap(&mut u, &mut v);
    }
    let un = u.limbs.len();
    let vn = v.limbs.len();
    if vn == 0 {
        return u.to_owned_mpz();
    }
    let mut uc = Limb::from(u.neg);
    let mut vc = Limb::from(v.neg);
    let mut rc = uc | vc;
    let ux = uc.wrapping_neg();
    let vx = vc.wrapping_neg();
    let rx = rc.wrapping_neg();

    let rn = if vx != 0 { vn } else { un };
    let mut rp = Vec::with_capacity(rn + rc as usize);
    for i in 0..vn {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let vl = (v.limbs[i] ^ vx).wrapping_add(vc);
        vc = Limb::from(vl < vc);
        let rl = ((ul | vl) ^ rx).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    for i in vn..rn {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let rl = ((ul | vx) ^ rx).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    if rc != 0 {
        rp.push(rc);
    }
    Mpz::from_limbs(rx != 0, rp)
}

pub fn mpz_xor<'a>(mut u: MpzRef<'a>, mut v: MpzRef<'a>) -> Mpz {
    if u.limbs.len() < v.limbs.len() {
        core::mem::swap(&mut u, &mut v);
    }
    let un = u.limbs.len();
    let vn = v.limbs.len();
    if vn == 0 {
        return u.to_owned_mpz();
    }
    let mut uc = Limb::from(u.neg);
    let mut vc = Limb::from(v.neg);
    let mut rc = uc ^ vc;
    let ux = uc.wrapping_neg();
    let vx = vc.wrapping_neg();
    let rx = rc.wrapping_neg();

    let mut rp = Vec::with_capacity(un + rc as usize);
    for i in 0..vn {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let vl = (v.limbs[i] ^ vx).wrapping_add(vc);
        vc = Limb::from(vl < vc);
        let rl = (ul ^ vl ^ rx).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    for i in vn..un {
        let ul = (u.limbs[i] ^ ux).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let rl = (ul ^ ux).wrapping_add(rc);
        rc = Limb::from(rl < rc);
        rp.push(rl);
    }
    if rc != 0 {
        rp.push(rc);
    }
    Mpz::from_limbs(rx != 0, rp)
}

#[inline]
pub fn mpz_popcount(u: MpzRef<'_>) -> u64 {
    if u.neg {
        u64::MAX
    } else {
        mpn_popcount(u.limbs)
    }
}

pub fn mpz_hamdist<'a>(mut u: MpzRef<'a>, mut v: MpzRef<'a>) -> u64 {
    if u.neg != v.neg {
        return u64::MAX;
    }
    if u.limbs.len() < v.limbs.len() {
        core::mem::swap(&mut u, &mut v);
    }
    let mut uc = Limb::from(u.neg);
    let mut vc = uc;
    let comp = uc.wrapping_neg();
    let mut c = 0u64;
    for i in 0..v.limbs.len() {
        let ul = (u.limbs[i] ^ comp).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        let vl = (v.limbs[i] ^ comp).wrapping_add(vc);
        vc = Limb::from(vl < vc);
        c += u64::from((ul ^ vl).count_ones());
    }
    for i in v.limbs.len()..u.limbs.len() {
        let ul = (u.limbs[i] ^ comp).wrapping_add(uc);
        uc = Limb::from(ul < uc);
        c += u64::from((ul ^ comp).count_ones());
    }
    c
}

pub fn mpz_scan1(u: MpzRef<'_>, starting_bit: u64) -> u64 {
    let un = u.limbs.len();
    let i = (starting_bit / GMP_LIMB_BITS as u64) as usize;
    if i >= un {
        return if !u.neg { u64::MAX } else { starting_bit };
    }
    let mut ux: Limb = 0;
    let mut limb = u.limbs[i];
    if starting_bit != 0 {
        if u.neg {
            ux = mpn_zero_p(&u.limbs[..i]) as Limb;
            limb = (!limb).wrapping_add(ux);
            ux = Limb::from(limb >= ux).wrapping_neg();
        }
        limb &= GMP_LIMB_MAX << (starting_bit % GMP_LIMB_BITS as u64);
    }
    mpn_common_scan(limb, i, u.limbs, ux)
}

pub fn mpz_scan0(u: MpzRef<'_>, starting_bit: u64) -> u64 {
    let un = u.limbs.len();
    let ux = Limb::from(!u.neg).wrapping_neg();
    let i = (starting_bit / GMP_LIMB_BITS as u64) as usize;
    if i >= un {
        return if ux != 0 { starting_bit } else { u64::MAX };
    }
    let mut limb = u.limbs[i] ^ ux;
    if ux == 0 {
        limb = limb.wrapping_sub(mpn_zero_p(&u.limbs[..i]) as Limb);
    }
    limb &= GMP_LIMB_MAX << (starting_bit % GMP_LIMB_BITS as u64);
    mpn_common_scan(limb, i, u.limbs, ux)
}

// ---- Base conversion & binary import/export ----

pub fn mpz_sizeinbase(u: MpzRef<'_>, base: i32) -> usize {
    assert!((2..=62).contains(&base));
    let un = u.limbs.len();
    if un == 0 {
        return 1;
    }
    let bits = (un - 1) * GMP_LIMB_BITS + limb_size_in_base_2(u.limbs[un - 1]);
    match base {
        2 => return bits,
        4 => return (bits + 1) / 2,
        8 => return (bits + 2) / 3,
        16 => return (bits + 3) / 4,
        32 => return (bits + 4) / 5,
        _ => {}
    }
    let mut tp = u.limbs.to_vec();
    let mut tn = un;
    let bi = div_qr_1_invert(base as Limb);
    let mut ndigits = 0usize;
    while tn > 0 {
        ndigits += 1;
        div_qr_1_preinv_in_place(&mut tp[..tn], &bi);
        if tp[tn - 1] == 0 {
            tn -= 1;
        }
    }
    ndigits
}

pub fn mpz_get_str_bytes(mut base: i32, u: MpzRef<'_>) -> Option<Vec<u8>> {
    const DIGITS_UPPER_62: &[u8; 62] =
        b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const DIGITS_LOWER_36: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

    let digits: &[u8] = if base > 1 {
        if base <= 36 {
            DIGITS_LOWER_36
        } else if base <= 62 {
            DIGITS_UPPER_62
        } else {
            return None;
        }
    } else if base >= -1 {
        base = 10;
        DIGITS_LOWER_36
    } else {
        base = -base;
        if base > 36 {
            return None;
        }
        DIGITS_UPPER_62
    };

    if u.is_zero() {
        return Some(vec![b'0']);
    }

    let raw_digits = mpn_get_str(base, u.limbs);
    let mut out = Vec::with_capacity(raw_digits.len() + usize::from(u.neg));
    if u.neg {
        out.push(b'-');
    }
    for d in raw_digits {
        out.push(digits[d as usize]);
    }
    Some(out)
}

#[inline]
fn is_c_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

pub fn mpz_set_str_bytes(bytes: &[u8], mut base: i32) -> Result<Mpz, ()> {
    assert!(base == 0 || (2..=62).contains(&base));
    let mut idx = 0usize;
    while idx < bytes.len() && is_c_space(bytes[idx]) {
        idx += 1;
    }
    let neg = if idx < bytes.len() && bytes[idx] == b'-' {
        idx += 1;
        true
    } else {
        false
    };

    if base == 0 {
        if idx < bytes.len() && bytes[idx] == b'0' {
            if idx + 1 < bytes.len() && (bytes[idx + 1] == b'x' || bytes[idx + 1] == b'X') {
                base = 16;
                idx += 2;
            } else if idx + 1 < bytes.len() && (bytes[idx + 1] == b'b' || bytes[idx + 1] == b'B') {
                base = 2;
                idx += 2;
            } else {
                base = 8;
            }
        } else {
            base = 10;
        }
    }

    if idx >= bytes.len() {
        return Err(());
    }

    let value_of_a: u32 = if base > 36 { 36 } else { 10 };
    let mut digits = Vec::with_capacity(bytes.len() - idx);
    while idx < bytes.len() {
        let ch = bytes[idx];
        idx += 1;
        if is_c_space(ch) {
            continue;
        }
        let digit = if ch.is_ascii_digit() {
            u32::from(ch - b'0')
        } else if ch.is_ascii_lowercase() {
            u32::from(ch - b'a') + value_of_a
        } else if ch.is_ascii_uppercase() {
            u32::from(ch - b'A') + 10
        } else {
            return Err(());
        };
        if digit >= base as u32 {
            return Err(());
        }
        digits.push(digit as u8);
    }

    if digits.is_empty() {
        return Err(());
    }

    let limbs = mpn_set_str(&digits, base);
    Ok(Mpz::from_limbs(neg, limbs))
}

#[inline]
fn native_endian_flag() -> i32 {
    if cfg!(target_endian = "big") {
        1
    } else {
        -1
    }
}

pub fn mpz_import_bytes(
    count: usize,
    order: i32,
    size: usize,
    mut endian: i32,
    nails: usize,
    src: &[u8],
) -> Mpz {
    assert_eq!(nails, 0, "mpz_import: Nails not supported.");
    assert!(order == 1 || order == -1);
    assert!((-1..=1).contains(&endian));
    if count == 0 || size == 0 {
        return Mpz::zero();
    }
    let total = count.checked_mul(size).expect("mpz_import overflow");
    assert!(src.len() >= total);
    if endian == 0 {
        endian = native_endian_flag();
    }

    let rn = (total + 7) / 8;
    let mut rp = Vec::with_capacity(rn);
    let mut limb: Limb = 0;
    let mut byte_in_limb = 0usize;

    for w_idx in 0..count {
        let word_idx = if order == 1 {
            count - 1 - w_idx
        } else {
            w_idx
        };
        let word_base = word_idx * size;
        for b_idx in 0..size {
            let pos = if endian == 1 {
                word_base + (size - 1 - b_idx)
            } else {
                word_base + b_idx
            };
            limb |= Limb::from(src[pos]) << (byte_in_limb * 8);
            byte_in_limb += 1;
            if byte_in_limb == 8 {
                rp.push(limb);
                limb = 0;
                byte_in_limb = 0;
            }
        }
    }
    if byte_in_limb > 0 && limb != 0 {
        rp.push(limb);
    }
    Mpz::from_limbs(false, rp)
}

pub fn mpz_export_bytes(
    order: i32,
    size: usize,
    mut endian: i32,
    nails: usize,
    u: MpzRef<'_>,
) -> (Vec<u8>, usize) {
    assert_eq!(nails, 0, "mpz_export: Nails not supported.");
    assert!(order == 1 || order == -1);
    assert!((-1..=1).contains(&endian));
    assert!(size > 0 || u.is_zero());

    if u.is_zero() {
        return (Vec::new(), 0);
    }
    if endian == 0 {
        endian = native_endian_flag();
    }

    let un = u.limbs.len();
    let mut top = u.limbs[un - 1];
    let mut k = 0usize;
    while top != 0 {
        k += 1;
        top >>= 8;
    }
    let count = (k + (un - 1) * 8 + size - 1) / size;
    let mut out = vec![0u8; count * size];

    let mut limb: Limb = 0;
    let mut bytes_left_in_limb = 0usize;
    let mut limb_idx = 0usize;

    for w_idx in 0..count {
        let word_idx = if order == 1 {
            count - 1 - w_idx
        } else {
            w_idx
        };
        let word_base = word_idx * size;
        for b_idx in 0..size {
            if bytes_left_in_limb == 0 {
                limb = if limb_idx < un {
                    let v = u.limbs[limb_idx];
                    limb_idx += 1;
                    v
                } else {
                    0
                };
                bytes_left_in_limb = 8;
            }
            let pos = if endian == 1 {
                word_base + (size - 1 - b_idx)
            } else {
                word_base + b_idx
            };
            out[pos] = limb as u8;
            limb >>= 8;
            bytes_left_in_limb -= 1;
        }
    }
    (out, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udiv_preinv_high_limb_wraparound_and_multilimb_division() {
        // Regression guard for nh == u64::MAX wrapping in udiv_qrnnd_preinv.
        let d = (1u64 << 63) | 1;
        let inv = div_qr_1_invert(d);
        let (q, r) = udiv_qrnnd_preinv(d - 1, u64::MAX, d, inv.di);
        let lhs = (u128::from(d - 1) << 64) | u128::from(u64::MAX);
        assert_eq!(u128::from(q) * u128::from(d) + u128::from(r), lhs);
        assert!(r < d);

        // Multi-limb 1-, 2-, and 5-limb Euclidean division across all sign combinations.
        let a_mag = Mpz::from_limbs(
            false,
            vec![
                0xffff_ffff_ffff_fffe,
                0x0123_4567_89ab_cdef,
                0xfedc_ba98_7654_3210,
                0x8000_0000_0000_0001,
                0x0000_0000_dead_beef,
            ],
        );
        for d_limbs in [
            vec![0x8000_0000_0000_0003],
            vec![0xffff_ffff_ffff_ffff, 0x8000_0000_0000_0000],
            vec![1, 0, 0x0000_0001_0000_0001],
        ] {
            let b_mag = Mpz::from_limbs(false, d_limbs);
            for neg_a in [false, true] {
                for neg_b in [false, true] {
                    let mut a = a_mag.clone();
                    a.neg = neg_a;
                    let mut b = b_mag.clone();
                    b.neg = neg_b;

                    let (Some(tq), Some(tr), _) =
                        mpz_div_qr(a.as_ref(), b.as_ref(), DivRoundMode::Trunc, true, true)
                    else {
                        unreachable!()
                    };
                    assert_eq!(mpz_add(mpz_mul(tq.as_ref(), b.as_ref()).as_ref(), tr.as_ref()), a);
                    assert!(mpz_cmpabs(tr.as_ref(), b.as_ref()) < 0);

                    let (Some(fq), Some(fr), _) =
                        mpz_div_qr(a.as_ref(), b.as_ref(), DivRoundMode::Floor, true, true)
                    else {
                        unreachable!()
                    };
                    assert_eq!(mpz_add(mpz_mul(fq.as_ref(), b.as_ref()).as_ref(), fr.as_ref()), a);
                    if !fr.is_zero() {
                        assert_eq!(fr.neg, b.neg);
                    }

                    let (Some(cq), Some(cr), _) =
                        mpz_div_qr(a.as_ref(), b.as_ref(), DivRoundMode::Ceil, true, true)
                    else {
                        unreachable!()
                    };
                    assert_eq!(mpz_add(mpz_mul(cq.as_ref(), b.as_ref()).as_ref(), cr.as_ref()), a);
                    if !cr.is_zero() {
                        assert_eq!(cr.neg, !b.neg);
                    }
                }
            }
        }
    }

    #[test]
    fn gcdext_invert_powm_rootrem_and_bpsw_mersenne_primes() {
        let a = mpz_set_str_bytes(b"1234567890123456789012345678901234567890", 10).unwrap();
        let b = mpz_set_str_bytes(b"98765432109876543210987654321", 10).unwrap();
        let (g, s, t) = mpz_gcdext(a.as_ref(), b.as_ref());
        let bezout = mpz_add(
            mpz_mul(a.as_ref(), s.as_ref()).as_ref(),
            mpz_mul(b.as_ref(), t.as_ref()).as_ref(),
        );
        assert_eq!(bezout, g);

        // Mersenne prime 2^127 - 1
        let m127 = mpz_sub_ui(mpz_ui_pow_ui(2, 127).as_ref(), 1);
        assert!(mpz_probab_prime_p(m127.as_ref(), 25) > 0);
        // Mersenne prime 2^89 - 1 and composite 2^67 - 1
        let m89 = mpz_sub_ui(mpz_ui_pow_ui(2, 89).as_ref(), 1);
        let c67 = mpz_sub_ui(mpz_ui_pow_ui(2, 67).as_ref(), 1);
        assert!(mpz_probab_prime_p(m89.as_ref(), 25) > 0);
        assert_eq!(mpz_probab_prime_p(c67.as_ref(), 25), 0);
        // Carmichael numbers must be rejected
        assert_eq!(mpz_probab_prime_p(Mpz::from_ui(561).as_ref(), 25), 0);
        assert_eq!(mpz_probab_prime_p(Mpz::from_ui(1729).as_ref(), 25), 0);

        // Modular inverse & Fermat's little theorem mod 2^127 - 1
        let base = Mpz::from_ui(42);
        let inv = mpz_invert(base.as_ref(), m127.as_ref()).expect("invertible mod prime");
        let (_, Some(rem_mod), _) = mpz_div_qr(
            mpz_mul(base.as_ref(), inv.as_ref()).as_ref(),
            m127.as_ref(),
            DivRoundMode::Floor,
            false,
            true,
        ) else {
            unreachable!()
        };
        assert_eq!(rem_mod, Mpz::from_ui(1));
        let exp = mpz_sub_ui(m127.as_ref(), 2);
        assert_eq!(mpz_powm(base.as_ref(), exp.as_ref(), m127.as_ref()), inv);

        // 5th root with remainder
        let target = mpz_add_ui(mpz_ui_pow_ui(1234567, 5).as_ref(), 999);
        let (root, rem) = mpz_rootrem(target.as_ref(), 5);
        assert_eq!(root, Mpz::from_ui(1234567));
        assert_eq!(rem, Mpz::from_ui(999));
    }

    #[test]
    fn negative_twos_complement_float_base_and_import_export_roundtrips() {
        let x = mpz_neg(mpz_ui_pow_ui(2, 193).as_ref());
        let y = mpz_sub_ui(mpz_ui_pow_ui(2, 130).as_ref(), 1);
        // De Morgan & partition identities on negative multi-limb integers
        let lhs = mpz_xor(
            mpz_and(x.as_ref(), y.as_ref()).as_ref(),
            mpz_and(x.as_ref(), mpz_com(y.as_ref()).as_ref()).as_ref(),
        );
        assert_eq!(lhs, x);
        assert_eq!(
            mpz_ior(x.as_ref(), y.as_ref()),
            mpz_com(mpz_and(mpz_com(x.as_ref()).as_ref(), mpz_com(y.as_ref()).as_ref()).as_ref())
        );

        // Float conversion & exact comparison
        let f_exact = 9_007_199_254_740_992.0_f64; // 2^53
        let z = mpz_from_d(f_exact);
        assert_eq!(mpz_get_d(z.as_ref()), f_exact);
        assert_eq!(mpz_cmp_d(z.as_ref(), f_exact), 0);
        assert!(mpz_cmp_d(mpz_add_ui(z.as_ref(), 1).as_ref(), f_exact) > 0);

        // Base -36..=62 roundtrips
        let sample = mpz_neg(mpz_set_str_bytes(b"deadbeefcafebabe123456789abcdef012345", 16).unwrap().as_ref());
        for base in [2, 8, 10, 16, 36, 62] {
            let text = mpz_get_str_bytes(base, sample.as_ref()).unwrap();
            let parsed = mpz_set_str_bytes(&text, base).unwrap();
            assert_eq!(parsed, sample, "base {base} roundtrip failed");
        }
        let upper = mpz_get_str_bytes(-16, sample.as_ref()).unwrap();
        assert!(upper.iter().any(|b| b.is_ascii_uppercase()));

        // Import/export across order {-1, 1}, endian {-1, 0, 1}, word size {1, 4, 8}
        let pos = mpz_abs(sample.as_ref());
        for order in [-1, 1] {
            for endian in [-1, 0, 1] {
                for size in [1usize, 4, 8] {
                    let (bytes, count) = mpz_export_bytes(order, size, endian, 0, pos.as_ref());
                    let back = mpz_import_bytes(count, order, size, endian, 0, &bytes);
                    assert_eq!(back, pos, "order={order} endian={endian} size={size}");
                }
            }
        }
    }
}
