#!/usr/bin/env python3
"""Derive a post-hoc reference-identity binding for the single-product channel.

The single-product G3 channel executes the installed reference directly, so it
has no pre-admission selection receipt. This tool produces an explicit binding
from two real inputs: an existing reference selection (for the epoch and the
product description) and the closed stable-suite receipts (for the identity that
actually ran). It re-hashes the installed program and dump, requires every
receipt to agree on the same reference product identity, and marks the document
as post-hoc verification with its timestamp; it never claims pre-admission.
"""
from __future__ import annotations
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re
import sys


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            digest.update(block)
    return digest.hexdigest()


def build_posthoc_selection(
    *,
    epoch: str,
    product: dict,
    upstream_version: str,
    upstream_commit: str,
    upstream_source: str,
    receipt_paths: list[Path],
    out: Path,
) -> dict:
    if not epoch or not epoch.strip():
        raise ValueError('reference-binding: --epoch must be non-empty')
    if not upstream_version or not upstream_version.strip():
        raise ValueError('reference-binding: --upstream-version must be non-empty')
    if not re.fullmatch(r'[0-9a-f]{40}', upstream_commit):
        raise ValueError(f'reference-binding: --upstream-commit must be 40 hex chars: {upstream_commit!r}')
    program = Path(product['program']).resolve()
    dump = Path(product['dump']).resolve()
    lisp_root = Path(product['resource_roots']['lisp']).resolve()
    if not program.is_file():
        raise FileNotFoundError(f'reference-binding: missing reference program: {program}')
    if not dump.is_file():
        raise FileNotFoundError(f'reference-binding: missing reference dump: {dump}')
    if not lisp_root.is_dir():
        raise FileNotFoundError(f'reference-binding: missing reference lisp root: {lisp_root}')
    data_root = product['resource_roots'].get('data')
    if data_root is not None and not Path(data_root).resolve().is_dir():
        raise FileNotFoundError(f'reference-binding: missing reference data root: {data_root}')

    prog_sha, prog_bytes = sha256(program), program.stat().st_size
    dump_sha, dump_bytes = sha256(dump), dump.stat().st_size
    identities, used = {}, []
    for receipt_path in receipt_paths:
        receipt = json.loads(receipt_path.resolve().read_text(encoding='utf-8'))
        identity = receipt['products_before']['reference']
        r_prog = identity.get('program') or {}
        r_dump = identity.get('dump') or {}
        if r_prog.get('sha256') != prog_sha or r_prog.get('bytes') != prog_bytes:
            raise ValueError(f'reference-binding: receipt reference program mismatch: {receipt_path}')
        if r_dump.get('sha256') != dump_sha or r_dump.get('bytes') != dump_bytes:
            raise ValueError(f'reference-binding: receipt reference dump mismatch: {receipt_path}')
        used.append(dict(receipt=str(receipt_path), product_identity=identity))
        identities.setdefault(json.dumps(identity, sort_keys=True), []).append(str(receipt_path))
    if len(identities) != 1:
        raise ValueError('reference-binding: closed receipts disagree on the reference identity')

    document = dict(
        schema='femacs-reference-selection-v1',
        scope='post-hoc reference identity binding for the single-product G3 channel',
        epoch=epoch,
        upstream=dict(
            version=upstream_version,
            commit=upstream_commit,
            tap='homebrew/emacs-plus',
            source=upstream_source,
        ),
        admission=dict(
            kind='post-hoc-verification',
            why='the channel ran the installed reference without a selection receipt; '
                'this binding is derived from the actual bytes after the runs',
            verified_at=datetime.datetime.now().astimezone().isoformat(timespec='seconds'),
        ),
        product=product,
        identity=dict(
            program=dict(bytes=prog_bytes, sha256=prog_sha),
            dump=dict(bytes=dump_bytes, sha256=dump_sha),
            lisp_root=product['resource_roots']['lisp'],
        ),
        closed_receipts=used,
    )
    body = json.dumps(document, sort_keys=True, separators=(',', ':')).encode()
    document['sha256'] = hashlib.sha256(body).hexdigest()
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(document, ensure_ascii=False, indent=1) + '\n', encoding='utf-8')
    return document


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--selection', type=Path,
                        help='existing reference selection that supplies the epoch')
    parser.add_argument('--program', type=Path,
                        help='installed reference binary (when --selection is omitted)')
    parser.add_argument('--dump', type=Path,
                        help='installed reference .pdmp dump (when --selection is omitted)')
    parser.add_argument('--lisp-root', type=Path,
                        help='installed reference lisp/ root (when --selection is omitted)')
    parser.add_argument('--data-root', type=Path,
                        help='installed reference etc/ data root (when --selection is omitted)')
    parser.add_argument('--epoch',
                        help='declared reference epoch (when --selection is omitted)')
    parser.add_argument('--upstream-version', default='32.0.50')
    parser.add_argument('--upstream-commit', default='fd7afced4036fc1a221567d70705d20a2bd64713')
    parser.add_argument('--receipt', type=Path, action='append', required=True,
                        help='closed stable-suite-run.json that used the same reference')
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args(argv)
    if options.selection is not None:
        if any(getattr(options, name) is not None for name in ('program', 'dump', 'lisp_root', 'data_root', 'epoch')):
            parser.error('--selection cannot be combined with direct installed-reference flags')
        selection = json.loads(options.selection.resolve().read_text(encoding='utf-8'))
        product = selection['original_reference']['product']
        epoch = selection['epoch']
        source_desc = 'existing reference selection epoch; identity re-hashed from the installed product'
    else:
        missing = [
            flag for flag, val in (
                ('--program', options.program),
                ('--dump', options.dump),
                ('--lisp-root', options.lisp_root),
                ('--data-root', options.data_root),
                ('--epoch', options.epoch),
            )
            if val is None
        ]
        if missing:
            parser.error('missing direct installed-reference options: ' + ', '.join(missing))
        product = dict(
            id='installed-reference',
            kind='reference',
            compiler_leg='installed',
            native=False,
            observer=False,
            test_hooks=False,
            collector='off',
            program=str(options.program.resolve()),
            dump=str(options.dump.resolve()),
            resource_roots=dict(
                lisp=str(options.lisp_root.resolve()),
                data=str(options.data_root.resolve()),
            ),
        )
        epoch = options.epoch
        source_desc = 'direct installed reference paths; identity re-hashed from actual installed bytes'
    try:
        document = build_posthoc_selection(
            epoch=epoch,
            product=product,
            upstream_version=options.upstream_version,
            upstream_commit=options.upstream_commit,
            upstream_source=source_desc,
            receipt_paths=options.receipt,
            out=options.out,
        )
    except (OSError, ValueError, KeyError, TypeError) as exc:
        print(str(exc), file=sys.stderr)
        return 2
    print(json.dumps(dict(epoch=document['epoch'], sha256=document['sha256'],
                          receipts=len(document['closed_receipts']),
                          kind=document['admission']['kind'])))
    return 0


if __name__ == '__main__':
    sys.exit(main())
