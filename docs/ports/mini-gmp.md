# emacs/lib/mini-gmp.c (4699 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 205
- [x] gmp_die  (line 293, static)
- [x] gmp_default_alloc  (line 300, static)
- [x] gmp_default_realloc  (line 314, static)
- [x] gmp_default_free  (line 327, static)
- [x] mp_get_memory_functions  (line 337)
- [x] mp_set_memory_functions  (line 352)
- [x] gmp_alloc_limbs  (line 373, static)
- [x] gmp_realloc_limbs  (line 379, static)
- [x] gmp_free_limbs  (line 386, static)
- [x] mpn_copyi  (line 395)
- [x] mpn_copyd  (line 403)
- [x] mpn_cmp  (line 410)
- [x] mpn_cmp4  (line 421, static)
- [x] mpn_normalized_size  (line 430, static)
- [x] mpn_zero_p  (line 438)
- [x] mpn_zero  (line 444)
- [x] mpn_add_1  (line 451)
- [x] mpn_add_n  (line 470)
- [x] mpn_add  (line 489)
- [x] mpn_sub_1  (line 502)
- [x] mpn_sub_n  (line 523)
- [x] mpn_sub  (line 541)
- [x] mpn_mul_1  (line 554)
- [x] mpn_addmul_1  (line 577)
- [x] mpn_submul_1  (line 603)
- [x] mpn_mul  (line 629)
- [x] mpn_mul_n  (line 654)
- [x] mpn_sqr  (line 660)
- [x] mpn_lshift  (line 666)
- [x] mpn_rshift  (line 696)
- [x] mpn_common_scan  (line 723, static)
- [x] mpn_scan1  (line 743)
- [x] mpn_scan0  (line 753)
- [x] mpn_com  (line 763)
- [x] mpn_neg  (line 770)
- [x] mpn_invert_3by2  (line 792)
- [x] mpn_div_qr_1_invert  (line 911, static)
- [x] mpn_div_qr_2_invert  (line 923, static)
- [x] mpn_div_qr_invert  (line 942, static)
- [x] mpn_div_qr_1_preinv  (line 975, static)
- [x] mpn_div_qr_2_preinv  (line 1015, static)
- [x] mpn_div_qr_pi1  (line 1059, static)
- [x] mpn_div_qr_preinv  (line 1122, static)
- [x] mpn_div_qr  (line 1156, static)
- [x] mpn_base_power_of_two_p  (line 1179, static)
- [x] mpn_get_base_info  (line 1204, static)
- [x] mpn_limb_size_in_base_2  (line 1219, static)
- [x] mpn_get_str_bits  (line 1229, static)
- [x] mpn_limb_get_str  (line 1260, static)
- [x] mpn_get_str_other  (line 1281, static)
- [x] mpn_get_str  (line 1325)
- [x] mpn_set_str_bits  (line 1345, static)
- [x] mpn_set_str_other  (line 1375, static)
- [x] mpn_set_str  (line 1413)
- [x] mpz_init  (line 1435)
- [x] mpz_init2  (line 1447)
- [x] mpz_clear  (line 1460)
- [x] mpz_realloc  (line 1467, static)
- [x] mpz_set_si  (line 1490)
- [x] mpz_set_ui  (line 1508)
- [x] mpz_set  (line 1529)
- [x] mpz_init_set_si  (line 1546)
- [x] mpz_init_set_ui  (line 1553)
- [x] mpz_init_set  (line 1560)
- [x] mpz_fits_slong_p  (line 1567)
- [x] mpn_absfits_ulong_p  (line 1573, static)
- [x] mpz_fits_ulong_p  (line 1585)
- [x] mpz_fits_sint_p  (line 1593)
- [x] mpz_fits_uint_p  (line 1599)
- [x] mpz_fits_sshort_p  (line 1605)
- [x] mpz_fits_ushort_p  (line 1611)
- [x] mpz_get_si  (line 1617)
- [x] mpz_get_ui  (line 1630)
- [x] mpz_size  (line 1647)
- [x] mpz_getlimbn  (line 1653)
- [x] mpz_realloc2  (line 1662)
- [x] mpz_limbs_read  (line 1668)
- [x] mpz_limbs_modify  (line 1674)
- [x] mpz_limbs_write  (line 1681)
- [x] mpz_limbs_finish  (line 1687)
- [x] mpz_roinit_normal_n  (line 1695, static)
- [x] mpz_roinit_n  (line 1704)
- [x] mpz_set_d  (line 1715)
- [x] mpz_init_set_d  (line 1766)
- [x] mpz_get_d  (line 1773)
- [x] mpz_cmpabs_d  (line 1811)
- [x] mpz_cmp_d  (line 1852)
- [x] mpz_sgn  (line 1874)
- [x] mpz_cmp_si  (line 1880)
- [x] mpz_cmp_ui  (line 1893)
- [x] mpz_cmp  (line 1904)
- [x] mpz_cmpabs_ui  (line 1918)
- [x] mpz_cmpabs  (line 1932)
- [x] mpz_abs  (line 1939)
- [x] mpz_neg  (line 1946)
- [x] mpz_swap  (line 1953)
- [x] mpz_add_ui  (line 1964)
- [x] mpz_sub_ui  (line 1973)
- [x] mpz_ui_sub  (line 1980)
- [x] mpz_abs_add  (line 1987, static)
- [x] mpz_abs_sub  (line 2009, static)
- [x] mpz_add  (line 2034)
- [x] mpz_sub  (line 2047)
- [x] mpz_mul_si  (line 2062)
- [x] mpz_mul_ui  (line 2074)
- [x] mpz_mul  (line 2084)
- [x] mpz_mul_2exp  (line 2122)
- [x] mpz_addmul_ui  (line 2156)
- [x] mpz_submul_ui  (line 2166)
- [x] mpz_addmul  (line 2176)
- [x] mpz_submul  (line 2186)
- [x] mpz_div_qr  (line 2201, static)
- [x] mpz_cdiv_qr  (line 2313)
- [x] mpz_fdiv_qr  (line 2319)
- [x] mpz_tdiv_qr  (line 2325)
- [x] mpz_cdiv_q  (line 2331)
- [x] mpz_fdiv_q  (line 2337)
- [x] mpz_tdiv_q  (line 2343)
- [x] mpz_cdiv_r  (line 2349)
- [x] mpz_fdiv_r  (line 2355)
- [x] mpz_tdiv_r  (line 2361)
- [x] mpz_mod  (line 2367)
- [x] mpz_div_q_2exp  (line 2373, static)
- [x] mpz_div_r_2exp  (line 2427, static)
- [x] mpz_cdiv_q_2exp  (line 2497)
- [x] mpz_fdiv_q_2exp  (line 2503)
- [x] mpz_tdiv_q_2exp  (line 2509)
- [x] mpz_cdiv_r_2exp  (line 2515)
- [x] mpz_fdiv_r_2exp  (line 2521)
- [x] mpz_tdiv_r_2exp  (line 2527)
- [x] mpz_divexact  (line 2533)
- [x] mpz_divisible_p  (line 2539)
- [x] mpz_congruent_p  (line 2545)
- [x] mpz_div_qr_ui  (line 2563, static)
- [x] mpz_cdiv_qr_ui  (line 2583)
- [x] mpz_fdiv_qr_ui  (line 2589)
- [x] mpz_tdiv_qr_ui  (line 2595)
- [x] mpz_cdiv_q_ui  (line 2601)
- [x] mpz_fdiv_q_ui  (line 2607)
- [x] mpz_tdiv_q_ui  (line 2613)
- [x] mpz_cdiv_r_ui  (line 2619)
- [x] mpz_fdiv_r_ui  (line 2624)
- [x] mpz_tdiv_r_ui  (line 2629)
- [x] mpz_cdiv_ui  (line 2635)
- [x] mpz_fdiv_ui  (line 2641)
- [x] mpz_tdiv_ui  (line 2647)
- [x] mpz_mod_ui  (line 2653)
- [x] mpz_divexact_ui  (line 2659)
- [x] mpz_divisible_ui_p  (line 2665)
- [x] mpn_gcd_11  (line 2673, static)
- [x] mpn_gcd  (line 2716)
- [x] mpz_gcd_ui  (line 2776)
- [x] mpz_make_odd  (line 2793, static)
- [x] mpz_gcd  (line 2806)
- [x] mpz_gcdext  (line 2842)
- [x] mpz_lcm  (line 3039)
- [x] mpz_lcm_ui  (line 3060)
- [x] mpz_invert  (line 3075)
- [x] mpz_pow_ui  (line 3110)
- [x] mpz_ui_pow_ui  (line 3131)
- [x] mpz_powm  (line 3141)
- [x] mpz_powm_ui  (line 3246)
- [x] mpz_rootrem  (line 3257)
- [x] mpz_root  (line 3318)
- [x] mpz_sqrtrem  (line 3333)
- [x] mpz_sqrt  (line 3339)
- [x] mpz_perfect_square_root  (line 3345)
- [x] mpz_perfect_square_p  (line 3359)
- [x] mpn_perfect_square_p  (line 3365)
- [x] mpn_sqrtrem  (line 3375)
- [x] mpz_mfac_uiui  (line 3400)
- [x] mpz_2fac_ui  (line 3409)
- [x] mpz_fac_ui  (line 3415)
- [x] mpz_bin_uiui  (line 3421)
- [x] gmp_jacobi_coprime  (line 3446, static)
- [x] gmp_lucas_step_k_2k  (line 3486, static)
- [x] gmp_lucas_mod  (line 3502, static)
- [x] gmp_stronglucas  (line 3560, static)
- [x] gmp_millerrabin  (line 3614, static)
- [x] mpz_probab_prime_p  (line 3642)
- [x] mpz_tstbit  (line 3734)
- [x] mpz_abs_add_bit  (line 3767, static)
- [x] mpz_abs_sub_bit  (line 3808, static)
- [x] mpz_setbit  (line 3829)
- [x] mpz_clrbit  (line 3841)
- [x] mpz_combit  (line 3853)
- [x] mpz_init_setbit  (line 3862)
- [x] mpz_com  (line 3870)
- [x] mpz_and  (line 3877)
- [x] mpz_ior  (line 3949)
- [x] mpz_xor  (line 4022)
- [x] gmp_popcount_limb  (line 4091, static)
- [x] mpn_popcount  (line 4113)
- [x] mpz_popcount  (line 4125)
- [x] mpz_hamdist  (line 4138)
- [x] mpz_scan1  (line 4189)
- [x] mpz_scan0  (line 4225)
- [x] mpz_sizeinbase  (line 4257)
- [x] mpz_get_str  (line 4311)
- [x] mpz_set_str  (line 4386)
- [x] mpz_init_set_str  (line 4492)
- [x] mpz_out_str  (line 4499)
- [x] gmp_detect_endian  (line 4515, static)
- [x] mpz_import  (line 4524)
- [x] mpz_export  (line 4597)

