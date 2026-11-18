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
import sys


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            digest.update(block)
    return digest.hexdigest()


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--selection', type=Path, required=True,
                        help='existing reference selection that supplies the epoch')
    parser.add_argument('--receipt', type=Path, action='append', required=True,
                        help='closed stable-suite-run.json that used the same reference')
    parser.add_argument('--out', type=Path, required=True)
    options = parser.parse_args(argv)
    selection = json.loads(options.selection.resolve().read_text(encoding='utf-8'))
    product = selection['original_reference']['product']
    program, dump = Path(product['program']), Path(product['dump'])
    identities, used = {}, []
    for receipt_path in options.receipt:
        receipt = json.loads(receipt_path.resolve().read_text(encoding='utf-8'))
        identity = receipt['products_before']['reference']
        used.append(dict(receipt=str(receipt_path), product_identity=identity))
        identities.setdefault(json.dumps(identity, sort_keys=True), []).append(str(receipt_path))
    if len(identities) != 1:
        print('reference-binding: closed receipts disagree on the reference identity', file=sys.stderr)
        return 2
    document = dict(
        schema='femacs-reference-selection-v1',
        scope='post-hoc reference identity binding for the single-product G3 channel',
        epoch=selection['epoch'],
        upstream=dict(version='32.0.50', commit='fd7afced4036fc1a221567d70705d20a2bd64713',
                      tap='homebrew/emacs-plus',
                      source='existing reference selection epoch; identity re-hashed from the installed product'),
        admission=dict(kind='post-hoc-verification',
                       why='the channel ran the installed reference without a selection receipt; '
                           'this binding is derived from the actual bytes after the runs',
                       verified_at=datetime.datetime.now().astimezone().isoformat(timespec='seconds')),
        product=product,
        identity=dict(program=dict(bytes=program.stat().st_size, sha256=sha256(program)),
                      dump=dict(bytes=dump.stat().st_size, sha256=sha256(dump)),
                      lisp_root=product['resource_roots']['lisp']),
        closed_receipts=used)
    body = json.dumps(document, sort_keys=True, separators=(',', ':')).encode()
    document['sha256'] = hashlib.sha256(body).hexdigest()
    out = options.out
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(document, ensure_ascii=False, indent=1) + '\n', encoding='utf-8')
    print(json.dumps(dict(epoch=document['epoch'], sha256=document['sha256'],
                          receipts=len(used), kind=document['admission']['kind'])))
    return 0


if __name__ == '__main__':
    sys.exit(main())
