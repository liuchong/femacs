#!/usr/bin/env python3
"""Compare the port's header-mirror constants against the *delivery target's* headers.

Three defects of one class have already reached the device through this gap:

  * `RENAME_EXCL` and `_CS_PATH` - macros bionic does not have at all (status 4.749);
  * `O_CLOEXEC` - present on both platforms with *different* values (Darwin 0x1000000,
    bionic 02000000), which made `pipe2` fail with EINVAL and took C's
    `if (pipe2 (...) < 0) emacs_abort ()` arm (status 4.780).

`docs/baseline/config/*.json` cannot express this: those entries say whether a macro is
defined, not what it expands to.  This tool asks the target's own preprocessor for every
numeric constant the mirrors declare and reports the ones whose value differs, so the
"freeze one platform's header and ship it everywhere" mistake is caught by a reading instead
of by a tombstone.

    python3 tools/check-android-constants.py [--ndk CC]

Exit status is 0 when every comparable constant agrees, 1 when any differs.
"""

from __future__ import annotations

import argparse
import glob
import json
import pathlib
import re
import subprocess
import sys
import uuid

REPO = pathlib.Path(__file__).resolve().parent.parent
HEADERS = ["fcntl.h", "unistd.h", "signal.h", "errno.h", "stdio.h", "string.h", "stdlib.h",
           "sys/stat.h", "sys/types.h", "sys/socket.h", "sys/ioctl.h", "sys/mman.h",
           "sys/wait.h", "sys/resource.h", "sys/time.h", "time.h", "dirent.h", "termios.h",
           "pwd.h", "grp.h", "limits.h", "poll.h"]
CONST = re.compile(r"pub const (\w+): (?:c_int|c_long|c_uint|c_ulong) = ([^;]+);")
TYPES = ("c_int", "c_long", "c_uint", "c_ulong")
FILE_FLAGS = ('O_RDONLY','O_WRONLY','O_RDWR','O_ACCMODE','O_CREAT','O_TRUNC',
              'O_EXCL','O_APPEND','O_NONBLOCK','O_CLOEXEC','O_DIRECTORY',
              'O_NOFOLLOW','O_SYNC','O_DIRECT','O_DSYNC','O_EXEC','O_SEARCH',
              'O_NOATIME','O_NOCTTY','O_RSYNC','O_NDELAY','F_DUPFD_CLOEXEC',
              'F_DUPFD','F_GETFD','F_SETFD','F_GETFL','F_SETFL','FD_CLOEXEC',
              'AT_FDCWD','AT_SYMLINK_NOFOLLOW','AT_REMOVEDIR',
              'AT_SYMLINK_FOLLOW','AT_EACCESS','AT_NO_AUTOMOUNT')


def llvm_flag_values(text, names=FILE_FLAGS):
    if not re.search(r'^target datalayout = "e-',text,re.M):
        raise ValueError('file-flag measurement requires little-endian target data')
    values={}
    for line in text.splitlines():
        match=re.match(r'^@femacs_flag_(\w+)\s*=.*?\bconstant\s+(.*)',line)
        if not match:continue
        name,value=match.groups()
        if name not in names or name in values:raise ValueError('unexpected or duplicate file flag')
        integer=re.match(r'i64\s+(-?\d+)(?:,|$)',value)
        if integer:values[name]=int(integer.group(1));continue
        array=re.match(r'\[8 x i8\]\s+(zeroinitializer|c"((?:\\[0-9A-Fa-f]{2}|[^"\\])*)")',value)
        if not array:raise ValueError('unrecognized file-flag data')
        data=bytearray()
        if array.group(1)=='zeroinitializer':data.extend(bytes(8))
        else:
            literal=array.group(2);offset=0
            while offset<len(literal):
                if literal[offset]=='\\':data.append(int(literal[offset+1:offset+3],16));offset+=3
                else:data.append(ord(literal[offset]));offset+=1
        if len(data)!=8:raise ValueError('file-flag integer has the wrong width')
        values[name]=int.from_bytes(data,'little',signed=True)
    if set(values)!=set(names):raise ValueError('incomplete mandatory file-flag family')
    return values