## Macros defined — 39
- [x] GMP_ABS  (line 75, body 7de42247ced6)
- [x] GMP_CMP  (line 81, body ec34ed0ec5eb)
- [x] GMP_DBL_MANT_BITS  (line 86, body 8e5fc939a076)
- [x] GMP_HLIMB_BIT  (line 69, body b0cfee7437d9)
- [x] GMP_LIMB_BITS  (line 64, body 95216f7e3789)
- [x] GMP_LIMB_HIGHBIT  (line 67, body 305285e26375)
- [x] GMP_LIMB_MAX  (line 66, body 28a2f6c9ec1f)
- [x] GMP_LLIMB_MASK  (line 70, body e05982afb31a)
- [x] GMP_MAX  (line 79, body 54a3705d26d6)
- [x] GMP_MIN  (line 78, body 7c03c63086ce)
- [x] GMP_MPN_OVERLAP_P  (line 92, body 2a57b3be5605)
- [x] GMP_NEG_CAST  (line 76, body 2a6971fcd109)
- [x] GMP_PRIME_MASK  (line 3639, body df9ee5694a79)
- [x] GMP_PRIME_PRODUCT  (line 3635, body 68cfc7bf5011)
- [x] GMP_ULONG_BITS  (line 72, body ef6b9aba7450)
- [x] GMP_ULONG_HIGHBIT  (line 73, body 278c0cb154e9)
- [x] MINI_GMP_ENABLE_FLOAT  (line 55, body 356a192b7913)
- [x] MPN_PTR_SWAP  (line 264, body 76c37f467595)
- [x] MPN_SRCPTR_SWAP  (line 269, body 6ec246c760e2)
- [x] MPZ_PTR_SWAP  (line 275, body f31bce0320ca)
- [x] MPZ_REALLOC  (line 1484, body 3e0a05e4a229)
- [x] MPZ_SRCPTR_SWAP  (line 281, body 6888d32d703e)
- [x] MP_BITCNT_T_SWAP  (line 245, body 4c75683bf68b)
- [x] MP_LIMB_T_SWAP  (line 233, body 6fb20034c322)
- [x] MP_PTR_SWAP  (line 251, body 400221c57bf3)
- [x] MP_SIZE_T_SWAP  (line 239, body 8b5af5e720ce)
- [x] MP_SRCPTR_SWAP  (line 257, body 6d7b496dcba3)
- [x] gmp_add_ssaaaa  (line 122, body 14fe83886431)
- [x] gmp_alloc  (line 368, body baa46d2b7c2f)
- [x] gmp_assert_nocarry  (line 95, body a61005163783)
- [x] gmp_clz  (line 101, body 892d746ce557)
- [x] gmp_ctz  (line 115, body dde8992ef0e5)
- [x] gmp_free  (line 369, body 564bb4bddfda)
- [x] gmp_realloc  (line 370, body 1598a03bf1b7)
- [x] gmp_sub_ddmmss  (line 130, body 100ce69ea049)
- [x] gmp_udiv_qr_3by2  (line 205, body 88ebe222a944)
- [x] gmp_udiv_qrnnd_preinv  (line 186, body a7c4c8b6dc10)
- [x] gmp_umul_ppmm  (line 138, body 539b150bf61e)
- [x] gmp_umullo_limb  (line 183, body 1cfc19e98ce2)

