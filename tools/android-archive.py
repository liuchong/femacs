#!/usr/bin/env python3
"""Generate APK archive members and measure actual DEX class definitions/asset identities.

Build sequencing, compiler choices and acceptance policy live in Rust xtask.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zipfile


def stream_digest(file):
    digest=hashlib.sha256()
    for block in iter(lambda:file.read(65536),b''):digest.update(block)
    return digest


def digest(path):
    with path.open('rb') as file:
        return stream_digest(file).hexdigest()


def class_names(data):
    if len(data) < 112 or data[:4] != b'dex\n':
        raise ValueError('invalid DEX header')
    def word(at):
        if at < 0 or at + 4 > len(data):
            raise ValueError('DEX offset outside file')
        return struct.unpack_from('<I', data, at)[0]
    strings_n, strings_off = word(56), word(60)
    types_n, types_off = word(64), word(68)
    defs_n, defs_off = word(96), word(100)
    if any(n > len(data) for n in (strings_n, types_n, defs_n)):
        raise ValueError('invalid DEX table size')
    strings = []
    for i in range(strings_n):
        at = word(strings_off + 4 * i)
        for _ in range(5):
            if at >= len(data):
                raise ValueError('truncated DEX string')
            byte = data[at]
            at += 1
            if byte < 128:
                break
        else:
            raise ValueError('invalid DEX string length')
        end = data.index(0, at)
        strings.append(data[at:end].decode('utf-8', errors='replace'))
    types = [strings[word(types_off + 4 * i)] for i in range(types_n)]
    names = []
    for i in range(defs_n):
        descriptor = types[word(defs_off + 32 * i)]
        if descriptor.startswith('L') and descriptor.endswith(';'):
            names.append(descriptor[1:-1].replace('/', '.'))
    return names


def pack(args):
    with zipfile.ZipFile(args.resources) as source, zipfile.ZipFile(args.out, 'x') as dest:
        for member in source.infolist():
            with source.open(member) as reader, dest.open(member, 'w') as writer:
                import shutil
                shutil.copyfileobj(reader, writer, 65536)
        additions = [(path.name, path, 0o100644) for path in sorted(args.dex.glob('*.dex'))]
        if args.native:
            additions.append(('lib/arm64-v8a/libemacs.so', args.native, 0o100755))
        if args.assets:
            additions.extend((p.relative_to(args.assets).as_posix(), p, 0o100644)
                             for p in sorted((args.assets / 'assets').rglob('*')) if p.is_file())
        if args.wrap:
            additions.append(('lib/arm64-v8a/wrap.sh', args.wrap, 0o100755))
        for name, path, mode in additions:
            entry = zipfile.ZipInfo(name)
            entry.compress_type = zipfile.ZIP_STORED
            entry.external_attr = mode << 16
            with path.open('rb') as reader, dest.open(entry, 'w') as writer:
                import shutil
                shutil.copyfileobj(reader, writer, 65536)


def inspect(args):
    with zipfile.ZipFile(args.apk) as archive:
        names = archive.namelist()
        if len(names) != len(set(names)):
            raise ValueError('duplicate APK member')
        classes = []
        dex_hash = hashlib.sha256()
        assets_hash = hashlib.sha256()
        native = None
        assets = []
        stored = True
        for member in archive.infolist():
            if member.filename.endswith('.dex') and '/' not in member.filename:
                if member.file_size > 64 * 1024 * 1024:
                    raise ValueError('oversized DEX measurement')
                data = archive.read(member)
                classes.extend(class_names(data))
                dex_hash.update(data)
            elif member.filename == 'lib/arm64-v8a/libemacs.so':
                with archive.open(member) as file:
                    native = stream_digest(file).hexdigest()
            elif member.filename.startswith('assets/') and not member.is_dir():
                assets.append(member.filename)
                assets_hash.update(member.filename.encode() + b'\0')
                with archive.open(member) as file:
                    assets_hash.update(stream_digest(file).digest())
                stored &= member.compress_type == zipfile.ZIP_STORED
        result = dict(members=names, sha256=digest(args.apk), classes=sorted(set(classes)),
                      dex_sha256=dex_hash.hexdigest(), native_sha256=native,
                      assets_count=len(assets), assets_sha256=assets_hash.hexdigest(),
                      assets_stored=stored, directory_tree='assets/directory-tree' in names,
                      wrap_mode=next((oct(i.external_attr >> 16) for i in archive.infolist()
                                      if i.filename == 'lib/arm64-v8a/wrap.sh'), None))
        with args.out.open('x') as file:
            json.dump(result, file, indent=2)
            file.write('\n')


def patch_wrap_mode(apk):
    # Change only the central directory's Unix mode. Preserve aligned member offsets/data.
    with zipfile.ZipFile(apk) as archive:
        position=archive.start_dir
        expected=len(archive.infolist())
    changed=0
    with Path(apk).open('r+b') as file:
        for _ in range(expected):
            file.seek(position)
            header=file.read(46)
            if len(header)!=46 or header[:4]!=b'PK\x01\x02':
                raise ValueError('invalid APK central directory')
            name_size,extra_size,comment_size=struct.unpack_from('<HHH',header,28)
            name=file.read(name_size).decode('utf-8')
            if name=='lib/arm64-v8a/wrap.sh':
                file.seek(position+38)
                file.write(struct.pack('<I',0o100755<<16))
                changed+=1
            position+=46+name_size+extra_size+comment_size
    if changed!=1:
        raise ValueError('explicit wrap helper central entry missing or duplicated')


def main():
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest='mode', required=True)
    p = sub.add_parser('pack')
    for key in ('resources', 'dex', 'out'):
        p.add_argument('--' + key, type=Path, required=True)
    for key in ('native', 'assets', 'wrap'):
        p.add_argument('--' + key, type=Path)
    p = sub.add_parser('inspect')
    p.add_argument('--apk', type=Path, required=True)
    p.add_argument('--out', type=Path, required=True)
    p = sub.add_parser('wrap-mode')
    p.add_argument('--apk', type=Path, required=True)
    args = parser.parse_args()
    if args.mode=='wrap-mode':patch_wrap_mode(args.apk)
    else:(pack if args.mode == 'pack' else inspect)(args)


if __name__ == '__main__':
    main()
