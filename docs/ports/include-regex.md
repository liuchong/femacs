# emacs/lib/regex.h (684 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 78
- [x] REGS_FIXED  (line 455, body da4b9237bacc)
- [x] REGS_REALLOCATE  (line 454, body 356a192b7913)
- [x] REGS_UNALLOCATED  (line 453, body b6589fc6ab0d)
- [x] REG_BADBR  (line 386, body 4767f93571c0)
- [x] REG_BADPAT  (line 378, body c46c1bf16b09)
- [x] REG_BADRPT  (line 389, body 6fcdd2d9f49b)
- [x] REG_EBRACE  (line 385, body f6051967d340)
- [x] REG_EBRACK  (line 383, body b72af49479f1)
- [x] REG_ECOLLATE  (line 379, body a11b487ebcfe)
- [x] REG_ECTYPE  (line 380, body c8b57771261c)
- [x] REG_EEND  (line 390, body 3d1d72ed9653)
- [x] REG_EESCAPE  (line 381, body b50b36cf9a2d)
- [x] REG_ENOSYS  (line 374, body 8fd43c15b481)
- [x] REG_EPAREN  (line 384, body 17dc2ee00d4a)
- [x] REG_ERANGE  (line 387, body 7b4a35faf3ba)
- [x] REG_ERPAREN  (line 392, body d4faecff9bd0)
- [x] REG_ESIZE  (line 391, body 94d5815aed22)
- [x] REG_ESPACE  (line 388, body 4261af5cd5e8)
- [x] REG_ESUBREG  (line 382, body b25f4db51f57)
- [x] REG_EXTENDED  (line 310, body 356a192b7913)
- [x] REG_ICASE  (line 314, body 8ef58e36184c)
- [x] REG_NEWLINE  (line 319, body 866deb7d5e44)
- [x] REG_NOERROR  (line 376, body ed95a89ecbc5)
- [x] REG_NOMATCH  (line 377, body 5c12e2d0030f)
- [x] REG_NOSUB  (line 323, body 552fdd497e31)
- [x] REG_NOTBOL  (line 333, body 356a192b7913)
- [x] REG_NOTEOL  (line 336, body 8ef58e36184c)
- [x] REG_STARTEND  (line 340, body 866deb7d5e44)
- [x] RE_BACKSLASH_ESCAPE_IN_LISTS  (line 76, body 8e4f9543bb12)
- [x] RE_BK_PLUS_QM  (line 81, body aba89b743255)
- [x] RE_CARET_ANCHORS_HERE  (line 195, body c4f47a7013c9)
- [x] RE_CHAR_CLASSES  (line 87, body f880385e9650)
- [x] RE_CONTEXT_INDEP_ANCHORS  (line 101, body 18bd113ec3ae)
- [x] RE_CONTEXT_INDEP_OPS  (line 109, body 61e52071bc96)
- [x] RE_CONTEXT_INVALID_DUP  (line 199, body ac94794ffb14)
- [x] RE_CONTEXT_INVALID_OPS  (line 113, body 450ccab36d86)
- [x] RE_DEBUG  (line 181, body 4cfd3f2f335f)
- [x] RE_DOT_NEWLINE  (line 117, body 3297714f290b)
- [x] RE_DOT_NOT_NULL  (line 121, body 97cfe596b587)
- [x] RE_DUP_MAX  (line 302, body d88192662cc6)
- [x] RE_HAT_LISTS_NOT_NEWLINE  (line 125, body 5f4a883f3a74)
- [x] RE_ICASE  (line 190, body f08aa149b098)
- [x] RE_INTERVALS  (line 130, body 918d92f5ce9f)
- [x] RE_INVALID_INTERVAL_ORD  (line 186, body e3af2708d3a3)
- [x] RE_LIMITED_OPS  (line 134, body 052e669b9acd)
- [x] RE_NEWLINE_ALT  (line 138, body be930c185369)
- [x] RE_NO_BK_BRACES  (line 143, body 2211c43f7a48)
- [x] RE_NO_BK_PARENS  (line 147, body 38933439512e)
- [x] RE_NO_BK_REFS  (line 151, body 946dfe62fda1)
- [x] RE_NO_BK_VBAR  (line 155, body c6eadc226274)
- [x] RE_NO_EMPTY_RANGES  (line 161, body 5c503d4cb6e7)
- [x] RE_NO_GNU_OPS  (line 173, body 2ba7b909b477)
- [x] RE_NO_POSIX_BACKTRACKING  (line 169, body f939d0a737d1)
- [x] RE_NO_SUB  (line 203, body 6e8c43849989)
- [x] RE_NREGS  (line 509, body 22d200f8670d)
- [x] RE_SYNTAX_AWK  (line 220, body 36586d8b64ad)
- [x] RE_SYNTAX_ED  (line 252, body c926ea503960)
- [x] RE_SYNTAX_EGREP  (line 243, body 49b9b79e7c86)
- [x] RE_SYNTAX_EMACS  (line 217, body 435bcca4cff0)
- [x] RE_SYNTAX_GNU_AWK  (line 228, body 073b3a750090)
- [x] RE_SYNTAX_GREP  (line 239, body f467ab5573ec)
- [x] RE_SYNTAX_POSIX_AWK  (line 234, body 8403e9e17ad2)
- [x] RE_SYNTAX_POSIX_BASIC  (line 261, body 9fecfdb878c3)
- [x] RE_SYNTAX_POSIX_EGREP  (line 248, body 6dafbd663c17)
- [x] RE_SYNTAX_POSIX_EXTENDED  (line 270, body 6213f2f590dc)
- [x] RE_SYNTAX_POSIX_MINIMAL_BASIC  (line 267, body d1c8776d01f2)
- [x] RE_SYNTAX_POSIX_MINIMAL_EXTENDED  (line 278, body 3b8e492869f5)
- [x] RE_SYNTAX_SED  (line 254, body c926ea503960)
- [x] RE_TRANSLATE_TYPE  (line 403, body 27b8dcaf40b1)
- [x] RE_UNMATCHED_RIGHT_PAREN_ORD  (line 165, body 89466b98b4f0)
- [x] _Attr_access_  (line 531, body da39a3ee5e6b)
- [x] _REGEX_H  (line 21, body 356a192b7913)
- [x] _RE_SYNTAX_POSIX_COMMON  (line 257, body 90629d5cf3de)
- [x] _Restrict_  (line 640, body da39a3ee5e6b)
- [x] _Restrict_arr_  (line 659, body da39a3ee5e6b)
- [x] __REPB_PREFIX  (line 410, body 7b9b6d0dcab3)
- [x] __RE_TRANSLATE_TYPE  (line 401, body f49d92be99f6)
- [x] __USE_GNU  (line 33, body 356a192b7913)

## Types and their fields — 2
- [x] re_pattern_buffer: size_t re_nsub
- [x] re_registers: __re_size_t num_regs, regoff_t* start, regoff_t* end

## File-scope objects — 1
- [x] extern reg_syntax_t re_syntax_options  (line 210)


## Decisions

`emacs/lib/regex.h` @ `fd7afced403`; port `femacs/lib/include/regex.rs`.

The mirror now checks **OK**: the one difference that kept it WIP was regex.h 76's
`# define RE_BACKSLASH_ESCAPE_IN_LISTS 1ul`, whose decimal literal carries C's `ul` suffix.
`tools/extract-abi.py`'s `norm_const` now strips an integer type suffix and compares the
decimal value, so Rust's bare `1` is the same value (this also subsumes the per-macro notes
rows the hex spellings needed).  The four `#if`-duplicated attribute macros
(`_Attr_access_`, `_Restrict_`, `_Restrict_arr_`, `__REPB_PREFIX`) are configuration branches
and are not body-compared, per the extractor's existing rule.