## Types and their fields — 2
- [x] gmp_div_inverse: unsigned shift, mp_limb_t d1, mp_limb_t d0, mp_limb_t di
- [x] mpn_base_info: unsigned exp, mp_limb_t bb

## File-scope objects — 1
- [x] const int mp_bits_per_limb  (line 288)

## Decisions

`emacs/lib/mini-gmp.c` @ `fd7afced403`; port `lib/mini-gmp.rs` (5,277 lines).
All 247 checklist items are ticked: every one of the 205 functions, the 39 macros, the two
structs and `mp_bits_per_limb` exist in the port with the reference's names and order.

- **Evidence.** `xtask build-tu lib/mini-gmp.c` → `built lib/mini-gmp.rs -> build/femacs/mini-gmp.o`
  (0 errors; the unit is `--crate-type=lib --emit=obj`, so nothing is linked here);
  `python3 tools/extract-abi.py --check emacs/lib/mini-gmp.c lib/mini-gmp.rs` → `OK`.
  Not yet run (the batch owner runs them): `gate-fast`, `spikes/gmp-abi-probe.sh`, ERT, and any
  behavior-level differential — no link test of the object has happened either.
- **`assert` → `assert!`.** The frozen build leaves `NDEBUG` undefined, so C's 98 assertions are
  live; Rust's `assert!` aborts the same way (`-C panic=abort`). No `debug_assert!`.
