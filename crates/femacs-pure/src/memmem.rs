#![forbid(unsafe_code)]
//! Pure Safe Rust Two-Way (`O(N + M)` time, `O(1)` space) substring search (`lib/memmem.c`).

/// Compute maximal lexicographical suffix and its period under normal (`invert = false`)
/// or inverted (`invert = true`) byte ordering (Crochemore-Perrin critical factorization).
fn critical_factorization_half(needle: &[u8], invert: bool) -> (usize, usize) {
    let n = needle.len();
    let mut max_suffix = usize::MAX;
    let mut j = 0usize;
    let mut k = 1usize;
    let mut p = 1usize;

    while j + k < n {
        let a = needle[j + k];
        let b = needle[max_suffix.wrapping_add(k)];
        let less = if invert { b < a } else { a < b };
        if less {
            j += k;
            k = 1;
            p = j.wrapping_sub(max_suffix);
        } else if a == b {
            if k != p {
                k += 1;
            } else {
                j += p;
                k = 1;
            }
        } else {
            max_suffix = j;
            j += 1;
            k = 1;
            p = 1;
        }
    }
    (max_suffix, p)
}

#[inline]
fn critical_factorization(needle: &[u8]) -> (usize, usize) {
    let (ms1, p1) = critical_factorization_half(needle, false);
    let (ms2, p2) = critical_factorization_half(needle, true);
    // `usize::MAX` represents `-1`; `wrapping_add(1)` maps `-1 -> 0` for proper comparison.
    if ms1.wrapping_add(1) >= ms2.wrapping_add(1) {
        (ms1.wrapping_add(1), p1)
    } else {
        (ms2.wrapping_add(1), p2)
    }
}

/// Find the first occurrence of `needle` in `haystack`.
/// Returns `Some(0)` when `needle` is empty (matching C `memmem` semantics).
pub fn memmem_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let n_len = needle.len();
    let h_len = haystack.len();
    if n_len == 0 {
        return Some(0);
    }
    if h_len < n_len {
        return None;
    }
    let first = needle[0];
    if n_len == 1 {
        return haystack.iter().position(|&b| b == first);
    }

    let (suffix, mut period) = critical_factorization(needle);
    let periodic = needle[..suffix] == needle[period..period + suffix];

    if periodic {
        let mut j = 0usize;
        let mut memory = 0usize;
        while j <= h_len - n_len {
            let mut i = suffix.max(memory);
            while i < n_len && needle[i] == haystack[j + i] {
                i += 1;
            }
            if i < n_len {
                j += i - suffix + 1;
                memory = 0;
                continue;
            }
            let mut left = suffix;
            while left > memory && needle[left - 1] == haystack[j + left - 1] {
                left -= 1;
            }
            if left <= memory {
                return Some(j);
            }
            j += period;
            memory = n_len - period;
        }
        None
    } else {
        period = suffix.max(n_len - suffix) + 1;
        let mut j = 0usize;
        while j <= h_len - n_len {
            if suffix == 0 {
                if let Some(rel) = haystack[j..=h_len - n_len].iter().position(|&b| b == first) {
                    j += rel;
                } else {
                    return None;
                }
            }
            let mut i = suffix;
            while i < n_len && needle[i] == haystack[j + i] {
                i += 1;
            }
            if i < n_len {
                j += i - suffix + 1;
                continue;
            }
            let mut left = suffix;
            while left > 0 && needle[left - 1] == haystack[j + left - 1] {
                left -= 1;
            }
            if left == 0 {
                return Some(j);
            }
            j += period;
        }
        None
    }
}

/// Find the last occurrence of `byte` in `haystack` (`lib/memrchr.c`).
#[inline]
pub fn memrchr_find(haystack: &[u8], byte: u8) -> Option<usize> {
    haystack.iter().rposition(|&b| b == byte)
}

/// Find the first occurrence of `byte` in `haystack` (`lib/rawmemchr.c` safe slice form).
#[inline]
pub fn rawmemchr_find(haystack: &[u8], byte: u8) -> Option<usize> {
    haystack.iter().position(|&b| b == byte)
}
