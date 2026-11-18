"""Bounded byte identity analysis of two explicit native bootstrap containers.

Layout authority: Darwin SDK mach-o/loader.h and the pinned CodeDirectory
declaration at https://github.com/apple-oss-distributions/xnu/blob/
xnu-11215.81.4/osfmk/kern/cs_blobs.h. No container is loaded or modified.
This evidence does not change an ERT result or accept a closed corpus case.
"""
import argparse
from contextlib import ExitStack
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import struct
import re

SCHEMA = 'femacs-native-bootstrap-identity-v1'
ROOT = Path(__file__).resolve().parent.parent
MAX_FILE = 64 * 1024 * 1024
MAX_COMMANDS = 4096
MAX_COMMAND_BYTES = 1024 * 1024
MAX_SIGNATURE = 1024 * 1024
CHUNK = 65536


def require(value, message):
    if not value:
        raise ValueError(message)


def identity(meta):
    return tuple(getattr(meta, key) for key in
                 ('st_dev', 'st_ino', 'st_size', 'st_mode', 'st_mtime_ns', 'st_ctime_ns'))


class Input:
    def __init__(self, path):
        self.path = Path(os.path.abspath(path))
        require(self.path.resolve() == self.path and not self.path.is_symlink(),
                'physical no-symlink input required')
        self.fd = os.open(self.path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            self.before = os.fstat(self.fd)
            require(stat.S_ISREG(self.before.st_mode) and 32 <= self.before.st_size <= MAX_FILE,
                    'bounded ordinary native container required')
            require(identity(self.before) == identity(self.path.lstat()), 'input replaced while opened')
            self.sha256 = self.digest(0, self.before.st_size)
        except BaseException:
            os.close(self.fd)
            raise

    def __enter__(self):
        return self

    def __exit__(self, kind, error, traceback):
        try:
            require(identity(self.before) == identity(os.fstat(self.fd))
                    == identity(self.path.lstat()), 'native input identity changed')
            require(self.sha256 == self.digest(0, self.before.st_size), 'native input content changed')
        finally:
            os.close(self.fd)

    def read(self, offset, size):
        require(type(offset) is int and type(size) is int and 0 <= offset
                and 0 <= size <= MAX_COMMAND_BYTES and offset + size <= self.before.st_size,
                'native read outside its declared bounds')
        value = os.pread(self.fd, size, offset)
        require(len(value) == size, 'short native container read')
        return value

    def digest(self, offset, size):
        result = hashlib.sha256()
        end = offset + size
        while offset < end:
            data = self.read(offset, min(CHUNK, end-offset))
            result.update(data)
            offset += len(data)
        return result.hexdigest()

    def public_identity(self):
        return dict(path=str(self.path), bytes=self.before.st_size,
                    mode=stat.S_IMODE(self.before.st_mode), sha256=self.sha256)


def name(raw):
    value, _, padding = raw.partition(b'\0')
    require(not any(padding) and value and all(32 <= byte < 127 for byte in value),
            'invalid Mach-O segment or section name')
    return value.decode('ascii')


def signature(image, offset, size, library_name, executable_segments):
    require(20 <= size <= MAX_SIGNATURE and offset+size == image.before.st_size,
            'bounded terminal signature required')
    raw = image.read(offset, size)
    magic, length, count, slot, start = struct.unpack_from('>5I', raw)
    require(magic == 0xfade0cc0 and length == size and count == 1 and slot == 0
            and start == 20, 'single original CodeDirectory signature required')
    cd = raw[start:]
    require(len(cd) >= 88, 'truncated CodeDirectory')
    (magic, length, version, flags, hashes, identifier, special, slots, limit,
     hash_size, hash_type, platform, page, spare) = struct.unpack_from('>9I4BI', cd)
    require(magic == 0xfade0c02 and length == len(cd) and version == 0x20400
            and flags == 0x20002 and hash_size == 32 and hash_type == 2
            and platform == 0 and page in (12, 14) and special == 0 and spare == 0,
            'unsupported native CodeDirectory format')
    scatter, team, spare3, limit64, base, extent, exec_flags = struct.unpack_from('>III4Q', cd, 44)
    require(scatter == team == spare3 == 0 and limit64 in (0, limit)
            and limit == offset and exec_flags == 0
            and any(start <= base and 0 < extent and base+extent <= start+size
                    for start, size in executable_segments),
            'CodeDirectory executable range or page coverage differs')
    require(88 <= identifier < hashes <= len(cd) and slots == (limit+(1 << page)-1)//(1 << page)
            and hashes+slots*hash_size == len(cd), 'CodeDirectory slots are out of range')
    identifier_raw = cd[identifier:hashes]
    identifier_text, separator, padding = identifier_raw.partition(b'\0')
    require(separator and not any(padding) and identifier_text == library_name.rsplit(b'/', 1)[-1],
            'signature identifier differs from the declared library')
    for index in range(slots):
        position = index*(1 << page)
        expected = hashlib.sha256(image.read(position, min(1 << page, limit-position))).digest()
        require(cd[hashes+index*32:hashes+(index+1)*32] == expected,
                'CodeDirectory signed page hash mismatch')
    normalized = bytearray(raw)
    normalized[start+identifier:start+hashes] = b'\0'*(hashes-identifier)
    normalized[start+hashes:] = b'\0'*(slots*32)
    return dict(version=version, flags=flags, bytes=size, code_limit=limit,
                executable_base=base, executable_extent=extent,
                page_size=1 << page, code_slots=slots, all_page_hashes_valid=True,
                identifier_sha256=hashlib.sha256(identifier_text).hexdigest(),
                normalized_sha256=hashlib.sha256(normalized).hexdigest())


def inspect(image):
    header = struct.unpack('<IiiIIIII', image.read(0, 32))
    magic, cpu, subtype, kind, commands, size, flags, reserved = header
    require(magic == 0xfeedfacf and cpu == 0x0100000c and kind == 6 and reserved == 0,
            'only little-endian ARM64 dynamic libraries are supported')
    require(0 < commands <= MAX_COMMANDS and 0 < size <= MAX_COMMAND_BYTES
            and 32+size <= image.before.st_size, 'invalid load-command bound')
    position, regions, sections, segments, executable = 32, {}, [], [], []
    library_name = None
    for _ in range(commands):
        command, length = struct.unpack('<II', image.read(position, 8))
        require(length >= 8 and length % 8 == 0 and position+length <= 32+size,
                'invalid load-command extent')
        data = image.read(position, length)
        if command == 0x19:
            require(length >= 72, 'short 64-bit segment')
            (_, _, raw_name, address, vm_size, offset, count,
             maximum, protection, nsections, segment_flags) = struct.unpack_from('<II16s4QiiII', data)
            segment_name = name(raw_name)
            require(nsections <= 1024 and length == 72+80*nsections
                    and offset+count <= image.before.st_size, 'invalid segment or section extent')
            segments.append(dict(name=segment_name, offset=offset, bytes=count,
                                 protection=protection))
            if protection & 4:
                executable.append((offset, count))
            for index in range(nsections):
                values = struct.unpack_from('<16s16sQQ8I', data, 72+index*80)
                section_name, parent = name(values[0]), name(values[1])
                extent, start, attributes = values[3], values[4], values[8]
                zero = attributes & 0xff in (1, 0xc, 0x12)
                require(parent == segment_name and extent <= MAX_FILE, 'invalid section parent or size')
                if not zero and extent:
                    require(32+size <= start and offset <= start and start+extent <= offset+count,
                            'section overlaps commands or exceeds segment')
                sections.append(dict(name=parent+','+section_name, offset=start, bytes=extent,
                    zerofill=zero, sha256=None if zero else image.digest(start, extent)))
        elif command == 0xd:
            require('LC_ID_DYLIB_name' not in regions and length >= 24, 'duplicate or short library identity')
            start = struct.unpack_from('<I', data, 8)[0]
            require(24 <= start < length, 'library identity name outside command')
            library_name, terminator, padding = data[start:].partition(b'\0')
            require(terminator and library_name.endswith(b'.eln') and not any(padding)
                    and len(library_name) <= 4096
                    and all(32 <= byte < 127 for byte in library_name), 'invalid native library name')
            regions['LC_ID_DYLIB_name'] = dict(offset=position+start, bytes=length-start)
        elif command == 0x1b:
            require('LC_UUID' not in regions and length == 24, 'duplicate or malformed UUID command')
            regions['LC_UUID'] = dict(offset=position+8, bytes=16)
        elif command == 0x1d:
            require('code_signature' not in regions and length == 16, 'duplicate or malformed signature command')
            start, count = struct.unpack_from('<II', data, 8)
            regions['code_signature'] = dict(offset=start, bytes=count)
        position += length
    require(position == 32+size and set(regions) == {'LC_ID_DYLIB_name', 'LC_UUID', 'code_signature'}
            and sections and executable, 'incomplete native layout')
    intervals = sorted((value['offset'], value['offset']+value['bytes']) for value in regions.values())
    require(all(0 <= a < b <= image.before.st_size for a, b in intervals)
            and all(left[1] <= right[0] for left, right in zip(intervals, intervals[1:])),
            'invalid or overlapping metadata regions')
    real_sections = [section for section in sections if not section['zerofill'] and section['bytes']]
    for start, end in intervals:
        require(all(end <= section['offset'] or section['offset']+section['bytes'] <= start
                    for section in real_sections), 'metadata mask overlaps a code/data section')
    code_signature = regions['code_signature']
    require(any(segment['name'] == '__LINKEDIT' and not segment['protection'] & 4
                and segment['offset'] <= code_signature['offset']
                and code_signature['offset']+code_signature['bytes'] <= segment['offset']+segment['bytes']
                for segment in segments), 'signature outside non-executable link metadata')
    sig = signature(image, code_signature['offset'], code_signature['bytes'], library_name, executable)
    return dict(architecture='arm64', header=list(header), metadata_regions=regions,
                sections=sections, signature=sig)


def compare(first, second):
    with ExitStack() as stack:
        left, right = [stack.enter_context(Input(path)) for path in (first, second)]
        layouts = [inspect(image) for image in (left, right)]
        same_regions = layouts[0]['metadata_regions'] == layouts[1]['metadata_regions']
        same_size = left.before.st_size == right.before.st_size
        counts = dict(LC_ID_DYLIB_name=0, LC_UUID=0, code_signature=0, outside_metadata=0)
        normalized = [hashlib.sha256(), hashlib.sha256()]
        if same_regions and same_size:
            regions = layouts[0]['metadata_regions']
            for position in range(0, left.before.st_size, CHUNK):
                blocks = [bytearray(image.read(position, min(CHUNK, image.before.st_size-position)))
                          for image in (left, right)]
                for index, (a, b) in enumerate(zip(*blocks)):
                    if a != b:
                        offset = position+index
                        region = next((key for key, value in regions.items()
                            if value['offset'] <= offset < value['offset']+value['bytes']), 'outside_metadata')
                        counts[region] += 1
                for block, digest in zip(blocks, normalized):
                    for value in regions.values():
                        start = max(position, value['offset'])-position
                        end = min(position+len(block), value['offset']+value['bytes'])-position
                        if start < end:
                            block[start:end] = b'\0'*(end-start)
                    digest.update(block)
        all_sections_equal = layouts[0]['sections'] == layouts[1]['sections']
        signature_equal = (layouts[0]['signature']['normalized_sha256']
                           == layouts[1]['signature']['normalized_sha256'])
        metadata_only = (same_regions and same_size and sum(counts.values()) > 0
            and counts['outside_metadata'] == 0 and all_sections_equal and signature_equal
            and normalized[0].digest() == normalized[1].digest())
        result = dict(schema=SCHEMA, collection_complete=True,
            inputs=[image.public_identity() for image in (left, right)], layouts=layouts,
            byte_identical=left.sha256 == right.sha256, metadata_layout_equal=same_regions,
            all_sections_equal=all_sections_equal, normalized_signatures_equal=signature_equal,
            difference_counts=counts if same_regions and same_size else None,
            normalized_sha256=[digest.hexdigest() for digest in normalized] if same_regions and same_size else None,
            differs_only_in_validated_metadata=metadata_only,
            actual_editor_execution=False, native_images_executed=False,
            closed_case_bound=False, original_failed_results_promoted=False,
            compatibility_accepted=False, goal_complete=False)
    result['inputs_unchanged_at_exit'] = True
    return result


def _pairs(items):
    result = {}
    for key, value in items:
        require(key not in result, 'duplicate evidence field')
        result[key] = value
    return result


def _json_input(image):
    raw = b''.join(image.read(start, min(CHUNK, image.before.st_size-start))
                   for start in range(0, image.before.st_size, CHUNK))
    value = json.loads(raw, object_pairs_hook=_pairs,
        parse_constant=lambda value: require(False, 'non-finite evidence number'))
    require(isinstance(value, dict), 'evidence object required')
    return value


def _relative(root, value):
    require(isinstance(value, str) and bool(value), 'explicit relative evidence path required')
    path = Path(value)
    require(not path.is_absolute() and '..' not in path.parts and path.as_posix() == value
            and value != '.', 'canonical relative evidence path required')
    return root/path


def compare_closed_file(pair_root, file_result):
    """Bind images to a sealed file without certifying its outer stage or ERT."""
    root = Path(os.path.abspath(pair_root))
    require(root.resolve() == root and root.is_dir(), 'physical pair root required')
    with ExitStack() as stack:
        evidence = stack.enter_context(Input(file_result))
        file = _json_input(evidence)
        label = file.get('product')
        logical, test_id = 'test/src/comp-tests.el', 'comp-tests-bootstrap'
        require(label in ('reference', 'candidate')
                and evidence.path.parent == root/'results'/label
                and file.get('schema') == 'femacs-stable-file-batch-v1'
                and file.get('file') == logical and file.get('selector') == 'all-discovered-ids'
                and file.get('complete') is True and file.get('exit_code') == 0
                and file.get('timed_out') is False and file.get('remaining_group_terminated') is False,
                'complete original compiler-file evidence required')
        case = file.get('cases', {}).get(test_id, {})
        require(case.get('status') == 'failed' and case.get('complete') is True
                and case.get('exit_code') == 0 and case.get('assessment', {}).get('passed') is False,
                'original failed bootstrap outcome required')
        owner_path = evidence.path.with_name(evidence.path.stem+'.owner-retirement.json')
        require(_relative(root, file.get('state_retirement_path')) == owner_path,
                'file points to another owner receipt')
        owner_input = stack.enter_context(Input(owner_path))
        owner = _json_input(owner_input)
        expected = dict(path=evidence.path.relative_to(root).as_posix(),
                        bytes=evidence.before.st_size, sha256=evidence.sha256)
        retirement = owner.get('state_retirement', {})
        require(owner.get('schema') == 'femacs-stable-owner-retirement-v1'
                and owner.get('file_result') == expected and retirement.get('status') == 'retained'
                and retirement.get('attempt_observation', {}).get('clear') is True,
                'retained owner does not bind the exact reaped file')
        pin = retirement.get('closure_state_proof', {})
        proof_path = _relative(owner_path.parent, pin.get('path'))
        require(proof_path == evidence.path.with_name(evidence.path.stem+'.retained-state.private.json'),
                'unexpected closure-time proof path')
        proof_input = stack.enter_context(Input(proof_path))
        require(proof_input.before.st_size == pin.get('bytes') and proof_input.sha256 == pin.get('sha256'),
                'closure-time proof identity changed')
        proof = _json_input(proof_input)
        snapshot = proof.get('snapshot', {})
        state_root = Path(file.get('owned_state_root', ''))
        container = Path(file.get('owned_container', ''))
        require(proof.get('schema') == 'femacs-stable-retained-state-v1'
                and proof.get('evidence_sha256') == evidence.sha256
                and state_root.is_absolute() and container.is_absolute() and state_root.parent == container
                and snapshot.get('root') == str(state_root) and snapshot.get('allow_symlinks') is True
                and len(snapshot.get('parents', [])) == 2
                and snapshot.get('names') == sorted(['home', 'cache', 'config', 'state', 'tmp',
                                                    'native-cache', 'out', 'cwd']),
                'closure-time proof has another file or state scope')
        verifier_input = stack.enter_context(Input(Path(__file__).with_name('runtime-retire.py')))
        spec = importlib.util.spec_from_file_location('bootstrap_state_verifier', verifier_input.path)
        verifier = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(verifier)
        verifier.verify_owned_state(snapshot)
        suite_input = stack.enter_context(Input(Path(__file__).with_name('stable-suite.py')))
        batch_input = stack.enter_context(Input(Path(__file__).with_name('stable-suite-batch.py')))
        spec = importlib.util.spec_from_file_location('bootstrap_file_protocol', suite_input.path)
        suite = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(suite)
        batch = suite._load_batch()
        require(Path(batch.__file__) == batch_input.path, 'file protocol reader is not co-located')
        # Reuse the maintained loaded/start/end/finished protocol validator.
        # File closure cannot be inferred from one convenient failed event.
        batch._verify_file_events(suite, root, file)
        progress = file.get('progress', {})
        progress_input = stack.enter_context(Input(_relative(root, progress.get('path'))))
        event_pin = case.get('evidence', {})
        require(progress_input.sha256 == progress.get('sha256')
                and progress_input.before.st_size == progress.get('bytes')
                and event_pin.get('path') == progress.get('path'), 'original progress identity changed')
        found, ordinal = None, 0
        # A separate no-follow descriptor keeps line parsing bounded; the
        # held input descriptor also rehashes the complete stream on exit.
        fd = os.open(progress_input.path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        with os.fdopen(fd, 'rb') as stream:
            require(identity(os.fstat(stream.fileno())) == identity(progress_input.before),
                    'original progress replaced before event read')
            while raw := stream.readline(MAX_COMMAND_BYTES+1):
                require(len(raw) <= MAX_COMMAND_BYTES and raw.endswith(b'\n'), 'oversized or incomplete original event')
                event = json.loads(raw, object_pairs_hook=_pairs)
                if event.get('event') != 'test-ended':
                    continue
                if hashlib.sha256(raw[:-1]).hexdigest() == event_pin.get('event_sha256'):
                    require(ordinal == event_pin.get('event_ordinal'), 'original event ordinal differs')
                    found = event
                    break
                ordinal += 1
        require(isinstance(found, dict) and found.get('file') == logical and found.get('test_id') == test_id
                and found.get('status') == 'failed' and found.get('diagnostic') == case.get('diagnostic')
                and isinstance(found.get('messages'), str), 'exact original bootstrap event required')
        matches = re.findall(r'^Comparing (\S+) (\S+)$', found['messages'], re.MULTILINE)
        require(len(matches) == 1, 'one original bootstrap comparison message required')
        images = [Path(value) for value in matches[0]]
        nodes = {node['path']: node for node in snapshot['nodes']}
        require(len(nodes) == len(snapshot['nodes']), 'duplicate closure-state node')
        for index, path in enumerate(images, 1):
            require(path.is_absolute() and path.is_relative_to(state_root)
                    and '-comp-stage'+str(index)+'-' in path.name and path.suffix == '.eln',
                    'comparison image is outside its original bootstrap state')
            node = nodes.get(path.relative_to(state_root).as_posix(), {})
            require(node.get('kind') == 'file', 'comparison image has no closure-time file proof')
        result = compare(*images)
        for value, path in zip(result['inputs'], images):
            node = nodes[path.relative_to(state_root).as_posix()]
            require(all(value[key] == node[key] for key in ('bytes', 'sha256', 'mode')),
                    'comparison image differs from its closure-time proof')
        verifier.verify_owned_state(snapshot)
        result.update(file_closure_bound=True, closed_outer_stage_bound=False,
            file_closure=dict(file=evidence.public_identity(), owner=owner_input.public_identity(),
                proof=proof_input.public_identity(), progress=progress_input.public_identity(),
                verifier=verifier_input.public_identity(), tree_sha256=snapshot['tree_sha256'],
                file_protocol_sources=[suite_input.public_identity(), batch_input.public_identity()],
                retained_state_root=str(state_root), retained_container=str(container),
                product=label, logical_file=logical, test_id=test_id,
                event_sha256=event_pin['event_sha256'], event_ordinal=event_pin['event_ordinal'],
                original_condition=found.get('condition'), raw_status='failed'))
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--first', type=Path)
    parser.add_argument('--second', type=Path)
    parser.add_argument('--pair-root', type=Path)
    parser.add_argument('--file-result', type=Path)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        out = Path(os.path.abspath(args.out))
        require(out.resolve() == out and not os.path.lexists(out)
                and not out.is_relative_to(ROOT/'emacs'), 'fresh owned analysis output required')
        direct = args.first is not None and args.second is not None
        closed = args.pair_root is not None and args.file_result is not None
        require((direct and args.pair_root is None and args.file_result is None)
                or (closed and args.first is None and args.second is None),
                'supply either --first/--second or --pair-root/--file-result')
        require(not closed or not out.is_relative_to(Path(os.path.abspath(args.pair_root))),
                'analysis output cannot change the closed pair')
        value = compare(args.first, args.second) if direct else compare_closed_file(args.pair_root, args.file_result)
        require(not closed or not out.is_relative_to(Path(value['file_closure']['retained_container'])),
                'analysis output cannot change sealed retained state')
        out.mkdir(parents=True)
        with (out/'results.json').open('x', encoding='utf-8') as stream:
            json.dump(value, stream, indent=2, allow_nan=False)
            stream.write('\n')
            stream.flush()
            os.fsync(stream.fileno())
        print(json.dumps(dict(collection_complete=True,
            differs_only_in_validated_metadata=value['differs_only_in_validated_metadata'])))
        return 0
    except (OSError, ValueError, struct.error, KeyError, TypeError, AttributeError) as error:
        parser.error(str(error))


if __name__ == '__main__':
    raise SystemExit(main())