- **`mpz_t`/`mpz_ptr`/`mpz_srcptr` parameters are the mirror's pointers** (P-49): C's
  `r->_mp_size` is `(*r)._mp_size`, a local `mpz_t` passes as `x.as_mut_ptr ()`/`x.as_ptr ()`,
  and `NULL`-able out-parameters (`mpz_cdiv_q`'s `r`, `mpz_export`'s `r`, `mpz_invert`'s `s`/`t`)
  are raw pointers checked with `is_null ()` exactly where C tests the pointer.
- **Carry arithmetic is `wrapping_*`** (PORTING.md 1.1): every `+`, `-`, `*`, `~`, unary `-` of
  `mp_limb_t`/`c_ulong`/`c_uint` in the mpn layer is spelled `wrapping_add`/`wrapping_sub`/
  `wrapping_mul`/`wrapping_neg` so C's wrap-around stays wrap-around under Rust's debug checks.
  Two **dead** arms only have to compile (`gmp_umul_ppmm`'s 32-bit arm, `mpz_get_ui`'s
  `GMP_LIMB_BITS < GMP_ULONG_BITS` arm): they use `wrapping_shr`/`wrapping_shl`, since a shift by
  this target's 64 is not a Rust expression.
- **`switch` → `match`, `goto` → labelled block** (PORTING.md 2.2): `mpn_base_power_of_two_p` and
  `mpz_sizeinbase` are the only switches; `mpz_get_str`'s `goto ret` is the labelled block `'ret`
  whose `break 'ret` is C's forward jump (the trailing `sp[sn] = 0` still runs, unlike an early
  `return`).
- **`double` is spelled `c_double`** (the `bignum.rs`/`floatfns.rs` precedent), and `<float.h>`'s
  `DBL_MANT_DIG` comes from `lib/include/float.in.rs`.
- **`zero_mpz ()`** is the port's spelling of C's uninitialized `mpz_t x;` local (PORTING.md 1.5:
  Rust needs a value before `as_mut_ptr ()` can be taken). It is the one helper this unit adds;
  every such local is `mpz_init`ed exactly where C inits it. Likewise `let mut x = x;` rebinds the
  parameters C assigns to (`np`, `dp`, `sp`, `rp`, `up`, `vp`, `u`, `v`, `s`, `t`, `alloc_func`, …).
- **System headers.** `malloc`/`realloc`/`free`/`abort`/`strlen`/`isspace`/`fprintf`/`fwrite` are
  inline `unsafe extern "C"` declarations (P-26/P-50, the `malloc.rs`/`mempcpy.rs` precedent),
  because the body needs one `gmp_die` message and one `fwrite`; `stderr` keeps C's name and binds
  the platform symbol (`__stderrp` on darwin). `CHAR_BIT` (11 uses) and the `UINT_MAX`/`SHRT_MAX`/
  `SHRT_MIN`/`USHRT_MAX` of the fits/compare family were added to
  `lib/include/limits.in.rs`'s system-`<limits.h>` section.
- **Ledgers touched, all recorded with reasons** (the token check is one-directional, so a *named*
  token is the deviation and everything else stays compared):
  - `docs/ports/macro-body-notes.tsv`: the twelve `do { … } while (0)` macros need `0` (no Rust
    do-while; the `byte_char_debug_check` precedent); `MP_SIZE_T_SWAP` additionally needs
    `mp_size_t` because `mpz_swap` swaps `int` fields through it; `GMP_PRIME_MASK` needs
    `xc96996dcul` (C's `0xc96996dcUL` is one token; Rust has no `UL` suffix — the `__memrchr`
    `0xffffffffU` precedent); `mpn_base_power_of_two_p`/`mpz_sizeinbase` need
    `switch`/`case`/`default` (C's switches are `match`es); `mpz_get_str` needs `goto`.
  - `docs/ports/line-drift-tolerance.tsv`: `mini-gmp 0.15` (C 4,699 vs the port's 5,277, 12.3%) —
    the causes are the explicit casts P-1.1/1.5 require, `as_mut_ptr ()`/`as_ptr ()` on every
    `mpz_t` argument (P-49), the `let mut x = x;` rebindings, `zero_mpz ()` locals, and
    `match`/labelled-block spellings.
- **One tool fix outside the port**: `xtask`'s `build-tu` only looked for `src/<stem>.rs`, so the
  documented `build-tu lib/mini-gmp.c` could not build any gnulib unit (it answered
  `src/mini-gmp.rs does not exist yet`). It now looks the unit up in `["src", "lib"]` exactly as
  `finished_units` already did.