def measure_file_flags(cc):
    """Compare the actually compiled Rust target arm, including duplicate names."""
    out=REPO/'build/android-constant-measurements'/uuid.uuid4().hex
    out.mkdir(parents=True,exist_ok=False)
    c='#include <fcntl.h>\n'
    # These two missing-system-macro fallbacks are the original gnulib template.
    for name in ('O_EXEC','O_SEARCH'):
        c+=f'#ifndef {name}\n#define {name} O_RDONLY\n#endif\n'
    c+=''.join(f'const long femacs_flag_{name} = (long)({name});\n' for name in FILE_FLAGS)
    rust='#![allow(dead_code,non_camel_case_types,unused_imports,unused_unsafe,unsafe_op_in_unsafe_fn)]\n'
    rust+='use core::ffi::*;\ninclude!('+json.dumps(str(REPO/'lib/include/fcntl.in.rs'))+');\n'
    rust+=''.join(f'#[used] #[unsafe(no_mangle)] pub static femacs_flag_{name}: i64 = {name} as i64;\n' for name in FILE_FLAGS)
    (out/'flags.c').write_text(c);(out/'flags.rs').write_text(rust)
    commands=[('c',[cc,'--target=aarch64-linux-android21','-S','-emit-llvm','-x','c',str(out/'flags.c'),'-o',str(out/'c.ll')]),
              ('rust',['rustc','--crate-name','android_file_flags','--edition=2024','--crate-type','lib','--target','aarch64-linux-android','--cfg','feature="have_android"','--emit=llvm-ir',str(out/'flags.rs'),'-o',str(out/'rust.ll')])]
    record={'schema':'femacs-android-file-flags-v1','passed':False,'commands':[]}
    try:
        for label,argv in commands:
            with (out/(label+'.stdout')).open('xb') as stdout,(out/(label+'.stderr')).open('xb') as stderr:
                result=subprocess.run(argv,cwd=REPO,stdout=stdout,stderr=stderr)
            record['commands'].append({'tool':label,'exit_code':result.returncode})
            if result.returncode:raise RuntimeError(label+' file-flag compiler failed; see '+str(out))
        c_values=llvm_flag_values((out/'c.ll').read_text())
        rust_values=llvm_flag_values((out/'rust.ll').read_text())
        record.update(c_values=c_values,rust_values=rust_values)
        record['mismatches']={n:{'c':c_values[n],'rust':rust_values[n]} for n in FILE_FLAGS if c_values[n]!=rust_values[n]}
        record['passed']=not record['mismatches']
        print('mandatory file flags:',len(FILE_FLAGS),'measured,',len(record['mismatches']),'mismatches; evidence',out.relative_to(REPO))
        return record['passed']
    finally:(out/'receipt.json').write_text(json.dumps(record,indent=2)+'\n')


def find_cc(explicit: str | None) -> str:
    if explicit:
        return explicit
    found = sorted(glob.glob(str(pathlib.Path.home() /
                                "Library/Android/sdk/ndk/*/toolchains/llvm/prebuilt/*/bin/clang")))
    if not found:
        sys.exit("check-android-constants: no NDK clang found; pass --ndk CC")
    return found[-1]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ndk", default=None)
    ap.add_argument("--file-flags-only", action="store_true",
                    help="run the complete compiled file-control family without broad discovery")
    a = ap.parse_args()
    cc = find_cc(a.ndk)
    file_flags_passed = measure_file_flags(cc)
    if a.file_flags_only:
        return 0 if file_flags_passed else 1
    prelude = "\n".join(f"#include <{h}>" for h in HEADERS) + "\n"

    def expand(expr: str) -> int | None:
        out = subprocess.run([cc, "--target=aarch64-linux-android24", "-E", "-P", "-x", "c", "-"],
                             input=prelude + expr + "\n", capture_output=True, text=True,
                             errors="replace").stdout
        # Only a name whose whole expansion is one integer can be compared: anything else is
        # either not a macro at all (the identifier survives, `no_argument`) or expands to an
        # expression this tool does not evaluate.  The earlier "take the last numeric token"
        # form read unrelated numbers out of the header text and reported 1,149 false
        # mismatches (status 4.781).
        lines = [l.strip() for l in out.splitlines() if not l.startswith("#") and l.strip()]
        if not lines:
            return None
        last = lines[-1]
        if not re.fullmatch(r"[-+]?(0[xX][0-9a-fA-F]+|\d+)", last):
            return None
        try:
            return int(last, 0)
        except ValueError:
            return None

    consts: dict[str, set[tuple[str, str]]] = {}
    for path in (sorted(glob.glob(str(REPO / "lib/include/*.rs"))) +
                 sorted(glob.glob(str(REPO / "src/include/*.rs")))):
        for m in CONST.finditer(pathlib.Path(path).read_text(errors="replace")):
            consts.setdefault(m.group(1), set()).add(
                (pathlib.Path(path).name, m.group(2).strip()))

    compared = 0
    bad: list[tuple[str, int, int, str, str]] = []
    for name, sites in sorted(consts.items()):
        # A name declared more than once is a hand-written platform pair (`#[cfg (feature =
        # "have_android")]` beside the frozen arm); the tool cannot see which arm the target
        # compiles, so it leaves those to the cfg and reports only the single-declaration
        # names - the class `O_CLOEXEC` was in before it got its pair.
        if len(sites) != 1:
            continue
        target = expand(name)
        if target is None:
            continue
        compared += 1
        for site, expr in sorted(sites):
            try:
                port = int(eval(expr, {"__builtins__": {}}, {}))  # noqa: S307 - our own text
            except Exception:
                continue
            if port != target:
                bad.append((name, port, target, site, expr))

    for name, port, target, site, expr in bad:
        print(f"{name:28s} port={port:<12} target={target:<12} {site}  ({expr})")
    print(f"check-android-constants: {compared} constant(s) compared, {len(bad)} mismatch(es)")
    return 1 if bad or not file_flags_passed else 0


if __name__ == "__main__":
    raise SystemExit(main())
