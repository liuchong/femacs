#!/bin/bash
# The gate for `src/include/jni.rs`, the port's mirror of the Android NDK's <jni.h>
# (a system header of *another* target's toolchain, so - like `math.rs`/`limits.in.rs` - it has
# no reference file to be paired with; spikes/syshdrs-probe.sh is the frozen target's analogue).
#
# Three things must hold, and each is checked here:
#   1. the mirror's `JNINativeInterface` field list is jni.h's own order (nothing added, dropped
#      or reordered) - the layout is that order;
#   2. the NDK's own clang agrees with every offset the mirror asserts, compiled for the real
#      target (`--target=aarch64-linux-android24`), by `_Static_assert`-ing the same numbers;
#   3. the Rust side compiles the entry table with those `offset_of!` asserts (the pinned
#      toolchain, which is what the `c_variadic` entry types need).
#
#     bash spikes/jni-probe.sh
set -uo pipefail
cd "$(dirname "$0")/.."
MIRROR=src/include/jni.rs
NDK="${ANDROID_NDK_ROOT:-$HOME/Library/Android/sdk/ndk/29.0.14206865}"
JNI_H="$NDK/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/jni.h"
OUT=build/jni-probe
mkdir -p "$OUT"

if [ ! -f "$JNI_H" ]; then
  echo "jni-probe: $JNI_H is missing; set ANDROID_NDK_ROOT to an installed NDK (the mirror was"
  echo "measured against 29.0.14206865).  Nothing was checked."
  exit 2
fi

python3 - "$JNI_H" "$MIRROR" "$OUT/verify.c" <<'PY' || exit 1
import re, sys
jni_h, mirror, out = sys.argv[1], sys.argv[2], sys.argv[3]
src = open(jni_h).read().split('\n')
start = next(i for i, l in enumerate(src) if l.startswith('struct JNINativeInterface'))
end = next(i for i, l in enumerate(src[start + 1:], start + 1) if l.startswith('};'))
body = re.sub(r'/\*.*?\*/', ' ', '\n'.join(src[start + 1:end]), flags=re.S)
decls = [' '.join(d.split()) for d in body.split(';') if ' '.join(d.split())]
names = []
for d in decls:
    m = re.match(r'^[A-Za-z_][\w ]*\**\s*\(\s*\*\s*([A-Za-z_]\w*)\s*\)', d)
    if m:
        names.append(m.group(1)); continue
    m2 = re.match(r'^void\s*\*\s*(reserved\d+)$', d)
    if m2:
        names.append(m2.group(1)); continue
    sys.exit('jni-probe: unparsed jni.h field: ' + d)
rust = open(mirror).read()
block = rust.split('pub struct JNINativeInterface', 1)[1]
block = block.split('\n}', 1)[0]
fields = re.findall(r'^  pub ([A-Za-z_]\w*):', block, re.M)
if names != fields:
    sys.exit('jni-probe: the mirror\'s field order differs from jni.h\n  jni.h:  %s\n  mirror: %s'
             % (', '.join(names[:6]), ', '.join(fields[:6])))
lines = ['#include <jni.h>', '#include <stddef.h>']
for i, n in enumerate(names):
    lines.append('_Static_assert (offsetof (struct JNINativeInterface, %s) == %d, "%s");'
                 % (n, 8 * i, n))
lines.append('_Static_assert (sizeof (struct JNINativeInterface) == %d, "size");' % (8 * len(names)))
lines.append('int main (void) { return 0; }')
open(out, 'w').write('\n'.join(lines) + '\n')
print('jni-probe: %d entries, jni.h order matches the mirror' % len(names))
PY

"$NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang" --target=aarch64-linux-android24 \
  --sysroot="$NDK/toolchains/llvm/prebuilt/darwin-x86_64/sysroot" \
  -c "$OUT/verify.c" -o "$OUT/verify.o" 2> "$OUT/c.err" \
  || { echo "jni-probe: the target's clang rejects an offset the mirror asserts:"; head -20 "$OUT/c.err"; exit 1; }
echo "jni-probe: the NDK's clang confirms all $(grep -c _Static_assert "$OUT/verify.c") assertions (aarch64-linux-android24)"

cat > "$OUT/rs.rs" <<RS
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables)]
use std::ffi::{c_char, c_int, c_void};
include!("$(cd src/include && pwd)/jni.rs");
fn main () {}
RS
(cd "$HERE" && cargo run -q -p xtask -- probe ../"$OUT"/rs.rs ../"$OUT"/rs) 2> "$OUT/rs.err" \
  || { echo "jni-probe: the Rust side failed to compile:"; head -20 "$OUT/rs.err"; exit 1; }
"$OUT/rs" > /dev/null || exit 1
echo "jni-probe: the mirror's offset_of!/size_of assertions hold (pinned toolchain, exit 0)"
